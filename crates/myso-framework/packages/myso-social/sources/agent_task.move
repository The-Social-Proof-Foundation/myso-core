// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

/// Agent task market: escrowed offers between a buyer and a named seller agent.
///
/// ```
/// create_task ─► OFFERED ─┬─ accept_task ─► ACCEPTED ─┬─ mark_delivered ─► DELIVERED ─┬─ settle_task ──┐
///                         ├─ cancel_offer ─► CANCELLED │                               ├─ release_task ─┴─► SETTLED
///                         ├─ decline_task ─► CANCELLED └─ refund_task ─► REFUNDED       └─ propose/accept_refund ─► REFUNDED
///                         └─ expire_offer ─► CANCELLED
/// SETTLED ─► review_seller / review_buyer_agent / review_buyer_principal (once per direction)
/// ```
///
/// Every state change asserts the expected `status` first, so settle, release and the refunds
/// race on one shared object and exactly one wins. There is **no unilateral buyer refund after
/// delivery**: the buyer settles (pays), a refund needs both parties, and an unattended delivered
/// task is released to the seller after `settle_window + release_grace`.
///
/// Fees, windows and the payout split are frozen into the offer, so a later `set_task_config`
/// never changes a pending or accepted task. Reviews feed `memory`'s bidirectional reputation.
#[allow(duplicate_alias, lint(public_entry), unused_const)]
module social_contracts::agent_task {
    use std::hash;
    use std::option::{Self, Option};
    use std::string::{Self, String};
    use std::vector;

    use myso::{
        balance::{Self, Balance},
        bcs,
        clock::{Self, Clock},
        coin::{Self, Coin},
        event,
        myso::MYSO,
        object::{Self, UID, ID},
        transfer,
        tx_context::{Self, TxContext},
    };

    use social_contracts::memory::{Self, ActingContext, MemoryAccount, MemoryConfig, ReputationParams, SubAgent};
    use social_contracts::platform::{Self, Platform};
    use social_contracts::upgrade::{Self, UpgradeAdminCap};

    // ============================================================
    // Status, kinds, roles
    // ============================================================

    const STATUS_OFFERED: u8 = 0;
    const STATUS_ACCEPTED: u8 = 1;
    const STATUS_DELIVERED: u8 = 2;
    const STATUS_SETTLED: u8 = 3;
    const STATUS_REFUNDED: u8 = 4;
    const STATUS_CANCELLED: u8 = 5;

    /// `close_kind` of a cancelled offer.
    const CLOSE_BUYER_CANCELLED: u8 = 0;
    const CLOSE_SELLER_DECLINED: u8 = 1;
    const CLOSE_EXPIRED: u8 = 2;

    const SETTLED_BY_BUYER: u8 = 1;
    const SETTLED_AUTO_RELEASED: u8 = 2;

    /// `refund_kind`: the seller never delivered / both parties agreed after delivery.
    const REFUND_DELIVERY_MISSED: u8 = 0;
    const REFUND_MUTUAL: u8 = 1;

    const PARTY_BUYER: u8 = 1;
    const PARTY_SELLER: u8 = 2;

    const CLEARED_WITHDRAWN: u8 = 0;
    const CLEARED_REJECTED: u8 = 1;

    const DIGEST_LENGTH: u64 = 32;

    // ============================================================
    // Errors
    // ============================================================

    const EWrongVersion: u64 = 1;
    const EMarketPaused: u64 = 2;
    const EWrongStatus: u64 = 3;
    const ETaskSelfDealing: u64 = 4;
    const ENotBuyer: u64 = 5;
    const ENotSeller: u64 = 6;
    const EAcceptExpired: u64 = 7;
    const ETermsMismatch: u64 = 8;
    const EWrongTaskObject: u64 = 9;
    const EDeliveryDeadlinePassed: u64 = 10;
    const ETooEarly: u64 = 11;
    const EInvalidDigest: u64 = 12;
    const ETitleTooLong: u64 = 13;
    const EUriTooLong: u64 = 14;
    const EEscrowTooSmall: u64 = 15;
    const EInvalidWindow: u64 = 16;
    const EInsufficientPayment: u64 = 17;
    const EInvalidStars: u64 = 18;
    const EReviewWindowClosed: u64 = 19;
    const EAlreadyReviewed: u64 = 20;
    const EWrongReviewTarget: u64 = 21;
    const EProposalPending: u64 = 22;
    const ENoProposal: u64 = 23;
    const ENotCounterparty: u64 = 24;
    const ENotProposer: u64 = 25;
    const EInvalidTaskConfig: u64 = 26;
    const EWrongPlatform: u64 = 27;
    const EAccountInactive: u64 = 28;

    // ============================================================
    // Genesis defaults
    // ============================================================

    const DEFAULT_PLATFORM_FEE_BPS: u64 = 200;
    const DEFAULT_ECOSYSTEM_FEE_BPS: u64 = 50;
    const DEFAULT_MIN_ESCROW_MIST: u64 = 10_000_000;
    const MS_MINUTE: u64 = 60_000;
    const MS_HOUR: u64 = 3_600_000;
    const MS_DAY: u64 = 86_400_000;
    const DEFAULT_RELEASE_GRACE_MS: u64 = 3 * 86_400_000;
    const DEFAULT_REVIEW_WINDOW_MS: u64 = 30 * 86_400_000;
    const DEFAULT_MAX_TITLE_BYTES: u64 = 128;
    const DEFAULT_MAX_URI_BYTES: u64 = 2048;

    const MAX_RELEASE_GRACE_MS: u64 = 30 * 86_400_000;
    const MAX_REVIEW_WINDOW_MS: u64 = 365 * 86_400_000;
    const BPS_DENOMINATOR: u64 = 10000;

    // ============================================================
    // Objects
    // ============================================================

    /// Admin capability for [`TaskConfig`].
    public struct TaskMarketCap has key, store {
        id: UID,
    }

    /// Shared market configuration. Pausing (`active = false`) blocks only new offers.
    public struct TaskConfig has key {
        id: UID,
        platform_fee_bps: u64,
        ecosystem_fee_bps: u64,
        ecosystem_fee_recipient: address,
        min_escrow_mist: u64,
        accept_window_min_ms: u64,
        accept_window_default_ms: u64,
        accept_window_max_ms: u64,
        delivery_window_min_ms: u64,
        delivery_window_max_ms: u64,
        settle_window_min_ms: u64,
        settle_window_max_ms: u64,
        release_grace_ms: u64,
        review_window_ms: u64,
        max_title_bytes: u64,
        max_uri_bytes: u64,
        active: bool,
        version: u64,
        reputation: ReputationParams,
    }

    // The verifier caps a struct at 32 fields, so the escrow keeps its rarely-touched groups in
    // nested structs rather than growing past it.
    public struct TaskTerms has store {
        platform_fee_bps: u64,
        ecosystem_fee_bps: u64,
        ecosystem_fee_recipient: address,
        accept_window_ms: u64,
        delivery_window_ms: u64,
        settle_window_ms: u64,
        terms_digest: vector<u8>,
    }

    public struct TaskPayout has store {
        platform_fee_mist: u64,
        ecosystem_fee_mist: u64,
        net_amount_mist: u64,
    }

    public struct TaskReviewState has store {
        buyer_credibility_bps: u64,
        seller_credibility_bps: u64,
        buyer_review_stars: u8,
        seller_review_stars: u8,
    }

    /// One escrowed task. Shared; the rarely-touched groups live in the nested structs above.
    public struct TaskEscrow has key {
        id: UID,
        buyer_memory_account_id: ID,
        buyer_agent_id: Option<ID>,
        buyer_principal: address,
        seller_memory_account_id: ID,
        seller_agent_id: ID,
        seller_principal: address,
        /// Acting organization of the buyer, for reporting only.
        organization_id: Option<ID>,
        room_group_id: Option<ID>,
        /// Platform whose treasury receives the platform fee; `none` folds it into the ecosystem fee.
        platform_id: Option<address>,
        title: String,
        scope_digest: vector<u8>,
        amount_mist: u64,
        escrowed: Balance<MYSO>,
        status: u8,
        close_kind: u8,
        settlement_kind: u8,
        /// Terms frozen at offer time; a later config change never touches them.
        terms: TaskTerms,
        // Timeline
        created_at_ms: u64,
        accept_deadline_ms: u64,
        accepted_at_ms: u64,
        delivery_deadline_ms: u64,
        delivered_at_ms: u64,
        settle_deadline_ms: u64,
        settled_at_ms: u64,
        review_deadline_ms: u64,
        // Deliverable
        deliverable_digest: Option<vector<u8>>,
        deliverable_uri: Option<String>,
        /// Amounts paid out at settlement.
        payout: TaskPayout,
        // Mutual refund proposal
        refund_proposer: u8,
        refund_proposed_at_ms: u64,
        /// Credibility snapshots and the stars each side gave.
        reviews: TaskReviewState,
    }

    // ============================================================
    // Events (flat; mirrored by the indexer's BCS structs)
    // ============================================================

    public struct TaskOffered has copy, drop {
        task_id: ID,
        buyer_principal: address,
        buyer_agent_id: Option<ID>,
        buyer_memory_account_id: ID,
        seller_principal: address,
        seller_agent_id: ID,
        seller_memory_account_id: ID,
        organization_id: Option<ID>,
        room_group_id: Option<ID>,
        platform_id: Option<address>,
        title: String,
        scope_digest: vector<u8>,
        amount_mist: u64,
        accept_window_ms: u64,
        delivery_window_ms: u64,
        settle_window_ms: u64,
        platform_fee_bps: u64,
        ecosystem_fee_bps: u64,
        terms_digest: vector<u8>,
        accept_deadline_ms: u64,
        created_at_ms: u64,
    }

    public struct TaskAccepted has copy, drop {
        task_id: ID,
        seller_agent_id: ID,
        seller_principal: address,
        accepted_at_ms: u64,
        delivery_deadline_ms: u64,
    }

    public struct TaskCancelled has copy, drop {
        task_id: ID,
        close_kind: u8,
        buyer_principal: address,
        amount_mist: u64,
        cancelled_at_ms: u64,
    }

    public struct TaskDelivered has copy, drop {
        task_id: ID,
        seller_agent_id: ID,
        deliverable_digest: vector<u8>,
        deliverable_uri: String,
        delivered_at_ms: u64,
        settle_deadline_ms: u64,
    }

    public struct TaskRefundProposed has copy, drop {
        task_id: ID,
        proposer_role: u8,
        proposed_at_ms: u64,
    }

    public struct TaskRefundProposalCleared has copy, drop {
        task_id: ID,
        cleared_by_role: u8,
        reason: u8,
        cleared_at_ms: u64,
    }

    public struct TaskRefunded has copy, drop {
        task_id: ID,
        refund_kind: u8,
        buyer_principal: address,
        amount_mist: u64,
        refunded_at_ms: u64,
    }

    /// `receipt_id` equals `task_id`: one receipt per settled task.
    public struct TaskReceiptSettled has copy, drop {
        task_id: ID,
        receipt_id: ID,
        buyer_principal: address,
        buyer_agent_id: Option<ID>,
        seller_principal: address,
        seller_agent_id: ID,
        organization_id: Option<ID>,
        room_group_id: Option<ID>,
        settlement_kind: u8,
        amount_mist: u64,
        platform_fee_mist: u64,
        ecosystem_fee_mist: u64,
        net_amount_mist: u64,
        deliverable_digest: Option<vector<u8>>,
        deliverable_uri: Option<String>,
        buyer_credibility_bps: u64,
        seller_credibility_bps: u64,
        settle_deadline_ms: u64,
        review_deadline_ms: u64,
        settled_at_ms: u64,
    }

    public struct TaskReviewSubmitted has copy, drop {
        task_id: ID,
        receipt_id: ID,
        /// 1 buyer reviewed the seller, 2 seller reviewed the buyer.
        rater_role: u8,
        rater_principal: address,
        rater_agent_id: Option<ID>,
        reviewee_principal: address,
        reviewee_agent_id: Option<ID>,
        stars: u8,
        rater_credibility_bps: u64,
        value_weight_bps: u64,
        base_weight_bps: u64,
        prior_pair_reviews: u64,
        weight_bps: u64,
        reviewed_at_ms: u64,
    }

    public struct TaskConfigUpdated has copy, drop {
        updated_by: address,
        platform_fee_bps: u64,
        ecosystem_fee_bps: u64,
        ecosystem_fee_recipient: address,
        min_escrow_mist: u64,
        accept_window_min_ms: u64,
        accept_window_default_ms: u64,
        accept_window_max_ms: u64,
        delivery_window_min_ms: u64,
        delivery_window_max_ms: u64,
        settle_window_min_ms: u64,
        settle_window_max_ms: u64,
        release_grace_ms: u64,
        review_window_ms: u64,
        max_title_bytes: u64,
        max_uri_bytes: u64,
        active: bool,
        prior_mean_milli: u64,
        prior_weight_bps: u64,
        value_ref_mist: u64,
        min_value_weight_bps: u64,
        max_value_weight_bps: u64,
        cred_base_bps: u64,
        cred_step_bps: u64,
        cred_cap_n: u64,
        cred_max_bps: u64,
        overall_seller_weight_bps: u64,
        overall_buyer_weight_bps: u64,
        timestamp_ms: u64,
    }

    // ============================================================
    // Bootstrap and configuration
    // ============================================================

    public(package) fun bootstrap_init(admin: address, clock: &Clock, ctx: &mut TxContext) {
        let config = TaskConfig {
            id: object::new(ctx),
            platform_fee_bps: DEFAULT_PLATFORM_FEE_BPS,
            ecosystem_fee_bps: DEFAULT_ECOSYSTEM_FEE_BPS,
            ecosystem_fee_recipient: admin,
            min_escrow_mist: DEFAULT_MIN_ESCROW_MIST,
            accept_window_min_ms: 5 * MS_MINUTE,
            accept_window_default_ms: MS_DAY,
            accept_window_max_ms: 14 * MS_DAY,
            delivery_window_min_ms: MS_HOUR,
            delivery_window_max_ms: 90 * MS_DAY,
            settle_window_min_ms: MS_HOUR,
            settle_window_max_ms: 60 * MS_DAY,
            release_grace_ms: DEFAULT_RELEASE_GRACE_MS,
            review_window_ms: DEFAULT_REVIEW_WINDOW_MS,
            max_title_bytes: DEFAULT_MAX_TITLE_BYTES,
            max_uri_bytes: DEFAULT_MAX_URI_BYTES,
            active: true,
            version: upgrade::current_version(),
            reputation: memory::new_reputation_params(
                3000,
                50000,
                1_000_000_000,
                500,
                20000,
                2000,
                500,
                16,
                10000,
                5000,
                5000,
            ),
        };
        validate_task_config(&config);
        emit_config_updated(&config, admin, clock);
        transfer::share_object(config);
    }

    public(package) fun create_task_market_cap(ctx: &mut TxContext): TaskMarketCap {
        TaskMarketCap { id: object::new(ctx) }
    }

    /// Replaces the whole configuration. Every bound is re-validated; a violation aborts.
    public entry fun set_task_config(
        _: &TaskMarketCap,
        config: &mut TaskConfig,
        platform_fee_bps: u64,
        ecosystem_fee_bps: u64,
        ecosystem_fee_recipient: address,
        min_escrow_mist: u64,
        accept_window_min_ms: u64,
        accept_window_default_ms: u64,
        accept_window_max_ms: u64,
        delivery_window_min_ms: u64,
        delivery_window_max_ms: u64,
        settle_window_min_ms: u64,
        settle_window_max_ms: u64,
        release_grace_ms: u64,
        review_window_ms: u64,
        max_title_bytes: u64,
        max_uri_bytes: u64,
        active: bool,
        prior_mean_milli: u64,
        prior_weight_bps: u64,
        value_ref_mist: u64,
        min_value_weight_bps: u64,
        max_value_weight_bps: u64,
        cred_base_bps: u64,
        cred_step_bps: u64,
        cred_cap_n: u64,
        cred_max_bps: u64,
        overall_seller_weight_bps: u64,
        overall_buyer_weight_bps: u64,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(config.version == upgrade::current_version(), EWrongVersion);
        config.platform_fee_bps = platform_fee_bps;
        config.ecosystem_fee_bps = ecosystem_fee_bps;
        config.ecosystem_fee_recipient = ecosystem_fee_recipient;
        config.min_escrow_mist = min_escrow_mist;
        config.accept_window_min_ms = accept_window_min_ms;
        config.accept_window_default_ms = accept_window_default_ms;
        config.accept_window_max_ms = accept_window_max_ms;
        config.delivery_window_min_ms = delivery_window_min_ms;
        config.delivery_window_max_ms = delivery_window_max_ms;
        config.settle_window_min_ms = settle_window_min_ms;
        config.settle_window_max_ms = settle_window_max_ms;
        config.release_grace_ms = release_grace_ms;
        config.review_window_ms = review_window_ms;
        config.max_title_bytes = max_title_bytes;
        config.max_uri_bytes = max_uri_bytes;
        config.active = active;
        // Validates the reputation bounds (aborts `EInvalidReputationParams`).
        config.reputation = memory::new_reputation_params(
            prior_mean_milli,
            prior_weight_bps,
            value_ref_mist,
            min_value_weight_bps,
            max_value_weight_bps,
            cred_base_bps,
            cred_step_bps,
            cred_cap_n,
            cred_max_bps,
            overall_seller_weight_bps,
            overall_buyer_weight_bps,
        );
        validate_task_config(config);
        emit_config_updated(config, tx_context::sender(ctx), clock);
    }

    public entry fun migrate_task_config(
        config: &mut TaskConfig,
        _: &UpgradeAdminCap,
        ctx: &mut TxContext,
    ) {
        let current_version = upgrade::current_version();
        assert!(config.version < current_version, EWrongVersion);
        let old_version = config.version;
        config.version = current_version;
        upgrade::emit_migration_event(
            object::id(config),
            string::utf8(b"TaskConfig"),
            old_version,
            tx_context::sender(ctx),
        );
    }

    /// Non-reputation bounds; reputation bounds live in `memory::assert_reputation_params`.
    fun validate_task_config(c: &TaskConfig) {
        assert!(c.platform_fee_bps <= BPS_DENOMINATOR, EInvalidTaskConfig);
        assert!(c.ecosystem_fee_bps <= BPS_DENOMINATOR, EInvalidTaskConfig);
        assert!(c.platform_fee_bps + c.ecosystem_fee_bps <= BPS_DENOMINATOR, EInvalidTaskConfig);
        assert!(c.min_escrow_mist >= 1, EInvalidTaskConfig);
        assert!(
            c.accept_window_min_ms > 0
                && c.accept_window_min_ms <= c.accept_window_default_ms
                && c.accept_window_default_ms <= c.accept_window_max_ms,
            EInvalidTaskConfig,
        );
        assert!(
            c.delivery_window_min_ms > 0 && c.delivery_window_min_ms <= c.delivery_window_max_ms,
            EInvalidTaskConfig,
        );
        assert!(
            c.settle_window_min_ms > 0 && c.settle_window_min_ms <= c.settle_window_max_ms,
            EInvalidTaskConfig,
        );
        assert!(c.release_grace_ms <= MAX_RELEASE_GRACE_MS, EInvalidTaskConfig);
        assert!(c.review_window_ms > 0 && c.review_window_ms <= MAX_REVIEW_WINDOW_MS, EInvalidTaskConfig);
        assert!(c.max_title_bytes >= 1 && c.max_uri_bytes >= 1, EInvalidTaskConfig);
        memory::assert_reputation_params(&c.reputation);
    }

    fun emit_config_updated(c: &TaskConfig, updated_by: address, clock: &Clock) {
        let p = &c.reputation;
        event::emit(TaskConfigUpdated {
            updated_by,
            platform_fee_bps: c.platform_fee_bps,
            ecosystem_fee_bps: c.ecosystem_fee_bps,
            ecosystem_fee_recipient: c.ecosystem_fee_recipient,
            min_escrow_mist: c.min_escrow_mist,
            accept_window_min_ms: c.accept_window_min_ms,
            accept_window_default_ms: c.accept_window_default_ms,
            accept_window_max_ms: c.accept_window_max_ms,
            delivery_window_min_ms: c.delivery_window_min_ms,
            delivery_window_max_ms: c.delivery_window_max_ms,
            settle_window_min_ms: c.settle_window_min_ms,
            settle_window_max_ms: c.settle_window_max_ms,
            release_grace_ms: c.release_grace_ms,
            review_window_ms: c.review_window_ms,
            max_title_bytes: c.max_title_bytes,
            max_uri_bytes: c.max_uri_bytes,
            active: c.active,
            prior_mean_milli: memory::reputation_prior_mean_milli(p),
            prior_weight_bps: memory::reputation_prior_weight_bps(p),
            value_ref_mist: memory::reputation_value_ref_mist(p),
            min_value_weight_bps: memory::reputation_min_value_weight_bps(p),
            max_value_weight_bps: memory::reputation_max_value_weight_bps(p),
            cred_base_bps: memory::reputation_cred_base_bps(p),
            cred_step_bps: memory::reputation_cred_step_bps(p),
            cred_cap_n: memory::reputation_cred_cap_n(p),
            cred_max_bps: memory::reputation_cred_max_bps(p),
            overall_seller_weight_bps: memory::reputation_overall_seller_weight_bps(p),
            overall_buyer_weight_bps: memory::reputation_overall_buyer_weight_bps(p),
            timestamp_ms: clock::timestamp_ms(clock),
        });
    }

    // ============================================================
    // Offer lifecycle: create, accept, decline, cancel, expire
    // ============================================================

    /// Escrows `escrow_amount` from `payment` as an offer to `seller_agent`.
    ///
    /// The buyer is resolved with `CAP_TASK_TRADE` and the spend cap (`escrow_amount`), so an
    /// agent's per-action limit and approval-required caps apply to funding an offer.
    public entry fun create_task(
        config: &TaskConfig,
        memory_config: &MemoryConfig,
        buyer_account: &MemoryAccount,
        seller_account: &MemoryAccount,
        seller_agent: &SubAgent,
        payment: &mut Coin<MYSO>,
        escrow_amount: u64,
        scope_digest: vector<u8>,
        title: String,
        accept_window_ms: u64,
        delivery_window_ms: u64,
        settle_window_ms: u64,
        room_group_id: Option<ID>,
        platform_id: Option<address>,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(config.version == upgrade::current_version(), EWrongVersion);
        assert!(config.active, EMarketPaused);
        assert!(escrow_amount >= config.min_escrow_mist, EEscrowTooSmall);
        assert!(coin::value(payment) >= escrow_amount, EInsufficientPayment);
        assert!(vector::length(&scope_digest) == DIGEST_LENGTH, EInvalidDigest);
        assert!(string::length(&title) <= config.max_title_bytes, ETitleTooLong);
        assert!(
            accept_window_ms >= config.accept_window_min_ms
                && accept_window_ms <= config.accept_window_max_ms,
            EInvalidWindow,
        );
        assert!(
            delivery_window_ms >= config.delivery_window_min_ms
                && delivery_window_ms <= config.delivery_window_max_ms,
            EInvalidWindow,
        );
        assert!(
            settle_window_ms >= config.settle_window_min_ms
                && settle_window_ms <= config.settle_window_max_ms,
            EInvalidWindow,
        );

        // Buyer: principal or an agent with the cap, within its spend limit and approval rules.
        let cap = memory::cap_task_trade();
        let acting = memory::resolve_actor_with_cap(
            memory_config,
            buyer_account,
            cap,
            option::none(),
            escrow_amount,
            clock,
            ctx,
        );
        memory::assert_direct_execution_allowed(buyer_account, cap, ctx);
        let buyer_agent_id = memory::acting_sub_agent_id(&acting);

        // Seller: a distinct principal's registered, active agent.
        let seller_agent_id = object::id(seller_agent);
        assert!(
            memory::sub_agent_memory_account_id(seller_agent) == object::id(seller_account),
            EWrongTaskObject,
        );
        assert!(object::id(buyer_account) != object::id(seller_account), ETaskSelfDealing);
        assert!(
            memory::owner(buyer_account) != memory::owner(seller_account),
            ETaskSelfDealing,
        );
        assert!(
            !option::contains(&buyer_agent_id, &seller_agent_id),
            ETaskSelfDealing,
        );
        assert!(memory::is_active(seller_account), EAccountInactive);
        memory::assert_sub_agent_active(seller_agent, clock);

        let now = clock::timestamp_ms(clock);
        let buyer_principal = memory::acting_principal_owner(&acting);
        let seller_principal = memory::owner(seller_account);
        let terms_digest = compute_terms_digest(
            buyer_principal,
            seller_agent_id,
            escrow_amount,
            &scope_digest,
            accept_window_ms,
            delivery_window_ms,
            settle_window_ms,
            config.platform_fee_bps,
            config.ecosystem_fee_bps,
            config.ecosystem_fee_recipient,
            &platform_id,
        );

        let escrowed = coin::into_balance(coin::split(payment, escrow_amount, ctx));
        let accept_deadline_ms = memory::checked_add_u64(now, accept_window_ms);
        let task = TaskEscrow {
            id: object::new(ctx),
            buyer_memory_account_id: object::id(buyer_account),
            buyer_agent_id,
            buyer_principal,
            seller_memory_account_id: object::id(seller_account),
            seller_agent_id,
            seller_principal,
            organization_id: memory::acting_organization_id(&acting),
            room_group_id,
            platform_id,
            title,
            scope_digest,
            amount_mist: escrow_amount,
            escrowed,
            status: STATUS_OFFERED,
            close_kind: 0,
            settlement_kind: 0,
            terms: TaskTerms {
                platform_fee_bps: config.platform_fee_bps,
                ecosystem_fee_bps: config.ecosystem_fee_bps,
                ecosystem_fee_recipient: config.ecosystem_fee_recipient,
                accept_window_ms,
                delivery_window_ms,
                settle_window_ms,
                terms_digest,
            },
            created_at_ms: now,
            accept_deadline_ms,
            accepted_at_ms: 0,
            delivery_deadline_ms: 0,
            delivered_at_ms: 0,
            settle_deadline_ms: 0,
            settled_at_ms: 0,
            review_deadline_ms: 0,
            deliverable_digest: option::none(),
            deliverable_uri: option::none(),
            payout: TaskPayout { platform_fee_mist: 0, ecosystem_fee_mist: 0, net_amount_mist: 0 },
            refund_proposer: 0,
            refund_proposed_at_ms: 0,
            reviews: TaskReviewState {
                buyer_credibility_bps: 0,
                seller_credibility_bps: 0,
                buyer_review_stars: 0,
                seller_review_stars: 0,
            },
        };

        event::emit(TaskOffered {
            task_id: object::id(&task),
            buyer_principal,
            buyer_agent_id,
            buyer_memory_account_id: task.buyer_memory_account_id,
            seller_principal,
            seller_agent_id,
            seller_memory_account_id: task.seller_memory_account_id,
            organization_id: task.organization_id,
            room_group_id,
            platform_id,
            title: task.title,
            scope_digest: task.scope_digest,
            amount_mist: escrow_amount,
            accept_window_ms,
            delivery_window_ms,
            settle_window_ms,
            platform_fee_bps: task.terms.platform_fee_bps,
            ecosystem_fee_bps: task.terms.ecosystem_fee_bps,
            terms_digest: task.terms.terms_digest,
            accept_deadline_ms,
            created_at_ms: now,
        });
        transfer::share_object(task);
    }

    /// The seller approves the exact terms. `expected_terms_digest` must equal the offer's
    /// `terms_digest`, so a seller cannot accept different scope, payment or windows by mistake.
    public entry fun accept_task(
        config: &TaskConfig,
        task: &mut TaskEscrow,
        memory_config: &MemoryConfig,
        seller_account: &MemoryAccount,
        seller_agent: &SubAgent,
        expected_terms_digest: vector<u8>,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(config.version == upgrade::current_version(), EWrongVersion);
        assert!(task.status == STATUS_OFFERED, EWrongStatus);
        let now = clock::timestamp_ms(clock);
        assert!(now <= task.accept_deadline_ms, EAcceptExpired);
        assert!(expected_terms_digest == task.terms.terms_digest, ETermsMismatch);
        assert_seller_with_cap(task, memory_config, seller_account, seller_agent, clock, ctx);
        // Accepting is taking on new work; deactivated or expired agents cannot.
        assert!(memory::is_active(seller_account), EAccountInactive);
        memory::assert_sub_agent_active(seller_agent, clock);

        task.status = STATUS_ACCEPTED;
        task.accepted_at_ms = now;
        task.delivery_deadline_ms = memory::checked_add_u64(now, task.terms.delivery_window_ms);
        event::emit(TaskAccepted {
            task_id: object::id(task),
            seller_agent_id: task.seller_agent_id,
            seller_principal: task.seller_principal,
            accepted_at_ms: now,
            delivery_deadline_ms: task.delivery_deadline_ms,
        });
    }

    /// The seller turns an offer down. No `active` check: a deactivated seller can still decline.
    public entry fun decline_task(
        task: &mut TaskEscrow,
        memory_config: &MemoryConfig,
        seller_account: &MemoryAccount,
        seller_agent: &SubAgent,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(task.status == STATUS_OFFERED, EWrongStatus);
        assert_seller_with_cap(task, memory_config, seller_account, seller_agent, clock, ctx);
        close_offer(task, CLOSE_SELLER_DECLINED, clock, ctx);
    }

    /// The buyer withdraws an offer the seller has not accepted.
    public entry fun cancel_offer(
        task: &mut TaskEscrow,
        memory_config: &MemoryConfig,
        buyer_account: &MemoryAccount,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(task.status == STATUS_OFFERED, EWrongStatus);
        resolve_buyer_actor(task, memory_config, buyer_account, 0, clock, ctx);
        close_offer(task, CLOSE_BUYER_CANCELLED, clock, ctx);
    }

    /// Anyone may return an unanswered offer's escrow once the accept deadline passed.
    public entry fun expire_offer(task: &mut TaskEscrow, clock: &Clock, ctx: &mut TxContext) {
        assert!(task.status == STATUS_OFFERED, EWrongStatus);
        assert!(clock::timestamp_ms(clock) > task.accept_deadline_ms, ETooEarly);
        close_offer(task, CLOSE_EXPIRED, clock, ctx);
    }

    fun close_offer(task: &mut TaskEscrow, close_kind: u8, clock: &Clock, ctx: &mut TxContext) {
        task.status = STATUS_CANCELLED;
        task.close_kind = close_kind;
        pay_buyer_back(task, ctx);
        event::emit(TaskCancelled {
            task_id: object::id(task),
            close_kind,
            buyer_principal: task.buyer_principal,
            amount_mist: task.amount_mist,
            cancelled_at_ms: clock::timestamp_ms(clock),
        });
    }

    // ============================================================
    // Delivery and settlement
    // ============================================================

    /// The accepted seller agent records the deliverable. No cap and no `active` check: an
    /// accepted task can still be delivered after the agent is deactivated.
    public entry fun mark_delivered(
        config: &TaskConfig,
        task: &mut TaskEscrow,
        seller_agent: &SubAgent,
        digest: vector<u8>,
        uri: String,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(config.version == upgrade::current_version(), EWrongVersion);
        assert!(task.status == STATUS_ACCEPTED, EWrongStatus);
        assert!(object::id(seller_agent) == task.seller_agent_id, EWrongTaskObject);
        assert!(
            tx_context::sender(ctx) == memory::sub_agent_derived_address(seller_agent),
            ENotSeller,
        );
        let now = clock::timestamp_ms(clock);
        assert!(now <= task.delivery_deadline_ms, EDeliveryDeadlinePassed);
        assert!(vector::length(&digest) == DIGEST_LENGTH, EInvalidDigest);
        assert!(string::length(&uri) <= config.max_uri_bytes, EUriTooLong);

        task.status = STATUS_DELIVERED;
        task.delivered_at_ms = now;
        task.settle_deadline_ms = memory::checked_add_u64(now, task.terms.settle_window_ms);
        event::emit(TaskDelivered {
            task_id: object::id(task),
            seller_agent_id: task.seller_agent_id,
            deliverable_digest: digest,
            deliverable_uri: uri,
            delivered_at_ms: now,
            settle_deadline_ms: task.settle_deadline_ms,
        });
        task.deliverable_digest = option::some(digest);
        task.deliverable_uri = option::some(uri);
    }

    /// The buyer pays for delivered work. No clock check: the buyer can settle until a release
    /// or mutual refund lands. Payment carries no rating; reviews are separate calls.
    public entry fun settle_task(
        config: &TaskConfig,
        task: &mut TaskEscrow,
        memory_config: &MemoryConfig,
        buyer_account: &mut MemoryAccount,
        seller_account: &mut MemoryAccount,
        platform: &mut Platform,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(config.version == upgrade::current_version(), EWrongVersion);
        assert!(task.status == STATUS_DELIVERED, EWrongStatus);
        resolve_buyer_actor(task, memory_config, buyer_account, 0, clock, ctx);
        finalize_settlement(config, task, buyer_account, seller_account, platform, SETTLED_BY_BUYER, clock, ctx);
    }

    /// Anyone may pay the seller once `settle_deadline + release_grace` has passed, whatever
    /// refund proposals are pending. Produces a receipt both parties may review.
    public entry fun release_task(
        config: &TaskConfig,
        task: &mut TaskEscrow,
        buyer_account: &mut MemoryAccount,
        seller_account: &mut MemoryAccount,
        platform: &mut Platform,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(config.version == upgrade::current_version(), EWrongVersion);
        assert!(task.status == STATUS_DELIVERED, EWrongStatus);
        let releasable_at = memory::checked_add_u64(task.settle_deadline_ms, config.release_grace_ms);
        assert!(clock::timestamp_ms(clock) > releasable_at, ETooEarly);
        finalize_settlement(config, task, buyer_account, seller_account, platform, SETTLED_AUTO_RELEASED, clock, ctx);
    }

    fun finalize_settlement(
        config: &TaskConfig,
        task: &mut TaskEscrow,
        buyer_account: &mut MemoryAccount,
        seller_account: &mut MemoryAccount,
        platform: &mut Platform,
        kind: u8,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(object::id(buyer_account) == task.buyer_memory_account_id, EWrongTaskObject);
        assert!(object::id(seller_account) == task.seller_memory_account_id, EWrongTaskObject);
        if (option::is_some(&task.platform_id)) {
            assert!(
                object::uid_to_address(platform::id(platform)) == *option::borrow(&task.platform_id),
                EWrongPlatform,
            );
        };

        pay_seller(task, platform, clock, ctx);

        let now = clock::timestamp_ms(clock);
        task.reviews.buyer_credibility_bps =
            memory::record_settlement(buyer_account, task.amount_mist, &config.reputation, clock);
        task.reviews.seller_credibility_bps =
            memory::record_settlement(seller_account, task.amount_mist, &config.reputation, clock);
        task.status = STATUS_SETTLED;
        task.settlement_kind = kind;
        task.settled_at_ms = now;
        task.review_deadline_ms = memory::checked_add_u64(now, config.review_window_ms);
        // A pending refund proposal is void once the task is paid.
        task.refund_proposer = 0;
        task.refund_proposed_at_ms = 0;

        event::emit(TaskReceiptSettled {
            task_id: object::id(task),
            receipt_id: object::id(task),
            buyer_principal: task.buyer_principal,
            buyer_agent_id: task.buyer_agent_id,
            seller_principal: task.seller_principal,
            seller_agent_id: task.seller_agent_id,
            organization_id: task.organization_id,
            room_group_id: task.room_group_id,
            settlement_kind: kind,
            amount_mist: task.amount_mist,
            platform_fee_mist: task.payout.platform_fee_mist,
            ecosystem_fee_mist: task.payout.ecosystem_fee_mist,
            net_amount_mist: task.payout.net_amount_mist,
            deliverable_digest: task.deliverable_digest,
            deliverable_uri: task.deliverable_uri,
            buyer_credibility_bps: task.reviews.buyer_credibility_bps,
            seller_credibility_bps: task.reviews.seller_credibility_bps,
            settle_deadline_ms: task.settle_deadline_ms,
            review_deadline_ms: task.review_deadline_ms,
            settled_at_ms: now,
        });
    }

    /// Splits the escrow with the frozen fee bps: platform treasury, ecosystem recipient, seller.
    /// Without a platform the platform share folds into the ecosystem fee.
    fun pay_seller(task: &mut TaskEscrow, platform: &mut Platform, clock: &Clock, ctx: &mut TxContext) {
        let amount = task.amount_mist;
        let platform_share = memory::checked_bps_of(amount, task.terms.platform_fee_bps);
        let ecosystem_share = memory::checked_bps_of(amount, task.terms.ecosystem_fee_bps);
        assert!(memory::checked_add_u64(platform_share, ecosystem_share) <= amount, EInvalidTaskConfig);

        let has_platform = option::is_some(&task.platform_id);
        let platform_fee = if (has_platform) { platform_share } else { 0 };
        let ecosystem_fee = if (has_platform) {
            ecosystem_share
        } else {
            memory::checked_add_u64(platform_share, ecosystem_share)
        };

        let mut payout = coin::from_balance(balance::withdraw_all(&mut task.escrowed), ctx);
        if (platform_fee > 0) {
            let mut platform_coin = coin::split(&mut payout, platform_fee, ctx);
            platform::fund_platform_treasury_from_coin(platform, &mut platform_coin, platform_fee, clock, ctx);
            if (coin::value(&platform_coin) > 0) {
                // Anything the treasury did not take goes to the seller with the net.
                coin::join(&mut payout, platform_coin);
            } else {
                coin::destroy_zero(platform_coin);
            };
        };
        if (ecosystem_fee > 0) {
            transfer::public_transfer(
                coin::split(&mut payout, ecosystem_fee, ctx),
                task.terms.ecosystem_fee_recipient,
            );
        };
        task.payout.platform_fee_mist = platform_fee;
        task.payout.ecosystem_fee_mist = ecosystem_fee;
        task.payout.net_amount_mist = coin::value(&payout);
        if (coin::value(&payout) > 0) {
            transfer::public_transfer(payout, task.seller_principal);
        } else {
            coin::destroy_zero(payout);
        };
    }

    // ============================================================
    // Refunds
    // ============================================================

    /// Anyone may refund an **accepted** task whose seller missed the delivery deadline.
    /// It aborts on OFFERED (expire/cancel/decline) and on DELIVERED (settle, release, or a
    /// mutual refund), so a delivered task can never be refunded unilaterally.
    public entry fun refund_task(task: &mut TaskEscrow, clock: &Clock, ctx: &mut TxContext) {
        assert!(task.status == STATUS_ACCEPTED, EWrongStatus);
        assert!(clock::timestamp_ms(clock) > task.delivery_deadline_ms, ETooEarly);
        close_as_refunded(task, REFUND_DELIVERY_MISSED, clock, ctx);
    }

    /// Either party proposes a full refund of a delivered task. A party can refresh its own
    /// proposal but cannot override the counterparty's pending one.
    public entry fun propose_refund(
        task: &mut TaskEscrow,
        memory_config: &MemoryConfig,
        buyer_account: &MemoryAccount,
        seller_agent: &SubAgent,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(task.status == STATUS_DELIVERED, EWrongStatus);
        let role = resolve_party(task, memory_config, buyer_account, seller_agent, clock, ctx);
        assert!(task.refund_proposer == 0 || task.refund_proposer == role, EProposalPending);
        task.refund_proposer = role;
        task.refund_proposed_at_ms = clock::timestamp_ms(clock);
        event::emit(TaskRefundProposed {
            task_id: object::id(task),
            proposer_role: role,
            proposed_at_ms: task.refund_proposed_at_ms,
        });
    }

    /// The counterparty accepts: full escrow back to the buyer, no fees.
    public entry fun accept_refund(
        task: &mut TaskEscrow,
        memory_config: &MemoryConfig,
        buyer_account: &MemoryAccount,
        seller_agent: &SubAgent,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(task.status == STATUS_DELIVERED, EWrongStatus);
        assert!(task.refund_proposer != 0, ENoProposal);
        let role = resolve_party(task, memory_config, buyer_account, seller_agent, clock, ctx);
        assert!(role != task.refund_proposer, ENotCounterparty);
        task.refund_proposer = 0;
        task.refund_proposed_at_ms = 0;
        close_as_refunded(task, REFUND_MUTUAL, clock, ctx);
    }

    /// The counterparty turns a proposal down.
    public entry fun reject_refund(
        task: &mut TaskEscrow,
        memory_config: &MemoryConfig,
        buyer_account: &MemoryAccount,
        seller_agent: &SubAgent,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(task.status == STATUS_DELIVERED, EWrongStatus);
        assert!(task.refund_proposer != 0, ENoProposal);
        let role = resolve_party(task, memory_config, buyer_account, seller_agent, clock, ctx);
        assert!(role != task.refund_proposer, ENotCounterparty);
        clear_proposal(task, role, CLEARED_REJECTED, clock);
    }

    /// The proposer takes its proposal back.
    public entry fun withdraw_refund(
        task: &mut TaskEscrow,
        memory_config: &MemoryConfig,
        buyer_account: &MemoryAccount,
        seller_agent: &SubAgent,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(task.status == STATUS_DELIVERED, EWrongStatus);
        assert!(task.refund_proposer != 0, ENoProposal);
        let role = resolve_party(task, memory_config, buyer_account, seller_agent, clock, ctx);
        assert!(role == task.refund_proposer, ENotProposer);
        clear_proposal(task, role, CLEARED_WITHDRAWN, clock);
    }

    fun clear_proposal(task: &mut TaskEscrow, by_role: u8, reason: u8, clock: &Clock) {
        task.refund_proposer = 0;
        task.refund_proposed_at_ms = 0;
        event::emit(TaskRefundProposalCleared {
            task_id: object::id(task),
            cleared_by_role: by_role,
            reason,
            cleared_at_ms: clock::timestamp_ms(clock),
        });
    }

    fun close_as_refunded(task: &mut TaskEscrow, refund_kind: u8, clock: &Clock, ctx: &mut TxContext) {
        task.status = STATUS_REFUNDED;
        pay_buyer_back(task, ctx);
        event::emit(TaskRefunded {
            task_id: object::id(task),
            refund_kind,
            buyer_principal: task.buyer_principal,
            amount_mist: task.amount_mist,
            refunded_at_ms: clock::timestamp_ms(clock),
        });
    }

    fun pay_buyer_back(task: &mut TaskEscrow, ctx: &mut TxContext) {
        let refund = coin::from_balance(balance::withdraw_all(&mut task.escrowed), ctx);
        transfer::public_transfer(refund, task.buyer_principal);
    }

    // ============================================================
    // Reviews (bidirectional, once per direction per receipt)
    // ============================================================

    /// The buyer rates the seller agent 1-5 stars. Updates the agent's seller-role aggregate.
    public entry fun review_seller(
        config: &TaskConfig,
        task: &mut TaskEscrow,
        memory_config: &MemoryConfig,
        buyer_account: &MemoryAccount,
        seller_account: &mut MemoryAccount,
        seller_agent: &mut SubAgent,
        stars: u8,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(config.version == upgrade::current_version(), EWrongVersion);
        assert_reviewable(task, stars, clock);
        assert!(task.reviews.buyer_review_stars == 0, EAlreadyReviewed);
        assert!(object::id(seller_agent) == task.seller_agent_id, EWrongTaskObject);
        assert!(object::id(seller_account) == task.seller_memory_account_id, EWrongTaskObject);
        resolve_buyer_actor(task, memory_config, buyer_account, 0, clock, ctx);

        let outcome = memory::apply_seller_review(
            seller_agent,
            seller_account,
            task.buyer_principal,
            stars,
            task.amount_mist,
            task.reviews.buyer_credibility_bps,
            &config.reputation,
            object::id(task),
            clock,
        );
        task.reviews.buyer_review_stars = stars;
        event::emit(TaskReviewSubmitted {
            task_id: object::id(task),
            receipt_id: object::id(task),
            rater_role: PARTY_BUYER,
            rater_principal: task.buyer_principal,
            rater_agent_id: task.buyer_agent_id,
            reviewee_principal: task.seller_principal,
            reviewee_agent_id: option::some(task.seller_agent_id),
            stars,
            rater_credibility_bps: task.reviews.buyer_credibility_bps,
            value_weight_bps: memory::outcome_value_weight_bps(&outcome),
            base_weight_bps: memory::outcome_base_weight_bps(&outcome),
            prior_pair_reviews: memory::outcome_prior_pair_reviews(&outcome),
            weight_bps: memory::outcome_weight_bps(&outcome),
            reviewed_at_ms: clock::timestamp_ms(clock),
        });
    }

    /// The seller rates a buyer **agent**. Aborts `EWrongReviewTarget` for a direct-buyer task.
    public entry fun review_buyer_agent(
        config: &TaskConfig,
        task: &mut TaskEscrow,
        buyer_account: &mut MemoryAccount,
        buyer_agent: &mut SubAgent,
        seller_agent: &SubAgent,
        stars: u8,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(config.version == upgrade::current_version(), EWrongVersion);
        assert_reviewable(task, stars, clock);
        assert!(task.reviews.seller_review_stars == 0, EAlreadyReviewed);
        assert_seller_sender(task, seller_agent, ctx);
        assert!(
            option::contains(&task.buyer_agent_id, &object::id(buyer_agent)),
            EWrongReviewTarget,
        );
        assert!(object::id(buyer_account) == task.buyer_memory_account_id, EWrongTaskObject);

        let outcome = memory::apply_buyer_agent_review(
            buyer_agent,
            buyer_account,
            task.seller_principal,
            stars,
            task.amount_mist,
            task.reviews.seller_credibility_bps,
            &config.reputation,
            object::id(task),
            clock,
        );
        record_seller_review(task, stars, &outcome, clock);
    }

    /// The seller rates a principal that bought directly. Aborts `EWrongReviewTarget` when the
    /// task had a buyer agent.
    public entry fun review_buyer_principal(
        config: &TaskConfig,
        task: &mut TaskEscrow,
        buyer_account: &mut MemoryAccount,
        seller_agent: &SubAgent,
        stars: u8,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(config.version == upgrade::current_version(), EWrongVersion);
        assert_reviewable(task, stars, clock);
        assert!(task.reviews.seller_review_stars == 0, EAlreadyReviewed);
        assert_seller_sender(task, seller_agent, ctx);
        assert!(option::is_none(&task.buyer_agent_id), EWrongReviewTarget);
        assert!(object::id(buyer_account) == task.buyer_memory_account_id, EWrongTaskObject);

        let outcome = memory::apply_human_buyer_review(
            buyer_account,
            task.seller_principal,
            stars,
            task.amount_mist,
            task.reviews.seller_credibility_bps,
            &config.reputation,
            object::id(task),
            clock,
        );
        record_seller_review(task, stars, &outcome, clock);
    }

    fun record_seller_review(
        task: &mut TaskEscrow,
        stars: u8,
        outcome: &memory::ReviewOutcome,
        clock: &Clock,
    ) {
        task.reviews.seller_review_stars = stars;
        event::emit(TaskReviewSubmitted {
            task_id: object::id(task),
            receipt_id: object::id(task),
            rater_role: PARTY_SELLER,
            rater_principal: task.seller_principal,
            rater_agent_id: option::some(task.seller_agent_id),
            reviewee_principal: task.buyer_principal,
            reviewee_agent_id: task.buyer_agent_id,
            stars,
            rater_credibility_bps: task.reviews.seller_credibility_bps,
            value_weight_bps: memory::outcome_value_weight_bps(outcome),
            base_weight_bps: memory::outcome_base_weight_bps(outcome),
            prior_pair_reviews: memory::outcome_prior_pair_reviews(outcome),
            weight_bps: memory::outcome_weight_bps(outcome),
            reviewed_at_ms: clock::timestamp_ms(clock),
        });
    }

    fun assert_reviewable(task: &TaskEscrow, stars: u8, clock: &Clock) {
        assert!(task.status == STATUS_SETTLED, EWrongStatus);
        assert!(clock::timestamp_ms(clock) <= task.review_deadline_ms, EReviewWindowClosed);
        assert!(stars >= 1 && stars <= 5, EInvalidStars);
    }

    // ============================================================
    // Authorization helpers
    // ============================================================

    /// The task's buyer: its principal, or the buyer agent that created the offer, holding the
    /// cap and within its spend limit. A different agent of the same principal is rejected.
    fun resolve_buyer_actor(
        task: &TaskEscrow,
        memory_config: &MemoryConfig,
        buyer_account: &MemoryAccount,
        spend_amount: u64,
        clock: &Clock,
        ctx: &TxContext,
    ): ActingContext {
        assert!(object::id(buyer_account) == task.buyer_memory_account_id, EWrongTaskObject);
        let cap = memory::cap_task_trade();
        let acting = memory::resolve_actor_with_cap(
            memory_config,
            buyer_account,
            cap,
            option::none(),
            spend_amount,
            clock,
            ctx,
        );
        memory::assert_direct_execution_allowed(buyer_account, cap, ctx);
        assert!(memory::acting_principal_owner(&acting) == task.buyer_principal, ENotBuyer);
        let actor_agent = memory::acting_sub_agent_id(&acting);
        if (option::is_some(&actor_agent)) {
            assert!(task.buyer_agent_id == actor_agent, ENotBuyer);
        };
        acting
    }

    /// Seller principal, or the accepted seller agent holding the cap (for accept and decline).
    fun assert_seller_with_cap(
        task: &TaskEscrow,
        memory_config: &MemoryConfig,
        seller_account: &MemoryAccount,
        seller_agent: &SubAgent,
        clock: &Clock,
        ctx: &TxContext,
    ) {
        assert!(object::id(seller_agent) == task.seller_agent_id, EWrongTaskObject);
        assert!(object::id(seller_account) == task.seller_memory_account_id, EWrongTaskObject);
        let sender = tx_context::sender(ctx);
        if (sender == task.seller_principal) {
            memory::resolve_human_actor(seller_account, ctx);
        } else {
            assert!(sender == memory::sub_agent_derived_address(seller_agent), ENotSeller);
            let cap = memory::cap_task_trade();
            let acting = memory::resolve_actor_with_cap(
                memory_config,
                seller_account,
                cap,
                option::none(),
                0,
                clock,
                ctx,
            );
            memory::assert_direct_execution_allowed(seller_account, cap, ctx);
            assert!(
                option::contains(&memory::acting_sub_agent_id(&acting), &task.seller_agent_id),
                ENotSeller,
            );
        };
    }

    /// Seller principal or the seller agent itself, with no cap: used where no funds move to
    /// the seller (review, refund proposals).
    fun assert_seller_sender(task: &TaskEscrow, seller_agent: &SubAgent, ctx: &TxContext) {
        assert!(object::id(seller_agent) == task.seller_agent_id, EWrongTaskObject);
        let sender = tx_context::sender(ctx);
        assert!(
            sender == task.seller_principal || sender == memory::sub_agent_derived_address(seller_agent),
            ENotSeller,
        );
    }

    /// Which side the sender is on: seller (principal or agent) or buyer (with the cap).
    fun resolve_party(
        task: &TaskEscrow,
        memory_config: &MemoryConfig,
        buyer_account: &MemoryAccount,
        seller_agent: &SubAgent,
        clock: &Clock,
        ctx: &TxContext,
    ): u8 {
        let sender = tx_context::sender(ctx);
        if (sender == task.seller_principal
            || (object::id(seller_agent) == task.seller_agent_id
                && sender == memory::sub_agent_derived_address(seller_agent))) {
            assert_seller_sender(task, seller_agent, ctx);
            PARTY_SELLER
        } else {
            resolve_buyer_actor(task, memory_config, buyer_account, 0, clock, ctx);
            PARTY_BUYER
        }
    }

    // ============================================================
    // Terms digest
    // ============================================================

    /// sha3-256 over the BCS of the fields the seller must approve, in this order:
    /// buyer principal, seller agent id, amount, scope digest, accept/delivery/settle windows,
    /// platform fee bps, ecosystem fee bps, ecosystem recipient, platform id.
    public fun compute_terms_digest(
        buyer_principal: address,
        seller_agent_id: ID,
        amount_mist: u64,
        scope_digest: &vector<u8>,
        accept_window_ms: u64,
        delivery_window_ms: u64,
        settle_window_ms: u64,
        platform_fee_bps: u64,
        ecosystem_fee_bps: u64,
        ecosystem_fee_recipient: address,
        platform_id: &Option<address>,
    ): vector<u8> {
        let mut bytes = vector::empty<u8>();
        vector::append(&mut bytes, bcs::to_bytes(&buyer_principal));
        vector::append(&mut bytes, bcs::to_bytes(&seller_agent_id));
        vector::append(&mut bytes, bcs::to_bytes(&amount_mist));
        vector::append(&mut bytes, bcs::to_bytes(scope_digest));
        vector::append(&mut bytes, bcs::to_bytes(&accept_window_ms));
        vector::append(&mut bytes, bcs::to_bytes(&delivery_window_ms));
        vector::append(&mut bytes, bcs::to_bytes(&settle_window_ms));
        vector::append(&mut bytes, bcs::to_bytes(&platform_fee_bps));
        vector::append(&mut bytes, bcs::to_bytes(&ecosystem_fee_bps));
        vector::append(&mut bytes, bcs::to_bytes(&ecosystem_fee_recipient));
        vector::append(&mut bytes, bcs::to_bytes(platform_id));
        hash::sha3_256(bytes)
    }

    // ============================================================
    // Views
    // ============================================================

    public fun status_offered(): u8 { STATUS_OFFERED }
    public fun status_accepted(): u8 { STATUS_ACCEPTED }
    public fun status_delivered(): u8 { STATUS_DELIVERED }
    public fun status_settled(): u8 { STATUS_SETTLED }
    public fun status_refunded(): u8 { STATUS_REFUNDED }
    public fun status_cancelled(): u8 { STATUS_CANCELLED }

    public fun status(task: &TaskEscrow): u8 { task.status }
    public fun close_kind(task: &TaskEscrow): u8 { task.close_kind }
    public fun settlement_kind(task: &TaskEscrow): u8 { task.settlement_kind }
    public fun amount_mist(task: &TaskEscrow): u64 { task.amount_mist }
    public fun escrowed_mist(task: &TaskEscrow): u64 { balance::value(&task.escrowed) }
    public fun buyer_principal(task: &TaskEscrow): address { task.buyer_principal }
    public fun seller_principal(task: &TaskEscrow): address { task.seller_principal }
    public fun buyer_agent_id(task: &TaskEscrow): Option<ID> { task.buyer_agent_id }
    public fun seller_agent_id(task: &TaskEscrow): ID { task.seller_agent_id }
    public fun terms_digest(task: &TaskEscrow): vector<u8> { task.terms.terms_digest }
    public fun platform_fee_bps(task: &TaskEscrow): u64 { task.terms.platform_fee_bps }
    public fun ecosystem_fee_bps(task: &TaskEscrow): u64 { task.terms.ecosystem_fee_bps }
    public fun accept_deadline_ms(task: &TaskEscrow): u64 { task.accept_deadline_ms }
    public fun delivery_deadline_ms(task: &TaskEscrow): u64 { task.delivery_deadline_ms }
    public fun settle_deadline_ms(task: &TaskEscrow): u64 { task.settle_deadline_ms }
    public fun review_deadline_ms(task: &TaskEscrow): u64 { task.review_deadline_ms }
    public fun settled_at_ms(task: &TaskEscrow): u64 { task.settled_at_ms }
    public fun platform_fee_mist(task: &TaskEscrow): u64 { task.payout.platform_fee_mist }
    public fun ecosystem_fee_mist(task: &TaskEscrow): u64 { task.payout.ecosystem_fee_mist }
    public fun net_amount_mist(task: &TaskEscrow): u64 { task.payout.net_amount_mist }
    public fun refund_proposer(task: &TaskEscrow): u8 { task.refund_proposer }
    public fun buyer_credibility_bps(task: &TaskEscrow): u64 { task.reviews.buyer_credibility_bps }
    public fun seller_credibility_bps(task: &TaskEscrow): u64 { task.reviews.seller_credibility_bps }
    public fun buyer_review_stars(task: &TaskEscrow): u8 { task.reviews.buyer_review_stars }
    public fun seller_review_stars(task: &TaskEscrow): u8 { task.reviews.seller_review_stars }

    public fun config_active(config: &TaskConfig): bool { config.active }
    public fun config_release_grace_ms(config: &TaskConfig): u64 { config.release_grace_ms }
    public fun config_review_window_ms(config: &TaskConfig): u64 { config.review_window_ms }
    public fun config_min_escrow_mist(config: &TaskConfig): u64 { config.min_escrow_mist }
    public fun config_reputation(config: &TaskConfig): ReputationParams { config.reputation }

    // ============================================================
    // Test helpers
    // ============================================================

    #[test_only]
    public fun test_bootstrap(admin: address, clock: &Clock, ctx: &mut TxContext) {
        bootstrap_init(admin, clock, ctx);
    }

    #[test_only]
    public fun test_create_task_market_cap(ctx: &mut TxContext): TaskMarketCap {
        create_task_market_cap(ctx)
    }

    #[test_only]
    public fun error_wrong_status(): u64 { EWrongStatus }
    #[test_only]
    public fun error_too_early(): u64 { ETooEarly }
    #[test_only]
    public fun error_invalid_task_config(): u64 { EInvalidTaskConfig }
}
