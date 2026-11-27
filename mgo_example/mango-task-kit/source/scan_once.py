#!/usr/bin/env python3
"""Scan one bounded batch from the registered local EVM fixture into PostgreSQL."""

import argparse
from datetime import datetime, timezone
import hashlib
import http.client
import ipaddress
import json
import os
from pathlib import Path
import re
import subprocess
import sys
from urllib.parse import urlsplit


EVENT_TOPIC = "0xa0cf5d01b54c6304cfe0e8b455f138f7ce951e59e5357a95915cdc794cc042a2"
ACTION = "issue_test_entitlement"
ROUTE = "demo-entitlement"
HEX32 = r"0x[0-9a-f]{64}"
HEX20 = r"0x[0-9a-f]{40}"
MAX_SAFE_INTEGER = 9_007_199_254_740_991
HISTORY_BLOCKS = 64
MAX_EVENTS = 512
MAX_BYTES = 4 * 1024 * 1024


class ScanError(Exception):
    pass


def require(condition, message):
    if not condition:
        raise ScanError(message)


def matches(value, pattern):
    return isinstance(value, str) and re.fullmatch(pattern, value) is not None


def number(value, field, minimum=0, maximum=MAX_SAFE_INTEGER):
    require(type(value) is int and minimum <= value <= maximum, f"Invalid {field}")
    return value


def quantity(value, field, maximum=MAX_SAFE_INTEGER):
    require(matches(value, r"0x(?:0|[1-9a-f][0-9a-f]{0,13})"), f"Invalid RPC {field}")
    return number(int(value, 16), field, maximum=maximum)


def canonical_digest(fields):
    hasher = hashlib.sha256()
    for field in fields:
        value = field.encode("utf-8")
        hasher.update(len(value).to_bytes(4, "big"))
        hasher.update(value)
    return "0x" + hasher.hexdigest()


class Fixture:
    def __init__(self, document):
        for key in ("source_genesis_hash", "target_genesis_digest", "target_package_id", "recipient"):
            require(matches(document[key], HEX32), f"Invalid manifest {key}")
        require(matches(document["source_contract"], HEX20), "Invalid manifest contract")
        require(document["source_genesis_hash"] != document["target_genesis_digest"], "Source and target domains coincide")
        self.genesis = document["source_genesis_hash"]
        self.contract = document["source_contract"]
        self.chain_id = number(document["chain_id"], "chain_id", minimum=1)
        self.start = number(document["start_block"], "start_block", minimum=1)
        self.depth = number(document["confirmation_depth"], "confirmation_depth", minimum=1, maximum=32)
        self.revision = number(document["route_revision"], "route_revision", minimum=1)
        self.route = {
            "target_genesis_digest": document["target_genesis_digest"],
            "target_package_id": document["target_package_id"],
            "action": ACTION,
        }
        self.payload = document["payload"]
        require(isinstance(self.payload, dict) and set(self.payload) == {
            "action", "recipient_mango", "campaign_id", "entitlement_code"
        }, "Invalid manifest payload fields")
        require(self.payload["action"] == ACTION and self.payload["recipient_mango"] == document["recipient"],
                "Payload action or recipient differs from the registered fixture")
        for key in ("campaign_id", "entitlement_code"):
            require(matches(self.payload[key], r"[!-~]{1,64}"), f"Invalid payload {key}")
        self.payload_digest = canonical_digest([
            "mango-entitlement-payload-v1", ACTION, self.payload["campaign_id"],
            self.payload["entitlement_code"], self.payload["recipient_mango"],
        ])
        require(document.get("event_topic", EVENT_TOPIC) == EVENT_TOPIC, "Unregistered event topic")
        self.runtime_hash = document.get("contract_code_sha256")
        require(self.runtime_hash is None or matches(self.runtime_hash, r"[0-9a-f]{64}"), "Invalid contract code SHA256")
        self.boundary = {
            "chain_id": self.chain_id, "genesis": self.genesis, "contract": self.contract, "event_topic": EVENT_TOPIC,
            "start_block": self.start, "confirmation_depth": self.depth, "payload_digest": self.payload_digest,
            "contract_code_sha256": self.runtime_hash, "route": self.route,
        }

    def source(self, observation):
        return {
            "source_genesis_hash": self.genesis,
            "source_contract": self.contract,
            "business_event_id": observation["event"],
            "source_transaction_hash": observation["tx"],
            "source_block_hash": observation["block"],
            "source_height": observation["height"],
            "source_log_index": observation["index"],
            "observed_at_unix_ms": observation["time"] * 1000,
            "is_simulated": True,
            "evidence_label": "LOCAL_FIXTURE",
            "emitted_payload_digest": self.payload_digest,
        }

    def task_id(self, observation):
        return canonical_digest([
            "mango-business-task-v1", self.genesis, self.contract, observation["event"], ROUTE,
        ])

    def entry(self, observation, status):
        source = self.source(observation)
        return {
            "task": {
                "task_id": self.task_id(observation),
                "source_genesis_hash": self.genesis,
                "source_contract": self.contract,
                "business_event_id": observation["event"],
                "stable_route_id": ROUTE,
                "route_revision_snapshot": self.revision,
                "route_snapshot": self.route,
                "policy_snapshot": {"mode": "FIXTURE_ONLY_NO_SIGNATURE", "policy_version": 1},
                "business_payload": self.payload,
                "business_payload_digest": self.payload_digest,
                "state": "WAITING_SOURCE",
            },
            "observation": {
                "tx_hash": observation["tx"], "block_hash": observation["block"],
                "source_height": observation["height"], "log_index": observation["index"],
                "canonical_status": status, "evidence": source,
                "observed_at": datetime.fromtimestamp(observation["time"], timezone.utc).isoformat(),
            },
        }


class LocalRpc:
    def __init__(self, url):
        parsed = urlsplit(url)
        require(parsed.scheme == "http" and parsed.path in ("", "/") and not parsed.query and not parsed.fragment
                and parsed.username is None and parsed.password is None and parsed.port is not None,
                "EVM RPC must be an explicit loopback HTTP host and port")
        require(parsed.hostname is not None and ipaddress.ip_address(parsed.hostname).is_loopback,
                "EVM RPC host must be a numeric loopback address")
        self.host = parsed.hostname
        self.port = parsed.port
        self.next_id = 0

    def call(self, method, *params):
        self.next_id += 1
        request = json.dumps({"jsonrpc": "2.0", "id": self.next_id, "method": method, "params": params})
        connection = http.client.HTTPConnection(self.host, self.port, timeout=8)
        try:
            connection.request("POST", "/", request, headers={"Content-Type": "application/json"})
            response = connection.getresponse()
            require(response.status == 200, f"EVM RPC {method} HTTP status {response.status}")
            raw = response.read(MAX_BYTES + 1)
        except (OSError, http.client.HTTPException) as error:
            raise ScanError(f"EVM RPC {method} transport failure") from error
        finally:
            connection.close()
        require(len(raw) <= MAX_BYTES, f"EVM RPC {method} response exceeds limit")
        try:
            reply = json.loads(raw)
        except (UnicodeError, ValueError) as error:
            raise ScanError(f"EVM RPC {method} invalid JSON response") from error
        require(isinstance(reply, dict) and reply.get("jsonrpc") == "2.0" and reply.get("id") == self.next_id
                and "result" in reply and "error" not in reply, f"EVM RPC {method} failed")
        return reply["result"]

    def block(self, height):
        block = self.call("eth_getBlockByNumber", "latest" if height is None else hex(height), False)
        require(isinstance(block, dict), "EVM RPC block is unavailable")
        block_height = quantity(block["number"], "block height")
        require(height is None or block_height == height, "EVM RPC returned a different height")
        require(matches(block["hash"], HEX32) and matches(block["parentHash"], HEX32), "Malformed EVM block hash")
        return block_height, block

    def verify(self, fixture):
        require(quantity(self.call("eth_chainId"), "chain ID") == fixture.chain_id, "EVM chain ID differs from manifest")
        _, genesis = self.block(0)
        require(genesis["hash"] == fixture.genesis, "EVM genesis differs from manifest")
        code = self.call("eth_getCode", fixture.contract, hex(fixture.start))
        require(matches(code, r"0x(?:[0-9a-f]{2})+"), "Registered EVM contract has no code at start block")
        if fixture.runtime_hash is not None:
            require(hashlib.sha256(bytes.fromhex(code[2:])).hexdigest() == fixture.runtime_hash,
                    "EVM contract bytecode differs from manifest")

    def events(self, fixture, height, block):
        height_tag = hex(height)
        logs = self.call("eth_getLogs", {
            "address": fixture.contract, "topics": [EVENT_TOPIC], "fromBlock": height_tag, "toBlock": height_tag,
        })
        require(isinstance(logs, list) and len(logs) <= MAX_EVENTS, "Malformed or oversized EVM log list")
        transactions = block["transactions"]
        require(isinstance(transactions, list) and all(matches(tx, HEX32) for tx in transactions),
                "Malformed EVM block transaction list")
        seen = set()
        events = []
        for log in logs:
            position = quantity(log["logIndex"], "log index", maximum=2_147_483_647)
            tx_position = quantity(log["transactionIndex"], "transaction index")
            require(position not in seen and log["removed"] is False
                    and log["blockHash"] == block["hash"] and log["blockNumber"] == height_tag
                    and tx_position < len(transactions) and transactions[tx_position] == log["transactionHash"],
                    "EVM log does not belong to the sampled block and transaction")
            required_topics = log["topics"]
            require(log["address"] == fixture.contract and isinstance(required_topics, list)
                    and len(required_topics) == 2 and required_topics[0] == EVENT_TOPIC
                    and matches(required_topics[1], HEX32) and required_topics[1] != "0x" + "0" * 64,
                    "EVM log does not match the registered event")
            require(log["data"] == fixture.payload_digest, "Event digest differs from registered payload")
            receipt = self.call("eth_getTransactionReceipt", log["transactionHash"])
            require(isinstance(receipt, dict) and receipt["status"] == "0x1"
                    and receipt["blockHash"] == block["hash"] and receipt["blockNumber"] == height_tag
                    and receipt["transactionHash"] == log["transactionHash"]
                    and receipt["transactionIndex"] == log["transactionIndex"],
                    "EVM log has no matching successful receipt")
            fields = ("address", "topics", "data", "blockHash", "blockNumber", "transactionHash", "transactionIndex", "logIndex")
            require(sum(all(candidate.get(field) == log[field] for field in fields)
                        for candidate in receipt["logs"]) == 1, "Receipt did not contain the exact EVM log")
            seen.add(position)
            events.append({
                "event": required_topics[1], "tx": log["transactionHash"], "block": block["hash"], "height": height,
                "index": position, "time": quantity(block["timestamp"], "block timestamp", maximum=253_402_300_799),
            })
        return events


def sql_text(value):
    return f"convert_from(decode('{value.encode('utf-8').hex()}', 'hex'), 'UTF8')"


def psql(query):
    # Pass only fixed SQL grammar with hex-encoded data and use the local libpq connection environment.
    env = {key: value for key, value in os.environ.items() if not key.startswith("PG") or key in {
        "PGHOST", "PGPORT", "PGDATABASE", "PGUSER", "PGPASSWORD", "PGPASSFILE",
    }}
    host = env.get("PGHOST")
    if host and not host.startswith("/"):
        require(host == "localhost" or ipaddress.ip_address(host).is_loopback, "PostgreSQL must be local")
    for key in ("PGDATABASE", "PGUSER"):
        require(key not in env or matches(env[key], r"[a-zA-Z0-9_.-]+"), f"Invalid PostgreSQL {key}")
    env.update({"PGCLIENTENCODING": "UTF8", "PGCONNECT_TIMEOUT": "5"})
    result = subprocess.run(
        ["psql", "-X", "-qAtw", "--set", "ON_ERROR_STOP=1"],
        input=query, text=True, capture_output=True, env=env, timeout=15,
    )
    require(result.returncode == 0, f"PostgreSQL scanner statement failed (psql exit {result.returncode})")
    require(len(result.stdout) <= MAX_BYTES, "PostgreSQL scanner cursor exceeds limit")
    return result.stdout.strip()


def read_cursor(fixture):
    output = psql(
        "SELECT jsonb_build_object('revision', revision, 'cursor', cursor) FROM scanner_cursors "
        f"WHERE source_namespace = {sql_text(fixture.genesis)};\n"
    )
    return json.loads(output) if output else None


def commit_batch(fixture, revision, cursor, entries):
    serialized_cursor = json.dumps(cursor, separators=(",", ":"))
    serialized_entries = json.dumps(entries, separators=(",", ":"))
    require(len(serialized_cursor) + len(serialized_entries) <= MAX_BYTES, "Scanner batch exceeds SQL input limit")
    output = psql(
        f"SELECT commit_source_batch({sql_text(fixture.genesis)}, {revision}, "
        f"{sql_text(serialized_cursor)}::jsonb, {sql_text(serialized_entries)}::jsonb);\n"
    )
    require(output == str(revision + 1), "Scanner cursor changed before commit")
    return int(output)


def scan(fixture, rpc, max_blocks):
    rpc.verify(fixture)
    head, head_block = rpc.block(None)
    stored = read_cursor(fixture)
    if stored is None:
        revision = 0
        anchor_height = fixture.start - 1
        _, anchor = rpc.block(anchor_height)
        previous = [{"height": anchor_height, "hash": anchor["hash"], "observations": []}]
    else:
        revision = number(stored["revision"], "cursor revision", minimum=1)
        require(stored["cursor"]["fixture"] == fixture.boundary, "Scanner manifest differs from the persisted cursor")
        previous = stored["cursor"]["blocks"]
        require(isinstance(previous, list) and 1 <= len(previous) <= HISTORY_BLOCKS, "Malformed persisted scanner history")

    # Rewind an orphaned suffix and keep its observations as invalidated evidence.
    ancestor = None
    for index in range(len(previous) - 1, -1, -1):
        candidate = previous[index]
        if candidate["height"] <= head:
            _, current = rpc.block(candidate["height"])
            if candidate["hash"] == current["hash"]:
                ancestor = index
                break
    require(ancestor is not None, "EVM reorganization exceeds retained scanner history")
    entries = [
        fixture.entry(observation, "INVALIDATED")
        for block in previous[ancestor + 1:]
        for observation in block["observations"]
    ]
    invalidated = len(entries)
    blocks = previous[:ancestor + 1]
    confirmed_through = head - fixture.depth
    for block in blocks:
        status = "CANONICAL" if block["height"] <= confirmed_through else "UNKNOWN"
        for observation in block["observations"]:
            if observation["status"] != status:
                entries.append(fixture.entry(observation, status))
                observation["status"] = status

    start = blocks[-1]["height"] + 1
    for height in range(start, min(head + 1, start + max_blocks)):
        _, block = rpc.block(height)
        require(block["parentHash"] == blocks[-1]["hash"], "EVM block sequence changed during scan")
        observations = rpc.events(fixture, height, block)
        status = "CANONICAL" if height <= confirmed_through else "UNKNOWN"
        for observation in observations:
            entries.append(fixture.entry(observation, status))
            observation["status"] = status
        require(len(entries) <= MAX_EVENTS, "Scanner batch exceeds event limit")
        blocks.append({"height": height, "hash": block["hash"], "observations": observations})

    _, checked_head = rpc.block(head)
    require(checked_head["hash"] == head_block["hash"], "EVM sampled head changed before cursor commit")
    last = blocks[-1]
    if last["height"] == head:
        checked_last = checked_head
    else:
        _, checked_last = rpc.block(last["height"])
    require(checked_last["hash"] == last["hash"], "EVM scanned range changed before cursor commit")
    require(len(entries) <= MAX_EVENTS, "Scanner batch exceeds event limit")
    blocks = blocks[-HISTORY_BLOCKS:]
    revision = commit_batch(fixture, revision, {"fixture": fixture.boundary, "blocks": blocks}, entries)
    retained = [item for block in blocks for item in block["observations"]]
    return {
        "cursor_revision": revision, "last_scanned_height": blocks[-1]["height"], "sampled_head": head,
        "confirmed_through": max(confirmed_through, 0), "invalidated_observations": invalidated,
        "pending_observations": sum(item["status"] == "UNKNOWN" for item in retained),
        "canonical_observations": [
            {"task_id": fixture.task_id(item), "source": fixture.source(item)}
            for item in retained if item["status"] == "CANONICAL"
        ],
    }


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--manifest", type=Path, required=True)
    parser.add_argument("--rpc-url", required=True)
    parser.add_argument("--max-blocks", type=int, default=24)
    args = parser.parse_args()
    try:
        require(args.manifest.stat().st_size <= 65_536, "Fixture manifest exceeds size limit")
        fixture = Fixture(json.loads(args.manifest.read_text(encoding="utf-8")))
        result = scan(fixture, LocalRpc(args.rpc_url), number(args.max_blocks, "maximum batch blocks", minimum=1, maximum=32))
    except (ScanError, OSError, ValueError, KeyError, TypeError, OverflowError, subprocess.TimeoutExpired) as error:
        print(f"Source scan failed. {error}", file=sys.stderr)
        return 1
    print(json.dumps(result, separators=(",", ":")))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
