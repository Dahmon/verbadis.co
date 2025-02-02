mod challenges;
mod words;

use askama_axum::{IntoResponse, Template};
use axum::{
    body::Body,
    http::{Request, Response, StatusCode, Uri},
    routing::get,
};
use challenges::challenges_router;
use std::sync::Arc;
use tower_http::services::ServeFile;

use words::words_router;

use crate::AppState;

#[derive(Template)]
#[template(path = "pages/root.html")]
struct RootPageTemplate {}

pub fn app_router(state: Arc<AppState>) -> axum::Router {
    axum::Router::new()
        .route("/", get(root))
        .route("/styles/styles.css", get(get_styles))
        .merge(words_router())
        .merge(challenges_router())
        .with_state(state)
}

async fn root() -> RootPageTemplate {
    RootPageTemplate {}
}

async fn get_styles(uri: Uri) -> Result<Response<Body>, (StatusCode, String)> {
    let req = Request::builder().uri(uri).body(Body::empty()).unwrap();
    match ServeFile::new("styles/styles.css").try_call(req).await {
        Ok(res) => Ok(res.into_response()),
        Err(err) => Err((
            StatusCode::NOT_FOUND,
            format!("Unable to find file: {}", err),
        )),
    }
}
