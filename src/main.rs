use axum;
use tokio::net::TcpListener;

mod business;
mod routes;
mod user;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:7878")
        .await
        .expect("Unable to connect to the server");
    let app = routes::app().await;

    println!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .await
        .expect("Error serving application");
}
