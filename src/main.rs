use axum;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::{
    collections::HashSet,
    env,
    sync::{Arc, Mutex},
};
use tokio::net::TcpListener;

mod business;
mod routes;
mod user;

#[derive(Clone)]
pub struct AppState {
    pool: PgPool,
    revoked_tokens: Arc<Mutex<HashSet<String>>>,
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind(&env::var("SERVER_ADDRESS").unwrap())
        .await
        .expect("Unable to connect to the server");

    // let pool = SqlitePool::connect(&env::var("DATABASE_URL").unwrap())
    //     .await
    //     .unwrap();

    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("POSTGRES_CONN").unwrap())
        .await
        .unwrap();
    sqlx::migrate!().run(&db).await.unwrap();

    let state = AppState {
        pool: db,
        revoked_tokens: Arc::new(Mutex::new(HashSet::new())),
    };

    let app = routes::app(state).await;

    println!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .await
        .expect("Error serving application");
}
