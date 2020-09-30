use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use super::schema::posts;
use super::schema::players;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub author: String,
    pub body: String,
    pub published_at: NaiveDateTime,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Player {
    pub id: String,
    pub pseudo: String,
    pub password: String,
    pub is_mj: i32,
}

#[derive(Insertable)]
#[table_name="players"]
pub struct NewPlayer<'a> {
    pub id: &'a str,
    pub pseudo: &'a str,
    pub password:  &'a str,
    pub is_mj: i32,
}

#[derive(Insertable)]
#[table_name="posts"]
pub struct NewPost<'a> {
    pub author: &'a str,
    pub body: &'a str,
    pub published_at: NaiveDateTime,
}