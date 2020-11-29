use serde::{Deserialize, Serialize};
use crate::item_generator::ailment::{WpnAilment};
use strum_macros::EnumString;

#[derive(Debug, PartialEq, EnumString)]
pub enum MonsterType {
    #[strum(serialize = "Banshee", serialize = "banshee")]
    Banshee,
    #[strum(serialize = "Barbarian", serialize = "barbarian")]
    Barbarian,
    #[strum(serialize = "Daemon", serialize = "daemon")]
    Daemon,
    #[strum(serialize = "Ghost", serialize = "ghost")]
    Ghost,
    #[strum(serialize = "Ghoul", serialize = "ghoul")]
    Ghoul,
    #[strum(serialize = "Goblin", serialize = "goblin")]
    Goblin,
    #[strum(serialize = "Gravedigger", serialize = "gravedigger")]
    Gravedigger,
    #[strum(serialize = "Knight", serialize = "knight")]
    Knight,
    #[strum(serialize = "Lich", serialize = "lich")]
    Lich,
    #[strum(serialize = "Necromancer", serialize = "necromancer")]
    Necromancer,
    #[strum(serialize = "Rat", serialize = "rat")]
    Rat,
    #[strum(serialize = "Rogue", serialize = "rogue")]
    Rogue,
    #[strum(serialize = "Skeleton", serialize = "skeleton")]
    Skeleton,
    #[strum(serialize = "Spider", serialize = "spider")]
    Spider,
    #[strum(serialize = "Succubus", serialize = "succubus")]
    Succubus,
    #[strum(serialize = "Toad", serialize = "toad")]
    Toad,
    #[strum(serialize = "Wasp", serialize = "wasp")]
    Wasp,
    #[strum(serialize = "Werewolf", serialize = "werewolf")]
    Werewolf,
    #[strum(serialize = "Witch", serialize = "witch")]
    Witch,
    #[strum(serialize = "Wizard", serialize = "wizard")]
    Wizard,
    #[strum(serialize = "Zombie", serialize = "zombie")]
    Zombie
}

impl std::fmt::Display for MonsterType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            MonsterType::Banshee => write!(f, " Banshee "),
            MonsterType::Barbarian => write!(f, " Barbarian "),
            MonsterType::Daemon => write!(f, " Daemon "),
            MonsterType::Ghost => write!(f, " Ghost "),
            MonsterType::Ghoul => write!(f, " Ghoul "),
            MonsterType::Goblin => write!(f, " Goblin "),
            MonsterType::Gravedigger => write!(f, " Gravedigger "),
            MonsterType::Knight => write!(f, " Knight "),
            MonsterType::Lich => write!(f, " Lich "),
            MonsterType::Necromancer => write!(f, " Necromancer "),
            MonsterType::Rat => write!(f, " Rat "),
            MonsterType::Rogue => write!(f, " Rogue "),
            MonsterType::Skeleton => write!(f, " Skeleton "),
            MonsterType::Spider => write!(f, " Spider "),
            MonsterType::Succubus => write!(f, " Succubus "),
            MonsterType::Toad => write!(f, " Toad "),
            MonsterType::Wasp => write!(f, " Wasp "),
            MonsterType::Werewolf => write!(f, " Werewolf "),
            MonsterType::Witch => write!(f, " Witch "),
            MonsterType::Wizard => write!(f, " Wizard "),
            MonsterType::Zombie => write!(f, " Zombie "),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Monster {
    pub id: u32,
    pub pv: u32,
    pub asset: String,
    pub base_armor: i32,
    pub min_damage: u16,
    pub max_damage: u16,
    pub resistances: u8,
    pub ailment: WpnAilment,
    pub on_death: DeathCallback 
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum DeathCallback {
    Monster
}

impl DeathCallback {
    fn callback(self) {
        use DeathCallback::*;
        let callback: fn() = match self {
            Monster => monster_death,
        };

        callback();
    }
}

pub fn monster_death() {
    use crate::item_generator::generator;
    generator::generate_loot();
    //TODO: set up xp
    //TODO send this through ws
}