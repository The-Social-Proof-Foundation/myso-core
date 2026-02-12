// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

module locked_stake::locked_stake;

use locked_stake::epoch_time_lock::{Self, EpochTimeLock};
use myso::balance::{Self, Balance};
use myso::coin;
use myso::myso::MYSO;
use myso::vec_map::{Self, VecMap};
use myso_system::staking_pool::StakedMySo;
use myso_system::myso_system::{Self, MySoSystemState};

const EInsufficientBalance: u64 = 0;
const EStakeObjectNonExistent: u64 = 1;

/// An object that locks MYSO tokens and stake objects until a given epoch, and allows
/// staking and unstaking operations when locked.
public struct LockedStake has key {
    id: UID,
    staked_myso: VecMap<ID, StakedMySo>,
    myso: Balance<MYSO>,
    locked_until_epoch: EpochTimeLock,
}

// ============================= basic operations =============================

/// Create a new LockedStake object with empty staked_myso and myso balance given a lock time.
/// Aborts if the given epoch has already passed.
public fun new(locked_until_epoch: u64, ctx: &mut TxContext): LockedStake {
    LockedStake {
        id: object::new(ctx),
        staked_myso: vec_map::empty(),
        myso: balance::zero(),
        locked_until_epoch: epoch_time_lock::new(locked_until_epoch, ctx),
    }
}

/// Unlocks and returns all the assets stored inside this LockedStake object.
/// Aborts if the unlock epoch is in the future.
public fun unlock(ls: LockedStake, ctx: &TxContext): (VecMap<ID, StakedMySo>, Balance<MYSO>) {
    let LockedStake { id, staked_myso, myso, locked_until_epoch } = ls;
    epoch_time_lock::destroy(locked_until_epoch, ctx);
    object::delete(id);
    (staked_myso, myso)
}

/// Deposit a new stake object to the LockedStake object.
public fun deposit_staked_myso(ls: &mut LockedStake, staked_myso: StakedMySo) {
    let id = object::id(&staked_myso);
    // This insertion can't abort since each object has a unique id.
    vec_map::insert(&mut ls.staked_myso, id, staked_myso);
}

/// Deposit myso balance to the LockedStake object.
public fun deposit_myso(ls: &mut LockedStake, myso: Balance<MYSO>) {
    balance::join(&mut ls.myso, myso);
}

/// Take `amount` of MYSO from the myso balance, stakes it, and puts the stake object
/// back into the staked myso vec map.
public fun stake(
    ls: &mut LockedStake,
    myso_system: &mut MySoSystemState,
    amount: u64,
    validator_address: address,
    ctx: &mut TxContext,
) {
    assert!(balance::value(&ls.myso) >= amount, EInsufficientBalance);
    let stake = myso_system::request_add_stake_non_entry(
        myso_system,
        coin::from_balance(balance::split(&mut ls.myso, amount), ctx),
        validator_address,
        ctx,
    );
    deposit_staked_myso(ls, stake);
}

/// Unstake the stake object with `staked_myso_id` and puts the resulting principal
/// and rewards back into the locked myso balance.
/// Returns the amount of MYSO unstaked, including both principal and rewards.
/// Aborts if no stake exists with the given id.
public fun unstake(
    ls: &mut LockedStake,
    myso_system: &mut MySoSystemState,
    staked_myso_id: ID,
    ctx: &mut TxContext,
): u64 {
    assert!(vec_map::contains(&ls.staked_myso, &staked_myso_id), EStakeObjectNonExistent);
    let (_, stake) = vec_map::remove(&mut ls.staked_myso, &staked_myso_id);
    let myso_balance = myso_system::request_withdraw_stake_non_entry(myso_system, stake, ctx);
    let amount = balance::value(&myso_balance);
    deposit_myso(ls, myso_balance);
    amount
}

// ============================= getters =============================

public fun staked_myso(ls: &LockedStake): &VecMap<ID, StakedMySo> {
    &ls.staked_myso
}

public fun myso_balance(ls: &LockedStake): u64 {
    balance::value(&ls.myso)
}

public fun locked_until_epoch(ls: &LockedStake): u64 {
    epoch_time_lock::epoch(&ls.locked_until_epoch)
}

// TODO: possibly add some scenarios like switching stake, creating a new LockedStake and transferring
// it to the sender, etc. But these can also be done as PTBs.
