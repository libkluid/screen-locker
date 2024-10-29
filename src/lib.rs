use axum::{routing::post, Router};
use screenlocker::lock_screen;

async fn system_lock() {
    lock_screen().unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
    })
}

pub async fn run() {
    let app = Router::new().route("/api/system/lock", post(system_lock));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
