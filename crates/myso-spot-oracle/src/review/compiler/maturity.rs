// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use chrono::{Duration, Utc};

use crate::resolver::MaturitySchedule;
use crate::review::CanonicalClaim;

const MIN_POLL_INTERVAL: Duration = Duration::minutes(1);
const PRE_DEADLINE_WINDOW: Duration = Duration::hours(1);

pub fn compute_schedule(canonical: &CanonicalClaim) -> MaturitySchedule {
    let now = Utc::now();
    let deadline = canonical
        .normalized_fields
        .deadline
        .expect("review validated deadline");

    let candidate = deadline - PRE_DEADLINE_WINDOW;
    let maturity_at = if candidate > now + MIN_POLL_INTERVAL {
        candidate
    } else {
        now + MIN_POLL_INTERVAL
    };

    MaturitySchedule {
        maturity_at,
        deadline,
    }
}
