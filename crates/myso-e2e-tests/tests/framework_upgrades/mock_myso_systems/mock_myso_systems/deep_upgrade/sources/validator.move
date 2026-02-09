// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

module myso_system::validator {
    use std::ascii;

    use myso::tx_context::TxContext;
    use std::string::{Self, String};
    use myso::bag::{Self, Bag};
    use myso::balance::{Self, Balance};
    use myso::myso::MYSO;

    public struct ValidatorMetadata has store {
        myso_address: address,
        protocol_pubkey_bytes: vector<u8>,
        network_pubkey_bytes: vector<u8>,
        worker_pubkey_bytes: vector<u8>,
        net_address: String,
        p2p_address: String,
        primary_address: String,
        worker_address: String,
        extra_fields: Bag,
    }

    public struct Validator has store {
        metadata: ValidatorMetadata,
        voting_power: u64,
        stake: Balance<MYSO>,
        extra_fields: Bag,
    }

    public struct ValidatorV2 has store {
        new_dummy_field: u64,
        metadata: ValidatorMetadata,
        voting_power: u64,
        stake: Balance<MYSO>,
        extra_fields: Bag,
    }

    public(package) fun new(
        myso_address: address,
        protocol_pubkey_bytes: vector<u8>,
        network_pubkey_bytes: vector<u8>,
        worker_pubkey_bytes: vector<u8>,
        net_address: vector<u8>,
        p2p_address: vector<u8>,
        primary_address: vector<u8>,
        worker_address: vector<u8>,
        init_stake: Balance<MYSO>,
        ctx: &mut TxContext
    ): Validator {
        let metadata = ValidatorMetadata {
            myso_address,
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

    public(package) fun v1_to_v2(v1: Validator): ValidatorV2 {
        let Validator {
            metadata,
            voting_power,
            stake,
            extra_fields,
        } = v1;
        ValidatorV2 {
            new_dummy_field: 100,
            metadata,
            voting_power,
            stake,
            extra_fields,
        }
    }
}
