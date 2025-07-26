use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;

use crate::account::*;
use crate::operation::Operation;

#[derive(Serialize, Deserialize)]
struct Operations {
    account: Account,
    operation: Operation,
}

impl Operations {
    pub fn new(account: Account, operation: Operation) -> Self {
        Self {
            account: account,
            operation: operation,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Database {
    db_version: String,
    accounts: Vec<Account>,
    operation: Vec<Operations>,
}

impl Database {
    pub fn load(filename: String) -> Self {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let obj = serde_json::from_reader(reader);
        obj.unwrap()
    }

    pub fn save(&self, filename: &String, db: &Database) {
        let file = File::open(filename).unwrap();
        let mut buffer = File::create(filename).unwrap();
        let j = serde_json::to_writer_pretty(buffer, self).unwrap();
    }

    pub fn new() -> Self {
        Self {
            db_version: "0.0.1".to_string(),
            accounts: Vec::new(),
            operation: Vec::new(),
        }
    }

    pub fn add_account(
        &mut self,
        name: String,
        account_type: AccountType,
        number: usize,
        bik: u32,
    ) {
        self.accounts
            .push(Account::new(name, account_type, number, bik));
    }
}
