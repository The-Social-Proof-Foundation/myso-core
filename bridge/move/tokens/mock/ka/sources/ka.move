// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

module bridged_ka::ka {
    use std::option;

    use myso::coin::{Self, CoinCreationAdminCap};
    use myso::transfer;
    use myso::tx_context::{Self, TxContext};

    struct KA has drop {}

    const DECIMAL: u8 = 9;

    fun init(_otw: KA, _ctx: &mut TxContext) {
        // Empty - coin creation moved to init_coin entry function
    }

    public entry fun init_coin(
        admin_cap: &CoinCreationAdminCap,
        ctx: &mut TxContext
    ) {
        let (treasury_cap, metadata) = coin::create_currency_with_admin<KA>(
            DECIMAL,
            b"Ka",
            b"Ka Coin",
            b"Ka, the opposite of MySo",
            option::none(),
            admin_cap,
            ctx
        );
        transfer::public_freeze_object(metadata);
        transfer::public_transfer(treasury_cap, tx_context::sender(ctx));
    }
}
