use std::sync::Arc;

use crate::{
    models::{challenge::Challenge, word::WordRow},
    AppState,
};

use askama_axum::Template;
use axum::{
    debug_handler,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    routing::{get, post},
    Form, Router,
};
use serde::Deserialize;

#[derive(Template)]
#[template(path = "pages/challenges.html")]
struct ChallengesPageTemplate {
    challenges: Vec<Challenge>,
    words: Vec<WordRow>,
}

#[derive(Template)]
#[template(path = "components/challenge-list.html")]
struct ChallengesListTemplate {
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
    let challenges: Vec<Challenge> = Challenge::query_all(&state).await;
    let words: Vec<WordRow> = sqlx::query_as("SELECT * FROM words")
        .fetch_all(&state.pool)
        .await
        .expect("Failed to get words");

    ChallengesPageTemplate { challenges, words }
}

async fn get_challenge(
    State(state): State<Arc<AppState>>,
    Path(challenge_id): Path<String>,
) -> ChallengePageTemplate {
    let challenge = Challenge::query(&state, &challenge_id).await;

    ChallengePageTemplate { challenge }
}

#[debug_handler]
async fn post_new_challenge(
    State(state): State<Arc<AppState>>,
    Form(form): Form<NewChallengeForm>,
) -> (StatusCode, HeaderMap) {
    let row_id = Challenge::create(&state, &form.word_id).await;
    let redirect_url = format!("/challenges/{}", row_id);

    let mut headers = HeaderMap::new();
    headers.insert("HX-Location", redirect_url.parse().unwrap());

    (StatusCode::CREATED, headers)
}

async fn delete_challenge(
    State(state): State<Arc<AppState>>,
    Path(challenge_id): Path<String>,
) -> ChallengesListTemplate {
    Challenge::delete(&state, &challenge_id).await;
    let challenges = Challenge::query_all(&state).await;

    ChallengesListTemplate { challenges }
}

pub fn challenges_router() -> Router<Arc<AppState>> {
    axum::Router::new()
        .route("/challenges", get(get_challenges))
        .route(
            "/challenges/:challenge_id",
            get(get_challenge).delete(delete_challenge),
        )
        .route("/challenges/new", post(post_new_challenge))
}
