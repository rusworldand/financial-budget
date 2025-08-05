#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use chrono::{Date, Local, Utc};
use egui_extras::{Column, Size, StripBuilder, TableBuilder};
use serde::{Deserialize, Serialize};

use crate::{account::Account, database::Database, database::Operations, operation::Operation};

use eframe::egui::{self, Response, response};

mod account;
mod database;
mod operation;
mod receipt;

// fn main() {
//     let mut db = Database::new();

//     println!("Hello, world!");
// }

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
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
}

impl App {
    fn new() -> Self {
        Self {
            db: Database::load("/home/user/rust_projects/file.json".to_string()),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");

            //ui.label(format!("Hello '{}', age {}", self.name, self.age));
            ui.label(format!("Accounts"));

            // for i in &self.db.accounts {
            //     ui.label(format!(
            //         "ID '{}', name {}, type {:?}, number {}, bik {}, sum {}",
            //         i.id, i.name, i.account_type, i.number, i.bik, i.sum
            //     ));
            // }
            // ui.label(format!("Operations"));
            // for Operations { account, operation } in &self.db.operations {
            //     ui.label(format!(
            //         "ID '{}', date time {}, type {:?}, summary {}, direction {:?}, receipt {:#?}",
            //         account,
            //         operation.date_time,
            //         operation.operation_type,
            //         operation.summary,
            //         operation.direction,
            //         operation.receipt
            //     ));
            // }
            //
            StripBuilder::new(ui)
                .size(Size::exact(100.0)) // for the table
                .vertical(|mut strip| {
                    strip.cell(|ui| {
                        let mut table = TableBuilder::new(ui)
                            .resizable(true)
                            .striped(true)
                            .id_salt("account_table")
                            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                            .column(Column::auto())
                            .column(Column::auto())
                            .min_scrolled_height(0.0)
                            .max_scroll_height(80.0)
                            .sense(egui::Sense::click());

                        table
                            .header(30.0, |mut header| {
                                header.col(|ui| {
                                    ui.strong("ID");
                                });
                                header.col(|ui| {
                                    ui.strong("Name");
                                });
                            })
                            .body(|mut body| {
                                for i in &self.db.accounts {
                                    body.row(30.0, |mut row| {
                                        let mut response: Option<Response> = None;
                                        let mut contents =
                                            |ui: &mut eframe::egui::Ui, text: String| {
                                                let variable = ui.label(text);
                                                if let Some(response) = &mut response {
                                                    *response = response.union(variable)
                                                } else {
                                                    response = Some(variable)
                                                }
                                            };
                                        row.col(|ui| contents(ui, format!("ID '{}'", i.id)));

                                        row.col(|ui| contents(ui, format!("Name '{}'", i.name)));

                                        let row_response = row.response();

                                        if let Some(response) = &mut response {
                                            *response = response.union(row_response)
                                        } else {
                                            response = Some(row_response)
                                        }
                                        if let Some(response) = response {
                                            if response.double_clicked() {
                                                println!("Double!")
                                            }
                                            if response.triple_clicked() {
                                                println!("Triple!")
                                            }
                                        };
                                    });
                                }
                            });
                    });
                });

            //ui.push_id(id_salt, add_contents)

            ui.separator();
            ui.label(format!("Operations"));

            StripBuilder::new(ui)
                .size(Size::exact(100.0)) // for the table
                .vertical(|mut strip| {
                    strip.cell(|ui| {
                        let mut table = TableBuilder::new(ui)
                            .resizable(true)
                            .striped(true)
                            .id_salt("operations_table")
                            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                            .column(Column::auto())
                            .column(Column::auto())
                            .min_scrolled_height(0.0)
                            .max_scroll_height(80.0)
                            .sense(egui::Sense::click());

                        table
                            .header(30.0, |mut header| {
                                header.col(|ui| {
                                    ui.strong("ID");
                                    // ui.push_id(id_salt, add_contents)
                                });
                                header.col(|ui| {
                                    ui.strong("Name");
                                });
                            })
                            .body(|mut body| {
                                for i in &self.db.accounts {
                                    body.row(30.0, |mut row| {
                                        let mut response: Option<Response> = None;
                                        let mut contents =
                                            |ui: &mut eframe::egui::Ui, text: String| {
                                                let variable = ui.label(text);
                                                if let Some(response) = &mut response {
                                                    *response = response.union(variable)
                                                } else {
                                                    response = Some(variable)
                                                }
                                            };
                                        row.col(|ui| contents(ui, format!("ID '{}'", i.id)));

                                        row.col(|ui| contents(ui, format!("Name '{}'", i.name)));

                                        let row_response = row.response();

                                        if let Some(response) = &mut response {
                                            *response = response.union(row_response)
                                        } else {
                                            response = Some(row_response)
                                        }
                                        if let Some(response) = response {
                                            if response.double_clicked() {
                                                println!("Double!")
                                            }
                                            if response.triple_clicked() {
                                                println!("Triple!")
                                            }
                                        };
                                    });
                                }
                            });
                    });
                });
            //
        });
    }
}
