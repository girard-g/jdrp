#[derive(FromForm, Clone)]
pub struct UserInput<> {
    // The raw, undecoded value. You _probably_ want `String` instead.
    pub username: String,
    pub password: String,
}

#[derive(FromForm, Clone)]
pub struct NewUserInput<> {
    // The raw, undecoded value. You _probably_ want `String` instead.
    pub username: String,
    pub password: String,
    pub checkpassword: String,
}

#[derive(FromForm, Clone, Debug)]
pub struct CaracterStats<> {
    pub name: String,
    pub race: String,
    pub class: String,
    pub level: String,
    pub age: String,
    pub reputation: String,
    pub particularity: String,
    pub alignment: String,
    pub weapon: String,
    pub distance_weapon: String,
    pub bare_hand: String,
    pub armor: String,
    pub strengh: String,
    pub dexterity: String,
    pub endurance: String,
    pub charism: String,
    pub perception: String,
    pub luck: String,
    pub willpower: String,
    pub education: String,
}

// #[derive(FromForm, Clone, Debug)]
// pub struct CaracterStats<> {
//     pub name: String,    
//     pub race: String,
//     pub class: String,
//     pub level: u8,
//     pub age: u8,
//     pub reputation: String,
//     pub particularity: String,
//     pub alignment: String,
//     pub weapon: u8,
//     pub distance_weapon: u8,
//     pub bare_hand: u8,
//     pub armor: u8,
//     pub strengh: u8,
//     pub dexterity: u8,
//     pub endurance: u8,
//     pub charism: u8,
//     pub perception: u8,
//     pub luck: u8,
//     pub willpower: u8,
//     pub education: u8,
// }