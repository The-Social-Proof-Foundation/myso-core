// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

/// MYSO price module. This module maintains the conversion rate
/// between MYSO and the base and quote assets.
module orderbook::myso_price;

use orderbook::{balances::{Self, Balances}, constants, math};
use myso::event;

// === Errors ===
const EDataPointRecentlyAdded: u64 = 1;
const ENoDataPoints: u64 = 2;

// === Constants ===
// Minimum of 1 minutes between data points
const MIN_DURATION_BETWEEN_DATA_POINTS_MS: u64 = 1000 * 60;
// Price points older than 1 day will be removed
const MAX_DATA_POINT_AGE_MS: u64 = 1000 * 60 * 60 * 24;
// Maximum number of data points to maintan
const MAX_DATA_POINTS: u64 = 100;

// === Structs ===
/// MYSO price point.
public struct Price has drop, store {
    conversion_rate: u64,
    timestamp: u64,
}

/// MYSO price point added event.
public struct PriceAdded has copy, drop {
    conversion_rate: u64,
    timestamp: u64,
    is_base_conversion: bool,
    reference_pool: ID,
    target_pool: ID,
}

/// MYSO price points used for trading fee calculations.
public struct MySoPrice has drop, store {
    base_prices: vector<Price>,
    cumulative_base: u64,
    quote_prices: vector<Price>,
    cumulative_quote: u64,
}

public struct OrderMySoPrice has copy, drop, store {
    asset_is_base: bool,
    myso_per_asset: u64,
}

// === Public-View Functions ===
public fun asset_is_base(self: &OrderMySoPrice): bool {
    self.asset_is_base
}

public fun myso_per_asset(self: &OrderMySoPrice): u64 {
    self.myso_per_asset
}

// === Public-Package Functions ===
public(package) fun empty(): MySoPrice {
    MySoPrice {
        base_prices: vector[],
        cumulative_base: 0,
        quote_prices: vector[],
        cumulative_quote: 0,
    }
}

public(package) fun new_order_myso_price(asset_is_base: bool, myso_per_asset: u64): OrderMySoPrice {
    OrderMySoPrice {
        asset_is_base: asset_is_base,
        myso_per_asset: myso_per_asset,
    }
}

public(package) fun get_order_myso_price(self: &MySoPrice, whitelisted: bool): OrderMySoPrice {
    let (asset_is_base, myso_per_asset) = self.calculate_order_myso_price(
        whitelisted,
    );

    new_order_myso_price(asset_is_base, myso_per_asset)
}

public(package) fun empty_myso_price(_self: &MySoPrice): OrderMySoPrice {
    new_order_myso_price(false, 0)
}

public(package) fun fee_quantity(
    self: &OrderMySoPrice,
    base_quantity: u64,
    quote_quantity: u64,
    is_bid: bool,
): Balances {
    let myso_quantity = if (self.asset_is_base) {
        math::mul(base_quantity, self.myso_per_asset)
    } else {
        math::mul(quote_quantity, self.myso_per_asset)
    };

    if (self.myso_per_asset > 0) {
        balances::new(0, 0, myso_quantity)
    } else if (is_bid) {
        balances::new(
            0,
            math::mul(
                quote_quantity,
                constants::fee_penalty_multiplier(),
            ),
            0,
        )
    } else {
        balances::new(
            math::mul(base_quantity, constants::fee_penalty_multiplier()),
            0,
            0,
        )
    }
}

public(package) fun myso_quantity_u128(
    self: &OrderMySoPrice,
    base_quantity: u128,
    quote_quantity: u128,
): u128 {
    if (self.asset_is_base) {
        math::mul_u128(base_quantity, self.myso_per_asset as u128)
    } else {
        math::mul_u128(quote_quantity, self.myso_per_asset as u128)
    }
}

/// Add a price point. If max data points are reached, the oldest data point is removed.
/// Remove all data points older than MAX_DATA_POINT_AGE_MS.
public(package) fun add_price_point(
    self: &mut MySoPrice,
    conversion_rate: u64,
    timestamp: u64,
    is_base_conversion: bool,
) {
    assert!(
        self.last_insert_timestamp(is_base_conversion) +
        MIN_DURATION_BETWEEN_DATA_POINTS_MS <
        timestamp,
        EDataPointRecentlyAdded,
    );
    let asset_prices = if (is_base_conversion) {
        &mut self.base_prices
    } else {
        &mut self.quote_prices
    };

    asset_prices.push_back(Price {
        timestamp: timestamp,
        conversion_rate: conversion_rate,
    });
    if (is_base_conversion) {
        self.cumulative_base = self.cumulative_base + conversion_rate;
        while (
            asset_prices.length() == MAX_DATA_POINTS + 1 ||
            asset_prices[0].timestamp + MAX_DATA_POINT_AGE_MS < timestamp
        ) {
            self.cumulative_base = self.cumulative_base - asset_prices[0].conversion_rate;
            asset_prices.remove(0);
        }
    } else {
        self.cumulative_quote = self.cumulative_quote + conversion_rate;
        while (
            asset_prices.length() == MAX_DATA_POINTS + 1 ||
            asset_prices[0].timestamp + MAX_DATA_POINT_AGE_MS < timestamp
        ) {
            self.cumulative_quote = self.cumulative_quote - asset_prices[0].conversion_rate;
            asset_prices.remove(0);
        }
    };
}

public(package) fun emit_myso_price_added(
    conversion_rate: u64,
    timestamp: u64,
    is_base_conversion: bool,
    reference_pool: ID,
    target_pool: ID,
) {
    event::emit(PriceAdded {
        conversion_rate,
        timestamp,
        is_base_conversion,
        reference_pool,
        target_pool,
    });
}

// === Private Functions ===
/// Returns the conversion rate of MYSO per asset token.
/// Quote will be used by default, if there are no quote data then base will be used
fun calculate_order_myso_price(self: &MySoPrice, whitelisted: bool): (bool, u64) {
    if (whitelisted) {
        return (false, 0) // no fees for whitelist
    };
    assert!(
        self.last_insert_timestamp(true) > 0 ||
        self.last_insert_timestamp(false) > 0,
        ENoDataPoints,
    );

    let is_base_conversion = self.last_insert_timestamp(false) == 0;

    let cumulative_asset = if (is_base_conversion) {
        self.cumulative_base
    } else {
        self.cumulative_quote
    };
    let asset_length = if (is_base_conversion) {
        self.base_prices.length()
    } else {
        self.quote_prices.length()
    };
    let myso_per_asset = cumulative_asset / asset_length;

    (is_base_conversion, myso_per_asset)
}

fun last_insert_timestamp(self: &MySoPrice, is_base_conversion: bool): u64 {
    let prices = if (is_base_conversion) {
        &self.base_prices
    } else {
        &self.quote_prices
    };
    if (prices.length() > 0) {
        prices[prices.length() - 1].timestamp
    } else {
        0
    }
}
