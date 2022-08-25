use crate::{errors::TransactionError, fixed_precision::FixedPrecision4};

#[derive(Debug, PartialEq, Eq)]
pub enum TransactionState {
    Normal,
    Dispute,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Transaction {
    pub amount: FixedPrecision4,
    pub state: TransactionState,
}

impl Transaction {
    pub fn new(amount: FixedPrecision4) -> Self {
        Transaction {
            amount,
            state: TransactionState::Normal,
        }
    }

    pub fn dispute(&mut self) -> Result<(), TransactionError> {
        match self.state {
            TransactionState::Normal => self.state = TransactionState::Dispute,
            TransactionState::Dispute => return Err(TransactionError::AlreadyDisputed),
        }
        Ok(())
    }

    pub fn resolve(&mut self) -> Result<(), TransactionError> {
        match self.state {
            TransactionState::Normal => return Err(TransactionError::NotDisputed),
            TransactionState::Dispute => self.state = TransactionState::Normal,
        }
        Ok(())
    }

    pub fn chargeback(&mut self) -> Result<(), TransactionError> {
        self.resolve()
    }
}
