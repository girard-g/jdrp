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