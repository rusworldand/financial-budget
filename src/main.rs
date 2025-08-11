#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use chrono::{Date, Local, NaiveDateTime, Utc};
use egui_extras::{Column, Size, StripBuilder, TableBuilder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{account::Account, database::Database, database::Operations, operation::Operation};

use eframe::egui::{self, Response, Ui, response};

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
// fn main() {
//     let mut db = Database::new();

//     println!("Hello, world!");
// }

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
}

impl App {
    fn new() -> Self {
        Self {
            db: Database::load("/home/user/rust_projects/file.json".to_string()),
            selected: None,
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

                                            // if let Some(response) = &mut inner_response {
                                            //     *response = response.union(row_response)
                                            // } else {
                                            //     inner_response = Some(row_response)
                                            // }

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
            });
    }
}
