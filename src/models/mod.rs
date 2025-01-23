use serde::{Deserialize, Serialize};

pub struct User {
    pub id: Option<i64>,
    pub username: String,
    pub hash_passwd: String,
}

#[derive(Deserialize)]
pub struct LogInSignUpRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct Redirection {
    pub then_success: String,
    pub then_error: String
}

pub struct Token {
    pub id: i64,
    pub user_id: i64,
    pub client_access_token: String,
    pub server_access_token: String,
    pub refresh_token: String,
    pub access_token_expiration: chrono::NaiveDateTime,
    pub refresh_token_expiration: chrono::NaiveDateTime,
}

impl Token {
    pub fn to_send_token(&self) -> SendToken {
        SendToken {
            user_id: self.user_id,
            client_access_token: self.client_access_token.clone(),
            refresh_token: self.refresh_token.clone(),
            access_token_expiration: self.access_token_expiration,
            refresh_token_expiration: self.refresh_token_expiration
        }
    }
}

#[derive(Serialize)]
pub struct SendToken {
    pub user_id: i64,
    pub client_access_token: String,
    pub refresh_token: String,
    pub access_token_expiration: chrono::NaiveDateTime,
    pub refresh_token_expiration: chrono::NaiveDateTime,
}

pub enum AuthError {
    UsernameNotFound,
    IncorrectPassword,
    CouldNotWriteToken(sqlx::Error),
    CouldNotWriteUser(sqlx::Error),
}
