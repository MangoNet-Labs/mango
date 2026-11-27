from dataclasses import replace
import json
import unittest

from cryptography.hazmat.primitives.asymmetric.ed25519 import Ed25519PrivateKey

from witness.auth import (IssueMessage, RouteBinding, SignedIssue, SourceObservation, WitnessEpoch,
                          WitnessPolicy, rotate_after_finalized_block, sign_issue, verify_fresh_issue,
                          verify_stored_issue)


def hex_bytes(byte: str, length: int = 32) -> str:
    return "0x" + byte * length


class WitnessTest(unittest.TestCase):
    def setUp(self) -> None:
        self.original = Ed25519PrivateKey.generate()
        self.replacement = Ed25519PrivateKey.generate()
        self.route = RouteBinding(
            route_id="s6-test-entitlement", route_revision=1, source_chain_id=97,
            source_genesis_hash=hex_bytes("11"), source_contract=hex_bytes("22", 20),
            source_event_topic=hex_bytes("33"), target_genesis_digest="11111111111111111111111111111112",
            target_package_id=hex_bytes("44"), recipient_mango=hex_bytes("55"),
        )
        self.observation = SourceObservation(hex_bytes("66"), hex_bytes("77"), hex_bytes("88"),
                                             18, 0, hex_bytes("99"), 20, hex_bytes("aa"))
        self.message = IssueMessage(self.route, self.observation, 1, 100, 150)
        self.policy = WitnessPolicy(self.route, (WitnessEpoch(1, 10, self.hex_key(self.original)),), 60)

    @staticmethod
    def hex_key(key: Ed25519PrivateKey) -> str:
        return key.public_key().public_bytes_raw().hex()

    def test_canonical_roundtrip_authenticates_same_task(self) -> None:
        signed = sign_issue(self.message, self.original)
        copy = SignedIssue.from_document(json.loads(json.dumps(signed.to_document())))
        self.assertEqual(self.message.canonical_bytes(), copy.message.canonical_bytes())
        self.assertEqual(self.message.task_id, copy.message.task_id)
        self.assertEqual(verify_fresh_issue(self.policy, copy, self.observation, 125), self.message)

    def test_issue_requires_fresh_expiry_and_route_match(self) -> None:
        signed = sign_issue(self.message, self.original)
        for now in (99, 150, 180):
            with self.assertRaisesRegex(ValueError, "time or expiry"):
                verify_fresh_issue(self.policy, signed, self.observation, now)
        different_route = replace(self.route, recipient_mango=hex_bytes("bb"))
        with self.assertRaisesRegex(ValueError, "route"):
            verify_fresh_issue(replace(self.policy, route=different_route), signed, self.observation, 125)

    def test_source_hashes_and_log_index_bound_to_signature(self) -> None:
        signed = sign_issue(self.message, self.original)
        for updates in ({"log_index": 1}, {"finalized_hash": hex_bytes("ba")},
                        {"tx_hash": hex_bytes("bc")}, {"payload_digest": hex_bytes("bd")}):
            adjusted = replace(self.observation, **updates)
            with self.assertRaisesRegex(ValueError, "does not match retained RPC evidence"):
                verify_fresh_issue(self.policy, signed, adjusted, 125)
            forged = replace(signed, message=replace(self.message, observation=adjusted))
            with self.assertRaisesRegex(ValueError, "Invalid witness signature"):
                verify_fresh_issue(self.policy, forged, adjusted, 125)

    def test_unregistered_key_cannot_authorize_task(self) -> None:
        rogue = sign_issue(self.message, self.replacement)
        with self.assertRaisesRegex(ValueError, "Invalid witness signature"):
            verify_fresh_issue(self.policy, rogue, self.observation, 125)

    def test_rotation_preserves_stored_receipt_authority_and_separates_ranges(self) -> None:
        old_signature = sign_issue(self.message, self.original)
        rotated = rotate_after_finalized_block(self.policy, self.hex_key(self.replacement), 20, hex_bytes("aa"))
        self.assertIsNone(self.policy.epochs[0].end_height_exclusive)
        self.assertEqual(rotated.epochs[0].end_height_exclusive, 21)
        self.assertEqual(rotated.epochs[1].start_height, 21)
        self.assertEqual(rotated.epochs[1].activation_finalized_hash, hex_bytes("aa"))
        restored = WitnessPolicy.from_public_document(json.loads(json.dumps(rotated.public_document())))
        self.assertEqual(restored, rotated)
        self.assertEqual(verify_stored_issue(restored, old_signature, self.observation, 125), self.message)
        with self.assertRaisesRegex(ValueError, "requires the current open signing epoch"):
            verify_fresh_issue(rotated, old_signature, self.observation, 126)

        new_observation = replace(self.observation, block_height=21, finalized_height=21,
                                  block_hash=hex_bytes("cc"), finalized_hash=hex_bytes("cc"))
        new_message = IssueMessage(self.route, new_observation, 2, 150, 185)
        new_signature = sign_issue(new_message, self.replacement)
        self.assertEqual(verify_fresh_issue(rotated, new_signature, new_observation, 175), new_message)
        with self.assertRaisesRegex(ValueError, "does not cover this source block height"):
            verify_stored_issue(rotated, sign_issue(replace(new_message, policy_epoch=1), self.original),
                                new_observation, 175)
        with self.assertRaisesRegex(ValueError, "does not cover this source block height"):
            verify_fresh_issue(rotated, sign_issue(replace(self.message, policy_epoch=2), self.replacement),
                               self.observation, 125)

    def test_incomplete_locator_and_overlapping_policy_rejected(self) -> None:
        document = sign_issue(self.message, self.original).to_document()
        del document["message"]["observation"]["log_index"]
        with self.assertRaisesRegex(ValueError, "Invalid signed issue document"):
            SignedIssue.from_document(document)
        with self.assertRaisesRegex(ValueError, "Conflicting hashes"):
            replace(self.observation, finalized_height=18)
        previous = replace(self.policy.epochs[0], end_height_exclusive=20)
        next_key = WitnessEpoch(2, 19, self.hex_key(self.replacement), None, hex_bytes("aa"))
        with self.assertRaisesRegex(ValueError, "ranges overlap"):
            WitnessPolicy(self.route, (previous, next_key))


if __name__ == "__main__":
    unittest.main()
