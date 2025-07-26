use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
enum AccountType {
    Account,
    DebetCard,
    CreditCard,
    CreditAccount,
    AccumulativeAccount,
    Deposit,
}

#[derive(Serialize, Deserialize)]
pub struct Account {
    id: Uuid,
    name: String,
    account_type: AccountType,
    number: usize,
    bik: u32,
    sum: usize,
}

impl Account {
    fn new(name: String, account_type: AccountType, number: usize, bik: u32) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name,
            account_type: account_type,
            number: number,
            bik: bik,
            sum: 0,
        }
    }
}
