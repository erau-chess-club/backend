use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;

use rocket::{
    form::{Form, FromForm},
    State,
};

use crate::model::Game;
use crate::{auth::UserSession, ApiError, ApiResponse, Db};

#[derive(FromForm)]
pub struct AddGameData {
    pub white_id: i32,
    pub black_id: i32,

    pub white_points: f32,
    pub black_points: f32,
    pub pgn: Option<String>,
    pub scorecard_image: Option<Vec<u8>>,

    pub game_end: i64,
    pub game_entered: Option<i64>,
}

#[post("/add_game", data = "<params>")]
pub async fn add(
    state: &State<Db>,
    params: Form<AddGameData>,
    session: UserSession,
) -> ApiResponse<UserSession> {
    let conn = state.connect();

    let end = NaiveDateTime::from_timestamp(params.game_end, 0);
    let entered_timestamp = match params.game_entered {
        Some(t) => t,
        None => Utc::now().timestamp(),
    };
    let entered = NaiveDateTime::from_timestamp(entered_timestamp, 0);

    let new_game = Game {
        white_id: params.white_id,
        black_id: params.black_id,
        white_points: params.white_points,
        black_points: params.black_points,
        pgn: params.pgn.clone(),
        scorecard_image: params.scorecard_image.clone(),
        game_end: end,
        game_entered: entered,
    };

    match diesel::insert_into(crate::schema::games::table)
        .values(new_game)
        .execute(&conn)
    {
        Ok(_) => ApiResponse(Ok(session)),
        Err(e) => {
            error!("Unknown Database Error during Registration {:?}", e);
            ApiResponse(Err(ApiError::Unknown("unknown database error".to_string())))
        }
    }
}
