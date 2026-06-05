# 5 Governance

Governance is how a user-owned coordination economy changes without becoming a platform dictatorship. It determines how disputes are resolved, how markets evolve, how treasury actions are routed, how platform communities govern themselves, and how infrastructure upgrades become legitimate. Because governance affects every other market in MySocial, it receives its own dedicated treatment here rather than only the brief summary in Section 4.7.

The on-chain governance module (`social_contracts::governance`) implements a two-stage decision process. Proposals first pass through a delegate council for review, then advance to a community voting phase when the council approves them. Separate registries keep ecosystem authority, Proof of Creativity authority, and platform authority from colliding. Stake-backed submission costs, quorum thresholds, quadratic vote pricing, optional anonymous ballots, and explicit treasury routing make outcomes auditable rather than discretionary.

Operational product work remains distinct from on-chain legitimacy. The Social Proof Foundation and application teams can ship software, respond to incidents, and coordinate day-to-day development, but they do not receive unilateral on-chain authority over community treasuries, delegate selection, or proposal finalization. That separation is intentional. Legitimacy comes from protocol rules, registry boundaries, and participant votes recorded on chain.

## 5.1 Coordination and Decision-Making

Good governance increases coordination capacity. It gives participants a way to propose changes, review tradeoffs, filter spam, debate consequences, vote, finalize outcomes, and execute decisions. In an agentic economy, governance also gives agents a legitimate source of policy. An agent can explain a rule, summarize a proposal, or prepare a vote recommendation, but it does not invent the rule.

MySocial preserves the distinction between global and local authority. Ecosystem decisions do not override platform community decisions, and platform decisions do not gain accidental control over ecosystem funds. Registry separation, public delegate rationales, quorum thresholds, configurable voting windows, and clear treasury routing maintain that boundary.

Each `GovernanceDAO` registry stores the parameters that define how decisions are made within its scope:

| Parameter | Role |
|-----------|------|
| `delegate_count` | Size of the active delegate council |
| `delegate_term_epochs` | Epoch length of a delegate term; panel refresh applies at completed term boundaries |
| `proposal_submission_cost` | MYSO stake placed into the proposal reward pool at submission |
| `max_votes_per_user` | Maximum vote weight a community participant may cast on one proposal |
| `quadratic_base_cost` | Base unit for quadratic pricing on additional vote weight |
| `voting_period_ms` | Community voting window after delegate approval |
| `quorum_votes` | Minimum total community vote weight required for a valid outcome |

Bootstrap defaults illustrate how scope changes the cadence of decisions. The ecosystem registry begins with three delegates, ninety-epoch terms, a 100 MYSO submission cost, up to ten votes per participant, a 10 MYSO quadratic base, a seven-day voting window, and a quorum of twenty vote weight. The Proof of Creativity registry uses a smaller council, longer terms, lower costs, shorter voting windows, and a lower quorum. Platform registries are created when a platform is approved and inherit parameters set at creation, with later updates controlled by the platform developer through the platform module.

Governance parameters themselves can be updated. Ecosystem and Proof of Creativity registries require the `GovernanceAdminCap`. Platform registries require the platform developer address and must remain typed as platform registries. This keeps parameter evolution inside the same authority model as the decisions those parameters govern.

## 5.2 Governance Registries

The governance layer operates through three proposal registries, each implemented as a shared `GovernanceDAO` object with its own delegate panel, proposal set, treasury balance, and configuration.

**Ecosystem registry (`PROPOSAL_TYPE_ECOSYSTEM`).** Covers ecosystem-wide upgrades, shared parameters, infrastructure decisions, and treasury routing that affects the broader MySocial economy.

**Proof of Creativity registry (`PROPOSAL_TYPE_PROOF_OF_CREATIVITY`).** Covers attribution disputes, creativity outcomes, redirection decisions, and related appeals tied to creative content references.

**SPoT registry (`PROPOSAL_TYPE_SPOT` = 2).** Shared global registry for contested Social Proof of Truth resolution proposals (binary ratification of oracle-escalated markets).

**Platform registry (`PROPOSAL_TYPE_PLATFORM` = 3).** Covers platform-specific policy, moderation, treasury use, access rules, and application-level decisions for an approved platform community. Platform registries are created through `create_platform_governance` when platform approval occurs. The approving transaction sender is seeded as the founding delegate under the same pattern used at bootstrap.

Proposal type and registry type must match. A participant submits to the registry that owns the decision domain. Applications, indexers, and companion services can derive reputation and source quality from protocol events without collapsing every signal into one universal on-chain reputation object unless a deployed module explicitly implements one.

## 5.3 Delegate Council

The delegate council is the first deliberative stage of MySocial governance. Delegates review proposals before they reach community voting, publish optional public rationales, and act as a spam filter and quality gate for stake-backed submissions. They are not a substitute for community authority. A proposal still requires community finalization after council approval.

### Nomination and community signaling

Any wallet may nominate itself through `nominate_delegate` without a profile requirement. A nominee is scheduled for the next completed term boundary and accumulates community support through upvotes and downvotes rather than through a centralized appointment process.

Community members signal support or opposition with `vote_for_delegate` and may later neutralize a prior signal with `clear_vote_for_delegate`. Votes are one address, one current position per target, and self-voting is prohibited. The same signaling applies to active delegates and to nominees waiting for the next panel refresh.

This design replaces a one-shot election event with continuous accountability. A delegate who loses community confidence can be downvoted before the next boundary refresh. A strong nominee can rise through net support without waiting for a separate candidacy gate enforced by an admin key.

### Panel refresh and term boundaries

Delegate panels refresh lazily at epoch boundaries through `try_update_delegate_panel_if_due`, which runs during nomination, delegate voting, proposal submission, delegate review votes, and community finalization. A dedicated keeper may also call `update_delegate_panel` when governance traffic is idle but a boundary is overdue.

At each completed boundary of `delegate_term_epochs`, the registry recomputes the council from incumbents and nominees. Candidates are ranked by net community score, using a baseline offset so downvotes can materially reduce standing. Ties favor incumbents. The top `delegate_count` addresses become the new council; counters for reviewed and submitted proposals reset for the new term, while historical sided-win and sided-loss statistics accumulate over time for indexer and product use.

The model scales by changing `delegate_count` through governance parameter updates. The contract requires more than one delegate but does not hard-code seven seats. Product policy may prefer odd-sized councils to reduce tie risk during review, yet the protocol expresses that preference through configuration rather than a fixed constitutional number.

### Delegate review of proposals

When a proposal is submitted, it enters `STATUS_DELEGATE_REVIEW`. Each delegate may vote once through `delegate_vote_on_proposal`, optionally attaching a public reason string that indexers can display.

Review outcomes follow simple majorities over the active council:

- If approvals exceed half of the current delegate table, the proposal moves to community voting.
- If rejections exceed half of the current delegate table, the proposal is rejected and its reward pool is routed according to registry rules.
- Otherwise the proposal remains in review until more delegates vote.

Delegates cannot use anonymous community ballots. The contract rejects anonymous submissions from active delegates so council accountability remains public even when community participants choose privacy.

## 5.4 Proposal Lifecycle

A proposal is a shared on-chain object with title, description, optional reference identifiers, optional metadata JSON, submission timestamp, status, vote counters, and a MYSO reward pool funded at submission.

### Submission and rescission

Submission functions require a MYSO payment at or above `proposal_submission_cost`. That payment is deposited into the proposal reward pool rather than burned, aligning submitter incentives with eventual implementation or forfeiture rules.

Ecosystem and Proof of Creativity proposals use typed entry points such as `submit_ecosystem_proposal` and `submit_proof_of_creativity_proposal`. Proof of Creativity submissions must reference creative content. Platform proposals are submitted against the platform registry created for that community.

While a proposal remains in delegate review, the submitter may `rescind_proposal` and recover the reward pool. After the council advances a proposal to community voting, rescission is no longer available because community participants may already have committed vote weight or encrypted ballots.

### Community phase and finalization

Council approval starts the community window. Voting opens at `voting_start_time` and closes at `voting_end_time`, measured in milliseconds from the chain clock.

After the window ends, any eligible finalization path may call `finalize_proposal` or the anonymous equivalent. Finalization requires the proposal to be in community voting and the voting period to have ended. If total vote weight meets quorum and approvals exceed rejections, the proposal becomes `STATUS_APPROVED`. If quorum is met but rejections prevail, or if quorum is not met, the proposal becomes `STATUS_REJECTED`.

Approved proposals can later be marked implemented. Implementation may release the reward pool to the submitter or route value through treasury objects depending on registry type and the implementing entry point. Failed proposals forfeit their pools to the ecosystem treasury or the platform governance treasury with explicit forfeit-reason events for indexers.

### Status model

The protocol tracks proposals across a explicit status set:

| Status | Meaning |
|--------|---------|
| Submitted | Reserved in the status model; new submissions enter delegate review directly |
| Delegate review | Council is deliberating |
| Community voting | Community participants may cast ballots |
| Approved | Community quorum and majority succeeded |
| Rejected | Council rejection, community rejection, or quorum failure |
| Implemented | Approved action recorded as executed |
| Owner rescinded | Submitter withdrew during council review |

This lifecycle makes governance legible to applications and agents. A governance assistant can monitor status transitions, surface council rationales, estimate quorum progress, and warn users before voting windows close without claiming authority it does not possess.

## 5.5 Community Voting, Quorum, and Anonymous Ballots

Community voting is the final on-chain authority for proposals that survive delegate review. It is not a token-weighted plutocracy in the contract layer. Participation is open to wallets that are not blocked by application policy, and vote weight is chosen per proposal within configured bounds.

### Quadratic vote weight

Each participant selects an integer `vote_count` up to `max_votes_per_user`. The first unit of vote weight is free. Additional weight costs MYSO according to a quadratic formula:

```
vote_cost = quadratic_base_cost × (vote_count² − 1)   when vote_count > 1
vote_cost = 0                                         when vote_count = 1
```

Paid vote costs are added to the proposal reward pool. The participant’s chosen weight is recorded as `vote_weight` and applied entirely to either the for or against tally. Quadratic pricing makes large vote purchases expensive, which reduces blunt buying of outcomes while still allowing committed participants to express intensity.

Each address may vote once per proposal in the public path. Finalization compares total vote weight for and against, not raw participant count.

### Quorum and outcome rules

Let `total_votes = community_votes_for + community_votes_against`.

- If `total_votes < quorum_votes`, the proposal fails for insufficient participation and the reward pool forfeits with reason quorum not met.
- If quorum is met and `community_votes_for > community_votes_against`, the proposal is approved.
- If quorum is met and approvals do not exceed rejections, the proposal is rejected and the reward pool forfeits with reason community rejected.

After finalization, the contract updates delegate track records when council members sided with or against the winning community outcome. That statistic supports reputation views but does not by itself remove a delegate from office. Removal flows through the panel refresh ranking described in Section 5.3.

### Anonymous community ballots

Community participants who are not active delegates may submit encrypted ballots through `community_vote_anonymous` using MyData BF-HMAC encryption objects. Anonymous votes are stored on the proposal until finalization, when authorized decryption merges them into the public tally through `finalize_proposal_anonymous`.

Anonymous voting is intentionally unavailable to delegates. Council review remains attributable; community participation may remain private. This split preserves review accountability without forcing every participant to reveal position publicly.

## 5.6 Dispute Resolution

Disputes are unavoidable when markets coordinate creativity, truth, risk, and reputation. Proof of Creativity disputes may involve similarity, originality, remix rights, or fee redirection. Social Proof of Truth disputes may involve oracle confidence, evidence quality, contested outcomes, or governance finalization. Platform disputes may involve moderation, membership, access, or treasury use.

MySocial treats dispute resolution as a core market function rather than an afterthought. Stake-backed proposals, delegate review, community voting, evidence references in proposal metadata, and escalation into the correct registry make contested outcomes more legitimate than ad hoc moderation alone.

The registries connect governance to the rest of the economy. A Proof of Creativity proposal can reference the creative content under dispute. Ecosystem proposals can reference shared infrastructure or treasury objects. Platform proposals remain scoped to the platform registry that owns local policy. Agents can organize evidence, summarize prior events, and explain procedural next steps, but they do not replace accountable voting.

## 5.7 Operational Authority and Treasury Separation

MySocial distinguishes three layers of authority that often collapse together in consumer products.

**Operational authority** belongs to teams that build, deploy, and maintain software. This includes foundation staff, core contributors, validators, indexers, and application developers. Operational teams can move quickly on implementation details, user experience, and incident response when those actions stay inside normal product and infrastructure boundaries.

**Council authority** belongs to delegates elected through the panel refresh process. Delegates filter proposals, publish reasons, and decide whether community voting should begin.

**Community authority** belongs to participants who cast public or anonymous ballots after the council approval threshold is met.

On-chain rules enforce important limits that operational authority cannot override. Delegates and community voters decide proposal outcomes through the contract paths above. Failed proposal stakes forfeit to treasury destinations recorded by event, not to arbitrary addresses. Ecosystem and Proof of Creativity rejections route to the ecosystem treasury object. Platform rejections route to the platform governance treasury balance. Implementation rewards and treasury movements use explicit entry points so indexers can audit them.

This is the practical meaning of checks and balances in MySocial. Operational speed remains available off chain and in application layers. Monetary and constitutional changes require stake, review, voting, quorum, and auditable settlement on chain.

## 5.8 Agents in Governance

Agents can make governance usable at scale. They can summarize proposals, compare parameter changes, detect duplicates, flag conflicts of interest, translate technical language, identify affected communities, and prepare vote recommendations. These roles are valuable because large coordination systems produce more information than most users can review manually.

The boundary remains authority. An agent submits proposals, casts votes, moves treasury funds, or finalizes outcomes only when the user or organization grants an explicit governance capability through the Memory layer. Governance remains a human and organizational legitimacy process, with agents increasing comprehension and throughput.

Useful agent workflows include the following.

- Monitoring proposals across ecosystem, Proof of Creativity, and platform registries without conflating their authority domains.
- Explaining quadratic vote cost before a user commits MYSO to additional weight.
- Tracking quorum shortfalls near voting deadlines.
- Summarizing delegate reasons during council review.
- Preparing implementation checklists after approval, while leaving execution to authorized humans or explicitly capped agents.

MySocial does not treat agents as a shadow executive branch. They are permissioned coordinators inside user-owned context. Governance events, delegate statistics, proposal metadata, and treasury routing events give those coordinators verifiable state to reason about.

## 5.9 Participant Roles

Governance participation is open at multiple depths.

**Submitter.** Any wallet meeting the submission cost can propose changes to the appropriate registry, reference relevant objects, and attach metadata for indexers and applications to interpret.

**Community signaler.** Any wallet can upvote or downvote delegates and nominees, shaping the next council without holding a formal office.

**Delegate.** Council members review proposals, publish reasons, and determine whether community voting begins.

**Voter.** Community participants choose vote weight, pay quadratic costs when increasing weight, and finalize outcomes through public or anonymous paths.

**Platform operator.** Approved platforms receive a platform registry and may tune governance parameters within platform authority, subject to the platform module’s ownership checks.

**Agent.** Delegated software actors may assist with comprehension and preparation when granted capabilities, but on-chain legitimacy still flows through the proposal, council, and community sequence.

Taken together, these roles implement the claim stated throughout the white paper. Users own the context that markets and agents depend on, and governance is the process by which that context evolves without surrendering it back to platform custody.
