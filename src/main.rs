#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use chrono::{Date, Local, NaiveDateTime, Utc};
use egui_extras::{Column, Size, StripBuilder, TableBuilder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{account::Account, database::Database, operation::Operation};

use eframe::egui::{self, Button, Response, Ui, response};

mod account;
mod database;
mod operation;
mod receipt;

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
    EditReceipt,
    ThripleDialog,
}

struct AccountFields {
    name: String,
    account_type: account::AccountType,
    number: String,
    bik: String,
    sum: usize,
}

impl AccountFields {
    fn new() -> Self {
        Self {
            name: "".to_string(),
            account_type: account::AccountType::Cash,
            number: "".to_string(),
            bik: "100000000".to_string(),
            sum: 0,
        }
    }
}

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([720.0, 480.0]),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| {
            // This gives us image support:
            //egui_extras::install_image_loaders(&cc.egui_ctx);

            let app = App::new();
            Ok(Box::new(app))
        }),
    )
}

struct App {
    db: Database,
    selected: Option<Selection>,
    statement: Statement,
    account_fields: AccountFields,
}

impl App {
    fn new() -> Self {
        Self {
            db: Database::load("/home/user/rust_projects/file.json".to_string()),
            selected: None,
            statement: Statement::Common,
            account_fields: AccountFields::new(),
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
            .size(Size::exact(100.0)) // for the table
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
                        .max_scroll_height(200.0)
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
                            Selection::Account(uuid) => ui.heading("Аккаунт"),
                            Selection::Operation(uuid) => ui.heading("Операция"),
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
                            Selection::Operation(uuid) => {}
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
                    self.db.save("/home/user/rust_projects/file.json");
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
                        .with_title("Edit Account")
                        .with_inner_size([200.0, 100.0]),
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
                ctx.show_viewport_immediate(
                    egui::ViewportId::from_hash_of("operation creating"),
                    egui::ViewportBuilder::default()
                        .with_title("New Account")
                        .with_inner_size([200.0, 100.0]),
                    |ctx, class| {
                        assert!(
                            class == egui::ViewportClass::Immediate,
                            "This egui backend doesn't support multiple viewports"
                        );

                        egui::CentralPanel::default().show(ctx, |ui| {
                            ui.label("Hello from immediate viewport");
                            let mut input_string: String = "".to_string();

                            let response = ui.add(egui::TextEdit::singleline(&mut input_string));
                            //if response.changed() {}

                            // self.db.add_account(name, account_type, number, bik);
                        });

                        if ctx.input(|i| i.viewport().close_requested()) {
                            // Tell parent viewport that we should not show next frame:
                            // self.show_immediate_viewport = false;
                            self.statement = Statement::Common;
                        }
                    },
                );
            }
            Statement::ThripleDialog => {
                todo!()
            }
            Statement::EditReceipt => todo!(),
        }
    }
}

// let window = eframe::egui::Window::new("New Account");
// let window2 = window.show(ctx, |ui| ui.label("text"));
