use std::sync::Arc;

use askama_axum::{IntoResponse, Template};
use axum::{
    extract::{Query, Request, State},
    routing::{get, post},
    Form, Router,
};
use serde::Deserialize;
use sqlx::prelude::FromRow;

use crate::AppState;

#[derive(FromRow)]
struct Word {
    id: u32,
    word: String,
    class: String,
    definition: String,
    example: Option<String>,
    created_at: String,
    updated_at: String,
}

#[derive(Deserialize)]
struct NewWordForm {
    word: String,
    class: String,
    definition: String,
    example: String,
}

#[derive(Deserialize)]
struct SearchForm {
    search: Option<String>,
}

#[derive(Template)]
#[template(path = "pages/words.html")]
struct WordsPageTemplate {
    words: Vec<Word>,
    search: String,
}

#[derive(Template)]
#[template(path = "components/word-list.html")]
struct WordsListTemplate {
    words: Vec<Word>,
}

enum WordsTemplates {
    Page(WordsPageTemplate),
    List(WordsListTemplate),
}

// Even though both values implement IntoResponse, need to manually re-implement here.
impl IntoResponse for WordsTemplates {
    fn into_response(self) -> askama_axum::Response {
        match self {
            WordsTemplates::List(a) => a.into_response(),
            WordsTemplates::Page(b) => b.into_response(),
        }
    }
}

#[axum::debug_handler]
async fn get_words(
    State(state): State<Arc<AppState>>,
    Query(query): Query<SearchForm>,
    req: Request,
) -> WordsTemplates {
    let is_search = req
        .headers()
        .get("HX-Target")
        .and_then(|value| value.to_str().ok())
        == Some("words-list");

    let search: String = match &query.search {
        Some(search) => search.clone(),
        None => String::new(),
    };

    let words: Vec<Word> = sqlx::query_as("SELECT * FROM words WHERE word LIKE '%' || $1 || '%'")
        .bind(&search)
        .fetch_all(&state.pool)
        .await
        .expect("Failed to get words");

    if is_search {
        WordsTemplates::List(WordsListTemplate { words })
    } else {
        WordsTemplates::Page(WordsPageTemplate {
            words,
            search: search.clone(),
        })
    }
}

async fn post_words_new(
    State(state): State<Arc<AppState>>,
    Form(form): Form<NewWordForm>,
) -> WordsListTemplate {
    sqlx::query("INSERT INTO words (word, class, definition, example) VALUES ($1, $2, $3, $4);")
        .bind(form.word)
        .bind(form.class)
        .bind(form.definition)
        .bind(form.example)
        .execute(&state.pool)
        .await
        .expect("Failed to insert new word");

    // TODO: Flash success/error message on session

    let words: Vec<Word> = sqlx::query_as("SELECT * FROM words")
        .fetch_all(&state.pool)
        .await
        .expect("Failed to get words");

    WordsListTemplate { words }
}

pub fn words_router() -> Router<Arc<AppState>> {
    axum::Router::new()
        .route("/words", get(get_words))
        .route("/words/new", post(post_words_new))
}
