use std::sync::Arc;

use axum::{routing::post, Router};

use crate::{handler::auth::login, state::AppState};

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/login", post(login))
}
