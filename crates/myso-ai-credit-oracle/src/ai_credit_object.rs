// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Typed BCS layout for `social_contracts::ai_credit::AiCreditBalance`.
//!
//! Field order must match [`ai_credit.move`](../../myso-framework/packages/myso-social/sources/ai_credit.move).

use move_core_types::account_address::AccountAddress;
use myso_types::balance::Balance;
use myso_types::collection_types::Table;
use myso_types::id::{ID, UID};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BcsAiCreditBalanceScalars {
    _id: UID,
    _memory_account_id: ID,
    _principal_owner: AccountAddress,
    _profile_id: AccountAddress,
    _balance: Balance,
    _spent_total_mist: u64,
    pub reserved_mist: u64,
    _spent_day_mist: u64,
    _reserved_day_mist: u64,
    _spent_month_mist: u64,
    _reserved_month_mist: u64,
    _day_anchor_ms: u64,
    _month_anchor_ms: u64,
    _daily_cap_mist: Option<u64>,
    _monthly_cap_mist: Option<u64>,
    pub settlement_nonce: u64,
    pub reservation_nonce: u64,
    _reservations: Table,
    _agent_budgets: Table,
    _active: bool,
    _version: u64,
}

pub fn parse_settlement_nonce(data: &[u8]) -> Result<u64, bcs::Error> {
    let parsed: BcsAiCreditBalanceScalars = bcs::from_bytes(data)?;
    Ok(parsed.settlement_nonce)
}

pub fn parse_reservation_state(data: &[u8]) -> Result<(u64, u64), bcs::Error> {
    let parsed: BcsAiCreditBalanceScalars = bcs::from_bytes(data)?;
    Ok((parsed.reservation_nonce, parsed.reserved_mist))
}

#[cfg(test)]
mod tests {
    use super::*;
    use myso_types::base_types::ObjectID;

    #[test]
    fn parses_settlement_nonce_from_known_layout() {
        let fixture = BcsAiCreditBalanceScalars {
            _id: UID::new(ObjectID::ZERO),
            _memory_account_id: ID {
                bytes: ObjectID::ZERO,
            },
            _principal_owner: AccountAddress::ZERO,
            _profile_id: AccountAddress::ZERO,
            _balance: Balance::new(0),
            _spent_total_mist: 0,
            reserved_mist: 0,
            _spent_day_mist: 0,
            _reserved_day_mist: 0,
            _spent_month_mist: 0,
            _reserved_month_mist: 0,
            _day_anchor_ms: 0,
            _month_anchor_ms: 0,
            _daily_cap_mist: None,
            _monthly_cap_mist: None,
            settlement_nonce: 3,
            reservation_nonce: 7,
            _reservations: Table::default(),
            _agent_budgets: Table::default(),
            _active: true,
            _version: 1,
        };
        let bytes = bcs::to_bytes(&fixture).unwrap();
        assert_eq!(parse_settlement_nonce(&bytes).unwrap(), 3);
        assert_eq!(parse_reservation_state(&bytes).unwrap(), (7, 0));
    }
}
