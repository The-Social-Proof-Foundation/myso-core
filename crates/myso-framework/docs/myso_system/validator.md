---
title: Module `myso_system::validator`
---



-  [Struct `ValidatorMetadata`](#myso_system_validator_ValidatorMetadata)
-  [Struct `Validator`](#myso_system_validator_Validator)
-  [Struct `StakingRequestEvent`](#myso_system_validator_StakingRequestEvent)
-  [Struct `UnstakingRequestEvent`](#myso_system_validator_UnstakingRequestEvent)
-  [Struct `ConvertingToFungibleStakedMySoEvent`](#myso_system_validator_ConvertingToFungibleStakedMySoEvent)
-  [Struct `RedeemingFungibleStakedMySoEvent`](#myso_system_validator_RedeemingFungibleStakedMySoEvent)
-  [Constants](#@Constants_0)
-  [Function `new_metadata`](#myso_system_validator_new_metadata)
-  [Function `new`](#myso_system_validator_new)
-  [Function `deactivate`](#myso_system_validator_deactivate)
-  [Function `activate`](#myso_system_validator_activate)
-  [Function `adjust_stake_and_gas_price`](#myso_system_validator_adjust_stake_and_gas_price)
-  [Function `request_add_stake`](#myso_system_validator_request_add_stake)
-  [Function `convert_to_fungible_staked_myso`](#myso_system_validator_convert_to_fungible_staked_myso)
-  [Function `redeem_fungible_staked_myso`](#myso_system_validator_redeem_fungible_staked_myso)
-  [Function `request_add_stake_at_genesis`](#myso_system_validator_request_add_stake_at_genesis)
-  [Function `request_withdraw_stake`](#myso_system_validator_request_withdraw_stake)
-  [Function `request_set_gas_price`](#myso_system_validator_request_set_gas_price)
-  [Function `set_candidate_gas_price`](#myso_system_validator_set_candidate_gas_price)
-  [Function `request_set_commission_rate`](#myso_system_validator_request_set_commission_rate)
-  [Function `set_candidate_commission_rate`](#myso_system_validator_set_candidate_commission_rate)
-  [Function `deposit_stake_rewards`](#myso_system_validator_deposit_stake_rewards)
-  [Function `process_pending_stakes_and_withdraws`](#myso_system_validator_process_pending_stakes_and_withdraws)
-  [Function `is_preactive`](#myso_system_validator_is_preactive)
-  [Function `metadata`](#myso_system_validator_metadata)
-  [Function `myso_address`](#myso_system_validator_myso_address)
-  [Function `name`](#myso_system_validator_name)
-  [Function `description`](#myso_system_validator_description)
-  [Function `image_url`](#myso_system_validator_image_url)
-  [Function `project_url`](#myso_system_validator_project_url)
-  [Function `network_address`](#myso_system_validator_network_address)
-  [Function `p2p_address`](#myso_system_validator_p2p_address)
-  [Function `primary_address`](#myso_system_validator_primary_address)
-  [Function `worker_address`](#myso_system_validator_worker_address)
-  [Function `protocol_pubkey_bytes`](#myso_system_validator_protocol_pubkey_bytes)
-  [Function `proof_of_possession`](#myso_system_validator_proof_of_possession)
-  [Function `network_pubkey_bytes`](#myso_system_validator_network_pubkey_bytes)
-  [Function `worker_pubkey_bytes`](#myso_system_validator_worker_pubkey_bytes)
-  [Function `next_epoch_network_address`](#myso_system_validator_next_epoch_network_address)
-  [Function `next_epoch_p2p_address`](#myso_system_validator_next_epoch_p2p_address)
-  [Function `next_epoch_primary_address`](#myso_system_validator_next_epoch_primary_address)
-  [Function `next_epoch_worker_address`](#myso_system_validator_next_epoch_worker_address)
-  [Function `next_epoch_protocol_pubkey_bytes`](#myso_system_validator_next_epoch_protocol_pubkey_bytes)
-  [Function `next_epoch_proof_of_possession`](#myso_system_validator_next_epoch_proof_of_possession)
-  [Function `next_epoch_network_pubkey_bytes`](#myso_system_validator_next_epoch_network_pubkey_bytes)
-  [Function `next_epoch_worker_pubkey_bytes`](#myso_system_validator_next_epoch_worker_pubkey_bytes)
-  [Function `operation_cap_id`](#myso_system_validator_operation_cap_id)
-  [Function `next_epoch_gas_price`](#myso_system_validator_next_epoch_gas_price)
-  [Function `total_stake_amount`](#myso_system_validator_total_stake_amount)
-  [Function `stake_amount`](#myso_system_validator_stake_amount)
-  [Function `total_stake`](#myso_system_validator_total_stake)
-  [Function `voting_power`](#myso_system_validator_voting_power)
-  [Function `set_voting_power`](#myso_system_validator_set_voting_power)
-  [Function `pending_stake_amount`](#myso_system_validator_pending_stake_amount)
-  [Function `pending_stake_withdraw_amount`](#myso_system_validator_pending_stake_withdraw_amount)
-  [Function `gas_price`](#myso_system_validator_gas_price)
-  [Function `commission_rate`](#myso_system_validator_commission_rate)
-  [Function `pool_token_exchange_rate_at_epoch`](#myso_system_validator_pool_token_exchange_rate_at_epoch)
-  [Function `staking_pool_id`](#myso_system_validator_staking_pool_id)
-  [Function `is_duplicate`](#myso_system_validator_is_duplicate)
-  [Macro function `both_some_and_equal`](#myso_system_validator_both_some_and_equal)
-  [Function `new_unverified_validator_operation_cap_and_transfer`](#myso_system_validator_new_unverified_validator_operation_cap_and_transfer)
-  [Function `update_name`](#myso_system_validator_update_name)
-  [Function `update_description`](#myso_system_validator_update_description)
-  [Function `update_image_url`](#myso_system_validator_update_image_url)
-  [Function `update_project_url`](#myso_system_validator_update_project_url)
-  [Function `update_next_epoch_network_address`](#myso_system_validator_update_next_epoch_network_address)
-  [Function `update_candidate_network_address`](#myso_system_validator_update_candidate_network_address)
-  [Function `update_next_epoch_p2p_address`](#myso_system_validator_update_next_epoch_p2p_address)
-  [Function `update_candidate_p2p_address`](#myso_system_validator_update_candidate_p2p_address)
-  [Function `update_next_epoch_primary_address`](#myso_system_validator_update_next_epoch_primary_address)
-  [Function `update_candidate_primary_address`](#myso_system_validator_update_candidate_primary_address)
-  [Function `update_next_epoch_worker_address`](#myso_system_validator_update_next_epoch_worker_address)
-  [Function `update_candidate_worker_address`](#myso_system_validator_update_candidate_worker_address)
-  [Function `update_next_epoch_protocol_pubkey`](#myso_system_validator_update_next_epoch_protocol_pubkey)
-  [Function `update_candidate_protocol_pubkey`](#myso_system_validator_update_candidate_protocol_pubkey)
-  [Function `update_next_epoch_network_pubkey`](#myso_system_validator_update_next_epoch_network_pubkey)
-  [Function `update_candidate_network_pubkey`](#myso_system_validator_update_candidate_network_pubkey)
-  [Function `update_next_epoch_worker_pubkey`](#myso_system_validator_update_next_epoch_worker_pubkey)
-  [Function `update_candidate_worker_pubkey`](#myso_system_validator_update_candidate_worker_pubkey)
-  [Function `effectuate_staged_metadata`](#myso_system_validator_effectuate_staged_metadata)
-  [Macro function `do_extract`](#myso_system_validator_do_extract)
-  [Function `validate_metadata`](#myso_system_validator_validate_metadata)
-  [Function `validate_metadata_bcs`](#myso_system_validator_validate_metadata_bcs)
-  [Function `get_staking_pool_ref`](#myso_system_validator_get_staking_pool_ref)
-  [Function `new_from_metadata`](#myso_system_validator_new_from_metadata)


<pre><code><b>use</b> <a href="../myso/accumulator.md#myso_accumulator">myso::accumulator</a>;
<b>use</b> <a href="../myso/accumulator_settlement.md#myso_accumulator_settlement">myso::accumulator_settlement</a>;
<b>use</b> <a href="../myso/address.md#myso_address">myso::address</a>;
<b>use</b> <a href="../myso/bag.md#myso_bag">myso::bag</a>;
<b>use</b> <a href="../myso/balance.md#myso_balance">myso::balance</a>;
<b>use</b> <a href="../myso/bcs.md#myso_bcs">myso::bcs</a>;
<b>use</b> <a href="../myso/coin.md#myso_coin">myso::coin</a>;
<b>use</b> <a href="../myso/config.md#myso_config">myso::config</a>;
<b>use</b> <a href="../myso/deny_list.md#myso_deny_list">myso::deny_list</a>;
<b>use</b> <a href="../myso/dynamic_field.md#myso_dynamic_field">myso::dynamic_field</a>;
<b>use</b> <a href="../myso/dynamic_object_field.md#myso_dynamic_object_field">myso::dynamic_object_field</a>;
<b>use</b> <a href="../myso/event.md#myso_event">myso::event</a>;
<b>use</b> <a href="../myso/funds_accumulator.md#myso_funds_accumulator">myso::funds_accumulator</a>;
<b>use</b> <a href="../myso/hash.md#myso_hash">myso::hash</a>;
<b>use</b> <a href="../myso/hex.md#myso_hex">myso::hex</a>;
<b>use</b> <a href="../myso/sui.md#myso_myso">myso::myso</a>;
<b>use</b> <a href="../myso/object.md#myso_object">myso::object</a>;
<b>use</b> <a href="../myso/party.md#myso_party">myso::party</a>;
<b>use</b> <a href="../myso/protocol_config.md#myso_protocol_config">myso::protocol_config</a>;
<b>use</b> <a href="../myso/table.md#myso_table">myso::table</a>;
<b>use</b> <a href="../myso/transfer.md#myso_transfer">myso::transfer</a>;
<b>use</b> <a href="../myso/tx_context.md#myso_tx_context">myso::tx_context</a>;
<b>use</b> <a href="../myso/types.md#myso_types">myso::types</a>;
<b>use</b> <a href="../myso/url.md#myso_url">myso::url</a>;
<b>use</b> <a href="../myso/vec_map.md#myso_vec_map">myso::vec_map</a>;
<b>use</b> <a href="../myso/vec_set.md#myso_vec_set">myso::vec_set</a>;
<b>use</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool">myso_system::staking_pool</a>;
<b>use</b> <a href="../myso_system/validator_cap.md#myso_system_validator_cap">myso_system::validator_cap</a>;
<b>use</b> <a href="../std/address.md#std_address">std::address</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/internal.md#std_internal">std::internal</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/type_name.md#std_type_name">std::type_name</a>;
<b>use</b> <a href="../std/u128.md#std_u128">std::u128</a>;
<b>use</b> <a href="../std/u64.md#std_u64">std::u64</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
</code></pre>



<a name="myso_system_validator_ValidatorMetadata"></a>

## Struct `ValidatorMetadata`



<pre><code><b>public</b> <b>struct</b> <a href="../myso_system/validator.md#myso_system_validator_ValidatorMetadata">ValidatorMetadata</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../myso_system/validator.md#myso_system_validator_myso_address">myso_address</a>: <b>address</b></code>
</dt>
<dd>
 The MySo Address of the validator. This is the sender that created the Validator object,
 and also the address to send validator/coins to during withdraws.
</dd>
<dt>
<code><a href="../myso_system/validator.md#myso_system_validator_protocol_pubkey_bytes">protocol_pubkey_bytes</a>: vector&lt;u8&gt;</code>
</dt>
<dd>
 The public key bytes corresponding to the private key that the validator
 holds to sign transactions. For now, this is the same as AuthorityName.
</dd>
<dt>
<code><a href="../myso_system/validator.md#myso_system_validator_network_pubkey_bytes">network_pubkey_bytes</a>: vector&lt;u8&gt;</code>
</dt>
<dd>
 The public key bytes corresponding to the private key that the validator
 uses to establish TLS connections
</dd>
<dt>
<code><a href="../myso_system/validator.md#myso_system_validator_worker_pubkey_bytes">worker_pubkey_bytes</a>: vector&lt;u8&gt;</code>
</dt>
<dd>
 The public key bytes correstponding to the Narwhal Worker
</dd>
<dt>
<code><a href="../myso_system/validator.md#myso_system_validator_proof_of_possession">proof_of_possession</a>: vector&lt;u8&gt;</code>
</dt>
<dd>
 This is a proof that the validator has ownership of the private key
</dd>
<dt>
<code><a href="../myso_system/validator.md#myso_system_validator_name">name</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
 A unique human-readable name of this validator.
</dd>
<dt>
<code><a href="../myso_system/validator.md#myso_system_validator_description">description</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../myso_system/validator.md#myso_system_validator_image_url">image_url</a>: <a href="../myso/url.md#myso_url_Url">myso::url::Url</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../myso_system/validator.md#myso_system_validator_project_url">project_url</a>: <a href="../myso/url.md#myso_url_Url">myso::url::Url</a></code>
</dt>
<dd>
</dd>
<dt>
<code>net_address: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
 The network address of the validator (could also contain extra info such as port, DNS and etc.).
</dd>
<dt>
<code><a href="../myso_system/validator.md#myso_system_validator_p2p_address">p2p_address</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
 The address of the validator used for p2p activities such as state sync (could also contain extra info such as port, DNS and etc.).
</dd>
<dt>
<code><a href="../myso_system/validator.md#myso_system_validator_primary_address">primary_address</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
 The address of the narwhal primary
</dd>
<dt>
<code><a href="../myso_system/validator.md#myso_system_validator_worker_address">worker_address</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
 The address of the narwhal worker
</dd>
<dt>
<code><a href="../myso_system/validator.md#myso_system_validator_next_epoch_protocol_pubkey_bytes">next_epoch_protocol_pubkey_bytes</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;u8&gt;&gt;</code>
</dt>
<dd>
 "next_epoch" metadata only takes effects in the next epoch.
 If none, current value will stay unchanged.
</dd>
<dt>
<code><a href="../myso_system/validator.md#myso_system_validator_next_epoch_proof_of_possession">next_epoch_proof_of_possession</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;u8&gt;&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../myso_system/validator.md#myso_system_validator_next_epoch_network_pubkey_bytes">next_epoch_network_pubkey_bytes</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;u8&gt;&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../myso_system/validator.md#myso_system_validator_next_epoch_worker_pubkey_bytes">next_epoch_worker_pubkey_bytes</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;u8&gt;&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>next_epoch_net_address: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../myso_system/validator.md#myso_system_validator_next_epoch_p2p_address">next_epoch_p2p_address</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../myso_system/validator.md#myso_system_validator_next_epoch_primary_address">next_epoch_primary_address</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../myso_system/validator.md#myso_system_validator_next_epoch_worker_address">next_epoch_worker_address</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>extra_fields: <a href="../myso/bag.md#myso_bag_Bag">myso::bag::Bag</a></code>
</dt>
<dd>
 Any extra fields that's not defined statically.
</dd>
</dl>


</details>

<a name="myso_system_validator_Validator"></a>

## Struct `Validator`



<pre><code><b>public</b> <b>struct</b> <a href="../myso_system/validator.md#myso_system_validator_Validator">Validator</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>: <a href="../myso_system/validator.md#myso_system_validator_ValidatorMetadata">myso_system::validator::ValidatorMetadata</a></code>
</dt>
<dd>
 Summary of the validator.
</dd>
<dt>
<code><a href="../myso_system/voting_power.md#myso_system_voting_power">voting_power</a>: u64</code>
</dt>
<dd>
 The voting power of this validator, which might be different from its
 stake amount.
</dd>
<dt>
<code><a href="../myso_system/validator.md#myso_system_validator_operation_cap_id">operation_cap_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
 The ID of this validator's current valid <code>UnverifiedValidatorOperationCap</code>
</dd>
<dt>
<code><a href="../myso_system/validator.md#myso_system_validator_gas_price">gas_price</a>: u64</code>
</dt>
<dd>
 Gas price quote, updated only at end of epoch.
</dd>
<dt>
<code><a href="../myso_system/staking_pool.md#myso_system_staking_pool">staking_pool</a>: <a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakingPool">myso_system::staking_pool::StakingPool</a></code>
</dt>
<dd>
 Staking pool for this validator.
</dd>
<dt>
<code><a href="../myso_system/validator.md#myso_system_validator_commission_rate">commission_rate</a>: u64</code>
</dt>
<dd>
 Commission rate of the validator, in basis point.
</dd>
<dt>
<code>next_epoch_stake: u64</code>
</dt>
<dd>
 Total amount of stake that would be active in the next epoch.
</dd>
<dt>
<code><a href="../myso_system/validator.md#myso_system_validator_next_epoch_gas_price">next_epoch_gas_price</a>: u64</code>
</dt>
<dd>
 This validator's gas price quote for the next epoch.
</dd>
<dt>
<code>next_epoch_commission_rate: u64</code>
</dt>
<dd>
 The commission rate of the validator starting the next epoch, in basis point.
</dd>
<dt>
<code>extra_fields: <a href="../myso/bag.md#myso_bag_Bag">myso::bag::Bag</a></code>
</dt>
<dd>
 Any extra fields that's not defined statically.
</dd>
</dl>


</details>

<a name="myso_system_validator_StakingRequestEvent"></a>

## Struct `StakingRequestEvent`

Event emitted when a new stake request is received.


<pre><code><b>public</b> <b>struct</b> <a href="../myso_system/validator.md#myso_system_validator_StakingRequestEvent">StakingRequestEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>pool_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>validator_address: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>staker_address: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>epoch: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="myso_system_validator_UnstakingRequestEvent"></a>

## Struct `UnstakingRequestEvent`

Event emitted when a new unstake request is received.


<pre><code><b>public</b> <b>struct</b> <a href="../myso_system/validator.md#myso_system_validator_UnstakingRequestEvent">UnstakingRequestEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>pool_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>validator_address: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>staker_address: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>stake_activation_epoch: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>unstaking_epoch: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>principal_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>reward_amount: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="myso_system_validator_ConvertingToFungibleStakedMySoEvent"></a>

## Struct `ConvertingToFungibleStakedMySoEvent`

Event emitted when a staked MYSO is converted to a fungible staked MYSO.


<pre><code><b>public</b> <b>struct</b> <a href="../myso_system/validator.md#myso_system_validator_ConvertingToFungibleStakedMySoEvent">ConvertingToFungibleStakedMySoEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>pool_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>stake_activation_epoch: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>staked_myso_principal_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>fungible_staked_myso_amount: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="myso_system_validator_RedeemingFungibleStakedMySoEvent"></a>

## Struct `RedeemingFungibleStakedMySoEvent`

Event emitted when a fungible staked MYSO is redeemed.


<pre><code><b>public</b> <b>struct</b> <a href="../myso_system/validator.md#myso_system_validator_RedeemingFungibleStakedMySoEvent">RedeemingFungibleStakedMySoEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>pool_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>fungible_staked_myso_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>myso_amount: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="myso_system_validator_EInvalidProofOfPossession"></a>

Invalid proof_of_possession field in ValidatorMetadata


<pre><code><b>const</b> <a href="../myso_system/validator.md#myso_system_validator_EInvalidProofOfPossession">EInvalidProofOfPossession</a>: u64 = 0;
</code></pre>



<a name="myso_system_validator_EMetadataInvalidPubkey"></a>

Invalid pubkey_bytes field in ValidatorMetadata


<pre><code><b>const</b> <a href="../myso_system/validator.md#myso_system_validator_EMetadataInvalidPubkey">EMetadataInvalidPubkey</a>: u64 = 1;
</code></pre>



<a name="myso_system_validator_EMetadataInvalidNetPubkey"></a>

Invalid network_pubkey_bytes field in ValidatorMetadata


<pre><code><b>const</b> <a href="../myso_system/validator.md#myso_system_validator_EMetadataInvalidNetPubkey">EMetadataInvalidNetPubkey</a>: u64 = 2;
</code></pre>



<a name="myso_system_validator_EMetadataInvalidWorkerPubkey"></a>

Invalid worker_pubkey_bytes field in ValidatorMetadata


<pre><code><b>const</b> <a href="../myso_system/validator.md#myso_system_validator_EMetadataInvalidWorkerPubkey">EMetadataInvalidWorkerPubkey</a>: u64 = 3;
</code></pre>



<a name="myso_system_validator_EMetadataInvalidNetAddr"></a>

Invalid net_address field in ValidatorMetadata


<pre><code><b>const</b> <a href="../myso_system/validator.md#myso_system_validator_EMetadataInvalidNetAddr">EMetadataInvalidNetAddr</a>: u64 = 4;
</code></pre>



<a name="myso_system_validator_EMetadataInvalidP2pAddr"></a>

Invalid p2p_address field in ValidatorMetadata


<pre><code><b>const</b> <a href="../myso_system/validator.md#myso_system_validator_EMetadataInvalidP2pAddr">EMetadataInvalidP2pAddr</a>: u64 = 5;
</code></pre>



<a name="myso_system_validator_EMetadataInvalidPrimaryAddr"></a>

Invalid primary_address field in ValidatorMetadata


<pre><code><b>const</b> <a href="../myso_system/validator.md#myso_system_validator_EMetadataInvalidPrimaryAddr">EMetadataInvalidPrimaryAddr</a>: u64 = 6;
</code></pre>



<a name="myso_system_validator_EMetadataInvalidWorkerAddr"></a>

Invalid worker_address field in ValidatorMetadata


<pre><code><b>const</b> <a href="../myso_system/validator.md#myso_system_validator_EMetadataInvalidWorkerAddr">EMetadataInvalidWorkerAddr</a>: u64 = 7;
</code></pre>



<a name="myso_system_validator_ECommissionRateTooHigh"></a>

Commission rate set by the validator is higher than the threshold


<pre><code><b>const</b> <a href="../myso_system/validator.md#myso_system_validator_ECommissionRateTooHigh">ECommissionRateTooHigh</a>: u64 = 8;
</code></pre>



<a name="myso_system_validator_EValidatorMetadataExceedingLengthLimit"></a>

Validator Metadata is too long


<pre><code><b>const</b> <a href="../myso_system/validator.md#myso_system_validator_EValidatorMetadataExceedingLengthLimit">EValidatorMetadataExceedingLengthLimit</a>: u64 = 9;
</code></pre>



<a name="myso_system_validator_ENotValidatorCandidate"></a>

Intended validator is not a candidate one.


<pre><code><b>const</b> <a href="../myso_system/validator.md#myso_system_validator_ENotValidatorCandidate">ENotValidatorCandidate</a>: u64 = 10;
</code></pre>



<a name="myso_system_validator_EInvalidStakeAmount"></a>

Stake amount is invalid or wrong.


<pre><code><b>const</b> <a href="../myso_system/validator.md#myso_system_validator_EInvalidStakeAmount">EInvalidStakeAmount</a>: u64 = 11;
</code></pre>



<a name="myso_system_validator_ECalledDuringNonGenesis"></a>

Function called during non-genesis times.


<pre><code><b>const</b> <a href="../myso_system/validator.md#myso_system_validator_ECalledDuringNonGenesis">ECalledDuringNonGenesis</a>: u64 = 12;
</code></pre>



<a name="myso_system_validator_ENewCapNotCreatedByValidatorItself"></a>

New Capability is not created by the validator itself


<pre><code><b>const</b> <a href="../myso_system/validator.md#myso_system_validator_ENewCapNotCreatedByValidatorItself">ENewCapNotCreatedByValidatorItself</a>: u64 = 100;
</code></pre>



<a name="myso_system_validator_EInvalidCap"></a>

Capability code is not valid


<pre><code><b>const</b> <a href="../myso_system/validator.md#myso_system_validator_EInvalidCap">EInvalidCap</a>: u64 = 101;
</code></pre>



<a name="myso_system_validator_EGasPriceHigherThanThreshold"></a>

Validator trying to set gas price higher than threshold.


<pre><code><b>const</b> <a href="../myso_system/validator.md#myso_system_validator_EGasPriceHigherThanThreshold">EGasPriceHigherThanThreshold</a>: u64 = 102;
</code></pre>



<a name="myso_system_validator_MAX_COMMISSION_RATE"></a>



<pre><code><b>const</b> <a href="../myso_system/validator.md#myso_system_validator_MAX_COMMISSION_RATE">MAX_COMMISSION_RATE</a>: u64 = 2000;
</code></pre>



<a name="myso_system_validator_MAX_VALIDATOR_METADATA_LENGTH"></a>



<pre><code><b>const</b> <a href="../myso_system/validator.md#myso_system_validator_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>: u64 = 256;
</code></pre>



<a name="myso_system_validator_MAX_VALIDATOR_GAS_PRICE"></a>

Max gas price a validator can set is 100K MIST.


<pre><code><b>const</b> <a href="../myso_system/validator.md#myso_system_validator_MAX_VALIDATOR_GAS_PRICE">MAX_VALIDATOR_GAS_PRICE</a>: u64 = 100000;
</code></pre>



<a name="myso_system_validator_new_metadata"></a>

## Function `new_metadata`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_new_metadata">new_metadata</a>(<a href="../myso_system/validator.md#myso_system_validator_myso_address">myso_address</a>: <b>address</b>, <a href="../myso_system/validator.md#myso_system_validator_protocol_pubkey_bytes">protocol_pubkey_bytes</a>: vector&lt;u8&gt;, <a href="../myso_system/validator.md#myso_system_validator_network_pubkey_bytes">network_pubkey_bytes</a>: vector&lt;u8&gt;, <a href="../myso_system/validator.md#myso_system_validator_worker_pubkey_bytes">worker_pubkey_bytes</a>: vector&lt;u8&gt;, <a href="../myso_system/validator.md#myso_system_validator_proof_of_possession">proof_of_possession</a>: vector&lt;u8&gt;, <a href="../myso_system/validator.md#myso_system_validator_name">name</a>: <a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../myso_system/validator.md#myso_system_validator_description">description</a>: <a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../myso_system/validator.md#myso_system_validator_image_url">image_url</a>: <a href="../myso/url.md#myso_url_Url">myso::url::Url</a>, <a href="../myso_system/validator.md#myso_system_validator_project_url">project_url</a>: <a href="../myso/url.md#myso_url_Url">myso::url::Url</a>, net_address: <a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../myso_system/validator.md#myso_system_validator_p2p_address">p2p_address</a>: <a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../myso_system/validator.md#myso_system_validator_primary_address">primary_address</a>: <a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../myso_system/validator.md#myso_system_validator_worker_address">worker_address</a>: <a href="../std/string.md#std_string_String">std::string::String</a>, extra_fields: <a href="../myso/bag.md#myso_bag_Bag">myso::bag::Bag</a>): <a href="../myso_system/validator.md#myso_system_validator_ValidatorMetadata">myso_system::validator::ValidatorMetadata</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_new_metadata">new_metadata</a>(
    <a href="../myso_system/validator.md#myso_system_validator_myso_address">myso_address</a>: <b>address</b>,
    <a href="../myso_system/validator.md#myso_system_validator_protocol_pubkey_bytes">protocol_pubkey_bytes</a>: vector&lt;u8&gt;,
    <a href="../myso_system/validator.md#myso_system_validator_network_pubkey_bytes">network_pubkey_bytes</a>: vector&lt;u8&gt;,
    <a href="../myso_system/validator.md#myso_system_validator_worker_pubkey_bytes">worker_pubkey_bytes</a>: vector&lt;u8&gt;,
    <a href="../myso_system/validator.md#myso_system_validator_proof_of_possession">proof_of_possession</a>: vector&lt;u8&gt;,
    <a href="../myso_system/validator.md#myso_system_validator_name">name</a>: String,
    <a href="../myso_system/validator.md#myso_system_validator_description">description</a>: String,
    <a href="../myso_system/validator.md#myso_system_validator_image_url">image_url</a>: Url,
    <a href="../myso_system/validator.md#myso_system_validator_project_url">project_url</a>: Url,
    net_address: String,
    <a href="../myso_system/validator.md#myso_system_validator_p2p_address">p2p_address</a>: String,
    <a href="../myso_system/validator.md#myso_system_validator_primary_address">primary_address</a>: String,
    <a href="../myso_system/validator.md#myso_system_validator_worker_address">worker_address</a>: String,
    extra_fields: Bag,
): <a href="../myso_system/validator.md#myso_system_validator_ValidatorMetadata">ValidatorMetadata</a> {
    <a href="../myso_system/validator.md#myso_system_validator_ValidatorMetadata">ValidatorMetadata</a> {
        <a href="../myso_system/validator.md#myso_system_validator_myso_address">myso_address</a>,
        <a href="../myso_system/validator.md#myso_system_validator_protocol_pubkey_bytes">protocol_pubkey_bytes</a>,
        <a href="../myso_system/validator.md#myso_system_validator_network_pubkey_bytes">network_pubkey_bytes</a>,
        <a href="../myso_system/validator.md#myso_system_validator_worker_pubkey_bytes">worker_pubkey_bytes</a>,
        <a href="../myso_system/validator.md#myso_system_validator_proof_of_possession">proof_of_possession</a>,
        <a href="../myso_system/validator.md#myso_system_validator_name">name</a>,
        <a href="../myso_system/validator.md#myso_system_validator_description">description</a>,
        <a href="../myso_system/validator.md#myso_system_validator_image_url">image_url</a>,
        <a href="../myso_system/validator.md#myso_system_validator_project_url">project_url</a>,
        net_address,
        <a href="../myso_system/validator.md#myso_system_validator_p2p_address">p2p_address</a>,
        <a href="../myso_system/validator.md#myso_system_validator_primary_address">primary_address</a>,
        <a href="../myso_system/validator.md#myso_system_validator_worker_address">worker_address</a>,
        <a href="../myso_system/validator.md#myso_system_validator_next_epoch_protocol_pubkey_bytes">next_epoch_protocol_pubkey_bytes</a>: option::none(),
        <a href="../myso_system/validator.md#myso_system_validator_next_epoch_network_pubkey_bytes">next_epoch_network_pubkey_bytes</a>: option::none(),
        <a href="../myso_system/validator.md#myso_system_validator_next_epoch_worker_pubkey_bytes">next_epoch_worker_pubkey_bytes</a>: option::none(),
        <a href="../myso_system/validator.md#myso_system_validator_next_epoch_proof_of_possession">next_epoch_proof_of_possession</a>: option::none(),
        next_epoch_net_address: option::none(),
        <a href="../myso_system/validator.md#myso_system_validator_next_epoch_p2p_address">next_epoch_p2p_address</a>: option::none(),
        <a href="../myso_system/validator.md#myso_system_validator_next_epoch_primary_address">next_epoch_primary_address</a>: option::none(),
        <a href="../myso_system/validator.md#myso_system_validator_next_epoch_worker_address">next_epoch_worker_address</a>: option::none(),
        extra_fields,
    }
}
</code></pre>



</details>

<a name="myso_system_validator_new"></a>

## Function `new`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_new">new</a>(<a href="../myso_system/validator.md#myso_system_validator_myso_address">myso_address</a>: <b>address</b>, <a href="../myso_system/validator.md#myso_system_validator_protocol_pubkey_bytes">protocol_pubkey_bytes</a>: vector&lt;u8&gt;, <a href="../myso_system/validator.md#myso_system_validator_network_pubkey_bytes">network_pubkey_bytes</a>: vector&lt;u8&gt;, <a href="../myso_system/validator.md#myso_system_validator_worker_pubkey_bytes">worker_pubkey_bytes</a>: vector&lt;u8&gt;, <a href="../myso_system/validator.md#myso_system_validator_proof_of_possession">proof_of_possession</a>: vector&lt;u8&gt;, <a href="../myso_system/validator.md#myso_system_validator_name">name</a>: vector&lt;u8&gt;, <a href="../myso_system/validator.md#myso_system_validator_description">description</a>: vector&lt;u8&gt;, <a href="../myso_system/validator.md#myso_system_validator_image_url">image_url</a>: vector&lt;u8&gt;, <a href="../myso_system/validator.md#myso_system_validator_project_url">project_url</a>: vector&lt;u8&gt;, net_address: vector&lt;u8&gt;, <a href="../myso_system/validator.md#myso_system_validator_p2p_address">p2p_address</a>: vector&lt;u8&gt;, <a href="../myso_system/validator.md#myso_system_validator_primary_address">primary_address</a>: vector&lt;u8&gt;, <a href="../myso_system/validator.md#myso_system_validator_worker_address">worker_address</a>: vector&lt;u8&gt;, <a href="../myso_system/validator.md#myso_system_validator_gas_price">gas_price</a>: u64, <a href="../myso_system/validator.md#myso_system_validator_commission_rate">commission_rate</a>: u64, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../myso_system/validator.md#myso_system_validator_Validator">myso_system::validator::Validator</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_new">new</a>(
    <a href="../myso_system/validator.md#myso_system_validator_myso_address">myso_address</a>: <b>address</b>,
    <a href="../myso_system/validator.md#myso_system_validator_protocol_pubkey_bytes">protocol_pubkey_bytes</a>: vector&lt;u8&gt;,
    <a href="../myso_system/validator.md#myso_system_validator_network_pubkey_bytes">network_pubkey_bytes</a>: vector&lt;u8&gt;,
    <a href="../myso_system/validator.md#myso_system_validator_worker_pubkey_bytes">worker_pubkey_bytes</a>: vector&lt;u8&gt;,
    <a href="../myso_system/validator.md#myso_system_validator_proof_of_possession">proof_of_possession</a>: vector&lt;u8&gt;,
    <a href="../myso_system/validator.md#myso_system_validator_name">name</a>: vector&lt;u8&gt;,
    <a href="../myso_system/validator.md#myso_system_validator_description">description</a>: vector&lt;u8&gt;,
    <a href="../myso_system/validator.md#myso_system_validator_image_url">image_url</a>: vector&lt;u8&gt;,
    <a href="../myso_system/validator.md#myso_system_validator_project_url">project_url</a>: vector&lt;u8&gt;,
    net_address: vector&lt;u8&gt;,
    <a href="../myso_system/validator.md#myso_system_validator_p2p_address">p2p_address</a>: vector&lt;u8&gt;,
    <a href="../myso_system/validator.md#myso_system_validator_primary_address">primary_address</a>: vector&lt;u8&gt;,
    <a href="../myso_system/validator.md#myso_system_validator_worker_address">worker_address</a>: vector&lt;u8&gt;,
    <a href="../myso_system/validator.md#myso_system_validator_gas_price">gas_price</a>: u64,
    <a href="../myso_system/validator.md#myso_system_validator_commission_rate">commission_rate</a>: u64,
    ctx: &<b>mut</b> TxContext,
): <a href="../myso_system/validator.md#myso_system_validator_Validator">Validator</a> {
    <b>assert</b>!(
        net_address.length() &lt;= <a href="../myso_system/validator.md#myso_system_validator_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>
            && <a href="../myso_system/validator.md#myso_system_validator_p2p_address">p2p_address</a>.length() &lt;= <a href="../myso_system/validator.md#myso_system_validator_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>
            && <a href="../myso_system/validator.md#myso_system_validator_primary_address">primary_address</a>.length() &lt;= <a href="../myso_system/validator.md#myso_system_validator_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>
            && <a href="../myso_system/validator.md#myso_system_validator_worker_address">worker_address</a>.length() &lt;= <a href="../myso_system/validator.md#myso_system_validator_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>
            && <a href="../myso_system/validator.md#myso_system_validator_name">name</a>.length() &lt;= <a href="../myso_system/validator.md#myso_system_validator_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>
            && <a href="../myso_system/validator.md#myso_system_validator_description">description</a>.length() &lt;= <a href="../myso_system/validator.md#myso_system_validator_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>
            && <a href="../myso_system/validator.md#myso_system_validator_image_url">image_url</a>.length() &lt;= <a href="../myso_system/validator.md#myso_system_validator_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>
            && <a href="../myso_system/validator.md#myso_system_validator_project_url">project_url</a>.length() &lt;= <a href="../myso_system/validator.md#myso_system_validator_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>,
        <a href="../myso_system/validator.md#myso_system_validator_EValidatorMetadataExceedingLengthLimit">EValidatorMetadataExceedingLengthLimit</a>,
    );
    <b>assert</b>!(<a href="../myso_system/validator.md#myso_system_validator_commission_rate">commission_rate</a> &lt;= <a href="../myso_system/validator.md#myso_system_validator_MAX_COMMISSION_RATE">MAX_COMMISSION_RATE</a>, <a href="../myso_system/validator.md#myso_system_validator_ECommissionRateTooHigh">ECommissionRateTooHigh</a>);
    <b>assert</b>!(<a href="../myso_system/validator.md#myso_system_validator_gas_price">gas_price</a> &lt; <a href="../myso_system/validator.md#myso_system_validator_MAX_VALIDATOR_GAS_PRICE">MAX_VALIDATOR_GAS_PRICE</a>, <a href="../myso_system/validator.md#myso_system_validator_EGasPriceHigherThanThreshold">EGasPriceHigherThanThreshold</a>);
    <b>let</b> <a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a> = <a href="../myso_system/validator.md#myso_system_validator_new_metadata">new_metadata</a>(
        <a href="../myso_system/validator.md#myso_system_validator_myso_address">myso_address</a>,
        <a href="../myso_system/validator.md#myso_system_validator_protocol_pubkey_bytes">protocol_pubkey_bytes</a>,
        <a href="../myso_system/validator.md#myso_system_validator_network_pubkey_bytes">network_pubkey_bytes</a>,
        <a href="../myso_system/validator.md#myso_system_validator_worker_pubkey_bytes">worker_pubkey_bytes</a>,
        <a href="../myso_system/validator.md#myso_system_validator_proof_of_possession">proof_of_possession</a>,
        <a href="../myso_system/validator.md#myso_system_validator_name">name</a>.to_ascii_string().to_string(),
        <a href="../myso_system/validator.md#myso_system_validator_description">description</a>.to_ascii_string().to_string(),
        url::new_unsafe_from_bytes(<a href="../myso_system/validator.md#myso_system_validator_image_url">image_url</a>),
        url::new_unsafe_from_bytes(<a href="../myso_system/validator.md#myso_system_validator_project_url">project_url</a>),
        net_address.to_ascii_string().to_string(),
        <a href="../myso_system/validator.md#myso_system_validator_p2p_address">p2p_address</a>.to_ascii_string().to_string(),
        <a href="../myso_system/validator.md#myso_system_validator_primary_address">primary_address</a>.to_ascii_string().to_string(),
        <a href="../myso_system/validator.md#myso_system_validator_worker_address">worker_address</a>.to_ascii_string().to_string(),
        bag::new(ctx),
    );
    // Checks that the keys & addresses & PoP are valid.
    <a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>.validate();
    <a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>.<a href="../myso_system/validator.md#myso_system_validator_new_from_metadata">new_from_metadata</a>(<a href="../myso_system/validator.md#myso_system_validator_gas_price">gas_price</a>, <a href="../myso_system/validator.md#myso_system_validator_commission_rate">commission_rate</a>, ctx)
}
</code></pre>



</details>

<a name="myso_system_validator_deactivate"></a>

## Function `deactivate`

Mark Validator's <code>StakingPool</code> as inactive by setting the <code>deactivation_epoch</code>.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_deactivate">deactivate</a>(self: &<b>mut</b> <a href="../myso_system/validator.md#myso_system_validator_Validator">myso_system::validator::Validator</a>, deactivation_epoch: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_deactivate">deactivate</a>(self: &<b>mut</b> <a href="../myso_system/validator.md#myso_system_validator_Validator">Validator</a>, deactivation_epoch: u64) {
    self.<a href="../myso_system/staking_pool.md#myso_system_staking_pool">staking_pool</a>.deactivate_staking_pool(deactivation_epoch)
}
</code></pre>



</details>

<a name="myso_system_validator_activate"></a>

## Function `activate`

Activate Validator's <code>StakingPool</code> by setting the <code>activation_epoch</code>.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_activate">activate</a>(self: &<b>mut</b> <a href="../myso_system/validator.md#myso_system_validator_Validator">myso_system::validator::Validator</a>, activation_epoch: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_activate">activate</a>(self: &<b>mut</b> <a href="../myso_system/validator.md#myso_system_validator_Validator">Validator</a>, activation_epoch: u64) {
    self.<a href="../myso_system/staking_pool.md#myso_system_staking_pool">staking_pool</a>.activate_staking_pool(activation_epoch);
}
</code></pre>



</details>

<a name="myso_system_validator_adjust_stake_and_gas_price"></a>

## Function `adjust_stake_and_gas_price`

Process pending stake and pending withdraws, and update the gas price.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_adjust_stake_and_gas_price">adjust_stake_and_gas_price</a>(self: &<b>mut</b> <a href="../myso_system/validator.md#myso_system_validator_Validator">myso_system::validator::Validator</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_adjust_stake_and_gas_price">adjust_stake_and_gas_price</a>(self: &<b>mut</b> <a href="../myso_system/validator.md#myso_system_validator_Validator">Validator</a>) {
    self.<a href="../myso_system/validator.md#myso_system_validator_gas_price">gas_price</a> = self.<a href="../myso_system/validator.md#myso_system_validator_next_epoch_gas_price">next_epoch_gas_price</a>;
    self.<a href="../myso_system/validator.md#myso_system_validator_commission_rate">commission_rate</a> = self.next_epoch_commission_rate;
}
</code></pre>



</details>

<a name="myso_system_validator_request_add_stake"></a>

## Function `request_add_stake`

Request to add stake to the validator's staking pool, processed at the end of the epoch.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_request_add_stake">request_add_stake</a>(self: &<b>mut</b> <a href="../myso_system/validator.md#myso_system_validator_Validator">myso_system::validator::Validator</a>, stake: <a href="../myso/balance.md#myso_balance_Balance">myso::balance::Balance</a>&lt;<a href="../myso/sui.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, staker_address: <b>address</b>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakedMySo">myso_system::staking_pool::StakedMySo</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_request_add_stake">request_add_stake</a>(
    self: &<b>mut</b> <a href="../myso_system/validator.md#myso_system_validator_Validator">Validator</a>,
    stake: Balance&lt;MYSO&gt;,
    staker_address: <b>address</b>,
    ctx: &<b>mut</b> TxContext,
): StakedMySo {
    <b>let</b> <a href="../myso_system/validator.md#myso_system_validator_stake_amount">stake_amount</a> = stake.value();
    <b>assert</b>!(<a href="../myso_system/validator.md#myso_system_validator_stake_amount">stake_amount</a> &gt; 0, <a href="../myso_system/validator.md#myso_system_validator_EInvalidStakeAmount">EInvalidStakeAmount</a>);
    <b>let</b> stake_epoch = ctx.epoch() + 1;
    <b>let</b> staked_myso = self.<a href="../myso_system/staking_pool.md#myso_system_staking_pool">staking_pool</a>.<a href="../myso_system/validator.md#myso_system_validator_request_add_stake">request_add_stake</a>(stake, stake_epoch, ctx);
    // Process stake right away <b>if</b> staking pool is preactive.
    <b>if</b> (self.<a href="../myso_system/staking_pool.md#myso_system_staking_pool">staking_pool</a>.<a href="../myso_system/validator.md#myso_system_validator_is_preactive">is_preactive</a>()) {
        self.<a href="../myso_system/staking_pool.md#myso_system_staking_pool">staking_pool</a>.process_pending_stake();
    };
    self.next_epoch_stake = self.next_epoch_stake + <a href="../myso_system/validator.md#myso_system_validator_stake_amount">stake_amount</a>;
    event::emit(<a href="../myso_system/validator.md#myso_system_validator_StakingRequestEvent">StakingRequestEvent</a> {
        pool_id: self.<a href="../myso_system/validator.md#myso_system_validator_staking_pool_id">staking_pool_id</a>(),
        validator_address: self.<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>.<a href="../myso_system/validator.md#myso_system_validator_myso_address">myso_address</a>,
        staker_address,
        epoch: ctx.epoch(),
        amount: <a href="../myso_system/validator.md#myso_system_validator_stake_amount">stake_amount</a>,
    });
    staked_myso
}
</code></pre>



</details>

<a name="myso_system_validator_convert_to_fungible_staked_myso"></a>

## Function `convert_to_fungible_staked_myso`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_convert_to_fungible_staked_myso">convert_to_fungible_staked_myso</a>(self: &<b>mut</b> <a href="../myso_system/validator.md#myso_system_validator_Validator">myso_system::validator::Validator</a>, staked_myso: <a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakedMySo">myso_system::staking_pool::StakedMySo</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../myso_system/staking_pool.md#myso_system_staking_pool_FungibleStakedMySo">myso_system::staking_pool::FungibleStakedMySo</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_convert_to_fungible_staked_myso">convert_to_fungible_staked_myso</a>(
    self: &<b>mut</b> <a href="../myso_system/validator.md#myso_system_validator_Validator">Validator</a>,
    staked_myso: StakedMySo,
    ctx: &<b>mut</b> TxContext,
): FungibleStakedMySo {
    <b>let</b> stake_activation_epoch = staked_myso.activation_epoch();
    <b>let</b> staked_myso_principal_amount = staked_myso.amount();
    <b>let</b> fungible_staked_myso = self.<a href="../myso_system/staking_pool.md#myso_system_staking_pool">staking_pool</a>.<a href="../myso_system/validator.md#myso_system_validator_convert_to_fungible_staked_myso">convert_to_fungible_staked_myso</a>(staked_myso, ctx);
    event::emit(<a href="../myso_system/validator.md#myso_system_validator_ConvertingToFungibleStakedMySoEvent">ConvertingToFungibleStakedMySoEvent</a> {
        pool_id: self.<a href="../myso_system/validator.md#myso_system_validator_staking_pool_id">staking_pool_id</a>(),
        stake_activation_epoch,
        staked_myso_principal_amount,
        fungible_staked_myso_amount: fungible_staked_myso.value(),
    });
    fungible_staked_myso
}
</code></pre>



</details>

<a name="myso_system_validator_redeem_fungible_staked_myso"></a>

## Function `redeem_fungible_staked_myso`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_redeem_fungible_staked_myso">redeem_fungible_staked_myso</a>(self: &<b>mut</b> <a href="../myso_system/validator.md#myso_system_validator_Validator">myso_system::validator::Validator</a>, fungible_staked_myso: <a href="../myso_system/staking_pool.md#myso_system_staking_pool_FungibleStakedMySo">myso_system::staking_pool::FungibleStakedMySo</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../myso/balance.md#myso_balance_Balance">myso::balance::Balance</a>&lt;<a href="../myso/sui.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_redeem_fungible_staked_myso">redeem_fungible_staked_myso</a>(
    self: &<b>mut</b> <a href="../myso_system/validator.md#myso_system_validator_Validator">Validator</a>,
    fungible_staked_myso: FungibleStakedMySo,
    ctx: &TxContext,
): Balance&lt;MYSO&gt; {
    <b>let</b> fungible_staked_myso_amount = fungible_staked_myso.value();
    <b>let</b> myso = self.<a href="../myso_system/staking_pool.md#myso_system_staking_pool">staking_pool</a>.<a href="../myso_system/validator.md#myso_system_validator_redeem_fungible_staked_myso">redeem_fungible_staked_myso</a>(fungible_staked_myso, ctx);
    self.next_epoch_stake = self.next_epoch_stake - myso.value();
    event::emit(<a href="../myso_system/validator.md#myso_system_validator_RedeemingFungibleStakedMySoEvent">RedeemingFungibleStakedMySoEvent</a> {
        pool_id: self.<a href="../myso_system/validator.md#myso_system_validator_staking_pool_id">staking_pool_id</a>(),
        fungible_staked_myso_amount,
        myso_amount: myso.value(),
    });
    myso
}
</code></pre>



</details>

<a name="myso_system_validator_request_add_stake_at_genesis"></a>

## Function `request_add_stake_at_genesis`

Request to add stake to the validator's staking pool at genesis


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_request_add_stake_at_genesis">request_add_stake_at_genesis</a>(self: &<b>mut</b> <a href="../myso_system/validator.md#myso_system_validator_Validator">myso_system::validator::Validator</a>, stake: <a href="../myso/balance.md#myso_balance_Balance">myso::balance::Balance</a>&lt;<a href="../myso/sui.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, staker_address: <b>address</b>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_request_add_stake_at_genesis">request_add_stake_at_genesis</a>(
    self: &<b>mut</b> <a href="../myso_system/validator.md#myso_system_validator_Validator">Validator</a>,
    stake: Balance&lt;MYSO&gt;,
    staker_address: <b>address</b>,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(ctx.epoch() == 0, <a href="../myso_system/validator.md#myso_system_validator_ECalledDuringNonGenesis">ECalledDuringNonGenesis</a>);
    <b>let</b> <a href="../myso_system/validator.md#myso_system_validator_stake_amount">stake_amount</a> = stake.value();
    <b>assert</b>!(<a href="../myso_system/validator.md#myso_system_validator_stake_amount">stake_amount</a> &gt; 0, <a href="../myso_system/validator.md#myso_system_validator_EInvalidStakeAmount">EInvalidStakeAmount</a>);
    // 0 = <a href="../myso_system/genesis.md#myso_system_genesis">genesis</a> epoch
    <b>let</b> staked_myso = self.<a href="../myso_system/staking_pool.md#myso_system_staking_pool">staking_pool</a>.<a href="../myso_system/validator.md#myso_system_validator_request_add_stake">request_add_stake</a>(stake, 0, ctx);
    transfer::public_transfer(staked_myso, staker_address);
    // Process stake right away
    self.<a href="../myso_system/staking_pool.md#myso_system_staking_pool">staking_pool</a>.process_pending_stake();
    self.next_epoch_stake = self.next_epoch_stake + <a href="../myso_system/validator.md#myso_system_validator_stake_amount">stake_amount</a>;
}
</code></pre>



</details>

<a name="myso_system_validator_request_withdraw_stake"></a>

## Function `request_withdraw_stake`

Request to withdraw stake from the validator's staking pool, processed at the end of the epoch.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_request_withdraw_stake">request_withdraw_stake</a>(self: &<b>mut</b> <a href="../myso_system/validator.md#myso_system_validator_Validator">myso_system::validator::Validator</a>, staked_myso: <a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakedMySo">myso_system::staking_pool::StakedMySo</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../myso/balance.md#myso_balance_Balance">myso::balance::Balance</a>&lt;<a href="../myso/sui.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_request_withdraw_stake">request_withdraw_stake</a>(
    self: &<b>mut</b> <a href="../myso_system/validator.md#myso_system_validator_Validator">Validator</a>,
    staked_myso: StakedMySo,
    ctx: &TxContext,
): Balance&lt;MYSO&gt; {
    <b>let</b> principal_amount = staked_myso.amount();
    <b>let</b> stake_activation_epoch = staked_myso.activation_epoch();
    <b>let</b> withdrawn_stake = self.<a href="../myso_system/staking_pool.md#myso_system_staking_pool">staking_pool</a>.<a href="../myso_system/validator.md#myso_system_validator_request_withdraw_stake">request_withdraw_stake</a>(staked_myso, ctx);
    <b>let</b> withdraw_amount = withdrawn_stake.value();
    <b>let</b> reward_amount = withdraw_amount - principal_amount;
    self.next_epoch_stake = self.next_epoch_stake - withdraw_amount;
    event::emit(<a href="../myso_system/validator.md#myso_system_validator_UnstakingRequestEvent">UnstakingRequestEvent</a> {
        pool_id: self.<a href="../myso_system/validator.md#myso_system_validator_staking_pool_id">staking_pool_id</a>(),
        validator_address: self.<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>.<a href="../myso_system/validator.md#myso_system_validator_myso_address">myso_address</a>,
        staker_address: ctx.sender(),
        stake_activation_epoch,
        unstaking_epoch: ctx.epoch(),
        principal_amount,
        reward_amount,
    });
    withdrawn_stake
}
</code></pre>



</details>

<a name="myso_system_validator_request_set_gas_price"></a>

## Function `request_set_gas_price`

Request to set new gas price for the next epoch.
Need to present a <code>ValidatorOperationCap</code>.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_request_set_gas_price">request_set_gas_price</a>(self: &<b>mut</b> <a href="../myso_system/validator.md#myso_system_validator_Validator">myso_system::validator::Validator</a>, verified_cap: <a href="../myso_system/validator_cap.md#myso_system_validator_cap_ValidatorOperationCap">myso_system::validator_cap::ValidatorOperationCap</a>, new_price: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_request_set_gas_price">request_set_gas_price</a>(
    self: &<b>mut</b> <a href="../myso_system/validator.md#myso_system_validator_Validator">Validator</a>,
    verified_cap: ValidatorOperationCap,
    new_price: u64,
) {
    <b>assert</b>!(new_price &lt; <a href="../myso_system/validator.md#myso_system_validator_MAX_VALIDATOR_GAS_PRICE">MAX_VALIDATOR_GAS_PRICE</a>, <a href="../myso_system/validator.md#myso_system_validator_EGasPriceHigherThanThreshold">EGasPriceHigherThanThreshold</a>);
    <b>let</b> validator_address = *verified_cap.verified_operation_cap_address();
    <b>assert</b>!(validator_address == self.<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>.<a href="../myso_system/validator.md#myso_system_validator_myso_address">myso_address</a>, <a href="../myso_system/validator.md#myso_system_validator_EInvalidCap">EInvalidCap</a>);
    self.<a href="../myso_system/validator.md#myso_system_validator_next_epoch_gas_price">next_epoch_gas_price</a> = new_price;
}
</code></pre>



</details>

<a name="myso_system_validator_set_candidate_gas_price"></a>

## Function `set_candidate_gas_price`

Set new gas price for the candidate validator.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_set_candidate_gas_price">set_candidate_gas_price</a>(self: &<b>mut</b> <a href="../myso_system/validator.md#myso_system_validator_Validator">myso_system::validator::Validator</a>, verified_cap: <a href="../myso_system/validator_cap.md#myso_system_validator_cap_ValidatorOperationCap">myso_system::validator_cap::ValidatorOperationCap</a>, new_price: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_set_candidate_gas_price">set_candidate_gas_price</a>(
    self: &<b>mut</b> <a href="../myso_system/validator.md#myso_system_validator_Validator">Validator</a>,
    verified_cap: ValidatorOperationCap,
    new_price: u64,
) {
    <b>assert</b>!(self.<a href="../myso_system/validator.md#myso_system_validator_is_preactive">is_preactive</a>(), <a href="../myso_system/validator.md#myso_system_validator_ENotValidatorCandidate">ENotValidatorCandidate</a>);
    <b>assert</b>!(new_price &lt; <a href="../myso_system/validator.md#myso_system_validator_MAX_VALIDATOR_GAS_PRICE">MAX_VALIDATOR_GAS_PRICE</a>, <a href="../myso_system/validator.md#myso_system_validator_EGasPriceHigherThanThreshold">EGasPriceHigherThanThreshold</a>);
    <b>let</b> validator_address = *verified_cap.verified_operation_cap_address();
    <b>assert</b>!(validator_address == self.<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>.<a href="../myso_system/validator.md#myso_system_validator_myso_address">myso_address</a>, <a href="../myso_system/validator.md#myso_system_validator_EInvalidCap">EInvalidCap</a>);
    self.<a href="../myso_system/validator.md#myso_system_validator_next_epoch_gas_price">next_epoch_gas_price</a> = new_price;
    self.<a href="../myso_system/validator.md#myso_system_validator_gas_price">gas_price</a> = new_price;
}
</code></pre>



</details>

<a name="myso_system_validator_request_set_commission_rate"></a>

## Function `request_set_commission_rate`

Request to set new commission rate for the next epoch.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_request_set_commission_rate">request_set_commission_rate</a>(self: &<b>mut</b> <a href="../myso_system/validator.md#myso_system_validator_Validator">myso_system::validator::Validator</a>, new_commission_rate: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_request_set_commission_rate">request_set_commission_rate</a>(self: &<b>mut</b> <a href="../myso_system/validator.md#myso_system_validator_Validator">Validator</a>, new_commission_rate: u64) {
    <b>assert</b>!(new_commission_rate &lt;= <a href="../myso_system/validator.md#myso_system_validator_MAX_COMMISSION_RATE">MAX_COMMISSION_RATE</a>, <a href="../myso_system/validator.md#myso_system_validator_ECommissionRateTooHigh">ECommissionRateTooHigh</a>);
    self.next_epoch_commission_rate = new_commission_rate;
}
</code></pre>



</details>

<a name="myso_system_validator_set_candidate_commission_rate"></a>

## Function `set_candidate_commission_rate`

Set new commission rate for the candidate validator.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_set_candidate_commission_rate">set_candidate_commission_rate</a>(self: &<b>mut</b> <a href="../myso_system/validator.md#myso_system_validator_Validator">myso_system::validator::Validator</a>, new_commission_rate: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_set_candidate_commission_rate">set_candidate_commission_rate</a>(self: &<b>mut</b> <a href="../myso_system/validator.md#myso_system_validator_Validator">Validator</a>, new_commission_rate: u64) {
    <b>assert</b>!(self.<a href="../myso_system/validator.md#myso_system_validator_is_preactive">is_preactive</a>(), <a href="../myso_system/validator.md#myso_system_validator_ENotValidatorCandidate">ENotValidatorCandidate</a>);
    <b>assert</b>!(new_commission_rate &lt;= <a href="../myso_system/validator.md#myso_system_validator_MAX_COMMISSION_RATE">MAX_COMMISSION_RATE</a>, <a href="../myso_system/validator.md#myso_system_validator_ECommissionRateTooHigh">ECommissionRateTooHigh</a>);
    self.<a href="../myso_system/validator.md#myso_system_validator_commission_rate">commission_rate</a> = new_commission_rate;
}
</code></pre>



</details>

<a name="myso_system_validator_deposit_stake_rewards"></a>

## Function `deposit_stake_rewards`

Deposit stakes rewards into the validator's staking pool, called at the end of the epoch.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_deposit_stake_rewards">deposit_stake_rewards</a>(self: &<b>mut</b> <a href="../myso_system/validator.md#myso_system_validator_Validator">myso_system::validator::Validator</a>, reward: <a href="../myso/balance.md#myso_balance_Balance">myso::balance::Balance</a>&lt;<a href="../myso/sui.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_deposit_stake_rewards">deposit_stake_rewards</a>(self: &<b>mut</b> <a href="../myso_system/validator.md#myso_system_validator_Validator">Validator</a>, reward: Balance&lt;MYSO&gt;) {
    self.next_epoch_stake = self.next_epoch_stake + reward.value();
    self.<a href="../myso_system/staking_pool.md#myso_system_staking_pool">staking_pool</a>.deposit_rewards(reward);
}
</code></pre>



</details>

<a name="myso_system_validator_process_pending_stakes_and_withdraws"></a>

## Function `process_pending_stakes_and_withdraws`

Process pending stakes and withdraws, called at the end of the epoch.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_process_pending_stakes_and_withdraws">process_pending_stakes_and_withdraws</a>(self: &<b>mut</b> <a href="../myso_system/validator.md#myso_system_validator_Validator">myso_system::validator::Validator</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_process_pending_stakes_and_withdraws">process_pending_stakes_and_withdraws</a>(self: &<b>mut</b> <a href="../myso_system/validator.md#myso_system_validator_Validator">Validator</a>, ctx: &TxContext) {
    self.<a href="../myso_system/staking_pool.md#myso_system_staking_pool">staking_pool</a>.<a href="../myso_system/validator.md#myso_system_validator_process_pending_stakes_and_withdraws">process_pending_stakes_and_withdraws</a>(ctx);
    // TODO: bring this assertion back when we are ready.
    // <b>assert</b>!(<a href="../myso_system/validator.md#myso_system_validator_stake_amount">stake_amount</a>(self) == self.next_epoch_stake, <a href="../myso_system/validator.md#myso_system_validator_EInvalidStakeAmount">EInvalidStakeAmount</a>);
}
</code></pre>



</details>

<a name="myso_system_validator_is_preactive"></a>

## Function `is_preactive`

Returns true if the validator is preactive.


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_is_preactive">is_preactive</a>(self: &<a href="../myso_system/validator.md#myso_system_validator_Validator">myso_system::validator::Validator</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_is_preactive">is_preactive</a>(self: &<a href="../myso_system/validator.md#myso_system_validator_Validator">Validator</a>): bool {
    self.<a href="../myso_system/staking_pool.md#myso_system_staking_pool">staking_pool</a>.<a href="../myso_system/validator.md#myso_system_validator_is_preactive">is_preactive</a>()
}
</code></pre>



</details>

<a name="myso_system_validator_metadata"></a>

## Function `metadata`



<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>(self: &<a href="../myso_system/validator.md#myso_system_validator_Validator">myso_system::validator::Validator</a>): &<a href="../myso_system/validator.md#myso_system_validator_ValidatorMetadata">myso_system::validator::ValidatorMetadata</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>(self: &<a href="../myso_system/validator.md#myso_system_validator_Validator">Validator</a>): &<a href="../myso_system/validator.md#myso_system_validator_ValidatorMetadata">ValidatorMetadata</a> {
    &self.<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>
}
</code></pre>



</details>

<a name="myso_system_validator_myso_address"></a>

## Function `myso_address`



<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_myso_address">myso_address</a>(self: &<a href="../myso_system/validator.md#myso_system_validator_Validator">myso_system::validator::Validator</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_myso_address">myso_address</a>(self: &<a href="../myso_system/validator.md#myso_system_validator_Validator">Validator</a>): <b>address</b> {
    self.<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>.<a href="../myso_system/validator.md#myso_system_validator_myso_address">myso_address</a>
}
</code></pre>



</details>

<a name="myso_system_validator_name"></a>

## Function `name`



<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_name">name</a>(self: &<a href="../myso_system/validator.md#myso_system_validator_Validator">myso_system::validator::Validator</a>): &<a href="../std/string.md#std_string_String">std::string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_name">name</a>(self: &<a href="../myso_system/validator.md#myso_system_validator_Validator">Validator</a>): &String {
    &self.<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>.<a href="../myso_system/validator.md#myso_system_validator_name">name</a>
}
</code></pre>



</details>

<a name="myso_system_validator_description"></a>

## Function `description`



<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_description">description</a>(self: &<a href="../myso_system/validator.md#myso_system_validator_Validator">myso_system::validator::Validator</a>): &<a href="../std/string.md#std_string_String">std::string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_description">description</a>(self: &<a href="../myso_system/validator.md#myso_system_validator_Validator">Validator</a>): &String {
    &self.<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>.<a href="../myso_system/validator.md#myso_system_validator_description">description</a>
}
</code></pre>



</details>

<a name="myso_system_validator_image_url"></a>

## Function `image_url`



<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_image_url">image_url</a>(self: &<a href="../myso_system/validator.md#myso_system_validator_Validator">myso_system::validator::Validator</a>): &<a href="../myso/url.md#myso_url_Url">myso::url::Url</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_image_url">image_url</a>(self: &<a href="../myso_system/validator.md#myso_system_validator_Validator">Validator</a>): &Url {
    &self.<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>.<a href="../myso_system/validator.md#myso_system_validator_image_url">image_url</a>
}
</code></pre>



</details>

<a name="myso_system_validator_project_url"></a>

## Function `project_url`



<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_project_url">project_url</a>(self: &<a href="../myso_system/validator.md#myso_system_validator_Validator">myso_system::validator::Validator</a>): &<a href="../myso/url.md#myso_url_Url">myso::url::Url</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_project_url">project_url</a>(self: &<a href="../myso_system/validator.md#myso_system_validator_Validator">Validator</a>): &Url {
    &self.<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>.<a href="../myso_system/validator.md#myso_system_validator_project_url">project_url</a>
}
</code></pre>



</details>

<a name="myso_system_validator_network_address"></a>

## Function `network_address`



<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_network_address">network_address</a>(self: &<a href="../myso_system/validator.md#myso_system_validator_Validator">myso_system::validator::Validator</a>): &<a href="../std/string.md#std_string_String">std::string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_network_address">network_address</a>(self: &<a href="../myso_system/validator.md#myso_system_validator_Validator">Validator</a>): &String {
    &self.<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>.net_address
}
</code></pre>



</details>

<a name="myso_system_validator_p2p_address"></a>

## Function `p2p_address`



<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_p2p_address">p2p_address</a>(self: &<a href="../myso_system/validator.md#myso_system_validator_Validator">myso_system::validator::Validator</a>): &<a href="../std/string.md#std_string_String">std::string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_p2p_address">p2p_address</a>(self: &<a href="../myso_system/validator.md#myso_system_validator_Validator">Validator</a>): &String {
    &self.<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>.<a href="../myso_system/validator.md#myso_system_validator_p2p_address">p2p_address</a>
}
</code></pre>



</details>

<a name="myso_system_validator_primary_address"></a>

## Function `primary_address`



<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_primary_address">primary_address</a>(self: &<a href="../myso_system/validator.md#myso_system_validator_Validator">myso_system::validator::Validator</a>): &<a href="../std/string.md#std_string_String">std::string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_primary_address">primary_address</a>(self: &<a href="../myso_system/validator.md#myso_system_validator_Validator">Validator</a>): &String {
    &self.<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>.<a href="../myso_system/validator.md#myso_system_validator_primary_address">primary_address</a>
}
</code></pre>



</details>

<a name="myso_system_validator_worker_address"></a>

## Function `worker_address`



<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_worker_address">worker_address</a>(self: &<a href="../myso_system/validator.md#myso_system_validator_Validator">myso_system::validator::Validator</a>): &<a href="../std/string.md#std_string_String">std::string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_worker_address">worker_address</a>(self: &<a href="../myso_system/validator.md#myso_system_validator_Validator">Validator</a>): &String {
    &self.<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>.<a href="../myso_system/validator.md#myso_system_validator_worker_address">worker_address</a>
}
</code></pre>



</details>

<a name="myso_system_validator_protocol_pubkey_bytes"></a>

## Function `protocol_pubkey_bytes`



<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_protocol_pubkey_bytes">protocol_pubkey_bytes</a>(self: &<a href="../myso_system/validator.md#myso_system_validator_Validator">myso_system::validator::Validator</a>): &vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_protocol_pubkey_bytes">protocol_pubkey_bytes</a>(self: &<a href="../myso_system/validator.md#myso_system_validator_Validator">Validator</a>): &vector&lt;u8&gt; {
    &self.<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>.<a href="../myso_system/validator.md#myso_system_validator_protocol_pubkey_bytes">protocol_pubkey_bytes</a>
}
</code></pre>



</details>

<a name="myso_system_validator_proof_of_possession"></a>

## Function `proof_of_possession`



<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_proof_of_possession">proof_of_possession</a>(self: &<a href="../myso_system/validator.md#myso_system_validator_Validator">myso_system::validator::Validator</a>): &vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_proof_of_possession">proof_of_possession</a>(self: &<a href="../myso_system/validator.md#myso_system_validator_Validator">Validator</a>): &vector&lt;u8&gt; {
    &self.<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>.<a href="../myso_system/validator.md#myso_system_validator_proof_of_possession">proof_of_possession</a>
}
</code></pre>



</details>

<a name="myso_system_validator_network_pubkey_bytes"></a>

## Function `network_pubkey_bytes`



<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_network_pubkey_bytes">network_pubkey_bytes</a>(self: &<a href="../myso_system/validator.md#myso_system_validator_Validator">myso_system::validator::Validator</a>): &vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_network_pubkey_bytes">network_pubkey_bytes</a>(self: &<a href="../myso_system/validator.md#myso_system_validator_Validator">Validator</a>): &vector&lt;u8&gt; {
    &self.<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>.<a href="../myso_system/validator.md#myso_system_validator_network_pubkey_bytes">network_pubkey_bytes</a>
}
</code></pre>



</details>

<a name="myso_system_validator_worker_pubkey_bytes"></a>

## Function `worker_pubkey_bytes`



<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_worker_pubkey_bytes">worker_pubkey_bytes</a>(self: &<a href="../myso_system/validator.md#myso_system_validator_Validator">myso_system::validator::Validator</a>): &vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_worker_pubkey_bytes">worker_pubkey_bytes</a>(self: &<a href="../myso_system/validator.md#myso_system_validator_Validator">Validator</a>): &vector&lt;u8&gt; {
    &self.<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>.<a href="../myso_system/validator.md#myso_system_validator_worker_pubkey_bytes">worker_pubkey_bytes</a>
}
</code></pre>



</details>

<a name="myso_system_validator_next_epoch_network_address"></a>

## Function `next_epoch_network_address`



<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_next_epoch_network_address">next_epoch_network_address</a>(self: &<a href="../myso_system/validator.md#myso_system_validator_Validator">myso_system::validator::Validator</a>): &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_next_epoch_network_address">next_epoch_network_address</a>(self: &<a href="../myso_system/validator.md#myso_system_validator_Validator">Validator</a>): &Option&lt;String&gt; {
    &self.<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>.next_epoch_net_address
}
</code></pre>



</details>

<a name="myso_system_validator_next_epoch_p2p_address"></a>

## Function `next_epoch_p2p_address`



<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_next_epoch_p2p_address">next_epoch_p2p_address</a>(self: &<a href="../myso_system/validator.md#myso_system_validator_Validator">myso_system::validator::Validator</a>): &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_next_epoch_p2p_address">next_epoch_p2p_address</a>(self: &<a href="../myso_system/validator.md#myso_system_validator_Validator">Validator</a>): &Option&lt;String&gt; {
    &self.<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>.<a href="../myso_system/validator.md#myso_system_validator_next_epoch_p2p_address">next_epoch_p2p_address</a>
}
</code></pre>



</details>

<a name="myso_system_validator_next_epoch_primary_address"></a>

## Function `next_epoch_primary_address`



<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_next_epoch_primary_address">next_epoch_primary_address</a>(self: &<a href="../myso_system/validator.md#myso_system_validator_Validator">myso_system::validator::Validator</a>): &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_next_epoch_primary_address">next_epoch_primary_address</a>(self: &<a href="../myso_system/validator.md#myso_system_validator_Validator">Validator</a>): &Option&lt;String&gt; {
    &self.<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>.<a href="../myso_system/validator.md#myso_system_validator_next_epoch_primary_address">next_epoch_primary_address</a>
}
</code></pre>



</details>

<a name="myso_system_validator_next_epoch_worker_address"></a>

## Function `next_epoch_worker_address`



<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_next_epoch_worker_address">next_epoch_worker_address</a>(self: &<a href="../myso_system/validator.md#myso_system_validator_Validator">myso_system::validator::Validator</a>): &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_next_epoch_worker_address">next_epoch_worker_address</a>(self: &<a href="../myso_system/validator.md#myso_system_validator_Validator">Validator</a>): &Option&lt;String&gt; {
    &self.<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>.<a href="../myso_system/validator.md#myso_system_validator_next_epoch_worker_address">next_epoch_worker_address</a>
}
</code></pre>



</details>

<a name="myso_system_validator_next_epoch_protocol_pubkey_bytes"></a>

## Function `next_epoch_protocol_pubkey_bytes`



<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_next_epoch_protocol_pubkey_bytes">next_epoch_protocol_pubkey_bytes</a>(self: &<a href="../myso_system/validator.md#myso_system_validator_Validator">myso_system::validator::Validator</a>): &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;u8&gt;&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_next_epoch_protocol_pubkey_bytes">next_epoch_protocol_pubkey_bytes</a>(self: &<a href="../myso_system/validator.md#myso_system_validator_Validator">Validator</a>): &Option&lt;vector&lt;u8&gt;&gt; {
    &self.<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>.<a href="../myso_system/validator.md#myso_system_validator_next_epoch_protocol_pubkey_bytes">next_epoch_protocol_pubkey_bytes</a>
}
</code></pre>



</details>

<a name="myso_system_validator_next_epoch_proof_of_possession"></a>

## Function `next_epoch_proof_of_possession`



<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_next_epoch_proof_of_possession">next_epoch_proof_of_possession</a>(self: &<a href="../myso_system/validator.md#myso_system_validator_Validator">myso_system::validator::Validator</a>): &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;u8&gt;&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_next_epoch_proof_of_possession">next_epoch_proof_of_possession</a>(self: &<a href="../myso_system/validator.md#myso_system_validator_Validator">Validator</a>): &Option&lt;vector&lt;u8&gt;&gt; {
    &self.<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>.<a href="../myso_system/validator.md#myso_system_validator_next_epoch_proof_of_possession">next_epoch_proof_of_possession</a>
}
</code></pre>



</details>

<a name="myso_system_validator_next_epoch_network_pubkey_bytes"></a>

## Function `next_epoch_network_pubkey_bytes`



<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_next_epoch_network_pubkey_bytes">next_epoch_network_pubkey_bytes</a>(self: &<a href="../myso_system/validator.md#myso_system_validator_Validator">myso_system::validator::Validator</a>): &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;u8&gt;&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_next_epoch_network_pubkey_bytes">next_epoch_network_pubkey_bytes</a>(self: &<a href="../myso_system/validator.md#myso_system_validator_Validator">Validator</a>): &Option&lt;vector&lt;u8&gt;&gt; {
    &self.<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>.<a href="../myso_system/validator.md#myso_system_validator_next_epoch_network_pubkey_bytes">next_epoch_network_pubkey_bytes</a>
}
</code></pre>



</details>

<a name="myso_system_validator_next_epoch_worker_pubkey_bytes"></a>

## Function `next_epoch_worker_pubkey_bytes`



<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_next_epoch_worker_pubkey_bytes">next_epoch_worker_pubkey_bytes</a>(self: &<a href="../myso_system/validator.md#myso_system_validator_Validator">myso_system::validator::Validator</a>): &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;u8&gt;&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_next_epoch_worker_pubkey_bytes">next_epoch_worker_pubkey_bytes</a>(self: &<a href="../myso_system/validator.md#myso_system_validator_Validator">Validator</a>): &Option&lt;vector&lt;u8&gt;&gt; {
    &self.<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>.<a href="../myso_system/validator.md#myso_system_validator_next_epoch_worker_pubkey_bytes">next_epoch_worker_pubkey_bytes</a>
}
</code></pre>



</details>

<a name="myso_system_validator_operation_cap_id"></a>

## Function `operation_cap_id`



<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_operation_cap_id">operation_cap_id</a>(self: &<a href="../myso_system/validator.md#myso_system_validator_Validator">myso_system::validator::Validator</a>): &<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_operation_cap_id">operation_cap_id</a>(self: &<a href="../myso_system/validator.md#myso_system_validator_Validator">Validator</a>): &ID {
    &self.<a href="../myso_system/validator.md#myso_system_validator_operation_cap_id">operation_cap_id</a>
}
</code></pre>



</details>

<a name="myso_system_validator_next_epoch_gas_price"></a>

## Function `next_epoch_gas_price`



<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_next_epoch_gas_price">next_epoch_gas_price</a>(self: &<a href="../myso_system/validator.md#myso_system_validator_Validator">myso_system::validator::Validator</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_next_epoch_gas_price">next_epoch_gas_price</a>(self: &<a href="../myso_system/validator.md#myso_system_validator_Validator">Validator</a>): u64 {
    self.<a href="../myso_system/validator.md#myso_system_validator_next_epoch_gas_price">next_epoch_gas_price</a>
}
</code></pre>



</details>

<a name="myso_system_validator_total_stake_amount"></a>

## Function `total_stake_amount`



<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_total_stake_amount">total_stake_amount</a>(self: &<a href="../myso_system/validator.md#myso_system_validator_Validator">myso_system::validator::Validator</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_total_stake_amount">total_stake_amount</a>(self: &<a href="../myso_system/validator.md#myso_system_validator_Validator">Validator</a>): u64 {
    self.<a href="../myso_system/staking_pool.md#myso_system_staking_pool">staking_pool</a>.myso_balance()
}
</code></pre>



</details>

<a name="myso_system_validator_stake_amount"></a>

## Function `stake_amount`



<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_stake_amount">stake_amount</a>(self: &<a href="../myso_system/validator.md#myso_system_validator_Validator">myso_system::validator::Validator</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_stake_amount">stake_amount</a>(self: &<a href="../myso_system/validator.md#myso_system_validator_Validator">Validator</a>): u64 {
    self.<a href="../myso_system/staking_pool.md#myso_system_staking_pool">staking_pool</a>.myso_balance()
}
</code></pre>



</details>

<a name="myso_system_validator_total_stake"></a>

## Function `total_stake`

Return the total amount staked with this validator


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_total_stake">total_stake</a>(self: &<a href="../myso_system/validator.md#myso_system_validator_Validator">myso_system::validator::Validator</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_total_stake">total_stake</a>(self: &<a href="../myso_system/validator.md#myso_system_validator_Validator">Validator</a>): u64 {
    self.<a href="../myso_system/staking_pool.md#myso_system_staking_pool">staking_pool</a>.myso_balance()
}
</code></pre>



</details>

<a name="myso_system_validator_voting_power"></a>

## Function `voting_power`

Return the voting power of this validator.


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/voting_power.md#myso_system_voting_power">voting_power</a>(self: &<a href="../myso_system/validator.md#myso_system_validator_Validator">myso_system::validator::Validator</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/voting_power.md#myso_system_voting_power">voting_power</a>(self: &<a href="../myso_system/validator.md#myso_system_validator_Validator">Validator</a>): u64 {
    self.<a href="../myso_system/voting_power.md#myso_system_voting_power">voting_power</a>
}
</code></pre>



</details>

<a name="myso_system_validator_set_voting_power"></a>

## Function `set_voting_power`

Set the voting power of this validator, called only from validator_set.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_set_voting_power">set_voting_power</a>(self: &<b>mut</b> <a href="../myso_system/validator.md#myso_system_validator_Validator">myso_system::validator::Validator</a>, new_voting_power: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_set_voting_power">set_voting_power</a>(self: &<b>mut</b> <a href="../myso_system/validator.md#myso_system_validator_Validator">Validator</a>, new_voting_power: u64) {
    self.<a href="../myso_system/voting_power.md#myso_system_voting_power">voting_power</a> = new_voting_power;
}
</code></pre>



</details>

<a name="myso_system_validator_pending_stake_amount"></a>

## Function `pending_stake_amount`



<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_pending_stake_amount">pending_stake_amount</a>(self: &<a href="../myso_system/validator.md#myso_system_validator_Validator">myso_system::validator::Validator</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_pending_stake_amount">pending_stake_amount</a>(self: &<a href="../myso_system/validator.md#myso_system_validator_Validator">Validator</a>): u64 {
    self.<a href="../myso_system/staking_pool.md#myso_system_staking_pool">staking_pool</a>.<a href="../myso_system/validator.md#myso_system_validator_pending_stake_amount">pending_stake_amount</a>()
}
</code></pre>



</details>

<a name="myso_system_validator_pending_stake_withdraw_amount"></a>

## Function `pending_stake_withdraw_amount`



<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_pending_stake_withdraw_amount">pending_stake_withdraw_amount</a>(self: &<a href="../myso_system/validator.md#myso_system_validator_Validator">myso_system::validator::Validator</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_pending_stake_withdraw_amount">pending_stake_withdraw_amount</a>(self: &<a href="../myso_system/validator.md#myso_system_validator_Validator">Validator</a>): u64 {
    self.<a href="../myso_system/staking_pool.md#myso_system_staking_pool">staking_pool</a>.<a href="../myso_system/validator.md#myso_system_validator_pending_stake_withdraw_amount">pending_stake_withdraw_amount</a>()
}
</code></pre>



</details>

<a name="myso_system_validator_gas_price"></a>

## Function `gas_price`



<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_gas_price">gas_price</a>(self: &<a href="../myso_system/validator.md#myso_system_validator_Validator">myso_system::validator::Validator</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_gas_price">gas_price</a>(self: &<a href="../myso_system/validator.md#myso_system_validator_Validator">Validator</a>): u64 {
    self.<a href="../myso_system/validator.md#myso_system_validator_gas_price">gas_price</a>
}
</code></pre>



</details>

<a name="myso_system_validator_commission_rate"></a>

## Function `commission_rate`



<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_commission_rate">commission_rate</a>(self: &<a href="../myso_system/validator.md#myso_system_validator_Validator">myso_system::validator::Validator</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_commission_rate">commission_rate</a>(self: &<a href="../myso_system/validator.md#myso_system_validator_Validator">Validator</a>): u64 {
    self.<a href="../myso_system/validator.md#myso_system_validator_commission_rate">commission_rate</a>
}
</code></pre>



</details>

<a name="myso_system_validator_pool_token_exchange_rate_at_epoch"></a>

## Function `pool_token_exchange_rate_at_epoch`



<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_pool_token_exchange_rate_at_epoch">pool_token_exchange_rate_at_epoch</a>(self: &<a href="../myso_system/validator.md#myso_system_validator_Validator">myso_system::validator::Validator</a>, epoch: u64): <a href="../myso_system/staking_pool.md#myso_system_staking_pool_PoolTokenExchangeRate">myso_system::staking_pool::PoolTokenExchangeRate</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_pool_token_exchange_rate_at_epoch">pool_token_exchange_rate_at_epoch</a>(self: &<a href="../myso_system/validator.md#myso_system_validator_Validator">Validator</a>, epoch: u64): PoolTokenExchangeRate {
    self.<a href="../myso_system/staking_pool.md#myso_system_staking_pool">staking_pool</a>.<a href="../myso_system/validator.md#myso_system_validator_pool_token_exchange_rate_at_epoch">pool_token_exchange_rate_at_epoch</a>(epoch)
}
</code></pre>



</details>

<a name="myso_system_validator_staking_pool_id"></a>

## Function `staking_pool_id`



<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_staking_pool_id">staking_pool_id</a>(self: &<a href="../myso_system/validator.md#myso_system_validator_Validator">myso_system::validator::Validator</a>): <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_staking_pool_id">staking_pool_id</a>(self: &<a href="../myso_system/validator.md#myso_system_validator_Validator">Validator</a>): ID {
    object::id(&self.<a href="../myso_system/staking_pool.md#myso_system_staking_pool">staking_pool</a>)
}
</code></pre>



</details>

<a name="myso_system_validator_is_duplicate"></a>

## Function `is_duplicate`



<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_is_duplicate">is_duplicate</a>(self: &<a href="../myso_system/validator.md#myso_system_validator_Validator">myso_system::validator::Validator</a>, other: &<a href="../myso_system/validator.md#myso_system_validator_Validator">myso_system::validator::Validator</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_is_duplicate">is_duplicate</a>(self: &<a href="../myso_system/validator.md#myso_system_validator_Validator">Validator</a>, other: &<a href="../myso_system/validator.md#myso_system_validator_Validator">Validator</a>): bool {
    <b>let</b> self = &self.<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>;
    <b>let</b> other = &other.<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>;
    self.<a href="../myso_system/validator.md#myso_system_validator_myso_address">myso_address</a> == other.<a href="../myso_system/validator.md#myso_system_validator_myso_address">myso_address</a>
        || self.<a href="../myso_system/validator.md#myso_system_validator_name">name</a> == other.<a href="../myso_system/validator.md#myso_system_validator_name">name</a>
        || self.net_address == other.net_address
        || self.<a href="../myso_system/validator.md#myso_system_validator_p2p_address">p2p_address</a> == other.<a href="../myso_system/validator.md#myso_system_validator_p2p_address">p2p_address</a>
        || self.<a href="../myso_system/validator.md#myso_system_validator_protocol_pubkey_bytes">protocol_pubkey_bytes</a> == other.<a href="../myso_system/validator.md#myso_system_validator_protocol_pubkey_bytes">protocol_pubkey_bytes</a>
        || self.<a href="../myso_system/validator.md#myso_system_validator_network_pubkey_bytes">network_pubkey_bytes</a> == other.<a href="../myso_system/validator.md#myso_system_validator_network_pubkey_bytes">network_pubkey_bytes</a>
        || self.<a href="../myso_system/validator.md#myso_system_validator_network_pubkey_bytes">network_pubkey_bytes</a> == other.<a href="../myso_system/validator.md#myso_system_validator_worker_pubkey_bytes">worker_pubkey_bytes</a>
        || self.<a href="../myso_system/validator.md#myso_system_validator_worker_pubkey_bytes">worker_pubkey_bytes</a> == other.<a href="../myso_system/validator.md#myso_system_validator_worker_pubkey_bytes">worker_pubkey_bytes</a>
        || self.<a href="../myso_system/validator.md#myso_system_validator_worker_pubkey_bytes">worker_pubkey_bytes</a> == other.<a href="../myso_system/validator.md#myso_system_validator_network_pubkey_bytes">network_pubkey_bytes</a>
        // All next epoch parameters.
        || <a href="../myso_system/validator.md#myso_system_validator_both_some_and_equal">both_some_and_equal</a>!(self.next_epoch_net_address, other.next_epoch_net_address)
        || <a href="../myso_system/validator.md#myso_system_validator_both_some_and_equal">both_some_and_equal</a>!(self.<a href="../myso_system/validator.md#myso_system_validator_next_epoch_p2p_address">next_epoch_p2p_address</a>, other.<a href="../myso_system/validator.md#myso_system_validator_next_epoch_p2p_address">next_epoch_p2p_address</a>)
        || <a href="../myso_system/validator.md#myso_system_validator_both_some_and_equal">both_some_and_equal</a>!(self.<a href="../myso_system/validator.md#myso_system_validator_next_epoch_protocol_pubkey_bytes">next_epoch_protocol_pubkey_bytes</a>, other.<a href="../myso_system/validator.md#myso_system_validator_next_epoch_protocol_pubkey_bytes">next_epoch_protocol_pubkey_bytes</a>)
        || <a href="../myso_system/validator.md#myso_system_validator_both_some_and_equal">both_some_and_equal</a>!(self.<a href="../myso_system/validator.md#myso_system_validator_next_epoch_network_pubkey_bytes">next_epoch_network_pubkey_bytes</a>, other.<a href="../myso_system/validator.md#myso_system_validator_next_epoch_network_pubkey_bytes">next_epoch_network_pubkey_bytes</a>)
        || <a href="../myso_system/validator.md#myso_system_validator_both_some_and_equal">both_some_and_equal</a>!(self.<a href="../myso_system/validator.md#myso_system_validator_next_epoch_network_pubkey_bytes">next_epoch_network_pubkey_bytes</a>, other.<a href="../myso_system/validator.md#myso_system_validator_next_epoch_worker_pubkey_bytes">next_epoch_worker_pubkey_bytes</a>)
        || <a href="../myso_system/validator.md#myso_system_validator_both_some_and_equal">both_some_and_equal</a>!(self.<a href="../myso_system/validator.md#myso_system_validator_next_epoch_worker_pubkey_bytes">next_epoch_worker_pubkey_bytes</a>, other.<a href="../myso_system/validator.md#myso_system_validator_next_epoch_worker_pubkey_bytes">next_epoch_worker_pubkey_bytes</a>)
        || <a href="../myso_system/validator.md#myso_system_validator_both_some_and_equal">both_some_and_equal</a>!(self.<a href="../myso_system/validator.md#myso_system_validator_next_epoch_worker_pubkey_bytes">next_epoch_worker_pubkey_bytes</a>, other.<a href="../myso_system/validator.md#myso_system_validator_next_epoch_network_pubkey_bytes">next_epoch_network_pubkey_bytes</a>)
        // My next epoch parameters with other current epoch parameters.
        || self.next_epoch_net_address.is_some_and!(|v| v == other.net_address)
        || self.<a href="../myso_system/validator.md#myso_system_validator_next_epoch_p2p_address">next_epoch_p2p_address</a>.is_some_and!(|v| v == other.<a href="../myso_system/validator.md#myso_system_validator_p2p_address">p2p_address</a>)
        || self.<a href="../myso_system/validator.md#myso_system_validator_next_epoch_protocol_pubkey_bytes">next_epoch_protocol_pubkey_bytes</a>.is_some_and!(|v| v == other.<a href="../myso_system/validator.md#myso_system_validator_protocol_pubkey_bytes">protocol_pubkey_bytes</a>)
        || self.<a href="../myso_system/validator.md#myso_system_validator_next_epoch_network_pubkey_bytes">next_epoch_network_pubkey_bytes</a>.is_some_and!(|v| v == other.<a href="../myso_system/validator.md#myso_system_validator_network_pubkey_bytes">network_pubkey_bytes</a>)
        || self.<a href="../myso_system/validator.md#myso_system_validator_next_epoch_network_pubkey_bytes">next_epoch_network_pubkey_bytes</a>.is_some_and!(|v| v == other.<a href="../myso_system/validator.md#myso_system_validator_worker_pubkey_bytes">worker_pubkey_bytes</a>)
        || self.<a href="../myso_system/validator.md#myso_system_validator_next_epoch_worker_pubkey_bytes">next_epoch_worker_pubkey_bytes</a>.is_some_and!(|v| v == other.<a href="../myso_system/validator.md#myso_system_validator_worker_pubkey_bytes">worker_pubkey_bytes</a>)
        || self.<a href="../myso_system/validator.md#myso_system_validator_next_epoch_worker_pubkey_bytes">next_epoch_worker_pubkey_bytes</a>.is_some_and!(|v| v == other.<a href="../myso_system/validator.md#myso_system_validator_network_pubkey_bytes">network_pubkey_bytes</a>)
        // Other next epoch parameters with my current epoch parameters.
        || other.next_epoch_net_address.is_some_and!(|v| v == self.net_address)
        || other.<a href="../myso_system/validator.md#myso_system_validator_next_epoch_p2p_address">next_epoch_p2p_address</a>.is_some_and!(|v| v == self.<a href="../myso_system/validator.md#myso_system_validator_p2p_address">p2p_address</a>)
        || other.<a href="../myso_system/validator.md#myso_system_validator_next_epoch_protocol_pubkey_bytes">next_epoch_protocol_pubkey_bytes</a>.is_some_and!(|v| v == self.<a href="../myso_system/validator.md#myso_system_validator_protocol_pubkey_bytes">protocol_pubkey_bytes</a>)
        || other.<a href="../myso_system/validator.md#myso_system_validator_next_epoch_network_pubkey_bytes">next_epoch_network_pubkey_bytes</a>.is_some_and!(|v| v == self.<a href="../myso_system/validator.md#myso_system_validator_network_pubkey_bytes">network_pubkey_bytes</a>)
        || other.<a href="../myso_system/validator.md#myso_system_validator_next_epoch_network_pubkey_bytes">next_epoch_network_pubkey_bytes</a>.is_some_and!(|v| v == self.<a href="../myso_system/validator.md#myso_system_validator_worker_pubkey_bytes">worker_pubkey_bytes</a>)
        || other.<a href="../myso_system/validator.md#myso_system_validator_next_epoch_worker_pubkey_bytes">next_epoch_worker_pubkey_bytes</a>.is_some_and!(|v| v == self.<a href="../myso_system/validator.md#myso_system_validator_worker_pubkey_bytes">worker_pubkey_bytes</a>)
        || other.<a href="../myso_system/validator.md#myso_system_validator_next_epoch_worker_pubkey_bytes">next_epoch_worker_pubkey_bytes</a>.is_some_and!(|v| v == self.<a href="../myso_system/validator.md#myso_system_validator_network_pubkey_bytes">network_pubkey_bytes</a>)
}
</code></pre>



</details>

<a name="myso_system_validator_both_some_and_equal"></a>

## Macro function `both_some_and_equal`



<pre><code><b>macro</b> <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_both_some_and_equal">both_some_and_equal</a>&lt;$T&gt;($a: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;$T&gt;, $b: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;$T&gt;): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>macro</b> <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_both_some_and_equal">both_some_and_equal</a>&lt;$T&gt;($a: Option&lt;$T&gt;, $b: Option&lt;$T&gt;): bool {
    <b>let</b> (a, b) = ($a, $b);
    a.is_some_and!(|a| b.is_some_and!(|b| a == b))
}
</code></pre>



</details>

<a name="myso_system_validator_new_unverified_validator_operation_cap_and_transfer"></a>

## Function `new_unverified_validator_operation_cap_and_transfer`

Create a new <code>UnverifiedValidatorOperationCap</code>, transfer to the validator,
and registers it, thus revoking the previous cap's permission.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_new_unverified_validator_operation_cap_and_transfer">new_unverified_validator_operation_cap_and_transfer</a>(self: &<b>mut</b> <a href="../myso_system/validator.md#myso_system_validator_Validator">myso_system::validator::Validator</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_new_unverified_validator_operation_cap_and_transfer">new_unverified_validator_operation_cap_and_transfer</a>(
    self: &<b>mut</b> <a href="../myso_system/validator.md#myso_system_validator_Validator">Validator</a>,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> sender = ctx.sender();
    <b>assert</b>!(sender == self.<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>.<a href="../myso_system/validator.md#myso_system_validator_myso_address">myso_address</a>, <a href="../myso_system/validator.md#myso_system_validator_ENewCapNotCreatedByValidatorItself">ENewCapNotCreatedByValidatorItself</a>);
    <b>let</b> new_id = <a href="../myso_system/validator_cap.md#myso_system_validator_cap_new_unverified_validator_operation_cap_and_transfer">validator_cap::new_unverified_validator_operation_cap_and_transfer</a>(sender, ctx);
    self.<a href="../myso_system/validator.md#myso_system_validator_operation_cap_id">operation_cap_id</a> = new_id;
}
</code></pre>



</details>

<a name="myso_system_validator_update_name"></a>

## Function `update_name`

Update name of the validator.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_update_name">update_name</a>(self: &<b>mut</b> <a href="../myso_system/validator.md#myso_system_validator_Validator">myso_system::validator::Validator</a>, <a href="../myso_system/validator.md#myso_system_validator_name">name</a>: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_update_name">update_name</a>(self: &<b>mut</b> <a href="../myso_system/validator.md#myso_system_validator_Validator">Validator</a>, <a href="../myso_system/validator.md#myso_system_validator_name">name</a>: vector&lt;u8&gt;) {
    <b>assert</b>!(<a href="../myso_system/validator.md#myso_system_validator_name">name</a>.length() &lt;= <a href="../myso_system/validator.md#myso_system_validator_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>, <a href="../myso_system/validator.md#myso_system_validator_EValidatorMetadataExceedingLengthLimit">EValidatorMetadataExceedingLengthLimit</a>);
    self.<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>.<a href="../myso_system/validator.md#myso_system_validator_name">name</a> = <a href="../myso_system/validator.md#myso_system_validator_name">name</a>.to_ascii_string().to_string();
}
</code></pre>



</details>

<a name="myso_system_validator_update_description"></a>

## Function `update_description`

Update description of the validator.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_update_description">update_description</a>(self: &<b>mut</b> <a href="../myso_system/validator.md#myso_system_validator_Validator">myso_system::validator::Validator</a>, <a href="../myso_system/validator.md#myso_system_validator_description">description</a>: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_update_description">update_description</a>(self: &<b>mut</b> <a href="../myso_system/validator.md#myso_system_validator_Validator">Validator</a>, <a href="../myso_system/validator.md#myso_system_validator_description">description</a>: vector&lt;u8&gt;) {
    <b>assert</b>!(
        <a href="../myso_system/validator.md#myso_system_validator_description">description</a>.length() &lt;= <a href="../myso_system/validator.md#myso_system_validator_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>,
        <a href="../myso_system/validator.md#myso_system_validator_EValidatorMetadataExceedingLengthLimit">EValidatorMetadataExceedingLengthLimit</a>,
    );
    self.<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>.<a href="../myso_system/validator.md#myso_system_validator_description">description</a> = <a href="../myso_system/validator.md#myso_system_validator_description">description</a>.to_ascii_string().to_string();
}
</code></pre>



</details>

<a name="myso_system_validator_update_image_url"></a>

## Function `update_image_url`

Update image url of the validator.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_update_image_url">update_image_url</a>(self: &<b>mut</b> <a href="../myso_system/validator.md#myso_system_validator_Validator">myso_system::validator::Validator</a>, <a href="../myso_system/validator.md#myso_system_validator_image_url">image_url</a>: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_update_image_url">update_image_url</a>(self: &<b>mut</b> <a href="../myso_system/validator.md#myso_system_validator_Validator">Validator</a>, <a href="../myso_system/validator.md#myso_system_validator_image_url">image_url</a>: vector&lt;u8&gt;) {
    <b>assert</b>!(
        <a href="../myso_system/validator.md#myso_system_validator_image_url">image_url</a>.length() &lt;= <a href="../myso_system/validator.md#myso_system_validator_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>,
        <a href="../myso_system/validator.md#myso_system_validator_EValidatorMetadataExceedingLengthLimit">EValidatorMetadataExceedingLengthLimit</a>,
    );
    self.<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>.<a href="../myso_system/validator.md#myso_system_validator_image_url">image_url</a> = url::new_unsafe_from_bytes(<a href="../myso_system/validator.md#myso_system_validator_image_url">image_url</a>);
}
</code></pre>



</details>

<a name="myso_system_validator_update_project_url"></a>

## Function `update_project_url`

Update project url of the validator.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_update_project_url">update_project_url</a>(self: &<b>mut</b> <a href="../myso_system/validator.md#myso_system_validator_Validator">myso_system::validator::Validator</a>, <a href="../myso_system/validator.md#myso_system_validator_project_url">project_url</a>: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_update_project_url">update_project_url</a>(self: &<b>mut</b> <a href="../myso_system/validator.md#myso_system_validator_Validator">Validator</a>, <a href="../myso_system/validator.md#myso_system_validator_project_url">project_url</a>: vector&lt;u8&gt;) {
    <b>assert</b>!(
        <a href="../myso_system/validator.md#myso_system_validator_project_url">project_url</a>.length() &lt;= <a href="../myso_system/validator.md#myso_system_validator_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>,
        <a href="../myso_system/validator.md#myso_system_validator_EValidatorMetadataExceedingLengthLimit">EValidatorMetadataExceedingLengthLimit</a>,
    );
    self.<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>.<a href="../myso_system/validator.md#myso_system_validator_project_url">project_url</a> = url::new_unsafe_from_bytes(<a href="../myso_system/validator.md#myso_system_validator_project_url">project_url</a>);
}
</code></pre>



</details>

<a name="myso_system_validator_update_next_epoch_network_address"></a>

## Function `update_next_epoch_network_address`

Update network address of this validator, taking effects from next epoch


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_update_next_epoch_network_address">update_next_epoch_network_address</a>(self: &<b>mut</b> <a href="../myso_system/validator.md#myso_system_validator_Validator">myso_system::validator::Validator</a>, net_address: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_update_next_epoch_network_address">update_next_epoch_network_address</a>(
    self: &<b>mut</b> <a href="../myso_system/validator.md#myso_system_validator_Validator">Validator</a>,
    net_address: vector&lt;u8&gt;,
) {
    <b>assert</b>!(
        net_address.length() &lt;= <a href="../myso_system/validator.md#myso_system_validator_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>,
        <a href="../myso_system/validator.md#myso_system_validator_EValidatorMetadataExceedingLengthLimit">EValidatorMetadataExceedingLengthLimit</a>,
    );
    <b>let</b> net_address = net_address.to_ascii_string().to_string();
    self.<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>.next_epoch_net_address = option::some(net_address);
    self.<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>.validate();
}
</code></pre>



</details>

<a name="myso_system_validator_update_candidate_network_address"></a>

## Function `update_candidate_network_address`

Update network address of this candidate validator


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_update_candidate_network_address">update_candidate_network_address</a>(self: &<b>mut</b> <a href="../myso_system/validator.md#myso_system_validator_Validator">myso_system::validator::Validator</a>, net_address: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_update_candidate_network_address">update_candidate_network_address</a>(
    self: &<b>mut</b> <a href="../myso_system/validator.md#myso_system_validator_Validator">Validator</a>,
    net_address: vector&lt;u8&gt;,
) {
    <b>assert</b>!(self.<a href="../myso_system/validator.md#myso_system_validator_is_preactive">is_preactive</a>(), <a href="../myso_system/validator.md#myso_system_validator_ENotValidatorCandidate">ENotValidatorCandidate</a>);
    <b>assert</b>!(
        net_address.length() &lt;= <a href="../myso_system/validator.md#myso_system_validator_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>,
        <a href="../myso_system/validator.md#myso_system_validator_EValidatorMetadataExceedingLengthLimit">EValidatorMetadataExceedingLengthLimit</a>,
    );
    <b>let</b> net_address = net_address.to_ascii_string().to_string();
    self.<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>.net_address = net_address;
    self.<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>.validate();
}
</code></pre>



</details>

<a name="myso_system_validator_update_next_epoch_p2p_address"></a>

## Function `update_next_epoch_p2p_address`

Update p2p address of this validator, taking effects from next epoch


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_update_next_epoch_p2p_address">update_next_epoch_p2p_address</a>(self: &<b>mut</b> <a href="../myso_system/validator.md#myso_system_validator_Validator">myso_system::validator::Validator</a>, <a href="../myso_system/validator.md#myso_system_validator_p2p_address">p2p_address</a>: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_update_next_epoch_p2p_address">update_next_epoch_p2p_address</a>(self: &<b>mut</b> <a href="../myso_system/validator.md#myso_system_validator_Validator">Validator</a>, <a href="../myso_system/validator.md#myso_system_validator_p2p_address">p2p_address</a>: vector&lt;u8&gt;) {
    <b>assert</b>!(
        <a href="../myso_system/validator.md#myso_system_validator_p2p_address">p2p_address</a>.length() &lt;= <a href="../myso_system/validator.md#myso_system_validator_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>,
        <a href="../myso_system/validator.md#myso_system_validator_EValidatorMetadataExceedingLengthLimit">EValidatorMetadataExceedingLengthLimit</a>,
    );
    <b>let</b> <a href="../myso_system/validator.md#myso_system_validator_p2p_address">p2p_address</a> = <a href="../myso_system/validator.md#myso_system_validator_p2p_address">p2p_address</a>.to_ascii_string().to_string();
    self.<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>.<a href="../myso_system/validator.md#myso_system_validator_next_epoch_p2p_address">next_epoch_p2p_address</a> = option::some(<a href="../myso_system/validator.md#myso_system_validator_p2p_address">p2p_address</a>);
    self.<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>.validate();
}
</code></pre>



</details>

<a name="myso_system_validator_update_candidate_p2p_address"></a>

## Function `update_candidate_p2p_address`

Update p2p address of this candidate validator


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_update_candidate_p2p_address">update_candidate_p2p_address</a>(self: &<b>mut</b> <a href="../myso_system/validator.md#myso_system_validator_Validator">myso_system::validator::Validator</a>, <a href="../myso_system/validator.md#myso_system_validator_p2p_address">p2p_address</a>: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_update_candidate_p2p_address">update_candidate_p2p_address</a>(self: &<b>mut</b> <a href="../myso_system/validator.md#myso_system_validator_Validator">Validator</a>, <a href="../myso_system/validator.md#myso_system_validator_p2p_address">p2p_address</a>: vector&lt;u8&gt;) {
    <b>assert</b>!(self.<a href="../myso_system/validator.md#myso_system_validator_is_preactive">is_preactive</a>(), <a href="../myso_system/validator.md#myso_system_validator_ENotValidatorCandidate">ENotValidatorCandidate</a>);
    <b>assert</b>!(
        <a href="../myso_system/validator.md#myso_system_validator_p2p_address">p2p_address</a>.length() &lt;= <a href="../myso_system/validator.md#myso_system_validator_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>,
        <a href="../myso_system/validator.md#myso_system_validator_EValidatorMetadataExceedingLengthLimit">EValidatorMetadataExceedingLengthLimit</a>,
    );
    <b>let</b> <a href="../myso_system/validator.md#myso_system_validator_p2p_address">p2p_address</a> = <a href="../myso_system/validator.md#myso_system_validator_p2p_address">p2p_address</a>.to_ascii_string().to_string();
    self.<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>.<a href="../myso_system/validator.md#myso_system_validator_p2p_address">p2p_address</a> = <a href="../myso_system/validator.md#myso_system_validator_p2p_address">p2p_address</a>;
    self.<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>.validate();
}
</code></pre>



</details>

<a name="myso_system_validator_update_next_epoch_primary_address"></a>

## Function `update_next_epoch_primary_address`

Update primary address of this validator, taking effects from next epoch


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_update_next_epoch_primary_address">update_next_epoch_primary_address</a>(self: &<b>mut</b> <a href="../myso_system/validator.md#myso_system_validator_Validator">myso_system::validator::Validator</a>, <a href="../myso_system/validator.md#myso_system_validator_primary_address">primary_address</a>: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_update_next_epoch_primary_address">update_next_epoch_primary_address</a>(
    self: &<b>mut</b> <a href="../myso_system/validator.md#myso_system_validator_Validator">Validator</a>,
    <a href="../myso_system/validator.md#myso_system_validator_primary_address">primary_address</a>: vector&lt;u8&gt;,
) {
    <b>assert</b>!(
        <a href="../myso_system/validator.md#myso_system_validator_primary_address">primary_address</a>.length() &lt;= <a href="../myso_system/validator.md#myso_system_validator_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>,
        <a href="../myso_system/validator.md#myso_system_validator_EValidatorMetadataExceedingLengthLimit">EValidatorMetadataExceedingLengthLimit</a>,
    );
    <b>let</b> <a href="../myso_system/validator.md#myso_system_validator_primary_address">primary_address</a> = <a href="../myso_system/validator.md#myso_system_validator_primary_address">primary_address</a>.to_ascii_string().to_string();
    self.<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>.<a href="../myso_system/validator.md#myso_system_validator_next_epoch_primary_address">next_epoch_primary_address</a> = option::some(<a href="../myso_system/validator.md#myso_system_validator_primary_address">primary_address</a>);
    self.<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>.validate();
}
</code></pre>



</details>

<a name="myso_system_validator_update_candidate_primary_address"></a>

## Function `update_candidate_primary_address`

Update primary address of this candidate validator


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_update_candidate_primary_address">update_candidate_primary_address</a>(self: &<b>mut</b> <a href="../myso_system/validator.md#myso_system_validator_Validator">myso_system::validator::Validator</a>, <a href="../myso_system/validator.md#myso_system_validator_primary_address">primary_address</a>: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_update_candidate_primary_address">update_candidate_primary_address</a>(
    self: &<b>mut</b> <a href="../myso_system/validator.md#myso_system_validator_Validator">Validator</a>,
    <a href="../myso_system/validator.md#myso_system_validator_primary_address">primary_address</a>: vector&lt;u8&gt;,
) {
    <b>assert</b>!(self.<a href="../myso_system/validator.md#myso_system_validator_is_preactive">is_preactive</a>(), <a href="../myso_system/validator.md#myso_system_validator_ENotValidatorCandidate">ENotValidatorCandidate</a>);
    <b>assert</b>!(
        <a href="../myso_system/validator.md#myso_system_validator_primary_address">primary_address</a>.length() &lt;= <a href="../myso_system/validator.md#myso_system_validator_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>,
        <a href="../myso_system/validator.md#myso_system_validator_EValidatorMetadataExceedingLengthLimit">EValidatorMetadataExceedingLengthLimit</a>,
    );
    <b>let</b> <a href="../myso_system/validator.md#myso_system_validator_primary_address">primary_address</a> = <a href="../myso_system/validator.md#myso_system_validator_primary_address">primary_address</a>.to_ascii_string().to_string();
    self.<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>.<a href="../myso_system/validator.md#myso_system_validator_primary_address">primary_address</a> = <a href="../myso_system/validator.md#myso_system_validator_primary_address">primary_address</a>;
    self.<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>.validate();
}
</code></pre>



</details>

<a name="myso_system_validator_update_next_epoch_worker_address"></a>

## Function `update_next_epoch_worker_address`

Update worker address of this validator, taking effects from next epoch


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_update_next_epoch_worker_address">update_next_epoch_worker_address</a>(self: &<b>mut</b> <a href="../myso_system/validator.md#myso_system_validator_Validator">myso_system::validator::Validator</a>, <a href="../myso_system/validator.md#myso_system_validator_worker_address">worker_address</a>: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_update_next_epoch_worker_address">update_next_epoch_worker_address</a>(
    self: &<b>mut</b> <a href="../myso_system/validator.md#myso_system_validator_Validator">Validator</a>,
    <a href="../myso_system/validator.md#myso_system_validator_worker_address">worker_address</a>: vector&lt;u8&gt;,
) {
    <b>assert</b>!(
        <a href="../myso_system/validator.md#myso_system_validator_worker_address">worker_address</a>.length() &lt;= <a href="../myso_system/validator.md#myso_system_validator_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>,
        <a href="../myso_system/validator.md#myso_system_validator_EValidatorMetadataExceedingLengthLimit">EValidatorMetadataExceedingLengthLimit</a>,
    );
    <b>let</b> <a href="../myso_system/validator.md#myso_system_validator_worker_address">worker_address</a> = <a href="../myso_system/validator.md#myso_system_validator_worker_address">worker_address</a>.to_ascii_string().to_string();
    self.<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>.<a href="../myso_system/validator.md#myso_system_validator_next_epoch_worker_address">next_epoch_worker_address</a> = option::some(<a href="../myso_system/validator.md#myso_system_validator_worker_address">worker_address</a>);
    self.<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>.validate();
}
</code></pre>



</details>

<a name="myso_system_validator_update_candidate_worker_address"></a>

## Function `update_candidate_worker_address`

Update worker address of this candidate validator


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_update_candidate_worker_address">update_candidate_worker_address</a>(self: &<b>mut</b> <a href="../myso_system/validator.md#myso_system_validator_Validator">myso_system::validator::Validator</a>, <a href="../myso_system/validator.md#myso_system_validator_worker_address">worker_address</a>: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_update_candidate_worker_address">update_candidate_worker_address</a>(
    self: &<b>mut</b> <a href="../myso_system/validator.md#myso_system_validator_Validator">Validator</a>,
    <a href="../myso_system/validator.md#myso_system_validator_worker_address">worker_address</a>: vector&lt;u8&gt;,
) {
    <b>assert</b>!(self.<a href="../myso_system/validator.md#myso_system_validator_is_preactive">is_preactive</a>(), <a href="../myso_system/validator.md#myso_system_validator_ENotValidatorCandidate">ENotValidatorCandidate</a>);
    <b>assert</b>!(
        <a href="../myso_system/validator.md#myso_system_validator_worker_address">worker_address</a>.length() &lt;= <a href="../myso_system/validator.md#myso_system_validator_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>,
        <a href="../myso_system/validator.md#myso_system_validator_EValidatorMetadataExceedingLengthLimit">EValidatorMetadataExceedingLengthLimit</a>,
    );
    <b>let</b> <a href="../myso_system/validator.md#myso_system_validator_worker_address">worker_address</a> = <a href="../myso_system/validator.md#myso_system_validator_worker_address">worker_address</a>.to_ascii_string().to_string();
    self.<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>.<a href="../myso_system/validator.md#myso_system_validator_worker_address">worker_address</a> = <a href="../myso_system/validator.md#myso_system_validator_worker_address">worker_address</a>;
    self.<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>.validate();
}
</code></pre>



</details>

<a name="myso_system_validator_update_next_epoch_protocol_pubkey"></a>

## Function `update_next_epoch_protocol_pubkey`

Update protocol public key of this validator, taking effects from next epoch


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_update_next_epoch_protocol_pubkey">update_next_epoch_protocol_pubkey</a>(self: &<b>mut</b> <a href="../myso_system/validator.md#myso_system_validator_Validator">myso_system::validator::Validator</a>, protocol_pubkey: vector&lt;u8&gt;, <a href="../myso_system/validator.md#myso_system_validator_proof_of_possession">proof_of_possession</a>: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_update_next_epoch_protocol_pubkey">update_next_epoch_protocol_pubkey</a>(
    self: &<b>mut</b> <a href="../myso_system/validator.md#myso_system_validator_Validator">Validator</a>,
    protocol_pubkey: vector&lt;u8&gt;,
    <a href="../myso_system/validator.md#myso_system_validator_proof_of_possession">proof_of_possession</a>: vector&lt;u8&gt;,
) {
    self.<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>.<a href="../myso_system/validator.md#myso_system_validator_next_epoch_protocol_pubkey_bytes">next_epoch_protocol_pubkey_bytes</a> = option::some(protocol_pubkey);
    self.<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>.<a href="../myso_system/validator.md#myso_system_validator_next_epoch_proof_of_possession">next_epoch_proof_of_possession</a> = option::some(<a href="../myso_system/validator.md#myso_system_validator_proof_of_possession">proof_of_possession</a>);
    self.<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>.validate();
}
</code></pre>



</details>

<a name="myso_system_validator_update_candidate_protocol_pubkey"></a>

## Function `update_candidate_protocol_pubkey`

Update protocol public key of this candidate validator


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_update_candidate_protocol_pubkey">update_candidate_protocol_pubkey</a>(self: &<b>mut</b> <a href="../myso_system/validator.md#myso_system_validator_Validator">myso_system::validator::Validator</a>, protocol_pubkey: vector&lt;u8&gt;, <a href="../myso_system/validator.md#myso_system_validator_proof_of_possession">proof_of_possession</a>: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_update_candidate_protocol_pubkey">update_candidate_protocol_pubkey</a>(
    self: &<b>mut</b> <a href="../myso_system/validator.md#myso_system_validator_Validator">Validator</a>,
    protocol_pubkey: vector&lt;u8&gt;,
    <a href="../myso_system/validator.md#myso_system_validator_proof_of_possession">proof_of_possession</a>: vector&lt;u8&gt;,
) {
    <b>assert</b>!(self.<a href="../myso_system/validator.md#myso_system_validator_is_preactive">is_preactive</a>(), <a href="../myso_system/validator.md#myso_system_validator_ENotValidatorCandidate">ENotValidatorCandidate</a>);
    self.<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>.<a href="../myso_system/validator.md#myso_system_validator_protocol_pubkey_bytes">protocol_pubkey_bytes</a> = protocol_pubkey;
    self.<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>.<a href="../myso_system/validator.md#myso_system_validator_proof_of_possession">proof_of_possession</a> = <a href="../myso_system/validator.md#myso_system_validator_proof_of_possession">proof_of_possession</a>;
    self.<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>.validate();
}
</code></pre>



</details>

<a name="myso_system_validator_update_next_epoch_network_pubkey"></a>

## Function `update_next_epoch_network_pubkey`

Update network public key of this validator, taking effects from next epoch


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_update_next_epoch_network_pubkey">update_next_epoch_network_pubkey</a>(self: &<b>mut</b> <a href="../myso_system/validator.md#myso_system_validator_Validator">myso_system::validator::Validator</a>, network_pubkey: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_update_next_epoch_network_pubkey">update_next_epoch_network_pubkey</a>(
    self: &<b>mut</b> <a href="../myso_system/validator.md#myso_system_validator_Validator">Validator</a>,
    network_pubkey: vector&lt;u8&gt;,
) {
    self.<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>.<a href="../myso_system/validator.md#myso_system_validator_next_epoch_network_pubkey_bytes">next_epoch_network_pubkey_bytes</a> = option::some(network_pubkey);
    self.<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>.validate();
}
</code></pre>



</details>

<a name="myso_system_validator_update_candidate_network_pubkey"></a>

## Function `update_candidate_network_pubkey`

Update network public key of this candidate validator


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_update_candidate_network_pubkey">update_candidate_network_pubkey</a>(self: &<b>mut</b> <a href="../myso_system/validator.md#myso_system_validator_Validator">myso_system::validator::Validator</a>, network_pubkey: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_update_candidate_network_pubkey">update_candidate_network_pubkey</a>(
    self: &<b>mut</b> <a href="../myso_system/validator.md#myso_system_validator_Validator">Validator</a>,
    network_pubkey: vector&lt;u8&gt;,
) {
    <b>assert</b>!(self.<a href="../myso_system/validator.md#myso_system_validator_is_preactive">is_preactive</a>(), <a href="../myso_system/validator.md#myso_system_validator_ENotValidatorCandidate">ENotValidatorCandidate</a>);
    self.<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>.<a href="../myso_system/validator.md#myso_system_validator_network_pubkey_bytes">network_pubkey_bytes</a> = network_pubkey;
    self.<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>.validate();
}
</code></pre>



</details>

<a name="myso_system_validator_update_next_epoch_worker_pubkey"></a>

## Function `update_next_epoch_worker_pubkey`

Update Narwhal worker public key of this validator, taking effects from next epoch


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_update_next_epoch_worker_pubkey">update_next_epoch_worker_pubkey</a>(self: &<b>mut</b> <a href="../myso_system/validator.md#myso_system_validator_Validator">myso_system::validator::Validator</a>, worker_pubkey: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_update_next_epoch_worker_pubkey">update_next_epoch_worker_pubkey</a>(
    self: &<b>mut</b> <a href="../myso_system/validator.md#myso_system_validator_Validator">Validator</a>,
    worker_pubkey: vector&lt;u8&gt;,
) {
    self.<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>.<a href="../myso_system/validator.md#myso_system_validator_next_epoch_worker_pubkey_bytes">next_epoch_worker_pubkey_bytes</a> = option::some(worker_pubkey);
    self.<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>.validate();
}
</code></pre>



</details>

<a name="myso_system_validator_update_candidate_worker_pubkey"></a>

## Function `update_candidate_worker_pubkey`

Update Narwhal worker public key of this candidate validator


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_update_candidate_worker_pubkey">update_candidate_worker_pubkey</a>(self: &<b>mut</b> <a href="../myso_system/validator.md#myso_system_validator_Validator">myso_system::validator::Validator</a>, worker_pubkey: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_update_candidate_worker_pubkey">update_candidate_worker_pubkey</a>(
    self: &<b>mut</b> <a href="../myso_system/validator.md#myso_system_validator_Validator">Validator</a>,
    worker_pubkey: vector&lt;u8&gt;,
) {
    <b>assert</b>!(self.<a href="../myso_system/validator.md#myso_system_validator_is_preactive">is_preactive</a>(), <a href="../myso_system/validator.md#myso_system_validator_ENotValidatorCandidate">ENotValidatorCandidate</a>);
    self.<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>.<a href="../myso_system/validator.md#myso_system_validator_worker_pubkey_bytes">worker_pubkey_bytes</a> = worker_pubkey;
    self.<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>.validate();
}
</code></pre>



</details>

<a name="myso_system_validator_effectuate_staged_metadata"></a>

## Function `effectuate_staged_metadata`

Effectutate all staged next epoch metadata for this validator.
NOTE: this function SHOULD ONLY be called by validator_set when
advancing an epoch.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_effectuate_staged_metadata">effectuate_staged_metadata</a>(self: &<b>mut</b> <a href="../myso_system/validator.md#myso_system_validator_Validator">myso_system::validator::Validator</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_effectuate_staged_metadata">effectuate_staged_metadata</a>(self: &<b>mut</b> <a href="../myso_system/validator.md#myso_system_validator_Validator">Validator</a>) {
    <a href="../myso_system/validator.md#myso_system_validator_do_extract">do_extract</a>!(&<b>mut</b> self.<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>.next_epoch_net_address, |v| {
        self.<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>.net_address = v
    });
    <a href="../myso_system/validator.md#myso_system_validator_do_extract">do_extract</a>!(&<b>mut</b> self.<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>.<a href="../myso_system/validator.md#myso_system_validator_next_epoch_p2p_address">next_epoch_p2p_address</a>, |v| {
        self.<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>.<a href="../myso_system/validator.md#myso_system_validator_p2p_address">p2p_address</a> = v
    });
    <a href="../myso_system/validator.md#myso_system_validator_do_extract">do_extract</a>!(&<b>mut</b> self.<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>.<a href="../myso_system/validator.md#myso_system_validator_next_epoch_primary_address">next_epoch_primary_address</a>, |v| {
        self.<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>.<a href="../myso_system/validator.md#myso_system_validator_primary_address">primary_address</a> = v
    });
    <a href="../myso_system/validator.md#myso_system_validator_do_extract">do_extract</a>!(&<b>mut</b> self.<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>.<a href="../myso_system/validator.md#myso_system_validator_next_epoch_worker_address">next_epoch_worker_address</a>, |v| {
        self.<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>.<a href="../myso_system/validator.md#myso_system_validator_worker_address">worker_address</a> = v
    });
    <a href="../myso_system/validator.md#myso_system_validator_do_extract">do_extract</a>!(&<b>mut</b> self.<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>.<a href="../myso_system/validator.md#myso_system_validator_next_epoch_protocol_pubkey_bytes">next_epoch_protocol_pubkey_bytes</a>, |v| {
        self.<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>.<a href="../myso_system/validator.md#myso_system_validator_protocol_pubkey_bytes">protocol_pubkey_bytes</a> = v;
        self.<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>.<a href="../myso_system/validator.md#myso_system_validator_proof_of_possession">proof_of_possession</a> = self.<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>.<a href="../myso_system/validator.md#myso_system_validator_next_epoch_proof_of_possession">next_epoch_proof_of_possession</a>.extract();
    });
    <a href="../myso_system/validator.md#myso_system_validator_do_extract">do_extract</a>!(&<b>mut</b> self.<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>.<a href="../myso_system/validator.md#myso_system_validator_next_epoch_network_pubkey_bytes">next_epoch_network_pubkey_bytes</a>, |v| {
        self.<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>.<a href="../myso_system/validator.md#myso_system_validator_network_pubkey_bytes">network_pubkey_bytes</a> = v
    });
    <a href="../myso_system/validator.md#myso_system_validator_do_extract">do_extract</a>!(&<b>mut</b> self.<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>.<a href="../myso_system/validator.md#myso_system_validator_next_epoch_worker_pubkey_bytes">next_epoch_worker_pubkey_bytes</a>, |v| {
        self.<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>.<a href="../myso_system/validator.md#myso_system_validator_worker_pubkey_bytes">worker_pubkey_bytes</a> = v
    });
}
</code></pre>



</details>

<a name="myso_system_validator_do_extract"></a>

## Macro function `do_extract`

Helper macro which extracts the value from <code>Some</code> and applies <code>$f</code> to it.


<pre><code><b>macro</b> <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_do_extract">do_extract</a>&lt;$T&gt;($o: &<b>mut</b> <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;$T&gt;, $f: |$T| -&gt; ())
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>macro</b> <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_do_extract">do_extract</a>&lt;$T&gt;($o: &<b>mut</b> Option&lt;$T&gt;, $f: |$T|) {
    <b>let</b> o = $o;
    <b>if</b> (o.is_some()) {
        $f(o.extract());
    }
}
</code></pre>



</details>

<a name="myso_system_validator_validate_metadata"></a>

## Function `validate_metadata`

Aborts if validator metadata is valid


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_validate_metadata">validate_metadata</a>(<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>: &<a href="../myso_system/validator.md#myso_system_validator_ValidatorMetadata">myso_system::validator::ValidatorMetadata</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_validate_metadata">validate_metadata</a>(<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>: &<a href="../myso_system/validator.md#myso_system_validator_ValidatorMetadata">ValidatorMetadata</a>) {
    <a href="../myso_system/validator.md#myso_system_validator_validate_metadata_bcs">validate_metadata_bcs</a>(bcs::to_bytes(<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>));
}
</code></pre>



</details>

<a name="myso_system_validator_validate_metadata_bcs"></a>

## Function `validate_metadata_bcs`



<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_validate_metadata_bcs">validate_metadata_bcs</a>(<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>native</b> <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_validate_metadata_bcs">validate_metadata_bcs</a>(<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>: vector&lt;u8&gt;);
</code></pre>



</details>

<a name="myso_system_validator_get_staking_pool_ref"></a>

## Function `get_staking_pool_ref`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_get_staking_pool_ref">get_staking_pool_ref</a>(self: &<a href="../myso_system/validator.md#myso_system_validator_Validator">myso_system::validator::Validator</a>): &<a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakingPool">myso_system::staking_pool::StakingPool</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_get_staking_pool_ref">get_staking_pool_ref</a>(self: &<a href="../myso_system/validator.md#myso_system_validator_Validator">Validator</a>): &StakingPool {
    &self.<a href="../myso_system/staking_pool.md#myso_system_staking_pool">staking_pool</a>
}
</code></pre>



</details>

<a name="myso_system_validator_new_from_metadata"></a>

## Function `new_from_metadata`

Create a new validator from the given <code><a href="../myso_system/validator.md#myso_system_validator_ValidatorMetadata">ValidatorMetadata</a></code>, called by both <code><a href="../myso_system/validator.md#myso_system_validator_new">new</a></code> and <code>new_for_testing</code>.


<pre><code><b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_new_from_metadata">new_from_metadata</a>(<a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>: <a href="../myso_system/validator.md#myso_system_validator_ValidatorMetadata">myso_system::validator::ValidatorMetadata</a>, <a href="../myso_system/validator.md#myso_system_validator_gas_price">gas_price</a>: u64, <a href="../myso_system/validator.md#myso_system_validator_commission_rate">commission_rate</a>: u64, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../myso_system/validator.md#myso_system_validator_Validator">myso_system::validator::Validator</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../myso_system/validator.md#myso_system_validator_new_from_metadata">new_from_metadata</a>(
    <a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>: <a href="../myso_system/validator.md#myso_system_validator_ValidatorMetadata">ValidatorMetadata</a>,
    <a href="../myso_system/validator.md#myso_system_validator_gas_price">gas_price</a>: u64,
    <a href="../myso_system/validator.md#myso_system_validator_commission_rate">commission_rate</a>: u64,
    ctx: &<b>mut</b> TxContext,
): <a href="../myso_system/validator.md#myso_system_validator_Validator">Validator</a> {
    <b>let</b> <a href="../myso_system/validator.md#myso_system_validator_myso_address">myso_address</a> = <a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>.<a href="../myso_system/validator.md#myso_system_validator_myso_address">myso_address</a>;
    <b>let</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool">staking_pool</a> = <a href="../myso_system/staking_pool.md#myso_system_staking_pool_new">staking_pool::new</a>(ctx);
    <b>let</b> <a href="../myso_system/validator.md#myso_system_validator_operation_cap_id">operation_cap_id</a> = <a href="../myso_system/validator_cap.md#myso_system_validator_cap_new_unverified_validator_operation_cap_and_transfer">validator_cap::new_unverified_validator_operation_cap_and_transfer</a>(
        <a href="../myso_system/validator.md#myso_system_validator_myso_address">myso_address</a>,
        ctx,
    );
    <a href="../myso_system/validator.md#myso_system_validator_Validator">Validator</a> {
        <a href="../myso_system/validator.md#myso_system_validator_metadata">metadata</a>,
        // Initialize the voting power to be 0.
        // At the epoch change where this <a href="../myso_system/validator.md#myso_system_validator">validator</a> is actually added to the
        // active <a href="../myso_system/validator.md#myso_system_validator">validator</a> set, the voting power will be updated accordingly.
        <a href="../myso_system/voting_power.md#myso_system_voting_power">voting_power</a>: 0,
        <a href="../myso_system/validator.md#myso_system_validator_operation_cap_id">operation_cap_id</a>,
        <a href="../myso_system/validator.md#myso_system_validator_gas_price">gas_price</a>,
        <a href="../myso_system/staking_pool.md#myso_system_staking_pool">staking_pool</a>,
        <a href="../myso_system/validator.md#myso_system_validator_commission_rate">commission_rate</a>,
        next_epoch_stake: 0,
        <a href="../myso_system/validator.md#myso_system_validator_next_epoch_gas_price">next_epoch_gas_price</a>: <a href="../myso_system/validator.md#myso_system_validator_gas_price">gas_price</a>,
        next_epoch_commission_rate: <a href="../myso_system/validator.md#myso_system_validator_commission_rate">commission_rate</a>,
        extra_fields: bag::new(ctx),
    }
}
</code></pre>



</details>
