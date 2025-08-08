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
    fn table(&mut self, table_type: TableType, ui: &mut Ui) -> Response {
        let mut response: Option<Response> = match table_type {
            TableType::Account => Some(ui.label(format!("Accounts"))),
            TableType::Operation => Some(ui.label(format!("Operations"))),
        };

        StripBuilder::new(ui)
            .size(Size::exact(100.0)) // for the table
            .vertical(|mut strip| {
                strip.cell(|ui| {
                    let mut table = TableBuilder::new(ui)
                        .resizable(true)
                        .striped(true)
                        .id_salt("operations_table")
                        .cell_layout(egui::Layout::left_to_right(egui::Align::Center));
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
                            let mut inner_response: Option<Response> = None;
                            //let mut temp_response: Option<Response> = None;
                            let mut contents = |ui: &mut eframe::egui::Ui, text: String| {
                                let variable = ui.label(text);
                                if let Some(response) = &mut temp_response {
                                    *response = response.union(variable)
                                } else {
                                    temp_response = Some(variable)
                                }
                            };
                            match table_type {
                                TableType::Account => {
                                    for i in &self.db.accounts {
                                        body.row(30.0, |mut row| {
                                            row.col(|ui| contents(ui, format!("ID '{}'", i.id)));

                                            row.col(|ui| {
                                                contents(ui, format!("Name '{}'", i.name))
                                            });

                                            let row_response = row.response();

                                            if let Some(response) = &mut inner_response {
                                                *response = response.union(row_response)
                                            } else {
                                                inner_response = Some(row_response)
                                            }
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
                                            row.col(|ui| {
                                                contents(ui, format!("ID '{}'", i.account))
                                            });

                                            row.col(|ui| {
                                                contents(
                                                    ui,
                                                    format!("Name '{}'", i.operation.date_time),
                                                )
                                            });

                                            row.col(|ui| {
                                                contents(
                                                    ui,
                                                    format!("Name '{}'", i.operation.summary),
                                                )
                                            });
                                            let row_response = row.response();

                                            if let Some(response) = &mut inner_response {
                                                *response = response.union(row_response)
                                            } else {
                                                inner_response = Some(row_response)
                                            }
                                            if let Some(response) = response {
                                                if response.double_clicked() {
                                                    println!("Double!");
                                                    println!("{}", i.account);
                                                    self.selected =
                                                        Some(Selection::Operation(i.account))
                                                }
                                                if response.triple_clicked() {
                                                    println!("Triple!");
                                                    println!("{}", i.account);
                                                    self.selected =
                                                        Some(Selection::Operation(i.account))
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

            ui.label(format!("Accounts"));

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
                                                println!("Double!");
                                                println!("{}", i.id);
                                                self.selected = Some(Selection::Account(i.id))
                                            }
                                            if response.triple_clicked() {
                                                println!("Triple!");
                                                println!("{}", i.id);
                                                self.selected = Some(Selection::Account(i.id))
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
                .size(Size::exact(200.0)) // for the table
                .vertical(|mut strip| {
                    strip.cell(|ui| {
                        let mut table = TableBuilder::new(ui)
                            .resizable(true)
                            .striped(true)
                            .id_salt("account_table")
                            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                            .column(Column::auto())
                            .column(Column::auto())
                            .column(Column::auto())
                            .min_scrolled_height(0.0)
                            .max_scroll_height(100.0)
                            .sense(egui::Sense::click());

                        table
                            .header(30.0, |mut header| {
                                header.col(|ui| {
                                    ui.strong("Account");
                                });
                                header.col(|ui| {
                                    ui.strong("Date / Time");
                                });
                                header.col(|ui| {
                                    ui.strong("Sum");
                                });
                            })
                            .body(|mut body| {
                                for i in &self.db.operations {
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
                                        row.col(|ui| contents(ui, format!("ID '{}'", i.account)));

                                        row.col(|ui| {
                                            contents(
                                                ui,
                                                format!("Name '{}'", i.operation.date_time),
                                            )
                                        });

                                        row.col(|ui| {
                                            contents(ui, format!("Name '{}'", i.operation.summary))
                                        });

                                        let row_response = row.response();

                                        if let Some(response) = &mut response {
                                            *response = response.union(row_response)
                                        } else {
                                            response = Some(row_response)
                                        }
                                        if let Some(response) = response {
                                            if response.double_clicked() {
                                                println!("Double!");
                                                println!("{}", i.account);
                                                self.selected =
                                                    Some(Selection::Operation(i.account))
                                            }
                                            if response.triple_clicked() {
                                                println!("Triple!");
                                                println!("{}", i.account);
                                                self.selected =
                                                    Some(Selection::Operation(i.account))
                                            }
                                        };
                                    });
                                }
                            });
                    });
                });
            //
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
                                    .find(|operation| operation.operation.id == *uuid);
                                if let Some(element) = iter {
                                    ui.label(format!("{}", element.operation.id));
                                }
                            }
                        }
                    }
                });
            });
    }
}
