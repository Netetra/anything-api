use std::sync::Arc;

use axum::{routing::get, Router};

use crate::{
    handler::greet::{hello, morning, night, noon},
    state::AppState,
};

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/hello", get(hello))
        .route("/morning", get(morning))
        .route("/noon", get(noon))
        .route("/night", get(night))
}
