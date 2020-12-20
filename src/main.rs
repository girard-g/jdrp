#![feature(
    proc_macro_hygiene,
    decl_macro,
    register_tool,
    register_attr,
    rustc_private,
    type_ascription
)]

// #[macro_use]
// extern crate ws;

extern crate dotenv;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
extern crate serde_json;

extern crate syslog;
#[macro_use]
extern crate log;
extern crate strum; // 0.10.0
#[macro_use]
extern crate strum_macros; // 0.10.0

// use std::thread;

mod repository;
mod resources;
mod item_generator;
mod logger;
mod route;

use crate::route::{get, api, static_files};
// use rocket_contrib::serve::StaticFiles;

mod chat;

// use crate::chat::ws_rs;
extern crate chrono;

use rocket::fairing::AdHoc;
use rocket::http::hyper::header::{AccessControlAllowOrigin};
// use rocket_cors::{AllowedHeaders, AllowedOrigins};
// use rocket::http::Method;

// use rocket::{Request, Response};
// use rocket::fairing::{Fairing, Info, Kind};
// use rocket::http::Header;

fn rocket() -> rocket::Rocket {
    let rocket_routes = routes![
        static_files::file,
        static_files::images,
        get::index,
        api::testobjectgenerationlol,
        api::testmonstergeneration,
        api::check_caracter_creation,
        api::get_player,
        api::send_options,
        get::dummy,
        // get::app,
        // get::submit_task,
        // get::logout,
        // get::player_dashboard,
        // get::create,
        // get::check_creation,
        // get::player_stats,
        // get::create_character,
        // get::check_caracter_creation,
        // get::testobjectgeneration,
        

    ];

    // pub struct CORS();

    // impl Fairing for CORS {
    //     fn info(&self) -> Info {
    //         Info {
    //             name: "Add CORS headers to requests",
    //             kind: Kind::Response
    //         }
    //     }

    //     fn on_response(&self, request: &Request, response: &mut Response) {
    //         response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
    //         response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
    //         response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
    //         response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    //     }
    // }
    
    // let cors = rocket_cors::CorsOptions {
    //     allowed_origins : AllowedOrigins::all(),
    //     allowed_methods: vec![Method::Get, Method::Post, Method::Options].into_iter().map(From::from).collect(),
    //     allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
    //     allow_credentials: true,
    //     fairing_route_base: String::from("/api"),
    //     ..Default::default()
    // };

    rocket::ignite()
    .mount("/", rocket_routes)
    // .attach(cors.to_cors().unwrap())
    // .attach(CORS())
    .attach(AdHoc::on_response("Add Header", |_, resp| {
        resp.adjoin_header(AccessControlAllowOrigin::Any);
        resp.adjoin_raw_header("Access-Control-Allow-Credentials", "true");

        // resp.adjoin_header(AccessControlAllowOrigin::Value(String::from("http://localhost:3000")));
    }))
    // .mount("/images", StaticFiles::from("/home/guillaume/Projects/Jdrp/static/images"))
}

fn main() {
    // thread::Builder::new()
    //     .name("Thread for chat".into())
    //     .spawn(|| {
    //         ws_rs::websocket();
    //     })
    //     .unwrap();

    logger::syslog::init().expect("Could not init logger. Ensure that your syslog process is running.");

    info!("Logger streaming through syslog");

    rocket().launch();
}
