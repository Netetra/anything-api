use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;

use crate::{
    service::{auth::AuthServiceTrait, user::UserServiceTrait},
    state::AppState,
};

#[derive(Deserialize)]
pub struct LoginPayload {
    name: String,
    password: String,
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginPayload>,
) -> impl IntoResponse {
    let user = state.user.find_by_name(&payload.name).await;

    match user {
        Ok(Some(user)) => {
            let result = state
                .auth
                .verify_password(&payload.password, &user.password)
                .await;
            match result {
                Ok(_) => (StatusCode::OK).into_response(),
                Err(_) => (StatusCode::UNAUTHORIZED).into_response(),
            }
        }
        Ok(None) => (StatusCode::NOT_FOUND).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    }
}
