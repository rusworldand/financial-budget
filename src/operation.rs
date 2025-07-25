use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::receipt;

#[derive(Serialize, Deserialize)]
enum OperationType {
    Initial,
    Buy,
    Sell,
    DebetingAccounts,       //Зачисление на счёт
    WithdrawalFromAccounts, // Списание со счёта
    ClosingAccounts,        // Закрытие счёта
}

#[derive(Serialize, Deserialize)]
enum Direction {
    Debet,
    Credit,
}

#[derive(Serialize, Deserialize)]
pub struct Operation {
    date_time: NaiveDateTime,
    operation_type: OperationType,
    summary: usize,
    direction: Direction,
    receipt: Option<receipt::Receipt>, // conduction: bool,
}
