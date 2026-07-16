// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

/// Governance module for the MySocial network
/// Manages the decentralized governance system with delegate council and community assembly
/// Implements proposal submission, voting, and execution processes
///
/// Delegate panel refresh runs inside [`try_update_delegate_panel_if_due`], which is invoked from
/// `update_delegate_panel` and piggybacked on delegate-election flows (nominations,
/// `vote_for_delegate` / `clear_vote_for_delegate`), proposal submission, delegate **proposal** review
/// votes ([`run_delegate_review_vote`]), and community **finalize** ([`finalize_community_voting_internals`]).
/// Community voting entries ([`community_vote_on_proposal`]) intentionally do not trigger refresh.
/// A refresh applies when the epoch has reached at least one full `delegate_term_epochs` boundary and
/// that boundary has not been applied yet (catch-up: e.g. epoch 93 applies boundary 90 if still pending).
/// Refresh is lazy until one of those transactions runs while eligible; use a keeper if you need a
/// strict wall-clock guarantee when governance is idle.

#[allow(duplicate_alias, unused_use, lint(public_entry))]
module social_contracts::governance {
    use std::{option, string::{Self, String}, vector};
    
    use myso::{
        object::{Self, UID, ID},
        tx_context::{Self, TxContext},
        clock::{Self, Clock},
        transfer,
        dynamic_field,
        vec_set::{Self, VecSet},
        event,
        table::{Self, Table},
        coin::{Self, Coin},
        balance::{Self, Balance}
    };
    use myso::myso::MYSO;

    use mydata::bf_hmac_encryption::{EncryptedObject, VerifiedDerivedKey, PublicKey, decrypt};
    
    use social_contracts::profile::{Self as profile, EcosystemTreasury};
    use social_contracts::upgrade::{Self, UpgradeAdminCap};

    /// Error codes
    const EUnauthorized: u64 = 0;
    const EInvalidAmount: u64 = 1;
    const EInvalidParameter: u64 = 2;
    const ENotDelegate: u64 = 3;
    const EAlreadyDelegate: u64 = 4;
    const EProposalNotFound: u64 = 6;
    const EInvalidProposalStatus: u64 = 7;
    const EAlreadyVoted: u64 = 8;
    const ENotVotingPhase: u64 = 9;
    const EVotingPeriodNotEnded: u64 = 11;
    const EVotingPeriodEnded: u64 = 12;
    const EExceedsMaxVotes: u64 = 13;
    const EInvalidVoteCount: u64 = 14;
    const EInvalidRegistry: u64 = 15;
    const EAlreadyNominated: u64 = 16;
    const EWrongVersion: u64 = 17;
    const EDelegateAnonNotAllowed: u64 = 18;
    const EOverflow: u64 = 19;
    const ENoExistingVote: u64 = 20;
    const EWrongRegistryForTreasuryRoute: u64 = 21;

    /// Forfeiture reason (indexer) — same values as old refund reason codes.
    const FORFEIT_REASON_QUORUM_NOT_MET: u8 = 0;
    const FORFEIT_REASON_COMMUNITY_REJECTED: u8 = 1;
    const FORFEIT_REASON_DELEGATE_REJECTED: u8 = 2;
    /// For [`ProposalRewardPoolForfeitedToTreasuryEvent`]: ecosystem treasury.
    const TREASURY_ROUTE_ECOSYSTEM: u8 = 0;
    /// Platform governance (on-platform treasury, not the shared ecosystem treasury object).
    const TREASURY_ROUTE_PLATFORM: u8 = 1;

    /// Maximum u64 value for overflow protection
    const MAX_U64: u64 = 18446744073709551615;

    /// Proposal type constants
    const PROPOSAL_TYPE_ECOSYSTEM: u8 = 0;
    const PROPOSAL_TYPE_PROOF_OF_CREATIVITY: u8 = 1;
    const PROPOSAL_TYPE_SPOT: u8 = 2;
    const PROPOSAL_TYPE_PLATFORM: u8 = 3;

    /// Proposal status constants
    const STATUS_SUBMITTED: u8 = 0;
    const STATUS_DELEGATE_REVIEW: u8 = 1;
    const STATUS_COMMUNITY_VOTING: u8 = 2;
    const STATUS_APPROVED: u8 = 3;
    const STATUS_REJECTED: u8 = 4;
    const STATUS_IMPLEMENTED: u8 = 5;
    const STATUS_OWNER_RESCIND: u8 = 6;

    /// Field names for dynamic fields
    const VOTED_COMMUNITY_FIELD: vector<u8> = b"voted_community";
    const DELEGATE_VOTES_FIELD: vector<u8> = b"delegate_votes";
    const VOTED_DELEGATES_LIST_FIELD: vector<u8> = b"voted_delegates_list";
    const DELEGATE_REASONS_FIELD: vector<u8> = b"delegate_reasons";
    const ENCRYPTED_VOTES_FIELD: vector<u8> = b"encrypted_votes";
    const ANON_VOTERS_FIELD: vector<u8> = b"anon_voters";

    /// Admin capability for Governance system management
    public struct GovernanceAdminCap has key, store {
        id: UID,
    }

    /// Governance registry that keeps track of all delegates and proposals
    public struct GovernanceDAO has key {
        id: UID,
        /// Registry type identifier (ecosystem, proof of creativity, platform)
        registry_type: u8,
        /// Configuration parameters
        delegate_count: u64,
        delegate_term_epochs: u64,
        proposal_submission_cost: u64,
        max_votes_per_user: u64,
        quadratic_base_cost: u64,
        voting_period_ms: u64,  // Voting period duration in milliseconds
        quorum_votes: u64,  // Minimum number of votes required for a valid proposal outcome
        /// Tables and mappings
        delegates: Table<address, Delegate>,
        proposals: Table<ID, bool>,
        proposal_types: Table<ID, u8>,
        proposals_by_status: Table<u8, vector<ID>>,
        treasury: Balance<MYSO>, /// Treasury for proposal costs and rewards
        nominated_delegates: Table<address, NominatedDelegate>,
        delegate_addresses: VecSet<address>,
        nominee_addresses: VecSet<address>,
        voters: Table<address, Table<address, bool>>, // target -> (voter -> upvote)
        version: u64,
        /// Largest epoch boundary `k * delegate_term_epochs` for which a full panel election has run (`0` = none yet).
        last_delegate_panel_boundary_epoch: u64,
    }

    /// Delegate struct representing a member of the delegate council
    public struct Delegate has store {
        address: address,
        upvotes: u64,
        downvotes: u64,
        proposals_reviewed: u64,
        proposals_submitted: u64,
        sided_winning_proposals: u64,  // Number of proposals where delegate voted with the winning side
        sided_losing_proposals: u64,   // Number of proposals where delegate voted with the losing side
        term_start: u64,  // Epoch when term started
        term_end: u64,    // Epoch when term ends
    }

    /// Nominee struct representing a user nominated but not yet active
    public struct NominatedDelegate has store {
        address: address,
        scheduled_term_start_epoch: u64,
        upvotes: u64,
        downvotes: u64,
    }

    /// Proposal struct representing a governance proposal
    public struct Proposal has key {
        id: UID,
        title: String,
        description: String,
        proposal_type: u8,  // Ecosystem, Proof of Creativity, Platform
        reference_id: Option<ID>,  // Optional reference to content/post ID
        metadata_json: Option<String>, // Optional JSON metadata string
        submitter: address,
        submission_time: u64,
        delegate_approval_count: u64,
        delegate_rejection_count: u64,
        community_votes_for: u64,
        community_votes_against: u64,
        status: u8,  // Submitted, DelegateReview, CommunityVoting, Approved, Rejected, Implemented
        voting_start_time: u64,  // Voting start time in milliseconds
        voting_end_time: u64,    // Voting end time in milliseconds
        reward_pool: Balance<MYSO>,
    }

    /// Vote struct for tracking individual votes - unused fields removed
    public struct Vote has store, drop {}

    /// Event emitted when a new delegate is elected
    public struct DelegateElectedEvent has copy, drop {
        delegate_address: address,
        term_start: u64,
        term_end: u64,
        registry_type: u8,
        upvotes: u64,
        downvotes: u64,
    }

    /// Event emitted when a user is nominated to become a delegate in a future term
    public struct DelegateNominatedEvent has copy, drop {
        nominee_address: address,
        scheduled_term_start_epoch: u64,
        registry_type: u8,
    }

    /// Event emitted when a delegate or nominee is voted for/against
    public struct DelegateVotedEvent has copy, drop {
        target_address: address,
        voter: address,
        is_active_delegate: bool,
        upvote: bool,
        new_upvote_count: u64,
        new_downvote_count: u64,
        registry_type: u8,
    }

    /// Event emitted when a user clears their delegate/nominee up or down vote
    public struct DelegateVoteClearedEvent has copy, drop {
        target_address: address,
        voter: address,
        is_active_delegate: bool,
        new_upvote_count: u64,
        new_downvote_count: u64,
        registry_type: u8,
    }

    /// Event emitted when a proposal is submitted
    public struct ProposalSubmittedEvent has copy, drop {
        proposal_id: ID,
        title: String,
        description: String,
        proposal_type: u8,
        reference_id: Option<ID>,
        metadata_json: Option<String>,
        submitter: address,
        reward_amount: u64,
        submission_time: u64,
    }

    /// Event emitted when a delegate votes on a proposal
    public struct DelegateVoteEvent has copy, drop {
        proposal_id: ID,
        delegate_address: address,
        approve: bool,
        vote_time: u64,
        reason: Option<String>,
    }

    /// Event emitted when a community member votes on a proposal
    public struct CommunityVoteEvent has copy, drop {
        proposal_id: ID,
        voter: address,
        vote_weight: u64,
        approve: bool,
        vote_time: u64,
        vote_cost: u64,
    }

    /// Event emitted when an anonymous vote is submitted
    public struct AnonymousVoteEvent has copy, drop {
        proposal_id: ID,
        voter: address,
        vote_time: u64,
        encrypted_vote_data: vector<u8>, // Raw encrypted vote data for indexer storage
    }

    /// Event emitted when a proposal is approved by the delegate council
    public struct ProposalApprovedForVotingEvent has copy, drop {
        proposal_id: ID,
        voting_start_time: u64,
        voting_end_time: u64,
    }

    /// Event emitted when a proposal is rejected by the delegate council
    public struct ProposalRejectedEvent has copy, drop {
        proposal_id: ID,
        rejection_time: u64,
    }

    /// Event emitted when a proposal is approved by the community
    public struct ProposalApprovedEvent has copy, drop {
        proposal_id: ID,
        approval_time: u64,
        votes_for: u64,
        votes_against: u64,
    }

    /// Event emitted when a proposal is rejected by the community
    public struct ProposalRejectedByCommunityEvent has copy, drop {
        proposal_id: ID,
        rejection_time: u64,
        votes_for: u64,
        votes_against: u64,
    }

    /// Event emitted when a proposal is implemented
    public struct ProposalImplementedEvent has copy, drop {
        proposal_id: ID,
        implementation_time: u64,
        description: Option<String>,
    }

    /// Failed proposal pool sent to a treasury (ecosystem or platform governance) instead of the submitter.
    public struct ProposalRewardPoolForfeitedToTreasuryEvent has copy, drop {
        proposal_id: ID,
        recipient: address,
        amount: u64,
        reason: u8,
        registry_type: u8,
        treasury_route: u8,
        forfeited_time: u64,
    }

    /// Approved proposal: reward pool sent to the submitter when marked implemented.
    public struct ProposalImplementationRewardToSubmitterEvent has copy, drop {
        proposal_id: ID,
        submitter: address,
        amount: u64,
        registry_type: u8,
        sent_time: u64,
    }

    /// Event emitted when vote decryption fails
    public struct VoteDecryptionFailedEvent has copy, drop {
        proposal_id: ID,
        voter: address,
        failure_reason: String,
        timestamp: u64,
    }

    /// Event emitted when a proposal is rescinded by its submitter
    public struct ProposalRescindedEvent has copy, drop {
        proposal_id: ID,
        submitter: address,
        rescind_time: u64,
        refund_amount: u64,
    }

    /// Event emitted when governance parameters are updated
    public struct GovernanceParametersUpdatedEvent has copy, drop {
        registry_type: u8,
        updated_by: address,
        delegate_count: u64,
        delegate_term_epochs: u64,
        proposal_submission_cost: u64,
        max_votes_per_user: u64,
        quadratic_base_cost: u64,
        voting_period_ms: u64,
        quorum_votes: u64,
        timestamp: u64,
    }

    /// Event emitted when a governance registry is created
    /// This event matches the GovernanceRegistryEvent structure expected by the indexer
    public struct GovernanceRegistryCreatedEvent has copy, drop {
        registry_id: ID,
        registry_type: u8,
        delegate_count: u64,
        delegate_term_epochs: u64,
        proposal_submission_cost: u64,
        max_votes_per_user: u64,
        quadratic_base_cost: u64,
        voting_period_ms: u64,
        quorum_votes: u64,
        updated_at: u64, // Using updated_at to match indexer structure (represents creation time)
    }

    /// Event emitted when a delegate panel refresh applies a completed term boundary
    /// (`last_delegate_panel_boundary_epoch` on the DAO).
    public struct DelegatePanelRefreshedEvent has copy, drop {
        registry_id: ID,
        boundary_epoch: u64,
        registry_type: u8,
        executed_at_epoch: u64,
    }

    /// PoC + SPoT governance registry IDs returned from bootstrap for wiring into module configs.
    public struct BootstrapGovernanceRegistryIds has copy, drop {
        poc_governance_registry_id: ID,
        spot_governance_registry_id: ID,
    }

    public fun poc_governance_registry_id(ids: &BootstrapGovernanceRegistryIds): ID {
        ids.poc_governance_registry_id
    }

    public fun spot_governance_registry_id(ids: &BootstrapGovernanceRegistryIds): ID {
        ids.spot_governance_registry_id
    }

    /// Bootstrap initialization function - creates the governance registries.
    /// Returns PoC and SPoT registry IDs for wiring into [`proof_of_creativity`] and [`social_proof_of_truth`].
    public(package) fun bootstrap_init(clock: &Clock, ctx: &mut TxContext): BootstrapGovernanceRegistryIds {
        let current_time = clock::timestamp_ms(clock);
        let founder = tx_context::sender(ctx);

        // Create MySocial Ecosystem Governance Registry
        let mut ecosystem_registry = GovernanceDAO {
            id: object::new(ctx),
            registry_type: PROPOSAL_TYPE_ECOSYSTEM,
            // Configuration parameters specific to ecosystem governance
            delegate_count: 3, // Larger council for ecosystem decisions
            delegate_term_epochs: 90, // 3 months for ecosystem delegates
            proposal_submission_cost: 100_000_000_000, // 100 MYSO for ecosystem proposals
            max_votes_per_user: 10, // Up to 10 votes per user
            quadratic_base_cost: 10_000_000_000, // 10 MYSO per additional vote
            voting_period_ms: 7 * 24 * 60 * 60 * 1000, // 7 days in milliseconds for ecosystem votes
            quorum_votes: 20, // 20 votes required for ecosystem proposals
            // Tables
            delegates: table::new<address, Delegate>(ctx),
            proposals: table::new<ID, bool>(ctx),
            proposal_types: table::new<ID, u8>(ctx),
            proposals_by_status: table::new<u8, vector<ID>>(ctx),
            treasury: balance::zero(),
            nominated_delegates: table::new<address, NominatedDelegate>(ctx),
            delegate_addresses: vec_set::empty<address>(),
            nominee_addresses: vec_set::empty<address>(),
            voters: table::new<address, Table<address, bool>>(ctx),
            version: upgrade::current_version(),
            last_delegate_panel_boundary_epoch: 0,
        };
        
        // Initialize ecosystem registry's status tables
        initialize_registry_tables(&mut ecosystem_registry, ctx);
        
        // Get ecosystem registry ID before sharing
        let ecosystem_registry_id = object::id(&ecosystem_registry);
        
        // Emit event for ecosystem registry creation
        event::emit(GovernanceRegistryCreatedEvent {
            registry_id: ecosystem_registry_id,
            registry_type: PROPOSAL_TYPE_ECOSYSTEM,
            delegate_count: ecosystem_registry.delegate_count,
            delegate_term_epochs: ecosystem_registry.delegate_term_epochs,
            proposal_submission_cost: ecosystem_registry.proposal_submission_cost,
            max_votes_per_user: ecosystem_registry.max_votes_per_user,
            quadratic_base_cost: ecosystem_registry.quadratic_base_cost,
            voting_period_ms: ecosystem_registry.voting_period_ms,
            quorum_votes: ecosystem_registry.quorum_votes,
            updated_at: current_time,
        });

        seed_founding_delegate(&mut ecosystem_registry, founder, ctx);
        
        // Share the ecosystem registry object
        transfer::share_object(ecosystem_registry);
        
        // Create Proof of Creativity Governance Registry
        let mut proof_of_creativity_registry = GovernanceDAO {
            id: object::new(ctx),
            registry_type: PROPOSAL_TYPE_PROOF_OF_CREATIVITY,
            // Configuration parameters specific to proof of creativity governance
            delegate_count: 2, // Smaller council for proof of creativity
            delegate_term_epochs: 180, // 3 months for proof of creativity delegates
            proposal_submission_cost: 25_000_000_000, // 25 MYSO for proof of creativity
            max_votes_per_user: 3, // Up to 3 votes per user
            quadratic_base_cost: 2_500_000_000, // 2.5 MYSO per additional vote
            voting_period_ms: 24 * 60 * 60 * 1000, // 1 day in milliseconds for proof of creativity votes
            quorum_votes: 10, // 10 votes required for proof of creativity proposals
            // Tables
            delegates: table::new<address, Delegate>(ctx),
            proposals: table::new<ID, bool>(ctx),
            proposal_types: table::new<ID, u8>(ctx),
            proposals_by_status: table::new<u8, vector<ID>>(ctx),
            treasury: balance::zero(),
            nominated_delegates: table::new<address, NominatedDelegate>(ctx),
            delegate_addresses: vec_set::empty<address>(),
            nominee_addresses: vec_set::empty<address>(),
            voters: table::new<address, Table<address, bool>>(ctx),
            version: upgrade::current_version(),
            last_delegate_panel_boundary_epoch: 0,
        };
        
        // Initialize proof of creativity registry's status tables
        initialize_registry_tables(&mut proof_of_creativity_registry, ctx);
        
        // Get proof of creativity registry ID before sharing
        let proof_of_creativity_registry_id = object::id(&proof_of_creativity_registry);
        
        // Emit event for proof of creativity registry creation
        event::emit(GovernanceRegistryCreatedEvent {
            registry_id: proof_of_creativity_registry_id,
            registry_type: PROPOSAL_TYPE_PROOF_OF_CREATIVITY,
            delegate_count: proof_of_creativity_registry.delegate_count,
            delegate_term_epochs: proof_of_creativity_registry.delegate_term_epochs,
            proposal_submission_cost: proof_of_creativity_registry.proposal_submission_cost,
            max_votes_per_user: proof_of_creativity_registry.max_votes_per_user,
            quadratic_base_cost: proof_of_creativity_registry.quadratic_base_cost,
            voting_period_ms: proof_of_creativity_registry.voting_period_ms,
            quorum_votes: proof_of_creativity_registry.quorum_votes,
            updated_at: current_time,
        });

        seed_founding_delegate(&mut proof_of_creativity_registry, founder, ctx);
        
        // Share the proof of creativity registry object
        transfer::share_object(proof_of_creativity_registry);

        // Create Social Proof of Truth Governance Registry
        let mut spot_registry = GovernanceDAO {
            id: object::new(ctx),
            registry_type: PROPOSAL_TYPE_SPOT,
            delegate_count: 3,
            delegate_term_epochs: 90,
            proposal_submission_cost: 10_000_000_000,
            max_votes_per_user: 5,
            quadratic_base_cost: 1_000_000_000,
            voting_period_ms: 3 * 24 * 60 * 60 * 1000,
            quorum_votes: 10,
            delegates: table::new<address, Delegate>(ctx),
            proposals: table::new<ID, bool>(ctx),
            proposal_types: table::new<ID, u8>(ctx),
            proposals_by_status: table::new<u8, vector<ID>>(ctx),
            treasury: balance::zero(),
            nominated_delegates: table::new<address, NominatedDelegate>(ctx),
            delegate_addresses: vec_set::empty<address>(),
            nominee_addresses: vec_set::empty<address>(),
            voters: table::new<address, Table<address, bool>>(ctx),
            version: upgrade::current_version(),
            last_delegate_panel_boundary_epoch: 0,
        };

        initialize_registry_tables(&mut spot_registry, ctx);

        let spot_registry_id = object::id(&spot_registry);

        event::emit(GovernanceRegistryCreatedEvent {
            registry_id: spot_registry_id,
            registry_type: PROPOSAL_TYPE_SPOT,
            delegate_count: spot_registry.delegate_count,
            delegate_term_epochs: spot_registry.delegate_term_epochs,
            proposal_submission_cost: spot_registry.proposal_submission_cost,
            max_votes_per_user: spot_registry.max_votes_per_user,
            quadratic_base_cost: spot_registry.quadratic_base_cost,
            voting_period_ms: spot_registry.voting_period_ms,
            quorum_votes: spot_registry.quorum_votes,
            updated_at: current_time,
        });

        seed_founding_delegate(&mut spot_registry, founder, ctx);
        transfer::share_object(spot_registry);

        BootstrapGovernanceRegistryIds {
            poc_governance_registry_id: proof_of_creativity_registry_id,
            spot_governance_registry_id: spot_registry_id,
        }
    }

    /// Install the founding delegate without going through nomination (bootstrap / platform creation).
    fun seed_founding_delegate(registry: &mut GovernanceDAO, addr: address, ctx: &TxContext) {
        let current_epoch = tx_context::epoch(ctx);
        let delegate_term_epochs = registry.delegate_term_epochs;
        let term_start = current_epoch;
        let term_end = term_start + delegate_term_epochs;

        let new_delegate = Delegate {
            address: addr,
            upvotes: 0,
            downvotes: 0,
            proposals_reviewed: 0,
            proposals_submitted: 0,
            sided_winning_proposals: 0,
            sided_losing_proposals: 0,
            term_start,
            term_end,
        };
        table::add(&mut registry.delegates, addr, new_delegate);
        vec_set::insert(&mut registry.delegate_addresses, addr);

        event::emit(DelegateElectedEvent {
            delegate_address: addr,
            term_start,
            term_end,
            registry_type: registry.registry_type,
            upvotes: 0,
            downvotes: 0,
        });
    }

    fun initialize_registry_tables(registry: &mut GovernanceDAO, _ctx: &mut TxContext) {
        table::add(&mut registry.proposals_by_status, STATUS_SUBMITTED, vector::empty<ID>());
        table::add(&mut registry.proposals_by_status, STATUS_DELEGATE_REVIEW, vector::empty<ID>());
        table::add(&mut registry.proposals_by_status, STATUS_COMMUNITY_VOTING, vector::empty<ID>());
        table::add(&mut registry.proposals_by_status, STATUS_APPROVED, vector::empty<ID>());
        table::add(&mut registry.proposals_by_status, STATUS_REJECTED, vector::empty<ID>());
        table::add(&mut registry.proposals_by_status, STATUS_IMPLEMENTED, vector::empty<ID>());
        table::add(&mut registry.proposals_by_status, STATUS_OWNER_RESCIND, vector::empty<ID>());
    }

    /// Update governance parameters (internal function)
    /// This function does not perform authorization checks - callers must verify permissions
    fun update_governance_parameters_internal(
        registry: &mut GovernanceDAO,
        delegate_count: u64,
        delegate_term_epochs: u64,
        proposal_submission_cost: u64,
        max_votes_per_user: u64,
        quadratic_base_cost: u64,
        voting_period_ms: u64,
        quorum_votes: u64,
        clock: &Clock,
        _ctx: &mut TxContext
    ) {
        // Check version compatibility
        assert!(registry.version == upgrade::current_version(), EWrongVersion);
        
        // Ensure parameters are sensible
        assert!(delegate_count > 1, EInvalidParameter);
        assert!(delegate_term_epochs > 0, EInvalidParameter);
        // proposal_submission_cost can be 0
        assert!(max_votes_per_user > 0, EInvalidParameter);
        // quadratic_base_cost can be 0 (if voting is free)
        assert!(voting_period_ms > 0, EInvalidParameter);
        assert!(quorum_votes > 0, EInvalidParameter);
        
        // Update parameters
        registry.delegate_count = delegate_count;
        registry.delegate_term_epochs = delegate_term_epochs;
        registry.proposal_submission_cost = proposal_submission_cost;
        registry.max_votes_per_user = max_votes_per_user;
        registry.quadratic_base_cost = quadratic_base_cost;
        registry.voting_period_ms = voting_period_ms;
        registry.quorum_votes = quorum_votes;
        
        // Emit governance parameters updated event
        event::emit(GovernanceParametersUpdatedEvent {
            registry_type: registry.registry_type,
            updated_by: tx_context::sender(_ctx),
            delegate_count,
            delegate_term_epochs,
            proposal_submission_cost,
            max_votes_per_user,
            quadratic_base_cost,
            voting_period_ms,
            quorum_votes,
            timestamp: clock::timestamp_ms(clock),
        });
    }

    /// Update governance parameters for platform registries
    /// Can only be called by the platform module (which verifies platform ownership)
    /// This function is package-private to prevent direct calls that bypass platform ownership verification
    public(package) fun update_platform_governance_parameters(
        registry: &mut GovernanceDAO,
        platform_developer: address,
        delegate_count: u64,
        delegate_term_epochs: u64,
        proposal_submission_cost: u64,
        max_votes_per_user: u64,
        quadratic_base_cost: u64,
        voting_period_ms: u64,
        quorum_votes: u64,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        // Verify this is a platform registry
        assert!(registry.registry_type == PROPOSAL_TYPE_PLATFORM, EInvalidRegistry);
        
        // Verify caller is platform developer
        let caller = tx_context::sender(ctx);
        assert!(platform_developer == caller, EUnauthorized);
        
        // Call the internal update function
        update_governance_parameters_internal(
            registry,
            delegate_count,
            delegate_term_epochs,
            proposal_submission_cost,
            max_votes_per_user,
            quadratic_base_cost,
            voting_period_ms,
            quorum_votes,
            clock,
            ctx
        );
    }

    /// Update governance parameters for ecosystem/proof-of-creativity registries
    /// Can only be called by governance admin
    public entry fun update_governance_parameters(
        registry: &mut GovernanceDAO,
        _: &GovernanceAdminCap,
        delegate_count: u64,
        delegate_term_epochs: u64,
        proposal_submission_cost: u64,
        max_votes_per_user: u64,
        quadratic_base_cost: u64,
        voting_period_ms: u64,
        quorum_votes: u64,
        clock: &Clock,
        _ctx: &mut TxContext
    ) {
        // Verify this is NOT a platform registry
        assert!(registry.registry_type != PROPOSAL_TYPE_PLATFORM, EInvalidRegistry);
        
        // Call the internal update function
        update_governance_parameters_internal(
            registry,
            delegate_count,
            delegate_term_epochs,
            proposal_submission_cost,
            max_votes_per_user,
            quadratic_base_cost,
            voting_period_ms,
            quorum_votes,
            clock,
            _ctx
        );
    }

    /// Nominate self as a delegate
    /// Uses wallet-level architecture - no profile required
    public entry fun nominate_delegate(
        registry: &mut GovernanceDAO,
        ctx: &mut TxContext
    ) {
        // Check version compatibility
        assert!(registry.version == upgrade::current_version(), EWrongVersion);
        
        let caller = tx_context::sender(ctx);
        let current_epoch = tx_context::epoch(ctx);
        
        // Check if already a delegate or nominee delegate
        assert!(!table::contains(&registry.delegates, caller), EAlreadyDelegate);
        assert!(!table::contains(&registry.nominated_delegates, caller), EAlreadyNominated);

        let scheduled_term_start_epoch = ((current_epoch / registry.delegate_term_epochs) + 1) * registry.delegate_term_epochs;

        let nominated_delegate = NominatedDelegate {
            address: caller,
            scheduled_term_start_epoch,
            upvotes: 0,
            downvotes: 0,
        };
        table::add(&mut registry.nominated_delegates, caller, nominated_delegate);
        vec_set::insert(&mut registry.nominee_addresses, caller);

        // Initialize voter tracking table for this nominee
        table::add(&mut registry.voters, caller, table::new<address, bool>(ctx));

        event::emit(DelegateNominatedEvent {
            nominee_address: caller,
            scheduled_term_start_epoch,
            registry_type: registry.registry_type,
        });

        try_update_delegate_panel_if_due(registry, ctx);
    }

    /// Vote for or against a delegate or nominee delegate
    /// Positive votes support the delegate, negative votes express disapproval
    /// Users can change their vote at any time
    public entry fun vote_for_delegate(
        registry: &mut GovernanceDAO,
        target_address: address,
        upvote: bool,
        ctx: &mut TxContext
    ) {
        // Check version compatibility
        assert!(registry.version == upgrade::current_version(), EWrongVersion);
        
        let caller = tx_context::sender(ctx);
        
        // Don't allow self-voting
        assert!(caller != target_address, EUnauthorized);
        
        // Variables for event emission
        let is_active_delegate: bool;
        let upvote_count: u64;
        let downvote_count: u64;
        
        // Handle voting for active delegates
        if (table::contains(&registry.delegates, target_address)) {
            is_active_delegate = true;
            let delegate = table::borrow_mut(&mut registry.delegates, target_address);
            
            // Create voter table for this target if it doesn't exist
            if (!table::contains(&registry.voters, target_address)) {
                table::add(&mut registry.voters, target_address, table::new<address, bool>(ctx));
            };
            
            let voter_table = table::borrow_mut(&mut registry.voters, target_address);
            
            // Check if user has already voted
            if (table::contains(voter_table, caller)) {
                let previous_vote = *table::borrow(voter_table, caller);
                
                // If same vote, do nothing
                if (previous_vote == upvote) {
                    try_update_delegate_panel_if_due(registry, ctx);
                    return
                };
                
                // Different vote - remove previous vote with underflow protection
                if (previous_vote) {
                    assert!(delegate.upvotes > 0, EOverflow);
                    delegate.upvotes = delegate.upvotes - 1;
                } else {
                    assert!(delegate.downvotes > 0, EOverflow);
                    delegate.downvotes = delegate.downvotes - 1;
                };
                
                // Add new vote
                if (upvote) {
                    assert!(delegate.upvotes <= MAX_U64 - 1, EOverflow);
                    delegate.upvotes = delegate.upvotes + 1;
                } else {
                    assert!(delegate.downvotes <= MAX_U64 - 1, EOverflow);
                    delegate.downvotes = delegate.downvotes + 1;
                };
                
                // Update record
                *table::borrow_mut(voter_table, caller) = upvote;
            } else {
                // First time voting for this target
                if (upvote) {
                    assert!(delegate.upvotes <= MAX_U64 - 1, EOverflow);
                    delegate.upvotes = delegate.upvotes + 1;
                } else {
                    assert!(delegate.downvotes <= MAX_U64 - 1, EOverflow);
                    delegate.downvotes = delegate.downvotes + 1;
                };
                
                // Record vote
                table::add(voter_table, caller, upvote);
            };
            
            upvote_count = delegate.upvotes;
            downvote_count = delegate.downvotes;
        }
        // Handle voting for nominee delegates
        else if (table::contains(&registry.nominated_delegates, target_address)) {
            is_active_delegate = false;
            let nominee = table::borrow_mut(&mut registry.nominated_delegates, target_address);
            
            // Create voter table for this target if it doesn't exist
            if (!table::contains(&registry.voters, target_address)) {
                table::add(&mut registry.voters, target_address, table::new<address, bool>(ctx));
            };
            
            let voter_table = table::borrow_mut(&mut registry.voters, target_address);
            
            // Check if user has already voted
            if (table::contains(voter_table, caller)) {
                let previous_vote = *table::borrow(voter_table, caller);
                
                // If same vote, do nothing
                if (previous_vote == upvote) {
                    try_update_delegate_panel_if_due(registry, ctx);
                    return
                };
                
                // Different vote - remove previous vote with underflow protection
                if (previous_vote) {
                    assert!(nominee.upvotes > 0, EOverflow);
                    nominee.upvotes = nominee.upvotes - 1;
                } else {
                    assert!(nominee.downvotes > 0, EOverflow);
                    nominee.downvotes = nominee.downvotes - 1;
                };
                
                // Add new vote
                if (upvote) {
                    assert!(nominee.upvotes <= MAX_U64 - 1, EOverflow);
                    nominee.upvotes = nominee.upvotes + 1;
                } else {
                    assert!(nominee.downvotes <= MAX_U64 - 1, EOverflow);
                    nominee.downvotes = nominee.downvotes + 1;
                };
                
                // Update record
                *table::borrow_mut(voter_table, caller) = upvote;
            } else {
                // First time voting for this nominee
                if (upvote) {
                    assert!(nominee.upvotes <= MAX_U64 - 1, EOverflow);
                    nominee.upvotes = nominee.upvotes + 1;
                } else {
                    assert!(nominee.downvotes <= MAX_U64 - 1, EOverflow);
                    nominee.downvotes = nominee.downvotes + 1;
                };
                
                // Record vote
                table::add(voter_table, caller, upvote);
            };
            
            upvote_count = nominee.upvotes;
            downvote_count = nominee.downvotes;
        }
        else { abort ENotDelegate };
        
        event::emit(DelegateVotedEvent {
            target_address,
            voter: caller,
            is_active_delegate,
            upvote,
            new_upvote_count: upvote_count,
            new_downvote_count: downvote_count,
            registry_type: registry.registry_type,
        });

        try_update_delegate_panel_if_due(registry, ctx);
    }

    /// Remove the caller's up or down vote on a delegate or nominee (neutral / undo).
    /// Aborts with `ENoExistingVote` if the caller has not voted on this target.
    public entry fun clear_vote_for_delegate(
        registry: &mut GovernanceDAO,
        target_address: address,
        ctx: &mut TxContext
    ) {
        assert!(registry.version == upgrade::current_version(), EWrongVersion);

        let caller = tx_context::sender(ctx);
        assert!(caller != target_address, EUnauthorized);

        let is_active_delegate: bool;
        let upvote_count: u64;
        let downvote_count: u64;

        if (table::contains(&registry.delegates, target_address)) {
            is_active_delegate = true;
            assert!(table::contains(&registry.voters, target_address), ENoExistingVote);
            let voter_table = table::borrow_mut(&mut registry.voters, target_address);
            assert!(table::contains(voter_table, caller), ENoExistingVote);

            let previous_vote = *table::borrow(voter_table, caller);
            let delegate = table::borrow_mut(&mut registry.delegates, target_address);
            if (previous_vote) {
                assert!(delegate.upvotes > 0, EOverflow);
                delegate.upvotes = delegate.upvotes - 1;
            } else {
                assert!(delegate.downvotes > 0, EOverflow);
                delegate.downvotes = delegate.downvotes - 1;
            };
            table::remove(voter_table, caller);

            upvote_count = delegate.upvotes;
            downvote_count = delegate.downvotes;
        } else if (table::contains(&registry.nominated_delegates, target_address)) {
            is_active_delegate = false;
            assert!(table::contains(&registry.voters, target_address), ENoExistingVote);
            let voter_table = table::borrow_mut(&mut registry.voters, target_address);
            assert!(table::contains(voter_table, caller), ENoExistingVote);

            let previous_vote = *table::borrow(voter_table, caller);
            let nominee = table::borrow_mut(&mut registry.nominated_delegates, target_address);
            if (previous_vote) {
                assert!(nominee.upvotes > 0, EOverflow);
                nominee.upvotes = nominee.upvotes - 1;
            } else {
                assert!(nominee.downvotes > 0, EOverflow);
                nominee.downvotes = nominee.downvotes - 1;
            };
            table::remove(voter_table, caller);

            upvote_count = nominee.upvotes;
            downvote_count = nominee.downvotes;
        } else {
            abort ENotDelegate
        };

        event::emit(DelegateVoteClearedEvent {
            target_address,
            voter: caller,
            is_active_delegate,
            new_upvote_count: upvote_count,
            new_downvote_count: downvote_count,
            registry_type: registry.registry_type,
        });

        try_update_delegate_panel_if_due(registry, ctx);
    }

    /// When `ctx.epoch()` has reached a completed term boundary that has not yet been applied,
    /// recomputes the delegate council from incumbents and nominees. Otherwise returns immediately.
    fun try_update_delegate_panel_if_due(
        registry: &mut GovernanceDAO,
        ctx: &TxContext
    ) {
        // Check version compatibility
        assert!(registry.version == upgrade::current_version(), EWrongVersion);
        
        let current_epoch = tx_context::epoch(ctx);
        let delegate_term_epochs = registry.delegate_term_epochs;
        assert!(delegate_term_epochs > 0, EInvalidParameter);

        if (current_epoch == 0) {
            return
        };

        let latest_completed_boundary =
            (current_epoch / delegate_term_epochs) * delegate_term_epochs;
        if (latest_completed_boundary == 0) {
            return
        };
        if (registry.last_delegate_panel_boundary_epoch >= latest_completed_boundary) {
            return
        };

        // --- Gather Candidates Data --- 
        let mut candidate_addresses = vector::empty<address>();
        let mut candidate_upvotes = vector::empty<u64>();
        let mut candidate_downvotes = vector::empty<u64>();
        let mut candidate_net_votes = vector::empty<u64>(); // For sorting calculations
        let mut candidate_is_incumbent = vector::empty<bool>(); // Track incumbent status for tie-breaking

        // Use 1000 as the baseline score where upvotes and downvotes are equal
        let baseline_score: u64 = 1000;

        // 1. Gather Active Delegates (using the tracking set)
        let active_delegate_keys_vec = vec_set::into_keys(registry.delegate_addresses); // Consumes the set
        let len_active = vector::length(&active_delegate_keys_vec);
        
        let mut i = 0;
        while (i < len_active) {
            let addr = *vector::borrow(&active_delegate_keys_vec, i);
            // Need to borrow from the actual table to get full data
            if (table::contains(&registry.delegates, addr)) { // Check existence before borrow
                let delegate: &Delegate = table::borrow(&registry.delegates, addr);
                vector::push_back(&mut candidate_addresses, addr);
                vector::push_back(&mut candidate_upvotes, delegate.upvotes);
                vector::push_back(&mut candidate_downvotes, delegate.downvotes);
                vector::push_back(&mut candidate_is_incumbent, true); // Mark as incumbent
                
                // Calculate net votes with baseline offset (allows negative effective scores)
                let net_votes = if (delegate.upvotes >= delegate.downvotes) {
                    // If more upvotes than downvotes, add the difference to baseline
                    baseline_score + (delegate.upvotes - delegate.downvotes)
                } else {
                    // If more downvotes than upvotes, subtract the difference from baseline
                    // But make sure we don't underflow
                    if (delegate.downvotes - delegate.upvotes >= baseline_score) {
                        0 // Minimum score
                    } else {
                        baseline_score - (delegate.downvotes - delegate.upvotes)
                    }
                };
                
                vector::push_back(&mut candidate_net_votes, net_votes);
            };
            // Note: If delegate wasn't found in table (somehow desynced), it's skipped.
            i = i + 1;
        };
        // Recreate the delegate_addresses set as empty, it will be repopulated with winners
        registry.delegate_addresses = vec_set::empty<address>();

        // 2. Gather Nominated Delegates (using the tracking set)
        let nominee_keys_vec = vec_set::into_keys(registry.nominee_addresses); // Consumes the set
        let len_nominated = vector::length(&nominee_keys_vec);
        
        let mut j = 0;
        while (j < len_nominated) {
            let addr = *vector::borrow(&nominee_keys_vec, j);
            // Check if it was already added as an active delegate (shouldn't happen if sets are correct)
            // We can skip this check if we ensure sets are managed correctly.
            // Need to borrow from the actual table to get full data
            if (table::contains(&registry.nominated_delegates, addr)) { // Check existence
                let nominee: &NominatedDelegate = table::borrow(&registry.nominated_delegates, addr);
                vector::push_back(&mut candidate_addresses, addr);
                vector::push_back(&mut candidate_upvotes, nominee.upvotes);
                vector::push_back(&mut candidate_downvotes, nominee.downvotes);
                vector::push_back(&mut candidate_is_incumbent, false); // Mark as non-incumbent
                
                // Calculate net votes with baseline offset (allows negative effective scores)
                let net_votes = if (nominee.upvotes >= nominee.downvotes) {
                    // If more upvotes than downvotes, add the difference to baseline
                    baseline_score + (nominee.upvotes - nominee.downvotes)
                } else {
                    // If more downvotes than upvotes, subtract the difference from baseline
                    // But make sure we don't underflow
                    if (nominee.downvotes - nominee.upvotes >= baseline_score) {
                        0 // Minimum score
                    } else {
                        baseline_score - (nominee.downvotes - nominee.upvotes)
                    }
                };
                
                vector::push_back(&mut candidate_net_votes, net_votes);
            };
            j = j + 1;
        };
        // Recreate the nominee_addresses set as empty, losers will not be re-added.
        registry.nominee_addresses = vec_set::empty<address>();

        // --- Sort Candidate Data by net votes (higher is better) --- 
        let candidate_count = vector::length(&candidate_addresses);
        let mut a = 0;
        while (a < candidate_count) {
            let mut b = a + 1;
            while (b < candidate_count) {
                // Compare net votes
                let a_votes = *vector::borrow(&candidate_net_votes, a);
                let b_votes = *vector::borrow(&candidate_net_votes, b);
                
                // If b has more net votes than a, or they're tied but b is an incumbent and a is not, swap them
                let should_swap = if (b_votes > a_votes) {
                    true
                } else if (b_votes == a_votes) {
                    // Tie-breaking: prefer incumbent delegates
                    let a_incumbent = *vector::borrow(&candidate_is_incumbent, a);
                    let b_incumbent = *vector::borrow(&candidate_is_incumbent, b);
                    
                    // Only swap if b is incumbent and a is not
                    b_incumbent && !a_incumbent
                } else {
                    false
                };
                
                if (should_swap) {
                    // Swap addresses
                    let temp_addr = *vector::borrow(&candidate_addresses, a);
                    *vector::borrow_mut(&mut candidate_addresses, a) = *vector::borrow(&candidate_addresses, b);
                    *vector::borrow_mut(&mut candidate_addresses, b) = temp_addr;
                    
                    // Swap upvotes
                    let temp_up = *vector::borrow(&candidate_upvotes, a);
                    *vector::borrow_mut(&mut candidate_upvotes, a) = *vector::borrow(&candidate_upvotes, b);
                    *vector::borrow_mut(&mut candidate_upvotes, b) = temp_up;
                    
                    // Swap downvotes
                    let temp_down = *vector::borrow(&candidate_downvotes, a);
                    *vector::borrow_mut(&mut candidate_downvotes, a) = *vector::borrow(&candidate_downvotes, b);
                    *vector::borrow_mut(&mut candidate_downvotes, b) = temp_down;
                    
                    // Swap net votes
                    let temp_net = *vector::borrow(&candidate_net_votes, a);
                    *vector::borrow_mut(&mut candidate_net_votes, a) = *vector::borrow(&candidate_net_votes, b);
                    *vector::borrow_mut(&mut candidate_net_votes, b) = temp_net;
                    
                    // Swap incumbent status
                    let temp_inc = *vector::borrow(&candidate_is_incumbent, a);
                    *vector::borrow_mut(&mut candidate_is_incumbent, a) = *vector::borrow(&candidate_is_incumbent, b);
                    *vector::borrow_mut(&mut candidate_is_incumbent, b) = temp_inc;
                };
                b = b + 1;
            };
            a = a + 1;
        };

        // --- Select Winners and Update State --- 
        let num_winners_to_select = registry.delegate_count;
        let actual_candidate_count = vector::length(&candidate_addresses);
        let final_winner_count = if (actual_candidate_count < num_winners_to_select) { actual_candidate_count } else { num_winners_to_select };

        // --- Cleanup Old State --- 
        // 1. Remove *all* previously active delegates from the table
        // We iterate using the keys vector we got earlier
        let mut k = 0;
        while (k < len_active) {
            let addr = *vector::borrow(&active_delegate_keys_vec, k);
            // Check if it exists before removing (might have been removed if somehow duplicated)
            if (table::contains(&registry.delegates, addr)) {
                 let old_delegate = table::remove(&mut registry.delegates, addr);
                 let Delegate { address: _, upvotes: _, downvotes: _, proposals_reviewed: _, proposals_submitted: _, sided_winning_proposals: _, sided_losing_proposals: _, term_start: _, term_end: _ } = old_delegate;
            };
            k = k + 1;
        };

        // 2. Remove *all* previously nominated delegates from the table
        let mut l = 0;
        while (l < len_nominated) {
            let addr = *vector::borrow(&nominee_keys_vec, l);
            if (table::contains(&registry.nominated_delegates, addr)) { 
                let old_nominee = table::remove(&mut registry.nominated_delegates, addr);
                let NominatedDelegate { address: _, scheduled_term_start_epoch: _, upvotes: _, downvotes: _ } = old_nominee;
             };
             l = l + 1;
        };

        // --- Add Winners to the Delegates Table --- 
        // The delegates table should now be empty. Add the winners.
        let mut m = 0;
        while (m < final_winner_count) {
            let winner_addr = *vector::borrow(&candidate_addresses, m);
            let winner_upvotes = *vector::borrow(&candidate_upvotes, m);
            let winner_downvotes = *vector::borrow(&candidate_downvotes, m);
            
            let term_start = current_epoch;
            let term_end = term_start + delegate_term_epochs;

            let new_delegate = Delegate {
                address: winner_addr,
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
            table::add(&mut registry.delegates, winner_addr, new_delegate);
            // Add winner's address back to the tracking set
            vec_set::insert(&mut registry.delegate_addresses, winner_addr);

            event::emit(DelegateElectedEvent {
                delegate_address: winner_addr,
                term_start: term_start,
                term_end: term_end,
                registry_type: registry.registry_type,
                upvotes: winner_upvotes,
                downvotes: winner_downvotes,
            });
            m = m + 1;
        };

        registry.last_delegate_panel_boundary_epoch = latest_completed_boundary;

        event::emit(DelegatePanelRefreshedEvent {
            registry_id: object::id(registry),
            boundary_epoch: latest_completed_boundary,
            registry_type: registry.registry_type,
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

    /// Updates delegate panel at the end of a delegate term cycle (keeper / manual backstop).
    public entry fun update_delegate_panel(
        registry: &mut GovernanceDAO,
        ctx: &mut TxContext
    ) {
        try_update_delegate_panel_if_due(registry, ctx);
    }

    /// Universal function to submit any type of proposal
    /// Handles proposal types: ecosystem and proof of creativity
    public entry fun submit_proposal(
        registry: &mut GovernanceDAO,
        proposal_type: u8,
        title: String,
        description: String,
        disputed_id: Option<ID>, // Optional ID for disputes (content only)
        reference_id: Option<ID>, // Optional reference
        metadata_json: Option<String>,
        coin: &mut Coin<MYSO>,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        // Check version compatibility
        assert!(registry.version == upgrade::current_version(), EWrongVersion);
        
        // Verify proposal type is valid
        assert!(proposal_type <= PROPOSAL_TYPE_PLATFORM, EInvalidParameter);
        
        // Verify registry type matches proposal type
        assert!(registry.registry_type == proposal_type, EInvalidRegistry);
        
        // Handle reference ID based on proposal type
        let actual_reference_id = if (proposal_type == PROPOSAL_TYPE_ECOSYSTEM) {
            // Ecosystem proposals use reference_id as provided
            reference_id
        } else if (proposal_type == PROPOSAL_TYPE_PROOF_OF_CREATIVITY) {
            // Proof of creativity proposals should have either reference_id or disputed_id
            if (option::is_some(&reference_id)) {
                reference_id
            } else if (option::is_some(&disputed_id)) {
                disputed_id
            } else {
                // Proof of creativity proposals should reference creative content
                assert!(false, EInvalidParameter);
                option::none<ID>()
            }
        } else if (proposal_type == PROPOSAL_TYPE_SPOT) {
            assert!(option::is_some(&reference_id), EInvalidParameter);
            reference_id
        } else {
            // For other proposal types (like platform), use reference_id if provided
            reference_id
        };
        
        let _proposal_id = submit_proposal_internal(
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

    /// Submit a new proposal to the ecosystem registry
    /// Requires staking MYSO tokens equal to the proposal submission cost
    public entry fun submit_ecosystem_proposal(
        registry: &mut GovernanceDAO,
        title: String,
        description: String,
        reference_id: Option<ID>,
        metadata_json: Option<String>,
        coin: &mut Coin<MYSO>,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        submit_proposal(
            registry,
            PROPOSAL_TYPE_ECOSYSTEM,
            title,
            description,
            option::none<ID>(), // No disputed ID for ecosystem proposals
            reference_id,
            metadata_json,
            coin,
            clock,
            ctx
        );
    }

    /// Submit a proof of creativity proposal
    public entry fun submit_proof_of_creativity_proposal(
        registry: &mut GovernanceDAO,
        title: String,
        description: String,
        creative_content_id: ID,
        metadata_json: Option<String>,
        coin: &mut Coin<MYSO>,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        submit_proposal(
            registry,
            PROPOSAL_TYPE_PROOF_OF_CREATIVITY,
            title,
            description,
            option::none<ID>(), // No disputed ID for proof of creativity
            option::some(creative_content_id), // Reference to creative content
            metadata_json,
            coin,
            clock,
            ctx
        );
    }

    /// Submit a SPoT resolution proposal (binary ratification of one outcome for a contested market).
    public entry fun submit_spot_resolution_proposal(
        registry: &mut GovernanceDAO,
        title: String,
        description: String,
        spot_record_id: ID,
        metadata_json: Option<String>,
        coin: &mut Coin<MYSO>,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        assert!(registry.registry_type == PROPOSAL_TYPE_SPOT, EInvalidRegistry);
        let _ = submit_spot_proposal_and_return_id(
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

    /// Package helper: submit a SPoT resolution proposal and return its ID.
    public(package) fun submit_spot_proposal_and_return_id(
        registry: &mut GovernanceDAO,
        title: String,
        description: String,
        spot_record_id: ID,
        metadata_json: Option<String>,
        coin: &mut Coin<MYSO>,
        clock: &Clock,
        ctx: &mut TxContext
    ): ID {
        assert!(registry.registry_type == PROPOSAL_TYPE_SPOT, EInvalidRegistry);
        submit_proposal_internal(
            registry,
            title,
            description,
            PROPOSAL_TYPE_SPOT,
            option::some(spot_record_id),
            metadata_json,
            coin,
            clock,
            ctx
        )
    }

    /// Internal function for submitting proposals
    fun submit_proposal_internal(
        registry: &mut GovernanceDAO,
        title: String,
        description: String,
        proposal_type: u8,
        reference_id: Option<ID>,
        metadata_json: Option<String>,
        coin: &mut Coin<MYSO>,
        clock: &Clock,
        ctx: &mut TxContext
    ): ID {
        let caller = tx_context::sender(ctx);
        let current_time = clock::timestamp_ms(clock);
        
        // Check stake amount
        assert!(coin::value(coin) >= registry.proposal_submission_cost, EInvalidAmount);
        
        // Create proposal
        let proposal_id = object::new(ctx);
        let mut proposal = Proposal {
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
            status: STATUS_DELEGATE_REVIEW,
            voting_start_time: 0,
            voting_end_time: 0,
            reward_pool: balance::zero(),
        };
        
        // Split coin and add to proposal's reward pool
        let proposal_coin = coin::split(coin, registry.proposal_submission_cost, ctx);
        balance::join(&mut proposal.reward_pool, coin::into_balance(proposal_coin));
        
        // Initialize dynamic fields for vote tracking
        let delegate_votes = table::new<address, bool>(ctx);
        dynamic_field::add(&mut proposal.id, DELEGATE_VOTES_FIELD, delegate_votes);
        
        let voted_community = vec_set::empty<address>();
        dynamic_field::add(&mut proposal.id, VOTED_COMMUNITY_FIELD, voted_community);
        
        // Share proposal as shared object and register in registry
        let proposal_id_copy = object::id(&proposal);
        transfer::share_object(proposal);
        table::add(&mut registry.proposals, proposal_id_copy, true);
        table::add(&mut registry.proposal_types, proposal_id_copy, proposal_type);
        
        // Add to proposals by status
        let proposals_of_status = table::borrow_mut(&mut registry.proposals_by_status, STATUS_DELEGATE_REVIEW);
        vector::push_back(proposals_of_status, proposal_id_copy);
        
        // Increment proposal count for delegate if applicable
        if (table::contains(&registry.delegates, caller)) {
            let delegate = table::borrow_mut(&mut registry.delegates, caller);
            delegate.proposals_submitted = delegate.proposals_submitted + 1;
        };
        
        // Emit event
        event::emit(ProposalSubmittedEvent {
            proposal_id: proposal_id_copy,
            title: title,
            description: description,
            proposal_type: proposal_type,
            reference_id: reference_id,
            metadata_json: metadata_json,
            submitter: caller,
            reward_amount: registry.proposal_submission_cost,
            submission_time: current_time,
        });

        try_update_delegate_panel_if_due(registry, ctx);

        proposal_id_copy
    }

    /// Allow a proposal owner to rescind their proposal if it's still in the delegate review stage
    public entry fun rescind_proposal(
        registry: &mut GovernanceDAO,
        proposal: &mut Proposal,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        // Check version compatibility
        assert!(registry.version == upgrade::current_version(), EWrongVersion);
        
        let caller = tx_context::sender(ctx);
        let current_time = clock::timestamp_ms(clock);
        let proposal_id = object::id(proposal);
        
        // Verify proposal exists and is in delegate review phase
        assert!(table::contains(&registry.proposals, proposal_id), EProposalNotFound);
        
        // Verify caller is the proposal submitter
        assert!(proposal.submitter == caller, EUnauthorized);
        
        // Verify proposal is still in delegate review stage
        assert!(proposal.status == STATUS_DELEGATE_REVIEW, EInvalidProposalStatus);
        
        // Update proposals by status - remove from delegate review
        let from_status = table::borrow_mut(&mut registry.proposals_by_status, STATUS_DELEGATE_REVIEW);
        let mut index = 0;
        let len = vector::length(from_status);
        while (index < len) {
            if (*vector::borrow(from_status, index) == proposal_id) {
                vector::remove(from_status, index);
                break
            };
            index = index + 1;
        };
        
        // Return staked tokens to submitter
        let refund_amount = balance::value(&proposal.reward_pool);
        if (refund_amount > 0) {
            let refund_coin = coin::from_balance(balance::withdraw_all(&mut proposal.reward_pool), ctx);
            transfer::public_transfer(refund_coin, caller);
        } else {
            // Empty the balance even if zero, for consistency
            balance::destroy_zero(balance::withdraw_all(&mut proposal.reward_pool));
        };
        
        // Emit event for the rescinded proposal
        event::emit(ProposalRescindedEvent {
            proposal_id,
            submitter: caller,
            rescind_time: current_time,
            refund_amount,
        });
        
        // Update proposal status to owner rescinded
        proposal.status = STATUS_OWNER_RESCIND;
        
        // Add to owner rescinded list
        let to_status = table::borrow_mut(&mut registry.proposals_by_status, STATUS_OWNER_RESCIND);
        vector::push_back(to_status, proposal_id);
    }

    /// Move a proposal to community voting phase
    fun move_to_community_voting(
        registry: &mut GovernanceDAO,
        proposal: &mut Proposal,
        clock: &Clock,
        _ctx: &TxContext
    ) {
        let proposal_id = object::id(proposal);
        
        // Update status
        proposal.status = STATUS_COMMUNITY_VOTING;
        
        // Get current timestamp in milliseconds for voting period calculation
        let current_time_ms = clock::timestamp_ms(clock);
        
        // Set voting period based on registry voting period (using milliseconds)
        proposal.voting_start_time = current_time_ms;
        proposal.voting_end_time = current_time_ms + registry.voting_period_ms;
        
        // Update proposals by status
        let from_status = table::borrow_mut(&mut registry.proposals_by_status, STATUS_DELEGATE_REVIEW);
        let mut index = 0;
        let len = vector::length(from_status);
        while (index < len) {
            if (*vector::borrow(from_status, index) == proposal_id) {
                vector::remove(from_status, index);
                break
            };
            index = index + 1;
        };
        
        let to_status = table::borrow_mut(&mut registry.proposals_by_status, STATUS_COMMUNITY_VOTING);
        vector::push_back(to_status, proposal_id);
        
        // Emit event - use millisecond timestamps
        event::emit(ProposalApprovedForVotingEvent {
            proposal_id,
            voting_start_time: current_time_ms,
            voting_end_time: current_time_ms + registry.voting_period_ms,
        });
    }

    /// Community vote on a proposal with quadratic voting
    /// Users can cast multiple votes by paying a quadratically increasing cost
    public entry fun community_vote_on_proposal(
        registry: &mut GovernanceDAO,
        proposal: &mut Proposal,
        vote_count: u64,
        approve: bool,
        coin: &mut Coin<MYSO>,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        // Check version compatibility
        assert!(registry.version == upgrade::current_version(), EWrongVersion);
        
        let caller = tx_context::sender(ctx);
        let current_time_ms = clock::timestamp_ms(clock);
        let proposal_id = object::id(proposal);
        
        // Calculate vote cost before borrowing from registry
        let quadratic_base_cost = registry.quadratic_base_cost;
        let vote_cost = if (vote_count <= 1) {
            0
        } else {
            // Quadratic cost formula: base_cost * (vote_count^2 - 1)
            quadratic_base_cost * ((vote_count * vote_count) - 1)
        };
        
        // Verify proposal exists and is in community voting phase
        assert!(table::contains(&registry.proposals, proposal_id), EProposalNotFound);
        assert!(proposal.status == STATUS_COMMUNITY_VOTING, ENotVotingPhase);
        
        // Verify voting period hasn't ended (check using milliseconds)
        assert!(current_time_ms <= proposal.voting_end_time, EVotingPeriodEnded);
        
        // Verify vote count is valid (at least 1, no more than max)
        assert!(vote_count > 0, EInvalidVoteCount);
        assert!(vote_count <= registry.max_votes_per_user, EExceedsMaxVotes);
        
        // Check if user has already voted
        let voted_community: &mut VecSet<address> = dynamic_field::borrow_mut(&mut proposal.id, VOTED_COMMUNITY_FIELD);
        assert!(!vec_set::contains(voted_community, &caller), EAlreadyVoted);
        
        // Check if user has enough funds for votes
        if (vote_cost > 0) {
            assert!(coin::value(coin) >= vote_cost, EInvalidAmount);
            
            // Split coin and add to proposal's reward pool for the additional votes
            let vote_coin = coin::split(coin, vote_cost, ctx);
            balance::join(&mut proposal.reward_pool, coin::into_balance(vote_coin));
        };
        
        // Record vote
        vec_set::insert(voted_community, caller);
        
        // Update vote tally
        if (approve) {
            proposal.community_votes_for = proposal.community_votes_for + vote_count;
        } else {
            proposal.community_votes_against = proposal.community_votes_against + vote_count;
        };
        
        // Emit event (use millisecond timestamp for the event record)
        event::emit(CommunityVoteEvent {
            proposal_id,
            voter: caller,
            vote_weight: vote_count,
            approve,
            vote_time: current_time_ms,
            vote_cost,
        });
    }

    /// Submit an anonymous encrypted vote on a proposal
    public fun community_vote_anonymous(
        registry: &mut GovernanceDAO,
        proposal: &mut Proposal,
        encrypted_vote: EncryptedObject,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        let caller = tx_context::sender(ctx);
        let current_time_ms = clock::timestamp_ms(clock);
        let proposal_id = object::id(proposal);

        assert!(table::contains(&registry.proposals, proposal_id), EProposalNotFound);
        assert!(proposal.status == STATUS_COMMUNITY_VOTING, ENotVotingPhase);
        assert!(current_time_ms <= proposal.voting_end_time, EVotingPeriodEnded);
        assert!(!table::contains(&registry.delegates, caller), EDelegateAnonNotAllowed);

        let voted_community: &mut VecSet<address> = dynamic_field::borrow_mut(&mut proposal.id, VOTED_COMMUNITY_FIELD);
        assert!(!vec_set::contains(voted_community, &caller), EAlreadyVoted);
        vec_set::insert(voted_community, caller);

        if (!dynamic_field::exists_(&proposal.id, ENCRYPTED_VOTES_FIELD)) {
            let tbl = table::new<address, EncryptedObject>(ctx);
            dynamic_field::add(&mut proposal.id, ENCRYPTED_VOTES_FIELD, tbl);
        };
        let enc_tbl: &mut Table<address, EncryptedObject> = dynamic_field::borrow_mut(&mut proposal.id, ENCRYPTED_VOTES_FIELD);
        table::add(enc_tbl, caller, encrypted_vote);

        if (!dynamic_field::exists_(&proposal.id, ANON_VOTERS_FIELD)) {
            let set = vec_set::empty<address>();
            dynamic_field::add(&mut proposal.id, ANON_VOTERS_FIELD, set);
        };
        let anon_set: &mut VecSet<address> = dynamic_field::borrow_mut(&mut proposal.id, ANON_VOTERS_FIELD);
        vec_set::insert(anon_set, caller);

        // Serialize the entire EncryptedObject for indexer storage
        let mut serialized_vote = vector::empty<u8>();
        serialized_vote.append(*encrypted_vote.blob());
        
        event::emit(AnonymousVoteEvent {
            proposal_id,
            voter: caller,
            vote_time: current_time_ms,
            encrypted_vote_data: serialized_vote, // Emit encrypted blob for indexer
        });
    }

    const DELEGATE_VOTE_STILL_IN_REVIEW: u8 = 0;
    const DELEGATE_VOTE_TO_COMMUNITY: u8 = 1;
    const DELEGATE_VOTE_TO_REJECT: u8 = 2;

    const FINALIZE_OUTCOME_APPROVED: u8 = 0;
    const FINALIZE_OUTCOME_REJECT_COMMUNITY: u8 = 1;
    const FINALIZE_OUTCOME_REJECT_QUORUM: u8 = 2;

    public fun community_finalize_voting_approved_value(): u8 { FINALIZE_OUTCOME_APPROVED }
    public fun community_finalize_voting_reject_community_value(): u8 { FINALIZE_OUTCOME_REJECT_COMMUNITY }
    public fun community_finalize_voting_reject_quorum_value(): u8 { FINALIZE_OUTCOME_REJECT_QUORUM }
    public fun delegate_vote_outcome_reject_value(): u8 { DELEGATE_VOTE_TO_REJECT }

    public fun forfeit_reason_quorum_not_met_value(): u8 { FORFEIT_REASON_QUORUM_NOT_MET }
    public fun forfeit_reason_community_rejected_value(): u8 { FORFEIT_REASON_COMMUNITY_REJECTED }
    public fun forfeit_reason_delegate_rejected_value(): u8 { FORFEIT_REASON_DELEGATE_REJECTED }

    public(package) fun assert_platform_delegate_manual_reject(
        registry: &GovernanceDAO,
        proposal: &Proposal,
        clock: &Clock,
        ctx: &TxContext
    ): u64 {
        assert!(registry.version == upgrade::current_version(), EWrongVersion);
        assert!(registry.registry_type == PROPOSAL_TYPE_PLATFORM, EWrongRegistryForTreasuryRoute);
        let caller = tx_context::sender(ctx);
        assert!(table::contains(&registry.delegates, caller), ENotDelegate);
        let proposal_id = object::id(proposal);
        assert!(table::contains(&registry.proposals, proposal_id), EProposalNotFound);
        assert!(proposal.status == STATUS_DELEGATE_REVIEW, EInvalidProposalStatus);
        clock::timestamp_ms(clock)
    }

    fun assert_shared_global_registry(registry: &GovernanceDAO) {
        assert!(
            registry.registry_type == PROPOSAL_TYPE_ECOSYSTEM
                || registry.registry_type == PROPOSAL_TYPE_PROOF_OF_CREATIVITY
                || registry.registry_type == PROPOSAL_TYPE_SPOT,
            EWrongRegistryForTreasuryRoute
        );
    }

    fun forfeit_proposal_pool_to_treasury_address(
        registry: &GovernanceDAO,
        proposal: &mut Proposal,
        recipient: address,
        reason: u8,
        treasury_route: u8,
        time_ms: u64,
        ctx: &mut TxContext
    ) {
        let proposal_id = object::id(proposal);
        let reg_type = registry.registry_type;
        let amount = balance::value(&proposal.reward_pool);
        if (amount > 0) {
            let c = coin::from_balance(balance::withdraw_all(&mut proposal.reward_pool), ctx);
            transfer::public_transfer(c, recipient);
        } else {
            balance::destroy_zero(balance::withdraw_all(&mut proposal.reward_pool));
        };
        event::emit(ProposalRewardPoolForfeitedToTreasuryEvent {
            proposal_id,
            recipient,
            amount,
            reason,
            registry_type: reg_type,
            treasury_route,
            forfeited_time: time_ms,
        });
    }

    /// Updates status tables only: delegate review to rejected. Caller sends pool + emits [ProposalRejectedEvent].
    fun transition_proposal_rejected_by_delegate_council(
        registry: &mut GovernanceDAO,
        proposal: &mut Proposal
    ) {
        let proposal_id = object::id(proposal);
        proposal.status = STATUS_REJECTED;
        let from_status = table::borrow_mut(&mut registry.proposals_by_status, STATUS_DELEGATE_REVIEW);
        let mut index = 0;
        let len = vector::length(from_status);
        while (index < len) {
            if (*vector::borrow(from_status, index) == proposal_id) {
                vector::remove(from_status, index);
                break
            };
            index = index + 1;
        };
        let to_status = table::borrow_mut(&mut registry.proposals_by_status, STATUS_REJECTED);
        vector::push_back(to_status, proposal_id);
    }

    fun complete_delegate_reject_to_ecosystem_treasury(
        registry: &mut GovernanceDAO,
        proposal: &mut Proposal,
        current_time: u64,
        ecosystem_treasury: &EcosystemTreasury,
        ctx: &mut TxContext
    ) {
        let proposal_id = object::id(proposal);
        transition_proposal_rejected_by_delegate_council(registry, proposal);
        let dest = profile::get_treasury_address(ecosystem_treasury);
        forfeit_proposal_pool_to_treasury_address(
            registry,
            proposal,
            dest,
            FORFEIT_REASON_DELEGATE_REJECTED,
            TREASURY_ROUTE_ECOSYSTEM,
            current_time,
            ctx,
        );
        event::emit(ProposalRejectedEvent {
            proposal_id,
            rejection_time: current_time,
        });
    }

    /// For platform: transition + forfeiture event (platform route) + [ProposalRejectedEvent]; returns pool balance for the caller to join into `Platform.treasury`.
    public(package) fun complete_delegate_reject_drain_to_platform_governance(
        registry: &mut GovernanceDAO,
        proposal: &mut Proposal,
        platform_treasury_address: address,
        current_time: u64,
        _ctx: &mut TxContext
    ): Balance<MYSO> {
        assert!(registry.registry_type == PROPOSAL_TYPE_PLATFORM, EWrongRegistryForTreasuryRoute);
        let proposal_id = object::id(proposal);
        transition_proposal_rejected_by_delegate_council(registry, proposal);
        let amount = balance::value(&proposal.reward_pool);
        event::emit(ProposalRewardPoolForfeitedToTreasuryEvent {
            proposal_id,
            recipient: platform_treasury_address,
            amount,
            reason: FORFEIT_REASON_DELEGATE_REJECTED,
            registry_type: registry.registry_type,
            treasury_route: TREASURY_ROUTE_PLATFORM,
            forfeited_time: current_time,
        });
        event::emit(ProposalRejectedEvent {
            proposal_id,
            rejection_time: current_time,
        });
        balance::withdraw_all(&mut proposal.reward_pool)
    }

    fun clone_bytes(b: &vector<u8>): vector<u8> {
        let mut out = vector::empty();
        let len = vector::length(b);
        let mut i = 0;
        while (i < len) {
            vector::push_back(&mut out, *vector::borrow(b, i));
            i = i + 1;
        };
        out
    }

    fun clone_option_string(opt: &Option<String>): Option<String> {
        if (option::is_none(opt)) {
            return option::none()
        };
        let s = option::borrow(opt);
        option::some(string::utf8(clone_bytes(string::as_bytes(s))))
    }

    /// Run delegate-in-review vote logic; returns 0=still in review, 1=advanced to community, 2=must complete delegate reject (ecosystem or platform) via separate entry.
    public(package) fun run_delegate_review_vote(
        registry: &mut GovernanceDAO,
        proposal: &mut Proposal,
        approve: bool,
        mut reason: Option<String>,
        clock: &Clock,
        ctx: &mut TxContext
    ): u8 {
        assert!(registry.version == upgrade::current_version(), EWrongVersion);
        try_update_delegate_panel_if_due(registry, ctx);
        let vote_time = clock::timestamp_ms(clock);
        let reason_for_event = clone_option_string(&reason);
        let caller = tx_context::sender(ctx);
        let proposal_id = object::id(proposal);
        assert!(table::contains(&registry.proposals, proposal_id), EProposalNotFound);
        assert!(table::contains(&registry.delegates, caller), ENotDelegate);
        assert!(proposal.status == STATUS_DELEGATE_REVIEW, EInvalidProposalStatus);

        let delegate_votes: &mut Table<address, bool> = dynamic_field::borrow_mut(&mut proposal.id, DELEGATE_VOTES_FIELD);
        assert!(!table::contains(delegate_votes, caller), EAlreadyVoted);

        table::add(delegate_votes, caller, approve);
        if (option::is_some(&reason)) {
            if (!dynamic_field::exists_(&proposal.id, DELEGATE_REASONS_FIELD)) {
                let reason_table = table::new<address, String>(ctx);
                dynamic_field::add(&mut proposal.id, DELEGATE_REASONS_FIELD, reason_table);
            };
            let reason_table: &mut Table<address, String> = dynamic_field::borrow_mut(
                &mut proposal.id,
                DELEGATE_REASONS_FIELD
            );
            table::add(reason_table, caller, option::extract(&mut reason));
        };
        if (!dynamic_field::exists_(&proposal.id, VOTED_DELEGATES_LIST_FIELD)) {
            let delegates_list = vector::empty<address>();
            dynamic_field::add(&mut proposal.id, VOTED_DELEGATES_LIST_FIELD, delegates_list);
        };
        let delegates_list: &mut vector<address> = dynamic_field::borrow_mut(
            &mut proposal.id,
            VOTED_DELEGATES_LIST_FIELD
        );
        vector::push_back(delegates_list, caller);

        if (approve) {
            proposal.delegate_approval_count = proposal.delegate_approval_count + 1;
        } else {
            proposal.delegate_rejection_count = proposal.delegate_rejection_count + 1;
        };

        let delegate = table::borrow_mut(&mut registry.delegates, caller);
        delegate.proposals_reviewed = delegate.proposals_reviewed + 1;

        let delegate_approval_count = proposal.delegate_approval_count;
        let delegate_rejection_count = proposal.delegate_rejection_count;
        let total_delegates = table::length(&registry.delegates);

        let outcome = if (delegate_approval_count > total_delegates / 2) {
            move_to_community_voting(registry, proposal, clock, ctx);
            DELEGATE_VOTE_TO_COMMUNITY
        } else if (delegate_rejection_count > total_delegates / 2) {
            DELEGATE_VOTE_TO_REJECT
        } else {
            DELEGATE_VOTE_STILL_IN_REVIEW
        };

        event::emit(DelegateVoteEvent {
            proposal_id,
            delegate_address: caller,
            approve,
            vote_time,
            reason: reason_for_event,
        });
        outcome
    }

    /// Ecosystem and proof-of-creativity only: `ecosystem_treasury` used when the delegate council rejects the proposal.
    public entry fun delegate_vote_on_proposal(
        registry: &mut GovernanceDAO,
        proposal: &mut Proposal,
        ecosystem_treasury: &EcosystemTreasury,
        approve: bool,
        reason: Option<String>,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        assert!(registry.version == upgrade::current_version(), EWrongVersion);
        assert_shared_global_registry(registry);
        let current_time = clock::timestamp_ms(clock);
        let out = run_delegate_review_vote(registry, proposal, approve, reason, clock, ctx);
        if (out == DELEGATE_VOTE_TO_REJECT) {
            complete_delegate_reject_to_ecosystem_treasury(
                registry,
                proposal,
                current_time,
                ecosystem_treasury,
                ctx
            );
        };
    }

    public(package) fun emit_implementation_reward_to_submitter_event(
        proposal_id: ID,
        submitter: address,
        amount: u64,
        registry_type: u8,
        sent_time: u64,
    ) {
        event::emit(ProposalImplementationRewardToSubmitterEvent {
            proposal_id,
            submitter,
            amount,
            registry_type,
            sent_time,
        });
    }

    public(package) fun finalize_community_voting_internals(
        registry: &mut GovernanceDAO,
        proposal: &mut Proposal,
        clock: &Clock,
        ctx: &TxContext,
    ): (u8, u64) {
        try_update_delegate_panel_if_due(registry, ctx);
        let current_time_ms = clock::timestamp_ms(clock);
        let proposal_id = object::id(proposal);
        assert!(table::contains(&registry.proposals, proposal_id), EProposalNotFound);
        assert!(proposal.status == STATUS_COMMUNITY_VOTING, EInvalidProposalStatus);
        assert!(current_time_ms > proposal.voting_end_time, EVotingPeriodNotEnded);

        let total_votes = proposal.community_votes_for + proposal.community_votes_against;
        let from_status = table::borrow_mut(&mut registry.proposals_by_status, STATUS_COMMUNITY_VOTING);
        let mut index = 0;
        let len = vector::length(from_status);
        while (index < len) {
            if (*vector::borrow(from_status, index) == proposal_id) {
                vector::remove(from_status, index);
                break
            };
            index = index + 1;
        };

        let quorum_met = total_votes >= registry.quorum_votes;
        let majority_approve = proposal.community_votes_for > proposal.community_votes_against;
        if (quorum_met) {
            if (dynamic_field::exists_(&proposal.id, VOTED_DELEGATES_LIST_FIELD)) {
                let delegates_list: &vector<address> = dynamic_field::borrow(
                    &proposal.id,
                    VOTED_DELEGATES_LIST_FIELD
                );
                let delegate_votes: &Table<address, bool> = dynamic_field::borrow(
                    &proposal.id,
                    DELEGATE_VOTES_FIELD
                );
                let mut i = 0;
                let delegates_count = vector::length(delegates_list);
                while (i < delegates_count) {
                    let delegate_addr = *vector::borrow(delegates_list, i);
                    if (table::contains(&registry.delegates, delegate_addr)) {
                        let delegate = table::borrow_mut(&mut registry.delegates, delegate_addr);
                        let delegate_approved = *table::borrow(delegate_votes, delegate_addr);
                        if ((delegate_approved && majority_approve) || (!delegate_approved && !majority_approve)) {
                            delegate.sided_winning_proposals = delegate.sided_winning_proposals + 1;
                        } else {
                            delegate.sided_losing_proposals = delegate.sided_losing_proposals + 1;
                        };
                    };
                    i = i + 1;
                };
            };
            if (majority_approve) {
                proposal.status = STATUS_APPROVED;
                let to_status = table::borrow_mut(&mut registry.proposals_by_status, STATUS_APPROVED);
                vector::push_back(to_status, proposal_id);
                event::emit(ProposalApprovedEvent {
                    proposal_id,
                    approval_time: current_time_ms,
                    votes_for: proposal.community_votes_for,
                    votes_against: proposal.community_votes_against,
                });
                (FINALIZE_OUTCOME_APPROVED, current_time_ms)
            } else {
                proposal.status = STATUS_REJECTED;
                let to_status = table::borrow_mut(&mut registry.proposals_by_status, STATUS_REJECTED);
                vector::push_back(to_status, proposal_id);
                event::emit(ProposalRejectedByCommunityEvent {
                    proposal_id,
                    rejection_time: current_time_ms,
                    votes_for: proposal.community_votes_for,
                    votes_against: proposal.community_votes_against,
                });
                (FINALIZE_OUTCOME_REJECT_COMMUNITY, current_time_ms)
            }
        } else {
            proposal.status = STATUS_REJECTED;
            let to_status = table::borrow_mut(&mut registry.proposals_by_status, STATUS_REJECTED);
            vector::push_back(to_status, proposal_id);
            event::emit(ProposalRejectedByCommunityEvent {
                proposal_id,
                rejection_time: current_time_ms,
                votes_for: proposal.community_votes_for,
                votes_against: proposal.community_votes_against,
            });
            (FINALIZE_OUTCOME_REJECT_QUORUM, current_time_ms)
        }
    }

    /// Ecosystem and proof-of-creativity: finalize after community vote; on failure, reward pool to ecosystem treasury.
    public entry fun finalize_proposal(
        registry: &mut GovernanceDAO,
        proposal: &mut Proposal,
        ecosystem_treasury: &EcosystemTreasury,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        assert!(registry.version == upgrade::current_version(), EWrongVersion);
        assert_shared_global_registry(registry);
        let (outcome, time_ms) = finalize_community_voting_internals(registry, proposal, clock, ctx);
        if (outcome == FINALIZE_OUTCOME_REJECT_COMMUNITY) {
            let dest = profile::get_treasury_address(ecosystem_treasury);
            forfeit_proposal_pool_to_treasury_address(
                registry,
                proposal,
                dest,
                FORFEIT_REASON_COMMUNITY_REJECTED,
                TREASURY_ROUTE_ECOSYSTEM,
                time_ms,
                ctx
            );
        } else if (outcome == FINALIZE_OUTCOME_REJECT_QUORUM) {
            let dest = profile::get_treasury_address(ecosystem_treasury);
            forfeit_proposal_pool_to_treasury_address(
                registry,
                proposal,
                dest,
                FORFEIT_REASON_QUORUM_NOT_MET,
                TREASURY_ROUTE_ECOSYSTEM,
                time_ms,
                ctx
            );
        };
    }

    /// After community voting fails for a platform registry: move funds into this platform’s governance (treasury) balance.
    public(package) fun forfeit_proposal_pool_after_community_reject_to_platform_treasury(
        registry: &GovernanceDAO,
        proposal: &mut Proposal,
        platform_treasury_address: address,
        reason: u8,
        time_ms: u64,
    ): Balance<MYSO> {
        assert!(registry.registry_type == PROPOSAL_TYPE_PLATFORM, EWrongRegistryForTreasuryRoute);
        let proposal_id = object::id(proposal);
        let amount = balance::value(&proposal.reward_pool);
        event::emit(ProposalRewardPoolForfeitedToTreasuryEvent {
            proposal_id,
            recipient: platform_treasury_address,
            amount,
            reason,
            registry_type: registry.registry_type,
            treasury_route: TREASURY_ROUTE_PLATFORM,
            forfeited_time: time_ms,
        });
        balance::withdraw_all(&mut proposal.reward_pool)
    }

    /// Decrypt anonymous votes, merge tallies, and resolve community finalization. Callable from [`social_contracts::platform`] for platform registries.
    public(package) fun anonymous_tally_votes_and_finalize_community_internals(
        registry: &mut GovernanceDAO,
        proposal: &mut Proposal,
        keys: &vector<VerifiedDerivedKey>,
        public_keys: &vector<PublicKey>,
        clock: &Clock,
        ctx: &TxContext,
    ): (u8, u64) {
        assert!(registry.version == upgrade::current_version(), EWrongVersion);
        let current_time_ms = clock::timestamp_ms(clock);
        let proposal_id = object::id(proposal);
        assert!(table::contains(&registry.proposals, proposal_id), EProposalNotFound);
        let mut votes_for = vector::empty<address>();
        let mut votes_against = vector::empty<address>();
        let mut invalid_votes = vector::empty<address>();
        {
            assert!(proposal.status == STATUS_COMMUNITY_VOTING, EInvalidProposalStatus);
            assert!(current_time_ms > proposal.voting_end_time, EVotingPeriodNotEnded);
            if (dynamic_field::exists_(&proposal.id, ENCRYPTED_VOTES_FIELD)) {
                let votes_tbl: &Table<address, EncryptedObject> = dynamic_field::borrow(
                    &proposal.id,
                    ENCRYPTED_VOTES_FIELD
                );
                let anon_set: &VecSet<address> = dynamic_field::borrow(&proposal.id, ANON_VOTERS_FIELD);
                let voters_vec = vec_set::into_keys(*anon_set);
                let mut i = 0;
                let len = vector::length(&voters_vec);
                while (i < len) {
                    let addr = *vector::borrow(&voters_vec, i);
                    let enc = table::borrow(votes_tbl, addr);
                    let dec = decrypt(enc, keys, public_keys);
                    if (option::is_some(&dec)) {
                        let b = option::borrow(&dec);
                        if (vector::length(b) == 1) {
                            let vote_value = *vector::borrow(b, 0);
                            if (vote_value == 1) {
                                vector::push_back(&mut votes_for, addr);
                            } else if (vote_value == 0) {
                                vector::push_back(&mut votes_against, addr);
                            } else {
                                vector::push_back(&mut invalid_votes, addr);
                                event::emit(VoteDecryptionFailedEvent {
                                    proposal_id,
                                    voter: addr,
                                    failure_reason: string::utf8(b"Invalid vote value - not 0 or 1"),
                                    timestamp: current_time_ms,
                                });
                            }
                        } else {
                            vector::push_back(&mut invalid_votes, addr);
                            event::emit(VoteDecryptionFailedEvent {
                                proposal_id,
                                voter: addr,
                                failure_reason: string::utf8(b"Invalid vote format - wrong byte length"),
                                timestamp: current_time_ms,
                            });
                        }
                    } else {
                        vector::push_back(&mut invalid_votes, addr);
                        event::emit(VoteDecryptionFailedEvent {
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
            let mut i = 0;
            let len = vector::length(&votes_for);
            while (i < len) {
                assert!(proposal.community_votes_for <= MAX_U64 - 1, EOverflow);
                proposal.community_votes_for = proposal.community_votes_for + 1;
                i = i + 1;
            };
            let mut i = 0;
            let len = vector::length(&votes_against);
            while (i < len) {
                assert!(proposal.community_votes_against <= MAX_U64 - 1, EOverflow);
                proposal.community_votes_against = proposal.community_votes_against + 1;
                i = i + 1;
            };
        };
        vector::destroy_empty(votes_for);
        vector::destroy_empty(votes_against);
        finalize_community_voting_internals(registry, proposal, clock, ctx)
    }

    /// Finalize a proposal with anonymous votes by decrypting them first (ecosystem / proof-of-creativity only).
    public fun finalize_proposal_anonymous(
        registry: &mut GovernanceDAO,
        proposal: &mut Proposal,
        keys: &vector<VerifiedDerivedKey>,
        public_keys: &vector<PublicKey>,
        ecosystem_treasury: &EcosystemTreasury,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        assert_shared_global_registry(registry);
        let (outcome, time_ms) = anonymous_tally_votes_and_finalize_community_internals(
            registry,
            proposal,
            keys,
            public_keys,
            clock,
            ctx,
        );
        if (outcome == FINALIZE_OUTCOME_REJECT_COMMUNITY) {
            let dest = profile::get_treasury_address(ecosystem_treasury);
            forfeit_proposal_pool_to_treasury_address(
                registry,
                proposal,
                dest,
                FORFEIT_REASON_COMMUNITY_REJECTED,
                TREASURY_ROUTE_ECOSYSTEM,
                time_ms,
                ctx
            );
        } else if (outcome == FINALIZE_OUTCOME_REJECT_QUORUM) {
            let dest = profile::get_treasury_address(ecosystem_treasury);
            forfeit_proposal_pool_to_treasury_address(
                registry,
                proposal,
                dest,
                FORFEIT_REASON_QUORUM_NOT_MET,
                TREASURY_ROUTE_ECOSYSTEM,
                time_ms,
                ctx
            );
        };
    }

    /// Shared implementation transition + drains `reward_pool` for routing by the caller.
    public(package) fun mark_proposal_implemented_take_pool(
        registry: &mut GovernanceDAO,
        proposal: &mut Proposal,
        description: Option<String>,
        clock: &Clock,
        ctx: &TxContext
    ): Balance<MYSO> {
        assert!(registry.version == upgrade::current_version(), EWrongVersion);
        
        let caller = tx_context::sender(ctx);
        let current_time = clock::timestamp_ms(clock);
        let proposal_id = object::id(proposal);
        
        assert!(table::contains(&registry.proposals, proposal_id), EProposalNotFound);
        assert!(proposal.status == STATUS_APPROVED, EInvalidProposalStatus);
        
        assert!(
            proposal.submitter == caller || table::contains(&registry.delegates, caller),
            EUnauthorized
        );
        
        proposal.status = STATUS_IMPLEMENTED;
        
        let from_status = table::borrow_mut(&mut registry.proposals_by_status, STATUS_APPROVED);
        let mut index = 0;
        let len = vector::length(from_status);
        while (index < len) {
            if (*vector::borrow(from_status, index) == proposal_id) {
                vector::remove(from_status, index);
                break
            };
            index = index + 1;
        };
        
        let to_status = table::borrow_mut(&mut registry.proposals_by_status, STATUS_IMPLEMENTED);
        vector::push_back(to_status, proposal_id);
        
        event::emit(ProposalImplementedEvent {
            proposal_id,
            implementation_time: current_time,
            description,
        });
        
        balance::withdraw_all(&mut proposal.reward_pool)
    }

    /// Mark implemented: reward the submitter with the implementation pool (ecosystem or proof-of-creativity registry).
    public entry fun mark_proposal_implemented_to_ecosystem_treasury(
        registry: &mut GovernanceDAO,
        proposal: &mut Proposal,
        _ecosystem_treasury: &EcosystemTreasury,
        description: Option<String>,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        assert_shared_global_registry(registry);
        let sent_time = clock::timestamp_ms(clock);
        let submitter = proposal.submitter;
        let bal = mark_proposal_implemented_take_pool(registry, proposal, description, clock, ctx);
        let amount = balance::value(&bal);
        if (amount > 0) {
            let c = coin::from_balance(bal, ctx);
            transfer::public_transfer(c, submitter);
        } else {
            balance::destroy_zero(bal);
        };
        event::emit(ProposalImplementationRewardToSubmitterEvent {
            proposal_id: object::id(proposal),
            submitter,
            amount,
            registry_type: registry.registry_type,
            sent_time,
        });
    }

    /// Mark implemented: platform registry when the linked [`Platform`] is not in the transaction — still pays the proposer.
    public entry fun mark_proposal_implemented_platform_to_ecosystem_treasury(
        registry: &mut GovernanceDAO,
        proposal: &mut Proposal,
        _ecosystem_treasury: &EcosystemTreasury,
        description: Option<String>,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        assert!(registry.registry_type == PROPOSAL_TYPE_PLATFORM, EWrongRegistryForTreasuryRoute);
        let sent_time = clock::timestamp_ms(clock);
        let submitter = proposal.submitter;
        let bal = mark_proposal_implemented_take_pool(registry, proposal, description, clock, ctx);
        let amount = balance::value(&bal);
        if (amount > 0) {
            let c = coin::from_balance(bal, ctx);
            transfer::public_transfer(c, submitter);
        } else {
            balance::destroy_zero(bal);
        };
        event::emit(ProposalImplementationRewardToSubmitterEvent {
            proposal_id: object::id(proposal),
            submitter,
            amount,
            registry_type: registry.registry_type,
            sent_time,
        });
    }

    /// Registry type discriminator for platform routing (see [`social_contracts::platform`]).
    public fun proposal_type_platform_value(): u8 {
        PROPOSAL_TYPE_PLATFORM
    }

    public fun proposal_type_spot_value(): u8 {
        PROPOSAL_TYPE_SPOT
    }

    public fun proposal_type_proof_of_creativity_value(): u8 {
        PROPOSAL_TYPE_PROOF_OF_CREATIVITY
    }

    public fun proposal_status(proposal: &Proposal): u8 {
        proposal.status
    }

    public fun proposal_reference_id(proposal: &Proposal): Option<ID> {
        proposal.reference_id
    }

    public fun status_approved_value(): u8 {
        STATUS_APPROVED
    }

    public fun status_rejected_value(): u8 {
        STATUS_REJECTED
    }

    /// Get all proposals of a specific type
    public fun get_proposals_by_type(
        registry: &GovernanceDAO,
        proposal_type: u8
    ): vector<ID> {
        assert!(proposal_type <= PROPOSAL_TYPE_PLATFORM, EInvalidParameter);
        let mut result = vector::empty<ID>();
        let statuses = vector[ STATUS_SUBMITTED, STATUS_DELEGATE_REVIEW, STATUS_COMMUNITY_VOTING, STATUS_APPROVED, STATUS_REJECTED, STATUS_IMPLEMENTED ];
        let mut s = 0;
        while (s < vector::length(&statuses)) {
            let status = *vector::borrow(&statuses, s);
            let proposals_of_status: &vector<ID> = table::borrow(&registry.proposals_by_status, status);
            let mut p = 0;
            let num_proposals = vector::length(proposals_of_status);
            
            while (p < num_proposals) {
                let proposal_id = *vector::borrow(proposals_of_status, p);
                if (table::contains(&registry.proposal_types, proposal_id) && *table::borrow(&registry.proposal_types, proposal_id) == proposal_type) {
                    vector::push_back(&mut result, proposal_id);
                };
                
                p = p + 1;
            };
            
            s = s + 1;
        };
        vector::destroy_empty(statuses);
        result
    }

    /// Get all proposals with a specific status
    public fun get_proposals_by_status(
        registry: &GovernanceDAO,
        status: u8
    ): vector<ID> {
        assert!(status <= STATUS_IMPLEMENTED, EInvalidParameter);
        *table::borrow(&registry.proposals_by_status, status)
    }

    /// Get number of delegates
    public fun get_delegate_count(registry: &GovernanceDAO): u64 {
        table::length(&registry.delegates)
    }

    /// Registry type discriminator (ecosystem, proof of creativity, platform).
    public fun registry_type(registry: &GovernanceDAO): u8 {
        registry.registry_type
    }

    /// Get delegate information
    public fun get_delegate_info(
        registry: &GovernanceDAO,
        addr: address
    ): (u64, u64, u64, u64, u64, u64, u64, u64) {
        assert!(table::contains(&registry.delegates, addr), ENotDelegate);
        
        let delegate = table::borrow(&registry.delegates, addr);
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

    /// Get proposal information
    public fun get_proposal_info(
        registry: &GovernanceDAO,
        proposal: &Proposal
    ): (String, String, u8, Option<ID>, Option<String>, address, u64, u8, u64, u64) {
        assert!(table::contains(&registry.proposals, object::id(proposal)), EProposalNotFound);
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

    public fun proposal_submitter(proposal: &Proposal): address {
        proposal.submitter
    }

    /// Get the current treasury balance
    public fun treasury_balance(registry: &GovernanceDAO): u64 {
        balance::value(&registry.treasury)
    }

    /// Calculate cost for additional votes beyond the first free vote
    public fun calculate_vote_cost(
        vote_count: u64,
        registry: &GovernanceDAO
    ): u64 {
        if (vote_count <= 1) {
            return 0
        };
        
        // Quadratic cost formula: base_cost * (vote_count^2 - 1)
        registry.quadratic_base_cost * ((vote_count * vote_count) - 1)
    }

    /// Check if an address is a delegate
    public fun is_delegate( registry: &GovernanceDAO, addr: address ): bool {
        table::contains(&registry.delegates, addr)
    }

    /// Check if delegate term has expired
    public fun is_delegate_term_expired(
        registry: &GovernanceDAO,
        addr: address,
        current_epoch: u64
    ): bool {
        if (!table::contains(&registry.delegates, addr)) {
            return false
        };
        
        let delegate = table::borrow(&registry.delegates, addr);
        delegate.term_end <= current_epoch
    }

    /// Get governance parameters
    public fun get_governance_parameters(
        registry: &GovernanceDAO
    ): (u64, u64, u64, u64, u64, u64, u64) {
        (
            registry.delegate_count,
            registry.delegate_term_epochs,
            registry.proposal_submission_cost,
            registry.max_votes_per_user,
            registry.quadratic_base_cost,
            registry.voting_period_ms,
            registry.quorum_votes
        )
    }

    /// If more than half of delegates reject, reject the proposal manually (ecosystem / proof-of-creativity only).
    public entry fun reject_proposal_manually(
        registry: &mut GovernanceDAO,
        proposal: &mut Proposal,
        ecosystem_treasury: &EcosystemTreasury,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        assert!(registry.version == upgrade::current_version(), EWrongVersion);
        assert_shared_global_registry(registry);
        let caller = tx_context::sender(ctx);
        assert!(table::contains(&registry.delegates, caller), ENotDelegate);
        let proposal_id = object::id(proposal);
        assert!(table::contains(&registry.proposals, proposal_id), EProposalNotFound);
        assert!(proposal.status == STATUS_DELEGATE_REVIEW, EInvalidProposalStatus);
        let current_time = clock::timestamp_ms(clock);
        complete_delegate_reject_to_ecosystem_treasury(
            registry,
            proposal,
            current_time,
            ecosystem_treasury,
            ctx
        );
    }

    /// Create a platform-specific governance registry when a platform is approved
    /// This function can only be called by the platform toggle_platform_approval function
    /// The transaction sender is seeded as the founding delegate (same pattern as `bootstrap_init`).
    public(package) fun create_platform_governance(
        delegate_count: u64,
        delegate_term_epochs: u64,
        proposal_submission_cost: u64,
        max_votes_per_user: u64,
        quadratic_base_cost: u64,
        voting_period_ms: u64,
        quorum_votes: u64,
        clock: &Clock,
        ctx: &mut TxContext
    ): ID {
        let current_time = clock::timestamp_ms(clock);
        let founding_delegate = tx_context::sender(ctx);
        
        // Create Platform Governance Registry with parameters
        let mut platform_registry = GovernanceDAO {
            id: object::new(ctx),
            registry_type: PROPOSAL_TYPE_PLATFORM,
            // Configuration parameters specific to platform governance
            delegate_count, 
            delegate_term_epochs,
            proposal_submission_cost,
            max_votes_per_user,
            quadratic_base_cost,
            voting_period_ms,
            quorum_votes,
            // Tables
            delegates: table::new<address, Delegate>(ctx),
            proposals: table::new<ID, bool>(ctx),
            proposal_types: table::new<ID, u8>(ctx),
            proposals_by_status: table::new<u8, vector<ID>>(ctx),
            treasury: balance::zero(),
            nominated_delegates: table::new<address, NominatedDelegate>(ctx),
            delegate_addresses: vec_set::empty<address>(),
            nominee_addresses: vec_set::empty<address>(),
            voters: table::new<address, Table<address, bool>>(ctx),
            version: upgrade::current_version(),
            last_delegate_panel_boundary_epoch: 0,
        };
        
        // Initialize registry tables
        initialize_registry_tables(&mut platform_registry, ctx);
        seed_founding_delegate(&mut platform_registry, founding_delegate, ctx);
        
        // Get the ID before sharing
        let registry_id = object::id(&platform_registry);
        
        // Emit event for platform registry creation
        event::emit(GovernanceRegistryCreatedEvent {
            registry_id,
            registry_type: PROPOSAL_TYPE_PLATFORM,
            delegate_count: platform_registry.delegate_count,
            delegate_term_epochs: platform_registry.delegate_term_epochs,
            proposal_submission_cost: platform_registry.proposal_submission_cost,
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

    /// Get version of GovernanceDAO
    public fun version(registry: &GovernanceDAO): u64 {
        registry.version
    }

    /// Largest `k * delegate_term_epochs` boundary for which a full delegate panel election has been applied.
    public fun last_delegate_panel_boundary_epoch(registry: &GovernanceDAO): u64 {
        registry.last_delegate_panel_boundary_epoch
    }

    /// Set version of GovernanceDAO
    public(package) fun set_version(registry: &mut GovernanceDAO, new_version: u64) {
        registry.version = new_version;
    }

    /// Public entry function that migrates registry to the latest version
    public entry fun migrate_registry(
        registry: &mut GovernanceDAO,
        _: &UpgradeAdminCap,
        ctx: &mut TxContext,
    ) {
        let current_version = registry.version;
        let latest_version = upgrade::current_version();

        assert!(current_version < latest_version, EWrongVersion);

        // Version-specific migrations would go here when needed

        let old_version = current_version;
        registry.version = latest_version;

        upgrade::emit_migration_event(
            object::id(registry),
            string::utf8(b"GovernanceDAO"),
            old_version,
            tx_context::sender(ctx),
        );
    }

    /// One-time admin path for existing networks that bootstrapped before the SPoT registry existed.
    public entry fun create_spot_governance_registry(
        _: &GovernanceAdminCap,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        let current_time = clock::timestamp_ms(clock);
        let founder = tx_context::sender(ctx);

        let mut spot_registry = GovernanceDAO {
            id: object::new(ctx),
            registry_type: PROPOSAL_TYPE_SPOT,
            delegate_count: 3,
            delegate_term_epochs: 90,
            proposal_submission_cost: 10_000_000_000,
            max_votes_per_user: 5,
            quadratic_base_cost: 1_000_000_000,
            voting_period_ms: 3 * 24 * 60 * 60 * 1000,
            quorum_votes: 10,
            delegates: table::new<address, Delegate>(ctx),
            proposals: table::new<ID, bool>(ctx),
            proposal_types: table::new<ID, u8>(ctx),
            proposals_by_status: table::new<u8, vector<ID>>(ctx),
            treasury: balance::zero(),
            nominated_delegates: table::new<address, NominatedDelegate>(ctx),
            delegate_addresses: vec_set::empty<address>(),
            nominee_addresses: vec_set::empty<address>(),
            voters: table::new<address, Table<address, bool>>(ctx),
            version: upgrade::current_version(),
            last_delegate_panel_boundary_epoch: 0,
        };

        initialize_registry_tables(&mut spot_registry, ctx);
        let spot_registry_id = object::id(&spot_registry);

        event::emit(GovernanceRegistryCreatedEvent {
            registry_id: spot_registry_id,
            registry_type: PROPOSAL_TYPE_SPOT,
            delegate_count: spot_registry.delegate_count,
            delegate_term_epochs: spot_registry.delegate_term_epochs,
            proposal_submission_cost: spot_registry.proposal_submission_cost,
            max_votes_per_user: spot_registry.max_votes_per_user,
            quadratic_base_cost: spot_registry.quadratic_base_cost,
            voting_period_ms: spot_registry.voting_period_ms,
            quorum_votes: spot_registry.quorum_votes,
            updated_at: current_time,
        });

        seed_founding_delegate(&mut spot_registry, founder, ctx);
        transfer::share_object(spot_registry);
    }

    /// Create a GovernanceAdminCap for bootstrap (package visibility only)
    /// This function is only callable by other modules in the same package
    public(package) fun create_governance_admin_cap(ctx: &mut TxContext): GovernanceAdminCap {
        GovernanceAdminCap {
            id: object::new(ctx)
        }
    }

    #[test_only]
    /// Grant a governance admin cap to the transaction sender (unit tests only).
    public fun test_grant_admin_cap(ctx: &mut TxContext) {
        let sender = tx_context::sender(ctx);
        transfer::public_transfer(create_governance_admin_cap(ctx), sender);
    }
}
