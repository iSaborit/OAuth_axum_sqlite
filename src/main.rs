use std::{env, net::SocketAddr};

use axum::{http::Method, routing::{get, post}, Router};
use sqlx::sqlite::SqlitePoolOptions;
use tower_http::cors::{Any, CorsLayer};

mod models;
mod controllers;
mod services;

use controllers::{login, logout, signup};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cors = CorsLayer::new()
        .allow_origin(Any) // Permite cualquier origen
        .allow_methods([Method::GET, Method::POST]) // Métodos permitidos
        .allow_headers(Any);

    dotenv::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Should have been able to connect to the DB.");

    sqlx::migrate!("./migrations").run(&pool).await?;
    
    let app = Router::new()
        .route("/", get(root))
        .route("/login", post(login))
        .route("/signup", post(signup))
        .route("/logout", post(logout))
        .route("/refresh-token", post("hello"))
        .route("/me", get("user searched"))
        .layer(cors)
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!(">>> Server running on: http://{addr}");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await?;

    Ok(())
}

async fn root() -> &'static str {
    "Hello O'Auth!"
}
