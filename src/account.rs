enum AccountType {
    Account,
    DebetCard,
    CreditCard,
    CreditAccount,
    AccumulativeAccount,
    Deposit,
}

pub struct Account {
    name: String,
    account_type: AccountType,
    number: usize,
    bank_bik: u32,
}
