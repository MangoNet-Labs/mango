#!/usr/bin/env python3
"""Read a task and its receipt from the local Task Kit HTTP API."""
import argparse
import http.client
import json
import os
import re
import sys


def request(conn, path, token):
    conn.request("GET", path, headers={"Authorization": "Bearer " + token, "Accept": "application/json"})
    response = conn.getresponse()
    value = json.loads(response.read(1024 * 1024))
    return response.status, value


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("task_id")
    parser.add_argument("--port", type=int, default=18080)
    args = parser.parse_args()
    if not re.fullmatch(r"0x[0-9a-f]{64}", args.task_id) or not 1 <= args.port < 65536:
        parser.error("Expected a lowercase Mango task ID and valid loopback port")
    token = os.environ.get("TASK_KIT_READER_TOKEN")
    if not token:
        parser.error("Set TASK_KIT_READER_TOKEN")
    conn = http.client.HTTPConnection("127.0.0.1", args.port, timeout=5)
    try:
        path = "/v1/tasks/" + args.task_id
        status, task = request(conn,path,token)
        if status != 200:
            print(json.dumps({"http_status":status,"error":task},ensure_ascii=False),file=sys.stderr)
            return 1
        status, receipt = request(conn,path+"/receipt",token)
        print(json.dumps({"task":task,"receipt_http_status":status,"receipt":receipt},ensure_ascii=False))
        return 0 if status in (200,202,409) else 1
    finally:
        conn.close()


if __name__ == "__main__":
    raise SystemExit(main())
