use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, AsStaticStr, Clone, Copy, Deserialize, Serialize)]
pub enum WpnElement {
    None,
    Fire,
    Water, //A weapon can come with a status effect. Weapons can only have one status effect enabled.
    Earth, //The success rate of landing an effect is determined by the suffix in the item name.
    Light,
    Dark,
}

impl Distribution<WpnElement> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> WpnElement {
        match rng.gen_range(0, 6) {
            0 => WpnElement::None,
            1 => WpnElement::Fire,
            2 => WpnElement::Water,
            3 => WpnElement::Earth,
            4 => WpnElement::Light,
            5 => WpnElement::Dark,
            _ => WpnElement::None,
        }
    }
}
