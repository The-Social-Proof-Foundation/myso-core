---
name: ""
overview: ""
todos: []
isProject: false
---

# Enrich SocialProofToken in GraphQL

## Current State

**SocialProofToken** (embedded in Profile/ProfileSummary) exposes only reservation-phase data:

- `isActive`, `poolId`, `tokenAddress`, `reservationPoolId`, `reservationPercentage`, `reservationStatus`, `totalReserved`, `requiredThreshold`

When `isActive` is true (token past reservation), we have `poolId` but no token metadata (symbol, name, price, supply).

**Data available but unused:**

- `spt_pools`: `symbol`, `name`, `circulating_supply`, `base_price`, `quadratic_coefficient`, `owner`, `created_at`
- `spt_price_history`: latest `price` per pool
- `SptPool` (standalone query) already exposes: symbol, name, price, totalSupply, owner, tokenType

**Enrichment flow:** `enrich_users_with_universal_data` in [profile.rs](crates/myso-indexer-alt-social-reader/src/profile.rs) joins `latest_spt_pools` and `latest_reservation_pools` but only selects `spt.pool_id`; it does not select symbol, name, circulating_supply, base_price, or current price.

---

## Implementation

### Phase 1: Extend Enrichment Query – Reader

**File:** [crates/myso-indexer-alt-social-reader/src/profile.rs](crates/myso-indexer-alt-social-reader/src/profile.rs)

1. **Add LATERAL join for latest price** in the enrichment query:

```sql
   LEFT JOIN LATERAL (
       SELECT price FROM spt_price_history
       WHERE pool_id = spt.pool_id
       ORDER BY time DESC
       LIMIT 1
   ) ph ON spt.pool_id IS NOT NULL
   

```

1. **Extend SELECT** to include from `spt` when pool exists:
  - `spt.symbol`, `spt.name`, `spt.circulating_supply`, `spt.base_price`, `spt.owner`, `spt.created_at`, `spt.token_type`
  - `ph.price AS current_price`
2. **Extend EnrichmentRow** with nullable fields: `spt_symbol`, `spt_name`, `spt_circulating_supply`, `spt_base_price`, `spt_current_price`, `spt_owner`, `spt_created_at`, `spt_token_type`.
3. **Extend SocialProofTokenInfo** with optional active-token fields:
  - `symbol: Option<String>`
  - `name: Option<String>`
  - `circulating_supply: Option<i64>`
  - `base_price: Option<i64>`
  - `current_price: Option<i64>`
  - `owner: Option<String>`
  - `created_at: Option<i64>`
  - `token_type: Option<i16>`
4. **Populate SocialProofTokenInfo** from enrichment row when `spt_pool_id` is present.

### Phase 2: GraphQL SocialProofToken Type

**File:** [crates/myso-indexer-alt-graphql/src/api/types/profile.rs](crates/myso-indexer-alt-graphql/src/api/types/profile.rs)

Add resolvers for the new fields (all optional, present when token is active):

- `symbol` – Token symbol (e.g. "BRAND")
- `name` – Token display name
- `circulatingSupply` – Total supply in circulation
- `basePrice` – Base price (smallest units)
- `currentPrice` – Latest price from spt_price_history
- `owner` – Pool owner address (MySoAddress)
- `createdAt` – Creation timestamp (ms)
- `tokenType` – 1=profile, 2=post

Place these after the existing reservation fields. Use `Option` return types; when token is in reservation phase only, these remain null.

### Phase 3: Schema and Snapshots

- Run `INSTA_UPDATE=1 cargo nextest run -p myso-indexer-alt-graphql -- test_schema_sdl_export` to regenerate schema.graphql and snapshots.

---

## Query Structure (Enrichment)

The enrichment query uses:

- `latest_profiles` – profiles by owner
- `latest_spt_pools` – spt_pools by associated_id (profile_X)
- `latest_reservation_pools` – spt_reservation_pools by associated_id

Add LATERAL join for price:

```sql
LEFT JOIN LATERAL (
    SELECT price FROM spt_price_history
    WHERE pool_id = spt.pool_id
    ORDER BY time DESC
    LIMIT 1
) ph ON spt.pool_id IS NOT NULL
```

Select: `spt.symbol, spt.name, spt.circulating_supply, spt.base_price, spt.owner, spt.created_at, spt.token_type, ph.price AS current_price`

---

## Verification

- `cargo check -p myso-indexer-alt-social-reader -p myso-indexer-alt-graphql`
- Query `profile(address: "0x...") { socialProofToken { symbol name currentPrice circulatingSupply } }` for a profile with an active SPT; confirm active-token fields are populated.
- For a profile in reservation phase only, confirm new fields are null.

