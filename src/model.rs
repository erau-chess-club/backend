use crate::schema::{games, users};

use chrono::NaiveDateTime;

#[derive(Queryable, Identifiable, Associations, Debug)]
#[table_name = "users"]
pub struct User {
    #[primary_key]
    pub id: i32,

    pub first_name: String,
    pub last_name: String,
    pub hash: String,

    pub erau_id: Option<i32>,
    pub signup_date: NaiveDateTime,
    pub is_officer: bool,
    pub chess_com_username: String,
    pub email: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a, 'b> {
    pub first_name: &'a str,
    pub last_name: &'b str,
    pub hash: String,
    pub erau_id: Option<i32>,
    pub chess_com_username: String,
    pub email: String,
    pub signup_date: NaiveDateTime,
}

#[derive(Queryable, Debug)]
pub struct Game {
    pub id: i32,
    pub white_id: i32,
    pub black_id: i32,

    pub white_points: f32,
    pub black_points: f32,
    pub pgn: Option<String>,
    pub scorecard_image: Option<Vec<u8>>,

    pub game_end: NaiveDateTime,
    pub added_on: NaiveDateTime,
    pub added_by: i32,
}

#[derive(Insertable, Debug)]
#[table_name = "games"]
pub struct NewGame {
    pub white_id: i32,
    pub black_id: i32,

    pub white_points: f32,
    pub black_points: f32,
    pub pgn: Option<String>,
    pub scorecard_image: Option<Vec<u8>>,

    pub game_end: NaiveDateTime,
    pub added_by: i32,
}
