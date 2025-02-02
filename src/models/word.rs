use std::sync::Arc;

use dotenv::Error;
use serde::Deserialize;
use sqlx::{Database, FromRow};

use crate::AppState;

#[derive(FromRow)]
pub struct WordRow {
    pub id: i64,
    pub word: String,
    pub class: String,
    pub definition: String,
    pub example: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Deserialize)]
pub struct NewWord {
    pub word: String,
    pub class: String,
    pub definition: String,
    pub example: String,
}

impl WordRow {
    pub async fn create(state: &Arc<AppState>, new_word: NewWord) -> i64 {
        let result = sqlx::query(
            "INSERT INTO words (word, class, definition, example) VALUES ($1, $2, $3, $4);",
        )
        .bind(&new_word.word)
        .bind(&new_word.class)
        .bind(&new_word.definition)
        .bind(&new_word.example)
        .execute(&state.pool)
        .await
        .expect("Failed to insert new word");

        result.last_insert_rowid()
    }

    pub async fn query_all(state: &Arc<AppState>, search: &String) -> Vec<WordRow> {
        sqlx::query_as("SELECT * FROM words WHERE word LIKE '%' || $1 || '%'")
            .bind(search)
            .fetch_all(&state.pool)
            .await
            .expect("Failed to get words with search")
    }

    pub async fn get_all(state: &Arc<AppState>) -> Vec<WordRow> {
        sqlx::query_as("SELECT * FROM words")
            .fetch_all(&state.pool)
            .await
            .expect("Failed to get words")
    }

    pub async fn delete(state: &Arc<AppState>, word_id: &i64) -> Result<u64, ()> {
        match sqlx::query("DELETE FROM words WHERE id = $1")
            .bind(word_id)
            .execute(&state.pool)
            .await
        {
            Ok(aaa) => Ok(aaa.rows_affected()),
            Err(sqlx::Error::Database(db_err)) => {
                if let Some(code) = &db_err.code() {
                    if code == "787" {
                        // TODO: Indicate foreign key constraint failed
                    }
                };
                Err(())
            }
            Err(_) => Err(()),
        }
    }
}
