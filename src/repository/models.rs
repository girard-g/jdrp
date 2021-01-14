// use chrono::NaiveDateTime;
use super::schema::{caracter, inventory, inventory_items_generated, items_generated, players};
use serde::{Deserialize, Serialize};
// use super::schema::caracter;

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
    pub stats: String,
    pub gold: i32,
}

#[derive(Identifiable, Queryable, Serialize, Deserialize)]
#[table_name = "items_generated"]
pub struct ItemGenerated {
    pub id: String,
    pub item_type: String,
    pub equiped: i32,
    pub stats: String,
}

#[derive(Identifiable, Queryable, Serialize, Deserialize)]
#[table_name = "inventory"]
pub struct Inventory {
    pub id: String,
    pub caracter_id: String,
}

#[derive(Identifiable, Associations, Queryable)]
#[belongs_to(Inventory, foreign_key = "inventory_id")]
#[primary_key(inventory_id, items_generated_id)]
#[table_name = "inventory_items_generated"]
pub struct InventoryItemsGenerated {
    pub inventory_id: String,
    pub items_generated_id: String,
    pub posx: i32,
    pub posy: i32,
}

#[derive(Insertable, Debug)]
#[table_name = "caracter"]
pub struct NewCaracter {
    pub id: String,
    pub player_id: String,
    pub stats: String,
    pub gold: i32,
}

#[derive(Insertable, Debug)]
#[table_name = "players"]
pub struct NewPlayer<'a> {
    pub id: String,
    pub pseudo: &'a String,
    pub password: String,
    pub is_mj: i32,
}
