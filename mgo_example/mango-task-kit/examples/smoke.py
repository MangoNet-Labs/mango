#!/usr/bin/env python3
"""Run the Task Kit API and a synthetic task against disposable PostgreSQL."""

import argparse
from contextlib import contextmanager
import http.client
import json
import os
from pathlib import Path
import re
import secrets
import subprocess
import sys
import tempfile
import time


PACKAGE = Path(__file__).resolve().parent.parent
TASK_ID = "0x" + "a1" * 32
TARGET_DIGEST = "smoke-target-digest"
DOMAIN = {
    "source_genesis_hash": "0x" + "01" * 32,
    "source_contract": "0x" + "02" * 20,
    "target_genesis_digest": "0x" + "03" * 32,
    "target_package_id": "0x" + "04" * 32,
    "recipient": "0x" + "05" * 32,
    "route_revision": 1,
}


def require(condition, message):
    if not condition:
        raise RuntimeError(message)


def docker(*args, input_text=None, env=None, timeout=30):
    result = subprocess.run(
        ["docker", *args], input=input_text, text=True, capture_output=True,
        env=env, timeout=timeout,
    )
    require(result.returncode == 0, f"docker {args[0]} exited with status {result.returncode}")
    return result.stdout.strip()


def wait_for(check, message, timeout=40):
    deadline = time.monotonic() + timeout
    while time.monotonic() < deadline:
        result = check()
        if result:
            return result
        time.sleep(0.25)
    raise RuntimeError(message)


@contextmanager
def database(image):
    name = "mango-task-kit-smoke-" + secrets.token_hex(6)
    env = {**os.environ, "POSTGRES_PASSWORD": secrets.token_hex(24)}
    try:
        docker("run", "--detach", "--rm", "--name", name, "--env", "POSTGRES_PASSWORD",
               "--publish", "127.0.0.1::5432", "--tmpfs", "/var/lib/postgresql/data",
               image, env=env, timeout=180)

        def ready():
            return subprocess.run(
                ["docker", "exec", name, "pg_isready", "-h", "127.0.0.1", "-U", "postgres"],
                stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, timeout=5,
            ).returncode == 0

        wait_for(ready, "PostgreSQL did not become ready")
        binding = docker("port", name, "5432/tcp")
        mapped = re.fullmatch(r"127\.0\.0\.1:(\d+)", binding)
        require(mapped is not None, "Expected one IPv4 loopback PostgreSQL port")

        def sql(statement):
            return docker(
                "exec", "-i", name, "psql", "-X", "-U", "postgres", "-d", "postgres",
                "-A", "-t", "-q", "--set", "ON_ERROR_STOP=1", input_text=statement,
            )

        version = sql("SHOW server_version;")
        require(version.startswith("16."), "The smoke check requires PostgreSQL 16")
        for migration in sorted((PACKAGE / "migrations").glob("*.sql")):
            sql(migration.read_text())
        api_password = secrets.token_hex(24)
        sql(f"CREATE ROLE taskkit_smoke_api LOGIN PASSWORD '{api_password}';\n"
            "GRANT mango_taskkit_api TO taskkit_smoke_api;")
        yield sql, f"postgresql://taskkit_smoke_api:{api_password}@{binding}/postgres", version
    finally:
        subprocess.run(
            ["docker", "rm", "-f", name], check=False, capture_output=True, timeout=25,
        )


def seed_task(sql):
    candidate = {
        "task_id": TASK_ID,
        "source_genesis_hash": DOMAIN["source_genesis_hash"],
        "source_contract": DOMAIN["source_contract"],
        "business_event_id": "0x" + "06" * 32,
        "stable_route_id": "demo-entitlement",
        "route_revision_snapshot": DOMAIN["route_revision"],
        "route_snapshot": DOMAIN,
        "policy_snapshot": {"mode": "FIXTURE_ONLY_NO_SIGNATURE"},
        "business_payload": {"entitlement_code": "smoke"},
        "business_payload_digest": "0x" + "07" * 32,
        "state": "READY",
    }
    record = json.dumps(candidate)
    statement = f"SELECT admit_task('{record}'::jsonb)->>'disposition';"
    require(sql(statement) == "CREATED", "Expected one synthetic task")
    require(sql(statement) == "REPLAY", "Expected replay of the same synthetic task")
    sql(f"""
        DO $smoke$
        DECLARE
            task tasks;
            fixture_bytes bytea := convert_to('synthetic-smoke', 'UTF8');
        BEGIN
            SELECT * INTO STRICT task FROM claim_task('{TASK_ID}', 'smoke-worker', 30);
            SELECT * INTO STRICT task FROM record_submission_intent(
                task.task_id, 'smoke-worker', task.lease_fencing_token, task.task_revision,
                '{DOMAIN["target_genesis_digest"]}', '{DOMAIN["target_package_id"]}',
                '{TARGET_DIGEST}', fixture_bytes, encode(sha256(fixture_bytes), 'hex')
            );
            PERFORM advance_task(
                task.task_id, 'smoke-worker', task.lease_fencing_token, task.task_revision,
                'TARGET_OUTCOME_UNKNOWN'
            );
        END;
        $smoke$;
    """)


@contextmanager
def service(binary, connection_url, directory, tokens):
    domain_path = directory / "domain.json"
    domain_path.write_text(json.dumps(DOMAIN))
    log_path = directory / "service.log"
    env = {
        **os.environ,
        "TASK_KIT_DATABASE_URL": connection_url,
        "TASK_KIT_DOMAIN": str(domain_path),
        "TASK_KIT_LISTEN": "127.0.0.1:0",
        **{f"TASK_KIT_{role.upper()}_TOKEN": token for role, token in tokens.items()},
    }

    with log_path.open("w") as log:
        with subprocess.Popen([binary], env=env, stdout=log, stderr=subprocess.STDOUT) as process:
            try:
                def listening():
                    require(process.poll() is None, "Task Kit exited before binding its loopback port")
                    match = re.search(r"Task Kit ready at 127\.0\.0\.1:(\d+)", log_path.read_text())
                    return int(match.group(1)) if match else None

                yield wait_for(listening, "Task Kit did not bind its loopback port")
            finally:
                if process.poll() is None:
                    process.terminate()
                    try:
                        process.wait(timeout=5)
                    except subprocess.TimeoutExpired:
                        process.kill()
                        process.wait()


def request(port, path, expected, token=None, body=None, *, decode_json=True):
    headers = {"Accept": "application/json"}
    if token:
        headers["Authorization"] = "Bearer " + token
    if body is not None:
        headers["Content-Type"] = "application/json"
    connection = http.client.HTTPConnection("127.0.0.1", port, timeout=5)
    try:
        method = "POST" if body is not None else "GET"
        payload = json.dumps(body).encode() if body is not None else None
        connection.request(method, path, body=payload, headers=headers)
        response = connection.getresponse()
        data = response.read(1024 * 1024)
        require(response.status == expected, f"{method} {path} returned {response.status}, expected {expected}")
        return json.loads(data) if decode_json else data
    finally:
        connection.close()


def smoke(binary, image):
    tokens = {role: secrets.token_hex(24) for role in ("reader", "operator", "ingest")}
    with tempfile.TemporaryDirectory(prefix="mango-task-kit-smoke-") as temporary:
        with database(image) as (sql, connection_url, version):
            seed_task(sql)
            with service(binary, connection_url, Path(temporary), tokens) as port:
                page = request(port, "/", 200, decode_json=False)
                require(b'id="lookup"' in page and re.search(rb'<html\s+lang="en(?:-US)?"', page),
                        "Expected the English Task Kit browser view")
                live = request(port, "/health/live", 200)
                require(live["alive"] is True, "Expected a live Task Kit instance")
                request(port, "/health/ready", 401)
                ready = request(port, "/health/ready", 200, tokens["reader"])
                require(ready["ready"] is True, "Expected a ready task store")
                path = "/v1/tasks/" + TASK_ID
                task = request(port, path, 200, tokens["reader"])
                require(task["task_id"] == TASK_ID and task["attempt_count"] == 1
                        and task["target_transaction_digest"] == TARGET_DIGEST
                        and task["state"] == "TARGET_OUTCOME_UNKNOWN",
                        "The read endpoint did not return the original synthetic intent")
                receipt = request(port, path + "/receipt", 202, tokens["reader"])
                require(receipt["task_id"] == TASK_ID and receipt["status"] == "PENDING",
                        "Expected one pending synthetic receipt")
                action = {"action_id": "smoke-reconcile", "expected_task_revision": task["task_revision"]}
                request(port, path + "/retry", 403, tokens["reader"], action)
                ack = request(port, path + "/retry", 202, tokens["operator"], action)
                replay = request(port, path + "/retry", 200, tokens["operator"], action)
                require(ack == replay and ack["action"] == "RECONCILE_ONLY"
                        and ack["creates_target_attempt_now"] is False,
                        "Replaying reconciliation did not return the original acknowledgement")
                updated = request(port, path, 200, tokens["reader"])
                require(updated["attempt_count"] == 1 and updated["target_transaction_digest"] == TARGET_DIGEST,
                        "Reconciliation replaced or duplicated the original intent")

    return {
        "result": "passed",
        "postgres_version": version,
        "contract_version": live["contract_version"],
        "browser_page": 200,
        "task_replay": "REPLAY",
        "receipt_status": "PENDING",
        "operator_action": ack["action"],
        "target_attempt_count": updated["attempt_count"],
    }


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", type=Path, required=True, help="Built mango-local-task-kit executable")
    parser.add_argument("--postgres-image", default="postgres:16-alpine", help="Docker image for PostgreSQL 16")
    args = parser.parse_args()
    binary = args.binary.resolve()
    if not binary.is_file() or not os.access(binary, os.X_OK):
        parser.error("--binary must point to an executable file")
    try:
        print(json.dumps(smoke(binary, args.postgres_image), indent=2))
    except (OSError, RuntimeError, ValueError, KeyError, subprocess.TimeoutExpired) as error:
        print(f"Task Kit smoke failed: {error}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
