mod db;

use axum::Router;
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

#[derive(Serialize, Deserialize)]
struct User {
    id: i32,
    name: String,
}

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    dotenv().ok();

    let pool = db::create_pool().await.expect("Unable to create pool");

    let router = Router::new().with_state(pool);
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();

    axum::serve(listener, router).await.unwrap();
}
