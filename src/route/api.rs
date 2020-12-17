use rocket_contrib::json::Json;
use crate::item_generator::resources::{Loot};
use crate::item_generator::monster::{Monster};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    preferred_username: String,
    email: String,
}

#[get("/api/testobjectgenerationlol")]
pub fn testobjectgenerationlol() -> Json<Loot> {
    use crate::item_generator::generator::generate_loot;
    let loot = generate_loot();
    Json(loot)
}

#[get("/api/testmonstergeneration/<monster>")]
pub fn testmonstergeneration(monster: String) -> Json<Monster> {
    use crate::item_generator::generator::generate_monster;
    let loot = generate_monster(monster);
    Json(loot)
}

#[post("/api/getcharacter", data = "<token>")]
pub fn get_player (token: String) -> Json<String> {
    use crate::repository::mainlib::get_player_stats;

    //TODO: check unwrap here
    let f = fs::read_to_string("./certs.json")
    .expect("Something went wrong reading the file");
    let json: serde_json::Value = serde_json::from_str(f.as_ref()).expect("JSON was not well-formatted");
    let token = decode::<Claims>(&token, &DecodingKey::from_rsa_components(json["keys"][0]["n"].as_str().unwrap(), json["keys"][0]["e"].as_str().unwrap()), &Validation::new(Algorithm::RS256)).unwrap();

    let pstats = get_player_stats(token.claims.email);


    match pstats {
        Some(e) => {
            //FIXME: double json encoded value 
            // println!("e.stats is {}", e.stats);
            // let toto:String = serde_json::from_str(&e.stats).unwrap();
            // Json(toto)
            Json(e.stats)
        }
        None => Json(String::from("FALSE"))
        // None => String::from("FALSE")
    }

    // Json(token.claims.email)
}