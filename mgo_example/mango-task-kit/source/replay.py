#!/usr/bin/env python3
"""Replay a bounded live source fixture range into its existing observation store."""

import argparse
import json
from pathlib import Path
import sys

from scan_once import Fixture, LocalRpc, ScanError, commit_batch, read_cursor, require


def replay(fixture, rpc, max_blocks):
    rpc.verify(fixture)
    head, head_block = rpc.block(None)
    require(0 <= head - fixture.start + 1 <= max_blocks, "Fixture block range exceeds the replay bound")
    persisted = read_cursor(fixture)
    require(persisted is not None and persisted["cursor"]["fixture"] == fixture.boundary,
            "Replay requires the existing scanner cursor for this fixture")
    cursor = persisted["cursor"]
    tip = cursor["blocks"][-1]
    require(tip["height"] == head and tip["hash"] == head_block["hash"],
            "Run the scanner through the current source block before replaying")
    _, parent = rpc.block(fixture.start - 1)
    entries = []
    locators = []
    statuses = {"CANONICAL": 0, "UNKNOWN": 0}
    for height in range(fixture.start, head + 1):
        _, block = rpc.block(height)
        require(block["parentHash"] == parent["hash"], "Source changed while replaying a contiguous range")
        status = "CANONICAL" if height <= head - fixture.depth else "UNKNOWN"
        for observation in rpc.events(fixture, height, block):
            entry = fixture.entry(observation, status)
            entries.append(entry)
            statuses[status] += 1
            locators.append({"task_id": entry["task"]["task_id"], "tx": observation["tx"],
                             "block": observation["block"], "status": status})
        require(len(entries) <= 1024, "Live replay produced too many observations")
        parent = block
    final_head, final_block = rpc.block(None)
    require(final_head == head and final_block["hash"] == head_block["hash"],
            "Source tip changed before replay could commit")
    revision = commit_batch(fixture, persisted["revision"], cursor, entries)
    return {"outcome": "PASS", "source_genesis": fixture.genesis, "start_block": fixture.start,
            "end_block": head, "observations": locators, "status_counts": statuses,
            "cursor_revision_before": persisted["revision"], "cursor_revision_after": revision,
            "target_writes": 0}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--manifest", required=True, type=Path)
    parser.add_argument("--rpc-url", required=True)
    parser.add_argument("--max-blocks", type=int, default=32)
    args = parser.parse_args()
    try:
        require(1 <= args.max_blocks <= 64, "Replay block limit must be between 1 and 64")
        require(args.manifest.stat().st_size <= 65_536, "Fixture manifest exceeds size limit")
        fixture = Fixture(json.loads(args.manifest.read_text(encoding="utf-8")))
        result = replay(fixture, LocalRpc(args.rpc_url), args.max_blocks)
    except (ScanError, KeyError, OSError, ValueError) as exc:
        print(f"Source replay failed. {exc}", file=sys.stderr)
        return 1
    print(json.dumps(result, separators=(",", ":")))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
