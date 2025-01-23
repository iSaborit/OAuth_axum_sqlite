use axum::{
    extract::{Query, State},
    http::{Response, StatusCode},
    response::{IntoResponse, Redirect},
    Json,
};
use serde::Deserialize;
use sqlx::SqlitePool;

use crate::{
    models::{LogInSignUpRequest, Redirection, SendToken},
    services::{authenticate_user, get_id_by_username, logout, register_user::register_user},
};

use self::logout::delete_tokens_id;

#[axum_macros::debug_handler]
pub async fn login(
    State(pool): State<SqlitePool>,
    Query(redirection): Query<Redirection>,
    Json(payload): Json<LogInSignUpRequest>,
) -> Result<Redirect, (StatusCode, Redirect)> {
    match authenticate_user::auth(&pool, payload.username, payload.password).await {
        Ok(token) => {
            let success_url = format!(
                "{}?status=success&token={}",
                redirection.then_success, token.client_access_token
            );

            // Redirigimos al cliente a la URL construida
            Ok(Redirect::permanent(&success_url))
        }
        Err(e) => return Err((e.0, Redirect::permanent(&redirection.then_error))),
    }
}

#[derive(Deserialize)]
pub struct Username {
    username: String
}

#[axum_macros::debug_handler]
pub async fn logout(
    State(pool): State<SqlitePool>,
    Json(payload): Json<Username>
) -> Result<impl IntoResponse, StatusCode> {
    let id = match get_id_by_username(&pool, &payload.username).await {
        Ok(id) => {
            if let Err(error) = delete_tokens_id(&pool, id).await {
                return Err(error.0)
            }
        }
        Err(e) => {
            eprintln!("{}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    };
    Ok(StatusCode::OK)
}

pub async fn signup(
    State(pool): State<SqlitePool>,
    Json(payload): Json<LogInSignUpRequest>,
) -> Result<Json<SendToken>, StatusCode> {
    match register_user(&pool, payload).await {
        Ok(token) => return Ok(Json(token)),
        Err(e) => return Err(e.0),
    }
}
