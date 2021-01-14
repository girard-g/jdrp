use rocket::response::NamedFile;
use std::io;
// use rocket_contrib::json::Json;
// use rocket::request::Form;
// use crate::route::models::{NewUserInput, UserInput};
// use rocket::response::Redirect;
use std::path::PathBuf;
// use rocket::http::{Cookies, Cookie};
// use crate::resources::models::CaracterStats;
// use crate::item_generator::resources::Object;

#[get("/")]
pub fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

// #[get("/app")]
// pub fn app() -> io::Result<NamedFile> {
//     NamedFile::open("static/connected.html")
// }

#[get("/<files..>", rank = 10)]
pub fn dummy(files: PathBuf) -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

// #[get("/create-character/<uid>")]
// pub fn create_character(uid: String) -> io::Result<NamedFile> {
//     NamedFile::open("static/chat/caracter_creation.html")
// }

// #[get("/create")]
// pub fn create() -> io::Result<NamedFile> {
//     NamedFile::open("static/chat/user_creation.html")
// }

// #[post("/login", data = "<user_input>")]
// pub fn submit_task(user_input: Form<UserInput>, cookies: Cookies) -> Redirect  {
//     use crate::repository::mainlib::get_user_password;
//     extern crate bcrypt;
//     use bcrypt::{verify};

//     let player = get_user_password(user_input.username.to_string());

//     match player {

//         Some(player) => {
//             let decoded = verify(&user_input.password, &player.password.to_string());

//             match decoded {
//                 Ok(true) => {
//                     return loginng(player.id.to_string(), cookies);
//                 },
//                 Ok(false) => Redirect::to(uri!(index)),
//                 _ => Redirect::to(uri!(index))

//             }
//         },
//         None => {
//             Redirect::to(uri!(index))

//         }

//     }

// }

// #[post("/check-creation", data = "<user_input>")]
// pub fn check_creation(user_input: Form<NewUserInput>, mut cookies: Cookies) -> Redirect  {
//     use crate::repository::mainlib::save_user;

//     match user_input.password == user_input.checkpassword {
//         true => {
//             let id = save_user(&user_input);
//             let s2 = id.clone();
//             //TODO: Make a form for creating his caracter
//             cookies.add_private(Cookie::new("user_id", id));
//             Redirect::to(uri!(create_character: s2))

//             // return loginng(id, cookies);

//         }
//         _ => Redirect::to(uri!(index))
//     }

// }

// #[post("/check-caracter-creation/<uid>", format = "application/json", data = "<user_input>")]
// pub fn check_caracter_creation(uid: String, user_input: Json<CaracterStats>, mut cookies: Cookies) -> Json<String>  {
//     use crate::repository::mainlib::save_player_stats;
//     // use crate::resources::models::CaracterStats;

//     let logged_in_user = cookies.get_private("user_id");

//     match logged_in_user {
//         Some(user) => {
//             let logged_in_uid = user.value().parse::<String>().unwrap();
//             println!("{:#?}", logged_in_uid);
//             println!("{:#?}", uid);
//             if logged_in_uid == uid {

//                 // let decoded: CaracterStats = serde_json::from_str(&user_input);
//                 println!("Player Saved {:#?}", user_input);
//                 save_player_stats(uid, user_input.into_inner());
//                 Json(String::from("TRUE"))

//             } else {
//                 Json(String::from("FALSE"))
//             }
//         },
//         None =>  Json(String::from("FALSE"))
//     }

// }

// fn loginng(id: String, mut cookies: Cookies) -> Redirect
// {
//     let s2 = id.clone();
//     cookies.add_private(Cookie::new("user_id", id));
//     Redirect::to(uri!(player_dashboard: s2))
// }

// //TODO: Redirect instead of serving a file. Only if the user is none authenticated
// #[get("/player/<uid>")]
// pub fn player_dashboard(uid: String, mut cookies: Cookies) -> io::Result<NamedFile> {

//     let logged_in_user = cookies.get_private("user_id");

//     match logged_in_user {
//         Some(user) => {
//             let logged_in_uid = user.value().parse::<String>().unwrap();
//             if logged_in_uid == uid {
//                 // Json(fetch_user_by_id(&db, uid))
//                 NamedFile::open("static/chat/player_dashboard.html")
//             } else {
//                 NamedFile::open("static/chat/login.html")
//             }
//         },
//         None =>  NamedFile::open("static/chat/login.html")
//     }
// }

// #[get("/users/player-stats/<uid>")]
// pub fn player_stats(uid: String, mut cookies: Cookies) -> String  {
//     use crate::repository::mainlib::get_player_stats;

//     let logged_in_user = cookies.get_private("user_id");

//     match logged_in_user {
//         Some(user) => {
//             let logged_in_uid = user.value().parse::<String>().unwrap();
//             if logged_in_uid == uid {

//                 let player_stats = get_player_stats(uid);
//                 match player_stats {
//                     Some(e) => {
//                         //FIXME: double json encoded value
//                         // println!("e.stats is {}", e.stats);
//                         // let toto:String = serde_json::from_str(&e.stats).unwrap();
//                         // Json(toto)
//                         e.stats
//                     }
//                     //None => Json(String::from("FALSE"))
//                     None => String::from("FALSE")
//                 }
//             } else {
//                 // Json(String::from("FALSE"))
//                 String::from("FALSE")
//             }
//         },
//         // None =>  Json(String::from("FALSE"))
//         None =>  String::from("FALSE")
//     }
// }

// #[get("/users/logout")]
// pub fn logout(mut cookies: Cookies) -> () {
//     cookies.remove_private(Cookie::named("user_id"));
// }

// #[get("/testobjectgeneration")]
// pub fn testobjectgeneration() -> io::Result<NamedFile> {
//     NamedFile::open("static/chat/item.html")
// }

// #[get("/api/testobjectgenerationlol")]
// pub fn testobjectgenerationlol() -> Json<Object> {
//     use crate::item_generator::generator::generate_loot;
//     let loot = generate_loot();
//     Json(loot)
// }
