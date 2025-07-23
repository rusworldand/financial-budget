use chrono::NaiveDateTime;

enum OperationType {
    Buy,
    Sell,
    CreditingAccounts,      //Зачисление на счёт
    WithdrawalFromAccounts, // Списание со счёта
    ClosingAccounts,        // Закрытие счёта
}

enum Direction {
    Debet,
    Credit,
}

pub struct operation {
    date_time: NaiveDateTime,
    operation_type: OperationType,
    summary: usize,
    direction: Direction,
    // conduction: bool,
}
