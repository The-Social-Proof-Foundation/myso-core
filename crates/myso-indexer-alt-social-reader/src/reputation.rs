// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Off-chain mirror of `memory.move`'s reputation scoring.
//!
//! The chain stores running sums; every score is recomputed from them with the **integer-only**
//! Bayesian formula, using the market's current prior. The test vectors below are the same
//! numbers the Move tests assert (`agent_task_tests.move`), so Move, this module and the
//! TypeScript display cannot drift apart silently.
//!
//! ```text
//! role    score = (C * m + S) / (C + W)             confidence = W * 10000 / (C + W)
//! overall score = (C*m*10000 + a_s*S_s + a_b*S_b) / (C*10000 + a_s*W_s + a_b*W_b)
//! ```
//! `W` is the weight sum in bps, `S` the weight-times-milli-star sum, `m` the prior mean in
//! milli-stars and `C` the prior weight in bps.

const BPS: u128 = 10_000;

/// The slice of `agent_task::TaskConfig` that scoring needs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScoringParams {
    pub prior_mean_milli: u64,
    pub prior_weight_bps: u64,
    pub overall_seller_weight_bps: u64,
    pub overall_buyer_weight_bps: u64,
}

impl Default for ScoringParams {
    /// The genesis defaults in `agent_task::bootstrap_init`, used until a `TaskConfigUpdated`
    /// event has been indexed.
    fn default() -> Self {
        Self {
            prior_mean_milli: 3000,
            prior_weight_bps: 50_000,
            overall_seller_weight_bps: 5000,
            overall_buyer_weight_bps: 5000,
        }
    }
}

/// Running sums for one subject and role.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Aggregate {
    pub review_count: u64,
    pub histogram: [u64; 5],
    pub weight_sum_bps: u128,
    pub weighted_star_sum: u128,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Score {
    pub score_milli: u64,
    pub confidence_bps: u64,
}

impl Score {
    /// Stars with three decimals, for display (`3076` -> `3.076`).
    pub fn stars(&self) -> f64 {
        self.score_milli as f64 / 1000.0
    }
}

/// Parses a decimal `u128` sum as stored by the indexer; `None` for anything else.
pub fn parse_sum(s: &str) -> Option<u128> {
    if s.is_empty() || !s.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    s.parse().ok()
}

fn finish(num: u128, den: u128, evidence: u128) -> Option<Score> {
    let score = u64::try_from(num.checked_div(den)?).ok()?;
    let confidence = u64::try_from(evidence.checked_mul(BPS)?.checked_div(den)?).ok()?;
    // Same invariant the contract asserts when it stores a score.
    ((1000..=5000).contains(&score) && confidence <= BPS as u64).then_some(Score {
        score_milli: score,
        confidence_bps: confidence,
    })
}

/// Per-role Bayesian score. With no reviews this is exactly the prior with zero confidence.
pub fn role_score(agg: &Aggregate, p: &ScoringParams) -> Option<Score> {
    let c = u128::from(p.prior_weight_bps);
    let num = c
        .checked_mul(u128::from(p.prior_mean_milli))?
        .checked_add(agg.weighted_star_sum)?;
    let den = c.checked_add(agg.weight_sum_bps)?;
    finish(num, den, agg.weight_sum_bps)
}

/// Weighted Bayesian average of a subject's seller and buyer roles. A role without reviews
/// contributes nothing, and a human buyer passes an empty seller aggregate.
pub fn overall_score(seller: &Aggregate, buyer: &Aggregate, p: &ScoringParams) -> Option<Score> {
    let c = u128::from(p.prior_weight_bps);
    let a_s = u128::from(p.overall_seller_weight_bps);
    let a_b = u128::from(p.overall_buyer_weight_bps);
    let prior_num = c.checked_mul(u128::from(p.prior_mean_milli))?.checked_mul(BPS)?;
    let prior_den = c.checked_mul(BPS)?;
    let num = prior_num
        .checked_add(a_s.checked_mul(seller.weighted_star_sum)?)?
        .checked_add(a_b.checked_mul(buyer.weighted_star_sum)?)?;
    let evidence = a_s
        .checked_mul(seller.weight_sum_bps)?
        .checked_add(a_b.checked_mul(buyer.weight_sum_bps)?)?;
    let den = prior_den.checked_add(evidence)?;
    finish(num, den, evidence)
}

#[cfg(test)]
mod tests {
    use super::*;

    const P: ScoringParams = ScoringParams {
        prior_mean_milli: 3000,
        prior_weight_bps: 50_000,
        overall_seller_weight_bps: 5000,
        overall_buyer_weight_bps: 5000,
    };

    fn agg(count: u64, histogram: [u64; 5], w: u128, s: u128) -> Aggregate {
        Aggregate {
            review_count: count,
            histogram,
            weight_sum_bps: w,
            weighted_star_sum: s,
        }
    }

    #[test]
    fn no_reviews_is_the_prior_with_no_confidence() {
        assert_eq!(
            role_score(&Aggregate::default(), &P),
            Some(Score { score_milli: 3000, confidence_bps: 0 })
        );
        assert_eq!(
            overall_score(&Aggregate::default(), &Aggregate::default(), &P),
            Some(Score { score_milli: 3000, confidence_bps: 0 })
        );
    }

    // The vectors below are asserted verbatim in agent_task_tests.move.

    #[test]
    fn one_five_star_review_at_base_credibility() {
        // w = 10000 * 2000 / 10000 = 2000; S = 2000 * 5000.
        let a = agg(1, [0, 0, 0, 0, 1], 2000, 10_000_000);
        assert_eq!(role_score(&a, &P), Some(Score { score_milli: 3076, confidence_bps: 384 }));
    }

    #[test]
    fn one_four_star_and_one_two_star_review() {
        assert_eq!(
            role_score(&agg(1, [0, 0, 0, 1, 0], 2000, 8_000_000), &P).unwrap().score_milli,
            3038
        );
        assert_eq!(
            role_score(&agg(1, [0, 1, 0, 0, 0], 2000, 4_000_000), &P).unwrap().score_milli,
            2961
        );
    }

    #[test]
    fn a_repeat_pair_review_adds_half_weight() {
        // Second review: credibility 2500 -> base 2500 -> halved to 1250.
        let a = agg(2, [0, 0, 0, 0, 2], 2000 + 1250, 10_000_000 + 6_250_000);
        assert_eq!(role_score(&a, &P).unwrap().score_milli, 3122);
    }

    #[test]
    fn overall_blends_the_two_roles() {
        let seller = agg(1, [0, 0, 0, 0, 1], 2000, 10_000_000);
        let overall = overall_score(&seller, &Aggregate::default(), &P).unwrap();
        assert_eq!(overall, Score { score_milli: 3039, confidence_bps: 196 });
        // Pulled toward the prior relative to the seller-only score.
        assert!(overall.score_milli < role_score(&seller, &P).unwrap().score_milli);
    }

    #[test]
    fn a_human_buyer_overall_equals_the_buyer_role_scaled_by_its_weight() {
        let buyer = agg(1, [0, 0, 0, 1, 0], 2000, 8_000_000);
        let overall = overall_score(&Aggregate::default(), &buyer, &P).unwrap();
        // (1.5e12 + 5000 * 8_000_000) / (5e8 + 5000 * 2000) = 3019.
        assert_eq!(overall.score_milli, 3019);
    }

    #[test]
    fn sums_beyond_u64_are_exact() {
        let a = agg(1, [0, 0, 0, 0, 1], 18_446_744_073_709_551_616, 92_233_720_368_547_758_080);
        // Weighted mean of 5 stars at overwhelming weight tends to 5000.
        assert_eq!(role_score(&a, &P).unwrap().score_milli, 4999);
    }

    #[test]
    fn overflow_returns_none_instead_of_panicking() {
        let a = agg(1, [0, 0, 0, 0, 1], u128::MAX, 1);
        assert!(role_score(&a, &P).is_none());
    }

    #[test]
    fn sums_parse_only_as_plain_decimals() {
        assert_eq!(parse_sum("340282366920938463463374607431768211455"), Some(u128::MAX));
        assert_eq!(parse_sum("12x"), None);
        assert_eq!(parse_sum(""), None);
        assert_eq!(parse_sum("-1"), None);
    }
}
