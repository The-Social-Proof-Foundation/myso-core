// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

/// Agent task market: escrowed offers between a buyer and a named seller agent.
///
/// ```
/// create_task ─► OFFERED ─┬─ accept_task ─► ACCEPTED ─┬─ mark_delivered ─► DELIVERED ─┬─ settle_task ──┐
///                         ├─ cancel_offer ─► CANCELLED │                               ├─ release_task ─┴─► SETTLED
///                         ├─ decline_task ─► CANCELLED └─ refund_task ─► REFUNDED       └─ propose/accept_refund ─► REFUNDED
///                         └─ expire_offer ─► CANCELLED
/// DELIVERED ─► DISPUTED ─┬─ jury verdict ──────────────┬─► SETTLED (pay seller / split 50-50) or REFUNDED
///                        ├─ both parties agree ────────┤
///                        └─ jury failed ─► settlement window ─► neutral 50/50 split
/// SETTLED ─► review_seller / review_buyer_agent / review_buyer_principal (once per direction)
/// ```
///
/// Every state change asserts the expected `status` first, so settle, release and the refunds
/// race on one shared object and exactly one wins. There is **no unilateral buyer refund after
/// delivery**: the buyer settles (pays), a refund needs both parties, and an unattended delivered
/// task is released to the seller after `settle_window + release_grace`.
///
/// A delivered task can be disputed once (`STATUS_DISPUTED`). Whoever resolves it (a jury, both
/// parties, or the neutral split after a failed jury) can only pick one of three fixed outcomes:
/// pay the seller, refund the buyer, or split 50/50. A failed jury never returns the task to
/// DELIVERED (where `release_task` would pay the seller): it ends in the neutral split. Only the
/// initiator's cancel before any juror is drawn returns it to DELIVERED, with the release clock
/// re-based, and the task can never be disputed again.
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
        dynamic_field,
        event,
        myso::MYSO,
        object::{Self, UID, ID},
        transfer,
        tx_context::{Self, TxContext},
    };

    use social_contracts::jury::{Self, JuryCase, JuryConfig, JurorRoster};
    use social_contracts::memory::{Self, ActingContext, MemoryAccount, MemoryConfig, ReputationParams, SubAgent};
    use social_contracts::platform::{Self, Platform, PlatformAdminCap};
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
    const STATUS_DISPUTED: u8 = 6;

    /// `close_kind` of a cancelled offer.
    const CLOSE_BUYER_CANCELLED: u8 = 0;
    const CLOSE_SELLER_DECLINED: u8 = 1;
    const CLOSE_EXPIRED: u8 = 2;

    const SETTLED_BY_BUYER: u8 = 1;
    const SETTLED_AUTO_RELEASED: u8 = 2;
    /// Dispute resolutions. A split gives neither side a credibility credit and opens no reviews.
    const SETTLED_BY_VERDICT: u8 = 3;
    const SETTLED_BY_DISPUTE_AGREEMENT: u8 = 4;
    const SETTLED_SPLIT_BY_VERDICT: u8 = 5;
    const SETTLED_SPLIT_BY_AGREEMENT: u8 = 6;
    const SETTLED_BY_NEUTRAL_SPLIT: u8 = 7;

    /// `refund_kind`: the seller never delivered / both parties agreed after delivery.
    const REFUND_DELIVERY_MISSED: u8 = 0;
    const REFUND_MUTUAL: u8 = 1;
    const REFUND_BY_VERDICT: u8 = 2;
    const REFUND_BY_DISPUTE_AGREEMENT: u8 = 3;

    /// The fixed menu every dispute resolves into. Nothing outside it can move the escrow.
    const OUTCOME_PAY_SELLER: u8 = 0;
    const OUTCOME_REFUND_BUYER: u8 = 1;
    const OUTCOME_SPLIT_50_50: u8 = 2;
    const DISPUTE_MENU_LEN: u64 = 3;

    /// Who produced the outcome being applied.
    const SOURCE_VERDICT: u8 = 1;
    const SOURCE_AGREEMENT: u8 = 2;
    const SOURCE_NEUTRAL: u8 = 3;

    const DISPUTE_OPEN: u8 = 1;
    const DISPUTE_CANCELLED: u8 = 2;
    const DISPUTE_RESOLVED: u8 = 3;

    const DISPUTE_FIELD: vector<u8> = b"dispute";

    /// What a dispute costs each side, paid as a case fee (never a juror deposit): a share of the
    /// escrow, bounded. An appeal costs a multiple of it. Jurors share the pool; see `settle_dispute_fees`.
    const DISPUTE_FEE_BPS: u64 = 300;
    const DISPUTE_FEE_MIN_MIST: u64 = 100_000_000;
    const DISPUTE_FEE_MAX_MIST: u64 = 50_000_000_000;
    const APPEAL_FEE_MULTIPLIER: u64 = 2;
    const DISPUTE_EVIDENCE_WINDOW_MS: u64 = 2 * 86_400_000;
    const DISPUTE_QUORUM: u64 = 2;
    const DISPUTE_APPROVAL_BPS: u64 = 5000;
    /// Jurors in the first round and on appeal. The appeal jury is 5 until the roster has enough
    /// distinct owners for a larger one (each round needs fresh owners).
    const DISPUTE_FIRST_JURY: u64 = 3;
    const DISPUTE_APPEAL_JURY: u64 = 5;
    /// `JuryCase.failure_path`: a failed task jury goes to the neutral split.
    const FAILURE_PATH_NEUTRAL: u8 = 1;
    const EVIDENCE_KIND_TASK_RECORD: u8 = 1;
    const EVIDENCE_KIND_PARTY: u8 = 2;

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
    const EDisputeWindowClosed: u64 = 29;
    const EDisputeExists: u64 = 30;
    const ENoDispute: u64 = 31;
    const EDisputeNotOpen: u64 = 32;
    const EInvalidOutcome: u64 = 33;
    const EInvalidRole: u64 = 34;
    const ENeutralWindowNotOpen: u64 = 35;
    const ENeutralWindowOpen: u64 = 36;
    const EWrongCase: u64 = 37;
    const ENoVerdict: u64 = 38;
    const ECaseNotFailed: u64 = 39;
    const EAlreadyPaid: u64 = 40;
    const EFeesSettled: u64 = 41;
    const ECaseNotFinished: u64 = 42;
    const ENotTallied: u64 = 43;
    const ENotAppellant: u64 = 44;
    const ENotInitiator: u64 = 45;
    const ECaseDrawn: u64 = 46;

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

    /// Dispute state, kept as a dynamic field on the escrow (`DISPUTE_FIELD`) because the escrow
    /// is already at the verifier's 32-field limit. Never removed, so a task is disputed once.
    public struct TaskDispute has store, drop {
        case_id: ID,
        opened_by_role: u8,
        opened_at_ms: u64,
        state: u8,
        /// Set when the jury failed: the settlement window ends here, then the neutral split runs.
        neutral_window_ends_ms: u64,
        /// Pending mutual agreement: who proposed it (0 none) and which menu outcome.
        agreement_proposer: u8,
        agreement_outcome: u8,
        /// The case fee each side owes, what each has put in (appeals included), and how the dispute
        /// ended, so the pool can be settled once the jury case is over.
        fee_each: u64,
        buyer_paid: u64,
        seller_paid: u64,
        resolved_source: u8,
        resolved_outcome: u8,
        fees_settled: bool,
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

    /// Fee recipient changes apply only to offers created after this update. Every existing
    /// TaskEscrow retains the recipient captured in its TaskTerms.
    public struct TaskFeeRecipientChanged has copy, drop {
        previous_recipient: address,
        new_recipient: address,
        updated_by: address,
        updated_at_ms: u64,
    }

    public entry fun update_fee_recipient(
        _: &PlatformAdminCap,
        config: &mut TaskConfig,
        new_recipient: address,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(config.version == upgrade::current_version(), EWrongVersion);
        assert!(new_recipient != @0x0, EInvalidTaskConfig);
        let previous_recipient = config.ecosystem_fee_recipient;
        assert!(previous_recipient != new_recipient, EInvalidTaskConfig);
        config.ecosystem_fee_recipient = new_recipient;
        event::emit(TaskFeeRecipientChanged {
            previous_recipient,
            new_recipient,
            updated_by: tx_context::sender(ctx),
            updated_at_ms: clock::timestamp_ms(clock),
        });
    }

    // Dispute events. The jury module emits its own (roster, draw, votes, verdict); these carry the
    // task's side of the story so the indexer can follow one task end to end.

    public struct TaskDisputed has copy, drop {
        task_id: ID,
        case_id: ID,
        opened_by_role: u8,
        fee_each_mist: u64,
        opener_paid_mist: u64,
        opened_at_ms: u64,
    }

    /// `kind` 1: the other side matched the case fee. 2: an appeal fee.
    public struct TaskDisputeFeePaid has copy, drop {
        task_id: ID,
        role: u8,
        amount_mist: u64,
        kind: u8,
        buyer_paid_mist: u64,
        seller_paid_mist: u64,
        paid_at_ms: u64,
    }

    public struct TaskDisputeCancelled has copy, drop {
        task_id: ID,
        case_id: ID,
        settle_deadline_ms: u64,
        cancelled_at_ms: u64,
    }

    public struct TaskNeutralWindowOpened has copy, drop {
        task_id: ID,
        case_id: ID,
        window_ends_ms: u64,
        opened_at_ms: u64,
    }

    public struct TaskDisputeSettlementProposed has copy, drop {
        task_id: ID,
        proposer_role: u8,
        outcome: u8,
        proposed_at_ms: u64,
    }

    public struct TaskDisputeSettlementCleared has copy, drop {
        task_id: ID,
        cleared_by_role: u8,
        cleared_at_ms: u64,
    }

    /// How the escrow was paid out: the menu `outcome` and its `source` (1 verdict, 2 agreement,
    /// 3 neutral split). The payout itself is the existing `TaskReceiptSettled` or `TaskRefunded`.
    public struct TaskDisputeResolved has copy, drop {
        task_id: ID,
        case_id: ID,
        outcome: u8,
        source: u8,
        resolved_at_ms: u64,
    }

    public struct TaskDisputeFeesSettled has copy, drop {
        task_id: ID,
        case_id: ID,
        buyer_refund_mist: u64,
        seller_refund_mist: u64,
        settled_at_ms: u64,
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
            reputation: memory::default_reputation_params(),
        };
        validate_task_config(&config);
        emit_config_updated(&config, admin, clock);
        transfer::share_object(config);
    }

    public(package) fun create_task_market_cap(ctx: &mut TxContext): TaskMarketCap {
        TaskMarketCap { id: object::new(ctx) }
    }

    /// Replaces the whole configuration. Every bound is re-validated; a violation aborts. The
    /// reputation params are copied to the jury so jurors are scored with the same math.
    public entry fun set_task_config(
        _: &TaskMarketCap,
        config: &mut TaskConfig,
        jury_config: &mut JuryConfig,
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
        jury::sync_reputation_params(jury_config, config.reputation, clock, ctx);
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
        let payout = coin::from_balance(balance::withdraw_all(&mut task.escrowed), ctx);
        let basis = task.amount_mist;
        distribute_to_seller(task, platform, payout, basis, clock, ctx);
    }

    /// Buyer half back in full; the seller half pays the frozen fee bps on that half only (fees are
    /// charged on money actually paid out). The seller keeps the odd unit.
    fun pay_split(task: &mut TaskEscrow, platform: &mut Platform, clock: &Clock, ctx: &mut TxContext) {
        let mut all = coin::from_balance(balance::withdraw_all(&mut task.escrowed), ctx);
        let total = coin::value(&all);
        let buyer_half = total / 2;
        if (buyer_half > 0) {
            transfer::public_transfer(coin::split(&mut all, buyer_half, ctx), task.buyer_principal);
        };
        distribute_to_seller(task, platform, all, total - buyer_half, clock, ctx);
    }

    /// Fees are `basis * bps`; `payout` is whatever the seller side received (the whole escrow, or
    /// the seller half of a split).
    fun distribute_to_seller(
        task: &mut TaskEscrow,
        platform: &mut Platform,
        mut payout: Coin<MYSO>,
        basis: u64,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        let platform_share = memory::checked_bps_of(basis, task.terms.platform_fee_bps);
        let ecosystem_share = memory::checked_bps_of(basis, task.terms.ecosystem_fee_bps);
        assert!(memory::checked_add_u64(platform_share, ecosystem_share) <= basis, EInvalidTaskConfig);

        let has_platform = option::is_some(&task.platform_id);
        let platform_fee = if (has_platform) { platform_share } else { 0 };
        let ecosystem_fee = if (has_platform) {
            ecosystem_share
        } else {
            memory::checked_add_u64(platform_share, ecosystem_share)
        };

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
    // Disputes
    // ============================================================

    /// Freezes a delivered task for a jury. Package-only: the jury integration calls it after it
    /// has opened the case. Allowed once per task, and only until `settle_deadline + release_grace`
    /// (after that anyone can already release the task to the seller).
    public(package) fun enter_dispute(
        config: &TaskConfig,
        task: &mut TaskEscrow,
        case_id: ID,
        opened_by_role: u8,
        fee_each: u64,
        opener_paid: u64,
        clock: &Clock,
    ) {
        assert!(config.version == upgrade::current_version(), EWrongVersion);
        assert!(task.status == STATUS_DELIVERED, EWrongStatus);
        assert!(opened_by_role == PARTY_BUYER || opened_by_role == PARTY_SELLER, EInvalidRole);
        assert!(!dynamic_field::exists_(&task.id, DISPUTE_FIELD), EDisputeExists);
        let now = clock::timestamp_ms(clock);
        let window_end = memory::checked_add_u64(task.settle_deadline_ms, config.release_grace_ms);
        assert!(now <= window_end, EDisputeWindowClosed);

        task.status = STATUS_DISPUTED;
        // A pending refund proposal is void once a jury has the task.
        task.refund_proposer = 0;
        task.refund_proposed_at_ms = 0;
        dynamic_field::add(
            &mut task.id,
            DISPUTE_FIELD,
            TaskDispute {
                case_id,
                opened_by_role,
                opened_at_ms: now,
                state: DISPUTE_OPEN,
                neutral_window_ends_ms: 0,
                agreement_proposer: 0,
                agreement_outcome: 0,
                fee_each,
                buyer_paid: if (opened_by_role == PARTY_BUYER) { opener_paid } else { 0 },
                seller_paid: if (opened_by_role == PARTY_SELLER) { opener_paid } else { 0 },
                resolved_source: 0,
                resolved_outcome: 0,
                fees_settled: false,
            },
        );
        event::emit(TaskDisputed {
            task_id: object::id(task),
            case_id,
            opened_by_role,
            fee_each_mist: fee_each,
            opener_paid_mist: opener_paid,
            opened_at_ms: now,
        });
    }

    /// The initiator withdraws before any juror is drawn (the jury integration enforces that). The
    /// task returns to DELIVERED with the release clock re-based, so the seller cannot collect
    /// earlier than `release_grace` after the cancel; the dispute record stays, so it cannot be
    /// repeated to restart the clock.
    public(package) fun cancel_dispute(task: &mut TaskEscrow, clock: &Clock) {
        assert!(task.status == STATUS_DISPUTED, EWrongStatus);
        let now = clock::timestamp_ms(clock);
        let case_id = {
            let dispute = dispute_mut(task);
            assert!(dispute.state == DISPUTE_OPEN && dispute.neutral_window_ends_ms == 0, EDisputeNotOpen);
            dispute.state = DISPUTE_CANCELLED;
            dispute.agreement_proposer = 0;
            dispute.agreement_outcome = 0;
            dispute.case_id
        };
        task.status = STATUS_DELIVERED;
        if (task.settle_deadline_ms < now) {
            task.settle_deadline_ms = now;
        };
        event::emit(TaskDisputeCancelled {
            task_id: object::id(task),
            case_id,
            settle_deadline_ms: task.settle_deadline_ms,
            cancelled_at_ms: now,
        });
    }

    /// The jury failed (could not form, could not decide, or ran out of time). Opens the
    /// settlement window; the escrow stays frozen until a neutral split or an agreement.
    public(package) fun begin_neutral_window(task: &mut TaskEscrow, window_ms: u64, clock: &Clock) {
        assert!(task.status == STATUS_DISPUTED, EWrongStatus);
        assert!(window_ms > 0, EInvalidWindow);
        let now = clock::timestamp_ms(clock);
        let ends = memory::checked_add_u64(now, window_ms);
        let task_id = object::id(task);
        let dispute = dispute_mut(task);
        assert!(dispute.state == DISPUTE_OPEN, EDisputeNotOpen);
        assert!(dispute.neutral_window_ends_ms == 0, ENeutralWindowOpen);
        dispute.neutral_window_ends_ms = ends;
        event::emit(TaskNeutralWindowOpened {
            task_id,
            case_id: dispute.case_id,
            window_ends_ms: ends,
            opened_at_ms: now,
        });
    }

    /// After a failed jury and its settlement window, anyone splits the escrow 50/50. Never pays
    /// the seller in full and never refunds the buyer in full.
    public entry fun execute_neutral_resolution(
        config: &TaskConfig,
        task: &mut TaskEscrow,
        buyer_account: &mut MemoryAccount,
        seller_account: &mut MemoryAccount,
        platform: &mut Platform,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(config.version == upgrade::current_version(), EWrongVersion);
        assert!(task.status == STATUS_DISPUTED, EWrongStatus);
        let ends = dispute_ref(task).neutral_window_ends_ms;
        assert!(ends != 0, ENeutralWindowNotOpen);
        assert!(clock::timestamp_ms(clock) > ends, ETooEarly);
        apply_dispute_outcome(
            config, task, buyer_account, seller_account, platform,
            OUTCOME_SPLIT_50_50, SOURCE_NEUTRAL, clock, ctx,
        );
    }

    /// Either party proposes how the disputed escrow resolves, one of the three menu outcomes.
    /// A party can refresh its own proposal but cannot override the counterparty's.
    public entry fun propose_dispute_settlement(
        task: &mut TaskEscrow,
        memory_config: &MemoryConfig,
        buyer_account: &MemoryAccount,
        seller_agent: &SubAgent,
        outcome: u8,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(task.status == STATUS_DISPUTED, EWrongStatus);
        assert!(outcome <= OUTCOME_SPLIT_50_50, EInvalidOutcome);
        let role = resolve_party(task, memory_config, buyer_account, seller_agent, clock, ctx);
        let task_id = object::id(task);
        let dispute = dispute_mut(task);
        assert!(dispute.state == DISPUTE_OPEN, EDisputeNotOpen);
        assert!(dispute.agreement_proposer == 0 || dispute.agreement_proposer == role, EProposalPending);
        dispute.agreement_proposer = role;
        dispute.agreement_outcome = outcome;
        event::emit(TaskDisputeSettlementProposed {
            task_id,
            proposer_role: role,
            outcome,
            proposed_at_ms: clock::timestamp_ms(clock),
        });
    }

    /// The counterparty accepts the pending proposal and the escrow resolves as proposed. Works
    /// before and during the settlement window, so parties can settle at any point.
    public entry fun accept_dispute_settlement(
        config: &TaskConfig,
        task: &mut TaskEscrow,
        memory_config: &MemoryConfig,
        buyer_account: &mut MemoryAccount,
        seller_account: &mut MemoryAccount,
        seller_agent: &SubAgent,
        platform: &mut Platform,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(config.version == upgrade::current_version(), EWrongVersion);
        assert!(task.status == STATUS_DISPUTED, EWrongStatus);
        let role = resolve_party(task, memory_config, buyer_account, seller_agent, clock, ctx);
        let (proposer, outcome) = {
            let dispute = dispute_ref(task);
            assert!(dispute.state == DISPUTE_OPEN, EDisputeNotOpen);
            (dispute.agreement_proposer, dispute.agreement_outcome)
        };
        assert!(proposer != 0, ENoProposal);
        assert!(role != proposer, ENotCounterparty);
        apply_dispute_outcome(
            config, task, buyer_account, seller_account, platform,
            outcome, SOURCE_AGREEMENT, clock, ctx,
        );
    }

    /// The proposer withdraws its proposal, or the counterparty turns it down.
    public entry fun clear_dispute_settlement(
        task: &mut TaskEscrow,
        memory_config: &MemoryConfig,
        buyer_account: &MemoryAccount,
        seller_agent: &SubAgent,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(task.status == STATUS_DISPUTED, EWrongStatus);
        let role = resolve_party(task, memory_config, buyer_account, seller_agent, clock, ctx);
        let task_id = object::id(task);
        let dispute = dispute_mut(task);
        assert!(dispute.state == DISPUTE_OPEN, EDisputeNotOpen);
        assert!(dispute.agreement_proposer != 0, ENoProposal);
        dispute.agreement_proposer = 0;
        dispute.agreement_outcome = 0;
        event::emit(TaskDisputeSettlementCleared {
            task_id,
            cleared_by_role: role,
            cleared_at_ms: clock::timestamp_ms(clock),
        });
    }

    /// The single place a dispute pays out. A jury verdict, an agreement and the neutral split all
    /// land here, so each can only choose from the fixed menu. `source` only selects the kind
    /// recorded on the receipt or refund; a neutral source may only be the 50/50 split.
    public(package) fun apply_dispute_outcome(
        config: &TaskConfig,
        task: &mut TaskEscrow,
        buyer_account: &mut MemoryAccount,
        seller_account: &mut MemoryAccount,
        platform: &mut Platform,
        outcome: u8,
        source: u8,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(task.status == STATUS_DISPUTED, EWrongStatus);
        assert!(outcome <= OUTCOME_SPLIT_50_50, EInvalidOutcome);
        assert!(source >= SOURCE_VERDICT && source <= SOURCE_NEUTRAL, EInvalidOutcome);
        assert!(source != SOURCE_NEUTRAL || outcome == OUTCOME_SPLIT_50_50, EInvalidOutcome);
        let case_id = {
            let dispute = dispute_mut(task);
            assert!(dispute.state == DISPUTE_OPEN, EDisputeNotOpen);
            dispute.state = DISPUTE_RESOLVED;
            dispute.agreement_proposer = 0;
            dispute.agreement_outcome = 0;
            dispute.resolved_source = source;
            dispute.resolved_outcome = outcome;
            dispute.case_id
        };
        event::emit(TaskDisputeResolved {
            task_id: object::id(task),
            case_id,
            outcome,
            source,
            resolved_at_ms: clock::timestamp_ms(clock),
        });

        if (outcome == OUTCOME_REFUND_BUYER) {
            let refund_kind = if (source == SOURCE_AGREEMENT) {
                REFUND_BY_DISPUTE_AGREEMENT
            } else {
                REFUND_BY_VERDICT
            };
            close_as_refunded(task, refund_kind, clock, ctx);
        } else if (outcome == OUTCOME_PAY_SELLER) {
            let kind = if (source == SOURCE_AGREEMENT) { SETTLED_BY_DISPUTE_AGREEMENT } else { SETTLED_BY_VERDICT };
            finalize_settlement(config, task, buyer_account, seller_account, platform, kind, clock, ctx);
        } else {
            let kind = if (source == SOURCE_NEUTRAL) {
                SETTLED_BY_NEUTRAL_SPLIT
            } else if (source == SOURCE_AGREEMENT) {
                SETTLED_SPLIT_BY_AGREEMENT
            } else {
                SETTLED_SPLIT_BY_VERDICT
            };
            settle_split(task, buyer_account, seller_account, platform, kind, clock, ctx);
        };
    }

    /// A split settles the task without crediting either side's reputation (no `record_settlement`)
    /// and with the review window closed (`review_deadline_ms = 0`).
    fun settle_split(
        task: &mut TaskEscrow,
        buyer_account: &MemoryAccount,
        seller_account: &MemoryAccount,
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

        pay_split(task, platform, clock, ctx);

        let now = clock::timestamp_ms(clock);
        task.status = STATUS_SETTLED;
        task.settlement_kind = kind;
        task.settled_at_ms = now;
        task.review_deadline_ms = 0;

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
            review_deadline_ms: 0,
            settled_at_ms: now,
        });
    }

    fun dispute_ref(task: &TaskEscrow): &TaskDispute {
        assert!(dynamic_field::exists_(&task.id, DISPUTE_FIELD), ENoDispute);
        dynamic_field::borrow(&task.id, DISPUTE_FIELD)
    }

    fun dispute_mut(task: &mut TaskEscrow): &mut TaskDispute {
        assert!(dynamic_field::exists_(&task.id, DISPUTE_FIELD), ENoDispute);
        dynamic_field::borrow_mut(&mut task.id, DISPUTE_FIELD)
    }

    // ============================================================
    // Disputes in front of a jury
    // ============================================================

    /// The fee each side owes for a dispute on an escrow of `amount_mist`.
    public fun dispute_fee_mist(amount_mist: u64): u64 {
        let raw = memory::checked_bps_of(amount_mist, DISPUTE_FEE_BPS);
        if (raw < DISPUTE_FEE_MIN_MIST) {
            DISPUTE_FEE_MIN_MIST
        } else if (raw > DISPUTE_FEE_MAX_MIST) {
            DISPUTE_FEE_MAX_MIST
        } else {
            raw
        }
    }

    /// A party takes a delivered task to a jury. The opener pays the case fee; the other side may
    /// match it before the jury is drawn. The case carries the task's own facts as its first
    /// evidence (terms, scope and deliverable digests), so neither side can leave them out. Refused
    /// while the roster is too thin to be fair.
    public entry fun open_dispute(
        config: &TaskConfig,
        jury_config: &JuryConfig,
        roster: &JurorRoster,
        task: &mut TaskEscrow,
        memory_config: &MemoryConfig,
        buyer_account: &MemoryAccount,
        seller_account: &MemoryAccount,
        seller_agent: &SubAgent,
        mut fee: Coin<MYSO>,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(config.version == upgrade::current_version(), EWrongVersion);
        assert!(task.status == STATUS_DELIVERED, EWrongStatus);
        assert!(object::id(seller_account) == task.seller_memory_account_id, EWrongTaskObject);
        let role = resolve_party(task, memory_config, buyer_account, seller_agent, clock, ctx);

        let owners = vector[task.buyer_principal, task.seller_principal];
        let mut orgs = vector[memory::sub_agent_organization_id(seller_agent)];
        if (option::is_some(&task.organization_id)) {
            vector::push_back(&mut orgs, *option::borrow(&task.organization_id));
        };
        let mut agents = vector[task.seller_agent_id];
        vector::append(&mut agents, memory::agent_ancestor_ids(memory_config, seller_account, task.seller_agent_id));
        if (option::is_some(&task.buyer_agent_id)) {
            let buyer_agent = *option::borrow(&task.buyer_agent_id);
            vector::push_back(&mut agents, buyer_agent);
            vector::append(&mut agents, memory::agent_ancestor_ids(memory_config, buyer_account, buyer_agent));
        };
        let round_sizes = vector[DISPUTE_FIRST_JURY, DISPUTE_APPEAL_JURY];
        jury::assert_case_formable(jury_config, roster, &owners, &orgs, &agents, &round_sizes, clock);

        let fee_each = dispute_fee_mist(task.amount_mist);
        assert!(coin::value(&fee) >= fee_each, EInsufficientPayment);
        let payment = coin::split(&mut fee, fee_each, ctx);
        if (coin::value(&fee) > 0) {
            transfer::public_transfer(fee, tx_context::sender(ctx));
        } else {
            coin::destroy_zero(fee);
        };

        let mut case = jury::open_case(
            jury_config,
            jury::subject_task(),
            object::id(task),
            0,
            3,
            DISPUTE_APPROVAL_BPS,
            DISPUTE_QUORUM,
            round_sizes,
            true,
            FAILURE_PATH_NEUTRAL,
            DISPUTE_EVIDENCE_WINDOW_MS,
            owners,
            orgs,
            agents,
            coin::into_balance(payment),
            fee_each / DISPUTE_FIRST_JURY,
            object::uid_to_address(&task.id),
            clock,
            ctx,
        );
        let facts = object::uid_to_address(&task.id);
        jury::add_evidence(jury_config, &mut case, facts, EVIDENCE_KIND_TASK_RECORD, task.terms.terms_digest, b"agent_task:terms", clock);
        jury::add_evidence(jury_config, &mut case, facts, EVIDENCE_KIND_TASK_RECORD, task.scope_digest, b"agent_task:scope", clock);
        jury::add_evidence(
            jury_config,
            &mut case,
            facts,
            EVIDENCE_KIND_TASK_RECORD,
            *option::borrow(&task.deliverable_digest),
            *string::as_bytes(option::borrow(&task.deliverable_uri)),
            clock,
        );
        let case_id = jury::share_case(case);
        enter_dispute(config, task, case_id, role, fee_each, fee_each, clock);
    }

    /// The other side matches the case fee. Only before the jury is drawn, so the pool is fixed
    /// before anyone is seated.
    public entry fun respond_dispute(
        task: &mut TaskEscrow,
        case: &mut JuryCase,
        memory_config: &MemoryConfig,
        buyer_account: &MemoryAccount,
        seller_agent: &SubAgent,
        mut fee: Coin<MYSO>,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(task.status == STATUS_DISPUTED, EWrongStatus);
        assert_case_matches(task, case);
        let role = resolve_party(task, memory_config, buyer_account, seller_agent, clock, ctx);
        assert!(jury::case_undrawn(case), ECaseDrawn);
        let (opener, fee_each, already_paid) = {
            let dispute = dispute_ref(task);
            assert!(dispute.state == DISPUTE_OPEN, EDisputeNotOpen);
            (
                dispute.opened_by_role,
                dispute.fee_each,
                if (role == PARTY_BUYER) { dispute.buyer_paid } else { dispute.seller_paid },
            )
        };
        assert!(role != opener, ENotCounterparty);
        assert!(already_paid == 0, EAlreadyPaid);
        assert!(coin::value(&fee) >= fee_each, EInsufficientPayment);
        let payment = coin::split(&mut fee, fee_each, ctx);
        if (coin::value(&fee) > 0) {
            transfer::public_transfer(fee, tx_context::sender(ctx));
        } else {
            coin::destroy_zero(fee);
        };
        jury::add_fee(case, coin::into_balance(payment));
        record_paid(task, role, fee_each, 1, clock);
    }

    /// A party adds evidence while the evidence window is open (and for an appeal round, before its
    /// draw). The jury module caps how much each side may add.
    public entry fun submit_dispute_evidence(
        jury_config: &JuryConfig,
        task: &mut TaskEscrow,
        case: &mut JuryCase,
        memory_config: &MemoryConfig,
        buyer_account: &MemoryAccount,
        seller_agent: &SubAgent,
        digest: vector<u8>,
        uri: vector<u8>,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(task.status == STATUS_DISPUTED, EWrongStatus);
        assert_case_matches(task, case);
        let role = resolve_party(task, memory_config, buyer_account, seller_agent, clock, ctx);
        let submitter = if (role == PARTY_BUYER) { task.buyer_principal } else { task.seller_principal };
        jury::add_evidence(jury_config, case, submitter, EVIDENCE_KIND_PARTY, digest, uri, clock);
    }

    /// The side the verdict went against (either side, if it was a split) asks for a larger jury of
    /// fresh owners. It pays a multiple of the case fee, which joins the pool.
    public entry fun appeal_dispute(
        task: &mut TaskEscrow,
        case: &mut JuryCase,
        memory_config: &MemoryConfig,
        buyer_account: &MemoryAccount,
        seller_agent: &SubAgent,
        mut fee: Coin<MYSO>,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(task.status == STATUS_DISPUTED, EWrongStatus);
        assert_case_matches(task, case);
        let role = resolve_party(task, memory_config, buyer_account, seller_agent, clock, ctx);
        assert!(jury::case_state(case) == jury::case_tallied(), ENotTallied);
        let outcome = *option::borrow(&jury::case_provisional_outcome(case));
        let lost = if (outcome == OUTCOME_PAY_SELLER) {
            role == PARTY_BUYER
        } else if (outcome == OUTCOME_REFUND_BUYER) {
            role == PARTY_SELLER
        } else {
            true
        };
        assert!(lost, ENotAppellant);
        let appeal_fee = memory::checked_mul_u64(dispute_ref(task).fee_each, APPEAL_FEE_MULTIPLIER);
        assert!(coin::value(&fee) >= appeal_fee, EInsufficientPayment);
        let payment = coin::split(&mut fee, appeal_fee, ctx);
        if (coin::value(&fee) > 0) {
            transfer::public_transfer(fee, tx_context::sender(ctx));
        } else {
            coin::destroy_zero(fee);
        };
        jury::add_fee(case, coin::into_balance(payment));
        record_paid(task, role, appeal_fee, 2, clock);
        jury::appeal(case, tx_context::sender(ctx), clock);
    }

    /// The opener withdraws before any juror is drawn. Fees come back through `settle_dispute_fees`.
    public entry fun cancel_dispute_before_draw(
        task: &mut TaskEscrow,
        case: &mut JuryCase,
        memory_config: &MemoryConfig,
        buyer_account: &MemoryAccount,
        seller_agent: &SubAgent,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(task.status == STATUS_DISPUTED, EWrongStatus);
        assert_case_matches(task, case);
        let role = resolve_party(task, memory_config, buyer_account, seller_agent, clock, ctx);
        assert!(role == dispute_ref(task).opened_by_role, ENotInitiator);
        jury::cancel_case(case, clock);
        cancel_dispute(task, clock);
    }

    /// Anyone applies a final verdict: the jury's outcome index runs the one fixed action it names.
    public entry fun execute_verdict(
        config: &TaskConfig,
        task: &mut TaskEscrow,
        case: &mut JuryCase,
        buyer_account: &mut MemoryAccount,
        seller_account: &mut MemoryAccount,
        platform: &mut Platform,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(config.version == upgrade::current_version(), EWrongVersion);
        assert!(task.status == STATUS_DISPUTED, EWrongStatus);
        assert_case_matches(task, case);
        let verdict = jury::finalized_outcome(case);
        assert!(option::is_some(&verdict), ENoVerdict);
        jury::mark_executed(case);
        apply_dispute_outcome(
            config, task, buyer_account, seller_account, platform,
            *option::borrow(&verdict), SOURCE_VERDICT, clock, ctx,
        );
    }

    /// Anyone, once the jury failed (could not form, could not decide, timed out, or was voided):
    /// opens the settlement window. Nothing is paid to either side by default.
    public entry fun start_neutral_resolution(
        jury_config: &JuryConfig,
        task: &mut TaskEscrow,
        case: &mut JuryCase,
        clock: &Clock,
    ) {
        assert!(task.status == STATUS_DISPUTED, EWrongStatus);
        assert_case_matches(task, case);
        assert!(jury::case_state(case) == jury::case_failed(), ECaseNotFailed);
        jury::mark_executed(case);
        begin_neutral_window(task, jury::neutral_window_ms(jury_config), clock);
    }

    /// After the parties settled (or the dispute was cancelled) while the jury case is still open:
    /// anyone ends the case so its seats are freed and its fee can be settled.
    public entry fun close_case_after_resolution(
        jury_config: &JuryConfig,
        roster: &mut JurorRoster,
        task: &TaskEscrow,
        case: &mut JuryCase,
        clock: &Clock,
    ) {
        assert!(task.status != STATUS_DISPUTED, EWrongStatus);
        assert_case_matches(task, case);
        assert!(!jury::case_is_finished(case), ECaseNotFinished);
        jury::end_case_by_settlement(jury_config, roster, case, clock);
    }

    /// Anyone, once the dispute is over and the jury case is finished: jurors keep their shares,
    /// and what is left goes back to the parties. After a verdict the loser pays: the winner's own
    /// fee comes back first and the rest returns to the loser. After a split verdict, an agreement,
    /// a cancel or a failed jury there is no loser, so the rest returns in proportion to what each
    /// side put in (an equal share of the juror cost).
    public entry fun settle_dispute_fees(task: &mut TaskEscrow, case: &mut JuryCase, clock: &Clock, ctx: &mut TxContext) {
        assert!(task.status != STATUS_DISPUTED, EWrongStatus);
        assert_case_matches(task, case);
        assert!(jury::case_is_finished(case), ECaseNotFinished);
        let (buyer_paid, seller_paid, source, outcome) = {
            let dispute = dispute_mut(task);
            assert!(dispute.state != DISPUTE_OPEN, EDisputeNotOpen);
            assert!(!dispute.fees_settled, EFeesSettled);
            dispute.fees_settled = true;
            (dispute.buyer_paid, dispute.seller_paid, dispute.resolved_source, dispute.resolved_outcome)
        };

        let remainder = jury::sweep_remainder(case);
        let total = balance::value(&remainder);
        let paid_total = memory::checked_add_u64(buyer_paid, seller_paid);
        let buyer_share = if (source == SOURCE_VERDICT && outcome != OUTCOME_SPLIT_50_50) {
            let buyer_won = outcome == OUTCOME_REFUND_BUYER;
            let (winner_paid, loser_paid) = if (buyer_won) { (buyer_paid, seller_paid) } else { (seller_paid, buyer_paid) };
            let winner_back = if (winner_paid < total) { winner_paid } else { total };
            let rest = total - winner_back;
            // The rest goes to the loser, or to the winner if the loser never paid anything in.
            let loser_back = if (loser_paid > 0) { rest } else { 0 };
            let winner_total = winner_back + (rest - loser_back);
            if (buyer_won) { winner_total } else { loser_back }
        } else if (paid_total == 0) {
            total
        } else {
            memory::checked_narrow_u64(
                memory::checked_div_u128(
                    memory::checked_mul_u128(total as u128, buyer_paid as u128),
                    paid_total as u128,
                ),
            )
        };

        event::emit(TaskDisputeFeesSettled {
            task_id: object::id(task),
            case_id: object::id(case),
            buyer_refund_mist: buyer_share,
            seller_refund_mist: total - buyer_share,
            settled_at_ms: clock::timestamp_ms(clock),
        });
        let mut pool = coin::from_balance(remainder, ctx);
        if (buyer_share > 0) {
            transfer::public_transfer(coin::split(&mut pool, buyer_share, ctx), task.buyer_principal);
        };
        if (coin::value(&pool) > 0) {
            transfer::public_transfer(pool, task.seller_principal);
        } else {
            coin::destroy_zero(pool);
        };
    }

    fun record_paid(task: &mut TaskEscrow, role: u8, amount: u64, kind: u8, clock: &Clock) {
        let task_id = object::id(task);
        let dispute = dispute_mut(task);
        if (role == PARTY_BUYER) {
            dispute.buyer_paid = memory::checked_add_u64(dispute.buyer_paid, amount);
        } else {
            dispute.seller_paid = memory::checked_add_u64(dispute.seller_paid, amount);
        };
        event::emit(TaskDisputeFeePaid {
            task_id,
            role,
            amount_mist: amount,
            kind,
            buyer_paid_mist: dispute.buyer_paid,
            seller_paid_mist: dispute.seller_paid,
            paid_at_ms: clock::timestamp_ms(clock),
        });
    }

    /// The case must be this task's own task-subject case, and the one the dispute recorded.
    fun assert_case_matches(task: &TaskEscrow, case: &JuryCase) {
        assert!(jury::case_subject_kind(case) == jury::subject_task(), EWrongCase);
        assert!(jury::case_subject_id(case) == object::id(task), EWrongCase);
        assert!(dispute_ref(task).case_id == object::id(case), EWrongCase);
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
    public fun status_disputed(): u8 { STATUS_DISPUTED }

    public fun outcome_pay_seller(): u8 { OUTCOME_PAY_SELLER }
    public fun outcome_refund_buyer(): u8 { OUTCOME_REFUND_BUYER }
    public fun outcome_split_50_50(): u8 { OUTCOME_SPLIT_50_50 }
    public fun dispute_menu_len(): u64 { DISPUTE_MENU_LEN }
    public fun source_verdict(): u8 { SOURCE_VERDICT }
    public fun source_agreement(): u8 { SOURCE_AGREEMENT }
    public fun source_neutral(): u8 { SOURCE_NEUTRAL }

    public fun status(task: &TaskEscrow): u8 { task.status }
    public fun close_kind(task: &TaskEscrow): u8 { task.close_kind }
    public fun settlement_kind(task: &TaskEscrow): u8 { task.settlement_kind }
    public fun amount_mist(task: &TaskEscrow): u64 { task.amount_mist }
    public fun escrowed_mist(task: &TaskEscrow): u64 { balance::value(&task.escrowed) }
    public fun buyer_principal(task: &TaskEscrow): address { task.buyer_principal }
    public fun seller_principal(task: &TaskEscrow): address { task.seller_principal }
    public fun buyer_agent_id(task: &TaskEscrow): Option<ID> { task.buyer_agent_id }
    public fun seller_agent_id(task: &TaskEscrow): ID { task.seller_agent_id }
    /// The messaging group pinned to this task, when task coordination uses a private room.
    public fun room_group_id(task: &TaskEscrow): Option<ID> { task.room_group_id }
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

    public fun has_dispute(task: &TaskEscrow): bool { dynamic_field::exists_(&task.id, DISPUTE_FIELD) }
    public fun dispute_case_id(task: &TaskEscrow): Option<ID> {
        if (has_dispute(task)) { option::some(dispute_ref(task).case_id) } else { option::none() }
    }
    /// 0 when the task was never disputed; otherwise open (1), cancelled (2) or resolved (3).
    public fun dispute_state(task: &TaskEscrow): u8 {
        if (has_dispute(task)) { dispute_ref(task).state } else { 0 }
    }
    public fun dispute_neutral_window_ends_ms(task: &TaskEscrow): u64 { dispute_ref(task).neutral_window_ends_ms }
    public fun dispute_agreement_proposer(task: &TaskEscrow): u8 { dispute_ref(task).agreement_proposer }
    public fun dispute_agreement_outcome(task: &TaskEscrow): u8 { dispute_ref(task).agreement_outcome }
    public fun dispute_fee_each_mist(task: &TaskEscrow): u64 { dispute_ref(task).fee_each }
    public fun dispute_buyer_paid_mist(task: &TaskEscrow): u64 { dispute_ref(task).buyer_paid }
    public fun dispute_seller_paid_mist(task: &TaskEscrow): u64 { dispute_ref(task).seller_paid }
    public fun dispute_fees_settled(task: &TaskEscrow): bool { dispute_ref(task).fees_settled }

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
