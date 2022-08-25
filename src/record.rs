use serde::Deserialize;

use crate::fixed_precision::FixedPrecision4;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RecordType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    ChargeBack,
}

#[derive(Debug, Deserialize)]
pub struct Record {
    #[serde(alias = "type")]
    pub record_type: RecordType,
    pub client: u16,
    pub tx: u32,
    pub amount: Option<FixedPrecision4>,
}
