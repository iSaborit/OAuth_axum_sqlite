use axum::http::StatusCode;
use chrono::{Duration, Utc};
use uuid::Uuid;

use crate::models::{AuthError, LogInSignUpRequest, SendToken};

use super::{store_token_db, store_user_db};

pub async fn register_user(
    pool: &sqlx::sqlite::SqlitePool,
    user: LogInSignUpRequest,
) -> Result<SendToken, (StatusCode, AuthError)> {
    let access_token_expiration = (Utc::now() + Duration::hours(1)).naive_utc();
    let refresh_token_expiration = (Utc::now() + Duration::days(7)).naive_utc();

    let id = store_user_db(pool, &user).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            AuthError::CouldNotWriteUser(e),
        )
    })?;

    let send_token = SendToken {
        user_id: id,
        access_token: Uuid::new_v4().to_string(),
        refresh_token: Uuid::new_v4().to_string(),
        access_token_expiration,
        refresh_token_expiration,
    };

    store_token_db(pool, &send_token).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            AuthError::CouldNotWriteToken(e),
        )
    })?;

    Ok(send_token)
}
