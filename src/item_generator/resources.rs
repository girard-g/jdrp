use crate::item_generator::ailment::WpnAilment;
use crate::item_generator::element::WpnElement;
use crate::item_generator::item::Consumable;
use crate::item_generator::spell::Spell;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Loot {
    pub object: Option<Object>,
    pub consumable: Option<Consumable>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Object {
    pub id: u32,
    pub name: String,
    pub ilevel: u8,
    pub item_type: ItemType,
    pub asset: String,
    pub rarity: String,
    pub equipement: Option<Equipment>,
    pub caracteristics_augmentation: Option<(String, u8)>,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum Slot {
    RightHand,
    LeftHand,
    TwoHand,
    Chest,
    Belt,
    Boot,
    Glove,
    Helmet,
    Necklace,
    Ring,
    Shield,
    Wrist,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum ItemType {
    Weapon,
    Armor,
    Jewel,
    Consumable,
}

impl Distribution<ItemType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ItemType {
        match rng.gen_range(0, 5) {
            0 => ItemType::Weapon,
            1 => ItemType::Armor,
            2 => ItemType::Consumable,
            3 => ItemType::Jewel,
            _ => ItemType::Consumable,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub enum ArmorClass {
    Light,
    Medium,
    Heavy,
    Shield,
}

impl Distribution<ArmorClass> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ArmorClass {
        match rng.gen_range(0, 3) {
            0 => ArmorClass::Light,
            1 => ArmorClass::Medium,
            2 => ArmorClass::Heavy,
            _ => ArmorClass::Shield,
        }
    }
}

impl std::fmt::Display for ArmorClass {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ArmorClass::Light => write!(f, " light "),
            ArmorClass::Medium => write!(f, " medium "),
            ArmorClass::Heavy => write!(f, " heavy "),
            ArmorClass::Shield => write!(f, " shield "),
        }
    }
}

impl std::fmt::Display for ItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ItemType::Weapon => write!(f, " weapon "),
            ItemType::Armor => write!(f, " armor "),
            ItemType::Jewel => write!(f, " jewel "),
            ItemType::Consumable => write!(f, " consumable "),
        }
    }
}

#[derive(Debug, AsStaticStr, Clone, Copy, Deserialize, Serialize)]
pub enum WeaponType {
    Sword,
    Dagger,
    Bow,
    Mace,
    Axe,
    TwoHandsAxe,
    TwoHandSword,
    Spear,
    Staff,
}

#[derive(Debug, AsStaticStr, Clone, Copy, Deserialize, Serialize)]
pub enum ArmorType {
    Chest,
    Belt,
    Boot,
    Glove,
    Helmet,
    Shield,
    Wrist,
}

#[derive(Debug, AsStaticStr, Clone, Copy, Deserialize, Serialize)]
pub enum JewelType {
    Necklace,
    Ring,
}

impl Distribution<WeaponType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> WeaponType {
        match rng.gen_range(0, 9) {
            0 => WeaponType::Sword,
            1 => WeaponType::Dagger,
            2 => WeaponType::Bow,
            3 => WeaponType::Mace,
            4 => WeaponType::Axe,
            5 => WeaponType::TwoHandsAxe,
            6 => WeaponType::TwoHandSword,
            7 => WeaponType::Spear,
            _ => WeaponType::Staff,
        }
    }
}

impl Distribution<ArmorType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ArmorType {
        match rng.gen_range(0, 9) {
            0 => ArmorType::Belt,
            1 => ArmorType::Boot,
            2 => ArmorType::Glove,
            3 => ArmorType::Helmet,
            6 => ArmorType::Shield,
            7 => ArmorType::Chest,
            _ => ArmorType::Wrist,
        }
    }
}

impl Distribution<JewelType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> JewelType {
        match rng.gen_range(0, 2) {
            0 => JewelType::Necklace,
            1 => JewelType::Ring,
            _ => JewelType::Ring,
        }
    }
}

impl std::fmt::Display for WeaponType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            WeaponType::Sword => write!(f, " sword "),
            WeaponType::Dagger => write!(f, " dagger "),
            WeaponType::Bow => write!(f, " bow "),
            WeaponType::Mace => write!(f, " mace "),
            WeaponType::Axe => write!(f, " axe "),
            WeaponType::TwoHandsAxe => write!(f, " two handed axe "),
            WeaponType::TwoHandSword => write!(f, " two handed sword "),
            WeaponType::Spear => write!(f, " spear "),
            WeaponType::Staff => write!(f, " staff "),
        }
    }
}

impl std::fmt::Display for ArmorType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ArmorType::Chest => write!(f, " chest "),
            ArmorType::Belt => write!(f, " belt "),
            ArmorType::Boot => write!(f, " boot "),
            ArmorType::Glove => write!(f, " glove "),
            ArmorType::Helmet => write!(f, " helmet "),
            ArmorType::Shield => write!(f, " shield "),
            ArmorType::Wrist => write!(f, " wrist "),
        }
    }
}

impl std::fmt::Display for JewelType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            JewelType::Necklace => write!(f, " necklace "),
            JewelType::Ring => write!(f, " ring "),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Equipment {
    pub slot: Slot,
    pub equipped: bool,
    pub weapon: Option<Weapon>,
    pub armor: Option<Armor>,
    pub jewel: Option<Jewel>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Weapon {
    pub min_damage: u16,
    pub max_damage: u16,
    pub weapon_type: WeaponType,
    pub element: Option<WpnElement>,
    pub ailment: Option<(WpnAilment, u8)>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Armor {
    pub armor: u16,
    pub armor_type: ArmorType,
    pub class: ArmorClass,
    pub resistances: u8,
    pub ailment: Option<WpnAilment>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Jewel {
    pub jewel_type: JewelType,
    pub spell: Spell,
    pub usage: u8,
    pub ailment: Option<WpnAilment>,
}
