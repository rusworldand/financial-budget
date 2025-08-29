use chrono::{Date, DateTime, Local, NaiveDate, NaiveDateTime, NaiveTime, Timelike, Utc};
use eframe::egui::{self, Response, Ui};
use egui_extras::{Column, Size, StripBuilder, TableBuilder};
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::{
    account::Account,
    app::{
        AccountFields, App, OperationFields, ReceiptFields, Selection, Statement, cbox,
        table::{self, TableType},
    },
    operation::{self, Operation, OperationType},
    receipt::{self, Receipt},
};

pub fn main_central_panel(app: &mut App, ui: &mut Ui) {
    ui.heading("My egui Application");
    //ui.push_id(id_salt, add_contents)
    table::table(app, TableType::Account, ui);
    ui.separator();
    table::table(app, TableType::Operation, ui);
}

pub fn main_right_panel(app: &mut App, ui: &mut Ui) {
    ui.vertical_centered(|ui| {
        if let Some(selection) = &app.selected {
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
        if let Some(selection) = &app.selected {
            match selection {
                Selection::Account(uuid) => {
                    let iter = &app.db.accounts.iter().find(|account| account.id == *uuid);
                    if let Some(element) = iter {
                        ui.label(format!("{}", element.id));
                    }
                }
                Selection::Operation(uuid) => {
                    let iter = &app
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
    if let Some(selection) = &app.selected {
        if ui.button("Edit").clicked() {
            match selection {
                Selection::Account(uuid) => {
                    let iter = &app
                        .db
                        .accounts
                        .iter()
                        .find(|account| account.id == *uuid)
                        .unwrap();

                    app.account_fields.name = iter.name.clone();
                    app.account_fields.account_type = iter.account_type.clone();
                    app.account_fields.number = iter.number.clone();
                    app.account_fields.bik = iter.bik.to_string();
                    app.statement = Statement::EditAccount(*uuid);
                }
                Selection::Operation(uuid) => {
                    let iter = &app
                        .db
                        .operations
                        .iter()
                        .find(|operation| operation.id == *uuid)
                        .unwrap();

                    app.operation_fields.date = iter.date_time.date();
                    app.operation_fields.hour = iter.date_time.time().hour();
                    app.operation_fields.minute = iter.date_time.time().minute();
                    app.operation_fields.account_id = iter.account_id;
                    app.operation_fields.operation_type = iter.operation_type.clone();
                    app.operation_fields.summary = iter.summary.to_string();
                    app.operation_fields.direction = iter.direction.clone();
                    app.statement = Statement::EditOperation(*uuid);
                }
            }
        }
    }
}

pub fn main_bottom_panel(app: &mut App, ui: &mut Ui) {
    {
        if ui.button("New Account").clicked() {
            app.statement = Statement::EditAccount(Uuid::new_v4());
            app.account_fields = AccountFields::new();
        }
        if ui.button("New Operation").clicked() {
            app.statement = Statement::EditOperation(Uuid::new_v4());
            app.operation_fields = OperationFields::new();
        }
        if ui.button("Save").clicked() {
            app.db.save(&app.file);
        }
    }
}

pub fn account(app: &mut App, acc_id: Uuid, ctx: &egui::Context, class: egui::ViewportClass) {
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
        ui.add(egui::TextEdit::singleline(&mut app.account_fields.name));
        cbox(ui, &mut app.account_fields.account_type, "Select one!");

        ui.add(egui::TextEdit::singleline(&mut app.account_fields.number).char_limit(30));
        ui.add(egui::TextEdit::singleline(&mut app.account_fields.bik).char_limit(9));
        if ui.button("Apply").clicked() {
            let iter = app
                .db
                .accounts
                .iter_mut()
                .find(|account| account.id == acc_id);
            if let Some(element) = iter {
                element.account_type = app.account_fields.account_type.clone();
                element.name = app.account_fields.name.clone();
                element.number = app.account_fields.number.clone();
                element.bik = app.account_fields.bik.parse::<u32>().unwrap();
            } else {
                app.db.accounts.push(Account {
                    id: acc_id,
                    name: app.account_fields.name.clone(),
                    account_type: app.account_fields.account_type.clone(),
                    number: app.account_fields.number.clone(),
                    bik: app.account_fields.bik.parse::<u32>().unwrap(),
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
        app.account_fields = AccountFields::new();
        app.statement = Statement::Common;
    }
}

pub fn operation(app: &mut App, op_id: Uuid, ctx: &egui::Context, class: egui::ViewportClass) {
    assert!(
        class == egui::ViewportClass::Immediate,
        "This egui backend doesn't support multiple viewports"
    );
    let mut close_request: bool = false;
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.label("Date and time");
        ui.add(egui_extras::DatePickerButton::new(
            &mut app.operation_fields.date,
        ));
        ui.add(
            egui::DragValue::new(&mut app.operation_fields.hour)
                .speed(1)
                .range(0..=23),
        );
        ui.add(
            egui::DragValue::new(&mut app.operation_fields.minute)
                .speed(1)
                .range(0..=59),
        );

        ui.label("Account");
        egui::ComboBox::from_label("Select account!")
            .selected_text(format!("{:?}", app.operation_fields.account_id))
            .show_ui(ui, |ui| {
                for element in app.db.accounts.iter() {
                    ui.selectable_value(
                        &mut app.operation_fields.account_id,
                        element.id,
                        format!("{}", element.name),
                    );
                }
            });
        ui.label("Operation type");
        cbox(ui, &mut app.operation_fields.operation_type, "Select type!");
        ui.label("Summ");
        ui.add(egui::TextEdit::singleline(
            &mut app.operation_fields.summary,
        ));

        ui.label("Direction");
        cbox(ui, &mut app.operation_fields.direction, "Select direction!");

        if ui.button("Receipt").clicked() {
            app.receipt_fields = ReceiptFields::new();
            let iter = app
                .db
                .operations
                .iter_mut()
                .find(|operation| operation.id == op_id);
            if let None = iter {
                app.statement = Statement::EditReceipt(Uuid::new_v4(), op_id, true)
            } else {
                if let Some(identificator) = app.operation_fields.receipt {
                    app.statement = Statement::EditReceipt(identificator, op_id, false);
                } else {
                    app.statement = Statement::EditReceipt(Uuid::new_v4(), op_id, false)
                }
            }
        }

        if ui.button("Apply").clicked() {
            let iter = app
                .db
                .operations
                .iter_mut()
                .find(|operation| operation.id == op_id);
            let time = chrono::NaiveTime::from_hms_opt(
                app.operation_fields.hour,
                app.operation_fields.minute,
                0,
            )
            .unwrap();
            if let Some(element) = iter {
                element.date_time = chrono::NaiveDateTime::new(app.operation_fields.date, time);
                element.account_id = app.operation_fields.account_id;
                element.operation_type = app.operation_fields.operation_type.clone();
                element.summary = app.operation_fields.summary.parse::<usize>().unwrap();
                element.direction = app.operation_fields.direction.clone();
            } else {
                app.db.operations.push(Operation {
                    id: op_id,
                    date_time: chrono::NaiveDateTime::new(app.operation_fields.date, time),
                    account_id: app.operation_fields.account_id,
                    operation_type: app.operation_fields.operation_type.clone(),
                    direction: app.operation_fields.direction.clone(),
                    receipt_id: None,
                    summary: app.operation_fields.summary.parse::<usize>().unwrap(),
                });
            }
            close_request = true;
        }
    });
    if ctx.input(|i| i.viewport().close_requested()) || close_request {
        app.operation_fields = OperationFields::new();
        app.statement = Statement::Common;
    }
}

pub fn receipt(
    app: &mut App,
    rec_id: Uuid,
    op_id: Uuid,
    signal: bool,
    ctx: &egui::Context,
    class: egui::ViewportClass,
) {
    assert!(
        class == egui::ViewportClass::Immediate,
        "This egui backend doesn't support multiple viewports"
    );
    let mut close_request: bool = false;
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.label("Date and time");
        ui.add(egui_extras::DatePickerButton::new(
            &mut app.receipt_fields.date,
        ));
        ui.add(
            egui::DragValue::new(&mut app.receipt_fields.hour)
                .speed(1)
                .range(0..=23),
        );
        ui.add(
            egui::DragValue::new(&mut app.receipt_fields.minute)
                .speed(1)
                .range(0..=59),
        );
        ui.label("Receipt type");
        cbox(
            ui,
            &mut app.receipt_fields.calculation_type,
            "Select calculation type!",
        );

        ui.label("Adress");
        ui.add(egui::TextEdit::singleline(&mut app.receipt_fields.address));
        ui.label("Point name");
        ui.add(egui::TextEdit::singleline(&mut app.receipt_fields.place));

        StripBuilder::new(ui)
            .size(Size::exact(200.0))
            .vertical(|mut strip| {
                strip.cell(|ui| {
                    let mut table = TableBuilder::new(ui)
                        .resizable(true)
                        .striped(true)
                        .id_salt("subject_table")
                        .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
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
                            for (i, element) in app.receipt_fields.subjects.iter_mut().enumerate() {
                                body.row(30.0, |mut row| {
                                    //let mut inner_response: Option<Response> = None;
                                    row.col(|ui| {
                                        ui.label(format!("'{}'", i));
                                    });

                                    row.col(|ui| {
                                        ui.add(egui::TextEdit::singleline(&mut element.name));
                                    });

                                    row.col(|ui| {
                                        let mut count = element.count.to_string();
                                        if ui.add(egui::TextEdit::singleline(&mut count)).changed()
                                        {
                                            element.count = count.parse().unwrap();
                                        };
                                    });

                                    row.col(|ui| {
                                        cbox(ui, &mut element.unit_type, "Select unit type!");
                                    });

                                    row.col(|ui| {
                                        ui.add(egui::TextEdit::singleline(
                                            &mut element.price.to_string(),
                                        ));
                                    });

                                    row.col(|ui| {
                                        ui.add(egui::TextEdit::singleline(
                                            &mut element.summary.to_string(),
                                        ));
                                    });

                                    row.col(|ui| {
                                        cbox(ui, &mut element.vat_type, "Select Vat type!");
                                    });

                                    row.col(|ui| {
                                        ui.add(egui::TextEdit::singleline(
                                            &mut element.vat.to_string(),
                                        ));
                                    });
                                });
                            }
                        });
                });
            });

        if ui.button("Add row").clicked() {
            &mut app.receipt_fields.subjects.push(receipt::Subject::empty());
        }

        //
        ui.label("summary");
        ui.add(egui::TextEdit::singleline(&mut app.receipt_fields.summary));
        ui.label("cash");
        ui.add(egui::TextEdit::singleline(&mut app.receipt_fields.cash));
        ui.label("cashless");
        ui.add(egui::TextEdit::singleline(&mut app.receipt_fields.cashless));
        ui.label("prepayment");
        ui.add(egui::TextEdit::singleline(
            &mut app.receipt_fields.prepayment,
        ));
        ui.label("postpayment");
        ui.add(egui::TextEdit::singleline(
            &mut app.receipt_fields.postpayment,
        ));
        ui.label("in_kind");
        ui.add(egui::TextEdit::singleline(&mut app.receipt_fields.in_kind));
        ui.label("vat");
        ui.add(egui::TextEdit::singleline(&mut app.receipt_fields.vat));
        ui.label("url");
        ui.add(egui::TextEdit::singleline(&mut app.receipt_fields.url));

        if ui.button("Apply").clicked() {
            let iter = app
                .db
                .receipts
                .iter_mut()
                .find(|receipt| receipt.id == rec_id);
            let time = chrono::NaiveTime::from_hms_opt(
                app.operation_fields.hour,
                app.operation_fields.minute,
                0,
            )
            .unwrap();
            if let Some(element) = iter {
                element.date_time = chrono::NaiveDateTime::new(app.operation_fields.date, time);
                element.calculation_type = app.receipt_fields.calculation_type;
                if app.receipt_fields.address == "".to_string() {
                    element.address = None;
                } else {
                    element.address = Some(app.receipt_fields.address.clone());
                }
                if app.receipt_fields.place == "".to_string() {
                    element.place = None;
                } else {
                    element.place = Some(app.receipt_fields.place.clone());
                }
                element.subjects = Vec::new();
                for j in 0..app.receipt_fields.subjects.len() {
                    element
                        .subjects
                        .push(app.receipt_fields.subjects[j].clone());
                }
                element.summary =
                    Decimal::from_str_exact(&app.receipt_fields.summary.clone()).unwrap();
                if app.receipt_fields.cash == "" || app.receipt_fields.cash == "0" {
                    element.cash = None;
                } else {
                    element.cash =
                        Some(Decimal::from_str_exact(&app.receipt_fields.cash.clone()).unwrap());
                }
                if app.receipt_fields.cashless == "" || app.receipt_fields.cashless == "0" {
                    element.cashless = None;
                } else {
                    element.cashless = Some(
                        Decimal::from_str_exact(&app.receipt_fields.cashless.clone()).unwrap(),
                    );
                }
                if app.receipt_fields.prepayment == "" || app.receipt_fields.prepayment == "0" {
                    element.prepayment = None;
                } else {
                    element.prepayment = Some(
                        Decimal::from_str_exact(&app.receipt_fields.prepayment.clone()).unwrap(),
                    );
                }
                if app.receipt_fields.postpayment == "" || app.receipt_fields.postpayment == "0" {
                    element.postpayment = None;
                } else {
                    element.postpayment = Some(
                        Decimal::from_str_exact(&app.receipt_fields.postpayment.clone()).unwrap(),
                    );
                }
                if app.receipt_fields.in_kind == "" || app.receipt_fields.in_kind == "0" {
                    element.in_kind = None;
                } else {
                    element.in_kind =
                        Some(Decimal::from_str_exact(&app.receipt_fields.in_kind.clone()).unwrap());
                }
                if app.receipt_fields.vat == "" || app.receipt_fields.vat == "0" {
                    element.vat = None;
                } else {
                    element.vat =
                        Some(Decimal::from_str_exact(&app.receipt_fields.vat.clone()).unwrap());
                }
                if app.receipt_fields.url == "".to_string() {
                    element.url = None;
                } else {
                    element.url = Some(app.receipt_fields.url.clone());
                }
            } else {
                let mut element = Receipt::empty_new();
                element.id = rec_id;
                element.date_time = chrono::NaiveDateTime::new(app.operation_fields.date, time);
                element.calculation_type = app.receipt_fields.calculation_type;
                if app.receipt_fields.address == "".to_string() {
                    element.address = None;
                } else {
                    element.address = Some(app.receipt_fields.address.clone());
                }
                if app.receipt_fields.place == "".to_string() {
                    element.place = None;
                } else {
                    element.place = Some(app.receipt_fields.place.clone());
                }
                element.subjects = Vec::new();
                for j in 0..app.receipt_fields.subjects.len() {
                    element
                        .subjects
                        .push(app.receipt_fields.subjects[j].clone());
                }
                element.summary =
                    Decimal::from_str_exact(&app.receipt_fields.summary.clone()).unwrap();
                if app.receipt_fields.cash == "" || app.receipt_fields.cash == "0" {
                    element.cash = None;
                } else {
                    element.cash =
                        Some(Decimal::from_str_exact(&app.receipt_fields.cash.clone()).unwrap());
                }
                if app.receipt_fields.cashless == "" || app.receipt_fields.cashless == "0" {
                    element.cashless = None;
                } else {
                    element.cashless = Some(
                        Decimal::from_str_exact(&app.receipt_fields.cashless.clone()).unwrap(),
                    );
                }
                if app.receipt_fields.prepayment == "" || app.receipt_fields.prepayment == "0" {
                    element.prepayment = None;
                } else {
                    element.prepayment = Some(
                        Decimal::from_str_exact(&app.receipt_fields.prepayment.clone()).unwrap(),
                    );
                }
                if app.receipt_fields.postpayment == "" || app.receipt_fields.postpayment == "0" {
                    element.postpayment = None;
                } else {
                    element.postpayment = Some(
                        Decimal::from_str_exact(&app.receipt_fields.postpayment.clone()).unwrap(),
                    );
                }
                if app.receipt_fields.in_kind == "" || app.receipt_fields.in_kind == "0" {
                    element.in_kind = None;
                } else {
                    element.in_kind =
                        Some(Decimal::from_str_exact(&app.receipt_fields.in_kind.clone()).unwrap());
                }
                if app.receipt_fields.vat == "" || app.receipt_fields.vat == "0" {
                    element.vat = None;
                } else {
                    element.vat =
                        Some(Decimal::from_str_exact(&app.receipt_fields.vat.clone()).unwrap());
                }
                if app.receipt_fields.url == "".to_string() {
                    element.url = None;
                } else {
                    element.url = Some(app.receipt_fields.url.clone());
                }
                app.db.receipts.push(element);
            }
            if signal {
                app.operation_fields.date = app.receipt_fields.date;
                app.operation_fields.hour = app.receipt_fields.hour;
                app.operation_fields.minute = app.receipt_fields.minute;
                match app.receipt_fields.calculation_type {
                    receipt::CalculationType::Inbound => {
                        app.operation_fields.direction = operation::FinanseDirection::Credit;
                        app.operation_fields.operation_type = OperationType::Buy;
                    }
                    receipt::CalculationType::Outbound => {
                        app.operation_fields.direction = operation::FinanseDirection::Debet;
                        app.operation_fields.operation_type = OperationType::Sell;
                    }
                    receipt::CalculationType::InboundReturn => {
                        app.operation_fields.direction = operation::FinanseDirection::Debet;
                        app.operation_fields.operation_type = OperationType::ReturnBuy;
                    }
                    receipt::CalculationType::OutboundReturn => {
                        app.operation_fields.direction = operation::FinanseDirection::Credit;
                        app.operation_fields.operation_type = OperationType::ReturnSell;
                    }
                }
                app.operation_fields.summary = app.receipt_fields.summary.clone();
                app.operation_fields.receipt = Some(rec_id);
            }
            close_request = true;
        }
    });
    if ctx.input(|i| i.viewport().close_requested()) || close_request {
        app.receipt_fields = ReceiptFields::new();
        app.statement = Statement::EditOperation(op_id);
    }
}
