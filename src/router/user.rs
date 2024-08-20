use std::sync::Arc;

use axum::Router;

use crate::state::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
}
