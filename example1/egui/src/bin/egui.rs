use std::fs;
use std::sync::mpsc::channel;
use anyhow::{Context, Result};
use eframe::egui;
use crate::models::pet::PetApp;

#[path = "../models/mod.rs"]
mod models;

#[path = "../dbaccess/mod.rs"]
mod dbaccess;
#[path = "../http/http.rs"]
mod http;
#[path = "../event.rs"]
mod event;

fn load_init_sql() -> std::io::Result<String> {
    fs::read_to_string("./egui/src/sql/pet.sql")
}

fn main() -> Result<()> {
    env_logger::init();

    let init_query = load_init_sql().expect("can not load initial sql");
    let db_con = sqlite::open(":memory:").expect("can not open sqlite");

    db_con.execute(init_query).expect("can not execute initial sql");

    let (background_event_sender, background_event_receiver) = channel();
    let (event_sender, event_receiver) = channel();

    std::thread::spawn(move || {
        while let Ok(event) = background_event_receiver.recv() {
            let sender = event_sender.clone();
            event::handler_events(event, sender);
        }
    });

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_always_on_top().with_inner_size([640.0, 480.0]),
        ..Default::default()
    };

    eframe::run_native(
        "PetApp",
        options,
        Box::new(|context| {
            egui_extras::install_image_loaders(&context.egui_ctx);
            Ok(PetApp::new(
                background_event_sender,
                event_receiver,
                db_con,
            )?)
        }),
    ).map_err(|e| anyhow::anyhow!("eframe error:{:?}", e))?;

    Ok(())
}

