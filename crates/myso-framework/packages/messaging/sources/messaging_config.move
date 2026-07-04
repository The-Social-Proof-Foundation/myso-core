/// Global configuration for paid messaging (fees, reply rules, dedupe limits).
module messaging::messaging_config;

use myso::clock::Clock;
use myso::event;
use social_contracts::bootstrap::MessagingAdminCap;

const BPS_DENOM: u64 = 10_000;

const DEFAULT_PAID_MSG_PLATFORM_FEE_BPS: u64 = 250;
const DEFAULT_PAID_MSG_TREASURY_FEE_BPS: u64 = 250;
const DEFAULT_PAYMENT_EXPIRATION_MS: u64 = 2_592_000_000;
const DEFAULT_MIN_REPLY_CHARS: u32 = 6;
const DEFAULT_MAX_DEDUPE_KEY_BYTES: u64 = 256;

const EInvalidFeeBps: u64 = 0;
const EInvalidPaymentExpiration: u64 = 1;
const EInvalidMinReplyChars: u64 = 2;
const EInvalidMaxDedupeKeyBytes: u64 = 3;

/// Shared singleton for paid-messaging parameters.
public struct MessagingConfig has key {
    id: UID,
    paid_msg_platform_fee_bps: u64,
    paid_msg_treasury_fee_bps: u64,
    payment_expiration_ms: u64,
    min_reply_chars: u32,
    max_dedupe_key_bytes: u64,
}

public struct MessagingConfigUpdatedEvent has copy, drop {
    updated_by: address,
    timestamp: u64,
    paid_msg_platform_fee_bps: u64,
    paid_msg_treasury_fee_bps: u64,
    payment_expiration_ms: u64,
    min_reply_chars: u32,
    max_dedupe_key_bytes: u64,
}

/// Shares the genesis [`MessagingConfig`] singleton. Called from `messaging::init`.
public(package) fun share_initial(ctx: &mut TxContext) {
    transfer::share_object(new_defaults(ctx));
}

public fun update_messaging_config(
    _admin: &MessagingAdminCap,
    config: &mut MessagingConfig,
    paid_msg_platform_fee_bps: u64,
    paid_msg_treasury_fee_bps: u64,
    payment_expiration_ms: u64,
    min_reply_chars: u32,
    max_dedupe_key_bytes: u64,
    clock: &Clock,
    ctx: &TxContext,
) {
    assert!(paid_msg_platform_fee_bps <= BPS_DENOM, EInvalidFeeBps);
    assert!(paid_msg_treasury_fee_bps <= BPS_DENOM, EInvalidFeeBps);
    assert!(payment_expiration_ms > 0, EInvalidPaymentExpiration);
    assert!(min_reply_chars > 0, EInvalidMinReplyChars);
    assert!(max_dedupe_key_bytes > 0, EInvalidMaxDedupeKeyBytes);

    config.paid_msg_platform_fee_bps = paid_msg_platform_fee_bps;
    config.paid_msg_treasury_fee_bps = paid_msg_treasury_fee_bps;
    config.payment_expiration_ms = payment_expiration_ms;
    config.min_reply_chars = min_reply_chars;
    config.max_dedupe_key_bytes = max_dedupe_key_bytes;

    event::emit(MessagingConfigUpdatedEvent {
        updated_by: ctx.sender(),
        timestamp: clock.timestamp_ms(),
        paid_msg_platform_fee_bps,
        paid_msg_treasury_fee_bps,
        payment_expiration_ms,
        min_reply_chars,
        max_dedupe_key_bytes,
    });
}

public fun paid_msg_platform_fee_bps(config: &MessagingConfig): u64 {
    config.paid_msg_platform_fee_bps
}

public fun paid_msg_treasury_fee_bps(config: &MessagingConfig): u64 {
    config.paid_msg_treasury_fee_bps
}

public fun payment_expiration_ms(config: &MessagingConfig): u64 {
    config.payment_expiration_ms
}

public fun min_reply_chars(config: &MessagingConfig): u32 {
    config.min_reply_chars
}

public fun max_dedupe_key_bytes(config: &MessagingConfig): u64 {
    config.max_dedupe_key_bytes
}

fun new_defaults(ctx: &mut TxContext): MessagingConfig {
    MessagingConfig {
        id: object::new(ctx),
        paid_msg_platform_fee_bps: DEFAULT_PAID_MSG_PLATFORM_FEE_BPS,
        paid_msg_treasury_fee_bps: DEFAULT_PAID_MSG_TREASURY_FEE_BPS,
        payment_expiration_ms: DEFAULT_PAYMENT_EXPIRATION_MS,
        min_reply_chars: DEFAULT_MIN_REPLY_CHARS,
        max_dedupe_key_bytes: DEFAULT_MAX_DEDUPE_KEY_BYTES,
    }
}

#[test_only]
public fun init_for_testing(ctx: &mut TxContext) {
    share_initial(ctx);
}
