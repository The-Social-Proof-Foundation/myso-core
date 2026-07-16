/// Fee distribution for claimed paid-message escrow (`MYSO`).
///
/// Fee BPS are read from [`messaging_config::MessagingConfig`].
///
/// When `platform_fee_recipient` is [`NO_PLATFORM_FEE_RECIPIENT`] (`@0x0`), the platform share is
/// combined with the ecosystem share and sent to `ecosystem_fee_recipient` (wallet paid DMs with no
/// associated platform).
///
/// Uses `transfer::public_transfer` to fee recipients. Credits to the live `Platform` treasury balance
/// require `social_contracts::platform::fund_platform_treasury_from_coin` (see
/// `distribute_escrow_with_platform_treasury`).
module messaging::paid_escrow_settlement;

use messaging::messaging_config::{Self, MessagingConfig};
use myso::coin::{Self, Coin};
use myso::clock::{Self, Clock};
use myso::myso::MYSO;
use myso::transfer;
use myso::tx_context::TxContext;
use social_contracts::platform::{Self, Platform};

/// Sentinel: pass as `platform_fee_recipient` when no platform is associated with the paid DM.
public fun no_platform_fee_recipient(): address {
    @0x0
}

/// Totals from a settled escrow split (for events and testing).
public struct EscrowFeeTotals has copy, drop, store {
    total_amount: u64,
    platform_fee: u64,
    treasury_fee: u64,
    net_amount: u64,
}

public fun total_amount(t: &EscrowFeeTotals): u64 {
    t.total_amount
}

public fun platform_fee(t: &EscrowFeeTotals): u64 {
    t.platform_fee
}

public fun treasury_fee(t: &EscrowFeeTotals): u64 {
    t.treasury_fee
}

public fun net_amount(t: &EscrowFeeTotals): u64 {
    t.net_amount
}

/// Splits `escrow_coin` per paid-message BPS: platform, ecosystem, then `primary_recipient`.
public fun distribute_escrow_to_recipients(
    config: &MessagingConfig,
    mut escrow_coin: Coin<MYSO>,
    platform_fee_recipient: address,
    ecosystem_fee_recipient: address,
    primary_recipient: address,
    ctx: &mut TxContext,
): EscrowFeeTotals {
    let total_amount = coin::value(&escrow_coin);
    let platform_fee_bps = messaging_config::paid_msg_platform_fee_bps(config);
    let treasury_fee_bps = messaging_config::paid_msg_treasury_fee_bps(config);
    let platform_fee = (((total_amount as u128) * (platform_fee_bps as u128)) / 10000u128) as u64;
    let treasury_fee = (((total_amount as u128) * (treasury_fee_bps as u128)) / 10000u128) as u64;
    let net_amount = total_amount - platform_fee - treasury_fee;

    if (platform_fee_recipient == no_platform_fee_recipient()) {
        let ecosystem_total = platform_fee + treasury_fee;
        if (ecosystem_total > 0) {
            transfer::public_transfer(
                coin::split(&mut escrow_coin, ecosystem_total, ctx),
                ecosystem_fee_recipient,
            );
        };
    } else {
        if (platform_fee > 0) {
            transfer::public_transfer(
                coin::split(&mut escrow_coin, platform_fee, ctx),
                platform_fee_recipient,
            );
        };
        if (treasury_fee > 0) {
            transfer::public_transfer(
                coin::split(&mut escrow_coin, treasury_fee, ctx),
                ecosystem_fee_recipient,
            );
        };
    };
    transfer::public_transfer(escrow_coin, primary_recipient);

    EscrowFeeTotals { total_amount, platform_fee, treasury_fee, net_amount }
}

/// Splits `escrow_coin` per paid-message BPS and deposits the platform share into `Platform.treasury`.
public fun distribute_escrow_with_platform_treasury(
    config: &MessagingConfig,
    mut escrow_coin: Coin<MYSO>,
    platform: &mut Platform,
    ecosystem_fee_recipient: address,
    primary_recipient: address,
    clock: &Clock,
    ctx: &mut TxContext,
): EscrowFeeTotals {
    let total_amount = coin::value(&escrow_coin);
    let platform_fee_bps = messaging_config::paid_msg_platform_fee_bps(config);
    let treasury_fee_bps = messaging_config::paid_msg_treasury_fee_bps(config);
    let platform_fee = (((total_amount as u128) * (platform_fee_bps as u128)) / 10000u128) as u64;
    let treasury_fee = (((total_amount as u128) * (treasury_fee_bps as u128)) / 10000u128) as u64;
    let net_amount = total_amount - platform_fee - treasury_fee;

    if (platform_fee > 0) {
        let mut platform_fee_coin = coin::split(&mut escrow_coin, platform_fee, ctx);
        platform::fund_platform_treasury_from_coin(platform, &mut platform_fee_coin, platform_fee, clock, ctx);
        if (coin::value(&platform_fee_coin) > 0) {
            transfer::public_transfer(platform_fee_coin, primary_recipient);
        } else {
            coin::destroy_zero(platform_fee_coin);
        };
    };
    if (treasury_fee > 0) {
        transfer::public_transfer(
            coin::split(&mut escrow_coin, treasury_fee, ctx),
            ecosystem_fee_recipient,
        );
    };
    transfer::public_transfer(escrow_coin, primary_recipient);

    EscrowFeeTotals { total_amount, platform_fee, treasury_fee, net_amount }
}
