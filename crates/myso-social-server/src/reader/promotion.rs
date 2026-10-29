// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::sql_types::{BigInt, Bool, Double, Nullable, Text};
use diesel::QueryableByName;
use diesel_async::RunQueryDsl;

use crate::error::SocialError;
use crate::reader::types::{
    PaginationInfo, PromotedPostSummaryRow, PromotionAnalyticsQuery, PromotionBudgetEventRow,
    PromotionBudgetHistoryResponse, PromotionDetailRow, PromotionHourlyRow, PromotionListQuery,
    PromotionPlatformBreakdownResponse, PromotionPlatformBreakdownRow, PromotionReconciliation,
    PromotionRollupRow, PromotionStatsRow, PromotionStatusEventRow, PromotionStatusHistoryResponse,
    PromotionTimeSeriesRow, PromotionViewRow, PromotionWindowInfo,
};
use myso_pg_db::Db;

const MS_PER_DAY: i64 = 86_400_000;
/// Caps how far back a single analytics read may scan.
const MAX_WINDOW_DAYS: i64 = 366;
const DEFAULT_BUCKET: &str = "30d";
/// Hourly aggregates describe a short recent window, so they default narrower than daily series.
const DEFAULT_HOURLY_BUCKET: &str = "7d";

/// Resolved analytics window: epoch-ms bounds plus the bucket that produced them.
#[derive(Debug, Clone)]
struct PromotionWindow {
    start_ms: i64,
    end_ms: i64,
    bucket: String,
    days: i64,
}

impl PromotionWindow {
    fn info(&self) -> PromotionWindowInfo {
        PromotionWindowInfo {
            start_ms: self.start_ms,
            end_ms: self.end_ms,
            bucket: self.bucket.clone(),
            days: self.days,
        }
    }
}

fn bucket_to_days(bucket: &str) -> Result<i64, String> {
    match bucket {
        "7d" => Ok(7),
        "30d" => Ok(30),
        "90d" => Ok(90),
        "180d" => Ok(180),
        "1y" => Ok(365),
        _ => Err(format!(
            "Invalid bucket '{}'. Must be one of: 7d, 30d, 90d, 180d, 1y",
            bucket
        )),
    }
}

/// Resolves `bucket`/`from`/`to` into concrete bounds. Explicit `from`/`to` win over
/// `bucket`; otherwise the window ends now and reaches back `bucket` days.
fn resolve_window(
    query: &PromotionAnalyticsQuery,
    default_bucket: &str,
) -> Result<PromotionWindow, SocialError> {
    let bucket = query
        .bucket
        .as_deref()
        .map(str::trim)
        .filter(|b| !b.is_empty())
        .unwrap_or(default_bucket)
        .to_ascii_lowercase();
    let days = bucket_to_days(&bucket).map_err(SocialError::bad_request)?;
    let now_ms = chrono::Utc::now().timestamp_millis();
    let end_ms = query.to.unwrap_or(now_ms);
    let start_ms = query.from.unwrap_or(end_ms - days * MS_PER_DAY);
    validate_bounds(start_ms, end_ms)?;
    Ok(PromotionWindow {
        start_ms,
        end_ms,
        bucket,
        days: ((end_ms - start_ms) / MS_PER_DAY).max(1),
    })
}

/// Bounds for endpoints that read a full history by default and only narrow when asked.
fn explicit_bounds(query: &PromotionAnalyticsQuery) -> Result<Option<(i64, i64)>, SocialError> {
    if query.from.is_none() && query.to.is_none() {
        return Ok(None);
    }
    let now_ms = chrono::Utc::now().timestamp_millis();
    let end_ms = query.to.unwrap_or(now_ms);
    let start_ms = query.from.unwrap_or(0);
    validate_bounds(start_ms, end_ms)?;
    Ok(Some((start_ms, end_ms)))
}

fn validate_bounds(start_ms: i64, end_ms: i64) -> Result<(), SocialError> {
    if start_ms < 0 {
        return Err(SocialError::bad_request(
            "'from' must be a non-negative epoch millisecond value",
        ));
    }
    if end_ms <= start_ms {
        return Err(SocialError::bad_request(
            "'to' must be greater than 'from' (epoch milliseconds)",
        ));
    }
    if (end_ms - start_ms) / MS_PER_DAY > MAX_WINDOW_DAYS {
        return Err(SocialError::bad_request(format!(
            "Analytics window must not exceed {} days",
            MAX_WINDOW_DAYS
        )));
    }
    Ok(())
}

fn pagination(total: i64, limit: i64, offset: i64) -> PaginationInfo {
    let limit = limit.max(1);
    PaginationInfo {
        total,
        limit,
        offset,
        page: offset / limit + 1,
        total_pages: (total + limit - 1) / limit,
    }
}

/// A value to bind into a generated statement.
enum SqlBind {
    Text(String),
    BigInt(i64),
    Bool(bool),
}

/// Accumulates statement text and positional bind values.
///
/// diesel binds parameters in call order and Postgres numbers them `$1`, `$2`, ... so
/// placeholders must be emitted in the same order they are appended. Every value that
/// comes from a request is emitted as a placeholder; only internal literals and column
/// aliases are interpolated into the SQL text.
#[derive(Default)]
struct SqlBuilder {
    sql: String,
    binds: Vec<SqlBind>,
}

impl SqlBuilder {
    fn new() -> Self {
        Self::default()
    }

    fn push(&mut self, sql: &str) {
        self.sql.push_str(sql);
    }

    fn text(&mut self, value: &str) -> String {
        self.placeholder(SqlBind::Text(value.to_string()))
    }

    fn bigint(&mut self, value: i64) -> String {
        self.placeholder(SqlBind::BigInt(value))
    }

    fn boolean(&mut self, value: bool) -> String {
        self.placeholder(SqlBind::Bool(value))
    }

    fn placeholder(&mut self, bind: SqlBind) -> String {
        self.binds.push(bind);
        format!("${}", self.binds.len())
    }

    /// Builds the statement with every accumulated value bound as a parameter.
    ///
    /// `SqlQuery::bind` returns a new type per call, so the boxed form is required to
    /// bind a dynamic number of parameters in a loop.
    fn into_query(
        self,
    ) -> diesel::query_builder::BoxedSqlQuery<
        'static,
        diesel::pg::Pg,
        diesel::query_builder::SqlQuery,
    > {
        let mut query = diesel::sql_query(self.sql).into_boxed::<diesel::pg::Pg>();
        for bind in self.binds {
            query = match bind {
                SqlBind::Text(value) => query.bind::<Text, _>(value),
                SqlBind::BigInt(value) => query.bind::<BigInt, _>(value),
                SqlBind::Bool(value) => query.bind::<Bool, _>(value),
            };
        }
        query
    }
}

/// Emits the deduplicated campaign set (`WITH scope AS (...)`) that every campaign list,
/// count, and rollup filters against, so all of them select the same promotions.
///
/// Filters on immutable columns (`owner`, `profile_id`, `post_id`) and the platform
/// existence check are applied inside the deduplicating subquery so they can use the
/// `promoted_posts` and `promotion_views` indexes. The mutable `active` flag must be
/// filtered outside, against the newest row per promotion.
fn push_campaign_scope_cte(b: &mut SqlBuilder, query: &PromotionListQuery) {
    b.push(
        "WITH scope AS (
             SELECT DISTINCT ON (promotion_id)
                    promotion_id, post_id, owner, profile_id, payment_per_view,
                    total_budget, remaining_budget, active, created_at
             FROM promoted_posts pp0
             WHERE 1 = 1",
    );
    if let Some(owner) = query.owner_filter() {
        let placeholder = b.text(owner);
        b.push(&format!(" AND pp0.owner = {}", placeholder));
    }
    if let Some(profile_id) = query.profile_filter() {
        let placeholder = b.text(profile_id);
        b.push(&format!(" AND pp0.profile_id = {}", placeholder));
    }
    if let Some(post_id) = query.post_filter() {
        let placeholder = b.text(post_id);
        b.push(&format!(" AND pp0.post_id = {}", placeholder));
    }
    if let Some(platform_id) = query.platform_filter() {
        let placeholder = b.text(platform_id);
        b.push(&format!(
            " AND EXISTS (SELECT 1 FROM promotion_views pv0
                           WHERE pv0.promotion_id = pp0.promotion_id
                             AND pv0.platform_id = {})",
            placeholder
        ));
    }
    b.push(" ORDER BY promotion_id, time DESC )");
}

/// Appends the status filter against the already-deduplicated campaign set.
fn push_campaign_status_filter(
    b: &mut SqlBuilder,
    query: &PromotionListQuery,
) -> Result<(), SocialError> {
    if let Some(active) = query.status_filter().map_err(SocialError::bad_request)? {
        let placeholder = b.boolean(active);
        b.push(&format!(" AND pp.active = {}", placeholder));
    }
    Ok(())
}

/// Per-campaign confirmed-view aggregates, restricted to campaigns already in `scope`
/// so the reads still use the `promotion_views(promotion_id, time)` index.
const VIEW_AGGREGATE_JOIN: &str = "
         LEFT JOIN (
             SELECT promotion_id,
                    COUNT(*)::bigint AS views,
                    COUNT(DISTINCT viewer)::bigint AS unique_viewers,
                    COALESCE(SUM(payment_amount), 0)::bigint AS total_spent,
                    COALESCE(SUM(platform_fee), 0)::bigint AS platform_fee_total,
                    COALESCE(SUM(ecosystem_fee), 0)::bigint AS ecosystem_fee_total,
                    COALESCE(SUM(recipient_amount), 0)::bigint AS recipient_total,
                    MIN(timestamp)::bigint AS first_view_at,
                    MAX(timestamp)::bigint AS last_view_at
             FROM promotion_views
             WHERE promotion_id IN (SELECT promotion_id FROM scope)
             GROUP BY promotion_id
         ) agg ON agg.promotion_id = pp.promotion_id";

/// Per-campaign withdrawals, used to reconcile budget against spend.
const WITHDRAWAL_AGGREGATE_JOIN: &str = "
         LEFT JOIN (
             SELECT promotion_id, COALESCE(SUM(amount), 0)::bigint AS total_withdrawn
             FROM promotion_budget_events
             WHERE event_type = 'withdrawal'
               AND promotion_id IN (SELECT promotion_id FROM scope)
             GROUP BY promotion_id
         ) withdrawn ON withdrawn.promotion_id = pp.promotion_id";

const SUMMARY_COLUMNS: &str = "pp.promotion_id, pp.post_id, pp.owner, pp.profile_id,
                pp.payment_per_view, pp.total_budget, pp.remaining_budget, pp.active,
                pp.created_at,
                COALESCE(agg.views, 0)::bigint AS views,
                COALESCE(agg.unique_viewers, 0)::bigint AS unique_viewers,
                COALESCE(agg.total_spent, 0)::bigint AS total_spent,
                COALESCE(withdrawn.total_withdrawn, 0)::bigint AS total_withdrawn,
                COALESCE(agg.platform_fee_total, 0)::bigint AS platform_fee_total,
                COALESCE(agg.ecosystem_fee_total, 0)::bigint AS ecosystem_fee_total,
                COALESCE(agg.recipient_total, 0)::bigint AS recipient_total,
                agg.first_view_at, agg.last_view_at";

#[derive(QueryableByName)]
struct SummaryRow {
    #[diesel(sql_type = Text)]
    promotion_id: String,
    #[diesel(sql_type = Text)]
    post_id: String,
    #[diesel(sql_type = Text)]
    owner: String,
    #[diesel(sql_type = Text)]
    profile_id: String,
    #[diesel(sql_type = BigInt)]
    payment_per_view: i64,
    #[diesel(sql_type = BigInt)]
    total_budget: i64,
    #[diesel(sql_type = BigInt)]
    remaining_budget: i64,
    #[diesel(sql_type = Bool)]
    active: bool,
    #[diesel(sql_type = BigInt)]
    created_at: i64,
    #[diesel(sql_type = BigInt)]
    views: i64,
    #[diesel(sql_type = BigInt)]
    unique_viewers: i64,
    #[diesel(sql_type = BigInt)]
    total_spent: i64,
    #[diesel(sql_type = BigInt)]
    total_withdrawn: i64,
    #[diesel(sql_type = BigInt)]
    platform_fee_total: i64,
    #[diesel(sql_type = BigInt)]
    ecosystem_fee_total: i64,
    #[diesel(sql_type = BigInt)]
    recipient_total: i64,
    #[diesel(sql_type = Nullable<BigInt>)]
    first_view_at: Option<i64>,
    #[diesel(sql_type = Nullable<BigInt>)]
    last_view_at: Option<i64>,
}

impl From<SummaryRow> for PromotedPostSummaryRow {
    fn from(row: SummaryRow) -> Self {
        Self {
            promotion_id: row.promotion_id,
            post_id: row.post_id,
            owner: row.owner,
            profile_id: row.profile_id,
            payment_per_view: row.payment_per_view,
            total_budget: row.total_budget,
            remaining_budget: row.remaining_budget,
            active: row.active,
            created_at: row.created_at,
            views: row.views,
            unique_viewers: row.unique_viewers,
            total_spent: row.total_spent,
            total_withdrawn: row.total_withdrawn,
            platform_fee_total: row.platform_fee_total,
            ecosystem_fee_total: row.ecosystem_fee_total,
            recipient_total: row.recipient_total,
            first_view_at: row.first_view_at,
            last_view_at: row.last_view_at,
        }
    }
}

/// Campaigns matching `query`, newest first, each with aggregated views and spend.
///
/// `platform_id` narrows the campaign set to promotions with confirmed views on that
/// platform; the returned aggregates stay campaign-wide. Use
/// [`get_promotion_platform_breakdown`] for platform-scoped numbers.
pub(crate) async fn list_campaigns(
    db: &Db,
    query: &PromotionListQuery,
) -> Result<Vec<PromotedPostSummaryRow>, SocialError> {
    let mut b = SqlBuilder::new();
    push_campaign_scope_cte(&mut b, query);
    b.push(" SELECT ");
    b.push(SUMMARY_COLUMNS);
    b.push(" FROM scope pp");
    b.push(VIEW_AGGREGATE_JOIN);
    b.push(WITHDRAWAL_AGGREGATE_JOIN);
    b.push(" WHERE 1 = 1");
    push_campaign_status_filter(&mut b, query)?;
    let limit = b.bigint(query.limit());
    let offset = b.bigint(query.offset());
    b.push(&format!(
        " ORDER BY pp.created_at DESC, pp.promotion_id ASC LIMIT {} OFFSET {}",
        limit, offset
    ));

    let mut conn = db.connect().await?;
    let rows = b.into_query().load::<SummaryRow>(&mut conn).await?;
    Ok(rows.into_iter().map(PromotedPostSummaryRow::from).collect())
}

/// Advertiser-level rollup over the same filtered campaign set as [`list_campaigns`].
///
/// `campaign_count` doubles as the pagination total for the campaign list, and
/// `unique_viewers` sums per-campaign distinct viewers rather than counting distinct
/// viewers across campaigns.
pub(crate) async fn get_promotion_rollup(
    db: &Db,
    query: &PromotionListQuery,
) -> Result<PromotionRollupRow, SocialError> {
    #[derive(QueryableByName)]
    struct Row {
        #[diesel(sql_type = BigInt)]
        campaign_count: i64,
        #[diesel(sql_type = BigInt)]
        active_campaigns: i64,
        #[diesel(sql_type = BigInt)]
        total_budget: i64,
        #[diesel(sql_type = BigInt)]
        total_spent: i64,
        #[diesel(sql_type = BigInt)]
        total_withdrawn: i64,
        #[diesel(sql_type = BigInt)]
        total_remaining_budget: i64,
        #[diesel(sql_type = BigInt)]
        total_views: i64,
        #[diesel(sql_type = BigInt)]
        unique_viewers: i64,
        #[diesel(sql_type = BigInt)]
        platform_fee_total: i64,
        #[diesel(sql_type = BigInt)]
        ecosystem_fee_total: i64,
        #[diesel(sql_type = BigInt)]
        recipient_total: i64,
        #[diesel(sql_type = Nullable<BigInt>)]
        first_view_at: Option<i64>,
        #[diesel(sql_type = Nullable<BigInt>)]
        last_view_at: Option<i64>,
    }

    let mut b = SqlBuilder::new();
    push_campaign_scope_cte(&mut b, query);
    b.push(
        " SELECT COUNT(*)::bigint AS campaign_count,
                 COUNT(*) FILTER (WHERE pp.active)::bigint AS active_campaigns,
                 COALESCE(SUM(pp.total_budget), 0)::bigint AS total_budget,
                 COALESCE(SUM(pp.remaining_budget), 0)::bigint AS total_remaining_budget,
                 COALESCE(SUM(agg.views), 0)::bigint AS total_views,
                 COALESCE(SUM(agg.unique_viewers), 0)::bigint AS unique_viewers,
                 COALESCE(SUM(agg.total_spent), 0)::bigint AS total_spent,
                 COALESCE(SUM(agg.platform_fee_total), 0)::bigint AS platform_fee_total,
                 COALESCE(SUM(agg.ecosystem_fee_total), 0)::bigint AS ecosystem_fee_total,
                 COALESCE(SUM(agg.recipient_total), 0)::bigint AS recipient_total,
                 COALESCE(SUM(withdrawn.total_withdrawn), 0)::bigint AS total_withdrawn,
                 MIN(agg.first_view_at) AS first_view_at,
                 MAX(agg.last_view_at) AS last_view_at
          FROM scope pp",
    );
    b.push(VIEW_AGGREGATE_JOIN);
    b.push(WITHDRAWAL_AGGREGATE_JOIN);
    b.push(" WHERE 1 = 1");
    push_campaign_status_filter(&mut b, query)?;

    let mut conn = db.connect().await?;
    let rows = b.into_query().load::<Row>(&mut conn).await?;
    let row = rows
        .into_iter()
        .next()
        .ok_or_else(|| SocialError::internal("promotion rollup aggregate returned no row"))?;
    Ok(PromotionRollupRow {
        campaign_count: row.campaign_count,
        active_campaigns: row.active_campaigns,
        total_budget: row.total_budget,
        total_spent: row.total_spent,
        total_withdrawn: row.total_withdrawn,
        total_remaining_budget: row.total_remaining_budget,
        total_views: row.total_views,
        unique_viewers: row.unique_viewers,
        platform_fee_total: row.platform_fee_total,
        ecosystem_fee_total: row.ecosystem_fee_total,
        recipient_total: row.recipient_total,
        first_view_at: row.first_view_at,
        last_view_at: row.last_view_at,
    })
}

/// Campaigns with the most confirmed views, carrying the same spend aggregates as the list.
pub(crate) async fn get_top_performing_promotions(
    db: &Db,
    limit: i64,
) -> Result<Vec<PromotedPostSummaryRow>, SocialError> {
    let query = PromotionListQuery {
        limit: Some(limit),
        ..PromotionListQuery::default()
    };

    let mut b = SqlBuilder::new();
    push_campaign_scope_cte(&mut b, &query);
    b.push(" SELECT ");
    b.push(SUMMARY_COLUMNS);
    b.push(" FROM scope pp");
    b.push(VIEW_AGGREGATE_JOIN);
    b.push(WITHDRAWAL_AGGREGATE_JOIN);
    let limit = b.bigint(query.limit());
    b.push(&format!(
        " ORDER BY COALESCE(agg.views, 0) DESC, pp.created_at DESC, pp.promotion_id ASC LIMIT {}",
        limit
    ));

    let mut conn = db.connect().await?;
    let rows = b.into_query().load::<SummaryRow>(&mut conn).await?;
    Ok(rows.into_iter().map(PromotedPostSummaryRow::from).collect())
}

/// The campaign attached to `post_id`, with aggregated views and spend.
pub(crate) async fn get_promotion_by_post_id(
    db: &Db,
    post_id: &str,
) -> Result<Option<PromotedPostSummaryRow>, SocialError> {
    let query = PromotionListQuery {
        post_id: Some(post_id.to_string()),
        limit: Some(1),
        ..PromotionListQuery::default()
    };
    Ok(list_campaigns(db, &query).await?.into_iter().next())
}

/// Whether a campaign has been indexed, so callers answer 404 instead of returning
/// zeroed aggregates for an unknown id.
async fn promotion_exists(db: &Db, promotion_id: &str) -> Result<bool, SocialError> {
    #[derive(QueryableByName)]
    struct ExistsRow {
        #[diesel(sql_type = Bool)]
        present: bool,
    }

    let mut conn = db.connect().await?;
    let rows = diesel::sql_query(
        "SELECT EXISTS (SELECT 1 FROM promoted_posts WHERE promotion_id = $1) AS present",
    )
    .bind::<Text, _>(promotion_id)
    .load::<ExistsRow>(&mut conn)
    .await?;
    Ok(rows
        .into_iter()
        .next()
        .map(|row| row.present)
        .unwrap_or(false))
}

async fn require_promotion(db: &Db, promotion_id: &str) -> Result<(), SocialError> {
    if promotion_exists(db, promotion_id).await? {
        Ok(())
    } else {
        Err(SocialError::not_found(format!(
            "Promotion '{}'",
            promotion_id
        )))
    }
}

#[derive(QueryableByName)]
struct DetailSqlRow {
    #[diesel(sql_type = Text)]
    promotion_id: String,
    #[diesel(sql_type = Text)]
    post_id: String,
    #[diesel(sql_type = Text)]
    owner: String,
    #[diesel(sql_type = Text)]
    profile_id: String,
    #[diesel(sql_type = BigInt)]
    payment_per_view: i64,
    #[diesel(sql_type = BigInt)]
    total_budget: i64,
    #[diesel(sql_type = BigInt)]
    remaining_budget: i64,
    #[diesel(sql_type = Bool)]
    active: bool,
    #[diesel(sql_type = BigInt)]
    created_at: i64,
    #[diesel(sql_type = BigInt)]
    views: i64,
    #[diesel(sql_type = BigInt)]
    unique_viewers: i64,
    #[diesel(sql_type = BigInt)]
    total_spent: i64,
    #[diesel(sql_type = BigInt)]
    total_withdrawn: i64,
    #[diesel(sql_type = BigInt)]
    platform_fee_total: i64,
    #[diesel(sql_type = BigInt)]
    ecosystem_fee_total: i64,
    #[diesel(sql_type = BigInt)]
    recipient_total: i64,
    #[diesel(sql_type = Nullable<BigInt>)]
    first_view_at: Option<i64>,
    #[diesel(sql_type = Nullable<BigInt>)]
    last_view_at: Option<i64>,
    #[diesel(sql_type = Nullable<BigInt>)]
    ledger_remaining_budget: Option<i64>,
    #[diesel(sql_type = BigInt)]
    budget_events: i64,
    #[diesel(sql_type = Nullable<BigInt>)]
    last_budget_event_at: Option<i64>,
    #[diesel(sql_type = BigInt)]
    status_events: i64,
    #[diesel(sql_type = Nullable<BigInt>)]
    last_status_event_at: Option<i64>,
}

/// Campaign state, aggregated delivery, and the budget reconciliation check.
///
/// Every CTE references `$1` (the promotion id), so the id is bound once.
pub(crate) async fn get_promotion_detail(
    db: &Db,
    promotion_id: &str,
) -> Result<Option<PromotionDetailRow>, SocialError> {
    let mut conn = db.connect().await?;
    let rows = diesel::sql_query(
        "WITH latest AS (
             SELECT DISTINCT ON (promotion_id)
                    promotion_id, post_id, owner, profile_id, payment_per_view,
                    total_budget, remaining_budget, active, created_at
             FROM promoted_posts
             WHERE promotion_id = $1
             ORDER BY promotion_id, time DESC
         ),
         views AS (
             SELECT promotion_id,
                    COUNT(*)::bigint AS views,
                    COUNT(DISTINCT viewer)::bigint AS unique_viewers,
                    COALESCE(SUM(payment_amount), 0)::bigint AS total_spent,
                    COALESCE(SUM(platform_fee), 0)::bigint AS platform_fee_total,
                    COALESCE(SUM(ecosystem_fee), 0)::bigint AS ecosystem_fee_total,
                    COALESCE(SUM(recipient_amount), 0)::bigint AS recipient_total,
                    MIN(timestamp)::bigint AS first_view_at,
                    MAX(timestamp)::bigint AS last_view_at
             FROM promotion_views
             WHERE promotion_id = $1
             GROUP BY promotion_id
         ),
         ledger AS (
             SELECT promotion_id,
                    COUNT(*)::bigint AS budget_events,
                    (ARRAY_AGG(remaining_budget ORDER BY time DESC, id DESC))[1]::bigint
                        AS ledger_remaining_budget,
                    MAX(timestamp)::bigint AS last_budget_event_at
             FROM promotion_budget_events
             WHERE promotion_id = $1
             GROUP BY promotion_id
         ),
         withdrawals AS (
             SELECT promotion_id, COALESCE(SUM(amount), 0)::bigint AS total_withdrawn
             FROM promotion_budget_events
             WHERE promotion_id = $1 AND event_type = 'withdrawal'
             GROUP BY promotion_id
         ),
         statuses AS (
             SELECT promotion_id,
                    COUNT(*)::bigint AS status_events,
                    MAX(timestamp)::bigint AS last_status_event_at
             FROM promotion_status_events
             WHERE promotion_id = $1
             GROUP BY promotion_id
         )
         SELECT pp.promotion_id, pp.post_id, pp.owner, pp.profile_id, pp.payment_per_view,
                pp.total_budget, pp.remaining_budget, pp.active, pp.created_at,
                COALESCE(views.views, 0)::bigint AS views,
                COALESCE(views.unique_viewers, 0)::bigint AS unique_viewers,
                COALESCE(views.total_spent, 0)::bigint AS total_spent,
                COALESCE(withdrawals.total_withdrawn, 0)::bigint AS total_withdrawn,
                COALESCE(views.platform_fee_total, 0)::bigint AS platform_fee_total,
                COALESCE(views.ecosystem_fee_total, 0)::bigint AS ecosystem_fee_total,
                COALESCE(views.recipient_total, 0)::bigint AS recipient_total,
                views.first_view_at, views.last_view_at,
                ledger.ledger_remaining_budget,
                COALESCE(ledger.budget_events, 0)::bigint AS budget_events,
                ledger.last_budget_event_at,
                COALESCE(statuses.status_events, 0)::bigint AS status_events,
                statuses.last_status_event_at
         FROM latest pp
         LEFT JOIN views ON views.promotion_id = pp.promotion_id
         LEFT JOIN ledger ON ledger.promotion_id = pp.promotion_id
         LEFT JOIN withdrawals ON withdrawals.promotion_id = pp.promotion_id
         LEFT JOIN statuses ON statuses.promotion_id = pp.promotion_id",
    )
    .bind::<Text, _>(promotion_id)
    .load::<DetailSqlRow>(&mut conn)
    .await?;

    let Some(row) = rows.into_iter().next() else {
        return Ok(None);
    };

    let accounted_for = row.total_spent + row.total_withdrawn;
    let unaccounted = row.total_budget - accounted_for - row.remaining_budget;
    let ledger_agrees = row
        .ledger_remaining_budget
        .is_none_or(|ledger| ledger == row.remaining_budget);

    Ok(Some(PromotionDetailRow {
        promotion_id: row.promotion_id,
        post_id: row.post_id,
        owner: row.owner,
        profile_id: row.profile_id,
        payment_per_view: row.payment_per_view,
        total_budget: row.total_budget,
        remaining_budget: row.remaining_budget,
        active: row.active,
        created_at: row.created_at,
        views: row.views,
        unique_viewers: row.unique_viewers,
        total_spent: row.total_spent,
        total_withdrawn: row.total_withdrawn,
        platform_fee_total: row.platform_fee_total,
        ecosystem_fee_total: row.ecosystem_fee_total,
        recipient_total: row.recipient_total,
        first_view_at: row.first_view_at,
        last_view_at: row.last_view_at,
        last_budget_event_at: row.last_budget_event_at,
        last_status_event_at: row.last_status_event_at,
        reconciliation: PromotionReconciliation {
            total_budget: row.total_budget,
            total_spent: row.total_spent,
            total_withdrawn: row.total_withdrawn,
            remaining_budget: row.remaining_budget,
            accounted_for,
            unaccounted,
            ledger_remaining_budget: row.ledger_remaining_budget,
            budget_events: row.budget_events,
            status_events: row.status_events,
            balanced: unaccounted == 0 && ledger_agrees,
        },
    }))
}

#[derive(QueryableByName)]
struct BudgetEventSqlRow {
    #[diesel(sql_type = Text)]
    promotion_id: String,
    #[diesel(sql_type = Text)]
    post_id: String,
    #[diesel(sql_type = Text)]
    event_type: String,
    #[diesel(sql_type = BigInt)]
    amount: i64,
    #[diesel(sql_type = BigInt)]
    remaining_budget: i64,
    #[diesel(sql_type = BigInt)]
    timestamp: i64,
    #[diesel(sql_type = Text)]
    transaction_id: String,
}

/// Budget ledger for a campaign, newest first, with the reconciliation check.
///
/// Rows are ordered by the hypertable's `time` partition column (which mirrors
/// `timestamp`) so the read walks the `(promotion_id, time)` index instead of sorting.
pub(crate) async fn get_promotion_budget_history(
    db: &Db,
    promotion_id: &str,
    query: &PromotionAnalyticsQuery,
) -> Result<PromotionBudgetHistoryResponse, SocialError> {
    let detail = get_promotion_detail(db, promotion_id)
        .await?
        .ok_or_else(|| SocialError::not_found(format!("Promotion '{}'", promotion_id)))?;

    #[derive(QueryableByName)]
    struct CountRow {
        #[diesel(sql_type = BigInt)]
        cnt: i64,
    }

    let limit = query.limit();
    let offset = query.offset();
    let mut conn = db.connect().await?;

    let total = diesel::sql_query(
        "SELECT COUNT(*)::bigint AS cnt FROM promotion_budget_events WHERE promotion_id = $1",
    )
    .bind::<Text, _>(promotion_id)
    .get_result::<CountRow>(&mut conn)
    .await?
    .cnt;

    let rows = diesel::sql_query(
        "SELECT promotion_id, post_id, event_type, amount, remaining_budget, timestamp,
                transaction_id
         FROM promotion_budget_events
         WHERE promotion_id = $1
         ORDER BY time DESC, id DESC
         LIMIT $2 OFFSET $3",
    )
    .bind::<Text, _>(promotion_id)
    .bind::<BigInt, _>(limit)
    .bind::<BigInt, _>(offset)
    .load::<BudgetEventSqlRow>(&mut conn)
    .await?;

    Ok(PromotionBudgetHistoryResponse {
        promotion_id: promotion_id.to_string(),
        events: rows
            .into_iter()
            .map(|row| PromotionBudgetEventRow {
                promotion_id: row.promotion_id,
                post_id: row.post_id,
                event_type: row.event_type,
                amount: row.amount,
                remaining_budget: row.remaining_budget,
                timestamp: row.timestamp,
                transaction_id: row.transaction_id,
            })
            .collect(),
        reconciliation: detail.reconciliation,
        pagination: pagination(total, limit, offset),
    })
}

#[derive(QueryableByName)]
struct StatusEventSqlRow {
    #[diesel(sql_type = Text)]
    promotion_id: String,
    #[diesel(sql_type = Text)]
    post_id: String,
    #[diesel(sql_type = Text)]
    event_type: String,
    #[diesel(sql_type = Text)]
    triggered_by: String,
    #[diesel(sql_type = Nullable<Bool>)]
    new_status: Option<bool>,
    #[diesel(sql_type = Nullable<BigInt>)]
    amount: Option<i64>,
    #[diesel(sql_type = BigInt)]
    timestamp: i64,
    #[diesel(sql_type = Text)]
    transaction_id: String,
}

/// Status history for a campaign (activation toggles and withdrawal deactivations).
pub(crate) async fn get_promotion_status_history(
    db: &Db,
    promotion_id: &str,
    query: &PromotionAnalyticsQuery,
) -> Result<PromotionStatusHistoryResponse, SocialError> {
    require_promotion(db, promotion_id).await?;

    #[derive(QueryableByName)]
    struct CountRow {
        #[diesel(sql_type = BigInt)]
        cnt: i64,
    }

    let limit = query.limit();
    let offset = query.offset();
    let mut conn = db.connect().await?;

    let total = diesel::sql_query(
        "SELECT COUNT(*)::bigint AS cnt FROM promotion_status_events WHERE promotion_id = $1",
    )
    .bind::<Text, _>(promotion_id)
    .get_result::<CountRow>(&mut conn)
    .await?
    .cnt;

    let rows = diesel::sql_query(
        "SELECT promotion_id, post_id, event_type, triggered_by, new_status, amount,
                timestamp, transaction_id
         FROM promotion_status_events
         WHERE promotion_id = $1
         ORDER BY time DESC, id DESC
         LIMIT $2 OFFSET $3",
    )
    .bind::<Text, _>(promotion_id)
    .bind::<BigInt, _>(limit)
    .bind::<BigInt, _>(offset)
    .load::<StatusEventSqlRow>(&mut conn)
    .await?;

    Ok(PromotionStatusHistoryResponse {
        promotion_id: promotion_id.to_string(),
        events: rows
            .into_iter()
            .map(|row| PromotionStatusEventRow {
                promotion_id: row.promotion_id,
                post_id: row.post_id,
                event_type: row.event_type,
                triggered_by: row.triggered_by,
                new_status: row.new_status,
                amount: row.amount,
                timestamp: row.timestamp,
                transaction_id: row.transaction_id,
            })
            .collect(),
        pagination: pagination(total, limit, offset),
    })
}

#[derive(QueryableByName)]
struct PlatformBreakdownSqlRow {
    #[diesel(sql_type = Text)]
    platform_id: String,
    #[diesel(sql_type = BigInt)]
    views: i64,
    #[diesel(sql_type = BigInt)]
    unique_viewers: i64,
    #[diesel(sql_type = BigInt)]
    spent: i64,
    #[diesel(sql_type = BigInt)]
    platform_fees: i64,
    #[diesel(sql_type = BigInt)]
    ecosystem_fees: i64,
    #[diesel(sql_type = BigInt)]
    recipient_total: i64,
    #[diesel(sql_type = Double)]
    avg_view_duration: f64,
    #[diesel(sql_type = Nullable<BigInt>)]
    first_view_at: Option<i64>,
    #[diesel(sql_type = Nullable<BigInt>)]
    last_view_at: Option<i64>,
}

/// Confirmed views for a campaign split by the platform they were recorded on.
pub(crate) async fn get_promotion_platform_breakdown(
    db: &Db,
    promotion_id: &str,
    query: &PromotionAnalyticsQuery,
) -> Result<PromotionPlatformBreakdownResponse, SocialError> {
    require_promotion(db, promotion_id).await?;
    let window = resolve_window(query, DEFAULT_BUCKET)?;

    #[derive(QueryableByName)]
    struct CountRow {
        #[diesel(sql_type = BigInt)]
        cnt: i64,
    }

    let limit = query.limit();
    let offset = query.offset();
    let mut conn = db.connect().await?;

    let total = diesel::sql_query(
        "SELECT COUNT(DISTINCT platform_id)::bigint AS cnt
         FROM promotion_views
         WHERE promotion_id = $1 AND timestamp >= $2 AND timestamp < $3",
    )
    .bind::<Text, _>(promotion_id)
    .bind::<BigInt, _>(window.start_ms)
    .bind::<BigInt, _>(window.end_ms)
    .get_result::<CountRow>(&mut conn)
    .await?
    .cnt;

    let rows = diesel::sql_query(
        "SELECT platform_id,
                COUNT(*)::bigint AS views,
                COUNT(DISTINCT viewer)::bigint AS unique_viewers,
                COALESCE(SUM(payment_amount), 0)::bigint AS spent,
                COALESCE(SUM(platform_fee), 0)::bigint AS platform_fees,
                COALESCE(SUM(ecosystem_fee), 0)::bigint AS ecosystem_fees,
                COALESCE(SUM(recipient_amount), 0)::bigint AS recipient_total,
                COALESCE(AVG(view_duration), 0)::float8 AS avg_view_duration,
                MIN(timestamp)::bigint AS first_view_at,
                MAX(timestamp)::bigint AS last_view_at
         FROM promotion_views
         WHERE promotion_id = $1 AND timestamp >= $2 AND timestamp < $3
         GROUP BY platform_id
         ORDER BY spent DESC, platform_id ASC
         LIMIT $4 OFFSET $5",
    )
    .bind::<Text, _>(promotion_id)
    .bind::<BigInt, _>(window.start_ms)
    .bind::<BigInt, _>(window.end_ms)
    .bind::<BigInt, _>(limit)
    .bind::<BigInt, _>(offset)
    .load::<PlatformBreakdownSqlRow>(&mut conn)
    .await?;

    Ok(PromotionPlatformBreakdownResponse {
        promotion_id: promotion_id.to_string(),
        window: window.info(),
        platforms: rows
            .into_iter()
            .map(|row| PromotionPlatformBreakdownRow {
                platform_id: row.platform_id,
                views: row.views,
                unique_viewers: row.unique_viewers,
                spent: row.spent,
                platform_fees: row.platform_fees,
                ecosystem_fees: row.ecosystem_fees,
                recipient_total: row.recipient_total,
                avg_view_duration: row.avg_view_duration,
                first_view_at: row.first_view_at,
                last_view_at: row.last_view_at,
            })
            .collect(),
        pagination: pagination(total, limit, offset),
    })
}

/// Individual confirmed views for a campaign, newest first.
///
/// Bounds are opt-in: an unbounded request returns the full ledger, matching the
/// endpoint's original behaviour.
pub(crate) async fn get_promotion_views(
    db: &Db,
    promotion_id: &str,
    query: &PromotionAnalyticsQuery,
) -> Result<Vec<PromotionViewRow>, SocialError> {
    require_promotion(db, promotion_id).await?;
    let bounds = explicit_bounds(query)?;

    #[derive(QueryableByName)]
    struct Row {
        #[diesel(sql_type = Text)]
        post_id: String,
        #[diesel(sql_type = Text)]
        promotion_id: String,
        #[diesel(sql_type = Text)]
        viewer: String,
        #[diesel(sql_type = BigInt)]
        payment_amount: i64,
        #[diesel(sql_type = BigInt)]
        platform_fee: i64,
        #[diesel(sql_type = BigInt)]
        ecosystem_fee: i64,
        #[diesel(sql_type = BigInt)]
        recipient_amount: i64,
        #[diesel(sql_type = BigInt)]
        view_duration: i64,
        #[diesel(sql_type = Text)]
        platform_id: String,
        #[diesel(sql_type = BigInt)]
        timestamp: i64,
    }

    let mut b = SqlBuilder::new();
    b.push(
        "SELECT post_id, promotion_id, viewer, payment_amount, platform_fee, ecosystem_fee,
                recipient_amount, view_duration, platform_id, timestamp
         FROM promotion_views
         WHERE promotion_id = ",
    );
    let id = b.text(promotion_id);
    b.push(&id);
    if let Some((start_ms, end_ms)) = bounds {
        let start = b.bigint(start_ms);
        let end = b.bigint(end_ms);
        b.push(&format!(
            " AND timestamp >= {} AND timestamp < {}",
            start, end
        ));
    }
    if let Some(platform_id) = query.platform_filter() {
        let platform = b.text(platform_id);
        b.push(&format!(" AND platform_id = {}", platform));
    }
    let limit = b.bigint(query.limit());
    let offset = b.bigint(query.offset());
    b.push(&format!(
        " ORDER BY time DESC, id DESC LIMIT {} OFFSET {}",
        limit, offset
    ));

    let mut conn = db.connect().await?;
    let rows = b.into_query().load::<Row>(&mut conn).await?;
    Ok(rows
        .into_iter()
        .map(|row| PromotionViewRow {
            post_id: row.post_id,
            promotion_id: row.promotion_id,
            viewer: row.viewer,
            payment_amount: row.payment_amount,
            platform_fee: row.platform_fee,
            ecosystem_fee: row.ecosystem_fee,
            recipient_amount: row.recipient_amount,
            view_duration: row.view_duration,
            platform_id: row.platform_id,
            timestamp: row.timestamp,
        })
        .collect())
}

/// Aggregated delivery stats for a campaign.
///
/// Returns `None` when the campaign has not been indexed, so callers can answer 404
/// instead of reporting zeroed stats for an unknown id.
pub(crate) async fn get_promotion_stats(
    db: &Db,
    promotion_id: &str,
) -> Result<Option<PromotionStatsRow>, SocialError> {
    #[derive(QueryableByName)]
    struct BudgetRow {
        #[diesel(sql_type = BigInt)]
        remaining_budget: i64,
    }

    #[derive(QueryableByName)]
    struct ViewsRow {
        #[diesel(sql_type = BigInt)]
        total_views: i64,
        #[diesel(sql_type = BigInt)]
        unique_viewers: i64,
        #[diesel(sql_type = BigInt)]
        total_spent: i64,
    }

    let mut conn = db.connect().await?;
    let budgets = diesel::sql_query(
        "SELECT remaining_budget FROM (
             SELECT DISTINCT ON (promotion_id) remaining_budget
             FROM promoted_posts
             WHERE promotion_id = $1
             ORDER BY promotion_id, time DESC
         ) sub",
    )
    .bind::<Text, _>(promotion_id)
    .load::<BudgetRow>(&mut conn)
    .await?;

    let Some(budget) = budgets.into_iter().next() else {
        return Ok(None);
    };

    let views = diesel::sql_query(
        "SELECT COUNT(*)::bigint AS total_views,
                COUNT(DISTINCT viewer)::bigint AS unique_viewers,
                COALESCE(SUM(payment_amount), 0)::bigint AS total_spent
         FROM promotion_views
         WHERE promotion_id = $1",
    )
    .bind::<Text, _>(promotion_id)
    .get_result::<ViewsRow>(&mut conn)
    .await?;

    Ok(Some(PromotionStatsRow {
        total_views: views.total_views,
        unique_viewers: views.unique_viewers,
        total_spent: views.total_spent,
        remaining_budget: budget.remaining_budget,
    }))
}

/// Daily delivery series for a campaign over the resolved window.
pub(crate) async fn get_promotion_time_series(
    db: &Db,
    promotion_id: &str,
    query: &PromotionAnalyticsQuery,
) -> Result<Vec<PromotionTimeSeriesRow>, SocialError> {
    require_promotion(db, promotion_id).await?;
    let window = resolve_window(query, DEFAULT_BUCKET)?;
    let limit = query.series_limit();

    let mut conn = db.connect().await?;
    let rows = diesel::sql_query(
        "SELECT DATE(to_timestamp(timestamp / 1000)) AS day,
                COUNT(*)::bigint AS views,
                COUNT(DISTINCT viewer)::bigint AS unique_viewers,
                COALESCE(SUM(payment_amount), 0)::bigint AS spent
         FROM promotion_views
         WHERE promotion_id = $1 AND timestamp >= $2 AND timestamp < $3
         GROUP BY DATE(to_timestamp(timestamp / 1000))
         ORDER BY day ASC
         LIMIT $4",
    )
    .bind::<Text, _>(promotion_id)
    .bind::<BigInt, _>(window.start_ms)
    .bind::<BigInt, _>(window.end_ms)
    .bind::<BigInt, _>(limit)
    .load::<PromotionTimeSeriesRow>(&mut conn)
    .await?;
    Ok(rows)
}

/// Hour-of-day delivery profile for a campaign over the resolved window.
pub(crate) async fn get_promotion_hourly(
    db: &Db,
    promotion_id: &str,
    query: &PromotionAnalyticsQuery,
) -> Result<Vec<PromotionHourlyRow>, SocialError> {
    require_promotion(db, promotion_id).await?;
    let window = resolve_window(query, DEFAULT_HOURLY_BUCKET)?;
    let limit = query.series_limit();

    let mut conn = db.connect().await?;
    let rows = diesel::sql_query(
        "SELECT EXTRACT(HOUR FROM to_timestamp(timestamp / 1000))::int AS hour,
                COUNT(*)::bigint AS views,
                COUNT(DISTINCT viewer)::bigint AS unique_viewers,
                COALESCE(SUM(payment_amount), 0)::bigint AS spent
         FROM promotion_views
         WHERE promotion_id = $1 AND timestamp >= $2 AND timestamp < $3
         GROUP BY EXTRACT(HOUR FROM to_timestamp(timestamp / 1000))
         ORDER BY hour ASC
         LIMIT $4",
    )
    .bind::<Text, _>(promotion_id)
    .bind::<BigInt, _>(window.start_ms)
    .bind::<BigInt, _>(window.end_ms)
    .bind::<BigInt, _>(limit)
    .load::<PromotionHourlyRow>(&mut conn)
    .await?;
    Ok(rows)
}

/// Daily spending across every campaign in scope (optionally one advertiser).
pub(crate) async fn get_spending_trends(
    db: &Db,
    query: &PromotionAnalyticsQuery,
) -> Result<Vec<PromotionTimeSeriesRow>, SocialError> {
    let window = resolve_window(query, DEFAULT_BUCKET)?;
    let limit = query.series_limit();

    let mut b = SqlBuilder::new();
    b.push(
        "SELECT DATE(to_timestamp(timestamp / 1000)) AS day,
                COUNT(*)::bigint AS views,
                COUNT(DISTINCT viewer)::bigint AS unique_viewers,
                COALESCE(SUM(payment_amount), 0)::bigint AS spent
         FROM promotion_views pv
         WHERE pv.timestamp >= ",
    );
    let start = b.bigint(window.start_ms);
    b.push(&start);
    b.push(" AND pv.timestamp < ");
    let end = b.bigint(window.end_ms);
    b.push(&end);
    if let Some(platform_id) = query.platform_filter() {
        let platform = b.text(platform_id);
        b.push(&format!(" AND pv.platform_id = {}", platform));
    }
    if let Some(owner) = query.owner_filter() {
        let owner = b.text(owner);
        b.push(&format!(
            " AND pv.promotion_id IN (SELECT promotion_id FROM promoted_posts WHERE owner = {})",
            owner
        ));
    }
    let limit = b.bigint(limit);
    b.push(&format!(
        " GROUP BY DATE(to_timestamp(timestamp / 1000)) ORDER BY day ASC LIMIT {}",
        limit
    ));

    let mut conn = db.connect().await?;
    let rows = b
        .into_query()
        .load::<PromotionTimeSeriesRow>(&mut conn)
        .await?;
    Ok(rows)
}
