// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

module myusd::myusd {
    use std::option;

    use myso::coin::{
        Self as coin,
        CoinCreationAdminCap,
        CoinMetadata,
        DenyCapV2,
        TreasuryCap,
    };
    use myso::deny_list::DenyList;
    use myso::transfer;
    use myso::tx_context::{Self, TxContext};
    use myso::url;

    struct MYUSD has drop {}

    const DECIMAL: u8 = 6;

    #[allow(lint(self_transfer))]
    public fun init_coin(
        admin_cap: &CoinCreationAdminCap,
        ctx: &mut TxContext,
    ) {
        let (treasury_cap, deny_cap, metadata): (TreasuryCap<MYUSD>, DenyCapV2<MYUSD>, CoinMetadata<MYUSD>) = coin::create_regulated_currency_with_admin<MYUSD>(
            DECIMAL,
            b"myUSD",
            b"MyUSD",
            b"The official MySocial USD stablecoin.",
            option::some(url::new_unsafe_from_bytes(b"https://www.mysocial.network/_next/image?url=%2FmyUSD_icon.webp&w=96&q=75")),
            false,
            admin_cap,
            ctx,
        );
        transfer::public_freeze_object(metadata);
        let sender = tx_context::sender(ctx);
        transfer::public_transfer(treasury_cap, sender);
        transfer::public_transfer(deny_cap, sender);
    }

    public fun add_addr_from_deny_list(
        denylist: &mut DenyList,
        denycap: &mut DenyCapV2<MYUSD>,
        denyaddy: address,
        ctx: &mut TxContext,
    ) {
        coin::deny_list_v2_add(denylist, denycap, denyaddy, ctx);
    }

    public fun remove_addr_from_deny_list(
        denylist: &mut DenyList,
        denycap: &mut DenyCapV2<MYUSD>,
        denyaddy: address,
        ctx: &mut TxContext,
    ) {
        coin::deny_list_v2_remove(denylist, denycap, denyaddy, ctx);
    }
}
