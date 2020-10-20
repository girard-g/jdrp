#![feature(
    proc_macro_hygiene,
    decl_macro,
    register_tool,
    register_attr,
    rustc_private,
    type_ascription
)]

#[macro_use]
extern crate ws;

extern crate dotenv;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
extern crate serde_json;

extern crate syslog;
#[macro_use]
extern crate log;

// use std::thread;

mod repository;
mod resources;
// use crate::repository::mainlib::{create_connection, get_five_last_posts};

mod logger;
mod route;

use crate::route::{get, static_files};

mod chat;

// use crate::chat::ws_rs;
extern crate chrono;

fn rocket() -> rocket::Rocket {
    let rocket_routes = routes![
        static_files::file,
        get::index,
        get::submit_task,
        get::logout,
        get::player_dashboard,
        get::create,
        get::check_creation,
        get::player_stats,
        get::create_character,
        get::check_caracter_creation,
    ];

    rocket::ignite().mount("/", rocket_routes)
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
