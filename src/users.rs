use crate::schema::users;
use crate::{auth::UserSession, model::User, ApiError, ApiResponse, Db};

use diesel::prelude::*;
use rocket::State;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct PublicUser {
    first_name: String,
    last_name: String,
    id: i32,
}

impl From<User> for PublicUser {
    fn from(user: User) -> Self {
        PublicUser {
            first_name: user.first_name,
            last_name: user.last_name,
            id: user.id,
        }
    }
}

#[get("/users/list")]
pub fn list(state: &State<Db>, session: UserSession) -> ApiResponse<Vec<PublicUser>> {
    info!("user: {}", session.user);
    let conn = state.connect();
    let result: Result<Vec<User>, _> = users::table.load(&conn);
    let response = match result {
        Ok(users) => {
            info!("Users: {:?}", users);
            Ok(users.into_iter().map(|u| u.into()).collect())
        }
        Err(err) => Err(ApiError::DatabaseError(err.to_string())),
    };

    ApiResponse(response)
}
