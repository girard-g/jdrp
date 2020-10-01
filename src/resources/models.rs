use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct CaracterStats<> {
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