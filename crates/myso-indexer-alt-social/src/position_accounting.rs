// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Weighted-average cost (WAC) updates for personal SPT positions.
//!
//! Position accounting follows token ownership, not tax lots.
//! Realized ROI uses disposed cost basis (`realized / disposed_cost_basis`).

use myso_indexer_alt_social_schema::models::{
    UserSptPositionState, POSITION_EVENT_BUY, POSITION_EVENT_LAUNCH, POSITION_EVENT_RESERVATION,
    POSITION_EVENT_SELL, POSITION_EVENT_TRANSFER_IN, POSITION_EVENT_TRANSFER_OUT,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PositionEventKind {
    Buy,
    Sell,
    Reservation,
    Launch,
    TransferIn,
    TransferOut,
}

impl PositionEventKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Buy => POSITION_EVENT_BUY,
            Self::Sell => POSITION_EVENT_SELL,
            Self::Reservation => POSITION_EVENT_RESERVATION,
            Self::Launch => POSITION_EVENT_LAUNCH,
            Self::TransferIn => POSITION_EVENT_TRANSFER_IN,
            Self::TransferOut => POSITION_EVENT_TRANSFER_OUT,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PositionEvent {
    pub kind: PositionEventKind,
    /// Token qty (nano-SPT). Unused for reservation (MYSO-only).
    pub token_qty: i64,
    /// MYSO paid (buy/reservation deposit) or received (sell). Reservation withdraw is negative.
    pub myso_amount: i64,
    /// Inherited cost basis for transfer-in when known.
    pub inherited_cost_myso: Option<i64>,
}

pub fn apply_spt_position_event(state: &mut UserSptPositionState, event: &PositionEvent) {
    match event.kind {
        PositionEventKind::Buy => apply_buy(state, event.token_qty, event.myso_amount),
        PositionEventKind::Sell => apply_sell(state, event.token_qty, event.myso_amount),
        PositionEventKind::Reservation => apply_reservation(state, event.myso_amount),
        PositionEventKind::Launch => apply_launch(state, event.token_qty),
        PositionEventKind::TransferOut => apply_transfer_out(state, event.token_qty),
        PositionEventKind::TransferIn => {
            apply_transfer_in(state, event.token_qty, event.inherited_cost_myso)
        }
    }
}

fn apply_buy(state: &mut UserSptPositionState, token_qty: i64, myso_paid: i64) {
    if token_qty <= 0 || myso_paid <= 0 {
        return;
    }
    state.token_balance = state.token_balance.saturating_add(token_qty);
    state.cost_basis_myso = state.cost_basis_myso.saturating_add(myso_paid);
    state.total_invested_myso = state.total_invested_myso.saturating_add(myso_paid);
}

fn apply_sell(state: &mut UserSptPositionState, sold_qty: i64, proceeds: i64) {
    if sold_qty <= 0 {
        return;
    }
    let qty = sold_qty.min(state.token_balance.max(0));
    if qty == 0 {
        return;
    }
    let disposed = proportional_cost(state.cost_basis_myso, qty, state.token_balance);
    state.realized_myso = state.realized_myso.saturating_add(proceeds.saturating_sub(disposed));
    state.disposed_cost_basis_myso = state.disposed_cost_basis_myso.saturating_add(disposed);
    state.sold_token_qty = state.sold_token_qty.saturating_add(qty);
    state.exit_proceeds_myso = state.exit_proceeds_myso.saturating_add(proceeds.max(0));
    state.total_returned_myso = state.total_returned_myso.saturating_add(proceeds.max(0));
    state.token_balance = state.token_balance.saturating_sub(qty);
    state.cost_basis_myso = state.cost_basis_myso.saturating_sub(disposed);
}

/// Reservation deposit (positive MYSO) or withdraw (negative MYSO). Tokens stay 0 until launch.
fn apply_reservation(state: &mut UserSptPositionState, myso_delta: i64) {
    if myso_delta > 0 {
        state.reservation_cost_myso = state.reservation_cost_myso.saturating_add(myso_delta);
        state.total_invested_myso = state.total_invested_myso.saturating_add(myso_delta);
    } else if myso_delta < 0 {
        let withdraw = myso_delta.saturating_neg().min(state.reservation_cost_myso.max(0));
        state.reservation_cost_myso = state.reservation_cost_myso.saturating_sub(withdraw);
        state.total_invested_myso = state.total_invested_myso.saturating_sub(withdraw);
    }
}

/// Convert reservation accrual into open-position cost basis with allocated tokens.
fn apply_launch(state: &mut UserSptPositionState, token_qty: i64) {
    if token_qty <= 0 {
        return;
    }
    let cost = state.reservation_cost_myso.max(0);
    state.token_balance = state.token_balance.saturating_add(token_qty);
    state.cost_basis_myso = state.cost_basis_myso.saturating_add(cost);
    state.reservation_cost_myso = 0;
}

fn apply_transfer_out(state: &mut UserSptPositionState, qty: i64) {
    if qty <= 0 {
        return;
    }
    let moved = qty.min(state.token_balance.max(0));
    if moved == 0 {
        return;
    }
    let disposed = proportional_cost(state.cost_basis_myso, moved, state.token_balance);
    state.token_balance = state.token_balance.saturating_sub(moved);
    state.cost_basis_myso = state.cost_basis_myso.saturating_sub(disposed);
}

fn apply_transfer_in(state: &mut UserSptPositionState, qty: i64, inherited_cost: Option<i64>) {
    if qty <= 0 {
        return;
    }
    state.token_balance = state.token_balance.saturating_add(qty);
    match inherited_cost {
        Some(cost) if cost > 0 => {
            state.cost_basis_myso = state.cost_basis_myso.saturating_add(cost);
            state.total_invested_myso = state.total_invested_myso.saturating_add(cost);
        }
        _ => {
            state.cost_basis_unknown = true;
        }
    }
}

fn proportional_cost(cost_basis: i64, qty: i64, balance: i64) -> i64 {
    if balance <= 0 || qty <= 0 {
        return 0;
    }
    let moved = qty.min(balance);
    ((cost_basis as i128) * (moved as i128) / (balance as i128)) as i64
}

pub fn avg_entry_price(state: &UserSptPositionState) -> Option<i64> {
    if state.token_balance <= 0 {
        return None;
    }
    Some(state.cost_basis_myso / state.token_balance)
}

pub fn avg_exit_price(state: &UserSptPositionState) -> Option<i64> {
    if state.sold_token_qty <= 0 {
        return None;
    }
    Some(state.exit_proceeds_myso / state.sold_token_qty)
}

/// Realized ROI % using disposed cost basis.
pub fn realized_roi_pct(state: &UserSptPositionState) -> Option<f64> {
    if state.disposed_cost_basis_myso <= 0 {
        return None;
    }
    Some((state.realized_myso as f64 / state.disposed_cost_basis_myso as f64) * 100.0)
}

pub fn unrealized_myso(state: &UserSptPositionState, mark_value: i64) -> i64 {
    mark_value.saturating_sub(state.cost_basis_myso)
}

pub fn unrealized_roi_pct(state: &UserSptPositionState, mark_value: i64) -> Option<f64> {
    if state.cost_basis_myso <= 0 {
        return None;
    }
    Some((unrealized_myso(state, mark_value) as f64 / state.cost_basis_myso as f64) * 100.0)
}

pub fn lifetime_net_myso(state: &UserSptPositionState, mark_value: i64) -> i64 {
    state.realized_myso.saturating_add(unrealized_myso(state, mark_value))
}

pub fn lifetime_roi_pct(state: &UserSptPositionState, mark_value: i64) -> Option<f64> {
    if state.total_invested_myso <= 0 {
        return None;
    }
    Some((lifetime_net_myso(state, mark_value) as f64 / state.total_invested_myso as f64) * 100.0)
}

/// Window return: window P/L over capital-at-risk. Never subtract two lifetime ROI percentages.
pub fn window_return_pct(window_net_pl: i64, window_capital_at_risk: i64) -> Option<f64> {
    if window_capital_at_risk <= 0 {
        return None;
    }
    Some((window_net_pl as f64 / window_capital_at_risk as f64) * 100.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn empty() -> UserSptPositionState {
        UserSptPositionState::empty("0xholder".into(), "0xpool".into())
    }

    #[test]
    fn buy_then_partial_sell_uses_disposed_cost_basis() {
        let mut s = empty();
        apply_spt_position_event(
            &mut s,
            &PositionEvent {
                kind: PositionEventKind::Buy,
                token_qty: 100,
                myso_amount: 200,
                inherited_cost_myso: None,
            },
        );
        apply_spt_position_event(
            &mut s,
            &PositionEvent {
                kind: PositionEventKind::Sell,
                token_qty: 40,
                myso_amount: 120,
                inherited_cost_myso: None,
            },
        );
        assert_eq!(s.token_balance, 60);
        assert_eq!(s.cost_basis_myso, 120);
        assert_eq!(s.disposed_cost_basis_myso, 80);
        assert_eq!(s.realized_myso, 40);
        assert_eq!(realized_roi_pct(&s), Some(50.0));
        assert_eq!(avg_exit_price(&s), Some(3));
    }

    #[test]
    fn reservation_then_launch_becomes_open_cost_basis() {
        let mut s = empty();
        apply_spt_position_event(
            &mut s,
            &PositionEvent {
                kind: PositionEventKind::Reservation,
                token_qty: 0,
                myso_amount: 1_000,
                inherited_cost_myso: None,
            },
        );
        assert_eq!(s.reservation_cost_myso, 1_000);
        assert_eq!(s.token_balance, 0);
        apply_spt_position_event(
            &mut s,
            &PositionEvent {
                kind: PositionEventKind::Launch,
                token_qty: 50,
                myso_amount: 0,
                inherited_cost_myso: None,
            },
        );
        assert_eq!(s.reservation_cost_myso, 0);
        assert_eq!(s.token_balance, 50);
        assert_eq!(s.cost_basis_myso, 1_000);
        assert_eq!(s.total_invested_myso, 1_000);
    }

    #[test]
    fn reservation_withdraw_reduces_accrual() {
        let mut s = empty();
        apply_reservation(&mut s, 500);
        apply_reservation(&mut s, -200);
        assert_eq!(s.reservation_cost_myso, 300);
        assert_eq!(s.total_invested_myso, 300);
    }

    #[test]
    fn transfer_out_reduces_basis_without_realized_pl() {
        let mut s = empty();
        apply_buy(&mut s, 100, 400);
        apply_transfer_out(&mut s, 25);
        assert_eq!(s.token_balance, 75);
        assert_eq!(s.cost_basis_myso, 300);
        assert_eq!(s.realized_myso, 0);
        assert_eq!(s.disposed_cost_basis_myso, 0);
    }

    #[test]
    fn transfer_in_inherits_or_marks_unknown() {
        let mut known = empty();
        apply_transfer_in(&mut known, 10, Some(50));
        assert_eq!(known.cost_basis_myso, 50);
        assert!(!known.cost_basis_unknown);

        let mut unknown = empty();
        apply_transfer_in(&mut unknown, 10, None);
        assert_eq!(unknown.cost_basis_myso, 0);
        assert!(unknown.cost_basis_unknown);
    }

    #[test]
    fn window_return_is_not_delta_lifetime_roi() {
        // User A: invest 100, make 50 (50% lifetime). Then add 900 more, make 10 more.
        // Lifetime ROI becomes 60/1000 = 6%. Delta lifetime ROI is -44pp — misleading.
        // Window P/L / capital-at-risk for the second window is 10 / 900 ≈ 1.11%.
        let window_pl = 10;
        let capital_at_risk = 900;
        let pct = window_return_pct(window_pl, capital_at_risk).unwrap();
        assert!((pct - 1.111).abs() < 0.01);
        let lifetime_now = 6.0;
        let lifetime_then = 50.0;
        let delta_roi = lifetime_now - lifetime_then;
        assert!(delta_roi < -40.0);
        assert!((pct - delta_roi).abs() > 40.0);
    }
}
