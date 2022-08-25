#[derive(Debug)]
pub enum TransactionError {
    AccountFrozen,
    AlreadyDisputed,
    NoSuchTransaction,
    NotDisputed,
    TooFewFunds,
    TransactionExists,
}
