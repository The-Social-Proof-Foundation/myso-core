// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

/// MIST-denominated AI spend escrow. Users deposit MYSO and the platform oracle reserves
/// the worst-case MIST charge before provider work begins, then captures the actual charge
/// or releases the reservation. Legacy signed usage receipts remain available during the
/// reserve-and-capture migration.

#[allow(duplicate_alias, lint(public_entry))]
module social_contracts::ai_credit {
    use std::bcs;
    use std::option::{Self, Option};
    use std::string;
    use std::vector;

    use myso::{
        balance::{Self, Balance},
        clock::{Self, Clock},
        coin::{Self, Coin},
        dynamic_field as df,
        ed25519,
        event,
        object::{Self, ID, UID},
        permissioned_group::PermissionedGroup,
        table::{Self, Table},
        transfer,
        tx_context::{Self, TxContext},
    };
    use myso::myso::MYSO;

    use social_contracts::memory::{
        Self,
        AgenticOrganization,
        MemoryAccount,
        MemoryConfig,
        MemorySharePackage,
        OrgBudgetManager,
        OrgSpendApprover,
        SubAgent,
    };
    use social_contracts::upgrade::{Self, UpgradeAdminCap};

    const MIST_PER_MYSO: u64 = 1_000_000_000;
    const DAY_MS: u64 = 86_400_000;
    const MONTH_MS: u64 = 30 * DAY_MS;
    const INTENT_AI_CREDIT_USAGE: u8 = 1;
    const INTENT_AI_CREDIT_RESERVE: u8 = 2;
    const INTENT_AI_CREDIT_CAPTURE: u8 = 3;
    const INTENT_AI_CREDIT_CANCEL: u8 = 4;
    const ED25519_PK_LEN: u64 = 32;
    const HASH_LEN: u64 = 32;
    const DEFAULT_ORACLE_MARKUP_BPS: u64 = 1500;
    const MAX_CAPTURE_WINDOW_MS: u64 = 15 * 60 * 1000;
    const MAX_RESERVATION_LIFETIME_MS: u64 = 30 * 60 * 1000;

    const USAGE_INFERENCE: u8 = 1;
    const USAGE_TOOL: u8 = 2;
    const USAGE_EMBED: u8 = 3;
    const USAGE_STORAGE: u8 = 4;
    const USAGE_WORKFLOW: u8 = 5;

    const ENotOwner: u64 = 1;
    const EWrongVersion: u64 = 2;
    const EInvalidAmount: u64 = 3;
    const EInsufficientBalance: u64 = 4;
    const EInactive: u64 = 5;
    const EInvalidSignature: u64 = 6;
    const EStaleReceipt: u64 = 7;
    const EInvalidNonce: u64 = 8;
    const ECapExceeded: u64 = 9;
    const EAgentNotFound: u64 = 10;
    const EAgentDisabled: u64 = 11;
    const EInvalidPubkey: u64 = 12;
    const EAccountMismatch: u64 = 13;
    const EAgentMissingCap: u64 = 15;
    const EBalanceAlreadyExists: u64 = 17;
    const EApprovalRequired: u64 = 18;
    const EApprovalExpired: u64 = 19;
    const EApprovalInsufficient: u64 = 20;
    const EApprovalNotFound: u64 = 21;
    const ENotDescendant: u64 = 22;
    const ENotParentSigner: u64 = 23;
    const EParentEnvelopeExceeded: u64 = 24;
    const ECannotManageSelf: u64 = 25;
    const EAgentNotInOrg: u64 = 26;
    const EInvalidExpiry: u64 = 27;
    const EReservationNotFound: u64 = 28;
    const EReservationExpired: u64 = 29;
    const EReservationNotExpired: u64 = 30;
    const ECaptureWindowClosed: u64 = 31;
    const EReservationMismatch: u64 = 32;
    const EInvalidReservationWindow: u64 = 33;
    const EInvalidHash: u64 = 34;
    const EMarkupMismatch: u64 = 35;

    public struct AiCreditOracleAdminCap has key, store {
        id: UID,
    }

    public struct AiCreditConfig has key {
        id: UID,
        oracle_pubkey: vector<u8>,
        treasury: address,
        min_deposit_mist: u64,
        max_single_settlement_mist: u64,
        receipt_ttl_ms: u64,
        oracle_markup_bps: u64,
        balances_by_memory: Table<ID, ID>,
        version: u64,
    }

    public struct AgentBudgetEntry has store, copy, drop {
        agent_object_id: ID,
        derived_address: address,
        enabled: bool,
        budget_mist: Option<u64>,
        spent_mist: u64,
        reserved_mist: u64,
        daily_cap_mist: Option<u64>,
        monthly_cap_mist: Option<u64>,
        spent_day_mist: u64,
        reserved_day_mist: u64,
        spent_month_mist: u64,
        reserved_month_mist: u64,
        day_anchor_ms: u64,
        month_anchor_ms: u64,
        require_approval_above_mist: Option<u64>,
    }

    /// Dynamic-field key on `AiCreditBalance.id` for the agent's live spend allowance.
    /// One allowance per agent; re-approving overwrites. Stored as a dynamic field so the
    /// `AiCreditBalance` struct layout never changes (upgrade-safe).
    public struct SpendApprovalKey has copy, drop, store {
        agent_object_id: ID,
    }

    /// One-shot spend allowance consumed by the first over-threshold settlement it covers.
    public struct SpendApproval has store, copy, drop {
        max_amount_mist: u64,
        expires_at_ms: u64,
        approved_by: address,
        approval_nonce: u64,
    }

    /// Dynamic-field key on `AiCreditBalance.id` for the monotonic approval nonce counter.
    public struct ApprovalNonceKey has copy, drop, store {}

    public struct AiCreditBalance has key {
        id: UID,
        memory_account_id: ID,
        principal_owner: address,
        profile_id: address,
        balance: Balance<MYSO>,
        spent_total_mist: u64,
        reserved_mist: u64,
        spent_day_mist: u64,
        reserved_day_mist: u64,
        spent_month_mist: u64,
        reserved_month_mist: u64,
        day_anchor_ms: u64,
        month_anchor_ms: u64,
        daily_cap_mist: Option<u64>,
        monthly_cap_mist: Option<u64>,
        settlement_nonce: u64,
        reservation_nonce: u64,
        reservations: Table<u64, SpendReservation>,
        agent_budgets: Table<ID, AgentBudgetEntry>,
        active: bool,
        version: u64,
    }

  public struct IntentMessage<T: drop> has copy, drop {
        intent: u8,
        timestamp_ms: u64,
        payload: T,
    }

    public struct UsageReceipt has copy, drop {
        balance_id: ID,
        agent_object_id: ID,
        receipt_id: u128,
        amount_mist: u64,
        usage_kind: u8,
        timestamp_ms: u64,
        settlement_nonce: u64,
    }

    /// Oracle-signed authorization to lock a deterministic worst-case MIST charge.
    public struct SpendReservationIntent has copy, drop {
        balance_id: ID,
        agent_object_id: ID,
        reservation_nonce: u64,
        max_amount_mist: u64,
        provider_envelope_hash: vector<u8>,
        request_hash: vector<u8>,
        fx_quote_id: vector<u8>,
        myso_usd_e8: u64,
        markup_bps: u64,
        timestamp_ms: u64,
        capture_deadline_ms: u64,
        hard_expiry_ms: u64,
    }

    /// Canonical live reservation stored directly in the greenfield account object.
    public struct SpendReservation has store, copy, drop {
        reservation_nonce: u64,
        agent_object_id: ID,
        max_amount_mist: u64,
        provider_envelope_hash: vector<u8>,
        request_hash: vector<u8>,
        fx_quote_id: vector<u8>,
        myso_usd_e8: u64,
        markup_bps: u64,
        created_at_ms: u64,
        capture_deadline_ms: u64,
        hard_expiry_ms: u64,
        account_day_anchor_ms: u64,
        account_month_anchor_ms: u64,
        agent_budget_reserved: bool,
        agent_day_anchor_ms: u64,
        agent_month_anchor_ms: u64,
    }

    /// Oracle-signed final provider charge for an existing reservation.
    public struct CaptureSpendIntent has copy, drop {
        balance_id: ID,
        agent_object_id: ID,
        reservation_nonce: u64,
        amount_mist: u64,
        provider_cost_usd_micros: u64,
        provider_generation_hash: vector<u8>,
        timestamp_ms: u64,
    }

    /// Oracle-signed release when the provider confirms that no billable generation exists.
    public struct CancelSpendIntent has copy, drop {
        balance_id: ID,
        agent_object_id: ID,
        reservation_nonce: u64,
        timestamp_ms: u64,
    }

    public struct AiCreditBalanceCreated has copy, drop {
        balance_id: ID,
        memory_account_id: ID,
        principal_owner: address,
        profile_id: address,
    }

    public struct AiCreditDeposited has copy, drop {
        balance_id: ID,
        amount_mist: u64,
        new_balance_mist: u64,
    }

    public struct AiCreditWithdrawn has copy, drop {
        balance_id: ID,
        amount_mist: u64,
        new_balance_mist: u64,
    }

    public struct AiCreditAccountCapsUpdated has copy, drop {
        balance_id: ID,
        daily_cap_mist: Option<u64>,
        monthly_cap_mist: Option<u64>,
    }

    public struct AiCreditAgentBudgetUpdated has copy, drop {
        balance_id: ID,
        agent_object_id: ID,
        budget_mist: Option<u64>,
        daily_cap_mist: Option<u64>,
        monthly_cap_mist: Option<u64>,
        require_approval_above_mist: Option<u64>,
    }

    public struct AiCreditAgentBudgetDisabled has copy, drop {
        balance_id: ID,
        agent_object_id: ID,
    }

    /// Audit-grade budget change event carrying previous and new values plus the actor.
    /// Emitted alongside the legacy `AiCreditAgentBudgetUpdated`/`Disabled` events.
    public struct AiCreditAgentBudgetChanged has copy, drop {
        balance_id: ID,
        agent_object_id: ID,
        had_previous_entry: bool,
        prev_budget_mist: Option<u64>,
        prev_daily_cap_mist: Option<u64>,
        prev_monthly_cap_mist: Option<u64>,
        prev_require_approval_above_mist: Option<u64>,
        prev_enabled: bool,
        budget_mist: Option<u64>,
        daily_cap_mist: Option<u64>,
        monthly_cap_mist: Option<u64>,
        require_approval_above_mist: Option<u64>,
        enabled: bool,
        set_by: address,
        /// Set when a parent agent changed a descendant's budget.
        set_by_agent_id: Option<ID>,
        /// Set when the change went through an org role gate.
        organization_id: Option<ID>,
        timestamp_ms: u64,
    }

    public struct AiCreditSpendApproved has copy, drop {
        balance_id: ID,
        agent_object_id: ID,
        approval_nonce: u64,
        max_amount_mist: u64,
        expires_at_ms: u64,
        approved_by: address,
        /// Set when a parent agent approved a descendant's spend.
        approved_by_agent_id: Option<ID>,
        /// Set when the approval went through an org role gate.
        organization_id: Option<ID>,
        timestamp_ms: u64,
    }

    public struct AiCreditSpendApprovalRevoked has copy, drop {
        balance_id: ID,
        agent_object_id: ID,
        approval_nonce: u64,
        revoked_by: address,
        timestamp_ms: u64,
    }

    public struct AiCreditSpendApprovalConsumed has copy, drop {
        balance_id: ID,
        agent_object_id: ID,
        approval_nonce: u64,
        amount_mist: u64,
        approved_by: address,
        timestamp_ms: u64,
    }

    public struct AiCreditUsageSettled has copy, drop {
        balance_id: ID,
        agent_object_id: ID,
        receipt_id: u128,
        amount_mist: u64,
        usage_kind: u8,
        settlement_nonce: u64,
        remaining_mist: u64,
    }

    public struct AiSpendReserved has copy, drop {
        balance_id: ID,
        agent_object_id: ID,
        reservation_nonce: u64,
        max_amount_mist: u64,
        provider_envelope_hash: vector<u8>,
        request_hash: vector<u8>,
        fx_quote_id: vector<u8>,
        myso_usd_e8: u64,
        markup_bps: u64,
        capture_deadline_ms: u64,
        hard_expiry_ms: u64,
        available_mist: u64,
    }

    public struct AiSpendCaptured has copy, drop {
        balance_id: ID,
        agent_object_id: ID,
        reservation_nonce: u64,
        reserved_mist: u64,
        captured_mist: u64,
        released_mist: u64,
        provider_cost_usd_micros: u64,
        provider_generation_hash: vector<u8>,
        fx_quote_id: vector<u8>,
        myso_usd_e8: u64,
        markup_bps: u64,
        captured_at_ms: u64,
        remaining_mist: u64,
        available_mist: u64,
    }

    public struct AiSpendCancelled has copy, drop {
        balance_id: ID,
        agent_object_id: ID,
        reservation_nonce: u64,
        released_mist: u64,
        cancelled_at_ms: u64,
        available_mist: u64,
    }

    public struct AiSpendExpired has copy, drop {
        balance_id: ID,
        agent_object_id: ID,
        reservation_nonce: u64,
        released_mist: u64,
        expired_at_ms: u64,
        available_mist: u64,
    }

    public struct AiCreditBalanceDepleted has copy, drop {
        balance_id: ID,
    }

    public struct AiCreditBalancePaused has copy, drop {
        balance_id: ID,
    }

    public struct AiCreditBalanceReactivated has copy, drop {
        balance_id: ID,
    }

    public struct AiCreditOraclePubkeyUpdated has copy, drop {
        updated_by: address,
        new_pubkey: vector<u8>,
    }

    public struct AiCreditSettlementLimitsUpdated has copy, drop {
        max_single_settlement_mist: u64,
        receipt_ttl_ms: u64,
    }

    public struct AiCreditMarkupUpdated has copy, drop {
        updated_by: address,
        oracle_markup_bps: u64,
    }

    public struct AiCreditMinDepositUpdated has copy, drop {
        updated_by: address,
        min_deposit_mist: u64,
    }

    public struct AiCreditConfigInitialized has copy, drop {
        oracle_pubkey: vector<u8>,
        treasury: address,
        min_deposit_mist: u64,
        max_single_settlement_mist: u64,
        receipt_ttl_ms: u64,
        oracle_markup_bps: u64,
    }

    // ============================================================
    // Bootstrap
    // ============================================================

    public(package) fun bootstrap_init(treasury: address, oracle_pubkey: vector<u8>, ctx: &mut TxContext) {
        assert!(vector::length(&oracle_pubkey) == ED25519_PK_LEN, EInvalidPubkey);
        let min_deposit_mist = MIST_PER_MYSO;
        let max_single_settlement_mist = 1000 * MIST_PER_MYSO;
        let receipt_ttl_ms = 300_000;
        let oracle_markup_bps = DEFAULT_ORACLE_MARKUP_BPS;
        event::emit(AiCreditConfigInitialized {
            oracle_pubkey,
            treasury,
            min_deposit_mist,
            max_single_settlement_mist,
            receipt_ttl_ms,
            oracle_markup_bps,
        });
        let config = AiCreditConfig {
            id: object::new(ctx),
            oracle_pubkey,
            treasury,
            min_deposit_mist,
            max_single_settlement_mist,
            receipt_ttl_ms,
            oracle_markup_bps,
            balances_by_memory: table::new(ctx),
            version: upgrade::current_version(),
        };
        transfer::share_object(config);
    }

    public(package) fun create_oracle_admin_cap(ctx: &mut TxContext): AiCreditOracleAdminCap {
        AiCreditOracleAdminCap {
            id: object::new(ctx),
        }
    }

    // ============================================================
    // Balance lifecycle
    // ============================================================

    /// Called only from [`profile::create_profile`] — one balance per memory account.
    public(package) fun create_and_share_balance(
        config: &mut AiCreditConfig,
        memory_account_id: ID,
        principal_owner: address,
        profile_id: address,
        clock: &Clock,
        ctx: &mut TxContext,
    ): ID {
        assert!(!table::contains(&config.balances_by_memory, memory_account_id), EBalanceAlreadyExists);
        let balance = new_balance(memory_account_id, principal_owner, profile_id, clock, ctx);
        let balance_id = object::id(&balance);
        table::add(&mut config.balances_by_memory, memory_account_id, balance_id);
        event::emit(AiCreditBalanceCreated {
            balance_id,
            memory_account_id,
            principal_owner,
            profile_id,
        });
        transfer::share_object(balance);
        balance_id
    }

    fun new_balance(
        memory_account_id: ID,
        principal_owner: address,
        profile_id: address,
        clock: &Clock,
        ctx: &mut TxContext,
    ): AiCreditBalance {
        let now = clock::timestamp_ms(clock);
        AiCreditBalance {
            id: object::new(ctx),
            memory_account_id,
            principal_owner,
            profile_id,
            balance: balance::zero(),
            spent_total_mist: 0,
            reserved_mist: 0,
            spent_day_mist: 0,
            reserved_day_mist: 0,
            spent_month_mist: 0,
            reserved_month_mist: 0,
            day_anchor_ms: now,
            month_anchor_ms: now,
            daily_cap_mist: option::none(),
            monthly_cap_mist: option::none(),
            settlement_nonce: 0,
            reservation_nonce: 0,
            reservations: table::new(ctx),
            agent_budgets: table::new(ctx),
            active: true,
            version: upgrade::current_version(),
        }
    }

    public entry fun deposit(
        config: &AiCreditConfig,
        balance: &mut AiCreditBalance,
        payment: Coin<MYSO>,
        ctx: &mut TxContext,
    ) {
        assert_version(config, balance);
        assert_owner(balance, ctx);
        assert_active(balance);
        let amount = coin::value(&payment);
        assert!(amount >= config.min_deposit_mist, EInvalidAmount);
        balance::join(&mut balance.balance, coin::into_balance(payment));
        let new_balance_mist = balance_mist(balance);
        event::emit(AiCreditDeposited {
            balance_id: object::id(balance),
            amount_mist: amount,
            new_balance_mist,
        });
    }

    public entry fun withdraw(
        config: &AiCreditConfig,
        balance: &mut AiCreditBalance,
        amount_mist: u64,
        ctx: &mut TxContext,
    ) {
        assert_version(config, balance);
        assert_owner(balance, ctx);
        assert_active(balance);
        assert!(amount_mist > 0, EInvalidAmount);
        let available = available_mist(balance);
        assert!(amount_mist <= available, EInsufficientBalance);
        let payout = balance::split(&mut balance.balance, amount_mist);
        transfer::public_transfer(coin::from_balance(payout, ctx), balance.principal_owner);
        event::emit(AiCreditWithdrawn {
            balance_id: object::id(balance),
            amount_mist,
            new_balance_mist: balance_mist(balance),
        });
    }

    public entry fun set_account_caps(
        config: &AiCreditConfig,
        balance: &mut AiCreditBalance,
        daily_cap_mist: Option<u64>,
        monthly_cap_mist: Option<u64>,
        ctx: &TxContext,
    ) {
        assert_version(config, balance);
        assert_owner(balance, ctx);
        balance.daily_cap_mist = daily_cap_mist;
        balance.monthly_cap_mist = monthly_cap_mist;
        event::emit(AiCreditAccountCapsUpdated {
            balance_id: object::id(balance),
            daily_cap_mist,
            monthly_cap_mist,
        });
    }

    public entry fun set_agent_budget(
        config: &AiCreditConfig,
        balance: &mut AiCreditBalance,
        agent: &SubAgent,
        budget_mist: Option<u64>,
        daily_cap_mist: Option<u64>,
        monthly_cap_mist: Option<u64>,
        require_approval_above_mist: Option<u64>,
        clock: &Clock,
        ctx: &TxContext,
    ) {
        assert_version(config, balance);
        assert_owner(balance, ctx);
        assert_agent_linked(balance, agent);
        memory::assert_sub_agent_active(agent, clock);
        upsert_agent_budget(
            balance,
            agent,
            budget_mist,
            daily_cap_mist,
            monthly_cap_mist,
            require_approval_above_mist,
            tx_context::sender(ctx),
            option::none(),
            option::none(),
            clock,
        );
    }

    /// Org role-gated budget management: a holder of `OrgBudgetManager` on the org's memory
    /// share group may manage budgets for agents belonging to that org, without the owner key.
    public entry fun set_agent_budget_as_manager(
        config: &AiCreditConfig,
        balance: &mut AiCreditBalance,
        account: &MemoryAccount,
        org: &AgenticOrganization,
        group: &PermissionedGroup<MemorySharePackage>,
        agent: &SubAgent,
        budget_mist: Option<u64>,
        daily_cap_mist: Option<u64>,
        monthly_cap_mist: Option<u64>,
        require_approval_above_mist: Option<u64>,
        clock: &Clock,
        ctx: &TxContext,
    ) {
        assert_version(config, balance);
        assert_active(balance);
        assert_agent_linked(balance, agent);
        assert_org_gate_for_agent(balance, account, org, agent);
        memory::assert_org_permission<OrgBudgetManager>(org, group, tx_context::sender(ctx));
        memory::assert_sub_agent_active(agent, clock);
        upsert_agent_budget(
            balance,
            agent,
            budget_mist,
            daily_cap_mist,
            monthly_cap_mist,
            require_approval_above_mist,
            tx_context::sender(ctx),
            option::none(),
            option::some(memory::organization_id(org)),
            clock,
        );
    }

    public entry fun disable_agent_budget(
        config: &AiCreditConfig,
        balance: &mut AiCreditBalance,
        agent_object_id: ID,
        clock: &Clock,
        ctx: &TxContext,
    ) {
        assert_version(config, balance);
        assert_owner(balance, ctx);
        disable_agent_budget_internal(
            balance,
            agent_object_id,
            tx_context::sender(ctx),
            option::none(),
            clock,
        );
    }

    // ============================================================
    // Spend approvals (one-shot allowances)
    //
    // Reject-before-sign contract with the oracle: an over-threshold usage request is
    // rejected off-chain until a live allowance exists, so no receipt (and no settlement
    // nonce) is ever created for an unapprovable spend. On-chain, `execute_settlement`
    // consumes the allowance — the chain is the enforcer, the oracle only pre-checks.
    // ============================================================

    /// Owner grants a one-shot allowance: the agent may settle a single usage receipt up to
    /// `max_amount_mist` above its approval threshold, until `expires_at_ms`. Re-approving
    /// overwrites any existing allowance for the agent.
    public entry fun approve_agent_spend(
        config: &AiCreditConfig,
        balance: &mut AiCreditBalance,
        agent_object_id: ID,
        max_amount_mist: u64,
        expires_at_ms: u64,
        clock: &Clock,
        ctx: &TxContext,
    ) {
        assert_version(config, balance);
        assert_owner(balance, ctx);
        assert_active(balance);
        store_spend_approval(
            balance,
            agent_object_id,
            max_amount_mist,
            expires_at_ms,
            tx_context::sender(ctx),
            option::none(),
            option::none(),
            clock,
        );
    }

    /// Org role-gated approval: a holder of `OrgSpendApprover` on the org's memory share
    /// group may approve spends for agents belonging to that org (Finance Approver flow).
    public entry fun approve_agent_spend_as_approver(
        config: &AiCreditConfig,
        balance: &mut AiCreditBalance,
        account: &MemoryAccount,
        org: &AgenticOrganization,
        group: &PermissionedGroup<MemorySharePackage>,
        agent: &SubAgent,
        max_amount_mist: u64,
        expires_at_ms: u64,
        clock: &Clock,
        ctx: &TxContext,
    ) {
        assert_version(config, balance);
        assert_active(balance);
        assert_agent_linked(balance, agent);
        assert_org_gate_for_agent(balance, account, org, agent);
        memory::assert_org_permission<OrgSpendApprover>(org, group, tx_context::sender(ctx));
        store_spend_approval(
            balance,
            memory::agent_object_id(agent),
            max_amount_mist,
            expires_at_ms,
            tx_context::sender(ctx),
            option::none(),
            option::some(memory::organization_id(org)),
            clock,
        );
    }

    /// Owner revokes an agent's live allowance.
    public entry fun revoke_agent_spend_approval(
        config: &AiCreditConfig,
        balance: &mut AiCreditBalance,
        agent_object_id: ID,
        clock: &Clock,
        ctx: &TxContext,
    ) {
        assert_version(config, balance);
        assert_owner(balance, ctx);
        let key = SpendApprovalKey { agent_object_id };
        assert!(
            df::exists_with_type<SpendApprovalKey, SpendApproval>(&balance.id, key),
            EApprovalNotFound,
        );
        let approval: SpendApproval = df::remove(&mut balance.id, key);
        event::emit(AiCreditSpendApprovalRevoked {
            balance_id: object::id(balance),
            agent_object_id,
            approval_nonce: approval.approval_nonce,
            revoked_by: tx_context::sender(ctx),
            timestamp_ms: clock::timestamp_ms(clock),
        });
    }

    // ============================================================
    // Delegated budgets (parent agents manage descendants without human txs)
    // ============================================================

    /// Parent agent (holding `CAP_BUDGET_MANAGE`) sets a descendant's budget. Child limits
    /// must be at least as strict as the parent's own envelope; the human owner remains the
    /// unconstrained root.
    public entry fun set_child_agent_budget(
        config: &AiCreditConfig,
        memory_config: &MemoryConfig,
        balance: &mut AiCreditBalance,
        account: &MemoryAccount,
        parent: &SubAgent,
        child: &SubAgent,
        budget_mist: Option<u64>,
        daily_cap_mist: Option<u64>,
        monthly_cap_mist: Option<u64>,
        require_approval_above_mist: Option<u64>,
        clock: &Clock,
        ctx: &TxContext,
    ) {
        assert_version(config, balance);
        assert_active(balance);
        assert_parent_manages_child(memory_config, balance, account, parent, child, clock, ctx);
        assert_child_budget_within_parent_envelope(
            balance,
            memory::agent_object_id(parent),
            &budget_mist,
            &daily_cap_mist,
            &monthly_cap_mist,
            &require_approval_above_mist,
        );
        memory::assert_sub_agent_active(child, clock);
        upsert_agent_budget(
            balance,
            child,
            budget_mist,
            daily_cap_mist,
            monthly_cap_mist,
            require_approval_above_mist,
            tx_context::sender(ctx),
            option::some(memory::agent_object_id(parent)),
            option::none(),
            clock,
        );
    }

    /// Parent kill switch for a descendant's budget.
    public entry fun disable_child_agent_budget(
        config: &AiCreditConfig,
        memory_config: &MemoryConfig,
        balance: &mut AiCreditBalance,
        account: &MemoryAccount,
        parent: &SubAgent,
        child: &SubAgent,
        clock: &Clock,
        ctx: &TxContext,
    ) {
        assert_version(config, balance);
        assert_parent_manages_child(memory_config, balance, account, parent, child, clock, ctx);
        disable_agent_budget_internal(
            balance,
            memory::agent_object_id(child),
            tx_context::sender(ctx),
            option::some(memory::agent_object_id(parent)),
            clock,
        );
    }

    /// Parent approves a descendant's over-threshold spend, but only within the parent's own
    /// envelope (its threshold and remaining caps). Beyond that, approval escalates up the
    /// tree — ultimately to the human owner via `approve_agent_spend`.
    public entry fun approve_child_agent_spend(
        config: &AiCreditConfig,
        memory_config: &MemoryConfig,
        balance: &mut AiCreditBalance,
        account: &MemoryAccount,
        parent: &SubAgent,
        child: &SubAgent,
        max_amount_mist: u64,
        expires_at_ms: u64,
        clock: &Clock,
        ctx: &TxContext,
    ) {
        assert_version(config, balance);
        assert_active(balance);
        assert_parent_manages_child(memory_config, balance, account, parent, child, clock, ctx);
        assert_within_parent_envelope(
            balance,
            memory::agent_object_id(parent),
            max_amount_mist,
            clock,
        );
        store_spend_approval(
            balance,
            memory::agent_object_id(child),
            max_amount_mist,
            expires_at_ms,
            tx_context::sender(ctx),
            option::some(memory::agent_object_id(parent)),
            option::none(),
            clock,
        );
    }

    public entry fun pause_balance(
        config: &AiCreditConfig,
        balance: &mut AiCreditBalance,
        ctx: &TxContext,
    ) {
        assert_version(config, balance);
        assert_owner(balance, ctx);
        balance.active = false;
        event::emit(AiCreditBalancePaused {
            balance_id: object::id(balance),
        });
    }

    public entry fun reactivate_balance(
        config: &AiCreditConfig,
        balance: &mut AiCreditBalance,
        ctx: &TxContext,
    ) {
        assert_version(config, balance);
        assert_owner(balance, ctx);
        balance.active = true;
        event::emit(AiCreditBalanceReactivated {
            balance_id: object::id(balance),
        });
    }

    // ============================================================
    // Reserve-and-capture spend
    // ============================================================

    /// Reserve an oracle-authorized, deterministic maximum charge before any provider work.
    /// The signed intent binds the balance, agent, pricing quote, request/envelope hashes,
    /// amount, nonce, and both deadlines.
    public fun reserve_signed_spend(
        config: &AiCreditConfig,
        balance: &mut AiCreditBalance,
        account: &MemoryAccount,
        agent: &SubAgent,
        reservation_nonce: u64,
        max_amount_mist: u64,
        provider_envelope_hash: vector<u8>,
        request_hash: vector<u8>,
        fx_quote_id: vector<u8>,
        myso_usd_e8: u64,
        markup_bps: u64,
        timestamp_ms: u64,
        capture_deadline_ms: u64,
        hard_expiry_ms: u64,
        signature: vector<u8>,
        clock: &Clock,
    ) {
        let intent = make_spend_reservation_intent_from_objects(
            balance,
            agent,
            reservation_nonce,
            max_amount_mist,
            provider_envelope_hash,
            request_hash,
            fx_quote_id,
            myso_usd_e8,
            markup_bps,
            timestamp_ms,
            capture_deadline_ms,
            hard_expiry_ms,
        );
        verify_reservation_signature(config, &intent, &signature);
        execute_reservation(config, balance, account, agent, intent, clock);
    }

    public fun make_spend_reservation_intent_from_objects(
        balance: &AiCreditBalance,
        agent: &SubAgent,
        reservation_nonce: u64,
        max_amount_mist: u64,
        provider_envelope_hash: vector<u8>,
        request_hash: vector<u8>,
        fx_quote_id: vector<u8>,
        myso_usd_e8: u64,
        markup_bps: u64,
        timestamp_ms: u64,
        capture_deadline_ms: u64,
        hard_expiry_ms: u64,
    ): SpendReservationIntent {
        SpendReservationIntent {
            balance_id: object::id(balance),
            agent_object_id: memory::agent_object_id(agent),
            reservation_nonce,
            max_amount_mist,
            provider_envelope_hash,
            request_hash,
            fx_quote_id,
            myso_usd_e8,
            markup_bps,
            timestamp_ms,
            capture_deadline_ms,
            hard_expiry_ms,
        }
    }

    /// Capture the actual MIST charge. The oracle signature binds provider cost and the
    /// provider generation hash to the reservation. Any unused maximum is released.
    public fun capture_reserved_spend(
        config: &AiCreditConfig,
        balance: &mut AiCreditBalance,
        reservation_nonce: u64,
        amount_mist: u64,
        provider_cost_usd_micros: u64,
        provider_generation_hash: vector<u8>,
        timestamp_ms: u64,
        signature: vector<u8>,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(table::contains(&balance.reservations, reservation_nonce), EReservationNotFound);
        let reservation = *table::borrow(&balance.reservations, reservation_nonce);
        let intent = CaptureSpendIntent {
            balance_id: object::id(balance),
            agent_object_id: reservation.agent_object_id,
            reservation_nonce,
            amount_mist,
            provider_cost_usd_micros,
            provider_generation_hash,
            timestamp_ms,
        };
        verify_capture_signature(config, &intent, &signature);
        execute_capture(config, balance, intent, clock, ctx);
    }

    /// Release a reservation only when the oracle confirms that no billable generation
    /// exists. Signed cancellation closes at `capture_deadline_ms`; after hard expiry anyone
    /// may call `expire_reservation` instead.
    public fun cancel_reserved_spend(
        config: &AiCreditConfig,
        balance: &mut AiCreditBalance,
        reservation_nonce: u64,
        timestamp_ms: u64,
        signature: vector<u8>,
        clock: &Clock,
    ) {
        assert!(table::contains(&balance.reservations, reservation_nonce), EReservationNotFound);
        let reservation = *table::borrow(&balance.reservations, reservation_nonce);
        let intent = CancelSpendIntent {
            balance_id: object::id(balance),
            agent_object_id: reservation.agent_object_id,
            reservation_nonce,
            timestamp_ms,
        };
        verify_cancel_signature(config, &intent, &signature);
        execute_cancellation(config, balance, intent, clock);
    }

    /// Permissionless safety valve so gateway failure cannot lock funds indefinitely.
    public fun expire_reservation(
        config: &AiCreditConfig,
        balance: &mut AiCreditBalance,
        reservation_nonce: u64,
        clock: &Clock,
    ) {
        assert_version(config, balance);
        assert!(table::contains(&balance.reservations, reservation_nonce), EReservationNotFound);
        let reservation = *table::borrow(&balance.reservations, reservation_nonce);
        let now = clock::timestamp_ms(clock);
        assert!(now >= reservation.hard_expiry_ms, EReservationNotExpired);
        let reservation = table::remove(&mut balance.reservations, reservation_nonce);
        release_reservation_counters(balance, &reservation, now);
        event::emit(AiSpendExpired {
            balance_id: object::id(balance),
            agent_object_id: reservation.agent_object_id,
            reservation_nonce,
            released_mist: reservation.max_amount_mist,
            expired_at_ms: now,
            available_mist: available_mist(balance),
        });
    }

    // ============================================================
    // Settlement
    // ============================================================

    public fun settle_usage(
        config: &AiCreditConfig,
        balance: &mut AiCreditBalance,
        account: &MemoryAccount,
        agent: &SubAgent,
        receipt: UsageReceipt,
        signature: vector<u8>,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        settle_usage_with_signature(config, balance, account, agent, receipt, signature, clock, ctx);
    }

    public fun settle_usage_batch(
        config: &AiCreditConfig,
        balance: &mut AiCreditBalance,
        account: &MemoryAccount,
        agent: &SubAgent,
        receipts: vector<UsageReceipt>,
        signatures: vector<vector<u8>>,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        let n = vector::length(&receipts);
        assert!(n == vector::length(&signatures), EInvalidAmount);
        let mut i = 0;
        while (i < n) {
            let receipt = *vector::borrow(&receipts, i);
            let sig = *vector::borrow(&signatures, i);
            settle_usage_with_signature(config, balance, account, agent, receipt, sig, clock, ctx);
            i = i + 1;
        };
    }

    /// PTB/oracle entry point: receipt fields are primitives; balance and agent IDs are derived
    /// from shared objects (not caller-supplied). Authorization is the oracle Ed25519 signature,
    /// not `AiCreditOracleAdminCap`.
    public fun settle_signed_usage(
        config: &AiCreditConfig,
        balance: &mut AiCreditBalance,
        account: &MemoryAccount,
        agent: &SubAgent,
        receipt_id: u128,
        amount_mist: u64,
        usage_kind: u8,
        timestamp_ms: u64,
        settlement_nonce: u64,
        signature: vector<u8>,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        let receipt = make_usage_receipt_from_objects(
            balance,
            agent,
            receipt_id,
            amount_mist,
            usage_kind,
            timestamp_ms,
            settlement_nonce,
        );
        settle_usage_with_signature(config, balance, account, agent, receipt, signature, clock, ctx);
    }

    public fun make_usage_receipt_from_objects(
        balance: &AiCreditBalance,
        agent: &SubAgent,
        receipt_id: u128,
        amount_mist: u64,
        usage_kind: u8,
        timestamp_ms: u64,
        settlement_nonce: u64,
    ): UsageReceipt {
        UsageReceipt {
            balance_id: object::id(balance),
            agent_object_id: memory::agent_object_id(agent),
            receipt_id,
            amount_mist,
            usage_kind,
            timestamp_ms,
            settlement_nonce,
        }
    }

    fun settle_usage_with_signature(
        config: &AiCreditConfig,
        balance: &mut AiCreditBalance,
        account: &MemoryAccount,
        agent: &SubAgent,
        receipt: UsageReceipt,
        signature: vector<u8>,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        verify_receipt_signature(config, &receipt, &signature);
        execute_settlement(config, balance, account, agent, receipt, clock, ctx);
    }

    fun execute_settlement(
        config: &AiCreditConfig,
        balance: &mut AiCreditBalance,
        account: &MemoryAccount,
        agent: &SubAgent,
        receipt: UsageReceipt,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert_version(config, balance);
        assert_active(balance);
        assert_agent_linked(balance, agent);
        assert!(object::id(account) == balance.memory_account_id, EAccountMismatch);
        assert!(memory::agent_object_id(agent) == receipt.agent_object_id, EAccountMismatch);
        assert!(object::id(balance) == receipt.balance_id, EAccountMismatch);
        memory::assert_sub_agent_active(agent, clock);
        assert!(
            memory::has_cap(memory::sub_agent_capabilities(agent), memory::cap_ai_spend()),
            EAgentMissingCap,
        );

        assert_receipt_fresh(config, &receipt, clock);
        assert!(receipt.amount_mist > 0, EInvalidAmount);
        assert!(receipt.amount_mist <= config.max_single_settlement_mist, ECapExceeded);
        assert!(receipt.settlement_nonce == balance.settlement_nonce + 1, EInvalidNonce);

        roll_account_windows(balance, clock::timestamp_ms(clock));
        assert_account_caps(balance, receipt.amount_mist);

        let agent_id = memory::agent_object_id(agent);
        if (table::contains(&balance.agent_budgets, agent_id)) {
            let entry = table::borrow_mut(&mut balance.agent_budgets, agent_id);
            assert!(entry.enabled, EAgentDisabled);
            roll_agent_windows(entry, clock::timestamp_ms(clock));
            assert_agent_caps(entry, receipt.amount_mist);
            entry.spent_mist = entry.spent_mist + receipt.amount_mist;
            entry.spent_day_mist = entry.spent_day_mist + receipt.amount_mist;
            entry.spent_month_mist = entry.spent_month_mist + receipt.amount_mist;
        };

        // Over-threshold settlements must consume a live spend allowance (the previously
        // unenforced `require_approval_above_mist` gate).
        maybe_consume_spend_approval(balance, agent_id, receipt.amount_mist, clock);

        assert!(receipt.amount_mist <= available_mist(balance), EInsufficientBalance);

        balance.settlement_nonce = receipt.settlement_nonce;
        balance.spent_total_mist = balance.spent_total_mist + receipt.amount_mist;
        balance.spent_day_mist = balance.spent_day_mist + receipt.amount_mist;
        balance.spent_month_mist = balance.spent_month_mist + receipt.amount_mist;

        let payout = balance::split(&mut balance.balance, receipt.amount_mist);
        transfer::public_transfer(coin::from_balance(payout, ctx), config.treasury);

        let remaining = balance_mist(balance);
        event::emit(AiCreditUsageSettled {
            balance_id: object::id(balance),
            agent_object_id: agent_id,
            receipt_id: receipt.receipt_id,
            amount_mist: receipt.amount_mist,
            usage_kind: receipt.usage_kind,
            settlement_nonce: receipt.settlement_nonce,
            remaining_mist: remaining,
        });
        if (remaining == 0) {
            event::emit(AiCreditBalanceDepleted {
                balance_id: object::id(balance),
            });
        };
    }

    fun execute_reservation(
        config: &AiCreditConfig,
        balance: &mut AiCreditBalance,
        account: &MemoryAccount,
        agent: &SubAgent,
        intent: SpendReservationIntent,
        clock: &Clock,
    ) {
        assert_version(config, balance);
        assert_active(balance);
        assert_agent_linked(balance, agent);
        assert!(object::id(account) == balance.memory_account_id, EAccountMismatch);
        assert!(object::id(balance) == intent.balance_id, EReservationMismatch);
        let agent_id = memory::agent_object_id(agent);
        assert!(agent_id == intent.agent_object_id, EReservationMismatch);
        memory::assert_sub_agent_active(agent, clock);
        assert!(
            memory::has_cap(memory::sub_agent_capabilities(agent), memory::cap_ai_spend()),
            EAgentMissingCap,
        );

        assert!(intent.max_amount_mist > 0, EInvalidAmount);
        assert!(intent.max_amount_mist <= config.max_single_settlement_mist, ECapExceeded);
        assert!(intent.reservation_nonce == balance.reservation_nonce + 1, EInvalidNonce);
        assert!(!table::contains(&balance.reservations, intent.reservation_nonce), EInvalidNonce);
        assert!(vector::length(&intent.provider_envelope_hash) == HASH_LEN, EInvalidHash);
        assert!(vector::length(&intent.request_hash) == HASH_LEN, EInvalidHash);
        assert!(vector::length(&intent.fx_quote_id) > 0, EInvalidHash);
        assert!(intent.myso_usd_e8 > 0, EInvalidAmount);
        assert!(intent.markup_bps == config.oracle_markup_bps, EMarkupMismatch);

        let now = clock::timestamp_ms(clock);
        assert_signed_timestamp_fresh(config, intent.timestamp_ms, now);
        assert!(intent.capture_deadline_ms > now, EInvalidReservationWindow);
        assert!(intent.hard_expiry_ms > intent.capture_deadline_ms, EInvalidReservationWindow);
        assert!(
            intent.capture_deadline_ms - now <= MAX_CAPTURE_WINDOW_MS,
            EInvalidReservationWindow,
        );
        assert!(
            intent.hard_expiry_ms - now <= MAX_RESERVATION_LIFETIME_MS,
            EInvalidReservationWindow,
        );

        roll_account_windows(balance, now);
        assert_account_caps(balance, intent.max_amount_mist);
        assert!(intent.max_amount_mist <= available_mist(balance), EInsufficientBalance);

        let agent_budget_reserved = table::contains(&balance.agent_budgets, agent_id);
        let (agent_day_anchor_ms, agent_month_anchor_ms) = if (agent_budget_reserved) {
            let entry = table::borrow_mut(&mut balance.agent_budgets, agent_id);
            assert!(entry.enabled, EAgentDisabled);
            roll_agent_windows(entry, now);
            assert_agent_caps(entry, intent.max_amount_mist);
            let day_anchor = entry.day_anchor_ms;
            let month_anchor = entry.month_anchor_ms;
            entry.reserved_mist = entry.reserved_mist + intent.max_amount_mist;
            entry.reserved_day_mist = entry.reserved_day_mist + intent.max_amount_mist;
            entry.reserved_month_mist = entry.reserved_month_mist + intent.max_amount_mist;
            (day_anchor, month_anchor)
        } else {
            (0, 0)
        };

        // Approval is consumed when funds become unavailable, not after provider spend.
        maybe_consume_spend_approval(balance, agent_id, intent.max_amount_mist, clock);

        balance.reservation_nonce = intent.reservation_nonce;
        balance.reserved_mist = balance.reserved_mist + intent.max_amount_mist;
        balance.reserved_day_mist = balance.reserved_day_mist + intent.max_amount_mist;
        balance.reserved_month_mist = balance.reserved_month_mist + intent.max_amount_mist;

        let reservation = SpendReservation {
            reservation_nonce: intent.reservation_nonce,
            agent_object_id: agent_id,
            max_amount_mist: intent.max_amount_mist,
            provider_envelope_hash: intent.provider_envelope_hash,
            request_hash: intent.request_hash,
            fx_quote_id: intent.fx_quote_id,
            myso_usd_e8: intent.myso_usd_e8,
            markup_bps: intent.markup_bps,
            created_at_ms: now,
            capture_deadline_ms: intent.capture_deadline_ms,
            hard_expiry_ms: intent.hard_expiry_ms,
            account_day_anchor_ms: balance.day_anchor_ms,
            account_month_anchor_ms: balance.month_anchor_ms,
            agent_budget_reserved,
            agent_day_anchor_ms,
            agent_month_anchor_ms,
        };
        table::add(&mut balance.reservations, intent.reservation_nonce, reservation);

        event::emit(AiSpendReserved {
            balance_id: object::id(balance),
            agent_object_id: agent_id,
            reservation_nonce: intent.reservation_nonce,
            max_amount_mist: intent.max_amount_mist,
            provider_envelope_hash: intent.provider_envelope_hash,
            request_hash: intent.request_hash,
            fx_quote_id: intent.fx_quote_id,
            myso_usd_e8: intent.myso_usd_e8,
            markup_bps: intent.markup_bps,
            capture_deadline_ms: intent.capture_deadline_ms,
            hard_expiry_ms: intent.hard_expiry_ms,
            available_mist: available_mist(balance),
        });
    }

    fun execute_capture(
        config: &AiCreditConfig,
        balance: &mut AiCreditBalance,
        intent: CaptureSpendIntent,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert_version(config, balance);
        assert!(table::contains(&balance.reservations, intent.reservation_nonce), EReservationNotFound);
        assert!(intent.balance_id == object::id(balance), EReservationMismatch);
        assert!(intent.amount_mist > 0, EInvalidAmount);
        assert!(intent.provider_cost_usd_micros > 0, EInvalidAmount);
        assert!(vector::length(&intent.provider_generation_hash) == HASH_LEN, EInvalidHash);
        let now = clock::timestamp_ms(clock);
        assert_signed_timestamp_fresh(config, intent.timestamp_ms, now);

        let reservation = *table::borrow(&balance.reservations, intent.reservation_nonce);
        assert!(intent.agent_object_id == reservation.agent_object_id, EReservationMismatch);
        assert!(intent.amount_mist <= reservation.max_amount_mist, ECapExceeded);
        assert!(now < reservation.hard_expiry_ms, EReservationExpired);

        let reservation = table::remove(&mut balance.reservations, intent.reservation_nonce);
        capture_reservation_counters(balance, &reservation, intent.amount_mist, now);
        let payout = balance::split(&mut balance.balance, intent.amount_mist);
        transfer::public_transfer(coin::from_balance(payout, ctx), config.treasury);

        let remaining = balance_mist(balance);
        event::emit(AiSpendCaptured {
            balance_id: object::id(balance),
            agent_object_id: reservation.agent_object_id,
            reservation_nonce: reservation.reservation_nonce,
            reserved_mist: reservation.max_amount_mist,
            captured_mist: intent.amount_mist,
            released_mist: reservation.max_amount_mist - intent.amount_mist,
            provider_cost_usd_micros: intent.provider_cost_usd_micros,
            provider_generation_hash: intent.provider_generation_hash,
            fx_quote_id: reservation.fx_quote_id,
            myso_usd_e8: reservation.myso_usd_e8,
            markup_bps: reservation.markup_bps,
            captured_at_ms: now,
            remaining_mist: remaining,
            available_mist: available_mist(balance),
        });
        if (remaining == 0) {
            event::emit(AiCreditBalanceDepleted {
                balance_id: object::id(balance),
            });
        };
    }

    fun execute_cancellation(
        config: &AiCreditConfig,
        balance: &mut AiCreditBalance,
        intent: CancelSpendIntent,
        clock: &Clock,
    ) {
        assert_version(config, balance);
        assert!(table::contains(&balance.reservations, intent.reservation_nonce), EReservationNotFound);
        assert!(intent.balance_id == object::id(balance), EReservationMismatch);
        let now = clock::timestamp_ms(clock);
        assert_signed_timestamp_fresh(config, intent.timestamp_ms, now);
        let reservation = *table::borrow(&balance.reservations, intent.reservation_nonce);
        assert!(intent.agent_object_id == reservation.agent_object_id, EReservationMismatch);
        assert!(now <= reservation.capture_deadline_ms, ECaptureWindowClosed);
        let reservation = table::remove(&mut balance.reservations, intent.reservation_nonce);
        release_reservation_counters(balance, &reservation, now);
        event::emit(AiSpendCancelled {
            balance_id: object::id(balance),
            agent_object_id: reservation.agent_object_id,
            reservation_nonce: reservation.reservation_nonce,
            released_mist: reservation.max_amount_mist,
            cancelled_at_ms: now,
            available_mist: available_mist(balance),
        });
    }

    // ============================================================
    // Oracle admin
    // ============================================================

    public entry fun update_oracle_pubkey(
        cap: &AiCreditOracleAdminCap,
        config: &mut AiCreditConfig,
        new_pk: vector<u8>,
        ctx: &TxContext,
    ) {
        assert_oracle_admin(cap, ctx);
        assert!(config.version == upgrade::current_version(), EWrongVersion);
        assert!(vector::length(&new_pk) == ED25519_PK_LEN, EInvalidPubkey);
        config.oracle_pubkey = new_pk;
        event::emit(AiCreditOraclePubkeyUpdated {
            updated_by: tx_context::sender(ctx),
            new_pubkey: new_pk,
        });
    }

    public entry fun update_oracle_markup(
        cap: &AiCreditOracleAdminCap,
        config: &mut AiCreditConfig,
        oracle_markup_bps: u64,
        ctx: &TxContext,
    ) {
        assert_oracle_admin(cap, ctx);
        assert!(oracle_markup_bps <= 10000, EInvalidAmount);
        config.oracle_markup_bps = oracle_markup_bps;
        event::emit(AiCreditMarkupUpdated {
            updated_by: tx_context::sender(ctx),
            oracle_markup_bps,
        });
    }

    public entry fun update_min_deposit(
        cap: &AiCreditOracleAdminCap,
        config: &mut AiCreditConfig,
        min_deposit_mist: u64,
        ctx: &TxContext,
    ) {
        assert_oracle_admin(cap, ctx);
        assert!(min_deposit_mist > 0, EInvalidAmount);
        config.min_deposit_mist = min_deposit_mist;
        event::emit(AiCreditMinDepositUpdated {
            updated_by: tx_context::sender(ctx),
            min_deposit_mist,
        });
    }

    public entry fun update_settlement_limits(
        cap: &AiCreditOracleAdminCap,
        config: &mut AiCreditConfig,
        max_single_settlement_mist: u64,
        receipt_ttl_ms: u64,
        ctx: &TxContext,
    ) {
        assert_oracle_admin(cap, ctx);
        assert!(max_single_settlement_mist > 0, EInvalidAmount);
        assert!(receipt_ttl_ms > 0, EInvalidAmount);
        config.max_single_settlement_mist = max_single_settlement_mist;
        config.receipt_ttl_ms = receipt_ttl_ms;
        event::emit(AiCreditSettlementLimitsUpdated {
            max_single_settlement_mist,
            receipt_ttl_ms,
        });
    }

  public entry fun update_treasury(
        cap: &AiCreditOracleAdminCap,
        config: &mut AiCreditConfig,
        treasury: address,
        ctx: &TxContext,
    ) {
        assert_oracle_admin(cap, ctx);
        config.treasury = treasury;
    }

    // ============================================================
    // Views
    // ============================================================

    public fun balance_mist(balance: &AiCreditBalance): u64 {
        balance::value(&balance.balance)
    }

    public fun available_mist(balance: &AiCreditBalance): u64 {
        balance_mist(balance) - balance.reserved_mist
    }

    public fun reserved_mist(balance: &AiCreditBalance): u64 {
        balance.reserved_mist
    }

    public fun latest_reservation_nonce(balance: &AiCreditBalance): u64 {
        balance.reservation_nonce
    }

    public fun reservation_for(
        balance: &AiCreditBalance,
        reservation_nonce: u64,
    ): Option<SpendReservation> {
        if (table::contains(&balance.reservations, reservation_nonce)) {
            option::some(*table::borrow(&balance.reservations, reservation_nonce))
        } else {
            option::none()
        }
    }

    public fun reservation_max_amount_mist(reservation: &SpendReservation): u64 {
        reservation.max_amount_mist
    }

    public fun reservation_agent_object_id(reservation: &SpendReservation): ID {
        reservation.agent_object_id
    }

    public fun reservation_hard_expiry_ms(reservation: &SpendReservation): u64 {
        reservation.hard_expiry_ms
    }

    public fun oracle_markup_bps(config: &AiCreditConfig): u64 {
        config.oracle_markup_bps
    }

    public fun min_deposit_mist(config: &AiCreditConfig): u64 {
        config.min_deposit_mist
    }

    /// Live allowance for an agent, if any (may be expired — check `approval_expires_at`).
    public fun spend_approval_for(
        balance: &AiCreditBalance,
        agent_object_id: ID,
    ): Option<SpendApproval> {
        let key = SpendApprovalKey { agent_object_id };
        if (df::exists_with_type<SpendApprovalKey, SpendApproval>(&balance.id, key)) {
            option::some(*df::borrow<SpendApprovalKey, SpendApproval>(&balance.id, key))
        } else {
            option::none()
        }
    }

    public fun approval_max_amount_mist(approval: &SpendApproval): u64 {
        approval.max_amount_mist
    }

    public fun approval_expires_at_ms(approval: &SpendApproval): u64 {
        approval.expires_at_ms
    }

    public fun approval_approved_by(approval: &SpendApproval): address {
        approval.approved_by
    }

    public fun approval_nonce(approval: &SpendApproval): u64 {
        approval.approval_nonce
    }

    /// Approval threshold on the agent's budget entry, if configured.
    public fun agent_approval_threshold(
        balance: &AiCreditBalance,
        agent_object_id: ID,
    ): Option<u64> {
        if (!table::contains(&balance.agent_budgets, agent_object_id)) {
            return option::none()
        };
        let entry = table::borrow(&balance.agent_budgets, agent_object_id);
        entry.require_approval_above_mist
    }

    public fun agent_remaining_mist(balance: &AiCreditBalance, agent_object_id: ID): Option<u64> {
        if (!table::contains(&balance.agent_budgets, agent_object_id)) {
            return option::none()
        };
        let entry = table::borrow(&balance.agent_budgets, agent_object_id);
        if (option::is_none(&entry.budget_mist)) {
            return option::none()
        };
        let max = *option::borrow(&entry.budget_mist);
        let committed = entry.spent_mist + entry.reserved_mist;
        if (committed >= max) {
            option::some(0)
        } else {
            option::some(max - committed)
        }
    }

    public fun usage_inference(): u8 { USAGE_INFERENCE }
    public fun usage_tool(): u8 { USAGE_TOOL }
    public fun usage_embed(): u8 { USAGE_EMBED }
    public fun usage_storage(): u8 { USAGE_STORAGE }
    public fun usage_workflow(): u8 { USAGE_WORKFLOW }
    public fun mist_per_myso(): u64 { MIST_PER_MYSO }

    public fun assert_agent_may_spend(
        balance: &mut AiCreditBalance,
        account: &MemoryAccount,
        agent: &SubAgent,
        amount_mist: u64,
        clock: &Clock,
    ) {
        assert_active(balance);
        assert_agent_linked(balance, agent);
        assert!(object::id(account) == balance.memory_account_id, EAccountMismatch);
        memory::assert_sub_agent_active(agent, clock);
        assert!(
            memory::has_cap(memory::sub_agent_capabilities(agent), memory::cap_ai_spend()),
            EAgentMissingCap,
        );
        roll_account_windows(balance, clock::timestamp_ms(clock));
        assert_account_caps(balance, amount_mist);
        let agent_id = memory::agent_object_id(agent);
        if (table::contains(&balance.agent_budgets, agent_id)) {
            let entry = table::borrow_mut(&mut balance.agent_budgets, agent_id);
            assert!(entry.enabled, EAgentDisabled);
            roll_agent_windows(entry, clock::timestamp_ms(clock));
            assert_agent_caps(entry, amount_mist);
        };
        assert!(amount_mist <= available_mist(balance), EInsufficientBalance);
    }

    // ============================================================
    // Internal helpers
    // ============================================================

    fun assert_version(config: &AiCreditConfig, balance: &AiCreditBalance) {
        assert!(config.version == upgrade::current_version(), EWrongVersion);
        assert!(balance.version == upgrade::current_version(), EWrongVersion);
    }

    /// Migrate AiCreditConfig to the current package version.
    public entry fun migrate_config(
        config: &mut AiCreditConfig,
        _: &UpgradeAdminCap,
        ctx: &mut TxContext,
    ) {
        let current_version = upgrade::current_version();
        assert!(config.version < current_version, EWrongVersion);
        let old_version = config.version;
        config.version = current_version;
        upgrade::emit_migration_event(
            object::id(config),
            string::utf8(b"AiCreditConfig"),
            old_version,
            tx_context::sender(ctx),
        );
    }

    /// Migrate a single AiCreditBalance to the current package version.
    public entry fun migrate_balance(
        balance: &mut AiCreditBalance,
        _: &UpgradeAdminCap,
        ctx: &mut TxContext,
    ) {
        let current_version = upgrade::current_version();
        assert!(balance.version < current_version, EWrongVersion);
        let old_version = balance.version;
        balance.version = current_version;
        upgrade::emit_migration_event(
            object::id(balance),
            string::utf8(b"AiCreditBalance"),
            old_version,
            tx_context::sender(ctx),
        );
    }

    fun assert_owner(balance: &AiCreditBalance, ctx: &TxContext) {
        assert!(tx_context::sender(ctx) == balance.principal_owner, ENotOwner);
    }

    fun assert_active(balance: &AiCreditBalance) {
        assert!(balance.active, EInactive);
    }

    fun assert_agent_linked(balance: &AiCreditBalance, agent: &SubAgent) {
        assert!(
            memory::sub_agent_memory_account_id(agent) == balance.memory_account_id,
            EAccountMismatch,
        );
    }

    /// Org role gates require: account matches the balance, org belongs to the account,
    /// and the target agent belongs to that org.
    fun assert_org_gate_for_agent(
        balance: &AiCreditBalance,
        account: &MemoryAccount,
        org: &AgenticOrganization,
        agent: &SubAgent,
    ) {
        assert!(object::id(account) == balance.memory_account_id, EAccountMismatch);
        assert!(
            memory::organization_memory_account_id(org) == object::id(account),
            EAccountMismatch,
        );
        assert!(
            memory::sub_agent_organization_id(agent) == memory::organization_id(org),
            EAgentNotInOrg,
        );
    }

    /// Common authorization for parent-delegated budget operations: sender is the parent's
    /// derived address, parent is active with `CAP_BUDGET_MANAGE`, both agents are linked to
    /// this balance, and the child sits strictly below the parent in the agent tree.
    fun assert_parent_manages_child(
        memory_config: &MemoryConfig,
        balance: &AiCreditBalance,
        account: &MemoryAccount,
        parent: &SubAgent,
        child: &SubAgent,
        clock: &Clock,
        ctx: &TxContext,
    ) {
        assert!(object::id(account) == balance.memory_account_id, EAccountMismatch);
        assert_agent_linked(balance, parent);
        assert_agent_linked(balance, child);
        assert!(
            tx_context::sender(ctx) == memory::sub_agent_derived_address(parent),
            ENotParentSigner,
        );
        memory::assert_sub_agent_active(parent, clock);
        assert!(
            memory::has_cap(memory::sub_agent_capabilities(parent), memory::cap_budget_manage()),
            EAgentMissingCap,
        );
        let parent_id = memory::agent_object_id(parent);
        let child_id = memory::agent_object_id(child);
        assert!(parent_id != child_id, ECannotManageSelf);
        assert!(memory::is_descendant_agent(memory_config, account, parent_id, child_id), ENotDescendant);
    }

    /// Child budget limits must be at least as strict as the parent's own entry (when the
    /// parent has one; an unconstrained parent may set anything).
    fun assert_child_budget_within_parent_envelope(
        balance: &AiCreditBalance,
        parent_id: ID,
        budget_mist: &Option<u64>,
        daily_cap_mist: &Option<u64>,
        monthly_cap_mist: &Option<u64>,
        require_approval_above_mist: &Option<u64>,
    ) {
        if (!table::contains(&balance.agent_budgets, parent_id)) {
            return
        };
        let parent_entry = table::borrow(&balance.agent_budgets, parent_id);
        assert_limit_not_looser(budget_mist, &parent_entry.budget_mist);
        assert_limit_not_looser(daily_cap_mist, &parent_entry.daily_cap_mist);
        assert_limit_not_looser(monthly_cap_mist, &parent_entry.monthly_cap_mist);
        assert_limit_not_looser(
            require_approval_above_mist,
            &parent_entry.require_approval_above_mist,
        );
    }

    /// `child` is not looser than `parent`: when the parent limit is set, the child limit
    /// must be set and must not exceed it.
    fun assert_limit_not_looser(child: &Option<u64>, parent: &Option<u64>) {
        if (option::is_none(parent)) {
            return
        };
        assert!(option::is_some(child), EParentEnvelopeExceeded);
        assert!(*option::borrow(child) <= *option::borrow(parent), EParentEnvelopeExceeded);
    }

    /// Parents may only approve amounts they could spend themselves: within their own
    /// approval threshold (if set) and remaining budget/day/month caps (if set).
    fun assert_within_parent_envelope(
        balance: &mut AiCreditBalance,
        parent_id: ID,
        amount_mist: u64,
        clock: &Clock,
    ) {
        if (!table::contains(&balance.agent_budgets, parent_id)) {
            return
        };
        let entry = table::borrow_mut(&mut balance.agent_budgets, parent_id);
        assert!(entry.enabled, EAgentDisabled);
        roll_agent_windows(entry, clock::timestamp_ms(clock));
        if (option::is_some(&entry.require_approval_above_mist)) {
            assert!(
                amount_mist <= *option::borrow(&entry.require_approval_above_mist),
                EParentEnvelopeExceeded,
            );
        };
        if (option::is_some(&entry.budget_mist)) {
            let max = *option::borrow(&entry.budget_mist);
            assert!(
                entry.spent_mist + entry.reserved_mist + amount_mist <= max,
                EParentEnvelopeExceeded,
            );
        };
        if (option::is_some(&entry.daily_cap_mist)) {
            let cap = *option::borrow(&entry.daily_cap_mist);
            assert!(
                entry.spent_day_mist + entry.reserved_day_mist + amount_mist <= cap,
                EParentEnvelopeExceeded,
            );
        };
        if (option::is_some(&entry.monthly_cap_mist)) {
            let cap = *option::borrow(&entry.monthly_cap_mist);
            assert!(
                entry.spent_month_mist + entry.reserved_month_mist + amount_mist <= cap,
                EParentEnvelopeExceeded,
            );
        };
    }

    /// Shared budget upsert used by owner, org-manager, and parent paths. Emits the legacy
    /// `AiCreditAgentBudgetUpdated` plus the audit-grade `AiCreditAgentBudgetChanged`.
    fun upsert_agent_budget(
        balance: &mut AiCreditBalance,
        agent: &SubAgent,
        budget_mist: Option<u64>,
        daily_cap_mist: Option<u64>,
        monthly_cap_mist: Option<u64>,
        require_approval_above_mist: Option<u64>,
        set_by: address,
        set_by_agent_id: Option<ID>,
        organization_id: Option<ID>,
        clock: &Clock,
    ) {
        let agent_id = memory::agent_object_id(agent);
        let now = clock::timestamp_ms(clock);

        let had_previous_entry = table::contains(&balance.agent_budgets, agent_id);
        let (prev_budget, prev_daily, prev_monthly, prev_approval, prev_enabled) =
            if (had_previous_entry) {
                let prev = table::borrow(&balance.agent_budgets, agent_id);
                (
                    prev.budget_mist,
                    prev.daily_cap_mist,
                    prev.monthly_cap_mist,
                    prev.require_approval_above_mist,
                    prev.enabled,
                )
            } else {
                (option::none(), option::none(), option::none(), option::none(), false)
            };

        let entry = if (had_previous_entry) {
            let e = table::borrow_mut(&mut balance.agent_budgets, agent_id);
            e.budget_mist = budget_mist;
            e.daily_cap_mist = daily_cap_mist;
            e.monthly_cap_mist = monthly_cap_mist;
            e.require_approval_above_mist = require_approval_above_mist;
            e.enabled = true;
            *e
        } else {
            let e = AgentBudgetEntry {
                agent_object_id: agent_id,
                derived_address: memory::sub_agent_derived_address(agent),
                enabled: true,
                budget_mist,
                spent_mist: 0,
                reserved_mist: 0,
                daily_cap_mist,
                monthly_cap_mist,
                spent_day_mist: 0,
                reserved_day_mist: 0,
                spent_month_mist: 0,
                reserved_month_mist: 0,
                day_anchor_ms: now,
                month_anchor_ms: now,
                require_approval_above_mist,
            };
            table::add(&mut balance.agent_budgets, agent_id, e);
            e
        };

        event::emit(AiCreditAgentBudgetUpdated {
            balance_id: object::id(balance),
            agent_object_id: entry.agent_object_id,
            budget_mist: entry.budget_mist,
            daily_cap_mist: entry.daily_cap_mist,
            monthly_cap_mist: entry.monthly_cap_mist,
            require_approval_above_mist: entry.require_approval_above_mist,
        });
        event::emit(AiCreditAgentBudgetChanged {
            balance_id: object::id(balance),
            agent_object_id: agent_id,
            had_previous_entry,
            prev_budget_mist: prev_budget,
            prev_daily_cap_mist: prev_daily,
            prev_monthly_cap_mist: prev_monthly,
            prev_require_approval_above_mist: prev_approval,
            prev_enabled,
            budget_mist,
            daily_cap_mist,
            monthly_cap_mist,
            require_approval_above_mist,
            enabled: true,
            set_by,
            set_by_agent_id,
            organization_id,
            timestamp_ms: now,
        });
    }

    fun disable_agent_budget_internal(
        balance: &mut AiCreditBalance,
        agent_object_id: ID,
        set_by: address,
        set_by_agent_id: Option<ID>,
        clock: &Clock,
    ) {
        assert!(table::contains(&balance.agent_budgets, agent_object_id), EAgentNotFound);
        let entry = table::borrow_mut(&mut balance.agent_budgets, agent_object_id);
        let prev_enabled = entry.enabled;
        entry.enabled = false;
        let snapshot = *entry;
        event::emit(AiCreditAgentBudgetDisabled {
            balance_id: object::id(balance),
            agent_object_id,
        });
        event::emit(AiCreditAgentBudgetChanged {
            balance_id: object::id(balance),
            agent_object_id,
            had_previous_entry: true,
            prev_budget_mist: snapshot.budget_mist,
            prev_daily_cap_mist: snapshot.daily_cap_mist,
            prev_monthly_cap_mist: snapshot.monthly_cap_mist,
            prev_require_approval_above_mist: snapshot.require_approval_above_mist,
            prev_enabled,
            budget_mist: snapshot.budget_mist,
            daily_cap_mist: snapshot.daily_cap_mist,
            monthly_cap_mist: snapshot.monthly_cap_mist,
            require_approval_above_mist: snapshot.require_approval_above_mist,
            enabled: false,
            set_by,
            set_by_agent_id,
            organization_id: option::none(),
            timestamp_ms: clock::timestamp_ms(clock),
        });
    }

    /// Store (or overwrite) the agent's one-shot allowance and emit the approval event.
    fun store_spend_approval(
        balance: &mut AiCreditBalance,
        agent_object_id: ID,
        max_amount_mist: u64,
        expires_at_ms: u64,
        approved_by: address,
        approved_by_agent_id: Option<ID>,
        organization_id: Option<ID>,
        clock: &Clock,
    ) {
        assert!(max_amount_mist > 0, EInvalidAmount);
        let now = clock::timestamp_ms(clock);
        assert!(expires_at_ms > now, EInvalidExpiry);

        let approval_nonce = next_approval_nonce(balance);
        let key = SpendApprovalKey { agent_object_id };
        if (df::exists_with_type<SpendApprovalKey, SpendApproval>(&balance.id, key)) {
            let _old: SpendApproval = df::remove(&mut balance.id, key);
        };
        df::add(&mut balance.id, key, SpendApproval {
            max_amount_mist,
            expires_at_ms,
            approved_by,
            approval_nonce,
        });

        event::emit(AiCreditSpendApproved {
            balance_id: object::id(balance),
            agent_object_id,
            approval_nonce,
            max_amount_mist,
            expires_at_ms,
            approved_by,
            approved_by_agent_id,
            organization_id,
            timestamp_ms: now,
        });
    }

    fun next_approval_nonce(balance: &mut AiCreditBalance): u64 {
        if (!df::exists_with_type<ApprovalNonceKey, u64>(&balance.id, ApprovalNonceKey {})) {
            df::add(&mut balance.id, ApprovalNonceKey {}, 0u64);
        };
        let counter = df::borrow_mut<ApprovalNonceKey, u64>(&mut balance.id, ApprovalNonceKey {});
        *counter = *counter + 1;
        *counter
    }

    /// Consume the agent's allowance when the settlement amount exceeds its approval
    /// threshold. Aborts when no live, sufficient allowance exists — this is the on-chain
    /// enforcement of `require_approval_above_mist`.
    fun maybe_consume_spend_approval(
        balance: &mut AiCreditBalance,
        agent_object_id: ID,
        amount_mist: u64,
        clock: &Clock,
    ) {
        if (!table::contains(&balance.agent_budgets, agent_object_id)) {
            return
        };
        let threshold_opt = {
            let entry = table::borrow(&balance.agent_budgets, agent_object_id);
            entry.require_approval_above_mist
        };
        if (option::is_none(&threshold_opt)) {
            return
        };
        if (amount_mist <= *option::borrow(&threshold_opt)) {
            return
        };

        let key = SpendApprovalKey { agent_object_id };
        assert!(
            df::exists_with_type<SpendApprovalKey, SpendApproval>(&balance.id, key),
            EApprovalRequired,
        );
        let approval: SpendApproval = df::remove(&mut balance.id, key);
        assert!(clock::timestamp_ms(clock) <= approval.expires_at_ms, EApprovalExpired);
        assert!(amount_mist <= approval.max_amount_mist, EApprovalInsufficient);

        event::emit(AiCreditSpendApprovalConsumed {
            balance_id: object::id(balance),
            agent_object_id,
            approval_nonce: approval.approval_nonce,
            amount_mist,
            approved_by: approval.approved_by,
            timestamp_ms: clock::timestamp_ms(clock),
        });
    }

    fun assert_oracle_admin(_cap: &AiCreditOracleAdminCap, _ctx: &TxContext) {
        // Holding `AiCreditOracleAdminCap` in the PTB proves admin authority.
    }

    fun verify_receipt_signature(config: &AiCreditConfig, receipt: &UsageReceipt, signature: &vector<u8>) {
        assert!(vector::length(signature) == 64, EInvalidSignature);
        let intent_message = IntentMessage {
            intent: INTENT_AI_CREDIT_USAGE,
            timestamp_ms: receipt.timestamp_ms,
            payload: *receipt,
        };
        let msg = bcs::to_bytes(&intent_message);
        assert!(
            ed25519::ed25519_verify(signature, &config.oracle_pubkey, &msg),
            EInvalidSignature,
        );
    }

    fun verify_reservation_signature(
        config: &AiCreditConfig,
        intent: &SpendReservationIntent,
        signature: &vector<u8>,
    ) {
        assert!(vector::length(signature) == 64, EInvalidSignature);
        let intent_message = IntentMessage {
            intent: INTENT_AI_CREDIT_RESERVE,
            timestamp_ms: intent.timestamp_ms,
            payload: *intent,
        };
        let msg = bcs::to_bytes(&intent_message);
        assert!(
            ed25519::ed25519_verify(signature, &config.oracle_pubkey, &msg),
            EInvalidSignature,
        );
    }

    fun verify_capture_signature(
        config: &AiCreditConfig,
        intent: &CaptureSpendIntent,
        signature: &vector<u8>,
    ) {
        assert!(vector::length(signature) == 64, EInvalidSignature);
        let intent_message = IntentMessage {
            intent: INTENT_AI_CREDIT_CAPTURE,
            timestamp_ms: intent.timestamp_ms,
            payload: *intent,
        };
        let msg = bcs::to_bytes(&intent_message);
        assert!(
            ed25519::ed25519_verify(signature, &config.oracle_pubkey, &msg),
            EInvalidSignature,
        );
    }

    fun verify_cancel_signature(
        config: &AiCreditConfig,
        intent: &CancelSpendIntent,
        signature: &vector<u8>,
    ) {
        assert!(vector::length(signature) == 64, EInvalidSignature);
        let intent_message = IntentMessage {
            intent: INTENT_AI_CREDIT_CANCEL,
            timestamp_ms: intent.timestamp_ms,
            payload: *intent,
        };
        let msg = bcs::to_bytes(&intent_message);
        assert!(
            ed25519::ed25519_verify(signature, &config.oracle_pubkey, &msg),
            EInvalidSignature,
        );
    }

    fun assert_receipt_fresh(config: &AiCreditConfig, receipt: &UsageReceipt, clock: &Clock) {
        let now = clock::timestamp_ms(clock);
        assert_signed_timestamp_fresh(config, receipt.timestamp_ms, now);
    }

    fun assert_signed_timestamp_fresh(config: &AiCreditConfig, timestamp_ms: u64, now: u64) {
        assert!(timestamp_ms <= now, EStaleReceipt);
        assert!(now - timestamp_ms <= config.receipt_ttl_ms, EStaleReceipt);
    }

    fun roll_account_windows(balance: &mut AiCreditBalance, now: u64) {
        if (now >= balance.day_anchor_ms + DAY_MS) {
            balance.spent_day_mist = 0;
            balance.reserved_day_mist = 0;
            balance.day_anchor_ms = now;
        };
        if (now >= balance.month_anchor_ms + MONTH_MS) {
            balance.spent_month_mist = 0;
            balance.reserved_month_mist = 0;
            balance.month_anchor_ms = now;
        };
    }

    fun roll_agent_windows(entry: &mut AgentBudgetEntry, now: u64) {
        if (now >= entry.day_anchor_ms + DAY_MS) {
            entry.spent_day_mist = 0;
            entry.reserved_day_mist = 0;
            entry.day_anchor_ms = now;
        };
        if (now >= entry.month_anchor_ms + MONTH_MS) {
            entry.spent_month_mist = 0;
            entry.reserved_month_mist = 0;
            entry.month_anchor_ms = now;
        };
    }

    fun assert_account_caps(balance: &AiCreditBalance, amount_mist: u64) {
        if (option::is_some(&balance.daily_cap_mist)) {
            let cap = *option::borrow(&balance.daily_cap_mist);
            assert!(
                balance.spent_day_mist + balance.reserved_day_mist + amount_mist <= cap,
                ECapExceeded,
            );
        };
        if (option::is_some(&balance.monthly_cap_mist)) {
            let cap = *option::borrow(&balance.monthly_cap_mist);
            assert!(
                balance.spent_month_mist + balance.reserved_month_mist + amount_mist <= cap,
                ECapExceeded,
            );
        };
    }

    fun assert_agent_caps(entry: &AgentBudgetEntry, amount_mist: u64) {
        if (option::is_some(&entry.budget_mist)) {
            let cap = *option::borrow(&entry.budget_mist);
            assert!(entry.spent_mist + entry.reserved_mist + amount_mist <= cap, ECapExceeded);
        };
        if (option::is_some(&entry.daily_cap_mist)) {
            let cap = *option::borrow(&entry.daily_cap_mist);
            assert!(
                entry.spent_day_mist + entry.reserved_day_mist + amount_mist <= cap,
                ECapExceeded,
            );
        };
        if (option::is_some(&entry.monthly_cap_mist)) {
            let cap = *option::borrow(&entry.monthly_cap_mist);
            assert!(
                entry.spent_month_mist + entry.reserved_month_mist + amount_mist <= cap,
                ECapExceeded,
            );
        };
    }

    fun release_reservation_counters(
        balance: &mut AiCreditBalance,
        reservation: &SpendReservation,
        now: u64,
    ) {
        roll_account_windows(balance, now);
        balance.reserved_mist = balance.reserved_mist - reservation.max_amount_mist;
        if (balance.day_anchor_ms == reservation.account_day_anchor_ms) {
            balance.reserved_day_mist =
                balance.reserved_day_mist - reservation.max_amount_mist;
        };
        if (balance.month_anchor_ms == reservation.account_month_anchor_ms) {
            balance.reserved_month_mist =
                balance.reserved_month_mist - reservation.max_amount_mist;
        };

        if (reservation.agent_budget_reserved) {
            assert!(
                table::contains(&balance.agent_budgets, reservation.agent_object_id),
                EReservationMismatch,
            );
            let entry = table::borrow_mut(
                &mut balance.agent_budgets,
                reservation.agent_object_id,
            );
            roll_agent_windows(entry, now);
            entry.reserved_mist = entry.reserved_mist - reservation.max_amount_mist;
            if (entry.day_anchor_ms == reservation.agent_day_anchor_ms) {
                entry.reserved_day_mist =
                    entry.reserved_day_mist - reservation.max_amount_mist;
            };
            if (entry.month_anchor_ms == reservation.agent_month_anchor_ms) {
                entry.reserved_month_mist =
                    entry.reserved_month_mist - reservation.max_amount_mist;
            };
        };
    }

    fun capture_reservation_counters(
        balance: &mut AiCreditBalance,
        reservation: &SpendReservation,
        amount_mist: u64,
        now: u64,
    ) {
        roll_account_windows(balance, now);
        balance.reserved_mist = balance.reserved_mist - reservation.max_amount_mist;
        balance.spent_total_mist = balance.spent_total_mist + amount_mist;
        if (balance.day_anchor_ms == reservation.account_day_anchor_ms) {
            balance.reserved_day_mist =
                balance.reserved_day_mist - reservation.max_amount_mist;
            balance.spent_day_mist = balance.spent_day_mist + amount_mist;
        };
        if (balance.month_anchor_ms == reservation.account_month_anchor_ms) {
            balance.reserved_month_mist =
                balance.reserved_month_mist - reservation.max_amount_mist;
            balance.spent_month_mist = balance.spent_month_mist + amount_mist;
        };

        if (reservation.agent_budget_reserved) {
            assert!(
                table::contains(&balance.agent_budgets, reservation.agent_object_id),
                EReservationMismatch,
            );
            let entry = table::borrow_mut(
                &mut balance.agent_budgets,
                reservation.agent_object_id,
            );
            roll_agent_windows(entry, now);
            entry.reserved_mist = entry.reserved_mist - reservation.max_amount_mist;
            entry.spent_mist = entry.spent_mist + amount_mist;
            if (entry.day_anchor_ms == reservation.agent_day_anchor_ms) {
                entry.reserved_day_mist =
                    entry.reserved_day_mist - reservation.max_amount_mist;
                entry.spent_day_mist = entry.spent_day_mist + amount_mist;
            };
            if (entry.month_anchor_ms == reservation.agent_month_anchor_ms) {
                entry.reserved_month_mist =
                    entry.reserved_month_mist - reservation.max_amount_mist;
                entry.spent_month_mist = entry.spent_month_mist + amount_mist;
            };
        };
    }

    // ============================================================
    // Test helpers
    // ============================================================

    #[test_only]
    public fun test_init(treasury: address, oracle_pubkey: vector<u8>, ctx: &mut TxContext) {
        bootstrap_init(treasury, oracle_pubkey, ctx);
        let cap = create_oracle_admin_cap(ctx);
        transfer::public_transfer(cap, tx_context::sender(ctx));
    }

    #[test_only]
    public fun test_create_and_share_balance(
        config: &mut AiCreditConfig,
        account: &MemoryAccount,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        let _ = create_and_share_balance(
            config,
            object::id(account),
            memory::owner(account),
            memory::profile_id(account),
            clock,
            ctx,
        );
    }

    #[test_only]
    public fun settle_usage_for_testing(
        config: &AiCreditConfig,
        balance: &mut AiCreditBalance,
        account: &MemoryAccount,
        agent: &SubAgent,
        receipt: UsageReceipt,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        execute_settlement(config, balance, account, agent, receipt, clock, ctx);
    }

    #[test_only]
    public fun reserve_spend_for_testing(
        config: &AiCreditConfig,
        balance: &mut AiCreditBalance,
        account: &MemoryAccount,
        agent: &SubAgent,
        reservation_nonce: u64,
        max_amount_mist: u64,
        provider_envelope_hash: vector<u8>,
        request_hash: vector<u8>,
        fx_quote_id: vector<u8>,
        myso_usd_e8: u64,
        timestamp_ms: u64,
        capture_deadline_ms: u64,
        hard_expiry_ms: u64,
        clock: &Clock,
    ) {
        let intent = make_spend_reservation_intent_from_objects(
            balance,
            agent,
            reservation_nonce,
            max_amount_mist,
            provider_envelope_hash,
            request_hash,
            fx_quote_id,
            myso_usd_e8,
            config.oracle_markup_bps,
            timestamp_ms,
            capture_deadline_ms,
            hard_expiry_ms,
        );
        execute_reservation(config, balance, account, agent, intent, clock);
    }

    #[test_only]
    public fun capture_spend_for_testing(
        config: &AiCreditConfig,
        balance: &mut AiCreditBalance,
        reservation_nonce: u64,
        amount_mist: u64,
        provider_cost_usd_micros: u64,
        provider_generation_hash: vector<u8>,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(table::contains(&balance.reservations, reservation_nonce), EReservationNotFound);
        let reservation = *table::borrow(&balance.reservations, reservation_nonce);
        let intent = CaptureSpendIntent {
            balance_id: object::id(balance),
            agent_object_id: reservation.agent_object_id,
            reservation_nonce,
            amount_mist,
            provider_cost_usd_micros,
            provider_generation_hash,
            timestamp_ms: clock::timestamp_ms(clock),
        };
        execute_capture(config, balance, intent, clock, ctx);
    }

    #[test_only]
    public fun cancel_spend_for_testing(
        config: &AiCreditConfig,
        balance: &mut AiCreditBalance,
        reservation_nonce: u64,
        clock: &Clock,
    ) {
        assert!(table::contains(&balance.reservations, reservation_nonce), EReservationNotFound);
        let reservation = *table::borrow(&balance.reservations, reservation_nonce);
        let intent = CancelSpendIntent {
            balance_id: object::id(balance),
            agent_object_id: reservation.agent_object_id,
            reservation_nonce,
            timestamp_ms: clock::timestamp_ms(clock),
        };
        execute_cancellation(config, balance, intent, clock);
    }

    #[test_only]
    public fun test_make_receipt(
        balance_id: ID,
        agent_object_id: ID,
        receipt_id: u128,
        amount_mist: u64,
        usage_kind: u8,
        timestamp_ms: u64,
        settlement_nonce: u64,
    ): UsageReceipt {
        UsageReceipt {
            balance_id,
            agent_object_id,
            receipt_id,
            amount_mist,
            usage_kind,
            timestamp_ms,
            settlement_nonce,
        }
    }

    #[test_only]
    public fun error_invalid_signature(): u64 { EInvalidSignature }
    #[test_only]
    public fun error_invalid_nonce(): u64 { EInvalidNonce }
    #[test_only]
    public fun error_insufficient_balance(): u64 { EInsufficientBalance }
    #[test_only]
    public fun error_cap_exceeded(): u64 { ECapExceeded }
    #[test_only]
    public fun error_approval_required(): u64 { EApprovalRequired }
    #[test_only]
    public fun error_approval_expired(): u64 { EApprovalExpired }
    #[test_only]
    public fun error_approval_insufficient(): u64 { EApprovalInsufficient }
    #[test_only]
    public fun error_approval_not_found(): u64 { EApprovalNotFound }
    #[test_only]
    public fun error_not_descendant(): u64 { ENotDescendant }
    #[test_only]
    public fun error_not_parent_signer(): u64 { ENotParentSigner }
    #[test_only]
    public fun error_parent_envelope_exceeded(): u64 { EParentEnvelopeExceeded }
    #[test_only]
    public fun error_cannot_manage_self(): u64 { ECannotManageSelf }
    #[test_only]
    public fun error_agent_not_in_org(): u64 { EAgentNotInOrg }
    #[test_only]
    public fun error_invalid_expiry(): u64 { EInvalidExpiry }
    #[test_only]
    public fun error_agent_missing_cap(): u64 { EAgentMissingCap }
    #[test_only]
    public fun error_agent_disabled(): u64 { EAgentDisabled }
    #[test_only]
    public fun error_reservation_not_found(): u64 { EReservationNotFound }
    #[test_only]
    public fun error_reservation_expired(): u64 { EReservationExpired }
    #[test_only]
    public fun error_reservation_not_expired(): u64 { EReservationNotExpired }
    #[test_only]
    public fun error_capture_window_closed(): u64 { ECaptureWindowClosed }
    #[test_only]
    public fun error_reservation_mismatch(): u64 { EReservationMismatch }
    #[test_only]
    public fun error_invalid_reservation_window(): u64 { EInvalidReservationWindow }
    #[test_only]
    public fun error_invalid_hash(): u64 { EInvalidHash }
    #[test_only]
    public fun error_markup_mismatch(): u64 { EMarkupMismatch }

    #[test_only]
    public fun config_version(config: &AiCreditConfig): u64 {
        config.version
    }

    #[test_only]
    public fun balance_version(balance: &AiCreditBalance): u64 {
        balance.version
    }

    #[test_only]
    public fun test_force_config_version(config: &mut AiCreditConfig, v: u64) {
        config.version = v;
    }

    #[test_only]
    public fun test_force_balance_version(balance: &mut AiCreditBalance, v: u64) {
        balance.version = v;
    }

    #[test_only]
    public fun error_wrong_version(): u64 {
        EWrongVersion
    }

    #[test_only]
    public entry fun test_migrate_config(
        config: &mut AiCreditConfig,
        cap: &UpgradeAdminCap,
        ctx: &mut TxContext,
    ) {
        if (upgrade::test_migration_available()) {
            migrate_config(config, cap, ctx);
        } else {
            let simulated_target = upgrade::test_pre_upgrade_object_version() + 1;
            assert!(config.version < simulated_target, EWrongVersion);
            let old_version = config.version;
            config.version = upgrade::current_version();
            upgrade::emit_migration_event(
                object::id(config),
                string::utf8(b"AiCreditConfig"),
                old_version,
                tx_context::sender(ctx),
            );
        }
    }

    #[test_only]
    public entry fun test_migrate_balance(
        balance: &mut AiCreditBalance,
        cap: &UpgradeAdminCap,
        ctx: &mut TxContext,
    ) {
        if (upgrade::test_migration_available()) {
            migrate_balance(balance, cap, ctx);
        } else {
            let simulated_target = upgrade::test_pre_upgrade_object_version() + 1;
            assert!(balance.version < simulated_target, EWrongVersion);
            let old_version = balance.version;
            balance.version = upgrade::current_version();
            upgrade::emit_migration_event(
                object::id(balance),
                string::utf8(b"AiCreditBalance"),
                old_version,
                tx_context::sender(ctx),
            );
        }
    }
}
