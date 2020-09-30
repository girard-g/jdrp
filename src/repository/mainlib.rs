use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;
use crate::repository::models::{Player, NewPlayer};
use crate::route::models::NewUserInput;
// use hex_literal::hex;


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

pub fn save_user(player :&NewUserInput)-> usize {
    use crate::repository::schema::players;
    extern crate bcrypt;
    use rand::{thread_rng, Rng};
    use rand::distributions::Alphanumeric;
    use bcrypt::{hash};

    let connection = create_connection();

    let hashed = hash(player.password.as_str(), 4);

    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .collect();

    match hashed {
        Ok(e) => {
            let new_player = NewPlayer{
                id: rand_string.as_str(),
                pseudo: player.username.as_str(),
                password: e.as_str(),
                is_mj:0
            };
        
            diesel::insert_into(players::table)
                .values(&new_player)
                .execute(&connection)
                .expect("Error saving new player")

        },
        Err(error) => {
            println!("error creating hashed password {:?}", error);
            return 0;
        }

    }

    
}
