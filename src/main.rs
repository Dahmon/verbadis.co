use axum::{response::Html, routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/train", get(get_train).post(post_train));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

async fn root() -> Html<&'static str> {
    Html("<h1>Word Training</h1><a href='/train'>Train</a>")
}

async fn get_train() -> Html<&'static str> {
    // Return training page
    Html("<h1>Train!</h1><a href='/'>Home</a>")
}

async fn post_train() {}
