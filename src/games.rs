use chrono::NaiveDateTime;
use diesel::prelude::*;

use rocket::{
    form::{Form, FromForm},
    State,
};

use crate::{
    auth::{OfficerUserSession, UserSession},
    model::{Game, NewGame},
};
use crate::{ApiError, ApiResponse, Db};

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

#[post("/game/add", data = "<params>")]
pub async fn add(
    state: &State<Db>,
    params: Form<AddGameData>,
    session: OfficerUserSession,
) -> ApiResponse<()> {
    let conn = state.connect();

    let end = NaiveDateTime::from_timestamp(params.game_end, 0);

    let new_game = NewGame {
        white_id: params.white_id,
        black_id: params.black_id,
        white_points: params.white_points,
        black_points: params.black_points,
        pgn: params.pgn.clone(),
        scorecard_image: params.scorecard_image.clone(),
        game_end: end,
        added_by: session.0.user,
    };

    match diesel::insert_into(crate::schema::games::table)
        .values(new_game)
        .execute(&conn)
    {
        Ok(_) => ApiResponse(Ok(())),
        Err(e) => {
            error!("Unknown Database Error during Registration {:?}", e);
            ApiResponse(Err(ApiError::Unknown("unknown database error".to_string())))
        }
    }
}

#[get("/game/list")]
pub fn list(state: &State<Db>) -> ApiResponse<UserSession> {
    let conn = state.connect();
    let games: Result<Vec<Game>, _> = crate::schema::games::table.load(&conn);
    info!("Got games: {:?}", games);

    ApiResponse(Err(ApiError::Unknown("unknown database error".to_string())))
}
