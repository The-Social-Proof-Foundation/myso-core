---
title: Module `social_contracts::post`
---

Post module for the MySocial network
Handles creation and management of posts and comments
Implements features like comments, reposts, and quotes


-  [Struct `Post`](#social_contracts_post_Post)
-  [Struct `PostAttribution`](#social_contracts_post_PostAttribution)
-  [Struct `CommentAttribution`](#social_contracts_post_CommentAttribution)
-  [Struct `EmbeddedAssetBinding`](#social_contracts_post_EmbeddedAssetBinding)
-  [Struct `UsageDecisionSnapshot`](#social_contracts_post_UsageDecisionSnapshot)
-  [Struct `ContainerUsageDenial`](#social_contracts_post_ContainerUsageDenial)
-  [Struct `EmbeddedBindingRecordedEvent`](#social_contracts_post_EmbeddedBindingRecordedEvent)
-  [Struct `UsageDecisionRefreshedEvent`](#social_contracts_post_UsageDecisionRefreshedEvent)
-  [Struct `ContainerUsageDeniedEvent`](#social_contracts_post_ContainerUsageDeniedEvent)
-  [Struct `ContainerUsageDenialLiftedEvent`](#social_contracts_post_ContainerUsageDenialLiftedEvent)
-  [Struct `CandidateManifestSubmittedEvent`](#social_contracts_post_CandidateManifestSubmittedEvent)
-  [Struct `PostSpotAnalysis`](#social_contracts_post_PostSpotAnalysis)
-  [Struct `Comment`](#social_contracts_post_Comment)
-  [Struct `Repost`](#social_contracts_post_Repost)
-  [Struct `PromotionView`](#social_contracts_post_PromotionView)
-  [Struct `PromotionData`](#social_contracts_post_PromotionData)
-  [Struct `PostAdminCap`](#social_contracts_post_PostAdminCap)
-  [Struct `PostConfig`](#social_contracts_post_PostConfig)
-  [Struct `PostParametersUpdatedEvent`](#social_contracts_post_PostParametersUpdatedEvent)
-  [Struct `PostCreatedEvent`](#social_contracts_post_PostCreatedEvent)
-  [Struct `CommentCreatedEvent`](#social_contracts_post_CommentCreatedEvent)
-  [Struct `RepostEvent`](#social_contracts_post_RepostEvent)
-  [Struct `RepostRemovedEvent`](#social_contracts_post_RepostRemovedEvent)
-  [Struct `ReactionEvent`](#social_contracts_post_ReactionEvent)
-  [Struct `RemoveReactionEvent`](#social_contracts_post_RemoveReactionEvent)
-  [Struct `TipEvent`](#social_contracts_post_TipEvent)
-  [Struct `OwnershipTransferEvent`](#social_contracts_post_OwnershipTransferEvent)
-  [Struct `PostModerationEvent`](#social_contracts_post_PostModerationEvent)
-  [Struct `PostUpdatedEvent`](#social_contracts_post_PostUpdatedEvent)
-  [Struct `CommentUpdatedEvent`](#social_contracts_post_CommentUpdatedEvent)
-  [Struct `PostReportedEvent`](#social_contracts_post_PostReportedEvent)
-  [Struct `CommentReportedEvent`](#social_contracts_post_CommentReportedEvent)
-  [Struct `PostDeletedEvent`](#social_contracts_post_PostDeletedEvent)
-  [Struct `CommentDeletedEvent`](#social_contracts_post_CommentDeletedEvent)
-  [Struct `PromotedPostCreatedEvent`](#social_contracts_post_PromotedPostCreatedEvent)
-  [Struct `PromotedViewConfirmItem`](#social_contracts_post_PromotedViewConfirmItem)
-  [Struct `PromotedPostViewsBatchConfirmedEvent`](#social_contracts_post_PromotedPostViewsBatchConfirmedEvent)
-  [Struct `PromotionStatusToggledEvent`](#social_contracts_post_PromotionStatusToggledEvent)
-  [Struct `PromotionFundsWithdrawnEvent`](#social_contracts_post_PromotionFundsWithdrawnEvent)
-  [Struct `ModerationRecord`](#social_contracts_post_ModerationRecord)
-  [Struct `PostSubscriptionAccessEvent`](#social_contracts_post_PostSubscriptionAccessEvent)
-  [Enum `PostAccess`](#social_contracts_post_PostAccess)
-  [Constants](#@Constants_0)
-  [Function `has_flag`](#social_contracts_post_has_flag)
-  [Function `set_flag`](#social_contracts_post_set_flag)
-  [Function `clear_flag`](#social_contracts_post_clear_flag)
-  [Function `permissions_bitfield`](#social_contracts_post_permissions_bitfield)
-  [Function `post_access_fields`](#social_contracts_post_post_access_fields)
-  [Function `requires_profile_subscription`](#social_contracts_post_requires_profile_subscription)
-  [Function `requires_marketplace_purchase`](#social_contracts_post_requires_marketplace_purchase)
-  [Function `linked_mydata`](#social_contracts_post_linked_mydata)
-  [Function `subscription_service`](#social_contracts_post_subscription_service)
-  [Function `subscription_min_tier_level`](#social_contracts_post_subscription_min_tier_level)
-  [Function `post_access_kind`](#social_contracts_post_post_access_kind)
-  [Function `post_access_from_parts`](#social_contracts_post_post_access_from_parts)
-  [Function `allow_comments`](#social_contracts_post_allow_comments)
-  [Function `allow_reactions`](#social_contracts_post_allow_reactions)
-  [Function `allow_reposts`](#social_contracts_post_allow_reposts)
-  [Function `allow_quotes`](#social_contracts_post_allow_quotes)
-  [Function `allow_tips`](#social_contracts_post_allow_tips)
-  [Function `is_spt_enabled`](#social_contracts_post_is_spt_enabled)
-  [Function `composition_status`](#social_contracts_post_composition_status)
-  [Function `monetization_status`](#social_contracts_post_monetization_status)
-  [Function `media_asset_ids`](#social_contracts_post_media_asset_ids)
-  [Function `composition_badge`](#social_contracts_post_composition_badge)
-  [Function `composition_analysis`](#social_contracts_post_composition_analysis)
-  [Function `revenue_manifest`](#social_contracts_post_revenue_manifest)
-  [Function `composition_disputes_submitted`](#social_contracts_post_composition_disputes_submitted)
-  [Function `monetization_enabled`](#social_contracts_post_monetization_enabled)
-  [Function `poc_disputes_submitted`](#social_contracts_post_poc_disputes_submitted)
-  [Function `has_composition_monetization`](#social_contracts_post_has_composition_monetization)
-  [Function `tip_post_requires_beneficiary_vault_for_amount`](#social_contracts_post_tip_post_requires_beneficiary_vault_for_amount)
-  [Function `actor_address`](#social_contracts_post_actor_address)
-  [Function `sub_agent_id`](#social_contracts_post_sub_agent_id)
-  [Function `action_identity_class`](#social_contracts_post_action_identity_class)
-  [Function `comment_actor_address`](#social_contracts_post_comment_actor_address)
-  [Function `comment_sub_agent_id`](#social_contracts_post_comment_sub_agent_id)
-  [Function `comment_action_identity_class`](#social_contracts_post_comment_action_identity_class)
-  [Function `post_attribution`](#social_contracts_post_post_attribution)
-  [Function `attach_post_attribution`](#social_contracts_post_attach_post_attribution)
-  [Function `comment_attribution`](#social_contracts_post_comment_attribution)
-  [Function `attach_comment_attribution`](#social_contracts_post_attach_comment_attribution)
-  [Function `get_spt_id`](#social_contracts_post_get_spt_id)
-  [Function `set_spt_id`](#social_contracts_post_set_spt_id)
-  [Function `set_enable_spt`](#social_contracts_post_set_enable_spt)
-  [Function `spot_status_pending`](#social_contracts_post_spot_status_pending)
-  [Function `spot_status_completed`](#social_contracts_post_spot_status_completed)
-  [Function `spot_status_completed_no_actionable`](#social_contracts_post_spot_status_completed_no_actionable)
-  [Function `has_spot_analysis`](#social_contracts_post_has_spot_analysis)
-  [Function `spot_analysis_status`](#social_contracts_post_spot_analysis_status)
-  [Function `spot_analysis_market_ids`](#social_contracts_post_spot_analysis_market_ids)
-  [Function `spot_analysis_claim_indexes`](#social_contracts_post_spot_analysis_claim_indexes)
-  [Function `spot_analysis_claim_ids`](#social_contracts_post_spot_analysis_claim_ids)
-  [Function `spot_analysis_future_accepted_count`](#social_contracts_post_spot_analysis_future_accepted_count)
-  [Function `ensure_spot_analysis_pending`](#social_contracts_post_ensure_spot_analysis_pending)
-  [Function `spot_analysis_append_future`](#social_contracts_post_spot_analysis_append_future)
-  [Function `finalize_spot_analysis`](#social_contracts_post_finalize_spot_analysis)
-  [Function `bootstrap_init`](#social_contracts_post_bootstrap_init)
-  [Function `convert_urls_to_strings`](#social_contracts_post_convert_urls_to_strings)
-  [Function `resolve_social_actor`](#social_contracts_post_resolve_social_actor)
-  [Function `assert_tip_spend_limit`](#social_contracts_post_assert_tip_spend_limit)
-  [Function `create_post_internal`](#social_contracts_post_create_post_internal)
-  [Function `emit_post_media_asset_usages`](#social_contracts_post_emit_post_media_asset_usages)
-  [Function `share_post`](#social_contracts_post_share_post)
-  [Function `assert_post_access_mydata_binding`](#social_contracts_post_assert_post_access_mydata_binding)
-  [Function `assert_post_access_mydata_object_binding`](#social_contracts_post_assert_post_access_mydata_object_binding)
-  [Function `linked_mydata_from_access`](#social_contracts_post_linked_mydata_from_access)
-  [Function `assert_profile_subscription_access_service`](#social_contracts_post_assert_profile_subscription_access_service)
-  [Function `create_post_entry_body`](#social_contracts_post_create_post_entry_body)
-  [Function `create_post_object_for_spt`](#social_contracts_post_create_post_object_for_spt)
-  [Function `share_and_emit_spt_post`](#social_contracts_post_share_and_emit_spt_post)
-  [Function `create_post`](#social_contracts_post_create_post)
-  [Function `validate_post_mydata_binding`](#social_contracts_post_validate_post_mydata_binding)
-  [Function `create_public_post`](#social_contracts_post_create_public_post)
-  [Function `create_profile_subscription_post`](#social_contracts_post_create_profile_subscription_post)
-  [Function `create_profile_subscription_post_with_mydata`](#social_contracts_post_create_profile_subscription_post_with_mydata)
-  [Function `create_marketplace_one_time_post`](#social_contracts_post_create_marketplace_one_time_post)
-  [Function `create_comment`](#social_contracts_post_create_comment)
-  [Function `create_repost`](#social_contracts_post_create_repost)
-  [Function `delete_post`](#social_contracts_post_delete_post)
-  [Function `delete_comment`](#social_contracts_post_delete_comment)
-  [Function `react_to_post`](#social_contracts_post_react_to_post)
-  [Function `tip_post`](#social_contracts_post_tip_post)
-  [Function `tip_post_simple`](#social_contracts_post_tip_post_simple)
-  [Function `apply_revenue_manifest_coin`](#social_contracts_post_apply_revenue_manifest_coin)
-  [Function `apply_revenue_manifest_coin_without_beneficiary_vault`](#social_contracts_post_apply_revenue_manifest_coin_without_beneficiary_vault)
-  [Function `embedded_bindings`](#social_contracts_post_embedded_bindings)
-  [Function `usage_decisions`](#social_contracts_post_usage_decisions)
-  [Function `usage_denials`](#social_contracts_post_usage_denials)
-  [Function `set_embedded_bindings`](#social_contracts_post_set_embedded_bindings)
-  [Function `set_usage_decisions`](#social_contracts_post_set_usage_decisions)
-  [Function `set_usage_denials`](#social_contracts_post_set_usage_denials)
-  [Function `find_binding_index`](#social_contracts_post_find_binding_index)
-  [Function `borrow_binding`](#social_contracts_post_borrow_binding)
-  [Function `upsert_binding`](#social_contracts_post_upsert_binding)
-  [Function `upsert_decision`](#social_contracts_post_upsert_decision)
-  [Function `upsert_denial`](#social_contracts_post_upsert_denial)
-  [Function `remove_denial`](#social_contracts_post_remove_denial)
-  [Function `apply_denials_to_snapshot`](#social_contracts_post_apply_denials_to_snapshot)
-  [Function `evaluate_binding_policy`](#social_contracts_post_evaluate_binding_policy)
-  [Function `refresh_composition_status_from_decisions`](#social_contracts_post_refresh_composition_status_from_decisions)
-  [Function `validate_binding`](#social_contracts_post_validate_binding)
-  [Function `write_usage_decision_for_binding`](#social_contracts_post_write_usage_decision_for_binding)
-  [Function `record_embedded_bindings`](#social_contracts_post_record_embedded_bindings)
-  [Function `refresh_post_asset_usage_decision`](#social_contracts_post_refresh_post_asset_usage_decision)
-  [Function `submit_candidate_revenue_manifest`](#social_contracts_post_submit_candidate_revenue_manifest)
-  [Function `deny_container_usage`](#social_contracts_post_deny_container_usage)
-  [Function `lift_container_usage_denial`](#social_contracts_post_lift_container_usage_denial)
-  [Function `set_composition_result`](#social_contracts_post_set_composition_result)
-  [Function `clear_composition_data`](#social_contracts_post_clear_composition_data)
-  [Function `clear_poc_data`](#social_contracts_post_clear_poc_data)
-  [Function `increment_composition_disputes_submitted`](#social_contracts_post_increment_composition_disputes_submitted)
-  [Function `increment_poc_disputes_submitted`](#social_contracts_post_increment_poc_disputes_submitted)
-  [Function `deposit_coin_to_beneficiary_vault`](#social_contracts_post_deposit_coin_to_beneficiary_vault)
-  [Function `tip_repost`](#social_contracts_post_tip_repost)
-  [Function `tip_comment`](#social_contracts_post_tip_comment)
-  [Function `tip_comment_simple`](#social_contracts_post_tip_comment_simple)
-  [Function `transfer_post_ownership`](#social_contracts_post_transfer_post_ownership)
-  [Function `admin_transfer_post_ownership`](#social_contracts_post_admin_transfer_post_ownership)
-  [Function `moderate_post`](#social_contracts_post_moderate_post)
-  [Function `moderate_comment`](#social_contracts_post_moderate_comment)
-  [Function `update_post`](#social_contracts_post_update_post)
-  [Function `update_comment`](#social_contracts_post_update_comment)
-  [Function `report_post`](#social_contracts_post_report_post)
-  [Function `report_comment`](#social_contracts_post_report_comment)
-  [Function `react_to_comment`](#social_contracts_post_react_to_comment)
-  [Function `edit_post`](#social_contracts_post_edit_post)
-  [Function `edit_comment`](#social_contracts_post_edit_comment)
-  [Function `remove_post_reaction`](#social_contracts_post_remove_post_reaction)
-  [Function `remove_comment_reaction`](#social_contracts_post_remove_comment_reaction)
-  [Function `remove_repost`](#social_contracts_post_remove_repost)
-  [Function `get_post_content`](#social_contracts_post_get_post_content)
-  [Function `get_post_owner`](#social_contracts_post_get_post_owner)
-  [Function `get_post_id`](#social_contracts_post_get_post_id)
-  [Function `get_post_comment_count`](#social_contracts_post_get_post_comment_count)
-  [Function `get_comment_owner`](#social_contracts_post_get_comment_owner)
-  [Function `get_comment_post_id`](#social_contracts_post_get_comment_post_id)
-  [Function `get_id_address`](#social_contracts_post_get_id_address)
-  [Function `get_reaction_count`](#social_contracts_post_get_reaction_count)
-  [Function `get_tips_received`](#social_contracts_post_get_tips_received)
-  [Function `get_comment_tips_received`](#social_contracts_post_get_comment_tips_received)
-  [Function `get_platform_id`](#social_contracts_post_get_platform_id)
-  [Function `get_revenue_redirect_to`](#social_contracts_post_get_revenue_redirect_to)
-  [Function `get_revenue_redirect_percentage`](#social_contracts_post_get_revenue_redirect_percentage)
-  [Function `poc_redirection_kind`](#social_contracts_post_poc_redirection_kind)
-  [Function `poc_redirection_none`](#social_contracts_post_poc_redirection_none)
-  [Function `version`](#social_contracts_post_version)
-  [Function `borrow_version_mut`](#social_contracts_post_borrow_version_mut)
-  [Function `comment_version`](#social_contracts_post_comment_version)
-  [Function `borrow_comment_version_mut`](#social_contracts_post_borrow_comment_version_mut)
-  [Function `repost_version`](#social_contracts_post_repost_version)
-  [Function `borrow_repost_version_mut`](#social_contracts_post_borrow_repost_version_mut)
-  [Function `migrate_post`](#social_contracts_post_migrate_post)
-  [Function `migrate_comment`](#social_contracts_post_migrate_comment)
-  [Function `migrate_repost`](#social_contracts_post_migrate_repost)
-  [Function `migrate_post_config`](#social_contracts_post_migrate_post_config)
-  [Function `update_post_parameters`](#social_contracts_post_update_post_parameters)
-  [Function `create_promoted_post`](#social_contracts_post_create_promoted_post)
-  [Function `confirm_promoted_post_views`](#social_contracts_post_confirm_promoted_post_views)
-  [Function `toggle_promotion_status`](#social_contracts_post_toggle_promotion_status)
-  [Function `withdraw_promotion_funds`](#social_contracts_post_withdraw_promotion_funds)
-  [Function `get_promotion_stats`](#social_contracts_post_get_promotion_stats)
-  [Function `has_user_viewed_promoted_post`](#social_contracts_post_has_user_viewed_promoted_post)
-  [Function `get_promotion_id`](#social_contracts_post_get_promotion_id)
-  [Function `set_moderation_status`](#social_contracts_post_set_moderation_status)
-  [Function `is_content_approved`](#social_contracts_post_is_content_approved)
-  [Function `assert_can_view_post`](#social_contracts_post_assert_can_view_post)
-  [Function `record_post_subscription_view`](#social_contracts_post_record_post_subscription_view)
-  [Function `create_post_admin_cap`](#social_contracts_post_create_post_admin_cap)


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
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
</code></pre>



<a name="social_contracts_post_Post"></a>

## Struct `Post`

Post object that contains content information


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/post.md#social_contracts_post_Post">Post</a> <b>has</b> key
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
<code>owner: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>profile_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>platform_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>content: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/post.md#social_contracts_post_media_asset_ids">media_asset_ids</a>: vector&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
 Canonical composition: ordered MediaAsset IDs (max 10).
</dd>
<dt>
<code>media: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;<a href="../myso/url.md#myso_url_Url">myso::url::Url</a>&gt;&gt;</code>
</dt>
<dd>
 Display/cache URLs — not authoritative for PoC.
</dd>
<dt>
<code>mentions: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;<b>address</b>&gt;&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>metadata_json: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>post_type: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code>parent_post_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>created_at: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>reaction_count: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>comment_count: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>repost_count: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>tips_received: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>removed_from_platform: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>user_reactions: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;<b>address</b>, <a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>reaction_counts: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>, u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>permissions: u8</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/post.md#social_contracts_post_composition_status">composition_status</a>: u8</code>
</dt>
<dd>
 Composition resolution status (display/use).
</dd>
<dt>
<code><a href="../social_contracts/post.md#social_contracts_post_monetization_status">monetization_status</a>: u8</code>
</dt>
<dd>
 Revenue eligibility status (separate from display).
</dd>
<dt>
<code><a href="../social_contracts/post.md#social_contracts_post_composition_badge">composition_badge</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../social_contracts/media_asset.md#social_contracts_media_asset_CompositionBadgeSnapshot">social_contracts::media_asset::CompositionBadgeSnapshot</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>access: <a href="../social_contracts/post.md#social_contracts_post_PostAccess">social_contracts::post::PostAccess</a></code>
</dt>
<dd>
</dd>
<dt>
<code>promotion_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>enable_spt: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>spt_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/post.md#social_contracts_post_version">version</a>: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_post_PostAttribution"></a>

## Struct `PostAttribution`

Published-action attribution stored as a dynamic field (Post is at the VM field-count limit).


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/post.md#social_contracts_post_PostAttribution">PostAttribution</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../social_contracts/post.md#social_contracts_post_actor_address">actor_address</a>: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/post.md#social_contracts_post_sub_agent_id">sub_agent_id</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>organization_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/post.md#social_contracts_post_action_identity_class">action_identity_class</a>: u8</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_post_CommentAttribution"></a>

## Struct `CommentAttribution`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/post.md#social_contracts_post_CommentAttribution">CommentAttribution</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../social_contracts/post.md#social_contracts_post_actor_address">actor_address</a>: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/post.md#social_contracts_post_sub_agent_id">sub_agent_id</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>organization_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/post.md#social_contracts_post_action_identity_class">action_identity_class</a>: u8</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_post_EmbeddedAssetBinding"></a>

## Struct `EmbeddedAssetBinding`

Phase 4 — embedded asset binding on a post container.


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/post.md#social_contracts_post_EmbeddedAssetBinding">EmbeddedAssetBinding</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>binding_id: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>source_asset_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>usage_class: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>stem: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>media_component: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>evidence_commitment: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;u8&gt;&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_post_UsageDecisionSnapshot"></a>

## Struct `UsageDecisionSnapshot`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/post.md#social_contracts_post_UsageDecisionSnapshot">UsageDecisionSnapshot</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>binding_id: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>policy_playback_permitted: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>playback_permitted: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>policy_reason_code: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>policy_version_at_decision: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_post_ContainerUsageDenial"></a>

## Struct `ContainerUsageDenial`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/post.md#social_contracts_post_ContainerUsageDenial">ContainerUsageDenial</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>binding_id: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>denial_scope: u8</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_post_EmbeddedBindingRecordedEvent"></a>

## Struct `EmbeddedBindingRecordedEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/post.md#social_contracts_post_EmbeddedBindingRecordedEvent">EmbeddedBindingRecordedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>post_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>bindings: vector&lt;<a href="../social_contracts/post.md#social_contracts_post_EmbeddedAssetBinding">social_contracts::post::EmbeddedAssetBinding</a>&gt;</code>
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

<a name="social_contracts_post_UsageDecisionRefreshedEvent"></a>

## Struct `UsageDecisionRefreshedEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/post.md#social_contracts_post_UsageDecisionRefreshedEvent">UsageDecisionRefreshedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>post_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>binding_id: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>policy_playback_permitted: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>playback_permitted: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>policy_reason_code: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>policy_version_at_decision: u64</code>
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

<a name="social_contracts_post_ContainerUsageDeniedEvent"></a>

## Struct `ContainerUsageDeniedEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/post.md#social_contracts_post_ContainerUsageDeniedEvent">ContainerUsageDeniedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>post_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>binding_id: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>denial_scope: u8</code>
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

<a name="social_contracts_post_ContainerUsageDenialLiftedEvent"></a>

## Struct `ContainerUsageDenialLiftedEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/post.md#social_contracts_post_ContainerUsageDenialLiftedEvent">ContainerUsageDenialLiftedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>post_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>binding_id: u64</code>
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

<a name="social_contracts_post_CandidateManifestSubmittedEvent"></a>

## Struct `CandidateManifestSubmittedEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/post.md#social_contracts_post_CandidateManifestSubmittedEvent">CandidateManifestSubmittedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>post_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>manifest_version: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>entries_json: vector&lt;<a href="../social_contracts/media_asset.md#social_contracts_media_asset_ManifestEntry">social_contracts::media_asset::ManifestEntry</a>&gt;</code>
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

<a name="social_contracts_post_PostSpotAnalysis"></a>

## Struct `PostSpotAnalysis`

Per-post multi-claim SPoT analysis, attached to <code><a href="../social_contracts/post.md#social_contracts_post">post</a>.id</code> via <code><a href="../social_contracts/post.md#social_contracts_post_SPOT_ANALYSIS_DF_KEY">SPOT_ANALYSIS_DF_KEY</a></code>.
Future-claim links are stored as aligned vectors (index/claim/market/policy); past
verdict detail lives off-chain (indexer/oracle) and is committed here only as counts
and manifest hashes.


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/post.md#social_contracts_post_PostSpotAnalysis">PostSpotAnalysis</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>status: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>detected_claim_count: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>rejected_claim_count: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>truncated_claim_count: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>future_accepted_count: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>past_verified_count: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_claim_per_post_applied: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>claim_indexes: vector&lt;u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>claim_ids: vector&lt;<b>address</b>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>market_ids: vector&lt;<b>address</b>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>policy_hashes: vector&lt;vector&lt;u8&gt;&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>claim_manifest_hash: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;u8&gt;&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>veracity_manifest_hash: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;u8&gt;&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_post_Comment"></a>

## Struct `Comment`

Comment object for posts, supporting nested comments


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/post.md#social_contracts_post_Comment">Comment</a> <b>has</b> key
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
<code>post_id: <b>address</b></code>
</dt>
<dd>
 The post this comment belongs to
</dd>
<dt>
<code>parent_comment_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;</code>
</dt>
<dd>
 Optional parent comment ID for nested comments
</dd>
<dt>
<code>owner: <b>address</b></code>
</dt>
<dd>
 Owner's wallet address (the true owner)
</dd>
<dt>
<code>profile_id: <b>address</b></code>
</dt>
<dd>
 Commenter's profile ID (reference only, not ownership)
</dd>
<dt>
<code>content: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
 Comment content
</dd>
<dt>
<code>media: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;<a href="../myso/url.md#myso_url_Url">myso::url::Url</a>&gt;&gt;</code>
</dt>
<dd>
 Optional media URLs
</dd>
<dt>
<code>mentions: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;<b>address</b>&gt;&gt;</code>
</dt>
<dd>
 Optional mentioned users (profile IDs)
</dd>
<dt>
<code>metadata_json: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
 Optional metadata in JSON format
</dd>
<dt>
<code>created_at: u64</code>
</dt>
<dd>
 Creation timestamp
</dd>
<dt>
<code>reaction_count: u64</code>
</dt>
<dd>
 Total number of reactions
</dd>
<dt>
<code>comment_count: u64</code>
</dt>
<dd>
 Number of nested comments
</dd>
<dt>
<code>repost_count: u64</code>
</dt>
<dd>
 Number of reposts
</dd>
<dt>
<code>tips_received: u64</code>
</dt>
<dd>
 Total tips received in MYSO (tracking only, not actual balance)
</dd>
<dt>
<code>removed_from_platform: bool</code>
</dt>
<dd>
 Whether the comment has been removed from its platform
</dd>
<dt>
<code>user_reactions: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;<b>address</b>, <a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
 Table of user wallet addresses to their reactions (emoji or text)
</dd>
<dt>
<code>reaction_counts: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>, u64&gt;</code>
</dt>
<dd>
 Table to count reactions by type
</dd>
<dt>
<code><a href="../social_contracts/post.md#social_contracts_post_version">version</a>: u64</code>
</dt>
<dd>
 Version for upgrades
</dd>
</dl>


</details>

<a name="social_contracts_post_Repost"></a>

## Struct `Repost`

Repost reference


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/post.md#social_contracts_post_Repost">Repost</a> <b>has</b> key
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
<code>original_id: <b>address</b></code>
</dt>
<dd>
 The post/comment being reposted
</dd>
<dt>
<code>is_original_post: bool</code>
</dt>
<dd>
 Whether the original is a post (true) or comment (false)
</dd>
<dt>
<code>owner: <b>address</b></code>
</dt>
<dd>
 Owner's wallet address (the true owner)
</dd>
<dt>
<code>profile_id: <b>address</b></code>
</dt>
<dd>
 Reposter's profile ID (reference only, not ownership)
</dd>
<dt>
<code>created_at: u64</code>
</dt>
<dd>
 Creation timestamp
</dd>
<dt>
<code><a href="../social_contracts/post.md#social_contracts_post_version">version</a>: u64</code>
</dt>
<dd>
 Version for upgrades
</dd>
</dl>


</details>

<a name="social_contracts_post_PromotionView"></a>

## Struct `PromotionView`

Promoted post view record


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/post.md#social_contracts_post_PromotionView">PromotionView</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>viewer: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>view_duration: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>view_timestamp: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>platform_id: <b>address</b></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_post_PromotionData"></a>

## Struct `PromotionData`

Promoted post metadata


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/post.md#social_contracts_post_PromotionData">PromotionData</a> <b>has</b> key
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
<code>post_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>payment_per_view: u64</code>
</dt>
<dd>
 Amount of MYSO to pay per view
</dd>
<dt>
<code>promotion_budget: <a href="../myso/balance.md#myso_balance_Balance">myso::balance::Balance</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;</code>
</dt>
<dd>
 MYSO balance available for payments
</dd>
<dt>
<code>paid_viewers: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;<b>address</b>, bool&gt;</code>
</dt>
<dd>
 Table tracking which users have already been paid for viewing
</dd>
<dt>
<code>views: vector&lt;<a href="../social_contracts/post.md#social_contracts_post_PromotionView">social_contracts::post::PromotionView</a>&gt;</code>
</dt>
<dd>
 List of all views for analytics
</dd>
<dt>
<code>active: bool</code>
</dt>
<dd>
 Whether the promotion is currently active
</dd>
<dt>
<code>created_at: u64</code>
</dt>
<dd>
 Promotion creation timestamp
</dd>
</dl>


</details>

<a name="social_contracts_post_PostAdminCap"></a>

## Struct `PostAdminCap`

Admin capability for post administration


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/post.md#social_contracts_post_PostAdminCap">PostAdminCap</a> <b>has</b> key, store
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

<a name="social_contracts_post_PostConfig"></a>

## Struct `PostConfig`

Global post feature configuration


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/post.md#social_contracts_post_PostConfig">PostConfig</a> <b>has</b> key
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
<code>max_content_length: u64</code>
</dt>
<dd>
 Maximum character length for post content
</dd>
<dt>
<code>max_media_urls: u64</code>
</dt>
<dd>
 Maximum number of media URLs per post
</dd>
<dt>
<code>max_mentions: u64</code>
</dt>
<dd>
 Maximum number of mentions in a post
</dd>
<dt>
<code>max_metadata_size: u64</code>
</dt>
<dd>
 Maximum size for post metadata in bytes
</dd>
<dt>
<code>max_description_length: u64</code>
</dt>
<dd>
 Maximum length for report descriptions
</dd>
<dt>
<code>max_reaction_length: u64</code>
</dt>
<dd>
 Maximum length for reactions
</dd>
<dt>
<code>commenter_tip_percentage: u64</code>
</dt>
<dd>
 Percentage of tip that goes to commenter (remainder to post owner)
</dd>
<dt>
<code>repost_tip_percentage: u64</code>
</dt>
<dd>
 Percentage of tip that goes to reposter (remainder to original post owner)
</dd>
<dt>
<code>min_promotion_amount: u64</code>
</dt>
<dd>
 Minimum payment per view for promoted posts (MIST)
</dd>
<dt>
<code>max_promotion_amount: u64</code>
</dt>
<dd>
 Maximum payment per view for promoted posts (MIST)
</dd>
<dt>
<code>min_view_duration_ms: u64</code>
</dt>
<dd>
 Minimum view duration for a promoted post view to count (ms)
</dd>
<dt>
<code>platform_fee_bps: u64</code>
</dt>
<dd>
 Platform fee bps taken from each confirmed promo view gross
</dd>
<dt>
<code>ecosystem_fee_bps: u64</code>
</dt>
<dd>
 Ecosystem fee bps taken from each confirmed promo view gross
</dd>
<dt>
<code><a href="../social_contracts/post.md#social_contracts_post_version">version</a>: u64</code>
</dt>
<dd>
 Version for upgrades
</dd>
</dl>


</details>

<a name="social_contracts_post_PostParametersUpdatedEvent"></a>

## Struct `PostParametersUpdatedEvent`

Event emitted when post parameters are updated


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/post.md#social_contracts_post_PostParametersUpdatedEvent">PostParametersUpdatedEvent</a> <b>has</b> <b>copy</b>, drop
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
<code>max_content_length: u64</code>
</dt>
<dd>
 New max content length value
</dd>
<dt>
<code>max_media_urls: u64</code>
</dt>
<dd>
 New max media URLs value
</dd>
<dt>
<code>max_mentions: u64</code>
</dt>
<dd>
 New max mentions value
</dd>
<dt>
<code>max_metadata_size: u64</code>
</dt>
<dd>
 New max metadata size value
</dd>
<dt>
<code>max_description_length: u64</code>
</dt>
<dd>
 New max description length value
</dd>
<dt>
<code>max_reaction_length: u64</code>
</dt>
<dd>
 New max reaction length value
</dd>
<dt>
<code>commenter_tip_percentage: u64</code>
</dt>
<dd>
 New commenter tip percentage value
</dd>
<dt>
<code>repost_tip_percentage: u64</code>
</dt>
<dd>
 New repost tip percentage value
</dd>
<dt>
<code>min_promotion_amount: u64</code>
</dt>
<dd>
 New min promotion amount value (MIST per view)
</dd>
<dt>
<code>max_promotion_amount: u64</code>
</dt>
<dd>
 New max promotion amount value (MIST per view)
</dd>
<dt>
<code>min_view_duration_ms: u64</code>
</dt>
<dd>
 New min view duration value (ms)
</dd>
<dt>
<code>platform_fee_bps: u64</code>
</dt>
<dd>
 New platform fee bps of each promo view gross
</dd>
<dt>
<code>ecosystem_fee_bps: u64</code>
</dt>
<dd>
 New ecosystem fee bps of each promo view gross
</dd>
</dl>


</details>

<a name="social_contracts_post_PostCreatedEvent"></a>

## Struct `PostCreatedEvent`

Post created event


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/post.md#social_contracts_post_PostCreatedEvent">PostCreatedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>post_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>owner: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>profile_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>platform_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>permissions: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>content: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code>post_type: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code>parent_post_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>mentions: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;<b>address</b>&gt;&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/post.md#social_contracts_post_media_asset_ids">media_asset_ids</a>: vector&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>media_urls: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>metadata_json: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>access: <a href="../social_contracts/post.md#social_contracts_post_PostAccess">social_contracts::post::PostAccess</a></code>
</dt>
<dd>
</dd>
<dt>
<code>promotion_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/post.md#social_contracts_post_composition_status">composition_status</a>: u8</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/post.md#social_contracts_post_monetization_status">monetization_status</a>: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>enable_spt: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>spt_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/post.md#social_contracts_post_actor_address">actor_address</a>: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/post.md#social_contracts_post_sub_agent_id">sub_agent_id</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>organization_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/post.md#social_contracts_post_action_identity_class">action_identity_class</a>: u8</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_post_CommentCreatedEvent"></a>

## Struct `CommentCreatedEvent`

Comment created event


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/post.md#social_contracts_post_CommentCreatedEvent">CommentCreatedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>comment_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>post_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>parent_comment_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>owner: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>profile_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>content: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code>mentions: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;<b>address</b>&gt;&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/post.md#social_contracts_post_actor_address">actor_address</a>: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/post.md#social_contracts_post_sub_agent_id">sub_agent_id</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>organization_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/post.md#social_contracts_post_action_identity_class">action_identity_class</a>: u8</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_post_RepostEvent"></a>

## Struct `RepostEvent`

Repost event


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/post.md#social_contracts_post_RepostEvent">RepostEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>repost_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>original_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>is_original_post: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>owner: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>profile_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/post.md#social_contracts_post_actor_address">actor_address</a>: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/post.md#social_contracts_post_sub_agent_id">sub_agent_id</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>organization_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/post.md#social_contracts_post_action_identity_class">action_identity_class</a>: u8</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_post_RepostRemovedEvent"></a>

## Struct `RepostRemovedEvent`

Repost removal event with principal and delegated-agent attribution.


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/post.md#social_contracts_post_RepostRemovedEvent">RepostRemovedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>repost_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>original_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>owner: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/post.md#social_contracts_post_actor_address">actor_address</a>: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/post.md#social_contracts_post_sub_agent_id">sub_agent_id</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>organization_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/post.md#social_contracts_post_action_identity_class">action_identity_class</a>: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>removed_at: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_post_ReactionEvent"></a>

## Struct `ReactionEvent`

Reaction event


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/post.md#social_contracts_post_ReactionEvent">ReactionEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>object_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>user: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>reaction: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code>is_post: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>principal_owner: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/post.md#social_contracts_post_actor_address">actor_address</a>: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/post.md#social_contracts_post_sub_agent_id">sub_agent_id</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>organization_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/post.md#social_contracts_post_action_identity_class">action_identity_class</a>: u8</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_post_RemoveReactionEvent"></a>

## Struct `RemoveReactionEvent`

Remove reaction event


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/post.md#social_contracts_post_RemoveReactionEvent">RemoveReactionEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>object_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>user: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>reaction: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code>is_post: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>principal_owner: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/post.md#social_contracts_post_actor_address">actor_address</a>: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/post.md#social_contracts_post_sub_agent_id">sub_agent_id</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>organization_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/post.md#social_contracts_post_action_identity_class">action_identity_class</a>: u8</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_post_TipEvent"></a>

## Struct `TipEvent`

Tip event


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/post.md#social_contracts_post_TipEvent">TipEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>object_id: <b>address</b></code>
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
</dd>
<dt>
<code>coin_type: <a href="../std/type_name.md#std_type_name_TypeName">std::type_name::TypeName</a></code>
</dt>
<dd>
</dd>
<dt>
<code>is_post: bool</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_post_OwnershipTransferEvent"></a>

## Struct `OwnershipTransferEvent`

Post ownership transfer event


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/post.md#social_contracts_post_OwnershipTransferEvent">OwnershipTransferEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>object_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>previous_owner: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>new_owner: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>is_post: bool</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_post_PostModerationEvent"></a>

## Struct `PostModerationEvent`

Post moderation event


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/post.md#social_contracts_post_PostModerationEvent">PostModerationEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>post_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>platform_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>removed: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>moderated_by: <b>address</b></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_post_PostUpdatedEvent"></a>

## Struct `PostUpdatedEvent`

Post updated event


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/post.md#social_contracts_post_PostUpdatedEvent">PostUpdatedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>post_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>owner: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>profile_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>content: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code>metadata_json: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
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

<a name="social_contracts_post_CommentUpdatedEvent"></a>

## Struct `CommentUpdatedEvent`

Comment updated event


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/post.md#social_contracts_post_CommentUpdatedEvent">CommentUpdatedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>comment_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>post_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>owner: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>profile_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>content: <a href="../std/string.md#std_string_String">std::string::String</a></code>
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

<a name="social_contracts_post_PostReportedEvent"></a>

## Struct `PostReportedEvent`

Post reported event


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/post.md#social_contracts_post_PostReportedEvent">PostReportedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>post_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>reporter: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>reason_code: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>description: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code>reported_at: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_post_CommentReportedEvent"></a>

## Struct `CommentReportedEvent`

Comment reported event


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/post.md#social_contracts_post_CommentReportedEvent">CommentReportedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>comment_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>reporter: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>reason_code: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>description: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code>reported_at: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_post_PostDeletedEvent"></a>

## Struct `PostDeletedEvent`

Post deleted event


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/post.md#social_contracts_post_PostDeletedEvent">PostDeletedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>post_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>owner: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>profile_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>post_type: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code>deleted_at: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_post_CommentDeletedEvent"></a>

## Struct `CommentDeletedEvent`

Comment deleted event


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/post.md#social_contracts_post_CommentDeletedEvent">CommentDeletedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>comment_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>post_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>owner: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>profile_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>deleted_at: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_post_PromotedPostCreatedEvent"></a>

## Struct `PromotedPostCreatedEvent`

Event emitted when a promoted post is created


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/post.md#social_contracts_post_PromotedPostCreatedEvent">PromotedPostCreatedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>post_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>owner: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>profile_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>payment_per_view: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>total_budget: u64</code>
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

<a name="social_contracts_post_PromotedViewConfirmItem"></a>

## Struct `PromotedViewConfirmItem`

One item inside a batch promoted-view confirmation


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/post.md#social_contracts_post_PromotedViewConfirmItem">PromotedViewConfirmItem</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>post_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>promotion_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>payment_amount: u64</code>
</dt>
<dd>
 Gross debit from this promotion's budget (<code>payment_per_view</code>)
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
<code>recipient_amount: u64</code>
</dt>
<dd>
 Net attributed to this view (part of the merged wallet transfer)
</dd>
<dt>
<code>view_duration: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_post_PromotedPostViewsBatchConfirmedEvent"></a>

## Struct `PromotedPostViewsBatchConfirmedEvent`

Event emitted when one viewer is paid for N (≥1) promoted views in a single tx


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/post.md#social_contracts_post_PromotedPostViewsBatchConfirmedEvent">PromotedPostViewsBatchConfirmedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>viewer: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>platform_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>timestamp: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>items: vector&lt;<a href="../social_contracts/post.md#social_contracts_post_PromotedViewConfirmItem">social_contracts::post::PromotedViewConfirmItem</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>total_payment_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>total_platform_fee: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>total_ecosystem_fee: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>total_recipient_amount: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_post_PromotionStatusToggledEvent"></a>

## Struct `PromotionStatusToggledEvent`

Event emitted when promotion status is toggled


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/post.md#social_contracts_post_PromotionStatusToggledEvent">PromotionStatusToggledEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>post_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>toggled_by: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>new_status: bool</code>
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

<a name="social_contracts_post_PromotionFundsWithdrawnEvent"></a>

## Struct `PromotionFundsWithdrawnEvent`

Event emitted when promotion funds are withdrawn


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/post.md#social_contracts_post_PromotionFundsWithdrawnEvent">PromotionFundsWithdrawnEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>post_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>owner: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>withdrawn_amount: u64</code>
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

<a name="social_contracts_post_ModerationRecord"></a>

## Struct `ModerationRecord`

Simple moderation record for tracking moderation decisions


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/post.md#social_contracts_post_ModerationRecord">ModerationRecord</a> <b>has</b> key
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
<code>post_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>platform_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>moderation_state: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>moderator: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>moderation_timestamp: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
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

<a name="social_contracts_post_PostSubscriptionAccessEvent"></a>

## Struct `PostSubscriptionAccessEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/post.md#social_contracts_post_PostSubscriptionAccessEvent">PostSubscriptionAccessEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>post_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>service_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
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
<code>timestamp: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_post_PostAccess"></a>

## Enum `PostAccess`

Post content access model (replaces legacy <code>mydata_id</code> + subscription gate dynamic field).


<pre><code><b>public</b> <b>enum</b> <a href="../social_contracts/post.md#social_contracts_post_PostAccess">PostAccess</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Variants</summary>


<dl>
<dt>
Variant <code>Public</code>
</dt>
<dd>
</dd>
<dt>
Variant <code>ProfileSubscription</code>
</dt>
<dd>
</dd>

<dl>
<dt>
<code>service_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
</dl>


<dl>
<dt>
<code>mydata_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
</dl>


<dl>
<dt>
<code>min_tier_level: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
</dl>

<dt>
Variant <code>MarketplaceOneTime</code>
</dt>
<dd>
</dd>

<dl>
<dt>
<code>mydata_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
</dl>

</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="social_contracts_post_EUnauthorized"></a>

Error codes


<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_EUnauthorized">EUnauthorized</a>: u64 = 0;
</code></pre>



<a name="social_contracts_post_EPostNotFound"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_EPostNotFound">EPostNotFound</a>: u64 = 1;
</code></pre>



<a name="social_contracts_post_EInvalidTipAmount"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_EInvalidTipAmount">EInvalidTipAmount</a>: u64 = 2;
</code></pre>



<a name="social_contracts_post_ESelfTipping"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_ESelfTipping">ESelfTipping</a>: u64 = 3;
</code></pre>



<a name="social_contracts_post_EInvalidParentReference"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_EInvalidParentReference">EInvalidParentReference</a>: u64 = 4;
</code></pre>



<a name="social_contracts_post_EContentTooLarge"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_EContentTooLarge">EContentTooLarge</a>: u64 = 5;
</code></pre>



<a name="social_contracts_post_ETooManyMediaUrls"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_ETooManyMediaUrls">ETooManyMediaUrls</a>: u64 = 6;
</code></pre>



<a name="social_contracts_post_EInvalidPostType"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_EInvalidPostType">EInvalidPostType</a>: u64 = 7;
</code></pre>



<a name="social_contracts_post_EUnauthorizedTransfer"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_EUnauthorizedTransfer">EUnauthorizedTransfer</a>: u64 = 8;
</code></pre>



<a name="social_contracts_post_EReportReasonInvalid"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_EReportReasonInvalid">EReportReasonInvalid</a>: u64 = 9;
</code></pre>



<a name="social_contracts_post_EReportDescriptionTooLong"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_EReportDescriptionTooLong">EReportDescriptionTooLong</a>: u64 = 10;
</code></pre>



<a name="social_contracts_post_EReactionContentTooLong"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_EReactionContentTooLong">EReactionContentTooLong</a>: u64 = 11;
</code></pre>



<a name="social_contracts_post_EUserNotJoinedPlatform"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_EUserNotJoinedPlatform">EUserNotJoinedPlatform</a>: u64 = 12;
</code></pre>



<a name="social_contracts_post_EUserBlockedByPlatform"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_EUserBlockedByPlatform">EUserBlockedByPlatform</a>: u64 = 13;
</code></pre>



<a name="social_contracts_post_EWrongVersion"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_EWrongVersion">EWrongVersion</a>: u64 = 14;
</code></pre>



<a name="social_contracts_post_EReactionsNotAllowed"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_EReactionsNotAllowed">EReactionsNotAllowed</a>: u64 = 15;
</code></pre>



<a name="social_contracts_post_ECommentsNotAllowed"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_ECommentsNotAllowed">ECommentsNotAllowed</a>: u64 = 16;
</code></pre>



<a name="social_contracts_post_ERepostsNotAllowed"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_ERepostsNotAllowed">ERepostsNotAllowed</a>: u64 = 17;
</code></pre>



<a name="social_contracts_post_EQuotesNotAllowed"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_EQuotesNotAllowed">EQuotesNotAllowed</a>: u64 = 18;
</code></pre>



<a name="social_contracts_post_ETipsNotAllowed"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_ETipsNotAllowed">ETipsNotAllowed</a>: u64 = 19;
</code></pre>



<a name="social_contracts_post_EInvalidConfig"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_EInvalidConfig">EInvalidConfig</a>: u64 = 20;
</code></pre>



<a name="social_contracts_post_ENoSubscriptionService"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_ENoSubscriptionService">ENoSubscriptionService</a>: u64 = 21;
</code></pre>



<a name="social_contracts_post_ENoEncryptedContent"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_ENoEncryptedContent">ENoEncryptedContent</a>: u64 = 22;
</code></pre>



<a name="social_contracts_post_EPriceMismatch"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_EPriceMismatch">EPriceMismatch</a>: u64 = 23;
</code></pre>



<a name="social_contracts_post_EPromotionAmountTooLow"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_EPromotionAmountTooLow">EPromotionAmountTooLow</a>: u64 = 24;
</code></pre>



<a name="social_contracts_post_EPromotionAmountTooHigh"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_EPromotionAmountTooHigh">EPromotionAmountTooHigh</a>: u64 = 25;
</code></pre>



<a name="social_contracts_post_ENotPromotedPost"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_ENotPromotedPost">ENotPromotedPost</a>: u64 = 26;
</code></pre>



<a name="social_contracts_post_EUserAlreadyViewed"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_EUserAlreadyViewed">EUserAlreadyViewed</a>: u64 = 27;
</code></pre>



<a name="social_contracts_post_EInsufficientPromotionFunds"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_EInsufficientPromotionFunds">EInsufficientPromotionFunds</a>: u64 = 28;
</code></pre>



<a name="social_contracts_post_EPromotionInactive"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_EPromotionInactive">EPromotionInactive</a>: u64 = 29;
</code></pre>



<a name="social_contracts_post_EInvalidViewDuration"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_EInvalidViewDuration">EInvalidViewDuration</a>: u64 = 30;
</code></pre>



<a name="social_contracts_post_EOverflow"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_EOverflow">EOverflow</a>: u64 = 31;
</code></pre>



<a name="social_contracts_post_EMyDataNotRegistered"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_EMyDataNotRegistered">EMyDataNotRegistered</a>: u64 = 32;
</code></pre>



<a name="social_contracts_post_EMyDataOwnerMismatch"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_EMyDataOwnerMismatch">EMyDataOwnerMismatch</a>: u64 = 33;
</code></pre>



<a name="social_contracts_post_EDisputeCapReached"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_EDisputeCapReached">EDisputeCapReached</a>: u64 = 36;
</code></pre>



<a name="social_contracts_post_EWrongBeneficiaryVault"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_EWrongBeneficiaryVault">EWrongBeneficiaryVault</a>: u64 = 37;
</code></pre>



<a name="social_contracts_post_ETipPostRequiresBeneficiaryVault"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_ETipPostRequiresBeneficiaryVault">ETipPostRequiresBeneficiaryVault</a>: u64 = 38;
</code></pre>



<a name="social_contracts_post_EInvalidBatch"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_EInvalidBatch">EInvalidBatch</a>: u64 = 39;
</code></pre>



<a name="social_contracts_post_ESptRequiresDedicatedCreate"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_ESptRequiresDedicatedCreate">ESptRequiresDedicatedCreate</a>: u64 = 40;
</code></pre>



<a name="social_contracts_post_EInvalidMediaAsset"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_EInvalidMediaAsset">EInvalidMediaAsset</a>: u64 = 41;
</code></pre>



<a name="social_contracts_post_EMonetizationNotEnabled"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_EMonetizationNotEnabled">EMonetizationNotEnabled</a>: u64 = 42;
</code></pre>



<a name="social_contracts_post_EInvalidManifest"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_EInvalidManifest">EInvalidManifest</a>: u64 = 43;
</code></pre>



<a name="social_contracts_post_ECompositionNotVerified"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_ECompositionNotVerified">ECompositionNotVerified</a>: u64 = 44;
</code></pre>



<a name="social_contracts_post_EBindingNotFound"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_EBindingNotFound">EBindingNotFound</a>: u64 = 95;
</code></pre>



<a name="social_contracts_post_EInvalidDenialScope"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_EInvalidDenialScope">EInvalidDenialScope</a>: u64 = 96;
</code></pre>



<a name="social_contracts_post_EBindingAssetMismatch"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_EBindingAssetMismatch">EBindingAssetMismatch</a>: u64 = 97;
</code></pre>



<a name="social_contracts_post_EInvalidBinding"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_EInvalidBinding">EInvalidBinding</a>: u64 = 98;
</code></pre>



<a name="social_contracts_post_MAX_CONTENT_LENGTH"></a>

Constants for size limits


<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_MAX_CONTENT_LENGTH">MAX_CONTENT_LENGTH</a>: u64 = 5000;
</code></pre>



<a name="social_contracts_post_MAX_MEDIA_URLS"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_MAX_MEDIA_URLS">MAX_MEDIA_URLS</a>: u64 = 10;
</code></pre>



<a name="social_contracts_post_MAX_MENTIONS"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_MAX_MENTIONS">MAX_MENTIONS</a>: u64 = 10;
</code></pre>



<a name="social_contracts_post_MAX_METADATA_SIZE"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_MAX_METADATA_SIZE">MAX_METADATA_SIZE</a>: u64 = 10000;
</code></pre>



<a name="social_contracts_post_MAX_DESCRIPTION_LENGTH"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_MAX_DESCRIPTION_LENGTH">MAX_DESCRIPTION_LENGTH</a>: u64 = 500;
</code></pre>



<a name="social_contracts_post_MAX_REACTION_LENGTH"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_MAX_REACTION_LENGTH">MAX_REACTION_LENGTH</a>: u64 = 20;
</code></pre>



<a name="social_contracts_post_COMMENTER_TIP_PERCENTAGE"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_COMMENTER_TIP_PERCENTAGE">COMMENTER_TIP_PERCENTAGE</a>: u64 = 80;
</code></pre>



<a name="social_contracts_post_REPOST_TIP_PERCENTAGE"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_REPOST_TIP_PERCENTAGE">REPOST_TIP_PERCENTAGE</a>: u64 = 50;
</code></pre>



<a name="social_contracts_post_MAX_U64"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_MAX_U64">MAX_U64</a>: u64 = 18446744073709551615;
</code></pre>



<a name="social_contracts_post_MIN_PROMOTION_AMOUNT"></a>

Constants for promoted posts


<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_MIN_PROMOTION_AMOUNT">MIN_PROMOTION_AMOUNT</a>: u64 = 1000;
</code></pre>



<a name="social_contracts_post_MAX_PROMOTION_AMOUNT"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_MAX_PROMOTION_AMOUNT">MAX_PROMOTION_AMOUNT</a>: u64 = 100000000;
</code></pre>



<a name="social_contracts_post_MIN_VIEW_DURATION"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_MIN_VIEW_DURATION">MIN_VIEW_DURATION</a>: u64 = 3000;
</code></pre>



<a name="social_contracts_post_MAX_PROMOTION_VIEW_BATCH"></a>

Max promotions confirmed in one <code><a href="../social_contracts/post.md#social_contracts_post_confirm_promoted_post_views">confirm_promoted_post_views</a></code> call (gas / object-lock bound)


<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_MAX_PROMOTION_VIEW_BATCH">MAX_PROMOTION_VIEW_BATCH</a>: u64 = 50;
</code></pre>



<a name="social_contracts_post_BPS_DENOM"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_BPS_DENOM">BPS_DENOM</a>: u64 = 10000;
</code></pre>



<a name="social_contracts_post_DEFAULT_PLATFORM_FEE_BPS"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_DEFAULT_PLATFORM_FEE_BPS">DEFAULT_PLATFORM_FEE_BPS</a>: u64 = 1000;
</code></pre>



<a name="social_contracts_post_DEFAULT_ECOSYSTEM_FEE_BPS"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_DEFAULT_ECOSYSTEM_FEE_BPS">DEFAULT_ECOSYSTEM_FEE_BPS</a>: u64 = 1000;
</code></pre>



<a name="social_contracts_post_POST_TYPE_STANDARD"></a>

Valid post types


<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_POST_TYPE_STANDARD">POST_TYPE_STANDARD</a>: vector&lt;u8&gt; = vector[115, 116, 97, 110, 100, 97, 114, 100];
</code></pre>



<a name="social_contracts_post_POST_TYPE_REPOST"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_POST_TYPE_REPOST">POST_TYPE_REPOST</a>: vector&lt;u8&gt; = vector[114, 101, 112, 111, 115, 116];
</code></pre>



<a name="social_contracts_post_POST_TYPE_QUOTE_REPOST"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_POST_TYPE_QUOTE_REPOST">POST_TYPE_QUOTE_REPOST</a>: vector&lt;u8&gt; = vector[113, 117, 111, 116, 101, 95, 114, 101, 112, 111, 115, 116];
</code></pre>



<a name="social_contracts_post_REPORT_REASON_SPAM"></a>

Constants for report reason codes


<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_REPORT_REASON_SPAM">REPORT_REASON_SPAM</a>: u8 = 1;
</code></pre>



<a name="social_contracts_post_REPORT_REASON_OFFENSIVE"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_REPORT_REASON_OFFENSIVE">REPORT_REASON_OFFENSIVE</a>: u8 = 2;
</code></pre>



<a name="social_contracts_post_REPORT_REASON_MISINFORMATION"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_REPORT_REASON_MISINFORMATION">REPORT_REASON_MISINFORMATION</a>: u8 = 3;
</code></pre>



<a name="social_contracts_post_REPORT_REASON_ILLEGAL"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_REPORT_REASON_ILLEGAL">REPORT_REASON_ILLEGAL</a>: u8 = 4;
</code></pre>



<a name="social_contracts_post_REPORT_REASON_IMPERSONATION"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_REPORT_REASON_IMPERSONATION">REPORT_REASON_IMPERSONATION</a>: u8 = 5;
</code></pre>



<a name="social_contracts_post_REPORT_REASON_HARASSMENT"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_REPORT_REASON_HARASSMENT">REPORT_REASON_HARASSMENT</a>: u8 = 6;
</code></pre>



<a name="social_contracts_post_REPORT_REASON_OTHER"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_REPORT_REASON_OTHER">REPORT_REASON_OTHER</a>: u8 = 99;
</code></pre>



<a name="social_contracts_post_MODERATION_APPROVED"></a>

Constants for moderation states


<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_MODERATION_APPROVED">MODERATION_APPROVED</a>: u8 = 1;
</code></pre>



<a name="social_contracts_post_MODERATION_FLAGGED"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_MODERATION_FLAGGED">MODERATION_FLAGGED</a>: u8 = 2;
</code></pre>



<a name="social_contracts_post_PERMISSION_ALLOW_COMMENTS"></a>

Bitfield constants for permission flags (allow_*)


<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_PERMISSION_ALLOW_COMMENTS">PERMISSION_ALLOW_COMMENTS</a>: u8 = 1;
</code></pre>



<a name="social_contracts_post_PERMISSION_ALLOW_REACTIONS"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_PERMISSION_ALLOW_REACTIONS">PERMISSION_ALLOW_REACTIONS</a>: u8 = 2;
</code></pre>



<a name="social_contracts_post_PERMISSION_ALLOW_REPOSTS"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_PERMISSION_ALLOW_REPOSTS">PERMISSION_ALLOW_REPOSTS</a>: u8 = 4;
</code></pre>



<a name="social_contracts_post_PERMISSION_ALLOW_QUOTES"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_PERMISSION_ALLOW_QUOTES">PERMISSION_ALLOW_QUOTES</a>: u8 = 8;
</code></pre>



<a name="social_contracts_post_PERMISSION_ALLOW_TIPS"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_PERMISSION_ALLOW_TIPS">PERMISSION_ALLOW_TIPS</a>: u8 = 16;
</code></pre>



<a name="social_contracts_post_POST_ACCESS_PUBLIC"></a>

Event/indexer tags for [<code><a href="../social_contracts/post.md#social_contracts_post_PostAccess">PostAccess</a></code>] (1=public, 2=profile_sub, 3=marketplace_one_time).


<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_POST_ACCESS_PUBLIC">POST_ACCESS_PUBLIC</a>: u8 = 1;
</code></pre>



<a name="social_contracts_post_POST_ACCESS_PROFILE_SUBSCRIPTION"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_POST_ACCESS_PROFILE_SUBSCRIPTION">POST_ACCESS_PROFILE_SUBSCRIPTION</a>: u8 = 2;
</code></pre>



<a name="social_contracts_post_POST_ACCESS_MARKETPLACE_ONE_TIME"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_POST_ACCESS_MARKETPLACE_ONE_TIME">POST_ACCESS_MARKETPLACE_ONE_TIME</a>: u8 = 3;
</code></pre>



<a name="social_contracts_post_POST_ATTRIBUTION_DF_KEY"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_POST_ATTRIBUTION_DF_KEY">POST_ATTRIBUTION_DF_KEY</a>: vector&lt;u8&gt; = vector[112, 111, 115, 116, 95, 97, 116, 116, 114, 105, 98, 117, 116, 105, 111, 110];
</code></pre>



<a name="social_contracts_post_COMMENT_ATTRIBUTION_DF_KEY"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_COMMENT_ATTRIBUTION_DF_KEY">COMMENT_ATTRIBUTION_DF_KEY</a>: vector&lt;u8&gt; = vector[99, 111, 109, 109, 101, 110, 116, 95, 97, 116, 116, 114, 105, 98, 117, 116, 105, 111, 110];
</code></pre>



<a name="social_contracts_post_COMPOSITION_ANALYSIS_DF_KEY"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_COMPOSITION_ANALYSIS_DF_KEY">COMPOSITION_ANALYSIS_DF_KEY</a>: vector&lt;u8&gt; = vector[99, 111, 109, 112, 111, 115, 105, 116, 105, 111, 110, 95, 97, 110, 97, 108, 121, 115, 105, 115];
</code></pre>



<a name="social_contracts_post_REVENUE_MANIFEST_DF_KEY"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_REVENUE_MANIFEST_DF_KEY">REVENUE_MANIFEST_DF_KEY</a>: vector&lt;u8&gt; = vector[114, 101, 118, 101, 110, 117, 101, 95, 109, 97, 110, 105, 102, 101, 115, 116];
</code></pre>



<a name="social_contracts_post_COMPOSITION_DISPUTES_DF_KEY"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_COMPOSITION_DISPUTES_DF_KEY">COMPOSITION_DISPUTES_DF_KEY</a>: vector&lt;u8&gt; = vector[99, 111, 109, 112, 111, 115, 105, 116, 105, 111, 110, 95, 100, 105, 115, 112, 117, 116, 101, 115, 95, 115, 117, 98, 109, 105, 116, 116, 101, 100];
</code></pre>



<a name="social_contracts_post_SPOT_ANALYSIS_DF_KEY"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_SPOT_ANALYSIS_DF_KEY">SPOT_ANALYSIS_DF_KEY</a>: vector&lt;u8&gt; = vector[115, 112, 111, 116, 95, 97, 110, 97, 108, 121, 115, 105, 115];
</code></pre>



<a name="social_contracts_post_EMBEDDED_BINDINGS_DF_KEY"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_EMBEDDED_BINDINGS_DF_KEY">EMBEDDED_BINDINGS_DF_KEY</a>: vector&lt;u8&gt; = vector[101, 109, 98, 101, 100, 100, 101, 100, 95, 98, 105, 110, 100, 105, 110, 103, 115];
</code></pre>



<a name="social_contracts_post_USAGE_DECISIONS_DF_KEY"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_USAGE_DECISIONS_DF_KEY">USAGE_DECISIONS_DF_KEY</a>: vector&lt;u8&gt; = vector[117, 115, 97, 103, 101, 95, 100, 101, 99, 105, 115, 105, 111, 110, 115];
</code></pre>



<a name="social_contracts_post_USAGE_DENIALS_DF_KEY"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_USAGE_DENIALS_DF_KEY">USAGE_DENIALS_DF_KEY</a>: vector&lt;u8&gt; = vector[117, 115, 97, 103, 101, 95, 100, 101, 110, 105, 97, 108, 115];
</code></pre>



<a name="social_contracts_post_CANDIDATE_MANIFEST_DF_KEY"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_CANDIDATE_MANIFEST_DF_KEY">CANDIDATE_MANIFEST_DF_KEY</a>: vector&lt;u8&gt; = vector[99, 97, 110, 100, 105, 100, 97, 116, 101, 95, 114, 101, 118, 101, 110, 117, 101, 95, 109, 97, 110, 105, 102, 101, 115, 116];
</code></pre>



<a name="social_contracts_post_REASON_ALLOWED"></a>

Application enforcement — policy reason codes for usage decision snapshots.


<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_REASON_ALLOWED">REASON_ALLOWED</a>: u8 = 0;
</code></pre>



<a name="social_contracts_post_REASON_NO_GRANT"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_REASON_NO_GRANT">REASON_NO_GRANT</a>: u8 = 1;
</code></pre>



<a name="social_contracts_post_REASON_RESOLVED_POLICY"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_REASON_RESOLVED_POLICY">REASON_RESOLVED_POLICY</a>: u8 = 2;
</code></pre>



<a name="social_contracts_post_REASON_DENIAL"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_REASON_DENIAL">REASON_DENIAL</a>: u8 = 3;
</code></pre>



<a name="social_contracts_post_DENIAL_SCOPE_PLAYBACK"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_DENIAL_SCOPE_PLAYBACK">DENIAL_SCOPE_PLAYBACK</a>: u8 = 1;
</code></pre>



<a name="social_contracts_post_SPOT_STATUS_PENDING"></a>

Multi-claim Social Proof of Truth analysis (Post is at the VM field-count limit).
SPoT multi-claim analysis lifecycle status (interpreted by <code><a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth">social_proof_of_truth</a></code>).


<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_SPOT_STATUS_PENDING">SPOT_STATUS_PENDING</a>: u8 = 0;
</code></pre>



<a name="social_contracts_post_SPOT_STATUS_COMPLETED"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_SPOT_STATUS_COMPLETED">SPOT_STATUS_COMPLETED</a>: u8 = 1;
</code></pre>



<a name="social_contracts_post_SPOT_STATUS_COMPLETED_NO_ACTIONABLE"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_SPOT_STATUS_COMPLETED_NO_ACTIONABLE">SPOT_STATUS_COMPLETED_NO_ACTIONABLE</a>: u8 = 2;
</code></pre>



<a name="social_contracts_post_ESpotAnalysisNotPending"></a>

Errors surfaced by SPoT analysis mutators.


<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_ESpotAnalysisNotPending">ESpotAnalysisNotPending</a>: u64 = 90;
</code></pre>



<a name="social_contracts_post_ESpotAnalysisVectorMismatch"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_ESpotAnalysisVectorMismatch">ESpotAnalysisVectorMismatch</a>: u64 = 91;
</code></pre>



<a name="social_contracts_post_ESpotAnalysisIndexOrder"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_ESpotAnalysisIndexOrder">ESpotAnalysisIndexOrder</a>: u64 = 92;
</code></pre>



<a name="social_contracts_post_ESpotAnalysisOverCap"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_ESpotAnalysisOverCap">ESpotAnalysisOverCap</a>: u64 = 93;
</code></pre>



<a name="social_contracts_post_ESpotAnalysisDuplicateMarket"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_ESpotAnalysisDuplicateMarket">ESpotAnalysisDuplicateMarket</a>: u64 = 94;
</code></pre>



<a name="social_contracts_post_has_flag"></a>

## Function `has_flag`

Helper: check if a bit is set in a bitfield


<pre><code><b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_has_flag">has_flag</a>(value: u8, flag: u8): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_has_flag">has_flag</a>(value: u8, flag: u8): bool {
    (value & flag) == flag
}
</code></pre>



</details>

<a name="social_contracts_post_set_flag"></a>

## Function `set_flag`

Helper: set a bit in a bitfield


<pre><code><b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_set_flag">set_flag</a>(value: &<b>mut</b> u8, flag: u8)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_set_flag">set_flag</a>(value: &<b>mut</b> u8, flag: u8) {
    *value = *value | flag
}
</code></pre>



</details>

<a name="social_contracts_post_clear_flag"></a>

## Function `clear_flag`

Helper: clear a bit in a bitfield


<pre><code><b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_clear_flag">clear_flag</a>(value: &<b>mut</b> u8, flag: u8)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_clear_flag">clear_flag</a>(value: &<b>mut</b> u8, flag: u8) {
    *value = *value & (255 - flag)
}
</code></pre>



</details>

<a name="social_contracts_post_permissions_bitfield"></a>

## Function `permissions_bitfield`

Bitfield for <code><a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>.permissions</code> / <code><a href="../social_contracts/post.md#social_contracts_post_PostCreatedEvent">PostCreatedEvent</a>.permissions</code> (matches <code><a href="../social_contracts/post.md#social_contracts_post_create_post_internal">create_post_internal</a></code>).


<pre><code><b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_permissions_bitfield">permissions_bitfield</a>(<a href="../social_contracts/post.md#social_contracts_post_allow_comments">allow_comments</a>: bool, <a href="../social_contracts/post.md#social_contracts_post_allow_reactions">allow_reactions</a>: bool, <a href="../social_contracts/post.md#social_contracts_post_allow_reposts">allow_reposts</a>: bool, <a href="../social_contracts/post.md#social_contracts_post_allow_quotes">allow_quotes</a>: bool, <a href="../social_contracts/post.md#social_contracts_post_allow_tips">allow_tips</a>: bool): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_permissions_bitfield">permissions_bitfield</a>(
    <a href="../social_contracts/post.md#social_contracts_post_allow_comments">allow_comments</a>: bool,
    <a href="../social_contracts/post.md#social_contracts_post_allow_reactions">allow_reactions</a>: bool,
    <a href="../social_contracts/post.md#social_contracts_post_allow_reposts">allow_reposts</a>: bool,
    <a href="../social_contracts/post.md#social_contracts_post_allow_quotes">allow_quotes</a>: bool,
    <a href="../social_contracts/post.md#social_contracts_post_allow_tips">allow_tips</a>: bool,
): u8 {
    <b>let</b> <b>mut</b> p: u8 = 0;
    <b>if</b> (<a href="../social_contracts/post.md#social_contracts_post_allow_comments">allow_comments</a>) { p = p | <a href="../social_contracts/post.md#social_contracts_post_PERMISSION_ALLOW_COMMENTS">PERMISSION_ALLOW_COMMENTS</a> };
    <b>if</b> (<a href="../social_contracts/post.md#social_contracts_post_allow_reactions">allow_reactions</a>) { p = p | <a href="../social_contracts/post.md#social_contracts_post_PERMISSION_ALLOW_REACTIONS">PERMISSION_ALLOW_REACTIONS</a> };
    <b>if</b> (<a href="../social_contracts/post.md#social_contracts_post_allow_reposts">allow_reposts</a>) { p = p | <a href="../social_contracts/post.md#social_contracts_post_PERMISSION_ALLOW_REPOSTS">PERMISSION_ALLOW_REPOSTS</a> };
    <b>if</b> (<a href="../social_contracts/post.md#social_contracts_post_allow_quotes">allow_quotes</a>) { p = p | <a href="../social_contracts/post.md#social_contracts_post_PERMISSION_ALLOW_QUOTES">PERMISSION_ALLOW_QUOTES</a> };
    <b>if</b> (<a href="../social_contracts/post.md#social_contracts_post_allow_tips">allow_tips</a>) { p = p | <a href="../social_contracts/post.md#social_contracts_post_PERMISSION_ALLOW_TIPS">PERMISSION_ALLOW_TIPS</a> };
    p
}
</code></pre>



</details>

<a name="social_contracts_post_post_access_fields"></a>

## Function `post_access_fields`

Single match site for [<code><a href="../social_contracts/post.md#social_contracts_post_PostAccess">PostAccess</a></code>] field extraction.


<pre><code><b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_post_access_fields">post_access_fields</a>(access: &<a href="../social_contracts/post.md#social_contracts_post_PostAccess">social_contracts::post::PostAccess</a>): (u8, <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;, <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;, <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_post_access_fields">post_access_fields</a>(access: &<a href="../social_contracts/post.md#social_contracts_post_PostAccess">PostAccess</a>): (u8, Option&lt;ID&gt;, Option&lt;ID&gt;, Option&lt;u64&gt;) {
    match (access) {
        PostAccess::Public =&gt; (<a href="../social_contracts/post.md#social_contracts_post_POST_ACCESS_PUBLIC">POST_ACCESS_PUBLIC</a>, option::none(), option::none(), option::none()),
        PostAccess::ProfileSubscription { service_id, mydata_id, min_tier_level } =&gt; {
            (
                <a href="../social_contracts/post.md#social_contracts_post_POST_ACCESS_PROFILE_SUBSCRIPTION">POST_ACCESS_PROFILE_SUBSCRIPTION</a>,
                option::some(*service_id),
                *mydata_id,
                *min_tier_level,
            )
        },
        PostAccess::MarketplaceOneTime { mydata_id } =&gt; {
            (<a href="../social_contracts/post.md#social_contracts_post_POST_ACCESS_MARKETPLACE_ONE_TIME">POST_ACCESS_MARKETPLACE_ONE_TIME</a>, option::none(), option::some(*mydata_id), option::none())
        },
    }
}
</code></pre>



</details>

<a name="social_contracts_post_requires_profile_subscription"></a>

## Function `requires_profile_subscription`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_requires_profile_subscription">requires_profile_subscription</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_requires_profile_subscription">requires_profile_subscription</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>): bool {
    <b>let</b> (kind, _, _, _) = <a href="../social_contracts/post.md#social_contracts_post_post_access_fields">post_access_fields</a>(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.access);
    kind == <a href="../social_contracts/post.md#social_contracts_post_POST_ACCESS_PROFILE_SUBSCRIPTION">POST_ACCESS_PROFILE_SUBSCRIPTION</a>
}
</code></pre>



</details>

<a name="social_contracts_post_requires_marketplace_purchase"></a>

## Function `requires_marketplace_purchase`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_requires_marketplace_purchase">requires_marketplace_purchase</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_requires_marketplace_purchase">requires_marketplace_purchase</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>): bool {
    <b>let</b> (kind, _, _, _) = <a href="../social_contracts/post.md#social_contracts_post_post_access_fields">post_access_fields</a>(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.access);
    kind == <a href="../social_contracts/post.md#social_contracts_post_POST_ACCESS_MARKETPLACE_ONE_TIME">POST_ACCESS_MARKETPLACE_ONE_TIME</a>
}
</code></pre>



</details>

<a name="social_contracts_post_linked_mydata"></a>

## Function `linked_mydata`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_linked_mydata">linked_mydata</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_linked_mydata">linked_mydata</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>): Option&lt;ID&gt; {
    <b>let</b> (_, _, mydata_id, _) = <a href="../social_contracts/post.md#social_contracts_post_post_access_fields">post_access_fields</a>(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.access);
    mydata_id
}
</code></pre>



</details>

<a name="social_contracts_post_subscription_service"></a>

## Function `subscription_service`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_subscription_service">subscription_service</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_subscription_service">subscription_service</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>): Option&lt;ID&gt; {
    <b>let</b> (_, service_id, _, _) = <a href="../social_contracts/post.md#social_contracts_post_post_access_fields">post_access_fields</a>(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.access);
    service_id
}
</code></pre>



</details>

<a name="social_contracts_post_subscription_min_tier_level"></a>

## Function `subscription_min_tier_level`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_subscription_min_tier_level">subscription_min_tier_level</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_subscription_min_tier_level">subscription_min_tier_level</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>): Option&lt;u64&gt; {
    <b>let</b> (_, _, _, min_tier_level) = <a href="../social_contracts/post.md#social_contracts_post_post_access_fields">post_access_fields</a>(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.access);
    min_tier_level
}
</code></pre>



</details>

<a name="social_contracts_post_post_access_kind"></a>

## Function `post_access_kind`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_post_access_kind">post_access_kind</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_post_access_kind">post_access_kind</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>): u8 {
    <b>let</b> (kind, _, _, _) = <a href="../social_contracts/post.md#social_contracts_post_post_access_fields">post_access_fields</a>(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.access);
    kind
}
</code></pre>



</details>

<a name="social_contracts_post_post_access_from_parts"></a>

## Function `post_access_from_parts`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_post_access_from_parts">post_access_from_parts</a>(access_kind: u8, subscription_service_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;, linked_mydata_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;, <a href="../social_contracts/post.md#social_contracts_post_subscription_min_tier_level">subscription_min_tier_level</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;): <a href="../social_contracts/post.md#social_contracts_post_PostAccess">social_contracts::post::PostAccess</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_post_access_from_parts">post_access_from_parts</a>(
    access_kind: u8,
    subscription_service_id: Option&lt;ID&gt;,
    linked_mydata_id: Option&lt;ID&gt;,
    <a href="../social_contracts/post.md#social_contracts_post_subscription_min_tier_level">subscription_min_tier_level</a>: Option&lt;u64&gt;,
): <a href="../social_contracts/post.md#social_contracts_post_PostAccess">PostAccess</a> {
    <b>if</b> (access_kind == <a href="../social_contracts/post.md#social_contracts_post_POST_ACCESS_PUBLIC">POST_ACCESS_PUBLIC</a>) {
        PostAccess::Public
    } <b>else</b> <b>if</b> (access_kind == <a href="../social_contracts/post.md#social_contracts_post_POST_ACCESS_PROFILE_SUBSCRIPTION">POST_ACCESS_PROFILE_SUBSCRIPTION</a>) {
        <b>assert</b>!(option::is_some(&subscription_service_id), <a href="../social_contracts/post.md#social_contracts_post_ENoSubscriptionService">ENoSubscriptionService</a>);
        PostAccess::ProfileSubscription {
            service_id: *option::borrow(&subscription_service_id),
            mydata_id: linked_mydata_id,
            min_tier_level: <a href="../social_contracts/post.md#social_contracts_post_subscription_min_tier_level">subscription_min_tier_level</a>,
        }
    } <b>else</b> <b>if</b> (access_kind == <a href="../social_contracts/post.md#social_contracts_post_POST_ACCESS_MARKETPLACE_ONE_TIME">POST_ACCESS_MARKETPLACE_ONE_TIME</a>) {
        <b>assert</b>!(option::is_some(&linked_mydata_id), <a href="../social_contracts/post.md#social_contracts_post_ENoEncryptedContent">ENoEncryptedContent</a>);
        PostAccess::MarketplaceOneTime {
            mydata_id: *option::borrow(&linked_mydata_id),
        }
    } <b>else</b> {
        <b>abort</b> <a href="../social_contracts/post.md#social_contracts_post_EUnauthorized">EUnauthorized</a>
    }
}
</code></pre>



</details>

<a name="social_contracts_post_allow_comments"></a>

## Function `allow_comments`

Query: check if comments are allowed


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_allow_comments">allow_comments</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_allow_comments">allow_comments</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>): bool {
    <a href="../social_contracts/post.md#social_contracts_post_has_flag">has_flag</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>.permissions, <a href="../social_contracts/post.md#social_contracts_post_PERMISSION_ALLOW_COMMENTS">PERMISSION_ALLOW_COMMENTS</a>)
}
</code></pre>



</details>

<a name="social_contracts_post_allow_reactions"></a>

## Function `allow_reactions`

Query: check if reactions are allowed


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_allow_reactions">allow_reactions</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_allow_reactions">allow_reactions</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>): bool {
    <a href="../social_contracts/post.md#social_contracts_post_has_flag">has_flag</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>.permissions, <a href="../social_contracts/post.md#social_contracts_post_PERMISSION_ALLOW_REACTIONS">PERMISSION_ALLOW_REACTIONS</a>)
}
</code></pre>



</details>

<a name="social_contracts_post_allow_reposts"></a>

## Function `allow_reposts`

Query: check if reposts are allowed


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_allow_reposts">allow_reposts</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_allow_reposts">allow_reposts</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>): bool {
    <a href="../social_contracts/post.md#social_contracts_post_has_flag">has_flag</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>.permissions, <a href="../social_contracts/post.md#social_contracts_post_PERMISSION_ALLOW_REPOSTS">PERMISSION_ALLOW_REPOSTS</a>)
}
</code></pre>



</details>

<a name="social_contracts_post_allow_quotes"></a>

## Function `allow_quotes`

Query: check if quotes are allowed


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_allow_quotes">allow_quotes</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_allow_quotes">allow_quotes</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>): bool {
    <a href="../social_contracts/post.md#social_contracts_post_has_flag">has_flag</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>.permissions, <a href="../social_contracts/post.md#social_contracts_post_PERMISSION_ALLOW_QUOTES">PERMISSION_ALLOW_QUOTES</a>)
}
</code></pre>



</details>

<a name="social_contracts_post_allow_tips"></a>

## Function `allow_tips`

Query: check if tips are allowed


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_allow_tips">allow_tips</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_allow_tips">allow_tips</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>): bool {
    <a href="../social_contracts/post.md#social_contracts_post_has_flag">has_flag</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>.permissions, <a href="../social_contracts/post.md#social_contracts_post_PERMISSION_ALLOW_TIPS">PERMISSION_ALLOW_TIPS</a>)
}
</code></pre>



</details>

<a name="social_contracts_post_is_spt_enabled"></a>

## Function `is_spt_enabled`

Query: check if SPT is enabled for this post


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_is_spt_enabled">is_spt_enabled</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_is_spt_enabled">is_spt_enabled</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>): bool {
    <a href="../social_contracts/post.md#social_contracts_post">post</a>.enable_spt
}
</code></pre>



</details>

<a name="social_contracts_post_composition_status"></a>

## Function `composition_status`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_composition_status">composition_status</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_composition_status">composition_status</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>): u8 {
    <a href="../social_contracts/post.md#social_contracts_post">post</a>.<a href="../social_contracts/post.md#social_contracts_post_composition_status">composition_status</a>
}
</code></pre>



</details>

<a name="social_contracts_post_monetization_status"></a>

## Function `monetization_status`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_monetization_status">monetization_status</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_monetization_status">monetization_status</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>): u8 {
    <a href="../social_contracts/post.md#social_contracts_post">post</a>.<a href="../social_contracts/post.md#social_contracts_post_monetization_status">monetization_status</a>
}
</code></pre>



</details>

<a name="social_contracts_post_media_asset_ids"></a>

## Function `media_asset_ids`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_media_asset_ids">media_asset_ids</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>): &vector&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_media_asset_ids">media_asset_ids</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>): &vector&lt;ID&gt; {
    &<a href="../social_contracts/post.md#social_contracts_post">post</a>.<a href="../social_contracts/post.md#social_contracts_post_media_asset_ids">media_asset_ids</a>
}
</code></pre>



</details>

<a name="social_contracts_post_composition_badge"></a>

## Function `composition_badge`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_composition_badge">composition_badge</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>): &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../social_contracts/media_asset.md#social_contracts_media_asset_CompositionBadgeSnapshot">social_contracts::media_asset::CompositionBadgeSnapshot</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_composition_badge">composition_badge</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>): &Option&lt;CompositionBadgeSnapshot&gt; {
    &<a href="../social_contracts/post.md#social_contracts_post">post</a>.<a href="../social_contracts/post.md#social_contracts_post_composition_badge">composition_badge</a>
}
</code></pre>



</details>

<a name="social_contracts_post_composition_analysis"></a>

## Function `composition_analysis`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_composition_analysis">composition_analysis</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../social_contracts/media_asset.md#social_contracts_media_asset_CompositionAnalysis">social_contracts::media_asset::CompositionAnalysis</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_composition_analysis">composition_analysis</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>): Option&lt;CompositionAnalysis&gt; {
    <b>if</b> (df::exists_with_type&lt;vector&lt;u8&gt;, CompositionAnalysis&gt;(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id, <a href="../social_contracts/post.md#social_contracts_post_COMPOSITION_ANALYSIS_DF_KEY">COMPOSITION_ANALYSIS_DF_KEY</a>)) {
        option::some(*df::borrow(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id, <a href="../social_contracts/post.md#social_contracts_post_COMPOSITION_ANALYSIS_DF_KEY">COMPOSITION_ANALYSIS_DF_KEY</a>))
    } <b>else</b> {
        option::none()
    }
}
</code></pre>



</details>

<a name="social_contracts_post_revenue_manifest"></a>

## Function `revenue_manifest`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_revenue_manifest">revenue_manifest</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../social_contracts/media_asset.md#social_contracts_media_asset_RevenueManifest">social_contracts::media_asset::RevenueManifest</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_revenue_manifest">revenue_manifest</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>): Option&lt;RevenueManifest&gt; {
    <b>if</b> (df::exists_with_type&lt;vector&lt;u8&gt;, RevenueManifest&gt;(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id, <a href="../social_contracts/post.md#social_contracts_post_REVENUE_MANIFEST_DF_KEY">REVENUE_MANIFEST_DF_KEY</a>)) {
        option::some(*df::borrow(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id, <a href="../social_contracts/post.md#social_contracts_post_REVENUE_MANIFEST_DF_KEY">REVENUE_MANIFEST_DF_KEY</a>))
    } <b>else</b> {
        option::none()
    }
}
</code></pre>



</details>

<a name="social_contracts_post_composition_disputes_submitted"></a>

## Function `composition_disputes_submitted`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_composition_disputes_submitted">composition_disputes_submitted</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_composition_disputes_submitted">composition_disputes_submitted</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>): u8 {
    <b>if</b> (df::exists_with_type&lt;vector&lt;u8&gt;, u8&gt;(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id, <a href="../social_contracts/post.md#social_contracts_post_COMPOSITION_DISPUTES_DF_KEY">COMPOSITION_DISPUTES_DF_KEY</a>)) {
        *df::borrow(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id, <a href="../social_contracts/post.md#social_contracts_post_COMPOSITION_DISPUTES_DF_KEY">COMPOSITION_DISPUTES_DF_KEY</a>)
    } <b>else</b> {
        0
    }
}
</code></pre>



</details>

<a name="social_contracts_post_monetization_enabled"></a>

## Function `monetization_enabled`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_monetization_enabled">monetization_enabled</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_monetization_enabled">monetization_enabled</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>): bool {
    <a href="../social_contracts/post.md#social_contracts_post">post</a>.<a href="../social_contracts/post.md#social_contracts_post_monetization_status">monetization_status</a> == <a href="../social_contracts/media_asset.md#social_contracts_media_asset_monetization_enabled">media_asset::monetization_enabled</a>()
}
</code></pre>



</details>

<a name="social_contracts_post_poc_disputes_submitted"></a>

## Function `poc_disputes_submitted`

Legacy alias used by dispute paths during migration.


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_poc_disputes_submitted">poc_disputes_submitted</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_poc_disputes_submitted">poc_disputes_submitted</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>): u8 {
    <a href="../social_contracts/post.md#social_contracts_post_composition_disputes_submitted">composition_disputes_submitted</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>)
}
</code></pre>



</details>

<a name="social_contracts_post_has_composition_monetization"></a>

## Function `has_composition_monetization`

Returns true when manifest-based revenue routing is active.


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_has_composition_monetization">has_composition_monetization</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_has_composition_monetization">has_composition_monetization</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>): bool {
    <a href="../social_contracts/post.md#social_contracts_post_monetization_enabled">monetization_enabled</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>) && option::is_some(&<a href="../social_contracts/post.md#social_contracts_post_revenue_manifest">revenue_manifest</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>))
}
</code></pre>



</details>

<a name="social_contracts_post_tip_post_requires_beneficiary_vault_for_amount"></a>

## Function `tip_post_requires_beneficiary_vault_for_amount`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_tip_post_requires_beneficiary_vault_for_amount">tip_post_requires_beneficiary_vault_for_amount</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, tip_amount: u64): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_tip_post_requires_beneficiary_vault_for_amount">tip_post_requires_beneficiary_vault_for_amount</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>, tip_amount: u64): bool {
    <b>if</b> (tip_amount == 0 || !<a href="../social_contracts/post.md#social_contracts_post_monetization_enabled">monetization_enabled</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>)) {
        <b>return</b> <b>false</b>
    };
    <b>let</b> manifest_opt = <a href="../social_contracts/post.md#social_contracts_post_revenue_manifest">revenue_manifest</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>);
    <b>if</b> (option::is_none(&manifest_opt)) {
        <b>return</b> <b>false</b>
    };
    <b>let</b> manifest = option::borrow(&manifest_opt);
    <b>let</b> entries = <a href="../social_contracts/media_asset.md#social_contracts_media_asset_manifest_entries">media_asset::manifest_entries</a>(manifest);
    <b>let</b> len = vector::length(entries);
    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (i &lt; len) {
        <b>let</b> e = vector::borrow(entries, i);
        <b>if</b> (<a href="../social_contracts/media_asset.md#social_contracts_media_asset_manifest_entry_payout_mode">media_asset::manifest_entry_payout_mode</a>(e) == <a href="../social_contracts/media_asset.md#social_contracts_media_asset_payout_escrow">media_asset::payout_escrow</a>()) {
            <b>let</b> slice = (tip_amount * <a href="../social_contracts/media_asset.md#social_contracts_media_asset_manifest_entry_share_bps">media_asset::manifest_entry_share_bps</a>(e)) / <a href="../social_contracts/media_asset.md#social_contracts_media_asset_manifest_bps_total">media_asset::manifest_bps_total</a>();
            <b>if</b> (slice &gt; 0) {
                <b>return</b> <b>true</b>
            };
        };
        i = i + 1;
    };
    <b>false</b>
}
</code></pre>



</details>

<a name="social_contracts_post_actor_address"></a>

## Function `actor_address`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_actor_address">actor_address</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_actor_address">actor_address</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>): <b>address</b> {
    <a href="../social_contracts/post.md#social_contracts_post_post_attribution">post_attribution</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>).<a href="../social_contracts/post.md#social_contracts_post_actor_address">actor_address</a>
}
</code></pre>



</details>

<a name="social_contracts_post_sub_agent_id"></a>

## Function `sub_agent_id`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_sub_agent_id">sub_agent_id</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_sub_agent_id">sub_agent_id</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>): Option&lt;ID&gt; {
    <a href="../social_contracts/post.md#social_contracts_post_post_attribution">post_attribution</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>).<a href="../social_contracts/post.md#social_contracts_post_sub_agent_id">sub_agent_id</a>
}
</code></pre>



</details>

<a name="social_contracts_post_action_identity_class"></a>

## Function `action_identity_class`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_action_identity_class">action_identity_class</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_action_identity_class">action_identity_class</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>): u8 {
    <a href="../social_contracts/post.md#social_contracts_post_post_attribution">post_attribution</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>).<a href="../social_contracts/post.md#social_contracts_post_action_identity_class">action_identity_class</a>
}
</code></pre>



</details>

<a name="social_contracts_post_comment_actor_address"></a>

## Function `comment_actor_address`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_comment_actor_address">comment_actor_address</a>(comment: &<a href="../social_contracts/post.md#social_contracts_post_Comment">social_contracts::post::Comment</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_comment_actor_address">comment_actor_address</a>(comment: &<a href="../social_contracts/post.md#social_contracts_post_Comment">Comment</a>): <b>address</b> {
    <a href="../social_contracts/post.md#social_contracts_post_comment_attribution">comment_attribution</a>(comment).<a href="../social_contracts/post.md#social_contracts_post_actor_address">actor_address</a>
}
</code></pre>



</details>

<a name="social_contracts_post_comment_sub_agent_id"></a>

## Function `comment_sub_agent_id`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_comment_sub_agent_id">comment_sub_agent_id</a>(comment: &<a href="../social_contracts/post.md#social_contracts_post_Comment">social_contracts::post::Comment</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_comment_sub_agent_id">comment_sub_agent_id</a>(comment: &<a href="../social_contracts/post.md#social_contracts_post_Comment">Comment</a>): Option&lt;ID&gt; {
    <a href="../social_contracts/post.md#social_contracts_post_comment_attribution">comment_attribution</a>(comment).<a href="../social_contracts/post.md#social_contracts_post_sub_agent_id">sub_agent_id</a>
}
</code></pre>



</details>

<a name="social_contracts_post_comment_action_identity_class"></a>

## Function `comment_action_identity_class`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_comment_action_identity_class">comment_action_identity_class</a>(comment: &<a href="../social_contracts/post.md#social_contracts_post_Comment">social_contracts::post::Comment</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_comment_action_identity_class">comment_action_identity_class</a>(comment: &<a href="../social_contracts/post.md#social_contracts_post_Comment">Comment</a>): u8 {
    <a href="../social_contracts/post.md#social_contracts_post_comment_attribution">comment_attribution</a>(comment).<a href="../social_contracts/post.md#social_contracts_post_action_identity_class">action_identity_class</a>
}
</code></pre>



</details>

<a name="social_contracts_post_post_attribution"></a>

## Function `post_attribution`



<pre><code><b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_post_attribution">post_attribution</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>): <a href="../social_contracts/post.md#social_contracts_post_PostAttribution">social_contracts::post::PostAttribution</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_post_attribution">post_attribution</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>): <a href="../social_contracts/post.md#social_contracts_post_PostAttribution">PostAttribution</a> {
    *df::borrow&lt;vector&lt;u8&gt;, <a href="../social_contracts/post.md#social_contracts_post_PostAttribution">PostAttribution</a>&gt;(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id, <a href="../social_contracts/post.md#social_contracts_post_POST_ATTRIBUTION_DF_KEY">POST_ATTRIBUTION_DF_KEY</a>)
}
</code></pre>



</details>

<a name="social_contracts_post_attach_post_attribution"></a>

## Function `attach_post_attribution`



<pre><code><b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_attach_post_attribution">attach_post_attribution</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, <a href="../social_contracts/post.md#social_contracts_post_actor_address">actor_address</a>: <b>address</b>, <a href="../social_contracts/post.md#social_contracts_post_sub_agent_id">sub_agent_id</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;, organization_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;, <a href="../social_contracts/post.md#social_contracts_post_action_identity_class">action_identity_class</a>: u8)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_attach_post_attribution">attach_post_attribution</a>(
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>,
    <a href="../social_contracts/post.md#social_contracts_post_actor_address">actor_address</a>: <b>address</b>,
    <a href="../social_contracts/post.md#social_contracts_post_sub_agent_id">sub_agent_id</a>: Option&lt;ID&gt;,
    organization_id: Option&lt;ID&gt;,
    <a href="../social_contracts/post.md#social_contracts_post_action_identity_class">action_identity_class</a>: u8,
) {
    df::add(
        &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post">post</a>.id,
        <a href="../social_contracts/post.md#social_contracts_post_POST_ATTRIBUTION_DF_KEY">POST_ATTRIBUTION_DF_KEY</a>,
        <a href="../social_contracts/post.md#social_contracts_post_PostAttribution">PostAttribution</a> {
            <a href="../social_contracts/post.md#social_contracts_post_actor_address">actor_address</a>,
            <a href="../social_contracts/post.md#social_contracts_post_sub_agent_id">sub_agent_id</a>,
            organization_id,
            <a href="../social_contracts/post.md#social_contracts_post_action_identity_class">action_identity_class</a>,
        },
    );
}
</code></pre>



</details>

<a name="social_contracts_post_comment_attribution"></a>

## Function `comment_attribution`



<pre><code><b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_comment_attribution">comment_attribution</a>(comment: &<a href="../social_contracts/post.md#social_contracts_post_Comment">social_contracts::post::Comment</a>): <a href="../social_contracts/post.md#social_contracts_post_CommentAttribution">social_contracts::post::CommentAttribution</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_comment_attribution">comment_attribution</a>(comment: &<a href="../social_contracts/post.md#social_contracts_post_Comment">Comment</a>): <a href="../social_contracts/post.md#social_contracts_post_CommentAttribution">CommentAttribution</a> {
    *df::borrow&lt;vector&lt;u8&gt;, <a href="../social_contracts/post.md#social_contracts_post_CommentAttribution">CommentAttribution</a>&gt;(&comment.id, <a href="../social_contracts/post.md#social_contracts_post_COMMENT_ATTRIBUTION_DF_KEY">COMMENT_ATTRIBUTION_DF_KEY</a>)
}
</code></pre>



</details>

<a name="social_contracts_post_attach_comment_attribution"></a>

## Function `attach_comment_attribution`



<pre><code><b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_attach_comment_attribution">attach_comment_attribution</a>(comment: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Comment">social_contracts::post::Comment</a>, <a href="../social_contracts/post.md#social_contracts_post_actor_address">actor_address</a>: <b>address</b>, <a href="../social_contracts/post.md#social_contracts_post_sub_agent_id">sub_agent_id</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;, organization_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;, <a href="../social_contracts/post.md#social_contracts_post_action_identity_class">action_identity_class</a>: u8)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_attach_comment_attribution">attach_comment_attribution</a>(
    comment: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Comment">Comment</a>,
    <a href="../social_contracts/post.md#social_contracts_post_actor_address">actor_address</a>: <b>address</b>,
    <a href="../social_contracts/post.md#social_contracts_post_sub_agent_id">sub_agent_id</a>: Option&lt;ID&gt;,
    organization_id: Option&lt;ID&gt;,
    <a href="../social_contracts/post.md#social_contracts_post_action_identity_class">action_identity_class</a>: u8,
) {
    df::add(
        &<b>mut</b> comment.id,
        <a href="../social_contracts/post.md#social_contracts_post_COMMENT_ATTRIBUTION_DF_KEY">COMMENT_ATTRIBUTION_DF_KEY</a>,
        <a href="../social_contracts/post.md#social_contracts_post_CommentAttribution">CommentAttribution</a> {
            <a href="../social_contracts/post.md#social_contracts_post_actor_address">actor_address</a>,
            <a href="../social_contracts/post.md#social_contracts_post_sub_agent_id">sub_agent_id</a>,
            organization_id,
            <a href="../social_contracts/post.md#social_contracts_post_action_identity_class">action_identity_class</a>,
        },
    );
}
</code></pre>



</details>

<a name="social_contracts_post_get_spt_id"></a>

## Function `get_spt_id`

Get the SPT pool ID for a post


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_get_spt_id">get_spt_id</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>): &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_get_spt_id">get_spt_id</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>): &Option&lt;<b>address</b>&gt; {
    &<a href="../social_contracts/post.md#social_contracts_post">post</a>.spt_id
}
</code></pre>



</details>

<a name="social_contracts_post_set_spt_id"></a>

## Function `set_spt_id`

Internal function to set SPT pool ID (package visibility only)


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_set_spt_id">set_spt_id</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, spt_id: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_set_spt_id">set_spt_id</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>, spt_id: <b>address</b>) {
    <a href="../social_contracts/post.md#social_contracts_post">post</a>.spt_id = option::some(spt_id);
}
</code></pre>



</details>

<a name="social_contracts_post_set_enable_spt"></a>

## Function `set_enable_spt`

Package helper to flip the SPT opt-in flag (late-enable / create-with-SPT paths).


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_set_enable_spt">set_enable_spt</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, enabled: bool)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_set_enable_spt">set_enable_spt</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>, enabled: bool) {
    <a href="../social_contracts/post.md#social_contracts_post">post</a>.enable_spt = enabled;
}
</code></pre>



</details>

<a name="social_contracts_post_spot_status_pending"></a>

## Function `spot_status_pending`

SPoT analysis status sentinels for cross-module use.


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_spot_status_pending">spot_status_pending</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_spot_status_pending">spot_status_pending</a>(): u8 { <a href="../social_contracts/post.md#social_contracts_post_SPOT_STATUS_PENDING">SPOT_STATUS_PENDING</a> }
</code></pre>



</details>

<a name="social_contracts_post_spot_status_completed"></a>

## Function `spot_status_completed`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_spot_status_completed">spot_status_completed</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_spot_status_completed">spot_status_completed</a>(): u8 { <a href="../social_contracts/post.md#social_contracts_post_SPOT_STATUS_COMPLETED">SPOT_STATUS_COMPLETED</a> }
</code></pre>



</details>

<a name="social_contracts_post_spot_status_completed_no_actionable"></a>

## Function `spot_status_completed_no_actionable`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_spot_status_completed_no_actionable">spot_status_completed_no_actionable</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_spot_status_completed_no_actionable">spot_status_completed_no_actionable</a>(): u8 { <a href="../social_contracts/post.md#social_contracts_post_SPOT_STATUS_COMPLETED_NO_ACTIONABLE">SPOT_STATUS_COMPLETED_NO_ACTIONABLE</a> }
</code></pre>



</details>

<a name="social_contracts_post_has_spot_analysis"></a>

## Function `has_spot_analysis`

Whether an analysis record has been attached to this post yet.


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_has_spot_analysis">has_spot_analysis</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_has_spot_analysis">has_spot_analysis</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>): bool {
    df::exists_with_type&lt;vector&lt;u8&gt;, <a href="../social_contracts/post.md#social_contracts_post_PostSpotAnalysis">PostSpotAnalysis</a>&gt;(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id, <a href="../social_contracts/post.md#social_contracts_post_SPOT_ANALYSIS_DF_KEY">SPOT_ANALYSIS_DF_KEY</a>)
}
</code></pre>



</details>

<a name="social_contracts_post_spot_analysis_status"></a>

## Function `spot_analysis_status`

Analysis status; <code>pending</code> (0) when no analysis has been attached.


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_spot_analysis_status">spot_analysis_status</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_spot_analysis_status">spot_analysis_status</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>): u8 {
    <b>if</b> (<a href="../social_contracts/post.md#social_contracts_post_has_spot_analysis">has_spot_analysis</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>)) {
        df::borrow&lt;vector&lt;u8&gt;, <a href="../social_contracts/post.md#social_contracts_post_PostSpotAnalysis">PostSpotAnalysis</a>&gt;(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id, <a href="../social_contracts/post.md#social_contracts_post_SPOT_ANALYSIS_DF_KEY">SPOT_ANALYSIS_DF_KEY</a>).status
    } <b>else</b> {
        <a href="../social_contracts/post.md#social_contracts_post_SPOT_STATUS_PENDING">SPOT_STATUS_PENDING</a>
    }
}
</code></pre>



</details>

<a name="social_contracts_post_spot_analysis_market_ids"></a>

## Function `spot_analysis_market_ids`

Future-linked market object ids in claim_index order (empty when none/pending).


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_spot_analysis_market_ids">spot_analysis_market_ids</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>): vector&lt;<b>address</b>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_spot_analysis_market_ids">spot_analysis_market_ids</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>): vector&lt;<b>address</b>&gt; {
    <b>if</b> (<a href="../social_contracts/post.md#social_contracts_post_has_spot_analysis">has_spot_analysis</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>)) {
        df::borrow&lt;vector&lt;u8&gt;, <a href="../social_contracts/post.md#social_contracts_post_PostSpotAnalysis">PostSpotAnalysis</a>&gt;(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id, <a href="../social_contracts/post.md#social_contracts_post_SPOT_ANALYSIS_DF_KEY">SPOT_ANALYSIS_DF_KEY</a>).market_ids
    } <b>else</b> {
        vector::empty()
    }
}
</code></pre>



</details>

<a name="social_contracts_post_spot_analysis_claim_indexes"></a>

## Function `spot_analysis_claim_indexes`

Future-linked claim indexes in ascending order (empty when none/pending).


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_spot_analysis_claim_indexes">spot_analysis_claim_indexes</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>): vector&lt;u64&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_spot_analysis_claim_indexes">spot_analysis_claim_indexes</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>): vector&lt;u64&gt; {
    <b>if</b> (<a href="../social_contracts/post.md#social_contracts_post_has_spot_analysis">has_spot_analysis</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>)) {
        df::borrow&lt;vector&lt;u8&gt;, <a href="../social_contracts/post.md#social_contracts_post_PostSpotAnalysis">PostSpotAnalysis</a>&gt;(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id, <a href="../social_contracts/post.md#social_contracts_post_SPOT_ANALYSIS_DF_KEY">SPOT_ANALYSIS_DF_KEY</a>).claim_indexes
    } <b>else</b> {
        vector::empty()
    }
}
</code></pre>



</details>

<a name="social_contracts_post_spot_analysis_claim_ids"></a>

## Function `spot_analysis_claim_ids`

Future-linked claim object ids in claim_index order (empty when none/pending).


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_spot_analysis_claim_ids">spot_analysis_claim_ids</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>): vector&lt;<b>address</b>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_spot_analysis_claim_ids">spot_analysis_claim_ids</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>): vector&lt;<b>address</b>&gt; {
    <b>if</b> (<a href="../social_contracts/post.md#social_contracts_post_has_spot_analysis">has_spot_analysis</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>)) {
        df::borrow&lt;vector&lt;u8&gt;, <a href="../social_contracts/post.md#social_contracts_post_PostSpotAnalysis">PostSpotAnalysis</a>&gt;(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id, <a href="../social_contracts/post.md#social_contracts_post_SPOT_ANALYSIS_DF_KEY">SPOT_ANALYSIS_DF_KEY</a>).claim_ids
    } <b>else</b> {
        vector::empty()
    }
}
</code></pre>



</details>

<a name="social_contracts_post_spot_analysis_future_accepted_count"></a>

## Function `spot_analysis_future_accepted_count`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_spot_analysis_future_accepted_count">spot_analysis_future_accepted_count</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_spot_analysis_future_accepted_count">spot_analysis_future_accepted_count</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>): u64 {
    <b>if</b> (<a href="../social_contracts/post.md#social_contracts_post_has_spot_analysis">has_spot_analysis</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>)) {
        df::borrow&lt;vector&lt;u8&gt;, <a href="../social_contracts/post.md#social_contracts_post_PostSpotAnalysis">PostSpotAnalysis</a>&gt;(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id, <a href="../social_contracts/post.md#social_contracts_post_SPOT_ANALYSIS_DF_KEY">SPOT_ANALYSIS_DF_KEY</a>).future_accepted_count
    } <b>else</b> { 0 }
}
</code></pre>



</details>

<a name="social_contracts_post_ensure_spot_analysis_pending"></a>

## Function `ensure_spot_analysis_pending`

Ensure a <code>pending</code> analysis exists so future links can accumulate before finalize.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_ensure_spot_analysis_pending">ensure_spot_analysis_pending</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, max_claim_per_post_applied: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_ensure_spot_analysis_pending">ensure_spot_analysis_pending</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>, max_claim_per_post_applied: u64) {
    <b>if</b> (!<a href="../social_contracts/post.md#social_contracts_post_has_spot_analysis">has_spot_analysis</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>)) {
        df::add(&<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post">post</a>.id, <a href="../social_contracts/post.md#social_contracts_post_SPOT_ANALYSIS_DF_KEY">SPOT_ANALYSIS_DF_KEY</a>, <a href="../social_contracts/post.md#social_contracts_post_PostSpotAnalysis">PostSpotAnalysis</a> {
            status: <a href="../social_contracts/post.md#social_contracts_post_SPOT_STATUS_PENDING">SPOT_STATUS_PENDING</a>,
            detected_claim_count: 0,
            rejected_claim_count: 0,
            truncated_claim_count: 0,
            future_accepted_count: 0,
            past_verified_count: 0,
            max_claim_per_post_applied,
            claim_indexes: vector::empty(),
            claim_ids: vector::empty(),
            market_ids: vector::empty(),
            policy_hashes: vector::empty(),
            claim_manifest_hash: option::none(),
            veracity_manifest_hash: option::none(),
        });
    }
}
</code></pre>



</details>

<a name="social_contracts_post_spot_analysis_append_future"></a>

## Function `spot_analysis_append_future`

Append one future-claim link (must be pending, strictly increasing claim_index, unique market).


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_spot_analysis_append_future">spot_analysis_append_future</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, claim_index: u64, claim_id: <b>address</b>, market_id: <b>address</b>, policy_hash: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_spot_analysis_append_future">spot_analysis_append_future</a>(
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>,
    claim_index: u64,
    claim_id: <b>address</b>,
    market_id: <b>address</b>,
    policy_hash: vector&lt;u8&gt;,
) {
    <a href="../social_contracts/post.md#social_contracts_post_ensure_spot_analysis_pending">ensure_spot_analysis_pending</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>, 0);
    <b>let</b> a = df::borrow_mut&lt;vector&lt;u8&gt;, <a href="../social_contracts/post.md#social_contracts_post_PostSpotAnalysis">PostSpotAnalysis</a>&gt;(&<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post">post</a>.id, <a href="../social_contracts/post.md#social_contracts_post_SPOT_ANALYSIS_DF_KEY">SPOT_ANALYSIS_DF_KEY</a>);
    <b>assert</b>!(a.status == <a href="../social_contracts/post.md#social_contracts_post_SPOT_STATUS_PENDING">SPOT_STATUS_PENDING</a>, <a href="../social_contracts/post.md#social_contracts_post_ESpotAnalysisNotPending">ESpotAnalysisNotPending</a>);
    <b>let</b> n = vector::length(&a.claim_indexes);
    <b>if</b> (n &gt; 0) {
        <b>assert</b>!(claim_index &gt; *vector::borrow(&a.claim_indexes, n - 1), <a href="../social_contracts/post.md#social_contracts_post_ESpotAnalysisIndexOrder">ESpotAnalysisIndexOrder</a>);
    };
    <b>assert</b>!(!vector::contains(&a.market_ids, &market_id), <a href="../social_contracts/post.md#social_contracts_post_ESpotAnalysisDuplicateMarket">ESpotAnalysisDuplicateMarket</a>);
    vector::push_back(&<b>mut</b> a.claim_indexes, claim_index);
    vector::push_back(&<b>mut</b> a.claim_ids, claim_id);
    vector::push_back(&<b>mut</b> a.market_ids, market_id);
    vector::push_back(&<b>mut</b> a.policy_hashes, policy_hash);
    a.future_accepted_count = a.future_accepted_count + 1;
}
</code></pre>



</details>

<a name="social_contracts_post_finalize_spot_analysis"></a>

## Function `finalize_spot_analysis`

Finalize analysis: validate future vectors, set terminal status, counts and manifests.
Aborts unless currently pending. Called by the SPoT batch-finalize entry.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_finalize_spot_analysis">finalize_spot_analysis</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, status: u8, detected_claim_count: u64, rejected_claim_count: u64, truncated_claim_count: u64, past_verified_count: u64, max_claim_per_post_applied: u64, claim_manifest_hash: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;u8&gt;&gt;, veracity_manifest_hash: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;u8&gt;&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_finalize_spot_analysis">finalize_spot_analysis</a>(
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>,
    status: u8,
    detected_claim_count: u64,
    rejected_claim_count: u64,
    truncated_claim_count: u64,
    past_verified_count: u64,
    max_claim_per_post_applied: u64,
    claim_manifest_hash: Option&lt;vector&lt;u8&gt;&gt;,
    veracity_manifest_hash: Option&lt;vector&lt;u8&gt;&gt;,
) {
    <a href="../social_contracts/post.md#social_contracts_post_ensure_spot_analysis_pending">ensure_spot_analysis_pending</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>, max_claim_per_post_applied);
    <b>let</b> a = df::borrow_mut&lt;vector&lt;u8&gt;, <a href="../social_contracts/post.md#social_contracts_post_PostSpotAnalysis">PostSpotAnalysis</a>&gt;(&<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post">post</a>.id, <a href="../social_contracts/post.md#social_contracts_post_SPOT_ANALYSIS_DF_KEY">SPOT_ANALYSIS_DF_KEY</a>);
    <b>assert</b>!(a.status == <a href="../social_contracts/post.md#social_contracts_post_SPOT_STATUS_PENDING">SPOT_STATUS_PENDING</a>, <a href="../social_contracts/post.md#social_contracts_post_ESpotAnalysisNotPending">ESpotAnalysisNotPending</a>);
    <b>let</b> future_len = vector::length(&a.claim_indexes);
    <b>assert</b>!(vector::length(&a.claim_ids) == future_len, <a href="../social_contracts/post.md#social_contracts_post_ESpotAnalysisVectorMismatch">ESpotAnalysisVectorMismatch</a>);
    <b>assert</b>!(vector::length(&a.market_ids) == future_len, <a href="../social_contracts/post.md#social_contracts_post_ESpotAnalysisVectorMismatch">ESpotAnalysisVectorMismatch</a>);
    <b>assert</b>!(vector::length(&a.policy_hashes) == future_len, <a href="../social_contracts/post.md#social_contracts_post_ESpotAnalysisVectorMismatch">ESpotAnalysisVectorMismatch</a>);
    <b>assert</b>!(a.future_accepted_count == future_len, <a href="../social_contracts/post.md#social_contracts_post_ESpotAnalysisVectorMismatch">ESpotAnalysisVectorMismatch</a>);
    <b>if</b> (max_claim_per_post_applied &gt; 0) {
        <b>assert</b>!(future_len + past_verified_count &lt;= max_claim_per_post_applied, <a href="../social_contracts/post.md#social_contracts_post_ESpotAnalysisOverCap">ESpotAnalysisOverCap</a>);
    };
    a.status = status;
    a.detected_claim_count = detected_claim_count;
    a.rejected_claim_count = rejected_claim_count;
    a.truncated_claim_count = truncated_claim_count;
    a.past_verified_count = past_verified_count;
    a.max_claim_per_post_applied = max_claim_per_post_applied;
    a.claim_manifest_hash = claim_manifest_hash;
    a.veracity_manifest_hash = veracity_manifest_hash;
}
</code></pre>



</details>

<a name="social_contracts_post_bootstrap_init"></a>

## Function `bootstrap_init`

Bootstrap initialization function - creates the post configuration


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_bootstrap_init">bootstrap_init</a>(clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_bootstrap_init">bootstrap_init</a>(clock: &Clock, ctx: &<b>mut</b> TxContext) {
    <b>let</b> admin = tx_context::sender(ctx);
    <b>let</b> config = <a href="../social_contracts/post.md#social_contracts_post_PostConfig">PostConfig</a> {
        id: object::new(ctx),
        max_content_length: <a href="../social_contracts/post.md#social_contracts_post_MAX_CONTENT_LENGTH">MAX_CONTENT_LENGTH</a>,
        max_media_urls: <a href="../social_contracts/post.md#social_contracts_post_MAX_MEDIA_URLS">MAX_MEDIA_URLS</a>,
        max_mentions: <a href="../social_contracts/post.md#social_contracts_post_MAX_MENTIONS">MAX_MENTIONS</a>,
        max_metadata_size: <a href="../social_contracts/post.md#social_contracts_post_MAX_METADATA_SIZE">MAX_METADATA_SIZE</a>,
        max_description_length: <a href="../social_contracts/post.md#social_contracts_post_MAX_DESCRIPTION_LENGTH">MAX_DESCRIPTION_LENGTH</a>,
        max_reaction_length: <a href="../social_contracts/post.md#social_contracts_post_MAX_REACTION_LENGTH">MAX_REACTION_LENGTH</a>,
        commenter_tip_percentage: <a href="../social_contracts/post.md#social_contracts_post_COMMENTER_TIP_PERCENTAGE">COMMENTER_TIP_PERCENTAGE</a>,
        repost_tip_percentage: <a href="../social_contracts/post.md#social_contracts_post_REPOST_TIP_PERCENTAGE">REPOST_TIP_PERCENTAGE</a>,
        min_promotion_amount: <a href="../social_contracts/post.md#social_contracts_post_MIN_PROMOTION_AMOUNT">MIN_PROMOTION_AMOUNT</a>,
        max_promotion_amount: <a href="../social_contracts/post.md#social_contracts_post_MAX_PROMOTION_AMOUNT">MAX_PROMOTION_AMOUNT</a>,
        min_view_duration_ms: <a href="../social_contracts/post.md#social_contracts_post_MIN_VIEW_DURATION">MIN_VIEW_DURATION</a>,
        platform_fee_bps: <a href="../social_contracts/post.md#social_contracts_post_DEFAULT_PLATFORM_FEE_BPS">DEFAULT_PLATFORM_FEE_BPS</a>,
        ecosystem_fee_bps: <a href="../social_contracts/post.md#social_contracts_post_DEFAULT_ECOSYSTEM_FEE_BPS">DEFAULT_ECOSYSTEM_FEE_BPS</a>,
        <a href="../social_contracts/post.md#social_contracts_post_version">version</a>: <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(),
    };
    // Emit event so indexer can populate post_config table
    event::emit(<a href="../social_contracts/post.md#social_contracts_post_PostParametersUpdatedEvent">PostParametersUpdatedEvent</a> {
        updated_by: admin,
        timestamp: clock::timestamp_ms(clock),
        max_content_length: <a href="../social_contracts/post.md#social_contracts_post_MAX_CONTENT_LENGTH">MAX_CONTENT_LENGTH</a>,
        max_media_urls: <a href="../social_contracts/post.md#social_contracts_post_MAX_MEDIA_URLS">MAX_MEDIA_URLS</a>,
        max_mentions: <a href="../social_contracts/post.md#social_contracts_post_MAX_MENTIONS">MAX_MENTIONS</a>,
        max_metadata_size: <a href="../social_contracts/post.md#social_contracts_post_MAX_METADATA_SIZE">MAX_METADATA_SIZE</a>,
        max_description_length: <a href="../social_contracts/post.md#social_contracts_post_MAX_DESCRIPTION_LENGTH">MAX_DESCRIPTION_LENGTH</a>,
        max_reaction_length: <a href="../social_contracts/post.md#social_contracts_post_MAX_REACTION_LENGTH">MAX_REACTION_LENGTH</a>,
        commenter_tip_percentage: <a href="../social_contracts/post.md#social_contracts_post_COMMENTER_TIP_PERCENTAGE">COMMENTER_TIP_PERCENTAGE</a>,
        repost_tip_percentage: <a href="../social_contracts/post.md#social_contracts_post_REPOST_TIP_PERCENTAGE">REPOST_TIP_PERCENTAGE</a>,
        min_promotion_amount: <a href="../social_contracts/post.md#social_contracts_post_MIN_PROMOTION_AMOUNT">MIN_PROMOTION_AMOUNT</a>,
        max_promotion_amount: <a href="../social_contracts/post.md#social_contracts_post_MAX_PROMOTION_AMOUNT">MAX_PROMOTION_AMOUNT</a>,
        min_view_duration_ms: <a href="../social_contracts/post.md#social_contracts_post_MIN_VIEW_DURATION">MIN_VIEW_DURATION</a>,
        platform_fee_bps: <a href="../social_contracts/post.md#social_contracts_post_DEFAULT_PLATFORM_FEE_BPS">DEFAULT_PLATFORM_FEE_BPS</a>,
        ecosystem_fee_bps: <a href="../social_contracts/post.md#social_contracts_post_DEFAULT_ECOSYSTEM_FEE_BPS">DEFAULT_ECOSYSTEM_FEE_BPS</a>,
    });
    // Create and share <a href="../social_contracts/post.md#social_contracts_post">post</a> configuration
    transfer::share_object(config);
}
</code></pre>



</details>

<a name="social_contracts_post_convert_urls_to_strings"></a>

## Function `convert_urls_to_strings`

Convert Option<vector<Url>> to Option<vector<String>> for events


<pre><code><b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_convert_urls_to_strings">convert_urls_to_strings</a>(media_option: &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;<a href="../myso/url.md#myso_url_Url">myso::url::Url</a>&gt;&gt;): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_convert_urls_to_strings">convert_urls_to_strings</a>(media_option: &Option&lt;vector&lt;Url&gt;&gt;): Option&lt;vector&lt;String&gt;&gt; {
    <b>if</b> (option::is_some(media_option)) {
        <b>let</b> urls = option::borrow(media_option);
        <b>let</b> <b>mut</b> url_strings = vector::empty&lt;String&gt;();
        <b>let</b> len = vector::length(urls);
        <b>let</b> <b>mut</b> i = 0;
        <b>while</b> (i &lt; len) {
            <b>let</b> url = vector::borrow(urls, i);
            <b>let</b> url_string = string::from_ascii(url::inner_url(url));
            vector::push_back(&<b>mut</b> url_strings, url_string);
            i = i + 1;
        };
        option::some(url_strings)
    } <b>else</b> {
        option::none()
    }
}
</code></pre>



</details>

<a name="social_contracts_post_resolve_social_actor"></a>

## Function `resolve_social_actor`

Resolve social actor: capability, principal platform join, block list, and approval gate.


<pre><code><b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_resolve_social_actor">resolve_social_actor</a>(memory_config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, registry: &<a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">social_contracts::profile::UsernameRegistry</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, memory_account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, required_cap: u64, spend_amount: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../social_contracts/memory.md#social_contracts_memory_ActingContext">social_contracts::memory::ActingContext</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_resolve_social_actor">resolve_social_actor</a>(
    memory_config: &MemoryConfig,
    registry: &UsernameRegistry,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">platform::Platform</a>,
    block_list_registry: &BlockListRegistry,
    memory_account: &MemoryAccount,
    required_cap: u64,
    spend_amount: u64,
    clock: &Clock,
    ctx: &TxContext,
): ActingContext {
    <b>let</b> platform_id = object::uid_to_address(<a href="../social_contracts/platform.md#social_contracts_platform_id">platform::id</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>));
    <b>let</b> acting = <a href="../social_contracts/memory.md#social_contracts_memory_resolve_actor_with_cap">memory::resolve_actor_with_cap</a>(
        memory_config,
        memory_account,
        required_cap,
        option::some(platform_id),
        spend_amount,
        clock,
        ctx,
    );
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_direct_execution_allowed">memory::assert_direct_execution_allowed</a>(memory_account, required_cap, ctx);
    <b>let</b> principal = <a href="../social_contracts/memory.md#social_contracts_memory_acting_principal_owner">memory::acting_principal_owner</a>(&acting);
    <b>assert</b>!(<a href="../social_contracts/memory.md#social_contracts_memory_owner">memory::owner</a>(memory_account) == principal, <a href="../social_contracts/post.md#social_contracts_post_EUnauthorized">EUnauthorized</a>);
    <b>let</b> profile_id = <a href="../social_contracts/memory.md#social_contracts_memory_acting_profile_id">memory::acting_profile_id</a>(&acting);
    <b>let</b> profile_id_option = <a href="../social_contracts/profile.md#social_contracts_profile_lookup_profile_by_owner">profile::lookup_profile_by_owner</a>(registry, principal);
    <b>assert</b>!(option::is_some(&profile_id_option), <a href="../social_contracts/post.md#social_contracts_post_EUnauthorized">EUnauthorized</a>);
    <b>assert</b>!(*option::borrow(&profile_id_option) == profile_id, <a href="../social_contracts/post.md#social_contracts_post_EUnauthorized">EUnauthorized</a>);
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform_has_joined_platform">platform::has_joined_platform</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, principal), <a href="../social_contracts/post.md#social_contracts_post_EUserNotJoinedPlatform">EUserNotJoinedPlatform</a>);
    <b>assert</b>!(
        !<a href="../social_contracts/block_list.md#social_contracts_block_list_is_blocked">block_list::is_blocked</a>(block_list_registry, platform_id, principal),
        <a href="../social_contracts/post.md#social_contracts_post_EUserBlockedByPlatform">EUserBlockedByPlatform</a>,
    );
    acting
}
</code></pre>



</details>

<a name="social_contracts_post_assert_tip_spend_limit"></a>

## Function `assert_tip_spend_limit`



<pre><code><b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_assert_tip_spend_limit">assert_tip_spend_limit</a>(memory_account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, amount: u64, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_assert_tip_spend_limit">assert_tip_spend_limit</a>(
    memory_account: &MemoryAccount,
    amount: u64,
    ctx: &TxContext,
) {
    <b>let</b> caller = tx_context::sender(ctx);
    <b>if</b> (<a href="../social_contracts/memory.md#social_contracts_memory_is_registered_agent">memory::is_registered_agent</a>(memory_account, caller)) {
        <a href="../social_contracts/memory.md#social_contracts_memory_assert_action_spend_limit">memory::assert_action_spend_limit</a>(memory_account, amount, ctx);
    };
}
</code></pre>



</details>

<a name="social_contracts_post_create_post_internal"></a>

## Function `create_post_internal`

Internal function to create a post object (not yet shared).


<pre><code><b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_create_post_internal">create_post_internal</a>(owner: <b>address</b>, profile_id: <b>address</b>, platform_id: <b>address</b>, content: <a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../social_contracts/post.md#social_contracts_post_media_asset_ids">media_asset_ids</a>: vector&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;, media_option: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;<a href="../myso/url.md#myso_url_Url">myso::url::Url</a>&gt;&gt;, mentions: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;<b>address</b>&gt;&gt;, metadata_json: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, post_type: <a href="../std/string.md#std_string_String">std::string::String</a>, parent_post_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;, <a href="../social_contracts/post.md#social_contracts_post_allow_comments">allow_comments</a>: bool, <a href="../social_contracts/post.md#social_contracts_post_allow_reactions">allow_reactions</a>: bool, <a href="../social_contracts/post.md#social_contracts_post_allow_reposts">allow_reposts</a>: bool, <a href="../social_contracts/post.md#social_contracts_post_allow_quotes">allow_quotes</a>: bool, <a href="../social_contracts/post.md#social_contracts_post_allow_tips">allow_tips</a>: bool, access: <a href="../social_contracts/post.md#social_contracts_post_PostAccess">social_contracts::post::PostAccess</a>, promotion_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;, enable_spt: bool, <a href="../social_contracts/post.md#social_contracts_post_actor_address">actor_address</a>: <b>address</b>, <a href="../social_contracts/post.md#social_contracts_post_sub_agent_id">sub_agent_id</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;, organization_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;, <a href="../social_contracts/post.md#social_contracts_post_action_identity_class">action_identity_class</a>: u8, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_create_post_internal">create_post_internal</a>(
    owner: <b>address</b>,
    profile_id: <b>address</b>,
    platform_id: <b>address</b>,
    content: String,
    <a href="../social_contracts/post.md#social_contracts_post_media_asset_ids">media_asset_ids</a>: vector&lt;ID&gt;,
    media_option: Option&lt;vector&lt;Url&gt;&gt;,
    mentions: Option&lt;vector&lt;<b>address</b>&gt;&gt;,
    metadata_json: Option&lt;String&gt;,
    post_type: String,
    parent_post_id: Option&lt;<b>address</b>&gt;,
    <a href="../social_contracts/post.md#social_contracts_post_allow_comments">allow_comments</a>: bool,
    <a href="../social_contracts/post.md#social_contracts_post_allow_reactions">allow_reactions</a>: bool,
    <a href="../social_contracts/post.md#social_contracts_post_allow_reposts">allow_reposts</a>: bool,
    <a href="../social_contracts/post.md#social_contracts_post_allow_quotes">allow_quotes</a>: bool,
    <a href="../social_contracts/post.md#social_contracts_post_allow_tips">allow_tips</a>: bool,
    access: <a href="../social_contracts/post.md#social_contracts_post_PostAccess">PostAccess</a>,
    promotion_id: Option&lt;<b>address</b>&gt;,
    enable_spt: bool,
    <a href="../social_contracts/post.md#social_contracts_post_actor_address">actor_address</a>: <b>address</b>,
    <a href="../social_contracts/post.md#social_contracts_post_sub_agent_id">sub_agent_id</a>: Option&lt;ID&gt;,
    organization_id: Option&lt;ID&gt;,
    <a href="../social_contracts/post.md#social_contracts_post_action_identity_class">action_identity_class</a>: u8,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
): <a href="../social_contracts/post.md#social_contracts_post_Post">Post</a> {
    <b>let</b> asset_len = vector::length(&<a href="../social_contracts/post.md#social_contracts_post_media_asset_ids">media_asset_ids</a>);
    <b>assert</b>!(asset_len &lt;= <a href="../social_contracts/post.md#social_contracts_post_MAX_MEDIA_URLS">MAX_MEDIA_URLS</a>, <a href="../social_contracts/post.md#social_contracts_post_ETooManyMediaUrls">ETooManyMediaUrls</a>);
    // Build permissions bitfield
    <b>let</b> <b>mut</b> permissions: u8 = 0;
    <b>if</b> (<a href="../social_contracts/post.md#social_contracts_post_allow_comments">allow_comments</a>) { permissions = permissions | <a href="../social_contracts/post.md#social_contracts_post_PERMISSION_ALLOW_COMMENTS">PERMISSION_ALLOW_COMMENTS</a> };
    <b>if</b> (<a href="../social_contracts/post.md#social_contracts_post_allow_reactions">allow_reactions</a>) { permissions = permissions | <a href="../social_contracts/post.md#social_contracts_post_PERMISSION_ALLOW_REACTIONS">PERMISSION_ALLOW_REACTIONS</a> };
    <b>if</b> (<a href="../social_contracts/post.md#social_contracts_post_allow_reposts">allow_reposts</a>) { permissions = permissions | <a href="../social_contracts/post.md#social_contracts_post_PERMISSION_ALLOW_REPOSTS">PERMISSION_ALLOW_REPOSTS</a> };
    <b>if</b> (<a href="../social_contracts/post.md#social_contracts_post_allow_quotes">allow_quotes</a>) { permissions = permissions | <a href="../social_contracts/post.md#social_contracts_post_PERMISSION_ALLOW_QUOTES">PERMISSION_ALLOW_QUOTES</a> };
    <b>if</b> (<a href="../social_contracts/post.md#social_contracts_post_allow_tips">allow_tips</a>) { permissions = permissions | <a href="../social_contracts/post.md#social_contracts_post_PERMISSION_ALLOW_TIPS">PERMISSION_ALLOW_TIPS</a> };
    <b>let</b> <b>mut</b> <a href="../social_contracts/post.md#social_contracts_post">post</a> = <a href="../social_contracts/post.md#social_contracts_post_Post">Post</a> {
        id: object::new(ctx),
        owner,
        profile_id,
        platform_id,
        content,
        <a href="../social_contracts/post.md#social_contracts_post_media_asset_ids">media_asset_ids</a>,
        media: media_option,
        mentions,
        metadata_json,
        post_type,
        parent_post_id,
        created_at: clock::timestamp_ms(clock),
        reaction_count: 0,
        comment_count: 0,
        repost_count: 0,
        tips_received: 0,
        removed_from_platform: <b>false</b>,
        user_reactions: table::new(ctx),
        reaction_counts: table::new(ctx),
        permissions,
        <a href="../social_contracts/post.md#social_contracts_post_composition_status">composition_status</a>: <a href="../social_contracts/media_asset.md#social_contracts_media_asset_composition_none">media_asset::composition_none</a>(),
        <a href="../social_contracts/post.md#social_contracts_post_monetization_status">monetization_status</a>: <a href="../social_contracts/media_asset.md#social_contracts_media_asset_monetization_none">media_asset::monetization_none</a>(),
        <a href="../social_contracts/post.md#social_contracts_post_composition_badge">composition_badge</a>: option::none(),
        access,
        promotion_id,
        enable_spt,
        spt_id: option::none(),
        <a href="../social_contracts/post.md#social_contracts_post_version">version</a>: <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(),
    };
    <a href="../social_contracts/post.md#social_contracts_post_attach_post_attribution">attach_post_attribution</a>(
        &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post">post</a>,
        <a href="../social_contracts/post.md#social_contracts_post_actor_address">actor_address</a>,
        <a href="../social_contracts/post.md#social_contracts_post_sub_agent_id">sub_agent_id</a>,
        organization_id,
        <a href="../social_contracts/post.md#social_contracts_post_action_identity_class">action_identity_class</a>,
    );
    <a href="../social_contracts/post.md#social_contracts_post">post</a>
}
</code></pre>



</details>

<a name="social_contracts_post_emit_post_media_asset_usages"></a>

## Function `emit_post_media_asset_usages`



<pre><code><b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_emit_post_media_asset_usages">emit_post_media_asset_usages</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_emit_post_media_asset_usages">emit_post_media_asset_usages</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>, clock: &Clock) {
    <b>let</b> post_id = object::uid_to_address(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id);
    <b>let</b> assets = &<a href="../social_contracts/post.md#social_contracts_post">post</a>.<a href="../social_contracts/post.md#social_contracts_post_media_asset_ids">media_asset_ids</a>;
    <b>let</b> len = vector::length(assets);
    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (i &lt; len) {
        <b>let</b> asset_id = *vector::borrow(assets, i);
        <a href="../social_contracts/media_asset.md#social_contracts_media_asset_emit_media_asset_used">media_asset::emit_media_asset_used</a>(
            post_id,
            <a href="../social_contracts/media_asset.md#social_contracts_media_asset_container_post">media_asset::container_post</a>(),
            asset_id,
            <a href="../social_contracts/media_asset.md#social_contracts_media_asset_usage_social_post">media_asset::usage_social_post</a>(),
            (i <b>as</b> u8),
            clock,
        );
        i = i + 1;
    };
}
</code></pre>



</details>

<a name="social_contracts_post_share_post"></a>

## Function `share_post`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_share_post">share_post</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: <a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_share_post">share_post</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: <a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>): <b>address</b> {
    <b>let</b> post_id = object::uid_to_address(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id);
    transfer::share_object(<a href="../social_contracts/post.md#social_contracts_post">post</a>);
    post_id
}
</code></pre>



</details>

<a name="social_contracts_post_assert_post_access_mydata_binding"></a>

## Function `assert_post_access_mydata_binding`

Validates registry ownership when a MyData listing is linked on <code>access</code>.


<pre><code><b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_assert_post_access_mydata_binding">assert_post_access_mydata_binding</a>(owner: <b>address</b>, access: &<a href="../social_contracts/post.md#social_contracts_post_PostAccess">social_contracts::post::PostAccess</a>, registry: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataRegistry">social_contracts::mydata::MyDataRegistry</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_assert_post_access_mydata_binding">assert_post_access_mydata_binding</a>(
    owner: <b>address</b>,
    access: &<a href="../social_contracts/post.md#social_contracts_post_PostAccess">PostAccess</a>,
    registry: &mydata::MyDataRegistry,
) {
    <b>let</b> linked = <a href="../social_contracts/post.md#social_contracts_post_linked_mydata_from_access">linked_mydata_from_access</a>(access);
    <b>if</b> (option::is_none(&linked)) {
        <b>return</b>
    };
    <b>let</b> linked_id = *option::borrow(&linked);
    <b>let</b> ip_id = object::id_to_address(&linked_id);
    <b>let</b> reg_owner = mydata::registry_get_owner(registry, ip_id);
    <b>assert</b>!(option::is_some(&reg_owner), <a href="../social_contracts/post.md#social_contracts_post_EMyDataNotRegistered">EMyDataNotRegistered</a>);
    <b>assert</b>!(*option::borrow(&reg_owner) == owner, <a href="../social_contracts/post.md#social_contracts_post_EMyDataOwnerMismatch">EMyDataOwnerMismatch</a>);
}
</code></pre>



</details>

<a name="social_contracts_post_assert_post_access_mydata_object_binding"></a>

## Function `assert_post_access_mydata_object_binding`

Cross-layer MyData access binding when the listing object is supplied in the PTB.


<pre><code><b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_assert_post_access_mydata_object_binding">assert_post_access_mydata_object_binding</a>(access: &<a href="../social_contracts/post.md#social_contracts_post_PostAccess">social_contracts::post::PostAccess</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_assert_post_access_mydata_object_binding">assert_post_access_mydata_object_binding</a>(
    access: &<a href="../social_contracts/post.md#social_contracts_post_PostAccess">PostAccess</a>,
    <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &mydata::MyData,
) {
    <b>let</b> linked = <a href="../social_contracts/post.md#social_contracts_post_linked_mydata_from_access">linked_mydata_from_access</a>(access);
    <b>assert</b>!(option::is_some(&linked), <a href="../social_contracts/post.md#social_contracts_post_ENoEncryptedContent">ENoEncryptedContent</a>);
    <b>let</b> linked_id = *option::borrow(&linked);
    <b>assert</b>!(object::id(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>) == linked_id, <a href="../social_contracts/post.md#social_contracts_post_EMyDataOwnerMismatch">EMyDataOwnerMismatch</a>);
    <b>assert</b>!(!mydata::requires_marketplace_subscription(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>), <a href="../social_contracts/post.md#social_contracts_post_ENoEncryptedContent">ENoEncryptedContent</a>);
    match (access) {
        PostAccess::Public =&gt; <b>abort</b> <a href="../social_contracts/post.md#social_contracts_post_EUnauthorized">EUnauthorized</a>,
        PostAccess::ProfileSubscription { .. } =&gt; {
            <b>assert</b>!(mydata::requires_profile_subscription_access(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>), <a href="../social_contracts/post.md#social_contracts_post_ENoEncryptedContent">ENoEncryptedContent</a>);
        },
        PostAccess::MarketplaceOneTime { .. } =&gt; {
            <b>assert</b>!(mydata::requires_marketplace_purchase(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>), <a href="../social_contracts/post.md#social_contracts_post_EPriceMismatch">EPriceMismatch</a>);
        },
    }
}
</code></pre>



</details>

<a name="social_contracts_post_linked_mydata_from_access"></a>

## Function `linked_mydata_from_access`



<pre><code><b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_linked_mydata_from_access">linked_mydata_from_access</a>(access: &<a href="../social_contracts/post.md#social_contracts_post_PostAccess">social_contracts::post::PostAccess</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_linked_mydata_from_access">linked_mydata_from_access</a>(access: &<a href="../social_contracts/post.md#social_contracts_post_PostAccess">PostAccess</a>): Option&lt;ID&gt; {
    <b>let</b> (_, _, mydata_id, _) = <a href="../social_contracts/post.md#social_contracts_post_post_access_fields">post_access_fields</a>(access);
    mydata_id
}
</code></pre>



</details>

<a name="social_contracts_post_assert_profile_subscription_access_service"></a>

## Function `assert_profile_subscription_access_service`



<pre><code><b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_assert_profile_subscription_access_service">assert_profile_subscription_access_service</a>(owner: <b>address</b>, access: &<a href="../social_contracts/post.md#social_contracts_post_PostAccess">social_contracts::post::PostAccess</a>, service: &<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">social_contracts::subscription::ProfileSubscriptionService</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_assert_profile_subscription_access_service">assert_profile_subscription_access_service</a>(
    owner: <b>address</b>,
    access: &<a href="../social_contracts/post.md#social_contracts_post_PostAccess">PostAccess</a>,
    service: &ProfileSubscriptionService,
) {
    match (access) {
        PostAccess::ProfileSubscription { service_id, .. } =&gt; {
            <b>assert</b>!(<a href="../social_contracts/subscription.md#social_contracts_subscription_service_profile_owner">subscription::service_profile_owner</a>(service) == owner, <a href="../social_contracts/post.md#social_contracts_post_EUnauthorized">EUnauthorized</a>);
            <b>assert</b>!(<a href="../social_contracts/subscription.md#social_contracts_subscription_service_is_active">subscription::service_is_active</a>(service), <a href="../social_contracts/post.md#social_contracts_post_ENoSubscriptionService">ENoSubscriptionService</a>);
            <b>assert</b>!(*service_id == object::id(service), <a href="../social_contracts/post.md#social_contracts_post_ENoSubscriptionService">ENoSubscriptionService</a>);
        },
        _ =&gt; <b>abort</b> <a href="../social_contracts/post.md#social_contracts_post_EUnauthorized">EUnauthorized</a>,
    }
}
</code></pre>



</details>

<a name="social_contracts_post_create_post_entry_body"></a>

## Function `create_post_entry_body`

Create a new post with interaction permissions.


<pre><code><b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_create_post_entry_body">create_post_entry_body</a>(registry: &<a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">social_contracts::profile::UsernameRegistry</a>, platform_registry: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">social_contracts::platform::PlatformRegistry</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, config: &<a href="../social_contracts/post.md#social_contracts_post_PostConfig">social_contracts::post::PostConfig</a>, memory_config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, content: <a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../social_contracts/post.md#social_contracts_post_media_asset_ids">media_asset_ids</a>: vector&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;, media_urls: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;&gt;, mentions: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;<b>address</b>&gt;&gt;, metadata_json: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, <a href="../social_contracts/post.md#social_contracts_post_allow_comments">allow_comments</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;bool&gt;, <a href="../social_contracts/post.md#social_contracts_post_allow_reactions">allow_reactions</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;bool&gt;, <a href="../social_contracts/post.md#social_contracts_post_allow_reposts">allow_reposts</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;bool&gt;, <a href="../social_contracts/post.md#social_contracts_post_allow_quotes">allow_quotes</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;bool&gt;, <a href="../social_contracts/post.md#social_contracts_post_allow_tips">allow_tips</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;bool&gt;, enable_spt: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;bool&gt;, enable_spot: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;bool&gt;, access: <a href="../social_contracts/post.md#social_contracts_post_PostAccess">social_contracts::post::PostAccess</a>, mydata_registry: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataRegistry">social_contracts::mydata::MyDataRegistry</a>, memory_account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_create_post_entry_body">create_post_entry_body</a>(
    registry: &UsernameRegistry,
    platform_registry: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">platform::PlatformRegistry</a>,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">platform::Platform</a>,
    block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">block_list::BlockListRegistry</a>,
    config: &<a href="../social_contracts/post.md#social_contracts_post_PostConfig">PostConfig</a>,
    memory_config: &MemoryConfig,
    content: String,
    <a href="../social_contracts/post.md#social_contracts_post_media_asset_ids">media_asset_ids</a>: vector&lt;ID&gt;,
    <b>mut</b> media_urls: Option&lt;vector&lt;String&gt;&gt;,
    mentions: Option&lt;vector&lt;<b>address</b>&gt;&gt;,
    metadata_json: Option&lt;String&gt;,
    <a href="../social_contracts/post.md#social_contracts_post_allow_comments">allow_comments</a>: Option&lt;bool&gt;,
    <a href="../social_contracts/post.md#social_contracts_post_allow_reactions">allow_reactions</a>: Option&lt;bool&gt;,
    <a href="../social_contracts/post.md#social_contracts_post_allow_reposts">allow_reposts</a>: Option&lt;bool&gt;,
    <a href="../social_contracts/post.md#social_contracts_post_allow_quotes">allow_quotes</a>: Option&lt;bool&gt;,
    <a href="../social_contracts/post.md#social_contracts_post_allow_tips">allow_tips</a>: Option&lt;bool&gt;,
    enable_spt: Option&lt;bool&gt;,
    enable_spot: Option&lt;bool&gt;,
    access: <a href="../social_contracts/post.md#social_contracts_post_PostAccess">PostAccess</a>,
    mydata_registry: &mydata::MyDataRegistry,
    memory_account: &MemoryAccount,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> acting = <a href="../social_contracts/post.md#social_contracts_post_resolve_social_actor">resolve_social_actor</a>(
        memory_config,
        registry,
        <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>,
        block_list_registry,
        memory_account,
        <a href="../social_contracts/memory.md#social_contracts_memory_cap_post_publish">memory::cap_post_publish</a>(),
        0,
        clock,
        ctx,
    );
    <b>let</b> owner = <a href="../social_contracts/memory.md#social_contracts_memory_acting_principal_owner">memory::acting_principal_owner</a>(&acting);
    <b>let</b> profile_id = <a href="../social_contracts/memory.md#social_contracts_memory_acting_profile_id">memory::acting_profile_id</a>(&acting);
    <b>let</b> <a href="../social_contracts/post.md#social_contracts_post_actor_address">actor_address</a> = <a href="../social_contracts/memory.md#social_contracts_memory_acting_actor_address">memory::acting_actor_address</a>(&acting);
    <b>let</b> <a href="../social_contracts/post.md#social_contracts_post_sub_agent_id">sub_agent_id</a> = <a href="../social_contracts/memory.md#social_contracts_memory_acting_sub_agent_id">memory::acting_sub_agent_id</a>(&acting);
    <b>let</b> organization_id = <a href="../social_contracts/memory.md#social_contracts_memory_acting_organization_id">memory::acting_organization_id</a>(&acting);
    <b>let</b> <a href="../social_contracts/post.md#social_contracts_post_action_identity_class">action_identity_class</a> = <a href="../social_contracts/memory.md#social_contracts_memory_acting_identity_class">memory::acting_identity_class</a>(&acting);
    <a href="../social_contracts/post.md#social_contracts_post_assert_post_access_mydata_binding">assert_post_access_mydata_binding</a>(owner, &access, mydata_registry);
    // Check <b>if</b> <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> is approved
    <b>let</b> platform_id = object::uid_to_address(<a href="../social_contracts/platform.md#social_contracts_platform_id">platform::id</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>));
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform_is_approved">platform::is_approved</a>(platform_registry, platform_id), <a href="../social_contracts/post.md#social_contracts_post_EUnauthorized">EUnauthorized</a>);
    // Validate content length using config
    <b>assert</b>!(string::length(&content) &lt;= config.max_content_length, <a href="../social_contracts/post.md#social_contracts_post_EContentTooLarge">EContentTooLarge</a>);
    // Validate metadata size <b>if</b> provided
    <b>if</b> (option::is_some(&metadata_json)) {
        <b>let</b> metadata_ref = option::borrow(&metadata_json);
        <b>assert</b>!(string::length(metadata_ref) &lt;= config.max_metadata_size, <a href="../social_contracts/post.md#social_contracts_post_EContentTooLarge">EContentTooLarge</a>);
    };
    // Convert and validate media URLs <b>if</b> provided
    <b>let</b> media_option = <b>if</b> (option::is_some(&media_urls)) {
        <b>let</b> url_strings = option::extract(&<b>mut</b> media_urls);
        // Validate media URLs count using config
        <b>assert</b>!(vector::length(&url_strings) &lt;= config.max_media_urls, <a href="../social_contracts/post.md#social_contracts_post_ETooManyMediaUrls">ETooManyMediaUrls</a>);
        // Convert string URLs to Url objects
        <b>let</b> <b>mut</b> urls = vector::empty&lt;Url&gt;();
        <b>let</b> <b>mut</b> i = 0;
        <b>let</b> len = vector::length(&url_strings);
        <b>while</b> (i &lt; len) {
            <b>let</b> url_string = vector::borrow(&url_strings, i);
            <b>let</b> url_bytes = string::as_bytes(url_string);
            vector::push_back(&<b>mut</b> urls, url::new_unsafe_from_bytes(*url_bytes));
            i = i + 1;
        };
        option::some(urls)
    } <b>else</b> {
        option::none&lt;vector&lt;Url&gt;&gt;()
    };
    // Validate mentions <b>if</b> provided using config
    <b>if</b> (option::is_some(&mentions)) {
        <b>let</b> mentions_ref = option::borrow(&mentions);
        <b>assert</b>!(vector::length(mentions_ref) &lt;= config.max_mentions, <a href="../social_contracts/post.md#social_contracts_post_EContentTooLarge">EContentTooLarge</a>);
    };
    // Set defaults <b>for</b> optional boolean parameters
    <b>let</b> final_allow_comments = <b>if</b> (option::is_some(&<a href="../social_contracts/post.md#social_contracts_post_allow_comments">allow_comments</a>)) {
        *option::borrow(&<a href="../social_contracts/post.md#social_contracts_post_allow_comments">allow_comments</a>)
    } <b>else</b> {
        <b>true</b> // Default to allowing comments
    };
    <b>let</b> final_allow_reactions = <b>if</b> (option::is_some(&<a href="../social_contracts/post.md#social_contracts_post_allow_reactions">allow_reactions</a>)) {
        *option::borrow(&<a href="../social_contracts/post.md#social_contracts_post_allow_reactions">allow_reactions</a>)
    } <b>else</b> {
        <b>true</b> // Default to allowing reactions
    };
    <b>let</b> final_allow_reposts = <b>if</b> (option::is_some(&<a href="../social_contracts/post.md#social_contracts_post_allow_reposts">allow_reposts</a>)) {
        *option::borrow(&<a href="../social_contracts/post.md#social_contracts_post_allow_reposts">allow_reposts</a>)
    } <b>else</b> {
        <b>true</b> // Default to allowing reposts
    };
    <b>let</b> final_allow_quotes = <b>if</b> (option::is_some(&<a href="../social_contracts/post.md#social_contracts_post_allow_quotes">allow_quotes</a>)) {
        *option::borrow(&<a href="../social_contracts/post.md#social_contracts_post_allow_quotes">allow_quotes</a>)
    } <b>else</b> {
        <b>true</b> // Default to allowing quotes
    };
    <b>let</b> final_allow_tips = <b>if</b> (option::is_some(&<a href="../social_contracts/post.md#social_contracts_post_allow_tips">allow_tips</a>)) {
        *option::borrow(&<a href="../social_contracts/post.md#social_contracts_post_allow_tips">allow_tips</a>)
    } <b>else</b> {
        <b>true</b> // Default to allowing tips
    };
    // Set defaults <b>for</b> feature flags (default to opt-out - users must explicitly opt-in)
    <b>let</b> final_enable_spt = <b>if</b> (option::is_some(&enable_spt)) {
        *option::borrow(&enable_spt)
    } <b>else</b> {
        <b>false</b> // Default to opt-out (user must explicitly opt-in)
    };
    // SPT opt-in at create must go through <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_create_post_with_reservation_pool">social_proof_tokens::create_post_with_reservation_pool</a>.
    <b>assert</b>!(!final_enable_spt, <a href="../social_contracts/post.md#social_contracts_post_ESptRequiresDedicatedCreate">ESptRequiresDedicatedCreate</a>);
    // enable_spot retained <b>for</b> <b>entry</b>-signature compatibility; SPoT is always-on.
    <b>let</b> _ = enable_spot;
    // Convert media URLs to strings <b>for</b> event (before moving media_option)
    <b>let</b> media_urls_for_event = <a href="../social_contracts/post.md#social_contracts_post_convert_urls_to_strings">convert_urls_to_strings</a>(&media_option);
    <b>let</b> media_asset_ids_for_event = <a href="../social_contracts/post.md#social_contracts_post_media_asset_ids">media_asset_ids</a>;
    <b>let</b> <a href="../social_contracts/post.md#social_contracts_post">post</a> = <a href="../social_contracts/post.md#social_contracts_post_create_post_internal">create_post_internal</a>(
        owner,
        profile_id,
        platform_id,
        content,
        <a href="../social_contracts/post.md#social_contracts_post_media_asset_ids">media_asset_ids</a>,
        media_option,
        mentions,
        metadata_json,
        string::utf8(<a href="../social_contracts/post.md#social_contracts_post_POST_TYPE_STANDARD">POST_TYPE_STANDARD</a>),
        option::none(),
        final_allow_comments,
        final_allow_reactions,
        final_allow_reposts,
        final_allow_quotes,
        final_allow_tips,
        access,
        option::none(), // promotion_id
        <b>false</b>,
        <a href="../social_contracts/post.md#social_contracts_post_actor_address">actor_address</a>,
        <a href="../social_contracts/post.md#social_contracts_post_sub_agent_id">sub_agent_id</a>,
        organization_id,
        <a href="../social_contracts/post.md#social_contracts_post_action_identity_class">action_identity_class</a>,
        clock,
        ctx
    );
    <a href="../social_contracts/post.md#social_contracts_post_emit_post_media_asset_usages">emit_post_media_asset_usages</a>(&<a href="../social_contracts/post.md#social_contracts_post">post</a>, clock);
    <b>let</b> post_id = <a href="../social_contracts/post.md#social_contracts_post_share_post">share_post</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>);
    <b>let</b> permissions_for_event = <a href="../social_contracts/post.md#social_contracts_post_permissions_bitfield">permissions_bitfield</a>(
        final_allow_comments,
        final_allow_reactions,
        final_allow_reposts,
        final_allow_quotes,
        final_allow_tips,
    );
    event::emit(<a href="../social_contracts/post.md#social_contracts_post_PostCreatedEvent">PostCreatedEvent</a> {
        post_id,
        owner,
        profile_id,
        platform_id,
        permissions: permissions_for_event,
        content,
        post_type: string::utf8(<a href="../social_contracts/post.md#social_contracts_post_POST_TYPE_STANDARD">POST_TYPE_STANDARD</a>),
        parent_post_id: option::none(),
        mentions,
        <a href="../social_contracts/post.md#social_contracts_post_media_asset_ids">media_asset_ids</a>: media_asset_ids_for_event,
        media_urls: media_urls_for_event,
        metadata_json,
        access,
        promotion_id: option::none(),
        <a href="../social_contracts/post.md#social_contracts_post_composition_status">composition_status</a>: <a href="../social_contracts/media_asset.md#social_contracts_media_asset_composition_none">media_asset::composition_none</a>(),
        <a href="../social_contracts/post.md#social_contracts_post_monetization_status">monetization_status</a>: <a href="../social_contracts/media_asset.md#social_contracts_media_asset_monetization_none">media_asset::monetization_none</a>(),
        enable_spt: <b>false</b>,
        spt_id: option::none(),
        <a href="../social_contracts/post.md#social_contracts_post_actor_address">actor_address</a>,
        <a href="../social_contracts/post.md#social_contracts_post_sub_agent_id">sub_agent_id</a>,
        organization_id,
        <a href="../social_contracts/post.md#social_contracts_post_action_identity_class">action_identity_class</a>,
    });
}
</code></pre>



</details>

<a name="social_contracts_post_create_post_object_for_spt"></a>

## Function `create_post_object_for_spt`

Build an unsared standard post with <code>enable_spt = <b>true</b></code> for SPT create-with-reservation.
Caller bootstraps the reservation pool, sets <code>spt_id</code>, then <code><a href="../social_contracts/post.md#social_contracts_post_share_and_emit_spt_post">share_and_emit_spt_post</a></code>.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_create_post_object_for_spt">create_post_object_for_spt</a>(registry: &<a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">social_contracts::profile::UsernameRegistry</a>, platform_registry: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">social_contracts::platform::PlatformRegistry</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, config: &<a href="../social_contracts/post.md#social_contracts_post_PostConfig">social_contracts::post::PostConfig</a>, memory_config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, content: <a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../social_contracts/post.md#social_contracts_post_media_asset_ids">media_asset_ids</a>: vector&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;, media_urls: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;&gt;, mentions: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;<b>address</b>&gt;&gt;, metadata_json: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, <a href="../social_contracts/post.md#social_contracts_post_allow_comments">allow_comments</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;bool&gt;, <a href="../social_contracts/post.md#social_contracts_post_allow_reactions">allow_reactions</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;bool&gt;, <a href="../social_contracts/post.md#social_contracts_post_allow_reposts">allow_reposts</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;bool&gt;, <a href="../social_contracts/post.md#social_contracts_post_allow_quotes">allow_quotes</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;bool&gt;, <a href="../social_contracts/post.md#social_contracts_post_allow_tips">allow_tips</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;bool&gt;, access: <a href="../social_contracts/post.md#social_contracts_post_PostAccess">social_contracts::post::PostAccess</a>, mydata_registry: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataRegistry">social_contracts::mydata::MyDataRegistry</a>, memory_account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_create_post_object_for_spt">create_post_object_for_spt</a>(
    registry: &UsernameRegistry,
    platform_registry: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">platform::PlatformRegistry</a>,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">platform::Platform</a>,
    block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">block_list::BlockListRegistry</a>,
    config: &<a href="../social_contracts/post.md#social_contracts_post_PostConfig">PostConfig</a>,
    memory_config: &MemoryConfig,
    content: String,
    <a href="../social_contracts/post.md#social_contracts_post_media_asset_ids">media_asset_ids</a>: vector&lt;ID&gt;,
    <b>mut</b> media_urls: Option&lt;vector&lt;String&gt;&gt;,
    mentions: Option&lt;vector&lt;<b>address</b>&gt;&gt;,
    metadata_json: Option&lt;String&gt;,
    <a href="../social_contracts/post.md#social_contracts_post_allow_comments">allow_comments</a>: Option&lt;bool&gt;,
    <a href="../social_contracts/post.md#social_contracts_post_allow_reactions">allow_reactions</a>: Option&lt;bool&gt;,
    <a href="../social_contracts/post.md#social_contracts_post_allow_reposts">allow_reposts</a>: Option&lt;bool&gt;,
    <a href="../social_contracts/post.md#social_contracts_post_allow_quotes">allow_quotes</a>: Option&lt;bool&gt;,
    <a href="../social_contracts/post.md#social_contracts_post_allow_tips">allow_tips</a>: Option&lt;bool&gt;,
    access: <a href="../social_contracts/post.md#social_contracts_post_PostAccess">PostAccess</a>,
    mydata_registry: &mydata::MyDataRegistry,
    memory_account: &MemoryAccount,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
): <a href="../social_contracts/post.md#social_contracts_post_Post">Post</a> {
    <b>let</b> acting = <a href="../social_contracts/post.md#social_contracts_post_resolve_social_actor">resolve_social_actor</a>(
        memory_config,
        registry,
        <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>,
        block_list_registry,
        memory_account,
        <a href="../social_contracts/memory.md#social_contracts_memory_cap_post_publish">memory::cap_post_publish</a>(),
        0,
        clock,
        ctx,
    );
    <b>let</b> owner = <a href="../social_contracts/memory.md#social_contracts_memory_acting_principal_owner">memory::acting_principal_owner</a>(&acting);
    <b>let</b> profile_id = <a href="../social_contracts/memory.md#social_contracts_memory_acting_profile_id">memory::acting_profile_id</a>(&acting);
    <b>let</b> <a href="../social_contracts/post.md#social_contracts_post_actor_address">actor_address</a> = <a href="../social_contracts/memory.md#social_contracts_memory_acting_actor_address">memory::acting_actor_address</a>(&acting);
    <b>let</b> <a href="../social_contracts/post.md#social_contracts_post_sub_agent_id">sub_agent_id</a> = <a href="../social_contracts/memory.md#social_contracts_memory_acting_sub_agent_id">memory::acting_sub_agent_id</a>(&acting);
    <b>let</b> organization_id = <a href="../social_contracts/memory.md#social_contracts_memory_acting_organization_id">memory::acting_organization_id</a>(&acting);
    <b>let</b> <a href="../social_contracts/post.md#social_contracts_post_action_identity_class">action_identity_class</a> = <a href="../social_contracts/memory.md#social_contracts_memory_acting_identity_class">memory::acting_identity_class</a>(&acting);
    <a href="../social_contracts/post.md#social_contracts_post_assert_post_access_mydata_binding">assert_post_access_mydata_binding</a>(owner, &access, mydata_registry);
    <b>let</b> platform_id = object::uid_to_address(<a href="../social_contracts/platform.md#social_contracts_platform_id">platform::id</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>));
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform_is_approved">platform::is_approved</a>(platform_registry, platform_id), <a href="../social_contracts/post.md#social_contracts_post_EUnauthorized">EUnauthorized</a>);
    <b>assert</b>!(string::length(&content) &lt;= config.max_content_length, <a href="../social_contracts/post.md#social_contracts_post_EContentTooLarge">EContentTooLarge</a>);
    <b>if</b> (option::is_some(&metadata_json)) {
        <b>let</b> metadata_ref = option::borrow(&metadata_json);
        <b>assert</b>!(string::length(metadata_ref) &lt;= config.max_metadata_size, <a href="../social_contracts/post.md#social_contracts_post_EContentTooLarge">EContentTooLarge</a>);
    };
    <b>let</b> media_option = <b>if</b> (option::is_some(&media_urls)) {
        <b>let</b> url_strings = option::extract(&<b>mut</b> media_urls);
        <b>assert</b>!(vector::length(&url_strings) &lt;= config.max_media_urls, <a href="../social_contracts/post.md#social_contracts_post_ETooManyMediaUrls">ETooManyMediaUrls</a>);
        <b>let</b> <b>mut</b> urls = vector::empty&lt;Url&gt;();
        <b>let</b> <b>mut</b> i = 0;
        <b>let</b> len = vector::length(&url_strings);
        <b>while</b> (i &lt; len) {
            <b>let</b> url_string = vector::borrow(&url_strings, i);
            <b>let</b> url_bytes = string::as_bytes(url_string);
            vector::push_back(&<b>mut</b> urls, url::new_unsafe_from_bytes(*url_bytes));
            i = i + 1;
        };
        option::some(urls)
    } <b>else</b> {
        option::none&lt;vector&lt;Url&gt;&gt;()
    };
    <b>if</b> (option::is_some(&mentions)) {
        <b>let</b> mentions_ref = option::borrow(&mentions);
        <b>assert</b>!(vector::length(mentions_ref) &lt;= config.max_mentions, <a href="../social_contracts/post.md#social_contracts_post_EContentTooLarge">EContentTooLarge</a>);
    };
    <b>let</b> final_allow_comments = <b>if</b> (option::is_some(&<a href="../social_contracts/post.md#social_contracts_post_allow_comments">allow_comments</a>)) {
        *option::borrow(&<a href="../social_contracts/post.md#social_contracts_post_allow_comments">allow_comments</a>)
    } <b>else</b> {
        <b>true</b>
    };
    <b>let</b> final_allow_reactions = <b>if</b> (option::is_some(&<a href="../social_contracts/post.md#social_contracts_post_allow_reactions">allow_reactions</a>)) {
        *option::borrow(&<a href="../social_contracts/post.md#social_contracts_post_allow_reactions">allow_reactions</a>)
    } <b>else</b> {
        <b>true</b>
    };
    <b>let</b> final_allow_reposts = <b>if</b> (option::is_some(&<a href="../social_contracts/post.md#social_contracts_post_allow_reposts">allow_reposts</a>)) {
        *option::borrow(&<a href="../social_contracts/post.md#social_contracts_post_allow_reposts">allow_reposts</a>)
    } <b>else</b> {
        <b>true</b>
    };
    <b>let</b> final_allow_quotes = <b>if</b> (option::is_some(&<a href="../social_contracts/post.md#social_contracts_post_allow_quotes">allow_quotes</a>)) {
        *option::borrow(&<a href="../social_contracts/post.md#social_contracts_post_allow_quotes">allow_quotes</a>)
    } <b>else</b> {
        <b>true</b>
    };
    <b>let</b> final_allow_tips = <b>if</b> (option::is_some(&<a href="../social_contracts/post.md#social_contracts_post_allow_tips">allow_tips</a>)) {
        *option::borrow(&<a href="../social_contracts/post.md#social_contracts_post_allow_tips">allow_tips</a>)
    } <b>else</b> {
        <b>true</b>
    };
    <a href="../social_contracts/post.md#social_contracts_post_create_post_internal">create_post_internal</a>(
        owner,
        profile_id,
        platform_id,
        content,
        <a href="../social_contracts/post.md#social_contracts_post_media_asset_ids">media_asset_ids</a>,
        media_option,
        mentions,
        metadata_json,
        string::utf8(<a href="../social_contracts/post.md#social_contracts_post_POST_TYPE_STANDARD">POST_TYPE_STANDARD</a>),
        option::none(),
        final_allow_comments,
        final_allow_reactions,
        final_allow_reposts,
        final_allow_quotes,
        final_allow_tips,
        access,
        option::none(),
        <b>true</b>, // enable_spt
        <a href="../social_contracts/post.md#social_contracts_post_actor_address">actor_address</a>,
        <a href="../social_contracts/post.md#social_contracts_post_sub_agent_id">sub_agent_id</a>,
        organization_id,
        <a href="../social_contracts/post.md#social_contracts_post_action_identity_class">action_identity_class</a>,
        clock,
        ctx
    )
}
</code></pre>



</details>

<a name="social_contracts_post_share_and_emit_spt_post"></a>

## Function `share_and_emit_spt_post`

Set reservation-pool <code>spt_id</code>, share the post, and emit <code><a href="../social_contracts/post.md#social_contracts_post_PostCreatedEvent">PostCreatedEvent</a></code> with SPT fields filled.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_share_and_emit_spt_post">share_and_emit_spt_post</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: <a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, spt_pool_id: <b>address</b>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_share_and_emit_spt_post">share_and_emit_spt_post</a>(<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post">post</a>: <a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>, spt_pool_id: <b>address</b>, clock: &Clock): <b>address</b> {
    <a href="../social_contracts/post.md#social_contracts_post_set_enable_spt">set_enable_spt</a>(&<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post">post</a>, <b>true</b>);
    <a href="../social_contracts/post.md#social_contracts_post_set_spt_id">set_spt_id</a>(&<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post">post</a>, spt_pool_id);
    <a href="../social_contracts/post.md#social_contracts_post_emit_post_media_asset_usages">emit_post_media_asset_usages</a>(&<a href="../social_contracts/post.md#social_contracts_post">post</a>, clock);
    <b>let</b> media_urls_for_event = <a href="../social_contracts/post.md#social_contracts_post_convert_urls_to_strings">convert_urls_to_strings</a>(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.media);
    <b>let</b> media_asset_ids_for_event = <a href="../social_contracts/post.md#social_contracts_post">post</a>.<a href="../social_contracts/post.md#social_contracts_post_media_asset_ids">media_asset_ids</a>;
    <b>let</b> attr = <a href="../social_contracts/post.md#social_contracts_post_post_attribution">post_attribution</a>(&<a href="../social_contracts/post.md#social_contracts_post">post</a>);
    <b>let</b> post_id = object::uid_to_address(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id);
    event::emit(<a href="../social_contracts/post.md#social_contracts_post_PostCreatedEvent">PostCreatedEvent</a> {
        post_id,
        owner: <a href="../social_contracts/post.md#social_contracts_post">post</a>.owner,
        profile_id: <a href="../social_contracts/post.md#social_contracts_post">post</a>.profile_id,
        platform_id: <a href="../social_contracts/post.md#social_contracts_post">post</a>.platform_id,
        permissions: <a href="../social_contracts/post.md#social_contracts_post">post</a>.permissions,
        content: <a href="../social_contracts/post.md#social_contracts_post">post</a>.content,
        post_type: <a href="../social_contracts/post.md#social_contracts_post">post</a>.post_type,
        parent_post_id: <a href="../social_contracts/post.md#social_contracts_post">post</a>.parent_post_id,
        mentions: <a href="../social_contracts/post.md#social_contracts_post">post</a>.mentions,
        <a href="../social_contracts/post.md#social_contracts_post_media_asset_ids">media_asset_ids</a>: media_asset_ids_for_event,
        media_urls: media_urls_for_event,
        metadata_json: <a href="../social_contracts/post.md#social_contracts_post">post</a>.metadata_json,
        access: <a href="../social_contracts/post.md#social_contracts_post">post</a>.access,
        promotion_id: <a href="../social_contracts/post.md#social_contracts_post">post</a>.promotion_id,
        <a href="../social_contracts/post.md#social_contracts_post_composition_status">composition_status</a>: <a href="../social_contracts/post.md#social_contracts_post">post</a>.<a href="../social_contracts/post.md#social_contracts_post_composition_status">composition_status</a>,
        <a href="../social_contracts/post.md#social_contracts_post_monetization_status">monetization_status</a>: <a href="../social_contracts/post.md#social_contracts_post">post</a>.<a href="../social_contracts/post.md#social_contracts_post_monetization_status">monetization_status</a>,
        enable_spt: <b>true</b>,
        spt_id: option::some(spt_pool_id),
        <a href="../social_contracts/post.md#social_contracts_post_actor_address">actor_address</a>: attr.<a href="../social_contracts/post.md#social_contracts_post_actor_address">actor_address</a>,
        <a href="../social_contracts/post.md#social_contracts_post_sub_agent_id">sub_agent_id</a>: attr.<a href="../social_contracts/post.md#social_contracts_post_sub_agent_id">sub_agent_id</a>,
        organization_id: attr.organization_id,
        <a href="../social_contracts/post.md#social_contracts_post_action_identity_class">action_identity_class</a>: attr.<a href="../social_contracts/post.md#social_contracts_post_action_identity_class">action_identity_class</a>,
    });
    transfer::share_object(<a href="../social_contracts/post.md#social_contracts_post">post</a>);
    post_id
}
</code></pre>



</details>

<a name="social_contracts_post_create_post"></a>

## Function `create_post`

Create a new post; caller supplies access via kind + optional service/mydata ids.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_create_post">create_post</a>(registry: &<a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">social_contracts::profile::UsernameRegistry</a>, platform_registry: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">social_contracts::platform::PlatformRegistry</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, config: &<a href="../social_contracts/post.md#social_contracts_post_PostConfig">social_contracts::post::PostConfig</a>, memory_config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, content: <a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../social_contracts/post.md#social_contracts_post_media_asset_ids">media_asset_ids</a>: vector&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;, media_urls: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;&gt;, mentions: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;<b>address</b>&gt;&gt;, metadata_json: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, <a href="../social_contracts/post.md#social_contracts_post_allow_comments">allow_comments</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;bool&gt;, <a href="../social_contracts/post.md#social_contracts_post_allow_reactions">allow_reactions</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;bool&gt;, <a href="../social_contracts/post.md#social_contracts_post_allow_reposts">allow_reposts</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;bool&gt;, <a href="../social_contracts/post.md#social_contracts_post_allow_quotes">allow_quotes</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;bool&gt;, <a href="../social_contracts/post.md#social_contracts_post_allow_tips">allow_tips</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;bool&gt;, enable_spt: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;bool&gt;, enable_spot: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;bool&gt;, access_kind: u8, subscription_service_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;, linked_mydata_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;, <a href="../social_contracts/post.md#social_contracts_post_subscription_min_tier_level">subscription_min_tier_level</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, mydata_registry: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataRegistry">social_contracts::mydata::MyDataRegistry</a>, memory_account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_create_post">create_post</a>(
    registry: &UsernameRegistry,
    platform_registry: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">platform::PlatformRegistry</a>,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">platform::Platform</a>,
    block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">block_list::BlockListRegistry</a>,
    config: &<a href="../social_contracts/post.md#social_contracts_post_PostConfig">PostConfig</a>,
    memory_config: &MemoryConfig,
    content: String,
    <a href="../social_contracts/post.md#social_contracts_post_media_asset_ids">media_asset_ids</a>: vector&lt;ID&gt;,
    media_urls: Option&lt;vector&lt;String&gt;&gt;,
    mentions: Option&lt;vector&lt;<b>address</b>&gt;&gt;,
    metadata_json: Option&lt;String&gt;,
    <a href="../social_contracts/post.md#social_contracts_post_allow_comments">allow_comments</a>: Option&lt;bool&gt;,
    <a href="../social_contracts/post.md#social_contracts_post_allow_reactions">allow_reactions</a>: Option&lt;bool&gt;,
    <a href="../social_contracts/post.md#social_contracts_post_allow_reposts">allow_reposts</a>: Option&lt;bool&gt;,
    <a href="../social_contracts/post.md#social_contracts_post_allow_quotes">allow_quotes</a>: Option&lt;bool&gt;,
    <a href="../social_contracts/post.md#social_contracts_post_allow_tips">allow_tips</a>: Option&lt;bool&gt;,
    enable_spt: Option&lt;bool&gt;,
    enable_spot: Option&lt;bool&gt;,
    access_kind: u8,
    subscription_service_id: Option&lt;ID&gt;,
    linked_mydata_id: Option&lt;ID&gt;,
    <a href="../social_contracts/post.md#social_contracts_post_subscription_min_tier_level">subscription_min_tier_level</a>: Option&lt;u64&gt;,
    mydata_registry: &mydata::MyDataRegistry,
    memory_account: &MemoryAccount,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> access = <a href="../social_contracts/post.md#social_contracts_post_post_access_from_parts">post_access_from_parts</a>(
        access_kind,
        subscription_service_id,
        linked_mydata_id,
        <a href="../social_contracts/post.md#social_contracts_post_subscription_min_tier_level">subscription_min_tier_level</a>,
    );
    <a href="../social_contracts/post.md#social_contracts_post_create_post_entry_body">create_post_entry_body</a>(
        registry,
        platform_registry,
        <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>,
        block_list_registry,
        config,
        memory_config,
        content,
        <a href="../social_contracts/post.md#social_contracts_post_media_asset_ids">media_asset_ids</a>,
        media_urls,
        mentions,
        metadata_json,
        <a href="../social_contracts/post.md#social_contracts_post_allow_comments">allow_comments</a>,
        <a href="../social_contracts/post.md#social_contracts_post_allow_reactions">allow_reactions</a>,
        <a href="../social_contracts/post.md#social_contracts_post_allow_reposts">allow_reposts</a>,
        <a href="../social_contracts/post.md#social_contracts_post_allow_quotes">allow_quotes</a>,
        <a href="../social_contracts/post.md#social_contracts_post_allow_tips">allow_tips</a>,
        enable_spt,
        enable_spot,
        access,
        mydata_registry,
        memory_account,
        clock,
        ctx,
    );
}
</code></pre>



</details>

<a name="social_contracts_post_validate_post_mydata_binding"></a>

## Function `validate_post_mydata_binding`

Optional object binding for [<code><a href="../social_contracts/post.md#social_contracts_post_create_post">create_post</a></code>] when a linked MyData listing is in the PTB.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_validate_post_mydata_binding">validate_post_mydata_binding</a>(access_kind: u8, subscription_service_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;, linked_mydata_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;, <a href="../social_contracts/post.md#social_contracts_post_subscription_min_tier_level">subscription_min_tier_level</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_validate_post_mydata_binding">validate_post_mydata_binding</a>(
    access_kind: u8,
    subscription_service_id: Option&lt;ID&gt;,
    linked_mydata_id: Option&lt;ID&gt;,
    <a href="../social_contracts/post.md#social_contracts_post_subscription_min_tier_level">subscription_min_tier_level</a>: Option&lt;u64&gt;,
    <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &mydata::MyData,
) {
    <b>let</b> access = <a href="../social_contracts/post.md#social_contracts_post_post_access_from_parts">post_access_from_parts</a>(
        access_kind,
        subscription_service_id,
        linked_mydata_id,
        <a href="../social_contracts/post.md#social_contracts_post_subscription_min_tier_level">subscription_min_tier_level</a>,
    );
    <a href="../social_contracts/post.md#social_contracts_post_assert_post_access_mydata_object_binding">assert_post_access_mydata_object_binding</a>(&access, <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>);
}
</code></pre>



</details>

<a name="social_contracts_post_create_public_post"></a>

## Function `create_public_post`

Create a public post (no subscription or marketplace gate).


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_create_public_post">create_public_post</a>(registry: &<a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">social_contracts::profile::UsernameRegistry</a>, platform_registry: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">social_contracts::platform::PlatformRegistry</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, config: &<a href="../social_contracts/post.md#social_contracts_post_PostConfig">social_contracts::post::PostConfig</a>, memory_config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, content: <a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../social_contracts/post.md#social_contracts_post_media_asset_ids">media_asset_ids</a>: vector&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;, media_urls: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;&gt;, mentions: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;<b>address</b>&gt;&gt;, metadata_json: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, <a href="../social_contracts/post.md#social_contracts_post_allow_comments">allow_comments</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;bool&gt;, <a href="../social_contracts/post.md#social_contracts_post_allow_reactions">allow_reactions</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;bool&gt;, <a href="../social_contracts/post.md#social_contracts_post_allow_reposts">allow_reposts</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;bool&gt;, <a href="../social_contracts/post.md#social_contracts_post_allow_quotes">allow_quotes</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;bool&gt;, <a href="../social_contracts/post.md#social_contracts_post_allow_tips">allow_tips</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;bool&gt;, enable_spt: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;bool&gt;, enable_spot: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;bool&gt;, mydata_registry: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataRegistry">social_contracts::mydata::MyDataRegistry</a>, memory_account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_create_public_post">create_public_post</a>(
    registry: &UsernameRegistry,
    platform_registry: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">platform::PlatformRegistry</a>,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">platform::Platform</a>,
    block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">block_list::BlockListRegistry</a>,
    config: &<a href="../social_contracts/post.md#social_contracts_post_PostConfig">PostConfig</a>,
    memory_config: &MemoryConfig,
    content: String,
    <a href="../social_contracts/post.md#social_contracts_post_media_asset_ids">media_asset_ids</a>: vector&lt;ID&gt;,
    media_urls: Option&lt;vector&lt;String&gt;&gt;,
    mentions: Option&lt;vector&lt;<b>address</b>&gt;&gt;,
    metadata_json: Option&lt;String&gt;,
    <a href="../social_contracts/post.md#social_contracts_post_allow_comments">allow_comments</a>: Option&lt;bool&gt;,
    <a href="../social_contracts/post.md#social_contracts_post_allow_reactions">allow_reactions</a>: Option&lt;bool&gt;,
    <a href="../social_contracts/post.md#social_contracts_post_allow_reposts">allow_reposts</a>: Option&lt;bool&gt;,
    <a href="../social_contracts/post.md#social_contracts_post_allow_quotes">allow_quotes</a>: Option&lt;bool&gt;,
    <a href="../social_contracts/post.md#social_contracts_post_allow_tips">allow_tips</a>: Option&lt;bool&gt;,
    enable_spt: Option&lt;bool&gt;,
    enable_spot: Option&lt;bool&gt;,
    mydata_registry: &mydata::MyDataRegistry,
    memory_account: &MemoryAccount,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <a href="../social_contracts/post.md#social_contracts_post_create_post_entry_body">create_post_entry_body</a>(
        registry,
        platform_registry,
        <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>,
        block_list_registry,
        config,
        memory_config,
        content,
        <a href="../social_contracts/post.md#social_contracts_post_media_asset_ids">media_asset_ids</a>,
        media_urls,
        mentions,
        metadata_json,
        <a href="../social_contracts/post.md#social_contracts_post_allow_comments">allow_comments</a>,
        <a href="../social_contracts/post.md#social_contracts_post_allow_reactions">allow_reactions</a>,
        <a href="../social_contracts/post.md#social_contracts_post_allow_reposts">allow_reposts</a>,
        <a href="../social_contracts/post.md#social_contracts_post_allow_quotes">allow_quotes</a>,
        <a href="../social_contracts/post.md#social_contracts_post_allow_tips">allow_tips</a>,
        enable_spt,
        enable_spot,
        PostAccess::Public,
        mydata_registry,
        memory_account,
        clock,
        ctx,
    );
}
</code></pre>



</details>

<a name="social_contracts_post_create_profile_subscription_post"></a>

## Function `create_profile_subscription_post`

Create a profile-subscription-gated post (optional linked MyData for encrypted body).


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_create_profile_subscription_post">create_profile_subscription_post</a>(registry: &<a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">social_contracts::profile::UsernameRegistry</a>, platform_registry: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">social_contracts::platform::PlatformRegistry</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, config: &<a href="../social_contracts/post.md#social_contracts_post_PostConfig">social_contracts::post::PostConfig</a>, memory_config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, content: <a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../social_contracts/post.md#social_contracts_post_media_asset_ids">media_asset_ids</a>: vector&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;, media_urls: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;&gt;, mentions: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;<b>address</b>&gt;&gt;, metadata_json: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, <a href="../social_contracts/post.md#social_contracts_post_allow_comments">allow_comments</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;bool&gt;, <a href="../social_contracts/post.md#social_contracts_post_allow_reactions">allow_reactions</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;bool&gt;, <a href="../social_contracts/post.md#social_contracts_post_allow_reposts">allow_reposts</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;bool&gt;, <a href="../social_contracts/post.md#social_contracts_post_allow_quotes">allow_quotes</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;bool&gt;, <a href="../social_contracts/post.md#social_contracts_post_allow_tips">allow_tips</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;bool&gt;, enable_spt: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;bool&gt;, enable_spot: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;bool&gt;, service: &<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">social_contracts::subscription::ProfileSubscriptionService</a>, min_tier_level: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, mydata_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;, mydata_registry: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataRegistry">social_contracts::mydata::MyDataRegistry</a>, memory_account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_create_profile_subscription_post">create_profile_subscription_post</a>(
    registry: &UsernameRegistry,
    platform_registry: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">platform::PlatformRegistry</a>,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">platform::Platform</a>,
    block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">block_list::BlockListRegistry</a>,
    config: &<a href="../social_contracts/post.md#social_contracts_post_PostConfig">PostConfig</a>,
    memory_config: &MemoryConfig,
    content: String,
    <a href="../social_contracts/post.md#social_contracts_post_media_asset_ids">media_asset_ids</a>: vector&lt;ID&gt;,
    media_urls: Option&lt;vector&lt;String&gt;&gt;,
    mentions: Option&lt;vector&lt;<b>address</b>&gt;&gt;,
    metadata_json: Option&lt;String&gt;,
    <a href="../social_contracts/post.md#social_contracts_post_allow_comments">allow_comments</a>: Option&lt;bool&gt;,
    <a href="../social_contracts/post.md#social_contracts_post_allow_reactions">allow_reactions</a>: Option&lt;bool&gt;,
    <a href="../social_contracts/post.md#social_contracts_post_allow_reposts">allow_reposts</a>: Option&lt;bool&gt;,
    <a href="../social_contracts/post.md#social_contracts_post_allow_quotes">allow_quotes</a>: Option&lt;bool&gt;,
    <a href="../social_contracts/post.md#social_contracts_post_allow_tips">allow_tips</a>: Option&lt;bool&gt;,
    enable_spt: Option&lt;bool&gt;,
    enable_spot: Option&lt;bool&gt;,
    service: &ProfileSubscriptionService,
    min_tier_level: Option&lt;u64&gt;,
    mydata_id: Option&lt;ID&gt;,
    mydata_registry: &mydata::MyDataRegistry,
    memory_account: &MemoryAccount,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> acting = <a href="../social_contracts/post.md#social_contracts_post_resolve_social_actor">resolve_social_actor</a>(
        memory_config,
        registry,
        <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>,
        block_list_registry,
        memory_account,
        <a href="../social_contracts/memory.md#social_contracts_memory_cap_post_publish">memory::cap_post_publish</a>(),
        0,
        clock,
        ctx,
    );
    <b>let</b> owner = <a href="../social_contracts/memory.md#social_contracts_memory_acting_principal_owner">memory::acting_principal_owner</a>(&acting);
    <b>let</b> access = PostAccess::ProfileSubscription {
        service_id: object::id(service),
        mydata_id,
        min_tier_level,
    };
    <a href="../social_contracts/post.md#social_contracts_post_assert_profile_subscription_access_service">assert_profile_subscription_access_service</a>(owner, &access, service);
    <a href="../social_contracts/post.md#social_contracts_post_create_post_entry_body">create_post_entry_body</a>(
        registry,
        platform_registry,
        <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>,
        block_list_registry,
        config,
        memory_config,
        content,
        <a href="../social_contracts/post.md#social_contracts_post_media_asset_ids">media_asset_ids</a>,
        media_urls,
        mentions,
        metadata_json,
        <a href="../social_contracts/post.md#social_contracts_post_allow_comments">allow_comments</a>,
        <a href="../social_contracts/post.md#social_contracts_post_allow_reactions">allow_reactions</a>,
        <a href="../social_contracts/post.md#social_contracts_post_allow_reposts">allow_reposts</a>,
        <a href="../social_contracts/post.md#social_contracts_post_allow_quotes">allow_quotes</a>,
        <a href="../social_contracts/post.md#social_contracts_post_allow_tips">allow_tips</a>,
        enable_spt,
        enable_spot,
        access,
        mydata_registry,
        memory_account,
        clock,
        ctx,
    );
}
</code></pre>



</details>

<a name="social_contracts_post_create_profile_subscription_post_with_mydata"></a>

## Function `create_profile_subscription_post_with_mydata`

Profile-subscription post with linked MyData object binding in the same PTB.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_create_profile_subscription_post_with_mydata">create_profile_subscription_post_with_mydata</a>(registry: &<a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">social_contracts::profile::UsernameRegistry</a>, platform_registry: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">social_contracts::platform::PlatformRegistry</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, config: &<a href="../social_contracts/post.md#social_contracts_post_PostConfig">social_contracts::post::PostConfig</a>, memory_config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, content: <a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../social_contracts/post.md#social_contracts_post_media_asset_ids">media_asset_ids</a>: vector&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;, media_urls: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;&gt;, mentions: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;<b>address</b>&gt;&gt;, metadata_json: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, <a href="../social_contracts/post.md#social_contracts_post_allow_comments">allow_comments</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;bool&gt;, <a href="../social_contracts/post.md#social_contracts_post_allow_reactions">allow_reactions</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;bool&gt;, <a href="../social_contracts/post.md#social_contracts_post_allow_reposts">allow_reposts</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;bool&gt;, <a href="../social_contracts/post.md#social_contracts_post_allow_quotes">allow_quotes</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;bool&gt;, <a href="../social_contracts/post.md#social_contracts_post_allow_tips">allow_tips</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;bool&gt;, enable_spt: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;bool&gt;, enable_spot: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;bool&gt;, service: &<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">social_contracts::subscription::ProfileSubscriptionService</a>, min_tier_level: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>, mydata_registry: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataRegistry">social_contracts::mydata::MyDataRegistry</a>, memory_account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_create_profile_subscription_post_with_mydata">create_profile_subscription_post_with_mydata</a>(
    registry: &UsernameRegistry,
    platform_registry: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">platform::PlatformRegistry</a>,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">platform::Platform</a>,
    block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">block_list::BlockListRegistry</a>,
    config: &<a href="../social_contracts/post.md#social_contracts_post_PostConfig">PostConfig</a>,
    memory_config: &MemoryConfig,
    content: String,
    <a href="../social_contracts/post.md#social_contracts_post_media_asset_ids">media_asset_ids</a>: vector&lt;ID&gt;,
    media_urls: Option&lt;vector&lt;String&gt;&gt;,
    mentions: Option&lt;vector&lt;<b>address</b>&gt;&gt;,
    metadata_json: Option&lt;String&gt;,
    <a href="../social_contracts/post.md#social_contracts_post_allow_comments">allow_comments</a>: Option&lt;bool&gt;,
    <a href="../social_contracts/post.md#social_contracts_post_allow_reactions">allow_reactions</a>: Option&lt;bool&gt;,
    <a href="../social_contracts/post.md#social_contracts_post_allow_reposts">allow_reposts</a>: Option&lt;bool&gt;,
    <a href="../social_contracts/post.md#social_contracts_post_allow_quotes">allow_quotes</a>: Option&lt;bool&gt;,
    <a href="../social_contracts/post.md#social_contracts_post_allow_tips">allow_tips</a>: Option&lt;bool&gt;,
    enable_spt: Option&lt;bool&gt;,
    enable_spot: Option&lt;bool&gt;,
    service: &ProfileSubscriptionService,
    min_tier_level: Option&lt;u64&gt;,
    <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &mydata::MyData,
    mydata_registry: &mydata::MyDataRegistry,
    memory_account: &MemoryAccount,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> acting = <a href="../social_contracts/post.md#social_contracts_post_resolve_social_actor">resolve_social_actor</a>(
        memory_config,
        registry,
        <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>,
        block_list_registry,
        memory_account,
        <a href="../social_contracts/memory.md#social_contracts_memory_cap_post_publish">memory::cap_post_publish</a>(),
        0,
        clock,
        ctx,
    );
    <b>let</b> owner = <a href="../social_contracts/memory.md#social_contracts_memory_acting_principal_owner">memory::acting_principal_owner</a>(&acting);
    <b>let</b> access = PostAccess::ProfileSubscription {
        service_id: object::id(service),
        mydata_id: option::some(object::id(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>)),
        min_tier_level,
    };
    <a href="../social_contracts/post.md#social_contracts_post_assert_profile_subscription_access_service">assert_profile_subscription_access_service</a>(owner, &access, service);
    <a href="../social_contracts/post.md#social_contracts_post_assert_post_access_mydata_binding">assert_post_access_mydata_binding</a>(owner, &access, mydata_registry);
    <a href="../social_contracts/post.md#social_contracts_post_assert_post_access_mydata_object_binding">assert_post_access_mydata_object_binding</a>(&access, <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>);
    <a href="../social_contracts/post.md#social_contracts_post_create_post_entry_body">create_post_entry_body</a>(
        registry,
        platform_registry,
        <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>,
        block_list_registry,
        config,
        memory_config,
        content,
        <a href="../social_contracts/post.md#social_contracts_post_media_asset_ids">media_asset_ids</a>,
        media_urls,
        mentions,
        metadata_json,
        <a href="../social_contracts/post.md#social_contracts_post_allow_comments">allow_comments</a>,
        <a href="../social_contracts/post.md#social_contracts_post_allow_reactions">allow_reactions</a>,
        <a href="../social_contracts/post.md#social_contracts_post_allow_reposts">allow_reposts</a>,
        <a href="../social_contracts/post.md#social_contracts_post_allow_quotes">allow_quotes</a>,
        <a href="../social_contracts/post.md#social_contracts_post_allow_tips">allow_tips</a>,
        enable_spt,
        enable_spot,
        access,
        mydata_registry,
        memory_account,
        clock,
        ctx,
    );
}
</code></pre>



</details>

<a name="social_contracts_post_create_marketplace_one_time_post"></a>

## Function `create_marketplace_one_time_post`

Create a marketplace one-time purchase gated post (requires linked MyData listing).


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_create_marketplace_one_time_post">create_marketplace_one_time_post</a>(registry: &<a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">social_contracts::profile::UsernameRegistry</a>, platform_registry: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">social_contracts::platform::PlatformRegistry</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, config: &<a href="../social_contracts/post.md#social_contracts_post_PostConfig">social_contracts::post::PostConfig</a>, memory_config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, content: <a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../social_contracts/post.md#social_contracts_post_media_asset_ids">media_asset_ids</a>: vector&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;, media_urls: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;&gt;, mentions: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;<b>address</b>&gt;&gt;, metadata_json: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, <a href="../social_contracts/post.md#social_contracts_post_allow_comments">allow_comments</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;bool&gt;, <a href="../social_contracts/post.md#social_contracts_post_allow_reactions">allow_reactions</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;bool&gt;, <a href="../social_contracts/post.md#social_contracts_post_allow_reposts">allow_reposts</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;bool&gt;, <a href="../social_contracts/post.md#social_contracts_post_allow_quotes">allow_quotes</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;bool&gt;, <a href="../social_contracts/post.md#social_contracts_post_allow_tips">allow_tips</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;bool&gt;, enable_spt: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;bool&gt;, enable_spot: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;bool&gt;, <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>, mydata_registry: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataRegistry">social_contracts::mydata::MyDataRegistry</a>, memory_account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_create_marketplace_one_time_post">create_marketplace_one_time_post</a>(
    registry: &UsernameRegistry,
    platform_registry: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">platform::PlatformRegistry</a>,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">platform::Platform</a>,
    block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">block_list::BlockListRegistry</a>,
    config: &<a href="../social_contracts/post.md#social_contracts_post_PostConfig">PostConfig</a>,
    memory_config: &MemoryConfig,
    content: String,
    <a href="../social_contracts/post.md#social_contracts_post_media_asset_ids">media_asset_ids</a>: vector&lt;ID&gt;,
    media_urls: Option&lt;vector&lt;String&gt;&gt;,
    mentions: Option&lt;vector&lt;<b>address</b>&gt;&gt;,
    metadata_json: Option&lt;String&gt;,
    <a href="../social_contracts/post.md#social_contracts_post_allow_comments">allow_comments</a>: Option&lt;bool&gt;,
    <a href="../social_contracts/post.md#social_contracts_post_allow_reactions">allow_reactions</a>: Option&lt;bool&gt;,
    <a href="../social_contracts/post.md#social_contracts_post_allow_reposts">allow_reposts</a>: Option&lt;bool&gt;,
    <a href="../social_contracts/post.md#social_contracts_post_allow_quotes">allow_quotes</a>: Option&lt;bool&gt;,
    <a href="../social_contracts/post.md#social_contracts_post_allow_tips">allow_tips</a>: Option&lt;bool&gt;,
    enable_spt: Option&lt;bool&gt;,
    enable_spot: Option&lt;bool&gt;,
    <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &mydata::MyData,
    mydata_registry: &mydata::MyDataRegistry,
    memory_account: &MemoryAccount,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> access = PostAccess::MarketplaceOneTime { mydata_id: object::id(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>) };
    <b>let</b> acting = <a href="../social_contracts/post.md#social_contracts_post_resolve_social_actor">resolve_social_actor</a>(
        memory_config,
        registry,
        <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>,
        block_list_registry,
        memory_account,
        <a href="../social_contracts/memory.md#social_contracts_memory_cap_post_publish">memory::cap_post_publish</a>(),
        0,
        clock,
        ctx,
    );
    <b>let</b> owner = <a href="../social_contracts/memory.md#social_contracts_memory_acting_principal_owner">memory::acting_principal_owner</a>(&acting);
    <a href="../social_contracts/post.md#social_contracts_post_assert_post_access_mydata_binding">assert_post_access_mydata_binding</a>(owner, &access, mydata_registry);
    <a href="../social_contracts/post.md#social_contracts_post_assert_post_access_mydata_object_binding">assert_post_access_mydata_object_binding</a>(&access, <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>);
    <a href="../social_contracts/post.md#social_contracts_post_create_post_entry_body">create_post_entry_body</a>(
        registry,
        platform_registry,
        <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>,
        block_list_registry,
        config,
        memory_config,
        content,
        <a href="../social_contracts/post.md#social_contracts_post_media_asset_ids">media_asset_ids</a>,
        media_urls,
        mentions,
        metadata_json,
        <a href="../social_contracts/post.md#social_contracts_post_allow_comments">allow_comments</a>,
        <a href="../social_contracts/post.md#social_contracts_post_allow_reactions">allow_reactions</a>,
        <a href="../social_contracts/post.md#social_contracts_post_allow_reposts">allow_reposts</a>,
        <a href="../social_contracts/post.md#social_contracts_post_allow_quotes">allow_quotes</a>,
        <a href="../social_contracts/post.md#social_contracts_post_allow_tips">allow_tips</a>,
        enable_spt,
        enable_spot,
        access,
        mydata_registry,
        memory_account,
        clock,
        ctx,
    );
}
</code></pre>



</details>

<a name="social_contracts_post_create_comment"></a>

## Function `create_comment`

Create a comment on a post or a reply to another comment
Returns the ID of the created comment


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_create_comment">create_comment</a>(registry: &<a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">social_contracts::profile::UsernameRegistry</a>, platform_registry: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">social_contracts::platform::PlatformRegistry</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, config: &<a href="../social_contracts/post.md#social_contracts_post_PostConfig">social_contracts::post::PostConfig</a>, memory_config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, memory_account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, parent_post: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, parent_comment_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;, content: <a href="../std/string.md#std_string_String">std::string::String</a>, media_urls: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;&gt;, mentions: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;<b>address</b>&gt;&gt;, metadata_json: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_create_comment">create_comment</a>(
    registry: &UsernameRegistry,
    platform_registry: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">platform::PlatformRegistry</a>,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">platform::Platform</a>,
    block_list_registry: &BlockListRegistry,
    config: &<a href="../social_contracts/post.md#social_contracts_post_PostConfig">PostConfig</a>,
    memory_config: &MemoryConfig,
    memory_account: &MemoryAccount,
    parent_post: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>,
    parent_comment_id: Option&lt;<b>address</b>&gt;,
    content: String,
    <b>mut</b> media_urls: Option&lt;vector&lt;String&gt;&gt;,
    mentions: Option&lt;vector&lt;<b>address</b>&gt;&gt;,
    metadata_json: Option&lt;String&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
): <b>address</b> {
    <b>let</b> acting = <a href="../social_contracts/post.md#social_contracts_post_resolve_social_actor">resolve_social_actor</a>(
        memory_config,
        registry,
        <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>,
        block_list_registry,
        memory_account,
        <a href="../social_contracts/memory.md#social_contracts_memory_cap_comment">memory::cap_comment</a>(),
        0,
        clock,
        ctx,
    );
    <b>let</b> owner = <a href="../social_contracts/memory.md#social_contracts_memory_acting_principal_owner">memory::acting_principal_owner</a>(&acting);
    <b>let</b> profile_id = <a href="../social_contracts/memory.md#social_contracts_memory_acting_profile_id">memory::acting_profile_id</a>(&acting);
    <b>let</b> <a href="../social_contracts/post.md#social_contracts_post_actor_address">actor_address</a> = <a href="../social_contracts/memory.md#social_contracts_memory_acting_actor_address">memory::acting_actor_address</a>(&acting);
    <b>let</b> <a href="../social_contracts/post.md#social_contracts_post_sub_agent_id">sub_agent_id</a> = <a href="../social_contracts/memory.md#social_contracts_memory_acting_sub_agent_id">memory::acting_sub_agent_id</a>(&acting);
    <b>let</b> organization_id = <a href="../social_contracts/memory.md#social_contracts_memory_acting_organization_id">memory::acting_organization_id</a>(&acting);
    <b>let</b> <a href="../social_contracts/post.md#social_contracts_post_action_identity_class">action_identity_class</a> = <a href="../social_contracts/memory.md#social_contracts_memory_acting_identity_class">memory::acting_identity_class</a>(&acting);
    // Check <b>if</b> <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> is approved
    <b>let</b> platform_id = object::uid_to_address(<a href="../social_contracts/platform.md#social_contracts_platform_id">platform::id</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>));
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform_is_approved">platform::is_approved</a>(platform_registry, platform_id), <a href="../social_contracts/post.md#social_contracts_post_EUnauthorized">EUnauthorized</a>);
    // Check <b>if</b> the actor is blocked by the <a href="../social_contracts/post.md#social_contracts_post">post</a> creator
    <b>assert</b>!(!<a href="../social_contracts/block_list.md#social_contracts_block_list_is_blocked">block_list::is_blocked</a>(block_list_registry, parent_post.owner, <a href="../social_contracts/post.md#social_contracts_post_actor_address">actor_address</a>), <a href="../social_contracts/post.md#social_contracts_post_EUnauthorized">EUnauthorized</a>);
    // Check <b>if</b> comments are allowed on the parent <a href="../social_contracts/post.md#social_contracts_post">post</a>
    <b>assert</b>!(<a href="../social_contracts/post.md#social_contracts_post_allow_comments">allow_comments</a>(parent_post), <a href="../social_contracts/post.md#social_contracts_post_ECommentsNotAllowed">ECommentsNotAllowed</a>);
    // Validate content length using config
    <b>assert</b>!(string::length(&content) &lt;= config.max_content_length, <a href="../social_contracts/post.md#social_contracts_post_EContentTooLarge">EContentTooLarge</a>);
    // Validate metadata size <b>if</b> provided
    <b>if</b> (option::is_some(&metadata_json)) {
        <b>let</b> metadata_ref = option::borrow(&metadata_json);
        <b>assert</b>!(string::length(metadata_ref) &lt;= config.max_metadata_size, <a href="../social_contracts/post.md#social_contracts_post_EContentTooLarge">EContentTooLarge</a>);
    };
    // Convert and validate media URLs <b>if</b> provided
    <b>let</b> media_option = <b>if</b> (option::is_some(&media_urls)) {
        <b>let</b> url_strings = option::extract(&<b>mut</b> media_urls);
        // Validate media URLs count using config
        <b>assert</b>!(vector::length(&url_strings) &lt;= config.max_media_urls, <a href="../social_contracts/post.md#social_contracts_post_ETooManyMediaUrls">ETooManyMediaUrls</a>);
        // Convert string URLs to Url objects
        <b>let</b> <b>mut</b> urls = vector::empty&lt;Url&gt;();
        <b>let</b> <b>mut</b> i = 0;
        <b>let</b> len = vector::length(&url_strings);
        <b>while</b> (i &lt; len) {
            <b>let</b> url_string = vector::borrow(&url_strings, i);
            <b>let</b> url_bytes = string::as_bytes(url_string);
            vector::push_back(&<b>mut</b> urls, url::new_unsafe_from_bytes(*url_bytes));
            i = i + 1;
        };
        option::some(urls)
    } <b>else</b> {
        option::none&lt;vector&lt;Url&gt;&gt;()
    };
    // Validate mentions <b>if</b> provided using config
    <b>if</b> (option::is_some(&mentions)) {
        <b>let</b> mentions_ref = option::borrow(&mentions);
        <b>assert</b>!(vector::length(mentions_ref) &lt;= config.max_mentions, <a href="../social_contracts/post.md#social_contracts_post_EContentTooLarge">EContentTooLarge</a>);
    };
    // Get parent <a href="../social_contracts/post.md#social_contracts_post">post</a> ID
    <b>let</b> parent_post_id = object::uid_to_address(&parent_post.id);
    // Create a proper <a href="../social_contracts/post.md#social_contracts_post_Comment">Comment</a> object instead of reusing <a href="../social_contracts/post.md#social_contracts_post">post</a> structure
    <b>let</b> <b>mut</b> comment = <a href="../social_contracts/post.md#social_contracts_post_Comment">Comment</a> {
        id: object::new(ctx),
        post_id: parent_post_id,
        parent_comment_id,
        owner,
        profile_id,
        content,
        media: media_option,
        mentions,
        metadata_json,
        created_at: clock::timestamp_ms(clock),
        reaction_count: 0,
        comment_count: 0,
        repost_count: 0,
        tips_received: 0,
        removed_from_platform: <b>false</b>,
        user_reactions: table::new(ctx),
        reaction_counts: table::new(ctx),
        <a href="../social_contracts/post.md#social_contracts_post_version">version</a>: <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(),
    };
    <a href="../social_contracts/post.md#social_contracts_post_attach_comment_attribution">attach_comment_attribution</a>(
        &<b>mut</b> comment,
        <a href="../social_contracts/post.md#social_contracts_post_actor_address">actor_address</a>,
        <a href="../social_contracts/post.md#social_contracts_post_sub_agent_id">sub_agent_id</a>,
        organization_id,
        <a href="../social_contracts/post.md#social_contracts_post_action_identity_class">action_identity_class</a>,
    );
    // Get comment ID before sharing
    <b>let</b> comment_id = object::uid_to_address(&comment.id);
    // Increment the parent <a href="../social_contracts/post.md#social_contracts_post">post</a>'s comment count with overflow protection
    // Stop incrementing at max but allow commenting to <b>continue</b>
    <b>if</b> (parent_post.comment_count &lt; <a href="../social_contracts/post.md#social_contracts_post_MAX_U64">MAX_U64</a>) {
        <b>assert</b>!(parent_post.comment_count &lt;= <a href="../social_contracts/post.md#social_contracts_post_MAX_U64">MAX_U64</a> - 1, <a href="../social_contracts/post.md#social_contracts_post_EOverflow">EOverflow</a>);
        parent_post.comment_count = parent_post.comment_count + 1;
    };
    // Emit comment created event
    event::emit(<a href="../social_contracts/post.md#social_contracts_post_CommentCreatedEvent">CommentCreatedEvent</a> {
        comment_id,
        post_id: parent_post_id,
        parent_comment_id,
        owner,
        profile_id,
        content,
        mentions,
        <a href="../social_contracts/post.md#social_contracts_post_actor_address">actor_address</a>,
        <a href="../social_contracts/post.md#social_contracts_post_sub_agent_id">sub_agent_id</a>,
        organization_id,
        <a href="../social_contracts/post.md#social_contracts_post_action_identity_class">action_identity_class</a>,
    });
    // Share the comment object
    transfer::share_object(comment);
    // Return the comment ID to the caller
    comment_id
}
</code></pre>



</details>

<a name="social_contracts_post_create_repost"></a>

## Function `create_repost`

Create a repost or quote repost depending on provided parameters
If content is provided, it's treated as a quote repost
If content is empty/none, it's treated as a standard repost


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_create_repost">create_repost</a>(registry: &<a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">social_contracts::profile::UsernameRegistry</a>, platform_registry: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">social_contracts::platform::PlatformRegistry</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, config: &<a href="../social_contracts/post.md#social_contracts_post_PostConfig">social_contracts::post::PostConfig</a>, memory_config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, original_post: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, content: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, <a href="../social_contracts/post.md#social_contracts_post_media_asset_ids">media_asset_ids</a>: vector&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;, media_urls: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;&gt;, mentions: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;<b>address</b>&gt;&gt;, metadata_json: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, <a href="../social_contracts/post.md#social_contracts_post_allow_comments">allow_comments</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;bool&gt;, <a href="../social_contracts/post.md#social_contracts_post_allow_reactions">allow_reactions</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;bool&gt;, <a href="../social_contracts/post.md#social_contracts_post_allow_reposts">allow_reposts</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;bool&gt;, <a href="../social_contracts/post.md#social_contracts_post_allow_quotes">allow_quotes</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;bool&gt;, <a href="../social_contracts/post.md#social_contracts_post_allow_tips">allow_tips</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;bool&gt;, enable_spt: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;bool&gt;, enable_spot: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;bool&gt;, memory_account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_create_repost">create_repost</a>(
    registry: &UsernameRegistry,
    platform_registry: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">platform::PlatformRegistry</a>,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">platform::Platform</a>,
    block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">block_list::BlockListRegistry</a>,
    config: &<a href="../social_contracts/post.md#social_contracts_post_PostConfig">PostConfig</a>,
    memory_config: &MemoryConfig,
    original_post: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>,
    <b>mut</b> content: Option&lt;String&gt;,
    <a href="../social_contracts/post.md#social_contracts_post_media_asset_ids">media_asset_ids</a>: vector&lt;ID&gt;,
    <b>mut</b> media_urls: Option&lt;vector&lt;String&gt;&gt;,
    mentions: Option&lt;vector&lt;<b>address</b>&gt;&gt;,
    metadata_json: Option&lt;String&gt;,
    <a href="../social_contracts/post.md#social_contracts_post_allow_comments">allow_comments</a>: Option&lt;bool&gt;,
    <a href="../social_contracts/post.md#social_contracts_post_allow_reactions">allow_reactions</a>: Option&lt;bool&gt;,
    <a href="../social_contracts/post.md#social_contracts_post_allow_reposts">allow_reposts</a>: Option&lt;bool&gt;,
    <a href="../social_contracts/post.md#social_contracts_post_allow_quotes">allow_quotes</a>: Option&lt;bool&gt;,
    <a href="../social_contracts/post.md#social_contracts_post_allow_tips">allow_tips</a>: Option&lt;bool&gt;,
    enable_spt: Option&lt;bool&gt;,
    enable_spot: Option&lt;bool&gt;,
    memory_account: &MemoryAccount,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> acting = <a href="../social_contracts/post.md#social_contracts_post_resolve_social_actor">resolve_social_actor</a>(
        memory_config,
        registry,
        <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>,
        block_list_registry,
        memory_account,
        <a href="../social_contracts/memory.md#social_contracts_memory_cap_post_publish">memory::cap_post_publish</a>(),
        0,
        clock,
        ctx,
    );
    <b>let</b> owner = <a href="../social_contracts/memory.md#social_contracts_memory_acting_principal_owner">memory::acting_principal_owner</a>(&acting);
    <b>let</b> profile_id = <a href="../social_contracts/memory.md#social_contracts_memory_acting_profile_id">memory::acting_profile_id</a>(&acting);
    <b>let</b> <a href="../social_contracts/post.md#social_contracts_post_actor_address">actor_address</a> = <a href="../social_contracts/memory.md#social_contracts_memory_acting_actor_address">memory::acting_actor_address</a>(&acting);
    <b>let</b> <a href="../social_contracts/post.md#social_contracts_post_sub_agent_id">sub_agent_id</a> = <a href="../social_contracts/memory.md#social_contracts_memory_acting_sub_agent_id">memory::acting_sub_agent_id</a>(&acting);
    <b>let</b> organization_id = <a href="../social_contracts/memory.md#social_contracts_memory_acting_organization_id">memory::acting_organization_id</a>(&acting);
    <b>let</b> <a href="../social_contracts/post.md#social_contracts_post_action_identity_class">action_identity_class</a> = <a href="../social_contracts/memory.md#social_contracts_memory_acting_identity_class">memory::acting_identity_class</a>(&acting);
    // Check <b>if</b> <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> is approved
    <b>let</b> platform_id = object::uid_to_address(<a href="../social_contracts/platform.md#social_contracts_platform_id">platform::id</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>));
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform_is_approved">platform::is_approved</a>(platform_registry, platform_id), <a href="../social_contracts/post.md#social_contracts_post_EUnauthorized">EUnauthorized</a>);
    <b>let</b> original_post_id = object::uid_to_address(&original_post.id);
    // Determine <b>if</b> this is a quote repost or standard repost
    <b>let</b> is_quote_repost = option::is_some(&content) && string::length(option::borrow(&content)) &gt; 0;
    // Check <a href="../social_contracts/post.md#social_contracts_post">post</a> permissions directly
    <b>if</b> (is_quote_repost) {
        // For quote reposts, check <b>if</b> quoting is allowed
        <b>assert</b>!(<a href="../social_contracts/post.md#social_contracts_post_allow_quotes">allow_quotes</a>(original_post), <a href="../social_contracts/post.md#social_contracts_post_EQuotesNotAllowed">EQuotesNotAllowed</a>);
    } <b>else</b> {
        // For regular reposts, check <b>if</b> reposting is allowed
        <b>assert</b>!(<a href="../social_contracts/post.md#social_contracts_post_allow_reposts">allow_reposts</a>(original_post), <a href="../social_contracts/post.md#social_contracts_post_ERepostsNotAllowed">ERepostsNotAllowed</a>);
    };
    // Initialize content string
    <b>let</b> content_string = <b>if</b> (is_quote_repost) {
        // Validate content length <b>for</b> quote reposts
        <b>let</b> content_value = option::extract(&<b>mut</b> content);
        // Use config value instead of hardcoded constant
        <b>assert</b>!(string::length(&content_value) &lt;= config.max_content_length, <a href="../social_contracts/post.md#social_contracts_post_EContentTooLarge">EContentTooLarge</a>);
        content_value
    } <b>else</b> {
        // Empty string <b>for</b> standard reposts
        string::utf8(b"")
    };
    // Validate and process media URLs <b>if</b> provided
    <b>let</b> media_option = <b>if</b> (option::is_some(&media_urls)) {
        <b>let</b> url_strings = option::extract(&<b>mut</b> media_urls);
        // Validate media URLs count
        <b>assert</b>!(vector::length(&url_strings) &lt;= config.max_media_urls, <a href="../social_contracts/post.md#social_contracts_post_ETooManyMediaUrls">ETooManyMediaUrls</a>);
        // Convert string URLs to Url objects
        <b>let</b> <b>mut</b> urls = vector::empty&lt;Url&gt;();
        <b>let</b> <b>mut</b> i = 0;
        <b>let</b> len = vector::length(&url_strings);
        <b>while</b> (i &lt; len) {
            <b>let</b> url_string = vector::borrow(&url_strings, i);
            <b>let</b> url_bytes = string::as_bytes(url_string);
            vector::push_back(&<b>mut</b> urls, url::new_unsafe_from_bytes(*url_bytes));
            i = i + 1;
        };
        option::some(urls)
    } <b>else</b> {
        option::none&lt;vector&lt;Url&gt;&gt;()
    };
    // Validate metadata size <b>if</b> provided
    <b>if</b> (option::is_some(&metadata_json)) {
        <b>let</b> metadata_ref = option::borrow(&metadata_json);
        <b>assert</b>!(string::length(metadata_ref) &lt;= config.max_metadata_size, <a href="../social_contracts/post.md#social_contracts_post_EContentTooLarge">EContentTooLarge</a>);
    };
    // Validate mentions <b>if</b> provided
    <b>if</b> (option::is_some(&mentions)) {
        <b>let</b> mentions_ref = option::borrow(&mentions);
        <b>assert</b>!(vector::length(mentions_ref) &lt;= config.max_mentions, <a href="../social_contracts/post.md#social_contracts_post_EContentTooLarge">EContentTooLarge</a>);
    };
    // Create repost <b>as</b> <a href="../social_contracts/post.md#social_contracts_post">post</a> with appropriate type
    <b>let</b> post_type = <b>if</b> (is_quote_repost) {
        string::utf8(<a href="../social_contracts/post.md#social_contracts_post_POST_TYPE_QUOTE_REPOST">POST_TYPE_QUOTE_REPOST</a>)
    } <b>else</b> {
        string::utf8(<a href="../social_contracts/post.md#social_contracts_post_POST_TYPE_REPOST">POST_TYPE_REPOST</a>)
    };
    // For standard reposts, also create a <a href="../social_contracts/post.md#social_contracts_post_Repost">Repost</a> object
    <b>if</b> (!is_quote_repost) {
        <b>let</b> repost = <a href="../social_contracts/post.md#social_contracts_post_Repost">Repost</a> {
            id: object::new(ctx),
            original_id: original_post_id,
            is_original_post: <b>true</b>,
            owner,
            profile_id,
            created_at: clock::timestamp_ms(clock),
            <a href="../social_contracts/post.md#social_contracts_post_version">version</a>: <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(),
        };
        // Get repost ID before sharing
        <b>let</b> repost_id = object::uid_to_address(&repost.id);
        // Emit repost event before sharing
        event::emit(<a href="../social_contracts/post.md#social_contracts_post_RepostEvent">RepostEvent</a> {
            repost_id,
            original_id: original_post_id,
            is_original_post: <b>true</b>,
            owner,
            profile_id,
            <a href="../social_contracts/post.md#social_contracts_post_actor_address">actor_address</a>,
            <a href="../social_contracts/post.md#social_contracts_post_sub_agent_id">sub_agent_id</a>,
            organization_id,
            <a href="../social_contracts/post.md#social_contracts_post_action_identity_class">action_identity_class</a>,
        });
        // Share repost object
        transfer::share_object(repost);
    };
    // Increment original <a href="../social_contracts/post.md#social_contracts_post">post</a> repost count
    <b>assert</b>!(original_post.repost_count &lt;= <a href="../social_contracts/post.md#social_contracts_post_MAX_U64">MAX_U64</a> - 1, <a href="../social_contracts/post.md#social_contracts_post_EOverflow">EOverflow</a>);
    original_post.repost_count = original_post.repost_count + 1;
    // Set defaults <b>for</b> optional boolean parameters
    <b>let</b> final_allow_comments = <b>if</b> (option::is_some(&<a href="../social_contracts/post.md#social_contracts_post_allow_comments">allow_comments</a>)) {
        *option::borrow(&<a href="../social_contracts/post.md#social_contracts_post_allow_comments">allow_comments</a>)
    } <b>else</b> {
        <b>true</b> // Default to allowing comments
    };
    <b>let</b> final_allow_reactions = <b>if</b> (option::is_some(&<a href="../social_contracts/post.md#social_contracts_post_allow_reactions">allow_reactions</a>)) {
        *option::borrow(&<a href="../social_contracts/post.md#social_contracts_post_allow_reactions">allow_reactions</a>)
    } <b>else</b> {
        <b>true</b> // Default to allowing reactions
    };
    <b>let</b> final_allow_reposts = <b>if</b> (option::is_some(&<a href="../social_contracts/post.md#social_contracts_post_allow_reposts">allow_reposts</a>)) {
        *option::borrow(&<a href="../social_contracts/post.md#social_contracts_post_allow_reposts">allow_reposts</a>)
    } <b>else</b> {
        <b>true</b> // Default to allowing reposts
    };
    <b>let</b> final_allow_quotes = <b>if</b> (option::is_some(&<a href="../social_contracts/post.md#social_contracts_post_allow_quotes">allow_quotes</a>)) {
        *option::borrow(&<a href="../social_contracts/post.md#social_contracts_post_allow_quotes">allow_quotes</a>)
    } <b>else</b> {
        <b>true</b> // Default to allowing quotes
    };
    <b>let</b> final_allow_tips = <b>if</b> (option::is_some(&<a href="../social_contracts/post.md#social_contracts_post_allow_tips">allow_tips</a>)) {
        *option::borrow(&<a href="../social_contracts/post.md#social_contracts_post_allow_tips">allow_tips</a>)
    } <b>else</b> {
        <b>true</b> // Default to allowing tips
    };
    // Set defaults <b>for</b> feature flags (default to opt-out - users must explicitly opt-in)
    <b>let</b> final_enable_spt = <b>if</b> (option::is_some(&enable_spt)) {
        *option::borrow(&enable_spt)
    } <b>else</b> {
        <b>false</b> // Default to opt-out (user must explicitly opt-in)
    };
    <b>assert</b>!(!final_enable_spt, <a href="../social_contracts/post.md#social_contracts_post_ESptRequiresDedicatedCreate">ESptRequiresDedicatedCreate</a>);
    // enable_spot retained <b>for</b> <b>entry</b>-signature compatibility; SPoT is always-on.
    <b>let</b> _ = enable_spot;
    // Convert media URLs to strings <b>for</b> event (before moving media_option)
    <b>let</b> media_urls_for_event = <a href="../social_contracts/post.md#social_contracts_post_convert_urls_to_strings">convert_urls_to_strings</a>(&media_option);
    <b>let</b> media_asset_ids_for_event = <a href="../social_contracts/post.md#social_contracts_post_media_asset_ids">media_asset_ids</a>;
    <b>let</b> <a href="../social_contracts/post.md#social_contracts_post">post</a> = <a href="../social_contracts/post.md#social_contracts_post_create_post_internal">create_post_internal</a>(
        owner,
        profile_id,
        platform_id,
        content_string,
        <a href="../social_contracts/post.md#social_contracts_post_media_asset_ids">media_asset_ids</a>,
        media_option,
        mentions,
        metadata_json,
        post_type,
        option::some(original_post_id),
        final_allow_comments,
        final_allow_reactions,
        final_allow_reposts,
        final_allow_quotes,
        final_allow_tips,
        PostAccess::Public,
        option::none(), // promotion_id
        <b>false</b>,
        <a href="../social_contracts/post.md#social_contracts_post_actor_address">actor_address</a>,
        <a href="../social_contracts/post.md#social_contracts_post_sub_agent_id">sub_agent_id</a>,
        organization_id,
        <a href="../social_contracts/post.md#social_contracts_post_action_identity_class">action_identity_class</a>,
        clock,
        ctx
    );
    <b>let</b> permissions_for_event = <a href="../social_contracts/post.md#social_contracts_post_permissions_bitfield">permissions_bitfield</a>(
        final_allow_comments,
        final_allow_reactions,
        final_allow_reposts,
        final_allow_quotes,
        final_allow_tips,
    );
    <a href="../social_contracts/post.md#social_contracts_post_emit_post_media_asset_usages">emit_post_media_asset_usages</a>(&<a href="../social_contracts/post.md#social_contracts_post">post</a>, clock);
    <b>let</b> post_id = <a href="../social_contracts/post.md#social_contracts_post_share_post">share_post</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>);
    event::emit(<a href="../social_contracts/post.md#social_contracts_post_PostCreatedEvent">PostCreatedEvent</a> {
        post_id,
        owner,
        profile_id,
        platform_id,
        permissions: permissions_for_event,
        content: content_string,
        post_type,
        parent_post_id: option::some(original_post_id),
        mentions,
        <a href="../social_contracts/post.md#social_contracts_post_media_asset_ids">media_asset_ids</a>: media_asset_ids_for_event,
        media_urls: media_urls_for_event,
        metadata_json,
        access: PostAccess::Public,
        promotion_id: option::none(),
        <a href="../social_contracts/post.md#social_contracts_post_composition_status">composition_status</a>: <a href="../social_contracts/media_asset.md#social_contracts_media_asset_composition_none">media_asset::composition_none</a>(),
        <a href="../social_contracts/post.md#social_contracts_post_monetization_status">monetization_status</a>: <a href="../social_contracts/media_asset.md#social_contracts_media_asset_monetization_none">media_asset::monetization_none</a>(),
        enable_spt: <b>false</b>,
        spt_id: option::none(),
        <a href="../social_contracts/post.md#social_contracts_post_actor_address">actor_address</a>,
        <a href="../social_contracts/post.md#social_contracts_post_sub_agent_id">sub_agent_id</a>,
        organization_id,
        <a href="../social_contracts/post.md#social_contracts_post_action_identity_class">action_identity_class</a>,
    });
}
</code></pre>



</details>

<a name="social_contracts_post_delete_post"></a>

## Function `delete_post`

Delete a post owned by the caller


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_delete_post">delete_post</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: <a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_delete_post">delete_post</a>(
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: <a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> sender = tx_context::sender(ctx);
    <b>assert</b>!(sender == <a href="../social_contracts/post.md#social_contracts_post">post</a>.owner, <a href="../social_contracts/post.md#social_contracts_post_EUnauthorized">EUnauthorized</a>);
    <b>let</b> post_id_addr = object::uid_to_address(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id);
    // Emit event <b>for</b> the <a href="../social_contracts/post.md#social_contracts_post">post</a> deletion
    event::emit(<a href="../social_contracts/post.md#social_contracts_post_PostDeletedEvent">PostDeletedEvent</a> {
        post_id: post_id_addr,
        owner: <a href="../social_contracts/post.md#social_contracts_post">post</a>.owner,
        profile_id: <a href="../social_contracts/post.md#social_contracts_post">post</a>.profile_id,
        post_type: <a href="../social_contracts/post.md#social_contracts_post">post</a>.post_type,
        deleted_at: clock::timestamp_ms(clock)
    });
    // Extract UID to delete the <a href="../social_contracts/post.md#social_contracts_post">post</a> object
    <b>let</b> <a href="../social_contracts/post.md#social_contracts_post_Post">Post</a> {
        id,
        owner: _,
        profile_id: _,
        platform_id: _,
        content: _,
        <a href="../social_contracts/post.md#social_contracts_post_media_asset_ids">media_asset_ids</a>: _,
        media: _,
        mentions: _,
        metadata_json: _,
        post_type: _,
        parent_post_id: _,
        created_at: _,
        reaction_count: _,
        comment_count: _,
        repost_count: _,
        tips_received: _,
        removed_from_platform: _,
        user_reactions,
        reaction_counts,
        permissions: _,
        <a href="../social_contracts/post.md#social_contracts_post_composition_status">composition_status</a>: _,
        <a href="../social_contracts/post.md#social_contracts_post_monetization_status">monetization_status</a>: _,
        <a href="../social_contracts/post.md#social_contracts_post_composition_badge">composition_badge</a>: _,
        access: _,
        promotion_id: _,
        enable_spt: _,
        spt_id: _,
        <a href="../social_contracts/post.md#social_contracts_post_version">version</a>: _,
    } = <a href="../social_contracts/post.md#social_contracts_post">post</a>;
    // Clean up associated data structures
    table::drop(user_reactions);
    table::drop(reaction_counts);
    // Delete the <a href="../social_contracts/post.md#social_contracts_post">post</a> object
    object::delete(id);
}
</code></pre>



</details>

<a name="social_contracts_post_delete_comment"></a>

## Function `delete_comment`

Delete a comment owned by the caller


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_delete_comment">delete_comment</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, comment: <a href="../social_contracts/post.md#social_contracts_post_Comment">social_contracts::post::Comment</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_delete_comment">delete_comment</a>(
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>,
    comment: <a href="../social_contracts/post.md#social_contracts_post_Comment">Comment</a>,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> sender = tx_context::sender(ctx);
    <b>assert</b>!(sender == comment.owner, <a href="../social_contracts/post.md#social_contracts_post_EUnauthorized">EUnauthorized</a>);
    // Verify the comment belongs to this <a href="../social_contracts/post.md#social_contracts_post">post</a>
    <b>let</b> comment_post_id = comment.post_id;
    <b>let</b> post_id = object::uid_to_address(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id);
    <b>assert</b>!(comment_post_id == post_id, <a href="../social_contracts/post.md#social_contracts_post_EPostNotFound">EPostNotFound</a>);
    // Decrement the <a href="../social_contracts/post.md#social_contracts_post">post</a>'s comment count
    <a href="../social_contracts/post.md#social_contracts_post">post</a>.comment_count = <a href="../social_contracts/post.md#social_contracts_post">post</a>.comment_count - 1;
    // Emit event <b>for</b> the comment deletion
    event::emit(<a href="../social_contracts/post.md#social_contracts_post_CommentDeletedEvent">CommentDeletedEvent</a> {
        comment_id: object::uid_to_address(&comment.id),
        post_id,
        owner: comment.owner,
        profile_id: comment.profile_id,
        deleted_at: clock::timestamp_ms(clock)
    });
    // Extract UID to delete the comment object
    <b>let</b> <a href="../social_contracts/post.md#social_contracts_post_Comment">Comment</a> {
        id,
        post_id: _,
        parent_comment_id: _,
        owner: _,
        profile_id: _,
        content: _,
        media: _,
        mentions: _,
        metadata_json: _,
        created_at: _,
        reaction_count: _,
        comment_count: _,
        repost_count: _,
        tips_received: _,
        removed_from_platform: _,
        user_reactions,
        reaction_counts,
        <a href="../social_contracts/post.md#social_contracts_post_version">version</a>: _,
    } = comment;
    // Clean up associated data structures
    table::drop(user_reactions);
    table::drop(reaction_counts);
    // Delete the comment object
    object::delete(id);
}
</code></pre>



</details>

<a name="social_contracts_post_react_to_post"></a>

## Function `react_to_post`

React to a post with a specific reaction (emoji or text)
If the user already has the exact same reaction, it will be removed (toggle behavior)


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_react_to_post">react_to_post</a>(registry: &<a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">social_contracts::profile::UsernameRegistry</a>, <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, platform_registry: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">social_contracts::platform::PlatformRegistry</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, config: &<a href="../social_contracts/post.md#social_contracts_post_PostConfig">social_contracts::post::PostConfig</a>, memory_config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, memory_account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, reaction: <a href="../std/string.md#std_string_String">std::string::String</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_react_to_post">react_to_post</a>(
    registry: &UsernameRegistry,
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>,
    platform_registry: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">platform::PlatformRegistry</a>,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">platform::Platform</a>,
    block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">block_list::BlockListRegistry</a>,
    config: &<a href="../social_contracts/post.md#social_contracts_post_PostConfig">PostConfig</a>,
    memory_config: &MemoryConfig,
    memory_account: &MemoryAccount,
    reaction: String,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> acting = <a href="../social_contracts/post.md#social_contracts_post_resolve_social_actor">resolve_social_actor</a>(
        memory_config,
        registry,
        <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>,
        block_list_registry,
        memory_account,
        <a href="../social_contracts/memory.md#social_contracts_memory_cap_react">memory::cap_react</a>(),
        0,
        clock,
        ctx,
    );
    <b>let</b> <a href="../social_contracts/post.md#social_contracts_post_actor_address">actor_address</a> = <a href="../social_contracts/memory.md#social_contracts_memory_acting_actor_address">memory::acting_actor_address</a>(&acting);
    <b>let</b> principal_owner = <a href="../social_contracts/memory.md#social_contracts_memory_acting_principal_owner">memory::acting_principal_owner</a>(&acting);
    <b>let</b> <a href="../social_contracts/post.md#social_contracts_post_sub_agent_id">sub_agent_id</a> = <a href="../social_contracts/memory.md#social_contracts_memory_acting_sub_agent_id">memory::acting_sub_agent_id</a>(&acting);
    <b>let</b> organization_id = <a href="../social_contracts/memory.md#social_contracts_memory_acting_organization_id">memory::acting_organization_id</a>(&acting);
    <b>let</b> <a href="../social_contracts/post.md#social_contracts_post_action_identity_class">action_identity_class</a> = <a href="../social_contracts/memory.md#social_contracts_memory_acting_identity_class">memory::acting_identity_class</a>(&acting);
    // Check <b>if</b> <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> is approved
    <b>let</b> platform_id = object::uid_to_address(<a href="../social_contracts/platform.md#social_contracts_platform_id">platform::id</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>));
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform_is_approved">platform::is_approved</a>(platform_registry, platform_id), <a href="../social_contracts/post.md#social_contracts_post_EUnauthorized">EUnauthorized</a>);
    // Validate reaction length using config
    <b>assert</b>!(string::length(&reaction) &lt;= config.max_reaction_length, <a href="../social_contracts/post.md#social_contracts_post_EReactionContentTooLong">EReactionContentTooLong</a>);
    // Check <b>if</b> reactions are allowed on this <a href="../social_contracts/post.md#social_contracts_post">post</a>
    <b>assert</b>!(<a href="../social_contracts/post.md#social_contracts_post_allow_reactions">allow_reactions</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>), <a href="../social_contracts/post.md#social_contracts_post_EReactionsNotAllowed">EReactionsNotAllowed</a>);
    // Check <b>if</b> user already reacted to the <a href="../social_contracts/post.md#social_contracts_post">post</a>
    <b>if</b> (table::contains(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.user_reactions, <a href="../social_contracts/post.md#social_contracts_post_actor_address">actor_address</a>)) {
        // Get the previous reaction
        <b>let</b> previous_reaction = *table::borrow(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.user_reactions, <a href="../social_contracts/post.md#social_contracts_post_actor_address">actor_address</a>);
        // If the reaction is the same, remove it (toggle behavior)
        <b>if</b> (reaction == previous_reaction) {
            // Remove user's reaction
            table::remove(&<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post">post</a>.user_reactions, <a href="../social_contracts/post.md#social_contracts_post_actor_address">actor_address</a>);
            // Decrease count <b>for</b> this reaction type
            <b>let</b> count = *table::borrow(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.reaction_counts, reaction);
            <b>if</b> (count &lt;= 1) {
                table::remove(&<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post">post</a>.reaction_counts, reaction);
            } <b>else</b> {
                *table::borrow_mut(&<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post">post</a>.reaction_counts, reaction) = count - 1;
            };
            // Decrement <a href="../social_contracts/post.md#social_contracts_post">post</a> reaction count with underflow protection
            <b>assert</b>!(<a href="../social_contracts/post.md#social_contracts_post">post</a>.reaction_count &gt; 0, <a href="../social_contracts/post.md#social_contracts_post_EOverflow">EOverflow</a>);
            <a href="../social_contracts/post.md#social_contracts_post">post</a>.reaction_count = <a href="../social_contracts/post.md#social_contracts_post">post</a>.reaction_count - 1;
            // Emit remove reaction event
            event::emit(<a href="../social_contracts/post.md#social_contracts_post_RemoveReactionEvent">RemoveReactionEvent</a> {
                object_id: object::uid_to_address(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id),
                user: <a href="../social_contracts/post.md#social_contracts_post_actor_address">actor_address</a>,
                reaction,
                is_post: <b>true</b>,
                principal_owner,
                <a href="../social_contracts/post.md#social_contracts_post_actor_address">actor_address</a>,
                <a href="../social_contracts/post.md#social_contracts_post_sub_agent_id">sub_agent_id</a>,
                organization_id,
                <a href="../social_contracts/post.md#social_contracts_post_action_identity_class">action_identity_class</a>,
            });
            <b>return</b>
        };
        // Different reaction, update existing one
        // Decrease count <b>for</b> previous reaction
        <b>let</b> previous_count = *table::borrow(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.reaction_counts, previous_reaction);
        <b>if</b> (previous_count &lt;= 1) {
            table::remove(&<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post">post</a>.reaction_counts, previous_reaction);
        } <b>else</b> {
            *table::borrow_mut(&<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post">post</a>.reaction_counts, previous_reaction) = previous_count - 1;
        };
        // Update user's reaction
        *table::borrow_mut(&<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post">post</a>.user_reactions, <a href="../social_contracts/post.md#social_contracts_post_actor_address">actor_address</a>) = reaction;
    } <b>else</b> {
        // New reaction from this user
        table::add(&<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post">post</a>.user_reactions, <a href="../social_contracts/post.md#social_contracts_post_actor_address">actor_address</a>, reaction);
        // Increment <a href="../social_contracts/post.md#social_contracts_post">post</a> reaction count
        <b>assert</b>!(<a href="../social_contracts/post.md#social_contracts_post">post</a>.reaction_count &lt;= <a href="../social_contracts/post.md#social_contracts_post_MAX_U64">MAX_U64</a> - 1, <a href="../social_contracts/post.md#social_contracts_post_EOverflow">EOverflow</a>);
        <a href="../social_contracts/post.md#social_contracts_post">post</a>.reaction_count = <a href="../social_contracts/post.md#social_contracts_post">post</a>.reaction_count + 1;
    };
    // Increment count <b>for</b> the reaction
    <b>if</b> (table::contains(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.reaction_counts, reaction)) {
        <b>let</b> count = *table::borrow(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.reaction_counts, reaction);
        <b>assert</b>!(count &lt;= <a href="../social_contracts/post.md#social_contracts_post_MAX_U64">MAX_U64</a> - 1, <a href="../social_contracts/post.md#social_contracts_post_EOverflow">EOverflow</a>);
        *table::borrow_mut(&<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post">post</a>.reaction_counts, reaction) = count + 1;
    } <b>else</b> {
        table::add(&<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post">post</a>.reaction_counts, reaction, 1);
    };
    // Emit reaction event
    event::emit(<a href="../social_contracts/post.md#social_contracts_post_ReactionEvent">ReactionEvent</a> {
        object_id: object::uid_to_address(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id),
        user: <a href="../social_contracts/post.md#social_contracts_post_actor_address">actor_address</a>,
        reaction,
        is_post: <b>true</b>,
        principal_owner,
        <a href="../social_contracts/post.md#social_contracts_post_actor_address">actor_address</a>,
        <a href="../social_contracts/post.md#social_contracts_post_sub_agent_id">sub_agent_id</a>,
        organization_id,
        <a href="../social_contracts/post.md#social_contracts_post_action_identity_class">action_identity_class</a>,
    });
}
</code></pre>



</details>

<a name="social_contracts_post_tip_post"></a>

## Function `tip_post`

Tip a post creator with coin type <code>T</code>. When manifest entries use escrow payout mode for this
<code>amount</code>, <code>beneficiary_vault</code> must match the beneficiary's shared vault.


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_tip_post">tip_post</a>&lt;T&gt;(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, beneficiary_vault: &<b>mut</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_PoCBeneficiaryVault">social_contracts::poc_vault::PoCBeneficiaryVault</a>, coins: &<b>mut</b> <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;T&gt;, amount: u64, min_vault_deposit_amount: u64, memory_account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_tip_post">tip_post</a>&lt;T&gt;(
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>,
    beneficiary_vault: &<b>mut</b> PoCBeneficiaryVault,
    coins: &<b>mut</b> Coin&lt;T&gt;,
    amount: u64,
    min_vault_deposit_amount: u64,
    memory_account: &MemoryAccount,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(amount &gt; 0, <a href="../social_contracts/post.md#social_contracts_post_EInvalidTipAmount">EInvalidTipAmount</a>);
    <a href="../social_contracts/post.md#social_contracts_post_assert_tip_spend_limit">assert_tip_spend_limit</a>(memory_account, amount, ctx);
    <b>let</b> tipper = tx_context::sender(ctx);
    <b>assert</b>!(tipper != <a href="../social_contracts/post.md#social_contracts_post">post</a>.owner, <a href="../social_contracts/post.md#social_contracts_post_ESelfTipping">ESelfTipping</a>);
    <b>assert</b>!(
        string::utf8(<a href="../social_contracts/post.md#social_contracts_post_POST_TYPE_REPOST">POST_TYPE_REPOST</a>) != <a href="../social_contracts/post.md#social_contracts_post">post</a>.post_type &&
        string::utf8(<a href="../social_contracts/post.md#social_contracts_post_POST_TYPE_QUOTE_REPOST">POST_TYPE_QUOTE_REPOST</a>) != <a href="../social_contracts/post.md#social_contracts_post">post</a>.post_type,
        <a href="../social_contracts/post.md#social_contracts_post_EInvalidPostType">EInvalidPostType</a>
    );
    <b>assert</b>!(<a href="../social_contracts/post.md#social_contracts_post_allow_tips">allow_tips</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>), <a href="../social_contracts/post.md#social_contracts_post_ETipsNotAllowed">ETipsNotAllowed</a>);
    <b>let</b> post_owner_addr = <a href="../social_contracts/post.md#social_contracts_post">post</a>.owner;
    <b>let</b> post_oid = object::uid_to_address(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id);
    <b>let</b> actual_received = <a href="../social_contracts/post.md#social_contracts_post_apply_revenue_manifest_coin">apply_revenue_manifest_coin</a>&lt;T&gt;(
        <a href="../social_contracts/post.md#social_contracts_post">post</a>,
        beneficiary_vault,
        post_owner_addr,
        amount,
        coins,
        tipper,
        post_oid,
        <b>true</b>,
        min_vault_deposit_amount,
        clock,
        ctx
    );
    <b>assert</b>!(<a href="../social_contracts/post.md#social_contracts_post">post</a>.tips_received &lt;= <a href="../social_contracts/post.md#social_contracts_post_MAX_U64">MAX_U64</a> - actual_received, <a href="../social_contracts/post.md#social_contracts_post_EOverflow">EOverflow</a>);
    <a href="../social_contracts/post.md#social_contracts_post">post</a>.tips_received = <a href="../social_contracts/post.md#social_contracts_post">post</a>.tips_received + actual_received;
    <b>if</b> (actual_received &gt; 0) {
        event::emit(<a href="../social_contracts/post.md#social_contracts_post_TipEvent">TipEvent</a> {
            object_id: post_oid,
            from: tipper,
            to: post_owner_addr,
            amount: actual_received,
            coin_type: type_name::with_defining_ids&lt;T&gt;(),
            is_post: <b>true</b>,
        });
    };
}
</code></pre>



</details>

<a name="social_contracts_post_tip_post_simple"></a>

## Function `tip_post_simple`

Like [<code><a href="../social_contracts/post.md#social_contracts_post_tip_post">tip_post</a></code>] but without a <code>PoCBeneficiaryVault</code> argument. Only for posts where
[<code><a href="../social_contracts/post.md#social_contracts_post_tip_post_requires_beneficiary_vault_for_amount">tip_post_requires_beneficiary_vault_for_amount</a></code>] is false for this <code>amount</code>.
If an escrow deposit is required, aborts with [<code><a href="../social_contracts/post.md#social_contracts_post_ETipPostRequiresBeneficiaryVault">ETipPostRequiresBeneficiaryVault</a></code>].


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_tip_post_simple">tip_post_simple</a>&lt;T&gt;(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, coins: &<b>mut</b> <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;T&gt;, amount: u64, memory_account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_tip_post_simple">tip_post_simple</a>&lt;T&gt;(
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>,
    coins: &<b>mut</b> Coin&lt;T&gt;,
    amount: u64,
    memory_account: &MemoryAccount,
    ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(amount &gt; 0, <a href="../social_contracts/post.md#social_contracts_post_EInvalidTipAmount">EInvalidTipAmount</a>);
    <b>assert</b>!(
        !<a href="../social_contracts/post.md#social_contracts_post_tip_post_requires_beneficiary_vault_for_amount">tip_post_requires_beneficiary_vault_for_amount</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>, amount),
        <a href="../social_contracts/post.md#social_contracts_post_ETipPostRequiresBeneficiaryVault">ETipPostRequiresBeneficiaryVault</a>
    );
    <a href="../social_contracts/post.md#social_contracts_post_assert_tip_spend_limit">assert_tip_spend_limit</a>(memory_account, amount, ctx);
    <b>let</b> tipper = tx_context::sender(ctx);
    <b>assert</b>!(tipper != <a href="../social_contracts/post.md#social_contracts_post">post</a>.owner, <a href="../social_contracts/post.md#social_contracts_post_ESelfTipping">ESelfTipping</a>);
    <b>assert</b>!(
        string::utf8(<a href="../social_contracts/post.md#social_contracts_post_POST_TYPE_REPOST">POST_TYPE_REPOST</a>) != <a href="../social_contracts/post.md#social_contracts_post">post</a>.post_type &&
        string::utf8(<a href="../social_contracts/post.md#social_contracts_post_POST_TYPE_QUOTE_REPOST">POST_TYPE_QUOTE_REPOST</a>) != <a href="../social_contracts/post.md#social_contracts_post">post</a>.post_type,
        <a href="../social_contracts/post.md#social_contracts_post_EInvalidPostType">EInvalidPostType</a>
    );
    <b>assert</b>!(<a href="../social_contracts/post.md#social_contracts_post_allow_tips">allow_tips</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>), <a href="../social_contracts/post.md#social_contracts_post_ETipsNotAllowed">ETipsNotAllowed</a>);
    <b>let</b> post_owner_addr = <a href="../social_contracts/post.md#social_contracts_post">post</a>.owner;
    <b>let</b> post_oid = object::uid_to_address(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id);
    <b>let</b> actual_received = <a href="../social_contracts/post.md#social_contracts_post_apply_revenue_manifest_coin_without_beneficiary_vault">apply_revenue_manifest_coin_without_beneficiary_vault</a>&lt;T&gt;(
        <a href="../social_contracts/post.md#social_contracts_post">post</a>,
        post_owner_addr,
        amount,
        coins,
        tipper,
        post_oid,
        <b>true</b>,
        ctx
    );
    <b>assert</b>!(<a href="../social_contracts/post.md#social_contracts_post">post</a>.tips_received &lt;= <a href="../social_contracts/post.md#social_contracts_post_MAX_U64">MAX_U64</a> - actual_received, <a href="../social_contracts/post.md#social_contracts_post_EOverflow">EOverflow</a>);
    <a href="../social_contracts/post.md#social_contracts_post">post</a>.tips_received = <a href="../social_contracts/post.md#social_contracts_post">post</a>.tips_received + actual_received;
    <b>if</b> (actual_received &gt; 0) {
        event::emit(<a href="../social_contracts/post.md#social_contracts_post_TipEvent">TipEvent</a> {
            object_id: post_oid,
            from: tipper,
            to: post_owner_addr,
            amount: actual_received,
            coin_type: type_name::with_defining_ids&lt;T&gt;(),
            is_post: <b>true</b>,
        });
    };
}
</code></pre>



</details>

<a name="social_contracts_post_apply_revenue_manifest_coin"></a>

## Function `apply_revenue_manifest_coin`

Manifest-based revenue split for tips and fees on the creator-attributable pool.


<pre><code><b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_apply_revenue_manifest_coin">apply_revenue_manifest_coin</a>&lt;T&gt;(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, beneficiary_vault: &<b>mut</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_PoCBeneficiaryVault">social_contracts::poc_vault::PoCBeneficiaryVault</a>, intended_recipient: <b>address</b>, amount: u64, coins: &<b>mut</b> <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;T&gt;, tipper: <b>address</b>, object_id: <b>address</b>, is_post_event: bool, min_vault_deposit_amount: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_apply_revenue_manifest_coin">apply_revenue_manifest_coin</a>&lt;T&gt;(
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>,
    beneficiary_vault: &<b>mut</b> PoCBeneficiaryVault,
    intended_recipient: <b>address</b>,
    amount: u64,
    coins: &<b>mut</b> Coin&lt;T&gt;,
    tipper: <b>address</b>,
    object_id: <b>address</b>,
    is_post_event: bool,
    min_vault_deposit_amount: u64,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
): u64 {
    <b>if</b> (intended_recipient != <a href="../social_contracts/post.md#social_contracts_post">post</a>.owner) {
        <b>let</b> tip_coins = coin::split(coins, amount, ctx);
        transfer::public_transfer(tip_coins, intended_recipient);
        <b>return</b> amount
    };
    <b>if</b> (!<a href="../social_contracts/post.md#social_contracts_post_monetization_enabled">monetization_enabled</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>)) {
        <b>let</b> tip_coins = coin::split(coins, amount, ctx);
        transfer::public_transfer(tip_coins, intended_recipient);
        <b>return</b> amount
    };
    <b>let</b> manifest_opt = <a href="../social_contracts/post.md#social_contracts_post_revenue_manifest">revenue_manifest</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>);
    <b>if</b> (option::is_none(&manifest_opt)) {
        <b>let</b> tip_coins = coin::split(coins, amount, ctx);
        transfer::public_transfer(tip_coins, intended_recipient);
        <b>return</b> amount
    };
    <b>let</b> manifest = option::borrow(&manifest_opt);
    <b>let</b> entries = <a href="../social_contracts/media_asset.md#social_contracts_media_asset_manifest_entries">media_asset::manifest_entries</a>(manifest);
    <b>let</b> len = vector::length(entries);
    <b>let</b> bps_total = <a href="../social_contracts/media_asset.md#social_contracts_media_asset_manifest_bps_total">media_asset::manifest_bps_total</a>();
    <b>let</b> coin_type = type_name::with_defining_ids&lt;T&gt;();
    <b>let</b> <b>mut</b> tip_coins = coin::split(coins, amount, ctx);
    <b>let</b> <b>mut</b> owner_received = 0u64;
    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (i &lt; len) {
        <b>let</b> e = vector::borrow(entries, i);
        <b>let</b> slice = (amount * <a href="../social_contracts/media_asset.md#social_contracts_media_asset_manifest_entry_share_bps">media_asset::manifest_entry_share_bps</a>(e)) / bps_total;
        <b>if</b> (slice &gt; 0) {
            <b>let</b> pay_coins = coin::split(&<b>mut</b> tip_coins, slice, ctx);
            <b>if</b> (<a href="../social_contracts/media_asset.md#social_contracts_media_asset_manifest_entry_payout_mode">media_asset::manifest_entry_payout_mode</a>(e) == <a href="../social_contracts/media_asset.md#social_contracts_media_asset_payout_escrow">media_asset::payout_escrow</a>()) {
                <b>assert</b>!(<a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_beneficiary_address">poc_vault::beneficiary_address</a>(beneficiary_vault) == <a href="../social_contracts/media_asset.md#social_contracts_media_asset_manifest_entry_beneficiary">media_asset::manifest_entry_beneficiary</a>(e), <a href="../social_contracts/post.md#social_contracts_post_EWrongBeneficiaryVault">EWrongBeneficiaryVault</a>);
                <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_deposit_coin">poc_vault::deposit_coin</a>&lt;T&gt;(
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
                event::emit(<a href="../social_contracts/post.md#social_contracts_post_TipEvent">TipEvent</a> {
                    object_id,
                    from: tipper,
                    to: <a href="../social_contracts/media_asset.md#social_contracts_media_asset_manifest_entry_beneficiary">media_asset::manifest_entry_beneficiary</a>(e),
                    amount: slice,
                    coin_type,
                    is_post: is_post_event,
                });
            };
            <b>if</b> (<a href="../social_contracts/media_asset.md#social_contracts_media_asset_manifest_entry_beneficiary">media_asset::manifest_entry_beneficiary</a>(e) == intended_recipient) {
                owner_received = owner_received + slice;
            };
        };
        i = i + 1;
    };
    <b>let</b> remainder = coin::value(&tip_coins);
    <b>if</b> (remainder &gt; 0) {
        transfer::public_transfer(tip_coins, intended_recipient);
        owner_received = owner_received + remainder;
    } <b>else</b> {
        coin::destroy_zero(tip_coins);
    };
    owner_received
}
</code></pre>



</details>

<a name="social_contracts_post_apply_revenue_manifest_coin_without_beneficiary_vault"></a>

## Function `apply_revenue_manifest_coin_without_beneficiary_vault`

Same as [<code><a href="../social_contracts/post.md#social_contracts_post_apply_revenue_manifest_coin">apply_revenue_manifest_coin</a></code>] for tip paths that never deposit into escrow for this <code>amount</code>.


<pre><code><b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_apply_revenue_manifest_coin_without_beneficiary_vault">apply_revenue_manifest_coin_without_beneficiary_vault</a>&lt;T&gt;(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, intended_recipient: <b>address</b>, amount: u64, coins: &<b>mut</b> <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;T&gt;, tipper: <b>address</b>, object_id: <b>address</b>, is_post_event: bool, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_apply_revenue_manifest_coin_without_beneficiary_vault">apply_revenue_manifest_coin_without_beneficiary_vault</a>&lt;T&gt;(
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>,
    intended_recipient: <b>address</b>,
    amount: u64,
    coins: &<b>mut</b> Coin&lt;T&gt;,
    tipper: <b>address</b>,
    object_id: <b>address</b>,
    is_post_event: bool,
    ctx: &<b>mut</b> TxContext
): u64 {
    <b>if</b> (intended_recipient != <a href="../social_contracts/post.md#social_contracts_post">post</a>.owner) {
        <b>let</b> tip_coins = coin::split(coins, amount, ctx);
        transfer::public_transfer(tip_coins, intended_recipient);
        <b>return</b> amount
    };
    <b>if</b> (!<a href="../social_contracts/post.md#social_contracts_post_monetization_enabled">monetization_enabled</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>)) {
        <b>let</b> tip_coins = coin::split(coins, amount, ctx);
        transfer::public_transfer(tip_coins, intended_recipient);
        <b>return</b> amount
    };
    <b>let</b> manifest_opt = <a href="../social_contracts/post.md#social_contracts_post_revenue_manifest">revenue_manifest</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>);
    <b>if</b> (option::is_none(&manifest_opt)) {
        <b>let</b> tip_coins = coin::split(coins, amount, ctx);
        transfer::public_transfer(tip_coins, intended_recipient);
        <b>return</b> amount
    };
    <b>let</b> manifest = option::borrow(&manifest_opt);
    <b>let</b> entries = <a href="../social_contracts/media_asset.md#social_contracts_media_asset_manifest_entries">media_asset::manifest_entries</a>(manifest);
    <b>let</b> len = vector::length(entries);
    <b>let</b> bps_total = <a href="../social_contracts/media_asset.md#social_contracts_media_asset_manifest_bps_total">media_asset::manifest_bps_total</a>();
    <b>let</b> coin_type = type_name::with_defining_ids&lt;T&gt;();
    <b>let</b> <b>mut</b> tip_coins = coin::split(coins, amount, ctx);
    <b>let</b> <b>mut</b> owner_received = 0u64;
    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (i &lt; len) {
        <b>let</b> e = vector::borrow(entries, i);
        <b>let</b> slice = (amount * <a href="../social_contracts/media_asset.md#social_contracts_media_asset_manifest_entry_share_bps">media_asset::manifest_entry_share_bps</a>(e)) / bps_total;
        <b>if</b> (slice &gt; 0) {
            <b>if</b> (<a href="../social_contracts/media_asset.md#social_contracts_media_asset_manifest_entry_payout_mode">media_asset::manifest_entry_payout_mode</a>(e) == <a href="../social_contracts/media_asset.md#social_contracts_media_asset_payout_escrow">media_asset::payout_escrow</a>()) {
                <b>abort</b> <a href="../social_contracts/post.md#social_contracts_post_ETipPostRequiresBeneficiaryVault">ETipPostRequiresBeneficiaryVault</a>
            };
            <b>let</b> pay_coins = coin::split(&<b>mut</b> tip_coins, slice, ctx);
            transfer::public_transfer(pay_coins, <a href="../social_contracts/media_asset.md#social_contracts_media_asset_manifest_entry_beneficiary">media_asset::manifest_entry_beneficiary</a>(e));
            event::emit(<a href="../social_contracts/post.md#social_contracts_post_TipEvent">TipEvent</a> {
                object_id,
                from: tipper,
                to: <a href="../social_contracts/media_asset.md#social_contracts_media_asset_manifest_entry_beneficiary">media_asset::manifest_entry_beneficiary</a>(e),
                amount: slice,
                coin_type,
                is_post: is_post_event,
            });
            <b>if</b> (<a href="../social_contracts/media_asset.md#social_contracts_media_asset_manifest_entry_beneficiary">media_asset::manifest_entry_beneficiary</a>(e) == intended_recipient) {
                owner_received = owner_received + slice;
            };
        };
        i = i + 1;
    };
    <b>let</b> remainder = coin::value(&tip_coins);
    <b>if</b> (remainder &gt; 0) {
        transfer::public_transfer(tip_coins, intended_recipient);
        owner_received = owner_received + remainder;
    } <b>else</b> {
        coin::destroy_zero(tip_coins);
    };
    owner_received
}
</code></pre>



</details>

<a name="social_contracts_post_embedded_bindings"></a>

## Function `embedded_bindings`



<pre><code><b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_embedded_bindings">embedded_bindings</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>): vector&lt;<a href="../social_contracts/post.md#social_contracts_post_EmbeddedAssetBinding">social_contracts::post::EmbeddedAssetBinding</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_embedded_bindings">embedded_bindings</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>): vector&lt;<a href="../social_contracts/post.md#social_contracts_post_EmbeddedAssetBinding">EmbeddedAssetBinding</a>&gt; {
    <b>if</b> (df::exists_with_type&lt;vector&lt;u8&gt;, vector&lt;<a href="../social_contracts/post.md#social_contracts_post_EmbeddedAssetBinding">EmbeddedAssetBinding</a>&gt;&gt;(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id, <a href="../social_contracts/post.md#social_contracts_post_EMBEDDED_BINDINGS_DF_KEY">EMBEDDED_BINDINGS_DF_KEY</a>)) {
        *df::borrow(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id, <a href="../social_contracts/post.md#social_contracts_post_EMBEDDED_BINDINGS_DF_KEY">EMBEDDED_BINDINGS_DF_KEY</a>)
    } <b>else</b> {
        vector[]
    }
}
</code></pre>



</details>

<a name="social_contracts_post_usage_decisions"></a>

## Function `usage_decisions`



<pre><code><b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_usage_decisions">usage_decisions</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>): vector&lt;<a href="../social_contracts/post.md#social_contracts_post_UsageDecisionSnapshot">social_contracts::post::UsageDecisionSnapshot</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_usage_decisions">usage_decisions</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>): vector&lt;<a href="../social_contracts/post.md#social_contracts_post_UsageDecisionSnapshot">UsageDecisionSnapshot</a>&gt; {
    <b>if</b> (df::exists_with_type&lt;vector&lt;u8&gt;, vector&lt;<a href="../social_contracts/post.md#social_contracts_post_UsageDecisionSnapshot">UsageDecisionSnapshot</a>&gt;&gt;(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id, <a href="../social_contracts/post.md#social_contracts_post_USAGE_DECISIONS_DF_KEY">USAGE_DECISIONS_DF_KEY</a>)) {
        *df::borrow(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id, <a href="../social_contracts/post.md#social_contracts_post_USAGE_DECISIONS_DF_KEY">USAGE_DECISIONS_DF_KEY</a>)
    } <b>else</b> {
        vector[]
    }
}
</code></pre>



</details>

<a name="social_contracts_post_usage_denials"></a>

## Function `usage_denials`



<pre><code><b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_usage_denials">usage_denials</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>): vector&lt;<a href="../social_contracts/post.md#social_contracts_post_ContainerUsageDenial">social_contracts::post::ContainerUsageDenial</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_usage_denials">usage_denials</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>): vector&lt;<a href="../social_contracts/post.md#social_contracts_post_ContainerUsageDenial">ContainerUsageDenial</a>&gt; {
    <b>if</b> (df::exists_with_type&lt;vector&lt;u8&gt;, vector&lt;<a href="../social_contracts/post.md#social_contracts_post_ContainerUsageDenial">ContainerUsageDenial</a>&gt;&gt;(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id, <a href="../social_contracts/post.md#social_contracts_post_USAGE_DENIALS_DF_KEY">USAGE_DENIALS_DF_KEY</a>)) {
        *df::borrow(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id, <a href="../social_contracts/post.md#social_contracts_post_USAGE_DENIALS_DF_KEY">USAGE_DENIALS_DF_KEY</a>)
    } <b>else</b> {
        vector[]
    }
}
</code></pre>



</details>

<a name="social_contracts_post_set_embedded_bindings"></a>

## Function `set_embedded_bindings`



<pre><code><b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_set_embedded_bindings">set_embedded_bindings</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, bindings: vector&lt;<a href="../social_contracts/post.md#social_contracts_post_EmbeddedAssetBinding">social_contracts::post::EmbeddedAssetBinding</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_set_embedded_bindings">set_embedded_bindings</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>, bindings: vector&lt;<a href="../social_contracts/post.md#social_contracts_post_EmbeddedAssetBinding">EmbeddedAssetBinding</a>&gt;) {
    <b>if</b> (df::exists_with_type&lt;vector&lt;u8&gt;, vector&lt;<a href="../social_contracts/post.md#social_contracts_post_EmbeddedAssetBinding">EmbeddedAssetBinding</a>&gt;&gt;(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id, <a href="../social_contracts/post.md#social_contracts_post_EMBEDDED_BINDINGS_DF_KEY">EMBEDDED_BINDINGS_DF_KEY</a>)) {
        <b>let</b> stored = df::borrow_mut(&<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post">post</a>.id, <a href="../social_contracts/post.md#social_contracts_post_EMBEDDED_BINDINGS_DF_KEY">EMBEDDED_BINDINGS_DF_KEY</a>);
        *stored = bindings;
    } <b>else</b> {
        df::add(&<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post">post</a>.id, <a href="../social_contracts/post.md#social_contracts_post_EMBEDDED_BINDINGS_DF_KEY">EMBEDDED_BINDINGS_DF_KEY</a>, bindings);
    };
}
</code></pre>



</details>

<a name="social_contracts_post_set_usage_decisions"></a>

## Function `set_usage_decisions`



<pre><code><b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_set_usage_decisions">set_usage_decisions</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, decisions: vector&lt;<a href="../social_contracts/post.md#social_contracts_post_UsageDecisionSnapshot">social_contracts::post::UsageDecisionSnapshot</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_set_usage_decisions">set_usage_decisions</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>, decisions: vector&lt;<a href="../social_contracts/post.md#social_contracts_post_UsageDecisionSnapshot">UsageDecisionSnapshot</a>&gt;) {
    <b>if</b> (df::exists_with_type&lt;vector&lt;u8&gt;, vector&lt;<a href="../social_contracts/post.md#social_contracts_post_UsageDecisionSnapshot">UsageDecisionSnapshot</a>&gt;&gt;(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id, <a href="../social_contracts/post.md#social_contracts_post_USAGE_DECISIONS_DF_KEY">USAGE_DECISIONS_DF_KEY</a>)) {
        <b>let</b> stored = df::borrow_mut(&<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post">post</a>.id, <a href="../social_contracts/post.md#social_contracts_post_USAGE_DECISIONS_DF_KEY">USAGE_DECISIONS_DF_KEY</a>);
        *stored = decisions;
    } <b>else</b> {
        df::add(&<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post">post</a>.id, <a href="../social_contracts/post.md#social_contracts_post_USAGE_DECISIONS_DF_KEY">USAGE_DECISIONS_DF_KEY</a>, decisions);
    };
}
</code></pre>



</details>

<a name="social_contracts_post_set_usage_denials"></a>

## Function `set_usage_denials`



<pre><code><b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_set_usage_denials">set_usage_denials</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, denials: vector&lt;<a href="../social_contracts/post.md#social_contracts_post_ContainerUsageDenial">social_contracts::post::ContainerUsageDenial</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_set_usage_denials">set_usage_denials</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>, denials: vector&lt;<a href="../social_contracts/post.md#social_contracts_post_ContainerUsageDenial">ContainerUsageDenial</a>&gt;) {
    <b>if</b> (df::exists_with_type&lt;vector&lt;u8&gt;, vector&lt;<a href="../social_contracts/post.md#social_contracts_post_ContainerUsageDenial">ContainerUsageDenial</a>&gt;&gt;(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id, <a href="../social_contracts/post.md#social_contracts_post_USAGE_DENIALS_DF_KEY">USAGE_DENIALS_DF_KEY</a>)) {
        <b>let</b> stored = df::borrow_mut(&<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post">post</a>.id, <a href="../social_contracts/post.md#social_contracts_post_USAGE_DENIALS_DF_KEY">USAGE_DENIALS_DF_KEY</a>);
        *stored = denials;
    } <b>else</b> {
        df::add(&<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post">post</a>.id, <a href="../social_contracts/post.md#social_contracts_post_USAGE_DENIALS_DF_KEY">USAGE_DENIALS_DF_KEY</a>, denials);
    };
}
</code></pre>



</details>

<a name="social_contracts_post_find_binding_index"></a>

## Function `find_binding_index`



<pre><code><b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_find_binding_index">find_binding_index</a>(bindings: &vector&lt;<a href="../social_contracts/post.md#social_contracts_post_EmbeddedAssetBinding">social_contracts::post::EmbeddedAssetBinding</a>&gt;, binding_id: u64): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_find_binding_index">find_binding_index</a>(bindings: &vector&lt;<a href="../social_contracts/post.md#social_contracts_post_EmbeddedAssetBinding">EmbeddedAssetBinding</a>&gt;, binding_id: u64): Option&lt;u64&gt; {
    <b>let</b> len = vector::length(bindings);
    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (i &lt; len) {
        <b>if</b> (vector::borrow(bindings, i).binding_id == binding_id) {
            <b>return</b> option::some(i)
        };
        i = i + 1;
    };
    option::none()
}
</code></pre>



</details>

<a name="social_contracts_post_borrow_binding"></a>

## Function `borrow_binding`



<pre><code><b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_borrow_binding">borrow_binding</a>(bindings: &vector&lt;<a href="../social_contracts/post.md#social_contracts_post_EmbeddedAssetBinding">social_contracts::post::EmbeddedAssetBinding</a>&gt;, binding_id: u64): &<a href="../social_contracts/post.md#social_contracts_post_EmbeddedAssetBinding">social_contracts::post::EmbeddedAssetBinding</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_borrow_binding">borrow_binding</a>(bindings: &vector&lt;<a href="../social_contracts/post.md#social_contracts_post_EmbeddedAssetBinding">EmbeddedAssetBinding</a>&gt;, binding_id: u64): &<a href="../social_contracts/post.md#social_contracts_post_EmbeddedAssetBinding">EmbeddedAssetBinding</a> {
    <b>let</b> idx = option::borrow(&<a href="../social_contracts/post.md#social_contracts_post_find_binding_index">find_binding_index</a>(bindings, binding_id));
    vector::borrow(bindings, *idx)
}
</code></pre>



</details>

<a name="social_contracts_post_upsert_binding"></a>

## Function `upsert_binding`



<pre><code><b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_upsert_binding">upsert_binding</a>(bindings: &<b>mut</b> vector&lt;<a href="../social_contracts/post.md#social_contracts_post_EmbeddedAssetBinding">social_contracts::post::EmbeddedAssetBinding</a>&gt;, binding: <a href="../social_contracts/post.md#social_contracts_post_EmbeddedAssetBinding">social_contracts::post::EmbeddedAssetBinding</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_upsert_binding">upsert_binding</a>(bindings: &<b>mut</b> vector&lt;<a href="../social_contracts/post.md#social_contracts_post_EmbeddedAssetBinding">EmbeddedAssetBinding</a>&gt;, binding: <a href="../social_contracts/post.md#social_contracts_post_EmbeddedAssetBinding">EmbeddedAssetBinding</a>) {
    <b>let</b> binding_id = binding.binding_id;
    <b>if</b> (option::is_some(&<a href="../social_contracts/post.md#social_contracts_post_find_binding_index">find_binding_index</a>(bindings, binding_id))) {
        <b>let</b> idx = *option::borrow(&<a href="../social_contracts/post.md#social_contracts_post_find_binding_index">find_binding_index</a>(bindings, binding_id));
        *vector::borrow_mut(bindings, idx) = binding;
    } <b>else</b> {
        vector::push_back(bindings, binding);
    };
}
</code></pre>



</details>

<a name="social_contracts_post_upsert_decision"></a>

## Function `upsert_decision`



<pre><code><b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_upsert_decision">upsert_decision</a>(decisions: &<b>mut</b> vector&lt;<a href="../social_contracts/post.md#social_contracts_post_UsageDecisionSnapshot">social_contracts::post::UsageDecisionSnapshot</a>&gt;, snapshot: <a href="../social_contracts/post.md#social_contracts_post_UsageDecisionSnapshot">social_contracts::post::UsageDecisionSnapshot</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_upsert_decision">upsert_decision</a>(decisions: &<b>mut</b> vector&lt;<a href="../social_contracts/post.md#social_contracts_post_UsageDecisionSnapshot">UsageDecisionSnapshot</a>&gt;, snapshot: <a href="../social_contracts/post.md#social_contracts_post_UsageDecisionSnapshot">UsageDecisionSnapshot</a>) {
    <b>let</b> binding_id = snapshot.binding_id;
    <b>let</b> len = vector::length(decisions);
    <b>let</b> <b>mut</b> i = 0;
    <b>let</b> <b>mut</b> found = <b>false</b>;
    <b>while</b> (i &lt; len) {
        <b>if</b> (vector::borrow(decisions, i).binding_id == binding_id) {
            *vector::borrow_mut(decisions, i) = snapshot;
            found = <b>true</b>;
            <b>break</b>
        };
        i = i + 1;
    };
    <b>if</b> (!found) {
        vector::push_back(decisions, snapshot);
    };
}
</code></pre>



</details>

<a name="social_contracts_post_upsert_denial"></a>

## Function `upsert_denial`



<pre><code><b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_upsert_denial">upsert_denial</a>(denials: &<b>mut</b> vector&lt;<a href="../social_contracts/post.md#social_contracts_post_ContainerUsageDenial">social_contracts::post::ContainerUsageDenial</a>&gt;, denial: <a href="../social_contracts/post.md#social_contracts_post_ContainerUsageDenial">social_contracts::post::ContainerUsageDenial</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_upsert_denial">upsert_denial</a>(denials: &<b>mut</b> vector&lt;<a href="../social_contracts/post.md#social_contracts_post_ContainerUsageDenial">ContainerUsageDenial</a>&gt;, denial: <a href="../social_contracts/post.md#social_contracts_post_ContainerUsageDenial">ContainerUsageDenial</a>) {
    <b>let</b> binding_id = denial.binding_id;
    <b>let</b> len = vector::length(denials);
    <b>let</b> <b>mut</b> i = 0;
    <b>let</b> <b>mut</b> found = <b>false</b>;
    <b>while</b> (i &lt; len) {
        <b>if</b> (vector::borrow(denials, i).binding_id == binding_id) {
            *vector::borrow_mut(denials, i) = denial;
            found = <b>true</b>;
            <b>break</b>
        };
        i = i + 1;
    };
    <b>if</b> (!found) {
        vector::push_back(denials, denial);
    };
}
</code></pre>



</details>

<a name="social_contracts_post_remove_denial"></a>

## Function `remove_denial`



<pre><code><b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_remove_denial">remove_denial</a>(denials: &<b>mut</b> vector&lt;<a href="../social_contracts/post.md#social_contracts_post_ContainerUsageDenial">social_contracts::post::ContainerUsageDenial</a>&gt;, binding_id: u64, denial_scope: u8)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_remove_denial">remove_denial</a>(denials: &<b>mut</b> vector&lt;<a href="../social_contracts/post.md#social_contracts_post_ContainerUsageDenial">ContainerUsageDenial</a>&gt;, binding_id: u64, denial_scope: u8) {
    <b>let</b> <b>mut</b> out = vector[];
    <b>let</b> len = vector::length(denials);
    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (i &lt; len) {
        <b>let</b> d = vector::borrow(denials, i);
        <b>if</b> (!(d.binding_id == binding_id && d.denial_scope == denial_scope)) {
            vector::push_back(&<b>mut</b> out, *d);
        };
        i = i + 1;
    };
    *denials = out;
}
</code></pre>



</details>

<a name="social_contracts_post_apply_denials_to_snapshot"></a>

## Function `apply_denials_to_snapshot`



<pre><code><b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_apply_denials_to_snapshot">apply_denials_to_snapshot</a>(policy_playback: bool, denials: &vector&lt;<a href="../social_contracts/post.md#social_contracts_post_ContainerUsageDenial">social_contracts::post::ContainerUsageDenial</a>&gt;, binding_id: u64): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_apply_denials_to_snapshot">apply_denials_to_snapshot</a>(
    policy_playback: bool,
    denials: &vector&lt;<a href="../social_contracts/post.md#social_contracts_post_ContainerUsageDenial">ContainerUsageDenial</a>&gt;,
    binding_id: u64,
): bool {
    <b>let</b> <b>mut</b> playback = policy_playback;
    <b>let</b> len = vector::length(denials);
    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (i &lt; len) {
        <b>let</b> d = vector::borrow(denials, i);
        <b>if</b> (d.binding_id == binding_id && d.denial_scope == <a href="../social_contracts/post.md#social_contracts_post_DENIAL_SCOPE_PLAYBACK">DENIAL_SCOPE_PLAYBACK</a>) {
            playback = <b>false</b>;
        };
        i = i + 1;
    };
    playback
}
</code></pre>



</details>

<a name="social_contracts_post_evaluate_binding_policy"></a>

## Function `evaluate_binding_policy`



<pre><code><b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_evaluate_binding_policy">evaluate_binding_policy</a>(asset: &<a href="../social_contracts/media_asset.md#social_contracts_media_asset_MediaAsset">social_contracts::media_asset::MediaAsset</a>, usage_class: u8, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>): (bool, u8, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_evaluate_binding_policy">evaluate_binding_policy</a>(
    asset: &MediaAsset,
    usage_class: u8,
    clock: &Clock,
): (bool, u8, u64) {
    <b>let</b> <a href="../social_contracts/post.md#social_contracts_post_version">version</a> = <b>if</b> (option::is_some(&<a href="../social_contracts/media_asset.md#social_contracts_media_asset_resolved_policy_version">media_asset::resolved_policy_version</a>(asset))) {
        *option::borrow(&<a href="../social_contracts/media_asset.md#social_contracts_media_asset_resolved_policy_version">media_asset::resolved_policy_version</a>(asset))
    } <b>else</b> {
        0
    };
    <b>let</b> grant_playback = <a href="../social_contracts/media_asset.md#social_contracts_media_asset_rights_permits_usage">media_asset::rights_permits_usage</a>(asset, usage_class, clock);
    <b>if</b> (!grant_playback) {
        <b>return</b> (<b>false</b>, <a href="../social_contracts/post.md#social_contracts_post_REASON_NO_GRANT">REASON_NO_GRANT</a>, <a href="../social_contracts/post.md#social_contracts_post_version">version</a>)
    };
    <b>if</b> (!<a href="../social_contracts/media_asset.md#social_contracts_media_asset_resolved_policy_permits_usage">media_asset::resolved_policy_permits_usage</a>(asset, usage_class)) {
        <b>return</b> (<b>false</b>, <a href="../social_contracts/post.md#social_contracts_post_REASON_RESOLVED_POLICY">REASON_RESOLVED_POLICY</a>, <a href="../social_contracts/post.md#social_contracts_post_version">version</a>)
    };
    (<b>true</b>, <a href="../social_contracts/post.md#social_contracts_post_REASON_ALLOWED">REASON_ALLOWED</a>, <a href="../social_contracts/post.md#social_contracts_post_version">version</a>)
}
</code></pre>



</details>

<a name="social_contracts_post_refresh_composition_status_from_decisions"></a>

## Function `refresh_composition_status_from_decisions`



<pre><code><b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_refresh_composition_status_from_decisions">refresh_composition_status_from_decisions</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_refresh_composition_status_from_decisions">refresh_composition_status_from_decisions</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>) {
    <b>let</b> decisions = <a href="../social_contracts/post.md#social_contracts_post_usage_decisions">usage_decisions</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>);
    <b>let</b> len = vector::length(&decisions);
    <b>if</b> (len == 0) {
        <b>return</b>
    };
    <b>let</b> <b>mut</b> any_restricted = <b>false</b>;
    <b>let</b> <b>mut</b> all_restricted = <b>true</b>;
    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (i &lt; len) {
        <b>let</b> d = vector::borrow(&decisions, i);
        <b>let</b> restricted = !d.playback_permitted;
        <b>if</b> (restricted) {
            any_restricted = <b>true</b>;
        } <b>else</b> {
            all_restricted = <b>false</b>;
        };
        i = i + 1;
    };
    <b>if</b> (all_restricted) {
        <a href="../social_contracts/post.md#social_contracts_post">post</a>.<a href="../social_contracts/post.md#social_contracts_post_composition_status">composition_status</a> = <a href="../social_contracts/media_asset.md#social_contracts_media_asset_composition_invalid">media_asset::composition_invalid</a>();
    } <b>else</b> <b>if</b> (any_restricted) {
        <a href="../social_contracts/post.md#social_contracts_post">post</a>.<a href="../social_contracts/post.md#social_contracts_post_composition_status">composition_status</a> = <a href="../social_contracts/media_asset.md#social_contracts_media_asset_composition_partially_restricted">media_asset::composition_partially_restricted</a>();
    };
}
</code></pre>



</details>

<a name="social_contracts_post_validate_binding"></a>

## Function `validate_binding`



<pre><code><b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_validate_binding">validate_binding</a>(binding: &<a href="../social_contracts/post.md#social_contracts_post_EmbeddedAssetBinding">social_contracts::post::EmbeddedAssetBinding</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_validate_binding">validate_binding</a>(binding: &<a href="../social_contracts/post.md#social_contracts_post_EmbeddedAssetBinding">EmbeddedAssetBinding</a>) {
    <b>assert</b>!(binding.binding_id &gt; 0, <a href="../social_contracts/post.md#social_contracts_post_EInvalidBinding">EInvalidBinding</a>);
    <a href="../social_contracts/media_asset.md#social_contracts_media_asset_assert_valid_usage_class">media_asset::assert_valid_usage_class</a>(binding.usage_class);
    // Oracle slot indices — must be readable at validation time <b>for</b> event/indexer ABI parity.
    <b>assert</b>!(binding.stem &lt;= 255, <a href="../social_contracts/post.md#social_contracts_post_EInvalidBinding">EInvalidBinding</a>);
    <b>assert</b>!(binding.media_component &lt;= 255, <a href="../social_contracts/post.md#social_contracts_post_EInvalidBinding">EInvalidBinding</a>);
    <a href="../social_contracts/media_asset.md#social_contracts_media_asset_assert_valid_optional_evidence_commitment">media_asset::assert_valid_optional_evidence_commitment</a>(&binding.evidence_commitment);
}
</code></pre>



</details>

<a name="social_contracts_post_write_usage_decision_for_binding"></a>

## Function `write_usage_decision_for_binding`



<pre><code><b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_write_usage_decision_for_binding">write_usage_decision_for_binding</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, asset: &<a href="../social_contracts/media_asset.md#social_contracts_media_asset_MediaAsset">social_contracts::media_asset::MediaAsset</a>, binding_id: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, timestamp: u64): <a href="../social_contracts/post.md#social_contracts_post_UsageDecisionSnapshot">social_contracts::post::UsageDecisionSnapshot</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_write_usage_decision_for_binding">write_usage_decision_for_binding</a>(
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>,
    asset: &MediaAsset,
    binding_id: u64,
    clock: &Clock,
    timestamp: u64,
): <a href="../social_contracts/post.md#social_contracts_post_UsageDecisionSnapshot">UsageDecisionSnapshot</a> {
    <b>let</b> bindings = <a href="../social_contracts/post.md#social_contracts_post_embedded_bindings">embedded_bindings</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>);
    <b>assert</b>!(option::is_some(&<a href="../social_contracts/post.md#social_contracts_post_find_binding_index">find_binding_index</a>(&bindings, binding_id)), <a href="../social_contracts/post.md#social_contracts_post_EBindingNotFound">EBindingNotFound</a>);
    <b>let</b> binding = <a href="../social_contracts/post.md#social_contracts_post_borrow_binding">borrow_binding</a>(&bindings, binding_id);
    <b>assert</b>!(binding.source_asset_id == object::id(asset), <a href="../social_contracts/post.md#social_contracts_post_EBindingAssetMismatch">EBindingAssetMismatch</a>);
    <b>let</b> (policy_playback, reason_code, policy_version) =
        <a href="../social_contracts/post.md#social_contracts_post_evaluate_binding_policy">evaluate_binding_policy</a>(asset, binding.usage_class, clock);
    <b>let</b> denials = <a href="../social_contracts/post.md#social_contracts_post_usage_denials">usage_denials</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>);
    <b>let</b> playback = <a href="../social_contracts/post.md#social_contracts_post_apply_denials_to_snapshot">apply_denials_to_snapshot</a>(policy_playback, &denials, binding_id);
    <b>let</b> reason = <b>if</b> (playback != policy_playback) {
        <a href="../social_contracts/post.md#social_contracts_post_REASON_DENIAL">REASON_DENIAL</a>
    } <b>else</b> {
        reason_code
    };
    <b>let</b> snapshot = <a href="../social_contracts/post.md#social_contracts_post_UsageDecisionSnapshot">UsageDecisionSnapshot</a> {
        binding_id,
        policy_playback_permitted: policy_playback,
        playback_permitted: playback,
        policy_reason_code: reason,
        policy_version_at_decision: policy_version,
    };
    <b>let</b> <b>mut</b> decisions = <a href="../social_contracts/post.md#social_contracts_post_usage_decisions">usage_decisions</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>);
    <a href="../social_contracts/post.md#social_contracts_post_upsert_decision">upsert_decision</a>(&<b>mut</b> decisions, snapshot);
    <a href="../social_contracts/post.md#social_contracts_post_set_usage_decisions">set_usage_decisions</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>, decisions);
    <a href="../social_contracts/post.md#social_contracts_post_refresh_composition_status_from_decisions">refresh_composition_status_from_decisions</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>);
    event::emit(<a href="../social_contracts/post.md#social_contracts_post_UsageDecisionRefreshedEvent">UsageDecisionRefreshedEvent</a> {
        post_id: <a href="../social_contracts/post.md#social_contracts_post_get_id_address">get_id_address</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>),
        binding_id,
        policy_playback_permitted: policy_playback,
        playback_permitted: playback,
        policy_reason_code: reason,
        policy_version_at_decision: policy_version,
        timestamp,
    });
    snapshot
}
</code></pre>



</details>

<a name="social_contracts_post_record_embedded_bindings"></a>

## Function `record_embedded_bindings`

Oracle records embedded asset bindings detected during composition analysis.


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_record_embedded_bindings">record_embedded_bindings</a>(oracle: <b>address</b>, <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, bindings: vector&lt;<a href="../social_contracts/post.md#social_contracts_post_EmbeddedAssetBinding">social_contracts::post::EmbeddedAssetBinding</a>&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_record_embedded_bindings">record_embedded_bindings</a>(
    oracle: <b>address</b>,
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>,
    bindings: vector&lt;<a href="../social_contracts/post.md#social_contracts_post_EmbeddedAssetBinding">EmbeddedAssetBinding</a>&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(tx_context::sender(ctx) == oracle, <a href="../social_contracts/post.md#social_contracts_post_EUnauthorized">EUnauthorized</a>);
    <b>let</b> timestamp = clock::timestamp_ms(clock);
    <b>let</b> <b>mut</b> stored = <a href="../social_contracts/post.md#social_contracts_post_embedded_bindings">embedded_bindings</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>);
    <b>let</b> len = vector::length(&bindings);
    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (i &lt; len) {
        <b>let</b> b = vector::borrow(&bindings, i);
        <a href="../social_contracts/post.md#social_contracts_post_validate_binding">validate_binding</a>(b);
        <a href="../social_contracts/post.md#social_contracts_post_upsert_binding">upsert_binding</a>(&<b>mut</b> stored, *b);
        i = i + 1;
    };
    <a href="../social_contracts/post.md#social_contracts_post_set_embedded_bindings">set_embedded_bindings</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>, stored);
    event::emit(<a href="../social_contracts/post.md#social_contracts_post_EmbeddedBindingRecordedEvent">EmbeddedBindingRecordedEvent</a> {
        post_id: <a href="../social_contracts/post.md#social_contracts_post_get_id_address">get_id_address</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>),
        bindings: <a href="../social_contracts/post.md#social_contracts_post_embedded_bindings">embedded_bindings</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>),
        timestamp,
    });
}
</code></pre>



</details>

<a name="social_contracts_post_refresh_post_asset_usage_decision"></a>

## Function `refresh_post_asset_usage_decision`

Oracle refreshes a usage decision snapshot for one binding (requires live <code>MediaAsset</code>).


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_refresh_post_asset_usage_decision">refresh_post_asset_usage_decision</a>(oracle: <b>address</b>, <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, asset: &<a href="../social_contracts/media_asset.md#social_contracts_media_asset_MediaAsset">social_contracts::media_asset::MediaAsset</a>, binding_id: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_refresh_post_asset_usage_decision">refresh_post_asset_usage_decision</a>(
    oracle: <b>address</b>,
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>,
    asset: &MediaAsset,
    binding_id: u64,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(tx_context::sender(ctx) == oracle, <a href="../social_contracts/post.md#social_contracts_post_EUnauthorized">EUnauthorized</a>);
    <b>let</b> timestamp = clock::timestamp_ms(clock);
    <b>let</b> _ = <a href="../social_contracts/post.md#social_contracts_post_write_usage_decision_for_binding">write_usage_decision_for_binding</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>, asset, binding_id, clock, timestamp);
}
</code></pre>



</details>

<a name="social_contracts_post_submit_candidate_revenue_manifest"></a>

## Function `submit_candidate_revenue_manifest`

Oracle submits a candidate revenue manifest for indexer materialization.


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_submit_candidate_revenue_manifest">submit_candidate_revenue_manifest</a>(oracle: <b>address</b>, <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, manifest: <a href="../social_contracts/media_asset.md#social_contracts_media_asset_RevenueManifest">social_contracts::media_asset::RevenueManifest</a>, manifest_version: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_submit_candidate_revenue_manifest">submit_candidate_revenue_manifest</a>(
    oracle: <b>address</b>,
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>,
    manifest: RevenueManifest,
    manifest_version: u64,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(tx_context::sender(ctx) == oracle, <a href="../social_contracts/post.md#social_contracts_post_EUnauthorized">EUnauthorized</a>);
    <a href="../social_contracts/media_asset.md#social_contracts_media_asset_validate_revenue_manifest">media_asset::validate_revenue_manifest</a>(&manifest);
    <b>let</b> timestamp = clock::timestamp_ms(clock);
    <b>let</b> entries = *<a href="../social_contracts/media_asset.md#social_contracts_media_asset_manifest_entries">media_asset::manifest_entries</a>(&manifest);
    <b>if</b> (df::exists_with_type&lt;vector&lt;u8&gt;, RevenueManifest&gt;(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id, <a href="../social_contracts/post.md#social_contracts_post_CANDIDATE_MANIFEST_DF_KEY">CANDIDATE_MANIFEST_DF_KEY</a>)) {
        <b>let</b> stored = df::borrow_mut(&<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post">post</a>.id, <a href="../social_contracts/post.md#social_contracts_post_CANDIDATE_MANIFEST_DF_KEY">CANDIDATE_MANIFEST_DF_KEY</a>);
        *stored = manifest;
    } <b>else</b> {
        df::add(&<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post">post</a>.id, <a href="../social_contracts/post.md#social_contracts_post_CANDIDATE_MANIFEST_DF_KEY">CANDIDATE_MANIFEST_DF_KEY</a>, manifest);
    };
    event::emit(<a href="../social_contracts/post.md#social_contracts_post_CandidateManifestSubmittedEvent">CandidateManifestSubmittedEvent</a> {
        post_id: <a href="../social_contracts/post.md#social_contracts_post_get_id_address">get_id_address</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>),
        manifest_version,
        entries_json: entries,
        timestamp,
    });
}
</code></pre>



</details>

<a name="social_contracts_post_deny_container_usage"></a>

## Function `deny_container_usage`

Rights holder denies container-scoped usage on this post for one binding.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_deny_container_usage">deny_container_usage</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, asset: &<a href="../social_contracts/media_asset.md#social_contracts_media_asset_MediaAsset">social_contracts::media_asset::MediaAsset</a>, binding_id: u64, denial_scope: u8, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_deny_container_usage">deny_container_usage</a>(
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>,
    asset: &MediaAsset,
    binding_id: u64,
    denial_scope: u8,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(denial_scope == <a href="../social_contracts/post.md#social_contracts_post_DENIAL_SCOPE_PLAYBACK">DENIAL_SCOPE_PLAYBACK</a>, <a href="../social_contracts/post.md#social_contracts_post_EInvalidDenialScope">EInvalidDenialScope</a>);
    <b>let</b> sender = tx_context::sender(ctx);
    <b>assert</b>!(<a href="../social_contracts/media_asset.md#social_contracts_media_asset_can_update_rights">media_asset::can_update_rights</a>(asset, sender), <a href="../social_contracts/post.md#social_contracts_post_EUnauthorized">EUnauthorized</a>);
    <b>let</b> bindings = <a href="../social_contracts/post.md#social_contracts_post_embedded_bindings">embedded_bindings</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>);
    <b>assert</b>!(option::is_some(&<a href="../social_contracts/post.md#social_contracts_post_find_binding_index">find_binding_index</a>(&bindings, binding_id)), <a href="../social_contracts/post.md#social_contracts_post_EBindingNotFound">EBindingNotFound</a>);
    <b>let</b> binding = <a href="../social_contracts/post.md#social_contracts_post_borrow_binding">borrow_binding</a>(&bindings, binding_id);
    <b>assert</b>!(binding.source_asset_id == object::id(asset), <a href="../social_contracts/post.md#social_contracts_post_EBindingAssetMismatch">EBindingAssetMismatch</a>);
    <b>let</b> timestamp = clock::timestamp_ms(clock);
    <b>let</b> <b>mut</b> denials = <a href="../social_contracts/post.md#social_contracts_post_usage_denials">usage_denials</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>);
    <a href="../social_contracts/post.md#social_contracts_post_upsert_denial">upsert_denial</a>(&<b>mut</b> denials, <a href="../social_contracts/post.md#social_contracts_post_ContainerUsageDenial">ContainerUsageDenial</a> { binding_id, denial_scope });
    <a href="../social_contracts/post.md#social_contracts_post_set_usage_denials">set_usage_denials</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>, denials);
    <b>let</b> _ = <a href="../social_contracts/post.md#social_contracts_post_write_usage_decision_for_binding">write_usage_decision_for_binding</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>, asset, binding_id, clock, timestamp);
    event::emit(<a href="../social_contracts/post.md#social_contracts_post_ContainerUsageDeniedEvent">ContainerUsageDeniedEvent</a> {
        post_id: <a href="../social_contracts/post.md#social_contracts_post_get_id_address">get_id_address</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>),
        binding_id,
        denial_scope,
        timestamp,
    });
}
</code></pre>



</details>

<a name="social_contracts_post_lift_container_usage_denial"></a>

## Function `lift_container_usage_denial`

Rights holder lifts a container-scoped denial for one binding and scope.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_lift_container_usage_denial">lift_container_usage_denial</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, asset: &<a href="../social_contracts/media_asset.md#social_contracts_media_asset_MediaAsset">social_contracts::media_asset::MediaAsset</a>, binding_id: u64, denial_scope: u8, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_lift_container_usage_denial">lift_container_usage_denial</a>(
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>,
    asset: &MediaAsset,
    binding_id: u64,
    denial_scope: u8,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(denial_scope == <a href="../social_contracts/post.md#social_contracts_post_DENIAL_SCOPE_PLAYBACK">DENIAL_SCOPE_PLAYBACK</a>, <a href="../social_contracts/post.md#social_contracts_post_EInvalidDenialScope">EInvalidDenialScope</a>);
    <b>let</b> sender = tx_context::sender(ctx);
    <b>assert</b>!(<a href="../social_contracts/media_asset.md#social_contracts_media_asset_can_update_rights">media_asset::can_update_rights</a>(asset, sender), <a href="../social_contracts/post.md#social_contracts_post_EUnauthorized">EUnauthorized</a>);
    <b>let</b> bindings = <a href="../social_contracts/post.md#social_contracts_post_embedded_bindings">embedded_bindings</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>);
    <b>assert</b>!(option::is_some(&<a href="../social_contracts/post.md#social_contracts_post_find_binding_index">find_binding_index</a>(&bindings, binding_id)), <a href="../social_contracts/post.md#social_contracts_post_EBindingNotFound">EBindingNotFound</a>);
    <b>let</b> binding = <a href="../social_contracts/post.md#social_contracts_post_borrow_binding">borrow_binding</a>(&bindings, binding_id);
    <b>assert</b>!(binding.source_asset_id == object::id(asset), <a href="../social_contracts/post.md#social_contracts_post_EBindingAssetMismatch">EBindingAssetMismatch</a>);
    <b>let</b> timestamp = clock::timestamp_ms(clock);
    <b>let</b> <b>mut</b> denials = <a href="../social_contracts/post.md#social_contracts_post_usage_denials">usage_denials</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>);
    <a href="../social_contracts/post.md#social_contracts_post_remove_denial">remove_denial</a>(&<b>mut</b> denials, binding_id, denial_scope);
    <a href="../social_contracts/post.md#social_contracts_post_set_usage_denials">set_usage_denials</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>, denials);
    <b>let</b> _ = <a href="../social_contracts/post.md#social_contracts_post_write_usage_decision_for_binding">write_usage_decision_for_binding</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>, asset, binding_id, clock, timestamp);
    event::emit(<a href="../social_contracts/post.md#social_contracts_post_ContainerUsageDenialLiftedEvent">ContainerUsageDenialLiftedEvent</a> {
        post_id: <a href="../social_contracts/post.md#social_contracts_post_get_id_address">get_id_address</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>),
        binding_id,
        timestamp,
    });
}
</code></pre>



</details>

<a name="social_contracts_post_set_composition_result"></a>

## Function `set_composition_result`

Internal function to store composition analysis + optional revenue manifest (oracle-only).


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_set_composition_result">set_composition_result</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, <a href="../social_contracts/post.md#social_contracts_post_composition_status">composition_status</a>: u8, <a href="../social_contracts/post.md#social_contracts_post_monetization_status">monetization_status</a>: u8, badge: <a href="../social_contracts/media_asset.md#social_contracts_media_asset_CompositionBadgeSnapshot">social_contracts::media_asset::CompositionBadgeSnapshot</a>, analysis: <a href="../social_contracts/media_asset.md#social_contracts_media_asset_CompositionAnalysis">social_contracts::media_asset::CompositionAnalysis</a>, manifest: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../social_contracts/media_asset.md#social_contracts_media_asset_RevenueManifest">social_contracts::media_asset::RevenueManifest</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_set_composition_result">set_composition_result</a>(
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>,
    <a href="../social_contracts/post.md#social_contracts_post_composition_status">composition_status</a>: u8,
    <a href="../social_contracts/post.md#social_contracts_post_monetization_status">monetization_status</a>: u8,
    badge: CompositionBadgeSnapshot,
    analysis: CompositionAnalysis,
    manifest: Option&lt;RevenueManifest&gt;,
) {
    <a href="../social_contracts/post.md#social_contracts_post">post</a>.<a href="../social_contracts/post.md#social_contracts_post_composition_status">composition_status</a> = <a href="../social_contracts/post.md#social_contracts_post_composition_status">composition_status</a>;
    <a href="../social_contracts/post.md#social_contracts_post">post</a>.<a href="../social_contracts/post.md#social_contracts_post_monetization_status">monetization_status</a> = <a href="../social_contracts/post.md#social_contracts_post_monetization_status">monetization_status</a>;
    <a href="../social_contracts/post.md#social_contracts_post">post</a>.<a href="../social_contracts/post.md#social_contracts_post_composition_badge">composition_badge</a> = option::some(badge);
    <b>if</b> (df::exists_with_type&lt;vector&lt;u8&gt;, CompositionAnalysis&gt;(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id, <a href="../social_contracts/post.md#social_contracts_post_COMPOSITION_ANALYSIS_DF_KEY">COMPOSITION_ANALYSIS_DF_KEY</a>)) {
        <b>let</b> stored = df::borrow_mut(&<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post">post</a>.id, <a href="../social_contracts/post.md#social_contracts_post_COMPOSITION_ANALYSIS_DF_KEY">COMPOSITION_ANALYSIS_DF_KEY</a>);
        *stored = analysis;
    } <b>else</b> {
        df::add(&<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post">post</a>.id, <a href="../social_contracts/post.md#social_contracts_post_COMPOSITION_ANALYSIS_DF_KEY">COMPOSITION_ANALYSIS_DF_KEY</a>, analysis);
    };
    <b>if</b> (option::is_some(&manifest)) {
        <b>let</b> m = *option::borrow(&manifest);
        <a href="../social_contracts/media_asset.md#social_contracts_media_asset_validate_revenue_manifest">media_asset::validate_revenue_manifest</a>(&m);
        <b>if</b> (df::exists_with_type&lt;vector&lt;u8&gt;, RevenueManifest&gt;(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id, <a href="../social_contracts/post.md#social_contracts_post_REVENUE_MANIFEST_DF_KEY">REVENUE_MANIFEST_DF_KEY</a>)) {
            <b>let</b> stored = df::borrow_mut(&<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post">post</a>.id, <a href="../social_contracts/post.md#social_contracts_post_REVENUE_MANIFEST_DF_KEY">REVENUE_MANIFEST_DF_KEY</a>);
            *stored = m;
        } <b>else</b> {
            df::add(&<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post">post</a>.id, <a href="../social_contracts/post.md#social_contracts_post_REVENUE_MANIFEST_DF_KEY">REVENUE_MANIFEST_DF_KEY</a>, m);
        };
    } <b>else</b> <b>if</b> (df::exists_with_type&lt;vector&lt;u8&gt;, RevenueManifest&gt;(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id, <a href="../social_contracts/post.md#social_contracts_post_REVENUE_MANIFEST_DF_KEY">REVENUE_MANIFEST_DF_KEY</a>)) {
        df::remove&lt;vector&lt;u8&gt;, RevenueManifest&gt;(&<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post">post</a>.id, <a href="../social_contracts/post.md#social_contracts_post_REVENUE_MANIFEST_DF_KEY">REVENUE_MANIFEST_DF_KEY</a>);
    };
}
</code></pre>



</details>

<a name="social_contracts_post_clear_composition_data"></a>

## Function `clear_composition_data`

Clears composition routing data on the post after dispute resolution (vault balances are unaffected).


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_clear_composition_data">clear_composition_data</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_clear_composition_data">clear_composition_data</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>) {
    <a href="../social_contracts/post.md#social_contracts_post">post</a>.<a href="../social_contracts/post.md#social_contracts_post_composition_status">composition_status</a> = <a href="../social_contracts/media_asset.md#social_contracts_media_asset_composition_none">media_asset::composition_none</a>();
    <a href="../social_contracts/post.md#social_contracts_post">post</a>.<a href="../social_contracts/post.md#social_contracts_post_monetization_status">monetization_status</a> = <a href="../social_contracts/media_asset.md#social_contracts_media_asset_monetization_none">media_asset::monetization_none</a>();
    <a href="../social_contracts/post.md#social_contracts_post">post</a>.<a href="../social_contracts/post.md#social_contracts_post_composition_badge">composition_badge</a> = option::none();
    <b>if</b> (df::exists_with_type&lt;vector&lt;u8&gt;, CompositionAnalysis&gt;(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id, <a href="../social_contracts/post.md#social_contracts_post_COMPOSITION_ANALYSIS_DF_KEY">COMPOSITION_ANALYSIS_DF_KEY</a>)) {
        df::remove&lt;vector&lt;u8&gt;, CompositionAnalysis&gt;(&<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post">post</a>.id, <a href="../social_contracts/post.md#social_contracts_post_COMPOSITION_ANALYSIS_DF_KEY">COMPOSITION_ANALYSIS_DF_KEY</a>);
    };
    <b>if</b> (df::exists_with_type&lt;vector&lt;u8&gt;, RevenueManifest&gt;(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id, <a href="../social_contracts/post.md#social_contracts_post_REVENUE_MANIFEST_DF_KEY">REVENUE_MANIFEST_DF_KEY</a>)) {
        df::remove&lt;vector&lt;u8&gt;, RevenueManifest&gt;(&<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post">post</a>.id, <a href="../social_contracts/post.md#social_contracts_post_REVENUE_MANIFEST_DF_KEY">REVENUE_MANIFEST_DF_KEY</a>);
    };
    <b>if</b> (df::exists_with_type&lt;vector&lt;u8&gt;, vector&lt;<a href="../social_contracts/post.md#social_contracts_post_EmbeddedAssetBinding">EmbeddedAssetBinding</a>&gt;&gt;(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id, <a href="../social_contracts/post.md#social_contracts_post_EMBEDDED_BINDINGS_DF_KEY">EMBEDDED_BINDINGS_DF_KEY</a>)) {
        df::remove&lt;vector&lt;u8&gt;, vector&lt;<a href="../social_contracts/post.md#social_contracts_post_EmbeddedAssetBinding">EmbeddedAssetBinding</a>&gt;&gt;(&<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post">post</a>.id, <a href="../social_contracts/post.md#social_contracts_post_EMBEDDED_BINDINGS_DF_KEY">EMBEDDED_BINDINGS_DF_KEY</a>);
    };
    <b>if</b> (df::exists_with_type&lt;vector&lt;u8&gt;, vector&lt;<a href="../social_contracts/post.md#social_contracts_post_UsageDecisionSnapshot">UsageDecisionSnapshot</a>&gt;&gt;(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id, <a href="../social_contracts/post.md#social_contracts_post_USAGE_DECISIONS_DF_KEY">USAGE_DECISIONS_DF_KEY</a>)) {
        df::remove&lt;vector&lt;u8&gt;, vector&lt;<a href="../social_contracts/post.md#social_contracts_post_UsageDecisionSnapshot">UsageDecisionSnapshot</a>&gt;&gt;(&<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post">post</a>.id, <a href="../social_contracts/post.md#social_contracts_post_USAGE_DECISIONS_DF_KEY">USAGE_DECISIONS_DF_KEY</a>);
    };
    <b>if</b> (df::exists_with_type&lt;vector&lt;u8&gt;, vector&lt;<a href="../social_contracts/post.md#social_contracts_post_ContainerUsageDenial">ContainerUsageDenial</a>&gt;&gt;(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id, <a href="../social_contracts/post.md#social_contracts_post_USAGE_DENIALS_DF_KEY">USAGE_DENIALS_DF_KEY</a>)) {
        df::remove&lt;vector&lt;u8&gt;, vector&lt;<a href="../social_contracts/post.md#social_contracts_post_ContainerUsageDenial">ContainerUsageDenial</a>&gt;&gt;(&<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post">post</a>.id, <a href="../social_contracts/post.md#social_contracts_post_USAGE_DENIALS_DF_KEY">USAGE_DENIALS_DF_KEY</a>);
    };
    <b>if</b> (df::exists_with_type&lt;vector&lt;u8&gt;, RevenueManifest&gt;(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id, <a href="../social_contracts/post.md#social_contracts_post_CANDIDATE_MANIFEST_DF_KEY">CANDIDATE_MANIFEST_DF_KEY</a>)) {
        df::remove&lt;vector&lt;u8&gt;, RevenueManifest&gt;(&<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post">post</a>.id, <a href="../social_contracts/post.md#social_contracts_post_CANDIDATE_MANIFEST_DF_KEY">CANDIDATE_MANIFEST_DF_KEY</a>);
    };
}
</code></pre>



</details>

<a name="social_contracts_post_clear_poc_data"></a>

## Function `clear_poc_data`

Legacy alias for dispute paths during migration.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_clear_poc_data">clear_poc_data</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_clear_poc_data">clear_poc_data</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>) {
    <a href="../social_contracts/post.md#social_contracts_post_clear_composition_data">clear_composition_data</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>);
}
</code></pre>



</details>

<a name="social_contracts_post_increment_composition_disputes_submitted"></a>

## Function `increment_composition_disputes_submitted`

Increment after each successful composition dispute submission.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_increment_composition_disputes_submitted">increment_composition_disputes_submitted</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, max_disputes: u8)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_increment_composition_disputes_submitted">increment_composition_disputes_submitted</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>, max_disputes: u8) {
    <b>let</b> submitted = <a href="../social_contracts/post.md#social_contracts_post_composition_disputes_submitted">composition_disputes_submitted</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>);
    <b>assert</b>!(submitted &lt; max_disputes, <a href="../social_contracts/post.md#social_contracts_post_EDisputeCapReached">EDisputeCapReached</a>);
    <b>let</b> next = submitted + 1;
    <b>if</b> (df::exists_with_type&lt;vector&lt;u8&gt;, u8&gt;(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id, <a href="../social_contracts/post.md#social_contracts_post_COMPOSITION_DISPUTES_DF_KEY">COMPOSITION_DISPUTES_DF_KEY</a>)) {
        <b>let</b> count = df::borrow_mut(&<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post">post</a>.id, <a href="../social_contracts/post.md#social_contracts_post_COMPOSITION_DISPUTES_DF_KEY">COMPOSITION_DISPUTES_DF_KEY</a>);
        *count = next;
    } <b>else</b> {
        df::add(&<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post">post</a>.id, <a href="../social_contracts/post.md#social_contracts_post_COMPOSITION_DISPUTES_DF_KEY">COMPOSITION_DISPUTES_DF_KEY</a>, next);
    };
}
</code></pre>



</details>

<a name="social_contracts_post_increment_poc_disputes_submitted"></a>

## Function `increment_poc_disputes_submitted`

Legacy alias for dispute paths during migration.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_increment_poc_disputes_submitted">increment_poc_disputes_submitted</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, max_disputes: u8)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_increment_poc_disputes_submitted">increment_poc_disputes_submitted</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>, max_disputes: u8) {
    <a href="../social_contracts/post.md#social_contracts_post_increment_composition_disputes_submitted">increment_composition_disputes_submitted</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>, max_disputes);
}
</code></pre>



</details>

<a name="social_contracts_post_deposit_coin_to_beneficiary_vault"></a>

## Function `deposit_coin_to_beneficiary_vault`

Deposit manifest-routed fees into the beneficiary vault when the post uses escrow payout mode.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_deposit_coin_to_beneficiary_vault">deposit_coin_to_beneficiary_vault</a>&lt;T&gt;(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, beneficiary_vault: &<b>mut</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_PoCBeneficiaryVault">social_contracts::poc_vault::PoCBeneficiaryVault</a>, fee_coin: <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;T&gt;, min_vault_deposit_amount: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_deposit_coin_to_beneficiary_vault">deposit_coin_to_beneficiary_vault</a>&lt;T&gt;(
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>,
    beneficiary_vault: &<b>mut</b> PoCBeneficiaryVault,
    fee_coin: Coin&lt;T&gt;,
    min_vault_deposit_amount: u64,
    clock: &Clock,
    ctx: &TxContext
) {
    <b>assert</b>!(<a href="../social_contracts/post.md#social_contracts_post_monetization_enabled">monetization_enabled</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>), <a href="../social_contracts/post.md#social_contracts_post_EMonetizationNotEnabled">EMonetizationNotEnabled</a>);
    <b>let</b> manifest_opt = <a href="../social_contracts/post.md#social_contracts_post_revenue_manifest">revenue_manifest</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>);
    <b>assert</b>!(option::is_some(&manifest_opt), <a href="../social_contracts/post.md#social_contracts_post_EInvalidManifest">EInvalidManifest</a>);
    <b>let</b> manifest = option::borrow(&manifest_opt);
    <b>let</b> ben = <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_beneficiary_address">poc_vault::beneficiary_address</a>(beneficiary_vault);
    <b>let</b> entries = <a href="../social_contracts/media_asset.md#social_contracts_media_asset_manifest_entries">media_asset::manifest_entries</a>(manifest);
    <b>let</b> len = vector::length(entries);
    <b>let</b> <b>mut</b> i = 0;
    <b>let</b> <b>mut</b> found_escrow = <b>false</b>;
    <b>while</b> (i &lt; len) {
        <b>let</b> e = vector::borrow(entries, i);
        <b>if</b> (<a href="../social_contracts/media_asset.md#social_contracts_media_asset_manifest_entry_payout_mode">media_asset::manifest_entry_payout_mode</a>(e) == <a href="../social_contracts/media_asset.md#social_contracts_media_asset_payout_escrow">media_asset::payout_escrow</a>()
            && <a href="../social_contracts/media_asset.md#social_contracts_media_asset_manifest_entry_beneficiary">media_asset::manifest_entry_beneficiary</a>(e) == ben) {
            found_escrow = <b>true</b>;
            <b>break</b>
        };
        i = i + 1;
    };
    <b>assert</b>!(found_escrow, <a href="../social_contracts/post.md#social_contracts_post_EInvalidManifest">EInvalidManifest</a>);
    <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_deposit_coin">poc_vault::deposit_coin</a>&lt;T&gt;(
        beneficiary_vault,
        ben,
        fee_coin,
        option::some(<a href="../social_contracts/post.md#social_contracts_post_get_id_address">get_id_address</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>)),
        min_vault_deposit_amount,
        clock,
        ctx
    );
}
</code></pre>



</details>

<a name="social_contracts_post_tip_repost"></a>

## Function `tip_repost`

Tip a repost or quote repost; splits per <code><a href="../social_contracts/post.md#social_contracts_post_PostConfig">PostConfig</a></code> between repost owner and original author.
Pass the shared <code>PoCBeneficiaryVault</code> for each post when manifest entries use escrow payout mode.


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_tip_repost">tip_repost</a>&lt;T&gt;(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, original_post: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, config: &<a href="../social_contracts/post.md#social_contracts_post_PostConfig">social_contracts::post::PostConfig</a>, vault_for_post: &<b>mut</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_PoCBeneficiaryVault">social_contracts::poc_vault::PoCBeneficiaryVault</a>, vault_for_original: &<b>mut</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_PoCBeneficiaryVault">social_contracts::poc_vault::PoCBeneficiaryVault</a>, coin: &<b>mut</b> <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;T&gt;, amount: u64, min_vault_deposit_amount: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_tip_repost">tip_repost</a>&lt;T&gt;(
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>,
    original_post: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>,
    config: &<a href="../social_contracts/post.md#social_contracts_post_PostConfig">PostConfig</a>,
    vault_for_post: &<b>mut</b> PoCBeneficiaryVault,
    vault_for_original: &<b>mut</b> PoCBeneficiaryVault,
    coin: &<b>mut</b> Coin&lt;T&gt;,
    amount: u64,
    min_vault_deposit_amount: u64,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> tipper = tx_context::sender(ctx);
    <b>assert</b>!(amount &gt; 0 && coin::value(coin) &gt;= amount, <a href="../social_contracts/post.md#social_contracts_post_EInvalidTipAmount">EInvalidTipAmount</a>);
    <b>assert</b>!(tipper != <a href="../social_contracts/post.md#social_contracts_post">post</a>.owner, <a href="../social_contracts/post.md#social_contracts_post_ESelfTipping">ESelfTipping</a>);
    <b>assert</b>!(
        string::utf8(<a href="../social_contracts/post.md#social_contracts_post_POST_TYPE_REPOST">POST_TYPE_REPOST</a>) == <a href="../social_contracts/post.md#social_contracts_post">post</a>.post_type ||
        string::utf8(<a href="../social_contracts/post.md#social_contracts_post_POST_TYPE_QUOTE_REPOST">POST_TYPE_QUOTE_REPOST</a>) == <a href="../social_contracts/post.md#social_contracts_post">post</a>.post_type,
        <a href="../social_contracts/post.md#social_contracts_post_EInvalidPostType">EInvalidPostType</a>
    );
    <b>assert</b>!(option::is_some(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.parent_post_id), <a href="../social_contracts/post.md#social_contracts_post_EInvalidParentReference">EInvalidParentReference</a>);
    <b>let</b> parent_id = *option::borrow(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.parent_post_id);
    <b>assert</b>!(parent_id == object::uid_to_address(&original_post.id), <a href="../social_contracts/post.md#social_contracts_post_EInvalidParentReference">EInvalidParentReference</a>);
    <b>assert</b>!(<a href="../social_contracts/post.md#social_contracts_post_allow_tips">allow_tips</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>), <a href="../social_contracts/post.md#social_contracts_post_ETipsNotAllowed">ETipsNotAllowed</a>);
    <b>assert</b>!(<a href="../social_contracts/post.md#social_contracts_post_allow_tips">allow_tips</a>(original_post), <a href="../social_contracts/post.md#social_contracts_post_ETipsNotAllowed">ETipsNotAllowed</a>);
    <b>let</b> ct = type_name::with_defining_ids&lt;T&gt;();
    <b>if</b> (<a href="../social_contracts/post.md#social_contracts_post">post</a>.owner == original_post.owner) {
        <b>let</b> po = <a href="../social_contracts/post.md#social_contracts_post">post</a>.owner;
        <b>let</b> poid = object::uid_to_address(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id);
        <b>let</b> actual_received = <a href="../social_contracts/post.md#social_contracts_post_apply_revenue_manifest_coin">apply_revenue_manifest_coin</a>&lt;T&gt;(
            <a href="../social_contracts/post.md#social_contracts_post">post</a>,
            vault_for_post,
            po,
            amount,
            coin,
            tipper,
            poid,
            <b>true</b>,
            min_vault_deposit_amount,
            clock,
            ctx
        );
        <b>assert</b>!(<a href="../social_contracts/post.md#social_contracts_post">post</a>.tips_received &lt;= <a href="../social_contracts/post.md#social_contracts_post_MAX_U64">MAX_U64</a> - actual_received, <a href="../social_contracts/post.md#social_contracts_post_EOverflow">EOverflow</a>);
        <a href="../social_contracts/post.md#social_contracts_post">post</a>.tips_received = <a href="../social_contracts/post.md#social_contracts_post">post</a>.tips_received + actual_received;
        <b>if</b> (actual_received &gt; 0) {
            event::emit(<a href="../social_contracts/post.md#social_contracts_post_TipEvent">TipEvent</a> {
                object_id: poid,
                from: tipper,
                to: po,
                amount: actual_received,
                coin_type: ct,
                is_post: <b>true</b>,
            });
        };
    } <b>else</b> {
        <b>let</b> repost_owner_amount = (amount * config.repost_tip_percentage) / 100;
        <b>let</b> original_owner_amount = amount - repost_owner_amount;
        <b>let</b> po = <a href="../social_contracts/post.md#social_contracts_post">post</a>.owner;
        <b>let</b> poid = object::uid_to_address(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id);
        <b>let</b> opo = original_post.owner;
        <b>let</b> opoid = object::uid_to_address(&original_post.id);
        <b>let</b> repost_actual_received = <a href="../social_contracts/post.md#social_contracts_post_apply_revenue_manifest_coin">apply_revenue_manifest_coin</a>&lt;T&gt;(
            <a href="../social_contracts/post.md#social_contracts_post">post</a>,
            vault_for_post,
            po,
            repost_owner_amount,
            coin,
            tipper,
            poid,
            <b>true</b>,
            min_vault_deposit_amount,
            clock,
            ctx
        );
        <b>let</b> original_actual_received = <a href="../social_contracts/post.md#social_contracts_post_apply_revenue_manifest_coin">apply_revenue_manifest_coin</a>&lt;T&gt;(
            original_post,
            vault_for_original,
            opo,
            original_owner_amount,
            coin,
            tipper,
            opoid,
            <b>true</b>,
            min_vault_deposit_amount,
            clock,
            ctx
        );
        <b>assert</b>!(<a href="../social_contracts/post.md#social_contracts_post">post</a>.tips_received &lt;= <a href="../social_contracts/post.md#social_contracts_post_MAX_U64">MAX_U64</a> - repost_actual_received, <a href="../social_contracts/post.md#social_contracts_post_EOverflow">EOverflow</a>);
        <a href="../social_contracts/post.md#social_contracts_post">post</a>.tips_received = <a href="../social_contracts/post.md#social_contracts_post">post</a>.tips_received + repost_actual_received;
        <b>assert</b>!(original_post.tips_received &lt;= <a href="../social_contracts/post.md#social_contracts_post_MAX_U64">MAX_U64</a> - original_actual_received, <a href="../social_contracts/post.md#social_contracts_post_EOverflow">EOverflow</a>);
        original_post.tips_received = original_post.tips_received + original_actual_received;
        <b>if</b> (repost_actual_received &gt; 0) {
            event::emit(<a href="../social_contracts/post.md#social_contracts_post_TipEvent">TipEvent</a> {
                object_id: poid,
                from: tipper,
                to: po,
                amount: repost_actual_received,
                coin_type: ct,
                is_post: <b>true</b>,
            });
        };
        <b>if</b> (original_actual_received &gt; 0) {
            event::emit(<a href="../social_contracts/post.md#social_contracts_post_TipEvent">TipEvent</a> {
                object_id: opoid,
                from: tipper,
                to: opo,
                amount: original_actual_received,
                coin_type: ct,
                is_post: <b>true</b>,
            });
        };
    }
}
</code></pre>



</details>

<a name="social_contracts_post_tip_comment"></a>

## Function `tip_comment`

Tip a comment; split per <code><a href="../social_contracts/post.md#social_contracts_post_PostConfig">PostConfig</a></code> with manifest-based routing on the post owner's share.


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_tip_comment">tip_comment</a>&lt;T&gt;(comment: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Comment">social_contracts::post::Comment</a>, <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, config: &<a href="../social_contracts/post.md#social_contracts_post_PostConfig">social_contracts::post::PostConfig</a>, beneficiary_vault: &<b>mut</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_PoCBeneficiaryVault">social_contracts::poc_vault::PoCBeneficiaryVault</a>, coin: &<b>mut</b> <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;T&gt;, amount: u64, min_vault_deposit_amount: u64, memory_account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_tip_comment">tip_comment</a>&lt;T&gt;(
    comment: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Comment">Comment</a>,
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>,
    config: &<a href="../social_contracts/post.md#social_contracts_post_PostConfig">PostConfig</a>,
    beneficiary_vault: &<b>mut</b> PoCBeneficiaryVault,
    coin: &<b>mut</b> Coin&lt;T&gt;,
    amount: u64,
    min_vault_deposit_amount: u64,
    memory_account: &MemoryAccount,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> tipper = tx_context::sender(ctx);
    <b>assert</b>!(amount &gt; 0 && coin::value(coin) &gt;= amount, <a href="../social_contracts/post.md#social_contracts_post_EInvalidTipAmount">EInvalidTipAmount</a>);
    <a href="../social_contracts/post.md#social_contracts_post_assert_tip_spend_limit">assert_tip_spend_limit</a>(memory_account, amount, ctx);
    <b>assert</b>!(tipper != comment.owner, <a href="../social_contracts/post.md#social_contracts_post_ESelfTipping">ESelfTipping</a>);
    <b>assert</b>!(<a href="../social_contracts/post.md#social_contracts_post_allow_tips">allow_tips</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>), <a href="../social_contracts/post.md#social_contracts_post_ETipsNotAllowed">ETipsNotAllowed</a>);
    <b>let</b> commenter_amount = (amount * config.commenter_tip_percentage) / 100;
    <b>let</b> post_owner_amount = amount - commenter_amount;
    <b>let</b> commenter_tip = coin::split(coin, commenter_amount, ctx);
    transfer::public_transfer(commenter_tip, comment.owner);
    <b>let</b> po = <a href="../social_contracts/post.md#social_contracts_post">post</a>.owner;
    <b>let</b> poid = object::uid_to_address(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id);
    <b>let</b> ct = type_name::with_defining_ids&lt;T&gt;();
    <b>let</b> post_owner_actual_received = <a href="../social_contracts/post.md#social_contracts_post_apply_revenue_manifest_coin">apply_revenue_manifest_coin</a>&lt;T&gt;(
        <a href="../social_contracts/post.md#social_contracts_post">post</a>,
        beneficiary_vault,
        po,
        post_owner_amount,
        coin,
        tipper,
        poid,
        <b>true</b>,
        min_vault_deposit_amount,
        clock,
        ctx
    );
    <b>assert</b>!(comment.tips_received &lt;= <a href="../social_contracts/post.md#social_contracts_post_MAX_U64">MAX_U64</a> - commenter_amount, <a href="../social_contracts/post.md#social_contracts_post_EOverflow">EOverflow</a>);
    comment.tips_received = comment.tips_received + commenter_amount;
    <b>assert</b>!(<a href="../social_contracts/post.md#social_contracts_post">post</a>.tips_received &lt;= <a href="../social_contracts/post.md#social_contracts_post_MAX_U64">MAX_U64</a> - post_owner_actual_received, <a href="../social_contracts/post.md#social_contracts_post_EOverflow">EOverflow</a>);
    <a href="../social_contracts/post.md#social_contracts_post">post</a>.tips_received = <a href="../social_contracts/post.md#social_contracts_post">post</a>.tips_received + post_owner_actual_received;
    event::emit(<a href="../social_contracts/post.md#social_contracts_post_TipEvent">TipEvent</a> {
        object_id: object::uid_to_address(&comment.id),
        from: tipper,
        to: comment.owner,
        amount: commenter_amount,
        coin_type: ct,
        is_post: <b>false</b>,
    });
    <b>if</b> (post_owner_actual_received &gt; 0) {
        event::emit(<a href="../social_contracts/post.md#social_contracts_post_TipEvent">TipEvent</a> {
            object_id: poid,
            from: tipper,
            to: po,
            amount: post_owner_actual_received,
            coin_type: ct,
            is_post: <b>true</b>,
        });
    };
}
</code></pre>



</details>

<a name="social_contracts_post_tip_comment_simple"></a>

## Function `tip_comment_simple`

Like [<code><a href="../social_contracts/post.md#social_contracts_post_tip_comment">tip_comment</a></code>] but without a <code>PoCBeneficiaryVault</code> / Clock / min vault deposit.
Only for posts where [<code><a href="../social_contracts/post.md#social_contracts_post_tip_post_requires_beneficiary_vault_for_amount">tip_post_requires_beneficiary_vault_for_amount</a></code>] is false for the
**post-owner slice** of this tip. If an escrow deposit is required, aborts with
[<code><a href="../social_contracts/post.md#social_contracts_post_ETipPostRequiresBeneficiaryVault">ETipPostRequiresBeneficiaryVault</a></code>].


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_tip_comment_simple">tip_comment_simple</a>&lt;T&gt;(comment: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Comment">social_contracts::post::Comment</a>, <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, config: &<a href="../social_contracts/post.md#social_contracts_post_PostConfig">social_contracts::post::PostConfig</a>, coin: &<b>mut</b> <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;T&gt;, amount: u64, memory_account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_tip_comment_simple">tip_comment_simple</a>&lt;T&gt;(
    comment: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Comment">Comment</a>,
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>,
    config: &<a href="../social_contracts/post.md#social_contracts_post_PostConfig">PostConfig</a>,
    coin: &<b>mut</b> Coin&lt;T&gt;,
    amount: u64,
    memory_account: &MemoryAccount,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> tipper = tx_context::sender(ctx);
    <b>assert</b>!(amount &gt; 0 && coin::value(coin) &gt;= amount, <a href="../social_contracts/post.md#social_contracts_post_EInvalidTipAmount">EInvalidTipAmount</a>);
    <a href="../social_contracts/post.md#social_contracts_post_assert_tip_spend_limit">assert_tip_spend_limit</a>(memory_account, amount, ctx);
    <b>assert</b>!(tipper != comment.owner, <a href="../social_contracts/post.md#social_contracts_post_ESelfTipping">ESelfTipping</a>);
    <b>assert</b>!(<a href="../social_contracts/post.md#social_contracts_post_allow_tips">allow_tips</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>), <a href="../social_contracts/post.md#social_contracts_post_ETipsNotAllowed">ETipsNotAllowed</a>);
    <b>let</b> commenter_amount = (amount * config.commenter_tip_percentage) / 100;
    <b>let</b> post_owner_amount = amount - commenter_amount;
    <b>assert</b>!(
        !<a href="../social_contracts/post.md#social_contracts_post_tip_post_requires_beneficiary_vault_for_amount">tip_post_requires_beneficiary_vault_for_amount</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>, post_owner_amount),
        <a href="../social_contracts/post.md#social_contracts_post_ETipPostRequiresBeneficiaryVault">ETipPostRequiresBeneficiaryVault</a>
    );
    <b>let</b> commenter_tip = coin::split(coin, commenter_amount, ctx);
    transfer::public_transfer(commenter_tip, comment.owner);
    <b>let</b> po = <a href="../social_contracts/post.md#social_contracts_post">post</a>.owner;
    <b>let</b> poid = object::uid_to_address(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id);
    <b>let</b> ct = type_name::with_defining_ids&lt;T&gt;();
    <b>let</b> post_owner_actual_received = <a href="../social_contracts/post.md#social_contracts_post_apply_revenue_manifest_coin_without_beneficiary_vault">apply_revenue_manifest_coin_without_beneficiary_vault</a>&lt;T&gt;(
        <a href="../social_contracts/post.md#social_contracts_post">post</a>,
        po,
        post_owner_amount,
        coin,
        tipper,
        poid,
        <b>true</b>,
        ctx
    );
    <b>assert</b>!(comment.tips_received &lt;= <a href="../social_contracts/post.md#social_contracts_post_MAX_U64">MAX_U64</a> - commenter_amount, <a href="../social_contracts/post.md#social_contracts_post_EOverflow">EOverflow</a>);
    comment.tips_received = comment.tips_received + commenter_amount;
    <b>assert</b>!(<a href="../social_contracts/post.md#social_contracts_post">post</a>.tips_received &lt;= <a href="../social_contracts/post.md#social_contracts_post_MAX_U64">MAX_U64</a> - post_owner_actual_received, <a href="../social_contracts/post.md#social_contracts_post_EOverflow">EOverflow</a>);
    <a href="../social_contracts/post.md#social_contracts_post">post</a>.tips_received = <a href="../social_contracts/post.md#social_contracts_post">post</a>.tips_received + post_owner_actual_received;
    event::emit(<a href="../social_contracts/post.md#social_contracts_post_TipEvent">TipEvent</a> {
        object_id: object::uid_to_address(&comment.id),
        from: tipper,
        to: comment.owner,
        amount: commenter_amount,
        coin_type: ct,
        is_post: <b>false</b>,
    });
    <b>if</b> (post_owner_actual_received &gt; 0) {
        event::emit(<a href="../social_contracts/post.md#social_contracts_post_TipEvent">TipEvent</a> {
            object_id: poid,
            from: tipper,
            to: po,
            amount: post_owner_actual_received,
            coin_type: ct,
            is_post: <b>true</b>,
        });
    };
}
</code></pre>



</details>

<a name="social_contracts_post_transfer_post_ownership"></a>

## Function `transfer_post_ownership`

Transfer post ownership to another user (by post owner)


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_transfer_post_ownership">transfer_post_ownership</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, new_owner: <b>address</b>, registry: &<a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">social_contracts::profile::UsernameRegistry</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_transfer_post_ownership">transfer_post_ownership</a>(
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>,
    new_owner: <b>address</b>,
    registry: &UsernameRegistry,
    ctx: &<b>mut</b> TxContext
) {
    // Check <a href="../social_contracts/post.md#social_contracts_post_version">version</a> compatibility
    <b>assert</b>!(<a href="../social_contracts/post.md#social_contracts_post">post</a>.<a href="../social_contracts/post.md#social_contracts_post_version">version</a> == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/post.md#social_contracts_post_EWrongVersion">EWrongVersion</a>);
    <b>let</b> current_owner = tx_context::sender(ctx);
    // Verify current owner is authorized
    <b>assert</b>!(current_owner == <a href="../social_contracts/post.md#social_contracts_post">post</a>.owner, <a href="../social_contracts/post.md#social_contracts_post_EUnauthorizedTransfer">EUnauthorizedTransfer</a>);
    // Look up the <a href="../social_contracts/profile.md#social_contracts_profile">profile</a> ID <b>for</b> the new owner (<b>for</b> reference, not ownership)
    <b>let</b> <b>mut</b> profile_id_option = <a href="../social_contracts/profile.md#social_contracts_profile_lookup_profile_by_owner">social_contracts::profile::lookup_profile_by_owner</a>(registry, new_owner);
    <b>assert</b>!(option::is_some(&profile_id_option), <a href="../social_contracts/post.md#social_contracts_post_EUnauthorized">EUnauthorized</a>);
    <b>let</b> new_profile_id = option::extract(&<b>mut</b> profile_id_option);
    // Update <a href="../social_contracts/post.md#social_contracts_post">post</a> ownership
    <b>let</b> previous_owner = <a href="../social_contracts/post.md#social_contracts_post">post</a>.owner;
    <a href="../social_contracts/post.md#social_contracts_post">post</a>.owner = new_owner;
    <a href="../social_contracts/post.md#social_contracts_post">post</a>.profile_id = new_profile_id;
    // Emit ownership transfer event
    event::emit(<a href="../social_contracts/post.md#social_contracts_post_OwnershipTransferEvent">OwnershipTransferEvent</a> {
        object_id: object::uid_to_address(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id),
        previous_owner,
        new_owner,
        is_post: <b>true</b>,
    });
}
</code></pre>



</details>

<a name="social_contracts_post_admin_transfer_post_ownership"></a>

## Function `admin_transfer_post_ownership`

Admin function to transfer post ownership (requires PostAdminCap)


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_admin_transfer_post_ownership">admin_transfer_post_ownership</a>(_: &<a href="../social_contracts/post.md#social_contracts_post_PostAdminCap">social_contracts::post::PostAdminCap</a>, <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, new_owner: <b>address</b>, registry: &<a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">social_contracts::profile::UsernameRegistry</a>, _ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_admin_transfer_post_ownership">admin_transfer_post_ownership</a>(
    _: &<a href="../social_contracts/post.md#social_contracts_post_PostAdminCap">PostAdminCap</a>,
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>,
    new_owner: <b>address</b>,
    registry: &UsernameRegistry,
    _ctx: &<b>mut</b> TxContext
) {
    // Admin capability verification is handled by type system
    // Check <a href="../social_contracts/post.md#social_contracts_post_version">version</a> compatibility
    <b>assert</b>!(<a href="../social_contracts/post.md#social_contracts_post">post</a>.<a href="../social_contracts/post.md#social_contracts_post_version">version</a> == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/post.md#social_contracts_post_EWrongVersion">EWrongVersion</a>);
    // Look up the <a href="../social_contracts/profile.md#social_contracts_profile">profile</a> ID <b>for</b> the new owner (<b>for</b> reference, not ownership)
    <b>let</b> <b>mut</b> profile_id_option = <a href="../social_contracts/profile.md#social_contracts_profile_lookup_profile_by_owner">social_contracts::profile::lookup_profile_by_owner</a>(registry, new_owner);
    <b>assert</b>!(option::is_some(&profile_id_option), <a href="../social_contracts/post.md#social_contracts_post_EUnauthorized">EUnauthorized</a>);
    <b>let</b> new_profile_id = option::extract(&<b>mut</b> profile_id_option);
    // Update <a href="../social_contracts/post.md#social_contracts_post">post</a> ownership
    <b>let</b> previous_owner = <a href="../social_contracts/post.md#social_contracts_post">post</a>.owner;
    <a href="../social_contracts/post.md#social_contracts_post">post</a>.owner = new_owner;
    <a href="../social_contracts/post.md#social_contracts_post">post</a>.profile_id = new_profile_id;
    // Emit ownership transfer event
    event::emit(<a href="../social_contracts/post.md#social_contracts_post_OwnershipTransferEvent">OwnershipTransferEvent</a> {
        object_id: object::uid_to_address(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id),
        previous_owner,
        new_owner,
        is_post: <b>true</b>,
    });
}
</code></pre>



</details>

<a name="social_contracts_post_moderate_post"></a>

## Function `moderate_post`

Moderate a post (remove/restore from platform)


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_moderate_post">moderate_post</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, group: &<a href="../myso/permissioned_group.md#myso_permissioned_group_PermissionedGroup">myso::permissioned_group::PermissionedGroup</a>&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformPackage">social_contracts::platform::PlatformPackage</a>&gt;, platform_registry: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">social_contracts::platform::PlatformRegistry</a>, remove: bool, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_moderate_post">moderate_post</a>(
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">platform::Platform</a>,
    group: &PermissionedGroup&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformPackage">platform::PlatformPackage</a>&gt;,
    platform_registry: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">platform::PlatformRegistry</a>,
    remove: bool,
    ctx: &<b>mut</b> TxContext
) {
    // Check <a href="../social_contracts/post.md#social_contracts_post_version">version</a> compatibility
    <b>assert</b>!(<a href="../social_contracts/post.md#social_contracts_post">post</a>.<a href="../social_contracts/post.md#social_contracts_post_version">version</a> == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/post.md#social_contracts_post_EWrongVersion">EWrongVersion</a>);
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform_platform_version">platform::platform_version</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>) == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/post.md#social_contracts_post_EWrongVersion">EWrongVersion</a>);
    <b>let</b> caller = tx_context::sender(ctx);
    <b>assert</b>!(
        <a href="../social_contracts/platform.md#social_contracts_platform_has_moderator_permission">platform::has_moderator_permission</a>&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformContentModerator">platform::PlatformContentModerator</a>&gt;(group, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, caller),
        <a href="../social_contracts/post.md#social_contracts_post_EUnauthorized">EUnauthorized</a>,
    );
    // Verify <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> is approved
    <b>let</b> platform_id = object::uid_to_address(<a href="../social_contracts/platform.md#social_contracts_platform_id">platform::id</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>));
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform_is_approved">platform::is_approved</a>(platform_registry, platform_id), <a href="../social_contracts/post.md#social_contracts_post_EUnauthorized">EUnauthorized</a>);
    // Update <a href="../social_contracts/post.md#social_contracts_post">post</a> status
    <a href="../social_contracts/post.md#social_contracts_post">post</a>.removed_from_platform = remove;
    // Emit moderation event
    event::emit(<a href="../social_contracts/post.md#social_contracts_post_PostModerationEvent">PostModerationEvent</a> {
        post_id: object::uid_to_address(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id),
        platform_id: object::uid_to_address(<a href="../social_contracts/platform.md#social_contracts_platform_id">platform::id</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>)),
        removed: remove,
        moderated_by: caller,
    });
}
</code></pre>



</details>

<a name="social_contracts_post_moderate_comment"></a>

## Function `moderate_comment`

Moderate a comment (remove/restore from platform)


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_moderate_comment">moderate_comment</a>(comment: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Comment">social_contracts::post::Comment</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, group: &<a href="../myso/permissioned_group.md#myso_permissioned_group_PermissionedGroup">myso::permissioned_group::PermissionedGroup</a>&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformPackage">social_contracts::platform::PlatformPackage</a>&gt;, platform_registry: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">social_contracts::platform::PlatformRegistry</a>, remove: bool, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_moderate_comment">moderate_comment</a>(
    comment: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Comment">Comment</a>,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">platform::Platform</a>,
    group: &PermissionedGroup&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformPackage">platform::PlatformPackage</a>&gt;,
    platform_registry: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">platform::PlatformRegistry</a>,
    remove: bool,
    ctx: &<b>mut</b> TxContext
) {
    // Check <a href="../social_contracts/post.md#social_contracts_post_version">version</a> compatibility
    <b>assert</b>!(comment.<a href="../social_contracts/post.md#social_contracts_post_version">version</a> == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/post.md#social_contracts_post_EWrongVersion">EWrongVersion</a>);
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform_platform_version">platform::platform_version</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>) == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/post.md#social_contracts_post_EWrongVersion">EWrongVersion</a>);
    <b>let</b> caller = tx_context::sender(ctx);
    <b>assert</b>!(
        <a href="../social_contracts/platform.md#social_contracts_platform_has_moderator_permission">platform::has_moderator_permission</a>&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformContentModerator">platform::PlatformContentModerator</a>&gt;(group, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, caller),
        <a href="../social_contracts/post.md#social_contracts_post_EUnauthorized">EUnauthorized</a>,
    );
    // Verify <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> is approved
    <b>let</b> platform_id = object::uid_to_address(<a href="../social_contracts/platform.md#social_contracts_platform_id">platform::id</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>));
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform_is_approved">platform::is_approved</a>(platform_registry, platform_id), <a href="../social_contracts/post.md#social_contracts_post_EUnauthorized">EUnauthorized</a>);
    // Update comment status
    comment.removed_from_platform = remove;
    // Emit moderation event
    event::emit(<a href="../social_contracts/post.md#social_contracts_post_PostModerationEvent">PostModerationEvent</a> {
        post_id: object::uid_to_address(&comment.id),
        platform_id: object::uid_to_address(<a href="../social_contracts/platform.md#social_contracts_platform_id">platform::id</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>)),
        removed: remove,
        moderated_by: caller,
    });
}
</code></pre>



</details>

<a name="social_contracts_post_update_post"></a>

## Function `update_post`

Update an existing post


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_update_post">update_post</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, config: &<a href="../social_contracts/post.md#social_contracts_post_PostConfig">social_contracts::post::PostConfig</a>, content: <a href="../std/string.md#std_string_String">std::string::String</a>, media_urls: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;&gt;, mentions: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;<b>address</b>&gt;&gt;, metadata_json: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_update_post">update_post</a>(
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>,
    config: &<a href="../social_contracts/post.md#social_contracts_post_PostConfig">PostConfig</a>,
    content: String,
    <b>mut</b> media_urls: Option&lt;vector&lt;String&gt;&gt;,
    mentions: Option&lt;vector&lt;<b>address</b>&gt;&gt;,
    metadata_json: Option&lt;String&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    // Verify caller is the owner
    <b>let</b> owner = tx_context::sender(ctx);
    <b>assert</b>!(owner == <a href="../social_contracts/post.md#social_contracts_post">post</a>.owner, <a href="../social_contracts/post.md#social_contracts_post_EUnauthorized">EUnauthorized</a>);
    // Validate content length using config
    <b>assert</b>!(string::length(&content) &lt;= config.max_content_length, <a href="../social_contracts/post.md#social_contracts_post_EContentTooLarge">EContentTooLarge</a>);
    // Validate and update metadata <b>if</b> provided
    <b>if</b> (option::is_some(&metadata_json)) {
        <b>let</b> metadata_string = option::borrow(& metadata_json);
        <b>assert</b>!(string::length(metadata_string) &lt;= config.max_metadata_size, <a href="../social_contracts/post.md#social_contracts_post_EContentTooLarge">EContentTooLarge</a>);
        // Clear the current value and set the new one
        <a href="../social_contracts/post.md#social_contracts_post">post</a>.metadata_json = option::some(*metadata_string);
    };
    // Convert and validate media URLs <b>if</b> provided
    <b>if</b> (option::is_some(&media_urls)) {
        <b>let</b> url_strings = option::extract(&<b>mut</b> media_urls);
        // Validate media URLs count
        <b>assert</b>!(vector::length(&url_strings) &lt;= config.max_media_urls, <a href="../social_contracts/post.md#social_contracts_post_ETooManyMediaUrls">ETooManyMediaUrls</a>);
        // Convert string URLs to Url objects
        <b>let</b> <b>mut</b> urls = vector::empty&lt;Url&gt;();
        <b>let</b> <b>mut</b> i = 0;
        <b>let</b> len = vector::length(&url_strings);
        <b>while</b> (i &lt; len) {
            <b>let</b> url_string = vector::borrow(&url_strings, i);
            <b>let</b> url_bytes = string::as_bytes(url_string);
            vector::push_back(&<b>mut</b> urls, url::new_unsafe_from_bytes(*url_bytes));
            i = i + 1;
        };
        <a href="../social_contracts/post.md#social_contracts_post">post</a>.media = option::some(urls);
    };
    // Validate mentions <b>if</b> provided using config
    <b>if</b> (option::is_some(&mentions)) {
        <b>let</b> mentions_ref = option::borrow(&mentions);
        <b>assert</b>!(vector::length(mentions_ref) &lt;= config.max_mentions, <a href="../social_contracts/post.md#social_contracts_post_EContentTooLarge">EContentTooLarge</a>);
        <a href="../social_contracts/post.md#social_contracts_post">post</a>.mentions = mentions;
    };
    // Update <a href="../social_contracts/post.md#social_contracts_post">post</a> content
    <a href="../social_contracts/post.md#social_contracts_post">post</a>.content = content;
    // Emit <a href="../social_contracts/post.md#social_contracts_post">post</a> updated event
    event::emit(<a href="../social_contracts/post.md#social_contracts_post_PostUpdatedEvent">PostUpdatedEvent</a> {
        post_id: object::uid_to_address(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id),
        owner: <a href="../social_contracts/post.md#social_contracts_post">post</a>.owner,
        profile_id: <a href="../social_contracts/post.md#social_contracts_post">post</a>.profile_id,
        content: <a href="../social_contracts/post.md#social_contracts_post">post</a>.content,
        metadata_json: <a href="../social_contracts/post.md#social_contracts_post">post</a>.metadata_json,
        updated_at: clock::timestamp_ms(clock),
    });
}
</code></pre>



</details>

<a name="social_contracts_post_update_comment"></a>

## Function `update_comment`

Update an existing comment


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_update_comment">update_comment</a>(comment: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Comment">social_contracts::post::Comment</a>, config: &<a href="../social_contracts/post.md#social_contracts_post_PostConfig">social_contracts::post::PostConfig</a>, content: <a href="../std/string.md#std_string_String">std::string::String</a>, mentions: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;<b>address</b>&gt;&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_update_comment">update_comment</a>(
    comment: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Comment">Comment</a>,
    config: &<a href="../social_contracts/post.md#social_contracts_post_PostConfig">PostConfig</a>,
    content: String,
    mentions: Option&lt;vector&lt;<b>address</b>&gt;&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    // Verify caller is the owner
    <b>let</b> owner = tx_context::sender(ctx);
    <b>assert</b>!(owner == comment.owner, <a href="../social_contracts/post.md#social_contracts_post_EUnauthorized">EUnauthorized</a>);
    // Validate content length using config
    <b>assert</b>!(string::length(&content) &lt;= config.max_content_length, <a href="../social_contracts/post.md#social_contracts_post_EContentTooLarge">EContentTooLarge</a>);
    // Validate mentions <b>if</b> provided using config
    <b>if</b> (option::is_some(&mentions)) {
        <b>let</b> mentions_ref = option::borrow(&mentions);
        <b>assert</b>!(vector::length(mentions_ref) &lt;= config.max_mentions, <a href="../social_contracts/post.md#social_contracts_post_EContentTooLarge">EContentTooLarge</a>);
        comment.mentions = mentions;
    };
    // Update comment content
    comment.content = content;
    // Emit comment updated event
    event::emit(<a href="../social_contracts/post.md#social_contracts_post_CommentUpdatedEvent">CommentUpdatedEvent</a> {
        comment_id: object::uid_to_address(&comment.id),
        post_id: comment.post_id,
        owner: comment.owner,
        profile_id: comment.profile_id,
        content: comment.content,
        updated_at: clock::timestamp_ms(clock),
    });
}
</code></pre>



</details>

<a name="social_contracts_post_report_post"></a>

## Function `report_post`

Report a post


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_report_post">report_post</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, config: &<a href="../social_contracts/post.md#social_contracts_post_PostConfig">social_contracts::post::PostConfig</a>, reason_code: u8, description: <a href="../std/string.md#std_string_String">std::string::String</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_report_post">report_post</a>(
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>,
    config: &<a href="../social_contracts/post.md#social_contracts_post_PostConfig">PostConfig</a>,
    reason_code: u8,
    description: String,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    // Validate reason code
    <b>assert</b>!(
        reason_code == <a href="../social_contracts/post.md#social_contracts_post_REPORT_REASON_SPAM">REPORT_REASON_SPAM</a> ||
        reason_code == <a href="../social_contracts/post.md#social_contracts_post_REPORT_REASON_OFFENSIVE">REPORT_REASON_OFFENSIVE</a> ||
        reason_code == <a href="../social_contracts/post.md#social_contracts_post_REPORT_REASON_MISINFORMATION">REPORT_REASON_MISINFORMATION</a> ||
        reason_code == <a href="../social_contracts/post.md#social_contracts_post_REPORT_REASON_ILLEGAL">REPORT_REASON_ILLEGAL</a> ||
        reason_code == <a href="../social_contracts/post.md#social_contracts_post_REPORT_REASON_IMPERSONATION">REPORT_REASON_IMPERSONATION</a> ||
        reason_code == <a href="../social_contracts/post.md#social_contracts_post_REPORT_REASON_HARASSMENT">REPORT_REASON_HARASSMENT</a> ||
        reason_code == <a href="../social_contracts/post.md#social_contracts_post_REPORT_REASON_OTHER">REPORT_REASON_OTHER</a>,
        <a href="../social_contracts/post.md#social_contracts_post_EReportReasonInvalid">EReportReasonInvalid</a>
    );
    // Validate description length using config
    <b>assert</b>!(string::length(&description) &lt;= config.max_description_length, <a href="../social_contracts/post.md#social_contracts_post_EReportDescriptionTooLong">EReportDescriptionTooLong</a>);
    // Get reporter's <b>address</b>
    <b>let</b> reporter = tx_context::sender(ctx);
    // Emit <a href="../social_contracts/post.md#social_contracts_post">post</a> reported event
    event::emit(<a href="../social_contracts/post.md#social_contracts_post_PostReportedEvent">PostReportedEvent</a> {
        post_id: object::uid_to_address(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id),
        reporter,
        reason_code,
        description,
        reported_at: clock::timestamp_ms(clock),
    });
}
</code></pre>



</details>

<a name="social_contracts_post_report_comment"></a>

## Function `report_comment`

Report a comment


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_report_comment">report_comment</a>(comment: &<a href="../social_contracts/post.md#social_contracts_post_Comment">social_contracts::post::Comment</a>, config: &<a href="../social_contracts/post.md#social_contracts_post_PostConfig">social_contracts::post::PostConfig</a>, reason_code: u8, description: <a href="../std/string.md#std_string_String">std::string::String</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_report_comment">report_comment</a>(
    comment: &<a href="../social_contracts/post.md#social_contracts_post_Comment">Comment</a>,
    config: &<a href="../social_contracts/post.md#social_contracts_post_PostConfig">PostConfig</a>,
    reason_code: u8,
    description: String,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    // Validate reason code
    <b>assert</b>!(
        reason_code == <a href="../social_contracts/post.md#social_contracts_post_REPORT_REASON_SPAM">REPORT_REASON_SPAM</a> ||
        reason_code == <a href="../social_contracts/post.md#social_contracts_post_REPORT_REASON_OFFENSIVE">REPORT_REASON_OFFENSIVE</a> ||
        reason_code == <a href="../social_contracts/post.md#social_contracts_post_REPORT_REASON_MISINFORMATION">REPORT_REASON_MISINFORMATION</a> ||
        reason_code == <a href="../social_contracts/post.md#social_contracts_post_REPORT_REASON_ILLEGAL">REPORT_REASON_ILLEGAL</a> ||
        reason_code == <a href="../social_contracts/post.md#social_contracts_post_REPORT_REASON_IMPERSONATION">REPORT_REASON_IMPERSONATION</a> ||
        reason_code == <a href="../social_contracts/post.md#social_contracts_post_REPORT_REASON_HARASSMENT">REPORT_REASON_HARASSMENT</a> ||
        reason_code == <a href="../social_contracts/post.md#social_contracts_post_REPORT_REASON_OTHER">REPORT_REASON_OTHER</a>,
        <a href="../social_contracts/post.md#social_contracts_post_EReportReasonInvalid">EReportReasonInvalid</a>
    );
    // Validate description length using config
    <b>assert</b>!(string::length(&description) &lt;= config.max_description_length, <a href="../social_contracts/post.md#social_contracts_post_EReportDescriptionTooLong">EReportDescriptionTooLong</a>);
    // Get reporter's <b>address</b>
    <b>let</b> reporter = tx_context::sender(ctx);
    // Emit comment reported event
    event::emit(<a href="../social_contracts/post.md#social_contracts_post_CommentReportedEvent">CommentReportedEvent</a> {
        comment_id: object::uid_to_address(&comment.id),
        reporter,
        reason_code,
        description,
        reported_at: clock::timestamp_ms(clock),
    });
}
</code></pre>



</details>

<a name="social_contracts_post_react_to_comment"></a>

## Function `react_to_comment`

React to a comment with a specific reaction (emoji or text)
If the user already has the exact same reaction, it will be removed (toggle behavior)


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_react_to_comment">react_to_comment</a>(registry: &<a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">social_contracts::profile::UsernameRegistry</a>, comment: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Comment">social_contracts::post::Comment</a>, platform_registry: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">social_contracts::platform::PlatformRegistry</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, config: &<a href="../social_contracts/post.md#social_contracts_post_PostConfig">social_contracts::post::PostConfig</a>, memory_config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, memory_account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, reaction: <a href="../std/string.md#std_string_String">std::string::String</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_react_to_comment">react_to_comment</a>(
    registry: &UsernameRegistry,
    comment: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Comment">Comment</a>,
    platform_registry: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">platform::PlatformRegistry</a>,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">platform::Platform</a>,
    block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">block_list::BlockListRegistry</a>,
    config: &<a href="../social_contracts/post.md#social_contracts_post_PostConfig">PostConfig</a>,
    memory_config: &MemoryConfig,
    memory_account: &MemoryAccount,
    reaction: String,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> acting = <a href="../social_contracts/post.md#social_contracts_post_resolve_social_actor">resolve_social_actor</a>(
        memory_config,
        registry,
        <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>,
        block_list_registry,
        memory_account,
        <a href="../social_contracts/memory.md#social_contracts_memory_cap_react">memory::cap_react</a>(),
        0,
        clock,
        ctx,
    );
    <b>let</b> <a href="../social_contracts/post.md#social_contracts_post_actor_address">actor_address</a> = <a href="../social_contracts/memory.md#social_contracts_memory_acting_actor_address">memory::acting_actor_address</a>(&acting);
    <b>let</b> principal_owner = <a href="../social_contracts/memory.md#social_contracts_memory_acting_principal_owner">memory::acting_principal_owner</a>(&acting);
    <b>let</b> <a href="../social_contracts/post.md#social_contracts_post_sub_agent_id">sub_agent_id</a> = <a href="../social_contracts/memory.md#social_contracts_memory_acting_sub_agent_id">memory::acting_sub_agent_id</a>(&acting);
    <b>let</b> organization_id = <a href="../social_contracts/memory.md#social_contracts_memory_acting_organization_id">memory::acting_organization_id</a>(&acting);
    <b>let</b> <a href="../social_contracts/post.md#social_contracts_post_action_identity_class">action_identity_class</a> = <a href="../social_contracts/memory.md#social_contracts_memory_acting_identity_class">memory::acting_identity_class</a>(&acting);
    // Check <b>if</b> <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> is approved
    <b>let</b> platform_id = object::uid_to_address(<a href="../social_contracts/platform.md#social_contracts_platform_id">platform::id</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>));
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform_is_approved">platform::is_approved</a>(platform_registry, platform_id), <a href="../social_contracts/post.md#social_contracts_post_EUnauthorized">EUnauthorized</a>);
    // Validate reaction length using config
    <b>assert</b>!(string::length(&reaction) &lt;= config.max_reaction_length, <a href="../social_contracts/post.md#social_contracts_post_EReactionContentTooLong">EReactionContentTooLong</a>);
    // Check <b>if</b> user already reacted to the comment
    <b>if</b> (table::contains(&comment.user_reactions, <a href="../social_contracts/post.md#social_contracts_post_actor_address">actor_address</a>)) {
        // Get the previous reaction
        <b>let</b> previous_reaction = *table::borrow(&comment.user_reactions, <a href="../social_contracts/post.md#social_contracts_post_actor_address">actor_address</a>);
        // If the reaction is the same, remove it (toggle behavior)
        <b>if</b> (reaction == previous_reaction) {
            // Remove user's reaction
            table::remove(&<b>mut</b> comment.user_reactions, <a href="../social_contracts/post.md#social_contracts_post_actor_address">actor_address</a>);
            // Decrease count <b>for</b> this reaction type
            <b>let</b> count = *table::borrow(&comment.reaction_counts, reaction);
            <b>if</b> (count &lt;= 1) {
                table::remove(&<b>mut</b> comment.reaction_counts, reaction);
            } <b>else</b> {
                *table::borrow_mut(&<b>mut</b> comment.reaction_counts, reaction) = count - 1;
            };
            // Decrement comment reaction count with underflow protection
            <b>assert</b>!(comment.reaction_count &gt; 0, <a href="../social_contracts/post.md#social_contracts_post_EOverflow">EOverflow</a>);
            comment.reaction_count = comment.reaction_count - 1;
            // Emit remove reaction event
            event::emit(<a href="../social_contracts/post.md#social_contracts_post_RemoveReactionEvent">RemoveReactionEvent</a> {
                object_id: object::uid_to_address(&comment.id),
                user: <a href="../social_contracts/post.md#social_contracts_post_actor_address">actor_address</a>,
                reaction,
                is_post: <b>false</b>,
                principal_owner,
                <a href="../social_contracts/post.md#social_contracts_post_actor_address">actor_address</a>,
                <a href="../social_contracts/post.md#social_contracts_post_sub_agent_id">sub_agent_id</a>,
                organization_id,
                <a href="../social_contracts/post.md#social_contracts_post_action_identity_class">action_identity_class</a>,
            });
            <b>return</b>
        };
        // Different reaction, update existing one
        // Decrease count <b>for</b> previous reaction
        <b>let</b> previous_count = *table::borrow(&comment.reaction_counts, previous_reaction);
        <b>if</b> (previous_count &lt;= 1) {
            table::remove(&<b>mut</b> comment.reaction_counts, previous_reaction);
        } <b>else</b> {
            *table::borrow_mut(&<b>mut</b> comment.reaction_counts, previous_reaction) = previous_count - 1;
        };
        // Update user's reaction
        *table::borrow_mut(&<b>mut</b> comment.user_reactions, <a href="../social_contracts/post.md#social_contracts_post_actor_address">actor_address</a>) = reaction;
    } <b>else</b> {
        // New reaction from this user
        table::add(&<b>mut</b> comment.user_reactions, <a href="../social_contracts/post.md#social_contracts_post_actor_address">actor_address</a>, reaction);
        // Increment comment reaction count
        <b>assert</b>!(comment.reaction_count &lt;= <a href="../social_contracts/post.md#social_contracts_post_MAX_U64">MAX_U64</a> - 1, <a href="../social_contracts/post.md#social_contracts_post_EOverflow">EOverflow</a>);
        comment.reaction_count = comment.reaction_count + 1;
    };
    // Increment count <b>for</b> the reaction
    <b>if</b> (table::contains(&comment.reaction_counts, reaction)) {
        <b>let</b> count = *table::borrow(&comment.reaction_counts, reaction);
        <b>assert</b>!(count &lt;= <a href="../social_contracts/post.md#social_contracts_post_MAX_U64">MAX_U64</a> - 1, <a href="../social_contracts/post.md#social_contracts_post_EOverflow">EOverflow</a>);
        *table::borrow_mut(&<b>mut</b> comment.reaction_counts, reaction) = count + 1;
    } <b>else</b> {
        table::add(&<b>mut</b> comment.reaction_counts, reaction, 1);
    };
    // Emit reaction event
    event::emit(<a href="../social_contracts/post.md#social_contracts_post_ReactionEvent">ReactionEvent</a> {
        object_id: object::uid_to_address(&comment.id),
        user: <a href="../social_contracts/post.md#social_contracts_post_actor_address">actor_address</a>,
        reaction,
        is_post: <b>false</b>,
        principal_owner,
        <a href="../social_contracts/post.md#social_contracts_post_actor_address">actor_address</a>,
        <a href="../social_contracts/post.md#social_contracts_post_sub_agent_id">sub_agent_id</a>,
        organization_id,
        <a href="../social_contracts/post.md#social_contracts_post_action_identity_class">action_identity_class</a>,
    });
}
</code></pre>



</details>

<a name="social_contracts_post_edit_post"></a>

## Function `edit_post`

Edit a post through the delegated-agent authorization model.


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_edit_post">edit_post</a>(registry: &<a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">social_contracts::profile::UsernameRegistry</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, config: &<a href="../social_contracts/post.md#social_contracts_post_PostConfig">social_contracts::post::PostConfig</a>, memory_config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, memory_account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, content: <a href="../std/string.md#std_string_String">std::string::String</a>, media_urls: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;&gt;, mentions: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;<b>address</b>&gt;&gt;, metadata_json: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_edit_post">edit_post</a>(
    registry: &UsernameRegistry,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">platform::Platform</a>,
    block_list_registry: &BlockListRegistry,
    config: &<a href="../social_contracts/post.md#social_contracts_post_PostConfig">PostConfig</a>,
    memory_config: &MemoryConfig,
    memory_account: &MemoryAccount,
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>,
    content: String,
    <b>mut</b> media_urls: Option&lt;vector&lt;String&gt;&gt;,
    mentions: Option&lt;vector&lt;<b>address</b>&gt;&gt;,
    metadata_json: Option&lt;String&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> acting = <a href="../social_contracts/post.md#social_contracts_post_resolve_social_actor">resolve_social_actor</a>(
        memory_config, registry, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, block_list_registry, memory_account,
        <a href="../social_contracts/memory.md#social_contracts_memory_cap_post_publish">memory::cap_post_publish</a>(), 0, clock, ctx,
    );
    <b>assert</b>!(<a href="../social_contracts/memory.md#social_contracts_memory_acting_principal_owner">memory::acting_principal_owner</a>(&acting) == <a href="../social_contracts/post.md#social_contracts_post">post</a>.owner, <a href="../social_contracts/post.md#social_contracts_post_EUnauthorized">EUnauthorized</a>);
    <b>assert</b>!(string::length(&content) &lt;= config.max_content_length, <a href="../social_contracts/post.md#social_contracts_post_EContentTooLarge">EContentTooLarge</a>);
    <b>if</b> (option::is_some(&metadata_json)) {
        <b>let</b> metadata_string = option::borrow(&metadata_json);
        <b>assert</b>!(string::length(metadata_string) &lt;= config.max_metadata_size, <a href="../social_contracts/post.md#social_contracts_post_EContentTooLarge">EContentTooLarge</a>);
        <a href="../social_contracts/post.md#social_contracts_post">post</a>.metadata_json = option::some(*metadata_string);
    };
    <b>if</b> (option::is_some(&media_urls)) {
        <b>let</b> url_strings = option::extract(&<b>mut</b> media_urls);
        <b>assert</b>!(vector::length(&url_strings) &lt;= config.max_media_urls, <a href="../social_contracts/post.md#social_contracts_post_ETooManyMediaUrls">ETooManyMediaUrls</a>);
        <b>let</b> <b>mut</b> urls = vector::empty&lt;Url&gt;();
        <b>let</b> <b>mut</b> i = 0;
        <b>let</b> len = vector::length(&url_strings);
        <b>while</b> (i &lt; len) {
            <b>let</b> url_string = vector::borrow(&url_strings, i);
            vector::push_back(
                &<b>mut</b> urls,
                url::new_unsafe_from_bytes(*string::as_bytes(url_string)),
            );
            i = i + 1;
        };
        <a href="../social_contracts/post.md#social_contracts_post">post</a>.media = option::some(urls);
    };
    <b>if</b> (option::is_some(&mentions)) {
        <b>assert</b>!(vector::length(option::borrow(&mentions)) &lt;= config.max_mentions, <a href="../social_contracts/post.md#social_contracts_post_EContentTooLarge">EContentTooLarge</a>);
        <a href="../social_contracts/post.md#social_contracts_post">post</a>.mentions = mentions;
    };
    <a href="../social_contracts/post.md#social_contracts_post">post</a>.content = content;
    event::emit(<a href="../social_contracts/post.md#social_contracts_post_PostUpdatedEvent">PostUpdatedEvent</a> {
        post_id: object::uid_to_address(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id),
        owner: <a href="../social_contracts/post.md#social_contracts_post">post</a>.owner,
        profile_id: <a href="../social_contracts/post.md#social_contracts_post">post</a>.profile_id,
        content: <a href="../social_contracts/post.md#social_contracts_post">post</a>.content,
        metadata_json: <a href="../social_contracts/post.md#social_contracts_post">post</a>.metadata_json,
        updated_at: clock::timestamp_ms(clock),
    });
}
</code></pre>



</details>

<a name="social_contracts_post_edit_comment"></a>

## Function `edit_comment`

Edit a comment through the delegated-agent authorization model.


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_edit_comment">edit_comment</a>(registry: &<a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">social_contracts::profile::UsernameRegistry</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, config: &<a href="../social_contracts/post.md#social_contracts_post_PostConfig">social_contracts::post::PostConfig</a>, memory_config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, memory_account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, comment: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Comment">social_contracts::post::Comment</a>, content: <a href="../std/string.md#std_string_String">std::string::String</a>, mentions: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;<b>address</b>&gt;&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_edit_comment">edit_comment</a>(
    registry: &UsernameRegistry,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">platform::Platform</a>,
    block_list_registry: &BlockListRegistry,
    config: &<a href="../social_contracts/post.md#social_contracts_post_PostConfig">PostConfig</a>,
    memory_config: &MemoryConfig,
    memory_account: &MemoryAccount,
    comment: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Comment">Comment</a>,
    content: String,
    mentions: Option&lt;vector&lt;<b>address</b>&gt;&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> acting = <a href="../social_contracts/post.md#social_contracts_post_resolve_social_actor">resolve_social_actor</a>(
        memory_config, registry, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, block_list_registry, memory_account,
        <a href="../social_contracts/memory.md#social_contracts_memory_cap_comment">memory::cap_comment</a>(), 0, clock, ctx,
    );
    <b>assert</b>!(<a href="../social_contracts/memory.md#social_contracts_memory_acting_principal_owner">memory::acting_principal_owner</a>(&acting) == comment.owner, <a href="../social_contracts/post.md#social_contracts_post_EUnauthorized">EUnauthorized</a>);
    <b>assert</b>!(string::length(&content) &lt;= config.max_content_length, <a href="../social_contracts/post.md#social_contracts_post_EContentTooLarge">EContentTooLarge</a>);
    <b>if</b> (option::is_some(&mentions)) {
        <b>assert</b>!(vector::length(option::borrow(&mentions)) &lt;= config.max_mentions, <a href="../social_contracts/post.md#social_contracts_post_EContentTooLarge">EContentTooLarge</a>);
        comment.mentions = mentions;
    };
    comment.content = content;
    event::emit(<a href="../social_contracts/post.md#social_contracts_post_CommentUpdatedEvent">CommentUpdatedEvent</a> {
        comment_id: object::uid_to_address(&comment.id),
        post_id: comment.post_id,
        owner: comment.owner,
        profile_id: comment.profile_id,
        content: comment.content,
        updated_at: clock::timestamp_ms(clock),
    });
}
</code></pre>



</details>

<a name="social_contracts_post_remove_post_reaction"></a>

## Function `remove_post_reaction`

Remove the caller principal's current post reaction without toggle semantics.


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_remove_post_reaction">remove_post_reaction</a>(registry: &<a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">social_contracts::profile::UsernameRegistry</a>, <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, memory_config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, memory_account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_remove_post_reaction">remove_post_reaction</a>(
    registry: &UsernameRegistry,
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">platform::Platform</a>,
    block_list_registry: &BlockListRegistry,
    memory_config: &MemoryConfig,
    memory_account: &MemoryAccount,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> acting = <a href="../social_contracts/post.md#social_contracts_post_resolve_social_actor">resolve_social_actor</a>(
        memory_config, registry, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, block_list_registry, memory_account,
        <a href="../social_contracts/memory.md#social_contracts_memory_cap_react">memory::cap_react</a>(), 0, clock, ctx,
    );
    <b>let</b> <a href="../social_contracts/post.md#social_contracts_post_actor_address">actor_address</a> = <a href="../social_contracts/memory.md#social_contracts_memory_acting_actor_address">memory::acting_actor_address</a>(&acting);
    <b>let</b> principal_owner = <a href="../social_contracts/memory.md#social_contracts_memory_acting_principal_owner">memory::acting_principal_owner</a>(&acting);
    <b>assert</b>!(table::contains(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.user_reactions, <a href="../social_contracts/post.md#social_contracts_post_actor_address">actor_address</a>), <a href="../social_contracts/post.md#social_contracts_post_EUnauthorized">EUnauthorized</a>);
    <b>let</b> reaction = table::remove(&<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post">post</a>.user_reactions, <a href="../social_contracts/post.md#social_contracts_post_actor_address">actor_address</a>);
    <b>let</b> count = *table::borrow(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.reaction_counts, reaction);
    <b>if</b> (count &lt;= 1) {
        table::remove(&<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post">post</a>.reaction_counts, reaction);
    } <b>else</b> {
        *table::borrow_mut(&<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post">post</a>.reaction_counts, reaction) = count - 1;
    };
    <b>assert</b>!(<a href="../social_contracts/post.md#social_contracts_post">post</a>.reaction_count &gt; 0, <a href="../social_contracts/post.md#social_contracts_post_EOverflow">EOverflow</a>);
    <a href="../social_contracts/post.md#social_contracts_post">post</a>.reaction_count = <a href="../social_contracts/post.md#social_contracts_post">post</a>.reaction_count - 1;
    event::emit(<a href="../social_contracts/post.md#social_contracts_post_RemoveReactionEvent">RemoveReactionEvent</a> {
        object_id: object::uid_to_address(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id),
        user: <a href="../social_contracts/post.md#social_contracts_post_actor_address">actor_address</a>,
        reaction,
        is_post: <b>true</b>,
        principal_owner,
        <a href="../social_contracts/post.md#social_contracts_post_actor_address">actor_address</a>,
        <a href="../social_contracts/post.md#social_contracts_post_sub_agent_id">sub_agent_id</a>: <a href="../social_contracts/memory.md#social_contracts_memory_acting_sub_agent_id">memory::acting_sub_agent_id</a>(&acting),
        organization_id: <a href="../social_contracts/memory.md#social_contracts_memory_acting_organization_id">memory::acting_organization_id</a>(&acting),
        <a href="../social_contracts/post.md#social_contracts_post_action_identity_class">action_identity_class</a>: <a href="../social_contracts/memory.md#social_contracts_memory_acting_identity_class">memory::acting_identity_class</a>(&acting),
    });
}
</code></pre>



</details>

<a name="social_contracts_post_remove_comment_reaction"></a>

## Function `remove_comment_reaction`

Remove the caller principal's current comment reaction without toggle semantics.


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_remove_comment_reaction">remove_comment_reaction</a>(registry: &<a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">social_contracts::profile::UsernameRegistry</a>, comment: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Comment">social_contracts::post::Comment</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, memory_config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, memory_account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_remove_comment_reaction">remove_comment_reaction</a>(
    registry: &UsernameRegistry,
    comment: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Comment">Comment</a>,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">platform::Platform</a>,
    block_list_registry: &BlockListRegistry,
    memory_config: &MemoryConfig,
    memory_account: &MemoryAccount,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> acting = <a href="../social_contracts/post.md#social_contracts_post_resolve_social_actor">resolve_social_actor</a>(
        memory_config, registry, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, block_list_registry, memory_account,
        <a href="../social_contracts/memory.md#social_contracts_memory_cap_react">memory::cap_react</a>(), 0, clock, ctx,
    );
    <b>let</b> <a href="../social_contracts/post.md#social_contracts_post_actor_address">actor_address</a> = <a href="../social_contracts/memory.md#social_contracts_memory_acting_actor_address">memory::acting_actor_address</a>(&acting);
    <b>let</b> principal_owner = <a href="../social_contracts/memory.md#social_contracts_memory_acting_principal_owner">memory::acting_principal_owner</a>(&acting);
    <b>assert</b>!(table::contains(&comment.user_reactions, <a href="../social_contracts/post.md#social_contracts_post_actor_address">actor_address</a>), <a href="../social_contracts/post.md#social_contracts_post_EUnauthorized">EUnauthorized</a>);
    <b>let</b> reaction = table::remove(&<b>mut</b> comment.user_reactions, <a href="../social_contracts/post.md#social_contracts_post_actor_address">actor_address</a>);
    <b>let</b> count = *table::borrow(&comment.reaction_counts, reaction);
    <b>if</b> (count &lt;= 1) {
        table::remove(&<b>mut</b> comment.reaction_counts, reaction);
    } <b>else</b> {
        *table::borrow_mut(&<b>mut</b> comment.reaction_counts, reaction) = count - 1;
    };
    <b>assert</b>!(comment.reaction_count &gt; 0, <a href="../social_contracts/post.md#social_contracts_post_EOverflow">EOverflow</a>);
    comment.reaction_count = comment.reaction_count - 1;
    event::emit(<a href="../social_contracts/post.md#social_contracts_post_RemoveReactionEvent">RemoveReactionEvent</a> {
        object_id: object::uid_to_address(&comment.id),
        user: <a href="../social_contracts/post.md#social_contracts_post_actor_address">actor_address</a>,
        reaction,
        is_post: <b>false</b>,
        principal_owner,
        <a href="../social_contracts/post.md#social_contracts_post_actor_address">actor_address</a>,
        <a href="../social_contracts/post.md#social_contracts_post_sub_agent_id">sub_agent_id</a>: <a href="../social_contracts/memory.md#social_contracts_memory_acting_sub_agent_id">memory::acting_sub_agent_id</a>(&acting),
        organization_id: <a href="../social_contracts/memory.md#social_contracts_memory_acting_organization_id">memory::acting_organization_id</a>(&acting),
        <a href="../social_contracts/post.md#social_contracts_post_action_identity_class">action_identity_class</a>: <a href="../social_contracts/memory.md#social_contracts_memory_acting_identity_class">memory::acting_identity_class</a>(&acting),
    });
}
</code></pre>



</details>

<a name="social_contracts_post_remove_repost"></a>

## Function `remove_repost`

Remove a repost and decrement its original post's aggregate count.


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_remove_repost">remove_repost</a>(registry: &<a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">social_contracts::profile::UsernameRegistry</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, memory_config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, memory_account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, original_post: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, repost: <a href="../social_contracts/post.md#social_contracts_post_Repost">social_contracts::post::Repost</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_remove_repost">remove_repost</a>(
    registry: &UsernameRegistry,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">platform::Platform</a>,
    block_list_registry: &BlockListRegistry,
    memory_config: &MemoryConfig,
    memory_account: &MemoryAccount,
    original_post: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>,
    repost: <a href="../social_contracts/post.md#social_contracts_post_Repost">Repost</a>,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> acting = <a href="../social_contracts/post.md#social_contracts_post_resolve_social_actor">resolve_social_actor</a>(
        memory_config, registry, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, block_list_registry, memory_account,
        <a href="../social_contracts/memory.md#social_contracts_memory_cap_post_publish">memory::cap_post_publish</a>(), 0, clock, ctx,
    );
    <b>let</b> principal_owner = <a href="../social_contracts/memory.md#social_contracts_memory_acting_principal_owner">memory::acting_principal_owner</a>(&acting);
    <b>assert</b>!(principal_owner == repost.owner, <a href="../social_contracts/post.md#social_contracts_post_EUnauthorized">EUnauthorized</a>);
    <b>assert</b>!(repost.is_original_post, <a href="../social_contracts/post.md#social_contracts_post_EInvalidParentReference">EInvalidParentReference</a>);
    <b>assert</b>!(repost.original_id == object::uid_to_address(&original_post.id), <a href="../social_contracts/post.md#social_contracts_post_EInvalidParentReference">EInvalidParentReference</a>);
    <b>assert</b>!(original_post.repost_count &gt; 0, <a href="../social_contracts/post.md#social_contracts_post_EOverflow">EOverflow</a>);
    original_post.repost_count = original_post.repost_count - 1;
    <b>let</b> repost_id = object::uid_to_address(&repost.id);
    event::emit(<a href="../social_contracts/post.md#social_contracts_post_RepostRemovedEvent">RepostRemovedEvent</a> {
        repost_id,
        original_id: repost.original_id,
        owner: principal_owner,
        <a href="../social_contracts/post.md#social_contracts_post_actor_address">actor_address</a>: <a href="../social_contracts/memory.md#social_contracts_memory_acting_actor_address">memory::acting_actor_address</a>(&acting),
        <a href="../social_contracts/post.md#social_contracts_post_sub_agent_id">sub_agent_id</a>: <a href="../social_contracts/memory.md#social_contracts_memory_acting_sub_agent_id">memory::acting_sub_agent_id</a>(&acting),
        organization_id: <a href="../social_contracts/memory.md#social_contracts_memory_acting_organization_id">memory::acting_organization_id</a>(&acting),
        <a href="../social_contracts/post.md#social_contracts_post_action_identity_class">action_identity_class</a>: <a href="../social_contracts/memory.md#social_contracts_memory_acting_identity_class">memory::acting_identity_class</a>(&acting),
        removed_at: clock::timestamp_ms(clock),
    });
    <b>let</b> <a href="../social_contracts/post.md#social_contracts_post_Repost">Repost</a> { id, original_id: _, is_original_post: _, owner: _, profile_id: _, created_at: _, <a href="../social_contracts/post.md#social_contracts_post_version">version</a>: _ } = repost;
    object::delete(id);
}
</code></pre>



</details>

<a name="social_contracts_post_get_post_content"></a>

## Function `get_post_content`

Get post content


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_get_post_content">get_post_content</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>): <a href="../std/string.md#std_string_String">std::string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_get_post_content">get_post_content</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>): String {
    <a href="../social_contracts/post.md#social_contracts_post">post</a>.content
}
</code></pre>



</details>

<a name="social_contracts_post_get_post_owner"></a>

## Function `get_post_owner`

Get post owner


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_get_post_owner">get_post_owner</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_get_post_owner">get_post_owner</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>): <b>address</b> {
    <a href="../social_contracts/post.md#social_contracts_post">post</a>.owner
}
</code></pre>



</details>

<a name="social_contracts_post_get_post_id"></a>

## Function `get_post_id`

Get post ID


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_get_post_id">get_post_id</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>): &<a href="../myso/object.md#myso_object_UID">myso::object::UID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_get_post_id">get_post_id</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>): &UID {
    &<a href="../social_contracts/post.md#social_contracts_post">post</a>.id
}
</code></pre>



</details>

<a name="social_contracts_post_get_post_comment_count"></a>

## Function `get_post_comment_count`

Get post comment count


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_get_post_comment_count">get_post_comment_count</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_get_post_comment_count">get_post_comment_count</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>): u64 {
    <a href="../social_contracts/post.md#social_contracts_post">post</a>.comment_count
}
</code></pre>



</details>

<a name="social_contracts_post_get_comment_owner"></a>

## Function `get_comment_owner`

Get comment owner


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_get_comment_owner">get_comment_owner</a>(comment: &<a href="../social_contracts/post.md#social_contracts_post_Comment">social_contracts::post::Comment</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_get_comment_owner">get_comment_owner</a>(comment: &<a href="../social_contracts/post.md#social_contracts_post_Comment">Comment</a>): <b>address</b> {
    comment.owner
}
</code></pre>



</details>

<a name="social_contracts_post_get_comment_post_id"></a>

## Function `get_comment_post_id`

Get comment post ID


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_get_comment_post_id">get_comment_post_id</a>(comment: &<a href="../social_contracts/post.md#social_contracts_post_Comment">social_contracts::post::Comment</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_get_comment_post_id">get_comment_post_id</a>(comment: &<a href="../social_contracts/post.md#social_contracts_post_Comment">Comment</a>): <b>address</b> {
    comment.post_id
}
</code></pre>



</details>

<a name="social_contracts_post_get_id_address"></a>

## Function `get_id_address`

Get the ID address of a post


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_get_id_address">get_id_address</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_get_id_address">get_id_address</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>): <b>address</b> {
    object::uid_to_address(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id)
}
</code></pre>



</details>

<a name="social_contracts_post_get_reaction_count"></a>

## Function `get_reaction_count`

Get the reaction count of a post


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_get_reaction_count">get_reaction_count</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_get_reaction_count">get_reaction_count</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>): u64 {
    <a href="../social_contracts/post.md#social_contracts_post">post</a>.reaction_count
}
</code></pre>



</details>

<a name="social_contracts_post_get_tips_received"></a>

## Function `get_tips_received`

Get the tips received for a post


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_get_tips_received">get_tips_received</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_get_tips_received">get_tips_received</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>): u64 {
    <a href="../social_contracts/post.md#social_contracts_post">post</a>.tips_received
}
</code></pre>



</details>

<a name="social_contracts_post_get_comment_tips_received"></a>

## Function `get_comment_tips_received`

Get the tips received for a comment (commenter share of tip_comment / tip_comment_simple).


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_get_comment_tips_received">get_comment_tips_received</a>(comment: &<a href="../social_contracts/post.md#social_contracts_post_Comment">social_contracts::post::Comment</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_get_comment_tips_received">get_comment_tips_received</a>(comment: &<a href="../social_contracts/post.md#social_contracts_post_Comment">Comment</a>): u64 {
    comment.tips_received
}
</code></pre>



</details>

<a name="social_contracts_post_get_platform_id"></a>

## Function `get_platform_id`

Get the platform ID for a post


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_get_platform_id">get_platform_id</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_get_platform_id">get_platform_id</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>): <b>address</b> {
    <a href="../social_contracts/post.md#social_contracts_post">post</a>.platform_id
}
</code></pre>



</details>

<a name="social_contracts_post_get_revenue_redirect_to"></a>

## Function `get_revenue_redirect_to`

Get the revenue redirect address for a post (legacy compat stub — always none).


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_get_revenue_redirect_to">get_revenue_redirect_to</a>(_post: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_get_revenue_redirect_to">get_revenue_redirect_to</a>(_post: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>): Option&lt;<b>address</b>&gt; {
    option::none()
}
</code></pre>



</details>

<a name="social_contracts_post_get_revenue_redirect_percentage"></a>

## Function `get_revenue_redirect_percentage`

Get the revenue redirect percentage for a post (legacy compat stub — always none).


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_get_revenue_redirect_percentage">get_revenue_redirect_percentage</a>(_post: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_get_revenue_redirect_percentage">get_revenue_redirect_percentage</a>(_post: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>): Option&lt;u64&gt; {
    option::none()
}
</code></pre>



</details>

<a name="social_contracts_post_poc_redirection_kind"></a>

## Function `poc_redirection_kind`

Legacy compat — derived from cached revenue manifest payout modes.


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_poc_redirection_kind">poc_redirection_kind</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_poc_redirection_kind">poc_redirection_kind</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>): u8 {
    <b>let</b> manifest_opt = <a href="../social_contracts/post.md#social_contracts_post_revenue_manifest">revenue_manifest</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>);
    <b>if</b> (option::is_none(&manifest_opt)) {
        <b>return</b> 0
    };
    <b>let</b> manifest = option::borrow(&manifest_opt);
    <b>if</b> (<a href="../social_contracts/media_asset.md#social_contracts_media_asset_manifest_uses_escrow_redirect">media_asset::manifest_uses_escrow_redirect</a>(manifest)) {
        2
    } <b>else</b> {
        1
    }
}
</code></pre>



</details>

<a name="social_contracts_post_poc_redirection_none"></a>

## Function `poc_redirection_none`

Legacy compat stub for fee-routing callers during migration.


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_poc_redirection_none">poc_redirection_none</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_poc_redirection_none">poc_redirection_none</a>(): u8 { 0 }
</code></pre>



</details>

<a name="social_contracts_post_version"></a>

## Function `version`

Get the version of a post


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_version">version</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_version">version</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>): u64 {
    <a href="../social_contracts/post.md#social_contracts_post">post</a>.<a href="../social_contracts/post.md#social_contracts_post_version">version</a>
}
</code></pre>



</details>

<a name="social_contracts_post_borrow_version_mut"></a>

## Function `borrow_version_mut`

Get a mutable reference to the post version (for upgrade module)


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_borrow_version_mut">borrow_version_mut</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>): &<b>mut</b> u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_borrow_version_mut">borrow_version_mut</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>): &<b>mut</b> u64 {
    &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post">post</a>.<a href="../social_contracts/post.md#social_contracts_post_version">version</a>
}
</code></pre>



</details>

<a name="social_contracts_post_comment_version"></a>

## Function `comment_version`

Get the version of a comment


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_comment_version">comment_version</a>(comment: &<a href="../social_contracts/post.md#social_contracts_post_Comment">social_contracts::post::Comment</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_comment_version">comment_version</a>(comment: &<a href="../social_contracts/post.md#social_contracts_post_Comment">Comment</a>): u64 {
    comment.<a href="../social_contracts/post.md#social_contracts_post_version">version</a>
}
</code></pre>



</details>

<a name="social_contracts_post_borrow_comment_version_mut"></a>

## Function `borrow_comment_version_mut`

Get a mutable reference to the comment version (for upgrade module)


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_borrow_comment_version_mut">borrow_comment_version_mut</a>(comment: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Comment">social_contracts::post::Comment</a>): &<b>mut</b> u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_borrow_comment_version_mut">borrow_comment_version_mut</a>(comment: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Comment">Comment</a>): &<b>mut</b> u64 {
    &<b>mut</b> comment.<a href="../social_contracts/post.md#social_contracts_post_version">version</a>
}
</code></pre>



</details>

<a name="social_contracts_post_repost_version"></a>

## Function `repost_version`

Get the version of a repost


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_repost_version">repost_version</a>(repost: &<a href="../social_contracts/post.md#social_contracts_post_Repost">social_contracts::post::Repost</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_repost_version">repost_version</a>(repost: &<a href="../social_contracts/post.md#social_contracts_post_Repost">Repost</a>): u64 {
    repost.<a href="../social_contracts/post.md#social_contracts_post_version">version</a>
}
</code></pre>



</details>

<a name="social_contracts_post_borrow_repost_version_mut"></a>

## Function `borrow_repost_version_mut`

Get a mutable reference to the repost version (for upgrade module)


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_borrow_repost_version_mut">borrow_repost_version_mut</a>(repost: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Repost">social_contracts::post::Repost</a>): &<b>mut</b> u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_borrow_repost_version_mut">borrow_repost_version_mut</a>(repost: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Repost">Repost</a>): &<b>mut</b> u64 {
    &<b>mut</b> repost.<a href="../social_contracts/post.md#social_contracts_post_version">version</a>
}
</code></pre>



</details>

<a name="social_contracts_post_migrate_post"></a>

## Function `migrate_post`

Migration function for Post


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_migrate_post">migrate_post</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, _: &<a href="../social_contracts/upgrade.md#social_contracts_upgrade_UpgradeAdminCap">social_contracts::upgrade::UpgradeAdminCap</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_migrate_post">migrate_post</a>(
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>,
    _: &UpgradeAdminCap,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> current_version = <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>();
    // Verify this is an <a href="../social_contracts/upgrade.md#social_contracts_upgrade">upgrade</a> (new <a href="../social_contracts/post.md#social_contracts_post_version">version</a> &gt; current <a href="../social_contracts/post.md#social_contracts_post_version">version</a>)
    <b>assert</b>!(<a href="../social_contracts/post.md#social_contracts_post">post</a>.<a href="../social_contracts/post.md#social_contracts_post_version">version</a> &lt; current_version, <a href="../social_contracts/post.md#social_contracts_post_EWrongVersion">EWrongVersion</a>);
    // Remember old <a href="../social_contracts/post.md#social_contracts_post_version">version</a> and update to new <a href="../social_contracts/post.md#social_contracts_post_version">version</a>
    <b>let</b> old_version = <a href="../social_contracts/post.md#social_contracts_post">post</a>.<a href="../social_contracts/post.md#social_contracts_post_version">version</a>;
    <b>if</b> (old_version &lt; 2) {
        <a href="../social_contracts/post.md#social_contracts_post">post</a>.<a href="../social_contracts/post.md#social_contracts_post_composition_status">composition_status</a> = <a href="../social_contracts/media_asset.md#social_contracts_media_asset_composition_none">media_asset::composition_none</a>();
        <a href="../social_contracts/post.md#social_contracts_post">post</a>.<a href="../social_contracts/post.md#social_contracts_post_monetization_status">monetization_status</a> = <a href="../social_contracts/media_asset.md#social_contracts_media_asset_monetization_none">media_asset::monetization_none</a>();
        <a href="../social_contracts/post.md#social_contracts_post">post</a>.<a href="../social_contracts/post.md#social_contracts_post_composition_badge">composition_badge</a> = option::none();
    };
    <b>if</b> (old_version &lt; 3) {
        <b>if</b> (!df::exists_with_type&lt;vector&lt;u8&gt;, <a href="../social_contracts/post.md#social_contracts_post_PostAttribution">PostAttribution</a>&gt;(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id, <a href="../social_contracts/post.md#social_contracts_post_POST_ATTRIBUTION_DF_KEY">POST_ATTRIBUTION_DF_KEY</a>)) {
            <b>let</b> owner = <a href="../social_contracts/post.md#social_contracts_post">post</a>.owner;
            <a href="../social_contracts/post.md#social_contracts_post_attach_post_attribution">attach_post_attribution</a>(
                <a href="../social_contracts/post.md#social_contracts_post">post</a>,
                owner,
                option::none(),
                option::none(),
                <a href="../social_contracts/memory.md#social_contracts_memory_class_human">memory::class_human</a>(),
            );
        };
    };
    <a href="../social_contracts/post.md#social_contracts_post">post</a>.<a href="../social_contracts/post.md#social_contracts_post_version">version</a> = current_version;
    // Initialize platform_id <b>for</b> existing posts (set to @0x0 <b>as</b> sentinel <b>for</b> unknown <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>)
    // This field was added in a later <a href="../social_contracts/post.md#social_contracts_post_version">version</a> - existing posts will have @0x0
    // Platform-specific features may require manual update of platform_id
    // Note: This assumes platform_id field exists. If migrating from <a href="../social_contracts/post.md#social_contracts_post_version">version</a> before platform_id was added,
    // the field will be initialized to @0x0 by default.
    // Emit event <b>for</b> object migration
    <b>let</b> post_id = object::id(<a href="../social_contracts/post.md#social_contracts_post">post</a>);
    <a href="../social_contracts/upgrade.md#social_contracts_upgrade_emit_migration_event">upgrade::emit_migration_event</a>(
        post_id,
        string::utf8(<a href="../social_contracts/post.md#social_contracts_post_POST_TYPE_STANDARD">POST_TYPE_STANDARD</a>),
        old_version,
        tx_context::sender(ctx)
    );
    // Any migration logic can be added here <b>for</b> future upgrades
}
</code></pre>



</details>

<a name="social_contracts_post_migrate_comment"></a>

## Function `migrate_comment`

Migration function for Comment


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_migrate_comment">migrate_comment</a>(comment: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Comment">social_contracts::post::Comment</a>, _: &<a href="../social_contracts/upgrade.md#social_contracts_upgrade_UpgradeAdminCap">social_contracts::upgrade::UpgradeAdminCap</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_migrate_comment">migrate_comment</a>(
    comment: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Comment">Comment</a>,
    _: &UpgradeAdminCap,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> current_version = <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>();
    // Verify this is an <a href="../social_contracts/upgrade.md#social_contracts_upgrade">upgrade</a> (new <a href="../social_contracts/post.md#social_contracts_post_version">version</a> &gt; current <a href="../social_contracts/post.md#social_contracts_post_version">version</a>)
    <b>assert</b>!(comment.<a href="../social_contracts/post.md#social_contracts_post_version">version</a> &lt; current_version, <a href="../social_contracts/post.md#social_contracts_post_EWrongVersion">EWrongVersion</a>);
    // Remember old <a href="../social_contracts/post.md#social_contracts_post_version">version</a> and update to new <a href="../social_contracts/post.md#social_contracts_post_version">version</a>
    <b>let</b> old_version = comment.<a href="../social_contracts/post.md#social_contracts_post_version">version</a>;
    comment.<a href="../social_contracts/post.md#social_contracts_post_version">version</a> = current_version;
    // Emit event <b>for</b> object migration
    <b>let</b> comment_id = object::id(comment);
    <a href="../social_contracts/upgrade.md#social_contracts_upgrade_emit_migration_event">upgrade::emit_migration_event</a>(
        comment_id,
        string::utf8(b"<a href="../social_contracts/post.md#social_contracts_post_Comment">Comment</a>"),
        old_version,
        tx_context::sender(ctx)
    );
    // Any migration logic can be added here <b>for</b> future upgrades
}
</code></pre>



</details>

<a name="social_contracts_post_migrate_repost"></a>

## Function `migrate_repost`

Migration function for Repost


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_migrate_repost">migrate_repost</a>(repost: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Repost">social_contracts::post::Repost</a>, _: &<a href="../social_contracts/upgrade.md#social_contracts_upgrade_UpgradeAdminCap">social_contracts::upgrade::UpgradeAdminCap</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_migrate_repost">migrate_repost</a>(
    repost: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Repost">Repost</a>,
    _: &UpgradeAdminCap,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> current_version = <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>();
    // Verify this is an <a href="../social_contracts/upgrade.md#social_contracts_upgrade">upgrade</a> (new <a href="../social_contracts/post.md#social_contracts_post_version">version</a> &gt; current <a href="../social_contracts/post.md#social_contracts_post_version">version</a>)
    <b>assert</b>!(repost.<a href="../social_contracts/post.md#social_contracts_post_version">version</a> &lt; current_version, <a href="../social_contracts/post.md#social_contracts_post_EWrongVersion">EWrongVersion</a>);
    // Remember old <a href="../social_contracts/post.md#social_contracts_post_version">version</a> and update to new <a href="../social_contracts/post.md#social_contracts_post_version">version</a>
    <b>let</b> old_version = repost.<a href="../social_contracts/post.md#social_contracts_post_version">version</a>;
    repost.<a href="../social_contracts/post.md#social_contracts_post_version">version</a> = current_version;
    // Emit event <b>for</b> object migration
    <b>let</b> repost_id = object::id(repost);
    <a href="../social_contracts/upgrade.md#social_contracts_upgrade_emit_migration_event">upgrade::emit_migration_event</a>(
        repost_id,
        string::utf8(b"<a href="../social_contracts/post.md#social_contracts_post_Repost">Repost</a>"),
        old_version,
        tx_context::sender(ctx)
    );
    // Any migration logic can be added here <b>for</b> future upgrades
}
</code></pre>



</details>

<a name="social_contracts_post_migrate_post_config"></a>

## Function `migrate_post_config`

Migration function for PostConfig


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_migrate_post_config">migrate_post_config</a>(config: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_PostConfig">social_contracts::post::PostConfig</a>, _: &<a href="../social_contracts/upgrade.md#social_contracts_upgrade_UpgradeAdminCap">social_contracts::upgrade::UpgradeAdminCap</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_migrate_post_config">migrate_post_config</a>(
    config: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_PostConfig">PostConfig</a>,
    _: &UpgradeAdminCap,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> current_version = <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>();
    // Verify this is an <a href="../social_contracts/upgrade.md#social_contracts_upgrade">upgrade</a> (new <a href="../social_contracts/post.md#social_contracts_post_version">version</a> &gt; current <a href="../social_contracts/post.md#social_contracts_post_version">version</a>)
    <b>assert</b>!(config.<a href="../social_contracts/post.md#social_contracts_post_version">version</a> &lt; current_version, <a href="../social_contracts/post.md#social_contracts_post_EWrongVersion">EWrongVersion</a>);
    // Remember old <a href="../social_contracts/post.md#social_contracts_post_version">version</a> and update to new <a href="../social_contracts/post.md#social_contracts_post_version">version</a>
    <b>let</b> old_version = config.<a href="../social_contracts/post.md#social_contracts_post_version">version</a>;
    config.<a href="../social_contracts/post.md#social_contracts_post_version">version</a> = current_version;
    // Seed promotion fee bps <b>for</b> configs created before these fields existed
    config.platform_fee_bps = <a href="../social_contracts/post.md#social_contracts_post_DEFAULT_PLATFORM_FEE_BPS">DEFAULT_PLATFORM_FEE_BPS</a>;
    config.ecosystem_fee_bps = <a href="../social_contracts/post.md#social_contracts_post_DEFAULT_ECOSYSTEM_FEE_BPS">DEFAULT_ECOSYSTEM_FEE_BPS</a>;
    // Emit event <b>for</b> object migration
    <b>let</b> config_id = object::id(config);
    <a href="../social_contracts/upgrade.md#social_contracts_upgrade_emit_migration_event">upgrade::emit_migration_event</a>(
        config_id,
        string::utf8(b"<a href="../social_contracts/post.md#social_contracts_post_PostConfig">PostConfig</a>"),
        old_version,
        tx_context::sender(ctx)
    );
}
</code></pre>



</details>

<a name="social_contracts_post_update_post_parameters"></a>

## Function `update_post_parameters`

Update post parameters (admin only)


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_update_post_parameters">update_post_parameters</a>(_admin_cap: &<a href="../social_contracts/post.md#social_contracts_post_PostAdminCap">social_contracts::post::PostAdminCap</a>, config: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_PostConfig">social_contracts::post::PostConfig</a>, max_content_length: u64, max_media_urls: u64, max_mentions: u64, max_metadata_size: u64, max_description_length: u64, max_reaction_length: u64, commenter_tip_percentage: u64, repost_tip_percentage: u64, min_promotion_amount: u64, max_promotion_amount: u64, min_view_duration_ms: u64, platform_fee_bps: u64, ecosystem_fee_bps: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_update_post_parameters">update_post_parameters</a>(
    _admin_cap: &<a href="../social_contracts/post.md#social_contracts_post_PostAdminCap">PostAdminCap</a>,
    config: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_PostConfig">PostConfig</a>,
    max_content_length: u64,
    max_media_urls: u64,
    max_mentions: u64,
    max_metadata_size: u64,
    max_description_length: u64,
    max_reaction_length: u64,
    commenter_tip_percentage: u64,
    repost_tip_percentage: u64,
    min_promotion_amount: u64,
    max_promotion_amount: u64,
    min_view_duration_ms: u64,
    platform_fee_bps: u64,
    ecosystem_fee_bps: u64,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    // Validation
    <b>assert</b>!(commenter_tip_percentage &lt;= 100, <a href="../social_contracts/post.md#social_contracts_post_EInvalidConfig">EInvalidConfig</a>);
    <b>assert</b>!(repost_tip_percentage &lt;= 100, <a href="../social_contracts/post.md#social_contracts_post_EInvalidConfig">EInvalidConfig</a>);
    <b>assert</b>!(max_content_length &gt; 0, <a href="../social_contracts/post.md#social_contracts_post_EInvalidConfig">EInvalidConfig</a>);
    <b>assert</b>!(max_media_urls &gt; 0, <a href="../social_contracts/post.md#social_contracts_post_EInvalidConfig">EInvalidConfig</a>);
    <b>assert</b>!(max_mentions &gt; 0, <a href="../social_contracts/post.md#social_contracts_post_EInvalidConfig">EInvalidConfig</a>);
    <b>assert</b>!(min_promotion_amount &gt; 0, <a href="../social_contracts/post.md#social_contracts_post_EInvalidConfig">EInvalidConfig</a>);
    <b>assert</b>!(max_promotion_amount &gt;= min_promotion_amount, <a href="../social_contracts/post.md#social_contracts_post_EInvalidConfig">EInvalidConfig</a>);
    <b>assert</b>!(platform_fee_bps &lt;= <a href="../social_contracts/post.md#social_contracts_post_BPS_DENOM">BPS_DENOM</a>, <a href="../social_contracts/post.md#social_contracts_post_EInvalidConfig">EInvalidConfig</a>);
    <b>assert</b>!(ecosystem_fee_bps &lt;= <a href="../social_contracts/post.md#social_contracts_post_BPS_DENOM">BPS_DENOM</a>, <a href="../social_contracts/post.md#social_contracts_post_EInvalidConfig">EInvalidConfig</a>);
    <b>assert</b>!(platform_fee_bps + ecosystem_fee_bps &lt;= <a href="../social_contracts/post.md#social_contracts_post_BPS_DENOM">BPS_DENOM</a>, <a href="../social_contracts/post.md#social_contracts_post_EInvalidConfig">EInvalidConfig</a>);
    // Update config
    config.max_content_length = max_content_length;
    config.max_media_urls = max_media_urls;
    config.max_mentions = max_mentions;
    config.max_metadata_size = max_metadata_size;
    config.max_description_length = max_description_length;
    config.max_reaction_length = max_reaction_length;
    config.commenter_tip_percentage = commenter_tip_percentage;
    config.repost_tip_percentage = repost_tip_percentage;
    config.min_promotion_amount = min_promotion_amount;
    config.max_promotion_amount = max_promotion_amount;
    config.min_view_duration_ms = min_view_duration_ms;
    config.platform_fee_bps = platform_fee_bps;
    config.ecosystem_fee_bps = ecosystem_fee_bps;
    // Emit update event
    event::emit(<a href="../social_contracts/post.md#social_contracts_post_PostParametersUpdatedEvent">PostParametersUpdatedEvent</a> {
        updated_by: tx_context::sender(ctx),
        timestamp: clock::timestamp_ms(clock),
        max_content_length,
        max_media_urls,
        max_mentions,
        max_metadata_size,
        max_description_length,
        max_reaction_length,
        commenter_tip_percentage,
        repost_tip_percentage,
        min_promotion_amount,
        max_promotion_amount,
        min_view_duration_ms,
        platform_fee_bps,
        ecosystem_fee_bps,
    });
}
</code></pre>



</details>

<a name="social_contracts_post_create_promoted_post"></a>

## Function `create_promoted_post`

Create a promoted post with MYSO tokens for viewer payments


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_create_promoted_post">create_promoted_post</a>(registry: &<a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">social_contracts::profile::UsernameRegistry</a>, platform_registry: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">social_contracts::platform::PlatformRegistry</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, config: &<a href="../social_contracts/post.md#social_contracts_post_PostConfig">social_contracts::post::PostConfig</a>, memory_config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, content: <a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../social_contracts/post.md#social_contracts_post_media_asset_ids">media_asset_ids</a>: vector&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;, media_urls: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;&gt;, mentions: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;<b>address</b>&gt;&gt;, metadata_json: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, payment_per_view: u64, promotion_budget: <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, enable_spt: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;bool&gt;, enable_spot: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;bool&gt;, mydata_registry: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataRegistry">social_contracts::mydata::MyDataRegistry</a>, memory_account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_create_promoted_post">create_promoted_post</a>(
    registry: &UsernameRegistry,
    platform_registry: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">platform::PlatformRegistry</a>,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">platform::Platform</a>,
    block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">block_list::BlockListRegistry</a>,
    config: &<a href="../social_contracts/post.md#social_contracts_post_PostConfig">PostConfig</a>,
    memory_config: &MemoryConfig,
    content: String,
    <a href="../social_contracts/post.md#social_contracts_post_media_asset_ids">media_asset_ids</a>: vector&lt;ID&gt;,
    <b>mut</b> media_urls: Option&lt;vector&lt;String&gt;&gt;,
    mentions: Option&lt;vector&lt;<b>address</b>&gt;&gt;,
    metadata_json: Option&lt;String&gt;,
    payment_per_view: u64,
    promotion_budget: Coin&lt;MYSO&gt;,
    enable_spt: Option&lt;bool&gt;,
    enable_spot: Option&lt;bool&gt;,
    mydata_registry: &mydata::MyDataRegistry,
    memory_account: &MemoryAccount,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> access = PostAccess::Public;
    <b>let</b> acting = <a href="../social_contracts/post.md#social_contracts_post_resolve_social_actor">resolve_social_actor</a>(
        memory_config,
        registry,
        <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>,
        block_list_registry,
        memory_account,
        <a href="../social_contracts/memory.md#social_contracts_memory_cap_post_publish">memory::cap_post_publish</a>(),
        0,
        clock,
        ctx,
    );
    <b>let</b> owner = <a href="../social_contracts/memory.md#social_contracts_memory_acting_principal_owner">memory::acting_principal_owner</a>(&acting);
    <b>let</b> profile_id = <a href="../social_contracts/memory.md#social_contracts_memory_acting_profile_id">memory::acting_profile_id</a>(&acting);
    <b>let</b> <a href="../social_contracts/post.md#social_contracts_post_actor_address">actor_address</a> = <a href="../social_contracts/memory.md#social_contracts_memory_acting_actor_address">memory::acting_actor_address</a>(&acting);
    <b>let</b> <a href="../social_contracts/post.md#social_contracts_post_sub_agent_id">sub_agent_id</a> = <a href="../social_contracts/memory.md#social_contracts_memory_acting_sub_agent_id">memory::acting_sub_agent_id</a>(&acting);
    <b>let</b> organization_id = <a href="../social_contracts/memory.md#social_contracts_memory_acting_organization_id">memory::acting_organization_id</a>(&acting);
    <b>let</b> <a href="../social_contracts/post.md#social_contracts_post_action_identity_class">action_identity_class</a> = <a href="../social_contracts/memory.md#social_contracts_memory_acting_identity_class">memory::acting_identity_class</a>(&acting);
    // Validate promotion parameters
    <b>assert</b>!(payment_per_view &gt;= config.min_promotion_amount, <a href="../social_contracts/post.md#social_contracts_post_EPromotionAmountTooLow">EPromotionAmountTooLow</a>);
    <b>assert</b>!(payment_per_view &lt;= config.max_promotion_amount, <a href="../social_contracts/post.md#social_contracts_post_EPromotionAmountTooHigh">EPromotionAmountTooHigh</a>);
    <b>assert</b>!(coin::value(&promotion_budget) &gt;= payment_per_view, <a href="../social_contracts/post.md#social_contracts_post_EInsufficientPromotionFunds">EInsufficientPromotionFunds</a>);
    <a href="../social_contracts/post.md#social_contracts_post_assert_post_access_mydata_binding">assert_post_access_mydata_binding</a>(owner, &access, mydata_registry);
    // Check <b>if</b> <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> is approved
    <b>let</b> platform_id = object::uid_to_address(<a href="../social_contracts/platform.md#social_contracts_platform_id">platform::id</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>));
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform_is_approved">platform::is_approved</a>(platform_registry, platform_id), <a href="../social_contracts/post.md#social_contracts_post_EUnauthorized">EUnauthorized</a>);
    // Validate content length using config
    <b>assert</b>!(string::length(&content) &lt;= config.max_content_length, <a href="../social_contracts/post.md#social_contracts_post_EContentTooLarge">EContentTooLarge</a>);
    // Validate and convert media URLs <b>if</b> provided
    <b>let</b> media_option = <b>if</b> (option::is_some(&media_urls)) {
        <b>let</b> url_strings = option::extract(&<b>mut</b> media_urls);
        <b>assert</b>!(vector::length(&url_strings) &lt;= config.max_media_urls, <a href="../social_contracts/post.md#social_contracts_post_ETooManyMediaUrls">ETooManyMediaUrls</a>);
        <b>let</b> <b>mut</b> urls = vector::empty&lt;Url&gt;();
        <b>let</b> <b>mut</b> i = 0;
        <b>while</b> (i &lt; vector::length(&url_strings)) {
            <b>let</b> url_string = vector::borrow(&url_strings, i);
            <b>let</b> url_bytes = string::as_bytes(url_string);
            <b>let</b> url = url::new_unsafe_from_bytes(*url_bytes);
            vector::push_back(&<b>mut</b> urls, url);
            i = i + 1;
        };
        option::some(urls)
    } <b>else</b> {
        option::none()
    };
    // Validate mentions <b>if</b> provided using config
    <b>if</b> (option::is_some(&mentions)) {
        <b>let</b> mentions_ref = option::borrow(&mentions);
        <b>assert</b>!(vector::length(mentions_ref) &lt;= config.max_mentions, <a href="../social_contracts/post.md#social_contracts_post_EContentTooLarge">EContentTooLarge</a>);
    };
    // Validate metadata <b>if</b> provided using config
    <b>if</b> (option::is_some(&metadata_json)) {
        <b>let</b> metadata_string = option::borrow(&metadata_json);
        <b>assert</b>!(string::length(metadata_string) &lt;= config.max_metadata_size, <a href="../social_contracts/post.md#social_contracts_post_EContentTooLarge">EContentTooLarge</a>);
    };
    // Create promotion data (starts <b>as</b> inactive until <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> activates it)
    <b>let</b> <b>mut</b> promotion_data = <a href="../social_contracts/post.md#social_contracts_post_PromotionData">PromotionData</a> {
        id: object::new(ctx),
        post_id: @0x0, // Will be set after <a href="../social_contracts/post.md#social_contracts_post">post</a> creation
        payment_per_view,
        promotion_budget: coin::into_balance(promotion_budget),
        paid_viewers: table::new(ctx),
        views: vector::empty(),
        active: <b>false</b>, // Starts inactive until <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> approves
        created_at: clock::timestamp_ms(clock),
    };
    <b>let</b> promotion_id = object::uid_to_address(&promotion_data.id);
    // Set defaults <b>for</b> feature flags (default to opt-out - users must explicitly opt-in)
    <b>let</b> final_enable_spt = <b>if</b> (option::is_some(&enable_spt)) {
        *option::borrow(&enable_spt)
    } <b>else</b> {
        <b>false</b> // Default to opt-out (user must explicitly opt-in)
    };
    <b>assert</b>!(!final_enable_spt, <a href="../social_contracts/post.md#social_contracts_post_ESptRequiresDedicatedCreate">ESptRequiresDedicatedCreate</a>);
    // enable_spot retained <b>for</b> <b>entry</b>-signature compatibility; SPoT is always-on.
    <b>let</b> _ = enable_spot;
    // Convert media URLs to strings <b>for</b> <a href="../social_contracts/post.md#social_contracts_post_PostCreatedEvent">PostCreatedEvent</a> (before moving media_option)
    <b>let</b> media_urls_for_event = <a href="../social_contracts/post.md#social_contracts_post_convert_urls_to_strings">convert_urls_to_strings</a>(&media_option);
    <b>let</b> media_asset_ids_for_event = <a href="../social_contracts/post.md#social_contracts_post_media_asset_ids">media_asset_ids</a>;
    <b>let</b> <a href="../social_contracts/post.md#social_contracts_post">post</a> = <a href="../social_contracts/post.md#social_contracts_post_create_post_internal">create_post_internal</a>(
        owner,
        profile_id,
        platform_id,
        content,
        <a href="../social_contracts/post.md#social_contracts_post_media_asset_ids">media_asset_ids</a>,
        media_option,
        mentions,
        metadata_json,
        string::utf8(<a href="../social_contracts/post.md#social_contracts_post_POST_TYPE_STANDARD">POST_TYPE_STANDARD</a>),
        option::none(),
        <b>true</b>, // <a href="../social_contracts/post.md#social_contracts_post_allow_comments">allow_comments</a>
        <b>true</b>, // <a href="../social_contracts/post.md#social_contracts_post_allow_reactions">allow_reactions</a>
        <b>true</b>, // <a href="../social_contracts/post.md#social_contracts_post_allow_reposts">allow_reposts</a>
        <b>true</b>, // <a href="../social_contracts/post.md#social_contracts_post_allow_quotes">allow_quotes</a>
        <b>true</b>, // <a href="../social_contracts/post.md#social_contracts_post_allow_tips">allow_tips</a>
        access,
        option::some(promotion_id),
        <b>false</b>,
        <a href="../social_contracts/post.md#social_contracts_post_actor_address">actor_address</a>,
        <a href="../social_contracts/post.md#social_contracts_post_sub_agent_id">sub_agent_id</a>,
        organization_id,
        <a href="../social_contracts/post.md#social_contracts_post_action_identity_class">action_identity_class</a>,
        clock,
        ctx
    );
    // Indexers read <a href="../social_contracts/post.md#social_contracts_post_PostCreatedEvent">PostCreatedEvent</a> to upsert `posts` with promotion_id before <a href="../social_contracts/post.md#social_contracts_post_PromotedPostCreatedEvent">PromotedPostCreatedEvent</a>
    <a href="../social_contracts/post.md#social_contracts_post_emit_post_media_asset_usages">emit_post_media_asset_usages</a>(&<a href="../social_contracts/post.md#social_contracts_post">post</a>, clock);
    <b>let</b> post_id = <a href="../social_contracts/post.md#social_contracts_post_share_post">share_post</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>);
    event::emit(<a href="../social_contracts/post.md#social_contracts_post_PostCreatedEvent">PostCreatedEvent</a> {
        post_id,
        owner,
        profile_id,
        platform_id,
        permissions: <a href="../social_contracts/post.md#social_contracts_post_permissions_bitfield">permissions_bitfield</a>(<b>true</b>, <b>true</b>, <b>true</b>, <b>true</b>, <b>true</b>),
        content,
        post_type: string::utf8(<a href="../social_contracts/post.md#social_contracts_post_POST_TYPE_STANDARD">POST_TYPE_STANDARD</a>),
        parent_post_id: option::none(),
        mentions,
        <a href="../social_contracts/post.md#social_contracts_post_media_asset_ids">media_asset_ids</a>: media_asset_ids_for_event,
        media_urls: media_urls_for_event,
        metadata_json,
        access,
        promotion_id: option::some(promotion_id),
        <a href="../social_contracts/post.md#social_contracts_post_composition_status">composition_status</a>: <a href="../social_contracts/media_asset.md#social_contracts_media_asset_composition_none">media_asset::composition_none</a>(),
        <a href="../social_contracts/post.md#social_contracts_post_monetization_status">monetization_status</a>: <a href="../social_contracts/media_asset.md#social_contracts_media_asset_monetization_none">media_asset::monetization_none</a>(),
        enable_spt: <b>false</b>,
        spt_id: option::none(),
        <a href="../social_contracts/post.md#social_contracts_post_actor_address">actor_address</a>,
        <a href="../social_contracts/post.md#social_contracts_post_sub_agent_id">sub_agent_id</a>,
        organization_id,
        <a href="../social_contracts/post.md#social_contracts_post_action_identity_class">action_identity_class</a>,
    });
    // Update promotion data with <a href="../social_contracts/post.md#social_contracts_post">post</a> ID
    promotion_data.post_id = post_id;
    // Get budget value before moving the promotion_data
    <b>let</b> total_budget = balance::value(&promotion_data.promotion_budget);
    // Share promotion data
    transfer::share_object(promotion_data);
    // Emit promoted <a href="../social_contracts/post.md#social_contracts_post">post</a> creation event
    event::emit(<a href="../social_contracts/post.md#social_contracts_post_PromotedPostCreatedEvent">PromotedPostCreatedEvent</a> {
        post_id,
        owner,
        profile_id,
        payment_per_view,
        total_budget,
        created_at: clock::timestamp_ms(clock),
    });
}
</code></pre>



</details>

<a name="social_contracts_post_confirm_promoted_post_views"></a>

## Function `confirm_promoted_post_views`

Confirm that one viewer was paid for N (≥1) promoted post views in a single transaction.
Takes promotions by value (MakeMoveVec), merges nets into one transfer, then re-shares each
<code><a href="../social_contracts/post.md#social_contracts_post_PromotionData">PromotionData</a></code>. Fees come out of each view's <code>payment_per_view</code> gross.


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_confirm_promoted_post_views">confirm_promoted_post_views</a>(promotions: vector&lt;<a href="../social_contracts/post.md#social_contracts_post_PromotionData">social_contracts::post::PromotionData</a>&gt;, view_durations: vector&lt;u64&gt;, config: &<a href="../social_contracts/post.md#social_contracts_post_PostConfig">social_contracts::post::PostConfig</a>, platform_obj: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, group: &<a href="../myso/permissioned_group.md#myso_permissioned_group_PermissionedGroup">myso::permissioned_group::PermissionedGroup</a>&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformPackage">social_contracts::platform::PlatformPackage</a>&gt;, treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, viewer_address: <b>address</b>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_confirm_promoted_post_views">confirm_promoted_post_views</a>(
    <b>mut</b> promotions: vector&lt;<a href="../social_contracts/post.md#social_contracts_post_PromotionData">PromotionData</a>&gt;,
    view_durations: vector&lt;u64&gt;,
    config: &<a href="../social_contracts/post.md#social_contracts_post_PostConfig">PostConfig</a>,
    platform_obj: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">platform::Platform</a>,
    group: &PermissionedGroup&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformPackage">platform::PlatformPackage</a>&gt;,
    treasury: &EcosystemTreasury,
    viewer_address: <b>address</b>,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> caller = tx_context::sender(ctx);
    <b>assert</b>!(
        <a href="../social_contracts/platform.md#social_contracts_platform_has_moderator_permission">platform::has_moderator_permission</a>&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformPromotionAdmin">platform::PlatformPromotionAdmin</a>&gt;(group, platform_obj, caller),
        <a href="../social_contracts/post.md#social_contracts_post_EUnauthorized">EUnauthorized</a>,
    );
    <b>let</b> n = vector::length(&promotions);
    <b>assert</b>!(n &gt; 0, <a href="../social_contracts/post.md#social_contracts_post_EInvalidBatch">EInvalidBatch</a>);
    <b>assert</b>!(n == vector::length(&view_durations), <a href="../social_contracts/post.md#social_contracts_post_EInvalidBatch">EInvalidBatch</a>);
    <b>assert</b>!(n &lt;= <a href="../social_contracts/post.md#social_contracts_post_MAX_PROMOTION_VIEW_BATCH">MAX_PROMOTION_VIEW_BATCH</a>, <a href="../social_contracts/post.md#social_contracts_post_EInvalidBatch">EInvalidBatch</a>);
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform_has_joined_platform">platform::has_joined_platform</a>(platform_obj, viewer_address), <a href="../social_contracts/post.md#social_contracts_post_EUserNotJoinedPlatform">EUserNotJoinedPlatform</a>);
    <b>assert</b>!(viewer_address != caller, <a href="../social_contracts/post.md#social_contracts_post_EUnauthorized">EUnauthorized</a>);
    <b>let</b> platform_id = object::uid_to_address(<a href="../social_contracts/platform.md#social_contracts_platform_id">platform::id</a>(platform_obj));
    <b>let</b> timestamp = clock::timestamp_ms(clock);
    <b>let</b> <b>mut</b> items = vector::empty&lt;<a href="../social_contracts/post.md#social_contracts_post_PromotedViewConfirmItem">PromotedViewConfirmItem</a>&gt;();
    <b>let</b> <b>mut</b> total_payment_amount = 0u64;
    <b>let</b> <b>mut</b> total_platform_fee = 0u64;
    <b>let</b> <b>mut</b> total_ecosystem_fee = 0u64;
    <b>let</b> <b>mut</b> total_recipient_amount = 0u64;
    <b>let</b> <b>mut</b> merged_payment: Option&lt;Coin&lt;MYSO&gt;&gt; = option::none();
    <b>let</b> <b>mut</b> i = 0u64;
    <b>while</b> (i &lt; n) {
        <b>let</b> promotion_data = vector::borrow_mut(&<b>mut</b> promotions, i);
        <b>let</b> view_duration = *vector::borrow(&view_durations, i);
        <b>assert</b>!(promotion_data.active, <a href="../social_contracts/post.md#social_contracts_post_EPromotionInactive">EPromotionInactive</a>);
        <b>assert</b>!(view_duration &gt;= config.min_view_duration_ms, <a href="../social_contracts/post.md#social_contracts_post_EInvalidViewDuration">EInvalidViewDuration</a>);
        <b>assert</b>!(!table::contains(&promotion_data.paid_viewers, viewer_address), <a href="../social_contracts/post.md#social_contracts_post_EUserAlreadyViewed">EUserAlreadyViewed</a>);
        <b>assert</b>!(
            balance::value(&promotion_data.promotion_budget) &gt;= promotion_data.payment_per_view,
            <a href="../social_contracts/post.md#social_contracts_post_EInsufficientPromotionFunds">EInsufficientPromotionFunds</a>,
        );
        <b>let</b> promotion_id = object::uid_to_address(&promotion_data.id);
        <b>let</b> post_id = promotion_data.post_id;
        vector::push_back(&<b>mut</b> promotion_data.views, <a href="../social_contracts/post.md#social_contracts_post_PromotionView">PromotionView</a> {
            viewer: viewer_address,
            view_duration,
            view_timestamp: timestamp,
            platform_id,
        });
        table::add(&<b>mut</b> promotion_data.paid_viewers, viewer_address, <b>true</b>);
        <b>let</b> gross = promotion_data.payment_per_view;
        <b>let</b> platform_fee = (gross * config.platform_fee_bps) / <a href="../social_contracts/post.md#social_contracts_post_BPS_DENOM">BPS_DENOM</a>;
        <b>let</b> ecosystem_fee = (gross * config.ecosystem_fee_bps) / <a href="../social_contracts/post.md#social_contracts_post_BPS_DENOM">BPS_DENOM</a>;
        <b>let</b> recipient_amount = gross - platform_fee - ecosystem_fee;
        <b>assert</b>!(total_payment_amount &lt;= <a href="../social_contracts/post.md#social_contracts_post_MAX_U64">MAX_U64</a> - gross, <a href="../social_contracts/post.md#social_contracts_post_EOverflow">EOverflow</a>);
        <b>assert</b>!(total_platform_fee &lt;= <a href="../social_contracts/post.md#social_contracts_post_MAX_U64">MAX_U64</a> - platform_fee, <a href="../social_contracts/post.md#social_contracts_post_EOverflow">EOverflow</a>);
        <b>assert</b>!(total_ecosystem_fee &lt;= <a href="../social_contracts/post.md#social_contracts_post_MAX_U64">MAX_U64</a> - ecosystem_fee, <a href="../social_contracts/post.md#social_contracts_post_EOverflow">EOverflow</a>);
        <b>assert</b>!(total_recipient_amount &lt;= <a href="../social_contracts/post.md#social_contracts_post_MAX_U64">MAX_U64</a> - recipient_amount, <a href="../social_contracts/post.md#social_contracts_post_EOverflow">EOverflow</a>);
        total_payment_amount = total_payment_amount + gross;
        total_platform_fee = total_platform_fee + platform_fee;
        total_ecosystem_fee = total_ecosystem_fee + ecosystem_fee;
        total_recipient_amount = total_recipient_amount + recipient_amount;
        <b>let</b> payment = coin::from_balance(
            balance::split(&<b>mut</b> promotion_data.promotion_budget, gross),
            ctx,
        );
        <b>if</b> (option::is_none(&merged_payment)) {
            option::fill(&<b>mut</b> merged_payment, payment);
        } <b>else</b> {
            coin::join(option::borrow_mut(&<b>mut</b> merged_payment), payment);
        };
        <b>if</b> (balance::value(&promotion_data.promotion_budget) &lt; promotion_data.payment_per_view) {
            promotion_data.active = <b>false</b>;
        };
        vector::push_back(&<b>mut</b> items, <a href="../social_contracts/post.md#social_contracts_post_PromotedViewConfirmItem">PromotedViewConfirmItem</a> {
            post_id,
            promotion_id,
            payment_amount: gross,
            platform_fee,
            ecosystem_fee,
            recipient_amount,
            view_duration,
        });
        i = i + 1;
    };
    <b>let</b> <b>mut</b> payment = option::extract(&<b>mut</b> merged_payment);
    option::destroy_none(merged_payment);
    <b>if</b> (total_ecosystem_fee &gt; 0) {
        <b>let</b> eco_coin = coin::split(&<b>mut</b> payment, total_ecosystem_fee, ctx);
        transfer::public_transfer(eco_coin, <a href="../social_contracts/profile.md#social_contracts_profile_get_treasury_address">profile::get_treasury_address</a>(treasury));
    };
    <b>if</b> (total_platform_fee &gt; 0) {
        <b>let</b> <b>mut</b> platform_coin = coin::split(&<b>mut</b> payment, total_platform_fee, ctx);
        <a href="../social_contracts/platform.md#social_contracts_platform_add_to_treasury">platform::add_to_treasury</a>(platform_obj, &<b>mut</b> platform_coin, total_platform_fee, clock, ctx);
        coin::destroy_zero(platform_coin);
    };
    transfer::public_transfer(payment, viewer_address);
    event::emit(<a href="../social_contracts/post.md#social_contracts_post_PromotedPostViewsBatchConfirmedEvent">PromotedPostViewsBatchConfirmedEvent</a> {
        viewer: viewer_address,
        platform_id,
        timestamp,
        items,
        total_payment_amount,
        total_platform_fee,
        total_ecosystem_fee,
        total_recipient_amount,
    });
    <b>while</b> (!vector::is_empty(&promotions)) {
        <b>let</b> promotion_data = vector::pop_back(&<b>mut</b> promotions);
        transfer::share_object(promotion_data);
    };
    vector::destroy_empty(promotions);
}
</code></pre>



</details>

<a name="social_contracts_post_toggle_promotion_status"></a>

## Function `toggle_promotion_status`

Toggle promotion status (platform can activate, both platform and owner can deactivate)
Use with activate: false to deactivate promotions


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_toggle_promotion_status">toggle_promotion_status</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, promotion_data: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_PromotionData">social_contracts::post::PromotionData</a>, platform_obj: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, group: &<a href="../myso/permissioned_group.md#myso_permissioned_group_PermissionedGroup">myso::permissioned_group::PermissionedGroup</a>&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformPackage">social_contracts::platform::PlatformPackage</a>&gt;, activate: bool, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_toggle_promotion_status">toggle_promotion_status</a>(
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>,
    promotion_data: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_PromotionData">PromotionData</a>,
    platform_obj: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">platform::Platform</a>,
    group: &PermissionedGroup&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformPackage">platform::PlatformPackage</a>&gt;,
    activate: bool,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> caller = tx_context::sender(ctx);
    // Verify the <a href="../social_contracts/post.md#social_contracts_post">post</a> is promoted
    <b>assert</b>!(option::is_some(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.promotion_id), <a href="../social_contracts/post.md#social_contracts_post_ENotPromotedPost">ENotPromotedPost</a>);
    <b>let</b> post_promotion_id = *option::borrow(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.promotion_id);
    <b>assert</b>!(post_promotion_id == object::uid_to_address(&promotion_data.id), <a href="../social_contracts/post.md#social_contracts_post_ENotPromotedPost">ENotPromotedPost</a>);
    <b>if</b> (activate) {
        <b>assert</b>!(
            <a href="../social_contracts/platform.md#social_contracts_platform_has_moderator_permission">platform::has_moderator_permission</a>&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformPromotionAdmin">platform::PlatformPromotionAdmin</a>&gt;(group, platform_obj, caller),
            <a href="../social_contracts/post.md#social_contracts_post_EUnauthorized">EUnauthorized</a>,
        );
    } <b>else</b> {
        <b>let</b> is_platform = <a href="../social_contracts/platform.md#social_contracts_platform_is_moderator">platform::is_moderator</a>(group, platform_obj, caller);
        <b>let</b> is_owner = caller == <a href="../social_contracts/post.md#social_contracts_post">post</a>.owner;
        <b>assert</b>!(is_platform || is_owner, <a href="../social_contracts/post.md#social_contracts_post_EUnauthorized">EUnauthorized</a>);
    };
    promotion_data.active = activate;
    // Emit status change event
    event::emit(<a href="../social_contracts/post.md#social_contracts_post_PromotionStatusToggledEvent">PromotionStatusToggledEvent</a> {
        post_id: post_promotion_id,
        toggled_by: caller,
        new_status: activate,
        timestamp: clock::timestamp_ms(clock),
    });
}
</code></pre>



</details>

<a name="social_contracts_post_withdraw_promotion_funds"></a>

## Function `withdraw_promotion_funds`

Withdraw all MYSO tokens from promotion (owner only, deactivates promotion)


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_withdraw_promotion_funds">withdraw_promotion_funds</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, promotion_data: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_PromotionData">social_contracts::post::PromotionData</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_withdraw_promotion_funds">withdraw_promotion_funds</a>(
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>,
    promotion_data: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_PromotionData">PromotionData</a>,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> caller = tx_context::sender(ctx);
    // Verify caller is <a href="../social_contracts/post.md#social_contracts_post">post</a> owner
    <b>assert</b>!(caller == <a href="../social_contracts/post.md#social_contracts_post">post</a>.owner, <a href="../social_contracts/post.md#social_contracts_post_EUnauthorized">EUnauthorized</a>);
    // Verify the <a href="../social_contracts/post.md#social_contracts_post">post</a> is promoted
    <b>assert</b>!(option::is_some(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.promotion_id), <a href="../social_contracts/post.md#social_contracts_post_ENotPromotedPost">ENotPromotedPost</a>);
    <b>let</b> post_promotion_id = *option::borrow(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.promotion_id);
    <b>assert</b>!(post_promotion_id == object::uid_to_address(&promotion_data.id), <a href="../social_contracts/post.md#social_contracts_post_ENotPromotedPost">ENotPromotedPost</a>);
    // Get remaining funds
    <b>let</b> remaining_amount = balance::value(&promotion_data.promotion_budget);
    // Extract all remaining balance and transfer to owner
    <b>let</b> withdrawn_balance = balance::withdraw_all(&<b>mut</b> promotion_data.promotion_budget);
    <b>let</b> withdrawn_coins = coin::from_balance(withdrawn_balance, ctx);
    transfer::public_transfer(withdrawn_coins, caller);
    // Deactivate promotion
    promotion_data.active = <b>false</b>;
    // Emit withdrawal event
    event::emit(<a href="../social_contracts/post.md#social_contracts_post_PromotionFundsWithdrawnEvent">PromotionFundsWithdrawnEvent</a> {
        post_id: post_promotion_id,
        owner: caller,
        withdrawn_amount: remaining_amount,
        timestamp: clock::timestamp_ms(clock),
    });
}
</code></pre>



</details>

<a name="social_contracts_post_get_promotion_stats"></a>

## Function `get_promotion_stats`

Get promotion statistics for a post


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_get_promotion_stats">get_promotion_stats</a>(promotion_data: &<a href="../social_contracts/post.md#social_contracts_post_PromotionData">social_contracts::post::PromotionData</a>): (u64, u64, bool, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_get_promotion_stats">get_promotion_stats</a>(promotion_data: &<a href="../social_contracts/post.md#social_contracts_post_PromotionData">PromotionData</a>): (u64, u64, bool, u64) {
    (
        promotion_data.payment_per_view,
        balance::value(&promotion_data.promotion_budget),
        promotion_data.active,
        vector::length(&promotion_data.views)
    )
}
</code></pre>



</details>

<a name="social_contracts_post_has_user_viewed_promoted_post"></a>

## Function `has_user_viewed_promoted_post`

Check if a user has already been paid for viewing a promoted post


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_has_user_viewed_promoted_post">has_user_viewed_promoted_post</a>(promotion_data: &<a href="../social_contracts/post.md#social_contracts_post_PromotionData">social_contracts::post::PromotionData</a>, user: <b>address</b>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_has_user_viewed_promoted_post">has_user_viewed_promoted_post</a>(promotion_data: &<a href="../social_contracts/post.md#social_contracts_post_PromotionData">PromotionData</a>, user: <b>address</b>): bool {
    table::contains(&promotion_data.paid_viewers, user)
}
</code></pre>



</details>

<a name="social_contracts_post_get_promotion_id"></a>

## Function `get_promotion_id`

Get the promotion ID from a post


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_get_promotion_id">get_promotion_id</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_get_promotion_id">get_promotion_id</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>): Option&lt;<b>address</b>&gt; {
    <a href="../social_contracts/post.md#social_contracts_post">post</a>.promotion_id
}
</code></pre>



</details>

<a name="social_contracts_post_set_moderation_status"></a>

## Function `set_moderation_status`

Set moderation status for a post (platform devs/mods only)


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_set_moderation_status">set_moderation_status</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, group: &<a href="../myso/permissioned_group.md#myso_permissioned_group_PermissionedGroup">myso::permissioned_group::PermissionedGroup</a>&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformPackage">social_contracts::platform::PlatformPackage</a>&gt;, platform_registry: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">social_contracts::platform::PlatformRegistry</a>, status: u8, reason: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_set_moderation_status">set_moderation_status</a>(
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">platform::Platform</a>,
    group: &PermissionedGroup&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformPackage">platform::PlatformPackage</a>&gt;,
    platform_registry: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">platform::PlatformRegistry</a>,
    status: u8, // <a href="../social_contracts/post.md#social_contracts_post_MODERATION_APPROVED">MODERATION_APPROVED</a> or <a href="../social_contracts/post.md#social_contracts_post_MODERATION_FLAGGED">MODERATION_FLAGGED</a>
    reason: Option&lt;String&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    // Check <a href="../social_contracts/post.md#social_contracts_post_version">version</a> compatibility
    <b>assert</b>!(<a href="../social_contracts/post.md#social_contracts_post">post</a>.<a href="../social_contracts/post.md#social_contracts_post_version">version</a> == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/post.md#social_contracts_post_EWrongVersion">EWrongVersion</a>);
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform_platform_version">platform::platform_version</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>) == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/post.md#social_contracts_post_EWrongVersion">EWrongVersion</a>);
    <b>let</b> caller = tx_context::sender(ctx);
    <b>assert</b>!(
        <a href="../social_contracts/platform.md#social_contracts_platform_has_moderator_permission">platform::has_moderator_permission</a>&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformContentModerator">platform::PlatformContentModerator</a>&gt;(group, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, caller),
        <a href="../social_contracts/post.md#social_contracts_post_EUnauthorized">EUnauthorized</a>,
    );
    // Verify <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> is approved
    <b>let</b> platform_id = object::uid_to_address(<a href="../social_contracts/platform.md#social_contracts_platform_id">platform::id</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>));
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform_is_approved">platform::is_approved</a>(platform_registry, platform_id), <a href="../social_contracts/post.md#social_contracts_post_EUnauthorized">EUnauthorized</a>);
    // Validate status
    <b>assert</b>!(status == <a href="../social_contracts/post.md#social_contracts_post_MODERATION_APPROVED">MODERATION_APPROVED</a> || status == <a href="../social_contracts/post.md#social_contracts_post_MODERATION_FLAGGED">MODERATION_FLAGGED</a>, <a href="../social_contracts/post.md#social_contracts_post_EUnauthorized">EUnauthorized</a>);
    // Update <a href="../social_contracts/post.md#social_contracts_post">post</a> status based on moderation decision
    <b>if</b> (status == <a href="../social_contracts/post.md#social_contracts_post_MODERATION_FLAGGED">MODERATION_FLAGGED</a>) {
        <a href="../social_contracts/post.md#social_contracts_post">post</a>.removed_from_platform = <b>true</b>;
    } <b>else</b> {
        <a href="../social_contracts/post.md#social_contracts_post">post</a>.removed_from_platform = <b>false</b>;
    };
    // Create or update moderation record
    <b>let</b> moderation_record = <a href="../social_contracts/post.md#social_contracts_post_ModerationRecord">ModerationRecord</a> {
        id: object::new(ctx),
        post_id: object::uid_to_address(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id),
        platform_id: object::uid_to_address(<a href="../social_contracts/platform.md#social_contracts_platform_id">platform::id</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>)),
        moderation_state: status,
        moderator: option::some(caller),
        moderation_timestamp: option::some(clock::timestamp_ms(clock)),
        reason,
    };
    transfer::share_object(moderation_record);
    // Emit moderation event
    event::emit(<a href="../social_contracts/post.md#social_contracts_post_PostModerationEvent">PostModerationEvent</a> {
        post_id: object::uid_to_address(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id),
        platform_id: object::uid_to_address(<a href="../social_contracts/platform.md#social_contracts_platform_id">platform::id</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>)),
        removed: (status == <a href="../social_contracts/post.md#social_contracts_post_MODERATION_FLAGGED">MODERATION_FLAGGED</a>),
        moderated_by: caller,
    });
}
</code></pre>



</details>

<a name="social_contracts_post_is_content_approved"></a>

## Function `is_content_approved`

Check if content is approved (not flagged)


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_is_content_approved">is_content_approved</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_is_content_approved">is_content_approved</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>): bool {
    !<a href="../social_contracts/post.md#social_contracts_post">post</a>.removed_from_platform
}
</code></pre>



</details>

<a name="social_contracts_post_assert_can_view_post"></a>

## Function `assert_can_view_post`

Abort unless caller is post owner or holds a valid profile subscription for the gated service.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_assert_can_view_post">assert_can_view_post</a>(block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, service: &<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">social_contracts::subscription::ProfileSubscriptionService</a>, <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>: &<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscription">social_contracts::subscription::ProfileSubscription</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_assert_can_view_post">assert_can_view_post</a>(
    block_list_registry: &BlockListRegistry,
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>,
    service: &ProfileSubscriptionService,
    <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>: &ProfileSubscription,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> sender = tx_context::sender(ctx);
    <b>if</b> (sender == <a href="../social_contracts/post.md#social_contracts_post">post</a>.owner) {
        <b>return</b>
    };
    <a href="../social_contracts/block_list.md#social_contracts_block_list_assert_not_blocked">block_list::assert_not_blocked</a>(block_list_registry, sender, <a href="../social_contracts/post.md#social_contracts_post">post</a>.owner);
    <b>assert</b>!(<a href="../social_contracts/post.md#social_contracts_post_requires_profile_subscription">requires_profile_subscription</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>), <a href="../social_contracts/post.md#social_contracts_post_ENoSubscriptionService">ENoSubscriptionService</a>);
    <b>let</b> service_id = <a href="../social_contracts/post.md#social_contracts_post_subscription_service">subscription_service</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>);
    <b>assert</b>!(option::is_some(&service_id), <a href="../social_contracts/post.md#social_contracts_post_ENoSubscriptionService">ENoSubscriptionService</a>);
    <b>assert</b>!(*option::borrow(&service_id) == object::id(service), <a href="../social_contracts/post.md#social_contracts_post_ENoSubscriptionService">ENoSubscriptionService</a>);
    <b>let</b> min_tier = <a href="../social_contracts/post.md#social_contracts_post_subscription_min_tier_level">subscription_min_tier_level</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>);
    <b>let</b> content_platform_id = option::some(<a href="../social_contracts/post.md#social_contracts_post">post</a>.platform_id);
    <b>assert</b>!(
        <a href="../social_contracts/subscription.md#social_contracts_subscription_subscription_satisfies_access">subscription::subscription_satisfies_access</a>(
            <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>,
            service,
            sender,
            min_tier,
            content_platform_id,
            clock,
        ),
        <a href="../social_contracts/post.md#social_contracts_post_EUnauthorized">EUnauthorized</a>,
    );
}
</code></pre>



</details>

<a name="social_contracts_post_record_post_subscription_view"></a>

## Function `record_post_subscription_view`

Record a subscriber view after access check; emits <code><a href="../social_contracts/post.md#social_contracts_post_PostSubscriptionAccessEvent">PostSubscriptionAccessEvent</a></code>.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_record_post_subscription_view">record_post_subscription_view</a>(block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, service: &<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">social_contracts::subscription::ProfileSubscriptionService</a>, <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>: &<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscription">social_contracts::subscription::ProfileSubscription</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_record_post_subscription_view">record_post_subscription_view</a>(
    block_list_registry: &BlockListRegistry,
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>,
    service: &ProfileSubscriptionService,
    <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>: &ProfileSubscription,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <a href="../social_contracts/post.md#social_contracts_post_assert_can_view_post">assert_can_view_post</a>(block_list_registry, <a href="../social_contracts/post.md#social_contracts_post">post</a>, service, <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>, clock, ctx);
    event::emit(<a href="../social_contracts/post.md#social_contracts_post_PostSubscriptionAccessEvent">PostSubscriptionAccessEvent</a> {
        post_id: object::id(<a href="../social_contracts/post.md#social_contracts_post">post</a>),
        service_id: object::id(service),
        subscription_id: object::id(<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>),
        subscriber: tx_context::sender(ctx),
        timestamp: clock::timestamp_ms(clock),
    });
}
</code></pre>



</details>

<a name="social_contracts_post_create_post_admin_cap"></a>

## Function `create_post_admin_cap`

Create a PostAdminCap for bootstrap (package visibility only)
This function is only callable by other modules in the same package


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_create_post_admin_cap">create_post_admin_cap</a>(ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../social_contracts/post.md#social_contracts_post_PostAdminCap">social_contracts::post::PostAdminCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_create_post_admin_cap">create_post_admin_cap</a>(ctx: &<b>mut</b> TxContext): <a href="../social_contracts/post.md#social_contracts_post_PostAdminCap">PostAdminCap</a> {
    <a href="../social_contracts/post.md#social_contracts_post_PostAdminCap">PostAdminCap</a> {
        id: object::new(ctx)
    }
}
</code></pre>



</details>
