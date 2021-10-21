#[macro_use]
extern crate rocket;
use rocket::{
    fs::FileServer,
    http::{ContentType, Status},
    response::Responder,
    Request, Response,
};

#[macro_use]
extern crate diesel;

mod auth;
mod games;
mod model;
mod schema;

pub use auth::UserSession;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use serde::Serialize;

use dotenv::dotenv;
use std::env;

pub struct Db {
    sqlite_path: String,
}

impl Db {
    pub fn new(sqlite_path: String) -> Self {
        Db { sqlite_path }
    }
    pub fn connect(&self) -> SqliteConnection {
        SqliteConnection::establish(&self.sqlite_path)
            .unwrap_or_else(|_| panic!("Error connecting to {}", self.sqlite_path))
    }
}

#[launch]
fn rocket() -> _ {
    env_logger::init();

    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db = Db::new(database_url);

    #[cfg(feature = "dist")]
    const STATIC_PATH: &str = "/etc/erauchess.org/public";
    #[cfg(not(feature = "dist"))]
    const STATIC_PATH: &str = "../frontend/";

    rocket::build()
        .manage(db)
        .mount("/", FileServer::from(STATIC_PATH))
        .mount("/api/v1", routes![auth::login, auth::register, games::add])
}

///Possible errors from web & comms API
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ApiError {
    Unknown(String),
    UserNotFound,

    NotLoggedIn,
    IncorrectCredentials,

    /// Input data failed validation
    InvalidInput(String),

    /// The email the user wanted is already taken
    EmailTaken,
}

impl From<ApiError> for Status {
    fn from(err: ApiError) -> Self {
        match err {
            ApiError::Unknown(_) => Status::InternalServerError,
            ApiError::UserNotFound => Status::NotFound,
            ApiError::NotLoggedIn => Status::Forbidden,
            ApiError::IncorrectCredentials => Status::Unauthorized,
            ApiError::InvalidInput(_msg) => Status::BadRequest,
            ApiError::EmailTaken => Status::BadRequest,
        }
    }
}

impl From<ApiError> for String {
    fn from(err: ApiError) -> Self {
        match err {
            ApiError::Unknown(msg) => format!("unknown error: {}", msg),
            ApiError::UserNotFound => "user not found".to_owned(),
            ApiError::NotLoggedIn => "not logged in".to_owned(),
            ApiError::IncorrectCredentials => "incorrect credentials".to_owned(),
            ApiError::InvalidInput(msg) => format!("invalid input: {}", msg),
            ApiError::EmailTaken => "email taken".to_owned(),
        }
    }
}

impl<'r, T> Responder<'r, 'static> for ApiResponse<T>
where
    T: Serialize + std::fmt::Debug,
{
    fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'static> {
        let (body, status) = match self.0 {
            Ok(t) => {
                match serde_json::to_string(&t) {
                    Err(err) => {
                        error!(
                            "Failed to encode json response for: {:?}, error: {}",
                            t, err
                        );
                        return Err(Status::InternalServerError);
                    }
                    Ok(json) => {
                        //Assume object. Remove last curly brace
                        if json.ends_with('}') {
                            error!(
                                "Json object doesn't end with a }} character! Json: {}",
                                json
                            );
                            return Err(Status::InternalServerError);
                        }
                        let mut tmp = json.as_bytes().to_vec();
                        let end_curly_brace = tmp.remove(tmp.len() - 1);
                        if end_curly_brace != b'}' {
                            //This is provably impossible because we checked that the last char is }
                            //earlier but we'll keep it in for ultimate safety
                            error!(
                                "Json object ended with a }} character before but now the last character is {}! Json: {}",
                                end_curly_brace,
                                json
                            );
                            return Err(Status::InternalServerError);
                        }

                        //Add the success status message, making sure to replace the closing } we removed earlier
                        if tmp.len() == 1 {
                            //The json was a an empty object before ({}) so don't add a trailing comma
                            tmp.append(&mut "\"status\":\"success\"}".as_bytes().to_vec());
                        } else {
                            tmp.append(&mut ",\"status\":\"success\"}".as_bytes().to_vec());
                        }
                        (String::from_utf8(tmp).unwrap(), Status::Ok)
                    }
                }
            }
            Err(err) => {
                let message: String = err.clone().into();
                (
                    serde_json::json!({"status":"failure","error":message}).to_string(),
                    err.into(),
                )
            }
        };

        Response::build()
            .sized_body(body.len(), std::io::Cursor::new(body))
            .status(status)
            .header(ContentType::JSON)
            .ok()
    }
}

pub struct ApiResponse<T: Serialize + std::fmt::Debug>(pub Result<T, ApiError>);
