use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    Json,
};
use serde::Deserialize;
use sqlx::SqlitePool;

use crate::{
    models::{LogInSignUpRequest, Redirection, SendToken},
    services::{authenticate_user, get_id_by_username, get_new_tokens, logout, register_user::register_user},
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

#[derive(Deserialize)]
pub struct RToken {
    refresh_tkn: String
}

pub async fn refresh_token(
    State(pool): State<SqlitePool>,
    Json(payload): Json<RToken>
) -> Result<impl IntoResponse, StatusCode> {
    if let Ok(values) = get_new_tokens(&pool, &payload.refresh_tkn).await {
        return Ok(values);
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
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
