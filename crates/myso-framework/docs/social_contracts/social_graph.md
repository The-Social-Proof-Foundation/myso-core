---
title: Module `social_contracts::social_graph`
---

Social graph module for the MySocial network
Manages social relationships between users (following/followers)


-  [Struct `SocialGraph`](#social_contracts_social_graph_SocialGraph)
-  [Struct `FollowEvent`](#social_contracts_social_graph_FollowEvent)
-  [Struct `UnfollowEvent`](#social_contracts_social_graph_UnfollowEvent)
-  [Constants](#@Constants_0)
-  [Function `bootstrap_init`](#social_contracts_social_graph_bootstrap_init)
-  [Function `follow`](#social_contracts_social_graph_follow)
-  [Function `follow_internal`](#social_contracts_social_graph_follow_internal)
-  [Function `principal_for_social_graph_action`](#social_contracts_social_graph_principal_for_social_graph_action)
-  [Function `follow_profile`](#social_contracts_social_graph_follow_profile)
-  [Function `unfollow_profile`](#social_contracts_social_graph_unfollow_profile)
-  [Function `unfollow`](#social_contracts_social_graph_unfollow)
-  [Function `unfollow_internal`](#social_contracts_social_graph_unfollow_internal)
-  [Function `block_wallet_as`](#social_contracts_social_graph_block_wallet_as)
-  [Function `block_platform_wallet`](#social_contracts_social_graph_block_platform_wallet)
-  [Function `block_wallet`](#social_contracts_social_graph_block_wallet)
-  [Function `unblock_wallet`](#social_contracts_social_graph_unblock_wallet)
-  [Function `block_profile`](#social_contracts_social_graph_block_profile)
-  [Function `unblock_profile`](#social_contracts_social_graph_unblock_profile)
-  [Function `migrate_social_graph`](#social_contracts_social_graph_migrate_social_graph)
-  [Function `borrow_version_mut`](#social_contracts_social_graph_borrow_version_mut)
-  [Function `version`](#social_contracts_social_graph_version)
-  [Function `is_following`](#social_contracts_social_graph_is_following)
-  [Function `following_count`](#social_contracts_social_graph_following_count)
-  [Function `follower_count`](#social_contracts_social_graph_follower_count)
-  [Function `get_following`](#social_contracts_social_graph_get_following)
-  [Function `get_followers`](#social_contracts_social_graph_get_followers)


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
<b>use</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph">social_contracts::derivative_graph</a>;
<b>use</b> <a href="../social_contracts/governance.md#social_contracts_governance">social_contracts::governance</a>;
<b>use</b> <a href="../social_contracts/media_asset.md#social_contracts_license_template">social_contracts::license_template</a>;
<b>use</b> <a href="../social_contracts/media_asset.md#social_contracts_media_asset">social_contracts::media_asset</a>;
<b>use</b> <a href="../social_contracts/memory.md#social_contracts_memory">social_contracts::memory</a>;
<b>use</b> <a href="../social_contracts/platform.md#social_contracts_platform">social_contracts::platform</a>;
<b>use</b> <a href="../social_contracts/profile.md#social_contracts_profile">social_contracts::profile</a>;
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



<a name="social_contracts_social_graph_SocialGraph"></a>

## Struct `SocialGraph`

Global social graph object that tracks relationships between wallet addresses
Uses wallet-level architecture - no profile required


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_SocialGraph">SocialGraph</a> <b>has</b> key
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
<code>following: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;<b>address</b>, <a href="../myso/vec_set.md#myso_vec_set_VecSet">myso::vec_set::VecSet</a>&lt;<b>address</b>&gt;&gt;</code>
</dt>
<dd>
 Table mapping wallet addresses to sets of addresses they are following
</dd>
<dt>
<code>followers: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;<b>address</b>, <a href="../myso/vec_set.md#myso_vec_set_VecSet">myso::vec_set::VecSet</a>&lt;<b>address</b>&gt;&gt;</code>
</dt>
<dd>
 Table mapping wallet addresses to sets of addresses following them
</dd>
<dt>
<code><a href="../social_contracts/social_graph.md#social_contracts_social_graph_version">version</a>: u64</code>
</dt>
<dd>
 Current version of the object
</dd>
</dl>


</details>

<a name="social_contracts_social_graph_FollowEvent"></a>

## Struct `FollowEvent`

Follow event - emitted when a wallet address follows another wallet address


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_FollowEvent">FollowEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>follower: <b>address</b></code>
</dt>
<dd>
 Wallet address of the follower
</dd>
<dt>
<code>following: <b>address</b></code>
</dt>
<dd>
 Wallet address being followed
</dd>
</dl>


</details>

<a name="social_contracts_social_graph_UnfollowEvent"></a>

## Struct `UnfollowEvent`

Unfollow event - emitted when a wallet address unfollows another wallet address


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_UnfollowEvent">UnfollowEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>follower: <b>address</b></code>
</dt>
<dd>
 Wallet address of the unfollower
</dd>
<dt>
<code>unfollowed: <b>address</b></code>
</dt>
<dd>
 Wallet address being unfollowed
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="social_contracts_social_graph_EAlreadyFollowing"></a>

Error codes


<pre><code><b>const</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_EAlreadyFollowing">EAlreadyFollowing</a>: u64 = 0;
</code></pre>



<a name="social_contracts_social_graph_ENotFollowing"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_ENotFollowing">ENotFollowing</a>: u64 = 1;
</code></pre>



<a name="social_contracts_social_graph_ECannotFollowSelf"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_ECannotFollowSelf">ECannotFollowSelf</a>: u64 = 2;
</code></pre>



<a name="social_contracts_social_graph_EUnauthorized"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_EUnauthorized">EUnauthorized</a>: u64 = 3;
</code></pre>



<a name="social_contracts_social_graph_EWrongVersion"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_EWrongVersion">EWrongVersion</a>: u64 = 4;
</code></pre>



<a name="social_contracts_social_graph_bootstrap_init"></a>

## Function `bootstrap_init`

Bootstrap initialization function - creates the social graph shared object


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_bootstrap_init">bootstrap_init</a>(_clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_bootstrap_init">bootstrap_init</a>(_clock: &Clock, ctx: &<b>mut</b> TxContext) {
    <b>let</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a> = <a href="../social_contracts/social_graph.md#social_contracts_social_graph_SocialGraph">SocialGraph</a> {
        id: object::new(ctx),
        following: table::new(ctx),
        followers: table::new(ctx),
        <a href="../social_contracts/social_graph.md#social_contracts_social_graph_version">version</a>: <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(),
    };
    // Share the social graph to make it globally accessible
    transfer::share_object(<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>);
}
</code></pre>



</details>

<a name="social_contracts_social_graph_follow"></a>

## Function `follow`

Follow a wallet address
Uses wallet-level architecture - no profile required


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_follow">follow</a>(<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>: &<b>mut</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_SocialGraph">social_contracts::social_graph::SocialGraph</a>, following_address: <b>address</b>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_follow">follow</a>(
    <a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>: &<b>mut</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_SocialGraph">SocialGraph</a>,
    following_address: <b>address</b>,
    ctx: &<b>mut</b> TxContext
) {
    // Check <a href="../social_contracts/social_graph.md#social_contracts_social_graph_version">version</a> compatibility
    <b>assert</b>!(<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>.<a href="../social_contracts/social_graph.md#social_contracts_social_graph_version">version</a> == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/social_graph.md#social_contracts_social_graph_EWrongVersion">EWrongVersion</a>);
    <b>let</b> sender = tx_context::sender(ctx);
    // Cannot <a href="../social_contracts/social_graph.md#social_contracts_social_graph_follow">follow</a> self
    <b>assert</b>!(sender != following_address, <a href="../social_contracts/social_graph.md#social_contracts_social_graph_ECannotFollowSelf">ECannotFollowSelf</a>);
    // Initialize follower's following set <b>if</b> it doesn't exist (lazy initialization)
    <b>if</b> (!table::contains(&<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>.following, sender)) {
        table::add(&<b>mut</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>.following, sender, vec_set::empty());
    };
    // Initialize followed's followers set <b>if</b> it doesn't exist (lazy initialization)
    <b>if</b> (!table::contains(&<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>.followers, following_address)) {
        table::add(&<b>mut</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>.followers, following_address, vec_set::empty());
    };
    // Get mutable references to the sets
    <b>let</b> follower_following = table::borrow_mut(&<b>mut</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>.following, sender);
    <b>let</b> following_followers = table::borrow_mut(&<b>mut</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>.followers, following_address);
    // Check <b>if</b> already following
    <b>assert</b>!(!vec_set::contains(follower_following, &following_address), <a href="../social_contracts/social_graph.md#social_contracts_social_graph_EAlreadyFollowing">EAlreadyFollowing</a>);
    // Add to sets
    vec_set::insert(follower_following, following_address);
    vec_set::insert(following_followers, sender);
    // Emit <a href="../social_contracts/social_graph.md#social_contracts_social_graph_follow">follow</a> event
    event::emit(<a href="../social_contracts/social_graph.md#social_contracts_social_graph_FollowEvent">FollowEvent</a> {
        follower: sender,
        following: following_address,
    });
}
</code></pre>



</details>

<a name="social_contracts_social_graph_follow_internal"></a>

## Function `follow_internal`

Package-scoped follow used by the agent-aware action coordinator after
resolving the transaction signer to its human principal.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_follow_internal">follow_internal</a>(<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>: &<b>mut</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_SocialGraph">social_contracts::social_graph::SocialGraph</a>, follower_address: <b>address</b>, following_address: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_follow_internal">follow_internal</a>(
    <a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>: &<b>mut</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_SocialGraph">SocialGraph</a>,
    follower_address: <b>address</b>,
    following_address: <b>address</b>,
) {
    <b>assert</b>!(<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>.<a href="../social_contracts/social_graph.md#social_contracts_social_graph_version">version</a> == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/social_graph.md#social_contracts_social_graph_EWrongVersion">EWrongVersion</a>);
    <b>assert</b>!(follower_address != following_address, <a href="../social_contracts/social_graph.md#social_contracts_social_graph_ECannotFollowSelf">ECannotFollowSelf</a>);
    <b>if</b> (!table::contains(&<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>.following, follower_address)) {
        table::add(&<b>mut</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>.following, follower_address, vec_set::empty());
    };
    <b>if</b> (!table::contains(&<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>.followers, following_address)) {
        table::add(&<b>mut</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>.followers, following_address, vec_set::empty());
    };
    <b>let</b> follower_following = table::borrow_mut(&<b>mut</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>.following, follower_address);
    <b>let</b> following_followers = table::borrow_mut(&<b>mut</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>.followers, following_address);
    <b>assert</b>!(!vec_set::contains(follower_following, &following_address), <a href="../social_contracts/social_graph.md#social_contracts_social_graph_EAlreadyFollowing">EAlreadyFollowing</a>);
    vec_set::insert(follower_following, following_address);
    vec_set::insert(following_followers, follower_address);
    event::emit(<a href="../social_contracts/social_graph.md#social_contracts_social_graph_FollowEvent">FollowEvent</a> { follower: follower_address, following: following_address });
}
</code></pre>



</details>

<a name="social_contracts_social_graph_principal_for_social_graph_action"></a>

## Function `principal_for_social_graph_action`

Resolve either the human owner or a registered delegated agent to the
wallet principal whose social graph is being changed.


<pre><code><b>fun</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_principal_for_social_graph_action">principal_for_social_graph_action</a>(memory_config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, memory_account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, <a href="../social_contracts/block_list.md#social_contracts_block_list">block_list</a>: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_principal_for_social_graph_action">principal_for_social_graph_action</a>(
    memory_config: &MemoryConfig,
    memory_account: &MemoryAccount,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &Platform,
    <a href="../social_contracts/block_list.md#social_contracts_block_list">block_list</a>: &BlockListRegistry,
    clock: &Clock,
    ctx: &TxContext,
): <b>address</b> {
    <b>let</b> platform_id = object::uid_to_address(<a href="../social_contracts/platform.md#social_contracts_platform_id">platform::id</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>));
    <b>let</b> acting = <a href="../social_contracts/memory.md#social_contracts_memory_resolve_actor_with_cap">memory::resolve_actor_with_cap</a>(
        memory_config,
        memory_account,
        <a href="../social_contracts/memory.md#social_contracts_memory_cap_social_graph">memory::cap_social_graph</a>(),
        option::some(platform_id),
        0,
        clock,
        ctx,
    );
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_direct_execution_allowed">memory::assert_direct_execution_allowed</a>(
        memory_account,
        <a href="../social_contracts/memory.md#social_contracts_memory_cap_social_graph">memory::cap_social_graph</a>(),
        ctx,
    );
    <b>let</b> principal = <a href="../social_contracts/memory.md#social_contracts_memory_acting_principal_owner">memory::acting_principal_owner</a>(&acting);
    <b>assert</b>!(<a href="../social_contracts/memory.md#social_contracts_memory_owner">memory::owner</a>(memory_account) == principal, <a href="../social_contracts/social_graph.md#social_contracts_social_graph_EUnauthorized">EUnauthorized</a>);
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform_has_joined_platform">platform::has_joined_platform</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, principal), <a href="../social_contracts/social_graph.md#social_contracts_social_graph_EUnauthorized">EUnauthorized</a>);
    <b>assert</b>!(!<a href="../social_contracts/block_list.md#social_contracts_block_list_is_blocked">block_list::is_blocked</a>(<a href="../social_contracts/block_list.md#social_contracts_block_list">block_list</a>, platform_id, principal), <a href="../social_contracts/social_graph.md#social_contracts_social_graph_EUnauthorized">EUnauthorized</a>);
    principal
}
</code></pre>



</details>

<a name="social_contracts_social_graph_follow_profile"></a>

## Function `follow_profile`

Follow through the same graph mutation used by wallet calls, after
resolving a delegated agent to its human principal.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_follow_profile">follow_profile</a>(memory_config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, memory_account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, <a href="../social_contracts/block_list.md#social_contracts_block_list">block_list</a>: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, graph: &<b>mut</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_SocialGraph">social_contracts::social_graph::SocialGraph</a>, target_owner: <b>address</b>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_follow_profile">follow_profile</a>(
    memory_config: &MemoryConfig,
    memory_account: &MemoryAccount,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &Platform,
    <a href="../social_contracts/block_list.md#social_contracts_block_list">block_list</a>: &BlockListRegistry,
    graph: &<b>mut</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_SocialGraph">SocialGraph</a>,
    target_owner: <b>address</b>,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> principal = <a href="../social_contracts/social_graph.md#social_contracts_social_graph_principal_for_social_graph_action">principal_for_social_graph_action</a>(
        memory_config, memory_account, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, <a href="../social_contracts/block_list.md#social_contracts_block_list">block_list</a>, clock, ctx,
    );
    <a href="../social_contracts/block_list.md#social_contracts_block_list_assert_not_blocked">block_list::assert_not_blocked</a>(<a href="../social_contracts/block_list.md#social_contracts_block_list">block_list</a>, principal, target_owner);
    <a href="../social_contracts/social_graph.md#social_contracts_social_graph_follow_internal">follow_internal</a>(graph, principal, target_owner);
}
</code></pre>



</details>

<a name="social_contracts_social_graph_unfollow_profile"></a>

## Function `unfollow_profile`

Unfollow through the same graph mutation used by wallet calls, after
resolving a delegated agent to its human principal.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_unfollow_profile">unfollow_profile</a>(memory_config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, memory_account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, <a href="../social_contracts/block_list.md#social_contracts_block_list">block_list</a>: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, graph: &<b>mut</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_SocialGraph">social_contracts::social_graph::SocialGraph</a>, target_owner: <b>address</b>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_unfollow_profile">unfollow_profile</a>(
    memory_config: &MemoryConfig,
    memory_account: &MemoryAccount,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &Platform,
    <a href="../social_contracts/block_list.md#social_contracts_block_list">block_list</a>: &BlockListRegistry,
    graph: &<b>mut</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_SocialGraph">SocialGraph</a>,
    target_owner: <b>address</b>,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> principal = <a href="../social_contracts/social_graph.md#social_contracts_social_graph_principal_for_social_graph_action">principal_for_social_graph_action</a>(
        memory_config, memory_account, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, <a href="../social_contracts/block_list.md#social_contracts_block_list">block_list</a>, clock, ctx,
    );
    <b>assert</b>!(<a href="../social_contracts/social_graph.md#social_contracts_social_graph_unfollow_internal">unfollow_internal</a>(graph, principal, target_owner), <a href="../social_contracts/social_graph.md#social_contracts_social_graph_ENotFollowing">ENotFollowing</a>);
}
</code></pre>



</details>

<a name="social_contracts_social_graph_unfollow"></a>

## Function `unfollow`

Unfollow a wallet address
Uses wallet-level architecture - no profile required


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_unfollow">unfollow</a>(<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>: &<b>mut</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_SocialGraph">social_contracts::social_graph::SocialGraph</a>, following_address: <b>address</b>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_unfollow">unfollow</a>(
    <a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>: &<b>mut</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_SocialGraph">SocialGraph</a>,
    following_address: <b>address</b>,
    ctx: &<b>mut</b> TxContext
) {
    // Check <a href="../social_contracts/social_graph.md#social_contracts_social_graph_version">version</a> compatibility
    <b>assert</b>!(<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>.<a href="../social_contracts/social_graph.md#social_contracts_social_graph_version">version</a> == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/social_graph.md#social_contracts_social_graph_EWrongVersion">EWrongVersion</a>);
    <b>let</b> sender = tx_context::sender(ctx);
    // Check <b>if</b> following sets exist
    <b>if</b> (!table::contains(&<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>.following, sender)) {
        <b>abort</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_ENotFollowing">ENotFollowing</a>
    };
    <b>if</b> (!table::contains(&<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>.followers, following_address)) {
        <b>abort</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_ENotFollowing">ENotFollowing</a>
    };
    // Get mutable references to the sets
    <b>let</b> follower_following = table::borrow_mut(&<b>mut</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>.following, sender);
    <b>let</b> following_followers = table::borrow_mut(&<b>mut</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>.followers, following_address);
    // Check <b>if</b> following
    <b>if</b> (!vec_set::contains(follower_following, &following_address)) {
        <b>abort</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_ENotFollowing">ENotFollowing</a>
    };
    // Remove from sets
    vec_set::remove(follower_following, &following_address);
    vec_set::remove(following_followers, &sender);
    // Emit <a href="../social_contracts/social_graph.md#social_contracts_social_graph_unfollow">unfollow</a> event
    event::emit(<a href="../social_contracts/social_graph.md#social_contracts_social_graph_UnfollowEvent">UnfollowEvent</a> {
        follower: sender,
        unfollowed: following_address,
    });
}
</code></pre>



</details>

<a name="social_contracts_social_graph_unfollow_internal"></a>

## Function `unfollow_internal`

Internal unfollow function that accepts explicit wallet addresses
Used for bidirectional unfollow during blocking operations
Returns true if unfollow occurred, false if not following


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_unfollow_internal">unfollow_internal</a>(<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>: &<b>mut</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_SocialGraph">social_contracts::social_graph::SocialGraph</a>, follower_address: <b>address</b>, following_address: <b>address</b>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_unfollow_internal">unfollow_internal</a>(
    <a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>: &<b>mut</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_SocialGraph">SocialGraph</a>,
    follower_address: <b>address</b>,
    following_address: <b>address</b>
): bool {
    // Check <b>if</b> following relationship exists
    <b>if</b> (!<a href="../social_contracts/social_graph.md#social_contracts_social_graph_is_following">is_following</a>(<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>, follower_address, following_address)) {
        <b>return</b> <b>false</b>  // Not following, nothing to do
    };
    // Check <b>if</b> following sets exist (defensive)
    <b>if</b> (!table::contains(&<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>.following, follower_address)) {
        <b>return</b> <b>false</b>
    };
    <b>if</b> (!table::contains(&<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>.followers, following_address)) {
        <b>return</b> <b>false</b>
    };
    // Get mutable references to the sets
    <b>let</b> follower_following = table::borrow_mut(&<b>mut</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>.following, follower_address);
    <b>let</b> following_followers = table::borrow_mut(&<b>mut</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>.followers, following_address);
    // Remove <b>if</b> present (defensive check)
    <b>if</b> (vec_set::contains(follower_following, &following_address)) {
        vec_set::remove(follower_following, &following_address);
        vec_set::remove(following_followers, &follower_address);
        // Emit <a href="../social_contracts/social_graph.md#social_contracts_social_graph_unfollow">unfollow</a> event
        event::emit(<a href="../social_contracts/social_graph.md#social_contracts_social_graph_UnfollowEvent">UnfollowEvent</a> {
            follower: follower_address,
            unfollowed: following_address,
        });
        <b>return</b> <b>true</b>
    };
    <b>false</b>
}
</code></pre>



</details>

<a name="social_contracts_social_graph_block_wallet_as"></a>

## Function `block_wallet_as`

Shared block implementation. Block-list storage stays in <code><a href="../social_contracts/block_list.md#social_contracts_block_list">block_list</a></code>;
graph cleanup stays here so the module dependency remains acyclic.


<pre><code><b>fun</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_block_wallet_as">block_wallet_as</a>(<a href="../social_contracts/block_list.md#social_contracts_block_list">block_list</a>: &<b>mut</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, graph: &<b>mut</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_SocialGraph">social_contracts::social_graph::SocialGraph</a>, blocker: <b>address</b>, target_owner: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_block_wallet_as">block_wallet_as</a>(
    <a href="../social_contracts/block_list.md#social_contracts_block_list">block_list</a>: &<b>mut</b> BlockListRegistry,
    graph: &<b>mut</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_SocialGraph">SocialGraph</a>,
    blocker: <b>address</b>,
    target_owner: <b>address</b>,
) {
    <a href="../social_contracts/block_list.md#social_contracts_block_list_block_wallet_internal">block_list::block_wallet_internal</a>(<a href="../social_contracts/block_list.md#social_contracts_block_list">block_list</a>, blocker, target_owner);
    <a href="../social_contracts/social_graph.md#social_contracts_social_graph_unfollow_internal">unfollow_internal</a>(graph, blocker, target_owner);
    <a href="../social_contracts/social_graph.md#social_contracts_social_graph_unfollow_internal">unfollow_internal</a>(graph, target_owner, blocker);
}
</code></pre>



</details>

<a name="social_contracts_social_graph_block_platform_wallet"></a>

## Function `block_platform_wallet`

Block a wallet as a platform after reusing the platform module's
developer/moderator authorization check.


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_block_platform_wallet">block_platform_wallet</a>(<a href="../social_contracts/block_list.md#social_contracts_block_list">block_list</a>: &<b>mut</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, graph: &<b>mut</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_SocialGraph">social_contracts::social_graph::SocialGraph</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, group: &<a href="../myso/permissioned_group.md#myso_permissioned_group_PermissionedGroup">myso::permissioned_group::PermissionedGroup</a>&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformPackage">social_contracts::platform::PlatformPackage</a>&gt;, target_owner: <b>address</b>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_block_platform_wallet">block_platform_wallet</a>(
    <a href="../social_contracts/block_list.md#social_contracts_block_list">block_list</a>: &<b>mut</b> BlockListRegistry,
    graph: &<b>mut</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_SocialGraph">SocialGraph</a>,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> Platform,
    group: &<a href="../myso/permissioned_group.md#myso_permissioned_group_PermissionedGroup">myso::permissioned_group::PermissionedGroup</a>&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformPackage">platform::PlatformPackage</a>&gt;,
    target_owner: <b>address</b>,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> platform_address = <a href="../social_contracts/platform.md#social_contracts_platform_assert_block_wallet_permission">platform::assert_block_wallet_permission</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, group, ctx);
    <a href="../social_contracts/social_graph.md#social_contracts_social_graph_block_wallet_as">block_wallet_as</a>(<a href="../social_contracts/block_list.md#social_contracts_block_list">block_list</a>, graph, platform_address, target_owner);
}
</code></pre>



</details>

<a name="social_contracts_social_graph_block_wallet"></a>

## Function `block_wallet`

Block a profile directly as the transaction sender.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_block_wallet">block_wallet</a>(<a href="../social_contracts/block_list.md#social_contracts_block_list">block_list</a>: &<b>mut</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, graph: &<b>mut</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_SocialGraph">social_contracts::social_graph::SocialGraph</a>, target_owner: <b>address</b>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_block_wallet">block_wallet</a>(
    <a href="../social_contracts/block_list.md#social_contracts_block_list">block_list</a>: &<b>mut</b> BlockListRegistry,
    graph: &<b>mut</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_SocialGraph">SocialGraph</a>,
    target_owner: <b>address</b>,
    ctx: &<b>mut</b> TxContext,
) {
    <a href="../social_contracts/social_graph.md#social_contracts_social_graph_block_wallet_as">block_wallet_as</a>(<a href="../social_contracts/block_list.md#social_contracts_block_list">block_list</a>, graph, tx_context::sender(ctx), target_owner);
}
</code></pre>



</details>

<a name="social_contracts_social_graph_unblock_wallet"></a>

## Function `unblock_wallet`

Unblock a profile directly as the transaction sender.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_unblock_wallet">unblock_wallet</a>(<a href="../social_contracts/block_list.md#social_contracts_block_list">block_list</a>: &<b>mut</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, target_owner: <b>address</b>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_unblock_wallet">unblock_wallet</a>(
    <a href="../social_contracts/block_list.md#social_contracts_block_list">block_list</a>: &<b>mut</b> BlockListRegistry,
    target_owner: <b>address</b>,
    ctx: &<b>mut</b> TxContext,
) {
    <a href="../social_contracts/block_list.md#social_contracts_block_list_unblock_wallet_internal">block_list::unblock_wallet_internal</a>(
        <a href="../social_contracts/block_list.md#social_contracts_block_list">block_list</a>,
        tx_context::sender(ctx),
        target_owner,
    );
}
</code></pre>



</details>

<a name="social_contracts_social_graph_block_profile"></a>

## Function `block_profile`

Block through the shared block/graph implementation after resolving a
delegated agent to its human principal.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_block_profile">block_profile</a>(memory_config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, memory_account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, <a href="../social_contracts/block_list.md#social_contracts_block_list">block_list</a>: &<b>mut</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, graph: &<b>mut</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_SocialGraph">social_contracts::social_graph::SocialGraph</a>, target_owner: <b>address</b>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_block_profile">block_profile</a>(
    memory_config: &MemoryConfig,
    memory_account: &MemoryAccount,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &Platform,
    <a href="../social_contracts/block_list.md#social_contracts_block_list">block_list</a>: &<b>mut</b> BlockListRegistry,
    graph: &<b>mut</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_SocialGraph">SocialGraph</a>,
    target_owner: <b>address</b>,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> principal = <a href="../social_contracts/social_graph.md#social_contracts_social_graph_principal_for_social_graph_action">principal_for_social_graph_action</a>(
        memory_config, memory_account, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, <a href="../social_contracts/block_list.md#social_contracts_block_list">block_list</a>, clock, ctx,
    );
    <a href="../social_contracts/social_graph.md#social_contracts_social_graph_block_wallet_as">block_wallet_as</a>(<a href="../social_contracts/block_list.md#social_contracts_block_list">block_list</a>, graph, principal, target_owner);
}
</code></pre>



</details>

<a name="social_contracts_social_graph_unblock_profile"></a>

## Function `unblock_profile`

Unblock through the shared block-list implementation after resolving a
delegated agent to its human principal.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_unblock_profile">unblock_profile</a>(memory_config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, memory_account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, <a href="../social_contracts/block_list.md#social_contracts_block_list">block_list</a>: &<b>mut</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, target_owner: <b>address</b>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_unblock_profile">unblock_profile</a>(
    memory_config: &MemoryConfig,
    memory_account: &MemoryAccount,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &Platform,
    <a href="../social_contracts/block_list.md#social_contracts_block_list">block_list</a>: &<b>mut</b> BlockListRegistry,
    target_owner: <b>address</b>,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> principal = <a href="../social_contracts/social_graph.md#social_contracts_social_graph_principal_for_social_graph_action">principal_for_social_graph_action</a>(
        memory_config, memory_account, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, <a href="../social_contracts/block_list.md#social_contracts_block_list">block_list</a>, clock, ctx,
    );
    <a href="../social_contracts/block_list.md#social_contracts_block_list_unblock_wallet_internal">block_list::unblock_wallet_internal</a>(<a href="../social_contracts/block_list.md#social_contracts_block_list">block_list</a>, principal, target_owner);
}
</code></pre>



</details>

<a name="social_contracts_social_graph_migrate_social_graph"></a>

## Function `migrate_social_graph`

Migrate the social graph to a new version
Only callable by the admin with the AdminCap


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_migrate_social_graph">migrate_social_graph</a>(<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>: &<b>mut</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_SocialGraph">social_contracts::social_graph::SocialGraph</a>, _: &<a href="../social_contracts/upgrade.md#social_contracts_upgrade_UpgradeAdminCap">social_contracts::upgrade::UpgradeAdminCap</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_migrate_social_graph">migrate_social_graph</a>(
    <a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>: &<b>mut</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_SocialGraph">SocialGraph</a>,
    _: &<a href="../social_contracts/upgrade.md#social_contracts_upgrade_UpgradeAdminCap">upgrade::UpgradeAdminCap</a>,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> current_version = <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>();
    // Verify this is an <a href="../social_contracts/upgrade.md#social_contracts_upgrade">upgrade</a> (new <a href="../social_contracts/social_graph.md#social_contracts_social_graph_version">version</a> &gt; current <a href="../social_contracts/social_graph.md#social_contracts_social_graph_version">version</a>)
    <b>assert</b>!(<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>.<a href="../social_contracts/social_graph.md#social_contracts_social_graph_version">version</a> &lt; current_version, <a href="../social_contracts/social_graph.md#social_contracts_social_graph_EWrongVersion">EWrongVersion</a>);
    // Remember old <a href="../social_contracts/social_graph.md#social_contracts_social_graph_version">version</a> and update to new <a href="../social_contracts/social_graph.md#social_contracts_social_graph_version">version</a>
    <b>let</b> old_version = <a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>.<a href="../social_contracts/social_graph.md#social_contracts_social_graph_version">version</a>;
    <a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>.<a href="../social_contracts/social_graph.md#social_contracts_social_graph_version">version</a> = current_version;
    // Emit event <b>for</b> object migration
    <b>let</b> graph_id = object::id(<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>);
    <a href="../social_contracts/upgrade.md#social_contracts_upgrade_emit_migration_event">upgrade::emit_migration_event</a>(
        graph_id,
        string::utf8(b"<a href="../social_contracts/social_graph.md#social_contracts_social_graph_SocialGraph">SocialGraph</a>"),
        old_version,
        tx_context::sender(ctx)
    );
    // Any migration logic can be added here <b>for</b> future upgrades
}
</code></pre>



</details>

<a name="social_contracts_social_graph_borrow_version_mut"></a>

## Function `borrow_version_mut`

Get a mutable reference to the version field (for upgrade module)


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_borrow_version_mut">borrow_version_mut</a>(<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>: &<b>mut</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_SocialGraph">social_contracts::social_graph::SocialGraph</a>): &<b>mut</b> u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_borrow_version_mut">borrow_version_mut</a>(<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>: &<b>mut</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_SocialGraph">SocialGraph</a>): &<b>mut</b> u64 {
    &<b>mut</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>.<a href="../social_contracts/social_graph.md#social_contracts_social_graph_version">version</a>
}
</code></pre>



</details>

<a name="social_contracts_social_graph_version"></a>

## Function `version`

Get the version of the social graph


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_version">version</a>(<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>: &<a href="../social_contracts/social_graph.md#social_contracts_social_graph_SocialGraph">social_contracts::social_graph::SocialGraph</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_version">version</a>(<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>: &<a href="../social_contracts/social_graph.md#social_contracts_social_graph_SocialGraph">SocialGraph</a>): u64 {
    <a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>.<a href="../social_contracts/social_graph.md#social_contracts_social_graph_version">version</a>
}
</code></pre>



</details>

<a name="social_contracts_social_graph_is_following"></a>

## Function `is_following`

Check if a wallet address is following another wallet address


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_is_following">is_following</a>(<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>: &<a href="../social_contracts/social_graph.md#social_contracts_social_graph_SocialGraph">social_contracts::social_graph::SocialGraph</a>, follower_address: <b>address</b>, following_address: <b>address</b>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_is_following">is_following</a>(<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>: &<a href="../social_contracts/social_graph.md#social_contracts_social_graph_SocialGraph">SocialGraph</a>, follower_address: <b>address</b>, following_address: <b>address</b>): bool {
    <b>if</b> (!table::contains(&<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>.following, follower_address)) {
        <b>return</b> <b>false</b>
    };
    <b>let</b> follower_following = table::borrow(&<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>.following, follower_address);
    vec_set::contains(follower_following, &following_address)
}
</code></pre>



</details>

<a name="social_contracts_social_graph_following_count"></a>

## Function `following_count`

Get the number of wallet addresses a user is following


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_following_count">following_count</a>(<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>: &<a href="../social_contracts/social_graph.md#social_contracts_social_graph_SocialGraph">social_contracts::social_graph::SocialGraph</a>, wallet_address: <b>address</b>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_following_count">following_count</a>(<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>: &<a href="../social_contracts/social_graph.md#social_contracts_social_graph_SocialGraph">SocialGraph</a>, wallet_address: <b>address</b>): u64 {
    <b>if</b> (!table::contains(&<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>.following, wallet_address)) {
        <b>return</b> 0
    };
    <b>let</b> following = table::borrow(&<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>.following, wallet_address);
    vec_set::length(following)
}
</code></pre>



</details>

<a name="social_contracts_social_graph_follower_count"></a>

## Function `follower_count`

Get the number of followers a wallet address has


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_follower_count">follower_count</a>(<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>: &<a href="../social_contracts/social_graph.md#social_contracts_social_graph_SocialGraph">social_contracts::social_graph::SocialGraph</a>, wallet_address: <b>address</b>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_follower_count">follower_count</a>(<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>: &<a href="../social_contracts/social_graph.md#social_contracts_social_graph_SocialGraph">SocialGraph</a>, wallet_address: <b>address</b>): u64 {
    <b>if</b> (!table::contains(&<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>.followers, wallet_address)) {
        <b>return</b> 0
    };
    <b>let</b> followers = table::borrow(&<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>.followers, wallet_address);
    vec_set::length(followers)
}
</code></pre>



</details>

<a name="social_contracts_social_graph_get_following"></a>

## Function `get_following`

Get the list of wallet addresses a user is following


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_get_following">get_following</a>(<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>: &<a href="../social_contracts/social_graph.md#social_contracts_social_graph_SocialGraph">social_contracts::social_graph::SocialGraph</a>, wallet_address: <b>address</b>): vector&lt;<b>address</b>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_get_following">get_following</a>(<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>: &<a href="../social_contracts/social_graph.md#social_contracts_social_graph_SocialGraph">SocialGraph</a>, wallet_address: <b>address</b>): vector&lt;<b>address</b>&gt; {
    <b>if</b> (!table::contains(&<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>.following, wallet_address)) {
        <b>return</b> vector::empty()
    };
    <b>let</b> following = table::borrow(&<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>.following, wallet_address);
    vec_set::into_keys(*following)
}
</code></pre>



</details>

<a name="social_contracts_social_graph_get_followers"></a>

## Function `get_followers`

Get the list of followers for a wallet address


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_get_followers">get_followers</a>(<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>: &<a href="../social_contracts/social_graph.md#social_contracts_social_graph_SocialGraph">social_contracts::social_graph::SocialGraph</a>, wallet_address: <b>address</b>): vector&lt;<b>address</b>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_get_followers">get_followers</a>(<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>: &<a href="../social_contracts/social_graph.md#social_contracts_social_graph_SocialGraph">SocialGraph</a>, wallet_address: <b>address</b>): vector&lt;<b>address</b>&gt; {
    <b>if</b> (!table::contains(&<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>.followers, wallet_address)) {
        <b>return</b> vector::empty()
    };
    <b>let</b> followers = table::borrow(&<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>.followers, wallet_address);
    vec_set::into_keys(*followers)
}
</code></pre>



</details>
