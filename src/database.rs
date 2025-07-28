use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;
use uuid::Uuid;

use crate::account::*;
use crate::operation::Operation;

#[derive(Serialize, Deserialize)]
struct Operations {
    account: Uuid,
    operation: Operation,
}

impl Operations {
    pub fn new(account: Uuid, operation: Operation) -> Self {
        Self {
            account: account,
            operation: operation,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Database {
    db_version: String,
    pub accounts: Vec<Account>,
    pub operations: Vec<Operations>,
}

impl Database {
    pub fn load(filename: String) -> Self {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let obj = serde_json::from_reader(reader);
        obj.unwrap()
    }

    pub fn save(&self, filename: &str) {
        let mut buffer = File::create(filename).unwrap();
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

    pub fn add_operations(&mut self, account: Uuid, operation: Operation) {
        if !self.accounts.iter().any(|item| item.id == account) {
            panic!("Account doesn't exist!");
        }
        self.operations.push(Operations::new(account, operation))
    }
}
