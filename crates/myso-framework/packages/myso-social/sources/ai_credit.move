// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

/// AI credit escrow — users deposit MYSO; platform oracle settles signed usage receipts
/// for sub-agent inference, tools, storage, and workflow costs.
///
/// Credits display: `credits = balance_mist / MIST_PER_MYSO` (1 MYSO = 1 credit).

#[allow(duplicate_alias, lint(public_entry))]
module social_contracts::ai_credit {
    use std::bcs;
    use std::option::{Self, Option};
    use std::vector;

    use myso::{
        balance::{Self, Balance},
        clock::{Self, Clock},
        coin::{Self, Coin},
        ed25519,
        event,
        object::{Self, ID, UID},
        table::{Self, Table},
        transfer,
        tx_context::{Self, TxContext},
    };
    use myso::myso::MYSO;

    use social_contracts::memory::{Self, MemoryAccount, SubAgent};
    use social_contracts::upgrade;

    const MIST_PER_MYSO: u64 = 1_000_000_000;
    const DAY_MS: u64 = 86_400_000;
    const MONTH_MS: u64 = 30 * DAY_MS;
    const INTENT_AI_CREDIT_USAGE: u8 = 1;
    const ED25519_PK_LEN: u64 = 32;

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
        balances_by_memory: Table<ID, ID>,
        version: u64,
    }

    public struct AgentBudgetEntry has store, copy, drop {
        agent_object_id: ID,
        derived_address: address,
        enabled: bool,
        budget_mist: Option<u64>,
        spent_mist: u64,
        daily_cap_mist: Option<u64>,
        monthly_cap_mist: Option<u64>,
        spent_day_mist: u64,
        spent_month_mist: u64,
        day_anchor_ms: u64,
        month_anchor_ms: u64,
        require_approval_above_mist: Option<u64>,
    }

    public struct AiCreditBalance has key {
        id: UID,
        memory_account_id: ID,
        principal_owner: address,
        profile_id: address,
        balance: Balance<MYSO>,
        spent_total_mist: u64,
        spent_day_mist: u64,
        spent_month_mist: u64,
        day_anchor_ms: u64,
        month_anchor_ms: u64,
        daily_cap_mist: Option<u64>,
        monthly_cap_mist: Option<u64>,
        settlement_nonce: u64,
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
        credits: u64,
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

    public struct AiCreditUsageSettled has copy, drop {
        balance_id: ID,
        agent_object_id: ID,
        receipt_id: u128,
        amount_mist: u64,
        usage_kind: u8,
        settlement_nonce: u64,
        remaining_mist: u64,
        credits_remaining: u64,
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
    }

    public struct AiCreditSettlementLimitsUpdated has copy, drop {
        max_single_settlement_mist: u64,
        receipt_ttl_ms: u64,
    }

    public struct AiCreditConfigInitialized has copy, drop {
        oracle_pubkey: vector<u8>,
        treasury: address,
        min_deposit_mist: u64,
        max_single_settlement_mist: u64,
        receipt_ttl_ms: u64,
    }

    // ============================================================
    // Bootstrap
    // ============================================================

    public(package) fun bootstrap_init(treasury: address, oracle_pubkey: vector<u8>, ctx: &mut TxContext) {
        assert!(vector::length(&oracle_pubkey) == ED25519_PK_LEN, EInvalidPubkey);
        let min_deposit_mist = MIST_PER_MYSO;
        let max_single_settlement_mist = 1000 * MIST_PER_MYSO;
        let receipt_ttl_ms = 300_000;
        event::emit(AiCreditConfigInitialized {
            oracle_pubkey,
            treasury,
            min_deposit_mist,
            max_single_settlement_mist,
            receipt_ttl_ms,
        });
        let config = AiCreditConfig {
            id: object::new(ctx),
            oracle_pubkey,
            treasury,
            min_deposit_mist,
            max_single_settlement_mist,
            receipt_ttl_ms,
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

    /// Called only from [`profile::transfer_profile_with_memory`].
    public(package) fun transfer_balance_owner(balance: &mut AiCreditBalance, new_owner: address) {
        balance.principal_owner = new_owner;
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
            spent_day_mist: 0,
            spent_month_mist: 0,
            day_anchor_ms: now,
            month_anchor_ms: now,
            daily_cap_mist: option::none(),
            monthly_cap_mist: option::none(),
            settlement_nonce: 0,
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
            credits: credits_from_mist(new_balance_mist),
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
        let agent_id = memory::agent_object_id(agent);
        let now = clock::timestamp_ms(clock);
        let entry = if (table::contains(&balance.agent_budgets, agent_id)) {
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
                daily_cap_mist,
                monthly_cap_mist,
                spent_day_mist: 0,
                spent_month_mist: 0,
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
    }

    public entry fun disable_agent_budget(
        config: &AiCreditConfig,
        balance: &mut AiCreditBalance,
        agent_object_id: ID,
        ctx: &TxContext,
    ) {
        assert_version(config, balance);
        assert_owner(balance, ctx);
        assert!(table::contains(&balance.agent_budgets, agent_object_id), EAgentNotFound);
        let entry = table::borrow_mut(&mut balance.agent_budgets, agent_object_id);
        entry.enabled = false;
        event::emit(AiCreditAgentBudgetDisabled {
            balance_id: object::id(balance),
            agent_object_id,
        });
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
            credits_remaining: credits_from_mist(remaining),
        });
        if (remaining == 0) {
            event::emit(AiCreditBalanceDepleted {
                balance_id: object::id(balance),
            });
        };
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
        assert!(vector::length(&new_pk) == ED25519_PK_LEN, EInvalidPubkey);
        config.oracle_pubkey = new_pk;
        event::emit(AiCreditOraclePubkeyUpdated {
            updated_by: tx_context::sender(ctx),
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
        balance_mist(balance)
    }

    public fun credits_from_mist(mist: u64): u64 {
        mist / MIST_PER_MYSO
    }

    public fun mist_from_credits(credits: u64): u64 {
        credits * MIST_PER_MYSO
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
        if (entry.spent_mist >= max) {
            option::some(0)
        } else {
            option::some(max - entry.spent_mist)
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

    fun assert_receipt_fresh(config: &AiCreditConfig, receipt: &UsageReceipt, clock: &Clock) {
        let now = clock::timestamp_ms(clock);
        assert!(receipt.timestamp_ms <= now, EStaleReceipt);
        assert!(now - receipt.timestamp_ms <= config.receipt_ttl_ms, EStaleReceipt);
    }

    fun roll_account_windows(balance: &mut AiCreditBalance, now: u64) {
        if (now >= balance.day_anchor_ms + DAY_MS) {
            balance.spent_day_mist = 0;
            balance.day_anchor_ms = now;
        };
        if (now >= balance.month_anchor_ms + MONTH_MS) {
            balance.spent_month_mist = 0;
            balance.month_anchor_ms = now;
        };
    }

    fun roll_agent_windows(entry: &mut AgentBudgetEntry, now: u64) {
        if (now >= entry.day_anchor_ms + DAY_MS) {
            entry.spent_day_mist = 0;
            entry.day_anchor_ms = now;
        };
        if (now >= entry.month_anchor_ms + MONTH_MS) {
            entry.spent_month_mist = 0;
            entry.month_anchor_ms = now;
        };
    }

    fun assert_account_caps(balance: &AiCreditBalance, amount_mist: u64) {
        if (option::is_some(&balance.daily_cap_mist)) {
            let cap = *option::borrow(&balance.daily_cap_mist);
            assert!(balance.spent_day_mist + amount_mist <= cap, ECapExceeded);
        };
        if (option::is_some(&balance.monthly_cap_mist)) {
            let cap = *option::borrow(&balance.monthly_cap_mist);
            assert!(balance.spent_month_mist + amount_mist <= cap, ECapExceeded);
        };
    }

    fun assert_agent_caps(entry: &AgentBudgetEntry, amount_mist: u64) {
        if (option::is_some(&entry.budget_mist)) {
            let cap = *option::borrow(&entry.budget_mist);
            assert!(entry.spent_mist + amount_mist <= cap, ECapExceeded);
        };
        if (option::is_some(&entry.daily_cap_mist)) {
            let cap = *option::borrow(&entry.daily_cap_mist);
            assert!(entry.spent_day_mist + amount_mist <= cap, ECapExceeded);
        };
        if (option::is_some(&entry.monthly_cap_mist)) {
            let cap = *option::borrow(&entry.monthly_cap_mist);
            assert!(entry.spent_month_mist + amount_mist <= cap, ECapExceeded);
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
}
