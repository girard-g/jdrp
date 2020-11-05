use rand::{
    Rng,
    distributions::{Distribution, Standard},
};
use crate::item_generator::ailment::WpnAilment;
use crate::item_generator::element::WpnElement;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Object {
    pub id: u32,
    pub name: String,
    pub ilevel: u8,
    pub item_type: ItemType,
    pub asset:  String,
    pub rarity: String,
    pub equipement: Option<Equipment>,
    pub caracteristics_augmentation: Option<(String, u8)>,
    pub special: Option<Special>
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum Slot {
    RightHand,
    LeftHand,
    Chest
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum ItemType{
    Weapon,
    Armor,
    Consumable
}

impl std::fmt::Display for ItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ItemType::Weapon => write!(f, " weapon "),
            ItemType::Armor => write!(f, " armor "),
            ItemType::Consumable => write!(f, " consumable ")
        }
    }
}

#[derive(Debug, AsStaticStr, Clone, Copy, Deserialize, Serialize)]
pub enum TypeEquip {
    Sword,
    Dagger,
    Bow,
    Mace,
    Axe,
    TwoHandsAxe,
    TwoHandSword,
    Spear,
    Staff
}


impl Distribution<TypeEquip> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> TypeEquip {
        match rng.gen_range(0, 9) {
            0 => TypeEquip::Sword,
            1 => TypeEquip::Dagger,
            2 => TypeEquip::Bow,
            3 => TypeEquip::Mace,
            4 => TypeEquip::Axe,
            5 => TypeEquip::TwoHandsAxe,
            6 => TypeEquip::TwoHandSword,
            7 => TypeEquip::Spear,
            _ => TypeEquip::Staff,
        }
    }
}

impl std::fmt::Display for TypeEquip {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            TypeEquip::Sword => write!(f, " sword "),
            TypeEquip::Dagger => write!(f, " dagger "),
            TypeEquip::Bow => write!(f, " bow "),
            TypeEquip::Mace => write!(f, " mace "),
            TypeEquip::Axe => write!(f, " axe "),
            TypeEquip::TwoHandsAxe => write!(f, " two handed axe "),
            TypeEquip::TwoHandSword => write!(f, " two handed sword "),
            TypeEquip::Spear => write!(f, " spear "),
            TypeEquip::Staff => write!(f, " staff "),
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct Equipment {
    pub slot: Slot,
    pub type_equip: TypeEquip,
    pub equipped: bool,
    pub weapon: Option<Weapon>,
    pub armor: Option<Armor>,

}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct Weapon {
    pub min_damage: u16,
    pub max_damage: u16,
    pub element: Option<WpnElement>,
    pub ailment: Option<(WpnAilment, u8)>,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct Special {
    pub heal: u16,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct Armor {
    armor: u16,
    resistances: u8
}