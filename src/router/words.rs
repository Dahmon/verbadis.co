use std::sync::Arc;

use askama_axum::{IntoResponse, Template};
use axum::{
    extract::{Path, Query, Request, State},
    http::StatusCode,
    routing::{delete, get, post},
    Form, Router,
};
use serde::Deserialize;

use crate::{
    models::word::{NewWord, WordRow},
    AppState,
};

#[derive(Deserialize)]
struct SearchForm {
    search: Option<String>,
}

#[derive(Template)]
#[template(path = "pages/words.html")]
struct WordsPageTemplate {
    words: Vec<WordRow>,
    search: Option<String>,
}

#[derive(Template)]
#[template(path = "components/word-list.html")]
struct WordsListTemplate {
    words: Vec<WordRow>,
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

    let words: Vec<WordRow> = match &query.search {
        Some(search) => WordRow::query_all(&state, search).await,
        None => WordRow::get_all(&state).await,
    };

    match is_search {
        true => WordsTemplates::List(WordsListTemplate { words }),
        false => WordsTemplates::Page(WordsPageTemplate {
            words,
            search: query.search,
        }),
    }
}

async fn post_words_new(
    State(state): State<Arc<AppState>>,
    Form(form): Form<NewWord>,
) -> (StatusCode, WordsListTemplate) {
    WordRow::create(&state, form).await;
    let words: Vec<WordRow> = WordRow::get_all(&state).await;

    // TODO: Flash success/error message on session

    (StatusCode::CREATED, WordsListTemplate { words })
}

async fn delete_word(
    State(state): State<Arc<AppState>>,
    Path(word_id): Path<String>,
) -> WordsListTemplate {
    WordRow::delete(&state, &word_id.parse().unwrap())
        .await
        .unwrap();
    let words: Vec<WordRow> = WordRow::get_all(&state).await;

    WordsListTemplate { words }
}

pub fn words_router() -> Router<Arc<AppState>> {
    axum::Router::new()
        .route("/words", get(get_words))
        .route("/words/:word_id", delete(delete_word))
        .route("/words/new", post(post_words_new))
}
