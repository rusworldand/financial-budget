use chrono::Local;
use chrono::NaiveDate;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::account;
use crate::receipt;

#[derive(Serialize, Deserialize)]
pub enum OperationType {
    Initial,
    Buy,
    Sell,
    DebetingAccounts,       //Зачисление на счёт
    WithdrawalFromAccounts, // Списание со счёта
    ClosingAccounts,        // Закрытие счёта
}

#[derive(Serialize, Deserialize)]
pub enum Direction {
    Debet,  //+
    Credit, //-
}

#[derive(Serialize, Deserialize)]
pub struct Operation {
    date_time: NaiveDateTime,
    operation_type: OperationType,
    summary: usize,
    direction: Direction,
    receipt: Option<receipt::Receipt>, // conduction: bool,
}

impl Operation {
    pub fn new(
        date_time: Option<NaiveDateTime>,
        operation_type: OperationType,
        summary: usize,
        direction: Direction,
        receipt: Option<receipt::Receipt>,
    ) -> Self {
        Self {
            date_time: match date_time {
                Some(_) => date_time.expect("Empty"),
                None => Local::now().naive_local(),
            },
            operation_type: operation_type,
            summary: summary,
            direction: direction,
            receipt: receipt,
        }
    }
}

// pub fn new(
//     date_time: NaiveDateTime,
//     operation_type: OperationType,
//     summary: usize,
//     direction: Direction,
//     receipt: Option<receipt::Receipt>,
// ) -> Self {
//     Self {
//         date_time: date_time,
//         operation_type: operation_type,
//         summary: summary,
//         direction: direction,
//         receipt: receipt,
//     }
// }
