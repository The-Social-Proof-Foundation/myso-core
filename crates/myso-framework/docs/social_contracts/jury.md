---
title: Module `social_contracts::jury`
---

AI jury engine. A free roster of willing agents, a random reputation-gated draw, hidden
commit-reveal votes, round tallies with escalation, and equal pay for the jurors who took part.

```
open_case ─► EVIDENCE ─► AWAITING_DRAW ─select_jury─► SELECTING ─► VOTING ─finalize_round─┬─► FINAL
▲                          │ replace_no_shows              ├─► TALLIED ─(appeal)─┐
└──── next round ◄─────────┴───────────────────────────────┤                    │
└─► FAILED           ▼
AWAITING_DRAW (round + 1)
```

This module is a leaf: it knows nothing about tasks or proposals. An integrator (<code><a href="../social_contracts/agent_task.md#social_contracts_agent_task">agent_task</a></code>,
<code><a href="../social_contracts/governance.md#social_contracts_governance">governance</a></code>) opens a case with a fixed outcome menu (just <code>menu_len</code> indexes), adds evidence and
the fee, and later reads <code>final_outcome</code> to run one fixed action. The jury never picks a recipient
or an amount. Jurors hold no stake: the only thing at risk is <code>duty_score</code> and roster standing,
and they are paid an equal share of the case fee for taking part, never for how they voted.

A juror vote is a commitment <code>sha3_256(case_id ‖ round ‖ agent_id ‖ outcome ‖ rationale_digest ‖
bcs(evidence_refs) ‖ bundle_digest ‖ salt)</code> (see [<code><a href="../social_contracts/jury.md#social_contracts_jury_compute_commitment">compute_commitment</a></code>]). Binding the agent id and
the case id means a copied commitment can never be revealed by anyone else.


-  [Struct `JuryAdminCap`](#social_contracts_jury_JuryAdminCap)
-  [Struct `EligibilityParams`](#social_contracts_jury_EligibilityParams)
-  [Struct `TimingParams`](#social_contracts_jury_TimingParams)
-  [Struct `DutyParams`](#social_contracts_jury_DutyParams)
-  [Struct `SelectionParams`](#social_contracts_jury_SelectionParams)
-  [Struct `JuryConfig`](#social_contracts_jury_JuryConfig)
-  [Struct `RosterEntry`](#social_contracts_jury_RosterEntry)
-  [Struct `JurorRoster`](#social_contracts_jury_JurorRoster)
-  [Struct `EvidenceItem`](#social_contracts_jury_EvidenceItem)
-  [Struct `JurorSlot`](#social_contracts_jury_JurorSlot)
-  [Struct `RoundState`](#social_contracts_jury_RoundState)
-  [Struct `Exclusions`](#social_contracts_jury_Exclusions)
-  [Struct `CaseFees`](#social_contracts_jury_CaseFees)
-  [Struct `JuryCase`](#social_contracts_jury_JuryCase)
-  [Struct `JuryConfigUpdated`](#social_contracts_jury_JuryConfigUpdated)
-  [Struct `RosterJoined`](#social_contracts_jury_RosterJoined)
-  [Struct `RosterLeft`](#social_contracts_jury_RosterLeft)
-  [Struct `RosterSuspended`](#social_contracts_jury_RosterSuspended)
-  [Struct `DutyScoreChanged`](#social_contracts_jury_DutyScoreChanged)
-  [Struct `CaseOpened`](#social_contracts_jury_CaseOpened)
-  [Struct `EvidenceSubmitted`](#social_contracts_jury_EvidenceSubmitted)
-  [Struct `JuryDrawn`](#social_contracts_jury_JuryDrawn)
-  [Struct `AssignmentAccepted`](#social_contracts_jury_AssignmentAccepted)
-  [Struct `JurorReplaced`](#social_contracts_jury_JurorReplaced)
-  [Struct `VotingStarted`](#social_contracts_jury_VotingStarted)
-  [Struct `VoteCommitted`](#social_contracts_jury_VoteCommitted)
-  [Struct `VoteRevealed`](#social_contracts_jury_VoteRevealed)
-  [Struct `RoundFinalized`](#social_contracts_jury_RoundFinalized)
-  [Struct `CaseAppealed`](#social_contracts_jury_CaseAppealed)
-  [Struct `CaseFinalized`](#social_contracts_jury_CaseFinalized)
-  [Struct `CaseFailed`](#social_contracts_jury_CaseFailed)
-  [Struct `CaseCancelled`](#social_contracts_jury_CaseCancelled)
-  [Struct `JurorPaid`](#social_contracts_jury_JurorPaid)
-  [Struct `Snapshot`](#social_contracts_jury_Snapshot)
-  [Constants](#@Constants_0)
-  [Function `bootstrap_init`](#social_contracts_jury_bootstrap_init)
-  [Function `create_jury_admin_cap`](#social_contracts_jury_create_jury_admin_cap)
-  [Function `validate_config`](#social_contracts_jury_validate_config)
-  [Function `assert_config_live`](#social_contracts_jury_assert_config_live)
-  [Function `set_active`](#social_contracts_jury_set_active)
-  [Function `set_eligibility`](#social_contracts_jury_set_eligibility)
-  [Function `set_timing`](#social_contracts_jury_set_timing)
-  [Function `set_duty`](#social_contracts_jury_set_duty)
-  [Function `set_selection`](#social_contracts_jury_set_selection)
-  [Function `set_reputation_params`](#social_contracts_jury_set_reputation_params)
-  [Function `set_genesis_sunset`](#social_contracts_jury_set_genesis_sunset)
-  [Function `add_genesis_agent`](#social_contracts_jury_add_genesis_agent)
-  [Function `remove_genesis_agent`](#social_contracts_jury_remove_genesis_agent)
-  [Function `emit_config_updated`](#social_contracts_jury_emit_config_updated)
-  [Function `migrate_jury_config`](#social_contracts_jury_migrate_jury_config)
-  [Function `migrate_jury_roster`](#social_contracts_jury_migrate_jury_roster)
-  [Function `migrate_jury_case`](#social_contracts_jury_migrate_jury_case)
-  [Function `join_roster`](#social_contracts_jury_join_roster)
-  [Function `leave_roster`](#social_contracts_jury_leave_roster)
-  [Function `set_availability`](#social_contracts_jury_set_availability)
-  [Function `refresh_roster_entry`](#social_contracts_jury_refresh_roster_entry)
-  [Function `evict_ineligible`](#social_contracts_jury_evict_ineligible)
-  [Function `add_to_roster`](#social_contracts_jury_add_to_roster)
-  [Function `remove_from_roster`](#social_contracts_jury_remove_from_roster)
-  [Function `take_snapshot`](#social_contracts_jury_take_snapshot)
-  [Function `is_genesis_exempt`](#social_contracts_jury_is_genesis_exempt)
-  [Function `meets_reputation`](#social_contracts_jury_meets_reputation)
-  [Function `draw_eligible`](#social_contracts_jury_draw_eligible)
-  [Function `eligible_count`](#social_contracts_jury_eligible_count)
-  [Function `assert_roster_ready`](#social_contracts_jury_assert_roster_ready)
-  [Function `open_case`](#social_contracts_jury_open_case)
-  [Function `share_case`](#social_contracts_jury_share_case)
-  [Function `add_evidence`](#social_contracts_jury_add_evidence)
-  [Function `add_fee`](#social_contracts_jury_add_fee)
-  [Function `appeal`](#social_contracts_jury_appeal)
-  [Function `cancel_case`](#social_contracts_jury_cancel_case)
-  [Function `sweep_remainder`](#social_contracts_jury_sweep_remainder)
-  [Function `mark_executed`](#social_contracts_jury_mark_executed)
-  [Function `is_finished`](#social_contracts_jury_is_finished)
-  [Function `select_jury`](#social_contracts_jury_select_jury)
-  [Function `draw_jurors`](#social_contracts_jury_draw_jurors)
-  [Function `conflicted`](#social_contracts_jury_conflicted)
-  [Function `accept_assignment`](#social_contracts_jury_accept_assignment)
-  [Function `all_seats_accepted`](#social_contracts_jury_all_seats_accepted)
-  [Function `start_voting`](#social_contracts_jury_start_voting)
-  [Function `replace_no_shows`](#social_contracts_jury_replace_no_shows)
-  [Function `commit_vote`](#social_contracts_jury_commit_vote)
-  [Function `reveal_vote`](#social_contracts_jury_reveal_vote)
-  [Function `none_uncommitted`](#social_contracts_jury_none_uncommitted)
-  [Function `compute_commitment`](#social_contracts_jury_compute_commitment)
-  [Function `finalize_round`](#social_contracts_jury_finalize_round)
-  [Function `all_revealed`](#social_contracts_jury_all_revealed)
-  [Function `evaluate_round`](#social_contracts_jury_evaluate_round)
-  [Function `finalize_case`](#social_contracts_jury_finalize_case)
-  [Function `void_case`](#social_contracts_jury_void_case)
-  [Function `end_case_by_settlement`](#social_contracts_jury_end_case_by_settlement)
-  [Function `force_end`](#social_contracts_jury_force_end)
-  [Function `finalize_outcome`](#social_contracts_jury_finalize_outcome)
-  [Function `end_failed`](#social_contracts_jury_end_failed)
-  [Function `close_round`](#social_contracts_jury_close_round)
-  [Function `reward_duty`](#social_contracts_jury_reward_duty)
-  [Function `penalize_duty`](#social_contracts_jury_penalize_duty)
-  [Function `release_seat`](#social_contracts_jury_release_seat)
-  [Function `settle_fees`](#social_contracts_jury_settle_fees)
-  [Function `claim_juror_fee`](#social_contracts_jury_claim_juror_fee)
-  [Function `assert_live`](#social_contracts_jury_assert_live)
-  [Function `assert_juror_actor`](#social_contracts_jury_assert_juror_actor)
-  [Function `subject_task`](#social_contracts_jury_subject_task)
-  [Function `subject_governance`](#social_contracts_jury_subject_governance)
-  [Function `abstain`](#social_contracts_jury_abstain)
-  [Function `case_evidence`](#social_contracts_jury_case_evidence)
-  [Function `case_awaiting_draw`](#social_contracts_jury_case_awaiting_draw)
-  [Function `case_selecting`](#social_contracts_jury_case_selecting)
-  [Function `case_voting`](#social_contracts_jury_case_voting)
-  [Function `case_tallied`](#social_contracts_jury_case_tallied)
-  [Function `case_final`](#social_contracts_jury_case_final)
-  [Function `case_failed`](#social_contracts_jury_case_failed)
-  [Function `case_cancelled`](#social_contracts_jury_case_cancelled)
-  [Function `fail_cannot_form`](#social_contracts_jury_fail_cannot_form)
-  [Function `fail_cannot_decide`](#social_contracts_jury_fail_cannot_decide)
-  [Function `fail_timeout`](#social_contracts_jury_fail_timeout)
-  [Function `fail_voided`](#social_contracts_jury_fail_voided)
-  [Function `slot_drafted`](#social_contracts_jury_slot_drafted)
-  [Function `slot_accepted`](#social_contracts_jury_slot_accepted)
-  [Function `slot_committed`](#social_contracts_jury_slot_committed)
-  [Function `slot_revealed`](#social_contracts_jury_slot_revealed)
-  [Function `slot_replaced`](#social_contracts_jury_slot_replaced)
-  [Function `slot_defaulted`](#social_contracts_jury_slot_defaulted)
-  [Function `roster_active`](#social_contracts_jury_roster_active)
-  [Function `roster_paused`](#social_contracts_jury_roster_paused)
-  [Function `roster_suspended`](#social_contracts_jury_roster_suspended)
-  [Function `config_active`](#social_contracts_jury_config_active)
-  [Function `max_exclusions`](#social_contracts_jury_max_exclusions)
-  [Function `neutral_window_ms`](#social_contracts_jury_neutral_window_ms)
-  [Function `accept_window_ms`](#social_contracts_jury_accept_window_ms)
-  [Function `commit_window_ms`](#social_contracts_jury_commit_window_ms)
-  [Function `reveal_window_ms`](#social_contracts_jury_reveal_window_ms)
-  [Function `reveal_grace_ms`](#social_contracts_jury_reveal_grace_ms)
-  [Function `appeal_window_ms`](#social_contracts_jury_appeal_window_ms)
-  [Function `max_case_duration_ms`](#social_contracts_jury_max_case_duration_ms)
-  [Function `min_eligible_roster`](#social_contracts_jury_min_eligible_roster)
-  [Function `initial_duty_score`](#social_contracts_jury_initial_duty_score)
-  [Function `genesis_sunset_ms`](#social_contracts_jury_genesis_sunset_ms)
-  [Function `roster_count`](#social_contracts_jury_roster_count)
-  [Function `on_roster`](#social_contracts_jury_on_roster)
-  [Function `entry_status`](#social_contracts_jury_entry_status)
-  [Function `entry_duty_score`](#social_contracts_jury_entry_duty_score)
-  [Function `entry_strikes`](#social_contracts_jury_entry_strikes)
-  [Function `entry_completed`](#social_contracts_jury_entry_completed)
-  [Function `entry_no_shows`](#social_contracts_jury_entry_no_shows)
-  [Function `entry_active_cases`](#social_contracts_jury_entry_active_cases)
-  [Function `entry_exempt`](#social_contracts_jury_entry_exempt)
-  [Function `entry_owner`](#social_contracts_jury_entry_owner)
-  [Function `entry_suspended_until_ms`](#social_contracts_jury_entry_suspended_until_ms)
-  [Function `case_state`](#social_contracts_jury_case_state)
-  [Function `case_is_finished`](#social_contracts_jury_case_is_finished)
-  [Function `case_undrawn`](#social_contracts_jury_case_undrawn)
-  [Function `case_subject_kind`](#social_contracts_jury_case_subject_kind)
-  [Function `case_subject_id`](#social_contracts_jury_case_subject_id)
-  [Function `case_stage`](#social_contracts_jury_case_stage)
-  [Function `case_round`](#social_contracts_jury_case_round)
-  [Function `case_menu_len`](#social_contracts_jury_case_menu_len)
-  [Function `case_failure_path`](#social_contracts_jury_case_failure_path)
-  [Function `case_fail_reason`](#social_contracts_jury_case_fail_reason)
-  [Function `case_final_outcome`](#social_contracts_jury_case_final_outcome)
-  [Function `case_provisional_outcome`](#social_contracts_jury_case_provisional_outcome)
-  [Function `case_prior_outcome`](#social_contracts_jury_case_prior_outcome)
-  [Function `case_executed`](#social_contracts_jury_case_executed)
-  [Function `case_failed_appeal`](#social_contracts_jury_case_failed_appeal)
-  [Function `case_appeals_used`](#social_contracts_jury_case_appeals_used)
-  [Function `case_appeal_deadline_ms`](#social_contracts_jury_case_appeal_deadline_ms)
-  [Function `case_evidence_deadline_ms`](#social_contracts_jury_case_evidence_deadline_ms)
-  [Function `case_evidence_count`](#social_contracts_jury_case_evidence_count)
-  [Function `case_participants`](#social_contracts_jury_case_participants)
-  [Function `case_per_juror_fee`](#social_contracts_jury_case_per_juror_fee)
-  [Function `case_fee_pool_mist`](#social_contracts_jury_case_fee_pool_mist)
-  [Function `case_fees_settled`](#social_contracts_jury_case_fees_settled)
-  [Function `case_sweep_to`](#social_contracts_jury_case_sweep_to)
-  [Function `finalized_outcome`](#social_contracts_jury_finalized_outcome)
-  [Function `round_jurors`](#social_contracts_jury_round_jurors)
-  [Function `round_tally`](#social_contracts_jury_round_tally)
-  [Function `round_revealed`](#social_contracts_jury_round_revealed)
-  [Function `round_accept_deadline_ms`](#social_contracts_jury_round_accept_deadline_ms)
-  [Function `round_commit_deadline_ms`](#social_contracts_jury_round_commit_deadline_ms)
-  [Function `round_reveal_deadline_ms`](#social_contracts_jury_round_reveal_deadline_ms)
-  [Function `has_slot`](#social_contracts_jury_has_slot)
-  [Function `slot_status`](#social_contracts_jury_slot_status)
-  [Function `slot_owner`](#social_contracts_jury_slot_owner)


<pre><code><b>use</b> <a href="../myso/accumulator.md#myso_accumulator">myso::accumulator</a>;
<b>use</b> <a href="../myso/accumulator_settlement.md#myso_accumulator_settlement">myso::accumulator_settlement</a>;
<b>use</b> <a href="../myso/address.md#myso_address">myso::address</a>;
<b>use</b> <a href="../myso/bag.md#myso_bag">myso::bag</a>;
<b>use</b> <a href="../myso/balance.md#myso_balance">myso::balance</a>;
<b>use</b> <a href="../myso/bcs.md#myso_bcs">myso::bcs</a>;
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
<b>use</b> <a href="../myso/random.md#myso_random">myso::random</a>;
<b>use</b> <a href="../myso/table.md#myso_table">myso::table</a>;
<b>use</b> <a href="../myso/transfer.md#myso_transfer">myso::transfer</a>;
<b>use</b> <a href="../myso/tx_context.md#myso_tx_context">myso::tx_context</a>;
<b>use</b> <a href="../myso/types.md#myso_types">myso::types</a>;
<b>use</b> <a href="../myso/unpause_cap.md#myso_unpause_cap">myso::unpause_cap</a>;
<b>use</b> <a href="../myso/url.md#myso_url">myso::url</a>;
<b>use</b> <a href="../myso/vec_map.md#myso_vec_map">myso::vec_map</a>;
<b>use</b> <a href="../myso/vec_set.md#myso_vec_set">myso::vec_set</a>;
<b>use</b> <a href="../myso/versioned.md#myso_versioned">myso::versioned</a>;
<b>use</b> <a href="../social_contracts/memory.md#social_contracts_memory">social_contracts::memory</a>;
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



<a name="social_contracts_jury_JuryAdminCap"></a>

## Struct `JuryAdminCap`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/jury.md#social_contracts_jury_JuryAdminCap">JuryAdminCap</a> <b>has</b> key, store
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

<a name="social_contracts_jury_EligibilityParams"></a>

## Struct `EligibilityParams`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/jury.md#social_contracts_jury_EligibilityParams">EligibilityParams</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>min_score_milli: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>min_confidence_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>min_settled_count: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>min_age_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_snapshot_age_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_active_cases: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/jury.md#social_contracts_jury_min_eligible_roster">min_eligible_roster</a>: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_jury_TimingParams"></a>

## Struct `TimingParams`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/jury.md#social_contracts_jury_TimingParams">TimingParams</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../social_contracts/jury.md#social_contracts_jury_accept_window_ms">accept_window_ms</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/jury.md#social_contracts_jury_commit_window_ms">commit_window_ms</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/jury.md#social_contracts_jury_reveal_window_ms">reveal_window_ms</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/jury.md#social_contracts_jury_reveal_grace_ms">reveal_grace_ms</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/jury.md#social_contracts_jury_appeal_window_ms">appeal_window_ms</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/jury.md#social_contracts_jury_neutral_window_ms">neutral_window_ms</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/jury.md#social_contracts_jury_max_case_duration_ms">max_case_duration_ms</a>: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_jury_DutyParams"></a>

## Struct `DutyParams`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/jury.md#social_contracts_jury_DutyParams">DutyParams</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>initial_score: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_score: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>min_score: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>gain: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>loss_accept: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>loss_noreveal: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_strikes: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>suspension_ms: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_jury_SelectionParams"></a>

## Struct `SelectionParams`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/jury.md#social_contracts_jury_SelectionParams">SelectionParams</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>max_draws: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_replacements: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_evidence_per_party: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/jury.md#social_contracts_jury_max_exclusions">max_exclusions</a>: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_jury_JuryConfig"></a>

## Struct `JuryConfig`

Global jury parameters. Per-registry governance policy lives on the registry, not here.


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">JuryConfig</a> <b>has</b> key
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
</dd>
<dt>
<code>active: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>eligibility: <a href="../social_contracts/jury.md#social_contracts_jury_EligibilityParams">social_contracts::jury::EligibilityParams</a></code>
</dt>
<dd>
</dd>
<dt>
<code>timing: <a href="../social_contracts/jury.md#social_contracts_jury_TimingParams">social_contracts::jury::TimingParams</a></code>
</dt>
<dd>
</dd>
<dt>
<code>duty: <a href="../social_contracts/jury.md#social_contracts_jury_DutyParams">social_contracts::jury::DutyParams</a></code>
</dt>
<dd>
</dd>
<dt>
<code>selection: <a href="../social_contracts/jury.md#social_contracts_jury_SelectionParams">social_contracts::jury::SelectionParams</a></code>
</dt>
<dd>
</dd>
<dt>
<code>reputation: <a href="../social_contracts/memory.md#social_contracts_memory_ReputationParams">social_contracts::memory::ReputationParams</a></code>
</dt>
<dd>
 Scores a juror is judged by. Keep in step with the task market's <code>ReputationParams</code>.
</dd>
<dt>
<code><a href="../social_contracts/jury.md#social_contracts_jury_genesis_sunset_ms">genesis_sunset_ms</a>: u64</code>
</dt>
<dd>
 Genesis agents skip the reputation bar until this time, then must qualify organically.
</dd>
<dt>
<code>genesis_agents: <a href="../myso/vec_set.md#myso_vec_set_VecSet">myso::vec_set::VecSet</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_jury_RosterEntry"></a>

## Struct `RosterEntry`

One willing agent. No balances: nothing is held, so nothing can be slashed.


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/jury.md#social_contracts_jury_RosterEntry">RosterEntry</a> <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>agent_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>owner: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>memory_account_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>organization_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>ancestor_ids: vector&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>payout_address: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>model_family: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>status: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>suspended_until_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>exempt: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>duty_score: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>completed: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>no_shows: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>strikes: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>last_duty_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>active_cases: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>joined_at_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>snapshot_at_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>snapshot_ok: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>snapshot_score_milli: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>snapshot_confidence_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>snapshot_settled_count: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>snapshot_created_at_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>snapshot_expires_at: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_jury_JurorRoster"></a>

## Struct `JurorRoster`

The roster with a dense index for O(1) uniform sampling.


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/jury.md#social_contracts_jury_JurorRoster">JurorRoster</a> <b>has</b> key
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
</dd>
<dt>
<code>slots: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;u64, <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>index: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>entries: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, <a href="../social_contracts/jury.md#social_contracts_jury_RosterEntry">social_contracts::jury::RosterEntry</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>count: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_jury_EvidenceItem"></a>

## Struct `EvidenceItem`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/jury.md#social_contracts_jury_EvidenceItem">EvidenceItem</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>submitter: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>round: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>kind: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>digest: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>uri: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>submitted_at_ms: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_jury_JurorSlot"></a>

## Struct `JurorSlot`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/jury.md#social_contracts_jury_JurorSlot">JurorSlot</a> <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>agent_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>owner: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>organization_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>payout_address: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>round: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>status: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>commitment: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>outcome: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>rationale_digest: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>evidence_refs: vector&lt;u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>bundle_digest: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>drafted_at_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>accepted_at_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>revealed_at_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>claimed: bool</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_jury_RoundState"></a>

## Struct `RoundState`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/jury.md#social_contracts_jury_RoundState">RoundState</a> <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>size: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>jurors: vector&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
 Current seat holders; a replaced juror leaves this list but keeps its slot record.
</dd>
<dt>
<code>accept_deadline_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>commit_deadline_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>reveal_deadline_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>replacements_used: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>tally: vector&lt;u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/jury.md#social_contracts_jury_abstain">abstain</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>revealed: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>closed: bool</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_jury_Exclusions"></a>

## Struct `Exclusions`

Who can never sit on this case, and who already has.


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/jury.md#social_contracts_jury_Exclusions">Exclusions</a> <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>owners: vector&lt;<b>address</b>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>orgs: vector&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>agents: vector&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>used_owners: vector&lt;<b>address</b>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>used_orgs: vector&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_jury_CaseFees"></a>

## Struct `CaseFees`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/jury.md#social_contracts_jury_CaseFees">CaseFees</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>pool: <a href="../myso/balance.md#myso_balance_Balance">myso::balance::Balance</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>max_juror_fee: u64</code>
</dt>
<dd>
 Cap on one juror's share; 0 = none (governance pays an equal share of the whole pool).
</dd>
<dt>
<code>sweep_to: <b>address</b></code>
</dt>
<dd>
 Where the integrator sends what is left after jurors are paid.
</dd>
<dt>
<code>participants: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>per_juror_fee: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>reserved: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>settled: bool</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_jury_JuryCase"></a>

## Struct `JuryCase`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">JuryCase</a> <b>has</b> key
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
</dd>
<dt>
<code>subject_kind: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>subject_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>stage: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>state: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>fail_reason: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>round: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>round_sizes: vector&lt;u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>menu_len: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>approval_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>quorum: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>appealable: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>failure_path: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>opened_by: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>created_at_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>evidence_deadline_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>appeal_deadline_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>prior_outcome: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>provisional_outcome: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>final_outcome: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>executed: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>appeals_used: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>failed_appeal: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>exclusions: <a href="../social_contracts/jury.md#social_contracts_jury_Exclusions">social_contracts::jury::Exclusions</a></code>
</dt>
<dd>
</dd>
<dt>
<code>evidence: vector&lt;<a href="../social_contracts/jury.md#social_contracts_jury_EvidenceItem">social_contracts::jury::EvidenceItem</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>rounds: vector&lt;<a href="../social_contracts/jury.md#social_contracts_jury_RoundState">social_contracts::jury::RoundState</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>slots: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, <a href="../social_contracts/jury.md#social_contracts_jury_JurorSlot">social_contracts::jury::JurorSlot</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>fees: <a href="../social_contracts/jury.md#social_contracts_jury_CaseFees">social_contracts::jury::CaseFees</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_jury_JuryConfigUpdated"></a>

## Struct `JuryConfigUpdated`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/jury.md#social_contracts_jury_JuryConfigUpdated">JuryConfigUpdated</a> <b>has</b> <b>copy</b>, drop
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
<code>section: u8</code>
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

<a name="social_contracts_jury_RosterJoined"></a>

## Struct `RosterJoined`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/jury.md#social_contracts_jury_RosterJoined">RosterJoined</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>agent_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>owner: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>memory_account_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>organization_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>payout_address: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>model_family: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>exempt: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>joined_at_ms: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_jury_RosterLeft"></a>

## Struct `RosterLeft`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/jury.md#social_contracts_jury_RosterLeft">RosterLeft</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>agent_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>reason: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>left_at_ms: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_jury_RosterSuspended"></a>

## Struct `RosterSuspended`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/jury.md#social_contracts_jury_RosterSuspended">RosterSuspended</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>agent_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>until_ms: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_jury_DutyScoreChanged"></a>

## Struct `DutyScoreChanged`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/jury.md#social_contracts_jury_DutyScoreChanged">DutyScoreChanged</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>agent_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>score: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>strikes: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>reason: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>case_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_jury_CaseOpened"></a>

## Struct `CaseOpened`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/jury.md#social_contracts_jury_CaseOpened">CaseOpened</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>case_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>subject_kind: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>subject_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>stage: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>menu_len: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>approval_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>quorum: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>round_sizes: vector&lt;u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>appealable: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>failure_path: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>opened_by: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>fee_mist: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>created_at_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>evidence_deadline_ms: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_jury_EvidenceSubmitted"></a>

## Struct `EvidenceSubmitted`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/jury.md#social_contracts_jury_EvidenceSubmitted">EvidenceSubmitted</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>case_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>index: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>submitter: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>round: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>kind: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>digest: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>uri: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>submitted_at_ms: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_jury_JuryDrawn"></a>

## Struct `JuryDrawn`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/jury.md#social_contracts_jury_JuryDrawn">JuryDrawn</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>case_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>round: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>jurors: vector&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>accept_deadline_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>replacement: bool</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_jury_AssignmentAccepted"></a>

## Struct `AssignmentAccepted`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/jury.md#social_contracts_jury_AssignmentAccepted">AssignmentAccepted</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>case_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>round: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>agent_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>accepted_at_ms: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_jury_JurorReplaced"></a>

## Struct `JurorReplaced`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/jury.md#social_contracts_jury_JurorReplaced">JurorReplaced</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>case_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>round: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>agent_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_jury_VotingStarted"></a>

## Struct `VotingStarted`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/jury.md#social_contracts_jury_VotingStarted">VotingStarted</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>case_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>round: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>jurors: vector&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>commit_deadline_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>reveal_deadline_ms: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_jury_VoteCommitted"></a>

## Struct `VoteCommitted`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/jury.md#social_contracts_jury_VoteCommitted">VoteCommitted</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>case_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>round: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>agent_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>commitment: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_jury_VoteRevealed"></a>

## Struct `VoteRevealed`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/jury.md#social_contracts_jury_VoteRevealed">VoteRevealed</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>case_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>round: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>agent_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>outcome: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>rationale_digest: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>evidence_refs: vector&lt;u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>bundle_digest: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>revealed_at_ms: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_jury_RoundFinalized"></a>

## Struct `RoundFinalized`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/jury.md#social_contracts_jury_RoundFinalized">RoundFinalized</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>case_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>round: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>tally: vector&lt;u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/jury.md#social_contracts_jury_abstain">abstain</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>revealed: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>decisive: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>leading: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>appeal_deadline_ms: u64</code>
</dt>
<dd>
 When the appeal window closes (and anyone may finalize); 0 if no window opened.
</dd>
</dl>


</details>

<a name="social_contracts_jury_CaseAppealed"></a>

## Struct `CaseAppealed`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/jury.md#social_contracts_jury_CaseAppealed">CaseAppealed</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>case_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>by: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>round: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>prior_outcome: u8</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_jury_CaseFinalized"></a>

## Struct `CaseFinalized`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/jury.md#social_contracts_jury_CaseFinalized">CaseFinalized</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>case_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>outcome: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>round: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>appeals_used: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>failed_appeal: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>participants: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>per_juror_fee: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>finalized_at_ms: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_jury_CaseFailed"></a>

## Struct `CaseFailed`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/jury.md#social_contracts_jury_CaseFailed">CaseFailed</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>case_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>reason: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>round: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>participants: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>per_juror_fee: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>failed_at_ms: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_jury_CaseCancelled"></a>

## Struct `CaseCancelled`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/jury.md#social_contracts_jury_CaseCancelled">CaseCancelled</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>case_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>cancelled_at_ms: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_jury_JurorPaid"></a>

## Struct `JurorPaid`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/jury.md#social_contracts_jury_JurorPaid">JurorPaid</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>case_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>agent_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>recipient: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>amount_mist: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_jury_Snapshot"></a>

## Struct `Snapshot`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/jury.md#social_contracts_jury_Snapshot">Snapshot</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>ok: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>score_milli: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>confidence_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>settled_count: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>created_at_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>expires_at: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>ancestors: vector&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="social_contracts_jury_SUBJECT_TASK"></a>



<pre><code><b>const</b> <a href="../social_contracts/jury.md#social_contracts_jury_SUBJECT_TASK">SUBJECT_TASK</a>: u8 = 1;
</code></pre>



<a name="social_contracts_jury_SUBJECT_GOVERNANCE"></a>



<pre><code><b>const</b> <a href="../social_contracts/jury.md#social_contracts_jury_SUBJECT_GOVERNANCE">SUBJECT_GOVERNANCE</a>: u8 = 2;
</code></pre>



<a name="social_contracts_jury_CASE_EVIDENCE"></a>



<pre><code><b>const</b> <a href="../social_contracts/jury.md#social_contracts_jury_CASE_EVIDENCE">CASE_EVIDENCE</a>: u8 = 1;
</code></pre>



<a name="social_contracts_jury_CASE_AWAITING_DRAW"></a>



<pre><code><b>const</b> <a href="../social_contracts/jury.md#social_contracts_jury_CASE_AWAITING_DRAW">CASE_AWAITING_DRAW</a>: u8 = 2;
</code></pre>



<a name="social_contracts_jury_CASE_SELECTING"></a>



<pre><code><b>const</b> <a href="../social_contracts/jury.md#social_contracts_jury_CASE_SELECTING">CASE_SELECTING</a>: u8 = 3;
</code></pre>



<a name="social_contracts_jury_CASE_VOTING"></a>



<pre><code><b>const</b> <a href="../social_contracts/jury.md#social_contracts_jury_CASE_VOTING">CASE_VOTING</a>: u8 = 4;
</code></pre>



<a name="social_contracts_jury_CASE_TALLIED"></a>



<pre><code><b>const</b> <a href="../social_contracts/jury.md#social_contracts_jury_CASE_TALLIED">CASE_TALLIED</a>: u8 = 5;
</code></pre>



<a name="social_contracts_jury_CASE_FINAL"></a>



<pre><code><b>const</b> <a href="../social_contracts/jury.md#social_contracts_jury_CASE_FINAL">CASE_FINAL</a>: u8 = 6;
</code></pre>



<a name="social_contracts_jury_CASE_FAILED"></a>



<pre><code><b>const</b> <a href="../social_contracts/jury.md#social_contracts_jury_CASE_FAILED">CASE_FAILED</a>: u8 = 7;
</code></pre>



<a name="social_contracts_jury_CASE_CANCELLED"></a>



<pre><code><b>const</b> <a href="../social_contracts/jury.md#social_contracts_jury_CASE_CANCELLED">CASE_CANCELLED</a>: u8 = 8;
</code></pre>



<a name="social_contracts_jury_FAIL_NONE"></a>



<pre><code><b>const</b> <a href="../social_contracts/jury.md#social_contracts_jury_FAIL_NONE">FAIL_NONE</a>: u8 = 0;
</code></pre>



<a name="social_contracts_jury_FAIL_CANNOT_FORM"></a>



<pre><code><b>const</b> <a href="../social_contracts/jury.md#social_contracts_jury_FAIL_CANNOT_FORM">FAIL_CANNOT_FORM</a>: u8 = 1;
</code></pre>



<a name="social_contracts_jury_FAIL_CANNOT_DECIDE"></a>



<pre><code><b>const</b> <a href="../social_contracts/jury.md#social_contracts_jury_FAIL_CANNOT_DECIDE">FAIL_CANNOT_DECIDE</a>: u8 = 2;
</code></pre>



<a name="social_contracts_jury_FAIL_TIMEOUT"></a>



<pre><code><b>const</b> <a href="../social_contracts/jury.md#social_contracts_jury_FAIL_TIMEOUT">FAIL_TIMEOUT</a>: u8 = 3;
</code></pre>



<a name="social_contracts_jury_FAIL_VOIDED"></a>



<pre><code><b>const</b> <a href="../social_contracts/jury.md#social_contracts_jury_FAIL_VOIDED">FAIL_VOIDED</a>: u8 = 4;
</code></pre>



<a name="social_contracts_jury_SLOT_DRAFTED"></a>



<pre><code><b>const</b> <a href="../social_contracts/jury.md#social_contracts_jury_SLOT_DRAFTED">SLOT_DRAFTED</a>: u8 = 1;
</code></pre>



<a name="social_contracts_jury_SLOT_ACCEPTED"></a>



<pre><code><b>const</b> <a href="../social_contracts/jury.md#social_contracts_jury_SLOT_ACCEPTED">SLOT_ACCEPTED</a>: u8 = 2;
</code></pre>



<a name="social_contracts_jury_SLOT_COMMITTED"></a>



<pre><code><b>const</b> <a href="../social_contracts/jury.md#social_contracts_jury_SLOT_COMMITTED">SLOT_COMMITTED</a>: u8 = 3;
</code></pre>



<a name="social_contracts_jury_SLOT_REVEALED"></a>



<pre><code><b>const</b> <a href="../social_contracts/jury.md#social_contracts_jury_SLOT_REVEALED">SLOT_REVEALED</a>: u8 = 4;
</code></pre>



<a name="social_contracts_jury_SLOT_REPLACED"></a>



<pre><code><b>const</b> <a href="../social_contracts/jury.md#social_contracts_jury_SLOT_REPLACED">SLOT_REPLACED</a>: u8 = 5;
</code></pre>



<a name="social_contracts_jury_SLOT_DEFAULTED"></a>



<pre><code><b>const</b> <a href="../social_contracts/jury.md#social_contracts_jury_SLOT_DEFAULTED">SLOT_DEFAULTED</a>: u8 = 6;
</code></pre>



<a name="social_contracts_jury_ROSTER_ACTIVE"></a>



<pre><code><b>const</b> <a href="../social_contracts/jury.md#social_contracts_jury_ROSTER_ACTIVE">ROSTER_ACTIVE</a>: u8 = 1;
</code></pre>



<a name="social_contracts_jury_ROSTER_PAUSED"></a>



<pre><code><b>const</b> <a href="../social_contracts/jury.md#social_contracts_jury_ROSTER_PAUSED">ROSTER_PAUSED</a>: u8 = 2;
</code></pre>



<a name="social_contracts_jury_ROSTER_SUSPENDED"></a>



<pre><code><b>const</b> <a href="../social_contracts/jury.md#social_contracts_jury_ROSTER_SUSPENDED">ROSTER_SUSPENDED</a>: u8 = 3;
</code></pre>



<a name="social_contracts_jury_DUTY_COMPLETED"></a>

<code><a href="../social_contracts/jury.md#social_contracts_jury_DutyScoreChanged">DutyScoreChanged</a>.reason</code>.


<pre><code><b>const</b> <a href="../social_contracts/jury.md#social_contracts_jury_DUTY_COMPLETED">DUTY_COMPLETED</a>: u8 = 1;
</code></pre>



<a name="social_contracts_jury_DUTY_NO_ACCEPT"></a>



<pre><code><b>const</b> <a href="../social_contracts/jury.md#social_contracts_jury_DUTY_NO_ACCEPT">DUTY_NO_ACCEPT</a>: u8 = 2;
</code></pre>



<a name="social_contracts_jury_DUTY_NO_REVEAL"></a>



<pre><code><b>const</b> <a href="../social_contracts/jury.md#social_contracts_jury_DUTY_NO_REVEAL">DUTY_NO_REVEAL</a>: u8 = 3;
</code></pre>



<a name="social_contracts_jury_ABSTAIN"></a>

A vote that backs no option. It counts toward quorum, never toward an option.


<pre><code><b>const</b> <a href="../social_contracts/jury.md#social_contracts_jury_ABSTAIN">ABSTAIN</a>: u8 = 255;
</code></pre>



<a name="social_contracts_jury_MAX_MENU_LEN"></a>



<pre><code><b>const</b> <a href="../social_contracts/jury.md#social_contracts_jury_MAX_MENU_LEN">MAX_MENU_LEN</a>: u8 = 16;
</code></pre>



<a name="social_contracts_jury_MAX_ROUNDS_LIMIT"></a>



<pre><code><b>const</b> <a href="../social_contracts/jury.md#social_contracts_jury_MAX_ROUNDS_LIMIT">MAX_ROUNDS_LIMIT</a>: u64 = 4;
</code></pre>



<a name="social_contracts_jury_DIGEST_LENGTH"></a>



<pre><code><b>const</b> <a href="../social_contracts/jury.md#social_contracts_jury_DIGEST_LENGTH">DIGEST_LENGTH</a>: u64 = 32;
</code></pre>



<a name="social_contracts_jury_MIN_SALT_BYTES"></a>



<pre><code><b>const</b> <a href="../social_contracts/jury.md#social_contracts_jury_MIN_SALT_BYTES">MIN_SALT_BYTES</a>: u64 = 16;
</code></pre>



<a name="social_contracts_jury_MAX_SALT_BYTES"></a>



<pre><code><b>const</b> <a href="../social_contracts/jury.md#social_contracts_jury_MAX_SALT_BYTES">MAX_SALT_BYTES</a>: u64 = 64;
</code></pre>



<a name="social_contracts_jury_MAX_URI_BYTES"></a>



<pre><code><b>const</b> <a href="../social_contracts/jury.md#social_contracts_jury_MAX_URI_BYTES">MAX_URI_BYTES</a>: u64 = 2048;
</code></pre>



<a name="social_contracts_jury_BPS_DENOMINATOR"></a>



<pre><code><b>const</b> <a href="../social_contracts/jury.md#social_contracts_jury_BPS_DENOMINATOR">BPS_DENOMINATOR</a>: u64 = 10000;
</code></pre>



<a name="social_contracts_jury_MS_HOUR"></a>



<pre><code><b>const</b> <a href="../social_contracts/jury.md#social_contracts_jury_MS_HOUR">MS_HOUR</a>: u64 = 3600000;
</code></pre>



<a name="social_contracts_jury_MS_DAY"></a>



<pre><code><b>const</b> <a href="../social_contracts/jury.md#social_contracts_jury_MS_DAY">MS_DAY</a>: u64 = 86400000;
</code></pre>



<a name="social_contracts_jury_EWrongVersion"></a>



<pre><code><b>const</b> <a href="../social_contracts/jury.md#social_contracts_jury_EWrongVersion">EWrongVersion</a>: u64 = 1;
</code></pre>



<a name="social_contracts_jury_EJuryPaused"></a>



<pre><code><b>const</b> <a href="../social_contracts/jury.md#social_contracts_jury_EJuryPaused">EJuryPaused</a>: u64 = 2;
</code></pre>



<a name="social_contracts_jury_EWrongCaseState"></a>



<pre><code><b>const</b> <a href="../social_contracts/jury.md#social_contracts_jury_EWrongCaseState">EWrongCaseState</a>: u64 = 3;
</code></pre>



<a name="social_contracts_jury_ENotYet"></a>



<pre><code><b>const</b> <a href="../social_contracts/jury.md#social_contracts_jury_ENotYet">ENotYet</a>: u64 = 4;
</code></pre>



<a name="social_contracts_jury_EInvalidConfig"></a>



<pre><code><b>const</b> <a href="../social_contracts/jury.md#social_contracts_jury_EInvalidConfig">EInvalidConfig</a>: u64 = 5;
</code></pre>



<a name="social_contracts_jury_EInvalidCase"></a>



<pre><code><b>const</b> <a href="../social_contracts/jury.md#social_contracts_jury_EInvalidCase">EInvalidCase</a>: u64 = 6;
</code></pre>



<a name="social_contracts_jury_EAlreadyOnRoster"></a>



<pre><code><b>const</b> <a href="../social_contracts/jury.md#social_contracts_jury_EAlreadyOnRoster">EAlreadyOnRoster</a>: u64 = 7;
</code></pre>



<a name="social_contracts_jury_ENotOnRoster"></a>



<pre><code><b>const</b> <a href="../social_contracts/jury.md#social_contracts_jury_ENotOnRoster">ENotOnRoster</a>: u64 = 8;
</code></pre>



<a name="social_contracts_jury_EIneligible"></a>



<pre><code><b>const</b> <a href="../social_contracts/jury.md#social_contracts_jury_EIneligible">EIneligible</a>: u64 = 9;
</code></pre>



<a name="social_contracts_jury_EWrongAgent"></a>



<pre><code><b>const</b> <a href="../social_contracts/jury.md#social_contracts_jury_EWrongAgent">EWrongAgent</a>: u64 = 10;
</code></pre>



<a name="social_contracts_jury_ENotJuror"></a>



<pre><code><b>const</b> <a href="../social_contracts/jury.md#social_contracts_jury_ENotJuror">ENotJuror</a>: u64 = 11;
</code></pre>



<a name="social_contracts_jury_ENotDrafted"></a>



<pre><code><b>const</b> <a href="../social_contracts/jury.md#social_contracts_jury_ENotDrafted">ENotDrafted</a>: u64 = 12;
</code></pre>



<a name="social_contracts_jury_EWrongSlotStatus"></a>



<pre><code><b>const</b> <a href="../social_contracts/jury.md#social_contracts_jury_EWrongSlotStatus">EWrongSlotStatus</a>: u64 = 13;
</code></pre>



<a name="social_contracts_jury_EWindowClosed"></a>



<pre><code><b>const</b> <a href="../social_contracts/jury.md#social_contracts_jury_EWindowClosed">EWindowClosed</a>: u64 = 14;
</code></pre>



<a name="social_contracts_jury_EInvalidDigest"></a>



<pre><code><b>const</b> <a href="../social_contracts/jury.md#social_contracts_jury_EInvalidDigest">EInvalidDigest</a>: u64 = 15;
</code></pre>



<a name="social_contracts_jury_EInvalidSalt"></a>



<pre><code><b>const</b> <a href="../social_contracts/jury.md#social_contracts_jury_EInvalidSalt">EInvalidSalt</a>: u64 = 16;
</code></pre>



<a name="social_contracts_jury_ECommitmentMismatch"></a>



<pre><code><b>const</b> <a href="../social_contracts/jury.md#social_contracts_jury_ECommitmentMismatch">ECommitmentMismatch</a>: u64 = 17;
</code></pre>



<a name="social_contracts_jury_EInvalidOutcome"></a>



<pre><code><b>const</b> <a href="../social_contracts/jury.md#social_contracts_jury_EInvalidOutcome">EInvalidOutcome</a>: u64 = 18;
</code></pre>



<a name="social_contracts_jury_EInvalidEvidenceRef"></a>



<pre><code><b>const</b> <a href="../social_contracts/jury.md#social_contracts_jury_EInvalidEvidenceRef">EInvalidEvidenceRef</a>: u64 = 19;
</code></pre>



<a name="social_contracts_jury_ERoundNotOver"></a>



<pre><code><b>const</b> <a href="../social_contracts/jury.md#social_contracts_jury_ERoundNotOver">ERoundNotOver</a>: u64 = 20;
</code></pre>



<a name="social_contracts_jury_ERosterTooSmall"></a>



<pre><code><b>const</b> <a href="../social_contracts/jury.md#social_contracts_jury_ERosterTooSmall">ERosterTooSmall</a>: u64 = 21;
</code></pre>



<a name="social_contracts_jury_EHasActiveCases"></a>



<pre><code><b>const</b> <a href="../social_contracts/jury.md#social_contracts_jury_EHasActiveCases">EHasActiveCases</a>: u64 = 22;
</code></pre>



<a name="social_contracts_jury_ETooMuchEvidence"></a>



<pre><code><b>const</b> <a href="../social_contracts/jury.md#social_contracts_jury_ETooMuchEvidence">ETooMuchEvidence</a>: u64 = 23;
</code></pre>



<a name="social_contracts_jury_EEvidenceClosed"></a>



<pre><code><b>const</b> <a href="../social_contracts/jury.md#social_contracts_jury_EEvidenceClosed">EEvidenceClosed</a>: u64 = 24;
</code></pre>



<a name="social_contracts_jury_EFeesNotSettled"></a>



<pre><code><b>const</b> <a href="../social_contracts/jury.md#social_contracts_jury_EFeesNotSettled">EFeesNotSettled</a>: u64 = 25;
</code></pre>



<a name="social_contracts_jury_EAlreadyClaimed"></a>



<pre><code><b>const</b> <a href="../social_contracts/jury.md#social_contracts_jury_EAlreadyClaimed">EAlreadyClaimed</a>: u64 = 26;
</code></pre>



<a name="social_contracts_jury_ENotAppealable"></a>



<pre><code><b>const</b> <a href="../social_contracts/jury.md#social_contracts_jury_ENotAppealable">ENotAppealable</a>: u64 = 27;
</code></pre>



<a name="social_contracts_jury_EAlreadyExecuted"></a>



<pre><code><b>const</b> <a href="../social_contracts/jury.md#social_contracts_jury_EAlreadyExecuted">EAlreadyExecuted</a>: u64 = 28;
</code></pre>



<a name="social_contracts_jury_ENotFinished"></a>



<pre><code><b>const</b> <a href="../social_contracts/jury.md#social_contracts_jury_ENotFinished">ENotFinished</a>: u64 = 29;
</code></pre>



<a name="social_contracts_jury_ETooManyExclusions"></a>



<pre><code><b>const</b> <a href="../social_contracts/jury.md#social_contracts_jury_ETooManyExclusions">ETooManyExclusions</a>: u64 = 30;
</code></pre>



<a name="social_contracts_jury_EInvalidUri"></a>



<pre><code><b>const</b> <a href="../social_contracts/jury.md#social_contracts_jury_EInvalidUri">EInvalidUri</a>: u64 = 31;
</code></pre>



<a name="social_contracts_jury_ENotOwner"></a>



<pre><code><b>const</b> <a href="../social_contracts/jury.md#social_contracts_jury_ENotOwner">ENotOwner</a>: u64 = 32;
</code></pre>



<a name="social_contracts_jury_ENotIneligible"></a>



<pre><code><b>const</b> <a href="../social_contracts/jury.md#social_contracts_jury_ENotIneligible">ENotIneligible</a>: u64 = 33;
</code></pre>



<a name="social_contracts_jury_bootstrap_init"></a>

## Function `bootstrap_init`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_bootstrap_init">bootstrap_init</a>(clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_bootstrap_init">bootstrap_init</a>(clock: &Clock, ctx: &<b>mut</b> TxContext) {
    <b>let</b> now = clock::timestamp_ms(clock);
    <b>let</b> config = <a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">JuryConfig</a> {
        id: object::new(ctx),
        version: <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(),
        active: <b>true</b>,
        eligibility: <a href="../social_contracts/jury.md#social_contracts_jury_EligibilityParams">EligibilityParams</a> {
            min_score_milli: 4000,
            min_confidence_bps: 2000,
            min_settled_count: 3,
            min_age_ms: 7 * <a href="../social_contracts/jury.md#social_contracts_jury_MS_DAY">MS_DAY</a>,
            max_snapshot_age_ms: 7 * <a href="../social_contracts/jury.md#social_contracts_jury_MS_DAY">MS_DAY</a>,
            max_active_cases: 3,
            <a href="../social_contracts/jury.md#social_contracts_jury_min_eligible_roster">min_eligible_roster</a>: 20,
        },
        timing: <a href="../social_contracts/jury.md#social_contracts_jury_TimingParams">TimingParams</a> {
            <a href="../social_contracts/jury.md#social_contracts_jury_accept_window_ms">accept_window_ms</a>: 12 * <a href="../social_contracts/jury.md#social_contracts_jury_MS_HOUR">MS_HOUR</a>,
            <a href="../social_contracts/jury.md#social_contracts_jury_commit_window_ms">commit_window_ms</a>: 24 * <a href="../social_contracts/jury.md#social_contracts_jury_MS_HOUR">MS_HOUR</a>,
            <a href="../social_contracts/jury.md#social_contracts_jury_reveal_window_ms">reveal_window_ms</a>: 12 * <a href="../social_contracts/jury.md#social_contracts_jury_MS_HOUR">MS_HOUR</a>,
            <a href="../social_contracts/jury.md#social_contracts_jury_reveal_grace_ms">reveal_grace_ms</a>: 6 * <a href="../social_contracts/jury.md#social_contracts_jury_MS_HOUR">MS_HOUR</a>,
            <a href="../social_contracts/jury.md#social_contracts_jury_appeal_window_ms">appeal_window_ms</a>: 48 * <a href="../social_contracts/jury.md#social_contracts_jury_MS_HOUR">MS_HOUR</a>,
            <a href="../social_contracts/jury.md#social_contracts_jury_neutral_window_ms">neutral_window_ms</a>: 72 * <a href="../social_contracts/jury.md#social_contracts_jury_MS_HOUR">MS_HOUR</a>,
            <a href="../social_contracts/jury.md#social_contracts_jury_max_case_duration_ms">max_case_duration_ms</a>: 30 * <a href="../social_contracts/jury.md#social_contracts_jury_MS_DAY">MS_DAY</a>,
        },
        duty: <a href="../social_contracts/jury.md#social_contracts_jury_DutyParams">DutyParams</a> {
            initial_score: 100,
            max_score: 200,
            min_score: 50,
            gain: 5,
            loss_accept: 10,
            loss_noreveal: 25,
            max_strikes: 3,
            suspension_ms: 14 * <a href="../social_contracts/jury.md#social_contracts_jury_MS_DAY">MS_DAY</a>,
        },
        selection: <a href="../social_contracts/jury.md#social_contracts_jury_SelectionParams">SelectionParams</a> {
            max_draws: 64,
            max_replacements: 3,
            max_evidence_per_party: 8,
            <a href="../social_contracts/jury.md#social_contracts_jury_max_exclusions">max_exclusions</a>: 64,
        },
        reputation: <a href="../social_contracts/memory.md#social_contracts_memory_new_reputation_params">memory::new_reputation_params</a>(
            3000, 50000, 1_000_000_000, 500, 20000, 2000, 500, 16, 10000, 5000, 5000,
        ),
        <a href="../social_contracts/jury.md#social_contracts_jury_genesis_sunset_ms">genesis_sunset_ms</a>: now + 180 * <a href="../social_contracts/jury.md#social_contracts_jury_MS_DAY">MS_DAY</a>,
        genesis_agents: vec_set::empty(),
    };
    <a href="../social_contracts/jury.md#social_contracts_jury_validate_config">validate_config</a>(&config);
    transfer::share_object(config);
    transfer::share_object(<a href="../social_contracts/jury.md#social_contracts_jury_JurorRoster">JurorRoster</a> {
        id: object::new(ctx),
        version: <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(),
        slots: table::new(ctx),
        index: table::new(ctx),
        entries: table::new(ctx),
        count: 0,
    });
}
</code></pre>



</details>

<a name="social_contracts_jury_create_jury_admin_cap"></a>

## Function `create_jury_admin_cap`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_create_jury_admin_cap">create_jury_admin_cap</a>(ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../social_contracts/jury.md#social_contracts_jury_JuryAdminCap">social_contracts::jury::JuryAdminCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_create_jury_admin_cap">create_jury_admin_cap</a>(ctx: &<b>mut</b> TxContext): <a href="../social_contracts/jury.md#social_contracts_jury_JuryAdminCap">JuryAdminCap</a> {
    <a href="../social_contracts/jury.md#social_contracts_jury_JuryAdminCap">JuryAdminCap</a> { id: object::new(ctx) }
}
</code></pre>



</details>

<a name="social_contracts_jury_validate_config"></a>

## Function `validate_config`



<pre><code><b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_validate_config">validate_config</a>(config: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">social_contracts::jury::JuryConfig</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_validate_config">validate_config</a>(config: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">JuryConfig</a>) {
    <b>let</b> e = &config.eligibility;
    <b>assert</b>!(e.min_confidence_bps &lt;= <a href="../social_contracts/jury.md#social_contracts_jury_BPS_DENOMINATOR">BPS_DENOMINATOR</a>, <a href="../social_contracts/jury.md#social_contracts_jury_EInvalidConfig">EInvalidConfig</a>);
    <b>assert</b>!(e.max_active_cases &gt; 0 && e.max_snapshot_age_ms &gt; 0, <a href="../social_contracts/jury.md#social_contracts_jury_EInvalidConfig">EInvalidConfig</a>);
    <b>let</b> t = &config.timing;
    <b>assert</b>!(
        t.<a href="../social_contracts/jury.md#social_contracts_jury_accept_window_ms">accept_window_ms</a> &gt; 0 && t.<a href="../social_contracts/jury.md#social_contracts_jury_commit_window_ms">commit_window_ms</a> &gt; 0 && t.<a href="../social_contracts/jury.md#social_contracts_jury_reveal_window_ms">reveal_window_ms</a> &gt; 0
            && t.<a href="../social_contracts/jury.md#social_contracts_jury_neutral_window_ms">neutral_window_ms</a> &gt; 0 && t.<a href="../social_contracts/jury.md#social_contracts_jury_max_case_duration_ms">max_case_duration_ms</a> &gt; 0,
        <a href="../social_contracts/jury.md#social_contracts_jury_EInvalidConfig">EInvalidConfig</a>,
    );
    <b>let</b> d = &config.duty;
    <b>assert</b>!(d.initial_score &lt;= d.max_score && d.min_score &lt;= d.initial_score, <a href="../social_contracts/jury.md#social_contracts_jury_EInvalidConfig">EInvalidConfig</a>);
    <b>assert</b>!(d.max_strikes &gt; 0 && d.gain &gt; 0, <a href="../social_contracts/jury.md#social_contracts_jury_EInvalidConfig">EInvalidConfig</a>);
    <b>let</b> s = &config.selection;
    <b>assert</b>!(s.max_draws &gt; 0 && s.max_evidence_per_party &gt; 0 && s.<a href="../social_contracts/jury.md#social_contracts_jury_max_exclusions">max_exclusions</a> &gt; 0, <a href="../social_contracts/jury.md#social_contracts_jury_EInvalidConfig">EInvalidConfig</a>);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_reputation_params">memory::assert_reputation_params</a>(&config.reputation);
}
</code></pre>



</details>

<a name="social_contracts_jury_assert_config_live"></a>

## Function `assert_config_live`



<pre><code><b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_assert_config_live">assert_config_live</a>(config: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">social_contracts::jury::JuryConfig</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_assert_config_live">assert_config_live</a>(config: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">JuryConfig</a>) {
    <b>assert</b>!(config.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/jury.md#social_contracts_jury_EWrongVersion">EWrongVersion</a>);
}
</code></pre>



</details>

<a name="social_contracts_jury_set_active"></a>

## Function `set_active`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_set_active">set_active</a>(_: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryAdminCap">social_contracts::jury::JuryAdminCap</a>, config: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">social_contracts::jury::JuryConfig</a>, active: bool, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_set_active">set_active</a>(_: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryAdminCap">JuryAdminCap</a>, config: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">JuryConfig</a>, active: bool, clock: &Clock, ctx: &TxContext) {
    <a href="../social_contracts/jury.md#social_contracts_jury_assert_config_live">assert_config_live</a>(config);
    config.active = active;
    <a href="../social_contracts/jury.md#social_contracts_jury_emit_config_updated">emit_config_updated</a>(0, clock, ctx);
}
</code></pre>



</details>

<a name="social_contracts_jury_set_eligibility"></a>

## Function `set_eligibility`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_set_eligibility">set_eligibility</a>(_: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryAdminCap">social_contracts::jury::JuryAdminCap</a>, config: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">social_contracts::jury::JuryConfig</a>, min_score_milli: u64, min_confidence_bps: u64, min_settled_count: u64, min_age_ms: u64, max_snapshot_age_ms: u64, max_active_cases: u64, <a href="../social_contracts/jury.md#social_contracts_jury_min_eligible_roster">min_eligible_roster</a>: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_set_eligibility">set_eligibility</a>(
    _: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryAdminCap">JuryAdminCap</a>,
    config: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">JuryConfig</a>,
    min_score_milli: u64,
    min_confidence_bps: u64,
    min_settled_count: u64,
    min_age_ms: u64,
    max_snapshot_age_ms: u64,
    max_active_cases: u64,
    <a href="../social_contracts/jury.md#social_contracts_jury_min_eligible_roster">min_eligible_roster</a>: u64,
    clock: &Clock,
    ctx: &TxContext,
) {
    <a href="../social_contracts/jury.md#social_contracts_jury_assert_config_live">assert_config_live</a>(config);
    config.eligibility = <a href="../social_contracts/jury.md#social_contracts_jury_EligibilityParams">EligibilityParams</a> {
        min_score_milli,
        min_confidence_bps,
        min_settled_count,
        min_age_ms,
        max_snapshot_age_ms,
        max_active_cases,
        <a href="../social_contracts/jury.md#social_contracts_jury_min_eligible_roster">min_eligible_roster</a>,
    };
    <a href="../social_contracts/jury.md#social_contracts_jury_validate_config">validate_config</a>(config);
    <a href="../social_contracts/jury.md#social_contracts_jury_emit_config_updated">emit_config_updated</a>(1, clock, ctx);
}
</code></pre>



</details>

<a name="social_contracts_jury_set_timing"></a>

## Function `set_timing`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_set_timing">set_timing</a>(_: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryAdminCap">social_contracts::jury::JuryAdminCap</a>, config: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">social_contracts::jury::JuryConfig</a>, <a href="../social_contracts/jury.md#social_contracts_jury_accept_window_ms">accept_window_ms</a>: u64, <a href="../social_contracts/jury.md#social_contracts_jury_commit_window_ms">commit_window_ms</a>: u64, <a href="../social_contracts/jury.md#social_contracts_jury_reveal_window_ms">reveal_window_ms</a>: u64, <a href="../social_contracts/jury.md#social_contracts_jury_reveal_grace_ms">reveal_grace_ms</a>: u64, <a href="../social_contracts/jury.md#social_contracts_jury_appeal_window_ms">appeal_window_ms</a>: u64, <a href="../social_contracts/jury.md#social_contracts_jury_neutral_window_ms">neutral_window_ms</a>: u64, <a href="../social_contracts/jury.md#social_contracts_jury_max_case_duration_ms">max_case_duration_ms</a>: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_set_timing">set_timing</a>(
    _: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryAdminCap">JuryAdminCap</a>,
    config: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">JuryConfig</a>,
    <a href="../social_contracts/jury.md#social_contracts_jury_accept_window_ms">accept_window_ms</a>: u64,
    <a href="../social_contracts/jury.md#social_contracts_jury_commit_window_ms">commit_window_ms</a>: u64,
    <a href="../social_contracts/jury.md#social_contracts_jury_reveal_window_ms">reveal_window_ms</a>: u64,
    <a href="../social_contracts/jury.md#social_contracts_jury_reveal_grace_ms">reveal_grace_ms</a>: u64,
    <a href="../social_contracts/jury.md#social_contracts_jury_appeal_window_ms">appeal_window_ms</a>: u64,
    <a href="../social_contracts/jury.md#social_contracts_jury_neutral_window_ms">neutral_window_ms</a>: u64,
    <a href="../social_contracts/jury.md#social_contracts_jury_max_case_duration_ms">max_case_duration_ms</a>: u64,
    clock: &Clock,
    ctx: &TxContext,
) {
    <a href="../social_contracts/jury.md#social_contracts_jury_assert_config_live">assert_config_live</a>(config);
    config.timing = <a href="../social_contracts/jury.md#social_contracts_jury_TimingParams">TimingParams</a> {
        <a href="../social_contracts/jury.md#social_contracts_jury_accept_window_ms">accept_window_ms</a>,
        <a href="../social_contracts/jury.md#social_contracts_jury_commit_window_ms">commit_window_ms</a>,
        <a href="../social_contracts/jury.md#social_contracts_jury_reveal_window_ms">reveal_window_ms</a>,
        <a href="../social_contracts/jury.md#social_contracts_jury_reveal_grace_ms">reveal_grace_ms</a>,
        <a href="../social_contracts/jury.md#social_contracts_jury_appeal_window_ms">appeal_window_ms</a>,
        <a href="../social_contracts/jury.md#social_contracts_jury_neutral_window_ms">neutral_window_ms</a>,
        <a href="../social_contracts/jury.md#social_contracts_jury_max_case_duration_ms">max_case_duration_ms</a>,
    };
    <a href="../social_contracts/jury.md#social_contracts_jury_validate_config">validate_config</a>(config);
    <a href="../social_contracts/jury.md#social_contracts_jury_emit_config_updated">emit_config_updated</a>(2, clock, ctx);
}
</code></pre>



</details>

<a name="social_contracts_jury_set_duty"></a>

## Function `set_duty`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_set_duty">set_duty</a>(_: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryAdminCap">social_contracts::jury::JuryAdminCap</a>, config: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">social_contracts::jury::JuryConfig</a>, initial_score: u64, max_score: u64, min_score: u64, gain: u64, loss_accept: u64, loss_noreveal: u64, max_strikes: u64, suspension_ms: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_set_duty">set_duty</a>(
    _: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryAdminCap">JuryAdminCap</a>,
    config: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">JuryConfig</a>,
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
    <a href="../social_contracts/jury.md#social_contracts_jury_assert_config_live">assert_config_live</a>(config);
    config.duty = <a href="../social_contracts/jury.md#social_contracts_jury_DutyParams">DutyParams</a> {
        initial_score,
        max_score,
        min_score,
        gain,
        loss_accept,
        loss_noreveal,
        max_strikes,
        suspension_ms,
    };
    <a href="../social_contracts/jury.md#social_contracts_jury_validate_config">validate_config</a>(config);
    <a href="../social_contracts/jury.md#social_contracts_jury_emit_config_updated">emit_config_updated</a>(3, clock, ctx);
}
</code></pre>



</details>

<a name="social_contracts_jury_set_selection"></a>

## Function `set_selection`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_set_selection">set_selection</a>(_: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryAdminCap">social_contracts::jury::JuryAdminCap</a>, config: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">social_contracts::jury::JuryConfig</a>, max_draws: u64, max_replacements: u64, max_evidence_per_party: u64, <a href="../social_contracts/jury.md#social_contracts_jury_max_exclusions">max_exclusions</a>: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_set_selection">set_selection</a>(
    _: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryAdminCap">JuryAdminCap</a>,
    config: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">JuryConfig</a>,
    max_draws: u64,
    max_replacements: u64,
    max_evidence_per_party: u64,
    <a href="../social_contracts/jury.md#social_contracts_jury_max_exclusions">max_exclusions</a>: u64,
    clock: &Clock,
    ctx: &TxContext,
) {
    <a href="../social_contracts/jury.md#social_contracts_jury_assert_config_live">assert_config_live</a>(config);
    config.selection = <a href="../social_contracts/jury.md#social_contracts_jury_SelectionParams">SelectionParams</a> { max_draws, max_replacements, max_evidence_per_party, <a href="../social_contracts/jury.md#social_contracts_jury_max_exclusions">max_exclusions</a> };
    <a href="../social_contracts/jury.md#social_contracts_jury_validate_config">validate_config</a>(config);
    <a href="../social_contracts/jury.md#social_contracts_jury_emit_config_updated">emit_config_updated</a>(4, clock, ctx);
}
</code></pre>



</details>

<a name="social_contracts_jury_set_reputation_params"></a>

## Function `set_reputation_params`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_set_reputation_params">set_reputation_params</a>(_: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryAdminCap">social_contracts::jury::JuryAdminCap</a>, config: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">social_contracts::jury::JuryConfig</a>, prior_mean_milli: u64, prior_weight_bps: u64, value_ref_mist: u64, min_value_weight_bps: u64, max_value_weight_bps: u64, cred_base_bps: u64, cred_step_bps: u64, cred_cap_n: u64, cred_max_bps: u64, overall_seller_weight_bps: u64, overall_buyer_weight_bps: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_set_reputation_params">set_reputation_params</a>(
    _: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryAdminCap">JuryAdminCap</a>,
    config: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">JuryConfig</a>,
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
    <a href="../social_contracts/jury.md#social_contracts_jury_assert_config_live">assert_config_live</a>(config);
    config.reputation = <a href="../social_contracts/memory.md#social_contracts_memory_new_reputation_params">memory::new_reputation_params</a>(
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
    <a href="../social_contracts/jury.md#social_contracts_jury_validate_config">validate_config</a>(config);
    <a href="../social_contracts/jury.md#social_contracts_jury_emit_config_updated">emit_config_updated</a>(5, clock, ctx);
}
</code></pre>



</details>

<a name="social_contracts_jury_set_genesis_sunset"></a>

## Function `set_genesis_sunset`

Agents in the allow-list skip the reputation bar until <code>sunset_ms</code>, so a roster can exist
before anyone has organic reputation. They still need the capability, an active agent and
every conflict rule.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_set_genesis_sunset">set_genesis_sunset</a>(_: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryAdminCap">social_contracts::jury::JuryAdminCap</a>, config: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">social_contracts::jury::JuryConfig</a>, sunset_ms: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_set_genesis_sunset">set_genesis_sunset</a>(_: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryAdminCap">JuryAdminCap</a>, config: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">JuryConfig</a>, sunset_ms: u64, clock: &Clock, ctx: &TxContext) {
    <a href="../social_contracts/jury.md#social_contracts_jury_assert_config_live">assert_config_live</a>(config);
    config.<a href="../social_contracts/jury.md#social_contracts_jury_genesis_sunset_ms">genesis_sunset_ms</a> = sunset_ms;
    <a href="../social_contracts/jury.md#social_contracts_jury_emit_config_updated">emit_config_updated</a>(6, clock, ctx);
}
</code></pre>



</details>

<a name="social_contracts_jury_add_genesis_agent"></a>

## Function `add_genesis_agent`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_add_genesis_agent">add_genesis_agent</a>(_: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryAdminCap">social_contracts::jury::JuryAdminCap</a>, config: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">social_contracts::jury::JuryConfig</a>, agent_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_add_genesis_agent">add_genesis_agent</a>(_: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryAdminCap">JuryAdminCap</a>, config: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">JuryConfig</a>, agent_id: ID, clock: &Clock, ctx: &TxContext) {
    <a href="../social_contracts/jury.md#social_contracts_jury_assert_config_live">assert_config_live</a>(config);
    <b>if</b> (!vec_set::contains(&config.genesis_agents, &agent_id)) {
        vec_set::insert(&<b>mut</b> config.genesis_agents, agent_id);
    };
    <a href="../social_contracts/jury.md#social_contracts_jury_emit_config_updated">emit_config_updated</a>(7, clock, ctx);
}
</code></pre>



</details>

<a name="social_contracts_jury_remove_genesis_agent"></a>

## Function `remove_genesis_agent`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_remove_genesis_agent">remove_genesis_agent</a>(_: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryAdminCap">social_contracts::jury::JuryAdminCap</a>, config: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">social_contracts::jury::JuryConfig</a>, agent_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_remove_genesis_agent">remove_genesis_agent</a>(_: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryAdminCap">JuryAdminCap</a>, config: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">JuryConfig</a>, agent_id: ID, clock: &Clock, ctx: &TxContext) {
    <a href="../social_contracts/jury.md#social_contracts_jury_assert_config_live">assert_config_live</a>(config);
    <b>if</b> (vec_set::contains(&config.genesis_agents, &agent_id)) {
        vec_set::remove(&<b>mut</b> config.genesis_agents, &agent_id);
    };
    <a href="../social_contracts/jury.md#social_contracts_jury_emit_config_updated">emit_config_updated</a>(7, clock, ctx);
}
</code></pre>



</details>

<a name="social_contracts_jury_emit_config_updated"></a>

## Function `emit_config_updated`



<pre><code><b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_emit_config_updated">emit_config_updated</a>(section: u8, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_emit_config_updated">emit_config_updated</a>(section: u8, clock: &Clock, ctx: &TxContext) {
    event::emit(<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfigUpdated">JuryConfigUpdated</a> {
        updated_by: tx_context::sender(ctx),
        section,
        timestamp_ms: clock::timestamp_ms(clock),
    });
}
</code></pre>



</details>

<a name="social_contracts_jury_migrate_jury_config"></a>

## Function `migrate_jury_config`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_migrate_jury_config">migrate_jury_config</a>(config: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">social_contracts::jury::JuryConfig</a>, _: &<a href="../social_contracts/upgrade.md#social_contracts_upgrade_UpgradeAdminCap">social_contracts::upgrade::UpgradeAdminCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_migrate_jury_config">migrate_jury_config</a>(config: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">JuryConfig</a>, _: &UpgradeAdminCap) {
    <b>assert</b>!(config.version &lt; <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/jury.md#social_contracts_jury_EWrongVersion">EWrongVersion</a>);
    config.version = <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>();
}
</code></pre>



</details>

<a name="social_contracts_jury_migrate_jury_roster"></a>

## Function `migrate_jury_roster`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_migrate_jury_roster">migrate_jury_roster</a>(roster: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JurorRoster">social_contracts::jury::JurorRoster</a>, _: &<a href="../social_contracts/upgrade.md#social_contracts_upgrade_UpgradeAdminCap">social_contracts::upgrade::UpgradeAdminCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_migrate_jury_roster">migrate_jury_roster</a>(roster: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JurorRoster">JurorRoster</a>, _: &UpgradeAdminCap) {
    <b>assert</b>!(roster.version &lt; <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/jury.md#social_contracts_jury_EWrongVersion">EWrongVersion</a>);
    roster.version = <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>();
}
</code></pre>



</details>

<a name="social_contracts_jury_migrate_jury_case"></a>

## Function `migrate_jury_case`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_migrate_jury_case">migrate_jury_case</a>(case: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">social_contracts::jury::JuryCase</a>, _: &<a href="../social_contracts/upgrade.md#social_contracts_upgrade_UpgradeAdminCap">social_contracts::upgrade::UpgradeAdminCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_migrate_jury_case">migrate_jury_case</a>(case: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">JuryCase</a>, _: &UpgradeAdminCap) {
    <b>assert</b>!(case.version &lt; <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/jury.md#social_contracts_jury_EWrongVersion">EWrongVersion</a>);
    case.version = <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>();
}
</code></pre>



</details>

<a name="social_contracts_jury_join_roster"></a>

## Function `join_roster`

Joins the roster for free. The owner signs; the agent must carry the jury capability and be
eligible now (or be an allow-listed genesis agent before the sunset).


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_join_roster">join_roster</a>(config: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">social_contracts::jury::JuryConfig</a>, roster: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JurorRoster">social_contracts::jury::JurorRoster</a>, memory_config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>, payout_address: <b>address</b>, model_family: u8, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_join_roster">join_roster</a>(
    config: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">JuryConfig</a>,
    roster: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JurorRoster">JurorRoster</a>,
    memory_config: &MemoryConfig,
    account: &MemoryAccount,
    agent: &SubAgent,
    payout_address: <b>address</b>,
    model_family: u8,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <a href="../social_contracts/jury.md#social_contracts_jury_assert_config_live">assert_config_live</a>(config);
    <b>assert</b>!(roster.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/jury.md#social_contracts_jury_EWrongVersion">EWrongVersion</a>);
    <b>assert</b>!(config.active, <a href="../social_contracts/jury.md#social_contracts_jury_EJuryPaused">EJuryPaused</a>);
    <a href="../social_contracts/memory.md#social_contracts_memory_resolve_human_actor">memory::resolve_human_actor</a>(account, ctx);
    <b>assert</b>!(<a href="../social_contracts/memory.md#social_contracts_memory_sub_agent_memory_account_id">memory::sub_agent_memory_account_id</a>(agent) == object::id(account), <a href="../social_contracts/jury.md#social_contracts_jury_EWrongAgent">EWrongAgent</a>);
    <b>let</b> agent_id = object::id(agent);
    <b>assert</b>!(!table::contains(&roster.entries, agent_id), <a href="../social_contracts/jury.md#social_contracts_jury_EAlreadyOnRoster">EAlreadyOnRoster</a>);
    <b>let</b> now = clock::timestamp_ms(clock);
    <b>let</b> snap = <a href="../social_contracts/jury.md#social_contracts_jury_take_snapshot">take_snapshot</a>(config, memory_config, account, agent);
    <b>assert</b>!(snap.ok, <a href="../social_contracts/jury.md#social_contracts_jury_EIneligible">EIneligible</a>);
    <b>let</b> exempt = <a href="../social_contracts/jury.md#social_contracts_jury_is_genesis_exempt">is_genesis_exempt</a>(config, agent_id, now);
    <b>assert</b>!(
        exempt || <a href="../social_contracts/jury.md#social_contracts_jury_meets_reputation">meets_reputation</a>(config, snap.score_milli, snap.confidence_bps, snap.settled_count, snap.created_at_ms, now),
        <a href="../social_contracts/jury.md#social_contracts_jury_EIneligible">EIneligible</a>,
    );
    <b>let</b> <b>entry</b> = <a href="../social_contracts/jury.md#social_contracts_jury_RosterEntry">RosterEntry</a> {
        agent_id,
        owner: <a href="../social_contracts/memory.md#social_contracts_memory_owner">memory::owner</a>(account),
        memory_account_id: object::id(account),
        organization_id: <a href="../social_contracts/memory.md#social_contracts_memory_sub_agent_organization_id">memory::sub_agent_organization_id</a>(agent),
        ancestor_ids: snap.ancestors,
        payout_address,
        model_family,
        status: <a href="../social_contracts/jury.md#social_contracts_jury_ROSTER_ACTIVE">ROSTER_ACTIVE</a>,
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
    event::emit(<a href="../social_contracts/jury.md#social_contracts_jury_RosterJoined">RosterJoined</a> {
        agent_id,
        owner: <b>entry</b>.owner,
        memory_account_id: <b>entry</b>.memory_account_id,
        organization_id: <b>entry</b>.organization_id,
        payout_address,
        model_family,
        exempt,
        joined_at_ms: now,
    });
    <a href="../social_contracts/jury.md#social_contracts_jury_add_to_roster">add_to_roster</a>(roster, agent_id, <b>entry</b>);
}
</code></pre>



</details>

<a name="social_contracts_jury_leave_roster"></a>

## Function `leave_roster`

Leaves the roster. Refused while the agent sits on an open case, so a seat is never abandoned.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_leave_roster">leave_roster</a>(roster: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JurorRoster">social_contracts::jury::JurorRoster</a>, account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, agent_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_leave_roster">leave_roster</a>(
    roster: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JurorRoster">JurorRoster</a>,
    account: &MemoryAccount,
    agent_id: ID,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(roster.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/jury.md#social_contracts_jury_EWrongVersion">EWrongVersion</a>);
    <a href="../social_contracts/memory.md#social_contracts_memory_resolve_human_actor">memory::resolve_human_actor</a>(account, ctx);
    <b>assert</b>!(table::contains(&roster.entries, agent_id), <a href="../social_contracts/jury.md#social_contracts_jury_ENotOnRoster">ENotOnRoster</a>);
    {
        <b>let</b> <b>entry</b> = table::borrow(&roster.entries, agent_id);
        <b>assert</b>!(<b>entry</b>.owner == <a href="../social_contracts/memory.md#social_contracts_memory_owner">memory::owner</a>(account), <a href="../social_contracts/jury.md#social_contracts_jury_ENotOwner">ENotOwner</a>);
        <b>assert</b>!(<b>entry</b>.active_cases == 0, <a href="../social_contracts/jury.md#social_contracts_jury_EHasActiveCases">EHasActiveCases</a>);
    };
    <a href="../social_contracts/jury.md#social_contracts_jury_remove_from_roster">remove_from_roster</a>(roster, agent_id);
    event::emit(<a href="../social_contracts/jury.md#social_contracts_jury_RosterLeft">RosterLeft</a> { agent_id, reason: 1, left_at_ms: clock::timestamp_ms(clock) });
}
</code></pre>



</details>

<a name="social_contracts_jury_set_availability"></a>

## Function `set_availability`

The owner pauses or resumes being drafted. A suspension cannot be lifted this way.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_set_availability">set_availability</a>(roster: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JurorRoster">social_contracts::jury::JurorRoster</a>, account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, agent_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, available: bool, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_set_availability">set_availability</a>(
    roster: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JurorRoster">JurorRoster</a>,
    account: &MemoryAccount,
    agent_id: ID,
    available: bool,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(roster.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/jury.md#social_contracts_jury_EWrongVersion">EWrongVersion</a>);
    <a href="../social_contracts/memory.md#social_contracts_memory_resolve_human_actor">memory::resolve_human_actor</a>(account, ctx);
    <b>assert</b>!(table::contains(&roster.entries, agent_id), <a href="../social_contracts/jury.md#social_contracts_jury_ENotOnRoster">ENotOnRoster</a>);
    <b>let</b> <b>entry</b> = table::borrow_mut(&<b>mut</b> roster.entries, agent_id);
    <b>assert</b>!(<b>entry</b>.owner == <a href="../social_contracts/memory.md#social_contracts_memory_owner">memory::owner</a>(account), <a href="../social_contracts/jury.md#social_contracts_jury_ENotOwner">ENotOwner</a>);
    <b>if</b> (<b>entry</b>.status == <a href="../social_contracts/jury.md#social_contracts_jury_ROSTER_SUSPENDED">ROSTER_SUSPENDED</a>) {
        <b>return</b>
    };
    <b>entry</b>.status = <b>if</b> (available) { <a href="../social_contracts/jury.md#social_contracts_jury_ROSTER_ACTIVE">ROSTER_ACTIVE</a> } <b>else</b> { <a href="../social_contracts/jury.md#social_contracts_jury_ROSTER_PAUSED">ROSTER_PAUSED</a> };
}
</code></pre>



</details>

<a name="social_contracts_jury_refresh_roster_entry"></a>

## Function `refresh_roster_entry`

Anyone passes the live objects to refresh the eligibility snapshot the draw filters on.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_refresh_roster_entry">refresh_roster_entry</a>(config: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">social_contracts::jury::JuryConfig</a>, roster: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JurorRoster">social_contracts::jury::JurorRoster</a>, memory_config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_refresh_roster_entry">refresh_roster_entry</a>(
    config: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">JuryConfig</a>,
    roster: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JurorRoster">JurorRoster</a>,
    memory_config: &MemoryConfig,
    account: &MemoryAccount,
    agent: &SubAgent,
    clock: &Clock,
) {
    <a href="../social_contracts/jury.md#social_contracts_jury_assert_config_live">assert_config_live</a>(config);
    <b>assert</b>!(roster.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/jury.md#social_contracts_jury_EWrongVersion">EWrongVersion</a>);
    <b>let</b> agent_id = object::id(agent);
    <b>assert</b>!(table::contains(&roster.entries, agent_id), <a href="../social_contracts/jury.md#social_contracts_jury_ENotOnRoster">ENotOnRoster</a>);
    <b>assert</b>!(<a href="../social_contracts/memory.md#social_contracts_memory_sub_agent_memory_account_id">memory::sub_agent_memory_account_id</a>(agent) == object::id(account), <a href="../social_contracts/jury.md#social_contracts_jury_EWrongAgent">EWrongAgent</a>);
    <b>let</b> now = clock::timestamp_ms(clock);
    <b>let</b> snap = <a href="../social_contracts/jury.md#social_contracts_jury_take_snapshot">take_snapshot</a>(config, memory_config, account, agent);
    <b>let</b> exempt = <a href="../social_contracts/jury.md#social_contracts_jury_is_genesis_exempt">is_genesis_exempt</a>(config, agent_id, now);
    <b>let</b> <b>entry</b> = table::borrow_mut(&<b>mut</b> roster.entries, agent_id);
    <b>entry</b>.snapshot_at_ms = now;
    <b>entry</b>.snapshot_ok = snap.ok;
    <b>entry</b>.snapshot_score_milli = snap.score_milli;
    <b>entry</b>.snapshot_confidence_bps = snap.confidence_bps;
    <b>entry</b>.snapshot_settled_count = snap.settled_count;
    <b>entry</b>.snapshot_created_at_ms = snap.created_at_ms;
    <b>entry</b>.snapshot_expires_at = snap.expires_at;
    <b>entry</b>.ancestor_ids = snap.ancestors;
    <b>entry</b>.exempt = exempt;
}
</code></pre>



</details>

<a name="social_contracts_jury_evict_ineligible"></a>

## Function `evict_ineligible`

Removes an entry whose snapshot says the agent can no longer serve (inactive, expired, lost
the capability or class). Needs a fresh snapshot, so a stale entry is refreshed first.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_evict_ineligible">evict_ineligible</a>(config: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">social_contracts::jury::JuryConfig</a>, roster: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JurorRoster">social_contracts::jury::JurorRoster</a>, agent_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_evict_ineligible">evict_ineligible</a>(
    config: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">JuryConfig</a>,
    roster: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JurorRoster">JurorRoster</a>,
    agent_id: ID,
    clock: &Clock,
) {
    <a href="../social_contracts/jury.md#social_contracts_jury_assert_config_live">assert_config_live</a>(config);
    <b>assert</b>!(roster.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/jury.md#social_contracts_jury_EWrongVersion">EWrongVersion</a>);
    <b>assert</b>!(table::contains(&roster.entries, agent_id), <a href="../social_contracts/jury.md#social_contracts_jury_ENotOnRoster">ENotOnRoster</a>);
    <b>let</b> now = clock::timestamp_ms(clock);
    {
        <b>let</b> <b>entry</b> = table::borrow(&roster.entries, agent_id);
        <b>assert</b>!(<b>entry</b>.active_cases == 0, <a href="../social_contracts/jury.md#social_contracts_jury_EHasActiveCases">EHasActiveCases</a>);
        <b>let</b> expired = option::is_some(&<b>entry</b>.snapshot_expires_at)
            && *option::borrow(&<b>entry</b>.snapshot_expires_at) &lt;= now;
        <b>assert</b>!(!<b>entry</b>.snapshot_ok || expired, <a href="../social_contracts/jury.md#social_contracts_jury_ENotIneligible">ENotIneligible</a>);
    };
    <a href="../social_contracts/jury.md#social_contracts_jury_remove_from_roster">remove_from_roster</a>(roster, agent_id);
    event::emit(<a href="../social_contracts/jury.md#social_contracts_jury_RosterLeft">RosterLeft</a> { agent_id, reason: 2, left_at_ms: now });
}
</code></pre>



</details>

<a name="social_contracts_jury_add_to_roster"></a>

## Function `add_to_roster`



<pre><code><b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_add_to_roster">add_to_roster</a>(roster: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JurorRoster">social_contracts::jury::JurorRoster</a>, agent_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, <b>entry</b>: <a href="../social_contracts/jury.md#social_contracts_jury_RosterEntry">social_contracts::jury::RosterEntry</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_add_to_roster">add_to_roster</a>(roster: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JurorRoster">JurorRoster</a>, agent_id: ID, <b>entry</b>: <a href="../social_contracts/jury.md#social_contracts_jury_RosterEntry">RosterEntry</a>) {
    <b>let</b> position = roster.count;
    table::add(&<b>mut</b> roster.entries, agent_id, <b>entry</b>);
    table::add(&<b>mut</b> roster.slots, position, agent_id);
    table::add(&<b>mut</b> roster.index, agent_id, position);
    roster.count = position + 1;
}
</code></pre>



</details>

<a name="social_contracts_jury_remove_from_roster"></a>

## Function `remove_from_roster`

Swap-remove keeps the slot index dense.


<pre><code><b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_remove_from_roster">remove_from_roster</a>(roster: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JurorRoster">social_contracts::jury::JurorRoster</a>, agent_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_remove_from_roster">remove_from_roster</a>(roster: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JurorRoster">JurorRoster</a>, agent_id: ID) {
    <b>let</b> position = table::remove(&<b>mut</b> roster.index, agent_id);
    <b>let</b> last = roster.count - 1;
    <b>if</b> (position != last) {
        <b>let</b> last_id = *table::borrow(&roster.slots, last);
        *table::borrow_mut(&<b>mut</b> roster.slots, position) = last_id;
        *table::borrow_mut(&<b>mut</b> roster.index, last_id) = position;
    };
    table::remove(&<b>mut</b> roster.slots, last);
    roster.count = last;
    table::remove(&<b>mut</b> roster.entries, agent_id);
}
</code></pre>



</details>

<a name="social_contracts_jury_take_snapshot"></a>

## Function `take_snapshot`



<pre><code><b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_take_snapshot">take_snapshot</a>(config: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">social_contracts::jury::JuryConfig</a>, memory_config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>): <a href="../social_contracts/jury.md#social_contracts_jury_Snapshot">social_contracts::jury::Snapshot</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_take_snapshot">take_snapshot</a>(
    config: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">JuryConfig</a>,
    memory_config: &MemoryConfig,
    account: &MemoryAccount,
    agent: &SubAgent,
): <a href="../social_contracts/jury.md#social_contracts_jury_Snapshot">Snapshot</a> {
    <b>let</b> caps = <a href="../social_contracts/memory.md#social_contracts_memory_sub_agent_capabilities">memory::sub_agent_capabilities</a>(agent);
    <b>let</b> ok = <a href="../social_contracts/memory.md#social_contracts_memory_sub_agent_active">memory::sub_agent_active</a>(agent)
        && <a href="../social_contracts/memory.md#social_contracts_memory_is_active">memory::is_active</a>(account)
        && <a href="../social_contracts/memory.md#social_contracts_memory_sub_agent_identity_class">memory::sub_agent_identity_class</a>(agent) == <a href="../social_contracts/memory.md#social_contracts_memory_class_delegated_ai">memory::class_delegated_ai</a>()
        && (caps & <a href="../social_contracts/memory.md#social_contracts_memory_cap_jury">memory::cap_jury</a>()) == <a href="../social_contracts/memory.md#social_contracts_memory_cap_jury">memory::cap_jury</a>();
    <b>let</b> (score, confidence) = <a href="../social_contracts/memory.md#social_contracts_memory_agent_overall">memory::agent_overall</a>(agent, &config.reputation);
    <a href="../social_contracts/jury.md#social_contracts_jury_Snapshot">Snapshot</a> {
        ok,
        score_milli: score,
        confidence_bps: confidence,
        settled_count: <a href="../social_contracts/memory.md#social_contracts_memory_account_settled_count">memory::account_settled_count</a>(account),
        created_at_ms: <a href="../social_contracts/memory.md#social_contracts_memory_sub_agent_created_at">memory::sub_agent_created_at</a>(agent),
        expires_at: <a href="../social_contracts/memory.md#social_contracts_memory_sub_agent_expires_at">memory::sub_agent_expires_at</a>(agent),
        ancestors: <a href="../social_contracts/memory.md#social_contracts_memory_agent_ancestor_ids">memory::agent_ancestor_ids</a>(memory_config, account, object::id(agent)),
    }
}
</code></pre>



</details>

<a name="social_contracts_jury_is_genesis_exempt"></a>

## Function `is_genesis_exempt`



<pre><code><b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_is_genesis_exempt">is_genesis_exempt</a>(config: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">social_contracts::jury::JuryConfig</a>, agent_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, now: u64): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_is_genesis_exempt">is_genesis_exempt</a>(config: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">JuryConfig</a>, agent_id: ID, now: u64): bool {
    vec_set::contains(&config.genesis_agents, &agent_id) && now &lt;= config.<a href="../social_contracts/jury.md#social_contracts_jury_genesis_sunset_ms">genesis_sunset_ms</a>
}
</code></pre>



</details>

<a name="social_contracts_jury_meets_reputation"></a>

## Function `meets_reputation`



<pre><code><b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_meets_reputation">meets_reputation</a>(config: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">social_contracts::jury::JuryConfig</a>, score_milli: u64, confidence_bps: u64, settled_count: u64, created_at_ms: u64, now: u64): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_meets_reputation">meets_reputation</a>(
    config: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">JuryConfig</a>,
    score_milli: u64,
    confidence_bps: u64,
    settled_count: u64,
    created_at_ms: u64,
    now: u64,
): bool {
    <b>let</b> e = &config.eligibility;
    score_milli &gt;= e.min_score_milli
        && confidence_bps &gt;= e.min_confidence_bps
        && settled_count &gt;= e.min_settled_count
        && now &gt;= created_at_ms
        && now - created_at_ms &gt;= e.min_age_ms
}
</code></pre>



</details>

<a name="social_contracts_jury_draw_eligible"></a>

## Function `draw_eligible`

The coarse filter the draw applies from the snapshot; <code><a href="../social_contracts/jury.md#social_contracts_jury_accept_assignment">accept_assignment</a></code> is the live check.


<pre><code><b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_draw_eligible">draw_eligible</a>(config: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">social_contracts::jury::JuryConfig</a>, <b>entry</b>: &<a href="../social_contracts/jury.md#social_contracts_jury_RosterEntry">social_contracts::jury::RosterEntry</a>, now: u64): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_draw_eligible">draw_eligible</a>(config: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">JuryConfig</a>, <b>entry</b>: &<a href="../social_contracts/jury.md#social_contracts_jury_RosterEntry">RosterEntry</a>, now: u64): bool {
    <b>if</b> (<b>entry</b>.status == <a href="../social_contracts/jury.md#social_contracts_jury_ROSTER_PAUSED">ROSTER_PAUSED</a>) {
        <b>return</b> <b>false</b>
    };
    <b>if</b> (<b>entry</b>.status == <a href="../social_contracts/jury.md#social_contracts_jury_ROSTER_SUSPENDED">ROSTER_SUSPENDED</a> && now &lt; <b>entry</b>.suspended_until_ms) {
        <b>return</b> <b>false</b>
    };
    <b>if</b> (!<b>entry</b>.snapshot_ok) {
        <b>return</b> <b>false</b>
    };
    <b>if</b> (now &gt; <b>entry</b>.snapshot_at_ms && now - <b>entry</b>.snapshot_at_ms &gt; config.eligibility.max_snapshot_age_ms) {
        <b>return</b> <b>false</b>
    };
    <b>if</b> (option::is_some(&<b>entry</b>.snapshot_expires_at) && *option::borrow(&<b>entry</b>.snapshot_expires_at) &lt;= now) {
        <b>return</b> <b>false</b>
    };
    <b>if</b> (<b>entry</b>.active_cases &gt;= config.eligibility.max_active_cases) {
        <b>return</b> <b>false</b>
    };
    <b>if</b> (<b>entry</b>.duty_score &lt; config.duty.min_score) {
        <b>return</b> <b>false</b>
    };
    <b>if</b> (<b>entry</b>.exempt && now &lt;= config.<a href="../social_contracts/jury.md#social_contracts_jury_genesis_sunset_ms">genesis_sunset_ms</a>) {
        <b>return</b> <b>true</b>
    };
    <a href="../social_contracts/jury.md#social_contracts_jury_meets_reputation">meets_reputation</a>(
        config,
        <b>entry</b>.snapshot_score_milli,
        <b>entry</b>.snapshot_confidence_bps,
        <b>entry</b>.snapshot_settled_count,
        <b>entry</b>.snapshot_created_at_ms,
        now,
    )
}
</code></pre>



</details>

<a name="social_contracts_jury_eligible_count"></a>

## Function `eligible_count`

Entries a draw could pick right now. An integrator refuses to open a case below the minimum,
so a thin roster fails closed instead of being gamed.


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_eligible_count">eligible_count</a>(config: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">social_contracts::jury::JuryConfig</a>, roster: &<a href="../social_contracts/jury.md#social_contracts_jury_JurorRoster">social_contracts::jury::JurorRoster</a>, now: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_eligible_count">eligible_count</a>(config: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">JuryConfig</a>, roster: &<a href="../social_contracts/jury.md#social_contracts_jury_JurorRoster">JurorRoster</a>, now: u64): u64 {
    <b>let</b> <b>mut</b> count = 0;
    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (i &lt; roster.count) {
        <b>let</b> agent_id = *table::borrow(&roster.slots, i);
        <b>if</b> (<a href="../social_contracts/jury.md#social_contracts_jury_draw_eligible">draw_eligible</a>(config, table::borrow(&roster.entries, agent_id), now)) {
            count = count + 1;
        };
        i = i + 1;
    };
    count
}
</code></pre>



</details>

<a name="social_contracts_jury_assert_roster_ready"></a>

## Function `assert_roster_ready`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_assert_roster_ready">assert_roster_ready</a>(config: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">social_contracts::jury::JuryConfig</a>, roster: &<a href="../social_contracts/jury.md#social_contracts_jury_JurorRoster">social_contracts::jury::JurorRoster</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_assert_roster_ready">assert_roster_ready</a>(config: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">JuryConfig</a>, roster: &<a href="../social_contracts/jury.md#social_contracts_jury_JurorRoster">JurorRoster</a>, clock: &Clock) {
    <b>assert</b>!(
        <a href="../social_contracts/jury.md#social_contracts_jury_eligible_count">eligible_count</a>(config, roster, clock::timestamp_ms(clock)) &gt;= config.eligibility.<a href="../social_contracts/jury.md#social_contracts_jury_min_eligible_roster">min_eligible_roster</a>,
        <a href="../social_contracts/jury.md#social_contracts_jury_ERosterTooSmall">ERosterTooSmall</a>,
    );
}
</code></pre>



</details>

<a name="social_contracts_jury_open_case"></a>

## Function `open_case`

Builds a case; the integrator adds evidence and fees, then calls [<code><a href="../social_contracts/jury.md#social_contracts_jury_share_case">share_case</a></code>]. <code>menu_len</code>
outcomes are indexes 0..menu_len-1; the integrator maps each to one fixed action.
<code>approval_bps</code> is the share of non-abstain reveals the leader needs (and it must lead
strictly); <code>quorum</code> is the minimum reveals (abstentions included) for a round to count.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_open_case">open_case</a>(config: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">social_contracts::jury::JuryConfig</a>, subject_kind: u8, subject_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, stage: u8, menu_len: u8, approval_bps: u64, quorum: u64, round_sizes: vector&lt;u64&gt;, appealable: bool, failure_path: u8, evidence_window_ms: u64, exclusion_owners: vector&lt;<b>address</b>&gt;, exclusion_orgs: vector&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;, exclusion_agents: vector&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;, fee: <a href="../myso/balance.md#myso_balance_Balance">myso::balance::Balance</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, max_juror_fee: u64, sweep_to: <b>address</b>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">social_contracts::jury::JuryCase</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_open_case">open_case</a>(
    config: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">JuryConfig</a>,
    subject_kind: u8,
    subject_id: ID,
    stage: u8,
    menu_len: u8,
    approval_bps: u64,
    quorum: u64,
    round_sizes: vector&lt;u64&gt;,
    appealable: bool,
    failure_path: u8,
    evidence_window_ms: u64,
    exclusion_owners: vector&lt;<b>address</b>&gt;,
    exclusion_orgs: vector&lt;ID&gt;,
    exclusion_agents: vector&lt;ID&gt;,
    fee: Balance&lt;MYSO&gt;,
    max_juror_fee: u64,
    sweep_to: <b>address</b>,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
): <a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">JuryCase</a> {
    <a href="../social_contracts/jury.md#social_contracts_jury_assert_config_live">assert_config_live</a>(config);
    <b>assert</b>!(config.active, <a href="../social_contracts/jury.md#social_contracts_jury_EJuryPaused">EJuryPaused</a>);
    <b>assert</b>!(subject_kind == <a href="../social_contracts/jury.md#social_contracts_jury_SUBJECT_TASK">SUBJECT_TASK</a> || subject_kind == <a href="../social_contracts/jury.md#social_contracts_jury_SUBJECT_GOVERNANCE">SUBJECT_GOVERNANCE</a>, <a href="../social_contracts/jury.md#social_contracts_jury_EInvalidCase">EInvalidCase</a>);
    <b>assert</b>!(menu_len &gt;= 2 && menu_len &lt;= <a href="../social_contracts/jury.md#social_contracts_jury_MAX_MENU_LEN">MAX_MENU_LEN</a>, <a href="../social_contracts/jury.md#social_contracts_jury_EInvalidCase">EInvalidCase</a>);
    <b>assert</b>!(approval_bps &gt;= 5000 && approval_bps &lt;= <a href="../social_contracts/jury.md#social_contracts_jury_BPS_DENOMINATOR">BPS_DENOMINATOR</a>, <a href="../social_contracts/jury.md#social_contracts_jury_EInvalidCase">EInvalidCase</a>);
    <b>assert</b>!(quorum &gt;= 1, <a href="../social_contracts/jury.md#social_contracts_jury_EInvalidCase">EInvalidCase</a>);
    <b>let</b> rounds = vector::length(&round_sizes);
    <b>assert</b>!(rounds &gt;= 1 && rounds &lt;= <a href="../social_contracts/jury.md#social_contracts_jury_MAX_ROUNDS_LIMIT">MAX_ROUNDS_LIMIT</a>, <a href="../social_contracts/jury.md#social_contracts_jury_EInvalidCase">EInvalidCase</a>);
    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (i &lt; rounds) {
        <b>assert</b>!(*vector::borrow(&round_sizes, i) &gt;= quorum, <a href="../social_contracts/jury.md#social_contracts_jury_EInvalidCase">EInvalidCase</a>);
        i = i + 1;
    };
    <b>assert</b>!(
        vector::length(&exclusion_owners) + vector::length(&exclusion_orgs) + vector::length(&exclusion_agents)
            &lt;= config.selection.<a href="../social_contracts/jury.md#social_contracts_jury_max_exclusions">max_exclusions</a>,
        <a href="../social_contracts/jury.md#social_contracts_jury_ETooManyExclusions">ETooManyExclusions</a>,
    );
    <b>let</b> now = clock::timestamp_ms(clock);
    <b>let</b> state = <b>if</b> (evidence_window_ms == 0) { <a href="../social_contracts/jury.md#social_contracts_jury_CASE_AWAITING_DRAW">CASE_AWAITING_DRAW</a> } <b>else</b> { <a href="../social_contracts/jury.md#social_contracts_jury_CASE_EVIDENCE">CASE_EVIDENCE</a> };
    <b>let</b> fee_mist = balance::value(&fee);
    <b>let</b> case = <a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">JuryCase</a> {
        id: object::new(ctx),
        version: <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(),
        subject_kind,
        subject_id,
        stage,
        state,
        fail_reason: <a href="../social_contracts/jury.md#social_contracts_jury_FAIL_NONE">FAIL_NONE</a>,
        round: 0,
        round_sizes,
        menu_len,
        approval_bps,
        quorum,
        appealable,
        failure_path,
        opened_by: tx_context::sender(ctx),
        created_at_ms: now,
        evidence_deadline_ms: <a href="../social_contracts/memory.md#social_contracts_memory_checked_add_u64">memory::checked_add_u64</a>(now, evidence_window_ms),
        appeal_deadline_ms: 0,
        prior_outcome: option::none(),
        provisional_outcome: option::none(),
        final_outcome: option::none(),
        executed: <b>false</b>,
        appeals_used: 0,
        failed_appeal: <b>false</b>,
        exclusions: <a href="../social_contracts/jury.md#social_contracts_jury_Exclusions">Exclusions</a> {
            owners: exclusion_owners,
            orgs: exclusion_orgs,
            agents: exclusion_agents,
            used_owners: vector::empty(),
            used_orgs: vector::empty(),
        },
        evidence: vector::empty(),
        rounds: vector::empty(),
        slots: table::new(ctx),
        fees: <a href="../social_contracts/jury.md#social_contracts_jury_CaseFees">CaseFees</a> {
            pool: fee,
            max_juror_fee,
            sweep_to,
            participants: 0,
            per_juror_fee: 0,
            reserved: 0,
            settled: <b>false</b>,
        },
    };
    event::emit(<a href="../social_contracts/jury.md#social_contracts_jury_CaseOpened">CaseOpened</a> {
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
</code></pre>



</details>

<a name="social_contracts_jury_share_case"></a>

## Function `share_case`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_share_case">share_case</a>(case: <a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">social_contracts::jury::JuryCase</a>): <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_share_case">share_case</a>(case: <a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">JuryCase</a>): ID {
    <b>let</b> case_id = object::id(&case);
    transfer::share_object(case);
    case_id
}
</code></pre>



</details>

<a name="social_contracts_jury_add_evidence"></a>

## Function `add_evidence`

Adds evidence before the evidence deadline (first round) or while an appeal round waits for its
draw (later rounds, flagged with the round so jurors can tell what is new). The integrator
checks who <code>submitter</code> is.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_add_evidence">add_evidence</a>(config: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">social_contracts::jury::JuryConfig</a>, case: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">social_contracts::jury::JuryCase</a>, submitter: <b>address</b>, kind: u8, digest: vector&lt;u8&gt;, uri: vector&lt;u8&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_add_evidence">add_evidence</a>(
    config: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">JuryConfig</a>,
    case: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">JuryCase</a>,
    submitter: <b>address</b>,
    kind: u8,
    digest: vector&lt;u8&gt;,
    uri: vector&lt;u8&gt;,
    clock: &Clock,
) {
    <b>let</b> now = clock::timestamp_ms(clock);
    <b>let</b> undrawn = vector::length(&case.rounds) == case.round;
    <b>let</b> open = (case.state == <a href="../social_contracts/jury.md#social_contracts_jury_CASE_EVIDENCE">CASE_EVIDENCE</a> && now &lt;= case.evidence_deadline_ms)
        || (case.state == <a href="../social_contracts/jury.md#social_contracts_jury_CASE_AWAITING_DRAW">CASE_AWAITING_DRAW</a> && undrawn);
    <b>assert</b>!(open, <a href="../social_contracts/jury.md#social_contracts_jury_EEvidenceClosed">EEvidenceClosed</a>);
    <b>assert</b>!(vector::length(&digest) == <a href="../social_contracts/jury.md#social_contracts_jury_DIGEST_LENGTH">DIGEST_LENGTH</a>, <a href="../social_contracts/jury.md#social_contracts_jury_EInvalidDigest">EInvalidDigest</a>);
    <b>assert</b>!(vector::length(&uri) &lt;= <a href="../social_contracts/jury.md#social_contracts_jury_MAX_URI_BYTES">MAX_URI_BYTES</a>, <a href="../social_contracts/jury.md#social_contracts_jury_EInvalidUri">EInvalidUri</a>);
    <b>let</b> <b>mut</b> count = 0;
    <b>let</b> <b>mut</b> i = 0;
    <b>let</b> len = vector::length(&case.evidence);
    <b>while</b> (i &lt; len) {
        <b>if</b> (vector::borrow(&case.evidence, i).submitter == submitter) {
            count = count + 1;
        };
        i = i + 1;
    };
    <b>assert</b>!(count &lt; config.selection.max_evidence_per_party, <a href="../social_contracts/jury.md#social_contracts_jury_ETooMuchEvidence">ETooMuchEvidence</a>);
    <b>let</b> item = <a href="../social_contracts/jury.md#social_contracts_jury_EvidenceItem">EvidenceItem</a> { submitter, round: case.round, kind, digest, uri, submitted_at_ms: now };
    event::emit(<a href="../social_contracts/jury.md#social_contracts_jury_EvidenceSubmitted">EvidenceSubmitted</a> {
        case_id: object::id(case),
        index: len,
        submitter,
        round: item.round,
        kind,
        digest: item.digest,
        uri: item.uri,
        submitted_at_ms: now,
    });
    vector::push_back(&<b>mut</b> case.evidence, item);
}
</code></pre>



</details>

<a name="social_contracts_jury_add_fee"></a>

## Function `add_fee`

Tops up the case fee (an appeal fee, a second party's fee). Only before the fees settle.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_add_fee">add_fee</a>(case: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">social_contracts::jury::JuryCase</a>, fee: <a href="../myso/balance.md#myso_balance_Balance">myso::balance::Balance</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_add_fee">add_fee</a>(case: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">JuryCase</a>, fee: Balance&lt;MYSO&gt;) {
    <b>assert</b>!(!case.fees.settled, <a href="../social_contracts/jury.md#social_contracts_jury_EFeesNotSettled">EFeesNotSettled</a>);
    balance::join(&<b>mut</b> case.fees.pool, fee);
}
</code></pre>



</details>

<a name="social_contracts_jury_appeal"></a>

## Function `appeal`

An appeal by the integrator on behalf of the losing side: a new, larger round with fresh jurors.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_appeal">appeal</a>(case: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">social_contracts::jury::JuryCase</a>, by: <b>address</b>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_appeal">appeal</a>(case: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">JuryCase</a>, by: <b>address</b>, clock: &Clock) {
    <b>assert</b>!(case.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/jury.md#social_contracts_jury_EWrongVersion">EWrongVersion</a>);
    <b>assert</b>!(case.state == <a href="../social_contracts/jury.md#social_contracts_jury_CASE_TALLIED">CASE_TALLIED</a>, <a href="../social_contracts/jury.md#social_contracts_jury_EWrongCaseState">EWrongCaseState</a>);
    <b>let</b> now = clock::timestamp_ms(clock);
    <b>assert</b>!(now &lt;= case.appeal_deadline_ms, <a href="../social_contracts/jury.md#social_contracts_jury_EWindowClosed">EWindowClosed</a>);
    <b>assert</b>!(case.round + 1 &lt; vector::length(&case.round_sizes), <a href="../social_contracts/jury.md#social_contracts_jury_ENotAppealable">ENotAppealable</a>);
    <b>let</b> prior = *option::borrow(&case.provisional_outcome);
    case.prior_outcome = option::some(prior);
    case.provisional_outcome = option::none();
    case.round = case.round + 1;
    case.appeals_used = case.appeals_used + 1;
    case.state = <a href="../social_contracts/jury.md#social_contracts_jury_CASE_AWAITING_DRAW">CASE_AWAITING_DRAW</a>;
    event::emit(<a href="../social_contracts/jury.md#social_contracts_jury_CaseAppealed">CaseAppealed</a> { case_id: object::id(case), by, round: case.round, prior_outcome: prior });
}
</code></pre>



</details>

<a name="social_contracts_jury_cancel_case"></a>

## Function `cancel_case`

Cancels a case no juror has been drawn for (a settlement, or the initiator withdrawing).


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_cancel_case">cancel_case</a>(case: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">social_contracts::jury::JuryCase</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_cancel_case">cancel_case</a>(case: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">JuryCase</a>, clock: &Clock) {
    <b>assert</b>!(case.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/jury.md#social_contracts_jury_EWrongVersion">EWrongVersion</a>);
    <b>let</b> undrawn = case.round == 0 && vector::length(&case.rounds) == 0;
    <b>assert</b>!(undrawn && (case.state == <a href="../social_contracts/jury.md#social_contracts_jury_CASE_EVIDENCE">CASE_EVIDENCE</a> || case.state == <a href="../social_contracts/jury.md#social_contracts_jury_CASE_AWAITING_DRAW">CASE_AWAITING_DRAW</a>), <a href="../social_contracts/jury.md#social_contracts_jury_EWrongCaseState">EWrongCaseState</a>);
    <b>let</b> now = clock::timestamp_ms(clock);
    case.state = <a href="../social_contracts/jury.md#social_contracts_jury_CASE_CANCELLED">CASE_CANCELLED</a>;
    <a href="../social_contracts/jury.md#social_contracts_jury_settle_fees">settle_fees</a>(case);
    event::emit(<a href="../social_contracts/jury.md#social_contracts_jury_CaseCancelled">CaseCancelled</a> { case_id: object::id(case), cancelled_at_ms: now });
}
</code></pre>



</details>

<a name="social_contracts_jury_sweep_remainder"></a>

## Function `sweep_remainder`

What is left after jurors are paid. Jurors' unclaimed shares stay reserved in the pool.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_sweep_remainder">sweep_remainder</a>(case: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">social_contracts::jury::JuryCase</a>): <a href="../myso/balance.md#myso_balance_Balance">myso::balance::Balance</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_sweep_remainder">sweep_remainder</a>(case: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">JuryCase</a>): Balance&lt;MYSO&gt; {
    <b>assert</b>!(case.fees.settled, <a href="../social_contracts/jury.md#social_contracts_jury_EFeesNotSettled">EFeesNotSettled</a>);
    <b>let</b> total = balance::value(&case.fees.pool);
    <b>let</b> excess = total - case.fees.reserved;
    balance::split(&<b>mut</b> case.fees.pool, excess)
}
</code></pre>



</details>

<a name="social_contracts_jury_mark_executed"></a>

## Function `mark_executed`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_mark_executed">mark_executed</a>(case: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">social_contracts::jury::JuryCase</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_mark_executed">mark_executed</a>(case: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">JuryCase</a>) {
    <b>assert</b>!(!case.executed, <a href="../social_contracts/jury.md#social_contracts_jury_EAlreadyExecuted">EAlreadyExecuted</a>);
    <b>assert</b>!(<a href="../social_contracts/jury.md#social_contracts_jury_is_finished">is_finished</a>(case.state), <a href="../social_contracts/jury.md#social_contracts_jury_ENotFinished">ENotFinished</a>);
    case.executed = <b>true</b>;
}
</code></pre>



</details>

<a name="social_contracts_jury_is_finished"></a>

## Function `is_finished`



<pre><code><b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_is_finished">is_finished</a>(state: u8): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_is_finished">is_finished</a>(state: u8): bool {
    state == <a href="../social_contracts/jury.md#social_contracts_jury_CASE_FINAL">CASE_FINAL</a> || state == <a href="../social_contracts/jury.md#social_contracts_jury_CASE_FAILED">CASE_FAILED</a> || state == <a href="../social_contracts/jury.md#social_contracts_jury_CASE_CANCELLED">CASE_CANCELLED</a>
}
</code></pre>



</details>

<a name="social_contracts_jury_select_jury"></a>

## Function `select_jury`

Draws the jurors for the current round. <code><b>entry</b></code>, never <code><b>public</b></code>, so a transaction cannot
compose the draw and revert when the result is unfavourable. Anyone may call it once the
evidence window has closed (or an appeal/next round is waiting).


<pre><code><b>entry</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_select_jury">select_jury</a>(config: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">social_contracts::jury::JuryConfig</a>, roster: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JurorRoster">social_contracts::jury::JurorRoster</a>, case: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">social_contracts::jury::JuryCase</a>, rand: &<a href="../myso/random.md#myso_random_Random">myso::random::Random</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>entry</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_select_jury">select_jury</a>(
    config: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">JuryConfig</a>,
    roster: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JurorRoster">JurorRoster</a>,
    case: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">JuryCase</a>,
    rand: &Random,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <a href="../social_contracts/jury.md#social_contracts_jury_assert_live">assert_live</a>(config, roster, case);
    <b>let</b> now = clock::timestamp_ms(clock);
    <b>let</b> ready = case.state == <a href="../social_contracts/jury.md#social_contracts_jury_CASE_AWAITING_DRAW">CASE_AWAITING_DRAW</a>
        || (case.state == <a href="../social_contracts/jury.md#social_contracts_jury_CASE_EVIDENCE">CASE_EVIDENCE</a> && now &gt; case.evidence_deadline_ms);
    <b>assert</b>!(ready, <a href="../social_contracts/jury.md#social_contracts_jury_EWrongCaseState">EWrongCaseState</a>);
    <b>let</b> round = case.round;
    <b>assert</b>!(vector::length(&case.rounds) == round, <a href="../social_contracts/jury.md#social_contracts_jury_EWrongCaseState">EWrongCaseState</a>);
    <b>let</b> size = *vector::borrow(&case.round_sizes, round);
    <b>let</b> <b>mut</b> gen = random::new_generator(rand, ctx);
    <b>let</b> drawn = <a href="../social_contracts/jury.md#social_contracts_jury_draw_jurors">draw_jurors</a>(config, roster, case, &<b>mut</b> gen, size, round, now);
    <b>let</b> accept_deadline = <a href="../social_contracts/memory.md#social_contracts_memory_checked_add_u64">memory::checked_add_u64</a>(now, config.timing.<a href="../social_contracts/jury.md#social_contracts_jury_accept_window_ms">accept_window_ms</a>);
    <b>let</b> <b>mut</b> tally = vector::empty&lt;u64&gt;();
    <b>let</b> <b>mut</b> m = 0u8;
    <b>while</b> (m &lt; case.menu_len) {
        vector::push_back(&<b>mut</b> tally, 0);
        m = m + 1;
    };
    vector::push_back(&<b>mut</b> case.rounds, <a href="../social_contracts/jury.md#social_contracts_jury_RoundState">RoundState</a> {
        size,
        jurors: <b>copy</b> drawn,
        accept_deadline_ms: accept_deadline,
        commit_deadline_ms: 0,
        reveal_deadline_ms: 0,
        replacements_used: 0,
        tally,
        <a href="../social_contracts/jury.md#social_contracts_jury_abstain">abstain</a>: 0,
        revealed: 0,
        closed: <b>false</b>,
    });
    case.state = <a href="../social_contracts/jury.md#social_contracts_jury_CASE_SELECTING">CASE_SELECTING</a>;
    event::emit(<a href="../social_contracts/jury.md#social_contracts_jury_JuryDrawn">JuryDrawn</a> {
        case_id: object::id(case),
        round,
        jurors: drawn,
        accept_deadline_ms: accept_deadline,
        replacement: <b>false</b>,
    });
}
</code></pre>



</details>

<a name="social_contracts_jury_draw_jurors"></a>

## Function `draw_jurors`

Uniform rejection sampling over the roster with a fixed attempt budget. A candidate is
skipped when the snapshot says it cannot serve or it conflicts with the case; one juror per
owner and per organisation is kept across every round of the case.


<pre><code><b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_draw_jurors">draw_jurors</a>(config: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">social_contracts::jury::JuryConfig</a>, roster: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JurorRoster">social_contracts::jury::JurorRoster</a>, case: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">social_contracts::jury::JuryCase</a>, gen: &<b>mut</b> <a href="../myso/random.md#myso_random_RandomGenerator">myso::random::RandomGenerator</a>, needed: u64, round: u64, now: u64): vector&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_draw_jurors">draw_jurors</a>(
    config: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">JuryConfig</a>,
    roster: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JurorRoster">JurorRoster</a>,
    case: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">JuryCase</a>,
    gen: &<b>mut</b> RandomGenerator,
    needed: u64,
    round: u64,
    now: u64,
): vector&lt;ID&gt; {
    <b>let</b> <b>mut</b> chosen = vector::empty&lt;ID&gt;();
    <b>if</b> (roster.count == 0 || needed == 0) {
        <b>return</b> chosen
    };
    <b>let</b> <b>mut</b> attempts = 0;
    <b>while</b> (attempts &lt; config.selection.max_draws && vector::length(&chosen) &lt; needed) {
        attempts = attempts + 1;
        <b>let</b> position = random::generate_u64_in_range(gen, 0, roster.count - 1);
        <b>let</b> agent_id = *table::borrow(&roster.slots, position);
        <b>let</b> (picked, owner, org, payout) = {
            <b>let</b> <b>entry</b> = table::borrow(&roster.entries, agent_id);
            <b>let</b> ok = <a href="../social_contracts/jury.md#social_contracts_jury_draw_eligible">draw_eligible</a>(config, <b>entry</b>, now) && !<a href="../social_contracts/jury.md#social_contracts_jury_conflicted">conflicted</a>(case, <b>entry</b>);
            (ok, <b>entry</b>.owner, <b>entry</b>.organization_id, <b>entry</b>.payout_address)
        };
        <b>if</b> (!picked) {
            <b>continue</b>
        };
        {
            <b>let</b> seated = table::borrow_mut(&<b>mut</b> roster.entries, agent_id);
            seated.active_cases = seated.active_cases + 1;
        };
        vector::push_back(&<b>mut</b> case.exclusions.used_owners, owner);
        vector::push_back(&<b>mut</b> case.exclusions.used_orgs, org);
        table::add(&<b>mut</b> case.slots, agent_id, <a href="../social_contracts/jury.md#social_contracts_jury_JurorSlot">JurorSlot</a> {
            agent_id,
            owner,
            organization_id: org,
            payout_address: payout,
            round,
            status: <a href="../social_contracts/jury.md#social_contracts_jury_SLOT_DRAFTED">SLOT_DRAFTED</a>,
            commitment: vector::empty(),
            outcome: <a href="../social_contracts/jury.md#social_contracts_jury_ABSTAIN">ABSTAIN</a>,
            rationale_digest: vector::empty(),
            evidence_refs: vector::empty(),
            bundle_digest: vector::empty(),
            drafted_at_ms: now,
            accepted_at_ms: 0,
            revealed_at_ms: 0,
            claimed: <b>false</b>,
        });
        vector::push_back(&<b>mut</b> chosen, agent_id);
    };
    chosen
}
</code></pre>



</details>

<a name="social_contracts_jury_conflicted"></a>

## Function `conflicted`



<pre><code><b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_conflicted">conflicted</a>(case: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">social_contracts::jury::JuryCase</a>, <b>entry</b>: &<a href="../social_contracts/jury.md#social_contracts_jury_RosterEntry">social_contracts::jury::RosterEntry</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_conflicted">conflicted</a>(case: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">JuryCase</a>, <b>entry</b>: &<a href="../social_contracts/jury.md#social_contracts_jury_RosterEntry">RosterEntry</a>): bool {
    <b>let</b> ex = &case.exclusions;
    <b>if</b> (vector::contains(&ex.owners, &<b>entry</b>.owner)
        || vector::contains(&ex.orgs, &<b>entry</b>.organization_id)
        || vector::contains(&ex.agents, &<b>entry</b>.agent_id)
        || vector::contains(&ex.used_owners, &<b>entry</b>.owner)
        || vector::contains(&ex.used_orgs, &<b>entry</b>.organization_id)) {
        <b>return</b> <b>true</b>
    };
    // A juror that descends from an excluded agent is <b>as</b> <a href="../social_contracts/jury.md#social_contracts_jury_conflicted">conflicted</a> <b>as</b> the agent itself.
    <b>let</b> <b>mut</b> i = 0;
    <b>let</b> len = vector::length(&<b>entry</b>.ancestor_ids);
    <b>while</b> (i &lt; len) {
        <b>if</b> (vector::contains(&ex.agents, vector::borrow(&<b>entry</b>.ancestor_ids, i))) {
            <b>return</b> <b>true</b>
        };
        i = i + 1;
    };
    <b>false</b>
}
</code></pre>



</details>

<a name="social_contracts_jury_accept_assignment"></a>

## Function `accept_assignment`

A drafted juror takes the seat. The juror proves the live state here (active, the capability,
reputation, class), which is the authoritative check behind the draw's snapshot.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_accept_assignment">accept_assignment</a>(config: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">social_contracts::jury::JuryConfig</a>, case: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">social_contracts::jury::JuryCase</a>, memory_config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_accept_assignment">accept_assignment</a>(
    config: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">JuryConfig</a>,
    case: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">JuryCase</a>,
    memory_config: &MemoryConfig,
    account: &MemoryAccount,
    agent: &SubAgent,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <a href="../social_contracts/jury.md#social_contracts_jury_assert_config_live">assert_config_live</a>(config);
    <b>assert</b>!(case.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/jury.md#social_contracts_jury_EWrongVersion">EWrongVersion</a>);
    <b>assert</b>!(case.state == <a href="../social_contracts/jury.md#social_contracts_jury_CASE_SELECTING">CASE_SELECTING</a>, <a href="../social_contracts/jury.md#social_contracts_jury_EWrongCaseState">EWrongCaseState</a>);
    <b>let</b> agent_id = object::id(agent);
    <b>assert</b>!(table::contains(&case.slots, agent_id), <a href="../social_contracts/jury.md#social_contracts_jury_ENotDrafted">ENotDrafted</a>);
    <a href="../social_contracts/jury.md#social_contracts_jury_assert_juror_actor">assert_juror_actor</a>(memory_config, account, agent, clock, ctx);
    <b>let</b> now = clock::timestamp_ms(clock);
    <b>let</b> round = case.round;
    {
        <b>let</b> slot = table::borrow(&case.slots, agent_id);
        <b>assert</b>!(slot.status == <a href="../social_contracts/jury.md#social_contracts_jury_SLOT_DRAFTED">SLOT_DRAFTED</a> && slot.round == round, <a href="../social_contracts/jury.md#social_contracts_jury_EWrongSlotStatus">EWrongSlotStatus</a>);
        <b>let</b> r = vector::borrow(&case.rounds, round);
        <b>assert</b>!(now &lt;= r.accept_deadline_ms, <a href="../social_contracts/jury.md#social_contracts_jury_EWindowClosed">EWindowClosed</a>);
    };
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_sub_agent_active">memory::assert_sub_agent_active</a>(agent, clock);
    <b>let</b> snap = <a href="../social_contracts/jury.md#social_contracts_jury_take_snapshot">take_snapshot</a>(config, memory_config, account, agent);
    <b>assert</b>!(snap.ok, <a href="../social_contracts/jury.md#social_contracts_jury_EIneligible">EIneligible</a>);
    <b>assert</b>!(
        <a href="../social_contracts/jury.md#social_contracts_jury_is_genesis_exempt">is_genesis_exempt</a>(config, agent_id, now)
            || <a href="../social_contracts/jury.md#social_contracts_jury_meets_reputation">meets_reputation</a>(config, snap.score_milli, snap.confidence_bps, snap.settled_count, snap.created_at_ms, now),
        <a href="../social_contracts/jury.md#social_contracts_jury_EIneligible">EIneligible</a>,
    );
    {
        <b>let</b> slot = table::borrow_mut(&<b>mut</b> case.slots, agent_id);
        slot.status = <a href="../social_contracts/jury.md#social_contracts_jury_SLOT_ACCEPTED">SLOT_ACCEPTED</a>;
        slot.accepted_at_ms = now;
    };
    event::emit(<a href="../social_contracts/jury.md#social_contracts_jury_AssignmentAccepted">AssignmentAccepted</a> { case_id: object::id(case), round, agent_id, accepted_at_ms: now });
    <b>if</b> (<a href="../social_contracts/jury.md#social_contracts_jury_all_seats_accepted">all_seats_accepted</a>(case, round)) {
        <a href="../social_contracts/jury.md#social_contracts_jury_start_voting">start_voting</a>(config, case, now);
    };
}
</code></pre>



</details>

<a name="social_contracts_jury_all_seats_accepted"></a>

## Function `all_seats_accepted`



<pre><code><b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_all_seats_accepted">all_seats_accepted</a>(case: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">social_contracts::jury::JuryCase</a>, round: u64): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_all_seats_accepted">all_seats_accepted</a>(case: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">JuryCase</a>, round: u64): bool {
    <b>let</b> r = vector::borrow(&case.rounds, round);
    <b>if</b> (vector::length(&r.jurors) &lt; r.size) {
        <b>return</b> <b>false</b>
    };
    <b>let</b> <b>mut</b> i = 0;
    <b>let</b> len = vector::length(&r.jurors);
    <b>while</b> (i &lt; len) {
        <b>let</b> id = *vector::borrow(&r.jurors, i);
        <b>if</b> (table::borrow(&case.slots, id).status == <a href="../social_contracts/jury.md#social_contracts_jury_SLOT_DRAFTED">SLOT_DRAFTED</a>) {
            <b>return</b> <b>false</b>
        };
        i = i + 1;
    };
    <b>true</b>
}
</code></pre>



</details>

<a name="social_contracts_jury_start_voting"></a>

## Function `start_voting`



<pre><code><b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_start_voting">start_voting</a>(config: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">social_contracts::jury::JuryConfig</a>, case: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">social_contracts::jury::JuryCase</a>, now: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_start_voting">start_voting</a>(config: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">JuryConfig</a>, case: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">JuryCase</a>, now: u64) {
    <b>let</b> round = case.round;
    <b>let</b> commit_deadline = <a href="../social_contracts/memory.md#social_contracts_memory_checked_add_u64">memory::checked_add_u64</a>(now, config.timing.<a href="../social_contracts/jury.md#social_contracts_jury_commit_window_ms">commit_window_ms</a>);
    <b>let</b> reveal_deadline = <a href="../social_contracts/memory.md#social_contracts_memory_checked_add_u64">memory::checked_add_u64</a>(
        commit_deadline,
        <a href="../social_contracts/memory.md#social_contracts_memory_checked_add_u64">memory::checked_add_u64</a>(config.timing.<a href="../social_contracts/jury.md#social_contracts_jury_reveal_window_ms">reveal_window_ms</a>, config.timing.<a href="../social_contracts/jury.md#social_contracts_jury_reveal_grace_ms">reveal_grace_ms</a>),
    );
    <b>let</b> jurors = {
        <b>let</b> r = vector::borrow_mut(&<b>mut</b> case.rounds, round);
        r.commit_deadline_ms = commit_deadline;
        r.reveal_deadline_ms = reveal_deadline;
        r.jurors
    };
    case.state = <a href="../social_contracts/jury.md#social_contracts_jury_CASE_VOTING">CASE_VOTING</a>;
    event::emit(<a href="../social_contracts/jury.md#social_contracts_jury_VotingStarted">VotingStarted</a> {
        case_id: object::id(case),
        round,
        jurors,
        commit_deadline_ms: commit_deadline,
        reveal_deadline_ms: reveal_deadline,
    });
}
</code></pre>



</details>

<a name="social_contracts_jury_replace_no_shows"></a>

## Function `replace_no_shows`

After the accept window: seats that never accepted are replaced by fresh draws (a few times),
each no-show costs the juror duty score and a strike, and if the seats still cannot be filled
the round goes ahead with the jurors it has when they meet the quorum, otherwise the case fails.


<pre><code><b>entry</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_replace_no_shows">replace_no_shows</a>(config: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">social_contracts::jury::JuryConfig</a>, roster: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JurorRoster">social_contracts::jury::JurorRoster</a>, case: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">social_contracts::jury::JuryCase</a>, rand: &<a href="../myso/random.md#myso_random_Random">myso::random::Random</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>entry</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_replace_no_shows">replace_no_shows</a>(
    config: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">JuryConfig</a>,
    roster: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JurorRoster">JurorRoster</a>,
    case: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">JuryCase</a>,
    rand: &Random,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <a href="../social_contracts/jury.md#social_contracts_jury_assert_live">assert_live</a>(config, roster, case);
    <b>assert</b>!(case.state == <a href="../social_contracts/jury.md#social_contracts_jury_CASE_SELECTING">CASE_SELECTING</a>, <a href="../social_contracts/jury.md#social_contracts_jury_EWrongCaseState">EWrongCaseState</a>);
    <b>let</b> now = clock::timestamp_ms(clock);
    <b>let</b> round = case.round;
    <b>let</b> (size, seats) = {
        <b>let</b> r = vector::borrow(&case.rounds, round);
        <b>assert</b>!(now &gt; r.accept_deadline_ms, <a href="../social_contracts/jury.md#social_contracts_jury_ENotYet">ENotYet</a>);
        (r.size, r.jurors)
    };
    <b>let</b> case_id = object::id(case);
    <b>let</b> <b>mut</b> kept = vector::empty&lt;ID&gt;();
    <b>let</b> <b>mut</b> i = 0;
    <b>let</b> len = vector::length(&seats);
    <b>while</b> (i &lt; len) {
        <b>let</b> agent_id = *vector::borrow(&seats, i);
        <b>if</b> (table::borrow(&case.slots, agent_id).status == <a href="../social_contracts/jury.md#social_contracts_jury_SLOT_DRAFTED">SLOT_DRAFTED</a>) {
            table::borrow_mut(&<b>mut</b> case.slots, agent_id).status = <a href="../social_contracts/jury.md#social_contracts_jury_SLOT_REPLACED">SLOT_REPLACED</a>;
            <a href="../social_contracts/jury.md#social_contracts_jury_penalize_duty">penalize_duty</a>(config, roster, agent_id, config.duty.loss_accept, <a href="../social_contracts/jury.md#social_contracts_jury_DUTY_NO_ACCEPT">DUTY_NO_ACCEPT</a>, case_id, now);
            <a href="../social_contracts/jury.md#social_contracts_jury_release_seat">release_seat</a>(roster, agent_id);
            event::emit(<a href="../social_contracts/jury.md#social_contracts_jury_JurorReplaced">JurorReplaced</a> { case_id, round, agent_id });
        } <b>else</b> {
            vector::push_back(&<b>mut</b> kept, agent_id);
        };
        i = i + 1;
    };
    <b>let</b> accepted = vector::length(&kept);
    vector::borrow_mut(&<b>mut</b> case.rounds, round).jurors = kept;
    <b>let</b> missing = size - accepted;
    <b>if</b> (missing == 0) {
        <a href="../social_contracts/jury.md#social_contracts_jury_start_voting">start_voting</a>(config, case, now);
        <b>return</b>
    };
    <b>let</b> used = vector::borrow(&case.rounds, round).replacements_used;
    <b>if</b> (used &lt; config.selection.max_replacements) {
        <b>let</b> <b>mut</b> gen = random::new_generator(rand, ctx);
        <b>let</b> drawn = <a href="../social_contracts/jury.md#social_contracts_jury_draw_jurors">draw_jurors</a>(config, roster, case, &<b>mut</b> gen, missing, round, now);
        <b>let</b> accept_deadline = <a href="../social_contracts/memory.md#social_contracts_memory_checked_add_u64">memory::checked_add_u64</a>(now, config.timing.<a href="../social_contracts/jury.md#social_contracts_jury_accept_window_ms">accept_window_ms</a>);
        {
            <b>let</b> r = vector::borrow_mut(&<b>mut</b> case.rounds, round);
            vector::append(&<b>mut</b> r.jurors, <b>copy</b> drawn);
            r.replacements_used = r.replacements_used + 1;
            r.accept_deadline_ms = accept_deadline;
        };
        event::emit(<a href="../social_contracts/jury.md#social_contracts_jury_JuryDrawn">JuryDrawn</a> { case_id, round, jurors: drawn, accept_deadline_ms: accept_deadline, replacement: <b>true</b> });
    } <b>else</b> <b>if</b> (accepted &gt;= case.quorum) {
        <a href="../social_contracts/jury.md#social_contracts_jury_start_voting">start_voting</a>(config, case, now);
    } <b>else</b> {
        <a href="../social_contracts/jury.md#social_contracts_jury_close_round">close_round</a>(config, roster, case, now, <b>true</b>);
        <a href="../social_contracts/jury.md#social_contracts_jury_end_failed">end_failed</a>(case, <a href="../social_contracts/jury.md#social_contracts_jury_FAIL_CANNOT_FORM">FAIL_CANNOT_FORM</a>, now);
    };
}
</code></pre>



</details>

<a name="social_contracts_jury_commit_vote"></a>

## Function `commit_vote`

Hidden vote. <code>commitment</code> binds the case, round, juror, outcome, rationale, evidence refs,
bundle and a secret salt; nobody can see or copy a vote until it is revealed.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_commit_vote">commit_vote</a>(case: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">social_contracts::jury::JuryCase</a>, memory_config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>, commitment: vector&lt;u8&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_commit_vote">commit_vote</a>(
    case: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">JuryCase</a>,
    memory_config: &MemoryConfig,
    account: &MemoryAccount,
    agent: &SubAgent,
    commitment: vector&lt;u8&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(case.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/jury.md#social_contracts_jury_EWrongVersion">EWrongVersion</a>);
    <b>assert</b>!(case.state == <a href="../social_contracts/jury.md#social_contracts_jury_CASE_VOTING">CASE_VOTING</a>, <a href="../social_contracts/jury.md#social_contracts_jury_EWrongCaseState">EWrongCaseState</a>);
    <b>let</b> agent_id = object::id(agent);
    <b>assert</b>!(table::contains(&case.slots, agent_id), <a href="../social_contracts/jury.md#social_contracts_jury_ENotDrafted">ENotDrafted</a>);
    <a href="../social_contracts/jury.md#social_contracts_jury_assert_juror_actor">assert_juror_actor</a>(memory_config, account, agent, clock, ctx);
    <b>assert</b>!(vector::length(&commitment) == <a href="../social_contracts/jury.md#social_contracts_jury_DIGEST_LENGTH">DIGEST_LENGTH</a>, <a href="../social_contracts/jury.md#social_contracts_jury_EInvalidDigest">EInvalidDigest</a>);
    <b>let</b> now = clock::timestamp_ms(clock);
    <b>let</b> round = case.round;
    <b>assert</b>!(now &lt;= vector::borrow(&case.rounds, round).commit_deadline_ms, <a href="../social_contracts/jury.md#social_contracts_jury_EWindowClosed">EWindowClosed</a>);
    <b>let</b> slot = table::borrow_mut(&<b>mut</b> case.slots, agent_id);
    <b>assert</b>!(slot.round == round && slot.status == <a href="../social_contracts/jury.md#social_contracts_jury_SLOT_ACCEPTED">SLOT_ACCEPTED</a>, <a href="../social_contracts/jury.md#social_contracts_jury_EWrongSlotStatus">EWrongSlotStatus</a>);
    slot.status = <a href="../social_contracts/jury.md#social_contracts_jury_SLOT_COMMITTED">SLOT_COMMITTED</a>;
    slot.commitment = <b>copy</b> commitment;
    event::emit(<a href="../social_contracts/jury.md#social_contracts_jury_VoteCommitted">VoteCommitted</a> { case_id: object::id(case), round, agent_id, commitment });
}
</code></pre>



</details>

<a name="social_contracts_jury_reveal_vote"></a>

## Function `reveal_vote`

Opens a vote. Allowed once the commit window is over, or earlier when every seat has
committed. Must hash to the commitment, back a menu outcome (or abstain), and cite evidence
that exists on the case.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_reveal_vote">reveal_vote</a>(case: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">social_contracts::jury::JuryCase</a>, memory_config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>, outcome: u8, rationale_digest: vector&lt;u8&gt;, evidence_refs: vector&lt;u64&gt;, bundle_digest: vector&lt;u8&gt;, salt: vector&lt;u8&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_reveal_vote">reveal_vote</a>(
    case: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">JuryCase</a>,
    memory_config: &MemoryConfig,
    account: &MemoryAccount,
    agent: &SubAgent,
    outcome: u8,
    rationale_digest: vector&lt;u8&gt;,
    evidence_refs: vector&lt;u64&gt;,
    bundle_digest: vector&lt;u8&gt;,
    salt: vector&lt;u8&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(case.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/jury.md#social_contracts_jury_EWrongVersion">EWrongVersion</a>);
    <b>assert</b>!(case.state == <a href="../social_contracts/jury.md#social_contracts_jury_CASE_VOTING">CASE_VOTING</a>, <a href="../social_contracts/jury.md#social_contracts_jury_EWrongCaseState">EWrongCaseState</a>);
    <b>let</b> agent_id = object::id(agent);
    <b>assert</b>!(table::contains(&case.slots, agent_id), <a href="../social_contracts/jury.md#social_contracts_jury_ENotDrafted">ENotDrafted</a>);
    <a href="../social_contracts/jury.md#social_contracts_jury_assert_juror_actor">assert_juror_actor</a>(memory_config, account, agent, clock, ctx);
    <b>let</b> now = clock::timestamp_ms(clock);
    <b>let</b> round = case.round;
    {
        <b>let</b> r = vector::borrow(&case.rounds, round);
        <b>assert</b>!(now &lt;= r.reveal_deadline_ms, <a href="../social_contracts/jury.md#social_contracts_jury_EWindowClosed">EWindowClosed</a>);
        <b>assert</b>!(now &gt; r.commit_deadline_ms || <a href="../social_contracts/jury.md#social_contracts_jury_none_uncommitted">none_uncommitted</a>(case, round), <a href="../social_contracts/jury.md#social_contracts_jury_ENotYet">ENotYet</a>);
    };
    <b>assert</b>!(vector::length(&rationale_digest) == <a href="../social_contracts/jury.md#social_contracts_jury_DIGEST_LENGTH">DIGEST_LENGTH</a>, <a href="../social_contracts/jury.md#social_contracts_jury_EInvalidDigest">EInvalidDigest</a>);
    <b>assert</b>!(vector::length(&bundle_digest) == <a href="../social_contracts/jury.md#social_contracts_jury_DIGEST_LENGTH">DIGEST_LENGTH</a>, <a href="../social_contracts/jury.md#social_contracts_jury_EInvalidDigest">EInvalidDigest</a>);
    <b>let</b> salt_len = vector::length(&salt);
    <b>assert</b>!(salt_len &gt;= <a href="../social_contracts/jury.md#social_contracts_jury_MIN_SALT_BYTES">MIN_SALT_BYTES</a> && salt_len &lt;= <a href="../social_contracts/jury.md#social_contracts_jury_MAX_SALT_BYTES">MAX_SALT_BYTES</a>, <a href="../social_contracts/jury.md#social_contracts_jury_EInvalidSalt">EInvalidSalt</a>);
    <b>assert</b>!(outcome == <a href="../social_contracts/jury.md#social_contracts_jury_ABSTAIN">ABSTAIN</a> || outcome &lt; case.menu_len, <a href="../social_contracts/jury.md#social_contracts_jury_EInvalidOutcome">EInvalidOutcome</a>);
    <b>if</b> (outcome != <a href="../social_contracts/jury.md#social_contracts_jury_ABSTAIN">ABSTAIN</a>) {
        <b>let</b> evidence_count = vector::length(&case.evidence);
        <b>assert</b>!(!vector::is_empty(&evidence_refs), <a href="../social_contracts/jury.md#social_contracts_jury_EInvalidEvidenceRef">EInvalidEvidenceRef</a>);
        <b>let</b> <b>mut</b> i = 0;
        <b>let</b> n = vector::length(&evidence_refs);
        <b>while</b> (i &lt; n) {
            <b>assert</b>!(*vector::borrow(&evidence_refs, i) &lt; evidence_count, <a href="../social_contracts/jury.md#social_contracts_jury_EInvalidEvidenceRef">EInvalidEvidenceRef</a>);
            i = i + 1;
        };
    };
    <b>let</b> expected = <a href="../social_contracts/jury.md#social_contracts_jury_compute_commitment">compute_commitment</a>(
        object::id(case), round, agent_id, outcome, &rationale_digest, &evidence_refs, &bundle_digest, &salt,
    );
    {
        <b>let</b> slot = table::borrow_mut(&<b>mut</b> case.slots, agent_id);
        <b>assert</b>!(slot.round == round && slot.status == <a href="../social_contracts/jury.md#social_contracts_jury_SLOT_COMMITTED">SLOT_COMMITTED</a>, <a href="../social_contracts/jury.md#social_contracts_jury_EWrongSlotStatus">EWrongSlotStatus</a>);
        <b>assert</b>!(slot.commitment == expected, <a href="../social_contracts/jury.md#social_contracts_jury_ECommitmentMismatch">ECommitmentMismatch</a>);
        slot.status = <a href="../social_contracts/jury.md#social_contracts_jury_SLOT_REVEALED">SLOT_REVEALED</a>;
        slot.outcome = outcome;
        slot.rationale_digest = <b>copy</b> rationale_digest;
        slot.evidence_refs = <b>copy</b> evidence_refs;
        slot.bundle_digest = <b>copy</b> bundle_digest;
        slot.revealed_at_ms = now;
    };
    {
        <b>let</b> r = vector::borrow_mut(&<b>mut</b> case.rounds, round);
        r.revealed = r.revealed + 1;
        <b>if</b> (outcome == <a href="../social_contracts/jury.md#social_contracts_jury_ABSTAIN">ABSTAIN</a>) {
            r.<a href="../social_contracts/jury.md#social_contracts_jury_abstain">abstain</a> = r.<a href="../social_contracts/jury.md#social_contracts_jury_abstain">abstain</a> + 1;
        } <b>else</b> {
            <b>let</b> count = vector::borrow_mut(&<b>mut</b> r.tally, outcome <b>as</b> u64);
            *count = *count + 1;
        };
    };
    case.fees.participants = case.fees.participants + 1;
    event::emit(<a href="../social_contracts/jury.md#social_contracts_jury_VoteRevealed">VoteRevealed</a> {
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
</code></pre>



</details>

<a name="social_contracts_jury_none_uncommitted"></a>

## Function `none_uncommitted`



<pre><code><b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_none_uncommitted">none_uncommitted</a>(case: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">social_contracts::jury::JuryCase</a>, round: u64): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_none_uncommitted">none_uncommitted</a>(case: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">JuryCase</a>, round: u64): bool {
    <b>let</b> r = vector::borrow(&case.rounds, round);
    <b>let</b> <b>mut</b> i = 0;
    <b>let</b> len = vector::length(&r.jurors);
    <b>while</b> (i &lt; len) {
        <b>let</b> id = *vector::borrow(&r.jurors, i);
        <b>if</b> (table::borrow(&case.slots, id).status == <a href="../social_contracts/jury.md#social_contracts_jury_SLOT_ACCEPTED">SLOT_ACCEPTED</a>) {
            <b>return</b> <b>false</b>
        };
        i = i + 1;
    };
    <b>true</b>
}
</code></pre>



</details>

<a name="social_contracts_jury_compute_commitment"></a>

## Function `compute_commitment`

The encoding the runner must reproduce: BCS of case id, round, agent id and outcome; the raw
32-byte rationale digest; BCS of the evidence refs; the raw 32-byte bundle digest; the salt.


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_compute_commitment">compute_commitment</a>(case_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, round: u64, agent_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, outcome: u8, rationale_digest: &vector&lt;u8&gt;, evidence_refs: &vector&lt;u64&gt;, bundle_digest: &vector&lt;u8&gt;, salt: &vector&lt;u8&gt;): vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_compute_commitment">compute_commitment</a>(
    case_id: ID,
    round: u64,
    agent_id: ID,
    outcome: u8,
    rationale_digest: &vector&lt;u8&gt;,
    evidence_refs: &vector&lt;u64&gt;,
    bundle_digest: &vector&lt;u8&gt;,
    salt: &vector&lt;u8&gt;,
): vector&lt;u8&gt; {
    <b>let</b> <b>mut</b> bytes = vector::empty&lt;u8&gt;();
    vector::append(&<b>mut</b> bytes, bcs::to_bytes(&case_id));
    vector::append(&<b>mut</b> bytes, bcs::to_bytes(&round));
    vector::append(&<b>mut</b> bytes, bcs::to_bytes(&agent_id));
    vector::append(&<b>mut</b> bytes, bcs::to_bytes(&outcome));
    vector::append(&<b>mut</b> bytes, *rationale_digest);
    vector::append(&<b>mut</b> bytes, bcs::to_bytes(evidence_refs));
    vector::append(&<b>mut</b> bytes, *bundle_digest);
    vector::append(&<b>mut</b> bytes, *salt);
    hash::sha3_256(bytes)
}
</code></pre>



</details>

<a name="social_contracts_jury_finalize_round"></a>

## Function `finalize_round`

Closes the voting round once the reveal window is over (or every seat revealed), pays duty
score for participation, and either finalizes, opens an appeal window, escalates to a larger
jury, or fails the case.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_finalize_round">finalize_round</a>(config: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">social_contracts::jury::JuryConfig</a>, roster: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JurorRoster">social_contracts::jury::JurorRoster</a>, case: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">social_contracts::jury::JuryCase</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_finalize_round">finalize_round</a>(
    config: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">JuryConfig</a>,
    roster: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JurorRoster">JurorRoster</a>,
    case: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">JuryCase</a>,
    clock: &Clock,
) {
    <a href="../social_contracts/jury.md#social_contracts_jury_assert_config_live">assert_config_live</a>(config);
    <b>assert</b>!(roster.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/jury.md#social_contracts_jury_EWrongVersion">EWrongVersion</a>);
    <b>assert</b>!(case.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/jury.md#social_contracts_jury_EWrongVersion">EWrongVersion</a>);
    <b>assert</b>!(case.state == <a href="../social_contracts/jury.md#social_contracts_jury_CASE_VOTING">CASE_VOTING</a>, <a href="../social_contracts/jury.md#social_contracts_jury_EWrongCaseState">EWrongCaseState</a>);
    <b>let</b> now = clock::timestamp_ms(clock);
    <b>let</b> round = case.round;
    {
        <b>let</b> r = vector::borrow(&case.rounds, round);
        <b>assert</b>!(now &gt; r.reveal_deadline_ms || <a href="../social_contracts/jury.md#social_contracts_jury_all_revealed">all_revealed</a>(case, round), <a href="../social_contracts/jury.md#social_contracts_jury_ERoundNotOver">ERoundNotOver</a>);
    };
    <a href="../social_contracts/jury.md#social_contracts_jury_close_round">close_round</a>(config, roster, case, now, <b>true</b>);
    <b>let</b> (decisive, leading) = <a href="../social_contracts/jury.md#social_contracts_jury_evaluate_round">evaluate_round</a>(case, round);
    <b>let</b> max_rounds = vector::length(&case.round_sizes);
    <b>let</b> appeal_deadline_ms = <b>if</b> (decisive && case.appealable && round + 1 &lt; max_rounds) {
        <a href="../social_contracts/memory.md#social_contracts_memory_checked_add_u64">memory::checked_add_u64</a>(now, config.timing.<a href="../social_contracts/jury.md#social_contracts_jury_appeal_window_ms">appeal_window_ms</a>)
    } <b>else</b> {
        0
    };
    {
        <b>let</b> r = vector::borrow(&case.rounds, round);
        event::emit(<a href="../social_contracts/jury.md#social_contracts_jury_RoundFinalized">RoundFinalized</a> {
            case_id: object::id(case),
            round,
            tally: r.tally,
            <a href="../social_contracts/jury.md#social_contracts_jury_abstain">abstain</a>: r.<a href="../social_contracts/jury.md#social_contracts_jury_abstain">abstain</a>,
            revealed: r.revealed,
            decisive,
            leading: <b>if</b> (decisive) { option::some(leading) } <b>else</b> { option::none() },
            appeal_deadline_ms,
        });
    };
    <b>if</b> (decisive) {
        case.provisional_outcome = option::some(leading);
        <b>if</b> (case.appealable && round + 1 &lt; max_rounds) {
            case.state = <a href="../social_contracts/jury.md#social_contracts_jury_CASE_TALLIED">CASE_TALLIED</a>;
            case.appeal_deadline_ms = <a href="../social_contracts/memory.md#social_contracts_memory_checked_add_u64">memory::checked_add_u64</a>(now, config.timing.<a href="../social_contracts/jury.md#social_contracts_jury_appeal_window_ms">appeal_window_ms</a>);
        } <b>else</b> {
            <a href="../social_contracts/jury.md#social_contracts_jury_finalize_outcome">finalize_outcome</a>(case, leading, now);
        };
    } <b>else</b> <b>if</b> (round + 1 &lt; max_rounds) {
        case.round = round + 1;
        case.state = <a href="../social_contracts/jury.md#social_contracts_jury_CASE_AWAITING_DRAW">CASE_AWAITING_DRAW</a>;
    } <b>else</b> {
        <a href="../social_contracts/jury.md#social_contracts_jury_end_failed">end_failed</a>(case, <a href="../social_contracts/jury.md#social_contracts_jury_FAIL_CANNOT_DECIDE">FAIL_CANNOT_DECIDE</a>, now);
    };
}
</code></pre>



</details>

<a name="social_contracts_jury_all_revealed"></a>

## Function `all_revealed`



<pre><code><b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_all_revealed">all_revealed</a>(case: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">social_contracts::jury::JuryCase</a>, round: u64): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_all_revealed">all_revealed</a>(case: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">JuryCase</a>, round: u64): bool {
    <b>let</b> r = vector::borrow(&case.rounds, round);
    <b>if</b> (vector::is_empty(&r.jurors)) {
        <b>return</b> <b>false</b>
    };
    <b>let</b> <b>mut</b> i = 0;
    <b>let</b> len = vector::length(&r.jurors);
    <b>while</b> (i &lt; len) {
        <b>let</b> id = *vector::borrow(&r.jurors, i);
        <b>if</b> (table::borrow(&case.slots, id).status != <a href="../social_contracts/jury.md#social_contracts_jury_SLOT_REVEALED">SLOT_REVEALED</a>) {
            <b>return</b> <b>false</b>
        };
        i = i + 1;
    };
    <b>true</b>
}
</code></pre>



</details>

<a name="social_contracts_jury_evaluate_round"></a>

## Function `evaluate_round`

A round is decisive when enough jurors revealed, and the leading option strictly leads and holds
at least <code>approval_bps</code> of the non-abstain reveals. Returns the leading option when decisive.


<pre><code><b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_evaluate_round">evaluate_round</a>(case: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">social_contracts::jury::JuryCase</a>, round: u64): (bool, u8)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_evaluate_round">evaluate_round</a>(case: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">JuryCase</a>, round: u64): (bool, u8) {
    <b>let</b> r = vector::borrow(&case.rounds, round);
    <b>if</b> (r.revealed &lt; case.quorum) {
        <b>return</b> (<b>false</b>, <a href="../social_contracts/jury.md#social_contracts_jury_ABSTAIN">ABSTAIN</a>)
    };
    <b>let</b> non_abstain = r.revealed - r.<a href="../social_contracts/jury.md#social_contracts_jury_abstain">abstain</a>;
    <b>if</b> (non_abstain == 0) {
        <b>return</b> (<b>false</b>, <a href="../social_contracts/jury.md#social_contracts_jury_ABSTAIN">ABSTAIN</a>)
    };
    <b>let</b> <b>mut</b> lead = 0;
    <b>let</b> <b>mut</b> lead_index = 0;
    <b>let</b> <b>mut</b> second = 0;
    <b>let</b> <b>mut</b> i = 0;
    <b>let</b> len = vector::length(&r.tally);
    <b>while</b> (i &lt; len) {
        <b>let</b> count = *vector::borrow(&r.tally, i);
        <b>if</b> (count &gt; lead) {
            second = lead;
            lead = count;
            lead_index = i;
        } <b>else</b> <b>if</b> (count &gt; second) {
            second = count;
        };
        i = i + 1;
    };
    <b>let</b> strong_enough = <a href="../social_contracts/memory.md#social_contracts_memory_checked_mul_u64">memory::checked_mul_u64</a>(lead, <a href="../social_contracts/jury.md#social_contracts_jury_BPS_DENOMINATOR">BPS_DENOMINATOR</a>)
        &gt;= <a href="../social_contracts/memory.md#social_contracts_memory_checked_mul_u64">memory::checked_mul_u64</a>(case.approval_bps, non_abstain);
    <b>if</b> (lead &gt; second && strong_enough) {
        (<b>true</b>, lead_index <b>as</b> u8)
    } <b>else</b> {
        (<b>false</b>, <a href="../social_contracts/jury.md#social_contracts_jury_ABSTAIN">ABSTAIN</a>)
    }
}
</code></pre>



</details>

<a name="social_contracts_jury_finalize_case"></a>

## Function `finalize_case`

Anyone, after the appeal window, finalizes a decisive round; or, once the case has outlived
<code>max_case_duration</code>, fails it (or, in an appeal round, lets the first verdict stand).


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_finalize_case">finalize_case</a>(config: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">social_contracts::jury::JuryConfig</a>, roster: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JurorRoster">social_contracts::jury::JurorRoster</a>, case: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">social_contracts::jury::JuryCase</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_finalize_case">finalize_case</a>(
    config: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">JuryConfig</a>,
    roster: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JurorRoster">JurorRoster</a>,
    case: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">JuryCase</a>,
    clock: &Clock,
) {
    <a href="../social_contracts/jury.md#social_contracts_jury_assert_config_live">assert_config_live</a>(config);
    <b>assert</b>!(roster.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/jury.md#social_contracts_jury_EWrongVersion">EWrongVersion</a>);
    <b>assert</b>!(case.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/jury.md#social_contracts_jury_EWrongVersion">EWrongVersion</a>);
    <b>let</b> now = clock::timestamp_ms(clock);
    <b>if</b> (case.state == <a href="../social_contracts/jury.md#social_contracts_jury_CASE_TALLIED">CASE_TALLIED</a>) {
        <b>assert</b>!(now &gt; case.appeal_deadline_ms, <a href="../social_contracts/jury.md#social_contracts_jury_ENotYet">ENotYet</a>);
        <b>let</b> outcome = *option::borrow(&case.provisional_outcome);
        <a href="../social_contracts/jury.md#social_contracts_jury_finalize_outcome">finalize_outcome</a>(case, outcome, now);
        <b>return</b>
    };
    <b>assert</b>!(
        case.state == <a href="../social_contracts/jury.md#social_contracts_jury_CASE_EVIDENCE">CASE_EVIDENCE</a> || case.state == <a href="../social_contracts/jury.md#social_contracts_jury_CASE_AWAITING_DRAW">CASE_AWAITING_DRAW</a>
            || case.state == <a href="../social_contracts/jury.md#social_contracts_jury_CASE_SELECTING">CASE_SELECTING</a> || case.state == <a href="../social_contracts/jury.md#social_contracts_jury_CASE_VOTING">CASE_VOTING</a>,
        <a href="../social_contracts/jury.md#social_contracts_jury_EWrongCaseState">EWrongCaseState</a>,
    );
    <b>let</b> deadline = <a href="../social_contracts/memory.md#social_contracts_memory_checked_add_u64">memory::checked_add_u64</a>(case.created_at_ms, config.timing.<a href="../social_contracts/jury.md#social_contracts_jury_max_case_duration_ms">max_case_duration_ms</a>);
    <b>assert</b>!(now &gt; deadline, <a href="../social_contracts/jury.md#social_contracts_jury_ENotYet">ENotYet</a>);
    <a href="../social_contracts/jury.md#social_contracts_jury_close_round">close_round</a>(config, roster, case, now, <b>true</b>);
    <a href="../social_contracts/jury.md#social_contracts_jury_end_failed">end_failed</a>(case, <a href="../social_contracts/jury.md#social_contracts_jury_FAIL_TIMEOUT">FAIL_TIMEOUT</a>, now);
}
</code></pre>



</details>

<a name="social_contracts_jury_void_case"></a>

## Function `void_case`

Emergency: ends an unfinished case. It can only fail the case (or leave the first verdict
standing in an appeal round); it never chooses an outcome.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_void_case">void_case</a>(_: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryAdminCap">social_contracts::jury::JuryAdminCap</a>, config: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">social_contracts::jury::JuryConfig</a>, roster: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JurorRoster">social_contracts::jury::JurorRoster</a>, case: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">social_contracts::jury::JuryCase</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_void_case">void_case</a>(
    _: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryAdminCap">JuryAdminCap</a>,
    config: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">JuryConfig</a>,
    roster: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JurorRoster">JurorRoster</a>,
    case: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">JuryCase</a>,
    clock: &Clock,
) {
    <a href="../social_contracts/jury.md#social_contracts_jury_assert_config_live">assert_config_live</a>(config);
    <b>assert</b>!(roster.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/jury.md#social_contracts_jury_EWrongVersion">EWrongVersion</a>);
    <b>assert</b>!(case.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/jury.md#social_contracts_jury_EWrongVersion">EWrongVersion</a>);
    <a href="../social_contracts/jury.md#social_contracts_jury_force_end">force_end</a>(config, roster, case, clock::timestamp_ms(clock));
}
</code></pre>



</details>

<a name="social_contracts_jury_end_case_by_settlement"></a>

## Function `end_case_by_settlement`

An integrator ends an unfinished case because its subject was resolved some other way (the
parties agreed). Like [<code><a href="../social_contracts/jury.md#social_contracts_jury_void_case">void_case</a></code>], it fails the case without an outcome and frees every seat
without a penalty; a tallied verdict simply becomes final.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_end_case_by_settlement">end_case_by_settlement</a>(config: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">social_contracts::jury::JuryConfig</a>, roster: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JurorRoster">social_contracts::jury::JurorRoster</a>, case: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">social_contracts::jury::JuryCase</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_end_case_by_settlement">end_case_by_settlement</a>(
    config: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">JuryConfig</a>,
    roster: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JurorRoster">JurorRoster</a>,
    case: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">JuryCase</a>,
    clock: &Clock,
) {
    <a href="../social_contracts/jury.md#social_contracts_jury_assert_config_live">assert_config_live</a>(config);
    <b>assert</b>!(roster.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/jury.md#social_contracts_jury_EWrongVersion">EWrongVersion</a>);
    <b>assert</b>!(case.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/jury.md#social_contracts_jury_EWrongVersion">EWrongVersion</a>);
    <a href="../social_contracts/jury.md#social_contracts_jury_force_end">force_end</a>(config, roster, case, clock::timestamp_ms(clock));
}
</code></pre>



</details>

<a name="social_contracts_jury_force_end"></a>

## Function `force_end`



<pre><code><b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_force_end">force_end</a>(config: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">social_contracts::jury::JuryConfig</a>, roster: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JurorRoster">social_contracts::jury::JurorRoster</a>, case: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">social_contracts::jury::JuryCase</a>, now: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_force_end">force_end</a>(config: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">JuryConfig</a>, roster: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JurorRoster">JurorRoster</a>, case: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">JuryCase</a>, now: u64) {
    <b>assert</b>!(!<a href="../social_contracts/jury.md#social_contracts_jury_is_finished">is_finished</a>(case.state), <a href="../social_contracts/jury.md#social_contracts_jury_EWrongCaseState">EWrongCaseState</a>);
    <b>if</b> (case.state == <a href="../social_contracts/jury.md#social_contracts_jury_CASE_TALLIED">CASE_TALLIED</a>) {
        <b>let</b> outcome = *option::borrow(&case.provisional_outcome);
        <a href="../social_contracts/jury.md#social_contracts_jury_finalize_outcome">finalize_outcome</a>(case, outcome, now);
        <b>return</b>
    };
    <a href="../social_contracts/jury.md#social_contracts_jury_close_round">close_round</a>(config, roster, case, now, <b>false</b>);
    <a href="../social_contracts/jury.md#social_contracts_jury_end_failed">end_failed</a>(case, <a href="../social_contracts/jury.md#social_contracts_jury_FAIL_VOIDED">FAIL_VOIDED</a>, now);
}
</code></pre>



</details>

<a name="social_contracts_jury_finalize_outcome"></a>

## Function `finalize_outcome`



<pre><code><b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_finalize_outcome">finalize_outcome</a>(case: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">social_contracts::jury::JuryCase</a>, outcome: u8, now: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_finalize_outcome">finalize_outcome</a>(case: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">JuryCase</a>, outcome: u8, now: u64) {
    case.state = <a href="../social_contracts/jury.md#social_contracts_jury_CASE_FINAL">CASE_FINAL</a>;
    case.final_outcome = option::some(outcome);
    <a href="../social_contracts/jury.md#social_contracts_jury_settle_fees">settle_fees</a>(case);
    event::emit(<a href="../social_contracts/jury.md#social_contracts_jury_CaseFinalized">CaseFinalized</a> {
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
</code></pre>



</details>

<a name="social_contracts_jury_end_failed"></a>

## Function `end_failed`

A failed appeal round leaves the first verdict standing; any other failure ends the case
without an outcome and the integrator applies its failure path.


<pre><code><b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_end_failed">end_failed</a>(case: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">social_contracts::jury::JuryCase</a>, reason: u8, now: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_end_failed">end_failed</a>(case: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">JuryCase</a>, reason: u8, now: u64) {
    <b>if</b> (option::is_some(&case.prior_outcome)) {
        <b>let</b> prior = *option::borrow(&case.prior_outcome);
        case.failed_appeal = <b>true</b>;
        <a href="../social_contracts/jury.md#social_contracts_jury_finalize_outcome">finalize_outcome</a>(case, prior, now);
    } <b>else</b> {
        case.state = <a href="../social_contracts/jury.md#social_contracts_jury_CASE_FAILED">CASE_FAILED</a>;
        case.fail_reason = reason;
        <a href="../social_contracts/jury.md#social_contracts_jury_settle_fees">settle_fees</a>(case);
        event::emit(<a href="../social_contracts/jury.md#social_contracts_jury_CaseFailed">CaseFailed</a> {
            case_id: object::id(case),
            reason,
            round: case.round,
            participants: case.fees.participants,
            per_juror_fee: case.fees.per_juror_fee,
            failed_at_ms: now,
        });
    };
}
</code></pre>



</details>

<a name="social_contracts_jury_close_round"></a>

## Function `close_round`

Settles every seat of the current round once: revealed jurors are credited, jurors who never
accepted (or accepted into a round that ran and never committed or revealed) are penalized,
and everyone is released. A seat that accepted into a round that never started voting is not
penalized: the failure was not the juror's. <code>penalize = <b>false</b></code> (an admin void) releases every
seat without a penalty.


<pre><code><b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_close_round">close_round</a>(config: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">social_contracts::jury::JuryConfig</a>, roster: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JurorRoster">social_contracts::jury::JurorRoster</a>, case: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">social_contracts::jury::JuryCase</a>, now: u64, penalize: bool)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_close_round">close_round</a>(config: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">JuryConfig</a>, roster: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JurorRoster">JurorRoster</a>, case: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">JuryCase</a>, now: u64, penalize: bool) {
    <b>let</b> round = case.round;
    <b>if</b> (vector::length(&case.rounds) &lt;= round) {
        <b>return</b>
    };
    <b>if</b> (vector::borrow(&case.rounds, round).closed) {
        <b>return</b>
    };
    <b>let</b> voting = case.state == <a href="../social_contracts/jury.md#social_contracts_jury_CASE_VOTING">CASE_VOTING</a>;
    <b>let</b> seats = vector::borrow(&case.rounds, round).jurors;
    <b>let</b> case_id = object::id(case);
    <b>let</b> <b>mut</b> i = 0;
    <b>let</b> len = vector::length(&seats);
    <b>while</b> (i &lt; len) {
        <b>let</b> agent_id = *vector::borrow(&seats, i);
        <b>let</b> status = table::borrow(&case.slots, agent_id).status;
        <b>if</b> (status == <a href="../social_contracts/jury.md#social_contracts_jury_SLOT_REVEALED">SLOT_REVEALED</a>) {
            <a href="../social_contracts/jury.md#social_contracts_jury_reward_duty">reward_duty</a>(config, roster, agent_id, case_id, now);
        } <b>else</b> <b>if</b> (penalize && status == <a href="../social_contracts/jury.md#social_contracts_jury_SLOT_DRAFTED">SLOT_DRAFTED</a>) {
            table::borrow_mut(&<b>mut</b> case.slots, agent_id).status = <a href="../social_contracts/jury.md#social_contracts_jury_SLOT_DEFAULTED">SLOT_DEFAULTED</a>;
            <a href="../social_contracts/jury.md#social_contracts_jury_penalize_duty">penalize_duty</a>(config, roster, agent_id, config.duty.loss_accept, <a href="../social_contracts/jury.md#social_contracts_jury_DUTY_NO_ACCEPT">DUTY_NO_ACCEPT</a>, case_id, now);
        } <b>else</b> <b>if</b> (penalize && voting && (status == <a href="../social_contracts/jury.md#social_contracts_jury_SLOT_ACCEPTED">SLOT_ACCEPTED</a> || status == <a href="../social_contracts/jury.md#social_contracts_jury_SLOT_COMMITTED">SLOT_COMMITTED</a>)) {
            table::borrow_mut(&<b>mut</b> case.slots, agent_id).status = <a href="../social_contracts/jury.md#social_contracts_jury_SLOT_DEFAULTED">SLOT_DEFAULTED</a>;
            <a href="../social_contracts/jury.md#social_contracts_jury_penalize_duty">penalize_duty</a>(config, roster, agent_id, config.duty.loss_noreveal, <a href="../social_contracts/jury.md#social_contracts_jury_DUTY_NO_REVEAL">DUTY_NO_REVEAL</a>, case_id, now);
        };
        <a href="../social_contracts/jury.md#social_contracts_jury_release_seat">release_seat</a>(roster, agent_id);
        i = i + 1;
    };
    vector::borrow_mut(&<b>mut</b> case.rounds, round).closed = <b>true</b>;
}
</code></pre>



</details>

<a name="social_contracts_jury_reward_duty"></a>

## Function `reward_duty`



<pre><code><b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_reward_duty">reward_duty</a>(config: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">social_contracts::jury::JuryConfig</a>, roster: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JurorRoster">social_contracts::jury::JurorRoster</a>, agent_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, case_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, now: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_reward_duty">reward_duty</a>(config: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">JuryConfig</a>, roster: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JurorRoster">JurorRoster</a>, agent_id: ID, case_id: ID, now: u64) {
    <b>if</b> (!table::contains(&roster.entries, agent_id)) {
        <b>return</b>
    };
    <b>let</b> <b>entry</b> = table::borrow_mut(&<b>mut</b> roster.entries, agent_id);
    <b>entry</b>.completed = <b>entry</b>.completed + 1;
    <b>entry</b>.last_duty_ms = now;
    <b>let</b> raised = <b>entry</b>.duty_score + config.duty.gain;
    <b>entry</b>.duty_score = <b>if</b> (raised &gt; config.duty.max_score) { config.duty.max_score } <b>else</b> { raised };
    <b>if</b> (<b>entry</b>.strikes &gt; 0) {
        <b>entry</b>.strikes = <b>entry</b>.strikes - 1;
    };
    <b>if</b> (<b>entry</b>.status == <a href="../social_contracts/jury.md#social_contracts_jury_ROSTER_SUSPENDED">ROSTER_SUSPENDED</a> && now &gt;= <b>entry</b>.suspended_until_ms) {
        <b>entry</b>.status = <a href="../social_contracts/jury.md#social_contracts_jury_ROSTER_ACTIVE">ROSTER_ACTIVE</a>;
    };
    event::emit(<a href="../social_contracts/jury.md#social_contracts_jury_DutyScoreChanged">DutyScoreChanged</a> {
        agent_id,
        score: <b>entry</b>.duty_score,
        strikes: <b>entry</b>.strikes,
        reason: <a href="../social_contracts/jury.md#social_contracts_jury_DUTY_COMPLETED">DUTY_COMPLETED</a>,
        case_id,
    });
}
</code></pre>



</details>

<a name="social_contracts_jury_penalize_duty"></a>

## Function `penalize_duty`



<pre><code><b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_penalize_duty">penalize_duty</a>(config: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">social_contracts::jury::JuryConfig</a>, roster: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JurorRoster">social_contracts::jury::JurorRoster</a>, agent_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, loss: u64, reason: u8, case_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, now: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_penalize_duty">penalize_duty</a>(
    config: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">JuryConfig</a>,
    roster: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JurorRoster">JurorRoster</a>,
    agent_id: ID,
    loss: u64,
    reason: u8,
    case_id: ID,
    now: u64,
) {
    <b>if</b> (!table::contains(&roster.entries, agent_id)) {
        <b>return</b>
    };
    <b>let</b> <b>entry</b> = table::borrow_mut(&<b>mut</b> roster.entries, agent_id);
    <b>entry</b>.no_shows = <b>entry</b>.no_shows + 1;
    <b>entry</b>.strikes = <b>entry</b>.strikes + 1;
    <b>entry</b>.last_duty_ms = now;
    <b>entry</b>.duty_score = <b>if</b> (<b>entry</b>.duty_score &gt; loss) { <b>entry</b>.duty_score - loss } <b>else</b> { 0 };
    <b>if</b> (<b>entry</b>.strikes &gt;= config.duty.max_strikes) {
        <b>let</b> until = <a href="../social_contracts/memory.md#social_contracts_memory_checked_add_u64">memory::checked_add_u64</a>(now, config.duty.suspension_ms);
        <b>entry</b>.status = <a href="../social_contracts/jury.md#social_contracts_jury_ROSTER_SUSPENDED">ROSTER_SUSPENDED</a>;
        <b>entry</b>.suspended_until_ms = until;
        <b>entry</b>.strikes = 0;
        event::emit(<a href="../social_contracts/jury.md#social_contracts_jury_RosterSuspended">RosterSuspended</a> { agent_id, until_ms: until });
    };
    event::emit(<a href="../social_contracts/jury.md#social_contracts_jury_DutyScoreChanged">DutyScoreChanged</a> {
        agent_id,
        score: <b>entry</b>.duty_score,
        strikes: <b>entry</b>.strikes,
        reason,
        case_id,
    });
}
</code></pre>



</details>

<a name="social_contracts_jury_release_seat"></a>

## Function `release_seat`



<pre><code><b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_release_seat">release_seat</a>(roster: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JurorRoster">social_contracts::jury::JurorRoster</a>, agent_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_release_seat">release_seat</a>(roster: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JurorRoster">JurorRoster</a>, agent_id: ID) {
    <b>if</b> (!table::contains(&roster.entries, agent_id)) {
        <b>return</b>
    };
    <b>let</b> <b>entry</b> = table::borrow_mut(&<b>mut</b> roster.entries, agent_id);
    <b>if</b> (<b>entry</b>.active_cases &gt; 0) {
        <b>entry</b>.active_cases = <b>entry</b>.active_cases - 1;
    };
}
</code></pre>



</details>

<a name="social_contracts_jury_settle_fees"></a>

## Function `settle_fees`

Fixes each participant's share: an equal split of the whole pool among the jurors who revealed,
optionally capped. What is not reserved for jurors can be swept by the integrator.


<pre><code><b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_settle_fees">settle_fees</a>(case: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">social_contracts::jury::JuryCase</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_settle_fees">settle_fees</a>(case: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">JuryCase</a>) {
    <b>if</b> (case.fees.settled) {
        <b>return</b>
    };
    case.fees.settled = <b>true</b>;
    <b>let</b> participants = case.fees.participants;
    <b>if</b> (participants == 0) {
        <b>return</b>
    };
    <b>let</b> net = balance::value(&case.fees.pool);
    <b>let</b> <b>mut</b> share = net / participants;
    <b>if</b> (case.fees.max_juror_fee &gt; 0 && share &gt; case.fees.max_juror_fee) {
        share = case.fees.max_juror_fee;
    };
    case.fees.per_juror_fee = share;
    case.fees.reserved = <a href="../social_contracts/memory.md#social_contracts_memory_checked_mul_u64">memory::checked_mul_u64</a>(share, participants);
}
</code></pre>



</details>

<a name="social_contracts_jury_claim_juror_fee"></a>

## Function `claim_juror_fee`

Anyone triggers a participant's payment; the money goes to the payout address on the roster.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_claim_juror_fee">claim_juror_fee</a>(case: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">social_contracts::jury::JuryCase</a>, agent_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_claim_juror_fee">claim_juror_fee</a>(case: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">JuryCase</a>, agent_id: ID, ctx: &<b>mut</b> TxContext) {
    <b>assert</b>!(case.fees.settled, <a href="../social_contracts/jury.md#social_contracts_jury_EFeesNotSettled">EFeesNotSettled</a>);
    <b>assert</b>!(table::contains(&case.slots, agent_id), <a href="../social_contracts/jury.md#social_contracts_jury_ENotDrafted">ENotDrafted</a>);
    <b>let</b> (recipient, amount) = {
        <b>let</b> slot = table::borrow_mut(&<b>mut</b> case.slots, agent_id);
        <b>assert</b>!(slot.status == <a href="../social_contracts/jury.md#social_contracts_jury_SLOT_REVEALED">SLOT_REVEALED</a>, <a href="../social_contracts/jury.md#social_contracts_jury_EWrongSlotStatus">EWrongSlotStatus</a>);
        <b>assert</b>!(!slot.claimed, <a href="../social_contracts/jury.md#social_contracts_jury_EAlreadyClaimed">EAlreadyClaimed</a>);
        slot.claimed = <b>true</b>;
        (slot.payout_address, case.fees.per_juror_fee)
    };
    <b>if</b> (amount &gt; 0) {
        case.fees.reserved = case.fees.reserved - amount;
        <b>let</b> payment = coin::from_balance(balance::split(&<b>mut</b> case.fees.pool, amount), ctx);
        transfer::public_transfer(payment, recipient);
    };
    event::emit(<a href="../social_contracts/jury.md#social_contracts_jury_JurorPaid">JurorPaid</a> { case_id: object::id(case), agent_id, recipient, amount_mist: amount });
}
</code></pre>



</details>

<a name="social_contracts_jury_assert_live"></a>

## Function `assert_live`



<pre><code><b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_assert_live">assert_live</a>(config: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">social_contracts::jury::JuryConfig</a>, roster: &<a href="../social_contracts/jury.md#social_contracts_jury_JurorRoster">social_contracts::jury::JurorRoster</a>, case: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">social_contracts::jury::JuryCase</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_assert_live">assert_live</a>(config: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">JuryConfig</a>, roster: &<a href="../social_contracts/jury.md#social_contracts_jury_JurorRoster">JurorRoster</a>, case: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">JuryCase</a>) {
    <a href="../social_contracts/jury.md#social_contracts_jury_assert_config_live">assert_config_live</a>(config);
    <b>assert</b>!(config.active, <a href="../social_contracts/jury.md#social_contracts_jury_EJuryPaused">EJuryPaused</a>);
    <b>assert</b>!(roster.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/jury.md#social_contracts_jury_EWrongVersion">EWrongVersion</a>);
    <b>assert</b>!(case.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/jury.md#social_contracts_jury_EWrongVersion">EWrongVersion</a>);
}
</code></pre>



</details>

<a name="social_contracts_jury_assert_juror_actor"></a>

## Function `assert_juror_actor`

The juror's owner, or the juror agent itself holding the jury capability. A different agent
of the same owner is rejected.


<pre><code><b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_assert_juror_actor">assert_juror_actor</a>(memory_config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_assert_juror_actor">assert_juror_actor</a>(
    memory_config: &MemoryConfig,
    account: &MemoryAccount,
    agent: &SubAgent,
    clock: &Clock,
    ctx: &TxContext,
) {
    <b>assert</b>!(<a href="../social_contracts/memory.md#social_contracts_memory_sub_agent_memory_account_id">memory::sub_agent_memory_account_id</a>(agent) == object::id(account), <a href="../social_contracts/jury.md#social_contracts_jury_EWrongAgent">EWrongAgent</a>);
    <b>let</b> sender = tx_context::sender(ctx);
    <b>if</b> (sender == <a href="../social_contracts/memory.md#social_contracts_memory_owner">memory::owner</a>(account)) {
        <a href="../social_contracts/memory.md#social_contracts_memory_resolve_human_actor">memory::resolve_human_actor</a>(account, ctx);
    } <b>else</b> {
        <b>assert</b>!(sender == <a href="../social_contracts/memory.md#social_contracts_memory_sub_agent_derived_address">memory::sub_agent_derived_address</a>(agent), <a href="../social_contracts/jury.md#social_contracts_jury_ENotJuror">ENotJuror</a>);
        <b>let</b> cap = <a href="../social_contracts/memory.md#social_contracts_memory_cap_jury">memory::cap_jury</a>();
        <b>let</b> acting = <a href="../social_contracts/memory.md#social_contracts_memory_resolve_actor_with_cap">memory::resolve_actor_with_cap</a>(
            memory_config, account, cap, option::none(), 0, clock, ctx,
        );
        <a href="../social_contracts/memory.md#social_contracts_memory_assert_direct_execution_allowed">memory::assert_direct_execution_allowed</a>(account, cap, ctx);
        <b>assert</b>!(option::contains(&<a href="../social_contracts/memory.md#social_contracts_memory_acting_sub_agent_id">memory::acting_sub_agent_id</a>(&acting), &object::id(agent)), <a href="../social_contracts/jury.md#social_contracts_jury_ENotJuror">ENotJuror</a>);
    };
}
</code></pre>



</details>

<a name="social_contracts_jury_subject_task"></a>

## Function `subject_task`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_subject_task">subject_task</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_subject_task">subject_task</a>(): u8 { <a href="../social_contracts/jury.md#social_contracts_jury_SUBJECT_TASK">SUBJECT_TASK</a> }
</code></pre>



</details>

<a name="social_contracts_jury_subject_governance"></a>

## Function `subject_governance`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_subject_governance">subject_governance</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_subject_governance">subject_governance</a>(): u8 { <a href="../social_contracts/jury.md#social_contracts_jury_SUBJECT_GOVERNANCE">SUBJECT_GOVERNANCE</a> }
</code></pre>



</details>

<a name="social_contracts_jury_abstain"></a>

## Function `abstain`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_abstain">abstain</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_abstain">abstain</a>(): u8 { <a href="../social_contracts/jury.md#social_contracts_jury_ABSTAIN">ABSTAIN</a> }
</code></pre>



</details>

<a name="social_contracts_jury_case_evidence"></a>

## Function `case_evidence`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_case_evidence">case_evidence</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_case_evidence">case_evidence</a>(): u8 { <a href="../social_contracts/jury.md#social_contracts_jury_CASE_EVIDENCE">CASE_EVIDENCE</a> }
</code></pre>



</details>

<a name="social_contracts_jury_case_awaiting_draw"></a>

## Function `case_awaiting_draw`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_case_awaiting_draw">case_awaiting_draw</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_case_awaiting_draw">case_awaiting_draw</a>(): u8 { <a href="../social_contracts/jury.md#social_contracts_jury_CASE_AWAITING_DRAW">CASE_AWAITING_DRAW</a> }
</code></pre>



</details>

<a name="social_contracts_jury_case_selecting"></a>

## Function `case_selecting`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_case_selecting">case_selecting</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_case_selecting">case_selecting</a>(): u8 { <a href="../social_contracts/jury.md#social_contracts_jury_CASE_SELECTING">CASE_SELECTING</a> }
</code></pre>



</details>

<a name="social_contracts_jury_case_voting"></a>

## Function `case_voting`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_case_voting">case_voting</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_case_voting">case_voting</a>(): u8 { <a href="../social_contracts/jury.md#social_contracts_jury_CASE_VOTING">CASE_VOTING</a> }
</code></pre>



</details>

<a name="social_contracts_jury_case_tallied"></a>

## Function `case_tallied`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_case_tallied">case_tallied</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_case_tallied">case_tallied</a>(): u8 { <a href="../social_contracts/jury.md#social_contracts_jury_CASE_TALLIED">CASE_TALLIED</a> }
</code></pre>



</details>

<a name="social_contracts_jury_case_final"></a>

## Function `case_final`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_case_final">case_final</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_case_final">case_final</a>(): u8 { <a href="../social_contracts/jury.md#social_contracts_jury_CASE_FINAL">CASE_FINAL</a> }
</code></pre>



</details>

<a name="social_contracts_jury_case_failed"></a>

## Function `case_failed`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_case_failed">case_failed</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_case_failed">case_failed</a>(): u8 { <a href="../social_contracts/jury.md#social_contracts_jury_CASE_FAILED">CASE_FAILED</a> }
</code></pre>



</details>

<a name="social_contracts_jury_case_cancelled"></a>

## Function `case_cancelled`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_case_cancelled">case_cancelled</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_case_cancelled">case_cancelled</a>(): u8 { <a href="../social_contracts/jury.md#social_contracts_jury_CASE_CANCELLED">CASE_CANCELLED</a> }
</code></pre>



</details>

<a name="social_contracts_jury_fail_cannot_form"></a>

## Function `fail_cannot_form`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_fail_cannot_form">fail_cannot_form</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_fail_cannot_form">fail_cannot_form</a>(): u8 { <a href="../social_contracts/jury.md#social_contracts_jury_FAIL_CANNOT_FORM">FAIL_CANNOT_FORM</a> }
</code></pre>



</details>

<a name="social_contracts_jury_fail_cannot_decide"></a>

## Function `fail_cannot_decide`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_fail_cannot_decide">fail_cannot_decide</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_fail_cannot_decide">fail_cannot_decide</a>(): u8 { <a href="../social_contracts/jury.md#social_contracts_jury_FAIL_CANNOT_DECIDE">FAIL_CANNOT_DECIDE</a> }
</code></pre>



</details>

<a name="social_contracts_jury_fail_timeout"></a>

## Function `fail_timeout`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_fail_timeout">fail_timeout</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_fail_timeout">fail_timeout</a>(): u8 { <a href="../social_contracts/jury.md#social_contracts_jury_FAIL_TIMEOUT">FAIL_TIMEOUT</a> }
</code></pre>



</details>

<a name="social_contracts_jury_fail_voided"></a>

## Function `fail_voided`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_fail_voided">fail_voided</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_fail_voided">fail_voided</a>(): u8 { <a href="../social_contracts/jury.md#social_contracts_jury_FAIL_VOIDED">FAIL_VOIDED</a> }
</code></pre>



</details>

<a name="social_contracts_jury_slot_drafted"></a>

## Function `slot_drafted`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_slot_drafted">slot_drafted</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_slot_drafted">slot_drafted</a>(): u8 { <a href="../social_contracts/jury.md#social_contracts_jury_SLOT_DRAFTED">SLOT_DRAFTED</a> }
</code></pre>



</details>

<a name="social_contracts_jury_slot_accepted"></a>

## Function `slot_accepted`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_slot_accepted">slot_accepted</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_slot_accepted">slot_accepted</a>(): u8 { <a href="../social_contracts/jury.md#social_contracts_jury_SLOT_ACCEPTED">SLOT_ACCEPTED</a> }
</code></pre>



</details>

<a name="social_contracts_jury_slot_committed"></a>

## Function `slot_committed`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_slot_committed">slot_committed</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_slot_committed">slot_committed</a>(): u8 { <a href="../social_contracts/jury.md#social_contracts_jury_SLOT_COMMITTED">SLOT_COMMITTED</a> }
</code></pre>



</details>

<a name="social_contracts_jury_slot_revealed"></a>

## Function `slot_revealed`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_slot_revealed">slot_revealed</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_slot_revealed">slot_revealed</a>(): u8 { <a href="../social_contracts/jury.md#social_contracts_jury_SLOT_REVEALED">SLOT_REVEALED</a> }
</code></pre>



</details>

<a name="social_contracts_jury_slot_replaced"></a>

## Function `slot_replaced`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_slot_replaced">slot_replaced</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_slot_replaced">slot_replaced</a>(): u8 { <a href="../social_contracts/jury.md#social_contracts_jury_SLOT_REPLACED">SLOT_REPLACED</a> }
</code></pre>



</details>

<a name="social_contracts_jury_slot_defaulted"></a>

## Function `slot_defaulted`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_slot_defaulted">slot_defaulted</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_slot_defaulted">slot_defaulted</a>(): u8 { <a href="../social_contracts/jury.md#social_contracts_jury_SLOT_DEFAULTED">SLOT_DEFAULTED</a> }
</code></pre>



</details>

<a name="social_contracts_jury_roster_active"></a>

## Function `roster_active`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_roster_active">roster_active</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_roster_active">roster_active</a>(): u8 { <a href="../social_contracts/jury.md#social_contracts_jury_ROSTER_ACTIVE">ROSTER_ACTIVE</a> }
</code></pre>



</details>

<a name="social_contracts_jury_roster_paused"></a>

## Function `roster_paused`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_roster_paused">roster_paused</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_roster_paused">roster_paused</a>(): u8 { <a href="../social_contracts/jury.md#social_contracts_jury_ROSTER_PAUSED">ROSTER_PAUSED</a> }
</code></pre>



</details>

<a name="social_contracts_jury_roster_suspended"></a>

## Function `roster_suspended`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_roster_suspended">roster_suspended</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_roster_suspended">roster_suspended</a>(): u8 { <a href="../social_contracts/jury.md#social_contracts_jury_ROSTER_SUSPENDED">ROSTER_SUSPENDED</a> }
</code></pre>



</details>

<a name="social_contracts_jury_config_active"></a>

## Function `config_active`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_config_active">config_active</a>(config: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">social_contracts::jury::JuryConfig</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_config_active">config_active</a>(config: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">JuryConfig</a>): bool { config.active }
</code></pre>



</details>

<a name="social_contracts_jury_max_exclusions"></a>

## Function `max_exclusions`

The most owners, organizations and agents a case may exclude; integrators trim to it.


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_max_exclusions">max_exclusions</a>(config: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">social_contracts::jury::JuryConfig</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_max_exclusions">max_exclusions</a>(config: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">JuryConfig</a>): u64 { config.selection.<a href="../social_contracts/jury.md#social_contracts_jury_max_exclusions">max_exclusions</a> }
</code></pre>



</details>

<a name="social_contracts_jury_neutral_window_ms"></a>

## Function `neutral_window_ms`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_neutral_window_ms">neutral_window_ms</a>(config: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">social_contracts::jury::JuryConfig</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_neutral_window_ms">neutral_window_ms</a>(config: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">JuryConfig</a>): u64 { config.timing.<a href="../social_contracts/jury.md#social_contracts_jury_neutral_window_ms">neutral_window_ms</a> }
</code></pre>



</details>

<a name="social_contracts_jury_accept_window_ms"></a>

## Function `accept_window_ms`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_accept_window_ms">accept_window_ms</a>(config: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">social_contracts::jury::JuryConfig</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_accept_window_ms">accept_window_ms</a>(config: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">JuryConfig</a>): u64 { config.timing.<a href="../social_contracts/jury.md#social_contracts_jury_accept_window_ms">accept_window_ms</a> }
</code></pre>



</details>

<a name="social_contracts_jury_commit_window_ms"></a>

## Function `commit_window_ms`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_commit_window_ms">commit_window_ms</a>(config: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">social_contracts::jury::JuryConfig</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_commit_window_ms">commit_window_ms</a>(config: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">JuryConfig</a>): u64 { config.timing.<a href="../social_contracts/jury.md#social_contracts_jury_commit_window_ms">commit_window_ms</a> }
</code></pre>



</details>

<a name="social_contracts_jury_reveal_window_ms"></a>

## Function `reveal_window_ms`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_reveal_window_ms">reveal_window_ms</a>(config: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">social_contracts::jury::JuryConfig</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_reveal_window_ms">reveal_window_ms</a>(config: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">JuryConfig</a>): u64 { config.timing.<a href="../social_contracts/jury.md#social_contracts_jury_reveal_window_ms">reveal_window_ms</a> }
</code></pre>



</details>

<a name="social_contracts_jury_reveal_grace_ms"></a>

## Function `reveal_grace_ms`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_reveal_grace_ms">reveal_grace_ms</a>(config: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">social_contracts::jury::JuryConfig</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_reveal_grace_ms">reveal_grace_ms</a>(config: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">JuryConfig</a>): u64 { config.timing.<a href="../social_contracts/jury.md#social_contracts_jury_reveal_grace_ms">reveal_grace_ms</a> }
</code></pre>



</details>

<a name="social_contracts_jury_appeal_window_ms"></a>

## Function `appeal_window_ms`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_appeal_window_ms">appeal_window_ms</a>(config: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">social_contracts::jury::JuryConfig</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_appeal_window_ms">appeal_window_ms</a>(config: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">JuryConfig</a>): u64 { config.timing.<a href="../social_contracts/jury.md#social_contracts_jury_appeal_window_ms">appeal_window_ms</a> }
</code></pre>



</details>

<a name="social_contracts_jury_max_case_duration_ms"></a>

## Function `max_case_duration_ms`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_max_case_duration_ms">max_case_duration_ms</a>(config: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">social_contracts::jury::JuryConfig</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_max_case_duration_ms">max_case_duration_ms</a>(config: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">JuryConfig</a>): u64 { config.timing.<a href="../social_contracts/jury.md#social_contracts_jury_max_case_duration_ms">max_case_duration_ms</a> }
</code></pre>



</details>

<a name="social_contracts_jury_min_eligible_roster"></a>

## Function `min_eligible_roster`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_min_eligible_roster">min_eligible_roster</a>(config: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">social_contracts::jury::JuryConfig</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_min_eligible_roster">min_eligible_roster</a>(config: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">JuryConfig</a>): u64 { config.eligibility.<a href="../social_contracts/jury.md#social_contracts_jury_min_eligible_roster">min_eligible_roster</a> }
</code></pre>



</details>

<a name="social_contracts_jury_initial_duty_score"></a>

## Function `initial_duty_score`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_initial_duty_score">initial_duty_score</a>(config: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">social_contracts::jury::JuryConfig</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_initial_duty_score">initial_duty_score</a>(config: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">JuryConfig</a>): u64 { config.duty.initial_score }
</code></pre>



</details>

<a name="social_contracts_jury_genesis_sunset_ms"></a>

## Function `genesis_sunset_ms`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_genesis_sunset_ms">genesis_sunset_ms</a>(config: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">social_contracts::jury::JuryConfig</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_genesis_sunset_ms">genesis_sunset_ms</a>(config: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">JuryConfig</a>): u64 { config.<a href="../social_contracts/jury.md#social_contracts_jury_genesis_sunset_ms">genesis_sunset_ms</a> }
</code></pre>



</details>

<a name="social_contracts_jury_roster_count"></a>

## Function `roster_count`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_roster_count">roster_count</a>(roster: &<a href="../social_contracts/jury.md#social_contracts_jury_JurorRoster">social_contracts::jury::JurorRoster</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_roster_count">roster_count</a>(roster: &<a href="../social_contracts/jury.md#social_contracts_jury_JurorRoster">JurorRoster</a>): u64 { roster.count }
</code></pre>



</details>

<a name="social_contracts_jury_on_roster"></a>

## Function `on_roster`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_on_roster">on_roster</a>(roster: &<a href="../social_contracts/jury.md#social_contracts_jury_JurorRoster">social_contracts::jury::JurorRoster</a>, agent_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_on_roster">on_roster</a>(roster: &<a href="../social_contracts/jury.md#social_contracts_jury_JurorRoster">JurorRoster</a>, agent_id: ID): bool { table::contains(&roster.entries, agent_id) }
</code></pre>



</details>

<a name="social_contracts_jury_entry_status"></a>

## Function `entry_status`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_entry_status">entry_status</a>(roster: &<a href="../social_contracts/jury.md#social_contracts_jury_JurorRoster">social_contracts::jury::JurorRoster</a>, agent_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_entry_status">entry_status</a>(roster: &<a href="../social_contracts/jury.md#social_contracts_jury_JurorRoster">JurorRoster</a>, agent_id: ID): u8 { table::borrow(&roster.entries, agent_id).status }
</code></pre>



</details>

<a name="social_contracts_jury_entry_duty_score"></a>

## Function `entry_duty_score`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_entry_duty_score">entry_duty_score</a>(roster: &<a href="../social_contracts/jury.md#social_contracts_jury_JurorRoster">social_contracts::jury::JurorRoster</a>, agent_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_entry_duty_score">entry_duty_score</a>(roster: &<a href="../social_contracts/jury.md#social_contracts_jury_JurorRoster">JurorRoster</a>, agent_id: ID): u64 { table::borrow(&roster.entries, agent_id).duty_score }
</code></pre>



</details>

<a name="social_contracts_jury_entry_strikes"></a>

## Function `entry_strikes`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_entry_strikes">entry_strikes</a>(roster: &<a href="../social_contracts/jury.md#social_contracts_jury_JurorRoster">social_contracts::jury::JurorRoster</a>, agent_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_entry_strikes">entry_strikes</a>(roster: &<a href="../social_contracts/jury.md#social_contracts_jury_JurorRoster">JurorRoster</a>, agent_id: ID): u64 { table::borrow(&roster.entries, agent_id).strikes }
</code></pre>



</details>

<a name="social_contracts_jury_entry_completed"></a>

## Function `entry_completed`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_entry_completed">entry_completed</a>(roster: &<a href="../social_contracts/jury.md#social_contracts_jury_JurorRoster">social_contracts::jury::JurorRoster</a>, agent_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_entry_completed">entry_completed</a>(roster: &<a href="../social_contracts/jury.md#social_contracts_jury_JurorRoster">JurorRoster</a>, agent_id: ID): u64 { table::borrow(&roster.entries, agent_id).completed }
</code></pre>



</details>

<a name="social_contracts_jury_entry_no_shows"></a>

## Function `entry_no_shows`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_entry_no_shows">entry_no_shows</a>(roster: &<a href="../social_contracts/jury.md#social_contracts_jury_JurorRoster">social_contracts::jury::JurorRoster</a>, agent_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_entry_no_shows">entry_no_shows</a>(roster: &<a href="../social_contracts/jury.md#social_contracts_jury_JurorRoster">JurorRoster</a>, agent_id: ID): u64 { table::borrow(&roster.entries, agent_id).no_shows }
</code></pre>



</details>

<a name="social_contracts_jury_entry_active_cases"></a>

## Function `entry_active_cases`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_entry_active_cases">entry_active_cases</a>(roster: &<a href="../social_contracts/jury.md#social_contracts_jury_JurorRoster">social_contracts::jury::JurorRoster</a>, agent_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_entry_active_cases">entry_active_cases</a>(roster: &<a href="../social_contracts/jury.md#social_contracts_jury_JurorRoster">JurorRoster</a>, agent_id: ID): u64 { table::borrow(&roster.entries, agent_id).active_cases }
</code></pre>



</details>

<a name="social_contracts_jury_entry_exempt"></a>

## Function `entry_exempt`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_entry_exempt">entry_exempt</a>(roster: &<a href="../social_contracts/jury.md#social_contracts_jury_JurorRoster">social_contracts::jury::JurorRoster</a>, agent_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_entry_exempt">entry_exempt</a>(roster: &<a href="../social_contracts/jury.md#social_contracts_jury_JurorRoster">JurorRoster</a>, agent_id: ID): bool { table::borrow(&roster.entries, agent_id).exempt }
</code></pre>



</details>

<a name="social_contracts_jury_entry_owner"></a>

## Function `entry_owner`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_entry_owner">entry_owner</a>(roster: &<a href="../social_contracts/jury.md#social_contracts_jury_JurorRoster">social_contracts::jury::JurorRoster</a>, agent_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_entry_owner">entry_owner</a>(roster: &<a href="../social_contracts/jury.md#social_contracts_jury_JurorRoster">JurorRoster</a>, agent_id: ID): <b>address</b> { table::borrow(&roster.entries, agent_id).owner }
</code></pre>



</details>

<a name="social_contracts_jury_entry_suspended_until_ms"></a>

## Function `entry_suspended_until_ms`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_entry_suspended_until_ms">entry_suspended_until_ms</a>(roster: &<a href="../social_contracts/jury.md#social_contracts_jury_JurorRoster">social_contracts::jury::JurorRoster</a>, agent_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_entry_suspended_until_ms">entry_suspended_until_ms</a>(roster: &<a href="../social_contracts/jury.md#social_contracts_jury_JurorRoster">JurorRoster</a>, agent_id: ID): u64 {
    table::borrow(&roster.entries, agent_id).suspended_until_ms
}
</code></pre>



</details>

<a name="social_contracts_jury_case_state"></a>

## Function `case_state`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_case_state">case_state</a>(case: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">social_contracts::jury::JuryCase</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_case_state">case_state</a>(case: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">JuryCase</a>): u8 { case.state }
</code></pre>



</details>

<a name="social_contracts_jury_case_is_finished"></a>

## Function `case_is_finished`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_case_is_finished">case_is_finished</a>(case: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">social_contracts::jury::JuryCase</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_case_is_finished">case_is_finished</a>(case: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">JuryCase</a>): bool { <a href="../social_contracts/jury.md#social_contracts_jury_is_finished">is_finished</a>(case.state) }
</code></pre>



</details>

<a name="social_contracts_jury_case_undrawn"></a>

## Function `case_undrawn`

True until the first jury is drawn: the window in which a case can still be cancelled and a
second party can still add its fee.


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_case_undrawn">case_undrawn</a>(case: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">social_contracts::jury::JuryCase</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_case_undrawn">case_undrawn</a>(case: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">JuryCase</a>): bool { case.round == 0 && vector::length(&case.rounds) == 0 }
</code></pre>



</details>

<a name="social_contracts_jury_case_subject_kind"></a>

## Function `case_subject_kind`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_case_subject_kind">case_subject_kind</a>(case: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">social_contracts::jury::JuryCase</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_case_subject_kind">case_subject_kind</a>(case: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">JuryCase</a>): u8 { case.subject_kind }
</code></pre>



</details>

<a name="social_contracts_jury_case_subject_id"></a>

## Function `case_subject_id`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_case_subject_id">case_subject_id</a>(case: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">social_contracts::jury::JuryCase</a>): <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_case_subject_id">case_subject_id</a>(case: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">JuryCase</a>): ID { case.subject_id }
</code></pre>



</details>

<a name="social_contracts_jury_case_stage"></a>

## Function `case_stage`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_case_stage">case_stage</a>(case: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">social_contracts::jury::JuryCase</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_case_stage">case_stage</a>(case: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">JuryCase</a>): u8 { case.stage }
</code></pre>



</details>

<a name="social_contracts_jury_case_round"></a>

## Function `case_round`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_case_round">case_round</a>(case: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">social_contracts::jury::JuryCase</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_case_round">case_round</a>(case: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">JuryCase</a>): u64 { case.round }
</code></pre>



</details>

<a name="social_contracts_jury_case_menu_len"></a>

## Function `case_menu_len`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_case_menu_len">case_menu_len</a>(case: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">social_contracts::jury::JuryCase</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_case_menu_len">case_menu_len</a>(case: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">JuryCase</a>): u8 { case.menu_len }
</code></pre>



</details>

<a name="social_contracts_jury_case_failure_path"></a>

## Function `case_failure_path`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_case_failure_path">case_failure_path</a>(case: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">social_contracts::jury::JuryCase</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_case_failure_path">case_failure_path</a>(case: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">JuryCase</a>): u8 { case.failure_path }
</code></pre>



</details>

<a name="social_contracts_jury_case_fail_reason"></a>

## Function `case_fail_reason`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_case_fail_reason">case_fail_reason</a>(case: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">social_contracts::jury::JuryCase</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_case_fail_reason">case_fail_reason</a>(case: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">JuryCase</a>): u8 { case.fail_reason }
</code></pre>



</details>

<a name="social_contracts_jury_case_final_outcome"></a>

## Function `case_final_outcome`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_case_final_outcome">case_final_outcome</a>(case: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">social_contracts::jury::JuryCase</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_case_final_outcome">case_final_outcome</a>(case: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">JuryCase</a>): Option&lt;u8&gt; { case.final_outcome }
</code></pre>



</details>

<a name="social_contracts_jury_case_provisional_outcome"></a>

## Function `case_provisional_outcome`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_case_provisional_outcome">case_provisional_outcome</a>(case: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">social_contracts::jury::JuryCase</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_case_provisional_outcome">case_provisional_outcome</a>(case: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">JuryCase</a>): Option&lt;u8&gt; { case.provisional_outcome }
</code></pre>



</details>

<a name="social_contracts_jury_case_prior_outcome"></a>

## Function `case_prior_outcome`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_case_prior_outcome">case_prior_outcome</a>(case: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">social_contracts::jury::JuryCase</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_case_prior_outcome">case_prior_outcome</a>(case: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">JuryCase</a>): Option&lt;u8&gt; { case.prior_outcome }
</code></pre>



</details>

<a name="social_contracts_jury_case_executed"></a>

## Function `case_executed`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_case_executed">case_executed</a>(case: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">social_contracts::jury::JuryCase</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_case_executed">case_executed</a>(case: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">JuryCase</a>): bool { case.executed }
</code></pre>



</details>

<a name="social_contracts_jury_case_failed_appeal"></a>

## Function `case_failed_appeal`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_case_failed_appeal">case_failed_appeal</a>(case: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">social_contracts::jury::JuryCase</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_case_failed_appeal">case_failed_appeal</a>(case: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">JuryCase</a>): bool { case.failed_appeal }
</code></pre>



</details>

<a name="social_contracts_jury_case_appeals_used"></a>

## Function `case_appeals_used`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_case_appeals_used">case_appeals_used</a>(case: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">social_contracts::jury::JuryCase</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_case_appeals_used">case_appeals_used</a>(case: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">JuryCase</a>): u64 { case.appeals_used }
</code></pre>



</details>

<a name="social_contracts_jury_case_appeal_deadline_ms"></a>

## Function `case_appeal_deadline_ms`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_case_appeal_deadline_ms">case_appeal_deadline_ms</a>(case: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">social_contracts::jury::JuryCase</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_case_appeal_deadline_ms">case_appeal_deadline_ms</a>(case: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">JuryCase</a>): u64 { case.appeal_deadline_ms }
</code></pre>



</details>

<a name="social_contracts_jury_case_evidence_deadline_ms"></a>

## Function `case_evidence_deadline_ms`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_case_evidence_deadline_ms">case_evidence_deadline_ms</a>(case: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">social_contracts::jury::JuryCase</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_case_evidence_deadline_ms">case_evidence_deadline_ms</a>(case: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">JuryCase</a>): u64 { case.evidence_deadline_ms }
</code></pre>



</details>

<a name="social_contracts_jury_case_evidence_count"></a>

## Function `case_evidence_count`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_case_evidence_count">case_evidence_count</a>(case: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">social_contracts::jury::JuryCase</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_case_evidence_count">case_evidence_count</a>(case: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">JuryCase</a>): u64 { vector::length(&case.evidence) }
</code></pre>



</details>

<a name="social_contracts_jury_case_participants"></a>

## Function `case_participants`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_case_participants">case_participants</a>(case: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">social_contracts::jury::JuryCase</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_case_participants">case_participants</a>(case: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">JuryCase</a>): u64 { case.fees.participants }
</code></pre>



</details>

<a name="social_contracts_jury_case_per_juror_fee"></a>

## Function `case_per_juror_fee`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_case_per_juror_fee">case_per_juror_fee</a>(case: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">social_contracts::jury::JuryCase</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_case_per_juror_fee">case_per_juror_fee</a>(case: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">JuryCase</a>): u64 { case.fees.per_juror_fee }
</code></pre>



</details>

<a name="social_contracts_jury_case_fee_pool_mist"></a>

## Function `case_fee_pool_mist`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_case_fee_pool_mist">case_fee_pool_mist</a>(case: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">social_contracts::jury::JuryCase</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_case_fee_pool_mist">case_fee_pool_mist</a>(case: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">JuryCase</a>): u64 { balance::value(&case.fees.pool) }
</code></pre>



</details>

<a name="social_contracts_jury_case_fees_settled"></a>

## Function `case_fees_settled`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_case_fees_settled">case_fees_settled</a>(case: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">social_contracts::jury::JuryCase</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_case_fees_settled">case_fees_settled</a>(case: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">JuryCase</a>): bool { case.fees.settled }
</code></pre>



</details>

<a name="social_contracts_jury_case_sweep_to"></a>

## Function `case_sweep_to`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_case_sweep_to">case_sweep_to</a>(case: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">social_contracts::jury::JuryCase</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_case_sweep_to">case_sweep_to</a>(case: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">JuryCase</a>): <b>address</b> { case.fees.sweep_to }
</code></pre>



</details>

<a name="social_contracts_jury_finalized_outcome"></a>

## Function `finalized_outcome`

The fixed outcome an integrator may act on: set only in FINAL, never for a failed or cancelled
case.


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_finalized_outcome">finalized_outcome</a>(case: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">social_contracts::jury::JuryCase</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_finalized_outcome">finalized_outcome</a>(case: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">JuryCase</a>): Option&lt;u8&gt; { case.final_outcome }
</code></pre>



</details>

<a name="social_contracts_jury_round_jurors"></a>

## Function `round_jurors`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_round_jurors">round_jurors</a>(case: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">social_contracts::jury::JuryCase</a>, round: u64): vector&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_round_jurors">round_jurors</a>(case: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">JuryCase</a>, round: u64): vector&lt;ID&gt; { vector::borrow(&case.rounds, round).jurors }
</code></pre>



</details>

<a name="social_contracts_jury_round_tally"></a>

## Function `round_tally`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_round_tally">round_tally</a>(case: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">social_contracts::jury::JuryCase</a>, round: u64): vector&lt;u64&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_round_tally">round_tally</a>(case: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">JuryCase</a>, round: u64): vector&lt;u64&gt; { vector::borrow(&case.rounds, round).tally }
</code></pre>



</details>

<a name="social_contracts_jury_round_revealed"></a>

## Function `round_revealed`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_round_revealed">round_revealed</a>(case: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">social_contracts::jury::JuryCase</a>, round: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_round_revealed">round_revealed</a>(case: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">JuryCase</a>, round: u64): u64 { vector::borrow(&case.rounds, round).revealed }
</code></pre>



</details>

<a name="social_contracts_jury_round_accept_deadline_ms"></a>

## Function `round_accept_deadline_ms`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_round_accept_deadline_ms">round_accept_deadline_ms</a>(case: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">social_contracts::jury::JuryCase</a>, round: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_round_accept_deadline_ms">round_accept_deadline_ms</a>(case: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">JuryCase</a>, round: u64): u64 {
    vector::borrow(&case.rounds, round).accept_deadline_ms
}
</code></pre>



</details>

<a name="social_contracts_jury_round_commit_deadline_ms"></a>

## Function `round_commit_deadline_ms`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_round_commit_deadline_ms">round_commit_deadline_ms</a>(case: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">social_contracts::jury::JuryCase</a>, round: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_round_commit_deadline_ms">round_commit_deadline_ms</a>(case: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">JuryCase</a>, round: u64): u64 {
    vector::borrow(&case.rounds, round).commit_deadline_ms
}
</code></pre>



</details>

<a name="social_contracts_jury_round_reveal_deadline_ms"></a>

## Function `round_reveal_deadline_ms`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_round_reveal_deadline_ms">round_reveal_deadline_ms</a>(case: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">social_contracts::jury::JuryCase</a>, round: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_round_reveal_deadline_ms">round_reveal_deadline_ms</a>(case: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">JuryCase</a>, round: u64): u64 {
    vector::borrow(&case.rounds, round).reveal_deadline_ms
}
</code></pre>



</details>

<a name="social_contracts_jury_has_slot"></a>

## Function `has_slot`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_has_slot">has_slot</a>(case: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">social_contracts::jury::JuryCase</a>, agent_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_has_slot">has_slot</a>(case: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">JuryCase</a>, agent_id: ID): bool { table::contains(&case.slots, agent_id) }
</code></pre>



</details>

<a name="social_contracts_jury_slot_status"></a>

## Function `slot_status`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_slot_status">slot_status</a>(case: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">social_contracts::jury::JuryCase</a>, agent_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_slot_status">slot_status</a>(case: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">JuryCase</a>, agent_id: ID): u8 { table::borrow(&case.slots, agent_id).status }
</code></pre>



</details>

<a name="social_contracts_jury_slot_owner"></a>

## Function `slot_owner`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_slot_owner">slot_owner</a>(case: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">social_contracts::jury::JuryCase</a>, agent_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/jury.md#social_contracts_jury_slot_owner">slot_owner</a>(case: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">JuryCase</a>, agent_id: ID): <b>address</b> { table::borrow(&case.slots, agent_id).owner }
</code></pre>



</details>
