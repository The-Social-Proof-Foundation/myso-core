// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

/// Proof of Creativity module for the MySocial network
/// Manages content originality verification through oracle analysis,
/// PoC badge issuance, revenue redirection, and community dispute voting

#[allow(duplicate_alias, unused_use, unused_const, unused_variable, lint(public_entry))]
module social_contracts::proof_of_creativity {
    use std::string::{Self, String};
    use std::option::{Self, Option};
    use std::vector;

    use myso::{
        object::{Self, UID, ID},
        tx_context::{Self, TxContext},
        clock::{Self, Clock},
        transfer,
        event,
        table::{Self, Table},
        coin::{Self, Coin},
        balance::{Self, Balance}
    };
    use myso::myso::MYSO;
    use social_contracts::upgrade::{Self, UpgradeAdminCap};
    use social_contracts::profile::{Self, EcosystemTreasury};
    use social_contracts::poc_vault::{Self as poc_vault, PoCBeneficiaryVault, PoCVaultDirectory};

    /// Error codes
    const EUnauthorized: u64 = 0;
    const EInvalidThreshold: u64 = 2;
    const EPostNotFound: u64 = 3;
    const EInvalidMediaType: u64 = 7;
    const EInsufficientFunds: u64 = 9;
    const EWrongVersion: u64 = 11;
    const ENotOracle: u64 = 12;
    const EInvalidStakeAmount: u64 = 14;
    const EVotingNotActive: u64 = 15;
    const EVotingEnded: u64 = 16;
    const EAlreadyVoted: u64 = 17;
    const ENoVotesToResolve: u64 = 18;
    const EInvalidReasoning: u64 = 19;
    const EInvalidEvidenceUrls: u64 = 20;
    const EDisabled: u64 = 21;
    const ETooManyVotes: u64 = 22;
    const EDuplicateVoteRewardClaim: u64 = 23;
    const ENoTokenPoolForPost: u64 = 24;
    const EDisputeCapReached: u64 = 25;

    /// `derivative_redirection_target` in analyze_and_update_post (similarity path only)
    const DERIVATIVE_TARGET_WALLET: u8 = 0;
    const DERIVATIVE_TARGET_ESCROW: u8 = 1;

    /// Aligns with post::POC_OUTCOME_*
    const OUTCOME_ORIGINAL: u8 = 1;
    const OUTCOME_DERIVATIVE_WALLET: u8 = 2;
    const OUTCOME_DERIVATIVE_ESCROW: u8 = 3;
    const OUTCOME_ROYALTY_FREE: u8 = 4;

    /// Aligns with post::POC_REDIRECT_* (treasury redirect removed — fees route via beneficiary vault + claim-time treasury bps)
    const REDIRECT_WALLET: u8 = 1;
    const REDIRECT_ESCROW: u8 = 2;
    const MEDIA_TYPE_IMAGE: u8 = 1;
    const MEDIA_TYPE_VIDEO: u8 = 2;
    const MEDIA_TYPE_AUDIO: u8 = 3;

    /// Dispute status constants
    const DISPUTE_STATUS_VOTING: u8 = 1;
    const DISPUTE_STATUS_RESOLVED_UPHELD: u8 = 2;  // Badge keeper wins
    const DISPUTE_STATUS_RESOLVED_OVERTURNED: u8 = 3;  // Challenger wins

    /// Vote option constants
    const VOTE_UPHOLD: u8 = 1;  // Keep original PoC decision
    const VOTE_OVERTURN: u8 = 2; // Overturn original PoC decision

    /// Configuration constants (default values)
    const DEFAULT_IMAGE_THRESHOLD: u64 = 95; // 0.95 as percentage (95/100)
    const DEFAULT_VIDEO_THRESHOLD: u64 = 95; // 0.95 as percentage
    const DEFAULT_AUDIO_THRESHOLD: u64 = 95; // 0.95 as percentage
    const DEFAULT_REVENUE_REDIRECT_PERCENTAGE: u64 = 100; // 100%
    /// Single dispute submission fee (previously split across dispute_cost + protocol fee).
    const DEFAULT_DISPUTE_COST: u64 = 5_000_000_000; // 5 MYSO
    const DEFAULT_MIN_VOTE_STAKE: u64 = 1_000_000_000; // 1 MYSO minimum to vote
    const DEFAULT_MAX_VOTE_STAKE: u64 = 100_000_000_000; // 100 MYSO maximum per vote
    const DEFAULT_VOTING_DURATION_MS: u64 = 7 * 24 * 60 * 60 * 1000; // 7 days
    const DEFAULT_MAX_VOTES_PER_DISPUTE: u64 = 10000; // Default maximum votes allowed per dispute
    /// Default protocol slice at vault claim (basis points)
    const DEFAULT_CLAIM_TREASURY_FEE_BPS: u64 = 100;
    /// Default max referrer slice of amount after treasury fee (basis points), applied only when referrer is `Some` at claim
    const DEFAULT_MAX_REFERRAL_BPS: u64 = 500;
    /// Default max redirect (as bps of gross traded) for VIDEO when infringement is detected on embedded audio only;
    /// converted to an integer percent 0-100 and then multiplied by the usual similarity delta ramp.
    const DEFAULT_VIDEO_EMBEDDED_AUDIO_REDIRECT_BPS: u64 = 3000;
    /// Default minimum total voting stake for round-1 disputes (`0` = quorum check disabled).
    const DEFAULT_DISPUTE_QUORUM_BASE_STAKE: u64 = 0;
    /// Default second-round fee multiplier (10000 bps = same as `dispute_cost`).
    const DEFAULT_SECOND_ROUND_FEE_MULTIPLIER_BPS: u64 = 10000;
    /// Default second-round quorum multiplier (10000 bps = same as round-1 quorum base).
    const DEFAULT_SECOND_ROUND_QUORUM_MULTIPLIER_BPS: u64 = 10000;
    /// Max successful dispute submissions per post (lifetime).
    const MAX_POC_DISPUTES_PER_POST: u8 = 2;
    const DISPUTE_ROUND_FIRST: u8 = 1;
    const DISPUTE_ROUND_SECOND: u8 = 2;
    /// Validation constants
    const MAX_REASONING_LENGTH: u64 = 5000; // Max characters for reasoning
    const MAX_EVIDENCE_URLS: u64 = 10; // Max number of evidence URLs

    /// Admin capability for Proof of Creativity system management
    public struct PoCAdminCap has key, store {
        id: UID,
    }

    /// Global configuration for Proof of Creativity system
    public struct PoCConfig has key {
        id: UID,
        /// Oracle address authorized to submit analysis results
        oracle_address: address,
        /// Similarity thresholds for different media types (stored as percentages 0-100)
        image_threshold: u64,
        video_threshold: u64,
        audio_threshold: u64,
        /// Percentage of revenue to redirect when similarity detected (0-100)
        revenue_redirect_percentage: u64,
        /// Cost to submit a dispute (paid to ecosystem treasury)
        dispute_cost: u64,
        /// Minimum stake amount required to vote on disputes
        min_vote_stake: u64,
        /// Maximum stake amount allowed per vote
        max_vote_stake: u64,
        /// Voting period duration in milliseconds (on-chain clock)
        voting_duration_ms: u64,
        /// Maximum length for reasoning text
        max_reasoning_length: u64,
        /// Maximum number of evidence URLs allowed
        max_evidence_urls: u64,
        /// Maximum number of votes allowed per dispute
        max_votes_per_dispute: u64,
        /// Governance registry ID for PoC disputes
        dispute_governance_id: ID,
        /// Treasury fee (bps of gross) taken at vault claim
        claim_treasury_fee_bps: u64,
        /// Max referral fee (bps of post-treasury gross) when beneficiary supplies `Some(referrer)` at claim
        max_referral_bps: u64,
        /// Max redirect ceiling for VIDEO posts when only embedded audio matches (`embedded_audio_only_derivative`), in bps (0-10000)
        video_embedded_audio_redirect_bps: u64,
        /// Minimum total voting stake (`uphold` + `overturn`) for round 1 to count as full participation (`0` disables).
        dispute_quorum_base_stake: u64,
        /// Round-2 fee multiplier in bps; applied as `dispute_cost * bps / 10000`. Must be >= 10000.
        dispute_second_round_fee_multiplier_bps: u64,
        /// Round-2 quorum multiplier in bps; applied as `dispute_quorum_base_stake * bps / 10000`. Must be >= 10000.
        dispute_second_round_quorum_multiplier_bps: u64,
        /// Version for upgrades
        version: u64,
    }

    /// Individual vote record in a dispute
    public struct Vote has store, copy, drop {
        /// Voter's address
        voter: address,
        /// Vote choice (VOTE_UPHOLD or VOTE_OVERTURN)
        vote_choice: u8,
        /// Amount of MySo staked with this vote
        stake_amount: u64,
        /// Vote timestamp in milliseconds (on-chain clock)
        voted_at: u64,
    }

    /// Dispute challenging a PoC badge or revenue redirection with community voting
    public struct PoCDispute has key {
        id: UID,
        /// Post being disputed
        post_id: address,
        /// Address that submitted the dispute (post owner)
        disputer: address,
        /// Type of dispute (challenging badge or redirection)
        dispute_type: u8, // 1=challenge badge, 2=challenge redirection
        /// Current status of dispute
        status: u8,
        /// Evidence or reasoning provided by disputer
        evidence: String,
        /// Dispute submission timestamp
        submitted_at: u64,
        /// Wall-clock ms when voting starts (inclusive)
        voting_start_ms: u64,
        /// Wall-clock ms when voting ends (inclusive)
        voting_end_ms: u64,
        /// All votes cast on this dispute
        votes: vector<Vote>,
        /// Total stake on uphold side
        uphold_stake: u64,
        /// Total stake on overturn side
        overturn_stake: u64,
        /// Mapping of voter addresses to prevent double voting
        voter_records: Table<address, bool>,
        /// Prevents double-claim of voting rewards after resolution
        voting_rewards_claimed: Table<address, bool>,
        /// Total reward pool from losing side (set after resolution)
        reward_pool: Balance<MYSO>,
        /// 1 = first dispute on post, 2 = second (final allowed).
        dispute_round: u8,
        /// Fee charged when this dispute was opened.
        effective_dispute_fee: u64,
        /// Minimum `uphold_stake + overturn_stake` for stake-weighted outcome (else default uphold).
        required_total_stake_quorum: u64,
        /// Version for upgrades
        version: u64,
    }

    /// Simplified registry to track PoC statistics
    public struct PoCRegistry has key {
        id: UID,
        /// Total badges issued
        total_badges_issued: u64,
        /// Total redirections created
        total_redirections_created: u64,
        /// Total disputes submitted
        total_disputes_submitted: u64,
        /// Total votes cast across all disputes
        total_votes_cast: u64,
        /// Version for upgrades
        version: u64,
    }

    // === Events ===

    /// Emitted when oracle applies an explicit or similarity-based PoC outcome (for indexers).
    public struct PoCResultAppliedEvent has copy, drop {
        post_id: address,
        poc_outcome: u8,
        poc_redirection_kind: u8,
        similarity_detected: bool,
        timestamp: u64,
    }

    /// Event emitted when oracle submits analysis results
    public struct AnalysisSubmittedEvent has copy, drop {
        post_id: address,
        media_type: u8,
        similarity_detected: bool,
        highest_similarity_score: u64,
        oracle_address: address,
        timestamp: u64,
        reasoning: Option<String>, // Optional reasoning from oracle
        evidence_urls: Option<vector<String>>, // Optional array of evidence URLs
    }

    /// Event emitted when a PoC badge is issued
    public struct PoCBadgeIssuedEvent has copy, drop {
        badge_id: address,
        post_id: address,
        media_type: u8,
        issued_by: address,
        beneficiary_address: Option<address>,
        matched_anchor_id: Option<address>,
        media_index: u8,
        timestamp: u64,
    }

    public struct RevenueRedirectionActivatedEvent has copy, drop {
        redirection_id: address,
        accused_post_id: address,
        original_post_id: address,
        redirect_percentage: u64,
        similarity_score: u64,
        timestamp: u64,
    }

    /// Event emitted when a PoC dispute is submitted
    public struct PoCDisputeSubmittedEvent has copy, drop {
        dispute_id: address,
        post_id: address,
        disputer: address,
        dispute_type: u8,
        /// Fee paid (same as `effective_fee`).
        stake_amount: u64,
        dispute_round: u8,
        effective_fee: u64,
        required_total_stake_quorum: u64,
        post_poc_disputes_submitted_after: u8,
        voting_start_ms: u64,
        voting_end_ms: u64,
        /// Disputer evidence (duplicate of shared `PoCDispute`; for indexers/RPC consumption).
        evidence: String,
        timestamp: u64,
    }

    /// Event emitted when a vote is cast on a dispute
    public struct DisputeVoteCastEvent has copy, drop {
        dispute_id: address,
        voter: address,
        vote_choice: u8,
        stake_amount: u64,
        total_uphold_stake: u64,
        total_overturn_stake: u64,
        timestamp: u64,
    }

    /// Event emitted when a dispute is resolved
    public struct PoCDisputeResolvedEvent has copy, drop {
        dispute_id: address,
        post_id: address,
        resolution: u8, // upheld or overturned
        winning_side: u8, // VOTE_UPHOLD or VOTE_OVERTURN
        total_winning_stake: u64,
        total_losing_stake: u64,
        badge_revoked: bool,
        redirection_removed: bool,
        quorum_met: bool,
        post_poc_disputes_submitted: u8,
        timestamp: u64,
    }

    /// Event emitted when voting rewards are claimed
    public struct VotingRewardClaimedEvent has copy, drop {
        dispute_id: address,
        voter: address,
        original_stake: u64,
        reward_amount: u64,
        total_payout: u64,
        timestamp: u64,
    }

    /// Event emitted when PoC configuration is updated
    public struct PoCConfigUpdatedEvent has copy, drop {
        updated_by: address,
        oracle_address: address,
        image_threshold: u64,
        video_threshold: u64,
        audio_threshold: u64,
        revenue_redirect_percentage: u64,
        dispute_cost: u64,
        min_vote_stake: u64,
        max_vote_stake: u64,
        voting_duration_ms: u64,
        max_reasoning_length: u64,
        max_evidence_urls: u64,
        max_votes_per_dispute: u64,
        claim_treasury_fee_bps: u64,
        max_referral_bps: u64,
        video_embedded_audio_redirect_bps: u64,
        dispute_quorum_base_stake: u64,
        dispute_second_round_fee_multiplier_bps: u64,
        dispute_second_round_quorum_multiplier_bps: u64,
        timestamp: u64,
    }

    // === Utility Functions ===

    fun mul_div_u64_loose(a: u64, b: u64, divisor: u64): u64 {
        assert!(divisor > 0, EInvalidThreshold);
        (((a as u128) * (b as u128)) / (divisor as u128)) as u64
    }

    /// Bootstrap initialization function - creates the PoC configuration and registry
    public(package) fun bootstrap_init(clock: &Clock, ctx: &mut TxContext) {
        let sender = tx_context::sender(ctx);

        let config = PoCConfig {
            id: object::new(ctx),
            oracle_address: sender, // Initially set to deployer, should be updated
            image_threshold: DEFAULT_IMAGE_THRESHOLD,
            video_threshold: DEFAULT_VIDEO_THRESHOLD,
            audio_threshold: DEFAULT_AUDIO_THRESHOLD,
            revenue_redirect_percentage: DEFAULT_REVENUE_REDIRECT_PERCENTAGE,
            dispute_cost: DEFAULT_DISPUTE_COST,
            min_vote_stake: DEFAULT_MIN_VOTE_STAKE,
            max_vote_stake: DEFAULT_MAX_VOTE_STAKE,
            voting_duration_ms: DEFAULT_VOTING_DURATION_MS,
            max_reasoning_length: MAX_REASONING_LENGTH,
            max_evidence_urls: MAX_EVIDENCE_URLS,
            max_votes_per_dispute: DEFAULT_MAX_VOTES_PER_DISPUTE,
            dispute_governance_id: object::id_from_address(@0x0), // Placeholder for future governance
            claim_treasury_fee_bps: DEFAULT_CLAIM_TREASURY_FEE_BPS,
            max_referral_bps: DEFAULT_MAX_REFERRAL_BPS,
            video_embedded_audio_redirect_bps: DEFAULT_VIDEO_EMBEDDED_AUDIO_REDIRECT_BPS,
            dispute_quorum_base_stake: DEFAULT_DISPUTE_QUORUM_BASE_STAKE,
            dispute_second_round_fee_multiplier_bps: DEFAULT_SECOND_ROUND_FEE_MULTIPLIER_BPS,
            dispute_second_round_quorum_multiplier_bps: DEFAULT_SECOND_ROUND_QUORUM_MULTIPLIER_BPS,
            version: upgrade::current_version(),
        };

        // Emit event so indexer can populate poc_configuration table
        event::emit(PoCConfigUpdatedEvent {
            updated_by: sender,
            oracle_address: sender,
            image_threshold: DEFAULT_IMAGE_THRESHOLD,
            video_threshold: DEFAULT_VIDEO_THRESHOLD,
            audio_threshold: DEFAULT_AUDIO_THRESHOLD,
            revenue_redirect_percentage: DEFAULT_REVENUE_REDIRECT_PERCENTAGE,
            dispute_cost: DEFAULT_DISPUTE_COST,
            min_vote_stake: DEFAULT_MIN_VOTE_STAKE,
            max_vote_stake: DEFAULT_MAX_VOTE_STAKE,
            voting_duration_ms: DEFAULT_VOTING_DURATION_MS,
            max_reasoning_length: MAX_REASONING_LENGTH,
            max_evidence_urls: MAX_EVIDENCE_URLS,
            max_votes_per_dispute: DEFAULT_MAX_VOTES_PER_DISPUTE,
            claim_treasury_fee_bps: DEFAULT_CLAIM_TREASURY_FEE_BPS,
            max_referral_bps: DEFAULT_MAX_REFERRAL_BPS,
            video_embedded_audio_redirect_bps: DEFAULT_VIDEO_EMBEDDED_AUDIO_REDIRECT_BPS,
            dispute_quorum_base_stake: DEFAULT_DISPUTE_QUORUM_BASE_STAKE,
            dispute_second_round_fee_multiplier_bps: DEFAULT_SECOND_ROUND_FEE_MULTIPLIER_BPS,
            dispute_second_round_quorum_multiplier_bps: DEFAULT_SECOND_ROUND_QUORUM_MULTIPLIER_BPS,
            timestamp: clock::timestamp_ms(clock),
        });

        // Create and share PoC configuration
        transfer::share_object(config);
        
        // Create and share PoC registry
        transfer::share_object(
            PoCRegistry {
                id: object::new(ctx),
                total_badges_issued: 0,
                total_redirections_created: 0,
                total_disputes_submitted: 0,
                total_votes_cast: 0,
                version: upgrade::current_version(),
            }
        );
        poc_vault::bootstrap_init_directory(ctx);
    }

    /// Update PoC configuration (admin only)
    public entry fun update_poc_config(
        _: &PoCAdminCap,
        config: &mut PoCConfig,
        oracle_address: address,
        image_threshold: u64,
        video_threshold: u64,
        audio_threshold: u64,
        revenue_redirect_percentage: u64,
        dispute_cost: u64,
        min_vote_stake: u64,
        max_vote_stake: u64,
        voting_duration_ms: u64,
        max_reasoning_length: u64,
        max_evidence_urls: u64,
        max_votes_per_dispute: u64,
        claim_treasury_fee_bps: u64,
        max_referral_bps: u64,
        video_embedded_audio_redirect_bps: u64,
        dispute_quorum_base_stake: u64,
        dispute_second_round_fee_multiplier_bps: u64,
        dispute_second_round_quorum_multiplier_bps: u64,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        // Admin capability verification is handled by type system
        
        // Validate thresholds (0-100)
        assert!(image_threshold <= 100, EInvalidThreshold);
        assert!(video_threshold <= 100, EInvalidThreshold);
        assert!(audio_threshold <= 100, EInvalidThreshold);
        assert!(revenue_redirect_percentage <= 100, EInvalidThreshold);
        assert!(
            claim_treasury_fee_bps <= 10000 &&
                max_referral_bps <= 10000 &&
                video_embedded_audio_redirect_bps <= 10000,
            EInvalidThreshold
        );

        // Validate voting parameters
        assert!(min_vote_stake > 0 && min_vote_stake <= max_vote_stake, EInvalidStakeAmount);
        assert!(voting_duration_ms > 0, EInvalidThreshold);

        // Validate reasoning and evidence URL parameters
        assert!(max_reasoning_length > 0, EInvalidThreshold);
        assert!(max_evidence_urls > 0, EInvalidThreshold);
        assert!(max_votes_per_dispute > 0, EInvalidThreshold);
        assert!(dispute_second_round_fee_multiplier_bps >= 10000, EInvalidThreshold);
        assert!(dispute_second_round_quorum_multiplier_bps >= 10000, EInvalidThreshold);

        // Update configuration
        config.oracle_address = oracle_address;
        config.image_threshold = image_threshold;
        config.video_threshold = video_threshold;
        config.audio_threshold = audio_threshold;
        config.revenue_redirect_percentage = revenue_redirect_percentage;
        config.dispute_cost = dispute_cost;
        config.min_vote_stake = min_vote_stake;
        config.max_vote_stake = max_vote_stake;
        config.voting_duration_ms = voting_duration_ms;
        config.max_reasoning_length = max_reasoning_length;
        config.max_evidence_urls = max_evidence_urls;
        config.max_votes_per_dispute = max_votes_per_dispute;
        config.claim_treasury_fee_bps = claim_treasury_fee_bps;
        config.max_referral_bps = max_referral_bps;
        config.video_embedded_audio_redirect_bps = video_embedded_audio_redirect_bps;
        config.dispute_quorum_base_stake = dispute_quorum_base_stake;
        config.dispute_second_round_fee_multiplier_bps = dispute_second_round_fee_multiplier_bps;
        config.dispute_second_round_quorum_multiplier_bps = dispute_second_round_quorum_multiplier_bps;

        // Emit configuration update event
        event::emit(PoCConfigUpdatedEvent {
            updated_by: tx_context::sender(ctx),
            oracle_address,
            image_threshold,
            video_threshold,
            audio_threshold,
            revenue_redirect_percentage,
            dispute_cost,
            min_vote_stake,
            max_vote_stake,
            voting_duration_ms,
            max_reasoning_length,
            max_evidence_urls,
            max_votes_per_dispute,
            claim_treasury_fee_bps,
            max_referral_bps,
            video_embedded_audio_redirect_bps,
            dispute_quorum_base_stake,
            dispute_second_round_fee_multiplier_bps,
            dispute_second_round_quorum_multiplier_bps,
            timestamp: clock::timestamp_ms(clock),
        });
    }

    fun mint_shared_poc_badge_object(
        post_id: address,
        beneficiary_address: Option<address>,
        matched_anchor_id: Option<address>,
        similarity_score: u64,
        media_type: u8,
        oracle_address: address,
        analyzed_at: u64,
        ctx: &mut TxContext
    ): ID {
        let badge = poc_vault::new_poc_badge_object(
            post_id,
            beneficiary_address,
            matched_anchor_id,
            poc_vault::media_index_unspecified(),
            option::none(),
            option::none(),
            option::some(similarity_score),
            option::some(media_type),
            option::some(oracle_address),
            option::some(analyzed_at),
            ctx
        );
        let badge_id = object::id(&badge);
        poc_vault::share_po_badge_object(badge);
        badge_id
    }

    /// Oracle analyzes content and updates post PoC status. Does not require a social token pool.
    /// `derivative_redirection_target`: wallet (0) or beneficiary vault (1) when similarity is detected.
    /// `embedded_audio_only_derivative`: VIDEO only — oracle detected match on embedded audio track; uses `audio_threshold` and `video_embedded_audio_redirect_bps` ceiling with the same delta ramp.
    /// `apply_explicit_outcome` + `explicit_poc_outcome == 4`: royalty-free — accumulation via beneficiary vault (same as escrow redirect mode).
    fun run_analyze_and_update_post(
        config: &PoCConfig,
        registry: &mut PoCRegistry,
        vault_directory: &mut PoCVaultDirectory,
        post: &mut social_contracts::post::Post,
        media_type: u8,
        highest_similarity_score: u64,
        mut original_creator: Option<address>,
        derivative_redirection_target: u8,
        embedded_audio_only_derivative: bool,
        apply_explicit_outcome: bool,
        explicit_poc_outcome: u8,
        reasoning: Option<String>,
        evidence_urls: Option<vector<String>>,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        let caller = tx_context::sender(ctx);
        let timestamp = clock::timestamp_ms(clock);
        let post_id = social_contracts::post::get_id_address(post);
        
        // Verify caller is authorized oracle
        assert!(caller == config.oracle_address, ENotOracle);
        
        // Verify media type is valid
        assert!(
            media_type == MEDIA_TYPE_IMAGE || 
            media_type == MEDIA_TYPE_VIDEO || 
            media_type == MEDIA_TYPE_AUDIO,
            EInvalidMediaType
        );

        assert!(
            derivative_redirection_target == DERIVATIVE_TARGET_WALLET ||
            derivative_redirection_target == DERIVATIVE_TARGET_ESCROW,
            EInvalidMediaType
        );
        assert_embed_audio_derivative_media_type(embedded_audio_only_derivative, media_type);
        
        // Validate reasoning if provided
        if (option::is_some(&reasoning)) {
            let reasoning_val = option::borrow(&reasoning);
            let reasoning_len = string::length(reasoning_val);
            assert!(reasoning_len <= config.max_reasoning_length, EInvalidReasoning);
        };
        
        // Validate evidence URLs array if provided
        if (option::is_some(&evidence_urls)) {
            let urls = option::borrow(&evidence_urls);
            assert!(vector::length(urls) <= config.max_evidence_urls, EInvalidEvidenceUrls);
        };

        let mut similarity_detected = false;

        if (apply_explicit_outcome) {
            assert!(explicit_poc_outcome == OUTCOME_ROYALTY_FREE, EInvalidMediaType);
            let beneficiary = social_contracts::post::get_post_owner(post);
            social_contracts::post::update_poc_result(
                post,
                2,
                OUTCOME_ROYALTY_FREE,
                REDIRECT_ESCROW,
                option::some(beneficiary),
                option::some(100),
                reasoning,
                evidence_urls,
                highest_similarity_score,
                media_type,
                caller,
                timestamp,
            );
            let badge_object_id = mint_shared_poc_badge_object(
                post_id,
                option::some(beneficiary),
                option::none(),
                highest_similarity_score,
                media_type,
                caller,
                timestamp,
                ctx,
            );
            social_contracts::post::set_poc_badge_object_id(post, badge_object_id);
            let _ = poc_vault::ensure_beneficiary_vault(vault_directory, beneficiary, ctx);
            registry.total_redirections_created = registry.total_redirections_created + 1;
            event::emit(RevenueRedirectionActivatedEvent {
                redirection_id: post_id,
                accused_post_id: post_id,
                original_post_id: beneficiary,
                redirect_percentage: 100,
                similarity_score: highest_similarity_score,
                timestamp,
            });
            event::emit(PoCResultAppliedEvent {
                post_id,
                poc_outcome: OUTCOME_ROYALTY_FREE,
                poc_redirection_kind: REDIRECT_ESCROW,
                similarity_detected: false,
                timestamp,
            });
        } else {
            let threshold = if (embedded_audio_only_derivative) {
                config.audio_threshold
            } else {
                get_threshold_for_media_type(config, media_type)
            };
            similarity_detected = highest_similarity_score >= threshold && option::is_some(&original_creator);
            
            if (similarity_detected) {
                let original_creator_address = option::extract(&mut original_creator);
                let redirect_ceiling = if (embedded_audio_only_derivative) {
                    bps_to_redirect_percent(config.video_embedded_audio_redirect_bps)
                } else {
                    config.revenue_redirect_percentage
                };
                let redirect_percentage = similarity_redirect_percentage(
                    threshold,
                    highest_similarity_score,
                    redirect_ceiling,
                );

                let poc_outcome = if (derivative_redirection_target == DERIVATIVE_TARGET_ESCROW) {
                    OUTCOME_DERIVATIVE_ESCROW
                } else {
                    OUTCOME_DERIVATIVE_WALLET
                };
                let redirect_kind = if (derivative_redirection_target == DERIVATIVE_TARGET_WALLET) {
                    REDIRECT_WALLET
                } else {
                    REDIRECT_ESCROW
                };
                let redirect_to_opt = option::some(original_creator_address);

                social_contracts::post::update_poc_result(
                    post,
                    2,
                    poc_outcome,
                    redirect_kind,
                    redirect_to_opt,
                    option::some(redirect_percentage),
                    reasoning,
                    evidence_urls,
                    highest_similarity_score,
                    media_type,
                    caller,
                    timestamp,
                );
                let badge_object_id = mint_shared_poc_badge_object(
                    post_id,
                    option::some(original_creator_address),
                    option::none(),
                    highest_similarity_score,
                    media_type,
                    caller,
                    timestamp,
                    ctx,
                );
                social_contracts::post::set_poc_badge_object_id(post, badge_object_id);
                if (redirect_kind == REDIRECT_ESCROW) {
                    let _ = poc_vault::ensure_beneficiary_vault(vault_directory, original_creator_address, ctx);
                };
                registry.total_redirections_created = registry.total_redirections_created + 1;

                let emit_original = *option::borrow(&redirect_to_opt);
                event::emit(RevenueRedirectionActivatedEvent {
                    redirection_id: post_id,
                    accused_post_id: post_id,
                    original_post_id: emit_original,
                    redirect_percentage,
                    similarity_score: highest_similarity_score,
                    timestamp,
                });
                event::emit(PoCResultAppliedEvent {
                    post_id,
                    poc_outcome,
                    poc_redirection_kind: redirect_kind,
                    similarity_detected: true,
                    timestamp,
                });
            } else {
                assert!(social_contracts::post::is_poc_enabled(post), EDisabled);
                social_contracts::post::update_poc_result(
                    post,
                    1,
                    OUTCOME_ORIGINAL,
                    social_contracts::post::poc_redirection_none(),
                    option::none(),
                    option::none(),
                    reasoning,
                    evidence_urls,
                    highest_similarity_score,
                    media_type,
                    caller,
                    timestamp,
                );
                let post_owner_addr = social_contracts::post::get_post_owner(post);
                let badge_object_id = mint_shared_poc_badge_object(
                    post_id,
                    option::some(post_owner_addr),
                    option::none(),
                    highest_similarity_score,
                    media_type,
                    caller,
                    timestamp,
                    ctx,
                );
                social_contracts::post::set_poc_badge_object_id(post, badge_object_id);
                registry.total_badges_issued = registry.total_badges_issued + 1;
                event::emit(PoCBadgeIssuedEvent {
                    badge_id: object::id_to_address(&badge_object_id),
                    post_id,
                    media_type,
                    issued_by: caller,
                    beneficiary_address: option::some(post_owner_addr),
                    matched_anchor_id: option::none(),
                    media_index: poc_vault::media_index_unspecified(),
                    timestamp,
                });
                event::emit(PoCResultAppliedEvent {
                    post_id,
                    poc_outcome: OUTCOME_ORIGINAL,
                    poc_redirection_kind: social_contracts::post::poc_redirection_none(),
                    similarity_detected: false,
                    timestamp,
                });
            };
        };

        let analysis_similarity = if (apply_explicit_outcome) {
            false
        } else {
            similarity_detected
        };
        event::emit(AnalysisSubmittedEvent {
            post_id,
            media_type,
            similarity_detected: analysis_similarity,
            highest_similarity_score,
            oracle_address: caller,
            timestamp,
            reasoning,
            evidence_urls,
        });
    }

    public entry fun analyze_and_update_post(
        config: &PoCConfig,
        registry: &mut PoCRegistry,
        vault_directory: &mut PoCVaultDirectory,
        post: &mut social_contracts::post::Post,
        media_type: u8,
        highest_similarity_score: u64,
        original_creator: Option<address>,
        derivative_redirection_target: u8,
        embedded_audio_only_derivative: bool,
        apply_explicit_outcome: bool,
        explicit_poc_outcome: u8,
        reasoning: Option<String>,
        evidence_urls: Option<vector<String>>,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        run_analyze_and_update_post(
            config,
            registry,
            vault_directory,
            post,
            media_type,
            highest_similarity_score,
            original_creator,
            derivative_redirection_target,
            embedded_audio_only_derivative,
            apply_explicit_outcome,
            explicit_poc_outcome,
            reasoning,
            evidence_urls,
            clock,
            ctx,
        );
    }

    /// Same as `analyze_and_update_post`, then mirrors PoC redirect fields onto `TokenPool` when the post has an SPT registry entry.
    public entry fun analyze_and_update_post_sync_token_pool(
        config: &PoCConfig,
        registry: &mut PoCRegistry,
        token_registry: &social_contracts::social_proof_tokens::TokenRegistry,
        vault_directory: &mut PoCVaultDirectory,
        post: &mut social_contracts::post::Post,
        pool: &mut social_contracts::social_proof_tokens::TokenPool,
        media_type: u8,
        highest_similarity_score: u64,
        original_creator: Option<address>,
        derivative_redirection_target: u8,
        embedded_audio_only_derivative: bool,
        apply_explicit_outcome: bool,
        explicit_poc_outcome: u8,
        reasoning: Option<String>,
        evidence_urls: Option<vector<String>>,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        run_analyze_and_update_post(
            config,
            registry,
            vault_directory,
            post,
            media_type,
            highest_similarity_score,
            original_creator,
            derivative_redirection_target,
            embedded_audio_only_derivative,
            apply_explicit_outcome,
            explicit_poc_outcome,
            reasoning,
            evidence_urls,
            clock,
            ctx,
        );
        let post_id = social_contracts::post::get_id_address(post);
        let caller = tx_context::sender(ctx);
        assert!(
            social_contracts::social_proof_tokens::token_exists(token_registry, post_id),
            ENoTokenPoolForPost,
        );
        social_contracts::social_proof_tokens::sync_token_pool_poc_from_post(
            token_registry,
            pool,
            post,
            caller,
            clock,
            ctx,
        );
    }

    /// Claim accumulated balance for coin type `T` from a beneficiary vault (splits per config bps + optional referrer).
    public entry fun claim_beneficiary_vault_balance<T>(
        config: &PoCConfig,
        treasury: &EcosystemTreasury,
        vault: &mut PoCBeneficiaryVault,
        referrer_opt: Option<address>,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        poc_vault::claim_vault_balance<T>(
            vault,
            treasury,
            config.claim_treasury_fee_bps,
            config.max_referral_bps,
            referrer_opt,
            clock,
            ctx,
        );
    }

    /// Submit a PoC dispute with community voting
    public entry fun submit_poc_dispute(
        config: &PoCConfig,
        registry: &mut PoCRegistry,
        treasury: &EcosystemTreasury,
        post: &mut social_contracts::post::Post,
        evidence: String,
        mut payment: Coin<MYSO>,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        let disputer = tx_context::sender(ctx);
        let now = clock::timestamp_ms(clock);
        let post_id = social_contracts::post::get_id_address(post);

        let submitted_before = social_contracts::post::poc_disputes_submitted(post);
        assert!(submitted_before < MAX_POC_DISPUTES_PER_POST, EDisputeCapReached);
        let dispute_round = submitted_before + 1;

        let fee = if (dispute_round == DISPUTE_ROUND_FIRST) {
            config.dispute_cost
        } else {
            mul_div_u64_loose(
                config.dispute_cost,
                config.dispute_second_round_fee_multiplier_bps,
                10000,
            )
        };
        let required_total_stake_quorum = if (dispute_round == DISPUTE_ROUND_FIRST) {
            config.dispute_quorum_base_stake
        } else {
            mul_div_u64_loose(
                config.dispute_quorum_base_stake,
                config.dispute_second_round_quorum_multiplier_bps,
                10000,
            )
        };

        // Verify sufficient payment
        assert!(coin::value(&payment) >= fee, EInsufficientFunds);

        // Validate evidence length
        let evidence_len = string::length(&evidence);
        assert!(evidence_len <= config.max_reasoning_length, EInvalidReasoning);

        // Verify only post owner can dispute their post's PoC status
        assert!(disputer == social_contracts::post::get_post_owner(post), EUnauthorized);

        // Verify the post has PoC data to dispute (badge / redirection metadata)
        let has_poc_data = option::is_some(social_contracts::post::get_revenue_redirect_to(post)) ||
                            social_contracts::post::has_poc_badge(post);
        assert!(has_poc_data, EPostNotFound);

        // Extract dispute fee and send to ecosystem treasury
        let dispute_fee = coin::split(&mut payment, fee, ctx);
        transfer::public_transfer(dispute_fee, social_contracts::profile::get_treasury_address(treasury));

        // Return excess payment
        if (coin::value(&payment) > 0) {
            transfer::public_transfer(payment, disputer);
        } else {
            coin::destroy_zero(payment);
        };

        social_contracts::post::increment_poc_disputes_submitted(post);
        let post_poc_disputes_submitted_after = social_contracts::post::poc_disputes_submitted(post);

        let voting_start_ms = now;
        let voting_end_ms = now + config.voting_duration_ms;
        let evidence_for_event = copy evidence;

        // Create dispute with voting mechanism
        let dispute = PoCDispute {
            id: object::new(ctx),
            post_id,
            disputer,
            dispute_type: 1, // Generic PoC dispute
            status: DISPUTE_STATUS_VOTING,
            evidence,
            submitted_at: now,
            voting_start_ms,
            voting_end_ms,
            votes: vector::empty(),
            uphold_stake: 0,
            overturn_stake: 0,
            voter_records: table::new(ctx),
            voting_rewards_claimed: table::new(ctx),
            reward_pool: balance::zero(),
            dispute_round,
            effective_dispute_fee: fee,
            required_total_stake_quorum,
            version: upgrade::current_version(),
        };

        let dispute_id = object::uid_to_address(&dispute.id);

        // Update registry tracking
        registry.total_disputes_submitted = registry.total_disputes_submitted + 1;

        // Emit dispute submitted event
        event::emit(PoCDisputeSubmittedEvent {
            dispute_id,
            post_id,
            disputer,
            dispute_type: 1,
            stake_amount: fee,
            dispute_round,
            effective_fee: fee,
            required_total_stake_quorum,
            post_poc_disputes_submitted_after,
            voting_start_ms,
            voting_end_ms,
            evidence: evidence_for_event,
            timestamp: now,
        });

        // Share dispute for public voting
        transfer::share_object(dispute);
    }

    /// Cast a vote on a PoC dispute (community voting)
    public entry fun vote_on_dispute(
        config: &PoCConfig,
        registry: &mut PoCRegistry,
        dispute: &mut PoCDispute,
        vote_choice: u8, // VOTE_UPHOLD or VOTE_OVERTURN
        stake_coin: Coin<MYSO>,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        let voter = tx_context::sender(ctx);
        let t = clock::timestamp_ms(clock);
        let stake_amount = coin::value(&stake_coin);

        // Validate vote choice
        assert!(vote_choice == VOTE_UPHOLD || vote_choice == VOTE_OVERTURN, EUnauthorized);

        // Check vote limit
        let current_votes = vector::length(&dispute.votes);
        assert!(current_votes < config.max_votes_per_dispute, ETooManyVotes);

        // Validate stake amount is within bounds
        assert!(stake_amount >= config.min_vote_stake && stake_amount <= config.max_vote_stake, EInvalidStakeAmount);

        // Verify voting period is active
        assert!(t >= dispute.voting_start_ms, EVotingNotActive);
        assert!(t <= dispute.voting_end_ms, EVotingEnded);

        // Verify voter hasn't already voted
        assert!(!table::contains(&dispute.voter_records, voter), EAlreadyVoted);

        // Record the vote
        let vote = Vote {
            voter,
            vote_choice,
            stake_amount,
            voted_at: t,
        };

        vector::push_back(&mut dispute.votes, vote);
        table::add(&mut dispute.voter_records, voter, true);

        // Update stake totals
        if (vote_choice == VOTE_UPHOLD) {
            dispute.uphold_stake = dispute.uphold_stake + stake_amount;
        } else {
            dispute.overturn_stake = dispute.overturn_stake + stake_amount;
        };

        // Take stake and hold it in the dispute
        let stake_balance = coin::into_balance(stake_coin);
        balance::join(&mut dispute.reward_pool, stake_balance);

        // Update registry tracking
        registry.total_votes_cast = registry.total_votes_cast + 1;

        // Emit vote event
        event::emit(DisputeVoteCastEvent {
            dispute_id: object::uid_to_address(&dispute.id),
            voter,
            vote_choice,
            stake_amount,
            total_uphold_stake: dispute.uphold_stake,
            total_overturn_stake: dispute.overturn_stake,
            timestamp: t,
        });
    }

    fun finalize_dispute_voting_resolution(
        dispute: &mut PoCDispute,
        post: &mut social_contracts::post::Post,
        clock: &Clock,
        ctx: &TxContext
    ) {
        let t = clock::timestamp_ms(clock);
        let dispute_id = object::uid_to_address(&dispute.id);

        // Verify voting period has ended
        assert!(t > dispute.voting_end_ms, EVotingNotActive);

        // Verify there are votes to resolve
        assert!(vector::length(&dispute.votes) > 0, ENoVotesToResolve);

        let total_stake = dispute.uphold_stake + dispute.overturn_stake;
        let quorum_met = dispute.required_total_stake_quorum == 0 ||
            total_stake >= dispute.required_total_stake_quorum;

        // Determine winning side (insufficient quorum defaults to uphold)
        let winning_side = if (!quorum_met) {
            VOTE_UPHOLD
        } else if (dispute.uphold_stake > dispute.overturn_stake) {
            VOTE_UPHOLD
        } else {
            VOTE_OVERTURN
        };

        let (total_winning_stake, total_losing_stake) = if (winning_side == VOTE_UPHOLD) {
            (dispute.uphold_stake, dispute.overturn_stake)
        } else {
            (dispute.overturn_stake, dispute.uphold_stake)
        };

        // Apply dispute resolution to post
        let (badge_revoked, redirection_removed) = if (winning_side == VOTE_OVERTURN) {
            // Challenger wins - clear PoC data and refund escrow to post owner
            social_contracts::post::clear_poc_data(post);
            (true, true)
        } else {
            // Original decision stands - no changes needed
            (false, false)
        };

        // Update dispute status
        dispute.status = if (winning_side == VOTE_UPHOLD) {
            DISPUTE_STATUS_RESOLVED_UPHELD
        } else {
            DISPUTE_STATUS_RESOLVED_OVERTURNED
        };

        let post_poc_disputes_submitted = social_contracts::post::poc_disputes_submitted(post);

        // Emit resolution event
        event::emit(PoCDisputeResolvedEvent {
            dispute_id,
            post_id: dispute.post_id,
            resolution: dispute.status,
            winning_side,
            total_winning_stake,
            total_losing_stake,
            badge_revoked,
            redirection_removed,
            quorum_met,
            post_poc_disputes_submitted,
            timestamp: t,
        });
    }

    /// Resolve PoC dispute after voting period ends
    public entry fun resolve_dispute_voting(
        dispute: &mut PoCDispute,
        post: &mut social_contracts::post::Post,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        finalize_dispute_voting_resolution(dispute, post, clock, ctx);
    }

    /// Same as `resolve_dispute_voting`, then mirrors post PoC state onto `TokenPool` when the post has an SPT registry entry.
    public entry fun resolve_dispute_voting_sync_token_pool(
        dispute: &mut PoCDispute,
        post: &mut social_contracts::post::Post,
        token_registry: &social_contracts::social_proof_tokens::TokenRegistry,
        pool: &mut social_contracts::social_proof_tokens::TokenPool,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        finalize_dispute_voting_resolution(dispute, post, clock, ctx);
        let post_id = social_contracts::post::get_id_address(post);
        let sender = tx_context::sender(ctx);
        assert!(
            social_contracts::social_proof_tokens::token_exists(token_registry, post_id),
            ENoTokenPoolForPost,
        );
        social_contracts::social_proof_tokens::sync_token_pool_poc_from_post(
            token_registry,
            pool,
            post,
            sender,
            clock,
            ctx,
        );
    }

    /// Claim voting rewards after dispute resolution
    public entry fun claim_voting_reward(
        dispute: &mut PoCDispute,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        let claimer = tx_context::sender(ctx);
        let timestamp = clock::timestamp_ms(clock);
        let dispute_id = object::uid_to_address(&dispute.id);
        
        // Verify dispute is resolved
        assert!(
            dispute.status == DISPUTE_STATUS_RESOLVED_UPHELD || 
            dispute.status == DISPUTE_STATUS_RESOLVED_OVERTURNED,
            EVotingNotActive
        );
        
        // Find the voter's vote and verify they voted on winning side
        let votes_len = vector::length(&dispute.votes);
        let mut vote_index = 0;
        let mut found_vote = false;
        let mut voter_stake = 0;
        let mut voter_choice = 0;
        
        while (vote_index < votes_len && !found_vote) {
            let vote = vector::borrow(&dispute.votes, vote_index);
            if (vote.voter == claimer) {
                found_vote = true;
                voter_stake = vote.stake_amount;
                voter_choice = vote.vote_choice;
            };
            vote_index = vote_index + 1;
        };
        
        assert!(found_vote, EUnauthorized);

        assert!(
            !table::contains(&dispute.voting_rewards_claimed, claimer),
            EDuplicateVoteRewardClaim
        );
        
        // Determine winning side from resolved status
        let winning_side = if (dispute.status == DISPUTE_STATUS_RESOLVED_UPHELD) {
            VOTE_UPHOLD
        } else {
            VOTE_OVERTURN
        };
        
        assert!(voter_choice == winning_side, EUnauthorized);
        
        // Calculate reward
        let (total_winning_stake, total_losing_stake) = if (winning_side == VOTE_UPHOLD) {
            (dispute.uphold_stake, dispute.overturn_stake)
        } else {
            (dispute.overturn_stake, dispute.uphold_stake)
        };
        
        // Calculate proportional reward: original stake + share of losing side
        let reward_from_losers = if (total_winning_stake > 0) {
            (((voter_stake as u128) * (total_losing_stake as u128)) / (total_winning_stake as u128)) as u64
        } else {
            0
        };
        
        let total_payout = voter_stake + reward_from_losers;
        
        // Verify sufficient balance in reward pool
        assert!(balance::value(&dispute.reward_pool) >= total_payout, EInsufficientFunds);
        
        // Transfer reward to voter
        let reward_coin = coin::from_balance(
            balance::split(&mut dispute.reward_pool, total_payout),
            ctx
        );
        transfer::public_transfer(reward_coin, claimer);
        
        table::add(&mut dispute.voting_rewards_claimed, claimer, true);

        // Emit reward event
        event::emit(VotingRewardClaimedEvent {
            dispute_id,
            voter: claimer,
            original_stake: voter_stake,
            reward_amount: reward_from_losers,
            total_payout,
            timestamp,
        });
    }

    // === Helper Functions ===

    fun assert_embed_audio_derivative_media_type(embedded_audio_only_derivative: bool, media_type: u8) {
        assert!(
            !embedded_audio_only_derivative || media_type == MEDIA_TYPE_VIDEO,
            EInvalidMediaType
        );
    }

    fun similarity_redirect_percentage(
        threshold: u64,
        highest_similarity_score: u64,
        redirect_ceiling: u64,
    ): u64 {
        let delta_numerator = highest_similarity_score - threshold;
        let delta_denominator = 100 - threshold;
        let delta_percentage = if (delta_denominator > 0) {
            (delta_numerator * 100) / delta_denominator
        } else {
            100
        };
        (redirect_ceiling * delta_percentage) / 100
    }

    /// Converts basis points (0-10000) to redirect percent 0-100 for `post::revenue_redirect_percentage` (nearest integer percent).
    fun bps_to_redirect_percent(bps: u64): u64 {
        let capped = if (bps > 10000) {
            10000
        } else {
            bps
        };
        let p = (capped + 50) / 100;
        if (p > 100) {
            100
        } else {
            p
        }
    }

    /// Get similarity threshold for a media type
    fun get_threshold_for_media_type(config: &PoCConfig, media_type: u8): u64 {
        if (media_type == MEDIA_TYPE_IMAGE) {
            config.image_threshold
        } else if (media_type == MEDIA_TYPE_VIDEO) {
            config.video_threshold
        } else if (media_type == MEDIA_TYPE_AUDIO) {
            config.audio_threshold
        } else {
            abort EInvalidMediaType
        }
    }

    // === Public Getter Functions ===

    /// Check if an address is the authorized oracle
    public fun is_authorized_oracle(config: &PoCConfig, caller: address): bool {
        caller == config.oracle_address
    }

    /// Get registry statistics
    public fun get_registry_stats(registry: &PoCRegistry): (u64, u64, u64, u64) {
        (
            registry.total_badges_issued,
            registry.total_redirections_created,
            registry.total_disputes_submitted,
            registry.total_votes_cast
        )
    }

    /// Check if a post has PoC data that can be disputed
    public fun has_poc_data(post: &social_contracts::post::Post): bool {
        option::is_some(social_contracts::post::get_revenue_redirect_to(post)) ||
        social_contracts::post::has_poc_badge(post)
    }

    /// Get dispute voting status; `current_time_ms` should be `clock::timestamp_ms(clock)`.
    public fun get_dispute_voting_status(dispute: &PoCDispute, current_time_ms: u64): (bool, bool, u8) {
        let voting_active = current_time_ms >= dispute.voting_start_ms && current_time_ms <= dispute.voting_end_ms;
        let voting_ended = current_time_ms > dispute.voting_end_ms;
        (voting_active, voting_ended, dispute.status)
    }

    /// Get dispute stake totals
    public fun get_dispute_stakes(dispute: &PoCDispute): (u64, u64, u64) {
        (dispute.uphold_stake, dispute.overturn_stake, vector::length(&dispute.votes))
    }

    /// Check if user has already voted on dispute
    public fun has_user_voted(dispute: &PoCDispute, user: address): bool {
        table::contains(&dispute.voter_records, user)
    }

    // === Versioning Functions ===

    /// Get the version of the PoC config
    public fun config_version(config: &PoCConfig): u64 {
        config.version
    }

    /// Get the version of a PoC dispute
    public fun dispute_version(dispute: &PoCDispute): u64 {
        dispute.version
    }

    /// Get the version of the PoC registry
    public fun registry_version(registry: &PoCRegistry): u64 {
        registry.version
    }

    /// Migration function for PoCConfig
    public entry fun migrate_poc_config(
        config: &mut PoCConfig,
        _: &UpgradeAdminCap,
        ctx: &mut TxContext
    ) {
        let current_version = upgrade::current_version();
        
        // Verify this is an upgrade
        assert!(config.version < current_version, EWrongVersion);
        
        // Remember old version and update to new version
        let old_version = config.version;
        config.version = current_version;
        
        // Emit event for object migration
        let config_id = object::id(config);
        upgrade::emit_migration_event(
            config_id,
            string::utf8(b"PoCConfig"),
            old_version,
            tx_context::sender(ctx)
        );
    }

    /// Migration function for PoCDispute
    public entry fun migrate_poc_dispute(
        dispute: &mut PoCDispute,
        _: &UpgradeAdminCap,
        ctx: &mut TxContext
    ) {
        let current_version = upgrade::current_version();
        
        // Verify this is an upgrade
        assert!(dispute.version < current_version, EWrongVersion);
        
        // Remember old version and update to new version
        let old_version = dispute.version;
        dispute.version = current_version;
        
        // Emit event for object migration
        let dispute_id = object::id(dispute);
        upgrade::emit_migration_event(
            dispute_id,
            string::utf8(b"PoCDispute"),
            old_version,
            tx_context::sender(ctx)
        );
    }

    /// Migration function for PoCRegistry
    public entry fun migrate_poc_registry(
        registry: &mut PoCRegistry,
        _: &UpgradeAdminCap,
        ctx: &mut TxContext
    ) {
        let current_version = upgrade::current_version();
        
        // Verify this is an upgrade
        assert!(registry.version < current_version, EWrongVersion);
        
        // Remember old version and update to new version
        let old_version = registry.version;
        registry.version = current_version;
        
        // Emit event for object migration
        let registry_id = object::id(registry);
        upgrade::emit_migration_event(
            registry_id,
            string::utf8(b"PoCRegistry"),
            old_version,
            tx_context::sender(ctx)
        );
    }

    /// Create a PoCAdminCap for bootstrap (package visibility only)
    /// This function is only callable by other modules in the same package
    public(package) fun create_poc_admin_cap(ctx: &mut TxContext): PoCAdminCap {
        PoCAdminCap {
            id: object::new(ctx)
        }
    }

    // === Test-only functions ===

    #[test_only]
    public fun test_bps_to_redirect_percent(bps: u64): u64 {
        bps_to_redirect_percent(bps)
    }

    #[test_only]
    public fun test_similarity_redirect_percentage(
        threshold: u64,
        highest_similarity_score: u64,
        redirect_ceiling: u64,
    ): u64 {
        similarity_redirect_percentage(threshold, highest_similarity_score, redirect_ceiling)
    }

    #[test_only]
    public fun test_assert_embed_audio_derivative_media_type(
        embedded_audio_only_derivative: bool,
        media_type: u8,
    ) {
        assert_embed_audio_derivative_media_type(embedded_audio_only_derivative, media_type);
    }

    #[test_only]
    public fun test_mul_div_u64_loose(a: u64, b: u64, divisor: u64): u64 {
        mul_div_u64_loose(a, b, divisor)
    }

    #[test_only]
    /// Initialize the PoC system for testing
    public fun test_init(clock: &Clock, ctx: &mut TxContext) {
        let sender = tx_context::sender(ctx);
        bootstrap_init(clock, ctx);
        
        // Create and transfer admin capabilities to the transaction sender
        transfer::public_transfer(PoCAdminCap { id: object::new(ctx) }, sender);
    }
}

#[allow(duplicate_alias, unused_use, lint(public_entry))]
module social_contracts::poc_vault {
    use std::option::{Self, Option};
    use std::string::String;
    use std::type_name::{Self, TypeName};
    use std::vector;

    use myso::{
        bag::{Self, Bag},
        balance::{Self, Balance},
        clock::{Self, Clock},
        coin::{Self, Coin},
        event,
        object::{Self, UID},
        table::{Self, Table},
        transfer,
        tx_context::{Self, TxContext},
    };
    use social_contracts::profile::{Self, EcosystemTreasury};
    use social_contracts::upgrade;

    const EUnauthorized: u64 = 1;
    const EWrongBeneficiary: u64 = 2;
    const EVaultEmpty: u64 = 3;
    const EInvalidReferrer: u64 = 4;
    const EBpsTooLarge: u64 = 5;
    const EDEPOSIT_BELOW_MINIMUM: u64 = 6;
    const EClaimInvariant: u64 = 7;

    /// Minimum amount (per asset) accepted into the vault; configurable later via governance if needed.
    const MIN_VAULT_DEPOSIT_AMOUNT: u64 = 1;

    /// Bag key for `Balance<T>` buckets (same phantom-key pattern as orderbook `BalanceKey`).
    public struct VaultBalanceKey<phantom T> has copy, drop, store {}

    /// Sentinel media index when oracle did not bind to a specific attachment slot.
    public fun media_index_unspecified(): u8 {
        255
    }

    /// Maps beneficiary wallet → shared `PoCBeneficiaryVault` object address (lookup only).
    public struct PoCVaultDirectory has key {
        id: UID,
        vault_by_beneficiary: Table<address, address>,
        version: u64,
    }

    /// One shared vault per beneficiary; anyone may deposit; only beneficiary may claim per coin type.
    public struct PoCBeneficiaryVault has key {
        id: UID,
        beneficiary: address,
        balances: Bag,
        version: u64,
    }

    /// Authoritative on-chain PoC badge record for a post (shared object).
    public struct PoCBadgeObject has key {
        id: UID,
        post_id: address,
        beneficiary_address: Option<address>,
        matched_anchor_id: Option<address>,
        media_index: u8,
        reasoning: Option<String>,
        evidence_urls: Option<vector<String>>,
        similarity_score: Option<u64>,
        media_type: Option<u8>,
        oracle_address: Option<address>,
        analyzed_at: Option<u64>,
        version: u64,
    }

    public struct PoCBeneficiaryVaultDepositEvent has copy, drop {
        vault_id: address,
        beneficiary: address,
        coin_type: TypeName,
        amount: u64,
        source_post_id: Option<address>,
        timestamp: u64,
    }

    public struct PoCBeneficiaryVaultClaimedEvent has copy, drop {
        vault_id: address,
        beneficiary: address,
        coin_type: TypeName,
        referrer: Option<address>,
        treasury_amount: u64,
        referrer_amount: u64,
        beneficiary_amount: u64,
        timestamp: u64,
    }

    public(package) fun bootstrap_init_directory(ctx: &mut TxContext) {
        transfer::share_object(PoCVaultDirectory {
            id: object::new(ctx),
            vault_by_beneficiary: table::new(ctx),
            version: upgrade::current_version(),
        });
    }

    /// Beneficiary wallet for this vault (depositor routing assertions).
    public fun beneficiary_address(vault: &PoCBeneficiaryVault): address {
        vault.beneficiary
    }

    /// Returns the vault object address for `beneficiary`, creating and sharing a vault if needed.
    public(package) fun ensure_beneficiary_vault(
        directory: &mut PoCVaultDirectory,
        beneficiary: address,
        ctx: &mut TxContext
    ): address {
        if (table::contains(&directory.vault_by_beneficiary, beneficiary)) {
            *table::borrow(&directory.vault_by_beneficiary, beneficiary)
        } else {
            let vault = PoCBeneficiaryVault {
                id: object::new(ctx),
                beneficiary,
                balances: bag::new(ctx),
                version: upgrade::current_version(),
            };
            let vault_address = object::uid_to_address(&vault.id);
            transfer::share_object(vault);
            table::add(&mut directory.vault_by_beneficiary, beneficiary, vault_address);
            vault_address
        }
    }

    public(package) fun deposit_coin<T>(
        vault: &mut PoCBeneficiaryVault,
        expected_beneficiary: address,
        fee_coin: Coin<T>,
        source_post_id: Option<address>,
        clock: &Clock,
        _ctx: &TxContext
    ) {
        assert!(vault.beneficiary == expected_beneficiary, EWrongBeneficiary);
        let amount = coin::value(&fee_coin);
        if (amount == 0) {
            coin::destroy_zero(fee_coin);
            return
        };
        assert!(amount >= MIN_VAULT_DEPOSIT_AMOUNT, EDEPOSIT_BELOW_MINIMUM);
        let vault_id = object::uid_to_address(&vault.id);
        let coin_type = type_name::with_defining_ids<T>();
        let key = VaultBalanceKey<T> {};
        let incoming = coin::into_balance(fee_coin);
        if (bag::contains(&vault.balances, key)) {
            let slot: &mut Balance<T> = bag::borrow_mut(&mut vault.balances, key);
            balance::join(slot, incoming);
        } else {
            bag::add(&mut vault.balances, key, incoming);
        };
        event::emit(PoCBeneficiaryVaultDepositEvent {
            vault_id,
            beneficiary: vault.beneficiary,
            coin_type,
            amount,
            source_post_id,
            timestamp: clock::timestamp_ms(clock),
        });
    }

    /// Claim entire balance for coin type `T` with treasury fee (bps) and optional referrer slice.
    public(package) fun claim_vault_balance<T>(
        vault: &mut PoCBeneficiaryVault,
        treasury: &EcosystemTreasury,
        treasury_fee_bps: u64,
        max_referral_bps: u64,
        referrer_opt: Option<address>,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        assert!(tx_context::sender(ctx) == vault.beneficiary, EUnauthorized);
        assert!(treasury_fee_bps <= 10000 && max_referral_bps <= 10000, EBpsTooLarge);

        let key = VaultBalanceKey<T> {};
        assert!(
            bag::contains_with_type<VaultBalanceKey<T>, Balance<T>>(&vault.balances, key),
            EVaultEmpty
        );
        let stored_balance = bag::remove<VaultBalanceKey<T>, Balance<T>>(&mut vault.balances, key);
        let gross = balance::value(&stored_balance);
        assert!(gross > 0, EVaultEmpty);

        if (option::is_some(&referrer_opt)) {
            let r = *option::borrow(&referrer_opt);
            assert!(r != @0x0 && r != vault.beneficiary, EInvalidReferrer);
        };

        let treasury_amt = (gross * treasury_fee_bps) / 10000;
        let after_treasury = gross - treasury_amt;
        let referrer_amt = if (option::is_some(&referrer_opt)) {
            (after_treasury * max_referral_bps) / 10000
        } else {
            0
        };
        let beneficiary_amt = gross - treasury_amt - referrer_amt;
        assert!(
            treasury_amt + referrer_amt + beneficiary_amt == gross,
            EClaimInvariant
        );

        let mut all_coin = coin::from_balance(stored_balance, ctx);

        if (treasury_amt > 0) {
            let treasury_coin = coin::split(&mut all_coin, treasury_amt, ctx);
            transfer::public_transfer(treasury_coin, profile::get_treasury_address(treasury));
        };
        if (referrer_amt > 0) {
            let ref_addr = *option::borrow(&referrer_opt);
            let ref_coin = coin::split(&mut all_coin, referrer_amt, ctx);
            transfer::public_transfer(ref_coin, ref_addr);
        };
        if (beneficiary_amt > 0) {
            let ben_coin = coin::split(&mut all_coin, beneficiary_amt, ctx);
            transfer::public_transfer(ben_coin, vault.beneficiary);
        };
        coin::destroy_zero(all_coin);

        event::emit(PoCBeneficiaryVaultClaimedEvent {
            vault_id: object::uid_to_address(&vault.id),
            beneficiary: vault.beneficiary,
            coin_type: type_name::with_defining_ids<T>(),
            referrer: referrer_opt,
            treasury_amount: treasury_amt,
            referrer_amount: referrer_amt,
            beneficiary_amount: beneficiary_amt,
            timestamp: clock::timestamp_ms(clock),
        });
    }

    public(package) fun new_poc_badge_object(
        post_id: address,
        beneficiary_address: Option<address>,
        matched_anchor_id: Option<address>,
        media_index: u8,
        reasoning: Option<String>,
        evidence_urls: Option<vector<String>>,
        similarity_score: Option<u64>,
        media_type: Option<u8>,
        oracle_address: Option<address>,
        analyzed_at: Option<u64>,
        ctx: &mut TxContext
    ): PoCBadgeObject {
        PoCBadgeObject {
            id: object::new(ctx),
            post_id,
            beneficiary_address,
            matched_anchor_id,
            media_index,
            reasoning,
            evidence_urls,
            similarity_score,
            media_type,
            oracle_address,
            analyzed_at,
            version: upgrade::current_version(),
        }
    }

    public(package) fun share_po_badge_object(badge: PoCBadgeObject) {
        transfer::share_object(badge);
    }

    public(package) fun po_badge_object_address(badge: &PoCBadgeObject): address {
        object::uid_to_address(&badge.id)
    }

    #[test_only]
    /// Shared empty vault for tests that must pass a `PoCBeneficiaryVault` when PoC vault-mode is inactive (unused).
    public fun create_shared_dummy_vault_for_testing(beneficiary: address, ctx: &mut TxContext) {
        transfer::share_object(PoCBeneficiaryVault {
            id: object::new(ctx),
            beneficiary,
            balances: bag::new(ctx),
            version: upgrade::current_version(),
        });
    }
}
