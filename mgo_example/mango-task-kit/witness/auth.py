"""Route-bound Ed25519 attestations for the S6 test entitlement service."""

from dataclasses import asdict, dataclass
import hashlib
import re

from cryptography.exceptions import InvalidSignature
from cryptography.hazmat.primitives.asymmetric.ed25519 import Ed25519PrivateKey, Ed25519PublicKey


_ISSUE_DOMAIN = "mango-s6-testnet-issue-v1"
_ACTION = "issue_test_entitlement"
_FINALITY_RULE = "bsc-finalized-v1"
_HEX20 = r"0x[0-9a-f]{40}"
_HEX32 = r"0x[0-9a-f]{64}"
_BASE58 = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz"
_MAX_INTEGER = (1 << 53) - 1


def _require(condition: bool, message: str) -> None:
    if not condition:
        raise ValueError(message)


def _integer(value: int, field: str, minimum: int = 0) -> None:
    _require(type(value) is int and minimum <= value <= _MAX_INTEGER, f"Invalid {field}")


def _matches(value: str, expression: str, field: str) -> None:
    _require(isinstance(value, str) and re.fullmatch(expression, value) is not None, f"Invalid {field}")


def _target_digest(value: str) -> None:
    _matches(value, f"[{_BASE58}]{{32,44}}", "target genesis digest")
    number = 0
    for char in value:
        number = 58 * number + _BASE58.index(char)
    leading_zeros = len(value) - len(value.lstrip("1"))
    _require(leading_zeros + (number.bit_length() + 7) // 8 == 32, "Invalid target genesis digest")


def _encode(fields: tuple[str, ...]) -> bytes:
    result = bytearray()
    for field in fields:
        encoded = field.encode("utf-8")
        result.extend(len(encoded).to_bytes(4, "big"))
        result.extend(encoded)
    return bytes(result)


@dataclass(frozen=True)
class RouteBinding:
    route_id: str
    route_revision: int
    source_chain_id: int
    source_genesis_hash: str
    source_contract: str
    source_event_topic: str
    target_genesis_digest: str
    target_package_id: str
    recipient_mango: str

    def __post_init__(self) -> None:
        _matches(self.route_id, r"[A-Za-z0-9][A-Za-z0-9._-]{0,63}", "route ID")
        _integer(self.route_revision, "route revision", 1)
        _integer(self.source_chain_id, "source chain ID", 1)
        _matches(self.source_genesis_hash, _HEX32, "source genesis hash")
        _matches(self.source_contract, _HEX20, "source contract")
        _matches(self.source_event_topic, _HEX32, "source event topic")
        _target_digest(self.target_genesis_digest)
        _matches(self.target_package_id, _HEX32, "target package ID")
        _matches(self.recipient_mango, _HEX32, "target recipient")


@dataclass(frozen=True)
class SourceObservation:
    """A log matched to a canonical finalized block by the testnet observer."""

    event_id: str
    tx_hash: str
    block_hash: str
    block_height: int
    log_index: int
    payload_digest: str
    finalized_height: int
    finalized_hash: str

    def __post_init__(self) -> None:
        for value, name in (
            (self.event_id, "event ID"),
            (self.tx_hash, "source transaction hash"),
            (self.block_hash, "source block hash"),
            (self.payload_digest, "business payload digest"),
            (self.finalized_hash, "finalized block hash"),
        ):
            _matches(value, _HEX32, name)
        _integer(self.block_height, "source block height")
        _integer(self.log_index, "source log index")
        _integer(self.finalized_height, "finalized height", self.block_height)
        _require(self.finalized_height != self.block_height or self.finalized_hash == self.block_hash,
                 "Conflicting hashes for the same source block height")


@dataclass(frozen=True)
class WitnessEpoch:
    epoch: int
    start_height: int
    public_key_hex: str
    end_height_exclusive: int | None = None
    activation_finalized_hash: str | None = None

    def __post_init__(self) -> None:
        _integer(self.epoch, "policy epoch", 1)
        _integer(self.start_height, "epoch start height")
        _matches(self.public_key_hex, r"[0-9a-f]{64}", "Ed25519 public key")
        if self.end_height_exclusive is not None:
            _integer(self.end_height_exclusive, "epoch end height", self.start_height + 1)
        if self.activation_finalized_hash is not None:
            _matches(self.activation_finalized_hash, _HEX32, "activation finalized hash")


@dataclass(frozen=True)
class WitnessPolicy:
    route: RouteBinding
    epochs: tuple[WitnessEpoch, ...]
    max_ttl_seconds: int = 300

    def __post_init__(self) -> None:
        _require(isinstance(self.route, RouteBinding), "Invalid registered route")
        _require(type(self.epochs) is tuple and bool(self.epochs), "Missing immutable witness history")
        _require(all(isinstance(key, WitnessEpoch) for key in self.epochs), "Invalid witness history")
        _integer(self.max_ttl_seconds, "maximum witness TTL", 1)
        keys = {key.public_key_hex for key in self.epochs}
        _require(len(keys) == len(self.epochs), "Cannot reregister a retired witness key")
        for previous, current in zip(self.epochs, self.epochs[1:]):
            _require(current.epoch == previous.epoch + 1, "Witness epochs must be consecutive")
            _require(previous.end_height_exclusive is not None
                     and previous.end_height_exclusive <= current.start_height,
                     "Witness epoch ranges overlap")
            _require(current.activation_finalized_hash is not None, "Witness rotation has no finalized anchor")

    def public_document(self) -> dict:
        """Public route and key history. Archive it as a versioned, trusted configuration."""
        return asdict(self)

    @classmethod
    def from_public_document(cls, document: dict) -> "WitnessPolicy":
        """Restore a pinned policy snapshot supplied by the operator."""
        try:
            fields = {
                **document,
                "route": RouteBinding(**document["route"]),
                "epochs": tuple(WitnessEpoch(**key) for key in document["epochs"]),
            }
            return cls(**fields)
        except (KeyError, TypeError) as error:
            raise ValueError("Invalid witness policy document") from error


@dataclass(frozen=True)
class IssueMessage:
    route: RouteBinding
    observation: SourceObservation
    policy_epoch: int
    issued_at: int
    expires_at: int

    def __post_init__(self) -> None:
        _require(isinstance(self.route, RouteBinding) and isinstance(self.observation, SourceObservation),
                 "Invalid issue bindings")
        _integer(self.policy_epoch, "policy epoch", 1)
        _integer(self.issued_at, "issued time", 1)
        _integer(self.expires_at, "expiry time", self.issued_at + 1)

    @property
    def task_id(self) -> str:
        fields = ("mango-business-task-v1", self.route.source_genesis_hash, self.route.source_contract,
                  self.observation.event_id, self.route.route_id)
        return "0x" + hashlib.sha256(_encode(fields)).hexdigest()

    def canonical_bytes(self) -> bytes:
        """Encode the issue domain and the exact ordered binding with u32 byte lengths."""
        route, source = self.route, self.observation
        return _encode((
            _ISSUE_DOMAIN, _ACTION, self.task_id, route.route_id, str(route.route_revision),
            str(route.source_chain_id), route.source_genesis_hash, route.source_contract,
            route.source_event_topic, source.event_id, source.tx_hash, source.block_hash,
            str(source.block_height), str(source.log_index), source.payload_digest,
            _FINALITY_RULE, str(source.finalized_height), source.finalized_hash,
            route.target_genesis_digest, route.target_package_id, route.recipient_mango,
            str(self.policy_epoch), str(self.issued_at), str(self.expires_at),
        ))


@dataclass(frozen=True)
class SignedIssue:
    message: IssueMessage
    signature_hex: str

    def __post_init__(self) -> None:
        _require(isinstance(self.message, IssueMessage), "Invalid signed issue")
        _matches(self.signature_hex, r"[0-9a-f]{128}", "Ed25519 signature")

    def to_document(self) -> dict:
        return asdict(self)

    @classmethod
    def from_document(cls, document: dict) -> "SignedIssue":
        try:
            unsigned = dict(document["message"])
            unsigned["route"] = RouteBinding(**unsigned["route"])
            unsigned["observation"] = SourceObservation(**unsigned["observation"])
            fields = {**document, "message": IssueMessage(**unsigned)}
            return cls(**fields)
        except (KeyError, TypeError) as error:
            raise ValueError("Invalid signed issue document") from error


def sign_issue(message: IssueMessage, private_key: Ed25519PrivateKey) -> SignedIssue:
    _require(isinstance(private_key, Ed25519PrivateKey), "Witness key must be Ed25519")
    return SignedIssue(message, private_key.sign(message.canonical_bytes()).hex())


def _verify_at(policy: WitnessPolicy, signed: SignedIssue, expected_observation: SourceObservation,
               accepted_at: int) -> IssueMessage:
    message = signed.message
    _integer(accepted_at, "trusted admission time", 1)
    _require(message.route == policy.route, "Witness route does not match the registered route")
    _require(message.observation == expected_observation, "Witness observation does not match retained RPC evidence")
    _require(message.issued_at <= accepted_at < message.expires_at
             and message.expires_at - message.issued_at <= policy.max_ttl_seconds,
             "Witness issue time or expiry is outside policy")
    key = next((epoch for epoch in policy.epochs if epoch.epoch == message.policy_epoch), None)
    height = message.observation.block_height
    _require(key is not None and key.start_height <= height
             and (key.end_height_exclusive is None or height < key.end_height_exclusive),
             "Signing epoch does not cover this source block height")
    public_key = Ed25519PublicKey.from_public_bytes(bytes.fromhex(key.public_key_hex))
    try:
        public_key.verify(bytes.fromhex(signed.signature_hex), message.canonical_bytes())
    except InvalidSignature as error:
        raise ValueError("Invalid witness signature") from error
    return message


def verify_fresh_issue(policy: WitnessPolicy, signed: SignedIssue, expected_observation: SourceObservation,
                       now: int) -> IssueMessage:
    """Authenticate a new task against the current active key and live observation."""
    active = policy.epochs[-1]
    _require(active.end_height_exclusive is None and signed.message.policy_epoch == active.epoch,
             "Fresh issue requires the current open signing epoch")
    return _verify_at(policy, signed, expected_observation, now)


def verify_stored_issue(policy: WitnessPolicy, signed: SignedIssue, expected_observation: SourceObservation,
                        admitted_at: int) -> IssueMessage:
    """Reauthenticate an existing admitted task. Never use request-provided admission time."""
    return _verify_at(policy, signed, expected_observation, admitted_at)


def rotate_after_finalized_block(policy: WitnessPolicy, new_public_key_hex: str, finalized_height: int,
                                 finalized_hash: str) -> WitnessPolicy:
    """Close the existing range at H, then activate the replacement starting at H+1."""
    active = policy.epochs[-1]
    _require(active.end_height_exclusive is None, "Rotation requires a current open signing epoch")
    _integer(finalized_height, "rotation finalized height", active.start_height)
    _matches(finalized_hash, _HEX32, "rotation finalized hash")
    next_start = finalized_height + 1
    retired = WitnessEpoch(active.epoch, active.start_height, active.public_key_hex, next_start,
                           active.activation_finalized_hash)
    replacement = WitnessEpoch(active.epoch + 1, next_start, new_public_key_hex, None, finalized_hash)
    return WitnessPolicy(policy.route, (*policy.epochs[:-1], retired, replacement), policy.max_ttl_seconds)
