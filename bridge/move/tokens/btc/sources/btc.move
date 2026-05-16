// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

module bridged_btc::btc {
    use std::option;

    use myso::coin::{Self, CoinCreationAdminCap};
    use myso::transfer;
    use myso::tx_context::{Self, TxContext};
    use myso::url;

    struct BTC has drop {}

    const DECIMAL: u8 = 8;

    fun init(_otw: BTC, _ctx: &mut TxContext) {
        // Empty - coin creation moved to init_coin entry function
    }

    public entry fun init_coin(
        admin_cap: &CoinCreationAdminCap,
        ctx: &mut TxContext
    ) {
        let (treasury_cap, metadata) = coin::create_currency_with_admin<BTC>(
            DECIMAL,
            b"BTC",
            b"Bitcoin",
            b"Bridged Bitcoin token",
            option::some(url::new_unsafe_from_bytes(b"https://www.mysocial.network/_next/image?url=%2Fbtc-icon.png&w=96&q=75")),
            admin_cap,
            ctx
        );
        transfer::public_freeze_object(metadata);
        transfer::public_transfer(treasury_cap, tx_context::sender(ctx));
    }
}
