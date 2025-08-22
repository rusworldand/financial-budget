use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
// use std::io::{BufReader, BufWriter, Write};

use crate::account::*;
use crate::operation::*;
use crate::receipt::*;

const VERSION: &str = "0.0.1";

#[derive(Serialize, Deserialize)]
pub struct Database {
    db_version: String,
    pub accounts: Vec<Account>,
    pub operations: Vec<Operation>,
    pub receipts: Vec<Receipt>,
}

// /home/user/rust_projects/file.json

impl Database {
    pub fn load(filename: String) -> Self {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let obj = serde_json::from_reader(reader);
        obj.unwrap()
    }

    pub fn save(&self, filename: &str) {
        let buffer = File::create(filename).unwrap();
        serde_json::to_writer_pretty(buffer, self).unwrap();
    }

    pub fn new() -> Self {
        Self {
            db_version: VERSION.to_string(),
            accounts: Vec::new(),
            operations: Vec::new(),
            receipts: Vec::new(),
        }
    }
}
