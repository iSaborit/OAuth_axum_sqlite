use axum::{http::StatusCode};
use bcrypt::verify;
use chrono::{Duration, Utc};
use uuid::Uuid;

use crate::models::{AuthError, SendToken, Token};

use super::{find_user_by_username, store_token_db};


pub async fn auth(pool: &sqlx::sqlite::SqlitePool, username: String, passwd: String) -> Result<SendToken, (StatusCode, AuthError)> {
    let user = match find_user_by_username(pool, &username).await {
        Some(user) => user,
        None => return Err((StatusCode::BAD_REQUEST, AuthError::UsernameNotFound))
    };


    if verify(passwd, &user.hash_passwd).unwrap() {
        let access_token_expiration = (Utc::now() + Duration::hours(1)).naive_utc();
        let refresh_token_expiration = (Utc::now() + Duration::days(7)).naive_utc();

        let token = Token {
            id: -1,
            user_id: user.id.unwrap(),
            client_access_token: Uuid::new_v4().to_string(),
            server_access_token: Uuid::new_v4().to_string(),
            refresh_token: Uuid::new_v4().to_string(),
            access_token_expiration,
            refresh_token_expiration,
        };

        store_token_db(pool, &token).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, AuthError::CouldNotWriteToken(e)))?;        
        Ok(token.to_send_token())

    } else {
        return Err((StatusCode::BAD_REQUEST, AuthError::IncorrectPassword))
    }

}
