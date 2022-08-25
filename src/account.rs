use std::{collections::HashMap, mem};

use serde::Serialize;

use crate::{errors::TransactionError, fixed_precision::FixedPrecision4, transaction::Transaction};

#[derive(Clone, Copy, Default)]
pub struct AccountFunds {
    available_funds: FixedPrecision4,
    held_funds: FixedPrecision4,
}

impl AccountFunds {
    fn total_funds(&self) -> FixedPrecision4 {
        self.available_funds + self.held_funds
    }
}

#[derive(Default)]
pub struct NormalAccountState {
    funds: AccountFunds,
    transactions: HashMap<u32, Transaction>,
}

impl NormalAccountState {
    fn deposit(&mut self, tx: u32, amount: FixedPrecision4) -> Result<(), TransactionError> {
        match self.transactions.get_mut(&tx) {
            None => {
                self.funds.available_funds += amount;
                self.transactions.insert(tx, Transaction::new(amount));
                Ok(())
            }
            _ => Err(TransactionError::TransactionExists),
        }
    }

    fn withdrawal(&mut self, tx: u32, amount: FixedPrecision4) -> Result<(), TransactionError> {
        match self.transactions.get_mut(&tx) {
            None => {
                if self.funds.available_funds < amount {
                    return Err(TransactionError::TooFewFunds);
                }
                self.funds.available_funds -= amount;
                self.transactions.insert(tx, Transaction::new(amount));
                Ok(())
            }
            _ => Err(TransactionError::TransactionExists),
        }
    }

    fn dispute(&mut self, tx: u32) -> Result<(), TransactionError> {
        match self.transactions.get_mut(&tx) {
            None => Err(TransactionError::NoSuchTransaction),
            Some(transaction) => {
                if self.funds.available_funds < transaction.amount {
                    return Err(TransactionError::TooFewFunds);
                }
                transaction.dispute()?;
                self.funds.available_funds -= transaction.amount;
                self.funds.held_funds += transaction.amount;
                Ok(())
            }
        }
    }

    fn resolve(&mut self, tx: u32) -> Result<(), TransactionError> {
        match self.transactions.get_mut(&tx) {
            None => Err(TransactionError::NoSuchTransaction),
            Some(transaction) => {
                transaction.resolve()?;
                self.funds.available_funds += transaction.amount;
                self.funds.held_funds -= transaction.amount;
                Ok(())
            }
        }
    }

    fn chargeback(&mut self, tx: u32) -> Result<(), TransactionError> {
        match self.transactions.get_mut(&tx) {
            None => Err(TransactionError::NoSuchTransaction),
            Some(transaction) => {
                transaction.chargeback()?;
                self.funds.held_funds -= transaction.amount;
                Ok(())
            }
        }
    }
}

pub enum Account {
    Normal(NormalAccountState),
    Frozen(AccountFunds),
}

impl Account {
    pub fn process_deposit(
        &mut self,
        tx: u32,
        amount: FixedPrecision4,
    ) -> Result<(), TransactionError> {
        match self {
            Account::Normal(normal_account_state) => normal_account_state.deposit(tx, amount),
            Account::Frozen(_) => Err(TransactionError::AccountFrozen),
        }
    }

    pub fn process_withdrawal(
        &mut self,
        tx: u32,
        amount: FixedPrecision4,
    ) -> Result<(), TransactionError> {
        match self {
            Account::Normal(normal_account_state) => normal_account_state.withdrawal(tx, amount),
            Account::Frozen(_) => Err(TransactionError::AccountFrozen),
        }
    }

    pub fn process_dispute(&mut self, tx: u32) -> Result<(), TransactionError> {
        match self {
            Account::Normal(normal_account_state) => normal_account_state.dispute(tx),
            Account::Frozen(_) => Err(TransactionError::AccountFrozen),
        }
    }

    pub fn process_resolve(&mut self, tx: u32) -> Result<(), TransactionError> {
        match self {
            Account::Normal(normal_account_state) => normal_account_state.resolve(tx),
            Account::Frozen(_) => Err(TransactionError::AccountFrozen),
        }
    }

    pub fn process_chargeback(&mut self, tx: u32) -> Result<(), TransactionError> {
        match self {
            Account::Normal(normal_account_state) => {
                normal_account_state.chargeback(tx)?;
                let mut new_account = Account::Frozen(normal_account_state.funds);
                mem::swap(self, &mut new_account);
                Ok(())
            }
            Account::Frozen(_) => Err(TransactionError::AccountFrozen),
        }
    }
}

#[derive(Serialize)]
pub struct AccountRecord {
    pub client: u32,
    available: FixedPrecision4,
    held: FixedPrecision4,
    total: FixedPrecision4,
    locked: bool,
}

impl From<&Account> for AccountRecord {
    fn from(account: &Account) -> Self {
        match account {
            Account::Frozen(frozen_account) => AccountRecord {
                client: 0,
                available: frozen_account.available_funds,
                held: frozen_account.held_funds,
                total: frozen_account.total_funds(),
                locked: true,
            },
            Account::Normal(normal_account) => AccountRecord {
                client: 0,
                available: normal_account.funds.available_funds,
                held: normal_account.funds.held_funds,
                total: normal_account.funds.total_funds(),
                locked: false,
            },
        }
    }
}

#[cfg(test)]
#[path = "./tests/test_account.rs"]
mod test_account;
