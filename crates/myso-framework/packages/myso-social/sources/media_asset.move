// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

/// Canonical MediaAsset registry types and asset-resolution entry points for PoC.
///
/// **Identity:** `MediaAsset.id` is the canonical creative identity; `content_commitment`
/// is registration evidence only.
///
/// **Three layers:**
/// 1. **Protocol claims** — asserted/verified statements (authorship, ownership, licensing authority, …)
/// 2. **Resolved rights interests** — `RightsInterest[]` produced from verified control/ownership claims
/// 3. **Product licenses** — `UsageGrant` terms per usage class (compensation, expiry, revocability)
///
/// Usage classes describe *product context*. Statutory-right bitflags map to U.S. copyright §106
/// (and §114 for sound recordings). `required_rights_for_usage()` is the protocol's normal
/// licensing requirement — not a claim that every legal instance requires that exact mask.
///
/// **Access control:**
/// - `submit_media_resolution` — public entry (anyone)
/// - `finalize_media_asset` / `oracle_update_media_asset_claims` — `public(package)`, oracle-gated
/// - `proof_of_creativity::finalize_media_asset` — public entry wrapper for oracle PTBs
/// - `update_media_asset_usage_grants` — rights controller or license authority
/// - `update_media_asset_economics` — authorized economics parties only
/// - Validation, grant checks, manifest helpers — `public(package)` (same package only)
///
/// **Not legal advice:** On-chain state records asserted/verified protocol claims. It does not
/// conclusively determine legal copyright ownership or replace fair use / statutory exceptions.

#[allow(duplicate_alias, unused_use, lint(public_entry))]
module social_contracts::media_asset {
    use std::string::String;
    use std::option::{Self, Option};
    use std::vector;
    use std::hash;

    use myso::{
        object::{Self, UID, ID},
        tx_context::{Self, TxContext},
        clock::{Self, Clock},
        transfer,
        event,
        dynamic_field as df,
    };
    use social_contracts::upgrade;
    use social_contracts::derivative_graph::{
        Self,
        AncestryMetadata,
        DerivativeRelationship,
        PendingComposeAccumulator,
        ResolvedRoyaltyObligation,
    };
    use social_contracts::license_template::{
        Self as license_tpl,
        LicenseTemplateVersion,
        LicenseInstance,
        AssetLicense,
        DirectUsageGrant,
    };

    const EClaimsRequireOracleVerification: u64 = 22;
    const EUnauthorized: u64 = 1;
    const EWrongVersion: u64 = 2;
    const ENotOracle: u64 = 3;
    const EInvalidMediaType: u64 = 4;
    const EInvalidCommitment: u64 = 5;
    const EInvalidBeneficiarySplits: u64 = 6;
    const EInvalidUsageClass: u64 = 7;
    const EUsageNotPermitted: u64 = 8;
    const EEmptyCreators: u64 = 9;
    const EInvalidManifest: u64 = 11;
    const EVersionMismatch: u64 = 13;
    const EInvalidRightsGrant: u64 = 14;
    const ETooManyUsageGrants: u64 = 15;
    const ETooManyRightsInterests: u64 = 16;
    const ETooManyClaims: u64 = 17;
    const EInvalidClaim: u64 = 18;
    const ENoVerifiedAuthorship: u64 = 19;
    const EInvalidAssetKind: u64 = 20;
    const ENoParentEdges: u64 = 23;
    const EHasParentEdges: u64 = 24;
    const EEdgeParentMismatch: u64 = 25;
    const EPolicyAlreadyMaterialized: u64 = 26;
    const EActiveRightsProposal: u64 = 27;
    const ENoActiveRightsProposal: u64 = 28;
    const ERightsDisputeCapReached: u64 = 29;

    const RIGHTS_DISPUTE_DF_KEY: vector<u8> = b"rights_dispute_state";
    const ERoyaltyOverflow: u64 = 29;

    public struct LineageCommitment has store, copy, drop {
        parent_asset_ids: vector<ID>,
        parent_policy_versions: vector<u64>,
        license_instance_ids: vector<ID>,
        template_version_ids: vector<ID>,
        ancestry_version: u64,
        lineage_hash: vector<u8>,
    }

    public struct ResolvedRightsPolicy has store, copy, drop {
        policy_version: u64,
        effective_rights: u64,
        derivatives_allowed: bool,
        attribution_required: bool,
        commercial_allowed: bool,
        lineage: LineageCommitment,
        resolved_at: u64,
    }

    public struct MediaAssetIdentity has copy, drop {
        content_commitment: vector<u8>,
        media_type: u8,
        asset_kind: u8,
        creator: address,
    }

    // === Media types ===
    public fun media_type_image(): u8 { 1 }
    public fun media_type_video(): u8 { 2 }
    public fun media_type_audio(): u8 { 3 }

    // === Product usage classes (what is the user doing?) ===
    public fun usage_social_post(): u8 { 1 }
    public fun usage_profile_picture(): u8 { 2 }
    public fun usage_cover_photo(): u8 { 3 }
    public fun usage_article_embed(): u8 { 4 }
    public fun usage_music_soundtrack(): u8 { 5 }
    public fun usage_advertisement(): u8 { 6 }
    public fun usage_remix(): u8 { 7 }
    public fun usage_download(): u8 { 8 }
    public fun usage_stream(): u8 { 9 }

    // === Container types (for MediaAssetUsedEvent) ===
    public fun container_post(): u8 { 1 }
    public fun container_profile(): u8 { 2 }
    public fun container_article(): u8 { 3 }

    // === U.S. copyright §106 / §114 statutory exclusive-right bitflags ===
    public fun right_reproduction(): u64 { 1 << 0 }
    public fun right_derivative_work(): u64 { 1 << 1 }
    public fun right_distribution(): u64 { 1 << 2 }
    public fun right_public_performance(): u64 { 1 << 3 }
    public fun right_public_display(): u64 { 1 << 4 }
    /// Sound recordings — digital audio transmission (17 U.S.C. §114).
    public fun right_digital_audio_transmission(): u64 { 1 << 5 }

    public fun all_statutory_rights(): u64 {
        right_reproduction() |
            right_derivative_work() |
            right_distribution() |
            right_public_performance() |
            right_public_display() |
            right_digital_audio_transmission()
    }

    // === License types (authorization form, not ownership) ===
    public fun license_non_exclusive(): u8 { 1 }
    public fun license_exclusive(): u8 { 2 }
    public fun license_royalty_free(): u8 { 3 }
    /// Synchronization — musical composition in timed relation with visual content.
    public fun license_synchronization(): u8 { 4 }
    /// Master-use — authorization for a specific sound recording.
    public fun license_master_use(): u8 { 5 }

    // === Compensation terms (economic outcome of the grant, not a global switch) ===
    public fun compensation_none(): u8 { 0 }
    public fun compensation_royalty_bearing(): u8 { 1 }
    public fun compensation_revenue_share(): u8 { 2 }
    public fun compensation_flat_fee(): u8 { 3 }

    // === Protocol claim types (asserted → verified protocol state) ===
    public fun claim_type_authorship(): u8 { 1 }
    public fun claim_type_copyright_ownership(): u8 { 2 }
    public fun claim_type_rights_control(): u8 { 3 }
    public fun claim_type_license_authority(): u8 { 4 }
    public fun claim_type_beneficiary(): u8 { 5 }

    // === Claim verification status ===
    public fun claim_unverified(): u8 { 0 }
    public fun claim_asserted(): u8 { 1 }
    public fun claim_oracle_verified(): u8 { 2 }
    public fun claim_disputed(): u8 { 3 }

    // === Creative-work kind (musical composition vs sound recording, etc.) ===
    public fun asset_kind_unspecified(): u8 { 0 }
    public fun asset_kind_visual_work(): u8 { 1 }
    public fun asset_kind_musical_composition(): u8 { 2 }
    public fun asset_kind_sound_recording(): u8 { 3 }

    // === Originality / provenance status ===
    public fun originality_unresolved(): u8 { 0 }
    public fun originality_original(): u8 { 1 }
    public fun originality_derivative(): u8 { 2 }

    public fun provenance_unverified(): u8 { 0 }
    public fun provenance_verified(): u8 { 1 }

    // === Composition status (display/use) ===
    public fun composition_none(): u8 { 0 }
    public fun composition_pending(): u8 { 1 }
    public fun composition_verified(): u8 { 2 }
    public fun composition_invalid(): u8 { 3 }
    public fun composition_partially_restricted(): u8 { 4 }

    // === Monetization status (post-level outcome after PoC composition analysis) ===
    public fun monetization_none(): u8 { 0 }
    public fun monetization_pending(): u8 { 1 }
    public fun monetization_enabled(): u8 { 2 }
    public fun monetization_restricted(): u8 { 3 }

    // === Manifest payout modes ===
    public fun payout_wallet(): u8 { 0 }
    public fun payout_escrow(): u8 { 1 }

    const MANIFEST_BPS_TOTAL: u64 = 10_000;
    const MAX_COMMITMENT_BYTES: u64 = 64;
    const MAX_CREATORS: u64 = 16;
    const MAX_BENEFICIARIES: u64 = 16;
    const MAX_MANIFEST_ENTRIES: u64 = 32;
    const MAX_COMPOSITION_ASSETS: u64 = 10;
    const MAX_USAGE_GRANTS: u64 = 16;
    const MAX_RIGHTS_INTERESTS: u64 = 16;
    const MAX_CLAIMS: u64 = 32;
    const MAX_DERIVATIVE_RELATIONSHIPS: u64 = 32;
    const MAX_ASSET_LICENSES: u64 = 16;
    const MAX_USAGE_CLASS: u8 = 9;

    /// Asserted or verified protocol claim. PoC/dispute resolution produces `RightsInterest[]`.
    public struct Claim has store, copy, drop {
        claim_type: u8,
        claimant: address,
        /// Populated after mint; `none` on finalize input.
        asset_id: Option<ID>,
        /// Statutory-right bitflags when claim_type is ownership/control; zero otherwise.
        rights_mask: u64,
        scope: u8,
        verification_status: u8,
        evidence_commitment: Option<vector<u8>>,
    }

    /// Resolved control/ownership state from verified claims — not a raw claim.
    public struct RightsInterest has store, copy, drop {
        controller: address,
        controlled_rights: u64,
        scope: u8,
    }

    /// License terms authorizing a specific product usage class.
    /// Compensation and commercial/derivative permissions live here — not on a global flag.
    public struct UsageGrant has store, copy, drop {
        usage_class: u8,
        granted_rights: u64,
        license_type: u8,
        compensation_type: u8,
        compensation_bps: u64,
        attribution_required: bool,
        derivatives_permitted: bool,
        commercial_use_permitted: bool,
        effective_from: u64,
        expires_at: Option<u64>,
        revocable: bool,
    }

    public struct BeneficiarySplit has store, copy, drop {
        beneficiary: address,
        share_bps: u64,
    }

    /// Canonical creative work. Object ID is the canonical identity.
    ///
    /// Musical compositions and sound recordings are separate works; link via `related_work_id`.
    public struct MediaAsset has key {
        id: UID,
        /// Immutable registration/evidence commitment — NOT identity.
        content_commitment: vector<u8>,
        media_type: u8,
        asset_kind: u8,
        /// Links a sound recording asset to its musical-composition asset (or vice versa).
        related_work_id: Option<ID>,
        /// Authorship — denormalized from verified authorship claims.
        creators: vector<address>,
        /// Raw protocol claims (asserted/verified).
        claims: vector<Claim>,
        /// Resolved from verified ownership/control claims.
        rights_interests: vector<RightsInterest>,
        /// Per-usage-class license grants.
        usage_grants: vector<UsageGrant>,
        /// Revenue beneficiary designation — denormalized from beneficiary claims / splits.
        beneficiaries: vector<address>,
        beneficiary_splits: vector<BeneficiarySplit>,
        rights_version: u64,
        economics_version: u64,
        provenance_status: u8,
        originality_status: u8,
        lineage_parent_id: Option<ID>,
        /// Transitive ancestor set for on-chain DAG cycle proofs.
        ancestry_metadata: AncestryMetadata,
        /// Protocol-recognized parent edges under exercised licenses.
        derivative_relationships: vector<DerivativeRelationship>,
        /// Standing license offers referencing immutable template versions.
        asset_licenses: vector<AssetLicense>,
        /// Narrow ad-hoc grants without full LicenseInstance.
        direct_usage_grants: vector<DirectUsageGrant>,
        /// Unmaterialized compose inputs persisted at Phase 2 finalize for Phase 3 resolution.
        compose_inputs: PendingComposeAccumulator,
        resolved_policy: Option<ResolvedRightsPolicy>,
        resolved_obligations: vector<ResolvedRoyaltyObligation>,
        verified_at: Option<u64>,
        poc_version: u64,
        registered_by: address,
        registered_at: u64,
        version: u64,
    }

    public struct AssetVersionCommitment has store, copy, drop {
        asset_id: ID,
        rights_version: u64,
        economics_version: u64,
        usage_class: u8,
    }

    public struct CompositionAnalysis has store, copy, drop {
        analyzed_at: u64,
        usage_context: u8,
        assets: vector<AssetVersionCommitment>,
    }

    public struct CompositionBadgeSnapshot has store, copy, drop {
        analyzed_at: u64,
        oracle_address: address,
        reasoning: Option<String>,
        evidence_urls: Option<vector<String>>,
        contains_derivatives: bool,
        contains_unresolved_assets: bool,
    }

    public struct ManifestEntry has store, copy, drop {
        beneficiary: address,
        share_bps: u64,
        source_asset_id: Option<ID>,
        payout_mode: u8,
    }

    /// Splits the creator-attributable pool only (not gross revenue).
    public struct RevenueManifest has store, copy, drop {
        entries: vector<ManifestEntry>,
    }

    // === Events ===

    public struct MediaResolutionRequestedEvent has copy, drop {
        request_id: address,
        content_commitment: vector<u8>,
        observed_fingerprint_commitment: vector<u8>,
        media_type: u8,
        submitter: address,
        timestamp: u64,
    }

    public struct MediaAssetResolvedEvent has copy, drop {
        media_asset_id: ID,
        linked_existing: bool,
        content_commitment: vector<u8>,
        media_type: u8,
        originality_status: u8,
        lineage_parent_id: Option<ID>,
        oracle_address: address,
        timestamp: u64,
    }

    public struct FingerprintLinkedEvent has copy, drop {
        media_asset_id: ID,
        fingerprint_commitment: vector<u8>,
        timestamp: u64,
    }

    public struct MediaAssetUsedEvent has copy, drop {
        container_id: address,
        container_type: u8,
        asset_id: ID,
        usage_class: u8,
        position: u8,
        timestamp: u64,
    }

    public struct MediaAssetRightsUpdatedEvent has copy, drop {
        media_asset_id: ID,
        rights_version: u64,
        timestamp: u64,
    }

    public struct MediaAssetRightsDisputeState has store, drop {
        active_proposal_id: Option<ID>,
        rights_disputes_submitted: u8,
        pending_claims_commitment: Option<vector<u8>>,
        claims_snapshot: vector<Claim>,
    }

    public struct ClaimsBundle has copy, drop {
        claims: vector<Claim>,
        usage_grants: vector<UsageGrant>,
    }

    public struct MediaAssetEconomicsUpdatedEvent has copy, drop {
        media_asset_id: ID,
        economics_version: u64,
        timestamp: u64,
    }

    public struct PostCompositionAnalyzedEvent has copy, drop {
        post_id: address,
        composition_status: u8,
        monetization_status: u8,
        timestamp: u64,
    }

    public struct MediaResolutionRequest has key {
        id: UID,
        content_commitment: vector<u8>,
        observed_fingerprint_commitment: vector<u8>,
        media_type: u8,
        submitter: address,
        requested_at: u64,
        version: u64,
    }

    public(package) fun assert_media_asset_version(asset: &MediaAsset) {
        assert!(asset.version == upgrade::current_version(), EWrongVersion);
    }

    public(package) fun media_asset_id(asset: &MediaAsset): ID {
        object::id(asset)
    }

    public(package) fun rights_version(asset: &MediaAsset): u64 {
        asset.rights_version
    }

    public(package) fun economics_version(asset: &MediaAsset): u64 {
        asset.economics_version
    }

    public(package) fun assert_valid_usage_class(usage_class: u8) {
        assert!(usage_class >= 1 && usage_class <= MAX_USAGE_CLASS, EInvalidUsageClass);
    }

    /// Protocol licensing requirement for a product usage class (not a legal conclusion).
    public(package) fun required_rights_for_usage(usage_class: u8): u64 {
        assert_valid_usage_class(usage_class);
        if (usage_class == usage_social_post()) {
            right_reproduction() | right_public_display()
        } else if (usage_class == usage_profile_picture() || usage_class == usage_cover_photo()) {
            right_reproduction() | right_public_display()
        } else if (usage_class == usage_article_embed()) {
            right_reproduction() | right_public_display() | right_distribution()
        } else if (usage_class == usage_music_soundtrack()) {
            right_reproduction() |
                right_derivative_work() |
                right_public_performance() |
                right_digital_audio_transmission()
        } else if (usage_class == usage_advertisement()) {
            right_reproduction() |
                right_public_display() |
                right_public_performance() |
                right_distribution()
        } else if (usage_class == usage_remix()) {
            right_derivative_work() | right_reproduction()
        } else if (usage_class == usage_download()) {
            right_reproduction() | right_distribution()
        } else if (usage_class == usage_stream()) {
            right_reproduction() |
                right_public_performance() |
                right_digital_audio_transmission()
        } else {
            0
        }
    }

    fun usage_requires_derivatives_flag(usage_class: u8): bool {
        usage_class == usage_remix() || usage_class == usage_music_soundtrack()
    }

    fun usage_requires_commercial_flag(usage_class: u8): bool {
        usage_class == usage_advertisement()
    }

    fun grant_is_effective(grant: &UsageGrant, now: u64): bool {
        if (now < grant.effective_from) {
            return false
        };
        if (option::is_some(&grant.expires_at)) {
            let expiry = *option::borrow(&grant.expires_at);
            if (now >= expiry) {
                return false
            };
        };
        true
    }

    fun grant_covers_usage(grant: &UsageGrant, usage_class: u8, now: u64): bool {
        if (grant.usage_class != usage_class) {
            return false
        };
        if (!grant_is_effective(grant, now)) {
            return false
        };
        let required = required_rights_for_usage(usage_class);
        if ((grant.granted_rights & required) != required) {
            return false
        };
        if (usage_requires_derivatives_flag(usage_class) && !grant.derivatives_permitted) {
            return false
        };
        if (usage_requires_commercial_flag(usage_class) && !grant.commercial_use_permitted) {
            return false
        };
        true
    }

    fun find_usage_grant(asset: &MediaAsset, usage_class: u8, now: u64): Option<UsageGrant> {
        let grants = &asset.usage_grants;
        let mut i = 0;
        let len = vector::length(grants);
        while (i < len) {
            let g = vector::borrow(grants, i);
            if (grant_covers_usage(g, usage_class, now)) {
                return option::some(*g)
            };
            i = i + 1;
        };
        option::none()
    }

    public(package) fun rights_permits_usage(asset: &MediaAsset, usage_class: u8, clock: &Clock): bool {
        assert_valid_usage_class(usage_class);
        let now = clock::timestamp_ms(clock);
        option::is_some(&find_usage_grant(asset, usage_class, now))
    }

    public(package) fun usage_requires_attribution(asset: &MediaAsset, usage_class: u8, clock: &Clock): bool {
        let now = clock::timestamp_ms(clock);
        let grant_opt = find_usage_grant(asset, usage_class, now);
        if (option::is_none(&grant_opt)) {
            return false
        };
        option::borrow(&grant_opt).attribution_required
    }

    public(package) fun usage_compensation_type(asset: &MediaAsset, usage_class: u8, clock: &Clock): Option<u8> {
        let now = clock::timestamp_ms(clock);
        let grant_opt = find_usage_grant(asset, usage_class, now);
        if (option::is_none(&grant_opt)) {
            return option::none()
        };
        option::some(option::borrow(&grant_opt).compensation_type)
    }

    public(package) fun usage_compensation_bps(asset: &MediaAsset, usage_class: u8, clock: &Clock): Option<u64> {
        let now = clock::timestamp_ms(clock);
        let grant_opt = find_usage_grant(asset, usage_class, now);
        if (option::is_none(&grant_opt)) {
            return option::none()
        };
        option::some(option::borrow(&grant_opt).compensation_bps)
    }

    /// Whether the applicable usage grant authorizes royalty/revenue-share compensation
    /// (post `monetization_status` is derived from grant terms at composition analysis time).
    public(package) fun usage_allows_paid_exploitation(asset: &MediaAsset, usage_class: u8, clock: &Clock): bool {
        let now = clock::timestamp_ms(clock);
        let grant_opt = find_usage_grant(asset, usage_class, now);
        if (option::is_none(&grant_opt)) {
            return false
        };
        let grant = option::borrow(&grant_opt);
        grant.compensation_type == compensation_royalty_bearing() ||
            grant.compensation_type == compensation_revenue_share() ||
            grant.compensation_type == compensation_flat_fee()
    }

    fun assert_valid_media_type(media_type: u8) {
        assert!(
            media_type == media_type_image() ||
                media_type == media_type_video() ||
                media_type == media_type_audio(),
            EInvalidMediaType,
        );
    }

    fun assert_valid_commitment(commitment: &vector<u8>) {
        let len = vector::length(commitment);
        assert!(len > 0 && len <= MAX_COMMITMENT_BYTES, EInvalidCommitment);
    }

    public(package) fun assert_valid_optional_evidence_commitment(
        commitment: &Option<vector<u8>>,
    ) {
        if (option::is_some(commitment)) {
            assert_valid_commitment(option::borrow(commitment));
        };
    }

    fun assert_beneficiary_splits_sum_to_total(splits: &vector<BeneficiarySplit>) {
        let len = vector::length(splits);
        assert!(len > 0 && len <= MAX_BENEFICIARIES, EInvalidBeneficiarySplits);
        let mut i = 0;
        let mut total = 0u64;
        while (i < len) {
            let s = vector::borrow(splits, i);
            total = total + s.share_bps;
            i = i + 1;
        };
        assert!(total == MANIFEST_BPS_TOTAL, EInvalidBeneficiarySplits);
    }

    fun assert_valid_asset_kind(asset_kind: u8) {
        assert!(
            asset_kind == asset_kind_unspecified() ||
                asset_kind == asset_kind_visual_work() ||
                asset_kind == asset_kind_musical_composition() ||
                asset_kind == asset_kind_sound_recording(),
            EInvalidAssetKind,
        );
    }

    fun claim_is_verified(claim: &Claim): bool {
        claim.verification_status == claim_oracle_verified()
    }

    fun assert_valid_claim(claim: &Claim) {
        let t = claim.claim_type;
        assert!(
            t == claim_type_authorship() ||
                t == claim_type_copyright_ownership() ||
                t == claim_type_rights_control() ||
                t == claim_type_license_authority() ||
                t == claim_type_beneficiary(),
            EInvalidClaim,
        );
        if (t == claim_type_copyright_ownership() || t == claim_type_rights_control()) {
            assert!(claim.rights_mask != 0, EInvalidClaim);
        };
        if (option::is_some(&claim.evidence_commitment)) {
            assert_valid_commitment(option::borrow(&claim.evidence_commitment));
        };
    }

    fun assert_valid_claims(claims: &vector<Claim>) {
        let len = vector::length(claims);
        assert!(len > 0 && len <= MAX_CLAIMS, ETooManyClaims);
        let mut i = 0;
        let mut has_authorship = false;
        while (i < len) {
            let c = vector::borrow(claims, i);
            assert_valid_claim(c);
            if (c.claim_type == claim_type_authorship() && claim_is_verified(c)) {
                has_authorship = true;
            };
            i = i + 1;
        };
        assert!(has_authorship, ENoVerifiedAuthorship);
    }

    fun assert_all_claims_oracle_verified(claims: &vector<Claim>) {
        let len = vector::length(claims);
        let mut i = 0;
        while (i < len) {
            let c = vector::borrow(claims, i);
            assert!(claim_is_verified(c), EClaimsRequireOracleVerification);
            i = i + 1;
        };
    }

    fun assert_valid_claims_for_oracle(claims: &vector<Claim>) {
        assert_valid_claims(claims);
        assert_all_claims_oracle_verified(claims);
    }

    /// Verified authorship claims → denormalized creator list.
    fun resolve_creators_from_claims(claims: &vector<Claim>): vector<address> {
        let mut creators = vector[];
        let len = vector::length(claims);
        let mut i = 0;
        while (i < len) {
            let c = vector::borrow(claims, i);
            if (c.claim_type == claim_type_authorship() && claim_is_verified(c)) {
                if (!vector::contains(&creators, &c.claimant)) {
                    vector::push_back(&mut creators, c.claimant);
                };
            };
            i = i + 1;
        };
        creators
    }

    /// Verified ownership/control claims → resolved `RightsInterest[]` (merged per holder).
    fun resolve_rights_interests_from_claims(claims: &vector<Claim>): vector<RightsInterest> {
        let mut interests: vector<RightsInterest> = vector[];
        let len = vector::length(claims);
        let mut i = 0;
        while (i < len) {
            let c = vector::borrow(claims, i);
            if (!claim_is_verified(c)) {
                i = i + 1;
                continue
            };
            if (c.claim_type == claim_type_copyright_ownership() ||
                c.claim_type == claim_type_rights_control()) {
                let mut found = false;
                let mut j = 0;
                let ilen = vector::length(&interests);
                while (j < ilen) {
                    let interest = vector::borrow_mut(&mut interests, j);
                    if (interest.controller == c.claimant && interest.scope == c.scope) {
                        interest.controlled_rights = interest.controlled_rights | c.rights_mask;
                        found = true;
                        break
                    };
                    j = j + 1;
                };
                if (!found) {
                    vector::push_back(
                        &mut interests,
                        RightsInterest {
                            controller: c.claimant,
                            controlled_rights: c.rights_mask,
                            scope: c.scope,
                        },
                    );
                };
            };
            i = i + 1;
        };
        interests
    }

    fun resolve_beneficiaries_from_claims(claims: &vector<Claim>): vector<address> {
        let mut beneficiaries = vector[];
        let len = vector::length(claims);
        let mut i = 0;
        while (i < len) {
            let c = vector::borrow(claims, i);
            if (c.claim_type == claim_type_beneficiary() && claim_is_verified(c)) {
                if (!vector::contains(&beneficiaries, &c.claimant)) {
                    vector::push_back(&mut beneficiaries, c.claimant);
                };
            };
            i = i + 1;
        };
        beneficiaries
    }

    fun beneficiaries_from_splits(splits: &vector<BeneficiarySplit>): vector<address> {
        let mut beneficiaries = vector[];
        let len = vector::length(splits);
        let mut i = 0;
        while (i < len) {
            let s = vector::borrow(splits, i);
            if (!vector::contains(&beneficiaries, &s.beneficiary)) {
                vector::push_back(&mut beneficiaries, s.beneficiary);
            };
            i = i + 1;
        };
        beneficiaries
    }

    fun stamp_claim_asset_ids(claims: &mut vector<Claim>, asset_id: ID) {
        let len = vector::length(claims);
        let mut i = 0;
        while (i < len) {
            let c = vector::borrow_mut(claims, i);
            c.asset_id = option::some(asset_id);
            i = i + 1;
        };
    }

    fun assert_valid_usage_grants(grants: &vector<UsageGrant>, now: u64, max_compensation_bps: u64) {
        let len = vector::length(grants);
        assert!(len > 0 && len <= MAX_USAGE_GRANTS, ETooManyUsageGrants);
        let mut i = 0;
        while (i < len) {
            let g = vector::borrow(grants, i);
            assert_valid_usage_class(g.usage_class);
            assert!(grant_covers_usage(g, g.usage_class, now), EInvalidRightsGrant);
            assert!(g.compensation_bps <= max_compensation_bps, EInvalidManifest);
            i = i + 1;
        };
    }

    fun assert_valid_rights_interests(interests: &vector<RightsInterest>) {
        let len = vector::length(interests);
        assert!(len > 0 && len <= MAX_RIGHTS_INTERESTS, ETooManyRightsInterests);
    }

    #[test_only]
    fun default_claim(claim_type: u8, claimant: address, rights_mask: u64): Claim {
        Claim {
            claim_type,
            claimant,
            asset_id: option::none(),
            rights_mask,
            scope: 0,
            verification_status: claim_oracle_verified(),
            evidence_commitment: option::none(),
        }
    }

    /// Standard V1 claim bundle for a sole creator registering original work (oracle path only).
    #[test_only]
    fun default_claims(owner: address): vector<Claim> {
        vector[
            default_claim(claim_type_authorship(), owner, 0),
            default_claim(claim_type_rights_control(), owner, all_statutory_rights()),
            default_claim(claim_type_license_authority(), owner, 0),
        ]
    }

    fun default_usage_grant(usage_class: u8, effective_from: u64): UsageGrant {
        UsageGrant {
            usage_class,
            granted_rights: required_rights_for_usage(usage_class),
            license_type: license_non_exclusive(),
            compensation_type: compensation_revenue_share(),
            compensation_bps: MANIFEST_BPS_TOTAL,
            attribution_required: false,
            derivatives_permitted: true,
            commercial_use_permitted: true,
            effective_from,
            expires_at: option::none(),
            revocable: true,
        }
    }

    /// Permissive V1 defaults: non-exclusive grants for standard product usage classes.
    fun default_usage_grants(effective_from: u64): vector<UsageGrant> {
        vector[
            default_usage_grant(usage_social_post(), effective_from),
            default_usage_grant(usage_profile_picture(), effective_from),
            default_usage_grant(usage_cover_photo(), effective_from),
            default_usage_grant(usage_article_embed(), effective_from),
            default_usage_grant(usage_music_soundtrack(), effective_from),
            default_usage_grant(usage_advertisement(), effective_from),
            default_usage_grant(usage_remix(), effective_from),
            default_usage_grant(usage_download(), effective_from),
            default_usage_grant(usage_stream(), effective_from),
        ]
    }

    fun default_beneficiary_splits(owner: address): vector<BeneficiarySplit> {
        vector[BeneficiarySplit { beneficiary: owner, share_bps: MANIFEST_BPS_TOTAL }]
    }

    fun is_rights_controller(asset: &MediaAsset, addr: address): bool {
        let interests = &asset.rights_interests;
        let mut i = 0;
        let len = vector::length(interests);
        while (i < len) {
            let interest = vector::borrow(interests, i);
            if (interest.controller == addr) {
                return true
            };
            i = i + 1;
        };
        false
    }

    fun has_license_authority(asset: &MediaAsset, addr: address): bool {
        let claims = &asset.claims;
        let mut i = 0;
        let len = vector::length(claims);
        while (i < len) {
            let c = vector::borrow(claims, i);
            if (c.claim_type == claim_type_license_authority() &&
                c.claimant == addr &&
                claim_is_verified(c)) {
                return true
            };
            i = i + 1;
        };
        false
    }

    fun has_verified_beneficiary_claim(asset: &MediaAsset, addr: address): bool {
        let claims = &asset.claims;
        let mut i = 0;
        let len = vector::length(claims);
        while (i < len) {
            let c = vector::borrow(claims, i);
            if (c.claim_type == claim_type_beneficiary() &&
                c.claimant == addr &&
                claim_is_verified(c)) {
                return true
            };
            i = i + 1;
        };
        false
    }

    fun can_update_economics(asset: &MediaAsset, addr: address): bool {
        is_rights_controller(asset, addr) ||
            has_license_authority(asset, addr) ||
            has_verified_beneficiary_claim(asset, addr) ||
            vector::contains(&asset.creators, &addr)
    }

    public(package) fun can_update_rights(asset: &MediaAsset, addr: address): bool {
        is_rights_controller(asset, addr) || has_license_authority(asset, addr)
    }

    /// Anyone may request off-chain/oracle resolution for an uploaded representation.
    public entry fun submit_media_resolution(
        content_commitment: vector<u8>,
        observed_fingerprint_commitment: vector<u8>,
        media_type: u8,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert_valid_commitment(&content_commitment);
        assert_valid_commitment(&observed_fingerprint_commitment);
        assert_valid_media_type(media_type);

        let submitter = tx_context::sender(ctx);
        let request = MediaResolutionRequest {
            id: object::new(ctx),
            content_commitment,
            observed_fingerprint_commitment,
            media_type,
            submitter,
            requested_at: clock::timestamp_ms(clock),
            version: upgrade::current_version(),
        };
        let request_id = object::uid_to_address(&request.id);
        let content_commitment_copy = request.content_commitment;
        let fingerprint_copy = request.observed_fingerprint_commitment;
        let media_type_copy = request.media_type;

        event::emit(MediaResolutionRequestedEvent {
            request_id,
            content_commitment: content_commitment_copy,
            observed_fingerprint_commitment: fingerprint_copy,
            media_type: media_type_copy,
            submitter,
            timestamp: clock::timestamp_ms(clock),
        });

        transfer::share_object(request);
    }

    /// Oracle-only: mint or link a canonical `MediaAsset` from a resolution request.
    /// Invoked via `proof_of_creativity::finalize_media_asset` (the public entry wrapper).
    public(package) fun finalize_media_asset(
        oracle_address: address,
        request: MediaResolutionRequest,
        link_to_existing_id: Option<ID>,
        originality_status: u8,
        lineage_parent_id: Option<ID>,
        asset_kind: u8,
        related_work_id: Option<ID>,
        claims: vector<Claim>,
        usage_grants: vector<UsageGrant>,
        beneficiary_splits: vector<BeneficiarySplit>,
        max_compensation_bps: u64,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert_oracle(oracle_address, ctx);
        assert_valid_asset_kind(asset_kind);
        assert_valid_claims_for_oracle(&claims);
        let now = clock::timestamp_ms(clock);
        assert_valid_usage_grants(&usage_grants, now, max_compensation_bps);
        assert_beneficiary_splits_sum_to_total(&beneficiary_splits);

        let creators = resolve_creators_from_claims(&claims);
        assert!(vector::length(&creators) > 0 && vector::length(&creators) <= MAX_CREATORS, EEmptyCreators);
        let rights_interests = resolve_rights_interests_from_claims(&claims);
        assert_valid_rights_interests(&rights_interests);

        let claim_beneficiaries = resolve_beneficiaries_from_claims(&claims);
        let split_beneficiaries = beneficiaries_from_splits(&beneficiary_splits);
        let beneficiaries = if (vector::length(&claim_beneficiaries) > 0) {
            claim_beneficiaries
        } else {
            split_beneficiaries
        };

        let MediaResolutionRequest {
            id,
            content_commitment,
            observed_fingerprint_commitment,
            media_type,
            submitter,
            requested_at: _,
            version: _,
        } = request;
        object::delete(id);

        let oracle = tx_context::sender(ctx);

        if (option::is_some(&link_to_existing_id)) {
            let existing_id = *option::borrow(&link_to_existing_id);
            event::emit(FingerprintLinkedEvent {
                media_asset_id: existing_id,
                fingerprint_commitment: observed_fingerprint_commitment,
                timestamp: now,
            });
            event::emit(MediaAssetResolvedEvent {
                media_asset_id: existing_id,
                linked_existing: true,
                content_commitment,
                media_type,
                originality_status,
                lineage_parent_id,
                oracle_address: oracle,
                timestamp: now,
            });
        } else {
            let (compose_inputs, resolved_policy, resolved_obligations) = default_compose_policy_fields();
            let mut asset = MediaAsset {
                id: object::new(ctx),
                content_commitment,
                media_type,
                asset_kind,
                related_work_id,
                creators,
                claims,
                rights_interests,
                usage_grants,
                beneficiaries,
                beneficiary_splits,
                rights_version: 1,
                economics_version: 1,
                provenance_status: provenance_verified(),
                originality_status,
                lineage_parent_id,
                ancestry_metadata: derivative_graph::empty_ancestry(),
                derivative_relationships: vector[],
                asset_licenses: vector[],
                direct_usage_grants: vector[],
                compose_inputs,
                resolved_policy,
                resolved_obligations,
                verified_at: option::some(now),
                poc_version: 1,
                registered_by: submitter,
                registered_at: now,
                version: upgrade::current_version(),
            };
            let asset_id = object::id(&asset);
            stamp_claim_asset_ids(&mut asset.claims, asset_id);
            event::emit(FingerprintLinkedEvent {
                media_asset_id: asset_id,
                fingerprint_commitment: observed_fingerprint_commitment,
                timestamp: now,
            });
            event::emit(MediaAssetResolvedEvent {
                media_asset_id: asset_id,
                linked_existing: false,
                content_commitment: asset.content_commitment,
                media_type: asset.media_type,
                originality_status,
                lineage_parent_id,
                oracle_address: oracle,
                timestamp: now,
            });
            transfer::share_object(asset);
        };
    }

    /// Rights controller or license authority: update product usage grants only (prospective).
    public fun update_media_asset_usage_grants(
        asset: &mut MediaAsset,
        usage_grants: vector<UsageGrant>,
        max_compensation_bps: u64,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert_media_asset_version(asset);
        let sender = tx_context::sender(ctx);
        assert!(can_update_rights(asset, sender), EUnauthorized);
        let now = clock::timestamp_ms(clock);
        assert_valid_usage_grants(&usage_grants, now, max_compensation_bps);
        asset.usage_grants = usage_grants;
        asset.rights_version = asset.rights_version + 1;
        event::emit(MediaAssetRightsUpdatedEvent {
            media_asset_id: object::id(asset),
            rights_version: asset.rights_version,
            timestamp: now,
        });
    }

    /// Oracle-only: replace verified claims and optional usage grants (dispute/provenance path).
    public(package) fun oracle_update_media_asset_claims(
        oracle_address: address,
        asset: &mut MediaAsset,
        claims: vector<Claim>,
        usage_grants: vector<UsageGrant>,
        max_compensation_bps: u64,
        clock: &Clock,
        ctx: &TxContext,
    ) {
        assert_oracle(oracle_address, ctx);
        assert_media_asset_version(asset);
        assert_valid_claims_for_oracle(&claims);
        let now = clock::timestamp_ms(clock);
        assert_valid_usage_grants(&usage_grants, now, max_compensation_bps);

        let creators = resolve_creators_from_claims(&claims);
        assert!(vector::length(&creators) > 0, EEmptyCreators);
        let rights_interests = resolve_rights_interests_from_claims(&claims);
        assert_valid_rights_interests(&rights_interests);

        let asset_id = object::id(asset);
        let mut stamped_claims = claims;
        stamp_claim_asset_ids(&mut stamped_claims, asset_id);

        asset.claims = stamped_claims;
        asset.creators = creators;
        asset.rights_interests = rights_interests;
        asset.usage_grants = usage_grants;
        asset.rights_version = asset.rights_version + 1;
        event::emit(MediaAssetRightsUpdatedEvent {
            media_asset_id: asset_id,
            rights_version: asset.rights_version,
            timestamp: now,
        });
    }

    fun mark_claims_disputed(claims: &mut vector<Claim>) {
        let len = vector::length(claims);
        let mut i = 0;
        while (i < len) {
            let c = vector::borrow_mut(claims, i);
            c.verification_status = claim_disputed();
            i = i + 1;
        };
    }

    public(package) fun borrow_rights_dispute_state(asset: &MediaAsset): &MediaAssetRightsDisputeState {
        assert!(df::exists_(&asset.id, RIGHTS_DISPUTE_DF_KEY), ENoActiveRightsProposal);
        df::borrow<vector<u8>, MediaAssetRightsDisputeState>(&asset.id, RIGHTS_DISPUTE_DF_KEY)
    }

    public(package) fun borrow_rights_dispute_state_mut(asset: &mut MediaAsset): &mut MediaAssetRightsDisputeState {
        assert!(df::exists_(&asset.id, RIGHTS_DISPUTE_DF_KEY), ENoActiveRightsProposal);
        df::borrow_mut<vector<u8>, MediaAssetRightsDisputeState>(&mut asset.id, RIGHTS_DISPUTE_DF_KEY)
    }

    public(package) fun rights_disputes_submitted(asset: &MediaAsset): u8 {
        if (!df::exists_(&asset.id, RIGHTS_DISPUTE_DF_KEY)) {
            return 0
        };
        df::borrow<vector<u8>, MediaAssetRightsDisputeState>(&asset.id, RIGHTS_DISPUTE_DF_KEY).rights_disputes_submitted
    }

    public(package) fun assert_no_active_rights_proposal(asset: &MediaAsset) {
        if (!df::exists_(&asset.id, RIGHTS_DISPUTE_DF_KEY)) {
            return
        };
        let state = df::borrow<vector<u8>, MediaAssetRightsDisputeState>(&asset.id, RIGHTS_DISPUTE_DF_KEY);
        assert!(option::is_none(&state.active_proposal_id), EActiveRightsProposal);
    }

    public(package) fun link_rights_proposal(
        asset: &mut MediaAsset,
        proposal_id: ID,
        claims_commitment: vector<u8>,
        max_disputes: u8,
    ) {
        assert!(asset.rights_version >= 1, EUnauthorized);
        assert_no_active_rights_proposal(asset);
        let submitted = rights_disputes_submitted(asset);
        assert!(submitted < max_disputes, ERightsDisputeCapReached);

        let snapshot = asset.claims;
        mark_claims_disputed(&mut asset.claims);

        if (!df::exists_(&asset.id, RIGHTS_DISPUTE_DF_KEY)) {
            df::add(
                &mut asset.id,
                RIGHTS_DISPUTE_DF_KEY,
                MediaAssetRightsDisputeState {
                    active_proposal_id: option::some(proposal_id),
                    rights_disputes_submitted: submitted + 1,
                    pending_claims_commitment: option::some(claims_commitment),
                    claims_snapshot: snapshot,
                },
            );
        } else {
            let state = df::borrow_mut<vector<u8>, MediaAssetRightsDisputeState>(&mut asset.id, RIGHTS_DISPUTE_DF_KEY);
            state.active_proposal_id = option::some(proposal_id);
            state.rights_disputes_submitted = submitted + 1;
            state.pending_claims_commitment = option::some(claims_commitment);
            state.claims_snapshot = snapshot;
        };
    }

    public(package) fun clear_rights_proposal_on_reject(asset: &mut MediaAsset, proposal_id: ID) {
        assert!(df::exists_(&asset.id, RIGHTS_DISPUTE_DF_KEY), ENoActiveRightsProposal);
        let state = df::borrow_mut<vector<u8>, MediaAssetRightsDisputeState>(&mut asset.id, RIGHTS_DISPUTE_DF_KEY);
        assert!(option::is_some(&state.active_proposal_id), ENoActiveRightsProposal);
        assert!(*option::borrow(&state.active_proposal_id) == proposal_id, EActiveRightsProposal);
        asset.claims = state.claims_snapshot;
        state.active_proposal_id = option::none();
        state.pending_claims_commitment = option::none();
        state.claims_snapshot = vector[];
    }

    public(package) fun clear_rights_proposal_on_implement(asset: &mut MediaAsset, proposal_id: ID) {
        assert!(df::exists_(&asset.id, RIGHTS_DISPUTE_DF_KEY), ENoActiveRightsProposal);
        let state = df::borrow_mut<vector<u8>, MediaAssetRightsDisputeState>(&mut asset.id, RIGHTS_DISPUTE_DF_KEY);
        assert!(option::is_some(&state.active_proposal_id), ENoActiveRightsProposal);
        assert!(*option::borrow(&state.active_proposal_id) == proposal_id, EActiveRightsProposal);
        state.active_proposal_id = option::none();
        state.pending_claims_commitment = option::none();
        state.claims_snapshot = vector[];
    }

    public(package) fun pending_claims_commitment(asset: &MediaAsset): vector<u8> {
        let state = borrow_rights_dispute_state(asset);
        assert!(option::is_some(&state.pending_claims_commitment), EInvalidClaim);
        *option::borrow(&state.pending_claims_commitment)
    }

    public(package) fun assert_registered_asset(asset: &MediaAsset) {
        assert!(asset.rights_version >= 1, EUnauthorized);
    }

    public(package) fun compute_claims_bundle_commitment(
        claims: &vector<Claim>,
        usage_grants: &vector<UsageGrant>,
    ): vector<u8> {
        let bundle = ClaimsBundle { claims: *claims, usage_grants: *usage_grants };
        hash::sha3_256(std::bcs::to_bytes(&bundle))
    }

    /// Rights controller, license authority, verified beneficiary, or creator: update economics.
    public fun update_media_asset_economics(
        asset: &mut MediaAsset,
        beneficiary_splits: vector<BeneficiarySplit>,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert_media_asset_version(asset);
        let sender = tx_context::sender(ctx);
        assert!(can_update_economics(asset, sender), EUnauthorized);
        assert_beneficiary_splits_sum_to_total(&beneficiary_splits);
        asset.beneficiaries = beneficiaries_from_splits(&beneficiary_splits);
        asset.beneficiary_splits = beneficiary_splits;
        asset.economics_version = asset.economics_version + 1;
        event::emit(MediaAssetEconomicsUpdatedEvent {
            media_asset_id: object::id(asset),
            economics_version: asset.economics_version,
            timestamp: clock::timestamp_ms(clock),
        });
    }

    public(package) fun emit_media_asset_used(
        container_id: address,
        container_type: u8,
        asset_id: ID,
        usage_class: u8,
        position: u8,
        clock: &Clock,
    ) {
        event::emit(MediaAssetUsedEvent {
            container_id,
            container_type,
            asset_id,
            usage_class,
            position,
            timestamp: clock::timestamp_ms(clock),
        });
    }

    public(package) fun assert_usage_permitted(asset: &MediaAsset, usage_class: u8, clock: &Clock) {
        assert!(rights_permits_usage(asset, usage_class, clock), EUsageNotPermitted);
    }

    public(package) fun validate_revenue_manifest(manifest: &RevenueManifest) {
        let entries = &manifest.entries;
        let len = vector::length(entries);
        assert!(len > 0 && len <= MAX_MANIFEST_ENTRIES, EInvalidManifest);
        let mut i = 0;
        let mut total = 0u64;
        while (i < len) {
            let e = vector::borrow(entries, i);
            assert!(e.share_bps > 0, EInvalidManifest);
            assert!(
                option::is_none(&e.source_asset_id) || option::is_some(&e.source_asset_id),
                EInvalidManifest,
            );
            total = total + e.share_bps;
            i = i + 1;
        };
        assert!(total == MANIFEST_BPS_TOTAL, EInvalidManifest);
    }

    /// Clamp embedded-source manifest slices to the ecosystem max; excess returns to post-creator entries.
    public(package) fun clamp_manifest_embedded_redirect_cap(
        manifest: RevenueManifest,
        max_bps_per_source: u64,
    ): RevenueManifest {
        let RevenueManifest { mut entries } = manifest;
        let len = vector::length(&entries);
        let mut reclaimed = 0u64;
        let mut i = 0;
        while (i < len) {
            let e = vector::borrow_mut(&mut entries, i);
            if (option::is_some(&e.source_asset_id) && e.share_bps > max_bps_per_source) {
                reclaimed = reclaimed + (e.share_bps - max_bps_per_source);
                e.share_bps = max_bps_per_source;
            };
            i = i + 1;
        };
        if (reclaimed > 0) {
            let mut j = 0;
            while (j < len && reclaimed > 0) {
                let e = vector::borrow_mut(&mut entries, j);
                if (option::is_none(&e.source_asset_id)) {
                    e.share_bps = e.share_bps + reclaimed;
                    reclaimed = 0;
                };
                j = j + 1;
            };
            assert!(reclaimed == 0, EInvalidManifest);
        };
        validate_revenue_manifest(&RevenueManifest { entries });
        RevenueManifest { entries }
    }

    /// Monetization is enabled when the manifest routes any share to an embedded source asset.
    public(package) fun derive_monetization_status_from_manifest(manifest: &Option<RevenueManifest>): u8 {
        if (option::is_none(manifest)) {
            return monetization_none()
        };
        let entries = manifest_entries(option::borrow(manifest));
        let len = vector::length(entries);
        let mut i = 0;
        while (i < len) {
            let e = vector::borrow(entries, i);
            if (option::is_some(&e.source_asset_id) && e.share_bps > 0) {
                return monetization_enabled()
            };
            i = i + 1;
        };
        monetization_none()
    }

    public(package) fun composition_analysis_matches_assets(
        analysis: &CompositionAnalysis,
        assets: &vector<ID>,
    ): bool {
        if (analysis.analyzed_at == 0 || analysis.usage_context == 0) {
            return false
        };
        let commitments = &analysis.assets;
        if (vector::length(commitments) != vector::length(assets)) {
            return false
        };
        let mut i = 0;
        let len = vector::length(assets);
        while (i < len) {
            let c = vector::borrow(commitments, i);
            let expected = vector::borrow(assets, i);
            if (c.asset_id != *expected || c.usage_class == 0) {
                return false
            };
            i = i + 1;
        };
        true
    }

    public(package) fun composition_analysis_is_current(
        analysis: &CompositionAnalysis,
        live_assets: &vector<MediaAsset>,
    ): bool {
        if (analysis.analyzed_at == 0 || analysis.usage_context == 0) {
            return false
        };
        let commitments = &analysis.assets;
        let len = vector::length(commitments);
        assert!(vector::length(live_assets) == len, EVersionMismatch);
        let mut i = 0;
        while (i < len) {
            let c = vector::borrow(commitments, i);
            let live = vector::borrow(live_assets, i);
            if (c.rights_version != live.rights_version ||
                c.economics_version != live.economics_version) {
                return false
            };
            i = i + 1;
        };
        true
    }

    public(package) fun manifest_bps_total(): u64 { MANIFEST_BPS_TOTAL }

    public(package) fun manifest_entries(manifest: &RevenueManifest): &vector<ManifestEntry> {
        &manifest.entries
    }

    public(package) fun manifest_entry_beneficiary(entry: &ManifestEntry): address {
        entry.beneficiary
    }

    public(package) fun manifest_entry_share_bps(entry: &ManifestEntry): u64 {
        entry.share_bps
    }

    public(package) fun manifest_entry_payout_mode(entry: &ManifestEntry): u8 {
        entry.payout_mode
    }

    public(package) fun manifest_entry_source_asset_id(entry: &ManifestEntry): Option<ID> {
        entry.source_asset_id
    }

    /// Whether resolved graph policy permits the usage class (no overlay when policy absent).
    public(package) fun resolved_policy_permits_usage(asset: &MediaAsset, usage_class: u8): bool {
        if (option::is_none(&asset.resolved_policy)) {
            return true
        };
        let policy = option::borrow(&asset.resolved_policy);
        let required = required_rights_for_usage(usage_class);
        if ((policy.effective_rights & required) != required) {
            return false
        };
        if (usage_requires_commercial_flag(usage_class) && !policy.commercial_allowed) {
            return false
        };
        if (usage_requires_derivatives_flag(usage_class) && !policy.derivatives_allowed) {
            return false
        };
        true
    }

    public(package) fun new_manifest_entry(
        beneficiary: address,
        share_bps: u64,
        payout_mode: u8,
    ): ManifestEntry {
        ManifestEntry {
            beneficiary,
            share_bps,
            source_asset_id: option::none(),
            payout_mode,
        }
    }

    public(package) fun new_revenue_manifest(entries: vector<ManifestEntry>): RevenueManifest {
        RevenueManifest { entries }
    }

    public(package) fun manifest_has_escrow_payout(manifest: &RevenueManifest, amount: u64): bool {
        if (amount == 0) {
            return false
        };
        let entries = manifest_entries(manifest);
        let len = vector::length(entries);
        let bps_total = manifest_bps_total();
        let mut i = 0;
        while (i < len) {
            let e = vector::borrow(entries, i);
            if (manifest_entry_payout_mode(e) == payout_escrow()) {
                let slice = (amount * manifest_entry_share_bps(e)) / bps_total;
                if (slice > 0) {
                    return true
                };
            };
            i = i + 1;
        };
        false
    }

    /// True when any manifest entry routes through escrow (ignores payment amount rounding).
    public(package) fun manifest_uses_escrow_redirect(manifest: &RevenueManifest): bool {
        let entries = manifest_entries(manifest);
        let len = vector::length(entries);
        let mut i = 0;
        while (i < len) {
            let e = vector::borrow(entries, i);
            if (
                manifest_entry_payout_mode(e) == payout_escrow()
                    && manifest_entry_share_bps(e) > 0
            ) {
                return true
            };
            i = i + 1;
        };
        false
    }

    public(package) fun manifest_redirect_beneficiary(
        manifest: &RevenueManifest,
        owner: address,
    ): Option<address> {
        let entries = manifest_entries(manifest);
        let len = vector::length(entries);
        let mut i = 0;
        while (i < len) {
            let e = vector::borrow(entries, i);
            let beneficiary = manifest_entry_beneficiary(e);
            if (beneficiary != owner) {
                return option::some(beneficiary)
            };
            i = i + 1;
        };
        option::none()
    }

    public(package) fun manifest_redirect_percentage(
        manifest: &RevenueManifest,
        owner: address,
    ): Option<u64> {
        let entries = manifest_entries(manifest);
        let len = vector::length(entries);
        let mut i = 0;
        while (i < len) {
            let e = vector::borrow(entries, i);
            let beneficiary = manifest_entry_beneficiary(e);
            if (beneficiary != owner) {
                return option::some(manifest_entry_share_bps(e) / 100)
            };
            i = i + 1;
        };
        option::none()
    }

    public(package) fun new_composition_badge_snapshot(
        analyzed_at: u64,
        oracle_address: address,
        reasoning: Option<String>,
        evidence_urls: Option<vector<String>>,
        contains_derivatives: bool,
        contains_unresolved_assets: bool,
    ): CompositionBadgeSnapshot {
        CompositionBadgeSnapshot {
            analyzed_at,
            oracle_address,
            reasoning,
            evidence_urls,
            contains_derivatives,
            contains_unresolved_assets,
        }
    }

    public(package) fun emit_post_composition_analyzed(
        post_id: address,
        composition_status: u8,
        monetization_status: u8,
        timestamp: u64,
    ) {
        event::emit(PostCompositionAnalyzedEvent {
            post_id,
            composition_status,
            monetization_status,
            timestamp,
        });
    }

    fun assert_oracle(oracle_address: address, ctx: &TxContext) {
        assert!(tx_context::sender(ctx) == oracle_address, ENotOracle);
    }

    public(package) fun max_composition_assets(): u64 { MAX_COMPOSITION_ASSETS }

    fun default_compose_policy_fields(): (
        PendingComposeAccumulator,
        Option<ResolvedRightsPolicy>,
        vector<ResolvedRoyaltyObligation>,
    ) {
        (
            derivative_graph::empty_compose_accumulator(),
            option::none(),
            vector[],
        )
    }

    fun default_graph_fields(): (
        AncestryMetadata,
        vector<DerivativeRelationship>,
        vector<AssetLicense>,
        vector<DirectUsageGrant>,
        PendingComposeAccumulator,
        Option<ResolvedRightsPolicy>,
        vector<ResolvedRoyaltyObligation>,
    ) {
        let (compose_inputs, resolved_policy, resolved_obligations) = default_compose_policy_fields();
        (
            derivative_graph::empty_ancestry(),
            vector[],
            vector[],
            vector[],
            compose_inputs,
            resolved_policy,
            resolved_obligations,
        )
    }

    /// Mint a derived work with empty ancestry. Parent edges added via `add_derivative_parent_edge`.
    public entry fun create_derivative_asset(
        content_commitment: vector<u8>,
        media_type: u8,
        asset_kind: u8,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert_valid_commitment(&content_commitment);
        assert_valid_media_type(media_type);
        let creator = tx_context::sender(ctx);
        let now = clock::timestamp_ms(clock);
        let (ancestry_metadata, derivative_relationships, asset_licenses, direct_usage_grants, compose_inputs, resolved_policy, resolved_obligations) =
            default_graph_fields();
        let asset = MediaAsset {
            id: object::new(ctx),
            content_commitment,
            media_type,
            asset_kind,
            related_work_id: option::none(),
            creators: vector[creator],
            claims: vector[],
            rights_interests: vector[],
            usage_grants: default_usage_grants(now),
            beneficiaries: vector[creator],
            beneficiary_splits: default_beneficiary_splits(creator),
            rights_version: 1,
            economics_version: 1,
            provenance_status: provenance_verified(),
            originality_status: originality_derivative(),
            lineage_parent_id: option::none(),
            ancestry_metadata,
            derivative_relationships,
            asset_licenses,
            direct_usage_grants,
            compose_inputs,
            resolved_policy,
            resolved_obligations,
            verified_at: option::some(now),
            poc_version: 1,
            registered_by: creator,
            registered_at: now,
            version: upgrade::current_version(),
        };
        transfer::share_object(asset);
    }

    /// Add one parent edge under an active exercised license. Royalties derived from pinned template.
    public entry fun add_derivative_parent_edge(
        child: &mut MediaAsset,
        parent: &MediaAsset,
        license_instance: &LicenseInstance,
        template_version: &LicenseTemplateVersion,
        relationship_type: u8,
        evidence_commitment: Option<vector<u8>>,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert_media_asset_version(child);
        assert_media_asset_version(parent);
        license_tpl::assert_instance_matches_template(license_instance, template_version);
        license_tpl::assert_instance_active(license_instance, clock);
        assert!(
            license_tpl::instance_licensor_asset_id(license_instance) == object::id(parent),
            EUnauthorized,
        );
        assert!(tx_context::sender(ctx) == license_tpl::instance_licensee(license_instance), EUnauthorized);
        assert!(license_tpl::template_allow_derivatives(template_version), EInvalidRightsGrant);

        let parent_id = object::id(parent);
        let child_id = object::id(child);
        derivative_graph::assert_valid_parent_child_edge(
            &parent_id,
            &child_id,
            derivative_graph::ancestor_ids(&parent.ancestry_metadata),
        );
        derivative_graph::assert_relationship_unique(
            &child.derivative_relationships,
            parent_id,
            child_id,
            relationship_type,
            license_tpl::license_instance_id(license_instance),
        );

        let parent_share_bps = license_tpl::derive_parent_share_bps(template_version, relationship_type);
        let relationship_id = vector::length(&child.derivative_relationships) + 1;
        let edge = derivative_graph::new_derivative_relationship(
            relationship_id,
            parent_id,
            child_id,
            relationship_type,
            license_tpl::license_instance_id(license_instance),
            license_tpl::template_version_id(template_version),
            parent_share_bps,
            evidence_commitment,
            clock::timestamp_ms(clock),
        );
        derivative_graph::merge_ancestry_for_edge(
            &mut child.ancestry_metadata,
            parent_id,
            derivative_graph::ancestor_ids(&parent.ancestry_metadata),
        );
        assert!(
            vector::length(&child.derivative_relationships) < MAX_DERIVATIVE_RELATIONSHIPS,
            EInvalidRightsGrant,
        );
        vector::push_back(&mut child.derivative_relationships, edge);
        child.rights_version = child.rights_version + 1;
        event::emit(MediaAssetRightsUpdatedEvent {
            media_asset_id: child_id,
            rights_version: child.rights_version,
            timestamp: clock::timestamp_ms(clock),
        });
    }

    public struct MediaAssetLicenseOfferDisabledEvent has copy, drop {
        licensor_asset_id: ID,
        template_version_id: ID,
        usage_class: u8,
        disabled_by: address,
        timestamp: u64,
    }

    /// Rights controller: publish or replace standing license offers on this asset.
    public(package) fun update_asset_licenses(
        asset: &mut MediaAsset,
        asset_licenses: vector<AssetLicense>,
        clock: &Clock,
        _ctx: &TxContext,
    ) {
        assert_media_asset_version(asset);
        assert!(can_update_rights(asset, tx_context::sender(_ctx)), EUnauthorized);
        assert!(vector::length(&asset_licenses) <= MAX_ASSET_LICENSES, EInvalidRightsGrant);
        asset.asset_licenses = asset_licenses;
        asset.rights_version = asset.rights_version + 1;
        event::emit(MediaAssetRightsUpdatedEvent {
            media_asset_id: object::id(asset),
            rights_version: asset.rights_version,
            timestamp: clock::timestamp_ms(clock),
        });
    }

    /// Disable one license offer — stops new instances only; existing agreements unchanged.
    public(package) fun disable_asset_license(
        asset: &mut MediaAsset,
        template_version_id: ID,
        usage_class: u8,
        clock: &Clock,
        ctx: &TxContext,
    ) {
        assert_media_asset_version(asset);
        assert!(can_update_rights(asset, tx_context::sender(ctx)), EUnauthorized);
        let found = license_tpl::disable_matching_asset_license(
            &mut asset.asset_licenses,
            template_version_id,
            usage_class,
        );
        assert!(found, EInvalidRightsGrant);
        event::emit(MediaAssetLicenseOfferDisabledEvent {
            licensor_asset_id: object::id(asset),
            template_version_id,
            usage_class,
            disabled_by: tx_context::sender(ctx),
            timestamp: clock::timestamp_ms(clock),
        });
        asset.rights_version = asset.rights_version + 1;
    }

    // =========================================================================
    // Phase 2 — PendingDerivativeAsset staging + payload-complete finalize events
    // =========================================================================

    public struct PendingDerivativeAsset has key, store {
        id: UID,
        content_commitment: vector<u8>,
        media_type: u8,
        asset_kind: u8,
        creator: address,
        pending_edges: vector<DerivativeRelationship>,
        ancestry_metadata: AncestryMetadata,
        compose_accumulator: PendingComposeAccumulator,
        created_at: u64,
    }

    public struct PendingPolicyRefresh has key, store {
        id: UID,
        target_asset_id: ID,
        refresh_accumulator: PendingComposeAccumulator,
        edges_snapshot: vector<DerivativeRelationship>,
        ancestry_version: u64,
        started_at: u64,
    }

    public struct DerivativeRelationshipAddedEvent has copy, drop {
        pending_id: ID,
        parent_asset_id: ID,
        relationship_id: u64,
        relationship_type: u8,
        license_instance_id: ID,
        template_version_id: ID,
        parent_share_bps: u64,
        timestamp: u64,
    }

    public struct AncestryUpdatedEvent has copy, drop {
        pending_id: ID,
        ancestry_version: u64,
        ancestor_ids: vector<ID>,
        ancestry_hash: vector<u8>,
        timestamp: u64,
    }

    public struct DerivativeAssetFinalizedEvent has copy, drop {
        pending_id: ID,
        child_asset_id: ID,
        identity: MediaAssetIdentity,
        parent_count: u64,
        total_edge_share_bps: u64,
        ancestry_version: u64,
        ancestor_ids: vector<ID>,
        ancestry_hash: vector<u8>,
        compose_inputs: PendingComposeAccumulator,
        finalized_edges: vector<DerivativeRelationship>,
        timestamp: u64,
    }

    public struct OriginalAssetFinalizedEvent has copy, drop {
        pending_id: ID,
        child_asset_id: ID,
        identity: MediaAssetIdentity,
        timestamp: u64,
    }

    public struct ResolvedPolicyUpdatedEvent has copy, drop {
        media_asset_id: ID,
        policy_version: u64,
        effective_rights: u64,
        derivatives_allowed: bool,
        attribution_required: bool,
        commercial_allowed: bool,
        lineage: LineageCommitment,
        timestamp: u64,
    }

    public struct ResolvedObligationRecordedEvent has copy, drop {
        media_asset_id: ID,
        policy_version: u64,
        obligation_index: u64,
        beneficiary_asset_id: Option<ID>,
        beneficiary_address: address,
        share_bps: u64,
        source_relationship_id: Option<u64>,
        source_license_instance_id: Option<ID>,
        obligation_kind: u8,
        timestamp: u64,
    }

    fun ancestry_hash_from_ids(ancestor_ids: &vector<ID>): vector<u8> {
        myso::hash::blake2b256(&std::bcs::to_bytes(ancestor_ids))
    }

    fun media_asset_identity_from_pending(pending: &PendingDerivativeAsset): MediaAssetIdentity {
        MediaAssetIdentity {
            content_commitment: pending.content_commitment,
            media_type: pending.media_type,
            asset_kind: pending.asset_kind,
            creator: pending.creator,
        }
    }

    fun bootstrap_parent_policy_snapshot(
        parent: &MediaAsset,
        template: &LicenseTemplateVersion,
    ): (u64, bool, bool, bool, u64, vector<ResolvedRoyaltyObligation>) {
        if (option::is_some(&parent.resolved_policy)) {
            let policy = option::borrow(&parent.resolved_policy);
            (
                policy.effective_rights,
                policy.derivatives_allowed,
                policy.commercial_allowed,
                policy.attribution_required,
                policy.policy_version,
                parent.resolved_obligations,
            )
        } else {
            (
                license_tpl::template_granted_rights(template),
                license_tpl::template_allow_derivatives(template),
                true,
                license_tpl::template_attribution_required(template),
                0,
                vector[],
            )
        }
    }

    fun edge_obligation(
        parent_id: ID,
        relationship_id: u64,
        license_instance_id: ID,
        parent_share_bps: u64,
        parent: &MediaAsset,
    ): ResolvedRoyaltyObligation {
        let beneficiary = if (vector::length(&parent.beneficiaries) > 0) {
            *vector::borrow(&parent.beneficiaries, 0)
        } else if (vector::length(&parent.creators) > 0) {
            *vector::borrow(&parent.creators, 0)
        } else {
            parent.registered_by
        };
        derivative_graph::new_resolved_royalty_obligation(
            option::some(parent_id),
            beneficiary,
            parent_share_bps,
            option::some(relationship_id),
            option::some(license_instance_id),
            derivative_graph::obligation_kind_edge(),
        )
    }

    fun build_lineage_commitment(
        compose_inputs: &PendingComposeAccumulator,
        ancestry_version: u64,
    ): LineageCommitment {
        LineageCommitment {
            parent_asset_ids: *derivative_graph::compose_parent_asset_ids(compose_inputs),
            parent_policy_versions: *derivative_graph::compose_parent_policy_versions(compose_inputs),
            license_instance_ids: *derivative_graph::compose_license_instance_ids(compose_inputs),
            template_version_ids: *derivative_graph::compose_template_version_ids(compose_inputs),
            ancestry_version,
            lineage_hash: ancestry_hash_from_ids(derivative_graph::compose_parent_asset_ids(compose_inputs)),
        }
    }

    fun materialize_resolved_policy_from_accumulator(
        asset: &mut MediaAsset,
        accumulator: PendingComposeAccumulator,
        ancestry_version: u64,
        clock: &Clock,
    ) {
        let now = clock::timestamp_ms(clock);
        let lineage = build_lineage_commitment(&accumulator, ancestry_version);
        let next_version = if (option::is_some(&asset.resolved_policy)) {
            option::borrow(&asset.resolved_policy).policy_version + 1
        } else {
            1
        };
        asset.resolved_policy = option::some(ResolvedRightsPolicy {
            policy_version: next_version,
            effective_rights: derivative_graph::compose_effective_rights(&accumulator),
            derivatives_allowed: derivative_graph::compose_derivatives_allowed(&accumulator),
            attribution_required: derivative_graph::compose_attribution_required(&accumulator),
            commercial_allowed: derivative_graph::compose_commercial_allowed(&accumulator),
            lineage,
            resolved_at: now,
        });
        asset.resolved_obligations = *derivative_graph::compose_accumulated_obligations(&accumulator);
        let asset_id = object::id(asset);
        let policy = option::borrow(&asset.resolved_policy);
        event::emit(ResolvedPolicyUpdatedEvent {
            media_asset_id: asset_id,
            policy_version: policy.policy_version,
            effective_rights: policy.effective_rights,
            derivatives_allowed: policy.derivatives_allowed,
            attribution_required: policy.attribution_required,
            commercial_allowed: policy.commercial_allowed,
            lineage: policy.lineage,
            timestamp: now,
        });
        let mut idx = 0;
        let obligation_len = vector::length(&asset.resolved_obligations);
        while (idx < obligation_len) {
            let obligation = vector::borrow(&asset.resolved_obligations, idx);
            event::emit(ResolvedObligationRecordedEvent {
                media_asset_id: asset_id,
                policy_version: policy.policy_version,
                obligation_index: idx,
                beneficiary_asset_id: derivative_graph::obligation_beneficiary_asset_id(obligation),
                beneficiary_address: derivative_graph::obligation_beneficiary_address(obligation),
                share_bps: derivative_graph::obligation_share_bps(obligation),
                source_relationship_id: derivative_graph::obligation_source_relationship_id(obligation),
                source_license_instance_id: derivative_graph::obligation_source_license_instance_id(obligation),
                obligation_kind: derivative_graph::obligation_obligation_kind(obligation),
                timestamp: now,
            });
            idx = idx + 1;
        };
    }

    fun mint_shared_media_asset_from_pending(
        pending: PendingDerivativeAsset,
        derivative_relationships: vector<DerivativeRelationship>,
        ancestry_metadata: AncestryMetadata,
        compose_inputs: PendingComposeAccumulator,
        originality_status: u8,
        clock: &Clock,
        ctx: &mut TxContext,
    ): (ID, vector<DerivativeRelationship>) {
        let PendingDerivativeAsset {
            id,
            content_commitment,
            media_type,
            asset_kind,
            creator,
            pending_edges: _,
            ancestry_metadata: _,
            compose_accumulator: _,
            created_at,
        } = pending;
        object::delete(id);
        let now = clock::timestamp_ms(clock);
        let mut asset = MediaAsset {
            id: object::new(ctx),
            content_commitment,
            media_type,
            asset_kind,
            related_work_id: option::none(),
            creators: vector[creator],
            claims: vector[],
            rights_interests: vector[],
            usage_grants: default_usage_grants(now),
            beneficiaries: vector[creator],
            beneficiary_splits: default_beneficiary_splits(creator),
            rights_version: 1,
            economics_version: 1,
            provenance_status: provenance_verified(),
            originality_status,
            lineage_parent_id: option::none(),
            ancestry_metadata,
            derivative_relationships: vector[],
            asset_licenses: vector[],
            direct_usage_grants: vector[],
            compose_inputs,
            resolved_policy: option::none(),
            resolved_obligations: vector[],
            verified_at: option::some(now),
            poc_version: 1,
            registered_by: creator,
            registered_at: created_at,
            version: upgrade::current_version(),
        };
        let asset_id = object::id(&asset);
        let mut finalized_edges = derivative_relationships;
        derivative_graph::set_child_asset_id_on_edges(&mut finalized_edges, asset_id);
        let event_edges = finalized_edges;
        asset.derivative_relationships = finalized_edges;
        transfer::share_object(asset);
        (asset_id, event_edges)
    }

    /// Owned staging object for multi-parent derivative construction.
    public entry fun create_pending_derivative_asset(
        content_commitment: vector<u8>,
        media_type: u8,
        asset_kind: u8,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert_valid_commitment(&content_commitment);
        assert_valid_media_type(media_type);
        assert_valid_asset_kind(asset_kind);
        let creator = tx_context::sender(ctx);
        let pending = PendingDerivativeAsset {
            id: object::new(ctx),
            content_commitment,
            media_type,
            asset_kind,
            creator,
            pending_edges: vector[],
            ancestry_metadata: derivative_graph::empty_ancestry(),
            compose_accumulator: derivative_graph::empty_compose_accumulator(),
            created_at: clock::timestamp_ms(clock),
        };
        transfer::transfer(pending, creator);
    }

    /// Add one parent edge to a pending derivative (Phase 2 graph construction).
    public entry fun add_derivative_parent_edge_to_pending(
        pending: &mut PendingDerivativeAsset,
        parent: &MediaAsset,
        license_instance: &LicenseInstance,
        template_version: &LicenseTemplateVersion,
        relationship_type: u8,
        evidence_commitment: Option<vector<u8>>,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(tx_context::sender(ctx) == pending.creator || tx_context::sender(ctx) == license_tpl::instance_licensee(license_instance), EUnauthorized);
        add_derivative_parent_edge_to_pending_inner(
            pending,
            parent,
            license_instance,
            template_version,
            relationship_type,
            evidence_commitment,
            clock,
            ctx,
        );
    }

    fun add_derivative_parent_edge_to_pending_inner(
        pending: &mut PendingDerivativeAsset,
        parent: &MediaAsset,
        license_instance: &LicenseInstance,
        template_version: &LicenseTemplateVersion,
        relationship_type: u8,
        evidence_commitment: Option<vector<u8>>,
        clock: &Clock,
        _ctx: &TxContext,
    ) {
        license_tpl::assert_instance_matches_template(license_instance, template_version);
        license_tpl::assert_instance_active(license_instance, clock);
        assert!(
            license_tpl::instance_licensor_asset_id(license_instance) == object::id(parent),
            EUnauthorized,
        );
        assert!(license_tpl::template_allow_derivatives(template_version), EInvalidRightsGrant);

        let parent_id = object::id(parent);
        let pending_id = object::id(pending);
        let placeholder_child = pending_id;
        derivative_graph::assert_valid_parent_child_edge(
            &parent_id,
            &placeholder_child,
            derivative_graph::ancestor_ids(&pending.ancestry_metadata),
        );
        derivative_graph::assert_relationship_unique(
            &pending.pending_edges,
            parent_id,
            placeholder_child,
            relationship_type,
            license_tpl::license_instance_id(license_instance),
        );

        let parent_share_bps = license_tpl::derive_parent_share_bps(template_version, relationship_type);
        let relationship_id = vector::length(&pending.pending_edges) + 1;
        let edge = derivative_graph::new_derivative_relationship(
            relationship_id,
            parent_id,
            placeholder_child,
            relationship_type,
            license_tpl::license_instance_id(license_instance),
            license_tpl::template_version_id(template_version),
            parent_share_bps,
            evidence_commitment,
            clock::timestamp_ms(clock),
        );
        derivative_graph::merge_ancestry_for_edge(
            &mut pending.ancestry_metadata,
            parent_id,
            derivative_graph::ancestor_ids(&parent.ancestry_metadata),
        );
        assert!(
            vector::length(&pending.pending_edges) < MAX_DERIVATIVE_RELATIONSHIPS,
            EInvalidRightsGrant,
        );

        let (effective_rights, derivatives_allowed, commercial_allowed, attribution_required, parent_policy_version, parent_obligations) =
            bootstrap_parent_policy_snapshot(parent, template_version);
        let edge_ob = edge_obligation(
            parent_id,
            relationship_id,
            license_tpl::license_instance_id(license_instance),
            parent_share_bps,
            parent,
        );
        derivative_graph::merge_compose_accumulator(
            &mut pending.compose_accumulator,
            effective_rights,
            derivatives_allowed,
            commercial_allowed,
            attribution_required,
            parent_share_bps,
            parent_obligations,
            edge_ob,
            parent_id,
            license_tpl::license_instance_id(license_instance),
            license_tpl::template_version_id(template_version),
            parent_policy_version,
        );
        vector::push_back(&mut pending.pending_edges, edge);
        let now = clock::timestamp_ms(clock);
        let ancestor_ids = *derivative_graph::ancestor_ids(&pending.ancestry_metadata);
        event::emit(DerivativeRelationshipAddedEvent {
            pending_id,
            parent_asset_id: parent_id,
            relationship_id,
            relationship_type,
            license_instance_id: license_tpl::license_instance_id(license_instance),
            template_version_id: license_tpl::template_version_id(template_version),
            parent_share_bps,
            timestamp: now,
        });
        event::emit(AncestryUpdatedEvent {
            pending_id,
            ancestry_version: derivative_graph::ancestry_version(&pending.ancestry_metadata),
            ancestor_ids,
            ancestry_hash: ancestry_hash_from_ids(&ancestor_ids),
            timestamp: now,
        });
    }

    /// Finalize a derivative pending asset (requires >= 1 parent edge). Graph only — no resolved policy.
    public entry fun finalize_derivative_asset(
        pending: PendingDerivativeAsset,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(tx_context::sender(ctx) == pending.creator, EUnauthorized);
        let _ = finalize_derivative_asset_inner(pending, clock, ctx);
    }

    /// Package helper returning the minted shared child asset id.
    public(package) fun finalize_derivative_asset_inner(
        pending: PendingDerivativeAsset,
        clock: &Clock,
        ctx: &mut TxContext,
    ): ID {
        assert!(vector::length(&pending.pending_edges) >= 1, ENoParentEdges);
        assert!(
            derivative_graph::compose_total_edge_share_bps(&pending.compose_accumulator) <= MANIFEST_BPS_TOTAL,
            ERoyaltyOverflow,
        );
        let pending_id = object::id(&pending);
        let identity = media_asset_identity_from_pending(&pending);
        let ancestry_metadata = pending.ancestry_metadata;
        let compose_inputs = pending.compose_accumulator;
        let compose_inputs_for_event = compose_inputs;
        let finalized_edges = pending.pending_edges;
        let ancestry_version = derivative_graph::ancestry_version(&ancestry_metadata);
        let ancestor_ids = *derivative_graph::ancestor_ids(&ancestry_metadata);
        let ancestry_hash = ancestry_hash_from_ids(&ancestor_ids);
        let parent_count = vector::length(&finalized_edges);
        let total_edge_share_bps = derivative_graph::compose_total_edge_share_bps(&compose_inputs);
        let (child_asset_id, event_edges) = mint_shared_media_asset_from_pending(
            pending,
            finalized_edges,
            ancestry_metadata,
            compose_inputs,
            originality_derivative(),
            clock,
            ctx,
        );
        event::emit(DerivativeAssetFinalizedEvent {
            pending_id,
            child_asset_id,
            identity,
            parent_count,
            total_edge_share_bps,
            ancestry_version,
            ancestor_ids,
            ancestry_hash,
            compose_inputs: compose_inputs_for_event,
            finalized_edges: event_edges,
            timestamp: clock::timestamp_ms(clock),
        });
        child_asset_id
    }

    public(package) fun pending_derivative_creator(pending: &PendingDerivativeAsset): address {
        pending.creator
    }

    public(package) fun pending_derivative_id(pending: &PendingDerivativeAsset): ID {
        object::id(pending)
    }

    /// Oracle-gated parent edge add for Phase 5 discovery finalize.
    public(package) fun add_derivative_parent_edge_to_pending_oracle(
        pending: &mut PendingDerivativeAsset,
        parent: &MediaAsset,
        license_instance: &LicenseInstance,
        template_version: &LicenseTemplateVersion,
        relationship_type: u8,
        evidence_commitment: Option<vector<u8>>,
        oracle: address,
        clock: &Clock,
        ctx: &TxContext,
    ) {
        assert!(tx_context::sender(ctx) == oracle, EUnauthorized);
        add_derivative_parent_edge_to_pending_inner(
            pending,
            parent,
            license_instance,
            template_version,
            relationship_type,
            evidence_commitment,
            clock,
            ctx,
        );
    }

    /// Finalize a pending asset as an original/root work (requires zero parent edges).
    public entry fun finalize_pending_as_original(
        pending: PendingDerivativeAsset,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(tx_context::sender(ctx) == pending.creator, EUnauthorized);
        assert!(vector::length(&pending.pending_edges) == 0, EHasParentEdges);
        let pending_id = object::id(&pending);
        let identity = media_asset_identity_from_pending(&pending);
        let empty_ancestry = derivative_graph::empty_ancestry();
        let (child_asset_id, _event_edges) = mint_shared_media_asset_from_pending(
            pending,
            vector[],
            empty_ancestry,
            derivative_graph::empty_compose_accumulator(),
            originality_original(),
            clock,
            ctx,
        );
        event::emit(OriginalAssetFinalizedEvent {
            pending_id,
            child_asset_id,
            identity,
            timestamp: clock::timestamp_ms(clock),
        });
    }

    /// Phase 3 — first-time policy materialization from persisted compose_inputs.
    public entry fun materialize_initial_resolved_policy(
        asset: &mut MediaAsset,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert_media_asset_version(asset);
        assert!(option::is_none(&asset.resolved_policy), EPolicyAlreadyMaterialized);
        let ancestry_version = derivative_graph::ancestry_version(&asset.ancestry_metadata);
        let compose_inputs = asset.compose_inputs;
        materialize_resolved_policy_from_accumulator(
            asset,
            compose_inputs,
            ancestry_version,
            clock,
        );
        let _ = ctx;
    }

    /// Phase 3 — begin multi-parent policy refresh staging.
    public fun begin_policy_refresh(
        asset: &MediaAsset,
        clock: &Clock,
        ctx: &mut TxContext,
    ): PendingPolicyRefresh {
        assert_media_asset_version(asset);
        let refresh = PendingPolicyRefresh {
            id: object::new(ctx),
            target_asset_id: object::id(asset),
            refresh_accumulator: derivative_graph::empty_compose_accumulator(),
            edges_snapshot: asset.derivative_relationships,
            ancestry_version: derivative_graph::ancestry_version(&asset.ancestry_metadata),
            started_at: clock::timestamp_ms(clock),
        };
        refresh
    }

    /// Phase 3 — merge one parent into refresh staging by relationship_id.
    public entry fun merge_parent_into_policy_refresh(
        refresh: &mut PendingPolicyRefresh,
        parent: &MediaAsset,
        template_version: &LicenseTemplateVersion,
        relationship_id: u64,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        let edge = derivative_graph::find_edge_by_relationship_id(&refresh.edges_snapshot, relationship_id);
        assert!(derivative_graph::edge_parent_asset_id(edge) == object::id(parent), EEdgeParentMismatch);
        let (effective_rights, derivatives_allowed, commercial_allowed, attribution_required, parent_policy_version, parent_obligations) =
            bootstrap_parent_policy_snapshot(parent, template_version);
        let edge_ob = edge_obligation(
            derivative_graph::edge_parent_asset_id(edge),
            derivative_graph::edge_relationship_id(edge),
            derivative_graph::edge_license_instance_id(edge),
            derivative_graph::edge_parent_share_bps(edge),
            parent,
        );
        derivative_graph::merge_compose_accumulator(
            &mut refresh.refresh_accumulator,
            effective_rights,
            derivatives_allowed,
            commercial_allowed,
            attribution_required,
            derivative_graph::edge_parent_share_bps(edge),
            parent_obligations,
            edge_ob,
            derivative_graph::edge_parent_asset_id(edge),
            derivative_graph::edge_license_instance_id(edge),
            derivative_graph::edge_template_version_id(edge),
            parent_policy_version,
        );
        let _ = clock;
        let _ = ctx;
    }

    /// Phase 3 — commit refresh staging onto the target asset.
    public entry fun finalize_policy_refresh(
        asset: &mut MediaAsset,
        refresh: PendingPolicyRefresh,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert_media_asset_version(asset);
        assert!(object::id(asset) == refresh.target_asset_id, EUnauthorized);
        let PendingPolicyRefresh {
            id,
            target_asset_id: _,
            refresh_accumulator,
            edges_snapshot: _,
            ancestry_version,
            started_at: _,
        } = refresh;
        object::delete(id);
        materialize_resolved_policy_from_accumulator(
            asset,
            refresh_accumulator,
            ancestry_version,
            clock,
        );
        let _ = ctx;
    }

    public(package) fun resolved_policy_version(asset: &MediaAsset): Option<u64> {
        if (option::is_some(&asset.resolved_policy)) {
            option::some(option::borrow(&asset.resolved_policy).policy_version)
        } else {
            option::none()
        }
    }

    #[test_only]
    public fun test_mint_media_asset(
        owner: address,
        content_commitment: vector<u8>,
        media_type: u8,
        ctx: &mut TxContext,
    ): MediaAsset {
        let claims = default_claims(owner);
        let (compose_inputs, resolved_policy, resolved_obligations) = default_compose_policy_fields();
        MediaAsset {
            id: object::new(ctx),
            content_commitment,
            media_type,
            asset_kind: asset_kind_unspecified(),
            related_work_id: option::none(),
            creators: resolve_creators_from_claims(&claims),
            claims,
            rights_interests: resolve_rights_interests_from_claims(&claims),
            usage_grants: default_usage_grants(0),
            beneficiaries: vector[owner],
            beneficiary_splits: default_beneficiary_splits(owner),
            rights_version: 1,
            economics_version: 1,
            provenance_status: provenance_verified(),
            originality_status: originality_original(),
            lineage_parent_id: option::none(),
            ancestry_metadata: derivative_graph::empty_ancestry(),
            derivative_relationships: vector[],
            asset_licenses: vector[],
            direct_usage_grants: vector[],
            compose_inputs,
            resolved_policy,
            resolved_obligations,
            verified_at: option::none(),
            poc_version: 1,
            registered_by: owner,
            registered_at: 0,
            version: upgrade::current_version(),
        }
    }

    #[test_only]
    public fun test_claim(
        claim_type: u8,
        claimant: address,
        rights_mask: u64,
        verification_status: u8,
    ): Claim {
        Claim {
            claim_type,
            claimant,
            asset_id: option::none(),
            rights_mask,
            scope: 0,
            verification_status,
            evidence_commitment: option::none(),
        }
    }

    #[test_only]
    public fun test_clamp_manifest_embedded_redirect_cap(
        manifest: RevenueManifest,
        max_bps: u64,
    ): RevenueManifest {
        clamp_manifest_embedded_redirect_cap(manifest, max_bps)
    }

    #[test_only]
    public fun test_validate_manifest(manifest: &RevenueManifest) {
        validate_revenue_manifest(manifest);
    }

    #[test_only]
    public fun test_usage_grant(
        usage_class: u8,
        granted_rights: u64,
        license_type: u8,
        compensation_type: u8,
        compensation_bps: u64,
        attribution_required: bool,
        derivatives_permitted: bool,
        commercial_use_permitted: bool,
    ): UsageGrant {
        UsageGrant {
            usage_class,
            granted_rights,
            license_type,
            compensation_type,
            compensation_bps,
            attribution_required,
            derivatives_permitted,
            commercial_use_permitted,
            effective_from: 0,
            expires_at: option::none(),
            revocable: true,
        }
    }

    #[test_only]
    public fun test_usage_grant_with_expiry(
        usage_class: u8,
        granted_rights: u64,
        license_type: u8,
        compensation_type: u8,
        compensation_bps: u64,
        attribution_required: bool,
        derivatives_permitted: bool,
        commercial_use_permitted: bool,
        effective_from: u64,
        expires_at: Option<u64>,
    ): UsageGrant {
        UsageGrant {
            usage_class,
            granted_rights,
            license_type,
            compensation_type,
            compensation_bps,
            attribution_required,
            derivatives_permitted,
            commercial_use_permitted,
            effective_from,
            expires_at,
            revocable: true,
        }
    }

    #[test_only]
    public fun test_manifest_entry(
        beneficiary: address,
        share_bps: u64,
        payout_mode: u8,
    ): ManifestEntry {
        new_manifest_entry(beneficiary, share_bps, payout_mode)
    }

    #[test_only]
    public fun test_manifest_entry_with_source(
        beneficiary: address,
        share_bps: u64,
        payout_mode: u8,
        source_asset_id: ID,
    ): ManifestEntry {
        ManifestEntry {
            beneficiary,
            share_bps,
            source_asset_id: option::some(source_asset_id),
            payout_mode,
        }
    }

    #[test_only]
    public fun test_manifest_entry_share_bps(entry: &ManifestEntry): u64 {
        entry.share_bps
    }

    #[test_only]
    public fun test_manifest_entries(manifest: &RevenueManifest): &vector<ManifestEntry> {
        manifest_entries(manifest)
    }

    #[test_only]
    public fun test_revenue_manifest(entries: vector<ManifestEntry>): RevenueManifest {
        new_revenue_manifest(entries)
    }

    #[test_only]
    public fun test_composition_analysis(
        analyzed_at: u64,
        usage_context: u8,
    ): CompositionAnalysis {
        CompositionAnalysis {
            analyzed_at,
            usage_context,
            assets: vector[],
        }
    }

    #[test_only]
    public fun test_set_usage_grants(asset: &mut MediaAsset, usage_grants: vector<UsageGrant>) {
        asset.usage_grants = usage_grants;
    }

    #[test_only]
    public fun test_asset_claim_count(asset: &MediaAsset): u64 {
        vector::length(&asset.claims)
    }

    #[test_only]
    public fun test_asset_rights_interest_count(asset: &MediaAsset): u64 {
        vector::length(&asset.rights_interests)
    }

    #[test_only]
    public fun test_asset_has_creator(asset: &MediaAsset, creator: address): bool {
        vector::contains(&asset.creators, &creator)
    }

    #[test_only]
    public fun test_share_media_asset(asset: MediaAsset) {
        transfer::share_object(asset);
    }

    #[test_only]
    public fun test_mint_pending_derivative(
        creator: address,
        content_commitment: vector<u8>,
        media_type: u8,
        ctx: &mut TxContext,
    ): PendingDerivativeAsset {
        PendingDerivativeAsset {
            id: object::new(ctx),
            content_commitment,
            media_type,
            asset_kind: asset_kind_unspecified(),
            creator,
            pending_edges: vector[],
            ancestry_metadata: derivative_graph::empty_ancestry(),
            compose_accumulator: derivative_graph::empty_compose_accumulator(),
            created_at: 0,
        }
    }

    #[test_only]
    public fun test_rights_disputes_submitted(asset: &MediaAsset): u8 {
        rights_disputes_submitted(asset)
    }

    #[test_only]
    public fun test_compute_claims_commitment(
        claims: &vector<Claim>,
        usage_grants: &vector<UsageGrant>,
    ): vector<u8> {
        compute_claims_bundle_commitment(claims, usage_grants)
    }

    #[test_only]
    public fun test_has_active_rights_proposal(asset: &MediaAsset): bool {
        if (!df::exists_(&asset.id, RIGHTS_DISPUTE_DF_KEY)) {
            return false
        };
        option::is_some(&df::borrow<vector<u8>, MediaAssetRightsDisputeState>(&asset.id, RIGHTS_DISPUTE_DF_KEY).active_proposal_id)
    }

    #[test_only]
    public fun test_rights_version(asset: &MediaAsset): u64 {
        asset.rights_version
    }

    #[test_only]
    public fun test_destroy_media_asset(asset: MediaAsset) {
        let MediaAsset {
            id,
            content_commitment: _,
            media_type: _,
            asset_kind: _,
            related_work_id: _,
            creators: _,
            claims: _,
            rights_interests: _,
            usage_grants: _,
            beneficiaries: _,
            beneficiary_splits: _,
            rights_version: _,
            economics_version: _,
            provenance_status: _,
            originality_status: _,
            lineage_parent_id: _,
            ancestry_metadata: _,
            derivative_relationships: _,
            asset_licenses: _,
            direct_usage_grants: _,
            compose_inputs: _,
            resolved_policy: _,
            resolved_obligations: _,
            verified_at: _,
            poc_version: _,
            registered_by: _,
            registered_at: _,
            version: _,
        } = asset;
        object::delete(id);
    }
}

// =============================================================================
// Derivative graph — application-agnostic lineage primitives
// =============================================================================

/// **Bounded DAG:** Version 1 enforces `MAX_ANCESTORS` as an intentional protocol constraint.
/// **Cycle prevention:** Each `MediaAsset` commits transitive `AncestryMetadata`.
#[allow(duplicate_alias)]
module social_contracts::derivative_graph {
    use std::option::{Self, Option};

    use myso::object::ID;

    public fun max_ancestors(): u64 { 64 }

    const E_SELF_EDGE: u64 = 1;
    const E_CYCLE_DETECTED: u64 = 2;
    const E_ANCESTOR_LIMIT: u64 = 3;
    const E_DUPLICATE_RELATIONSHIP: u64 = 4;

    public fun relationship_remix(): u8 { 1 }
    public fun relationship_sample(): u8 { 2 }
    public fun relationship_cover(): u8 { 3 }
    public fun relationship_mashup(): u8 { 4 }

    public struct AncestryMetadata has store, copy, drop {
        ancestor_ids: vector<ID>,
        ancestry_version: u64,
    }

    public struct DerivativeRelationship has store, copy, drop {
        relationship_id: u64,
        parent_asset_id: ID,
        child_asset_id: ID,
        relationship_type: u8,
        license_instance_id: ID,
        template_version_id: ID,
        parent_share_bps: u64,
        evidence_commitment: Option<vector<u8>>,
        created_at: u64,
    }

    public fun obligation_kind_edge(): u8 { 1 }
    public fun obligation_kind_inherited(): u8 { 2 }
    public fun obligation_kind_template(): u8 { 3 }

    public struct ResolvedRoyaltyObligation has store, copy, drop {
        beneficiary_asset_id: Option<ID>,
        beneficiary_address: address,
        share_bps: u64,
        source_relationship_id: Option<u64>,
        source_license_instance_id: Option<ID>,
        obligation_kind: u8,
    }

    public struct PendingComposeAccumulator has store, copy, drop {
        effective_rights: u64,
        derivatives_allowed: bool,
        commercial_allowed: bool,
        attribution_required: bool,
        total_edge_share_bps: u64,
        accumulated_obligations: vector<ResolvedRoyaltyObligation>,
        parent_asset_ids: vector<ID>,
        license_instance_ids: vector<ID>,
        template_version_ids: vector<ID>,
        parent_policy_versions: vector<u64>,
    }

    const E_ROYALTY_OVERFLOW: u64 = 5;
    const BPS_TOTAL: u64 = 10_000;

    public(package) fun empty_compose_accumulator(): PendingComposeAccumulator {
        PendingComposeAccumulator {
            effective_rights: std::u64::max_value!(),
            derivatives_allowed: true,
            commercial_allowed: true,
            attribution_required: false,
            total_edge_share_bps: 0,
            accumulated_obligations: vector[],
            parent_asset_ids: vector[],
            license_instance_ids: vector[],
            template_version_ids: vector[],
            parent_policy_versions: vector[],
        }
    }

    public(package) fun compose_total_edge_share_bps(acc: &PendingComposeAccumulator): u64 {
        acc.total_edge_share_bps
    }

    public(package) fun compose_accumulated_obligations(
        acc: &PendingComposeAccumulator,
    ): &vector<ResolvedRoyaltyObligation> {
        &acc.accumulated_obligations
    }

    public(package) fun compose_parent_asset_ids(acc: &PendingComposeAccumulator): &vector<ID> {
        &acc.parent_asset_ids
    }

    public(package) fun compose_license_instance_ids(acc: &PendingComposeAccumulator): &vector<ID> {
        &acc.license_instance_ids
    }

    public(package) fun compose_template_version_ids(acc: &PendingComposeAccumulator): &vector<ID> {
        &acc.template_version_ids
    }

    public(package) fun compose_parent_policy_versions(acc: &PendingComposeAccumulator): &vector<u64> {
        &acc.parent_policy_versions
    }

    public(package) fun compose_effective_rights(acc: &PendingComposeAccumulator): u64 {
        acc.effective_rights
    }

    public(package) fun compose_derivatives_allowed(acc: &PendingComposeAccumulator): bool {
        acc.derivatives_allowed
    }

    public(package) fun compose_commercial_allowed(acc: &PendingComposeAccumulator): bool {
        acc.commercial_allowed
    }

    public(package) fun compose_attribution_required(acc: &PendingComposeAccumulator): bool {
        acc.attribution_required
    }

    fun find_parent_index(parent_asset_ids: &vector<ID>, parent_id: &ID): u64 {
        let len = vector::length(parent_asset_ids);
        let mut i = 0;
        while (i < len) {
            if (vector::borrow(parent_asset_ids, i) == parent_id) {
                return i
            };
            i = i + 1;
        };
        len
    }

    /// Keeps parallel lineage vectors index-aligned when parent_asset_ids is canonically sorted.
    public(package) fun insert_parent_into_lineage_vectors(
        parent_asset_id: ID,
        license_instance_id: ID,
        template_version_id: ID,
        parent_policy_version: u64,
        parent_asset_ids: &mut vector<ID>,
        license_instance_ids: &mut vector<ID>,
        template_version_ids: &mut vector<ID>,
        parent_policy_versions: &mut vector<u64>,
    ) {
        let insert_at = find_parent_index(parent_asset_ids, &parent_asset_id);
        vector::insert(parent_asset_ids, parent_asset_id, insert_at);
        vector::insert(license_instance_ids, license_instance_id, insert_at);
        vector::insert(template_version_ids, template_version_id, insert_at);
        vector::insert(parent_policy_versions, parent_policy_version, insert_at);
    }

    public(package) fun merge_compose_accumulator(
        acc: &mut PendingComposeAccumulator,
        parent_effective_rights: u64,
        parent_derivatives_allowed: bool,
        parent_commercial_allowed: bool,
        parent_attribution_required: bool,
        parent_share_bps: u64,
        parent_obligations: vector<ResolvedRoyaltyObligation>,
        edge_obligation: ResolvedRoyaltyObligation,
        parent_asset_id: ID,
        license_instance_id: ID,
        template_version_id: ID,
        parent_policy_version: u64,
    ) {
        acc.effective_rights = acc.effective_rights & parent_effective_rights;
        acc.derivatives_allowed = acc.derivatives_allowed && parent_derivatives_allowed;
        acc.commercial_allowed = acc.commercial_allowed && parent_commercial_allowed;
        acc.attribution_required = acc.attribution_required || parent_attribution_required;
        let new_total = acc.total_edge_share_bps + parent_share_bps;
        assert!(new_total <= BPS_TOTAL, E_ROYALTY_OVERFLOW);
        acc.total_edge_share_bps = new_total;
        let mut i = 0;
        let parent_len = vector::length(&parent_obligations);
        while (i < parent_len) {
            vector::push_back(&mut acc.accumulated_obligations, *vector::borrow(&parent_obligations, i));
            i = i + 1;
        };
        vector::push_back(&mut acc.accumulated_obligations, edge_obligation);
        insert_parent_into_lineage_vectors(
            parent_asset_id,
            license_instance_id,
            template_version_id,
            parent_policy_version,
            &mut acc.parent_asset_ids,
            &mut acc.license_instance_ids,
            &mut acc.template_version_ids,
            &mut acc.parent_policy_versions,
        );
    }

    public(package) fun find_edge_by_relationship_id(
        edges: &vector<DerivativeRelationship>,
        relationship_id: u64,
    ): &DerivativeRelationship {
        let len = vector::length(edges);
        let mut i = 0;
        while (i < len) {
            let edge = vector::borrow(edges, i);
            if (edge.relationship_id == relationship_id) {
                return edge
            };
            i = i + 1;
        };
        abort E_DUPLICATE_RELATIONSHIP
    }

    public(package) fun set_child_asset_id_on_edges(
        edges: &mut vector<DerivativeRelationship>,
        child_asset_id: ID,
    ) {
        let len = vector::length(edges);
        let mut i = 0;
        while (i < len) {
            let edge = vector::borrow_mut(edges, i);
            edge.child_asset_id = child_asset_id;
            i = i + 1;
        };
    }

    public(package) fun empty_ancestry(): AncestryMetadata {
        AncestryMetadata {
            ancestor_ids: vector[],
            ancestry_version: 0,
        }
    }

    public(package) fun ancestry_version(metadata: &AncestryMetadata): u64 {
        metadata.ancestry_version
    }

    public(package) fun ancestor_ids(metadata: &AncestryMetadata): &vector<ID> {
        &metadata.ancestor_ids
    }

    public(package) fun new_derivative_relationship(
        relationship_id: u64,
        parent_asset_id: ID,
        child_asset_id: ID,
        relationship_type: u8,
        license_instance_id: ID,
        template_version_id: ID,
        parent_share_bps: u64,
        evidence_commitment: Option<vector<u8>>,
        created_at: u64,
    ): DerivativeRelationship {
        DerivativeRelationship {
            relationship_id,
            parent_asset_id,
            child_asset_id,
            relationship_type,
            license_instance_id,
            template_version_id,
            parent_share_bps,
            evidence_commitment,
            created_at,
        }
    }

    public(package) fun edge_parent_asset_id(edge: &DerivativeRelationship): ID {
        edge.parent_asset_id
    }

    public(package) fun edge_relationship_id(edge: &DerivativeRelationship): u64 {
        edge.relationship_id
    }

    public(package) fun edge_license_instance_id(edge: &DerivativeRelationship): ID {
        edge.license_instance_id
    }

    public(package) fun edge_template_version_id(edge: &DerivativeRelationship): ID {
        edge.template_version_id
    }

    public(package) fun edge_parent_share_bps(edge: &DerivativeRelationship): u64 {
        edge.parent_share_bps
    }

    public(package) fun new_resolved_royalty_obligation(
        beneficiary_asset_id: Option<ID>,
        beneficiary_address: address,
        share_bps: u64,
        source_relationship_id: Option<u64>,
        source_license_instance_id: Option<ID>,
        obligation_kind: u8,
    ): ResolvedRoyaltyObligation {
        ResolvedRoyaltyObligation {
            beneficiary_asset_id,
            beneficiary_address,
            share_bps,
            source_relationship_id,
            source_license_instance_id,
            obligation_kind,
        }
    }

    public(package) fun obligation_beneficiary_asset_id(
        obligation: &ResolvedRoyaltyObligation,
    ): Option<ID> {
        obligation.beneficiary_asset_id
    }

    public(package) fun obligation_beneficiary_address(
        obligation: &ResolvedRoyaltyObligation,
    ): address {
        obligation.beneficiary_address
    }

    public(package) fun obligation_share_bps(obligation: &ResolvedRoyaltyObligation): u64 {
        obligation.share_bps
    }

    public(package) fun obligation_source_relationship_id(
        obligation: &ResolvedRoyaltyObligation,
    ): Option<u64> {
        obligation.source_relationship_id
    }

    public(package) fun obligation_source_license_instance_id(
        obligation: &ResolvedRoyaltyObligation,
    ): Option<ID> {
        obligation.source_license_instance_id
    }

    public(package) fun obligation_obligation_kind(obligation: &ResolvedRoyaltyObligation): u8 {
        obligation.obligation_kind
    }

    public(package) fun id_vector_contains(haystack: &vector<ID>, needle: &ID): bool {
        let len = vector::length(haystack);
        let mut i = 0;
        while (i < len) {
            let candidate = vector::borrow(haystack, i);
            if (candidate == needle) {
                return true
            };
            i = i + 1;
        };
        false
    }

    fun id_less_than(a: &ID, b: &ID): bool {
        let a_bytes = object::id_to_bytes(a);
        let b_bytes = object::id_to_bytes(b);
        let a_len = vector::length(&a_bytes);
        let b_len = vector::length(&b_bytes);
        let min_len = if (a_len < b_len) { a_len } else { b_len };
        let mut i = 0;
        while (i < min_len) {
            let a_byte = *vector::borrow(&a_bytes, i);
            let b_byte = *vector::borrow(&b_bytes, i);
            if (a_byte < b_byte) {
                return true
            };
            if (a_byte > b_byte) {
                return false
            };
            i = i + 1;
        };
        a_len < b_len
    }

    fun sort_ids_ascending(ids: &mut vector<ID>) {
        let len = vector::length(ids);
        if (len <= 1) {
            return
        };
        let mut i = 1;
        while (i < len) {
            let key = *vector::borrow(ids, i);
            let mut j = i;
            while (j > 0) {
                let prev = vector::borrow(ids, j - 1);
                if (!id_less_than(&key, prev)) {
                    break
                };
                *vector::borrow_mut(ids, j) = *prev;
                j = j - 1;
            };
            *vector::borrow_mut(ids, j) = key;
            i = i + 1;
        };
    }

    fun push_id_if_absent(ids: &mut vector<ID>, id: ID) {
        if (!id_vector_contains(ids, &id)) {
            vector::push_back(ids, id);
        };
    }

    public(package) fun canonical_union(
        a: &vector<ID>,
        b: &vector<ID>,
        extra: Option<ID>,
    ): vector<ID> {
        let mut merged = vector[];
        let a_len = vector::length(a);
        let mut i = 0;
        while (i < a_len) {
            push_id_if_absent(&mut merged, *vector::borrow(a, i));
            i = i + 1;
        };
        let b_len = vector::length(b);
        i = 0;
        while (i < b_len) {
            push_id_if_absent(&mut merged, *vector::borrow(b, i));
            i = i + 1;
        };
        if (option::is_some(&extra)) {
            push_id_if_absent(&mut merged, *option::borrow(&extra));
        };
        sort_ids_ascending(&mut merged);
        assert!(vector::length(&merged) <= max_ancestors(), E_ANCESTOR_LIMIT);
        merged
    }

    public(package) fun assert_valid_parent_child_edge(
        parent_id: &ID,
        child_id: &ID,
        parent_ancestors: &vector<ID>,
    ) {
        assert!(parent_id != child_id, E_SELF_EDGE);
        assert!(!id_vector_contains(parent_ancestors, child_id), E_CYCLE_DETECTED);
    }

    public(package) fun merge_ancestry_for_edge(
        child: &mut AncestryMetadata,
        parent_id: ID,
        parent_ancestors: &vector<ID>,
    ) {
        child.ancestor_ids = canonical_union(
            &child.ancestor_ids,
            parent_ancestors,
            option::some(parent_id),
        );
        child.ancestry_version = child.ancestry_version + 1;
    }

    public(package) fun assert_relationship_unique(
        existing: &vector<DerivativeRelationship>,
        parent_asset_id: ID,
        child_asset_id: ID,
        relationship_type: u8,
        license_instance_id: ID,
    ) {
        let len = vector::length(existing);
        let mut i = 0;
        while (i < len) {
            let edge = vector::borrow(existing, i);
            if (edge.parent_asset_id == parent_asset_id &&
                edge.child_asset_id == child_asset_id &&
                edge.relationship_type == relationship_type &&
                edge.license_instance_id == license_instance_id) {
                abort E_DUPLICATE_RELATIONSHIP
            };
            i = i + 1;
        };
    }

    #[test_only]
    public fun test_empty_ancestry(): AncestryMetadata {
        empty_ancestry()
    }

    #[test_only]
    public fun test_canonical_union(
        a: &vector<ID>,
        b: &vector<ID>,
        extra: Option<ID>,
    ): vector<ID> {
        canonical_union(a, b, extra)
    }

    #[test_only]
    public fun test_assert_valid_parent_child_edge(
        parent_id: &ID,
        child_id: &ID,
        parent_ancestors: &vector<ID>,
    ) {
        assert_valid_parent_child_edge(parent_id, child_id, parent_ancestors);
    }

    #[test_only]
    public fun test_merge_ancestry_for_edge(
        child: &mut AncestryMetadata,
        parent_id: ID,
        parent_ancestors: &vector<ID>,
    ) {
        merge_ancestry_for_edge(child, parent_id, parent_ancestors);
    }
}

// =============================================================================
// License templates — immutable terms, exercised agreements, ad-hoc grants
// =============================================================================

/// **Revocation semantics:** disabling an offer stops new instances; instance revoke
/// affects one agreement only when pinned template permits it. Lineage is never erased.
#[allow(duplicate_alias)]
module social_contracts::license_template {
    use std::string::String;
    use std::option::{Self, Option};
    use std::vector;

    use myso::{
        object::{Self, UID, ID},
        tx_context::{Self, TxContext},
        clock::{Self, Clock},
        transfer,
        event,
        url::{Self, Url},
    };

    const ETemplateVersionMismatch: u64 = 2;
    const EInstanceNotActive: u64 = 3;
    const EInstanceNotRevocable: u64 = 4;
    const ELicenseOfferDisabled: u64 = 5;
    const EInvalidShareBps: u64 = 6;
    const EInvalidRelationshipType: u64 = 7;

    const BPS_TOTAL: u64 = 10_000;

    public fun license_instance_active(): u8 { 1 }
    public fun license_instance_expired(): u8 { 2 }
    public fun license_instance_revoked(): u8 { 3 }

    public struct LicenseTemplateVersion has key, store {
        id: UID,
        family_id: ID,
        version: u64,
        supersedes: Option<ID>,
        creator: address,
        granted_rights: u64,
        allow_derivatives: bool,
        attribution_required: bool,
        derivative_reciprocal_terms: bool,
        royalty_bps: u64,
        derivative_royalty_bps: u64,
        transferable: bool,
        sublicensable: bool,
        effective_from: Option<u64>,
        expires_at: Option<u64>,
        scope: vector<u8>,
        legal_terms_uri: Url,
        legal_terms_hash: vector<u8>,
        governing_law: vector<u8>,
        license_schema_version: u64,
        instance_revocable: bool,
    }

    public struct AssetLicense has store, copy, drop {
        template_version_id: ID,
        usage_class: u8,
        enabled: bool,
    }

    public struct LicenseInstance has key, store {
        id: UID,
        template_version_id: ID,
        licensor_asset_id: ID,
        licensee: address,
        child_asset_id: Option<ID>,
        accepted_at: u64,
        acceptance_evidence: Option<vector<u8>>,
        expires_at: Option<u64>,
        status: u8,
    }

    public struct DirectUsageGrant has store, copy, drop {
        grantee: address,
        usage_class: u8,
        expires_at: Option<u64>,
        granted_rights: u64,
        reason: Option<String>,
    }

    public struct LicenseTemplatePublishedEvent has copy, drop {
        template_version_id: ID,
        family_id: ID,
        version: u64,
        creator: address,
        granted_rights: u64,
        allow_derivatives: bool,
        attribution_required: bool,
        royalty_bps: u64,
        derivative_royalty_bps: u64,
        instance_revocable: bool,
        timestamp: u64,
    }

    public struct LicenseInstanceAcceptedEvent has copy, drop {
        license_instance_id: ID,
        template_version_id: ID,
        licensor_asset_id: ID,
        licensee: address,
        timestamp: u64,
    }

    public struct LicenseInstanceRevokedEvent has copy, drop {
        license_instance_id: ID,
        template_version_id: ID,
        revoked_by: address,
        timestamp: u64,
    }

    public fun template_version_id(template: &LicenseTemplateVersion): ID {
        object::id(template)
    }

    public fun license_instance_id(instance: &LicenseInstance): ID {
        object::id(instance)
    }

    public fun instance_template_version_id(instance: &LicenseInstance): ID {
        instance.template_version_id
    }

    public fun instance_status(instance: &LicenseInstance): u8 {
        instance.status
    }

    public fun instance_licensee(instance: &LicenseInstance): address {
        instance.licensee
    }

    public fun instance_licensor_asset_id(instance: &LicenseInstance): ID {
        instance.licensor_asset_id
    }

    public fun template_allow_derivatives(template: &LicenseTemplateVersion): bool {
        template.allow_derivatives
    }

    public fun template_granted_rights(template: &LicenseTemplateVersion): u64 {
        template.granted_rights
    }

    public fun template_attribution_required(template: &LicenseTemplateVersion): bool {
        template.attribution_required
    }

    public fun instance_is_active(instance: &LicenseInstance, clock: &Clock): bool {
        if (instance.status != license_instance_active()) {
            return false
        };
        if (option::is_some(&instance.expires_at)) {
            let expires_at = *option::borrow(&instance.expires_at);
            return clock::timestamp_ms(clock) <= expires_at
        };
        true
    }

    public fun assert_instance_matches_template(
        instance: &LicenseInstance,
        template: &LicenseTemplateVersion,
    ) {
        assert!(
            instance.template_version_id == object::id(template),
            ETemplateVersionMismatch,
        );
    }

    public fun assert_instance_active(instance: &LicenseInstance, clock: &Clock) {
        assert!(instance_is_active(instance, clock), EInstanceNotActive);
    }

    public fun derive_parent_share_bps(
        template: &LicenseTemplateVersion,
        relationship_type: u8,
    ): u64 {
        assert!(relationship_type >= 1 && relationship_type <= 4, EInvalidRelationshipType);
        let bps = template.derivative_royalty_bps;
        assert!(bps <= BPS_TOTAL, EInvalidShareBps);
        bps
    }

    public(package) fun asset_license_template_id(offer: &AssetLicense): ID {
        offer.template_version_id
    }

    public(package) fun asset_license_usage_class(offer: &AssetLicense): u8 {
        offer.usage_class
    }

    public(package) fun set_asset_license_enabled(offer: &mut AssetLicense, enabled: bool) {
        offer.enabled = enabled;
    }

    public(package) fun disable_matching_asset_license(
        offers: &mut vector<AssetLicense>,
        template_version_id: ID,
        usage_class: u8,
    ): bool {
        let len = vector::length(offers);
        let mut i = 0;
        while (i < len) {
            let offer = vector::borrow_mut(offers, i);
            if (offer.template_version_id == template_version_id && offer.usage_class == usage_class) {
                offer.enabled = false;
                return true
            };
            i = i + 1;
        };
        false
    }

    public fun publish_license_template_version(
        family_id: ID,
        version: u64,
        supersedes: Option<ID>,
        granted_rights: u64,
        allow_derivatives: bool,
        attribution_required: bool,
        derivative_reciprocal_terms: bool,
        royalty_bps: u64,
        derivative_royalty_bps: u64,
        transferable: bool,
        sublicensable: bool,
        effective_from: Option<u64>,
        expires_at: Option<u64>,
        scope: vector<u8>,
        legal_terms_uri_bytes: vector<u8>,
        legal_terms_hash: vector<u8>,
        governing_law: vector<u8>,
        license_schema_version: u64,
        instance_revocable: bool,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(royalty_bps <= BPS_TOTAL, EInvalidShareBps);
        assert!(derivative_royalty_bps <= BPS_TOTAL, EInvalidShareBps);
        let creator = tx_context::sender(ctx);
        let legal_terms_uri = url::new_unsafe_from_bytes(legal_terms_uri_bytes);
        let template = LicenseTemplateVersion {
            id: object::new(ctx),
            family_id,
            version,
            supersedes,
            creator,
            granted_rights,
            allow_derivatives,
            attribution_required,
            derivative_reciprocal_terms,
            royalty_bps,
            derivative_royalty_bps,
            transferable,
            sublicensable,
            effective_from,
            expires_at,
            scope,
            legal_terms_uri,
            legal_terms_hash,
            governing_law,
            license_schema_version,
            instance_revocable,
        };
        let template_version_id = object::id(&template);
        event::emit(LicenseTemplatePublishedEvent {
            template_version_id,
            family_id,
            version,
            creator,
            granted_rights,
            allow_derivatives,
            attribution_required,
            royalty_bps,
            derivative_royalty_bps,
            instance_revocable,
            timestamp: clock::timestamp_ms(clock),
        });
        transfer::share_object(template);
    }

    public fun accept_license_instance(
        template: &LicenseTemplateVersion,
        licensor_asset_id: ID,
        expires_at: Option<u64>,
        acceptance_evidence: Option<vector<u8>>,
        clock: &Clock,
        ctx: &mut TxContext,
    ): LicenseInstance {
        let licensee = tx_context::sender(ctx);
        let instance = LicenseInstance {
            id: object::new(ctx),
            template_version_id: object::id(template),
            licensor_asset_id,
            licensee,
            child_asset_id: option::none(),
            accepted_at: clock::timestamp_ms(clock),
            acceptance_evidence,
            expires_at,
            status: license_instance_active(),
        };
        let license_instance_id = object::id(&instance);
        event::emit(LicenseInstanceAcceptedEvent {
            license_instance_id,
            template_version_id: object::id(template),
            licensor_asset_id,
            licensee,
            timestamp: clock::timestamp_ms(clock),
        });
        instance
    }

    public fun revoke_license_instance(
        instance: &mut LicenseInstance,
        template: &LicenseTemplateVersion,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert_instance_matches_template(instance, template);
        assert!(template.instance_revocable, EInstanceNotRevocable);
        assert!(instance.status == license_instance_active(), EInstanceNotActive);
        instance.status = license_instance_revoked();
        event::emit(LicenseInstanceRevokedEvent {
            license_instance_id: object::id(instance),
            template_version_id: object::id(template),
            revoked_by: tx_context::sender(ctx),
            timestamp: clock::timestamp_ms(clock),
        });
    }

    public(package) fun new_asset_license(
        template_version_id: ID,
        usage_class: u8,
        enabled: bool,
    ): AssetLicense {
        AssetLicense {
            template_version_id,
            usage_class,
            enabled,
        }
    }

    public(package) fun assert_asset_license_enabled(offer: &AssetLicense) {
        assert!(offer.enabled, ELicenseOfferDisabled);
    }

    public(package) fun new_direct_usage_grant(
        grantee: address,
        usage_class: u8,
        expires_at: Option<u64>,
        granted_rights: u64,
        reason: Option<String>,
    ): DirectUsageGrant {
        DirectUsageGrant {
            grantee,
            usage_class,
            expires_at,
            granted_rights,
            reason,
        }
    }

    #[test_only]
    public fun test_publish_template(
        family_id: ID,
        derivative_royalty_bps: u64,
        instance_revocable: bool,
        ctx: &mut TxContext,
    ): LicenseTemplateVersion {
        LicenseTemplateVersion {
            id: object::new(ctx),
            family_id,
            version: 1,
            supersedes: option::none(),
            creator: tx_context::sender(ctx),
            granted_rights: 0,
            allow_derivatives: true,
            attribution_required: true,
            derivative_reciprocal_terms: false,
            royalty_bps: 0,
            derivative_royalty_bps,
            transferable: false,
            sublicensable: false,
            effective_from: option::none(),
            expires_at: option::none(),
            scope: vector[],
            legal_terms_uri: url::new_unsafe_from_bytes(b"https://example.com/license"),
            legal_terms_hash: x"00",
            governing_law: vector[],
            license_schema_version: 1,
            instance_revocable,
        }
    }

    #[test_only]
    public fun test_accept_instance(
        template: &LicenseTemplateVersion,
        licensor_asset_id: ID,
        ctx: &mut TxContext,
    ): LicenseInstance {
        LicenseInstance {
            id: object::new(ctx),
            template_version_id: object::id(template),
            licensor_asset_id,
            licensee: tx_context::sender(ctx),
            child_asset_id: option::none(),
            accepted_at: 0,
            acceptance_evidence: option::none(),
            expires_at: option::none(),
            status: license_instance_active(),
        }
    }

    #[test_only]
    public fun test_destroy_template(template: LicenseTemplateVersion) {
        let LicenseTemplateVersion {
            id,
            family_id: _,
            version: _,
            supersedes: _,
            creator: _,
            granted_rights: _,
            allow_derivatives: _,
            attribution_required: _,
            derivative_reciprocal_terms: _,
            royalty_bps: _,
            derivative_royalty_bps: _,
            transferable: _,
            sublicensable: _,
            effective_from: _,
            expires_at: _,
            scope: _,
            legal_terms_uri: _,
            legal_terms_hash: _,
            governing_law: _,
            license_schema_version: _,
            instance_revocable: _,
        } = template;
        object::delete(id);
    }

    #[test_only]
    public fun test_destroy_instance(instance: LicenseInstance) {
        let LicenseInstance {
            id,
            template_version_id: _,
            licensor_asset_id: _,
            licensee: _,
            child_asset_id: _,
            accepted_at: _,
            acceptance_evidence: _,
            expires_at: _,
            status: _,
        } = instance;
        object::delete(id);
    }
}
