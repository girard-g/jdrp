// extern crate strum; // 0.10.0
use strum::AsStaticRef;

use crate::item_generator::ailment::{PoisonAliment, StunAliment, WpnAilment};
use crate::item_generator::item::{Consumable, ConsumableType, UseCallback};
use crate::item_generator::monster::{
    BaseTypeMonster, DeathCallback, Monster, MonsterStats, MonsterType,
};
use crate::item_generator::resources::{
    Armor, ArmorClass, ArmorType, Equipment, ItemType, Jewel, JewelType, Loot, Object, Slot,
    Weapon, WeaponType,
};
use crate::item_generator::spell::{Incante, LowLevelSpells, Spell, SpellDamage};
use rand::{
    distributions::{Distribution, WeightedIndex},
    Rng,
};
// use crate::item_generator::element::{WpnElement};
use glob::glob;
use rand::seq::SliceRandom;
use std::str::FromStr;

const PLAYER_LEVEL: u8 = 20;

struct Transition {
    level: u8,
    value: f32,
}

fn rarity_impl() -> (u8, String) {
    fn chances(table: &[Transition], level: u8) -> f32 {
        table
            .iter()
            .rev()
            .find(|transition| level >= transition.level)
            .map_or(0.0, |transition| transition.value)
    }

    let common_chance = chances(
        &[
            Transition {
                level: 1,
                value: 90.0,
            },
            Transition {
                level: 10,
                value: 70.0,
            },
            Transition {
                level: 20,
                value: 55.0,
            },
        ],
        PLAYER_LEVEL,
    );
    let magic_chance = chances(
        &[
            Transition {
                level: 1,
                value: 5.0,
            },
            Transition {
                level: 10,
                value: 15.0,
            },
            Transition {
                level: 20,
                value: 20.0,
            },
        ],
        PLAYER_LEVEL,
    );
    let rare_chance = chances(
        &[
            Transition {
                level: 1,
                value: 2.0,
            },
            Transition {
                level: 10,
                value: 4.0,
            },
            Transition {
                level: 20,
                value: 5.0,
            },
        ],
        PLAYER_LEVEL,
    );
    let epic_chance = chances(
        &[
            Transition {
                level: 1,
                value: 1.0,
            },
            Transition {
                level: 10,
                value: 1.25,
            },
            Transition {
                level: 20,
                value: 2.0,
            },
        ],
        PLAYER_LEVEL,
    );
    let legendary_chance = chances(
        &[
            Transition {
                level: 1,
                value: 0.01,
            },
            Transition {
                level: 10,
                value: 0.1,
            },
            Transition {
                level: 20,
                value: 0.2,
            },
        ],
        PLAYER_LEVEL,
    );

    let choices = ["common", "magic", "rare", "epic", "legendary"];

    let weights = [
        common_chance,
        magic_chance,
        rare_chance,
        epic_chance,
        legendary_chance,
    ];
    let rariry_choice = WeightedIndex::new(&weights).unwrap();

    match choices[rariry_choice.sample(&mut rand::thread_rng())] {
        "common" => {
            let mut rng = rand::thread_rng();
            (rng.gen_range(0, 50), String::from("common"))
        }
        "magic" => {
            let mut rng = rand::thread_rng();
            (rng.gen_range(51, 90), String::from("magic"))
        }
        "rare" => {
            let mut rng = rand::thread_rng();
            (rng.gen_range(91, 140), String::from("rare"))
        }
        "epic" => {
            let mut rng = rand::thread_rng();
            (rng.gen_range(141, 190), String::from("epic"))
        }
        "legendary" => {
            let mut rng = rand::thread_rng();
            (rng.gen_range(191, 255), String::from("legendary"))
        }
        _ => unreachable!(),
    }
}

fn calculate_weapon_damage(ilvl: u8) -> (u16, u16) {
    let mut rng = rand::thread_rng();

    match ilvl {
        0..=50 => {
            let min_damage = rng.gen_range(3, 125);
            let max_damage = rng.gen_range(min_damage, 158);
            (min_damage, max_damage)
        }
        51..=90 => {
            let min_damage = rng.gen_range(75, 200);
            let max_damage = rng.gen_range(min_damage, 253);
            (min_damage, max_damage)
        }
        91..=140 => {
            let min_damage = rng.gen_range(150, 349);
            let max_damage = rng.gen_range(min_damage, 442);
            (min_damage, max_damage)
        }
        141..=190 => {
            let min_damage = rng.gen_range(299, 473);
            let max_damage = rng.gen_range(min_damage, 599);
            (min_damage, max_damage)
        }
        191..=255 => {
            let min_damage = rng.gen_range(498, 635);
            let max_damage = rng.gen_range(min_damage, 804);
            (min_damage, max_damage)
        }
    }
}

fn calculate_armor(ilvl: u8) -> u16 {
    let mut rng = rand::thread_rng();

    match ilvl {
        0..=50 => {
            let min_damage = rng.gen_range(3, 125);
            let max_damage = rng.gen_range(min_damage, 158);
            (min_damage + max_damage) / 2
        }
        51..=90 => {
            let min_damage = rng.gen_range(75, 200);
            let max_damage = rng.gen_range(min_damage, 253);
            (min_damage + max_damage) / 2
        }
        91..=140 => {
            let min_damage = rng.gen_range(150, 349);
            let max_damage = rng.gen_range(min_damage, 442);
            (min_damage + max_damage) / 2
        }
        141..=190 => {
            let min_damage = rng.gen_range(299, 473);
            let max_damage = rng.gen_range(min_damage, 599);
            (min_damage + max_damage) / 2
        }
        191..=255 => {
            let min_damage = rng.gen_range(498, 635);
            let max_damage = rng.gen_range(min_damage, 804);
            (min_damage + max_damage) / 2
        }
    }
}

fn calculate_resistances(ilvl: u8) -> u8 {
    let mut rng = rand::thread_rng();

    match ilvl {
        0..=50 => rng.gen_range(4, 8),
        51..=90 => rng.gen_range(8, 15),
        91..=140 => rng.gen_range(15, 25),
        141..=190 => rng.gen_range(25, 35),
        191..=255 => rng.gen_range(35, 40),
    }
}

fn directory_search(str_to_search: &str) -> Vec<String> {
    let mut vec1 = vec![];

    for path in glob(str_to_search).unwrap().filter_map(Result::ok) {
        // println!("{}", path.display());
        vec1.push(path.display().to_string());
    }

    vec1
}

fn jewel_stats(l: &LowLevelSpells) -> (String, Incante) {
    match l {
        LowLevelSpells::AnimationDesMorts => (String::from(""), Incante::Concentration),
        LowLevelSpells::RayonEmpoisonne => (SpellDamage::D1d8.to_string(), Incante::Action),
        LowLevelSpells::RayonAffaiblissant => (String::from(""), Incante::Action),
        LowLevelSpells::ToucherDuVampire => (SpellDamage::D3d6.to_string(), Incante::Action),
        LowLevelSpells::Amis => (String::from(""), Incante::Action),
        LowLevelSpells::RepresailleInfernales => {
            (SpellDamage::D2d10.to_string(), Incante::Reaction)
        }
        LowLevelSpells::LameTonnante => (SpellDamage::D1d8.to_string(), Incante::Action),
        LowLevelSpells::LameAuxFlammesVertes => (SpellDamage::D2d10.to_string(), Incante::Action),
        LowLevelSpells::Gelure => (SpellDamage::D1d6.to_string(), Incante::Action),
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
    fn generate() -> Loot {
        let mut rng = rand::thread_rng();

        let id = rng.gen::<u32>();
        let ilvltuple = rarity_impl();
        let damagetuple = calculate_weapon_damage(ilvltuple.0);
        let wpn_ailment: WpnAilment = rand::random();
        let placeholders: WeaponType = rand::random();

        let globaltuple =
            if ilvltuple.1 == *"magic" || ilvltuple.1 == *"rare" || ilvltuple.1 == *"epic" {
                let ailmenttuple = match wpn_ailment {
                    WpnAilment::Poison => {
                        let wpn_alignment_power: PoisonAliment = rand::random();
                        let mut name: String = wpn_alignment_power.as_static().to_owned();
                        name.push_str(&placeholders.to_string());
                        let itm_name = name; // Toxic dagger

                        let caracteristics_augmentation =
                            Some((WpnAilment::Poison, wpn_alignment_power as u8));
                        (itm_name, caracteristics_augmentation)
                    }
                    WpnAilment::Stun => {
                        let wpn_alignment_power: StunAliment = rand::random();
                        let mut name: String = wpn_alignment_power.as_static().to_owned();
                        name.push_str(&placeholders.to_string());
                        let itm_name = name; // Toxic dagger

                        let caracteristics_augmentation =
                            Some((WpnAilment::Stun, wpn_alignment_power as u8));
                        (itm_name, caracteristics_augmentation)
                    }
                    _ => {
                        let result = match ilvltuple.1.as_str() {
                            "magic" => {
                                let mut s = String::from("magic");
                                s.push_str(&placeholders.to_string());
                                (s, Some((WpnAilment::None, 0 as u8)))
                            }
                            "rare" => {
                                let mut s = String::from("rare");
                                s.push_str(&placeholders.to_string());
                                (s, Some((WpnAilment::None, 0 as u8)))
                            }
                            "epic" => {
                                let mut s = String::from("epic");
                                s.push_str(&placeholders.to_string());
                                (s, Some((WpnAilment::None, 0 as u8)))
                            }
                            _ => (String::from("none"), Some((WpnAilment::None, 0 as u8))),
                        };
                        result
                    }
                };

                ailmenttuple
            } else if ilvltuple.1 == *"common" {
                let mut s = String::from("common");
                s.push_str(&placeholders.to_string());
                (s, Some((WpnAilment::None, 0 as u8)))
            } else {
                let mut s = String::from("legendary");
                s.push_str(&placeholders.to_string());
                (s, Some((WpnAilment::None, 0 as u8)))
            };

        let ass_ets = match placeholders {
            WeaponType::Sword => {
                let vector =
                    directory_search("./static/images/weapons/swords1h/ActionLoot_([0-9])*.png");
                let toto = vector.choose(&mut rand::thread_rng()).unwrap();
                let string = toto.to_string();
                (string, Slot::RightHand)
            }
            WeaponType::Axe => {
                let vector =
                    directory_search("./static/images/weapons/axes1h/ActionLoot_([0-9])*.png");
                let toto = vector.choose(&mut rand::thread_rng()).unwrap();
                let string = toto.to_string();
                (string, Slot::RightHand)
            }
            WeaponType::Bow => {
                let vector =
                    directory_search("./static/images/weapons/bows/ActionLoot_([0-9])*.png");
                let toto = vector.choose(&mut rand::thread_rng()).unwrap();
                let string = toto.to_string();
                (string, Slot::RightHand)
            }
            WeaponType::Dagger => {
                let vector =
                    directory_search("./static/images/weapons/daggers/ActionLoot_([0-9])*.png");
                let toto = vector.choose(&mut rand::thread_rng()).unwrap();
                let string = toto.to_string();
                (string, Slot::RightHand)
            }
            WeaponType::Mace => {
                let vector =
                    directory_search("./static/images/weapons/maces/ActionLoot_([0-9])*.png");
                let toto = vector.choose(&mut rand::thread_rng()).unwrap();
                let string = toto.to_string();
                (string, Slot::RightHand)
            }
            WeaponType::TwoHandsAxe => {
                let vector =
                    directory_search("./static/images/weapons/axes2h/ActionLoot_([0-9])*.png");
                let toto = vector.choose(&mut rand::thread_rng()).unwrap();
                let string = toto.to_string();
                (string, Slot::TwoHand)
            }
            WeaponType::TwoHandSword => {
                let vector =
                    directory_search("./static/images/weapons/swords2h/ActionLoot_([0-9])*.png");
                let toto = vector.choose(&mut rand::thread_rng()).unwrap();
                let string = toto.to_string();
                (string, Slot::TwoHand)
            }
            WeaponType::Spear => {
                let vector =
                    directory_search("./static/images/weapons/spears/ActionLoot_([0-9])*.png");
                let toto = vector.choose(&mut rand::thread_rng()).unwrap();
                let string = toto.to_string();
                (string, Slot::TwoHand)
            }
            WeaponType::Staff => {
                let vector =
                    directory_search("./static/images/weapons/staff/ActionLoot_([0-9])*.png");
                let toto = vector.choose(&mut rand::thread_rng()).unwrap();
                let string = toto.to_string();
                (string, Slot::TwoHand)
            }
        };

        Loot {
            object: Some(Object {
                id,
                name: globaltuple.0,
                ilevel: ilvltuple.0,
                item_type: ItemType::Weapon,
                rarity: ilvltuple.1,
                asset: ass_ets.0.to_string(),
                equipement: Some(Equipment {
                    slot: ass_ets.1,
                    equipped: false,
                    weapon: Some(Weapon {
                        min_damage: damagetuple.0,
                        max_damage: damagetuple.1,
                        weapon_type: placeholders,
                        element: None,
                        ailment: Some(globaltuple.1.unwrap()),
                    }),
                    jewel: None,
                    armor: None,
                }),
                caracteristics_augmentation: None,
            }),
            consumable: None,
        }
    }
}

impl Armor {
    fn generate() -> Loot {
        let mut rng = rand::thread_rng();

        let id = rng.gen::<u32>();
        let ilvltuple = rarity_impl();
        let armor = calculate_armor(ilvltuple.0);
        let wpn_ailment: WpnAilment = rand::random();
        let placeholders: ArmorType = rand::random();

        let globaltuple =
            if ilvltuple.1 == *"magic" || ilvltuple.1 == *"rare" || ilvltuple.1 == *"epic" {
                let ailmenttuple = match wpn_ailment {
                    WpnAilment::Poison => {
                        let wpn_alignment_power: PoisonAliment = rand::random();
                        let mut name: String = wpn_alignment_power.as_static().to_owned();
                        name.push_str(&placeholders.to_string());
                        let itm_name = name; // Toxic dagger

                        let caracteristics_augmentation = Some(WpnAilment::Poison);
                        let res = calculate_resistances(ilvltuple.0);

                        (itm_name, caracteristics_augmentation, res)
                    }
                    WpnAilment::Stun => {
                        let wpn_alignment_power: StunAliment = rand::random();
                        let mut name: String = wpn_alignment_power.as_static().to_owned();
                        name.push_str(&placeholders.to_string());
                        let itm_name = name; // Toxic dagger

                        let caracteristics_augmentation = Some(WpnAilment::Stun);
                        let res = calculate_resistances(ilvltuple.0);

                        (itm_name, caracteristics_augmentation, res)
                    }
                    _ => {
                        let result = match ilvltuple.1.as_str() {
                            "magic" => {
                                let mut s = String::from("magic");
                                s.push_str(&placeholders.to_string());
                                (s, Some(WpnAilment::None), 0)
                            }
                            "rare" => {
                                let mut s = String::from("rare");
                                s.push_str(&placeholders.to_string());
                                (s, Some(WpnAilment::None), 0)
                            }
                            "epic" => {
                                let mut s = String::from("epic");
                                s.push_str(&placeholders.to_string());
                                (s, Some(WpnAilment::None), 0)
                            }
                            _ => (String::from("none"), Some(WpnAilment::None), 0),
                        };
                        result
                    }
                };

                ailmenttuple
            } else if ilvltuple.1 == *"common" {
                let mut s = String::from("common");
                s.push_str(&placeholders.to_string());
                (s, Some(WpnAilment::None), 0)
            } else {
                let mut s = String::from("legendary");
                s.push_str(&placeholders.to_string());
                (s, Some(WpnAilment::None), 0)
            };

        let ass_ets = match placeholders {
            ArmorType::Chest => {
                let vector =
                    directory_search("./static/images/armors/chests/ActionLoot_([0-9])*.png");
                let toto = vector.choose(&mut rand::thread_rng()).unwrap();
                let string = toto.to_string();
                (string, Slot::Chest)
            }
            ArmorType::Belt => {
                let vector =
                    directory_search("./static/images/armors/belt/ActionLoot_([0-9])*.png");
                let toto = vector.choose(&mut rand::thread_rng()).unwrap();
                let string = toto.to_string();
                (string, Slot::Belt)
            }
            ArmorType::Boot => {
                let vector =
                    directory_search("./static/images/armors/feet/ActionLoot_([0-9])*.png");
                let toto = vector.choose(&mut rand::thread_rng()).unwrap();
                let string = toto.to_string();
                (string, Slot::Boot)
            }
            ArmorType::Glove => {
                let vector =
                    directory_search("./static/images/armors/gloves/ActionLoot_([0-9])*.png");
                let toto = vector.choose(&mut rand::thread_rng()).unwrap();
                let string = toto.to_string();
                (string, Slot::Glove)
            }
            ArmorType::Helmet => {
                let vector =
                    directory_search("./static/images/armors/helmets/ActionLoot_([0-9])*.png");
                let toto = vector.choose(&mut rand::thread_rng()).unwrap();
                let string = toto.to_string();
                (string, Slot::Helmet)
            }
            ArmorType::Shield => {
                let vector =
                    directory_search("./static/images/armors/shields/ActionLoot_([0-9])*.png");
                let toto = vector.choose(&mut rand::thread_rng()).unwrap();
                let string = toto.to_string();
                (string, Slot::Shield)
            }
            ArmorType::Wrist => {
                let vector =
                    directory_search("./static/images/armors/wrists/ActionLoot_([0-9])*.png");
                let toto = vector.choose(&mut rand::thread_rng()).unwrap();
                let string = toto.to_string();
                (string, Slot::Wrist)
            }
        };

        let armor_class: ArmorClass = rand::random();
        info!("object: {:?}", ass_ets);

        Loot {
            object: Some(Object {
                id,
                name: globaltuple.0,
                ilevel: ilvltuple.0,
                item_type: ItemType::Armor,
                rarity: ilvltuple.1,
                asset: ass_ets.0.to_string(),
                equipement: Some(Equipment {
                    slot: ass_ets.1,
                    equipped: false,
                    weapon: None,
                    armor: Some(Armor {
                        armor,
                        armor_type: placeholders,
                        class: armor_class,
                        resistances: globaltuple.2,
                        ailment: Some(globaltuple.1.unwrap()),
                    }),
                    jewel: None,
                }),
                caracteristics_augmentation: None,
            }),
            consumable: None,
        }
    }
}

impl Object {
    fn generate() -> Loot {
        let mut rng = rand::thread_rng();

        let placeholders: ConsumableType = rand::random();

        let ass_ets = match placeholders {
            ConsumableType::HealthPotion => {
                let vector = directory_search("./static/images/items/ActionLoot_(6).png");
                let string = vector[0].to_string();
                (
                    string,
                    String::from("Potion de vie"),
                    ConsumableType::HealthPotion,
                    UseCallback::HealthPotion,
                )
            }
            ConsumableType::Bandages => {
                let vector = directory_search("./static/images/items/ActionLoot_(9).png");
                let string = vector[0].to_string();
                (
                    string,
                    String::from("Bandage"),
                    ConsumableType::Bandages,
                    UseCallback::Bandages,
                )
            }
            ConsumableType::Antidot => {
                let vector = directory_search("./static/images/items/ActionLoot_(8).png");
                let string = vector[0].to_string();
                (
                    string,
                    String::from("Antidote"),
                    ConsumableType::Antidot,
                    UseCallback::Antidot,
                )
            }
        };

        Loot {
            object: None,
            consumable: Some(Consumable {
                id: rng.gen::<u32>(),
                name: ass_ets.1,
                ilevel: 1,
                item_type: ass_ets.2,
                asset: ass_ets.0,
                rarity: String::from("Common"),
                description: ass_ets.2.to_string(),
                on_use: ass_ets.3,
            }),
        }
    }
}

impl Jewel {
    fn generate(rarity: String) -> Loot {
        let mut rng = rand::thread_rng();
        let placeholders: JewelType = rand::random();
        let ilvltuple = rarity_impl();
        let low_lvl_spell: LowLevelSpells = rand::random();
        let low_lvl_spell_string: String = low_lvl_spell.to_string();
        let stats_tuple: (String, Incante) = jewel_stats(&low_lvl_spell);

        let ass_ets = match placeholders {
            JewelType::Necklace => {
                let vector =
                    directory_search("./static/images/armors/necklaces/ActionLoot_([0-9])*.png");
                let toto = vector.choose(&mut rand::thread_rng()).unwrap();
                let string = toto.to_string();
                (string, Slot::Necklace, JewelType::Necklace)
            }
            JewelType::Ring => {
                let vector =
                    directory_search("./static/images/armors/rings/ActionLoot_([0-9])*.png");
                let toto = vector.choose(&mut rand::thread_rng()).unwrap();
                let string = toto.to_string();
                (string, Slot::Ring, JewelType::Ring)
            }
        };

        let name = if ilvltuple.1 == *"common" {
            let mut s = String::from("common");
            s.push_str(&placeholders.to_string());
            s
        } else if ilvltuple.1 == *"magic" {
            let mut s = String::from("magic");
            s.push_str(&placeholders.to_string());
            s
        } else if ilvltuple.1 == *"rare" {
            let mut s = String::from("rare");
            s.push_str(&placeholders.to_string());
            s
        } else if ilvltuple.1 == *"epic" {
            let mut s = String::from("epic");
            s.push_str(&placeholders.to_string());
            s
        } else {
            let mut s = String::from("legendary");
            s.push_str(&placeholders.to_string());
            s
        };

        Loot {
            object: Some(Object {
                id: rng.gen::<u32>(),
                name,
                ilevel: 1,
                asset: ass_ets.0.to_string(),
                item_type: ItemType::Jewel,
                rarity,
                equipement: Some(Equipment {
                    slot: ass_ets.1,
                    equipped: false,
                    weapon: None,
                    armor: None,
                    jewel: Some(Jewel {
                        jewel_type: ass_ets.2,
                        spell: Spell {
                            name: low_lvl_spell,
                            incante: stats_tuple.1,
                            damage: stats_tuple.0,
                            description: low_lvl_spell_string,
                        },
                        usage: 2,
                        ailment: None,
                    }),
                }),
                caracteristics_augmentation: None,
            }),
            consumable: None,
        }
    }
}

// fn rarity_monster() -> (u8, String) {
//     fn chances(table: &[Transition], level: u8) -> f32 {
//         table
//             .iter()
//             .rev()
//             .find(|transition| level >= transition.level)
//             .map_or(0.0, |transition| transition.value)
//     }

// let common_chance = chances(
//     &[
//         Transition {
//             level: 1,
//             value: 90.0,
//         },
//         Transition {
//             level: 10,
//             value: 70.0,
//         },
//         Transition {
//             level: 20,
//             value: 55.0,
//         },
//     ],
//     PLAYER_LEVEL,
// );
// let magic_chance = chances(
//     &[
//         Transition {
//             level: 1,
//             value: 5.0,
//         },
//         Transition {
//             level: 10,
//             value: 15.0,
//         },
//         Transition {
//             level: 20,
//             value: 20.0,
//         },
//     ],
//     PLAYER_LEVEL,
// );
// let rare_chance = chances(
//     &[
//         Transition {
//             level: 1,
//             value: 2.0,
//         },
//         Transition {
//             level: 10,
//             value: 4.0,
//         },
//         Transition {
//             level: 20,
//             value: 5.0,
//         },
//     ],
//     PLAYER_LEVEL,
// );
// let epic_chance = chances(
//     &[
//         Transition {
//             level: 1,
//             value: 1.0,
//         },
//         Transition {
//             level: 10,
//             value: 1.25,
//         },
//         Transition {
//             level: 20,
//             value: 2.0,
//         },
//     ],
//     PLAYER_LEVEL,
// );
// let legendary_chance = chances(
//     &[
//         Transition {
//             level: 1,
//             value: 0.01,
//         },
//         Transition {
//             level: 10,
//             value: 0.1,
//         },
//         Transition {
//             level: 20,
//             value: 0.2,
//         },
//     ],
//     PLAYER_LEVEL,
// );

// let choices = ["common", "magic", "rare", "epic", "legendary"];

// let weights = [
//     common_chance,
//     magic_chance,
//     rare_chance,
//     epic_chance,
//     legendary_chance,
// ];
// let rariry_choice = WeightedIndex::new(&weights).unwrap();

// match choices[rariry_choice.sample(&mut rand::thread_rng())] {
//     "common" => {
//         let mut rng = rand::thread_rng();
//         (rng.gen_range(0, 50), String::from("common"))
//     }
//     "magic" => {
//         let mut rng = rand::thread_rng();
//         (rng.gen_range(51, 90), String::from("magic"))
//     }
//     "rare" => {
//         let mut rng = rand::thread_rng();
//         (rng.gen_range(91, 140), String::from("rare"))
//     }
//     "epic" => {
//         let mut rng = rand::thread_rng();
//         (rng.gen_range(141, 190), String::from("epic"))
//     }
//     "legendary" => {
//         let mut rng = rand::thread_rng();
//         (rng.gen_range(191, 255), String::from("legendary"))
//     }
//     _ => unreachable!(),
// };
// }

pub fn calculate_monster_stats(base: BaseTypeMonster) -> MonsterStats {
    let mut rng = rand::thread_rng();

    let initiative = rng.gen_range(10, 70);
    let endurance = rng.gen_range(10, 70);
    let willpower = rng.gen_range(20, 70);
    let dexterity = rng.gen_range(20, 70);
    let strenght = rng.gen_range(20, 70);

    let monster_stats = match base {
        BaseTypeMonster::Strengh => (willpower - 20, dexterity - 10, strenght + 20),
        BaseTypeMonster::Dexterity => (willpower - 10, dexterity + 20, strenght - 20),
        BaseTypeMonster::Willpower => (willpower + 20, dexterity - 10, strenght - 20),
    };

    MonsterStats {
        strenght: monster_stats.2,
        dexterity: monster_stats.1,
        willpower: monster_stats.0,
        endurance,
        initiative,
    }
}

impl Monster {
    fn generate(monster_type: String) -> Monster {
        let mut rng = rand::thread_rng();

        let id = rng.gen::<u32>();
        // TODO impl rarity to monsters

        let tuple_rng: (u8, u8) = match PLAYER_LEVEL {
            0..=2 => {
                let ivlvmonster: u8 = rng.gen_range(0, PLAYER_LEVEL + 2);
                let coef_playerlvl: u8 = rng.gen_range(0, PLAYER_LEVEL + 3);
                (ivlvmonster, coef_playerlvl)
            }
            3 => {
                let ivlvmonster: u8 = rng.gen_range(PLAYER_LEVEL - 2, PLAYER_LEVEL + 2);
                let coef_playerlvl: u8 = rng.gen_range(0, PLAYER_LEVEL + 3);
                (ivlvmonster, coef_playerlvl)
            }
            _ => {
                let ivlvmonster: u8 = rng.gen_range(PLAYER_LEVEL - 2, PLAYER_LEVEL + 2);
                let coef_playerlvl: u8 = rng.gen_range(PLAYER_LEVEL - 3, PLAYER_LEVEL + 3);
                (ivlvmonster, coef_playerlvl)
            }
        };

        let ivlvmonster: u8 = tuple_rng.0;
        let monster_constitution = rng.gen_range(40, 50);
        let coef = monster_constitution / 10;
        let coef_playerlvl: u8 = tuple_rng.1;

        let monster_armor = (1 + (PLAYER_LEVEL / 100)) as u16 * calculate_armor(ivlvmonster);
        let tuple_damage = calculate_weapon_damage(ivlvmonster);

        let monster_pv: u32 = ((monster_constitution as u32 + ivlvmonster as u32)
            + ivlvmonster as u32)
            * coef
            * coef_playerlvl as u32;

        let mut monster_resistances = calculate_resistances(ivlvmonster);
        let monster_ailment: WpnAilment = rand::random();

        if monster_ailment == WpnAilment::None {
            monster_resistances = 0;
        }

        let monster_enum = MonsterType::from_str(monster_type.as_ref()).unwrap();

        let assets = match monster_enum {
            MonsterType::Banshee => {
                let ret = directory_search("./static/images/monsters/Tex_banshee.png");
                let stats = calculate_monster_stats(BaseTypeMonster::Willpower);
                (ret, stats)
            }
            MonsterType::Barbarian => {
                let ret = directory_search("./static/images/monsters/Tex_barbarian.png");
                let stats = calculate_monster_stats(BaseTypeMonster::Strengh);
                (ret, stats)
            }
            MonsterType::Daemon => {
                let ret = directory_search("./static/images/monsters/Tex_deamon.png");
                let stats = calculate_monster_stats(BaseTypeMonster::Strengh);
                (ret, stats)
            }
            MonsterType::Ghost => {
                let ret = directory_search("./static/images/monsters/Tex_ghost.png");
                let stats = calculate_monster_stats(BaseTypeMonster::Dexterity);
                (ret, stats)
            }
            MonsterType::Ghoul => {
                let ret = directory_search("./static/images/monsters/Tex_ghoul.png");
                let stats = calculate_monster_stats(BaseTypeMonster::Willpower);
                (ret, stats)
            }
            MonsterType::Goblin => {
                let ret = directory_search("./static/images/monsters/Tex_goblin.png");
                let stats = calculate_monster_stats(BaseTypeMonster::Dexterity);
                (ret, stats)
            }
            MonsterType::Gravedigger => {
                let ret = directory_search("./static/images/monsters/Tex_gravedigger.png");
                let stats = calculate_monster_stats(BaseTypeMonster::Strengh);
                (ret, stats)
            }
            MonsterType::Knight => {
                let ret = directory_search("./static/images/monsters/Tex_knight.png");
                let stats = calculate_monster_stats(BaseTypeMonster::Strengh);
                (ret, stats)
            }
            MonsterType::Lich => {
                let ret = directory_search("./static/images/monsters/Tex_lich.png");
                let stats = calculate_monster_stats(BaseTypeMonster::Willpower);
                (ret, stats)
            }
            MonsterType::Necromancer => {
                let ret = directory_search("./static/images/monsters/Tex_necromancer.png");
                let stats = calculate_monster_stats(BaseTypeMonster::Willpower);
                (ret, stats)
            }
            MonsterType::Rat => {
                let ret = directory_search("./static/images/monsters/Tex_rat.png");
                let stats = calculate_monster_stats(BaseTypeMonster::Dexterity);
                (ret, stats)
            }
            MonsterType::Rogue => {
                let ret = directory_search("./static/images/monsters/Tex_rogue.png");
                let stats = calculate_monster_stats(BaseTypeMonster::Dexterity);
                (ret, stats)
            }
            MonsterType::Skeleton => {
                let ret = directory_search("./static/images/monsters/Tex_skeleton.png");
                let stats = calculate_monster_stats(BaseTypeMonster::Strengh);
                (ret, stats)
            }
            MonsterType::Spider => {
                let ret = directory_search("./static/images/monsters/Tex_spider.png");
                let stats = calculate_monster_stats(BaseTypeMonster::Dexterity);
                (ret, stats)
            }
            MonsterType::Succubus => {
                let ret = directory_search("./static/images/monsters/Tex_succubus.png");
                let stats = calculate_monster_stats(BaseTypeMonster::Willpower);
                (ret, stats)
            }
            MonsterType::Toad => {
                let ret = directory_search("./static/images/monsters/Tex_toad.png");
                let stats = calculate_monster_stats(BaseTypeMonster::Strengh);
                (ret, stats)
            }
            MonsterType::Wasp => {
                let ret = directory_search("./static/images/monsters/Tex_wasp.png");
                let stats = calculate_monster_stats(BaseTypeMonster::Dexterity);
                (ret, stats)
            }
            MonsterType::Werewolf => {
                let ret = directory_search("./static/images/monsters/Tex_werewolf.png");
                let stats = calculate_monster_stats(BaseTypeMonster::Willpower);
                (ret, stats)
            }
            MonsterType::Witch => {
                let ret = directory_search("./static/images/monsters/Tex_witch.png");
                let stats = calculate_monster_stats(BaseTypeMonster::Willpower);
                (ret, stats)
            }
            MonsterType::Wizard => {
                let ret = directory_search("./static/images/monsters/Tex_wizard.png");
                let stats = calculate_monster_stats(BaseTypeMonster::Willpower);
                (ret, stats)
            }
            MonsterType::Zombie => {
                let ret = directory_search("./static/images/monsters/Tex_zombie.png");
                let stats = calculate_monster_stats(BaseTypeMonster::Strengh);
                (ret, stats)
            }
        };

        //TODO: impl skills + hide callback impl a construct for setting it
        Monster {
            id,
            pv: monster_pv,
            asset: assets.0[0].to_string(),
            base_armor: monster_armor as i32,
            stats: assets.1,
            min_damage: tuple_damage.0,
            max_damage: tuple_damage.1,
            resistances: monster_resistances,
            ailment: monster_ailment,
            on_death: DeathCallback::Monster,
        }
    }
}

pub fn generate_loot() -> Loot {
    let item_pool: ItemType = rand::random();

    match item_pool {
        ItemType::Weapon => Weapon::generate(),
        ItemType::Armor => Armor::generate(),
        ItemType::Consumable => Object::generate(),
        ItemType::Jewel => Jewel::generate(String::from("common")),
    }
}

pub fn generate_monster(monster_type: String) -> Monster {
    Monster::generate(monster_type)
}
