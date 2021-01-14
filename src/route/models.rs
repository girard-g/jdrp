use serde::{Deserialize, Serialize};

#[derive(FromForm, Clone)]
pub struct UserInput {
    // The raw, undecoded value. You _probably_ want `String` instead.
    pub username: String,
    pub password: String,
}

#[derive(FromForm, Clone)]
pub struct NewUserInput {
    pub username: String,
    pub password: String,
    pub checkpassword: String,
}

// #[derive(Serialize, Deserialize)]
// pub struct CaracterStats<> {
//     pub stats: String,
// }

#[derive(FromForm, Clone, Serialize, Deserialize, Debug)]
pub struct CaracterStats {
    pub stats: String,
}
