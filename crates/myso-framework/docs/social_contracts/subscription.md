---
title: Module `social_contracts::subscription`
---

Subscription module for the MySocial network
Handles subscription services for profiles & MyData


-  [Struct `SubscriptionAdminCap`](#social_contracts_subscription_SubscriptionAdminCap)
-  [Struct `SubscriptionConfig`](#social_contracts_subscription_SubscriptionConfig)
-  [Struct `SubscriptionConfigUpdatedEvent`](#social_contracts_subscription_SubscriptionConfigUpdatedEvent)
-  [Struct `ProfileSubscriptionService`](#social_contracts_subscription_ProfileSubscriptionService)
-  [Struct `ProfileSubscription`](#social_contracts_subscription_ProfileSubscription)
-  [Struct `ProfileSubscriptionCreatedEvent`](#social_contracts_subscription_ProfileSubscriptionCreatedEvent)
-  [Struct `ProfileSubscriptionRenewedEvent`](#social_contracts_subscription_ProfileSubscriptionRenewedEvent)
-  [Struct `ProfileSubscriptionCancelledEvent`](#social_contracts_subscription_ProfileSubscriptionCancelledEvent)
-  [Struct `ProfileSubscriptionUpdatedEvent`](#social_contracts_subscription_ProfileSubscriptionUpdatedEvent)
-  [Struct `ProfileSubscriptionServiceCreatedEvent`](#social_contracts_subscription_ProfileSubscriptionServiceCreatedEvent)
-  [Struct `RenewalBalanceFundedEvent`](#social_contracts_subscription_RenewalBalanceFundedEvent)
-  [Struct `ProfileSubscriptionServiceDeactivatedEvent`](#social_contracts_subscription_ProfileSubscriptionServiceDeactivatedEvent)
-  [Constants](#@Constants_0)
-  [Function `validate_fee_config`](#social_contracts_subscription_validate_fee_config)
-  [Function `calculate_subscription_fees`](#social_contracts_subscription_calculate_subscription_fees)
-  [Function `route_non_platform_platform_fee`](#social_contracts_subscription_route_non_platform_platform_fee)
-  [Function `distribute_subscription_payment_fees_no_platform`](#social_contracts_subscription_distribute_subscription_payment_fees_no_platform)
-  [Function `distribute_subscription_payment_fees_with_platform`](#social_contracts_subscription_distribute_subscription_payment_fees_with_platform)
-  [Function `emit_subscription_config_updated`](#social_contracts_subscription_emit_subscription_config_updated)
-  [Function `create_profile_service`](#social_contracts_subscription_create_profile_service)
-  [Function `create_profile_service_entry`](#social_contracts_subscription_create_profile_service_entry)
-  [Function `subscribe_to_profile_internal_no_platform`](#social_contracts_subscription_subscribe_to_profile_internal_no_platform)
-  [Function `subscribe_to_profile_internal_with_platform`](#social_contracts_subscription_subscribe_to_profile_internal_with_platform)
-  [Function `finish_subscribe`](#social_contracts_subscription_finish_subscribe)
-  [Function `subscribe_to_profile`](#social_contracts_subscription_subscribe_to_profile)
-  [Function `subscribe_to_profile_with_platform`](#social_contracts_subscription_subscribe_to_profile_with_platform)
-  [Function `renew_subscription_internal_no_platform`](#social_contracts_subscription_renew_subscription_internal_no_platform)
-  [Function `renew_subscription_internal_with_platform`](#social_contracts_subscription_renew_subscription_internal_with_platform)
-  [Function `emit_subscription_renewed`](#social_contracts_subscription_emit_subscription_renewed)
-  [Function `renew_subscription`](#social_contracts_subscription_renew_subscription)
-  [Function `renew_subscription_with_platform`](#social_contracts_subscription_renew_subscription_with_platform)
-  [Function `auto_renew_subscription_internal_no_platform`](#social_contracts_subscription_auto_renew_subscription_internal_no_platform)
-  [Function `auto_renew_subscription_internal_with_platform`](#social_contracts_subscription_auto_renew_subscription_internal_with_platform)
-  [Function `auto_renew_subscription`](#social_contracts_subscription_auto_renew_subscription)
-  [Function `auto_renew_subscription_with_platform`](#social_contracts_subscription_auto_renew_subscription_with_platform)
-  [Function `can_auto_renew`](#social_contracts_subscription_can_auto_renew)
-  [Function `fund_renewal_balance`](#social_contracts_subscription_fund_renewal_balance)
-  [Function `is_subscription_valid`](#social_contracts_subscription_is_subscription_valid)
-  [Function `service_profile_owner`](#social_contracts_subscription_service_profile_owner)
-  [Function `service_is_active`](#social_contracts_subscription_service_is_active)
-  [Function `is_subscription_valid_for`](#social_contracts_subscription_is_subscription_valid_for)
-  [Function `update_service_fee`](#social_contracts_subscription_update_service_fee)
-  [Function `deactivate_service`](#social_contracts_subscription_deactivate_service)
-  [Function `cancel_subscription`](#social_contracts_subscription_cancel_subscription)
-  [Function `service_monthly_fee`](#social_contracts_subscription_service_monthly_fee)
-  [Function `service_subscriber_count`](#social_contracts_subscription_service_subscriber_count)
-  [Function `subscription_expires_at`](#social_contracts_subscription_subscription_expires_at)
-  [Function `subscription_auto_renew`](#social_contracts_subscription_subscription_auto_renew)
-  [Function `subscription_renewal_balance`](#social_contracts_subscription_subscription_renewal_balance)
-  [Function `bootstrap_init`](#social_contracts_subscription_bootstrap_init)
-  [Function `create_subscription_admin_cap`](#social_contracts_subscription_create_subscription_admin_cap)
-  [Function `update_subscription_config`](#social_contracts_subscription_update_subscription_config)
-  [Function `migrate_config`](#social_contracts_subscription_migrate_config)
-  [Function `migrate_service`](#social_contracts_subscription_migrate_service)


<pre><code><b>use</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption">mydata::bf_hmac_encryption</a>;
<b>use</b> <a href="../mydata/gf256.md#mydata_gf256">mydata::gf256</a>;
<b>use</b> <a href="../mydata/hmac256ctr.md#mydata_hmac256ctr">mydata::hmac256ctr</a>;
<b>use</b> <a href="../mydata/kdf.md#mydata_kdf">mydata::kdf</a>;
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
<b>use</b> <a href="../social_contracts/governance.md#social_contracts_governance">social_contracts::governance</a>;
<b>use</b> <a href="../social_contracts/memory.md#social_contracts_memory">social_contracts::memory</a>;
<b>use</b> <a href="../social_contracts/platform.md#social_contracts_platform">social_contracts::platform</a>;
<b>use</b> <a href="../social_contracts/profile.md#social_contracts_profile">social_contracts::profile</a>;
<b>use</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_contracts::social_graph</a>;
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



<a name="social_contracts_subscription_SubscriptionAdminCap"></a>

## Struct `SubscriptionAdminCap`

Admin capability for subscription configuration


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_SubscriptionAdminCap">SubscriptionAdminCap</a> <b>has</b> key, store
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

<a name="social_contracts_subscription_SubscriptionConfig"></a>

## Struct `SubscriptionConfig`

Global subscription feature configuration


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_SubscriptionConfig">SubscriptionConfig</a> <b>has</b> key
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
<code>billing_period_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_renewal_months: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>platform_fee_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>ecosystem_fee_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>non_platform_platform_to_creator_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>non_platform_platform_to_treasury_bps: u64</code>
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

<a name="social_contracts_subscription_SubscriptionConfigUpdatedEvent"></a>

## Struct `SubscriptionConfigUpdatedEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_SubscriptionConfigUpdatedEvent">SubscriptionConfigUpdatedEvent</a> <b>has</b> <b>copy</b>, drop
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
<code>billing_period_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_renewal_months: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>platform_fee_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>ecosystem_fee_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>non_platform_platform_to_creator_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>non_platform_platform_to_treasury_bps: u64</code>
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

<a name="social_contracts_subscription_ProfileSubscriptionService"></a>

## Struct `ProfileSubscriptionService`

Profile subscription service - one per profile


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">ProfileSubscriptionService</a> <b>has</b> key
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
<code>profile_owner: <b>address</b></code>
</dt>
<dd>
 Profile owner who receives subscription fees
</dd>
<dt>
<code>monthly_fee: u64</code>
</dt>
<dd>
 Monthly subscription fee in MYSO
</dd>
<dt>
<code>active: bool</code>
</dt>
<dd>
 Whether this service allows new subscriptions
</dd>
<dt>
<code>subscriber_count: u64</code>
</dt>
<dd>
 Total number of active subscribers
</dd>
<dt>
<code>version: u64</code>
</dt>
<dd>
 Version for upgrades
</dd>
</dl>


</details>

<a name="social_contracts_subscription_ProfileSubscription"></a>

## Struct `ProfileSubscription`

Individual subscription to a profile


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscription">ProfileSubscription</a> <b>has</b> key
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
<code>service_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
 The profile service this subscription is for
</dd>
<dt>
<code>subscriber: <b>address</b></code>
</dt>
<dd>
 Subscriber's address
</dd>
<dt>
<code>created_at: u64</code>
</dt>
<dd>
 When the subscription was created
</dd>
<dt>
<code>expires_at: u64</code>
</dt>
<dd>
 When the subscription expires (timestamp in ms)
</dd>
<dt>
<code>auto_renew: bool</code>
</dt>
<dd>
 Whether auto-renewal is enabled
</dd>
<dt>
<code>renewal_balance: <a href="../myso/balance.md#myso_balance_Balance">myso::balance::Balance</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;</code>
</dt>
<dd>
 Balance for auto-renewal payments
</dd>
<dt>
<code>renewal_count: u64</code>
</dt>
<dd>
 Number of times this subscription has been renewed
</dd>
</dl>


</details>

<a name="social_contracts_subscription_ProfileSubscriptionCreatedEvent"></a>

## Struct `ProfileSubscriptionCreatedEvent`

Events


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionCreatedEvent">ProfileSubscriptionCreatedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>service_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>subscriber: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>expires_at: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>monthly_fee: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>auto_renew: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>platform_fee: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>ecosystem_fee: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>creator_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>platform_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_subscription_ProfileSubscriptionRenewedEvent"></a>

## Struct `ProfileSubscriptionRenewedEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionRenewedEvent">ProfileSubscriptionRenewedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>subscription_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>subscriber: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>new_expires_at: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>renewal_count: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>auto_renewed: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>platform_fee: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>ecosystem_fee: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>creator_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>platform_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_subscription_ProfileSubscriptionCancelledEvent"></a>

## Struct `ProfileSubscriptionCancelledEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionCancelledEvent">ProfileSubscriptionCancelledEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>subscription_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>subscriber: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>refunded_amount: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_subscription_ProfileSubscriptionUpdatedEvent"></a>

## Struct `ProfileSubscriptionUpdatedEvent`

Additional event for fee updates


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionUpdatedEvent">ProfileSubscriptionUpdatedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>service_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>old_fee: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>new_fee: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>updated_by: <b>address</b></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_subscription_ProfileSubscriptionServiceCreatedEvent"></a>

## Struct `ProfileSubscriptionServiceCreatedEvent`

Event emitted when a subscription service is created


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionServiceCreatedEvent">ProfileSubscriptionServiceCreatedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>service_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>profile_owner: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>monthly_fee: u64</code>
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

<a name="social_contracts_subscription_RenewalBalanceFundedEvent"></a>

## Struct `RenewalBalanceFundedEvent`

Event emitted when renewal balance is funded


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_RenewalBalanceFundedEvent">RenewalBalanceFundedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>subscription_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>subscriber: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>funded_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>new_balance: u64</code>
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

<a name="social_contracts_subscription_ProfileSubscriptionServiceDeactivatedEvent"></a>

## Struct `ProfileSubscriptionServiceDeactivatedEvent`

Event emitted when a subscription service is deactivated


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionServiceDeactivatedEvent">ProfileSubscriptionServiceDeactivatedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>service_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>profile_owner: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>deactivated_at: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="social_contracts_subscription_EInvalidFee"></a>

Error codes


<pre><code><b>const</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_EInvalidFee">EInvalidFee</a>: u64 = 12;
</code></pre>



<a name="social_contracts_subscription_ENoAccess"></a>



<pre><code><b>const</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_ENoAccess">ENoAccess</a>: u64 = 77;
</code></pre>



<a name="social_contracts_subscription_ESubscriptionExpired"></a>



<pre><code><b>const</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_ESubscriptionExpired">ESubscriptionExpired</a>: u64 = 78;
</code></pre>



<a name="social_contracts_subscription_EAutoRenewalDisabled"></a>



<pre><code><b>const</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_EAutoRenewalDisabled">EAutoRenewalDisabled</a>: u64 = 79;
</code></pre>



<a name="social_contracts_subscription_ENotSubscriptionOwner"></a>



<pre><code><b>const</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_ENotSubscriptionOwner">ENotSubscriptionOwner</a>: u64 = 80;
</code></pre>



<a name="social_contracts_subscription_EWrongVersion"></a>



<pre><code><b>const</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_EWrongVersion">EWrongVersion</a>: u64 = 81;
</code></pre>



<a name="social_contracts_subscription_EOverflow"></a>



<pre><code><b>const</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_EOverflow">EOverflow</a>: u64 = 82;
</code></pre>



<a name="social_contracts_subscription_EInvalidInput"></a>



<pre><code><b>const</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_EInvalidInput">EInvalidInput</a>: u64 = 83;
</code></pre>



<a name="social_contracts_subscription_EInvalidConfig"></a>



<pre><code><b>const</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_EInvalidConfig">EInvalidConfig</a>: u64 = 84;
</code></pre>



<a name="social_contracts_subscription_MAX_RENEWAL_MONTHS"></a>

Default bootstrap values (used only at init)


<pre><code><b>const</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_MAX_RENEWAL_MONTHS">MAX_RENEWAL_MONTHS</a>: u64 = 120;
</code></pre>



<a name="social_contracts_subscription_MAX_U64"></a>



<pre><code><b>const</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_MAX_U64">MAX_U64</a>: u64 = 18446744073709551615;
</code></pre>



<a name="social_contracts_subscription_THIRTY_DAYS_MS"></a>



<pre><code><b>const</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_THIRTY_DAYS_MS">THIRTY_DAYS_MS</a>: u64 = 2592000000;
</code></pre>



<a name="social_contracts_subscription_BPS_DENOM"></a>



<pre><code><b>const</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_BPS_DENOM">BPS_DENOM</a>: u64 = 10000;
</code></pre>



<a name="social_contracts_subscription_DEFAULT_PLATFORM_FEE_BPS"></a>



<pre><code><b>const</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_DEFAULT_PLATFORM_FEE_BPS">DEFAULT_PLATFORM_FEE_BPS</a>: u64 = 250;
</code></pre>



<a name="social_contracts_subscription_DEFAULT_ECOSYSTEM_FEE_BPS"></a>



<pre><code><b>const</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_DEFAULT_ECOSYSTEM_FEE_BPS">DEFAULT_ECOSYSTEM_FEE_BPS</a>: u64 = 250;
</code></pre>



<a name="social_contracts_subscription_DEFAULT_NON_PLATFORM_PLATFORM_TO_CREATOR_BPS"></a>



<pre><code><b>const</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_DEFAULT_NON_PLATFORM_PLATFORM_TO_CREATOR_BPS">DEFAULT_NON_PLATFORM_PLATFORM_TO_CREATOR_BPS</a>: u64 = 0;
</code></pre>



<a name="social_contracts_subscription_DEFAULT_NON_PLATFORM_PLATFORM_TO_TREASURY_BPS"></a>



<pre><code><b>const</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_DEFAULT_NON_PLATFORM_PLATFORM_TO_TREASURY_BPS">DEFAULT_NON_PLATFORM_PLATFORM_TO_TREASURY_BPS</a>: u64 = 10000;
</code></pre>



<a name="social_contracts_subscription_validate_fee_config"></a>

## Function `validate_fee_config`



<pre><code><b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_validate_fee_config">validate_fee_config</a>(platform_fee_bps: u64, ecosystem_fee_bps: u64, non_platform_platform_to_creator_bps: u64, non_platform_platform_to_treasury_bps: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_validate_fee_config">validate_fee_config</a>(
    platform_fee_bps: u64,
    ecosystem_fee_bps: u64,
    non_platform_platform_to_creator_bps: u64,
    non_platform_platform_to_treasury_bps: u64,
) {
    <b>assert</b>!(platform_fee_bps &lt;= <a href="../social_contracts/subscription.md#social_contracts_subscription_BPS_DENOM">BPS_DENOM</a>, <a href="../social_contracts/subscription.md#social_contracts_subscription_EInvalidConfig">EInvalidConfig</a>);
    <b>assert</b>!(ecosystem_fee_bps &lt;= <a href="../social_contracts/subscription.md#social_contracts_subscription_BPS_DENOM">BPS_DENOM</a>, <a href="../social_contracts/subscription.md#social_contracts_subscription_EInvalidConfig">EInvalidConfig</a>);
    <b>assert</b>!(platform_fee_bps + ecosystem_fee_bps &lt;= <a href="../social_contracts/subscription.md#social_contracts_subscription_BPS_DENOM">BPS_DENOM</a>, <a href="../social_contracts/subscription.md#social_contracts_subscription_EInvalidConfig">EInvalidConfig</a>);
    <b>assert</b>!(non_platform_platform_to_creator_bps &lt;= <a href="../social_contracts/subscription.md#social_contracts_subscription_BPS_DENOM">BPS_DENOM</a>, <a href="../social_contracts/subscription.md#social_contracts_subscription_EInvalidConfig">EInvalidConfig</a>);
    <b>assert</b>!(non_platform_platform_to_treasury_bps &lt;= <a href="../social_contracts/subscription.md#social_contracts_subscription_BPS_DENOM">BPS_DENOM</a>, <a href="../social_contracts/subscription.md#social_contracts_subscription_EInvalidConfig">EInvalidConfig</a>);
    <b>assert</b>!(
        non_platform_platform_to_creator_bps + non_platform_platform_to_treasury_bps == <a href="../social_contracts/subscription.md#social_contracts_subscription_BPS_DENOM">BPS_DENOM</a>,
        <a href="../social_contracts/subscription.md#social_contracts_subscription_EInvalidConfig">EInvalidConfig</a>,
    );
}
</code></pre>



</details>

<a name="social_contracts_subscription_calculate_subscription_fees"></a>

## Function `calculate_subscription_fees`



<pre><code><b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_calculate_subscription_fees">calculate_subscription_fees</a>(config: &<a href="../social_contracts/subscription.md#social_contracts_subscription_SubscriptionConfig">social_contracts::subscription::SubscriptionConfig</a>, gross: u64): (u64, u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_calculate_subscription_fees">calculate_subscription_fees</a>(config: &<a href="../social_contracts/subscription.md#social_contracts_subscription_SubscriptionConfig">SubscriptionConfig</a>, gross: u64): (u64, u64, u64) {
    <b>let</b> platform_fee = (gross * config.platform_fee_bps) / <a href="../social_contracts/subscription.md#social_contracts_subscription_BPS_DENOM">BPS_DENOM</a>;
    <b>let</b> ecosystem_fee = (gross * config.ecosystem_fee_bps) / <a href="../social_contracts/subscription.md#social_contracts_subscription_BPS_DENOM">BPS_DENOM</a>;
    <b>let</b> creator_amount = gross - platform_fee - ecosystem_fee;
    (platform_fee, ecosystem_fee, creator_amount)
}
</code></pre>



</details>

<a name="social_contracts_subscription_route_non_platform_platform_fee"></a>

## Function `route_non_platform_platform_fee`



<pre><code><b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_route_non_platform_platform_fee">route_non_platform_platform_fee</a>(config: &<a href="../social_contracts/subscription.md#social_contracts_subscription_SubscriptionConfig">social_contracts::subscription::SubscriptionConfig</a>, treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, platform_fee: u64, creator_amount: u64, payment: &<b>mut</b> <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_route_non_platform_platform_fee">route_non_platform_platform_fee</a>(
    config: &<a href="../social_contracts/subscription.md#social_contracts_subscription_SubscriptionConfig">SubscriptionConfig</a>,
    treasury: &EcosystemTreasury,
    platform_fee: u64,
    creator_amount: u64,
    payment: &<b>mut</b> Coin&lt;MYSO&gt;,
    ctx: &<b>mut</b> TxContext,
): u64 {
    <b>let</b> platform_fee_to_creator =
        (platform_fee * config.non_platform_platform_to_creator_bps) / <a href="../social_contracts/subscription.md#social_contracts_subscription_BPS_DENOM">BPS_DENOM</a>;
    <b>let</b> platform_fee_to_treasury = platform_fee - platform_fee_to_creator;
    <b>let</b> creator_amount = creator_amount + platform_fee_to_creator;
    <b>if</b> (platform_fee_to_treasury &gt; 0) {
        <b>let</b> treasury_coin = coin::split(payment, platform_fee_to_treasury, ctx);
        transfer::public_transfer(treasury_coin, <a href="../social_contracts/profile.md#social_contracts_profile_get_treasury_address">profile::get_treasury_address</a>(treasury));
    };
    creator_amount
}
</code></pre>



</details>

<a name="social_contracts_subscription_distribute_subscription_payment_fees_no_platform"></a>

## Function `distribute_subscription_payment_fees_no_platform`



<pre><code><b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_distribute_subscription_payment_fees_no_platform">distribute_subscription_payment_fees_no_platform</a>(config: &<a href="../social_contracts/subscription.md#social_contracts_subscription_SubscriptionConfig">social_contracts::subscription::SubscriptionConfig</a>, treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, profile_owner: <b>address</b>, payment: <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): (u64, u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_distribute_subscription_payment_fees_no_platform">distribute_subscription_payment_fees_no_platform</a>(
    config: &<a href="../social_contracts/subscription.md#social_contracts_subscription_SubscriptionConfig">SubscriptionConfig</a>,
    treasury: &EcosystemTreasury,
    profile_owner: <b>address</b>,
    payment: Coin&lt;MYSO&gt;,
    ctx: &<b>mut</b> TxContext,
): (u64, u64, u64) {
    <b>let</b> gross = coin::value(&payment);
    <b>let</b> (platform_fee, ecosystem_fee, creator_amount) = <a href="../social_contracts/subscription.md#social_contracts_subscription_calculate_subscription_fees">calculate_subscription_fees</a>(config, gross);
    <b>let</b> <b>mut</b> payment = payment;
    <b>if</b> (ecosystem_fee &gt; 0) {
        <b>let</b> eco_coin = coin::split(&<b>mut</b> payment, ecosystem_fee, ctx);
        transfer::public_transfer(eco_coin, <a href="../social_contracts/profile.md#social_contracts_profile_get_treasury_address">profile::get_treasury_address</a>(treasury));
    };
    <b>let</b> creator_amount = <b>if</b> (platform_fee &gt; 0) {
        <a href="../social_contracts/subscription.md#social_contracts_subscription_route_non_platform_platform_fee">route_non_platform_platform_fee</a>(
            config,
            treasury,
            platform_fee,
            creator_amount,
            &<b>mut</b> payment,
            ctx,
        )
    } <b>else</b> {
        creator_amount
    };
    transfer::public_transfer(payment, profile_owner);
    (platform_fee, ecosystem_fee, creator_amount)
}
</code></pre>



</details>

<a name="social_contracts_subscription_distribute_subscription_payment_fees_with_platform"></a>

## Function `distribute_subscription_payment_fees_with_platform`



<pre><code><b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_distribute_subscription_payment_fees_with_platform">distribute_subscription_payment_fees_with_platform</a>(config: &<a href="../social_contracts/subscription.md#social_contracts_subscription_SubscriptionConfig">social_contracts::subscription::SubscriptionConfig</a>, treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, profile_owner: <b>address</b>, payment: <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): (u64, u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_distribute_subscription_payment_fees_with_platform">distribute_subscription_payment_fees_with_platform</a>(
    config: &<a href="../social_contracts/subscription.md#social_contracts_subscription_SubscriptionConfig">SubscriptionConfig</a>,
    treasury: &EcosystemTreasury,
    profile_owner: <b>address</b>,
    payment: Coin&lt;MYSO&gt;,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> Platform,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
): (u64, u64, u64) {
    <b>let</b> gross = coin::value(&payment);
    <b>let</b> (platform_fee, ecosystem_fee, creator_amount) = <a href="../social_contracts/subscription.md#social_contracts_subscription_calculate_subscription_fees">calculate_subscription_fees</a>(config, gross);
    <b>let</b> <b>mut</b> payment = payment;
    <b>if</b> (ecosystem_fee &gt; 0) {
        <b>let</b> eco_coin = coin::split(&<b>mut</b> payment, ecosystem_fee, ctx);
        transfer::public_transfer(eco_coin, <a href="../social_contracts/profile.md#social_contracts_profile_get_treasury_address">profile::get_treasury_address</a>(treasury));
    };
    <b>if</b> (platform_fee &gt; 0) {
        <b>let</b> <b>mut</b> platform_coin = coin::split(&<b>mut</b> payment, platform_fee, ctx);
        <a href="../social_contracts/platform.md#social_contracts_platform_add_to_treasury">platform::add_to_treasury</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, &<b>mut</b> platform_coin, platform_fee, clock, ctx);
        coin::destroy_zero(platform_coin);
    };
    transfer::public_transfer(payment, profile_owner);
    (platform_fee, ecosystem_fee, creator_amount)
}
</code></pre>



</details>

<a name="social_contracts_subscription_emit_subscription_config_updated"></a>

## Function `emit_subscription_config_updated`



<pre><code><b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_emit_subscription_config_updated">emit_subscription_config_updated</a>(config: &<a href="../social_contracts/subscription.md#social_contracts_subscription_SubscriptionConfig">social_contracts::subscription::SubscriptionConfig</a>, updated_by: <b>address</b>, timestamp: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_emit_subscription_config_updated">emit_subscription_config_updated</a>(
    config: &<a href="../social_contracts/subscription.md#social_contracts_subscription_SubscriptionConfig">SubscriptionConfig</a>,
    updated_by: <b>address</b>,
    timestamp: u64,
) {
    event::emit(<a href="../social_contracts/subscription.md#social_contracts_subscription_SubscriptionConfigUpdatedEvent">SubscriptionConfigUpdatedEvent</a> {
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
</code></pre>



</details>

<a name="social_contracts_subscription_create_profile_service"></a>

## Function `create_profile_service`

Create a subscription service for a profile (called by profile owner)


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_create_profile_service">create_profile_service</a>(profile_owner: <b>address</b>, monthly_fee: u64, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">social_contracts::subscription::ProfileSubscriptionService</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_create_profile_service">create_profile_service</a>(
    profile_owner: <b>address</b>,
    monthly_fee: u64,
    ctx: &<b>mut</b> TxContext
): <a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">ProfileSubscriptionService</a> {
    <b>assert</b>!(monthly_fee &gt; 0, <a href="../social_contracts/subscription.md#social_contracts_subscription_EInvalidFee">EInvalidFee</a>);
    <a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">ProfileSubscriptionService</a> {
        id: object::new(ctx),
        profile_owner,
        monthly_fee,
        active: <b>true</b>,
        subscriber_count: 0,
        version: <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(),
    }
}
</code></pre>



</details>

<a name="social_contracts_subscription_create_profile_service_entry"></a>

## Function `create_profile_service_entry`

Entry function to create and share a profile subscription service


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_create_profile_service_entry">create_profile_service_entry</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<a href="../social_contracts/profile.md#social_contracts_profile_Profile">social_contracts::profile::Profile</a>, monthly_fee: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_create_profile_service_entry">create_profile_service_entry</a>(
    <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &Profile,
    monthly_fee: u64,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(tx_context::sender(ctx) == <a href="../social_contracts/profile.md#social_contracts_profile_get_owner">profile::get_owner</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>), <a href="../social_contracts/subscription.md#social_contracts_subscription_ENoAccess">ENoAccess</a>);
    <b>assert</b>!(monthly_fee &gt; 0, <a href="../social_contracts/subscription.md#social_contracts_subscription_EInvalidFee">EInvalidFee</a>);
    <b>let</b> profile_owner = <a href="../social_contracts/profile.md#social_contracts_profile_get_owner">profile::get_owner</a>(<a href="../social_contracts/profile.md#social_contracts_profile">profile</a>);
    <b>let</b> service = <a href="../social_contracts/subscription.md#social_contracts_subscription_create_profile_service">create_profile_service</a>(
        profile_owner,
        monthly_fee,
        ctx
    );
    <b>let</b> service_id = object::id(&service);
    transfer::share_object(service);
    event::emit(<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionServiceCreatedEvent">ProfileSubscriptionServiceCreatedEvent</a> {
        service_id,
        profile_owner,
        monthly_fee,
        created_at: clock::timestamp_ms(clock),
    });
}
</code></pre>



</details>

<a name="social_contracts_subscription_subscribe_to_profile_internal_no_platform"></a>

## Function `subscribe_to_profile_internal_no_platform`



<pre><code><b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_subscribe_to_profile_internal_no_platform">subscribe_to_profile_internal_no_platform</a>(config: &<a href="../social_contracts/subscription.md#social_contracts_subscription_SubscriptionConfig">social_contracts::subscription::SubscriptionConfig</a>, service: &<b>mut</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">social_contracts::subscription::ProfileSubscriptionService</a>, treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, payment: &<b>mut</b> <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, auto_renew: bool, renewal_months: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_subscribe_to_profile_internal_no_platform">subscribe_to_profile_internal_no_platform</a>(
    config: &<a href="../social_contracts/subscription.md#social_contracts_subscription_SubscriptionConfig">SubscriptionConfig</a>,
    service: &<b>mut</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">ProfileSubscriptionService</a>,
    treasury: &EcosystemTreasury,
    payment: &<b>mut</b> Coin&lt;MYSO&gt;,
    auto_renew: bool,
    renewal_months: u64,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(service.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/subscription.md#social_contracts_subscription_EWrongVersion">EWrongVersion</a>);
    <b>assert</b>!(service.active, <a href="../social_contracts/subscription.md#social_contracts_subscription_ENoAccess">ENoAccess</a>);
    <b>if</b> (auto_renew) {
        <b>assert</b>!(renewal_months &lt;= config.max_renewal_months, <a href="../social_contracts/subscription.md#social_contracts_subscription_EInvalidInput">EInvalidInput</a>);
    };
    <b>let</b> subscriber = tx_context::sender(ctx);
    <b>let</b> now = clock::timestamp_ms(clock);
    <b>let</b> months_to_pay = <b>if</b> (auto_renew) { 1 + renewal_months } <b>else</b> { 1 };
    <b>assert</b>!(months_to_pay &lt;= <a href="../social_contracts/subscription.md#social_contracts_subscription_MAX_U64">MAX_U64</a> / service.monthly_fee, <a href="../social_contracts/subscription.md#social_contracts_subscription_EOverflow">EOverflow</a>);
    <b>let</b> total_required = service.monthly_fee * months_to_pay;
    <b>assert</b>!(coin::value(payment) &gt;= total_required, <a href="../social_contracts/subscription.md#social_contracts_subscription_EInvalidFee">EInvalidFee</a>);
    <b>let</b> first_month_payment = coin::split(payment, service.monthly_fee, ctx);
    <b>let</b> (platform_fee, ecosystem_fee, creator_amount) =
        <a href="../social_contracts/subscription.md#social_contracts_subscription_distribute_subscription_payment_fees_no_platform">distribute_subscription_payment_fees_no_platform</a>(
            config,
            treasury,
            service.profile_owner,
            first_month_payment,
            ctx,
        );
    <a href="../social_contracts/subscription.md#social_contracts_subscription_finish_subscribe">finish_subscribe</a>(
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
</code></pre>



</details>

<a name="social_contracts_subscription_subscribe_to_profile_internal_with_platform"></a>

## Function `subscribe_to_profile_internal_with_platform`



<pre><code><b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_subscribe_to_profile_internal_with_platform">subscribe_to_profile_internal_with_platform</a>(config: &<a href="../social_contracts/subscription.md#social_contracts_subscription_SubscriptionConfig">social_contracts::subscription::SubscriptionConfig</a>, service: &<b>mut</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">social_contracts::subscription::ProfileSubscriptionService</a>, treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, payment: &<b>mut</b> <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, auto_renew: bool, renewal_months: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_subscribe_to_profile_internal_with_platform">subscribe_to_profile_internal_with_platform</a>(
    config: &<a href="../social_contracts/subscription.md#social_contracts_subscription_SubscriptionConfig">SubscriptionConfig</a>,
    service: &<b>mut</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">ProfileSubscriptionService</a>,
    treasury: &EcosystemTreasury,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> Platform,
    payment: &<b>mut</b> Coin&lt;MYSO&gt;,
    auto_renew: bool,
    renewal_months: u64,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(service.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/subscription.md#social_contracts_subscription_EWrongVersion">EWrongVersion</a>);
    <b>assert</b>!(service.active, <a href="../social_contracts/subscription.md#social_contracts_subscription_ENoAccess">ENoAccess</a>);
    <b>if</b> (auto_renew) {
        <b>assert</b>!(renewal_months &lt;= config.max_renewal_months, <a href="../social_contracts/subscription.md#social_contracts_subscription_EInvalidInput">EInvalidInput</a>);
    };
    <b>let</b> subscriber = tx_context::sender(ctx);
    <b>let</b> now = clock::timestamp_ms(clock);
    <b>let</b> months_to_pay = <b>if</b> (auto_renew) { 1 + renewal_months } <b>else</b> { 1 };
    <b>assert</b>!(months_to_pay &lt;= <a href="../social_contracts/subscription.md#social_contracts_subscription_MAX_U64">MAX_U64</a> / service.monthly_fee, <a href="../social_contracts/subscription.md#social_contracts_subscription_EOverflow">EOverflow</a>);
    <b>let</b> total_required = service.monthly_fee * months_to_pay;
    <b>assert</b>!(coin::value(payment) &gt;= total_required, <a href="../social_contracts/subscription.md#social_contracts_subscription_EInvalidFee">EInvalidFee</a>);
    <b>let</b> first_month_payment = coin::split(payment, service.monthly_fee, ctx);
    <b>let</b> (platform_fee, ecosystem_fee, creator_amount) =
        <a href="../social_contracts/subscription.md#social_contracts_subscription_distribute_subscription_payment_fees_with_platform">distribute_subscription_payment_fees_with_platform</a>(
            config,
            treasury,
            service.profile_owner,
            first_month_payment,
            <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>,
            clock,
            ctx,
        );
    <a href="../social_contracts/subscription.md#social_contracts_subscription_finish_subscribe">finish_subscribe</a>(
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
        option::some(object::uid_to_address(<a href="../social_contracts/platform.md#social_contracts_platform_id">platform::id</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>))),
        ctx,
    );
}
</code></pre>



</details>

<a name="social_contracts_subscription_finish_subscribe"></a>

## Function `finish_subscribe`



<pre><code><b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_finish_subscribe">finish_subscribe</a>(config: &<a href="../social_contracts/subscription.md#social_contracts_subscription_SubscriptionConfig">social_contracts::subscription::SubscriptionConfig</a>, service: &<b>mut</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">social_contracts::subscription::ProfileSubscriptionService</a>, payment: &<b>mut</b> <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, auto_renew: bool, renewal_months: u64, subscriber: <b>address</b>, now: u64, platform_fee: u64, ecosystem_fee: u64, creator_amount: u64, platform_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_finish_subscribe">finish_subscribe</a>(
    config: &<a href="../social_contracts/subscription.md#social_contracts_subscription_SubscriptionConfig">SubscriptionConfig</a>,
    service: &<b>mut</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">ProfileSubscriptionService</a>,
    payment: &<b>mut</b> Coin&lt;MYSO&gt;,
    auto_renew: bool,
    renewal_months: u64,
    subscriber: <b>address</b>,
    now: u64,
    platform_fee: u64,
    ecosystem_fee: u64,
    creator_amount: u64,
    platform_id: Option&lt;<b>address</b>&gt;,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> renewal_balance = <b>if</b> (auto_renew && renewal_months &gt; 0) {
        <b>assert</b>!(renewal_months &lt;= <a href="../social_contracts/subscription.md#social_contracts_subscription_MAX_U64">MAX_U64</a> / service.monthly_fee, <a href="../social_contracts/subscription.md#social_contracts_subscription_EOverflow">EOverflow</a>);
        <b>let</b> renewal_amount = service.monthly_fee * renewal_months;
        <b>let</b> renewal_payment = coin::split(payment, renewal_amount, ctx);
        coin::into_balance(renewal_payment)
    } <b>else</b> {
        balance::zero&lt;MYSO&gt;()
    };
    <b>let</b> billing_period_ms = config.billing_period_ms;
    <b>assert</b>!(now &lt;= <a href="../social_contracts/subscription.md#social_contracts_subscription_MAX_U64">MAX_U64</a> - billing_period_ms, <a href="../social_contracts/subscription.md#social_contracts_subscription_EOverflow">EOverflow</a>);
    <b>let</b> expires_at = now + billing_period_ms;
    <b>let</b> <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a> = <a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscription">ProfileSubscription</a> {
        id: object::new(ctx),
        service_id: object::id(service),
        subscriber,
        created_at: now,
        expires_at,
        auto_renew,
        renewal_balance,
        renewal_count: 0,
    };
    <b>assert</b>!(service.subscriber_count &lt;= <a href="../social_contracts/subscription.md#social_contracts_subscription_MAX_U64">MAX_U64</a> - 1, <a href="../social_contracts/subscription.md#social_contracts_subscription_EOverflow">EOverflow</a>);
    service.subscriber_count = service.subscriber_count + 1;
    event::emit(<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionCreatedEvent">ProfileSubscriptionCreatedEvent</a> {
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
    transfer::transfer(<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>, subscriber);
}
</code></pre>



</details>

<a name="social_contracts_subscription_subscribe_to_profile"></a>

## Function `subscribe_to_profile`

Subscribe to a profile with optional auto-renewal (no platform fee recipient).


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_subscribe_to_profile">subscribe_to_profile</a>(config: &<a href="../social_contracts/subscription.md#social_contracts_subscription_SubscriptionConfig">social_contracts::subscription::SubscriptionConfig</a>, service: &<b>mut</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">social_contracts::subscription::ProfileSubscriptionService</a>, treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, payment: &<b>mut</b> <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, auto_renew: bool, renewal_months: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_subscribe_to_profile">subscribe_to_profile</a>(
    config: &<a href="../social_contracts/subscription.md#social_contracts_subscription_SubscriptionConfig">SubscriptionConfig</a>,
    service: &<b>mut</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">ProfileSubscriptionService</a>,
    treasury: &EcosystemTreasury,
    payment: &<b>mut</b> Coin&lt;MYSO&gt;,
    auto_renew: bool,
    renewal_months: u64,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <a href="../social_contracts/subscription.md#social_contracts_subscription_subscribe_to_profile_internal_no_platform">subscribe_to_profile_internal_no_platform</a>(
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
</code></pre>



</details>

<a name="social_contracts_subscription_subscribe_to_profile_with_platform"></a>

## Function `subscribe_to_profile_with_platform`

Subscribe to a profile with platform treasury routing for the platform-fee slice.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_subscribe_to_profile_with_platform">subscribe_to_profile_with_platform</a>(config: &<a href="../social_contracts/subscription.md#social_contracts_subscription_SubscriptionConfig">social_contracts::subscription::SubscriptionConfig</a>, service: &<b>mut</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">social_contracts::subscription::ProfileSubscriptionService</a>, treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, payment: &<b>mut</b> <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, auto_renew: bool, renewal_months: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_subscribe_to_profile_with_platform">subscribe_to_profile_with_platform</a>(
    config: &<a href="../social_contracts/subscription.md#social_contracts_subscription_SubscriptionConfig">SubscriptionConfig</a>,
    service: &<b>mut</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">ProfileSubscriptionService</a>,
    treasury: &EcosystemTreasury,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> Platform,
    payment: &<b>mut</b> Coin&lt;MYSO&gt;,
    auto_renew: bool,
    renewal_months: u64,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <a href="../social_contracts/subscription.md#social_contracts_subscription_subscribe_to_profile_internal_with_platform">subscribe_to_profile_internal_with_platform</a>(
        config,
        service,
        treasury,
        <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>,
        payment,
        auto_renew,
        renewal_months,
        clock,
        ctx,
    );
}
</code></pre>



</details>

<a name="social_contracts_subscription_renew_subscription_internal_no_platform"></a>

## Function `renew_subscription_internal_no_platform`



<pre><code><b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_renew_subscription_internal_no_platform">renew_subscription_internal_no_platform</a>(config: &<a href="../social_contracts/subscription.md#social_contracts_subscription_SubscriptionConfig">social_contracts::subscription::SubscriptionConfig</a>, service: &<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">social_contracts::subscription::ProfileSubscriptionService</a>, <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>: &<b>mut</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscription">social_contracts::subscription::ProfileSubscription</a>, payment: <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_renew_subscription_internal_no_platform">renew_subscription_internal_no_platform</a>(
    config: &<a href="../social_contracts/subscription.md#social_contracts_subscription_SubscriptionConfig">SubscriptionConfig</a>,
    service: &<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">ProfileSubscriptionService</a>,
    <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>: &<b>mut</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscription">ProfileSubscription</a>,
    payment: Coin&lt;MYSO&gt;,
    treasury: &EcosystemTreasury,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(service.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/subscription.md#social_contracts_subscription_EWrongVersion">EWrongVersion</a>);
    <b>let</b> subscriber = tx_context::sender(ctx);
    <b>assert</b>!(<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.subscriber == subscriber, <a href="../social_contracts/subscription.md#social_contracts_subscription_ENotSubscriptionOwner">ENotSubscriptionOwner</a>);
    <b>assert</b>!(<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.service_id == object::id(service), <a href="../social_contracts/subscription.md#social_contracts_subscription_ENoAccess">ENoAccess</a>);
    <b>assert</b>!(coin::value(&payment) &gt;= service.monthly_fee, <a href="../social_contracts/subscription.md#social_contracts_subscription_EInvalidFee">EInvalidFee</a>);
    <b>let</b> (platform_fee, ecosystem_fee, creator_amount) =
        <a href="../social_contracts/subscription.md#social_contracts_subscription_distribute_subscription_payment_fees_no_platform">distribute_subscription_payment_fees_no_platform</a>(
            config,
            treasury,
            service.profile_owner,
            payment,
            ctx,
        );
    <a href="../social_contracts/subscription.md#social_contracts_subscription_emit_subscription_renewed">emit_subscription_renewed</a>(
        <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>,
        subscriber,
        config,
        platform_fee,
        ecosystem_fee,
        creator_amount,
        option::none(),
        <b>false</b>,
        clock,
    );
}
</code></pre>



</details>

<a name="social_contracts_subscription_renew_subscription_internal_with_platform"></a>

## Function `renew_subscription_internal_with_platform`



<pre><code><b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_renew_subscription_internal_with_platform">renew_subscription_internal_with_platform</a>(config: &<a href="../social_contracts/subscription.md#social_contracts_subscription_SubscriptionConfig">social_contracts::subscription::SubscriptionConfig</a>, service: &<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">social_contracts::subscription::ProfileSubscriptionService</a>, <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>: &<b>mut</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscription">social_contracts::subscription::ProfileSubscription</a>, payment: <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_renew_subscription_internal_with_platform">renew_subscription_internal_with_platform</a>(
    config: &<a href="../social_contracts/subscription.md#social_contracts_subscription_SubscriptionConfig">SubscriptionConfig</a>,
    service: &<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">ProfileSubscriptionService</a>,
    <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>: &<b>mut</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscription">ProfileSubscription</a>,
    payment: Coin&lt;MYSO&gt;,
    treasury: &EcosystemTreasury,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> Platform,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(service.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/subscription.md#social_contracts_subscription_EWrongVersion">EWrongVersion</a>);
    <b>let</b> subscriber = tx_context::sender(ctx);
    <b>assert</b>!(<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.subscriber == subscriber, <a href="../social_contracts/subscription.md#social_contracts_subscription_ENotSubscriptionOwner">ENotSubscriptionOwner</a>);
    <b>assert</b>!(<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.service_id == object::id(service), <a href="../social_contracts/subscription.md#social_contracts_subscription_ENoAccess">ENoAccess</a>);
    <b>assert</b>!(coin::value(&payment) &gt;= service.monthly_fee, <a href="../social_contracts/subscription.md#social_contracts_subscription_EInvalidFee">EInvalidFee</a>);
    <b>let</b> (platform_fee, ecosystem_fee, creator_amount) =
        <a href="../social_contracts/subscription.md#social_contracts_subscription_distribute_subscription_payment_fees_with_platform">distribute_subscription_payment_fees_with_platform</a>(
            config,
            treasury,
            service.profile_owner,
            payment,
            <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>,
            clock,
            ctx,
        );
    <a href="../social_contracts/subscription.md#social_contracts_subscription_emit_subscription_renewed">emit_subscription_renewed</a>(
        <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>,
        subscriber,
        config,
        platform_fee,
        ecosystem_fee,
        creator_amount,
        option::some(object::uid_to_address(<a href="../social_contracts/platform.md#social_contracts_platform_id">platform::id</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>))),
        <b>false</b>,
        clock,
    );
}
</code></pre>



</details>

<a name="social_contracts_subscription_emit_subscription_renewed"></a>

## Function `emit_subscription_renewed`



<pre><code><b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_emit_subscription_renewed">emit_subscription_renewed</a>(<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>: &<b>mut</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscription">social_contracts::subscription::ProfileSubscription</a>, subscriber: <b>address</b>, config: &<a href="../social_contracts/subscription.md#social_contracts_subscription_SubscriptionConfig">social_contracts::subscription::SubscriptionConfig</a>, platform_fee: u64, ecosystem_fee: u64, creator_amount: u64, platform_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;, auto_renewed: bool, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_emit_subscription_renewed">emit_subscription_renewed</a>(
    <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>: &<b>mut</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscription">ProfileSubscription</a>,
    subscriber: <b>address</b>,
    config: &<a href="../social_contracts/subscription.md#social_contracts_subscription_SubscriptionConfig">SubscriptionConfig</a>,
    platform_fee: u64,
    ecosystem_fee: u64,
    creator_amount: u64,
    platform_id: Option&lt;<b>address</b>&gt;,
    auto_renewed: bool,
    clock: &Clock,
) {
    <b>let</b> now = clock::timestamp_ms(clock);
    <b>let</b> extension = config.billing_period_ms;
    <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.expires_at = <b>if</b> (now &gt; <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.expires_at) {
        <b>assert</b>!(now &lt;= <a href="../social_contracts/subscription.md#social_contracts_subscription_MAX_U64">MAX_U64</a> - extension, <a href="../social_contracts/subscription.md#social_contracts_subscription_EOverflow">EOverflow</a>);
        now + extension
    } <b>else</b> {
        <b>assert</b>!(<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.expires_at &lt;= <a href="../social_contracts/subscription.md#social_contracts_subscription_MAX_U64">MAX_U64</a> - extension, <a href="../social_contracts/subscription.md#social_contracts_subscription_EOverflow">EOverflow</a>);
        <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.expires_at + extension
    };
    <b>assert</b>!(<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.renewal_count &lt;= <a href="../social_contracts/subscription.md#social_contracts_subscription_MAX_U64">MAX_U64</a> - 1, <a href="../social_contracts/subscription.md#social_contracts_subscription_EOverflow">EOverflow</a>);
    <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.renewal_count = <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.renewal_count + 1;
    event::emit(<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionRenewedEvent">ProfileSubscriptionRenewedEvent</a> {
        subscription_id: object::id(<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>),
        subscriber,
        new_expires_at: <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.expires_at,
        renewal_count: <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.renewal_count,
        auto_renewed,
        platform_fee,
        ecosystem_fee,
        creator_amount,
        platform_id,
    });
}
</code></pre>



</details>

<a name="social_contracts_subscription_renew_subscription"></a>

## Function `renew_subscription`

Manually renew a subscription (no platform).


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_renew_subscription">renew_subscription</a>(config: &<a href="../social_contracts/subscription.md#social_contracts_subscription_SubscriptionConfig">social_contracts::subscription::SubscriptionConfig</a>, service: &<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">social_contracts::subscription::ProfileSubscriptionService</a>, <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>: &<b>mut</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscription">social_contracts::subscription::ProfileSubscription</a>, treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, payment: <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_renew_subscription">renew_subscription</a>(
    config: &<a href="../social_contracts/subscription.md#social_contracts_subscription_SubscriptionConfig">SubscriptionConfig</a>,
    service: &<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">ProfileSubscriptionService</a>,
    <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>: &<b>mut</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscription">ProfileSubscription</a>,
    treasury: &EcosystemTreasury,
    payment: Coin&lt;MYSO&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <a href="../social_contracts/subscription.md#social_contracts_subscription_renew_subscription_internal_no_platform">renew_subscription_internal_no_platform</a>(
        config,
        service,
        <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>,
        payment,
        treasury,
        clock,
        ctx,
    );
}
</code></pre>



</details>

<a name="social_contracts_subscription_renew_subscription_with_platform"></a>

## Function `renew_subscription_with_platform`

Manually renew a subscription with platform treasury routing.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_renew_subscription_with_platform">renew_subscription_with_platform</a>(config: &<a href="../social_contracts/subscription.md#social_contracts_subscription_SubscriptionConfig">social_contracts::subscription::SubscriptionConfig</a>, service: &<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">social_contracts::subscription::ProfileSubscriptionService</a>, <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>: &<b>mut</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscription">social_contracts::subscription::ProfileSubscription</a>, treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, payment: <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_renew_subscription_with_platform">renew_subscription_with_platform</a>(
    config: &<a href="../social_contracts/subscription.md#social_contracts_subscription_SubscriptionConfig">SubscriptionConfig</a>,
    service: &<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">ProfileSubscriptionService</a>,
    <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>: &<b>mut</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscription">ProfileSubscription</a>,
    treasury: &EcosystemTreasury,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> Platform,
    payment: Coin&lt;MYSO&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <a href="../social_contracts/subscription.md#social_contracts_subscription_renew_subscription_internal_with_platform">renew_subscription_internal_with_platform</a>(
        config,
        service,
        <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>,
        payment,
        treasury,
        <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>,
        clock,
        ctx,
    );
}
</code></pre>



</details>

<a name="social_contracts_subscription_auto_renew_subscription_internal_no_platform"></a>

## Function `auto_renew_subscription_internal_no_platform`



<pre><code><b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_auto_renew_subscription_internal_no_platform">auto_renew_subscription_internal_no_platform</a>(config: &<a href="../social_contracts/subscription.md#social_contracts_subscription_SubscriptionConfig">social_contracts::subscription::SubscriptionConfig</a>, service: &<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">social_contracts::subscription::ProfileSubscriptionService</a>, <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>: &<b>mut</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscription">social_contracts::subscription::ProfileSubscription</a>, treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_auto_renew_subscription_internal_no_platform">auto_renew_subscription_internal_no_platform</a>(
    config: &<a href="../social_contracts/subscription.md#social_contracts_subscription_SubscriptionConfig">SubscriptionConfig</a>,
    service: &<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">ProfileSubscriptionService</a>,
    <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>: &<b>mut</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscription">ProfileSubscription</a>,
    treasury: &EcosystemTreasury,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(service.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/subscription.md#social_contracts_subscription_EWrongVersion">EWrongVersion</a>);
    <b>assert</b>!(<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.service_id == object::id(service), <a href="../social_contracts/subscription.md#social_contracts_subscription_ENoAccess">ENoAccess</a>);
    <b>assert</b>!(<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.auto_renew, <a href="../social_contracts/subscription.md#social_contracts_subscription_EAutoRenewalDisabled">EAutoRenewalDisabled</a>);
    <b>assert</b>!(service.active, <a href="../social_contracts/subscription.md#social_contracts_subscription_ENoAccess">ENoAccess</a>);
    <b>let</b> now = clock::timestamp_ms(clock);
    <b>assert</b>!(<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.expires_at &lt;= now, <a href="../social_contracts/subscription.md#social_contracts_subscription_ESubscriptionExpired">ESubscriptionExpired</a>);
    <b>let</b> renewal_balance_value = balance::value(&<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.renewal_balance);
    <b>if</b> (renewal_balance_value &lt; service.monthly_fee) {
        <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.auto_renew = <b>false</b>;
        event::emit(<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionCancelledEvent">ProfileSubscriptionCancelledEvent</a> {
            subscription_id: object::id(<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>),
            subscriber: <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.subscriber,
            refunded_amount: 0,
        });
        <b>return</b>
    };
    <b>let</b> renewal_payment = coin::from_balance(
        balance::split(&<b>mut</b> <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.renewal_balance, service.monthly_fee),
        ctx
    );
    <b>let</b> (platform_fee, ecosystem_fee, creator_amount) =
        <a href="../social_contracts/subscription.md#social_contracts_subscription_distribute_subscription_payment_fees_no_platform">distribute_subscription_payment_fees_no_platform</a>(
            config,
            treasury,
            service.profile_owner,
            renewal_payment,
            ctx,
        );
    <b>let</b> subscriber = <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.subscriber;
    <a href="../social_contracts/subscription.md#social_contracts_subscription_emit_subscription_renewed">emit_subscription_renewed</a>(
        <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>,
        subscriber,
        config,
        platform_fee,
        ecosystem_fee,
        creator_amount,
        option::none(),
        <b>true</b>,
        clock,
    );
}
</code></pre>



</details>

<a name="social_contracts_subscription_auto_renew_subscription_internal_with_platform"></a>

## Function `auto_renew_subscription_internal_with_platform`



<pre><code><b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_auto_renew_subscription_internal_with_platform">auto_renew_subscription_internal_with_platform</a>(config: &<a href="../social_contracts/subscription.md#social_contracts_subscription_SubscriptionConfig">social_contracts::subscription::SubscriptionConfig</a>, service: &<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">social_contracts::subscription::ProfileSubscriptionService</a>, <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>: &<b>mut</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscription">social_contracts::subscription::ProfileSubscription</a>, treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_auto_renew_subscription_internal_with_platform">auto_renew_subscription_internal_with_platform</a>(
    config: &<a href="../social_contracts/subscription.md#social_contracts_subscription_SubscriptionConfig">SubscriptionConfig</a>,
    service: &<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">ProfileSubscriptionService</a>,
    <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>: &<b>mut</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscription">ProfileSubscription</a>,
    treasury: &EcosystemTreasury,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> Platform,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(service.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/subscription.md#social_contracts_subscription_EWrongVersion">EWrongVersion</a>);
    <b>assert</b>!(<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.service_id == object::id(service), <a href="../social_contracts/subscription.md#social_contracts_subscription_ENoAccess">ENoAccess</a>);
    <b>assert</b>!(<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.auto_renew, <a href="../social_contracts/subscription.md#social_contracts_subscription_EAutoRenewalDisabled">EAutoRenewalDisabled</a>);
    <b>assert</b>!(service.active, <a href="../social_contracts/subscription.md#social_contracts_subscription_ENoAccess">ENoAccess</a>);
    <b>let</b> now = clock::timestamp_ms(clock);
    <b>assert</b>!(<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.expires_at &lt;= now, <a href="../social_contracts/subscription.md#social_contracts_subscription_ESubscriptionExpired">ESubscriptionExpired</a>);
    <b>let</b> renewal_balance_value = balance::value(&<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.renewal_balance);
    <b>if</b> (renewal_balance_value &lt; service.monthly_fee) {
        <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.auto_renew = <b>false</b>;
        event::emit(<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionCancelledEvent">ProfileSubscriptionCancelledEvent</a> {
            subscription_id: object::id(<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>),
            subscriber: <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.subscriber,
            refunded_amount: 0,
        });
        <b>return</b>
    };
    <b>let</b> renewal_payment = coin::from_balance(
        balance::split(&<b>mut</b> <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.renewal_balance, service.monthly_fee),
        ctx
    );
    <b>let</b> (platform_fee, ecosystem_fee, creator_amount) =
        <a href="../social_contracts/subscription.md#social_contracts_subscription_distribute_subscription_payment_fees_with_platform">distribute_subscription_payment_fees_with_platform</a>(
            config,
            treasury,
            service.profile_owner,
            renewal_payment,
            <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>,
            clock,
            ctx,
        );
    <b>let</b> subscriber = <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.subscriber;
    <a href="../social_contracts/subscription.md#social_contracts_subscription_emit_subscription_renewed">emit_subscription_renewed</a>(
        <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>,
        subscriber,
        config,
        platform_fee,
        ecosystem_fee,
        creator_amount,
        option::some(object::uid_to_address(<a href="../social_contracts/platform.md#social_contracts_platform_id">platform::id</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>))),
        <b>true</b>,
        clock,
    );
}
</code></pre>



</details>

<a name="social_contracts_subscription_auto_renew_subscription"></a>

## Function `auto_renew_subscription`

Gas-optimized auto-renew using pre-funded renewal balance (no platform).


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_auto_renew_subscription">auto_renew_subscription</a>(config: &<a href="../social_contracts/subscription.md#social_contracts_subscription_SubscriptionConfig">social_contracts::subscription::SubscriptionConfig</a>, service: &<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">social_contracts::subscription::ProfileSubscriptionService</a>, <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>: &<b>mut</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscription">social_contracts::subscription::ProfileSubscription</a>, treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_auto_renew_subscription">auto_renew_subscription</a>(
    config: &<a href="../social_contracts/subscription.md#social_contracts_subscription_SubscriptionConfig">SubscriptionConfig</a>,
    service: &<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">ProfileSubscriptionService</a>,
    <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>: &<b>mut</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscription">ProfileSubscription</a>,
    treasury: &EcosystemTreasury,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <a href="../social_contracts/subscription.md#social_contracts_subscription_auto_renew_subscription_internal_no_platform">auto_renew_subscription_internal_no_platform</a>(
        config,
        service,
        <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>,
        treasury,
        clock,
        ctx,
    );
}
</code></pre>



</details>

<a name="social_contracts_subscription_auto_renew_subscription_with_platform"></a>

## Function `auto_renew_subscription_with_platform`

Gas-optimized auto-renew with platform treasury routing.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_auto_renew_subscription_with_platform">auto_renew_subscription_with_platform</a>(config: &<a href="../social_contracts/subscription.md#social_contracts_subscription_SubscriptionConfig">social_contracts::subscription::SubscriptionConfig</a>, service: &<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">social_contracts::subscription::ProfileSubscriptionService</a>, <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>: &<b>mut</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscription">social_contracts::subscription::ProfileSubscription</a>, treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_auto_renew_subscription_with_platform">auto_renew_subscription_with_platform</a>(
    config: &<a href="../social_contracts/subscription.md#social_contracts_subscription_SubscriptionConfig">SubscriptionConfig</a>,
    service: &<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">ProfileSubscriptionService</a>,
    <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>: &<b>mut</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscription">ProfileSubscription</a>,
    treasury: &EcosystemTreasury,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> Platform,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <a href="../social_contracts/subscription.md#social_contracts_subscription_auto_renew_subscription_internal_with_platform">auto_renew_subscription_internal_with_platform</a>(
        config,
        service,
        <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>,
        treasury,
        <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>,
        clock,
        ctx,
    );
}
</code></pre>



</details>

<a name="social_contracts_subscription_can_auto_renew"></a>

## Function `can_auto_renew`

Check if subscription is eligible for auto-renewal without expensive operations


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_can_auto_renew">can_auto_renew</a>(<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>: &<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscription">social_contracts::subscription::ProfileSubscription</a>, service: &<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">social_contracts::subscription::ProfileSubscriptionService</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_can_auto_renew">can_auto_renew</a>(
    <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>: &<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscription">ProfileSubscription</a>,
    service: &<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">ProfileSubscriptionService</a>,
    clock: &Clock
): bool {
    <b>if</b> (!<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.auto_renew) <b>return</b> <b>false</b>;
    <b>if</b> (<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.service_id != object::id(service)) <b>return</b> <b>false</b>;
    <b>if</b> (!service.active) <b>return</b> <b>false</b>;
    <b>let</b> now = clock::timestamp_ms(clock);
    <b>if</b> (<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.expires_at &gt; now) <b>return</b> <b>false</b>;
    balance::value(&<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.renewal_balance) &gt;= service.monthly_fee
}
</code></pre>



</details>

<a name="social_contracts_subscription_fund_renewal_balance"></a>

## Function `fund_renewal_balance`

User funds their renewal balance for auto-renewal


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_fund_renewal_balance">fund_renewal_balance</a>(<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>: &<b>mut</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscription">social_contracts::subscription::ProfileSubscription</a>, payment: <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_fund_renewal_balance">fund_renewal_balance</a>(
    <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>: &<b>mut</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscription">ProfileSubscription</a>,
    payment: Coin&lt;MYSO&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> subscriber = tx_context::sender(ctx);
    <b>assert</b>!(<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.subscriber == subscriber, <a href="../social_contracts/subscription.md#social_contracts_subscription_ENotSubscriptionOwner">ENotSubscriptionOwner</a>);
    <b>let</b> funded_amount = coin::value(&payment);
    balance::join(&<b>mut</b> <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.renewal_balance, coin::into_balance(payment));
    event::emit(<a href="../social_contracts/subscription.md#social_contracts_subscription_RenewalBalanceFundedEvent">RenewalBalanceFundedEvent</a> {
        subscription_id: object::id(<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>),
        subscriber,
        funded_amount,
        new_balance: balance::value(&<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.renewal_balance),
        timestamp: clock::timestamp_ms(clock),
    });
}
</code></pre>



</details>

<a name="social_contracts_subscription_is_subscription_valid"></a>

## Function `is_subscription_valid`

Check if a subscription is valid for access


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_is_subscription_valid">is_subscription_valid</a>(<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>: &<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscription">social_contracts::subscription::ProfileSubscription</a>, service: &<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">social_contracts::subscription::ProfileSubscriptionService</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_is_subscription_valid">is_subscription_valid</a>(
    <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>: &<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscription">ProfileSubscription</a>,
    service: &<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">ProfileSubscriptionService</a>,
    clock: &Clock,
): bool {
    <b>if</b> (object::id(service) != <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.service_id) {
        <b>return</b> <b>false</b>
    };
    <b>let</b> now = clock::timestamp_ms(clock);
    <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.expires_at &gt; now
}
</code></pre>



</details>

<a name="social_contracts_subscription_service_profile_owner"></a>

## Function `service_profile_owner`

Profile owner for a subscription service (for cross-module gate checks).


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_service_profile_owner">service_profile_owner</a>(service: &<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">social_contracts::subscription::ProfileSubscriptionService</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_service_profile_owner">service_profile_owner</a>(service: &<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">ProfileSubscriptionService</a>): <b>address</b> {
    service.profile_owner
}
</code></pre>



</details>

<a name="social_contracts_subscription_service_is_active"></a>

## Function `service_is_active`

Whether the service accepts new subscriptions.


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_service_is_active">service_is_active</a>(service: &<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">social_contracts::subscription::ProfileSubscriptionService</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_service_is_active">service_is_active</a>(service: &<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">ProfileSubscriptionService</a>): bool {
    service.active
}
</code></pre>



</details>

<a name="social_contracts_subscription_is_subscription_valid_for"></a>

## Function `is_subscription_valid_for`

Whether <code>subscriber</code> holds a valid subscription to <code>service</code> at <code>clock</code>.


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_is_subscription_valid_for">is_subscription_valid_for</a>(<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>: &<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscription">social_contracts::subscription::ProfileSubscription</a>, service: &<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">social_contracts::subscription::ProfileSubscriptionService</a>, subscriber: <b>address</b>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_is_subscription_valid_for">is_subscription_valid_for</a>(
    <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>: &<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscription">ProfileSubscription</a>,
    service: &<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">ProfileSubscriptionService</a>,
    subscriber: <b>address</b>,
    clock: &Clock,
): bool {
    <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.subscriber == subscriber
        && <a href="../social_contracts/subscription.md#social_contracts_subscription_is_subscription_valid">is_subscription_valid</a>(<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>, service, clock)
}
</code></pre>



</details>

<a name="social_contracts_subscription_update_service_fee"></a>

## Function `update_service_fee`

Update service fee (profile owner only)


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_update_service_fee">update_service_fee</a>(service: &<b>mut</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">social_contracts::subscription::ProfileSubscriptionService</a>, new_fee: u64, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_update_service_fee">update_service_fee</a>(
    service: &<b>mut</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">ProfileSubscriptionService</a>,
    new_fee: u64,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(service.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/subscription.md#social_contracts_subscription_EWrongVersion">EWrongVersion</a>);
    <b>assert</b>!(tx_context::sender(ctx) == service.profile_owner, <a href="../social_contracts/subscription.md#social_contracts_subscription_ENotSubscriptionOwner">ENotSubscriptionOwner</a>);
    <b>assert</b>!(new_fee &gt; 0, <a href="../social_contracts/subscription.md#social_contracts_subscription_EInvalidFee">EInvalidFee</a>);
    <b>let</b> old_fee = service.monthly_fee;
    service.monthly_fee = new_fee;
    event::emit(<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionUpdatedEvent">ProfileSubscriptionUpdatedEvent</a> {
        service_id: object::id(service),
        old_fee,
        new_fee,
        updated_by: tx_context::sender(ctx),
    });
}
</code></pre>



</details>

<a name="social_contracts_subscription_deactivate_service"></a>

## Function `deactivate_service`

Deactivate service (profile owner only)


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_deactivate_service">deactivate_service</a>(service: &<b>mut</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">social_contracts::subscription::ProfileSubscriptionService</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_deactivate_service">deactivate_service</a>(
    service: &<b>mut</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">ProfileSubscriptionService</a>,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(service.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/subscription.md#social_contracts_subscription_EWrongVersion">EWrongVersion</a>);
    <b>assert</b>!(tx_context::sender(ctx) == service.profile_owner, <a href="../social_contracts/subscription.md#social_contracts_subscription_ENotSubscriptionOwner">ENotSubscriptionOwner</a>);
    service.active = <b>false</b>;
    event::emit(<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionServiceDeactivatedEvent">ProfileSubscriptionServiceDeactivatedEvent</a> {
        service_id: object::id(service),
        profile_owner: service.profile_owner,
        deactivated_at: clock::timestamp_ms(clock),
    });
}
</code></pre>



</details>

<a name="social_contracts_subscription_cancel_subscription"></a>

## Function `cancel_subscription`

Cancel subscription and get refund of unused renewal balance


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_cancel_subscription">cancel_subscription</a>(service: &<b>mut</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">social_contracts::subscription::ProfileSubscriptionService</a>, <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>: <a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscription">social_contracts::subscription::ProfileSubscription</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_cancel_subscription">cancel_subscription</a>(
    service: &<b>mut</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">ProfileSubscriptionService</a>,
    <b>mut</b> <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>: <a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscription">ProfileSubscription</a>,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(service.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/subscription.md#social_contracts_subscription_EWrongVersion">EWrongVersion</a>);
    <b>let</b> subscriber = tx_context::sender(ctx);
    <b>assert</b>!(<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.subscriber == subscriber, <a href="../social_contracts/subscription.md#social_contracts_subscription_ENotSubscriptionOwner">ENotSubscriptionOwner</a>);
    <b>assert</b>!(<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.service_id == object::id(service), <a href="../social_contracts/subscription.md#social_contracts_subscription_ENoAccess">ENoAccess</a>);
    <b>let</b> refund_amount = balance::value(&<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.renewal_balance);
    <b>if</b> (refund_amount &gt; 0) {
        <b>let</b> refund = coin::from_balance(
            balance::withdraw_all(&<b>mut</b> <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.renewal_balance),
            ctx
        );
        transfer::public_transfer(refund, subscriber);
    };
    <b>assert</b>!(service.subscriber_count &gt; 0, <a href="../social_contracts/subscription.md#social_contracts_subscription_EOverflow">EOverflow</a>);
    service.subscriber_count = service.subscriber_count - 1;
    event::emit(<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionCancelledEvent">ProfileSubscriptionCancelledEvent</a> {
        subscription_id: object::id(&<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>),
        subscriber,
        refunded_amount: refund_amount,
    });
    <b>let</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscription">ProfileSubscription</a> {
        id,
        service_id: _,
        subscriber: _,
        created_at: _,
        expires_at: _,
        auto_renew: _,
        renewal_balance,
        renewal_count: _,
    } = <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>;
    balance::destroy_zero(renewal_balance);
    object::delete(id);
}
</code></pre>



</details>

<a name="social_contracts_subscription_service_monthly_fee"></a>

## Function `service_monthly_fee`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_service_monthly_fee">service_monthly_fee</a>(service: &<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">social_contracts::subscription::ProfileSubscriptionService</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_service_monthly_fee">service_monthly_fee</a>(service: &<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">ProfileSubscriptionService</a>): u64 {
    service.monthly_fee
}
</code></pre>



</details>

<a name="social_contracts_subscription_service_subscriber_count"></a>

## Function `service_subscriber_count`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_service_subscriber_count">service_subscriber_count</a>(service: &<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">social_contracts::subscription::ProfileSubscriptionService</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_service_subscriber_count">service_subscriber_count</a>(service: &<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">ProfileSubscriptionService</a>): u64 {
    service.subscriber_count
}
</code></pre>



</details>

<a name="social_contracts_subscription_subscription_expires_at"></a>

## Function `subscription_expires_at`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_subscription_expires_at">subscription_expires_at</a>(<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>: &<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscription">social_contracts::subscription::ProfileSubscription</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_subscription_expires_at">subscription_expires_at</a>(<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>: &<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscription">ProfileSubscription</a>): u64 {
    <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.expires_at
}
</code></pre>



</details>

<a name="social_contracts_subscription_subscription_auto_renew"></a>

## Function `subscription_auto_renew`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_subscription_auto_renew">subscription_auto_renew</a>(<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>: &<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscription">social_contracts::subscription::ProfileSubscription</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_subscription_auto_renew">subscription_auto_renew</a>(<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>: &<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscription">ProfileSubscription</a>): bool {
    <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.auto_renew
}
</code></pre>



</details>

<a name="social_contracts_subscription_subscription_renewal_balance"></a>

## Function `subscription_renewal_balance`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_subscription_renewal_balance">subscription_renewal_balance</a>(<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>: &<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscription">social_contracts::subscription::ProfileSubscription</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_subscription_renewal_balance">subscription_renewal_balance</a>(<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>: &<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscription">ProfileSubscription</a>): u64 {
    balance::value(&<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>.renewal_balance)
}
</code></pre>



</details>

<a name="social_contracts_subscription_bootstrap_init"></a>

## Function `bootstrap_init`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_bootstrap_init">bootstrap_init</a>(clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_bootstrap_init">bootstrap_init</a>(clock: &Clock, ctx: &<b>mut</b> TxContext) {
    <b>let</b> admin = tx_context::sender(ctx);
    <b>let</b> config = <a href="../social_contracts/subscription.md#social_contracts_subscription_SubscriptionConfig">SubscriptionConfig</a> {
        id: object::new(ctx),
        billing_period_ms: <a href="../social_contracts/subscription.md#social_contracts_subscription_THIRTY_DAYS_MS">THIRTY_DAYS_MS</a>,
        max_renewal_months: <a href="../social_contracts/subscription.md#social_contracts_subscription_MAX_RENEWAL_MONTHS">MAX_RENEWAL_MONTHS</a>,
        platform_fee_bps: <a href="../social_contracts/subscription.md#social_contracts_subscription_DEFAULT_PLATFORM_FEE_BPS">DEFAULT_PLATFORM_FEE_BPS</a>,
        ecosystem_fee_bps: <a href="../social_contracts/subscription.md#social_contracts_subscription_DEFAULT_ECOSYSTEM_FEE_BPS">DEFAULT_ECOSYSTEM_FEE_BPS</a>,
        non_platform_platform_to_creator_bps: <a href="../social_contracts/subscription.md#social_contracts_subscription_DEFAULT_NON_PLATFORM_PLATFORM_TO_CREATOR_BPS">DEFAULT_NON_PLATFORM_PLATFORM_TO_CREATOR_BPS</a>,
        non_platform_platform_to_treasury_bps: <a href="../social_contracts/subscription.md#social_contracts_subscription_DEFAULT_NON_PLATFORM_PLATFORM_TO_TREASURY_BPS">DEFAULT_NON_PLATFORM_PLATFORM_TO_TREASURY_BPS</a>,
        version: <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(),
    };
    <a href="../social_contracts/subscription.md#social_contracts_subscription_emit_subscription_config_updated">emit_subscription_config_updated</a>(
        &config,
        admin,
        clock::timestamp_ms(clock),
    );
    transfer::share_object(config);
}
</code></pre>



</details>

<a name="social_contracts_subscription_create_subscription_admin_cap"></a>

## Function `create_subscription_admin_cap`

Create a SubscriptionAdminCap for bootstrap (package visibility only)


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_create_subscription_admin_cap">create_subscription_admin_cap</a>(ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../social_contracts/subscription.md#social_contracts_subscription_SubscriptionAdminCap">social_contracts::subscription::SubscriptionAdminCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_create_subscription_admin_cap">create_subscription_admin_cap</a>(ctx: &<b>mut</b> TxContext): <a href="../social_contracts/subscription.md#social_contracts_subscription_SubscriptionAdminCap">SubscriptionAdminCap</a> {
    <a href="../social_contracts/subscription.md#social_contracts_subscription_SubscriptionAdminCap">SubscriptionAdminCap</a> {
        id: object::new(ctx),
    }
}
</code></pre>



</details>

<a name="social_contracts_subscription_update_subscription_config"></a>

## Function `update_subscription_config`

Update subscription configuration (admin only)


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_update_subscription_config">update_subscription_config</a>(_: &<a href="../social_contracts/subscription.md#social_contracts_subscription_SubscriptionAdminCap">social_contracts::subscription::SubscriptionAdminCap</a>, config: &<b>mut</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_SubscriptionConfig">social_contracts::subscription::SubscriptionConfig</a>, billing_period_ms: u64, max_renewal_months: u64, platform_fee_bps: u64, ecosystem_fee_bps: u64, non_platform_platform_to_creator_bps: u64, non_platform_platform_to_treasury_bps: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_update_subscription_config">update_subscription_config</a>(
    _: &<a href="../social_contracts/subscription.md#social_contracts_subscription_SubscriptionAdminCap">SubscriptionAdminCap</a>,
    config: &<b>mut</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_SubscriptionConfig">SubscriptionConfig</a>,
    billing_period_ms: u64,
    max_renewal_months: u64,
    platform_fee_bps: u64,
    ecosystem_fee_bps: u64,
    non_platform_platform_to_creator_bps: u64,
    non_platform_platform_to_treasury_bps: u64,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(billing_period_ms &gt; 0, <a href="../social_contracts/subscription.md#social_contracts_subscription_EInvalidConfig">EInvalidConfig</a>);
    <b>assert</b>!(max_renewal_months &gt; 0, <a href="../social_contracts/subscription.md#social_contracts_subscription_EInvalidConfig">EInvalidConfig</a>);
    <a href="../social_contracts/subscription.md#social_contracts_subscription_validate_fee_config">validate_fee_config</a>(
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
    <a href="../social_contracts/subscription.md#social_contracts_subscription_emit_subscription_config_updated">emit_subscription_config_updated</a>(
        config,
        tx_context::sender(ctx),
        clock::timestamp_ms(clock),
    );
}
</code></pre>



</details>

<a name="social_contracts_subscription_migrate_config"></a>

## Function `migrate_config`

Migration function for SubscriptionConfig


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_migrate_config">migrate_config</a>(config: &<b>mut</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_SubscriptionConfig">social_contracts::subscription::SubscriptionConfig</a>, _: &<a href="../social_contracts/upgrade.md#social_contracts_upgrade_UpgradeAdminCap">social_contracts::upgrade::UpgradeAdminCap</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_migrate_config">migrate_config</a>(
    config: &<b>mut</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_SubscriptionConfig">SubscriptionConfig</a>,
    _: &UpgradeAdminCap,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> current_version = <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>();
    <b>assert</b>!(config.version &lt; current_version, <a href="../social_contracts/subscription.md#social_contracts_subscription_EWrongVersion">EWrongVersion</a>);
    <b>let</b> old_version = config.version;
    config.platform_fee_bps = <a href="../social_contracts/subscription.md#social_contracts_subscription_DEFAULT_PLATFORM_FEE_BPS">DEFAULT_PLATFORM_FEE_BPS</a>;
    config.ecosystem_fee_bps = <a href="../social_contracts/subscription.md#social_contracts_subscription_DEFAULT_ECOSYSTEM_FEE_BPS">DEFAULT_ECOSYSTEM_FEE_BPS</a>;
    config.non_platform_platform_to_creator_bps = <a href="../social_contracts/subscription.md#social_contracts_subscription_DEFAULT_NON_PLATFORM_PLATFORM_TO_CREATOR_BPS">DEFAULT_NON_PLATFORM_PLATFORM_TO_CREATOR_BPS</a>;
    config.non_platform_platform_to_treasury_bps = <a href="../social_contracts/subscription.md#social_contracts_subscription_DEFAULT_NON_PLATFORM_PLATFORM_TO_TREASURY_BPS">DEFAULT_NON_PLATFORM_PLATFORM_TO_TREASURY_BPS</a>;
    config.version = current_version;
    <a href="../social_contracts/upgrade.md#social_contracts_upgrade_emit_migration_event">upgrade::emit_migration_event</a>(
        object::id(config),
        string::utf8(b"<a href="../social_contracts/subscription.md#social_contracts_subscription_SubscriptionConfig">SubscriptionConfig</a>"),
        old_version,
        tx_context::sender(ctx)
    );
}
</code></pre>



</details>

<a name="social_contracts_subscription_migrate_service"></a>

## Function `migrate_service`

Migration function for ProfileSubscriptionService


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_migrate_service">migrate_service</a>(service: &<b>mut</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">social_contracts::subscription::ProfileSubscriptionService</a>, _: &<a href="../social_contracts/upgrade.md#social_contracts_upgrade_UpgradeAdminCap">social_contracts::upgrade::UpgradeAdminCap</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_migrate_service">migrate_service</a>(
    service: &<b>mut</b> <a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">ProfileSubscriptionService</a>,
    _: &UpgradeAdminCap,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> current_version = <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>();
    <b>assert</b>!(service.version &lt; current_version, <a href="../social_contracts/subscription.md#social_contracts_subscription_EWrongVersion">EWrongVersion</a>);
    <b>let</b> old_version = service.version;
    service.version = current_version;
    <a href="../social_contracts/upgrade.md#social_contracts_upgrade_emit_migration_event">upgrade::emit_migration_event</a>(
        object::id(service),
        string::utf8(b"<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">ProfileSubscriptionService</a>"),
        old_version,
        tx_context::sender(ctx)
    );
}
</code></pre>



</details>
