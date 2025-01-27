use askama_axum::Template;
use axum::{response::Html, routing::get, Router};

#[derive(Template)]
#[template(path = "root.html")]
struct RootTemplate {}

#[derive(Template)]
#[template(path = "train.html")]
struct TrainTemplate {}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/train", get(get_train).post(post_train));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

async fn root() -> RootTemplate {
    RootTemplate {}
}

async fn get_train() -> TrainTemplate {
    TrainTemplate {}
}

async fn post_train() -> Html<&'static str> {
    Html("")
}
