use chrono::NaiveDateTime;

enum CalculationType {
    Inbound,
    Outbound,
    InboundReturn,
    OutboundReturn,
}

enum VatType {
    Vat20,
    Vat10,
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

pub struct Subject {
    name: String,
    unit_type: UnitType,
    count: usize,
    price: usize,
    summary: usize,
    vat_type: VatType,
    vat: usize,
}

pub struct Element {
    date_time: NaiveDateTime,
    calculation_type: CalculationType,
    address: Option<String>,
    place: Option<String>,
    subjects: Vec<Subject>,
    summary: usize,
    cash: Option<usize>,
    cashless: Option<usize>,
    prepayment: Option<usize>,
    postpayment: Option<usize>,
    in_kind: Option<usize>,
    vat: Option<usize>,
    url: Option<String>,
}
