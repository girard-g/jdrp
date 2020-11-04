use serde::{Deserialize, Serialize};

#[derive(Debug, AsStaticStr, Clone, Copy, Deserialize, Serialize)]
pub enum WpnElement {
    None,
    Fire,
    Water,               //A weapon can come with a status effect. Weapons can only have one status effect enabled.
    Earth,             //The success rate of landing an effect is determined by the suffix in the item name.
    Light,
    Dark
}