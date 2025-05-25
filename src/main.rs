use std::sync::Arc;

use embeddings::{
    meaning::{self, MeaningEmbeddingFunction},
    ngram::{self, NgramEmbeddingFunction},
};

use lancedb::{
    arrow::arrow_schema::{DataType, Field, Schema},
    embeddings::EmbeddingDefinition,
    Connection, Table,
};
use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    Pool, Sqlite,
};

mod embeddings;
mod models;
mod router;

struct AppState {
    pool: Pool<Sqlite>,
    table: Table,
}

async fn create_or_get_table(db: &Connection) -> Result<Table, Box<dyn std::error::Error>> {
    let table = db.open_table("words").execute().await;

    if let lancedb::Result::Ok(table) = table {
        return Ok(table);
    }
    let point_field = Arc::new(Field::new("point", DataType::Float32, false));
    let ngram_field = Arc::new(Field::new("month", DataType::Float32, false));
    let schema = Schema::new(vec![
        Field::new("word", DataType::Utf8, false),
        Field::new(
            "meaning_embed",
            DataType::FixedSizeList(point_field, 768),
            false,
        ),
        Field::new(
            "ngram_vector",
            DataType::FixedSizeList(ngram_field, 150),
            false,
        ),
    ]);

    let table = db
        .create_empty_table("words2", Arc::new(schema))
        .add_embedding(EmbeddingDefinition::new(
            "word",
            "meaning",
            Some("meaning_embed"),
        ))?
        .add_embedding(EmbeddingDefinition::new(
            "word",
            "ngram",
            Some("ngram_vector"),
        ))?
        .execute()
        .await?;

    Ok(table)
}

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());

    // Old Sqlite
    let options = SqliteConnectOptions::new()
        .filename(dotenv::var("DATABASE_PATH").unwrap())
        .create_if_missing(true);
    let pool = SqlitePoolOptions::new()
        .connect_with(options)
        .await
        .unwrap();

    // LanceDB
    let meaing_embedding = MeaningEmbeddingFunction::new("nomic-embed-text");
    let ngram_embedding = NgramEmbeddingFunction::new();

    let db = lancedb::connect("data/db").execute().await.unwrap();
    db.embedding_registry()
        .register("meaning", Arc::new(meaing_embedding))
        .unwrap();
    db.embedding_registry()
        .register("ngram", Arc::new(ngram_embedding))
        .unwrap();

    let table = create_or_get_table(&db).await.unwrap();
    let app_state = Arc::new(AppState { pool, table });

    axum::serve(listener, router::app_router(app_state))
        .await
        .unwrap();
}
