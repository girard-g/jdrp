// extern crate strum; // 0.10.0
use strum::AsStaticRef;

use rand::{
    Rng,
    distributions::{WeightedIndex, Distribution},
};
use crate::item_generator::resources::{Weapon, Object, TypeEquip, ItemType, Equipment, Slot, Special};
use crate::item_generator::ailment::{WpnAilment, PoisonAliment,StunAliment};
use rand::seq::SliceRandom;
use glob::{glob};

const PLAYER_LEVEL:u8 = 2;
const ASSET_PATH:&str = "images/";

struct Transition {
    level: u8,
    value: f32,
}

fn rarity_impl() -> (u8, u16, u16, String)
{
    let mut rng = rand::thread_rng();

    fn chances(table: &[Transition], level: u8) -> f32 {
        table
            .iter()
            .rev()
            .find(|transition| level >= transition.level)
            .map_or(0.0, |transition| transition.value)
    }

    let common_chance = chances(
        &[
            Transition {level: 1, value: 90.0,},
            Transition {level: 10, value: 70.0,},
            Transition {level: 20, value: 55.0,},
        ],
        PLAYER_LEVEL
    );
    let magic_chance = chances(
        &[
            Transition {level: 1, value: 5.0,},
            Transition {level: 10, value: 15.0,},
            Transition {level: 20, value: 20.0,},
        ],
        PLAYER_LEVEL
    );
    let rare_chance = chances(
        &[
            Transition {level: 1, value: 2.0,},
            Transition {level: 10, value: 4.0,},
            Transition {level: 20, value: 5.0,},
        ],
        PLAYER_LEVEL
    );
    let epic_chance = chances(
        &[
            Transition {level: 1, value: 1.0,},
            Transition {level: 10, value: 1.25,},
            Transition {level: 20, value: 2.0,},
        ],
        PLAYER_LEVEL
    );
    let legendary_chance = chances(
        &[
            Transition {level: 1, value: 0.01,},
            Transition {level: 10, value: 0.1,},
            Transition {level: 20, value: 0.2,},
        ],
        PLAYER_LEVEL
    );

    let choices = ["common", "magic", "rare", "epic", "legendary"];

    let weights = [common_chance, magic_chance, rare_chance, epic_chance, legendary_chance];
    let rariry_choice = WeightedIndex::new(&weights).unwrap();

    let ilvl = match choices[rariry_choice.sample(&mut rand::thread_rng())] {
        "common" => {
            let mut rng = rand::thread_rng();
            rng.gen_range(0, 50)
        },
        "magic" => {
            let mut rng = rand::thread_rng();
            rng.gen_range(51, 90)
        },
        "rare" => {
            let mut rng = rand::thread_rng();
            rng.gen_range(91, 140)
        },
        "epic" => {
            let mut rng = rand::thread_rng();
            rng.gen_range(141, 190)
        },
        "legendary" => {
            let mut rng = rand::thread_rng();
            rng.gen_range(191, 255)
        }
        _ => unreachable!(),
    };

    
    let ilvltuple = match ilvl {
        0..=50 =>{
            let min_damage = rng.gen_range(3, 125);
            let max_damage = rng.gen_range(min_damage, 158);
            (ilvl, min_damage, max_damage, String::from("common"))
        },
        51..=90 =>{
            let min_damage = rng.gen_range(75, 200);
            let max_damage = rng.gen_range(min_damage, 253);
            (ilvl, min_damage, max_damage, String::from("magic"))
        },
        91..=140 =>{
            let min_damage = rng.gen_range(150, 349);
            let max_damage = rng.gen_range(min_damage, 442);
            (ilvl, min_damage, max_damage, String::from("rare"))
        },
        141..=190 =>{
            let min_damage = rng.gen_range(299, 473);
            let max_damage = rng.gen_range(min_damage, 599);
            (ilvl, min_damage, max_damage, String::from("epic"))
        },
        191..=255 =>{
            let min_damage = rng.gen_range(498, 635);
            let max_damage = rng.gen_range(min_damage, 804);
            (ilvl, min_damage, max_damage, String::from("legendary"))
        },
    };

    ilvltuple

}


impl Weapon {
    fn generate() -> Object {
        let mut rng = rand::thread_rng();
        
        let id = rng.gen::<u32>();
        let ilvltuple = rarity_impl();
        let wpn_ailment: WpnAilment = rand::random();
        let placeholders: TypeEquip = rand::random();


        let globaltuple = if  ilvltuple.3 == String::from("magic")  || ilvltuple.3 == String::from("rare") || ilvltuple.3 == String::from("epic")
        {
            let ailmenttuple = match wpn_ailment {
                WpnAilment::Poison => {
                    let wpn_alignment_power: PoisonAliment = rand::random();
                    let mut name :String = wpn_alignment_power.as_static().to_owned();
                    name.push_str(&placeholders.to_string());
                    let itm_name = name; // Toxic dagger

                    let caracteristics_augmentation = Some((WpnAilment::Poison, wpn_alignment_power as u8));
                    (itm_name, caracteristics_augmentation)
                },
                WpnAilment::Stun => {
                    let wpn_alignment_power: StunAliment = rand::random();
                    let mut name :String = wpn_alignment_power.as_static().to_owned();
                    name.push_str(&placeholders.to_string());
                    let itm_name =  name; // Toxic dagger

                    let caracteristics_augmentation = Some((WpnAilment::Stun, wpn_alignment_power as u8));
                    (itm_name, caracteristics_augmentation)
                },
                _ => {
                    let result = match ilvltuple.3.as_str() {
                        "magic" =>{
                            let mut s = String::from("magic");
                            s.push_str(&placeholders.to_string());
                            (s, Some((WpnAilment::None, 0 as u8)))
                        },
                       "rare" =>{
                            let mut s = String::from("rare");
                            s.push_str(&placeholders.to_string());
                            (s, Some((WpnAilment::None, 0 as u8)))
                        },
                        "epic" =>{
                            let mut s = String::from("epic");
                            s.push_str(&placeholders.to_string());
                            (s, Some((WpnAilment::None, 0 as u8)))
                        }
                        _ => {
                            (String::from("none"), Some((WpnAilment::None, 0 as u8)))
                        }
                    };
                    result

                }
    
            };

            ailmenttuple

        } else if ilvltuple.3 == String::from("common") {
            let mut s = String::from("common");
             s.push_str(&placeholders.to_string());
            (s, Some((WpnAilment::None, 0 as u8)))
        }else{
            let mut s = String::from("legendary");
             s.push_str(&placeholders.to_string());
            (s, Some((WpnAilment::None, 0 as u8)))
        };

        let ass_ets = match placeholders {
            TypeEquip::Sword => {
                // "images/weapons/swords1h/ActionLoot_(1).png"
                let vector = directory_search("./static/images/weapons/swords1h/ActionLoot_([0-9])*.png");
                let toto = vector.choose(&mut rand::thread_rng()).unwrap();
                let string = toto.to_string();
                string
            },
            TypeEquip::Axe =>{
                let vector = directory_search("./static/images/weapons/axes1h/ActionLoot_([0-9])*.png");
                let toto = vector.choose(&mut rand::thread_rng()).unwrap();
                let string = toto.to_string();
                string
            },
            TypeEquip::Bow =>{
                let vector = directory_search("./static/images/weapons/bows/ActionLoot_([0-9])*.png");
                let toto = vector.choose(&mut rand::thread_rng()).unwrap();
                let string = toto.to_string();
                string
            },
            TypeEquip::Dagger =>{
                let vector = directory_search("./static/images/weapons/daggers/ActionLoot_([0-9])*.png");
                let toto = vector.choose(&mut rand::thread_rng()).unwrap();
                let string = toto.to_string();
                string
            },
            TypeEquip::Mace =>{
                let vector = directory_search("./static/images/weapons/maces/ActionLoot_([0-9])*.png");
                let toto = vector.choose(&mut rand::thread_rng()).unwrap();
                let string = toto.to_string();
                string
            },
            TypeEquip::TwoHandsAxe =>{
                let vector = directory_search("./static/images/weapons/axes2h/ActionLoot_([0-9])*.png");
                let toto = vector.choose(&mut rand::thread_rng()).unwrap();
                let string = toto.to_string();
                string
            },
            TypeEquip::TwoHandSword =>{
                let vector = directory_search("./static/images/weapons/swords2h/ActionLoot_([0-9])*.png");
                let toto = vector.choose(&mut rand::thread_rng()).unwrap();
                let string = toto.to_string();
                string
            },
            TypeEquip::Spear =>{
                let vector = directory_search("./static/images/weapons/spears/ActionLoot_([0-9])*.png");
                let toto = vector.choose(&mut rand::thread_rng()).unwrap();
                let string = toto.to_string();
                string
            },
            TypeEquip::Staff =>{
                let vector = directory_search("./static/images/weapons/staff/ActionLoot_([0-9])*.png");
                let toto = vector.choose(&mut rand::thread_rng()).unwrap();
                let string = toto.to_string();
                string
            },
        };


        Object {
            id: id,
            name: globaltuple.0,
            ilevel: ilvltuple.0,
            item_type:  ItemType::Weapon,
            rarity: ilvltuple.3,
            asset: ass_ets.to_string(),
            equipement: Some(
                Equipment{
                    slot: Slot::RightHand,
                    type_equip:placeholders,
                    equipped: false,
                    weapon:Some(
                        Weapon{
                            min_damage: ilvltuple.1,
                            max_damage: ilvltuple.2,
                            element: None,
                            ailment: Some(globaltuple.1.unwrap()),
                        }
                    ),
                    armor:None,
                }
            ),
            caracteristics_augmentation: None,
            special: None
        }
    }

}

// error_chain! {
//     foreign_links {
//         Glob(glob::GlobError);
//         Pattern(glob::PatternError);
//     }
// }

fn directory_search(str_to_search: &str) -> Vec<String>
{

    let mut vec1 = vec!();

    for path in glob(str_to_search).unwrap().filter_map(Result::ok) {
        println!("{}", path.display());
        vec1.push(path.display().to_string());
    }

    vec1

}


impl Object{
    fn generate(name: String, rarity: String, special: u16) -> Object {

        let mut rng = rand::thread_rng();

        Object {
            id: rng.gen::<u32>(),
            name: name,
            ilevel: 1,
            asset: ASSET_PATH.to_string(),
            item_type: ItemType::Consumable,
            rarity: rarity,
            equipement: None,
            caracteristics_augmentation: None,
            special: Some(Special{heal:special})
        }
    }
}

pub fn generate_weapon() -> Object {
    Weapon::generate()
}