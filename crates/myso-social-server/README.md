# Social Server

The Social Server is a Rust application that provides a RESTful API for the MySo social platform. It serves profile, platform, post, social graph, and related data from the social indexer database.

**Default port:** 9009 (configurable via `--server-port` or `SERVER_PORT` env var)

---

## Health & System

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/health` | Health check; returns HTTP 200 OK if the server is running |
| GET | `/stats/system` | System statistics |

```bash
curl http://localhost:9009/health
```

---

## Platforms

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/platforms` | List all platforms. Query: `approved` (optional), `governance` (optional), `limit`, `offset`, `page` |
| GET | `/platforms/approved` | List approved platforms only. Query: `limit`, `offset`, `page` |
| GET | `/platforms/:id` | Get platform by ID |
| GET | `/platforms/:id/moderators` | Get platform moderators |
| GET | `/platforms/:id/approval` | Get platform approval info |
| GET | `/platforms/:id/blocked` | Get platform blocklist |
| GET | `/platforms/:id/members` | Get platform members |
| GET | `/platforms/:id/membership/:profile_address` | Check if profile is a member |
| GET | `/platforms/:id/events` | Get platform events |

---

## Profiles

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/profiles` | List latest profiles |
| GET | `/profiles/daily-stats` | Daily profile event aggregates for charts (Timescale `profile_daily_stats`). Query: `bucket` (optional: `7d`, `30d`, `90d`, `180d`, `1y`; default `30d`) |
| GET | `/profiles/address/:address` | Get profile by wallet address |
| GET | `/profiles/username/:username` | Get profile by username |
| GET | `/profiles/username/:username/availability` | Check username availability |
| GET | `/profiles/:address/posts` | Get profile posts |
| GET | `/profiles/:address/events` | Get profile events |
| GET | `/profiles/:address/platform-memberships` | Get profile platform memberships |
| GET | `/profiles/:address/platforms` | Get profile platform events |
| GET | `/profiles/:address/blocking-history` | Get profile blocking history |
| GET | `/profiles/:address/badges` | Get profile badges |
| GET | `/profiles/:address/following` | Get profiles this address follows |
| GET | `/profiles/:address/followers` | Get followers of this address |
| GET | `/profiles/:address/recommendations` | Follow suggestions for the browsing user: subject's friends-of-friends filtered/ranked by `viewer_id`'s network (excludes viewer's and subject's existing follows and blocks). Omit `viewer_id` to treat the profile as the viewer. Query: `limit`, `offset`, `page`, `viewer_id`, `mutual_connections_limit` (optional, default 3, max 10 — profiles returned in `mutual_connections` for avatar stacks; does not cap `mutual_count`) |
| GET | `/profiles/:address/social-stats` | Get profile social stats |
| GET | `/profiles/:address/blocked` | Get blocked profiles |
| GET | `/profiles/:address/blocked-platforms` | Get blocked platforms |
| GET | `/profiles/:owner/subscription-services` | List profile subscription services |
| GET | `/profiles/:address/subscription-services` | (via subscription-services) |

---

## Social Graph & Blocklist

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/social-graph/check/:follower/:following` | Check if follower follows following |
| GET | `/social-graph/chart-data` | Get social graph chart data (Timescale `social_graph_daily_stats`). Query: `bucket` (optional, same as `/profiles/daily-stats`) |
| GET | `/blocklist/check/profile/:blocker/:blocked` | Check if `blocker` blocked `blocked` (directional) |
| GET | `/blocklist/check/either/:a/:b` | Check if **either** wallet blocked the other (bidirectional). Path order does not matter. Used by the messaging relayer for DM send gating. |
| GET | `/blocklist/check/platform/:profile/:platform` | Check if platform is blocked for profile |

### Blocklist response formats

**Directional** (`/blocklist/check/profile/:blocker/:blocked`):

```json
{ "is_blocked": true }
```

**Bidirectional** (`/blocklist/check/either/:a/:b`):

```json
{ "blocked": true }
```

Returns `blocked: true` when `a` blocked `b`, `b` blocked `a`, or both. Mirrors on-chain `either_blocked(a, b)` in the MySo social framework.

```bash
# Either direction blocked
curl http://localhost:9009/blocklist/check/either/0xabc.../0xdef...

# Directional check (blocker must be first)
curl http://localhost:9009/blocklist/check/profile/0xabc.../0xdef...
```

### Charts and Timescale continuous aggregates

Endpoints `/profiles/daily-stats` and `/social-graph/chart-data` read from Timescale **continuous aggregates** backed by `profile_events` and `social_graph_events`. Those tables are populated by **myso-indexer-alt-social** against the same PostgreSQL database this server uses.

- After a fresh database or migration, aggregates may be empty until the scheduled refresh policy runs (typically hourly). To backfill immediately in `psql`, you can run:  
  `CALL refresh_continuous_aggregate('profile_daily_stats', NULL, NULL);`  
  `CALL refresh_continuous_aggregate('social_graph_daily_stats', NULL, NULL);`  
  (On large datasets, prefer a bounded time window instead of `NULL` bounds; decompress compressed chunks first if refresh errors.)
- If charts stay flat, confirm the indexer is running and writing rows, then check raw counts: `SELECT count(*) FROM profile_events;` and `SELECT count(*) FROM social_graph_events;`.

---

## Badges

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/badges` | List badges |
| GET | `/badges/:badge_id` | Get badge by ID |

---

## Posts

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/posts` | List posts |
| GET | `/posts/configuration` | Get post configuration |
| GET | `/posts/trending` | Get trending posts |
| GET | `/posts/:id` | Get post by ID |
| GET | `/posts/:id/comments` | Get post comments |
| GET | `/posts/:id/reactions` | Get post reactions |
| GET | `/posts/:id/reposts` | Get post reposts |
| GET | `/posts/:id/promotion` | Get post promotion |
| GET | `/posts/:id/poc-badges` | Get post POC badges |
| GET | `/posts/:id/revenue-redirections` | Get post revenue redirections |
| GET | `/posts/:id/transfers` | Get post ownership transfers |
| GET | `/posts/:id/reports` | Get user reports against this post |
| GET | `/posts/:id/moderation-events` | Platform moderation history for this post (newest first) |
| GET | `/posts/:id/deletion-events` | Deletion history for this post id in `posts_deletion_events` (newest first) |

After `moderate_post` / `moderate_comment` and indexer catch-up, you can confirm rows in `posts_moderation_events` (filter `object_id` = post or comment id), the GraphQL `post { moderationEvents { eventId objectId platformId removed moderatedBy moderatedAt time transactionId } }` field, and the moderation REST route.

After `delete_post` / `delete_comment` and indexer catch-up, confirm rows in `posts_deletion_events` (`object_id` = post or comment id), GraphQL `post { deletionEvents { … } }` and `… comments { … deletionEvents { eventId objectId isPost postType postId deletedAt time transactionId } }`, and `GET /posts/:id/deletion-events`.

---

## Promotions

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/promotions` | List promotions |
| GET | `/promotions/analytics/top-performing` | Top performing promotions |
| GET | `/promotions/analytics/spending-trends` | Spending trends |
| GET | `/promotions/:id/views` | Get promotion views |
| GET | `/promotions/:id/stats` | Get promotion stats |
| GET | `/promotions/:id/analytics/time-series` | Promotion time series |
| GET | `/promotions/:id/analytics/hourly` | Promotion hourly analytics |

---

## POC (Proof of Contribution)

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/poc/badges` | List POC badges |
| GET | `/poc/badges/:id` | Get POC badge by ID |
| GET | `/poc/revenue-redirections` | List POC revenue redirections |
| GET | `/poc/analysis-results` | List POC analysis results |
| GET | `/poc/disputes` | List POC disputes |
| GET | `/poc/disputes/:id` | Get POC dispute by ID |
| GET | `/poc/disputes/:id/votes` | Get POC dispute votes |
| GET | `/poc/analytics` | Get POC analytics |
| GET | `/poc/configuration` | Get POC configuration |

---

## Subscriptions

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/subscriptions` | List subscriptions |
| GET | `/subscriptions/:id` | Get subscription by ID |
| GET | `/subscriptions/:id/status` | Get subscription status |
| GET | `/subscriptions/subscriber/:address` | List subscriptions by subscriber |
| GET | `/subscription-services` | List subscription services |
| GET | `/subscription-services/:service_id` | Get subscription service |
| GET | `/subscription-services/:service_id/revenue` | Get subscription revenue by service |
| GET | `/subscription-revenue` | List subscription revenue |
| GET | `/subscribers/:address/summary` | Get subscriber summary |
| GET | `/subscription-access/:subscriber/:service_id` | Check subscription access |
| GET | `/subscription-analytics` | Get subscription analytics |
| GET | `/service-performance` | Get service performance |

---

## Vesting

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/vesting/wallets` | List vesting wallets |
| GET | `/vesting/wallets/active` | List active vesting wallets |
| GET | `/vesting/wallets/:wallet_id` | Get vesting wallet |
| GET | `/vesting/wallets/:wallet_id/events` | Get vesting wallet events |
| GET | `/vesting/wallets/:wallet_id/claimable` | Get vesting claimable amount |
| GET | `/vesting/users/:address/wallets` | Get user vesting wallets |
| GET | `/vesting/events` | List vesting events |
| GET | `/vesting/analytics` | Get vesting analytics |
| GET | `/vesting/leaderboard` | Get vesting leaderboard |

---

## Revenue

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/revenue/dashboard` | Revenue dashboard |
| GET | `/revenue/leaderboard` | Revenue leaderboard |
| GET | `/revenue/chart-data` | Revenue chart data |
| GET | `/revenue/unified` | Unified revenue |
| GET | `/revenue/creators/:address/stats` | Creator revenue stats |
| GET | `/revenue/platforms/:address/stats` | Platform revenue stats |

---

## Treasury

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/treasury/current` | Current treasury state |
| GET | `/treasury/history` | Treasury history |

---

## Search

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/search` | Search (profiles, posts, etc.) |

---

## MyData

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/mydata` | List MyData items |
| GET | `/mydata/configuration` | Get MyData configuration |
| GET | `/mydata/popular` | Get popular MyData |
| GET | `/mydata/:id` | Get MyData by ID |
| GET | `/mydata/:id/purchases` | Get MyData purchases |
| GET | `/mydata/:id/subscriptions` | Get MyData subscriptions |
| GET | `/mydata/:id/revenue` | Get MyData revenue |
| GET | `/mydata/:id/access-logs` | Get MyData access logs |
| GET | `/mydata/:id/stats` | Get MyData stats |
| GET | `/mydata/:id/revenue-timeline` | Get MyData revenue timeline |
| GET | `/mydata/:id/access-analytics` | Get MyData access analytics |
| GET | `/creators/:id/mydata` | Get creator MyData |

---

## Insurance

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/insurance/configuration` | Get insurance configuration |
| GET | `/insurance/vaults` | List insurance vaults |
| GET | `/insurance/vaults/:vault_id` | Get insurance vault |
| GET | `/insurance/vaults/:vault_id/transactions` | List vault transactions |
| GET | `/insurance/vaults/:vault_id/exposures` | Get vault exposures |
| GET | `/insurance/policies` | List insurance policies |
| GET | `/insurance/policies/:policy_id` | Get insurance policy |
| GET | `/insurance/markets/:market_id/policies` | List market policies |

---

## Spot

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/spot/contested-records` | List contested spot records (`DAO_REQUIRED`) |
| GET | `/spot/records/:post_id` | Get spot record |
| GET | `/spot/records/:post_id/bets` | List spot bets |
| GET | `/spot/records/:post_id/payouts` | List spot payouts |
| GET | `/spot/records/:post_id/refunds` | List spot refunds |
| GET | `/spot/configuration` | Get spot configuration |

---

## SPT (Social Proof Token)

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/spt/pools` | List SPT pools |
| GET | `/spt/pools/:id` | Get SPT pool |
| GET | `/spt/pools/by-associated-id/:id` | Get SPT pool by associated ID |
| GET | `/spt/pools/:id/transactions` | Get pool transactions |
| GET | `/spt/pools/:id/holdings` | Get pool holdings |
| GET | `/spt/pools/:id/price-history` | Get pool price history |
| GET | `/spt/pools/:id/revenue` | Get pool revenue |
| GET | `/spt/pools/:id/liquidity-profile` | Get pool liquidity profile |
| GET | `/spt/popular` | Get popular SPT pools |
| GET | `/spt/users/:address/holdings` | Get user SPT holdings |
| GET | `/spt/users/:address/reservations` | Get user SPT reservations |
| GET | `/spt/analytics/top-performers` | Top performers analytics |
| GET | `/spt/portfolios/:address/performance` | Portfolio performance |
| GET | `/spt/creators/:address/revenue-streams` | Creator revenue streams |
| GET | `/spt/market-sentiment` | Market sentiment |
| GET | `/spt/configuration` | Get SPT configuration |
| GET | `/spt/reservation-pools` | List reservation pools |
| GET | `/spt/reservation-pools/:id` | Get reservation pool |
| GET | `/spt/reservation-pools/:id/reservations` | List pool reservations |

---

## Governance

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/governance/proposals` | List governance proposals |
| GET | `/governance/proposals/:id` | Get proposal |
| GET | `/governance/proposals/:id/community-votes` | Get community votes |
| GET | `/governance/proposals/:id/anonymous-stats` | Get anonymous stats |
| GET | `/governance/proposals/:id/anonymous-votes` | Get anonymous votes |
| GET | `/governance/proposals/:id/decryption-failures` | Get decryption failures |
| GET | `/governance/delegates` | List delegates |
| GET | `/governance/delegates/:address` | Get delegate |
| GET | `/governance/delegates/:address/proposals` | Get delegate proposals |
| GET | `/governance/delegates/:address/ratings` | Get delegate ratings |
| GET | `/governance/nominees` | List nominees |
| GET | `/governance/registries` | List registries |
| GET | `/governance/registries/:registry_type` | Get registry |
| GET | `/governance/events` | List governance events |
| GET | `/governance/anonymous-voting/trends` | Anonymous voting trends |

---

## Upgrade

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/upgrade/events` | List upgrade events |
| GET | `/upgrade/migrations` | List object migrated events |

---

## Response Formats

See [API_RESPONSE_FORMATS.md](./API_RESPONSE_FORMATS.md) for detailed JSON response formats for followers, following, platforms, and approved platforms.
