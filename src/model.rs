use crate::schema::users;

use chrono::NaiveDateTime;

#[derive(Queryable)]
pub struct User {
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
#[table_name="users"]
pub struct NewUser<'a, 'b> {
    pub first_name: &'a str,
    pub last_name: &'b str,
    pub hash: String,
    pub erau_id: Option<i32>,
    pub chess_com_username: String,
    pub email: String,
    pub signup_date: NaiveDateTime,
}
