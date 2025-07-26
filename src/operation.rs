use chrono::NaiveDate;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::account;
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
    // fn new(
    //     date_time: Option<NaiveDateTime>,
    //     operation_type: OperationType,
    //     summary: usize,
    //     direction: Direction,
    //     receipt: Option<receipt::Receipt>,
    // ) -> Self {
    //     Self {
    //         date_time: match date_time {
    //             Some(_) => date_time.expect("Empty"),
    //             None => NaiveDateTime::new(NaiveDate::, time),
    //         },
    //         operation_type,
    //         summary,
    //         direction,
    //         receipt,
    //     }
    // }
    fn new(
        date_time: NaiveDateTime,
        operation_type: OperationType,
        summary: usize,
        direction: Direction,
        receipt: Option<receipt::Receipt>,
    ) -> Self {
        Self {
            date_time: date_time,
            operation_type: operation_type,
            summary: summary,
            direction: direction,
            receipt: receipt,
        }
    }
}
