use rocket_contrib::json::Json;
use crate::item_generator::resources::{Loot};
use crate::item_generator::monster::{Monster};
use jsonwebtoken::{decode, TokenData, Algorithm, Validation, DecodingKey};
use serde::{Serialize, Deserialize};
use std::fs;
use rocket::request::{FromRequest, Request, Outcome};
use rocket::response::Response;
use rocket::http::Status;
use crate::resources::models::CaracterStats;
use std::path::PathBuf;



#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    preferred_username: String,
    email: String,
}

#[derive(Debug)]
pub enum ApiTokenError {
    Missing,
    Invalid,
}

pub struct Token(Claims);

impl<'a, 'r> FromRequest<'a, 'r> for Token {
    type Error = ApiTokenError;

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let token = request.headers().get_one("Authorization");

        match token {
            Some(t) => {

                let f = fs::read_to_string("./certs.json")
                .expect("Something went wrong reading the file");
                let json: serde_json::Value = serde_json::from_str(f.as_ref()).expect("JSON was not well-formatted");
                let token = decode::<Claims>(&t, &DecodingKey::from_rsa_components(json["keys"][0]["n"].as_str().unwrap(), json["keys"][0]["e"].as_str().unwrap()), &Validation::new(Algorithm::RS256)).unwrap();
                
                Outcome::Success(Token(token.claims))
            }
            
            None =>{
                Outcome::Failure((Status::Unauthorized, ApiTokenError::Missing))
            } 
        }

    }

} 

#[options("/<files..>")]
pub fn send_options<'a>(files: PathBuf) -> Response<'a> {
    let mut res = Response::new();
    res.set_status(Status::new(200, "No Content"));
    res.adjoin_raw_header("Content-Type", "text/plain");
    res.adjoin_raw_header("Access-Control-Allow-Methods", "POST, GET, OPTIONS");
    res.adjoin_raw_header("Access-Control-Allow-Headers", "Content-Type, Authorization");
    res
}


#[post("/api/getcharacter")]
pub fn get_player (token: Token) -> Json<String> {
    use crate::repository::mainlib::get_player_stats; 

    let pstats = get_player_stats(token.0.email);

    match pstats {
        Some(e) => {
            Json(e.stats)
        }
        None => Json(String::from("FALSE"))
    }

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

#[post("/api/check-caracter-creation", format = "json", data = "<u>")]
pub fn check_caracter_creation(_token: Token, u: Json<CaracterStats>) -> Json<bool>  {
    // use crate::repository::mainlib::save_player_stats;

    let tmp = vec![u.strengh, u.dexterity, u.endurance, u.charism, u.perception, u.luck, u.willpower, u.education];
    let w: Vec<&u8> = tmp.iter().filter(|s| s > &&70 || s < &&10).collect();
    let sum: u16 = tmp.iter().map(|&a| a as u16).sum();

    if w.is_empty() == false || sum > 300 {
        Json(false)
    }else {
        println!("vec: {:#?}", sum);
        println!("Player Saved {:#?}", u);
        // save_player_stats(uid, user_input.into_inner());
        Json(true)
    }

}