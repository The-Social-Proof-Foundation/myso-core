// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

module bridge::treasury;

use std::ascii::{Self, String};
use std::type_name::{Self, TypeName};
use myso::address;
use myso::balance::{Self, Balance};
use myso::bag::{Self, Bag};
use myso::coin::{Self, Coin, TreasuryCap, CoinMetadata};
use myso::event;
use myso::hex;
use myso::myso::MYSO;
use myso::object_bag::{Self, ObjectBag};
use myso::package::{Self, UpgradeCap};
use myso::vec_map::{Self, VecMap};

const EUnsupportedTokenType: u64 = 1;
const EInvalidUpgradeCap: u64 = 2;
const ETokenSupplyNonZero: u64 = 3;
const EInvalidNotionalValue: u64 = 4;
const ENativeBridgeAlreadyInitialized: u64 = 5;
const EInvalidBootstrapAmount: u64 = 6;
const ENativeBridgeNotInitialized: u64 = 7;
const EInsufficientNativeEscrow: u64 = 8;
const EInsufficientRailReserve: u64 = 9;
const ETokenRequiresRailId: u64 = 10;
const ETokenIdAlreadyExists: u64 = 11;

const MIST_PER_WHOLE_MYSO: u64 = 1_000_000_000;
const BOOTSTRAP_NATIVE_MYSO_MIST: u64 = 50_000_000_000_000_000;
/// Initial USD notional for limiter ($1.00 at 8 decimal places), same scale as bridge limiter.
const NATIVE_MYSO_INITIAL_NOTIONAL_USD: u64 = 100_000_000;

#[test_only]
const USD_VALUE_MULTIPLIER: u64 = 100000000; // 8 DP accuracy

//////////////////////////////////////////////////////
// Types
//

public struct BridgeTreasury has store {
    // token treasuries, values are TreasuryCaps for native bridge V1.
    treasuries: ObjectBag,
    supported_tokens: VecMap<TypeName, BridgeTokenMetadata>,
    // Mapping token id to type name (many ids may share one type)
    id_token_type_map: VecMap<u8, TypeName>,
    // Bag for storing potential new token waiting to be approved
    waiting_room: Bag,
    /// Escrow for native MYSO (bootstrap + send_myso_token locks); claims release from here.
    native_myso_escrow: Balance<MYSO>,
    native_bridge_initialized: bool,
    /// Type -> rail ids. Length 1 for 1:1 wrappers; length > 1 for shared rails (myUSD).
    type_to_ids: VecMap<TypeName, vector<u8>>,
    /// Outstanding minted amount per rail id (EVM vault backing).
    rail_reserve: VecMap<u8, u64>,
}

public struct BridgeTokenMetadata has copy, drop, store {
    id: u8,
    decimal_multiplier: u64,
    notional_value: u64,
    native_token: bool,
}

public struct ForeignTokenRegistration has store {
    type_name: TypeName,
    uc: UpgradeCap,
    decimal: u8,
}

public struct UpdateTokenPriceEvent has copy, drop {
    token_id: u8,
    new_price: u64,
}

public struct NewTokenEvent has copy, drop {
    token_id: u8,
    type_name: TypeName,
    native_token: bool,
    decimal_multiplier: u64,
    notional_value: u64,
}

public struct TokenRegistrationEvent has copy, drop {
    type_name: TypeName,
    decimal: u8,
    native_token: bool,
}

public fun token_id<T>(self: &BridgeTreasury): u8 {
    let ids = self.type_ids<T>();
    assert!(ids.length() == 1, ETokenRequiresRailId);
    ids[0]
}

public fun type_ids<T>(self: &BridgeTreasury): vector<u8> {
    let coin_type = type_name::with_defining_ids<T>();
    assert!(self.type_to_ids.contains(&coin_type), EUnsupportedTokenType);
    *self.type_to_ids.get(&coin_type)
}

public fun type_matches_token_id<T>(self: &BridgeTreasury, token_id: u8): bool {
    if (!self.id_token_type_map.contains(&token_id)) {
        return false
    };
    self.id_token_type_map[&token_id] == type_name::with_defining_ids<T>()
}

public fun rail_reserve(self: &BridgeTreasury, token_id: u8): u64 {
    if (!self.rail_reserve.contains(&token_id)) {
        return 0
    };
    self.rail_reserve[&token_id]
}

public fun decimal_multiplier<T>(self: &BridgeTreasury): u64 {
    let metadata = self.get_token_metadata<T>();
    metadata.decimal_multiplier
}

public fun notional_value<T>(self: &BridgeTreasury): u64 {
    let metadata = self.get_token_metadata<T>();
    metadata.notional_value
}

public fun decimal_multiplier_by_id(self: &BridgeTreasury, token_id: u8): u64 {
    let type_name = self.id_token_type_map.try_get(&token_id);
    assert!(type_name.is_some(), EUnsupportedTokenType);
    let type_name = type_name.destroy_some();
    let metadata = self.supported_tokens.try_get(&type_name);
    assert!(metadata.is_some(), EUnsupportedTokenType);
    metadata.destroy_some().decimal_multiplier
}

public fun notional_value_by_id(self: &BridgeTreasury, token_id: u8): u64 {
    let type_name = self.id_token_type_map.try_get(&token_id);
    assert!(type_name.is_some(), EUnsupportedTokenType);
    let type_name = type_name.destroy_some();
    let metadata = self.supported_tokens.try_get(&type_name);
    assert!(metadata.is_some(), EUnsupportedTokenType);
    metadata.destroy_some().notional_value
}

//////////////////////////////////////////////////////
// Internal functions
//

public(package) fun register_foreign_token<T>(
    self: &mut BridgeTreasury,
    tc: TreasuryCap<T>,
    uc: UpgradeCap,
    metadata: &CoinMetadata<T>,
) {
    // Make sure TreasuryCap has not been minted before.
    assert!(coin::total_supply(&tc) == 0, ETokenSupplyNonZero);
    let type_name = type_name::with_defining_ids<T>();
    let address_bytes = hex::decode(ascii::into_bytes(type_name::address_string(&type_name)));
    let coin_address = address::from_bytes(address_bytes);
    // Make sure upgrade cap is for the Coin package
    // FIXME: add test
    assert!(
        object::id_to_address(&package::upgrade_package(&uc)) == coin_address,
        EInvalidUpgradeCap,
    );
    let registration = ForeignTokenRegistration {
        type_name,
        uc,
        decimal: coin::get_decimals(metadata),
    };
    self.waiting_room.add(type_name::into_string(type_name), registration);
    self.treasuries.add(type_name, tc);

    event::emit(TokenRegistrationEvent {
        type_name,
        decimal: coin::get_decimals(metadata),
        native_token: false,
    });
}

public(package) fun add_new_token(
    self: &mut BridgeTreasury,
    token_name: String,
    token_id: u8,
    native_token: bool,
    notional_value: u64,
) {
    if (native_token) {
        return
    };
    assert!(notional_value > 0, EInvalidNotionalValue);
    assert!(!self.id_token_type_map.contains(&token_id), ETokenIdAlreadyExists);

    if (self.waiting_room.contains(token_name)) {
        let ForeignTokenRegistration {
            type_name,
            uc,
            decimal,
        } = self.waiting_room.remove<String, ForeignTokenRegistration>(token_name);
        let decimal_multiplier = 10u64.pow(decimal);
        if (!self.supported_tokens.contains(&type_name)) {
            self
                .supported_tokens
                .insert(
                    type_name,
                    BridgeTokenMetadata {
                        id: token_id,
                        decimal_multiplier,
                        notional_value,
                        native_token,
                    },
                );
            self.type_to_ids.insert(type_name, vector[token_id]);
        } else {
            self.type_to_ids.get_mut(&type_name).push_back(token_id);
            let meta = self.supported_tokens.get_mut(&type_name);
            meta.notional_value = notional_value;
        };
        self.id_token_type_map.insert(token_id, type_name);
        self.rail_reserve.insert(token_id, 0);
        transfer::public_freeze_object(uc);
        event::emit(NewTokenEvent {
            token_id,
            type_name,
            native_token,
            decimal_multiplier,
            notional_value,
        });
        return
    };

    let type_name = self.find_supported_type(token_name);
    assert!(type_name.is_some(), EUnsupportedTokenType);
    let type_name = type_name.destroy_some();
    self.type_to_ids.get_mut(&type_name).push_back(token_id);
    self.id_token_type_map.insert(token_id, type_name);
    self.rail_reserve.insert(token_id, 0);
    let meta = self.supported_tokens.get_mut(&type_name);
    meta.notional_value = notional_value;
    event::emit(NewTokenEvent {
        token_id,
        type_name,
        native_token,
        decimal_multiplier: meta.decimal_multiplier,
        notional_value,
    });
}

public(package) fun create(ctx: &mut TxContext): BridgeTreasury {
    BridgeTreasury {
        treasuries: object_bag::new(ctx),
        supported_tokens: vec_map::empty(),
        id_token_type_map: vec_map::empty(),
        waiting_room: bag::new(ctx),
        native_myso_escrow: balance::zero(),
        native_bridge_initialized: false,
        type_to_ids: vec_map::empty(),
        rail_reserve: vec_map::empty(),
    }
}

/// One-time bootstrap: lock exactly `BOOTSTRAP_NATIVE_MYSO_MIST`, register MYSO as token id 0 for limiter metadata.
public(package) fun bootstrap_native_myso_once(self: &mut BridgeTreasury, coin: Coin<MYSO>) {
    assert!(!self.native_bridge_initialized, ENativeBridgeAlreadyInitialized);
    assert!(coin::value(&coin) == BOOTSTRAP_NATIVE_MYSO_MIST, EInvalidBootstrapAmount);
    balance::join(&mut self.native_myso_escrow, coin::into_balance(coin));
    self.native_bridge_initialized = true;

    let type_m = type_name::with_defining_ids<MYSO>();
    assert!(!self.supported_tokens.contains(&type_m), EUnsupportedTokenType);
    assert!(!self.id_token_type_map.contains(&0), EUnsupportedTokenType);

    let decimal_multiplier = MIST_PER_WHOLE_MYSO;
    self
        .supported_tokens
        .insert(
            type_m,
            BridgeTokenMetadata {
                id: 0,
                decimal_multiplier,
                notional_value: NATIVE_MYSO_INITIAL_NOTIONAL_USD,
                native_token: true,
            },
        );
    self.id_token_type_map.insert(0, type_m);
    self.type_to_ids.insert(type_m, vector[0]);

    event::emit(NewTokenEvent {
        token_id: 0,
        type_name: type_m,
        native_token: true,
        decimal_multiplier,
        notional_value: NATIVE_MYSO_INITIAL_NOTIONAL_USD,
    });
}

/// Lock MYSO into bridge escrow for `send_myso_token` (not TreasuryCap burn).
public(package) fun lock_native_myso(self: &mut BridgeTreasury, coin: Coin<MYSO>) {
    assert!(self.native_bridge_initialized, ENativeBridgeNotInitialized);
    balance::join(&mut self.native_myso_escrow, coin::into_balance(coin));
}

/// Release MYSO from escrow for a completed inbound transfer claim.
public(package) fun unlock_native_myso(
    self: &mut BridgeTreasury,
    amount: u64,
    ctx: &mut TxContext,
): Coin<MYSO> {
    assert!(self.native_bridge_initialized, ENativeBridgeNotInitialized);
    assert!(balance::value(&self.native_myso_escrow) >= amount, EInsufficientNativeEscrow);
    let b = balance::split(&mut self.native_myso_escrow, amount);
    coin::from_balance(b, ctx)
}

public(package) fun native_myso_locked_amount(self: &BridgeTreasury): u64 {
    balance::value(&self.native_myso_escrow)
}

public(package) fun native_bridge_ready(self: &BridgeTreasury): bool {
    self.native_bridge_initialized
}

public(package) fun burn<T>(self: &mut BridgeTreasury, token: Coin<T>) {
    let treasury = &mut self.treasuries[type_name::with_defining_ids<T>()];
    coin::burn(treasury, token);
}

public(package) fun mint<T>(self: &mut BridgeTreasury, amount: u64, ctx: &mut TxContext): Coin<T> {
    let treasury = &mut self.treasuries[type_name::with_defining_ids<T>()];
    coin::mint(treasury, amount, ctx)
}

public(package) fun credit_rail(self: &mut BridgeTreasury, token_id: u8, amount: u64) {
    if (!self.rail_reserve.contains(&token_id)) {
        self.rail_reserve.insert(token_id, 0);
    };
    let reserved = self.rail_reserve.get_mut(&token_id);
    *reserved = *reserved + amount;
}

public(package) fun debit_rail(self: &mut BridgeTreasury, token_id: u8, amount: u64) {
    assert!(self.rail_reserve.contains(&token_id), EInsufficientRailReserve);
    let reserved = self.rail_reserve.get_mut(&token_id);
    assert!(*reserved >= amount, EInsufficientRailReserve);
    *reserved = *reserved - amount;
}

public(package) fun update_asset_notional_price(
    self: &mut BridgeTreasury,
    token_id: u8,
    new_usd_price: u64,
) {
    let type_name = self.id_token_type_map.try_get(&token_id);
    assert!(type_name.is_some(), EUnsupportedTokenType);
    assert!(new_usd_price > 0, EInvalidNotionalValue);
    let type_name = type_name.destroy_some();
    let metadata = self.supported_tokens.get_mut(&type_name);
    metadata.notional_value = new_usd_price;

    event::emit(UpdateTokenPriceEvent {
        token_id,
        new_price: new_usd_price,
    })
}

fun get_token_metadata<T>(self: &BridgeTreasury): BridgeTokenMetadata {
    let coin_type = type_name::with_defining_ids<T>();
    let metadata = self.supported_tokens.try_get(&coin_type);
    assert!(metadata.is_some(), EUnsupportedTokenType);
    metadata.destroy_some()
}

fun find_supported_type(self: &BridgeTreasury, token_name: String): Option<TypeName> {
    let keys = self.supported_tokens.keys();
    let mut i = 0;
    while (i < keys.length()) {
        let tn = keys[i];
        if (type_name::into_string(tn) == token_name) {
            return option::some(tn)
        };
        i = i + 1;
    };
    option::none()
}

//////////////////////////////////////////////////////
// Test functions
//

#[test_only]
public struct ETH has drop {}
#[test_only]
public struct BTC has drop {}
#[test_only]
public struct USDT has drop {}
#[test_only]
public struct USDC has drop {}

#[test_only]
public fun new_for_testing(ctx: &mut TxContext): BridgeTreasury {
    create(ctx)
}

#[test_only]
public fun mock_for_test(ctx: &mut TxContext): BridgeTreasury {
    let mut treasury = new_for_testing(ctx);
    treasury.setup_for_testing();
    treasury
}

#[test_only]
public fun setup_for_testing(treasury: &mut BridgeTreasury) {
    treasury
        .supported_tokens
        .insert(
            type_name::with_defining_ids<BTC>(),
            BridgeTokenMetadata {
                id: 1,
                decimal_multiplier: 100_000_000,
                notional_value: 50_000 * USD_VALUE_MULTIPLIER,
                native_token: false,
            },
        );
    treasury
        .supported_tokens
        .insert(
            type_name::with_defining_ids<ETH>(),
            BridgeTokenMetadata {
                id: 2,
                decimal_multiplier: 100_000_000,
                notional_value: 3_000 * USD_VALUE_MULTIPLIER,
                native_token: false,
            },
        );
    treasury
        .supported_tokens
        .insert(
            type_name::with_defining_ids<USDC>(),
            BridgeTokenMetadata {
                id: 3,
                decimal_multiplier: 1_000_000,
                notional_value: USD_VALUE_MULTIPLIER,
                native_token: false,
            },
        );
    treasury
        .supported_tokens
        .insert(
            type_name::with_defining_ids<USDT>(),
            BridgeTokenMetadata {
                id: 4,
                decimal_multiplier: 1_000_000,
                notional_value: USD_VALUE_MULTIPLIER,
                native_token: false,
            },
        );

    let btc_t = type_name::with_defining_ids<BTC>();
    let eth_t = type_name::with_defining_ids<ETH>();
    let usdc_t = type_name::with_defining_ids<USDC>();
    let usdt_t = type_name::with_defining_ids<USDT>();
    treasury.id_token_type_map.insert(1, btc_t);
    treasury.id_token_type_map.insert(2, eth_t);
    treasury.id_token_type_map.insert(3, usdc_t);
    treasury.id_token_type_map.insert(4, usdt_t);
    treasury.type_to_ids.insert(btc_t, vector[1]);
    treasury.type_to_ids.insert(eth_t, vector[2]);
    treasury.type_to_ids.insert(usdc_t, vector[3]);
    treasury.type_to_ids.insert(usdt_t, vector[4]);
    treasury.rail_reserve.insert(1, 0);
    treasury.rail_reserve.insert(2, 0);
    treasury.rail_reserve.insert(3, 0);
    treasury.rail_reserve.insert(4, 0);
}

#[test_only]
public fun waiting_room(treasury: &BridgeTreasury): &Bag {
    &treasury.waiting_room
}

#[test_only]
public fun treasuries(treasury: &BridgeTreasury): &ObjectBag {
    &treasury.treasuries
}

#[test_only]
public fun unwrap_update_event(event: UpdateTokenPriceEvent): (u8, u64) {
    (event.token_id, event.new_price)
}

#[test_only]
public fun unwrap_new_token_event(event: NewTokenEvent): (u8, TypeName, bool, u64, u64) {
    (
        event.token_id,
        event.type_name,
        event.native_token,
        event.decimal_multiplier,
        event.notional_value,
    )
}

#[test_only]
public fun unwrap_registration_event(event: TokenRegistrationEvent): (TypeName, u8, bool) {
    (event.type_name, event.decimal, event.native_token)
}
