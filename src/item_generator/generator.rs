// extern crate strum; // 0.10.0
use strum::AsStaticRef;

use rand::{
    Rng,
    distributions::{WeightedIndex, Distribution},
};
use crate::item_generator::resources::{Weapon, Armor, Object, WeaponType, ArmorType, ArmorClass, ItemType, Equipment, Slot, Special, Jewel, JewelType};
use crate::item_generator::ailment::{WpnAilment, PoisonAliment,StunAliment};
use crate::item_generator::spell::{Spell, LowLevelSpells, Incante, SpellDamage};
// use crate::item_generator::element::{WpnElement};
use rand::seq::SliceRandom;
use glob::{glob};

const PLAYER_LEVEL:u8 = 20;

struct Transition {
    level: u8,
    value: f32,
}

fn rarity_impl() -> (u8, String)
{
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
            (rng.gen_range(0, 50), String::from("common"))
        },
        "magic" => {
            let mut rng = rand::thread_rng();
            (rng.gen_range(51, 90), String::from("magic"))
        },
        "rare" => {
            let mut rng = rand::thread_rng();
            (rng.gen_range(91, 140),  String::from("rare"))
        },
        "epic" => {
            let mut rng = rand::thread_rng();
            (rng.gen_range(141, 190),  String::from("epic"))
        },
        "legendary" => {
            let mut rng = rand::thread_rng();
            (rng.gen_range(191, 255),  String::from("legendary"))
        }
        _ => unreachable!(),
    };

    ilvl

}

fn calculate_weapon_damage(ilvl: u8) -> (u16, u16){

    let mut rng = rand::thread_rng();

    let ilvltuple = match ilvl {
        0..=50 =>{
            let min_damage = rng.gen_range(3, 125);
            let max_damage = rng.gen_range(min_damage, 158);
            (min_damage, max_damage)
        },
        51..=90 =>{
            let min_damage = rng.gen_range(75, 200);
            let max_damage = rng.gen_range(min_damage, 253);
            (min_damage, max_damage)
        },
        91..=140 =>{
            let min_damage = rng.gen_range(150, 349);
            let max_damage = rng.gen_range(min_damage, 442);
            (min_damage, max_damage)
        },
        141..=190 =>{
            let min_damage = rng.gen_range(299, 473);
            let max_damage = rng.gen_range(min_damage, 599);
            (min_damage, max_damage)
        },
        191..=255 =>{
            let min_damage = rng.gen_range(498, 635);
            let max_damage = rng.gen_range(min_damage, 804);
            (min_damage, max_damage)
        },
    };

    ilvltuple
}

fn calculate_armor(ilvl: u8) -> u16{

    let mut rng = rand::thread_rng();

    let ilvltuple = match ilvl {
        0..=50 =>{
            let min_damage = rng.gen_range(3, 125);
            let max_damage = rng.gen_range(min_damage, 158);
            let armor = (min_damage + max_damage)/2;
            armor
        },
        51..=90 =>{
            let min_damage = rng.gen_range(75, 200);
            let max_damage = rng.gen_range(min_damage, 253);
            let armor = (min_damage + max_damage)/2;
            armor
        },
        91..=140 =>{
            let min_damage = rng.gen_range(150, 349);
            let max_damage = rng.gen_range(min_damage, 442);
            let armor = (min_damage + max_damage)/2;
            armor
        },
        141..=190 =>{
            let min_damage = rng.gen_range(299, 473);
            let max_damage = rng.gen_range(min_damage, 599);
            let armor = (min_damage + max_damage)/2;
            armor
        },
        191..=255 =>{
            let min_damage = rng.gen_range(498, 635);
            let max_damage = rng.gen_range(min_damage, 804);
            let armor = (min_damage + max_damage)/2;
            armor
        },
    };

    ilvltuple
}

fn calculate_resistances(ilvl: u8) -> u8{

    let mut rng = rand::thread_rng();

    let res = match ilvl {
        0..=50 =>{
           rng.gen_range(4, 8)
        },
        51..=90 =>{
            rng.gen_range(8, 15)
        },
        91..=140 =>{
            rng.gen_range(15, 25)
        },
        141..=190 =>{
            rng.gen_range(25, 35)
        },
        191..=255 =>{
            rng.gen_range(35, 40)
        },
    };
    res
}


fn directory_search(str_to_search: &str) -> Vec<String>
{

    let mut vec1 = vec!();

    for path in glob(str_to_search).unwrap().filter_map(Result::ok) {
        // println!("{}", path.display());
        vec1.push(path.display().to_string());
    }

    vec1

}

fn jewel_stats(l: &LowLevelSpells) ->  (String, Incante){
    match l  {
        LowLevelSpells::AnimationDesMorts => (String::from(""), Incante::Concentration),
        LowLevelSpells::RayonEmpoisonne => (SpellDamage::D1d8.to_string(), Incante::Action),
        LowLevelSpells::RayonAffaiblissant => (String::from(""), Incante::Action),
        LowLevelSpells::ToucherDuVampire => (SpellDamage::D3d6.to_string(), Incante::Action),
        LowLevelSpells::Amis => (String::from(""), Incante::Action),
        LowLevelSpells::RepresailleInfernales => (SpellDamage::D2d10.to_string(), Incante::Reaction),
        LowLevelSpells::LameTonnante => (SpellDamage::D1d8.to_string(), Incante::Action),
        LowLevelSpells::LameAuxFlammesVertes => (SpellDamage::D2d10.to_string(), Incante::Action),
        LowLevelSpells::Gelure =>(SpellDamage::D1d6.to_string(), Incante::Action),
        LowLevelSpells::ToileDAraignee => (String::from(""), Incante::Action),
        LowLevelSpells::ServiteurInvisible => (String::from(""), Incante::Rituel),
        LowLevelSpells::RespirationAquatique => (String::from(""), Incante::Rituel),
        LowLevelSpells::VisionDansLeNoir => (String::from(""), Incante::Rituel),
        LowLevelSpells::FlechesEnflammees => (SpellDamage::D1d6.to_string(), Incante::Action),
        LowLevelSpells::Deblocage => (String::from(""), Incante::Action),
        LowLevelSpells::Bagou => (String::from(""), Incante::Action),
    }
}

impl Weapon {
    fn generate() -> Object {
        let mut rng = rand::thread_rng();
        
        let id = rng.gen::<u32>();
        let ilvltuple = rarity_impl();
        let damagetuple = calculate_weapon_damage(ilvltuple.0);
        let wpn_ailment: WpnAilment = rand::random();
        let placeholders: WeaponType = rand::random();

        let globaltuple = if  ilvltuple.1 == String::from("magic")  || ilvltuple.1 == String::from("rare") || ilvltuple.1 == String::from("epic")
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
                    let result = match ilvltuple.1.as_str() {
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

        } else if ilvltuple.1 == String::from("common") {
            let mut s = String::from("common");
             s.push_str(&placeholders.to_string());
            (s, Some((WpnAilment::None, 0 as u8)))
        }else{
            let mut s = String::from("legendary");
             s.push_str(&placeholders.to_string());
            (s, Some((WpnAilment::None, 0 as u8)))
        };

        let ass_ets = match placeholders {
            WeaponType::Sword => {
                let vector = directory_search("./static/images/weapons/swords1h/ActionLoot_([0-9])*.png");
                let toto = vector.choose(&mut rand::thread_rng()).unwrap();
                let string = toto.to_string();
                (string, Slot::RightHand)
            },
            WeaponType::Axe =>{
                let vector = directory_search("./static/images/weapons/axes1h/ActionLoot_([0-9])*.png");
                let toto = vector.choose(&mut rand::thread_rng()).unwrap();
                let string = toto.to_string();
                (string, Slot::RightHand)
            },
            WeaponType::Bow =>{
                let vector = directory_search("./static/images/weapons/bows/ActionLoot_([0-9])*.png");
                let toto = vector.choose(&mut rand::thread_rng()).unwrap();
                let string = toto.to_string();
                (string, Slot::RightHand)
            },
            WeaponType::Dagger =>{
                let vector = directory_search("./static/images/weapons/daggers/ActionLoot_([0-9])*.png");
                let toto = vector.choose(&mut rand::thread_rng()).unwrap();
                let string = toto.to_string();
                (string, Slot::RightHand)
            },
            WeaponType::Mace =>{
                let vector = directory_search("./static/images/weapons/maces/ActionLoot_([0-9])*.png");
                let toto = vector.choose(&mut rand::thread_rng()).unwrap();
                let string = toto.to_string();
                (string, Slot::RightHand)
            },
            WeaponType::TwoHandsAxe =>{
                let vector = directory_search("./static/images/weapons/axes2h/ActionLoot_([0-9])*.png");
                let toto = vector.choose(&mut rand::thread_rng()).unwrap();
                let string = toto.to_string();
                (string, Slot::TwoHand)
            },
            WeaponType::TwoHandSword =>{
                let vector = directory_search("./static/images/weapons/swords2h/ActionLoot_([0-9])*.png");
                let toto = vector.choose(&mut rand::thread_rng()).unwrap();
                let string = toto.to_string();
                (string, Slot::TwoHand)
            },
            WeaponType::Spear =>{
                let vector = directory_search("./static/images/weapons/spears/ActionLoot_([0-9])*.png");
                let toto = vector.choose(&mut rand::thread_rng()).unwrap();
                let string = toto.to_string();
                (string, Slot::TwoHand)
            },
            WeaponType::Staff =>{
                let vector = directory_search("./static/images/weapons/staff/ActionLoot_([0-9])*.png");
                let toto = vector.choose(&mut rand::thread_rng()).unwrap();
                let string = toto.to_string();
                (string, Slot::TwoHand)
            },
            
        };

        Object {
            id: id,
            name: globaltuple.0,
            ilevel: ilvltuple.0,
            item_type:  ItemType::Weapon,
            rarity: ilvltuple.1,
            asset: ass_ets.0.to_string(),
            equipement: Some(
                Equipment{
                    slot: ass_ets.1,
                    equipped: false,
                    weapon:Some(
                        Weapon{
                            min_damage: damagetuple.0,
                            max_damage: damagetuple.1,
                            weapon_type: placeholders,
                            element: None,
                            ailment: Some(globaltuple.1.unwrap()),
                        }
                    ),
                    jewel:None,
                    armor:None,
                }
            ),
            caracteristics_augmentation: None,
            special: None
        }
    }

}


impl Armor {
    fn generate() -> Object {
        let mut rng = rand::thread_rng();

        let id = rng.gen::<u32>();
        let ilvltuple = rarity_impl();
        let armor = calculate_armor(ilvltuple.0);
        let wpn_ailment: WpnAilment = rand::random();
        let placeholders: ArmorType = rand::random();

        let globaltuple = if  ilvltuple.1 == String::from("magic")  || ilvltuple.1 == String::from("rare") || ilvltuple.1 == String::from("epic")
        {
            let ailmenttuple = match wpn_ailment {
                WpnAilment::Poison => {
                    let wpn_alignment_power: PoisonAliment = rand::random();
                    let mut name :String = wpn_alignment_power.as_static().to_owned();
                    name.push_str(&placeholders.to_string());
                    let itm_name = name; // Toxic dagger

                    let caracteristics_augmentation = Some(WpnAilment::Poison);
                    let res = calculate_resistances(ilvltuple.0);

                    (itm_name, caracteristics_augmentation, res)
                },
                WpnAilment::Stun => {
                    let wpn_alignment_power: StunAliment = rand::random();
                    let mut name :String = wpn_alignment_power.as_static().to_owned();
                    name.push_str(&placeholders.to_string());
                    let itm_name =  name; // Toxic dagger

                    let caracteristics_augmentation = Some(WpnAilment::Stun);
                    let res = calculate_resistances(ilvltuple.0);

                    (itm_name, caracteristics_augmentation, res)
                },
                _ => {
                    let result = match ilvltuple.1.as_str() {
                        "magic" =>{
                            let mut s = String::from("magic");
                            s.push_str(&placeholders.to_string());
                            (s, Some(WpnAilment::None), 0)
                        },
                       "rare" =>{
                            let mut s = String::from("rare");
                            s.push_str(&placeholders.to_string());
                            (s, Some(WpnAilment::None), 0)
                        },
                        "epic" =>{
                            let mut s = String::from("epic");
                            s.push_str(&placeholders.to_string());
                            (s, Some(WpnAilment::None), 0)
                        }
                        _ => {
                            (String::from("none"), Some(WpnAilment::None), 0)
                        }
                    };
                    result

                }
    
            };

            ailmenttuple

        } else if ilvltuple.1 == String::from("common") {
            let mut s = String::from("common");
             s.push_str(&placeholders.to_string());
            (s, Some(WpnAilment::None), 0)
        }else{
            let mut s = String::from("legendary");
             s.push_str(&placeholders.to_string());
            (s, Some(WpnAilment::None), 0)
        };

        let ass_ets = match placeholders {
            ArmorType::Chest => {
                let vector = directory_search("./static/images/armors/chests/ActionLoot_([0-9])*.png");
                let toto = vector.choose(&mut rand::thread_rng()).unwrap();
                let string = toto.to_string();
                (string, Slot::Chest)
            },
            ArmorType::Belt =>{
                let vector = directory_search("./static/images/armors/belt/ActionLoot_([0-9])*.png");
                let toto = vector.choose(&mut rand::thread_rng()).unwrap();
                let string = toto.to_string();
                (string, Slot::Belt)
            },
            ArmorType::Boot =>{
                let vector = directory_search("./static/images/armors/feet/ActionLoot_([0-9])*.png");
                let toto = vector.choose(&mut rand::thread_rng()).unwrap();
                let string = toto.to_string();
                (string, Slot::Boot)
            },
            ArmorType::Glove =>{
                let vector = directory_search("./static/images/armors/gloves/ActionLoot_([0-9])*.png");
                let toto = vector.choose(&mut rand::thread_rng()).unwrap();
                let string = toto.to_string();
                (string, Slot::Glove)
            },
            ArmorType::Helmet =>{
                let vector = directory_search("./static/images/armors/helmets/ActionLoot_([0-9])*.png");
                let toto = vector.choose(&mut rand::thread_rng()).unwrap();
                let string = toto.to_string();
                (string, Slot::Helmet)
            },
            ArmorType::Shield =>{
                let vector = directory_search("./static/images/armors/shields/ActionLoot_([0-9])*.png");
                let toto = vector.choose(&mut rand::thread_rng()).unwrap();
                let string = toto.to_string();
                (string, Slot::Shield)
            },
            ArmorType::Wrist =>{
                let vector = directory_search("./static/images/armors/wrists/ActionLoot_([0-9])*.png");
                let toto = vector.choose(&mut rand::thread_rng()).unwrap();
                let string = toto.to_string();
                (string, Slot::Wrist)
            },
            
        };

        let armor_class: ArmorClass = rand::random();
        info!("object: {:?}", ass_ets);

        Object {
            id: id,
            name: globaltuple.0,
            ilevel: ilvltuple.0,
            item_type: ItemType::Armor,
            rarity: ilvltuple.1,
            asset: ass_ets.0.to_string(),
            equipement: Some(
                Equipment{
                    slot: ass_ets.1,
                    equipped: false,
                    weapon:None,
                    armor:Some(
                        Armor{
                            armor: armor,
                            armor_type: placeholders,
                            class: armor_class,
                            resistances: globaltuple.2,
                            ailment: Some(globaltuple.1.unwrap()),
                        }
                    ),
                    jewel:None,
                }
            ),
            caracteristics_augmentation: None,
            special: None
        }
    }
}


impl Object{
    fn generate(name: String, rarity: String, special: u16) -> Object {

        let mut rng = rand::thread_rng();

        //TODO:faire uyn enum pour traiter les consommables
        let assets: String = match name.as_str() {
            "health potion" => {
               let to_return = String::from("./static/images/items/ActionLoot_(6).png");
               to_return
            },
            _ => { 
                let to_return = String::from("./static/images/items/ActionLoot_(6).png");
                to_return
            }
        };

        Object {
            id: rng.gen::<u32>(),
            name: name,
            ilevel: 1,
            asset: assets,
            item_type: ItemType::Consumable,
            rarity: rarity,
            equipement: None,
            caracteristics_augmentation: None,
            special: Some(Special{heal:special})
        }
    }
}

impl Jewel{
    fn generate(rarity: String) -> Object {

        let mut rng = rand::thread_rng();
        let placeholders: JewelType = rand::random();
        let ilvltuple = rarity_impl();
        let low_lvl_spell: LowLevelSpells = rand::random();
        let low_lvl_spell_string: String = low_lvl_spell.to_string();
        let stats_tuple:  (String, Incante) = jewel_stats(&low_lvl_spell);

        let ass_ets = match placeholders {
            JewelType::Necklace =>{
                let vector = directory_search("./static/images/armors/necklaces/ActionLoot_([0-9])*.png");
                let toto = vector.choose(&mut rand::thread_rng()).unwrap();
                let string = toto.to_string();
                (string, Slot::Necklace, JewelType::Necklace)
            },
            JewelType::Ring =>{
                let vector = directory_search("./static/images/armors/rings/ActionLoot_([0-9])*.png");
                let toto = vector.choose(&mut rand::thread_rng()).unwrap();
                let string = toto.to_string();
                (string, Slot::Ring, JewelType::Ring)
            },
        };

        let name = if ilvltuple.1 == String::from("common") {
            let mut s = String::from("common");
            s.push_str(&placeholders.to_string());
            s
        } else if ilvltuple.1 == String::from("magic") {
            let mut s = String::from("magic");
                s.push_str(&placeholders.to_string());
            s
        } else if ilvltuple.1 == String::from("rare") {
            let mut s = String::from("rare");
                s.push_str(&placeholders.to_string());
            s
        } else if ilvltuple.1 == String::from("epic") {
            let mut s = String::from("epic");
                s.push_str(&placeholders.to_string());
            s
        }else{
            let mut s = String::from("legendary");
            s.push_str(&placeholders.to_string());
            s
        };

        Object {
            id: rng.gen::<u32>(),
            name: name,
            ilevel: 1,
            asset: ass_ets.0.to_string(),
            item_type: ItemType::Jewel,
            rarity: rarity,
            equipement: Some(
                Equipment{
                    slot: ass_ets.1,
                    equipped: false,
                    weapon:None,
                    armor:None,
                    jewel:Some(
                        Jewel{ 
                            jewel_type: ass_ets.2,
                            spell: Spell {
                                name: low_lvl_spell,
                                incante: stats_tuple.1,
                                damage: stats_tuple.0,
                                description: low_lvl_spell_string
                            },
                            usage: 2,
                            ailment: None,
                        }
                    ),
                }
            ),
            caracteristics_augmentation: None,
            special: None
        }

    }
}


pub fn generate_loot() -> Object {

    let item_pool: ItemType = rand::random();

    let loot = match item_pool {
        ItemType::Weapon =>{
            Weapon::generate()
        },
        ItemType::Armor =>{
            Armor::generate()
        }
        ItemType::Consumable=>{
            Object::generate(String::from("health potion"), String::from("common"), 25)
        }
        ItemType::Jewel=>{
            Jewel::generate(String::from("common"))
        }
    };

    loot
}