// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use anyhow::Context as _;
use async_graphql::*;
use move_core_types::annotated_value as A;
use move_core_types::annotated_visitor as AV;

use crate::api::scalars::big_int::BigInt;
use crate::error::RpcError;

use myso_types::myso_system_state::myso_system_state_inner_v1::StakeSubsidyV1;

/// Parameters that control the distribution of the stake subsidy.
#[derive(Clone, Debug, PartialEq, Eq, SimpleObject)]
pub(crate) struct StakeSubsidy {
    /// MYSO set aside for stake subsidies — reduces over time as stake subsidies are paid out.
    pub balance: Option<BigInt>,

    /// Number of times stake subsidies have been distributed (with other staking rewards at end of epoch).
    pub distribution_counter: Option<u64>,

    /// Current stake subsidy APY in basis points — decays over time.
    pub current_apy_bps: Option<BigInt>,

    /// Maximum number of stake subsidy distributions that occur with the same APY (before the APY is reduced).
    pub period_length: Option<u64>,

    /// Percentage of the current APY to deduct at the end of the current subsidy period, in basis points.
    pub decrease_rate: Option<u64>,

    /// Maximum APY cap (in basis points). Effective APY will never exceed this.
    pub max_apy_bps: Option<BigInt>,

    /// Minimum APY floor (in basis points). Effective APY will never go below this.
    pub min_apy_bps: Option<BigInt>,

    /// Target duration for subsidy pool in years (e.g., 10). Used for stake-aware APY reduction.
    pub intended_duration_years: Option<BigInt>,

    /// The annual percentage yield from the stake subsidy in basis points. Divide by 100 for percentage.
    pub apy: Option<u64>,
}

impl StakeSubsidy {
    pub(crate) fn from_v1(v: &StakeSubsidyV1) -> Self {
        Self {
            balance: Some(BigInt::from(v.balance.value())),
            distribution_counter: Some(v.distribution_counter),
            current_apy_bps: Some(BigInt::from(v.current_apy_bps)),
            period_length: Some(v.stake_subsidy_period_length),
            decrease_rate: Some(v.stake_subsidy_decrease_rate as u64),
            max_apy_bps: Some(BigInt::from(v.max_apy_bps)),
            min_apy_bps: Some(BigInt::from(v.min_apy_bps)),
            intended_duration_years: Some(BigInt::from(v.intended_duration_years)),
            apy: Some(v.current_apy_bps),
        }
    }
}

/// Extract stake subsidy from serialized system state inner bytes using its layout.
pub(crate) fn extract_from_system_state(
    bytes: &[u8],
    layout: &A::MoveTypeLayout,
) -> Result<Option<StakeSubsidyV1>, RpcError> {
    #[derive(Default)]
    struct State {
        stake_subsidy_bytes: Option<Vec<u8>>,
    }

    struct Traversal<'a>(&'a mut State);

    #[derive(thiserror::Error, Debug)]
    enum Error {
        #[error(transparent)]
        Bcs(#[from] bcs::Error),
        #[error(transparent)]
        Visitor(#[from] AV::Error),
    }

    impl<'b, 'l> AV::Traversal<'b, 'l> for Traversal<'_> {
        type Error = Error;

        fn traverse_struct(
            &mut self,
            driver: &mut AV::StructDriver<'_, 'b, 'l>,
        ) -> Result<(), Error> {
            while let Some(field) = driver.peek_field() {
                if field.name.as_str() == "stake_subsidy" {
                    let lo = driver.position();
                    driver.skip_field()?;
                    let hi = driver.position();
                    self.0.stake_subsidy_bytes = Some(driver.bytes()[lo..hi].to_vec());
                    break;
                }
                driver.skip_field()?;
            }
            Ok(())
        }
    }

    let mut state = State::default();
    let mut traversal = Traversal(&mut state);
    A::MoveValue::visit_deserialize(bytes, layout, &mut traversal)
        .context("Failed to deserialize system state for stake subsidy")?;

    Ok(state
        .stake_subsidy_bytes
        .as_ref()
        .and_then(|b| bcs::from_bytes::<StakeSubsidyV1>(b).ok()))
}
