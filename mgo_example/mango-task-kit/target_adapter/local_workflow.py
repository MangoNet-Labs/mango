"""Process an authorized entitlement against the isolated local Mango fixture."""

import json
import re
from subprocess import SubprocessError
from time import monotonic, sleep
from urllib.parse import urlsplit
from uuid import uuid4


def _require(condition, message):
    if not condition:
        raise RuntimeError(message)


def _sql_string(value):
    return "'" + value.replace("'", "''") + "'"


def _sql_json(value):
    return _sql_string(json.dumps(value, separators=(",", ":"))) + "::jsonb"


def _fence(row):
    return [
        _sql_string(row["task_id"]), _sql_string(row["lease_owner"]),
        str(row["lease_fencing_token"]), str(row["task_revision"]),
    ]


def _read_task(db, task_id, source):
    data = db.query(f"""
        SELECT to_jsonb(t) || jsonb_build_object(
            'active_lease', t.lease_expires_at > clock_timestamp(),
            'source_status', (SELECT o.canonical_status FROM source_observations o
                WHERE o.task_id = t.task_id AND o.evidence = {_sql_json(source)}
                  AND o.tx_hash = o.evidence ->> 'source_transaction_hash'
                  AND o.block_hash = o.evidence ->> 'source_block_hash'
                  AND to_jsonb(o.source_height) = o.evidence -> 'source_height'
                  AND to_jsonb(o.log_index) = o.evidence -> 'source_log_index'))
        FROM tasks t WHERE t.task_id = {_sql_string(task_id)};
    """)
    _require(data, "No task admitted by the fixture scanner")
    return json.loads(data)


def _check_lease_and_source(db, row, source):
    current = _read_task(db, row["task_id"], source)
    _require(current["active_lease"] is True and _fence(current) == _fence(row),
             "The worker no longer owns the current lease")
    _require(current["source_status"] == "CANONICAL", "The fixture observation is no longer canonical")


def _result(row, digest, receipt, effect, *, replayed, send_requests):
    return {
        **effect,
        "replayed": replayed,
        "digest": digest,
        "receipt": receipt,
        "attempt_count": row["attempt_count"],
        "entitlement_id": effect["registry_task_binding"]["entitlement"],
        "registry_count": effect.get("registry_count"),
        "send_requests": send_requests,
    }


def _read_success(db, row):
    data = db.query(f"""
        SELECT jsonb_build_object('digest', e.transaction_digest,
            'receipt', r.confirmed_receipt, 'effect', e.confirmed_effect)
        FROM receipts r JOIN target_effects e USING (task_id)
        WHERE r.task_id = {_sql_string(row['task_id'])} AND e.effect_status = 'SUCCESS'
          AND e.attempt_number = {row['attempt_count']};
    """)
    _require(data, "The successful task lacks a persisted effect or receipt")
    saved = json.loads(data)
    return _result(row, **saved, replayed=True, send_requests=0)


def _read_intent(db, row, *, confirmed_failure=False):
    data = db.query(f"""
        SELECT jsonb_build_object('digest', transaction_digest, 'outcome', outcome,
            'matches', route_snapshot = {_sql_json(row['route_snapshot'])}
                AND target_genesis_digest = route_snapshot ->> 'target_genesis_digest'
                AND target_package_id = route_snapshot ->> 'target_package_id'
                AND signed_bytes_sha256 = encode(sha256(signed_transaction_bytes), 'hex'))
        FROM submission_intents WHERE task_id = {_sql_string(row['task_id'])}
          AND attempt_number = {row['attempt_count']};
    """)
    _require(data, "The current task attempt lacks a persisted intent")
    intent = json.loads(data)
    outcomes = {"CONFIRMED_FAILURE"} if confirmed_failure else {"INTENT_PERSISTED", "SUBMITTED", "BROADCAST_UNKNOWN"}
    _require(intent["matches"] and intent["outcome"] in outcomes,
             "The existing intent cannot be reconciled by this local workflow")
    return intent["digest"]


def _lookup_transaction(target, digest):
    not_found = {
        "code": -32602,
        "message": f"Could not find the referenced transaction [TransactionDigest({digest})].",
    }
    deadline = monotonic() + 4
    response = target.lookup(digest)
    while (isinstance(response, dict) and "result" not in response and response.get("error") == not_found
           and type(response["error"]["code"]) is int):
        remaining = deadline - monotonic()
        if remaining <= 0:
            break
        sleep(min(0.25, remaining))
        if monotonic() >= deadline:
            break
        response = target.lookup(digest)

    result = response.get("result") if isinstance(response, dict) and "error" not in response else None
    effects = result.get("effects") if isinstance(result, dict) else None
    status = effects.get("status") if isinstance(effects, dict) else None
    _require(isinstance(status, dict) and status.get("status") in {"success", "failure"}
             and result.get("digest") == effects.get("transactionDigest") == digest
             and not result.get("errors"), "The original transaction digest was not confirmed by local Mango")
    return result


def _registry_count(target):
    registry = target.chain.client("Read local entitlement registry", "object", target.state["registry_id"])
    return int(registry["content"]["fields"]["issuance_count"])


def _failure_proof(target, task_id, transaction):
    effects, checkpoint = transaction["effects"], transaction.get("checkpoint")
    events, changes = transaction.get("events"), transaction.get("objectChanges")
    _require(effects["status"]["status"] == "failure" and effects["status"].get("error")
             and type(checkpoint) in (str, int) and re.fullmatch(r"[0-9]+", str(checkpoint))
             and effects.get("created", []) == [] and not effects.get("eventsDigest") and events == []
             and isinstance(changes, list) and all(change["type"] != "created" for change in changes),
             "The original transaction lacks finalized failure evidence without issuance")
    absence = target.registry_binding(task_id).get("result")
    _require(isinstance(absence, dict) and not absence.get("data") and absence.get("error") == {
        "code": "dynamicFieldNotFound", "parent_object_id": target.state["registry_id"],
    }, "The local registry did not confirm that the task has no issued entitlement")
    return {"effects": effects, "checkpoint": checkpoint, "events": events, "object_changes": changes,
            "registry_binding_absence": absence, "registry_count": _registry_count(target)}


def _read_failure(db, row):
    digest = _read_intent(db, row, confirmed_failure=True)
    stored = db.query(f"""
        SELECT confirmed_effect FROM target_effects
        WHERE task_id = {_sql_string(row['task_id'])} AND attempt_number = {row['attempt_count']}
          AND transaction_digest = {_sql_string(digest)} AND effect_status = 'FAILURE';
    """)
    _require(stored, "The original attempt lacks a persisted, confirmed failure")
    proof = json.loads(stored)
    _require(proof.get("checkpoint") is not None and proof.get("effects", {}).get("status", {}).get("status") == "failure"
             and proof["effects"].get("transactionDigest") == digest,
             "The persisted effect lacks finalized evidence for the original failed transaction")
    return digest, proof


def _failure_result(row, digest, proof, *, replayed, send_requests):
    return {**proof, "outcome": "CONFIRMED_FAILURE", "state": row["state"], "reason_code": row["last_error_code"],
            "task_id": row["task_id"], "task_revision": row["task_revision"], "digest": digest,
            "attempt_count": row["attempt_count"], "send_requests": send_requests, "replayed": replayed,
            "receipt": None, "entitlement_id": None}


def _check_retry_action(db, row, action_id, *, claimed=False):
    accepted_revision = row["task_revision"] - int(claimed)
    accepted = db.query(f"""
        SELECT accepted_response FROM operator_actions
        WHERE task_id = {_sql_string(row['task_id'])} AND action_id = {_sql_string(action_id)}
          AND action = 'RETRY_CONFIRMED_FAILURE' AND work_status = 'PENDING'
          AND expected_task_revision + 1 = {accepted_revision};
    """)
    _require(accepted, "The operator retry action is missing or no longer matches the current task revision")
    _require(json.loads(accepted) == {
        "action_id": action_id, "task_id": row["task_id"], "accepted_task_revision": accepted_revision,
        "action": "RETRY_CONFIRMED_FAILURE", "task_identity_unchanged": True, "creates_target_attempt_now": False,
    }, "The operator retry acknowledgement does not authorize this task revision")


def _record_intent(db, row, domain, digest, signed, signed_sha, *, retry):
    values = [_sql_string(domain["target_genesis_digest"]), _sql_string(domain["target_package_id"]),
              _sql_string(digest), f"decode('{signed.hex()}', 'hex')", _sql_string(signed_sha)]
    if not retry:
        return db.row("record_submission_intent", [*_fence(row), *values])
    _require(re.fullmatch(r"[1-9A-HJ-NP-Za-km-z]{30,60}", digest) and re.fullmatch(r"[a-f0-9]{64}", signed_sha),
             "The retry did not produce a valid local transaction digest and byte hash")
    # If either fenced procedure returns no row, the incomplete retry is rolled back.
    stored = db.query(f"""
        BEGIN;
        DO $retry$
        DECLARE resumed tasks; recorded tasks;
        BEGIN
            SELECT * INTO STRICT resumed FROM advance_task({','.join(_fence(row))}, 'READY');
            SELECT * INTO STRICT recorded FROM record_submission_intent(
                resumed.task_id, resumed.lease_owner, resumed.lease_fencing_token, resumed.task_revision,
                {','.join(values)});
        END;
        $retry$;
        SELECT to_jsonb(t) FROM tasks t WHERE task_id = {_sql_string(row['task_id'])};
        COMMIT;
    """)
    _require(stored, "The authorized retry intent was not persisted")
    return json.loads(stored)



def _hold_unsupported_version(db, row, source, lease_seconds):
    reason = "UNSUPPORTED_TASK_VERSION"
    _require(row["attempt_count"] == 0, "An unsupported in-flight task cannot be activated")
    if row["state"] != "MANUAL_REVIEW":
        owner = "local-version-review-" + uuid4().hex
        revision = row["task_revision"]
        claimed = db.row("claim_task", [_sql_string(row["task_id"]), _sql_string(owner), str(lease_seconds)])
        try:
            _require(claimed["task_revision"] == revision + 1 and claimed["attempt_count"] == 0,
                     "The pending task changed before the version hold")
            _check_lease_and_source(db, claimed, source)
            if claimed["state"] == "OBSERVED":
                claimed = db.transition(claimed, "WAITING_SOURCE")
            claimed = db.transition(claimed, "MANUAL_REVIEW", reason)
        finally:
            row = db.row("release_task", _fence(claimed))
    _require(row["last_error_code"] == reason, "The task is held for a different review reason")
    return {"outcome": "MANUAL_REVIEW", "state": row["state"], "task_id": row["task_id"],
            "route_revision_snapshot": row["route_revision_snapshot"], "reason_code": reason,
            "attempt_count": 0, "send_requests": 0, "receipt": None}


def process_local_task(db, target, binding, source, payload, *, verify_issue, build_receipt,
                       gas_budget, lease_seconds=240, retry_action_id=None):
    """Use the scanner's persistent task and an already validated fixture binding.

    The database owns task state and lease fencing. A later call can reconcile a
    committed attempt by digest, including when its submission response was lost.
    Exactly one retry can be authorized by an operator after finalized failure.
    """
    _require(type(lease_seconds) is int and 1 <= lease_seconds <= 2_147_483_647, "Invalid task lease duration")
    state = target.state
    isolation = state["isolation"]
    rpc = urlsplit(isolation["rpc"])
    _require(isolation["network"] == "none" and rpc.scheme == "http" and rpc.hostname == "127.0.0.1",
             "This workflow requires an isolated loopback Mango target")
    domain = {key: binding[key] for key in ("target_genesis_digest", "target_package_id", "recipient")}
    _require(binding.get("authorized") is True
             and source.get("is_simulated") is True and source.get("evidence_label") == "LOCAL_FIXTURE"
             and domain["target_genesis_digest"] == state["target_genesis_digest"]
             and domain["target_package_id"] == state["package_id"]
             and source["source_genesis_hash"] != domain["target_genesis_digest"]
             and payload["recipient_mango"] == domain["recipient"]
             and payload["action"] == "issue_test_entitlement",
             "The authorization, source, payload or local target domain does not match")

    task_id = binding["task_id"]
    row = _read_task(db, task_id, source)
    route = row["route_snapshot"]
    _require(all(row[key] == source[key] for key in ("source_genesis_hash", "source_contract", "business_event_id"))
             and row["stable_route_id"] == "demo-entitlement"
             and row["business_payload"] == payload
             and row["business_payload_digest"] == binding["business_payload_digest"] == source["emitted_payload_digest"]
             and route["target_genesis_digest"] == domain["target_genesis_digest"]
             and route["target_package_id"] == domain["target_package_id"]
             and route.get("action", payload["action"]) == payload["action"]
             and route.get("recipient", domain["recipient"]) == domain["recipient"],
             "The fixture binding differs from the persisted task or route snapshot")
    if row["state"] == "SUCCEEDED":
        _require(row["source_status"], "The source observation is absent from the stored lineage")
        return _read_success(db, row)

    _require(row["source_status"] == "CANONICAL", "The fixture observation is not canonical")
    if row["route_revision_snapshot"] != 1 or row["policy_snapshot"] != {
        "mode": "FIXTURE_ONLY_NO_SIGNATURE", "policy_version": 1
    }:
        return _hold_unsupported_version(db, row, source, lease_seconds)
    failed = row["state"] == "RETRY_WAIT" and row["last_error_code"] == "TARGET_CONFIRMED_FAILURE"
    retry = retry_action_id is not None
    if retry:
        _require(isinstance(retry_action_id, str) and re.fullmatch(r"[a-zA-Z0-9][a-zA-Z0-9._:-]{0,127}", retry_action_id)
                 and failed and row["attempt_count"] == 1 and row["resume_phase"] == "READY",
                 "A retry requires the first confirmed failure and a current operator action")
        _check_retry_action(db, row, retry_action_id)
    elif failed:
        digest, proof = _read_failure(db, row)
        return _failure_result(row, digest, proof, replayed=True, send_requests=0)

    owner = "local-entitlement-" + uuid4().hex
    row = db.row("claim_task", [_sql_string(task_id), _sql_string(owner), str(lease_seconds)])
    send_count = target.send_count
    try:
        if retry:
            _require(row["state"] == "RETRY_WAIT" and row["resume_phase"] == "READY"
                     and row["last_error_code"] == "TARGET_CONFIRMED_FAILURE" and row["attempt_count"] == 1,
                     "The claimed task no longer has the first confirmed failure")
            _check_retry_action(db, row, retry_action_id, claimed=True)
            failed_digest, persisted_failure = _read_failure(db, row)
            current_failure = _failure_proof(target, task_id, _lookup_transaction(target, failed_digest))
            _require(all(current_failure[key] == persisted_failure[key] for key in ("effects", "checkpoint")),
                     "The original finalized failure changed after operator approval")
        elif row["attempt_count"] == 0:
            if row["state"] in {"OBSERVED", "SOURCE_INVALIDATED"}:
                row = db.transition(row, "WAITING_SOURCE")
            transitions = ("WAITING_SOURCE", "SOURCE_CONFIRMED", "AUTHENTICATED", "READY")
            _require(row["state"] in transitions, "The task cannot submit its first local attempt")
            for phase in transitions[transitions.index(row["state"]) + 1:]:
                row = db.transition(row, phase)

        if retry or row["attempt_count"] == 0:
            _check_lease_and_source(db, row, source)
            digest, signed, signed_sha = target.prepare_issue(binding, gas_budget)
            _require(not retry or digest != failed_digest, "The retry attempted to reuse the original failed digest")
            row = _record_intent(db, row, domain, digest, signed, signed_sha, retry=retry)
            _require(_read_intent(db, row) == digest, "The committed intent differs from the prepared digest")
            _check_lease_and_source(db, row, source)
            try:
                response = target.send_once(signed).get("result")
            except (OSError, SubprocessError, RuntimeError, ValueError):
                response = None
            if isinstance(response, dict) and response.get("digest") == digest:
                row = db.transition(row, "TARGET_SUBMITTED")
            else:
                row = db.transition(row, "TARGET_OUTCOME_UNKNOWN", "RESPONSE_UNKNOWN")
        else:
            digest = _read_intent(db, row)
            if row["state"] == "READY":
                row = db.transition(row, "TARGET_OUTCOME_UNKNOWN", "PERSISTED_INTENT_RECOVERY")

        _require(row["state"] in {"TARGET_SUBMITTED", "TARGET_OUTCOME_UNKNOWN"},
                 "The task cannot reconcile its existing attempt")
        try:
            transaction = _lookup_transaction(target, digest)
            if transaction["effects"]["status"]["status"] == "failure":
                deadline = monotonic() + 5
                while transaction.get("checkpoint") is None and monotonic() < deadline:
                    sleep(0.2)
                    transaction = _lookup_transaction(target, digest)
                failure = _failure_proof(target, task_id, transaction)
            else:
                failure = None
        except (OSError, SubprocessError, RuntimeError, ValueError):
            if row["state"] != "TARGET_OUTCOME_UNKNOWN":
                row = db.transition(row, "TARGET_OUTCOME_UNKNOWN", "LOOKUP_UNCONFIRMED")
            raise

        if failure is not None:
            _check_lease_and_source(db, row, source)
            row = db.row("record_target_failure", [*_fence(row), str(row["attempt_count"]), _sql_string(digest),
                                                    _sql_json(failure)])
            row = db.row("release_task", _fence(row))
            return _failure_result(row, digest, failure, replayed=False, send_requests=target.send_count - send_count)

        effects = transaction["effects"]
        entitlement_id, event = verify_issue(transaction, state, domain["target_package_id"], binding)
        _require(event["id"]["txDigest"] == digest, "The issuance event belongs to another transaction")
        field = target.registry_binding(task_id)["result"]["data"]["content"]["fields"]
        registry_binding = field["value"]["fields"]
        _require(field["name"] == list(bytes.fromhex(task_id[2:])) and registry_binding == {
            "business_payload_digest": list(bytes.fromhex(binding["business_payload_digest"][2:])),
            "entitlement": entitlement_id,
            "recipient": domain["recipient"],
        }, "The registry entitlement differs from the original transaction binding")
        registry_count = _registry_count(target)
        _require(registry_count > 0, "The registry does not contain a recorded issuance")
        receipt = build_receipt(binding, source, domain, transaction, state)
        _require(receipt["evidence_class"] == "LOCAL_FIXTURE_SOURCE_REAL_TARGET"
                 and any(item["reference"]["target_transaction_digest"] == digest for item in receipt["evidence"]
                         if item["evidence_role"] == "TARGET"),
                 "The receipt does not preserve the fixture source and original target transaction")
        effect = {"effects": effects, "event": event, "registry_task_binding": registry_binding,
                  "registry_count": registry_count}
        _check_lease_and_source(db, row, source)
        row = db.row("complete_task", [*_fence(row), str(row["attempt_count"]), _sql_string(digest),
                                       _sql_json(effect), _sql_json(receipt)])
        return _result(row, digest, receipt, effect, replayed=False, send_requests=target.send_count - send_count)
    except Exception as error:
        try:
            db.query(f"SELECT 1 FROM release_task({','.join(_fence(row))});")
        except (OSError, SubprocessError, RuntimeError, ValueError) as release_error:
            error.add_note(f"The fenced lease could not be released: {release_error}")
        raise
