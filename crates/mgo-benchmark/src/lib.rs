// Copyright (c) MangoNet Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
use anyhow::bail;
use async_trait::async_trait;
use embedded_reconfig_observer::EmbeddedReconfigObserver;
use fullnode_reconfig_observer::FullNodeReconfigObserver;
use futures::{stream::FuturesUnordered, StreamExt};
use mango_metrics::GaugeGuard;
use prometheus::Registry;
use rand::Rng;
use roaring::RoaringBitmap;
use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
    time::Duration,
};
use mgo_config::genesis::Genesis;
use mgo_core::{
    authority_aggregator::{AuthorityAggregator, AuthorityAggregatorBuilder},
    authority_client::{
        make_authority_clients_with_timeout_config, AuthorityAPI, NetworkAuthorityClient,
    },
    quorum_driver::{
        QuorumDriver, QuorumDriverHandler, QuorumDriverHandlerBuilder, QuorumDriverMetrics,
    },
};
use mgo_json_rpc_types::{
    MgoObjectDataOptions, MgoObjectResponse, MgoObjectResponseQuery, MgoTransactionBlockEffects,
    MgoTransactionBlockEffectsAPI, MgoTransactionBlockResponseOptions,
};
use mgo_network::{DEFAULT_CONNECT_TIMEOUT_SEC, DEFAULT_REQUEST_TIMEOUT_SEC};
use mgo_sdk::{MgoClient, MgoClientBuilder};
use mgo_types::base_types::ConciseableName;
use mgo_types::committee::CommitteeTrait;
use mgo_types::effects::{CertifiedTransactionEffects, TransactionEffectsAPI, TransactionEvents};
use mgo_types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use mgo_types::mgo_system_state::mgo_system_state_summary::MgoSystemStateSummary;
use mgo_types::transaction::Argument;
use mgo_types::transaction::CallArg;
use mgo_types::transaction::ObjectArg;
use mgo_types::{
    base_types::ObjectID,
    committee::{Committee, EpochId},
    crypto::{
        AggregateAuthenticator, AggregateAuthoritySignature, AuthorityQuorumSignInfo,
        AuthoritySignature,
    },
    message_envelope::Envelope,
    object::Object,
    transaction::{CertifiedTransaction, Transaction},
};
use mgo_types::{base_types::ObjectRef, crypto::AuthorityStrongQuorumSignInfo, object::Owner};
use mgo_types::{base_types::SequenceNumber, gas_coin::GasCoin};
use mgo_types::{
    base_types::{AuthorityName, MgoAddress},
    mgo_system_state::MgoSystemStateTrait,
};
use mgo_types::{error::MgoError, gas::GasCostSummary};
use tokio::{
    task::JoinSet,
    time::{sleep, timeout},
};
use tracing::{error, info};

pub mod bank;
pub mod benchmark_setup;
pub mod drivers;
pub mod embedded_reconfig_observer;
pub mod fullnode_reconfig_observer;
pub mod in_memory_wallet;
pub mod options;
pub mod system_state_observer;
pub mod util;
pub mod workloads;
use futures::FutureExt;
use mgo_types::messages_grpc::{HandleCertificateResponseV2, TransactionStatus};
use mgo_types::quorum_driver_types::{QuorumDriverError, QuorumDriverResponse};

#[derive(Debug)]
/// A wrapper on execution results to accommodate different types of
/// responses from LocalValidatorAggregatorProxy and FullNodeProxy
#[allow(clippy::large_enum_variant)]
pub enum ExecutionEffects {
    CertifiedTransactionEffects(CertifiedTransactionEffects, TransactionEvents),
    MgoTransactionBlockEffects(MgoTransactionBlockEffects),
}

impl ExecutionEffects {
    pub fn mutated(&self) -> Vec<(ObjectRef, Owner)> {
        match self {
            ExecutionEffects::CertifiedTransactionEffects(certified_effects, ..) => {
                certified_effects.data().mutated().to_vec()
            }
            ExecutionEffects::MgoTransactionBlockEffects(mgo_tx_effects) => mgo_tx_effects
                .mutated()
                .iter()
                .map(|refe| (refe.reference.to_object_ref(), refe.owner))
                .collect(),
        }
    }

    pub fn created(&self) -> Vec<(ObjectRef, Owner)> {
        match self {
            ExecutionEffects::CertifiedTransactionEffects(certified_effects, ..) => {
                certified_effects.data().created()
            }
            ExecutionEffects::MgoTransactionBlockEffects(mgo_tx_effects) => mgo_tx_effects
                .created()
                .iter()
                .map(|refe| (refe.reference.to_object_ref(), refe.owner))
                .collect(),
        }
    }

    pub fn deleted(&self) -> Vec<ObjectRef> {
        match self {
            ExecutionEffects::CertifiedTransactionEffects(certified_effects, ..) => {
                certified_effects.data().deleted().to_vec()
            }
            ExecutionEffects::MgoTransactionBlockEffects(mgo_tx_effects) => mgo_tx_effects
                .deleted()
                .iter()
                .map(|refe| refe.to_object_ref())
                .collect(),
        }
    }

    pub fn quorum_sig(&self) -> Option<&AuthorityStrongQuorumSignInfo> {
        match self {
            ExecutionEffects::CertifiedTransactionEffects(certified_effects, ..) => {
                Some(certified_effects.auth_sig())
            }
            ExecutionEffects::MgoTransactionBlockEffects(_) => None,
        }
    }

    pub fn gas_object(&self) -> (ObjectRef, Owner) {
        match self {
            ExecutionEffects::CertifiedTransactionEffects(certified_effects, ..) => {
                certified_effects.data().gas_object()
            }
            ExecutionEffects::MgoTransactionBlockEffects(mgo_tx_effects) => {
                let refe = &mgo_tx_effects.gas_object();
                (refe.reference.to_object_ref(), refe.owner)
            }
        }
    }

    pub fn sender(&self) -> MgoAddress {
        match self.gas_object().1 {
            Owner::AddressOwner(a) => a,
            Owner::ObjectOwner(_) | Owner::Shared { .. } | Owner::Immutable => unreachable!(), // owner of gas object is always an address
        }
    }

    pub fn is_ok(&self) -> bool {
        match self {
            ExecutionEffects::CertifiedTransactionEffects(certified_effects, ..) => {
                certified_effects.data().status().is_ok()
            }
            ExecutionEffects::MgoTransactionBlockEffects(mgo_tx_effects) => {
                mgo_tx_effects.status().is_ok()
            }
        }
    }

    pub fn status(&self) -> String {
        match self {
            ExecutionEffects::CertifiedTransactionEffects(certified_effects, ..) => {
                format!("{:#?}", certified_effects.data().status())
            }
            ExecutionEffects::MgoTransactionBlockEffects(mgo_tx_effects) => {
                format!("{:#?}", mgo_tx_effects.status())
            }
        }
    }

    pub fn gas_cost_summary(&self) -> GasCostSummary {
        match self {
            crate::ExecutionEffects::CertifiedTransactionEffects(a, _) => {
                a.data().gas_cost_summary().clone()
            }
            crate::ExecutionEffects::MgoTransactionBlockEffects(b) => {
                std::convert::Into::<GasCostSummary>::into(b.gas_cost_summary().clone())
            }
        }
    }

    pub fn gas_used(&self) -> u64 {
        self.gas_cost_summary().gas_used()
    }

    pub fn net_gas_used(&self) -> i64 {
        self.gas_cost_summary().net_gas_usage()
    }

    pub fn print_gas_summary(&self) {
        let gas_object = self.gas_object();
        let sender = self.sender();
        let status = self.status();
        let gas_cost_summary = self.gas_cost_summary();
        let gas_used = self.gas_used();
        let net_gas_used = self.net_gas_used();

        info!(
            "Summary:\n\
             Gas Object: {gas_object:?}\n\
             Sender: {sender:?}\n\
             status: {status}\n\
             Gas Cost Summary: {gas_cost_summary:#?}\n\
             Gas Used: {gas_used}\n\
             Net Gas Used: {net_gas_used}"
        );
    }
}

#[async_trait]
pub trait ValidatorProxy {
    async fn get_object(&self, object_id: ObjectID) -> Result<Object, anyhow::Error>;

    async fn get_owned_objects(
        &self,
        account_address: MgoAddress,
    ) -> Result<Vec<(u64, Object)>, anyhow::Error>;

    async fn get_latest_system_state_object(&self) -> Result<MgoSystemStateSummary, anyhow::Error>;

    async fn execute_transaction_block(&self, tx: Transaction) -> anyhow::Result<ExecutionEffects>;

    /// This function is similar to `execute_transaction` but does not check any validator's
    /// signature. It should only be used for benchmarks.
    async fn execute_bench_transaction(&self, tx: Transaction) -> anyhow::Result<ExecutionEffects>;

    fn clone_committee(&self) -> Arc<Committee>;

    fn get_current_epoch(&self) -> EpochId;

    fn clone_new(&self) -> Box<dyn ValidatorProxy + Send + Sync>;

    async fn get_validators(&self) -> Result<Vec<MgoAddress>, anyhow::Error>;
}

// TODO: Eventually remove this proxy because we shouldn't rely on validators to read objects.
pub struct LocalValidatorAggregatorProxy {
    _qd_handler: QuorumDriverHandler<NetworkAuthorityClient>,
    // Stress client does not verify individual validator signatures since this is very expensive
    qd: Arc<QuorumDriver<NetworkAuthorityClient>>,
    committee: Committee,
    clients: BTreeMap<AuthorityName, NetworkAuthorityClient>,
    requests: Mutex<JoinSet<()>>,
}

impl LocalValidatorAggregatorProxy {
    pub async fn from_genesis(
        genesis: &Genesis,
        registry: &Registry,
        reconfig_fullnode_rpc_url: Option<&str>,
    ) -> Self {
        let (aggregator, _) = AuthorityAggregatorBuilder::from_genesis(genesis)
            .with_registry(registry)
            .build()
            .unwrap();

        let committee = genesis.committee_with_network();
        let clients = make_authority_clients_with_timeout_config(
            &committee,
            DEFAULT_CONNECT_TIMEOUT_SEC,
            DEFAULT_REQUEST_TIMEOUT_SEC,
        )
        .unwrap();

        Self::new_impl(
            aggregator,
            registry,
            reconfig_fullnode_rpc_url,
            clients,
            committee.committee,
        )
        .await
    }

    async fn new_impl(
        aggregator: AuthorityAggregator<NetworkAuthorityClient>,
        registry: &Registry,
        reconfig_fullnode_rpc_url: Option<&str>,
        clients: BTreeMap<AuthorityName, NetworkAuthorityClient>,
        committee: Committee,
    ) -> Self {
        let quorum_driver_metrics = Arc::new(QuorumDriverMetrics::new(registry));
        let qd_handler = (if let Some(reconfig_fullnode_rpc_url) = reconfig_fullnode_rpc_url {
            let qd_handler_builder = QuorumDriverHandlerBuilder::new(
                Arc::new(aggregator.clone()),
                quorum_driver_metrics,
            );
            info!(
                "Using FullNodeReconfigObserver: {:?}",
                reconfig_fullnode_rpc_url
            );
            let committee_store = aggregator.clone_committee_store();
            let reconfig_observer = Arc::new(
                FullNodeReconfigObserver::new(
                    reconfig_fullnode_rpc_url,
                    committee_store,
                    aggregator.safe_client_metrics_base.clone(),
                    aggregator.metrics.clone(),
                )
                .await,
            );
            qd_handler_builder.with_reconfig_observer(reconfig_observer)
        } else {
            info!("Using EmbeddedReconfigObserver");
            let observer = EmbeddedReconfigObserver::new();
            // Get the latest committee from config observer
            let new_agg = observer
                .get_committee(Arc::new(aggregator))
                .await
                .expect("Failed to get latest committee");
            let qd_handler_builder =
                QuorumDriverHandlerBuilder::new(new_agg, quorum_driver_metrics);
            qd_handler_builder.with_reconfig_observer(Arc::new(EmbeddedReconfigObserver::new()))
        })
        .start();

        let qd = qd_handler.clone_quorum_driver();
        Self {
            _qd_handler: qd_handler,
            qd,
            clients,
            committee,
            requests: Mutex::new(JoinSet::new()),
        }
    }
}

#[async_trait]
impl ValidatorProxy for LocalValidatorAggregatorProxy {
    async fn get_object(&self, object_id: ObjectID) -> Result<Object, anyhow::Error> {
        let auth_agg = self.qd.authority_aggregator().load();
        Ok(auth_agg
            .get_latest_object_version_for_testing(object_id)
            .await?)
    }

    async fn get_owned_objects(
        &self,
        _account_address: MgoAddress,
    ) -> Result<Vec<(u64, Object)>, anyhow::Error> {
        unimplemented!("Not available for local proxy");
    }

    async fn get_latest_system_state_object(&self) -> Result<MgoSystemStateSummary, anyhow::Error> {
        let auth_agg = self.qd.authority_aggregator().load();
        Ok(auth_agg
            .get_latest_system_state_object_for_testing()
            .await?
            .into_mgo_system_state_summary())
    }

    async fn execute_transaction_block(&self, tx: Transaction) -> anyhow::Result<ExecutionEffects> {
        if std::env::var("BENCH_MODE").is_ok() {
            return self.execute_bench_transaction(tx).await;
        }
        let tx_digest = *tx.digest();
        let mut retry_cnt = 0;
        while retry_cnt < 3 {
            let ticket = self.qd.submit_transaction(tx.clone()).await?;
            // The ticket only times out when QuorumDriver exceeds the retry times
            match ticket.await {
                Ok(resp) => {
                    let QuorumDriverResponse {
                        effects_cert,
                        events,
                    } = resp;
                    return Ok(ExecutionEffects::CertifiedTransactionEffects(
                        effects_cert.into(),
                        events,
                    ));
                }
                Err(QuorumDriverError::NonRecoverableTransactionError { errors }) => {
                    bail!(QuorumDriverError::NonRecoverableTransactionError { errors });
                }
                Err(err) => {
                    let delay = Duration::from_millis(rand::thread_rng().gen_range(100..1000));
                    error!(
                        ?tx_digest,
                        retry_cnt,
                        "Transaction failed with err: {:?}. Sleeping for {:?} ...",
                        err,
                        delay,
                    );
                    retry_cnt += 1;
                    sleep(delay).await;
                }
            }
        }
        bail!("Transaction {:?} failed for {retry_cnt} times", tx_digest);
    }

    async fn execute_bench_transaction(&self, tx: Transaction) -> anyhow::Result<ExecutionEffects> {
        // Store the epoch number; we read it from the votes and use it later to create the certificate.
        let mut epoch = 0;
        let auth_agg = self.qd.authority_aggregator().load();

        // Send the transaction to all validators.
        let tx_guard = GaugeGuard::acquire(&auth_agg.metrics.inflight_transactions);
        let mut futures = FuturesUnordered::new();
        for (name, client) in self.clients.iter() {
            let fut = client.handle_transaction(tx.clone()).map(|r| (r, *name));
            futures.push(fut);
        }
        auth_agg
            .metrics
            .inflight_transaction_requests
            .add(futures.len() as i64);

        // TODO: This following aggregation will not work well at epoch boundary.

        // Listen to the replies from the first 2f+1 votes.
        let mut total_stake = 0;
        let mut votes = Vec::new();
        let mut certificate = None;
        while let Some((response, name)) = futures.next().await {
            auth_agg.metrics.inflight_transaction_requests.dec();
            match response {
                Ok(response) => match response.status {
                    // If all goes well, the authority returns a vote.
                    TransactionStatus::Signed(signature) => {
                        epoch = signature.epoch;
                        total_stake += self.committee.weight(&signature.authority);
                        votes.push(signature);
                    }
                    // The transaction may be submitted again in case the certificate's submission failed.
                    TransactionStatus::Executed(cert, _effects, _) => {
                        tracing::warn!("Transaction already submitted: {tx:?}");
                        if let Some(cert) = cert {
                            certificate = Some(CertifiedTransaction::new_from_data_and_sig(
                                tx.data().clone(),
                                cert,
                            ));
                        }
                    }
                },
                // This typically happens when the validators are overloaded and the transaction is
                // immediately rejected.
                Err(e) => {
                    self.qd
                        .authority_aggregator()
                        .load()
                        .metrics
                        .process_tx_errors
                        .with_label_values(&[&name.concise().to_string(), e.as_ref()])
                        .inc();
                    tracing::warn!("Failed to submit transaction: {e}")
                }
            }

            if total_stake >= self.committee.quorum_threshold() {
                break;
            }

            if certificate.is_some() {
                break;
            }
        }

        // Assemble a certificate from the validator's replies.
        let certified_transaction: CertifiedTransaction = match certificate {
            Some(x) => x,
            None => {
                let signatures: BTreeMap<_, _> = votes
                    .into_iter()
                    .map(|a| (a.authority, a.signature))
                    .collect();
                let mut signers_map = RoaringBitmap::new();
                for pk in signatures.keys() {
                    signers_map.insert(
                        self.committee
                            .authority_index(pk)
                            .ok_or(MgoError::UnknownSigner {
                                signer: Some(pk.concise().to_string()),
                                index: None,
                                committee: Box::new(self.committee.clone()),
                            })
                            .expect("Received signature from unknown validator"),
                    );
                }
                let sigs: Vec<AuthoritySignature> = signatures.into_values().collect();

                let quorum_signature = AuthorityQuorumSignInfo {
                    epoch,
                    // Note: This function simply aggregates signatures (it does not check that they
                    // are individually valid).
                    signature: AggregateAuthoritySignature::aggregate(&sigs)
                        .map_err(|e| MgoError::InvalidSignature {
                            error: e.to_string(),
                        })
                        .expect("Validator returned invalid signature"),
                    signers_map,
                };

                Envelope::new_from_data_and_sig(tx.into_data(), quorum_signature)
            }
        };
        auth_agg
            .metrics
            .inflight_transaction_requests
            .sub(futures.len() as i64);
        drop(tx_guard);

        // Send the certificate to all validators.
        let _cert_guard = GaugeGuard::acquire(&auth_agg.metrics.inflight_certificates);
        let mut futures = FuturesUnordered::new();
        total_stake = 0;
        let mut transaction_effects = None;
        let mut transaction_events = None;
        for (name, client) in self.clients.iter() {
            let client = client.clone();
            let certificate = certified_transaction.clone();
            let name = *name;
            futures.push(async move {
                client
                    .handle_certificate_v2(certificate)
                    .map(move |r| (r, name))
                    .await
            });
        }
        auth_agg
            .metrics
            .inflight_certificate_requests
            .add(futures.len() as i64);

        // Wait for the replies from a quorum of validators.
        while let Some((response, name)) = futures.next().await {
            auth_agg.metrics.inflight_certificate_requests.dec();
            match response {
                // If all goes well, the validators reply with signed effects.
                Ok(HandleCertificateResponseV2 {
                    signed_effects,
                    events,
                    fastpath_input_objects: _, // unused field
                }) => {
                    let author = signed_effects.auth_sig().authority;
                    transaction_effects = Some(signed_effects.data().clone());
                    transaction_events = Some(events);
                    total_stake += self.committee.weight(&author);
                }

                // This typically happens when the validators are overloaded and the certificate is
                // immediately rejected.
                Err(e) => {
                    auth_agg
                        .metrics
                        .process_cert_errors
                        .with_label_values(&[&name.concise().to_string(), e.as_ref()])
                        .inc();
                    tracing::warn!("Failed to submit certificate: {e}")
                }
            }

            if total_stake >= self.committee.quorum_threshold() {
                break;
            }
        }

        // Abort if we failed to submit the certificate to enough validators. This typically
        // happens when the validators are overloaded and the requests timed out.
        if transaction_effects.is_none() || total_stake < self.committee.quorum_threshold() {
            bail!("Failed to submit certificate to quorum of validators");
        }

        // Wait for 10 more seconds on remaining requests asynchronously.
        const WAIT_TIMEOUT: Duration = Duration::from_secs(10);
        {
            let auth_agg = auth_agg.clone();
            let mut requests = self.requests.lock().unwrap();
            requests.spawn(async move {
                let _ = timeout(WAIT_TIMEOUT, async {
                    while futures.next().await.is_some() {
                        auth_agg.metrics.inflight_certificate_requests.dec();
                    }
                })
                .await;
                auth_agg
                    .metrics
                    .inflight_certificate_requests
                    .sub(futures.len() as i64);
            });
        }

        // Package the certificate and effects to return.
        let signed_material = certified_transaction.auth_sig().clone();
        let effects = ExecutionEffects::CertifiedTransactionEffects(
            Envelope::new_from_data_and_sig(transaction_effects.unwrap(), signed_material),
            transaction_events.unwrap(),
        );
        Ok(effects)
    }

    fn clone_committee(&self) -> Arc<Committee> {
        self.qd.clone_committee()
    }

    fn get_current_epoch(&self) -> EpochId {
        self.qd.current_epoch()
    }

    fn clone_new(&self) -> Box<dyn ValidatorProxy + Send + Sync> {
        let qdh = self._qd_handler.clone_new();
        let qd = qdh.clone_quorum_driver();
        Box::new(Self {
            _qd_handler: qdh,
            qd,
            clients: self.clients.clone(),
            committee: self.committee.clone(),
            requests: Mutex::new(JoinSet::new()),
        })
    }

    async fn get_validators(&self) -> Result<Vec<MgoAddress>, anyhow::Error> {
        let system_state = self.get_latest_system_state_object().await?;
        Ok(system_state
            .active_validators
            .iter()
            .map(|v| v.mgo_address)
            .collect())
    }
}

pub struct FullNodeProxy {
    mgo_client: MgoClient,
    committee: Arc<Committee>,
}

impl FullNodeProxy {
    pub async fn from_url(http_url: &str) -> Result<Self, anyhow::Error> {
        // Each request times out after 60s (default value)
        let mgo_client = MgoClientBuilder::default()
            .max_concurrent_requests(500_000)
            .build(http_url)
            .await?;

        let resp = mgo_client.read_api().get_committee_info(None).await?;
        let epoch = resp.epoch;
        let committee_vec = resp.validators;
        let committee_map = BTreeMap::from_iter(committee_vec.into_iter());
        let committee =
            Committee::new_for_testing_with_normalized_voting_power(epoch, committee_map);

        Ok(Self {
            mgo_client,
            committee: Arc::new(committee),
        })
    }
}

#[async_trait]
impl ValidatorProxy for FullNodeProxy {
    async fn get_object(&self, object_id: ObjectID) -> Result<Object, anyhow::Error> {
        let response = self
            .mgo_client
            .read_api()
            .get_object_with_options(object_id, MgoObjectDataOptions::bcs_lossless())
            .await?;

        if let Some(mgo_object) = response.data {
            mgo_object.try_into()
        } else if let Some(error) = response.error {
            bail!("Error getting object {:?}: {}", object_id, error)
        } else {
            bail!("Object {:?} not found and no error provided", object_id)
        }
    }

    async fn get_owned_objects(
        &self,
        account_address: MgoAddress,
    ) -> Result<Vec<(u64, Object)>, anyhow::Error> {
        let mut objects: Vec<MgoObjectResponse> = Vec::new();
        let mut cursor = None;
        loop {
            let response = self
                .mgo_client
                .read_api()
                .get_owned_objects(
                    account_address,
                    Some(MgoObjectResponseQuery::new_with_options(
                        MgoObjectDataOptions::bcs_lossless(),
                    )),
                    cursor,
                    None,
                )
                .await?;

            objects.extend(response.data);

            if response.has_next_page {
                cursor = response.next_cursor;
            } else {
                break;
            }
        }

        let mut values_objects = Vec::new();

        for object in objects {
            let o = object.data;
            if let Some(o) = o {
                let temp: Object = o.clone().try_into()?;
                let gas_coin = GasCoin::try_from(&temp)?;
                values_objects.push((gas_coin.value(), o.clone().try_into()?));
            }
        }

        Ok(values_objects)
    }

    async fn get_latest_system_state_object(&self) -> Result<MgoSystemStateSummary, anyhow::Error> {
        Ok(self
            .mgo_client
            .governance_api()
            .get_latest_mgo_system_state()
            .await?)
    }

    async fn execute_transaction_block(&self, tx: Transaction) -> anyhow::Result<ExecutionEffects> {
        let tx_digest = *tx.digest();
        let mut retry_cnt = 0;
        while retry_cnt < 10 {
            // Fullnode could time out after WAIT_FOR_FINALITY_TIMEOUT (30s) in TransactionOrchestrator
            // MgoClient times out after 60s
            match self
                .mgo_client
                .quorum_driver_api()
                .execute_transaction_block(
                    tx.clone(),
                    MgoTransactionBlockResponseOptions::new().with_effects(),
                    None,
                )
                .await
            {
                Ok(resp) => {
                    let effects = ExecutionEffects::MgoTransactionBlockEffects(
                        resp.effects.expect("effects field should not be None"),
                    );
                    return Ok(effects);
                }
                Err(err) => {
                    error!(
                        ?tx_digest,
                        retry_cnt, "Transaction failed with err: {:?}", err
                    );
                    retry_cnt += 1;
                }
            }
        }
        bail!("Transaction {:?} failed for {retry_cnt} times", tx_digest);
    }

    async fn execute_bench_transaction(&self, tx: Transaction) -> anyhow::Result<ExecutionEffects> {
        self.execute_transaction_block(tx).await
    }

    fn clone_committee(&self) -> Arc<Committee> {
        self.committee.clone()
    }

    fn get_current_epoch(&self) -> EpochId {
        self.committee.epoch
    }

    fn clone_new(&self) -> Box<dyn ValidatorProxy + Send + Sync> {
        Box::new(Self {
            mgo_client: self.mgo_client.clone(),
            committee: self.clone_committee(),
        })
    }

    async fn get_validators(&self) -> Result<Vec<MgoAddress>, anyhow::Error> {
        let validators = self
            .mgo_client
            .governance_api()
            .get_latest_mgo_system_state()
            .await?
            .active_validators;
        Ok(validators.into_iter().map(|v| v.mgo_address).collect())
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum BenchMoveCallArg {
    Pure(Vec<u8>),
    Shared((ObjectID, SequenceNumber, bool)),
    ImmOrOwnedObject(ObjectRef),
    ImmOrOwnedObjectVec(Vec<ObjectRef>),
    SharedObjectVec(Vec<(ObjectID, SequenceNumber, bool)>),
}

impl From<bool> for BenchMoveCallArg {
    fn from(b: bool) -> Self {
        // unwrap safe because every u8 value is BCS-serializable
        BenchMoveCallArg::Pure(bcs::to_bytes(&b).unwrap())
    }
}

impl From<u8> for BenchMoveCallArg {
    fn from(n: u8) -> Self {
        // unwrap safe because every u8 value is BCS-serializable
        BenchMoveCallArg::Pure(bcs::to_bytes(&n).unwrap())
    }
}

impl From<u16> for BenchMoveCallArg {
    fn from(n: u16) -> Self {
        // unwrap safe because every u16 value is BCS-serializable
        BenchMoveCallArg::Pure(bcs::to_bytes(&n).unwrap())
    }
}

impl From<u32> for BenchMoveCallArg {
    fn from(n: u32) -> Self {
        // unwrap safe because every u32 value is BCS-serializable
        BenchMoveCallArg::Pure(bcs::to_bytes(&n).unwrap())
    }
}

impl From<u64> for BenchMoveCallArg {
    fn from(n: u64) -> Self {
        // unwrap safe because every u64 value is BCS-serializable
        BenchMoveCallArg::Pure(bcs::to_bytes(&n).unwrap())
    }
}

impl From<u128> for BenchMoveCallArg {
    fn from(n: u128) -> Self {
        // unwrap safe because every u128 value is BCS-serializable
        BenchMoveCallArg::Pure(bcs::to_bytes(&n).unwrap())
    }
}

impl From<&Vec<u8>> for BenchMoveCallArg {
    fn from(v: &Vec<u8>) -> Self {
        // unwrap safe because every vec<u8> value is BCS-serializable
        BenchMoveCallArg::Pure(bcs::to_bytes(v).unwrap())
    }
}

impl From<ObjectRef> for BenchMoveCallArg {
    fn from(obj: ObjectRef) -> Self {
        BenchMoveCallArg::ImmOrOwnedObject(obj)
    }
}

impl From<CallArg> for BenchMoveCallArg {
    fn from(ca: CallArg) -> Self {
        match ca {
            CallArg::Pure(p) => BenchMoveCallArg::Pure(p),
            CallArg::Object(obj) => match obj {
                ObjectArg::ImmOrOwnedObject(imo) => BenchMoveCallArg::ImmOrOwnedObject(imo),
                ObjectArg::SharedObject {
                    id,
                    initial_shared_version,
                    mutable,
                } => BenchMoveCallArg::Shared((id, initial_shared_version, mutable)),
                ObjectArg::Receiving(_) => {
                    unimplemented!("Receiving is not supported for benchmarks")
                }
            },
        }
    }
}

/// Convert MoveCallArg to Vector of Argument for PT
pub fn convert_move_call_args(
    args: &[BenchMoveCallArg],
    pt_builder: &mut ProgrammableTransactionBuilder,
) -> Vec<Argument> {
    args.iter()
        .map(|arg| match arg {
            BenchMoveCallArg::Pure(bytes) => {
                pt_builder.input(CallArg::Pure(bytes.clone())).unwrap()
            }
            BenchMoveCallArg::Shared((id, initial_shared_version, mutable)) => pt_builder
                .input(CallArg::Object(ObjectArg::SharedObject {
                    id: *id,
                    initial_shared_version: *initial_shared_version,
                    mutable: *mutable,
                }))
                .unwrap(),
            BenchMoveCallArg::ImmOrOwnedObject(obj_ref) => {
                pt_builder.input((*obj_ref).into()).unwrap()
            }
            BenchMoveCallArg::ImmOrOwnedObjectVec(obj_refs) => pt_builder
                .make_obj_vec(obj_refs.iter().map(|q| ObjectArg::ImmOrOwnedObject(*q)))
                .unwrap(),
            BenchMoveCallArg::SharedObjectVec(obj_refs) => pt_builder
                .make_obj_vec(
                    obj_refs
                        .iter()
                        .map(
                            |(id, initial_shared_version, mutable)| ObjectArg::SharedObject {
                                id: *id,
                                initial_shared_version: *initial_shared_version,
                                mutable: *mutable,
                            },
                        ),
                )
                .unwrap(),
        })
        .collect()
}
