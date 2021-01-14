use crate::repository::models::{
    Caracter,
    NewCaracter, // Inventory, InventoryItemsGenerated, ItemGenerated, NewCaracter, NewPlayer, Player,
};
use crate::resources::models::CaracterStats;
// use crate::route::models::NewUserInput;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

pub fn create_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    info!(
        "Connecting to database using URL : `{}`",
        database_url.as_str()
    );

    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

// pub fn save_user(player: &NewUserInput) -> String {
//     use crate::repository::schema::players;
//     extern crate bcrypt;
//     use bcrypt::hash;
//     use rand::distributions::Alphanumeric;
//     use rand::{thread_rng, Rng};

//     let connection = create_connection();

//     let hashed = hash(player.password.as_str(), 4).unwrap();

//     let rand_string: String = thread_rng().sample_iter(&Alphanumeric).take(30).collect();

//     let is_mj = 0;

//     let new_player = NewPlayer {
//         id: rand_string,
//         pseudo: &player.username,
//         password: hashed,
//         is_mj,
//     };

//     info!("Saving new user {:?}", &new_player);

//     diesel::insert_into(players::table)
//         .values(&new_player)
//         .execute(&connection)
//         .expect("Error saving new player");

//     new_player.id
// }

pub fn save_player_stats(playerid: String, caracter_stats: CaracterStats) {
    use crate::repository::schema::caracter;
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};

    let connection = create_connection();

    let rand_string: String = thread_rng().sample_iter(&Alphanumeric).take(30).collect();

    let json_caracter_stats = serde_json::to_string(&caracter_stats).unwrap();

    let new_caracter = NewCaracter {
        id: rand_string,
        player_id: playerid,
        stats: json_caracter_stats,
        gold: 0,
    };

    info!("Saving player stats {:?}", &new_caracter);

    diesel::insert_into(caracter::table)
        .values(&new_caracter)
        .execute(&connection)
        .expect("Error saving new player");
}

pub fn get_player_stats(playerid: String) -> Option<Caracter> {
    use crate::repository::schema::caracter::dsl::*;

    let connection = create_connection();
    info!(
        "Fetching player stats for player id: `{}`",
        playerid.as_str()
    );

    caracter
        .filter(player_id.eq(playerid))
        .first::<Caracter>(&connection)
        .optional()
        .unwrap()
}

// pub fn get_items_from_inventory(playerid: String) -> Vec<ItemGenerated> {
//     use crate::repository::schema::inventory::dsl::*;
//     use crate::repository::schema::inventory_items_generated;
//     use crate::repository::schema::items_generated;

//     let connection = create_connection();

//     let results: Inventory = inventory
//         .filter(caracter_id.eq(playerid))
//         .first::<Inventory>(&connection)
//         .unwrap();

//     let res: Vec<(InventoryItemsGenerated, ItemGenerated)> =
//         InventoryItemsGenerated::belonging_to(&results)
//             .inner_join(
//                 items_generated::table
//                     .on(items_generated::id.eq(inventory_items_generated::items_generated_id)),
//             )
//             .load(&connection)
//             .unwrap();

//     res.into_iter().map(|(_i, it)| it).collect()
// }
