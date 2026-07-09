# Resolver Compiler

The Resolver Compiler turns a **canonical claim** (deterministic normalized fields from LLM extraction + canonicalization) into an immutable **`ResolverDefinition`** that the resolver engine executes at maturity.

## Pipeline

```
Post text
  → LLM `ExtractedClaim` (interpretation only)
  → `canonicalize()` → `CanonicalClaim`
  → `rules::evaluate()` → Accepted | Rejected
  → `ResolverCompiler::compile()` → `CompiledMarketSpec`
  → persist `resolver_definitions` + enqueue create/resolve/refund jobs
```

The scheduler and resolver engine **never** read post text or call the LLM.

## LLM JSON schema (`ExtractedClaim`)

| Field | Required | Notes |
|-------|----------|-------|
| `subject`, `predicate`, `object` | yes | Core claim triple |
| `metric` | price claims | e.g. `"price"` |
| `comparison` | price/custom | `lt\|lte\|gt\|gte\|eq\|neq` |
| `threshold` | price | Decimal string |
| `deadline` | non-price | ISO-8601 UTC |
| `outcome_type` | yes | `binary\|multi_choice\|scalar` |
| `suggested_sources` | no | Adapter hints |
| `suggested_options` | yes | 2–10 unique labels |
| `claim_category` | yes | See below |
| `resolver_hints` | per kind | Kind-specific fields |

### `claim_category`

| Value | `ResolverKind` | Required hints |
|-------|----------------|------------------|
| `price_threshold` | `PriceThreshold` | threshold + comparison |
| `release_published` | `ReleasePublished` | owner + repo (or `object` as `owner/repo`) |
| `event_occurrence` | `EventOccurrence` | feed_url + match_predicate |
| `custom_http` | `CustomHttp` | url + json_path + expected |
| `unsupported` | — | Rejected at review |

### `resolver_hints` fields

- **Price:** `preferred_sources[]`
- **Release:** `owner`, `repo`, `tag_predicate`
- **Event:** `feed_url`, `match_predicate`, `match_fields[]`
- **Custom HTTP:** `url`, `json_path`, `comparison`, `expected`

## Compile algorithm

1. Dispatch by `claim_category` to `price.rs`, `release.rs`, `event.rs`, or `custom_http.rs`.
2. `source_select.rs` picks a registered `TrustedSource` deterministically (preferred → fallback order → trust score → lexicographic id).
3. `maturity.rs` sets `maturity_at` and `deadline`.
4. `validate.rs` ensures every spec field is non-empty and adapters support the definition.
5. `compile_fingerprint` = SHA-256(canonical fields + spec + sorted source_ids).

## Examples

### BTC price (E2E runnable default)

Post: *"Will BTC trade above $1?"*

- category: `price_threshold`
- asset: `bitcoin`, quote: `usd`, comparator: `gt`, threshold: `1`
- source: `coingecko`, json_path: `bitcoin.usd`
- options: `["Yes", "No"]`

### GitHub release

Post: *"Will rust-lang/rust tag 1.80 ship by Friday?"*

- category: `release_published`
- owner: `rust-lang`, repo: `rust`, tag_predicate: `1.80`
- source: `github_releases`

### RSS event

Post: *"Will the Fed cut rates this month?"*

- category: `event_occurrence`
- feed_url: Federal Reserve press RSS
- match_predicate: `rate cut`
- source: `rss_event`

## Audit

- Raw LLM output: `llm_extractions.raw_response`
- Canonical fields: `canonical_claims.normalized_fields`
- Compiled spec: `resolver_definitions` (immutable)
- Fingerprint: `resolver_definitions.compile_fingerprint`
