#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use std::{env, ops::Deref};

use chrono::{Date, DateTime, Local, NaiveDate, NaiveDateTime, NaiveTime, Timelike, Utc};
use egui_extras::{Column, Size, StripBuilder, TableBuilder};
use rust_decimal::{self, Decimal, dec};
use uuid::Uuid;

use crate::{
    account::{self, *},
    database::*,
    operation::{self, *},
    receipt::{self, *},
};

use eframe::egui::{self, Response, Ui};

enum Selection {
    Account(Uuid),
    Operation(Uuid),
}

enum TableType {
    Account,
    Operation,
}

enum Statement {
    Common,
    EditAccount(Uuid),
    EditOperation(Uuid),
    EditReceipt(Uuid, Uuid, bool),
    ThripleDialog,
}

struct AccountFields {
    name: String,
    account_type: account::AccountType,
    number: String,
    bik: String,
}

impl AccountFields {
    fn new() -> Self {
        Self {
            name: "".to_string(),
            account_type: account::AccountType::Cash,
            number: "".to_string(),
            bik: "100000000".to_string(),
        }
    }
}

struct OperationFields {
    date: NaiveDate,
    hour: u32,
    minute: u32,
    account_id: Uuid,
    operation_type: OperationType,
    summary: String,
    direction: FinanseDirection,
    receipt: Option<Uuid>,
}

impl OperationFields {
    fn new() -> Self {
        Self {
            date: chrono::Local::now().date_naive(),
            hour: 0,
            minute: 0,
            account_id: Uuid::nil(),
            operation_type: OperationType::Initial,
            summary: "0".to_string(),
            direction: FinanseDirection::Credit,
            receipt: None,
        }
    }
}

struct ReceiptFields {
    date: NaiveDate,
    hour: u32,
    minute: u32,
    calculation_type: receipt::CalculationType,
    address: String,
    place: String,
    subjects: Vec<receipt::Subject>,
    summary: String,
    cash: String,
    cashless: String,
    prepayment: String,
    postpayment: String,
    in_kind: String,
    vat: String,
    url: String,
}

impl ReceiptFields {
    fn new() -> Self {
        Self {
            date: chrono::Local::now().date_naive(),
            hour: 0,
            minute: 0,
            calculation_type: receipt::CalculationType::Inbound,
            address: "".to_string(),
            place: "".to_string(),
            subjects: Vec::new(),
            summary: "".to_string(),
            cash: "".to_string(),
            cashless: "".to_string(),
            prepayment: "".to_string(),
            postpayment: "".to_string(),
            in_kind: "".to_string(),
            vat: "".to_string(),
            url: "".to_string(),
        }
    }
}

pub struct App {
    db: Database,
    file: String,
    selected: Option<Selection>,
    statement: Statement,
    account_fields: AccountFields,
    operation_fields: OperationFields,
    receipt_fields: ReceiptFields,
}

impl App {
    pub fn new(arg: Option<&String>) -> Self {
        if let Some(arg) = arg {
            Self {
                db: Database::load(arg.clone()),
                file: arg.clone(),
                selected: None,
                statement: Statement::Common,
                account_fields: AccountFields::new(),
                operation_fields: OperationFields::new(),
                receipt_fields: ReceiptFields::new(),
            }
        } else {
            Self {
                db: Database::new(),
                file: "file.json".to_string(),
                selected: None,
                statement: Statement::Common,
                account_fields: AccountFields::new(),
                operation_fields: OperationFields::new(),
                receipt_fields: ReceiptFields::new(),
            }
        }
    }
    fn response_compare(variable: Response, temp_response: &mut Option<Response>) {
        if let Some(response) = temp_response {
            *response = response.union(variable);
        } else {
            *temp_response = Some(variable);
        }
    }
    fn table(&mut self, table_type: TableType, ui: &mut Ui) -> Response {
        match table_type {
            TableType::Account => Some(ui.label(format!("Accounts"))),
            TableType::Operation => Some(ui.label(format!("Operations"))),
        };

        StripBuilder::new(ui)
            .size(Size::exact(200.0)) // for the table
            .vertical(|mut strip| {
                strip.cell(|ui| {
                    let mut table = TableBuilder::new(ui).resizable(true).striped(true);
                    match table_type {
                        TableType::Account => table = table.id_salt("accounts_table"),
                        TableType::Operation => table = table.id_salt("operations_table"),
                    }
                    table = table.cell_layout(egui::Layout::left_to_right(egui::Align::Center));
                    match table_type {
                        TableType::Account => {
                            table = table.column(Column::auto()).column(Column::auto());
                        }
                        TableType::Operation => {
                            table = table
                                .column(Column::auto())
                                .column(Column::auto())
                                .column(Column::auto());
                        }
                    }
                    table = table
                        .min_scrolled_height(0.0)
                        .max_scroll_height(500.0)
                        .sense(egui::Sense::click());

                    table
                        .header(30.0, |mut header| {
                            match table_type {
                                TableType::Account => {
                                    header.col(|ui| {
                                        ui.strong("ID");
                                        // ui.push_id(id_salt, add_contents)
                                    });
                                    header.col(|ui| {
                                        ui.strong("Name");
                                    });
                                }
                                TableType::Operation => {
                                    header.col(|ui| {
                                        ui.strong("Account");
                                    });
                                    header.col(|ui| {
                                        ui.strong("Date / Time");
                                    });
                                    header.col(|ui| {
                                        ui.strong("Sum");
                                    });
                                }
                            }
                        })
                        .body(|mut body| {
                            //let mut temp_response: Option<Response> = None;
                            let contents =
                                |ui: &mut eframe::egui::Ui,
                                 text: String,
                                 response: &mut Option<Response>| {
                                    App::response_compare(ui.label(text), response);
                                };
                            match table_type {
                                TableType::Account => {
                                    for i in &self.db.accounts {
                                        body.row(30.0, |mut row| {
                                            let mut inner_response: Option<Response> = None;
                                            row.col(|ui| {
                                                contents(
                                                    ui,
                                                    format!("ID '{}'", i.id),
                                                    &mut inner_response,
                                                )
                                            });

                                            row.col(|ui| {
                                                contents(
                                                    ui,
                                                    format!("Name '{}'", i.name),
                                                    &mut inner_response,
                                                )
                                            });

                                            let row_response = row.response();

                                            App::response_compare(
                                                row_response,
                                                &mut inner_response,
                                            );

                                            if let Some(response) = inner_response {
                                                if response.double_clicked() {
                                                    println!("Double!");
                                                    println!("{}", i.id);
                                                    self.selected = Some(Selection::Account(i.id))
                                                }
                                                if response.triple_clicked() {
                                                    println!("Triple!");
                                                    println!("{}", i.id);
                                                    self.selected = Some(Selection::Account(i.id))
                                                }
                                            }
                                        });
                                    }
                                }
                                TableType::Operation => {
                                    for i in &self.db.operations {
                                        body.row(30.0, |mut row| {
                                            let mut inner_response: Option<Response> = None;
                                            row.col(|ui| {
                                                contents(
                                                    ui,
                                                    format!("ID '{}'", i.account_id),
                                                    &mut inner_response,
                                                )
                                            });

                                            row.col(|ui| {
                                                contents(
                                                    ui,
                                                    format!("Name '{}'", i.date_time),
                                                    &mut inner_response,
                                                )
                                            });

                                            row.col(|ui| {
                                                contents(
                                                    ui,
                                                    format!("Name '{}'", i.summary),
                                                    &mut inner_response,
                                                )
                                            });
                                            let row_response = row.response();

                                            App::response_compare(
                                                row_response,
                                                &mut inner_response,
                                            );
                                            if let Some(response) = inner_response {
                                                if response.double_clicked() {
                                                    println!("Double!");
                                                    println!("{}", i.id);
                                                    self.selected = Some(Selection::Operation(i.id))
                                                }
                                                if response.triple_clicked() {
                                                    println!("Triple!");
                                                    println!("{}", i.id);
                                                    self.selected = Some(Selection::Operation(i.id))
                                                }
                                            };
                                        });
                                    }
                                }
                            }
                        });
                });
            })
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            //ui.push_id(id_salt, add_contents)
            self.table(TableType::Account, ui);
            ui.separator();
            self.table(TableType::Operation, ui);
        });
        egui::SidePanel::right("right_panel")
            .resizable(true)
            .default_width(150.0)
            .width_range(150.0..=200.0)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    if let Some(selection) = &self.selected {
                        match selection {
                            Selection::Account(_) => ui.heading("Аккаунт"),
                            Selection::Operation(_) => ui.heading("Операция"),
                        }
                    } else {
                        ui.heading("Элемент")
                    }
                });
                egui::ScrollArea::vertical().show(ui, |ui| {
                    // lorem_ipsum(ui);
                    if let Some(selection) = &self.selected {
                        match selection {
                            Selection::Account(uuid) => {
                                let iter =
                                    &self.db.accounts.iter().find(|account| account.id == *uuid);
                                if let Some(element) = iter {
                                    ui.label(format!("{}", element.id));
                                }
                            }
                            Selection::Operation(uuid) => {
                                let iter = &self
                                    .db
                                    .operations
                                    .iter()
                                    .find(|operation| operation.id == *uuid);
                                if let Some(element) = iter {
                                    ui.label(format!("{}", element.id));
                                }
                            }
                        }
                    }
                });

                if let Some(selection) = &self.selected {
                    if ui.button("Edit").clicked() {
                        match selection {
                            Selection::Account(uuid) => {
                                let iter = &self
                                    .db
                                    .accounts
                                    .iter()
                                    .find(|account| account.id == *uuid)
                                    .unwrap();

                                self.account_fields.name = iter.name.clone();
                                self.account_fields.account_type = iter.account_type.clone();
                                self.account_fields.number = iter.number.clone();
                                self.account_fields.bik = iter.bik.to_string();
                                self.statement = Statement::EditAccount(*uuid);
                            }
                            Selection::Operation(uuid) => {
                                let iter = &self
                                    .db
                                    .operations
                                    .iter()
                                    .find(|operation| operation.id == *uuid)
                                    .unwrap();

                                self.operation_fields.date = iter.date_time.date();
                                self.operation_fields.hour = iter.date_time.time().hour();
                                self.operation_fields.minute = iter.date_time.time().minute();
                                self.operation_fields.account_id = iter.account_id;
                                self.operation_fields.operation_type = iter.operation_type.clone();
                                self.operation_fields.summary = iter.summary.to_string();
                                self.operation_fields.direction = iter.direction.clone();
                                self.statement = Statement::EditOperation(*uuid);
                            }
                        }
                    }
                }
            });
        egui::TopBottomPanel::bottom("bottom_panel")
            .resizable(false)
            .min_height(0.0)
            .show(ctx, |ui| {
                if ui.button("New Account").clicked() {
                    self.statement = Statement::EditAccount(Uuid::new_v4());
                    self.account_fields = AccountFields::new();
                }
                if ui.button("New Operation").clicked() {
                    self.statement = Statement::EditOperation(Uuid::new_v4());
                }
                if ui.button("Save").clicked() {
                    self.db.save(&self.file);
                }
            });

        match &mut self.statement {
            Statement::Common => {}
            Statement::EditAccount(uuid) => {
                //if let Some(AccountFields) = fields {}
                let acc_id = uuid.clone();
                ctx.show_viewport_immediate(
                    egui::ViewportId::from_hash_of("account window"),
                    egui::ViewportBuilder::default()
                        .with_title("Account")
                        .with_inner_size([400.0, 200.0]),
                    |ctx, class| {
                        assert!(
                            class == egui::ViewportClass::Immediate,
                            "This egui backend doesn't support multiple viewports"
                        );

                        let mut close_request: bool = false;

                        egui::CentralPanel::default().show(ctx, |ui| {
                            ui.label("Name");
                            // self.db.add_account(name, account_type, number, bik);
                            //let mut response: Response;
                            //response =
                            ui.add(egui::TextEdit::singleline(&mut self.account_fields.name));
                            egui::ComboBox::from_label("Select one!")
                                .selected_text(format!("{:?}", self.account_fields.account_type))
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(
                                        &mut self.account_fields.account_type,
                                        account::AccountType::Account,
                                        "Common Account",
                                    );
                                    ui.selectable_value(
                                        &mut self.account_fields.account_type,
                                        account::AccountType::Cash,
                                        "Cash",
                                    );
                                    ui.selectable_value(
                                        &mut self.account_fields.account_type,
                                        account::AccountType::DebetCard,
                                        "DebetCard",
                                    );
                                    ui.selectable_value(
                                        &mut self.account_fields.account_type,
                                        account::AccountType::CreditCard,
                                        "CreditCard",
                                    );
                                    ui.selectable_value(
                                        &mut self.account_fields.account_type,
                                        account::AccountType::CreditAccount,
                                        "CreditAccount",
                                    );
                                    ui.selectable_value(
                                        &mut self.account_fields.account_type,
                                        account::AccountType::AccumulativeAccount,
                                        "AccumulativeAccount",
                                    );
                                    ui.selectable_value(
                                        &mut self.account_fields.account_type,
                                        account::AccountType::Deposit,
                                        "Deposit",
                                    );
                                });

                            ui.add(
                                egui::TextEdit::singleline(&mut self.account_fields.number)
                                    .char_limit(30),
                            );
                            ui.add(
                                egui::TextEdit::singleline(&mut self.account_fields.bik)
                                    .char_limit(9),
                            );
                            if ui.button("Apply").clicked() {
                                let iter = self
                                    .db
                                    .accounts
                                    .iter_mut()
                                    .find(|account| account.id == acc_id);
                                if let Some(element) = iter {
                                    element.account_type = self.account_fields.account_type.clone();
                                    element.name = self.account_fields.name.clone();
                                    element.number = self.account_fields.number.clone();
                                    element.bik = self.account_fields.bik.parse::<u32>().unwrap();
                                } else {
                                    self.db.accounts.push(Account {
                                        id: acc_id,
                                        name: self.account_fields.name.clone(),
                                        account_type: self.account_fields.account_type.clone(),
                                        number: self.account_fields.number.clone(),
                                        bik: self.account_fields.bik.parse::<u32>().unwrap(),
                                        sum: 0,
                                    });
                                }
                                close_request = true;
                            }
                        });

                        if ctx.input(|i| i.viewport().close_requested()) || close_request {
                            // Tell parent viewport that we should not show next frame:
                            // self.show_immediate_viewport = false;
                            //
                            self.account_fields = AccountFields::new();
                            self.statement = Statement::Common;
                        }
                    },
                );

                // let window = eframe::egui::Window::new("New Account");
                // let window2 = window.show(ctx, |ui| ui.label("text"));
            }
            Statement::EditOperation(uuid) => {
                let op_id = uuid.clone();
                ctx.show_viewport_immediate(
                    egui::ViewportId::from_hash_of("operation window"),
                    egui::ViewportBuilder::default()
                        .with_title("Operation")
                        .with_inner_size([400.0, 400.0]),
                    |ctx, class| {
                        assert!(
                            class == egui::ViewportClass::Immediate,
                            "This egui backend doesn't support multiple viewports"
                        );

                        let mut close_request: bool = false;

                        egui::CentralPanel::default().show(ctx, |ui| {
                            ui.label("Date and time");
                            ui.add(egui_extras::DatePickerButton::new(
                                &mut self.operation_fields.date,
                            ));
                            ui.add(
                                egui::DragValue::new(&mut self.operation_fields.hour)
                                    .speed(1)
                                    .range(0..=23),
                            );
                            ui.add(
                                egui::DragValue::new(&mut self.operation_fields.minute)
                                    .speed(1)
                                    .range(0..=59),
                            );

                            ui.label("Account");
                            egui::ComboBox::from_label("Select account!")
                                .selected_text(format!("{:?}", self.operation_fields.account_id))
                                .show_ui(ui, |ui| {
                                    for element in self.db.accounts.iter() {
                                        ui.selectable_value(
                                            &mut self.operation_fields.account_id,
                                            element.id,
                                            format!("{}", element.name),
                                        );
                                    }
                                });
                            ui.label("Operation type");
                            egui::ComboBox::from_label("Select type!")
                                .selected_text(format!(
                                    "{:?}",
                                    self.operation_fields.operation_type
                                ))
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(
                                        &mut self.operation_fields.operation_type,
                                        operation::OperationType::Initial,
                                        "Initial",
                                    );
                                    ui.selectable_value(
                                        &mut self.operation_fields.operation_type,
                                        operation::OperationType::Buy,
                                        "Buy",
                                    );
                                    ui.selectable_value(
                                        &mut self.operation_fields.operation_type,
                                        operation::OperationType::Sell,
                                        "Sell",
                                    );
                                    ui.selectable_value(
                                        &mut self.operation_fields.operation_type,
                                        operation::OperationType::ReturnBuy,
                                        "Return Buy",
                                    );
                                    ui.selectable_value(
                                        &mut self.operation_fields.operation_type,
                                        operation::OperationType::ReturnSell,
                                        "Return Sell",
                                    );
                                    ui.selectable_value(
                                        &mut self.operation_fields.operation_type,
                                        operation::OperationType::DebetingAccounts,
                                        "Debeting Accounts",
                                    );
                                    ui.selectable_value(
                                        &mut self.operation_fields.operation_type,
                                        operation::OperationType::WithdrawalFromAccounts,
                                        "Withdrawal From Account",
                                    );
                                    ui.selectable_value(
                                        &mut self.operation_fields.operation_type,
                                        operation::OperationType::ClosingAccounts,
                                        "Closing Account",
                                    );
                                });
                            ui.label("Summ");
                            ui.add(egui::TextEdit::singleline(
                                &mut self.operation_fields.summary,
                            ));

                            ui.label("Direction");
                            egui::ComboBox::from_label("Select direction!")
                                .selected_text(format!("{:?}", self.operation_fields.direction))
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(
                                        &mut self.operation_fields.direction,
                                        operation::FinanseDirection::Debet,
                                        "Debet",
                                    );
                                    ui.selectable_value(
                                        &mut self.operation_fields.direction,
                                        operation::FinanseDirection::Credit,
                                        "Credit",
                                    );
                                });
                            if ui.button("Receipt").clicked() {
                                self.receipt_fields = ReceiptFields::new();
                                let iter = self
                                    .db
                                    .operations
                                    .iter_mut()
                                    .find(|operation| operation.id == op_id);
                                if let None = iter {
                                    self.statement =
                                        Statement::EditReceipt(Uuid::new_v4(), op_id, true)
                                } else {
                                    if let Some(identificator) = self.operation_fields.receipt {
                                        self.statement =
                                            Statement::EditReceipt(identificator, op_id, false);
                                    } else {
                                        self.statement =
                                            Statement::EditReceipt(Uuid::new_v4(), op_id, false)
                                    }
                                }
                            }
                            // self.operation_fields.date = iter.date_time.date();
                            // self.operation_fields.time = iter.date_time.time();
                            // self.operation_fields.hour = self.operation_fields.time.hour();
                            // self.operation_fields.minute = self.operation_fields.time.minute();
                            // self.operation_fields.account_id = iter.account_id;
                            // self.operation_fields.operation_type = iter.operation_type.clone();
                            // self.operation_fields.summary = iter.summary.to_string();
                            // self.operation_fields.direction = iter.direction.clone();

                            if ui.button("Apply").clicked() {
                                let iter = self
                                    .db
                                    .operations
                                    .iter_mut()
                                    .find(|operation| operation.id == op_id);
                                let time = chrono::NaiveTime::from_hms_opt(
                                    self.operation_fields.hour,
                                    self.operation_fields.minute,
                                    0,
                                )
                                .unwrap();
                                if let Some(element) = iter {
                                    element.date_time = chrono::NaiveDateTime::new(
                                        self.operation_fields.date,
                                        time,
                                    );
                                    element.account_id = self.operation_fields.account_id;
                                    element.operation_type =
                                        self.operation_fields.operation_type.clone();
                                    element.summary =
                                        self.operation_fields.summary.parse::<usize>().unwrap();
                                    element.direction = self.operation_fields.direction.clone();
                                } else {
                                    self.db.operations.push(Operation {
                                        id: op_id,
                                        date_time: chrono::NaiveDateTime::new(
                                            self.operation_fields.date,
                                            time,
                                        ),
                                        account_id: self.operation_fields.account_id,
                                        operation_type: self
                                            .operation_fields
                                            .operation_type
                                            .clone(),
                                        direction: self.operation_fields.direction.clone(),
                                        receipt_id: None,
                                        summary: self
                                            .operation_fields
                                            .summary
                                            .parse::<usize>()
                                            .unwrap(),
                                    });
                                }
                                close_request = true;
                            }
                        });

                        if ctx.input(|i| i.viewport().close_requested()) || close_request {
                            // Tell parent viewport that we should not show next frame:
                            // self.show_immediate_viewport = false;
                            self.operation_fields = OperationFields::new();
                            self.statement = Statement::Common;
                        }
                    },
                );
            }

            Statement::EditReceipt(uuid, op_uuid, signal) => {
                let rec_id = uuid.clone();
                let op_id = op_uuid.clone();
                let signal = signal.clone();
                ctx.show_viewport_immediate(
                    egui::ViewportId::from_hash_of("receipt window"),
                    egui::ViewportBuilder::default()
                        .with_title("Receipt")
                        .with_inner_size([400.0, 200.0]),
                    |ctx, class| {
                        assert!(
                            class == egui::ViewportClass::Immediate,
                            "This egui backend doesn't support multiple viewports"
                        );

                        let mut close_request: bool = false;

                        egui::CentralPanel::default().show(ctx, |ui| {
                            ui.label("Date and time");
                            ui.add(egui_extras::DatePickerButton::new(
                                &mut self.receipt_fields.date,
                            ));
                            ui.add(
                                egui::DragValue::new(&mut self.receipt_fields.hour)
                                    .speed(1)
                                    .range(0..=23),
                            );
                            ui.add(
                                egui::DragValue::new(&mut self.receipt_fields.minute)
                                    .speed(1)
                                    .range(0..=59),
                            );
                            ui.label("Receipt type");
                            egui::ComboBox::from_label("Select calculation type!")
                                .selected_text(format!(
                                    "{:?}",
                                    self.receipt_fields.calculation_type
                                ))
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(
                                        &mut self.receipt_fields.calculation_type,
                                        receipt::CalculationType::Inbound,
                                        "Inbound",
                                    );
                                    ui.selectable_value(
                                        &mut self.receipt_fields.calculation_type,
                                        receipt::CalculationType::InboundReturn,
                                        "Inbound Return",
                                    );
                                    ui.selectable_value(
                                        &mut self.receipt_fields.calculation_type,
                                        receipt::CalculationType::Outbound,
                                        "Outbound",
                                    );
                                    ui.selectable_value(
                                        &mut self.receipt_fields.calculation_type,
                                        receipt::CalculationType::OutboundReturn,
                                        "Outbound Return",
                                    );
                                });

                            ui.label("Adress");
                            ui.add(egui::TextEdit::singleline(&mut self.receipt_fields.address));
                            ui.label("Point name");
                            ui.add(egui::TextEdit::singleline(&mut self.receipt_fields.place));

                            StripBuilder::new(ui)
                                .size(Size::exact(200.0))
                                .vertical(|mut strip| {
                                    strip.cell(|ui| {
                                        let mut table = TableBuilder::new(ui)
                                            .resizable(true)
                                            .striped(true)
                                            .id_salt("subject_table")
                                            .cell_layout(egui::Layout::left_to_right(
                                                egui::Align::Center,
                                            ))
                                            .column(Column::auto())
                                            .column(Column::auto())
                                            .column(Column::auto())
                                            .column(Column::auto())
                                            .column(Column::auto())
                                            .column(Column::auto())
                                            .column(Column::auto())
                                            .column(Column::auto())
                                            .min_scrolled_height(0.0)
                                            .max_scroll_height(500.0)
                                            .sense(egui::Sense::click());
                                        table
                                            .header(30.0, |mut header| {
                                                header.col(|ui| {
                                                    ui.strong("N");
                                                });
                                                header.col(|ui| {
                                                    ui.strong("Name");
                                                });
                                                header.col(|ui| {
                                                    ui.strong("Count");
                                                });
                                                header.col(|ui| {
                                                    ui.strong("Count Type");
                                                });
                                                header.col(|ui| {
                                                    ui.strong("Price");
                                                });
                                                header.col(|ui| {
                                                    ui.strong("Summ");
                                                });
                                                header.col(|ui| {
                                                    ui.strong("Vat type");
                                                });
                                                header.col(|ui| {
                                                    ui.strong("Vat");
                                                });
                                            })
                                            .body(|mut body| {
                                                // let contents =
                                                //     |ui: &mut eframe::egui::Ui,
                                                //      text: String,
                                                //      response: &mut Option<Response>| {
                                                //         App::response_compare(ui.label(text), response);
                                                //     };
                                                for i in 0..self.receipt_fields.subjects.len() {
                                                    body.row(30.0, |mut row| {
                                                        //let mut inner_response: Option<Response> = None;
                                                        row.col(|ui| {
                                                            ui.label(format!("'{}'", i));
                                                        });

                                                        row.col(|ui| {
                                                            ui.add(egui::TextEdit::singleline(
                                                                &mut self.receipt_fields.subjects
                                                                    [i]
                                                                    .name,
                                                            ));
                                                        });

                                                        row.col(|ui| {
                                                            ui.add(egui::TextEdit::singleline(
                                                                &mut self.receipt_fields.subjects
                                                                    [i]
                                                                    .count
                                                                    .to_string(),
                                                            ));
                                                        });

                                                        row.col(|ui| {
                                                            egui::ComboBox::from_label(
                                                                "Select unit type!",
                                                            )
                                                            .selected_text(format!(
                                                                "{:?}",
                                                                self.receipt_fields.subjects[i]
                                                                    .unit_type
                                                            ))
                                                            .show_ui(ui, |ui| {
                                                                ui.selectable_value(
                                                                    &mut self
                                                                        .receipt_fields
                                                                        .subjects[i]
                                                                        .unit_type,
                                                                    receipt::UnitType::Pieces,
                                                                    "Pieces",
                                                                );
                                                                ui.selectable_value(
                                                                    &mut self
                                                                        .receipt_fields
                                                                        .subjects[i]
                                                                        .unit_type,
                                                                    receipt::UnitType::Gramm,
                                                                    "Gramm",
                                                                );
                                                                ui.selectable_value(
                                                                    &mut self
                                                                        .receipt_fields
                                                                        .subjects[i]
                                                                        .unit_type,
                                                                    receipt::UnitType::Kilogamm,
                                                                    "Kilogamm",
                                                                );
                                                            });
                                                        });

                                                        row.col(|ui| {
                                                            ui.add(egui::TextEdit::singleline(
                                                                &mut self.receipt_fields.subjects
                                                                    [i]
                                                                    .price
                                                                    .to_string(),
                                                            ));
                                                        });

                                                        row.col(|ui| {
                                                            ui.add(egui::TextEdit::singleline(
                                                                &mut self.receipt_fields.subjects
                                                                    [i]
                                                                    .summary
                                                                    .to_string(),
                                                            ));
                                                        });

                                                        row.col(|ui| {
                                                            egui::ComboBox::from_label(
                                                                "Select Vat type!",
                                                            )
                                                            .selected_text(format!(
                                                                "{:?}",
                                                                self.receipt_fields.subjects[i]
                                                                    .vat_type
                                                            ))
                                                            .show_ui(ui, |ui| {
                                                                ui.selectable_value(
                                                                    &mut self
                                                                        .receipt_fields
                                                                        .subjects[i]
                                                                        .vat_type,
                                                                    receipt::VatType::Vat0,
                                                                    "Initial",
                                                                );
                                                                ui.selectable_value(
                                                                    &mut self
                                                                        .receipt_fields
                                                                        .subjects[i]
                                                                        .vat_type,
                                                                    receipt::VatType::Vat5,
                                                                    "Buy",
                                                                );
                                                                ui.selectable_value(
                                                                    &mut self
                                                                        .receipt_fields
                                                                        .subjects[i]
                                                                        .vat_type,
                                                                    receipt::VatType::Vat7,
                                                                    "Sell",
                                                                );
                                                                ui.selectable_value(
                                                                    &mut self
                                                                        .receipt_fields
                                                                        .subjects[i]
                                                                        .vat_type,
                                                                    receipt::VatType::Vat10,
                                                                    "Debeting Accounts",
                                                                );
                                                                ui.selectable_value(
                                                                    &mut self
                                                                        .receipt_fields
                                                                        .subjects[i]
                                                                        .vat_type,
                                                                    receipt::VatType::Vat20,
                                                                    "Withdrawal From Account",
                                                                );
                                                            });
                                                        });

                                                        row.col(|ui| {
                                                            ui.add(egui::TextEdit::singleline(
                                                                &mut self.receipt_fields.subjects
                                                                    [i]
                                                                    .vat
                                                                    .to_string(),
                                                            ));
                                                        });
                                                    });
                                                }
                                            });
                                    });
                                });

                            if ui.button("Add row").clicked() {
                                &mut self.receipt_fields.subjects.push(receipt::Subject::empty());
                            }

                            //
                            ui.label("summary");
                            ui.add(egui::TextEdit::singleline(&mut self.receipt_fields.summary));
                            ui.label("cash");
                            ui.add(egui::TextEdit::singleline(&mut self.receipt_fields.cash));
                            ui.label("cashless");
                            ui.add(egui::TextEdit::singleline(
                                &mut self.receipt_fields.cashless,
                            ));
                            ui.label("prepayment");
                            ui.add(egui::TextEdit::singleline(
                                &mut self.receipt_fields.prepayment,
                            ));
                            ui.label("postpayment");
                            ui.add(egui::TextEdit::singleline(
                                &mut self.receipt_fields.postpayment,
                            ));
                            ui.label("in_kind");
                            ui.add(egui::TextEdit::singleline(&mut self.receipt_fields.in_kind));
                            ui.label("vat");
                            ui.add(egui::TextEdit::singleline(&mut self.receipt_fields.vat));
                            ui.label("url");
                            ui.add(egui::TextEdit::singleline(&mut self.receipt_fields.url));

                            if ui.button("Apply").clicked() {
                                let iter = self
                                    .db
                                    .receipts
                                    .iter_mut()
                                    .find(|receipt| receipt.id == rec_id);
                                let time = chrono::NaiveTime::from_hms_opt(
                                    self.operation_fields.hour,
                                    self.operation_fields.minute,
                                    0,
                                )
                                .unwrap();
                                if let Some(element) = iter {
                                    element.date_time = chrono::NaiveDateTime::new(
                                        self.operation_fields.date,
                                        time,
                                    );
                                    element.calculation_type = self.receipt_fields.calculation_type;
                                    if self.receipt_fields.address == "".to_string() {
                                        element.address = None;
                                    } else {
                                        element.address = Some(self.receipt_fields.address.clone());
                                    }
                                    if self.receipt_fields.place == "".to_string() {
                                        element.place = None;
                                    } else {
                                        element.place = Some(self.receipt_fields.place.clone());
                                    }
                                    element.subjects = Vec::new();
                                    for j in 0..self.receipt_fields.subjects.len() {
                                        element
                                            .subjects
                                            .push(self.receipt_fields.subjects[j].clone());
                                    }
                                    element.summary = Decimal::from_str_exact(
                                        &self.receipt_fields.summary.clone(),
                                    )
                                    .unwrap();
                                    if self.receipt_fields.cash == ""
                                        || self.receipt_fields.cash == "0"
                                    {
                                        element.cash = None;
                                    } else {
                                        element.cash = Some(
                                            Decimal::from_str_exact(
                                                &self.receipt_fields.cash.clone(),
                                            )
                                            .unwrap(),
                                        );
                                    }
                                    if self.receipt_fields.cashless == ""
                                        || self.receipt_fields.cashless == "0"
                                    {
                                        element.cashless = None;
                                    } else {
                                        element.cashless = Some(
                                            Decimal::from_str_exact(
                                                &self.receipt_fields.cashless.clone(),
                                            )
                                            .unwrap(),
                                        );
                                    }
                                    if self.receipt_fields.prepayment == ""
                                        || self.receipt_fields.prepayment == "0"
                                    {
                                        element.prepayment = None;
                                    } else {
                                        element.prepayment = Some(
                                            Decimal::from_str_exact(
                                                &self.receipt_fields.prepayment.clone(),
                                            )
                                            .unwrap(),
                                        );
                                    }
                                    if self.receipt_fields.postpayment == ""
                                        || self.receipt_fields.postpayment == "0"
                                    {
                                        element.postpayment = None;
                                    } else {
                                        element.postpayment = Some(
                                            Decimal::from_str_exact(
                                                &self.receipt_fields.postpayment.clone(),
                                            )
                                            .unwrap(),
                                        );
                                    }
                                    if self.receipt_fields.in_kind == ""
                                        || self.receipt_fields.in_kind == "0"
                                    {
                                        element.in_kind = None;
                                    } else {
                                        element.in_kind = Some(
                                            Decimal::from_str_exact(
                                                &self.receipt_fields.in_kind.clone(),
                                            )
                                            .unwrap(),
                                        );
                                    }
                                    if self.receipt_fields.vat == ""
                                        || self.receipt_fields.vat == "0"
                                    {
                                        element.vat = None;
                                    } else {
                                        element.vat = Some(
                                            Decimal::from_str_exact(
                                                &self.receipt_fields.vat.clone(),
                                            )
                                            .unwrap(),
                                        );
                                    }
                                    if self.receipt_fields.url == "".to_string() {
                                        element.url = None;
                                    } else {
                                        element.url = Some(self.receipt_fields.url.clone());
                                    }
                                } else {
                                    let mut element = Receipt::empty_new();
                                    element.id = rec_id;
                                    element.date_time = chrono::NaiveDateTime::new(
                                        self.operation_fields.date,
                                        time,
                                    );
                                    element.calculation_type = self.receipt_fields.calculation_type;
                                    if self.receipt_fields.address == "".to_string() {
                                        element.address = None;
                                    } else {
                                        element.address = Some(self.receipt_fields.address.clone());
                                    }
                                    if self.receipt_fields.place == "".to_string() {
                                        element.place = None;
                                    } else {
                                        element.place = Some(self.receipt_fields.place.clone());
                                    }
                                    element.subjects = Vec::new();
                                    for j in 0..self.receipt_fields.subjects.len() {
                                        element
                                            .subjects
                                            .push(self.receipt_fields.subjects[j].clone());
                                    }
                                    element.summary = Decimal::from_str_exact(
                                        &self.receipt_fields.summary.clone(),
                                    )
                                    .unwrap();
                                    if self.receipt_fields.cash == ""
                                        || self.receipt_fields.cash == "0"
                                    {
                                        element.cash = None;
                                    } else {
                                        element.cash = Some(
                                            Decimal::from_str_exact(
                                                &self.receipt_fields.cash.clone(),
                                            )
                                            .unwrap(),
                                        );
                                    }
                                    if self.receipt_fields.cashless == ""
                                        || self.receipt_fields.cashless == "0"
                                    {
                                        element.cashless = None;
                                    } else {
                                        element.cashless = Some(
                                            Decimal::from_str_exact(
                                                &self.receipt_fields.cashless.clone(),
                                            )
                                            .unwrap(),
                                        );
                                    }
                                    if self.receipt_fields.prepayment == ""
                                        || self.receipt_fields.prepayment == "0"
                                    {
                                        element.prepayment = None;
                                    } else {
                                        element.prepayment = Some(
                                            Decimal::from_str_exact(
                                                &self.receipt_fields.prepayment.clone(),
                                            )
                                            .unwrap(),
                                        );
                                    }
                                    if self.receipt_fields.postpayment == ""
                                        || self.receipt_fields.postpayment == "0"
                                    {
                                        element.postpayment = None;
                                    } else {
                                        element.postpayment = Some(
                                            Decimal::from_str_exact(
                                                &self.receipt_fields.postpayment.clone(),
                                            )
                                            .unwrap(),
                                        );
                                    }
                                    if self.receipt_fields.in_kind == ""
                                        || self.receipt_fields.in_kind == "0"
                                    {
                                        element.in_kind = None;
                                    } else {
                                        element.in_kind = Some(
                                            Decimal::from_str_exact(
                                                &self.receipt_fields.in_kind.clone(),
                                            )
                                            .unwrap(),
                                        );
                                    }
                                    if self.receipt_fields.vat == ""
                                        || self.receipt_fields.vat == "0"
                                    {
                                        element.vat = None;
                                    } else {
                                        element.vat = Some(
                                            Decimal::from_str_exact(
                                                &self.receipt_fields.vat.clone(),
                                            )
                                            .unwrap(),
                                        );
                                    }
                                    if self.receipt_fields.url == "".to_string() {
                                        element.url = None;
                                    } else {
                                        element.url = Some(self.receipt_fields.url.clone());
                                    }
                                    self.db.receipts.push(element);
                                }
                                if signal {
                                    self.operation_fields.date = self.receipt_fields.date;
                                    self.operation_fields.hour = self.receipt_fields.hour;
                                    self.operation_fields.minute = self.receipt_fields.minute;
                                    match self.receipt_fields.calculation_type {
                                        receipt::CalculationType::Inbound => {
                                            self.operation_fields.direction =
                                                operation::FinanseDirection::Credit;
                                            self.operation_fields.operation_type =
                                                OperationType::Buy;
                                        }
                                        receipt::CalculationType::Outbound => {
                                            self.operation_fields.direction =
                                                operation::FinanseDirection::Debet;
                                            self.operation_fields.operation_type =
                                                OperationType::Sell;
                                        }
                                        receipt::CalculationType::InboundReturn => {
                                            self.operation_fields.direction =
                                                operation::FinanseDirection::Debet;
                                            self.operation_fields.operation_type =
                                                OperationType::ReturnBuy;
                                        }
                                        receipt::CalculationType::OutboundReturn => {
                                            self.operation_fields.direction =
                                                operation::FinanseDirection::Credit;
                                            self.operation_fields.operation_type =
                                                OperationType::ReturnSell;
                                        }
                                    }
                                    self.operation_fields.summary =
                                        self.receipt_fields.summary.clone();
                                    self.operation_fields.receipt = Some(rec_id);
                                }
                                close_request = true;
                            }
                        });

                        if ctx.input(|i| i.viewport().close_requested()) || close_request {
                            // Tell parent viewport that we should not show next frame:
                            // self.show_immediate_viewport = false;
                            self.receipt_fields = ReceiptFields::new();
                            self.statement = Statement::EditOperation(op_id);
                        }
                    },
                );
            }

            Statement::ThripleDialog => {
                todo!()
            }
        }
    }
}
