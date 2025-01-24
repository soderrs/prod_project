use std::{
    collections::HashSet,
    env,
    sync::{Arc, Mutex},
};

use axum;
use sqlx::SqlitePool;
use tokio::net::TcpListener;

mod business;
mod routes;
mod user;

#[derive(Clone)]
pub struct AppState {
    pool: SqlitePool,
    revoked_tokens: Arc<Mutex<HashSet<String>>>,
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:7878")
        .await
        .expect("Unable to connect to the server");

    let pool = SqlitePool::connect(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    let state = AppState {
        pool,
        revoked_tokens: Arc::new(Mutex::new(HashSet::new())),
    };

    let app = routes::app(state).await;

    println!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .await
        .expect("Error serving application");
}
