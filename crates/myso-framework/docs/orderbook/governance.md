---
title: Module `orderbook::governance`
---

Governance module handles the governance of the <code>Pool</code> that it's attached
to.
Users with non zero stake can create proposals and vote on them. Winning
proposals are used to set the trade parameters for the next epoch.


-  [Struct `Proposal`](#orderbook_governance_Proposal)
-  [Struct `Governance`](#orderbook_governance_Governance)
-  [Struct `TradeParamsUpdateEvent`](#orderbook_governance_TradeParamsUpdateEvent)
-  [Constants](#@Constants_0)
-  [Function `empty`](#orderbook_governance_empty)
-  [Function `whitelisted`](#orderbook_governance_whitelisted)
-  [Function `stable`](#orderbook_governance_stable)
-  [Function `quorum`](#orderbook_governance_quorum)
-  [Function `update`](#orderbook_governance_update)
-  [Function `add_proposal`](#orderbook_governance_add_proposal)
-  [Function `adjust_vote`](#orderbook_governance_adjust_vote)
-  [Function `adjust_voting_power`](#orderbook_governance_adjust_voting_power)
-  [Function `trade_params`](#orderbook_governance_trade_params)
-  [Function `next_trade_params`](#orderbook_governance_next_trade_params)
-  [Function `stake_to_voting_power`](#orderbook_governance_stake_to_voting_power)
-  [Function `new_proposal`](#orderbook_governance_new_proposal)
-  [Function `remove_lowest_proposal`](#orderbook_governance_remove_lowest_proposal)
-  [Function `to_trade_params`](#orderbook_governance_to_trade_params)


<pre><code><b>use</b> <a href="../myso/accumulator.md#myso_accumulator">myso::accumulator</a>;
<b>use</b> <a href="../myso/accumulator_settlement.md#myso_accumulator_settlement">myso::accumulator_settlement</a>;
<b>use</b> <a href="../myso/address.md#myso_address">myso::address</a>;
<b>use</b> <a href="../myso/bcs.md#myso_bcs">myso::bcs</a>;
<b>use</b> <a href="../myso/dynamic_field.md#myso_dynamic_field">myso::dynamic_field</a>;
<b>use</b> <a href="../myso/event.md#myso_event">myso::event</a>;
<b>use</b> <a href="../myso/hash.md#myso_hash">myso::hash</a>;
<b>use</b> <a href="../myso/hex.md#myso_hex">myso::hex</a>;
<b>use</b> <a href="../myso/object.md#myso_object">myso::object</a>;
<b>use</b> <a href="../myso/party.md#myso_party">myso::party</a>;
<b>use</b> <a href="../myso/transfer.md#myso_transfer">myso::transfer</a>;
<b>use</b> <a href="../myso/tx_context.md#myso_tx_context">myso::tx_context</a>;
<b>use</b> <a href="../myso/vec_map.md#myso_vec_map">myso::vec_map</a>;
<b>use</b> <a href="../orderbook/constants.md#orderbook_constants">orderbook::constants</a>;
<b>use</b> <a href="../orderbook/math.md#orderbook_math">orderbook::math</a>;
<b>use</b> <a href="../orderbook/trade_params.md#orderbook_trade_params">orderbook::trade_params</a>;
<b>use</b> <a href="../std/address.md#std_address">std::address</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/type_name.md#std_type_name">std::type_name</a>;
<b>use</b> <a href="../std/u128.md#std_u128">std::u128</a>;
<b>use</b> <a href="../std/u64.md#std_u64">std::u64</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
</code></pre>



<a name="orderbook_governance_Proposal"></a>

## Struct `Proposal`

<code><a href="../orderbook/governance.md#orderbook_governance_Proposal">Proposal</a></code> struct that holds the parameters of a proposal and its current
total votes.


<pre><code><b>public</b> <b>struct</b> <a href="../orderbook/governance.md#orderbook_governance_Proposal">Proposal</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>taker_fee: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>maker_fee: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>stake_required: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>votes: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="orderbook_governance_Governance"></a>

## Struct `Governance`

Details of a pool. This is refreshed every epoch by the first
<code>State</code> action against this pool.


<pre><code><b>public</b> <b>struct</b> <a href="../orderbook/governance.md#orderbook_governance_Governance">Governance</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>epoch: u64</code>
</dt>
<dd>
 Tracks refreshes.
</dd>
<dt>
<code><a href="../orderbook/governance.md#orderbook_governance_whitelisted">whitelisted</a>: bool</code>
</dt>
<dd>
 If Pool is whitelisted.
</dd>
<dt>
<code><a href="../orderbook/governance.md#orderbook_governance_stable">stable</a>: bool</code>
</dt>
<dd>
 If Pool is stable or volatile.
</dd>
<dt>
<code>proposals: <a href="../myso/vec_map.md#myso_vec_map_VecMap">myso::vec_map::VecMap</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, <a href="../orderbook/governance.md#orderbook_governance_Proposal">orderbook::governance::Proposal</a>&gt;</code>
</dt>
<dd>
 List of proposals for the current epoch.
</dd>
<dt>
<code><a href="../orderbook/trade_params.md#orderbook_trade_params">trade_params</a>: <a href="../orderbook/trade_params.md#orderbook_trade_params_TradeParams">orderbook::trade_params::TradeParams</a></code>
</dt>
<dd>
 Trade parameters for the current epoch.
</dd>
<dt>
<code><a href="../orderbook/governance.md#orderbook_governance_next_trade_params">next_trade_params</a>: <a href="../orderbook/trade_params.md#orderbook_trade_params_TradeParams">orderbook::trade_params::TradeParams</a></code>
</dt>
<dd>
 Trade parameters for the next epoch.
</dd>
<dt>
<code>voting_power: u64</code>
</dt>
<dd>
 All voting power from the current stakes.
</dd>
<dt>
<code><a href="../orderbook/governance.md#orderbook_governance_quorum">quorum</a>: u64</code>
</dt>
<dd>
 Quorum for the current epoch.
</dd>
</dl>


</details>

<a name="orderbook_governance_TradeParamsUpdateEvent"></a>

## Struct `TradeParamsUpdateEvent`

Event emitted when trade parameters are updated.


<pre><code><b>public</b> <b>struct</b> <a href="../orderbook/governance.md#orderbook_governance_TradeParamsUpdateEvent">TradeParamsUpdateEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>taker_fee: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>maker_fee: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>stake_required: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="orderbook_governance_EInvalidMakerFee"></a>



<pre><code><b>const</b> <a href="../orderbook/governance.md#orderbook_governance_EInvalidMakerFee">EInvalidMakerFee</a>: u64 = 1;
</code></pre>



<a name="orderbook_governance_EInvalidTakerFee"></a>



<pre><code><b>const</b> <a href="../orderbook/governance.md#orderbook_governance_EInvalidTakerFee">EInvalidTakerFee</a>: u64 = 2;
</code></pre>



<a name="orderbook_governance_EProposalDoesNotExist"></a>



<pre><code><b>const</b> <a href="../orderbook/governance.md#orderbook_governance_EProposalDoesNotExist">EProposalDoesNotExist</a>: u64 = 3;
</code></pre>



<a name="orderbook_governance_EMaxProposalsReachedNotEnoughVotes"></a>



<pre><code><b>const</b> <a href="../orderbook/governance.md#orderbook_governance_EMaxProposalsReachedNotEnoughVotes">EMaxProposalsReachedNotEnoughVotes</a>: u64 = 4;
</code></pre>



<a name="orderbook_governance_EWhitelistedPoolCannotChange"></a>



<pre><code><b>const</b> <a href="../orderbook/governance.md#orderbook_governance_EWhitelistedPoolCannotChange">EWhitelistedPoolCannotChange</a>: u64 = 5;
</code></pre>



<a name="orderbook_governance_FEE_MULTIPLE"></a>



<pre><code><b>const</b> <a href="../orderbook/governance.md#orderbook_governance_FEE_MULTIPLE">FEE_MULTIPLE</a>: u64 = 1000;
</code></pre>



<a name="orderbook_governance_MIN_TAKER_STABLE"></a>



<pre><code><b>const</b> <a href="../orderbook/governance.md#orderbook_governance_MIN_TAKER_STABLE">MIN_TAKER_STABLE</a>: u64 = 10000;
</code></pre>



<a name="orderbook_governance_MAX_TAKER_STABLE"></a>



<pre><code><b>const</b> <a href="../orderbook/governance.md#orderbook_governance_MAX_TAKER_STABLE">MAX_TAKER_STABLE</a>: u64 = 100000;
</code></pre>



<a name="orderbook_governance_MIN_MAKER_STABLE"></a>



<pre><code><b>const</b> <a href="../orderbook/governance.md#orderbook_governance_MIN_MAKER_STABLE">MIN_MAKER_STABLE</a>: u64 = 0;
</code></pre>



<a name="orderbook_governance_MAX_MAKER_STABLE"></a>



<pre><code><b>const</b> <a href="../orderbook/governance.md#orderbook_governance_MAX_MAKER_STABLE">MAX_MAKER_STABLE</a>: u64 = 50000;
</code></pre>



<a name="orderbook_governance_MIN_TAKER_VOLATILE"></a>



<pre><code><b>const</b> <a href="../orderbook/governance.md#orderbook_governance_MIN_TAKER_VOLATILE">MIN_TAKER_VOLATILE</a>: u64 = 100000;
</code></pre>



<a name="orderbook_governance_MAX_TAKER_VOLATILE"></a>



<pre><code><b>const</b> <a href="../orderbook/governance.md#orderbook_governance_MAX_TAKER_VOLATILE">MAX_TAKER_VOLATILE</a>: u64 = 1000000;
</code></pre>



<a name="orderbook_governance_MIN_MAKER_VOLATILE"></a>



<pre><code><b>const</b> <a href="../orderbook/governance.md#orderbook_governance_MIN_MAKER_VOLATILE">MIN_MAKER_VOLATILE</a>: u64 = 0;
</code></pre>



<a name="orderbook_governance_MAX_MAKER_VOLATILE"></a>



<pre><code><b>const</b> <a href="../orderbook/governance.md#orderbook_governance_MAX_MAKER_VOLATILE">MAX_MAKER_VOLATILE</a>: u64 = 500000;
</code></pre>



<a name="orderbook_governance_MAX_PROPOSALS"></a>



<pre><code><b>const</b> <a href="../orderbook/governance.md#orderbook_governance_MAX_PROPOSALS">MAX_PROPOSALS</a>: u64 = 100;
</code></pre>



<a name="orderbook_governance_VOTING_POWER_THRESHOLD"></a>



<pre><code><b>const</b> <a href="../orderbook/governance.md#orderbook_governance_VOTING_POWER_THRESHOLD">VOTING_POWER_THRESHOLD</a>: u64 = 100000000000;
</code></pre>



<a name="orderbook_governance_empty"></a>

## Function `empty`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/governance.md#orderbook_governance_empty">empty</a>(<a href="../orderbook/governance.md#orderbook_governance_whitelisted">whitelisted</a>: bool, stable_pool: bool, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../orderbook/governance.md#orderbook_governance_Governance">orderbook::governance::Governance</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/governance.md#orderbook_governance_empty">empty</a>(<a href="../orderbook/governance.md#orderbook_governance_whitelisted">whitelisted</a>: bool, stable_pool: bool, ctx: &TxContext): <a href="../orderbook/governance.md#orderbook_governance_Governance">Governance</a> {
    <b>let</b> default_taker = <b>if</b> (<a href="../orderbook/governance.md#orderbook_governance_whitelisted">whitelisted</a>) {
        0
    } <b>else</b> <b>if</b> (stable_pool) {
        <a href="../orderbook/governance.md#orderbook_governance_MAX_TAKER_STABLE">MAX_TAKER_STABLE</a>
    } <b>else</b> {
        <a href="../orderbook/governance.md#orderbook_governance_MAX_TAKER_VOLATILE">MAX_TAKER_VOLATILE</a>
    };
    <b>let</b> default_maker = <b>if</b> (<a href="../orderbook/governance.md#orderbook_governance_whitelisted">whitelisted</a>) {
        0
    } <b>else</b> <b>if</b> (stable_pool) {
        <a href="../orderbook/governance.md#orderbook_governance_MAX_MAKER_STABLE">MAX_MAKER_STABLE</a>
    } <b>else</b> {
        <a href="../orderbook/governance.md#orderbook_governance_MAX_MAKER_VOLATILE">MAX_MAKER_VOLATILE</a>
    };
    <a href="../orderbook/governance.md#orderbook_governance_Governance">Governance</a> {
        epoch: ctx.epoch(),
        <a href="../orderbook/governance.md#orderbook_governance_whitelisted">whitelisted</a>,
        <a href="../orderbook/governance.md#orderbook_governance_stable">stable</a>: stable_pool,
        proposals: vec_map::empty(),
        <a href="../orderbook/trade_params.md#orderbook_trade_params">trade_params</a>: <a href="../orderbook/trade_params.md#orderbook_trade_params_new">trade_params::new</a>(
            default_taker,
            default_maker,
            <a href="../orderbook/constants.md#orderbook_constants_default_stake_required">constants::default_stake_required</a>(),
        ),
        <a href="../orderbook/governance.md#orderbook_governance_next_trade_params">next_trade_params</a>: <a href="../orderbook/trade_params.md#orderbook_trade_params_new">trade_params::new</a>(
            default_taker,
            default_maker,
            <a href="../orderbook/constants.md#orderbook_constants_default_stake_required">constants::default_stake_required</a>(),
        ),
        voting_power: 0,
        <a href="../orderbook/governance.md#orderbook_governance_quorum">quorum</a>: 0,
    }
}
</code></pre>



</details>

<a name="orderbook_governance_whitelisted"></a>

## Function `whitelisted`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/governance.md#orderbook_governance_whitelisted">whitelisted</a>(self: &<a href="../orderbook/governance.md#orderbook_governance_Governance">orderbook::governance::Governance</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/governance.md#orderbook_governance_whitelisted">whitelisted</a>(self: &<a href="../orderbook/governance.md#orderbook_governance_Governance">Governance</a>): bool {
    self.<a href="../orderbook/governance.md#orderbook_governance_whitelisted">whitelisted</a>
}
</code></pre>



</details>

<a name="orderbook_governance_stable"></a>

## Function `stable`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/governance.md#orderbook_governance_stable">stable</a>(self: &<a href="../orderbook/governance.md#orderbook_governance_Governance">orderbook::governance::Governance</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/governance.md#orderbook_governance_stable">stable</a>(self: &<a href="../orderbook/governance.md#orderbook_governance_Governance">Governance</a>): bool {
    self.<a href="../orderbook/governance.md#orderbook_governance_stable">stable</a>
}
</code></pre>



</details>

<a name="orderbook_governance_quorum"></a>

## Function `quorum`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/governance.md#orderbook_governance_quorum">quorum</a>(self: &<a href="../orderbook/governance.md#orderbook_governance_Governance">orderbook::governance::Governance</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/governance.md#orderbook_governance_quorum">quorum</a>(self: &<a href="../orderbook/governance.md#orderbook_governance_Governance">Governance</a>): u64 {
    self.<a href="../orderbook/governance.md#orderbook_governance_quorum">quorum</a>
}
</code></pre>



</details>

<a name="orderbook_governance_update"></a>

## Function `update`

Update the governance state. This is called at the start of every epoch.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/governance.md#orderbook_governance_update">update</a>(self: &<b>mut</b> <a href="../orderbook/governance.md#orderbook_governance_Governance">orderbook::governance::Governance</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/governance.md#orderbook_governance_update">update</a>(self: &<b>mut</b> <a href="../orderbook/governance.md#orderbook_governance_Governance">Governance</a>, ctx: &TxContext) {
    <b>let</b> epoch = ctx.epoch();
    <b>if</b> (self.epoch == epoch) <b>return</b>;
    self.epoch = epoch;
    self.<a href="../orderbook/governance.md#orderbook_governance_quorum">quorum</a> = <a href="../orderbook/math.md#orderbook_math_mul">math::mul</a>(self.voting_power, <a href="../orderbook/constants.md#orderbook_constants_half">constants::half</a>());
    self.proposals = vec_map::empty();
    self.<a href="../orderbook/trade_params.md#orderbook_trade_params">trade_params</a> = self.<a href="../orderbook/governance.md#orderbook_governance_next_trade_params">next_trade_params</a>;
    event::emit(<a href="../orderbook/governance.md#orderbook_governance_TradeParamsUpdateEvent">TradeParamsUpdateEvent</a> {
        taker_fee: self.<a href="../orderbook/trade_params.md#orderbook_trade_params">trade_params</a>.taker_fee(),
        maker_fee: self.<a href="../orderbook/trade_params.md#orderbook_trade_params">trade_params</a>.maker_fee(),
        stake_required: self.<a href="../orderbook/trade_params.md#orderbook_trade_params">trade_params</a>.stake_required(),
    });
}
</code></pre>



</details>

<a name="orderbook_governance_add_proposal"></a>

## Function `add_proposal`

Add a new proposal to governance.
Check if proposer already voted, if so will give error.
If proposer has not voted, and there are already MAX_PROPOSALS proposals,
remove the proposal with the lowest votes if it has less votes than the
voting power.
Validation of the account adding is done in <code>State</code>.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/governance.md#orderbook_governance_add_proposal">add_proposal</a>(self: &<b>mut</b> <a href="../orderbook/governance.md#orderbook_governance_Governance">orderbook::governance::Governance</a>, taker_fee: u64, maker_fee: u64, stake_required: u64, stake_amount: u64, balance_manager_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/governance.md#orderbook_governance_add_proposal">add_proposal</a>(
    self: &<b>mut</b> <a href="../orderbook/governance.md#orderbook_governance_Governance">Governance</a>,
    taker_fee: u64,
    maker_fee: u64,
    stake_required: u64,
    stake_amount: u64,
    balance_manager_id: ID,
) {
    <b>assert</b>!(!self.<a href="../orderbook/governance.md#orderbook_governance_whitelisted">whitelisted</a>, <a href="../orderbook/governance.md#orderbook_governance_EWhitelistedPoolCannotChange">EWhitelistedPoolCannotChange</a>);
    <b>assert</b>!(taker_fee % <a href="../orderbook/governance.md#orderbook_governance_FEE_MULTIPLE">FEE_MULTIPLE</a> == 0, <a href="../orderbook/governance.md#orderbook_governance_EInvalidTakerFee">EInvalidTakerFee</a>);
    <b>assert</b>!(maker_fee % <a href="../orderbook/governance.md#orderbook_governance_FEE_MULTIPLE">FEE_MULTIPLE</a> == 0, <a href="../orderbook/governance.md#orderbook_governance_EInvalidMakerFee">EInvalidMakerFee</a>);
    <b>if</b> (self.<a href="../orderbook/governance.md#orderbook_governance_stable">stable</a>) {
        <b>assert</b>!(taker_fee &gt;= <a href="../orderbook/governance.md#orderbook_governance_MIN_TAKER_STABLE">MIN_TAKER_STABLE</a> && taker_fee &lt;= <a href="../orderbook/governance.md#orderbook_governance_MAX_TAKER_STABLE">MAX_TAKER_STABLE</a>, <a href="../orderbook/governance.md#orderbook_governance_EInvalidTakerFee">EInvalidTakerFee</a>);
        <b>assert</b>!(maker_fee &gt;= <a href="../orderbook/governance.md#orderbook_governance_MIN_MAKER_STABLE">MIN_MAKER_STABLE</a> && maker_fee &lt;= <a href="../orderbook/governance.md#orderbook_governance_MAX_MAKER_STABLE">MAX_MAKER_STABLE</a>, <a href="../orderbook/governance.md#orderbook_governance_EInvalidMakerFee">EInvalidMakerFee</a>);
    } <b>else</b> {
        <b>assert</b>!(
            taker_fee &gt;= <a href="../orderbook/governance.md#orderbook_governance_MIN_TAKER_VOLATILE">MIN_TAKER_VOLATILE</a> && taker_fee &lt;= <a href="../orderbook/governance.md#orderbook_governance_MAX_TAKER_VOLATILE">MAX_TAKER_VOLATILE</a>,
            <a href="../orderbook/governance.md#orderbook_governance_EInvalidTakerFee">EInvalidTakerFee</a>,
        );
        <b>assert</b>!(
            maker_fee &gt;= <a href="../orderbook/governance.md#orderbook_governance_MIN_MAKER_VOLATILE">MIN_MAKER_VOLATILE</a> && maker_fee &lt;= <a href="../orderbook/governance.md#orderbook_governance_MAX_MAKER_VOLATILE">MAX_MAKER_VOLATILE</a>,
            <a href="../orderbook/governance.md#orderbook_governance_EInvalidMakerFee">EInvalidMakerFee</a>,
        );
    };
    <b>let</b> voting_power = <a href="../orderbook/governance.md#orderbook_governance_stake_to_voting_power">stake_to_voting_power</a>(stake_amount);
    <b>if</b> (self.proposals.length() == <a href="../orderbook/governance.md#orderbook_governance_MAX_PROPOSALS">MAX_PROPOSALS</a>) {
        self.<a href="../orderbook/governance.md#orderbook_governance_remove_lowest_proposal">remove_lowest_proposal</a>(voting_power);
    };
    <b>let</b> <a href="../orderbook/governance.md#orderbook_governance_new_proposal">new_proposal</a> = <a href="../orderbook/governance.md#orderbook_governance_new_proposal">new_proposal</a>(taker_fee, maker_fee, stake_required);
    self.proposals.insert(balance_manager_id, <a href="../orderbook/governance.md#orderbook_governance_new_proposal">new_proposal</a>);
}
</code></pre>



</details>

<a name="orderbook_governance_adjust_vote"></a>

## Function `adjust_vote`

Vote on a proposal. Validation of the account and stake is done in <code>State</code>.
If <code>from_proposal_id</code> is some, the account is removing their vote from that
proposal.
If <code>to_proposal_id</code> is some, the account is voting for that proposal.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/governance.md#orderbook_governance_adjust_vote">adjust_vote</a>(self: &<b>mut</b> <a href="../orderbook/governance.md#orderbook_governance_Governance">orderbook::governance::Governance</a>, from_proposal_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;, to_proposal_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;, stake_amount: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/governance.md#orderbook_governance_adjust_vote">adjust_vote</a>(
    self: &<b>mut</b> <a href="../orderbook/governance.md#orderbook_governance_Governance">Governance</a>,
    from_proposal_id: Option&lt;ID&gt;,
    to_proposal_id: Option&lt;ID&gt;,
    stake_amount: u64,
) {
    <b>let</b> votes = <a href="../orderbook/governance.md#orderbook_governance_stake_to_voting_power">stake_to_voting_power</a>(stake_amount);
    <b>if</b> (
        from_proposal_id.is_some() && self
            .proposals
            .contains(from_proposal_id.borrow())
    ) {
        <b>let</b> proposal = &<b>mut</b> self.proposals[from_proposal_id.borrow()];
        proposal.votes = proposal.votes - votes;
        <b>if</b> (proposal.votes + votes &gt; self.<a href="../orderbook/governance.md#orderbook_governance_quorum">quorum</a> && proposal.votes &lt; self.<a href="../orderbook/governance.md#orderbook_governance_quorum">quorum</a>) {
            self.<a href="../orderbook/governance.md#orderbook_governance_next_trade_params">next_trade_params</a> = self.<a href="../orderbook/trade_params.md#orderbook_trade_params">trade_params</a>;
        };
    };
    to_proposal_id.do_ref!(|proposal_id| {
        <b>assert</b>!(self.proposals.contains(proposal_id), <a href="../orderbook/governance.md#orderbook_governance_EProposalDoesNotExist">EProposalDoesNotExist</a>);
        <b>let</b> proposal = &<b>mut</b> self.proposals[proposal_id];
        proposal.votes = proposal.votes + votes;
        <b>if</b> (proposal.votes &gt; self.<a href="../orderbook/governance.md#orderbook_governance_quorum">quorum</a>) {
            self.<a href="../orderbook/governance.md#orderbook_governance_next_trade_params">next_trade_params</a> = proposal.<a href="../orderbook/governance.md#orderbook_governance_to_trade_params">to_trade_params</a>();
        };
    });
}
</code></pre>



</details>

<a name="orderbook_governance_adjust_voting_power"></a>

## Function `adjust_voting_power`

Adjust the total voting power by adding and removing stake. For example, if
an account's
stake goes from 2000 to 3000, then <code>stake_before</code> is 2000 and <code>stake_after</code>
is 3000.
Validation of inputs done in <code>State</code>.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/governance.md#orderbook_governance_adjust_voting_power">adjust_voting_power</a>(self: &<b>mut</b> <a href="../orderbook/governance.md#orderbook_governance_Governance">orderbook::governance::Governance</a>, stake_before: u64, stake_after: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/governance.md#orderbook_governance_adjust_voting_power">adjust_voting_power</a>(
    self: &<b>mut</b> <a href="../orderbook/governance.md#orderbook_governance_Governance">Governance</a>,
    stake_before: u64,
    stake_after: u64,
) {
    self.voting_power =
        self.voting_power +
        <a href="../orderbook/governance.md#orderbook_governance_stake_to_voting_power">stake_to_voting_power</a>(stake_after) -
        <a href="../orderbook/governance.md#orderbook_governance_stake_to_voting_power">stake_to_voting_power</a>(stake_before);
}
</code></pre>



</details>

<a name="orderbook_governance_trade_params"></a>

## Function `trade_params`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/trade_params.md#orderbook_trade_params">trade_params</a>(self: &<a href="../orderbook/governance.md#orderbook_governance_Governance">orderbook::governance::Governance</a>): <a href="../orderbook/trade_params.md#orderbook_trade_params_TradeParams">orderbook::trade_params::TradeParams</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/trade_params.md#orderbook_trade_params">trade_params</a>(self: &<a href="../orderbook/governance.md#orderbook_governance_Governance">Governance</a>): TradeParams {
    self.<a href="../orderbook/trade_params.md#orderbook_trade_params">trade_params</a>
}
</code></pre>



</details>

<a name="orderbook_governance_next_trade_params"></a>

## Function `next_trade_params`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/governance.md#orderbook_governance_next_trade_params">next_trade_params</a>(self: &<a href="../orderbook/governance.md#orderbook_governance_Governance">orderbook::governance::Governance</a>): <a href="../orderbook/trade_params.md#orderbook_trade_params_TradeParams">orderbook::trade_params::TradeParams</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/governance.md#orderbook_governance_next_trade_params">next_trade_params</a>(self: &<a href="../orderbook/governance.md#orderbook_governance_Governance">Governance</a>): TradeParams {
    self.<a href="../orderbook/governance.md#orderbook_governance_next_trade_params">next_trade_params</a>
}
</code></pre>



</details>

<a name="orderbook_governance_stake_to_voting_power"></a>

## Function `stake_to_voting_power`

Convert stake to voting power.


<pre><code><b>fun</b> <a href="../orderbook/governance.md#orderbook_governance_stake_to_voting_power">stake_to_voting_power</a>(stake: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../orderbook/governance.md#orderbook_governance_stake_to_voting_power">stake_to_voting_power</a>(stake: u64): u64 {
    <b>let</b> <b>mut</b> voting_power = stake.min(<a href="../orderbook/governance.md#orderbook_governance_VOTING_POWER_THRESHOLD">VOTING_POWER_THRESHOLD</a>);
    <b>if</b> (stake &gt; <a href="../orderbook/governance.md#orderbook_governance_VOTING_POWER_THRESHOLD">VOTING_POWER_THRESHOLD</a>) {
        voting_power =
            voting_power + <a href="../orderbook/math.md#orderbook_math_sqrt">math::sqrt</a>(stake, <a href="../orderbook/constants.md#orderbook_constants_myso_unit">constants::myso_unit</a>()) -
            <a href="../orderbook/math.md#orderbook_math_sqrt">math::sqrt</a>(<a href="../orderbook/governance.md#orderbook_governance_VOTING_POWER_THRESHOLD">VOTING_POWER_THRESHOLD</a>, <a href="../orderbook/constants.md#orderbook_constants_myso_unit">constants::myso_unit</a>());
    };
    voting_power
}
</code></pre>



</details>

<a name="orderbook_governance_new_proposal"></a>

## Function `new_proposal`



<pre><code><b>fun</b> <a href="../orderbook/governance.md#orderbook_governance_new_proposal">new_proposal</a>(taker_fee: u64, maker_fee: u64, stake_required: u64): <a href="../orderbook/governance.md#orderbook_governance_Proposal">orderbook::governance::Proposal</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../orderbook/governance.md#orderbook_governance_new_proposal">new_proposal</a>(taker_fee: u64, maker_fee: u64, stake_required: u64): <a href="../orderbook/governance.md#orderbook_governance_Proposal">Proposal</a> {
    <a href="../orderbook/governance.md#orderbook_governance_Proposal">Proposal</a> { taker_fee, maker_fee, stake_required, votes: 0 }
}
</code></pre>



</details>

<a name="orderbook_governance_remove_lowest_proposal"></a>

## Function `remove_lowest_proposal`

Remove the proposal with the lowest votes if it has less votes than the
voting power.
If there are multiple proposals with the same lowest votes, the latest one
is removed.


<pre><code><b>fun</b> <a href="../orderbook/governance.md#orderbook_governance_remove_lowest_proposal">remove_lowest_proposal</a>(self: &<b>mut</b> <a href="../orderbook/governance.md#orderbook_governance_Governance">orderbook::governance::Governance</a>, voting_power: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../orderbook/governance.md#orderbook_governance_remove_lowest_proposal">remove_lowest_proposal</a>(self: &<b>mut</b> <a href="../orderbook/governance.md#orderbook_governance_Governance">Governance</a>, voting_power: u64) {
    <b>let</b> <b>mut</b> removal_id = option::none();
    <b>let</b> <b>mut</b> cur_lowest_votes = <a href="../orderbook/constants.md#orderbook_constants_max_u64">constants::max_u64</a>();
    <b>let</b> (keys, values) = self.proposals.into_keys_values();
    self.proposals.length().do!(|i| {
        <b>let</b> proposal_votes = values[i].votes;
        <b>if</b> (proposal_votes &lt; voting_power && proposal_votes &lt;= cur_lowest_votes) {
            removal_id = option::some(keys[i]);
            cur_lowest_votes = proposal_votes;
        };
    });
    <b>assert</b>!(removal_id.is_some(), <a href="../orderbook/governance.md#orderbook_governance_EMaxProposalsReachedNotEnoughVotes">EMaxProposalsReachedNotEnoughVotes</a>);
    self.proposals.remove(removal_id.borrow());
}
</code></pre>



</details>

<a name="orderbook_governance_to_trade_params"></a>

## Function `to_trade_params`



<pre><code><b>fun</b> <a href="../orderbook/governance.md#orderbook_governance_to_trade_params">to_trade_params</a>(proposal: &<a href="../orderbook/governance.md#orderbook_governance_Proposal">orderbook::governance::Proposal</a>): <a href="../orderbook/trade_params.md#orderbook_trade_params_TradeParams">orderbook::trade_params::TradeParams</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../orderbook/governance.md#orderbook_governance_to_trade_params">to_trade_params</a>(proposal: &<a href="../orderbook/governance.md#orderbook_governance_Proposal">Proposal</a>): TradeParams {
    <a href="../orderbook/trade_params.md#orderbook_trade_params_new">trade_params::new</a>(
        proposal.taker_fee,
        proposal.maker_fee,
        proposal.stake_required,
    )
}
</code></pre>



</details>
