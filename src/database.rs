use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;

use crate::Account;
use crate::operation::Operation;

#[derive(Serialize, Deserialize)]
struct Operations {
    account: Account,
    operation: Operation,
}

impl Operations {
    fn new(account: Account, operation: Operation) -> Self {
        Self {
            account: account,
            operation: operation,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Database {
    db_version: String,
    account: Vec<Account>,
    operation: Vec<Operations>,
}

impl Database {
    fn load(filename: String) -> Self {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let obj = serde_json::from_reader(reader);
        obj.unwrap()
    }

    fn save(&self, filename: &String, db: &Database) {
        let file = File::open(filename).unwrap();
        let mut buffer = File::create(filename).unwrap();
        let j = serde_json::to_writer_pretty(buffer, self).unwrap();
    }

    fn new() -> Self {
        Self {
            db_version: "0.0.1".to_string(),
            account: Vec::new(),
            operation: Vec::new(),
        }
    }
}
