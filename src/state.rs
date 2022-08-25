use std::io::{Read, Write};

use crate::{
    account::{Account, AccountRecord},
    errors::TransactionError,
    record::Record,
};

pub struct State {
    pub accounts: Vec<Option<Box<Account>>>,
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

impl State {
    fn new() -> Self {
        let mut accounts = Vec::<Option<Box<Account>>>::new();
        accounts.resize_with(u16::MAX as usize + 1, || None);
        State { accounts }
    }

    fn process_record(&mut self, record: &Record) -> Result<(), TransactionError> {
        let account = &mut self.accounts[record.client as usize];
        if account.is_none() {
            *account = Some(Box::new(Account::Normal(Default::default())));
        }
        let account = account.as_mut().unwrap();
        match record.record_type {
            crate::record::RecordType::Deposit => {
                account.process_deposit(record.tx, record.amount.unwrap())
            }
            crate::record::RecordType::Withdrawal => {
                account.process_withdrawal(record.tx, record.amount.unwrap())
            }
            crate::record::RecordType::Dispute => account.process_dispute(record.tx),
            crate::record::RecordType::Resolve => account.process_resolve(record.tx),
            crate::record::RecordType::ChargeBack => account.process_chargeback(record.tx),
        }
    }

    pub fn process_data<R: Read>(&mut self, rdr: &mut csv::Reader<R>) {
        for record in rdr.deserialize::<Record>() {
            match record {
                Err(err) => std::eprint!("Error while parsing the file: {err}"),
                Ok(record) => {
                    if let Err(err) = self.process_record(&record) {
                        std::eprintln!("Error while record processing: {record:?}: {err:?}");
                    }
                }
            }
        }
    }

    pub fn serialize_result<W: Write>(&mut self, writer: &mut csv::Writer<W>) {
        for (index, mut account_record) in
            self.accounts
                .iter()
                .enumerate()
                .filter_map(|(index, account)| {
                    Some((
                        index,
                        <&Account as Into<AccountRecord>>::into(account.as_ref()?),
                    ))
                })
        {
            account_record.client = index as u32;
            writer.serialize(account_record).unwrap();
        }
    }
}
