use std::{borrow::Cow, sync::Arc};

use arrow_array::Array;
use lancedb::{
    arrow::arrow_schema::{DataType, Field},
    embeddings::EmbeddingFunction,
};

#[derive(Debug)]
pub struct NgramEmbeddingFunction {}

impl NgramEmbeddingFunction {
    pub fn new() -> NgramEmbeddingFunction {
        Self {}
    }
}

impl EmbeddingFunction for NgramEmbeddingFunction {
    fn name(&self) -> &str {
        "ngram"
    }

    fn source_type(&self) -> lancedb::Result<Cow<lancedb::arrow::arrow_schema::DataType>> {
        lancedb::Result::Ok(Cow::Owned(DataType::Utf8))
    }

    fn compute_source_embeddings(&self, source: Arc<dyn Array>) -> lancedb::Result<Arc<dyn Array>> {
        let source_str = source
            .as_any()
            .downcast_ref::<arrow_array::StringArray>()
            .unwrap();
    }

    fn dest_type(&self) -> lancedb::Result<Cow<lancedb::arrow::arrow_schema::DataType>> {
        lancedb::Result::Ok(Cow::Owned(DataType::FixedSizeList(
            Arc::new(Field::new("point", DataType::Float32, false)),
            150,
        )))
    }

    fn compute_query_embeddings(&self, input: Arc<dyn Array>) -> lancedb::Result<Arc<dyn Array>> {
        let source_str = input
            .as_any()
            .downcast_ref::<arrow_array::StringArray>()
            .unwrap();
    }
}
