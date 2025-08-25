use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use strum::EnumIter;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, EnumIter)]
pub enum OperationType {
    Initial,
    Buy,
    Sell,
    ReturnBuy,
    ReturnSell,
    DebetingAccounts,       //Зачисление на счёт
    WithdrawalFromAccounts, // Списание со счёта
    ClosingAccounts,        // Закрытие счёта
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, EnumIter)]
pub enum FinanseDirection {
    Debet,  //+
    Credit, //-
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Operation {
    pub id: Uuid,
    pub date_time: NaiveDateTime,
    pub account_id: Uuid,
    pub operation_type: OperationType,
    pub summary: usize,
    pub direction: FinanseDirection,
    pub receipt_id: Option<Uuid>, // conduction: bool,
}

// impl Operation {
//     pub fn new(
//         account_id: Uuid,
//         date_time: Option<NaiveDateTime>,
//         operation_type: OperationType,
//         summary: usize,
//         direction: FinanseDirection,
//         receipt: Option<Uuid>,
//     ) -> Self {
//         Self {
//             id: Uuid::new_v4(),
//             date_time: match date_time {
//                 Some(_) => date_time.expect("Empty"),
//                 None => Local::now().naive_local(),
//             },
//             account_id: account_id,
//             operation_type: operation_type,
//             summary: summary,
//             direction: direction,
//             receipt_id: receipt,
//         }
//     }
// }

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
