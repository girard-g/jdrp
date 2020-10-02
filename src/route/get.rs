use std::io;
use rocket::response::NamedFile;
use rocket::response::content::Json;
use rocket::request::Form;
use crate::route::models::{NewUserInput, UserInput, CaracterStats};
use rocket::response::Redirect;
use rocket::http::{Cookies, Cookie};

#[get("/")]
pub fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/chat/login.html")
}

#[get("/test")]
pub fn test() -> io::Result<NamedFile> {
    NamedFile::open("static/chat/caracter_creation.html")
}

#[get("/create")]
pub fn create() -> io::Result<NamedFile> {
    NamedFile::open("static/chat/user_creation.html")
}

#[post("/login", data = "<user_input>")]
pub fn submit_task(user_input: Form<UserInput>, cookies: Cookies) -> Redirect  {
    use crate::repository::mainlib::get_user_password;
    extern crate bcrypt;
    use bcrypt::{verify};

    let player = get_user_password(user_input.username.to_string());

    match player {

        Some(player) => {
            let decoded = verify(&user_input.password, &player.password.to_string());

            match decoded {
                Ok(true) => { 
                    return loginng(player.id.to_string(), cookies);
                },
                Ok(false) => Redirect::to(uri!(index)),
                _ => Redirect::to(uri!(index))
        
            }
        },
        None => {
            Redirect::to(uri!(index))

        }
        
    }

}

#[post("/check-creation", data = "<user_input>")]
pub fn check_creation(user_input: Form<NewUserInput>, mut cookies: Cookies) -> Redirect  {
    use crate::repository::mainlib::save_user;

    match user_input.password == user_input.checkpassword {
        true => {
            let id = save_user(&user_input);

            //TODO: Make a form for creating his caracter
            cookies.add_private(Cookie::new("user_id", id));
            Redirect::to(uri!(test))

            // return loginng(id, cookies);

        }
        _ => Redirect::to(uri!(create))
    }

}


#[post("/check-caracter-creation/<uid>", data = "<user_input>")]
pub fn check_caracter_creation(uid: String, user_input: Form<CaracterStats>, mut cookies: Cookies) -> Redirect  {
    use crate::repository::mainlib::save_player_stats;
    use crate::resources::models::CaracterStats;

    let logged_in_user = cookies.get_private("user_id");

    match logged_in_user {
        Some(user) => {
            let logged_in_uid = user.value().parse::<String>().unwrap();
            if logged_in_uid == uid {
               
                println!("{:#?}", user_input);
                Redirect::to(uri!(player_dashboard: uid))

            } else {
                Redirect::to(uri!(index))
            } 
        },
        None =>  Redirect::to(uri!(index))
    }




}


fn loginng(id: String, mut cookies: Cookies) -> Redirect
{
    let s2 = id.clone();
    cookies.add_private(Cookie::new("user_id", id));
    Redirect::to(uri!(player_dashboard: s2))
}


//TODO: Redirect instead of serving a file. Only if the user is none authenticated
#[get("/player/<uid>")]
pub fn player_dashboard(uid: String, mut cookies: Cookies) -> io::Result<NamedFile> {

    let logged_in_user = cookies.get_private("user_id");

    match logged_in_user {
        Some(user) => {
            let logged_in_uid = user.value().parse::<String>().unwrap();
            if logged_in_uid == uid {
                // Json(fetch_user_by_id(&db, uid))
                NamedFile::open("static/chat/player_dashboard.html")
            } else {
                NamedFile::open("static/chat/login.html")
            }
        },
        None =>  NamedFile::open("static/chat/login.html")
    }
}


#[post("/users/player-stats/<uid>")]
pub fn player_stats(uid: String, mut cookies: Cookies) -> Json<String>  {
    use crate::repository::mainlib::get_player_stats;

    let logged_in_user = cookies.get_private("user_id");

    match logged_in_user {
        Some(user) => {
            let logged_in_uid = user.value().parse::<String>().unwrap();
            if logged_in_uid == uid {

                let player_stats = get_player_stats(uid);
                match player_stats {
                    Some(e) => {
                        //FIXME: double json encoded value 
                        Json(e.stats)
                    }
                    None => Json(String::from("FALSE"))
                }
            } else {
                Json(String::from("FALSE"))
            }
        },
        None =>  Json(String::from("FALSE"))
    }
}

#[get("/users/logout")]
pub fn logout(mut cookies: Cookies) -> () {
    cookies.remove_private(Cookie::named("user_id"));
}

// #[get("/chat")]
// pub fn chat() -> io::Result<NamedFile> {
//     NamedFile::open("static/chat/index.html")
// }

// #[get("/posts")]
// pub fn posts() -> Json<String> {
//     let posts = get_five_last_posts();
//     return Json(serde_json::to_string(&posts).unwrap());
// }
