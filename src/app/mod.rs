#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use chrono::NaiveDate;

use uuid::Uuid;

use crate::{
    account::{self},
    app::cbox::*,
    database::*,
    operation::*,
    receipt::{self},
};

use eframe::egui::{self};

mod cbox;
mod compare;
mod context;
mod table;

enum Selection {
    Account(Uuid),
    Operation(Uuid),
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
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| context::main_central_panel(self, ui));
        egui::SidePanel::right("right_panel")
            .resizable(true)
            .default_width(150.0)
            .width_range(150.0..=200.0)
            .show(ctx, |ui| context::main_right_panel(self, ui));
        egui::TopBottomPanel::bottom("bottom_panel")
            .resizable(false)
            .min_height(0.0)
            .show(ctx, |ui| context::main_bottom_panel(self, ui));

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
                    |ctx, class| context::account(self, acc_id, ctx, class),
                );
            }
            Statement::EditOperation(uuid) => {
                let op_id = uuid.clone();
                ctx.show_viewport_immediate(
                    egui::ViewportId::from_hash_of("operation window"),
                    egui::ViewportBuilder::default()
                        .with_title("Operation")
                        .with_inner_size([400.0, 400.0]),
                    |ctx, class| context::operation(self, op_id, ctx, class),
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
                    |ctx, class| context::receipt(self, rec_id, op_id, signal, ctx, class),
                );
            }

            Statement::ThripleDialog => {
                todo!()
            }
        }
    }
}

// let window = eframe::egui::Window::new("New Account");
// let window2 = window.show(ctx, |ui| ui.label("text"));
