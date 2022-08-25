use crate::errors::TransactionError;
use crate::transaction::Transaction;
use crate::transaction::TransactionState;

use super::Account;
use super::FixedPrecision4;
use super::NormalAccountState;

#[test]
fn test_normal_account_state_deposit() {
    let mut state = NormalAccountState::default();

    // first deposit
    state.deposit(5, FixedPrecision4::from(32.0)).unwrap();
    assert_eq!(state.funds.available_funds, FixedPrecision4::from(32.0));
    assert_eq!(
        state.transactions.get(&5),
        Some(&Transaction::new(FixedPrecision4::from(32.0)))
    );

    // deposit with already existing transaction id
    match state.deposit(5, FixedPrecision4::from(15.0)) {
        Err(TransactionError::TransactionExists) => (),
        _ => assert!(false),
    }
    assert_eq!(
        state.transactions.get(&5),
        Some(&Transaction::new(FixedPrecision4::from(32.0)))
    );

    // normal addiction to already existing transaction
    state.deposit(17, FixedPrecision4::from(13.0)).unwrap();
    assert_eq!(state.funds.available_funds, FixedPrecision4::from(45.0));
    assert_eq!(
        state.transactions.get(&5),
        Some(&Transaction::new(FixedPrecision4::from(32.0)))
    );
    assert_eq!(
        state.transactions.get(&17),
        Some(&Transaction::new(FixedPrecision4::from(13.0)))
    );
}

#[test]
fn test_normal_account_state_withdrawal() {
    let mut state = NormalAccountState::default();

    // not enough funds
    match state.withdrawal(5, FixedPrecision4::from(32.0)) {
        Err(TransactionError::TooFewFunds) => (),
        _ => assert!(false),
    }
    assert_eq!(state.transactions.get(&5), None);

    state.deposit(13, FixedPrecision4::from(100.0)).unwrap();

    // normal withdrawal
    state.withdrawal(14, FixedPrecision4::from(42.0)).unwrap();
    assert_eq!(state.funds.available_funds, FixedPrecision4::from(58.0));
    assert_eq!(
        state.transactions.get(&13),
        Some(&Transaction::new(FixedPrecision4::from(100.0)))
    );

    assert_eq!(
        state.transactions.get(&14),
        Some(&Transaction::new(FixedPrecision4::from(42.0)))
    );
}

#[test]
fn test_normal_account_state_dispute() {
    let mut state = NormalAccountState::default();

    // no such transaction
    match state.dispute(5) {
        Err(TransactionError::NoSuchTransaction) => (),
        _ => assert!(false),
    }

    // normal dispute
    state.deposit(5, FixedPrecision4::from(100.0)).unwrap();
    state.deposit(6, FixedPrecision4::from(300.0)).unwrap();
    state.dispute(5).unwrap();

    assert_eq!(state.funds.available_funds, FixedPrecision4::from(300.0));
    assert_eq!(state.funds.held_funds, FixedPrecision4::from(100.0));
    assert_eq!(
        state.transactions.get(&6),
        Some(&Transaction::new(FixedPrecision4::from(300.0)))
    );
    let mut disputed_transaction = Transaction::new(FixedPrecision4::from(100.0));
    disputed_transaction.state = TransactionState::Dispute;
    assert_eq!(state.transactions.get(&5), Some(&disputed_transaction));

    // dispute the second time
    match state.dispute(5) {
        Err(TransactionError::AlreadyDisputed) => (),
        _ => assert!(false),
    }
}

#[test]
fn test_normal_account_state_resolve() {
    let mut state = NormalAccountState::default();

    // no such transaction
    match state.resolve(5) {
        Err(TransactionError::NoSuchTransaction) => (),
        _ => assert!(false),
    }

    // not disputed
    state.deposit(5, FixedPrecision4::from(100.0)).unwrap();
    match state.resolve(5) {
        Err(TransactionError::NotDisputed) => (),
        _ => assert!(false),
    }

    // normal resolve
    state.dispute(5).unwrap();
    state.resolve(5).unwrap();
    assert_eq!(state.funds.available_funds, FixedPrecision4::from(100.0));
    assert_eq!(state.funds.held_funds, FixedPrecision4::from(0.0));
    assert_eq!(
        state.transactions.get(&5),
        Some(&Transaction::new(FixedPrecision4::from(100.0)))
    )
}

#[test]
fn test_normal_account_state_chargeback() {
    let mut state = NormalAccountState::default();

    // no such transaction
    match state.chargeback(5) {
        Err(TransactionError::NoSuchTransaction) => (),
        _ => assert!(false),
    }

    // not disputed
    state.deposit(5, FixedPrecision4::from(100.0)).unwrap();
    state.deposit(6, FixedPrecision4::from(300.0)).unwrap();
    match state.resolve(5) {
        Err(TransactionError::NotDisputed) => (),
        _ => assert!(false),
    }

    // normal chargeback
    state.dispute(5).unwrap();
    state.chargeback(5).unwrap();
    assert_eq!(state.funds.available_funds, FixedPrecision4::from(300.0));
    assert_eq!(state.funds.held_funds, FixedPrecision4::from(0.0));
}

#[test]
fn test_account_chargeback() {
    let mut account = Account::Normal(NormalAccountState::default());

    // no such transaction
    match account.process_chargeback(5) {
        Err(TransactionError::NoSuchTransaction) => (),
        _ => assert!(false),
    }

    // not disputed
    account
        .process_deposit(5, FixedPrecision4::from(100.0))
        .unwrap();
    account
        .process_deposit(6, FixedPrecision4::from(300.0))
        .unwrap();
    match account.process_resolve(5) {
        Err(TransactionError::NotDisputed) => (),
        _ => assert!(false),
    }

    // normal chargeback
    account.process_dispute(5).unwrap();
    account.process_chargeback(5).unwrap();
    match account {
        Account::Frozen(account_funds) => {
            assert_eq!(account_funds.available_funds, FixedPrecision4::from(300.0));
            assert_eq!(account_funds.held_funds, FixedPrecision4::from(0.0));
        }
        _ => assert!(false),
    }
}
