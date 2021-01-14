use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Spell {
    pub name: LowLevelSpells,
    pub incante: Incante,
    pub damage: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize, Copy, Clone)]
pub enum Incante {
    Rituel,
    Concentration,
    Action,
    Reaction,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum LowLevelSpells {
    AnimationDesMorts,
    RayonEmpoisonne,
    RayonAffaiblissant,
    ToucherDuVampire,
    Amis,
    RepresailleInfernales,
    LameTonnante,
    LameAuxFlammesVertes,
    Gelure,
    ToileDAraignee,
    ServiteurInvisible,
    RespirationAquatique,
    VisionDansLeNoir,
    FlechesEnflammees,
    Deblocage,
    Bagou,
}

impl Distribution<LowLevelSpells> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> LowLevelSpells {
        match rng.gen_range(0, 16) {
            0 => LowLevelSpells::AnimationDesMorts,
            1 => LowLevelSpells::RayonEmpoisonne,
            2 => LowLevelSpells::RayonAffaiblissant,
            3 => LowLevelSpells::ToucherDuVampire,
            4 => LowLevelSpells::Amis,
            5 => LowLevelSpells::RepresailleInfernales,
            6 => LowLevelSpells::LameTonnante,
            7 => LowLevelSpells::LameAuxFlammesVertes,
            8 => LowLevelSpells::Gelure,
            9 => LowLevelSpells::ToileDAraignee,
            10 => LowLevelSpells::ServiteurInvisible,
            11 => LowLevelSpells::RespirationAquatique,
            12 => LowLevelSpells::VisionDansLeNoir,
            13 => LowLevelSpells::FlechesEnflammees,
            14 => LowLevelSpells::Deblocage,
            15 => LowLevelSpells::Bagou,
            _ => LowLevelSpells::AnimationDesMorts,
        }
    }
}

impl std::fmt::Display for LowLevelSpells {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            LowLevelSpells::AnimationDesMorts => write!(f, "Crée un squelette à partir d'os ou un zombie à partir d'un cadavre qui est sous le contrôle du lanceur."),
            LowLevelSpells::RayonEmpoisonne => write!(f, "Si l'attaque touche, inflige 1d8 dégâts de poison (dégâts/niv) et la cible peut être empoisonnée (JdS de Con)."),
            LowLevelSpells::RayonAffaiblissant => write!(f, "Si l'attaque avec un sort touche, la cible n'inflige que la moitié des dégâts avec une arme qui utilise la Force (JdS de Con)."),
            LowLevelSpells::ToucherDuVampire => write!(f, "Si l'attaque avec un sort touche, inflige des dégâts nécrotiques (dégâts/niv) et le lanceur récupère 50% de ces pv ."),
            LowLevelSpells::Amis => write!(f, "Le lanceur obtient l'avantage aux jets de Charisme contre une créature choisie qui ne lui est pas hostile."),
            LowLevelSpells::RepresailleInfernales => write!(f, "Piège magique qui se déclanche quand le lanceur est attaqué."),
            LowLevelSpells::LameTonnante => write!(f, "Si une attaque avec une arme touche, inflige 1d8 dégâts de tonnerre si la cible bouge (dégâts/niv)."),
            LowLevelSpells::LameAuxFlammesVertes => write!(f, "Si une attaque avec une arme touche, inflige aussi 2d10 dégâts feu à une autre créature (dégâts/niv)."),
            LowLevelSpells::Gelure => write!(f, "La cible doit réussir un JdS de Con. ou subir 1d6 dégâts de froid et avoir un désavantage à l'attaque (dégâts/niv)."),
            LowLevelSpells::ToileDAraignee => write!(f, "Crée un cube de 6 m de toiles d'araignées collantes qui peuvent entraver des créatures "),
            LowLevelSpells::ServiteurInvisible => write!(f, "Crée un serviteur invisible qui exécute des tâches simples (rapporter qq chose, nettoyer, entretenir un feu, servir, etc)."),
            LowLevelSpells::RespirationAquatique => write!(f, "Jusqu'à 10 créatures obtiennent la capacité de respirer sous l'eau."),
            LowLevelSpells::VisionDansLeNoir => write!(f, "La cible peut voir dans le noir à 18 mètres."),
            LowLevelSpells::FlechesEnflammees => write!(f, "12 flèches infligent 1d6 dégâts de feu extra"),
            LowLevelSpells::Deblocage => write!(f, "Déverrouille ou débloque 1 objet (porte, coffre, cadenas, menottes, etc)"),
            LowLevelSpells::Bagou => write!(f, "Donne 15 à un jet de Charisme et masque les mensonges"),
        }
    }
}

//TODO
#[derive(Debug, Deserialize, Serialize)]
pub enum HighLevelSpells {
    CageDesAmes,
    GlasFunebre,
    HurlementPsychique,
    CoffreSecretDeLeomund,
    PasLointain,
    ManoirSomptueuxDeMordenkainen,
    ChangementDeForme,
}

#[derive(Debug, Deserialize, Serialize, Copy, Clone)]
pub enum SpellDamage {
    None,
    D1d8,
    D1d6,
    D2d10,
    D3d6,
    D14d6,
}

impl std::fmt::Display for SpellDamage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            SpellDamage::D1d8 => write!(f, "1d8"),
            SpellDamage::D1d6 => write!(f, "1d6"),
            SpellDamage::D2d10 => write!(f, "2d10"),
            SpellDamage::D3d6 => write!(f, "3d6"),
            SpellDamage::D14d6 => write!(f, "14d6"),
            _ => write!(f, " empty "),
        }
    }
}
