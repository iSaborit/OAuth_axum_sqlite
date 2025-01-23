pub(crate) mod authenticate_user;
pub(crate) mod register_user;
pub(crate) mod logout;

use crate::models::{LogInSignUpRequest, SendToken, Token, User};
use sqlx::{query, query_as};

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

pub async fn get_id_by_username(
    pool: &sqlx::sqlite::SqlitePool,
    username: &str,
) -> Result<i64, sqlx::Error> {
    let user_id = query_as!(User, "SELECT * FROM users WHERE username = ?", username)
        .fetch_one(pool)
        .await?
        .id
        .expect("Should have an ID!");

    Ok(user_id)
}

async fn store_user_db(pool: &sqlx::sqlite::SqlitePool, user: &LogInSignUpRequest) -> Result<i64, sqlx::Error> {
    let hash_passwd = bcrypt::hash(user.password.clone(), 4).unwrap();
    query!(
        r#"INSERT INTO users
    (username,
    hash_passwd)
    VALUES (?, ?)"#,
        user.username,
        hash_passwd
    )
    .execute(pool)
    .await?;

    get_id_by_username(pool, &user.username).await
}

async fn store_token_db(
    pool: &sqlx::sqlite::SqlitePool,
    token: &Token,
) -> Result<(), sqlx::Error> {
    query!(
        r#"
    INSERT INTO tokens 
    (user_id,
    client_access_token,
    server_access_token,
    refresh_token,
    access_token_expiration,
    refresh_token_expiration)
    VALUES (?, ?, ?, ?, ?, ?)"#,
        token.user_id,
        token.client_access_token,
        token.server_access_token,
        token.refresh_token,
        token.access_token_expiration,
        token.refresh_token_expiration
    )
    .execute(pool)
    .await?;

    Ok(())
}
