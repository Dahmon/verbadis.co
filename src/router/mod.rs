mod words;

use std::sync::Arc;

use askama_axum::{IntoResponse, Template};
use axum::{
    debug_handler,
    extract::{Path, Query, Request, State},
    response::Redirect,
    routing::{get, post},
    Form,
};
use serde::Deserialize;
use sqlx::prelude::FromRow;

use crate::AppState;

#[derive(Template)]
#[template(path = "root.html")]
struct RootPageTemplate {}

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

#[derive(FromRow)]
struct Challenge {
    id: u32,
    word_id: u32,
    answer: Option<String>,
    corrected_answer: Option<String>,
    score: Option<u8>,
    created_at: String,
    updated_at: String,
}

#[derive(Template)]
#[template(path = "pages/challenges.html")]
struct ChallengesPageTemplate {
    challenges: Vec<Challenge>,
}

#[derive(Template)]
#[template(path = "pages/challenge.html")]
struct ChallengePageTemplate {
    challenge: Challenge,
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

pub fn app_router(state: Arc<AppState>) -> axum::Router {
    axum::Router::new()
        .route("/", get(root))
        .route("/words", get(get_words))
        .route("/words/new", post(post_words_new))
        .route("/challenges", get(get_challenges))
        .route("/challenges/{id}", get(get_challenge))
        .route("/challenges/new", post(post_new_challenge))
        .with_state(state)
}

async fn root() -> RootPageTemplate {
    RootPageTemplate {}
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

async fn get_challenges(State(state): State<Arc<AppState>>) -> ChallengesPageTemplate {
    let challenges: Vec<Challenge> = sqlx::query_as("SELECT * FROM challenges")
        .fetch_all(&state.pool)
        .await
        .expect("Failed to get challenges");

    ChallengesPageTemplate { challenges }
}

async fn get_challenge(
    State(state): State<Arc<AppState>>,
    Path(challenge_id): Path<String>,
) -> ChallengePageTemplate {
    let challenge: Challenge = sqlx::query_as("SELECT * FROM challenges WHERE id = $1")
        .bind(&challenge_id)
        .fetch_one(&state.pool)
        .await
        .expect("Failed to get challenge");

    ChallengePageTemplate { challenge }
}

#[derive(Deserialize)]
struct NewChallengeForm {
    word_id: String,
}

#[debug_handler]
async fn post_new_challenge(
    State(state): State<Arc<AppState>>,
    Form(form): Form<NewChallengeForm>,
) -> Redirect {
    let results = sqlx::query("INSERT INTO challenges (word_id) VALUES ($1);")
        .bind(&form.word_id)
        .execute(&state.pool)
        .await
        .expect("Failed to insert new challenge");

    let redirect_url = format!("/challenges/{}", results.last_insert_rowid());

    Redirect::to(&redirect_url)
}
