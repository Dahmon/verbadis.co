mod challenges;
mod words;

use askama_axum::Template;
use axum::routing::get;
use challenges::challenges_router;
use std::sync::Arc;

use words::words_router;

use crate::AppState;

#[derive(Template)]
#[template(path = "pages/root.html")]
struct RootPageTemplate {}

pub fn app_router(state: Arc<AppState>) -> axum::Router {
    axum::Router::new()
        .route("/", get(root))
        .merge(words_router())
        .merge(challenges_router())
        .with_state(state)
}

async fn root() -> RootPageTemplate {
    RootPageTemplate {}
}
