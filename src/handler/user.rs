use std::sync::Arc;

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;

use crate::service::user::UserServiceTrait;
use crate::state::AppState;

pub async fn user_list(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let service = &state.clone().user;
    match service.get_list().await {
        Ok(users) => (StatusCode::OK, Json(users)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    }
}
