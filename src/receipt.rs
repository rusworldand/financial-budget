use chrono::NaiveDateTime;
use rust_decimal::{self, Decimal};
use serde::{Deserialize, Serialize};

//Признак рассчёта - тип чека
#[derive(Default, Serialize, Deserialize, Debug)]
pub enum CalculationType {
    #[default]
    Inbound, // Чек прихода
    Outbound,       // Чек расхода
    InboundReturn,  // Возврат прихода
    OutboundReturn, // Возврат расхода
}

#[derive(Serialize, Deserialize, Debug)]
pub enum VatType {
    Vat20,
    Vat10,
    Vat7,
    Vat5,
    Vat0,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum UnitType {
    Pieces,
    Gramm,
    Kilogamm,
    // Грамм
    // Килограмм
    // Тонна
    // Сантиметр
    // Дециметр
    // Метр
    // Квадратный сантиметр
    // Квадратный дециметр
    // Квадратный метр
    // Миллилитр
    // Литр
    // Кубический метр
    // Киловатт час
    // Гигакалория
    // Сутки (день)
    // Час
    // Минута
    // Секунда
    // Килобайт
    // Мегабайт
    // Гигабайт
    // Терабайт
}

#[derive(Serialize, Deserialize, Debug)]
pub enum CashlessOpType {
    Payment,
    Cansel,
    Return,
}

#[derive(Serialize, Deserialize, Debug)]
enum Currency {
    Rub,
    Usd,
}
// Предмет рассчёта

#[derive(Serialize, Deserialize, Debug)]
pub struct Subject {
    name: String,        // Найменование
    unit_type: UnitType, // Тип количества
    count: usize,        // Количество
    price: Decimal,      // Цена
    summary: Decimal,    // Сумма
    vat_type: VatType,   // Тип НДС
    vat: Decimal,        // НДС
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Slip {
    id: usize,                      //Номер терминала
    op_type: CashlessOpType,        // Тип операции
    date_time: NaiveDateTime,       // Дата - время
    summary: usize,                 // Сумма
    currency: Currency,             // Валюта
    comm_summary: Option<usize>,    // Сумма комиссионного вознаграждения
    auth_code: String,              // Код авторизации
    card: String,                   // Номер карты
    address: Option<String>,        // Адрес
    place: Option<String>,          // Наименование магазина
    payment_system: Option<String>, // Платёжная система
    doc_id: Option<usize>,          // Номер документа
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Receipt {
    /// Дата время
    date_time: NaiveDateTime,
    /// Признак рассчёта - тип чека
    calculation_type: CalculationType,
    /// Адрес
    address: Option<String>,
    /// Место - Название учреждения
    place: Option<String>,
    /// Предмет рассчёта - позиции в документе
    subjects: Vec<Subject>,
    /// Сумма
    summary: Decimal,
    /// Нал
    cash: Option<Decimal>,
    /// Безнал
    cashless: Option<Decimal>,
    /// Аванс
    prepayment: Option<Decimal>,
    /// Кредит
    postpayment: Option<Decimal>,
    /// За счёт з/п
    in_kind: Option<Decimal>,
    /// Сумма НДС
    vat: Option<Decimal>,
    /// Ссылка на чек
    url: Option<String>,
    /// Слип-чек
    slip: Option<Slip>,
}

impl Receipt {
    pub fn short_new(summary: Decimal) -> Self {
        Self {
            summary,
            ..Default::default()
        }
    }
}
