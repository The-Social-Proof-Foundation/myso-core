---
name: ""
overview: ""
todos: []
isProject: false
---

# Plan: Update Governance API, Schema, and Server for Proposal Non-Transferable

## Goal

Align the Social Indexer, Social Server, and Social Schema with the governance refactor that made `Proposal` non-transferable. Proposal is now a shared object; clients must pass the Proposal object (not just `proposal_id`) when calling proposal-mutating entry functions. The API and schema must support proposal types, expose object IDs for transaction building, and document the new client workflow.

## Context: What Changed On-Chain

- **Proposal** is now `has key` (no `store`) — a shared object created with `transfer::share_object(proposal)`.
- **Entry functions** that mutate proposals now take `proposal: &mut Proposal` (or `&Proposal`) instead of `proposal_id: ID`:
  - `rescind_proposal`, `delegate_vote_on_proposal`, `community_vote_on_proposal`, `finalize_proposal`, `finalize_proposal_anonymous`, `mark_proposal_implemented`, `reject_proposal_manually`, `community_vote_anonymous`
- **Proposal ID = Object ID**: The `id` returned by the API is the object ID of the shared Proposal. Clients use it to fetch the object and pass it as an input to the chain.

## Scope


| Component         | Path                                     | Changes                                                                                   |
| ----------------- | ---------------------------------------- | ----------------------------------------------------------------------------------------- |
| Social Schema     | `crates/myso-indexer-alt-social-schema/` | Add proposal type constants; ensure schema supports filtering by type                     |
| Social Indexer    | `crates/myso-indexer-alt-social/`        | No structural changes (events unchanged); verify proposal_type flows correctly            |
| Social Server     | `crates/myso-social-server/`             | Add `object_id` to proposal responses; document proposal types; ensure query params align |
| API Documentation | (inline or separate)                     | Document that clients must pass Proposal object for mutating calls                        |


---

## Implementation Steps

### Step 1: Add Proposal Type Constants to Social Schema

**File:** [myso-indexer-alt-social-schema/src/lib.rs](crates/myso-indexer-alt-social-schema/src/lib.rs)

Add constants matching the Move governance module:

```rust
// Proposal types (must match governance.move)
pub const PROPOSAL_TYPE_ECOSYSTEM: i16 = 0;
pub const PROPOSAL_TYPE_PROOF_OF_CREATIVITY: i16 = 1;
pub const PROPOSAL_TYPE_PLATFORM: i16 = 3;
```

**Rationale:** Enables type-safe filtering and validation in the server and clients.

---

### Step 2: Update ProposalRow and API Responses for Transaction Building

**File:** [myso-social-server/src/reader/types/governance.rs](crates/myso-social-server/src/reader/types/governance.rs)

- Add `object_id` field to `ProposalRow` (or document that `id` is the object ID).
- **Option A:** Add `object_id` as an alias/serde rename of `id` for clarity in API docs.
- **Option B:** Keep `id` as-is; add OpenAPI/response schema documentation that `id` is the Proposal object ID for use in transaction inputs.

**Recommendation:** Keep `id` as the proposal/object ID. Add a short doc comment or response example clarifying that `id` is the object ID clients must pass when building rescind/vote/finalize transactions.

**File:** [myso-social-server/src/server/handlers/governance.rs](crates/myso-social-server/src/server/handlers/governance.rs)

- Ensure `get_governance_proposal` response includes `id` prominently.
- Optionally add `object_id` as a duplicate of `id` in the JSON response for explicit documentation, e.g.:

```json
{
  "proposal": { "id": "0x...", "object_id": "0x...", ... },
  "delegate_votes": [...],
  ...
}
```

**Recommendation:** Add `object_id` to the proposal object in the response, set equal to `id`, so API consumers clearly understand this is the object to pass to the chain.

---

### Step 3: Validate and Document Proposal Type Query Parameter

**File:** [myso-social-server/src/server/mod.rs](crates/myso-social-server/src/server/mod.rs)

- `GovernanceProposalQuery` already has `proposal_type: Option<i16>`.
- Add validation: when `proposal_type` is provided, reject invalid values (only 0, 1, 3 are valid).
- Document the valid values in the struct or via OpenAPI: `0` = Ecosystem, `1` = Proof of Creativity, `3` = Platform.

**File:** [myso-social-server/src/reader/governance.rs](crates/myso-social-server/src/reader/governance.rs)

- `list_proposals` already filters by `proposal_type`. No change needed if validation is added at the handler layer.

---

### Step 4: Add Proposal Type to List Response Metadata (Optional)

If the API returns a list of proposals, each row already has `proposal_type`. Ensure the OpenAPI/schema documents the enum:

- `0` — Ecosystem
- `1` — Proof of Creativity  
- `3` — Platform

No schema migration required; `proposal_type` already exists in the `proposals` table.

---

### Step 5: Indexer Verification

**File:** [myso-indexer-alt-social/src/handlers/governance.rs](crates/myso-indexer-alt-social/src/handlers/governance.rs)

- Events (`ProposalSubmittedEvent`, `DelegateVoteEvent`, etc.) still emit `proposal_id`.
- `process_proposal_submitted_event` already sets `proposal_type` from the event.
- **No code changes required** — the indexer is event-driven and does not build transactions.
- **Verification:** Run indexer tests to ensure governance events still process correctly.

---

### Step 6: API Documentation / Response Format

Create or update API documentation (e.g. `API_RESPONSE_FORMATS.md` or OpenAPI spec) to state:

1. **Proposal `id`** is the object ID of the shared Proposal on-chain.
2. For **rescind_proposal**, **delegate_vote_on_proposal**, **community_vote_on_proposal**, **finalize_proposal**, **mark_proposal_implemented**, **reject_proposal_manually**, and **community_vote_anonymous**, clients must:
  - Fetch the Proposal object at `id` (e.g. via `myso_getObject` or equivalent).
  - Include the Proposal object as an input in the Programmable Transaction Block (PTB).
3. **Proposal types** for filtering: `0` (Ecosystem), `1` (Proof of Creativity), `3` (Platform).

---

## Summary of File Changes


| File                                                      | Change                                                          |
| --------------------------------------------------------- | --------------------------------------------------------------- |
| `myso-indexer-alt-social-schema/src/lib.rs`               | Add `PROPOSAL_TYPE`_* constants                                 |
| `myso-social-server/src/reader/types/governance.rs`       | Add `object_id` to `ProposalRow` (or doc that `id` = object ID) |
| `myso-social-server/src/server/mod.rs`                    | Validate `proposal_type` in `GovernanceProposalQuery`           |
| `myso-social-server/src/server/handlers/governance.rs`    | Include `object_id` in `get_governance_proposal` response       |
| `myso-social-server/API_RESPONSE_FORMATS.md` (or similar) | Document proposal object ID and proposal types                  |


---

## Verification

1. **Schema:** `cargo build -p myso-indexer-alt-social-schema`
2. **Server:** `cargo build -p myso-social-server`
3. **Indexer:** `cargo build -p myso-indexer-alt-social`
4. **Tests:** Run any existing governance API or indexer tests.

---

## Risk Mitigation

- **Backward compatibility:** Adding `object_id` (or documenting `id`) does not break existing clients; it clarifies usage.
- **Proposal type validation:** Rejecting invalid `proposal_type` values prevents nonsensical queries.

