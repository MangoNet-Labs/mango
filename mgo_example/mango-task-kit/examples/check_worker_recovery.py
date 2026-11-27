#!/usr/bin/env python3
"""Check fenced worker recovery with disposable PostgreSQL and a synthetic Mango target."""

import argparse
import hashlib
import json
import sys
from pathlib import Path

from smoke import DOMAIN, database, require

sys.path.insert(0, str(Path(__file__).resolve().parent.parent))
from target_adapter.local_workflow import process_local_task


SIGNED = b'{"synthetic":"original signed bytes"}'
DIGEST = "F" * 43
PAYLOAD_DIGEST = "0x" + "07" * 32
PAYLOAD = {"action": "issue_test_entitlement", "recipient_mango": DOMAIN["recipient"]}
ENTITLEMENT = "0x" + "08" * 32


def sql_text(value):
    return "'" + value.replace("'", "''") + "'"


def sql_json(value):
    return sql_text(json.dumps(value)) + "::jsonb"


class WorkerDatabase:
    def __init__(self, sql):
        self.sql = sql

    def query(self, statement):
        return self.sql("SET ROLE mango_taskkit_worker;\n" + statement)

    def row(self, function, args):
        result = self.query(f"SELECT to_jsonb(row) FROM {function}({','.join(args)}) AS row;")
        require(bool(result), f"The fenced operation {function} returned no row")
        return json.loads(result)

    def transition(self, row, state, error=None):
        return self.row("advance_task", [
            sql_text(row["task_id"]), sql_text(row["lease_owner"]),
            str(row["lease_fencing_token"]), str(row["task_revision"]),
            sql_text(state), "NULL", sql_text(error) if error is not None else "NULL",
        ])


class SyntheticTarget:
    def __init__(self, sql, task_id, *, allow_send=False):
        self.sql = sql
        self.task_id = task_id
        self.allow_send = allow_send
        self.send_count = 0
        self.lookups = []
        self.state = {
            "isolation": {"network": "none", "rpc": "http://127.0.0.1:1"},
            "target_genesis_digest": DOMAIN["target_genesis_digest"],
            "package_id": DOMAIN["target_package_id"],
            "registry_id": "synthetic-registry",
        }
        self.chain = self

    def prepare_issue(self, *_args):
        raise AssertionError("A restarted worker prepared a new transaction")

    def send_once(self, signed):
        require(self.allow_send and self.send_count == 0, "An uncertain attempt was sent again")
        require(signed == SIGNED, "Recovery must send the original signed bytes")
        intent = read_attempt(self.sql, self.task_id)
        require(intent["outcome"] == "BROADCAST_UNKNOWN", "The dispatch marker must precede the send")
        require(intent["digest"] == DIGEST, "The persisted transaction digest changed")
        self.send_count += 1
        return {"result": {"digest": DIGEST}}

    def lookup(self, digest):
        require(digest == DIGEST, "Recovery looked up a different transaction")
        self.lookups.append(digest)
        if self.send_count == 0:
            return {"error": {"code": -32602, "message":
                              f"Could not find the referenced transaction [TransactionDigest({digest})]."}}
        return {"result": {"digest": digest, "effects": {
            "transactionDigest": digest, "status": {"status": "success"},
        }}}

    def registry_binding(self, task_id):
        require(task_id == self.task_id, "Expected the original task in the registry lookup")
        return {"result": {"data": {"content": {"fields": {
            "name": list(bytes.fromhex(task_id[2:])),
            "value": {"fields": {
                "business_payload_digest": list(bytes.fromhex(PAYLOAD_DIGEST[2:])),
                "entitlement": ENTITLEMENT,
                "recipient": DOMAIN["recipient"],
            }},
        }}}}}

    def client(self, _reason, command, registry_id):
        require(command == "object" and registry_id == self.state["registry_id"],
                "Expected a synthetic registry object read")
        return {"content": {"fields": {"issuance_count": self.send_count}}}


def seed_attempt(sql, db, number, *, legacy=False, marked=False):
    task_id = "0x" + f"{number:02x}" * 32
    source = {
        "is_simulated": True,
        "evidence_label": "LOCAL_FIXTURE",
        "source_genesis_hash": DOMAIN["source_genesis_hash"],
        "source_contract": DOMAIN["source_contract"],
        "business_event_id": "0x" + f"{number + 16:02x}" * 32,
        "emitted_payload_digest": PAYLOAD_DIGEST,
        "source_transaction_hash": "0x" + f"{number + 32:02x}" * 32,
        "source_block_hash": "0x" + "04" * 32,
        "source_height": 7,
        "source_log_index": 0,
    }
    record = {
        "task_id": task_id,
        **{key: source[key] for key in ("source_genesis_hash", "source_contract", "business_event_id")},
        "stable_route_id": "demo-entitlement",
        "route_revision_snapshot": 1,
        "route_snapshot": DOMAIN,
        "policy_snapshot": {"mode": "FIXTURE_ONLY_NO_SIGNATURE", "policy_version": 1},
        "business_payload": PAYLOAD,
        "business_payload_digest": PAYLOAD_DIGEST,
        "state": "READY",
    }
    require(sql(f"SET ROLE mango_taskkit_api; SELECT admit_task({sql_json(record)})->>'disposition';") == "CREATED",
            "Could not admit the synthetic recovery task")
    task = sql_text(task_id)
    sql(f"""
        INSERT INTO source_observations
            (task_id, tx_hash, block_hash, source_height, log_index, canonical_status, evidence, observed_at)
        VALUES ({task}, {sql_text(source['source_transaction_hash'])}, {sql_text(source['source_block_hash'])},
                7, 0, 'CANONICAL', {sql_json(source)}, clock_timestamp());
    """)
    function = "record_submission_intent" if legacy else "record_recoverable_submission_intent"
    mark = ""
    if marked:
        mark = f"""
            IF begin_submission_dispatch(worker.task_id, worker.lease_owner, worker.lease_fencing_token,
                                         worker.task_revision, {sql_text(DIGEST)})->>'digest'
                IS DISTINCT FROM {sql_text(DIGEST)} THEN
                RAISE EXCEPTION 'could not persist the synthetic dispatch marker';
            END IF;
        """
    db.query(f"""
        DO $recovery$
        DECLARE worker tasks;
        BEGIN
            SELECT * INTO STRICT worker FROM claim_task({task}, 'crashed-worker', 1);
            SELECT * INTO STRICT worker FROM {function}(
                worker.task_id, worker.lease_owner, worker.lease_fencing_token, worker.task_revision,
                {sql_text(DOMAIN['target_genesis_digest'])}, {sql_text(DOMAIN['target_package_id'])},
                {sql_text(DIGEST)}, decode('{SIGNED.hex()}', 'hex'), {sql_text(hashlib.sha256(SIGNED).hexdigest())}
            );
            {mark}
        END;
        $recovery$;
    """)
    return task_id, source


def read_attempt(sql, task_id):
    task = sql_text(task_id)
    return json.loads(sql(f"""
        SELECT jsonb_build_object(
            'digest', i.transaction_digest, 'signed_hex', encode(i.signed_transaction_bytes, 'hex'),
            'outcome', i.outcome, 'attempt_count', t.attempt_count, 'fence', t.lease_fencing_token,
            'receipt_count', (SELECT count(*) FROM receipts WHERE task_id = t.task_id))
        FROM tasks t JOIN submission_intents i ON t.task_id = i.task_id
        WHERE t.task_id = {task};
    """))


def process(db, target, task_id, source):
    binding = {
        "authorized": True, "task_id": task_id, "business_payload_digest": PAYLOAD_DIGEST,
        **{key: DOMAIN[key] for key in ("target_genesis_digest", "target_package_id", "recipient")},
    }

    def verify_issue(transaction, _state, _package, _binding):
        return ENTITLEMENT, {"id": {"txDigest": transaction["digest"]}}

    def build_receipt(_binding, _source, _domain, transaction, _state):
        return {
            "evidence_class": "LOCAL_FIXTURE_SOURCE_REAL_TARGET", "task_id": task_id,
            "business_payload_digest": PAYLOAD_DIGEST, "route_revision_snapshot": 1, "state": "SUCCEEDED",
            "evidence": [{"evidence_role": "TARGET", "reference": {
                "target_transaction_digest": transaction["digest"],
            }}],
        }

    return process_local_task(db, target, binding, source, PAYLOAD, verify_issue=verify_issue,
                              build_receipt=build_receipt, gas_budget=1)


def check_recovery(sql):
    db = WorkerDatabase(sql)
    ready_task, ready_source = seed_attempt(sql, db, 1)
    marked_task, marked_source = seed_attempt(sql, db, 2, marked=True)
    legacy_task, legacy_source = seed_attempt(sql, db, 3, legacy=True)
    sql("SELECT pg_sleep(1.1);")

    target = SyntheticTarget(sql, ready_task, allow_send=True)
    result = process(db, target, ready_task, ready_source)
    require(result["digest"] == DIGEST and result["send_requests"] == 1, "The restart did not send the persisted attempt")
    saved = read_attempt(sql, ready_task)
    require(saved == {"digest": DIGEST, "signed_hex": SIGNED.hex(), "outcome": "CONFIRMED_SUCCESS",
                      "attempt_count": 1, "fence": 2, "receipt_count": 1},
            "The recovered worker did not preserve the original attempt and receipt")
    replay = process(db, target, ready_task, ready_source)
    require(replay["replayed"] and replay["send_requests"] == 0 and target.lookups == [DIGEST],
            "Replay after completion should return the stored receipt without sending")

    for task_id, source in ((marked_task, marked_source), (legacy_task, legacy_source)):
        target = SyntheticTarget(sql, task_id)
        try:
            process(db, target, task_id, source)
        except RuntimeError as error:
            require(str(error) == "The original transaction digest was not confirmed by local Mango",
                    f"The lookup-only path failed unexpectedly: {error}")
        else:
            raise AssertionError("An unconfirmed attempt unexpectedly completed")
        attempt = read_attempt(sql, task_id)
        require(target.send_count == 0 and target.lookups and attempt["attempt_count"] == 1
                and attempt["signed_hex"] == SIGNED.hex() and attempt["receipt_count"] == 0
                and attempt["outcome"] == "BROADCAST_UNKNOWN", "An ambiguous attempt must reconcile without sending")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--postgres-image", default="postgres:16-alpine")
    args = parser.parse_args()
    with database(args.postgres_image) as (sql, _connection, version):
        check_recovery(sql)
    print(f"Worker recovery checks passed on PostgreSQL {version}: one exact send, two lookup-only cases.")


if __name__ == "__main__":
    main()
