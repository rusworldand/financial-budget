use chrono::NaiveDateTime;
use rust_decimal::{self, Decimal};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

//Признак рассчёта - тип чека
#[derive(Default, Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub enum CalculationType {
    #[default]
    Inbound, // Чек прихода
    Outbound,       // Чек расхода
    InboundReturn,  // Возврат прихода
    OutboundReturn, // Возврат расхода
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum VatType {
    Vat20,
    Vat10,
    Vat7,
    Vat5,
    Vat0,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
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

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum CashlessOpType {
    Payment,
    Cansel,
    Return,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Currency {
    Rub,
    Usd,
}
// Предмет рассчёта

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Subject {
    pub name: String,        // Найменование
    pub unit_type: UnitType, // Тип количества
    pub count: usize,        // Количество
    pub price: Decimal,      // Цена
    pub summary: Decimal,    // Сумма
    pub vat_type: VatType,   // Тип НДС
    pub vat: Decimal,        // НДС
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Slip {
    pub id: usize,                      //Номер терминала
    pub op_type: CashlessOpType,        // Тип операции
    pub date_time: NaiveDateTime,       // Дата - время
    pub summary: usize,                 // Сумма
    pub currency: Currency,             // Валюта
    pub comm_summary: Option<usize>,    // Сумма комиссионного вознаграждения
    pub auth_code: String,              // Код авторизации
    pub card: String,                   // Номер карты
    pub address: Option<String>,        // Адрес
    pub place: Option<String>,          // Наименование магазина
    pub payment_system: Option<String>, // Платёжная система
    pub doc_id: Option<usize>,          // Номер документа
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Receipt {
    /// Идентификатор
    pub id: Uuid,
    /// Дата время
    pub date_time: NaiveDateTime,
    /// Признак рассчёта - тип чека
    pub calculation_type: CalculationType,
    /// Адрес
    pub address: Option<String>,
    /// Место - Название учреждения
    pub place: Option<String>,
    /// Предмет рассчёта - позиции в документе
    pub subjects: Vec<Subject>,
    /// Сумма
    pub summary: Decimal,
    /// Нал
    pub cash: Option<Decimal>,
    /// Безнал
    pub cashless: Option<Decimal>,
    /// Аванс
    pub prepayment: Option<Decimal>,
    /// Кредит
    pub postpayment: Option<Decimal>,
    /// За счёт з/п
    pub in_kind: Option<Decimal>,
    /// Сумма НДС
    pub vat: Option<Decimal>,
    /// Ссылка на чек
    pub url: Option<String>,
    /// Слип-чек
    pub slip: Option<Slip>,
}

impl Receipt {
    pub fn empty_new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
