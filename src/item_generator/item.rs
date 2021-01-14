use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Consumable {
    pub id: u32,
    pub name: String,
    pub ilevel: u8,
    pub item_type: ConsumableType,
    pub asset: String,
    pub rarity: String,
    pub description: String,
    pub on_use: UseCallback,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum ConsumableType {
    HealthPotion = 25,
    Bandages,
    Antidot,
}

impl std::fmt::Display for ConsumableType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ConsumableType::HealthPotion => write!(
                f,
                "{}",
                format!("Rends {} points de vie", ConsumableType::HealthPotion as u8)
            ),
            ConsumableType::Bandages => write!(f, "Soigne les saignements"),
            ConsumableType::Antidot => write!(f, "Soigne le poison"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum UseCallback {
    HealthPotion,
    Bandages,
    Antidot,
}

impl UseCallback {
    fn callback(self) {
        use UseCallback::*;
        let callback: fn() = match self {
            HealthPotion => health_potion_usage,
            Bandages => bandage_usage,
            Antidot => antidot_usage,
        };

        callback();
    }
}

impl Distribution<ConsumableType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ConsumableType {
        match rng.gen_range(0, 3) {
            0 => ConsumableType::HealthPotion,
            1 => ConsumableType::Bandages,
            2 => ConsumableType::Antidot,
            _ => ConsumableType::HealthPotion,
        }
    }
}

pub fn health_potion_usage() {
    //get player health and add an amout of life
    //TODO send this through ws
}

pub fn bandage_usage() {
    //get player status and remove bleeding ailement
    //TODO send this through ws
}

pub fn antidot_usage() {
    //get player status and remove poisoned ailement
    //TODO send this through ws
}
