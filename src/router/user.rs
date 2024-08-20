use std::sync::Arc;

use axum::{routing::get, Router};

use crate::{handler::user::user_list, state::AppState};

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/", get(user_list))
}
