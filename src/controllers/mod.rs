use axum::{extract::State, http::StatusCode, Json};
use sqlx::SqlitePool;

use crate::{models::{LogInSignUpRequest, SendToken}, services::{authenticate_user, register_user::register_user}};


#[axum_macros::debug_handler]
pub async fn login(State(pool): State<SqlitePool>, Json(payload): Json<LogInSignUpRequest>) -> Result<Json<SendToken>, StatusCode> {
    match authenticate_user::auth(&pool, payload.username, payload.password).await {
        Ok(token) => return Ok(Json(token)),
        Err(e) => return Err(e.0)
    }
}

pub async fn signup(State(pool): State<SqlitePool>, Json(payload): Json<LogInSignUpRequest>) -> Result<Json<SendToken>, StatusCode> {
    match register_user(&pool, payload).await {
        Ok(token) => return Ok(Json(token)),
        Err(e) => return Err(e.0)
    }
}
