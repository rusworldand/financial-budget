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
    bank_bik: u32,
}
