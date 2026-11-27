use serde::Deserialize;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

const MAX_SAFE_INTEGER: u64 = 9_007_199_254_740_991;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Ingest {
    pub stable_route_id: String,
    pub requested_route_revision: u64,
    pub source: Source,
    pub payload: Payload,
    pub fixture_authorization: Authorization,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Source {
    pub source_genesis_hash: String,
    pub source_contract: String,
    pub business_event_id: String,
    pub source_transaction_hash: String,
    pub source_block_hash: String,
    pub source_height: u64,
    pub source_log_index: u64,
    pub observed_at_unix_ms: u64,
    pub is_simulated: bool,
    pub evidence_label: String,
    pub emitted_payload_digest: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Payload {
    pub action: String,
    pub campaign_id: String,
    pub entitlement_code: String,
    pub recipient_mango: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Authorization {
    pub mode: String,
    pub policy_version: u64,
    pub authorization_expires_at_unix_s: u64,
    pub authorization_digest: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Retry {
    pub action_id: String,
    pub expected_task_revision: u64,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Domain {
    pub source_genesis_hash: String,
    pub source_contract: String,
    pub target_genesis_digest: String,
    pub target_package_id: String,
    pub recipient: String,
    pub route_revision: u64,
}

pub fn hex_id(value: &str, bytes: usize) -> bool {
    value.strip_prefix("0x").is_some_and(|s| {
        s.len() == bytes * 2
            && s.bytes()
                .all(|c| c.is_ascii_digit() || (b'a'..=b'f').contains(&c))
    })
}

pub fn bounded_integer(value: u64) -> bool {
    (1..=MAX_SAFE_INTEGER).contains(&value)
}

pub fn valid_action_id(value: &str) -> bool {
    (1..=128).contains(&value.len())
        && value.as_bytes()[0].is_ascii_alphanumeric()
        && value
            .bytes()
            .all(|c| c.is_ascii_alphanumeric() || b"._:-".contains(&c))
}

fn printable(value: &str) -> bool {
    (1..=64).contains(&value.len()) && value.bytes().all(|b| (0x21..=0x7e).contains(&b))
}

fn digest(fields: &[&str]) -> String {
    let mut hasher = Sha256::new();
    for field in fields {
        hasher.update((field.len() as u32).to_be_bytes());
        hasher.update(field.as_bytes());
    }
    format!("0x{}", hex::encode(hasher.finalize()))
}

impl Domain {
    pub fn validate(&self) -> bool {
        self.source_genesis_hash != self.target_genesis_digest
            && hex_id(&self.source_genesis_hash, 32)
            && hex_id(&self.source_contract, 20)
            && hex_id(&self.target_genesis_digest, 32)
            && hex_id(&self.target_package_id, 32)
            && hex_id(&self.recipient, 32)
            && bounded_integer(self.route_revision)
    }
}

impl Ingest {
    pub fn task(&self, domain: &Domain, now: u64) -> Result<Value, &'static str> {
        let source = &self.source;
        let payload = &self.payload;
        let auth = &self.fixture_authorization;
        if self.stable_route_id != "demo-entitlement" {
            return Err("UNKNOWN_ROUTE");
        }
        if !bounded_integer(self.requested_route_revision)
            || !hex_id(&source.business_event_id, 32)
            || !hex_id(&source.source_transaction_hash, 32)
            || !hex_id(&source.source_block_hash, 32)
            || source.source_height > MAX_SAFE_INTEGER
            || source.observed_at_unix_ms > MAX_SAFE_INTEGER
            || source.source_log_index > i32::MAX as u64
            || !printable(&payload.campaign_id)
            || !printable(&payload.entitlement_code)
            || !hex_id(&auth.authorization_digest, 32)
        {
            return Err("INVALID_REQUEST");
        }
        if source.source_genesis_hash != domain.source_genesis_hash
            || source.source_contract != domain.source_contract
            || !source.is_simulated
            || source.evidence_label != "LOCAL_FIXTURE"
            || auth.mode != "FIXTURE_ONLY_NO_SIGNATURE"
            || auth.policy_version != 1
            || !bounded_integer(auth.authorization_expires_at_unix_s)
            || auth.authorization_expires_at_unix_s <= now
            || auth.authorization_expires_at_unix_s > now + 60
            || payload.action != "issue_test_entitlement"
            || payload.recipient_mango != domain.recipient
        {
            return Err("FIXTURE_POLICY_REJECTED");
        }
        let task_id = digest(&[
            "mango-business-task-v1",
            &source.source_genesis_hash,
            &source.source_contract,
            &source.business_event_id,
            &self.stable_route_id,
        ]);
        let payload_digest = digest(&[
            "mango-entitlement-payload-v1",
            &payload.action,
            &payload.campaign_id,
            &payload.entitlement_code,
            &payload.recipient_mango,
        ]);
        let authorization_digest = digest(&[
            "mango-business-auth-v1",
            &task_id,
            &payload_digest,
            &self.stable_route_id,
            &self.requested_route_revision.to_string(),
            &domain.target_genesis_digest,
            &domain.target_package_id,
            &payload.action,
            &auth.authorization_expires_at_unix_s.to_string(),
            &auth.policy_version.to_string(),
        ]);
        if source.emitted_payload_digest != payload_digest
            || authorization_digest != auth.authorization_digest
        {
            return Err("FIXTURE_POLICY_REJECTED");
        }
        Ok(json!({
            "task_id": task_id,
            "source_genesis_hash": source.source_genesis_hash,
            "source_contract": source.source_contract,
            "business_event_id": source.business_event_id,
            "stable_route_id": self.stable_route_id,
            "route_revision_snapshot": self.requested_route_revision,
            "route_snapshot": {
                "target_genesis_digest": domain.target_genesis_digest,
                "target_package_id": domain.target_package_id,
                "action": payload.action
            },
            "policy_snapshot": {"mode": auth.mode, "policy_version": auth.policy_version},
            "business_payload": {
                "action": payload.action,
                "recipient_mango": payload.recipient_mango,
                "campaign_id": payload.campaign_id,
                "entitlement_code": payload.entitlement_code
            },
            "business_payload_digest": payload_digest,
            "state": "WAITING_SOURCE",
        }))
    }
}
