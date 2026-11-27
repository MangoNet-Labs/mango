// SPDX-License-Identifier: Apache-2.0
pragma solidity 0.8.28;

/// Isolated local fixture for a non-financial test entitlement.
contract TestEntitlementSource {
    mapping(bytes32 => bytes32) private recorded;

    event TestEntitlementRequested(bytes32 indexed businessEventId, bytes32 payloadDigest);

    function record(bytes32 businessEventId, bytes32 payloadDigest) external {
        require(businessEventId != bytes32(0) && payloadDigest != bytes32(0), "empty fixture input");
        bytes32 existing = recorded[businessEventId];
        if (existing == bytes32(0)) {
            recorded[businessEventId] = payloadDigest;
            emit TestEntitlementRequested(businessEventId, payloadDigest);
        } else {
            require(existing == payloadDigest, "fixture event payload conflict");
        }
    }
}
