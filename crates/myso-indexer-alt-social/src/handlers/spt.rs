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

fn json_to_i64(v: &serde_json::Value) -> i64 {
    v.as_i64()
        .or_else(|| v.as_u64().and_then(|u| u.try_into().ok()))
        .unwrap_or(0)
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
            process_reservation_created_event(data, &transaction_id, ts, now)
        }
        "ReservationWithdrawnEvent" => {
            process_reservation_withdrawn_event(data, &transaction_id, ts, now)
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

    let pool = NewSptPool {
        pool_id: id.clone(),
        token_type,
        owner: owner.clone(),
        associated_id: associated_id.clone(),
        symbol,
        name,
        circulating_supply: 0,
        base_price,
        quadratic_coefficient,
        created_at: ts,
        time: now,
        transaction_id: transaction_id.to_string(),
    };

    let price_history = NewSptPriceHistory {
        pool_id: id,
        price: base_price,
        circulating_supply: 0,
        time: now,
        transaction_id: transaction_id.to_string(),
    };

    Some(vec![
        SocialEventRow::SptPool(pool),
        SocialEventRow::SptPriceHistory(price_history),
    ])
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
            facebook_username: None,
            reddit_username: None,
            github_username: None,
            instagram_username: None,
            linkedin_username: None,
            twitch_username: None,
            min_offer_amount: None,
            username: None,
            selected_badge_id: None,
            selected_ecosystem_badge_id: None,
            paid_messaging_enabled: None,
            paid_messaging_min_cost: None,
            reservation_pool_address: Some(Some(pool_object_id)),
        };
        rows.push(SocialEventRow::ProfileUpdate(profile_update));
    }

    Some(rows)
}

fn process_reservation_created_event(
    data: &serde_json::Value,
    transaction_id: &str,
    _ts: i64,
    now: chrono::DateTime<chrono::Utc>,
) -> Option<Vec<SocialEventRow>> {
    let associated_id = json_str(data.get("associated_id")?)?;
    let reserver = json_str(data.get("reserver")?)?;
    let amount = json_to_i64(data.get("amount")?);
    let total_reserved = json_to_i64(data.get("total_reserved")?);
    let threshold_met = data.get("threshold_met")?.as_bool().unwrap_or(false);
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

    let reservation = NewSptReservation {
        pool_id: pool_id.clone(),
        reserver_address: reserver,
        amount,
        reserved_at,
        fee_amount,
        creator_fee,
        platform_fee,
        treasury_fee,
        time: now,
        transaction_id: transaction_id.to_string(),
    };

    Some(vec![
        SocialEventRow::SptReservation(reservation),
        SocialEventRow::SptReservationPoolUpdate {
            pool_id,
            associated_id: associated_id.clone(),
            total_reserved,
            status: Some(status),
        },
    ])
}

fn process_reservation_withdrawn_event(
    data: &serde_json::Value,
    transaction_id: &str,
    _ts: i64,
    now: chrono::DateTime<chrono::Utc>,
) -> Option<Vec<SocialEventRow>> {
    let associated_id = json_str(data.get("associated_id")?)?;
    let reserver = json_str(data.get("reserver")?)?;
    let total_reserved = json_to_i64(data.get("total_reserved")?);
    let withdrawn_at = json_to_i64(data.get("withdrawn_at")?);
    let fee_amount = data.get("fee_amount").map(json_to_i64);
    let creator_fee = data.get("creator_fee").map(json_to_i64);
    let platform_fee = data.get("platform_fee").map(json_to_i64);
    let treasury_fee = data.get("treasury_fee").map(json_to_i64);

    let pool_id = format!("reservation_pool_{}", associated_id);

    let reservation = NewSptReservation {
        pool_id: pool_id.clone(),
        reserver_address: reserver,
        amount: 0,
        reserved_at: withdrawn_at,
        fee_amount,
        creator_fee,
        platform_fee,
        treasury_fee,
        time: now,
        transaction_id: transaction_id.to_string(),
    };

    Some(vec![
        SocialEventRow::SptReservation(reservation),
        SocialEventRow::SptReservationPoolUpdate {
            pool_id,
            associated_id: associated_id.clone(),
            total_reserved,
            status: None,
        },
    ])
}

fn process_threshold_met_event(
    data: &serde_json::Value,
    transaction_id: &str,
    ts: i64,
    now: chrono::DateTime<chrono::Utc>,
) -> Option<Vec<SocialEventRow>> {
    let associated_id = json_str(data.get("associated_id")?)?;
    let token_type = token_type_from_u8(data.get("token_type")?.as_u64()?)?;
    let owner = json_str(data.get("owner")?)?;
    let total_reserved = json_to_i64(data.get("total_reserved")?);
    let required_threshold = json_to_i64(data.get("required_threshold")?);

    let pool_id = format!("reservation_pool_{}", associated_id);

    let pool = NewSptReservationPool {
        pool_id,
        associated_id,
        token_type,
        owner,
        total_reserved,
        required_threshold,
        status: RESERVATION_POOL_STATUS_THRESHOLD_MET.to_string(),
        created_at: ts,
        time: now,
        transaction_id: transaction_id.to_string(),
    };

    Some(vec![SocialEventRow::SptReservationPool(pool)])
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
        trading_enabled: true,
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
        trading_enabled,
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
