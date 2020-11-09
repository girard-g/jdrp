use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;
use crate::repository::models::{Player, NewPlayer, Caracter, NewCaracter};
use crate::route::models::NewUserInput;
use crate::resources::models::{CaracterStats};

pub fn create_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    info!("Connecting to database using URL : `{}`", database_url.as_str());

    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn get_user_password(username :String) -> Option<Player> {
    use crate::repository::schema::players::dsl::*;

    let connection = create_connection();
    info!("Fetching user (`{}`) password", username.as_str());

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

    let is_mj = 0;

    let new_player = NewPlayer{
        id: rand_string,
        pseudo: &player.username,
        password: hashed,
        is_mj
    };

    info!("Saving new user {:?}", &new_player);

    diesel::insert_into(players::table)
        .values(&new_player)
        .execute(&connection)
        .expect("Error saving new player");

        new_player.id

}

pub fn save_player_stats(playerid :String, caracter_stats: CaracterStats) -> () 
{
    use crate::repository::schema::caracter;
    use rand::{thread_rng, Rng};
    use rand::distributions::Alphanumeric;

    let connection = create_connection();


    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .collect();

        let json_caracter_stats = serde_json::to_string(&caracter_stats).unwrap();

    let new_caracter = NewCaracter{
        id: rand_string,
        player_id: playerid,
        stats: json_caracter_stats
    };

    info!("Saving player stats {:?}", &new_caracter);

    diesel::insert_into(caracter::table)
        .values(&new_caracter)
        .execute(&connection)
        .expect("Error saving new player");


}

pub fn get_player_stats(playerid :String) ->  Option<Caracter> {
    use crate::repository::schema::caracter::dsl::*;

    let connection = create_connection();
    info!("Fetching player stats for player id: `{}`", playerid.as_str());

    let results = caracter.filter(player_id.eq(playerid))
    .first::<Caracter>(&connection)
    .optional()
    .unwrap();

    return results;

}