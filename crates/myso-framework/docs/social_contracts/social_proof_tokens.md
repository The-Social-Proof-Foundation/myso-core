---
title: Module `social_contracts::social_proof_tokens`
---

Social Proof Tokens module for the MySocial network.
This module provides functionality for creation and trading of both profile tokens
and post tokens using an Automated Market Maker (AMM) with a quadratic pricing curve.
It includes fee distribution mechanisms for transactions, splitting between profile owner,
platform, and ecosystem treasury.

**SPT amounts (nano-SPT):** <code><a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialToken">SocialToken</a>.amount</code>, <code><a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenInfo">TokenInfo</a>.circulating_supply</code>, pool holder
balances, and SPT quantity fields in events are stored in fixed-point **nano-SPT** units:
<code>10^9</code> nano-SPT = <code>1.0</code> display token (same decimal count as native MYSO). MYSO payments and
prices remain in MYSO smallest units. Buy/sell cost uses a continuous integral of the marginal
price curve over nano-supply to stay well-defined at sub-token precision.

Use <code><a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_nano_spt_from_whole_tokens">nano_spt_from_whole_tokens</a></code> / <code><a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_nano_spt_from_whole_and_fraction">nano_spt_from_whole_and_fraction</a></code> to convert display amounts
plus optional sub-token nano remainder into the <code>u64</code> nano-SPT values passed to buy/sell entrypoints.


-  [Struct `SocialProofTokensAdminCap`](#social_contracts_social_proof_tokens_SocialProofTokensAdminCap)
-  [Struct `SocialProofTokensConfig`](#social_contracts_social_proof_tokens_SocialProofTokensConfig)
-  [Struct `TokenRegistry`](#social_contracts_social_proof_tokens_TokenRegistry)
-  [Struct `ReservationPool`](#social_contracts_social_proof_tokens_ReservationPool)
-  [Struct `TokenInfo`](#social_contracts_social_proof_tokens_TokenInfo)
-  [Struct `TokenPool`](#social_contracts_social_proof_tokens_TokenPool)
-  [Struct `SocialToken`](#social_contracts_social_proof_tokens_SocialToken)
-  [Struct `ReservationPoolObject`](#social_contracts_social_proof_tokens_ReservationPoolObject)
-  [Struct `TokenPoolCreatedEvent`](#social_contracts_social_proof_tokens_TokenPoolCreatedEvent)
-  [Struct `TokenBoughtEvent`](#social_contracts_social_proof_tokens_TokenBoughtEvent)
-  [Struct `TokenSoldEvent`](#social_contracts_social_proof_tokens_TokenSoldEvent)
-  [Struct `TokenSwappedEvent`](#social_contracts_social_proof_tokens_TokenSwappedEvent)
-  [Struct `TokenTransferredEvent`](#social_contracts_social_proof_tokens_TokenTransferredEvent)
-  [Struct `ReservationCreatedEvent`](#social_contracts_social_proof_tokens_ReservationCreatedEvent)
-  [Struct `ReservationWithdrawnEvent`](#social_contracts_social_proof_tokens_ReservationWithdrawnEvent)
-  [Struct `ThresholdMetEvent`](#social_contracts_social_proof_tokens_ThresholdMetEvent)
-  [Struct `ReservationPoolCreatedEvent`](#social_contracts_social_proof_tokens_ReservationPoolCreatedEvent)
-  [Struct `ConfigUpdatedEvent`](#social_contracts_social_proof_tokens_ConfigUpdatedEvent)
-  [Struct `TokensAddedEvent`](#social_contracts_social_proof_tokens_TokensAddedEvent)
-  [Struct `EmergencyKillSwitchEvent`](#social_contracts_social_proof_tokens_EmergencyKillSwitchEvent)
-  [Struct `PocRedirectionUpdatedEvent`](#social_contracts_social_proof_tokens_PocRedirectionUpdatedEvent)
-  [Constants](#@Constants_0)
-  [Function `split_social_token`](#social_contracts_social_proof_tokens_split_social_token)
-  [Function `merge_social_tokens`](#social_contracts_social_proof_tokens_merge_social_tokens)
-  [Function `split_social_token_entry`](#social_contracts_social_proof_tokens_split_social_token_entry)
-  [Function `merge_social_tokens_entry`](#social_contracts_social_proof_tokens_merge_social_tokens_entry)
-  [Function `bootstrap_init`](#social_contracts_social_proof_tokens_bootstrap_init)
-  [Function `update_social_proof_tokens_config`](#social_contracts_social_proof_tokens_update_social_proof_tokens_config)
-  [Function `toggle_emergency_kill_switch`](#social_contracts_social_proof_tokens_toggle_emergency_kill_switch)
-  [Function `is_trading_enabled`](#social_contracts_social_proof_tokens_is_trading_enabled)
-  [Function `calculate_total_fee_bps`](#social_contracts_social_proof_tokens_calculate_total_fee_bps)
-  [Function `calculate_reservation_total_fee_bps`](#social_contracts_social_proof_tokens_calculate_reservation_total_fee_bps)
-  [Function `validate_trading_fees`](#social_contracts_social_proof_tokens_validate_trading_fees)
-  [Function `validate_reservation_fees`](#social_contracts_social_proof_tokens_validate_reservation_fees)
-  [Function `calculate_fee_amount_safe`](#social_contracts_social_proof_tokens_calculate_fee_amount_safe)
-  [Function `calculate_component_fee_safe`](#social_contracts_social_proof_tokens_calculate_component_fee_safe)
-  [Function `reserve_towards_post`](#social_contracts_social_proof_tokens_reserve_towards_post)
-  [Function `reserve_towards_post_with_platform`](#social_contracts_social_proof_tokens_reserve_towards_post_with_platform)
-  [Function `reserve_towards_profile`](#social_contracts_social_proof_tokens_reserve_towards_profile)
-  [Function `reserve_towards_profile_with_platform`](#social_contracts_social_proof_tokens_reserve_towards_profile_with_platform)
-  [Function `apply_reservation_withdrawal_ledger`](#social_contracts_social_proof_tokens_apply_reservation_withdrawal_ledger)
-  [Function `reservation_withdrawal_fee_split`](#social_contracts_social_proof_tokens_reservation_withdrawal_fee_split)
-  [Function `distribute_reservation_withdraw_fees_non_platform_post`](#social_contracts_social_proof_tokens_distribute_reservation_withdraw_fees_non_platform_post)
-  [Function `distribute_reservation_withdraw_fees_non_platform_profile`](#social_contracts_social_proof_tokens_distribute_reservation_withdraw_fees_non_platform_profile)
-  [Function `distribute_reservation_withdraw_fees_platform_post`](#social_contracts_social_proof_tokens_distribute_reservation_withdraw_fees_platform_post)
-  [Function `distribute_reservation_withdraw_fees_platform_profile`](#social_contracts_social_proof_tokens_distribute_reservation_withdraw_fees_platform_profile)
-  [Function `withdraw_reservation_for_post`](#social_contracts_social_proof_tokens_withdraw_reservation_for_post)
-  [Function `withdraw_reservation_for_profile`](#social_contracts_social_proof_tokens_withdraw_reservation_for_profile)
-  [Function `withdraw_reservation_with_platform_for_post`](#social_contracts_social_proof_tokens_withdraw_reservation_with_platform_for_post)
-  [Function `withdraw_reservation_with_platform_for_profile`](#social_contracts_social_proof_tokens_withdraw_reservation_with_platform_for_profile)
-  [Function `bootstrap_reservation_pool_for_post_id`](#social_contracts_social_proof_tokens_bootstrap_reservation_pool_for_post_id)
-  [Function `create_post_with_reservation_pool`](#social_contracts_social_proof_tokens_create_post_with_reservation_pool)
-  [Function `enable_spt_for_post`](#social_contracts_social_proof_tokens_enable_spt_for_post)
-  [Function `create_reservation_pool_for_profile`](#social_contracts_social_proof_tokens_create_reservation_pool_for_profile)
-  [Function `can_create_auction`](#social_contracts_social_proof_tokens_can_create_auction)
-  [Function `create_social_proof_token`](#social_contracts_social_proof_tokens_create_social_proof_token)
-  [Function `sync_token_pool_manifest_from_post`](#social_contracts_social_proof_tokens_sync_token_pool_manifest_from_post)
-  [Function `sync_token_pool_poc_from_post`](#social_contracts_social_proof_tokens_sync_token_pool_poc_from_post)
-  [Function `update_token_poc_data`](#social_contracts_social_proof_tokens_update_token_poc_data)
-  [Function `pool_manifest_has_escrow_payout`](#social_contracts_social_proof_tokens_pool_manifest_has_escrow_payout)
-  [Function `should_apply_pool_revenue_manifest`](#social_contracts_social_proof_tokens_should_apply_pool_revenue_manifest)
-  [Function `apply_pool_revenue_manifest_coin`](#social_contracts_social_proof_tokens_apply_pool_revenue_manifest_coin)
-  [Function `apply_post_revenue_manifest_coin`](#social_contracts_social_proof_tokens_apply_post_revenue_manifest_coin)
-  [Function `distribute_creator_fee`](#social_contracts_social_proof_tokens_distribute_creator_fee)
-  [Function `distribute_creator_fee_from_pool`](#social_contracts_social_proof_tokens_distribute_creator_fee_from_pool)
-  [Function `apply_post_poc_redirection`](#social_contracts_social_proof_tokens_apply_post_poc_redirection)
-  [Function `distribute_reservation_creator_fee_with_owner`](#social_contracts_social_proof_tokens_distribute_reservation_creator_fee_with_owner)
-  [Function `distribute_reservation_creator_fee`](#social_contracts_social_proof_tokens_distribute_reservation_creator_fee)
-  [Function `distribute_reservation_creator_fee_no_poc_with_owner`](#social_contracts_social_proof_tokens_distribute_reservation_creator_fee_no_poc_with_owner)
-  [Function `distribute_reservation_creator_fee_no_poc`](#social_contracts_social_proof_tokens_distribute_reservation_creator_fee_no_poc)
-  [Function `distribute_reservation_fees_with_post`](#social_contracts_social_proof_tokens_distribute_reservation_fees_with_post)
-  [Function `distribute_reservation_fees_with_post_and_platform`](#social_contracts_social_proof_tokens_distribute_reservation_fees_with_post_and_platform)
-  [Function `distribute_reservation_fees_no_poc`](#social_contracts_social_proof_tokens_distribute_reservation_fees_no_poc)
-  [Function `distribute_reservation_fees_no_poc_with_platform`](#social_contracts_social_proof_tokens_distribute_reservation_fees_no_poc_with_platform)
-  [Function `buy_tokens`](#social_contracts_social_proof_tokens_buy_tokens)
-  [Function `buy_tokens_with_platform`](#social_contracts_social_proof_tokens_buy_tokens_with_platform)
-  [Function `buy_more_tokens`](#social_contracts_social_proof_tokens_buy_more_tokens)
-  [Function `buy_more_tokens_with_platform`](#social_contracts_social_proof_tokens_buy_more_tokens_with_platform)
-  [Function `sell_tokens`](#social_contracts_social_proof_tokens_sell_tokens)
-  [Function `sell_tokens_with_platform`](#social_contracts_social_proof_tokens_sell_tokens_with_platform)
-  [Function `transfer_tokens`](#social_contracts_social_proof_tokens_transfer_tokens)
-  [Function `swap_tokens`](#social_contracts_social_proof_tokens_swap_tokens)
-  [Function `swap_more_tokens`](#social_contracts_social_proof_tokens_swap_more_tokens)
-  [Function `swap_tokens_with_platform`](#social_contracts_social_proof_tokens_swap_tokens_with_platform)
-  [Function `swap_more_tokens_with_platform`](#social_contracts_social_proof_tokens_swap_more_tokens_with_platform)
-  [Function `execute_swap_non_platform`](#social_contracts_social_proof_tokens_execute_swap_non_platform)
-  [Function `execute_swap_with_platform`](#social_contracts_social_proof_tokens_execute_swap_with_platform)
-  [Function `unwrap_u256_opt`](#social_contracts_social_proof_tokens_unwrap_u256_opt)
-  [Function `u256_add_with_carry`](#social_contracts_social_proof_tokens_u256_add_with_carry)
-  [Function `u256_mul_widen`](#social_contracts_social_proof_tokens_u256_mul_widen)
-  [Function `u512_bit`](#social_contracts_social_proof_tokens_u512_bit)
-  [Function `u512_shl1_or_bit`](#social_contracts_social_proof_tokens_u512_shl1_or_bit)
-  [Function `u512_ge_u256`](#social_contracts_social_proof_tokens_u512_ge_u256)
-  [Function `u512_sub_u256`](#social_contracts_social_proof_tokens_u512_sub_u256)
-  [Function `u512_div_u256_floor`](#social_contracts_social_proof_tokens_u512_div_u256_floor)
-  [Function `quad_poly_buy`](#social_contracts_social_proof_tokens_quad_poly_buy)
-  [Function `quad_poly_sell`](#social_contracts_social_proof_tokens_quad_poly_sell)
-  [Function `quad_integral_leg_mist`](#social_contracts_social_proof_tokens_quad_integral_leg_mist)
-  [Function `mist_amount_u256_to_u64`](#social_contracts_social_proof_tokens_mist_amount_u256_to_u64)
-  [Function `calculate_token_price`](#social_contracts_social_proof_tokens_calculate_token_price)
-  [Function `calculate_buy_price`](#social_contracts_social_proof_tokens_calculate_buy_price)
-  [Function `calculate_sell_price`](#social_contracts_social_proof_tokens_calculate_sell_price)
-  [Function `calculate_swap_proceeds`](#social_contracts_social_proof_tokens_calculate_swap_proceeds)
-  [Function `calculate_max_buy_amount`](#social_contracts_social_proof_tokens_calculate_max_buy_amount)
-  [Function `calculate_swap_quote`](#social_contracts_social_proof_tokens_calculate_swap_quote)
-  [Function `spt_amount_scale`](#social_contracts_social_proof_tokens_spt_amount_scale)
-  [Function `spt_amount_decimals`](#social_contracts_social_proof_tokens_spt_amount_decimals)
-  [Function `nano_spt_from_whole_tokens`](#social_contracts_social_proof_tokens_nano_spt_from_whole_tokens)
-  [Function `nano_spt_from_whole_and_fraction`](#social_contracts_social_proof_tokens_nano_spt_from_whole_and_fraction)
-  [Function `get_token_info`](#social_contracts_social_proof_tokens_get_token_info)
-  [Function `token_info_circulating_supply`](#social_contracts_social_proof_tokens_token_info_circulating_supply)
-  [Function `token_exists`](#social_contracts_social_proof_tokens_token_exists)
-  [Function `get_token_owner`](#social_contracts_social_proof_tokens_get_token_owner)
-  [Function `get_pool_price`](#social_contracts_social_proof_tokens_get_pool_price)
-  [Function `get_user_balance`](#social_contracts_social_proof_tokens_get_user_balance)
-  [Function `get_revenue_manifest`](#social_contracts_social_proof_tokens_get_revenue_manifest)
-  [Function `has_poc_redirection`](#social_contracts_social_proof_tokens_has_poc_redirection)
-  [Function `get_poc_redirect_to`](#social_contracts_social_proof_tokens_get_poc_redirect_to)
-  [Function `get_poc_redirect_percentage`](#social_contracts_social_proof_tokens_get_poc_redirect_percentage)
-  [Function `get_pool_associated_id`](#social_contracts_social_proof_tokens_get_pool_associated_id)
-  [Function `set_revenue_manifest`](#social_contracts_social_proof_tokens_set_revenue_manifest)
-  [Function `set_poc_redirection`](#social_contracts_social_proof_tokens_set_poc_redirection)
-  [Function `set_poc_redirection_entry`](#social_contracts_social_proof_tokens_set_poc_redirection_entry)
-  [Function `set_poc_redirection_admin`](#social_contracts_social_proof_tokens_set_poc_redirection_admin)
-  [Function `clear_poc_redirection`](#social_contracts_social_proof_tokens_clear_poc_redirection)
-  [Function `registry_version`](#social_contracts_social_proof_tokens_registry_version)
-  [Function `borrow_registry_version_mut`](#social_contracts_social_proof_tokens_borrow_registry_version_mut)
-  [Function `pool_version`](#social_contracts_social_proof_tokens_pool_version)
-  [Function `borrow_pool_version_mut`](#social_contracts_social_proof_tokens_borrow_pool_version_mut)
-  [Function `reservation_pool_version`](#social_contracts_social_proof_tokens_reservation_pool_version)
-  [Function `borrow_reservation_pool_version_mut`](#social_contracts_social_proof_tokens_borrow_reservation_pool_version_mut)
-  [Function `config_version`](#social_contracts_social_proof_tokens_config_version)
-  [Function `borrow_config_version_mut`](#social_contracts_social_proof_tokens_borrow_config_version_mut)
-  [Function `migrate_token_registry`](#social_contracts_social_proof_tokens_migrate_token_registry)
-  [Function `migrate_token_pool`](#social_contracts_social_proof_tokens_migrate_token_pool)
-  [Function `migrate_reservation_pool`](#social_contracts_social_proof_tokens_migrate_reservation_pool)
-  [Function `migrate_social_proof_tokens_config`](#social_contracts_social_proof_tokens_migrate_social_proof_tokens_config)
-  [Function `create_social_proof_tokens_admin_cap`](#social_contracts_social_proof_tokens_create_social_proof_tokens_admin_cap)


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
<b>use</b> <a href="../myso/ed25519.md#myso_ed25519">myso::ed25519</a>;
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
<b>use</b> <a href="../myso/permissioned_group.md#myso_permissioned_group">myso::permissioned_group</a>;
<b>use</b> <a href="../myso/permissions_table.md#myso_permissions_table">myso::permissions_table</a>;
<b>use</b> <a href="../myso/protocol_config.md#myso_protocol_config">myso::protocol_config</a>;
<b>use</b> <a href="../myso/table.md#myso_table">myso::table</a>;
<b>use</b> <a href="../myso/transfer.md#myso_transfer">myso::transfer</a>;
<b>use</b> <a href="../myso/tx_context.md#myso_tx_context">myso::tx_context</a>;
<b>use</b> <a href="../myso/types.md#myso_types">myso::types</a>;
<b>use</b> <a href="../myso/unpause_cap.md#myso_unpause_cap">myso::unpause_cap</a>;
<b>use</b> <a href="../myso/url.md#myso_url">myso::url</a>;
<b>use</b> <a href="../myso/vec_map.md#myso_vec_map">myso::vec_map</a>;
<b>use</b> <a href="../myso/vec_set.md#myso_vec_set">myso::vec_set</a>;
<b>use</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit">social_contracts::ai_credit</a>;
<b>use</b> <a href="../social_contracts/block_list.md#social_contracts_block_list">social_contracts::block_list</a>;
<b>use</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph">social_contracts::derivative_graph</a>;
<b>use</b> <a href="../social_contracts/governance.md#social_contracts_governance">social_contracts::governance</a>;
<b>use</b> <a href="../social_contracts/media_asset.md#social_contracts_license_template">social_contracts::license_template</a>;
<b>use</b> <a href="../social_contracts/media_asset.md#social_contracts_media_asset">social_contracts::media_asset</a>;
<b>use</b> <a href="../social_contracts/memory.md#social_contracts_memory">social_contracts::memory</a>;
<b>use</b> <a href="../social_contracts/mydata.md#social_contracts_mydata">social_contracts::mydata</a>;
<b>use</b> <a href="../social_contracts/platform.md#social_contracts_platform">social_contracts::platform</a>;
<b>use</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault">social_contracts::poc_vault</a>;
<b>use</b> <a href="../social_contracts/post.md#social_contracts_post">social_contracts::post</a>;
<b>use</b> <a href="../social_contracts/profile.md#social_contracts_profile">social_contracts::profile</a>;
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
<b>use</b> <a href="../std/u256.md#std_u256">std::u256</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
</code></pre>



<a name="social_contracts_social_proof_tokens_SocialProofTokensAdminCap"></a>

## Struct `SocialProofTokensAdminCap`

Admin capability for the social proof tokens system


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensAdminCap">SocialProofTokensAdminCap</a> <b>has</b> key, store
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

<a name="social_contracts_social_proof_tokens_SocialProofTokensConfig"></a>

## Struct `SocialProofTokensConfig`

Global social proof tokens configuration


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">SocialProofTokensConfig</a> <b>has</b> key
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
<code>version: u64</code>
</dt>
<dd>
 Version for upgrades
</dd>
<dt>
<code>trading_creator_fee_bps: u64</code>
</dt>
<dd>
 Creator fee percentage in basis points (for trading)
</dd>
<dt>
<code>trading_platform_fee_bps: u64</code>
</dt>
<dd>
 Platform fee percentage in basis points (for trading)
</dd>
<dt>
<code>trading_treasury_fee_bps: u64</code>
</dt>
<dd>
 Treasury fee percentage in basis points (for trading)
</dd>
<dt>
<code>reservation_creator_fee_bps: u64</code>
</dt>
<dd>
 Creator reservation fee percentage in basis points
</dd>
<dt>
<code>reservation_platform_fee_bps: u64</code>
</dt>
<dd>
 Platform reservation fee percentage in basis points
</dd>
<dt>
<code>reservation_treasury_fee_bps: u64</code>
</dt>
<dd>
 Treasury reservation fee percentage in basis points
</dd>
<dt>
<code>base_price: u64</code>
</dt>
<dd>
 Base price for new tokens
</dd>
<dt>
<code>quadratic_coefficient: u64</code>
</dt>
<dd>
 Quadratic coefficient for pricing curve
</dd>
<dt>
<code>max_hold_percent_bps: u64</code>
</dt>
<dd>
 Max fraction of circulating supply one wallet may hold, in basis points (<code>10_000</code> = 100%).
</dd>
<dt>
<code>post_threshold: u64</code>
</dt>
<dd>
 Reservation thresholds for social proof token creation
</dd>
<dt>
<code>profile_threshold: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_individual_reservation_bps: u64</code>
</dt>
<dd>
 Max MYSO reservation per wallet vs pool threshold, in bps (<code>10_000</code> = 100% of threshold).
</dd>
<dt>
<code>max_reservers_per_pool: u64</code>
</dt>
<dd>
 Maximum unique reservers per reservation pool (DoS / gas bound).
</dd>
<dt>
<code>trading_enabled: bool</code>
</dt>
<dd>
 Emergency kill switch - when false, all trading is halted
</dd>
<dt>
<code>non_platform_platform_to_creator_bps: u64</code>
</dt>
<dd>
 Non-platform path: share of the platform-fee bucket routed to the creator (bps of that bucket).
</dd>
<dt>
<code>non_platform_platform_to_treasury_bps: u64</code>
</dt>
<dd>
 Non-platform path: share of the platform-fee bucket routed to the ecosystem treasury (bps of that bucket).
</dd>
</dl>


</details>

<a name="social_contracts_social_proof_tokens_TokenRegistry"></a>

## Struct `TokenRegistry`

Registry of all tokens in the exchange


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">TokenRegistry</a> <b>has</b> key
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
<code>tokens: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;<b>address</b>, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenInfo">social_contracts::social_proof_tokens::TokenInfo</a>&gt;</code>
</dt>
<dd>
 Table keyed by associated_id (post/profile ID), not pool ID, to token info
</dd>
<dt>
<code>reservation_pools: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;<b>address</b>, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ReservationPool">social_contracts::social_proof_tokens::ReservationPool</a>&gt;</code>
</dt>
<dd>
 Table from profile/post ID to reservation pool info
</dd>
<dt>
<code>version: u64</code>
</dt>
<dd>
 Version for upgrades
</dd>
</dl>


</details>

<a name="social_contracts_social_proof_tokens_ReservationPool"></a>

## Struct `ReservationPool`

Reservation pool for a specific post or profile
Note: reservers vector is only stored in ReservationPoolObject, not in registry


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ReservationPool">ReservationPool</a> <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>associated_id: <b>address</b></code>
</dt>
<dd>
 Associated profile or post ID
</dd>
<dt>
<code>token_type: u8</code>
</dt>
<dd>
 Token type (1=profile, 2=post)
</dd>
<dt>
<code>owner: <b>address</b></code>
</dt>
<dd>
 Owner of the profile/post
</dd>
<dt>
<code>total_reserved: u64</code>
</dt>
<dd>
 Total MYSO reserved towards this post/profile
</dd>
<dt>
<code>required_threshold: u64</code>
</dt>
<dd>
 Required threshold to enable auction creation
</dd>
<dt>
<code>created_at: u64</code>
</dt>
<dd>
 Creation timestamp
</dd>
</dl>


</details>

<a name="social_contracts_social_proof_tokens_TokenInfo"></a>

## Struct `TokenInfo`

Information about a token


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenInfo">TokenInfo</a> <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <b>address</b></code>
</dt>
<dd>
 The token ID (object ID of the pool)
</dd>
<dt>
<code>token_type: u8</code>
</dt>
<dd>
 Type of token (1=profile, 2=post)
</dd>
<dt>
<code>owner: <b>address</b></code>
</dt>
<dd>
 Owner/creator of the token
</dd>
<dt>
<code>associated_id: <b>address</b></code>
</dt>
<dd>
 Associated profile or post ID
</dd>
<dt>
<code>circulating_supply: u64</code>
</dt>
<dd>
 Circulating supply in **nano-SPT** (<code>10^9</code> per 1.0 token).
</dd>
<dt>
<code>base_price: u64</code>
</dt>
<dd>
 Base price for this token
</dd>
<dt>
<code>quadratic_coefficient: u64</code>
</dt>
<dd>
 Quadratic coefficient for this token's pricing curve
</dd>
<dt>
<code>created_at: u64</code>
</dt>
<dd>
 Creation timestamp
</dd>
</dl>


</details>

<a name="social_contracts_social_proof_tokens_TokenPool"></a>

## Struct `TokenPool`

Liquidity pool for a token (key only - not transferable)


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">TokenPool</a> <b>has</b> key
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
<code>info: <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenInfo">social_contracts::social_proof_tokens::TokenInfo</a></code>
</dt>
<dd>
 The token's info
</dd>
<dt>
<code>myso_balance: <a href="../myso/balance.md#myso_balance_Balance">myso::balance::Balance</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;</code>
</dt>
<dd>
 MYSO balance in the pool
</dd>
<dt>
<code>holders: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;<b>address</b>, u64&gt;</code>
</dt>
<dd>
 Holder balances in **nano-SPT**.
</dd>
<dt>
<code>revenue_manifest: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../social_contracts/media_asset.md#social_contracts_media_asset_RevenueManifest">social_contracts::media_asset::RevenueManifest</a>&gt;</code>
</dt>
<dd>
 Cached post revenue manifest for creator-fee routing (post tokens only)
</dd>
<dt>
<code>version: u64</code>
</dt>
<dd>
 Version for upgrades
</dd>
</dl>


</details>

<a name="social_contracts_social_proof_tokens_SocialToken"></a>

## Struct `SocialToken`

Social token that represents a user's owned tokens.
Intentionally has only <code>key</code> (no <code>store</code>) to prevent free P2P transfer: the pool
<code>holders</code> table is keyed by transaction sender, so transferring this object between
addresses would permanently desynchronise the on-chain balance state.


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialToken">SocialToken</a> <b>has</b> key
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
<code>pool_id: <b>address</b></code>
</dt>
<dd>
 Token pool ID
</dd>
<dt>
<code>token_type: u8</code>
</dt>
<dd>
 Token type (1=profile, 2=post)
</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>
 Balance in **nano-SPT** (<code>10^9</code> per 1.0 token).
</dd>
</dl>


</details>

<a name="social_contracts_social_proof_tokens_ReservationPoolObject"></a>

## Struct `ReservationPoolObject`

Reservation pool for collecting MYSO reservations towards posts/profiles


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ReservationPoolObject">ReservationPoolObject</a> <b>has</b> key
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
<code>info: <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ReservationPool">social_contracts::social_proof_tokens::ReservationPool</a></code>
</dt>
<dd>
 Reservation pool info (without reservers - kept separately below)
</dd>
<dt>
<code>myso_balance: <a href="../myso/balance.md#myso_balance_Balance">myso::balance::Balance</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;</code>
</dt>
<dd>
 MYSO balance reserved in this pool
</dd>
<dt>
<code>reservations: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;<b>address</b>, u64&gt;</code>
</dt>
<dd>
 Mapping of reservers' addresses to their reservation amounts
</dd>
<dt>
<code>reservers: vector&lt;<b>address</b>&gt;</code>
</dt>
<dd>
 List of all reservers (for efficient iteration) - only in object, not in registry
</dd>
<dt>
<code>converted: bool</code>
</dt>
<dd>
 Flag indicating if this pool has been converted to a token
</dd>
<dt>
<code>version: u64</code>
</dt>
<dd>
 Version for upgrades
</dd>
</dl>


</details>

<a name="social_contracts_social_proof_tokens_TokenPoolCreatedEvent"></a>

## Struct `TokenPoolCreatedEvent`

Event emitted when a token pool is created


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPoolCreatedEvent">TokenPoolCreatedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>token_type: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>owner: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>associated_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>base_price: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>quadratic_coefficient: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>circulating_supply: u64</code>
</dt>
<dd>
 Circulating supply in **nano-SPT** (<code>10^9</code> per 1.0 token).
</dd>
<dt>
<code>total_reserved_at_launch: u64</code>
</dt>
<dd>
 MYSO reserved (smallest units).
</dd>
</dl>


</details>

<a name="social_contracts_social_proof_tokens_TokenBoughtEvent"></a>

## Struct `TokenBoughtEvent`

Event emitted when a post pool is auto-initialized by SPoT flow
Event emitted when tokens are bought


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenBoughtEvent">TokenBoughtEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>buyer: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>
 SPT quantity in **nano-SPT**.
</dd>
<dt>
<code>myso_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>fee_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>creator_fee: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>platform_fee: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>treasury_fee: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>new_price: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_social_proof_tokens_TokenSoldEvent"></a>

## Struct `TokenSoldEvent`

Event emitted when tokens are sold


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenSoldEvent">TokenSoldEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>seller: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>
 SPT quantity in **nano-SPT**.
</dd>
<dt>
<code>myso_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>fee_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>creator_fee: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>platform_fee: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>treasury_fee: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>new_price: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_social_proof_tokens_TokenSwappedEvent"></a>

## Struct `TokenSwappedEvent`

Atomic summary of an SPT→SPT swap (emitted after TokenSoldEvent + TokenBoughtEvent).


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenSwappedEvent">TokenSwappedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>source_pool_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>dest_pool_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>trader: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>sell_amount: u64</code>
</dt>
<dd>
 Nano-SPT sold from the source pool.
</dd>
<dt>
<code>dest_amount: u64</code>
</dt>
<dd>
 Nano-SPT bought into the dest pool.
</dd>
<dt>
<code>sell_myso_gross: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>buy_myso_gross: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>sell_fee_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>buy_fee_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>sell_creator_fee: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>sell_platform_fee: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>sell_treasury_fee: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>buy_creator_fee: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>buy_platform_fee: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>buy_treasury_fee: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>leftover_myso: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>source_new_price: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>dest_new_price: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_social_proof_tokens_TokenTransferredEvent"></a>

## Struct `TokenTransferredEvent`

P2P SPT transfer that updates the pool <code>holders</code> ledger.


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenTransferredEvent">TokenTransferredEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>pool_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>from: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>to: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>
 Nano-SPT transferred.
</dd>
</dl>


</details>

<a name="social_contracts_social_proof_tokens_ReservationCreatedEvent"></a>

## Struct `ReservationCreatedEvent`

Event emitted when MYSO is reserved towards a post/profile


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ReservationCreatedEvent">ReservationCreatedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>associated_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>token_type: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>reserver: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>total_reserved: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>threshold_met: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>reserved_at: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>fee_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>creator_fee: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>platform_fee: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>treasury_fee: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_social_proof_tokens_ReservationWithdrawnEvent"></a>

## Struct `ReservationWithdrawnEvent`

Event emitted when MYSO reservation is withdrawn


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ReservationWithdrawnEvent">ReservationWithdrawnEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>associated_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>token_type: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>reserver: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>total_reserved: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>withdrawn_at: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>fee_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>creator_fee: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>platform_fee: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>treasury_fee: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_social_proof_tokens_ThresholdMetEvent"></a>

## Struct `ThresholdMetEvent`

Event emitted when reservation threshold is met for the first time


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ThresholdMetEvent">ThresholdMetEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>associated_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>token_type: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>owner: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>total_reserved: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>required_threshold: u64</code>
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

<a name="social_contracts_social_proof_tokens_ReservationPoolCreatedEvent"></a>

## Struct `ReservationPoolCreatedEvent`

Event emitted when a reservation pool is created


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ReservationPoolCreatedEvent">ReservationPoolCreatedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>associated_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>token_type: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>owner: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>required_threshold: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>pool_object_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>created_at: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_social_proof_tokens_ConfigUpdatedEvent"></a>

## Struct `ConfigUpdatedEvent`

Event emitted when social proof tokens config is updated


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ConfigUpdatedEvent">ConfigUpdatedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>updated_by: <b>address</b></code>
</dt>
<dd>
 Who performed the update
</dd>
<dt>
<code>timestamp: u64</code>
</dt>
<dd>
 When the update occurred
</dd>
<dt>
<code>total_fee_bps: u64</code>
</dt>
<dd>
 Trading fee percentages
</dd>
<dt>
<code>trading_creator_fee_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>trading_platform_fee_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>trading_treasury_fee_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>reservation_total_fee_bps: u64</code>
</dt>
<dd>
 Reservation fee percentages
</dd>
<dt>
<code>reservation_creator_fee_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>reservation_platform_fee_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>reservation_treasury_fee_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>base_price: u64</code>
</dt>
<dd>
 Curve parameters
</dd>
<dt>
<code>quadratic_coefficient: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_hold_percent_bps: u64</code>
</dt>
<dd>
 Maximum hold percentage
</dd>
<dt>
<code>post_threshold: u64</code>
</dt>
<dd>
 Reservation thresholds
</dd>
<dt>
<code>profile_threshold: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_individual_reservation_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_reservers_per_pool: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>non_platform_platform_to_creator_bps: u64</code>
</dt>
<dd>
 Non-platform split: creator share of platform-fee bucket (bps)
</dd>
<dt>
<code>non_platform_platform_to_treasury_bps: u64</code>
</dt>
<dd>
 Non-platform split: ecosystem treasury share of platform-fee bucket (bps)
</dd>
<dt>
<code>trading_enabled: bool</code>
</dt>
<dd>
 Whether SPT trading is enabled
</dd>
</dl>


</details>

<a name="social_contracts_social_proof_tokens_TokensAddedEvent"></a>

## Struct `TokensAddedEvent`

Event emitted when tokens are purchased by someone who already has a social token


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokensAddedEvent">TokensAddedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>owner: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>pool_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>
 SPT quantity in **nano-SPT**.
</dd>
</dl>


</details>

<a name="social_contracts_social_proof_tokens_EmergencyKillSwitchEvent"></a>

## Struct `EmergencyKillSwitchEvent`

Event emitted when emergency kill switch is toggled


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EmergencyKillSwitchEvent">EmergencyKillSwitchEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>admin: <b>address</b></code>
</dt>
<dd>
 Admin who activated/deactivated the kill switch
</dd>
<dt>
<code>trading_enabled: bool</code>
</dt>
<dd>
 New state of trading (true = enabled, false = halted)
</dd>
<dt>
<code>timestamp: u64</code>
</dt>
<dd>
 Timestamp of the action
</dd>
<dt>
<code>reason: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
 Reason for the action (optional)
</dd>
</dl>


</details>

<a name="social_contracts_social_proof_tokens_PocRedirectionUpdatedEvent"></a>

## Struct `PocRedirectionUpdatedEvent`

Event emitted when PoC redirection data is updated for a token pool


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_PocRedirectionUpdatedEvent">PocRedirectionUpdatedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>pool_id: <b>address</b></code>
</dt>
<dd>
 Token pool ID
</dd>
<dt>
<code>post_id: <b>address</b></code>
</dt>
<dd>
 Associated post ID
</dd>
<dt>
<code>redirect_to: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;</code>
</dt>
<dd>
 Address to redirect revenue to (None if cleared)
</dd>
<dt>
<code>redirect_percentage: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
 Percentage of revenue to redirect (None if cleared)
</dd>
<dt>
<code>poc_redirection_kind: u8</code>
</dt>
<dd>
 Mirrors post PoC redirect kind (<code>0</code> none, <code>1</code> wallet, <code>2</code> escrow)
</dd>
<dt>
<code>updated_by: <b>address</b></code>
</dt>
<dd>
 Who performed the update
</dd>
<dt>
<code>timestamp: u64</code>
</dt>
<dd>
 Timestamp of the update
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="social_contracts_social_proof_tokens_ENotAuthorized"></a>

Operation can only be performed by the admin


<pre><code><b>const</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ENotAuthorized">ENotAuthorized</a>: u64 = 0;
</code></pre>



<a name="social_contracts_social_proof_tokens_EInvalidFeeConfig"></a>

Invalid fee percentages configuration


<pre><code><b>const</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInvalidFeeConfig">EInvalidFeeConfig</a>: u64 = 1;
</code></pre>



<a name="social_contracts_social_proof_tokens_ETokenAlreadyExists"></a>

The token already exists


<pre><code><b>const</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ETokenAlreadyExists">ETokenAlreadyExists</a>: u64 = 2;
</code></pre>



<a name="social_contracts_social_proof_tokens_ETokenNotFound"></a>

The token does not exist


<pre><code><b>const</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ETokenNotFound">ETokenNotFound</a>: u64 = 3;
</code></pre>



<a name="social_contracts_social_proof_tokens_EExceededMaxHold"></a>

Exceeded maximum token hold percentage


<pre><code><b>const</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EExceededMaxHold">EExceededMaxHold</a>: u64 = 4;
</code></pre>



<a name="social_contracts_social_proof_tokens_EInsufficientFunds"></a>

Insufficient funds for operation


<pre><code><b>const</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInsufficientFunds">EInsufficientFunds</a>: u64 = 5;
</code></pre>



<a name="social_contracts_social_proof_tokens_ENoTokensOwned"></a>

Sender doesn't own any tokens


<pre><code><b>const</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ENoTokensOwned">ENoTokensOwned</a>: u64 = 6;
</code></pre>



<a name="social_contracts_social_proof_tokens_EInvalidID"></a>

Invalid post or profile ID


<pre><code><b>const</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInvalidID">EInvalidID</a>: u64 = 7;
</code></pre>



<a name="social_contracts_social_proof_tokens_EInsufficientLiquidity"></a>

Insufficient token liquidity


<pre><code><b>const</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInsufficientLiquidity">EInsufficientLiquidity</a>: u64 = 8;
</code></pre>



<a name="social_contracts_social_proof_tokens_ETokenAlreadyInitialized"></a>

Token already initialized in pool


<pre><code><b>const</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ETokenAlreadyInitialized">ETokenAlreadyInitialized</a>: u64 = 9;
</code></pre>



<a name="social_contracts_social_proof_tokens_EInvalidCurveParams"></a>

Curve parameters must be positive


<pre><code><b>const</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInvalidCurveParams">EInvalidCurveParams</a>: u64 = 10;
</code></pre>



<a name="social_contracts_social_proof_tokens_EInvalidTokenType"></a>

Invalid token type


<pre><code><b>const</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInvalidTokenType">EInvalidTokenType</a>: u64 = 11;
</code></pre>



<a name="social_contracts_social_proof_tokens_EViralThresholdNotMet"></a>

Viral threshold not met


<pre><code><b>const</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EViralThresholdNotMet">EViralThresholdNotMet</a>: u64 = 12;
</code></pre>



<a name="social_contracts_social_proof_tokens_EAuctionInProgress"></a>

Auction already in progress


<pre><code><b>const</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EAuctionInProgress">EAuctionInProgress</a>: u64 = 13;
</code></pre>



<a name="social_contracts_social_proof_tokens_EInvalidAuctionDuration"></a>

Invalid auction duration


<pre><code><b>const</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInvalidAuctionDuration">EInvalidAuctionDuration</a>: u64 = 14;
</code></pre>



<a name="social_contracts_social_proof_tokens_EAuctionNotActive"></a>

Auction not active


<pre><code><b>const</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EAuctionNotActive">EAuctionNotActive</a>: u64 = 15;
</code></pre>



<a name="social_contracts_social_proof_tokens_EAuctionNotEnded"></a>

Auction not ended


<pre><code><b>const</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EAuctionNotEnded">EAuctionNotEnded</a>: u64 = 16;
</code></pre>



<a name="social_contracts_social_proof_tokens_EAuctionAlreadyFinalized"></a>

Auction already finalized


<pre><code><b>const</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EAuctionAlreadyFinalized">EAuctionAlreadyFinalized</a>: u64 = 17;
</code></pre>



<a name="social_contracts_social_proof_tokens_ENoContribution"></a>

No contribution to auction


<pre><code><b>const</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ENoContribution">ENoContribution</a>: u64 = 18;
</code></pre>



<a name="social_contracts_social_proof_tokens_EBlockedUser"></a>

Cannot buy token from a blocked user


<pre><code><b>const</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EBlockedUser">EBlockedUser</a>: u64 = 19;
</code></pre>



<a name="social_contracts_social_proof_tokens_ETradingHalted"></a>

Trading is halted by emergency kill switch


<pre><code><b>const</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ETradingHalted">ETradingHalted</a>: u64 = 20;
</code></pre>



<a name="social_contracts_social_proof_tokens_EOverflow"></a>

Arithmetic overflow detected


<pre><code><b>const</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EOverflow">EOverflow</a>: u64 = 21;
</code></pre>



<a name="social_contracts_social_proof_tokens_EWrongVersion"></a>

Wrong version - object version mismatch


<pre><code><b>const</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EWrongVersion">EWrongVersion</a>: u64 = 22;
</code></pre>



<a name="social_contracts_social_proof_tokens_EUserNotJoinedPlatform"></a>

User has not joined the platform


<pre><code><b>const</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EUserNotJoinedPlatform">EUserNotJoinedPlatform</a>: u64 = 23;
</code></pre>



<a name="social_contracts_social_proof_tokens_EUserBlockedByPlatform"></a>

User is blocked by the platform


<pre><code><b>const</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EUserBlockedByPlatform">EUserBlockedByPlatform</a>: u64 = 24;
</code></pre>



<a name="social_contracts_social_proof_tokens_EReservationPoolConverted"></a>

Reservation pool already converted to token


<pre><code><b>const</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EReservationPoolConverted">EReservationPoolConverted</a>: u64 = 25;
</code></pre>



<a name="social_contracts_social_proof_tokens_EAlreadyOwnsTokens"></a>

User already owns tokens for this pool


<pre><code><b>const</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EAlreadyOwnsTokens">EAlreadyOwnsTokens</a>: u64 = 26;
</code></pre>



<a name="social_contracts_social_proof_tokens_ETooManyReservers"></a>

Too many reservers for conversion (DoS prevention)


<pre><code><b>const</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ETooManyReservers">ETooManyReservers</a>: u64 = 27;
</code></pre>



<a name="social_contracts_social_proof_tokens_ECannotSplit"></a>

Cannot split token - amount must be positive and less than token amount


<pre><code><b>const</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ECannotSplit">ECannotSplit</a>: u64 = 28;
</code></pre>



<a name="social_contracts_social_proof_tokens_ECannotMerge"></a>

Cannot merge tokens - tokens must be from the same pool


<pre><code><b>const</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ECannotMerge">ECannotMerge</a>: u64 = 29;
</code></pre>



<a name="social_contracts_social_proof_tokens_EPostPoolEscrowTradingBlocked"></a>

Post token pools in on-post PoC escrow mode require an entrypoint that supplies <code>&Post</code> for PoC-aware fee routing.


<pre><code><b>const</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EPostPoolEscrowTradingBlocked">EPostPoolEscrowTradingBlocked</a>: u64 = 30;
</code></pre>



<a name="social_contracts_social_proof_tokens_ESamePool"></a>

Cannot swap a pool into itself


<pre><code><b>const</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ESamePool">ESamePool</a>: u64 = 31;
</code></pre>



<a name="social_contracts_social_proof_tokens_ESlippageExceeded"></a>

Dest fill below <code>min_dest_amount</code> slippage bound


<pre><code><b>const</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ESlippageExceeded">ESlippageExceeded</a>: u64 = 32;
</code></pre>



<a name="social_contracts_social_proof_tokens_ESelfTransfer"></a>

Cannot transfer SPT to the same address


<pre><code><b>const</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ESelfTransfer">ESelfTransfer</a>: u64 = 33;
</code></pre>



<a name="social_contracts_social_proof_tokens_ESptAlreadyEnabled"></a>

Post already has SPT / reservation pool enabled


<pre><code><b>const</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ESptAlreadyEnabled">ESptAlreadyEnabled</a>: u64 = 34;
</code></pre>



<a name="social_contracts_social_proof_tokens_EInvalidTransferAmount"></a>

Transfer amount must be positive


<pre><code><b>const</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInvalidTransferAmount">EInvalidTransferAmount</a>: u64 = 35;
</code></pre>



<a name="social_contracts_social_proof_tokens_TOKEN_TYPE_PROFILE"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TOKEN_TYPE_PROFILE">TOKEN_TYPE_PROFILE</a>: u8 = 1;
</code></pre>



<a name="social_contracts_social_proof_tokens_TOKEN_TYPE_POST"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TOKEN_TYPE_POST">TOKEN_TYPE_POST</a>: u8 = 2;
</code></pre>



<a name="social_contracts_social_proof_tokens_DEFAULT_TRADING_CREATOR_FEE_BPS"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_DEFAULT_TRADING_CREATOR_FEE_BPS">DEFAULT_TRADING_CREATOR_FEE_BPS</a>: u64 = 100;
</code></pre>



<a name="social_contracts_social_proof_tokens_DEFAULT_TRADING_PLATFORM_FEE_BPS"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_DEFAULT_TRADING_PLATFORM_FEE_BPS">DEFAULT_TRADING_PLATFORM_FEE_BPS</a>: u64 = 25;
</code></pre>



<a name="social_contracts_social_proof_tokens_DEFAULT_TRADING_TREASURY_FEE_BPS"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_DEFAULT_TRADING_TREASURY_FEE_BPS">DEFAULT_TRADING_TREASURY_FEE_BPS</a>: u64 = 25;
</code></pre>



<a name="social_contracts_social_proof_tokens_DEFAULT_RESERVATION_CREATOR_FEE_BPS"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_DEFAULT_RESERVATION_CREATOR_FEE_BPS">DEFAULT_RESERVATION_CREATOR_FEE_BPS</a>: u64 = 100;
</code></pre>



<a name="social_contracts_social_proof_tokens_DEFAULT_RESERVATION_PLATFORM_FEE_BPS"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_DEFAULT_RESERVATION_PLATFORM_FEE_BPS">DEFAULT_RESERVATION_PLATFORM_FEE_BPS</a>: u64 = 25;
</code></pre>



<a name="social_contracts_social_proof_tokens_DEFAULT_RESERVATION_TREASURY_FEE_BPS"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_DEFAULT_RESERVATION_TREASURY_FEE_BPS">DEFAULT_RESERVATION_TREASURY_FEE_BPS</a>: u64 = 25;
</code></pre>



<a name="social_contracts_social_proof_tokens_DEFAULT_NON_PLATFORM_PLATFORM_TO_CREATOR_BPS"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_DEFAULT_NON_PLATFORM_PLATFORM_TO_CREATOR_BPS">DEFAULT_NON_PLATFORM_PLATFORM_TO_CREATOR_BPS</a>: u64 = 5000;
</code></pre>



<a name="social_contracts_social_proof_tokens_DEFAULT_NON_PLATFORM_PLATFORM_TO_TREASURY_BPS"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_DEFAULT_NON_PLATFORM_PLATFORM_TO_TREASURY_BPS">DEFAULT_NON_PLATFORM_PLATFORM_TO_TREASURY_BPS</a>: u64 = 5000;
</code></pre>



<a name="social_contracts_social_proof_tokens_MAX_HOLD_PERCENT_BPS"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_MAX_HOLD_PERCENT_BPS">MAX_HOLD_PERCENT_BPS</a>: u64 = 500;
</code></pre>



<a name="social_contracts_social_proof_tokens_BPS_DENOM"></a>

**Permyriad / fee scale only:** <code>10_000</code> = 100%. Not SPT decimal places (<code><a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SPT_DECIMALS">SPT_DECIMALS</a></code> = 9).


<pre><code><b>const</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_BPS_DENOM">BPS_DENOM</a>: u64 = 10000;
</code></pre>



<a name="social_contracts_social_proof_tokens_MAX_ONCHAIN_U64_U128"></a>

Numeric value of <code><a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_MAX_U64">MAX_U64</a></code>; max storable on-chain amount as <code>u128</code> for safe compares / products.


<pre><code><b>const</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_MAX_ONCHAIN_U64_U128">MAX_ONCHAIN_U64_U128</a>: u128 = 18446744073709551615;
</code></pre>



<a name="social_contracts_social_proof_tokens_MAX_RESERVATION_THRESHOLD_BPS_PRODUCT_U128"></a>

Max <code>threshold * max_individual_reservation_bps</code> so <code>(product / <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_BPS_DENOM">BPS_DENOM</a>)</code> fits <code>u64</code> (equals <code><a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_MAX_U64">MAX_U64</a> * <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_BPS_DENOM">BPS_DENOM</a></code>).


<pre><code><b>const</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_MAX_RESERVATION_THRESHOLD_BPS_PRODUCT_U128">MAX_RESERVATION_THRESHOLD_BPS_PRODUCT_U128</a>: u128 = 184467440737095516150000;
</code></pre>



<a name="social_contracts_social_proof_tokens_MAX_SPT_ADMIN_PERCENT_BPS"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_MAX_SPT_ADMIN_PERCENT_BPS">MAX_SPT_ADMIN_PERCENT_BPS</a>: u64 = 1000000;
</code></pre>



<a name="social_contracts_social_proof_tokens_MAX_RESERVERS_PER_POOL_CAP"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_MAX_RESERVERS_PER_POOL_CAP">MAX_RESERVERS_PER_POOL_CAP</a>: u64 = 10000000;
</code></pre>



<a name="social_contracts_social_proof_tokens_SPT_DECIMALS"></a>

Display decimals for SPT (matches native MYSO coin metadata).


<pre><code><b>const</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SPT_DECIMALS">SPT_DECIMALS</a>: u8 = 9;
</code></pre>



<a name="social_contracts_social_proof_tokens_SPT_SCALE"></a>

Nano-SPT per 1.0 whole display token.


<pre><code><b>const</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SPT_SCALE">SPT_SCALE</a>: u64 = 1000000000;
</code></pre>



<a name="social_contracts_social_proof_tokens_DEFAULT_BASE_PRICE"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_DEFAULT_BASE_PRICE">DEFAULT_BASE_PRICE</a>: u64 = 1000000000;
</code></pre>



<a name="social_contracts_social_proof_tokens_DEFAULT_QUADRATIC_COEFFICIENT"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_DEFAULT_QUADRATIC_COEFFICIENT">DEFAULT_QUADRATIC_COEFFICIENT</a>: u64 = 100000;
</code></pre>



<a name="social_contracts_social_proof_tokens_DEFAULT_POST_THRESHOLD"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_DEFAULT_POST_THRESHOLD">DEFAULT_POST_THRESHOLD</a>: u64 = 1000000000000;
</code></pre>



<a name="social_contracts_social_proof_tokens_DEFAULT_PROFILE_THRESHOLD"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_DEFAULT_PROFILE_THRESHOLD">DEFAULT_PROFILE_THRESHOLD</a>: u64 = 10000000000000;
</code></pre>



<a name="social_contracts_social_proof_tokens_DEFAULT_MAX_INDIVIDUAL_RESERVATION_BPS"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_DEFAULT_MAX_INDIVIDUAL_RESERVATION_BPS">DEFAULT_MAX_INDIVIDUAL_RESERVATION_BPS</a>: u64 = 2000;
</code></pre>



<a name="social_contracts_social_proof_tokens_DEFAULT_MAX_RESERVERS_PER_POOL"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_DEFAULT_MAX_RESERVERS_PER_POOL">DEFAULT_MAX_RESERVERS_PER_POOL</a>: u64 = 1000;
</code></pre>



<a name="social_contracts_social_proof_tokens_MAX_U64"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_MAX_U64">MAX_U64</a>: u64 = 18446744073709551615;
</code></pre>



<a name="social_contracts_social_proof_tokens_split_social_token"></a>

## Function `split_social_token`

Split a SocialToken into two tokens
Returns a new SocialToken with the specified amount
The original token's amount is reduced by the split amount


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_split_social_token">split_social_token</a>(token: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialToken">social_contracts::social_proof_tokens::SocialToken</a>, split_amount: u64, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialToken">social_contracts::social_proof_tokens::SocialToken</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_split_social_token">split_social_token</a>(
    token: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialToken">SocialToken</a>,
    split_amount: u64,
    ctx: &<b>mut</b> TxContext
): <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialToken">SocialToken</a> {
    // Validation
    <b>assert</b>!(split_amount &gt; 0, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ECannotSplit">ECannotSplit</a>);
    <b>assert</b>!(token.amount &gt;= split_amount, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInsufficientFunds">EInsufficientFunds</a>);
    <b>assert</b>!(split_amount &lt; token.amount, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ECannotSplit">ECannotSplit</a>);
    // Update original token
    token.amount = token.amount - split_amount;
    // Create new token
    <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialToken">SocialToken</a> {
        id: object::new(ctx),
        pool_id: token.pool_id,
        token_type: token.token_type,
        amount: split_amount,
    }
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_merge_social_tokens"></a>

## Function `merge_social_tokens`

Merge two SocialTokens from the same pool
Consumes the second token and adds its amount to the first
Both tokens must have the same pool_id and token_type


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_merge_social_tokens">merge_social_tokens</a>(token1: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialToken">social_contracts::social_proof_tokens::SocialToken</a>, token2: <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialToken">social_contracts::social_proof_tokens::SocialToken</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_merge_social_tokens">merge_social_tokens</a>(
    token1: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialToken">SocialToken</a>,
    token2: <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialToken">SocialToken</a>
) {
    // Validation
    <b>assert</b>!(token1.pool_id == token2.pool_id, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ECannotMerge">ECannotMerge</a>);
    <b>assert</b>!(token1.token_type == token2.token_type, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ECannotMerge">ECannotMerge</a>);
    <b>assert</b>!(token1.amount &lt;= <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_MAX_U64">MAX_U64</a> - token2.amount, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EOverflow">EOverflow</a>);
    // Merge amounts
    token1.amount = token1.amount + token2.amount;
    // Destroy second token's ID
    <b>let</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialToken">SocialToken</a> { id, pool_id: _, token_type: _, amount: _ } = token2;
    object::delete(id);
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_split_social_token_entry"></a>

## Function `split_social_token_entry`

Entry function to split a SocialToken


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_split_social_token_entry">split_social_token_entry</a>(token: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialToken">social_contracts::social_proof_tokens::SocialToken</a>, split_amount: u64, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_split_social_token_entry">split_social_token_entry</a>(
    token: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialToken">SocialToken</a>,
    split_amount: u64,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> new_token = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_split_social_token">split_social_token</a>(token, split_amount, ctx);
    transfer::transfer(new_token, tx_context::sender(ctx));
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_merge_social_tokens_entry"></a>

## Function `merge_social_tokens_entry`

Entry function to merge two SocialTokens


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_merge_social_tokens_entry">merge_social_tokens_entry</a>(token1: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialToken">social_contracts::social_proof_tokens::SocialToken</a>, token2: <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialToken">social_contracts::social_proof_tokens::SocialToken</a>, _ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_merge_social_tokens_entry">merge_social_tokens_entry</a>(
    token1: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialToken">SocialToken</a>,
    token2: <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialToken">SocialToken</a>,
    _ctx: &<b>mut</b> TxContext
) {
    <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_merge_social_tokens">merge_social_tokens</a>(token1, token2);
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_bootstrap_init"></a>

## Function `bootstrap_init`

Bootstrap initialization function - creates the social proof tokens configuration and registry


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_bootstrap_init">bootstrap_init</a>(clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_bootstrap_init">bootstrap_init</a>(clock: &Clock, ctx: &<b>mut</b> TxContext) {
    <b>let</b> sender = tx_context::sender(ctx);
    <b>let</b> config = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">SocialProofTokensConfig</a> {
        id: object::new(ctx),
        version: <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(),
        trading_creator_fee_bps: <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_DEFAULT_TRADING_CREATOR_FEE_BPS">DEFAULT_TRADING_CREATOR_FEE_BPS</a>,
        trading_platform_fee_bps: <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_DEFAULT_TRADING_PLATFORM_FEE_BPS">DEFAULT_TRADING_PLATFORM_FEE_BPS</a>,
        trading_treasury_fee_bps: <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_DEFAULT_TRADING_TREASURY_FEE_BPS">DEFAULT_TRADING_TREASURY_FEE_BPS</a>,
        reservation_creator_fee_bps: <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_DEFAULT_RESERVATION_CREATOR_FEE_BPS">DEFAULT_RESERVATION_CREATOR_FEE_BPS</a>,
        reservation_platform_fee_bps: <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_DEFAULT_RESERVATION_PLATFORM_FEE_BPS">DEFAULT_RESERVATION_PLATFORM_FEE_BPS</a>,
        reservation_treasury_fee_bps: <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_DEFAULT_RESERVATION_TREASURY_FEE_BPS">DEFAULT_RESERVATION_TREASURY_FEE_BPS</a>,
        base_price: <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_DEFAULT_BASE_PRICE">DEFAULT_BASE_PRICE</a>,
        quadratic_coefficient: <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_DEFAULT_QUADRATIC_COEFFICIENT">DEFAULT_QUADRATIC_COEFFICIENT</a>,
        max_hold_percent_bps: <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_MAX_HOLD_PERCENT_BPS">MAX_HOLD_PERCENT_BPS</a>,
        post_threshold: <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_DEFAULT_POST_THRESHOLD">DEFAULT_POST_THRESHOLD</a>,
        profile_threshold: <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_DEFAULT_PROFILE_THRESHOLD">DEFAULT_PROFILE_THRESHOLD</a>,
        max_individual_reservation_bps: <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_DEFAULT_MAX_INDIVIDUAL_RESERVATION_BPS">DEFAULT_MAX_INDIVIDUAL_RESERVATION_BPS</a>,
        max_reservers_per_pool: <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_DEFAULT_MAX_RESERVERS_PER_POOL">DEFAULT_MAX_RESERVERS_PER_POOL</a>,
        trading_enabled: <b>true</b>,
        non_platform_platform_to_creator_bps: <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_DEFAULT_NON_PLATFORM_PLATFORM_TO_CREATOR_BPS">DEFAULT_NON_PLATFORM_PLATFORM_TO_CREATOR_BPS</a>,
        non_platform_platform_to_treasury_bps: <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_DEFAULT_NON_PLATFORM_PLATFORM_TO_TREASURY_BPS">DEFAULT_NON_PLATFORM_PLATFORM_TO_TREASURY_BPS</a>,
    };
    // Emit event so indexer can populate spt_exchange_config table
    <b>let</b> total_fee_bps = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_DEFAULT_TRADING_CREATOR_FEE_BPS">DEFAULT_TRADING_CREATOR_FEE_BPS</a> + <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_DEFAULT_TRADING_PLATFORM_FEE_BPS">DEFAULT_TRADING_PLATFORM_FEE_BPS</a> + <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_DEFAULT_TRADING_TREASURY_FEE_BPS">DEFAULT_TRADING_TREASURY_FEE_BPS</a>;
    <b>let</b> reservation_total_fee_bps = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_DEFAULT_RESERVATION_CREATOR_FEE_BPS">DEFAULT_RESERVATION_CREATOR_FEE_BPS</a> + <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_DEFAULT_RESERVATION_PLATFORM_FEE_BPS">DEFAULT_RESERVATION_PLATFORM_FEE_BPS</a> + <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_DEFAULT_RESERVATION_TREASURY_FEE_BPS">DEFAULT_RESERVATION_TREASURY_FEE_BPS</a>;
    event::emit(<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ConfigUpdatedEvent">ConfigUpdatedEvent</a> {
        updated_by: sender,
        timestamp: clock::timestamp_ms(clock),
        total_fee_bps,
        trading_creator_fee_bps: <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_DEFAULT_TRADING_CREATOR_FEE_BPS">DEFAULT_TRADING_CREATOR_FEE_BPS</a>,
        trading_platform_fee_bps: <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_DEFAULT_TRADING_PLATFORM_FEE_BPS">DEFAULT_TRADING_PLATFORM_FEE_BPS</a>,
        trading_treasury_fee_bps: <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_DEFAULT_TRADING_TREASURY_FEE_BPS">DEFAULT_TRADING_TREASURY_FEE_BPS</a>,
        reservation_total_fee_bps,
        reservation_creator_fee_bps: <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_DEFAULT_RESERVATION_CREATOR_FEE_BPS">DEFAULT_RESERVATION_CREATOR_FEE_BPS</a>,
        reservation_platform_fee_bps: <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_DEFAULT_RESERVATION_PLATFORM_FEE_BPS">DEFAULT_RESERVATION_PLATFORM_FEE_BPS</a>,
        reservation_treasury_fee_bps: <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_DEFAULT_RESERVATION_TREASURY_FEE_BPS">DEFAULT_RESERVATION_TREASURY_FEE_BPS</a>,
        base_price: <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_DEFAULT_BASE_PRICE">DEFAULT_BASE_PRICE</a>,
        quadratic_coefficient: <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_DEFAULT_QUADRATIC_COEFFICIENT">DEFAULT_QUADRATIC_COEFFICIENT</a>,
        max_hold_percent_bps: <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_MAX_HOLD_PERCENT_BPS">MAX_HOLD_PERCENT_BPS</a>,
        post_threshold: <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_DEFAULT_POST_THRESHOLD">DEFAULT_POST_THRESHOLD</a>,
        profile_threshold: <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_DEFAULT_PROFILE_THRESHOLD">DEFAULT_PROFILE_THRESHOLD</a>,
        max_individual_reservation_bps: <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_DEFAULT_MAX_INDIVIDUAL_RESERVATION_BPS">DEFAULT_MAX_INDIVIDUAL_RESERVATION_BPS</a>,
        max_reservers_per_pool: <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_DEFAULT_MAX_RESERVERS_PER_POOL">DEFAULT_MAX_RESERVERS_PER_POOL</a>,
        non_platform_platform_to_creator_bps: <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_DEFAULT_NON_PLATFORM_PLATFORM_TO_CREATOR_BPS">DEFAULT_NON_PLATFORM_PLATFORM_TO_CREATOR_BPS</a>,
        non_platform_platform_to_treasury_bps: <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_DEFAULT_NON_PLATFORM_PLATFORM_TO_TREASURY_BPS">DEFAULT_NON_PLATFORM_PLATFORM_TO_TREASURY_BPS</a>,
        trading_enabled: <b>true</b>,
    });
    // Create and share social proof tokens config with proper treasury
    transfer::share_object(config);
    // Create and share token registry
    transfer::share_object(
        <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">TokenRegistry</a> {
            id: object::new(ctx),
            tokens: table::new(ctx),
            reservation_pools: table::new(ctx),
            version: <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(),
        }
    );
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_update_social_proof_tokens_config"></a>

## Function `update_social_proof_tokens_config`

Update social proof tokens configuration


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_update_social_proof_tokens_config">update_social_proof_tokens_config</a>(_admin_cap: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensAdminCap">social_contracts::social_proof_tokens::SocialProofTokensAdminCap</a>, config: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">social_contracts::social_proof_tokens::SocialProofTokensConfig</a>, trading_creator_fee_bps: u64, trading_platform_fee_bps: u64, trading_treasury_fee_bps: u64, reservation_creator_fee_bps: u64, reservation_platform_fee_bps: u64, reservation_treasury_fee_bps: u64, base_price: u64, quadratic_coefficient: u64, max_hold_percent_bps: u64, post_threshold: u64, profile_threshold: u64, max_individual_reservation_bps: u64, max_reservers_per_pool: u64, non_platform_platform_to_creator_bps: u64, non_platform_platform_to_treasury_bps: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_update_social_proof_tokens_config">update_social_proof_tokens_config</a>(
    _admin_cap: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensAdminCap">SocialProofTokensAdminCap</a>,
    config: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">SocialProofTokensConfig</a>,
    trading_creator_fee_bps: u64,
    trading_platform_fee_bps: u64,
    trading_treasury_fee_bps: u64,
    reservation_creator_fee_bps: u64,
    reservation_platform_fee_bps: u64,
    reservation_treasury_fee_bps: u64,
    base_price: u64,
    quadratic_coefficient: u64,
    max_hold_percent_bps: u64,
    post_threshold: u64,
    profile_threshold: u64,
    max_individual_reservation_bps: u64,
    max_reservers_per_pool: u64,
    non_platform_platform_to_creator_bps: u64,
    non_platform_platform_to_treasury_bps: u64,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    // Verify curve parameters are valid
    <b>assert</b>!(base_price &gt; 0 && quadratic_coefficient &gt; 0, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInvalidCurveParams">EInvalidCurveParams</a>);
    // Validate fee configurations to prevent division by zero and overflow
    // Calculate totals before updating to validate
    <b>let</b> total_fee_bps = trading_creator_fee_bps + trading_platform_fee_bps + trading_treasury_fee_bps;
    <b>let</b> reservation_total_fee_bps = reservation_creator_fee_bps + reservation_platform_fee_bps + reservation_treasury_fee_bps;
    // Ensure fee totals are valid (prevent division by zero and overflow)
    <b>assert</b>!(total_fee_bps &gt; 0 && total_fee_bps &lt;= <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_BPS_DENOM">BPS_DENOM</a>, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInvalidFeeConfig">EInvalidFeeConfig</a>);
    <b>assert</b>!(reservation_total_fee_bps &gt; 0 && reservation_total_fee_bps &lt;= <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_BPS_DENOM">BPS_DENOM</a>, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInvalidFeeConfig">EInvalidFeeConfig</a>);
    // Validate individual fee components don't exceed 100%
    // Hold cap: bps may exceed 10_000 (100%) so whales can hold &gt;100% of supply <b>if</b> policy allows.
    <b>assert</b>!(max_hold_percent_bps &gt; 0 && max_hold_percent_bps &lt;= <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_MAX_SPT_ADMIN_PERCENT_BPS">MAX_SPT_ADMIN_PERCENT_BPS</a>, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInvalidFeeConfig">EInvalidFeeConfig</a>);
    // Validate thresholds (must be positive)
    <b>assert</b>!(post_threshold &gt; 0, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInvalidFeeConfig">EInvalidFeeConfig</a>);
    <b>assert</b>!(profile_threshold &gt; 0, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInvalidFeeConfig">EInvalidFeeConfig</a>);
    <b>assert</b>!(max_individual_reservation_bps &gt; 0 && max_individual_reservation_bps &lt;= <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_MAX_SPT_ADMIN_PERCENT_BPS">MAX_SPT_ADMIN_PERCENT_BPS</a>, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInvalidFeeConfig">EInvalidFeeConfig</a>);
    // `(threshold * bps) / <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_BPS_DENOM">BPS_DENOM</a>` must fit `u64` (reservation cap in MYSO smallest units).
    <b>assert</b>!(
        (post_threshold <b>as</b> u128) * (max_individual_reservation_bps <b>as</b> u128)
            &lt;= <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_MAX_RESERVATION_THRESHOLD_BPS_PRODUCT_U128">MAX_RESERVATION_THRESHOLD_BPS_PRODUCT_U128</a>,
        <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInvalidFeeConfig">EInvalidFeeConfig</a>
    );
    <b>assert</b>!(
        (profile_threshold <b>as</b> u128) * (max_individual_reservation_bps <b>as</b> u128)
            &lt;= <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_MAX_RESERVATION_THRESHOLD_BPS_PRODUCT_U128">MAX_RESERVATION_THRESHOLD_BPS_PRODUCT_U128</a>,
        <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInvalidFeeConfig">EInvalidFeeConfig</a>
    );
    <b>assert</b>!(
        max_reservers_per_pool &gt; 0 && max_reservers_per_pool &lt;= <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_MAX_RESERVERS_PER_POOL_CAP">MAX_RESERVERS_PER_POOL_CAP</a>,
        <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInvalidFeeConfig">EInvalidFeeConfig</a>
    );
    <b>assert</b>!(trading_creator_fee_bps &lt;= <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_BPS_DENOM">BPS_DENOM</a>, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInvalidFeeConfig">EInvalidFeeConfig</a>);
    <b>assert</b>!(trading_platform_fee_bps &lt;= <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_BPS_DENOM">BPS_DENOM</a>, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInvalidFeeConfig">EInvalidFeeConfig</a>);
    <b>assert</b>!(trading_treasury_fee_bps &lt;= <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_BPS_DENOM">BPS_DENOM</a>, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInvalidFeeConfig">EInvalidFeeConfig</a>);
    <b>assert</b>!(reservation_creator_fee_bps &lt;= <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_BPS_DENOM">BPS_DENOM</a>, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInvalidFeeConfig">EInvalidFeeConfig</a>);
    <b>assert</b>!(reservation_platform_fee_bps &lt;= <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_BPS_DENOM">BPS_DENOM</a>, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInvalidFeeConfig">EInvalidFeeConfig</a>);
    <b>assert</b>!(reservation_treasury_fee_bps &lt;= <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_BPS_DENOM">BPS_DENOM</a>, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInvalidFeeConfig">EInvalidFeeConfig</a>);
    // Non-<a href="../social_contracts/platform.md#social_contracts_platform">platform</a> split must partition the <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>-fee bucket exactly (sum == <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_BPS_DENOM">BPS_DENOM</a>).
    <b>assert</b>!(non_platform_platform_to_creator_bps &lt;= <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_BPS_DENOM">BPS_DENOM</a>, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInvalidFeeConfig">EInvalidFeeConfig</a>);
    <b>assert</b>!(non_platform_platform_to_treasury_bps &lt;= <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_BPS_DENOM">BPS_DENOM</a>, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInvalidFeeConfig">EInvalidFeeConfig</a>);
    <b>assert</b>!(
        non_platform_platform_to_creator_bps + non_platform_platform_to_treasury_bps == <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_BPS_DENOM">BPS_DENOM</a>,
        <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInvalidFeeConfig">EInvalidFeeConfig</a>,
    );
    // Update trading fee config
    config.trading_creator_fee_bps = trading_creator_fee_bps;
    config.trading_platform_fee_bps = trading_platform_fee_bps;
    config.trading_treasury_fee_bps = trading_treasury_fee_bps;
    // Update reservation fee config
    config.reservation_creator_fee_bps = reservation_creator_fee_bps;
    config.reservation_platform_fee_bps = reservation_platform_fee_bps;
    config.reservation_treasury_fee_bps = reservation_treasury_fee_bps;
    // Update curve parameters
    config.base_price = base_price;
    config.quadratic_coefficient = quadratic_coefficient;
    // Update max hold percentage
    config.max_hold_percent_bps = max_hold_percent_bps;
    // Update reservation thresholds
    config.post_threshold = post_threshold;
    config.profile_threshold = profile_threshold;
    config.max_individual_reservation_bps = max_individual_reservation_bps;
    config.max_reservers_per_pool = max_reservers_per_pool;
    config.non_platform_platform_to_creator_bps = non_platform_platform_to_creator_bps;
    config.non_platform_platform_to_treasury_bps = non_platform_platform_to_treasury_bps;
    // Calculate totals <b>for</b> event emission
    <b>let</b> total_fee_bps = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_total_fee_bps">calculate_total_fee_bps</a>(config);
    <b>let</b> reservation_total_fee_bps = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_reservation_total_fee_bps">calculate_reservation_total_fee_bps</a>(config);
    // Emit config updated event
    event::emit(<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ConfigUpdatedEvent">ConfigUpdatedEvent</a> {
        updated_by: tx_context::sender(ctx),
        timestamp: clock::timestamp_ms(clock),
        total_fee_bps,
        trading_creator_fee_bps,
        trading_platform_fee_bps,
        trading_treasury_fee_bps,
        reservation_total_fee_bps,
        reservation_creator_fee_bps,
        reservation_platform_fee_bps,
        reservation_treasury_fee_bps,
        base_price,
        quadratic_coefficient,
        max_hold_percent_bps,
        post_threshold,
        profile_threshold,
        max_individual_reservation_bps,
        max_reservers_per_pool,
        non_platform_platform_to_creator_bps,
        non_platform_platform_to_treasury_bps,
        trading_enabled: config.trading_enabled,
    });
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_toggle_emergency_kill_switch"></a>

## Function `toggle_emergency_kill_switch`

Emergency kill switch function - only callable by admin
This function can immediately enable or halt all trading on the platform


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_toggle_emergency_kill_switch">toggle_emergency_kill_switch</a>(_admin_cap: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensAdminCap">social_contracts::social_proof_tokens::SocialProofTokensAdminCap</a>, config: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">social_contracts::social_proof_tokens::SocialProofTokensConfig</a>, enable_trading: bool, reason: vector&lt;u8&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_toggle_emergency_kill_switch">toggle_emergency_kill_switch</a>(
    _admin_cap: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensAdminCap">SocialProofTokensAdminCap</a>,
    config: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">SocialProofTokensConfig</a>,
    enable_trading: bool,
    reason: vector&lt;u8&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    // Update the trading enabled status
    config.trading_enabled = enable_trading;
    // Emit event <b>for</b> audit trail
    event::emit(<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EmergencyKillSwitchEvent">EmergencyKillSwitchEvent</a> {
        admin: tx_context::sender(ctx),
        trading_enabled: enable_trading,
        timestamp: clock::timestamp_ms(clock),
        reason: string::utf8(reason),
    });
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_is_trading_enabled"></a>

## Function `is_trading_enabled`

Check if trading is currently enabled


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_is_trading_enabled">is_trading_enabled</a>(config: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">social_contracts::social_proof_tokens::SocialProofTokensConfig</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_is_trading_enabled">is_trading_enabled</a>(config: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">SocialProofTokensConfig</a>): bool {
    config.trading_enabled
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_calculate_total_fee_bps"></a>

## Function `calculate_total_fee_bps`

Calculate total trading fee from component fees


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_total_fee_bps">calculate_total_fee_bps</a>(config: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">social_contracts::social_proof_tokens::SocialProofTokensConfig</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_total_fee_bps">calculate_total_fee_bps</a>(config: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">SocialProofTokensConfig</a>): u64 {
    config.trading_creator_fee_bps + config.trading_platform_fee_bps + config.trading_treasury_fee_bps
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_calculate_reservation_total_fee_bps"></a>

## Function `calculate_reservation_total_fee_bps`

Calculate total reservation fee from component fees


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_reservation_total_fee_bps">calculate_reservation_total_fee_bps</a>(config: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">social_contracts::social_proof_tokens::SocialProofTokensConfig</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_reservation_total_fee_bps">calculate_reservation_total_fee_bps</a>(config: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">SocialProofTokensConfig</a>): u64 {
    config.reservation_creator_fee_bps + config.reservation_platform_fee_bps + config.reservation_treasury_fee_bps
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_validate_trading_fees"></a>

## Function `validate_trading_fees`

Validate trading fee config before use


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_validate_trading_fees">validate_trading_fees</a>(config: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">social_contracts::social_proof_tokens::SocialProofTokensConfig</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_validate_trading_fees">validate_trading_fees</a>(config: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">SocialProofTokensConfig</a>) {
    <b>let</b> total_fee_bps = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_total_fee_bps">calculate_total_fee_bps</a>(config);
    <b>assert</b>!(total_fee_bps &gt; 0 && total_fee_bps &lt;= <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_BPS_DENOM">BPS_DENOM</a>, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInvalidFeeConfig">EInvalidFeeConfig</a>);
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_validate_reservation_fees"></a>

## Function `validate_reservation_fees`

Validate reservation fee config before use


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_validate_reservation_fees">validate_reservation_fees</a>(config: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">social_contracts::social_proof_tokens::SocialProofTokensConfig</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_validate_reservation_fees">validate_reservation_fees</a>(config: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">SocialProofTokensConfig</a>) {
    <b>let</b> reservation_total_fee_bps = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_reservation_total_fee_bps">calculate_reservation_total_fee_bps</a>(config);
    <b>assert</b>!(reservation_total_fee_bps &gt; 0 && reservation_total_fee_bps &lt;= <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_BPS_DENOM">BPS_DENOM</a>, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInvalidFeeConfig">EInvalidFeeConfig</a>);
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_calculate_fee_amount_safe"></a>

## Function `calculate_fee_amount_safe`

Calculate fee amount with overflow protection
amount * fee_bps can overflow before division, so check first


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_fee_amount_safe">calculate_fee_amount_safe</a>(amount: u64, fee_bps: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_fee_amount_safe">calculate_fee_amount_safe</a>(amount: u64, fee_bps: u64): u64 {
    // Overflow protection: amount * fee_bps can overflow before division
    <b>if</b> (amount == 0 || fee_bps == 0) {
        <b>return</b> 0
    };
    <b>assert</b>!(amount &lt;= <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_MAX_U64">MAX_U64</a> / fee_bps, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EOverflow">EOverflow</a>);
    (amount * fee_bps) / <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_BPS_DENOM">BPS_DENOM</a>
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_calculate_component_fee_safe"></a>

## Function `calculate_component_fee_safe`

Calculate component fee from total fee amount
component_fee = (fee_amount * component_bps) / total_bps
With overflow protection


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_component_fee_safe">calculate_component_fee_safe</a>(fee_amount: u64, component_bps: u64, total_bps: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_component_fee_safe">calculate_component_fee_safe</a>(fee_amount: u64, component_bps: u64, total_bps: u64): u64 {
    <b>if</b> (fee_amount == 0 || component_bps == 0 || total_bps == 0) {
        <b>return</b> 0
    };
    // Overflow protection: fee_amount * component_bps can overflow
    <b>assert</b>!(fee_amount &lt;= <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_MAX_U64">MAX_U64</a> / component_bps, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EOverflow">EOverflow</a>);
    (fee_amount * component_bps) / total_bps
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_reserve_towards_post"></a>

## Function `reserve_towards_post`

Reserve MYSO tokens towards a post to support social proof token creation
Anyone can call this function - the post owner is stored in the reservation pool
Reserve MYSO tokens towards a post to support social proof token creation
Non-platform version: platform fees go to ecosystem treasury


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_reserve_towards_post">reserve_towards_post</a>(registry: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">social_contracts::social_proof_tokens::TokenRegistry</a>, config: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">social_contracts::social_proof_tokens::SocialProofTokensConfig</a>, min_vault_deposit_amount: u64, reservation_pool_object: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ReservationPoolObject">social_contracts::social_proof_tokens::ReservationPoolObject</a>, treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, beneficiary_vault: &<b>mut</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_PoCBeneficiaryVault">social_contracts::poc_vault::PoCBeneficiaryVault</a>, payment: <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, amount: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_reserve_towards_post">reserve_towards_post</a>(
    registry: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">TokenRegistry</a>,
    config: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">SocialProofTokensConfig</a>,
    min_vault_deposit_amount: u64,
    reservation_pool_object: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ReservationPoolObject">ReservationPoolObject</a>,
    treasury: &EcosystemTreasury,
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &Post,
    beneficiary_vault: &<b>mut</b> PoCBeneficiaryVault,
    payment: Coin&lt;MYSO&gt;,
    amount: u64,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    // Check <b>if</b> trading is halted
    <b>assert</b>!(config.trading_enabled, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ETradingHalted">ETradingHalted</a>);
    // Prevent reservations after conversion to token
    <b>assert</b>!(!reservation_pool_object.converted, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EReservationPoolConverted">EReservationPoolConverted</a>);
    <b>let</b> reserver = tx_context::sender(ctx);
    // Get <a href="../social_contracts/post.md#social_contracts_post">post</a> ID and owner from reservation pool
    <b>let</b> post_id = reservation_pool_object.info.associated_id;
    <b>let</b> post_owner = reservation_pool_object.info.owner;
    <b>let</b> now = clock::timestamp_ms(clock);
    // Verify reservation pool is <b>for</b> a <a href="../social_contracts/post.md#social_contracts_post">post</a>
    <b>assert</b>!(reservation_pool_object.info.token_type == <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TOKEN_TYPE_POST">TOKEN_TYPE_POST</a>, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInvalidTokenType">EInvalidTokenType</a>);
    // Verify <a href="../social_contracts/post.md#social_contracts_post">post</a> matches reservation pool
    <b>assert</b>!(<a href="../social_contracts/post.md#social_contracts_post_get_id_address">post::get_id_address</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>) == post_id, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInvalidID">EInvalidID</a>);
    // Ensure reserver <b>has</b> enough funds
    <b>assert</b>!(coin::value(&payment) &gt;= amount && amount &gt; 0, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInsufficientFunds">EInsufficientFunds</a>);
    // Calculate fees upfront based on desired reservation amount
    <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_validate_reservation_fees">validate_reservation_fees</a>(config);
    <b>let</b> reservation_total_fee_bps = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_reservation_total_fee_bps">calculate_reservation_total_fee_bps</a>(config);
    <b>let</b> fee_amount = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_fee_amount_safe">calculate_fee_amount_safe</a>(amount, reservation_total_fee_bps);
    // Determine <b>if</b> fees should be on top or deducted from amount
    <b>let</b> fees_on_top = coin::value(&payment) &gt;= amount + fee_amount;
    <b>let</b> net_amount = <b>if</b> (fees_on_top) {
        // User <b>has</b> enough: reserve full amount, fees on top
        amount
    } <b>else</b> {
        // User doesn't have enough <b>for</b> fees on top: deduct fees from amount
        <b>assert</b>!(coin::value(&payment) &gt;= amount, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInsufficientFunds">EInsufficientFunds</a>);
        amount - fee_amount
    };
    // Calculate and distribute fees (non-<a href="../social_contracts/platform.md#social_contracts_platform">platform</a> version)
    // Fee distribution calculates fees from 'amount' and deducts from payment
    // When fees_on_top: payment <b>has</b> amount+fees, after distribution: remaining = amount (correct!)
    // When fees deducted: payment <b>has</b> amount, after distribution: remaining = amount - fees (correct!)
    <b>let</b> (<b>mut</b> remaining_payment, fee_amount, creator_fee, platform_fee, treasury_fee) = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_distribute_reservation_fees_with_post">distribute_reservation_fees_with_post</a>(
        config,
        min_vault_deposit_amount,
        reservation_pool_object,
        <a href="../social_contracts/post.md#social_contracts_post">post</a>,
        beneficiary_vault,
        amount,
        payment,
        treasury,
        clock,
        ctx
    );
    // Check individual reservation limit (based on net amount)
    <b>let</b> max_individual_reservation = (config.post_threshold * config.max_individual_reservation_bps) / <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_BPS_DENOM">BPS_DENOM</a>;
    <b>let</b> current_reservation = <b>if</b> (table::contains(&reservation_pool_object.reservations, reserver)) {
        *table::borrow(&reservation_pool_object.reservations, reserver)
    } <b>else</b> {
        0
    };
    <b>assert</b>!(current_reservation + net_amount &lt;= max_individual_reservation, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EExceededMaxHold">EExceededMaxHold</a>);
    // Extract net reservation payment
    <b>let</b> reservation_payment = coin::split(&<b>mut</b> remaining_payment, net_amount, ctx);
    balance::join(&<b>mut</b> reservation_pool_object.myso_balance, coin::into_balance(reservation_payment));
    // Update reserver's balance in the pool (store net amount)
    <b>if</b> (table::contains(&reservation_pool_object.reservations, reserver)) {
        <b>let</b> reservation_balance = table::borrow_mut(&<b>mut</b> reservation_pool_object.reservations, reserver);
        <b>assert</b>!(*reservation_balance &lt;= <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_MAX_U64">MAX_U64</a> - net_amount, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EOverflow">EOverflow</a>);
        *reservation_balance = *reservation_balance + net_amount;
    } <b>else</b> {
        // DoS protection: limit number of unique reservers per pool
        <b>let</b> current_reservers_count = vector::length(&reservation_pool_object.reservers);
        <b>assert</b>!(current_reservers_count &lt; config.max_reservers_per_pool, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ETooManyReservers">ETooManyReservers</a>);
        table::add(&<b>mut</b> reservation_pool_object.reservations, reserver, net_amount);
        // Add to reservers list <b>for</b> tracking
        vector::push_back(&<b>mut</b> reservation_pool_object.reservers, reserver);
    };
    // Update total reserved (with net amount)
    <b>assert</b>!(reservation_pool_object.info.total_reserved &lt;= <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_MAX_U64">MAX_U64</a> - net_amount, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EOverflow">EOverflow</a>);
    reservation_pool_object.info.total_reserved = reservation_pool_object.info.total_reserved + net_amount;
    // Update registry
    <b>if</b> (table::contains(&registry.reservation_pools, post_id)) {
        <b>let</b> registry_pool = table::borrow_mut(&<b>mut</b> registry.reservation_pools, post_id);
        registry_pool.total_reserved = reservation_pool_object.info.total_reserved;
    } <b>else</b> {
        // Create registry <b>entry</b> <b>if</b> it doesn't exist
        <b>let</b> reservation_pool = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ReservationPool">ReservationPool</a> {
            associated_id: post_id,
            token_type: <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TOKEN_TYPE_POST">TOKEN_TYPE_POST</a>,
            owner: post_owner,
            total_reserved: reservation_pool_object.info.total_reserved,
            required_threshold: config.post_threshold,
            created_at: now,
        };
        table::add(&<b>mut</b> registry.reservation_pools, post_id, reservation_pool);
    };
    // Check <b>if</b> threshold was just met
    <b>let</b> threshold_met = reservation_pool_object.info.total_reserved &gt;= config.post_threshold;
    <b>let</b> was_threshold_met = (reservation_pool_object.info.total_reserved - net_amount) &gt;= config.post_threshold;
    // Emit threshold met event <b>if</b> this reservation pushed us over the threshold
    <b>if</b> (threshold_met && !was_threshold_met) {
        event::emit(<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ThresholdMetEvent">ThresholdMetEvent</a> {
            associated_id: post_id,
            token_type: <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TOKEN_TYPE_POST">TOKEN_TYPE_POST</a>,
            owner: post_owner,
            total_reserved: reservation_pool_object.info.total_reserved,
            required_threshold: config.post_threshold,
            timestamp: now,
        });
    };
    // Return excess payment
    <b>if</b> (coin::value(&remaining_payment) &gt; 0) {
        transfer::public_transfer(remaining_payment, reserver);
    } <b>else</b> {
        coin::destroy_zero(remaining_payment);
    };
    // Emit reservation created event
    // amount field represents the actual reserved amount (net_amount)
    event::emit(<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ReservationCreatedEvent">ReservationCreatedEvent</a> {
        associated_id: post_id,
        token_type: <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TOKEN_TYPE_POST">TOKEN_TYPE_POST</a>,
        reserver,
        amount: net_amount,
        total_reserved: reservation_pool_object.info.total_reserved,
        threshold_met,
        reserved_at: now,
        fee_amount,
        creator_fee,
        platform_fee,
        treasury_fee,
    });
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_reserve_towards_post_with_platform"></a>

## Function `reserve_towards_post_with_platform`

Reserve MYSO tokens towards a post to support social proof token creation
Platform version: platform fees go to platform treasury, includes platform validation


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_reserve_towards_post_with_platform">reserve_towards_post_with_platform</a>(registry: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">social_contracts::social_proof_tokens::TokenRegistry</a>, config: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">social_contracts::social_proof_tokens::SocialProofTokensConfig</a>, min_vault_deposit_amount: u64, reservation_pool_object: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ReservationPoolObject">social_contracts::social_proof_tokens::ReservationPoolObject</a>, treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, platform_registry: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">social_contracts::platform::PlatformRegistry</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, beneficiary_vault: &<b>mut</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_PoCBeneficiaryVault">social_contracts::poc_vault::PoCBeneficiaryVault</a>, payment: <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, amount: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_reserve_towards_post_with_platform">reserve_towards_post_with_platform</a>(
    registry: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">TokenRegistry</a>,
    config: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">SocialProofTokensConfig</a>,
    min_vault_deposit_amount: u64,
    reservation_pool_object: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ReservationPoolObject">ReservationPoolObject</a>,
    treasury: &EcosystemTreasury,
    platform_registry: &PlatformRegistry,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>,
    block_list_registry: &BlockListRegistry,
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &Post,
    beneficiary_vault: &<b>mut</b> PoCBeneficiaryVault,
    payment: Coin&lt;MYSO&gt;,
    amount: u64,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    // Check <b>if</b> trading is halted
    <b>assert</b>!(config.trading_enabled, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ETradingHalted">ETradingHalted</a>);
    // Prevent reservations after conversion to token
    <b>assert</b>!(!reservation_pool_object.converted, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EReservationPoolConverted">EReservationPoolConverted</a>);
    <b>let</b> reserver = tx_context::sender(ctx);
    // Get <a href="../social_contracts/post.md#social_contracts_post">post</a> ID and owner from reservation pool
    <b>let</b> post_id = reservation_pool_object.info.associated_id;
    <b>let</b> post_owner = reservation_pool_object.info.owner;
    <b>let</b> now = clock::timestamp_ms(clock);
    // Verify reservation pool is <b>for</b> a <a href="../social_contracts/post.md#social_contracts_post">post</a>
    <b>assert</b>!(reservation_pool_object.info.token_type == <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TOKEN_TYPE_POST">TOKEN_TYPE_POST</a>, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInvalidTokenType">EInvalidTokenType</a>);
    // Verify <a href="../social_contracts/post.md#social_contracts_post">post</a> matches reservation pool
    <b>assert</b>!(<a href="../social_contracts/post.md#social_contracts_post_get_id_address">post::get_id_address</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>) == post_id, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInvalidID">EInvalidID</a>);
    // Ensure reserver <b>has</b> enough funds
    <b>assert</b>!(coin::value(&payment) &gt;= amount && amount &gt; 0, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInsufficientFunds">EInsufficientFunds</a>);
    // Platform validation
    <b>let</b> platform_id = object::uid_to_address(<a href="../social_contracts/platform.md#social_contracts_platform_id">platform::id</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>));
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform_is_approved">platform::is_approved</a>(platform_registry, platform_id), <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ENotAuthorized">ENotAuthorized</a>);
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform_has_joined_platform">platform::has_joined_platform</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, reserver), <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EUserNotJoinedPlatform">EUserNotJoinedPlatform</a>);
    <b>assert</b>!(!<a href="../social_contracts/block_list.md#social_contracts_block_list_is_blocked">block_list::is_blocked</a>(block_list_registry, platform_id, reserver), <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EUserBlockedByPlatform">EUserBlockedByPlatform</a>);
    // Calculate fees upfront based on desired reservation amount
    <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_validate_reservation_fees">validate_reservation_fees</a>(config);
    <b>let</b> reservation_total_fee_bps = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_reservation_total_fee_bps">calculate_reservation_total_fee_bps</a>(config);
    <b>let</b> fee_amount = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_fee_amount_safe">calculate_fee_amount_safe</a>(amount, reservation_total_fee_bps);
    // Determine <b>if</b> fees should be on top or deducted from amount
    <b>let</b> fees_on_top = coin::value(&payment) &gt;= amount + fee_amount;
    <b>let</b> net_amount = <b>if</b> (fees_on_top) {
        // User <b>has</b> enough: reserve full amount, fees on top
        amount
    } <b>else</b> {
        // User doesn't have enough <b>for</b> fees on top: deduct fees from amount
        <b>assert</b>!(coin::value(&payment) &gt;= amount, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInsufficientFunds">EInsufficientFunds</a>);
        amount - fee_amount
    };
    // Calculate and distribute fees (<a href="../social_contracts/platform.md#social_contracts_platform">platform</a> version)
    // Fee distribution calculates fees from 'amount' and deducts from payment
    // When fees_on_top: payment <b>has</b> amount+fees, after distribution: remaining = amount (correct!)
    // When fees deducted: payment <b>has</b> amount, after distribution: remaining = amount - fees (correct!)
    <b>let</b> (<b>mut</b> remaining_payment, fee_amount, creator_fee, platform_fee, treasury_fee) = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_distribute_reservation_fees_with_post_and_platform">distribute_reservation_fees_with_post_and_platform</a>(
        config,
        min_vault_deposit_amount,
        reservation_pool_object,
        <a href="../social_contracts/post.md#social_contracts_post">post</a>,
        beneficiary_vault,
        amount,
        payment,
        treasury,
        <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>,
        clock,
        ctx
    );
    // Check individual reservation limit (based on net amount)
    <b>let</b> max_individual_reservation = (config.post_threshold * config.max_individual_reservation_bps) / <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_BPS_DENOM">BPS_DENOM</a>;
    <b>let</b> current_reservation = <b>if</b> (table::contains(&reservation_pool_object.reservations, reserver)) {
        *table::borrow(&reservation_pool_object.reservations, reserver)
    } <b>else</b> {
        0
    };
    <b>assert</b>!(current_reservation + net_amount &lt;= max_individual_reservation, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EExceededMaxHold">EExceededMaxHold</a>);
    // Extract net reservation payment
    <b>let</b> reservation_payment = coin::split(&<b>mut</b> remaining_payment, net_amount, ctx);
    balance::join(&<b>mut</b> reservation_pool_object.myso_balance, coin::into_balance(reservation_payment));
    // Update reserver's balance in the pool (store net amount)
    <b>if</b> (table::contains(&reservation_pool_object.reservations, reserver)) {
        <b>let</b> reservation_balance = table::borrow_mut(&<b>mut</b> reservation_pool_object.reservations, reserver);
        <b>assert</b>!(*reservation_balance &lt;= <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_MAX_U64">MAX_U64</a> - net_amount, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EOverflow">EOverflow</a>);
        *reservation_balance = *reservation_balance + net_amount;
    } <b>else</b> {
        // DoS protection: limit number of unique reservers per pool
        <b>let</b> current_reservers_count = vector::length(&reservation_pool_object.reservers);
        <b>assert</b>!(current_reservers_count &lt; config.max_reservers_per_pool, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ETooManyReservers">ETooManyReservers</a>);
        table::add(&<b>mut</b> reservation_pool_object.reservations, reserver, net_amount);
        // Add to reservers list <b>for</b> tracking
        vector::push_back(&<b>mut</b> reservation_pool_object.reservers, reserver);
    };
    // Update total reserved (with net amount)
    <b>assert</b>!(reservation_pool_object.info.total_reserved &lt;= <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_MAX_U64">MAX_U64</a> - net_amount, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EOverflow">EOverflow</a>);
    reservation_pool_object.info.total_reserved = reservation_pool_object.info.total_reserved + net_amount;
    // Update registry
    <b>if</b> (table::contains(&registry.reservation_pools, post_id)) {
        <b>let</b> registry_pool = table::borrow_mut(&<b>mut</b> registry.reservation_pools, post_id);
        registry_pool.total_reserved = reservation_pool_object.info.total_reserved;
    } <b>else</b> {
        // Create registry <b>entry</b> <b>if</b> it doesn't exist
        <b>let</b> reservation_pool = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ReservationPool">ReservationPool</a> {
            associated_id: post_id,
            token_type: <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TOKEN_TYPE_POST">TOKEN_TYPE_POST</a>,
            owner: post_owner,
            total_reserved: reservation_pool_object.info.total_reserved,
            required_threshold: config.post_threshold,
            created_at: now,
        };
        table::add(&<b>mut</b> registry.reservation_pools, post_id, reservation_pool);
    };
    // Check <b>if</b> threshold was just met
    <b>let</b> threshold_met = reservation_pool_object.info.total_reserved &gt;= config.post_threshold;
    <b>let</b> was_threshold_met = (reservation_pool_object.info.total_reserved - net_amount) &gt;= config.post_threshold;
    // Emit threshold met event <b>if</b> this reservation pushed us over the threshold
    <b>if</b> (threshold_met && !was_threshold_met) {
        event::emit(<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ThresholdMetEvent">ThresholdMetEvent</a> {
            associated_id: post_id,
            token_type: <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TOKEN_TYPE_POST">TOKEN_TYPE_POST</a>,
            owner: post_owner,
            total_reserved: reservation_pool_object.info.total_reserved,
            required_threshold: config.post_threshold,
            timestamp: now,
        });
    };
    // Return excess payment
    <b>if</b> (coin::value(&remaining_payment) &gt; 0) {
        transfer::public_transfer(remaining_payment, reserver);
    } <b>else</b> {
        coin::destroy_zero(remaining_payment);
    };
    // Emit reservation created event
    // amount field represents the actual reserved amount (net_amount)
    event::emit(<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ReservationCreatedEvent">ReservationCreatedEvent</a> {
        associated_id: post_id,
        token_type: <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TOKEN_TYPE_POST">TOKEN_TYPE_POST</a>,
        reserver,
        amount: net_amount,
        total_reserved: reservation_pool_object.info.total_reserved,
        threshold_met,
        reserved_at: now,
        fee_amount,
        creator_fee,
        platform_fee,
        treasury_fee,
    });
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_reserve_towards_profile"></a>

## Function `reserve_towards_profile`

Reserve MYSO tokens towards a profile to support social proof token creation
Non-platform version: platform fees go to ecosystem treasury
Anyone can call this function - the profile owner is stored in the reservation pool


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_reserve_towards_profile">reserve_towards_profile</a>(registry: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">social_contracts::social_proof_tokens::TokenRegistry</a>, config: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">social_contracts::social_proof_tokens::SocialProofTokensConfig</a>, reservation_pool_object: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ReservationPoolObject">social_contracts::social_proof_tokens::ReservationPoolObject</a>, treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, payment: <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, amount: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_reserve_towards_profile">reserve_towards_profile</a>(
    registry: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">TokenRegistry</a>,
    config: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">SocialProofTokensConfig</a>,
    reservation_pool_object: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ReservationPoolObject">ReservationPoolObject</a>,
    treasury: &EcosystemTreasury,
    payment: Coin&lt;MYSO&gt;,
    amount: u64,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    // Check <b>if</b> trading is halted
    <b>assert</b>!(config.trading_enabled, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ETradingHalted">ETradingHalted</a>);
    // Prevent reservations after conversion to token
    <b>assert</b>!(!reservation_pool_object.converted, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EReservationPoolConverted">EReservationPoolConverted</a>);
    <b>let</b> reserver = tx_context::sender(ctx);
    // Get <a href="../social_contracts/profile.md#social_contracts_profile">profile</a> ID and owner from reservation pool
    <b>let</b> profile_id = reservation_pool_object.info.associated_id;
    <b>let</b> profile_owner = reservation_pool_object.info.owner;
    <b>let</b> now = clock::timestamp_ms(clock);
    // Verify reservation pool is <b>for</b> a <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>
    <b>assert</b>!(reservation_pool_object.info.token_type == <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TOKEN_TYPE_PROFILE">TOKEN_TYPE_PROFILE</a>, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInvalidTokenType">EInvalidTokenType</a>);
    // Ensure reserver <b>has</b> enough funds
    <b>assert</b>!(coin::value(&payment) &gt;= amount && amount &gt; 0, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInsufficientFunds">EInsufficientFunds</a>);
    // Calculate fees upfront based on desired reservation amount
    <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_validate_reservation_fees">validate_reservation_fees</a>(config);
    <b>let</b> reservation_total_fee_bps = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_reservation_total_fee_bps">calculate_reservation_total_fee_bps</a>(config);
    <b>let</b> fee_amount = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_fee_amount_safe">calculate_fee_amount_safe</a>(amount, reservation_total_fee_bps);
    // Determine <b>if</b> fees should be on top or deducted from amount
    <b>let</b> fees_on_top = coin::value(&payment) &gt;= amount + fee_amount;
    <b>let</b> net_amount = <b>if</b> (fees_on_top) {
        // User <b>has</b> enough: reserve full amount, fees on top
        amount
    } <b>else</b> {
        // User doesn't have enough <b>for</b> fees on top: deduct fees from amount
        <b>assert</b>!(coin::value(&payment) &gt;= amount, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInsufficientFunds">EInsufficientFunds</a>);
        amount - fee_amount
    };
    // Calculate and distribute fees (non-<a href="../social_contracts/platform.md#social_contracts_platform">platform</a> version, no PoC <b>for</b> profiles)
    // Fee distribution calculates fees from 'amount' and deducts from payment
    // When fees_on_top: payment <b>has</b> amount+fees, after distribution: remaining = amount (correct!)
    // When fees deducted: payment <b>has</b> amount, after distribution: remaining = amount - fees (correct!)
    <b>let</b> (<b>mut</b> remaining_payment, fee_amount, creator_fee, platform_fee, treasury_fee) = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_distribute_reservation_fees_no_poc">distribute_reservation_fees_no_poc</a>(
        config,
        reservation_pool_object,
        amount,
        payment,
        treasury,
        ctx
    );
    // Check individual reservation limit (based on net amount)
    <b>let</b> max_individual_reservation = (config.profile_threshold * config.max_individual_reservation_bps) / <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_BPS_DENOM">BPS_DENOM</a>;
    <b>let</b> current_reservation = <b>if</b> (table::contains(&reservation_pool_object.reservations, reserver)) {
        *table::borrow(&reservation_pool_object.reservations, reserver)
    } <b>else</b> {
        0
    };
    <b>assert</b>!(current_reservation + net_amount &lt;= max_individual_reservation, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EExceededMaxHold">EExceededMaxHold</a>);
    // Extract net reservation payment
    <b>let</b> reservation_payment = coin::split(&<b>mut</b> remaining_payment, net_amount, ctx);
    balance::join(&<b>mut</b> reservation_pool_object.myso_balance, coin::into_balance(reservation_payment));
    // Update reserver's balance in the pool (store net amount)
    <b>if</b> (table::contains(&reservation_pool_object.reservations, reserver)) {
        <b>let</b> reservation_balance = table::borrow_mut(&<b>mut</b> reservation_pool_object.reservations, reserver);
        <b>assert</b>!(*reservation_balance &lt;= <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_MAX_U64">MAX_U64</a> - net_amount, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EOverflow">EOverflow</a>);
        *reservation_balance = *reservation_balance + net_amount;
    } <b>else</b> {
        // DoS protection: limit number of unique reservers per pool
        <b>let</b> current_reservers_count = vector::length(&reservation_pool_object.reservers);
        <b>assert</b>!(current_reservers_count &lt; config.max_reservers_per_pool, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ETooManyReservers">ETooManyReservers</a>);
        table::add(&<b>mut</b> reservation_pool_object.reservations, reserver, net_amount);
        // Add to reservers list <b>for</b> tracking
        vector::push_back(&<b>mut</b> reservation_pool_object.reservers, reserver);
    };
    // Update total reserved (with net amount)
    <b>assert</b>!(reservation_pool_object.info.total_reserved &lt;= <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_MAX_U64">MAX_U64</a> - net_amount, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EOverflow">EOverflow</a>);
    reservation_pool_object.info.total_reserved = reservation_pool_object.info.total_reserved + net_amount;
    // Update registry
    <b>if</b> (table::contains(&registry.reservation_pools, profile_id)) {
        <b>let</b> registry_pool = table::borrow_mut(&<b>mut</b> registry.reservation_pools, profile_id);
        registry_pool.total_reserved = reservation_pool_object.info.total_reserved;
    } <b>else</b> {
        // Create registry <b>entry</b> <b>if</b> it doesn't exist
        <b>let</b> reservation_pool = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ReservationPool">ReservationPool</a> {
            associated_id: profile_id,
            token_type: <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TOKEN_TYPE_PROFILE">TOKEN_TYPE_PROFILE</a>,
            owner: profile_owner,
            total_reserved: reservation_pool_object.info.total_reserved,
            required_threshold: config.profile_threshold,
            created_at: now,
        };
        table::add(&<b>mut</b> registry.reservation_pools, profile_id, reservation_pool);
    };
    // Check <b>if</b> threshold was just met
    <b>let</b> threshold_met = reservation_pool_object.info.total_reserved &gt;= config.profile_threshold;
    <b>let</b> was_threshold_met = (reservation_pool_object.info.total_reserved - net_amount) &gt;= config.profile_threshold;
    // Emit threshold met event <b>if</b> this reservation pushed us over the threshold
    <b>if</b> (threshold_met && !was_threshold_met) {
        event::emit(<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ThresholdMetEvent">ThresholdMetEvent</a> {
            associated_id: profile_id,
            token_type: <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TOKEN_TYPE_PROFILE">TOKEN_TYPE_PROFILE</a>,
            owner: profile_owner,
            total_reserved: reservation_pool_object.info.total_reserved,
            required_threshold: config.profile_threshold,
            timestamp: now,
        });
    };
    // Return excess payment
    <b>if</b> (coin::value(&remaining_payment) &gt; 0) {
        transfer::public_transfer(remaining_payment, reserver);
    } <b>else</b> {
        coin::destroy_zero(remaining_payment);
    };
    // Emit reservation created event
    // amount field represents the actual reserved amount (net_amount)
    event::emit(<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ReservationCreatedEvent">ReservationCreatedEvent</a> {
        associated_id: profile_id,
        token_type: <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TOKEN_TYPE_PROFILE">TOKEN_TYPE_PROFILE</a>,
        reserver,
        amount: net_amount,
        total_reserved: reservation_pool_object.info.total_reserved,
        threshold_met,
        reserved_at: now,
        fee_amount,
        creator_fee,
        platform_fee,
        treasury_fee,
    });
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_reserve_towards_profile_with_platform"></a>

## Function `reserve_towards_profile_with_platform`

Reserve MYSO tokens towards a profile to support social proof token creation
Platform version: platform fees go to platform treasury, includes platform validation
Anyone can call this function - the profile owner is stored in the reservation pool


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_reserve_towards_profile_with_platform">reserve_towards_profile_with_platform</a>(registry: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">social_contracts::social_proof_tokens::TokenRegistry</a>, config: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">social_contracts::social_proof_tokens::SocialProofTokensConfig</a>, reservation_pool_object: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ReservationPoolObject">social_contracts::social_proof_tokens::ReservationPoolObject</a>, treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, platform_registry: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">social_contracts::platform::PlatformRegistry</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, payment: <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, amount: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_reserve_towards_profile_with_platform">reserve_towards_profile_with_platform</a>(
    registry: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">TokenRegistry</a>,
    config: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">SocialProofTokensConfig</a>,
    reservation_pool_object: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ReservationPoolObject">ReservationPoolObject</a>,
    treasury: &EcosystemTreasury,
    platform_registry: &PlatformRegistry,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>,
    block_list_registry: &BlockListRegistry,
    payment: Coin&lt;MYSO&gt;,
    amount: u64,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    // Check <b>if</b> trading is halted
    <b>assert</b>!(config.trading_enabled, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ETradingHalted">ETradingHalted</a>);
    // Prevent reservations after conversion to token
    <b>assert</b>!(!reservation_pool_object.converted, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EReservationPoolConverted">EReservationPoolConverted</a>);
    <b>let</b> reserver = tx_context::sender(ctx);
    // Get <a href="../social_contracts/profile.md#social_contracts_profile">profile</a> ID and owner from reservation pool
    <b>let</b> profile_id = reservation_pool_object.info.associated_id;
    <b>let</b> profile_owner = reservation_pool_object.info.owner;
    <b>let</b> now = clock::timestamp_ms(clock);
    // Verify reservation pool is <b>for</b> a <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>
    <b>assert</b>!(reservation_pool_object.info.token_type == <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TOKEN_TYPE_PROFILE">TOKEN_TYPE_PROFILE</a>, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInvalidTokenType">EInvalidTokenType</a>);
    // Ensure reserver <b>has</b> enough funds
    <b>assert</b>!(coin::value(&payment) &gt;= amount && amount &gt; 0, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInsufficientFunds">EInsufficientFunds</a>);
    // Platform validation
    <b>let</b> platform_id = object::uid_to_address(<a href="../social_contracts/platform.md#social_contracts_platform_id">platform::id</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>));
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform_is_approved">platform::is_approved</a>(platform_registry, platform_id), <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ENotAuthorized">ENotAuthorized</a>);
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform_has_joined_platform">platform::has_joined_platform</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, reserver), <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EUserNotJoinedPlatform">EUserNotJoinedPlatform</a>);
    <b>assert</b>!(!<a href="../social_contracts/block_list.md#social_contracts_block_list_is_blocked">block_list::is_blocked</a>(block_list_registry, platform_id, reserver), <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EUserBlockedByPlatform">EUserBlockedByPlatform</a>);
    // Calculate fees upfront based on desired reservation amount
    <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_validate_reservation_fees">validate_reservation_fees</a>(config);
    <b>let</b> reservation_total_fee_bps = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_reservation_total_fee_bps">calculate_reservation_total_fee_bps</a>(config);
    <b>let</b> fee_amount = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_fee_amount_safe">calculate_fee_amount_safe</a>(amount, reservation_total_fee_bps);
    // Determine <b>if</b> fees should be on top or deducted from amount
    <b>let</b> fees_on_top = coin::value(&payment) &gt;= amount + fee_amount;
    <b>let</b> net_amount = <b>if</b> (fees_on_top) {
        // User <b>has</b> enough: reserve full amount, fees on top
        amount
    } <b>else</b> {
        // User doesn't have enough <b>for</b> fees on top: deduct fees from amount
        <b>assert</b>!(coin::value(&payment) &gt;= amount, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInsufficientFunds">EInsufficientFunds</a>);
        amount - fee_amount
    };
    // Calculate and distribute fees (<a href="../social_contracts/platform.md#social_contracts_platform">platform</a> version, no PoC <b>for</b> profiles)
    // Fee distribution calculates fees from 'amount' and deducts from payment
    // When fees_on_top: payment <b>has</b> amount+fees, after distribution: remaining = amount (correct!)
    // When fees deducted: payment <b>has</b> amount, after distribution: remaining = amount - fees (correct!)
    <b>let</b> (<b>mut</b> remaining_payment, fee_amount, creator_fee, platform_fee, treasury_fee) = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_distribute_reservation_fees_no_poc_with_platform">distribute_reservation_fees_no_poc_with_platform</a>(
        config,
        reservation_pool_object,
        amount,
        payment,
        treasury,
        <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>,
        clock,
        ctx
    );
    // Check individual reservation limit (based on net amount)
    <b>let</b> max_individual_reservation = (config.profile_threshold * config.max_individual_reservation_bps) / <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_BPS_DENOM">BPS_DENOM</a>;
    <b>let</b> current_reservation = <b>if</b> (table::contains(&reservation_pool_object.reservations, reserver)) {
        *table::borrow(&reservation_pool_object.reservations, reserver)
    } <b>else</b> {
        0
    };
    <b>assert</b>!(current_reservation + net_amount &lt;= max_individual_reservation, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EExceededMaxHold">EExceededMaxHold</a>);
    // Extract net reservation payment
    <b>let</b> reservation_payment = coin::split(&<b>mut</b> remaining_payment, net_amount, ctx);
    balance::join(&<b>mut</b> reservation_pool_object.myso_balance, coin::into_balance(reservation_payment));
    // Update reserver's balance in the pool (store net amount)
    <b>if</b> (table::contains(&reservation_pool_object.reservations, reserver)) {
        <b>let</b> reservation_balance = table::borrow_mut(&<b>mut</b> reservation_pool_object.reservations, reserver);
        <b>assert</b>!(*reservation_balance &lt;= <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_MAX_U64">MAX_U64</a> - net_amount, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EOverflow">EOverflow</a>);
        *reservation_balance = *reservation_balance + net_amount;
    } <b>else</b> {
        // DoS protection: limit number of unique reservers per pool
        <b>let</b> current_reservers_count = vector::length(&reservation_pool_object.reservers);
        <b>assert</b>!(current_reservers_count &lt; config.max_reservers_per_pool, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ETooManyReservers">ETooManyReservers</a>);
        table::add(&<b>mut</b> reservation_pool_object.reservations, reserver, net_amount);
        // Add to reservers list <b>for</b> tracking
        vector::push_back(&<b>mut</b> reservation_pool_object.reservers, reserver);
    };
    // Update total reserved (with net amount)
    <b>assert</b>!(reservation_pool_object.info.total_reserved &lt;= <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_MAX_U64">MAX_U64</a> - net_amount, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EOverflow">EOverflow</a>);
    reservation_pool_object.info.total_reserved = reservation_pool_object.info.total_reserved + net_amount;
    // Update registry
    <b>if</b> (table::contains(&registry.reservation_pools, profile_id)) {
        <b>let</b> registry_pool = table::borrow_mut(&<b>mut</b> registry.reservation_pools, profile_id);
        registry_pool.total_reserved = reservation_pool_object.info.total_reserved;
    } <b>else</b> {
        // Create registry <b>entry</b> <b>if</b> it doesn't exist
        <b>let</b> reservation_pool = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ReservationPool">ReservationPool</a> {
            associated_id: profile_id,
            token_type: <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TOKEN_TYPE_PROFILE">TOKEN_TYPE_PROFILE</a>,
            owner: profile_owner,
            total_reserved: reservation_pool_object.info.total_reserved,
            required_threshold: config.profile_threshold,
            created_at: now,
        };
        table::add(&<b>mut</b> registry.reservation_pools, profile_id, reservation_pool);
    };
    // Check <b>if</b> threshold was just met
    <b>let</b> threshold_met = reservation_pool_object.info.total_reserved &gt;= config.profile_threshold;
    <b>let</b> was_threshold_met = (reservation_pool_object.info.total_reserved - net_amount) &gt;= config.profile_threshold;
    // Emit threshold met event <b>if</b> this reservation pushed us over the threshold
    <b>if</b> (threshold_met && !was_threshold_met) {
        event::emit(<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ThresholdMetEvent">ThresholdMetEvent</a> {
            associated_id: profile_id,
            token_type: <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TOKEN_TYPE_PROFILE">TOKEN_TYPE_PROFILE</a>,
            owner: profile_owner,
            total_reserved: reservation_pool_object.info.total_reserved,
            required_threshold: config.profile_threshold,
            timestamp: now,
        });
    };
    // Return excess payment
    <b>if</b> (coin::value(&remaining_payment) &gt; 0) {
        transfer::public_transfer(remaining_payment, reserver);
    } <b>else</b> {
        coin::destroy_zero(remaining_payment);
    };
    // Emit reservation created event
    // amount field represents the actual reserved amount (net_amount)
    event::emit(<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ReservationCreatedEvent">ReservationCreatedEvent</a> {
        associated_id: profile_id,
        token_type: <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TOKEN_TYPE_PROFILE">TOKEN_TYPE_PROFILE</a>,
        reserver,
        amount: net_amount,
        total_reserved: reservation_pool_object.info.total_reserved,
        threshold_met,
        reserved_at: now,
        fee_amount,
        creator_fee,
        platform_fee,
        treasury_fee,
    });
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_apply_reservation_withdrawal_ledger"></a>

## Function `apply_reservation_withdrawal_ledger`

Deduct gross withdrawal from reservation ledger and registry mirror.


<pre><code><b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_apply_reservation_withdrawal_ledger">apply_reservation_withdrawal_ledger</a>(registry: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">social_contracts::social_proof_tokens::TokenRegistry</a>, reservation_pool_object: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ReservationPoolObject">social_contracts::social_proof_tokens::ReservationPoolObject</a>, reserver: <b>address</b>, associated_id: <b>address</b>, amount: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_apply_reservation_withdrawal_ledger">apply_reservation_withdrawal_ledger</a>(
    registry: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">TokenRegistry</a>,
    reservation_pool_object: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ReservationPoolObject">ReservationPoolObject</a>,
    reserver: <b>address</b>,
    associated_id: <b>address</b>,
    amount: u64,
) {
    <b>let</b> current_reservation = *table::borrow(&reservation_pool_object.reservations, reserver);
    <b>assert</b>!(current_reservation &gt;= amount, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInsufficientLiquidity">EInsufficientLiquidity</a>);
    <b>if</b> (current_reservation == amount) {
        table::remove(&<b>mut</b> reservation_pool_object.reservations, reserver);
        <b>let</b> <b>mut</b> i = 0;
        <b>let</b> len = vector::length(&reservation_pool_object.reservers);
        <b>while</b> (i &lt; len) {
            <b>if</b> (*vector::borrow(&reservation_pool_object.reservers, i) == reserver) {
                vector::remove(&<b>mut</b> reservation_pool_object.reservers, i);
                <b>break</b>
            };
            i = i + 1;
        };
    } <b>else</b> {
        <b>let</b> reservation_balance = table::borrow_mut(&<b>mut</b> reservation_pool_object.reservations, reserver);
        *reservation_balance = *reservation_balance - amount;
    };
    reservation_pool_object.info.total_reserved = reservation_pool_object.info.total_reserved - amount;
    <b>if</b> (table::contains(&registry.reservation_pools, associated_id)) {
        <b>let</b> registry_pool = table::borrow_mut(&<b>mut</b> registry.reservation_pools, associated_id);
        registry_pool.total_reserved = reservation_pool_object.info.total_reserved;
    };
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_reservation_withdrawal_fee_split"></a>

## Function `reservation_withdrawal_fee_split`



<pre><code><b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_reservation_withdrawal_fee_split">reservation_withdrawal_fee_split</a>(config: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">social_contracts::social_proof_tokens::SocialProofTokensConfig</a>, amount: u64): (u64, u64, u64, u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_reservation_withdrawal_fee_split">reservation_withdrawal_fee_split</a>(
    config: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">SocialProofTokensConfig</a>,
    amount: u64
): (u64, u64, u64, u64, u64) {
    <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_validate_reservation_fees">validate_reservation_fees</a>(config);
    <b>let</b> reservation_total_fee_bps = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_reservation_total_fee_bps">calculate_reservation_total_fee_bps</a>(config);
    <b>let</b> fee_amount = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_fee_amount_safe">calculate_fee_amount_safe</a>(amount, reservation_total_fee_bps);
    <b>let</b> creator_fee = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_component_fee_safe">calculate_component_fee_safe</a>(
        fee_amount,
        config.reservation_creator_fee_bps,
        reservation_total_fee_bps
    );
    <b>let</b> platform_fee = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_component_fee_safe">calculate_component_fee_safe</a>(
        fee_amount,
        config.reservation_platform_fee_bps,
        reservation_total_fee_bps
    );
    <b>let</b> treasury_fee = fee_amount - creator_fee - platform_fee;
    <b>let</b> net_refund = amount - fee_amount;
    (fee_amount, creator_fee, platform_fee, treasury_fee, net_refund)
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_distribute_reservation_withdraw_fees_non_platform_post"></a>

## Function `distribute_reservation_withdraw_fees_non_platform_post`

Non-platform post withdrawal: split configured platform fee between PoC-aware creator
routing and ecosystem per <code>config.non_platform_platform_to_*_bps</code> (defaults 50/50).


<pre><code><b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_distribute_reservation_withdraw_fees_non_platform_post">distribute_reservation_withdraw_fees_non_platform_post</a>(config: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">social_contracts::social_proof_tokens::SocialProofTokensConfig</a>, min_vault_deposit_amount: u64, pool_owner: <b>address</b>, <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, beneficiary_vault: &<b>mut</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_PoCBeneficiaryVault">social_contracts::poc_vault::PoCBeneficiaryVault</a>, treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, creator_fee: u64, platform_fee: u64, treasury_fee: u64, pool_balance: &<b>mut</b> <a href="../myso/balance.md#myso_balance_Balance">myso::balance::Balance</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): (u64, u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_distribute_reservation_withdraw_fees_non_platform_post">distribute_reservation_withdraw_fees_non_platform_post</a>(
    config: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">SocialProofTokensConfig</a>,
    min_vault_deposit_amount: u64,
    pool_owner: <b>address</b>,
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &Post,
    beneficiary_vault: &<b>mut</b> PoCBeneficiaryVault,
    treasury: &EcosystemTreasury,
    creator_fee: u64,
    platform_fee: u64,
    treasury_fee: u64,
    pool_balance: &<b>mut</b> Balance&lt;MYSO&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
): (u64, u64, u64) {
    <b>let</b> platform_fee_to_creator = (platform_fee * config.non_platform_platform_to_creator_bps) / <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_BPS_DENOM">BPS_DENOM</a>;
    <b>let</b> platform_fee_to_treasury = platform_fee - platform_fee_to_creator;
    <b>let</b> creator_total = creator_fee + platform_fee_to_creator;
    <b>let</b> treasury_total = treasury_fee + platform_fee_to_treasury;
    <b>if</b> (creator_total &gt; 0) {
        <b>let</b> <b>mut</b> creator_coin = coin::from_balance(balance::split(pool_balance, creator_total), ctx);
        <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_distribute_reservation_creator_fee_with_owner">distribute_reservation_creator_fee_with_owner</a>(
            pool_owner,
            <a href="../social_contracts/post.md#social_contracts_post">post</a>,
            beneficiary_vault,
            creator_total,
            &<b>mut</b> creator_coin,
            min_vault_deposit_amount,
            clock,
            ctx
        );
        coin::destroy_zero(creator_coin);
    };
    <b>if</b> (treasury_total &gt; 0) {
        <b>let</b> treasury_coin = coin::from_balance(balance::split(pool_balance, treasury_total), ctx);
        transfer::public_transfer(treasury_coin, <a href="../social_contracts/profile.md#social_contracts_profile_get_treasury_address">profile::get_treasury_address</a>(treasury));
    };
    (creator_total, 0, treasury_total)
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_distribute_reservation_withdraw_fees_non_platform_profile"></a>

## Function `distribute_reservation_withdraw_fees_non_platform_profile`

Non-platform profile withdrawal: same 50/50 platform-fee convention as
<code><a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_distribute_reservation_fees_no_poc">distribute_reservation_fees_no_poc</a></code>.


<pre><code><b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_distribute_reservation_withdraw_fees_non_platform_profile">distribute_reservation_withdraw_fees_non_platform_profile</a>(config: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">social_contracts::social_proof_tokens::SocialProofTokensConfig</a>, pool_owner: <b>address</b>, treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, creator_fee: u64, platform_fee: u64, treasury_fee: u64, pool_balance: &<b>mut</b> <a href="../myso/balance.md#myso_balance_Balance">myso::balance::Balance</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): (u64, u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_distribute_reservation_withdraw_fees_non_platform_profile">distribute_reservation_withdraw_fees_non_platform_profile</a>(
    config: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">SocialProofTokensConfig</a>,
    pool_owner: <b>address</b>,
    treasury: &EcosystemTreasury,
    creator_fee: u64,
    platform_fee: u64,
    treasury_fee: u64,
    pool_balance: &<b>mut</b> Balance&lt;MYSO&gt;,
    ctx: &<b>mut</b> TxContext
): (u64, u64, u64) {
    <b>let</b> platform_fee_to_creator = (platform_fee * config.non_platform_platform_to_creator_bps) / <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_BPS_DENOM">BPS_DENOM</a>;
    <b>let</b> platform_fee_to_treasury = platform_fee - platform_fee_to_creator;
    <b>let</b> creator_total = creator_fee + platform_fee_to_creator;
    <b>let</b> treasury_total = treasury_fee + platform_fee_to_treasury;
    <b>if</b> (creator_total &gt; 0) {
        <b>let</b> <b>mut</b> creator_coin = coin::from_balance(balance::split(pool_balance, creator_total), ctx);
        <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_distribute_reservation_creator_fee_no_poc_with_owner">distribute_reservation_creator_fee_no_poc_with_owner</a>(
            pool_owner,
            creator_total,
            &<b>mut</b> creator_coin,
            ctx
        );
        coin::destroy_zero(creator_coin);
    };
    <b>if</b> (treasury_total &gt; 0) {
        <b>let</b> treasury_coin = coin::from_balance(balance::split(pool_balance, treasury_total), ctx);
        transfer::public_transfer(treasury_coin, <a href="../social_contracts/profile.md#social_contracts_profile_get_treasury_address">profile::get_treasury_address</a>(treasury));
    };
    (creator_total, 0, treasury_total)
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_distribute_reservation_withdraw_fees_platform_post"></a>

## Function `distribute_reservation_withdraw_fees_platform_post`



<pre><code><b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_distribute_reservation_withdraw_fees_platform_post">distribute_reservation_withdraw_fees_platform_post</a>(pool_owner: <b>address</b>, <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, beneficiary_vault: &<b>mut</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_PoCBeneficiaryVault">social_contracts::poc_vault::PoCBeneficiaryVault</a>, treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, creator_fee: u64, platform_fee: u64, treasury_fee: u64, pool_balance: &<b>mut</b> <a href="../myso/balance.md#myso_balance_Balance">myso::balance::Balance</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, min_vault_deposit_amount: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_distribute_reservation_withdraw_fees_platform_post">distribute_reservation_withdraw_fees_platform_post</a>(
    pool_owner: <b>address</b>,
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &Post,
    beneficiary_vault: &<b>mut</b> PoCBeneficiaryVault,
    treasury: &EcosystemTreasury,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>,
    creator_fee: u64,
    platform_fee: u64,
    treasury_fee: u64,
    pool_balance: &<b>mut</b> Balance&lt;MYSO&gt;,
    min_vault_deposit_amount: u64,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <b>if</b> (creator_fee &gt; 0) {
        <b>let</b> <b>mut</b> creator_coin = coin::from_balance(balance::split(pool_balance, creator_fee), ctx);
        <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_distribute_reservation_creator_fee_with_owner">distribute_reservation_creator_fee_with_owner</a>(
            pool_owner,
            <a href="../social_contracts/post.md#social_contracts_post">post</a>,
            beneficiary_vault,
            creator_fee,
            &<b>mut</b> creator_coin,
            min_vault_deposit_amount,
            clock,
            ctx
        );
        coin::destroy_zero(creator_coin);
    };
    <b>if</b> (platform_fee &gt; 0) {
        <b>let</b> <b>mut</b> platform_fee_coin = coin::from_balance(balance::split(pool_balance, platform_fee), ctx);
        <a href="../social_contracts/platform.md#social_contracts_platform_add_to_treasury">social_contracts::platform::add_to_treasury</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, &<b>mut</b> platform_fee_coin, platform_fee, clock, ctx);
        coin::destroy_zero(platform_fee_coin);
    };
    <b>if</b> (treasury_fee &gt; 0) {
        <b>let</b> treasury_coin = coin::from_balance(balance::split(pool_balance, treasury_fee), ctx);
        transfer::public_transfer(treasury_coin, <a href="../social_contracts/profile.md#social_contracts_profile_get_treasury_address">profile::get_treasury_address</a>(treasury));
    };
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_distribute_reservation_withdraw_fees_platform_profile"></a>

## Function `distribute_reservation_withdraw_fees_platform_profile`



<pre><code><b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_distribute_reservation_withdraw_fees_platform_profile">distribute_reservation_withdraw_fees_platform_profile</a>(pool_owner: <b>address</b>, treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, creator_fee: u64, platform_fee: u64, treasury_fee: u64, pool_balance: &<b>mut</b> <a href="../myso/balance.md#myso_balance_Balance">myso::balance::Balance</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_distribute_reservation_withdraw_fees_platform_profile">distribute_reservation_withdraw_fees_platform_profile</a>(
    pool_owner: <b>address</b>,
    treasury: &EcosystemTreasury,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>,
    creator_fee: u64,
    platform_fee: u64,
    treasury_fee: u64,
    pool_balance: &<b>mut</b> Balance&lt;MYSO&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <b>if</b> (creator_fee &gt; 0) {
        <b>let</b> <b>mut</b> creator_coin = coin::from_balance(balance::split(pool_balance, creator_fee), ctx);
        <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_distribute_reservation_creator_fee_no_poc_with_owner">distribute_reservation_creator_fee_no_poc_with_owner</a>(
            pool_owner,
            creator_fee,
            &<b>mut</b> creator_coin,
            ctx
        );
        coin::destroy_zero(creator_coin);
    };
    <b>if</b> (platform_fee &gt; 0) {
        <b>let</b> <b>mut</b> platform_fee_coin = coin::from_balance(balance::split(pool_balance, platform_fee), ctx);
        <a href="../social_contracts/platform.md#social_contracts_platform_add_to_treasury">social_contracts::platform::add_to_treasury</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, &<b>mut</b> platform_fee_coin, platform_fee, clock, ctx);
        coin::destroy_zero(platform_fee_coin);
    };
    <b>if</b> (treasury_fee &gt; 0) {
        <b>let</b> treasury_coin = coin::from_balance(balance::split(pool_balance, treasury_fee), ctx);
        transfer::public_transfer(treasury_coin, <a href="../social_contracts/profile.md#social_contracts_profile_get_treasury_address">profile::get_treasury_address</a>(treasury));
    };
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_withdraw_reservation_for_post"></a>

## Function `withdraw_reservation_for_post`

Withdraw MYSO reservation for a **post** pool (non-platform).
<code>amount</code> is gross ledger reduction; caller receives <code>amount - reservation_fees</code>.
Matches non-platform reserve fee routing (50/50 platform share; PoC on creator path).


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_withdraw_reservation_for_post">withdraw_reservation_for_post</a>(registry: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">social_contracts::social_proof_tokens::TokenRegistry</a>, config: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">social_contracts::social_proof_tokens::SocialProofTokensConfig</a>, min_vault_deposit_amount: u64, reservation_pool_object: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ReservationPoolObject">social_contracts::social_proof_tokens::ReservationPoolObject</a>, treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, beneficiary_vault: &<b>mut</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_PoCBeneficiaryVault">social_contracts::poc_vault::PoCBeneficiaryVault</a>, amount: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_withdraw_reservation_for_post">withdraw_reservation_for_post</a>(
    registry: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">TokenRegistry</a>,
    config: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">SocialProofTokensConfig</a>,
    min_vault_deposit_amount: u64,
    reservation_pool_object: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ReservationPoolObject">ReservationPoolObject</a>,
    treasury: &EcosystemTreasury,
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &Post,
    beneficiary_vault: &<b>mut</b> PoCBeneficiaryVault,
    amount: u64,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> reserver = tx_context::sender(ctx);
    <b>let</b> associated_id = reservation_pool_object.info.associated_id;
    <b>let</b> now = clock::timestamp_ms(clock);
    <b>assert</b>!(!reservation_pool_object.converted, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EReservationPoolConverted">EReservationPoolConverted</a>);
    <b>assert</b>!(amount &gt; 0, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInsufficientFunds">EInsufficientFunds</a>);
    <b>assert</b>!(reservation_pool_object.info.token_type == <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TOKEN_TYPE_POST">TOKEN_TYPE_POST</a>, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInvalidTokenType">EInvalidTokenType</a>);
    <b>assert</b>!(<a href="../social_contracts/post.md#social_contracts_post_get_id_address">post::get_id_address</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>) == associated_id, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInvalidID">EInvalidID</a>);
    <b>assert</b>!(table::contains(&reservation_pool_object.reservations, reserver), <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ENoTokensOwned">ENoTokensOwned</a>);
    <b>let</b> current_reservation = *table::borrow(&reservation_pool_object.reservations, reserver);
    <b>let</b> pool_owner = reservation_pool_object.info.owner;
    <b>let</b> (fee_amount, creator_fee, platform_fee, treasury_fee, net_refund) =
        <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_reservation_withdrawal_fee_split">reservation_withdrawal_fee_split</a>(config, amount);
    <b>assert</b>!(current_reservation &gt;= amount, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInsufficientLiquidity">EInsufficientLiquidity</a>);
    <b>assert</b>!(balance::value(&reservation_pool_object.myso_balance) &gt;= amount, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInsufficientLiquidity">EInsufficientLiquidity</a>);
    <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_apply_reservation_withdrawal_ledger">apply_reservation_withdrawal_ledger</a>(registry, reservation_pool_object, reserver, associated_id, amount);
    <b>let</b> (ev_creator, ev_platform, ev_treasury) = <b>if</b> (fee_amount &gt; 0) {
        <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_distribute_reservation_withdraw_fees_non_platform_post">distribute_reservation_withdraw_fees_non_platform_post</a>(
            config,
            min_vault_deposit_amount,
            pool_owner,
            <a href="../social_contracts/post.md#social_contracts_post">post</a>,
            beneficiary_vault,
            treasury,
            creator_fee,
            platform_fee,
            treasury_fee,
            &<b>mut</b> reservation_pool_object.myso_balance,
            clock,
            ctx
        )
    } <b>else</b> {
        (0, 0, 0)
    };
    <b>let</b> refund_balance = balance::split(&<b>mut</b> reservation_pool_object.myso_balance, net_refund);
    <b>let</b> refund_coin = coin::from_balance(refund_balance, ctx);
    transfer::public_transfer(refund_coin, reserver);
    event::emit(<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ReservationWithdrawnEvent">ReservationWithdrawnEvent</a> {
        associated_id,
        token_type: reservation_pool_object.info.token_type,
        reserver,
        amount,
        total_reserved: reservation_pool_object.info.total_reserved,
        withdrawn_at: now,
        fee_amount,
        creator_fee: ev_creator,
        platform_fee: ev_platform,
        treasury_fee: ev_treasury,
    });
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_withdraw_reservation_for_profile"></a>

## Function `withdraw_reservation_for_profile`

Withdraw MYSO reservation for a **profile** pool (non-platform).


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_withdraw_reservation_for_profile">withdraw_reservation_for_profile</a>(registry: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">social_contracts::social_proof_tokens::TokenRegistry</a>, config: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">social_contracts::social_proof_tokens::SocialProofTokensConfig</a>, reservation_pool_object: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ReservationPoolObject">social_contracts::social_proof_tokens::ReservationPoolObject</a>, treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, amount: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_withdraw_reservation_for_profile">withdraw_reservation_for_profile</a>(
    registry: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">TokenRegistry</a>,
    config: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">SocialProofTokensConfig</a>,
    reservation_pool_object: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ReservationPoolObject">ReservationPoolObject</a>,
    treasury: &EcosystemTreasury,
    amount: u64,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> reserver = tx_context::sender(ctx);
    <b>let</b> associated_id = reservation_pool_object.info.associated_id;
    <b>let</b> now = clock::timestamp_ms(clock);
    <b>assert</b>!(!reservation_pool_object.converted, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EReservationPoolConverted">EReservationPoolConverted</a>);
    <b>assert</b>!(amount &gt; 0, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInsufficientFunds">EInsufficientFunds</a>);
    <b>assert</b>!(reservation_pool_object.info.token_type == <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TOKEN_TYPE_PROFILE">TOKEN_TYPE_PROFILE</a>, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInvalidTokenType">EInvalidTokenType</a>);
    <b>assert</b>!(table::contains(&reservation_pool_object.reservations, reserver), <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ENoTokensOwned">ENoTokensOwned</a>);
    <b>let</b> current_reservation = *table::borrow(&reservation_pool_object.reservations, reserver);
    <b>let</b> pool_owner = reservation_pool_object.info.owner;
    <b>let</b> (fee_amount, creator_fee, platform_fee, treasury_fee, net_refund) =
        <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_reservation_withdrawal_fee_split">reservation_withdrawal_fee_split</a>(config, amount);
    <b>assert</b>!(current_reservation &gt;= amount, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInsufficientLiquidity">EInsufficientLiquidity</a>);
    <b>assert</b>!(balance::value(&reservation_pool_object.myso_balance) &gt;= amount, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInsufficientLiquidity">EInsufficientLiquidity</a>);
    <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_apply_reservation_withdrawal_ledger">apply_reservation_withdrawal_ledger</a>(registry, reservation_pool_object, reserver, associated_id, amount);
    <b>let</b> (ev_creator, ev_platform, ev_treasury) = <b>if</b> (fee_amount &gt; 0) {
        <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_distribute_reservation_withdraw_fees_non_platform_profile">distribute_reservation_withdraw_fees_non_platform_profile</a>(
            config,
            pool_owner,
            treasury,
            creator_fee,
            platform_fee,
            treasury_fee,
            &<b>mut</b> reservation_pool_object.myso_balance,
            ctx
        )
    } <b>else</b> {
        (0, 0, 0)
    };
    <b>let</b> refund_balance = balance::split(&<b>mut</b> reservation_pool_object.myso_balance, net_refund);
    <b>let</b> refund_coin = coin::from_balance(refund_balance, ctx);
    transfer::public_transfer(refund_coin, reserver);
    event::emit(<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ReservationWithdrawnEvent">ReservationWithdrawnEvent</a> {
        associated_id,
        token_type: reservation_pool_object.info.token_type,
        reserver,
        amount,
        total_reserved: reservation_pool_object.info.total_reserved,
        withdrawn_at: now,
        fee_amount,
        creator_fee: ev_creator,
        platform_fee: ev_platform,
        treasury_fee: ev_treasury,
    });
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_withdraw_reservation_with_platform_for_post"></a>

## Function `withdraw_reservation_with_platform_for_post`

Withdraw from a **post** reservation pool via an approved platform (PoC-aware creator fees).


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_withdraw_reservation_with_platform_for_post">withdraw_reservation_with_platform_for_post</a>(registry: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">social_contracts::social_proof_tokens::TokenRegistry</a>, config: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">social_contracts::social_proof_tokens::SocialProofTokensConfig</a>, min_vault_deposit_amount: u64, reservation_pool_object: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ReservationPoolObject">social_contracts::social_proof_tokens::ReservationPoolObject</a>, treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, platform_registry: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">social_contracts::platform::PlatformRegistry</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, beneficiary_vault: &<b>mut</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_PoCBeneficiaryVault">social_contracts::poc_vault::PoCBeneficiaryVault</a>, amount: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_withdraw_reservation_with_platform_for_post">withdraw_reservation_with_platform_for_post</a>(
    registry: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">TokenRegistry</a>,
    config: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">SocialProofTokensConfig</a>,
    min_vault_deposit_amount: u64,
    reservation_pool_object: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ReservationPoolObject">ReservationPoolObject</a>,
    treasury: &EcosystemTreasury,
    platform_registry: &PlatformRegistry,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>,
    block_list_registry: &BlockListRegistry,
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &Post,
    beneficiary_vault: &<b>mut</b> PoCBeneficiaryVault,
    amount: u64,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> reserver = tx_context::sender(ctx);
    <b>let</b> associated_id = reservation_pool_object.info.associated_id;
    <b>let</b> now = clock::timestamp_ms(clock);
    <b>assert</b>!(!reservation_pool_object.converted, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EReservationPoolConverted">EReservationPoolConverted</a>);
    <b>assert</b>!(amount &gt; 0, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInsufficientFunds">EInsufficientFunds</a>);
    <b>assert</b>!(reservation_pool_object.info.token_type == <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TOKEN_TYPE_POST">TOKEN_TYPE_POST</a>, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInvalidTokenType">EInvalidTokenType</a>);
    <b>assert</b>!(<a href="../social_contracts/post.md#social_contracts_post_get_id_address">post::get_id_address</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>) == associated_id, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInvalidID">EInvalidID</a>);
    <b>let</b> platform_id = object::uid_to_address(<a href="../social_contracts/platform.md#social_contracts_platform_id">platform::id</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>));
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform_is_approved">platform::is_approved</a>(platform_registry, platform_id), <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ENotAuthorized">ENotAuthorized</a>);
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform_has_joined_platform">platform::has_joined_platform</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, reserver), <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EUserNotJoinedPlatform">EUserNotJoinedPlatform</a>);
    <b>assert</b>!(!<a href="../social_contracts/block_list.md#social_contracts_block_list_is_blocked">block_list::is_blocked</a>(block_list_registry, platform_id, reserver), <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EUserBlockedByPlatform">EUserBlockedByPlatform</a>);
    <b>assert</b>!(table::contains(&reservation_pool_object.reservations, reserver), <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ENoTokensOwned">ENoTokensOwned</a>);
    <b>let</b> current_reservation = *table::borrow(&reservation_pool_object.reservations, reserver);
    <b>let</b> pool_owner = reservation_pool_object.info.owner;
    <b>let</b> (fee_amount, creator_fee, platform_fee, treasury_fee, net_refund) =
        <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_reservation_withdrawal_fee_split">reservation_withdrawal_fee_split</a>(config, amount);
    <b>assert</b>!(current_reservation &gt;= amount, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInsufficientLiquidity">EInsufficientLiquidity</a>);
    <b>assert</b>!(balance::value(&reservation_pool_object.myso_balance) &gt;= amount, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInsufficientLiquidity">EInsufficientLiquidity</a>);
    <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_apply_reservation_withdrawal_ledger">apply_reservation_withdrawal_ledger</a>(registry, reservation_pool_object, reserver, associated_id, amount);
    <b>if</b> (fee_amount &gt; 0) {
        <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_distribute_reservation_withdraw_fees_platform_post">distribute_reservation_withdraw_fees_platform_post</a>(
            pool_owner,
            <a href="../social_contracts/post.md#social_contracts_post">post</a>,
            beneficiary_vault,
            treasury,
            <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>,
            creator_fee,
            platform_fee,
            treasury_fee,
            &<b>mut</b> reservation_pool_object.myso_balance,
            min_vault_deposit_amount,
            clock,
            ctx
        );
    };
    <b>let</b> refund_balance = balance::split(&<b>mut</b> reservation_pool_object.myso_balance, net_refund);
    <b>let</b> refund_coin = coin::from_balance(refund_balance, ctx);
    transfer::public_transfer(refund_coin, reserver);
    event::emit(<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ReservationWithdrawnEvent">ReservationWithdrawnEvent</a> {
        associated_id,
        token_type: reservation_pool_object.info.token_type,
        reserver,
        amount,
        total_reserved: reservation_pool_object.info.total_reserved,
        withdrawn_at: now,
        fee_amount,
        creator_fee,
        platform_fee,
        treasury_fee,
    });
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_withdraw_reservation_with_platform_for_profile"></a>

## Function `withdraw_reservation_with_platform_for_profile`

Withdraw from a **profile** reservation pool via an approved platform.


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_withdraw_reservation_with_platform_for_profile">withdraw_reservation_with_platform_for_profile</a>(registry: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">social_contracts::social_proof_tokens::TokenRegistry</a>, config: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">social_contracts::social_proof_tokens::SocialProofTokensConfig</a>, reservation_pool_object: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ReservationPoolObject">social_contracts::social_proof_tokens::ReservationPoolObject</a>, treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, platform_registry: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">social_contracts::platform::PlatformRegistry</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, amount: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_withdraw_reservation_with_platform_for_profile">withdraw_reservation_with_platform_for_profile</a>(
    registry: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">TokenRegistry</a>,
    config: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">SocialProofTokensConfig</a>,
    reservation_pool_object: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ReservationPoolObject">ReservationPoolObject</a>,
    treasury: &EcosystemTreasury,
    platform_registry: &PlatformRegistry,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>,
    block_list_registry: &BlockListRegistry,
    amount: u64,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> reserver = tx_context::sender(ctx);
    <b>let</b> associated_id = reservation_pool_object.info.associated_id;
    <b>let</b> now = clock::timestamp_ms(clock);
    <b>assert</b>!(!reservation_pool_object.converted, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EReservationPoolConverted">EReservationPoolConverted</a>);
    <b>assert</b>!(amount &gt; 0, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInsufficientFunds">EInsufficientFunds</a>);
    <b>assert</b>!(reservation_pool_object.info.token_type == <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TOKEN_TYPE_PROFILE">TOKEN_TYPE_PROFILE</a>, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInvalidTokenType">EInvalidTokenType</a>);
    <b>let</b> platform_id = object::uid_to_address(<a href="../social_contracts/platform.md#social_contracts_platform_id">platform::id</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>));
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform_is_approved">platform::is_approved</a>(platform_registry, platform_id), <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ENotAuthorized">ENotAuthorized</a>);
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform_has_joined_platform">platform::has_joined_platform</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, reserver), <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EUserNotJoinedPlatform">EUserNotJoinedPlatform</a>);
    <b>assert</b>!(!<a href="../social_contracts/block_list.md#social_contracts_block_list_is_blocked">block_list::is_blocked</a>(block_list_registry, platform_id, reserver), <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EUserBlockedByPlatform">EUserBlockedByPlatform</a>);
    <b>assert</b>!(table::contains(&reservation_pool_object.reservations, reserver), <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ENoTokensOwned">ENoTokensOwned</a>);
    <b>let</b> current_reservation = *table::borrow(&reservation_pool_object.reservations, reserver);
    <b>let</b> pool_owner = reservation_pool_object.info.owner;
    <b>let</b> (fee_amount, creator_fee, platform_fee, treasury_fee, net_refund) =
        <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_reservation_withdrawal_fee_split">reservation_withdrawal_fee_split</a>(config, amount);
    <b>assert</b>!(current_reservation &gt;= amount, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInsufficientLiquidity">EInsufficientLiquidity</a>);
    <b>assert</b>!(balance::value(&reservation_pool_object.myso_balance) &gt;= amount, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInsufficientLiquidity">EInsufficientLiquidity</a>);
    <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_apply_reservation_withdrawal_ledger">apply_reservation_withdrawal_ledger</a>(registry, reservation_pool_object, reserver, associated_id, amount);
    <b>if</b> (fee_amount &gt; 0) {
        <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_distribute_reservation_withdraw_fees_platform_profile">distribute_reservation_withdraw_fees_platform_profile</a>(
            pool_owner,
            treasury,
            <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>,
            creator_fee,
            platform_fee,
            treasury_fee,
            &<b>mut</b> reservation_pool_object.myso_balance,
            clock,
            ctx
        );
    };
    <b>let</b> refund_balance = balance::split(&<b>mut</b> reservation_pool_object.myso_balance, net_refund);
    <b>let</b> refund_coin = coin::from_balance(refund_balance, ctx);
    transfer::public_transfer(refund_coin, reserver);
    event::emit(<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ReservationWithdrawnEvent">ReservationWithdrawnEvent</a> {
        associated_id,
        token_type: reservation_pool_object.info.token_type,
        reserver,
        amount,
        total_reserved: reservation_pool_object.info.total_reserved,
        withdrawn_at: now,
        fee_amount,
        creator_fee,
        platform_fee,
        treasury_fee,
    });
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_bootstrap_reservation_pool_for_post_id"></a>

## Function `bootstrap_reservation_pool_for_post_id`

Shared pool-creation logic for create-with-SPT and late-enable entries.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_bootstrap_reservation_pool_for_post_id">bootstrap_reservation_pool_for_post_id</a>(registry: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">social_contracts::social_proof_tokens::TokenRegistry</a>, config: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">social_contracts::social_proof_tokens::SocialProofTokensConfig</a>, associated_id: <b>address</b>, owner: <b>address</b>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_bootstrap_reservation_pool_for_post_id">bootstrap_reservation_pool_for_post_id</a>(
    registry: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">TokenRegistry</a>,
    config: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">SocialProofTokensConfig</a>,
    associated_id: <b>address</b>,
    owner: <b>address</b>,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
): <b>address</b> {
    <b>assert</b>!(config.trading_enabled, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ETradingHalted">ETradingHalted</a>);
    <b>assert</b>!(!table::contains(&registry.reservation_pools, associated_id), <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ETokenAlreadyExists">ETokenAlreadyExists</a>);
    <b>let</b> now = clock::timestamp_ms(clock);
    <b>let</b> required_threshold = config.post_threshold;
    <b>let</b> reservation_pool = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ReservationPool">ReservationPool</a> {
        associated_id,
        token_type: <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TOKEN_TYPE_POST">TOKEN_TYPE_POST</a>,
        owner,
        total_reserved: 0,
        required_threshold,
        created_at: now,
    };
    <b>let</b> reservation_pool_object = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ReservationPoolObject">ReservationPoolObject</a> {
        id: object::new(ctx),
        info: reservation_pool,
        myso_balance: balance::zero(),
        reservations: table::new(ctx),
        reservers: vector::empty(),
        converted: <b>false</b>,
        version: <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(),
    };
    <b>let</b> pool_info = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ReservationPool">ReservationPool</a> {
        associated_id: reservation_pool_object.info.associated_id,
        token_type: reservation_pool_object.info.token_type,
        owner: reservation_pool_object.info.owner,
        total_reserved: reservation_pool_object.info.total_reserved,
        required_threshold: reservation_pool_object.info.required_threshold,
        created_at: reservation_pool_object.info.created_at,
    };
    table::add(&<b>mut</b> registry.reservation_pools, associated_id, pool_info);
    <b>let</b> pool_object_id = object::uid_to_address(&reservation_pool_object.id);
    event::emit(<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ReservationPoolCreatedEvent">ReservationPoolCreatedEvent</a> {
        associated_id,
        token_type: <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TOKEN_TYPE_POST">TOKEN_TYPE_POST</a>,
        owner,
        required_threshold,
        pool_object_id,
        created_at: now,
    });
    transfer::share_object(reservation_pool_object);
    pool_object_id
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_create_post_with_reservation_pool"></a>

## Function `create_post_with_reservation_pool`

Create a post and bootstrap its SPT reservation pool in one transaction.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_create_post_with_reservation_pool">create_post_with_reservation_pool</a>(token_registry: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">social_contracts::social_proof_tokens::TokenRegistry</a>, spt_config: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">social_contracts::social_proof_tokens::SocialProofTokensConfig</a>, registry: &<a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">social_contracts::profile::UsernameRegistry</a>, platform_registry: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">social_contracts::platform::PlatformRegistry</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, post_config: &<a href="../social_contracts/post.md#social_contracts_post_PostConfig">social_contracts::post::PostConfig</a>, memory_config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, content: <a href="../std/string.md#std_string_String">std::string::String</a>, media_urls: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;&gt;, mentions: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;<b>address</b>&gt;&gt;, metadata_json: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, allow_comments: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;bool&gt;, allow_reactions: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;bool&gt;, allow_reposts: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;bool&gt;, allow_quotes: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;bool&gt;, allow_tips: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;bool&gt;, access_kind: u8, subscription_service_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;, linked_mydata_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;, subscription_min_tier_level: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, mydata_registry: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataRegistry">social_contracts::mydata::MyDataRegistry</a>, memory_account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_create_post_with_reservation_pool">create_post_with_reservation_pool</a>(
    token_registry: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">TokenRegistry</a>,
    spt_config: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">SocialProofTokensConfig</a>,
    registry: &UsernameRegistry,
    platform_registry: &PlatformRegistry,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">platform::Platform</a>,
    block_list_registry: &BlockListRegistry,
    post_config: &PostConfig,
    memory_config: &MemoryConfig,
    content: String,
    media_urls: Option&lt;vector&lt;String&gt;&gt;,
    mentions: Option&lt;vector&lt;<b>address</b>&gt;&gt;,
    metadata_json: Option&lt;String&gt;,
    allow_comments: Option&lt;bool&gt;,
    allow_reactions: Option&lt;bool&gt;,
    allow_reposts: Option&lt;bool&gt;,
    allow_quotes: Option&lt;bool&gt;,
    allow_tips: Option&lt;bool&gt;,
    access_kind: u8,
    subscription_service_id: Option&lt;ID&gt;,
    linked_mydata_id: Option&lt;ID&gt;,
    subscription_min_tier_level: Option&lt;u64&gt;,
    mydata_registry: &mydata::MyDataRegistry,
    memory_account: &MemoryAccount,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(spt_config.trading_enabled, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ETradingHalted">ETradingHalted</a>);
    <b>let</b> access = <a href="../social_contracts/post.md#social_contracts_post_post_access_from_parts">post::post_access_from_parts</a>(
        access_kind,
        subscription_service_id,
        linked_mydata_id,
        subscription_min_tier_level,
    );
    <b>let</b> <a href="../social_contracts/post.md#social_contracts_post">post</a> = <a href="../social_contracts/post.md#social_contracts_post_create_post_object_for_spt">post::create_post_object_for_spt</a>(
        registry,
        platform_registry,
        <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>,
        block_list_registry,
        post_config,
        memory_config,
        content,
        vector[],
        media_urls,
        mentions,
        metadata_json,
        allow_comments,
        allow_reactions,
        allow_reposts,
        allow_quotes,
        allow_tips,
        access,
        mydata_registry,
        memory_account,
        clock,
        ctx,
    );
    <b>let</b> associated_id = <a href="../social_contracts/post.md#social_contracts_post_get_id_address">post::get_id_address</a>(&<a href="../social_contracts/post.md#social_contracts_post">post</a>);
    <b>let</b> owner = <a href="../social_contracts/post.md#social_contracts_post_get_post_owner">post::get_post_owner</a>(&<a href="../social_contracts/post.md#social_contracts_post">post</a>);
    <b>let</b> pool_object_id = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_bootstrap_reservation_pool_for_post_id">bootstrap_reservation_pool_for_post_id</a>(
        token_registry,
        spt_config,
        associated_id,
        owner,
        clock,
        ctx,
    );
    <b>let</b> _post_id = <a href="../social_contracts/post.md#social_contracts_post_share_and_emit_spt_post">post::share_and_emit_spt_post</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>, pool_object_id, clock);
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_enable_spt_for_post"></a>

## Function `enable_spt_for_post`

Late-enable SPT on an existing post that is not already SPT-enabled.
Replaces the old <code>create_reservation_pool_for_post</code> entry.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_enable_spt_for_post">enable_spt_for_post</a>(registry: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">social_contracts::social_proof_tokens::TokenRegistry</a>, config: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">social_contracts::social_proof_tokens::SocialProofTokensConfig</a>, <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_enable_spt_for_post">enable_spt_for_post</a>(
    registry: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">TokenRegistry</a>,
    config: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">SocialProofTokensConfig</a>,
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> Post,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> caller = tx_context::sender(ctx);
    <b>let</b> associated_id = <a href="../social_contracts/post.md#social_contracts_post_get_id_address">post::get_id_address</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>);
    <b>let</b> owner = <a href="../social_contracts/post.md#social_contracts_post_get_post_owner">post::get_post_owner</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>);
    <b>assert</b>!(caller == owner, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ENotAuthorized">ENotAuthorized</a>);
    <b>assert</b>!(option::is_none(<a href="../social_contracts/post.md#social_contracts_post_get_spt_id">post::get_spt_id</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>)), <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ESptAlreadyEnabled">ESptAlreadyEnabled</a>);
    <b>assert</b>!(!table::contains(&registry.reservation_pools, associated_id), <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ESptAlreadyEnabled">ESptAlreadyEnabled</a>);
    <b>assert</b>!(!table::contains(&registry.tokens, associated_id), <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ESptAlreadyEnabled">ESptAlreadyEnabled</a>);
    <b>let</b> pool_object_id = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_bootstrap_reservation_pool_for_post_id">bootstrap_reservation_pool_for_post_id</a>(
        registry,
        config,
        associated_id,
        owner,
        clock,
        ctx,
    );
    <a href="../social_contracts/post.md#social_contracts_post_set_enable_spt">post::set_enable_spt</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>, <b>true</b>);
    <a href="../social_contracts/post.md#social_contracts_post_set_spt_id">post::set_spt_id</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>, pool_object_id);
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_create_reservation_pool_for_profile"></a>

## Function `create_reservation_pool_for_profile`

Create a new reservation pool for a profile


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_create_reservation_pool_for_profile">create_reservation_pool_for_profile</a>(registry: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">social_contracts::social_proof_tokens::TokenRegistry</a>, config: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">social_contracts::social_proof_tokens::SocialProofTokensConfig</a>, <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<a href="../social_contracts/profile.md#social_contracts_profile_Profile">social_contracts::profile::Profile</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_create_reservation_pool_for_profile">create_reservation_pool_for_profile</a>(
    registry: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">TokenRegistry</a>,
    config: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">SocialProofTokensConfig</a>,
    <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &Profile,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    // Check <b>if</b> trading is halted
    <b>assert</b>!(config.trading_enabled, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ETradingHalted">ETradingHalted</a>);
    <b>let</b> caller = tx_context::sender(ctx);
    <b>let</b> associated_id = <a href="../social_contracts/profile.md#social_contracts_profile_get_id_address">profile::get_id_address</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>);
    <b>let</b> owner = <a href="../social_contracts/profile.md#social_contracts_profile_owner">profile::owner</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>);
    // Verify caller is the actual <a href="../social_contracts/profile.md#social_contracts_profile">profile</a> owner
    <b>assert</b>!(caller == owner, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ENotAuthorized">ENotAuthorized</a>);
    // Verify <a href="../social_contracts/profile.md#social_contracts_profile">profile</a> ID matches
    <b>assert</b>!(associated_id == <a href="../social_contracts/profile.md#social_contracts_profile_get_id_address">profile::get_id_address</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>), <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInvalidID">EInvalidID</a>);
    // Check <b>if</b> reservation pool already exists
    <b>assert</b>!(!table::contains(&registry.reservation_pools, associated_id), <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ETokenAlreadyExists">ETokenAlreadyExists</a>);
    <b>let</b> now = clock::timestamp_ms(clock);
    <b>let</b> required_threshold = config.profile_threshold;
    // Create reservation pool info (without reservers vector - only in <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ReservationPoolObject">ReservationPoolObject</a>)
    <b>let</b> reservation_pool = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ReservationPool">ReservationPool</a> {
        associated_id,
        token_type: <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TOKEN_TYPE_PROFILE">TOKEN_TYPE_PROFILE</a>,
        owner,
        total_reserved: 0,
        required_threshold,
        created_at: now,
    };
    // Create reservation pool object first (before moving reservation_pool)
    <b>let</b> reservation_pool_object = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ReservationPoolObject">ReservationPoolObject</a> {
        id: object::new(ctx),
        info: reservation_pool,
        myso_balance: balance::zero(),
        reservations: table::new(ctx),
        reservers: vector::empty(),
        converted: <b>false</b>,
        version: <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(),
    };
    // Add to registry (reconstruct <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ReservationPool">ReservationPool</a> from object's info since original was moved)
    <b>let</b> pool_info = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ReservationPool">ReservationPool</a> {
        associated_id: reservation_pool_object.info.associated_id,
        token_type: reservation_pool_object.info.token_type,
        owner: reservation_pool_object.info.owner,
        total_reserved: reservation_pool_object.info.total_reserved,
        required_threshold: reservation_pool_object.info.required_threshold,
        created_at: reservation_pool_object.info.created_at,
    };
    table::add(&<b>mut</b> registry.reservation_pools, associated_id, pool_info);
    <b>let</b> pool_object_id = object::uid_to_address(&reservation_pool_object.id);
    // Emit reservation pool created event
    event::emit(<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ReservationPoolCreatedEvent">ReservationPoolCreatedEvent</a> {
        associated_id,
        token_type: <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TOKEN_TYPE_PROFILE">TOKEN_TYPE_PROFILE</a>,
        owner,
        required_threshold,
        pool_object_id,
        created_at: now,
    });
    transfer::share_object(reservation_pool_object);
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_can_create_auction"></a>

## Function `can_create_auction`

Check if reservation threshold is met for auction creation


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_can_create_auction">can_create_auction</a>(registry: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">social_contracts::social_proof_tokens::TokenRegistry</a>, config: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">social_contracts::social_proof_tokens::SocialProofTokensConfig</a>, associated_id: <b>address</b>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_can_create_auction">can_create_auction</a>(
    registry: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">TokenRegistry</a>,
    config: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">SocialProofTokensConfig</a>,
    associated_id: <b>address</b>
): bool {
    <b>if</b> (!table::contains(&registry.reservation_pools, associated_id)) {
        <b>return</b> <b>false</b>
    };
    <b>let</b> reservation_pool = table::borrow(&registry.reservation_pools, associated_id);
    <b>let</b> required_threshold = <b>if</b> (reservation_pool.token_type == <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TOKEN_TYPE_PROFILE">TOKEN_TYPE_PROFILE</a>) {
        config.profile_threshold
    } <b>else</b> {
        config.post_threshold
    };
    reservation_pool.total_reserved &gt;= required_threshold
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_create_social_proof_token"></a>

## Function `create_social_proof_token`

Create a social proof token directly from a reservation pool once threshold is met
This replaces the auction system - only the post/profile owner can call this


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_create_social_proof_token">create_social_proof_token</a>(registry: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">social_contracts::social_proof_tokens::TokenRegistry</a>, config: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">social_contracts::social_proof_tokens::SocialProofTokensConfig</a>, reservation_pool_object: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ReservationPoolObject">social_contracts::social_proof_tokens::ReservationPoolObject</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_create_social_proof_token">create_social_proof_token</a>(
    registry: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">TokenRegistry</a>,
    config: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">SocialProofTokensConfig</a>,
    reservation_pool_object: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ReservationPoolObject">ReservationPoolObject</a>,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    // Check <b>if</b> trading is halted
    <b>assert</b>!(config.trading_enabled, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ETradingHalted">ETradingHalted</a>);
    <b>let</b> caller = tx_context::sender(ctx);
    <b>let</b> associated_id = reservation_pool_object.info.associated_id;
    // Verify caller is the owner of the <a href="../social_contracts/post.md#social_contracts_post">post</a>/<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>
    <b>assert</b>!(caller == reservation_pool_object.info.owner, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ENotAuthorized">ENotAuthorized</a>);
    // Check <b>if</b> reservation threshold <b>has</b> been met
    <b>assert</b>!(<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_can_create_auction">can_create_auction</a>(registry, config, associated_id), <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EViralThresholdNotMet">EViralThresholdNotMet</a>);
    // Verify token <b>has</b> not already been created
    <b>assert</b>!(!table::contains(&registry.tokens, associated_id), <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ETokenAlreadyExists">ETokenAlreadyExists</a>);
    // Initial nano-SPT supply: net nano-MYSO reserved × (nano-SPT per whole SPT) / `base_price`,
    // so implied cost per display SPT at the linear curve leg matches reservation (same `base_price`
    // stored on the pool). Reservers still split this supply proportionally by reservation_amount.
    <b>let</b> total_reserved = reservation_pool_object.info.total_reserved;
    <b>assert</b>!(total_reserved &gt; 0, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ENoContribution">ENoContribution</a>);
    <b>let</b> base_price = config.base_price;
    <b>let</b> product_u128 = (total_reserved <b>as</b> u128) * (<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SPT_SCALE">SPT_SCALE</a> <b>as</b> u128);
    <b>assert</b>!(product_u128 &lt;= <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_MAX_ONCHAIN_U64_U128">MAX_ONCHAIN_U64_U128</a>, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EOverflow">EOverflow</a>);
    <b>let</b> initial_u128 = product_u128 / (base_price <b>as</b> u128);
    <b>assert</b>!(initial_u128 &gt; 0 && initial_u128 &lt;= <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_MAX_ONCHAIN_U64_U128">MAX_ONCHAIN_U64_U128</a>, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInvalidCurveParams">EInvalidCurveParams</a>);
    <b>let</b> initial_token_supply = initial_u128 <b>as</b> u64;
    // Create token info
    <b>let</b> token_info = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenInfo">TokenInfo</a> {
        id: @0x0, // Temporary, will be updated
        token_type: reservation_pool_object.info.token_type,
        owner: reservation_pool_object.info.owner,
        associated_id,
        circulating_supply: initial_token_supply,
        base_price: config.base_price,
        quadratic_coefficient: config.quadratic_coefficient,
        created_at: clock::timestamp_ms(clock),
    };
    // Create token pool
    <b>let</b> pool_id = object::new(ctx);
    <b>let</b> pool_address = object::uid_to_address(&pool_id);
    // Update token info with actual pool <b>address</b>
    <b>let</b> <b>mut</b> updated_token_info = token_info;
    updated_token_info.id = pool_address;
    // Create a <b>copy</b> of token info <b>for</b> the pool (since <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenInfo">TokenInfo</a> doesn't have <b>copy</b>, we'll reconstruct)
    <b>let</b> pool_token_info = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenInfo">TokenInfo</a> {
        id: updated_token_info.id,
        token_type: updated_token_info.token_type,
        owner: updated_token_info.owner,
        associated_id: updated_token_info.associated_id,
        circulating_supply: updated_token_info.circulating_supply,
        base_price: updated_token_info.base_price,
        quadratic_coefficient: updated_token_info.quadratic_coefficient,
        created_at: updated_token_info.created_at,
    };
    <b>let</b> <b>mut</b> token_pool = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">TokenPool</a> {
        id: pool_id,
        info: pool_token_info,
        myso_balance: balance::zero(),
        holders: table::new(ctx),
        revenue_manifest: option::none(),
        version: <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(),
    };
    // Distribute tokens to reservers proportionally
    // Limit number of reservers to prevent DoS via gas exhaustion
    <b>let</b> reservers = &reservation_pool_object.reservers;
    <b>let</b> num_reservers = vector::length(reservers);
    <b>assert</b>!(num_reservers &lt;= config.max_reservers_per_pool, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ETooManyReservers">ETooManyReservers</a>);
    <b>let</b> <b>mut</b> distributed_total = 0;
    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (i &lt; num_reservers) {
        <b>let</b> reserver = *vector::borrow(reservers, i);
        <b>let</b> reservation_amount = *table::borrow(&reservation_pool_object.reservations, reserver);
        // Calculate token amount based on reserver's proportion of total reservation
        // Use u128 to avoid overflow when reservation_amount * initial_token_supply exceeds u64
        <b>let</b> token_amount = (((reservation_amount <b>as</b> u128) * (initial_token_supply <b>as</b> u128)) / (total_reserved <b>as</b> u128)) <b>as</b> u64;
        <b>if</b> (token_amount &gt; 0) {
            // Update holder's balance in the pool
            // Handle duplicate reservers: <b>if</b> already exists, add to existing balance
            <b>if</b> (table::contains(&token_pool.holders, reserver)) {
                <b>let</b> existing_balance = table::borrow_mut(&<b>mut</b> token_pool.holders, reserver);
                <b>assert</b>!(*existing_balance &lt;= <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_MAX_U64">MAX_U64</a> - token_amount, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EOverflow">EOverflow</a>);
                *existing_balance = *existing_balance + token_amount;
            } <b>else</b> {
                table::add(&<b>mut</b> token_pool.holders, reserver, token_amount);
            };
            // Track distributed tokens to ensure accurate circulating supply
            <b>assert</b>!(distributed_total &lt;= <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_MAX_U64">MAX_U64</a> - token_amount, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EOverflow">EOverflow</a>);
            distributed_total = distributed_total + token_amount;
            // Create social token <b>for</b> the reserver
            <b>let</b> social_token = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialToken">SocialToken</a> {
                id: object::new(ctx),
                pool_id: pool_address,
                token_type: reservation_pool_object.info.token_type,
                amount: token_amount,
            };
            // Transfer social token to reserver
            transfer::transfer(social_token, reserver);
        };
        i = i + 1;
    };
    // Handle rounding remainder: allocate any undistributed tokens to the owner
    <b>let</b> remainder = initial_token_supply - distributed_total;
    <b>if</b> (remainder &gt; 0) {
        // Allocate remainder to the owner
        <b>if</b> (table::contains(&token_pool.holders, reservation_pool_object.info.owner)) {
            <b>let</b> owner_balance = table::borrow_mut(&<b>mut</b> token_pool.holders, reservation_pool_object.info.owner);
            <b>assert</b>!(*owner_balance &lt;= <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_MAX_U64">MAX_U64</a> - remainder, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EOverflow">EOverflow</a>);
            *owner_balance = *owner_balance + remainder;
        } <b>else</b> {
            table::add(&<b>mut</b> token_pool.holders, reservation_pool_object.info.owner, remainder);
        };
        distributed_total = distributed_total + remainder;
    };
    // Update circulating supply to match actually distributed tokens
    token_pool.info.circulating_supply = distributed_total;
    updated_token_info.circulating_supply = distributed_total;
    // Transfer all reserved MYSO to the token pool <b>as</b> initial liquidity
    balance::join(&<b>mut</b> token_pool.myso_balance, balance::withdraw_all(&<b>mut</b> reservation_pool_object.myso_balance));
    // Snapshot <b>for</b> event (denominator <b>for</b> indexer proportional split) before clearing on-chain state
    <b>let</b> total_reserved_at_launch = reservation_pool_object.info.total_reserved;
    // Mark reservation pool <b>as</b> converted and clear total reserved
    reservation_pool_object.converted = <b>true</b>;
    reservation_pool_object.info.total_reserved = 0;
    // Update registry reservation pool <b>entry</b> to reflect conversion
    <b>if</b> (table::contains(&registry.reservation_pools, associated_id)) {
        <b>let</b> registry_pool = table::borrow_mut(&<b>mut</b> registry.reservation_pools, associated_id);
        registry_pool.total_reserved = 0;
    };
    // Add to registry (<b>use</b> updated_token_info which <b>has</b> the correct circulating_supply)
    table::add(&<b>mut</b> registry.tokens, associated_id, updated_token_info);
    // Emit token created event (<b>use</b> token_pool.info which <b>has</b> the final state)
    event::emit(<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPoolCreatedEvent">TokenPoolCreatedEvent</a> {
        id: pool_address,
        token_type: token_pool.info.token_type,
        owner: token_pool.info.owner,
        associated_id: token_pool.info.associated_id,
        base_price: token_pool.info.base_price,
        quadratic_coefficient: token_pool.info.quadratic_coefficient,
        circulating_supply: token_pool.info.circulating_supply,
        total_reserved_at_launch,
    });
    // Share the token pool
    transfer::share_object(token_pool);
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_sync_token_pool_manifest_from_post"></a>

## Function `sync_token_pool_manifest_from_post`

Copy <code><a href="../social_contracts/post.md#social_contracts_post_revenue_manifest">post::revenue_manifest</a></code> into a matching POST <code><a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">TokenPool</a></code> and emit <code><a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_PocRedirectionUpdatedEvent">PocRedirectionUpdatedEvent</a></code>.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_sync_token_pool_manifest_from_post">sync_token_pool_manifest_from_post</a>(registry: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">social_contracts::social_proof_tokens::TokenRegistry</a>, pool: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">social_contracts::social_proof_tokens::TokenPool</a>, <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, updated_by: <b>address</b>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, _ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_sync_token_pool_manifest_from_post">sync_token_pool_manifest_from_post</a>(
    registry: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">TokenRegistry</a>,
    pool: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">TokenPool</a>,
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &Post,
    updated_by: <b>address</b>,
    clock: &Clock,
    _ctx: &TxContext,
) {
    <b>assert</b>!(pool.info.token_type == <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TOKEN_TYPE_POST">TOKEN_TYPE_POST</a>, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInvalidTokenType">EInvalidTokenType</a>);
    <b>let</b> post_id = <a href="../social_contracts/post.md#social_contracts_post_get_id_address">post::get_id_address</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>);
    <b>assert</b>!(post_id == pool.info.associated_id, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInvalidID">EInvalidID</a>);
    <b>assert</b>!(<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_token_exists">token_exists</a>(registry, post_id), <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ETokenNotFound">ETokenNotFound</a>);
    pool.revenue_manifest = <a href="../social_contracts/post.md#social_contracts_post_revenue_manifest">post::revenue_manifest</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>);
    event::emit(<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_PocRedirectionUpdatedEvent">PocRedirectionUpdatedEvent</a> {
        pool_id: object::uid_to_address(&pool.id),
        post_id,
        redirect_to: option::none(),
        redirect_percentage: option::none(),
        poc_redirection_kind: 0,
        updated_by,
        timestamp: clock::timestamp_ms(clock),
    });
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_sync_token_pool_poc_from_post"></a>

## Function `sync_token_pool_poc_from_post`

Legacy alias for manifest sync callers during migration.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_sync_token_pool_poc_from_post">sync_token_pool_poc_from_post</a>(registry: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">social_contracts::social_proof_tokens::TokenRegistry</a>, pool: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">social_contracts::social_proof_tokens::TokenPool</a>, <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, updated_by: <b>address</b>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_sync_token_pool_poc_from_post">sync_token_pool_poc_from_post</a>(
    registry: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">TokenRegistry</a>,
    pool: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">TokenPool</a>,
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &Post,
    updated_by: <b>address</b>,
    clock: &Clock,
    ctx: &TxContext,
) {
    <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_sync_token_pool_manifest_from_post">sync_token_pool_manifest_from_post</a>(registry, pool, <a href="../social_contracts/post.md#social_contracts_post">post</a>, updated_by, clock, ctx);
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_update_token_poc_data"></a>

## Function `update_token_poc_data`

Update PoC redirection data for a token pool (called by PoC system)
This function copies PoC data from a post into the corresponding token pool


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_update_token_poc_data">update_token_poc_data</a>(registry: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">social_contracts::social_proof_tokens::TokenRegistry</a>, pool: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">social_contracts::social_proof_tokens::TokenPool</a>, <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_update_token_poc_data">update_token_poc_data</a>(
    registry: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">TokenRegistry</a>,
    pool: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">TokenPool</a>,
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &Post,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> caller = tx_context::sender(ctx);
    <b>assert</b>!(caller == <a href="../social_contracts/post.md#social_contracts_post_get_post_owner">post::get_post_owner</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>), <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ENotAuthorized">ENotAuthorized</a>);
    <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_sync_token_pool_manifest_from_post">sync_token_pool_manifest_from_post</a>(registry, pool, <a href="../social_contracts/post.md#social_contracts_post">post</a>, caller, clock, ctx);
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_pool_manifest_has_escrow_payout"></a>

## Function `pool_manifest_has_escrow_payout`



<pre><code><b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_pool_manifest_has_escrow_payout">pool_manifest_has_escrow_payout</a>(pool: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">social_contracts::social_proof_tokens::TokenPool</a>, amount: u64): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_pool_manifest_has_escrow_payout">pool_manifest_has_escrow_payout</a>(pool: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">TokenPool</a>, amount: u64): bool {
    <b>if</b> (amount == 0 || option::is_none(&pool.revenue_manifest)) {
        <b>return</b> <b>false</b>
    };
    <a href="../social_contracts/media_asset.md#social_contracts_media_asset_manifest_has_escrow_payout">media_asset::manifest_has_escrow_payout</a>(option::borrow(&pool.revenue_manifest), amount)
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_should_apply_pool_revenue_manifest"></a>

## Function `should_apply_pool_revenue_manifest`



<pre><code><b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_should_apply_pool_revenue_manifest">should_apply_pool_revenue_manifest</a>(pool: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">social_contracts::social_proof_tokens::TokenPool</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_should_apply_pool_revenue_manifest">should_apply_pool_revenue_manifest</a>(pool: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">TokenPool</a>): bool {
    pool.info.token_type == <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TOKEN_TYPE_POST">TOKEN_TYPE_POST</a> && option::is_some(&pool.revenue_manifest)
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_apply_pool_revenue_manifest_coin"></a>

## Function `apply_pool_revenue_manifest_coin`



<pre><code><b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_apply_pool_revenue_manifest_coin">apply_pool_revenue_manifest_coin</a>(pool: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">social_contracts::social_proof_tokens::TokenPool</a>, intended_recipient: <b>address</b>, amount: u64, coins: &<b>mut</b> <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_apply_pool_revenue_manifest_coin">apply_pool_revenue_manifest_coin</a>(
    pool: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">TokenPool</a>,
    intended_recipient: <b>address</b>,
    amount: u64,
    coins: &<b>mut</b> Coin&lt;MYSO&gt;,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> manifest = option::borrow(&pool.revenue_manifest);
    <b>let</b> entries = <a href="../social_contracts/media_asset.md#social_contracts_media_asset_manifest_entries">media_asset::manifest_entries</a>(manifest);
    <b>let</b> len = vector::length(entries);
    <b>let</b> bps_total = <a href="../social_contracts/media_asset.md#social_contracts_media_asset_manifest_bps_total">media_asset::manifest_bps_total</a>();
    <b>let</b> <b>mut</b> fee_coins = coin::split(coins, amount, ctx);
    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (i &lt; len) {
        <b>let</b> e = vector::borrow(entries, i);
        <b>let</b> slice = (amount * <a href="../social_contracts/media_asset.md#social_contracts_media_asset_manifest_entry_share_bps">media_asset::manifest_entry_share_bps</a>(e)) / bps_total;
        <b>if</b> (slice &gt; 0) {
            <b>let</b> pay_coins = coin::split(&<b>mut</b> fee_coins, slice, ctx);
            <b>assert</b>!(<a href="../social_contracts/media_asset.md#social_contracts_media_asset_manifest_entry_payout_mode">media_asset::manifest_entry_payout_mode</a>(e) == <a href="../social_contracts/media_asset.md#social_contracts_media_asset_payout_wallet">media_asset::payout_wallet</a>(), <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EPostPoolEscrowTradingBlocked">EPostPoolEscrowTradingBlocked</a>);
            transfer::public_transfer(pay_coins, <a href="../social_contracts/media_asset.md#social_contracts_media_asset_manifest_entry_beneficiary">media_asset::manifest_entry_beneficiary</a>(e));
        };
        i = i + 1;
    };
    <b>let</b> remainder = coin::value(&fee_coins);
    <b>if</b> (remainder &gt; 0) {
        transfer::public_transfer(fee_coins, intended_recipient);
    } <b>else</b> {
        coin::destroy_zero(fee_coins);
    };
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_apply_post_revenue_manifest_coin"></a>

## Function `apply_post_revenue_manifest_coin`



<pre><code><b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_apply_post_revenue_manifest_coin">apply_post_revenue_manifest_coin</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, beneficiary_vault: &<b>mut</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_PoCBeneficiaryVault">social_contracts::poc_vault::PoCBeneficiaryVault</a>, intended_recipient: <b>address</b>, amount: u64, coins: &<b>mut</b> <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, object_id: <b>address</b>, min_vault_deposit_amount: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_apply_post_revenue_manifest_coin">apply_post_revenue_manifest_coin</a>(
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &Post,
    beneficiary_vault: &<b>mut</b> PoCBeneficiaryVault,
    intended_recipient: <b>address</b>,
    amount: u64,
    coins: &<b>mut</b> Coin&lt;MYSO&gt;,
    object_id: <b>address</b>,
    min_vault_deposit_amount: u64,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <b>if</b> (!<a href="../social_contracts/post.md#social_contracts_post_monetization_enabled">post::monetization_enabled</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>)) {
        <b>let</b> fee_coin = coin::split(coins, amount, ctx);
        transfer::public_transfer(fee_coin, intended_recipient);
        <b>return</b>
    };
    <b>let</b> manifest_opt = <a href="../social_contracts/post.md#social_contracts_post_revenue_manifest">post::revenue_manifest</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>);
    <b>if</b> (option::is_none(&manifest_opt)) {
        <b>let</b> fee_coin = coin::split(coins, amount, ctx);
        transfer::public_transfer(fee_coin, intended_recipient);
        <b>return</b>
    };
    <b>let</b> manifest = option::borrow(&manifest_opt);
    <b>let</b> entries = <a href="../social_contracts/media_asset.md#social_contracts_media_asset_manifest_entries">media_asset::manifest_entries</a>(manifest);
    <b>let</b> len = vector::length(entries);
    <b>let</b> bps_total = <a href="../social_contracts/media_asset.md#social_contracts_media_asset_manifest_bps_total">media_asset::manifest_bps_total</a>();
    <b>let</b> <b>mut</b> fee_coins = coin::split(coins, amount, ctx);
    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (i &lt; len) {
        <b>let</b> e = vector::borrow(entries, i);
        <b>let</b> slice = (amount * <a href="../social_contracts/media_asset.md#social_contracts_media_asset_manifest_entry_share_bps">media_asset::manifest_entry_share_bps</a>(e)) / bps_total;
        <b>if</b> (slice &gt; 0) {
            <b>let</b> pay_coins = coin::split(&<b>mut</b> fee_coins, slice, ctx);
            <b>if</b> (<a href="../social_contracts/media_asset.md#social_contracts_media_asset_manifest_entry_payout_mode">media_asset::manifest_entry_payout_mode</a>(e) == <a href="../social_contracts/media_asset.md#social_contracts_media_asset_payout_escrow">media_asset::payout_escrow</a>()) {
                <b>assert</b>!(<a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_beneficiary_address">poc_vault::beneficiary_address</a>(beneficiary_vault) == <a href="../social_contracts/media_asset.md#social_contracts_media_asset_manifest_entry_beneficiary">media_asset::manifest_entry_beneficiary</a>(e), <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInvalidFeeConfig">EInvalidFeeConfig</a>);
                <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_deposit_coin">poc_vault::deposit_coin</a>&lt;MYSO&gt;(
                    beneficiary_vault,
                    <a href="../social_contracts/media_asset.md#social_contracts_media_asset_manifest_entry_beneficiary">media_asset::manifest_entry_beneficiary</a>(e),
                    pay_coins,
                    option::some(object_id),
                    min_vault_deposit_amount,
                    clock,
                    ctx
                );
            } <b>else</b> {
                transfer::public_transfer(pay_coins, <a href="../social_contracts/media_asset.md#social_contracts_media_asset_manifest_entry_beneficiary">media_asset::manifest_entry_beneficiary</a>(e));
            };
        };
        i = i + 1;
    };
    <b>let</b> remainder = coin::value(&fee_coins);
    <b>if</b> (remainder &gt; 0) {
        transfer::public_transfer(fee_coins, intended_recipient);
    } <b>else</b> {
        coin::destroy_zero(fee_coins);
    };
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_distribute_creator_fee"></a>

## Function `distribute_creator_fee`

Distribute creator fees with automatic manifest-based revenue routing


<pre><code><b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_distribute_creator_fee">distribute_creator_fee</a>(pool: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">social_contracts::social_proof_tokens::TokenPool</a>, creator_fee_amount: u64, creator_fee_coin: &<b>mut</b> <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_distribute_creator_fee">distribute_creator_fee</a>(
    pool: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">TokenPool</a>,
    creator_fee_amount: u64,
    creator_fee_coin: &<b>mut</b> Coin&lt;MYSO&gt;,
    ctx: &<b>mut</b> TxContext
) {
    <b>if</b> (creator_fee_amount == 0) {
        <b>return</b>
    };
    <b>assert</b>!(
        !(pool.info.token_type == <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TOKEN_TYPE_POST">TOKEN_TYPE_POST</a> && <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_pool_manifest_has_escrow_payout">pool_manifest_has_escrow_payout</a>(pool, creator_fee_amount)),
        <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EPostPoolEscrowTradingBlocked">EPostPoolEscrowTradingBlocked</a>
    );
    <b>if</b> (<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_should_apply_pool_revenue_manifest">should_apply_pool_revenue_manifest</a>(pool)) {
        <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_apply_pool_revenue_manifest_coin">apply_pool_revenue_manifest_coin</a>(pool, pool.info.owner, creator_fee_amount, creator_fee_coin, ctx);
    } <b>else</b> {
        <b>let</b> fee_coin = coin::split(creator_fee_coin, creator_fee_amount, ctx);
        transfer::public_transfer(fee_coin, pool.info.owner);
    };
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_distribute_creator_fee_from_pool"></a>

## Function `distribute_creator_fee_from_pool`

Distribute creator fees from pool balance with manifest-based revenue routing


<pre><code><b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_distribute_creator_fee_from_pool">distribute_creator_fee_from_pool</a>(pool: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">social_contracts::social_proof_tokens::TokenPool</a>, creator_fee: u64, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_distribute_creator_fee_from_pool">distribute_creator_fee_from_pool</a>(
    pool: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">TokenPool</a>,
    creator_fee: u64,
    ctx: &<b>mut</b> TxContext
) {
    <b>if</b> (creator_fee == 0) {
        <b>return</b>
    };
    <b>assert</b>!(
        !(pool.info.token_type == <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TOKEN_TYPE_POST">TOKEN_TYPE_POST</a> && <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_pool_manifest_has_escrow_payout">pool_manifest_has_escrow_payout</a>(pool, creator_fee)),
        <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EPostPoolEscrowTradingBlocked">EPostPoolEscrowTradingBlocked</a>
    );
    <b>if</b> (<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_should_apply_pool_revenue_manifest">should_apply_pool_revenue_manifest</a>(pool)) {
        <b>let</b> <b>mut</b> fee_coin = coin::from_balance(balance::split(&<b>mut</b> pool.myso_balance, creator_fee), ctx);
        <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_apply_pool_revenue_manifest_coin">apply_pool_revenue_manifest_coin</a>(pool, pool.info.owner, creator_fee, &<b>mut</b> fee_coin, ctx);
        coin::destroy_zero(fee_coin);
    } <b>else</b> {
        <b>let</b> fee_coin = coin::from_balance(balance::split(&<b>mut</b> pool.myso_balance, creator_fee), ctx);
        transfer::public_transfer(fee_coin, pool.info.owner);
    };
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_apply_post_poc_redirection"></a>

## Function `apply_post_poc_redirection`

Legacy alias retained for package callers during migration.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_apply_post_poc_redirection">apply_post_poc_redirection</a>(_post: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, amount: u64): (u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_apply_post_poc_redirection">apply_post_poc_redirection</a>(
    _post: &Post,
    amount: u64
): (u64, u64) {
    (0, amount)
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_distribute_reservation_creator_fee_with_owner"></a>

## Function `distribute_reservation_creator_fee_with_owner`

PoC-aware post reservation creator fee using an explicit pool owner (avoids borrow conflicts
when paying from <code>&<b>mut</b> reservation_pool_object.myso_balance</code> during withdrawal).


<pre><code><b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_distribute_reservation_creator_fee_with_owner">distribute_reservation_creator_fee_with_owner</a>(pool_owner: <b>address</b>, <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, beneficiary_vault: &<b>mut</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_PoCBeneficiaryVault">social_contracts::poc_vault::PoCBeneficiaryVault</a>, creator_fee_amount: u64, creator_fee_coin: &<b>mut</b> <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, min_vault_deposit_amount: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_distribute_reservation_creator_fee_with_owner">distribute_reservation_creator_fee_with_owner</a>(
    pool_owner: <b>address</b>,
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &Post,
    beneficiary_vault: &<b>mut</b> PoCBeneficiaryVault,
    creator_fee_amount: u64,
    creator_fee_coin: &<b>mut</b> Coin&lt;MYSO&gt;,
    min_vault_deposit_amount: u64,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <b>if</b> (creator_fee_amount == 0) {
        <b>return</b>
    };
    <b>if</b> (<a href="../social_contracts/post.md#social_contracts_post_monetization_enabled">post::monetization_enabled</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>) && option::is_some(&<a href="../social_contracts/post.md#social_contracts_post_revenue_manifest">post::revenue_manifest</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>))) {
        <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_apply_post_revenue_manifest_coin">apply_post_revenue_manifest_coin</a>(
            <a href="../social_contracts/post.md#social_contracts_post">post</a>,
            beneficiary_vault,
            pool_owner,
            creator_fee_amount,
            creator_fee_coin,
            <a href="../social_contracts/post.md#social_contracts_post_get_id_address">post::get_id_address</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>),
            min_vault_deposit_amount,
            clock,
            ctx
        );
    } <b>else</b> {
        <b>let</b> fee_coin = coin::split(creator_fee_coin, creator_fee_amount, ctx);
        transfer::public_transfer(fee_coin, pool_owner);
    };
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_distribute_reservation_creator_fee"></a>

## Function `distribute_reservation_creator_fee`

Distribute creator fees with PoC redirection from post (reuses existing pattern)
This follows the same logic as distribute_creator_fee but works with Post instead of TokenPool


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_distribute_reservation_creator_fee">distribute_reservation_creator_fee</a>(reservation_pool: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ReservationPoolObject">social_contracts::social_proof_tokens::ReservationPoolObject</a>, <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, beneficiary_vault: &<b>mut</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_PoCBeneficiaryVault">social_contracts::poc_vault::PoCBeneficiaryVault</a>, creator_fee_amount: u64, creator_fee_coin: &<b>mut</b> <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, min_vault_deposit_amount: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_distribute_reservation_creator_fee">distribute_reservation_creator_fee</a>(
    reservation_pool: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ReservationPoolObject">ReservationPoolObject</a>,
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &Post,
    beneficiary_vault: &<b>mut</b> PoCBeneficiaryVault,
    creator_fee_amount: u64,
    creator_fee_coin: &<b>mut</b> Coin&lt;MYSO&gt;,
    min_vault_deposit_amount: u64,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_distribute_reservation_creator_fee_with_owner">distribute_reservation_creator_fee_with_owner</a>(
        reservation_pool.info.owner,
        <a href="../social_contracts/post.md#social_contracts_post">post</a>,
        beneficiary_vault,
        creator_fee_amount,
        creator_fee_coin,
        min_vault_deposit_amount,
        clock,
        ctx
    );
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_distribute_reservation_creator_fee_no_poc_with_owner"></a>

## Function `distribute_reservation_creator_fee_no_poc_with_owner`



<pre><code><b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_distribute_reservation_creator_fee_no_poc_with_owner">distribute_reservation_creator_fee_no_poc_with_owner</a>(pool_owner: <b>address</b>, creator_fee_amount: u64, creator_fee_coin: &<b>mut</b> <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_distribute_reservation_creator_fee_no_poc_with_owner">distribute_reservation_creator_fee_no_poc_with_owner</a>(
    pool_owner: <b>address</b>,
    creator_fee_amount: u64,
    creator_fee_coin: &<b>mut</b> Coin&lt;MYSO&gt;,
    ctx: &<b>mut</b> TxContext
) {
    <b>if</b> (creator_fee_amount == 0) {
        <b>return</b>
    };
    <b>let</b> fee_coin = coin::split(creator_fee_coin, creator_fee_amount, ctx);
    transfer::public_transfer(fee_coin, pool_owner);
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_distribute_reservation_creator_fee_no_poc"></a>

## Function `distribute_reservation_creator_fee_no_poc`

Distribute creator fees without PoC (for profile reservations)


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_distribute_reservation_creator_fee_no_poc">distribute_reservation_creator_fee_no_poc</a>(reservation_pool: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ReservationPoolObject">social_contracts::social_proof_tokens::ReservationPoolObject</a>, creator_fee_amount: u64, creator_fee_coin: &<b>mut</b> <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_distribute_reservation_creator_fee_no_poc">distribute_reservation_creator_fee_no_poc</a>(
    reservation_pool: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ReservationPoolObject">ReservationPoolObject</a>,
    creator_fee_amount: u64,
    creator_fee_coin: &<b>mut</b> Coin&lt;MYSO&gt;,
    ctx: &<b>mut</b> TxContext
) {
    <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_distribute_reservation_creator_fee_no_poc_with_owner">distribute_reservation_creator_fee_no_poc_with_owner</a>(
        reservation_pool.info.owner,
        creator_fee_amount,
        creator_fee_coin,
        ctx
    );
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_distribute_reservation_fees_with_post"></a>

## Function `distribute_reservation_fees_with_post`

Calculate and distribute all reservation fees (for post reservations with PoC)
Non-platform version: split platform fee 50/50 between creator and treasury; emit platform_fee 0


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_distribute_reservation_fees_with_post">distribute_reservation_fees_with_post</a>(config: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">social_contracts::social_proof_tokens::SocialProofTokensConfig</a>, min_vault_deposit_amount: u64, reservation_pool: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ReservationPoolObject">social_contracts::social_proof_tokens::ReservationPoolObject</a>, <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, beneficiary_vault: &<b>mut</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_PoCBeneficiaryVault">social_contracts::poc_vault::PoCBeneficiaryVault</a>, amount: u64, payment: <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): (<a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, u64, u64, u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_distribute_reservation_fees_with_post">distribute_reservation_fees_with_post</a>(
    config: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">SocialProofTokensConfig</a>,
    min_vault_deposit_amount: u64,
    reservation_pool: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ReservationPoolObject">ReservationPoolObject</a>,
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &Post,
    beneficiary_vault: &<b>mut</b> PoCBeneficiaryVault,
    amount: u64,
    <b>mut</b> payment: Coin&lt;MYSO&gt;,
    treasury: &EcosystemTreasury,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
): (Coin&lt;MYSO&gt;, u64, u64, u64, u64) {
    // Validate fees and calculate with overflow protection
    <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_validate_reservation_fees">validate_reservation_fees</a>(config);
    <b>let</b> reservation_total_fee_bps = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_reservation_total_fee_bps">calculate_reservation_total_fee_bps</a>(config);
    <b>let</b> fee_amount = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_fee_amount_safe">calculate_fee_amount_safe</a>(amount, reservation_total_fee_bps);
    <b>let</b> creator_fee = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_component_fee_safe">calculate_component_fee_safe</a>(fee_amount, config.reservation_creator_fee_bps, reservation_total_fee_bps);
    <b>let</b> platform_fee = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_component_fee_safe">calculate_component_fee_safe</a>(fee_amount, config.reservation_platform_fee_bps, reservation_total_fee_bps);
    <b>let</b> treasury_fee = fee_amount - creator_fee - platform_fee;
    <b>let</b> platform_fee_to_creator = (platform_fee * config.non_platform_platform_to_creator_bps) / <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_BPS_DENOM">BPS_DENOM</a>;
    <b>let</b> platform_fee_to_treasury = platform_fee - platform_fee_to_creator;
    <b>if</b> (fee_amount &gt; 0) {
        <b>let</b> creator_total = creator_fee + platform_fee_to_creator;
        <b>if</b> (creator_total &gt; 0) {
            <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_distribute_reservation_creator_fee">distribute_reservation_creator_fee</a>(
                reservation_pool,
                <a href="../social_contracts/post.md#social_contracts_post">post</a>,
                beneficiary_vault,
                creator_total,
                &<b>mut</b> payment,
                min_vault_deposit_amount,
                clock,
                ctx
            );
        };
        <b>let</b> treasury_total = treasury_fee + platform_fee_to_treasury;
        <b>if</b> (treasury_total &gt; 0) {
            <b>let</b> treasury_fee_coin = coin::split(&<b>mut</b> payment, treasury_total, ctx);
            transfer::public_transfer(treasury_fee_coin, <a href="../social_contracts/profile.md#social_contracts_profile_get_treasury_address">profile::get_treasury_address</a>(treasury));
        };
    };
    (payment, fee_amount, creator_fee + platform_fee_to_creator, 0, treasury_fee + platform_fee_to_treasury)
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_distribute_reservation_fees_with_post_and_platform"></a>

## Function `distribute_reservation_fees_with_post_and_platform`

Calculate and distribute all reservation fees (for post reservations with PoC)
Platform version: routes platform fees to platform treasury


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_distribute_reservation_fees_with_post_and_platform">distribute_reservation_fees_with_post_and_platform</a>(config: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">social_contracts::social_proof_tokens::SocialProofTokensConfig</a>, min_vault_deposit_amount: u64, reservation_pool: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ReservationPoolObject">social_contracts::social_proof_tokens::ReservationPoolObject</a>, <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, beneficiary_vault: &<b>mut</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_PoCBeneficiaryVault">social_contracts::poc_vault::PoCBeneficiaryVault</a>, amount: u64, payment: <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): (<a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, u64, u64, u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_distribute_reservation_fees_with_post_and_platform">distribute_reservation_fees_with_post_and_platform</a>(
    config: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">SocialProofTokensConfig</a>,
    min_vault_deposit_amount: u64,
    reservation_pool: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ReservationPoolObject">ReservationPoolObject</a>,
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &Post,
    beneficiary_vault: &<b>mut</b> PoCBeneficiaryVault,
    amount: u64,
    <b>mut</b> payment: Coin&lt;MYSO&gt;,
    treasury: &EcosystemTreasury,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
): (Coin&lt;MYSO&gt;, u64, u64, u64, u64) {
    // Validate fees and calculate with overflow protection
    <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_validate_reservation_fees">validate_reservation_fees</a>(config);
    <b>let</b> reservation_total_fee_bps = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_reservation_total_fee_bps">calculate_reservation_total_fee_bps</a>(config);
    <b>let</b> fee_amount = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_fee_amount_safe">calculate_fee_amount_safe</a>(amount, reservation_total_fee_bps);
    <b>let</b> creator_fee = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_component_fee_safe">calculate_component_fee_safe</a>(fee_amount, config.reservation_creator_fee_bps, reservation_total_fee_bps);
    <b>let</b> platform_fee = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_component_fee_safe">calculate_component_fee_safe</a>(fee_amount, config.reservation_platform_fee_bps, reservation_total_fee_bps);
    <b>let</b> treasury_fee = fee_amount - creator_fee - platform_fee;
    // Distribute fees (same pattern <b>as</b> trading fees)
    <b>if</b> (fee_amount &gt; 0) {
        // Send creator fee with PoC redirection support
        <b>if</b> (creator_fee &gt; 0) {
            <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_distribute_reservation_creator_fee">distribute_reservation_creator_fee</a>(
                reservation_pool,
                <a href="../social_contracts/post.md#social_contracts_post">post</a>,
                beneficiary_vault,
                creator_fee,
                &<b>mut</b> payment,
                min_vault_deposit_amount,
                clock,
                ctx
            );
        };
        // Send <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> fee to <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> treasury
        <b>if</b> (platform_fee &gt; 0) {
            <b>let</b> <b>mut</b> platform_fee_coin = coin::split(&<b>mut</b> payment, platform_fee, ctx);
            <a href="../social_contracts/platform.md#social_contracts_platform_add_to_treasury">social_contracts::platform::add_to_treasury</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, &<b>mut</b> platform_fee_coin, platform_fee, clock, ctx);
            coin::destroy_zero(platform_fee_coin);
        };
        // Send treasury fee
        <b>if</b> (treasury_fee &gt; 0) {
            <b>let</b> treasury_fee_coin = coin::split(&<b>mut</b> payment, treasury_fee, ctx);
            transfer::public_transfer(treasury_fee_coin, <a href="../social_contracts/profile.md#social_contracts_profile_get_treasury_address">profile::get_treasury_address</a>(treasury));
        };
    };
    // Return remaining payment and fee amounts
    (payment, fee_amount, creator_fee, platform_fee, treasury_fee)
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_distribute_reservation_fees_no_poc"></a>

## Function `distribute_reservation_fees_no_poc`

Calculate and distribute all reservation fees (for profile reservations without PoC)
Non-platform version: split platform fee between creator and treasury per config; emit platform_fee 0


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_distribute_reservation_fees_no_poc">distribute_reservation_fees_no_poc</a>(config: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">social_contracts::social_proof_tokens::SocialProofTokensConfig</a>, reservation_pool: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ReservationPoolObject">social_contracts::social_proof_tokens::ReservationPoolObject</a>, amount: u64, payment: <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): (<a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, u64, u64, u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_distribute_reservation_fees_no_poc">distribute_reservation_fees_no_poc</a>(
    config: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">SocialProofTokensConfig</a>,
    reservation_pool: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ReservationPoolObject">ReservationPoolObject</a>,
    amount: u64,
    <b>mut</b> payment: Coin&lt;MYSO&gt;,
    treasury: &EcosystemTreasury,
    ctx: &<b>mut</b> TxContext
): (Coin&lt;MYSO&gt;, u64, u64, u64, u64) {
    // Validate fees and calculate with overflow protection
    <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_validate_reservation_fees">validate_reservation_fees</a>(config);
    <b>let</b> reservation_total_fee_bps = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_reservation_total_fee_bps">calculate_reservation_total_fee_bps</a>(config);
    <b>let</b> fee_amount = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_fee_amount_safe">calculate_fee_amount_safe</a>(amount, reservation_total_fee_bps);
    <b>let</b> creator_fee = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_component_fee_safe">calculate_component_fee_safe</a>(fee_amount, config.reservation_creator_fee_bps, reservation_total_fee_bps);
    <b>let</b> platform_fee = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_component_fee_safe">calculate_component_fee_safe</a>(fee_amount, config.reservation_platform_fee_bps, reservation_total_fee_bps);
    <b>let</b> treasury_fee = fee_amount - creator_fee - platform_fee;
    <b>let</b> platform_fee_to_creator = (platform_fee * config.non_platform_platform_to_creator_bps) / <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_BPS_DENOM">BPS_DENOM</a>;
    <b>let</b> platform_fee_to_treasury = platform_fee - platform_fee_to_creator;
    <b>if</b> (fee_amount &gt; 0) {
        <b>let</b> creator_total = creator_fee + platform_fee_to_creator;
        <b>if</b> (creator_total &gt; 0) {
            <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_distribute_reservation_creator_fee_no_poc">distribute_reservation_creator_fee_no_poc</a>(reservation_pool, creator_total, &<b>mut</b> payment, ctx);
        };
        <b>let</b> treasury_total = treasury_fee + platform_fee_to_treasury;
        <b>if</b> (treasury_total &gt; 0) {
            <b>let</b> treasury_fee_coin = coin::split(&<b>mut</b> payment, treasury_total, ctx);
            transfer::public_transfer(treasury_fee_coin, <a href="../social_contracts/profile.md#social_contracts_profile_get_treasury_address">profile::get_treasury_address</a>(treasury));
        };
    };
    (payment, fee_amount, creator_fee + platform_fee_to_creator, 0, treasury_fee + platform_fee_to_treasury)
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_distribute_reservation_fees_no_poc_with_platform"></a>

## Function `distribute_reservation_fees_no_poc_with_platform`

Calculate and distribute all reservation fees (for profile reservations without PoC)
Platform version: routes platform fees to platform treasury


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_distribute_reservation_fees_no_poc_with_platform">distribute_reservation_fees_no_poc_with_platform</a>(config: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">social_contracts::social_proof_tokens::SocialProofTokensConfig</a>, reservation_pool: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ReservationPoolObject">social_contracts::social_proof_tokens::ReservationPoolObject</a>, amount: u64, payment: <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): (<a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, u64, u64, u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_distribute_reservation_fees_no_poc_with_platform">distribute_reservation_fees_no_poc_with_platform</a>(
    config: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">SocialProofTokensConfig</a>,
    reservation_pool: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ReservationPoolObject">ReservationPoolObject</a>,
    amount: u64,
    <b>mut</b> payment: Coin&lt;MYSO&gt;,
    treasury: &EcosystemTreasury,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
): (Coin&lt;MYSO&gt;, u64, u64, u64, u64) {
    // Validate fees and calculate with overflow protection
    <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_validate_reservation_fees">validate_reservation_fees</a>(config);
    <b>let</b> reservation_total_fee_bps = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_reservation_total_fee_bps">calculate_reservation_total_fee_bps</a>(config);
    <b>let</b> fee_amount = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_fee_amount_safe">calculate_fee_amount_safe</a>(amount, reservation_total_fee_bps);
    <b>let</b> creator_fee = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_component_fee_safe">calculate_component_fee_safe</a>(fee_amount, config.reservation_creator_fee_bps, reservation_total_fee_bps);
    <b>let</b> platform_fee = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_component_fee_safe">calculate_component_fee_safe</a>(fee_amount, config.reservation_platform_fee_bps, reservation_total_fee_bps);
    <b>let</b> treasury_fee = fee_amount - creator_fee - platform_fee;
    // Distribute fees (same pattern <b>as</b> trading fees)
    <b>if</b> (fee_amount &gt; 0) {
        // Send creator fee without PoC redirection
        <b>if</b> (creator_fee &gt; 0) {
            <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_distribute_reservation_creator_fee_no_poc">distribute_reservation_creator_fee_no_poc</a>(reservation_pool, creator_fee, &<b>mut</b> payment, ctx);
        };
        // Send <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> fee to <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> treasury
        <b>if</b> (platform_fee &gt; 0) {
            <b>let</b> <b>mut</b> platform_fee_coin = coin::split(&<b>mut</b> payment, platform_fee, ctx);
            <a href="../social_contracts/platform.md#social_contracts_platform_add_to_treasury">social_contracts::platform::add_to_treasury</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, &<b>mut</b> platform_fee_coin, platform_fee, clock, ctx);
            coin::destroy_zero(platform_fee_coin);
        };
        // Send treasury fee
        <b>if</b> (treasury_fee &gt; 0) {
            <b>let</b> treasury_fee_coin = coin::split(&<b>mut</b> payment, treasury_fee, ctx);
            transfer::public_transfer(treasury_fee_coin, <a href="../social_contracts/profile.md#social_contracts_profile_get_treasury_address">profile::get_treasury_address</a>(treasury));
        };
    };
    // Return remaining payment and fee amounts
    (payment, fee_amount, creator_fee, platform_fee, treasury_fee)
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_buy_tokens"></a>

## Function `buy_tokens`

Buy tokens from the pool - first purchase
Non-platform version: platform fees go to ecosystem treasury
This function handles buying tokens for first-time buyers of a specific token


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_buy_tokens">buy_tokens</a>(_registry: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">social_contracts::social_proof_tokens::TokenRegistry</a>, pool: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">social_contracts::social_proof_tokens::TokenPool</a>, config: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">social_contracts::social_proof_tokens::SocialProofTokensConfig</a>, treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, profile_registry: &<a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">social_contracts::profile::UsernameRegistry</a>, block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, payment: <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, amount: u64, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_buy_tokens">buy_tokens</a>(
    _registry: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">TokenRegistry</a>,
    pool: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">TokenPool</a>,
    config: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">SocialProofTokensConfig</a>,
    treasury: &EcosystemTreasury,
    profile_registry: &UsernameRegistry,
    block_list_registry: &BlockListRegistry,
    <b>mut</b> payment: Coin&lt;MYSO&gt;,
    amount: u64,
    ctx: &<b>mut</b> TxContext
) {
    // Check version compatibility
    <b>assert</b>!(pool.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EWrongVersion">EWrongVersion</a>);
    // Check <b>if</b> trading is halted
    <b>assert</b>!(config.trading_enabled, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ETradingHalted">ETradingHalted</a>);
    <b>let</b> buyer = tx_context::sender(ctx);
    // Look up the buyer's <a href="../social_contracts/profile.md#social_contracts_profile">profile</a> ID
    <b>let</b> profile_id_option = <a href="../social_contracts/profile.md#social_contracts_profile_lookup_profile_by_owner">profile::lookup_profile_by_owner</a>(profile_registry, buyer);
    <b>assert</b>!(option::is_some(&profile_id_option), <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ENotAuthorized">ENotAuthorized</a>);
    // Check <b>if</b> token owner is blocked by the buyer
    <b>assert</b>!(!<a href="../social_contracts/block_list.md#social_contracts_block_list_is_blocked">block_list::is_blocked</a>(block_list_registry, buyer, pool.info.owner), <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EBlockedUser">EBlockedUser</a>);
    // Calculate the price <b>for</b> the tokens based on quadratic curve
    <b>let</b> (price, _) = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_buy_price">calculate_buy_price</a>(
        pool.info.base_price,
        pool.info.quadratic_coefficient,
        pool.info.circulating_supply,
        amount
    );
    // Ensure buyer <b>has</b> enough funds
    <b>assert</b>!(coin::value(&payment) &gt;= price, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInsufficientFunds">EInsufficientFunds</a>);
    // Validate fees and calculate with overflow protection
    <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_validate_trading_fees">validate_trading_fees</a>(config);
    <b>let</b> total_fee_bps = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_total_fee_bps">calculate_total_fee_bps</a>(config);
    <b>let</b> fee_amount = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_fee_amount_safe">calculate_fee_amount_safe</a>(price, total_fee_bps);
    <b>let</b> creator_fee = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_component_fee_safe">calculate_component_fee_safe</a>(fee_amount, config.trading_creator_fee_bps, total_fee_bps);
    <b>let</b> platform_fee = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_component_fee_safe">calculate_component_fee_safe</a>(fee_amount, config.trading_platform_fee_bps, total_fee_bps);
    <b>let</b> treasury_fee = fee_amount - creator_fee - platform_fee;
    // Calculate the net amount to the liquidity pool
    <b>let</b> net_amount = price - fee_amount;
    // Extract payment and distribute fees with PoC redirection support
    <b>if</b> (fee_amount &gt; 0) {
        // Send creator fee with PoC redirection support
        <b>if</b> (creator_fee &gt; 0) {
            <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_distribute_creator_fee">distribute_creator_fee</a>(pool, creator_fee, &<b>mut</b> payment, ctx);
        };
        // Send <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> fee to ecosystem treasury (no <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> involved)
        <b>if</b> (platform_fee &gt; 0) {
            <b>let</b> platform_fee_coin = coin::split(&<b>mut</b> payment, platform_fee, ctx);
            transfer::public_transfer(platform_fee_coin, <a href="../social_contracts/profile.md#social_contracts_profile_get_treasury_address">profile::get_treasury_address</a>(treasury));
        };
        // Send treasury fee
        <b>if</b> (treasury_fee &gt; 0) {
            <b>let</b> treasury_fee_coin = coin::split(&<b>mut</b> payment, treasury_fee, ctx);
            transfer::public_transfer(treasury_fee_coin, <a href="../social_contracts/profile.md#social_contracts_profile_get_treasury_address">profile::get_treasury_address</a>(treasury));
        };
    };
    // Add remaining payment to pool
    <b>let</b> pool_payment = coin::split(&<b>mut</b> payment, net_amount, ctx);
    balance::join(&<b>mut</b> pool.myso_balance, coin::into_balance(pool_payment));
    // Refund any excess payment
    <b>if</b> (coin::value(&payment) &gt; 0) {
        transfer::public_transfer(payment, buyer);
    } <b>else</b> {
        coin::destroy_zero(payment);
    };
    // Update holder's balance with overflow protection
    // First check addition overflow
    <b>assert</b>!(pool.info.circulating_supply &lt;= <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_MAX_U64">MAX_U64</a> - amount, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EOverflow">EOverflow</a>);
    <b>let</b> new_supply = pool.info.circulating_supply + amount;
    // Then check multiplication overflow <b>for</b> max_hold calculation
    <b>assert</b>!(new_supply == 0 || new_supply &lt;= <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_MAX_U64">MAX_U64</a> / config.max_hold_percent_bps, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EOverflow">EOverflow</a>);
    <b>let</b> max_hold = (new_supply * config.max_hold_percent_bps) / <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_BPS_DENOM">BPS_DENOM</a>;
    <b>let</b> current_hold = <b>if</b> (table::contains(&pool.holders, buyer)) {
        *table::borrow(&pool.holders, buyer)
    } <b>else</b> {
        0
    };
    // Check max holding limit with overflow protection
    <b>assert</b>!(current_hold &lt;= <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_MAX_U64">MAX_U64</a> - amount, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EOverflow">EOverflow</a>);
    <b>assert</b>!(current_hold + amount &lt;= max_hold, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EExceededMaxHold">EExceededMaxHold</a>);
    // Check that this is the first purchase (user must not already own tokens)
    <b>assert</b>!(current_hold == 0, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EAlreadyOwnsTokens">EAlreadyOwnsTokens</a>);
    // Update holder's balance
    table::add(&<b>mut</b> pool.holders, buyer, amount);
    // Update circulating supply
    <b>assert</b>!(pool.info.circulating_supply &lt;= <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_MAX_U64">MAX_U64</a> - amount, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EOverflow">EOverflow</a>);
    pool.info.circulating_supply = pool.info.circulating_supply + amount;
    // Mint new social token <b>for</b> the user
    <b>let</b> social_token = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialToken">SocialToken</a> {
        id: object::new(ctx),
        pool_id: object::uid_to_address(&pool.id),
        token_type: pool.info.token_type,
        amount,
    };
    transfer::transfer(social_token, buyer);
    // Calculate the new price after purchase
    <b>let</b> new_price = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_token_price">calculate_token_price</a>(
        pool.info.base_price,
        pool.info.quadratic_coefficient,
        pool.info.circulating_supply
    );
    // Emit buy event
    event::emit(<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenBoughtEvent">TokenBoughtEvent</a> {
        id: object::uid_to_address(&pool.id),
        buyer,
        amount,
        myso_amount: price,
        fee_amount,
        creator_fee,
        platform_fee,
        treasury_fee,
        new_price,
    });
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_buy_tokens_with_platform"></a>

## Function `buy_tokens_with_platform`

Buy tokens from the pool - first purchase
Platform version: platform fees go to platform treasury, includes platform validation
This function handles buying tokens for first-time buyers of a specific token


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_buy_tokens_with_platform">buy_tokens_with_platform</a>(_registry: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">social_contracts::social_proof_tokens::TokenRegistry</a>, pool: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">social_contracts::social_proof_tokens::TokenPool</a>, config: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">social_contracts::social_proof_tokens::SocialProofTokensConfig</a>, treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, platform_registry: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">social_contracts::platform::PlatformRegistry</a>, profile_registry: &<a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">social_contracts::profile::UsernameRegistry</a>, block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, payment: <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, amount: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_buy_tokens_with_platform">buy_tokens_with_platform</a>(
    _registry: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">TokenRegistry</a>,
    pool: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">TokenPool</a>,
    config: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">SocialProofTokensConfig</a>,
    treasury: &EcosystemTreasury,
    platform_registry: &PlatformRegistry,
    profile_registry: &UsernameRegistry,
    block_list_registry: &BlockListRegistry,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>,
    <b>mut</b> payment: Coin&lt;MYSO&gt;,
    amount: u64,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    // Check version compatibility
    <b>assert</b>!(pool.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EWrongVersion">EWrongVersion</a>);
    // Check <b>if</b> trading is halted
    <b>assert</b>!(config.trading_enabled, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ETradingHalted">ETradingHalted</a>);
    <b>let</b> buyer = tx_context::sender(ctx);
    // Look up the buyer's <a href="../social_contracts/profile.md#social_contracts_profile">profile</a> ID
    <b>let</b> profile_id_option = <a href="../social_contracts/profile.md#social_contracts_profile_lookup_profile_by_owner">profile::lookup_profile_by_owner</a>(profile_registry, buyer);
    <b>assert</b>!(option::is_some(&profile_id_option), <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ENotAuthorized">ENotAuthorized</a>);
    // Platform validation
    <b>let</b> platform_id = object::uid_to_address(<a href="../social_contracts/platform.md#social_contracts_platform_id">platform::id</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>));
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform_is_approved">platform::is_approved</a>(platform_registry, platform_id), <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ENotAuthorized">ENotAuthorized</a>);
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform_has_joined_platform">platform::has_joined_platform</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, buyer), <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EUserNotJoinedPlatform">EUserNotJoinedPlatform</a>);
    <b>assert</b>!(!<a href="../social_contracts/block_list.md#social_contracts_block_list_is_blocked">block_list::is_blocked</a>(block_list_registry, platform_id, buyer), <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EUserBlockedByPlatform">EUserBlockedByPlatform</a>);
    // Check <b>if</b> token owner is blocked by the buyer
    <b>assert</b>!(!<a href="../social_contracts/block_list.md#social_contracts_block_list_is_blocked">block_list::is_blocked</a>(block_list_registry, buyer, pool.info.owner), <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EBlockedUser">EBlockedUser</a>);
    // Calculate the price <b>for</b> the tokens based on quadratic curve
    <b>let</b> (price, _) = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_buy_price">calculate_buy_price</a>(
        pool.info.base_price,
        pool.info.quadratic_coefficient,
        pool.info.circulating_supply,
        amount
    );
    // Ensure buyer <b>has</b> enough funds
    <b>assert</b>!(coin::value(&payment) &gt;= price, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInsufficientFunds">EInsufficientFunds</a>);
    // Validate fees and calculate with overflow protection
    <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_validate_trading_fees">validate_trading_fees</a>(config);
    <b>let</b> total_fee_bps = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_total_fee_bps">calculate_total_fee_bps</a>(config);
    <b>let</b> fee_amount = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_fee_amount_safe">calculate_fee_amount_safe</a>(price, total_fee_bps);
    <b>let</b> creator_fee = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_component_fee_safe">calculate_component_fee_safe</a>(fee_amount, config.trading_creator_fee_bps, total_fee_bps);
    <b>let</b> platform_fee = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_component_fee_safe">calculate_component_fee_safe</a>(fee_amount, config.trading_platform_fee_bps, total_fee_bps);
    <b>let</b> treasury_fee = fee_amount - creator_fee - platform_fee;
    // Calculate the net amount to the liquidity pool
    <b>let</b> net_amount = price - fee_amount;
    // Extract payment and distribute fees with PoC redirection support
    <b>if</b> (fee_amount &gt; 0) {
        // Send creator fee with PoC redirection support
        <b>if</b> (creator_fee &gt; 0) {
            <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_distribute_creator_fee">distribute_creator_fee</a>(pool, creator_fee, &<b>mut</b> payment, ctx);
        };
        // Send <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> fee to <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> treasury
        <b>if</b> (platform_fee &gt; 0) {
            <b>let</b> <b>mut</b> platform_fee_coin = coin::split(&<b>mut</b> payment, platform_fee, ctx);
            <a href="../social_contracts/platform.md#social_contracts_platform_add_to_treasury">social_contracts::platform::add_to_treasury</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, &<b>mut</b> platform_fee_coin, platform_fee, clock, ctx);
            coin::destroy_zero(platform_fee_coin);
        };
        // Send treasury fee
        <b>if</b> (treasury_fee &gt; 0) {
            <b>let</b> treasury_fee_coin = coin::split(&<b>mut</b> payment, treasury_fee, ctx);
            transfer::public_transfer(treasury_fee_coin, <a href="../social_contracts/profile.md#social_contracts_profile_get_treasury_address">profile::get_treasury_address</a>(treasury));
        };
    };
    // Add remaining payment to pool
    <b>let</b> pool_payment = coin::split(&<b>mut</b> payment, net_amount, ctx);
    balance::join(&<b>mut</b> pool.myso_balance, coin::into_balance(pool_payment));
    // Refund any excess payment
    <b>if</b> (coin::value(&payment) &gt; 0) {
        transfer::public_transfer(payment, buyer);
    } <b>else</b> {
        coin::destroy_zero(payment);
    };
    // Update holder's balance with overflow protection
    // First check addition overflow
    <b>assert</b>!(pool.info.circulating_supply &lt;= <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_MAX_U64">MAX_U64</a> - amount, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EOverflow">EOverflow</a>);
    <b>let</b> new_supply = pool.info.circulating_supply + amount;
    // Then check multiplication overflow <b>for</b> max_hold calculation
    <b>assert</b>!(new_supply == 0 || new_supply &lt;= <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_MAX_U64">MAX_U64</a> / config.max_hold_percent_bps, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EOverflow">EOverflow</a>);
    <b>let</b> max_hold = (new_supply * config.max_hold_percent_bps) / <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_BPS_DENOM">BPS_DENOM</a>;
    <b>let</b> current_hold = <b>if</b> (table::contains(&pool.holders, buyer)) {
        *table::borrow(&pool.holders, buyer)
    } <b>else</b> {
        0
    };
    // Check max holding limit with overflow protection
    <b>assert</b>!(current_hold &lt;= <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_MAX_U64">MAX_U64</a> - amount, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EOverflow">EOverflow</a>);
    <b>assert</b>!(current_hold + amount &lt;= max_hold, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EExceededMaxHold">EExceededMaxHold</a>);
    // Check that this is the first purchase (user must not already own tokens)
    <b>assert</b>!(current_hold == 0, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EAlreadyOwnsTokens">EAlreadyOwnsTokens</a>);
    // Update holder's balance
    table::add(&<b>mut</b> pool.holders, buyer, amount);
    // Update circulating supply
    <b>assert</b>!(pool.info.circulating_supply &lt;= <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_MAX_U64">MAX_U64</a> - amount, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EOverflow">EOverflow</a>);
    pool.info.circulating_supply = pool.info.circulating_supply + amount;
    // Mint new social token <b>for</b> the user
    <b>let</b> social_token = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialToken">SocialToken</a> {
        id: object::new(ctx),
        pool_id: object::uid_to_address(&pool.id),
        token_type: pool.info.token_type,
        amount,
    };
    transfer::transfer(social_token, buyer);
    // Calculate the new price after purchase
    <b>let</b> new_price = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_token_price">calculate_token_price</a>(
        pool.info.base_price,
        pool.info.quadratic_coefficient,
        pool.info.circulating_supply
    );
    // Emit buy event
    event::emit(<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenBoughtEvent">TokenBoughtEvent</a> {
        id: object::uid_to_address(&pool.id),
        buyer,
        amount,
        myso_amount: price,
        fee_amount,
        creator_fee,
        platform_fee,
        treasury_fee,
        new_price,
    });
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_buy_more_tokens"></a>

## Function `buy_more_tokens`

Buy more tokens when you already have a social token
Non-platform version: platform fees go to ecosystem treasury
This function allows users to add to their existing token holdings using MYSO Coin


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_buy_more_tokens">buy_more_tokens</a>(_registry: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">social_contracts::social_proof_tokens::TokenRegistry</a>, pool: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">social_contracts::social_proof_tokens::TokenPool</a>, config: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">social_contracts::social_proof_tokens::SocialProofTokensConfig</a>, treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, profile_registry: &<a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">social_contracts::profile::UsernameRegistry</a>, block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, payment: <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, amount: u64, social_token: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialToken">social_contracts::social_proof_tokens::SocialToken</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_buy_more_tokens">buy_more_tokens</a>(
    _registry: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">TokenRegistry</a>,
    pool: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">TokenPool</a>,
    config: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">SocialProofTokensConfig</a>,
    treasury: &EcosystemTreasury,
    profile_registry: &UsernameRegistry,
    block_list_registry: &BlockListRegistry,
    <b>mut</b> payment: Coin&lt;MYSO&gt;,
    amount: u64,
    social_token: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialToken">SocialToken</a>,
    ctx: &<b>mut</b> TxContext
) {
    // Check version compatibility
    <b>assert</b>!(pool.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EWrongVersion">EWrongVersion</a>);
    // Check <b>if</b> trading is halted
    <b>assert</b>!(config.trading_enabled, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ETradingHalted">ETradingHalted</a>);
    <b>let</b> buyer = tx_context::sender(ctx);
    // Look up the buyer's <a href="../social_contracts/profile.md#social_contracts_profile">profile</a> ID
    <b>let</b> profile_id_option = <a href="../social_contracts/profile.md#social_contracts_profile_lookup_profile_by_owner">profile::lookup_profile_by_owner</a>(profile_registry, buyer);
    <b>assert</b>!(option::is_some(&profile_id_option), <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ENotAuthorized">ENotAuthorized</a>);
    // Check <b>if</b> token owner is blocked by the buyer
    <b>assert</b>!(!<a href="../social_contracts/block_list.md#social_contracts_block_list_is_blocked">block_list::is_blocked</a>(block_list_registry, buyer, pool.info.owner), <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EBlockedUser">EBlockedUser</a>);
    // Verify social token matches the pool and is an active position
    <b>assert</b>!(social_token.pool_id == object::uid_to_address(&pool.id), <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInvalidID">EInvalidID</a>);
    <b>assert</b>!(social_token.amount &gt; 0, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ENoTokensOwned">ENoTokensOwned</a>);
    // Calculate the price <b>for</b> the tokens based on quadratic curve
    <b>let</b> (price, _) = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_buy_price">calculate_buy_price</a>(
        pool.info.base_price,
        pool.info.quadratic_coefficient,
        pool.info.circulating_supply,
        amount
    );
    // Ensure buyer <b>has</b> enough funds
    <b>assert</b>!(coin::value(&payment) &gt;= price, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInsufficientFunds">EInsufficientFunds</a>);
    // Validate fees and calculate with overflow protection
    <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_validate_trading_fees">validate_trading_fees</a>(config);
    <b>let</b> total_fee_bps = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_total_fee_bps">calculate_total_fee_bps</a>(config);
    <b>let</b> fee_amount = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_fee_amount_safe">calculate_fee_amount_safe</a>(price, total_fee_bps);
    <b>let</b> creator_fee = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_component_fee_safe">calculate_component_fee_safe</a>(fee_amount, config.trading_creator_fee_bps, total_fee_bps);
    <b>let</b> platform_fee = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_component_fee_safe">calculate_component_fee_safe</a>(fee_amount, config.trading_platform_fee_bps, total_fee_bps);
    <b>let</b> treasury_fee = fee_amount - creator_fee - platform_fee;
    // Calculate the net amount to the liquidity pool
    <b>let</b> net_amount = price - fee_amount;
    // Extract payment and distribute fees with PoC redirection support
    <b>if</b> (fee_amount &gt; 0) {
        // Send creator fee with PoC redirection support
        <b>if</b> (creator_fee &gt; 0) {
            <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_distribute_creator_fee">distribute_creator_fee</a>(pool, creator_fee, &<b>mut</b> payment, ctx);
        };
        // Send <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> fee to ecosystem treasury (no <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> involved)
        <b>if</b> (platform_fee &gt; 0) {
            <b>let</b> platform_fee_coin = coin::split(&<b>mut</b> payment, platform_fee, ctx);
            transfer::public_transfer(platform_fee_coin, <a href="../social_contracts/profile.md#social_contracts_profile_get_treasury_address">profile::get_treasury_address</a>(treasury));
        };
        // Send treasury fee
        <b>if</b> (treasury_fee &gt; 0) {
            <b>let</b> treasury_fee_coin = coin::split(&<b>mut</b> payment, treasury_fee, ctx);
            transfer::public_transfer(treasury_fee_coin, <a href="../social_contracts/profile.md#social_contracts_profile_get_treasury_address">profile::get_treasury_address</a>(treasury));
        };
    };
    // Add remaining payment to pool
    <b>let</b> pool_payment = coin::split(&<b>mut</b> payment, net_amount, ctx);
    balance::join(&<b>mut</b> pool.myso_balance, coin::into_balance(pool_payment));
    // Refund any excess payment
    <b>if</b> (coin::value(&payment) &gt; 0) {
        transfer::public_transfer(payment, buyer);
    } <b>else</b> {
        coin::destroy_zero(payment);
    };
    // Update holder's balance with overflow protection
    // First check addition overflow
    <b>assert</b>!(pool.info.circulating_supply &lt;= <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_MAX_U64">MAX_U64</a> - amount, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EOverflow">EOverflow</a>);
    <b>let</b> new_supply = pool.info.circulating_supply + amount;
    // Then check multiplication overflow <b>for</b> max_hold calculation
    <b>assert</b>!(new_supply == 0 || new_supply &lt;= <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_MAX_U64">MAX_U64</a> / config.max_hold_percent_bps, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EOverflow">EOverflow</a>);
    <b>let</b> max_hold = (new_supply * config.max_hold_percent_bps) / <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_BPS_DENOM">BPS_DENOM</a>;
    <b>let</b> current_hold = <b>if</b> (table::contains(&pool.holders, buyer)) {
        *table::borrow(&pool.holders, buyer)
    } <b>else</b> {
        0
    };
    // Check max holding limit with overflow protection
    <b>assert</b>!(current_hold &lt;= <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_MAX_U64">MAX_U64</a> - amount, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EOverflow">EOverflow</a>);
    <b>assert</b>!(current_hold + amount &lt;= max_hold, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EExceededMaxHold">EExceededMaxHold</a>);
    // Update holder's balance
    <b>if</b> (table::contains(&pool.holders, buyer)) {
        <b>let</b> holder_balance = table::borrow_mut(&<b>mut</b> pool.holders, buyer);
        <b>assert</b>!(*holder_balance &lt;= <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_MAX_U64">MAX_U64</a> - amount, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EOverflow">EOverflow</a>);
        *holder_balance = *holder_balance + amount;
    } <b>else</b> {
        table::add(&<b>mut</b> pool.holders, buyer, amount);
    };
    // Update circulating supply
    <b>assert</b>!(pool.info.circulating_supply &lt;= <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_MAX_U64">MAX_U64</a> - amount, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EOverflow">EOverflow</a>);
    pool.info.circulating_supply = pool.info.circulating_supply + amount;
    // Update the user's social token
    <b>assert</b>!(social_token.amount &lt;= <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_MAX_U64">MAX_U64</a> - amount, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EOverflow">EOverflow</a>);
    social_token.amount = social_token.amount + amount;
    // Calculate the new price after purchase
    <b>let</b> new_price = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_token_price">calculate_token_price</a>(
        pool.info.base_price,
        pool.info.quadratic_coefficient,
        pool.info.circulating_supply
    );
    // Emit buy event
    event::emit(<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenBoughtEvent">TokenBoughtEvent</a> {
        id: object::uid_to_address(&pool.id),
        buyer,
        amount,
        myso_amount: price,
        fee_amount,
        creator_fee,
        platform_fee,
        treasury_fee,
        new_price,
    });
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_buy_more_tokens_with_platform"></a>

## Function `buy_more_tokens_with_platform`

Buy more tokens when you already have a social token
Platform version: platform fees go to platform treasury, includes platform validation
This function allows users to add to their existing token holdings using MYSO Coin


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_buy_more_tokens_with_platform">buy_more_tokens_with_platform</a>(_registry: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">social_contracts::social_proof_tokens::TokenRegistry</a>, pool: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">social_contracts::social_proof_tokens::TokenPool</a>, config: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">social_contracts::social_proof_tokens::SocialProofTokensConfig</a>, treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, platform_registry: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">social_contracts::platform::PlatformRegistry</a>, profile_registry: &<a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">social_contracts::profile::UsernameRegistry</a>, block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, payment: <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, amount: u64, social_token: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialToken">social_contracts::social_proof_tokens::SocialToken</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_buy_more_tokens_with_platform">buy_more_tokens_with_platform</a>(
    _registry: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">TokenRegistry</a>,
    pool: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">TokenPool</a>,
    config: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">SocialProofTokensConfig</a>,
    treasury: &EcosystemTreasury,
    platform_registry: &PlatformRegistry,
    profile_registry: &UsernameRegistry,
    block_list_registry: &BlockListRegistry,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>,
    <b>mut</b> payment: Coin&lt;MYSO&gt;,
    amount: u64,
    social_token: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialToken">SocialToken</a>,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    // Check version compatibility
    <b>assert</b>!(pool.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EWrongVersion">EWrongVersion</a>);
    // Check <b>if</b> trading is halted
    <b>assert</b>!(config.trading_enabled, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ETradingHalted">ETradingHalted</a>);
    <b>let</b> buyer = tx_context::sender(ctx);
    // Look up the buyer's <a href="../social_contracts/profile.md#social_contracts_profile">profile</a> ID
    <b>let</b> profile_id_option = <a href="../social_contracts/profile.md#social_contracts_profile_lookup_profile_by_owner">profile::lookup_profile_by_owner</a>(profile_registry, buyer);
    <b>assert</b>!(option::is_some(&profile_id_option), <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ENotAuthorized">ENotAuthorized</a>);
    // Platform validation
    <b>let</b> platform_id = object::uid_to_address(<a href="../social_contracts/platform.md#social_contracts_platform_id">platform::id</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>));
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform_is_approved">platform::is_approved</a>(platform_registry, platform_id), <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ENotAuthorized">ENotAuthorized</a>);
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform_has_joined_platform">platform::has_joined_platform</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, buyer), <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EUserNotJoinedPlatform">EUserNotJoinedPlatform</a>);
    <b>assert</b>!(!<a href="../social_contracts/block_list.md#social_contracts_block_list_is_blocked">block_list::is_blocked</a>(block_list_registry, platform_id, buyer), <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EUserBlockedByPlatform">EUserBlockedByPlatform</a>);
    // Check <b>if</b> token owner is blocked by the buyer
    <b>assert</b>!(!<a href="../social_contracts/block_list.md#social_contracts_block_list_is_blocked">block_list::is_blocked</a>(block_list_registry, buyer, pool.info.owner), <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EBlockedUser">EBlockedUser</a>);
    // Verify social token matches the pool and is an active position
    <b>assert</b>!(social_token.pool_id == object::uid_to_address(&pool.id), <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInvalidID">EInvalidID</a>);
    <b>assert</b>!(social_token.amount &gt; 0, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ENoTokensOwned">ENoTokensOwned</a>);
    // Calculate the price <b>for</b> the tokens based on quadratic curve
    <b>let</b> (price, _) = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_buy_price">calculate_buy_price</a>(
        pool.info.base_price,
        pool.info.quadratic_coefficient,
        pool.info.circulating_supply,
        amount
    );
    // Ensure buyer <b>has</b> enough funds
    <b>assert</b>!(coin::value(&payment) &gt;= price, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInsufficientFunds">EInsufficientFunds</a>);
    // Validate fees and calculate with overflow protection
    <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_validate_trading_fees">validate_trading_fees</a>(config);
    <b>let</b> total_fee_bps = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_total_fee_bps">calculate_total_fee_bps</a>(config);
    <b>let</b> fee_amount = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_fee_amount_safe">calculate_fee_amount_safe</a>(price, total_fee_bps);
    <b>let</b> creator_fee = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_component_fee_safe">calculate_component_fee_safe</a>(fee_amount, config.trading_creator_fee_bps, total_fee_bps);
    <b>let</b> platform_fee = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_component_fee_safe">calculate_component_fee_safe</a>(fee_amount, config.trading_platform_fee_bps, total_fee_bps);
    <b>let</b> treasury_fee = fee_amount - creator_fee - platform_fee;
    // Calculate the net amount to the liquidity pool
    <b>let</b> net_amount = price - fee_amount;
    // Extract payment and distribute fees with PoC redirection support
    <b>if</b> (fee_amount &gt; 0) {
        // Send creator fee with PoC redirection support
        <b>if</b> (creator_fee &gt; 0) {
            <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_distribute_creator_fee">distribute_creator_fee</a>(pool, creator_fee, &<b>mut</b> payment, ctx);
        };
        // Send <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> fee to <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> treasury
        <b>if</b> (platform_fee &gt; 0) {
            <b>let</b> <b>mut</b> platform_fee_coin = coin::split(&<b>mut</b> payment, platform_fee, ctx);
            <a href="../social_contracts/platform.md#social_contracts_platform_add_to_treasury">social_contracts::platform::add_to_treasury</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, &<b>mut</b> platform_fee_coin, platform_fee, clock, ctx);
            coin::destroy_zero(platform_fee_coin);
        };
        // Send treasury fee
        <b>if</b> (treasury_fee &gt; 0) {
            <b>let</b> treasury_fee_coin = coin::split(&<b>mut</b> payment, treasury_fee, ctx);
            transfer::public_transfer(treasury_fee_coin, <a href="../social_contracts/profile.md#social_contracts_profile_get_treasury_address">profile::get_treasury_address</a>(treasury));
        };
    };
    // Add remaining payment to pool
    <b>let</b> pool_payment = coin::split(&<b>mut</b> payment, net_amount, ctx);
    balance::join(&<b>mut</b> pool.myso_balance, coin::into_balance(pool_payment));
    // Refund any excess payment
    <b>if</b> (coin::value(&payment) &gt; 0) {
        transfer::public_transfer(payment, buyer);
    } <b>else</b> {
        coin::destroy_zero(payment);
    };
    // Update holder's balance with overflow protection
    // First check addition overflow
    <b>assert</b>!(pool.info.circulating_supply &lt;= <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_MAX_U64">MAX_U64</a> - amount, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EOverflow">EOverflow</a>);
    <b>let</b> new_supply = pool.info.circulating_supply + amount;
    // Then check multiplication overflow <b>for</b> max_hold calculation
    <b>assert</b>!(new_supply == 0 || new_supply &lt;= <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_MAX_U64">MAX_U64</a> / config.max_hold_percent_bps, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EOverflow">EOverflow</a>);
    <b>let</b> max_hold = (new_supply * config.max_hold_percent_bps) / <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_BPS_DENOM">BPS_DENOM</a>;
    <b>let</b> current_hold = <b>if</b> (table::contains(&pool.holders, buyer)) {
        *table::borrow(&pool.holders, buyer)
    } <b>else</b> {
        0
    };
    // Check max holding limit with overflow protection
    <b>assert</b>!(current_hold &lt;= <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_MAX_U64">MAX_U64</a> - amount, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EOverflow">EOverflow</a>);
    <b>assert</b>!(current_hold + amount &lt;= max_hold, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EExceededMaxHold">EExceededMaxHold</a>);
    // Update holder's balance
    <b>if</b> (table::contains(&pool.holders, buyer)) {
        <b>let</b> holder_balance = table::borrow_mut(&<b>mut</b> pool.holders, buyer);
        <b>assert</b>!(*holder_balance &lt;= <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_MAX_U64">MAX_U64</a> - amount, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EOverflow">EOverflow</a>);
        *holder_balance = *holder_balance + amount;
    } <b>else</b> {
        table::add(&<b>mut</b> pool.holders, buyer, amount);
    };
    // Update circulating supply
    <b>assert</b>!(pool.info.circulating_supply &lt;= <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_MAX_U64">MAX_U64</a> - amount, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EOverflow">EOverflow</a>);
    pool.info.circulating_supply = pool.info.circulating_supply + amount;
    // Update the user's social token
    <b>assert</b>!(social_token.amount &lt;= <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_MAX_U64">MAX_U64</a> - amount, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EOverflow">EOverflow</a>);
    social_token.amount = social_token.amount + amount;
    // Calculate the new price after purchase
    <b>let</b> new_price = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_token_price">calculate_token_price</a>(
        pool.info.base_price,
        pool.info.quadratic_coefficient,
        pool.info.circulating_supply
    );
    // Emit buy event
    event::emit(<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenBoughtEvent">TokenBoughtEvent</a> {
        id: object::uid_to_address(&pool.id),
        buyer,
        amount,
        myso_amount: price,
        fee_amount,
        creator_fee,
        platform_fee,
        treasury_fee,
        new_price,
    });
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_sell_tokens"></a>

## Function `sell_tokens`

Sell tokens back to the pool
Non-platform version: platform fees go to ecosystem treasury
Consumes the SocialToken by value. On a partial sell a new remainder token is minted and
transferred back to the seller; on a full sell the object is deleted — no zombie tokens.


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_sell_tokens">sell_tokens</a>(_registry: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">social_contracts::social_proof_tokens::TokenRegistry</a>, pool: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">social_contracts::social_proof_tokens::TokenPool</a>, config: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">social_contracts::social_proof_tokens::SocialProofTokensConfig</a>, treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, profile_registry: &<a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">social_contracts::profile::UsernameRegistry</a>, _block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, social_token: <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialToken">social_contracts::social_proof_tokens::SocialToken</a>, amount: u64, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_sell_tokens">sell_tokens</a>(
    _registry: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">TokenRegistry</a>,
    pool: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">TokenPool</a>,
    config: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">SocialProofTokensConfig</a>,
    treasury: &EcosystemTreasury,
    profile_registry: &UsernameRegistry,
    _block_list_registry: &BlockListRegistry,
    social_token: <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialToken">SocialToken</a>,
    amount: u64,
    ctx: &<b>mut</b> TxContext
) {
    // Check version compatibility
    <b>assert</b>!(pool.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EWrongVersion">EWrongVersion</a>);
    // Check <b>if</b> trading is halted
    <b>assert</b>!(config.trading_enabled, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ETradingHalted">ETradingHalted</a>);
    <b>let</b> seller = tx_context::sender(ctx);
    <b>let</b> pool_id = object::uid_to_address(&pool.id);
    // Look up the seller's <a href="../social_contracts/profile.md#social_contracts_profile">profile</a> ID
    <b>let</b> profile_id_option = <a href="../social_contracts/profile.md#social_contracts_profile_lookup_profile_by_owner">profile::lookup_profile_by_owner</a>(profile_registry, seller);
    <b>assert</b>!(option::is_some(&profile_id_option), <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ENotAuthorized">ENotAuthorized</a>);
    // Verify social token matches the pool and <b>has</b> sufficient balance
    <b>assert</b>!(social_token.pool_id == pool_id, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInvalidID">EInvalidID</a>);
    <b>assert</b>!(social_token.amount &gt;= amount, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInsufficientLiquidity">EInsufficientLiquidity</a>);
    // Calculate the sell price based on quadratic curve
    <b>let</b> (refund_amount, _) = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_sell_price">calculate_sell_price</a>(
        pool.info.base_price,
        pool.info.quadratic_coefficient,
        pool.info.circulating_supply,
        amount
    );
    // Validate fees and calculate with overflow protection
    <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_validate_trading_fees">validate_trading_fees</a>(config);
    <b>let</b> total_fee_bps = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_total_fee_bps">calculate_total_fee_bps</a>(config);
    <b>let</b> fee_amount = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_fee_amount_safe">calculate_fee_amount_safe</a>(refund_amount, total_fee_bps);
    <b>let</b> creator_fee = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_component_fee_safe">calculate_component_fee_safe</a>(fee_amount, config.trading_creator_fee_bps, total_fee_bps);
    <b>let</b> platform_fee = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_component_fee_safe">calculate_component_fee_safe</a>(fee_amount, config.trading_platform_fee_bps, total_fee_bps);
    <b>let</b> treasury_fee = fee_amount - creator_fee - platform_fee;
    // Calculate net refund
    <b>let</b> net_refund = refund_amount - fee_amount;
    // Ensure pool <b>has</b> enough liquidity <b>for</b> refund + all fees
    <b>assert</b>!(balance::value(&pool.myso_balance) &gt;= refund_amount, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInsufficientLiquidity">EInsufficientLiquidity</a>);
    // Verify seller <b>has</b> tokens in the pool
    <b>assert</b>!(table::contains(&pool.holders, seller), <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ENoTokensOwned">ENoTokensOwned</a>);
    // Update holder balance
    <b>let</b> holder_balance = table::borrow_mut(&<b>mut</b> pool.holders, seller);
    <b>if</b> (*holder_balance == amount) {
        // Remove holder completely <b>if</b> selling all tokens
        table::remove(&<b>mut</b> pool.holders, seller);
    } <b>else</b> {
        // Reduce balance
        *holder_balance = *holder_balance - amount;
    };
    // Consume the social token: on partial sell mint a remainder token; on full sell delete.
    <b>let</b> remainder = social_token.amount - amount;
    <b>let</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialToken">SocialToken</a> { id, pool_id: token_pool_id, token_type, amount: _ } = social_token;
    object::delete(id);
    <b>if</b> (remainder &gt; 0) {
        <b>let</b> remainder_token = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialToken">SocialToken</a> {
            id: object::new(ctx),
            pool_id: token_pool_id,
            token_type,
            amount: remainder,
        };
        transfer::transfer(remainder_token, seller);
    };
    // Update circulating supply
    pool.info.circulating_supply = pool.info.circulating_supply - amount;
    // Extract net refund from pool
    <b>let</b> refund_balance = balance::split(&<b>mut</b> pool.myso_balance, net_refund);
    // Process and distribute fees with PoC redirection support
    <b>if</b> (fee_amount &gt; 0) {
        // Send fee to creator with PoC redirection support
        <b>if</b> (creator_fee &gt; 0) {
            <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_distribute_creator_fee_from_pool">distribute_creator_fee_from_pool</a>(pool, creator_fee, ctx);
        };
        // Send <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> fee to ecosystem treasury (no <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> involved)
        <b>if</b> (platform_fee &gt; 0) {
            <b>let</b> platform_fee_coin = coin::from_balance(balance::split(&<b>mut</b> pool.myso_balance, platform_fee), ctx);
            transfer::public_transfer(platform_fee_coin, <a href="../social_contracts/profile.md#social_contracts_profile_get_treasury_address">profile::get_treasury_address</a>(treasury));
        };
        // Send fee to treasury
        <b>if</b> (treasury_fee &gt; 0) {
            <b>let</b> treasury_fee_coin = coin::from_balance(balance::split(&<b>mut</b> pool.myso_balance, treasury_fee), ctx);
            transfer::public_transfer(treasury_fee_coin, <a href="../social_contracts/profile.md#social_contracts_profile_get_treasury_address">profile::get_treasury_address</a>(treasury));
        };
    };
    // Transfer refund to seller
    <b>let</b> refund_coin = coin::from_balance(refund_balance, ctx);
    transfer::public_transfer(refund_coin, seller);
    // Calculate the new price after sale
    <b>let</b> new_price = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_token_price">calculate_token_price</a>(
        pool.info.base_price,
        pool.info.quadratic_coefficient,
        pool.info.circulating_supply
    );
    // Emit sell event
    event::emit(<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenSoldEvent">TokenSoldEvent</a> {
        id: pool_id,
        seller,
        amount,
        myso_amount: refund_amount,
        fee_amount,
        creator_fee,
        platform_fee,
        treasury_fee,
        new_price,
    });
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_sell_tokens_with_platform"></a>

## Function `sell_tokens_with_platform`

Sell tokens back to the pool
Platform version: platform fees go to platform treasury, includes platform validation.
Consumes the SocialToken by value. On a partial sell a new remainder token is minted and
transferred back to the seller; on a full sell the object is deleted — no zombie tokens.


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_sell_tokens_with_platform">sell_tokens_with_platform</a>(_registry: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">social_contracts::social_proof_tokens::TokenRegistry</a>, pool: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">social_contracts::social_proof_tokens::TokenPool</a>, config: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">social_contracts::social_proof_tokens::SocialProofTokensConfig</a>, treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, platform_registry: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">social_contracts::platform::PlatformRegistry</a>, profile_registry: &<a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">social_contracts::profile::UsernameRegistry</a>, block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, social_token: <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialToken">social_contracts::social_proof_tokens::SocialToken</a>, amount: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_sell_tokens_with_platform">sell_tokens_with_platform</a>(
    _registry: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">TokenRegistry</a>,
    pool: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">TokenPool</a>,
    config: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">SocialProofTokensConfig</a>,
    treasury: &EcosystemTreasury,
    platform_registry: &PlatformRegistry,
    profile_registry: &UsernameRegistry,
    block_list_registry: &BlockListRegistry,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>,
    social_token: <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialToken">SocialToken</a>,
    amount: u64,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    // Check version compatibility
    <b>assert</b>!(pool.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EWrongVersion">EWrongVersion</a>);
    // Check <b>if</b> trading is halted
    <b>assert</b>!(config.trading_enabled, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ETradingHalted">ETradingHalted</a>);
    <b>let</b> seller = tx_context::sender(ctx);
    <b>let</b> pool_id = object::uid_to_address(&pool.id);
    // Look up the seller's <a href="../social_contracts/profile.md#social_contracts_profile">profile</a> ID
    <b>let</b> profile_id_option = <a href="../social_contracts/profile.md#social_contracts_profile_lookup_profile_by_owner">profile::lookup_profile_by_owner</a>(profile_registry, seller);
    <b>assert</b>!(option::is_some(&profile_id_option), <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ENotAuthorized">ENotAuthorized</a>);
    // Platform validation
    <b>let</b> platform_id = object::uid_to_address(<a href="../social_contracts/platform.md#social_contracts_platform_id">platform::id</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>));
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform_is_approved">platform::is_approved</a>(platform_registry, platform_id), <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ENotAuthorized">ENotAuthorized</a>);
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform_has_joined_platform">platform::has_joined_platform</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, seller), <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EUserNotJoinedPlatform">EUserNotJoinedPlatform</a>);
    <b>assert</b>!(!<a href="../social_contracts/block_list.md#social_contracts_block_list_is_blocked">block_list::is_blocked</a>(block_list_registry, platform_id, seller), <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EUserBlockedByPlatform">EUserBlockedByPlatform</a>);
    // Verify social token matches the pool and <b>has</b> sufficient balance
    <b>assert</b>!(social_token.pool_id == pool_id, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInvalidID">EInvalidID</a>);
    <b>assert</b>!(social_token.amount &gt;= amount, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInsufficientLiquidity">EInsufficientLiquidity</a>);
    // Calculate the sell price based on quadratic curve
    <b>let</b> (refund_amount, _) = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_sell_price">calculate_sell_price</a>(
        pool.info.base_price,
        pool.info.quadratic_coefficient,
        pool.info.circulating_supply,
        amount
    );
    // Validate fees and calculate with overflow protection
    <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_validate_trading_fees">validate_trading_fees</a>(config);
    <b>let</b> total_fee_bps = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_total_fee_bps">calculate_total_fee_bps</a>(config);
    <b>let</b> fee_amount = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_fee_amount_safe">calculate_fee_amount_safe</a>(refund_amount, total_fee_bps);
    <b>let</b> creator_fee = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_component_fee_safe">calculate_component_fee_safe</a>(fee_amount, config.trading_creator_fee_bps, total_fee_bps);
    <b>let</b> platform_fee = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_component_fee_safe">calculate_component_fee_safe</a>(fee_amount, config.trading_platform_fee_bps, total_fee_bps);
    <b>let</b> treasury_fee = fee_amount - creator_fee - platform_fee;
    // Calculate net refund
    <b>let</b> net_refund = refund_amount - fee_amount;
    // Ensure pool <b>has</b> enough liquidity <b>for</b> refund + all fees
    <b>assert</b>!(balance::value(&pool.myso_balance) &gt;= refund_amount, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInsufficientLiquidity">EInsufficientLiquidity</a>);
    // Verify seller <b>has</b> tokens in the pool
    <b>assert</b>!(table::contains(&pool.holders, seller), <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ENoTokensOwned">ENoTokensOwned</a>);
    // Update holder balance
    <b>let</b> holder_balance = table::borrow_mut(&<b>mut</b> pool.holders, seller);
    <b>if</b> (*holder_balance == amount) {
        // Remove holder completely <b>if</b> selling all tokens
        table::remove(&<b>mut</b> pool.holders, seller);
    } <b>else</b> {
        // Reduce balance
        *holder_balance = *holder_balance - amount;
    };
    // Consume the social token: on partial sell mint a remainder token; on full sell delete.
    <b>let</b> remainder = social_token.amount - amount;
    <b>let</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialToken">SocialToken</a> { id, pool_id: token_pool_id, token_type, amount: _ } = social_token;
    object::delete(id);
    <b>if</b> (remainder &gt; 0) {
        <b>let</b> remainder_token = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialToken">SocialToken</a> {
            id: object::new(ctx),
            pool_id: token_pool_id,
            token_type,
            amount: remainder,
        };
        transfer::transfer(remainder_token, seller);
    };
    // Update circulating supply
    pool.info.circulating_supply = pool.info.circulating_supply - amount;
    // Extract net refund from pool
    <b>let</b> refund_balance = balance::split(&<b>mut</b> pool.myso_balance, net_refund);
    // Process and distribute fees with PoC redirection support
    <b>if</b> (fee_amount &gt; 0) {
        // Send fee to creator with PoC redirection support
        <b>if</b> (creator_fee &gt; 0) {
            <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_distribute_creator_fee_from_pool">distribute_creator_fee_from_pool</a>(pool, creator_fee, ctx);
        };
        // Send <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> fee to <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> treasury
        <b>if</b> (platform_fee &gt; 0) {
            <b>let</b> <b>mut</b> platform_fee_coin = coin::from_balance(balance::split(&<b>mut</b> pool.myso_balance, platform_fee), ctx);
            <a href="../social_contracts/platform.md#social_contracts_platform_add_to_treasury">social_contracts::platform::add_to_treasury</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, &<b>mut</b> platform_fee_coin, platform_fee, clock, ctx);
            coin::destroy_zero(platform_fee_coin);
        };
        // Send fee to treasury
        <b>if</b> (treasury_fee &gt; 0) {
            <b>let</b> treasury_fee_coin = coin::from_balance(balance::split(&<b>mut</b> pool.myso_balance, treasury_fee), ctx);
            transfer::public_transfer(treasury_fee_coin, <a href="../social_contracts/profile.md#social_contracts_profile_get_treasury_address">profile::get_treasury_address</a>(treasury));
        };
    };
    // Transfer refund to seller
    <b>let</b> refund_coin = coin::from_balance(refund_balance, ctx);
    transfer::public_transfer(refund_coin, seller);
    // Calculate the new price after sale
    <b>let</b> new_price = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_token_price">calculate_token_price</a>(
        pool.info.base_price,
        pool.info.quadratic_coefficient,
        pool.info.circulating_supply
    );
    // Emit sell event
    event::emit(<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenSoldEvent">TokenSoldEvent</a> {
        id: pool_id,
        seller,
        amount,
        myso_amount: refund_amount,
        fee_amount,
        creator_fee,
        platform_fee,
        treasury_fee,
        new_price,
    });
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_transfer_tokens"></a>

## Function `transfer_tokens`

Transfer a <code><a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialToken">SocialToken</a></code> to another address and update the pool <code>holders</code> ledger.
Split first if only part of a balance should be sent.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_transfer_tokens">transfer_tokens</a>(pool: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">social_contracts::social_proof_tokens::TokenPool</a>, config: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">social_contracts::social_proof_tokens::SocialProofTokensConfig</a>, token: <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialToken">social_contracts::social_proof_tokens::SocialToken</a>, recipient: <b>address</b>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_transfer_tokens">transfer_tokens</a>(
    pool: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">TokenPool</a>,
    config: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">SocialProofTokensConfig</a>,
    token: <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialToken">SocialToken</a>,
    recipient: <b>address</b>,
    ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(config.trading_enabled, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ETradingHalted">ETradingHalted</a>);
    <b>assert</b>!(pool.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EWrongVersion">EWrongVersion</a>);
    <b>let</b> sender = tx_context::sender(ctx);
    <b>assert</b>!(recipient != sender, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ESelfTransfer">ESelfTransfer</a>);
    <b>let</b> pool_id = object::uid_to_address(&pool.id);
    <b>assert</b>!(token.pool_id == pool_id, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInvalidID">EInvalidID</a>);
    <b>let</b> amount = token.amount;
    <b>assert</b>!(amount &gt; 0, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInvalidTransferAmount">EInvalidTransferAmount</a>);
    <b>assert</b>!(table::contains(&pool.holders, sender), <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ENoTokensOwned">ENoTokensOwned</a>);
    <b>let</b> sender_balance = table::borrow_mut(&<b>mut</b> pool.holders, sender);
    <b>assert</b>!(*sender_balance &gt;= amount, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInsufficientLiquidity">EInsufficientLiquidity</a>);
    <b>if</b> (*sender_balance == amount) {
        table::remove(&<b>mut</b> pool.holders, sender);
    } <b>else</b> {
        *sender_balance = *sender_balance - amount;
    };
    <b>let</b> recipient_hold = <b>if</b> (table::contains(&pool.holders, recipient)) {
        *table::borrow(&pool.holders, recipient)
    } <b>else</b> {
        0
    };
    <b>let</b> supply = pool.info.circulating_supply;
    <b>assert</b>!(supply == 0 || supply &lt;= <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_MAX_U64">MAX_U64</a> / config.max_hold_percent_bps, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EOverflow">EOverflow</a>);
    <b>let</b> max_hold = (supply * config.max_hold_percent_bps) / <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_BPS_DENOM">BPS_DENOM</a>;
    <b>assert</b>!(recipient_hold &lt;= <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_MAX_U64">MAX_U64</a> - amount, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EOverflow">EOverflow</a>);
    <b>assert</b>!(recipient_hold + amount &lt;= max_hold, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EExceededMaxHold">EExceededMaxHold</a>);
    <b>if</b> (table::contains(&pool.holders, recipient)) {
        <b>let</b> recipient_balance = table::borrow_mut(&<b>mut</b> pool.holders, recipient);
        *recipient_balance = *recipient_balance + amount;
    } <b>else</b> {
        table::add(&<b>mut</b> pool.holders, recipient, amount);
    };
    event::emit(<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenTransferredEvent">TokenTransferredEvent</a> {
        pool_id,
        from: sender,
        to: recipient,
        amount,
    });
    transfer::transfer(token, recipient);
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_swap_tokens"></a>

## Function `swap_tokens`

Exact-in SPT→SPT swap for a first dest position (mints a new dest <code><a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialToken">SocialToken</a></code>).
Non-platform: platform fee legs go to the ecosystem treasury.


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_swap_tokens">swap_tokens</a>(_registry: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">social_contracts::social_proof_tokens::TokenRegistry</a>, source_pool: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">social_contracts::social_proof_tokens::TokenPool</a>, dest_pool: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">social_contracts::social_proof_tokens::TokenPool</a>, config: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">social_contracts::social_proof_tokens::SocialProofTokensConfig</a>, treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, profile_registry: &<a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">social_contracts::profile::UsernameRegistry</a>, block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, source_token: <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialToken">social_contracts::social_proof_tokens::SocialToken</a>, sell_amount: u64, min_dest_amount: u64, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_swap_tokens">swap_tokens</a>(
    _registry: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">TokenRegistry</a>,
    source_pool: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">TokenPool</a>,
    dest_pool: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">TokenPool</a>,
    config: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">SocialProofTokensConfig</a>,
    treasury: &EcosystemTreasury,
    profile_registry: &UsernameRegistry,
    block_list_registry: &BlockListRegistry,
    source_token: <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialToken">SocialToken</a>,
    sell_amount: u64,
    min_dest_amount: u64,
    ctx: &<b>mut</b> TxContext
) {
    <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_execute_swap_non_platform">execute_swap_non_platform</a>(
        source_pool,
        dest_pool,
        config,
        treasury,
        profile_registry,
        block_list_registry,
        source_token,
        sell_amount,
        min_dest_amount,
        <b>false</b>,
        ctx
    );
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_swap_more_tokens"></a>

## Function `swap_more_tokens`

Exact-in SPT→SPT swap when the trader already holds dest tokens.


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_swap_more_tokens">swap_more_tokens</a>(_registry: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">social_contracts::social_proof_tokens::TokenRegistry</a>, source_pool: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">social_contracts::social_proof_tokens::TokenPool</a>, dest_pool: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">social_contracts::social_proof_tokens::TokenPool</a>, config: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">social_contracts::social_proof_tokens::SocialProofTokensConfig</a>, treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, profile_registry: &<a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">social_contracts::profile::UsernameRegistry</a>, block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, source_token: <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialToken">social_contracts::social_proof_tokens::SocialToken</a>, dest_token: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialToken">social_contracts::social_proof_tokens::SocialToken</a>, sell_amount: u64, min_dest_amount: u64, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_swap_more_tokens">swap_more_tokens</a>(
    _registry: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">TokenRegistry</a>,
    source_pool: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">TokenPool</a>,
    dest_pool: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">TokenPool</a>,
    config: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">SocialProofTokensConfig</a>,
    treasury: &EcosystemTreasury,
    profile_registry: &UsernameRegistry,
    block_list_registry: &BlockListRegistry,
    source_token: <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialToken">SocialToken</a>,
    dest_token: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialToken">SocialToken</a>,
    sell_amount: u64,
    min_dest_amount: u64,
    ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(dest_token.pool_id == object::uid_to_address(&dest_pool.id), <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInvalidID">EInvalidID</a>);
    <b>let</b> dest_amount = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_execute_swap_non_platform">execute_swap_non_platform</a>(
        source_pool,
        dest_pool,
        config,
        treasury,
        profile_registry,
        block_list_registry,
        source_token,
        sell_amount,
        min_dest_amount,
        <b>true</b>,
        ctx
    );
    dest_token.amount = dest_token.amount + dest_amount;
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_swap_tokens_with_platform"></a>

## Function `swap_tokens_with_platform`

Exact-in SPT→SPT swap (mint dest) with platform fee routing.


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_swap_tokens_with_platform">swap_tokens_with_platform</a>(_registry: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">social_contracts::social_proof_tokens::TokenRegistry</a>, source_pool: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">social_contracts::social_proof_tokens::TokenPool</a>, dest_pool: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">social_contracts::social_proof_tokens::TokenPool</a>, config: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">social_contracts::social_proof_tokens::SocialProofTokensConfig</a>, treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, platform_registry: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">social_contracts::platform::PlatformRegistry</a>, profile_registry: &<a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">social_contracts::profile::UsernameRegistry</a>, block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, source_token: <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialToken">social_contracts::social_proof_tokens::SocialToken</a>, sell_amount: u64, min_dest_amount: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_swap_tokens_with_platform">swap_tokens_with_platform</a>(
    _registry: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">TokenRegistry</a>,
    source_pool: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">TokenPool</a>,
    dest_pool: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">TokenPool</a>,
    config: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">SocialProofTokensConfig</a>,
    treasury: &EcosystemTreasury,
    platform_registry: &PlatformRegistry,
    profile_registry: &UsernameRegistry,
    block_list_registry: &BlockListRegistry,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>,
    source_token: <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialToken">SocialToken</a>,
    sell_amount: u64,
    min_dest_amount: u64,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> trader = tx_context::sender(ctx);
    <b>let</b> platform_id = object::uid_to_address(<a href="../social_contracts/platform.md#social_contracts_platform_id">platform::id</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>));
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform_is_approved">platform::is_approved</a>(platform_registry, platform_id), <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ENotAuthorized">ENotAuthorized</a>);
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform_has_joined_platform">platform::has_joined_platform</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, trader), <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EUserNotJoinedPlatform">EUserNotJoinedPlatform</a>);
    <b>assert</b>!(!<a href="../social_contracts/block_list.md#social_contracts_block_list_is_blocked">block_list::is_blocked</a>(block_list_registry, platform_id, trader), <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EUserBlockedByPlatform">EUserBlockedByPlatform</a>);
    <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_execute_swap_with_platform">execute_swap_with_platform</a>(
        source_pool,
        dest_pool,
        config,
        treasury,
        profile_registry,
        block_list_registry,
        <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>,
        source_token,
        sell_amount,
        min_dest_amount,
        <b>false</b>,
        clock,
        ctx
    );
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_swap_more_tokens_with_platform"></a>

## Function `swap_more_tokens_with_platform`

Exact-in SPT→SPT swap (credit dest) with platform fee routing.


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_swap_more_tokens_with_platform">swap_more_tokens_with_platform</a>(_registry: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">social_contracts::social_proof_tokens::TokenRegistry</a>, source_pool: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">social_contracts::social_proof_tokens::TokenPool</a>, dest_pool: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">social_contracts::social_proof_tokens::TokenPool</a>, config: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">social_contracts::social_proof_tokens::SocialProofTokensConfig</a>, treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, platform_registry: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">social_contracts::platform::PlatformRegistry</a>, profile_registry: &<a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">social_contracts::profile::UsernameRegistry</a>, block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, source_token: <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialToken">social_contracts::social_proof_tokens::SocialToken</a>, dest_token: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialToken">social_contracts::social_proof_tokens::SocialToken</a>, sell_amount: u64, min_dest_amount: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_swap_more_tokens_with_platform">swap_more_tokens_with_platform</a>(
    _registry: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">TokenRegistry</a>,
    source_pool: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">TokenPool</a>,
    dest_pool: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">TokenPool</a>,
    config: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">SocialProofTokensConfig</a>,
    treasury: &EcosystemTreasury,
    platform_registry: &PlatformRegistry,
    profile_registry: &UsernameRegistry,
    block_list_registry: &BlockListRegistry,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>,
    source_token: <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialToken">SocialToken</a>,
    dest_token: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialToken">SocialToken</a>,
    sell_amount: u64,
    min_dest_amount: u64,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(dest_token.pool_id == object::uid_to_address(&dest_pool.id), <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInvalidID">EInvalidID</a>);
    <b>let</b> trader = tx_context::sender(ctx);
    <b>let</b> platform_id = object::uid_to_address(<a href="../social_contracts/platform.md#social_contracts_platform_id">platform::id</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>));
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform_is_approved">platform::is_approved</a>(platform_registry, platform_id), <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ENotAuthorized">ENotAuthorized</a>);
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform_has_joined_platform">platform::has_joined_platform</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, trader), <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EUserNotJoinedPlatform">EUserNotJoinedPlatform</a>);
    <b>assert</b>!(!<a href="../social_contracts/block_list.md#social_contracts_block_list_is_blocked">block_list::is_blocked</a>(block_list_registry, platform_id, trader), <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EUserBlockedByPlatform">EUserBlockedByPlatform</a>);
    <b>let</b> dest_amount = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_execute_swap_with_platform">execute_swap_with_platform</a>(
        source_pool,
        dest_pool,
        config,
        treasury,
        profile_registry,
        block_list_registry,
        <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>,
        source_token,
        sell_amount,
        min_dest_amount,
        <b>true</b>,
        clock,
        ctx
    );
    dest_token.amount = dest_token.amount + dest_amount;
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_execute_swap_non_platform"></a>

## Function `execute_swap_non_platform`

Returns dest nano-SPT purchased. When <code>credit_existing</code> is false, mints a new SocialToken.
When true, only updates holders/supply; caller must credit <code>&<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialToken">SocialToken</a></code>.


<pre><code><b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_execute_swap_non_platform">execute_swap_non_platform</a>(source_pool: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">social_contracts::social_proof_tokens::TokenPool</a>, dest_pool: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">social_contracts::social_proof_tokens::TokenPool</a>, config: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">social_contracts::social_proof_tokens::SocialProofTokensConfig</a>, treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, profile_registry: &<a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">social_contracts::profile::UsernameRegistry</a>, block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, source_token: <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialToken">social_contracts::social_proof_tokens::SocialToken</a>, sell_amount: u64, min_dest_amount: u64, credit_existing: bool, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_execute_swap_non_platform">execute_swap_non_platform</a>(
    source_pool: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">TokenPool</a>,
    dest_pool: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">TokenPool</a>,
    config: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">SocialProofTokensConfig</a>,
    treasury: &EcosystemTreasury,
    profile_registry: &UsernameRegistry,
    block_list_registry: &BlockListRegistry,
    source_token: <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialToken">SocialToken</a>,
    sell_amount: u64,
    min_dest_amount: u64,
    credit_existing: bool,
    ctx: &<b>mut</b> TxContext
): u64 {
    <b>assert</b>!(source_pool.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EWrongVersion">EWrongVersion</a>);
    <b>assert</b>!(dest_pool.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EWrongVersion">EWrongVersion</a>);
    <b>assert</b>!(config.trading_enabled, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ETradingHalted">ETradingHalted</a>);
    <b>let</b> trader = tx_context::sender(ctx);
    <b>let</b> source_pool_id = object::uid_to_address(&source_pool.id);
    <b>let</b> dest_pool_id = object::uid_to_address(&dest_pool.id);
    <b>assert</b>!(source_pool_id != dest_pool_id, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ESamePool">ESamePool</a>);
    <b>let</b> profile_id_option = <a href="../social_contracts/profile.md#social_contracts_profile_lookup_profile_by_owner">profile::lookup_profile_by_owner</a>(profile_registry, trader);
    <b>assert</b>!(option::is_some(&profile_id_option), <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ENotAuthorized">ENotAuthorized</a>);
    <b>assert</b>!(!<a href="../social_contracts/block_list.md#social_contracts_block_list_is_blocked">block_list::is_blocked</a>(block_list_registry, trader, dest_pool.info.owner), <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EBlockedUser">EBlockedUser</a>);
    <b>assert</b>!(source_token.pool_id == source_pool_id, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInvalidID">EInvalidID</a>);
    <b>assert</b>!(sell_amount &gt; 0, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInsufficientFunds">EInsufficientFunds</a>);
    <b>assert</b>!(source_token.amount &gt;= sell_amount, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInsufficientLiquidity">EInsufficientLiquidity</a>);
    <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_validate_trading_fees">validate_trading_fees</a>(config);
    <b>let</b> total_fee_bps = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_total_fee_bps">calculate_total_fee_bps</a>(config);
    <b>let</b> (sell_gross, sell_fee_amount, net_bridge) = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_swap_proceeds">calculate_swap_proceeds</a>(
        source_pool.info.base_price,
        source_pool.info.quadratic_coefficient,
        source_pool.info.circulating_supply,
        sell_amount,
        total_fee_bps
    );
    <b>let</b> sell_creator_fee = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_component_fee_safe">calculate_component_fee_safe</a>(sell_fee_amount, config.trading_creator_fee_bps, total_fee_bps);
    <b>let</b> sell_platform_fee = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_component_fee_safe">calculate_component_fee_safe</a>(sell_fee_amount, config.trading_platform_fee_bps, total_fee_bps);
    <b>let</b> sell_treasury_fee = sell_fee_amount - sell_creator_fee - sell_platform_fee;
    <b>assert</b>!(balance::value(&source_pool.myso_balance) &gt;= sell_gross, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInsufficientLiquidity">EInsufficientLiquidity</a>);
    <b>assert</b>!(table::contains(&source_pool.holders, trader), <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ENoTokensOwned">ENoTokensOwned</a>);
    <b>let</b> holder_balance = table::borrow_mut(&<b>mut</b> source_pool.holders, trader);
    <b>if</b> (*holder_balance == sell_amount) {
        table::remove(&<b>mut</b> source_pool.holders, trader);
    } <b>else</b> {
        *holder_balance = *holder_balance - sell_amount;
    };
    <b>let</b> remainder = source_token.amount - sell_amount;
    <b>let</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialToken">SocialToken</a> { id, pool_id: token_pool_id, token_type, amount: _ } = source_token;
    object::delete(id);
    <b>if</b> (remainder &gt; 0) {
        transfer::transfer(<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialToken">SocialToken</a> {
            id: object::new(ctx),
            pool_id: token_pool_id,
            token_type,
            amount: remainder,
        }, trader);
    };
    source_pool.info.circulating_supply = source_pool.info.circulating_supply - sell_amount;
    <b>let</b> <b>mut</b> bridge = balance::split(&<b>mut</b> source_pool.myso_balance, net_bridge);
    <b>if</b> (sell_fee_amount &gt; 0) {
        <b>if</b> (sell_creator_fee &gt; 0) {
            <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_distribute_creator_fee_from_pool">distribute_creator_fee_from_pool</a>(source_pool, sell_creator_fee, ctx);
        };
        <b>if</b> (sell_platform_fee &gt; 0) {
            <b>let</b> c = coin::from_balance(balance::split(&<b>mut</b> source_pool.myso_balance, sell_platform_fee), ctx);
            transfer::public_transfer(c, <a href="../social_contracts/profile.md#social_contracts_profile_get_treasury_address">profile::get_treasury_address</a>(treasury));
        };
        <b>if</b> (sell_treasury_fee &gt; 0) {
            <b>let</b> c = coin::from_balance(balance::split(&<b>mut</b> source_pool.myso_balance, sell_treasury_fee), ctx);
            transfer::public_transfer(c, <a href="../social_contracts/profile.md#social_contracts_profile_get_treasury_address">profile::get_treasury_address</a>(treasury));
        };
    };
    <b>let</b> source_new_price = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_token_price">calculate_token_price</a>(
        source_pool.info.base_price,
        source_pool.info.quadratic_coefficient,
        source_pool.info.circulating_supply
    );
    event::emit(<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenSoldEvent">TokenSoldEvent</a> {
        id: source_pool_id,
        seller: trader,
        amount: sell_amount,
        myso_amount: sell_gross,
        fee_amount: sell_fee_amount,
        creator_fee: sell_creator_fee,
        platform_fee: sell_platform_fee,
        treasury_fee: sell_treasury_fee,
        new_price: source_new_price,
    });
    <b>let</b> bridge_value = balance::value(&bridge);
    <b>let</b> (dest_amount, buy_gross) = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_max_buy_amount">calculate_max_buy_amount</a>(
        dest_pool.info.base_price,
        dest_pool.info.quadratic_coefficient,
        dest_pool.info.circulating_supply,
        bridge_value
    );
    <b>assert</b>!(dest_amount &gt;= min_dest_amount, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ESlippageExceeded">ESlippageExceeded</a>);
    <b>assert</b>!(dest_amount &gt; 0, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInsufficientFunds">EInsufficientFunds</a>);
    <b>assert</b>!(bridge_value &gt;= buy_gross, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInsufficientFunds">EInsufficientFunds</a>);
    <b>let</b> buy_fee_amount = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_fee_amount_safe">calculate_fee_amount_safe</a>(buy_gross, total_fee_bps);
    <b>let</b> buy_creator_fee = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_component_fee_safe">calculate_component_fee_safe</a>(buy_fee_amount, config.trading_creator_fee_bps, total_fee_bps);
    <b>let</b> buy_platform_fee = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_component_fee_safe">calculate_component_fee_safe</a>(buy_fee_amount, config.trading_platform_fee_bps, total_fee_bps);
    <b>let</b> buy_treasury_fee = buy_fee_amount - buy_creator_fee - buy_platform_fee;
    <b>let</b> buy_net = buy_gross - buy_fee_amount;
    <b>let</b> <b>mut</b> payment = coin::from_balance(balance::split(&<b>mut</b> bridge, buy_gross), ctx);
    <b>if</b> (buy_fee_amount &gt; 0) {
        <b>if</b> (buy_creator_fee &gt; 0) {
            <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_distribute_creator_fee">distribute_creator_fee</a>(dest_pool, buy_creator_fee, &<b>mut</b> payment, ctx);
        };
        <b>if</b> (buy_platform_fee &gt; 0) {
            <b>let</b> c = coin::split(&<b>mut</b> payment, buy_platform_fee, ctx);
            transfer::public_transfer(c, <a href="../social_contracts/profile.md#social_contracts_profile_get_treasury_address">profile::get_treasury_address</a>(treasury));
        };
        <b>if</b> (buy_treasury_fee &gt; 0) {
            <b>let</b> c = coin::split(&<b>mut</b> payment, buy_treasury_fee, ctx);
            transfer::public_transfer(c, <a href="../social_contracts/profile.md#social_contracts_profile_get_treasury_address">profile::get_treasury_address</a>(treasury));
        };
    };
    <b>let</b> pool_payment = coin::split(&<b>mut</b> payment, buy_net, ctx);
    balance::join(&<b>mut</b> dest_pool.myso_balance, coin::into_balance(pool_payment));
    coin::destroy_zero(payment);
    <b>let</b> leftover_myso = balance::value(&bridge);
    <b>if</b> (leftover_myso &gt; 0) {
        transfer::public_transfer(coin::from_balance(bridge, ctx), trader);
    } <b>else</b> {
        balance::destroy_zero(bridge);
    };
    <b>assert</b>!(dest_pool.info.circulating_supply &lt;= <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_MAX_U64">MAX_U64</a> - dest_amount, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EOverflow">EOverflow</a>);
    <b>let</b> new_supply = dest_pool.info.circulating_supply + dest_amount;
    <b>assert</b>!(new_supply == 0 || new_supply &lt;= <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_MAX_U64">MAX_U64</a> / config.max_hold_percent_bps, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EOverflow">EOverflow</a>);
    <b>let</b> max_hold = (new_supply * config.max_hold_percent_bps) / <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_BPS_DENOM">BPS_DENOM</a>;
    <b>let</b> current_hold = <b>if</b> (table::contains(&dest_pool.holders, trader)) {
        *table::borrow(&dest_pool.holders, trader)
    } <b>else</b> {
        0
    };
    <b>assert</b>!(current_hold &lt;= <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_MAX_U64">MAX_U64</a> - dest_amount, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EOverflow">EOverflow</a>);
    <b>assert</b>!(current_hold + dest_amount &lt;= max_hold, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EExceededMaxHold">EExceededMaxHold</a>);
    <b>if</b> (credit_existing) {
        <b>assert</b>!(current_hold &gt; 0, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ENoTokensOwned">ENoTokensOwned</a>);
        *table::borrow_mut(&<b>mut</b> dest_pool.holders, trader) = current_hold + dest_amount;
    } <b>else</b> {
        <b>assert</b>!(current_hold == 0, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EAlreadyOwnsTokens">EAlreadyOwnsTokens</a>);
        table::add(&<b>mut</b> dest_pool.holders, trader, dest_amount);
        transfer::transfer(<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialToken">SocialToken</a> {
            id: object::new(ctx),
            pool_id: dest_pool_id,
            token_type: dest_pool.info.token_type,
            amount: dest_amount,
        }, trader);
    };
    dest_pool.info.circulating_supply = new_supply;
    <b>let</b> dest_new_price = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_token_price">calculate_token_price</a>(
        dest_pool.info.base_price,
        dest_pool.info.quadratic_coefficient,
        dest_pool.info.circulating_supply
    );
    event::emit(<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenBoughtEvent">TokenBoughtEvent</a> {
        id: dest_pool_id,
        buyer: trader,
        amount: dest_amount,
        myso_amount: buy_gross,
        fee_amount: buy_fee_amount,
        creator_fee: buy_creator_fee,
        platform_fee: buy_platform_fee,
        treasury_fee: buy_treasury_fee,
        new_price: dest_new_price,
    });
    event::emit(<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenSwappedEvent">TokenSwappedEvent</a> {
        source_pool_id,
        dest_pool_id,
        trader,
        sell_amount,
        dest_amount,
        sell_myso_gross: sell_gross,
        buy_myso_gross: buy_gross,
        sell_fee_amount,
        buy_fee_amount,
        sell_creator_fee,
        sell_platform_fee,
        sell_treasury_fee,
        buy_creator_fee,
        buy_platform_fee,
        buy_treasury_fee,
        leftover_myso,
        source_new_price,
        dest_new_price,
    });
    dest_amount
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_execute_swap_with_platform"></a>

## Function `execute_swap_with_platform`



<pre><code><b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_execute_swap_with_platform">execute_swap_with_platform</a>(source_pool: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">social_contracts::social_proof_tokens::TokenPool</a>, dest_pool: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">social_contracts::social_proof_tokens::TokenPool</a>, config: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">social_contracts::social_proof_tokens::SocialProofTokensConfig</a>, treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, profile_registry: &<a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">social_contracts::profile::UsernameRegistry</a>, block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, source_token: <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialToken">social_contracts::social_proof_tokens::SocialToken</a>, sell_amount: u64, min_dest_amount: u64, credit_existing: bool, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_execute_swap_with_platform">execute_swap_with_platform</a>(
    source_pool: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">TokenPool</a>,
    dest_pool: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">TokenPool</a>,
    config: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">SocialProofTokensConfig</a>,
    treasury: &EcosystemTreasury,
    profile_registry: &UsernameRegistry,
    block_list_registry: &BlockListRegistry,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>,
    source_token: <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialToken">SocialToken</a>,
    sell_amount: u64,
    min_dest_amount: u64,
    credit_existing: bool,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
): u64 {
    <b>assert</b>!(source_pool.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EWrongVersion">EWrongVersion</a>);
    <b>assert</b>!(dest_pool.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EWrongVersion">EWrongVersion</a>);
    <b>assert</b>!(config.trading_enabled, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ETradingHalted">ETradingHalted</a>);
    <b>let</b> trader = tx_context::sender(ctx);
    <b>let</b> source_pool_id = object::uid_to_address(&source_pool.id);
    <b>let</b> dest_pool_id = object::uid_to_address(&dest_pool.id);
    <b>assert</b>!(source_pool_id != dest_pool_id, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ESamePool">ESamePool</a>);
    <b>let</b> profile_id_option = <a href="../social_contracts/profile.md#social_contracts_profile_lookup_profile_by_owner">profile::lookup_profile_by_owner</a>(profile_registry, trader);
    <b>assert</b>!(option::is_some(&profile_id_option), <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ENotAuthorized">ENotAuthorized</a>);
    <b>assert</b>!(!<a href="../social_contracts/block_list.md#social_contracts_block_list_is_blocked">block_list::is_blocked</a>(block_list_registry, trader, dest_pool.info.owner), <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EBlockedUser">EBlockedUser</a>);
    <b>assert</b>!(source_token.pool_id == source_pool_id, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInvalidID">EInvalidID</a>);
    <b>assert</b>!(sell_amount &gt; 0, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInsufficientFunds">EInsufficientFunds</a>);
    <b>assert</b>!(source_token.amount &gt;= sell_amount, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInsufficientLiquidity">EInsufficientLiquidity</a>);
    <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_validate_trading_fees">validate_trading_fees</a>(config);
    <b>let</b> total_fee_bps = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_total_fee_bps">calculate_total_fee_bps</a>(config);
    <b>let</b> (sell_gross, sell_fee_amount, net_bridge) = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_swap_proceeds">calculate_swap_proceeds</a>(
        source_pool.info.base_price,
        source_pool.info.quadratic_coefficient,
        source_pool.info.circulating_supply,
        sell_amount,
        total_fee_bps
    );
    <b>let</b> sell_creator_fee = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_component_fee_safe">calculate_component_fee_safe</a>(sell_fee_amount, config.trading_creator_fee_bps, total_fee_bps);
    <b>let</b> sell_platform_fee = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_component_fee_safe">calculate_component_fee_safe</a>(sell_fee_amount, config.trading_platform_fee_bps, total_fee_bps);
    <b>let</b> sell_treasury_fee = sell_fee_amount - sell_creator_fee - sell_platform_fee;
    <b>assert</b>!(balance::value(&source_pool.myso_balance) &gt;= sell_gross, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInsufficientLiquidity">EInsufficientLiquidity</a>);
    <b>assert</b>!(table::contains(&source_pool.holders, trader), <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ENoTokensOwned">ENoTokensOwned</a>);
    <b>let</b> holder_balance = table::borrow_mut(&<b>mut</b> source_pool.holders, trader);
    <b>if</b> (*holder_balance == sell_amount) {
        table::remove(&<b>mut</b> source_pool.holders, trader);
    } <b>else</b> {
        *holder_balance = *holder_balance - sell_amount;
    };
    <b>let</b> remainder = source_token.amount - sell_amount;
    <b>let</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialToken">SocialToken</a> { id, pool_id: token_pool_id, token_type, amount: _ } = source_token;
    object::delete(id);
    <b>if</b> (remainder &gt; 0) {
        transfer::transfer(<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialToken">SocialToken</a> {
            id: object::new(ctx),
            pool_id: token_pool_id,
            token_type,
            amount: remainder,
        }, trader);
    };
    source_pool.info.circulating_supply = source_pool.info.circulating_supply - sell_amount;
    <b>let</b> <b>mut</b> bridge = balance::split(&<b>mut</b> source_pool.myso_balance, net_bridge);
    <b>if</b> (sell_fee_amount &gt; 0) {
        <b>if</b> (sell_creator_fee &gt; 0) {
            <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_distribute_creator_fee_from_pool">distribute_creator_fee_from_pool</a>(source_pool, sell_creator_fee, ctx);
        };
        <b>if</b> (sell_platform_fee &gt; 0) {
            <b>let</b> <b>mut</b> c = coin::from_balance(balance::split(&<b>mut</b> source_pool.myso_balance, sell_platform_fee), ctx);
            <a href="../social_contracts/platform.md#social_contracts_platform_add_to_treasury">social_contracts::platform::add_to_treasury</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, &<b>mut</b> c, sell_platform_fee, clock, ctx);
            coin::destroy_zero(c);
        };
        <b>if</b> (sell_treasury_fee &gt; 0) {
            <b>let</b> c = coin::from_balance(balance::split(&<b>mut</b> source_pool.myso_balance, sell_treasury_fee), ctx);
            transfer::public_transfer(c, <a href="../social_contracts/profile.md#social_contracts_profile_get_treasury_address">profile::get_treasury_address</a>(treasury));
        };
    };
    <b>let</b> source_new_price = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_token_price">calculate_token_price</a>(
        source_pool.info.base_price,
        source_pool.info.quadratic_coefficient,
        source_pool.info.circulating_supply
    );
    event::emit(<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenSoldEvent">TokenSoldEvent</a> {
        id: source_pool_id,
        seller: trader,
        amount: sell_amount,
        myso_amount: sell_gross,
        fee_amount: sell_fee_amount,
        creator_fee: sell_creator_fee,
        platform_fee: sell_platform_fee,
        treasury_fee: sell_treasury_fee,
        new_price: source_new_price,
    });
    <b>let</b> bridge_value = balance::value(&bridge);
    <b>let</b> (dest_amount, buy_gross) = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_max_buy_amount">calculate_max_buy_amount</a>(
        dest_pool.info.base_price,
        dest_pool.info.quadratic_coefficient,
        dest_pool.info.circulating_supply,
        bridge_value
    );
    <b>assert</b>!(dest_amount &gt;= min_dest_amount, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ESlippageExceeded">ESlippageExceeded</a>);
    <b>assert</b>!(dest_amount &gt; 0, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInsufficientFunds">EInsufficientFunds</a>);
    <b>assert</b>!(bridge_value &gt;= buy_gross, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInsufficientFunds">EInsufficientFunds</a>);
    <b>let</b> buy_fee_amount = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_fee_amount_safe">calculate_fee_amount_safe</a>(buy_gross, total_fee_bps);
    <b>let</b> buy_creator_fee = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_component_fee_safe">calculate_component_fee_safe</a>(buy_fee_amount, config.trading_creator_fee_bps, total_fee_bps);
    <b>let</b> buy_platform_fee = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_component_fee_safe">calculate_component_fee_safe</a>(buy_fee_amount, config.trading_platform_fee_bps, total_fee_bps);
    <b>let</b> buy_treasury_fee = buy_fee_amount - buy_creator_fee - buy_platform_fee;
    <b>let</b> buy_net = buy_gross - buy_fee_amount;
    <b>let</b> <b>mut</b> payment = coin::from_balance(balance::split(&<b>mut</b> bridge, buy_gross), ctx);
    <b>if</b> (buy_fee_amount &gt; 0) {
        <b>if</b> (buy_creator_fee &gt; 0) {
            <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_distribute_creator_fee">distribute_creator_fee</a>(dest_pool, buy_creator_fee, &<b>mut</b> payment, ctx);
        };
        <b>if</b> (buy_platform_fee &gt; 0) {
            <b>let</b> <b>mut</b> c = coin::split(&<b>mut</b> payment, buy_platform_fee, ctx);
            <a href="../social_contracts/platform.md#social_contracts_platform_add_to_treasury">social_contracts::platform::add_to_treasury</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, &<b>mut</b> c, buy_platform_fee, clock, ctx);
            coin::destroy_zero(c);
        };
        <b>if</b> (buy_treasury_fee &gt; 0) {
            <b>let</b> c = coin::split(&<b>mut</b> payment, buy_treasury_fee, ctx);
            transfer::public_transfer(c, <a href="../social_contracts/profile.md#social_contracts_profile_get_treasury_address">profile::get_treasury_address</a>(treasury));
        };
    };
    <b>let</b> pool_payment = coin::split(&<b>mut</b> payment, buy_net, ctx);
    balance::join(&<b>mut</b> dest_pool.myso_balance, coin::into_balance(pool_payment));
    coin::destroy_zero(payment);
    <b>let</b> leftover_myso = balance::value(&bridge);
    <b>if</b> (leftover_myso &gt; 0) {
        transfer::public_transfer(coin::from_balance(bridge, ctx), trader);
    } <b>else</b> {
        balance::destroy_zero(bridge);
    };
    <b>assert</b>!(dest_pool.info.circulating_supply &lt;= <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_MAX_U64">MAX_U64</a> - dest_amount, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EOverflow">EOverflow</a>);
    <b>let</b> new_supply = dest_pool.info.circulating_supply + dest_amount;
    <b>assert</b>!(new_supply == 0 || new_supply &lt;= <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_MAX_U64">MAX_U64</a> / config.max_hold_percent_bps, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EOverflow">EOverflow</a>);
    <b>let</b> max_hold = (new_supply * config.max_hold_percent_bps) / <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_BPS_DENOM">BPS_DENOM</a>;
    <b>let</b> current_hold = <b>if</b> (table::contains(&dest_pool.holders, trader)) {
        *table::borrow(&dest_pool.holders, trader)
    } <b>else</b> {
        0
    };
    <b>assert</b>!(current_hold &lt;= <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_MAX_U64">MAX_U64</a> - dest_amount, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EOverflow">EOverflow</a>);
    <b>assert</b>!(current_hold + dest_amount &lt;= max_hold, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EExceededMaxHold">EExceededMaxHold</a>);
    <b>if</b> (credit_existing) {
        <b>assert</b>!(current_hold &gt; 0, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ENoTokensOwned">ENoTokensOwned</a>);
        *table::borrow_mut(&<b>mut</b> dest_pool.holders, trader) = current_hold + dest_amount;
    } <b>else</b> {
        <b>assert</b>!(current_hold == 0, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EAlreadyOwnsTokens">EAlreadyOwnsTokens</a>);
        table::add(&<b>mut</b> dest_pool.holders, trader, dest_amount);
        transfer::transfer(<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialToken">SocialToken</a> {
            id: object::new(ctx),
            pool_id: dest_pool_id,
            token_type: dest_pool.info.token_type,
            amount: dest_amount,
        }, trader);
    };
    dest_pool.info.circulating_supply = new_supply;
    <b>let</b> dest_new_price = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_token_price">calculate_token_price</a>(
        dest_pool.info.base_price,
        dest_pool.info.quadratic_coefficient,
        dest_pool.info.circulating_supply
    );
    event::emit(<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenBoughtEvent">TokenBoughtEvent</a> {
        id: dest_pool_id,
        buyer: trader,
        amount: dest_amount,
        myso_amount: buy_gross,
        fee_amount: buy_fee_amount,
        creator_fee: buy_creator_fee,
        platform_fee: buy_platform_fee,
        treasury_fee: buy_treasury_fee,
        new_price: dest_new_price,
    });
    event::emit(<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenSwappedEvent">TokenSwappedEvent</a> {
        source_pool_id,
        dest_pool_id,
        trader,
        sell_amount,
        dest_amount,
        sell_myso_gross: sell_gross,
        buy_myso_gross: buy_gross,
        sell_fee_amount,
        buy_fee_amount,
        sell_creator_fee,
        sell_platform_fee,
        sell_treasury_fee,
        buy_creator_fee,
        buy_platform_fee,
        buy_treasury_fee,
        leftover_myso,
        source_new_price,
        dest_new_price,
    });
    dest_amount
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_unwrap_u256_opt"></a>

## Function `unwrap_u256_opt`



<pre><code><b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_unwrap_u256_opt">unwrap_u256_opt</a>(o: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u256&gt;): u256
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_unwrap_u256_opt">unwrap_u256_opt</a>(o: Option&lt;u256&gt;): u256 {
    <b>assert</b>!(option::is_some(&o), <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EOverflow">EOverflow</a>);
    option::destroy_some(o)
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_u256_add_with_carry"></a>

## Function `u256_add_with_carry`

Returns <code>(sum mod 2^256, carry)</code> where carry is <code>0</code> or <code>1</code>.


<pre><code><b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_u256_add_with_carry">u256_add_with_carry</a>(a: u256, b: u256): (u256, u256)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_u256_add_with_carry">u256_add_with_carry</a>(a: u256, b: u256): (u256, u256) {
    <b>let</b> s = u256::checked_add(a, b);
    <b>if</b> (option::is_some(&s)) {
        (option::destroy_some(s), 0u256)
    } <b>else</b> {
        <b>let</b> max = u256::max_value!();
        <b>let</b> t = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_unwrap_u256_opt">unwrap_u256_opt</a>(u256::checked_sub(max, b));
        <b>let</b> u = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_unwrap_u256_opt">unwrap_u256_opt</a>(u256::checked_add(t, 1u256));
        <b>let</b> low = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_unwrap_u256_opt">unwrap_u256_opt</a>(u256::checked_sub(a, u));
        (low, 1u256)
    }
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_u256_mul_widen"></a>

## Function `u256_mul_widen`



<pre><code><b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_u256_mul_widen">u256_mul_widen</a>(x: u256, y: u256): (u256, u256)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_u256_mul_widen">u256_mul_widen</a>(x: u256, y: u256): (u256, u256) {
    <b>let</b> b128 = 1u256 &lt;&lt; 128;
    <b>let</b> mask = b128 - 1u256;
    <b>let</b> xl = x & mask;
    <b>let</b> xh = x &gt;&gt; 128;
    <b>let</b> yl = y & mask;
    <b>let</b> yh = y &gt;&gt; 128;
    <b>let</b> p0 = xl * yl;
    <b>let</b> p1 = xh * yl;
    <b>let</b> p2 = xl * yh;
    <b>let</b> p3 = xh * yh;
    <b>let</b> p1_lo = p1 & mask;
    <b>let</b> p1_hi = p1 &gt;&gt; 128;
    <b>let</b> p2_lo = p2 & mask;
    <b>let</b> p2_hi = p2 &gt;&gt; 128;
    <b>let</b> t_lo = p1_lo + p2_lo;
    <b>let</b> carry_tl = t_lo &gt;&gt; 128;
    <b>let</b> t_lo_final = t_lo & mask;
    <b>let</b> t_hi = p1_hi + p2_hi + carry_tl;
    <b>let</b> shift_part = t_lo_final * b128;
    <b>let</b> (w0, c0) = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_u256_add_with_carry">u256_add_with_carry</a>(p0, shift_part);
    <b>let</b> (t_sum, c1) = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_u256_add_with_carry">u256_add_with_carry</a>(p3, t_hi);
    <b>let</b> (hi_acc, c2) = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_u256_add_with_carry">u256_add_with_carry</a>(t_sum, c0);
    <b>assert</b>!(c1 == 0u256 && c2 == 0u256, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EOverflow">EOverflow</a>);
    (hi_acc, w0)
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_u512_bit"></a>

## Function `u512_bit`



<pre><code><b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_u512_bit">u512_bit</a>(n_hi: u256, n_lo: u256, i: u64): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_u512_bit">u512_bit</a>(n_hi: u256, n_lo: u256, i: u64): bool {
    <b>if</b> (i &lt; 256) {
        ((n_lo &gt;&gt; (i <b>as</b> u8)) & 1u256) != 0u256
    } <b>else</b> {
        <b>let</b> j = i - 256;
        ((n_hi &gt;&gt; (j <b>as</b> u8)) & 1u256) != 0u256
    }
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_u512_shl1_or_bit"></a>

## Function `u512_shl1_or_bit`



<pre><code><b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_u512_shl1_or_bit">u512_shl1_or_bit</a>(r_hi: u256, r_lo: u256, bit: bool): (u256, u256)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_u512_shl1_or_bit">u512_shl1_or_bit</a>(r_hi: u256, r_lo: u256, bit: bool): (u256, u256) {
    <b>let</b> carry = r_lo &gt;&gt; 255;
    <b>let</b> nl_lo = (r_lo &lt;&lt; 1) | (<b>if</b> (bit) { 1u256 } <b>else</b> { 0u256 });
    <b>let</b> nl_hi = (r_hi &lt;&lt; 1) | carry;
    (nl_hi, nl_lo)
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_u512_ge_u256"></a>

## Function `u512_ge_u256`



<pre><code><b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_u512_ge_u256">u512_ge_u256</a>(r_hi: u256, r_lo: u256, d: u256): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_u512_ge_u256">u512_ge_u256</a>(r_hi: u256, r_lo: u256, d: u256): bool {
    r_hi &gt; 0u256 || r_lo &gt;= d
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_u512_sub_u256"></a>

## Function `u512_sub_u256`



<pre><code><b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_u512_sub_u256">u512_sub_u256</a>(r_hi: u256, r_lo: u256, d: u256): (u256, u256)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_u512_sub_u256">u512_sub_u256</a>(r_hi: u256, r_lo: u256, d: u256): (u256, u256) {
    <b>let</b> sub_lo = u256::checked_sub(r_lo, d);
    <b>if</b> (option::is_some(&sub_lo)) {
        (r_hi, option::destroy_some(sub_lo))
    } <b>else</b> {
        <b>let</b> new_hi = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_unwrap_u256_opt">unwrap_u256_opt</a>(u256::checked_sub(r_hi, 1u256));
        <b>let</b> bump = u256::max_value!() - d + 1u256;
        <b>let</b> new_lo = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_unwrap_u256_opt">unwrap_u256_opt</a>(u256::checked_add(r_lo, bump));
        (new_hi, new_lo)
    }
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_u512_div_u256_floor"></a>

## Function `u512_div_u256_floor`

<code>floor((n_hi*2^256 + n_lo) / d)</code> for <code>d &gt; 0</code>.


<pre><code><b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_u512_div_u256_floor">u512_div_u256_floor</a>(n_hi: u256, n_lo: u256, d: u256): u256
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_u512_div_u256_floor">u512_div_u256_floor</a>(n_hi: u256, n_lo: u256, d: u256): u256 {
    <b>assert</b>!(d &gt; 0u256, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInvalidCurveParams">EInvalidCurveParams</a>);
    <b>if</b> (n_hi == 0u256) {
        <b>return</b> n_lo / d
    };
    <b>let</b> <b>mut</b> r_hi = 0u256;
    <b>let</b> <b>mut</b> r_lo = 0u256;
    <b>let</b> <b>mut</b> q = 0u256;
    <b>let</b> <b>mut</b> i = 512u64;
    <b>while</b> (i &gt; 0) {
        i = i - 1;
        <b>let</b> bit = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_u512_bit">u512_bit</a>(n_hi, n_lo, i);
        <b>let</b> (nr_hi, nr_lo) = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_u512_shl1_or_bit">u512_shl1_or_bit</a>(r_hi, r_lo, bit);
        r_hi = nr_hi;
        r_lo = nr_lo;
        <b>if</b> (<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_u512_ge_u256">u512_ge_u256</a>(r_hi, r_lo, d)) {
            <b>let</b> (sr_hi, sr_lo) = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_u512_sub_u256">u512_sub_u256</a>(r_hi, r_lo, d);
            r_hi = sr_hi;
            r_lo = sr_lo;
            <b>let</b> inc = 1u256 &lt;&lt; (i <b>as</b> u8);
            q = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_unwrap_u256_opt">unwrap_u256_opt</a>(u256::checked_add(q, inc));
        };
    };
    q
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_quad_poly_buy"></a>

## Function `quad_poly_buy`

<code>3*s*s + 3*s*a + a*a</code> for buy integral <code>(s+a)^3 - s^3 = a * poly</code>.


<pre><code><b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_quad_poly_buy">quad_poly_buy</a>(s: u256, a: u256): u256
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_quad_poly_buy">quad_poly_buy</a>(s: u256, a: u256): u256 {
    <b>let</b> s2 = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_unwrap_u256_opt">unwrap_u256_opt</a>(u256::checked_mul(s, s));
    <b>let</b> three_s2 = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_unwrap_u256_opt">unwrap_u256_opt</a>(u256::checked_mul(3u256, s2));
    <b>let</b> sa = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_unwrap_u256_opt">unwrap_u256_opt</a>(u256::checked_mul(s, a));
    <b>let</b> three_sa = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_unwrap_u256_opt">unwrap_u256_opt</a>(u256::checked_mul(3u256, sa));
    <b>let</b> a2 = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_unwrap_u256_opt">unwrap_u256_opt</a>(u256::checked_mul(a, a));
    <b>let</b> t = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_unwrap_u256_opt">unwrap_u256_opt</a>(u256::checked_add(three_s2, three_sa));
    <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_unwrap_u256_opt">unwrap_u256_opt</a>(u256::checked_add(t, a2))
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_quad_poly_sell"></a>

## Function `quad_poly_sell`

<code>3*s*s - 3*s*a + a*a</code> for sell integral <code>s^3 - (s-a)^3 = a * poly</code>.


<pre><code><b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_quad_poly_sell">quad_poly_sell</a>(s: u256, a: u256): u256
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_quad_poly_sell">quad_poly_sell</a>(s: u256, a: u256): u256 {
    <b>let</b> s2 = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_unwrap_u256_opt">unwrap_u256_opt</a>(u256::checked_mul(s, s));
    <b>let</b> three_s2 = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_unwrap_u256_opt">unwrap_u256_opt</a>(u256::checked_mul(3u256, s2));
    <b>let</b> sa = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_unwrap_u256_opt">unwrap_u256_opt</a>(u256::checked_mul(s, a));
    <b>let</b> three_sa = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_unwrap_u256_opt">unwrap_u256_opt</a>(u256::checked_mul(3u256, sa));
    <b>let</b> a2 = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_unwrap_u256_opt">unwrap_u256_opt</a>(u256::checked_mul(a, a));
    <b>let</b> t = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_unwrap_u256_opt">unwrap_u256_opt</a>(u256::checked_sub(three_s2, three_sa));
    <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_unwrap_u256_opt">unwrap_u256_opt</a>(u256::checked_add(t, a2))
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_quad_integral_leg_mist"></a>

## Function `quad_integral_leg_mist`



<pre><code><b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_quad_integral_leg_mist">quad_integral_leg_mist</a>(coeff: u256, s: u256, a: u256, scale: u256, is_buy: bool): u256
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_quad_integral_leg_mist">quad_integral_leg_mist</a>(coeff: u256, s: u256, a: u256, scale: u256, is_buy: bool): u256 {
    <b>let</b> poly = <b>if</b> (is_buy) {
        <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_quad_poly_buy">quad_poly_buy</a>(s, a)
    } <b>else</b> {
        <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_quad_poly_sell">quad_poly_sell</a>(s, a)
    };
    <b>let</b> ca = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_unwrap_u256_opt">unwrap_u256_opt</a>(u256::checked_mul(coeff, a));
    <b>let</b> (numer_hi, numer_lo) = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_u256_mul_widen">u256_mul_widen</a>(ca, poly);
    <b>let</b> denom = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_unwrap_u256_opt">unwrap_u256_opt</a>(u256::checked_mul(
        30000u256,
        <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_unwrap_u256_opt">unwrap_u256_opt</a>(u256::checked_mul(scale, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_unwrap_u256_opt">unwrap_u256_opt</a>(u256::checked_mul(scale, scale))))
    ));
    <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_u512_div_u256_floor">u512_div_u256_floor</a>(numer_hi, numer_lo, denom)
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_mist_amount_u256_to_u64"></a>

## Function `mist_amount_u256_to_u64`

MYSO amounts on-chain are <code>u64</code> (smallest units). Abort <code><a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EOverflow">EOverflow</a></code> if <code>x</code> does not fit.


<pre><code><b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_mist_amount_u256_to_u64">mist_amount_u256_to_u64</a>(x: u256): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_mist_amount_u256_to_u64">mist_amount_u256_to_u64</a>(x: u256): u64 {
    <b>let</b> o = u256::try_as_u64(x);
    <b>assert</b>!(option::is_some(&o), <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EOverflow">EOverflow</a>);
    option::destroy_some(o)
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_calculate_token_price"></a>

## Function `calculate_token_price`

Marginal MYSO price for the next infinitesimal nano-SPT at <code>supply_nano</code> (nano-SPT in pool).
<code>p(s) = base_price + quadratic_coefficient * (s / <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SPT_SCALE">SPT_SCALE</a>)^2 / <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_BPS_DENOM">BPS_DENOM</a></code> (permyriad).


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_token_price">calculate_token_price</a>(base_price: u64, quadratic_coefficient: u64, supply_nano: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_token_price">calculate_token_price</a>(
    base_price: u64,
    quadratic_coefficient: u64,
    supply_nano: u64
): u64 {
    <b>let</b> base = base_price <b>as</b> u256;
    <b>let</b> coeff = quadratic_coefficient <b>as</b> u256;
    <b>let</b> s = supply_nano <b>as</b> u256;
    <b>let</b> scale = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SPT_SCALE">SPT_SCALE</a> <b>as</b> u256;
    <b>let</b> scale2 = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_unwrap_u256_opt">unwrap_u256_opt</a>(u256::checked_mul(scale, scale));
    <b>let</b> denom = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_unwrap_u256_opt">unwrap_u256_opt</a>(u256::checked_mul(scale2, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_BPS_DENOM">BPS_DENOM</a> <b>as</b> u256));
    <b>assert</b>!(denom &gt; 0u256, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInvalidCurveParams">EInvalidCurveParams</a>);
    <b>let</b> s2 = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_unwrap_u256_opt">unwrap_u256_opt</a>(u256::checked_mul(s, s));
    <b>let</b> coeff_s2 = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_unwrap_u256_opt">unwrap_u256_opt</a>(u256::checked_mul(coeff, s2));
    <b>let</b> quad = coeff_s2 / denom;
    <b>let</b> total = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_unwrap_u256_opt">unwrap_u256_opt</a>(u256::checked_add(base, quad));
    <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_mist_amount_u256_to_u64">mist_amount_u256_to_u64</a>(total)
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_calculate_buy_price"></a>

## Function `calculate_buy_price`

Total MYSO cost to buy <code>amount_nano</code> nano-SPT when current circulating supply is <code>current_supply_nano</code>.
Uses the closed-form integral of the marginal quadratic curve over human supply
(continuous approximation; <code>amount</code> and <code>supply</code> are nano-SPT).
Returns <code>(total_mysos, avg_mysos_per_nano_unit)</code>.


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_buy_price">calculate_buy_price</a>(base_price: u64, quadratic_coefficient: u64, current_supply_nano: u64, amount_nano: u64): (u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_buy_price">calculate_buy_price</a>(
    base_price: u64,
    quadratic_coefficient: u64,
    current_supply_nano: u64,
    amount_nano: u64
): (u64, u64) {
    <b>if</b> (amount_nano == 0) {
        <b>return</b> (0, 0)
    };
    <b>let</b> base = base_price <b>as</b> u256;
    <b>let</b> coeff = quadratic_coefficient <b>as</b> u256;
    <b>let</b> s = current_supply_nano <b>as</b> u256;
    <b>let</b> a = amount_nano <b>as</b> u256;
    <b>let</b> scale = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SPT_SCALE">SPT_SCALE</a> <b>as</b> u256;
    <b>let</b> base_prod = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_unwrap_u256_opt">unwrap_u256_opt</a>(u256::checked_mul(base, a));
    <b>let</b> base_part = base_prod / scale;
    <b>let</b> quad_part = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_quad_integral_leg_mist">quad_integral_leg_mist</a>(coeff, s, a, scale, <b>true</b>);
    <b>let</b> total = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_unwrap_u256_opt">unwrap_u256_opt</a>(u256::checked_add(base_part, quad_part));
    <b>let</b> total_u64 = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_mist_amount_u256_to_u64">mist_amount_u256_to_u64</a>(total);
    <b>let</b> avg_u64 = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_mist_amount_u256_to_u64">mist_amount_u256_to_u64</a>(total / a);
    (total_u64, avg_u64)
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_calculate_sell_price"></a>

## Function `calculate_sell_price`

MYSO refund for selling <code>amount_nano</code> nano-SPT when current circulating supply is <code>current_supply_nano</code>.
Returns <code>(total_refund_mysos, avg_mysos_per_nano_unit)</code>.


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_sell_price">calculate_sell_price</a>(base_price: u64, quadratic_coefficient: u64, current_supply_nano: u64, amount_nano: u64): (u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_sell_price">calculate_sell_price</a>(
    base_price: u64,
    quadratic_coefficient: u64,
    current_supply_nano: u64,
    amount_nano: u64
): (u64, u64) {
    <b>if</b> (amount_nano == 0) {
        <b>return</b> (0, 0)
    };
    <b>assert</b>!(current_supply_nano &gt;= amount_nano, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInsufficientLiquidity">EInsufficientLiquidity</a>);
    <b>let</b> base = base_price <b>as</b> u256;
    <b>let</b> coeff = quadratic_coefficient <b>as</b> u256;
    <b>let</b> s = current_supply_nano <b>as</b> u256;
    <b>let</b> a = amount_nano <b>as</b> u256;
    <b>let</b> scale = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SPT_SCALE">SPT_SCALE</a> <b>as</b> u256;
    <b>let</b> base_prod = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_unwrap_u256_opt">unwrap_u256_opt</a>(u256::checked_mul(base, a));
    <b>let</b> base_part = base_prod / scale;
    <b>let</b> quad_part = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_quad_integral_leg_mist">quad_integral_leg_mist</a>(coeff, s, a, scale, <b>false</b>);
    <b>let</b> total = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_unwrap_u256_opt">unwrap_u256_opt</a>(u256::checked_add(base_part, quad_part));
    <b>let</b> total_u64 = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_mist_amount_u256_to_u64">mist_amount_u256_to_u64</a>(total);
    <b>let</b> avg_u64 = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_mist_amount_u256_to_u64">mist_amount_u256_to_u64</a>(total / a);
    (total_u64, avg_u64)
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_calculate_swap_proceeds"></a>

## Function `calculate_swap_proceeds`

Gross MYSO proceeds, sell fee, and net MYSO after selling <code>sell_amount</code> nano-SPT.


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_swap_proceeds">calculate_swap_proceeds</a>(base_price: u64, quadratic_coefficient: u64, current_supply_nano: u64, sell_amount: u64, total_fee_bps: u64): (u64, u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_swap_proceeds">calculate_swap_proceeds</a>(
    base_price: u64,
    quadratic_coefficient: u64,
    current_supply_nano: u64,
    sell_amount: u64,
    total_fee_bps: u64
): (u64, u64, u64) {
    <b>let</b> (sell_gross, _) = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_sell_price">calculate_sell_price</a>(
        base_price,
        quadratic_coefficient,
        current_supply_nano,
        sell_amount
    );
    <b>let</b> sell_fee = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_fee_amount_safe">calculate_fee_amount_safe</a>(sell_gross, total_fee_bps);
    <b>assert</b>!(sell_gross &gt;= sell_fee, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EOverflow">EOverflow</a>);
    (sell_gross, sell_fee, sell_gross - sell_fee)
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_calculate_max_buy_amount"></a>

## Function `calculate_max_buy_amount`

Largest nano-SPT buy whose gross MYSO cost is <code>&lt;= myso_budget</code> (binary search).
Returns <code>(dest_amount, buy_gross)</code>.


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_max_buy_amount">calculate_max_buy_amount</a>(base_price: u64, quadratic_coefficient: u64, current_supply_nano: u64, myso_budget: u64): (u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_max_buy_amount">calculate_max_buy_amount</a>(
    base_price: u64,
    quadratic_coefficient: u64,
    current_supply_nano: u64,
    myso_budget: u64
): (u64, u64) {
    <b>if</b> (myso_budget == 0) {
        <b>return</b> (0, 0)
    };
    <b>let</b> (cost1, _) = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_buy_price">calculate_buy_price</a>(
        base_price,
        quadratic_coefficient,
        current_supply_nano,
        1
    );
    <b>if</b> (cost1 &gt; myso_budget) {
        <b>return</b> (0, 0)
    };
    // Expand upper bound until cost exceeds budget.
    <b>let</b> <b>mut</b> lo: u64 = 1;
    <b>let</b> <b>mut</b> lo_cost: u64 = cost1;
    <b>let</b> <b>mut</b> hi: u64 = 2;
    <b>let</b> <b>mut</b> guard = 0u64;
    <b>while</b> (guard &lt; 63) {
        <b>let</b> (cost, _) = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_buy_price">calculate_buy_price</a>(
            base_price,
            quadratic_coefficient,
            current_supply_nano,
            hi
        );
        <b>if</b> (cost &gt; myso_budget) {
            <b>break</b>
        };
        lo = hi;
        lo_cost = cost;
        <b>if</b> (hi &gt; <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_MAX_U64">MAX_U64</a> / 2) {
            <b>return</b> (lo, lo_cost)
        };
        hi = hi * 2;
        guard = guard + 1;
    };
    // Binary search in (lo, hi)
    <b>while</b> (lo + 1 &lt; hi) {
        <b>let</b> mid = lo + (hi - lo) / 2;
        <b>let</b> (cost, _) = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_buy_price">calculate_buy_price</a>(
            base_price,
            quadratic_coefficient,
            current_supply_nano,
            mid
        );
        <b>if</b> (cost &lt;= myso_budget) {
            lo = mid;
            lo_cost = cost;
        } <b>else</b> {
            hi = mid;
        };
    };
    (lo, lo_cost)
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_calculate_swap_quote"></a>

## Function `calculate_swap_quote`

Quote an exact-in swap: sell <code>sell_amount</code> from source curve into dest curve.
Returns <code>(dest_amount, sell_gross, buy_gross, net_bridge, leftover_myso)</code>.


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_swap_quote">calculate_swap_quote</a>(source_base_price: u64, source_quadratic_coefficient: u64, source_supply_nano: u64, dest_base_price: u64, dest_quadratic_coefficient: u64, dest_supply_nano: u64, sell_amount: u64, total_fee_bps: u64): (u64, u64, u64, u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_swap_quote">calculate_swap_quote</a>(
    source_base_price: u64,
    source_quadratic_coefficient: u64,
    source_supply_nano: u64,
    dest_base_price: u64,
    dest_quadratic_coefficient: u64,
    dest_supply_nano: u64,
    sell_amount: u64,
    total_fee_bps: u64
): (u64, u64, u64, u64, u64) {
    <b>let</b> (sell_gross, _sell_fee, net_bridge) = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_swap_proceeds">calculate_swap_proceeds</a>(
        source_base_price,
        source_quadratic_coefficient,
        source_supply_nano,
        sell_amount,
        total_fee_bps
    );
    <b>let</b> (dest_amount, buy_gross) = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_max_buy_amount">calculate_max_buy_amount</a>(
        dest_base_price,
        dest_quadratic_coefficient,
        dest_supply_nano,
        net_bridge
    );
    <b>assert</b>!(net_bridge &gt;= buy_gross, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EOverflow">EOverflow</a>);
    (dest_amount, sell_gross, buy_gross, net_bridge, net_bridge - buy_gross)
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_spt_amount_scale"></a>

## Function `spt_amount_scale`

<code>10^9</code> nano-SPT per 1.0 display token (for clients / indexers).


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_spt_amount_scale">spt_amount_scale</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_spt_amount_scale">spt_amount_scale</a>(): u64 {
    <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SPT_SCALE">SPT_SCALE</a>
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_spt_amount_decimals"></a>

## Function `spt_amount_decimals`

Display decimals for SPT quantities (matches native MYSO).


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_spt_amount_decimals">spt_amount_decimals</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_spt_amount_decimals">spt_amount_decimals</a>(): u8 {
    <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SPT_DECIMALS">SPT_DECIMALS</a>
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_nano_spt_from_whole_tokens"></a>

## Function `nano_spt_from_whole_tokens`

Whole display tokens → nano-SPT (<code>whole * <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SPT_SCALE">SPT_SCALE</a></code>). Aborts with <code><a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EOverflow">EOverflow</a></code> if the product does not fit <code>u64</code>.


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_nano_spt_from_whole_tokens">nano_spt_from_whole_tokens</a>(whole: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_nano_spt_from_whole_tokens">nano_spt_from_whole_tokens</a>(whole: u64): u64 {
    <b>let</b> p = (whole <b>as</b> u128) * (<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SPT_SCALE">SPT_SCALE</a> <b>as</b> u128);
    <b>assert</b>!(p &lt;= <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_MAX_ONCHAIN_U64_U128">MAX_ONCHAIN_U64_U128</a>, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EOverflow">EOverflow</a>);
    p <b>as</b> u64
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_nano_spt_from_whole_and_fraction"></a>

## Function `nano_spt_from_whole_and_fraction`

<code>whole</code> display tokens plus <code>fraction_nano</code> nano-SPT remainder (<code>fraction_nano &lt; <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SPT_SCALE">SPT_SCALE</a></code>).


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_nano_spt_from_whole_and_fraction">nano_spt_from_whole_and_fraction</a>(whole: u64, fraction_nano: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_nano_spt_from_whole_and_fraction">nano_spt_from_whole_and_fraction</a>(whole: u64, fraction_nano: u64): u64 {
    <b>assert</b>!(fraction_nano &lt; <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SPT_SCALE">SPT_SCALE</a>, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInvalidCurveParams">EInvalidCurveParams</a>);
    <b>let</b> w = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_nano_spt_from_whole_tokens">nano_spt_from_whole_tokens</a>(whole);
    <b>assert</b>!(w &lt;= <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_MAX_U64">MAX_U64</a> - fraction_nano, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EOverflow">EOverflow</a>);
    w + fraction_nano
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_get_token_info"></a>

## Function `get_token_info`

Get token info from registry by associated_id (post/profile ID), not pool ID
Returns a reference since TokenInfo no longer has copy ability


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_get_token_info">get_token_info</a>(registry: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">social_contracts::social_proof_tokens::TokenRegistry</a>, id: <b>address</b>): &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenInfo">social_contracts::social_proof_tokens::TokenInfo</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_get_token_info">get_token_info</a>(registry: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">TokenRegistry</a>, id: <b>address</b>): &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenInfo">TokenInfo</a> {
    <b>assert</b>!(table::contains(&registry.tokens, id), <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ETokenNotFound">ETokenNotFound</a>);
    table::borrow(&registry.tokens, id)
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_token_info_circulating_supply"></a>

## Function `token_info_circulating_supply`

Circulating supply (nano-SPT) from a <code><a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenInfo">TokenInfo</a></code> reference.


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_token_info_circulating_supply">token_info_circulating_supply</a>(info: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenInfo">social_contracts::social_proof_tokens::TokenInfo</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_token_info_circulating_supply">token_info_circulating_supply</a>(info: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenInfo">TokenInfo</a>): u64 {
    info.circulating_supply
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_token_exists"></a>

## Function `token_exists`

Check if a token exists in the registry


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_token_exists">token_exists</a>(registry: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">social_contracts::social_proof_tokens::TokenRegistry</a>, id: <b>address</b>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_token_exists">token_exists</a>(registry: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">TokenRegistry</a>, id: <b>address</b>): bool {
    table::contains(&registry.tokens, id)
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_get_token_owner"></a>

## Function `get_token_owner`

Get token owner's address


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_get_token_owner">get_token_owner</a>(registry: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">social_contracts::social_proof_tokens::TokenRegistry</a>, id: <b>address</b>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_get_token_owner">get_token_owner</a>(registry: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">TokenRegistry</a>, id: <b>address</b>): <b>address</b> {
    <b>let</b> info = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_get_token_info">get_token_info</a>(registry, id);
    info.owner
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_get_pool_price"></a>

## Function `get_pool_price`

Get current token price for a specific pool


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_get_pool_price">get_pool_price</a>(pool: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">social_contracts::social_proof_tokens::TokenPool</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_get_pool_price">get_pool_price</a>(pool: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">TokenPool</a>): u64 {
    <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_calculate_token_price">calculate_token_price</a>(
        pool.info.base_price,
        pool.info.quadratic_coefficient,
        pool.info.circulating_supply
    )
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_get_user_balance"></a>

## Function `get_user_balance`

Get user's token balance


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_get_user_balance">get_user_balance</a>(pool: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">social_contracts::social_proof_tokens::TokenPool</a>, user: <b>address</b>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_get_user_balance">get_user_balance</a>(pool: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">TokenPool</a>, user: <b>address</b>): u64 {
    <b>if</b> (table::contains(&pool.holders, user)) {
        *table::borrow(&pool.holders, user)
    } <b>else</b> {
        0
    }
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_get_revenue_manifest"></a>

## Function `get_revenue_manifest`

Get cached revenue manifest from token pool


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_get_revenue_manifest">get_revenue_manifest</a>(pool: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">social_contracts::social_proof_tokens::TokenPool</a>): &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../social_contracts/media_asset.md#social_contracts_media_asset_RevenueManifest">social_contracts::media_asset::RevenueManifest</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_get_revenue_manifest">get_revenue_manifest</a>(pool: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">TokenPool</a>): &Option&lt;RevenueManifest&gt; {
    &pool.revenue_manifest
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_has_poc_redirection"></a>

## Function `has_poc_redirection`

Legacy alias — true when a cached revenue manifest is present.


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_has_poc_redirection">has_poc_redirection</a>(pool: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">social_contracts::social_proof_tokens::TokenPool</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_has_poc_redirection">has_poc_redirection</a>(pool: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">TokenPool</a>): bool {
    option::is_some(&pool.revenue_manifest)
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_get_poc_redirect_to"></a>

## Function `get_poc_redirect_to`

Legacy compat — first non-owner manifest beneficiary.


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_get_poc_redirect_to">get_poc_redirect_to</a>(pool: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">social_contracts::social_proof_tokens::TokenPool</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_get_poc_redirect_to">get_poc_redirect_to</a>(pool: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">TokenPool</a>): Option&lt;<b>address</b>&gt; {
    <b>if</b> (option::is_none(&pool.revenue_manifest)) {
        <b>return</b> option::none()
    };
    <a href="../social_contracts/media_asset.md#social_contracts_media_asset_manifest_redirect_beneficiary">media_asset::manifest_redirect_beneficiary</a>(
        option::borrow(&pool.revenue_manifest),
        pool.info.owner,
    )
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_get_poc_redirect_percentage"></a>

## Function `get_poc_redirect_percentage`

Legacy compat — redirect share as whole-number percent (e.g. 75 = 75%).


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_get_poc_redirect_percentage">get_poc_redirect_percentage</a>(pool: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">social_contracts::social_proof_tokens::TokenPool</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_get_poc_redirect_percentage">get_poc_redirect_percentage</a>(pool: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">TokenPool</a>): Option&lt;u64&gt; {
    <b>if</b> (option::is_none(&pool.revenue_manifest)) {
        <b>return</b> option::none()
    };
    <a href="../social_contracts/media_asset.md#social_contracts_media_asset_manifest_redirect_percentage">media_asset::manifest_redirect_percentage</a>(
        option::borrow(&pool.revenue_manifest),
        pool.info.owner,
    )
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_get_pool_associated_id"></a>

## Function `get_pool_associated_id`

Get the associated ID (post/profile ID) from a token pool


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_get_pool_associated_id">get_pool_associated_id</a>(pool: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">social_contracts::social_proof_tokens::TokenPool</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_get_pool_associated_id">get_pool_associated_id</a>(pool: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">TokenPool</a>): <b>address</b> {
    pool.info.associated_id
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_set_revenue_manifest"></a>

## Function `set_revenue_manifest`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_set_revenue_manifest">set_revenue_manifest</a>(pool: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">social_contracts::social_proof_tokens::TokenPool</a>, manifest: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../social_contracts/media_asset.md#social_contracts_media_asset_RevenueManifest">social_contracts::media_asset::RevenueManifest</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_set_revenue_manifest">set_revenue_manifest</a>(pool: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">TokenPool</a>, manifest: Option&lt;RevenueManifest&gt;) {
    pool.revenue_manifest = manifest;
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_set_poc_redirection"></a>

## Function `set_poc_redirection`

Build or clear cached manifest from legacy redirect parameters.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_set_poc_redirection">set_poc_redirection</a>(pool: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">social_contracts::social_proof_tokens::TokenPool</a>, redirect_to: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;, redirect_percentage: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, poc_redirection_kind: u8)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_set_poc_redirection">set_poc_redirection</a>(
    pool: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">TokenPool</a>,
    redirect_to: Option&lt;<b>address</b>&gt;,
    redirect_percentage: Option&lt;u64&gt;,
    poc_redirection_kind: u8,
) {
    <b>if</b> (poc_redirection_kind == 0) {
        pool.revenue_manifest = option::none();
        <b>return</b>
    };
    <b>assert</b>!(
        option::is_some(&redirect_to) && option::is_some(&redirect_percentage),
        <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInvalidFeeConfig">EInvalidFeeConfig</a>,
    );
    <b>let</b> redirect_to_addr = *option::borrow(&redirect_to);
    <b>let</b> redirect_pct = *option::borrow(&redirect_percentage);
    <b>assert</b>!(redirect_pct &gt; 0 && redirect_pct &lt;= 100, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInvalidFeeConfig">EInvalidFeeConfig</a>);
    <b>let</b> redirect_bps = redirect_pct * 100;
    <b>let</b> owner_bps = <a href="../social_contracts/media_asset.md#social_contracts_media_asset_manifest_bps_total">media_asset::manifest_bps_total</a>() - redirect_bps;
    <b>let</b> owner = pool.info.owner;
    <b>let</b> redirect_payout_mode = <b>if</b> (poc_redirection_kind == 2) {
        <a href="../social_contracts/media_asset.md#social_contracts_media_asset_payout_escrow">media_asset::payout_escrow</a>()
    } <b>else</b> {
        <a href="../social_contracts/media_asset.md#social_contracts_media_asset_payout_wallet">media_asset::payout_wallet</a>()
    };
    <b>let</b> <b>mut</b> entries = vector[];
    <b>if</b> (redirect_bps &gt; 0) {
        vector::push_back(
            &<b>mut</b> entries,
            <a href="../social_contracts/media_asset.md#social_contracts_media_asset_new_manifest_entry">media_asset::new_manifest_entry</a>(
                redirect_to_addr,
                redirect_bps,
                redirect_payout_mode,
            ),
        );
    };
    <b>if</b> (owner_bps &gt; 0) {
        vector::push_back(
            &<b>mut</b> entries,
            <a href="../social_contracts/media_asset.md#social_contracts_media_asset_new_manifest_entry">media_asset::new_manifest_entry</a>(
                owner,
                owner_bps,
                <a href="../social_contracts/media_asset.md#social_contracts_media_asset_payout_wallet">media_asset::payout_wallet</a>(),
            ),
        );
    };
    <b>let</b> manifest = <a href="../social_contracts/media_asset.md#social_contracts_media_asset_new_revenue_manifest">media_asset::new_revenue_manifest</a>(entries);
    <a href="../social_contracts/media_asset.md#social_contracts_media_asset_validate_revenue_manifest">media_asset::validate_revenue_manifest</a>(&manifest);
    pool.revenue_manifest = option::some(manifest);
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_set_poc_redirection_entry"></a>

## Function `set_poc_redirection_entry`

Entry function to set PoC redirection (requires pool owner)


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_set_poc_redirection_entry">set_poc_redirection_entry</a>(registry: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">social_contracts::social_proof_tokens::TokenRegistry</a>, pool: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">social_contracts::social_proof_tokens::TokenPool</a>, redirect_to: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;, redirect_percentage: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, poc_redirection_kind: u8, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_set_poc_redirection_entry">set_poc_redirection_entry</a>(
    registry: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">TokenRegistry</a>,
    pool: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">TokenPool</a>,
    redirect_to: Option&lt;<b>address</b>&gt;,
    redirect_percentage: Option&lt;u64&gt;,
    poc_redirection_kind: u8,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> caller = tx_context::sender(ctx);
    <b>let</b> token_info = <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_get_token_info">get_token_info</a>(registry, pool.info.associated_id);
    // Require caller to be pool owner
    <b>assert</b>!(caller == token_info.owner, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ENotAuthorized">ENotAuthorized</a>);
    <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_set_poc_redirection">set_poc_redirection</a>(pool, redirect_to, redirect_percentage, poc_redirection_kind);
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_set_poc_redirection_admin"></a>

## Function `set_poc_redirection_admin`

Admin entry function to set PoC redirection (requires admin cap)


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_set_poc_redirection_admin">set_poc_redirection_admin</a>(_registry: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">social_contracts::social_proof_tokens::TokenRegistry</a>, pool: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">social_contracts::social_proof_tokens::TokenPool</a>, _admin_cap: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensAdminCap">social_contracts::social_proof_tokens::SocialProofTokensAdminCap</a>, redirect_to: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;, redirect_percentage: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, poc_redirection_kind: u8, _ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_set_poc_redirection_admin">set_poc_redirection_admin</a>(
    _registry: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">TokenRegistry</a>,
    pool: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">TokenPool</a>,
    _admin_cap: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensAdminCap">SocialProofTokensAdminCap</a>,
    redirect_to: Option&lt;<b>address</b>&gt;,
    redirect_percentage: Option&lt;u64&gt;,
    poc_redirection_kind: u8,
    _ctx: &<b>mut</b> TxContext
) {
    // Admin can set redirection <b>for</b> any pool
    <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_set_poc_redirection">set_poc_redirection</a>(pool, redirect_to, redirect_percentage, poc_redirection_kind);
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_clear_poc_redirection"></a>

## Function `clear_poc_redirection`

Clear cached revenue manifest from a token pool.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_clear_poc_redirection">clear_poc_redirection</a>(pool: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">social_contracts::social_proof_tokens::TokenPool</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_clear_poc_redirection">clear_poc_redirection</a>(pool: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">TokenPool</a>) {
    pool.revenue_manifest = option::none();
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_registry_version"></a>

## Function `registry_version`

Get the version of the token registry


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_registry_version">registry_version</a>(registry: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">social_contracts::social_proof_tokens::TokenRegistry</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_registry_version">registry_version</a>(registry: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">TokenRegistry</a>): u64 {
    registry.version
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_borrow_registry_version_mut"></a>

## Function `borrow_registry_version_mut`

Get a mutable reference to the registry version (for upgrade module)


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_borrow_registry_version_mut">borrow_registry_version_mut</a>(registry: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">social_contracts::social_proof_tokens::TokenRegistry</a>): &<b>mut</b> u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_borrow_registry_version_mut">borrow_registry_version_mut</a>(registry: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">TokenRegistry</a>): &<b>mut</b> u64 {
    &<b>mut</b> registry.version
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_pool_version"></a>

## Function `pool_version`

Get the version of a token pool


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_pool_version">pool_version</a>(pool: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">social_contracts::social_proof_tokens::TokenPool</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_pool_version">pool_version</a>(pool: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">TokenPool</a>): u64 {
    pool.version
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_borrow_pool_version_mut"></a>

## Function `borrow_pool_version_mut`

Get a mutable reference to the pool version (for upgrade module)


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_borrow_pool_version_mut">borrow_pool_version_mut</a>(pool: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">social_contracts::social_proof_tokens::TokenPool</a>): &<b>mut</b> u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_borrow_pool_version_mut">borrow_pool_version_mut</a>(pool: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">TokenPool</a>): &<b>mut</b> u64 {
    &<b>mut</b> pool.version
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_reservation_pool_version"></a>

## Function `reservation_pool_version`

Get the version of a reservation pool


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_reservation_pool_version">reservation_pool_version</a>(pool: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ReservationPoolObject">social_contracts::social_proof_tokens::ReservationPoolObject</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_reservation_pool_version">reservation_pool_version</a>(pool: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ReservationPoolObject">ReservationPoolObject</a>): u64 {
    pool.version
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_borrow_reservation_pool_version_mut"></a>

## Function `borrow_reservation_pool_version_mut`

Get a mutable reference to the reservation pool version (for upgrade module)


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_borrow_reservation_pool_version_mut">borrow_reservation_pool_version_mut</a>(pool: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ReservationPoolObject">social_contracts::social_proof_tokens::ReservationPoolObject</a>): &<b>mut</b> u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_borrow_reservation_pool_version_mut">borrow_reservation_pool_version_mut</a>(pool: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ReservationPoolObject">ReservationPoolObject</a>): &<b>mut</b> u64 {
    &<b>mut</b> pool.version
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_config_version"></a>

## Function `config_version`

Get the version of the social proof tokens config


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_config_version">config_version</a>(config: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">social_contracts::social_proof_tokens::SocialProofTokensConfig</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_config_version">config_version</a>(config: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">SocialProofTokensConfig</a>): u64 {
    config.version
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_borrow_config_version_mut"></a>

## Function `borrow_config_version_mut`

Get a mutable reference to the config version (for upgrade module)


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_borrow_config_version_mut">borrow_config_version_mut</a>(config: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">social_contracts::social_proof_tokens::SocialProofTokensConfig</a>): &<b>mut</b> u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_borrow_config_version_mut">borrow_config_version_mut</a>(config: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">SocialProofTokensConfig</a>): &<b>mut</b> u64 {
    &<b>mut</b> config.version
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_migrate_token_registry"></a>

## Function `migrate_token_registry`

Migration function for TokenRegistry


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_migrate_token_registry">migrate_token_registry</a>(registry: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">social_contracts::social_proof_tokens::TokenRegistry</a>, _: &<a href="../social_contracts/upgrade.md#social_contracts_upgrade_UpgradeAdminCap">social_contracts::upgrade::UpgradeAdminCap</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_migrate_token_registry">migrate_token_registry</a>(
    registry: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">TokenRegistry</a>,
    _: &UpgradeAdminCap,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> current_version = <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>();
    // Verify this is an <a href="../social_contracts/upgrade.md#social_contracts_upgrade">upgrade</a> (new version &gt; current version)
    <b>assert</b>!(registry.version &lt; current_version, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInvalidFeeConfig">EInvalidFeeConfig</a>);
    // Remember old version and update to new version
    <b>let</b> old_version = registry.version;
    registry.version = current_version;
    // Emit event <b>for</b> object migration
    <b>let</b> registry_id = object::id(registry);
    <a href="../social_contracts/upgrade.md#social_contracts_upgrade_emit_migration_event">upgrade::emit_migration_event</a>(
        registry_id,
        string::utf8(b"<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">TokenRegistry</a>"),
        old_version,
        tx_context::sender(ctx)
    );
    // Any migration logic can be added here <b>for</b> future upgrades
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_migrate_token_pool"></a>

## Function `migrate_token_pool`

Migration function for TokenPool


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_migrate_token_pool">migrate_token_pool</a>(pool: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">social_contracts::social_proof_tokens::TokenPool</a>, _: &<a href="../social_contracts/upgrade.md#social_contracts_upgrade_UpgradeAdminCap">social_contracts::upgrade::UpgradeAdminCap</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_migrate_token_pool">migrate_token_pool</a>(
    pool: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">TokenPool</a>,
    _: &UpgradeAdminCap,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> current_version = <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>();
    // Verify this is an <a href="../social_contracts/upgrade.md#social_contracts_upgrade">upgrade</a> (new version &gt; current version)
    <b>assert</b>!(pool.version &lt; current_version, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInvalidFeeConfig">EInvalidFeeConfig</a>);
    // Remember old version and update to new version
    <b>let</b> old_version = pool.version;
    pool.version = current_version;
    // Emit event <b>for</b> object migration
    <b>let</b> pool_id = object::id(pool);
    <a href="../social_contracts/upgrade.md#social_contracts_upgrade_emit_migration_event">upgrade::emit_migration_event</a>(
        pool_id,
        string::utf8(b"<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenPool">TokenPool</a>"),
        old_version,
        tx_context::sender(ctx)
    );
    // Any migration logic can be added here <b>for</b> future upgrades
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_migrate_reservation_pool"></a>

## Function `migrate_reservation_pool`

Migration function for ReservationPoolObject


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_migrate_reservation_pool">migrate_reservation_pool</a>(pool: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ReservationPoolObject">social_contracts::social_proof_tokens::ReservationPoolObject</a>, _: &<a href="../social_contracts/upgrade.md#social_contracts_upgrade_UpgradeAdminCap">social_contracts::upgrade::UpgradeAdminCap</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_migrate_reservation_pool">migrate_reservation_pool</a>(
    pool: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ReservationPoolObject">ReservationPoolObject</a>,
    _: &UpgradeAdminCap,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> current_version = <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>();
    // Verify this is an <a href="../social_contracts/upgrade.md#social_contracts_upgrade">upgrade</a> (new version &gt; current version)
    <b>assert</b>!(pool.version &lt; current_version, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EInvalidFeeConfig">EInvalidFeeConfig</a>);
    // Remember old version and update to new version
    <b>let</b> old_version = pool.version;
    pool.version = current_version;
    // Emit event <b>for</b> object migration
    <b>let</b> pool_id = object::id(pool);
    <a href="../social_contracts/upgrade.md#social_contracts_upgrade_emit_migration_event">upgrade::emit_migration_event</a>(
        pool_id,
        string::utf8(b"<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_ReservationPoolObject">ReservationPoolObject</a>"),
        old_version,
        tx_context::sender(ctx)
    );
    // Any migration logic can be added here <b>for</b> future upgrades
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_migrate_social_proof_tokens_config"></a>

## Function `migrate_social_proof_tokens_config`

Migration function for SocialProofTokensConfig


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_migrate_social_proof_tokens_config">migrate_social_proof_tokens_config</a>(config: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">social_contracts::social_proof_tokens::SocialProofTokensConfig</a>, _: &<a href="../social_contracts/upgrade.md#social_contracts_upgrade_UpgradeAdminCap">social_contracts::upgrade::UpgradeAdminCap</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_migrate_social_proof_tokens_config">migrate_social_proof_tokens_config</a>(
    config: &<b>mut</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">SocialProofTokensConfig</a>,
    _: &UpgradeAdminCap,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> current_version = <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>();
    // Verify this is an <a href="../social_contracts/upgrade.md#social_contracts_upgrade">upgrade</a> (new version &gt; current version)
    <b>assert</b>!(config.version &lt; current_version, <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_EWrongVersion">EWrongVersion</a>);
    // Remember old version and update to new version
    <b>let</b> old_version = config.version;
    config.version = current_version;
    // Emit event <b>for</b> object migration
    <b>let</b> config_id = object::id(config);
    <a href="../social_contracts/upgrade.md#social_contracts_upgrade_emit_migration_event">upgrade::emit_migration_event</a>(
        config_id,
        string::utf8(b"<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensConfig">SocialProofTokensConfig</a>"),
        old_version,
        tx_context::sender(ctx)
    );
    // Any migration logic can be added here <b>for</b> future upgrades
}
</code></pre>



</details>

<a name="social_contracts_social_proof_tokens_create_social_proof_tokens_admin_cap"></a>

## Function `create_social_proof_tokens_admin_cap`

Create a SocialProofTokensAdminCap for bootstrap (package visibility only)
This function is only callable by other modules in the same package


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_create_social_proof_tokens_admin_cap">create_social_proof_tokens_admin_cap</a>(ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensAdminCap">social_contracts::social_proof_tokens::SocialProofTokensAdminCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_create_social_proof_tokens_admin_cap">create_social_proof_tokens_admin_cap</a>(ctx: &<b>mut</b> TxContext): <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensAdminCap">SocialProofTokensAdminCap</a> {
    <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_SocialProofTokensAdminCap">SocialProofTokensAdminCap</a> {
        id: object::new(ctx)
    }
}
</code></pre>



</details>
