use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CaracterStats {
    pub name: String,
    pub race: String,
    pub class: String,
    // pub level: u8,
    pub age: u16,
    pub reputation: String,
    pub particularity: String,
    pub alignment: String,
    // pub weapon: u8,
    // pub distance_weapon: u8,
    // pub bare_hand: u8,
    // pub armor: i8,
    pub strengh: u8,
    pub dexterity: u8,
    pub endurance: u8,
    pub charism: u8,
    pub perception: u8,
    pub luck: u8,
    pub willpower: u8,
    pub education: u8,
    pub portrait: String,
}
