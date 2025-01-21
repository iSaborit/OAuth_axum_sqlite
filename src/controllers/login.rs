use axum::{extract::State, http::StatusCode, Json};
use serde::Deserialize;
use sqlx::SqlitePool;

use crate::{models::SendToken, services::authenticate_user};

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[axum_macros::debug_handler]
pub async fn login(State(pool): State<SqlitePool>, Json(payload): Json<LoginRequest>) -> Result<Json<SendToken>, StatusCode> {
    match authenticate_user::auth(&pool, payload.username, payload.password).await {
        Ok(token) => return Ok(Json(token)),
        Err(e) => return Err(e.0)
    }
}
