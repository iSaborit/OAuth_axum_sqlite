use std::error::Error;

use sqlx::{query, query_as};

use crate::models::{SendToken, Token, User};

pub(crate) mod authenticate_user;

async fn find_user_by_username(pool: &sqlx::sqlite::SqlitePool, username: &str) -> Option<User> {
    if let Ok(user) = query_as!(User, "SELECT * FROM USERS WHERE username = ?", username)
        .fetch_one(pool)
        .await
    {
        Some(user)
    } else {
        None
    }
}

async fn store_token_db(
    pool: &sqlx::sqlite::SqlitePool,
    token: &SendToken,
) -> Result<(), sqlx::Error> {
    query!(
        r#"
    INSERT INTO tokens 
    (user_id,
    access_token,
    refresh_token,
    access_token_expiration,
    refresh_token_expiration)
    VALUES (?, ?, ?, ?, ?)"#,
        token.user_id,
        token.access_token,
        token.refresh_token,
        token.access_token_expiration,
        token.refresh_token_expiration
    )
    .execute(pool)
    .await
    .map_err(|e| return e);

    Ok(())
}
