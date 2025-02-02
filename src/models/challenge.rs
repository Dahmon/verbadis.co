use std::sync::Arc;

use sqlx::FromRow;

use crate::AppState;

#[derive(FromRow)]
pub struct ChallengeRow {
    pub id: i64,
    pub word_id: i64,
    pub answer: Option<String>,
    pub corrected_answer: Option<String>,
    pub score: Option<u8>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(FromRow)]
pub struct Challenge {
    pub id: i64,
    pub word: String,
    pub class: String,
    pub definition: String,
    pub answer: Option<String>,
    pub corrected_answer: Option<String>,
    pub score: Option<u8>,
    pub created_at: String,
    pub updated_at: String,
}

impl Challenge {
    pub async fn create(state: &Arc<AppState>, word_id: &String) -> i64 {
        let results = sqlx::query("INSERT INTO challenges (word_id) VALUES ($1);")
            .bind(word_id)
            .execute(&state.pool)
            .await
            .expect("Failed to insert new challenge");

        results.last_insert_rowid()
    }

    pub async fn query(state: &Arc<AppState>, challenge_id: &String) -> Challenge {
        sqlx::query_as(
        "SELECT words.word, words.class, words.definition, challenges.id, challenges.answer, challenges.corrected_answer, challenges.score, challenges.created_at, challenges.updated_at
        FROM challenges
        INNER JOIN words
        ON challenges.word_id = words.id
        WHERE challenges.id = $1")
        .bind(challenge_id)
        .fetch_one(&state.pool)
        .await
        .expect("Failed to get challenge")
    }

    pub async fn query_all(state: &Arc<AppState>) -> Vec<Challenge> {
        sqlx::query_as(
        "SELECT words.word, words.class, words.definition, challenges.id, challenges.answer, challenges.corrected_answer, challenges.score, challenges.created_at, challenges.updated_at
        FROM challenges
        INNER JOIN words
        ON challenges.word_id = words.id")
        .fetch_all(&state.pool)
        .await
        .expect("Failed to get challenges")
    }

    pub async fn delete(state: &Arc<AppState>, challenge_id: &String) -> u64 {
        let result = sqlx::query("DELETE FROM challenges WHERE id = $1")
            .bind(challenge_id)
            .execute(&state.pool)
            .await
            .expect("Failed to delete challenge");

        result.rows_affected()
    }
}
