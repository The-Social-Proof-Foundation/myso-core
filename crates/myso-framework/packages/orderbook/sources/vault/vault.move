// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

/// The vault holds all of the assets for this pool. At the end of all
/// transaction processing, the vault is used to settle the balances for the user.
module orderbook::vault;

use orderbook::{balance_manager::{TradeProof, BalanceManager}, balances::Balances};
use std::type_name::{Self, TypeName};
use myso::{balance::{Self, Balance}, coin::Coin, event};
use myso::myso::MYSO;

// === Errors ===
const ENotEnoughBaseForLoan: u64 = 1;
const ENotEnoughQuoteForLoan: u64 = 2;
const EInvalidLoanQuantity: u64 = 3;
const EIncorrectLoanPool: u64 = 4;
const EIncorrectTypeReturned: u64 = 5;
const EIncorrectQuantityReturned: u64 = 6;
const ENoBalanceToSettle: u64 = 7;
const EHasOwedBalances: u64 = 8;

// === Structs ===
public struct Vault<phantom BaseAsset, phantom QuoteAsset> has store {
    base_balance: Balance<BaseAsset>,
    quote_balance: Balance<QuoteAsset>,
    myso_balance: Balance<MYSO>,
}

public struct FlashLoan {
    pool_id: ID,
    borrow_quantity: u64,
    type_name: TypeName,
}

public struct FlashLoanBorrowed has copy, drop {
    pool_id: ID,
    borrow_quantity: u64,
    type_name: TypeName,
}

// === Public-Package Functions ===
public(package) fun balances<BaseAsset, QuoteAsset>(
    self: &Vault<BaseAsset, QuoteAsset>,
): (u64, u64, u64) {
    (self.base_balance.value(), self.quote_balance.value(), self.myso_balance.value())
}

public(package) fun empty<BaseAsset, QuoteAsset>(): Vault<BaseAsset, QuoteAsset> {
    Vault {
        base_balance: balance::zero(),
        quote_balance: balance::zero(),
        myso_balance: balance::zero(),
    }
}

/// Transfer any settled amounts for the `balance_manager`.
public(package) fun settle_balance_manager<BaseAsset, QuoteAsset>(
    self: &mut Vault<BaseAsset, QuoteAsset>,
    balances_out: Balances,
    balances_in: Balances,
    balance_manager: &mut BalanceManager,
    trade_proof: &TradeProof,
) {
    balance_manager.validate_proof(trade_proof);
    if (balances_out.base() > balances_in.base()) {
        let balance = self.base_balance.split(balances_out.base() - balances_in.base());
        balance_manager.deposit_with_proof(trade_proof, balance);
    };
    if (balances_out.quote() > balances_in.quote()) {
        let balance = self.quote_balance.split(balances_out.quote() - balances_in.quote());
        balance_manager.deposit_with_proof(trade_proof, balance);
    };
    if (balances_out.myso() > balances_in.myso()) {
        let balance = self.myso_balance.split(balances_out.myso() - balances_in.myso());
        balance_manager.deposit_with_proof(trade_proof, balance);
    };
    if (balances_in.base() > balances_out.base()) {
        let balance = balance_manager.withdraw_with_proof(
            trade_proof,
            balances_in.base() - balances_out.base(),
            false,
        );
        self.base_balance.join(balance);
    };
    if (balances_in.quote() > balances_out.quote()) {
        let balance = balance_manager.withdraw_with_proof(
            trade_proof,
            balances_in.quote() - balances_out.quote(),
            false,
        );
        self.quote_balance.join(balance);
    };
    if (balances_in.myso() > balances_out.myso()) {
        let balance = balance_manager.withdraw_with_proof(
            trade_proof,
            balances_in.myso() - balances_out.myso(),
            false,
        );
        self.myso_balance.join(balance);
    };
}

/// Transfer any settled amounts for the `balance_manager`.
public(package) fun settle_balance_manager_permissionless<BaseAsset, QuoteAsset>(
    self: &mut Vault<BaseAsset, QuoteAsset>,
    balances_out: Balances,
    balances_in: Balances,
    balance_manager: &mut BalanceManager,
) {
    assert!(
        balances_in.base() == 0 && balances_in.quote() == 0 && balances_in.myso() == 0,
        EHasOwedBalances,
    );
    let has_settled_balances =
        balances_out.base() > 0
        || balances_out.quote() > 0
        || balances_out.myso() > 0;
    assert!(has_settled_balances, ENoBalanceToSettle);

    if (balances_out.base() > 0) {
        let balance = self.base_balance.split(balances_out.base());
        balance_manager.deposit_permissionless(balance);
    };
    if (balances_out.quote() > 0) {
        let balance = self.quote_balance.split(balances_out.quote());
        balance_manager.deposit_permissionless(balance);
    };
    if (balances_out.myso() > 0) {
        let balance = self.myso_balance.split(balances_out.myso());
        balance_manager.deposit_permissionless(balance);
    };
}

public(package) fun withdraw_myso_to_burn<BaseAsset, QuoteAsset>(
    self: &mut Vault<BaseAsset, QuoteAsset>,
    amount_to_burn: u64,
): Balance<MYSO> {
    self.myso_balance.split(amount_to_burn)
}

public(package) fun borrow_flashloan_base<BaseAsset, QuoteAsset>(
    self: &mut Vault<BaseAsset, QuoteAsset>,
    pool_id: ID,
    borrow_quantity: u64,
    ctx: &mut TxContext,
): (Coin<BaseAsset>, FlashLoan) {
    assert!(borrow_quantity > 0, EInvalidLoanQuantity);
    assert!(self.base_balance.value() >= borrow_quantity, ENotEnoughBaseForLoan);
    let borrow_type_name = type_name::with_defining_ids<BaseAsset>();
    let borrow: Coin<BaseAsset> = self.base_balance.split(borrow_quantity).into_coin(ctx);

    let flash_loan = FlashLoan {
        pool_id,
        borrow_quantity,
        type_name: borrow_type_name,
    };

    event::emit(FlashLoanBorrowed {
        pool_id,
        borrow_quantity,
        type_name: borrow_type_name,
    });

    (borrow, flash_loan)
}

public(package) fun borrow_flashloan_quote<BaseAsset, QuoteAsset>(
    self: &mut Vault<BaseAsset, QuoteAsset>,
    pool_id: ID,
    borrow_quantity: u64,
    ctx: &mut TxContext,
): (Coin<QuoteAsset>, FlashLoan) {
    assert!(borrow_quantity > 0, EInvalidLoanQuantity);
    assert!(self.quote_balance.value() >= borrow_quantity, ENotEnoughQuoteForLoan);
    let borrow_type_name = type_name::with_defining_ids<QuoteAsset>();
    let borrow: Coin<QuoteAsset> = self.quote_balance.split(borrow_quantity).into_coin(ctx);

    let flash_loan = FlashLoan {
        pool_id,
        borrow_quantity,
        type_name: borrow_type_name,
    };

    event::emit(FlashLoanBorrowed {
        pool_id,
        borrow_quantity,
        type_name: borrow_type_name,
    });

    (borrow, flash_loan)
}

public(package) fun return_flashloan_base<BaseAsset, QuoteAsset>(
    self: &mut Vault<BaseAsset, QuoteAsset>,
    pool_id: ID,
    coin: Coin<BaseAsset>,
    flash_loan: FlashLoan,
) {
    assert!(pool_id == flash_loan.pool_id, EIncorrectLoanPool);
    assert!(
        type_name::with_defining_ids<BaseAsset>() == flash_loan.type_name,
        EIncorrectTypeReturned,
    );
    assert!(coin.value() == flash_loan.borrow_quantity, EIncorrectQuantityReturned);

    self.base_balance.join(coin.into_balance<BaseAsset>());

    let FlashLoan {
        pool_id: _,
        borrow_quantity: _,
        type_name: _,
    } = flash_loan;
}

public(package) fun return_flashloan_quote<BaseAsset, QuoteAsset>(
    self: &mut Vault<BaseAsset, QuoteAsset>,
    pool_id: ID,
    coin: Coin<QuoteAsset>,
    flash_loan: FlashLoan,
) {
    assert!(pool_id == flash_loan.pool_id, EIncorrectLoanPool);
    assert!(
        type_name::with_defining_ids<QuoteAsset>() == flash_loan.type_name,
        EIncorrectTypeReturned,
    );
    assert!(coin.value() == flash_loan.borrow_quantity, EIncorrectQuantityReturned);

    self.quote_balance.join(coin.into_balance<QuoteAsset>());

    let FlashLoan {
        pool_id: _,
        borrow_quantity: _,
        type_name: _,
    } = flash_loan;
}
