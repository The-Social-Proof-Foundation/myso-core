---
title: Module `social_contracts::governance`
---

Governance module for the MySocial network
Manages the decentralized governance system with delegate council and community assembly
Implements proposal submission, voting, and execution processes

Delegate panel refresh runs inside [<code><a href="../social_contracts/governance.md#social_contracts_governance_try_update_delegate_panel_if_due">try_update_delegate_panel_if_due</a></code>], which is invoked from
<code><a href="../social_contracts/governance.md#social_contracts_governance_update_delegate_panel">update_delegate_panel</a></code> and piggybacked on delegate-election flows (nominations,
<code><a href="../social_contracts/governance.md#social_contracts_governance_vote_for_delegate">vote_for_delegate</a></code> / <code><a href="../social_contracts/governance.md#social_contracts_governance_clear_vote_for_delegate">clear_vote_for_delegate</a></code>), proposal submission, delegate **proposal** review
votes ([<code><a href="../social_contracts/governance.md#social_contracts_governance_run_delegate_review_vote">run_delegate_review_vote</a></code>]), and community **finalize** ([<code><a href="../social_contracts/governance.md#social_contracts_governance_finalize_community_voting_internals">finalize_community_voting_internals</a></code>]).
Community voting entries ([<code><a href="../social_contracts/governance.md#social_contracts_governance_community_vote_on_proposal">community_vote_on_proposal</a></code>]) intentionally do not trigger refresh.
A refresh applies when the epoch has reached at least one full <code>delegate_term_epochs</code> boundary and
that boundary has not been applied yet (catch-up: e.g. epoch 93 applies boundary 90 if still pending).
Refresh is lazy until one of those transactions runs while eligible; use a keeper if you need a
strict wall-clock guarantee when governance is idle.


-  [Struct `GovernanceAdminCap`](#social_contracts_governance_GovernanceAdminCap)
-  [Struct `GovernanceDAO`](#social_contracts_governance_GovernanceDAO)
-  [Struct `Delegate`](#social_contracts_governance_Delegate)
-  [Struct `NominatedDelegate`](#social_contracts_governance_NominatedDelegate)
-  [Struct `Proposal`](#social_contracts_governance_Proposal)
-  [Struct `Vote`](#social_contracts_governance_Vote)
-  [Struct `RegistryJuryPolicy`](#social_contracts_governance_RegistryJuryPolicy)
-  [Struct `JuryReview`](#social_contracts_governance_JuryReview)
-  [Struct `VotePayments`](#social_contracts_governance_VotePayments)
-  [Struct `VoteRefund`](#social_contracts_governance_VoteRefund)
-  [Struct `JuryUnavailableEvent`](#social_contracts_governance_JuryUnavailableEvent)
-  [Struct `ProposalRefundedJuryUnavailableEvent`](#social_contracts_governance_ProposalRefundedJuryUnavailableEvent)
-  [Struct `VoteRefundClaimedEvent`](#social_contracts_governance_VoteRefundClaimedEvent)
-  [Struct `VoteRefundsSweptEvent`](#social_contracts_governance_VoteRefundsSweptEvent)
-  [Struct `JuryPolicyUpdatedEvent`](#social_contracts_governance_JuryPolicyUpdatedEvent)
-  [Struct `DelegateReferredToJuryEvent`](#social_contracts_governance_DelegateReferredToJuryEvent)
-  [Struct `ProposalEscalatedToJuryEvent`](#social_contracts_governance_ProposalEscalatedToJuryEvent)
-  [Struct `CommunityVoteNeedsJuryEvent`](#social_contracts_governance_CommunityVoteNeedsJuryEvent)
-  [Struct `JuryDecisionAppliedEvent`](#social_contracts_governance_JuryDecisionAppliedEvent)
-  [Struct `ProposalRejectedByJuryEvent`](#social_contracts_governance_ProposalRejectedByJuryEvent)
-  [Struct `DelegateElectedEvent`](#social_contracts_governance_DelegateElectedEvent)
-  [Struct `DelegateNominatedEvent`](#social_contracts_governance_DelegateNominatedEvent)
-  [Struct `DelegateVotedEvent`](#social_contracts_governance_DelegateVotedEvent)
-  [Struct `DelegateVoteClearedEvent`](#social_contracts_governance_DelegateVoteClearedEvent)
-  [Struct `ProposalSubmittedEvent`](#social_contracts_governance_ProposalSubmittedEvent)
-  [Struct `DelegateVoteEvent`](#social_contracts_governance_DelegateVoteEvent)
-  [Struct `CommunityVoteEvent`](#social_contracts_governance_CommunityVoteEvent)
-  [Struct `AnonymousVoteEvent`](#social_contracts_governance_AnonymousVoteEvent)
-  [Struct `ProposalApprovedForVotingEvent`](#social_contracts_governance_ProposalApprovedForVotingEvent)
-  [Struct `ProposalRejectedEvent`](#social_contracts_governance_ProposalRejectedEvent)
-  [Struct `ProposalApprovedEvent`](#social_contracts_governance_ProposalApprovedEvent)
-  [Struct `ProposalRejectedByCommunityEvent`](#social_contracts_governance_ProposalRejectedByCommunityEvent)
-  [Struct `ProposalImplementedEvent`](#social_contracts_governance_ProposalImplementedEvent)
-  [Struct `ProposalRewardPoolForfeitedToTreasuryEvent`](#social_contracts_governance_ProposalRewardPoolForfeitedToTreasuryEvent)
-  [Struct `ProposalImplementationRewardToSubmitterEvent`](#social_contracts_governance_ProposalImplementationRewardToSubmitterEvent)
-  [Struct `VoteDecryptionFailedEvent`](#social_contracts_governance_VoteDecryptionFailedEvent)
-  [Struct `ProposalRescindedEvent`](#social_contracts_governance_ProposalRescindedEvent)
-  [Struct `GovernanceParametersUpdatedEvent`](#social_contracts_governance_GovernanceParametersUpdatedEvent)
-  [Struct `GovernanceRegistryCreatedEvent`](#social_contracts_governance_GovernanceRegistryCreatedEvent)
-  [Struct `DelegatePanelRefreshedEvent`](#social_contracts_governance_DelegatePanelRefreshedEvent)
-  [Struct `BootstrapGovernanceRegistryIds`](#social_contracts_governance_BootstrapGovernanceRegistryIds)
-  [Constants](#@Constants_0)
-  [Function `poc_governance_registry_id`](#social_contracts_governance_poc_governance_registry_id)
-  [Function `spot_governance_registry_id`](#social_contracts_governance_spot_governance_registry_id)
-  [Function `bootstrap_init`](#social_contracts_governance_bootstrap_init)
-  [Function `seed_founding_delegate`](#social_contracts_governance_seed_founding_delegate)
-  [Function `assert_has_profile`](#social_contracts_governance_assert_has_profile)
-  [Function `initialize_registry_tables`](#social_contracts_governance_initialize_registry_tables)
-  [Function `update_governance_parameters_internal`](#social_contracts_governance_update_governance_parameters_internal)
-  [Function `update_platform_governance_parameters`](#social_contracts_governance_update_platform_governance_parameters)
-  [Function `update_governance_parameters`](#social_contracts_governance_update_governance_parameters)
-  [Function `nominate_delegate`](#social_contracts_governance_nominate_delegate)
-  [Function `vote_for_delegate`](#social_contracts_governance_vote_for_delegate)
-  [Function `clear_vote_for_delegate`](#social_contracts_governance_clear_vote_for_delegate)
-  [Function `try_update_delegate_panel_if_due`](#social_contracts_governance_try_update_delegate_panel_if_due)
-  [Function `update_delegate_panel`](#social_contracts_governance_update_delegate_panel)
-  [Function `submit_proposal`](#social_contracts_governance_submit_proposal)
-  [Function `submit_ecosystem_proposal`](#social_contracts_governance_submit_ecosystem_proposal)
-  [Function `submit_proof_of_creativity_proposal`](#social_contracts_governance_submit_proof_of_creativity_proposal)
-  [Function `submit_spot_resolution_proposal`](#social_contracts_governance_submit_spot_resolution_proposal)
-  [Function `submit_spot_proposal_and_return_id`](#social_contracts_governance_submit_spot_proposal_and_return_id)
-  [Function `submit_poc_proposal_and_return_id`](#social_contracts_governance_submit_poc_proposal_and_return_id)
-  [Function `submit_proposal_internal`](#social_contracts_governance_submit_proposal_internal)
-  [Function `rescind_proposal`](#social_contracts_governance_rescind_proposal)
-  [Function `move_to_community_voting`](#social_contracts_governance_move_to_community_voting)
-  [Function `community_vote_on_proposal`](#social_contracts_governance_community_vote_on_proposal)
-  [Function `community_vote_internal`](#social_contracts_governance_community_vote_internal)
-  [Function `community_vote_anonymous`](#social_contracts_governance_community_vote_anonymous)
-  [Function `community_vote_anonymous_internal`](#social_contracts_governance_community_vote_anonymous_internal)
-  [Function `community_finalize_voting_approved_value`](#social_contracts_governance_community_finalize_voting_approved_value)
-  [Function `community_finalize_voting_reject_community_value`](#social_contracts_governance_community_finalize_voting_reject_community_value)
-  [Function `community_finalize_voting_reject_quorum_value`](#social_contracts_governance_community_finalize_voting_reject_quorum_value)
-  [Function `community_finalize_voting_needs_jury_value`](#social_contracts_governance_community_finalize_voting_needs_jury_value)
-  [Function `delegate_vote_outcome_reject_value`](#social_contracts_governance_delegate_vote_outcome_reject_value)
-  [Function `forfeit_reason_quorum_not_met_value`](#social_contracts_governance_forfeit_reason_quorum_not_met_value)
-  [Function `forfeit_reason_community_rejected_value`](#social_contracts_governance_forfeit_reason_community_rejected_value)
-  [Function `forfeit_reason_delegate_rejected_value`](#social_contracts_governance_forfeit_reason_delegate_rejected_value)
-  [Function `forfeit_reason_jury_rejected_value`](#social_contracts_governance_forfeit_reason_jury_rejected_value)
-  [Function `assert_platform_delegate_manual_reject`](#social_contracts_governance_assert_platform_delegate_manual_reject)
-  [Function `assert_shared_global_registry`](#social_contracts_governance_assert_shared_global_registry)
-  [Function `forfeit_proposal_pool_to_treasury_address`](#social_contracts_governance_forfeit_proposal_pool_to_treasury_address)
-  [Function `transition_proposal_rejected_by_delegate_council`](#social_contracts_governance_transition_proposal_rejected_by_delegate_council)
-  [Function `complete_delegate_reject_to_ecosystem_treasury`](#social_contracts_governance_complete_delegate_reject_to_ecosystem_treasury)
-  [Function `complete_delegate_reject_drain_to_platform_governance`](#social_contracts_governance_complete_delegate_reject_drain_to_platform_governance)
-  [Function `clone_bytes`](#social_contracts_governance_clone_bytes)
-  [Function `clone_option_string`](#social_contracts_governance_clone_option_string)
-  [Function `run_delegate_review_vote`](#social_contracts_governance_run_delegate_review_vote)
-  [Function `delegate_vote_on_proposal`](#social_contracts_governance_delegate_vote_on_proposal)
-  [Function `emit_implementation_reward_to_submitter_event`](#social_contracts_governance_emit_implementation_reward_to_submitter_event)
-  [Function `finalize_community_voting_internals`](#social_contracts_governance_finalize_community_voting_internals)
-  [Function `finalize_proposal`](#social_contracts_governance_finalize_proposal)
-  [Function `forfeit_proposal_pool_after_community_reject_to_platform_treasury`](#social_contracts_governance_forfeit_proposal_pool_after_community_reject_to_platform_treasury)
-  [Function `anonymous_tally_votes_and_finalize_community_internals`](#social_contracts_governance_anonymous_tally_votes_and_finalize_community_internals)
-  [Function `finalize_proposal_anonymous`](#social_contracts_governance_finalize_proposal_anonymous)
-  [Function `mark_proposal_implemented_take_pool`](#social_contracts_governance_mark_proposal_implemented_take_pool)
-  [Function `mark_proposal_implemented_take_pool_poc_oracle`](#social_contracts_governance_mark_proposal_implemented_take_pool_poc_oracle)
-  [Function `mark_proposal_implemented_to_ecosystem_treasury`](#social_contracts_governance_mark_proposal_implemented_to_ecosystem_treasury)
-  [Function `mark_proposal_implemented_platform_to_ecosystem_treasury`](#social_contracts_governance_mark_proposal_implemented_platform_to_ecosystem_treasury)
-  [Function `proposal_type_platform_value`](#social_contracts_governance_proposal_type_platform_value)
-  [Function `proposal_type_spot_value`](#social_contracts_governance_proposal_type_spot_value)
-  [Function `proposal_type_proof_of_creativity_value`](#social_contracts_governance_proposal_type_proof_of_creativity_value)
-  [Function `proposal_status`](#social_contracts_governance_proposal_status)
-  [Function `proposal_reference_id`](#social_contracts_governance_proposal_reference_id)
-  [Function `status_approved_value`](#social_contracts_governance_status_approved_value)
-  [Function `status_rejected_value`](#social_contracts_governance_status_rejected_value)
-  [Function `get_proposals_by_type`](#social_contracts_governance_get_proposals_by_type)
-  [Function `get_proposals_by_status`](#social_contracts_governance_get_proposals_by_status)
-  [Function `get_delegate_count`](#social_contracts_governance_get_delegate_count)
-  [Function `registry_type`](#social_contracts_governance_registry_type)
-  [Function `get_delegate_info`](#social_contracts_governance_get_delegate_info)
-  [Function `get_proposal_info`](#social_contracts_governance_get_proposal_info)
-  [Function `proposal_submission_cost`](#social_contracts_governance_proposal_submission_cost)
-  [Function `proposal_submitter`](#social_contracts_governance_proposal_submitter)
-  [Function `treasury_balance`](#social_contracts_governance_treasury_balance)
-  [Function `calculate_vote_cost`](#social_contracts_governance_calculate_vote_cost)
-  [Function `is_delegate`](#social_contracts_governance_is_delegate)
-  [Function `is_delegate_term_expired`](#social_contracts_governance_is_delegate_term_expired)
-  [Function `get_governance_parameters`](#social_contracts_governance_get_governance_parameters)
-  [Function `reject_proposal_manually`](#social_contracts_governance_reject_proposal_manually)
-  [Function `create_platform_governance`](#social_contracts_governance_create_platform_governance)
-  [Function `version`](#social_contracts_governance_version)
-  [Function `last_delegate_panel_boundary_epoch`](#social_contracts_governance_last_delegate_panel_boundary_epoch)
-  [Function `set_version`](#social_contracts_governance_set_version)
-  [Function `migrate_registry`](#social_contracts_governance_migrate_registry)
-  [Function `create_spot_governance_registry`](#social_contracts_governance_create_spot_governance_registry)
-  [Function `create_governance_admin_cap`](#social_contracts_governance_create_governance_admin_cap)
-  [Function `jury_state`](#social_contracts_governance_jury_state)
-  [Function `council_jury_has_jurisdiction`](#social_contracts_governance_council_jury_has_jurisdiction)
-  [Function `jury_policy_of`](#social_contracts_governance_jury_policy_of)
-  [Function `community_jury_available`](#social_contracts_governance_community_jury_available)
-  [Function `set_registry_jury_policy`](#social_contracts_governance_set_registry_jury_policy)
-  [Function `set_jury_policy_internal`](#social_contracts_governance_set_jury_policy_internal)
-  [Function `live_referrer_count`](#social_contracts_governance_live_referrer_count)
-  [Function `live_voter_count`](#social_contracts_governance_live_voter_count)
-  [Function `delegate_refer_to_jury`](#social_contracts_governance_delegate_refer_to_jury)
-  [Function `council_trigger`](#social_contracts_governance_council_trigger)
-  [Function `proposal_content_digest`](#social_contracts_governance_proposal_content_digest)
-  [Function `stakeholder_owners`](#social_contracts_governance_stakeholder_owners)
-  [Function `add_excluded_owner`](#social_contracts_governance_add_excluded_owner)
-  [Function `take_jury_fee`](#social_contracts_governance_take_jury_fee)
-  [Function `open_jury_for_proposal`](#social_contracts_governance_open_jury_for_proposal)
-  [Function `ensure_jury_review`](#social_contracts_governance_ensure_jury_review)
-  [Function `resolve_council_without_jury`](#social_contracts_governance_resolve_council_without_jury)
-  [Function `refund_community_without_jury`](#social_contracts_governance_refund_community_without_jury)
-  [Function `record_vote_payment`](#social_contracts_governance_record_vote_payment)
-  [Function `claim_vote_refund_internal`](#social_contracts_governance_claim_vote_refund_internal)
-  [Function `claim_vote_refund`](#social_contracts_governance_claim_vote_refund)
-  [Function `take_unclaimed_vote_refunds`](#social_contracts_governance_take_unclaimed_vote_refunds)
-  [Function `sweep_vote_refunds`](#social_contracts_governance_sweep_vote_refunds)
-  [Function `jury_default_forfeit_reason`](#social_contracts_governance_jury_default_forfeit_reason)
-  [Function `remove_from_bucket`](#social_contracts_governance_remove_from_bucket)
-  [Function `apply_default_community_reject`](#social_contracts_governance_apply_default_community_reject)
-  [Function `apply_jury_case`](#social_contracts_governance_apply_jury_case)
-  [Function `escalate_to_jury`](#social_contracts_governance_escalate_to_jury)
-  [Function `apply_jury_decision`](#social_contracts_governance_apply_jury_decision)
-  [Function `defer_after_unformed_case`](#social_contracts_governance_defer_after_unformed_case)
-  [Function `sweep_jury_case`](#social_contracts_governance_sweep_jury_case)
-  [Function `jury_policy_enabled`](#social_contracts_governance_jury_policy_enabled)
-  [Function `jury_council_state`](#social_contracts_governance_jury_council_state)
-  [Function `jury_community_state`](#social_contracts_governance_jury_community_state)
-  [Function `jury_stage_council_value`](#social_contracts_governance_jury_stage_council_value)
-  [Function `jury_stage_community_value`](#social_contracts_governance_jury_stage_community_value)
-  [Function `jury_action_advance_value`](#social_contracts_governance_jury_action_advance_value)
-  [Function `jury_action_approve_value`](#social_contracts_governance_jury_action_approve_value)
-  [Function `jury_action_reject_value`](#social_contracts_governance_jury_action_reject_value)
-  [Function `jury_action_deferred_value`](#social_contracts_governance_jury_action_deferred_value)
-  [Function `escalate_opened_value`](#social_contracts_governance_escalate_opened_value)
-  [Function `escalate_jury_paused_value`](#social_contracts_governance_escalate_jury_paused_value)
-  [Function `escalate_deferred_value`](#social_contracts_governance_escalate_deferred_value)
-  [Function `escalate_advanced_value`](#social_contracts_governance_escalate_advanced_value)
-  [Function `escalate_refunded_value`](#social_contracts_governance_escalate_refunded_value)
-  [Function `jury_defer_max_ms`](#social_contracts_governance_jury_defer_max_ms)
-  [Function `vote_refund_claim_window_ms`](#social_contracts_governance_vote_refund_claim_window_ms)
-  [Function `jury_deferred_since_ms`](#social_contracts_governance_jury_deferred_since_ms)
-  [Function `vote_refunded`](#social_contracts_governance_vote_refunded)
-  [Function `vote_paid_mist`](#social_contracts_governance_vote_paid_mist)
-  [Function `vote_refund_claim_deadline_ms`](#social_contracts_governance_vote_refund_claim_deadline_ms)
-  [Function `vote_refund_outstanding_mist`](#social_contracts_governance_vote_refund_outstanding_mist)
-  [Function `proposal_pool_value`](#social_contracts_governance_proposal_pool_value)


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
<b>use</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit">social_contracts::ai_credit</a>;
<b>use</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph">social_contracts::derivative_graph</a>;
<b>use</b> <a href="../social_contracts/jury.md#social_contracts_jury">social_contracts::jury</a>;
<b>use</b> <a href="../social_contracts/media_asset.md#social_contracts_license_template">social_contracts::license_template</a>;
<b>use</b> <a href="../social_contracts/media_asset.md#social_contracts_media_asset">social_contracts::media_asset</a>;
<b>use</b> <a href="../social_contracts/memory.md#social_contracts_memory">social_contracts::memory</a>;
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



<a name="social_contracts_governance_GovernanceAdminCap"></a>

## Struct `GovernanceAdminCap`

Admin capability for Governance system management


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceAdminCap">GovernanceAdminCap</a> <b>has</b> key, store
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

<a name="social_contracts_governance_GovernanceDAO"></a>

## Struct `GovernanceDAO`

Governance registry that keeps track of all delegates and proposals


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a> <b>has</b> key
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
<code><a href="../social_contracts/governance.md#social_contracts_governance_registry_type">registry_type</a>: u8</code>
</dt>
<dd>
 Registry type identifier (ecosystem, proof of creativity, platform)
</dd>
<dt>
<code>delegate_count: u64</code>
</dt>
<dd>
 Configuration parameters
</dd>
<dt>
<code>delegate_term_epochs: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/governance.md#social_contracts_governance_proposal_submission_cost">proposal_submission_cost</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_votes_per_user: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>quadratic_base_cost: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>voting_period_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>quorum_votes: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>delegates: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;<b>address</b>, <a href="../social_contracts/governance.md#social_contracts_governance_Delegate">social_contracts::governance::Delegate</a>&gt;</code>
</dt>
<dd>
 Tables and mappings
</dd>
<dt>
<code>proposals: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, bool&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>proposal_types: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>proposals_by_status: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;u8, vector&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>treasury: <a href="../myso/balance.md#myso_balance_Balance">myso::balance::Balance</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>nominated_delegates: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;<b>address</b>, <a href="../social_contracts/governance.md#social_contracts_governance_NominatedDelegate">social_contracts::governance::NominatedDelegate</a>&gt;</code>
</dt>
<dd>
 Treasury for proposal costs and rewards
</dd>
<dt>
<code>delegate_addresses: <a href="../myso/vec_set.md#myso_vec_set_VecSet">myso::vec_set::VecSet</a>&lt;<b>address</b>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>nominee_addresses: <a href="../myso/vec_set.md#myso_vec_set_VecSet">myso::vec_set::VecSet</a>&lt;<b>address</b>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>voters: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;<b>address</b>, <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;<b>address</b>, bool&gt;&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/governance.md#social_contracts_governance_version">version</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/governance.md#social_contracts_governance_last_delegate_panel_boundary_epoch">last_delegate_panel_boundary_epoch</a>: u64</code>
</dt>
<dd>
 Largest epoch boundary <code>k * delegate_term_epochs</code> for which a full panel election has run (<code>0</code> = none yet).
</dd>
</dl>


</details>

<a name="social_contracts_governance_Delegate"></a>

## Struct `Delegate`

Delegate struct representing a member of the delegate council


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/governance.md#social_contracts_governance_Delegate">Delegate</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><b>address</b>: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>upvotes: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>downvotes: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>proposals_reviewed: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>proposals_submitted: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>sided_winning_proposals: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>sided_losing_proposals: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>term_start: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>term_end: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_governance_NominatedDelegate"></a>

## Struct `NominatedDelegate`

Nominee struct representing a user nominated but not yet active


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/governance.md#social_contracts_governance_NominatedDelegate">NominatedDelegate</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><b>address</b>: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>scheduled_term_start_epoch: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>upvotes: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>downvotes: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_governance_Proposal"></a>

## Struct `Proposal`

Proposal struct representing a governance proposal


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">Proposal</a> <b>has</b> key
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
<code>title: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code>description: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code>proposal_type: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>reference_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>metadata_json: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>submitter: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>submission_time: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>delegate_approval_count: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>delegate_rejection_count: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>community_votes_for: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>community_votes_against: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>status: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>voting_start_time: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>voting_end_time: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>reward_pool: <a href="../myso/balance.md#myso_balance_Balance">myso::balance::Balance</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_governance_Vote"></a>

## Struct `Vote`

Vote struct for tracking individual votes - unused fields removed


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/governance.md#social_contracts_governance_Vote">Vote</a> <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
</dl>


</details>

<a name="social_contracts_governance_RegistryJuryPolicy"></a>

## Struct `RegistryJuryPolicy`

How the AI jury backs up the human steps of one registry. Stored as a dynamic field on the
registry, so adding it changes no object layout.


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/governance.md#social_contracts_governance_RegistryJuryPolicy">RegistryJuryPolicy</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>council_stage_enabled: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>community_stage_enabled: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>enabled_at_ms: u64</code>
</dt>
<dd>
 A proposal's council window runs from the later of its submission and this time, so
 proposals already waiting when the policy is switched on get a full window.
</dd>
<dt>
<code>council_window_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>jury_size: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>round2_size: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>jury_fee_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>jury_fee_cap_mist: u64</code>
</dt>
<dd>
 0 means no cap.
</dd>
<dt>
<code>min_pool_for_jury_mist: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>quorum: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>approval_bps_council: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>approval_bps_community: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_governance_JuryReview"></a>

## Struct `JuryReview`

One step of one proposal that the jury has taken, or may take, over.


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/governance.md#social_contracts_governance_JuryReview">JuryReview</a> <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
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
<code>trigger: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>referrers: vector&lt;<b>address</b>&gt;</code>
</dt>
<dd>
 Delegates who asked for the jury instead of voting (council step only).
</dd>
<dt>
<code>case_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>escalated_at_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>decided_at_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>decision: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>deferred_since_ms: u64</code>
</dt>
<dd>
 When this step first found no jury able to form; 0 if it never has. The 30-day wait is
 measured from here and never restarts.
</dd>
</dl>


</details>

<a name="social_contracts_governance_VotePayments"></a>

## Struct `VotePayments`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/governance.md#social_contracts_governance_VotePayments">VotePayments</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>paid: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;<b>address</b>, u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>total_mist: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_governance_VoteRefund"></a>

## Struct `VoteRefund`

A community step refunded because no jury could form. Voters pull their own share, so the
refund never loops over every voter.


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/governance.md#social_contracts_governance_VoteRefund">VoteRefund</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>refunded_at_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>claim_deadline_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>voter_pool_mist: u64</code>
</dt>
<dd>
 What the pool held for voters when the refund happened.
</dd>
<dt>
<code>paid_total_mist: u64</code>
</dt>
<dd>
 Sum of what voters paid; each claim pays <code>paid * voter_pool_mist / paid_total_mist</code>.
</dd>
<dt>
<code>outstanding_paid_mist: u64</code>
</dt>
<dd>
 Paid amounts not yet claimed. Zero means only rounding dust is left in the pool.
</dd>
<dt>
<code>claimed: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;<b>address</b>, u64&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_governance_JuryUnavailableEvent"></a>

## Struct `JuryUnavailableEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/governance.md#social_contracts_governance_JuryUnavailableEvent">JuryUnavailableEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>proposal_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>stage: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>case_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
 Set when a case opened but could not seat its jurors; its fee went back to the pool.
</dd>
<dt>
<code>formable_seats: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>required_seats: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>deferred_since_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>at_ms: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_governance_ProposalRefundedJuryUnavailableEvent"></a>

## Struct `ProposalRefundedJuryUnavailableEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/governance.md#social_contracts_governance_ProposalRefundedJuryUnavailableEvent">ProposalRefundedJuryUnavailableEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>proposal_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>submitter: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>submitter_refund_mist: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>voter_refund_pool_mist: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>claim_deadline_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>refunded_at_ms: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_governance_VoteRefundClaimedEvent"></a>

## Struct `VoteRefundClaimedEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/governance.md#social_contracts_governance_VoteRefundClaimedEvent">VoteRefundClaimedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>proposal_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>voter: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>amount_mist: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>claimed_at_ms: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_governance_VoteRefundsSweptEvent"></a>

## Struct `VoteRefundsSweptEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/governance.md#social_contracts_governance_VoteRefundsSweptEvent">VoteRefundsSweptEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>proposal_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
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
<dt>
<code>swept_at_ms: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_governance_JuryPolicyUpdatedEvent"></a>

## Struct `JuryPolicyUpdatedEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/governance.md#social_contracts_governance_JuryPolicyUpdatedEvent">JuryPolicyUpdatedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>registry_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>council_stage_enabled: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>community_stage_enabled: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>council_window_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>jury_size: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>round2_size: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>jury_fee_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>jury_fee_cap_mist: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>min_pool_for_jury_mist: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>quorum: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>approval_bps_council: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>approval_bps_community: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>updated_at_ms: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_governance_DelegateReferredToJuryEvent"></a>

## Struct `DelegateReferredToJuryEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/governance.md#social_contracts_governance_DelegateReferredToJuryEvent">DelegateReferredToJuryEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>proposal_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>delegate_address: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>reason: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>referral_count: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>majority_reached: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>referred_at_ms: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_governance_ProposalEscalatedToJuryEvent"></a>

## Struct `ProposalEscalatedToJuryEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/governance.md#social_contracts_governance_ProposalEscalatedToJuryEvent">ProposalEscalatedToJuryEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>proposal_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>stage: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>trigger: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>case_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>jury_fee_mist: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>pool_remaining_mist: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>escalated_at_ms: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_governance_CommunityVoteNeedsJuryEvent"></a>

## Struct `CommunityVoteNeedsJuryEvent`

The community vote ended with no quorum or a tie, and the registry hands that to the jury.


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/governance.md#social_contracts_governance_CommunityVoteNeedsJuryEvent">CommunityVoteNeedsJuryEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>proposal_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>trigger: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>votes_for: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>votes_against: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>quorum_votes: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>time_ms: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_governance_JuryDecisionAppliedEvent"></a>

## Struct `JuryDecisionAppliedEvent`

<code>decision</code>: 0 forward (advance or approve), 1 reject. <code>failed</code> means the jury could not
decide and the step's default applied.


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/governance.md#social_contracts_governance_JuryDecisionAppliedEvent">JuryDecisionAppliedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>proposal_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>stage: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>decision: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>failed: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>case_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>applied_at_ms: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_governance_ProposalRejectedByJuryEvent"></a>

## Struct `ProposalRejectedByJuryEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/governance.md#social_contracts_governance_ProposalRejectedByJuryEvent">ProposalRejectedByJuryEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>proposal_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>stage: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>rejection_time: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_governance_DelegateElectedEvent"></a>

## Struct `DelegateElectedEvent`

Event emitted when a new delegate is elected


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/governance.md#social_contracts_governance_DelegateElectedEvent">DelegateElectedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>delegate_address: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>term_start: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>term_end: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/governance.md#social_contracts_governance_registry_type">registry_type</a>: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>upvotes: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>downvotes: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_governance_DelegateNominatedEvent"></a>

## Struct `DelegateNominatedEvent`

Event emitted when a user is nominated to become a delegate in a future term


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/governance.md#social_contracts_governance_DelegateNominatedEvent">DelegateNominatedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>nominee_address: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>scheduled_term_start_epoch: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/governance.md#social_contracts_governance_registry_type">registry_type</a>: u8</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_governance_DelegateVotedEvent"></a>

## Struct `DelegateVotedEvent`

Event emitted when a delegate or nominee is voted for/against


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/governance.md#social_contracts_governance_DelegateVotedEvent">DelegateVotedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>target_address: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>voter: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>is_active_delegate: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>upvote: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>new_upvote_count: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>new_downvote_count: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/governance.md#social_contracts_governance_registry_type">registry_type</a>: u8</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_governance_DelegateVoteClearedEvent"></a>

## Struct `DelegateVoteClearedEvent`

Event emitted when a user clears their delegate/nominee up or down vote


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/governance.md#social_contracts_governance_DelegateVoteClearedEvent">DelegateVoteClearedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>target_address: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>voter: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>is_active_delegate: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>new_upvote_count: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>new_downvote_count: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/governance.md#social_contracts_governance_registry_type">registry_type</a>: u8</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_governance_ProposalSubmittedEvent"></a>

## Struct `ProposalSubmittedEvent`

Event emitted when a proposal is submitted


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/governance.md#social_contracts_governance_ProposalSubmittedEvent">ProposalSubmittedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>proposal_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>title: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code>description: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code>proposal_type: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>reference_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>metadata_json: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>submitter: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>reward_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>submission_time: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_governance_DelegateVoteEvent"></a>

## Struct `DelegateVoteEvent`

Event emitted when a delegate votes on a proposal


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/governance.md#social_contracts_governance_DelegateVoteEvent">DelegateVoteEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>proposal_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>delegate_address: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>approve: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>vote_time: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>reason: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_governance_CommunityVoteEvent"></a>

## Struct `CommunityVoteEvent`

Event emitted when a community member votes on a proposal


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/governance.md#social_contracts_governance_CommunityVoteEvent">CommunityVoteEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>proposal_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>voter: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>vote_weight: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>approve: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>vote_time: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>vote_cost: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_governance_AnonymousVoteEvent"></a>

## Struct `AnonymousVoteEvent`

Event emitted when an anonymous vote is submitted


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/governance.md#social_contracts_governance_AnonymousVoteEvent">AnonymousVoteEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>proposal_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>voter: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>vote_time: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>encrypted_vote_data: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_governance_ProposalApprovedForVotingEvent"></a>

## Struct `ProposalApprovedForVotingEvent`

Event emitted when a proposal is approved by the delegate council


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/governance.md#social_contracts_governance_ProposalApprovedForVotingEvent">ProposalApprovedForVotingEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>proposal_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>voting_start_time: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>voting_end_time: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_governance_ProposalRejectedEvent"></a>

## Struct `ProposalRejectedEvent`

Event emitted when a proposal is rejected by the delegate council


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/governance.md#social_contracts_governance_ProposalRejectedEvent">ProposalRejectedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>proposal_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>rejection_time: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_governance_ProposalApprovedEvent"></a>

## Struct `ProposalApprovedEvent`

Event emitted when a proposal is approved by the community


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/governance.md#social_contracts_governance_ProposalApprovedEvent">ProposalApprovedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>proposal_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>approval_time: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>votes_for: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>votes_against: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_governance_ProposalRejectedByCommunityEvent"></a>

## Struct `ProposalRejectedByCommunityEvent`

Event emitted when a proposal is rejected by the community


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/governance.md#social_contracts_governance_ProposalRejectedByCommunityEvent">ProposalRejectedByCommunityEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>proposal_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>rejection_time: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>votes_for: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>votes_against: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_governance_ProposalImplementedEvent"></a>

## Struct `ProposalImplementedEvent`

Event emitted when a proposal is implemented


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/governance.md#social_contracts_governance_ProposalImplementedEvent">ProposalImplementedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>proposal_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>implementation_time: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>description: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_governance_ProposalRewardPoolForfeitedToTreasuryEvent"></a>

## Struct `ProposalRewardPoolForfeitedToTreasuryEvent`

Failed proposal pool sent to a treasury (ecosystem or platform governance) instead of the submitter.


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/governance.md#social_contracts_governance_ProposalRewardPoolForfeitedToTreasuryEvent">ProposalRewardPoolForfeitedToTreasuryEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>proposal_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
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
<code>reason: u8</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/governance.md#social_contracts_governance_registry_type">registry_type</a>: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>treasury_route: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>forfeited_time: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_governance_ProposalImplementationRewardToSubmitterEvent"></a>

## Struct `ProposalImplementationRewardToSubmitterEvent`

Approved proposal: reward pool sent to the submitter when marked implemented.


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/governance.md#social_contracts_governance_ProposalImplementationRewardToSubmitterEvent">ProposalImplementationRewardToSubmitterEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>proposal_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>submitter: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/governance.md#social_contracts_governance_registry_type">registry_type</a>: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>sent_time: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_governance_VoteDecryptionFailedEvent"></a>

## Struct `VoteDecryptionFailedEvent`

Event emitted when vote decryption fails


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/governance.md#social_contracts_governance_VoteDecryptionFailedEvent">VoteDecryptionFailedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>proposal_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>voter: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>failure_reason: <a href="../std/string.md#std_string_String">std::string::String</a></code>
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

<a name="social_contracts_governance_ProposalRescindedEvent"></a>

## Struct `ProposalRescindedEvent`

Event emitted when a proposal is rescinded by its submitter


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/governance.md#social_contracts_governance_ProposalRescindedEvent">ProposalRescindedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>proposal_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>submitter: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>rescind_time: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>refund_amount: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_governance_GovernanceParametersUpdatedEvent"></a>

## Struct `GovernanceParametersUpdatedEvent`

Event emitted when governance parameters are updated


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceParametersUpdatedEvent">GovernanceParametersUpdatedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../social_contracts/governance.md#social_contracts_governance_registry_type">registry_type</a>: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>updated_by: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>delegate_count: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>delegate_term_epochs: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/governance.md#social_contracts_governance_proposal_submission_cost">proposal_submission_cost</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_votes_per_user: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>quadratic_base_cost: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>voting_period_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>quorum_votes: u64</code>
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

<a name="social_contracts_governance_GovernanceRegistryCreatedEvent"></a>

## Struct `GovernanceRegistryCreatedEvent`

Event emitted when a governance registry is created
This event matches the GovernanceRegistryEvent structure expected by the indexer


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceRegistryCreatedEvent">GovernanceRegistryCreatedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>registry_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/governance.md#social_contracts_governance_registry_type">registry_type</a>: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>delegate_count: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>delegate_term_epochs: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/governance.md#social_contracts_governance_proposal_submission_cost">proposal_submission_cost</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_votes_per_user: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>quadratic_base_cost: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>voting_period_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>quorum_votes: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>updated_at: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_governance_DelegatePanelRefreshedEvent"></a>

## Struct `DelegatePanelRefreshedEvent`

Event emitted when a delegate panel refresh applies a completed term boundary
(<code><a href="../social_contracts/governance.md#social_contracts_governance_last_delegate_panel_boundary_epoch">last_delegate_panel_boundary_epoch</a></code> on the DAO).


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/governance.md#social_contracts_governance_DelegatePanelRefreshedEvent">DelegatePanelRefreshedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>registry_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>boundary_epoch: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/governance.md#social_contracts_governance_registry_type">registry_type</a>: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>executed_at_epoch: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_governance_BootstrapGovernanceRegistryIds"></a>

## Struct `BootstrapGovernanceRegistryIds`

PoC + SPoT governance registry IDs returned from bootstrap for wiring into module configs.


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/governance.md#social_contracts_governance_BootstrapGovernanceRegistryIds">BootstrapGovernanceRegistryIds</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../social_contracts/governance.md#social_contracts_governance_poc_governance_registry_id">poc_governance_registry_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/governance.md#social_contracts_governance_spot_governance_registry_id">spot_governance_registry_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="social_contracts_governance_EUnauthorized"></a>

Error codes


<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_EUnauthorized">EUnauthorized</a>: u64 = 0;
</code></pre>



<a name="social_contracts_governance_EInvalidAmount"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_EInvalidAmount">EInvalidAmount</a>: u64 = 1;
</code></pre>



<a name="social_contracts_governance_EInvalidParameter"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_EInvalidParameter">EInvalidParameter</a>: u64 = 2;
</code></pre>



<a name="social_contracts_governance_ENotDelegate"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_ENotDelegate">ENotDelegate</a>: u64 = 3;
</code></pre>



<a name="social_contracts_governance_EAlreadyDelegate"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_EAlreadyDelegate">EAlreadyDelegate</a>: u64 = 4;
</code></pre>



<a name="social_contracts_governance_EProposalNotFound"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_EProposalNotFound">EProposalNotFound</a>: u64 = 6;
</code></pre>



<a name="social_contracts_governance_EInvalidProposalStatus"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_EInvalidProposalStatus">EInvalidProposalStatus</a>: u64 = 7;
</code></pre>



<a name="social_contracts_governance_EAlreadyVoted"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_EAlreadyVoted">EAlreadyVoted</a>: u64 = 8;
</code></pre>



<a name="social_contracts_governance_ENotVotingPhase"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_ENotVotingPhase">ENotVotingPhase</a>: u64 = 9;
</code></pre>



<a name="social_contracts_governance_EVotingPeriodNotEnded"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_EVotingPeriodNotEnded">EVotingPeriodNotEnded</a>: u64 = 11;
</code></pre>



<a name="social_contracts_governance_EVotingPeriodEnded"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_EVotingPeriodEnded">EVotingPeriodEnded</a>: u64 = 12;
</code></pre>



<a name="social_contracts_governance_EExceedsMaxVotes"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_EExceedsMaxVotes">EExceedsMaxVotes</a>: u64 = 13;
</code></pre>



<a name="social_contracts_governance_EInvalidVoteCount"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_EInvalidVoteCount">EInvalidVoteCount</a>: u64 = 14;
</code></pre>



<a name="social_contracts_governance_EInvalidRegistry"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_EInvalidRegistry">EInvalidRegistry</a>: u64 = 15;
</code></pre>



<a name="social_contracts_governance_EAlreadyNominated"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_EAlreadyNominated">EAlreadyNominated</a>: u64 = 16;
</code></pre>



<a name="social_contracts_governance_EWrongVersion"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_EWrongVersion">EWrongVersion</a>: u64 = 17;
</code></pre>



<a name="social_contracts_governance_EDelegateAnonNotAllowed"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_EDelegateAnonNotAllowed">EDelegateAnonNotAllowed</a>: u64 = 18;
</code></pre>



<a name="social_contracts_governance_EOverflow"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_EOverflow">EOverflow</a>: u64 = 19;
</code></pre>



<a name="social_contracts_governance_ENoExistingVote"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_ENoExistingVote">ENoExistingVote</a>: u64 = 20;
</code></pre>



<a name="social_contracts_governance_EWrongRegistryForTreasuryRoute"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_EWrongRegistryForTreasuryRoute">EWrongRegistryForTreasuryRoute</a>: u64 = 21;
</code></pre>



<a name="social_contracts_governance_EUsePlatformVoteEntry"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_EUsePlatformVoteEntry">EUsePlatformVoteEntry</a>: u64 = 22;
</code></pre>



<a name="social_contracts_governance_EProfileRequired"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_EProfileRequired">EProfileRequired</a>: u64 = 23;
</code></pre>



<a name="social_contracts_governance_EJuryHasJurisdiction"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_EJuryHasJurisdiction">EJuryHasJurisdiction</a>: u64 = 24;
</code></pre>



<a name="social_contracts_governance_EJuryStageDisabled"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_EJuryStageDisabled">EJuryStageDisabled</a>: u64 = 25;
</code></pre>



<a name="social_contracts_governance_EAlreadyReferred"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_EAlreadyReferred">EAlreadyReferred</a>: u64 = 26;
</code></pre>



<a name="social_contracts_governance_EDelegateAlreadyActed"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_EDelegateAlreadyActed">EDelegateAlreadyActed</a>: u64 = 27;
</code></pre>



<a name="social_contracts_governance_ENoEscalationTrigger"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_ENoEscalationTrigger">ENoEscalationTrigger</a>: u64 = 28;
</code></pre>



<a name="social_contracts_governance_EPoolTooSmallForJury"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_EPoolTooSmallForJury">EPoolTooSmallForJury</a>: u64 = 29;
</code></pre>



<a name="social_contracts_governance_EWrongJuryCase"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_EWrongJuryCase">EWrongJuryCase</a>: u64 = 30;
</code></pre>



<a name="social_contracts_governance_EJuryNotFinished"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_EJuryNotFinished">EJuryNotFinished</a>: u64 = 31;
</code></pre>



<a name="social_contracts_governance_EInvalidJuryPolicy"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_EInvalidJuryPolicy">EInvalidJuryPolicy</a>: u64 = 32;
</code></pre>



<a name="social_contracts_governance_ENoJuryRecord"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_ENoJuryRecord">ENoJuryRecord</a>: u64 = 33;
</code></pre>



<a name="social_contracts_governance_ERefundsOutstanding"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_ERefundsOutstanding">ERefundsOutstanding</a>: u64 = 34;
</code></pre>



<a name="social_contracts_governance_EClaimWindowClosed"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_EClaimWindowClosed">EClaimWindowClosed</a>: u64 = 35;
</code></pre>



<a name="social_contracts_governance_ENothingToClaim"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_ENothingToClaim">ENothingToClaim</a>: u64 = 36;
</code></pre>



<a name="social_contracts_governance_EAlreadyClaimed"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_EAlreadyClaimed">EAlreadyClaimed</a>: u64 = 37;
</code></pre>



<a name="social_contracts_governance_ENotRefunded"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_ENotRefunded">ENotRefunded</a>: u64 = 38;
</code></pre>



<a name="social_contracts_governance_FORFEIT_REASON_QUORUM_NOT_MET"></a>

Forfeiture reason (indexer) — same values as old refund reason codes.


<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_FORFEIT_REASON_QUORUM_NOT_MET">FORFEIT_REASON_QUORUM_NOT_MET</a>: u8 = 0;
</code></pre>



<a name="social_contracts_governance_FORFEIT_REASON_COMMUNITY_REJECTED"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_FORFEIT_REASON_COMMUNITY_REJECTED">FORFEIT_REASON_COMMUNITY_REJECTED</a>: u8 = 1;
</code></pre>



<a name="social_contracts_governance_FORFEIT_REASON_DELEGATE_REJECTED"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_FORFEIT_REASON_DELEGATE_REJECTED">FORFEIT_REASON_DELEGATE_REJECTED</a>: u8 = 2;
</code></pre>



<a name="social_contracts_governance_FORFEIT_REASON_JURY_REJECTED"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_FORFEIT_REASON_JURY_REJECTED">FORFEIT_REASON_JURY_REJECTED</a>: u8 = 3;
</code></pre>



<a name="social_contracts_governance_TREASURY_ROUTE_ECOSYSTEM"></a>

For [<code><a href="../social_contracts/governance.md#social_contracts_governance_ProposalRewardPoolForfeitedToTreasuryEvent">ProposalRewardPoolForfeitedToTreasuryEvent</a></code>]: ecosystem treasury.


<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_TREASURY_ROUTE_ECOSYSTEM">TREASURY_ROUTE_ECOSYSTEM</a>: u8 = 0;
</code></pre>



<a name="social_contracts_governance_TREASURY_ROUTE_PLATFORM"></a>

Platform governance (on-platform treasury, not the shared ecosystem treasury object).


<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_TREASURY_ROUTE_PLATFORM">TREASURY_ROUTE_PLATFORM</a>: u8 = 1;
</code></pre>



<a name="social_contracts_governance_MAX_U64"></a>

Maximum u64 value for overflow protection


<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_MAX_U64">MAX_U64</a>: u64 = 18446744073709551615;
</code></pre>



<a name="social_contracts_governance_PROPOSAL_TYPE_ECOSYSTEM"></a>

Proposal type constants


<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_PROPOSAL_TYPE_ECOSYSTEM">PROPOSAL_TYPE_ECOSYSTEM</a>: u8 = 0;
</code></pre>



<a name="social_contracts_governance_PROPOSAL_TYPE_PROOF_OF_CREATIVITY"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_PROPOSAL_TYPE_PROOF_OF_CREATIVITY">PROPOSAL_TYPE_PROOF_OF_CREATIVITY</a>: u8 = 1;
</code></pre>



<a name="social_contracts_governance_PROPOSAL_TYPE_SPOT"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_PROPOSAL_TYPE_SPOT">PROPOSAL_TYPE_SPOT</a>: u8 = 2;
</code></pre>



<a name="social_contracts_governance_PROPOSAL_TYPE_PLATFORM"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_PROPOSAL_TYPE_PLATFORM">PROPOSAL_TYPE_PLATFORM</a>: u8 = 3;
</code></pre>



<a name="social_contracts_governance_STATUS_SUBMITTED"></a>

Proposal status constants


<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_SUBMITTED">STATUS_SUBMITTED</a>: u8 = 0;
</code></pre>



<a name="social_contracts_governance_STATUS_DELEGATE_REVIEW"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_DELEGATE_REVIEW">STATUS_DELEGATE_REVIEW</a>: u8 = 1;
</code></pre>



<a name="social_contracts_governance_STATUS_COMMUNITY_VOTING"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_COMMUNITY_VOTING">STATUS_COMMUNITY_VOTING</a>: u8 = 2;
</code></pre>



<a name="social_contracts_governance_STATUS_APPROVED"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_APPROVED">STATUS_APPROVED</a>: u8 = 3;
</code></pre>



<a name="social_contracts_governance_STATUS_REJECTED"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_REJECTED">STATUS_REJECTED</a>: u8 = 4;
</code></pre>



<a name="social_contracts_governance_STATUS_IMPLEMENTED"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_IMPLEMENTED">STATUS_IMPLEMENTED</a>: u8 = 5;
</code></pre>



<a name="social_contracts_governance_STATUS_OWNER_RESCIND"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_OWNER_RESCIND">STATUS_OWNER_RESCIND</a>: u8 = 6;
</code></pre>



<a name="social_contracts_governance_VOTED_COMMUNITY_FIELD"></a>

Field names for dynamic fields


<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_VOTED_COMMUNITY_FIELD">VOTED_COMMUNITY_FIELD</a>: vector&lt;u8&gt; = vector[118, 111, 116, 101, 100, 95, 99, 111, 109, 109, 117, 110, 105, 116, 121];
</code></pre>



<a name="social_contracts_governance_DELEGATE_VOTES_FIELD"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_DELEGATE_VOTES_FIELD">DELEGATE_VOTES_FIELD</a>: vector&lt;u8&gt; = vector[100, 101, 108, 101, 103, 97, 116, 101, 95, 118, 111, 116, 101, 115];
</code></pre>



<a name="social_contracts_governance_VOTED_DELEGATES_LIST_FIELD"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_VOTED_DELEGATES_LIST_FIELD">VOTED_DELEGATES_LIST_FIELD</a>: vector&lt;u8&gt; = vector[118, 111, 116, 101, 100, 95, 100, 101, 108, 101, 103, 97, 116, 101, 115, 95, 108, 105, 115, 116];
</code></pre>



<a name="social_contracts_governance_DELEGATE_REASONS_FIELD"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_DELEGATE_REASONS_FIELD">DELEGATE_REASONS_FIELD</a>: vector&lt;u8&gt; = vector[100, 101, 108, 101, 103, 97, 116, 101, 95, 114, 101, 97, 115, 111, 110, 115];
</code></pre>



<a name="social_contracts_governance_ENCRYPTED_VOTES_FIELD"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_ENCRYPTED_VOTES_FIELD">ENCRYPTED_VOTES_FIELD</a>: vector&lt;u8&gt; = vector[101, 110, 99, 114, 121, 112, 116, 101, 100, 95, 118, 111, 116, 101, 115];
</code></pre>



<a name="social_contracts_governance_ANON_VOTERS_FIELD"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_ANON_VOTERS_FIELD">ANON_VOTERS_FIELD</a>: vector&lt;u8&gt; = vector[97, 110, 111, 110, 95, 118, 111, 116, 101, 114, 115];
</code></pre>



<a name="social_contracts_governance_JURY_POLICY_FIELD"></a>

Per-registry jury policy; absent means the AI jury is off and governance behaves as it always has.


<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_JURY_POLICY_FIELD">JURY_POLICY_FIELD</a>: vector&lt;u8&gt; = vector[106, 117, 114, 121, 95, 112, 111, 108, 105, 99, 121];
</code></pre>



<a name="social_contracts_governance_JURY_COUNCIL_FIELD"></a>

Per-proposal jury records, one per human step the jury can take over.


<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_JURY_COUNCIL_FIELD">JURY_COUNCIL_FIELD</a>: vector&lt;u8&gt; = vector[106, 117, 114, 121, 95, 99, 111, 117, 110, 99, 105, 108];
</code></pre>



<a name="social_contracts_governance_JURY_COMMUNITY_FIELD"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_JURY_COMMUNITY_FIELD">JURY_COMMUNITY_FIELD</a>: vector&lt;u8&gt; = vector[106, 117, 114, 121, 95, 99, 111, 109, 109, 117, 110, 105, 116, 121];
</code></pre>



<a name="social_contracts_governance_VOTE_PAID_FIELD"></a>

What each community voter paid for extra votes, so a refund can pay each one back exactly.


<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_VOTE_PAID_FIELD">VOTE_PAID_FIELD</a>: vector&lt;u8&gt; = vector[118, 111, 116, 101, 95, 112, 97, 105, 100];
</code></pre>



<a name="social_contracts_governance_VOTE_REFUND_FIELD"></a>

Set when a community step is refunded because no jury could form.


<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_VOTE_REFUND_FIELD">VOTE_REFUND_FIELD</a>: vector&lt;u8&gt; = vector[118, 111, 116, 101, 95, 114, 101, 102, 117, 110, 100];
</code></pre>



<a name="social_contracts_governance_JURY_DEFER_MAX_MS"></a>

How long a step waits for a jury that cannot form before it resolves without one: the
council step goes to the community vote, the community step is refunded.


<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_JURY_DEFER_MAX_MS">JURY_DEFER_MAX_MS</a>: u64 = 2592000000;
</code></pre>



<a name="social_contracts_governance_VOTE_REFUND_CLAIM_WINDOW_MS"></a>

How long voters have to claim a refund. After it, the unclaimed rest may go to the treasury.


<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_VOTE_REFUND_CLAIM_WINDOW_MS">VOTE_REFUND_CLAIM_WINDOW_MS</a>: u64 = 31536000000;
</code></pre>



<a name="social_contracts_governance_ESCALATE_OPENED"></a>

What [<code><a href="../social_contracts/governance.md#social_contracts_governance_open_jury_for_proposal">open_jury_for_proposal</a></code>] did.


<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_ESCALATE_OPENED">ESCALATE_OPENED</a>: u8 = 0;
</code></pre>



<a name="social_contracts_governance_ESCALATE_JURY_PAUSED"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_ESCALATE_JURY_PAUSED">ESCALATE_JURY_PAUSED</a>: u8 = 1;
</code></pre>



<a name="social_contracts_governance_ESCALATE_DEFERRED"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_ESCALATE_DEFERRED">ESCALATE_DEFERRED</a>: u8 = 2;
</code></pre>



<a name="social_contracts_governance_ESCALATE_ADVANCED"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_ESCALATE_ADVANCED">ESCALATE_ADVANCED</a>: u8 = 3;
</code></pre>



<a name="social_contracts_governance_ESCALATE_REFUNDED"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_ESCALATE_REFUNDED">ESCALATE_REFUNDED</a>: u8 = 4;
</code></pre>



<a name="social_contracts_governance_JURY_STAGE_COUNCIL"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_JURY_STAGE_COUNCIL">JURY_STAGE_COUNCIL</a>: u8 = 1;
</code></pre>



<a name="social_contracts_governance_JURY_STAGE_COMMUNITY"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_JURY_STAGE_COMMUNITY">JURY_STAGE_COMMUNITY</a>: u8 = 2;
</code></pre>



<a name="social_contracts_governance_JURY_STATE_REFERRING"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_JURY_STATE_REFERRING">JURY_STATE_REFERRING</a>: u8 = 1;
</code></pre>



<a name="social_contracts_governance_JURY_STATE_AWAITING"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_JURY_STATE_AWAITING">JURY_STATE_AWAITING</a>: u8 = 2;
</code></pre>



<a name="social_contracts_governance_JURY_STATE_ESCALATED"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_JURY_STATE_ESCALATED">JURY_STATE_ESCALATED</a>: u8 = 3;
</code></pre>



<a name="social_contracts_governance_JURY_STATE_DECIDED"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_JURY_STATE_DECIDED">JURY_STATE_DECIDED</a>: u8 = 4;
</code></pre>



<a name="social_contracts_governance_JURY_TRIGGER_REFERRAL_MAJORITY"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_JURY_TRIGGER_REFERRAL_MAJORITY">JURY_TRIGGER_REFERRAL_MAJORITY</a>: u8 = 1;
</code></pre>



<a name="social_contracts_governance_JURY_TRIGGER_COUNCIL_TIE"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_JURY_TRIGGER_COUNCIL_TIE">JURY_TRIGGER_COUNCIL_TIE</a>: u8 = 2;
</code></pre>



<a name="social_contracts_governance_JURY_TRIGGER_COUNCIL_DEADLINE"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_JURY_TRIGGER_COUNCIL_DEADLINE">JURY_TRIGGER_COUNCIL_DEADLINE</a>: u8 = 3;
</code></pre>



<a name="social_contracts_governance_JURY_TRIGGER_QUORUM_NOT_MET"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_JURY_TRIGGER_QUORUM_NOT_MET">JURY_TRIGGER_QUORUM_NOT_MET</a>: u8 = 4;
</code></pre>



<a name="social_contracts_governance_JURY_TRIGGER_VOTE_TIE"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_JURY_TRIGGER_VOTE_TIE">JURY_TRIGGER_VOTE_TIE</a>: u8 = 5;
</code></pre>



<a name="social_contracts_governance_JURY_OUTCOME_FORWARD"></a>

Jury menu: option 0 moves the proposal forward (council: to the community vote; community:
approve), option 1 rejects it.


<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_JURY_OUTCOME_FORWARD">JURY_OUTCOME_FORWARD</a>: u8 = 0;
</code></pre>



<a name="social_contracts_governance_JURY_OUTCOME_REJECT"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_JURY_OUTCOME_REJECT">JURY_OUTCOME_REJECT</a>: u8 = 1;
</code></pre>



<a name="social_contracts_governance_JURY_ACTION_ADVANCE"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_JURY_ACTION_ADVANCE">JURY_ACTION_ADVANCE</a>: u8 = 0;
</code></pre>



<a name="social_contracts_governance_JURY_ACTION_APPROVE"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_JURY_ACTION_APPROVE">JURY_ACTION_APPROVE</a>: u8 = 1;
</code></pre>



<a name="social_contracts_governance_JURY_ACTION_REJECT"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_JURY_ACTION_REJECT">JURY_ACTION_REJECT</a>: u8 = 2;
</code></pre>



<a name="social_contracts_governance_JURY_ACTION_DEFERRED"></a>

The case could not seat a jury; its fee went back to the pool and the step waits again.


<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_JURY_ACTION_DEFERRED">JURY_ACTION_DEFERRED</a>: u8 = 3;
</code></pre>



<a name="social_contracts_governance_JURY_EVIDENCE_KIND_PROPOSAL"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_JURY_EVIDENCE_KIND_PROPOSAL">JURY_EVIDENCE_KIND_PROPOSAL</a>: u8 = 1;
</code></pre>



<a name="social_contracts_governance_JURY_FAILURE_PATH_DEFAULT_DECISION"></a>

Stored on a case so an indexer can tell it is not a task case's failure path.


<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_JURY_FAILURE_PATH_DEFAULT_DECISION">JURY_FAILURE_PATH_DEFAULT_DECISION</a>: u8 = 2;
</code></pre>



<a name="social_contracts_governance_NO_DECISION"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_NO_DECISION">NO_DECISION</a>: u8 = 255;
</code></pre>



<a name="social_contracts_governance_DELEGATE_VOTE_STILL_IN_REVIEW"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_DELEGATE_VOTE_STILL_IN_REVIEW">DELEGATE_VOTE_STILL_IN_REVIEW</a>: u8 = 0;
</code></pre>



<a name="social_contracts_governance_DELEGATE_VOTE_TO_COMMUNITY"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_DELEGATE_VOTE_TO_COMMUNITY">DELEGATE_VOTE_TO_COMMUNITY</a>: u8 = 1;
</code></pre>



<a name="social_contracts_governance_DELEGATE_VOTE_TO_REJECT"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_DELEGATE_VOTE_TO_REJECT">DELEGATE_VOTE_TO_REJECT</a>: u8 = 2;
</code></pre>



<a name="social_contracts_governance_FINALIZE_OUTCOME_APPROVED"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_FINALIZE_OUTCOME_APPROVED">FINALIZE_OUTCOME_APPROVED</a>: u8 = 0;
</code></pre>



<a name="social_contracts_governance_FINALIZE_OUTCOME_REJECT_COMMUNITY"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_FINALIZE_OUTCOME_REJECT_COMMUNITY">FINALIZE_OUTCOME_REJECT_COMMUNITY</a>: u8 = 1;
</code></pre>



<a name="social_contracts_governance_FINALIZE_OUTCOME_REJECT_QUORUM"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_FINALIZE_OUTCOME_REJECT_QUORUM">FINALIZE_OUTCOME_REJECT_QUORUM</a>: u8 = 2;
</code></pre>



<a name="social_contracts_governance_FINALIZE_OUTCOME_NEEDS_JURY"></a>

The jury decides; no status, bucket or delegate statistic has changed.


<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_FINALIZE_OUTCOME_NEEDS_JURY">FINALIZE_OUTCOME_NEEDS_JURY</a>: u8 = 3;
</code></pre>



<a name="social_contracts_governance_poc_governance_registry_id"></a>

## Function `poc_governance_registry_id`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_poc_governance_registry_id">poc_governance_registry_id</a>(ids: &<a href="../social_contracts/governance.md#social_contracts_governance_BootstrapGovernanceRegistryIds">social_contracts::governance::BootstrapGovernanceRegistryIds</a>): <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_poc_governance_registry_id">poc_governance_registry_id</a>(ids: &<a href="../social_contracts/governance.md#social_contracts_governance_BootstrapGovernanceRegistryIds">BootstrapGovernanceRegistryIds</a>): ID {
    ids.<a href="../social_contracts/governance.md#social_contracts_governance_poc_governance_registry_id">poc_governance_registry_id</a>
}
</code></pre>



</details>

<a name="social_contracts_governance_spot_governance_registry_id"></a>

## Function `spot_governance_registry_id`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_spot_governance_registry_id">spot_governance_registry_id</a>(ids: &<a href="../social_contracts/governance.md#social_contracts_governance_BootstrapGovernanceRegistryIds">social_contracts::governance::BootstrapGovernanceRegistryIds</a>): <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_spot_governance_registry_id">spot_governance_registry_id</a>(ids: &<a href="../social_contracts/governance.md#social_contracts_governance_BootstrapGovernanceRegistryIds">BootstrapGovernanceRegistryIds</a>): ID {
    ids.<a href="../social_contracts/governance.md#social_contracts_governance_spot_governance_registry_id">spot_governance_registry_id</a>
}
</code></pre>



</details>

<a name="social_contracts_governance_bootstrap_init"></a>

## Function `bootstrap_init`

Bootstrap initialization function - creates the governance registries.
Returns PoC and SPoT registry IDs for wiring into [<code><a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity">proof_of_creativity</a></code>] and [<code><a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth">social_proof_of_truth</a></code>].


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_bootstrap_init">bootstrap_init</a>(clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../social_contracts/governance.md#social_contracts_governance_BootstrapGovernanceRegistryIds">social_contracts::governance::BootstrapGovernanceRegistryIds</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_bootstrap_init">bootstrap_init</a>(clock: &Clock, ctx: &<b>mut</b> TxContext): <a href="../social_contracts/governance.md#social_contracts_governance_BootstrapGovernanceRegistryIds">BootstrapGovernanceRegistryIds</a> {
    <b>let</b> current_time = clock::timestamp_ms(clock);
    <b>let</b> founder = tx_context::sender(ctx);
    // Create MySocial Ecosystem Governance Registry
    <b>let</b> <b>mut</b> ecosystem_registry = <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a> {
        id: object::new(ctx),
        <a href="../social_contracts/governance.md#social_contracts_governance_registry_type">registry_type</a>: <a href="../social_contracts/governance.md#social_contracts_governance_PROPOSAL_TYPE_ECOSYSTEM">PROPOSAL_TYPE_ECOSYSTEM</a>,
        // Configuration parameters specific to ecosystem <a href="../social_contracts/governance.md#social_contracts_governance">governance</a>
        delegate_count: 3, // Larger council <b>for</b> ecosystem decisions
        delegate_term_epochs: 90, // 3 months <b>for</b> ecosystem delegates
        <a href="../social_contracts/governance.md#social_contracts_governance_proposal_submission_cost">proposal_submission_cost</a>: 100_000_000_000, // 100 MYSO <b>for</b> ecosystem proposals
        max_votes_per_user: 10, // Up to 10 votes per user
        quadratic_base_cost: 10_000_000_000, // 10 MYSO per additional vote
        voting_period_ms: 7 * 24 * 60 * 60 * 1000, // 7 days in milliseconds <b>for</b> ecosystem votes
        quorum_votes: 20, // 20 votes required <b>for</b> ecosystem proposals
        // Tables
        delegates: table::new&lt;<b>address</b>, <a href="../social_contracts/governance.md#social_contracts_governance_Delegate">Delegate</a>&gt;(ctx),
        proposals: table::new&lt;ID, bool&gt;(ctx),
        proposal_types: table::new&lt;ID, u8&gt;(ctx),
        proposals_by_status: table::new&lt;u8, vector&lt;ID&gt;&gt;(ctx),
        treasury: balance::zero(),
        nominated_delegates: table::new&lt;<b>address</b>, <a href="../social_contracts/governance.md#social_contracts_governance_NominatedDelegate">NominatedDelegate</a>&gt;(ctx),
        delegate_addresses: vec_set::empty&lt;<b>address</b>&gt;(),
        nominee_addresses: vec_set::empty&lt;<b>address</b>&gt;(),
        voters: table::new&lt;<b>address</b>, Table&lt;<b>address</b>, bool&gt;&gt;(ctx),
        <a href="../social_contracts/governance.md#social_contracts_governance_version">version</a>: <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(),
        <a href="../social_contracts/governance.md#social_contracts_governance_last_delegate_panel_boundary_epoch">last_delegate_panel_boundary_epoch</a>: 0,
    };
    // Initialize ecosystem registry's status tables
    <a href="../social_contracts/governance.md#social_contracts_governance_initialize_registry_tables">initialize_registry_tables</a>(&<b>mut</b> ecosystem_registry, ctx);
    // Get ecosystem registry ID before sharing
    <b>let</b> ecosystem_registry_id = object::id(&ecosystem_registry);
    // Emit event <b>for</b> ecosystem registry creation
    event::emit(<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceRegistryCreatedEvent">GovernanceRegistryCreatedEvent</a> {
        registry_id: ecosystem_registry_id,
        <a href="../social_contracts/governance.md#social_contracts_governance_registry_type">registry_type</a>: <a href="../social_contracts/governance.md#social_contracts_governance_PROPOSAL_TYPE_ECOSYSTEM">PROPOSAL_TYPE_ECOSYSTEM</a>,
        delegate_count: ecosystem_registry.delegate_count,
        delegate_term_epochs: ecosystem_registry.delegate_term_epochs,
        <a href="../social_contracts/governance.md#social_contracts_governance_proposal_submission_cost">proposal_submission_cost</a>: ecosystem_registry.<a href="../social_contracts/governance.md#social_contracts_governance_proposal_submission_cost">proposal_submission_cost</a>,
        max_votes_per_user: ecosystem_registry.max_votes_per_user,
        quadratic_base_cost: ecosystem_registry.quadratic_base_cost,
        voting_period_ms: ecosystem_registry.voting_period_ms,
        quorum_votes: ecosystem_registry.quorum_votes,
        updated_at: current_time,
    });
    <a href="../social_contracts/governance.md#social_contracts_governance_seed_founding_delegate">seed_founding_delegate</a>(&<b>mut</b> ecosystem_registry, founder, ctx);
    // Share the ecosystem registry object
    transfer::share_object(ecosystem_registry);
    // Create Proof of Creativity Governance Registry
    <b>let</b> <b>mut</b> proof_of_creativity_registry = <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a> {
        id: object::new(ctx),
        <a href="../social_contracts/governance.md#social_contracts_governance_registry_type">registry_type</a>: <a href="../social_contracts/governance.md#social_contracts_governance_PROPOSAL_TYPE_PROOF_OF_CREATIVITY">PROPOSAL_TYPE_PROOF_OF_CREATIVITY</a>,
        // Configuration parameters specific to proof of creativity <a href="../social_contracts/governance.md#social_contracts_governance">governance</a>
        delegate_count: 2, // Smaller council <b>for</b> proof of creativity
        delegate_term_epochs: 180, // 3 months <b>for</b> proof of creativity delegates
        <a href="../social_contracts/governance.md#social_contracts_governance_proposal_submission_cost">proposal_submission_cost</a>: 25_000_000_000, // 25 MYSO <b>for</b> proof of creativity
        max_votes_per_user: 3, // Up to 3 votes per user
        quadratic_base_cost: 2_500_000_000, // 2.5 MYSO per additional vote
        voting_period_ms: 24 * 60 * 60 * 1000, // 1 day in milliseconds <b>for</b> proof of creativity votes
        quorum_votes: 10, // 10 votes required <b>for</b> proof of creativity proposals
        // Tables
        delegates: table::new&lt;<b>address</b>, <a href="../social_contracts/governance.md#social_contracts_governance_Delegate">Delegate</a>&gt;(ctx),
        proposals: table::new&lt;ID, bool&gt;(ctx),
        proposal_types: table::new&lt;ID, u8&gt;(ctx),
        proposals_by_status: table::new&lt;u8, vector&lt;ID&gt;&gt;(ctx),
        treasury: balance::zero(),
        nominated_delegates: table::new&lt;<b>address</b>, <a href="../social_contracts/governance.md#social_contracts_governance_NominatedDelegate">NominatedDelegate</a>&gt;(ctx),
        delegate_addresses: vec_set::empty&lt;<b>address</b>&gt;(),
        nominee_addresses: vec_set::empty&lt;<b>address</b>&gt;(),
        voters: table::new&lt;<b>address</b>, Table&lt;<b>address</b>, bool&gt;&gt;(ctx),
        <a href="../social_contracts/governance.md#social_contracts_governance_version">version</a>: <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(),
        <a href="../social_contracts/governance.md#social_contracts_governance_last_delegate_panel_boundary_epoch">last_delegate_panel_boundary_epoch</a>: 0,
    };
    // Initialize proof of creativity registry's status tables
    <a href="../social_contracts/governance.md#social_contracts_governance_initialize_registry_tables">initialize_registry_tables</a>(&<b>mut</b> proof_of_creativity_registry, ctx);
    // Get proof of creativity registry ID before sharing
    <b>let</b> proof_of_creativity_registry_id = object::id(&proof_of_creativity_registry);
    // Emit event <b>for</b> proof of creativity registry creation
    event::emit(<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceRegistryCreatedEvent">GovernanceRegistryCreatedEvent</a> {
        registry_id: proof_of_creativity_registry_id,
        <a href="../social_contracts/governance.md#social_contracts_governance_registry_type">registry_type</a>: <a href="../social_contracts/governance.md#social_contracts_governance_PROPOSAL_TYPE_PROOF_OF_CREATIVITY">PROPOSAL_TYPE_PROOF_OF_CREATIVITY</a>,
        delegate_count: proof_of_creativity_registry.delegate_count,
        delegate_term_epochs: proof_of_creativity_registry.delegate_term_epochs,
        <a href="../social_contracts/governance.md#social_contracts_governance_proposal_submission_cost">proposal_submission_cost</a>: proof_of_creativity_registry.<a href="../social_contracts/governance.md#social_contracts_governance_proposal_submission_cost">proposal_submission_cost</a>,
        max_votes_per_user: proof_of_creativity_registry.max_votes_per_user,
        quadratic_base_cost: proof_of_creativity_registry.quadratic_base_cost,
        voting_period_ms: proof_of_creativity_registry.voting_period_ms,
        quorum_votes: proof_of_creativity_registry.quorum_votes,
        updated_at: current_time,
    });
    <a href="../social_contracts/governance.md#social_contracts_governance_seed_founding_delegate">seed_founding_delegate</a>(&<b>mut</b> proof_of_creativity_registry, founder, ctx);
    // Share the proof of creativity registry object
    transfer::share_object(proof_of_creativity_registry);
    // Create Social Proof of Truth Governance Registry
    <b>let</b> <b>mut</b> spot_registry = <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a> {
        id: object::new(ctx),
        <a href="../social_contracts/governance.md#social_contracts_governance_registry_type">registry_type</a>: <a href="../social_contracts/governance.md#social_contracts_governance_PROPOSAL_TYPE_SPOT">PROPOSAL_TYPE_SPOT</a>,
        delegate_count: 3,
        delegate_term_epochs: 90,
        <a href="../social_contracts/governance.md#social_contracts_governance_proposal_submission_cost">proposal_submission_cost</a>: 10_000_000_000,
        max_votes_per_user: 5,
        quadratic_base_cost: 1_000_000_000,
        voting_period_ms: 3 * 24 * 60 * 60 * 1000,
        quorum_votes: 10,
        delegates: table::new&lt;<b>address</b>, <a href="../social_contracts/governance.md#social_contracts_governance_Delegate">Delegate</a>&gt;(ctx),
        proposals: table::new&lt;ID, bool&gt;(ctx),
        proposal_types: table::new&lt;ID, u8&gt;(ctx),
        proposals_by_status: table::new&lt;u8, vector&lt;ID&gt;&gt;(ctx),
        treasury: balance::zero(),
        nominated_delegates: table::new&lt;<b>address</b>, <a href="../social_contracts/governance.md#social_contracts_governance_NominatedDelegate">NominatedDelegate</a>&gt;(ctx),
        delegate_addresses: vec_set::empty&lt;<b>address</b>&gt;(),
        nominee_addresses: vec_set::empty&lt;<b>address</b>&gt;(),
        voters: table::new&lt;<b>address</b>, Table&lt;<b>address</b>, bool&gt;&gt;(ctx),
        <a href="../social_contracts/governance.md#social_contracts_governance_version">version</a>: <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(),
        <a href="../social_contracts/governance.md#social_contracts_governance_last_delegate_panel_boundary_epoch">last_delegate_panel_boundary_epoch</a>: 0,
    };
    <a href="../social_contracts/governance.md#social_contracts_governance_initialize_registry_tables">initialize_registry_tables</a>(&<b>mut</b> spot_registry, ctx);
    <b>let</b> spot_registry_id = object::id(&spot_registry);
    event::emit(<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceRegistryCreatedEvent">GovernanceRegistryCreatedEvent</a> {
        registry_id: spot_registry_id,
        <a href="../social_contracts/governance.md#social_contracts_governance_registry_type">registry_type</a>: <a href="../social_contracts/governance.md#social_contracts_governance_PROPOSAL_TYPE_SPOT">PROPOSAL_TYPE_SPOT</a>,
        delegate_count: spot_registry.delegate_count,
        delegate_term_epochs: spot_registry.delegate_term_epochs,
        <a href="../social_contracts/governance.md#social_contracts_governance_proposal_submission_cost">proposal_submission_cost</a>: spot_registry.<a href="../social_contracts/governance.md#social_contracts_governance_proposal_submission_cost">proposal_submission_cost</a>,
        max_votes_per_user: spot_registry.max_votes_per_user,
        quadratic_base_cost: spot_registry.quadratic_base_cost,
        voting_period_ms: spot_registry.voting_period_ms,
        quorum_votes: spot_registry.quorum_votes,
        updated_at: current_time,
    });
    <a href="../social_contracts/governance.md#social_contracts_governance_seed_founding_delegate">seed_founding_delegate</a>(&<b>mut</b> spot_registry, founder, ctx);
    transfer::share_object(spot_registry);
    <a href="../social_contracts/governance.md#social_contracts_governance_BootstrapGovernanceRegistryIds">BootstrapGovernanceRegistryIds</a> {
        <a href="../social_contracts/governance.md#social_contracts_governance_poc_governance_registry_id">poc_governance_registry_id</a>: proof_of_creativity_registry_id,
        <a href="../social_contracts/governance.md#social_contracts_governance_spot_governance_registry_id">spot_governance_registry_id</a>: spot_registry_id,
    }
}
</code></pre>



</details>

<a name="social_contracts_governance_seed_founding_delegate"></a>

## Function `seed_founding_delegate`

Install the founding delegate without going through nomination (bootstrap / platform creation).


<pre><code><b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_seed_founding_delegate">seed_founding_delegate</a>(registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, addr: <b>address</b>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_seed_founding_delegate">seed_founding_delegate</a>(registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>, addr: <b>address</b>, ctx: &TxContext) {
    <b>let</b> current_epoch = tx_context::epoch(ctx);
    <b>let</b> delegate_term_epochs = registry.delegate_term_epochs;
    <b>let</b> term_start = current_epoch;
    <b>let</b> term_end = term_start + delegate_term_epochs;
    <b>let</b> new_delegate = <a href="../social_contracts/governance.md#social_contracts_governance_Delegate">Delegate</a> {
        <b>address</b>: addr,
        upvotes: 0,
        downvotes: 0,
        proposals_reviewed: 0,
        proposals_submitted: 0,
        sided_winning_proposals: 0,
        sided_losing_proposals: 0,
        term_start,
        term_end,
    };
    table::add(&<b>mut</b> registry.delegates, addr, new_delegate);
    vec_set::insert(&<b>mut</b> registry.delegate_addresses, addr);
    event::emit(<a href="../social_contracts/governance.md#social_contracts_governance_DelegateElectedEvent">DelegateElectedEvent</a> {
        delegate_address: addr,
        term_start,
        term_end,
        <a href="../social_contracts/governance.md#social_contracts_governance_registry_type">registry_type</a>: registry.<a href="../social_contracts/governance.md#social_contracts_governance_registry_type">registry_type</a>,
        upvotes: 0,
        downvotes: 0,
    });
}
</code></pre>



</details>

<a name="social_contracts_governance_assert_has_profile"></a>

## Function `assert_has_profile`



<pre><code><b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_assert_has_profile">assert_has_profile</a>(username_registry: &<a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">social_contracts::profile::UsernameRegistry</a>, addr: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_assert_has_profile">assert_has_profile</a>(username_registry: &UsernameRegistry, addr: <b>address</b>) {
    <b>assert</b>!(option::is_some(&<a href="../social_contracts/profile.md#social_contracts_profile_lookup_profile_by_owner">profile::lookup_profile_by_owner</a>(username_registry, addr)), <a href="../social_contracts/governance.md#social_contracts_governance_EProfileRequired">EProfileRequired</a>);
}
</code></pre>



</details>

<a name="social_contracts_governance_initialize_registry_tables"></a>

## Function `initialize_registry_tables`



<pre><code><b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_initialize_registry_tables">initialize_registry_tables</a>(registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, _ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_initialize_registry_tables">initialize_registry_tables</a>(registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>, _ctx: &<b>mut</b> TxContext) {
    table::add(&<b>mut</b> registry.proposals_by_status, <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_SUBMITTED">STATUS_SUBMITTED</a>, vector::empty&lt;ID&gt;());
    table::add(&<b>mut</b> registry.proposals_by_status, <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_DELEGATE_REVIEW">STATUS_DELEGATE_REVIEW</a>, vector::empty&lt;ID&gt;());
    table::add(&<b>mut</b> registry.proposals_by_status, <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_COMMUNITY_VOTING">STATUS_COMMUNITY_VOTING</a>, vector::empty&lt;ID&gt;());
    table::add(&<b>mut</b> registry.proposals_by_status, <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_APPROVED">STATUS_APPROVED</a>, vector::empty&lt;ID&gt;());
    table::add(&<b>mut</b> registry.proposals_by_status, <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_REJECTED">STATUS_REJECTED</a>, vector::empty&lt;ID&gt;());
    table::add(&<b>mut</b> registry.proposals_by_status, <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_IMPLEMENTED">STATUS_IMPLEMENTED</a>, vector::empty&lt;ID&gt;());
    table::add(&<b>mut</b> registry.proposals_by_status, <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_OWNER_RESCIND">STATUS_OWNER_RESCIND</a>, vector::empty&lt;ID&gt;());
}
</code></pre>



</details>

<a name="social_contracts_governance_update_governance_parameters_internal"></a>

## Function `update_governance_parameters_internal`

Update governance parameters (internal function)
This function does not perform authorization checks - callers must verify permissions


<pre><code><b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_update_governance_parameters_internal">update_governance_parameters_internal</a>(registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, delegate_count: u64, delegate_term_epochs: u64, <a href="../social_contracts/governance.md#social_contracts_governance_proposal_submission_cost">proposal_submission_cost</a>: u64, max_votes_per_user: u64, quadratic_base_cost: u64, voting_period_ms: u64, quorum_votes: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, _ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_update_governance_parameters_internal">update_governance_parameters_internal</a>(
    registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    delegate_count: u64,
    delegate_term_epochs: u64,
    <a href="../social_contracts/governance.md#social_contracts_governance_proposal_submission_cost">proposal_submission_cost</a>: u64,
    max_votes_per_user: u64,
    quadratic_base_cost: u64,
    voting_period_ms: u64,
    quorum_votes: u64,
    clock: &Clock,
    _ctx: &<b>mut</b> TxContext
) {
    // Check <a href="../social_contracts/governance.md#social_contracts_governance_version">version</a> compatibility
    <b>assert</b>!(registry.<a href="../social_contracts/governance.md#social_contracts_governance_version">version</a> == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/governance.md#social_contracts_governance_EWrongVersion">EWrongVersion</a>);
    // Ensure parameters are sensible
    <b>assert</b>!(delegate_count &gt; 1, <a href="../social_contracts/governance.md#social_contracts_governance_EInvalidParameter">EInvalidParameter</a>);
    <b>assert</b>!(delegate_term_epochs &gt; 0, <a href="../social_contracts/governance.md#social_contracts_governance_EInvalidParameter">EInvalidParameter</a>);
    // <a href="../social_contracts/governance.md#social_contracts_governance_proposal_submission_cost">proposal_submission_cost</a> can be 0
    <b>assert</b>!(max_votes_per_user &gt; 0, <a href="../social_contracts/governance.md#social_contracts_governance_EInvalidParameter">EInvalidParameter</a>);
    // quadratic_base_cost can be 0 (<b>if</b> voting is free)
    <b>assert</b>!(voting_period_ms &gt; 0, <a href="../social_contracts/governance.md#social_contracts_governance_EInvalidParameter">EInvalidParameter</a>);
    <b>assert</b>!(quorum_votes &gt; 0, <a href="../social_contracts/governance.md#social_contracts_governance_EInvalidParameter">EInvalidParameter</a>);
    // Update parameters
    registry.delegate_count = delegate_count;
    registry.delegate_term_epochs = delegate_term_epochs;
    registry.<a href="../social_contracts/governance.md#social_contracts_governance_proposal_submission_cost">proposal_submission_cost</a> = <a href="../social_contracts/governance.md#social_contracts_governance_proposal_submission_cost">proposal_submission_cost</a>;
    registry.max_votes_per_user = max_votes_per_user;
    registry.quadratic_base_cost = quadratic_base_cost;
    registry.voting_period_ms = voting_period_ms;
    registry.quorum_votes = quorum_votes;
    // Emit <a href="../social_contracts/governance.md#social_contracts_governance">governance</a> parameters updated event
    event::emit(<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceParametersUpdatedEvent">GovernanceParametersUpdatedEvent</a> {
        <a href="../social_contracts/governance.md#social_contracts_governance_registry_type">registry_type</a>: registry.<a href="../social_contracts/governance.md#social_contracts_governance_registry_type">registry_type</a>,
        updated_by: tx_context::sender(_ctx),
        delegate_count,
        delegate_term_epochs,
        <a href="../social_contracts/governance.md#social_contracts_governance_proposal_submission_cost">proposal_submission_cost</a>,
        max_votes_per_user,
        quadratic_base_cost,
        voting_period_ms,
        quorum_votes,
        timestamp: clock::timestamp_ms(clock),
    });
}
</code></pre>



</details>

<a name="social_contracts_governance_update_platform_governance_parameters"></a>

## Function `update_platform_governance_parameters`

Update governance parameters for platform registries
Can only be called by the platform module (which verifies platform ownership)
This function is package-private to prevent direct calls that bypass platform ownership verification


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_update_platform_governance_parameters">update_platform_governance_parameters</a>(registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, platform_developer: <b>address</b>, delegate_count: u64, delegate_term_epochs: u64, <a href="../social_contracts/governance.md#social_contracts_governance_proposal_submission_cost">proposal_submission_cost</a>: u64, max_votes_per_user: u64, quadratic_base_cost: u64, voting_period_ms: u64, quorum_votes: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_update_platform_governance_parameters">update_platform_governance_parameters</a>(
    registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    platform_developer: <b>address</b>,
    delegate_count: u64,
    delegate_term_epochs: u64,
    <a href="../social_contracts/governance.md#social_contracts_governance_proposal_submission_cost">proposal_submission_cost</a>: u64,
    max_votes_per_user: u64,
    quadratic_base_cost: u64,
    voting_period_ms: u64,
    quorum_votes: u64,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    // Verify this is a <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> registry
    <b>assert</b>!(registry.<a href="../social_contracts/governance.md#social_contracts_governance_registry_type">registry_type</a> == <a href="../social_contracts/governance.md#social_contracts_governance_PROPOSAL_TYPE_PLATFORM">PROPOSAL_TYPE_PLATFORM</a>, <a href="../social_contracts/governance.md#social_contracts_governance_EInvalidRegistry">EInvalidRegistry</a>);
    // Verify caller is <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> developer
    <b>let</b> caller = tx_context::sender(ctx);
    <b>assert</b>!(platform_developer == caller, <a href="../social_contracts/governance.md#social_contracts_governance_EUnauthorized">EUnauthorized</a>);
    // Call the internal update function
    <a href="../social_contracts/governance.md#social_contracts_governance_update_governance_parameters_internal">update_governance_parameters_internal</a>(
        registry,
        delegate_count,
        delegate_term_epochs,
        <a href="../social_contracts/governance.md#social_contracts_governance_proposal_submission_cost">proposal_submission_cost</a>,
        max_votes_per_user,
        quadratic_base_cost,
        voting_period_ms,
        quorum_votes,
        clock,
        ctx
    );
}
</code></pre>



</details>

<a name="social_contracts_governance_update_governance_parameters"></a>

## Function `update_governance_parameters`

Update governance parameters for ecosystem/proof-of-creativity registries
Can only be called by governance admin


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_update_governance_parameters">update_governance_parameters</a>(registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, _: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceAdminCap">social_contracts::governance::GovernanceAdminCap</a>, delegate_count: u64, delegate_term_epochs: u64, <a href="../social_contracts/governance.md#social_contracts_governance_proposal_submission_cost">proposal_submission_cost</a>: u64, max_votes_per_user: u64, quadratic_base_cost: u64, voting_period_ms: u64, quorum_votes: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, _ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_update_governance_parameters">update_governance_parameters</a>(
    registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    _: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceAdminCap">GovernanceAdminCap</a>,
    delegate_count: u64,
    delegate_term_epochs: u64,
    <a href="../social_contracts/governance.md#social_contracts_governance_proposal_submission_cost">proposal_submission_cost</a>: u64,
    max_votes_per_user: u64,
    quadratic_base_cost: u64,
    voting_period_ms: u64,
    quorum_votes: u64,
    clock: &Clock,
    _ctx: &<b>mut</b> TxContext
) {
    // Verify this is NOT a <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> registry
    <b>assert</b>!(registry.<a href="../social_contracts/governance.md#social_contracts_governance_registry_type">registry_type</a> != <a href="../social_contracts/governance.md#social_contracts_governance_PROPOSAL_TYPE_PLATFORM">PROPOSAL_TYPE_PLATFORM</a>, <a href="../social_contracts/governance.md#social_contracts_governance_EInvalidRegistry">EInvalidRegistry</a>);
    // Call the internal update function
    <a href="../social_contracts/governance.md#social_contracts_governance_update_governance_parameters_internal">update_governance_parameters_internal</a>(
        registry,
        delegate_count,
        delegate_term_epochs,
        <a href="../social_contracts/governance.md#social_contracts_governance_proposal_submission_cost">proposal_submission_cost</a>,
        max_votes_per_user,
        quadratic_base_cost,
        voting_period_ms,
        quorum_votes,
        clock,
        _ctx
    );
}
</code></pre>



</details>

<a name="social_contracts_governance_nominate_delegate"></a>

## Function `nominate_delegate`

Nominate self as a delegate. The sender must own a profile.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_nominate_delegate">nominate_delegate</a>(registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, username_registry: &<a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">social_contracts::profile::UsernameRegistry</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_nominate_delegate">nominate_delegate</a>(
    registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    username_registry: &UsernameRegistry,
    ctx: &<b>mut</b> TxContext
) {
    // Check <a href="../social_contracts/governance.md#social_contracts_governance_version">version</a> compatibility
    <b>assert</b>!(registry.<a href="../social_contracts/governance.md#social_contracts_governance_version">version</a> == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/governance.md#social_contracts_governance_EWrongVersion">EWrongVersion</a>);
    <b>let</b> caller = tx_context::sender(ctx);
    <a href="../social_contracts/governance.md#social_contracts_governance_assert_has_profile">assert_has_profile</a>(username_registry, caller);
    <b>let</b> current_epoch = tx_context::epoch(ctx);
    // Check <b>if</b> already a delegate or nominee delegate
    <b>assert</b>!(!table::contains(&registry.delegates, caller), <a href="../social_contracts/governance.md#social_contracts_governance_EAlreadyDelegate">EAlreadyDelegate</a>);
    <b>assert</b>!(!table::contains(&registry.nominated_delegates, caller), <a href="../social_contracts/governance.md#social_contracts_governance_EAlreadyNominated">EAlreadyNominated</a>);
    <b>let</b> scheduled_term_start_epoch = ((current_epoch / registry.delegate_term_epochs) + 1) * registry.delegate_term_epochs;
    <b>let</b> nominated_delegate = <a href="../social_contracts/governance.md#social_contracts_governance_NominatedDelegate">NominatedDelegate</a> {
        <b>address</b>: caller,
        scheduled_term_start_epoch,
        upvotes: 0,
        downvotes: 0,
    };
    table::add(&<b>mut</b> registry.nominated_delegates, caller, nominated_delegate);
    vec_set::insert(&<b>mut</b> registry.nominee_addresses, caller);
    // Initialize voter tracking table <b>for</b> this nominee
    table::add(&<b>mut</b> registry.voters, caller, table::new&lt;<b>address</b>, bool&gt;(ctx));
    event::emit(<a href="../social_contracts/governance.md#social_contracts_governance_DelegateNominatedEvent">DelegateNominatedEvent</a> {
        nominee_address: caller,
        scheduled_term_start_epoch,
        <a href="../social_contracts/governance.md#social_contracts_governance_registry_type">registry_type</a>: registry.<a href="../social_contracts/governance.md#social_contracts_governance_registry_type">registry_type</a>,
    });
    <a href="../social_contracts/governance.md#social_contracts_governance_try_update_delegate_panel_if_due">try_update_delegate_panel_if_due</a>(registry, ctx);
}
</code></pre>



</details>

<a name="social_contracts_governance_vote_for_delegate"></a>

## Function `vote_for_delegate`

Vote for or against a delegate or nominee delegate
Positive votes support the delegate, negative votes express disapproval
Users can change their vote at any time. The sender must own a profile.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_vote_for_delegate">vote_for_delegate</a>(registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, username_registry: &<a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">social_contracts::profile::UsernameRegistry</a>, target_address: <b>address</b>, upvote: bool, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_vote_for_delegate">vote_for_delegate</a>(
    registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    username_registry: &UsernameRegistry,
    target_address: <b>address</b>,
    upvote: bool,
    ctx: &<b>mut</b> TxContext
) {
    // Check <a href="../social_contracts/governance.md#social_contracts_governance_version">version</a> compatibility
    <b>assert</b>!(registry.<a href="../social_contracts/governance.md#social_contracts_governance_version">version</a> == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/governance.md#social_contracts_governance_EWrongVersion">EWrongVersion</a>);
    <b>let</b> caller = tx_context::sender(ctx);
    <a href="../social_contracts/governance.md#social_contracts_governance_assert_has_profile">assert_has_profile</a>(username_registry, caller);
    // Don't allow self-voting
    <b>assert</b>!(caller != target_address, <a href="../social_contracts/governance.md#social_contracts_governance_EUnauthorized">EUnauthorized</a>);
    // Variables <b>for</b> event emission
    <b>let</b> is_active_delegate: bool;
    <b>let</b> upvote_count: u64;
    <b>let</b> downvote_count: u64;
    // Handle voting <b>for</b> active delegates
    <b>if</b> (table::contains(&registry.delegates, target_address)) {
        is_active_delegate = <b>true</b>;
        <b>let</b> delegate = table::borrow_mut(&<b>mut</b> registry.delegates, target_address);
        // Create voter table <b>for</b> this target <b>if</b> it doesn't exist
        <b>if</b> (!table::contains(&registry.voters, target_address)) {
            table::add(&<b>mut</b> registry.voters, target_address, table::new&lt;<b>address</b>, bool&gt;(ctx));
        };
        <b>let</b> voter_table = table::borrow_mut(&<b>mut</b> registry.voters, target_address);
        // Check <b>if</b> user <b>has</b> already voted
        <b>if</b> (table::contains(voter_table, caller)) {
            <b>let</b> previous_vote = *table::borrow(voter_table, caller);
            // If same vote, do nothing
            <b>if</b> (previous_vote == upvote) {
                <a href="../social_contracts/governance.md#social_contracts_governance_try_update_delegate_panel_if_due">try_update_delegate_panel_if_due</a>(registry, ctx);
                <b>return</b>
            };
            // Different vote - remove previous vote with underflow protection
            <b>if</b> (previous_vote) {
                <b>assert</b>!(delegate.upvotes &gt; 0, <a href="../social_contracts/governance.md#social_contracts_governance_EOverflow">EOverflow</a>);
                delegate.upvotes = delegate.upvotes - 1;
            } <b>else</b> {
                <b>assert</b>!(delegate.downvotes &gt; 0, <a href="../social_contracts/governance.md#social_contracts_governance_EOverflow">EOverflow</a>);
                delegate.downvotes = delegate.downvotes - 1;
            };
            // Add new vote
            <b>if</b> (upvote) {
                <b>assert</b>!(delegate.upvotes &lt;= <a href="../social_contracts/governance.md#social_contracts_governance_MAX_U64">MAX_U64</a> - 1, <a href="../social_contracts/governance.md#social_contracts_governance_EOverflow">EOverflow</a>);
                delegate.upvotes = delegate.upvotes + 1;
            } <b>else</b> {
                <b>assert</b>!(delegate.downvotes &lt;= <a href="../social_contracts/governance.md#social_contracts_governance_MAX_U64">MAX_U64</a> - 1, <a href="../social_contracts/governance.md#social_contracts_governance_EOverflow">EOverflow</a>);
                delegate.downvotes = delegate.downvotes + 1;
            };
            // Update record
            *table::borrow_mut(voter_table, caller) = upvote;
        } <b>else</b> {
            // First time voting <b>for</b> this target
            <b>if</b> (upvote) {
                <b>assert</b>!(delegate.upvotes &lt;= <a href="../social_contracts/governance.md#social_contracts_governance_MAX_U64">MAX_U64</a> - 1, <a href="../social_contracts/governance.md#social_contracts_governance_EOverflow">EOverflow</a>);
                delegate.upvotes = delegate.upvotes + 1;
            } <b>else</b> {
                <b>assert</b>!(delegate.downvotes &lt;= <a href="../social_contracts/governance.md#social_contracts_governance_MAX_U64">MAX_U64</a> - 1, <a href="../social_contracts/governance.md#social_contracts_governance_EOverflow">EOverflow</a>);
                delegate.downvotes = delegate.downvotes + 1;
            };
            // Record vote
            table::add(voter_table, caller, upvote);
        };
        upvote_count = delegate.upvotes;
        downvote_count = delegate.downvotes;
    }
    // Handle voting <b>for</b> nominee delegates
    <b>else</b> <b>if</b> (table::contains(&registry.nominated_delegates, target_address)) {
        is_active_delegate = <b>false</b>;
        <b>let</b> nominee = table::borrow_mut(&<b>mut</b> registry.nominated_delegates, target_address);
        // Create voter table <b>for</b> this target <b>if</b> it doesn't exist
        <b>if</b> (!table::contains(&registry.voters, target_address)) {
            table::add(&<b>mut</b> registry.voters, target_address, table::new&lt;<b>address</b>, bool&gt;(ctx));
        };
        <b>let</b> voter_table = table::borrow_mut(&<b>mut</b> registry.voters, target_address);
        // Check <b>if</b> user <b>has</b> already voted
        <b>if</b> (table::contains(voter_table, caller)) {
            <b>let</b> previous_vote = *table::borrow(voter_table, caller);
            // If same vote, do nothing
            <b>if</b> (previous_vote == upvote) {
                <a href="../social_contracts/governance.md#social_contracts_governance_try_update_delegate_panel_if_due">try_update_delegate_panel_if_due</a>(registry, ctx);
                <b>return</b>
            };
            // Different vote - remove previous vote with underflow protection
            <b>if</b> (previous_vote) {
                <b>assert</b>!(nominee.upvotes &gt; 0, <a href="../social_contracts/governance.md#social_contracts_governance_EOverflow">EOverflow</a>);
                nominee.upvotes = nominee.upvotes - 1;
            } <b>else</b> {
                <b>assert</b>!(nominee.downvotes &gt; 0, <a href="../social_contracts/governance.md#social_contracts_governance_EOverflow">EOverflow</a>);
                nominee.downvotes = nominee.downvotes - 1;
            };
            // Add new vote
            <b>if</b> (upvote) {
                <b>assert</b>!(nominee.upvotes &lt;= <a href="../social_contracts/governance.md#social_contracts_governance_MAX_U64">MAX_U64</a> - 1, <a href="../social_contracts/governance.md#social_contracts_governance_EOverflow">EOverflow</a>);
                nominee.upvotes = nominee.upvotes + 1;
            } <b>else</b> {
                <b>assert</b>!(nominee.downvotes &lt;= <a href="../social_contracts/governance.md#social_contracts_governance_MAX_U64">MAX_U64</a> - 1, <a href="../social_contracts/governance.md#social_contracts_governance_EOverflow">EOverflow</a>);
                nominee.downvotes = nominee.downvotes + 1;
            };
            // Update record
            *table::borrow_mut(voter_table, caller) = upvote;
        } <b>else</b> {
            // First time voting <b>for</b> this nominee
            <b>if</b> (upvote) {
                <b>assert</b>!(nominee.upvotes &lt;= <a href="../social_contracts/governance.md#social_contracts_governance_MAX_U64">MAX_U64</a> - 1, <a href="../social_contracts/governance.md#social_contracts_governance_EOverflow">EOverflow</a>);
                nominee.upvotes = nominee.upvotes + 1;
            } <b>else</b> {
                <b>assert</b>!(nominee.downvotes &lt;= <a href="../social_contracts/governance.md#social_contracts_governance_MAX_U64">MAX_U64</a> - 1, <a href="../social_contracts/governance.md#social_contracts_governance_EOverflow">EOverflow</a>);
                nominee.downvotes = nominee.downvotes + 1;
            };
            // Record vote
            table::add(voter_table, caller, upvote);
        };
        upvote_count = nominee.upvotes;
        downvote_count = nominee.downvotes;
    }
    <b>else</b> { <b>abort</b> <a href="../social_contracts/governance.md#social_contracts_governance_ENotDelegate">ENotDelegate</a> };
    event::emit(<a href="../social_contracts/governance.md#social_contracts_governance_DelegateVotedEvent">DelegateVotedEvent</a> {
        target_address,
        voter: caller,
        is_active_delegate,
        upvote,
        new_upvote_count: upvote_count,
        new_downvote_count: downvote_count,
        <a href="../social_contracts/governance.md#social_contracts_governance_registry_type">registry_type</a>: registry.<a href="../social_contracts/governance.md#social_contracts_governance_registry_type">registry_type</a>,
    });
    <a href="../social_contracts/governance.md#social_contracts_governance_try_update_delegate_panel_if_due">try_update_delegate_panel_if_due</a>(registry, ctx);
}
</code></pre>



</details>

<a name="social_contracts_governance_clear_vote_for_delegate"></a>

## Function `clear_vote_for_delegate`

Remove the caller's up or down vote on a delegate or nominee (neutral / undo).
Aborts with <code><a href="../social_contracts/governance.md#social_contracts_governance_ENoExistingVote">ENoExistingVote</a></code> if the caller has not voted on this target.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_clear_vote_for_delegate">clear_vote_for_delegate</a>(registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, target_address: <b>address</b>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_clear_vote_for_delegate">clear_vote_for_delegate</a>(
    registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    target_address: <b>address</b>,
    ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(registry.<a href="../social_contracts/governance.md#social_contracts_governance_version">version</a> == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/governance.md#social_contracts_governance_EWrongVersion">EWrongVersion</a>);
    <b>let</b> caller = tx_context::sender(ctx);
    <b>assert</b>!(caller != target_address, <a href="../social_contracts/governance.md#social_contracts_governance_EUnauthorized">EUnauthorized</a>);
    <b>let</b> is_active_delegate: bool;
    <b>let</b> upvote_count: u64;
    <b>let</b> downvote_count: u64;
    <b>if</b> (table::contains(&registry.delegates, target_address)) {
        is_active_delegate = <b>true</b>;
        <b>assert</b>!(table::contains(&registry.voters, target_address), <a href="../social_contracts/governance.md#social_contracts_governance_ENoExistingVote">ENoExistingVote</a>);
        <b>let</b> voter_table = table::borrow_mut(&<b>mut</b> registry.voters, target_address);
        <b>assert</b>!(table::contains(voter_table, caller), <a href="../social_contracts/governance.md#social_contracts_governance_ENoExistingVote">ENoExistingVote</a>);
        <b>let</b> previous_vote = *table::borrow(voter_table, caller);
        <b>let</b> delegate = table::borrow_mut(&<b>mut</b> registry.delegates, target_address);
        <b>if</b> (previous_vote) {
            <b>assert</b>!(delegate.upvotes &gt; 0, <a href="../social_contracts/governance.md#social_contracts_governance_EOverflow">EOverflow</a>);
            delegate.upvotes = delegate.upvotes - 1;
        } <b>else</b> {
            <b>assert</b>!(delegate.downvotes &gt; 0, <a href="../social_contracts/governance.md#social_contracts_governance_EOverflow">EOverflow</a>);
            delegate.downvotes = delegate.downvotes - 1;
        };
        table::remove(voter_table, caller);
        upvote_count = delegate.upvotes;
        downvote_count = delegate.downvotes;
    } <b>else</b> <b>if</b> (table::contains(&registry.nominated_delegates, target_address)) {
        is_active_delegate = <b>false</b>;
        <b>assert</b>!(table::contains(&registry.voters, target_address), <a href="../social_contracts/governance.md#social_contracts_governance_ENoExistingVote">ENoExistingVote</a>);
        <b>let</b> voter_table = table::borrow_mut(&<b>mut</b> registry.voters, target_address);
        <b>assert</b>!(table::contains(voter_table, caller), <a href="../social_contracts/governance.md#social_contracts_governance_ENoExistingVote">ENoExistingVote</a>);
        <b>let</b> previous_vote = *table::borrow(voter_table, caller);
        <b>let</b> nominee = table::borrow_mut(&<b>mut</b> registry.nominated_delegates, target_address);
        <b>if</b> (previous_vote) {
            <b>assert</b>!(nominee.upvotes &gt; 0, <a href="../social_contracts/governance.md#social_contracts_governance_EOverflow">EOverflow</a>);
            nominee.upvotes = nominee.upvotes - 1;
        } <b>else</b> {
            <b>assert</b>!(nominee.downvotes &gt; 0, <a href="../social_contracts/governance.md#social_contracts_governance_EOverflow">EOverflow</a>);
            nominee.downvotes = nominee.downvotes - 1;
        };
        table::remove(voter_table, caller);
        upvote_count = nominee.upvotes;
        downvote_count = nominee.downvotes;
    } <b>else</b> {
        <b>abort</b> <a href="../social_contracts/governance.md#social_contracts_governance_ENotDelegate">ENotDelegate</a>
    };
    event::emit(<a href="../social_contracts/governance.md#social_contracts_governance_DelegateVoteClearedEvent">DelegateVoteClearedEvent</a> {
        target_address,
        voter: caller,
        is_active_delegate,
        new_upvote_count: upvote_count,
        new_downvote_count: downvote_count,
        <a href="../social_contracts/governance.md#social_contracts_governance_registry_type">registry_type</a>: registry.<a href="../social_contracts/governance.md#social_contracts_governance_registry_type">registry_type</a>,
    });
    <a href="../social_contracts/governance.md#social_contracts_governance_try_update_delegate_panel_if_due">try_update_delegate_panel_if_due</a>(registry, ctx);
}
</code></pre>



</details>

<a name="social_contracts_governance_try_update_delegate_panel_if_due"></a>

## Function `try_update_delegate_panel_if_due`

When <code>ctx.epoch()</code> has reached a completed term boundary that has not yet been applied,
recomputes the delegate council from incumbents and nominees. Otherwise returns immediately.


<pre><code><b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_try_update_delegate_panel_if_due">try_update_delegate_panel_if_due</a>(registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_try_update_delegate_panel_if_due">try_update_delegate_panel_if_due</a>(
    registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    ctx: &TxContext
) {
    // Check <a href="../social_contracts/governance.md#social_contracts_governance_version">version</a> compatibility
    <b>assert</b>!(registry.<a href="../social_contracts/governance.md#social_contracts_governance_version">version</a> == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/governance.md#social_contracts_governance_EWrongVersion">EWrongVersion</a>);
    <b>let</b> current_epoch = tx_context::epoch(ctx);
    <b>let</b> delegate_term_epochs = registry.delegate_term_epochs;
    <b>assert</b>!(delegate_term_epochs &gt; 0, <a href="../social_contracts/governance.md#social_contracts_governance_EInvalidParameter">EInvalidParameter</a>);
    <b>if</b> (current_epoch == 0) {
        <b>return</b>
    };
    <b>let</b> latest_completed_boundary =
        (current_epoch / delegate_term_epochs) * delegate_term_epochs;
    <b>if</b> (latest_completed_boundary == 0) {
        <b>return</b>
    };
    <b>if</b> (registry.<a href="../social_contracts/governance.md#social_contracts_governance_last_delegate_panel_boundary_epoch">last_delegate_panel_boundary_epoch</a> &gt;= latest_completed_boundary) {
        <b>return</b>
    };
    // --- Gather Candidates Data ---
    <b>let</b> <b>mut</b> candidate_addresses = vector::empty&lt;<b>address</b>&gt;();
    <b>let</b> <b>mut</b> candidate_upvotes = vector::empty&lt;u64&gt;();
    <b>let</b> <b>mut</b> candidate_downvotes = vector::empty&lt;u64&gt;();
    <b>let</b> <b>mut</b> candidate_net_votes = vector::empty&lt;u64&gt;(); // For sorting calculations
    <b>let</b> <b>mut</b> candidate_is_incumbent = vector::empty&lt;bool&gt;(); // Track incumbent status <b>for</b> tie-breaking
    // Use 1000 <b>as</b> the baseline score where upvotes and downvotes are equal
    <b>let</b> baseline_score: u64 = 1000;
    // 1. Gather Active Delegates (using the tracking set)
    <b>let</b> active_delegate_keys_vec = vec_set::into_keys(registry.delegate_addresses); // Consumes the set
    <b>let</b> len_active = vector::length(&active_delegate_keys_vec);
    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (i &lt; len_active) {
        <b>let</b> addr = *vector::borrow(&active_delegate_keys_vec, i);
        // Need to borrow from the actual table to get full data
        <b>if</b> (table::contains(&registry.delegates, addr)) { // Check existence before borrow
            <b>let</b> delegate: &<a href="../social_contracts/governance.md#social_contracts_governance_Delegate">Delegate</a> = table::borrow(&registry.delegates, addr);
            vector::push_back(&<b>mut</b> candidate_addresses, addr);
            vector::push_back(&<b>mut</b> candidate_upvotes, delegate.upvotes);
            vector::push_back(&<b>mut</b> candidate_downvotes, delegate.downvotes);
            vector::push_back(&<b>mut</b> candidate_is_incumbent, <b>true</b>); // Mark <b>as</b> incumbent
            // Calculate net votes with baseline offset (allows negative effective scores)
            <b>let</b> net_votes = <b>if</b> (delegate.upvotes &gt;= delegate.downvotes) {
                // If more upvotes than downvotes, add the difference to baseline
                baseline_score + (delegate.upvotes - delegate.downvotes)
            } <b>else</b> {
                // If more downvotes than upvotes, subtract the difference from baseline
                // But make sure we don't underflow
                <b>if</b> (delegate.downvotes - delegate.upvotes &gt;= baseline_score) {
                    0 // Minimum score
                } <b>else</b> {
                    baseline_score - (delegate.downvotes - delegate.upvotes)
                }
            };
            vector::push_back(&<b>mut</b> candidate_net_votes, net_votes);
        };
        // Note: If delegate wasn't found in table (somehow desynced), it's skipped.
        i = i + 1;
    };
    // Recreate the delegate_addresses set <b>as</b> empty, it will be repopulated with winners
    registry.delegate_addresses = vec_set::empty&lt;<b>address</b>&gt;();
    // 2. Gather Nominated Delegates (using the tracking set)
    <b>let</b> nominee_keys_vec = vec_set::into_keys(registry.nominee_addresses); // Consumes the set
    <b>let</b> len_nominated = vector::length(&nominee_keys_vec);
    <b>let</b> <b>mut</b> j = 0;
    <b>while</b> (j &lt; len_nominated) {
        <b>let</b> addr = *vector::borrow(&nominee_keys_vec, j);
        // Check <b>if</b> it was already added <b>as</b> an active delegate (shouldn't happen <b>if</b> sets are correct)
        // We can skip this check <b>if</b> we ensure sets are managed correctly.
        // Need to borrow from the actual table to get full data
        <b>if</b> (table::contains(&registry.nominated_delegates, addr)) { // Check existence
            <b>let</b> nominee: &<a href="../social_contracts/governance.md#social_contracts_governance_NominatedDelegate">NominatedDelegate</a> = table::borrow(&registry.nominated_delegates, addr);
            vector::push_back(&<b>mut</b> candidate_addresses, addr);
            vector::push_back(&<b>mut</b> candidate_upvotes, nominee.upvotes);
            vector::push_back(&<b>mut</b> candidate_downvotes, nominee.downvotes);
            vector::push_back(&<b>mut</b> candidate_is_incumbent, <b>false</b>); // Mark <b>as</b> non-incumbent
            // Calculate net votes with baseline offset (allows negative effective scores)
            <b>let</b> net_votes = <b>if</b> (nominee.upvotes &gt;= nominee.downvotes) {
                // If more upvotes than downvotes, add the difference to baseline
                baseline_score + (nominee.upvotes - nominee.downvotes)
            } <b>else</b> {
                // If more downvotes than upvotes, subtract the difference from baseline
                // But make sure we don't underflow
                <b>if</b> (nominee.downvotes - nominee.upvotes &gt;= baseline_score) {
                    0 // Minimum score
                } <b>else</b> {
                    baseline_score - (nominee.downvotes - nominee.upvotes)
                }
            };
            vector::push_back(&<b>mut</b> candidate_net_votes, net_votes);
        };
        j = j + 1;
    };
    // Recreate the nominee_addresses set <b>as</b> empty, losers will not be re-added.
    registry.nominee_addresses = vec_set::empty&lt;<b>address</b>&gt;();
    // --- Sort Candidate Data by net votes (higher is better) ---
    <b>let</b> candidate_count = vector::length(&candidate_addresses);
    <b>let</b> <b>mut</b> a = 0;
    <b>while</b> (a &lt; candidate_count) {
        <b>let</b> <b>mut</b> b = a + 1;
        <b>while</b> (b &lt; candidate_count) {
            // Compare net votes
            <b>let</b> a_votes = *vector::borrow(&candidate_net_votes, a);
            <b>let</b> b_votes = *vector::borrow(&candidate_net_votes, b);
            // If b <b>has</b> more net votes than a, or they're tied but b is an incumbent and a is not, swap them
            <b>let</b> should_swap = <b>if</b> (b_votes &gt; a_votes) {
                <b>true</b>
            } <b>else</b> <b>if</b> (b_votes == a_votes) {
                // Tie-breaking: prefer incumbent delegates
                <b>let</b> a_incumbent = *vector::borrow(&candidate_is_incumbent, a);
                <b>let</b> b_incumbent = *vector::borrow(&candidate_is_incumbent, b);
                // Only swap <b>if</b> b is incumbent and a is not
                b_incumbent && !a_incumbent
            } <b>else</b> {
                <b>false</b>
            };
            <b>if</b> (should_swap) {
                // Swap addresses
                <b>let</b> temp_addr = *vector::borrow(&candidate_addresses, a);
                *vector::borrow_mut(&<b>mut</b> candidate_addresses, a) = *vector::borrow(&candidate_addresses, b);
                *vector::borrow_mut(&<b>mut</b> candidate_addresses, b) = temp_addr;
                // Swap upvotes
                <b>let</b> temp_up = *vector::borrow(&candidate_upvotes, a);
                *vector::borrow_mut(&<b>mut</b> candidate_upvotes, a) = *vector::borrow(&candidate_upvotes, b);
                *vector::borrow_mut(&<b>mut</b> candidate_upvotes, b) = temp_up;
                // Swap downvotes
                <b>let</b> temp_down = *vector::borrow(&candidate_downvotes, a);
                *vector::borrow_mut(&<b>mut</b> candidate_downvotes, a) = *vector::borrow(&candidate_downvotes, b);
                *vector::borrow_mut(&<b>mut</b> candidate_downvotes, b) = temp_down;
                // Swap net votes
                <b>let</b> temp_net = *vector::borrow(&candidate_net_votes, a);
                *vector::borrow_mut(&<b>mut</b> candidate_net_votes, a) = *vector::borrow(&candidate_net_votes, b);
                *vector::borrow_mut(&<b>mut</b> candidate_net_votes, b) = temp_net;
                // Swap incumbent status
                <b>let</b> temp_inc = *vector::borrow(&candidate_is_incumbent, a);
                *vector::borrow_mut(&<b>mut</b> candidate_is_incumbent, a) = *vector::borrow(&candidate_is_incumbent, b);
                *vector::borrow_mut(&<b>mut</b> candidate_is_incumbent, b) = temp_inc;
            };
            b = b + 1;
        };
        a = a + 1;
    };
    // --- Select Winners and Update State ---
    <b>let</b> num_winners_to_select = registry.delegate_count;
    <b>let</b> actual_candidate_count = vector::length(&candidate_addresses);
    <b>let</b> final_winner_count = <b>if</b> (actual_candidate_count &lt; num_winners_to_select) { actual_candidate_count } <b>else</b> { num_winners_to_select };
    // --- Cleanup Old State ---
    // 1. Remove *all* previously active delegates from the table
    // We iterate using the keys vector we got earlier
    <b>let</b> <b>mut</b> k = 0;
    <b>while</b> (k &lt; len_active) {
        <b>let</b> addr = *vector::borrow(&active_delegate_keys_vec, k);
        // Check <b>if</b> it exists before removing (might have been removed <b>if</b> somehow duplicated)
        <b>if</b> (table::contains(&registry.delegates, addr)) {
             <b>let</b> old_delegate = table::remove(&<b>mut</b> registry.delegates, addr);
             <b>let</b> <a href="../social_contracts/governance.md#social_contracts_governance_Delegate">Delegate</a> { <b>address</b>: _, upvotes: _, downvotes: _, proposals_reviewed: _, proposals_submitted: _, sided_winning_proposals: _, sided_losing_proposals: _, term_start: _, term_end: _ } = old_delegate;
        };
        k = k + 1;
    };
    // 2. Remove *all* previously nominated delegates from the table
    <b>let</b> <b>mut</b> l = 0;
    <b>while</b> (l &lt; len_nominated) {
        <b>let</b> addr = *vector::borrow(&nominee_keys_vec, l);
        <b>if</b> (table::contains(&registry.nominated_delegates, addr)) {
            <b>let</b> old_nominee = table::remove(&<b>mut</b> registry.nominated_delegates, addr);
            <b>let</b> <a href="../social_contracts/governance.md#social_contracts_governance_NominatedDelegate">NominatedDelegate</a> { <b>address</b>: _, scheduled_term_start_epoch: _, upvotes: _, downvotes: _ } = old_nominee;
         };
         l = l + 1;
    };
    // --- Add Winners to the Delegates Table ---
    // The delegates table should now be empty. Add the winners.
    <b>let</b> <b>mut</b> m = 0;
    <b>while</b> (m &lt; final_winner_count) {
        <b>let</b> winner_addr = *vector::borrow(&candidate_addresses, m);
        <b>let</b> winner_upvotes = *vector::borrow(&candidate_upvotes, m);
        <b>let</b> winner_downvotes = *vector::borrow(&candidate_downvotes, m);
        <b>let</b> term_start = current_epoch;
        <b>let</b> term_end = term_start + delegate_term_epochs;
        <b>let</b> new_delegate = <a href="../social_contracts/governance.md#social_contracts_governance_Delegate">Delegate</a> {
            <b>address</b>: winner_addr,
            upvotes: winner_upvotes,
            downvotes: winner_downvotes,
            proposals_reviewed: 0, // Reset counters
            proposals_submitted: 0,
            sided_winning_proposals: 0,
            sided_losing_proposals: 0,
            term_start: term_start,
            term_end: term_end,
        };
        // Add winner to the (previously emptied) delegates table
        table::add(&<b>mut</b> registry.delegates, winner_addr, new_delegate);
        // Add winner's <b>address</b> back to the tracking set
        vec_set::insert(&<b>mut</b> registry.delegate_addresses, winner_addr);
        event::emit(<a href="../social_contracts/governance.md#social_contracts_governance_DelegateElectedEvent">DelegateElectedEvent</a> {
            delegate_address: winner_addr,
            term_start: term_start,
            term_end: term_end,
            <a href="../social_contracts/governance.md#social_contracts_governance_registry_type">registry_type</a>: registry.<a href="../social_contracts/governance.md#social_contracts_governance_registry_type">registry_type</a>,
            upvotes: winner_upvotes,
            downvotes: winner_downvotes,
        });
        m = m + 1;
    };
    registry.<a href="../social_contracts/governance.md#social_contracts_governance_last_delegate_panel_boundary_epoch">last_delegate_panel_boundary_epoch</a> = latest_completed_boundary;
    event::emit(<a href="../social_contracts/governance.md#social_contracts_governance_DelegatePanelRefreshedEvent">DelegatePanelRefreshedEvent</a> {
        registry_id: object::id(registry),
        boundary_epoch: latest_completed_boundary,
        <a href="../social_contracts/governance.md#social_contracts_governance_registry_type">registry_type</a>: registry.<a href="../social_contracts/governance.md#social_contracts_governance_registry_type">registry_type</a>,
        executed_at_epoch: current_epoch,
    });
    // Drop helper vectors (still contain sorted candidates / keys after winner pass)
    candidate_addresses.destroy!(|_| {});
    candidate_upvotes.destroy!(|_| {});
    candidate_downvotes.destroy!(|_| {});
    candidate_net_votes.destroy!(|_| {});
    candidate_is_incumbent.destroy!(|_| {});
    active_delegate_keys_vec.destroy!(|_| {});
    nominee_keys_vec.destroy!(|_| {});
}
</code></pre>



</details>

<a name="social_contracts_governance_update_delegate_panel"></a>

## Function `update_delegate_panel`

Updates delegate panel at the end of a delegate term cycle (keeper / manual backstop).


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_update_delegate_panel">update_delegate_panel</a>(registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_update_delegate_panel">update_delegate_panel</a>(
    registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    ctx: &<b>mut</b> TxContext
) {
    <a href="../social_contracts/governance.md#social_contracts_governance_try_update_delegate_panel_if_due">try_update_delegate_panel_if_due</a>(registry, ctx);
}
</code></pre>



</details>

<a name="social_contracts_governance_submit_proposal"></a>

## Function `submit_proposal`

Universal function to submit any type of proposal
Handles proposal types: ecosystem and proof of creativity


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_submit_proposal">submit_proposal</a>(registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, proposal_type: u8, title: <a href="../std/string.md#std_string_String">std::string::String</a>, description: <a href="../std/string.md#std_string_String">std::string::String</a>, disputed_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;, reference_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;, metadata_json: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, coin: &<b>mut</b> <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_submit_proposal">submit_proposal</a>(
    registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    proposal_type: u8,
    title: String,
    description: String,
    disputed_id: Option&lt;ID&gt;, // Optional ID <b>for</b> disputes (content only)
    reference_id: Option&lt;ID&gt;, // Optional reference
    metadata_json: Option&lt;String&gt;,
    coin: &<b>mut</b> Coin&lt;MYSO&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    // Check <a href="../social_contracts/governance.md#social_contracts_governance_version">version</a> compatibility
    <b>assert</b>!(registry.<a href="../social_contracts/governance.md#social_contracts_governance_version">version</a> == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/governance.md#social_contracts_governance_EWrongVersion">EWrongVersion</a>);
    // Verify proposal type is valid
    <b>assert</b>!(proposal_type &lt;= <a href="../social_contracts/governance.md#social_contracts_governance_PROPOSAL_TYPE_PLATFORM">PROPOSAL_TYPE_PLATFORM</a>, <a href="../social_contracts/governance.md#social_contracts_governance_EInvalidParameter">EInvalidParameter</a>);
    // Verify registry type matches proposal type
    <b>assert</b>!(registry.<a href="../social_contracts/governance.md#social_contracts_governance_registry_type">registry_type</a> == proposal_type, <a href="../social_contracts/governance.md#social_contracts_governance_EInvalidRegistry">EInvalidRegistry</a>);
    // Handle reference ID based on proposal type
    <b>let</b> actual_reference_id = <b>if</b> (proposal_type == <a href="../social_contracts/governance.md#social_contracts_governance_PROPOSAL_TYPE_ECOSYSTEM">PROPOSAL_TYPE_ECOSYSTEM</a>) {
        // Ecosystem proposals <b>use</b> reference_id <b>as</b> provided
        reference_id
    } <b>else</b> <b>if</b> (proposal_type == <a href="../social_contracts/governance.md#social_contracts_governance_PROPOSAL_TYPE_PROOF_OF_CREATIVITY">PROPOSAL_TYPE_PROOF_OF_CREATIVITY</a>) {
        // Proof of creativity proposals should have either reference_id or disputed_id
        <b>if</b> (option::is_some(&reference_id)) {
            reference_id
        } <b>else</b> <b>if</b> (option::is_some(&disputed_id)) {
            disputed_id
        } <b>else</b> {
            // Proof of creativity proposals should reference creative content
            <b>assert</b>!(<b>false</b>, <a href="../social_contracts/governance.md#social_contracts_governance_EInvalidParameter">EInvalidParameter</a>);
            option::none&lt;ID&gt;()
        }
    } <b>else</b> <b>if</b> (proposal_type == <a href="../social_contracts/governance.md#social_contracts_governance_PROPOSAL_TYPE_SPOT">PROPOSAL_TYPE_SPOT</a>) {
        <b>assert</b>!(option::is_some(&reference_id), <a href="../social_contracts/governance.md#social_contracts_governance_EInvalidParameter">EInvalidParameter</a>);
        reference_id
    } <b>else</b> {
        // For other proposal types (like <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>), <b>use</b> reference_id <b>if</b> provided
        reference_id
    };
    <b>let</b> _proposal_id = <a href="../social_contracts/governance.md#social_contracts_governance_submit_proposal_internal">submit_proposal_internal</a>(
        registry,
        title,
        description,
        proposal_type,
        actual_reference_id,
        metadata_json,
        coin,
        clock,
        ctx
    );
}
</code></pre>



</details>

<a name="social_contracts_governance_submit_ecosystem_proposal"></a>

## Function `submit_ecosystem_proposal`

Submit a new proposal to the ecosystem registry
Requires staking MYSO tokens equal to the proposal submission cost


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_submit_ecosystem_proposal">submit_ecosystem_proposal</a>(registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, title: <a href="../std/string.md#std_string_String">std::string::String</a>, description: <a href="../std/string.md#std_string_String">std::string::String</a>, reference_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;, metadata_json: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, coin: &<b>mut</b> <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_submit_ecosystem_proposal">submit_ecosystem_proposal</a>(
    registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    title: String,
    description: String,
    reference_id: Option&lt;ID&gt;,
    metadata_json: Option&lt;String&gt;,
    coin: &<b>mut</b> Coin&lt;MYSO&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <a href="../social_contracts/governance.md#social_contracts_governance_submit_proposal">submit_proposal</a>(
        registry,
        <a href="../social_contracts/governance.md#social_contracts_governance_PROPOSAL_TYPE_ECOSYSTEM">PROPOSAL_TYPE_ECOSYSTEM</a>,
        title,
        description,
        option::none&lt;ID&gt;(), // No disputed ID <b>for</b> ecosystem proposals
        reference_id,
        metadata_json,
        coin,
        clock,
        ctx
    );
}
</code></pre>



</details>

<a name="social_contracts_governance_submit_proof_of_creativity_proposal"></a>

## Function `submit_proof_of_creativity_proposal`

Submit a proof of creativity proposal


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_submit_proof_of_creativity_proposal">submit_proof_of_creativity_proposal</a>(registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, title: <a href="../std/string.md#std_string_String">std::string::String</a>, description: <a href="../std/string.md#std_string_String">std::string::String</a>, creative_content_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, metadata_json: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, coin: &<b>mut</b> <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_submit_proof_of_creativity_proposal">submit_proof_of_creativity_proposal</a>(
    registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    title: String,
    description: String,
    creative_content_id: ID,
    metadata_json: Option&lt;String&gt;,
    coin: &<b>mut</b> Coin&lt;MYSO&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <a href="../social_contracts/governance.md#social_contracts_governance_submit_proposal">submit_proposal</a>(
        registry,
        <a href="../social_contracts/governance.md#social_contracts_governance_PROPOSAL_TYPE_PROOF_OF_CREATIVITY">PROPOSAL_TYPE_PROOF_OF_CREATIVITY</a>,
        title,
        description,
        option::none&lt;ID&gt;(), // No disputed ID <b>for</b> proof of creativity
        option::some(creative_content_id), // Reference to creative content
        metadata_json,
        coin,
        clock,
        ctx
    );
}
</code></pre>



</details>

<a name="social_contracts_governance_submit_spot_resolution_proposal"></a>

## Function `submit_spot_resolution_proposal`

Submit a SPoT resolution proposal (binary ratification of one outcome for a contested market).


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_submit_spot_resolution_proposal">submit_spot_resolution_proposal</a>(registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, title: <a href="../std/string.md#std_string_String">std::string::String</a>, description: <a href="../std/string.md#std_string_String">std::string::String</a>, spot_record_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, metadata_json: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, coin: &<b>mut</b> <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_submit_spot_resolution_proposal">submit_spot_resolution_proposal</a>(
    registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    title: String,
    description: String,
    spot_record_id: ID,
    metadata_json: Option&lt;String&gt;,
    coin: &<b>mut</b> Coin&lt;MYSO&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(registry.<a href="../social_contracts/governance.md#social_contracts_governance_registry_type">registry_type</a> == <a href="../social_contracts/governance.md#social_contracts_governance_PROPOSAL_TYPE_SPOT">PROPOSAL_TYPE_SPOT</a>, <a href="../social_contracts/governance.md#social_contracts_governance_EInvalidRegistry">EInvalidRegistry</a>);
    <b>let</b> _ = <a href="../social_contracts/governance.md#social_contracts_governance_submit_spot_proposal_and_return_id">submit_spot_proposal_and_return_id</a>(
        registry,
        title,
        description,
        spot_record_id,
        metadata_json,
        coin,
        clock,
        ctx
    );
}
</code></pre>



</details>

<a name="social_contracts_governance_submit_spot_proposal_and_return_id"></a>

## Function `submit_spot_proposal_and_return_id`

Package helper: submit a SPoT resolution proposal and return its ID.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_submit_spot_proposal_and_return_id">submit_spot_proposal_and_return_id</a>(registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, title: <a href="../std/string.md#std_string_String">std::string::String</a>, description: <a href="../std/string.md#std_string_String">std::string::String</a>, spot_record_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, metadata_json: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, coin: &<b>mut</b> <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_submit_spot_proposal_and_return_id">submit_spot_proposal_and_return_id</a>(
    registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    title: String,
    description: String,
    spot_record_id: ID,
    metadata_json: Option&lt;String&gt;,
    coin: &<b>mut</b> Coin&lt;MYSO&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
): ID {
    <b>assert</b>!(registry.<a href="../social_contracts/governance.md#social_contracts_governance_registry_type">registry_type</a> == <a href="../social_contracts/governance.md#social_contracts_governance_PROPOSAL_TYPE_SPOT">PROPOSAL_TYPE_SPOT</a>, <a href="../social_contracts/governance.md#social_contracts_governance_EInvalidRegistry">EInvalidRegistry</a>);
    <a href="../social_contracts/governance.md#social_contracts_governance_submit_proposal_internal">submit_proposal_internal</a>(
        registry,
        title,
        description,
        <a href="../social_contracts/governance.md#social_contracts_governance_PROPOSAL_TYPE_SPOT">PROPOSAL_TYPE_SPOT</a>,
        option::some(spot_record_id),
        metadata_json,
        coin,
        clock,
        ctx
    )
}
</code></pre>



</details>

<a name="social_contracts_governance_submit_poc_proposal_and_return_id"></a>

## Function `submit_poc_proposal_and_return_id`

Package helper: submit a PoC governance proposal and return its ID.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_submit_poc_proposal_and_return_id">submit_poc_proposal_and_return_id</a>(registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, title: <a href="../std/string.md#std_string_String">std::string::String</a>, description: <a href="../std/string.md#std_string_String">std::string::String</a>, reference_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, metadata_json: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, coin: &<b>mut</b> <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_submit_poc_proposal_and_return_id">submit_poc_proposal_and_return_id</a>(
    registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    title: String,
    description: String,
    reference_id: ID,
    metadata_json: Option&lt;String&gt;,
    coin: &<b>mut</b> Coin&lt;MYSO&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
): ID {
    <b>assert</b>!(registry.<a href="../social_contracts/governance.md#social_contracts_governance_registry_type">registry_type</a> == <a href="../social_contracts/governance.md#social_contracts_governance_PROPOSAL_TYPE_PROOF_OF_CREATIVITY">PROPOSAL_TYPE_PROOF_OF_CREATIVITY</a>, <a href="../social_contracts/governance.md#social_contracts_governance_EInvalidRegistry">EInvalidRegistry</a>);
    <a href="../social_contracts/governance.md#social_contracts_governance_submit_proposal_internal">submit_proposal_internal</a>(
        registry,
        title,
        description,
        <a href="../social_contracts/governance.md#social_contracts_governance_PROPOSAL_TYPE_PROOF_OF_CREATIVITY">PROPOSAL_TYPE_PROOF_OF_CREATIVITY</a>,
        option::some(reference_id),
        metadata_json,
        coin,
        clock,
        ctx,
    )
}
</code></pre>



</details>

<a name="social_contracts_governance_submit_proposal_internal"></a>

## Function `submit_proposal_internal`

Internal function for submitting proposals


<pre><code><b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_submit_proposal_internal">submit_proposal_internal</a>(registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, title: <a href="../std/string.md#std_string_String">std::string::String</a>, description: <a href="../std/string.md#std_string_String">std::string::String</a>, proposal_type: u8, reference_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;, metadata_json: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, coin: &<b>mut</b> <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_submit_proposal_internal">submit_proposal_internal</a>(
    registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    title: String,
    description: String,
    proposal_type: u8,
    reference_id: Option&lt;ID&gt;,
    metadata_json: Option&lt;String&gt;,
    coin: &<b>mut</b> Coin&lt;MYSO&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
): ID {
    <b>let</b> caller = tx_context::sender(ctx);
    <b>let</b> current_time = clock::timestamp_ms(clock);
    // Check stake amount
    <b>assert</b>!(coin::value(coin) &gt;= registry.<a href="../social_contracts/governance.md#social_contracts_governance_proposal_submission_cost">proposal_submission_cost</a>, <a href="../social_contracts/governance.md#social_contracts_governance_EInvalidAmount">EInvalidAmount</a>);
    // Create proposal
    <b>let</b> proposal_id = object::new(ctx);
    <b>let</b> <b>mut</b> proposal = <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">Proposal</a> {
        id: proposal_id,
        title: title,
        description: description,
        proposal_type: proposal_type,
        reference_id: reference_id,
        metadata_json: metadata_json,
        submitter: caller,
        submission_time: current_time,
        delegate_approval_count: 0,
        delegate_rejection_count: 0,
        community_votes_for: 0,
        community_votes_against: 0,
        status: <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_DELEGATE_REVIEW">STATUS_DELEGATE_REVIEW</a>,
        voting_start_time: 0,
        voting_end_time: 0,
        reward_pool: balance::zero(),
    };
    // Split coin and add to proposal's reward pool
    <b>let</b> proposal_coin = coin::split(coin, registry.<a href="../social_contracts/governance.md#social_contracts_governance_proposal_submission_cost">proposal_submission_cost</a>, ctx);
    balance::join(&<b>mut</b> proposal.reward_pool, coin::into_balance(proposal_coin));
    // Initialize dynamic fields <b>for</b> vote tracking
    <b>let</b> delegate_votes = table::new&lt;<b>address</b>, bool&gt;(ctx);
    dynamic_field::add(&<b>mut</b> proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_DELEGATE_VOTES_FIELD">DELEGATE_VOTES_FIELD</a>, delegate_votes);
    <b>let</b> voted_community = vec_set::empty&lt;<b>address</b>&gt;();
    dynamic_field::add(&<b>mut</b> proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_VOTED_COMMUNITY_FIELD">VOTED_COMMUNITY_FIELD</a>, voted_community);
    // Share proposal <b>as</b> shared object and register in registry
    <b>let</b> proposal_id_copy = object::id(&proposal);
    transfer::share_object(proposal);
    table::add(&<b>mut</b> registry.proposals, proposal_id_copy, <b>true</b>);
    table::add(&<b>mut</b> registry.proposal_types, proposal_id_copy, proposal_type);
    // Add to proposals by status
    <b>let</b> proposals_of_status = table::borrow_mut(&<b>mut</b> registry.proposals_by_status, <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_DELEGATE_REVIEW">STATUS_DELEGATE_REVIEW</a>);
    vector::push_back(proposals_of_status, proposal_id_copy);
    // Increment proposal count <b>for</b> delegate <b>if</b> applicable
    <b>if</b> (table::contains(&registry.delegates, caller)) {
        <b>let</b> delegate = table::borrow_mut(&<b>mut</b> registry.delegates, caller);
        delegate.proposals_submitted = delegate.proposals_submitted + 1;
    };
    // Emit event
    event::emit(<a href="../social_contracts/governance.md#social_contracts_governance_ProposalSubmittedEvent">ProposalSubmittedEvent</a> {
        proposal_id: proposal_id_copy,
        title: title,
        description: description,
        proposal_type: proposal_type,
        reference_id: reference_id,
        metadata_json: metadata_json,
        submitter: caller,
        reward_amount: registry.<a href="../social_contracts/governance.md#social_contracts_governance_proposal_submission_cost">proposal_submission_cost</a>,
        submission_time: current_time,
    });
    <a href="../social_contracts/governance.md#social_contracts_governance_try_update_delegate_panel_if_due">try_update_delegate_panel_if_due</a>(registry, ctx);
    proposal_id_copy
}
</code></pre>



</details>

<a name="social_contracts_governance_rescind_proposal"></a>

## Function `rescind_proposal`

Allow a proposal owner to rescind their proposal if it's still in the delegate review stage


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_rescind_proposal">rescind_proposal</a>(registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">social_contracts::governance::Proposal</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_rescind_proposal">rescind_proposal</a>(
    registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">Proposal</a>,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    // Check <a href="../social_contracts/governance.md#social_contracts_governance_version">version</a> compatibility
    <b>assert</b>!(registry.<a href="../social_contracts/governance.md#social_contracts_governance_version">version</a> == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/governance.md#social_contracts_governance_EWrongVersion">EWrongVersion</a>);
    <b>let</b> caller = tx_context::sender(ctx);
    <b>let</b> current_time = clock::timestamp_ms(clock);
    <b>let</b> proposal_id = object::id(proposal);
    // Verify proposal exists and is in delegate review phase
    <b>assert</b>!(table::contains(&registry.proposals, proposal_id), <a href="../social_contracts/governance.md#social_contracts_governance_EProposalNotFound">EProposalNotFound</a>);
    // Verify caller is the proposal submitter
    <b>assert</b>!(proposal.submitter == caller, <a href="../social_contracts/governance.md#social_contracts_governance_EUnauthorized">EUnauthorized</a>);
    // Verify proposal is still in delegate review stage
    <b>assert</b>!(proposal.status == <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_DELEGATE_REVIEW">STATUS_DELEGATE_REVIEW</a>, <a href="../social_contracts/governance.md#social_contracts_governance_EInvalidProposalStatus">EInvalidProposalStatus</a>);
    // Once a <a href="../social_contracts/jury.md#social_contracts_jury">jury</a> <b>has</b> the proposal the submitter cannot withdraw it (and the remaining pool)
    // to dodge the verdict.
    <b>assert</b>!(!<a href="../social_contracts/governance.md#social_contracts_governance_council_jury_has_jurisdiction">council_jury_has_jurisdiction</a>(proposal), <a href="../social_contracts/governance.md#social_contracts_governance_EJuryHasJurisdiction">EJuryHasJurisdiction</a>);
    // Update proposals by status - remove from delegate review
    <b>let</b> from_status = table::borrow_mut(&<b>mut</b> registry.proposals_by_status, <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_DELEGATE_REVIEW">STATUS_DELEGATE_REVIEW</a>);
    <b>let</b> <b>mut</b> index = 0;
    <b>let</b> len = vector::length(from_status);
    <b>while</b> (index &lt; len) {
        <b>if</b> (*vector::borrow(from_status, index) == proposal_id) {
            vector::remove(from_status, index);
            <b>break</b>
        };
        index = index + 1;
    };
    // Return staked tokens to submitter
    <b>let</b> refund_amount = balance::value(&proposal.reward_pool);
    <b>if</b> (refund_amount &gt; 0) {
        <b>let</b> refund_coin = coin::from_balance(balance::withdraw_all(&<b>mut</b> proposal.reward_pool), ctx);
        transfer::public_transfer(refund_coin, caller);
    } <b>else</b> {
        // Empty the balance even <b>if</b> zero, <b>for</b> consistency
        balance::destroy_zero(balance::withdraw_all(&<b>mut</b> proposal.reward_pool));
    };
    // Emit event <b>for</b> the rescinded proposal
    event::emit(<a href="../social_contracts/governance.md#social_contracts_governance_ProposalRescindedEvent">ProposalRescindedEvent</a> {
        proposal_id,
        submitter: caller,
        rescind_time: current_time,
        refund_amount,
    });
    // Update proposal status to owner rescinded
    proposal.status = <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_OWNER_RESCIND">STATUS_OWNER_RESCIND</a>;
    // Add to owner rescinded list
    <b>let</b> to_status = table::borrow_mut(&<b>mut</b> registry.proposals_by_status, <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_OWNER_RESCIND">STATUS_OWNER_RESCIND</a>);
    vector::push_back(to_status, proposal_id);
}
</code></pre>



</details>

<a name="social_contracts_governance_move_to_community_voting"></a>

## Function `move_to_community_voting`

Move a proposal to community voting phase


<pre><code><b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_move_to_community_voting">move_to_community_voting</a>(registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">social_contracts::governance::Proposal</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, _ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_move_to_community_voting">move_to_community_voting</a>(
    registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">Proposal</a>,
    clock: &Clock,
    _ctx: &TxContext
) {
    <b>let</b> proposal_id = object::id(proposal);
    // Update status
    proposal.status = <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_COMMUNITY_VOTING">STATUS_COMMUNITY_VOTING</a>;
    // Get current timestamp in milliseconds <b>for</b> voting period calculation
    <b>let</b> current_time_ms = clock::timestamp_ms(clock);
    // Set voting period based on registry voting period (using milliseconds)
    proposal.voting_start_time = current_time_ms;
    proposal.voting_end_time = current_time_ms + registry.voting_period_ms;
    // Update proposals by status
    <b>let</b> from_status = table::borrow_mut(&<b>mut</b> registry.proposals_by_status, <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_DELEGATE_REVIEW">STATUS_DELEGATE_REVIEW</a>);
    <b>let</b> <b>mut</b> index = 0;
    <b>let</b> len = vector::length(from_status);
    <b>while</b> (index &lt; len) {
        <b>if</b> (*vector::borrow(from_status, index) == proposal_id) {
            vector::remove(from_status, index);
            <b>break</b>
        };
        index = index + 1;
    };
    <b>let</b> to_status = table::borrow_mut(&<b>mut</b> registry.proposals_by_status, <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_COMMUNITY_VOTING">STATUS_COMMUNITY_VOTING</a>);
    vector::push_back(to_status, proposal_id);
    // Emit event - <b>use</b> millisecond timestamps
    event::emit(<a href="../social_contracts/governance.md#social_contracts_governance_ProposalApprovedForVotingEvent">ProposalApprovedForVotingEvent</a> {
        proposal_id,
        voting_start_time: current_time_ms,
        voting_end_time: current_time_ms + registry.voting_period_ms,
    });
}
</code></pre>



</details>

<a name="social_contracts_governance_community_vote_on_proposal"></a>

## Function `community_vote_on_proposal`

Community vote on a proposal with quadratic voting
Users can cast multiple votes by paying a quadratically increasing cost. The sender must own a profile.
Platform registries must use <code><a href="../social_contracts/platform.md#social_contracts_platform_community_vote_on_platform_governance_proposal">platform::community_vote_on_platform_governance_proposal</a></code>.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_community_vote_on_proposal">community_vote_on_proposal</a>(registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, username_registry: &<a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">social_contracts::profile::UsernameRegistry</a>, proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">social_contracts::governance::Proposal</a>, vote_count: u64, approve: bool, coin: &<b>mut</b> <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_community_vote_on_proposal">community_vote_on_proposal</a>(
    registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    username_registry: &UsernameRegistry,
    proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">Proposal</a>,
    vote_count: u64,
    approve: bool,
    coin: &<b>mut</b> Coin&lt;MYSO&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(registry.<a href="../social_contracts/governance.md#social_contracts_governance_registry_type">registry_type</a> != <a href="../social_contracts/governance.md#social_contracts_governance_PROPOSAL_TYPE_PLATFORM">PROPOSAL_TYPE_PLATFORM</a>, <a href="../social_contracts/governance.md#social_contracts_governance_EUsePlatformVoteEntry">EUsePlatformVoteEntry</a>);
    <a href="../social_contracts/governance.md#social_contracts_governance_community_vote_internal">community_vote_internal</a>(registry, username_registry, proposal, vote_count, approve, coin, clock, ctx)
}
</code></pre>



</details>

<a name="social_contracts_governance_community_vote_internal"></a>

## Function `community_vote_internal`

Enforces the profile requirement; callers must enforce any registry-specific voter
eligibility (platform membership / blocks).


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_community_vote_internal">community_vote_internal</a>(registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, username_registry: &<a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">social_contracts::profile::UsernameRegistry</a>, proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">social_contracts::governance::Proposal</a>, vote_count: u64, approve: bool, coin: &<b>mut</b> <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_community_vote_internal">community_vote_internal</a>(
    registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    username_registry: &UsernameRegistry,
    proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">Proposal</a>,
    vote_count: u64,
    approve: bool,
    coin: &<b>mut</b> Coin&lt;MYSO&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    // Check <a href="../social_contracts/governance.md#social_contracts_governance_version">version</a> compatibility
    <b>assert</b>!(registry.<a href="../social_contracts/governance.md#social_contracts_governance_version">version</a> == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/governance.md#social_contracts_governance_EWrongVersion">EWrongVersion</a>);
    <b>let</b> caller = tx_context::sender(ctx);
    <a href="../social_contracts/governance.md#social_contracts_governance_assert_has_profile">assert_has_profile</a>(username_registry, caller);
    <b>let</b> current_time_ms = clock::timestamp_ms(clock);
    <b>let</b> proposal_id = object::id(proposal);
    // Calculate vote cost before borrowing from registry
    <b>let</b> quadratic_base_cost = registry.quadratic_base_cost;
    <b>let</b> vote_cost = <b>if</b> (vote_count &lt;= 1) {
        0
    } <b>else</b> {
        // Quadratic cost formula: base_cost * (vote_count^2 - 1)
        quadratic_base_cost * ((vote_count * vote_count) - 1)
    };
    // Verify proposal exists and is in community voting phase
    <b>assert</b>!(table::contains(&registry.proposals, proposal_id), <a href="../social_contracts/governance.md#social_contracts_governance_EProposalNotFound">EProposalNotFound</a>);
    <b>assert</b>!(proposal.status == <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_COMMUNITY_VOTING">STATUS_COMMUNITY_VOTING</a>, <a href="../social_contracts/governance.md#social_contracts_governance_ENotVotingPhase">ENotVotingPhase</a>);
    // Verify voting period hasn't ended (check using milliseconds)
    <b>assert</b>!(current_time_ms &lt;= proposal.voting_end_time, <a href="../social_contracts/governance.md#social_contracts_governance_EVotingPeriodEnded">EVotingPeriodEnded</a>);
    // Verify vote count is valid (at least 1, no more than max)
    <b>assert</b>!(vote_count &gt; 0, <a href="../social_contracts/governance.md#social_contracts_governance_EInvalidVoteCount">EInvalidVoteCount</a>);
    <b>assert</b>!(vote_count &lt;= registry.max_votes_per_user, <a href="../social_contracts/governance.md#social_contracts_governance_EExceedsMaxVotes">EExceedsMaxVotes</a>);
    // Check <b>if</b> user <b>has</b> already voted
    <b>let</b> voted_community: &<b>mut</b> VecSet&lt;<b>address</b>&gt; = dynamic_field::borrow_mut(&<b>mut</b> proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_VOTED_COMMUNITY_FIELD">VOTED_COMMUNITY_FIELD</a>);
    <b>assert</b>!(!vec_set::contains(voted_community, &caller), <a href="../social_contracts/governance.md#social_contracts_governance_EAlreadyVoted">EAlreadyVoted</a>);
    // Check <b>if</b> user <b>has</b> enough funds <b>for</b> votes
    <b>if</b> (vote_cost &gt; 0) {
        <b>assert</b>!(coin::value(coin) &gt;= vote_cost, <a href="../social_contracts/governance.md#social_contracts_governance_EInvalidAmount">EInvalidAmount</a>);
        // Split coin and add to proposal's reward pool <b>for</b> the additional votes
        <b>let</b> vote_coin = coin::split(coin, vote_cost, ctx);
        balance::join(&<b>mut</b> proposal.reward_pool, coin::into_balance(vote_coin));
    };
    // Record vote
    vec_set::insert(voted_community, caller);
    <b>if</b> (vote_cost &gt; 0) {
        <a href="../social_contracts/governance.md#social_contracts_governance_record_vote_payment">record_vote_payment</a>(proposal, caller, vote_cost, ctx);
    };
    // Update vote tally
    <b>if</b> (approve) {
        proposal.community_votes_for = proposal.community_votes_for + vote_count;
    } <b>else</b> {
        proposal.community_votes_against = proposal.community_votes_against + vote_count;
    };
    // Emit event (<b>use</b> millisecond timestamp <b>for</b> the event record)
    event::emit(<a href="../social_contracts/governance.md#social_contracts_governance_CommunityVoteEvent">CommunityVoteEvent</a> {
        proposal_id,
        voter: caller,
        vote_weight: vote_count,
        approve,
        vote_time: current_time_ms,
        vote_cost,
    });
}
</code></pre>



</details>

<a name="social_contracts_governance_community_vote_anonymous"></a>

## Function `community_vote_anonymous`

Submit an anonymous encrypted vote on a proposal. The sender must own a profile.
Platform registries must use <code><a href="../social_contracts/platform.md#social_contracts_platform_community_vote_anonymous_on_platform_governance_proposal">platform::community_vote_anonymous_on_platform_governance_proposal</a></code>.


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_community_vote_anonymous">community_vote_anonymous</a>(registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, username_registry: &<a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">social_contracts::profile::UsernameRegistry</a>, proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">social_contracts::governance::Proposal</a>, encrypted_vote: <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_EncryptedObject">mydata::bf_hmac_encryption::EncryptedObject</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_community_vote_anonymous">community_vote_anonymous</a>(
    registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    username_registry: &UsernameRegistry,
    proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">Proposal</a>,
    encrypted_vote: EncryptedObject,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(registry.<a href="../social_contracts/governance.md#social_contracts_governance_registry_type">registry_type</a> != <a href="../social_contracts/governance.md#social_contracts_governance_PROPOSAL_TYPE_PLATFORM">PROPOSAL_TYPE_PLATFORM</a>, <a href="../social_contracts/governance.md#social_contracts_governance_EUsePlatformVoteEntry">EUsePlatformVoteEntry</a>);
    <a href="../social_contracts/governance.md#social_contracts_governance_community_vote_anonymous_internal">community_vote_anonymous_internal</a>(registry, username_registry, proposal, encrypted_vote, clock, ctx)
}
</code></pre>



</details>

<a name="social_contracts_governance_community_vote_anonymous_internal"></a>

## Function `community_vote_anonymous_internal`

Enforces the profile requirement; callers must enforce any registry-specific voter
eligibility (platform membership / blocks).


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_community_vote_anonymous_internal">community_vote_anonymous_internal</a>(registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, username_registry: &<a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">social_contracts::profile::UsernameRegistry</a>, proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">social_contracts::governance::Proposal</a>, encrypted_vote: <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_EncryptedObject">mydata::bf_hmac_encryption::EncryptedObject</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_community_vote_anonymous_internal">community_vote_anonymous_internal</a>(
    registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    username_registry: &UsernameRegistry,
    proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">Proposal</a>,
    encrypted_vote: EncryptedObject,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(registry.<a href="../social_contracts/governance.md#social_contracts_governance_version">version</a> == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/governance.md#social_contracts_governance_EWrongVersion">EWrongVersion</a>);
    <b>let</b> caller = tx_context::sender(ctx);
    <a href="../social_contracts/governance.md#social_contracts_governance_assert_has_profile">assert_has_profile</a>(username_registry, caller);
    <b>let</b> current_time_ms = clock::timestamp_ms(clock);
    <b>let</b> proposal_id = object::id(proposal);
    <b>assert</b>!(table::contains(&registry.proposals, proposal_id), <a href="../social_contracts/governance.md#social_contracts_governance_EProposalNotFound">EProposalNotFound</a>);
    <b>assert</b>!(proposal.status == <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_COMMUNITY_VOTING">STATUS_COMMUNITY_VOTING</a>, <a href="../social_contracts/governance.md#social_contracts_governance_ENotVotingPhase">ENotVotingPhase</a>);
    <b>assert</b>!(current_time_ms &lt;= proposal.voting_end_time, <a href="../social_contracts/governance.md#social_contracts_governance_EVotingPeriodEnded">EVotingPeriodEnded</a>);
    <b>assert</b>!(!table::contains(&registry.delegates, caller), <a href="../social_contracts/governance.md#social_contracts_governance_EDelegateAnonNotAllowed">EDelegateAnonNotAllowed</a>);
    <b>let</b> voted_community: &<b>mut</b> VecSet&lt;<b>address</b>&gt; = dynamic_field::borrow_mut(&<b>mut</b> proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_VOTED_COMMUNITY_FIELD">VOTED_COMMUNITY_FIELD</a>);
    <b>assert</b>!(!vec_set::contains(voted_community, &caller), <a href="../social_contracts/governance.md#social_contracts_governance_EAlreadyVoted">EAlreadyVoted</a>);
    vec_set::insert(voted_community, caller);
    <b>if</b> (!dynamic_field::exists_(&proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_ENCRYPTED_VOTES_FIELD">ENCRYPTED_VOTES_FIELD</a>)) {
        <b>let</b> tbl = table::new&lt;<b>address</b>, EncryptedObject&gt;(ctx);
        dynamic_field::add(&<b>mut</b> proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_ENCRYPTED_VOTES_FIELD">ENCRYPTED_VOTES_FIELD</a>, tbl);
    };
    <b>let</b> enc_tbl: &<b>mut</b> Table&lt;<b>address</b>, EncryptedObject&gt; = dynamic_field::borrow_mut(&<b>mut</b> proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_ENCRYPTED_VOTES_FIELD">ENCRYPTED_VOTES_FIELD</a>);
    table::add(enc_tbl, caller, encrypted_vote);
    <b>if</b> (!dynamic_field::exists_(&proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_ANON_VOTERS_FIELD">ANON_VOTERS_FIELD</a>)) {
        <b>let</b> set = vec_set::empty&lt;<b>address</b>&gt;();
        dynamic_field::add(&<b>mut</b> proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_ANON_VOTERS_FIELD">ANON_VOTERS_FIELD</a>, set);
    };
    <b>let</b> anon_set: &<b>mut</b> VecSet&lt;<b>address</b>&gt; = dynamic_field::borrow_mut(&<b>mut</b> proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_ANON_VOTERS_FIELD">ANON_VOTERS_FIELD</a>);
    vec_set::insert(anon_set, caller);
    // Serialize the entire EncryptedObject <b>for</b> indexer storage
    <b>let</b> <b>mut</b> serialized_vote = vector::empty&lt;u8&gt;();
    serialized_vote.append(*encrypted_vote.blob());
    event::emit(<a href="../social_contracts/governance.md#social_contracts_governance_AnonymousVoteEvent">AnonymousVoteEvent</a> {
        proposal_id,
        voter: caller,
        vote_time: current_time_ms,
        encrypted_vote_data: serialized_vote, // Emit encrypted blob <b>for</b> indexer
    });
}
</code></pre>



</details>

<a name="social_contracts_governance_community_finalize_voting_approved_value"></a>

## Function `community_finalize_voting_approved_value`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_community_finalize_voting_approved_value">community_finalize_voting_approved_value</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_community_finalize_voting_approved_value">community_finalize_voting_approved_value</a>(): u8 { <a href="../social_contracts/governance.md#social_contracts_governance_FINALIZE_OUTCOME_APPROVED">FINALIZE_OUTCOME_APPROVED</a> }
</code></pre>



</details>

<a name="social_contracts_governance_community_finalize_voting_reject_community_value"></a>

## Function `community_finalize_voting_reject_community_value`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_community_finalize_voting_reject_community_value">community_finalize_voting_reject_community_value</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_community_finalize_voting_reject_community_value">community_finalize_voting_reject_community_value</a>(): u8 { <a href="../social_contracts/governance.md#social_contracts_governance_FINALIZE_OUTCOME_REJECT_COMMUNITY">FINALIZE_OUTCOME_REJECT_COMMUNITY</a> }
</code></pre>



</details>

<a name="social_contracts_governance_community_finalize_voting_reject_quorum_value"></a>

## Function `community_finalize_voting_reject_quorum_value`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_community_finalize_voting_reject_quorum_value">community_finalize_voting_reject_quorum_value</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_community_finalize_voting_reject_quorum_value">community_finalize_voting_reject_quorum_value</a>(): u8 { <a href="../social_contracts/governance.md#social_contracts_governance_FINALIZE_OUTCOME_REJECT_QUORUM">FINALIZE_OUTCOME_REJECT_QUORUM</a> }
</code></pre>



</details>

<a name="social_contracts_governance_community_finalize_voting_needs_jury_value"></a>

## Function `community_finalize_voting_needs_jury_value`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_community_finalize_voting_needs_jury_value">community_finalize_voting_needs_jury_value</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_community_finalize_voting_needs_jury_value">community_finalize_voting_needs_jury_value</a>(): u8 { <a href="../social_contracts/governance.md#social_contracts_governance_FINALIZE_OUTCOME_NEEDS_JURY">FINALIZE_OUTCOME_NEEDS_JURY</a> }
</code></pre>



</details>

<a name="social_contracts_governance_delegate_vote_outcome_reject_value"></a>

## Function `delegate_vote_outcome_reject_value`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_delegate_vote_outcome_reject_value">delegate_vote_outcome_reject_value</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_delegate_vote_outcome_reject_value">delegate_vote_outcome_reject_value</a>(): u8 { <a href="../social_contracts/governance.md#social_contracts_governance_DELEGATE_VOTE_TO_REJECT">DELEGATE_VOTE_TO_REJECT</a> }
</code></pre>



</details>

<a name="social_contracts_governance_forfeit_reason_quorum_not_met_value"></a>

## Function `forfeit_reason_quorum_not_met_value`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_forfeit_reason_quorum_not_met_value">forfeit_reason_quorum_not_met_value</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_forfeit_reason_quorum_not_met_value">forfeit_reason_quorum_not_met_value</a>(): u8 { <a href="../social_contracts/governance.md#social_contracts_governance_FORFEIT_REASON_QUORUM_NOT_MET">FORFEIT_REASON_QUORUM_NOT_MET</a> }
</code></pre>



</details>

<a name="social_contracts_governance_forfeit_reason_community_rejected_value"></a>

## Function `forfeit_reason_community_rejected_value`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_forfeit_reason_community_rejected_value">forfeit_reason_community_rejected_value</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_forfeit_reason_community_rejected_value">forfeit_reason_community_rejected_value</a>(): u8 { <a href="../social_contracts/governance.md#social_contracts_governance_FORFEIT_REASON_COMMUNITY_REJECTED">FORFEIT_REASON_COMMUNITY_REJECTED</a> }
</code></pre>



</details>

<a name="social_contracts_governance_forfeit_reason_delegate_rejected_value"></a>

## Function `forfeit_reason_delegate_rejected_value`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_forfeit_reason_delegate_rejected_value">forfeit_reason_delegate_rejected_value</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_forfeit_reason_delegate_rejected_value">forfeit_reason_delegate_rejected_value</a>(): u8 { <a href="../social_contracts/governance.md#social_contracts_governance_FORFEIT_REASON_DELEGATE_REJECTED">FORFEIT_REASON_DELEGATE_REJECTED</a> }
</code></pre>



</details>

<a name="social_contracts_governance_forfeit_reason_jury_rejected_value"></a>

## Function `forfeit_reason_jury_rejected_value`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_forfeit_reason_jury_rejected_value">forfeit_reason_jury_rejected_value</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_forfeit_reason_jury_rejected_value">forfeit_reason_jury_rejected_value</a>(): u8 { <a href="../social_contracts/governance.md#social_contracts_governance_FORFEIT_REASON_JURY_REJECTED">FORFEIT_REASON_JURY_REJECTED</a> }
</code></pre>



</details>

<a name="social_contracts_governance_assert_platform_delegate_manual_reject"></a>

## Function `assert_platform_delegate_manual_reject`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_assert_platform_delegate_manual_reject">assert_platform_delegate_manual_reject</a>(registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, proposal: &<a href="../social_contracts/governance.md#social_contracts_governance_Proposal">social_contracts::governance::Proposal</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_assert_platform_delegate_manual_reject">assert_platform_delegate_manual_reject</a>(
    registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    proposal: &<a href="../social_contracts/governance.md#social_contracts_governance_Proposal">Proposal</a>,
    clock: &Clock,
    ctx: &TxContext
): u64 {
    <b>assert</b>!(registry.<a href="../social_contracts/governance.md#social_contracts_governance_version">version</a> == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/governance.md#social_contracts_governance_EWrongVersion">EWrongVersion</a>);
    <b>assert</b>!(registry.<a href="../social_contracts/governance.md#social_contracts_governance_registry_type">registry_type</a> == <a href="../social_contracts/governance.md#social_contracts_governance_PROPOSAL_TYPE_PLATFORM">PROPOSAL_TYPE_PLATFORM</a>, <a href="../social_contracts/governance.md#social_contracts_governance_EWrongRegistryForTreasuryRoute">EWrongRegistryForTreasuryRoute</a>);
    <b>let</b> caller = tx_context::sender(ctx);
    <b>assert</b>!(table::contains(&registry.delegates, caller), <a href="../social_contracts/governance.md#social_contracts_governance_ENotDelegate">ENotDelegate</a>);
    <b>let</b> proposal_id = object::id(proposal);
    <b>assert</b>!(table::contains(&registry.proposals, proposal_id), <a href="../social_contracts/governance.md#social_contracts_governance_EProposalNotFound">EProposalNotFound</a>);
    <b>assert</b>!(proposal.status == <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_DELEGATE_REVIEW">STATUS_DELEGATE_REVIEW</a>, <a href="../social_contracts/governance.md#social_contracts_governance_EInvalidProposalStatus">EInvalidProposalStatus</a>);
    <b>assert</b>!(!<a href="../social_contracts/governance.md#social_contracts_governance_council_jury_has_jurisdiction">council_jury_has_jurisdiction</a>(proposal), <a href="../social_contracts/governance.md#social_contracts_governance_EJuryHasJurisdiction">EJuryHasJurisdiction</a>);
    clock::timestamp_ms(clock)
}
</code></pre>



</details>

<a name="social_contracts_governance_assert_shared_global_registry"></a>

## Function `assert_shared_global_registry`



<pre><code><b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_assert_shared_global_registry">assert_shared_global_registry</a>(registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_assert_shared_global_registry">assert_shared_global_registry</a>(registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>) {
    <b>assert</b>!(
        registry.<a href="../social_contracts/governance.md#social_contracts_governance_registry_type">registry_type</a> == <a href="../social_contracts/governance.md#social_contracts_governance_PROPOSAL_TYPE_ECOSYSTEM">PROPOSAL_TYPE_ECOSYSTEM</a>
            || registry.<a href="../social_contracts/governance.md#social_contracts_governance_registry_type">registry_type</a> == <a href="../social_contracts/governance.md#social_contracts_governance_PROPOSAL_TYPE_PROOF_OF_CREATIVITY">PROPOSAL_TYPE_PROOF_OF_CREATIVITY</a>
            || registry.<a href="../social_contracts/governance.md#social_contracts_governance_registry_type">registry_type</a> == <a href="../social_contracts/governance.md#social_contracts_governance_PROPOSAL_TYPE_SPOT">PROPOSAL_TYPE_SPOT</a>,
        <a href="../social_contracts/governance.md#social_contracts_governance_EWrongRegistryForTreasuryRoute">EWrongRegistryForTreasuryRoute</a>
    );
}
</code></pre>



</details>

<a name="social_contracts_governance_forfeit_proposal_pool_to_treasury_address"></a>

## Function `forfeit_proposal_pool_to_treasury_address`



<pre><code><b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_forfeit_proposal_pool_to_treasury_address">forfeit_proposal_pool_to_treasury_address</a>(registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">social_contracts::governance::Proposal</a>, recipient: <b>address</b>, reason: u8, treasury_route: u8, time_ms: u64, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_forfeit_proposal_pool_to_treasury_address">forfeit_proposal_pool_to_treasury_address</a>(
    registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">Proposal</a>,
    recipient: <b>address</b>,
    reason: u8,
    treasury_route: u8,
    time_ms: u64,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> proposal_id = object::id(proposal);
    <b>let</b> reg_type = registry.<a href="../social_contracts/governance.md#social_contracts_governance_registry_type">registry_type</a>;
    <b>let</b> amount = balance::value(&proposal.reward_pool);
    <b>if</b> (amount &gt; 0) {
        <b>let</b> c = coin::from_balance(balance::withdraw_all(&<b>mut</b> proposal.reward_pool), ctx);
        transfer::public_transfer(c, recipient);
    } <b>else</b> {
        balance::destroy_zero(balance::withdraw_all(&<b>mut</b> proposal.reward_pool));
    };
    event::emit(<a href="../social_contracts/governance.md#social_contracts_governance_ProposalRewardPoolForfeitedToTreasuryEvent">ProposalRewardPoolForfeitedToTreasuryEvent</a> {
        proposal_id,
        recipient,
        amount,
        reason,
        <a href="../social_contracts/governance.md#social_contracts_governance_registry_type">registry_type</a>: reg_type,
        treasury_route,
        forfeited_time: time_ms,
    });
}
</code></pre>



</details>

<a name="social_contracts_governance_transition_proposal_rejected_by_delegate_council"></a>

## Function `transition_proposal_rejected_by_delegate_council`

Updates status tables only: delegate review to rejected. Caller sends pool + emits [ProposalRejectedEvent].


<pre><code><b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_transition_proposal_rejected_by_delegate_council">transition_proposal_rejected_by_delegate_council</a>(registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">social_contracts::governance::Proposal</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_transition_proposal_rejected_by_delegate_council">transition_proposal_rejected_by_delegate_council</a>(
    registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">Proposal</a>
) {
    <b>let</b> proposal_id = object::id(proposal);
    proposal.status = <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_REJECTED">STATUS_REJECTED</a>;
    <b>let</b> from_status = table::borrow_mut(&<b>mut</b> registry.proposals_by_status, <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_DELEGATE_REVIEW">STATUS_DELEGATE_REVIEW</a>);
    <b>let</b> <b>mut</b> index = 0;
    <b>let</b> len = vector::length(from_status);
    <b>while</b> (index &lt; len) {
        <b>if</b> (*vector::borrow(from_status, index) == proposal_id) {
            vector::remove(from_status, index);
            <b>break</b>
        };
        index = index + 1;
    };
    <b>let</b> to_status = table::borrow_mut(&<b>mut</b> registry.proposals_by_status, <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_REJECTED">STATUS_REJECTED</a>);
    vector::push_back(to_status, proposal_id);
}
</code></pre>



</details>

<a name="social_contracts_governance_complete_delegate_reject_to_ecosystem_treasury"></a>

## Function `complete_delegate_reject_to_ecosystem_treasury`



<pre><code><b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_complete_delegate_reject_to_ecosystem_treasury">complete_delegate_reject_to_ecosystem_treasury</a>(registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">social_contracts::governance::Proposal</a>, current_time: u64, ecosystem_treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_complete_delegate_reject_to_ecosystem_treasury">complete_delegate_reject_to_ecosystem_treasury</a>(
    registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">Proposal</a>,
    current_time: u64,
    ecosystem_treasury: &EcosystemTreasury,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> proposal_id = object::id(proposal);
    <a href="../social_contracts/governance.md#social_contracts_governance_transition_proposal_rejected_by_delegate_council">transition_proposal_rejected_by_delegate_council</a>(registry, proposal);
    <b>let</b> dest = <a href="../social_contracts/profile.md#social_contracts_profile_get_treasury_address">profile::get_treasury_address</a>(ecosystem_treasury);
    <a href="../social_contracts/governance.md#social_contracts_governance_forfeit_proposal_pool_to_treasury_address">forfeit_proposal_pool_to_treasury_address</a>(
        registry,
        proposal,
        dest,
        <a href="../social_contracts/governance.md#social_contracts_governance_FORFEIT_REASON_DELEGATE_REJECTED">FORFEIT_REASON_DELEGATE_REJECTED</a>,
        <a href="../social_contracts/governance.md#social_contracts_governance_TREASURY_ROUTE_ECOSYSTEM">TREASURY_ROUTE_ECOSYSTEM</a>,
        current_time,
        ctx,
    );
    event::emit(<a href="../social_contracts/governance.md#social_contracts_governance_ProposalRejectedEvent">ProposalRejectedEvent</a> {
        proposal_id,
        rejection_time: current_time,
    });
}
</code></pre>



</details>

<a name="social_contracts_governance_complete_delegate_reject_drain_to_platform_governance"></a>

## Function `complete_delegate_reject_drain_to_platform_governance`

For platform: transition + forfeiture event (platform route) + [ProposalRejectedEvent]; returns pool balance for the caller to join into <code>Platform.treasury</code>.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_complete_delegate_reject_drain_to_platform_governance">complete_delegate_reject_drain_to_platform_governance</a>(registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">social_contracts::governance::Proposal</a>, platform_treasury_address: <b>address</b>, current_time: u64, _ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../myso/balance.md#myso_balance_Balance">myso::balance::Balance</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_complete_delegate_reject_drain_to_platform_governance">complete_delegate_reject_drain_to_platform_governance</a>(
    registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">Proposal</a>,
    platform_treasury_address: <b>address</b>,
    current_time: u64,
    _ctx: &<b>mut</b> TxContext
): Balance&lt;MYSO&gt; {
    <b>assert</b>!(registry.<a href="../social_contracts/governance.md#social_contracts_governance_registry_type">registry_type</a> == <a href="../social_contracts/governance.md#social_contracts_governance_PROPOSAL_TYPE_PLATFORM">PROPOSAL_TYPE_PLATFORM</a>, <a href="../social_contracts/governance.md#social_contracts_governance_EWrongRegistryForTreasuryRoute">EWrongRegistryForTreasuryRoute</a>);
    <b>let</b> proposal_id = object::id(proposal);
    <a href="../social_contracts/governance.md#social_contracts_governance_transition_proposal_rejected_by_delegate_council">transition_proposal_rejected_by_delegate_council</a>(registry, proposal);
    <b>let</b> amount = balance::value(&proposal.reward_pool);
    event::emit(<a href="../social_contracts/governance.md#social_contracts_governance_ProposalRewardPoolForfeitedToTreasuryEvent">ProposalRewardPoolForfeitedToTreasuryEvent</a> {
        proposal_id,
        recipient: platform_treasury_address,
        amount,
        reason: <a href="../social_contracts/governance.md#social_contracts_governance_FORFEIT_REASON_DELEGATE_REJECTED">FORFEIT_REASON_DELEGATE_REJECTED</a>,
        <a href="../social_contracts/governance.md#social_contracts_governance_registry_type">registry_type</a>: registry.<a href="../social_contracts/governance.md#social_contracts_governance_registry_type">registry_type</a>,
        treasury_route: <a href="../social_contracts/governance.md#social_contracts_governance_TREASURY_ROUTE_PLATFORM">TREASURY_ROUTE_PLATFORM</a>,
        forfeited_time: current_time,
    });
    event::emit(<a href="../social_contracts/governance.md#social_contracts_governance_ProposalRejectedEvent">ProposalRejectedEvent</a> {
        proposal_id,
        rejection_time: current_time,
    });
    balance::withdraw_all(&<b>mut</b> proposal.reward_pool)
}
</code></pre>



</details>

<a name="social_contracts_governance_clone_bytes"></a>

## Function `clone_bytes`



<pre><code><b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_clone_bytes">clone_bytes</a>(b: &vector&lt;u8&gt;): vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_clone_bytes">clone_bytes</a>(b: &vector&lt;u8&gt;): vector&lt;u8&gt; {
    <b>let</b> <b>mut</b> out = vector::empty();
    <b>let</b> len = vector::length(b);
    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (i &lt; len) {
        vector::push_back(&<b>mut</b> out, *vector::borrow(b, i));
        i = i + 1;
    };
    out
}
</code></pre>



</details>

<a name="social_contracts_governance_clone_option_string"></a>

## Function `clone_option_string`



<pre><code><b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_clone_option_string">clone_option_string</a>(opt: &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_clone_option_string">clone_option_string</a>(opt: &Option&lt;String&gt;): Option&lt;String&gt; {
    <b>if</b> (option::is_none(opt)) {
        <b>return</b> option::none()
    };
    <b>let</b> s = option::borrow(opt);
    option::some(string::utf8(<a href="../social_contracts/governance.md#social_contracts_governance_clone_bytes">clone_bytes</a>(string::as_bytes(s))))
}
</code></pre>



</details>

<a name="social_contracts_governance_run_delegate_review_vote"></a>

## Function `run_delegate_review_vote`

Run delegate-in-review vote logic; returns 0=still in review, 1=advanced to community, 2=must complete delegate reject (ecosystem or platform) via separate entry.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_run_delegate_review_vote">run_delegate_review_vote</a>(registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">social_contracts::governance::Proposal</a>, approve: bool, reason: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_run_delegate_review_vote">run_delegate_review_vote</a>(
    registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">Proposal</a>,
    approve: bool,
    <b>mut</b> reason: Option&lt;String&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
): u8 {
    <b>assert</b>!(registry.<a href="../social_contracts/governance.md#social_contracts_governance_version">version</a> == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/governance.md#social_contracts_governance_EWrongVersion">EWrongVersion</a>);
    <a href="../social_contracts/governance.md#social_contracts_governance_try_update_delegate_panel_if_due">try_update_delegate_panel_if_due</a>(registry, ctx);
    <b>let</b> vote_time = clock::timestamp_ms(clock);
    <b>let</b> reason_for_event = <a href="../social_contracts/governance.md#social_contracts_governance_clone_option_string">clone_option_string</a>(&reason);
    <b>let</b> caller = tx_context::sender(ctx);
    <b>let</b> proposal_id = object::id(proposal);
    <b>assert</b>!(table::contains(&registry.proposals, proposal_id), <a href="../social_contracts/governance.md#social_contracts_governance_EProposalNotFound">EProposalNotFound</a>);
    <b>assert</b>!(table::contains(&registry.delegates, caller), <a href="../social_contracts/governance.md#social_contracts_governance_ENotDelegate">ENotDelegate</a>);
    <b>assert</b>!(proposal.status == <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_DELEGATE_REVIEW">STATUS_DELEGATE_REVIEW</a>, <a href="../social_contracts/governance.md#social_contracts_governance_EInvalidProposalStatus">EInvalidProposalStatus</a>);
    <b>assert</b>!(!<a href="../social_contracts/governance.md#social_contracts_governance_council_jury_has_jurisdiction">council_jury_has_jurisdiction</a>(proposal), <a href="../social_contracts/governance.md#social_contracts_governance_EJuryHasJurisdiction">EJuryHasJurisdiction</a>);
    <b>let</b> delegate_votes: &<b>mut</b> Table&lt;<b>address</b>, bool&gt; = dynamic_field::borrow_mut(&<b>mut</b> proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_DELEGATE_VOTES_FIELD">DELEGATE_VOTES_FIELD</a>);
    <b>assert</b>!(!table::contains(delegate_votes, caller), <a href="../social_contracts/governance.md#social_contracts_governance_EAlreadyVoted">EAlreadyVoted</a>);
    table::add(delegate_votes, caller, approve);
    <b>if</b> (option::is_some(&reason)) {
        <b>if</b> (!dynamic_field::exists_(&proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_DELEGATE_REASONS_FIELD">DELEGATE_REASONS_FIELD</a>)) {
            <b>let</b> reason_table = table::new&lt;<b>address</b>, String&gt;(ctx);
            dynamic_field::add(&<b>mut</b> proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_DELEGATE_REASONS_FIELD">DELEGATE_REASONS_FIELD</a>, reason_table);
        };
        <b>let</b> reason_table: &<b>mut</b> Table&lt;<b>address</b>, String&gt; = dynamic_field::borrow_mut(
            &<b>mut</b> proposal.id,
            <a href="../social_contracts/governance.md#social_contracts_governance_DELEGATE_REASONS_FIELD">DELEGATE_REASONS_FIELD</a>
        );
        table::add(reason_table, caller, option::extract(&<b>mut</b> reason));
    };
    <b>if</b> (!dynamic_field::exists_(&proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_VOTED_DELEGATES_LIST_FIELD">VOTED_DELEGATES_LIST_FIELD</a>)) {
        <b>let</b> delegates_list = vector::empty&lt;<b>address</b>&gt;();
        dynamic_field::add(&<b>mut</b> proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_VOTED_DELEGATES_LIST_FIELD">VOTED_DELEGATES_LIST_FIELD</a>, delegates_list);
    };
    <b>let</b> delegates_list: &<b>mut</b> vector&lt;<b>address</b>&gt; = dynamic_field::borrow_mut(
        &<b>mut</b> proposal.id,
        <a href="../social_contracts/governance.md#social_contracts_governance_VOTED_DELEGATES_LIST_FIELD">VOTED_DELEGATES_LIST_FIELD</a>
    );
    vector::push_back(delegates_list, caller);
    <b>if</b> (approve) {
        proposal.delegate_approval_count = proposal.delegate_approval_count + 1;
    } <b>else</b> {
        proposal.delegate_rejection_count = proposal.delegate_rejection_count + 1;
    };
    <b>let</b> delegate = table::borrow_mut(&<b>mut</b> registry.delegates, caller);
    delegate.proposals_reviewed = delegate.proposals_reviewed + 1;
    <b>let</b> delegate_approval_count = proposal.delegate_approval_count;
    <b>let</b> delegate_rejection_count = proposal.delegate_rejection_count;
    <b>let</b> total_delegates = table::length(&registry.delegates);
    <b>let</b> outcome = <b>if</b> (delegate_approval_count &gt; total_delegates / 2) {
        <a href="../social_contracts/governance.md#social_contracts_governance_move_to_community_voting">move_to_community_voting</a>(registry, proposal, clock, ctx);
        <a href="../social_contracts/governance.md#social_contracts_governance_DELEGATE_VOTE_TO_COMMUNITY">DELEGATE_VOTE_TO_COMMUNITY</a>
    } <b>else</b> <b>if</b> (delegate_rejection_count &gt; total_delegates / 2) {
        <a href="../social_contracts/governance.md#social_contracts_governance_DELEGATE_VOTE_TO_REJECT">DELEGATE_VOTE_TO_REJECT</a>
    } <b>else</b> {
        <a href="../social_contracts/governance.md#social_contracts_governance_DELEGATE_VOTE_STILL_IN_REVIEW">DELEGATE_VOTE_STILL_IN_REVIEW</a>
    };
    event::emit(<a href="../social_contracts/governance.md#social_contracts_governance_DelegateVoteEvent">DelegateVoteEvent</a> {
        proposal_id,
        delegate_address: caller,
        approve,
        vote_time,
        reason: reason_for_event,
    });
    outcome
}
</code></pre>



</details>

<a name="social_contracts_governance_delegate_vote_on_proposal"></a>

## Function `delegate_vote_on_proposal`

Ecosystem and proof-of-creativity only: <code>ecosystem_treasury</code> used when the delegate council rejects the proposal.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_delegate_vote_on_proposal">delegate_vote_on_proposal</a>(registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">social_contracts::governance::Proposal</a>, ecosystem_treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, approve: bool, reason: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_delegate_vote_on_proposal">delegate_vote_on_proposal</a>(
    registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">Proposal</a>,
    ecosystem_treasury: &EcosystemTreasury,
    approve: bool,
    reason: Option&lt;String&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(registry.<a href="../social_contracts/governance.md#social_contracts_governance_version">version</a> == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/governance.md#social_contracts_governance_EWrongVersion">EWrongVersion</a>);
    <a href="../social_contracts/governance.md#social_contracts_governance_assert_shared_global_registry">assert_shared_global_registry</a>(registry);
    <b>let</b> current_time = clock::timestamp_ms(clock);
    <b>let</b> out = <a href="../social_contracts/governance.md#social_contracts_governance_run_delegate_review_vote">run_delegate_review_vote</a>(registry, proposal, approve, reason, clock, ctx);
    <b>if</b> (out == <a href="../social_contracts/governance.md#social_contracts_governance_DELEGATE_VOTE_TO_REJECT">DELEGATE_VOTE_TO_REJECT</a>) {
        <a href="../social_contracts/governance.md#social_contracts_governance_complete_delegate_reject_to_ecosystem_treasury">complete_delegate_reject_to_ecosystem_treasury</a>(
            registry,
            proposal,
            current_time,
            ecosystem_treasury,
            ctx
        );
    };
}
</code></pre>



</details>

<a name="social_contracts_governance_emit_implementation_reward_to_submitter_event"></a>

## Function `emit_implementation_reward_to_submitter_event`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_emit_implementation_reward_to_submitter_event">emit_implementation_reward_to_submitter_event</a>(proposal_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, submitter: <b>address</b>, amount: u64, <a href="../social_contracts/governance.md#social_contracts_governance_registry_type">registry_type</a>: u8, sent_time: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_emit_implementation_reward_to_submitter_event">emit_implementation_reward_to_submitter_event</a>(
    proposal_id: ID,
    submitter: <b>address</b>,
    amount: u64,
    <a href="../social_contracts/governance.md#social_contracts_governance_registry_type">registry_type</a>: u8,
    sent_time: u64,
) {
    event::emit(<a href="../social_contracts/governance.md#social_contracts_governance_ProposalImplementationRewardToSubmitterEvent">ProposalImplementationRewardToSubmitterEvent</a> {
        proposal_id,
        submitter,
        amount,
        <a href="../social_contracts/governance.md#social_contracts_governance_registry_type">registry_type</a>,
        sent_time,
    });
}
</code></pre>



</details>

<a name="social_contracts_governance_finalize_community_voting_internals"></a>

## Function `finalize_community_voting_internals`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_finalize_community_voting_internals">finalize_community_voting_internals</a>(registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">social_contracts::governance::Proposal</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): (u8, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_finalize_community_voting_internals">finalize_community_voting_internals</a>(
    registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">Proposal</a>,
    clock: &Clock,
    ctx: &TxContext,
): (u8, u64) {
    <a href="../social_contracts/governance.md#social_contracts_governance_try_update_delegate_panel_if_due">try_update_delegate_panel_if_due</a>(registry, ctx);
    <b>let</b> current_time_ms = clock::timestamp_ms(clock);
    <b>let</b> proposal_id = object::id(proposal);
    <b>assert</b>!(table::contains(&registry.proposals, proposal_id), <a href="../social_contracts/governance.md#social_contracts_governance_EProposalNotFound">EProposalNotFound</a>);
    <b>assert</b>!(proposal.status == <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_COMMUNITY_VOTING">STATUS_COMMUNITY_VOTING</a>, <a href="../social_contracts/governance.md#social_contracts_governance_EInvalidProposalStatus">EInvalidProposalStatus</a>);
    <b>assert</b>!(current_time_ms &gt; proposal.voting_end_time, <a href="../social_contracts/governance.md#social_contracts_governance_EVotingPeriodNotEnded">EVotingPeriodNotEnded</a>);
    <b>assert</b>!(!dynamic_field::exists_(&proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_JURY_COMMUNITY_FIELD">JURY_COMMUNITY_FIELD</a>), <a href="../social_contracts/governance.md#social_contracts_governance_EJuryHasJurisdiction">EJuryHasJurisdiction</a>);
    <b>let</b> total_votes = proposal.community_votes_for + proposal.community_votes_against;
    // A quorum miss (including no votes at all) or a tie goes to the <a href="../social_contracts/jury.md#social_contracts_jury">jury</a> instead of being
    // rejected, when the registry <b>has</b> switched that on and the pool can pay jurors. Nothing
    // <b>else</b> changes here: status, buckets and delegate stats wait <b>for</b> the <a href="../social_contracts/jury.md#social_contracts_jury">jury</a>'s decision.
    {
        <b>let</b> quorum_met = total_votes &gt;= registry.quorum_votes;
        <b>let</b> tie = proposal.community_votes_for == proposal.community_votes_against;
        <b>if</b> ((!quorum_met || tie) && <a href="../social_contracts/governance.md#social_contracts_governance_community_jury_available">community_jury_available</a>(registry, proposal)) {
            <b>let</b> trigger = <b>if</b> (!quorum_met) { <a href="../social_contracts/governance.md#social_contracts_governance_JURY_TRIGGER_QUORUM_NOT_MET">JURY_TRIGGER_QUORUM_NOT_MET</a> } <b>else</b> { <a href="../social_contracts/governance.md#social_contracts_governance_JURY_TRIGGER_VOTE_TIE">JURY_TRIGGER_VOTE_TIE</a> };
            dynamic_field::add(
                &<b>mut</b> proposal.id,
                <a href="../social_contracts/governance.md#social_contracts_governance_JURY_COMMUNITY_FIELD">JURY_COMMUNITY_FIELD</a>,
                <a href="../social_contracts/governance.md#social_contracts_governance_JuryReview">JuryReview</a> {
                    stage: <a href="../social_contracts/governance.md#social_contracts_governance_JURY_STAGE_COMMUNITY">JURY_STAGE_COMMUNITY</a>,
                    state: <a href="../social_contracts/governance.md#social_contracts_governance_JURY_STATE_AWAITING">JURY_STATE_AWAITING</a>,
                    trigger,
                    referrers: vector::empty(),
                    case_id: option::none(),
                    escalated_at_ms: 0,
                    decided_at_ms: 0,
                    decision: <a href="../social_contracts/governance.md#social_contracts_governance_NO_DECISION">NO_DECISION</a>,
                    deferred_since_ms: 0,
                },
            );
            event::emit(<a href="../social_contracts/governance.md#social_contracts_governance_CommunityVoteNeedsJuryEvent">CommunityVoteNeedsJuryEvent</a> {
                proposal_id,
                trigger,
                votes_for: proposal.community_votes_for,
                votes_against: proposal.community_votes_against,
                quorum_votes: registry.quorum_votes,
                time_ms: current_time_ms,
            });
            <b>return</b> (<a href="../social_contracts/governance.md#social_contracts_governance_FINALIZE_OUTCOME_NEEDS_JURY">FINALIZE_OUTCOME_NEEDS_JURY</a>, current_time_ms)
        };
    };
    <b>let</b> from_status = table::borrow_mut(&<b>mut</b> registry.proposals_by_status, <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_COMMUNITY_VOTING">STATUS_COMMUNITY_VOTING</a>);
    <b>let</b> <b>mut</b> index = 0;
    <b>let</b> len = vector::length(from_status);
    <b>while</b> (index &lt; len) {
        <b>if</b> (*vector::borrow(from_status, index) == proposal_id) {
            vector::remove(from_status, index);
            <b>break</b>
        };
        index = index + 1;
    };
    <b>let</b> quorum_met = total_votes &gt;= registry.quorum_votes;
    <b>let</b> majority_approve = proposal.community_votes_for &gt; proposal.community_votes_against;
    <b>if</b> (quorum_met) {
        <b>if</b> (dynamic_field::exists_(&proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_VOTED_DELEGATES_LIST_FIELD">VOTED_DELEGATES_LIST_FIELD</a>)) {
            <b>let</b> delegates_list: &vector&lt;<b>address</b>&gt; = dynamic_field::borrow(
                &proposal.id,
                <a href="../social_contracts/governance.md#social_contracts_governance_VOTED_DELEGATES_LIST_FIELD">VOTED_DELEGATES_LIST_FIELD</a>
            );
            <b>let</b> delegate_votes: &Table&lt;<b>address</b>, bool&gt; = dynamic_field::borrow(
                &proposal.id,
                <a href="../social_contracts/governance.md#social_contracts_governance_DELEGATE_VOTES_FIELD">DELEGATE_VOTES_FIELD</a>
            );
            <b>let</b> <b>mut</b> i = 0;
            <b>let</b> delegates_count = vector::length(delegates_list);
            <b>while</b> (i &lt; delegates_count) {
                <b>let</b> delegate_addr = *vector::borrow(delegates_list, i);
                <b>if</b> (table::contains(&registry.delegates, delegate_addr)) {
                    <b>let</b> delegate = table::borrow_mut(&<b>mut</b> registry.delegates, delegate_addr);
                    <b>let</b> delegate_approved = *table::borrow(delegate_votes, delegate_addr);
                    <b>if</b> ((delegate_approved && majority_approve) || (!delegate_approved && !majority_approve)) {
                        delegate.sided_winning_proposals = delegate.sided_winning_proposals + 1;
                    } <b>else</b> {
                        delegate.sided_losing_proposals = delegate.sided_losing_proposals + 1;
                    };
                };
                i = i + 1;
            };
        };
        <b>if</b> (majority_approve) {
            proposal.status = <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_APPROVED">STATUS_APPROVED</a>;
            <b>let</b> to_status = table::borrow_mut(&<b>mut</b> registry.proposals_by_status, <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_APPROVED">STATUS_APPROVED</a>);
            vector::push_back(to_status, proposal_id);
            event::emit(<a href="../social_contracts/governance.md#social_contracts_governance_ProposalApprovedEvent">ProposalApprovedEvent</a> {
                proposal_id,
                approval_time: current_time_ms,
                votes_for: proposal.community_votes_for,
                votes_against: proposal.community_votes_against,
            });
            (<a href="../social_contracts/governance.md#social_contracts_governance_FINALIZE_OUTCOME_APPROVED">FINALIZE_OUTCOME_APPROVED</a>, current_time_ms)
        } <b>else</b> {
            proposal.status = <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_REJECTED">STATUS_REJECTED</a>;
            <b>let</b> to_status = table::borrow_mut(&<b>mut</b> registry.proposals_by_status, <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_REJECTED">STATUS_REJECTED</a>);
            vector::push_back(to_status, proposal_id);
            event::emit(<a href="../social_contracts/governance.md#social_contracts_governance_ProposalRejectedByCommunityEvent">ProposalRejectedByCommunityEvent</a> {
                proposal_id,
                rejection_time: current_time_ms,
                votes_for: proposal.community_votes_for,
                votes_against: proposal.community_votes_against,
            });
            (<a href="../social_contracts/governance.md#social_contracts_governance_FINALIZE_OUTCOME_REJECT_COMMUNITY">FINALIZE_OUTCOME_REJECT_COMMUNITY</a>, current_time_ms)
        }
    } <b>else</b> {
        proposal.status = <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_REJECTED">STATUS_REJECTED</a>;
        <b>let</b> to_status = table::borrow_mut(&<b>mut</b> registry.proposals_by_status, <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_REJECTED">STATUS_REJECTED</a>);
        vector::push_back(to_status, proposal_id);
        event::emit(<a href="../social_contracts/governance.md#social_contracts_governance_ProposalRejectedByCommunityEvent">ProposalRejectedByCommunityEvent</a> {
            proposal_id,
            rejection_time: current_time_ms,
            votes_for: proposal.community_votes_for,
            votes_against: proposal.community_votes_against,
        });
        (<a href="../social_contracts/governance.md#social_contracts_governance_FINALIZE_OUTCOME_REJECT_QUORUM">FINALIZE_OUTCOME_REJECT_QUORUM</a>, current_time_ms)
    }
}
</code></pre>



</details>

<a name="social_contracts_governance_finalize_proposal"></a>

## Function `finalize_proposal`

Ecosystem and proof-of-creativity: finalize after community vote; on failure, reward pool to ecosystem treasury.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_finalize_proposal">finalize_proposal</a>(registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">social_contracts::governance::Proposal</a>, ecosystem_treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_finalize_proposal">finalize_proposal</a>(
    registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">Proposal</a>,
    ecosystem_treasury: &EcosystemTreasury,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(registry.<a href="../social_contracts/governance.md#social_contracts_governance_version">version</a> == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/governance.md#social_contracts_governance_EWrongVersion">EWrongVersion</a>);
    <a href="../social_contracts/governance.md#social_contracts_governance_assert_shared_global_registry">assert_shared_global_registry</a>(registry);
    <b>let</b> (outcome, time_ms) = <a href="../social_contracts/governance.md#social_contracts_governance_finalize_community_voting_internals">finalize_community_voting_internals</a>(registry, proposal, clock, ctx);
    <b>if</b> (outcome == <a href="../social_contracts/governance.md#social_contracts_governance_FINALIZE_OUTCOME_REJECT_COMMUNITY">FINALIZE_OUTCOME_REJECT_COMMUNITY</a>) {
        <b>let</b> dest = <a href="../social_contracts/profile.md#social_contracts_profile_get_treasury_address">profile::get_treasury_address</a>(ecosystem_treasury);
        <a href="../social_contracts/governance.md#social_contracts_governance_forfeit_proposal_pool_to_treasury_address">forfeit_proposal_pool_to_treasury_address</a>(
            registry,
            proposal,
            dest,
            <a href="../social_contracts/governance.md#social_contracts_governance_FORFEIT_REASON_COMMUNITY_REJECTED">FORFEIT_REASON_COMMUNITY_REJECTED</a>,
            <a href="../social_contracts/governance.md#social_contracts_governance_TREASURY_ROUTE_ECOSYSTEM">TREASURY_ROUTE_ECOSYSTEM</a>,
            time_ms,
            ctx
        );
    } <b>else</b> <b>if</b> (outcome == <a href="../social_contracts/governance.md#social_contracts_governance_FINALIZE_OUTCOME_REJECT_QUORUM">FINALIZE_OUTCOME_REJECT_QUORUM</a>) {
        <b>let</b> dest = <a href="../social_contracts/profile.md#social_contracts_profile_get_treasury_address">profile::get_treasury_address</a>(ecosystem_treasury);
        <a href="../social_contracts/governance.md#social_contracts_governance_forfeit_proposal_pool_to_treasury_address">forfeit_proposal_pool_to_treasury_address</a>(
            registry,
            proposal,
            dest,
            <a href="../social_contracts/governance.md#social_contracts_governance_FORFEIT_REASON_QUORUM_NOT_MET">FORFEIT_REASON_QUORUM_NOT_MET</a>,
            <a href="../social_contracts/governance.md#social_contracts_governance_TREASURY_ROUTE_ECOSYSTEM">TREASURY_ROUTE_ECOSYSTEM</a>,
            time_ms,
            ctx
        );
    };
}
</code></pre>



</details>

<a name="social_contracts_governance_forfeit_proposal_pool_after_community_reject_to_platform_treasury"></a>

## Function `forfeit_proposal_pool_after_community_reject_to_platform_treasury`

After community voting fails for a platform registry: move funds into this platform’s governance (treasury) balance.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_forfeit_proposal_pool_after_community_reject_to_platform_treasury">forfeit_proposal_pool_after_community_reject_to_platform_treasury</a>(registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">social_contracts::governance::Proposal</a>, platform_treasury_address: <b>address</b>, reason: u8, time_ms: u64): <a href="../myso/balance.md#myso_balance_Balance">myso::balance::Balance</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_forfeit_proposal_pool_after_community_reject_to_platform_treasury">forfeit_proposal_pool_after_community_reject_to_platform_treasury</a>(
    registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">Proposal</a>,
    platform_treasury_address: <b>address</b>,
    reason: u8,
    time_ms: u64,
): Balance&lt;MYSO&gt; {
    <b>assert</b>!(registry.<a href="../social_contracts/governance.md#social_contracts_governance_registry_type">registry_type</a> == <a href="../social_contracts/governance.md#social_contracts_governance_PROPOSAL_TYPE_PLATFORM">PROPOSAL_TYPE_PLATFORM</a>, <a href="../social_contracts/governance.md#social_contracts_governance_EWrongRegistryForTreasuryRoute">EWrongRegistryForTreasuryRoute</a>);
    <b>let</b> proposal_id = object::id(proposal);
    <b>let</b> amount = balance::value(&proposal.reward_pool);
    event::emit(<a href="../social_contracts/governance.md#social_contracts_governance_ProposalRewardPoolForfeitedToTreasuryEvent">ProposalRewardPoolForfeitedToTreasuryEvent</a> {
        proposal_id,
        recipient: platform_treasury_address,
        amount,
        reason,
        <a href="../social_contracts/governance.md#social_contracts_governance_registry_type">registry_type</a>: registry.<a href="../social_contracts/governance.md#social_contracts_governance_registry_type">registry_type</a>,
        treasury_route: <a href="../social_contracts/governance.md#social_contracts_governance_TREASURY_ROUTE_PLATFORM">TREASURY_ROUTE_PLATFORM</a>,
        forfeited_time: time_ms,
    });
    balance::withdraw_all(&<b>mut</b> proposal.reward_pool)
}
</code></pre>



</details>

<a name="social_contracts_governance_anonymous_tally_votes_and_finalize_community_internals"></a>

## Function `anonymous_tally_votes_and_finalize_community_internals`

Decrypt anonymous votes, merge tallies, and resolve community finalization. Callable from [<code><a href="../social_contracts/platform.md#social_contracts_platform">social_contracts::platform</a></code>] for platform registries.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_anonymous_tally_votes_and_finalize_community_internals">anonymous_tally_votes_and_finalize_community_internals</a>(registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">social_contracts::governance::Proposal</a>, keys: &vector&lt;<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_VerifiedDerivedKey">mydata::bf_hmac_encryption::VerifiedDerivedKey</a>&gt;, public_keys: &vector&lt;<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_PublicKey">mydata::bf_hmac_encryption::PublicKey</a>&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): (u8, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_anonymous_tally_votes_and_finalize_community_internals">anonymous_tally_votes_and_finalize_community_internals</a>(
    registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">Proposal</a>,
    keys: &vector&lt;VerifiedDerivedKey&gt;,
    public_keys: &vector&lt;PublicKey&gt;,
    clock: &Clock,
    ctx: &TxContext,
): (u8, u64) {
    <b>assert</b>!(registry.<a href="../social_contracts/governance.md#social_contracts_governance_version">version</a> == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/governance.md#social_contracts_governance_EWrongVersion">EWrongVersion</a>);
    <b>let</b> current_time_ms = clock::timestamp_ms(clock);
    <b>let</b> proposal_id = object::id(proposal);
    <b>assert</b>!(table::contains(&registry.proposals, proposal_id), <a href="../social_contracts/governance.md#social_contracts_governance_EProposalNotFound">EProposalNotFound</a>);
    // The tally is merged into the proposal before finalizing; once the <a href="../social_contracts/jury.md#social_contracts_jury">jury</a> is waiting a second
    // pass would count every vote again.
    <b>assert</b>!(!dynamic_field::exists_(&proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_JURY_COMMUNITY_FIELD">JURY_COMMUNITY_FIELD</a>), <a href="../social_contracts/governance.md#social_contracts_governance_EJuryHasJurisdiction">EJuryHasJurisdiction</a>);
    <b>let</b> <b>mut</b> votes_for = vector::empty&lt;<b>address</b>&gt;();
    <b>let</b> <b>mut</b> votes_against = vector::empty&lt;<b>address</b>&gt;();
    <b>let</b> <b>mut</b> invalid_votes = vector::empty&lt;<b>address</b>&gt;();
    {
        <b>assert</b>!(proposal.status == <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_COMMUNITY_VOTING">STATUS_COMMUNITY_VOTING</a>, <a href="../social_contracts/governance.md#social_contracts_governance_EInvalidProposalStatus">EInvalidProposalStatus</a>);
        <b>assert</b>!(current_time_ms &gt; proposal.voting_end_time, <a href="../social_contracts/governance.md#social_contracts_governance_EVotingPeriodNotEnded">EVotingPeriodNotEnded</a>);
        <b>if</b> (dynamic_field::exists_(&proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_ENCRYPTED_VOTES_FIELD">ENCRYPTED_VOTES_FIELD</a>)) {
            <b>let</b> votes_tbl: &Table&lt;<b>address</b>, EncryptedObject&gt; = dynamic_field::borrow(
                &proposal.id,
                <a href="../social_contracts/governance.md#social_contracts_governance_ENCRYPTED_VOTES_FIELD">ENCRYPTED_VOTES_FIELD</a>
            );
            <b>let</b> anon_set: &VecSet&lt;<b>address</b>&gt; = dynamic_field::borrow(&proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_ANON_VOTERS_FIELD">ANON_VOTERS_FIELD</a>);
            <b>let</b> voters_vec = vec_set::into_keys(*anon_set);
            <b>let</b> <b>mut</b> i = 0;
            <b>let</b> len = vector::length(&voters_vec);
            <b>while</b> (i &lt; len) {
                <b>let</b> addr = *vector::borrow(&voters_vec, i);
                <b>let</b> enc = table::borrow(votes_tbl, addr);
                <b>let</b> dec = decrypt(enc, keys, public_keys);
                <b>if</b> (option::is_some(&dec)) {
                    <b>let</b> b = option::borrow(&dec);
                    <b>if</b> (vector::length(b) == 1) {
                        <b>let</b> vote_value = *vector::borrow(b, 0);
                        <b>if</b> (vote_value == 1) {
                            vector::push_back(&<b>mut</b> votes_for, addr);
                        } <b>else</b> <b>if</b> (vote_value == 0) {
                            vector::push_back(&<b>mut</b> votes_against, addr);
                        } <b>else</b> {
                            vector::push_back(&<b>mut</b> invalid_votes, addr);
                            event::emit(<a href="../social_contracts/governance.md#social_contracts_governance_VoteDecryptionFailedEvent">VoteDecryptionFailedEvent</a> {
                                proposal_id,
                                voter: addr,
                                failure_reason: string::utf8(b"Invalid vote value - not 0 or 1"),
                                timestamp: current_time_ms,
                            });
                        }
                    } <b>else</b> {
                        vector::push_back(&<b>mut</b> invalid_votes, addr);
                        event::emit(<a href="../social_contracts/governance.md#social_contracts_governance_VoteDecryptionFailedEvent">VoteDecryptionFailedEvent</a> {
                            proposal_id,
                            voter: addr,
                            failure_reason: string::utf8(b"Invalid vote format - wrong byte length"),
                            timestamp: current_time_ms,
                        });
                    }
                } <b>else</b> {
                    vector::push_back(&<b>mut</b> invalid_votes, addr);
                    event::emit(<a href="../social_contracts/governance.md#social_contracts_governance_VoteDecryptionFailedEvent">VoteDecryptionFailedEvent</a> {
                        proposal_id,
                        voter: addr,
                        failure_reason: string::utf8(b"Decryption failed - invalid keys or corrupted data"),
                        timestamp: current_time_ms,
                    });
                };
                i = i + 1;
            };
            vector::destroy_empty(voters_vec);
        };
    };
    vector::destroy_empty(invalid_votes);
    {
        <b>let</b> <b>mut</b> i = 0;
        <b>let</b> len = vector::length(&votes_for);
        <b>while</b> (i &lt; len) {
            <b>assert</b>!(proposal.community_votes_for &lt;= <a href="../social_contracts/governance.md#social_contracts_governance_MAX_U64">MAX_U64</a> - 1, <a href="../social_contracts/governance.md#social_contracts_governance_EOverflow">EOverflow</a>);
            proposal.community_votes_for = proposal.community_votes_for + 1;
            i = i + 1;
        };
        <b>let</b> <b>mut</b> i = 0;
        <b>let</b> len = vector::length(&votes_against);
        <b>while</b> (i &lt; len) {
            <b>assert</b>!(proposal.community_votes_against &lt;= <a href="../social_contracts/governance.md#social_contracts_governance_MAX_U64">MAX_U64</a> - 1, <a href="../social_contracts/governance.md#social_contracts_governance_EOverflow">EOverflow</a>);
            proposal.community_votes_against = proposal.community_votes_against + 1;
            i = i + 1;
        };
    };
    vector::destroy_empty(votes_for);
    vector::destroy_empty(votes_against);
    <a href="../social_contracts/governance.md#social_contracts_governance_finalize_community_voting_internals">finalize_community_voting_internals</a>(registry, proposal, clock, ctx)
}
</code></pre>



</details>

<a name="social_contracts_governance_finalize_proposal_anonymous"></a>

## Function `finalize_proposal_anonymous`

Finalize a proposal with anonymous votes by decrypting them first (ecosystem / proof-of-creativity only).


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_finalize_proposal_anonymous">finalize_proposal_anonymous</a>(registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">social_contracts::governance::Proposal</a>, keys: &vector&lt;<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_VerifiedDerivedKey">mydata::bf_hmac_encryption::VerifiedDerivedKey</a>&gt;, public_keys: &vector&lt;<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_PublicKey">mydata::bf_hmac_encryption::PublicKey</a>&gt;, ecosystem_treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_finalize_proposal_anonymous">finalize_proposal_anonymous</a>(
    registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">Proposal</a>,
    keys: &vector&lt;VerifiedDerivedKey&gt;,
    public_keys: &vector&lt;PublicKey&gt;,
    ecosystem_treasury: &EcosystemTreasury,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <a href="../social_contracts/governance.md#social_contracts_governance_assert_shared_global_registry">assert_shared_global_registry</a>(registry);
    <b>let</b> (outcome, time_ms) = <a href="../social_contracts/governance.md#social_contracts_governance_anonymous_tally_votes_and_finalize_community_internals">anonymous_tally_votes_and_finalize_community_internals</a>(
        registry,
        proposal,
        keys,
        public_keys,
        clock,
        ctx,
    );
    <b>if</b> (outcome == <a href="../social_contracts/governance.md#social_contracts_governance_FINALIZE_OUTCOME_REJECT_COMMUNITY">FINALIZE_OUTCOME_REJECT_COMMUNITY</a>) {
        <b>let</b> dest = <a href="../social_contracts/profile.md#social_contracts_profile_get_treasury_address">profile::get_treasury_address</a>(ecosystem_treasury);
        <a href="../social_contracts/governance.md#social_contracts_governance_forfeit_proposal_pool_to_treasury_address">forfeit_proposal_pool_to_treasury_address</a>(
            registry,
            proposal,
            dest,
            <a href="../social_contracts/governance.md#social_contracts_governance_FORFEIT_REASON_COMMUNITY_REJECTED">FORFEIT_REASON_COMMUNITY_REJECTED</a>,
            <a href="../social_contracts/governance.md#social_contracts_governance_TREASURY_ROUTE_ECOSYSTEM">TREASURY_ROUTE_ECOSYSTEM</a>,
            time_ms,
            ctx
        );
    } <b>else</b> <b>if</b> (outcome == <a href="../social_contracts/governance.md#social_contracts_governance_FINALIZE_OUTCOME_REJECT_QUORUM">FINALIZE_OUTCOME_REJECT_QUORUM</a>) {
        <b>let</b> dest = <a href="../social_contracts/profile.md#social_contracts_profile_get_treasury_address">profile::get_treasury_address</a>(ecosystem_treasury);
        <a href="../social_contracts/governance.md#social_contracts_governance_forfeit_proposal_pool_to_treasury_address">forfeit_proposal_pool_to_treasury_address</a>(
            registry,
            proposal,
            dest,
            <a href="../social_contracts/governance.md#social_contracts_governance_FORFEIT_REASON_QUORUM_NOT_MET">FORFEIT_REASON_QUORUM_NOT_MET</a>,
            <a href="../social_contracts/governance.md#social_contracts_governance_TREASURY_ROUTE_ECOSYSTEM">TREASURY_ROUTE_ECOSYSTEM</a>,
            time_ms,
            ctx
        );
    };
}
</code></pre>



</details>

<a name="social_contracts_governance_mark_proposal_implemented_take_pool"></a>

## Function `mark_proposal_implemented_take_pool`

Shared implementation transition + drains <code>reward_pool</code> for routing by the caller.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_mark_proposal_implemented_take_pool">mark_proposal_implemented_take_pool</a>(registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">social_contracts::governance::Proposal</a>, description: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../myso/balance.md#myso_balance_Balance">myso::balance::Balance</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_mark_proposal_implemented_take_pool">mark_proposal_implemented_take_pool</a>(
    registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">Proposal</a>,
    description: Option&lt;String&gt;,
    clock: &Clock,
    ctx: &TxContext
): Balance&lt;MYSO&gt; {
    <b>assert</b>!(registry.<a href="../social_contracts/governance.md#social_contracts_governance_version">version</a> == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/governance.md#social_contracts_governance_EWrongVersion">EWrongVersion</a>);
    <b>let</b> caller = tx_context::sender(ctx);
    <b>let</b> current_time = clock::timestamp_ms(clock);
    <b>let</b> proposal_id = object::id(proposal);
    <b>assert</b>!(table::contains(&registry.proposals, proposal_id), <a href="../social_contracts/governance.md#social_contracts_governance_EProposalNotFound">EProposalNotFound</a>);
    <b>assert</b>!(proposal.status == <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_APPROVED">STATUS_APPROVED</a>, <a href="../social_contracts/governance.md#social_contracts_governance_EInvalidProposalStatus">EInvalidProposalStatus</a>);
    <b>assert</b>!(
        proposal.submitter == caller || table::contains(&registry.delegates, caller),
        <a href="../social_contracts/governance.md#social_contracts_governance_EUnauthorized">EUnauthorized</a>
    );
    proposal.status = <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_IMPLEMENTED">STATUS_IMPLEMENTED</a>;
    <b>let</b> from_status = table::borrow_mut(&<b>mut</b> registry.proposals_by_status, <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_APPROVED">STATUS_APPROVED</a>);
    <b>let</b> <b>mut</b> index = 0;
    <b>let</b> len = vector::length(from_status);
    <b>while</b> (index &lt; len) {
        <b>if</b> (*vector::borrow(from_status, index) == proposal_id) {
            vector::remove(from_status, index);
            <b>break</b>
        };
        index = index + 1;
    };
    <b>let</b> to_status = table::borrow_mut(&<b>mut</b> registry.proposals_by_status, <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_IMPLEMENTED">STATUS_IMPLEMENTED</a>);
    vector::push_back(to_status, proposal_id);
    event::emit(<a href="../social_contracts/governance.md#social_contracts_governance_ProposalImplementedEvent">ProposalImplementedEvent</a> {
        proposal_id,
        implementation_time: current_time,
        description,
    });
    balance::withdraw_all(&<b>mut</b> proposal.reward_pool)
}
</code></pre>



</details>

<a name="social_contracts_governance_mark_proposal_implemented_take_pool_poc_oracle"></a>

## Function `mark_proposal_implemented_take_pool_poc_oracle`

PoC oracle may implement approved media-asset rights proposals (same package only).


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_mark_proposal_implemented_take_pool_poc_oracle">mark_proposal_implemented_take_pool_poc_oracle</a>(registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">social_contracts::governance::Proposal</a>, authorized_oracle: <b>address</b>, description: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../myso/balance.md#myso_balance_Balance">myso::balance::Balance</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_mark_proposal_implemented_take_pool_poc_oracle">mark_proposal_implemented_take_pool_poc_oracle</a>(
    registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">Proposal</a>,
    authorized_oracle: <b>address</b>,
    description: Option&lt;String&gt;,
    clock: &Clock,
    ctx: &TxContext,
): Balance&lt;MYSO&gt; {
    <b>assert</b>!(registry.<a href="../social_contracts/governance.md#social_contracts_governance_version">version</a> == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/governance.md#social_contracts_governance_EWrongVersion">EWrongVersion</a>);
    <b>assert</b>!(registry.<a href="../social_contracts/governance.md#social_contracts_governance_registry_type">registry_type</a> == <a href="../social_contracts/governance.md#social_contracts_governance_PROPOSAL_TYPE_PROOF_OF_CREATIVITY">PROPOSAL_TYPE_PROOF_OF_CREATIVITY</a>, <a href="../social_contracts/governance.md#social_contracts_governance_EInvalidRegistry">EInvalidRegistry</a>);
    <b>assert</b>!(tx_context::sender(ctx) == authorized_oracle, <a href="../social_contracts/governance.md#social_contracts_governance_EUnauthorized">EUnauthorized</a>);
    <b>let</b> current_time = clock::timestamp_ms(clock);
    <b>let</b> proposal_id = object::id(proposal);
    <b>assert</b>!(table::contains(&registry.proposals, proposal_id), <a href="../social_contracts/governance.md#social_contracts_governance_EProposalNotFound">EProposalNotFound</a>);
    <b>assert</b>!(proposal.status == <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_APPROVED">STATUS_APPROVED</a>, <a href="../social_contracts/governance.md#social_contracts_governance_EInvalidProposalStatus">EInvalidProposalStatus</a>);
    proposal.status = <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_IMPLEMENTED">STATUS_IMPLEMENTED</a>;
    <b>let</b> from_status = table::borrow_mut(&<b>mut</b> registry.proposals_by_status, <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_APPROVED">STATUS_APPROVED</a>);
    <b>let</b> <b>mut</b> index = 0;
    <b>let</b> len = vector::length(from_status);
    <b>while</b> (index &lt; len) {
        <b>if</b> (*vector::borrow(from_status, index) == proposal_id) {
            vector::remove(from_status, index);
            <b>break</b>
        };
        index = index + 1;
    };
    <b>let</b> to_status = table::borrow_mut(&<b>mut</b> registry.proposals_by_status, <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_IMPLEMENTED">STATUS_IMPLEMENTED</a>);
    vector::push_back(to_status, proposal_id);
    event::emit(<a href="../social_contracts/governance.md#social_contracts_governance_ProposalImplementedEvent">ProposalImplementedEvent</a> {
        proposal_id,
        implementation_time: current_time,
        description,
    });
    balance::withdraw_all(&<b>mut</b> proposal.reward_pool)
}
</code></pre>



</details>

<a name="social_contracts_governance_mark_proposal_implemented_to_ecosystem_treasury"></a>

## Function `mark_proposal_implemented_to_ecosystem_treasury`

Mark implemented: reward the submitter with the implementation pool (ecosystem or proof-of-creativity registry).


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_mark_proposal_implemented_to_ecosystem_treasury">mark_proposal_implemented_to_ecosystem_treasury</a>(registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">social_contracts::governance::Proposal</a>, _ecosystem_treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, description: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_mark_proposal_implemented_to_ecosystem_treasury">mark_proposal_implemented_to_ecosystem_treasury</a>(
    registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">Proposal</a>,
    _ecosystem_treasury: &EcosystemTreasury,
    description: Option&lt;String&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <a href="../social_contracts/governance.md#social_contracts_governance_assert_shared_global_registry">assert_shared_global_registry</a>(registry);
    <b>let</b> sent_time = clock::timestamp_ms(clock);
    <b>let</b> submitter = proposal.submitter;
    <b>let</b> bal = <a href="../social_contracts/governance.md#social_contracts_governance_mark_proposal_implemented_take_pool">mark_proposal_implemented_take_pool</a>(registry, proposal, description, clock, ctx);
    <b>let</b> amount = balance::value(&bal);
    <b>if</b> (amount &gt; 0) {
        <b>let</b> c = coin::from_balance(bal, ctx);
        transfer::public_transfer(c, submitter);
    } <b>else</b> {
        balance::destroy_zero(bal);
    };
    event::emit(<a href="../social_contracts/governance.md#social_contracts_governance_ProposalImplementationRewardToSubmitterEvent">ProposalImplementationRewardToSubmitterEvent</a> {
        proposal_id: object::id(proposal),
        submitter,
        amount,
        <a href="../social_contracts/governance.md#social_contracts_governance_registry_type">registry_type</a>: registry.<a href="../social_contracts/governance.md#social_contracts_governance_registry_type">registry_type</a>,
        sent_time,
    });
}
</code></pre>



</details>

<a name="social_contracts_governance_mark_proposal_implemented_platform_to_ecosystem_treasury"></a>

## Function `mark_proposal_implemented_platform_to_ecosystem_treasury`

Mark implemented: platform registry when the linked [<code>Platform</code>] is not in the transaction — still pays the proposer.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_mark_proposal_implemented_platform_to_ecosystem_treasury">mark_proposal_implemented_platform_to_ecosystem_treasury</a>(registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">social_contracts::governance::Proposal</a>, _ecosystem_treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, description: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_mark_proposal_implemented_platform_to_ecosystem_treasury">mark_proposal_implemented_platform_to_ecosystem_treasury</a>(
    registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">Proposal</a>,
    _ecosystem_treasury: &EcosystemTreasury,
    description: Option&lt;String&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(registry.<a href="../social_contracts/governance.md#social_contracts_governance_registry_type">registry_type</a> == <a href="../social_contracts/governance.md#social_contracts_governance_PROPOSAL_TYPE_PLATFORM">PROPOSAL_TYPE_PLATFORM</a>, <a href="../social_contracts/governance.md#social_contracts_governance_EWrongRegistryForTreasuryRoute">EWrongRegistryForTreasuryRoute</a>);
    <b>let</b> sent_time = clock::timestamp_ms(clock);
    <b>let</b> submitter = proposal.submitter;
    <b>let</b> bal = <a href="../social_contracts/governance.md#social_contracts_governance_mark_proposal_implemented_take_pool">mark_proposal_implemented_take_pool</a>(registry, proposal, description, clock, ctx);
    <b>let</b> amount = balance::value(&bal);
    <b>if</b> (amount &gt; 0) {
        <b>let</b> c = coin::from_balance(bal, ctx);
        transfer::public_transfer(c, submitter);
    } <b>else</b> {
        balance::destroy_zero(bal);
    };
    event::emit(<a href="../social_contracts/governance.md#social_contracts_governance_ProposalImplementationRewardToSubmitterEvent">ProposalImplementationRewardToSubmitterEvent</a> {
        proposal_id: object::id(proposal),
        submitter,
        amount,
        <a href="../social_contracts/governance.md#social_contracts_governance_registry_type">registry_type</a>: registry.<a href="../social_contracts/governance.md#social_contracts_governance_registry_type">registry_type</a>,
        sent_time,
    });
}
</code></pre>



</details>

<a name="social_contracts_governance_proposal_type_platform_value"></a>

## Function `proposal_type_platform_value`

Registry type discriminator for platform routing (see [<code><a href="../social_contracts/platform.md#social_contracts_platform">social_contracts::platform</a></code>]).


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_proposal_type_platform_value">proposal_type_platform_value</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_proposal_type_platform_value">proposal_type_platform_value</a>(): u8 {
    <a href="../social_contracts/governance.md#social_contracts_governance_PROPOSAL_TYPE_PLATFORM">PROPOSAL_TYPE_PLATFORM</a>
}
</code></pre>



</details>

<a name="social_contracts_governance_proposal_type_spot_value"></a>

## Function `proposal_type_spot_value`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_proposal_type_spot_value">proposal_type_spot_value</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_proposal_type_spot_value">proposal_type_spot_value</a>(): u8 {
    <a href="../social_contracts/governance.md#social_contracts_governance_PROPOSAL_TYPE_SPOT">PROPOSAL_TYPE_SPOT</a>
}
</code></pre>



</details>

<a name="social_contracts_governance_proposal_type_proof_of_creativity_value"></a>

## Function `proposal_type_proof_of_creativity_value`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_proposal_type_proof_of_creativity_value">proposal_type_proof_of_creativity_value</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_proposal_type_proof_of_creativity_value">proposal_type_proof_of_creativity_value</a>(): u8 {
    <a href="../social_contracts/governance.md#social_contracts_governance_PROPOSAL_TYPE_PROOF_OF_CREATIVITY">PROPOSAL_TYPE_PROOF_OF_CREATIVITY</a>
}
</code></pre>



</details>

<a name="social_contracts_governance_proposal_status"></a>

## Function `proposal_status`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_proposal_status">proposal_status</a>(proposal: &<a href="../social_contracts/governance.md#social_contracts_governance_Proposal">social_contracts::governance::Proposal</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_proposal_status">proposal_status</a>(proposal: &<a href="../social_contracts/governance.md#social_contracts_governance_Proposal">Proposal</a>): u8 {
    proposal.status
}
</code></pre>



</details>

<a name="social_contracts_governance_proposal_reference_id"></a>

## Function `proposal_reference_id`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_proposal_reference_id">proposal_reference_id</a>(proposal: &<a href="../social_contracts/governance.md#social_contracts_governance_Proposal">social_contracts::governance::Proposal</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_proposal_reference_id">proposal_reference_id</a>(proposal: &<a href="../social_contracts/governance.md#social_contracts_governance_Proposal">Proposal</a>): Option&lt;ID&gt; {
    proposal.reference_id
}
</code></pre>



</details>

<a name="social_contracts_governance_status_approved_value"></a>

## Function `status_approved_value`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_status_approved_value">status_approved_value</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_status_approved_value">status_approved_value</a>(): u8 {
    <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_APPROVED">STATUS_APPROVED</a>
}
</code></pre>



</details>

<a name="social_contracts_governance_status_rejected_value"></a>

## Function `status_rejected_value`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_status_rejected_value">status_rejected_value</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_status_rejected_value">status_rejected_value</a>(): u8 {
    <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_REJECTED">STATUS_REJECTED</a>
}
</code></pre>



</details>

<a name="social_contracts_governance_get_proposals_by_type"></a>

## Function `get_proposals_by_type`

Get all proposals of a specific type


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_get_proposals_by_type">get_proposals_by_type</a>(registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, proposal_type: u8): vector&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_get_proposals_by_type">get_proposals_by_type</a>(
    registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    proposal_type: u8
): vector&lt;ID&gt; {
    <b>assert</b>!(proposal_type &lt;= <a href="../social_contracts/governance.md#social_contracts_governance_PROPOSAL_TYPE_PLATFORM">PROPOSAL_TYPE_PLATFORM</a>, <a href="../social_contracts/governance.md#social_contracts_governance_EInvalidParameter">EInvalidParameter</a>);
    <b>let</b> <b>mut</b> result = vector::empty&lt;ID&gt;();
    <b>let</b> statuses = vector[ <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_SUBMITTED">STATUS_SUBMITTED</a>, <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_DELEGATE_REVIEW">STATUS_DELEGATE_REVIEW</a>, <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_COMMUNITY_VOTING">STATUS_COMMUNITY_VOTING</a>, <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_APPROVED">STATUS_APPROVED</a>, <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_REJECTED">STATUS_REJECTED</a>, <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_IMPLEMENTED">STATUS_IMPLEMENTED</a> ];
    <b>let</b> <b>mut</b> s = 0;
    <b>while</b> (s &lt; vector::length(&statuses)) {
        <b>let</b> status = *vector::borrow(&statuses, s);
        <b>let</b> proposals_of_status: &vector&lt;ID&gt; = table::borrow(&registry.proposals_by_status, status);
        <b>let</b> <b>mut</b> p = 0;
        <b>let</b> num_proposals = vector::length(proposals_of_status);
        <b>while</b> (p &lt; num_proposals) {
            <b>let</b> proposal_id = *vector::borrow(proposals_of_status, p);
            <b>if</b> (table::contains(&registry.proposal_types, proposal_id) && *table::borrow(&registry.proposal_types, proposal_id) == proposal_type) {
                vector::push_back(&<b>mut</b> result, proposal_id);
            };
            p = p + 1;
        };
        s = s + 1;
    };
    vector::destroy_empty(statuses);
    result
}
</code></pre>



</details>

<a name="social_contracts_governance_get_proposals_by_status"></a>

## Function `get_proposals_by_status`

Get all proposals with a specific status


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_get_proposals_by_status">get_proposals_by_status</a>(registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, status: u8): vector&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_get_proposals_by_status">get_proposals_by_status</a>(
    registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    status: u8
): vector&lt;ID&gt; {
    <b>assert</b>!(status &lt;= <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_IMPLEMENTED">STATUS_IMPLEMENTED</a>, <a href="../social_contracts/governance.md#social_contracts_governance_EInvalidParameter">EInvalidParameter</a>);
    *table::borrow(&registry.proposals_by_status, status)
}
</code></pre>



</details>

<a name="social_contracts_governance_get_delegate_count"></a>

## Function `get_delegate_count`

Get number of delegates


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_get_delegate_count">get_delegate_count</a>(registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_get_delegate_count">get_delegate_count</a>(registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>): u64 {
    table::length(&registry.delegates)
}
</code></pre>



</details>

<a name="social_contracts_governance_registry_type"></a>

## Function `registry_type`

Registry type discriminator (ecosystem, proof of creativity, platform).


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_registry_type">registry_type</a>(registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_registry_type">registry_type</a>(registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>): u8 {
    registry.<a href="../social_contracts/governance.md#social_contracts_governance_registry_type">registry_type</a>
}
</code></pre>



</details>

<a name="social_contracts_governance_get_delegate_info"></a>

## Function `get_delegate_info`

Get delegate information


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_get_delegate_info">get_delegate_info</a>(registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, addr: <b>address</b>): (u64, u64, u64, u64, u64, u64, u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_get_delegate_info">get_delegate_info</a>(
    registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    addr: <b>address</b>
): (u64, u64, u64, u64, u64, u64, u64, u64) {
    <b>assert</b>!(table::contains(&registry.delegates, addr), <a href="../social_contracts/governance.md#social_contracts_governance_ENotDelegate">ENotDelegate</a>);
    <b>let</b> delegate = table::borrow(&registry.delegates, addr);
    (
        delegate.upvotes,
        delegate.downvotes,
        delegate.proposals_reviewed,
        delegate.proposals_submitted,
        delegate.sided_winning_proposals,
        delegate.sided_losing_proposals,
        delegate.term_start,
        delegate.term_end
    )
}
</code></pre>



</details>

<a name="social_contracts_governance_get_proposal_info"></a>

## Function `get_proposal_info`

Get proposal information


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_get_proposal_info">get_proposal_info</a>(registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, proposal: &<a href="../social_contracts/governance.md#social_contracts_governance_Proposal">social_contracts::governance::Proposal</a>): (<a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../std/string.md#std_string_String">std::string::String</a>, u8, <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;, <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, <b>address</b>, u64, u8, u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_get_proposal_info">get_proposal_info</a>(
    registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    proposal: &<a href="../social_contracts/governance.md#social_contracts_governance_Proposal">Proposal</a>
): (String, String, u8, Option&lt;ID&gt;, Option&lt;String&gt;, <b>address</b>, u64, u8, u64, u64) {
    <b>assert</b>!(table::contains(&registry.proposals, object::id(proposal)), <a href="../social_contracts/governance.md#social_contracts_governance_EProposalNotFound">EProposalNotFound</a>);
    (
        proposal.title,
        proposal.description,
        proposal.proposal_type,
        proposal.reference_id,
        proposal.metadata_json,
        proposal.submitter,
        proposal.submission_time,
        proposal.status,
        proposal.community_votes_for,
        proposal.community_votes_against
    )
}
</code></pre>



</details>

<a name="social_contracts_governance_proposal_submission_cost"></a>

## Function `proposal_submission_cost`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_proposal_submission_cost">proposal_submission_cost</a>(registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_proposal_submission_cost">proposal_submission_cost</a>(registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>): u64 {
    registry.<a href="../social_contracts/governance.md#social_contracts_governance_proposal_submission_cost">proposal_submission_cost</a>
}
</code></pre>



</details>

<a name="social_contracts_governance_proposal_submitter"></a>

## Function `proposal_submitter`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_proposal_submitter">proposal_submitter</a>(proposal: &<a href="../social_contracts/governance.md#social_contracts_governance_Proposal">social_contracts::governance::Proposal</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_proposal_submitter">proposal_submitter</a>(proposal: &<a href="../social_contracts/governance.md#social_contracts_governance_Proposal">Proposal</a>): <b>address</b> {
    proposal.submitter
}
</code></pre>



</details>

<a name="social_contracts_governance_treasury_balance"></a>

## Function `treasury_balance`

Get the current treasury balance


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_treasury_balance">treasury_balance</a>(registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_treasury_balance">treasury_balance</a>(registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>): u64 {
    balance::value(&registry.treasury)
}
</code></pre>



</details>

<a name="social_contracts_governance_calculate_vote_cost"></a>

## Function `calculate_vote_cost`

Calculate cost for additional votes beyond the first free vote


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_calculate_vote_cost">calculate_vote_cost</a>(vote_count: u64, registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_calculate_vote_cost">calculate_vote_cost</a>(
    vote_count: u64,
    registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>
): u64 {
    <b>if</b> (vote_count &lt;= 1) {
        <b>return</b> 0
    };
    // Quadratic cost formula: base_cost * (vote_count^2 - 1)
    registry.quadratic_base_cost * ((vote_count * vote_count) - 1)
}
</code></pre>



</details>

<a name="social_contracts_governance_is_delegate"></a>

## Function `is_delegate`

Check if an address is a delegate


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_is_delegate">is_delegate</a>(registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, addr: <b>address</b>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_is_delegate">is_delegate</a>( registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>, addr: <b>address</b> ): bool {
    table::contains(&registry.delegates, addr)
}
</code></pre>



</details>

<a name="social_contracts_governance_is_delegate_term_expired"></a>

## Function `is_delegate_term_expired`

Check if delegate term has expired


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_is_delegate_term_expired">is_delegate_term_expired</a>(registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, addr: <b>address</b>, current_epoch: u64): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_is_delegate_term_expired">is_delegate_term_expired</a>(
    registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    addr: <b>address</b>,
    current_epoch: u64
): bool {
    <b>if</b> (!table::contains(&registry.delegates, addr)) {
        <b>return</b> <b>false</b>
    };
    <b>let</b> delegate = table::borrow(&registry.delegates, addr);
    delegate.term_end &lt;= current_epoch
}
</code></pre>



</details>

<a name="social_contracts_governance_get_governance_parameters"></a>

## Function `get_governance_parameters`

Get governance parameters


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_get_governance_parameters">get_governance_parameters</a>(registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>): (u64, u64, u64, u64, u64, u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_get_governance_parameters">get_governance_parameters</a>(
    registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>
): (u64, u64, u64, u64, u64, u64, u64) {
    (
        registry.delegate_count,
        registry.delegate_term_epochs,
        registry.<a href="../social_contracts/governance.md#social_contracts_governance_proposal_submission_cost">proposal_submission_cost</a>,
        registry.max_votes_per_user,
        registry.quadratic_base_cost,
        registry.voting_period_ms,
        registry.quorum_votes
    )
}
</code></pre>



</details>

<a name="social_contracts_governance_reject_proposal_manually"></a>

## Function `reject_proposal_manually`

If more than half of delegates reject, reject the proposal manually (ecosystem / proof-of-creativity only).


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_reject_proposal_manually">reject_proposal_manually</a>(registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">social_contracts::governance::Proposal</a>, ecosystem_treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_reject_proposal_manually">reject_proposal_manually</a>(
    registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">Proposal</a>,
    ecosystem_treasury: &EcosystemTreasury,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(registry.<a href="../social_contracts/governance.md#social_contracts_governance_version">version</a> == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/governance.md#social_contracts_governance_EWrongVersion">EWrongVersion</a>);
    <a href="../social_contracts/governance.md#social_contracts_governance_assert_shared_global_registry">assert_shared_global_registry</a>(registry);
    <b>let</b> caller = tx_context::sender(ctx);
    <b>assert</b>!(table::contains(&registry.delegates, caller), <a href="../social_contracts/governance.md#social_contracts_governance_ENotDelegate">ENotDelegate</a>);
    <b>let</b> proposal_id = object::id(proposal);
    <b>assert</b>!(table::contains(&registry.proposals, proposal_id), <a href="../social_contracts/governance.md#social_contracts_governance_EProposalNotFound">EProposalNotFound</a>);
    <b>assert</b>!(proposal.status == <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_DELEGATE_REVIEW">STATUS_DELEGATE_REVIEW</a>, <a href="../social_contracts/governance.md#social_contracts_governance_EInvalidProposalStatus">EInvalidProposalStatus</a>);
    <b>assert</b>!(!<a href="../social_contracts/governance.md#social_contracts_governance_council_jury_has_jurisdiction">council_jury_has_jurisdiction</a>(proposal), <a href="../social_contracts/governance.md#social_contracts_governance_EJuryHasJurisdiction">EJuryHasJurisdiction</a>);
    <b>let</b> current_time = clock::timestamp_ms(clock);
    <a href="../social_contracts/governance.md#social_contracts_governance_complete_delegate_reject_to_ecosystem_treasury">complete_delegate_reject_to_ecosystem_treasury</a>(
        registry,
        proposal,
        current_time,
        ecosystem_treasury,
        ctx
    );
}
</code></pre>



</details>

<a name="social_contracts_governance_create_platform_governance"></a>

## Function `create_platform_governance`

Create a platform-specific governance registry when a platform is approved
This function can only be called by the platform toggle_platform_approval function
The transaction sender is seeded as the founding delegate (same pattern as <code><a href="../social_contracts/governance.md#social_contracts_governance_bootstrap_init">bootstrap_init</a></code>).


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_create_platform_governance">create_platform_governance</a>(delegate_count: u64, delegate_term_epochs: u64, <a href="../social_contracts/governance.md#social_contracts_governance_proposal_submission_cost">proposal_submission_cost</a>: u64, max_votes_per_user: u64, quadratic_base_cost: u64, voting_period_ms: u64, quorum_votes: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_create_platform_governance">create_platform_governance</a>(
    delegate_count: u64,
    delegate_term_epochs: u64,
    <a href="../social_contracts/governance.md#social_contracts_governance_proposal_submission_cost">proposal_submission_cost</a>: u64,
    max_votes_per_user: u64,
    quadratic_base_cost: u64,
    voting_period_ms: u64,
    quorum_votes: u64,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
): ID {
    <b>let</b> current_time = clock::timestamp_ms(clock);
    <b>let</b> founding_delegate = tx_context::sender(ctx);
    // Create Platform Governance Registry with parameters
    <b>let</b> <b>mut</b> platform_registry = <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a> {
        id: object::new(ctx),
        <a href="../social_contracts/governance.md#social_contracts_governance_registry_type">registry_type</a>: <a href="../social_contracts/governance.md#social_contracts_governance_PROPOSAL_TYPE_PLATFORM">PROPOSAL_TYPE_PLATFORM</a>,
        // Configuration parameters specific to <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> <a href="../social_contracts/governance.md#social_contracts_governance">governance</a>
        delegate_count,
        delegate_term_epochs,
        <a href="../social_contracts/governance.md#social_contracts_governance_proposal_submission_cost">proposal_submission_cost</a>,
        max_votes_per_user,
        quadratic_base_cost,
        voting_period_ms,
        quorum_votes,
        // Tables
        delegates: table::new&lt;<b>address</b>, <a href="../social_contracts/governance.md#social_contracts_governance_Delegate">Delegate</a>&gt;(ctx),
        proposals: table::new&lt;ID, bool&gt;(ctx),
        proposal_types: table::new&lt;ID, u8&gt;(ctx),
        proposals_by_status: table::new&lt;u8, vector&lt;ID&gt;&gt;(ctx),
        treasury: balance::zero(),
        nominated_delegates: table::new&lt;<b>address</b>, <a href="../social_contracts/governance.md#social_contracts_governance_NominatedDelegate">NominatedDelegate</a>&gt;(ctx),
        delegate_addresses: vec_set::empty&lt;<b>address</b>&gt;(),
        nominee_addresses: vec_set::empty&lt;<b>address</b>&gt;(),
        voters: table::new&lt;<b>address</b>, Table&lt;<b>address</b>, bool&gt;&gt;(ctx),
        <a href="../social_contracts/governance.md#social_contracts_governance_version">version</a>: <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(),
        <a href="../social_contracts/governance.md#social_contracts_governance_last_delegate_panel_boundary_epoch">last_delegate_panel_boundary_epoch</a>: 0,
    };
    // Initialize registry tables
    <a href="../social_contracts/governance.md#social_contracts_governance_initialize_registry_tables">initialize_registry_tables</a>(&<b>mut</b> platform_registry, ctx);
    <a href="../social_contracts/governance.md#social_contracts_governance_seed_founding_delegate">seed_founding_delegate</a>(&<b>mut</b> platform_registry, founding_delegate, ctx);
    // Get the ID before sharing
    <b>let</b> registry_id = object::id(&platform_registry);
    // Emit event <b>for</b> <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> registry creation
    event::emit(<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceRegistryCreatedEvent">GovernanceRegistryCreatedEvent</a> {
        registry_id,
        <a href="../social_contracts/governance.md#social_contracts_governance_registry_type">registry_type</a>: <a href="../social_contracts/governance.md#social_contracts_governance_PROPOSAL_TYPE_PLATFORM">PROPOSAL_TYPE_PLATFORM</a>,
        delegate_count: platform_registry.delegate_count,
        delegate_term_epochs: platform_registry.delegate_term_epochs,
        <a href="../social_contracts/governance.md#social_contracts_governance_proposal_submission_cost">proposal_submission_cost</a>: platform_registry.<a href="../social_contracts/governance.md#social_contracts_governance_proposal_submission_cost">proposal_submission_cost</a>,
        max_votes_per_user: platform_registry.max_votes_per_user,
        quadratic_base_cost: platform_registry.quadratic_base_cost,
        voting_period_ms: platform_registry.voting_period_ms,
        quorum_votes: platform_registry.quorum_votes,
        updated_at: current_time,
    });
    // Share the registry object
    transfer::share_object(platform_registry);
    // Return the registry ID
    registry_id
}
</code></pre>



</details>

<a name="social_contracts_governance_version"></a>

## Function `version`

Get version of GovernanceDAO


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_version">version</a>(registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_version">version</a>(registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>): u64 {
    registry.<a href="../social_contracts/governance.md#social_contracts_governance_version">version</a>
}
</code></pre>



</details>

<a name="social_contracts_governance_last_delegate_panel_boundary_epoch"></a>

## Function `last_delegate_panel_boundary_epoch`

Largest <code>k * delegate_term_epochs</code> boundary for which a full delegate panel election has been applied.


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_last_delegate_panel_boundary_epoch">last_delegate_panel_boundary_epoch</a>(registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_last_delegate_panel_boundary_epoch">last_delegate_panel_boundary_epoch</a>(registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>): u64 {
    registry.<a href="../social_contracts/governance.md#social_contracts_governance_last_delegate_panel_boundary_epoch">last_delegate_panel_boundary_epoch</a>
}
</code></pre>



</details>

<a name="social_contracts_governance_set_version"></a>

## Function `set_version`

Set version of GovernanceDAO


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_set_version">set_version</a>(registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, new_version: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_set_version">set_version</a>(registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>, new_version: u64) {
    registry.<a href="../social_contracts/governance.md#social_contracts_governance_version">version</a> = new_version;
}
</code></pre>



</details>

<a name="social_contracts_governance_migrate_registry"></a>

## Function `migrate_registry`

Public entry function that migrates registry to the latest version


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_migrate_registry">migrate_registry</a>(registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, _: &<a href="../social_contracts/upgrade.md#social_contracts_upgrade_UpgradeAdminCap">social_contracts::upgrade::UpgradeAdminCap</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_migrate_registry">migrate_registry</a>(
    registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    _: &UpgradeAdminCap,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> current_version = registry.<a href="../social_contracts/governance.md#social_contracts_governance_version">version</a>;
    <b>let</b> latest_version = <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>();
    <b>assert</b>!(current_version &lt; latest_version, <a href="../social_contracts/governance.md#social_contracts_governance_EWrongVersion">EWrongVersion</a>);
    // Version-specific migrations would go here when needed
    <b>let</b> old_version = current_version;
    registry.<a href="../social_contracts/governance.md#social_contracts_governance_version">version</a> = latest_version;
    <a href="../social_contracts/upgrade.md#social_contracts_upgrade_emit_migration_event">upgrade::emit_migration_event</a>(
        object::id(registry),
        string::utf8(b"<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>"),
        old_version,
        tx_context::sender(ctx),
    );
}
</code></pre>



</details>

<a name="social_contracts_governance_create_spot_governance_registry"></a>

## Function `create_spot_governance_registry`

One-time admin path for existing networks that bootstrapped before the SPoT registry existed.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_create_spot_governance_registry">create_spot_governance_registry</a>(_: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceAdminCap">social_contracts::governance::GovernanceAdminCap</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_create_spot_governance_registry">create_spot_governance_registry</a>(
    _: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceAdminCap">GovernanceAdminCap</a>,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> current_time = clock::timestamp_ms(clock);
    <b>let</b> founder = tx_context::sender(ctx);
    <b>let</b> <b>mut</b> spot_registry = <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a> {
        id: object::new(ctx),
        <a href="../social_contracts/governance.md#social_contracts_governance_registry_type">registry_type</a>: <a href="../social_contracts/governance.md#social_contracts_governance_PROPOSAL_TYPE_SPOT">PROPOSAL_TYPE_SPOT</a>,
        delegate_count: 3,
        delegate_term_epochs: 90,
        <a href="../social_contracts/governance.md#social_contracts_governance_proposal_submission_cost">proposal_submission_cost</a>: 10_000_000_000,
        max_votes_per_user: 5,
        quadratic_base_cost: 1_000_000_000,
        voting_period_ms: 3 * 24 * 60 * 60 * 1000,
        quorum_votes: 10,
        delegates: table::new&lt;<b>address</b>, <a href="../social_contracts/governance.md#social_contracts_governance_Delegate">Delegate</a>&gt;(ctx),
        proposals: table::new&lt;ID, bool&gt;(ctx),
        proposal_types: table::new&lt;ID, u8&gt;(ctx),
        proposals_by_status: table::new&lt;u8, vector&lt;ID&gt;&gt;(ctx),
        treasury: balance::zero(),
        nominated_delegates: table::new&lt;<b>address</b>, <a href="../social_contracts/governance.md#social_contracts_governance_NominatedDelegate">NominatedDelegate</a>&gt;(ctx),
        delegate_addresses: vec_set::empty&lt;<b>address</b>&gt;(),
        nominee_addresses: vec_set::empty&lt;<b>address</b>&gt;(),
        voters: table::new&lt;<b>address</b>, Table&lt;<b>address</b>, bool&gt;&gt;(ctx),
        <a href="../social_contracts/governance.md#social_contracts_governance_version">version</a>: <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(),
        <a href="../social_contracts/governance.md#social_contracts_governance_last_delegate_panel_boundary_epoch">last_delegate_panel_boundary_epoch</a>: 0,
    };
    <a href="../social_contracts/governance.md#social_contracts_governance_initialize_registry_tables">initialize_registry_tables</a>(&<b>mut</b> spot_registry, ctx);
    <b>let</b> spot_registry_id = object::id(&spot_registry);
    event::emit(<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceRegistryCreatedEvent">GovernanceRegistryCreatedEvent</a> {
        registry_id: spot_registry_id,
        <a href="../social_contracts/governance.md#social_contracts_governance_registry_type">registry_type</a>: <a href="../social_contracts/governance.md#social_contracts_governance_PROPOSAL_TYPE_SPOT">PROPOSAL_TYPE_SPOT</a>,
        delegate_count: spot_registry.delegate_count,
        delegate_term_epochs: spot_registry.delegate_term_epochs,
        <a href="../social_contracts/governance.md#social_contracts_governance_proposal_submission_cost">proposal_submission_cost</a>: spot_registry.<a href="../social_contracts/governance.md#social_contracts_governance_proposal_submission_cost">proposal_submission_cost</a>,
        max_votes_per_user: spot_registry.max_votes_per_user,
        quadratic_base_cost: spot_registry.quadratic_base_cost,
        voting_period_ms: spot_registry.voting_period_ms,
        quorum_votes: spot_registry.quorum_votes,
        updated_at: current_time,
    });
    <a href="../social_contracts/governance.md#social_contracts_governance_seed_founding_delegate">seed_founding_delegate</a>(&<b>mut</b> spot_registry, founder, ctx);
    transfer::share_object(spot_registry);
}
</code></pre>



</details>

<a name="social_contracts_governance_create_governance_admin_cap"></a>

## Function `create_governance_admin_cap`

Create a GovernanceAdminCap for bootstrap (package visibility only)
This function is only callable by other modules in the same package


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_create_governance_admin_cap">create_governance_admin_cap</a>(ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceAdminCap">social_contracts::governance::GovernanceAdminCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_create_governance_admin_cap">create_governance_admin_cap</a>(ctx: &<b>mut</b> TxContext): <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceAdminCap">GovernanceAdminCap</a> {
    <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceAdminCap">GovernanceAdminCap</a> {
        id: object::new(ctx)
    }
}
</code></pre>



</details>

<a name="social_contracts_governance_jury_state"></a>

## Function `jury_state`



<pre><code><b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_jury_state">jury_state</a>(proposal: &<a href="../social_contracts/governance.md#social_contracts_governance_Proposal">social_contracts::governance::Proposal</a>, field: vector&lt;u8&gt;): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_jury_state">jury_state</a>(proposal: &<a href="../social_contracts/governance.md#social_contracts_governance_Proposal">Proposal</a>, field: vector&lt;u8&gt;): u8 {
    <b>if</b> (!dynamic_field::exists_(&proposal.id, field)) {
        <b>return</b> 0
    };
    <b>let</b> review: &<a href="../social_contracts/governance.md#social_contracts_governance_JuryReview">JuryReview</a> = dynamic_field::borrow(&proposal.id, field);
    review.state
}
</code></pre>



</details>

<a name="social_contracts_governance_council_jury_has_jurisdiction"></a>

## Function `council_jury_has_jurisdiction`

True once the jury has the council step (or has already decided it): delegates can no longer
vote, reject or rescind.


<pre><code><b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_council_jury_has_jurisdiction">council_jury_has_jurisdiction</a>(proposal: &<a href="../social_contracts/governance.md#social_contracts_governance_Proposal">social_contracts::governance::Proposal</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_council_jury_has_jurisdiction">council_jury_has_jurisdiction</a>(proposal: &<a href="../social_contracts/governance.md#social_contracts_governance_Proposal">Proposal</a>): bool {
    <b>let</b> state = <a href="../social_contracts/governance.md#social_contracts_governance_jury_state">jury_state</a>(proposal, <a href="../social_contracts/governance.md#social_contracts_governance_JURY_COUNCIL_FIELD">JURY_COUNCIL_FIELD</a>);
    state == <a href="../social_contracts/governance.md#social_contracts_governance_JURY_STATE_ESCALATED">JURY_STATE_ESCALATED</a> || state == <a href="../social_contracts/governance.md#social_contracts_governance_JURY_STATE_DECIDED">JURY_STATE_DECIDED</a>
}
</code></pre>



</details>

<a name="social_contracts_governance_jury_policy_of"></a>

## Function `jury_policy_of`



<pre><code><b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_jury_policy_of">jury_policy_of</a>(registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../social_contracts/governance.md#social_contracts_governance_RegistryJuryPolicy">social_contracts::governance::RegistryJuryPolicy</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_jury_policy_of">jury_policy_of</a>(registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>): Option&lt;<a href="../social_contracts/governance.md#social_contracts_governance_RegistryJuryPolicy">RegistryJuryPolicy</a>&gt; {
    <b>if</b> (dynamic_field::exists_(&registry.id, <a href="../social_contracts/governance.md#social_contracts_governance_JURY_POLICY_FIELD">JURY_POLICY_FIELD</a>)) {
        <b>let</b> policy: &<a href="../social_contracts/governance.md#social_contracts_governance_RegistryJuryPolicy">RegistryJuryPolicy</a> = dynamic_field::borrow(&registry.id, <a href="../social_contracts/governance.md#social_contracts_governance_JURY_POLICY_FIELD">JURY_POLICY_FIELD</a>);
        option::some(*policy)
    } <b>else</b> {
        option::none()
    }
}
</code></pre>



</details>

<a name="social_contracts_governance_community_jury_available"></a>

## Function `community_jury_available`

Whether a quorum miss or tie at the community step goes to the jury: the policy is on and the
pool is big enough to pay jurors. Decided from the registry and proposal alone so the
existing finalize entries need no new arguments.


<pre><code><b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_community_jury_available">community_jury_available</a>(registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, proposal: &<a href="../social_contracts/governance.md#social_contracts_governance_Proposal">social_contracts::governance::Proposal</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_community_jury_available">community_jury_available</a>(registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>, proposal: &<a href="../social_contracts/governance.md#social_contracts_governance_Proposal">Proposal</a>): bool {
    <b>let</b> policy = <a href="../social_contracts/governance.md#social_contracts_governance_jury_policy_of">jury_policy_of</a>(registry);
    <b>if</b> (option::is_none(&policy)) {
        <b>return</b> <b>false</b>
    };
    <b>let</b> policy = option::destroy_some(policy);
    policy.community_stage_enabled
        && balance::value(&proposal.reward_pool) &gt;= policy.min_pool_for_jury_mist
        && policy.min_pool_for_jury_mist &gt; 0
}
</code></pre>



</details>

<a name="social_contracts_governance_set_registry_jury_policy"></a>

## Function `set_registry_jury_policy`

Set (or replace) a registry's jury policy. Until one is set the jury does nothing.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_set_registry_jury_policy">set_registry_jury_policy</a>(_: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceAdminCap">social_contracts::governance::GovernanceAdminCap</a>, registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, council_stage_enabled: bool, community_stage_enabled: bool, council_window_ms: u64, jury_size: u64, round2_size: u64, jury_fee_bps: u64, jury_fee_cap_mist: u64, min_pool_for_jury_mist: u64, quorum: u64, approval_bps_council: u64, approval_bps_community: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_set_registry_jury_policy">set_registry_jury_policy</a>(
    _: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceAdminCap">GovernanceAdminCap</a>,
    registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    council_stage_enabled: bool,
    community_stage_enabled: bool,
    council_window_ms: u64,
    jury_size: u64,
    round2_size: u64,
    jury_fee_bps: u64,
    jury_fee_cap_mist: u64,
    min_pool_for_jury_mist: u64,
    quorum: u64,
    approval_bps_council: u64,
    approval_bps_community: u64,
    clock: &Clock,
) {
    <a href="../social_contracts/governance.md#social_contracts_governance_set_jury_policy_internal">set_jury_policy_internal</a>(
        registry,
        council_stage_enabled,
        community_stage_enabled,
        council_window_ms,
        jury_size,
        round2_size,
        jury_fee_bps,
        jury_fee_cap_mist,
        min_pool_for_jury_mist,
        quorum,
        approval_bps_council,
        approval_bps_community,
        clock,
    );
}
</code></pre>



</details>

<a name="social_contracts_governance_set_jury_policy_internal"></a>

## Function `set_jury_policy_internal`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_set_jury_policy_internal">set_jury_policy_internal</a>(registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, council_stage_enabled: bool, community_stage_enabled: bool, council_window_ms: u64, jury_size: u64, round2_size: u64, jury_fee_bps: u64, jury_fee_cap_mist: u64, min_pool_for_jury_mist: u64, quorum: u64, approval_bps_council: u64, approval_bps_community: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_set_jury_policy_internal">set_jury_policy_internal</a>(
    registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    council_stage_enabled: bool,
    community_stage_enabled: bool,
    council_window_ms: u64,
    jury_size: u64,
    round2_size: u64,
    jury_fee_bps: u64,
    jury_fee_cap_mist: u64,
    min_pool_for_jury_mist: u64,
    quorum: u64,
    approval_bps_council: u64,
    approval_bps_community: u64,
    clock: &Clock,
) {
    <b>assert</b>!(registry.<a href="../social_contracts/governance.md#social_contracts_governance_version">version</a> == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/governance.md#social_contracts_governance_EWrongVersion">EWrongVersion</a>);
    <b>assert</b>!(quorum &gt;= 1 && jury_size &gt;= quorum && round2_size &gt;= quorum, <a href="../social_contracts/governance.md#social_contracts_governance_EInvalidJuryPolicy">EInvalidJuryPolicy</a>);
    <b>assert</b>!(jury_fee_bps &lt;= 5000, <a href="../social_contracts/governance.md#social_contracts_governance_EInvalidJuryPolicy">EInvalidJuryPolicy</a>);
    <b>assert</b>!(approval_bps_council &gt;= 5000 && approval_bps_council &lt;= 10000, <a href="../social_contracts/governance.md#social_contracts_governance_EInvalidJuryPolicy">EInvalidJuryPolicy</a>);
    <b>assert</b>!(approval_bps_community &gt;= 5000 && approval_bps_community &lt;= 10000, <a href="../social_contracts/governance.md#social_contracts_governance_EInvalidJuryPolicy">EInvalidJuryPolicy</a>);
    // A <a href="../social_contracts/jury.md#social_contracts_jury">jury</a> that is switched on but cannot be paid would be a free vote.
    <b>assert</b>!(
        !(council_stage_enabled || community_stage_enabled) || (jury_fee_bps &gt; 0 && min_pool_for_jury_mist &gt; 0),
        <a href="../social_contracts/governance.md#social_contracts_governance_EInvalidJuryPolicy">EInvalidJuryPolicy</a>,
    );
    <b>let</b> now = clock::timestamp_ms(clock);
    <b>let</b> existing = <a href="../social_contracts/governance.md#social_contracts_governance_jury_policy_of">jury_policy_of</a>(registry);
    <b>let</b> enabled_at_ms = <b>if</b> (option::is_some(&existing) && option::borrow(&existing).enabled_at_ms &gt; 0) {
        option::borrow(&existing).enabled_at_ms
    } <b>else</b> {
        now
    };
    <b>let</b> policy = <a href="../social_contracts/governance.md#social_contracts_governance_RegistryJuryPolicy">RegistryJuryPolicy</a> {
        council_stage_enabled,
        community_stage_enabled,
        enabled_at_ms,
        council_window_ms,
        jury_size,
        round2_size,
        jury_fee_bps,
        jury_fee_cap_mist,
        min_pool_for_jury_mist,
        quorum,
        approval_bps_council,
        approval_bps_community,
    };
    <b>if</b> (dynamic_field::exists_(&registry.id, <a href="../social_contracts/governance.md#social_contracts_governance_JURY_POLICY_FIELD">JURY_POLICY_FIELD</a>)) {
        <b>let</b> stored: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_RegistryJuryPolicy">RegistryJuryPolicy</a> = dynamic_field::borrow_mut(&<b>mut</b> registry.id, <a href="../social_contracts/governance.md#social_contracts_governance_JURY_POLICY_FIELD">JURY_POLICY_FIELD</a>);
        *stored = policy;
    } <b>else</b> {
        dynamic_field::add(&<b>mut</b> registry.id, <a href="../social_contracts/governance.md#social_contracts_governance_JURY_POLICY_FIELD">JURY_POLICY_FIELD</a>, policy);
    };
    event::emit(<a href="../social_contracts/governance.md#social_contracts_governance_JuryPolicyUpdatedEvent">JuryPolicyUpdatedEvent</a> {
        registry_id: object::id(registry),
        council_stage_enabled,
        community_stage_enabled,
        council_window_ms,
        jury_size,
        round2_size,
        jury_fee_bps,
        jury_fee_cap_mist,
        min_pool_for_jury_mist,
        quorum,
        approval_bps_council,
        approval_bps_community,
        updated_at_ms: now,
    });
}
</code></pre>



</details>

<a name="social_contracts_governance_live_referrer_count"></a>

## Function `live_referrer_count`

Delegates in the registry who are among the referrers of this proposal's council step.


<pre><code><b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_live_referrer_count">live_referrer_count</a>(registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, review: &<a href="../social_contracts/governance.md#social_contracts_governance_JuryReview">social_contracts::governance::JuryReview</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_live_referrer_count">live_referrer_count</a>(registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>, review: &<a href="../social_contracts/governance.md#social_contracts_governance_JuryReview">JuryReview</a>): u64 {
    <b>let</b> <b>mut</b> count = 0;
    <b>let</b> <b>mut</b> i = 0;
    <b>let</b> len = vector::length(&review.referrers);
    <b>while</b> (i &lt; len) {
        <b>if</b> (table::contains(&registry.delegates, *vector::borrow(&review.referrers, i))) {
            count = count + 1;
        };
        i = i + 1;
    };
    count
}
</code></pre>



</details>

<a name="social_contracts_governance_live_voter_count"></a>

## Function `live_voter_count`

Current delegates who have voted approve or reject on the proposal.


<pre><code><b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_live_voter_count">live_voter_count</a>(registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, proposal: &<a href="../social_contracts/governance.md#social_contracts_governance_Proposal">social_contracts::governance::Proposal</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_live_voter_count">live_voter_count</a>(registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>, proposal: &<a href="../social_contracts/governance.md#social_contracts_governance_Proposal">Proposal</a>): u64 {
    <b>let</b> votes: &Table&lt;<b>address</b>, bool&gt; = dynamic_field::borrow(&proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_DELEGATE_VOTES_FIELD">DELEGATE_VOTES_FIELD</a>);
    <b>let</b> addrs = vec_set::keys(&registry.delegate_addresses);
    <b>let</b> <b>mut</b> count = 0;
    <b>let</b> <b>mut</b> i = 0;
    <b>let</b> len = vector::length(addrs);
    <b>while</b> (i &lt; len) {
        <b>let</b> addr = *vector::borrow(addrs, i);
        <b>if</b> (table::contains(&registry.delegates, addr) && table::contains(votes, addr)) {
            count = count + 1;
        };
        i = i + 1;
    };
    count
}
</code></pre>



</details>

<a name="social_contracts_governance_delegate_refer_to_jury"></a>

## Function `delegate_refer_to_jury`

A delegate asks the AI jury to take the council step instead of voting. When more than half
of the current delegates have asked, anyone may escalate. Referrals never enter the delegate
vote tables, so finalization statistics are unaffected.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_delegate_refer_to_jury">delegate_refer_to_jury</a>(registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">social_contracts::governance::Proposal</a>, reason: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_delegate_refer_to_jury">delegate_refer_to_jury</a>(
    registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">Proposal</a>,
    reason: Option&lt;String&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(registry.<a href="../social_contracts/governance.md#social_contracts_governance_version">version</a> == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/governance.md#social_contracts_governance_EWrongVersion">EWrongVersion</a>);
    <a href="../social_contracts/governance.md#social_contracts_governance_try_update_delegate_panel_if_due">try_update_delegate_panel_if_due</a>(registry, ctx);
    <b>let</b> caller = tx_context::sender(ctx);
    <b>let</b> proposal_id = object::id(proposal);
    <b>assert</b>!(table::contains(&registry.proposals, proposal_id), <a href="../social_contracts/governance.md#social_contracts_governance_EProposalNotFound">EProposalNotFound</a>);
    <b>assert</b>!(table::contains(&registry.delegates, caller), <a href="../social_contracts/governance.md#social_contracts_governance_ENotDelegate">ENotDelegate</a>);
    <b>assert</b>!(proposal.status == <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_DELEGATE_REVIEW">STATUS_DELEGATE_REVIEW</a>, <a href="../social_contracts/governance.md#social_contracts_governance_EInvalidProposalStatus">EInvalidProposalStatus</a>);
    <b>assert</b>!(!<a href="../social_contracts/governance.md#social_contracts_governance_council_jury_has_jurisdiction">council_jury_has_jurisdiction</a>(proposal), <a href="../social_contracts/governance.md#social_contracts_governance_EJuryHasJurisdiction">EJuryHasJurisdiction</a>);
    <b>let</b> policy = <a href="../social_contracts/governance.md#social_contracts_governance_jury_policy_of">jury_policy_of</a>(registry);
    <b>assert</b>!(option::is_some(&policy), <a href="../social_contracts/governance.md#social_contracts_governance_EJuryStageDisabled">EJuryStageDisabled</a>);
    <b>let</b> policy = option::destroy_some(policy);
    <b>assert</b>!(policy.council_stage_enabled, <a href="../social_contracts/governance.md#social_contracts_governance_EJuryStageDisabled">EJuryStageDisabled</a>);
    <b>assert</b>!(balance::value(&proposal.reward_pool) &gt;= policy.min_pool_for_jury_mist, <a href="../social_contracts/governance.md#social_contracts_governance_EPoolTooSmallForJury">EPoolTooSmallForJury</a>);
    {
        <b>let</b> votes: &Table&lt;<b>address</b>, bool&gt; = dynamic_field::borrow(&proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_DELEGATE_VOTES_FIELD">DELEGATE_VOTES_FIELD</a>);
        <b>assert</b>!(!table::contains(votes, caller), <a href="../social_contracts/governance.md#social_contracts_governance_EDelegateAlreadyActed">EDelegateAlreadyActed</a>);
    };
    <b>if</b> (!dynamic_field::exists_(&proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_JURY_COUNCIL_FIELD">JURY_COUNCIL_FIELD</a>)) {
        dynamic_field::add(
            &<b>mut</b> proposal.id,
            <a href="../social_contracts/governance.md#social_contracts_governance_JURY_COUNCIL_FIELD">JURY_COUNCIL_FIELD</a>,
            <a href="../social_contracts/governance.md#social_contracts_governance_JuryReview">JuryReview</a> {
                stage: <a href="../social_contracts/governance.md#social_contracts_governance_JURY_STAGE_COUNCIL">JURY_STAGE_COUNCIL</a>,
                state: <a href="../social_contracts/governance.md#social_contracts_governance_JURY_STATE_REFERRING">JURY_STATE_REFERRING</a>,
                trigger: 0,
                referrers: vector::empty(),
                case_id: option::none(),
                escalated_at_ms: 0,
                decided_at_ms: 0,
                decision: <a href="../social_contracts/governance.md#social_contracts_governance_NO_DECISION">NO_DECISION</a>,
                deferred_since_ms: 0,
            },
        );
    };
    <b>let</b> total_delegates = table::length(&registry.delegates);
    <b>let</b> (referral_count, majority_reached) = {
        <b>let</b> review: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_JuryReview">JuryReview</a> = dynamic_field::borrow_mut(&<b>mut</b> proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_JURY_COUNCIL_FIELD">JURY_COUNCIL_FIELD</a>);
        <b>assert</b>!(!vector::contains(&review.referrers, &caller), <a href="../social_contracts/governance.md#social_contracts_governance_EAlreadyReferred">EAlreadyReferred</a>);
        vector::push_back(&<b>mut</b> review.referrers, caller);
        <b>let</b> count = <a href="../social_contracts/governance.md#social_contracts_governance_live_referrer_count">live_referrer_count</a>(registry, review);
        <b>let</b> majority = count &gt; total_delegates / 2;
        <b>if</b> (majority && review.trigger == 0) {
            review.trigger = <a href="../social_contracts/governance.md#social_contracts_governance_JURY_TRIGGER_REFERRAL_MAJORITY">JURY_TRIGGER_REFERRAL_MAJORITY</a>;
        };
        (count, majority)
    };
    event::emit(<a href="../social_contracts/governance.md#social_contracts_governance_DelegateReferredToJuryEvent">DelegateReferredToJuryEvent</a> {
        proposal_id,
        delegate_address: caller,
        reason,
        referral_count,
        majority_reached,
        referred_at_ms: clock::timestamp_ms(clock),
    });
}
</code></pre>



</details>

<a name="social_contracts_governance_council_trigger"></a>

## Function `council_trigger`

Why the council step may go to the jury now, or aborts if it may not: a referral majority,
every current delegate having acted without a majority, or the council window running out.


<pre><code><b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_council_trigger">council_trigger</a>(registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, proposal: &<a href="../social_contracts/governance.md#social_contracts_governance_Proposal">social_contracts::governance::Proposal</a>, policy: &<a href="../social_contracts/governance.md#social_contracts_governance_RegistryJuryPolicy">social_contracts::governance::RegistryJuryPolicy</a>, now: u64): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_council_trigger">council_trigger</a>(
    registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    proposal: &<a href="../social_contracts/governance.md#social_contracts_governance_Proposal">Proposal</a>,
    policy: &<a href="../social_contracts/governance.md#social_contracts_governance_RegistryJuryPolicy">RegistryJuryPolicy</a>,
    now: u64,
): u8 {
    <b>let</b> total = table::length(&registry.delegates);
    <b>let</b> referrers = <b>if</b> (dynamic_field::exists_(&proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_JURY_COUNCIL_FIELD">JURY_COUNCIL_FIELD</a>)) {
        <b>let</b> review: &<a href="../social_contracts/governance.md#social_contracts_governance_JuryReview">JuryReview</a> = dynamic_field::borrow(&proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_JURY_COUNCIL_FIELD">JURY_COUNCIL_FIELD</a>);
        <a href="../social_contracts/governance.md#social_contracts_governance_live_referrer_count">live_referrer_count</a>(registry, review)
    } <b>else</b> {
        0
    };
    <b>if</b> (referrers &gt; total / 2) {
        <b>return</b> <a href="../social_contracts/governance.md#social_contracts_governance_JURY_TRIGGER_REFERRAL_MAJORITY">JURY_TRIGGER_REFERRAL_MAJORITY</a>
    };
    <b>if</b> (total &gt; 0 && <a href="../social_contracts/governance.md#social_contracts_governance_live_voter_count">live_voter_count</a>(registry, proposal) + referrers &gt;= total) {
        <b>return</b> <a href="../social_contracts/governance.md#social_contracts_governance_JURY_TRIGGER_COUNCIL_TIE">JURY_TRIGGER_COUNCIL_TIE</a>
    };
    <b>let</b> start = <b>if</b> (proposal.submission_time &gt; policy.enabled_at_ms) {
        proposal.submission_time
    } <b>else</b> {
        policy.enabled_at_ms
    };
    <b>if</b> (policy.council_window_ms &gt; 0 && now &gt; start + policy.council_window_ms) {
        <b>return</b> <a href="../social_contracts/governance.md#social_contracts_governance_JURY_TRIGGER_COUNCIL_DEADLINE">JURY_TRIGGER_COUNCIL_DEADLINE</a>
    };
    <b>abort</b> <a href="../social_contracts/governance.md#social_contracts_governance_ENoEscalationTrigger">ENoEscalationTrigger</a>
}
</code></pre>



</details>

<a name="social_contracts_governance_proposal_content_digest"></a>

## Function `proposal_content_digest`

Content the jury judges, fixed on the case so it cannot be swapped later.


<pre><code><b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_proposal_content_digest">proposal_content_digest</a>(proposal: &<a href="../social_contracts/governance.md#social_contracts_governance_Proposal">social_contracts::governance::Proposal</a>): vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_proposal_content_digest">proposal_content_digest</a>(proposal: &<a href="../social_contracts/governance.md#social_contracts_governance_Proposal">Proposal</a>): vector&lt;u8&gt; {
    <b>let</b> <b>mut</b> bytes = vector::empty&lt;u8&gt;();
    vector::append(&<b>mut</b> bytes, bcs::to_bytes(&proposal.title));
    vector::append(&<b>mut</b> bytes, bcs::to_bytes(&proposal.description));
    vector::append(&<b>mut</b> bytes, bcs::to_bytes(&proposal.metadata_json));
    vector::append(&<b>mut</b> bytes, bcs::to_bytes(&proposal.reference_id));
    hash::sha3_256(bytes)
}
</code></pre>



</details>

<a name="social_contracts_governance_stakeholder_owners"></a>

## Function `stakeholder_owners`

Everyone who has a stake in this proposal's outcome, up to the jury's exclusion limit:
the current delegates, the submitter, any extra addresses (a platform's owner) and, at the
community step, the voters. The limit is a cap on a case, so on a very large vote only the
first voters are excluded.


<pre><code><b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_stakeholder_owners">stakeholder_owners</a>(registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, proposal: &<a href="../social_contracts/governance.md#social_contracts_governance_Proposal">social_contracts::governance::Proposal</a>, extra: vector&lt;<b>address</b>&gt;, include_voters: bool, limit: u64): vector&lt;<b>address</b>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_stakeholder_owners">stakeholder_owners</a>(
    registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    proposal: &<a href="../social_contracts/governance.md#social_contracts_governance_Proposal">Proposal</a>,
    extra: vector&lt;<b>address</b>&gt;,
    include_voters: bool,
    limit: u64,
): vector&lt;<b>address</b>&gt; {
    <b>let</b> <b>mut</b> out = vec_set::empty&lt;<b>address</b>&gt;();
    <b>let</b> delegates = vec_set::keys(&registry.delegate_addresses);
    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (i &lt; vector::length(delegates)) {
        <a href="../social_contracts/governance.md#social_contracts_governance_add_excluded_owner">add_excluded_owner</a>(&<b>mut</b> out, *vector::borrow(delegates, i), limit);
        i = i + 1;
    };
    <a href="../social_contracts/governance.md#social_contracts_governance_add_excluded_owner">add_excluded_owner</a>(&<b>mut</b> out, proposal.submitter, limit);
    <b>let</b> <b>mut</b> j = 0;
    <b>while</b> (j &lt; vector::length(&extra)) {
        <a href="../social_contracts/governance.md#social_contracts_governance_add_excluded_owner">add_excluded_owner</a>(&<b>mut</b> out, *vector::borrow(&extra, j), limit);
        j = j + 1;
    };
    <b>if</b> (include_voters) {
        <b>if</b> (dynamic_field::exists_(&proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_VOTED_COMMUNITY_FIELD">VOTED_COMMUNITY_FIELD</a>)) {
            <b>let</b> voters: &VecSet&lt;<b>address</b>&gt; = dynamic_field::borrow(&proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_VOTED_COMMUNITY_FIELD">VOTED_COMMUNITY_FIELD</a>);
            <b>let</b> keys = vec_set::keys(voters);
            <b>let</b> <b>mut</b> k = 0;
            <b>while</b> (k &lt; vector::length(keys)) {
                <a href="../social_contracts/governance.md#social_contracts_governance_add_excluded_owner">add_excluded_owner</a>(&<b>mut</b> out, *vector::borrow(keys, k), limit);
                k = k + 1;
            };
        };
        <b>if</b> (dynamic_field::exists_(&proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_ANON_VOTERS_FIELD">ANON_VOTERS_FIELD</a>)) {
            <b>let</b> voters: &VecSet&lt;<b>address</b>&gt; = dynamic_field::borrow(&proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_ANON_VOTERS_FIELD">ANON_VOTERS_FIELD</a>);
            <b>let</b> keys = vec_set::keys(voters);
            <b>let</b> <b>mut</b> k = 0;
            <b>while</b> (k &lt; vector::length(keys)) {
                <a href="../social_contracts/governance.md#social_contracts_governance_add_excluded_owner">add_excluded_owner</a>(&<b>mut</b> out, *vector::borrow(keys, k), limit);
                k = k + 1;
            };
        };
    };
    vec_set::into_keys(out)
}
</code></pre>



</details>

<a name="social_contracts_governance_add_excluded_owner"></a>

## Function `add_excluded_owner`



<pre><code><b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_add_excluded_owner">add_excluded_owner</a>(out: &<b>mut</b> <a href="../myso/vec_set.md#myso_vec_set_VecSet">myso::vec_set::VecSet</a>&lt;<b>address</b>&gt;, addr: <b>address</b>, limit: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_add_excluded_owner">add_excluded_owner</a>(out: &<b>mut</b> VecSet&lt;<b>address</b>&gt;, addr: <b>address</b>, limit: u64) {
    <b>if</b> (vec_set::length(out) &lt; limit && !vec_set::contains(out, &addr)) {
        vec_set::insert(out, addr);
    };
}
</code></pre>



</details>

<a name="social_contracts_governance_take_jury_fee"></a>

## Function `take_jury_fee`

Carve the jury fee out of the proposal's pool: <code>jury_fee_bps</code> of what is left, capped.


<pre><code><b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_take_jury_fee">take_jury_fee</a>(proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">social_contracts::governance::Proposal</a>, policy: &<a href="../social_contracts/governance.md#social_contracts_governance_RegistryJuryPolicy">social_contracts::governance::RegistryJuryPolicy</a>): <a href="../myso/balance.md#myso_balance_Balance">myso::balance::Balance</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_take_jury_fee">take_jury_fee</a>(proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">Proposal</a>, policy: &<a href="../social_contracts/governance.md#social_contracts_governance_RegistryJuryPolicy">RegistryJuryPolicy</a>): Balance&lt;MYSO&gt; {
    <b>let</b> pool = balance::value(&proposal.reward_pool);
    <b>assert</b>!(pool &gt;= policy.min_pool_for_jury_mist, <a href="../social_contracts/governance.md#social_contracts_governance_EPoolTooSmallForJury">EPoolTooSmallForJury</a>);
    <b>let</b> <b>mut</b> fee = ((((pool <b>as</b> u128) * (policy.jury_fee_bps <b>as</b> u128)) / 10000u128) <b>as</b> u64);
    <b>if</b> (policy.jury_fee_cap_mist &gt; 0 && fee &gt; policy.jury_fee_cap_mist) {
        fee = policy.jury_fee_cap_mist;
    };
    <b>assert</b>!(fee &gt; 0, <a href="../social_contracts/governance.md#social_contracts_governance_EPoolTooSmallForJury">EPoolTooSmallForJury</a>);
    balance::split(&<b>mut</b> proposal.reward_pool, fee)
}
</code></pre>



</details>

<a name="social_contracts_governance_open_jury_for_proposal"></a>

## Function `open_jury_for_proposal`

Anyone moves a proposal to the jury at the step that has earned it: the council step once a
referral majority, a tie, or the council window allows it, or the community step after a vote
that ended with no quorum or a tie.

No fee is taken unless a jury can actually form for this case (enough independent owners
outside the case's stakeholders for every round). When one cannot, the step waits, and once
it has waited <code><a href="../social_contracts/governance.md#social_contracts_governance_JURY_DEFER_MAX_MS">JURY_DEFER_MAX_MS</a></code> the next call resolves it without a jury: the council step
goes to the community vote, the community step refunds everyone. A juror shortage never
forfeits the pool.

Returns <code><a href="../social_contracts/governance.md#social_contracts_governance_ESCALATE_OPENED">ESCALATE_OPENED</a></code>, <code><a href="../social_contracts/governance.md#social_contracts_governance_ESCALATE_DEFERRED">ESCALATE_DEFERRED</a></code>, <code><a href="../social_contracts/governance.md#social_contracts_governance_ESCALATE_ADVANCED">ESCALATE_ADVANCED</a></code>, <code><a href="../social_contracts/governance.md#social_contracts_governance_ESCALATE_REFUNDED">ESCALATE_REFUNDED</a></code>, or
<code><a href="../social_contracts/governance.md#social_contracts_governance_ESCALATE_JURY_PAUSED">ESCALATE_JURY_PAUSED</a></code> when the jury is switched off and the community default (reject)
applied; only then does the caller forfeit the pool, for <code><a href="../social_contracts/governance.md#social_contracts_governance_jury_default_forfeit_reason">jury_default_forfeit_reason</a></code>.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_open_jury_for_proposal">open_jury_for_proposal</a>(registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">social_contracts::governance::Proposal</a>, jury_config: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">social_contracts::jury::JuryConfig</a>, roster: &<a href="../social_contracts/jury.md#social_contracts_jury_JurorRoster">social_contracts::jury::JurorRoster</a>, stage: u8, sweep_to: <b>address</b>, extra_exclusions: vector&lt;<b>address</b>&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_open_jury_for_proposal">open_jury_for_proposal</a>(
    registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">Proposal</a>,
    jury_config: &JuryConfig,
    roster: &JurorRoster,
    stage: u8,
    sweep_to: <b>address</b>,
    extra_exclusions: vector&lt;<b>address</b>&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
): u8 {
    <b>assert</b>!(registry.<a href="../social_contracts/governance.md#social_contracts_governance_version">version</a> == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/governance.md#social_contracts_governance_EWrongVersion">EWrongVersion</a>);
    <b>let</b> now = clock::timestamp_ms(clock);
    <b>let</b> proposal_id = object::id(proposal);
    <b>assert</b>!(table::contains(&registry.proposals, proposal_id), <a href="../social_contracts/governance.md#social_contracts_governance_EProposalNotFound">EProposalNotFound</a>);
    <b>let</b> policy = <a href="../social_contracts/governance.md#social_contracts_governance_jury_policy_of">jury_policy_of</a>(registry);
    <b>assert</b>!(option::is_some(&policy), <a href="../social_contracts/governance.md#social_contracts_governance_EJuryStageDisabled">EJuryStageDisabled</a>);
    <b>let</b> policy = option::destroy_some(policy);
    <b>let</b> trigger = <b>if</b> (stage == <a href="../social_contracts/governance.md#social_contracts_governance_JURY_STAGE_COUNCIL">JURY_STAGE_COUNCIL</a>) {
        <b>assert</b>!(proposal.status == <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_DELEGATE_REVIEW">STATUS_DELEGATE_REVIEW</a>, <a href="../social_contracts/governance.md#social_contracts_governance_EInvalidProposalStatus">EInvalidProposalStatus</a>);
        <b>assert</b>!(policy.council_stage_enabled, <a href="../social_contracts/governance.md#social_contracts_governance_EJuryStageDisabled">EJuryStageDisabled</a>);
        <b>assert</b>!(!<a href="../social_contracts/governance.md#social_contracts_governance_council_jury_has_jurisdiction">council_jury_has_jurisdiction</a>(proposal), <a href="../social_contracts/governance.md#social_contracts_governance_EJuryHasJurisdiction">EJuryHasJurisdiction</a>);
        <a href="../social_contracts/governance.md#social_contracts_governance_council_trigger">council_trigger</a>(registry, proposal, &policy, now)
    } <b>else</b> {
        <b>assert</b>!(stage == <a href="../social_contracts/governance.md#social_contracts_governance_JURY_STAGE_COMMUNITY">JURY_STAGE_COMMUNITY</a>, <a href="../social_contracts/governance.md#social_contracts_governance_EInvalidParameter">EInvalidParameter</a>);
        <b>assert</b>!(proposal.status == <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_COMMUNITY_VOTING">STATUS_COMMUNITY_VOTING</a>, <a href="../social_contracts/governance.md#social_contracts_governance_EInvalidProposalStatus">EInvalidProposalStatus</a>);
        <b>assert</b>!(<a href="../social_contracts/governance.md#social_contracts_governance_jury_state">jury_state</a>(proposal, <a href="../social_contracts/governance.md#social_contracts_governance_JURY_COMMUNITY_FIELD">JURY_COMMUNITY_FIELD</a>) == <a href="../social_contracts/governance.md#social_contracts_governance_JURY_STATE_AWAITING">JURY_STATE_AWAITING</a>, <a href="../social_contracts/governance.md#social_contracts_governance_ENoJuryRecord">ENoJuryRecord</a>);
        <b>let</b> review: &<a href="../social_contracts/governance.md#social_contracts_governance_JuryReview">JuryReview</a> = dynamic_field::borrow(&proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_JURY_COMMUNITY_FIELD">JURY_COMMUNITY_FIELD</a>);
        review.trigger
    };
    <b>if</b> (!<a href="../social_contracts/jury.md#social_contracts_jury_config_active">jury::config_active</a>(jury_config)) {
        // A paused <a href="../social_contracts/jury.md#social_contracts_jury">jury</a> must not strand a finished vote. The council step simply stays with the
        // humans; the community step takes its default. Pausing is a <a href="../social_contracts/governance.md#social_contracts_governance">governance</a> decision, not a
        // shortage, so this keeps the default reject.
        <b>assert</b>!(stage == <a href="../social_contracts/governance.md#social_contracts_governance_JURY_STAGE_COMMUNITY">JURY_STAGE_COMMUNITY</a>, <a href="../social_contracts/governance.md#social_contracts_governance_EJuryStageDisabled">EJuryStageDisabled</a>);
        <a href="../social_contracts/governance.md#social_contracts_governance_apply_default_community_reject">apply_default_community_reject</a>(registry, proposal, now);
        <b>return</b> <a href="../social_contracts/governance.md#social_contracts_governance_ESCALATE_JURY_PAUSED">ESCALATE_JURY_PAUSED</a>
    };
    <b>let</b> field = <b>if</b> (stage == <a href="../social_contracts/governance.md#social_contracts_governance_JURY_STAGE_COUNCIL">JURY_STAGE_COUNCIL</a>) { <a href="../social_contracts/governance.md#social_contracts_governance_JURY_COUNCIL_FIELD">JURY_COUNCIL_FIELD</a> } <b>else</b> { <a href="../social_contracts/governance.md#social_contracts_governance_JURY_COMMUNITY_FIELD">JURY_COMMUNITY_FIELD</a> };
    // A council step can reach the <a href="../social_contracts/jury.md#social_contracts_jury">jury</a> by tie or deadline with no referral ever recorded.
    <a href="../social_contracts/governance.md#social_contracts_governance_ensure_jury_review">ensure_jury_review</a>(proposal, field, stage);
    <b>let</b> owners = <a href="../social_contracts/governance.md#social_contracts_governance_stakeholder_owners">stakeholder_owners</a>(
        registry,
        proposal,
        extra_exclusions,
        stage == <a href="../social_contracts/governance.md#social_contracts_governance_JURY_STAGE_COMMUNITY">JURY_STAGE_COMMUNITY</a>,
        <a href="../social_contracts/jury.md#social_contracts_jury_max_exclusions">jury::max_exclusions</a>(jury_config),
    );
    <b>let</b> round_sizes = vector[policy.jury_size, policy.round2_size];
    <b>let</b> required = <a href="../social_contracts/jury.md#social_contracts_jury_required_seats">jury::required_seats</a>(jury_config, &round_sizes);
    <b>let</b> no_ids = vector::empty&lt;ID&gt;();
    <b>let</b> seats = <a href="../social_contracts/jury.md#social_contracts_jury_formable_seats">jury::formable_seats</a>(jury_config, roster, &owners, &no_ids, &no_ids, now, required);
    <b>if</b> (seats &lt; required) {
        <b>let</b> deferred_since = {
            <b>let</b> review: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_JuryReview">JuryReview</a> = dynamic_field::borrow_mut(&<b>mut</b> proposal.id, field);
            <b>if</b> (review.deferred_since_ms == 0) {
                review.deferred_since_ms = now;
            };
            review.deferred_since_ms
        };
        <b>if</b> (now - deferred_since &gt;= <a href="../social_contracts/governance.md#social_contracts_governance_JURY_DEFER_MAX_MS">JURY_DEFER_MAX_MS</a>) {
            <b>if</b> (stage == <a href="../social_contracts/governance.md#social_contracts_governance_JURY_STAGE_COUNCIL">JURY_STAGE_COUNCIL</a>) {
                <a href="../social_contracts/governance.md#social_contracts_governance_resolve_council_without_jury">resolve_council_without_jury</a>(registry, proposal, clock, ctx);
                <b>return</b> <a href="../social_contracts/governance.md#social_contracts_governance_ESCALATE_ADVANCED">ESCALATE_ADVANCED</a>
            };
            <a href="../social_contracts/governance.md#social_contracts_governance_refund_community_without_jury">refund_community_without_jury</a>(registry, proposal, now, ctx);
            <b>return</b> <a href="../social_contracts/governance.md#social_contracts_governance_ESCALATE_REFUNDED">ESCALATE_REFUNDED</a>
        };
        event::emit(<a href="../social_contracts/governance.md#social_contracts_governance_JuryUnavailableEvent">JuryUnavailableEvent</a> {
            proposal_id,
            stage,
            case_id: option::none(),
            formable_seats: seats,
            required_seats: required,
            deferred_since_ms: deferred_since,
            at_ms: now,
        });
        <b>return</b> <a href="../social_contracts/governance.md#social_contracts_governance_ESCALATE_DEFERRED">ESCALATE_DEFERRED</a>
    };
    <b>let</b> fee = <a href="../social_contracts/governance.md#social_contracts_governance_take_jury_fee">take_jury_fee</a>(proposal, &policy);
    <b>let</b> fee_mist = balance::value(&fee);
    <b>let</b> approval_bps = <b>if</b> (stage == <a href="../social_contracts/governance.md#social_contracts_governance_JURY_STAGE_COUNCIL">JURY_STAGE_COUNCIL</a>) { policy.approval_bps_council } <b>else</b> { policy.approval_bps_community };
    <b>let</b> <b>mut</b> case = <a href="../social_contracts/jury.md#social_contracts_jury_open_case">jury::open_case</a>(
        jury_config,
        <a href="../social_contracts/jury.md#social_contracts_jury_subject_governance">jury::subject_governance</a>(),
        proposal_id,
        stage,
        2,
        approval_bps,
        policy.quorum,
        round_sizes,
        <b>false</b>,
        <a href="../social_contracts/governance.md#social_contracts_governance_JURY_FAILURE_PATH_DEFAULT_DECISION">JURY_FAILURE_PATH_DEFAULT_DECISION</a>,
        0,
        owners,
        no_ids,
        vector::empty&lt;ID&gt;(),
        fee,
        fee_mist / policy.jury_size,
        sweep_to,
        clock,
        ctx,
    );
    <a href="../social_contracts/jury.md#social_contracts_jury_add_evidence">jury::add_evidence</a>(
        jury_config,
        &<b>mut</b> case,
        proposal.submitter,
        <a href="../social_contracts/governance.md#social_contracts_governance_JURY_EVIDENCE_KIND_PROPOSAL">JURY_EVIDENCE_KIND_PROPOSAL</a>,
        <a href="../social_contracts/governance.md#social_contracts_governance_proposal_content_digest">proposal_content_digest</a>(proposal),
        b"<a href="../social_contracts/governance.md#social_contracts_governance">governance</a>:proposal",
        clock,
    );
    <b>let</b> case_id = <a href="../social_contracts/jury.md#social_contracts_jury_share_case">jury::share_case</a>(case);
    {
        <b>let</b> review: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_JuryReview">JuryReview</a> = dynamic_field::borrow_mut(&<b>mut</b> proposal.id, field);
        review.state = <a href="../social_contracts/governance.md#social_contracts_governance_JURY_STATE_ESCALATED">JURY_STATE_ESCALATED</a>;
        review.trigger = trigger;
        review.case_id = option::some(case_id);
        review.escalated_at_ms = now;
    };
    event::emit(<a href="../social_contracts/governance.md#social_contracts_governance_ProposalEscalatedToJuryEvent">ProposalEscalatedToJuryEvent</a> {
        proposal_id,
        stage,
        trigger,
        case_id,
        jury_fee_mist: fee_mist,
        pool_remaining_mist: balance::value(&proposal.reward_pool),
        escalated_at_ms: now,
    });
    <a href="../social_contracts/governance.md#social_contracts_governance_ESCALATE_OPENED">ESCALATE_OPENED</a>
}
</code></pre>



</details>

<a name="social_contracts_governance_ensure_jury_review"></a>

## Function `ensure_jury_review`



<pre><code><b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_ensure_jury_review">ensure_jury_review</a>(proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">social_contracts::governance::Proposal</a>, field: vector&lt;u8&gt;, stage: u8)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_ensure_jury_review">ensure_jury_review</a>(proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">Proposal</a>, field: vector&lt;u8&gt;, stage: u8) {
    <b>if</b> (!dynamic_field::exists_(&proposal.id, field)) {
        dynamic_field::add(
            &<b>mut</b> proposal.id,
            field,
            <a href="../social_contracts/governance.md#social_contracts_governance_JuryReview">JuryReview</a> {
                stage,
                state: <a href="../social_contracts/governance.md#social_contracts_governance_JURY_STATE_REFERRING">JURY_STATE_REFERRING</a>,
                trigger: 0,
                referrers: vector::empty(),
                case_id: option::none(),
                escalated_at_ms: 0,
                decided_at_ms: 0,
                decision: <a href="../social_contracts/governance.md#social_contracts_governance_NO_DECISION">NO_DECISION</a>,
                deferred_since_ms: 0,
            },
        );
    };
}
</code></pre>



</details>

<a name="social_contracts_governance_resolve_council_without_jury"></a>

## Function `resolve_council_without_jury`

The council step waited the full period for a jury: the humans decide, as when a council
jury fails. No fee was taken.


<pre><code><b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_resolve_council_without_jury">resolve_council_without_jury</a>(registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">social_contracts::governance::Proposal</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_resolve_council_without_jury">resolve_council_without_jury</a>(
    registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">Proposal</a>,
    clock: &Clock,
    ctx: &TxContext,
) {
    <b>let</b> now = clock::timestamp_ms(clock);
    <b>let</b> proposal_id = object::id(proposal);
    {
        <b>let</b> review: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_JuryReview">JuryReview</a> = dynamic_field::borrow_mut(&<b>mut</b> proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_JURY_COUNCIL_FIELD">JURY_COUNCIL_FIELD</a>);
        review.state = <a href="../social_contracts/governance.md#social_contracts_governance_JURY_STATE_DECIDED">JURY_STATE_DECIDED</a>;
        review.decision = <a href="../social_contracts/governance.md#social_contracts_governance_JURY_OUTCOME_FORWARD">JURY_OUTCOME_FORWARD</a>;
        review.decided_at_ms = now;
        review.case_id = option::none();
    };
    <a href="../social_contracts/governance.md#social_contracts_governance_move_to_community_voting">move_to_community_voting</a>(registry, proposal, clock, ctx);
    event::emit(<a href="../social_contracts/governance.md#social_contracts_governance_JuryDecisionAppliedEvent">JuryDecisionAppliedEvent</a> {
        proposal_id,
        stage: <a href="../social_contracts/governance.md#social_contracts_governance_JURY_STAGE_COUNCIL">JURY_STAGE_COUNCIL</a>,
        decision: <a href="../social_contracts/governance.md#social_contracts_governance_JURY_OUTCOME_FORWARD">JURY_OUTCOME_FORWARD</a>,
        failed: <b>true</b>,
        case_id: option::none(),
        applied_at_ms: now,
    });
}
</code></pre>



</details>

<a name="social_contracts_governance_refund_community_without_jury"></a>

## Function `refund_community_without_jury`

The community step waited the full period for a jury. Nothing is forfeited: the submitter
gets the stake back now, and each voter can claim exactly what they paid until the deadline.
The proposal ends rejected, since it never won.


<pre><code><b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_refund_community_without_jury">refund_community_without_jury</a>(registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">social_contracts::governance::Proposal</a>, now: u64, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_refund_community_without_jury">refund_community_without_jury</a>(
    registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">Proposal</a>,
    now: u64,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> proposal_id = object::id(proposal);
    <b>let</b> pool = balance::value(&proposal.reward_pool);
    <b>let</b> paid_total = <b>if</b> (dynamic_field::exists_(&proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_VOTE_PAID_FIELD">VOTE_PAID_FIELD</a>)) {
        <b>let</b> payments: &<a href="../social_contracts/governance.md#social_contracts_governance_VotePayments">VotePayments</a> = dynamic_field::borrow(&proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_VOTE_PAID_FIELD">VOTE_PAID_FIELD</a>);
        payments.total_mist
    } <b>else</b> {
        0
    };
    // Voters are repaid first; a fee an earlier council <a href="../social_contracts/jury.md#social_contracts_jury">jury</a> spent comes out of the stake.
    <b>let</b> voter_pool = <b>if</b> (pool &lt; paid_total) { pool } <b>else</b> { paid_total };
    <b>let</b> submitter_refund = pool - voter_pool;
    <b>if</b> (submitter_refund &gt; 0) {
        <b>let</b> stake = coin::from_balance(balance::split(&<b>mut</b> proposal.reward_pool, submitter_refund), ctx);
        transfer::public_transfer(stake, proposal.submitter);
    };
    <a href="../social_contracts/governance.md#social_contracts_governance_remove_from_bucket">remove_from_bucket</a>(registry, <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_COMMUNITY_VOTING">STATUS_COMMUNITY_VOTING</a>, proposal_id);
    proposal.status = <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_REJECTED">STATUS_REJECTED</a>;
    vector::push_back(table::borrow_mut(&<b>mut</b> registry.proposals_by_status, <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_REJECTED">STATUS_REJECTED</a>), proposal_id);
    {
        <b>let</b> review: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_JuryReview">JuryReview</a> = dynamic_field::borrow_mut(&<b>mut</b> proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_JURY_COMMUNITY_FIELD">JURY_COMMUNITY_FIELD</a>);
        review.state = <a href="../social_contracts/governance.md#social_contracts_governance_JURY_STATE_DECIDED">JURY_STATE_DECIDED</a>;
        review.decision = <a href="../social_contracts/governance.md#social_contracts_governance_JURY_OUTCOME_REJECT">JURY_OUTCOME_REJECT</a>;
        review.decided_at_ms = now;
        review.case_id = option::none();
    };
    <b>let</b> claim_deadline_ms = now + <a href="../social_contracts/governance.md#social_contracts_governance_VOTE_REFUND_CLAIM_WINDOW_MS">VOTE_REFUND_CLAIM_WINDOW_MS</a>;
    dynamic_field::add(&<b>mut</b> proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_VOTE_REFUND_FIELD">VOTE_REFUND_FIELD</a>, <a href="../social_contracts/governance.md#social_contracts_governance_VoteRefund">VoteRefund</a> {
        refunded_at_ms: now,
        claim_deadline_ms,
        voter_pool_mist: voter_pool,
        paid_total_mist: paid_total,
        outstanding_paid_mist: paid_total,
        claimed: table::new(ctx),
    });
    event::emit(<a href="../social_contracts/governance.md#social_contracts_governance_ProposalRefundedJuryUnavailableEvent">ProposalRefundedJuryUnavailableEvent</a> {
        proposal_id,
        submitter: proposal.submitter,
        submitter_refund_mist: submitter_refund,
        voter_refund_pool_mist: voter_pool,
        claim_deadline_ms,
        refunded_at_ms: now,
    });
}
</code></pre>



</details>

<a name="social_contracts_governance_record_vote_payment"></a>

## Function `record_vote_payment`



<pre><code><b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_record_vote_payment">record_vote_payment</a>(proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">social_contracts::governance::Proposal</a>, voter: <b>address</b>, amount: u64, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_record_vote_payment">record_vote_payment</a>(proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">Proposal</a>, voter: <b>address</b>, amount: u64, ctx: &<b>mut</b> TxContext) {
    <b>if</b> (!dynamic_field::exists_(&proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_VOTE_PAID_FIELD">VOTE_PAID_FIELD</a>)) {
        dynamic_field::add(&<b>mut</b> proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_VOTE_PAID_FIELD">VOTE_PAID_FIELD</a>, <a href="../social_contracts/governance.md#social_contracts_governance_VotePayments">VotePayments</a> { paid: table::new(ctx), total_mist: 0 });
    };
    <b>let</b> payments: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_VotePayments">VotePayments</a> = dynamic_field::borrow_mut(&<b>mut</b> proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_VOTE_PAID_FIELD">VOTE_PAID_FIELD</a>);
    <b>if</b> (table::contains(&payments.paid, voter)) {
        <b>let</b> prior = table::borrow_mut(&<b>mut</b> payments.paid, voter);
        *prior = *prior + amount;
    } <b>else</b> {
        table::add(&<b>mut</b> payments.paid, voter, amount);
    };
    payments.total_mist = payments.total_mist + amount;
}
</code></pre>



</details>

<a name="social_contracts_governance_claim_vote_refund_internal"></a>

## Function `claim_vote_refund_internal`

What the sender paid to vote on a refunded proposal, taken out of the pool.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_claim_vote_refund_internal">claim_vote_refund_internal</a>(registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">social_contracts::governance::Proposal</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../myso/balance.md#myso_balance_Balance">myso::balance::Balance</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_claim_vote_refund_internal">claim_vote_refund_internal</a>(
    registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">Proposal</a>,
    clock: &Clock,
    ctx: &TxContext,
): Balance&lt;MYSO&gt; {
    <b>assert</b>!(registry.<a href="../social_contracts/governance.md#social_contracts_governance_version">version</a> == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/governance.md#social_contracts_governance_EWrongVersion">EWrongVersion</a>);
    <b>let</b> proposal_id = object::id(proposal);
    <b>assert</b>!(table::contains(&registry.proposals, proposal_id), <a href="../social_contracts/governance.md#social_contracts_governance_EProposalNotFound">EProposalNotFound</a>);
    <b>assert</b>!(dynamic_field::exists_(&proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_VOTE_REFUND_FIELD">VOTE_REFUND_FIELD</a>), <a href="../social_contracts/governance.md#social_contracts_governance_ENotRefunded">ENotRefunded</a>);
    <b>let</b> now = clock::timestamp_ms(clock);
    <b>let</b> voter = tx_context::sender(ctx);
    <b>let</b> paid = <b>if</b> (dynamic_field::exists_(&proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_VOTE_PAID_FIELD">VOTE_PAID_FIELD</a>)) {
        <b>let</b> payments: &<a href="../social_contracts/governance.md#social_contracts_governance_VotePayments">VotePayments</a> = dynamic_field::borrow(&proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_VOTE_PAID_FIELD">VOTE_PAID_FIELD</a>);
        <b>if</b> (table::contains(&payments.paid, voter)) { *table::borrow(&payments.paid, voter) } <b>else</b> { 0 }
    } <b>else</b> {
        0
    };
    <b>let</b> amount = {
        <b>let</b> refund: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_VoteRefund">VoteRefund</a> = dynamic_field::borrow_mut(&<b>mut</b> proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_VOTE_REFUND_FIELD">VOTE_REFUND_FIELD</a>);
        <b>assert</b>!(now &lt;= refund.claim_deadline_ms, <a href="../social_contracts/governance.md#social_contracts_governance_EClaimWindowClosed">EClaimWindowClosed</a>);
        <b>assert</b>!(!table::contains(&refund.claimed, voter), <a href="../social_contracts/governance.md#social_contracts_governance_EAlreadyClaimed">EAlreadyClaimed</a>);
        <b>assert</b>!(paid &gt; 0, <a href="../social_contracts/governance.md#social_contracts_governance_ENothingToClaim">ENothingToClaim</a>);
        <b>let</b> amount = (((paid <b>as</b> u128) * (refund.voter_pool_mist <b>as</b> u128) / (refund.paid_total_mist <b>as</b> u128)) <b>as</b> u64);
        refund.outstanding_paid_mist = refund.outstanding_paid_mist - paid;
        table::add(&<b>mut</b> refund.claimed, voter, now);
        amount
    };
    event::emit(<a href="../social_contracts/governance.md#social_contracts_governance_VoteRefundClaimedEvent">VoteRefundClaimedEvent</a> { proposal_id, voter, amount_mist: amount, claimed_at_ms: now });
    balance::split(&<b>mut</b> proposal.reward_pool, amount)
}
</code></pre>



</details>

<a name="social_contracts_governance_claim_vote_refund"></a>

## Function `claim_vote_refund`

Global registries: claim a vote refund. Platform registries use
<code><a href="../social_contracts/platform.md#social_contracts_platform_claim_platform_vote_refund">platform::claim_platform_vote_refund</a></code>.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_claim_vote_refund">claim_vote_refund</a>(registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">social_contracts::governance::Proposal</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_claim_vote_refund">claim_vote_refund</a>(
    registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">Proposal</a>,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(registry.<a href="../social_contracts/governance.md#social_contracts_governance_registry_type">registry_type</a> != <a href="../social_contracts/governance.md#social_contracts_governance_PROPOSAL_TYPE_PLATFORM">PROPOSAL_TYPE_PLATFORM</a>, <a href="../social_contracts/governance.md#social_contracts_governance_EUsePlatformVoteEntry">EUsePlatformVoteEntry</a>);
    <b>let</b> refund = <a href="../social_contracts/governance.md#social_contracts_governance_claim_vote_refund_internal">claim_vote_refund_internal</a>(registry, proposal, clock, ctx);
    <b>if</b> (balance::value(&refund) &gt; 0) {
        transfer::public_transfer(coin::from_balance(refund, ctx), tx_context::sender(ctx));
    } <b>else</b> {
        balance::destroy_zero(refund);
    };
}
</code></pre>



</details>

<a name="social_contracts_governance_take_unclaimed_vote_refunds"></a>

## Function `take_unclaimed_vote_refunds`

The only way money leaves a refunded pool other than a claim: allowed once every voter has
claimed (only rounding dust is left) or once the claim deadline has passed.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_take_unclaimed_vote_refunds">take_unclaimed_vote_refunds</a>(registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">social_contracts::governance::Proposal</a>, recipient: <b>address</b>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>): <a href="../myso/balance.md#myso_balance_Balance">myso::balance::Balance</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_take_unclaimed_vote_refunds">take_unclaimed_vote_refunds</a>(
    registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">Proposal</a>,
    recipient: <b>address</b>,
    clock: &Clock,
): Balance&lt;MYSO&gt; {
    <b>assert</b>!(registry.<a href="../social_contracts/governance.md#social_contracts_governance_version">version</a> == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/governance.md#social_contracts_governance_EWrongVersion">EWrongVersion</a>);
    <b>let</b> proposal_id = object::id(proposal);
    <b>assert</b>!(table::contains(&registry.proposals, proposal_id), <a href="../social_contracts/governance.md#social_contracts_governance_EProposalNotFound">EProposalNotFound</a>);
    <b>assert</b>!(dynamic_field::exists_(&proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_VOTE_REFUND_FIELD">VOTE_REFUND_FIELD</a>), <a href="../social_contracts/governance.md#social_contracts_governance_ENotRefunded">ENotRefunded</a>);
    <b>let</b> now = clock::timestamp_ms(clock);
    {
        <b>let</b> refund: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_VoteRefund">VoteRefund</a> = dynamic_field::borrow_mut(&<b>mut</b> proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_VOTE_REFUND_FIELD">VOTE_REFUND_FIELD</a>);
        <b>assert</b>!(refund.outstanding_paid_mist == 0 || now &gt; refund.claim_deadline_ms, <a href="../social_contracts/governance.md#social_contracts_governance_ERefundsOutstanding">ERefundsOutstanding</a>);
        refund.outstanding_paid_mist = 0;
    };
    <b>let</b> rest = balance::withdraw_all(&<b>mut</b> proposal.reward_pool);
    event::emit(<a href="../social_contracts/governance.md#social_contracts_governance_VoteRefundsSweptEvent">VoteRefundsSweptEvent</a> {
        proposal_id,
        recipient,
        amount_mist: balance::value(&rest),
        swept_at_ms: now,
    });
    rest
}
</code></pre>



</details>

<a name="social_contracts_governance_sweep_vote_refunds"></a>

## Function `sweep_vote_refunds`

Global registries: send what voters did not claim (or rounding dust) to the ecosystem
treasury. Permissionless.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_sweep_vote_refunds">sweep_vote_refunds</a>(registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">social_contracts::governance::Proposal</a>, ecosystem_treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_sweep_vote_refunds">sweep_vote_refunds</a>(
    registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">Proposal</a>,
    ecosystem_treasury: &EcosystemTreasury,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <a href="../social_contracts/governance.md#social_contracts_governance_assert_shared_global_registry">assert_shared_global_registry</a>(registry);
    <b>let</b> dest = <a href="../social_contracts/profile.md#social_contracts_profile_get_treasury_address">profile::get_treasury_address</a>(ecosystem_treasury);
    <b>let</b> rest = <a href="../social_contracts/governance.md#social_contracts_governance_take_unclaimed_vote_refunds">take_unclaimed_vote_refunds</a>(registry, proposal, dest, clock);
    <b>if</b> (balance::value(&rest) &gt; 0) {
        transfer::public_transfer(coin::from_balance(rest, ctx), dest);
    } <b>else</b> {
        balance::destroy_zero(rest);
    };
}
</code></pre>



</details>

<a name="social_contracts_governance_jury_default_forfeit_reason"></a>

## Function `jury_default_forfeit_reason`

Why a defaulted community rejection forfeits the pool: a tie is a community rejection, a
quorum miss is what it always was.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_jury_default_forfeit_reason">jury_default_forfeit_reason</a>(proposal: &<a href="../social_contracts/governance.md#social_contracts_governance_Proposal">social_contracts::governance::Proposal</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_jury_default_forfeit_reason">jury_default_forfeit_reason</a>(proposal: &<a href="../social_contracts/governance.md#social_contracts_governance_Proposal">Proposal</a>): u8 {
    <b>let</b> review: &<a href="../social_contracts/governance.md#social_contracts_governance_JuryReview">JuryReview</a> = dynamic_field::borrow(&proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_JURY_COMMUNITY_FIELD">JURY_COMMUNITY_FIELD</a>);
    <b>if</b> (review.trigger == <a href="../social_contracts/governance.md#social_contracts_governance_JURY_TRIGGER_VOTE_TIE">JURY_TRIGGER_VOTE_TIE</a>) {
        <a href="../social_contracts/governance.md#social_contracts_governance_FORFEIT_REASON_COMMUNITY_REJECTED">FORFEIT_REASON_COMMUNITY_REJECTED</a>
    } <b>else</b> {
        <a href="../social_contracts/governance.md#social_contracts_governance_FORFEIT_REASON_QUORUM_NOT_MET">FORFEIT_REASON_QUORUM_NOT_MET</a>
    }
}
</code></pre>



</details>

<a name="social_contracts_governance_remove_from_bucket"></a>

## Function `remove_from_bucket`



<pre><code><b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_remove_from_bucket">remove_from_bucket</a>(registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, status: u8, proposal_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_remove_from_bucket">remove_from_bucket</a>(registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>, status: u8, proposal_id: ID) {
    <b>let</b> from_status = table::borrow_mut(&<b>mut</b> registry.proposals_by_status, status);
    <b>let</b> <b>mut</b> index = 0;
    <b>let</b> len = vector::length(from_status);
    <b>while</b> (index &lt; len) {
        <b>if</b> (*vector::borrow(from_status, index) == proposal_id) {
            vector::remove(from_status, index);
            <b>break</b>
        };
        index = index + 1;
    };
}
</code></pre>



</details>

<a name="social_contracts_governance_apply_default_community_reject"></a>

## Function `apply_default_community_reject`

The community step ended with the jury switched off: what the vote would have done anyway.


<pre><code><b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_apply_default_community_reject">apply_default_community_reject</a>(registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">social_contracts::governance::Proposal</a>, now: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_apply_default_community_reject">apply_default_community_reject</a>(registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>, proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">Proposal</a>, now: u64) {
    <b>let</b> proposal_id = object::id(proposal);
    <a href="../social_contracts/governance.md#social_contracts_governance_remove_from_bucket">remove_from_bucket</a>(registry, <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_COMMUNITY_VOTING">STATUS_COMMUNITY_VOTING</a>, proposal_id);
    proposal.status = <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_REJECTED">STATUS_REJECTED</a>;
    vector::push_back(table::borrow_mut(&<b>mut</b> registry.proposals_by_status, <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_REJECTED">STATUS_REJECTED</a>), proposal_id);
    {
        <b>let</b> review: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_JuryReview">JuryReview</a> = dynamic_field::borrow_mut(&<b>mut</b> proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_JURY_COMMUNITY_FIELD">JURY_COMMUNITY_FIELD</a>);
        review.state = <a href="../social_contracts/governance.md#social_contracts_governance_JURY_STATE_DECIDED">JURY_STATE_DECIDED</a>;
        review.decision = <a href="../social_contracts/governance.md#social_contracts_governance_JURY_OUTCOME_REJECT">JURY_OUTCOME_REJECT</a>;
        review.decided_at_ms = now;
    };
    event::emit(<a href="../social_contracts/governance.md#social_contracts_governance_ProposalRejectedByCommunityEvent">ProposalRejectedByCommunityEvent</a> {
        proposal_id,
        rejection_time: now,
        votes_for: proposal.community_votes_for,
        votes_against: proposal.community_votes_against,
    });
    event::emit(<a href="../social_contracts/governance.md#social_contracts_governance_JuryDecisionAppliedEvent">JuryDecisionAppliedEvent</a> {
        proposal_id,
        stage: <a href="../social_contracts/governance.md#social_contracts_governance_JURY_STAGE_COMMUNITY">JURY_STAGE_COMMUNITY</a>,
        decision: <a href="../social_contracts/governance.md#social_contracts_governance_JURY_OUTCOME_REJECT">JURY_OUTCOME_REJECT</a>,
        failed: <b>true</b>,
        case_id: option::none(),
        applied_at_ms: now,
    });
}
</code></pre>



</details>

<a name="social_contracts_governance_apply_jury_case"></a>

## Function `apply_jury_case`

Apply a finished jury case to its proposal. Returns the action taken so the caller can route
funds: <code><a href="../social_contracts/governance.md#social_contracts_governance_JURY_ACTION_ADVANCE">JURY_ACTION_ADVANCE</a></code> (the council step moved to the community vote),
<code><a href="../social_contracts/governance.md#social_contracts_governance_JURY_ACTION_APPROVE">JURY_ACTION_APPROVE</a></code>, <code><a href="../social_contracts/governance.md#social_contracts_governance_JURY_ACTION_REJECT">JURY_ACTION_REJECT</a></code> (the caller forfeits the pool), or
<code><a href="../social_contracts/governance.md#social_contracts_governance_JURY_ACTION_DEFERRED">JURY_ACTION_DEFERRED</a></code> (nothing to route).

A case that could not seat its jurors is a shortage, not a decision: its unspent fee goes back
to the pool and the step waits for a jury again, under the same 30-day bound. A jury that
could not decide (or timed out, or was voided) uses the step's default: the council step
advances (humans decide), the community step rejects (what a no-quorum or tied vote did
before). Delegate statistics are not touched: a jury decision says nothing about how
delegates sided.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_apply_jury_case">apply_jury_case</a>(registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">social_contracts::governance::Proposal</a>, case: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">social_contracts::jury::JuryCase</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): (u8, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_apply_jury_case">apply_jury_case</a>(
    registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">Proposal</a>,
    case: &<b>mut</b> JuryCase,
    clock: &Clock,
    ctx: &TxContext,
): (u8, u64) {
    <b>assert</b>!(registry.<a href="../social_contracts/governance.md#social_contracts_governance_version">version</a> == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/governance.md#social_contracts_governance_EWrongVersion">EWrongVersion</a>);
    <b>let</b> now = clock::timestamp_ms(clock);
    <b>let</b> proposal_id = object::id(proposal);
    <b>assert</b>!(table::contains(&registry.proposals, proposal_id), <a href="../social_contracts/governance.md#social_contracts_governance_EProposalNotFound">EProposalNotFound</a>);
    <b>assert</b>!(<a href="../social_contracts/jury.md#social_contracts_jury_case_subject_kind">jury::case_subject_kind</a>(case) == <a href="../social_contracts/jury.md#social_contracts_jury_subject_governance">jury::subject_governance</a>(), <a href="../social_contracts/governance.md#social_contracts_governance_EWrongJuryCase">EWrongJuryCase</a>);
    <b>assert</b>!(<a href="../social_contracts/jury.md#social_contracts_jury_case_subject_id">jury::case_subject_id</a>(case) == proposal_id, <a href="../social_contracts/governance.md#social_contracts_governance_EWrongJuryCase">EWrongJuryCase</a>);
    <b>let</b> stage = <a href="../social_contracts/jury.md#social_contracts_jury_case_stage">jury::case_stage</a>(case);
    <b>let</b> field = <b>if</b> (stage == <a href="../social_contracts/governance.md#social_contracts_governance_JURY_STAGE_COUNCIL">JURY_STAGE_COUNCIL</a>) { <a href="../social_contracts/governance.md#social_contracts_governance_JURY_COUNCIL_FIELD">JURY_COUNCIL_FIELD</a> } <b>else</b> { <a href="../social_contracts/governance.md#social_contracts_governance_JURY_COMMUNITY_FIELD">JURY_COMMUNITY_FIELD</a> };
    <b>assert</b>!(<a href="../social_contracts/governance.md#social_contracts_governance_jury_state">jury_state</a>(proposal, field) == <a href="../social_contracts/governance.md#social_contracts_governance_JURY_STATE_ESCALATED">JURY_STATE_ESCALATED</a>, <a href="../social_contracts/governance.md#social_contracts_governance_ENoJuryRecord">ENoJuryRecord</a>);
    {
        <b>let</b> review: &<a href="../social_contracts/governance.md#social_contracts_governance_JuryReview">JuryReview</a> = dynamic_field::borrow(&proposal.id, field);
        <b>assert</b>!(*option::borrow(&review.case_id) == object::id(case), <a href="../social_contracts/governance.md#social_contracts_governance_EWrongJuryCase">EWrongJuryCase</a>);
    };
    <b>assert</b>!(<a href="../social_contracts/jury.md#social_contracts_jury_case_is_finished">jury::case_is_finished</a>(case), <a href="../social_contracts/governance.md#social_contracts_governance_EJuryNotFinished">EJuryNotFinished</a>);
    <b>let</b> state = <a href="../social_contracts/jury.md#social_contracts_jury_case_state">jury::case_state</a>(case);
    <b>assert</b>!(state == <a href="../social_contracts/jury.md#social_contracts_jury_case_final">jury::case_final</a>() || state == <a href="../social_contracts/jury.md#social_contracts_jury_case_failed">jury::case_failed</a>(), <a href="../social_contracts/governance.md#social_contracts_governance_EJuryNotFinished">EJuryNotFinished</a>);
    <a href="../social_contracts/jury.md#social_contracts_jury_mark_executed">jury::mark_executed</a>(case);
    <b>let</b> failed = state == <a href="../social_contracts/jury.md#social_contracts_jury_case_failed">jury::case_failed</a>();
    <b>if</b> (failed && <a href="../social_contracts/jury.md#social_contracts_jury_case_fail_reason">jury::case_fail_reason</a>(case) == <a href="../social_contracts/jury.md#social_contracts_jury_fail_cannot_form">jury::fail_cannot_form</a>()) {
        <a href="../social_contracts/governance.md#social_contracts_governance_defer_after_unformed_case">defer_after_unformed_case</a>(proposal, case, stage, field, now);
        <b>return</b> (<a href="../social_contracts/governance.md#social_contracts_governance_JURY_ACTION_DEFERRED">JURY_ACTION_DEFERRED</a>, now)
    };
    <b>let</b> forward = <b>if</b> (failed) {
        stage == <a href="../social_contracts/governance.md#social_contracts_governance_JURY_STAGE_COUNCIL">JURY_STAGE_COUNCIL</a>
    } <b>else</b> {
        *option::borrow(&<a href="../social_contracts/jury.md#social_contracts_jury_case_final_outcome">jury::case_final_outcome</a>(case)) == <a href="../social_contracts/governance.md#social_contracts_governance_JURY_OUTCOME_FORWARD">JURY_OUTCOME_FORWARD</a>
    };
    <b>let</b> case_id = object::id(case);
    <b>let</b> action = <b>if</b> (stage == <a href="../social_contracts/governance.md#social_contracts_governance_JURY_STAGE_COUNCIL">JURY_STAGE_COUNCIL</a>) {
        <b>assert</b>!(proposal.status == <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_DELEGATE_REVIEW">STATUS_DELEGATE_REVIEW</a>, <a href="../social_contracts/governance.md#social_contracts_governance_EInvalidProposalStatus">EInvalidProposalStatus</a>);
        <b>if</b> (forward) {
            <a href="../social_contracts/governance.md#social_contracts_governance_move_to_community_voting">move_to_community_voting</a>(registry, proposal, clock, ctx);
            <a href="../social_contracts/governance.md#social_contracts_governance_JURY_ACTION_ADVANCE">JURY_ACTION_ADVANCE</a>
        } <b>else</b> {
            <a href="../social_contracts/governance.md#social_contracts_governance_transition_proposal_rejected_by_delegate_council">transition_proposal_rejected_by_delegate_council</a>(registry, proposal);
            event::emit(<a href="../social_contracts/governance.md#social_contracts_governance_ProposalRejectedEvent">ProposalRejectedEvent</a> { proposal_id, rejection_time: now });
            <a href="../social_contracts/governance.md#social_contracts_governance_JURY_ACTION_REJECT">JURY_ACTION_REJECT</a>
        }
    } <b>else</b> {
        <b>assert</b>!(proposal.status == <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_COMMUNITY_VOTING">STATUS_COMMUNITY_VOTING</a>, <a href="../social_contracts/governance.md#social_contracts_governance_EInvalidProposalStatus">EInvalidProposalStatus</a>);
        <a href="../social_contracts/governance.md#social_contracts_governance_remove_from_bucket">remove_from_bucket</a>(registry, <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_COMMUNITY_VOTING">STATUS_COMMUNITY_VOTING</a>, proposal_id);
        <b>if</b> (forward) {
            proposal.status = <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_APPROVED">STATUS_APPROVED</a>;
            vector::push_back(table::borrow_mut(&<b>mut</b> registry.proposals_by_status, <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_APPROVED">STATUS_APPROVED</a>), proposal_id);
            event::emit(<a href="../social_contracts/governance.md#social_contracts_governance_ProposalApprovedEvent">ProposalApprovedEvent</a> {
                proposal_id,
                approval_time: now,
                votes_for: proposal.community_votes_for,
                votes_against: proposal.community_votes_against,
            });
            <a href="../social_contracts/governance.md#social_contracts_governance_JURY_ACTION_APPROVE">JURY_ACTION_APPROVE</a>
        } <b>else</b> {
            proposal.status = <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_REJECTED">STATUS_REJECTED</a>;
            vector::push_back(table::borrow_mut(&<b>mut</b> registry.proposals_by_status, <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_REJECTED">STATUS_REJECTED</a>), proposal_id);
            event::emit(<a href="../social_contracts/governance.md#social_contracts_governance_ProposalRejectedByCommunityEvent">ProposalRejectedByCommunityEvent</a> {
                proposal_id,
                rejection_time: now,
                votes_for: proposal.community_votes_for,
                votes_against: proposal.community_votes_against,
            });
            <a href="../social_contracts/governance.md#social_contracts_governance_JURY_ACTION_REJECT">JURY_ACTION_REJECT</a>
        }
    };
    {
        <b>let</b> review: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_JuryReview">JuryReview</a> = dynamic_field::borrow_mut(&<b>mut</b> proposal.id, field);
        review.state = <a href="../social_contracts/governance.md#social_contracts_governance_JURY_STATE_DECIDED">JURY_STATE_DECIDED</a>;
        review.decision = <b>if</b> (forward) { <a href="../social_contracts/governance.md#social_contracts_governance_JURY_OUTCOME_FORWARD">JURY_OUTCOME_FORWARD</a> } <b>else</b> { <a href="../social_contracts/governance.md#social_contracts_governance_JURY_OUTCOME_REJECT">JURY_OUTCOME_REJECT</a> };
        review.decided_at_ms = now;
    };
    <b>if</b> (action == <a href="../social_contracts/governance.md#social_contracts_governance_JURY_ACTION_REJECT">JURY_ACTION_REJECT</a>) {
        event::emit(<a href="../social_contracts/governance.md#social_contracts_governance_ProposalRejectedByJuryEvent">ProposalRejectedByJuryEvent</a> { proposal_id, stage, rejection_time: now });
    };
    event::emit(<a href="../social_contracts/governance.md#social_contracts_governance_JuryDecisionAppliedEvent">JuryDecisionAppliedEvent</a> {
        proposal_id,
        stage,
        decision: <b>if</b> (forward) { <a href="../social_contracts/governance.md#social_contracts_governance_JURY_OUTCOME_FORWARD">JURY_OUTCOME_FORWARD</a> } <b>else</b> { <a href="../social_contracts/governance.md#social_contracts_governance_JURY_OUTCOME_REJECT">JURY_OUTCOME_REJECT</a> },
        failed,
        case_id: option::some(case_id),
        applied_at_ms: now,
    });
    (action, now)
}
</code></pre>



</details>

<a name="social_contracts_governance_escalate_to_jury"></a>

## Function `escalate_to_jury`

Global registries (ecosystem, proof of creativity, SPoT): send a proposal to the jury at the
step that has earned it. Permissionless. Unspent juror shares are swept to the ecosystem
treasury. When no jury can form the step waits (see [<code><a href="../social_contracts/governance.md#social_contracts_governance_open_jury_for_proposal">open_jury_for_proposal</a></code>]). When the
jury is switched off at the community step, the proposal is rejected as a no-quorum or tied
vote always was.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_escalate_to_jury">escalate_to_jury</a>(registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">social_contracts::governance::Proposal</a>, jury_config: &<a href="../social_contracts/jury.md#social_contracts_jury_JuryConfig">social_contracts::jury::JuryConfig</a>, roster: &<a href="../social_contracts/jury.md#social_contracts_jury_JurorRoster">social_contracts::jury::JurorRoster</a>, ecosystem_treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, stage: u8, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_escalate_to_jury">escalate_to_jury</a>(
    registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">Proposal</a>,
    jury_config: &JuryConfig,
    roster: &JurorRoster,
    ecosystem_treasury: &EcosystemTreasury,
    stage: u8,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <a href="../social_contracts/governance.md#social_contracts_governance_assert_shared_global_registry">assert_shared_global_registry</a>(registry);
    <b>let</b> dest = <a href="../social_contracts/profile.md#social_contracts_profile_get_treasury_address">profile::get_treasury_address</a>(ecosystem_treasury);
    <b>let</b> result = <a href="../social_contracts/governance.md#social_contracts_governance_open_jury_for_proposal">open_jury_for_proposal</a>(
        registry,
        proposal,
        jury_config,
        roster,
        stage,
        dest,
        vector::empty&lt;<b>address</b>&gt;(),
        clock,
        ctx,
    );
    <b>if</b> (result == <a href="../social_contracts/governance.md#social_contracts_governance_ESCALATE_JURY_PAUSED">ESCALATE_JURY_PAUSED</a>) {
        <b>let</b> reason = <a href="../social_contracts/governance.md#social_contracts_governance_jury_default_forfeit_reason">jury_default_forfeit_reason</a>(proposal);
        <a href="../social_contracts/governance.md#social_contracts_governance_forfeit_proposal_pool_to_treasury_address">forfeit_proposal_pool_to_treasury_address</a>(
            registry,
            proposal,
            dest,
            reason,
            <a href="../social_contracts/governance.md#social_contracts_governance_TREASURY_ROUTE_ECOSYSTEM">TREASURY_ROUTE_ECOSYSTEM</a>,
            clock::timestamp_ms(clock),
            ctx,
        );
    };
}
</code></pre>



</details>

<a name="social_contracts_governance_apply_jury_decision"></a>

## Function `apply_jury_decision`

Global registries: apply a finished jury case to its proposal. Permissionless.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_apply_jury_decision">apply_jury_decision</a>(registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">social_contracts::governance::Proposal</a>, case: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">social_contracts::jury::JuryCase</a>, ecosystem_treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_apply_jury_decision">apply_jury_decision</a>(
    registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">Proposal</a>,
    case: &<b>mut</b> JuryCase,
    ecosystem_treasury: &EcosystemTreasury,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <a href="../social_contracts/governance.md#social_contracts_governance_assert_shared_global_registry">assert_shared_global_registry</a>(registry);
    <b>let</b> (action, now) = <a href="../social_contracts/governance.md#social_contracts_governance_apply_jury_case">apply_jury_case</a>(registry, proposal, case, clock, ctx);
    <b>if</b> (action == <a href="../social_contracts/governance.md#social_contracts_governance_JURY_ACTION_REJECT">JURY_ACTION_REJECT</a>) {
        <b>let</b> dest = <a href="../social_contracts/profile.md#social_contracts_profile_get_treasury_address">profile::get_treasury_address</a>(ecosystem_treasury);
        <a href="../social_contracts/governance.md#social_contracts_governance_forfeit_proposal_pool_to_treasury_address">forfeit_proposal_pool_to_treasury_address</a>(
            registry,
            proposal,
            dest,
            <a href="../social_contracts/governance.md#social_contracts_governance_FORFEIT_REASON_JURY_REJECTED">FORFEIT_REASON_JURY_REJECTED</a>,
            <a href="../social_contracts/governance.md#social_contracts_governance_TREASURY_ROUTE_ECOSYSTEM">TREASURY_ROUTE_ECOSYSTEM</a>,
            now,
            ctx,
        );
    };
}
</code></pre>



</details>

<a name="social_contracts_governance_defer_after_unformed_case"></a>

## Function `defer_after_unformed_case`



<pre><code><b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_defer_after_unformed_case">defer_after_unformed_case</a>(proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">social_contracts::governance::Proposal</a>, case: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">social_contracts::jury::JuryCase</a>, stage: u8, field: vector&lt;u8&gt;, now: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_defer_after_unformed_case">defer_after_unformed_case</a>(proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">Proposal</a>, case: &<b>mut</b> JuryCase, stage: u8, field: vector&lt;u8&gt;, now: u64) {
    <b>if</b> (stage == <a href="../social_contracts/governance.md#social_contracts_governance_JURY_STAGE_COUNCIL">JURY_STAGE_COUNCIL</a>) {
        <b>assert</b>!(proposal.status == <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_DELEGATE_REVIEW">STATUS_DELEGATE_REVIEW</a>, <a href="../social_contracts/governance.md#social_contracts_governance_EInvalidProposalStatus">EInvalidProposalStatus</a>);
    } <b>else</b> {
        <b>assert</b>!(proposal.status == <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_COMMUNITY_VOTING">STATUS_COMMUNITY_VOTING</a>, <a href="../social_contracts/governance.md#social_contracts_governance_EInvalidProposalStatus">EInvalidProposalStatus</a>);
    };
    balance::join(&<b>mut</b> proposal.reward_pool, <a href="../social_contracts/jury.md#social_contracts_jury_sweep_remainder">jury::sweep_remainder</a>(case));
    <b>let</b> proposal_id = object::id(proposal);
    <b>let</b> deferred_since = {
        <b>let</b> review: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_JuryReview">JuryReview</a> = dynamic_field::borrow_mut(&<b>mut</b> proposal.id, field);
        // The council step goes back to the delegates; the community step waits <b>for</b> a <a href="../social_contracts/jury.md#social_contracts_jury">jury</a>.
        review.state = <b>if</b> (stage == <a href="../social_contracts/governance.md#social_contracts_governance_JURY_STAGE_COUNCIL">JURY_STAGE_COUNCIL</a>) { <a href="../social_contracts/governance.md#social_contracts_governance_JURY_STATE_REFERRING">JURY_STATE_REFERRING</a> } <b>else</b> { <a href="../social_contracts/governance.md#social_contracts_governance_JURY_STATE_AWAITING">JURY_STATE_AWAITING</a> };
        <b>if</b> (review.deferred_since_ms == 0) {
            review.deferred_since_ms = now;
        };
        review.deferred_since_ms
    };
    event::emit(<a href="../social_contracts/governance.md#social_contracts_governance_JuryUnavailableEvent">JuryUnavailableEvent</a> {
        proposal_id,
        stage,
        case_id: option::some(object::id(case)),
        formable_seats: 0,
        required_seats: 0,
        deferred_since_ms: deferred_since,
        at_ms: now,
    });
}
</code></pre>



</details>

<a name="social_contracts_governance_sweep_jury_case"></a>

## Function `sweep_jury_case`

Pay what the jurors did not claim back to the address the case named, once the case has ended
and its fees are settled. Permissionless. A governance case must be applied first, so a case
that could not form returns its fee to the proposal instead of the treasury.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_sweep_jury_case">sweep_jury_case</a>(case: &<b>mut</b> <a href="../social_contracts/jury.md#social_contracts_jury_JuryCase">social_contracts::jury::JuryCase</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_sweep_jury_case">sweep_jury_case</a>(case: &<b>mut</b> JuryCase, ctx: &<b>mut</b> TxContext) {
    <b>if</b> (<a href="../social_contracts/jury.md#social_contracts_jury_case_subject_kind">jury::case_subject_kind</a>(case) == <a href="../social_contracts/jury.md#social_contracts_jury_subject_governance">jury::subject_governance</a>()) {
        <b>assert</b>!(<a href="../social_contracts/jury.md#social_contracts_jury_case_executed">jury::case_executed</a>(case), <a href="../social_contracts/governance.md#social_contracts_governance_EJuryNotFinished">EJuryNotFinished</a>);
    };
    <b>let</b> remainder = <a href="../social_contracts/jury.md#social_contracts_jury_sweep_remainder">jury::sweep_remainder</a>(case);
    <b>if</b> (balance::value(&remainder) &gt; 0) {
        transfer::public_transfer(coin::from_balance(remainder, ctx), <a href="../social_contracts/jury.md#social_contracts_jury_case_sweep_to">jury::case_sweep_to</a>(case));
    } <b>else</b> {
        balance::destroy_zero(remainder);
    };
}
</code></pre>



</details>

<a name="social_contracts_governance_jury_policy_enabled"></a>

## Function `jury_policy_enabled`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_jury_policy_enabled">jury_policy_enabled</a>(registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_jury_policy_enabled">jury_policy_enabled</a>(registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>): bool {
    <b>let</b> policy = <a href="../social_contracts/governance.md#social_contracts_governance_jury_policy_of">jury_policy_of</a>(registry);
    option::is_some(&policy) && {
        <b>let</b> p = option::borrow(&policy);
        p.council_stage_enabled || p.community_stage_enabled
    }
}
</code></pre>



</details>

<a name="social_contracts_governance_jury_council_state"></a>

## Function `jury_council_state`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_jury_council_state">jury_council_state</a>(proposal: &<a href="../social_contracts/governance.md#social_contracts_governance_Proposal">social_contracts::governance::Proposal</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_jury_council_state">jury_council_state</a>(proposal: &<a href="../social_contracts/governance.md#social_contracts_governance_Proposal">Proposal</a>): u8 { <a href="../social_contracts/governance.md#social_contracts_governance_jury_state">jury_state</a>(proposal, <a href="../social_contracts/governance.md#social_contracts_governance_JURY_COUNCIL_FIELD">JURY_COUNCIL_FIELD</a>) }
</code></pre>



</details>

<a name="social_contracts_governance_jury_community_state"></a>

## Function `jury_community_state`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_jury_community_state">jury_community_state</a>(proposal: &<a href="../social_contracts/governance.md#social_contracts_governance_Proposal">social_contracts::governance::Proposal</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_jury_community_state">jury_community_state</a>(proposal: &<a href="../social_contracts/governance.md#social_contracts_governance_Proposal">Proposal</a>): u8 { <a href="../social_contracts/governance.md#social_contracts_governance_jury_state">jury_state</a>(proposal, <a href="../social_contracts/governance.md#social_contracts_governance_JURY_COMMUNITY_FIELD">JURY_COMMUNITY_FIELD</a>) }
</code></pre>



</details>

<a name="social_contracts_governance_jury_stage_council_value"></a>

## Function `jury_stage_council_value`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_jury_stage_council_value">jury_stage_council_value</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_jury_stage_council_value">jury_stage_council_value</a>(): u8 { <a href="../social_contracts/governance.md#social_contracts_governance_JURY_STAGE_COUNCIL">JURY_STAGE_COUNCIL</a> }
</code></pre>



</details>

<a name="social_contracts_governance_jury_stage_community_value"></a>

## Function `jury_stage_community_value`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_jury_stage_community_value">jury_stage_community_value</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_jury_stage_community_value">jury_stage_community_value</a>(): u8 { <a href="../social_contracts/governance.md#social_contracts_governance_JURY_STAGE_COMMUNITY">JURY_STAGE_COMMUNITY</a> }
</code></pre>



</details>

<a name="social_contracts_governance_jury_action_advance_value"></a>

## Function `jury_action_advance_value`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_jury_action_advance_value">jury_action_advance_value</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_jury_action_advance_value">jury_action_advance_value</a>(): u8 { <a href="../social_contracts/governance.md#social_contracts_governance_JURY_ACTION_ADVANCE">JURY_ACTION_ADVANCE</a> }
</code></pre>



</details>

<a name="social_contracts_governance_jury_action_approve_value"></a>

## Function `jury_action_approve_value`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_jury_action_approve_value">jury_action_approve_value</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_jury_action_approve_value">jury_action_approve_value</a>(): u8 { <a href="../social_contracts/governance.md#social_contracts_governance_JURY_ACTION_APPROVE">JURY_ACTION_APPROVE</a> }
</code></pre>



</details>

<a name="social_contracts_governance_jury_action_reject_value"></a>

## Function `jury_action_reject_value`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_jury_action_reject_value">jury_action_reject_value</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_jury_action_reject_value">jury_action_reject_value</a>(): u8 { <a href="../social_contracts/governance.md#social_contracts_governance_JURY_ACTION_REJECT">JURY_ACTION_REJECT</a> }
</code></pre>



</details>

<a name="social_contracts_governance_jury_action_deferred_value"></a>

## Function `jury_action_deferred_value`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_jury_action_deferred_value">jury_action_deferred_value</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_jury_action_deferred_value">jury_action_deferred_value</a>(): u8 { <a href="../social_contracts/governance.md#social_contracts_governance_JURY_ACTION_DEFERRED">JURY_ACTION_DEFERRED</a> }
</code></pre>



</details>

<a name="social_contracts_governance_escalate_opened_value"></a>

## Function `escalate_opened_value`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_escalate_opened_value">escalate_opened_value</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_escalate_opened_value">escalate_opened_value</a>(): u8 { <a href="../social_contracts/governance.md#social_contracts_governance_ESCALATE_OPENED">ESCALATE_OPENED</a> }
</code></pre>



</details>

<a name="social_contracts_governance_escalate_jury_paused_value"></a>

## Function `escalate_jury_paused_value`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_escalate_jury_paused_value">escalate_jury_paused_value</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_escalate_jury_paused_value">escalate_jury_paused_value</a>(): u8 { <a href="../social_contracts/governance.md#social_contracts_governance_ESCALATE_JURY_PAUSED">ESCALATE_JURY_PAUSED</a> }
</code></pre>



</details>

<a name="social_contracts_governance_escalate_deferred_value"></a>

## Function `escalate_deferred_value`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_escalate_deferred_value">escalate_deferred_value</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_escalate_deferred_value">escalate_deferred_value</a>(): u8 { <a href="../social_contracts/governance.md#social_contracts_governance_ESCALATE_DEFERRED">ESCALATE_DEFERRED</a> }
</code></pre>



</details>

<a name="social_contracts_governance_escalate_advanced_value"></a>

## Function `escalate_advanced_value`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_escalate_advanced_value">escalate_advanced_value</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_escalate_advanced_value">escalate_advanced_value</a>(): u8 { <a href="../social_contracts/governance.md#social_contracts_governance_ESCALATE_ADVANCED">ESCALATE_ADVANCED</a> }
</code></pre>



</details>

<a name="social_contracts_governance_escalate_refunded_value"></a>

## Function `escalate_refunded_value`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_escalate_refunded_value">escalate_refunded_value</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_escalate_refunded_value">escalate_refunded_value</a>(): u8 { <a href="../social_contracts/governance.md#social_contracts_governance_ESCALATE_REFUNDED">ESCALATE_REFUNDED</a> }
</code></pre>



</details>

<a name="social_contracts_governance_jury_defer_max_ms"></a>

## Function `jury_defer_max_ms`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_jury_defer_max_ms">jury_defer_max_ms</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_jury_defer_max_ms">jury_defer_max_ms</a>(): u64 { <a href="../social_contracts/governance.md#social_contracts_governance_JURY_DEFER_MAX_MS">JURY_DEFER_MAX_MS</a> }
</code></pre>



</details>

<a name="social_contracts_governance_vote_refund_claim_window_ms"></a>

## Function `vote_refund_claim_window_ms`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_vote_refund_claim_window_ms">vote_refund_claim_window_ms</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_vote_refund_claim_window_ms">vote_refund_claim_window_ms</a>(): u64 { <a href="../social_contracts/governance.md#social_contracts_governance_VOTE_REFUND_CLAIM_WINDOW_MS">VOTE_REFUND_CLAIM_WINDOW_MS</a> }
</code></pre>



</details>

<a name="social_contracts_governance_jury_deferred_since_ms"></a>

## Function `jury_deferred_since_ms`

When the step first found no jury able to form; 0 if never.


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_jury_deferred_since_ms">jury_deferred_since_ms</a>(proposal: &<a href="../social_contracts/governance.md#social_contracts_governance_Proposal">social_contracts::governance::Proposal</a>, stage: u8): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_jury_deferred_since_ms">jury_deferred_since_ms</a>(proposal: &<a href="../social_contracts/governance.md#social_contracts_governance_Proposal">Proposal</a>, stage: u8): u64 {
    <b>let</b> field = <b>if</b> (stage == <a href="../social_contracts/governance.md#social_contracts_governance_JURY_STAGE_COUNCIL">JURY_STAGE_COUNCIL</a>) { <a href="../social_contracts/governance.md#social_contracts_governance_JURY_COUNCIL_FIELD">JURY_COUNCIL_FIELD</a> } <b>else</b> { <a href="../social_contracts/governance.md#social_contracts_governance_JURY_COMMUNITY_FIELD">JURY_COMMUNITY_FIELD</a> };
    <b>if</b> (!dynamic_field::exists_(&proposal.id, field)) {
        <b>return</b> 0
    };
    <b>let</b> review: &<a href="../social_contracts/governance.md#social_contracts_governance_JuryReview">JuryReview</a> = dynamic_field::borrow(&proposal.id, field);
    review.deferred_since_ms
}
</code></pre>



</details>

<a name="social_contracts_governance_vote_refunded"></a>

## Function `vote_refunded`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_vote_refunded">vote_refunded</a>(proposal: &<a href="../social_contracts/governance.md#social_contracts_governance_Proposal">social_contracts::governance::Proposal</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_vote_refunded">vote_refunded</a>(proposal: &<a href="../social_contracts/governance.md#social_contracts_governance_Proposal">Proposal</a>): bool { dynamic_field::exists_(&proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_VOTE_REFUND_FIELD">VOTE_REFUND_FIELD</a>) }
</code></pre>



</details>

<a name="social_contracts_governance_vote_paid_mist"></a>

## Function `vote_paid_mist`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_vote_paid_mist">vote_paid_mist</a>(proposal: &<a href="../social_contracts/governance.md#social_contracts_governance_Proposal">social_contracts::governance::Proposal</a>, voter: <b>address</b>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_vote_paid_mist">vote_paid_mist</a>(proposal: &<a href="../social_contracts/governance.md#social_contracts_governance_Proposal">Proposal</a>, voter: <b>address</b>): u64 {
    <b>if</b> (!dynamic_field::exists_(&proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_VOTE_PAID_FIELD">VOTE_PAID_FIELD</a>)) {
        <b>return</b> 0
    };
    <b>let</b> payments: &<a href="../social_contracts/governance.md#social_contracts_governance_VotePayments">VotePayments</a> = dynamic_field::borrow(&proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_VOTE_PAID_FIELD">VOTE_PAID_FIELD</a>);
    <b>if</b> (table::contains(&payments.paid, voter)) { *table::borrow(&payments.paid, voter) } <b>else</b> { 0 }
}
</code></pre>



</details>

<a name="social_contracts_governance_vote_refund_claim_deadline_ms"></a>

## Function `vote_refund_claim_deadline_ms`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_vote_refund_claim_deadline_ms">vote_refund_claim_deadline_ms</a>(proposal: &<a href="../social_contracts/governance.md#social_contracts_governance_Proposal">social_contracts::governance::Proposal</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_vote_refund_claim_deadline_ms">vote_refund_claim_deadline_ms</a>(proposal: &<a href="../social_contracts/governance.md#social_contracts_governance_Proposal">Proposal</a>): u64 {
    <b>let</b> refund: &<a href="../social_contracts/governance.md#social_contracts_governance_VoteRefund">VoteRefund</a> = dynamic_field::borrow(&proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_VOTE_REFUND_FIELD">VOTE_REFUND_FIELD</a>);
    refund.claim_deadline_ms
}
</code></pre>



</details>

<a name="social_contracts_governance_vote_refund_outstanding_mist"></a>

## Function `vote_refund_outstanding_mist`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_vote_refund_outstanding_mist">vote_refund_outstanding_mist</a>(proposal: &<a href="../social_contracts/governance.md#social_contracts_governance_Proposal">social_contracts::governance::Proposal</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_vote_refund_outstanding_mist">vote_refund_outstanding_mist</a>(proposal: &<a href="../social_contracts/governance.md#social_contracts_governance_Proposal">Proposal</a>): u64 {
    <b>let</b> refund: &<a href="../social_contracts/governance.md#social_contracts_governance_VoteRefund">VoteRefund</a> = dynamic_field::borrow(&proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_VOTE_REFUND_FIELD">VOTE_REFUND_FIELD</a>);
    refund.outstanding_paid_mist
}
</code></pre>



</details>

<a name="social_contracts_governance_proposal_pool_value"></a>

## Function `proposal_pool_value`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_proposal_pool_value">proposal_pool_value</a>(proposal: &<a href="../social_contracts/governance.md#social_contracts_governance_Proposal">social_contracts::governance::Proposal</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_proposal_pool_value">proposal_pool_value</a>(proposal: &<a href="../social_contracts/governance.md#social_contracts_governance_Proposal">Proposal</a>): u64 { balance::value(&proposal.reward_pool) }
</code></pre>



</details>
