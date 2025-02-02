use crate::AppState;

use askama_axum::Template;
use axum::{
    debug_handler,
    extract::{Path, State},
    response::Redirect,
    routing::{get, post},
    Form, Router,
};
use serde::Deserialize;
use sqlx::prelude::FromRow;
use std::sync::Arc;

#[derive(FromRow)]
struct Challenge {
    id: u32,
    word: String,
    class: String,
    definition: String,
    answer: Option<String>,
    corrected_answer: Option<String>,
    score: Option<u8>,
    created_at: String,
    updated_at: String,
}

#[derive(Template)]
#[template(path = "pages/challenges.html")]
struct ChallengesPageTemplate {
    // words: ,
    challenges: Vec<Challenge>,
}

#[derive(Template)]
#[template(path = "pages/challenge.html")]
struct ChallengePageTemplate {
    challenge: Challenge,
}

#[derive(Deserialize)]
struct NewChallengeForm {
    word_id: String,
}

async fn get_challenges(State(state): State<Arc<AppState>>) -> ChallengesPageTemplate {
    let challenges: Vec<Challenge> = sqlx::query_as(
        "SELECT words.word, words.class, words.definition, challenges.id, challenges.answer, challenges.corrected_answer, challenges.score, challenges.created_at, challenges.updated_at
        FROM challenges
        INNER JOIN words
        ON challenges.word_id = words.id")
        .fetch_all(&state.pool)
        .await
        .expect("Failed to get challenges");

    ChallengesPageTemplate { challenges }
}

async fn get_challenge(
    State(state): State<Arc<AppState>>,
    Path(challenge_id): Path<String>,
) -> ChallengePageTemplate {
    let challenge: Challenge = sqlx::query_as(
        "SELECT words.word, words.class, words.definition, challenges.id, challenges.answer, challenges.corrected_answer, challenges.score, challenges.created_at, challenges.updated_at
        FROM challenges
        INNER JOIN words
        ON challenges.word_id = words.id
        WHERE challenges.id = $1")
        .bind(&challenge_id)
        .fetch_one(&state.pool)
        .await
        .expect("Failed to get challenge");

    ChallengePageTemplate { challenge }
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

pub fn challenges_router() -> Router<Arc<AppState>> {
    axum::Router::new()
        .route("/challenges", get(get_challenges))
        .route("/challenges/:challenge_id", get(get_challenge))
        .route("/challenges/new", post(post_new_challenge))
}
