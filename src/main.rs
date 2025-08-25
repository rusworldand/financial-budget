#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use std::{env, ops::Deref};

use chrono::{Date, DateTime, Local, NaiveDate, NaiveDateTime, NaiveTime, Timelike, Utc};
use egui_extras::{Column, Size, StripBuilder, TableBuilder};
use rust_decimal::{self, Decimal, dec};
use uuid::Uuid;

use crate::{
    account::Account,
    app::App,
    database::Database,
    operation::{FinanseDirection, Operation, OperationType},
    receipt::{Receipt, VatType},
};

use eframe::egui::{self, Response, Ui};

mod account;
mod app;
mod database;
mod operation;
mod receipt;

fn main() -> eframe::Result {
    let args: Vec<String> = env::args().collect();
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

            let app = App::new(args.get(1));
            Ok(Box::new(app))
        }),
    )
}

// let window = eframe::egui::Window::new("New Account");
// let window2 = window.show(ctx, |ui| ui.label("text"));
