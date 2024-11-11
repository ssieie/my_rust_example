use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Receiver, Sender};
use sqlite::Connection;
use anyhow::Result;
use eframe::egui::Context;
use eframe::Frame;
use crate::dbaccess::pet::{get_pets_from_db};
use crate::event::Event;

#[derive(Debug, PartialEq, Clone)]
pub struct PetKind(pub(crate) String);

#[derive(Debug, PartialEq, Clone)]
pub struct Pet {
    pub id: i64,
    pub name: String,
    pub age: i64,
    pub kind: PetKind,
}

pub struct PetApp {
    pub app_state: AppState,
    pub background_event_sender: Sender<Event>,
    pub event_receiver: Receiver<Event>,
    pub db_con: Arc<Mutex<Connection>>,
}

impl PetApp {
    pub fn new(background_event_sender: Sender<Event>, event_receiver: Receiver<Event>, db_con: Connection) -> Result<Box<Self>> {
        let db_con = Arc::new(Mutex::new(db_con));
        let pets = get_pets_from_db(db_con.clone())?;

        Ok(Box::new(Self {
            app_state: AppState {
                selected_pet: None,
                pets,
                pet_image: None,
                add_form: AddForm {
                    show: false,
                    name: String::default(),
                    age: String::default(),
                    kind: String::default(),
                },
            },
            background_event_sender,
            event_receiver,
            db_con,
        }))
    }
    pub fn handle_gui_events(&mut self){
        while let Ok(event) = self.event_receiver.try_recv() {

        }
    }
}

impl eframe::App for PetApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub selected_pet: Option<Pet>,
    pub pets: Vec<Pet>,
    pub pet_image: Option<String>,
    pub add_form: AddForm,
}

#[derive(Debug, Clone)]
pub struct AddForm {
    show: bool,
    name: String,
    age: String,
    kind: String,
}
