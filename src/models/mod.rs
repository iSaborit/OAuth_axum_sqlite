use serde::Serialize;

pub struct User {
    pub id: Option<i64>,
    pub username: String,
    pub hash_passwd: String,
}

pub struct Token {
    pub id: i64,
    pub user_id: i64,
    pub access_token: String,
    pub refresh_token: String,
    pub access_token_expiration: chrono::NaiveDateTime,
    pub refresh_token_expiration: chrono::NaiveDateTime,
}

#[derive(Serialize)]
pub struct SendToken {
    pub user_id: i64,
    pub access_token: String,
    pub refresh_token: String,
    pub access_token_expiration: chrono::NaiveDateTime,
    pub refresh_token_expiration: chrono::NaiveDateTime,
}

pub enum AuthError {
    UsernameNotFound,
    IncorrectPassword,
    CouldNotWriteToken(sqlx::Error)
}

