use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;
use crate::repository::models::{Player, NewPlayer, Caracter};
use crate::route::models::NewUserInput;


pub fn create_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn get_user_password(username :String) -> Option<Player> {
    use crate::repository::schema::players::dsl::*;

    let connection = create_connection();

    let results = players.filter(pseudo.eq(username))
        .first::<Player>(&connection)
        .optional()
        .unwrap();

        return results;
}

pub fn save_user(player :&NewUserInput)-> String {
    use crate::repository::schema::players;
    extern crate bcrypt;
    use rand::{thread_rng, Rng};
    use rand::distributions::Alphanumeric;
    use bcrypt::{hash};

    let connection = create_connection();

    let hashed = hash(player.password.as_str(), 4).unwrap();

    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .collect();

    let new_player = NewPlayer{
        id: rand_string,
        pseudo: &player.username,
        password: hashed,
        is_mj:0
    };

    diesel::insert_into(players::table)
        .values(&new_player)
        .execute(&connection)
        .expect("Error saving new player");

        new_player.id

}

pub fn get_player_stats(playerid :String) ->  Option<Caracter> {
    use crate::repository::schema::caracter::dsl::*;

    let connection = create_connection();

    let results = caracter.filter(player_id.eq(playerid))
    .first::<Caracter>(&connection)
    .optional()
    .unwrap();

    return results;

}