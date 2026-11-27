// SPDX-License-Identifier: Apache-2.0
pragma solidity 0.8.28;

/// Testnet source of non-financial test entitlements.
contract S6TestEntitlementSource {
    address public publisher;
    mapping(bytes32 => bytes32) private recorded;

    event TestEntitlementRequested(bytes32 indexed businessEventId, bytes32 payloadDigest);

    error UnauthorizedPublisher();
    error EmptyInput();
    error PayloadConflict();

    constructor() {
        publisher = msg.sender;
    }

    function record(bytes32 businessEventId, bytes32 payloadDigest) external {
        if (msg.sender != publisher) revert UnauthorizedPublisher();
        if (businessEventId == bytes32(0) || payloadDigest == bytes32(0)) revert EmptyInput();
        bytes32 existing = recorded[businessEventId];
        if (existing == bytes32(0)) {
            recorded[businessEventId] = payloadDigest;
            emit TestEntitlementRequested(businessEventId, payloadDigest);
        } else if (existing != payloadDigest) {
            revert PayloadConflict();
        }
    }
}
