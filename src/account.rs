use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum AccountType {
    Account,
    Cash,
    DebetCard,
    CreditCard,
    CreditAccount,
    AccumulativeAccount,
    Deposit,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    pub id: Uuid,
    pub name: String,
    pub account_type: AccountType,
    pub number: String,
    pub bik: u32,
    pub sum: usize,
}

// impl Account {
//     pub fn new(name: String, account_type: AccountType, number: String, bik: u32) -> Self {
//         Self {
//             id: Uuid::new_v4(),
//             name: name,
//             account_type: account_type,
//             number: number,
//             bik: bik,
//             sum: 0,
//         }
//     }
// }
