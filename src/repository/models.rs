// use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use super::schema::players;
use super::schema::caracter;


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

#[derive(Insertable)]
#[table_name="caracter"]
pub struct NewCaracter {
    pub id: String,
    pub player_id: String,
    pub stats: String
}

#[derive(Insertable)]
#[table_name="players"]
pub struct NewPlayer<'a> {
    pub id: String,
    pub pseudo: &'a String,
    pub password: String,
    pub is_mj: i32,
}