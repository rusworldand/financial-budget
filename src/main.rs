use chrono::{Date, Local, Utc};
use serde::{Deserialize, Serialize};

use crate::{account::Account, database::Database, operation::Operation};

mod account;
mod database;
mod operation;
mod receipt;

fn main() {
    let mut db = Database::new();

    db.add_account(
        "Account1".to_string(),
        account::AccountType::Account,
        "4527545525245151".to_string(),
        045342768,
    );
    db.add_account(
        "Account2".to_string(),
        account::AccountType::Cash,
        "1".to_string(),
        0,
    );
    db.add_operations(
        db.accounts[1].id,
        Operation::new(
            None,
            operation::OperationType::Initial,
            10000,
            operation::Direction::Debet,
            Some(receipt::Receipt::short_new(10000)),
        ),
    );
    db.add_operations(
        db.accounts[0].id,
        Operation::new(
            None,
            operation::OperationType::Initial,
            20000,
            operation::Direction::Debet,
            Some(receipt::Receipt::short_new(20000)),
        ),
    );
    db.add_operations(
        db.accounts[0].id,
        Operation::new(
            None,
            operation::OperationType::Buy,
            100,
            operation::Direction::Credit,
            None,
        ),
    );

    db.save("/home/user/rust_projects/file.json");

    println!("Hello, world!");
}
