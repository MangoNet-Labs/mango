#!/usr/bin/env python3
"""Verify the published testnet receipt pairs and their cross-chain event bindings."""

from hashlib import sha256
import json
import re
from urllib.error import URLError
from urllib.request import Request, urlopen


BNB_RPC = "https://bsc-testnet-dataseed.bnbchain.org"
MANGO_RPC = "https://fullnode.testnet2.mangonetwork.io:443/"
BNB_GENESIS = "0x6d3c66c5357ec91d5c43af47e234a939b22557cbb552dc45bebbceeed90fbe34"
BNB_SOURCE_CONTRACT = "0xd4931a1ecd9404377edda9adcc0a2fa0b05c5763"
BNB_REQUEST_TOPIC = "0xa0cf5d01b54c6304cfe0e8b455f138f7ce951e59e5357a95915cdc794cc042a2"
MANGO_PACKAGE = "0x78ce0ce0b057796fab88a43b33c4bd8a93bc6d289724d07c6aa9974fd8ac0e17"
MANGO_REGISTRY = "0x29ab1abb867b54815519f64b456fe5b74e522518e38682bccfec4d9fe02be382"
MANGO_ISSUANCE_TYPE = f"{MANGO_PACKAGE}::entitlement::EntitlementIssued"
ROUTE_ID = "s6-test-entitlement"
HEX32 = re.compile(r"0x[0-9a-f]{64}")
RECEIPTS = (
    (
        "0xe8a5910b2ac74f8226ad25155c6a819271c38b46d98afcdfac2758fc9f98927b",
        144998369,
        "1BUfuaWiuxCeo2kVn7kQsGPF8wCzJuqpQHdmixsb5R6",
        58279421,
    ),
    (
        "0xca5d7fa0dd171c8b25c4f387b1bb9f8a75deb5c133c656b45a887af50c2989e6",
        145000790,
        "4naN4pLYra1ncWz3Xy4DU5We2QuoUUhGivnHyvRaNrAN",
        58279642,
    ),
)


def rpc(endpoint, method, params):
    data = json.dumps({"jsonrpc": "2.0", "id": 1, "method": method, "params": params}).encode()
    request = Request(endpoint, data=data, headers={"Content-Type": "application/json"})
    with urlopen(request, timeout=20) as response:
        document = json.loads(response.read(1_048_576))
    if document.get("id") != 1 or document.get("error") or document.get("result") is None:
        raise ValueError(f"{method} returned no matching result")
    return document["result"]


def source_binding(receipt):
    events = [
        log
        for log in receipt["logs"]
        if log["address"] == BNB_SOURCE_CONTRACT and log["topics"][:1] == [BNB_REQUEST_TOPIC]
    ]
    if len(events) != 1:
        raise ValueError("Expected one entitlement request from the BNB source contract")
    event = events[0]
    if (
        event["removed"] is not False
        or event["transactionHash"] != receipt["transactionHash"]
        or event["blockHash"] != receipt["blockHash"]
        or event["blockNumber"] != receipt["blockNumber"]
        or len(event["topics"]) != 2
    ):
        raise ValueError("The BNB entitlement request did not match its receipt")
    event_id, payload_digest = event["topics"][1], event["data"]
    if HEX32.fullmatch(event_id) is None or HEX32.fullmatch(payload_digest) is None:
        raise ValueError("Expected a bytes32 event ID and payload digest on BNB")
    return event_id, payload_digest


def task_id(event_id):
    fields = ("mango-business-task-v1", BNB_GENESIS, BNB_SOURCE_CONTRACT, event_id, ROUTE_ID)
    digest = sha256()
    for field in fields:
        encoded = field.encode("utf-8")
        digest.update(len(encoded).to_bytes(4, "big"))
        digest.update(encoded)
    return "0x" + digest.hexdigest()


def event_bytes32(value, field):
    if not isinstance(value, list) or len(value) != 32 or any(type(byte) is not int for byte in value):
        raise ValueError(f"Expected a 32-byte Mango event field for {field}")
    return "0x" + bytes(value).hex()


def verify_target_binding(transaction, event_id, payload_digest):
    events = [
        event
        for event in transaction["events"]
        if event["packageId"] == MANGO_PACKAGE
        and event["transactionModule"] == "entitlement"
        and event["type"] == MANGO_ISSUANCE_TYPE
    ]
    if len(events) != 1 or events[0]["id"]["txDigest"] != transaction["digest"]:
        raise ValueError("Expected one issuance from the Mango package in the target receipt")
    fields = events[0]["parsedJson"]
    if fields["registry"] != MANGO_REGISTRY:
        raise ValueError("The issuance did not use the expected Mango registry")
    if event_bytes32(fields["business_payload_digest"], "business_payload_digest") != payload_digest:
        raise ValueError("The BNB request and Mango issuance payload digests differ")
    if event_bytes32(fields["task_id"], "task_id") != task_id(event_id):
        raise ValueError("The Mango issuance task ID does not bind the BNB request event")


def main():
    if rpc(BNB_RPC, "eth_chainId", []) != "0x61":
        raise ValueError("The BNB RPC did not identify chain 97")
    if rpc(BNB_RPC, "eth_getBlockByNumber", ["0x0", False])["hash"] != BNB_GENESIS:
        raise ValueError("The BNB RPC did not return the expected testnet genesis")
    if rpc(MANGO_RPC, "mgo_getChainIdentifier", []) != "88b02231":
        raise ValueError("The Mango RPC did not identify Testnet2")
    finalized = int(rpc(BNB_RPC, "eth_getBlockByNumber", ["finalized", False])["number"], 16)
    for source_hash, source_height, target_digest, target_checkpoint in RECEIPTS:
        source = rpc(BNB_RPC, "eth_getTransactionReceipt", [source_hash])
        block = rpc(BNB_RPC, "eth_getBlockByNumber", [hex(source_height), False])
        if (
            source["status"] != "0x1"
            or source["transactionHash"] != source_hash
            or source["to"] != BNB_SOURCE_CONTRACT
            or int(source["blockNumber"], 16) != source_height
            or source["blockHash"] != block["hash"]
            or source_hash not in block["transactions"]
            or source_height > finalized
        ):
            raise ValueError(f"BNB receipt did not match finalized block {source_height}")
        event_id, payload_digest = source_binding(source)
        target = rpc(
            MANGO_RPC, "mgo_getTransactionBlock", [target_digest, {"showEffects": True, "showEvents": True}]
        )
        checkpoint = rpc(MANGO_RPC, "mgo_getCheckpoint", [str(target_checkpoint)])
        if (
            target["digest"] != target_digest
            or int(target["checkpoint"]) != target_checkpoint
            or target["effects"]["status"]["status"] != "success"
            or int(checkpoint["sequenceNumber"]) != target_checkpoint
            or target_digest not in checkpoint["transactions"]
        ):
            raise ValueError(f"Mango receipt did not match checkpoint {target_checkpoint}")
        verify_target_binding(target, event_id, payload_digest)
        print(f"Verified BNB block {source_height} and Mango checkpoint {target_checkpoint} with matching event bindings")


if __name__ == "__main__":
    try:
        main()
    except (URLError, OSError, ValueError, KeyError, TypeError) as error:
        raise SystemExit(f"Receipt check failed: {error}") from error
