use serde::{Deserialize, Serialize};

use crate::{account::Account, database::Database};

mod account;
mod database;
mod operation;
mod receipt;

fn main() {
    //let mut accounts: Vec<Account>;
    //let mut operations: Vec<Operations>;
    let mut db = Database::new();

    println!("Hello, world!");
}
