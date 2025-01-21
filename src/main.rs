use std::{env, net::SocketAddr};

use axum::{routing::{get, post}, Router};
use sqlx::sqlite::SqlitePoolOptions;

mod models;
mod controllers;
mod services;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

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
        .route("/login", post(controllers::login::login))
        .route("/logout", post("logging out..."))
        .route("/refresh-token", post("hello"))
        .route("/me", get("user searched"))
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
