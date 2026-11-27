"""Compare a fixed local source range with stored evidence and its Mango registry."""

import hashlib
import json
import re
from subprocess import SubprocessError
from urllib.parse import urlsplit


READ_ERRORS = (OSError, SubprocessError, RuntimeError, ValueError, TypeError, KeyError, AttributeError)
PENDING_REASONS = {
    "OBSERVED": "AWAITING_SOURCE_CONFIRMATION",
    "WAITING_SOURCE": "AWAITING_SOURCE_CONFIRMATION",
    "SOURCE_CONFIRMED": "AWAITING_AUTHORIZATION",
    "AUTHENTICATED": "AWAITING_TARGET_BINDING",
    "READY": "AWAITING_SUBMISSION",
    "TARGET_SUBMITTED": "AWAITING_TARGET_CONFIRMATION",
    "TARGET_OUTCOME_UNKNOWN": "TARGET_OUTCOME_UNKNOWN",
    "RETRY_WAIT": "AWAITING_RETRY",
    "MANUAL_REVIEW": "MANUAL_REVIEW_REQUIRED",
    "REJECTED": "TASK_REJECTED",
    "SOURCE_INVALIDATED": "SOURCE_INVALIDATED",
}


def _hex_id(value):
    return isinstance(value, str) and re.fullmatch(r"0x[0-9a-f]{64}", value) is not None


def _digest(fields):
    digest = hashlib.sha256()
    for field in fields:
        value = field.encode()
        digest.update(len(value).to_bytes(4, "big"))
        digest.update(value)
    return "0x" + digest.hexdigest()


def _snapshot(db, source_genesis, target_genesis, package_id):
    return json.loads(db.query(f"""
        SELECT jsonb_build_object(
            'tasks', COALESCE((SELECT jsonb_agg(jsonb_build_object(
                'task', to_jsonb(t),
                'intents', COALESCE((SELECT jsonb_agg((to_jsonb(i) - 'signed_transaction_bytes') ||
                    jsonb_build_object('bytes_hash_matches',
                        i.signed_bytes_sha256 = encode(sha256(i.signed_transaction_bytes), 'hex'))
                    ORDER BY i.attempt_number) FROM public.submission_intents i WHERE i.task_id = t.task_id), '[]'::jsonb),
                'effects', COALESCE((SELECT jsonb_agg(to_jsonb(e) ORDER BY e.attempt_number)
                    FROM public.target_effects e WHERE e.task_id = t.task_id), '[]'::jsonb),
                'receipt', (SELECT to_jsonb(r) FROM public.receipts r WHERE r.task_id = t.task_id),
                'source_lineage', COALESCE((SELECT jsonb_agg(to_jsonb(o) ORDER BY o.observation_id)
                    FROM public.source_observations o WHERE o.task_id = t.task_id), '[]'::jsonb)
            ) ORDER BY t.task_id) FROM public.tasks t WHERE t.source_genesis_hash = '{source_genesis}'), '[]'::jsonb),
            'other_source_registry_success_count', (SELECT count(*) FROM public.tasks t
                JOIN public.target_effects e ON e.task_id = t.task_id AND e.effect_status = 'SUCCESS'
                WHERE t.source_genesis_hash <> '{source_genesis}' AND t.state = 'SUCCEEDED'
                  AND t.route_snapshot ->> 'target_genesis_digest' = '{target_genesis}'
                  AND t.route_snapshot ->> 'target_package_id' = '{package_id}'));
    """))


def _reason(task, lineage):
    if task["state"] == "SUCCEEDED":
        return None
    if task["state"] == "WAITING_SOURCE" and lineage and all(
        row["canonical_status"] == "INVALIDATED" for row in lineage
    ):
        return "SOURCE_INVALIDATED"
    return task["last_error_code"] or PENDING_REASONS.get(task["state"], "UNEXPLAINED_STATE")


def _check(mismatches, condition, code, detail):
    if not condition:
        mismatches.append({"code": code, "detail": detail})


def _registry_count(target):
    state = target.state
    registry = target.chain.client("Read local registry for reconciliation", "object", state["registry_id"])
    fields = registry["content"]["fields"]
    count = fields["issuance_count"]
    if (registry["objectId"] != state["registry_id"] or fields["id"]["id"] != state["registry_id"]
        or registry["type"] != f"{state['package_id']}::entitlement::Registry" or "Shared" not in registry["owner"]
        or type(count) not in (int, str) or not re.fullmatch(r"[0-9]+", str(count))):
        raise RuntimeError("The local node returned an invalid registry object")
    return int(count), registry


def _lookup(target, intent, effect, mismatches):
    digest = intent["transaction_digest"]
    proof = {"requested_digest": digest, "attempt_number": intent["attempt_number"]}
    try:
        response = target.lookup(digest)
        if "error" in response:
            proof["rpc_error"] = response["error"]
            raise RuntimeError("The original transaction is unavailable")
        transaction = response["result"]
        effects = transaction["effects"]
        proof.update({key: transaction[key] for key in ("digest", "checkpoint", "effects", "events", "objectChanges", "errors")
                      if key in transaction})
        if transaction["digest"] != digest or effects["transactionDigest"] != digest or transaction.get("errors"):
            raise RuntimeError("The node did not establish the matching transaction digest and effects")
        if effect is None:
            raise RuntimeError("The node returned an outcome without a matching PostgreSQL effect")
        status = effect["effect_status"].lower()
        if effects["status"]["status"] != status or effects != effect["confirmed_effect"].get("effects"):
            raise RuntimeError("The node effects differ from the recorded outcome")
        if status == "failure" and (effects.get("created") or transaction.get("events") != []
                                    or any(change["type"] == "created" for change in transaction["objectChanges"])
                                    or effect["confirmed_effect"].get("checkpoint") != transaction.get("checkpoint")):
            raise RuntimeError("The recorded failure lacks matching finality or has object creation or emitted events")
    except READ_ERRORS as error:
        mismatches.append({"code": "TARGET_LOOKUP_MISMATCH", "detail": f"{digest}: {error}"})
        return proof, None
    return proof, transaction


def _check_receipt(task, saved, digest, transaction, state, mismatches):
    if saved is None:
        mismatches.append({"code": "MISSING_RECEIPT", "detail": "The successful task lacks its persisted receipt"})
        return
    receipt = saved["confirmed_receipt"]
    evidence = receipt.get("evidence", [])
    source_refs = [record for record in evidence if record.get("evidence_role") == "SOURCE"]
    target_refs = [record for record in evidence if record.get("evidence_role") == "TARGET"]
    _check(mismatches, saved["task_id"] == receipt.get("task_id") == task["task_id"]
           and saved["route_revision_snapshot"] == receipt.get("route_revision_snapshot") == task["route_revision_snapshot"]
           and saved["business_payload_digest"] == receipt.get("business_payload_digest") == task["business_payload_digest"]
           and receipt.get("state") == "SUCCEEDED" and receipt.get("evidence_class") == "LOCAL_FIXTURE_SOURCE_REAL_TARGET",
           "RECEIPT_TASK_MISMATCH", "The receipt and its database envelope do not match the immutable task")
    _check(mismatches, len(source_refs) == 1 and source_refs[0]["label"] == "LOCAL_FIXTURE"
           and source_refs[0]["is_simulated"] is True and source_refs[0]["network_identity"] == task["source_genesis_hash"]
           and source_refs[0]["reference"]["source_event_id"] == task["business_event_id"],
           "RECEIPT_SOURCE_MISMATCH", "The receipt does not preserve the original fixture source and event")
    _check(mismatches, len(target_refs) == 1 and target_refs[0]["network_identity"] == state["target_genesis_digest"]
           and target_refs[0]["label"] == "TARGET_CONFIRMED_EFFECT" and target_refs[0]["is_simulated"] is False
           and all(target_refs[0]["reference"].get(key) == value for key, value in {
               "task_id": task["task_id"], "business_payload_digest": task["business_payload_digest"],
               "target_transaction_digest": digest, "target_package_id": state["package_id"],
               "target_package_digest": state["target_package_digest"], "effect_status": "SUCCESS",
           }.items()), "RECEIPT_TARGET_MISMATCH", "The receipt does not cite the original target transaction")
    if transaction is not None and len(target_refs) == 1:
        checkpoint = target_refs[0]["reference"].get("target_checkpoint")
        _check(mismatches, checkpoint is None or str(checkpoint) == str(transaction.get("checkpoint")),
               "CHECKPOINT_MISMATCH", "The recorded checkpoint differs from original transaction lookup")


def _check_issuance(task, success, transaction, registry, state, mismatches):
    try:
        field = registry["result"]["data"]["content"]["fields"]
        key = list(bytes.fromhex(task["task_id"][2:]))
        binding = field["value"]["fields"]
        entitlement = binding["entitlement"]
        recipient = task["business_payload"]["recipient_mango"]
        _check(mismatches, _hex_id(entitlement) and field["name"] == key and binding == {
            "business_payload_digest": list(bytes.fromhex(task["business_payload_digest"][2:])),
            "entitlement": entitlement, "recipient": recipient,
        } == success["confirmed_effect"].get("registry_task_binding"),
            "REGISTRY_BINDING_MISMATCH", "The on-chain registry and persisted effect do not bind the same task and recipient")
        if transaction is not None:
            issued = [event for event in transaction["events"]
                      if event["type"] == f"{state['package_id']}::entitlement::EntitlementIssued"]
            recorded_event = success["confirmed_effect"].get("event") or {}
            _check(mismatches, len(issued) == 1 and issued[0]["id"]["txDigest"] == transaction["digest"]
                   and issued[0]["sender"] == state["issuer"]
                   and issued[0]["parsedJson"] == {**binding, "registry": state["registry_id"], "task_id": key}
                   and all(issued[0].get(name) == recorded_event.get(name) for name in ("id", "type", "sender", "parsedJson")),
                   "ISSUANCE_EVENT_MISMATCH", "The original transaction did not emit the one recorded bound issuance")
            created = [change for change in transaction["objectChanges"] if change["type"] == "created"
                       and change["objectType"] == f"{state['package_id']}::entitlement::TestEntitlement"]
            _check(mismatches, len(created) == 1 and created[0]["objectId"] == entitlement
                   and created[0]["owner"] == {"AddressOwner": recipient},
                   "ENTITLEMENT_OWNER_MISMATCH", "The original transaction did not create the bound recipient's entitlement")
    except (KeyError, TypeError, ValueError) as error:
        mismatches.append({"code": "ISSUANCE_EVIDENCE_INCOMPLETE", "detail": f"Missing or malformed issuance evidence: {error}"})
        return None
    return entitlement


def _check_task(record, target):
    task, intents, effects = (record[key] for key in ("task", "intents", "effects"))
    task_id, payload, route = (task[key] for key in ("task_id", "business_payload", "route_snapshot"))
    state = target.state
    mismatches = []
    _check(mismatches, task_id == _digest(("mango-business-task-v1", task["source_genesis_hash"], task["source_contract"],
                                          task["business_event_id"], task["stable_route_id"]))
           and task["business_payload_digest"] == _digest(("mango-entitlement-payload-v1", payload["action"],
               payload["campaign_id"], payload["entitlement_code"], payload["recipient_mango"]))
           and _hex_id(payload["recipient_mango"]) and route.get("recipient", payload["recipient_mango"]) == payload["recipient_mango"],
           "TASK_BINDING_MISMATCH", "The stored task identity, payload or recipient does not match its evidence")
    _check(mismatches, [intent["attempt_number"] for intent in intents] == list(range(1, task["attempt_count"] + 1)),
           "ATTEMPT_COUNT_MISMATCH", "The task attempt count and stored intents are inconsistent")
    by_attempt = {effect["attempt_number"]: effect for effect in effects}
    _check(mismatches, set(by_attempt).issubset(intent["attempt_number"] for intent in intents),
           "ORPHAN_EFFECT", "A recorded effect has no matching intent")
    proofs, transactions = [], {}
    for intent in intents:
        number, digest = intent["attempt_number"], intent["transaction_digest"]
        effect = by_attempt.get(number)
        _check(mismatches, intent["bytes_hash_matches"] and intent["route_snapshot"] == route
               and intent["target_genesis_digest"] == state["target_genesis_digest"]
               and intent["target_package_id"] == state["package_id"],
               "INTENT_BINDING_MISMATCH", f"Attempt {number} differs from its pinned route or signed-byte hash")
        _check(mismatches, effect is not None and effect["transaction_digest"] == digest
               and intent["outcome"] == "CONFIRMED_" + effect["effect_status"],
               "INTENT_OUTCOME_MISMATCH", f"Attempt {number} has no matching confirmed effect")
        proof, transaction = _lookup(target, intent, effect, mismatches)
        proofs.append(proof)
        transactions[number] = transaction

    try:
        registry = target.registry_binding(task_id)
    except READ_ERRORS as error:
        registry = None
        mismatches.append({"code": "REGISTRY_LOOKUP_FAILED", "detail": str(error)})
    successes = [effect for effect in effects if effect["effect_status"] == "SUCCESS"]
    succeeded = task["state"] == "SUCCEEDED"
    _check(mismatches, len(successes) == int(succeeded), "SUCCESS_EFFECT_COUNT_MISMATCH",
           "A successful task needs one recorded successful effect and other tasks need zero")
    entitlement, digest = None, None
    if succeeded and len(successes) == 1:
        success = successes[0]
        digest = success["transaction_digest"]
        transaction = transactions.get(success["attempt_number"])
        _check_receipt(task, record["receipt"], digest, transaction, state, mismatches)
        if registry is not None:
            entitlement = _check_issuance(task, success, transaction, registry, state, mismatches)
    elif not succeeded:
        _check(mismatches, record["receipt"] is None, "UNEXPECTED_RECEIPT", "A non-successful task has a success receipt")
        absence = registry.get("result") if isinstance(registry, dict) and "error" not in registry else None
        _check(mismatches, isinstance(absence, dict) and not absence.get("data") and absence.get("error") == {
            "code": "dynamicFieldNotFound", "parent_object_id": state["registry_id"],
        }, "REGISTRY_ABSENCE_UNVERIFIED", "The target did not prove absence of an issuance for the task")

    return {"task_id": task_id, "state": task["state"], "reason": _reason(task, record["source_lineage"]),
            "digest": digest, "entitlement_id": entitlement, "attempt_count": task["attempt_count"],
            "database": record, "registry_binding": registry, "target_lookups": proofs, "mismatches": mismatches}


def reconcile_local(db, target, source_genesis, expected_task_ids):
    """Read PostgreSQL and Mango without claiming tasks, writing state or submitting transactions."""
    expected = tuple(expected_task_ids)
    state = target.state
    target_genesis, package_id = state["target_genesis_digest"], state["package_id"]
    rpc = urlsplit(state["isolation"]["rpc"])
    if (not all(_hex_id(value) for value in (source_genesis, target_genesis, package_id, *expected))
        or source_genesis == target_genesis or len(set(expected)) != len(expected)
        or state["isolation"]["network"] != "none" or rpc.scheme != "http" or rpc.hostname != "127.0.0.1"):
        raise ValueError("Invalid task range or isolated Mango target")

    snapshot = _snapshot(db, source_genesis, target_genesis, package_id)
    scoped, excluded = [], []
    for record in snapshot["tasks"]:
        route = record["task"]["route_snapshot"]
        if (route.get("target_genesis_digest"), route.get("target_package_id")) == (target_genesis, package_id):
            scoped.append(record)
        else:
            excluded.append(record["task"]["task_id"])

    actual = {record["task"]["task_id"] for record in scoped}
    missing = sorted(set(expected) - actual)
    unexpected = sorted(actual - set(expected))
    mismatches = [{"task_id": task_id, "code": "EXPECTED_TASK_MISSING", "detail": "Task absent from the scoped database"}
                  for task_id in missing]
    mismatches.extend({"task_id": task_id, "code": "UNEXPECTED_TASK", "detail": "Database task outside the fixed input set"}
                      for task_id in unexpected)
    checks = []
    for record in scoped:
        try:
            checks.append(_check_task(record, target))
        except (KeyError, TypeError, ValueError, AttributeError) as error:
            task = record["task"]
            checks.append({"task_id": task["task_id"], "state": task["state"],
                           "reason": "EVIDENCE_INCOMPLETE", "entitlement_id": None, "database": record,
                           "mismatches": [{"code": "TASK_EVIDENCE_INCOMPLETE", "detail": str(error)}]})
    mismatches.extend({"task_id": check["task_id"], **entry} for check in checks for entry in check["mismatches"])
    entitlements = [check["entitlement_id"] for check in checks if check["entitlement_id"] is not None]
    if len(entitlements) != len(set(entitlements)):
        mismatches.append({"task_id": None, "code": "DUPLICATE_ENTITLEMENT",
                           "detail": "Several database tasks refer to the same target entitlement"})
    successful = sum(check["state"] == "SUCCEEDED" for check in checks)
    other_source_count = snapshot["other_source_registry_success_count"]
    registry_count, registry = None, None
    try:
        registry_count, registry = _registry_count(target)
    except READ_ERRORS as error:
        mismatches.append({"task_id": None, "code": "REGISTRY_COUNT_UNAVAILABLE", "detail": str(error)})
    if registry_count is not None and registry_count != successful + other_source_count:
        mismatches.append({"task_id": None, "code": "REGISTRY_COUNT_MISMATCH",
                           "detail": f"Registry reports {registry_count}, expected {successful + other_source_count}"})
    return {"outcome": "FAIL" if mismatches else "PASS", "source_genesis": source_genesis,
            "target_genesis": target_genesis, "target_package_id": package_id, "registry_id": state["registry_id"],
            "task_count": len(checks), "source_task_count": len(snapshot["tasks"]),
            "expected_task_count": len(expected), "success_count": successful,
            "pending_count": len(checks) - successful, "registry_count": registry_count, "registry": registry,
            "intent_count": sum(len(record["intents"]) for record in scoped),
            "effect_count": sum(len(record["effects"]) for record in scoped),
            "receipt_count": sum(record["receipt"] is not None for record in scoped),
            "other_source_registry_success_count": other_source_count, "excluded_task_ids": excluded,
            "missing_task_ids": missing, "unexpected_task_ids": unexpected, "mismatch_count": len(mismatches),
            "mismatches": mismatches, "tasks": checks}
