"""One-shot Mango adapter for a pinned, disposable loopback-only localnet."""
import base64
import hashlib
import json
import subprocess

OPTIONS = {"showEffects": True, "showEvents": True, "showObjectChanges": True}


class MangoLocal:
    def __init__(self, chain, state, digest_executable):
        self.chain = chain
        self.state = state
        self.digest_executable = digest_executable
        self.send_count = 0
        self.rpc_observer = None
        self.chain.verify_isolation(state["session"])

    def rpc(self, method, params):
        request = json.dumps({"jsonrpc":"2.0","id":1,"method":method,"params":params}, separators=(",",":"))
        result = self.chain.exec("Run fixed local Mango RPC", "curl", "--silent", "--show-error", "--fail-with-body",
            "--max-time", "35", "-H", "content-type: application/json",
            "--data-binary", request, self.state["isolation"]["rpc"], timeout=40, check=False)
        if self.rpc_observer is not None:
            lookup_digest = params[0] if method == "mgo_getTransactionBlock" else None
            self.rpc_observer(method, lookup_digest, result)
        if result is None:
            raise RuntimeError(f"Local Mango RPC timed out for {method}")
        if result.returncode != 0:
            raise RuntimeError(f"Local Mango RPC {method} returned transport exit {result.returncode}")
        response = json.loads(result.stdout)
        if response.get("id") != 1 or ("result" in response) == ("error" in response):
            raise RuntimeError("Invalid local RPC outcome")
        return response

    def prepare_issue(self, binding, gas_budget):
        state = self.state
        if (binding["target_package_id"] != state["package_id"]
            or binding["target_genesis_digest"] != state["target_genesis_digest"]
            or binding.get("authorized") is not True):
            raise RuntimeError("Fixture binding does not match local Mango domain")
        issue = self.chain.exec("Prepare unsigned fixed entitlement transaction", "/s3-bin/mgo", "client",
            "--client.config", "/tmp/mango-s3/config/client.yaml", "call", "--package", state["package_id"],
            "--module", "entitlement", "--function", "issue", "--gas-budget", gas_budget,
            "--args", state["registry_id"], state["issuer_cap_id"], binding["task_id"],
            binding["business_payload_digest"], binding["recipient"], "--serialize-unsigned-transaction", timeout=45)
        tx_bytes = issue.stdout.strip()
        if not base64.b64decode(tx_bytes, validate=True):
            raise RuntimeError("Unsigned Mango transaction did not contain BCS data")
        digest = subprocess.run([str(self.digest_executable)], input=tx_bytes, text=True,
                                capture_output=True, timeout=10, check=True).stdout.strip()
        signed = self.chain.exec("Sign only the fixed local fixture transaction", "/s3-bin/mgo", "keytool", "--json",
            "sign", "--address", state["issuer"], "--data", tx_bytes, timeout=20)
        response = json.loads(signed.stdout)
        signature = response["mgoSignature"] if "mgoSignature" in response else response["mgo_signature"]
        if not base64.b64decode(signature, validate=True):
            raise RuntimeError("Missing local fixture signature")
        document = {"transaction_data_bcs":tx_bytes,"signatures":[signature]}
        raw = json.dumps(document,sort_keys=True,separators=(",",":")).encode()
        return digest, raw, hashlib.sha256(raw).hexdigest()

    def send_once(self, raw):
        if self.send_count != 0:
            raise RuntimeError("Only one submission is allowed by this local worker")
        self.chain.verify_isolation(self.state["session"])
        signed = json.loads(raw)
        self.send_count += 1
        return self.rpc("mgo_executeTransactionBlock",
            [signed["transaction_data_bcs"], signed["signatures"], {}, "WaitForEffectsCert"])

    def lookup(self, digest):
        return self.rpc("mgo_getTransactionBlock", [digest, OPTIONS])

    def registry_binding(self, task_id):
        return self.rpc("mgox_getDynamicFieldObject", [self.state["registry_id"], {
            "type":"vector<u8>","value":list(bytes.fromhex(task_id[2:])),
        }])
