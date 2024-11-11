use std::sync::{Arc, Mutex};
use anyhow::{anyhow, Result};
use crate::models::pet::{Pet, PetKind};
use sqlite::Connection;

const GET_PET_BY_ID: &str = "SELECT id, name, age, kind FROM pets where id = ?";
const DELETE_PET_BY_ID: &str = "DELETE FROM pets where id = ?";
const INSERT_PET: &str =
    "INSERT INTO pets (name, age, kind) VALUES (?, ?, ?) RETURNING id, name, age, kind";
const GET_PETS: &str = "SELECT id, name, age, kind FROM pets";

pub fn insert_pet_to_db(db_con: Arc<Mutex<Connection>>, pet: Pet) -> Result<Pet> {
    let con = db_con
        .lock()
        .map_err(|_| anyhow!("error while locking db connection"))?;

    let mut stmt = con.prepare(INSERT_PET)?;
    stmt.bind((1, pet.name.as_str()))?;
    stmt.bind((2, pet.age))?;
    stmt.bind((3, pet.kind.0.as_str()))?;

    if stmt.next()? == sqlite::State::Row {
        let id = stmt.read::<i64, _>(0)?;
        let name = stmt.read::<String, _>(1)?;
        let age = stmt.read::<i64, _>(2)?;
        let kind = stmt.read::<String, _>(3)?;
        let kind = PetKind(kind);

        return Ok(Pet {
            id,
            name,
            age,
            kind,
        });
    }

    Err(anyhow!("error while inserting pet into db"))
}

pub fn delete_pet_from_db(db_con: Arc<Mutex<Connection>>, pet_id: i64) -> Result<()> {
    let con = db_con
        .lock()
        .map_err(|_| anyhow!("error while locking db connection"))?;

    let mut stmt = con.prepare(DELETE_PET_BY_ID)?;
    stmt.bind((1, pet_id))?;

    if stmt.next()? == sqlite::State::Done {
        Ok(())
    } else {
        Err(anyhow!("error while deleting pet with id {}", pet_id))
    }
}

pub fn get_pet_from_db(db_con: Arc<Mutex<Connection>>, pet_id: i64) -> Result<Option<Pet>> {
    let con = db_con
        .lock()
        .map_err(|_| anyhow!("error while locking db connection"))?;
    let mut stmt = con.prepare(GET_PET_BY_ID)?;
    stmt.bind((1, pet_id))?;

    if stmt.next()? == sqlite::State::Row {
        let id = stmt.read::<i64, _>(0)?;
        let name = stmt.read::<String, _>(1)?;
        let age = stmt.read::<i64, _>(2)?;
        let kind = stmt.read::<String, _>(3)?;
        let kind = PetKind(kind);
        return Ok(Some(Pet {
            id,
            name,
            age,
            kind,
        }));
    }

    Ok(None)
}

pub fn get_pets_from_db(db_con: Arc<Mutex<Connection>>) -> Result<Vec<Pet>> {
    let con = db_con
        .lock()
        .map_err(|_| anyhow!("error while locking db connection"))?;

    let mut pets = Vec::new();
    let mut stmt = con.prepare(GET_PETS)?;

    for row in stmt.iter() {
        let row = row?;
        let id = row.read::<i64, _>(0)?;
        let name = row.read::<&str, _>(1)?;
        let age = row.read::<i64, _>(2)?;
        let kind = row.read::<&str, _>(3)?;

        pets.push(Pet {
            id,
            name: name.to_owned(),
            age,
            kind: PetKind(kind.to_owned()),
        })
    }

    Ok(pets)
}