// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Profile cash-flow P&L from indexed social tables.
//!
//! All amounts are **blockchain MYSO base units** (same as `BIGINT` columns in Postgres).
//!
//! # Semantics
//! - **Not** mark-to-market or FIFO realized P&L; this is **wallet cash-flow** over a time window.
//! - **Swaps** ([`spt_transactions`](myso_indexer_alt_social_schema::schema::spt_transactions)):
//!   net MYSO for the trader is `SUM(-myso_amount)` (buys store positive `myso_amount`, sells negative).
//! - **Reservations** ([`spt_reservations`](myso_indexer_alt_social_schema::schema::spt_reservations)):
//!   net MYSO for the reserver is `SUM(-amount)` (deposits positive, withdrawals negative).
//! - **Unified revenue** ([`unified_revenue`](myso_indexer_alt_social_schema::schema::unified_revenue)):
//!   inbound payouts use `recipient_address = profile owner`; rows are split into non-overlapping buckets
//!   by `revenue_source` / `revenue_type`.
//!
//! Time filtering uses each table's hypertable `time` column (`TIMESTAMPTZ`), consistent with other readers.

use diesel::QueryableByName;
use diesel::sql_types::BigInt;
use diesel_async::RunQueryDsl;
use myso_pg_db::Connection;
use serde::{Deserialize, Serialize};

/// Rolling window (or all time) for P&L aggregates. Cutoffs use **database** `NOW()`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProfilePnLWindow {
    Days7,
    Days30,
    Days180,
    Days365,
    All,
}

impl ProfilePnLWindow {
    /// Days for `time >= NOW() - N days`; `-1` means no lower bound (all time).
    pub fn days_parameter(self) -> i64 {
        match self {
            ProfilePnLWindow::Days7 => 7,
            ProfilePnLWindow::Days30 => 30,
            ProfilePnLWindow::Days180 => 180,
            ProfilePnLWindow::Days365 => 365,
            ProfilePnLWindow::All => -1,
        }
    }
}

/// One window's aggregate buckets plus derived totals.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProfilePnLWindowResult {
    pub window: ProfilePnLWindow,
    /// Net MYSO from SPT swaps for the profile owner (`SUM(-myso_amount)`).
    pub swap_net_myso: i64,
    /// Net MYSO from reservation deposits/withdrawals (`SUM(-amount)`).
    pub reservation_net_myso: i64,
    /// SPT creator fee share credited in unified revenue (`revenue_source = spt`, `revenue_type = creator_fee`).
    pub spt_creator_fees_myso: i64,
    /// Tips (`revenue_source = tips`).
    pub tips_myso: i64,
    /// Subscriptions (`revenue_source = subscription`).
    pub subscriptions_myso: i64,
    /// MyData / IP sales (`revenue_source` in `my_ip`, `mydata`).
    pub mydata_myso: i64,
    /// Post monetization (`revenue_source = posts` and monetization types).
    pub posts_monetization_myso: i64,
    /// Paid messaging inbound revenue (`revenue_source = messaging`).
    pub messaging_myso: i64,
    /// Unified revenue rows to this recipient not covered by the explicit buckets above.
    pub other_inbound_myso: i64,
    /// Sum of inbound unified-revenue buckets (creator fees + tips + subscriptions + mydata + posts + other).
    pub gross_inbound_myso: i64,
    /// `swap_net_myso + reservation_net_myso + gross_inbound_myso`.
    pub net_cash_flow_myso: i64,
}

#[derive(Debug, Clone, QueryableByName)]
struct ProfilePnLRawRow {
    #[diesel(sql_type = BigInt)]
    swap_net_myso: i64,
    #[diesel(sql_type = BigInt)]
    reservation_net_myso: i64,
    #[diesel(sql_type = BigInt)]
    spt_creator_fees_myso: i64,
    #[diesel(sql_type = BigInt)]
    tips_myso: i64,
    #[diesel(sql_type = BigInt)]
    subscriptions_myso: i64,
    #[diesel(sql_type = BigInt)]
    mydata_myso: i64,
    #[diesel(sql_type = BigInt)]
    posts_monetization_myso: i64,
    #[diesel(sql_type = BigInt)]
    messaging_myso: i64,
    #[diesel(sql_type = BigInt)]
    other_inbound_myso: i64,
}

/// Load P&L for each requested window (one SQL round-trip per window).
pub async fn get_profile_pnl_for_windows(
    conn: &mut Connection<'_>,
    owner_address: &str,
    windows: &[ProfilePnLWindow],
) -> anyhow::Result<Vec<ProfilePnLWindowResult>> {
    let mut out = Vec::with_capacity(windows.len());
    for &window in windows {
        let days = window.days_parameter();
        let row = profile_pnl_one_window(conn, owner_address, days).await?;
        let gross_inbound_myso = row
            .spt_creator_fees_myso
            .saturating_add(row.tips_myso)
            .saturating_add(row.subscriptions_myso)
            .saturating_add(row.mydata_myso)
            .saturating_add(row.posts_monetization_myso)
            .saturating_add(row.messaging_myso)
            .saturating_add(row.other_inbound_myso);
        let net_cash_flow_myso = row
            .swap_net_myso
            .saturating_add(row.reservation_net_myso)
            .saturating_add(gross_inbound_myso);
        out.push(ProfilePnLWindowResult {
            window,
            swap_net_myso: row.swap_net_myso,
            reservation_net_myso: row.reservation_net_myso,
            spt_creator_fees_myso: row.spt_creator_fees_myso,
            tips_myso: row.tips_myso,
            subscriptions_myso: row.subscriptions_myso,
            mydata_myso: row.mydata_myso,
            posts_monetization_myso: row.posts_monetization_myso,
            messaging_myso: row.messaging_myso,
            other_inbound_myso: row.other_inbound_myso,
            gross_inbound_myso,
            net_cash_flow_myso,
        });
    }
    Ok(out)
}

async fn profile_pnl_one_window(
    conn: &mut Connection<'_>,
    owner_address: &str,
    days: i64,
) -> anyhow::Result<ProfilePnLRawRow> {
    let query = r#"
        SELECT
            (SELECT COALESCE(SUM(-myso_amount), 0)::bigint FROM spt_transactions
             WHERE sender = $1
               AND ($2::bigint < 0 OR time >= (NOW() - ($2::bigint * INTERVAL '1 day')))) AS swap_net_myso,
            (SELECT COALESCE(SUM(-amount), 0)::bigint FROM spt_reservations
             WHERE reserver_address = $1
               AND ($2::bigint < 0 OR time >= (NOW() - ($2::bigint * INTERVAL '1 day')))) AS reservation_net_myso,
            (SELECT COALESCE(SUM(amount), 0)::bigint FROM unified_revenue
             WHERE recipient_address = $1
               AND ($2::bigint < 0 OR time >= (NOW() - ($2::bigint * INTERVAL '1 day')))
               AND revenue_source = 'spt' AND revenue_type = 'creator_fee') AS spt_creator_fees_myso,
            (SELECT COALESCE(SUM(amount), 0)::bigint FROM unified_revenue
             WHERE recipient_address = $1
               AND ($2::bigint < 0 OR time >= (NOW() - ($2::bigint * INTERVAL '1 day')))
               AND revenue_source = 'tips') AS tips_myso,
            (SELECT COALESCE(SUM(amount), 0)::bigint FROM unified_revenue
             WHERE recipient_address = $1
               AND ($2::bigint < 0 OR time >= (NOW() - ($2::bigint * INTERVAL '1 day')))
               AND revenue_source = 'subscription'
               AND revenue_type NOT IN ('platform_fee', 'ecosystem_fee')) AS subscriptions_myso,
            (SELECT COALESCE(SUM(amount), 0)::bigint FROM unified_revenue
             WHERE recipient_address = $1
               AND ($2::bigint < 0 OR time >= (NOW() - ($2::bigint * INTERVAL '1 day')))
               AND revenue_source IN ('my_ip', 'mydata')
               AND revenue_type NOT IN ('platform_fee', 'ecosystem_fee')) AS mydata_myso,
            (SELECT COALESCE(SUM(amount), 0)::bigint FROM unified_revenue
             WHERE recipient_address = $1
               AND ($2::bigint < 0 OR time >= (NOW() - ($2::bigint * INTERVAL '1 day')))
               AND revenue_source = 'posts'
               AND revenue_type IN ('post_monetization', 'premium_content')) AS posts_monetization_myso,
            (SELECT COALESCE(SUM(amount), 0)::bigint FROM unified_revenue
             WHERE recipient_address = $1
               AND ($2::bigint < 0 OR time >= (NOW() - ($2::bigint * INTERVAL '1 day')))
               AND revenue_source = 'messaging') AS messaging_myso,
            (SELECT COALESCE(SUM(amount), 0)::bigint FROM unified_revenue
             WHERE recipient_address = $1
               AND ($2::bigint < 0 OR time >= (NOW() - ($2::bigint * INTERVAL '1 day')))
               AND NOT (revenue_source = 'spt' AND revenue_type = 'creator_fee')
               AND NOT (revenue_source = 'tips')
               AND NOT (revenue_source = 'subscription')
               AND NOT (revenue_source IN ('my_ip', 'mydata'))
               AND NOT (revenue_source = 'posts' AND revenue_type IN ('post_monetization', 'premium_content'))
               AND NOT (revenue_source = 'messaging')
            ) AS other_inbound_myso
    "#;

    diesel::sql_query(query)
        .bind::<diesel::sql_types::Text, _>(owner_address)
        .bind::<BigInt, _>(days)
        .get_result::<ProfilePnLRawRow>(conn)
        .await
        .map_err(Into::into)
}

#[cfg(test)]
mod tests {
    use super::ProfilePnLWindow;

    #[test]
    fn window_days_parameter_all_time_negative_one() {
        assert_eq!(ProfilePnLWindow::All.days_parameter(), -1);
        assert_eq!(ProfilePnLWindow::Days7.days_parameter(), 7);
        assert_eq!(ProfilePnLWindow::Days365.days_parameter(), 365);
    }
}
