---
title: Module `social_contracts::insurance`
---

Insurance module for SPoT positions
Sells coverage against losing outcomes and pays out deterministically on SPoT resolution.


-  [Struct `InsuranceAdminCap`](#social_contracts_insurance_InsuranceAdminCap)
-  [Struct `InsuranceConfig`](#social_contracts_insurance_InsuranceConfig)
-  [Struct `InsuranceRouterConfig`](#social_contracts_insurance_InsuranceRouterConfig)
-  [Struct `InsuranceBackstopPool`](#social_contracts_insurance_InsuranceBackstopPool)
-  [Struct `CoverageRoute`](#social_contracts_insurance_CoverageRoute)
-  [Struct `UnderwriterVault`](#social_contracts_insurance_UnderwriterVault)
-  [Struct `MarketExposure`](#social_contracts_insurance_MarketExposure)
-  [Struct `CoveragePolicy`](#social_contracts_insurance_CoveragePolicy)
-  [Struct `PremiumQuote`](#social_contracts_insurance_PremiumQuote)
-  [Struct `VaultCoverageQuote`](#social_contracts_insurance_VaultCoverageQuote)
-  [Struct `RiskPricingConfigUpdatedEvent`](#social_contracts_insurance_RiskPricingConfigUpdatedEvent)
-  [Struct `ConfigInitializedEvent`](#social_contracts_insurance_ConfigInitializedEvent)
-  [Struct `UnderwriterVaultCreatedEvent`](#social_contracts_insurance_UnderwriterVaultCreatedEvent)
-  [Struct `VaultStatusUpdatedEvent`](#social_contracts_insurance_VaultStatusUpdatedEvent)
-  [Struct `UnderwriterVaultDepositedEvent`](#social_contracts_insurance_UnderwriterVaultDepositedEvent)
-  [Struct `UnderwriterVaultWithdrawnEvent`](#social_contracts_insurance_UnderwriterVaultWithdrawnEvent)
-  [Struct `CoveragePurchasedEvent`](#social_contracts_insurance_CoveragePurchasedEvent)
-  [Struct `CoverageRoutedEvent`](#social_contracts_insurance_CoverageRoutedEvent)
-  [Struct `RouteFillEvent`](#social_contracts_insurance_RouteFillEvent)
-  [Struct `BackstopUsedEvent`](#social_contracts_insurance_BackstopUsedEvent)
-  [Struct `BackstopTreasuryDepositEvent`](#social_contracts_insurance_BackstopTreasuryDepositEvent)
-  [Struct `CoverageCancelledEvent`](#social_contracts_insurance_CoverageCancelledEvent)
-  [Struct `CoverageClaimedEvent`](#social_contracts_insurance_CoverageClaimedEvent)
-  [Struct `ConfigUpdatedEvent`](#social_contracts_insurance_ConfigUpdatedEvent)
-  [Struct `PolicyExpiredEvent`](#social_contracts_insurance_PolicyExpiredEvent)
-  [Constants](#@Constants_0)
-  [Function `init_config`](#social_contracts_insurance_init_config)
-  [Function `set_config`](#social_contracts_insurance_set_config)
-  [Function `set_risk_pricing_config`](#social_contracts_insurance_set_risk_pricing_config)
-  [Function `set_enable_flag`](#social_contracts_insurance_set_enable_flag)
-  [Function `create_insurance_admin_cap`](#social_contracts_insurance_create_insurance_admin_cap)
-  [Function `bootstrap_init`](#social_contracts_insurance_bootstrap_init)
-  [Function `create_vault`](#social_contracts_insurance_create_vault)
-  [Function `set_vault_status`](#social_contracts_insurance_set_vault_status)
-  [Function `set_router_flags`](#social_contracts_insurance_set_router_flags)
-  [Function `set_router_limits`](#social_contracts_insurance_set_router_limits)
-  [Function `set_market_pause`](#social_contracts_insurance_set_market_pause)
-  [Function `set_backstop_caps`](#social_contracts_insurance_set_backstop_caps)
-  [Function `set_tail_mode`](#social_contracts_insurance_set_tail_mode)
-  [Function `set_backstop_paused`](#social_contracts_insurance_set_backstop_paused)
-  [Function `deposit_backstop_treasury`](#social_contracts_insurance_deposit_backstop_treasury)
-  [Function `tail_pay_shortfall`](#social_contracts_insurance_tail_pay_shortfall)
-  [Function `min_cap_sub`](#social_contracts_insurance_min_cap_sub)
-  [Function `min_u64`](#social_contracts_insurance_min_u64)
-  [Function `copy_id_vec`](#social_contracts_insurance_copy_id_vec)
-  [Function `new_router_config_defaults`](#social_contracts_insurance_new_router_config_defaults)
-  [Function `new_backstop_pool_defaults`](#social_contracts_insurance_new_backstop_pool_defaults)
-  [Function `assert_market_router_open`](#social_contracts_insurance_assert_market_router_open)
-  [Function `assert_vault_buy_guards`](#social_contracts_insurance_assert_vault_buy_guards)
-  [Function `deposit_capital`](#social_contracts_insurance_deposit_capital)
-  [Function `withdraw_capital`](#social_contracts_insurance_withdraw_capital)
-  [Function `premium_quote_premium`](#social_contracts_insurance_premium_quote_premium)
-  [Function `premium_quote_implied_prob_win_bps`](#social_contracts_insurance_premium_quote_implied_prob_win_bps)
-  [Function `premium_quote_risk_multiplier_bps`](#social_contracts_insurance_premium_quote_risk_multiplier_bps)
-  [Function `premium_quote_premium_raw`](#social_contracts_insurance_premium_quote_premium_raw)
-  [Function `quote_base_premium`](#social_contracts_insurance_quote_base_premium)
-  [Function `quote_premium`](#social_contracts_insurance_quote_premium)
-  [Function `get_market_option_reserved`](#social_contracts_insurance_get_market_option_reserved)
-  [Function `compute_spot_risk_quote`](#social_contracts_insurance_compute_spot_risk_quote)
-  [Function `quote_premium_with_spot_risk`](#social_contracts_insurance_quote_premium_with_spot_risk)
-  [Function `coverage_quote_skipped`](#social_contracts_insurance_coverage_quote_skipped)
-  [Function `max_fill_covered_for_vault`](#social_contracts_insurance_max_fill_covered_for_vault)
-  [Function `reserve_to_covered`](#social_contracts_insurance_reserve_to_covered)
-  [Function `quote_vault_for_spot_coverage`](#social_contracts_insurance_quote_vault_for_spot_coverage)
-  [Function `vault_utilization_bps`](#social_contracts_insurance_vault_utilization_bps)
-  [Function `buy_coverage_execute`](#social_contracts_insurance_buy_coverage_execute)
-  [Function `buy_coverage`](#social_contracts_insurance_buy_coverage)
-  [Function `route_buy_coverage_4`](#social_contracts_insurance_route_buy_coverage_4)
-  [Function `cancel_coverage`](#social_contracts_insurance_cancel_coverage)
-  [Function `claim`](#social_contracts_insurance_claim)
-  [Function `expire_policy`](#social_contracts_insurance_expire_policy)
-  [Function `compute_reserve`](#social_contracts_insurance_compute_reserve)
-  [Function `enforce_exposure_limits`](#social_contracts_insurance_enforce_exposure_limits)
-  [Function `add_exposure`](#social_contracts_insurance_add_exposure)
-  [Function `release_exposure`](#social_contracts_insurance_release_exposure)
-  [Function `get_market_exposure_mut`](#social_contracts_insurance_get_market_exposure_mut)
-  [Function `get_user_exposure`](#social_contracts_insurance_get_user_exposure)
-  [Function `set_user_exposure`](#social_contracts_insurance_set_user_exposure)
-  [Function `get_option_reserved`](#social_contracts_insurance_get_option_reserved)
-  [Function `set_option_reserved`](#social_contracts_insurance_set_option_reserved)
-  [Function `migrate_config`](#social_contracts_insurance_migrate_config)
-  [Function `migrate_vault`](#social_contracts_insurance_migrate_vault)


<pre><code><b>use</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption">mydata::bf_hmac_encryption</a>;
<b>use</b> <a href="../mydata/gf256.md#mydata_gf256">mydata::gf256</a>;
<b>use</b> <a href="../mydata/hmac256ctr.md#mydata_hmac256ctr">mydata::hmac256ctr</a>;
<b>use</b> <a href="../mydata/kdf.md#mydata_kdf">mydata::kdf</a>;
<b>use</b> <a href="../mydata/merkle.md#mydata_merkle">mydata::merkle</a>;
<b>use</b> <a href="../mydata/polynomial.md#mydata_polynomial">mydata::polynomial</a>;
<b>use</b> <a href="../myso/accumulator.md#myso_accumulator">myso::accumulator</a>;
<b>use</b> <a href="../myso/accumulator_settlement.md#myso_accumulator_settlement">myso::accumulator_settlement</a>;
<b>use</b> <a href="../myso/address.md#myso_address">myso::address</a>;
<b>use</b> <a href="../myso/bag.md#myso_bag">myso::bag</a>;
<b>use</b> <a href="../myso/balance.md#myso_balance">myso::balance</a>;
<b>use</b> <a href="../myso/bcs.md#myso_bcs">myso::bcs</a>;
<b>use</b> <a href="../myso/bls12381.md#myso_bls12381">myso::bls12381</a>;
<b>use</b> <a href="../myso/bootstrap_key.md#myso_bootstrap_key">myso::bootstrap_key</a>;
<b>use</b> <a href="../myso/clock.md#myso_clock">myso::clock</a>;
<b>use</b> <a href="../myso/coin.md#myso_coin">myso::coin</a>;
<b>use</b> <a href="../myso/config.md#myso_config">myso::config</a>;
<b>use</b> <a href="../myso/deny_list.md#myso_deny_list">myso::deny_list</a>;
<b>use</b> <a href="../myso/derived_object.md#myso_derived_object">myso::derived_object</a>;
<b>use</b> <a href="../myso/dynamic_field.md#myso_dynamic_field">myso::dynamic_field</a>;
<b>use</b> <a href="../myso/dynamic_object_field.md#myso_dynamic_object_field">myso::dynamic_object_field</a>;
<b>use</b> <a href="../myso/event.md#myso_event">myso::event</a>;
<b>use</b> <a href="../myso/funds_accumulator.md#myso_funds_accumulator">myso::funds_accumulator</a>;
<b>use</b> <a href="../myso/group_ops.md#myso_group_ops">myso::group_ops</a>;
<b>use</b> <a href="../myso/hash.md#myso_hash">myso::hash</a>;
<b>use</b> <a href="../myso/hex.md#myso_hex">myso::hex</a>;
<b>use</b> <a href="../myso/hmac.md#myso_hmac">myso::hmac</a>;
<b>use</b> <a href="../myso/myso.md#myso_myso">myso::myso</a>;
<b>use</b> <a href="../myso/object.md#myso_object">myso::object</a>;
<b>use</b> <a href="../myso/package.md#myso_package">myso::package</a>;
<b>use</b> <a href="../myso/party.md#myso_party">myso::party</a>;
<b>use</b> <a href="../myso/protocol_config.md#myso_protocol_config">myso::protocol_config</a>;
<b>use</b> <a href="../myso/table.md#myso_table">myso::table</a>;
<b>use</b> <a href="../myso/transfer.md#myso_transfer">myso::transfer</a>;
<b>use</b> <a href="../myso/tx_context.md#myso_tx_context">myso::tx_context</a>;
<b>use</b> <a href="../myso/types.md#myso_types">myso::types</a>;
<b>use</b> <a href="../myso/url.md#myso_url">myso::url</a>;
<b>use</b> <a href="../myso/vec_map.md#myso_vec_map">myso::vec_map</a>;
<b>use</b> <a href="../myso/vec_set.md#myso_vec_set">myso::vec_set</a>;
<b>use</b> <a href="../myso_groups/permissioned_group.md#myso_groups_permissioned_group">myso_groups::permissioned_group</a>;
<b>use</b> <a href="../myso_groups/permissions_table.md#myso_groups_permissions_table">myso_groups::permissions_table</a>;
<b>use</b> <a href="../myso_groups/unpause_cap.md#myso_groups_unpause_cap">myso_groups::unpause_cap</a>;
<b>use</b> <a href="../social_contracts/block_list.md#social_contracts_block_list">social_contracts::block_list</a>;
<b>use</b> <a href="../social_contracts/governance.md#social_contracts_governance">social_contracts::governance</a>;
<b>use</b> <a href="../social_contracts/memory.md#social_contracts_memory">social_contracts::memory</a>;
<b>use</b> <a href="../social_contracts/mydata.md#social_contracts_mydata">social_contracts::mydata</a>;
<b>use</b> <a href="../social_contracts/platform.md#social_contracts_platform">social_contracts::platform</a>;
<b>use</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault">social_contracts::poc_vault</a>;
<b>use</b> <a href="../social_contracts/post.md#social_contracts_post">social_contracts::post</a>;
<b>use</b> <a href="../social_contracts/profile.md#social_contracts_profile">social_contracts::profile</a>;
<b>use</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_contracts::social_graph</a>;
<b>use</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth">social_contracts::social_proof_of_truth</a>;
<b>use</b> <a href="../social_contracts/subscription.md#social_contracts_subscription">social_contracts::subscription</a>;
<b>use</b> <a href="../social_contracts/upgrade.md#social_contracts_upgrade">social_contracts::upgrade</a>;
<b>use</b> <a href="../std/address.md#std_address">std::address</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/hash.md#std_hash">std::hash</a>;
<b>use</b> <a href="../std/internal.md#std_internal">std::internal</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/type_name.md#std_type_name">std::type_name</a>;
<b>use</b> <a href="../std/u128.md#std_u128">std::u128</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
</code></pre>



<a name="social_contracts_insurance_InsuranceAdminCap"></a>

## Struct `InsuranceAdminCap`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceAdminCap">InsuranceAdminCap</a> <b>has</b> key, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../myso/object.md#myso_object_UID">myso::object::UID</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_insurance_InsuranceConfig"></a>

## Struct `InsuranceConfig`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceConfig">InsuranceConfig</a> <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../myso/object.md#myso_object_UID">myso::object::UID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>enable_flag: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>min_coverage_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_coverage_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_duration_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>fee_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>min_spot_total_liquidity: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_coverage_fraction_of_option_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_risk_multiplier_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>min_premium_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>spot_smoothing_per_option: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>implied_prob_floor_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>odds_floor_1x: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>odds_cap_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>liq_cap_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>liq_ref_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>exposure_cap_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>exposure_k_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>version: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_insurance_InsuranceRouterConfig"></a>

## Struct `InsuranceRouterConfig`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceRouterConfig">InsuranceRouterConfig</a> <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../myso/object.md#myso_object_UID">myso::object::UID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>router_enabled: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>router_paused: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>max_route_reserve_market: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_route_reserve_user: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_route_reserve_option: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_vault_concentration_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>min_vault_health_factor_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>market_pause: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;<b>address</b>, bool&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>version: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_insurance_InsuranceBackstopPool"></a>

## Struct `InsuranceBackstopPool`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceBackstopPool">InsuranceBackstopPool</a> <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../myso/object.md#myso_object_UID">myso::object::UID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>capital: <a href="../myso/balance.md#myso_balance_Balance">myso::balance::Balance</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>total_paid_out: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>paid_by_market: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;<b>address</b>, u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>max_payout_per_market: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_payout_per_event: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>global_hard_cap: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>tail_mode_enabled: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>paused: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>sweep_premium_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>tail_pay_partial_on_cap: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>version: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_insurance_CoverageRoute"></a>

## Struct `CoverageRoute`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_CoverageRoute">CoverageRoute</a> <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../myso/object.md#myso_object_UID">myso::object::UID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>insured: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>market_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>option_id: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>coverage_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>start_time_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>expiry_time_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>policy_ids: vector&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>vault_ids: vector&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>total_covered: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>total_premium: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>total_reserve: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>total_backstop_sweep: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>version: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_insurance_UnderwriterVault"></a>

## Struct `UnderwriterVault`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">UnderwriterVault</a> <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../myso/object.md#myso_object_UID">myso::object::UID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>underwriter: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>capital: <a href="../myso/balance.md#myso_balance_Balance">myso::balance::Balance</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>reserved: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>base_rate_bps_per_day: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>utilization_multiplier_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_exposure_per_market: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_exposure_per_user: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_exposure_per_option: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>enabled: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>paused: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>market_exposures: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;<b>address</b>, <a href="../social_contracts/insurance.md#social_contracts_insurance_MarketExposure">social_contracts::insurance::MarketExposure</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>user_exposures: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;<b>address</b>, u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>version: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_insurance_MarketExposure"></a>

## Struct `MarketExposure`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_MarketExposure">MarketExposure</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>market_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>total_reserved: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>reserved_by_option: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;u8, u64&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_insurance_CoveragePolicy"></a>

## Struct `CoveragePolicy`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_CoveragePolicy">CoveragePolicy</a> <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../myso/object.md#myso_object_UID">myso::object::UID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>market_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>insured: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>option_id: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>covered_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>coverage_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>premium_paid: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>start_time_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>expiry_time_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>vault_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>status: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>route_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>route_leg_index: u8</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_insurance_PremiumQuote"></a>

## Struct `PremiumQuote`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_PremiumQuote">PremiumQuote</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>premium: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>premium_raw: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>implied_prob_win_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>risk_multiplier_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>market_total_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>option_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>base_premium: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_insurance_VaultCoverageQuote"></a>

## Struct `VaultCoverageQuote`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_VaultCoverageQuote">VaultCoverageQuote</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>premium: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>premium_raw: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>reserve_required: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>available_capacity_reserve: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>risk_multiplier_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>implied_prob_win_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>utilization_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_fill_covered_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>skipped_reason: u8</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_insurance_RiskPricingConfigUpdatedEvent"></a>

## Struct `RiskPricingConfigUpdatedEvent`

Events


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_RiskPricingConfigUpdatedEvent">RiskPricingConfigUpdatedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>updated_by: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>min_spot_total_liquidity: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_coverage_fraction_of_option_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_risk_multiplier_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>min_premium_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>spot_smoothing_per_option: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>implied_prob_floor_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>odds_floor_1x: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>odds_cap_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>liq_cap_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>liq_ref_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>exposure_cap_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>exposure_k_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>timestamp: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_insurance_ConfigInitializedEvent"></a>

## Struct `ConfigInitializedEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_ConfigInitializedEvent">ConfigInitializedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>admin: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>min_coverage_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_coverage_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_duration_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>fee_bps: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_insurance_UnderwriterVaultCreatedEvent"></a>

## Struct `UnderwriterVaultCreatedEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVaultCreatedEvent">UnderwriterVaultCreatedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>vault_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>underwriter: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>base_rate_bps_per_day: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>utilization_multiplier_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_exposure_per_market: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_exposure_per_user: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_exposure_per_option: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>enabled: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>paused: bool</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_insurance_VaultStatusUpdatedEvent"></a>

## Struct `VaultStatusUpdatedEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_VaultStatusUpdatedEvent">VaultStatusUpdatedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>vault_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>enabled: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>paused: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>max_exposure_per_option: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_exposure_per_market: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_exposure_per_user: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>base_rate_bps_per_day: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>utilization_multiplier_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>updated_by: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>timestamp_ms: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_insurance_UnderwriterVaultDepositedEvent"></a>

## Struct `UnderwriterVaultDepositedEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVaultDepositedEvent">UnderwriterVaultDepositedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>vault_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>new_balance: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_insurance_UnderwriterVaultWithdrawnEvent"></a>

## Struct `UnderwriterVaultWithdrawnEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVaultWithdrawnEvent">UnderwriterVaultWithdrawnEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>vault_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>new_balance: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_insurance_CoveragePurchasedEvent"></a>

## Struct `CoveragePurchasedEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_CoveragePurchasedEvent">CoveragePurchasedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>policy_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>vault_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>market_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>insured: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>option_id: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>covered_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>coverage_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>premium_paid: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>premium_raw: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>reserve_locked: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>expiry_time_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>implied_probability_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>risk_multiplier_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>base_premium: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>market_total_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>option_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>backstop_sweep_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>route_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>route_leg_index: u8</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_insurance_CoverageRoutedEvent"></a>

## Struct `CoverageRoutedEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_CoverageRoutedEvent">CoverageRoutedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>route_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>insured: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>market_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>option_id: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>coverage_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>duration_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>total_covered: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>total_premium: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>total_reserve: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>total_backstop_sweep: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>expiry_time_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>policy_ids: vector&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>vault_ids: vector&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_insurance_RouteFillEvent"></a>

## Struct `RouteFillEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_RouteFillEvent">RouteFillEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>route_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>leg_index: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>vault_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>policy_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>covered_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>premium_paid: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>reserve_locked: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>backstop_sweep_amount: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_insurance_BackstopUsedEvent"></a>

## Struct `BackstopUsedEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_BackstopUsedEvent">BackstopUsedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>market_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>recipient: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>total_paid_out_after: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>tail_mode_enabled: bool</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_insurance_BackstopTreasuryDepositEvent"></a>

## Struct `BackstopTreasuryDepositEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_BackstopTreasuryDepositEvent">BackstopTreasuryDepositEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>depositor: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>new_balance: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_insurance_CoverageCancelledEvent"></a>

## Struct `CoverageCancelledEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_CoverageCancelledEvent">CoverageCancelledEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>policy_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>insured: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>refunded_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>fee_paid: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_insurance_CoverageClaimedEvent"></a>

## Struct `CoverageClaimedEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_CoverageClaimedEvent">CoverageClaimedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>policy_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>insured: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>payout: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_insurance_ConfigUpdatedEvent"></a>

## Struct `ConfigUpdatedEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_ConfigUpdatedEvent">ConfigUpdatedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>updated_by: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>enable_flag: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>min_coverage_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_coverage_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_duration_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>fee_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>timestamp: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_insurance_PolicyExpiredEvent"></a>

## Struct `PolicyExpiredEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_PolicyExpiredEvent">PolicyExpiredEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>policy_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>insured: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>market_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>vault_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>reserve_released: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>expiry_time_ms: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="social_contracts_insurance_ENotAdmin"></a>

Errors


<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_ENotAdmin">ENotAdmin</a>: u64 = 1;
</code></pre>



<a name="social_contracts_insurance_EDisabled"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_EDisabled">EDisabled</a>: u64 = 2;
</code></pre>



<a name="social_contracts_insurance_EInvalidCoverage"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_EInvalidCoverage">EInvalidCoverage</a>: u64 = 3;
</code></pre>



<a name="social_contracts_insurance_EInvalidDuration"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_EInvalidDuration">EInvalidDuration</a>: u64 = 4;
</code></pre>



<a name="social_contracts_insurance_EInvalidAmount"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_EInvalidAmount">EInvalidAmount</a>: u64 = 5;
</code></pre>



<a name="social_contracts_insurance_EInvalidVault"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_EInvalidVault">EInvalidVault</a>: u64 = 6;
</code></pre>



<a name="social_contracts_insurance_EInsufficientCapital"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_EInsufficientCapital">EInsufficientCapital</a>: u64 = 7;
</code></pre>



<a name="social_contracts_insurance_EMarketClosed"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_EMarketClosed">EMarketClosed</a>: u64 = 8;
</code></pre>



<a name="social_contracts_insurance_EPolicyNotActive"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_EPolicyNotActive">EPolicyNotActive</a>: u64 = 9;
</code></pre>



<a name="social_contracts_insurance_EPolicyExpired"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_EPolicyExpired">EPolicyExpired</a>: u64 = 10;
</code></pre>



<a name="social_contracts_insurance_ENotPolicyOwner"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_ENotPolicyOwner">ENotPolicyOwner</a>: u64 = 11;
</code></pre>



<a name="social_contracts_insurance_EOverflow"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_EOverflow">EOverflow</a>: u64 = 12;
</code></pre>



<a name="social_contracts_insurance_EMarketMismatch"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_EMarketMismatch">EMarketMismatch</a>: u64 = 13;
</code></pre>



<a name="social_contracts_insurance_EExposureLimit"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_EExposureLimit">EExposureLimit</a>: u64 = 14;
</code></pre>



<a name="social_contracts_insurance_EInsufficientPremium"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_EInsufficientPremium">EInsufficientPremium</a>: u64 = 15;
</code></pre>



<a name="social_contracts_insurance_EExposureInvariantBroken"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_EExposureInvariantBroken">EExposureInvariantBroken</a>: u64 = 16;
</code></pre>



<a name="social_contracts_insurance_EWrongVersion"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_EWrongVersion">EWrongVersion</a>: u64 = 17;
</code></pre>



<a name="social_contracts_insurance_EThinMarket"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_EThinMarket">EThinMarket</a>: u64 = 18;
</code></pre>



<a name="social_contracts_insurance_ECoverageTooLargeVersusPool"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_ECoverageTooLargeVersusPool">ECoverageTooLargeVersusPool</a>: u64 = 19;
</code></pre>



<a name="social_contracts_insurance_ERiskMultiplierTooHigh"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_ERiskMultiplierTooHigh">ERiskMultiplierTooHigh</a>: u64 = 20;
</code></pre>



<a name="social_contracts_insurance_EVaultDisabled"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_EVaultDisabled">EVaultDisabled</a>: u64 = 21;
</code></pre>



<a name="social_contracts_insurance_EVaultPaused"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_EVaultPaused">EVaultPaused</a>: u64 = 22;
</code></pre>



<a name="social_contracts_insurance_ERouterPaused"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_ERouterPaused">ERouterPaused</a>: u64 = 23;
</code></pre>



<a name="social_contracts_insurance_ERouteDisabled"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_ERouteDisabled">ERouteDisabled</a>: u64 = 24;
</code></pre>



<a name="social_contracts_insurance_EDeadlinePassed"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_EDeadlinePassed">EDeadlinePassed</a>: u64 = 25;
</code></pre>



<a name="social_contracts_insurance_ESlippagePremium"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_ESlippagePremium">ESlippagePremium</a>: u64 = 26;
</code></pre>



<a name="social_contracts_insurance_ESlippageCovered"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_ESlippageCovered">ESlippageCovered</a>: u64 = 27;
</code></pre>



<a name="social_contracts_insurance_EDuplicateVaultInRoute"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_EDuplicateVaultInRoute">EDuplicateVaultInRoute</a>: u64 = 28;
</code></pre>



<a name="social_contracts_insurance_EBackstopPaused"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_EBackstopPaused">EBackstopPaused</a>: u64 = 29;
</code></pre>



<a name="social_contracts_insurance_ETailModeDisabled"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_ETailModeDisabled">ETailModeDisabled</a>: u64 = 30;
</code></pre>



<a name="social_contracts_insurance_EBackstopPayoutLimit"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_EBackstopPayoutLimit">EBackstopPayoutLimit</a>: u64 = 31;
</code></pre>



<a name="social_contracts_insurance_EInvalidFills"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_EInvalidFills">EInvalidFills</a>: u64 = 32;
</code></pre>



<a name="social_contracts_insurance_EVaultConcentration"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_EVaultConcentration">EVaultConcentration</a>: u64 = 33;
</code></pre>



<a name="social_contracts_insurance_STATUS_ACTIVE"></a>

Status


<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_STATUS_ACTIVE">STATUS_ACTIVE</a>: u8 = 1;
</code></pre>



<a name="social_contracts_insurance_STATUS_CANCELLED"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_STATUS_CANCELLED">STATUS_CANCELLED</a>: u8 = 2;
</code></pre>



<a name="social_contracts_insurance_STATUS_CLAIMED"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_STATUS_CLAIMED">STATUS_CLAIMED</a>: u8 = 3;
</code></pre>



<a name="social_contracts_insurance_STATUS_EXPIRED"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_STATUS_EXPIRED">STATUS_EXPIRED</a>: u8 = 4;
</code></pre>



<a name="social_contracts_insurance_BPS_DENOM"></a>

Constants


<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_BPS_DENOM">BPS_DENOM</a>: u64 = 10000;
</code></pre>



<a name="social_contracts_insurance_DAY_MS"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_DAY_MS">DAY_MS</a>: u64 = 86400000;
</code></pre>



<a name="social_contracts_insurance_MAX_U64"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_MAX_U64">MAX_U64</a>: u64 = 18446744073709551615;
</code></pre>



<a name="social_contracts_insurance_DEFAULT_VERSION"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_VERSION">DEFAULT_VERSION</a>: u64 = 1;
</code></pre>



<a name="social_contracts_insurance_DEFAULT_MIN_COVERAGE_BPS"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_MIN_COVERAGE_BPS">DEFAULT_MIN_COVERAGE_BPS</a>: u64 = 1000;
</code></pre>



<a name="social_contracts_insurance_DEFAULT_MAX_COVERAGE_BPS"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_MAX_COVERAGE_BPS">DEFAULT_MAX_COVERAGE_BPS</a>: u64 = 9000;
</code></pre>



<a name="social_contracts_insurance_DEFAULT_MAX_DURATION_MS"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_MAX_DURATION_MS">DEFAULT_MAX_DURATION_MS</a>: u64 = 2592000000;
</code></pre>



<a name="social_contracts_insurance_DEFAULT_FEE_BPS"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_FEE_BPS">DEFAULT_FEE_BPS</a>: u64 = 50;
</code></pre>



<a name="social_contracts_insurance_DEFAULT_MIN_SPOT_TOTAL_LIQUIDITY"></a>

Default SPoT risk pricing (baseline pool size ~1000 MYSO at 10^9 scaling).


<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_MIN_SPOT_TOTAL_LIQUIDITY">DEFAULT_MIN_SPOT_TOTAL_LIQUIDITY</a>: u64 = 1;
</code></pre>



<a name="social_contracts_insurance_DEFAULT_MAX_COVERAGE_FRACTION_OF_OPTION_BPS"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_MAX_COVERAGE_FRACTION_OF_OPTION_BPS">DEFAULT_MAX_COVERAGE_FRACTION_OF_OPTION_BPS</a>: u64 = 10000;
</code></pre>



<a name="social_contracts_insurance_DEFAULT_MAX_RISK_MULTIPLIER_BPS"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_MAX_RISK_MULTIPLIER_BPS">DEFAULT_MAX_RISK_MULTIPLIER_BPS</a>: u64 = 500000;
</code></pre>



<a name="social_contracts_insurance_DEFAULT_MIN_PREMIUM_AMOUNT"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_MIN_PREMIUM_AMOUNT">DEFAULT_MIN_PREMIUM_AMOUNT</a>: u64 = 1;
</code></pre>



<a name="social_contracts_insurance_DEFAULT_SPOT_SMOOTHING_PER_OPTION"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_SPOT_SMOOTHING_PER_OPTION">DEFAULT_SPOT_SMOOTHING_PER_OPTION</a>: u64 = 0;
</code></pre>



<a name="social_contracts_insurance_DEFAULT_IMPLIED_PROB_FLOOR_BPS"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_IMPLIED_PROB_FLOOR_BPS">DEFAULT_IMPLIED_PROB_FLOOR_BPS</a>: u64 = 10;
</code></pre>



<a name="social_contracts_insurance_DEFAULT_ODDS_CAP_BPS"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_ODDS_CAP_BPS">DEFAULT_ODDS_CAP_BPS</a>: u64 = 500000;
</code></pre>



<a name="social_contracts_insurance_DEFAULT_LIQ_CAP_BPS"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_LIQ_CAP_BPS">DEFAULT_LIQ_CAP_BPS</a>: u64 = 500000;
</code></pre>



<a name="social_contracts_insurance_DEFAULT_LIQ_REF_AMOUNT"></a>

Target pool size such that liquidity multiplier ≈ 1× when <code>total_option_escrow == liq_ref_amount</code>.


<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_LIQ_REF_AMOUNT">DEFAULT_LIQ_REF_AMOUNT</a>: u64 = 1000000000000;
</code></pre>



<a name="social_contracts_insurance_DEFAULT_EXPOSURE_CAP_BPS"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_EXPOSURE_CAP_BPS">DEFAULT_EXPOSURE_CAP_BPS</a>: u64 = 30000;
</code></pre>



<a name="social_contracts_insurance_DEFAULT_EXPOSURE_K_BPS"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_EXPOSURE_K_BPS">DEFAULT_EXPOSURE_K_BPS</a>: u64 = 5000;
</code></pre>



<a name="social_contracts_insurance_SKIPPED_OK"></a>

<code><a href="../social_contracts/insurance.md#social_contracts_insurance_VaultCoverageQuote">VaultCoverageQuote</a>.skipped_reason</code> — 0 = quotable at <code>max_fill_covered_amount</code>.


<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_SKIPPED_OK">SKIPPED_OK</a>: u8 = 0;
</code></pre>



<a name="social_contracts_insurance_SKIPPED_VAULT_DISABLED"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_SKIPPED_VAULT_DISABLED">SKIPPED_VAULT_DISABLED</a>: u8 = 1;
</code></pre>



<a name="social_contracts_insurance_SKIPPED_VAULT_PAUSED"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_SKIPPED_VAULT_PAUSED">SKIPPED_VAULT_PAUSED</a>: u8 = 2;
</code></pre>



<a name="social_contracts_insurance_SKIPPED_ROUTER_PAUSED"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_SKIPPED_ROUTER_PAUSED">SKIPPED_ROUTER_PAUSED</a>: u8 = 3;
</code></pre>



<a name="social_contracts_insurance_SKIPPED_MARKET_PAUSED"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_SKIPPED_MARKET_PAUSED">SKIPPED_MARKET_PAUSED</a>: u8 = 4;
</code></pre>



<a name="social_contracts_insurance_SKIPPED_UNHEALTHY_VAULT"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_SKIPPED_UNHEALTHY_VAULT">SKIPPED_UNHEALTHY_VAULT</a>: u8 = 5;
</code></pre>



<a name="social_contracts_insurance_SKIPPED_RISK_MULTIPLIER"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_SKIPPED_RISK_MULTIPLIER">SKIPPED_RISK_MULTIPLIER</a>: u8 = 6;
</code></pre>



<a name="social_contracts_insurance_SKIPPED_ZERO_CAPACITY"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_SKIPPED_ZERO_CAPACITY">SKIPPED_ZERO_CAPACITY</a>: u8 = 7;
</code></pre>



<a name="social_contracts_insurance_SKIPPED_THIN_OR_POOL"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_SKIPPED_THIN_OR_POOL">SKIPPED_THIN_OR_POOL</a>: u8 = 8;
</code></pre>



<a name="social_contracts_insurance_MAX_ROUTE_LEGS"></a>



<pre><code><b>const</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_MAX_ROUTE_LEGS">MAX_ROUTE_LEGS</a>: u64 = 4;
</code></pre>



<a name="social_contracts_insurance_init_config"></a>

## Function `init_config`

Initialize config (package only)
Creates InsuranceConfig and transfers InsuranceAdminCap to caller.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_init_config">init_config</a>(min_coverage_bps: u64, max_coverage_bps: u64, max_duration_ms: u64, fee_bps: u64, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_init_config">init_config</a>(
    min_coverage_bps: u64,
    max_coverage_bps: u64,
    max_duration_ms: u64,
    fee_bps: u64,
    ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(min_coverage_bps &gt; 0, <a href="../social_contracts/insurance.md#social_contracts_insurance_EInvalidCoverage">EInvalidCoverage</a>);
    <b>assert</b>!(min_coverage_bps &lt;= max_coverage_bps, <a href="../social_contracts/insurance.md#social_contracts_insurance_EInvalidCoverage">EInvalidCoverage</a>);
    <b>assert</b>!(max_coverage_bps &lt;= <a href="../social_contracts/insurance.md#social_contracts_insurance_BPS_DENOM">BPS_DENOM</a>, <a href="../social_contracts/insurance.md#social_contracts_insurance_EInvalidCoverage">EInvalidCoverage</a>);
    <b>assert</b>!(max_duration_ms &gt; 0, <a href="../social_contracts/insurance.md#social_contracts_insurance_EInvalidDuration">EInvalidDuration</a>);
    <b>assert</b>!(fee_bps &lt;= <a href="../social_contracts/insurance.md#social_contracts_insurance_BPS_DENOM">BPS_DENOM</a>, <a href="../social_contracts/insurance.md#social_contracts_insurance_EInvalidCoverage">EInvalidCoverage</a>);
    <b>let</b> admin = tx_context::sender(ctx);
    <b>let</b> ts = tx_context::epoch_timestamp_ms(ctx);
    transfer::share_object(<a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceConfig">InsuranceConfig</a> {
        id: object::new(ctx),
        enable_flag: <b>false</b>,
        min_coverage_bps,
        max_coverage_bps,
        max_duration_ms,
        fee_bps,
        min_spot_total_liquidity: <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_MIN_SPOT_TOTAL_LIQUIDITY">DEFAULT_MIN_SPOT_TOTAL_LIQUIDITY</a>,
        max_coverage_fraction_of_option_bps: <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_MAX_COVERAGE_FRACTION_OF_OPTION_BPS">DEFAULT_MAX_COVERAGE_FRACTION_OF_OPTION_BPS</a>,
        max_risk_multiplier_bps: <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_MAX_RISK_MULTIPLIER_BPS">DEFAULT_MAX_RISK_MULTIPLIER_BPS</a>,
        min_premium_amount: <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_MIN_PREMIUM_AMOUNT">DEFAULT_MIN_PREMIUM_AMOUNT</a>,
        spot_smoothing_per_option: <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_SPOT_SMOOTHING_PER_OPTION">DEFAULT_SPOT_SMOOTHING_PER_OPTION</a>,
        implied_prob_floor_bps: <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_IMPLIED_PROB_FLOOR_BPS">DEFAULT_IMPLIED_PROB_FLOOR_BPS</a>,
        odds_floor_1x: <b>true</b>,
        odds_cap_bps: <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_ODDS_CAP_BPS">DEFAULT_ODDS_CAP_BPS</a>,
        liq_cap_bps: <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_LIQ_CAP_BPS">DEFAULT_LIQ_CAP_BPS</a>,
        liq_ref_amount: <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_LIQ_REF_AMOUNT">DEFAULT_LIQ_REF_AMOUNT</a>,
        exposure_cap_bps: <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_EXPOSURE_CAP_BPS">DEFAULT_EXPOSURE_CAP_BPS</a>,
        exposure_k_bps: <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_EXPOSURE_K_BPS">DEFAULT_EXPOSURE_K_BPS</a>,
        version: <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_VERSION">DEFAULT_VERSION</a>,
    });
    transfer::share_object(<a href="../social_contracts/insurance.md#social_contracts_insurance_new_router_config_defaults">new_router_config_defaults</a>(ctx));
    transfer::share_object(<a href="../social_contracts/insurance.md#social_contracts_insurance_new_backstop_pool_defaults">new_backstop_pool_defaults</a>(ctx));
    transfer::public_transfer(<a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceAdminCap">InsuranceAdminCap</a> { id: object::new(ctx) }, admin);
    event::emit(<a href="../social_contracts/insurance.md#social_contracts_insurance_ConfigInitializedEvent">ConfigInitializedEvent</a> {
        admin,
        min_coverage_bps,
        max_coverage_bps,
        max_duration_ms,
        fee_bps,
    });
    event::emit(<a href="../social_contracts/insurance.md#social_contracts_insurance_RiskPricingConfigUpdatedEvent">RiskPricingConfigUpdatedEvent</a> {
        updated_by: admin,
        min_spot_total_liquidity: <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_MIN_SPOT_TOTAL_LIQUIDITY">DEFAULT_MIN_SPOT_TOTAL_LIQUIDITY</a>,
        max_coverage_fraction_of_option_bps: <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_MAX_COVERAGE_FRACTION_OF_OPTION_BPS">DEFAULT_MAX_COVERAGE_FRACTION_OF_OPTION_BPS</a>,
        max_risk_multiplier_bps: <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_MAX_RISK_MULTIPLIER_BPS">DEFAULT_MAX_RISK_MULTIPLIER_BPS</a>,
        min_premium_amount: <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_MIN_PREMIUM_AMOUNT">DEFAULT_MIN_PREMIUM_AMOUNT</a>,
        spot_smoothing_per_option: <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_SPOT_SMOOTHING_PER_OPTION">DEFAULT_SPOT_SMOOTHING_PER_OPTION</a>,
        implied_prob_floor_bps: <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_IMPLIED_PROB_FLOOR_BPS">DEFAULT_IMPLIED_PROB_FLOOR_BPS</a>,
        odds_floor_1x: <b>true</b>,
        odds_cap_bps: <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_ODDS_CAP_BPS">DEFAULT_ODDS_CAP_BPS</a>,
        liq_cap_bps: <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_LIQ_CAP_BPS">DEFAULT_LIQ_CAP_BPS</a>,
        liq_ref_amount: <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_LIQ_REF_AMOUNT">DEFAULT_LIQ_REF_AMOUNT</a>,
        exposure_cap_bps: <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_EXPOSURE_CAP_BPS">DEFAULT_EXPOSURE_CAP_BPS</a>,
        exposure_k_bps: <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_EXPOSURE_K_BPS">DEFAULT_EXPOSURE_K_BPS</a>,
        timestamp: ts,
    });
}
</code></pre>



</details>

<a name="social_contracts_insurance_set_config"></a>

## Function `set_config`

Update config (admin only)


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_set_config">set_config</a>(_: &<a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceAdminCap">social_contracts::insurance::InsuranceAdminCap</a>, config: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceConfig">social_contracts::insurance::InsuranceConfig</a>, min_coverage_bps: u64, max_coverage_bps: u64, max_duration_ms: u64, fee_bps: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_set_config">set_config</a>(
    _: &<a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceAdminCap">InsuranceAdminCap</a>,
    config: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceConfig">InsuranceConfig</a>,
    min_coverage_bps: u64,
    max_coverage_bps: u64,
    max_duration_ms: u64,
    fee_bps: u64,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(min_coverage_bps &gt; 0, <a href="../social_contracts/insurance.md#social_contracts_insurance_EInvalidCoverage">EInvalidCoverage</a>);
    <b>assert</b>!(min_coverage_bps &lt;= max_coverage_bps, <a href="../social_contracts/insurance.md#social_contracts_insurance_EInvalidCoverage">EInvalidCoverage</a>);
    <b>assert</b>!(max_coverage_bps &lt;= <a href="../social_contracts/insurance.md#social_contracts_insurance_BPS_DENOM">BPS_DENOM</a>, <a href="../social_contracts/insurance.md#social_contracts_insurance_EInvalidCoverage">EInvalidCoverage</a>);
    <b>assert</b>!(max_duration_ms &gt; 0, <a href="../social_contracts/insurance.md#social_contracts_insurance_EInvalidDuration">EInvalidDuration</a>);
    <b>assert</b>!(fee_bps &lt;= <a href="../social_contracts/insurance.md#social_contracts_insurance_BPS_DENOM">BPS_DENOM</a>, <a href="../social_contracts/insurance.md#social_contracts_insurance_EInvalidCoverage">EInvalidCoverage</a>);
    config.min_coverage_bps = min_coverage_bps;
    config.max_coverage_bps = max_coverage_bps;
    config.max_duration_ms = max_duration_ms;
    config.fee_bps = fee_bps;
    <b>let</b> updated_by = tx_context::sender(ctx);
    <b>let</b> timestamp = clock::timestamp_ms(clock);
    event::emit(<a href="../social_contracts/insurance.md#social_contracts_insurance_ConfigUpdatedEvent">ConfigUpdatedEvent</a> {
        updated_by,
        enable_flag: config.enable_flag,
        min_coverage_bps,
        max_coverage_bps,
        max_duration_ms,
        fee_bps,
        timestamp,
    });
}
</code></pre>



</details>

<a name="social_contracts_insurance_set_risk_pricing_config"></a>

## Function `set_risk_pricing_config`

Update SPoT-linked risk pricing (admin only).


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_set_risk_pricing_config">set_risk_pricing_config</a>(_: &<a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceAdminCap">social_contracts::insurance::InsuranceAdminCap</a>, config: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceConfig">social_contracts::insurance::InsuranceConfig</a>, min_spot_total_liquidity: u64, max_coverage_fraction_of_option_bps: u64, max_risk_multiplier_bps: u64, min_premium_amount: u64, spot_smoothing_per_option: u64, implied_prob_floor_bps: u64, odds_floor_1x: bool, odds_cap_bps: u64, liq_cap_bps: u64, liq_ref_amount: u64, exposure_cap_bps: u64, exposure_k_bps: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_set_risk_pricing_config">set_risk_pricing_config</a>(
    _: &<a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceAdminCap">InsuranceAdminCap</a>,
    config: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceConfig">InsuranceConfig</a>,
    min_spot_total_liquidity: u64,
    max_coverage_fraction_of_option_bps: u64,
    max_risk_multiplier_bps: u64,
    min_premium_amount: u64,
    spot_smoothing_per_option: u64,
    implied_prob_floor_bps: u64,
    odds_floor_1x: bool,
    odds_cap_bps: u64,
    liq_cap_bps: u64,
    liq_ref_amount: u64,
    exposure_cap_bps: u64,
    exposure_k_bps: u64,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(
        max_coverage_fraction_of_option_bps &gt; 0 && max_coverage_fraction_of_option_bps &lt;= <a href="../social_contracts/insurance.md#social_contracts_insurance_BPS_DENOM">BPS_DENOM</a>,
        <a href="../social_contracts/insurance.md#social_contracts_insurance_EInvalidCoverage">EInvalidCoverage</a>
    );
    <b>assert</b>!(
        exposure_cap_bps &gt;= <a href="../social_contracts/insurance.md#social_contracts_insurance_BPS_DENOM">BPS_DENOM</a> && odds_cap_bps &gt;= <a href="../social_contracts/insurance.md#social_contracts_insurance_BPS_DENOM">BPS_DENOM</a> && liq_cap_bps &gt;= <a href="../social_contracts/insurance.md#social_contracts_insurance_BPS_DENOM">BPS_DENOM</a>,
        <a href="../social_contracts/insurance.md#social_contracts_insurance_EInvalidCoverage">EInvalidCoverage</a>
    );
    <b>assert</b>!(
        implied_prob_floor_bps &gt; 0 && implied_prob_floor_bps &lt;= <a href="../social_contracts/insurance.md#social_contracts_insurance_BPS_DENOM">BPS_DENOM</a>,
        <a href="../social_contracts/insurance.md#social_contracts_insurance_EInvalidCoverage">EInvalidCoverage</a>
    );
    <b>assert</b>!(max_risk_multiplier_bps &gt;= <a href="../social_contracts/insurance.md#social_contracts_insurance_BPS_DENOM">BPS_DENOM</a>, <a href="../social_contracts/insurance.md#social_contracts_insurance_EInvalidCoverage">EInvalidCoverage</a>);
    <b>assert</b>!(min_premium_amount &gt; 0, <a href="../social_contracts/insurance.md#social_contracts_insurance_EInvalidAmount">EInvalidAmount</a>);
    config.min_spot_total_liquidity = min_spot_total_liquidity;
    config.max_coverage_fraction_of_option_bps = max_coverage_fraction_of_option_bps;
    config.max_risk_multiplier_bps = max_risk_multiplier_bps;
    config.min_premium_amount = min_premium_amount;
    config.spot_smoothing_per_option = spot_smoothing_per_option;
    config.implied_prob_floor_bps = implied_prob_floor_bps;
    config.odds_floor_1x = odds_floor_1x;
    config.odds_cap_bps = odds_cap_bps;
    config.liq_cap_bps = liq_cap_bps;
    config.liq_ref_amount = liq_ref_amount;
    config.exposure_cap_bps = exposure_cap_bps;
    config.exposure_k_bps = exposure_k_bps;
    event::emit(<a href="../social_contracts/insurance.md#social_contracts_insurance_RiskPricingConfigUpdatedEvent">RiskPricingConfigUpdatedEvent</a> {
        updated_by: tx_context::sender(ctx),
        min_spot_total_liquidity,
        max_coverage_fraction_of_option_bps,
        max_risk_multiplier_bps,
        min_premium_amount,
        spot_smoothing_per_option,
        implied_prob_floor_bps,
        odds_floor_1x,
        odds_cap_bps,
        liq_cap_bps,
        liq_ref_amount,
        exposure_cap_bps,
        exposure_k_bps,
        timestamp: clock::timestamp_ms(clock),
    });
}
</code></pre>



</details>

<a name="social_contracts_insurance_set_enable_flag"></a>

## Function `set_enable_flag`

Emergency enable/disable toggle (admin only)


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_set_enable_flag">set_enable_flag</a>(_: &<a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceAdminCap">social_contracts::insurance::InsuranceAdminCap</a>, config: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceConfig">social_contracts::insurance::InsuranceConfig</a>, enable_flag: bool, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_set_enable_flag">set_enable_flag</a>(
    _: &<a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceAdminCap">InsuranceAdminCap</a>,
    config: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceConfig">InsuranceConfig</a>,
    enable_flag: bool,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    config.enable_flag = enable_flag;
    <b>let</b> updated_by = tx_context::sender(ctx);
    <b>let</b> timestamp = clock::timestamp_ms(clock);
    event::emit(<a href="../social_contracts/insurance.md#social_contracts_insurance_ConfigUpdatedEvent">ConfigUpdatedEvent</a> {
        updated_by,
        enable_flag: config.enable_flag,
        min_coverage_bps: config.min_coverage_bps,
        max_coverage_bps: config.max_coverage_bps,
        max_duration_ms: config.max_duration_ms,
        fee_bps: config.fee_bps,
        timestamp,
    });
}
</code></pre>



</details>

<a name="social_contracts_insurance_create_insurance_admin_cap"></a>

## Function `create_insurance_admin_cap`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_create_insurance_admin_cap">create_insurance_admin_cap</a>(ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceAdminCap">social_contracts::insurance::InsuranceAdminCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_create_insurance_admin_cap">create_insurance_admin_cap</a>(ctx: &<b>mut</b> TxContext): <a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceAdminCap">InsuranceAdminCap</a> {
    <a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceAdminCap">InsuranceAdminCap</a> { id: object::new(ctx) }
}
</code></pre>



</details>

<a name="social_contracts_insurance_bootstrap_init"></a>

## Function `bootstrap_init`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_bootstrap_init">bootstrap_init</a>(ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_bootstrap_init">bootstrap_init</a>(ctx: &<b>mut</b> TxContext) {
    <b>let</b> admin = tx_context::sender(ctx);
    <b>let</b> ts = tx_context::epoch_timestamp_ms(ctx);
    <b>let</b> config = <a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceConfig">InsuranceConfig</a> {
        id: object::new(ctx),
        enable_flag: <b>false</b>,
        min_coverage_bps: <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_MIN_COVERAGE_BPS">DEFAULT_MIN_COVERAGE_BPS</a>,
        max_coverage_bps: <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_MAX_COVERAGE_BPS">DEFAULT_MAX_COVERAGE_BPS</a>,
        max_duration_ms: <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_MAX_DURATION_MS">DEFAULT_MAX_DURATION_MS</a>,
        fee_bps: <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_FEE_BPS">DEFAULT_FEE_BPS</a>,
        min_spot_total_liquidity: <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_MIN_SPOT_TOTAL_LIQUIDITY">DEFAULT_MIN_SPOT_TOTAL_LIQUIDITY</a>,
        max_coverage_fraction_of_option_bps: <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_MAX_COVERAGE_FRACTION_OF_OPTION_BPS">DEFAULT_MAX_COVERAGE_FRACTION_OF_OPTION_BPS</a>,
        max_risk_multiplier_bps: <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_MAX_RISK_MULTIPLIER_BPS">DEFAULT_MAX_RISK_MULTIPLIER_BPS</a>,
        min_premium_amount: <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_MIN_PREMIUM_AMOUNT">DEFAULT_MIN_PREMIUM_AMOUNT</a>,
        spot_smoothing_per_option: <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_SPOT_SMOOTHING_PER_OPTION">DEFAULT_SPOT_SMOOTHING_PER_OPTION</a>,
        implied_prob_floor_bps: <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_IMPLIED_PROB_FLOOR_BPS">DEFAULT_IMPLIED_PROB_FLOOR_BPS</a>,
        odds_floor_1x: <b>true</b>,
        odds_cap_bps: <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_ODDS_CAP_BPS">DEFAULT_ODDS_CAP_BPS</a>,
        liq_cap_bps: <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_LIQ_CAP_BPS">DEFAULT_LIQ_CAP_BPS</a>,
        liq_ref_amount: <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_LIQ_REF_AMOUNT">DEFAULT_LIQ_REF_AMOUNT</a>,
        exposure_cap_bps: <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_EXPOSURE_CAP_BPS">DEFAULT_EXPOSURE_CAP_BPS</a>,
        exposure_k_bps: <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_EXPOSURE_K_BPS">DEFAULT_EXPOSURE_K_BPS</a>,
        version: <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_VERSION">DEFAULT_VERSION</a>,
    };
    transfer::share_object(<a href="../social_contracts/insurance.md#social_contracts_insurance_new_router_config_defaults">new_router_config_defaults</a>(ctx));
    transfer::share_object(<a href="../social_contracts/insurance.md#social_contracts_insurance_new_backstop_pool_defaults">new_backstop_pool_defaults</a>(ctx));
    event::emit(<a href="../social_contracts/insurance.md#social_contracts_insurance_ConfigUpdatedEvent">ConfigUpdatedEvent</a> {
        updated_by: admin,
        enable_flag: <b>false</b>,
        min_coverage_bps: <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_MIN_COVERAGE_BPS">DEFAULT_MIN_COVERAGE_BPS</a>,
        max_coverage_bps: <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_MAX_COVERAGE_BPS">DEFAULT_MAX_COVERAGE_BPS</a>,
        max_duration_ms: <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_MAX_DURATION_MS">DEFAULT_MAX_DURATION_MS</a>,
        fee_bps: <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_FEE_BPS">DEFAULT_FEE_BPS</a>,
        timestamp: ts,
    });
    event::emit(<a href="../social_contracts/insurance.md#social_contracts_insurance_RiskPricingConfigUpdatedEvent">RiskPricingConfigUpdatedEvent</a> {
        updated_by: admin,
        min_spot_total_liquidity: <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_MIN_SPOT_TOTAL_LIQUIDITY">DEFAULT_MIN_SPOT_TOTAL_LIQUIDITY</a>,
        max_coverage_fraction_of_option_bps: <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_MAX_COVERAGE_FRACTION_OF_OPTION_BPS">DEFAULT_MAX_COVERAGE_FRACTION_OF_OPTION_BPS</a>,
        max_risk_multiplier_bps: <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_MAX_RISK_MULTIPLIER_BPS">DEFAULT_MAX_RISK_MULTIPLIER_BPS</a>,
        min_premium_amount: <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_MIN_PREMIUM_AMOUNT">DEFAULT_MIN_PREMIUM_AMOUNT</a>,
        spot_smoothing_per_option: <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_SPOT_SMOOTHING_PER_OPTION">DEFAULT_SPOT_SMOOTHING_PER_OPTION</a>,
        implied_prob_floor_bps: <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_IMPLIED_PROB_FLOOR_BPS">DEFAULT_IMPLIED_PROB_FLOOR_BPS</a>,
        odds_floor_1x: <b>true</b>,
        odds_cap_bps: <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_ODDS_CAP_BPS">DEFAULT_ODDS_CAP_BPS</a>,
        liq_cap_bps: <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_LIQ_CAP_BPS">DEFAULT_LIQ_CAP_BPS</a>,
        liq_ref_amount: <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_LIQ_REF_AMOUNT">DEFAULT_LIQ_REF_AMOUNT</a>,
        exposure_cap_bps: <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_EXPOSURE_CAP_BPS">DEFAULT_EXPOSURE_CAP_BPS</a>,
        exposure_k_bps: <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_EXPOSURE_K_BPS">DEFAULT_EXPOSURE_K_BPS</a>,
        timestamp: ts,
    });
    transfer::share_object(config);
}
</code></pre>



</details>

<a name="social_contracts_insurance_create_vault"></a>

## Function `create_vault`

Create an underwriter vault


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_create_vault">create_vault</a>(base_rate_bps_per_day: u64, utilization_multiplier_bps: u64, max_exposure_per_market: u64, max_exposure_per_user: u64, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_create_vault">create_vault</a>(
    base_rate_bps_per_day: u64,
    utilization_multiplier_bps: u64,
    max_exposure_per_market: u64,
    max_exposure_per_user: u64,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> underwriter = tx_context::sender(ctx);
    <b>let</b> max_exposure_per_option = 0;
    <b>let</b> enabled = <b>true</b>;
    <b>let</b> paused = <b>false</b>;
    <b>let</b> vault = <a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">UnderwriterVault</a> {
        id: object::new(ctx),
        underwriter,
        capital: balance::zero(),
        reserved: 0,
        base_rate_bps_per_day,
        utilization_multiplier_bps,
        max_exposure_per_market,
        max_exposure_per_user,
        max_exposure_per_option,
        enabled,
        paused,
        market_exposures: table::new(ctx),
        user_exposures: table::new(ctx),
        version: <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_VERSION">DEFAULT_VERSION</a>,
    };
    <b>let</b> vault_id = object::id(&vault);
    transfer::share_object(vault);
    event::emit(<a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVaultCreatedEvent">UnderwriterVaultCreatedEvent</a> {
        vault_id,
        underwriter,
        base_rate_bps_per_day,
        utilization_multiplier_bps,
        max_exposure_per_market,
        max_exposure_per_user,
        max_exposure_per_option,
        enabled,
        paused,
    });
}
</code></pre>



</details>

<a name="social_contracts_insurance_set_vault_status"></a>

## Function `set_vault_status`

Underwriter updates vault listing parameters (emit for indexer discovery).


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_set_vault_status">set_vault_status</a>(vault: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">social_contracts::insurance::UnderwriterVault</a>, enabled: bool, paused: bool, max_exposure_per_option: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_set_vault_status">set_vault_status</a>(
    vault: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">UnderwriterVault</a>,
    enabled: bool,
    paused: bool,
    max_exposure_per_option: u64,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(tx_context::sender(ctx) == vault.underwriter, <a href="../social_contracts/insurance.md#social_contracts_insurance_ENotAdmin">ENotAdmin</a>);
    vault.enabled = enabled;
    vault.paused = paused;
    vault.max_exposure_per_option = max_exposure_per_option;
    event::emit(<a href="../social_contracts/insurance.md#social_contracts_insurance_VaultStatusUpdatedEvent">VaultStatusUpdatedEvent</a> {
        vault_id: object::id(vault),
        enabled,
        paused,
        max_exposure_per_option,
        max_exposure_per_market: vault.max_exposure_per_market,
        max_exposure_per_user: vault.max_exposure_per_user,
        base_rate_bps_per_day: vault.base_rate_bps_per_day,
        utilization_multiplier_bps: vault.utilization_multiplier_bps,
        updated_by: tx_context::sender(ctx),
        timestamp_ms: clock::timestamp_ms(clock),
    });
}
</code></pre>



</details>

<a name="social_contracts_insurance_set_router_flags"></a>

## Function `set_router_flags`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_set_router_flags">set_router_flags</a>(_: &<a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceAdminCap">social_contracts::insurance::InsuranceAdminCap</a>, router_cfg: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceRouterConfig">social_contracts::insurance::InsuranceRouterConfig</a>, router_enabled: bool, router_paused: bool, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_set_router_flags">set_router_flags</a>(
    _: &<a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceAdminCap">InsuranceAdminCap</a>,
    router_cfg: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceRouterConfig">InsuranceRouterConfig</a>,
    router_enabled: bool,
    router_paused: bool,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    router_cfg.router_enabled = router_enabled;
    router_cfg.router_paused = router_paused;
    <b>let</b> _ = clock;
    <b>let</b> _ = ctx;
}
</code></pre>



</details>

<a name="social_contracts_insurance_set_router_limits"></a>

## Function `set_router_limits`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_set_router_limits">set_router_limits</a>(_: &<a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceAdminCap">social_contracts::insurance::InsuranceAdminCap</a>, router_cfg: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceRouterConfig">social_contracts::insurance::InsuranceRouterConfig</a>, max_route_reserve_market: u64, max_route_reserve_user: u64, max_route_reserve_option: u64, max_vault_concentration_bps: u64, min_vault_health_factor_bps: u64, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_set_router_limits">set_router_limits</a>(
    _: &<a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceAdminCap">InsuranceAdminCap</a>,
    router_cfg: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceRouterConfig">InsuranceRouterConfig</a>,
    max_route_reserve_market: u64,
    max_route_reserve_user: u64,
    max_route_reserve_option: u64,
    max_vault_concentration_bps: u64,
    min_vault_health_factor_bps: u64,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(
        max_vault_concentration_bps &gt; 0 && max_vault_concentration_bps &lt;= <a href="../social_contracts/insurance.md#social_contracts_insurance_BPS_DENOM">BPS_DENOM</a>,
        <a href="../social_contracts/insurance.md#social_contracts_insurance_EInvalidCoverage">EInvalidCoverage</a>
    );
    <b>assert</b>!(min_vault_health_factor_bps &gt; 0, <a href="../social_contracts/insurance.md#social_contracts_insurance_EInvalidAmount">EInvalidAmount</a>);
    router_cfg.max_route_reserve_market = max_route_reserve_market;
    router_cfg.max_route_reserve_user = max_route_reserve_user;
    router_cfg.max_route_reserve_option = max_route_reserve_option;
    router_cfg.max_vault_concentration_bps = max_vault_concentration_bps;
    router_cfg.min_vault_health_factor_bps = min_vault_health_factor_bps;
    <b>let</b> _ = ctx;
}
</code></pre>



</details>

<a name="social_contracts_insurance_set_market_pause"></a>

## Function `set_market_pause`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_set_market_pause">set_market_pause</a>(_: &<a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceAdminCap">social_contracts::insurance::InsuranceAdminCap</a>, router_cfg: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceRouterConfig">social_contracts::insurance::InsuranceRouterConfig</a>, market_id: <b>address</b>, paused: bool, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_set_market_pause">set_market_pause</a>(
    _: &<a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceAdminCap">InsuranceAdminCap</a>,
    router_cfg: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceRouterConfig">InsuranceRouterConfig</a>,
    market_id: <b>address</b>,
    paused: bool,
    ctx: &<b>mut</b> TxContext,
) {
    <b>if</b> (table::contains(&router_cfg.market_pause, market_id)) {
        *table::borrow_mut(&<b>mut</b> router_cfg.market_pause, market_id) = paused;
    } <b>else</b> {
        table::add(&<b>mut</b> router_cfg.market_pause, market_id, paused);
    };
    <b>let</b> _ = ctx;
}
</code></pre>



</details>

<a name="social_contracts_insurance_set_backstop_caps"></a>

## Function `set_backstop_caps`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_set_backstop_caps">set_backstop_caps</a>(_: &<a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceAdminCap">social_contracts::insurance::InsuranceAdminCap</a>, pool: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceBackstopPool">social_contracts::insurance::InsuranceBackstopPool</a>, max_payout_per_market: u64, max_payout_per_event: u64, global_hard_cap: u64, sweep_premium_bps: u64, tail_pay_partial_on_cap: bool, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_set_backstop_caps">set_backstop_caps</a>(
    _: &<a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceAdminCap">InsuranceAdminCap</a>,
    pool: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceBackstopPool">InsuranceBackstopPool</a>,
    max_payout_per_market: u64,
    max_payout_per_event: u64,
    global_hard_cap: u64,
    sweep_premium_bps: u64,
    tail_pay_partial_on_cap: bool,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(sweep_premium_bps &lt;= <a href="../social_contracts/insurance.md#social_contracts_insurance_BPS_DENOM">BPS_DENOM</a>, <a href="../social_contracts/insurance.md#social_contracts_insurance_EInvalidCoverage">EInvalidCoverage</a>);
    pool.max_payout_per_market = max_payout_per_market;
    pool.max_payout_per_event = max_payout_per_event;
    pool.global_hard_cap = global_hard_cap;
    pool.sweep_premium_bps = sweep_premium_bps;
    pool.tail_pay_partial_on_cap = tail_pay_partial_on_cap;
    <b>let</b> _ = ctx;
}
</code></pre>



</details>

<a name="social_contracts_insurance_set_tail_mode"></a>

## Function `set_tail_mode`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_set_tail_mode">set_tail_mode</a>(_: &<a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceAdminCap">social_contracts::insurance::InsuranceAdminCap</a>, pool: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceBackstopPool">social_contracts::insurance::InsuranceBackstopPool</a>, tail_mode_enabled: bool, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_set_tail_mode">set_tail_mode</a>(
    _: &<a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceAdminCap">InsuranceAdminCap</a>,
    pool: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceBackstopPool">InsuranceBackstopPool</a>,
    tail_mode_enabled: bool,
    ctx: &<b>mut</b> TxContext,
) {
    pool.tail_mode_enabled = tail_mode_enabled;
    <b>let</b> _ = ctx;
}
</code></pre>



</details>

<a name="social_contracts_insurance_set_backstop_paused"></a>

## Function `set_backstop_paused`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_set_backstop_paused">set_backstop_paused</a>(_: &<a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceAdminCap">social_contracts::insurance::InsuranceAdminCap</a>, pool: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceBackstopPool">social_contracts::insurance::InsuranceBackstopPool</a>, paused: bool, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_set_backstop_paused">set_backstop_paused</a>(
    _: &<a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceAdminCap">InsuranceAdminCap</a>,
    pool: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceBackstopPool">InsuranceBackstopPool</a>,
    paused: bool,
    ctx: &<b>mut</b> TxContext,
) {
    pool.paused = paused;
    <b>let</b> _ = ctx;
}
</code></pre>



</details>

<a name="social_contracts_insurance_deposit_backstop_treasury"></a>

## Function `deposit_backstop_treasury`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_deposit_backstop_treasury">deposit_backstop_treasury</a>(_: &<a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceAdminCap">social_contracts::insurance::InsuranceAdminCap</a>, pool: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceBackstopPool">social_contracts::insurance::InsuranceBackstopPool</a>, payment: <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_deposit_backstop_treasury">deposit_backstop_treasury</a>(
    _: &<a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceAdminCap">InsuranceAdminCap</a>,
    pool: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceBackstopPool">InsuranceBackstopPool</a>,
    payment: Coin&lt;MYSO&gt;,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> amt = coin::value(&payment);
    <b>assert</b>!(amt &gt; 0, <a href="../social_contracts/insurance.md#social_contracts_insurance_EInvalidAmount">EInvalidAmount</a>);
    <b>let</b> sender = tx_context::sender(ctx);
    balance::join(&<b>mut</b> pool.capital, coin::into_balance(payment));
    <b>let</b> new_balance = balance::value(&pool.capital);
    event::emit(<a href="../social_contracts/insurance.md#social_contracts_insurance_BackstopTreasuryDepositEvent">BackstopTreasuryDepositEvent</a> {
        depositor: sender,
        amount: amt,
        new_balance,
    });
}
</code></pre>



</details>

<a name="social_contracts_insurance_tail_pay_shortfall"></a>

## Function `tail_pay_shortfall`

Tail shortfall payout only (<code>tail_mode_enabled</code> + caps). Does not interact with <code><a href="../social_contracts/insurance.md#social_contracts_insurance_claim">claim</a></code>.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_tail_pay_shortfall">tail_pay_shortfall</a>(_: &<a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceAdminCap">social_contracts::insurance::InsuranceAdminCap</a>, pool: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceBackstopPool">social_contracts::insurance::InsuranceBackstopPool</a>, config: &<a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceConfig">social_contracts::insurance::InsuranceConfig</a>, recipient: <b>address</b>, market_id: <b>address</b>, amount_requested: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_tail_pay_shortfall">tail_pay_shortfall</a>(
    _: &<a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceAdminCap">InsuranceAdminCap</a>,
    pool: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceBackstopPool">InsuranceBackstopPool</a>,
    config: &<a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceConfig">InsuranceConfig</a>,
    recipient: <b>address</b>,
    market_id: <b>address</b>,
    amount_requested: u64,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(config.enable_flag, <a href="../social_contracts/insurance.md#social_contracts_insurance_EDisabled">EDisabled</a>);
    <b>assert</b>!(pool.tail_mode_enabled, <a href="../social_contracts/insurance.md#social_contracts_insurance_ETailModeDisabled">ETailModeDisabled</a>);
    <b>assert</b>!(!pool.paused, <a href="../social_contracts/insurance.md#social_contracts_insurance_EBackstopPaused">EBackstopPaused</a>);
    <b>assert</b>!(amount_requested &gt; 0, <a href="../social_contracts/insurance.md#social_contracts_insurance_EInvalidAmount">EInvalidAmount</a>);
    <b>let</b> pool_balance = balance::value(&pool.capital);
    <b>let</b> <b>mut</b> paid_market = 0;
    <b>if</b> (table::contains(&pool.paid_by_market, market_id)) {
        paid_market = *table::borrow(&pool.paid_by_market, market_id);
    };
    <b>let</b> remaining_market = <a href="../social_contracts/insurance.md#social_contracts_insurance_min_cap_sub">min_cap_sub</a>(pool.max_payout_per_market, paid_market);
    <b>let</b> remaining_global = <a href="../social_contracts/insurance.md#social_contracts_insurance_min_cap_sub">min_cap_sub</a>(pool.global_hard_cap, pool.total_paid_out);
    <b>let</b> cap_event = pool.max_payout_per_event;
    <b>let</b> pay_cap = <a href="../social_contracts/insurance.md#social_contracts_insurance_min_u64">min_u64</a>(
        <a href="../social_contracts/insurance.md#social_contracts_insurance_min_u64">min_u64</a>(<a href="../social_contracts/insurance.md#social_contracts_insurance_min_u64">min_u64</a>(amount_requested, pool_balance), remaining_market),
        <a href="../social_contracts/insurance.md#social_contracts_insurance_min_u64">min_u64</a>(remaining_global, cap_event),
    );
    <b>if</b> (pay_cap == 0) {
        <b>assert</b>!(pool.tail_pay_partial_on_cap, <a href="../social_contracts/insurance.md#social_contracts_insurance_EBackstopPayoutLimit">EBackstopPayoutLimit</a>);
    } <b>else</b> {
        <b>if</b> (pay_cap &lt; amount_requested) {
            <b>assert</b>!(pool.tail_pay_partial_on_cap, <a href="../social_contracts/insurance.md#social_contracts_insurance_EBackstopPayoutLimit">EBackstopPayoutLimit</a>);
        };
        <b>let</b> pay_bal = balance::split(&<b>mut</b> pool.capital, pay_cap);
        <b>let</b> coin_out = coin::from_balance(pay_bal, ctx);
        transfer::public_transfer(coin_out, recipient);
        pool.total_paid_out = pool.total_paid_out + pay_cap;
        <b>if</b> (table::contains(&pool.paid_by_market, market_id)) {
            <b>let</b> e = table::borrow_mut(&<b>mut</b> pool.paid_by_market, market_id);
            *e = *e + pay_cap;
        } <b>else</b> {
            table::add(&<b>mut</b> pool.paid_by_market, market_id, pay_cap);
        };
        event::emit(<a href="../social_contracts/insurance.md#social_contracts_insurance_BackstopUsedEvent">BackstopUsedEvent</a> {
            market_id,
            recipient,
            amount: pay_cap,
            total_paid_out_after: pool.total_paid_out,
            tail_mode_enabled: pool.tail_mode_enabled,
        });
    };
    <b>let</b> _ = clock;
    <b>let</b> _ = ctx;
}
</code></pre>



</details>

<a name="social_contracts_insurance_min_cap_sub"></a>

## Function `min_cap_sub`



<pre><code><b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_min_cap_sub">min_cap_sub</a>(cap: u64, used: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_min_cap_sub">min_cap_sub</a>(cap: u64, used: u64): u64 {
    <b>if</b> (cap &gt;= used) {
        cap - used
    } <b>else</b> {
        0
    }
}
</code></pre>



</details>

<a name="social_contracts_insurance_min_u64"></a>

## Function `min_u64`



<pre><code><b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_min_u64">min_u64</a>(a: u64, b: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_min_u64">min_u64</a>(a: u64, b: u64): u64 {
    <b>if</b> (a &lt; b) {
        a
    } <b>else</b> {
        b
    }
}
</code></pre>



</details>

<a name="social_contracts_insurance_copy_id_vec"></a>

## Function `copy_id_vec`



<pre><code><b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_copy_id_vec">copy_id_vec</a>(src: &vector&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;): vector&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_copy_id_vec">copy_id_vec</a>(src: &vector&lt;ID&gt;): vector&lt;ID&gt; {
    <b>let</b> <b>mut</b> out = vector::empty();
    <b>let</b> len = vector::length(src);
    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (i &lt; len) {
        vector::push_back(&<b>mut</b> out, *vector::borrow(src, i));
        i = i + 1;
    };
    out
}
</code></pre>



</details>

<a name="social_contracts_insurance_new_router_config_defaults"></a>

## Function `new_router_config_defaults`



<pre><code><b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_new_router_config_defaults">new_router_config_defaults</a>(ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceRouterConfig">social_contracts::insurance::InsuranceRouterConfig</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_new_router_config_defaults">new_router_config_defaults</a>(ctx: &<b>mut</b> TxContext): <a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceRouterConfig">InsuranceRouterConfig</a> {
    <a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceRouterConfig">InsuranceRouterConfig</a> {
        id: object::new(ctx),
        router_enabled: <b>true</b>,
        router_paused: <b>false</b>,
        max_route_reserve_market: 0,
        max_route_reserve_user: 0,
        max_route_reserve_option: 0,
        max_vault_concentration_bps: <a href="../social_contracts/insurance.md#social_contracts_insurance_BPS_DENOM">BPS_DENOM</a>,
        min_vault_health_factor_bps: <a href="../social_contracts/insurance.md#social_contracts_insurance_BPS_DENOM">BPS_DENOM</a>,
        market_pause: table::new(ctx),
        version: <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_VERSION">DEFAULT_VERSION</a>,
    }
}
</code></pre>



</details>

<a name="social_contracts_insurance_new_backstop_pool_defaults"></a>

## Function `new_backstop_pool_defaults`



<pre><code><b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_new_backstop_pool_defaults">new_backstop_pool_defaults</a>(ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceBackstopPool">social_contracts::insurance::InsuranceBackstopPool</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_new_backstop_pool_defaults">new_backstop_pool_defaults</a>(ctx: &<b>mut</b> TxContext): <a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceBackstopPool">InsuranceBackstopPool</a> {
    <a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceBackstopPool">InsuranceBackstopPool</a> {
        id: object::new(ctx),
        capital: balance::zero(),
        total_paid_out: 0,
        paid_by_market: table::new(ctx),
        max_payout_per_market: <a href="../social_contracts/insurance.md#social_contracts_insurance_MAX_U64">MAX_U64</a>,
        max_payout_per_event: <a href="../social_contracts/insurance.md#social_contracts_insurance_MAX_U64">MAX_U64</a>,
        global_hard_cap: <a href="../social_contracts/insurance.md#social_contracts_insurance_MAX_U64">MAX_U64</a>,
        tail_mode_enabled: <b>false</b>,
        paused: <b>false</b>,
        sweep_premium_bps: 0,
        tail_pay_partial_on_cap: <b>true</b>,
        version: <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_VERSION">DEFAULT_VERSION</a>,
    }
}
</code></pre>



</details>

<a name="social_contracts_insurance_assert_market_router_open"></a>

## Function `assert_market_router_open`



<pre><code><b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_assert_market_router_open">assert_market_router_open</a>(router_cfg: &<a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceRouterConfig">social_contracts::insurance::InsuranceRouterConfig</a>, market_id: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_assert_market_router_open">assert_market_router_open</a>(router_cfg: &<a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceRouterConfig">InsuranceRouterConfig</a>, market_id: <b>address</b>) {
    <b>if</b> (table::contains(&router_cfg.market_pause, market_id)) {
        <b>assert</b>!(!*table::borrow(&router_cfg.market_pause, market_id), <a href="../social_contracts/insurance.md#social_contracts_insurance_EMarketClosed">EMarketClosed</a>);
    };
}
</code></pre>



</details>

<a name="social_contracts_insurance_assert_vault_buy_guards"></a>

## Function `assert_vault_buy_guards`



<pre><code><b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_assert_vault_buy_guards">assert_vault_buy_guards</a>(vault: &<a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">social_contracts::insurance::UnderwriterVault</a>, router_cfg: &<a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceRouterConfig">social_contracts::insurance::InsuranceRouterConfig</a>, check_health: bool)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_assert_vault_buy_guards">assert_vault_buy_guards</a>(
    vault: &<a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">UnderwriterVault</a>,
    router_cfg: &<a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceRouterConfig">InsuranceRouterConfig</a>,
    check_health: bool,
) {
    <b>assert</b>!(vault.enabled, <a href="../social_contracts/insurance.md#social_contracts_insurance_EVaultDisabled">EVaultDisabled</a>);
    <b>assert</b>!(!vault.paused, <a href="../social_contracts/insurance.md#social_contracts_insurance_EVaultPaused">EVaultPaused</a>);
    <b>if</b> (check_health) {
        <b>let</b> cap = balance::value(&vault.capital);
        <b>let</b> r = vault.reserved;
        <b>assert</b>!(cap * <a href="../social_contracts/insurance.md#social_contracts_insurance_BPS_DENOM">BPS_DENOM</a> &gt;= r * router_cfg.min_vault_health_factor_bps, <a href="../social_contracts/insurance.md#social_contracts_insurance_EInsufficientCapital">EInsufficientCapital</a>);
    };
}
</code></pre>



</details>

<a name="social_contracts_insurance_deposit_capital"></a>

## Function `deposit_capital`

Deposit capital into vault


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_deposit_capital">deposit_capital</a>(config: &<a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceConfig">social_contracts::insurance::InsuranceConfig</a>, vault: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">social_contracts::insurance::UnderwriterVault</a>, payment: <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_deposit_capital">deposit_capital</a>(
    config: &<a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceConfig">InsuranceConfig</a>,
    vault: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">UnderwriterVault</a>,
    payment: Coin&lt;MYSO&gt;,
    ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(config.enable_flag, <a href="../social_contracts/insurance.md#social_contracts_insurance_EDisabled">EDisabled</a>);
    <b>let</b> deposit_amount = coin::value(&payment);
    <b>assert</b>!(deposit_amount &gt; 0, <a href="../social_contracts/insurance.md#social_contracts_insurance_EInvalidAmount">EInvalidAmount</a>);
    balance::join(&<b>mut</b> vault.capital, coin::into_balance(payment));
    event::emit(<a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVaultDepositedEvent">UnderwriterVaultDepositedEvent</a> {
        vault_id: object::id(vault),
        amount: deposit_amount,
        new_balance: balance::value(&vault.capital),
    });
}
</code></pre>



</details>

<a name="social_contracts_insurance_withdraw_capital"></a>

## Function `withdraw_capital`

Withdraw unreserved capital (underwriter only)


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_withdraw_capital">withdraw_capital</a>(config: &<a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceConfig">social_contracts::insurance::InsuranceConfig</a>, vault: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">social_contracts::insurance::UnderwriterVault</a>, amount: u64, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_withdraw_capital">withdraw_capital</a>(
    config: &<a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceConfig">InsuranceConfig</a>,
    vault: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">UnderwriterVault</a>,
    amount: u64,
    ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(config.enable_flag, <a href="../social_contracts/insurance.md#social_contracts_insurance_EDisabled">EDisabled</a>);
    <b>assert</b>!(tx_context::sender(ctx) == vault.underwriter, <a href="../social_contracts/insurance.md#social_contracts_insurance_ENotAdmin">ENotAdmin</a>);
    <b>assert</b>!(amount &gt; 0, <a href="../social_contracts/insurance.md#social_contracts_insurance_EInvalidAmount">EInvalidAmount</a>);
    <b>let</b> capital_value = balance::value(&vault.capital);
    <b>assert</b>!(capital_value &gt;= vault.reserved, <a href="../social_contracts/insurance.md#social_contracts_insurance_EOverflow">EOverflow</a>);
    <b>let</b> free_capital = capital_value - vault.reserved;
    <b>assert</b>!(free_capital &gt;= amount, <a href="../social_contracts/insurance.md#social_contracts_insurance_EInsufficientCapital">EInsufficientCapital</a>);
    <b>let</b> payout_balance = balance::split(&<b>mut</b> vault.capital, amount);
    <b>let</b> payout_coin = coin::from_balance(payout_balance, ctx);
    transfer::public_transfer(payout_coin, vault.underwriter);
    event::emit(<a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVaultWithdrawnEvent">UnderwriterVaultWithdrawnEvent</a> {
        vault_id: object::id(vault),
        amount,
        new_balance: balance::value(&vault.capital),
    });
}
</code></pre>



</details>

<a name="social_contracts_insurance_premium_quote_premium"></a>

## Function `premium_quote_premium`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_premium_quote_premium">premium_quote_premium</a>(q: &<a href="../social_contracts/insurance.md#social_contracts_insurance_PremiumQuote">social_contracts::insurance::PremiumQuote</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_premium_quote_premium">premium_quote_premium</a>(q: &<a href="../social_contracts/insurance.md#social_contracts_insurance_PremiumQuote">PremiumQuote</a>): u64 {
    q.premium
}
</code></pre>



</details>

<a name="social_contracts_insurance_premium_quote_implied_prob_win_bps"></a>

## Function `premium_quote_implied_prob_win_bps`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_premium_quote_implied_prob_win_bps">premium_quote_implied_prob_win_bps</a>(q: &<a href="../social_contracts/insurance.md#social_contracts_insurance_PremiumQuote">social_contracts::insurance::PremiumQuote</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_premium_quote_implied_prob_win_bps">premium_quote_implied_prob_win_bps</a>(q: &<a href="../social_contracts/insurance.md#social_contracts_insurance_PremiumQuote">PremiumQuote</a>): u64 {
    q.implied_prob_win_bps
}
</code></pre>



</details>

<a name="social_contracts_insurance_premium_quote_risk_multiplier_bps"></a>

## Function `premium_quote_risk_multiplier_bps`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_premium_quote_risk_multiplier_bps">premium_quote_risk_multiplier_bps</a>(q: &<a href="../social_contracts/insurance.md#social_contracts_insurance_PremiumQuote">social_contracts::insurance::PremiumQuote</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_premium_quote_risk_multiplier_bps">premium_quote_risk_multiplier_bps</a>(q: &<a href="../social_contracts/insurance.md#social_contracts_insurance_PremiumQuote">PremiumQuote</a>): u64 {
    q.risk_multiplier_bps
}
</code></pre>



</details>

<a name="social_contracts_insurance_premium_quote_premium_raw"></a>

## Function `premium_quote_premium_raw`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_premium_quote_premium_raw">premium_quote_premium_raw</a>(q: &<a href="../social_contracts/insurance.md#social_contracts_insurance_PremiumQuote">social_contracts::insurance::PremiumQuote</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_premium_quote_premium_raw">premium_quote_premium_raw</a>(q: &<a href="../social_contracts/insurance.md#social_contracts_insurance_PremiumQuote">PremiumQuote</a>): u64 {
    q.premium_raw
}
</code></pre>



</details>

<a name="social_contracts_insurance_quote_base_premium"></a>

## Function `quote_base_premium`



<pre><code><b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_quote_base_premium">quote_base_premium</a>(vault: &<a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">social_contracts::insurance::UnderwriterVault</a>, covered_amount: u64, coverage_bps: u64, duration_ms: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_quote_base_premium">quote_base_premium</a>(
    vault: &<a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">UnderwriterVault</a>,
    covered_amount: u64,
    coverage_bps: u64,
    duration_ms: u64
): u64 {
    <b>let</b> capital_value = balance::value(&vault.capital);
    <b>let</b> utilization_bps = <b>if</b> (capital_value == 0) {
        <a href="../social_contracts/insurance.md#social_contracts_insurance_BPS_DENOM">BPS_DENOM</a>
    } <b>else</b> {
        <b>let</b> utilization_u128 = (vault.reserved <b>as</b> u128) * (<a href="../social_contracts/insurance.md#social_contracts_insurance_BPS_DENOM">BPS_DENOM</a> <b>as</b> u128) / (capital_value <b>as</b> u128);
        <b>assert</b>!(utilization_u128 &lt;= (<a href="../social_contracts/insurance.md#social_contracts_insurance_MAX_U64">MAX_U64</a> <b>as</b> u128), <a href="../social_contracts/insurance.md#social_contracts_insurance_EOverflow">EOverflow</a>);
        utilization_u128 <b>as</b> u64
    };
    <b>let</b> utilization_factor = (utilization_bps * vault.utilization_multiplier_bps) / <a href="../social_contracts/insurance.md#social_contracts_insurance_BPS_DENOM">BPS_DENOM</a>;
    <b>let</b> total_rate_bps_per_day = vault.base_rate_bps_per_day + utilization_factor;
    <b>let</b> numerator = (covered_amount <b>as</b> u128)
        * (coverage_bps <b>as</b> u128)
        * (total_rate_bps_per_day <b>as</b> u128)
        * (duration_ms <b>as</b> u128);
    <b>let</b> denominator = (<a href="../social_contracts/insurance.md#social_contracts_insurance_BPS_DENOM">BPS_DENOM</a> <b>as</b> u128) * (<a href="../social_contracts/insurance.md#social_contracts_insurance_BPS_DENOM">BPS_DENOM</a> <b>as</b> u128) * (<a href="../social_contracts/insurance.md#social_contracts_insurance_DAY_MS">DAY_MS</a> <b>as</b> u128);
    <b>let</b> premium_u128 = numerator / denominator;
    <b>assert</b>!(premium_u128 &lt;= (<a href="../social_contracts/insurance.md#social_contracts_insurance_MAX_U64">MAX_U64</a> <b>as</b> u128), <a href="../social_contracts/insurance.md#social_contracts_insurance_EOverflow">EOverflow</a>);
    premium_u128 <b>as</b> u64
}
</code></pre>



</details>

<a name="social_contracts_insurance_quote_premium"></a>

## Function `quote_premium`

Utilization curve only (<code><a href="../social_contracts/insurance.md#social_contracts_insurance_quote_base_premium">quote_base_premium</a></code>).


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_quote_premium">quote_premium</a>(vault: &<a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">social_contracts::insurance::UnderwriterVault</a>, covered_amount: u64, coverage_bps: u64, duration_ms: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_quote_premium">quote_premium</a>(
    vault: &<a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">UnderwriterVault</a>,
    covered_amount: u64,
    coverage_bps: u64,
    duration_ms: u64
): u64 {
    <a href="../social_contracts/insurance.md#social_contracts_insurance_quote_base_premium">quote_base_premium</a>(vault, covered_amount, coverage_bps, duration_ms)
}
</code></pre>



</details>

<a name="social_contracts_insurance_get_market_option_reserved"></a>

## Function `get_market_option_reserved`



<pre><code><b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_get_market_option_reserved">get_market_option_reserved</a>(vault: &<a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">social_contracts::insurance::UnderwriterVault</a>, market_id: <b>address</b>, option_id: u8): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_get_market_option_reserved">get_market_option_reserved</a>(vault: &<a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">UnderwriterVault</a>, market_id: <b>address</b>, option_id: u8): u64 {
    <b>if</b> (!table::contains(&vault.market_exposures, market_id)) {
        0
    } <b>else</b> {
        <b>let</b> exposure = table::borrow(&vault.market_exposures, market_id);
        <a href="../social_contracts/insurance.md#social_contracts_insurance_get_option_reserved">get_option_reserved</a>(exposure, option_id)
    }
}
</code></pre>



</details>

<a name="social_contracts_insurance_compute_spot_risk_quote"></a>

## Function `compute_spot_risk_quote`



<pre><code><b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_compute_spot_risk_quote">compute_spot_risk_quote</a>(config: &<a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceConfig">social_contracts::insurance::InsuranceConfig</a>, vault: &<a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">social_contracts::insurance::UnderwriterVault</a>, record: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotRecord">social_contracts::social_proof_of_truth::SpotRecord</a>, vault_market_id: <b>address</b>, option_id: u8, covered_amount: u64, coverage_bps: u64, duration_ms: u64, enforce_max_risk: bool): <a href="../social_contracts/insurance.md#social_contracts_insurance_PremiumQuote">social_contracts::insurance::PremiumQuote</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_compute_spot_risk_quote">compute_spot_risk_quote</a>(
    config: &<a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceConfig">InsuranceConfig</a>,
    vault: &<a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">UnderwriterVault</a>,
    record: &spot::SpotRecord,
    vault_market_id: <b>address</b>,
    option_id: u8,
    covered_amount: u64,
    coverage_bps: u64,
    duration_ms: u64,
    enforce_max_risk: bool,
): <a href="../social_contracts/insurance.md#social_contracts_insurance_PremiumQuote">PremiumQuote</a> {
    spot::assert_valid_option_id(record, option_id);
    <b>let</b> t_total = spot::total_option_escrow(record);
    <b>assert</b>!(t_total &gt;= config.min_spot_total_liquidity, <a href="../social_contracts/insurance.md#social_contracts_insurance_EThinMarket">EThinMarket</a>);
    <b>let</b> a_opt = spot::get_option_escrow(record, option_id);
    <b>let</b> denom_cov = <b>if</b> (a_opt &gt;= 1) {
        a_opt
    } <b>else</b> {
        1
    };
    <b>assert</b>!(
        (covered_amount <b>as</b> u128) * (<a href="../social_contracts/insurance.md#social_contracts_insurance_BPS_DENOM">BPS_DENOM</a> <b>as</b> u128)
            &lt;= (config.max_coverage_fraction_of_option_bps <b>as</b> u128) * (denom_cov <b>as</b> u128),
        <a href="../social_contracts/insurance.md#social_contracts_insurance_ECoverageTooLargeVersusPool">ECoverageTooLargeVersusPool</a>
    );
    <b>let</b> n_opts = spot::num_betting_options(record);
    <b>let</b> w = config.spot_smoothing_per_option;
    <b>let</b> nw_u128 = (n_opts <b>as</b> u128) * (w <b>as</b> u128);
    <b>assert</b>!(nw_u128 &lt;= (<a href="../social_contracts/insurance.md#social_contracts_insurance_MAX_U64">MAX_U64</a> <b>as</b> u128), <a href="../social_contracts/insurance.md#social_contracts_insurance_EOverflow">EOverflow</a>);
    <b>let</b> smoothed_t = (t_total <b>as</b> u128) + nw_u128;
    <b>assert</b>!(smoothed_t &gt; 0 && smoothed_t &lt;= (<a href="../social_contracts/insurance.md#social_contracts_insurance_MAX_U64">MAX_U64</a> <b>as</b> u128) * 2, <a href="../social_contracts/insurance.md#social_contracts_insurance_EOverflow">EOverflow</a>);
    <b>let</b> smoothed_a = (a_opt <b>as</b> u128) + (w <b>as</b> u128);
    <b>let</b> p_win_u128 = (smoothed_a * (<a href="../social_contracts/insurance.md#social_contracts_insurance_BPS_DENOM">BPS_DENOM</a> <b>as</b> u128)) / smoothed_t;
    <b>assert</b>!(p_win_u128 &lt;= (<a href="../social_contracts/insurance.md#social_contracts_insurance_MAX_U64">MAX_U64</a> <b>as</b> u128), <a href="../social_contracts/insurance.md#social_contracts_insurance_EOverflow">EOverflow</a>);
    <b>let</b> p_win_bps = p_win_u128 <b>as</b> u64;
    <b>let</b> reserved_opt = <a href="../social_contracts/insurance.md#social_contracts_insurance_get_market_option_reserved">get_market_option_reserved</a>(vault, vault_market_id, option_id);
    <b>let</b> p_floor = config.implied_prob_floor_bps;
    <b>let</b> denom_p = <b>if</b> (p_win_bps &gt; p_floor) { p_win_bps } <b>else</b> { p_floor };
    <b>let</b> odds_core_u128 = (5000u128) * (<a href="../social_contracts/insurance.md#social_contracts_insurance_BPS_DENOM">BPS_DENOM</a> <b>as</b> u128) / (denom_p <b>as</b> u128);
    <b>assert</b>!(odds_core_u128 &lt;= (<a href="../social_contracts/insurance.md#social_contracts_insurance_MAX_U64">MAX_U64</a> <b>as</b> u128), <a href="../social_contracts/insurance.md#social_contracts_insurance_EOverflow">EOverflow</a>);
    <b>let</b> odds_core = odds_core_u128 <b>as</b> u64;
    <b>let</b> <b>mut</b> odds_mult_bps = <b>if</b> (config.odds_cap_bps &lt; odds_core) {
        config.odds_cap_bps
    } <b>else</b> {
        odds_core
    };
    <b>if</b> (config.odds_floor_1x && odds_mult_bps &lt; <a href="../social_contracts/insurance.md#social_contracts_insurance_BPS_DENOM">BPS_DENOM</a>) {
        odds_mult_bps = <a href="../social_contracts/insurance.md#social_contracts_insurance_BPS_DENOM">BPS_DENOM</a>;
    };
    <b>let</b> t_for_liq = <b>if</b> (t_total &gt;= 1) {
        t_total
    } <b>else</b> {
        1
    };
    <b>let</b> liq_uncapped_u128 = (config.liq_ref_amount <b>as</b> u128)
        * (<a href="../social_contracts/insurance.md#social_contracts_insurance_BPS_DENOM">BPS_DENOM</a> <b>as</b> u128) / (t_for_liq <b>as</b> u128);
    <b>assert</b>!(liq_uncapped_u128 &lt;= (<a href="../social_contracts/insurance.md#social_contracts_insurance_MAX_U64">MAX_U64</a> <b>as</b> u128), <a href="../social_contracts/insurance.md#social_contracts_insurance_EOverflow">EOverflow</a>);
    <b>let</b> liq_uncapped = liq_uncapped_u128 <b>as</b> u64;
    <b>let</b> liq_mult_bps = <b>if</b> (config.liq_cap_bps &lt; liq_uncapped) {
        config.liq_cap_bps
    } <b>else</b> {
        liq_uncapped
    };
    <b>let</b> extra_num_u128 = (config.exposure_k_bps <b>as</b> u128) * (reserved_opt <b>as</b> u128) / (denom_cov <b>as</b> u128);
    <b>assert</b>!(extra_num_u128 &lt;= (<a href="../social_contracts/insurance.md#social_contracts_insurance_MAX_U64">MAX_U64</a> <b>as</b> u128), <a href="../social_contracts/insurance.md#social_contracts_insurance_EOverflow">EOverflow</a>);
    <b>let</b> extra_term = extra_num_u128 <b>as</b> u64;
    <b>let</b> max_extra_bps = config.exposure_cap_bps - <a href="../social_contracts/insurance.md#social_contracts_insurance_BPS_DENOM">BPS_DENOM</a>;
    <b>let</b> extra_bounded = <b>if</b> (extra_term &gt; max_extra_bps) {
        max_extra_bps
    } <b>else</b> {
        extra_term
    };
    <b>assert</b>!(extra_bounded &lt;= <a href="../social_contracts/insurance.md#social_contracts_insurance_MAX_U64">MAX_U64</a> - <a href="../social_contracts/insurance.md#social_contracts_insurance_BPS_DENOM">BPS_DENOM</a>, <a href="../social_contracts/insurance.md#social_contracts_insurance_EOverflow">EOverflow</a>);
    <b>let</b> exposure_mult_bps = <a href="../social_contracts/insurance.md#social_contracts_insurance_BPS_DENOM">BPS_DENOM</a> + extra_bounded;
    <b>let</b> risk_u128 =
        ((odds_mult_bps <b>as</b> u128) * (liq_mult_bps <b>as</b> u128) * (exposure_mult_bps <b>as</b> u128))
            / (<a href="../social_contracts/insurance.md#social_contracts_insurance_BPS_DENOM">BPS_DENOM</a> <b>as</b> u128) / (<a href="../social_contracts/insurance.md#social_contracts_insurance_BPS_DENOM">BPS_DENOM</a> <b>as</b> u128);
    <b>assert</b>!(risk_u128 &lt;= (<a href="../social_contracts/insurance.md#social_contracts_insurance_MAX_U64">MAX_U64</a> <b>as</b> u128), <a href="../social_contracts/insurance.md#social_contracts_insurance_EOverflow">EOverflow</a>);
    <b>let</b> risk_multiplier_bps = risk_u128 <b>as</b> u64;
    <b>if</b> (enforce_max_risk) {
        <b>assert</b>!(risk_multiplier_bps &lt;= config.max_risk_multiplier_bps, <a href="../social_contracts/insurance.md#social_contracts_insurance_ERiskMultiplierTooHigh">ERiskMultiplierTooHigh</a>);
    };
    <b>let</b> base_premium =
        <a href="../social_contracts/insurance.md#social_contracts_insurance_quote_base_premium">quote_base_premium</a>(vault, covered_amount, coverage_bps, duration_ms);
    <b>let</b> premium_raw_u128 = ((base_premium <b>as</b> u128) * (risk_multiplier_bps <b>as</b> u128))
        / (<a href="../social_contracts/insurance.md#social_contracts_insurance_BPS_DENOM">BPS_DENOM</a> <b>as</b> u128);
    <b>assert</b>!(premium_raw_u128 &lt;= (<a href="../social_contracts/insurance.md#social_contracts_insurance_MAX_U64">MAX_U64</a> <b>as</b> u128), <a href="../social_contracts/insurance.md#social_contracts_insurance_EOverflow">EOverflow</a>);
    <b>let</b> premium_raw = premium_raw_u128 <b>as</b> u64;
    <b>let</b> premium = <b>if</b> (premium_raw &gt;= config.min_premium_amount) {
        premium_raw
    } <b>else</b> {
        config.min_premium_amount
    };
    <b>assert</b>!(premium &gt; 0, <a href="../social_contracts/insurance.md#social_contracts_insurance_EInsufficientPremium">EInsufficientPremium</a>);
    <a href="../social_contracts/insurance.md#social_contracts_insurance_PremiumQuote">PremiumQuote</a> {
        premium,
        premium_raw,
        implied_prob_win_bps: p_win_bps,
        risk_multiplier_bps,
        market_total_amount: t_total,
        option_amount: a_opt,
        base_premium,
    }
}
</code></pre>



</details>

<a name="social_contracts_insurance_quote_premium_with_spot_risk"></a>

## Function `quote_premium_with_spot_risk`

Preview premium with SPoT pool odds, liquidity, and vault concentration on this option (<code>reserved</code> excludes a not-yet-open policy).


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_quote_premium_with_spot_risk">quote_premium_with_spot_risk</a>(config: &<a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceConfig">social_contracts::insurance::InsuranceConfig</a>, vault: &<a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">social_contracts::insurance::UnderwriterVault</a>, record: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotRecord">social_contracts::social_proof_of_truth::SpotRecord</a>, option_id: u8, covered_amount: u64, coverage_bps: u64, duration_ms: u64): <a href="../social_contracts/insurance.md#social_contracts_insurance_PremiumQuote">social_contracts::insurance::PremiumQuote</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_quote_premium_with_spot_risk">quote_premium_with_spot_risk</a>(
    config: &<a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceConfig">InsuranceConfig</a>,
    vault: &<a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">UnderwriterVault</a>,
    record: &spot::SpotRecord,
    option_id: u8,
    covered_amount: u64,
    coverage_bps: u64,
    duration_ms: u64,
): <a href="../social_contracts/insurance.md#social_contracts_insurance_PremiumQuote">PremiumQuote</a> {
    <b>let</b> market_id = spot::get_id_address(record);
    <a href="../social_contracts/insurance.md#social_contracts_insurance_compute_spot_risk_quote">compute_spot_risk_quote</a>(
        config,
        vault,
        record,
        market_id,
        option_id,
        covered_amount,
        coverage_bps,
        duration_ms,
        <b>true</b>,
    )
}
</code></pre>



</details>

<a name="social_contracts_insurance_coverage_quote_skipped"></a>

## Function `coverage_quote_skipped`



<pre><code><b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_coverage_quote_skipped">coverage_quote_skipped</a>(reason: u8): <a href="../social_contracts/insurance.md#social_contracts_insurance_VaultCoverageQuote">social_contracts::insurance::VaultCoverageQuote</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_coverage_quote_skipped">coverage_quote_skipped</a>(reason: u8): <a href="../social_contracts/insurance.md#social_contracts_insurance_VaultCoverageQuote">VaultCoverageQuote</a> {
    <a href="../social_contracts/insurance.md#social_contracts_insurance_VaultCoverageQuote">VaultCoverageQuote</a> {
        premium: 0,
        premium_raw: 0,
        reserve_required: 0,
        available_capacity_reserve: 0,
        risk_multiplier_bps: 0,
        implied_prob_win_bps: 0,
        utilization_bps: 0,
        max_fill_covered_amount: 0,
        skipped_reason: reason,
    }
}
</code></pre>



</details>

<a name="social_contracts_insurance_max_fill_covered_for_vault"></a>

## Function `max_fill_covered_for_vault`



<pre><code><b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_max_fill_covered_for_vault">max_fill_covered_for_vault</a>(config: &<a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceConfig">social_contracts::insurance::InsuranceConfig</a>, vault: &<a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">social_contracts::insurance::UnderwriterVault</a>, record: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotRecord">social_contracts::social_proof_of_truth::SpotRecord</a>, market_id: <b>address</b>, insured: <b>address</b>, option_id: u8, coverage_bps: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_max_fill_covered_for_vault">max_fill_covered_for_vault</a>(
    config: &<a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceConfig">InsuranceConfig</a>,
    vault: &<a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">UnderwriterVault</a>,
    record: &spot::SpotRecord,
    market_id: <b>address</b>,
    insured: <b>address</b>,
    option_id: u8,
    coverage_bps: u64,
): u64 {
    <b>let</b> position = spot::get_user_option_amount(record, insured, option_id);
    <b>let</b> a_opt = spot::get_option_escrow(record, option_id);
    <b>let</b> denom_cov = <b>if</b> (a_opt &gt;= 1) {
        a_opt
    } <b>else</b> {
        1
    };
    <b>let</b> pool_max_u128 =
        (config.max_coverage_fraction_of_option_bps <b>as</b> u128) * (denom_cov <b>as</b> u128) / (<a href="../social_contracts/insurance.md#social_contracts_insurance_BPS_DENOM">BPS_DENOM</a> <b>as</b> u128);
    <b>assert</b>!(pool_max_u128 &lt;= (<a href="../social_contracts/insurance.md#social_contracts_insurance_MAX_U64">MAX_U64</a> <b>as</b> u128), <a href="../social_contracts/insurance.md#social_contracts_insurance_EOverflow">EOverflow</a>);
    <b>let</b> pool_max = pool_max_u128 <b>as</b> u64;
    <b>let</b> capital_value = balance::value(&vault.capital);
    <b>let</b> free_capital = <b>if</b> (capital_value &gt;= vault.reserved) {
        capital_value - vault.reserved
    } <b>else</b> {
        0
    };
    <b>let</b> vault_cov_max = <b>if</b> (coverage_bps &gt; 0) {
        <b>let</b> v = (free_capital <b>as</b> u128) * (<a href="../social_contracts/insurance.md#social_contracts_insurance_BPS_DENOM">BPS_DENOM</a> <b>as</b> u128) / (coverage_bps <b>as</b> u128);
        <b>assert</b>!(v &lt;= (<a href="../social_contracts/insurance.md#social_contracts_insurance_MAX_U64">MAX_U64</a> <b>as</b> u128), <a href="../social_contracts/insurance.md#social_contracts_insurance_EOverflow">EOverflow</a>);
        v <b>as</b> u64
    } <b>else</b> {
        <a href="../social_contracts/insurance.md#social_contracts_insurance_MAX_U64">MAX_U64</a>
    };
    <b>let</b> <b>mut</b> market_head_reserve = <a href="../social_contracts/insurance.md#social_contracts_insurance_MAX_U64">MAX_U64</a>;
    <b>if</b> (vault.max_exposure_per_market &gt; 0) {
        <b>let</b> tr = <b>if</b> (table::contains(&vault.market_exposures, market_id)) {
            table::borrow(&vault.market_exposures, market_id).total_reserved
        } <b>else</b> {
            0
        };
        market_head_reserve = <a href="../social_contracts/insurance.md#social_contracts_insurance_min_cap_sub">min_cap_sub</a>(vault.max_exposure_per_market, tr);
    };
    <b>let</b> market_cov_max = <a href="../social_contracts/insurance.md#social_contracts_insurance_reserve_to_covered">reserve_to_covered</a>(market_head_reserve, coverage_bps);
    <b>let</b> <b>mut</b> opt_head_reserve = <a href="../social_contracts/insurance.md#social_contracts_insurance_MAX_U64">MAX_U64</a>;
    <b>if</b> (vault.max_exposure_per_option &gt; 0) {
        <b>let</b> ors = <a href="../social_contracts/insurance.md#social_contracts_insurance_get_market_option_reserved">get_market_option_reserved</a>(vault, market_id, option_id);
        opt_head_reserve = <a href="../social_contracts/insurance.md#social_contracts_insurance_min_cap_sub">min_cap_sub</a>(vault.max_exposure_per_option, ors);
    };
    <b>let</b> opt_cov_max = <a href="../social_contracts/insurance.md#social_contracts_insurance_reserve_to_covered">reserve_to_covered</a>(opt_head_reserve, coverage_bps);
    <b>let</b> <b>mut</b> user_head_reserve = <a href="../social_contracts/insurance.md#social_contracts_insurance_MAX_U64">MAX_U64</a>;
    <b>if</b> (vault.max_exposure_per_user &gt; 0) {
        <b>let</b> ue = <a href="../social_contracts/insurance.md#social_contracts_insurance_get_user_exposure">get_user_exposure</a>(vault, insured);
        user_head_reserve = <a href="../social_contracts/insurance.md#social_contracts_insurance_min_cap_sub">min_cap_sub</a>(vault.max_exposure_per_user, ue);
    };
    <b>let</b> user_cov_max = <a href="../social_contracts/insurance.md#social_contracts_insurance_reserve_to_covered">reserve_to_covered</a>(user_head_reserve, coverage_bps);
    <b>let</b> <b>mut</b> m = position;
    <b>if</b> (pool_max &lt; m) {
        m = pool_max
    };
    <b>if</b> (vault_cov_max &lt; m) {
        m = vault_cov_max
    };
    <b>if</b> (market_cov_max &lt; m) {
        m = market_cov_max
    };
    <b>if</b> (opt_cov_max &lt; m) {
        m = opt_cov_max
    };
    <b>if</b> (user_cov_max &lt; m) {
        m = user_cov_max
    };
    m
}
</code></pre>



</details>

<a name="social_contracts_insurance_reserve_to_covered"></a>

## Function `reserve_to_covered`



<pre><code><b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_reserve_to_covered">reserve_to_covered</a>(head_reserve: u64, coverage_bps: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_reserve_to_covered">reserve_to_covered</a>(head_reserve: u64, coverage_bps: u64): u64 {
    <b>if</b> (coverage_bps == 0) {
        <a href="../social_contracts/insurance.md#social_contracts_insurance_MAX_U64">MAX_U64</a>
    } <b>else</b> {
        <b>let</b> v = (head_reserve <b>as</b> u128) * (<a href="../social_contracts/insurance.md#social_contracts_insurance_BPS_DENOM">BPS_DENOM</a> <b>as</b> u128) / (coverage_bps <b>as</b> u128);
        <b>if</b> (v &gt; (<a href="../social_contracts/insurance.md#social_contracts_insurance_MAX_U64">MAX_U64</a> <b>as</b> u128)) {
            <a href="../social_contracts/insurance.md#social_contracts_insurance_MAX_U64">MAX_U64</a>
        } <b>else</b> {
            v <b>as</b> u64
        }
    }
}
</code></pre>



</details>

<a name="social_contracts_insurance_quote_vault_for_spot_coverage"></a>

## Function `quote_vault_for_spot_coverage`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_quote_vault_for_spot_coverage">quote_vault_for_spot_coverage</a>(config: &<a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceConfig">social_contracts::insurance::InsuranceConfig</a>, router_cfg: &<a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceRouterConfig">social_contracts::insurance::InsuranceRouterConfig</a>, vault: &<a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">social_contracts::insurance::UnderwriterVault</a>, record: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotRecord">social_contracts::social_proof_of_truth::SpotRecord</a>, insured: <b>address</b>, option_id: u8, requested_coverage_amount: u64, coverage_bps: u64, duration_ms: u64): <a href="../social_contracts/insurance.md#social_contracts_insurance_VaultCoverageQuote">social_contracts::insurance::VaultCoverageQuote</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_quote_vault_for_spot_coverage">quote_vault_for_spot_coverage</a>(
    config: &<a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceConfig">InsuranceConfig</a>,
    router_cfg: &<a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceRouterConfig">InsuranceRouterConfig</a>,
    vault: &<a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">UnderwriterVault</a>,
    record: &spot::SpotRecord,
    insured: <b>address</b>,
    option_id: u8,
    requested_coverage_amount: u64,
    coverage_bps: u64,
    duration_ms: u64,
): <a href="../social_contracts/insurance.md#social_contracts_insurance_VaultCoverageQuote">VaultCoverageQuote</a> {
    <b>if</b> (router_cfg.router_paused) {
        <b>return</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_coverage_quote_skipped">coverage_quote_skipped</a>(<a href="../social_contracts/insurance.md#social_contracts_insurance_SKIPPED_ROUTER_PAUSED">SKIPPED_ROUTER_PAUSED</a>)
    };
    <b>let</b> market_id = spot::get_id_address(record);
    <b>if</b> (table::contains(&router_cfg.market_pause, market_id)
        && *table::borrow(&router_cfg.market_pause, market_id)) {
        <b>return</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_coverage_quote_skipped">coverage_quote_skipped</a>(<a href="../social_contracts/insurance.md#social_contracts_insurance_SKIPPED_MARKET_PAUSED">SKIPPED_MARKET_PAUSED</a>)
    };
    <b>if</b> (!vault.enabled) {
        <b>return</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_coverage_quote_skipped">coverage_quote_skipped</a>(<a href="../social_contracts/insurance.md#social_contracts_insurance_SKIPPED_VAULT_DISABLED">SKIPPED_VAULT_DISABLED</a>)
    };
    <b>if</b> (vault.paused) {
        <b>return</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_coverage_quote_skipped">coverage_quote_skipped</a>(<a href="../social_contracts/insurance.md#social_contracts_insurance_SKIPPED_VAULT_PAUSED">SKIPPED_VAULT_PAUSED</a>)
    };
    <b>let</b> cap = balance::value(&vault.capital);
    <b>let</b> r = vault.reserved;
    <b>if</b> (cap * <a href="../social_contracts/insurance.md#social_contracts_insurance_BPS_DENOM">BPS_DENOM</a> &lt; r * router_cfg.min_vault_health_factor_bps) {
        <b>return</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_coverage_quote_skipped">coverage_quote_skipped</a>(<a href="../social_contracts/insurance.md#social_contracts_insurance_SKIPPED_UNHEALTHY_VAULT">SKIPPED_UNHEALTHY_VAULT</a>)
    };
    <b>let</b> t_total = spot::total_option_escrow(record);
    <b>if</b> (t_total &lt; config.min_spot_total_liquidity) {
        <b>return</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_coverage_quote_skipped">coverage_quote_skipped</a>(<a href="../social_contracts/insurance.md#social_contracts_insurance_SKIPPED_THIN_OR_POOL">SKIPPED_THIN_OR_POOL</a>)
    };
    <b>if</b> ((option_id <b>as</b> u64) &gt;= spot::num_betting_options(record)) {
        <b>return</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_coverage_quote_skipped">coverage_quote_skipped</a>(<a href="../social_contracts/insurance.md#social_contracts_insurance_SKIPPED_THIN_OR_POOL">SKIPPED_THIN_OR_POOL</a>)
    };
    <b>let</b> max_fill =
        <a href="../social_contracts/insurance.md#social_contracts_insurance_max_fill_covered_for_vault">max_fill_covered_for_vault</a>(config, vault, record, market_id, insured, option_id, coverage_bps);
    <b>if</b> (max_fill == 0) {
        <b>return</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_coverage_quote_skipped">coverage_quote_skipped</a>(<a href="../social_contracts/insurance.md#social_contracts_insurance_SKIPPED_ZERO_CAPACITY">SKIPPED_ZERO_CAPACITY</a>)
    };
    <b>let</b> trade = <b>if</b> (requested_coverage_amount &lt;= max_fill) {
        requested_coverage_amount
    } <b>else</b> {
        max_fill
    };
    <b>let</b> pq = <a href="../social_contracts/insurance.md#social_contracts_insurance_compute_spot_risk_quote">compute_spot_risk_quote</a>(
        config,
        vault,
        record,
        market_id,
        option_id,
        trade,
        coverage_bps,
        duration_ms,
        <b>false</b>,
    );
    <b>if</b> (pq.risk_multiplier_bps &gt; config.max_risk_multiplier_bps) {
        <b>return</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_VaultCoverageQuote">VaultCoverageQuote</a> {
            premium: 0,
            premium_raw: 0,
            reserve_required: 0,
            available_capacity_reserve: <a href="../social_contracts/insurance.md#social_contracts_insurance_compute_reserve">compute_reserve</a>(max_fill, coverage_bps),
            risk_multiplier_bps: pq.risk_multiplier_bps,
            implied_prob_win_bps: pq.implied_prob_win_bps,
            utilization_bps: <a href="../social_contracts/insurance.md#social_contracts_insurance_vault_utilization_bps">vault_utilization_bps</a>(vault),
            max_fill_covered_amount: max_fill,
            skipped_reason: <a href="../social_contracts/insurance.md#social_contracts_insurance_SKIPPED_RISK_MULTIPLIER">SKIPPED_RISK_MULTIPLIER</a>,
        }
    };
    <b>let</b> res_req = <a href="../social_contracts/insurance.md#social_contracts_insurance_compute_reserve">compute_reserve</a>(trade, coverage_bps);
    <b>let</b> avail_res = <a href="../social_contracts/insurance.md#social_contracts_insurance_compute_reserve">compute_reserve</a>(max_fill, coverage_bps);
    <b>let</b> util_bps = <a href="../social_contracts/insurance.md#social_contracts_insurance_vault_utilization_bps">vault_utilization_bps</a>(vault);
    <a href="../social_contracts/insurance.md#social_contracts_insurance_VaultCoverageQuote">VaultCoverageQuote</a> {
        premium: pq.premium,
        premium_raw: pq.premium_raw,
        reserve_required: res_req,
        available_capacity_reserve: avail_res,
        risk_multiplier_bps: pq.risk_multiplier_bps,
        implied_prob_win_bps: pq.implied_prob_win_bps,
        utilization_bps: util_bps,
        max_fill_covered_amount: max_fill,
        skipped_reason: <a href="../social_contracts/insurance.md#social_contracts_insurance_SKIPPED_OK">SKIPPED_OK</a>,
    }
}
</code></pre>



</details>

<a name="social_contracts_insurance_vault_utilization_bps"></a>

## Function `vault_utilization_bps`



<pre><code><b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_vault_utilization_bps">vault_utilization_bps</a>(vault: &<a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">social_contracts::insurance::UnderwriterVault</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_vault_utilization_bps">vault_utilization_bps</a>(vault: &<a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">UnderwriterVault</a>): u64 {
    <b>let</b> capital_value = balance::value(&vault.capital);
    <b>if</b> (capital_value == 0) {
        <a href="../social_contracts/insurance.md#social_contracts_insurance_BPS_DENOM">BPS_DENOM</a>
    } <b>else</b> {
        <b>let</b> utilization_u128 = (vault.reserved <b>as</b> u128) * (<a href="../social_contracts/insurance.md#social_contracts_insurance_BPS_DENOM">BPS_DENOM</a> <b>as</b> u128) / (capital_value <b>as</b> u128);
        <b>assert</b>!(utilization_u128 &lt;= (<a href="../social_contracts/insurance.md#social_contracts_insurance_MAX_U64">MAX_U64</a> <b>as</b> u128), <a href="../social_contracts/insurance.md#social_contracts_insurance_EOverflow">EOverflow</a>);
        utilization_u128 <b>as</b> u64
    }
}
</code></pre>



</details>

<a name="social_contracts_insurance_buy_coverage_execute"></a>

## Function `buy_coverage_execute`



<pre><code><b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_buy_coverage_execute">buy_coverage_execute</a>(config: &<a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceConfig">social_contracts::insurance::InsuranceConfig</a>, router_cfg: &<a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceRouterConfig">social_contracts::insurance::InsuranceRouterConfig</a>, backstop: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceBackstopPool">social_contracts::insurance::InsuranceBackstopPool</a>, spot_config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">social_contracts::social_proof_of_truth::SpotConfig</a>, vault: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">social_contracts::insurance::UnderwriterVault</a>, record: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotRecord">social_contracts::social_proof_of_truth::SpotRecord</a>, option_id: u8, covered_amount: u64, coverage_bps: u64, duration_ms: u64, payment: &<b>mut</b> <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, route_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;, route_leg_index: u8, check_market_router: bool, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): (<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, u64, u64, u64, u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_buy_coverage_execute">buy_coverage_execute</a>(
    config: &<a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceConfig">InsuranceConfig</a>,
    router_cfg: &<a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceRouterConfig">InsuranceRouterConfig</a>,
    backstop: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceBackstopPool">InsuranceBackstopPool</a>,
    spot_config: &spot::SpotConfig,
    vault: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">UnderwriterVault</a>,
    record: &spot::SpotRecord,
    option_id: u8,
    covered_amount: u64,
    coverage_bps: u64,
    duration_ms: u64,
    payment: &<b>mut</b> Coin&lt;MYSO&gt;,
    clock: &Clock,
    route_id: Option&lt;ID&gt;,
    route_leg_index: u8,
    check_market_router: bool,
    ctx: &<b>mut</b> TxContext,
): (ID, ID, u64, u64, u64, u64, u64) {
    <b>assert</b>!(config.enable_flag, <a href="../social_contracts/insurance.md#social_contracts_insurance_EDisabled">EDisabled</a>);
    <b>assert</b>!(spot::is_enabled(spot_config), <a href="../social_contracts/insurance.md#social_contracts_insurance_EMarketClosed">EMarketClosed</a>);
    <b>assert</b>!(spot::is_open(record), <a href="../social_contracts/insurance.md#social_contracts_insurance_EMarketClosed">EMarketClosed</a>);
    <b>assert</b>!(coverage_bps &gt;= config.min_coverage_bps, <a href="../social_contracts/insurance.md#social_contracts_insurance_EInvalidCoverage">EInvalidCoverage</a>);
    <b>assert</b>!(coverage_bps &lt;= config.max_coverage_bps, <a href="../social_contracts/insurance.md#social_contracts_insurance_EInvalidCoverage">EInvalidCoverage</a>);
    <b>assert</b>!(duration_ms &gt; 0 && duration_ms &lt;= config.max_duration_ms, <a href="../social_contracts/insurance.md#social_contracts_insurance_EInvalidDuration">EInvalidDuration</a>);
    <b>assert</b>!(covered_amount &gt; 0, <a href="../social_contracts/insurance.md#social_contracts_insurance_EInvalidAmount">EInvalidAmount</a>);
    <b>let</b> insured = tx_context::sender(ctx);
    <b>let</b> market_id = spot::get_id_address(record);
    <b>if</b> (check_market_router) {
        <a href="../social_contracts/insurance.md#social_contracts_insurance_assert_market_router_open">assert_market_router_open</a>(router_cfg, market_id);
    };
    <a href="../social_contracts/insurance.md#social_contracts_insurance_assert_vault_buy_guards">assert_vault_buy_guards</a>(vault, router_cfg, <b>true</b>);
    <b>let</b> position_amount = spot::get_user_option_amount(record, insured, option_id);
    <b>assert</b>!(covered_amount &lt;= position_amount, <a href="../social_contracts/insurance.md#social_contracts_insurance_EInvalidAmount">EInvalidAmount</a>);
    <b>let</b> reserve_amount = <a href="../social_contracts/insurance.md#social_contracts_insurance_compute_reserve">compute_reserve</a>(covered_amount, coverage_bps);
    <b>let</b> capital_value = balance::value(&vault.capital);
    <b>assert</b>!(capital_value &gt;= vault.reserved, <a href="../social_contracts/insurance.md#social_contracts_insurance_EOverflow">EOverflow</a>);
    <b>let</b> free_capital = capital_value - vault.reserved;
    <b>assert</b>!(free_capital &gt;= reserve_amount, <a href="../social_contracts/insurance.md#social_contracts_insurance_EInsufficientCapital">EInsufficientCapital</a>);
    <b>assert</b>!(vault.reserved &lt;= <a href="../social_contracts/insurance.md#social_contracts_insurance_MAX_U64">MAX_U64</a> - reserve_amount, <a href="../social_contracts/insurance.md#social_contracts_insurance_EOverflow">EOverflow</a>);
    <b>let</b> pq = <a href="../social_contracts/insurance.md#social_contracts_insurance_compute_spot_risk_quote">compute_spot_risk_quote</a>(
        config,
        vault,
        record,
        market_id,
        option_id,
        covered_amount,
        coverage_bps,
        duration_ms,
        <b>true</b>,
    );
    <b>let</b> premium = pq.premium;
    <a href="../social_contracts/insurance.md#social_contracts_insurance_enforce_exposure_limits">enforce_exposure_limits</a>(vault, market_id, insured, option_id, reserve_amount, ctx);
    <b>assert</b>!(coin::value(payment) &gt;= premium, <a href="../social_contracts/insurance.md#social_contracts_insurance_EInsufficientPremium">EInsufficientPremium</a>);
    <b>let</b> sweep_bps = backstop.sweep_premium_bps;
    <b>if</b> (sweep_bps &gt; 0) {
        <b>assert</b>!(!backstop.paused, <a href="../social_contracts/insurance.md#social_contracts_insurance_EBackstopPaused">EBackstopPaused</a>);
    };
    <b>let</b> sweep_amt = (premium * sweep_bps) / <a href="../social_contracts/insurance.md#social_contracts_insurance_BPS_DENOM">BPS_DENOM</a>;
    <b>let</b> to_vault_amt = premium - sweep_amt;
    <b>let</b> <b>mut</b> prem_coin = coin::split(payment, premium, ctx);
    <b>if</b> (sweep_amt &gt; 0) {
        <b>let</b> sc = coin::split(&<b>mut</b> prem_coin, sweep_amt, ctx);
        balance::join(&<b>mut</b> backstop.capital, coin::into_balance(sc));
    };
    balance::join(&<b>mut</b> vault.capital, coin::into_balance(prem_coin));
    vault.reserved = vault.reserved + reserve_amount;
    <a href="../social_contracts/insurance.md#social_contracts_insurance_add_exposure">add_exposure</a>(vault, market_id, insured, option_id, reserve_amount, ctx);
    <b>let</b> now = clock::timestamp_ms(clock);
    <b>assert</b>!(now &lt;= <a href="../social_contracts/insurance.md#social_contracts_insurance_MAX_U64">MAX_U64</a> - duration_ms, <a href="../social_contracts/insurance.md#social_contracts_insurance_EOverflow">EOverflow</a>);
    <b>let</b> expiry_time_ms = now + duration_ms;
    <b>let</b> vault_id_ins = object::id(vault);
    <b>let</b> policy = <a href="../social_contracts/insurance.md#social_contracts_insurance_CoveragePolicy">CoveragePolicy</a> {
        id: object::new(ctx),
        market_id,
        insured,
        option_id,
        covered_amount,
        coverage_bps,
        premium_paid: premium,
        start_time_ms: now,
        expiry_time_ms,
        vault_id: vault_id_ins,
        status: <a href="../social_contracts/insurance.md#social_contracts_insurance_STATUS_ACTIVE">STATUS_ACTIVE</a>,
        route_id,
        route_leg_index,
    };
    <b>let</b> policy_id = object::id(&policy);
    transfer::share_object(policy);
    event::emit(<a href="../social_contracts/insurance.md#social_contracts_insurance_CoveragePurchasedEvent">CoveragePurchasedEvent</a> {
        policy_id,
        vault_id: vault_id_ins,
        market_id,
        insured,
        option_id,
        covered_amount,
        coverage_bps,
        premium_paid: premium,
        premium_raw: pq.premium_raw,
        reserve_locked: reserve_amount,
        expiry_time_ms,
        implied_probability_bps: pq.implied_prob_win_bps,
        risk_multiplier_bps: pq.risk_multiplier_bps,
        base_premium: pq.base_premium,
        market_total_amount: pq.market_total_amount,
        option_amount: pq.option_amount,
        backstop_sweep_amount: sweep_amt,
        route_id,
        route_leg_index,
    });
    (policy_id, vault_id_ins, premium, reserve_amount, sweep_amt, covered_amount, expiry_time_ms)
}
</code></pre>



</details>

<a name="social_contracts_insurance_buy_coverage"></a>

## Function `buy_coverage`

Buy coverage for a SPoT position


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_buy_coverage">buy_coverage</a>(config: &<a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceConfig">social_contracts::insurance::InsuranceConfig</a>, router_cfg: &<a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceRouterConfig">social_contracts::insurance::InsuranceRouterConfig</a>, backstop: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceBackstopPool">social_contracts::insurance::InsuranceBackstopPool</a>, spot_config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">social_contracts::social_proof_of_truth::SpotConfig</a>, vault: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">social_contracts::insurance::UnderwriterVault</a>, record: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotRecord">social_contracts::social_proof_of_truth::SpotRecord</a>, option_id: u8, requested_coverage_amount: u64, coverage_bps: u64, duration_ms: u64, payment: <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_buy_coverage">buy_coverage</a>(
    config: &<a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceConfig">InsuranceConfig</a>,
    router_cfg: &<a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceRouterConfig">InsuranceRouterConfig</a>,
    backstop: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceBackstopPool">InsuranceBackstopPool</a>,
    spot_config: &spot::SpotConfig,
    vault: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">UnderwriterVault</a>,
    record: &spot::SpotRecord,
    option_id: u8,
    requested_coverage_amount: u64,
    coverage_bps: u64,
    duration_ms: u64,
    <b>mut</b> payment: Coin&lt;MYSO&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(config.enable_flag, <a href="../social_contracts/insurance.md#social_contracts_insurance_EDisabled">EDisabled</a>);
    <b>assert</b>!(spot::is_enabled(spot_config), <a href="../social_contracts/insurance.md#social_contracts_insurance_EMarketClosed">EMarketClosed</a>);
    <b>assert</b>!(spot::is_open(record), <a href="../social_contracts/insurance.md#social_contracts_insurance_EMarketClosed">EMarketClosed</a>);
    <b>assert</b>!(coverage_bps &gt;= config.min_coverage_bps, <a href="../social_contracts/insurance.md#social_contracts_insurance_EInvalidCoverage">EInvalidCoverage</a>);
    <b>assert</b>!(coverage_bps &lt;= config.max_coverage_bps, <a href="../social_contracts/insurance.md#social_contracts_insurance_EInvalidCoverage">EInvalidCoverage</a>);
    <b>assert</b>!(duration_ms &gt; 0 && duration_ms &lt;= config.max_duration_ms, <a href="../social_contracts/insurance.md#social_contracts_insurance_EInvalidDuration">EInvalidDuration</a>);
    <b>assert</b>!(requested_coverage_amount &gt; 0, <a href="../social_contracts/insurance.md#social_contracts_insurance_EInvalidAmount">EInvalidAmount</a>);
    <b>let</b> insured = tx_context::sender(ctx);
    <b>let</b> position_amount = spot::get_user_option_amount(record, insured, option_id);
    <b>let</b> covered_amount = <b>if</b> (requested_coverage_amount &lt;= position_amount) {
        requested_coverage_amount
    } <b>else</b> {
        position_amount
    };
    <b>assert</b>!(covered_amount &gt; 0, <a href="../social_contracts/insurance.md#social_contracts_insurance_EInvalidAmount">EInvalidAmount</a>);
    <a href="../social_contracts/insurance.md#social_contracts_insurance_buy_coverage_execute">buy_coverage_execute</a>(
        config,
        router_cfg,
        backstop,
        spot_config,
        vault,
        record,
        option_id,
        covered_amount,
        coverage_bps,
        duration_ms,
        &<b>mut</b> payment,
        clock,
        option::none(),
        0,
        <b>false</b>,
        ctx,
    );
    <b>if</b> (coin::value(&payment) &gt; 0) {
        transfer::public_transfer(payment, insured);
    } <b>else</b> {
        coin::destroy_zero(payment);
    };
}
</code></pre>



</details>

<a name="social_contracts_insurance_route_buy_coverage_4"></a>

## Function `route_buy_coverage_4`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_route_buy_coverage_4">route_buy_coverage_4</a>(config: &<a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceConfig">social_contracts::insurance::InsuranceConfig</a>, router_cfg: &<a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceRouterConfig">social_contracts::insurance::InsuranceRouterConfig</a>, backstop: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceBackstopPool">social_contracts::insurance::InsuranceBackstopPool</a>, spot_config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">social_contracts::social_proof_of_truth::SpotConfig</a>, record: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotRecord">social_contracts::social_proof_of_truth::SpotRecord</a>, v0: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">social_contracts::insurance::UnderwriterVault</a>, v1: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">social_contracts::insurance::UnderwriterVault</a>, v2: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">social_contracts::insurance::UnderwriterVault</a>, v3: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">social_contracts::insurance::UnderwriterVault</a>, option_id: u8, fill_0: u64, fill_1: u64, fill_2: u64, fill_3: u64, coverage_bps: u64, duration_ms: u64, deadline_ms: u64, min_total_covered: u64, max_total_premium: u64, payment: <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_route_buy_coverage_4">route_buy_coverage_4</a>(
    config: &<a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceConfig">InsuranceConfig</a>,
    router_cfg: &<a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceRouterConfig">InsuranceRouterConfig</a>,
    backstop: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceBackstopPool">InsuranceBackstopPool</a>,
    spot_config: &spot::SpotConfig,
    record: &spot::SpotRecord,
    v0: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">UnderwriterVault</a>,
    v1: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">UnderwriterVault</a>,
    v2: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">UnderwriterVault</a>,
    v3: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">UnderwriterVault</a>,
    option_id: u8,
    fill_0: u64,
    fill_1: u64,
    fill_2: u64,
    fill_3: u64,
    coverage_bps: u64,
    duration_ms: u64,
    deadline_ms: u64,
    min_total_covered: u64,
    max_total_premium: u64,
    <b>mut</b> payment: Coin&lt;MYSO&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(clock::timestamp_ms(clock) &lt;= deadline_ms, <a href="../social_contracts/insurance.md#social_contracts_insurance_EDeadlinePassed">EDeadlinePassed</a>);
    <b>assert</b>!(config.enable_flag, <a href="../social_contracts/insurance.md#social_contracts_insurance_EDisabled">EDisabled</a>);
    <b>assert</b>!(router_cfg.router_enabled, <a href="../social_contracts/insurance.md#social_contracts_insurance_ERouteDisabled">ERouteDisabled</a>);
    <b>assert</b>!(!router_cfg.router_paused, <a href="../social_contracts/insurance.md#social_contracts_insurance_ERouterPaused">ERouterPaused</a>);
    <b>assert</b>!(spot::is_enabled(spot_config), <a href="../social_contracts/insurance.md#social_contracts_insurance_EMarketClosed">EMarketClosed</a>);
    <b>assert</b>!(spot::is_open(record), <a href="../social_contracts/insurance.md#social_contracts_insurance_EMarketClosed">EMarketClosed</a>);
    <b>assert</b>!(coverage_bps &gt;= config.min_coverage_bps, <a href="../social_contracts/insurance.md#social_contracts_insurance_EInvalidCoverage">EInvalidCoverage</a>);
    <b>assert</b>!(coverage_bps &lt;= config.max_coverage_bps, <a href="../social_contracts/insurance.md#social_contracts_insurance_EInvalidCoverage">EInvalidCoverage</a>);
    <b>assert</b>!(duration_ms &gt; 0 && duration_ms &lt;= config.max_duration_ms, <a href="../social_contracts/insurance.md#social_contracts_insurance_EInvalidDuration">EInvalidDuration</a>);
    <b>let</b> insured = tx_context::sender(ctx);
    <b>let</b> market_id = spot::get_id_address(record);
    <a href="../social_contracts/insurance.md#social_contracts_insurance_assert_market_router_open">assert_market_router_open</a>(router_cfg, market_id);
    <b>let</b> position_amount = spot::get_user_option_amount(record, insured, option_id);
    <b>let</b> total_covered = fill_0 + fill_1 + fill_2 + fill_3;
    <b>assert</b>!(total_covered &gt; 0, <a href="../social_contracts/insurance.md#social_contracts_insurance_EInvalidAmount">EInvalidAmount</a>);
    <b>assert</b>!(total_covered &gt;= min_total_covered, <a href="../social_contracts/insurance.md#social_contracts_insurance_ESlippageCovered">ESlippageCovered</a>);
    <b>assert</b>!(total_covered &lt;= position_amount, <a href="../social_contracts/insurance.md#social_contracts_insurance_EInvalidAmount">EInvalidAmount</a>);
    <b>if</b> (fill_0 &gt; 0 && fill_1 &gt; 0) {
        <b>assert</b>!(object::id(v0) != object::id(v1), <a href="../social_contracts/insurance.md#social_contracts_insurance_EDuplicateVaultInRoute">EDuplicateVaultInRoute</a>);
    };
    <b>if</b> (fill_0 &gt; 0 && fill_2 &gt; 0) {
        <b>assert</b>!(object::id(v0) != object::id(v2), <a href="../social_contracts/insurance.md#social_contracts_insurance_EDuplicateVaultInRoute">EDuplicateVaultInRoute</a>);
    };
    <b>if</b> (fill_0 &gt; 0 && fill_3 &gt; 0) {
        <b>assert</b>!(object::id(v0) != object::id(v3), <a href="../social_contracts/insurance.md#social_contracts_insurance_EDuplicateVaultInRoute">EDuplicateVaultInRoute</a>);
    };
    <b>if</b> (fill_1 &gt; 0 && fill_2 &gt; 0) {
        <b>assert</b>!(object::id(v1) != object::id(v2), <a href="../social_contracts/insurance.md#social_contracts_insurance_EDuplicateVaultInRoute">EDuplicateVaultInRoute</a>);
    };
    <b>if</b> (fill_1 &gt; 0 && fill_3 &gt; 0) {
        <b>assert</b>!(object::id(v1) != object::id(v3), <a href="../social_contracts/insurance.md#social_contracts_insurance_EDuplicateVaultInRoute">EDuplicateVaultInRoute</a>);
    };
    <b>if</b> (fill_2 &gt; 0 && fill_3 &gt; 0) {
        <b>assert</b>!(object::id(v2) != object::id(v3), <a href="../social_contracts/insurance.md#social_contracts_insurance_EDuplicateVaultInRoute">EDuplicateVaultInRoute</a>);
    };
    <b>let</b> r0 = <b>if</b> (fill_0 &gt; 0) {
        <a href="../social_contracts/insurance.md#social_contracts_insurance_compute_reserve">compute_reserve</a>(fill_0, coverage_bps)
    } <b>else</b> {
        0
    };
    <b>let</b> r1 = <b>if</b> (fill_1 &gt; 0) {
        <a href="../social_contracts/insurance.md#social_contracts_insurance_compute_reserve">compute_reserve</a>(fill_1, coverage_bps)
    } <b>else</b> {
        0
    };
    <b>let</b> r2 = <b>if</b> (fill_2 &gt; 0) {
        <a href="../social_contracts/insurance.md#social_contracts_insurance_compute_reserve">compute_reserve</a>(fill_2, coverage_bps)
    } <b>else</b> {
        0
    };
    <b>let</b> r3 = <b>if</b> (fill_3 &gt; 0) {
        <a href="../social_contracts/insurance.md#social_contracts_insurance_compute_reserve">compute_reserve</a>(fill_3, coverage_bps)
    } <b>else</b> {
        0
    };
    <b>let</b> total_res = r0 + r1 + r2 + r3;
    <b>if</b> (router_cfg.max_route_reserve_market &gt; 0) {
        <b>assert</b>!(total_res &lt;= router_cfg.max_route_reserve_market, <a href="../social_contracts/insurance.md#social_contracts_insurance_EExposureLimit">EExposureLimit</a>);
    };
    <b>if</b> (router_cfg.max_route_reserve_user &gt; 0) {
        <b>assert</b>!(total_res &lt;= router_cfg.max_route_reserve_user, <a href="../social_contracts/insurance.md#social_contracts_insurance_EExposureLimit">EExposureLimit</a>);
    };
    <b>if</b> (router_cfg.max_route_reserve_option &gt; 0) {
        <b>assert</b>!(total_res &lt;= router_cfg.max_route_reserve_option, <a href="../social_contracts/insurance.md#social_contracts_insurance_EExposureLimit">EExposureLimit</a>);
    };
    <b>let</b> conc = router_cfg.max_vault_concentration_bps;
    <b>if</b> (r0 &gt; 0) {
        <b>assert</b>!(
            (r0 <b>as</b> u128) * (<a href="../social_contracts/insurance.md#social_contracts_insurance_BPS_DENOM">BPS_DENOM</a> <b>as</b> u128) &lt;= (total_res <b>as</b> u128) * (conc <b>as</b> u128),
            <a href="../social_contracts/insurance.md#social_contracts_insurance_EVaultConcentration">EVaultConcentration</a>
        );
    };
    <b>if</b> (r1 &gt; 0) {
        <b>assert</b>!(
            (r1 <b>as</b> u128) * (<a href="../social_contracts/insurance.md#social_contracts_insurance_BPS_DENOM">BPS_DENOM</a> <b>as</b> u128) &lt;= (total_res <b>as</b> u128) * (conc <b>as</b> u128),
            <a href="../social_contracts/insurance.md#social_contracts_insurance_EVaultConcentration">EVaultConcentration</a>
        );
    };
    <b>if</b> (r2 &gt; 0) {
        <b>assert</b>!(
            (r2 <b>as</b> u128) * (<a href="../social_contracts/insurance.md#social_contracts_insurance_BPS_DENOM">BPS_DENOM</a> <b>as</b> u128) &lt;= (total_res <b>as</b> u128) * (conc <b>as</b> u128),
            <a href="../social_contracts/insurance.md#social_contracts_insurance_EVaultConcentration">EVaultConcentration</a>
        );
    };
    <b>if</b> (r3 &gt; 0) {
        <b>assert</b>!(
            (r3 <b>as</b> u128) * (<a href="../social_contracts/insurance.md#social_contracts_insurance_BPS_DENOM">BPS_DENOM</a> <b>as</b> u128) &lt;= (total_res <b>as</b> u128) * (conc <b>as</b> u128),
            <a href="../social_contracts/insurance.md#social_contracts_insurance_EVaultConcentration">EVaultConcentration</a>
        );
    };
    <b>let</b> now = clock::timestamp_ms(clock);
    <b>assert</b>!(now &lt;= <a href="../social_contracts/insurance.md#social_contracts_insurance_MAX_U64">MAX_U64</a> - duration_ms, <a href="../social_contracts/insurance.md#social_contracts_insurance_EOverflow">EOverflow</a>);
    <b>let</b> expiry_time_ms = now + duration_ms;
    <b>let</b> <b>mut</b> route = <a href="../social_contracts/insurance.md#social_contracts_insurance_CoverageRoute">CoverageRoute</a> {
        id: object::new(ctx),
        insured,
        market_id,
        option_id,
        coverage_bps,
        start_time_ms: now,
        expiry_time_ms,
        policy_ids: vector::empty(),
        vault_ids: vector::empty(),
        total_covered: 0,
        total_premium: 0,
        total_reserve: 0,
        total_backstop_sweep: 0,
        version: <a href="../social_contracts/insurance.md#social_contracts_insurance_DEFAULT_VERSION">DEFAULT_VERSION</a>,
    };
    <b>let</b> route_id = object::id(&route);
    <b>let</b> <b>mut</b> leg: u8 = 0;
    <b>let</b> <b>mut</b> total_premium: u64 = 0;
    <b>if</b> (fill_0 &gt; 0) {
        <b>let</b> (pid, vid, prem, res, sw, cov, _) = <a href="../social_contracts/insurance.md#social_contracts_insurance_buy_coverage_execute">buy_coverage_execute</a>(
            config,
            router_cfg,
            backstop,
            spot_config,
            v0,
            record,
            option_id,
            fill_0,
            coverage_bps,
            duration_ms,
            &<b>mut</b> payment,
            clock,
            option::some(route_id),
            leg,
            <b>true</b>,
            ctx,
        );
        vector::push_back(&<b>mut</b> route.policy_ids, pid);
        vector::push_back(&<b>mut</b> route.vault_ids, vid);
        route.total_covered = route.total_covered + cov;
        route.total_premium = route.total_premium + prem;
        route.total_reserve = route.total_reserve + res;
        route.total_backstop_sweep = route.total_backstop_sweep + sw;
        total_premium = total_premium + prem;
        event::emit(<a href="../social_contracts/insurance.md#social_contracts_insurance_RouteFillEvent">RouteFillEvent</a> {
            route_id,
            leg_index: leg,
            vault_id: vid,
            policy_id: pid,
            covered_amount: cov,
            premium_paid: prem,
            reserve_locked: res,
            backstop_sweep_amount: sw,
        });
        leg = leg + 1;
    };
    <b>if</b> (fill_1 &gt; 0) {
        <b>let</b> (pid, vid, prem, res, sw, cov, _) = <a href="../social_contracts/insurance.md#social_contracts_insurance_buy_coverage_execute">buy_coverage_execute</a>(
            config,
            router_cfg,
            backstop,
            spot_config,
            v1,
            record,
            option_id,
            fill_1,
            coverage_bps,
            duration_ms,
            &<b>mut</b> payment,
            clock,
            option::some(route_id),
            leg,
            <b>true</b>,
            ctx,
        );
        vector::push_back(&<b>mut</b> route.policy_ids, pid);
        vector::push_back(&<b>mut</b> route.vault_ids, vid);
        route.total_covered = route.total_covered + cov;
        route.total_premium = route.total_premium + prem;
        route.total_reserve = route.total_reserve + res;
        route.total_backstop_sweep = route.total_backstop_sweep + sw;
        total_premium = total_premium + prem;
        event::emit(<a href="../social_contracts/insurance.md#social_contracts_insurance_RouteFillEvent">RouteFillEvent</a> {
            route_id,
            leg_index: leg,
            vault_id: vid,
            policy_id: pid,
            covered_amount: cov,
            premium_paid: prem,
            reserve_locked: res,
            backstop_sweep_amount: sw,
        });
        leg = leg + 1;
    };
    <b>if</b> (fill_2 &gt; 0) {
        <b>let</b> (pid, vid, prem, res, sw, cov, _) = <a href="../social_contracts/insurance.md#social_contracts_insurance_buy_coverage_execute">buy_coverage_execute</a>(
            config,
            router_cfg,
            backstop,
            spot_config,
            v2,
            record,
            option_id,
            fill_2,
            coverage_bps,
            duration_ms,
            &<b>mut</b> payment,
            clock,
            option::some(route_id),
            leg,
            <b>true</b>,
            ctx,
        );
        vector::push_back(&<b>mut</b> route.policy_ids, pid);
        vector::push_back(&<b>mut</b> route.vault_ids, vid);
        route.total_covered = route.total_covered + cov;
        route.total_premium = route.total_premium + prem;
        route.total_reserve = route.total_reserve + res;
        route.total_backstop_sweep = route.total_backstop_sweep + sw;
        total_premium = total_premium + prem;
        event::emit(<a href="../social_contracts/insurance.md#social_contracts_insurance_RouteFillEvent">RouteFillEvent</a> {
            route_id,
            leg_index: leg,
            vault_id: vid,
            policy_id: pid,
            covered_amount: cov,
            premium_paid: prem,
            reserve_locked: res,
            backstop_sweep_amount: sw,
        });
        leg = leg + 1;
    };
    <b>if</b> (fill_3 &gt; 0) {
        <b>let</b> (pid, vid, prem, res, sw, cov, _) = <a href="../social_contracts/insurance.md#social_contracts_insurance_buy_coverage_execute">buy_coverage_execute</a>(
            config,
            router_cfg,
            backstop,
            spot_config,
            v3,
            record,
            option_id,
            fill_3,
            coverage_bps,
            duration_ms,
            &<b>mut</b> payment,
            clock,
            option::some(route_id),
            leg,
            <b>true</b>,
            ctx,
        );
        vector::push_back(&<b>mut</b> route.policy_ids, pid);
        vector::push_back(&<b>mut</b> route.vault_ids, vid);
        route.total_covered = route.total_covered + cov;
        route.total_premium = route.total_premium + prem;
        route.total_reserve = route.total_reserve + res;
        route.total_backstop_sweep = route.total_backstop_sweep + sw;
        total_premium = total_premium + prem;
        event::emit(<a href="../social_contracts/insurance.md#social_contracts_insurance_RouteFillEvent">RouteFillEvent</a> {
            route_id,
            leg_index: leg,
            vault_id: vid,
            policy_id: pid,
            covered_amount: cov,
            premium_paid: prem,
            reserve_locked: res,
            backstop_sweep_amount: sw,
        });
    };
    <b>assert</b>!(total_premium &lt;= max_total_premium, <a href="../social_contracts/insurance.md#social_contracts_insurance_ESlippagePremium">ESlippagePremium</a>);
    <b>let</b> pids = <a href="../social_contracts/insurance.md#social_contracts_insurance_copy_id_vec">copy_id_vec</a>(&route.policy_ids);
    <b>let</b> vids = <a href="../social_contracts/insurance.md#social_contracts_insurance_copy_id_vec">copy_id_vec</a>(&route.vault_ids);
    event::emit(<a href="../social_contracts/insurance.md#social_contracts_insurance_CoverageRoutedEvent">CoverageRoutedEvent</a> {
        route_id,
        insured,
        market_id,
        option_id,
        coverage_bps,
        duration_ms,
        total_covered: route.total_covered,
        total_premium: route.total_premium,
        total_reserve: route.total_reserve,
        total_backstop_sweep: route.total_backstop_sweep,
        expiry_time_ms: route.expiry_time_ms,
        policy_ids: pids,
        vault_ids: vids,
    });
    transfer::share_object(route);
    <b>if</b> (coin::value(&payment) &gt; 0) {
        transfer::public_transfer(payment, insured);
    } <b>else</b> {
        coin::destroy_zero(payment);
    };
}
</code></pre>



</details>

<a name="social_contracts_insurance_cancel_coverage"></a>

## Function `cancel_coverage`

Cancel coverage while the market is open
Cancellation can result in 0 refund due to fee + rounding


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_cancel_coverage">cancel_coverage</a>(config: &<a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceConfig">social_contracts::insurance::InsuranceConfig</a>, spot_config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">social_contracts::social_proof_of_truth::SpotConfig</a>, treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, vault: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">social_contracts::insurance::UnderwriterVault</a>, record: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotRecord">social_contracts::social_proof_of_truth::SpotRecord</a>, policy: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_CoveragePolicy">social_contracts::insurance::CoveragePolicy</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_cancel_coverage">cancel_coverage</a>(
    config: &<a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceConfig">InsuranceConfig</a>,
    spot_config: &spot::SpotConfig,
    treasury: &EcosystemTreasury,
    vault: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">UnderwriterVault</a>,
    record: &spot::SpotRecord,
    policy: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_CoveragePolicy">CoveragePolicy</a>,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(config.enable_flag, <a href="../social_contracts/insurance.md#social_contracts_insurance_EDisabled">EDisabled</a>);
    <b>assert</b>!(spot::is_enabled(spot_config), <a href="../social_contracts/insurance.md#social_contracts_insurance_EMarketClosed">EMarketClosed</a>);
    <b>assert</b>!(spot::is_open(record), <a href="../social_contracts/insurance.md#social_contracts_insurance_EMarketClosed">EMarketClosed</a>);
    <b>assert</b>!(policy.status == <a href="../social_contracts/insurance.md#social_contracts_insurance_STATUS_ACTIVE">STATUS_ACTIVE</a>, <a href="../social_contracts/insurance.md#social_contracts_insurance_EPolicyNotActive">EPolicyNotActive</a>);
    <b>assert</b>!(tx_context::sender(ctx) == policy.insured, <a href="../social_contracts/insurance.md#social_contracts_insurance_ENotPolicyOwner">ENotPolicyOwner</a>);
    <b>assert</b>!(policy.market_id == spot::get_id_address(record), <a href="../social_contracts/insurance.md#social_contracts_insurance_EMarketMismatch">EMarketMismatch</a>);
    <b>assert</b>!(policy.vault_id == object::id(vault), <a href="../social_contracts/insurance.md#social_contracts_insurance_EInvalidVault">EInvalidVault</a>);
    <b>let</b> now = clock::timestamp_ms(clock);
    <b>assert</b>!(now &lt; policy.expiry_time_ms, <a href="../social_contracts/insurance.md#social_contracts_insurance_EPolicyExpired">EPolicyExpired</a>);
    <b>let</b> total_duration = policy.expiry_time_ms - policy.start_time_ms;
    <b>let</b> remaining = policy.expiry_time_ms - now;
    <b>let</b> refund_u128 = (policy.premium_paid <b>as</b> u128) * (remaining <b>as</b> u128) / (total_duration <b>as</b> u128);
    <b>assert</b>!(refund_u128 &lt;= (<a href="../social_contracts/insurance.md#social_contracts_insurance_MAX_U64">MAX_U64</a> <b>as</b> u128), <a href="../social_contracts/insurance.md#social_contracts_insurance_EOverflow">EOverflow</a>);
    <b>let</b> original_refund = refund_u128 <b>as</b> u64;
    <b>let</b> fee = (original_refund * config.fee_bps) / <a href="../social_contracts/insurance.md#social_contracts_insurance_BPS_DENOM">BPS_DENOM</a>;
    <b>let</b> net_refund = original_refund - fee;
    // original_refund == fee + net_refund; ensure vault can fund both splits
    <b>let</b> capital_value = balance::value(&vault.capital);
    <b>assert</b>!(capital_value &gt;= original_refund, <a href="../social_contracts/insurance.md#social_contracts_insurance_EInsufficientCapital">EInsufficientCapital</a>);
    <b>if</b> (fee &gt; 0) {
        <b>let</b> fee_balance = balance::split(&<b>mut</b> vault.capital, fee);
        <b>let</b> fee_coin = coin::from_balance(fee_balance, ctx);
        transfer::public_transfer(fee_coin, <a href="../social_contracts/profile.md#social_contracts_profile_get_treasury_address">profile::get_treasury_address</a>(treasury));
    };
    <b>if</b> (net_refund &gt; 0) {
        <b>let</b> refund_balance = balance::split(&<b>mut</b> vault.capital, net_refund);
        <b>let</b> refund_coin = coin::from_balance(refund_balance, ctx);
        transfer::public_transfer(refund_coin, policy.insured);
    };
    <b>let</b> reserve_amount = <a href="../social_contracts/insurance.md#social_contracts_insurance_compute_reserve">compute_reserve</a>(policy.covered_amount, policy.coverage_bps);
    <a href="../social_contracts/insurance.md#social_contracts_insurance_release_exposure">release_exposure</a>(vault, policy.market_id, policy.insured, policy.option_id, reserve_amount);
    <b>assert</b>!(vault.reserved &gt;= reserve_amount, <a href="../social_contracts/insurance.md#social_contracts_insurance_EOverflow">EOverflow</a>);
    vault.reserved = vault.reserved - reserve_amount;
    policy.status = <a href="../social_contracts/insurance.md#social_contracts_insurance_STATUS_CANCELLED">STATUS_CANCELLED</a>;
    event::emit(<a href="../social_contracts/insurance.md#social_contracts_insurance_CoverageCancelledEvent">CoverageCancelledEvent</a> {
        policy_id: object::id(policy),
        insured: policy.insured,
        refunded_amount: net_refund,
        fee_paid: fee,
    });
}
</code></pre>



</details>

<a name="social_contracts_insurance_claim"></a>

## Function `claim`

Claim payout after SPoT resolution
Payout is calculated as min(current_position, covered_amount) * coverage_bps / BPS_DENOM
Dynamic coverage: payout adjusts if user reduces their SPoT position after buying insurance.
This prevents exploitation where user buys insurance then exits bet.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_claim">claim</a>(config: &<a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceConfig">social_contracts::insurance::InsuranceConfig</a>, spot_config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">social_contracts::social_proof_of_truth::SpotConfig</a>, vault: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">social_contracts::insurance::UnderwriterVault</a>, record: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotRecord">social_contracts::social_proof_of_truth::SpotRecord</a>, policy: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_CoveragePolicy">social_contracts::insurance::CoveragePolicy</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_claim">claim</a>(
    config: &<a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceConfig">InsuranceConfig</a>,
    spot_config: &spot::SpotConfig,
    vault: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">UnderwriterVault</a>,
    record: &spot::SpotRecord,
    policy: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_CoveragePolicy">CoveragePolicy</a>,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(config.enable_flag, <a href="../social_contracts/insurance.md#social_contracts_insurance_EDisabled">EDisabled</a>);
    <b>assert</b>!(spot::is_enabled(spot_config), <a href="../social_contracts/insurance.md#social_contracts_insurance_EMarketClosed">EMarketClosed</a>);
    <b>assert</b>!(policy.status == <a href="../social_contracts/insurance.md#social_contracts_insurance_STATUS_ACTIVE">STATUS_ACTIVE</a>, <a href="../social_contracts/insurance.md#social_contracts_insurance_EPolicyNotActive">EPolicyNotActive</a>);
    <b>assert</b>!(tx_context::sender(ctx) == policy.insured, <a href="../social_contracts/insurance.md#social_contracts_insurance_ENotPolicyOwner">ENotPolicyOwner</a>);
    <b>assert</b>!(policy.market_id == spot::get_id_address(record), <a href="../social_contracts/insurance.md#social_contracts_insurance_EMarketMismatch">EMarketMismatch</a>);
    <b>assert</b>!(policy.vault_id == object::id(vault), <a href="../social_contracts/insurance.md#social_contracts_insurance_EInvalidVault">EInvalidVault</a>);
    <b>assert</b>!(spot::is_resolved(record), <a href="../social_contracts/insurance.md#social_contracts_insurance_EMarketClosed">EMarketClosed</a>);
    <b>let</b> now = clock::timestamp_ms(clock);
    <b>assert</b>!(now &lt;= policy.expiry_time_ms, <a href="../social_contracts/insurance.md#social_contracts_insurance_EPolicyExpired">EPolicyExpired</a>);
    <b>let</b> outcome_opt = spot::get_outcome(record);
    <b>assert</b>!(option::is_some(outcome_opt), <a href="../social_contracts/insurance.md#social_contracts_insurance_EMarketClosed">EMarketClosed</a>);
    <b>let</b> outcome = *option::borrow(outcome_opt);
    <b>let</b> <b>mut</b> payout = 0;
    <b>if</b> (outcome != spot::outcome_draw() && outcome != spot::outcome_unapplicable()) {
        <b>if</b> (outcome != policy.option_id) {
            // Dynamic coverage: payout adjusts <b>if</b> user reduces their SPoT position after buying <a href="../social_contracts/insurance.md#social_contracts_insurance">insurance</a>
            <b>let</b> current_position = spot::get_user_option_amount(record, policy.insured, policy.option_id);
            <b>let</b> eligible_amount = <b>if</b> (current_position &lt; policy.covered_amount) {
                current_position
            } <b>else</b> {
                policy.covered_amount
            };
            <b>let</b> payout_u128 = (eligible_amount <b>as</b> u128) * (policy.coverage_bps <b>as</b> u128) / (<a href="../social_contracts/insurance.md#social_contracts_insurance_BPS_DENOM">BPS_DENOM</a> <b>as</b> u128);
            <b>assert</b>!(payout_u128 &lt;= (<a href="../social_contracts/insurance.md#social_contracts_insurance_MAX_U64">MAX_U64</a> <b>as</b> u128), <a href="../social_contracts/insurance.md#social_contracts_insurance_EOverflow">EOverflow</a>);
            payout = payout_u128 <b>as</b> u64;
        };
    };
    <b>if</b> (payout &gt; 0) {
        <b>let</b> payout_balance = balance::split(&<b>mut</b> vault.capital, payout);
        <b>let</b> payout_coin = coin::from_balance(payout_balance, ctx);
        transfer::public_transfer(payout_coin, policy.insured);
    };
    <b>let</b> reserve_amount = <a href="../social_contracts/insurance.md#social_contracts_insurance_compute_reserve">compute_reserve</a>(policy.covered_amount, policy.coverage_bps);
    <a href="../social_contracts/insurance.md#social_contracts_insurance_release_exposure">release_exposure</a>(vault, policy.market_id, policy.insured, policy.option_id, reserve_amount);
    <b>assert</b>!(vault.reserved &gt;= reserve_amount, <a href="../social_contracts/insurance.md#social_contracts_insurance_EOverflow">EOverflow</a>);
    vault.reserved = vault.reserved - reserve_amount;
    policy.status = <a href="../social_contracts/insurance.md#social_contracts_insurance_STATUS_CLAIMED">STATUS_CLAIMED</a>;
    event::emit(<a href="../social_contracts/insurance.md#social_contracts_insurance_CoverageClaimedEvent">CoverageClaimedEvent</a> {
        policy_id: object::id(policy),
        insured: policy.insured,
        payout,
    });
}
</code></pre>



</details>

<a name="social_contracts_insurance_expire_policy"></a>

## Function `expire_policy`

Expire policy and release reserves


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_expire_policy">expire_policy</a>(vault: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">social_contracts::insurance::UnderwriterVault</a>, policy: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_CoveragePolicy">social_contracts::insurance::CoveragePolicy</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_expire_policy">expire_policy</a>(
    vault: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">UnderwriterVault</a>,
    policy: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_CoveragePolicy">CoveragePolicy</a>,
    clock: &Clock
) {
    <b>if</b> (policy.status != <a href="../social_contracts/insurance.md#social_contracts_insurance_STATUS_ACTIVE">STATUS_ACTIVE</a>) {
        <b>return</b>
    };
    <b>if</b> (policy.vault_id != object::id(vault)) {
        <b>return</b>
    };
    <b>let</b> now = clock::timestamp_ms(clock);
    <b>if</b> (now &lt; policy.expiry_time_ms) {
        <b>return</b>
    };
    <b>let</b> reserve_amount = <a href="../social_contracts/insurance.md#social_contracts_insurance_compute_reserve">compute_reserve</a>(policy.covered_amount, policy.coverage_bps);
    <a href="../social_contracts/insurance.md#social_contracts_insurance_release_exposure">release_exposure</a>(vault, policy.market_id, policy.insured, policy.option_id, reserve_amount);
    <b>assert</b>!(vault.reserved &gt;= reserve_amount, <a href="../social_contracts/insurance.md#social_contracts_insurance_EOverflow">EOverflow</a>);
    vault.reserved = vault.reserved - reserve_amount;
    policy.status = <a href="../social_contracts/insurance.md#social_contracts_insurance_STATUS_EXPIRED">STATUS_EXPIRED</a>;
    event::emit(<a href="../social_contracts/insurance.md#social_contracts_insurance_PolicyExpiredEvent">PolicyExpiredEvent</a> {
        policy_id: object::id(policy),
        insured: policy.insured,
        market_id: policy.market_id,
        vault_id: policy.vault_id,
        reserve_released: reserve_amount,
        expiry_time_ms: policy.expiry_time_ms,
    });
}
</code></pre>



</details>

<a name="social_contracts_insurance_compute_reserve"></a>

## Function `compute_reserve`



<pre><code><b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_compute_reserve">compute_reserve</a>(covered_amount: u64, coverage_bps: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_compute_reserve">compute_reserve</a>(covered_amount: u64, coverage_bps: u64): u64 {
    <b>let</b> reserve_u128 = (covered_amount <b>as</b> u128) * (coverage_bps <b>as</b> u128);
    <b>let</b> reserve_u128 = reserve_u128 / (<a href="../social_contracts/insurance.md#social_contracts_insurance_BPS_DENOM">BPS_DENOM</a> <b>as</b> u128);
    <b>assert</b>!(reserve_u128 &lt;= (<a href="../social_contracts/insurance.md#social_contracts_insurance_MAX_U64">MAX_U64</a> <b>as</b> u128), <a href="../social_contracts/insurance.md#social_contracts_insurance_EOverflow">EOverflow</a>);
    reserve_u128 <b>as</b> u64
}
</code></pre>



</details>

<a name="social_contracts_insurance_enforce_exposure_limits"></a>

## Function `enforce_exposure_limits`



<pre><code><b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_enforce_exposure_limits">enforce_exposure_limits</a>(vault: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">social_contracts::insurance::UnderwriterVault</a>, market_id: <b>address</b>, insured: <b>address</b>, option_id: u8, reserve_amount: u64, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_enforce_exposure_limits">enforce_exposure_limits</a>(
    vault: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">UnderwriterVault</a>,
    market_id: <b>address</b>,
    insured: <b>address</b>,
    option_id: u8,
    reserve_amount: u64,
    ctx: &<b>mut</b> TxContext
) {
    // Read limit values before creating mutable borrows
    <b>let</b> max_exposure_per_market = vault.max_exposure_per_market;
    <b>let</b> max_exposure_per_user = vault.max_exposure_per_user;
    <b>let</b> max_exposure_per_option = vault.max_exposure_per_option;
    // Check user exposure limit first (doesn't require market exposure)
    <b>if</b> (max_exposure_per_user &gt; 0) {
        <b>let</b> current_user = <a href="../social_contracts/insurance.md#social_contracts_insurance_get_user_exposure">get_user_exposure</a>(vault, insured);
        <b>assert</b>!(current_user &lt;= <a href="../social_contracts/insurance.md#social_contracts_insurance_MAX_U64">MAX_U64</a> - reserve_amount, <a href="../social_contracts/insurance.md#social_contracts_insurance_EOverflow">EOverflow</a>);
        <b>let</b> new_user = current_user + reserve_amount;
        <b>assert</b>!(new_user &lt;= max_exposure_per_user, <a href="../social_contracts/insurance.md#social_contracts_insurance_EExposureLimit">EExposureLimit</a>);
    };
    // Now get mutable reference to market exposure <b>for</b> market and option checks
    <b>let</b> exposure = <a href="../social_contracts/insurance.md#social_contracts_insurance_get_market_exposure_mut">get_market_exposure_mut</a>(vault, market_id, ctx);
    <b>if</b> (max_exposure_per_market &gt; 0) {
        <b>assert</b>!(exposure.total_reserved &lt;= <a href="../social_contracts/insurance.md#social_contracts_insurance_MAX_U64">MAX_U64</a> - reserve_amount, <a href="../social_contracts/insurance.md#social_contracts_insurance_EOverflow">EOverflow</a>);
        <b>let</b> new_total = exposure.total_reserved + reserve_amount;
        <b>assert</b>!(new_total &lt;= max_exposure_per_market, <a href="../social_contracts/insurance.md#social_contracts_insurance_EExposureLimit">EExposureLimit</a>);
    };
    <b>let</b> option_reserved = <a href="../social_contracts/insurance.md#social_contracts_insurance_get_option_reserved">get_option_reserved</a>(exposure, option_id);
    <b>assert</b>!(option_reserved &lt;= <a href="../social_contracts/insurance.md#social_contracts_insurance_MAX_U64">MAX_U64</a> - reserve_amount, <a href="../social_contracts/insurance.md#social_contracts_insurance_EOverflow">EOverflow</a>);
    <b>let</b> new_opt_reserved = option_reserved + reserve_amount;
    <b>if</b> (max_exposure_per_option &gt; 0) {
        <b>assert</b>!(new_opt_reserved &lt;= max_exposure_per_option, <a href="../social_contracts/insurance.md#social_contracts_insurance_EExposureLimit">EExposureLimit</a>);
    };
}
</code></pre>



</details>

<a name="social_contracts_insurance_add_exposure"></a>

## Function `add_exposure`



<pre><code><b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_add_exposure">add_exposure</a>(vault: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">social_contracts::insurance::UnderwriterVault</a>, market_id: <b>address</b>, insured: <b>address</b>, option_id: u8, reserve_amount: u64, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_add_exposure">add_exposure</a>(
    vault: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">UnderwriterVault</a>,
    market_id: <b>address</b>,
    insured: <b>address</b>,
    option_id: u8,
    reserve_amount: u64,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> exposure = <a href="../social_contracts/insurance.md#social_contracts_insurance_get_market_exposure_mut">get_market_exposure_mut</a>(vault, market_id, ctx);
    <b>assert</b>!(exposure.total_reserved &lt;= <a href="../social_contracts/insurance.md#social_contracts_insurance_MAX_U64">MAX_U64</a> - reserve_amount, <a href="../social_contracts/insurance.md#social_contracts_insurance_EOverflow">EOverflow</a>);
    exposure.total_reserved = exposure.total_reserved + reserve_amount;
    <b>let</b> option_reserved = <a href="../social_contracts/insurance.md#social_contracts_insurance_get_option_reserved">get_option_reserved</a>(exposure, option_id);
    <b>assert</b>!(option_reserved &lt;= <a href="../social_contracts/insurance.md#social_contracts_insurance_MAX_U64">MAX_U64</a> - reserve_amount, <a href="../social_contracts/insurance.md#social_contracts_insurance_EOverflow">EOverflow</a>);
    <b>let</b> new_option_reserved = option_reserved + reserve_amount;
    <a href="../social_contracts/insurance.md#social_contracts_insurance_set_option_reserved">set_option_reserved</a>(exposure, option_id, new_option_reserved);
    <b>let</b> current_user = <a href="../social_contracts/insurance.md#social_contracts_insurance_get_user_exposure">get_user_exposure</a>(vault, insured);
    <b>assert</b>!(current_user &lt;= <a href="../social_contracts/insurance.md#social_contracts_insurance_MAX_U64">MAX_U64</a> - reserve_amount, <a href="../social_contracts/insurance.md#social_contracts_insurance_EOverflow">EOverflow</a>);
    <b>let</b> new_user = current_user + reserve_amount;
    <a href="../social_contracts/insurance.md#social_contracts_insurance_set_user_exposure">set_user_exposure</a>(vault, insured, new_user);
}
</code></pre>



</details>

<a name="social_contracts_insurance_release_exposure"></a>

## Function `release_exposure`



<pre><code><b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_release_exposure">release_exposure</a>(vault: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">social_contracts::insurance::UnderwriterVault</a>, market_id: <b>address</b>, insured: <b>address</b>, option_id: u8, reserve_amount: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_release_exposure">release_exposure</a>(
    vault: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">UnderwriterVault</a>,
    market_id: <b>address</b>,
    insured: <b>address</b>,
    option_id: u8,
    reserve_amount: u64
) {
    <b>if</b> (reserve_amount == 0) {
        <b>return</b>
    };
    <b>assert</b>!(table::contains(&vault.market_exposures, market_id), <a href="../social_contracts/insurance.md#social_contracts_insurance_EExposureInvariantBroken">EExposureInvariantBroken</a>);
    <b>let</b> exposure = table::borrow_mut(&<b>mut</b> vault.market_exposures, market_id);
    <b>assert</b>!(exposure.total_reserved &gt;= reserve_amount, <a href="../social_contracts/insurance.md#social_contracts_insurance_EExposureInvariantBroken">EExposureInvariantBroken</a>);
    exposure.total_reserved = exposure.total_reserved - reserve_amount;
    <b>assert</b>!(table::contains(&exposure.reserved_by_option, option_id), <a href="../social_contracts/insurance.md#social_contracts_insurance_EExposureInvariantBroken">EExposureInvariantBroken</a>);
    <b>let</b> current_option = *table::borrow(&exposure.reserved_by_option, option_id);
    <b>assert</b>!(current_option &gt;= reserve_amount, <a href="../social_contracts/insurance.md#social_contracts_insurance_EExposureInvariantBroken">EExposureInvariantBroken</a>);
    <b>let</b> option_ref = table::borrow_mut(&<b>mut</b> exposure.reserved_by_option, option_id);
    *option_ref = current_option - reserve_amount;
    <b>assert</b>!(table::contains(&vault.user_exposures, insured), <a href="../social_contracts/insurance.md#social_contracts_insurance_EExposureInvariantBroken">EExposureInvariantBroken</a>);
    <b>let</b> current_user = *table::borrow(&vault.user_exposures, insured);
    <b>assert</b>!(current_user &gt;= reserve_amount, <a href="../social_contracts/insurance.md#social_contracts_insurance_EExposureInvariantBroken">EExposureInvariantBroken</a>);
    <b>let</b> user_ref = table::borrow_mut(&<b>mut</b> vault.user_exposures, insured);
    *user_ref = current_user - reserve_amount;
}
</code></pre>



</details>

<a name="social_contracts_insurance_get_market_exposure_mut"></a>

## Function `get_market_exposure_mut`



<pre><code><b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_get_market_exposure_mut">get_market_exposure_mut</a>(vault: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">social_contracts::insurance::UnderwriterVault</a>, market_id: <b>address</b>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_MarketExposure">social_contracts::insurance::MarketExposure</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_get_market_exposure_mut">get_market_exposure_mut</a>(
    vault: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">UnderwriterVault</a>,
    market_id: <b>address</b>,
    ctx: &<b>mut</b> TxContext
): &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_MarketExposure">MarketExposure</a> {
    <b>if</b> (!table::contains(&vault.market_exposures, market_id)) {
        <b>let</b> exposure = <a href="../social_contracts/insurance.md#social_contracts_insurance_MarketExposure">MarketExposure</a> {
            market_id,
            total_reserved: 0,
            reserved_by_option: table::new(ctx),
        };
        table::add(&<b>mut</b> vault.market_exposures, market_id, exposure);
    };
    table::borrow_mut(&<b>mut</b> vault.market_exposures, market_id)
}
</code></pre>



</details>

<a name="social_contracts_insurance_get_user_exposure"></a>

## Function `get_user_exposure`



<pre><code><b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_get_user_exposure">get_user_exposure</a>(vault: &<a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">social_contracts::insurance::UnderwriterVault</a>, insured: <b>address</b>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_get_user_exposure">get_user_exposure</a>(vault: &<a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">UnderwriterVault</a>, insured: <b>address</b>): u64 {
    <b>if</b> (table::contains(&vault.user_exposures, insured)) {
        *table::borrow(&vault.user_exposures, insured)
    } <b>else</b> {
        0
    }
}
</code></pre>



</details>

<a name="social_contracts_insurance_set_user_exposure"></a>

## Function `set_user_exposure`



<pre><code><b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_set_user_exposure">set_user_exposure</a>(vault: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">social_contracts::insurance::UnderwriterVault</a>, insured: <b>address</b>, amount: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_set_user_exposure">set_user_exposure</a>(vault: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">UnderwriterVault</a>, insured: <b>address</b>, amount: u64) {
    <b>if</b> (table::contains(&vault.user_exposures, insured)) {
        <b>let</b> user_ref = table::borrow_mut(&<b>mut</b> vault.user_exposures, insured);
        *user_ref = amount;
    } <b>else</b> {
        table::add(&<b>mut</b> vault.user_exposures, insured, amount);
    };
}
</code></pre>



</details>

<a name="social_contracts_insurance_get_option_reserved"></a>

## Function `get_option_reserved`



<pre><code><b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_get_option_reserved">get_option_reserved</a>(exposure: &<a href="../social_contracts/insurance.md#social_contracts_insurance_MarketExposure">social_contracts::insurance::MarketExposure</a>, option_id: u8): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_get_option_reserved">get_option_reserved</a>(exposure: &<a href="../social_contracts/insurance.md#social_contracts_insurance_MarketExposure">MarketExposure</a>, option_id: u8): u64 {
    <b>if</b> (table::contains(&exposure.reserved_by_option, option_id)) {
        *table::borrow(&exposure.reserved_by_option, option_id)
    } <b>else</b> {
        0
    }
}
</code></pre>



</details>

<a name="social_contracts_insurance_set_option_reserved"></a>

## Function `set_option_reserved`



<pre><code><b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_set_option_reserved">set_option_reserved</a>(exposure: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_MarketExposure">social_contracts::insurance::MarketExposure</a>, option_id: u8, amount: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_set_option_reserved">set_option_reserved</a>(exposure: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_MarketExposure">MarketExposure</a>, option_id: u8, amount: u64) {
    <b>if</b> (table::contains(&exposure.reserved_by_option, option_id)) {
        <b>let</b> option_ref = table::borrow_mut(&<b>mut</b> exposure.reserved_by_option, option_id);
        *option_ref = amount;
    } <b>else</b> {
        table::add(&<b>mut</b> exposure.reserved_by_option, option_id, amount);
    };
}
</code></pre>



</details>

<a name="social_contracts_insurance_migrate_config"></a>

## Function `migrate_config`

Migration function for InsuranceConfig


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_migrate_config">migrate_config</a>(config: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceConfig">social_contracts::insurance::InsuranceConfig</a>, _: &<a href="../social_contracts/upgrade.md#social_contracts_upgrade_UpgradeAdminCap">social_contracts::upgrade::UpgradeAdminCap</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_migrate_config">migrate_config</a>(
    config: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceConfig">InsuranceConfig</a>,
    _: &UpgradeAdminCap,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> current_version = <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>();
    // Verify this is an <a href="../social_contracts/upgrade.md#social_contracts_upgrade">upgrade</a> (new version &gt; current version)
    <b>assert</b>!(config.version &lt; current_version, <a href="../social_contracts/insurance.md#social_contracts_insurance_EWrongVersion">EWrongVersion</a>);
    // Remember old version and update to new version
    <b>let</b> old_version = config.version;
    config.version = current_version;
    // Emit event <b>for</b> object migration
    <b>let</b> config_id = object::id(config);
    <a href="../social_contracts/upgrade.md#social_contracts_upgrade_emit_migration_event">upgrade::emit_migration_event</a>(
        config_id,
        string::utf8(b"<a href="../social_contracts/insurance.md#social_contracts_insurance_InsuranceConfig">InsuranceConfig</a>"),
        old_version,
        tx_context::sender(ctx)
    );
}
</code></pre>



</details>

<a name="social_contracts_insurance_migrate_vault"></a>

## Function `migrate_vault`

Migration function for UnderwriterVault


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_migrate_vault">migrate_vault</a>(vault: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">social_contracts::insurance::UnderwriterVault</a>, _: &<a href="../social_contracts/upgrade.md#social_contracts_upgrade_UpgradeAdminCap">social_contracts::upgrade::UpgradeAdminCap</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_migrate_vault">migrate_vault</a>(
    vault: &<b>mut</b> <a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">UnderwriterVault</a>,
    _: &UpgradeAdminCap,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> current_version = <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>();
    // Verify this is an <a href="../social_contracts/upgrade.md#social_contracts_upgrade">upgrade</a> (new version &gt; current version)
    <b>assert</b>!(vault.version &lt; current_version, <a href="../social_contracts/insurance.md#social_contracts_insurance_EWrongVersion">EWrongVersion</a>);
    // Remember old version and update to new version
    <b>let</b> old_version = vault.version;
    vault.version = current_version;
    // Emit event <b>for</b> object migration
    <b>let</b> vault_id = object::id(vault);
    <a href="../social_contracts/upgrade.md#social_contracts_upgrade_emit_migration_event">upgrade::emit_migration_event</a>(
        vault_id,
        string::utf8(b"<a href="../social_contracts/insurance.md#social_contracts_insurance_UnderwriterVault">UnderwriterVault</a>"),
        old_version,
        tx_context::sender(ctx)
    );
}
</code></pre>



</details>
