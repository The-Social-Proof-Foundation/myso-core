// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use super::{ProfileUpdate, SocialEventRow};
use myso_indexer_alt_social_schema::models::{
    NewSocialProofTokensConfig, NewSocialProofTokensEvent, NewSptExchangeConfig, NewSptHolding,
    NewSptPool, NewSptPriceHistory, NewSptReservation, NewSptReservationPool, NewSptTransaction,
    RESERVATION_POOL_STATUS_ACTIVE, RESERVATION_POOL_STATUS_THRESHOLD_MET, TOKEN_TYPE_POST,
    TOKEN_TYPE_PROFILE, TRANSACTION_TYPE_BUY, TRANSACTION_TYPE_SELL,
};

fn transaction_id_from_event_id(event_id: &str) -> String {
    event_id.split(':').next().unwrap_or(event_id).to_string()
}

/// Minimum `reserved_at` / `withdrawn_at` value (milliseconds) treated as Unix epoch ms for hypertable
/// `time`. Chain-local or sim clocks often emit small values (e.g. ms since genesis); those are not
/// wall-clock Unix ms and would map to 1970 if used directly.
const MIN_PLAUSIBLE_RESERVATION_UNIX_MS: i64 = 1_000_000_000_000;

/// Hypertable `time` for reservation ledger rows. Prefers on-chain timestamps when they look like Unix
/// ms; otherwise uses checkpoint ms (`created_at`) or `fallback` so analytics windows match indexing time.
fn reservation_row_time(
    chain_event_ms: i64,
    checkpoint_ts_ms: u64,
    fallback: chrono::DateTime<chrono::Utc>,
) -> chrono::DateTime<chrono::Utc> {
    if chain_event_ms >= MIN_PLAUSIBLE_RESERVATION_UNIX_MS {
        return chrono::DateTime::from_timestamp_millis(chain_event_ms).unwrap_or(fallback);
    }
    if checkpoint_ts_ms > 0 {
        chrono::DateTime::from_timestamp_millis(checkpoint_ts_ms as i64).unwrap_or(fallback)
    } else {
        fallback
    }
}

fn json_to_i64(v: &serde_json::Value) -> i64 {
    v.as_i64()
        .or_else(|| v.as_u64().and_then(|u| u.try_into().ok()))
        .unwrap_or(0)
}

/// JSON number from chain events (`u64` in Move) clamped to `i64` for BIGINT columns.
fn json_u64_to_i64_opt(v: Option<&serde_json::Value>) -> i64 {
    let Some(v) = v else {
        return 0;
    };
    if let Some(i) = v.as_i64() {
        return i;
    }
    if let Some(u) = v.as_u64() {
        return u.min(i64::MAX as u64) as i64;
    }
    0
}

fn json_str(v: &serde_json::Value) -> Option<String> {
    v.as_str().map(String::from)
}

fn token_type_from_u8(t: u64) -> Option<i16> {
    match t {
        1 => Some(TOKEN_TYPE_PROFILE),
        2 => Some(TOKEN_TYPE_POST),
        _ => None,
    }
}

/// `spt_reservations.amount` sign convention for volume analytics:
/// - **Positive**: net MYSO deposited into the reservation pool (ReservationCreatedEvent).
/// - **Negative**: net MYSO withdrawn by the reserver (ReservationWithdrawnEvent; magnitude is the withdrawn amount).

pub fn handle_spt_event(
    event_name: &str,
    data: &serde_json::Value,
    event_id: &str,
    _epoch: u64,
    timestamp_ms: u64,
) -> Option<Vec<SocialEventRow>> {
    let transaction_id = transaction_id_from_event_id(event_id);
    let now = chrono::Utc::now();
    let ts = timestamp_ms as i64;

    match event_name {
        "TokenPoolCreatedEvent" | "PoolCreatedEvent" => {
            process_token_pool_created_event(data, &transaction_id, ts, now)
        }
        "TokenBoughtEvent" | "BuyEvent" => {
            process_token_bought_event(data, &transaction_id, ts, now)
        }
        "TokenSoldEvent" | "SellEvent" => process_token_sold_event(data, &transaction_id, ts, now),
        "ReservationPoolCreatedEvent" => {
            process_reservation_pool_created_event(data, &transaction_id, ts, now)
        }
        "ReservationCreatedEvent" => {
            process_reservation_created_event(data, event_id, timestamp_ms, ts, now)
        }
        "ReservationWithdrawnEvent" => {
            process_reservation_withdrawn_event(data, event_id, timestamp_ms, ts, now)
        }
        "ThresholdMetEvent" => process_threshold_met_event(data, &transaction_id, ts, now),
        "ConfigUpdatedEvent" => process_spt_config_updated_event(data, &transaction_id, ts, now),
        "EmergencyKillSwitchEvent" => {
            process_emergency_kill_switch_event(data, event_id, &transaction_id, ts, now)
        }
        "SocialProofInitPoolEvent" | "InitPoolEvent" => {
            process_social_proof_init_pool_event(data, &transaction_id, ts, now)
        }
        "SocialProofBuyEvent" => process_social_proof_buy_event(data, &transaction_id, ts, now),
        "SocialProofSellEvent" => process_social_proof_sell_event(data, &transaction_id, ts, now),
        "TokensAddedEvent" => process_tokens_added_event(data, &transaction_id, ts, now),
        "PocRedirectionUpdatedEvent" => {
            process_poc_redirection_updated_event(data, &transaction_id)
        }
        _ => None,
    }
}

fn process_token_pool_created_event(
    data: &serde_json::Value,
    transaction_id: &str,
    ts: i64,
    now: chrono::DateTime<chrono::Utc>,
) -> Option<Vec<SocialEventRow>> {
    let id = json_str(data.get("id")?)?;
    let token_type = token_type_from_u8(data.get("token_type")?.as_u64()?)?;
    let owner = json_str(data.get("owner")?)?;
    let associated_id = json_str(data.get("associated_id")?)?;
    let symbol = json_str(data.get("symbol")?).unwrap_or_default();
    let name = json_str(data.get("name")?).unwrap_or_default();
    let base_price = json_to_i64(data.get("base_price")?);
    let quadratic_coefficient = json_to_i64(data.get("quadratic_coefficient")?);
    let circulating_supply = json_u64_to_i64_opt(data.get("circulating_supply"));
    let total_reserved_at_launch = json_u64_to_i64_opt(data.get("total_reserved_at_launch"));

    let pool = NewSptPool {
        pool_id: id.clone(),
        token_type,
        owner: owner.clone(),
        associated_id: associated_id.clone(),
        symbol,
        name,
        circulating_supply,
        base_price,
        quadratic_coefficient,
        created_at: ts,
        time: now,
        transaction_id: transaction_id.to_string(),
    };

    let price_history = NewSptPriceHistory {
        pool_id: id.clone(),
        price: base_price,
        circulating_supply,
        time: now,
        transaction_id: transaction_id.to_string(),
    };

    let mut rows = vec![
        SocialEventRow::SptPool(pool),
        SocialEventRow::SptPriceHistory(price_history),
    ];

    if token_type == TOKEN_TYPE_PROFILE {
        rows.push(SocialEventRow::ProfileUpdate(ProfileUpdate {
            profile_id: associated_id.clone(),
            owner_address: owner.clone(),
            display_name: None,
            bio: None,
            profile_photo: None,
            cover_photo: None,
            birthdate: None,
            current_location: None,
            raised_location: None,
            phone: None,
            email: None,
            gender: None,
            political_view: None,
            religion: None,
            education: None,
            primary_language: None,
            relationship_status: None,
            x_username: None,
            min_offer_amount: None,
            username: None,
            selected_badge_id: None,
            selected_ecosystem_badge_id: None,
            paid_messaging_enabled: None,
            paid_messaging_min_cost: None,
            reservation_pool_address: None,
            social_proof_token_address: Some(Some(id.clone())),
        }));
    }

    if circulating_supply > 0 {
        rows.push(SocialEventRow::SptLaunchHoldingsFromReservations {
            pool_id: id,
            associated_id,
            owner,
            circulating_supply,
            total_reserved_at_launch,
            created_at: ts,
            time: now,
            transaction_id: transaction_id.to_string(),
        });
    }

    Some(rows)
}

fn process_token_bought_event(
    data: &serde_json::Value,
    transaction_id: &str,
    ts: i64,
    now: chrono::DateTime<chrono::Utc>,
) -> Option<Vec<SocialEventRow>> {
    let id = json_str(data.get("id")?)?;
    let buyer = json_str(data.get("buyer")?)?;
    let amount = json_to_i64(data.get("amount")?);
    let myso_amount = json_to_i64(data.get("myso_amount")?);
    let fee_amount = json_to_i64(data.get("fee_amount")?);
    let creator_fee = json_to_i64(data.get("creator_fee")?);
    let platform_fee = json_to_i64(data.get("platform_fee")?);
    let treasury_fee = json_to_i64(data.get("treasury_fee")?);
    let new_price = json_to_i64(data.get("new_price")?);

    let tx = NewSptTransaction {
        pool_id: id.clone(),
        transaction_type: TRANSACTION_TYPE_BUY.to_string(),
        sender: buyer.clone(),
        amount,
        myso_amount,
        fee_amount,
        creator_fee,
        platform_fee,
        treasury_fee,
        price: new_price,
        created_at: ts,
        time: now,
        transaction_id: transaction_id.to_string(),
    };

    let holding = NewSptHolding {
        pool_id: id.clone(),
        holder_address: buyer.clone(),
        amount,
        acquired_at: ts,
        time: now,
        transaction_id: transaction_id.to_string(),
    };

    let price_history = NewSptPriceHistory {
        pool_id: id.clone(),
        price: new_price,
        circulating_supply: 0,
        time: now,
        transaction_id: transaction_id.to_string(),
    };

    let mut rows: Vec<SocialEventRow> = vec![
        SocialEventRow::SptTransaction(tx),
        SocialEventRow::SptHolding(holding),
        SocialEventRow::SptPoolSupplyUpdate {
            pool_id: id.clone(),
            delta: amount,
        },
        SocialEventRow::SptPriceHistory(price_history),
    ];

    if creator_fee != 0 || platform_fee != 0 || treasury_fee != 0 {
        rows.push(SocialEventRow::SptBuySellRevenueData {
            pool_id: id.clone(),
            associated_id: String::new(),
            token_type: 0,
            trader: buyer,
            transaction_type: TRANSACTION_TYPE_BUY.to_string(),
            creator_fee,
            platform_fee,
            treasury_fee,
            amount,
            myso_amount,
            token_price: new_price,
            revenue_time: ts,
            transaction_id: transaction_id.to_string(),
        });
    }

    Some(rows)
}

fn process_token_sold_event(
    data: &serde_json::Value,
    transaction_id: &str,
    ts: i64,
    now: chrono::DateTime<chrono::Utc>,
) -> Option<Vec<SocialEventRow>> {
    let id = json_str(data.get("id")?)?;
    let seller = json_str(data.get("seller")?)?;
    let amount = json_to_i64(data.get("amount")?);
    let myso_amount = json_to_i64(data.get("myso_amount")?);
    let fee_amount = json_to_i64(data.get("fee_amount")?);
    let creator_fee = json_to_i64(data.get("creator_fee")?);
    let platform_fee = json_to_i64(data.get("platform_fee")?);
    let treasury_fee = json_to_i64(data.get("treasury_fee")?);
    let new_price = json_to_i64(data.get("new_price")?);

    let tx = NewSptTransaction {
        pool_id: id.clone(),
        transaction_type: TRANSACTION_TYPE_SELL.to_string(),
        sender: seller.clone(),
        amount: -amount,
        myso_amount: -myso_amount,
        fee_amount,
        creator_fee,
        platform_fee,
        treasury_fee,
        price: new_price,
        created_at: ts,
        time: now,
        transaction_id: transaction_id.to_string(),
    };

    let holding = NewSptHolding {
        pool_id: id.clone(),
        holder_address: seller.clone(),
        amount: -amount,
        acquired_at: ts,
        time: now,
        transaction_id: transaction_id.to_string(),
    };

    let price_history = NewSptPriceHistory {
        pool_id: id.clone(),
        price: new_price,
        circulating_supply: 0,
        time: now,
        transaction_id: transaction_id.to_string(),
    };

    let mut rows: Vec<SocialEventRow> = vec![
        SocialEventRow::SptTransaction(tx),
        SocialEventRow::SptHolding(holding),
        SocialEventRow::SptPoolSupplyUpdate {
            pool_id: id.clone(),
            delta: -amount,
        },
        SocialEventRow::SptPriceHistory(price_history),
    ];

    if creator_fee != 0 || platform_fee != 0 || treasury_fee != 0 {
        rows.push(SocialEventRow::SptBuySellRevenueData {
            pool_id: id.clone(),
            associated_id: String::new(),
            token_type: 0,
            trader: seller,
            transaction_type: TRANSACTION_TYPE_SELL.to_string(),
            creator_fee,
            platform_fee,
            treasury_fee,
            amount,
            myso_amount,
            token_price: new_price,
            revenue_time: ts,
            transaction_id: transaction_id.to_string(),
        });
    }

    Some(rows)
}

fn process_tokens_added_event(
    data: &serde_json::Value,
    transaction_id: &str,
    ts: i64,
    now: chrono::DateTime<chrono::Utc>,
) -> Option<Vec<SocialEventRow>> {
    let pool_id = json_str(data.get("pool_id")?)?;
    let owner = json_str(data.get("owner")?)?;
    let amount = json_to_i64(data.get("amount")?);

    let holding = NewSptHolding {
        pool_id: pool_id.clone(),
        holder_address: owner,
        amount,
        acquired_at: ts,
        time: now,
        transaction_id: transaction_id.to_string(),
    };

    Some(vec![
        SocialEventRow::SptHolding(holding),
        SocialEventRow::SptPoolSupplyUpdate {
            pool_id,
            delta: amount,
        },
    ])
}

fn process_reservation_pool_created_event(
    data: &serde_json::Value,
    transaction_id: &str,
    ts: i64,
    now: chrono::DateTime<chrono::Utc>,
) -> Option<Vec<SocialEventRow>> {
    let pool_object_id = json_str(data.get("pool_object_id")?)?;
    let associated_id = json_str(data.get("associated_id")?)?;
    let token_type = token_type_from_u8(data.get("token_type")?.as_u64()?)?;
    let owner = json_str(data.get("owner")?)?;
    let required_threshold = json_to_i64(data.get("required_threshold")?);

    let pool = NewSptReservationPool {
        pool_id: pool_object_id.clone(),
        associated_id: associated_id.clone(),
        token_type,
        owner: owner.clone(),
        total_reserved: 0,
        required_threshold,
        status: RESERVATION_POOL_STATUS_ACTIVE.to_string(),
        created_at: ts,
        time: now,
        transaction_id: transaction_id.to_string(),
    };

    let mut rows = vec![SocialEventRow::SptReservationPool(pool)];

    if token_type == TOKEN_TYPE_PROFILE {
        let profile_update = ProfileUpdate {
            profile_id: associated_id,
            owner_address: owner,
            display_name: None,
            bio: None,
            profile_photo: None,
            cover_photo: None,
            birthdate: None,
            current_location: None,
            raised_location: None,
            phone: None,
            email: None,
            gender: None,
            political_view: None,
            religion: None,
            education: None,
            primary_language: None,
            relationship_status: None,
            x_username: None,
            min_offer_amount: None,
            username: None,
            selected_badge_id: None,
            selected_ecosystem_badge_id: None,
            paid_messaging_enabled: None,
            paid_messaging_min_cost: None,
            reservation_pool_address: Some(Some(pool_object_id)),
            social_proof_token_address: None,
        };
        rows.push(SocialEventRow::ProfileUpdate(profile_update));
    }

    Some(rows)
}

fn process_reservation_created_event(
    data: &serde_json::Value,
    event_id: &str,
    checkpoint_ts_ms: u64,
    ts: i64,
    now: chrono::DateTime<chrono::Utc>,
) -> Option<Vec<SocialEventRow>> {
    let associated_id = json_str(data.get("associated_id")?)?;
    let reserver = json_str(data.get("reserver")?)?;
    let amount = json_to_i64(data.get("amount")?);
    let total_reserved = json_to_i64(data.get("total_reserved")?);
    let threshold_met = data
        .get("threshold_met")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let token_type =
        token_type_from_u8(data.get("token_type")?.as_u64()?).unwrap_or(TOKEN_TYPE_POST);
    // Move uses `epoch_timestamp_ms` for this field — already milliseconds (including sim / small values).
    let reserved_at = json_to_i64(data.get("reserved_at")?);
    let fee_amount = data.get("fee_amount").map(json_to_i64);
    let creator_fee = data.get("creator_fee").map(json_to_i64);
    let platform_fee = data.get("platform_fee").map(json_to_i64);
    let treasury_fee = data.get("treasury_fee").map(json_to_i64);

    let pool_id = format!("reservation_pool_{}", associated_id);
    let status = if threshold_met {
        RESERVATION_POOL_STATUS_THRESHOLD_MET.to_string()
    } else {
        RESERVATION_POOL_STATUS_ACTIVE.to_string()
    };

    let row_time = reservation_row_time(reserved_at, checkpoint_ts_ms, now);
    let reservation = NewSptReservation {
        pool_id: pool_id.clone(),
        reserver_address: reserver,
        amount,
        reserved_at,
        created_at: ts,
        fee_amount,
        creator_fee,
        platform_fee,
        treasury_fee,
        time: row_time,
        transaction_id: event_id.to_string(),
    };

    Some(vec![
        SocialEventRow::SptReservation {
            associated_id: associated_id.clone(),
            reservation,
            token_type,
            total_reserved,
            threshold_met,
            created_at: reserved_at,
        },
        SocialEventRow::SptReservationPoolUpdate {
            pool_id,
            associated_id: associated_id.clone(),
            total_reserved,
            status: Some(status),
            required_threshold: None,
        },
    ])
}

fn process_reservation_withdrawn_event(
    data: &serde_json::Value,
    event_id: &str,
    checkpoint_ts_ms: u64,
    ts: i64,
    now: chrono::DateTime<chrono::Utc>,
) -> Option<Vec<SocialEventRow>> {
    let associated_id = json_str(data.get("associated_id")?)?;
    let reserver = json_str(data.get("reserver")?)?;
    let total_reserved = json_to_i64(data.get("total_reserved")?);
    let token_type =
        token_type_from_u8(data.get("token_type")?.as_u64()?).unwrap_or(TOKEN_TYPE_POST);
    // Same as `reserved_at`: chain supplies epoch milliseconds.
    let withdrawn_at = json_to_i64(data.get("withdrawn_at")?);
    let fee_amount = data.get("fee_amount").map(json_to_i64);
    let creator_fee = data.get("creator_fee").map(json_to_i64);
    let platform_fee = data.get("platform_fee").map(json_to_i64);
    let treasury_fee = data.get("treasury_fee").map(json_to_i64);

    let pool_id = format!("reservation_pool_{}", associated_id);

    let withdrawn = data
        .get("amount")
        .map(json_to_i64)
        .filter(|&a| a > 0)
        .unwrap_or(0);
    let amount = withdrawn.checked_neg().unwrap_or(i64::MIN);

    let row_time = reservation_row_time(withdrawn_at, checkpoint_ts_ms, now);
    let reservation = NewSptReservation {
        pool_id: pool_id.clone(),
        reserver_address: reserver,
        amount,
        reserved_at: withdrawn_at,
        created_at: ts,
        fee_amount,
        creator_fee,
        platform_fee,
        treasury_fee,
        time: row_time,
        transaction_id: event_id.to_string(),
    };

    Some(vec![
        SocialEventRow::SptReservation {
            associated_id: associated_id.clone(),
            reservation,
            token_type,
            total_reserved,
            threshold_met: false,
            created_at: withdrawn_at,
        },
        SocialEventRow::SptReservationPoolUpdate {
            pool_id,
            associated_id: associated_id.clone(),
            total_reserved,
            status: None,
            required_threshold: None,
        },
    ])
}

fn process_threshold_met_event(
    data: &serde_json::Value,
    _transaction_id: &str,
    _ts: i64,
    _now: chrono::DateTime<chrono::Utc>,
) -> Option<Vec<SocialEventRow>> {
    let associated_id = json_str(data.get("associated_id")?)?;
    let total_reserved = json_to_i64(data.get("total_reserved")?);
    let required_threshold = json_to_i64(data.get("required_threshold")?);

    let pool_id = format!("reservation_pool_{}", associated_id);

    Some(vec![SocialEventRow::SptReservationPoolUpdate {
        pool_id,
        associated_id,
        total_reserved,
        status: Some(RESERVATION_POOL_STATUS_THRESHOLD_MET.to_string()),
        required_threshold: Some(required_threshold),
    }])
}

fn process_spt_config_updated_event(
    data: &serde_json::Value,
    transaction_id: &str,
    ts: i64,
    now: chrono::DateTime<chrono::Utc>,
) -> Option<Vec<SocialEventRow>> {
    let updated_by = json_str(data.get("updated_by")?)?;
    let total_fee_bps = json_to_i64(data.get("total_fee_bps")?);
    let trading_creator_fee_bps = json_to_i64(data.get("trading_creator_fee_bps")?);
    let trading_platform_fee_bps = json_to_i64(data.get("trading_platform_fee_bps")?);
    let trading_treasury_fee_bps = json_to_i64(data.get("trading_treasury_fee_bps")?);
    let _reservation_total_fee_bps = json_to_i64(data.get("reservation_total_fee_bps")?);
    let reservation_creator_fee_bps = json_to_i64(data.get("reservation_creator_fee_bps")?);
    let reservation_platform_fee_bps = json_to_i64(data.get("reservation_platform_fee_bps")?);
    let reservation_treasury_fee_bps = json_to_i64(data.get("reservation_treasury_fee_bps")?);
    let base_price = json_to_i64(data.get("base_price")?);
    let quadratic_coefficient = json_to_i64(data.get("quadratic_coefficient")?);
    let max_hold_percent_bps = json_to_i64(data.get("max_hold_percent_bps")?);
    let post_threshold = json_to_i64(data.get("post_threshold")?);
    let profile_threshold = json_to_i64(data.get("profile_threshold")?);
    let max_individual_reservation_bps = json_to_i64(data.get("max_individual_reservation_bps")?);
    let max_reservers_per_pool = json_to_i64(data.get("max_reservers_per_pool")?);

    let config = NewSptExchangeConfig {
        updated_by,
        post_threshold,
        profile_threshold,
        max_individual_reservation_bps,
        total_fee_bps,
        creator_fee_bps: trading_creator_fee_bps,
        platform_fee_bps: trading_platform_fee_bps,
        treasury_fee_bps: trading_treasury_fee_bps,
        trading_creator_fee_bps,
        trading_platform_fee_bps,
        trading_treasury_fee_bps,
        reservation_creator_fee_bps,
        reservation_platform_fee_bps,
        reservation_treasury_fee_bps,
        max_reservers_per_pool,
        base_price,
        quadratic_coefficient,
        max_hold_percent_bps,
        trading_enabled: None,
        apply_trading_enabled_only: false,
        updated_at: ts,
        time: now,
        transaction_id: transaction_id.to_string(),
    };

    Some(vec![SocialEventRow::SptExchangeConfig(config)])
}

fn process_emergency_kill_switch_event(
    data: &serde_json::Value,
    event_id: &str,
    transaction_id: &str,
    ts: i64,
    now: chrono::DateTime<chrono::Utc>,
) -> Option<Vec<SocialEventRow>> {
    let admin = json_str(data.get("admin")?)?;
    let trading_enabled = data.get("trading_enabled")?.as_bool().unwrap_or(false);
    let reason = json_str(data.get("reason")?).unwrap_or_default();

    let config = NewSocialProofTokensConfig {
        trading_enabled,
        admin_address: admin.clone(),
        reason: reason.clone(),
        timestamp_ms: ts,
        updated_at: now,
        transaction_id: transaction_id.to_string(),
    };

    let event_log = NewSocialProofTokensEvent {
        event_type: "EmergencyKillSwitchEvent".to_string(),
        event_data: data.clone(),
        event_id: event_id.to_string(),
        created_at: now,
    };

    let exchange_config = myso_indexer_alt_social_schema::models::NewSptExchangeConfig {
        updated_by: admin,
        post_threshold: 0,
        profile_threshold: 0,
        max_individual_reservation_bps: 0,
        total_fee_bps: 0,
        creator_fee_bps: 0,
        platform_fee_bps: 0,
        treasury_fee_bps: 0,
        trading_creator_fee_bps: 0,
        trading_platform_fee_bps: 0,
        trading_treasury_fee_bps: 0,
        reservation_creator_fee_bps: 0,
        reservation_platform_fee_bps: 0,
        reservation_treasury_fee_bps: 0,
        max_reservers_per_pool: 0,
        base_price: 0,
        quadratic_coefficient: 0,
        max_hold_percent_bps: 0,
        trading_enabled: Some(trading_enabled),
        apply_trading_enabled_only: true,
        updated_at: ts,
        time: now,
        transaction_id: transaction_id.to_string(),
    };

    Some(vec![
        SocialEventRow::SptExchangeConfig(exchange_config),
        SocialEventRow::SocialProofTokensConfig(config),
        SocialEventRow::SocialProofTokensEvent(event_log),
    ])
}

fn process_social_proof_init_pool_event(
    data: &serde_json::Value,
    transaction_id: &str,
    ts: i64,
    now: chrono::DateTime<chrono::Utc>,
) -> Option<Vec<SocialEventRow>> {
    let id = json_str(data.get("id")?)?;
    let token_type = token_type_from_u8(data.get("token_type")?.as_u64()?)?;
    let owner = json_str(data.get("owner")?)?;
    let associated_id = json_str(data.get("associated_id")?)?;
    let symbol = json_str(data.get("symbol")?).unwrap_or_default();
    let name = json_str(data.get("name")?).unwrap_or_default();
    let base_price = json_to_i64(data.get("base_price")?);
    let quadratic_coefficient = json_to_i64(data.get("quadratic_coefficient")?);

    let pool = NewSptPool {
        pool_id: id,
        token_type,
        owner,
        associated_id,
        symbol,
        name,
        circulating_supply: 0,
        base_price,
        quadratic_coefficient,
        created_at: ts,
        time: now,
        transaction_id: transaction_id.to_string(),
    };

    Some(vec![SocialEventRow::SptPool(pool)])
}

fn process_social_proof_buy_event(
    data: &serde_json::Value,
    transaction_id: &str,
    ts: i64,
    now: chrono::DateTime<chrono::Utc>,
) -> Option<Vec<SocialEventRow>> {
    process_token_bought_event(data, transaction_id, ts, now)
}

fn process_social_proof_sell_event(
    data: &serde_json::Value,
    transaction_id: &str,
    ts: i64,
    now: chrono::DateTime<chrono::Utc>,
) -> Option<Vec<SocialEventRow>> {
    process_token_sold_event(data, transaction_id, ts, now)
}

fn process_poc_redirection_updated_event(
    data: &serde_json::Value,
    _transaction_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let post_id = json_str(data.get("post_id")?)?;
    let revenue_redirect_to = data
        .get("redirect_to")
        .and_then(|v| v.as_str())
        .map(String::from)
        .unwrap_or_default();
    let revenue_redirect_percentage = json_to_i64(data.get("redirect_percentage")?);

    Some(vec![SocialEventRow::PostRevenueRedirectUpdate {
        post_id,
        revenue_redirect_to,
        revenue_redirect_percentage,
    }])
}

#[cfg(test)]
mod tests {
    use super::super::SocialEventRow;
    use super::handle_spt_event;
    use myso_indexer_alt_social_schema::models::{
        NewSptExchangeConfig, RESERVATION_POOL_STATUS_ACTIVE,
        RESERVATION_POOL_STATUS_THRESHOLD_MET, TOKEN_TYPE_PROFILE,
    };
    use serde_json::json;

    #[test]
    fn reservation_created_small_reserved_at_uses_checkpoint_for_hypertable_time() {
        const CK_MS: u64 = 1_700_000_000_000;
        let data = json!({
            "associated_id": "0xprof",
            "token_type": TOKEN_TYPE_PROFILE,
            "reserver": "0xr",
            "amount": 1i64,
            "total_reserved": 1i64,
            "threshold_met": false,
            "reserved_at": 126000i64,
        });
        let rows =
            handle_spt_event("ReservationCreatedEvent", &data, "tx:0", 0, CK_MS).expect("rows");
        let reservation = rows.iter().find_map(|r| {
            if let SocialEventRow::SptReservation { reservation, .. } = r {
                Some(reservation)
            } else {
                None
            }
        });
        let res = reservation.expect("reservation");
        assert_eq!(
            res.reserved_at, 126000,
            "raw chain field preserved; values below 1e12 are not used as Unix ms for time"
        );
        assert_eq!(res.transaction_id, "tx:0");
        assert_eq!(
            res.time,
            chrono::DateTime::from_timestamp_millis(CK_MS as i64).unwrap(),
            "sim/genesis-relative reserved_at must not set hypertable time to 1970"
        );
        assert_eq!(res.created_at, CK_MS as i64);
    }

    #[test]
    fn threshold_met_emits_pool_update_not_new_pool_row() {
        let data = json!({
            "associated_id": "0xabc123",
            "token_type": 1u64,
            "owner": "0xowner",
            "total_reserved": 5000i64,
            "required_threshold": 100i64,
            "timestamp": 1u64,
        });
        let rows = handle_spt_event("ThresholdMetEvent", &data, "tx1:0", 0, 1000)
            .expect("handler should return rows");

        assert_eq!(rows.len(), 1);
        match &rows[0] {
            SocialEventRow::SptReservationPoolUpdate {
                associated_id,
                total_reserved,
                status,
                required_threshold,
                ..
            } => {
                assert_eq!(associated_id, "0xabc123");
                assert_eq!(*total_reserved, 5000);
                assert_eq!(
                    status.as_deref(),
                    Some(RESERVATION_POOL_STATUS_THRESHOLD_MET)
                );
                assert_eq!(*required_threshold, Some(100));
            }
            SocialEventRow::SptReservationPool(_) => {
                panic!("ThresholdMetEvent must not insert a new spt_reservation_pools row");
            }
            other => panic!("unexpected row: {other:?}"),
        }
    }

    #[test]
    fn reservation_pool_created_uses_object_id_threshold_met_does_not_add_second_pool() {
        let pool_object_id = "0xpoolobjectdeadbeef";
        let created = json!({
            "pool_object_id": pool_object_id,
            "associated_id": "0xprofile",
            "token_type": 1u64,
            "owner": "0xowner",
            "required_threshold": 10_000_000_000_000i64,
            "created_at": 100i64,
        });
        let created_rows =
            handle_spt_event("ReservationPoolCreatedEvent", &created, "tx0:0", 0, 1000)
                .expect("pool created");
        assert!(
            created_rows
                .iter()
                .any(|r| matches!(r, SocialEventRow::SptReservationPool(p) if p.pool_id == pool_object_id)),
            "expected reservation pool row with on-chain pool_object_id"
        );

        let threshold = json!({
            "associated_id": "0xprofile",
            "token_type": 1u64,
            "owner": "0xowner",
            "total_reserved": 10_000_000_000_000i64,
            "required_threshold": 10_000_000_000_000i64,
            "timestamp": 200u64,
        });
        let th_rows = handle_spt_event("ThresholdMetEvent", &threshold, "tx1:0", 0, 2000)
            .expect("threshold met");
        assert!(
            th_rows
                .iter()
                .all(|r| !matches!(r, SocialEventRow::SptReservationPool(_))),
            "threshold met must only update the existing pool, not insert another row"
        );

        let reserve = json!({
            "associated_id": "0xprofile",
            "token_type": TOKEN_TYPE_PROFILE,
            "reserver": "0xreserver",
            "amount": 100i64,
            "total_reserved": 100i64,
            "threshold_met": false,
            "reserved_at": 300i64,
        });
        let res_rows = handle_spt_event("ReservationCreatedEvent", &reserve, "tx2:0", 0, 3000)
            .expect("reservation");
        let reservation = res_rows.iter().find_map(|r| {
            if let SocialEventRow::SptReservation { reservation, .. } = r {
                Some(reservation)
            } else {
                None
            }
        });
        assert!(reservation.is_some(), "expected SptReservation row");
        let res = reservation.unwrap();
        let placeholder = format!("reservation_pool_{}", "0xprofile");
        assert_eq!(
            res.pool_id, placeholder,
            "handler placeholder id; DB apply overwrites from latest spt_reservation_pools.pool_id"
        );
        assert_eq!(res.transaction_id, "tx2:0");
        assert_eq!(
            res.time,
            chrono::DateTime::from_timestamp_millis(3000).unwrap(),
            "reserved_at 300 is not plausible Unix ms; time follows checkpoint"
        );
        assert_eq!(res.created_at, 3000);

        let update = res_rows.iter().find_map(|r| {
            if let SocialEventRow::SptReservationPoolUpdate {
                total_reserved,
                status,
                required_threshold,
                ..
            } = r
            {
                Some((total_reserved, status, required_threshold))
            } else {
                None
            }
        });
        assert_eq!(
            update,
            Some((
                &100i64,
                &Some(RESERVATION_POOL_STATUS_ACTIVE.to_string()),
                &None
            )),
            "reservation-created pool update"
        );
    }

    #[test]
    fn token_pool_created_sets_supply_price_profile_and_launch_holdings_marker() {
        let data = json!({
            "id": "0xpool1",
            "token_type": 1u64,
            "owner": "0xowner",
            "associated_id": "0xprof",
            "symbol": "S",
            "name": "N",
            "base_price": 5i64,
            "quadratic_coefficient": 1i64,
            "circulating_supply": 100u64,
            "total_reserved_at_launch": 1000u64,
        });
        let rows = handle_spt_event("TokenPoolCreatedEvent", &data, "tx:0", 0, 5000).expect("rows");
        let pool = rows
            .iter()
            .find_map(|r| {
                if let SocialEventRow::SptPool(p) = r {
                    Some(p)
                } else {
                    None
                }
            })
            .expect("SptPool");
        assert_eq!(pool.circulating_supply, 100);
        let ph = rows
            .iter()
            .find_map(|r| {
                if let SocialEventRow::SptPriceHistory(h) = r {
                    Some(h)
                } else {
                    None
                }
            })
            .expect("SptPriceHistory");
        assert_eq!(ph.circulating_supply, 100);
        assert!(
            rows.iter()
                .any(|r| matches!(r, SocialEventRow::ProfileUpdate(up) if up.social_proof_token_address == Some(Some("0xpool1".to_string())))),
            "profile token sets social_proof_token_address"
        );
        let launch = rows.iter().find_map(|r| {
            if let SocialEventRow::SptLaunchHoldingsFromReservations {
                circulating_supply,
                total_reserved_at_launch,
                ..
            } = r
            {
                Some((*circulating_supply, *total_reserved_at_launch))
            } else {
                None
            }
        })
        .expect("SptLaunchHoldingsFromReservations");
        assert_eq!(launch, (100, 1000));
    }

    #[test]
    fn token_pool_created_post_type_has_no_profile_update() {
        let data = json!({
            "id": "0xpool2",
            "token_type": 2u64,
            "owner": "0xowner",
            "associated_id": "0xpost",
            "symbol": "",
            "name": "",
            "base_price": 0i64,
            "quadratic_coefficient": 0i64,
            "circulating_supply": 10u64,
            "total_reserved_at_launch": 100u64,
        });
        let rows = handle_spt_event("TokenPoolCreatedEvent", &data, "tx:0", 0, 5000).expect("rows");
        assert!(
            !rows
                .iter()
                .any(|r| matches!(r, SocialEventRow::ProfileUpdate(_))),
            "post token must not emit ProfileUpdate"
        );
        assert!(
            rows
                .iter()
                .any(|r| matches!(r, SocialEventRow::SptLaunchHoldingsFromReservations { .. })),
            "launch row when circulating_supply > 0"
        );
    }

    #[test]
    fn token_pool_created_without_supply_skips_launch_row() {
        let data = json!({
            "id": "0xpool1",
            "token_type": 2u64,
            "owner": "0xowner",
            "associated_id": "0xpost",
            "symbol": "",
            "name": "",
            "base_price": 0i64,
            "quadratic_coefficient": 0i64,
        });
        let rows = handle_spt_event("TokenPoolCreatedEvent", &data, "tx:0", 0, 5000).expect("rows");
        assert!(
            !rows
                .iter()
                .any(|r| matches!(r, SocialEventRow::SptLaunchHoldingsFromReservations { .. })),
            "legacy JSON omits circulating_supply → no launch holdings row"
        );
        let pool = rows
            .iter()
            .find_map(|r| {
                if let SocialEventRow::SptPool(p) = r {
                    Some(p)
                } else {
                    None
                }
            })
            .expect("SptPool");
        assert_eq!(pool.circulating_supply, 0);
    }

    #[test]
    fn config_updated_event_does_not_set_trading_enabled_payload() {
        let data = json!({
            "updated_by": "0xadmin",
            "total_fee_bps": 100i64,
            "trading_creator_fee_bps": 30i64,
            "trading_platform_fee_bps": 30i64,
            "trading_treasury_fee_bps": 40i64,
            "reservation_total_fee_bps": 100i64,
            "reservation_creator_fee_bps": 30i64,
            "reservation_platform_fee_bps": 30i64,
            "reservation_treasury_fee_bps": 40i64,
            "base_price": 1i64,
            "quadratic_coefficient": 1i64,
            "max_hold_percent_bps": 1000i64,
            "post_threshold": 1i64,
            "profile_threshold": 1i64,
            "max_individual_reservation_bps": 100i64,
            "max_reservers_per_pool": 100i64,
        });
        let rows =
            handle_spt_event("ConfigUpdatedEvent", &data, "tx:0", 0, 1000).expect("rows");
        let cfg = rows.iter().find_map(|r| {
            if let SocialEventRow::SptExchangeConfig(c) = r {
                Some(c)
            } else {
                None
            }
        });
        let c: &NewSptExchangeConfig = cfg.expect("exchange config");
        assert_eq!(c.trading_enabled, None);
        assert!(!c.apply_trading_enabled_only);
    }

    #[test]
    fn emergency_kill_switch_sets_explicit_trading_enabled_on_exchange_config() {
        let data = json!({
            "admin": "0xadmin",
            "trading_enabled": true,
            "timestamp": 1u64,
            "reason": "",
        });
        let rows =
            handle_spt_event("EmergencyKillSwitchEvent", &data, "tx:0:e", 0, 1000).expect("rows");
        let cfg = rows.iter().find_map(|r| {
            if let SocialEventRow::SptExchangeConfig(c) = r {
                Some(c)
            } else {
                None
            }
        });
        let c: &NewSptExchangeConfig = cfg.expect("exchange config");
        assert_eq!(c.trading_enabled, Some(true));
        assert!(c.apply_trading_enabled_only);
    }
}
