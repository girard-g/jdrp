use serde::{Serialize, Deserialize};
use crate::item_generator::monster::{MonsterStats};
use rand::{
    Rng,
    distributions::{WeightedIndex, Distribution},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct CaracterStats<> {
    pub name: String,
    pub race: String,
    pub class: String,
    pub level: u8,
    pub age: u8,
    pub reputation: String,
    pub particularity: String,
    pub alignment: String,
    pub weapon: u8,
    pub distance_weapon: u8,
    pub bare_hand: u8,
    pub armor: i8,
    pub strengh: u8,
    pub dexterity: u8,
    pub endurance: u8,
    pub charism: u8,
    pub perception: u8,
    pub luck: u8,
    pub willpower: u8,
    pub education: u8,
}


#[derive(Serialize, Deserialize, Debug)]
pub enum CharacterType {
    Monster,
    Player
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Character {
    pub id: u32,
    pub pv: i32,
    pub asset: String,
    pub base_armor: i32,
    pub min_damage: u16,
    pub max_damage: u16,
    pub resistances: u8,
    pub character_type : CharacterType,
    pub character_stats: Option<CaracterStats>,
    pub monster_stats: Option<MonsterStats>
}

impl Character{
    pub fn new(&self){
      
    }

    pub fn attack(&self, c: Character){
        let mut rng = rand::thread_rng();

        let damage = match self.character_type {
            CharacterType::Player => {
                let wpn_damage:i32 = i32::from(rng.gen_range(self.min_damage, self.max_damage));

                let coef: i32 = (1+(i32::from(self.character_stats.unwrap().level)/100))* wpn_damage / (wpn_damage + (c.base_armor *(1+(i32::from(c.resistances)/100))));
                
                wpn_damage * coef

            },
            CharacterType::Monster => {
                12_i32
            }
        };

        c.pv - damage;
        
        if c.pv < 0 {
            c.pv = 0;
        }
        
    }

}