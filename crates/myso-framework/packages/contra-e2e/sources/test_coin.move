// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

/// Minimal test coin for contra E2E scripts (`scripts/contra-runnable.sh`).
module contra_e2e::test_coin;

use myso::coin::{CoinCreationAdminCap, TreasuryCap};
use myso::coin_registry::{Self, CoinRegistry};

public struct TEST_COIN has key { id: UID }

/// Register `TEST_COIN` on the shared `CoinRegistry` and return `TreasuryCap` to the caller.
public fun create(
    registry: &mut CoinRegistry,
    admin_cap: &CoinCreationAdminCap,
    ctx: &mut TxContext,
): TreasuryCap<TEST_COIN> {
    let (currency, treasury_cap) = coin_registry::new_currency(
        registry,
        8,
        b"CTST".to_string(),
        b"ContraTestCoin".to_string(),
        b"Contra E2E test coin".to_string(),
        b"".to_string(),
        admin_cap,
        ctx,
    );
    let metadata_cap = currency.finalize(ctx);
    transfer::public_transfer(metadata_cap, ctx.sender());
    treasury_cap
}
