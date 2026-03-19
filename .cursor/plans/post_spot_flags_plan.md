# Post-Level SPoT Flags Implementation Plan

## Overview

Add `enable_spot` and `spot_id` to the GraphQL `Post` type. These columns exist in the `posts` table (per migration `20251230000000_update_post_opt_in_flags`) but are not currently selected by the reader or exposed in GraphQL.

## Schema Reference

From [`crates/myso-indexer-alt-social-schema/src/schema.rs`](crates/myso-indexer-alt-social-schema/src/schema.rs):
- `enable_spot` → `Bool` (NOT NULL, default false)
- `spot_id` → `Nullable<Text>` (address of SpotRecord object when SPoT record is created)

---

## Phase 1: Reader Layer (myso-indexer-alt-social-reader)

### 1.1 Extend PostRow in [`crates/myso-indexer-alt-social-reader/src/post.rs`](crates/myso-indexer-alt-social-reader/src/post.rs)

Add two fields to the `PostRow` struct (after `poc_analyzed_at`):

```rust
#[diesel(sql_type = diesel::sql_types::Bool)]
pub enable_spot: bool,
#[diesel(sql_type = Nullable<Text>)]
pub spot_id: Option<String>,
```

### 1.2 Update get_post_by_id SELECT

In the same file, extend the `get_post_by_id` query SELECT list to include:

```
enable_spot, spot_id
```

Current SELECT ends with `poc_oracle_address, poc_analyzed_at`. Append `, enable_spot, spot_id`.

### 1.3 Update list_posts SELECT

In the same file, extend the `list_posts` query SELECT list to include:

```
enable_spot, spot_id
```

Same pattern: append `, enable_spot, spot_id` to the existing column list.

---

## Phase 2: GraphQL Post Type (myso-indexer-alt-graphql)

### 2.1 Add fields to Post in [`crates/myso-indexer-alt-graphql/src/api/types/post.rs`](crates/myso-indexer-alt-graphql/src/api/types/post.rs)

Add two new `#[Object]` methods to the `Post` impl (place near other opt-in/linked-ID fields like `poc_id`, `revenue_redirect_to`):

```rust
/// Whether SPoT (Social Proof of Truth) prediction markets are enabled for this post.
async fn enable_spot(&self) -> bool {
    self.inner.enable_spot
}

/// Address of the SpotRecord object (set when a SPoT record is created). Null if no record.
async fn spot_id(&self) -> Option<&str> {
    self.inner.spot_id.as_deref()
}
```

---

## Phase 3: Snapshot Updates

### 3.1 Schema SDL Snapshot

Run:
```bash
INSTA_UPDATE=always cargo test -p myso-indexer-alt-graphql test_schema_sdl_export
```

This will update `crates/myso-indexer-alt-graphql/src/snapshots/myso_indexer_alt_graphql__tests__schema.graphql.snap` and `schema.graphql`.

### 3.2 Field Pipelines Snapshot

Run:
```bash
INSTA_UPDATE=always cargo test -p myso-indexer-alt-graphql test_registry_collect_pipelines_snapshot
```

This will update `crates/myso-indexer-alt-graphql/src/api/types/snapshots/myso_indexer_alt_graphql__api__types__available_range__field_piplines_tests__registry_collect_pipelines_snapshot.snap`.

---

## File Change Summary

| Crate | File | Changes |
|-------|------|---------|
| myso-indexer-alt-social-reader | `src/post.rs` | Add `enable_spot`, `spot_id` to PostRow; add to get_post_by_id and list_posts SELECT |
| myso-indexer-alt-graphql | `src/api/types/post.rs` | Add `enable_spot` and `spot_id` resolvers to Post |
| myso-indexer-alt-graphql | `src/snapshots/...schema.graphql.snap` | Auto-updated via test |
| myso-indexer-alt-graphql | `src/api/types/snapshots/...registry_collect_pipelines...snap` | Auto-updated via test |

---

## Validation

1. `cargo check -p myso-indexer-alt-social-reader -p myso-indexer-alt-graphql`
2. `MYSO_SKIP_SIMTESTS=1 cargo nextest run -p myso-indexer-alt-graphql --lib`

Do **not** run clippy per user request.
