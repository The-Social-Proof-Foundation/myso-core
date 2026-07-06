// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

/// Subscription module for the MySocial network
/// Handles subscription services for profiles & MyData

#[allow(duplicate_alias, lint(public_entry))]
module social_contracts::subscription {
    use std::option::{Self, Option};
    use std::string;

    use myso::{
        object::{Self, UID, ID},
        tx_context::{Self, TxContext},
        transfer,
        clock::{Self, Clock},
        coin::{Self, Coin},
        balance::{Self, Balance},
        event
    };
    use myso::myso::MYSO;
    use social_contracts::upgrade::{Self, UpgradeAdminCap};
    use social_contracts::platform::{Self, Platform};
    use social_contracts::profile::{Self, EcosystemTreasury, Profile};

    /// Error codes
    const EInvalidFee: u64 = 12;
    const ENoAccess: u64 = 77;
    const ESubscriptionExpired: u64 = 78;
    const EAutoRenewalDisabled: u64 = 79;
    const ENotSubscriptionOwner: u64 = 80;
    const EWrongVersion: u64 = 81;
    const EOverflow: u64 = 82;
    const EInvalidInput: u64 = 83;
    const EInvalidConfig: u64 = 84;

    /// Default bootstrap values (used only at init)
    const MAX_RENEWAL_MONTHS: u64 = 120;
    const MAX_U64: u64 = 18446744073709551615;
    const THIRTY_DAYS_MS: u64 = 2_592_000_000;
    const BPS_DENOM: u64 = 10_000;
    const DEFAULT_PLATFORM_FEE_BPS: u64 = 250;
    const DEFAULT_ECOSYSTEM_FEE_BPS: u64 = 250;
    const DEFAULT_NON_PLATFORM_PLATFORM_TO_CREATOR_BPS: u64 = 0;
    const DEFAULT_NON_PLATFORM_PLATFORM_TO_TREASURY_BPS: u64 = 10_000;

    /// Admin capability for subscription configuration
    public struct SubscriptionAdminCap has key, store {
        id: UID,
    }

    /// Global subscription feature configuration
    public struct SubscriptionConfig has key {
        id: UID,
        billing_period_ms: u64,
        max_renewal_months: u64,
        platform_fee_bps: u64,
        ecosystem_fee_bps: u64,
        non_platform_platform_to_creator_bps: u64,
        non_platform_platform_to_treasury_bps: u64,
        version: u64,
    }

    public struct SubscriptionConfigUpdatedEvent has copy, drop {
        updated_by: address,
        billing_period_ms: u64,
        max_renewal_months: u64,
        platform_fee_bps: u64,
        ecosystem_fee_bps: u64,
        non_platform_platform_to_creator_bps: u64,
        non_platform_platform_to_treasury_bps: u64,
        timestamp: u64,
    }

    /// Profile subscription service - one per profile
    public struct ProfileSubscriptionService has key {
        id: UID,
        /// Profile owner who receives subscription fees
        profile_owner: address,
        /// Monthly subscription fee in MYSO
        monthly_fee: u64,
        /// Whether this service allows new subscriptions
        active: bool,
        /// Total number of active subscribers
        subscriber_count: u64,
        /// Version for upgrades
        version: u64,
    }

    /// Individual subscription to a profile
    public struct ProfileSubscription has key {
        id: UID,
        /// The profile service this subscription is for
        service_id: ID,
        /// Subscriber's address
        subscriber: address,
        /// When the subscription was created
        created_at: u64,
        /// When the subscription expires (timestamp in ms)
        expires_at: u64,
        /// Whether auto-renewal is enabled
        auto_renew: bool,
        /// Balance for auto-renewal payments
        renewal_balance: Balance<MYSO>,
        /// Number of times this subscription has been renewed
        renewal_count: u64,
    }

    /// Events
    public struct ProfileSubscriptionCreatedEvent has copy, drop {
        service_id: ID,
        subscriber: address,
        expires_at: u64,
        monthly_fee: u64,
        auto_renew: bool,
        platform_fee: u64,
        ecosystem_fee: u64,
        creator_amount: u64,
        platform_id: Option<address>,
    }

    public struct ProfileSubscriptionRenewedEvent has copy, drop {
        subscription_id: ID,
        subscriber: address,
        new_expires_at: u64,
        renewal_count: u64,
        auto_renewed: bool,
        platform_fee: u64,
        ecosystem_fee: u64,
        creator_amount: u64,
        platform_id: Option<address>,
    }

    public struct ProfileSubscriptionCancelledEvent has copy, drop {
        subscription_id: ID,
        subscriber: address,
        refunded_amount: u64,
    }

    /// Additional event for fee updates
    public struct ProfileSubscriptionUpdatedEvent has copy, drop {
        service_id: ID,
        old_fee: u64,
        new_fee: u64,
        updated_by: address,
    }

    /// Event emitted when a subscription service is created
    public struct ProfileSubscriptionServiceCreatedEvent has copy, drop {
        service_id: ID,
        profile_owner: address,
        monthly_fee: u64,
        created_at: u64,
    }

    /// Event emitted when renewal balance is funded
    public struct RenewalBalanceFundedEvent has copy, drop {
        subscription_id: ID,
        subscriber: address,
        funded_amount: u64,
        new_balance: u64,
        timestamp: u64,
    }

    /// Event emitted when a subscription service is deactivated
    public struct ProfileSubscriptionServiceDeactivatedEvent has copy, drop {
        service_id: ID,
        profile_owner: address,
        deactivated_at: u64,
    }

    fun validate_fee_config(
        platform_fee_bps: u64,
        ecosystem_fee_bps: u64,
        non_platform_platform_to_creator_bps: u64,
        non_platform_platform_to_treasury_bps: u64,
    ) {
        assert!(platform_fee_bps <= BPS_DENOM, EInvalidConfig);
        assert!(ecosystem_fee_bps <= BPS_DENOM, EInvalidConfig);
        assert!(platform_fee_bps + ecosystem_fee_bps <= BPS_DENOM, EInvalidConfig);
        assert!(non_platform_platform_to_creator_bps <= BPS_DENOM, EInvalidConfig);
        assert!(non_platform_platform_to_treasury_bps <= BPS_DENOM, EInvalidConfig);
        assert!(
            non_platform_platform_to_creator_bps + non_platform_platform_to_treasury_bps == BPS_DENOM,
            EInvalidConfig,
        );
    }

    fun calculate_subscription_fees(config: &SubscriptionConfig, gross: u64): (u64, u64, u64) {
        let platform_fee = (gross * config.platform_fee_bps) / BPS_DENOM;
        let ecosystem_fee = (gross * config.ecosystem_fee_bps) / BPS_DENOM;
        let creator_amount = gross - platform_fee - ecosystem_fee;
        (platform_fee, ecosystem_fee, creator_amount)
    }

    fun route_non_platform_platform_fee(
        config: &SubscriptionConfig,
        treasury: &EcosystemTreasury,
        platform_fee: u64,
        creator_amount: u64,
        payment: &mut Coin<MYSO>,
        ctx: &mut TxContext,
    ): u64 {
        let platform_fee_to_creator =
            (platform_fee * config.non_platform_platform_to_creator_bps) / BPS_DENOM;
        let platform_fee_to_treasury = platform_fee - platform_fee_to_creator;
        let creator_amount = creator_amount + platform_fee_to_creator;
        if (platform_fee_to_treasury > 0) {
            let treasury_coin = coin::split(payment, platform_fee_to_treasury, ctx);
            transfer::public_transfer(treasury_coin, profile::get_treasury_address(treasury));
        };
        creator_amount
    }

    fun distribute_subscription_payment_fees_no_platform(
        config: &SubscriptionConfig,
        treasury: &EcosystemTreasury,
        profile_owner: address,
        payment: Coin<MYSO>,
        ctx: &mut TxContext,
    ): (u64, u64, u64) {
        let gross = coin::value(&payment);
        let (platform_fee, ecosystem_fee, creator_amount) = calculate_subscription_fees(config, gross);
        let mut payment = payment;

        if (ecosystem_fee > 0) {
            let eco_coin = coin::split(&mut payment, ecosystem_fee, ctx);
            transfer::public_transfer(eco_coin, profile::get_treasury_address(treasury));
        };

        let creator_amount = if (platform_fee > 0) {
            route_non_platform_platform_fee(
                config,
                treasury,
                platform_fee,
                creator_amount,
                &mut payment,
                ctx,
            )
        } else {
            creator_amount
        };

        transfer::public_transfer(payment, profile_owner);
        (platform_fee, ecosystem_fee, creator_amount)
    }

    fun distribute_subscription_payment_fees_with_platform(
        config: &SubscriptionConfig,
        treasury: &EcosystemTreasury,
        profile_owner: address,
        payment: Coin<MYSO>,
        platform: &mut Platform,
        clock: &Clock,
        ctx: &mut TxContext,
    ): (u64, u64, u64) {
        let gross = coin::value(&payment);
        let (platform_fee, ecosystem_fee, creator_amount) = calculate_subscription_fees(config, gross);
        let mut payment = payment;

        if (ecosystem_fee > 0) {
            let eco_coin = coin::split(&mut payment, ecosystem_fee, ctx);
            transfer::public_transfer(eco_coin, profile::get_treasury_address(treasury));
        };

        if (platform_fee > 0) {
            let mut platform_coin = coin::split(&mut payment, platform_fee, ctx);
            platform::add_to_treasury(platform, &mut platform_coin, platform_fee, clock, ctx);
            coin::destroy_zero(platform_coin);
        };

        transfer::public_transfer(payment, profile_owner);
        (platform_fee, ecosystem_fee, creator_amount)
    }

    fun emit_subscription_config_updated(
        config: &SubscriptionConfig,
        updated_by: address,
        timestamp: u64,
    ) {
        event::emit(SubscriptionConfigUpdatedEvent {
            updated_by,
            billing_period_ms: config.billing_period_ms,
            max_renewal_months: config.max_renewal_months,
            platform_fee_bps: config.platform_fee_bps,
            ecosystem_fee_bps: config.ecosystem_fee_bps,
            non_platform_platform_to_creator_bps: config.non_platform_platform_to_creator_bps,
            non_platform_platform_to_treasury_bps: config.non_platform_platform_to_treasury_bps,
            timestamp,
        });
    }

    /// Create a subscription service for a profile (called by profile owner)
    public fun create_profile_service(
        profile_owner: address,
        monthly_fee: u64,
        ctx: &mut TxContext
    ): ProfileSubscriptionService {
        assert!(monthly_fee > 0, EInvalidFee);

        ProfileSubscriptionService {
            id: object::new(ctx),
            profile_owner,
            monthly_fee,
            active: true,
            subscriber_count: 0,
            version: upgrade::current_version(),
        }
    }

    /// Entry function to create and share a profile subscription service
    public entry fun create_profile_service_entry(
        profile: &Profile,
        monthly_fee: u64,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        assert!(tx_context::sender(ctx) == profile::get_owner(profile), ENoAccess);
        assert!(monthly_fee > 0, EInvalidFee);

        let profile_owner = profile::get_owner(profile);
        let service = create_profile_service(
            profile_owner,
            monthly_fee,
            ctx
        );
        let service_id = object::id(&service);

        transfer::share_object(service);

        event::emit(ProfileSubscriptionServiceCreatedEvent {
            service_id,
            profile_owner,
            monthly_fee,
            created_at: clock::timestamp_ms(clock),
        });
    }

    fun subscribe_to_profile_internal_no_platform(
        config: &SubscriptionConfig,
        service: &mut ProfileSubscriptionService,
        treasury: &EcosystemTreasury,
        payment: &mut Coin<MYSO>,
        auto_renew: bool,
        renewal_months: u64,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(service.version == upgrade::current_version(), EWrongVersion);
        assert!(service.active, ENoAccess);

        if (auto_renew) {
            assert!(renewal_months <= config.max_renewal_months, EInvalidInput);
        };

        let subscriber = tx_context::sender(ctx);
        let now = clock::timestamp_ms(clock);

        let months_to_pay = if (auto_renew) { 1 + renewal_months } else { 1 };
        assert!(months_to_pay <= MAX_U64 / service.monthly_fee, EOverflow);
        let total_required = service.monthly_fee * months_to_pay;
        assert!(coin::value(payment) >= total_required, EInvalidFee);

        let first_month_payment = coin::split(payment, service.monthly_fee, ctx);
        let (platform_fee, ecosystem_fee, creator_amount) =
            distribute_subscription_payment_fees_no_platform(
                config,
                treasury,
                service.profile_owner,
                first_month_payment,
                ctx,
            );

        finish_subscribe(
            config,
            service,
            payment,
            auto_renew,
            renewal_months,
            subscriber,
            now,
            platform_fee,
            ecosystem_fee,
            creator_amount,
            option::none(),
            ctx,
        );
    }

    fun subscribe_to_profile_internal_with_platform(
        config: &SubscriptionConfig,
        service: &mut ProfileSubscriptionService,
        treasury: &EcosystemTreasury,
        platform: &mut Platform,
        payment: &mut Coin<MYSO>,
        auto_renew: bool,
        renewal_months: u64,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(service.version == upgrade::current_version(), EWrongVersion);
        assert!(service.active, ENoAccess);

        if (auto_renew) {
            assert!(renewal_months <= config.max_renewal_months, EInvalidInput);
        };

        let subscriber = tx_context::sender(ctx);
        let now = clock::timestamp_ms(clock);

        let months_to_pay = if (auto_renew) { 1 + renewal_months } else { 1 };
        assert!(months_to_pay <= MAX_U64 / service.monthly_fee, EOverflow);
        let total_required = service.monthly_fee * months_to_pay;
        assert!(coin::value(payment) >= total_required, EInvalidFee);

        let first_month_payment = coin::split(payment, service.monthly_fee, ctx);
        let (platform_fee, ecosystem_fee, creator_amount) =
            distribute_subscription_payment_fees_with_platform(
                config,
                treasury,
                service.profile_owner,
                first_month_payment,
                platform,
                clock,
                ctx,
            );

        finish_subscribe(
            config,
            service,
            payment,
            auto_renew,
            renewal_months,
            subscriber,
            now,
            platform_fee,
            ecosystem_fee,
            creator_amount,
            option::some(object::uid_to_address(platform::id(platform))),
            ctx,
        );
    }

    fun finish_subscribe(
        config: &SubscriptionConfig,
        service: &mut ProfileSubscriptionService,
        payment: &mut Coin<MYSO>,
        auto_renew: bool,
        renewal_months: u64,
        subscriber: address,
        now: u64,
        platform_fee: u64,
        ecosystem_fee: u64,
        creator_amount: u64,
        platform_id: Option<address>,
        ctx: &mut TxContext,
    ) {
        let renewal_balance = if (auto_renew && renewal_months > 0) {
            assert!(renewal_months <= MAX_U64 / service.monthly_fee, EOverflow);
            let renewal_amount = service.monthly_fee * renewal_months;
            let renewal_payment = coin::split(payment, renewal_amount, ctx);
            coin::into_balance(renewal_payment)
        } else {
            balance::zero<MYSO>()
        };

        let billing_period_ms = config.billing_period_ms;
        assert!(now <= MAX_U64 - billing_period_ms, EOverflow);
        let expires_at = now + billing_period_ms;

        let subscription = ProfileSubscription {
            id: object::new(ctx),
            service_id: object::id(service),
            subscriber,
            created_at: now,
            expires_at,
            auto_renew,
            renewal_balance,
            renewal_count: 0,
        };

        assert!(service.subscriber_count <= MAX_U64 - 1, EOverflow);
        service.subscriber_count = service.subscriber_count + 1;

        event::emit(ProfileSubscriptionCreatedEvent {
            service_id: object::id(service),
            subscriber,
            expires_at,
            monthly_fee: service.monthly_fee,
            auto_renew,
            platform_fee,
            ecosystem_fee,
            creator_amount,
            platform_id,
        });

        transfer::transfer(subscription, subscriber);
    }

    /// Subscribe to a profile with optional auto-renewal (no platform fee recipient).
    public entry fun subscribe_to_profile(
        config: &SubscriptionConfig,
        service: &mut ProfileSubscriptionService,
        treasury: &EcosystemTreasury,
        payment: &mut Coin<MYSO>,
        auto_renew: bool,
        renewal_months: u64,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        subscribe_to_profile_internal_no_platform(
            config,
            service,
            treasury,
            payment,
            auto_renew,
            renewal_months,
            clock,
            ctx,
        );
    }

    /// Subscribe to a profile with platform treasury routing for the platform-fee slice.
    public entry fun subscribe_to_profile_with_platform(
        config: &SubscriptionConfig,
        service: &mut ProfileSubscriptionService,
        treasury: &EcosystemTreasury,
        platform: &mut Platform,
        payment: &mut Coin<MYSO>,
        auto_renew: bool,
        renewal_months: u64,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        subscribe_to_profile_internal_with_platform(
            config,
            service,
            treasury,
            platform,
            payment,
            auto_renew,
            renewal_months,
            clock,
            ctx,
        );
    }

    fun renew_subscription_internal_no_platform(
        config: &SubscriptionConfig,
        service: &ProfileSubscriptionService,
        subscription: &mut ProfileSubscription,
        payment: Coin<MYSO>,
        treasury: &EcosystemTreasury,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(service.version == upgrade::current_version(), EWrongVersion);

        let subscriber = tx_context::sender(ctx);
        assert!(subscription.subscriber == subscriber, ENotSubscriptionOwner);
        assert!(subscription.service_id == object::id(service), ENoAccess);
        assert!(coin::value(&payment) >= service.monthly_fee, EInvalidFee);

        let (platform_fee, ecosystem_fee, creator_amount) =
            distribute_subscription_payment_fees_no_platform(
                config,
                treasury,
                service.profile_owner,
                payment,
                ctx,
            );

        emit_subscription_renewed(
            subscription,
            subscriber,
            config,
            platform_fee,
            ecosystem_fee,
            creator_amount,
            option::none(),
            false,
            clock,
        );
    }

    fun renew_subscription_internal_with_platform(
        config: &SubscriptionConfig,
        service: &ProfileSubscriptionService,
        subscription: &mut ProfileSubscription,
        payment: Coin<MYSO>,
        treasury: &EcosystemTreasury,
        platform: &mut Platform,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(service.version == upgrade::current_version(), EWrongVersion);

        let subscriber = tx_context::sender(ctx);
        assert!(subscription.subscriber == subscriber, ENotSubscriptionOwner);
        assert!(subscription.service_id == object::id(service), ENoAccess);
        assert!(coin::value(&payment) >= service.monthly_fee, EInvalidFee);

        let (platform_fee, ecosystem_fee, creator_amount) =
            distribute_subscription_payment_fees_with_platform(
                config,
                treasury,
                service.profile_owner,
                payment,
                platform,
                clock,
                ctx,
            );

        emit_subscription_renewed(
            subscription,
            subscriber,
            config,
            platform_fee,
            ecosystem_fee,
            creator_amount,
            option::some(object::uid_to_address(platform::id(platform))),
            false,
            clock,
        );
    }

    fun emit_subscription_renewed(
        subscription: &mut ProfileSubscription,
        subscriber: address,
        config: &SubscriptionConfig,
        platform_fee: u64,
        ecosystem_fee: u64,
        creator_amount: u64,
        platform_id: Option<address>,
        auto_renewed: bool,
        clock: &Clock,
    ) {
        let now = clock::timestamp_ms(clock);
        let extension = config.billing_period_ms;

        subscription.expires_at = if (now > subscription.expires_at) {
            assert!(now <= MAX_U64 - extension, EOverflow);
            now + extension
        } else {
            assert!(subscription.expires_at <= MAX_U64 - extension, EOverflow);
            subscription.expires_at + extension
        };

        assert!(subscription.renewal_count <= MAX_U64 - 1, EOverflow);
        subscription.renewal_count = subscription.renewal_count + 1;

        event::emit(ProfileSubscriptionRenewedEvent {
            subscription_id: object::id(subscription),
            subscriber,
            new_expires_at: subscription.expires_at,
            renewal_count: subscription.renewal_count,
            auto_renewed,
            platform_fee,
            ecosystem_fee,
            creator_amount,
            platform_id,
        });
    }

    /// Manually renew a subscription (no platform).
    public entry fun renew_subscription(
        config: &SubscriptionConfig,
        service: &ProfileSubscriptionService,
        subscription: &mut ProfileSubscription,
        treasury: &EcosystemTreasury,
        payment: Coin<MYSO>,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        renew_subscription_internal_no_platform(
            config,
            service,
            subscription,
            payment,
            treasury,
            clock,
            ctx,
        );
    }

    /// Manually renew a subscription with platform treasury routing.
    public entry fun renew_subscription_with_platform(
        config: &SubscriptionConfig,
        service: &ProfileSubscriptionService,
        subscription: &mut ProfileSubscription,
        treasury: &EcosystemTreasury,
        platform: &mut Platform,
        payment: Coin<MYSO>,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        renew_subscription_internal_with_platform(
            config,
            service,
            subscription,
            payment,
            treasury,
            platform,
            clock,
            ctx,
        );
    }

    fun auto_renew_subscription_internal_no_platform(
        config: &SubscriptionConfig,
        service: &ProfileSubscriptionService,
        subscription: &mut ProfileSubscription,
        treasury: &EcosystemTreasury,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(service.version == upgrade::current_version(), EWrongVersion);
        assert!(subscription.service_id == object::id(service), ENoAccess);
        assert!(subscription.auto_renew, EAutoRenewalDisabled);
        assert!(service.active, ENoAccess);

        let now = clock::timestamp_ms(clock);
        assert!(subscription.expires_at <= now, ESubscriptionExpired);

        let renewal_balance_value = balance::value(&subscription.renewal_balance);
        if (renewal_balance_value < service.monthly_fee) {
            subscription.auto_renew = false;
            event::emit(ProfileSubscriptionCancelledEvent {
                subscription_id: object::id(subscription),
                subscriber: subscription.subscriber,
                refunded_amount: 0,
            });
            return
        };

        let renewal_payment = coin::from_balance(
            balance::split(&mut subscription.renewal_balance, service.monthly_fee),
            ctx
        );
        let (platform_fee, ecosystem_fee, creator_amount) =
            distribute_subscription_payment_fees_no_platform(
                config,
                treasury,
                service.profile_owner,
                renewal_payment,
                ctx,
            );

        let subscriber = subscription.subscriber;
        emit_subscription_renewed(
            subscription,
            subscriber,
            config,
            platform_fee,
            ecosystem_fee,
            creator_amount,
            option::none(),
            true,
            clock,
        );
    }

    fun auto_renew_subscription_internal_with_platform(
        config: &SubscriptionConfig,
        service: &ProfileSubscriptionService,
        subscription: &mut ProfileSubscription,
        treasury: &EcosystemTreasury,
        platform: &mut Platform,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(service.version == upgrade::current_version(), EWrongVersion);
        assert!(subscription.service_id == object::id(service), ENoAccess);
        assert!(subscription.auto_renew, EAutoRenewalDisabled);
        assert!(service.active, ENoAccess);

        let now = clock::timestamp_ms(clock);
        assert!(subscription.expires_at <= now, ESubscriptionExpired);

        let renewal_balance_value = balance::value(&subscription.renewal_balance);
        if (renewal_balance_value < service.monthly_fee) {
            subscription.auto_renew = false;
            event::emit(ProfileSubscriptionCancelledEvent {
                subscription_id: object::id(subscription),
                subscriber: subscription.subscriber,
                refunded_amount: 0,
            });
            return
        };

        let renewal_payment = coin::from_balance(
            balance::split(&mut subscription.renewal_balance, service.monthly_fee),
            ctx
        );
        let (platform_fee, ecosystem_fee, creator_amount) =
            distribute_subscription_payment_fees_with_platform(
                config,
                treasury,
                service.profile_owner,
                renewal_payment,
                platform,
                clock,
                ctx,
            );

        let subscriber = subscription.subscriber;
        emit_subscription_renewed(
            subscription,
            subscriber,
            config,
            platform_fee,
            ecosystem_fee,
            creator_amount,
            option::some(object::uid_to_address(platform::id(platform))),
            true,
            clock,
        );
    }

    /// Gas-optimized auto-renew using pre-funded renewal balance (no platform).
    public entry fun auto_renew_subscription(
        config: &SubscriptionConfig,
        service: &ProfileSubscriptionService,
        subscription: &mut ProfileSubscription,
        treasury: &EcosystemTreasury,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        auto_renew_subscription_internal_no_platform(
            config,
            service,
            subscription,
            treasury,
            clock,
            ctx,
        );
    }

    /// Gas-optimized auto-renew with platform treasury routing.
    public entry fun auto_renew_subscription_with_platform(
        config: &SubscriptionConfig,
        service: &ProfileSubscriptionService,
        subscription: &mut ProfileSubscription,
        treasury: &EcosystemTreasury,
        platform: &mut Platform,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        auto_renew_subscription_internal_with_platform(
            config,
            service,
            subscription,
            treasury,
            platform,
            clock,
            ctx,
        );
    }

    /// Check if subscription is eligible for auto-renewal without expensive operations
    public fun can_auto_renew(
        subscription: &ProfileSubscription,
        service: &ProfileSubscriptionService,
        clock: &Clock
    ): bool {
        if (!subscription.auto_renew) return false;
        if (subscription.service_id != object::id(service)) return false;
        if (!service.active) return false;

        let now = clock::timestamp_ms(clock);
        if (subscription.expires_at > now) return false;

        balance::value(&subscription.renewal_balance) >= service.monthly_fee
    }

    /// User funds their renewal balance for auto-renewal
    public entry fun fund_renewal_balance(
        subscription: &mut ProfileSubscription,
        payment: Coin<MYSO>,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        let subscriber = tx_context::sender(ctx);
        assert!(subscription.subscriber == subscriber, ENotSubscriptionOwner);

        let funded_amount = coin::value(&payment);
        balance::join(&mut subscription.renewal_balance, coin::into_balance(payment));

        event::emit(RenewalBalanceFundedEvent {
            subscription_id: object::id(subscription),
            subscriber,
            funded_amount,
            new_balance: balance::value(&subscription.renewal_balance),
            timestamp: clock::timestamp_ms(clock),
        });
    }

    /// Check if a subscription is valid for access
    public fun is_subscription_valid(
        subscription: &ProfileSubscription,
        service: &ProfileSubscriptionService,
        clock: &Clock,
    ): bool {
        if (object::id(service) != subscription.service_id) {
            return false
        };

        let now = clock::timestamp_ms(clock);
        subscription.expires_at > now
    }

    /// Update service fee (profile owner only)
    public entry fun update_service_fee(
        service: &mut ProfileSubscriptionService,
        new_fee: u64,
        ctx: &mut TxContext,
    ) {
        assert!(service.version == upgrade::current_version(), EWrongVersion);
        assert!(tx_context::sender(ctx) == service.profile_owner, ENotSubscriptionOwner);
        assert!(new_fee > 0, EInvalidFee);

        let old_fee = service.monthly_fee;
        service.monthly_fee = new_fee;

        event::emit(ProfileSubscriptionUpdatedEvent {
            service_id: object::id(service),
            old_fee,
            new_fee,
            updated_by: tx_context::sender(ctx),
        });
    }

    /// Deactivate service (profile owner only)
    public entry fun deactivate_service(
        service: &mut ProfileSubscriptionService,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(service.version == upgrade::current_version(), EWrongVersion);
        assert!(tx_context::sender(ctx) == service.profile_owner, ENotSubscriptionOwner);
        service.active = false;

        event::emit(ProfileSubscriptionServiceDeactivatedEvent {
            service_id: object::id(service),
            profile_owner: service.profile_owner,
            deactivated_at: clock::timestamp_ms(clock),
        });
    }

    /// Cancel subscription and get refund of unused renewal balance
    public entry fun cancel_subscription(
        service: &mut ProfileSubscriptionService,
        mut subscription: ProfileSubscription,
        ctx: &mut TxContext,
    ) {
        assert!(service.version == upgrade::current_version(), EWrongVersion);

        let subscriber = tx_context::sender(ctx);
        assert!(subscription.subscriber == subscriber, ENotSubscriptionOwner);
        assert!(subscription.service_id == object::id(service), ENoAccess);

        let refund_amount = balance::value(&subscription.renewal_balance);
        if (refund_amount > 0) {
            let refund = coin::from_balance(
                balance::withdraw_all(&mut subscription.renewal_balance),
                ctx
            );
            transfer::public_transfer(refund, subscriber);
        };

        assert!(service.subscriber_count > 0, EOverflow);
        service.subscriber_count = service.subscriber_count - 1;

        event::emit(ProfileSubscriptionCancelledEvent {
            subscription_id: object::id(&subscription),
            subscriber,
            refunded_amount: refund_amount,
        });

        let ProfileSubscription {
            id,
            service_id: _,
            subscriber: _,
            created_at: _,
            expires_at: _,
            auto_renew: _,
            renewal_balance,
            renewal_count: _,
        } = subscription;

        balance::destroy_zero(renewal_balance);
        object::delete(id);
    }

    // === Read-only functions ===

    public fun service_monthly_fee(service: &ProfileSubscriptionService): u64 {
        service.monthly_fee
    }

    public fun service_subscriber_count(service: &ProfileSubscriptionService): u64 {
        service.subscriber_count
    }

    public fun subscription_expires_at(subscription: &ProfileSubscription): u64 {
        subscription.expires_at
    }

    public fun subscription_auto_renew(subscription: &ProfileSubscription): bool {
        subscription.auto_renew
    }

    public fun subscription_renewal_balance(subscription: &ProfileSubscription): u64 {
        balance::value(&subscription.renewal_balance)
    }

    public(package) fun bootstrap_init(clock: &Clock, ctx: &mut TxContext) {
        let admin = tx_context::sender(ctx);
        let config = SubscriptionConfig {
            id: object::new(ctx),
            billing_period_ms: THIRTY_DAYS_MS,
            max_renewal_months: MAX_RENEWAL_MONTHS,
            platform_fee_bps: DEFAULT_PLATFORM_FEE_BPS,
            ecosystem_fee_bps: DEFAULT_ECOSYSTEM_FEE_BPS,
            non_platform_platform_to_creator_bps: DEFAULT_NON_PLATFORM_PLATFORM_TO_CREATOR_BPS,
            non_platform_platform_to_treasury_bps: DEFAULT_NON_PLATFORM_PLATFORM_TO_TREASURY_BPS,
            version: upgrade::current_version(),
        };
        emit_subscription_config_updated(
            &config,
            admin,
            clock::timestamp_ms(clock),
        );
        transfer::share_object(config);
    }

    /// Create a SubscriptionAdminCap for bootstrap (package visibility only)
    public(package) fun create_subscription_admin_cap(ctx: &mut TxContext): SubscriptionAdminCap {
        SubscriptionAdminCap {
            id: object::new(ctx),
        }
    }

    /// Update subscription configuration (admin only)
    public entry fun update_subscription_config(
        _: &SubscriptionAdminCap,
        config: &mut SubscriptionConfig,
        billing_period_ms: u64,
        max_renewal_months: u64,
        platform_fee_bps: u64,
        ecosystem_fee_bps: u64,
        non_platform_platform_to_creator_bps: u64,
        non_platform_platform_to_treasury_bps: u64,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(billing_period_ms > 0, EInvalidConfig);
        assert!(max_renewal_months > 0, EInvalidConfig);
        validate_fee_config(
            platform_fee_bps,
            ecosystem_fee_bps,
            non_platform_platform_to_creator_bps,
            non_platform_platform_to_treasury_bps,
        );

        config.billing_period_ms = billing_period_ms;
        config.max_renewal_months = max_renewal_months;
        config.platform_fee_bps = platform_fee_bps;
        config.ecosystem_fee_bps = ecosystem_fee_bps;
        config.non_platform_platform_to_creator_bps = non_platform_platform_to_creator_bps;
        config.non_platform_platform_to_treasury_bps = non_platform_platform_to_treasury_bps;

        emit_subscription_config_updated(
            config,
            tx_context::sender(ctx),
            clock::timestamp_ms(clock),
        );
    }

    /// Migration function for SubscriptionConfig
    public entry fun migrate_config(
        config: &mut SubscriptionConfig,
        _: &UpgradeAdminCap,
        ctx: &mut TxContext
    ) {
        let current_version = upgrade::current_version();
        assert!(config.version < current_version, EWrongVersion);

        let old_version = config.version;
        config.platform_fee_bps = DEFAULT_PLATFORM_FEE_BPS;
        config.ecosystem_fee_bps = DEFAULT_ECOSYSTEM_FEE_BPS;
        config.non_platform_platform_to_creator_bps = DEFAULT_NON_PLATFORM_PLATFORM_TO_CREATOR_BPS;
        config.non_platform_platform_to_treasury_bps = DEFAULT_NON_PLATFORM_PLATFORM_TO_TREASURY_BPS;
        config.version = current_version;

        upgrade::emit_migration_event(
            object::id(config),
            string::utf8(b"SubscriptionConfig"),
            old_version,
            tx_context::sender(ctx)
        );
    }

    #[test_only]
    public fun destroy_for_testing(service: ProfileSubscriptionService, subscription: ProfileSubscription) {
        let ProfileSubscriptionService { id, .. } = service;
        object::delete(id);
        let ProfileSubscription { id, renewal_balance, .. } = subscription;
        balance::destroy_zero(renewal_balance);
        object::delete(id);
    }

    #[test_only]
    public fun fee_breakdown_for_testing(config: &SubscriptionConfig, gross: u64): (u64, u64, u64) {
        calculate_subscription_fees(config, gross)
    }

    #[test_only]
    public fun create_config_for_testing(ctx: &mut TxContext): SubscriptionConfig {
        SubscriptionConfig {
            id: object::new(ctx),
            billing_period_ms: THIRTY_DAYS_MS,
            max_renewal_months: MAX_RENEWAL_MONTHS,
            platform_fee_bps: DEFAULT_PLATFORM_FEE_BPS,
            ecosystem_fee_bps: DEFAULT_ECOSYSTEM_FEE_BPS,
            non_platform_platform_to_creator_bps: DEFAULT_NON_PLATFORM_PLATFORM_TO_CREATOR_BPS,
            non_platform_platform_to_treasury_bps: DEFAULT_NON_PLATFORM_PLATFORM_TO_TREASURY_BPS,
            version: upgrade::current_version(),
        }
    }

    #[test_only]
    public fun destroy_config_for_testing(config: SubscriptionConfig) {
        let SubscriptionConfig { id, .. } = config;
        object::delete(id);
    }

    /// Migration function for ProfileSubscriptionService
    public entry fun migrate_service(
        service: &mut ProfileSubscriptionService,
        _: &UpgradeAdminCap,
        ctx: &mut TxContext
    ) {
        let current_version = upgrade::current_version();
        assert!(service.version < current_version, EWrongVersion);

        let old_version = service.version;
        service.version = current_version;

        upgrade::emit_migration_event(
            object::id(service),
            string::utf8(b"ProfileSubscriptionService"),
            old_version,
            tx_context::sender(ctx)
        );
    }
}
