use std::sync::Arc;

use askama_axum::Template;
use axum::{
    response::Html,
    routing::{get, post},
    Form,
};
use serde::Deserialize;

use crate::AppState;

#[derive(Template)]
#[template(path = "root.html")]
struct RootTemplate {}

struct Word<'a> {
    id: u32,
    word: &'a str,
    class: &'a str,
    definition: &'a str,
    example: Option<&'a str>,
}

#[derive(Template)]
#[template(path = "pages/words.html")]
struct WordsTemplate<'a> {
    words: Vec<Word<'a>>,
}

#[derive(Template)]
#[template(path = "train.html")]
struct TrainTemplate {}

#[derive(Deserialize)]
struct TrainSubmission {
    word: String,
    definition: String,
}

pub fn app_router(state: Arc<AppState>) -> axum::Router {
    axum::Router::new()
        .route("/", get(root))
        .route("/words", get(get_words))
        .route("/words/new", post(post_words_new))
        .route("/train", get(get_train).post(post_train))
        .with_state(state)
}

async fn root() -> RootTemplate {
    RootTemplate {}
}

async fn get_words<'a>() -> WordsTemplate<'a> {
    let words = vec![Word {
        id: 0,
        word: "Hello",
        class: "injunction",
        definition: "A greeting",
        example: None,
    }];

    WordsTemplate { words }
}

async fn post_words_new() {}

async fn get_train() -> TrainTemplate {
    TrainTemplate {}
}

async fn post_train(Form(training): Form<TrainSubmission>) -> Html<String> {
    Html(format!(
        "<div>Word: {}, Definition: {}</div>",
        training.word, training.definition
    ))
}
