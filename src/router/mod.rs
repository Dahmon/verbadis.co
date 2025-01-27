use std::sync::Arc;

use askama_axum::Template;
use axum::{
    extract::State,
    response::Redirect,
    routing::{get, post},
    Form,
};
use serde::Deserialize;
use sqlx::prelude::FromRow;

use crate::AppState;

#[derive(Template)]
#[template(path = "root.html")]
struct RootTemplate {}

#[derive(FromRow)]
struct Word {
    id: u32,
    word: String,
    class: String,
    definition: String,
    example: Option<String>,
}

#[derive(Deserialize)]
struct NewWordForm {
    word: String,
    class: String,
    definition: String,
    example: String,
}

#[derive(Template)]
#[template(path = "pages/words.html")]
struct WordsTemplate {
    words: Vec<Word>,
}

pub fn app_router(state: Arc<AppState>) -> axum::Router {
    axum::Router::new()
        .route("/", get(root))
        .route("/words", get(get_words))
        .route("/words/new", post(post_words_new))
        .with_state(state)
}

async fn root() -> RootTemplate {
    RootTemplate {}
}

async fn get_words(State(state): State<Arc<AppState>>) -> WordsTemplate {
    let words: Vec<Word> = sqlx::query_as("SELECT * FROM words")
        .fetch_all(&state.pool)
        .await
        .expect("Failed to get words");

    WordsTemplate { words }
}

async fn post_words_new(
    State(state): State<Arc<AppState>>,
    Form(form): Form<NewWordForm>,
) -> Redirect {
    sqlx::query("INSERT INTO words (word, class, definition, example) VALUES ($1, $2, $3, $4);")
        .bind(form.word)
        .bind(form.class)
        .bind(form.definition)
        .bind(form.example)
        .execute(&state.pool)
        .await
        .expect("Failed to insert new word");

    // TODO: Flash success/error message on session

    // TODO: Replace #words-list with fragment
    Redirect::to("/words")
}
