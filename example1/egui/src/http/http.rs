use std::sync::mpsc::Sender;
use eframe::egui;
use crate::event::Event;
use crate::models::pet::{ PetKind};



pub fn fetch_pet_image(ctx: egui::Context, pet_kind: PetKind, sender: Sender<Event>) {
    let url = if pet_kind.0 == "dog" {
        "https://dog.ceo/api/breeds/image/random"
    } else {
        "https://api.thecatapi.com/v1/images/search"
    };

    ehttp::fetch(
        ehttp::Request::get(url),
        move |result| {
            if let Ok(result) = result {
                let image_url = if pet_kind.0 == "dog" {
                    if let Ok(json) = result.json() {
                        Some(json)
                    } else { None }
                } else if let Ok(json) = result.json() {
                    Some(json.item.url)
                } else {
                    None
                };

                let _ = sender.send(Event::SetPetImage(image_url));

                ctx.request_repaint();
            }
        },
    )
}