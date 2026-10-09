# Agent Rooms, Task Market & Bidirectional Star Reputation — Greenfield Plan (rev 6)

## Context
We are adding three things to the MySo stack, sequenced Rooms → Tasks → Reputation (reputation ships only after settled-task receipts exist):
1. **Agent rooms**: an existing `PermissionedGroup<Messaging>` + EncryptionHistory + MessageLog triple, tagged in group metadata. No second encryption stack.
2. **Task market**: MYSO escrow between a buyer and a seller agent, with offer / accept / deliver / settle / mutual refund / cancel.
3. **Reputation**: after a task settles, **buyer and seller each rate the other 1–5 stars, once per receipt, independently**. Each rating is weighted by verified transaction value and the rater's credibility, decayed for repeat pairs, and folded into a **Bayesian average**. Every **agent accumulates its own seller-role score and its own buyer-role score** (agents do not share a score through their principal), and an **overall score** is a weighted Bayesian average of those two roles. A principal that buys directly (no agent) has its own human-buyer score. **Credibility stays principal-level.** All aggregates are stored on-chain and updated in **O(1)** per review.

Rev 6 closes three production gaps found in rev 5, without changing the approved reputation design:
1. **Seller acceptance:** funds are escrowed as an *offer*; the seller must explicitly accept the exact scope, payment and windows before work starts. Offers expire, can be cancelled by the buyer, and can be declined by the seller.
2. **Refund protection:** a buyer can no longer unilaterally refund delivered work. Post-delivery refunds need **mutual agreement** (one party proposes, the other accepts). The automatic seller payment after the grace period is preserved, and a pending proposal never delays it.
3. **Autonomous agents:** task events feed the existing `myso-memory` automation engine, and agents act through the existing registered-action pipeline (`@socialproof/sub-agents` + memory relayer prepare/submit) using scoped delegates, so agents can discover, accept, execute, deliver, settle and review with no UI.

Carried forward unchanged: unsigned-only on-chain values (no `i64`), flat events, bidirectional 1–5★ reviews once per receipt per direction, per-agent seller/buyer scores, principal-level credibility and pair-decay anti-farming, weighted Bayesian role and overall scores, checked u128 arithmetic with validated config, the deactivated-agent rules, and the `TaskMarketCap` admin dialog. Greenfield: no deployed state, no legacy objects, no migrations, no backward compatibility.

Scope of code touched (anchors re-verified in this repo):
- Move: `myso-core/crates/myso-framework/packages/{myso-social,messaging}`
- Indexer stack: `myso-core/crates/{myso-indexer-alt-social, -social-schema, -social-reader, myso-social-server, myso-indexer-alt-graphql}`
- `myso-memory`: now also changed (see Workstream 5): `packages/sub-agents`, `services/server`, `services/automation`, `services/automation-sidecar`, `docs`.
- `myso-messaging-stack/chat-app`
- `mysocial-frontend` (ecosystem admin dialog only — new TaskMarketCap section)

Pre-work (P0): save this plan to `myso-core/.cursor/plans/agent_rooms_task_receipts_reputation.plan.md`. The stale `myso-messaging-stack/move/packages/messaging` mirror is out of scope and left as is; myso-core is the only Move source of truth for this work.

## Verified anchors (myso-social/sources/memory.move, messaging/sources/messaging.move)
- Caps use bits 1–16 (`CAP_MEMORY_READ`=1 … `CAP_SOCIAL_GRAPH`=65536, :75-93). Bit 17 (131072) is free, so `CAP_TASK_TRADE = 131072`. Accessors at :689-704; `has_cap` at :2964.
- `SubAgent` struct :378-400 is built only at :2286 and destructured at :2538 with `..`.
- `MemoryAccount` struct :422-432 is built only at :859.
- `dynamic_field as df` is already imported in memory.move (:45).
- `resolve_actor_with_cap` :1909 (the human principal short-circuits), `assert_direct_execution_allowed` :1952, `assert_sub_agent_active` :1987, `sub_agent_derived_address` :1999, `bootstrap_init` :772.
- messaging.move: `AGENT_CHAT_KEY` :93, `AgentGroupCreated` :140, `create_agent_group` :454, `create_agent_and_share_group` :568, `grant_principal_oversight` :1135, `attach_agent_creator_metadata` :1194, `is_direct_message_group` :1312.
- The metadata edit functions at :746-798 require `MetadataAdmin`, which nobody holds today. No `i64` exists in any myso-social Move source.
- Indexer: `SUB_AGENT_REGISTRY_EVENTS` at sub_agent_registry_handler.rs:29-35. Latest migration is `20260929140000_profile_deleted_at`.

## Workstream 1 — Rooms (unchanged)
**Move (messaging.move)**
- In `grant_principal_oversight`, add `group.grant_permission<Messaging, MetadataAdmin>(principal, ctx)`. No new authority: the principal's `PermissionsAdmin` could already self-grant it.
- Add constants `ROOM_KIND_KEY="room_kind"`, `ROOM_KIND_AGENT_ROOM: u8 = 1`, `ROOM_TASK_ID_KEY="task_id"` and accessor `room_kind_agent_room()`.
- Add public getter `group_data(&GroupManager, &PermissionedGroup<Messaging>, String): Option<String>` (closes the read gap, since `metadata::get_data_value` is `public(package)`).
- Append `room_kind: Option<u8>, room_task_id: Option<ID>` to `create_agent_group` and `create_agent_and_share_group`. Metadata is stamped in the creation path like `attach_agent_creator_metadata`; an unmapped kind aborts with `EUnknownRoomKind`; emit `AgentRoomBound { group_id, room_kind, task_id, creator_actor, creator_principal, organization_id, created_at_ms }` after `AgentGroupCreated`.
- `AgentGroupCreated` and the eight `message_log` events are untouched. Regenerate `package_summaries` after the signature change.

**Indexer:** `BcsAgentRoomBound` plus an arm in `parse_messaging_event`; add the event to the `messaging.rs` handler list. Migration `20261010120000_agent_rooms` (`agent_rooms`, PK `group_id`, indexes on creator_principal, organization_id, task_id). Existing `MessagingHandler` (no new watermark).

**REST + chat-app:** `GET /agents/rooms`, `GET /organizations/:id/rooms`, `GET /groups/:group_id/digests`. chat-app: `chain-ids.ts`, `createAgentRoomTx`, `fetchAgentRooms` / `fetchGroupDigests`, hooks, `RoomsPanel`, `CreateRoomDialog`, `RoomVerifyPanel`, and a `'rooms'` sidebar view.

## Workstream 2 — Task Market with seller acceptance (`social_contracts::agent_task` in myso-social)
It lives in the myso-social package because messaging depends on myso-social (the reverse import would be a cycle). `room_group_id` is an opaque `Option<ID>` validated off-chain. The fee split is re-implemented against frozen fee snapshots (same math as `paid_escrow_settlement`). `platform::fund_platform_treasury_from_coin` is reused. `agent_task` imports `memory`; the shared reputation types live in `memory.move`.

**Lifecycle**
```
create_task (escrow funded)            ─► OFFERED ─┬─ accept_task (seller, before accept_deadline) ─► ACCEPTED
                                                   ├─ cancel_offer (buyer)       ─► CANCELLED (close_kind 0)
                                                   ├─ decline_task (seller)      ─► CANCELLED (close_kind 1)
                                                   └─ expire_offer (anyone, after accept_deadline) ─► CANCELLED (close_kind 2)
ACCEPTED ─┬─ mark_delivered (seller, before delivery deadline) ─► DELIVERED
          └─ refund_task (anyone, after delivery deadline)      ─► REFUNDED (refund_kind 0)
DELIVERED ─┬─ settle_task (buyer)                                ─► SETTLED (kind 1)
           ├─ release_task (anyone, after settle window + grace)  ─► SETTLED (kind 2, pays seller)
           └─ propose_refund (either party) + accept_refund (the other party) ─► REFUNDED (refund_kind 1)
SETTLED ─► review_seller / review_buyer_*  (Workstream 3)
```
Status values: 0 OFFERED, 1 ACCEPTED, 2 DELIVERED, 3 SETTLED, 4 REFUNDED, 5 CANCELLED.

**TaskConfig (shared, genesis)**
- Fees: `platform_fee_bps` 200, `ecosystem_fee_bps` 50, `ecosystem_fee_recipient`.
- Escrow and windows (all durations, so the seller approves lengths, not wall-clock instants): `min_escrow_mist`; `accept_window_ms` min/default/max (5 min / 24 h / 14 d); `delivery_window_ms` min/max (1 h / 90 d); `settle_window_ms` min/max (1 h / 60 d); `release_grace_ms` (default 3 d, ≤ 30 d); `review_window_ms` (default 30 d, ≤ 365 d); `max_title_bytes`, `max_uri_bytes`.
- `active`, `version`, plus an embedded `ReputationParams` (Workstream 3).
- **Validated bounds:** `validate_task_config(...)` runs at genesis and in every `set_task_config`; a violation aborts with `EInvalidTaskConfig` / `EInvalidReputationParams`. Fee sum ≤ 10000; `min_escrow_mist ≥ 1`; every window has `0 < min ≤ default ≤ max`; the caps above; title/URI limits ≥ 1; plus the reputation bounds in Workstream 3.
- `bootstrap_init` and `TaskMarketCap`: called from `bootstrap.move` beside `memory::bootstrap_init`.

**TaskEscrow (shared, flat)**
- Parties (`buyer_memory_account_id`, `buyer_agent_id: Option<ID>`, `buyer_principal`, `seller_memory_account_id`, `seller_agent_id`, `seller_principal`), `organization_id`, `room_group_id`, title, 32-byte `scope_digest`, `amount_mist`, `escrowed: Balance<MYSO>`, status, `close_kind`, `settlement_kind`, deliverable digest/URI.
- **Frozen terms:** `platform_fee_bps` and `ecosystem_fee_bps` copied from config at create, plus `accept_window_ms`, `delivery_window_ms`, `settle_window_ms` and `terms_digest` (sha3-256 of the BCS of buyer principal, seller agent id, amount, scope digest, the three windows and both fee bps). A later `set_task_config` never changes an accepted or pending offer.
- **Timestamps:** `created_at_ms`, `accept_deadline_ms = created + accept_window`, `accepted_at_ms`, `delivery_deadline_ms = accepted_at + delivery_window`, `delivered_at_ms`, `settle_deadline_ms = delivered_at + settle_window`, `settled_at_ms`, `review_deadline_ms`.
- **Refund proposal:** `refund_proposer: u8` (0 none, 1 buyer, 2 seller), `refund_proposed_at_ms`.
- **Review state:** `buyer_credibility_bps` and `seller_credibility_bps` (snapshots at settlement), `buyer_review_stars`, `seller_review_stars` (Workstream 3).

**Entrypoints and authorization** (every state change asserts the expected `status` first; an `active` check exists only where stated)
- `create_task(config, memory_config, buyer_account, seller_agent, payment, scope_digest, title, windows…, room_group_id, clock, ctx)`:
  - the buyer is resolved with `resolve_actor_with_cap(cap_task_trade, escrow_amount)` plus `assert_direct_execution_allowed`, so the existing per-action spend cap and approval-required caps apply to funding an offer;
  - distinct buyer/seller accounts and agents (`ETaskSelfDealing`); buyer account/agent active and **seller agent active**; `escrow ≥ min_escrow_mist`; each window within config bounds; 32-byte `scope_digest`; title/URI limits; market `active`;
  - escrows the funds, freezes terms, emits `TaskOffered`.
- `accept_task(config, task, seller_account: &MemoryAccount, seller_agent: &SubAgent, expected_terms_digest, clock, ctx)`:
  - status OFFERED; `clock <= accept_deadline_ms`; `expected_terms_digest == task.terms_digest` (**the seller explicitly approves the exact scope, payment and windows; a bait-and-switch cannot be accepted by mistake**);
  - the sender is the seller agent (`sub_agent_derived_address == sender`) resolved through `resolve_actor_with_cap(cap_task_trade)`, or the seller principal; `seller_agent` and `seller_account` match the task ids;
  - the seller agent is **active** (accepting new work);
  - sets ACCEPTED, `accepted_at_ms`, `delivery_deadline_ms`; emits `TaskAccepted`.
- `decline_task(task, seller_account, seller_agent, clock, ctx)`: OFFERED; seller agent (with cap) or seller principal; **no `active` check** (a deactivated seller can still decline); refunds the escrow in full; CANCELLED `close_kind = 1`; emits `TaskCancelled`.
- `cancel_offer(task, buyer_account, clock, ctx)`: OFFERED only; buyer actor with the cap; full refund; `close_kind = 0`. Once accepted the buyer can no longer cancel.
- `expire_offer(task, clock)`: **permissionless**; OFFERED; `clock > accept_deadline_ms`; full refund to the buyer; `close_kind = 2`.
- `mark_delivered(task, seller_agent, digest, uri, clock, ctx)`: status **ACCEPTED**; sender is the accepted seller agent; `clock <= delivery_deadline_ms`; 32-byte digest and URI limit; no cap; no `active` check (an accepted task can still be delivered after deactivation). Sets DELIVERED, `delivered_at_ms`, `settle_deadline_ms`.
- `settle_task(config, task, buyer_account: &mut MemoryAccount, seller_account: &mut MemoryAccount, clock, ctx)`: buyer actor with the cap; DELIVERED; **no clock check** (the buyer can settle until a release or mutual refund lands). Pays out using the frozen fee bps (skip zero splits; fold the platform share into the ecosystem recipient when the platform is `@0x0`), SETTLED, `settlement_kind = 1`, snapshots then updates both credibilities, `review_deadline_ms = settled_at + review_window_ms` (checked add), emits `TaskReceiptSettled`. It never touches `SubAgent` or reputation aggregates.
- `release_task`: **permissionless**; DELIVERED; `clock > settle_deadline_ms + release_grace_ms` (checked add). Same payout, snapshots and credibility update with `settlement_kind = 2`. This preserves automatic seller payment, and a pending refund proposal **never delays it** (any proposal is voided when the task settles).
- `refund_task`: **permissionless**; **ACCEPTED only**; `clock > delivery_deadline_ms` (the seller never delivered). Full refund, `refund_kind = 0`. It aborts on OFFERED and DELIVERED tasks.
- **Mutual refund after delivery (no unilateral buyer refund exists):**
  - `propose_refund(task, …)`: DELIVERED; sender is the buyer actor (with the cap) or the seller agent/principal; sets `refund_proposer`; emits `TaskRefundProposed`. A new proposal by the same party refreshes it; if the other party had a pending proposal, this call aborts (`EProposalPending`), so the counterparty must accept or reject first.
  - `accept_refund(task, …)`: DELIVERED; a proposal exists; the sender must be the **other** party (a proposer cannot accept its own proposal, `ENotCounterparty`); full escrow to the buyer, no fees, REFUNDED `refund_kind = 1`, emits `TaskRefunded`. Accepting needs no `active` check on either side.
  - `reject_refund(task, …)` (the counterparty) and `withdraw_refund(task, …)` (the proposer) clear the proposal; both emit `TaskRefundProposalCleared`.
  - No `cap` is needed for the seller side (no funds move to the seller); the buyer side uses `cap_task_trade` because it initiates a funds-affecting action.
- `set_task_config` (`TaskMarketCap`).
- `review_seller` and `review_buyer_agent` / `review_buyer_principal` (Workstream 3).
- Settle, release, refund and `accept_refund` race on the shared `TaskEscrow`, and the `status` transition ensures exactly one wins. Cancelled, refunded and unaccepted tasks never create a receipt, so they allow no reviews and move no credibility.

**Flat events:** `TaskOffered` (replaces `TaskCreated`: ids, parties, `organization_id`, `room_group_id`, title, `scope_digest`, `amount_mist`, the three windows, fee bps, `terms_digest`, `accept_deadline_ms`, `created_at_ms`), `TaskAccepted { task_id, seller_agent_id, seller_principal, accepted_at_ms, delivery_deadline_ms }`, `TaskCancelled { task_id, close_kind, buyer_principal, amount_mist, cancelled_at_ms }`, `TaskDelivered { …, delivered_at_ms, settle_deadline_ms }`, `TaskRefundProposed { task_id, proposer_role, proposed_at_ms }`, `TaskRefundProposalCleared { task_id, cleared_by_role, reason (0 withdrawn, 1 rejected), cleared_at_ms }`, `TaskRefunded { task_id, refund_kind, buyer_principal, amount_mist, refunded_at_ms }`, `TaskReceiptSettled` (as in rev 5, plus `settle_deadline_ms`), `TaskConfigUpdated` (all config fields including the new windows). Review and reputation events: Workstream 3. All flat; no nested payloads.

**Indexer / REST / chat-app (summary; reputation parts in Workstream 3; automation in Workstream 5)**
- Handler `agent_task_handler.rs` mirroring `sub_agent_registry_handler.rs`; migration `20261010120100_agent_task_market` (`agent_tasks` with every field above, `task_receipts`, `task_reviews`, `task_config`, `task_config_history`, `automation_event_outbox`).
- Routes: `/tasks` (filters: buyer principal, seller agent, buyer agent, organization, room, status, `awaiting=seller_acceptance|buyer_settlement|…`), `/tasks/:id`, `/agents/:id/receipts`, `/task/config`.
- chat-app builders: `createTaskTx`, `acceptTaskTx`, `declineTaskTx`, `cancelOfferTx`, `expireOfferTx`, `markTaskDeliveredTx`, `settleTaskTx`, `refundTaskTx`, `releaseTaskTx`, `proposeRefundTx`, `acceptRefundTx`, `rejectRefundTx`, `withdrawRefundTx`, `reviewSellerTx`, `reviewBuyerTx`. Argument order mirrors Move exactly.
- Components: `TasksPanel` (tabs by role: Offers, Active, Delivered, Done), `CreateTaskDialog` (seller picker, amount, accept/delivery/settle windows, optional room), `TaskDetailPane`, `AcceptTaskDialog` (shows the full terms, the computed `terms_digest` and the fee that will apply, then Accept or Decline), `SettleTaskDialog` (confirm, with an optional "rate now" star picker appended to the same PTB), `RefundProposalCard` (propose, accept, reject, withdraw), `RefundTaskButton` (refund after a missed delivery, expire an offer, release to seller after the grace period), `ReviewTaskDialog`, and a Tasks tab in `AgentsPage`.

## Workstream 3 — Bidirectional reputation (stars + weighting + Bayesian average)

### Design
All math is unsigned integer math (u128 intermediates, checked). A review's influence is its **weight**, which cannot be forged because only a settled receipt can produce one, and each direction can be used once per receipt.

**Roles and subjects**
- **Per agent (`SubAgent`)**: `as_seller: ReputationAggregate` (buyers rate it as a seller) and `as_buyer: ReputationAggregate` (sellers rate it as a buyer). Each agent builds its own history in each role, so one principal's agents do not inherit each other's scores.
- **Human direct buyer (`MemoryAccount`)**: `human_as_buyer: ReputationAggregate`, used only when the buyer acts as the principal itself (`buyer_agent_id = none`). Sellers are always agents, so there is no human seller aggregate.
- **Credibility (principal level, both roles)**: `credibility: CredibilityStats` on `MemoryAccount`. This is the only principal-shared reputation state, and it is derived purely from completed transactions, never from ratings.
- **Overall** is derived per subject (see below) and is not stored.

**1. Review weight** (bps, 10000 = 1.0), identical for both directions:
- `value_weight_bps = clamp(amount_mist * 10000 / value_ref_mist, min_value_weight_bps, max_value_weight_bps)`. The cap stops a whale transaction from dominating, and the floor stops dust from counting for nothing.
- `rater_credibility_bps` = the rater principal's credibility **snapshotted on the receipt at settlement** (`buyer_credibility_bps` or `seller_credibility_bps`), so a party cannot boost its own weight with the same transaction, and the weight is fixed regardless of when the review arrives.
- `base_weight = value_weight_bps * rater_credibility_bps / 10000`.
- **Repeat-pair decay:** `w = base_weight / (1 + prior_pair_reviews)`. The 1st review from one rater principal to one reviewee principal in a given direction counts fully, the 2nd at 1/2, the 3rd at 1/3, and so on. Integer division may floor to 0, which is intended: the review still increments the histogram and `review_count` but adds nothing to the weighted sums.

**2. Credibility** (O(1), updated at settlement/release for **both** principals):
- `CredibilityStats { settled_count: u64, volume_mist: u128, credibility_bps: u64 }` on `MemoryAccount`.
- `credibility_bps = min(cred_max_bps, cred_base_bps + min(settled_count, cred_cap_n) * cred_step_bps)`. A new principal starts low; verified settled history (either role) raises it.
- The receipt snapshots the pre-update values. Credibility depends only on completed, paid transactions, never on ratings given or received.
- Emits `CredibilityUpdated` (one per principal per settlement).

**3. Pair interaction counter** (dynamic field, O(1), directional):
- A dynamic field on the **reviewee principal's `MemoryAccount` UID**, key `ReviewPairKey { rater_principal: address, rater_role: u8 }` (a `copy, drop, store` struct private to `memory.move`, so it cannot be forged or reset), value `u64`. Buyer→seller reviews are keyed `(buyer_principal, BUYER)` on the seller's account; seller→buyer reviews are keyed `(seller_principal, SELLER)` on the buyer's account.
- **Anti-farming stays principal-level even though scores are per agent:** the counter is keyed by principal pair, so rotating agents under the same two principals gives a fresh agent score but never a fresh pair-decay discount. Spinning up new agents cannot be used to farm reviews between the same two principals.
- `apply_review` does one `df::exists_` / `df::borrow_mut` (or `df::add` on first use), computes the decayed weight, then increments with a checked add. No history scan.

**4. Aggregates** (`ReputationAggregate`, a plain struct; nothing is stored per review):
```
review_count: u64
star_1..star_5: u64            // histogram
weight_sum_bps: u128           // Σ w
weighted_star_sum: u128        // Σ w * stars * 1000  (milli-star scale)
bayesian_score_milli: u64      // cached, 1000..5000
confidence_bps: u64            // cached W / (W + C), 0..10000
updated_at_ms: u64, last_receipt_id: Option<ID>
```

**5. Per-role Bayesian average** (integer-only, recomputed from the sums in O(1)):
```
score_milli = (C * m + weighted_star_sum) / (C + weight_sum_bps)
confidence  = weight_sum_bps * 10000 / (C + weight_sum_bps)
```
`m = prior_mean_milli` (default 3000) and `C = prior_weight_bps` (default 5 × 10000). A subject with no reviews shows exactly `m`. A few reviews, or reviews from low-credibility or low-value raters, barely move the score.

**6. Overall score** (weighted Bayesian average of both roles, derived in O(1) from the two aggregates of the same subject):
```
a_s = overall_seller_weight_bps,  a_b = overall_buyer_weight_bps   (a_s + a_b = 10000)
overall_score_milli = (C * m * 10000 + a_s * S_seller + a_b * S_buyer)
                    / (C * 10000      + a_s * W_seller + a_b * W_buyer)
overall_confidence  = (a_s * W_seller + a_b * W_buyer) * 10000 / (C * 10000 + a_s * W_seller + a_b * W_buyer)
```
- Same formula as the per-role score, with each role's evidence scaled by its role weight, so one prior `(m, C)` is shared and the overall score is itself a Bayesian average. A role with no reviews contributes nothing, and a new subject shows `m`.
- Both inputs now live on the same `SubAgent`, so the overall score depends on one object. On-chain: `public fun overall_reputation(seller: &ReputationAggregate, buyer: &ReputationAggregate, params: &ReputationParams): (u64 score_milli, u64 confidence_bps)` and `agent_overall(agent: &SubAgent, params)`.
- It is **derived, not stored**, so there is no second copy that can drift from the role aggregates and no extra write per review. Computing it costs a few integer operations. The reader joins the agent's two `reputation_aggregates` rows (roles seller and buyer, same `subject_id`) and applies the same formula with NUMERIC math. A shared test-vector file proves Move, the Rust reader and the TypeScript display agree.
- A human direct buyer's overall is its `human_as_buyer` score only (the seller term contributes nothing).

**7. Mutators** (all in `memory.move`; `agent_task` only calls them)
- Shared helper `apply_to_aggregate(agg: &mut ReputationAggregate, stars, weight_bps, params, receipt_id, clock)`: checked add of the weight and `weight * stars * 1000`, histogram and count increments, then recompute `bayesian_score_milli` and `confidence_bps`. All three aggregate locations reuse it.
- `public(package) fun apply_seller_review(seller_agent: &mut SubAgent, seller_account: &mut MemoryAccount, …)`: decays by the `(buyer_principal, BUYER)` pair counter on `seller_account`, then updates `seller_agent.as_seller`.
- `public(package) fun apply_buyer_agent_review(buyer_agent: &mut SubAgent, buyer_account: &mut MemoryAccount, …)`: decays by the `(seller_principal, SELLER)` pair counter on `buyer_account`, then updates `buyer_agent.as_buyer`.
- `public(package) fun apply_human_buyer_review(buyer_account: &mut MemoryAccount, …)`: same decay, then updates `buyer_account.human_as_buyer`.
- `public(package) fun record_settlement(account: &mut MemoryAccount, amount_mist, params, clock): u64`: returns the pre-update `credibility_bps` for the snapshot and updates `CredibilityStats`.
- None of them read `active`. A deactivated agent keeps accruing reputation on tasks it already accepted.

**8. Review entrypoints (agent_task.move)**
- `review_seller(config, task, buyer_account: &MemoryAccount, seller_account: &mut MemoryAccount, seller_agent: &mut SubAgent, stars, clock, ctx)`:
  - the sender must be the task's buyer (principal or the task's buyer agent), resolved through `resolve_actor_with_cap(cap_task_trade)` on `buyer_account`, which must match `task.buyer_memory_account_id`;
  - `task.status == SETTLED`; `clock <= review_deadline_ms`; `buyer_review_stars == 0`; `stars ∈ [1,5]` (`EInvalidStars`);
  - `seller_agent` and `seller_account` must match the task's seller ids;
  - weight uses `task.buyer_credibility_bps`;
  - sets `buyer_review_stars`, calls `apply_seller_review`, emits `TaskReviewSubmitted` and `ReputationUpdated`.
- The seller reviews the buyer through one of two entrypoints, chosen by whether the task had a buyer agent. Each asserts that it matches `task.buyer_agent_id`, so the wrong one aborts with `EWrongReviewTarget`:
  - `review_buyer_agent(config, task, buyer_account: &mut MemoryAccount, buyer_agent: &mut SubAgent, seller_agent: &SubAgent, stars, clock, ctx)` for tasks where `buyer_agent_id` is some. `buyer_agent` must be that agent. Updates `buyer_agent.as_buyer`.
  - `review_buyer_principal(config, task, buyer_account: &mut MemoryAccount, seller_agent: &SubAgent, stars, clock, ctx)` for tasks where `buyer_agent_id` is none (the human principal bought directly). Updates `buyer_account.human_as_buyer`.
  - Both: the sender must be the seller agent (`sub_agent_derived_address == sender`) or the seller principal; `SETTLED`, within the window, `seller_review_stars == 0`, valid stars; weight uses `task.seller_credibility_bps`; `buyer_account` must match `task.buyer_memory_account_id`; emits `TaskReviewSubmitted` and `ReputationUpdated` with `rater_role = SELLER`.
- The two directions are independent: neither review is required for, nor blocks, the other. Reviews are public on-chain at submission (blind reveal is out of scope for v1; see Risks).
- Anti-farming reused: `ETaskSelfDealing` at create, `min_escrow_mist`, the value-weight floor/cap, credibility growth, one review per direction per receipt, and the principal-keyed repeat-pair decay. Auto-released receipts are reviewable too.

**9. Checked arithmetic**
- Private helpers `checked_add_u128`, `checked_mul_u128`, `checked_div_pos` assert fit and a nonzero divisor, aborting with `EArithmeticOverflow` / `EDivideByZero`. All operands are widened to `u128` before multiplying. Counters use a checked `+ 1`.
- `bayesian_score_milli` is cast to `u64` only after asserting `[1000, 5000]`; confidence is asserted `≤ 10000`; the fee split asserts `platform_fee + ecosystem_fee ≤ amount`. The overall-score formula uses the same helpers.
- `validate_task_config` ensures no stored parameter can overflow or divide by zero.

**10. ReputationParams** (defined in `memory.move`, embedded in `TaskConfig`, passed by reference): `prior_mean_milli` [1000, 5000], `prior_weight_bps` (0, 1_000_000], `value_ref_mist > 0`, `min_value_weight_bps` > 0 and ≤ `max_value_weight_bps` ≤ 100_000, `cred_base_bps` > 0 and ≤ `cred_max_bps` ≤ 10000, `cred_step_bps ≤ 10000`, `cred_cap_n ≤ 10_000`, `overall_seller_weight_bps + overall_buyer_weight_bps == 10000`. All are checked by `validate_task_config`.

**11. Events (flat)**
- `TaskReviewSubmitted { task_id, receipt_id, rater_role (1 buyer, 2 seller), rater_principal, rater_agent_id: Option<ID>, reviewee_principal, reviewee_agent_id: Option<ID>, stars, rater_credibility_bps, value_weight_bps, base_weight_bps, prior_pair_reviews, weight_bps, reviewed_at_ms }`. The `task_reviews` row key `(task_id, rater_role)` mirrors the on-chain once-per-direction rule.
- `ReputationUpdated { subject_kind (1 agent-as-seller, 2 agent-as-buyer, 3 principal-as-human-buyer), subject_id (agent object id, or memory account id for kind 3), principal_owner, organization_id: Option<ID>, role (1 seller, 2 buyer), review_count, star_1..star_5, weight_sum_bps, weighted_star_sum, bayesian_score_milli, confidence_bps, receipt_id, updated_at_ms }`.
- `CredibilityUpdated { account_id, principal_owner, settled_count, volume_mist, credibility_bps, updated_at_ms }`.
- Replaces rev 3's `AgentReputationUpdated` and `ReviewerCredibilityUpdated`.

**12. Accessors:** `sub_agent_seller_reputation(&SubAgent)`, `sub_agent_buyer_reputation(&SubAgent)`, `account_human_buyer_reputation(&MemoryAccount)`, `account_credibility(&MemoryAccount)`, `overall_reputation(...)`, `agent_overall(...)`, and `empty_reputation(params)` (zeros with the score at the prior).

### Indexer / reads — every new event is read and processed
| Event (module) | Parser (`events.rs`) | Handler | Table written |
|---|---|---|---|
| `AgentRoomBound` (messaging) | `parse_messaging_event` | `MessagingHandler` | `agent_rooms` |
| `TaskOffered` (agent_task) | `parse_agent_task_event` | `TaskMarketHandler` | `agent_tasks` (insert) + outbox |
| `TaskAccepted` | same | same | `agent_tasks` (ACCEPTED, `accepted_at_ms`, `delivery_deadline_ms`) + outbox |
| `TaskDelivered` | same | same | `agent_tasks` (DELIVERED, deliverable, `settle_deadline_ms`) + outbox |
| `TaskRefundProposed` | same | same | `agent_tasks` (`refund_proposer`, `refund_proposed_at_ms`) + outbox |
| `TaskRefundProposalCleared` | same | same | `agent_tasks` (clears proposal) + outbox |
| `TaskReceiptSettled` | same | same | `agent_tasks` (status, `settlement_kind`, snapshots, `review_deadline_ms`) + `task_receipts` + outbox |
| `TaskRefunded` | same | same | `agent_tasks` (REFUNDED, `refund_kind`) + outbox |
| `TaskCancelled` | same | same | `agent_tasks` (CANCELLED, `close_kind`) + outbox |
| `TaskReviewSubmitted` | same | same | `task_reviews` (PK `task_id, rater_role`) + `agent_tasks.buyer_review_stars` / `seller_review_stars` + outbox |
| `TaskConfigUpdated` | same | same | `task_config` (latest) + `task_config_history` |
| `ReputationUpdated` (memory) | `parse_memory_event` | `SubAgentRegistryHandler` | `reputation_aggregates` (PK `(subject_id, role)`) |
| `CredibilityUpdated` (memory) | `parse_memory_event` | `SubAgentRegistryHandler` | `credibility` (PK `account_id`) |

- `TaskMarketHandler` is order-tolerant within a checkpoint: upserts use `on_conflict(...).do_update()`, task status only moves forward, and the refund-proposal fields are keyed by event order. "outbox" means the handler also inserts rows into `automation_event_outbox` in the same transaction (Workstream 5).
- `SubAgentRegistryHandler`: add `ReputationUpdated` and `CredibilityUpdated` to `SUB_AGENT_REGISTRY_EVENTS` with matching `SubAgentRegistryRow` variants and upserts.
- **Coverage guard test:** a Rust test lists every event struct emitted by `agent_task.move`, the new memory events and `AgentRoomBound`, and asserts each is in a handler's event list and has a parse arm.
- Migration `20261010120200_reputation`: `reputation_aggregates` (PK `(subject_id, role)`, `subject_kind`, histogram, sums as NUMERIC, cached score and confidence as plain columns, index on `(role, bayesian_score_milli)`) and `credibility`. Migration `20261010120100_agent_task_market` holds the task tables, including `task_reviews`.
- **Reader** (`myso-indexer-alt-social-reader`): for an agent, LEFT JOIN `reputation_aggregates` twice on the agent's own id (role seller and role buyer) and LEFT JOIN `credibility` on `memory_account_id`; an absent row means zero reviews, score = the prior from `task_config`. The overall score is computed in the reader with the shared integer formula using the current `task_config` role weights and prior.
- **GraphQL:** `SubAgent.reputation { asSeller, asBuyer, overall, credibility }` (each with counters, histogram, `score`, `confidence`), an account/profile `buyerReputation` (the human direct-buyer aggregate), and a root query `taskConfiguration` mirroring `memoryConfiguration` (update the schema snapshots).
- **Social server:** enrich `/profiles/:address/sub-agents` and `/sub-agents/:derivedAddress`; add `/agents/:id/reputation` (the agent's own seller, buyer and overall scores, plus its principal's credibility) and `/principals/:address/reputation` (human direct-buyer score and credibility).

### chat-app
- `ReputationBadge.tsx` shows the overall score ("★ 4.2 · 18 reviews") with a confidence indicator. Its tooltip shows the agent's own seller-role and buyer-role scores with counts and the histogram. Render in AgentChartNodeCard, AgentProfileDrawer, AgentListView and TasksPanel rows. Task rows show the **buyer agent's own buyer-role score** (or the human buyer's score) to sellers deciding whether to deliver, and the **seller's role score** to buyers choosing a seller.
- `StarRatingInput.tsx` (keyboard-accessible radio group) and `ReviewTaskDialog.tsx`: on a SETTLED task, each party sees their own "Rate buyer/seller" action until they have rated or `review_deadline_ms` passes; `TaskDetailPane` shows both stars once present. The buyer's `review_seller` can be appended to the settle PTB.
- `capabilities.ts`: add `task_trade: 1 << 17` to `CAP` and the registration preset; map `ESubAgentMissingCap` to "enable Trade tasks for this agent".
- `social-api.ts`: extend `SubAgentRow` with `reputation`; add `fetchAgentReputation`, `fetchPrincipalReputation`.
- Reputation is read-only except through the on-chain review calls.

## Workstream 4 — mysocial-frontend: TaskMarketCap section in the /ecosystem admin dialog
Existing pattern: `components/sections/ecosystem/ecosystem-admin-dialog.tsx` renders one `*Admin` section per config (`MemoryAdmin` wraps `ScalarProtocolConfigAdmin`, which checks `useAdminCapOwned(adminCapMoveType)` and builds the `moveCall`).
- **New `components/sections/ecosystem/task-market-admin.tsx`** (`TaskMarketAdmin`), rendered after `MessagingAdmin`. `adminCapMoveType = ${addresses.mysocialPackageId}::agent_task::TaskMarketCap`, `configAddressKey="taskConfigAddress"`, `moveTarget="agent_task::set_task_config"`, `apiPath="/api/task-market/configuration"`. Only a `TaskMarketCap` holder can edit.
- Fields (arg order mirrors `set_task_config`):
  - fees and escrow: `platform_fee_bps`, `ecosystem_fee_bps`, `min_escrow_mist`, the accept/delivery/settle window min, default and max values, `release_grace_ms`, `review_window_ms`, `max_title_bytes`, `max_uri_bytes`;
  - reputation: `prior_mean_milli`, `prior_weight_bps`, `value_ref_mist`, `min_value_weight_bps`, `max_value_weight_bps`, `cred_base_bps`, `cred_step_bps`, `cred_cap_n`, `cred_max_bps`, `overall_seller_weight_bps`, `overall_buyer_weight_bps`.
- **Permission toggles:** add a `type: 'toggle'` field kind to `ScalarProtocolConfigAdmin` (a Switch that sends a `bool` arg) for `active` (pause or resume the market). Everyone sees the toggle; only the cap holder can change it.
- `validateBeforeSave` mirrors `validate_task_config` exactly, including `min ≤ default ≤ max` for each window. The two role weights are linked: editing one auto-fills the other as `10000 − x`.
- Plumbing: `app/api/task-market/configuration/route.ts` (via `handleConfigurationGraphQLRoute`), `TASK_CONFIGURATION_QUERY` in `app/api/graphql/queries/ecosystem-configurations.ts`, `taskConfigAddress` / `taskMarketCapAddress` in `lib/mysocial-chain-addresses.ts`, and `taskConfig` + an `ownedSingleton('taskMarketCap', …)` entry in `mysocial-genesis-objects.ts`.
- Cap listing: add `{ key: 'taskMarket', type: '0x50c1::agent_task::TaskMarketCap' }` to `ADMIN_CAP_TYPES` in `components/sections/wallet/profile.tsx`, and widen `isAdminCapMoveRepr` (it only accepts names ending in `AdminCap`) to also accept `MarketCap`.
- `bootstrap.move` transfers the `TaskMarketCap` to the admin; `scripts/bootstrap.sh` needs no change.
- Tests: a unit test for the toggle and `validateBeforeSave` if the project has a test runner; otherwise a type check and `npm run build`. The rest of mysocial-frontend stays read-only.

## Workstream 5 — Autonomous agent execution (myso-memory automation + registered actions)

### What exists today (inspected)
- **Automation engine** `myso-memory/services/automation` (Rust): jobs with triggers (interval, cron, `event` with family/type and shallow `payload_filter`, scoped by `organization_id` / `account_id` / `agent_object_id`), run store, retries, budgets, per-job `cooldown_ms`, an in-process event bus, `/internal/automation/events` ingestion, and an additive event vocabulary (`docs/event_registry.json`, 15 families, "new families are additive — no engine code changes"). Only the `memory_relayer_call` action works; `social_action` and `webhook` currently record "not implemented". Few services publish events yet.
- **Memory bridge** `services/automation-sidecar` (TS): non-custodial, signs as an on-chain **delegate** (sub-agent) whose encrypted seed was uploaded from the owner's browser; it refuses delegates with capabilities beyond memory read/write and requires a spend cap and expiry.
- **Registered-action pipeline**: `@socialproof/sub-agents` (`action-registry/production-catalog.ts`, `ptb/*` builders, `SocialClient`) and the relayer (`services/server/src/routes.rs`: `registered_action_policy` maps each action id to a required capability and risk tier; `POST /api/chain/actions/prepare` and `/submit` with idempotency keys, sponsored gas through `scripts/sidecar-server.ts` `/social/prepare-registered`, `chain_actions` and `chain_action_approvals` tables, owner approval for risk tier 3; `memory_contract.rs` mirrors the capability bits).
- **Chat-app** already has `automation-client.ts`, `automation-delegate.ts`, `automation-job.ts`, `AutomationPanel`, `AutomationJobForm` and `AutomationDelegates`.
- **Prerequisite:** the engine README declares feature development on these three services frozen until the Railway end-to-end test passes. This workstream requires that gate to be lifted or the test passed first (see Sequencing).

### Design: reuse, don't rebuild
Agents act through the same path the UI-less social actions already use. The pieces added:

**A. Task actions in the registered-action catalog** (`myso-memory/packages/sub-agents`, `services/server`)
- New action ids, all requiring `CAP_TASK_TRADE` (mirrored as a new constant in `contract.ts` and `memory_contract.rs`, with its capability-name mapping), risk tier **1B** (automatic, with the on-chain per-action spend cap and `approval_required_caps` still applying, so owners can force approval on `task.create`/`task.settle` without any new mechanism): `task.create.v1`, `task.accept.v1`, `task.decline.v1`, `task.cancel_offer.v1`, `task.deliver.v1`, `task.settle.v1`, `task.refund.v1` (missed delivery), `task.release.v1`, `task.expire_offer.v1`, `task.propose_refund.v1`, `task.accept_refund.v1`, `task.reject_refund.v1`, `task.withdraw_refund.v1`, `task.review.v1` (the PTB builder picks `review_seller`, `review_buyer_agent` or `review_buyer_principal` from the task's roles).
- `ptb/task.ts` builders whose argument order mirrors the Move entrypoints; `SocialChainConfig` gains `taskConfigId`; `production-catalog.ts`, its tests and `PRODUCTION_ACTION_CATALOG_VERSION` bump to 1.4.0; the server's `SOCIAL_ACTION_REGISTRY_VERSION`, `registered_action_policy` and `registered_action_package_id` update in lockstep; `chain_discovery.rs` gains a `taskConfig` target (`TASK_CONFIG_ID`); `scripts/check-compatibility-contract.mjs` is extended so SDK, server and Move capability bits cannot drift; the sponsor `sidecar-server.ts` builder dispatch learns the `task.*` ids.
- Parameter validation uses the existing canonical-JSON parameter hash and idempotency scope, so an action can be retried without double execution.

**B. Task events into the automation engine**
- `docs/event_registry.json`: add family `task` (version 1) with types `offered`, `accepted`, `cancelled`, `delivered`, `refund_proposed`, `refund_cleared`, `refunded`, `settled`, `review_open`, `reviewed`, `expired_offer_due`, `release_due`. No engine change is needed for the vocabulary.
- **Outbox, not inline HTTP:** `TaskMarketHandler` writes `automation_event_outbox` rows in the same database transaction as the task update (`event_id`, `family`, `type`, `account_id`, `agent_object_id`, `organization_id`, payload, `dedup_key`, `created_at`, `sent_at`). One row per party that may need to react (for example `offered` targets the seller agent; `delivered` targets the buyer actor; `settled` yields a `review_open` row for each party), so triggers can be scoped by `agent_object_id` or `account_id`. Payloads are flat: `task_id`, `amount_mist`, windows and deadlines, counterparty ids, the `terms_digest`, and the counterparty's role-score and credibility so agent policies need no extra lookup.
- **Forwarder** in `myso-social-server` (a background task): selects unsent rows in order, `POST`s a `PlatformEvent` to `AUTOMATION_EVENTS_URL/internal/automation/events` with `x-internal-sync-secret`, marks `sent_at`, retries with backoff, and relies on the engine's `deduplication_key` so re-delivery is harmless. It runs only when the env is configured, so deployments without the engine are unaffected.
- **Time-driven steps:** `expire_offer`, `refund_task` and `release_task` are permissionless but time-triggered. The forwarder also emits `expired_offer_due` and `release_due` rows when `accept_deadline_ms` / `settle_deadline_ms + grace` pass (a small scan of indexed deadlines), so a keeper job can execute them. Any agent or keeper may call them; the contract guarantees funds are never locked.

**C. Engine: implement the two missing actions** (`services/automation/src/executor.rs`, `lib.rs`, `handlers.rs`, a new migration)
- `registered_action` replaces `social_action`: `config { action_id, params (templated from the triggering event, e.g. {{event.payload.task_id}}), policy }`. Before acting, the engine evaluates the **policy** against the event payload (for auto-accept: `min_amount_mist`, `max_amount_mist`, `min_delivery_window_ms`, `max_open_tasks`, `min_buyer_score_milli`, `min_buyer_credibility_bps`, allowed buyer principals, `scope_digest` allowlist). A failing policy records the run as **skipped** with the reason, never failed. It then calls the bridge, with an idempotency key derived from `(job_id, dedup_key)`.
- `webhook`: the "agent brain" hook for the **execute** step. The engine POSTs the signed event and task context to the job's HTTPS URL and accepts a JSON **directive** back (`{"action_id": "task.deliver.v1", "params": {…}}` or `{"noop": true}`); the directive is validated against the job's `allowed_action_ids`, then executed through the same registered-action path. Safeguards: HTTPS only, host allowlist per job, HMAC-SHA256 request signature with a per-job secret, 15 s timeout, 256 KiB response cap, private-network and metadata addresses blocked (dev override flag only), no redirects, directive size limits.
- Failure classes: transport errors retry per `retry_policy`; on-chain status aborts (`EWrongStatus`, deadline passed, already settled) are **permanent** and recorded as skipped/failed without retry; budget preflight applies only to AI-credit actions (chain actions are bounded on-chain by the delegate's spend cap).
- Events are ignored once stale (the payload's deadline passed), and `cooldown_ms` stays available for self-trigger protection.

**D. Bridge: delegates that can act on-chain** (`services/automation-sidecar`)
- New route `POST /internal/chain/act` (secret-gated like the other internal routes): the bridge opens the delegate row, verifies its chain state, then drives `prepare → sign → submit` through the relayer's registered-action API with the delegate's key (using `SocialClient`) and returns the digest. It never returns a key.
- **Capability allowlist per delegate class:** the current rule ("memory read/write only") becomes a set of classes. `memory` (today's caps 7) is unchanged. New class `task_worker`: memory caps plus `CAP_TASK_TRADE`. The bridge refuses any delegate carrying other caps (agent management, trading, AI spend, posting, and so on), and refuses to act unless the delegate has a spend cap and an expiry ≤ 90 days. The delegate row also stores an `allowed_action_ids` list (task actions only for this class), and the bridge rejects any `action_id` outside it, so a compromised engine cannot make a task worker post or trade.
- Nothing is cached: revocation, expiry or deactivation takes effect on the next call. `DELEGATE` binding to the job's `account_id` stays as is.

**E. Relayer proxy checks** (`services/server/src/automation_proxy.rs`): job creation derives `account_id` from the verified signature (as today) and additionally validates `registered_action` / `webhook` jobs: action ids in the task allowlist, the job's key ref is a `task_worker` delegate of the same account, trigger families limited to `task`, webhook host in the account's allowlist. The proxy stamps `agent_object_id` filters from the delegate, so an owner cannot subscribe to another account's events.

**F. chat-app** (`myso-messaging-stack/chat-app`)
- `automation-delegate.ts` + `AutomationDelegates.tsx`: a **Task worker** preset (capabilities `7 | task_trade`, a required spend cap and expiry, `allowed_action_ids` pinned to the task actions). `capabilities.ts` already gains `task_trade` (Workstream 3); the delegate view shows the class and spend cap.
- `automation-client.ts` types: `registered_action`, `webhook`, the `task` event family, `allowed_action_ids`, `policy`. `automation-job.ts` templates (pure and unit-tested): **Auto-accept offers** (trigger `task.offered` for the delegate's agent, policy fields from a small form, action `task.accept.v1`), **Task worker** (trigger `task.accepted`, action `webhook`, which returns a `task.deliver.v1` directive), **Buyer verifier** (trigger `task.delivered`, webhook returns `task.settle.v1`, `task.propose_refund.v1` or noop), **Auto-review** (trigger `task.review_open`, action `task.review.v1` with a fixed or webhook-chosen star value), **Keeper** (triggers `task.expired_offer_due` and `task.release_due`, actions `task.expire_offer.v1` and `task.release.v1`).
- `AutomationJobForm.tsx` gets a "Task automation" section that builds these templates, and `AutomationPanel` shows each run's action id, task id, digest and skip reasons. Everything the UI does is also reachable without it, because the same actions are plain registered actions.
- Optional room notification: a template step can call the existing `messaging.send_message.v1` action into the task's bound room (`room_group_id`) so humans following the room see "accepted / delivered / settled". It reuses the existing messaging capability and is off by default.

**G. Discovery**
- Tasks are **directed offers** (the buyer names the seller agent), so discovery for an agent is: (1) the `task.offered` event, and (2) polling `GET /tasks?seller_agent_id=…&status=OFFERED` (and `/agents/:id/receipts`, `/agents/:id/reputation`) for an agent that starts up late or recovers from downtime. An open marketplace board is out of scope for v1.

### Autonomous flow (no UI)
1. Buyer agent (a job or its own logic) runs `task.create.v1` for a named seller agent. The on-chain spend cap bounds the escrow.
2. `TaskOffered` → outbox → forwarder → engine event `task.offered` (scoped to the seller agent). The seller's auto-accept job checks its **policy** (amount, windows, buyer score and credibility) and runs `task.accept.v1`, which carries the `terms_digest` it verified from the event.
3. `task.accepted` fires the worker job: the webhook executes the work and returns a `task.deliver.v1` directive with the deliverable digest and URI.
4. `task.delivered` fires the buyer's verifier, which settles (`task.settle.v1`) or proposes a refund. If nobody acts, the keeper's `release_due` job pays the seller after the grace period.
5. `task.review_open` lets each side submit its review automatically.

## Affected repositories (summary)
| Repository | Changes |
|---|---|
| `myso-core` — Move (`myso-social`, `messaging`) | `agent_task` (acceptance flow, mutual refunds, frozen terms, reviews), `memory.move` (cap bit 17, reputation structs and mutators, credibility, pair counter), `bootstrap.move` (TaskConfig + TaskMarketCap), messaging room edits; tests |
| `myso-core` — indexer stack | `myso-indexer-alt-social` (parsers, `TaskMarketHandler`, outbox, reputation rows), `-social-schema` (migrations `20261010120000…120200`), `-social-reader`, `myso-social-server` (REST, outbox forwarder), `myso-indexer-alt-graphql` (reputation fields, `taskConfiguration`) |
| `myso-memory` | `packages/sub-agents` (task catalog entries, PTB builders, caps), `services/server` (registered-action policy, cap bits, discovery of `TASK_CONFIG_ID`, automation proxy validation), `services/automation` (`registered_action` and `webhook` actions, `task` family, migration), `services/automation-sidecar` (`/internal/chain/act`, delegate classes), docs and compatibility script |
| `myso-messaging-stack/chat-app` | rooms, task market UI, acceptance and refund-proposal UI, reputation UI, delegate preset, automation templates |
| `mysocial-frontend` | `/ecosystem` admin dialog `TaskMarketAdmin`, config route and query, chain-address and cap-listing entries |
| not touched | `myso-messaging-stack/move/packages/messaging` (stale mirror, out of scope), `myso-memory/apps/*`, all other repositories |

## Edge cases
- **Acceptance:** a seller cannot accept after `accept_deadline_ms`, with the wrong `terms_digest`, from the wrong sender, without `cap_task_trade` (agent sender), or while deactivated. An offer to a deactivated seller can still be declined by the seller principal, cancelled by the buyer, or expired by anyone, so the escrow always returns. The seller's `accept_task` and the buyer's `cancel_offer` race on the shared object; whichever lands first wins.
- **Frozen terms:** fee bps and windows are copied at create, so a `set_task_config` change never alters a pending or accepted task, and a paused market (`active = false`) only blocks new offers: accept, deliver, settle, release, refunds and reviews continue so nothing is trapped.
- **Mutual refund:** only DELIVERED tasks use it. A proposer cannot accept its own proposal. A pending proposal never extends `settle_deadline_ms`, never blocks `release_task`, and is voided by settlement. A proposal by one party blocks a competing proposal by the other until cleared. If the seller never answers, the buyer's only unilateral options are to settle (pay) or wait for the release; if the buyer never answers a seller-proposed refund, the normal settle or release path still applies.
- **Auto-payment preserved:** `release_task` pays the seller after `delivered_at + settle_window + release_grace`, without any party action and regardless of proposals.
- **No locked escrow:** OFFERED → accept, cancel, decline or expire; ACCEPTED → deliver or refund after the delivery deadline; DELIVERED → settle, release or mutual refund.
- Refunded, cancelled and unaccepted tasks have no receipt, so no reviews and no credibility change. Settled and auto-released tasks both create a receipt that either party may review once per direction within `review_deadline_ms`; the two directions are independent.
- **Deactivated agents:** `create_task` and `accept_task` require an active agent, so a deactivated agent cannot take on new work. Everything after acceptance (deliver, settle, release, refunds, decline, reviews, all aggregate updates) performs no `active` check, and reputation history stays on the objects.
- Self-dealing is rejected at create. Dust is blocked by `min_escrow_mist` and the weight floor; whales are capped by the weight ceiling; repeat pairs are decayed per direction at principal level; new principals start with low credibility; scores are per agent, and fresh agents still inherit principal credibility and pair decay.
- A task with a buyer agent is reviewable only through `review_buyer_agent`, and a human-direct task only through `review_buyer_principal`.
- **Automation:** duplicate events are absorbed by the dedup key and the action idempotency key; a stale event whose deadline passed is skipped; an on-chain abort for a wrong status is permanent and never retried; a webhook that returns an action outside `allowed_action_ids` is rejected and recorded; a revoked or expired delegate fails closed on the next call; a job paused or deleted stops all further actions; an engine outage does not lose events (they stay in the outbox); the forwarder being down never blocks indexing.
- Overflow aborts with `EArithmeticOverflow`; division by zero is excluded by config validation; zero fees, `@0x0` platform and bad lengths behave as in rev 2.

## Tests and verification
**Move** (`cd myso-core/crates/myso-framework/packages/myso-social && myso move test`):
- `agent_task_tests.move`:
  - **acceptance:** create → accept → deliver → settle; accept after the deadline aborts; wrong `terms_digest` aborts; accept by a non-seller, by an agent without the cap, and by a deactivated agent aborts; accept twice aborts; the seller principal can accept and decline; `decline_task` (even deactivated) refunds in full; `cancel_offer` works only before acceptance; `expire_offer` only after the deadline and by anyone; deliver on an OFFERED task aborts; frozen terms: changing config after create leaves fees and windows unchanged; `active = false` blocks `create_task` only;
  - **windows:** delivery deadline = `accepted_at + delivery_window`; settle deadline = `delivered_at + settle_window`; the release boundary is exact at `settle_deadline + grace`; checked-add overflow aborts;
  - **refunds:** `refund_task` works only on ACCEPTED past the delivery deadline and aborts on OFFERED and DELIVERED; there is no entrypoint for a unilateral post-delivery buyer refund (compile-level absence plus tests of every other path); `propose_refund` by the buyer then `accept_refund` by the seller refunds in full; the same with the seller proposing and the buyer accepting; a proposer cannot accept its own proposal; the second party cannot propose while one is pending; `reject_refund` and `withdraw_refund` clear it; a pending proposal does not stop `release_task` or `settle_task`, and after a release `accept_refund` aborts; after `accept_refund`, settle and release abort;
  - **races:** settle vs release vs refund vs accept_refund (the loser aborts on status); accept vs cancel vs expire;
  - credibility snapshots are taken before the update; both principals' credibility rises on settle and release; unaccepted, cancelled and refunded tasks change no credibility;
  - deactivated seller after acceptance: deliver, settle, release, reviews and aggregate updates all still work;
  - `expected_failure`: self-dealing, dust, invalid windows, missing cap, bad lengths, wrong deliverer, double settle, every `validate_task_config` bound at genesis and in `set_task_config`.
- `agent_reputation_tests.move`:
  - review gating: stars 0 or 6 abort; a review before SETTLED aborts; a review after `review_deadline_ms` aborts; a second review in the same direction aborts; refunded and cancelled tasks cannot be reviewed; a wrong-party sender aborts; both directions can review the same receipt in either order, and reviews of an auto-released receipt work;
  - role separation: a buyer→seller review changes only the seller agent's `as_seller` and the seller account's pair counter; a seller→buyer review on an agent-bought task changes only that buyer agent's `as_buyer`; on a human-bought task it changes only the account's `human_as_buyer`; the wrong buyer entrypoint aborts with `EWrongReviewTarget`;
  - **per-agent isolation:** two agents under one principal accumulate independent seller and buyer scores, and reviewing one leaves the other at the prior; both still share the principal's credibility and pair counters;
  - a new subject's score equals the prior; one review matches the hand-computed Bayesian value; low-credibility and low-value reviews have less impact; the weight cap holds; N identical 5★ reviews converge monotonically; the histogram sums to `review_count`;
  - **overall:** with no reviews it equals the prior; with only seller reviews it equals the formula with `a_b` contributing nothing; with both roles on one agent it matches the weighted hand computation; a human direct buyer's overall equals its buyer-role score; changing the role weights shifts the result as expected; `agent_overall` equals the off-chain reader vectors;
  - pair decay in both directions: the 1st, 2nd and 3rd review apply `base`, `base/2`, `base/3`; a different rater principal starts at full weight; switching or newly registering agents under the same two principals does not reset the decay; the dynamic field is created once then incremented; buyer→seller and seller→buyer counters are independent;
  - checked arithmetic: accumulators near `u128::MAX` abort with `EArithmeticOverflow`; zero divisors abort; the cached score stays in `[1000, 5000]`; event payloads equal state.
- Regressions: `memory_tests`, `memory_hierarchy_tests`, `memory_organization_tests`, `access_hardening_tests`, `upgrade_tests`. Extend `messaging/tests/agent_messaging_tests.move` for `MetadataAdmin`, `group_data` and `AgentRoomBound`.

**Rust (myso-core):** row/parse tests for every new event, the coverage guard test (every Move event has a parse arm, a handler entry and, for task events, an outbox mapping), handler tests that one event produces the right outbox rows per party, forwarder tests (retry, `sent_at`, secret header, dedup key stable on re-send, no send when unconfigured), reader tests for the overall formula against the shared vectors, then `cargo check -p myso-indexer-alt-social -p myso-indexer-alt-social-schema -p myso-indexer-alt-social-reader -p myso-social-server -p myso-indexer-alt-graphql` and `MYSO_SKIP_SIMTESTS=1 cargo nextest run -p myso-indexer-alt-social`.

**myso-memory:**
- `packages/sub-agents`: catalog test (every `task.*` id present, tier 1B, requires the task cap), PTB builder argument-order tests per entrypoint, canonical parameter-hash vectors, `check-compatibility-contract.mjs` passing for the new cap bit and version.
- `services/server` (`cargo test`): `registered_action_policy` for all task ids (cap and tier), prepare/submit idempotency for a task action, approval-required behaviour when the owner marks `task_trade` approval-required, automation proxy validation (wrong account, foreign delegate, non-task action id, bad webhook host all rejected).
- `services/automation` (`pnpm test:automation`): the `task` family is accepted by the registry, event triggers filter by `agent_object_id`, policy evaluation (each field pass and fail), skipped-versus-failed classification, idempotency key stability, stale-event skip, `webhook` safeguards (HTTPS only, allowlist, SSRF block, redirect refusal, timeout, size cap, HMAC signature, directive allowlist), permanent-versus-retryable errors.
- `services/automation-sidecar` (`pnpm test:bridge`): `task_worker` delegates accepted; delegates with any extra cap, no spend cap or too long an expiry refused; `allowed_action_ids` enforced; revocation honoured on the next call; keys never in any response or log.
- `pnpm smoke:automation` extended with a stubbed task flow (offer event → auto-accept → deliver directive), and the §12 Railway end-to-end checklist gets a task scenario.

**Frontend:**
- chat-app: `src/lib/agents/{tasks,rooms,reputation}.test.ts` (including acceptance-terms digest parity with Move, refund-proposal state machine helpers, overall-score formatting against the shared vectors), `automation-job.test.ts` for the new templates, `automation-delegate.test.ts` for the task worker preset, and `automation-client.test.ts` for the new types; then `pnpm test`, `pnpm build`, `./scripts/lint.sh`.
- mysocial-frontend: the toggle and `validateBeforeSave` unit test if a runner exists, otherwise a type check and `npm run build`.

**End-to-end (fresh genesis, no UI interaction for the agent steps):** bootstrap the chain; register two agents under different principals, give each a `task_worker` delegate with a spend cap; start the indexer, social server (with forwarder), automation engine and bridge. Run the autonomous flow above: the buyer creates an offer, the seller auto-accepts within its policy, the worker webhook delivers, the buyer verifier settles, both sides auto-review, and the DB shows `agent_tasks`, `task_receipts`, `task_reviews`, `credibility` and `reputation_aggregates` rows plus `sent` outbox rows and succeeded engine runs. Then check the variants: an offer outside the seller's policy is **skipped** and later expires and refunds; an unanswered offer is expired by the keeper job; a missed delivery is refunded by the keeper; a delivered task with no buyer action is released to the seller after the grace period; a buyer-proposed refund is accepted by the seller and refunds in full, while the same proposal ignored by the seller leaves the release path intact; a seller deactivated after acceptance can still deliver and be paid; an engine or forwarder outage delays but never loses actions; a revoked delegate's next action fails closed. Check the admin dialog toggles and fields against `taskConfiguration`.

(Per project preference, the user runs all builds and tests; implementation writes code and tests only. This document is the plan only; nothing is implemented yet.)

## Sequencing
- P0: persist the plan; confirm the automation-stack freeze is lifted or the Railway end-to-end test passes (Workstream 5 depends on it).
- P1: Rooms.
- P2: Task market with acceptance and mutual refunds, frozen terms, credibility snapshots, `taskConfiguration` and the admin dialog (Workstream 4). No reviews yet.
- P3: Reputation (aggregates, credibility, review entrypoints, events, indexer, reads, overall score, badges, review dialog).
- P4: Automation: registered-action catalog and server policy, outbox and forwarder, engine `registered_action` and `webhook`, bridge `/internal/chain/act` and delegate classes, proxy validation, chat-app templates.
- P5: Polish (capability copy, optional SDK codegen, optional room notifications).

## Risks
- **Automation freeze and maturity:** the engine and bridge are marked testnet-only with a pending integration test, and `webhook` and chain actions are new surface there. Mitigations: lift the freeze deliberately, ship behind `AUTOMATION_ENABLED`, bound every delegate with a spend cap, expiry and action allowlist, and keep all money movement guarded on-chain regardless of the engine.
- **Delegate custody:** a compromised bridge and database yield delegates that can run task actions up to their spend cap and expiry. The `task_worker` class is deliberately narrow (task actions only, no posting, trading or AI spend), and owners can revoke at any time or require approval for `task_trade` through the existing `approval_required_caps`.
- **Webhook trust:** a worker endpoint decides what to deliver or settle. The engine's allowlist, signature, SSRF controls and directive allowlist limit the damage, but owners remain responsible for their endpoint. Settlement policies should favour conservative defaults (verify before settle).
- **Mutual refunds can deadlock by design:** if the seller never accepts a proposed refund, the buyer's only options are to settle or wait for the release. This favours the seller by intent (to protect delivered work) and a future dispute step could add arbitration.
- **Offer spam:** a buyer could lock small escrows against many sellers. The cost is the escrowed funds plus `min_escrow_mist`, and offers expire; seller-side policy filters amounts and buyer quality. A per-seller open-offer cap is intentionally not added to avoid contention on the seller's `SubAgent`.
- Parameter tuning: the reputation defaults (prior 3.0★, C = 5 reviews, equal role weights, credibility ramp) and the new window bounds need a sanity pass; all are on-chain config.
- Adding `credibility` and `human_as_buyer` to `MemoryAccount` and `as_seller` / `as_buyer` to `SubAgent` touches core structs. Each has a single construction site, and the regression suites gate it.
- Reviews are public at submission, so a second reviewer can see the first. Mitigations are credibility, value weighting and pair decay; commit-reveal is a possible later addition.
- Scores are per agent, so a principal can abandon a poorly rated agent and start a fresh one. Principal-level credibility and pair decay survive the switch, and the UI shows credibility with every score.
- Contention: settlement touches both principals' `MemoryAccount`s and the `TaskEscrow`; reviews touch the reviewed agent and the relevant account.
- BCS and contract drift: each event has several mirrors (Rust parser, indexer model, TS types, fixtures), and the capability bit now exists in four places (Move, `contract.ts`, `memory_contract.rs`, chat-app `capabilities.ts`). Every event gets a parse test, the compatibility script guards the cap bit, and there are no nested payloads.
