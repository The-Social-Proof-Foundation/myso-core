// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use chrono::{Duration, Utc};

use crate::resolver::MaturitySchedule;
use crate::review::CanonicalClaim;
use crate::types::ClaimCategory;

const MIN_POLL_INTERVAL: Duration = Duration::minutes(1);
const PRE_DEADLINE_WINDOW: Duration = Duration::hours(1);

pub fn compute_schedule(
    canonical: &CanonicalClaim,
    category: ClaimCategory,
) -> MaturitySchedule {
    let now = Utc::now();
    let deadline = canonical
        .normalized_fields
        .deadline
        .unwrap_or_else(|| now + Duration::hours(24));

    let maturity_at = match category {
        ClaimCategory::PriceThreshold => now + MIN_POLL_INTERVAL,
        _ => {
            let candidate = deadline - PRE_DEADLINE_WINDOW;
            if candidate > now + MIN_POLL_INTERVAL {
                candidate
            } else {
                now + MIN_POLL_INTERVAL
            }
        }
    };

    MaturitySchedule {
        maturity_at,
        deadline,
    }
}
