use crate::app::{self, compare::response_compare};
use app::Selection;
use eframe::egui::{self, Response, Ui};
use egui_extras::{Column, Size, StripBuilder, TableBuilder}; //

pub enum TableType {
    Account,
    Operation,
}

pub fn table(app: &mut app::App, table_type: TableType, ui: &mut Ui) -> Response {
    match table_type {
        TableType::Account => Some(ui.label(format!("Accounts"))),
        TableType::Operation => Some(ui.label(format!("Operations"))),
    };

    StripBuilder::new(ui)
        .size(Size::exact(250.0)) // for the table
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
                                response_compare(ui.label(text), response);
                            };
                        match table_type {
                            TableType::Account => {
                                for i in &app.db.accounts {
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

                                        response_compare(row_response, &mut inner_response);

                                        if let Some(response) = inner_response {
                                            if response.double_clicked() {
                                                println!("Double!");
                                                println!("{}", i.id);
                                                app.selected = Some(Selection::Account(i.id))
                                            }
                                            if response.triple_clicked() {
                                                println!("Triple!");
                                                println!("{}", i.id);
                                                app.selected = Some(Selection::Account(i.id))
                                            }
                                        }
                                    });
                                }
                            }
                            TableType::Operation => {
                                for i in &app.db.operations {
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

                                        response_compare(row_response, &mut inner_response);
                                        if let Some(response) = inner_response {
                                            if response.double_clicked() {
                                                println!("Double!");
                                                println!("{}", i.id);
                                                app.selected = Some(Selection::Operation(i.id))
                                            }
                                            if response.triple_clicked() {
                                                println!("Triple!");
                                                println!("{}", i.id);
                                                app.selected = Some(Selection::Operation(i.id))
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
