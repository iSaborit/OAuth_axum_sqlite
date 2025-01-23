use axum::{http::StatusCode, Json};
use serde_json::json;
use sqlx::{query, sqlite::SqlitePool};

pub async fn delete_tokens_id(
    pool: &SqlitePool,
    id: i64,
) -> Result<StatusCode, (StatusCode, Json<serde_json::Value>)> {
    query!("DELETE FROM tokens WHERE user_id = ?", id)
        .execute(pool)
        .await
        .map_err(|e| {
            (StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"success": "false", "error": e.to_string()})))
        })?;
    
    Ok(StatusCode::OK)
}
