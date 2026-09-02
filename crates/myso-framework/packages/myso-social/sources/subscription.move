// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

/// Subscription module for the MySocial network
/// Handles subscription services for profiles & MyData

#[allow(duplicate_alias, lint(public_entry))]
module social_contracts::subscription {
    use std::option::{Self, Option};
    use std::string;
    use std::type_name::{Self, TypeName};

    use myso::{
        object::{Self, UID, ID},
        tx_context::{Self, TxContext},
        transfer,
        clock::{Self, Clock},
        coin::{Self, Coin},
        balance::{Self, Balance},
        bag::{Self, Bag},
        event,
        table::{Self, Table},
    };
    use social_contracts::upgrade::{Self, UpgradeAdminCap};
    use social_contracts::platform::{Self, Platform};
    use social_contracts::profile::{Self, EcosystemTreasury, Profile};
    use social_contracts::block_list::{Self, BlockListRegistry};

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
    const EPlanNotFound: u64 = 85;
    const ENoActivePlans: u64 = 86;
    const ECoinTypeMismatch: u64 = 87;

    /// Default bootstrap values (used only at init)
    const MAX_RENEWAL_MONTHS: u64 = 12;
    const MAX_U64: u64 = 18446744073709551615;
    const THIRTY_DAYS_MS: u64 = 2_592_000_000;
    const BPS_DENOM: u64 = 10_000;
    const DEFAULT_PLATFORM_FEE_BPS: u64 = 250;
    const DEFAULT_ECOSYSTEM_FEE_BPS: u64 = 250;
    const DEFAULT_NON_PLATFORM_PLATFORM_TO_CREATOR_BPS: u64 = 0;
    const DEFAULT_NON_PLATFORM_PLATFORM_TO_TREASURY_BPS: u64 = 10_000;

    fun assert_subscriber_not_blocked(
        registry: &BlockListRegistry,
        subscriber: address,
        profile_owner: address,
    ) {
        block_list::assert_not_blocked(registry, subscriber, profile_owner);
    }

    /// Admin capability for subscription configuration
    public struct SubscriptionAdminCap has key, store {
        id: UID,
    }

    /// Global subscription feature configuration
    public struct SubscriptionConfig has key {
        id: UID,
        default_billing_period_ms: u64,
        max_renewal_months: u64,
        platform_fee_bps: u64,
        ecosystem_fee_bps: u64,
        non_platform_platform_to_creator_bps: u64,
        non_platform_platform_to_treasury_bps: u64,
        version: u64,
    }

    public struct SubscriptionConfigUpdatedEvent has copy, drop {
        updated_by: address,
        default_billing_period_ms: u64,
        max_renewal_months: u64,
        platform_fee_bps: u64,
        ecosystem_fee_bps: u64,
        non_platform_platform_to_creator_bps: u64,
        non_platform_platform_to_treasury_bps: u64,
        timestamp: u64,
    }

    /// Sellable plan on a profile subscription service.
    public struct SubscriptionPlan has store, drop {
        title: string::String,
        description: Option<string::String>,
        price: u64,
        duration_ms: u64,
        tier_level: Option<u64>,
        platform_id: Option<address>,
        coin_type: TypeName,
        active: bool,
        created_at: u64,
        updated_at: u64,
    }

    /// Bag key for a `Balance<T>` renewal slot on a subscription.
    public struct RenewalBalanceKey<phantom T> has copy, drop, store {}

    /// Profile subscription service - one per profile, holds multiple plans
    public struct ProfileSubscriptionService has key {
        id: UID,
        /// Profile owner who receives subscription fees
        profile_owner: address,
        /// Profile object this service belongs to
        profile_id: ID,
        /// Active and inactive plans keyed by plan id
        plans: Table<ID, SubscriptionPlan>,
        /// Number of plans ever created on this service
        plan_count: u64,
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
        /// Plan purchased at subscribe time
        plan_id: ID,
        /// Tier copied from plan at purchase time
        tier_level: Option<u64>,
        /// Platform scope copied from plan at purchase time
        platform_id: Option<address>,
        /// Subscriber's address
        subscriber: address,
        /// When the subscription was created
        created_at: u64,
        /// When the subscription expires (timestamp in ms)
        expires_at: u64,
        /// Whether auto-renewal is enabled
        auto_renew: bool,
        /// Coin type copied from the plan at purchase time
        coin_type: TypeName,
        /// Per-coin pre-funded renewal balances
        renewal_balances: Bag,
        /// Number of times this subscription has been renewed
        renewal_count: u64,
    }

    /// Events
    public struct ProfileSubscriptionCreatedEvent has copy, drop {
        subscription_id: ID,
        service_id: ID,
        plan_id: ID,
        subscriber: address,
        expires_at: u64,
        price: u64,
        duration_ms: u64,
        tier_level: Option<u64>,
        platform_id: Option<address>,
        auto_renew: bool,
        platform_fee: u64,
        ecosystem_fee: u64,
        creator_amount: u64,
        payment_platform_id: Option<address>,
        coin_type: TypeName,
    }

    public struct ProfileSubscriptionRenewedEvent has copy, drop {
        subscription_id: ID,
        subscriber: address,
        plan_id: ID,
        new_expires_at: u64,
        renewal_count: u64,
        auto_renewed: bool,
        price: u64,
        duration_ms: u64,
        tier_level: Option<u64>,
        platform_id: Option<address>,
        platform_fee: u64,
        ecosystem_fee: u64,
        creator_amount: u64,
        payment_platform_id: Option<address>,
        coin_type: TypeName,
    }

    public struct ProfileSubscriptionCancelledEvent has copy, drop {
        subscription_id: ID,
        subscriber: address,
        refunded_amount: u64,
    }

    /// Additional event for plan updates
    public struct SubscriptionPlanUpdatedEvent has copy, drop {
        service_id: ID,
        plan_id: ID,
        title: string::String,
        description: Option<string::String>,
        price: u64,
        duration_ms: u64,
        tier_level: Option<u64>,
        platform_id: Option<address>,
        coin_type: TypeName,
        active: bool,
        updated_by: address,
        updated_at: u64,
    }

    /// Event emitted when a subscription service is created
    public struct ProfileSubscriptionServiceCreatedEvent has copy, drop {
        service_id: ID,
        profile_owner: address,
        profile_id: ID,
        created_at: u64,
    }

    public struct SubscriptionPlanCreatedEvent has copy, drop {
        service_id: ID,
        plan_id: ID,
        title: string::String,
        description: Option<string::String>,
        price: u64,
        duration_ms: u64,
        tier_level: Option<u64>,
        platform_id: Option<address>,
        coin_type: TypeName,
        created_at: u64,
    }

    public struct SubscriptionPlanDeactivatedEvent has copy, drop {
        service_id: ID,
        plan_id: ID,
        deactivated_at: u64,
    }

    /// Event emitted when renewal balance is funded
    public struct RenewalBalanceFundedEvent has copy, drop {
        subscription_id: ID,
        subscriber: address,
        funded_amount: u64,
        new_balance: u64,
        coin_type: TypeName,
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

    fun route_non_platform_platform_fee<T>(
        config: &SubscriptionConfig,
        treasury: &EcosystemTreasury,
        platform_fee: u64,
        creator_amount: u64,
        payment: &mut Coin<T>,
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

    fun distribute_subscription_payment_fees_no_platform<T>(
        config: &SubscriptionConfig,
        treasury: &EcosystemTreasury,
        profile_owner: address,
        payment: Coin<T>,
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

    fun distribute_subscription_payment_fees_with_platform<T>(
        config: &SubscriptionConfig,
        treasury: &EcosystemTreasury,
        profile_owner: address,
        payment: Coin<T>,
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
            platform::fund_platform_treasury_from_coin(platform, &mut platform_coin, platform_fee, clock, ctx);
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
            default_billing_period_ms: config.default_billing_period_ms,
            max_renewal_months: config.max_renewal_months,
            platform_fee_bps: config.platform_fee_bps,
            ecosystem_fee_bps: config.ecosystem_fee_bps,
            non_platform_platform_to_creator_bps: config.non_platform_platform_to_creator_bps,
            non_platform_platform_to_treasury_bps: config.non_platform_platform_to_treasury_bps,
            timestamp,
        });
    }

    fun new_plan_id(ctx: &mut TxContext): ID {
        let id = object::new(ctx);
        let plan_id = object::uid_to_inner(&id);
        object::delete(id);
        plan_id
    }

    fun effective_tier_level(tier_level: Option<u64>): u64 {
        if (option::is_some(&tier_level)) {
            *option::borrow(&tier_level)
        } else {
            0
        }
    }

    fun tier_satisfies(subscription_tier: Option<u64>, min_tier: Option<u64>): bool {
        if (option::is_none(&min_tier)) {
            true
        } else {
            effective_tier_level(subscription_tier) >= *option::borrow(&min_tier)
        }
    }

    fun platform_satisfies(
        subscription_platform: Option<address>,
        content_platform_id: Option<address>,
    ): bool {
        if (option::is_none(&subscription_platform)) {
            true
        } else if (option::is_none(&content_platform_id)) {
            false
        } else {
            *option::borrow(&subscription_platform) == *option::borrow(&content_platform_id)
        }
    }

    fun borrow_active_plan(service: &ProfileSubscriptionService, plan_id: ID): &SubscriptionPlan {
        assert!(table::contains(&service.plans, plan_id), EPlanNotFound);
        let plan = table::borrow(&service.plans, plan_id);
        assert!(plan.active, EPlanNotFound);
        plan
    }

    fun borrow_plan_for_renewal(
        service: &ProfileSubscriptionService,
        plan_id: ID,
    ): &SubscriptionPlan {
        assert!(table::contains(&service.plans, plan_id), EPlanNotFound);
        table::borrow(&service.plans, plan_id)
    }

    fun assert_plan_coin_type<T>(plan: &SubscriptionPlan) {
        assert!(plan.coin_type == type_name::with_defining_ids<T>(), ECoinTypeMismatch);
    }

    fun assert_subscription_coin_type<T>(subscription: &ProfileSubscription) {
        assert!(subscription.coin_type == type_name::with_defining_ids<T>(), ECoinTypeMismatch);
    }

    fun renewal_balance_value<T>(subscription: &ProfileSubscription): u64 {
        let key = RenewalBalanceKey<T> {};
        if (!bag::contains_with_type<RenewalBalanceKey<T>, Balance<T>>(&subscription.renewal_balances, key)) {
            return 0
        };
        let slot: &Balance<T> = bag::borrow(&subscription.renewal_balances, key);
        balance::value(slot)
    }

    fun join_renewal_balance<T>(subscription: &mut ProfileSubscription, incoming: Balance<T>) {
        if (balance::value(&incoming) == 0) {
            balance::destroy_zero(incoming);
            return
        };
        let key = RenewalBalanceKey<T> {};
        if (bag::contains(&subscription.renewal_balances, key)) {
            let slot: &mut Balance<T> = bag::borrow_mut(&mut subscription.renewal_balances, key);
            balance::join(slot, incoming);
        } else {
            bag::add(&mut subscription.renewal_balances, key, incoming);
        };
    }

    fun split_renewal_balance<T>(subscription: &mut ProfileSubscription, amount: u64): Balance<T> {
        let key = RenewalBalanceKey<T> {};
        assert!(
            bag::contains_with_type<RenewalBalanceKey<T>, Balance<T>>(&subscription.renewal_balances, key),
            EInvalidFee,
        );
        let slot: &mut Balance<T> = bag::borrow_mut(&mut subscription.renewal_balances, key);
        balance::split(slot, amount)
    }

    /// Create a subscription service container for a profile (called by profile owner)
    public fun create_profile_service(
        profile_owner: address,
        profile_id: ID,
        ctx: &mut TxContext,
    ): ProfileSubscriptionService {
        ProfileSubscriptionService {
            id: object::new(ctx),
            profile_owner,
            profile_id,
            plans: table::new(ctx),
            plan_count: 0,
            active: true,
            subscriber_count: 0,
            version: upgrade::current_version(),
        }
    }

    /// Entry function to create and share a profile subscription service
    public entry fun create_profile_service_entry(
        profile: &Profile,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(tx_context::sender(ctx) == profile::get_owner(profile), ENoAccess);

        let profile_owner = profile::get_owner(profile);
        let profile_id = object::id(profile);
        let service = create_profile_service(profile_owner, profile_id, ctx);
        let service_id = object::id(&service);

        transfer::share_object(service);

        event::emit(ProfileSubscriptionServiceCreatedEvent {
            service_id,
            profile_owner,
            profile_id,
            created_at: clock::timestamp_ms(clock),
        });
    }

    fun resolve_plan_duration_ms(config: &SubscriptionConfig, duration_ms: u64): u64 {
        let resolved_duration_ms = if (duration_ms == 0) {
            config.default_billing_period_ms
        } else {
            duration_ms
        };
        assert!(resolved_duration_ms > 0, EInvalidInput);
        resolved_duration_ms
    }

    /// Create a sellable plan on a profile subscription service (profile owner only).
    public entry fun create_subscription_plan<T>(
        config: &SubscriptionConfig,
        service: &mut ProfileSubscriptionService,
        title: string::String,
        description: Option<string::String>,
        price: u64,
        duration_ms: u64,
        tier_level: Option<u64>,
        platform_id: Option<address>,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(service.version == upgrade::current_version(), EWrongVersion);
        assert!(tx_context::sender(ctx) == service.profile_owner, ENotSubscriptionOwner);
        assert!(price > 0, EInvalidFee);
        let resolved_duration_ms = resolve_plan_duration_ms(config, duration_ms);
        let coin_type = type_name::with_defining_ids<T>();

        let now = clock::timestamp_ms(clock);
        let plan_id = new_plan_id(ctx);
        let service_id = object::id(service);
        let plan = SubscriptionPlan {
            title,
            description,
            price,
            duration_ms: resolved_duration_ms,
            tier_level,
            platform_id,
            coin_type,
            active: true,
            created_at: now,
            updated_at: now,
        };

        table::add(&mut service.plans, plan_id, plan);
        assert!(service.plan_count <= MAX_U64 - 1, EOverflow);
        service.plan_count = service.plan_count + 1;

        event::emit(SubscriptionPlanCreatedEvent {
            service_id,
            plan_id,
            title: table::borrow(&service.plans, plan_id).title,
            description: table::borrow(&service.plans, plan_id).description,
            price,
            duration_ms: resolved_duration_ms,
            tier_level,
            platform_id,
            coin_type,
            created_at: now,
        });
    }

    /// Update an existing plan (profile owner only).
    public entry fun update_subscription_plan(
        config: &SubscriptionConfig,
        service: &mut ProfileSubscriptionService,
        plan_id: ID,
        title: string::String,
        description: Option<string::String>,
        price: u64,
        duration_ms: u64,
        tier_level: Option<u64>,
        platform_id: Option<address>,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(service.version == upgrade::current_version(), EWrongVersion);
        assert!(tx_context::sender(ctx) == service.profile_owner, ENotSubscriptionOwner);
        assert!(table::contains(&service.plans, plan_id), EPlanNotFound);
        assert!(price > 0, EInvalidFee);
        let resolved_duration_ms = resolve_plan_duration_ms(config, duration_ms);

        let service_id = object::id(service);
        let updated_by = tx_context::sender(ctx);
        {
            let plan = table::borrow_mut(&mut service.plans, plan_id);
            plan.title = title;
            plan.description = description;
            plan.price = price;
            plan.duration_ms = resolved_duration_ms;
            plan.tier_level = tier_level;
            plan.platform_id = platform_id;
            plan.updated_at = clock::timestamp_ms(clock);
        };

        let plan = table::borrow(&service.plans, plan_id);
        event::emit(SubscriptionPlanUpdatedEvent {
            service_id,
            plan_id,
            title: plan.title,
            description: plan.description,
            price: plan.price,
            duration_ms: plan.duration_ms,
            tier_level: plan.tier_level,
            platform_id: plan.platform_id,
            coin_type: plan.coin_type,
            active: plan.active,
            updated_by,
            updated_at: plan.updated_at,
        });
    }

    /// Deactivate a plan so it no longer accepts new subscriptions (profile owner only).
    public entry fun deactivate_subscription_plan(
        service: &mut ProfileSubscriptionService,
        plan_id: ID,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(service.version == upgrade::current_version(), EWrongVersion);
        assert!(tx_context::sender(ctx) == service.profile_owner, ENotSubscriptionOwner);
        assert!(table::contains(&service.plans, plan_id), EPlanNotFound);

        let service_id = object::id(service);
        {
            let plan = table::borrow_mut(&mut service.plans, plan_id);
            plan.active = false;
            plan.updated_at = clock::timestamp_ms(clock);
        };

        event::emit(SubscriptionPlanDeactivatedEvent {
            service_id,
            plan_id,
            deactivated_at: table::borrow(&service.plans, plan_id).updated_at,
        });
    }

    fun subscribe_to_profile_internal_no_platform<T>(
        block_list_registry: &BlockListRegistry,
        config: &SubscriptionConfig,
        service: &mut ProfileSubscriptionService,
        plan_id: ID,
        treasury: &EcosystemTreasury,
        payment: &mut Coin<T>,
        auto_renew: bool,
        renewal_periods: u64,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(service.version == upgrade::current_version(), EWrongVersion);
        assert!(service.active, ENoAccess);
        assert!(service.plan_count > 0, ENoActivePlans);

        if (auto_renew) {
            assert!(renewal_periods <= config.max_renewal_months, EInvalidInput);
        };

        let plan = borrow_active_plan(service, plan_id);
        assert_plan_coin_type<T>(plan);
        let plan_price = plan.price;
        let plan_duration_ms = plan.duration_ms;
        let plan_tier_level = plan.tier_level;
        let plan_platform_id = plan.platform_id;

        let subscriber = tx_context::sender(ctx);
        assert_subscriber_not_blocked(block_list_registry, subscriber, service.profile_owner);
        let now = clock::timestamp_ms(clock);

        let periods_to_pay = if (auto_renew) { 1 + renewal_periods } else { 1 };
        assert!(periods_to_pay <= MAX_U64 / plan_price, EOverflow);
        let total_required = plan_price * periods_to_pay;
        assert!(coin::value(payment) >= total_required, EInvalidFee);

        let first_period_payment = coin::split(payment, plan_price, ctx);
        let (platform_fee, ecosystem_fee, creator_amount) =
            distribute_subscription_payment_fees_no_platform(
                config,
                treasury,
                service.profile_owner,
                first_period_payment,
                ctx,
            );

        finish_subscribe(
            service,
            plan_id,
            plan_price,
            plan_duration_ms,
            plan_tier_level,
            plan_platform_id,
            payment,
            auto_renew,
            renewal_periods,
            subscriber,
            now,
            platform_fee,
            ecosystem_fee,
            creator_amount,
            option::none(),
            ctx,
        );
    }

    fun subscribe_to_profile_internal_with_platform<T>(
        block_list_registry: &BlockListRegistry,
        config: &SubscriptionConfig,
        service: &mut ProfileSubscriptionService,
        plan_id: ID,
        treasury: &EcosystemTreasury,
        platform: &mut Platform,
        payment: &mut Coin<T>,
        auto_renew: bool,
        renewal_periods: u64,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(service.version == upgrade::current_version(), EWrongVersion);
        assert!(service.active, ENoAccess);
        assert!(service.plan_count > 0, ENoActivePlans);

        if (auto_renew) {
            assert!(renewal_periods <= config.max_renewal_months, EInvalidInput);
        };

        let plan = borrow_active_plan(service, plan_id);
        assert_plan_coin_type<T>(plan);
        let plan_price = plan.price;
        let plan_duration_ms = plan.duration_ms;
        let plan_tier_level = plan.tier_level;
        let plan_platform_id = plan.platform_id;

        let subscriber = tx_context::sender(ctx);
        assert_subscriber_not_blocked(block_list_registry, subscriber, service.profile_owner);
        let now = clock::timestamp_ms(clock);

        let periods_to_pay = if (auto_renew) { 1 + renewal_periods } else { 1 };
        assert!(periods_to_pay <= MAX_U64 / plan_price, EOverflow);
        let total_required = plan_price * periods_to_pay;
        assert!(coin::value(payment) >= total_required, EInvalidFee);

        let first_period_payment = coin::split(payment, plan_price, ctx);
        let (platform_fee, ecosystem_fee, creator_amount) =
            distribute_subscription_payment_fees_with_platform(
                config,
                treasury,
                service.profile_owner,
                first_period_payment,
                platform,
                clock,
                ctx,
            );

        finish_subscribe(
            service,
            plan_id,
            plan_price,
            plan_duration_ms,
            plan_tier_level,
            plan_platform_id,
            payment,
            auto_renew,
            renewal_periods,
            subscriber,
            now,
            platform_fee,
            ecosystem_fee,
            creator_amount,
            option::some(object::uid_to_address(platform::id(platform))),
            ctx,
        );
    }

    fun finish_subscribe<T>(
        service: &mut ProfileSubscriptionService,
        plan_id: ID,
        plan_price: u64,
        plan_duration_ms: u64,
        plan_tier_level: Option<u64>,
        plan_platform_id: Option<address>,
        payment: &mut Coin<T>,
        auto_renew: bool,
        renewal_periods: u64,
        subscriber: address,
        now: u64,
        platform_fee: u64,
        ecosystem_fee: u64,
        creator_amount: u64,
        payment_platform_id: Option<address>,
        ctx: &mut TxContext,
    ) {
        let mut renewal_balances = bag::new(ctx);
        if (auto_renew && renewal_periods > 0) {
            assert!(renewal_periods <= MAX_U64 / plan_price, EOverflow);
            let renewal_amount = plan_price * renewal_periods;
            let incoming = coin::into_balance(coin::split(payment, renewal_amount, ctx));
            bag::add(&mut renewal_balances, RenewalBalanceKey<T> {}, incoming);
        };

        assert!(now <= MAX_U64 - plan_duration_ms, EOverflow);
        let expires_at = now + plan_duration_ms;
        let coin_type = type_name::with_defining_ids<T>();

        let subscription = ProfileSubscription {
            id: object::new(ctx),
            service_id: object::id(service),
            plan_id,
            tier_level: plan_tier_level,
            platform_id: plan_platform_id,
            subscriber,
            created_at: now,
            expires_at,
            auto_renew,
            coin_type,
            renewal_balances,
            renewal_count: 0,
        };

        assert!(service.subscriber_count <= MAX_U64 - 1, EOverflow);
        service.subscriber_count = service.subscriber_count + 1;

        let subscription_id = object::id(&subscription);
        event::emit(ProfileSubscriptionCreatedEvent {
            subscription_id,
            service_id: object::id(service),
            plan_id,
            subscriber,
            expires_at,
            price: plan_price,
            duration_ms: plan_duration_ms,
            tier_level: plan_tier_level,
            platform_id: plan_platform_id,
            auto_renew,
            platform_fee,
            ecosystem_fee,
            creator_amount,
            payment_platform_id,
            coin_type,
        });

        transfer::transfer(subscription, subscriber);
    }

    /// Subscribe to a profile plan with optional auto-renewal (no platform fee recipient).
    public entry fun subscribe_to_profile<T>(
        block_list_registry: &BlockListRegistry,
        config: &SubscriptionConfig,
        service: &mut ProfileSubscriptionService,
        plan_id: ID,
        treasury: &EcosystemTreasury,
        payment: &mut Coin<T>,
        auto_renew: bool,
        renewal_periods: u64,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        subscribe_to_profile_internal_no_platform(
            block_list_registry,
            config,
            service,
            plan_id,
            treasury,
            payment,
            auto_renew,
            renewal_periods,
            clock,
            ctx,
        );
    }

    /// Subscribe to a profile plan with platform treasury routing for the platform-fee slice.
    public entry fun subscribe_to_profile_with_platform<T>(
        block_list_registry: &BlockListRegistry,
        config: &SubscriptionConfig,
        service: &mut ProfileSubscriptionService,
        plan_id: ID,
        treasury: &EcosystemTreasury,
        platform: &mut Platform,
        payment: &mut Coin<T>,
        auto_renew: bool,
        renewal_periods: u64,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        subscribe_to_profile_internal_with_platform(
            block_list_registry,
            config,
            service,
            plan_id,
            treasury,
            platform,
            payment,
            auto_renew,
            renewal_periods,
            clock,
            ctx,
        );
    }

    fun renew_subscription_internal_no_platform<T>(
        block_list_registry: &BlockListRegistry,
        config: &SubscriptionConfig,
        service: &ProfileSubscriptionService,
        subscription: &mut ProfileSubscription,
        payment: Coin<T>,
        treasury: &EcosystemTreasury,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(service.version == upgrade::current_version(), EWrongVersion);

        let subscriber = tx_context::sender(ctx);
        assert!(subscription.subscriber == subscriber, ENotSubscriptionOwner);
        assert!(subscription.service_id == object::id(service), ENoAccess);
        assert_subscriber_not_blocked(block_list_registry, subscriber, service.profile_owner);

        let plan = borrow_plan_for_renewal(service, subscription.plan_id);
        assert_plan_coin_type<T>(plan);
        assert_subscription_coin_type<T>(subscription);
        assert!(coin::value(&payment) >= plan.price, EInvalidFee);

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
            service,
            platform_fee,
            ecosystem_fee,
            creator_amount,
            option::none(),
            false,
            clock,
        );
    }

    fun renew_subscription_internal_with_platform<T>(
        block_list_registry: &BlockListRegistry,
        config: &SubscriptionConfig,
        service: &ProfileSubscriptionService,
        subscription: &mut ProfileSubscription,
        payment: Coin<T>,
        treasury: &EcosystemTreasury,
        platform: &mut Platform,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(service.version == upgrade::current_version(), EWrongVersion);

        let subscriber = tx_context::sender(ctx);
        assert!(subscription.subscriber == subscriber, ENotSubscriptionOwner);
        assert!(subscription.service_id == object::id(service), ENoAccess);
        assert_subscriber_not_blocked(block_list_registry, subscriber, service.profile_owner);

        let plan = borrow_plan_for_renewal(service, subscription.plan_id);
        assert_plan_coin_type<T>(plan);
        assert_subscription_coin_type<T>(subscription);
        assert!(coin::value(&payment) >= plan.price, EInvalidFee);

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
            service,
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
        service: &ProfileSubscriptionService,
        platform_fee: u64,
        ecosystem_fee: u64,
        creator_amount: u64,
        payment_platform_id: Option<address>,
        auto_renewed: bool,
        clock: &Clock,
    ) {
        let plan = borrow_plan_for_renewal(service, subscription.plan_id);
        let extension = plan.duration_ms;
        let now = clock::timestamp_ms(clock);

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
            plan_id: subscription.plan_id,
            new_expires_at: subscription.expires_at,
            renewal_count: subscription.renewal_count,
            auto_renewed,
            price: plan.price,
            duration_ms: plan.duration_ms,
            tier_level: subscription.tier_level,
            platform_id: subscription.platform_id,
            platform_fee,
            ecosystem_fee,
            creator_amount,
            payment_platform_id,
            coin_type: subscription.coin_type,
        });
    }

    /// Manually renew a subscription (no platform).
    public entry fun renew_subscription<T>(
        block_list_registry: &BlockListRegistry,
        config: &SubscriptionConfig,
        service: &ProfileSubscriptionService,
        subscription: &mut ProfileSubscription,
        treasury: &EcosystemTreasury,
        payment: Coin<T>,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        renew_subscription_internal_no_platform(
            block_list_registry,
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
    public entry fun renew_subscription_with_platform<T>(
        block_list_registry: &BlockListRegistry,
        config: &SubscriptionConfig,
        service: &ProfileSubscriptionService,
        subscription: &mut ProfileSubscription,
        treasury: &EcosystemTreasury,
        platform: &mut Platform,
        payment: Coin<T>,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        renew_subscription_internal_with_platform(
            block_list_registry,
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

    fun auto_renew_subscription_internal_no_platform<T>(
        block_list_registry: &BlockListRegistry,
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
        assert_subscriber_not_blocked(
            block_list_registry,
            subscription.subscriber,
            service.profile_owner,
        );

        let plan = borrow_plan_for_renewal(service, subscription.plan_id);
        assert_plan_coin_type<T>(plan);
        assert_subscription_coin_type<T>(subscription);
        let plan_price = plan.price;

        let now = clock::timestamp_ms(clock);
        assert!(subscription.expires_at <= now, ESubscriptionExpired);

        let renewal_balance_value = renewal_balance_value<T>(subscription);
        if (renewal_balance_value < plan_price) {
            subscription.auto_renew = false;
            event::emit(ProfileSubscriptionCancelledEvent {
                subscription_id: object::id(subscription),
                subscriber: subscription.subscriber,
                refunded_amount: 0,
            });
            return
        };

        let renewal_payment = coin::from_balance(split_renewal_balance<T>(subscription, plan_price), ctx);
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
            service,
            platform_fee,
            ecosystem_fee,
            creator_amount,
            option::none(),
            true,
            clock,
        );
    }

    fun auto_renew_subscription_internal_with_platform<T>(
        block_list_registry: &BlockListRegistry,
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
        assert_subscriber_not_blocked(
            block_list_registry,
            subscription.subscriber,
            service.profile_owner,
        );

        let plan = borrow_plan_for_renewal(service, subscription.plan_id);
        assert_plan_coin_type<T>(plan);
        assert_subscription_coin_type<T>(subscription);
        let plan_price = plan.price;

        let now = clock::timestamp_ms(clock);
        assert!(subscription.expires_at <= now, ESubscriptionExpired);

        let renewal_balance_value = renewal_balance_value<T>(subscription);
        if (renewal_balance_value < plan_price) {
            subscription.auto_renew = false;
            event::emit(ProfileSubscriptionCancelledEvent {
                subscription_id: object::id(subscription),
                subscriber: subscription.subscriber,
                refunded_amount: 0,
            });
            return
        };

        let renewal_payment = coin::from_balance(split_renewal_balance<T>(subscription, plan_price), ctx);
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
            service,
            platform_fee,
            ecosystem_fee,
            creator_amount,
            option::some(object::uid_to_address(platform::id(platform))),
            true,
            clock,
        );
    }

    /// Gas-optimized auto-renew using pre-funded renewal balance (no platform).
    public entry fun auto_renew_subscription<T>(
        block_list_registry: &BlockListRegistry,
        config: &SubscriptionConfig,
        service: &ProfileSubscriptionService,
        subscription: &mut ProfileSubscription,
        treasury: &EcosystemTreasury,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        auto_renew_subscription_internal_no_platform<T>(
            block_list_registry,
            config,
            service,
            subscription,
            treasury,
            clock,
            ctx,
        );
    }

    /// Gas-optimized auto-renew with platform treasury routing.
    public entry fun auto_renew_subscription_with_platform<T>(
        block_list_registry: &BlockListRegistry,
        config: &SubscriptionConfig,
        service: &ProfileSubscriptionService,
        subscription: &mut ProfileSubscription,
        treasury: &EcosystemTreasury,
        platform: &mut Platform,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        auto_renew_subscription_internal_with_platform<T>(
            block_list_registry,
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
    public fun can_auto_renew<T>(
        subscription: &ProfileSubscription,
        service: &ProfileSubscriptionService,
        clock: &Clock,
    ): bool {
        if (!subscription.auto_renew) return false;
        if (subscription.service_id != object::id(service)) return false;
        if (!service.active) return false;
        if (!table::contains(&service.plans, subscription.plan_id)) return false;
        if (subscription.coin_type != type_name::with_defining_ids<T>()) return false;

        let now = clock::timestamp_ms(clock);
        if (subscription.expires_at > now) return false;

        let plan = table::borrow(&service.plans, subscription.plan_id);
        if (plan.coin_type != type_name::with_defining_ids<T>()) return false;
        renewal_balance_value<T>(subscription) >= plan.price
    }

    /// User funds their renewal balance for auto-renewal
    public entry fun fund_renewal_balance<T>(
        block_list_registry: &BlockListRegistry,
        service: &ProfileSubscriptionService,
        subscription: &mut ProfileSubscription,
        payment: Coin<T>,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        let subscriber = tx_context::sender(ctx);
        assert!(subscription.subscriber == subscriber, ENotSubscriptionOwner);
        assert!(subscription.service_id == object::id(service), ENoAccess);
        assert_subscription_coin_type<T>(subscription);
        assert_subscriber_not_blocked(block_list_registry, subscriber, service.profile_owner);

        let funded_amount = coin::value(&payment);
        join_renewal_balance(subscription, coin::into_balance(payment));

        event::emit(RenewalBalanceFundedEvent {
            subscription_id: object::id(subscription),
            subscriber,
            funded_amount,
            new_balance: renewal_balance_value<T>(subscription),
            coin_type: type_name::with_defining_ids<T>(),
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

    /// Profile owner for a subscription service (for cross-module gate checks).
    public fun service_profile_owner(service: &ProfileSubscriptionService): address {
        service.profile_owner
    }

    /// Whether the service accepts new subscriptions.
    public fun service_is_active(service: &ProfileSubscriptionService): bool {
        service.active
    }

    /// Whether `subscriber` holds a valid subscription to `service` at `clock`.
    public fun is_subscription_valid_for(
        subscription: &ProfileSubscription,
        service: &ProfileSubscriptionService,
        subscriber: address,
        clock: &Clock,
    ): bool {
        subscription.subscriber == subscriber
            && is_subscription_valid(subscription, service, clock)
    }

    /// Whether subscription satisfies optional tier and platform constraints for content access.
    public fun subscription_satisfies_access(
        subscription: &ProfileSubscription,
        service: &ProfileSubscriptionService,
        subscriber: address,
        min_tier_level: Option<u64>,
        content_platform_id: Option<address>,
        clock: &Clock,
    ): bool {
        if (!is_subscription_valid_for(subscription, service, subscriber, clock)) {
            return false
        };
        if (!tier_satisfies(subscription.tier_level, min_tier_level)) {
            return false
        };
        platform_satisfies(subscription.platform_id, content_platform_id)
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
    public entry fun cancel_subscription<T>(
        service: &mut ProfileSubscriptionService,
        mut subscription: ProfileSubscription,
        ctx: &mut TxContext,
    ) {
        assert!(service.version == upgrade::current_version(), EWrongVersion);

        let subscriber = tx_context::sender(ctx);
        assert!(subscription.subscriber == subscriber, ENotSubscriptionOwner);
        assert!(subscription.service_id == object::id(service), ENoAccess);
        assert_subscription_coin_type<T>(&subscription);

        let refund_amount = renewal_balance_value<T>(&subscription);
        if (refund_amount > 0) {
            let key = RenewalBalanceKey<T> {};
            let stored = bag::remove<RenewalBalanceKey<T>, Balance<T>>(&mut subscription.renewal_balances, key);
            transfer::public_transfer(coin::from_balance(stored, ctx), subscriber);
        } else if (bag::contains_with_type<RenewalBalanceKey<T>, Balance<T>>(
            &subscription.renewal_balances,
            RenewalBalanceKey<T> {},
        )) {
            let stored = bag::remove<RenewalBalanceKey<T>, Balance<T>>(
                &mut subscription.renewal_balances,
                RenewalBalanceKey<T> {},
            );
            balance::destroy_zero(stored);
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
            plan_id: _,
            tier_level: _,
            platform_id: _,
            subscriber: _,
            created_at: _,
            expires_at: _,
            auto_renew: _,
            coin_type: _,
            renewal_balances,
            renewal_count: _,
        } = subscription;

        bag::destroy_empty(renewal_balances);
        object::delete(id);
    }

    // === Read-only functions ===

    public fun service_profile_id(service: &ProfileSubscriptionService): ID {
        service.profile_id
    }

    public fun service_plan_count(service: &ProfileSubscriptionService): u64 {
        service.plan_count
    }

    public fun subscription_plan_id(subscription: &ProfileSubscription): ID {
        subscription.plan_id
    }

    public fun subscription_tier_level(subscription: &ProfileSubscription): Option<u64> {
        subscription.tier_level
    }

    public fun subscription_platform_id(subscription: &ProfileSubscription): Option<address> {
        subscription.platform_id
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

    public fun subscription_renewal_balance<T>(subscription: &ProfileSubscription): u64 {
        renewal_balance_value<T>(subscription)
    }

    public fun subscription_coin_type(subscription: &ProfileSubscription): TypeName {
        subscription.coin_type
    }

    public fun plan_coin_type(plan: &SubscriptionPlan): TypeName {
        plan.coin_type
    }

    public(package) fun bootstrap_init(clock: &Clock, ctx: &mut TxContext) {
        let admin = tx_context::sender(ctx);
        let config = SubscriptionConfig {
            id: object::new(ctx),
            default_billing_period_ms: THIRTY_DAYS_MS,
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
        default_billing_period_ms: u64,
        max_renewal_months: u64,
        platform_fee_bps: u64,
        ecosystem_fee_bps: u64,
        non_platform_platform_to_creator_bps: u64,
        non_platform_platform_to_treasury_bps: u64,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(default_billing_period_ms > 0, EInvalidConfig);
        assert!(max_renewal_months > 0, EInvalidConfig);
        validate_fee_config(
            platform_fee_bps,
            ecosystem_fee_bps,
            non_platform_platform_to_creator_bps,
            non_platform_platform_to_treasury_bps,
        );

        config.default_billing_period_ms = default_billing_period_ms;
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
        let ProfileSubscriptionService { id, plans, .. } = service;
        table::destroy_empty(plans);
        object::delete(id);
        let ProfileSubscription { id, renewal_balances, .. } = subscription;
        bag::destroy_empty(renewal_balances);
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
            default_billing_period_ms: THIRTY_DAYS_MS,
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

    #[test_only]
    public fun resolve_plan_duration_ms_for_testing(
        config: &SubscriptionConfig,
        duration_ms: u64,
    ): u64 {
        resolve_plan_duration_ms(config, duration_ms)
    }

    #[test_only]
    public fun test_init(clock: &Clock, ctx: &mut TxContext) {
        bootstrap_init(clock, ctx);
    }

    #[test_only]
    public fun test_share_empty_service(
        profile_owner: address,
        profile_id: ID,
        ctx: &mut TxContext,
    ) {
        transfer::share_object(create_profile_service(profile_owner, profile_id, ctx));
    }

    #[test_only]
    public struct TestPlanRef has key, store {
        id: UID,
        plan_id: ID,
    }

    #[test_only]
    public fun test_share_service_with_plan(
        profile_owner: address,
        profile_id: ID,
        plan_recipient: address,
        price: u64,
        duration_ms: u64,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        let mut service = create_profile_service(profile_owner, profile_id, ctx);
        let plan_id = test_create_plan(
            &mut service,
            b"Basic",
            price,
            duration_ms,
            option::none(),
            option::none(),
            clock,
            ctx,
        );
        transfer::share_object(service);
        transfer::transfer(
            TestPlanRef {
                id: object::new(ctx),
                plan_id,
            },
            plan_recipient,
        );
    }

    #[test_only]
    public fun test_take_plan_id(plan_ref: TestPlanRef): ID {
        let TestPlanRef { id, plan_id } = plan_ref;
        object::delete(id);
        plan_id
    }

    #[test_only]
    public fun test_create_plan(
        service: &mut ProfileSubscriptionService,
        title: vector<u8>,
        price: u64,
        duration_ms: u64,
        tier_level: Option<u64>,
        platform_id: Option<address>,
        clock: &Clock,
        ctx: &mut TxContext,
    ): ID {
        let plan_id = new_plan_id(ctx);
        let now = clock::timestamp_ms(clock);
        let plan = SubscriptionPlan {
            title: string::utf8(title),
            description: option::none(),
            price,
            duration_ms,
            tier_level,
            platform_id,
            coin_type: type_name::with_defining_ids<myso::myso::MYSO>(),
            active: true,
            created_at: now,
            updated_at: now,
        };
        table::add(&mut service.plans, plan_id, plan);
        service.plan_count = service.plan_count + 1;
        plan_id
    }

    #[test_only]
    public fun test_create_plan_with_coin<T>(
        service: &mut ProfileSubscriptionService,
        title: vector<u8>,
        price: u64,
        duration_ms: u64,
        tier_level: Option<u64>,
        platform_id: Option<address>,
        clock: &Clock,
        ctx: &mut TxContext,
    ): ID {
        let plan_id = new_plan_id(ctx);
        let now = clock::timestamp_ms(clock);
        let plan = SubscriptionPlan {
            title: string::utf8(title),
            description: option::none(),
            price,
            duration_ms,
            tier_level,
            platform_id,
            coin_type: type_name::with_defining_ids<T>(),
            active: true,
            created_at: now,
            updated_at: now,
        };
        table::add(&mut service.plans, plan_id, plan);
        service.plan_count = service.plan_count + 1;
        plan_id
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
