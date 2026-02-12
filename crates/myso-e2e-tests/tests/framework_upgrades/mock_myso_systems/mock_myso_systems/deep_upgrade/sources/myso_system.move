// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

module myso_system::myso_system {
    use std::vector;

    use myso::balance::Balance;
    use myso::object::UID;
    use myso::myso::MYSO;
    use myso::transfer;
    use myso::tx_context::{Self, TxContext};
    use myso::dynamic_field;

    use myso_system::validator::Validator;
    use myso_system::myso_system_state_inner::{Self, MySoSystemStateInnerV2, MySoSystemStateInner};

    public struct MySoSystemState has key {
        id: UID,
        version: u64,
    }

    public(package) fun create(
        id: UID,
        validators: vector<Validator>,
        storage_fund: Balance<MYSO>,
        protocol_version: u64,
        epoch_start_timestamp_ms: u64,
        epoch_duration_ms: u64,
        ctx: &mut TxContext,
    ) {
        let system_state = myso_system_state_inner::create(
            validators,
            storage_fund,
            protocol_version,
            epoch_start_timestamp_ms,
            epoch_duration_ms,
            ctx,
        );
        let version = myso_system_state_inner::genesis_system_state_version();
        let mut self = MySoSystemState {
            id,
            version,
        };
        dynamic_field::add(&mut self.id, version, system_state);
        transfer::share_object(self);
    }

    fun advance_epoch(
        storage_reward: Balance<MYSO>,
        computation_reward: Balance<MYSO>,
        wrapper: &mut MySoSystemState,
        new_epoch: u64,
        next_protocol_version: u64,
        storage_rebate: u64,
        _non_refundable_storage_fee: u64,
        _storage_fund_reinvest_rate: u64, // share of storage fund's rewards that's reinvested
                                         // into storage fund, in basis point.
        _reward_slashing_rate: u64, // how much rewards are slashed to punish a validator, in bps.
        epoch_start_timestamp_ms: u64, // Timestamp of the epoch start
        ctx: &mut TxContext,
    ) : Balance<MYSO> {
        let self = load_system_state_mut(wrapper);
        assert!(tx_context::sender(ctx) == @0x0, 0);
        let storage_rebate = myso_system_state_inner::advance_epoch(
            self,
            new_epoch,
            next_protocol_version,
            storage_reward,
            computation_reward,
            storage_rebate,
            epoch_start_timestamp_ms,
        );

        storage_rebate
    }

    public fun active_validator_addresses(wrapper: &mut MySoSystemState): vector<address> {
        vector::empty()
    }

    fun load_system_state_mut(self: &mut MySoSystemState): &mut MySoSystemStateInnerV2 {
        load_inner_maybe_upgrade(self)
    }

    fun load_inner_maybe_upgrade(self: &mut MySoSystemState): &mut MySoSystemStateInnerV2 {
        let mut version = self.version;
        if (version == myso_system_state_inner::genesis_system_state_version()) {
            let inner: MySoSystemStateInner = dynamic_field::remove(&mut self.id, version);
            let new_inner = myso_system_state_inner::v1_to_v2(inner);
            version = myso_system_state_inner::system_state_version(&new_inner);
            dynamic_field::add(&mut self.id, version, new_inner);
            self.version = version;
        };

        let inner: &mut MySoSystemStateInnerV2 = dynamic_field::borrow_mut(&mut self.id, version);
        assert!(myso_system_state_inner::system_state_version(inner) == version, 0);
        inner
    }

    fun store_execution_time_estimates(wrapper: &mut MySoSystemState, estimates_bytes: vector<u8>) {
        let self = load_system_state_mut(wrapper);
        myso_system_state_inner::store_execution_time_estimates(self, estimates_bytes)
    }

    fun store_execution_time_estimates_v2(wrapper: &mut MySoSystemState, estimate_chunks: vector<vector<u8>>) {
        let self = load_system_state_mut(wrapper);
        myso_system_state_inner::store_execution_time_estimates_v2(self, estimate_chunks)
    }

    fun write_accumulator_storage_cost(
        _wrapper: &mut MySoSystemState,
        _storage_cost: u64,
        _ctx: &TxContext,
    ) {
    }
}
