// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! GraphQL types for profile cash-flow P&L ([`super::profile::Profile::pnl`]).
//!
//! Amounts are **MYSO base units** (on-chain integer scale), same as storage in
//! `spt_transactions`, `spt_reservations`, and `unified_revenue`. This is wallet
//! **cash-flow**, not FIFO realized or mark-to-market P&L.

use async_graphql::{Enum, SimpleObject};

/// Time window for P&L aggregation (cutoff from database `NOW()`).
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
#[graphql(rename_items = "SCREAMING_SNAKE_CASE")]
pub(crate) enum ProfilePnLWindowGql {
    Days7,
    Days30,
    Days180,
    Days365,
    All,
}

impl From<ProfilePnLWindowGql> for myso_indexer_alt_social_reader::ProfilePnLWindow {
    fn from(w: ProfilePnLWindowGql) -> Self {
        match w {
            ProfilePnLWindowGql::Days7 => Self::Days7,
            ProfilePnLWindowGql::Days30 => Self::Days30,
            ProfilePnLWindowGql::Days180 => Self::Days180,
            ProfilePnLWindowGql::Days365 => Self::Days365,
            ProfilePnLWindowGql::All => Self::All,
        }
    }
}

/// Per-window cash-flow breakdown for the profile owner wallet.
#[derive(SimpleObject, Clone)]
pub(crate) struct ProfilePnLWindowStats {
    pub window: ProfilePnLWindowGql,
    pub swap_net_myso: i64,
    pub reservation_net_myso: i64,
    pub spt_creator_fees_myso: i64,
    pub tips_myso: i64,
    pub subscriptions_myso: i64,
    pub mydata_myso: i64,
    pub posts_monetization_myso: i64,
    pub messaging_myso: i64,
    pub other_inbound_myso: i64,
    pub gross_inbound_myso: i64,
    pub net_cash_flow_myso: i64,
}

impl From<myso_indexer_alt_social_reader::ProfilePnLWindowResult> for ProfilePnLWindowStats {
    fn from(r: myso_indexer_alt_social_reader::ProfilePnLWindowResult) -> Self {
        let window = match r.window {
            myso_indexer_alt_social_reader::ProfilePnLWindow::Days7 => ProfilePnLWindowGql::Days7,
            myso_indexer_alt_social_reader::ProfilePnLWindow::Days30 => ProfilePnLWindowGql::Days30,
            myso_indexer_alt_social_reader::ProfilePnLWindow::Days180 => {
                ProfilePnLWindowGql::Days180
            }
            myso_indexer_alt_social_reader::ProfilePnLWindow::Days365 => {
                ProfilePnLWindowGql::Days365
            }
            myso_indexer_alt_social_reader::ProfilePnLWindow::All => ProfilePnLWindowGql::All,
        };
        Self {
            window,
            swap_net_myso: r.swap_net_myso,
            reservation_net_myso: r.reservation_net_myso,
            spt_creator_fees_myso: r.spt_creator_fees_myso,
            tips_myso: r.tips_myso,
            subscriptions_myso: r.subscriptions_myso,
            mydata_myso: r.mydata_myso,
            posts_monetization_myso: r.posts_monetization_myso,
            messaging_myso: r.messaging_myso,
            other_inbound_myso: r.other_inbound_myso,
            gross_inbound_myso: r.gross_inbound_myso,
            net_cash_flow_myso: r.net_cash_flow_myso,
        }
    }
}
