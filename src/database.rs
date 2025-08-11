use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use uuid::Uuid;

use crate::account::*;
use crate::operation::*;
use crate::receipt;

#[derive(Serialize, Deserialize)]
pub struct Database {
    db_version: String,
    pub accounts: Vec<Account>,
    pub operations: Vec<Operation>,
}

impl Database {
    pub fn load(filename: String) -> Self {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let obj = serde_json::from_reader(reader);
        obj.unwrap()
    }

    pub fn save(&self, filename: &str) {
        let buffer = File::create(filename).unwrap();
        let j = serde_json::to_writer_pretty(buffer, self).unwrap();
    }

    pub fn new() -> Self {
        Self {
            db_version: "0.0.1".to_string(),
            accounts: Vec::new(),
            operations: Vec::new(),
        }
    }

    pub fn add_account(
        &mut self,
        name: String,
        account_type: AccountType,
        number: String,
        bik: u32,
    ) {
        self.accounts
            .push(Account::new(name, account_type, number, bik));
    }

    pub fn add_operations(
        &mut self,
        account: Uuid,
        date_time: Option<NaiveDateTime>,
        operation_type: OperationType,
        summary: usize,
        direction: FinanseDirection,
        receipt: Option<receipt::Receipt>,
    ) {
        if !self.accounts.iter().any(|item| item.id == account) {
            panic!("Account doesn't exist!");
        }
        self.operations.push(Operation::new(
            account,
            date_time,
            operation_type,
            summary,
            direction,
            receipt,
        ))
    }
}

// account_id: Uuid,
// date_time: Option<NaiveDateTime>,
// operation_type: OperationType,
// summary: usize,
// direction: Direction,
// receipt: Option<receipt::Receipt>,
