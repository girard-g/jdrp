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
use crate::configuration::settings::{Settings};

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

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    code: usize,
    message: String
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
pub fn get_player(token: Token) -> Json<String> {
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


#[get("/api/config")]
pub fn configfile() -> Result<Json<Settings>, Json<Error>> {

    let s = Settings::new();
    match s {
        Ok(config) => {
            Ok(Json(config))
        }
        Err(e) => {
            println!("{:#?}", e);
            Err(Json(Error{code:60421676, message:e.to_string()}))
        }

    }
}


#[post("/api/check-caracter-creation", format = "json", data = "<u>")]
pub fn check_caracter_creation(token: Token, mut u: Json<CaracterStats>) -> Result<Json<bool>, Json<Error>>   {
    use crate::repository::mainlib::save_player_stats;

    let s = Settings::new();

    match s {
        Ok(config) => {

            let tmp = vec![u.strengh, u.dexterity, u.endurance, u.charism, u.perception, u.luck, u.willpower, u.education];
            let w: Vec<&u8> = tmp.iter().filter(|s| s > &&config.game_stats.max_per_cat || s < &&config.game_stats.min_per_cat).collect();
        
            if w.is_empty() == false {
                Err(Json(Error{code:16873154, message:String::from("Stats have been altered")}))
            }else {

                let stats_from_race = config.race_stats.get_stats(u.race.as_mut());
                let stats_from_class = config.class_stats.get_stats(u.class.as_mut());

                let mut check: Vec<i8> = Vec::new();

                println!("u.strengh: {:#?}", u.strengh);
                println!("stats_from_race.strengh: {:#?}", stats_from_race.strengh);
                println!("stats_from_class.strengh: {:#?}", stats_from_class.strengh);

                check.push(u.strengh as i8 - (stats_from_race.strengh + stats_from_class.strengh));
                check.push(u.dexterity as i8 - (stats_from_race.dexterity + stats_from_class.dexterity));
                check.push(u.luck as i8 - (stats_from_race.luck + stats_from_class.luck));
                check.push(u.willpower as i8 - (stats_from_race.willpower + stats_from_race.willpower));
                check.push(u.endurance as i8 - (stats_from_race.endurance + stats_from_class.endurance));
                check.push(u.charism as i8 - (stats_from_race.charism + stats_from_class.charism));
                check.push(u.perception as i8 - (stats_from_race.perception + stats_from_class.perception));
                check.push(u.education as i8 - (stats_from_race.education + stats_from_class.education));
                
                let sum: u16 = check.iter().map(|&a| a as u16).sum();

                println!("sum: {:#?}", sum);
                println!("max stats: {:#?}", config.game_stats.max_stat);

                if sum > config.game_stats.max_stat {
                    Err(Json(Error{code:16873154, message:String::from("Stats have been altered")}))
                }else{
                    println!("vec: {:#?}", sum);
                    println!("Player Saved {:#?}", u);
                    save_player_stats(token.0.email, u.into_inner());
                    Ok(Json(true))
                }
            }
        }
        Err(e) => {
            println!("{:#?}", e);
            Err(Json(Error{code:60421676, message:e.to_string()}))
            // Err(Json(Error{code:60421676, message:String::from("Can parse config file")}))

        }
    }


}