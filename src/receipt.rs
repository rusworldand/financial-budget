use chrono::NaiveDateTime;

//Признак рассчёта - тип чека
enum CalculationType {
    Inbound,        // Чек прихода
    Outbound,       // Чек расхода
    InboundReturn,  // Возврат прихода
    OutboundReturn, // Возврат расхода
}

enum VatType {
    Vat20,
    Vat10,
    Vat7,
    Vat5,
    Vat0,
}

enum UnitType {
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

enum CashlessOpType {
    Payment,
    Cansel,
    Return,
}

enum Valut {
    Rub,
    Usd,
}
// Предмет рассчёта

pub struct Subject {
    name: String,        // Найменование
    unit_type: UnitType, // Тип количества
    count: usize,        // Количество
    price: usize,        // Цена
    summary: usize,      // Сумма
    vat_type: VatType,   // Тип НДС
    vat: usize,          // НДС
}

pub struct Slip {
    id: usize,                      //Номер терминала
    op_type: CashlessOpType,        // Тип операции
    date_time: NaiveDateTime,       // Дата - время
    summary: usize,                 // Сумма
    valut: Valut,                   // Валюта
    comm_summary: Option<usize>,    // Сумма комиссионного вознаграждения
    auth_code: String,              // Код авторизации
    card: String,                   // Номер карты
    address: Option<String>,        // Адрес
    place: Option<String>,          // Наименование магазина
    payment_system: Option<String>, // Платёжная система
    doc_id: Option<usize>,          // Номер документа
}

pub struct Receipt {
    date_time: NaiveDateTime,          //Дата время
    calculation_type: CalculationType, //Признак рассчёта - тип чека
    address: Option<String>,           // Адрес
    place: Option<String>,             // Место - Название учреждения
    subjects: Vec<Subject>,            // Предмет рассчёта - позиции в документе
    summary: usize,                    // Сумма
    cash: Option<usize>,               // Нал
    cashless: Option<usize>,           // Безнал
    prepayment: Option<usize>,         // Аванс
    postpayment: Option<usize>,        // Кредит
    in_kind: Option<usize>,            // За счёт з/п
    vat: Option<usize>,                // Сумма НДС
    url: Option<String>,               // Ссылка на чек
    slip: Option<Slip>,                // Слип-чек
}
