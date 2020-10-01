// use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use super::schema::players;


#[derive(Queryable, Serialize, Deserialize)]
pub struct Player {
    pub id: String,
    pub pseudo: String,
    pub password: String,
    pub is_mj: i32,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Caracter {
    pub id: String,
    pub player_id: String,
    pub stats: String
}


#[derive(Queryable, Serialize, Deserialize)]
pub struct PlayerStats<> {
    pub name: String,
    pub race: String,
    pub class: String,
    pub level: u8,
    pub reputation: String,
    pub particularity: String,
    pub alignment: String,
    pub weapon: u8,
    pub distance_weapon: u8,
    pub bare_hand: u8,
    pub armor: u8,
    pub strengh: u8,
    pub dexterity: u8,
    pub endurance: u8,
    pub charism: u8,
    pub perception: u8,
    pub luck: u8,
    pub willpower: u8,
    pub education: u8,
}

#[derive(Insertable)]
#[table_name="players"]
pub struct NewPlayer<'a> {
    pub id: String,
    pub pseudo: &'a String,
    pub password: String,
    pub is_mj: i32,
}