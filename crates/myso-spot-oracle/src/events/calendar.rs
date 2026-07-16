// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Computable recurring public event dates (elections, etc.).

use chrono::{Datelike, NaiveDate, TimeZone, Utc};

/// First Tuesday after November 1 for a given year (U.S. general election day).
pub fn us_election_day(year: i32) -> NaiveDate {
    let nov1 = NaiveDate::from_ymd_opt(year, 11, 1).expect("valid nov 1");
    let weekday = nov1.weekday().num_days_from_monday();
    // Tuesday = 1
    let days_until_tuesday = if weekday <= 1 {
        1 - weekday
    } else {
        8 - weekday
    };
    nov1 + chrono::Duration::days(days_until_tuesday as i64)
}

/// Presidential election years (every 4 years from 2024).
pub fn is_us_presidential_year(year: i32) -> bool {
    year >= 2024 && (year - 2024) % 4 == 0
}

/// Midterm election years (even years that are not presidential).
pub fn is_us_midterm_year(year: i32) -> bool {
    year % 2 == 0 && !is_us_presidential_year(year)
}

pub fn end_of_utc_day(date: NaiveDate) -> chrono::DateTime<Utc> {
    date.and_hms_milli_opt(23, 59, 59, 999)
        .map(|t| Utc.from_utc_datetime(&t))
        .unwrap_or_else(|| Utc.from_utc_datetime(&date.and_hms_opt(0, 0, 0).unwrap()))
}

pub fn us_presidential_election_deadline(year: i32) -> Option<chrono::DateTime<Utc>> {
    if !is_us_presidential_year(year) {
        return None;
    }
    Some(end_of_utc_day(us_election_day(year)))
}

pub fn us_midterm_election_deadline(year: i32) -> Option<chrono::DateTime<Utc>> {
    if !is_us_midterm_year(year) {
        return None;
    }
    Some(end_of_utc_day(us_election_day(year)))
}

/// End of quarter for Q1–Q4 patterns.
pub fn quarter_end(year: i32, quarter: u32) -> Option<NaiveDate> {
    match quarter {
        1 => NaiveDate::from_ymd_opt(year, 3, 31),
        2 => NaiveDate::from_ymd_opt(year, 6, 30),
        3 => NaiveDate::from_ymd_opt(year, 9, 30),
        4 => NaiveDate::from_ymd_opt(year, 12, 31),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn us_election_2028_is_nov_7() {
        assert_eq!(
            us_election_day(2028),
            NaiveDate::from_ymd_opt(2028, 11, 7).unwrap()
        );
    }

    #[test]
    fn us_election_2024_is_nov_5() {
        assert_eq!(
            us_election_day(2024),
            NaiveDate::from_ymd_opt(2024, 11, 5).unwrap()
        );
    }

    #[test]
    fn midterm_2026_is_nov_3() {
        assert_eq!(
            us_election_day(2026),
            NaiveDate::from_ymd_opt(2026, 11, 3).unwrap()
        );
    }
}
