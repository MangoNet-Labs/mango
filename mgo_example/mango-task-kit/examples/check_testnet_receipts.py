#!/usr/bin/env python3
"""Read the published Mango Task Kit testnet evidence from the two public RPCs."""

import json
from urllib.error import URLError
from urllib.request import Request, urlopen


BNB_RPC = "https://bsc-testnet-dataseed.bnbchain.org"
MANGO_RPC = "https://fullnode.testnet2.mangonetwork.io:443/"
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


def main():
    if rpc(BNB_RPC, "eth_chainId", []) != "0x61":
        raise ValueError("The BNB RPC did not identify chain 97")
    if rpc(MANGO_RPC, "mgo_getChainIdentifier", []) != "88b02231":
        raise ValueError("The Mango RPC did not identify Testnet2")
    finalized = int(rpc(BNB_RPC, "eth_getBlockByNumber", ["finalized", False])["number"], 16)
    for source_hash, source_height, target_digest, target_checkpoint in RECEIPTS:
        source = rpc(BNB_RPC, "eth_getTransactionReceipt", [source_hash])
        block = rpc(BNB_RPC, "eth_getBlockByNumber", [hex(source_height), False])
        if (
            source["status"] != "0x1"
            or source["transactionHash"] != source_hash
            or int(source["blockNumber"], 16) != source_height
            or source["blockHash"] != block["hash"]
            or source_height > finalized
        ):
            raise ValueError(f"BNB receipt did not match finalized block {source_height}")
        target = rpc(MANGO_RPC, "mgo_getTransactionBlock", [target_digest, {"showEffects": True}])
        checkpoint = rpc(MANGO_RPC, "mgo_getCheckpoint", [str(target_checkpoint)])
        if (
            target["digest"] != target_digest
            or int(target["checkpoint"]) != target_checkpoint
            or target["effects"]["status"]["status"] != "success"
            or int(checkpoint["sequenceNumber"]) != target_checkpoint
            or target_digest not in checkpoint["transactions"]
        ):
            raise ValueError(f"Mango receipt did not match checkpoint {target_checkpoint}")
        print(f"Confirmed BNB block {source_height} and Mango checkpoint {target_checkpoint}")


if __name__ == "__main__":
    try:
        main()
    except (URLError, OSError, ValueError, KeyError) as error:
        raise SystemExit(f"Receipt check failed: {error}") from error
