use crate::model::{NewUser, User};
use crate::{schema::users, ApiError, ApiResponse, Db};

use chrono::{NaiveDateTime, Utc};
use data_encoding::BASE64;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use log::info;
use rocket::{
    form::{Form, FromForm},
    http::{Cookie, CookieJar, Status},
    request::{FromRequest, Outcome, Request},
    State,
};
use serde::{Deserialize, Serialize};

#[derive(FromForm)]
pub struct RegisterRequest {
    pub first_name: String,
    pub last_name: String,
    pub erau_id: Option<i32>,
    pub chess_com_username: String,
    pub email: String,
    pub password: String,
}

#[post("/register", data = "<params>")]
pub async fn register(
    state: &State<Db>,
    params: Form<RegisterRequest>,
    cookies: &CookieJar<'_>,
) -> ApiResponse<UserSession> {
    let conn = state.connect();

    let b64 = compute_password_hash(&params.password, &params.email);

    let now = NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0);
    let new_user = NewUser {
        first_name: params.first_name.as_str(),
        last_name: params.last_name.as_str(),
        hash: b64,
        erau_id: params.erau_id,
        chess_com_username: params.chess_com_username.clone(),
        email: params.email.clone(),
        signup_date: now,
    };

    match diesel::insert_into(users::table)
        .values(&new_user)
        .execute(&conn)
    {
        Ok(new_id) => {
            info!("inserted user: {}", new_id);
            let session = UserSession::new(0, false);
            cookies.add_private(session.as_cookie());
            ApiResponse(Ok(session))
        }
        Err(DieselError::DatabaseError(
            diesel::result::DatabaseErrorKind::UniqueViolation,
            desc,
        )) => {
            info!("Failed to add user: {:?}", desc);
            ApiResponse(Err(ApiError::EmailTaken))
        }
        Err(e) => {
            error!("Unknown Database Error during Registration {:?}", e);
            ApiResponse(Err(ApiError::Unknown("unknown database error".to_string())))
        }
    }
}

#[derive(FromForm)]
pub struct LoginParams {
    email: String,
    hash: String,
}

#[post("/login", data = "<params>")]
pub async fn login(
    state: &State<Db>,
    params: Form<LoginParams>,
    cookies: &CookieJar<'_>,
) -> ApiResponse<UserSession> {
    let conn = state.connect();

    use crate::schema::users::dsl::*;

    trace!("filtering by email {}", &params.email);
    let query = users
        .filter(email.eq(params.email.clone()))
        .first::<User>(&conn);

    // Decide whether to query by email or by username
    match query {
        Ok(user) => {
            let b64 = compute_password_hash(&params.hash, &params.email);
            if b64 == user.hash {
                let session = UserSession::new(user.id, user.is_officer);
                cookies.add_private(session.as_cookie());
                ApiResponse(Ok(session))
            } else {
                ApiResponse(Err(ApiError::IncorrectCredentials))
            }
        }
        Err(DieselError::NotFound) => ApiResponse(Err(ApiError::UserNotFound)),
        Err(e) => {
            error!("Database error: {}", e);
            ApiResponse(Err(ApiError::Unknown("unknown database error".to_string())))
        }
    }
}

pub fn is_session_valid(session: &UserSession) -> bool {
    let cur_time: u64 = std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    cur_time < session.start + session.length
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserSession {
    pub user: i32,
    pub length: u64,
    pub is_officer: bool,
    pub start: u64,
}

impl UserSession {
    fn new(user: i32, is_officer: bool) -> Self {
        UserSession {
            start: std::time::SystemTime::now()
                .duration_since(std::time::SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            user,
            is_officer,
            length: 30 * 24 * 60 * 60,
        }
    }

    fn as_cookie(&self) -> Cookie<'static> {
        let mut cookie = Cookie::new(
            "Session",
            serde_json::to_string(self)
                .expect("must be able to serialize a session cookie we generate"),
        );

        cookie.set_expires(Some(
            time::OffsetDateTime::from(std::time::SystemTime::now())
                + time::Duration::seconds(self.length as i64),
        ));
        cookie.set_http_only(Some(false)); //frontend uses this to figure out if we're logged in or not
        cookie
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserSession {
    type Error = ApiError;
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match request.cookies().get_private("Session") {
            Some(session) => match serde_json::from_str::<UserSession>(session.value()) {
                Ok(obj) => match is_session_valid(&obj) {
                    false => {
                        trace!("not logged in! - session before entry in logouts db");
                        Outcome::Failure((Status::Forbidden, ApiError::NotLoggedIn))
                    }
                    true => Outcome::Success(obj.clone()),
                },
                Err(_) => Outcome::Failure((Status::BadRequest, ApiError::NotLoggedIn)),
            },
            None => Outcome::Failure((Status::BadRequest, ApiError::NotLoggedIn)),
        }
    }
}

fn compute_password_hash(password: &str, salt: &str) -> String {
    let digest = crypto::sha2::Sha512::new();
    let mut mac = crypto::hmac::Hmac::new(digest, password.as_bytes());
    let mut buff = [0u8; 64];
    let start = std::time::Instant::now();
    crypto::pbkdf2::pbkdf2(&mut mac, salt.as_bytes(), 750, &mut buff);
    info!(
        "pbkdf took {} ms",
        (std::time::Instant::now() - start).as_micros() as f64 / 1000.0
    );
    BASE64.encode(&buff)
}
