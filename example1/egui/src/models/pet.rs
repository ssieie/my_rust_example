use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Receiver, Sender};
use sqlite::Connection;
use anyhow::Result;
use eframe::egui::Context;
use eframe::{egui, Frame};
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
    pub fn handle_gui_events(&mut self) {
        while let Ok(event) = self.event_receiver.try_recv() {
            match event {
                Event::SetPetImage(pet_image) => {
                    self.app_state.pet_image = pet_image;
                }
                Event::SetSelectedPet(pet) => self.app_state.selected_pet = pet,
                Event::SetPets(pets) => {
                    if let Some(ref selected_pet) = self.app_state.selected_pet {
                        if !pets.iter().any(|pet| pet.id == selected_pet.id) {
                            self.app_state.selected_pet = None;
                        }
                    }
                    self.app_state.pets = pets;
                }
                _ => ()
            }
        }
    }
}

impl eframe::App for PetApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        self.handle_gui_events();

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::SidePanel::left("left panel")
                .resizable(false)
                .default_width(200.0)
                .show_inside(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading("Pets");
                        ui.separator();
                        if ui.button("Add new Pet").clicked() {
                            self.app_state.add_form.show = !self.app_state.add_form.show;
                        }
                        if self.app_state.add_form.show {
                            ui.separator();

                            ui.vertical_centered(|ui| {
                                ui.horizontal(|ui| {
                                    ui.vertical(|ui| {
                                        ui.label("name:");
                                        ui.label("age");
                                        ui.label("kind");
                                    });
                                    ui.end_row();
                                    ui.vertical(|ui| {
                                        ui.text_edit_singleline(&mut self.app_state.add_form.name);
                                        ui.text_edit_singleline(&mut self.app_state.add_form.age);
                                        ui.text_edit_singleline(&mut self.app_state.add_form.kind);
                                    });
                                });

                                if ui.button("Submit").clicked() {
                                    let add_form = &mut self.app_state.add_form;
                                    let age = add_form.age.parse().unwrap_or(0);
                                    let kind = match add_form.kind.as_str() {
                                        "cat" => PetKind(String::from("cat")),
                                        _ => PetKind(String::from("dog")),
                                    };
                                    let name = add_form.name.to_owned();
                                    if !name.is_empty() && age > 0 {
                                        let _ = self.background_event_sender.send(
                                            Event::InsertPetToDB(
                                                ctx.clone(),
                                                self.db_con.clone(),
                                                Pet {
                                                    id: -1,
                                                    name,
                                                    age,
                                                    kind: kind.clone(),
                                                },
                                            ),
                                        );
                                        let _ = self
                                            .background_event_sender
                                            .send(Event::GetPetImage(ctx.clone(), kind));
                                        add_form.name = String::default();
                                        add_form.age = String::default();
                                        add_form.kind = String::default();
                                    }
                                }
                            });
                        }

                        ui.separator();
                        self.app_state.pets.iter().for_each(|pet| {
                            if ui
                                .selectable_value(
                                    &mut self.app_state.selected_pet,
                                    Some(pet.to_owned()),
                                    pet.name.clone(),
                                )
                                .changed()
                            {
                                let _ = self.background_event_sender.send(Event::GetPetFromDB(
                                    ctx.clone(),
                                    self.db_con.clone(),
                                    pet.id,
                                ));
                                let _ = self
                                    .background_event_sender
                                    .send(Event::GetPetImage(ctx.clone(), pet.kind.clone()));
                            }
                        });
                    });
                });
            egui::CentralPanel::default().show_inside(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Details");
                    if let Some(pet) = &self.app_state.selected_pet {
                        ui.vertical(|ui| {
                            ui.horizontal(|ui| {
                                if ui.button("Delete").clicked() {
                                    let _ =
                                        self.background_event_sender.send(Event::DeletePetFromDB(
                                            ctx.clone(),
                                            self.db_con.clone(),
                                            pet.id,
                                        ));
                                }
                            });
                            ui.separator();
                            ui.vertical(|ui| {
                                ui.horizontal(|ui| {
                                    ui.vertical(|ui| {
                                        ui.label("id:");
                                        ui.label("name:");
                                        ui.label("age");
                                        ui.label("kind");
                                    });
                                    ui.end_row();
                                    ui.vertical(|ui| {
                                        ui.label(pet.id.to_string());
                                        ui.label(&pet.name);
                                        ui.label(pet.age.to_string());
                                        ui.label(&pet.kind.0);
                                    });
                                });
                                ui.separator();
                                if let Some(ref pet_image) = self.app_state.pet_image {
                                    ui.add(egui::Image::from_uri(pet_image).max_width(200.0));
                                }
                            });
                        });
                    } else {
                        ui.label("No pet selected.");
                    }
                });
            });
        });
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
