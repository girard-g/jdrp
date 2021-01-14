use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, AsStaticStr, Deserialize, Serialize)]
pub enum WpnAilment {
    None,
    Poison,
    Stun, //A weapon can come with a status effect. Weapons can only have one status effect enabled.
    Freeze, //The success rate of landing an effect is determined by the suffix in the item name.
    Death,
    Sleep,
}

impl PartialEq for WpnAilment {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

impl std::fmt::Display for WpnAilment {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            WpnAilment::None => write!(f, " empty "),
            WpnAilment::Poison => write!(f, " poison "),
            WpnAilment::Stun => write!(f, " stun "),
            WpnAilment::Freeze => write!(f, " freeze "),
            WpnAilment::Death => write!(f, " death "),
            WpnAilment::Sleep => write!(f, " sleep "),
        }
    }
}

impl Distribution<WpnAilment> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> WpnAilment {
        match rng.gen_range(0, 6) {
            0 => WpnAilment::None,
            1 => WpnAilment::Poison,
            2 => WpnAilment::Stun,
            3 => WpnAilment::Freeze,
            4 => WpnAilment::Death,
            5 => WpnAilment::Sleep,
            _ => WpnAilment::None,
        }
    }
}

#[derive(Debug, AsStaticStr)]
pub enum PoisonAliment {
    Bacteria = 10,
    Contaminated = 20,
    Tainted = 30,
    Toxic = 40,
    Infected = 50,
    Viral = 60,
    Venomous = 70,
    Virulent = 80,
    Noxious = 90,
    Biohazardous = 100,
}

impl Distribution<PoisonAliment> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PoisonAliment {
        match rng.gen_range(0, 9) {
            0 => PoisonAliment::Bacteria,
            1 => PoisonAliment::Contaminated,
            2 => PoisonAliment::Tainted,
            3 => PoisonAliment::Toxic,
            4 => PoisonAliment::Infected,
            5 => PoisonAliment::Viral,
            6 => PoisonAliment::Venomous,
            7 => PoisonAliment::Virulent,
            8 => PoisonAliment::Noxious,
            9 => PoisonAliment::Biohazardous,
            _ => PoisonAliment::Bacteria,
        }
    }
}

#[derive(Debug, AsStaticStr)]
pub enum StunAliment {
    Tingling = 10,
    Numbing = 20,
    Stiffening = 30,
    Immobilizing = 40,
    Disabling = 50,
    Debilitating = 60,
    Paralyzing = 70,
    Disarming = 80,
    Arresting = 90,
    Enfeebling = 100,
}

impl Distribution<StunAliment> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> StunAliment {
        match rng.gen_range(0, 9) {
            0 => StunAliment::Tingling,
            1 => StunAliment::Numbing,
            2 => StunAliment::Stiffening,
            3 => StunAliment::Immobilizing,
            4 => StunAliment::Disabling,
            5 => StunAliment::Debilitating,
            6 => StunAliment::Paralyzing,
            7 => StunAliment::Disarming,
            8 => StunAliment::Arresting,
            9 => StunAliment::Enfeebling,
            _ => StunAliment::Tingling,
        }
    }
}
