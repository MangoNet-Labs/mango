// Copyright (c) MangoNet Labs Ltd.
// SPDX-License-Identifier: Apache-2.0

use clap::*;
use mgo_protocol_config_macros::{ProtocolConfigAccessors, ProtocolConfigFeatureFlagsGetters};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::cell::RefCell;
use std::collections::BTreeSet;
use std::sync::atomic::{AtomicBool, Ordering};
use tracing::{info, warn};

/// The minimum and maximum protocol versions supported by this build.
const MIN_PROTOCOL_VERSION: u64 = 1;
const MAX_PROTOCOL_VERSION: u64 = 3;

// Record history of protocol version allocations here:
//
// Version 1: Original version.
#[derive(Copy, Clone, Debug, Hash, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProtocolVersion(u64);

impl ProtocolVersion {
    // The minimum and maximum protocol version supported by this binary. Counterintuitively, this constant may
    // change over time as support for old protocol versions is removed from the source. This
    // ensures that when a new network (such as a testnet) is created, its genesis committee will
    // use a protocol version that is actually supported by the binary.
    pub const MIN: Self = Self(MIN_PROTOCOL_VERSION);

    pub const MAX: Self = Self(MAX_PROTOCOL_VERSION);

    #[cfg(not(msim))]
    const MAX_ALLOWED: Self = Self::MAX;

    // We create one additional "fake" version in simulator builds so that we can test upgrades.
    #[cfg(msim)]
    pub const MAX_ALLOWED: Self = Self(MAX_PROTOCOL_VERSION + 1);

    pub fn new(v: u64) -> Self {
        Self(v)
    }

    pub const fn as_u64(&self) -> u64 {
        self.0
    }

    // For serde deserialization - we don't define a Default impl because there isn't a single
    // universally appropriate default value.
    pub fn max() -> Self {
        Self::MAX
    }
}

impl From<u64> for ProtocolVersion {
    fn from(v: u64) -> Self {
        Self::new(v)
    }
}

impl std::ops::Sub<u64> for ProtocolVersion {
    type Output = Self;
    fn sub(self, rhs: u64) -> Self::Output {
        Self::new(self.0 - rhs)
    }
}

impl std::ops::Add<u64> for ProtocolVersion {
    type Output = Self;
    fn add(self, rhs: u64) -> Self::Output {
        Self::new(self.0 + rhs)
    }
}

/// Models the set of protocol versions supported by a validator.
/// The `mgo-node` binary will always use the SYSTEM_DEFAULT constant, but for testing we need
/// to be able to inject arbitrary versions into MgoNode.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct SupportedProtocolVersions {
    pub min: ProtocolVersion,
    pub max: ProtocolVersion,
}

impl SupportedProtocolVersions {
    pub const SYSTEM_DEFAULT: Self = Self {
        min: ProtocolVersion::MIN,
        max: ProtocolVersion::MAX,
    };

    /// Use by VersionedProtocolMessage implementors to describe in which range of versions a
    /// message variant is supported.
    pub fn new_for_message(min: u64, max: u64) -> Self {
        let min = ProtocolVersion::new(min);
        let max = ProtocolVersion::new(max);
        Self { min, max }
    }

    pub fn new_for_testing(min: u64, max: u64) -> Self {
        let min = min.into();
        let max = max.into();
        Self { min, max }
    }

    pub fn is_version_supported(&self, v: ProtocolVersion) -> bool {
        v.0 >= self.min.0 && v.0 <= self.max.0
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Copy, PartialOrd, Ord, Eq, ValueEnum)]
pub enum Chain {
    Mainnet,
    Testnet,
    Unknown,
}

impl Default for Chain {
    fn default() -> Self {
        Self::Unknown
    }
}

pub struct Error(pub String);

/// Records on/off feature flags that may vary at each protocol version.
#[derive(Default, Clone, Serialize, Debug, ProtocolConfigFeatureFlagsGetters)]
struct FeatureFlags {
    // Add feature flags here, e.g.:
    // new_protocol_feature: bool,
    #[serde(skip_serializing_if = "is_false")]
    package_upgrades: bool,
    // If true, validators will commit to the root state digest
    // in end of epoch checkpoint proposals
    #[serde(skip_serializing_if = "is_false")]
    commit_root_state_digest: bool,
    // Pass epoch start time to advance_epoch safe mode function.
    #[serde(skip_serializing_if = "is_false")]
    advance_epoch_start_time_in_safe_mode: bool,
    // If true, apply the fix to correctly capturing loaded child object versions in execution's
    // object runtime.
    #[serde(skip_serializing_if = "is_false")]
    loaded_child_objects_fixed: bool,
    // If true, treat missing types in the upgraded modules when creating an upgraded package as a
    // compatibility error.
    #[serde(skip_serializing_if = "is_false")]
    missing_type_is_compatibility_error: bool,
    // If true, then the scoring decision mechanism will not get disabled when we do have more than
    // f low scoring authorities, but it will simply flag as low scoring only up to f authorities.
    #[serde(skip_serializing_if = "is_false")]
    scoring_decision_with_validity_cutoff: bool,

    // DEPRECATED: this was an ephemeral feature flag only used by consensus handler, which has now
    // been deployed everywhere.
    #[serde(skip_serializing_if = "is_false")]
    consensus_order_end_of_epoch_last: bool,

    // Disallow adding abilities to types during package upgrades.
    #[serde(skip_serializing_if = "is_false")]
    disallow_adding_abilities_on_upgrade: bool,
    // Disables unnecessary invariant check in the Move VM when swapping the value out of a local
    #[serde(skip_serializing_if = "is_false")]
    disable_invariant_violation_check_in_swap_loc: bool,
    // advance to highest supported protocol version at epoch change, instead of the next consecutive
    // protocol version.
    #[serde(skip_serializing_if = "is_false")]
    advance_to_highest_supported_protocol_version: bool,
    // If true, disallow entry modifiers on entry functions
    #[serde(skip_serializing_if = "is_false")]
    ban_entry_init: bool,
    // If true, hash module bytes individually when calculating package digests for upgrades
    #[serde(skip_serializing_if = "is_false")]
    package_digest_hash_module: bool,
    // If true, disallow changing struct type parameters during package upgrades
    #[serde(skip_serializing_if = "is_false")]
    disallow_change_struct_type_params_on_upgrade: bool,
    // If true, checks no extra bytes in a compiled module
    #[serde(skip_serializing_if = "is_false")]
    no_extraneous_module_bytes: bool,
    // If true, then use the versioned metadata format in narwhal entities.
    #[serde(skip_serializing_if = "is_false")]
    narwhal_versioned_metadata: bool,

    // Enable zklogin auth
    #[serde(skip_serializing_if = "is_false")]
    zklogin_auth: bool,
    // How we order transactions coming out of consensus before sending to execution.
    #[serde(skip_serializing_if = "ConsensusTransactionOrdering::is_none")]
    consensus_transaction_ordering: ConsensusTransactionOrdering,

    // Previously, the unwrapped_then_deleted field in TransactionEffects makes a distinction between
    // whether an object has existed in the store previously (i.e. whether there is a tombstone).
    // Such dependency makes effects generation inefficient, and requires us to include wrapped
    // tombstone in state root hash.
    // To prepare for effects V2, with this flag set to true, we simplify the definition of
    // unwrapped_then_deleted to always include unwrapped then deleted objects,
    // regardless of their previous state in the store.
    #[serde(skip_serializing_if = "is_false")]
    simplified_unwrap_then_delete: bool,
    // Enable upgraded multisig support
    #[serde(skip_serializing_if = "is_false")]
    upgraded_multisig_supported: bool,
    // If true minimum txn charge is a multiplier of the gas price
    #[serde(skip_serializing_if = "is_false")]
    txn_base_cost_as_multiplier: bool,

    // If true, the ability to delete shared objects is in effect
    #[serde(skip_serializing_if = "is_false")]
    shared_object_deletion: bool,

    // If true, then the new algorithm for the leader election schedule will be used
    #[serde(skip_serializing_if = "is_false")]
    narwhal_new_leader_election_schedule: bool,

    // A list of supported OIDC providers that can be used for zklogin.
    #[serde(skip_serializing_if = "is_empty")]
    zklogin_supported_providers: BTreeSet<String>,

    // If true, use the new child object format
    #[serde(skip_serializing_if = "is_false")]
    loaded_child_object_format: bool,

    #[serde(skip_serializing_if = "is_false")]
    enable_jwk_consensus_updates: bool,

    #[serde(skip_serializing_if = "is_false")]
    end_of_epoch_transaction_supported: bool,

    // Perform simple conservation checks keeping into account out of gas scenarios
    // while charging for storage.
    #[serde(skip_serializing_if = "is_false")]
    simple_conservation_checks: bool,

    // If true, use the new child object format type logging
    #[serde(skip_serializing_if = "is_false")]
    loaded_child_object_format_type: bool,

    // Enable receiving sent objects
    #[serde(skip_serializing_if = "is_false")]
    receive_objects: bool,

    // Enable v2 of Headers for Narwhal
    #[serde(skip_serializing_if = "is_false")]
    narwhal_header_v2: bool,

    // Enable random beacon protocol
    #[serde(skip_serializing_if = "is_false")]
    random_beacon: bool,

    #[serde(skip_serializing_if = "is_false")]
    enable_effects_v2: bool,

    // If true, then use CertificateV2 in narwhal.
    #[serde(skip_serializing_if = "is_false")]
    narwhal_certificate_v2: bool,

    // If true, allow verify with legacy zklogin address
    #[serde(skip_serializing_if = "is_false")]
    verify_legacy_zklogin_address: bool,

    // Enable throughput aware consensus submission
    #[serde(skip_serializing_if = "is_false")]
    throughput_aware_consensus_submission: bool,

    // If true, recompute has_public_transfer from the type instead of what is stored in the object
    #[serde(skip_serializing_if = "is_false")]
    recompute_has_public_transfer_in_execution: bool,

    // If true, multisig containing zkLogin sig is accepted.
    #[serde(skip_serializing_if = "is_false")]
    accept_zklogin_in_multisig: bool,

    // If true, consensus prologue transaction also includes the consensus output digest.
    // It can be used to detect consensus output folk.
    #[serde(skip_serializing_if = "is_false")]
    include_consensus_digest_in_prologue: bool,

    // If true, use the hardened OTW check
    #[serde(skip_serializing_if = "is_false")]
    hardened_otw_check: bool,

    // If true allow calling receiving_object_id function
    #[serde(skip_serializing_if = "is_false")]
    allow_receiving_object_id: bool,

    // Enable the poseidon hash function
    #[serde(skip_serializing_if = "is_false")]
    enable_poseidon: bool,

    // If true, enable the coin deny list.
    #[serde(skip_serializing_if = "is_false")]
    enable_coin_deny_list: bool,

    // Enable native functions for group operations.
    #[serde(skip_serializing_if = "is_false")]
    enable_group_ops_native_functions: bool,
}

fn is_false(b: &bool) -> bool {
    !b
}

fn is_empty(b: &BTreeSet<String>) -> bool {
    b.is_empty()
}

/// Ordering mechanism for transactions in one Narwhal consensus output.
#[derive(Default, Copy, Clone, PartialEq, Eq, Serialize, Debug)]
pub enum ConsensusTransactionOrdering {
    /// No ordering. Transactions are processed in the order they appear in the consensus output.
    #[default]
    None,
    /// Order transactions by gas price, highest first.
    ByGasPrice,
}

impl ConsensusTransactionOrdering {
    pub fn is_none(&self) -> bool {
        matches!(self, ConsensusTransactionOrdering::None)
    }
}

/// Constants that change the behavior of the protocol.
///
/// The value of each constant here must be fixed for a given protocol version. To change the value
/// of a constant, advance the protocol version, and add support for it in `get_for_version` under
/// the new version number.
/// (below).
///
/// To add a new field to this struct, use the following procedure:
/// - Advance the protocol version.
/// - Add the field as a private `Option<T>` to the struct.
/// - Initialize the field to `None` in prior protocol versions.
/// - Initialize the field to `Some(val)` for your new protocol version.
/// - Add a public getter that simply unwraps the field.
/// - Two public getters of the form `field(&self) -> field_type`
///     and `field_as_option(&self) -> Option<field_type>` will be automatically generated for you.
/// Example for a field: `new_constant: Option<u64>`
/// ```rust,ignore
///      pub fn new_constant(&self) -> u64 {
///         self.new_constant.expect(Self::CONSTANT_ERR_MSG)
///     }
///      pub fn new_constant_as_option(&self) -> Option<u64> {
///         self.new_constant.expect(Self::CONSTANT_ERR_MSG)
///     }
/// ```
/// With `pub fn new_constant(&self) -> u64`, if the constant is accessed in a protocol version
/// in which it is not defined, the validator will crash. (Crashing is necessary because
/// this type of error would almost always result in forking if not prevented here).
/// If you don't want the validator to crash, you can use the
/// `pub fn new_constant_as_option(&self) -> Option<u64>` getter, which will
/// return `None` if the field is not defined at that version.
/// - If you want a customized getter, you can add a method in the impl.
#[skip_serializing_none]
#[derive(Clone, Serialize, Debug, ProtocolConfigAccessors)]
pub struct ProtocolConfig {
    pub version: ProtocolVersion,

    feature_flags: FeatureFlags,

    // ==== Transaction input limits ====
    /// Maximum serialized size of a transaction (in bytes).
    max_tx_size_bytes: Option<u64>,

    /// Maximum number of input objects to a transaction. Enforced by the transaction input checker
    max_input_objects: Option<u64>,

    /// Max size of objects a transaction can write to disk after completion. Enforce by the Mgo adapter.
    /// This is the sum of the serialized size of all objects written to disk.
    /// The max size of individual objects on the other hand is `max_move_object_size`.
    max_size_written_objects: Option<u64>,
    /// Max size of objects a system transaction can write to disk after completion. Enforce by the Mgo adapter.
    /// Similar to `max_size_written_objects` but for system transactions.
    max_size_written_objects_system_tx: Option<u64>,

    /// Maximum size of serialized transaction effects.
    max_serialized_tx_effects_size_bytes: Option<u64>,

    /// Maximum size of serialized transaction effects for system transactions.
    max_serialized_tx_effects_size_bytes_system_tx: Option<u64>,

    /// Maximum number of gas payment objects for a transaction.
    max_gas_payment_objects: Option<u32>,

    /// Maximum number of modules in a Publish transaction.
    max_modules_in_publish: Option<u32>,

    /// Maximum number of arguments in a move call or a ProgrammableTransaction's
    /// TransferObjects command.
    max_arguments: Option<u32>,

    /// Maximum number of total type arguments, computed recursively.
    max_type_arguments: Option<u32>,

    /// Maximum depth of an individual type argument.
    max_type_argument_depth: Option<u32>,

    /// Maximum size of a Pure CallArg.
    max_pure_argument_size: Option<u32>,

    /// Maximum number of Commands in a ProgrammableTransaction.
    max_programmable_tx_commands: Option<u32>,

    // ==== Move VM, Move bytecode verifier, and execution limits ===
    /// Maximum Move bytecode version the VM understands. All older versions are accepted.
    move_binary_format_version: Option<u32>,

    /// Maximum size of the `contents` part of an object, in bytes. Enforced by the Mgo adapter when effects are produced.
    max_move_object_size: Option<u64>,

    // TODO: Option<increase to 500 KB. currently, publishing a package > 500 KB exceeds the max computation gas cost
    /// Maximum size of a Move package object, in bytes. Enforced by the Mgo adapter at the end of a publish transaction.
    max_move_package_size: Option<u64>,

    /// Max number of publish or upgrade commands allowed in a programmable transaction block.
    max_publish_or_upgrade_per_ptb: Option<u64>,

    /// Maximum number of gas units that a single MoveCall transaction can use. Enforced by the Mgo adapter.
    max_tx_gas: Option<u64>,

    /// Maximum amount of the proposed gas price in MANGO (defined in the transaction).
    max_gas_price: Option<u64>,

    /// The max computation bucket for gas. This is the max that can be charged for computation.
    max_gas_computation_bucket: Option<u64>,

    // Define the value used to round up computation gas charges
    gas_rounding_step: Option<u64>,

    /// Maximum number of nested loops. Enforced by the Move bytecode verifier.
    max_loop_depth: Option<u64>,

    /// Maximum number of type arguments that can be bound to generic type parameters. Enforced by the Move bytecode verifier.
    max_generic_instantiation_length: Option<u64>,

    /// Maximum number of parameters that a Move function can have. Enforced by the Move bytecode verifier.
    max_function_parameters: Option<u64>,

    /// Maximum number of basic blocks that a Move function can have. Enforced by the Move bytecode verifier.
    max_basic_blocks: Option<u64>,

    /// Maximum stack size value. Enforced by the Move bytecode verifier.
    max_value_stack_size: Option<u64>,

    /// Maximum number of "type nodes", a metric for how big a SignatureToken will be when expanded into a fully qualified type. Enforced by the Move bytecode verifier.
    max_type_nodes: Option<u64>,

    /// Maximum number of push instructions in one function. Enforced by the Move bytecode verifier.
    max_push_size: Option<u64>,

    /// Maximum number of struct definitions in a module. Enforced by the Move bytecode verifier.
    max_struct_definitions: Option<u64>,

    /// Maximum number of function definitions in a module. Enforced by the Move bytecode verifier.
    max_function_definitions: Option<u64>,

    /// Maximum number of fields allowed in a struct definition. Enforced by the Move bytecode verifier.
    max_fields_in_struct: Option<u64>,

    /// Maximum dependency depth. Enforced by the Move linker when loading dependent modules.
    max_dependency_depth: Option<u64>,

    /// Maximum number of Move events that a single transaction can emit. Enforced by the VM during execution.
    max_num_event_emit: Option<u64>,

    /// Maximum number of new IDs that a single transaction can create. Enforced by the VM during execution.
    max_num_new_move_object_ids: Option<u64>,

    /// Maximum number of new IDs that a single system transaction can create. Enforced by the VM during execution.
    max_num_new_move_object_ids_system_tx: Option<u64>,

    /// Maximum number of IDs that a single transaction can delete. Enforced by the VM during execution.
    max_num_deleted_move_object_ids: Option<u64>,

    /// Maximum number of IDs that a single system transaction can delete. Enforced by the VM during execution.
    max_num_deleted_move_object_ids_system_tx: Option<u64>,

    /// Maximum number of IDs that a single transaction can transfer. Enforced by the VM during execution.
    max_num_transferred_move_object_ids: Option<u64>,

    /// Maximum number of IDs that a single system transaction can transfer. Enforced by the VM during execution.
    max_num_transferred_move_object_ids_system_tx: Option<u64>,

    /// Maximum size of a Move user event. Enforced by the VM during execution.
    max_event_emit_size: Option<u64>,

    /// Maximum size of a Move user event. Enforced by the VM during execution.
    max_event_emit_size_total: Option<u64>,

    /// Maximum length of a vector in Move. Enforced by the VM during execution, and for constants, by the verifier.
    max_move_vector_len: Option<u64>,

    /// Maximum length of an `Identifier` in Move. Enforced by the bytecode verifier at signing.
    max_move_identifier_len: Option<u64>,

    /// Maximum depth of a Move value within the VM.
    max_move_value_depth: Option<u64>,

    /// Maximum number of back edges in Move function. Enforced by the bytecode verifier at signing.
    max_back_edges_per_function: Option<u64>,

    /// Maximum number of back edges in Move module. Enforced by the bytecode verifier at signing.
    max_back_edges_per_module: Option<u64>,

    /// Maximum number of meter `ticks` spent verifying a Move function. Enforced by the bytecode verifier at signing.
    max_verifier_meter_ticks_per_function: Option<u64>,

    /// Maximum number of meter `ticks` spent verifying a Move function. Enforced by the bytecode verifier at signing.
    max_meter_ticks_per_module: Option<u64>,

    // === Object runtime internal operation limits ====
    // These affect dynamic fields
    /// Maximum number of cached objects in the object runtime ObjectStore. Enforced by object runtime during execution
    object_runtime_max_num_cached_objects: Option<u64>,

    /// Maximum number of cached objects in the object runtime ObjectStore in system transaction. Enforced by object runtime during execution
    object_runtime_max_num_cached_objects_system_tx: Option<u64>,

    /// Maximum number of stored objects accessed by object runtime ObjectStore. Enforced by object runtime during execution
    object_runtime_max_num_store_entries: Option<u64>,

    /// Maximum number of stored objects accessed by object runtime ObjectStore in system transaction. Enforced by object runtime during execution
    object_runtime_max_num_store_entries_system_tx: Option<u64>,

    // === Execution gas costs ====
    /// Base cost for any Mgo transaction
    base_tx_cost_fixed: Option<u64>,

    /// Additional cost for a transaction that publishes a package
    /// i.e., the base cost of such a transaction is base_tx_cost_fixed + package_publish_cost_fixed
    package_publish_cost_fixed: Option<u64>,

    /// Cost per byte of a Move call transaction
    /// i.e., the cost of such a transaction is base_cost + (base_tx_cost_per_byte * size)
    base_tx_cost_per_byte: Option<u64>,

    /// Cost per byte for a transaction that publishes a package
    package_publish_cost_per_byte: Option<u64>,

    // Per-byte cost of reading an object during transaction execution
    obj_access_cost_read_per_byte: Option<u64>,

    // Per-byte cost of writing an object during transaction execution
    obj_access_cost_mutate_per_byte: Option<u64>,

    // Per-byte cost of deleting an object during transaction execution
    obj_access_cost_delete_per_byte: Option<u64>,

    /// Per-byte cost charged for each input object to a transaction.
    /// Meant to approximate the cost of checking locks for each object
    // TODO: Option<I'm not sure that this cost makes sense. Checking locks is "free"
    // in the sense that an invalid tx that can never be committed/pay gas can
    // force validators to check an arbitrary number of locks. If those checks are
    // "free" for invalid transactions, why charge for them in valid transactions
    // TODO: Option<if we keep this, I think we probably want it to be a fixed cost rather
    // than a per-byte cost. checking an object lock should not require loading an
    // entire object, just consulting an ID -> tx digest map
    obj_access_cost_verify_per_byte: Option<u64>,

    /// === Gas version. gas model ===

    /// Gas model version, what code we are using to charge gas
    gas_model_version: Option<u64>,

    /// === Storage gas costs ===

    /// Per-byte cost of storing an object in the Mgo global object store. Some of this cost may be refundable if the object is later freed
    obj_data_cost_refundable: Option<u64>,

    // Per-byte cost of storing an object in the Mgo transaction log (e.g., in CertifiedTransactionEffects)
    // This depends on the size of various fields including the effects
    // TODO: Option<I don't fully understand this^ and more details would be useful
    obj_metadata_cost_non_refundable: Option<u64>,

    /// === Tokenomics ===

    // TODO: Option<this should be changed to u64.
    /// Sender of a txn that touches an object will get this percent of the storage rebate back.
    /// In basis point.
    storage_rebate_rate: Option<u64>,

    /// 5% of the storage fund's share of rewards are reinvested into the storage fund.
    /// In basis point.
    storage_fund_reinvest_rate: Option<u64>,

    /// The share of rewards that will be slashed and redistributed is 50%.
    /// In basis point.
    reward_slashing_rate: Option<u64>,

    /// Unit gas price, Mango per internal gas unit.
    storage_gas_price: Option<u64>,

    /// === Core Protocol ===

    /// Max number of transactions per checkpoint.
    /// Note that this is a protocol constant and not a config as validators must have this set to
    /// the same value, otherwise they *will* fork.
    max_transactions_per_checkpoint: Option<u64>,

    /// Max size of a checkpoint in bytes.
    /// Note that this is a protocol constant and not a config as validators must have this set to
    /// the same value, otherwise they *will* fork.
    max_checkpoint_size_bytes: Option<u64>,

    /// A protocol upgrade always requires 2f+1 stake to agree. We support a buffer of additional
    /// stake (as a fraction of f, expressed in basis points) that is required before an upgrade
    /// can happen automatically. 10000bps would indicate that complete unanimity is required (all
    /// 3f+1 must vote), while 0bps would indicate that 2f+1 is sufficient.
    buffer_stake_for_protocol_upgrade_bps: Option<u64>,

    // === Native Function Costs ===

    // `address` module
    // Cost params for the Move native function `address::from_bytes(bytes: vector<u8>)`
    address_from_bytes_cost_base: Option<u64>,
    // Cost params for the Move native function `address::to_u256(address): u256`
    address_to_u256_cost_base: Option<u64>,
    // Cost params for the Move native function `address::from_u256(u256): address`
    address_from_u256_cost_base: Option<u64>,

    // `dynamic_field` module
    // Cost params for the Move native function `hash_type_and_key<K: copy + drop + store>(parent: address, k: K): address`
    dynamic_field_hash_type_and_key_cost_base: Option<u64>,
    dynamic_field_hash_type_and_key_type_cost_per_byte: Option<u64>,
    dynamic_field_hash_type_and_key_value_cost_per_byte: Option<u64>,
    dynamic_field_hash_type_and_key_type_tag_cost_per_byte: Option<u64>,
    // Cost params for the Move native function `add_child_object<Child: key>(parent: address, child: Child)`
    dynamic_field_add_child_object_cost_base: Option<u64>,
    dynamic_field_add_child_object_type_cost_per_byte: Option<u64>,
    dynamic_field_add_child_object_value_cost_per_byte: Option<u64>,
    dynamic_field_add_child_object_struct_tag_cost_per_byte: Option<u64>,
    // Cost params for the Move native function `borrow_child_object_mut<Child: key>(parent: &mut UID, id: address): &mut Child`
    dynamic_field_borrow_child_object_cost_base: Option<u64>,
    dynamic_field_borrow_child_object_child_ref_cost_per_byte: Option<u64>,
    dynamic_field_borrow_child_object_type_cost_per_byte: Option<u64>,
    // Cost params for the Move native function `remove_child_object<Child: key>(parent: address, id: address): Child`
    dynamic_field_remove_child_object_cost_base: Option<u64>,
    dynamic_field_remove_child_object_child_cost_per_byte: Option<u64>,
    dynamic_field_remove_child_object_type_cost_per_byte: Option<u64>,
    // Cost params for the Move native function `has_child_object(parent: address, id: address): bool`
    dynamic_field_has_child_object_cost_base: Option<u64>,
    // Cost params for the Move native function `has_child_object_with_ty<Child: key>(parent: address, id: address): bool`
    dynamic_field_has_child_object_with_ty_cost_base: Option<u64>,
    dynamic_field_has_child_object_with_ty_type_cost_per_byte: Option<u64>,
    dynamic_field_has_child_object_with_ty_type_tag_cost_per_byte: Option<u64>,

    // `event` module
    // Cost params for the Move native function `event::emit<T: copy + drop>(event: T)`
    event_emit_cost_base: Option<u64>,
    event_emit_value_size_derivation_cost_per_byte: Option<u64>,
    event_emit_tag_size_derivation_cost_per_byte: Option<u64>,
    event_emit_output_cost_per_byte: Option<u64>,

    //  `object` module
    // Cost params for the Move native function `borrow_uid<T: key>(obj: &T): &UID`
    object_borrow_uid_cost_base: Option<u64>,
    // Cost params for the Move native function `delete_impl(id: address)`
    object_delete_impl_cost_base: Option<u64>,
    // Cost params for the Move native function `record_new_uid(id: address)`
    object_record_new_uid_cost_base: Option<u64>,

    // Transfer
    // Cost params for the Move native function `transfer_impl<T: key>(obj: T, recipient: address)`
    transfer_transfer_internal_cost_base: Option<u64>,
    // Cost params for the Move native function `freeze_object<T: key>(obj: T)`
    transfer_freeze_object_cost_base: Option<u64>,
    // Cost params for the Move native function `share_object<T: key>(obj: T)`
    transfer_share_object_cost_base: Option<u64>,
    // Cost params for the Move native function
    // `receive_object<T: key>(p: &mut UID, recv: Receiving<T>T)`
    transfer_receive_object_cost_base: Option<u64>,

    // TxContext
    // Cost params for the Move native function `transfer_impl<T: key>(obj: T, recipient: address)`
    tx_context_derive_id_cost_base: Option<u64>,

    // Types
    // Cost params for the Move native function `is_one_time_witness<T: drop>(_: &T): bool`
    types_is_one_time_witness_cost_base: Option<u64>,
    types_is_one_time_witness_type_tag_cost_per_byte: Option<u64>,
    types_is_one_time_witness_type_cost_per_byte: Option<u64>,

    // Validator
    // Cost params for the Move native function `validate_metadata_bcs(metadata: vector<u8>)`
    validator_validate_metadata_cost_base: Option<u64>,
    validator_validate_metadata_data_cost_per_byte: Option<u64>,

    // Crypto natives
    crypto_invalid_arguments_cost: Option<u64>,
    // bls12381::bls12381_min_sig_verify
    bls12381_bls12381_min_sig_verify_cost_base: Option<u64>,
    bls12381_bls12381_min_sig_verify_msg_cost_per_byte: Option<u64>,
    bls12381_bls12381_min_sig_verify_msg_cost_per_block: Option<u64>,

    // bls12381::bls12381_min_pk_verify
    bls12381_bls12381_min_pk_verify_cost_base: Option<u64>,
    bls12381_bls12381_min_pk_verify_msg_cost_per_byte: Option<u64>,
    bls12381_bls12381_min_pk_verify_msg_cost_per_block: Option<u64>,

    // ecdsa_k1::ecrecover
    ecdsa_k1_ecrecover_keccak256_cost_base: Option<u64>,
    ecdsa_k1_ecrecover_keccak256_msg_cost_per_byte: Option<u64>,
    ecdsa_k1_ecrecover_keccak256_msg_cost_per_block: Option<u64>,
    ecdsa_k1_ecrecover_sha256_cost_base: Option<u64>,
    ecdsa_k1_ecrecover_sha256_msg_cost_per_byte: Option<u64>,
    ecdsa_k1_ecrecover_sha256_msg_cost_per_block: Option<u64>,

    // ecdsa_k1::decompress_pubkey
    ecdsa_k1_decompress_pubkey_cost_base: Option<u64>,

    // ecdsa_k1::secp256k1_verify
    ecdsa_k1_secp256k1_verify_keccak256_cost_base: Option<u64>,
    ecdsa_k1_secp256k1_verify_keccak256_msg_cost_per_byte: Option<u64>,
    ecdsa_k1_secp256k1_verify_keccak256_msg_cost_per_block: Option<u64>,
    ecdsa_k1_secp256k1_verify_sha256_cost_base: Option<u64>,
    ecdsa_k1_secp256k1_verify_sha256_msg_cost_per_byte: Option<u64>,
    ecdsa_k1_secp256k1_verify_sha256_msg_cost_per_block: Option<u64>,

    // ecdsa_r1::ecrecover
    ecdsa_r1_ecrecover_keccak256_cost_base: Option<u64>,
    ecdsa_r1_ecrecover_keccak256_msg_cost_per_byte: Option<u64>,
    ecdsa_r1_ecrecover_keccak256_msg_cost_per_block: Option<u64>,
    ecdsa_r1_ecrecover_sha256_cost_base: Option<u64>,
    ecdsa_r1_ecrecover_sha256_msg_cost_per_byte: Option<u64>,
    ecdsa_r1_ecrecover_sha256_msg_cost_per_block: Option<u64>,

    // ecdsa_r1::secp256k1_verify
    ecdsa_r1_secp256r1_verify_keccak256_cost_base: Option<u64>,
    ecdsa_r1_secp256r1_verify_keccak256_msg_cost_per_byte: Option<u64>,
    ecdsa_r1_secp256r1_verify_keccak256_msg_cost_per_block: Option<u64>,
    ecdsa_r1_secp256r1_verify_sha256_cost_base: Option<u64>,
    ecdsa_r1_secp256r1_verify_sha256_msg_cost_per_byte: Option<u64>,
    ecdsa_r1_secp256r1_verify_sha256_msg_cost_per_block: Option<u64>,

    // ecvrf::verify
    ecvrf_ecvrf_verify_cost_base: Option<u64>,
    ecvrf_ecvrf_verify_alpha_string_cost_per_byte: Option<u64>,
    ecvrf_ecvrf_verify_alpha_string_cost_per_block: Option<u64>,

    // ed25519
    ed25519_ed25519_verify_cost_base: Option<u64>,
    ed25519_ed25519_verify_msg_cost_per_byte: Option<u64>,
    ed25519_ed25519_verify_msg_cost_per_block: Option<u64>,

    // groth16::prepare_verifying_key
    groth16_prepare_verifying_key_bls12381_cost_base: Option<u64>,
    groth16_prepare_verifying_key_bn254_cost_base: Option<u64>,

    // groth16::verify_groth16_proof_internal
    groth16_verify_groth16_proof_internal_bls12381_cost_base: Option<u64>,
    groth16_verify_groth16_proof_internal_bls12381_cost_per_public_input: Option<u64>,
    groth16_verify_groth16_proof_internal_bn254_cost_base: Option<u64>,
    groth16_verify_groth16_proof_internal_bn254_cost_per_public_input: Option<u64>,
    groth16_verify_groth16_proof_internal_public_input_cost_per_byte: Option<u64>,

    // hash::blake2b256
    hash_blake2b256_cost_base: Option<u64>,
    hash_blake2b256_data_cost_per_byte: Option<u64>,
    hash_blake2b256_data_cost_per_block: Option<u64>,

    // hash::keccak256
    hash_keccak256_cost_base: Option<u64>,
    hash_keccak256_data_cost_per_byte: Option<u64>,
    hash_keccak256_data_cost_per_block: Option<u64>,

    // poseidon::poseidon_bn254
    poseidon_bn254_cost_base: Option<u64>,
    poseidon_bn254_cost_per_block: Option<u64>,

    // group_ops
    group_ops_bls12381_decode_scalar_cost: Option<u64>,
    group_ops_bls12381_decode_g1_cost: Option<u64>,
    group_ops_bls12381_decode_g2_cost: Option<u64>,
    group_ops_bls12381_decode_gt_cost: Option<u64>,
    group_ops_bls12381_scalar_add_cost: Option<u64>,
    group_ops_bls12381_g1_add_cost: Option<u64>,
    group_ops_bls12381_g2_add_cost: Option<u64>,
    group_ops_bls12381_gt_add_cost: Option<u64>,
    group_ops_bls12381_scalar_sub_cost: Option<u64>,
    group_ops_bls12381_g1_sub_cost: Option<u64>,
    group_ops_bls12381_g2_sub_cost: Option<u64>,
    group_ops_bls12381_gt_sub_cost: Option<u64>,
    group_ops_bls12381_scalar_mul_cost: Option<u64>,
    group_ops_bls12381_g1_mul_cost: Option<u64>,
    group_ops_bls12381_g2_mul_cost: Option<u64>,
    group_ops_bls12381_gt_mul_cost: Option<u64>,
    group_ops_bls12381_scalar_div_cost: Option<u64>,
    group_ops_bls12381_g1_div_cost: Option<u64>,
    group_ops_bls12381_g2_div_cost: Option<u64>,
    group_ops_bls12381_gt_div_cost: Option<u64>,
    group_ops_bls12381_g1_hash_to_base_cost: Option<u64>,
    group_ops_bls12381_g2_hash_to_base_cost: Option<u64>,
    group_ops_bls12381_g1_hash_to_cost_per_byte: Option<u64>,
    group_ops_bls12381_g2_hash_to_cost_per_byte: Option<u64>,
    group_ops_bls12381_g1_msm_base_cost: Option<u64>,
    group_ops_bls12381_g2_msm_base_cost: Option<u64>,
    group_ops_bls12381_g1_msm_base_cost_per_input: Option<u64>,
    group_ops_bls12381_g2_msm_base_cost_per_input: Option<u64>,
    group_ops_bls12381_msm_max_len: Option<u32>,
    group_ops_bls12381_pairing_cost: Option<u64>,

    // hmac::hmac_sha3_256
    hmac_hmac_sha3_256_cost_base: Option<u64>,
    hmac_hmac_sha3_256_input_cost_per_byte: Option<u64>,
    hmac_hmac_sha3_256_input_cost_per_block: Option<u64>,

    // zklogin::check_zklogin_id
    check_zklogin_id_cost_base: Option<u64>,
    // zklogin::check_zklogin_issuer
    check_zklogin_issuer_cost_base: Option<u64>,

    // Const params for consensus scoring decision
    // The scaling factor property for the MED outlier detection
    scoring_decision_mad_divisor: Option<f64>,
    // The cutoff value for the MED outlier detection
    scoring_decision_cutoff_value: Option<f64>,

    /// === Execution Version ===
    execution_version: Option<u64>,

    // Dictates the threshold (percentage of stake) that is used to calculate the "bad" nodes to be
    // swapped when creating the consensus schedule. The values should be of the range [0 - 33]. Anything
    // above 33 (f) will not be allowed.
    consensus_bad_nodes_stake_threshold: Option<u64>,

    max_jwk_votes_per_validator_per_epoch: Option<u64>,
    // The maximum age of a JWK in epochs before it is removed from the AuthenticatorState object.
    // Applied at the end of an epoch as a delta from the new epoch value, so setting this to 1
    // will cause the new epoch to start with JWKs from the previous epoch still valid.
    max_age_of_jwk_in_epochs: Option<u64>,

    /// === random beacon ===

    /// Maximum allowed precision loss when reducing voting weights for the random beacon
    /// protocol.
    random_beacon_reduction_allowed_delta: Option<u16>,

    /// The maximum serialised transaction size (in bytes) accepted by consensus. That should be bigger than the
    /// `max_tx_size_bytes` with some additional headroom.
    consensus_max_transaction_size_bytes: Option<u64>,
    /// The maximum size of transactions included in a consensus proposed block
    consensus_max_transactions_in_block_bytes: Option<u64>,
}

// feature flags
impl ProtocolConfig {
    // Add checks for feature flag support here, e.g.:
    // pub fn check_new_protocol_feature_supported(&self) -> Result<(), Error> {
    //     if self.feature_flags.new_protocol_feature_supported {
    //         Ok(())
    //     } else {
    //         Err(Error(format!(
    //             "new_protocol_feature is not supported at {:?}",
    //             self.version
    //         )))
    //     }
    // }

    pub fn check_package_upgrades_supported(&self) -> Result<(), Error> {
        if self.feature_flags.package_upgrades {
            Ok(())
        } else {
            Err(Error(format!(
                "package upgrades are not supported at {:?}",
                self.version
            )))
        }
    }

    pub fn allow_receiving_object_id(&self) -> bool {
        self.feature_flags.allow_receiving_object_id
    }

    pub fn receiving_objects_supported(&self) -> bool {
        self.feature_flags.receive_objects
    }

    pub fn package_upgrades_supported(&self) -> bool {
        self.feature_flags.package_upgrades
    }

    pub fn check_commit_root_state_digest_supported(&self) -> bool {
        self.feature_flags.commit_root_state_digest
    }

    pub fn get_advance_epoch_start_time_in_safe_mode(&self) -> bool {
        self.feature_flags.advance_epoch_start_time_in_safe_mode
    }

    pub fn loaded_child_objects_fixed(&self) -> bool {
        self.feature_flags.loaded_child_objects_fixed
    }

    pub fn missing_type_is_compatibility_error(&self) -> bool {
        self.feature_flags.missing_type_is_compatibility_error
    }

    pub fn scoring_decision_with_validity_cutoff(&self) -> bool {
        self.feature_flags.scoring_decision_with_validity_cutoff
    }

    pub fn narwhal_versioned_metadata(&self) -> bool {
        self.feature_flags.narwhal_versioned_metadata
    }

    pub fn consensus_order_end_of_epoch_last(&self) -> bool {
        self.feature_flags.consensus_order_end_of_epoch_last
    }

    pub fn disallow_adding_abilities_on_upgrade(&self) -> bool {
        self.feature_flags.disallow_adding_abilities_on_upgrade
    }

    pub fn disable_invariant_violation_check_in_swap_loc(&self) -> bool {
        self.feature_flags
            .disable_invariant_violation_check_in_swap_loc
    }

    pub fn advance_to_highest_supported_protocol_version(&self) -> bool {
        self.feature_flags
            .advance_to_highest_supported_protocol_version
    }

    pub fn ban_entry_init(&self) -> bool {
        self.feature_flags.ban_entry_init
    }

    pub fn package_digest_hash_module(&self) -> bool {
        self.feature_flags.package_digest_hash_module
    }

    pub fn disallow_change_struct_type_params_on_upgrade(&self) -> bool {
        self.feature_flags
            .disallow_change_struct_type_params_on_upgrade
    }

    pub fn no_extraneous_module_bytes(&self) -> bool {
        self.feature_flags.no_extraneous_module_bytes
    }

    pub fn zklogin_auth(&self) -> bool {
        self.feature_flags.zklogin_auth
    }

    pub fn zklogin_supported_providers(&self) -> &BTreeSet<String> {
        &self.feature_flags.zklogin_supported_providers
    }

    pub fn consensus_transaction_ordering(&self) -> ConsensusTransactionOrdering {
        self.feature_flags.consensus_transaction_ordering
    }

    pub fn simplified_unwrap_then_delete(&self) -> bool {
        self.feature_flags.simplified_unwrap_then_delete
    }

    pub fn supports_upgraded_multisig(&self) -> bool {
        self.feature_flags.upgraded_multisig_supported
    }

    pub fn txn_base_cost_as_multiplier(&self) -> bool {
        self.feature_flags.txn_base_cost_as_multiplier
    }

    pub fn shared_object_deletion(&self) -> bool {
        self.feature_flags.shared_object_deletion
    }

    pub fn narwhal_new_leader_election_schedule(&self) -> bool {
        self.feature_flags.narwhal_new_leader_election_schedule
    }

    pub fn loaded_child_object_format(&self) -> bool {
        self.feature_flags.loaded_child_object_format
    }

    pub fn enable_jwk_consensus_updates(&self) -> bool {
        let ret = self.feature_flags.enable_jwk_consensus_updates;
        if ret {
            // jwk updates required end-of-epoch transactions
            assert!(self.feature_flags.end_of_epoch_transaction_supported);
        }
        ret
    }

    pub fn simple_conservation_checks(&self) -> bool {
        self.feature_flags.simple_conservation_checks
    }

    pub fn loaded_child_object_format_type(&self) -> bool {
        self.feature_flags.loaded_child_object_format_type
    }

    pub fn end_of_epoch_transaction_supported(&self) -> bool {
        let ret = self.feature_flags.end_of_epoch_transaction_supported;
        if !ret {
            // jwk updates required end-of-epoch transactions
            assert!(!self.feature_flags.enable_jwk_consensus_updates);
        }
        ret
    }

    pub fn recompute_has_public_transfer_in_execution(&self) -> bool {
        self.feature_flags
            .recompute_has_public_transfer_in_execution
    }

    // this function only exists for readability in the genesis code.
    pub fn create_authenticator_state_in_genesis(&self) -> bool {
        self.enable_jwk_consensus_updates()
    }

    pub fn narwhal_header_v2(&self) -> bool {
        self.feature_flags.narwhal_header_v2
    }

    pub fn random_beacon(&self) -> bool {
        let ret = self.feature_flags.random_beacon;
        if ret {
            // random beacon requires narwhal v2 headers
            assert!(self.feature_flags.narwhal_header_v2);
        }
        ret
    }

    pub fn enable_effects_v2(&self) -> bool {
        self.feature_flags.enable_effects_v2
    }

    pub fn narwhal_certificate_v2(&self) -> bool {
        self.feature_flags.narwhal_certificate_v2
    }

    pub fn verify_legacy_zklogin_address(&self) -> bool {
        self.feature_flags.verify_legacy_zklogin_address
    }

    pub fn accept_zklogin_in_multisig(&self) -> bool {
        self.feature_flags.accept_zklogin_in_multisig
    }

    pub fn throughput_aware_consensus_submission(&self) -> bool {
        self.feature_flags.throughput_aware_consensus_submission
    }

    pub fn include_consensus_digest_in_prologue(&self) -> bool {
        self.feature_flags.include_consensus_digest_in_prologue
    }

    pub fn hardened_otw_check(&self) -> bool {
        self.feature_flags.hardened_otw_check
    }

    pub fn enable_poseidon(&self) -> bool {
        self.feature_flags.enable_poseidon
    }

    pub fn enable_coin_deny_list(&self) -> bool {
        self.feature_flags.enable_coin_deny_list
    }

    pub fn enable_group_ops_native_functions(&self) -> bool {
        self.feature_flags.enable_group_ops_native_functions
    }
}

#[cfg(not(msim))]
static POISON_VERSION_METHODS: AtomicBool = AtomicBool::new(false);

// Use a thread local in sim tests for test isolation.
#[cfg(msim)]
thread_local! {
    static POISON_VERSION_METHODS: AtomicBool = AtomicBool::new(false);
}

// Instantiations for each protocol version.
impl ProtocolConfig {
    /// Get the value ProtocolConfig that are in effect during the given protocol version.
    pub fn get_for_version(version: ProtocolVersion, chain: Chain) -> Self {
        // ProtocolVersion can be deserialized so we need to check it here as well.
        assert!(version.0 >= ProtocolVersion::MIN.0, "{:?}", version);
        assert!(version.0 <= ProtocolVersion::MAX_ALLOWED.0, "{:?}", version);

        let mut ret = Self::get_for_version_impl(version, chain);
        ret.version = version;

        CONFIG_OVERRIDE.with(|ovr| {
            if let Some(override_fn) = &*ovr.borrow() {
                warn!(
                    "overriding ProtocolConfig settings with custom settings (you should not see this log outside of tests)"
                );
                override_fn(version, ret)
            } else {
                ret
            }
        })
    }

    /// Get the value ProtocolConfig that are in effect during the given protocol version.
    /// Or none if the version is not supported.
    pub fn get_for_version_if_supported(version: ProtocolVersion, chain: Chain) -> Option<Self> {
        if version.0 >= ProtocolVersion::MIN.0 && version.0 <= ProtocolVersion::MAX_ALLOWED.0 {
            let mut ret = Self::get_for_version_impl(version, chain);
            ret.version = version;
            Some(ret)
        } else {
            None
        }
    }

    #[cfg(not(msim))]
    pub fn poison_get_for_min_version() {
        POISON_VERSION_METHODS.store(true, Ordering::Relaxed);
    }

    #[cfg(not(msim))]
    fn load_poison_get_for_min_version() -> bool {
        POISON_VERSION_METHODS.load(Ordering::Relaxed)
    }

    #[cfg(msim)]
    pub fn poison_get_for_min_version() {
        POISON_VERSION_METHODS.with(|p| p.store(true, Ordering::Relaxed));
    }

    #[cfg(msim)]
    fn load_poison_get_for_min_version() -> bool {
        POISON_VERSION_METHODS.with(|p| p.load(Ordering::Relaxed))
    }

    /// Convenience to get the constants at the current minimum supported version.
    /// Mainly used by client code that may not yet be protocol-version aware.
    pub fn get_for_min_version() -> Self {
        if Self::load_poison_get_for_min_version() {
            panic!("get_for_min_version called on validator");
        }
        ProtocolConfig::get_for_version(ProtocolVersion::MIN, Chain::Unknown)
    }

    /// CAREFUL! - You probably want to use `get_for_version` instead.
    ///
    /// Convenience to get the constants at the current maximum supported version.
    /// Mainly used by genesis. Note well that this function uses the max version
    /// supported locally by the node, which is not necessarily the current version
    /// of the network. ALSO, this function disregards chain specific config (by
    /// using Chain::Unknown), thereby potentially returning a protocol config that
    /// is incorrect for some feature flags. Definitely safe for testing and for
    /// protocol version 11 and prior.
    #[allow(non_snake_case)]
    pub fn get_for_max_version_UNSAFE() -> Self {
        if Self::load_poison_get_for_min_version() {
            panic!("get_for_max_version_UNSAFE called on validator");
        }
        ProtocolConfig::get_for_version(ProtocolVersion::MAX, Chain::Unknown)
    }

    fn get_for_version_impl(version: ProtocolVersion, chain: Chain) -> Self {
        #[cfg(msim)]
        {
            // populate the fake simulator version # with a different base tx cost.
            if version == ProtocolVersion::MAX_ALLOWED {
                let mut config = Self::get_for_version_impl(version - 1, Chain::Unknown);
                config.base_tx_cost_fixed = Some(config.base_tx_cost_fixed() + 1000);
                return config;
            }
        }

        // IMPORTANT: Never modify the value of any constant for a pre-existing protocol version.
        // To change the values here you must create a new protocol version with the new values!
        let mut cfg = Self {
            // will be overwritten before being returned
            version,

            // All flags are disabled in V1
            feature_flags: Default::default(),

            max_tx_size_bytes: Some(128 * 1024),
            // We need this number to be at least 100x less than `max_serialized_tx_effects_size_bytes`otherwise effects can be huge
            max_input_objects: Some(2048),
            max_serialized_tx_effects_size_bytes: Some(512 * 1024),
            max_serialized_tx_effects_size_bytes_system_tx: Some(512 * 1024 * 16),
            max_gas_payment_objects: Some(256),
            max_modules_in_publish: Some(128),
            max_arguments: Some(512),
            max_type_arguments: Some(16),
            max_type_argument_depth: Some(16),
            max_pure_argument_size: Some(16 * 1024),
            max_programmable_tx_commands: Some(1024),
            move_binary_format_version: Some(6),
            max_move_object_size: Some(250 * 1024),
            max_move_package_size: Some(100 * 1024),
            max_publish_or_upgrade_per_ptb: None,
            max_tx_gas: Some(10_000_000_000),
            max_gas_price: Some(100_000),
            max_gas_computation_bucket: Some(5_000_000),
            max_loop_depth: Some(5),
            max_generic_instantiation_length: Some(32),
            max_function_parameters: Some(128),
            max_basic_blocks: Some(1024),
            max_value_stack_size: Some(1024),
            max_type_nodes: Some(256),
            max_push_size: Some(10000),
            max_struct_definitions: Some(200),
            max_function_definitions: Some(1000),
            max_fields_in_struct: Some(32),
            max_dependency_depth: Some(100),
            max_num_event_emit: Some(256),
            max_num_new_move_object_ids: Some(2048),
            max_num_new_move_object_ids_system_tx: Some(2048 * 16),
            max_num_deleted_move_object_ids: Some(2048),
            max_num_deleted_move_object_ids_system_tx: Some(2048 * 16),
            max_num_transferred_move_object_ids: Some(2048),
            max_num_transferred_move_object_ids_system_tx: Some(2048 * 16),
            max_event_emit_size: Some(250 * 1024),
            max_move_vector_len: Some(256 * 1024),

            // TODO: Is this too low/high?
            max_back_edges_per_function: Some(10_000),

            // TODO:  Is this too low/high?
            max_back_edges_per_module: Some(10_000),

            // TODO: Is this too low/high?
            max_verifier_meter_ticks_per_function: Some(6_000_000),

            // TODO: Is this too low/high?
            max_meter_ticks_per_module: Some(6_000_000),

            object_runtime_max_num_cached_objects: Some(1000),
            object_runtime_max_num_cached_objects_system_tx: Some(1000 * 16),
            object_runtime_max_num_store_entries: Some(1000),
            object_runtime_max_num_store_entries_system_tx: Some(1000 * 16),
            base_tx_cost_fixed: Some(110_000),
            package_publish_cost_fixed: Some(1_000),
            base_tx_cost_per_byte: Some(0),
            package_publish_cost_per_byte: Some(80),
            obj_access_cost_read_per_byte: Some(15),
            obj_access_cost_mutate_per_byte: Some(40),
            obj_access_cost_delete_per_byte: Some(40),
            obj_access_cost_verify_per_byte: Some(200),
            obj_data_cost_refundable: Some(100),
            obj_metadata_cost_non_refundable: Some(50),
            gas_model_version: Some(1),
            storage_rebate_rate: Some(9900),
            storage_fund_reinvest_rate: Some(500),
            reward_slashing_rate: Some(5000),
            storage_gas_price: Some(1),
            max_transactions_per_checkpoint: Some(10_000),
            max_checkpoint_size_bytes: Some(30 * 1024 * 1024),

            // For now, perform upgrades with a bare quorum of validators.
            // MUSTFIX: This number should be increased to at least 2000 (20%) for mainnet.
            buffer_stake_for_protocol_upgrade_bps: Some(0),

            // === Native Function Costs ===
            // `address` module
            // Cost params for the Move native function `address::from_bytes(bytes: vector<u8>)`
            address_from_bytes_cost_base: Some(52),
            // Cost params for the Move native function `address::to_u256(address): u256`
            address_to_u256_cost_base: Some(52),
            // Cost params for the Move native function `address::from_u256(u256): address`
            address_from_u256_cost_base: Some(52),

            // `dynamic_field` module
            // Cost params for the Move native function `hash_type_and_key<K: copy + drop + store>(parent: address, k: K): address`
            dynamic_field_hash_type_and_key_cost_base: Some(100),
            dynamic_field_hash_type_and_key_type_cost_per_byte: Some(2),
            dynamic_field_hash_type_and_key_value_cost_per_byte: Some(2),
            dynamic_field_hash_type_and_key_type_tag_cost_per_byte: Some(2),
            // Cost params for the Move native function `add_child_object<Child: key>(parent: address, child: Child)`
            dynamic_field_add_child_object_cost_base: Some(100),
            dynamic_field_add_child_object_type_cost_per_byte: Some(10),
            dynamic_field_add_child_object_value_cost_per_byte: Some(10),
            dynamic_field_add_child_object_struct_tag_cost_per_byte: Some(10),
            // Cost params for the Move native function `borrow_child_object_mut<Child: key>(parent: &mut UID, id: address): &mut Child`
            dynamic_field_borrow_child_object_cost_base: Some(100),
            dynamic_field_borrow_child_object_child_ref_cost_per_byte: Some(10),
            dynamic_field_borrow_child_object_type_cost_per_byte: Some(10),
            // Cost params for the Move native function `remove_child_object<Child: key>(parent: address, id: address): Child`
            dynamic_field_remove_child_object_cost_base: Some(100),
            dynamic_field_remove_child_object_child_cost_per_byte: Some(2),
            dynamic_field_remove_child_object_type_cost_per_byte: Some(2),
            // Cost params for the Move native function `has_child_object(parent: address, id: address): bool`
            dynamic_field_has_child_object_cost_base: Some(100),
            // Cost params for the Move native function `has_child_object_with_ty<Child: key>(parent: address, id: address): bool`
            dynamic_field_has_child_object_with_ty_cost_base: Some(100),
            dynamic_field_has_child_object_with_ty_type_cost_per_byte: Some(2),
            dynamic_field_has_child_object_with_ty_type_tag_cost_per_byte: Some(2),

            // `event` module
            // Cost params for the Move native function `event::emit<T: copy + drop>(event: T)`
            event_emit_cost_base: Some(52),
            event_emit_value_size_derivation_cost_per_byte: Some(2),
            event_emit_tag_size_derivation_cost_per_byte: Some(5),
            event_emit_output_cost_per_byte: Some(10),

            //  `object` module
            // Cost params for the Move native function `borrow_uid<T: key>(obj: &T): &UID`
            object_borrow_uid_cost_base: Some(52),
            // Cost params for the Move native function `delete_impl(id: address)`
            object_delete_impl_cost_base: Some(52),
            // Cost params for the Move native function `record_new_uid(id: address)`
            object_record_new_uid_cost_base: Some(52),

            // `transfer` module
            // Cost params for the Move native function `transfer_impl<T: key>(obj: T, recipient: address)`
            transfer_transfer_internal_cost_base: Some(52),
            // Cost params for the Move native function `freeze_object<T: key>(obj: T)`
            transfer_freeze_object_cost_base: Some(52),
            // Cost params for the Move native function `share_object<T: key>(obj: T)`
            transfer_share_object_cost_base: Some(52),
            transfer_receive_object_cost_base: None,

            // `tx_context` module
            // Cost params for the Move native function `transfer_impl<T: key>(obj: T, recipient: address)`
            tx_context_derive_id_cost_base: Some(52),

            // `types` module
            // Cost params for the Move native function `is_one_time_witness<T: drop>(_: &T): bool`
            types_is_one_time_witness_cost_base: Some(52),
            types_is_one_time_witness_type_tag_cost_per_byte: Some(2),
            types_is_one_time_witness_type_cost_per_byte: Some(2),

            // `validator` module
            // Cost params for the Move native function `validate_metadata_bcs(metadata: vector<u8>)`
            validator_validate_metadata_cost_base: Some(52),
            validator_validate_metadata_data_cost_per_byte: Some(2),

            // Crypto
            crypto_invalid_arguments_cost: Some(100),
            // bls12381::bls12381_min_pk_verify
            bls12381_bls12381_min_sig_verify_cost_base: Some(52),
            bls12381_bls12381_min_sig_verify_msg_cost_per_byte: Some(2),
            bls12381_bls12381_min_sig_verify_msg_cost_per_block: Some(2),

            // bls12381::bls12381_min_pk_verify
            bls12381_bls12381_min_pk_verify_cost_base: Some(52),
            bls12381_bls12381_min_pk_verify_msg_cost_per_byte: Some(2),
            bls12381_bls12381_min_pk_verify_msg_cost_per_block: Some(2),

            // ecdsa_k1::ecrecover
            ecdsa_k1_ecrecover_keccak256_cost_base: Some(52),
            ecdsa_k1_ecrecover_keccak256_msg_cost_per_byte: Some(2),
            ecdsa_k1_ecrecover_keccak256_msg_cost_per_block: Some(2),
            ecdsa_k1_ecrecover_sha256_cost_base: Some(52),
            ecdsa_k1_ecrecover_sha256_msg_cost_per_byte: Some(2),
            ecdsa_k1_ecrecover_sha256_msg_cost_per_block: Some(2),

            // ecdsa_k1::decompress_pubkey
            ecdsa_k1_decompress_pubkey_cost_base: Some(52),

            // ecdsa_k1::secp256k1_verify
            ecdsa_k1_secp256k1_verify_keccak256_cost_base: Some(52),
            ecdsa_k1_secp256k1_verify_keccak256_msg_cost_per_byte: Some(2),
            ecdsa_k1_secp256k1_verify_keccak256_msg_cost_per_block: Some(2),
            ecdsa_k1_secp256k1_verify_sha256_cost_base: Some(52),
            ecdsa_k1_secp256k1_verify_sha256_msg_cost_per_byte: Some(2),
            ecdsa_k1_secp256k1_verify_sha256_msg_cost_per_block: Some(2),

            // ecdsa_r1::ecrecover
            ecdsa_r1_ecrecover_keccak256_cost_base: Some(52),
            ecdsa_r1_ecrecover_keccak256_msg_cost_per_byte: Some(2),
            ecdsa_r1_ecrecover_keccak256_msg_cost_per_block: Some(2),
            ecdsa_r1_ecrecover_sha256_cost_base: Some(52),
            ecdsa_r1_ecrecover_sha256_msg_cost_per_byte: Some(2),
            ecdsa_r1_ecrecover_sha256_msg_cost_per_block: Some(2),

            // ecdsa_r1::secp256k1_verify
            ecdsa_r1_secp256r1_verify_keccak256_cost_base: Some(52),
            ecdsa_r1_secp256r1_verify_keccak256_msg_cost_per_byte: Some(2),
            ecdsa_r1_secp256r1_verify_keccak256_msg_cost_per_block: Some(2),
            ecdsa_r1_secp256r1_verify_sha256_cost_base: Some(52),
            ecdsa_r1_secp256r1_verify_sha256_msg_cost_per_byte: Some(2),
            ecdsa_r1_secp256r1_verify_sha256_msg_cost_per_block: Some(2),

            // ecvrf::verify
            ecvrf_ecvrf_verify_cost_base: Some(52),
            ecvrf_ecvrf_verify_alpha_string_cost_per_byte: Some(2),
            ecvrf_ecvrf_verify_alpha_string_cost_per_block: Some(2),

            // ed25519
            ed25519_ed25519_verify_cost_base: Some(52),
            ed25519_ed25519_verify_msg_cost_per_byte: Some(2),
            ed25519_ed25519_verify_msg_cost_per_block: Some(2),

            // groth16::prepare_verifying_key
            groth16_prepare_verifying_key_bls12381_cost_base: Some(52),
            groth16_prepare_verifying_key_bn254_cost_base: Some(52),

            // groth16::verify_groth16_proof_internal
            groth16_verify_groth16_proof_internal_bls12381_cost_base: Some(52),
            groth16_verify_groth16_proof_internal_bls12381_cost_per_public_input: Some(2),
            groth16_verify_groth16_proof_internal_bn254_cost_base: Some(52),
            groth16_verify_groth16_proof_internal_bn254_cost_per_public_input: Some(2),
            groth16_verify_groth16_proof_internal_public_input_cost_per_byte: Some(2),

            // hash::blake2b256
            hash_blake2b256_cost_base: Some(52),
            hash_blake2b256_data_cost_per_byte: Some(2),
            hash_blake2b256_data_cost_per_block: Some(2),
            // hash::keccak256
            hash_keccak256_cost_base: Some(52),
            hash_keccak256_data_cost_per_byte: Some(2),
            hash_keccak256_data_cost_per_block: Some(2),

            poseidon_bn254_cost_base: None,
            poseidon_bn254_cost_per_block: None,

            // hmac::hmac_sha3_256
            hmac_hmac_sha3_256_cost_base: Some(52),
            hmac_hmac_sha3_256_input_cost_per_byte: Some(2),
            hmac_hmac_sha3_256_input_cost_per_block: Some(2),

            // group ops
            group_ops_bls12381_decode_scalar_cost: None,
            group_ops_bls12381_decode_g1_cost: None,
            group_ops_bls12381_decode_g2_cost: None,
            group_ops_bls12381_decode_gt_cost: None,
            group_ops_bls12381_scalar_add_cost: None,
            group_ops_bls12381_g1_add_cost: None,
            group_ops_bls12381_g2_add_cost: None,
            group_ops_bls12381_gt_add_cost: None,
            group_ops_bls12381_scalar_sub_cost: None,
            group_ops_bls12381_g1_sub_cost: None,
            group_ops_bls12381_g2_sub_cost: None,
            group_ops_bls12381_gt_sub_cost: None,
            group_ops_bls12381_scalar_mul_cost: None,
            group_ops_bls12381_g1_mul_cost: None,
            group_ops_bls12381_g2_mul_cost: None,
            group_ops_bls12381_gt_mul_cost: None,
            group_ops_bls12381_scalar_div_cost: None,
            group_ops_bls12381_g1_div_cost: None,
            group_ops_bls12381_g2_div_cost: None,
            group_ops_bls12381_gt_div_cost: None,
            group_ops_bls12381_g1_hash_to_base_cost: None,
            group_ops_bls12381_g2_hash_to_base_cost: None,
            group_ops_bls12381_g1_hash_to_cost_per_byte: None,
            group_ops_bls12381_g2_hash_to_cost_per_byte: None,
            group_ops_bls12381_g1_msm_base_cost: None,
            group_ops_bls12381_g2_msm_base_cost: None,
            group_ops_bls12381_g1_msm_base_cost_per_input: None,
            group_ops_bls12381_g2_msm_base_cost_per_input: None,
            group_ops_bls12381_msm_max_len: None,
            group_ops_bls12381_pairing_cost: None,

            // zklogin::check_zklogin_id
            check_zklogin_id_cost_base: None,
            // zklogin::check_zklogin_issuer
            check_zklogin_issuer_cost_base: None,

            max_size_written_objects: None,
            max_size_written_objects_system_tx: None,

            // Const params for consensus scoring decision
            scoring_decision_mad_divisor: None,
            scoring_decision_cutoff_value: None,

            // Limits the length of a Move identifier
            max_move_identifier_len: None,
            max_move_value_depth: None,

            gas_rounding_step: None,

            execution_version: None,

            max_event_emit_size_total: None,

            consensus_bad_nodes_stake_threshold: None,

            max_jwk_votes_per_validator_per_epoch: None,

            max_age_of_jwk_in_epochs: None,

            random_beacon_reduction_allowed_delta: None,

            consensus_max_transaction_size_bytes: None,

            consensus_max_transactions_in_block_bytes: None,
            // When adding a new constant, set it to None in the earliest version, like this:
            // new_constant: None,
        };
        for cur in 2..=version.0 {
            match cur {
                1 => unreachable!(),
                2 => {
                    cfg.feature_flags.advance_epoch_start_time_in_safe_mode = true;

                    cfg.max_tx_gas = Some(50_000_000_000);
                    cfg.storage_gas_price = Some(76);
                    cfg.feature_flags.loaded_child_objects_fixed = true;
                    cfg.max_size_written_objects = Some(5 * 1000 * 1000);
                    cfg.max_size_written_objects_system_tx = Some(50 * 1000 * 1000);
                    cfg.feature_flags.package_upgrades = true;

                    cfg.reward_slashing_rate = Some(10000);

                    cfg.feature_flags.missing_type_is_compatibility_error = true;
                    cfg.feature_flags.scoring_decision_with_validity_cutoff = true;
                    cfg.scoring_decision_mad_divisor = Some(2.3);
                    cfg.scoring_decision_cutoff_value = Some(2.5);

                    cfg.buffer_stake_for_protocol_upgrade_bps = Some(5000);
                    cfg.feature_flags.consensus_order_end_of_epoch_last = true;

                    cfg.feature_flags.disallow_adding_abilities_on_upgrade = true;
                    cfg.feature_flags
                        .disable_invariant_violation_check_in_swap_loc = true;
                    cfg.feature_flags.ban_entry_init = true;
                    cfg.feature_flags.package_digest_hash_module = true;

                    cfg.feature_flags
                        .disallow_change_struct_type_params_on_upgrade = true;

                    cfg.max_move_identifier_len = Some(128);
                    cfg.feature_flags.no_extraneous_module_bytes = true;
                    cfg.feature_flags
                        .advance_to_highest_supported_protocol_version = true;

                    cfg.max_verifier_meter_ticks_per_function = Some(16_000_000);
                    cfg.max_meter_ticks_per_module = Some(16_000_000);

                    cfg.max_move_value_depth = Some(128);

                    cfg.feature_flags.narwhal_versioned_metadata = true;

                    cfg.gas_rounding_step = Some(1_000);

                    cfg.feature_flags.consensus_transaction_ordering =
                        ConsensusTransactionOrdering::ByGasPrice;

                    cfg.feature_flags.simplified_unwrap_then_delete = true;

                    cfg.feature_flags.upgraded_multisig_supported = true;

                    cfg.feature_flags.txn_base_cost_as_multiplier = true;
                    cfg.base_tx_cost_fixed = Some(1_000);

                    cfg.max_num_event_emit = Some(1024);
                    cfg.max_event_emit_size_total = Some(
                        256 /* former event count limit */ * 250 * 1024, /* size limit per event */
                    );

                    cfg.feature_flags.commit_root_state_digest = true;

                    cfg.feature_flags.loaded_child_object_format = true;

                    cfg.feature_flags.loaded_child_object_format_type = true;
                    cfg.feature_flags.narwhal_new_leader_election_schedule = true;
                    cfg.consensus_bad_nodes_stake_threshold = Some(20);

                    cfg.feature_flags.simple_conservation_checks = true;
                    cfg.max_publish_or_upgrade_per_ptb = Some(5);

                    cfg.feature_flags.end_of_epoch_transaction_supported = true;

                    cfg.feature_flags.zklogin_auth = true;
                    cfg.feature_flags.enable_jwk_consensus_updates = true;
                    cfg.max_jwk_votes_per_validator_per_epoch = Some(240);
                    cfg.max_age_of_jwk_in_epochs = Some(1);

                    cfg.gas_model_version = Some(8);

                    cfg.check_zklogin_id_cost_base = Some(200);
                    cfg.check_zklogin_issuer_cost_base = Some(200);

                    cfg.feature_flags.verify_legacy_zklogin_address = true;

                    cfg.random_beacon_reduction_allowed_delta = Some(800);
                    cfg.feature_flags.zklogin_supported_providers = BTreeSet::default();
                    cfg.feature_flags.recompute_has_public_transfer_in_execution = true;

                    cfg.execution_version = Some(2);

                    if chain != Chain::Mainnet && chain != Chain::Testnet {
                        cfg.feature_flags.accept_zklogin_in_multisig = true;
                    }

                    cfg.feature_flags.narwhal_header_v2 = true;
                    cfg.feature_flags.random_beacon = true;

                    cfg.feature_flags.include_consensus_digest_in_prologue = true;
                    cfg.feature_flags.narwhal_certificate_v2 = true;

                    cfg.feature_flags.hardened_otw_check = true;
                    cfg.feature_flags.allow_receiving_object_id = true;
                    cfg.transfer_receive_object_cost_base = Some(52);
                    cfg.feature_flags.receive_objects = true;
                    cfg.feature_flags.enable_effects_v2 = true;

                    if chain != Chain::Mainnet && chain != Chain::Testnet {
                        cfg.feature_flags.enable_poseidon = true;
                        cfg.poseidon_bn254_cost_base = Some(260);
                        cfg.poseidon_bn254_cost_per_block = Some(10);
                    }
                    cfg.feature_flags.enable_coin_deny_list = true;

                    if chain != Chain::Mainnet && chain != Chain::Testnet {
                        cfg.feature_flags.enable_group_ops_native_functions = true;
                        cfg.group_ops_bls12381_decode_scalar_cost = Some(52);
                        cfg.group_ops_bls12381_decode_g1_cost = Some(52);
                        cfg.group_ops_bls12381_decode_g2_cost = Some(52);
                        cfg.group_ops_bls12381_decode_gt_cost = Some(52);
                        cfg.group_ops_bls12381_scalar_add_cost = Some(52);
                        cfg.group_ops_bls12381_g1_add_cost = Some(52);
                        cfg.group_ops_bls12381_g2_add_cost = Some(52);
                        cfg.group_ops_bls12381_gt_add_cost = Some(52);
                        cfg.group_ops_bls12381_scalar_sub_cost = Some(52);
                        cfg.group_ops_bls12381_g1_sub_cost = Some(52);
                        cfg.group_ops_bls12381_g2_sub_cost = Some(52);
                        cfg.group_ops_bls12381_gt_sub_cost = Some(52);
                        cfg.group_ops_bls12381_scalar_mul_cost = Some(52);
                        cfg.group_ops_bls12381_g1_mul_cost = Some(52);
                        cfg.group_ops_bls12381_g2_mul_cost = Some(52);
                        cfg.group_ops_bls12381_gt_mul_cost = Some(52);
                        cfg.group_ops_bls12381_scalar_div_cost = Some(52);
                        cfg.group_ops_bls12381_g1_div_cost = Some(52);
                        cfg.group_ops_bls12381_g2_div_cost = Some(52);
                        cfg.group_ops_bls12381_gt_div_cost = Some(52);
                        cfg.group_ops_bls12381_g1_hash_to_base_cost = Some(52);
                        cfg.group_ops_bls12381_g2_hash_to_base_cost = Some(52);
                        cfg.group_ops_bls12381_g1_hash_to_cost_per_byte = Some(2);
                        cfg.group_ops_bls12381_g2_hash_to_cost_per_byte = Some(2);
                        cfg.group_ops_bls12381_g1_msm_base_cost = Some(52);
                        cfg.group_ops_bls12381_g2_msm_base_cost = Some(52);
                        cfg.group_ops_bls12381_g1_msm_base_cost_per_input = Some(52);
                        cfg.group_ops_bls12381_g2_msm_base_cost_per_input = Some(52);
                        cfg.group_ops_bls12381_msm_max_len = Some(32);
                        cfg.group_ops_bls12381_pairing_cost = Some(52);
                    }
                    cfg.feature_flags.shared_object_deletion = true;

                    cfg.consensus_max_transaction_size_bytes = Some(256 * 1024); // 256KB
                    cfg.consensus_max_transactions_in_block_bytes = Some(6 * 1_024 * 1024);
                }
                3 => {}
                _ => panic!("unsupported version {:?}", version),
            }
        }
        cfg
    }

    /// Override one or more settings in the config, for testing.
    /// This must be called at the beginning of the test, before get_for_(min|max)_version is
    /// called, since those functions cache their return value.
    pub fn apply_overrides_for_testing(
        override_fn: impl Fn(ProtocolVersion, Self) -> Self + Send + 'static,
    ) -> OverrideGuard {
        CONFIG_OVERRIDE.with(|ovr| {
            let mut cur = ovr.borrow_mut();
            assert!(cur.is_none(), "config override already present");
            *cur = Some(Box::new(override_fn));
            OverrideGuard
        })
    }
}

// Setters for tests
impl ProtocolConfig {
    pub fn set_package_upgrades_for_testing(&mut self, val: bool) {
        self.feature_flags.package_upgrades = val
    }
    pub fn set_advance_to_highest_supported_protocol_version_for_testing(&mut self, val: bool) {
        self.feature_flags
            .advance_to_highest_supported_protocol_version = val
    }
    pub fn set_commit_root_state_digest_supported(&mut self, val: bool) {
        self.feature_flags.commit_root_state_digest = val
    }
    pub fn set_zklogin_auth_for_testing(&mut self, val: bool) {
        self.feature_flags.zklogin_auth = val
    }
    pub fn set_enable_jwk_consensus_updates_for_testing(&mut self, val: bool) {
        self.feature_flags.enable_jwk_consensus_updates = val
    }
    pub fn set_random_beacon_for_testing(&mut self, val: bool) {
        self.feature_flags.random_beacon = val
    }
    pub fn set_upgraded_multisig_for_testing(&mut self, val: bool) {
        self.feature_flags.upgraded_multisig_supported = val
    }
    #[cfg(msim)]
    pub fn set_simplified_unwrap_then_delete(&mut self, val: bool) {
        self.feature_flags.simplified_unwrap_then_delete = val;
        if val == false {
            // Given that we will never enable effect V2 before turning on simplified_unwrap_then_delete, we also need to disable effect V2 here.
            self.set_enable_effects_v2(false);
        }
    }
    pub fn set_shared_object_deletion(&mut self, val: bool) {
        self.feature_flags.shared_object_deletion = val;
    }

    pub fn set_narwhal_new_leader_election_schedule(&mut self, val: bool) {
        self.feature_flags.narwhal_new_leader_election_schedule = val;
    }

    pub fn set_consensus_bad_nodes_stake_threshold(&mut self, val: u64) {
        self.consensus_bad_nodes_stake_threshold = Some(val);
    }
    pub fn set_receive_object_for_testing(&mut self, val: bool) {
        self.feature_flags.receive_objects = val
    }
    pub fn set_narwhal_certificate_v2(&mut self, val: bool) {
        self.feature_flags.narwhal_certificate_v2 = val
    }
    pub fn set_verify_legacy_zklogin_address(&mut self, val: bool) {
        self.feature_flags.verify_legacy_zklogin_address = val
    }
    pub fn set_enable_effects_v2(&mut self, val: bool) {
        self.feature_flags.enable_effects_v2 = val;
    }
    pub fn set_consensus_max_transaction_size_bytes(&mut self, val: u64) {
        self.consensus_max_transaction_size_bytes = Some(val);
    }
    pub fn set_consensus_max_transactions_in_block_bytes(&mut self, val: u64) {
        self.consensus_max_transactions_in_block_bytes = Some(val);
    }
}

type OverrideFn = dyn Fn(ProtocolVersion, ProtocolConfig) -> ProtocolConfig + Send;

thread_local! {
    static CONFIG_OVERRIDE: RefCell<Option<Box<OverrideFn>>> = RefCell::new(None);
}

#[must_use]
pub struct OverrideGuard;

impl Drop for OverrideGuard {
    fn drop(&mut self) {
        info!("restoring override fn");
        CONFIG_OVERRIDE.with(|ovr| {
            *ovr.borrow_mut() = None;
        });
    }
}

/// Defines which limit got crossed.
/// The value which crossed the limit and value of the limit crossed are embedded
#[derive(PartialEq, Eq)]
pub enum LimitThresholdCrossed {
    None,
    Soft(u128, u128),
    Hard(u128, u128),
}

/// Convenience function for comparing limit ranges
/// V::MAX must be at >= U::MAX and T::MAX
pub fn check_limit_in_range<T: Into<V>, U: Into<V>, V: PartialOrd + Into<u128>>(
    x: T,
    soft_limit: U,
    hard_limit: V,
) -> LimitThresholdCrossed {
    let x: V = x.into();
    let soft_limit: V = soft_limit.into();

    debug_assert!(soft_limit <= hard_limit);

    // It is important to preserve this comparison order because if soft_limit == hard_limit
    // we want LimitThresholdCrossed::Hard
    if x >= hard_limit {
        LimitThresholdCrossed::Hard(x.into(), hard_limit.into())
    } else if x < soft_limit {
        LimitThresholdCrossed::None
    } else {
        LimitThresholdCrossed::Soft(x.into(), soft_limit.into())
    }
}

#[macro_export]
macro_rules! check_limit {
    ($x:expr, $hard:expr) => {
        check_limit!($x, $hard, $hard)
    };
    ($x:expr, $soft:expr, $hard:expr) => {
        check_limit_in_range($x as u64, $soft, $hard)
    };
}

/// Used to check which limits were crossed if the TX is metered (not system tx)
/// Args are: is_metered, value_to_check, metered_limit, unmetered_limit
/// metered_limit is always less than or equal to unmetered_hard_limit
#[macro_export]
macro_rules! check_limit_by_meter {
    ($is_metered:expr, $x:expr, $metered_limit:expr, $unmetered_hard_limit:expr, $metric:expr) => {{
        // If this is metered, we use the metered_limit limit as the upper bound
        let (h, metered_str) = if $is_metered {
            ($metered_limit, "metered")
        } else {
            // Unmetered gets more headroom
            ($unmetered_hard_limit, "unmetered")
        };
        use mgo_protocol_config::check_limit_in_range;
        let result = check_limit_in_range($x as u64, $metered_limit, h);
        match result {
            LimitThresholdCrossed::None => {}
            LimitThresholdCrossed::Soft(_, _) => {
                $metric.with_label_values(&[metered_str, "soft"]).inc();
            }
            LimitThresholdCrossed::Hard(_, _) => {
                $metric.with_label_values(&[metered_str, "hard"]).inc();
            }
        };
        result
    }};
}

#[cfg(all(test, not(msim)))]
mod test {
    use super::*;
    use insta::assert_yaml_snapshot;

    #[test]
    fn snapshot_tests() {
        println!("\n============================================================================");
        println!("!                                                                          !");
        println!("! IMPORTANT: never update snapshots from this test. only add new versions! !");
        println!("!                                                                          !");
        println!("============================================================================\n");
        for chain_id in &[Chain::Unknown, Chain::Mainnet, Chain::Testnet] {
            // make Chain::Unknown snapshots compatible with pre-chain-id snapshots so that we
            // don't break the release-time compatibility tests. Once Chain Id configs have been
            // released everywhere, we can remove this and only test Mainnet and Testnet
            let chain_str = match chain_id {
                Chain::Unknown => "".to_string(),
                _ => format!("{:?}_", chain_id),
            };
            for i in MIN_PROTOCOL_VERSION..=MAX_PROTOCOL_VERSION {
                let cur = ProtocolVersion::new(i);
                assert_yaml_snapshot!(
                    format!("{}version_{}", chain_str, cur.as_u64()),
                    ProtocolConfig::get_for_version(cur, *chain_id)
                );
            }
        }
    }

    #[test]
    fn test_getters() {
        let prot: ProtocolConfig =
            ProtocolConfig::get_for_version(ProtocolVersion::new(1), Chain::Unknown);
        assert_eq!(
            prot.max_arguments(),
            prot.max_arguments_as_option().unwrap()
        );
    }

    #[test]
    fn test_setters() {
        let mut prot: ProtocolConfig =
            ProtocolConfig::get_for_version(ProtocolVersion::new(1), Chain::Unknown);
        prot.set_max_arguments_for_testing(123);
        assert_eq!(prot.max_arguments(), 123);

        prot.set_max_arguments_from_str_for_testing("321".to_string());
        assert_eq!(prot.max_arguments(), 321);

        prot.disable_max_arguments_for_testing();
        assert_eq!(prot.max_arguments_as_option(), None);

        prot.set_attr_for_testing("max_arguments".to_string(), "456".to_string());
        assert_eq!(prot.max_arguments(), 456);
    }

    #[test]
    fn lookup_by_string_test() {
        let prot: ProtocolConfig =
            ProtocolConfig::get_for_version(ProtocolVersion::new(1), Chain::Unknown);
        // Does not exist
        assert!(prot.lookup_attr("some random string".to_string()).is_none());

        assert!(
            prot.lookup_attr("max_arguments".to_string())
                == Some(ProtocolConfigValue::u32(prot.max_arguments())),
        );

        // We didnt have this in version 1
        assert!(prot
            .lookup_attr("max_move_identifier_len".to_string())
            .is_none());

        // But we did in version 9
        let prot: ProtocolConfig =
            ProtocolConfig::get_for_version(ProtocolVersion::new(9), Chain::Unknown);
        assert!(
            prot.lookup_attr("max_move_identifier_len".to_string())
                == Some(ProtocolConfigValue::u64(prot.max_move_identifier_len()))
        );

        let prot: ProtocolConfig =
            ProtocolConfig::get_for_version(ProtocolVersion::new(1), Chain::Unknown);
        // We didnt have this in version 1
        assert!(prot
            .attr_map()
            .get("max_move_identifier_len")
            .unwrap()
            .is_none());
        // We had this in version 1
        assert!(
            prot.attr_map().get("max_arguments").unwrap()
                == &Some(ProtocolConfigValue::u32(prot.max_arguments()))
        );

        // Check feature flags
        let prot: ProtocolConfig =
            ProtocolConfig::get_for_version(ProtocolVersion::new(1), Chain::Unknown);
        // Does not exist
        assert!(prot
            .feature_flags
            .lookup_attr("some random string".to_owned())
            .is_none());
        assert!(prot
            .feature_flags
            .attr_map()
            .get("some random string")
            .is_none());

        // Was false in v1
        assert!(
            prot.feature_flags
                .lookup_attr("package_upgrades".to_owned())
                == Some(false)
        );
        assert!(
            prot.feature_flags
                .attr_map()
                .get("package_upgrades")
                .unwrap()
                == &false
        );
        let prot: ProtocolConfig =
            ProtocolConfig::get_for_version(ProtocolVersion::new(4), Chain::Unknown);
        // Was true from v3 and up
        assert!(
            prot.feature_flags
                .lookup_attr("package_upgrades".to_owned())
                == Some(true)
        );
        assert!(
            prot.feature_flags
                .attr_map()
                .get("package_upgrades")
                .unwrap()
                == &true
        );
    }

    #[test]
    fn limit_range_fn_test() {
        let low = 100u32;
        let high = 10000u64;

        assert!(check_limit!(1u8, low, high) == LimitThresholdCrossed::None);
        assert!(matches!(
            check_limit!(255u16, low, high),
            LimitThresholdCrossed::Soft(255u128, 100)
        ));
        // This wont compile because lossy
        //assert!(check_limit!(100000000u128, low, high) == LimitThresholdCrossed::None);
        // This wont compile because lossy
        //assert!(check_limit!(100000000usize, low, high) == LimitThresholdCrossed::None);

        assert!(matches!(
            check_limit!(2550000u64, low, high),
            LimitThresholdCrossed::Hard(2550000, 10000)
        ));

        assert!(matches!(
            check_limit!(2550000u64, high, high),
            LimitThresholdCrossed::Hard(2550000, 10000)
        ));

        assert!(matches!(
            check_limit!(1u8, high),
            LimitThresholdCrossed::None
        ));

        assert!(check_limit!(255u16, high) == LimitThresholdCrossed::None);

        assert!(matches!(
            check_limit!(2550000u64, high),
            LimitThresholdCrossed::Hard(2550000, 10000)
        ));
    }
}
