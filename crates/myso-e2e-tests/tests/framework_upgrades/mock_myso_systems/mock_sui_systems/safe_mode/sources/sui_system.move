// Copyright (c) Mysten Labs, Inc.
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
    use myso_system::myso_system_state_inner::MySoSystemStateInner;
    use myso_system::myso_system_state_inner;

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
        _new_epoch: u64,
        _next_protocol_version: u64,
        storage_rebate: u64,
        _non_refundable_storage_fee: u64,
        _storage_fund_reinvest_rate: u64,
        _reward_slashing_rate: u64,
        _epoch_start_timestamp_ms: u64,
        ctx: &mut TxContext,
    ) : Balance<MYSO> {
        let self = load_system_state_mut(wrapper);
        assert!(tx_context::sender(ctx) == @0x1, 0); // aborts here
        myso_system_state_inner::advance_epoch(
            self,
            storage_reward,
            computation_reward,
            storage_rebate,
        )
    }

    public fun active_validator_addresses(wrapper: &mut MySoSystemState): vector<address> {
        vector::empty()
    }

    fun load_system_state_mut(self: &mut MySoSystemState): &mut MySoSystemStateInner {
        let version = self.version;
        dynamic_field::borrow_mut(&mut self.id, version)
    }

    fun write_accumulator_storage_cost(
        _wrapper: &mut MySoSystemState,
        _storage_cost: u64,
        _ctx: &TxContext,
    ) {
    }
}
