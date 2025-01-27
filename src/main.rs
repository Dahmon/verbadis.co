use std::{env, sync::Arc};

use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    Pool, Sqlite,
};

mod router;

struct AppState {
    pool: Pool<Sqlite>,
}

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());

    let options = SqliteConnectOptions::new()
        .filename(dotenv::var("DATABASE_PATH").unwrap())
        .create_if_missing(true);
    let pool = SqlitePoolOptions::new()
        .connect_with(options)
        .await
        .unwrap();

    let app_state = Arc::new(AppState { pool });

    axum::serve(listener, router::app_router(app_state))
        .await
        .unwrap();
}
