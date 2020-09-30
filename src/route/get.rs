use std::io;
use rocket::response::NamedFile;
// use rocket::response::content::Json;
// use rocket::response::content;
use rocket::request::Form;
use crate::route::models::{NewUserInput, UserInput};
use rocket::response::Redirect;
use rocket::http::{Cookies, Cookie};


#[get("/")]
pub fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/chat/login.html")
}

#[get("/create")]
pub fn create() -> io::Result<NamedFile> {
    NamedFile::open("static/chat/user_creation.html")
}

#[post("/login", data = "<user_input>")]
pub fn submit_task(user_input: Form<UserInput>, mut cookies: Cookies) -> Redirect  {
    use crate::repository::mainlib::get_user_password;
    extern crate bcrypt;
    use bcrypt::{verify};

    let player = get_user_password(user_input.username.to_string());

    match player {

        Some(player) =>{
            let decoded = verify(&user_input.password, &player.password.to_string());

            match decoded {
                Ok(true) => { 
                    cookies.add_private(Cookie::new("user_id", player.id.to_string()));
                    Redirect::to(uri!(player_dashboard: player.id.to_string()))
        
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

    

    save_user(&user_input);

    // match player {

    //     Some(player) =>{
    //         let decoded = verify(&user_input.password, &player.password.to_string());

    //         match decoded {
    //             Ok(true) => { 
    //                 cookies.add_private(Cookie::new("user_id", player.id.to_string()));
    //                 Redirect::to(uri!(player_dashboard: player.id.to_string()))
        
    //             },
    //             Ok(false) => Redirect::to(uri!(index)),
    //             _ => Redirect::to(uri!(index))
        
    //         }
    //     },
    //     None => {
            Redirect::to(uri!(index))

    //     }
        
    // }

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
    

#[post("/users/logout", format="json")]
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
