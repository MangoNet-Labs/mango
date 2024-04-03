// Copyright (c) MangoNet Labs Ltd.
// SPDX-License-Identifier: Apache-2.0

module mgo_system::validator {
    use std::ascii;

    use mgo::tx_context::TxContext;
    use std::string::{Self, String};
    use mgo::bag::{Self, Bag};
    use mgo::balance::{Self, Balance};
    use mgo::mgo::MGO;

    friend mgo_system::genesis;
    friend mgo_system::mgo_system_state_inner;
    friend mgo_system::validator_wrapper;

    struct ValidatorMetadata has store {
        mgo_address: address,
        protocol_pubkey_bytes: vector<u8>,
        network_pubkey_bytes: vector<u8>,
        worker_pubkey_bytes: vector<u8>,
        net_address: String,
        p2p_address: String,
        primary_address: String,
        worker_address: String,
        extra_fields: Bag,
    }

    struct Validator has store {
        metadata: ValidatorMetadata,
        voting_power: u64,
        stake: Balance<MGO>,
        extra_fields: Bag,
    }

    public(friend) fun new(
        mgo_address: address,
        protocol_pubkey_bytes: vector<u8>,
        network_pubkey_bytes: vector<u8>,
        worker_pubkey_bytes: vector<u8>,
        net_address: vector<u8>,
        p2p_address: vector<u8>,
        primary_address: vector<u8>,
        worker_address: vector<u8>,
        init_stake: Balance<MGO>,
        ctx: &mut TxContext
    ): Validator {
        let metadata = ValidatorMetadata {
            mgo_address,
            protocol_pubkey_bytes,
            network_pubkey_bytes,
            worker_pubkey_bytes,
            net_address: string::from_ascii(ascii::string(net_address)),
            p2p_address: string::from_ascii(ascii::string(p2p_address)),
            primary_address: string::from_ascii(ascii::string(primary_address)),
            worker_address: string::from_ascii(ascii::string(worker_address)),
            extra_fields: bag::new(ctx),
        };

        Validator {
            metadata,
            voting_power: balance::value(&init_stake),
            stake: init_stake,
            extra_fields: bag::new(ctx),
        }
    }

    public(friend) fun new_dummy_inactive_validator(
        ctx: &mut TxContext
    ): Validator {
        let metadata = ValidatorMetadata {
            mgo_address: @0x0,
            protocol_pubkey_bytes: vector[],
            network_pubkey_bytes: vector[],
            worker_pubkey_bytes: vector[],
            net_address: string::utf8(vector[]),
            p2p_address: string::utf8(vector[]),
            primary_address: string::utf8(vector[]),
            worker_address: string::utf8(vector[]),
            extra_fields: bag::new(ctx),
        };

        Validator {
            metadata,
            voting_power: 0,
            stake: balance::zero(),
            extra_fields: bag::new(ctx),
        }
    }
}
