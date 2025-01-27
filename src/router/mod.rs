use askama_axum::Template;
use axum::{response::Html, routing::get, Form};
use serde::Deserialize;

#[derive(Template)]
#[template(path = "root.html")]
struct RootTemplate {}

#[derive(Template)]
#[template(path = "train.html")]
struct TrainTemplate {}

#[derive(Deserialize)]
struct TrainSubmission {
    word: String,
    definition: String,
}

pub fn app_router() -> axum::Router {
    axum::Router::new()
        .route("/", get(root))
        .route("/train", get(get_train).post(post_train))
}

async fn root() -> RootTemplate {
    RootTemplate {}
}

async fn get_train() -> TrainTemplate {
    TrainTemplate {}
}

async fn post_train(Form(training): Form<TrainSubmission>) -> Html<String> {
    Html(format!(
        "<div>Word: {}, Definition: {}</div>",
        training.word, training.definition
    ))
}
