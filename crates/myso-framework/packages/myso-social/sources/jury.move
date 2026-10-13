// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

/// AI jury engine. A free roster of willing agents, a random reputation-gated draw, hidden
/// commit-reveal votes, round tallies with escalation, and equal pay for the jurors who took part.
///
/// ```
/// open_case ─► EVIDENCE ─► AWAITING_DRAW ─select_jury─► SELECTING ─► VOTING ─finalize_round─┬─► FINAL
///                                ▲                          │ replace_no_shows              ├─► TALLIED ─(appeal)─┐
///                                └──── next round ◄─────────┴───────────────────────────────┤                    │
///                                                                                            └─► FAILED           ▼
///                                                                                       AWAITING_DRAW (round + 1)
/// ```
///
/// This module is a leaf: it knows nothing about tasks or proposals. An integrator (`agent_task`,
/// `governance`) opens a case with a fixed outcome menu (just `menu_len` indexes), adds evidence and
/// the fee, and later reads `final_outcome` to run one fixed action. The jury never picks a recipient
/// or an amount. Jurors hold no stake: the only thing at risk is `duty_score` and roster standing,
/// and they are paid an equal share of the case fee for taking part, never for how they voted.
///
/// A juror vote is a commitment `sha3_256(case_id ‖ round ‖ agent_id ‖ outcome ‖ rationale_digest ‖
/// bcs(evidence_refs) ‖ bundle_digest ‖ salt)` (see [`compute_commitment`]). Binding the agent id and
/// the case id means a copied commitment can never be revealed by anyone else.
#[allow(duplicate_alias, lint(public_entry), unused_const, unused_use)]
module social_contracts::jury {
    use std::hash;
    use std::option::{Self, Option};
    use std::vector;

    use myso::{
        balance::{Self, Balance},
        bcs,
        clock::{Self, Clock},
        coin,
        event,
        myso::MYSO,
        object::{Self, UID, ID},
        random::{Self, Random, RandomGenerator},
        table::{Self, Table},
        transfer,
        tx_context::{Self, TxContext},
        vec_set::{Self, VecSet},
    };

    use social_contracts::memory::{Self, MemoryAccount, MemoryConfig, ReputationParams, SubAgent};
    use social_contracts::upgrade::{Self, UpgradeAdminCap};

    // ============================================================
    // Constants
    // ============================================================

    const SUBJECT_TASK: u8 = 1;
    const SUBJECT_GOVERNANCE: u8 = 2;

    const CASE_EVIDENCE: u8 = 1;
    const CASE_AWAITING_DRAW: u8 = 2;
    const CASE_SELECTING: u8 = 3;
    const CASE_VOTING: u8 = 4;
    const CASE_TALLIED: u8 = 5;
    const CASE_FINAL: u8 = 6;
    const CASE_FAILED: u8 = 7;
    const CASE_CANCELLED: u8 = 8;

    const FAIL_NONE: u8 = 0;
    const FAIL_CANNOT_FORM: u8 = 1;
    const FAIL_CANNOT_DECIDE: u8 = 2;
    const FAIL_TIMEOUT: u8 = 3;
    const FAIL_VOIDED: u8 = 4;

    const SLOT_DRAFTED: u8 = 1;
    const SLOT_ACCEPTED: u8 = 2;
    const SLOT_COMMITTED: u8 = 3;
    const SLOT_REVEALED: u8 = 4;
    const SLOT_REPLACED: u8 = 5;
    const SLOT_DEFAULTED: u8 = 6;

    const ROSTER_ACTIVE: u8 = 1;
    const ROSTER_PAUSED: u8 = 2;
    const ROSTER_SUSPENDED: u8 = 3;

    /// `DutyScoreChanged.reason`.
    const DUTY_COMPLETED: u8 = 1;
    const DUTY_NO_ACCEPT: u8 = 2;
    const DUTY_NO_REVEAL: u8 = 3;

    /// A vote that backs no option. It counts toward quorum, never toward an option.
    const ABSTAIN: u8 = 255;
    const MAX_MENU_LEN: u8 = 16;
    const MAX_ROUNDS_LIMIT: u64 = 4;
    const DIGEST_LENGTH: u64 = 32;
    const MIN_SALT_BYTES: u64 = 16;
    const MAX_SALT_BYTES: u64 = 64;
    const MAX_URI_BYTES: u64 = 2048;
    const BPS_DENOMINATOR: u64 = 10000;

    const MS_HOUR: u64 = 3_600_000;
    const MS_DAY: u64 = 86_400_000;

    // ============================================================
    // Errors
    // ============================================================

    const EWrongVersion: u64 = 1;
    const EJuryPaused: u64 = 2;
    const EWrongCaseState: u64 = 3;
    const ENotYet: u64 = 4;
    const EInvalidConfig: u64 = 5;
    const EInvalidCase: u64 = 6;
    const EAlreadyOnRoster: u64 = 7;
    const ENotOnRoster: u64 = 8;
    const EIneligible: u64 = 9;
    const EWrongAgent: u64 = 10;
    const ENotJuror: u64 = 11;
    const ENotDrafted: u64 = 12;
    const EWrongSlotStatus: u64 = 13;
    const EWindowClosed: u64 = 14;
    const EInvalidDigest: u64 = 15;
    const EInvalidSalt: u64 = 16;
    const ECommitmentMismatch: u64 = 17;
    const EInvalidOutcome: u64 = 18;
    const EInvalidEvidenceRef: u64 = 19;
    const ERoundNotOver: u64 = 20;
    const ERosterTooSmall: u64 = 21;
    const EHasActiveCases: u64 = 22;
    const ETooMuchEvidence: u64 = 23;
    const EEvidenceClosed: u64 = 24;
    const EFeesNotSettled: u64 = 25;
    const EAlreadyClaimed: u64 = 26;
    const ENotAppealable: u64 = 27;
    const EAlreadyExecuted: u64 = 28;
    const ENotFinished: u64 = 29;
    const ETooManyExclusions: u64 = 30;
    const EInvalidUri: u64 = 31;
    const ENotOwner: u64 = 32;
    const ENotIneligible: u64 = 33;

    // ============================================================
    // Objects
    // ============================================================

    public struct JuryAdminCap has key, store {
        id: UID,
    }

    public struct EligibilityParams has store, copy, drop {
        min_score_milli: u64,
        min_confidence_bps: u64,
        min_settled_count: u64,
        min_age_ms: u64,
        max_snapshot_age_ms: u64,
        max_active_cases: u64,
        min_eligible_roster: u64,
    }

    public struct TimingParams has store, copy, drop {
        accept_window_ms: u64,
        commit_window_ms: u64,
        reveal_window_ms: u64,
        reveal_grace_ms: u64,
        appeal_window_ms: u64,
        neutral_window_ms: u64,
        max_case_duration_ms: u64,
    }

    public struct DutyParams has store, copy, drop {
        initial_score: u64,
        max_score: u64,
        min_score: u64,
        gain: u64,
        loss_accept: u64,
        loss_noreveal: u64,
        max_strikes: u64,
        suspension_ms: u64,
    }

    public struct SelectionParams has store, copy, drop {
        max_draws: u64,
        max_replacements: u64,
        max_evidence_per_party: u64,
        max_exclusions: u64,
    }

    /// Global jury parameters. Per-registry governance policy lives on the registry, not here.
    public struct JuryConfig has key {
        id: UID,
        version: u64,
        active: bool,
        eligibility: EligibilityParams,
        timing: TimingParams,
        duty: DutyParams,
        selection: SelectionParams,
        /// Scores a juror is judged by. Keep in step with the task market's `ReputationParams`.
        reputation: ReputationParams,
        /// Genesis agents skip the reputation bar until this time, then must qualify organically.
        genesis_sunset_ms: u64,
        genesis_agents: VecSet<ID>,
    }

    /// One willing agent. No balances: nothing is held, so nothing can be slashed.
    public struct RosterEntry has store, drop {
        agent_id: ID,
        owner: address,
        memory_account_id: ID,
        organization_id: ID,
        ancestor_ids: vector<ID>,
        payout_address: address,
        model_family: u8,
        status: u8,
        suspended_until_ms: u64,
        exempt: bool,
        duty_score: u64,
        completed: u64,
        no_shows: u64,
        strikes: u64,
        last_duty_ms: u64,
        active_cases: u64,
        joined_at_ms: u64,
        // Eligibility snapshot: the draw cannot load arbitrary agent objects, so it filters on this
        // and the drafted juror proves the live state when accepting.
        snapshot_at_ms: u64,
        snapshot_ok: bool,
        snapshot_score_milli: u64,
        snapshot_confidence_bps: u64,
        snapshot_settled_count: u64,
        snapshot_created_at_ms: u64,
        snapshot_expires_at: Option<u64>,
    }

    /// The roster with a dense index for O(1) uniform sampling.
    public struct JurorRoster has key {
        id: UID,
        version: u64,
        slots: Table<u64, ID>,
        index: Table<ID, u64>,
        entries: Table<ID, RosterEntry>,
        count: u64,
    }

    public struct EvidenceItem has store, copy, drop {
        submitter: address,
        round: u64,
        kind: u8,
        digest: vector<u8>,
        uri: vector<u8>,
        submitted_at_ms: u64,
    }

    public struct JurorSlot has store, drop {
        agent_id: ID,
        owner: address,
        organization_id: ID,
        payout_address: address,
        round: u64,
        status: u8,
        commitment: vector<u8>,
        outcome: u8,
        rationale_digest: vector<u8>,
        evidence_refs: vector<u64>,
        bundle_digest: vector<u8>,
        drafted_at_ms: u64,
        accepted_at_ms: u64,
        revealed_at_ms: u64,
        claimed: bool,
    }

    public struct RoundState has store, drop {
        size: u64,
        /// Current seat holders; a replaced juror leaves this list but keeps its slot record.
        jurors: vector<ID>,
        accept_deadline_ms: u64,
        commit_deadline_ms: u64,
        reveal_deadline_ms: u64,
        replacements_used: u64,
        tally: vector<u64>,
        abstain: u64,
        revealed: u64,
        closed: bool,
    }

    /// Who can never sit on this case, and who already has.
    public struct Exclusions has store, drop {
        owners: vector<address>,
        orgs: vector<ID>,
        agents: vector<ID>,
        used_owners: vector<address>,
        used_orgs: vector<ID>,
    }

    public struct CaseFees has store {
        pool: Balance<MYSO>,
        /// Cap on one juror's share; 0 = none (governance pays an equal share of the whole pool).
        max_juror_fee: u64,
        /// Where the integrator sends what is left after jurors are paid.
        sweep_to: address,
        participants: u64,
        per_juror_fee: u64,
        reserved: u64,
        settled: bool,
    }

    public struct JuryCase has key {
        id: UID,
        version: u64,
        subject_kind: u8,
        subject_id: ID,
        stage: u8,
        state: u8,
        fail_reason: u8,
        round: u64,
        round_sizes: vector<u64>,
        menu_len: u8,
        approval_bps: u64,
        quorum: u64,
        appealable: bool,
        failure_path: u8,
        opened_by: address,
        created_at_ms: u64,
        evidence_deadline_ms: u64,
        appeal_deadline_ms: u64,
        prior_outcome: Option<u8>,
        provisional_outcome: Option<u8>,
        final_outcome: Option<u8>,
        executed: bool,
        appeals_used: u64,
        failed_appeal: bool,
        exclusions: Exclusions,
        evidence: vector<EvidenceItem>,
        rounds: vector<RoundState>,
        slots: Table<ID, JurorSlot>,
        fees: CaseFees,
    }

    // ============================================================
    // Events (flat; mirrored by the indexer's BCS structs)
    // ============================================================

    public struct JuryConfigUpdated has copy, drop {
        updated_by: address,
        section: u8,
        timestamp_ms: u64,
    }

    public struct RosterJoined has copy, drop {
        agent_id: ID,
        owner: address,
        memory_account_id: ID,
        organization_id: ID,
        payout_address: address,
        model_family: u8,
        exempt: bool,
        joined_at_ms: u64,
    }

    public struct RosterLeft has copy, drop {
        agent_id: ID,
        reason: u8,
        left_at_ms: u64,
    }

    public struct RosterSuspended has copy, drop {
        agent_id: ID,
        until_ms: u64,
    }

    public struct DutyScoreChanged has copy, drop {
        agent_id: ID,
        score: u64,
        strikes: u64,
        reason: u8,
        case_id: ID,
    }

    public struct CaseOpened has copy, drop {
        case_id: ID,
        subject_kind: u8,
        subject_id: ID,
        stage: u8,
        menu_len: u8,
        approval_bps: u64,
        quorum: u64,
        round_sizes: vector<u64>,
        appealable: bool,
        failure_path: u8,
        opened_by: address,
        fee_mist: u64,
        created_at_ms: u64,
        evidence_deadline_ms: u64,
    }

    public struct EvidenceSubmitted has copy, drop {
        case_id: ID,
        index: u64,
        submitter: address,
        round: u64,
        kind: u8,
        digest: vector<u8>,
        uri: vector<u8>,
        submitted_at_ms: u64,
    }

    public struct JuryDrawn has copy, drop {
        case_id: ID,
        round: u64,
        jurors: vector<ID>,
        accept_deadline_ms: u64,
        replacement: bool,
    }

    public struct AssignmentAccepted has copy, drop {
        case_id: ID,
        round: u64,
        agent_id: ID,
        accepted_at_ms: u64,
    }

    public struct JurorReplaced has copy, drop {
        case_id: ID,
        round: u64,
        agent_id: ID,
    }

    public struct VotingStarted has copy, drop {
        case_id: ID,
        round: u64,
        jurors: vector<ID>,
        commit_deadline_ms: u64,
        reveal_deadline_ms: u64,
    }

    public struct VoteCommitted has copy, drop {
        case_id: ID,
        round: u64,
        agent_id: ID,
        commitment: vector<u8>,
    }

    public struct VoteRevealed has copy, drop {
        case_id: ID,
        round: u64,
        agent_id: ID,
        outcome: u8,
        rationale_digest: vector<u8>,
        evidence_refs: vector<u64>,
        bundle_digest: vector<u8>,
        revealed_at_ms: u64,
    }

    public struct RoundFinalized has copy, drop {
        case_id: ID,
        round: u64,
        tally: vector<u64>,
        abstain: u64,
        revealed: u64,
        decisive: bool,
        leading: Option<u8>,
        /// When the appeal window closes (and anyone may finalize); 0 if no window opened.
        appeal_deadline_ms: u64,
    }

    public struct CaseAppealed has copy, drop {
        case_id: ID,
        by: address,
        round: u64,
        prior_outcome: u8,
    }

    public struct CaseFinalized has copy, drop {
        case_id: ID,
        outcome: u8,
        round: u64,
        appeals_used: u64,
        failed_appeal: bool,
        participants: u64,
        per_juror_fee: u64,
        finalized_at_ms: u64,
    }

    public struct CaseFailed has copy, drop {
        case_id: ID,
        reason: u8,
        round: u64,
        participants: u64,
        per_juror_fee: u64,
        failed_at_ms: u64,
    }

    public struct CaseCancelled has copy, drop {
        case_id: ID,
        cancelled_at_ms: u64,
    }

    public struct JurorPaid has copy, drop {
        case_id: ID,
        agent_id: ID,
        recipient: address,
        amount_mist: u64,
    }

    // ============================================================
    // Bootstrap and configuration
    // ============================================================

    public(package) fun bootstrap_init(clock: &Clock, ctx: &mut TxContext) {
        let now = clock::timestamp_ms(clock);
        let config = JuryConfig {
            id: object::new(ctx),
            version: upgrade::current_version(),
            active: true,
            eligibility: EligibilityParams {
                min_score_milli: 4000,
                min_confidence_bps: 2000,
                min_settled_count: 3,
                min_age_ms: 7 * MS_DAY,
                max_snapshot_age_ms: 7 * MS_DAY,
                max_active_cases: 3,
                min_eligible_roster: 20,
            },
            timing: TimingParams {
                accept_window_ms: 12 * MS_HOUR,
                commit_window_ms: 24 * MS_HOUR,
                reveal_window_ms: 12 * MS_HOUR,
                reveal_grace_ms: 6 * MS_HOUR,
                appeal_window_ms: 48 * MS_HOUR,
                neutral_window_ms: 72 * MS_HOUR,
                max_case_duration_ms: 30 * MS_DAY,
            },
            duty: DutyParams {
                initial_score: 100,
                max_score: 200,
                min_score: 50,
                gain: 5,
                loss_accept: 10,
                loss_noreveal: 25,
                max_strikes: 3,
                suspension_ms: 14 * MS_DAY,
            },
            selection: SelectionParams {
                max_draws: 64,
                max_replacements: 3,
                max_evidence_per_party: 8,
                max_exclusions: 64,
            },
            reputation: memory::new_reputation_params(
                3000, 50000, 1_000_000_000, 500, 20000, 2000, 500, 16, 10000, 5000, 5000,
            ),
            genesis_sunset_ms: now + 180 * MS_DAY,
            genesis_agents: vec_set::empty(),
        };
        validate_config(&config);
        transfer::share_object(config);
        transfer::share_object(JurorRoster {
            id: object::new(ctx),
            version: upgrade::current_version(),
            slots: table::new(ctx),
            index: table::new(ctx),
            entries: table::new(ctx),
            count: 0,
        });
    }

    public(package) fun create_jury_admin_cap(ctx: &mut TxContext): JuryAdminCap {
        JuryAdminCap { id: object::new(ctx) }
    }

    fun validate_config(config: &JuryConfig) {
        let e = &config.eligibility;
        assert!(e.min_confidence_bps <= BPS_DENOMINATOR, EInvalidConfig);
        assert!(e.max_active_cases > 0 && e.max_snapshot_age_ms > 0, EInvalidConfig);
        let t = &config.timing;
        assert!(
            t.accept_window_ms > 0 && t.commit_window_ms > 0 && t.reveal_window_ms > 0
                && t.neutral_window_ms > 0 && t.max_case_duration_ms > 0,
            EInvalidConfig,
        );
        let d = &config.duty;
        assert!(d.initial_score <= d.max_score && d.min_score <= d.initial_score, EInvalidConfig);
        assert!(d.max_strikes > 0 && d.gain > 0, EInvalidConfig);
        let s = &config.selection;
        assert!(s.max_draws > 0 && s.max_evidence_per_party > 0 && s.max_exclusions > 0, EInvalidConfig);
        memory::assert_reputation_params(&config.reputation);
    }

    fun assert_config_live(config: &JuryConfig) {
        assert!(config.version == upgrade::current_version(), EWrongVersion);
    }

    public entry fun set_active(_: &JuryAdminCap, config: &mut JuryConfig, active: bool, clock: &Clock, ctx: &TxContext) {
        assert_config_live(config);
        config.active = active;
        emit_config_updated(0, clock, ctx);
    }

    public entry fun set_eligibility(
        _: &JuryAdminCap,
        config: &mut JuryConfig,
        min_score_milli: u64,
        min_confidence_bps: u64,
        min_settled_count: u64,
        min_age_ms: u64,
        max_snapshot_age_ms: u64,
        max_active_cases: u64,
        min_eligible_roster: u64,
        clock: &Clock,
        ctx: &TxContext,
    ) {
        assert_config_live(config);
        config.eligibility = EligibilityParams {
            min_score_milli,
            min_confidence_bps,
            min_settled_count,
            min_age_ms,
            max_snapshot_age_ms,
            max_active_cases,
            min_eligible_roster,
        };
        validate_config(config);
        emit_config_updated(1, clock, ctx);
    }

    public entry fun set_timing(
        _: &JuryAdminCap,
        config: &mut JuryConfig,
        accept_window_ms: u64,
        commit_window_ms: u64,
        reveal_window_ms: u64,
        reveal_grace_ms: u64,
        appeal_window_ms: u64,
        neutral_window_ms: u64,
        max_case_duration_ms: u64,
        clock: &Clock,
        ctx: &TxContext,
    ) {
        assert_config_live(config);
        config.timing = TimingParams {
            accept_window_ms,
            commit_window_ms,
            reveal_window_ms,
            reveal_grace_ms,
            appeal_window_ms,
            neutral_window_ms,
            max_case_duration_ms,
        };
        validate_config(config);
        emit_config_updated(2, clock, ctx);
    }

    public entry fun set_duty(
        _: &JuryAdminCap,
        config: &mut JuryConfig,
        initial_score: u64,
        max_score: u64,
        min_score: u64,
        gain: u64,
        loss_accept: u64,
        loss_noreveal: u64,
        max_strikes: u64,
        suspension_ms: u64,
        clock: &Clock,
        ctx: &TxContext,
    ) {
        assert_config_live(config);
        config.duty = DutyParams {
            initial_score,
            max_score,
            min_score,
            gain,
            loss_accept,
            loss_noreveal,
            max_strikes,
            suspension_ms,
        };
        validate_config(config);
        emit_config_updated(3, clock, ctx);
    }

    public entry fun set_selection(
        _: &JuryAdminCap,
        config: &mut JuryConfig,
        max_draws: u64,
        max_replacements: u64,
        max_evidence_per_party: u64,
        max_exclusions: u64,
        clock: &Clock,
        ctx: &TxContext,
    ) {
        assert_config_live(config);
        config.selection = SelectionParams { max_draws, max_replacements, max_evidence_per_party, max_exclusions };
        validate_config(config);
        emit_config_updated(4, clock, ctx);
    }

    public entry fun set_reputation_params(
        _: &JuryAdminCap,
        config: &mut JuryConfig,
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
        ctx: &TxContext,
    ) {
        assert_config_live(config);
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
        validate_config(config);
        emit_config_updated(5, clock, ctx);
    }

    /// Agents in the allow-list skip the reputation bar until `sunset_ms`, so a roster can exist
    /// before anyone has organic reputation. They still need the capability, an active agent and
    /// every conflict rule.
    public entry fun set_genesis_sunset(_: &JuryAdminCap, config: &mut JuryConfig, sunset_ms: u64, clock: &Clock, ctx: &TxContext) {
        assert_config_live(config);
        config.genesis_sunset_ms = sunset_ms;
        emit_config_updated(6, clock, ctx);
    }

    public entry fun add_genesis_agent(_: &JuryAdminCap, config: &mut JuryConfig, agent_id: ID, clock: &Clock, ctx: &TxContext) {
        assert_config_live(config);
        if (!vec_set::contains(&config.genesis_agents, &agent_id)) {
            vec_set::insert(&mut config.genesis_agents, agent_id);
        };
        emit_config_updated(7, clock, ctx);
    }

    public entry fun remove_genesis_agent(_: &JuryAdminCap, config: &mut JuryConfig, agent_id: ID, clock: &Clock, ctx: &TxContext) {
        assert_config_live(config);
        if (vec_set::contains(&config.genesis_agents, &agent_id)) {
            vec_set::remove(&mut config.genesis_agents, &agent_id);
        };
        emit_config_updated(7, clock, ctx);
    }

    fun emit_config_updated(section: u8, clock: &Clock, ctx: &TxContext) {
        event::emit(JuryConfigUpdated {
            updated_by: tx_context::sender(ctx),
            section,
            timestamp_ms: clock::timestamp_ms(clock),
        });
    }

    public entry fun migrate_jury_config(config: &mut JuryConfig, _: &UpgradeAdminCap) {
        assert!(config.version < upgrade::current_version(), EWrongVersion);
        config.version = upgrade::current_version();
    }

    public entry fun migrate_jury_roster(roster: &mut JurorRoster, _: &UpgradeAdminCap) {
        assert!(roster.version < upgrade::current_version(), EWrongVersion);
        roster.version = upgrade::current_version();
    }

    public entry fun migrate_jury_case(case: &mut JuryCase, _: &UpgradeAdminCap) {
        assert!(case.version < upgrade::current_version(), EWrongVersion);
        case.version = upgrade::current_version();
    }

    // ============================================================
    // Roster
    // ============================================================

    /// Joins the roster for free. The owner signs; the agent must carry the jury capability and be
    /// eligible now (or be an allow-listed genesis agent before the sunset).
    public entry fun join_roster(
        config: &JuryConfig,
        roster: &mut JurorRoster,
        memory_config: &MemoryConfig,
        account: &MemoryAccount,
        agent: &SubAgent,
        payout_address: address,
        model_family: u8,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert_config_live(config);
        assert!(roster.version == upgrade::current_version(), EWrongVersion);
        assert!(config.active, EJuryPaused);
        memory::resolve_human_actor(account, ctx);
        assert!(memory::sub_agent_memory_account_id(agent) == object::id(account), EWrongAgent);
        let agent_id = object::id(agent);
        assert!(!table::contains(&roster.entries, agent_id), EAlreadyOnRoster);

        let now = clock::timestamp_ms(clock);
        let snap = take_snapshot(config, memory_config, account, agent);
        assert!(snap.ok, EIneligible);
        let exempt = is_genesis_exempt(config, agent_id, now);
        assert!(
            exempt || meets_reputation(config, snap.score_milli, snap.confidence_bps, snap.settled_count, snap.created_at_ms, now),
            EIneligible,
        );

        let entry = RosterEntry {
            agent_id,
            owner: memory::owner(account),
            memory_account_id: object::id(account),
            organization_id: memory::sub_agent_organization_id(agent),
            ancestor_ids: snap.ancestors,
            payout_address,
            model_family,
            status: ROSTER_ACTIVE,
            suspended_until_ms: 0,
            exempt,
            duty_score: config.duty.initial_score,
            completed: 0,
            no_shows: 0,
            strikes: 0,
            last_duty_ms: 0,
            active_cases: 0,
            joined_at_ms: now,
            snapshot_at_ms: now,
            snapshot_ok: snap.ok,
            snapshot_score_milli: snap.score_milli,
            snapshot_confidence_bps: snap.confidence_bps,
            snapshot_settled_count: snap.settled_count,
            snapshot_created_at_ms: snap.created_at_ms,
            snapshot_expires_at: snap.expires_at,
        };
        event::emit(RosterJoined {
            agent_id,
            owner: entry.owner,
            memory_account_id: entry.memory_account_id,
            organization_id: entry.organization_id,
            payout_address,
            model_family,
            exempt,
            joined_at_ms: now,
        });
        add_to_roster(roster, agent_id, entry);
    }

    /// Leaves the roster. Refused while the agent sits on an open case, so a seat is never abandoned.
    public entry fun leave_roster(
        roster: &mut JurorRoster,
        account: &MemoryAccount,
        agent_id: ID,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(roster.version == upgrade::current_version(), EWrongVersion);
        memory::resolve_human_actor(account, ctx);
        assert!(table::contains(&roster.entries, agent_id), ENotOnRoster);
        {
            let entry = table::borrow(&roster.entries, agent_id);
            assert!(entry.owner == memory::owner(account), ENotOwner);
            assert!(entry.active_cases == 0, EHasActiveCases);
        };
        remove_from_roster(roster, agent_id);
        event::emit(RosterLeft { agent_id, reason: 1, left_at_ms: clock::timestamp_ms(clock) });
    }

    /// The owner pauses or resumes being drafted. A suspension cannot be lifted this way.
    public entry fun set_availability(
        roster: &mut JurorRoster,
        account: &MemoryAccount,
        agent_id: ID,
        available: bool,
        ctx: &mut TxContext,
    ) {
        assert!(roster.version == upgrade::current_version(), EWrongVersion);
        memory::resolve_human_actor(account, ctx);
        assert!(table::contains(&roster.entries, agent_id), ENotOnRoster);
        let entry = table::borrow_mut(&mut roster.entries, agent_id);
        assert!(entry.owner == memory::owner(account), ENotOwner);
        if (entry.status == ROSTER_SUSPENDED) {
            return
        };
        entry.status = if (available) { ROSTER_ACTIVE } else { ROSTER_PAUSED };
    }

    /// Anyone passes the live objects to refresh the eligibility snapshot the draw filters on.
    public entry fun refresh_roster_entry(
        config: &JuryConfig,
        roster: &mut JurorRoster,
        memory_config: &MemoryConfig,
        account: &MemoryAccount,
        agent: &SubAgent,
        clock: &Clock,
    ) {
        assert_config_live(config);
        assert!(roster.version == upgrade::current_version(), EWrongVersion);
        let agent_id = object::id(agent);
        assert!(table::contains(&roster.entries, agent_id), ENotOnRoster);
        assert!(memory::sub_agent_memory_account_id(agent) == object::id(account), EWrongAgent);
        let now = clock::timestamp_ms(clock);
        let snap = take_snapshot(config, memory_config, account, agent);
        let exempt = is_genesis_exempt(config, agent_id, now);
        let entry = table::borrow_mut(&mut roster.entries, agent_id);
        entry.snapshot_at_ms = now;
        entry.snapshot_ok = snap.ok;
        entry.snapshot_score_milli = snap.score_milli;
        entry.snapshot_confidence_bps = snap.confidence_bps;
        entry.snapshot_settled_count = snap.settled_count;
        entry.snapshot_created_at_ms = snap.created_at_ms;
        entry.snapshot_expires_at = snap.expires_at;
        entry.ancestor_ids = snap.ancestors;
        entry.exempt = exempt;
    }

    /// Removes an entry whose snapshot says the agent can no longer serve (inactive, expired, lost
    /// the capability or class). Needs a fresh snapshot, so a stale entry is refreshed first.
    public entry fun evict_ineligible(
        config: &JuryConfig,
        roster: &mut JurorRoster,
        agent_id: ID,
        clock: &Clock,
    ) {
        assert_config_live(config);
        assert!(roster.version == upgrade::current_version(), EWrongVersion);
        assert!(table::contains(&roster.entries, agent_id), ENotOnRoster);
        let now = clock::timestamp_ms(clock);
        {
            let entry = table::borrow(&roster.entries, agent_id);
            assert!(entry.active_cases == 0, EHasActiveCases);
            let expired = option::is_some(&entry.snapshot_expires_at)
                && *option::borrow(&entry.snapshot_expires_at) <= now;
            assert!(!entry.snapshot_ok || expired, ENotIneligible);
        };
        remove_from_roster(roster, agent_id);
        event::emit(RosterLeft { agent_id, reason: 2, left_at_ms: now });
    }

    fun add_to_roster(roster: &mut JurorRoster, agent_id: ID, entry: RosterEntry) {
        let position = roster.count;
        table::add(&mut roster.entries, agent_id, entry);
        table::add(&mut roster.slots, position, agent_id);
        table::add(&mut roster.index, agent_id, position);
        roster.count = position + 1;
    }

    /// Swap-remove keeps the slot index dense.
    fun remove_from_roster(roster: &mut JurorRoster, agent_id: ID) {
        let position = table::remove(&mut roster.index, agent_id);
        let last = roster.count - 1;
        if (position != last) {
            let last_id = *table::borrow(&roster.slots, last);
            *table::borrow_mut(&mut roster.slots, position) = last_id;
            *table::borrow_mut(&mut roster.index, last_id) = position;
        };
        table::remove(&mut roster.slots, last);
        roster.count = last;
        table::remove(&mut roster.entries, agent_id);
    }

    public struct Snapshot has copy, drop {
        ok: bool,
        score_milli: u64,
        confidence_bps: u64,
        settled_count: u64,
        created_at_ms: u64,
        expires_at: Option<u64>,
        ancestors: vector<ID>,
    }

    fun take_snapshot(
        config: &JuryConfig,
        memory_config: &MemoryConfig,
        account: &MemoryAccount,
        agent: &SubAgent,
    ): Snapshot {
        let caps = memory::sub_agent_capabilities(agent);
        let ok = memory::sub_agent_active(agent)
            && memory::is_active(account)
            && memory::sub_agent_identity_class(agent) == memory::class_delegated_ai()
            && (caps & memory::cap_jury()) == memory::cap_jury();
        let (score, confidence) = memory::agent_overall(agent, &config.reputation);
        Snapshot {
            ok,
            score_milli: score,
            confidence_bps: confidence,
            settled_count: memory::account_settled_count(account),
            created_at_ms: memory::sub_agent_created_at(agent),
            expires_at: memory::sub_agent_expires_at(agent),
            ancestors: memory::agent_ancestor_ids(memory_config, account, object::id(agent)),
        }
    }

    fun is_genesis_exempt(config: &JuryConfig, agent_id: ID, now: u64): bool {
        vec_set::contains(&config.genesis_agents, &agent_id) && now <= config.genesis_sunset_ms
    }

    fun meets_reputation(
        config: &JuryConfig,
        score_milli: u64,
        confidence_bps: u64,
        settled_count: u64,
        created_at_ms: u64,
        now: u64,
    ): bool {
        let e = &config.eligibility;
        score_milli >= e.min_score_milli
            && confidence_bps >= e.min_confidence_bps
            && settled_count >= e.min_settled_count
            && now >= created_at_ms
            && now - created_at_ms >= e.min_age_ms
    }

    /// The coarse filter the draw applies from the snapshot; `accept_assignment` is the live check.
    fun draw_eligible(config: &JuryConfig, entry: &RosterEntry, now: u64): bool {
        if (entry.status == ROSTER_PAUSED) {
            return false
        };
        if (entry.status == ROSTER_SUSPENDED && now < entry.suspended_until_ms) {
            return false
        };
        if (!entry.snapshot_ok) {
            return false
        };
        if (now > entry.snapshot_at_ms && now - entry.snapshot_at_ms > config.eligibility.max_snapshot_age_ms) {
            return false
        };
        if (option::is_some(&entry.snapshot_expires_at) && *option::borrow(&entry.snapshot_expires_at) <= now) {
            return false
        };
        if (entry.active_cases >= config.eligibility.max_active_cases) {
            return false
        };
        if (entry.duty_score < config.duty.min_score) {
            return false
        };
        if (entry.exempt && now <= config.genesis_sunset_ms) {
            return true
        };
        meets_reputation(
            config,
            entry.snapshot_score_milli,
            entry.snapshot_confidence_bps,
            entry.snapshot_settled_count,
            entry.snapshot_created_at_ms,
            now,
        )
    }

    /// Entries a draw could pick right now. An integrator refuses to open a case below the minimum,
    /// so a thin roster fails closed instead of being gamed.
    public fun eligible_count(config: &JuryConfig, roster: &JurorRoster, now: u64): u64 {
        let mut count = 0;
        let mut i = 0;
        while (i < roster.count) {
            let agent_id = *table::borrow(&roster.slots, i);
            if (draw_eligible(config, table::borrow(&roster.entries, agent_id), now)) {
                count = count + 1;
            };
            i = i + 1;
        };
        count
    }

    public(package) fun assert_roster_ready(config: &JuryConfig, roster: &JurorRoster, clock: &Clock) {
        assert!(
            eligible_count(config, roster, clock::timestamp_ms(clock)) >= config.eligibility.min_eligible_roster,
            ERosterTooSmall,
        );
    }

    // ============================================================
    // Cases (opened and settled by an integrator in this package)
    // ============================================================

    /// Builds a case; the integrator adds evidence and fees, then calls [`share_case`]. `menu_len`
    /// outcomes are indexes 0..menu_len-1; the integrator maps each to one fixed action.
    /// `approval_bps` is the share of non-abstain reveals the leader needs (and it must lead
    /// strictly); `quorum` is the minimum reveals (abstentions included) for a round to count.
    public(package) fun open_case(
        config: &JuryConfig,
        subject_kind: u8,
        subject_id: ID,
        stage: u8,
        menu_len: u8,
        approval_bps: u64,
        quorum: u64,
        round_sizes: vector<u64>,
        appealable: bool,
        failure_path: u8,
        evidence_window_ms: u64,
        exclusion_owners: vector<address>,
        exclusion_orgs: vector<ID>,
        exclusion_agents: vector<ID>,
        fee: Balance<MYSO>,
        max_juror_fee: u64,
        sweep_to: address,
        clock: &Clock,
        ctx: &mut TxContext,
    ): JuryCase {
        assert_config_live(config);
        assert!(config.active, EJuryPaused);
        assert!(subject_kind == SUBJECT_TASK || subject_kind == SUBJECT_GOVERNANCE, EInvalidCase);
        assert!(menu_len >= 2 && menu_len <= MAX_MENU_LEN, EInvalidCase);
        assert!(approval_bps >= 5000 && approval_bps <= BPS_DENOMINATOR, EInvalidCase);
        assert!(quorum >= 1, EInvalidCase);
        let rounds = vector::length(&round_sizes);
        assert!(rounds >= 1 && rounds <= MAX_ROUNDS_LIMIT, EInvalidCase);
        let mut i = 0;
        while (i < rounds) {
            assert!(*vector::borrow(&round_sizes, i) >= quorum, EInvalidCase);
            i = i + 1;
        };
        assert!(
            vector::length(&exclusion_owners) + vector::length(&exclusion_orgs) + vector::length(&exclusion_agents)
                <= config.selection.max_exclusions,
            ETooManyExclusions,
        );

        let now = clock::timestamp_ms(clock);
        let state = if (evidence_window_ms == 0) { CASE_AWAITING_DRAW } else { CASE_EVIDENCE };
        let fee_mist = balance::value(&fee);
        let case = JuryCase {
            id: object::new(ctx),
            version: upgrade::current_version(),
            subject_kind,
            subject_id,
            stage,
            state,
            fail_reason: FAIL_NONE,
            round: 0,
            round_sizes,
            menu_len,
            approval_bps,
            quorum,
            appealable,
            failure_path,
            opened_by: tx_context::sender(ctx),
            created_at_ms: now,
            evidence_deadline_ms: memory::checked_add_u64(now, evidence_window_ms),
            appeal_deadline_ms: 0,
            prior_outcome: option::none(),
            provisional_outcome: option::none(),
            final_outcome: option::none(),
            executed: false,
            appeals_used: 0,
            failed_appeal: false,
            exclusions: Exclusions {
                owners: exclusion_owners,
                orgs: exclusion_orgs,
                agents: exclusion_agents,
                used_owners: vector::empty(),
                used_orgs: vector::empty(),
            },
            evidence: vector::empty(),
            rounds: vector::empty(),
            slots: table::new(ctx),
            fees: CaseFees {
                pool: fee,
                max_juror_fee,
                sweep_to,
                participants: 0,
                per_juror_fee: 0,
                reserved: 0,
                settled: false,
            },
        };
        event::emit(CaseOpened {
            case_id: object::id(&case),
            subject_kind,
            subject_id,
            stage,
            menu_len,
            approval_bps,
            quorum,
            round_sizes: case.round_sizes,
            appealable,
            failure_path,
            opened_by: case.opened_by,
            fee_mist,
            created_at_ms: now,
            evidence_deadline_ms: case.evidence_deadline_ms,
        });
        case
    }

    public(package) fun share_case(case: JuryCase): ID {
        let case_id = object::id(&case);
        transfer::share_object(case);
        case_id
    }

    /// Adds evidence before the evidence deadline (first round) or while an appeal round waits for its
    /// draw (later rounds, flagged with the round so jurors can tell what is new). The integrator
    /// checks who `submitter` is.
    public(package) fun add_evidence(
        config: &JuryConfig,
        case: &mut JuryCase,
        submitter: address,
        kind: u8,
        digest: vector<u8>,
        uri: vector<u8>,
        clock: &Clock,
    ) {
        let now = clock::timestamp_ms(clock);
        let undrawn = vector::length(&case.rounds) == case.round;
        let open = (case.state == CASE_EVIDENCE && now <= case.evidence_deadline_ms)
            || (case.state == CASE_AWAITING_DRAW && undrawn);
        assert!(open, EEvidenceClosed);
        assert!(vector::length(&digest) == DIGEST_LENGTH, EInvalidDigest);
        assert!(vector::length(&uri) <= MAX_URI_BYTES, EInvalidUri);
        let mut count = 0;
        let mut i = 0;
        let len = vector::length(&case.evidence);
        while (i < len) {
            if (vector::borrow(&case.evidence, i).submitter == submitter) {
                count = count + 1;
            };
            i = i + 1;
        };
        assert!(count < config.selection.max_evidence_per_party, ETooMuchEvidence);
        let item = EvidenceItem { submitter, round: case.round, kind, digest, uri, submitted_at_ms: now };
        event::emit(EvidenceSubmitted {
            case_id: object::id(case),
            index: len,
            submitter,
            round: item.round,
            kind,
            digest: item.digest,
            uri: item.uri,
            submitted_at_ms: now,
        });
        vector::push_back(&mut case.evidence, item);
    }

    /// Tops up the case fee (an appeal fee, a second party's fee). Only before the fees settle.
    public(package) fun add_fee(case: &mut JuryCase, fee: Balance<MYSO>) {
        assert!(!case.fees.settled, EFeesNotSettled);
        balance::join(&mut case.fees.pool, fee);
    }

    /// An appeal by the integrator on behalf of the losing side: a new, larger round with fresh jurors.
    public(package) fun appeal(case: &mut JuryCase, by: address, clock: &Clock) {
        assert!(case.version == upgrade::current_version(), EWrongVersion);
        assert!(case.state == CASE_TALLIED, EWrongCaseState);
        let now = clock::timestamp_ms(clock);
        assert!(now <= case.appeal_deadline_ms, EWindowClosed);
        assert!(case.round + 1 < vector::length(&case.round_sizes), ENotAppealable);
        let prior = *option::borrow(&case.provisional_outcome);
        case.prior_outcome = option::some(prior);
        case.provisional_outcome = option::none();
        case.round = case.round + 1;
        case.appeals_used = case.appeals_used + 1;
        case.state = CASE_AWAITING_DRAW;
        event::emit(CaseAppealed { case_id: object::id(case), by, round: case.round, prior_outcome: prior });
    }

    /// Cancels a case no juror has been drawn for (a settlement, or the initiator withdrawing).
    public(package) fun cancel_case(case: &mut JuryCase, clock: &Clock) {
        assert!(case.version == upgrade::current_version(), EWrongVersion);
        let undrawn = case.round == 0 && vector::length(&case.rounds) == 0;
        assert!(undrawn && (case.state == CASE_EVIDENCE || case.state == CASE_AWAITING_DRAW), EWrongCaseState);
        let now = clock::timestamp_ms(clock);
        case.state = CASE_CANCELLED;
        settle_fees(case);
        event::emit(CaseCancelled { case_id: object::id(case), cancelled_at_ms: now });
    }

    /// What is left after jurors are paid. Jurors' unclaimed shares stay reserved in the pool.
    public(package) fun sweep_remainder(case: &mut JuryCase): Balance<MYSO> {
        assert!(case.fees.settled, EFeesNotSettled);
        let total = balance::value(&case.fees.pool);
        let excess = total - case.fees.reserved;
        balance::split(&mut case.fees.pool, excess)
    }

    public(package) fun mark_executed(case: &mut JuryCase) {
        assert!(!case.executed, EAlreadyExecuted);
        assert!(is_finished(case.state), ENotFinished);
        case.executed = true;
    }

    fun is_finished(state: u8): bool {
        state == CASE_FINAL || state == CASE_FAILED || state == CASE_CANCELLED
    }

    // ============================================================
    // Drawing
    // ============================================================

    /// Draws the jurors for the current round. `entry`, never `public`, so a transaction cannot
    /// compose the draw and revert when the result is unfavourable. Anyone may call it once the
    /// evidence window has closed (or an appeal/next round is waiting).
    entry fun select_jury(
        config: &JuryConfig,
        roster: &mut JurorRoster,
        case: &mut JuryCase,
        rand: &Random,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert_live(config, roster, case);
        let now = clock::timestamp_ms(clock);
        let ready = case.state == CASE_AWAITING_DRAW
            || (case.state == CASE_EVIDENCE && now > case.evidence_deadline_ms);
        assert!(ready, EWrongCaseState);
        let round = case.round;
        assert!(vector::length(&case.rounds) == round, EWrongCaseState);
        let size = *vector::borrow(&case.round_sizes, round);

        let mut gen = random::new_generator(rand, ctx);
        let drawn = draw_jurors(config, roster, case, &mut gen, size, round, now);
        let accept_deadline = memory::checked_add_u64(now, config.timing.accept_window_ms);
        let mut tally = vector::empty<u64>();
        let mut m = 0u8;
        while (m < case.menu_len) {
            vector::push_back(&mut tally, 0);
            m = m + 1;
        };
        vector::push_back(&mut case.rounds, RoundState {
            size,
            jurors: copy drawn,
            accept_deadline_ms: accept_deadline,
            commit_deadline_ms: 0,
            reveal_deadline_ms: 0,
            replacements_used: 0,
            tally,
            abstain: 0,
            revealed: 0,
            closed: false,
        });
        case.state = CASE_SELECTING;
        event::emit(JuryDrawn {
            case_id: object::id(case),
            round,
            jurors: drawn,
            accept_deadline_ms: accept_deadline,
            replacement: false,
        });
    }

    /// Uniform rejection sampling over the roster with a fixed attempt budget. A candidate is
    /// skipped when the snapshot says it cannot serve or it conflicts with the case; one juror per
    /// owner and per organisation is kept across every round of the case.
    fun draw_jurors(
        config: &JuryConfig,
        roster: &mut JurorRoster,
        case: &mut JuryCase,
        gen: &mut RandomGenerator,
        needed: u64,
        round: u64,
        now: u64,
    ): vector<ID> {
        let mut chosen = vector::empty<ID>();
        if (roster.count == 0 || needed == 0) {
            return chosen
        };
        let mut attempts = 0;
        while (attempts < config.selection.max_draws && vector::length(&chosen) < needed) {
            attempts = attempts + 1;
            let position = random::generate_u64_in_range(gen, 0, roster.count - 1);
            let agent_id = *table::borrow(&roster.slots, position);
            let (picked, owner, org, payout) = {
                let entry = table::borrow(&roster.entries, agent_id);
                let ok = draw_eligible(config, entry, now) && !conflicted(case, entry);
                (ok, entry.owner, entry.organization_id, entry.payout_address)
            };
            if (!picked) {
                continue
            };
            {
                let seated = table::borrow_mut(&mut roster.entries, agent_id);
                seated.active_cases = seated.active_cases + 1;
            };
            vector::push_back(&mut case.exclusions.used_owners, owner);
            vector::push_back(&mut case.exclusions.used_orgs, org);
            table::add(&mut case.slots, agent_id, JurorSlot {
                agent_id,
                owner,
                organization_id: org,
                payout_address: payout,
                round,
                status: SLOT_DRAFTED,
                commitment: vector::empty(),
                outcome: ABSTAIN,
                rationale_digest: vector::empty(),
                evidence_refs: vector::empty(),
                bundle_digest: vector::empty(),
                drafted_at_ms: now,
                accepted_at_ms: 0,
                revealed_at_ms: 0,
                claimed: false,
            });
            vector::push_back(&mut chosen, agent_id);
        };
        chosen
    }

    fun conflicted(case: &JuryCase, entry: &RosterEntry): bool {
        let ex = &case.exclusions;
        if (vector::contains(&ex.owners, &entry.owner)
            || vector::contains(&ex.orgs, &entry.organization_id)
            || vector::contains(&ex.agents, &entry.agent_id)
            || vector::contains(&ex.used_owners, &entry.owner)
            || vector::contains(&ex.used_orgs, &entry.organization_id)) {
            return true
        };
        // A juror that descends from an excluded agent is as conflicted as the agent itself.
        let mut i = 0;
        let len = vector::length(&entry.ancestor_ids);
        while (i < len) {
            if (vector::contains(&ex.agents, vector::borrow(&entry.ancestor_ids, i))) {
                return true
            };
            i = i + 1;
        };
        false
    }

    // ============================================================
    // Accepting, replacing, voting
    // ============================================================

    /// A drafted juror takes the seat. The juror proves the live state here (active, the capability,
    /// reputation, class), which is the authoritative check behind the draw's snapshot.
    public entry fun accept_assignment(
        config: &JuryConfig,
        case: &mut JuryCase,
        memory_config: &MemoryConfig,
        account: &MemoryAccount,
        agent: &SubAgent,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert_config_live(config);
        assert!(case.version == upgrade::current_version(), EWrongVersion);
        assert!(case.state == CASE_SELECTING, EWrongCaseState);
        let agent_id = object::id(agent);
        assert!(table::contains(&case.slots, agent_id), ENotDrafted);
        assert_juror_actor(memory_config, account, agent, clock, ctx);

        let now = clock::timestamp_ms(clock);
        let round = case.round;
        {
            let slot = table::borrow(&case.slots, agent_id);
            assert!(slot.status == SLOT_DRAFTED && slot.round == round, EWrongSlotStatus);
            let r = vector::borrow(&case.rounds, round);
            assert!(now <= r.accept_deadline_ms, EWindowClosed);
        };
        memory::assert_sub_agent_active(agent, clock);
        let snap = take_snapshot(config, memory_config, account, agent);
        assert!(snap.ok, EIneligible);
        assert!(
            is_genesis_exempt(config, agent_id, now)
                || meets_reputation(config, snap.score_milli, snap.confidence_bps, snap.settled_count, snap.created_at_ms, now),
            EIneligible,
        );

        {
            let slot = table::borrow_mut(&mut case.slots, agent_id);
            slot.status = SLOT_ACCEPTED;
            slot.accepted_at_ms = now;
        };
        event::emit(AssignmentAccepted { case_id: object::id(case), round, agent_id, accepted_at_ms: now });
        if (all_seats_accepted(case, round)) {
            start_voting(config, case, now);
        };
    }

    fun all_seats_accepted(case: &JuryCase, round: u64): bool {
        let r = vector::borrow(&case.rounds, round);
        if (vector::length(&r.jurors) < r.size) {
            return false
        };
        let mut i = 0;
        let len = vector::length(&r.jurors);
        while (i < len) {
            let id = *vector::borrow(&r.jurors, i);
            if (table::borrow(&case.slots, id).status == SLOT_DRAFTED) {
                return false
            };
            i = i + 1;
        };
        true
    }

    fun start_voting(config: &JuryConfig, case: &mut JuryCase, now: u64) {
        let round = case.round;
        let commit_deadline = memory::checked_add_u64(now, config.timing.commit_window_ms);
        let reveal_deadline = memory::checked_add_u64(
            commit_deadline,
            memory::checked_add_u64(config.timing.reveal_window_ms, config.timing.reveal_grace_ms),
        );
        let jurors = {
            let r = vector::borrow_mut(&mut case.rounds, round);
            r.commit_deadline_ms = commit_deadline;
            r.reveal_deadline_ms = reveal_deadline;
            r.jurors
        };
        case.state = CASE_VOTING;
        event::emit(VotingStarted {
            case_id: object::id(case),
            round,
            jurors,
            commit_deadline_ms: commit_deadline,
            reveal_deadline_ms: reveal_deadline,
        });
    }

    /// After the accept window: seats that never accepted are replaced by fresh draws (a few times),
    /// each no-show costs the juror duty score and a strike, and if the seats still cannot be filled
    /// the round goes ahead with the jurors it has when they meet the quorum, otherwise the case fails.
    entry fun replace_no_shows(
        config: &JuryConfig,
        roster: &mut JurorRoster,
        case: &mut JuryCase,
        rand: &Random,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert_live(config, roster, case);
        assert!(case.state == CASE_SELECTING, EWrongCaseState);
        let now = clock::timestamp_ms(clock);
        let round = case.round;
        let (size, seats) = {
            let r = vector::borrow(&case.rounds, round);
            assert!(now > r.accept_deadline_ms, ENotYet);
            (r.size, r.jurors)
        };

        let case_id = object::id(case);
        let mut kept = vector::empty<ID>();
        let mut i = 0;
        let len = vector::length(&seats);
        while (i < len) {
            let agent_id = *vector::borrow(&seats, i);
            if (table::borrow(&case.slots, agent_id).status == SLOT_DRAFTED) {
                table::borrow_mut(&mut case.slots, agent_id).status = SLOT_REPLACED;
                penalize_duty(config, roster, agent_id, config.duty.loss_accept, DUTY_NO_ACCEPT, case_id, now);
                release_seat(roster, agent_id);
                event::emit(JurorReplaced { case_id, round, agent_id });
            } else {
                vector::push_back(&mut kept, agent_id);
            };
            i = i + 1;
        };
        let accepted = vector::length(&kept);
        vector::borrow_mut(&mut case.rounds, round).jurors = kept;
        let missing = size - accepted;

        if (missing == 0) {
            start_voting(config, case, now);
            return
        };
        let used = vector::borrow(&case.rounds, round).replacements_used;
        if (used < config.selection.max_replacements) {
            let mut gen = random::new_generator(rand, ctx);
            let drawn = draw_jurors(config, roster, case, &mut gen, missing, round, now);
            let accept_deadline = memory::checked_add_u64(now, config.timing.accept_window_ms);
            {
                let r = vector::borrow_mut(&mut case.rounds, round);
                vector::append(&mut r.jurors, copy drawn);
                r.replacements_used = r.replacements_used + 1;
                r.accept_deadline_ms = accept_deadline;
            };
            event::emit(JuryDrawn { case_id, round, jurors: drawn, accept_deadline_ms: accept_deadline, replacement: true });
        } else if (accepted >= case.quorum) {
            start_voting(config, case, now);
        } else {
            close_round(config, roster, case, now, true);
            end_failed(case, FAIL_CANNOT_FORM, now);
        };
    }

    /// Hidden vote. `commitment` binds the case, round, juror, outcome, rationale, evidence refs,
    /// bundle and a secret salt; nobody can see or copy a vote until it is revealed.
    public entry fun commit_vote(
        case: &mut JuryCase,
        memory_config: &MemoryConfig,
        account: &MemoryAccount,
        agent: &SubAgent,
        commitment: vector<u8>,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(case.version == upgrade::current_version(), EWrongVersion);
        assert!(case.state == CASE_VOTING, EWrongCaseState);
        let agent_id = object::id(agent);
        assert!(table::contains(&case.slots, agent_id), ENotDrafted);
        assert_juror_actor(memory_config, account, agent, clock, ctx);
        assert!(vector::length(&commitment) == DIGEST_LENGTH, EInvalidDigest);
        let now = clock::timestamp_ms(clock);
        let round = case.round;
        assert!(now <= vector::borrow(&case.rounds, round).commit_deadline_ms, EWindowClosed);
        let slot = table::borrow_mut(&mut case.slots, agent_id);
        assert!(slot.round == round && slot.status == SLOT_ACCEPTED, EWrongSlotStatus);
        slot.status = SLOT_COMMITTED;
        slot.commitment = copy commitment;
        event::emit(VoteCommitted { case_id: object::id(case), round, agent_id, commitment });
    }

    /// Opens a vote. Allowed once the commit window is over, or earlier when every seat has
    /// committed. Must hash to the commitment, back a menu outcome (or abstain), and cite evidence
    /// that exists on the case.
    public entry fun reveal_vote(
        case: &mut JuryCase,
        memory_config: &MemoryConfig,
        account: &MemoryAccount,
        agent: &SubAgent,
        outcome: u8,
        rationale_digest: vector<u8>,
        evidence_refs: vector<u64>,
        bundle_digest: vector<u8>,
        salt: vector<u8>,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(case.version == upgrade::current_version(), EWrongVersion);
        assert!(case.state == CASE_VOTING, EWrongCaseState);
        let agent_id = object::id(agent);
        assert!(table::contains(&case.slots, agent_id), ENotDrafted);
        assert_juror_actor(memory_config, account, agent, clock, ctx);

        let now = clock::timestamp_ms(clock);
        let round = case.round;
        {
            let r = vector::borrow(&case.rounds, round);
            assert!(now <= r.reveal_deadline_ms, EWindowClosed);
            assert!(now > r.commit_deadline_ms || none_uncommitted(case, round), ENotYet);
        };
        assert!(vector::length(&rationale_digest) == DIGEST_LENGTH, EInvalidDigest);
        assert!(vector::length(&bundle_digest) == DIGEST_LENGTH, EInvalidDigest);
        let salt_len = vector::length(&salt);
        assert!(salt_len >= MIN_SALT_BYTES && salt_len <= MAX_SALT_BYTES, EInvalidSalt);
        assert!(outcome == ABSTAIN || outcome < case.menu_len, EInvalidOutcome);
        if (outcome != ABSTAIN) {
            let evidence_count = vector::length(&case.evidence);
            assert!(!vector::is_empty(&evidence_refs), EInvalidEvidenceRef);
            let mut i = 0;
            let n = vector::length(&evidence_refs);
            while (i < n) {
                assert!(*vector::borrow(&evidence_refs, i) < evidence_count, EInvalidEvidenceRef);
                i = i + 1;
            };
        };

        let expected = compute_commitment(
            object::id(case), round, agent_id, outcome, &rationale_digest, &evidence_refs, &bundle_digest, &salt,
        );
        {
            let slot = table::borrow_mut(&mut case.slots, agent_id);
            assert!(slot.round == round && slot.status == SLOT_COMMITTED, EWrongSlotStatus);
            assert!(slot.commitment == expected, ECommitmentMismatch);
            slot.status = SLOT_REVEALED;
            slot.outcome = outcome;
            slot.rationale_digest = copy rationale_digest;
            slot.evidence_refs = copy evidence_refs;
            slot.bundle_digest = copy bundle_digest;
            slot.revealed_at_ms = now;
        };
        {
            let r = vector::borrow_mut(&mut case.rounds, round);
            r.revealed = r.revealed + 1;
            if (outcome == ABSTAIN) {
                r.abstain = r.abstain + 1;
            } else {
                let count = vector::borrow_mut(&mut r.tally, outcome as u64);
                *count = *count + 1;
            };
        };
        case.fees.participants = case.fees.participants + 1;
        event::emit(VoteRevealed {
            case_id: object::id(case),
            round,
            agent_id,
            outcome,
            rationale_digest,
            evidence_refs,
            bundle_digest,
            revealed_at_ms: now,
        });
    }

    fun none_uncommitted(case: &JuryCase, round: u64): bool {
        let r = vector::borrow(&case.rounds, round);
        let mut i = 0;
        let len = vector::length(&r.jurors);
        while (i < len) {
            let id = *vector::borrow(&r.jurors, i);
            if (table::borrow(&case.slots, id).status == SLOT_ACCEPTED) {
                return false
            };
            i = i + 1;
        };
        true
    }

    /// The encoding the runner must reproduce: BCS of case id, round, agent id and outcome; the raw
    /// 32-byte rationale digest; BCS of the evidence refs; the raw 32-byte bundle digest; the salt.
    public fun compute_commitment(
        case_id: ID,
        round: u64,
        agent_id: ID,
        outcome: u8,
        rationale_digest: &vector<u8>,
        evidence_refs: &vector<u64>,
        bundle_digest: &vector<u8>,
        salt: &vector<u8>,
    ): vector<u8> {
        let mut bytes = vector::empty<u8>();
        vector::append(&mut bytes, bcs::to_bytes(&case_id));
        vector::append(&mut bytes, bcs::to_bytes(&round));
        vector::append(&mut bytes, bcs::to_bytes(&agent_id));
        vector::append(&mut bytes, bcs::to_bytes(&outcome));
        vector::append(&mut bytes, *rationale_digest);
        vector::append(&mut bytes, bcs::to_bytes(evidence_refs));
        vector::append(&mut bytes, *bundle_digest);
        vector::append(&mut bytes, *salt);
        hash::sha3_256(bytes)
    }

    // ============================================================
    // Tallying, appeals, finality
    // ============================================================

    /// Closes the voting round once the reveal window is over (or every seat revealed), pays duty
    /// score for participation, and either finalizes, opens an appeal window, escalates to a larger
    /// jury, or fails the case.
    public entry fun finalize_round(
        config: &JuryConfig,
        roster: &mut JurorRoster,
        case: &mut JuryCase,
        clock: &Clock,
    ) {
        assert_config_live(config);
        assert!(roster.version == upgrade::current_version(), EWrongVersion);
        assert!(case.version == upgrade::current_version(), EWrongVersion);
        assert!(case.state == CASE_VOTING, EWrongCaseState);
        let now = clock::timestamp_ms(clock);
        let round = case.round;
        {
            let r = vector::borrow(&case.rounds, round);
            assert!(now > r.reveal_deadline_ms || all_revealed(case, round), ERoundNotOver);
        };
        close_round(config, roster, case, now, true);

        let (decisive, leading) = evaluate_round(case, round);
        let max_rounds = vector::length(&case.round_sizes);
        let appeal_deadline_ms = if (decisive && case.appealable && round + 1 < max_rounds) {
            memory::checked_add_u64(now, config.timing.appeal_window_ms)
        } else {
            0
        };
        {
            let r = vector::borrow(&case.rounds, round);
            event::emit(RoundFinalized {
                case_id: object::id(case),
                round,
                tally: r.tally,
                abstain: r.abstain,
                revealed: r.revealed,
                decisive,
                leading: if (decisive) { option::some(leading) } else { option::none() },
                appeal_deadline_ms,
            });
        };

        if (decisive) {
            case.provisional_outcome = option::some(leading);
            if (case.appealable && round + 1 < max_rounds) {
                case.state = CASE_TALLIED;
                case.appeal_deadline_ms = memory::checked_add_u64(now, config.timing.appeal_window_ms);
            } else {
                finalize_outcome(case, leading, now);
            };
        } else if (round + 1 < max_rounds) {
            case.round = round + 1;
            case.state = CASE_AWAITING_DRAW;
        } else {
            end_failed(case, FAIL_CANNOT_DECIDE, now);
        };
    }

    fun all_revealed(case: &JuryCase, round: u64): bool {
        let r = vector::borrow(&case.rounds, round);
        if (vector::is_empty(&r.jurors)) {
            return false
        };
        let mut i = 0;
        let len = vector::length(&r.jurors);
        while (i < len) {
            let id = *vector::borrow(&r.jurors, i);
            if (table::borrow(&case.slots, id).status != SLOT_REVEALED) {
                return false
            };
            i = i + 1;
        };
        true
    }

    /// A round is decisive when enough jurors revealed, and the leading option strictly leads and holds
    /// at least `approval_bps` of the non-abstain reveals. Returns the leading option when decisive.
    fun evaluate_round(case: &JuryCase, round: u64): (bool, u8) {
        let r = vector::borrow(&case.rounds, round);
        if (r.revealed < case.quorum) {
            return (false, ABSTAIN)
        };
        let non_abstain = r.revealed - r.abstain;
        if (non_abstain == 0) {
            return (false, ABSTAIN)
        };
        let mut lead = 0;
        let mut lead_index = 0;
        let mut second = 0;
        let mut i = 0;
        let len = vector::length(&r.tally);
        while (i < len) {
            let count = *vector::borrow(&r.tally, i);
            if (count > lead) {
                second = lead;
                lead = count;
                lead_index = i;
            } else if (count > second) {
                second = count;
            };
            i = i + 1;
        };
        let strong_enough = memory::checked_mul_u64(lead, BPS_DENOMINATOR)
            >= memory::checked_mul_u64(case.approval_bps, non_abstain);
        if (lead > second && strong_enough) {
            (true, lead_index as u8)
        } else {
            (false, ABSTAIN)
        }
    }

    /// Anyone, after the appeal window, finalizes a decisive round; or, once the case has outlived
    /// `max_case_duration`, fails it (or, in an appeal round, lets the first verdict stand).
    public entry fun finalize_case(
        config: &JuryConfig,
        roster: &mut JurorRoster,
        case: &mut JuryCase,
        clock: &Clock,
    ) {
        assert_config_live(config);
        assert!(roster.version == upgrade::current_version(), EWrongVersion);
        assert!(case.version == upgrade::current_version(), EWrongVersion);
        let now = clock::timestamp_ms(clock);
        if (case.state == CASE_TALLIED) {
            assert!(now > case.appeal_deadline_ms, ENotYet);
            let outcome = *option::borrow(&case.provisional_outcome);
            finalize_outcome(case, outcome, now);
            return
        };
        assert!(
            case.state == CASE_EVIDENCE || case.state == CASE_AWAITING_DRAW
                || case.state == CASE_SELECTING || case.state == CASE_VOTING,
            EWrongCaseState,
        );
        let deadline = memory::checked_add_u64(case.created_at_ms, config.timing.max_case_duration_ms);
        assert!(now > deadline, ENotYet);
        close_round(config, roster, case, now, true);
        end_failed(case, FAIL_TIMEOUT, now);
    }

    /// Emergency: ends an unfinished case. It can only fail the case (or leave the first verdict
    /// standing in an appeal round); it never chooses an outcome.
    public entry fun void_case(
        _: &JuryAdminCap,
        config: &JuryConfig,
        roster: &mut JurorRoster,
        case: &mut JuryCase,
        clock: &Clock,
    ) {
        assert_config_live(config);
        assert!(roster.version == upgrade::current_version(), EWrongVersion);
        assert!(case.version == upgrade::current_version(), EWrongVersion);
        force_end(config, roster, case, clock::timestamp_ms(clock));
    }

    /// An integrator ends an unfinished case because its subject was resolved some other way (the
    /// parties agreed). Like [`void_case`], it fails the case without an outcome and frees every seat
    /// without a penalty; a tallied verdict simply becomes final.
    public(package) fun end_case_by_settlement(
        config: &JuryConfig,
        roster: &mut JurorRoster,
        case: &mut JuryCase,
        clock: &Clock,
    ) {
        assert_config_live(config);
        assert!(roster.version == upgrade::current_version(), EWrongVersion);
        assert!(case.version == upgrade::current_version(), EWrongVersion);
        force_end(config, roster, case, clock::timestamp_ms(clock));
    }

    fun force_end(config: &JuryConfig, roster: &mut JurorRoster, case: &mut JuryCase, now: u64) {
        assert!(!is_finished(case.state), EWrongCaseState);
        if (case.state == CASE_TALLIED) {
            let outcome = *option::borrow(&case.provisional_outcome);
            finalize_outcome(case, outcome, now);
            return
        };
        close_round(config, roster, case, now, false);
        end_failed(case, FAIL_VOIDED, now);
    }

    fun finalize_outcome(case: &mut JuryCase, outcome: u8, now: u64) {
        case.state = CASE_FINAL;
        case.final_outcome = option::some(outcome);
        settle_fees(case);
        event::emit(CaseFinalized {
            case_id: object::id(case),
            outcome,
            round: case.round,
            appeals_used: case.appeals_used,
            failed_appeal: case.failed_appeal,
            participants: case.fees.participants,
            per_juror_fee: case.fees.per_juror_fee,
            finalized_at_ms: now,
        });
    }

    /// A failed appeal round leaves the first verdict standing; any other failure ends the case
    /// without an outcome and the integrator applies its failure path.
    fun end_failed(case: &mut JuryCase, reason: u8, now: u64) {
        if (option::is_some(&case.prior_outcome)) {
            let prior = *option::borrow(&case.prior_outcome);
            case.failed_appeal = true;
            finalize_outcome(case, prior, now);
        } else {
            case.state = CASE_FAILED;
            case.fail_reason = reason;
            settle_fees(case);
            event::emit(CaseFailed {
                case_id: object::id(case),
                reason,
                round: case.round,
                participants: case.fees.participants,
                per_juror_fee: case.fees.per_juror_fee,
                failed_at_ms: now,
            });
        };
    }

    // ============================================================
    // Duty score, strikes, seats
    // ============================================================

    /// Settles every seat of the current round once: revealed jurors are credited, jurors who never
    /// accepted (or accepted into a round that ran and never committed or revealed) are penalized,
    /// and everyone is released. A seat that accepted into a round that never started voting is not
    /// penalized: the failure was not the juror's. `penalize = false` (an admin void) releases every
    /// seat without a penalty.
    fun close_round(config: &JuryConfig, roster: &mut JurorRoster, case: &mut JuryCase, now: u64, penalize: bool) {
        let round = case.round;
        if (vector::length(&case.rounds) <= round) {
            return
        };
        if (vector::borrow(&case.rounds, round).closed) {
            return
        };
        let voting = case.state == CASE_VOTING;
        let seats = vector::borrow(&case.rounds, round).jurors;
        let case_id = object::id(case);
        let mut i = 0;
        let len = vector::length(&seats);
        while (i < len) {
            let agent_id = *vector::borrow(&seats, i);
            let status = table::borrow(&case.slots, agent_id).status;
            if (status == SLOT_REVEALED) {
                reward_duty(config, roster, agent_id, case_id, now);
            } else if (penalize && status == SLOT_DRAFTED) {
                table::borrow_mut(&mut case.slots, agent_id).status = SLOT_DEFAULTED;
                penalize_duty(config, roster, agent_id, config.duty.loss_accept, DUTY_NO_ACCEPT, case_id, now);
            } else if (penalize && voting && (status == SLOT_ACCEPTED || status == SLOT_COMMITTED)) {
                table::borrow_mut(&mut case.slots, agent_id).status = SLOT_DEFAULTED;
                penalize_duty(config, roster, agent_id, config.duty.loss_noreveal, DUTY_NO_REVEAL, case_id, now);
            };
            release_seat(roster, agent_id);
            i = i + 1;
        };
        vector::borrow_mut(&mut case.rounds, round).closed = true;
    }

    fun reward_duty(config: &JuryConfig, roster: &mut JurorRoster, agent_id: ID, case_id: ID, now: u64) {
        if (!table::contains(&roster.entries, agent_id)) {
            return
        };
        let entry = table::borrow_mut(&mut roster.entries, agent_id);
        entry.completed = entry.completed + 1;
        entry.last_duty_ms = now;
        let raised = entry.duty_score + config.duty.gain;
        entry.duty_score = if (raised > config.duty.max_score) { config.duty.max_score } else { raised };
        if (entry.strikes > 0) {
            entry.strikes = entry.strikes - 1;
        };
        if (entry.status == ROSTER_SUSPENDED && now >= entry.suspended_until_ms) {
            entry.status = ROSTER_ACTIVE;
        };
        event::emit(DutyScoreChanged {
            agent_id,
            score: entry.duty_score,
            strikes: entry.strikes,
            reason: DUTY_COMPLETED,
            case_id,
        });
    }

    fun penalize_duty(
        config: &JuryConfig,
        roster: &mut JurorRoster,
        agent_id: ID,
        loss: u64,
        reason: u8,
        case_id: ID,
        now: u64,
    ) {
        if (!table::contains(&roster.entries, agent_id)) {
            return
        };
        let entry = table::borrow_mut(&mut roster.entries, agent_id);
        entry.no_shows = entry.no_shows + 1;
        entry.strikes = entry.strikes + 1;
        entry.last_duty_ms = now;
        entry.duty_score = if (entry.duty_score > loss) { entry.duty_score - loss } else { 0 };
        if (entry.strikes >= config.duty.max_strikes) {
            let until = memory::checked_add_u64(now, config.duty.suspension_ms);
            entry.status = ROSTER_SUSPENDED;
            entry.suspended_until_ms = until;
            entry.strikes = 0;
            event::emit(RosterSuspended { agent_id, until_ms: until });
        };
        event::emit(DutyScoreChanged {
            agent_id,
            score: entry.duty_score,
            strikes: entry.strikes,
            reason,
            case_id,
        });
    }

    fun release_seat(roster: &mut JurorRoster, agent_id: ID) {
        if (!table::contains(&roster.entries, agent_id)) {
            return
        };
        let entry = table::borrow_mut(&mut roster.entries, agent_id);
        if (entry.active_cases > 0) {
            entry.active_cases = entry.active_cases - 1;
        };
    }

    // ============================================================
    // Fees
    // ============================================================

    /// Fixes each participant's share: an equal split of the whole pool among the jurors who revealed,
    /// optionally capped. What is not reserved for jurors can be swept by the integrator.
    fun settle_fees(case: &mut JuryCase) {
        if (case.fees.settled) {
            return
        };
        case.fees.settled = true;
        let participants = case.fees.participants;
        if (participants == 0) {
            return
        };
        let net = balance::value(&case.fees.pool);
        let mut share = net / participants;
        if (case.fees.max_juror_fee > 0 && share > case.fees.max_juror_fee) {
            share = case.fees.max_juror_fee;
        };
        case.fees.per_juror_fee = share;
        case.fees.reserved = memory::checked_mul_u64(share, participants);
    }

    /// Anyone triggers a participant's payment; the money goes to the payout address on the roster.
    public entry fun claim_juror_fee(case: &mut JuryCase, agent_id: ID, ctx: &mut TxContext) {
        assert!(case.fees.settled, EFeesNotSettled);
        assert!(table::contains(&case.slots, agent_id), ENotDrafted);
        let (recipient, amount) = {
            let slot = table::borrow_mut(&mut case.slots, agent_id);
            assert!(slot.status == SLOT_REVEALED, EWrongSlotStatus);
            assert!(!slot.claimed, EAlreadyClaimed);
            slot.claimed = true;
            (slot.payout_address, case.fees.per_juror_fee)
        };
        if (amount > 0) {
            case.fees.reserved = case.fees.reserved - amount;
            let payment = coin::from_balance(balance::split(&mut case.fees.pool, amount), ctx);
            transfer::public_transfer(payment, recipient);
        };
        event::emit(JurorPaid { case_id: object::id(case), agent_id, recipient, amount_mist: amount });
    }

    // ============================================================
    // Authorization and version helpers
    // ============================================================

    fun assert_live(config: &JuryConfig, roster: &JurorRoster, case: &JuryCase) {
        assert_config_live(config);
        assert!(config.active, EJuryPaused);
        assert!(roster.version == upgrade::current_version(), EWrongVersion);
        assert!(case.version == upgrade::current_version(), EWrongVersion);
    }

    /// The juror's owner, or the juror agent itself holding the jury capability. A different agent
    /// of the same owner is rejected.
    fun assert_juror_actor(
        memory_config: &MemoryConfig,
        account: &MemoryAccount,
        agent: &SubAgent,
        clock: &Clock,
        ctx: &TxContext,
    ) {
        assert!(memory::sub_agent_memory_account_id(agent) == object::id(account), EWrongAgent);
        let sender = tx_context::sender(ctx);
        if (sender == memory::owner(account)) {
            memory::resolve_human_actor(account, ctx);
        } else {
            assert!(sender == memory::sub_agent_derived_address(agent), ENotJuror);
            let cap = memory::cap_jury();
            let acting = memory::resolve_actor_with_cap(
                memory_config, account, cap, option::none(), 0, clock, ctx,
            );
            memory::assert_direct_execution_allowed(account, cap, ctx);
            assert!(option::contains(&memory::acting_sub_agent_id(&acting), &object::id(agent)), ENotJuror);
        };
    }

    // ============================================================
    // Views
    // ============================================================

    public fun subject_task(): u8 { SUBJECT_TASK }
    public fun subject_governance(): u8 { SUBJECT_GOVERNANCE }
    public fun abstain(): u8 { ABSTAIN }
    public fun case_evidence(): u8 { CASE_EVIDENCE }
    public fun case_awaiting_draw(): u8 { CASE_AWAITING_DRAW }
    public fun case_selecting(): u8 { CASE_SELECTING }
    public fun case_voting(): u8 { CASE_VOTING }
    public fun case_tallied(): u8 { CASE_TALLIED }
    public fun case_final(): u8 { CASE_FINAL }
    public fun case_failed(): u8 { CASE_FAILED }
    public fun case_cancelled(): u8 { CASE_CANCELLED }
    public fun fail_cannot_form(): u8 { FAIL_CANNOT_FORM }
    public fun fail_cannot_decide(): u8 { FAIL_CANNOT_DECIDE }
    public fun fail_timeout(): u8 { FAIL_TIMEOUT }
    public fun fail_voided(): u8 { FAIL_VOIDED }
    public fun slot_drafted(): u8 { SLOT_DRAFTED }
    public fun slot_accepted(): u8 { SLOT_ACCEPTED }
    public fun slot_committed(): u8 { SLOT_COMMITTED }
    public fun slot_revealed(): u8 { SLOT_REVEALED }
    public fun slot_replaced(): u8 { SLOT_REPLACED }
    public fun slot_defaulted(): u8 { SLOT_DEFAULTED }
    public fun roster_active(): u8 { ROSTER_ACTIVE }
    public fun roster_paused(): u8 { ROSTER_PAUSED }
    public fun roster_suspended(): u8 { ROSTER_SUSPENDED }

    public fun config_active(config: &JuryConfig): bool { config.active }
    /// The most owners, organizations and agents a case may exclude; integrators trim to it.
    public fun max_exclusions(config: &JuryConfig): u64 { config.selection.max_exclusions }
    public fun neutral_window_ms(config: &JuryConfig): u64 { config.timing.neutral_window_ms }
    public fun accept_window_ms(config: &JuryConfig): u64 { config.timing.accept_window_ms }
    public fun commit_window_ms(config: &JuryConfig): u64 { config.timing.commit_window_ms }
    public fun reveal_window_ms(config: &JuryConfig): u64 { config.timing.reveal_window_ms }
    public fun reveal_grace_ms(config: &JuryConfig): u64 { config.timing.reveal_grace_ms }
    public fun appeal_window_ms(config: &JuryConfig): u64 { config.timing.appeal_window_ms }
    public fun max_case_duration_ms(config: &JuryConfig): u64 { config.timing.max_case_duration_ms }
    public fun min_eligible_roster(config: &JuryConfig): u64 { config.eligibility.min_eligible_roster }
    public fun initial_duty_score(config: &JuryConfig): u64 { config.duty.initial_score }
    public fun genesis_sunset_ms(config: &JuryConfig): u64 { config.genesis_sunset_ms }

    public fun roster_count(roster: &JurorRoster): u64 { roster.count }
    public fun on_roster(roster: &JurorRoster, agent_id: ID): bool { table::contains(&roster.entries, agent_id) }
    public fun entry_status(roster: &JurorRoster, agent_id: ID): u8 { table::borrow(&roster.entries, agent_id).status }
    public fun entry_duty_score(roster: &JurorRoster, agent_id: ID): u64 { table::borrow(&roster.entries, agent_id).duty_score }
    public fun entry_strikes(roster: &JurorRoster, agent_id: ID): u64 { table::borrow(&roster.entries, agent_id).strikes }
    public fun entry_completed(roster: &JurorRoster, agent_id: ID): u64 { table::borrow(&roster.entries, agent_id).completed }
    public fun entry_no_shows(roster: &JurorRoster, agent_id: ID): u64 { table::borrow(&roster.entries, agent_id).no_shows }
    public fun entry_active_cases(roster: &JurorRoster, agent_id: ID): u64 { table::borrow(&roster.entries, agent_id).active_cases }
    public fun entry_exempt(roster: &JurorRoster, agent_id: ID): bool { table::borrow(&roster.entries, agent_id).exempt }
    public fun entry_owner(roster: &JurorRoster, agent_id: ID): address { table::borrow(&roster.entries, agent_id).owner }
    public fun entry_suspended_until_ms(roster: &JurorRoster, agent_id: ID): u64 {
        table::borrow(&roster.entries, agent_id).suspended_until_ms
    }

    public fun case_state(case: &JuryCase): u8 { case.state }
    public fun case_is_finished(case: &JuryCase): bool { is_finished(case.state) }
    /// True until the first jury is drawn: the window in which a case can still be cancelled and a
    /// second party can still add its fee.
    public fun case_undrawn(case: &JuryCase): bool { case.round == 0 && vector::length(&case.rounds) == 0 }
    public fun case_subject_kind(case: &JuryCase): u8 { case.subject_kind }
    public fun case_subject_id(case: &JuryCase): ID { case.subject_id }
    public fun case_stage(case: &JuryCase): u8 { case.stage }
    public fun case_round(case: &JuryCase): u64 { case.round }
    public fun case_menu_len(case: &JuryCase): u8 { case.menu_len }
    public fun case_failure_path(case: &JuryCase): u8 { case.failure_path }
    public fun case_fail_reason(case: &JuryCase): u8 { case.fail_reason }
    public fun case_final_outcome(case: &JuryCase): Option<u8> { case.final_outcome }
    public fun case_provisional_outcome(case: &JuryCase): Option<u8> { case.provisional_outcome }
    public fun case_prior_outcome(case: &JuryCase): Option<u8> { case.prior_outcome }
    public fun case_executed(case: &JuryCase): bool { case.executed }
    public fun case_failed_appeal(case: &JuryCase): bool { case.failed_appeal }
    public fun case_appeals_used(case: &JuryCase): u64 { case.appeals_used }
    public fun case_appeal_deadline_ms(case: &JuryCase): u64 { case.appeal_deadline_ms }
    public fun case_evidence_deadline_ms(case: &JuryCase): u64 { case.evidence_deadline_ms }
    public fun case_evidence_count(case: &JuryCase): u64 { vector::length(&case.evidence) }
    public fun case_participants(case: &JuryCase): u64 { case.fees.participants }
    public fun case_per_juror_fee(case: &JuryCase): u64 { case.fees.per_juror_fee }
    public fun case_fee_pool_mist(case: &JuryCase): u64 { balance::value(&case.fees.pool) }
    public fun case_fees_settled(case: &JuryCase): bool { case.fees.settled }
    public fun case_sweep_to(case: &JuryCase): address { case.fees.sweep_to }

    /// The fixed outcome an integrator may act on: set only in FINAL, never for a failed or cancelled
    /// case.
    public fun finalized_outcome(case: &JuryCase): Option<u8> { case.final_outcome }

    public fun round_jurors(case: &JuryCase, round: u64): vector<ID> { vector::borrow(&case.rounds, round).jurors }
    public fun round_tally(case: &JuryCase, round: u64): vector<u64> { vector::borrow(&case.rounds, round).tally }
    public fun round_revealed(case: &JuryCase, round: u64): u64 { vector::borrow(&case.rounds, round).revealed }
    public fun round_accept_deadline_ms(case: &JuryCase, round: u64): u64 {
        vector::borrow(&case.rounds, round).accept_deadline_ms
    }
    public fun round_commit_deadline_ms(case: &JuryCase, round: u64): u64 {
        vector::borrow(&case.rounds, round).commit_deadline_ms
    }
    public fun round_reveal_deadline_ms(case: &JuryCase, round: u64): u64 {
        vector::borrow(&case.rounds, round).reveal_deadline_ms
    }
    public fun has_slot(case: &JuryCase, agent_id: ID): bool { table::contains(&case.slots, agent_id) }
    public fun slot_status(case: &JuryCase, agent_id: ID): u8 { table::borrow(&case.slots, agent_id).status }
    public fun slot_owner(case: &JuryCase, agent_id: ID): address { table::borrow(&case.slots, agent_id).owner }

    // ============================================================
    // Test helpers
    // ============================================================

    #[test_only]
    public fun test_bootstrap(clock: &Clock, ctx: &mut TxContext) {
        bootstrap_init(clock, ctx);
    }

    #[test_only]
    public fun test_create_admin_cap(ctx: &mut TxContext): JuryAdminCap {
        create_jury_admin_cap(ctx)
    }

    /// `select_jury` and `replace_no_shows` are private `entry` functions (they take `Random`), so
    /// other modules cannot call them; tests go through these wrappers.
    #[test_only]
    public fun test_select_jury(
        config: &JuryConfig,
        roster: &mut JurorRoster,
        case: &mut JuryCase,
        rand: &Random,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        select_jury(config, roster, case, rand, clock, ctx);
    }

    #[test_only]
    public fun test_replace_no_shows(
        config: &JuryConfig,
        roster: &mut JurorRoster,
        case: &mut JuryCase,
        rand: &Random,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        replace_no_shows(config, roster, case, rand, clock, ctx);
    }
}
