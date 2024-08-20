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
    let user = match state.user.find_by_name(&payload.name).await {
        Ok(Some(user)) => user,
        Ok(None) => return (StatusCode::NOT_FOUND).into_response(),
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    };

    if (state
        .auth
        .verify_password(&payload.password, &user.password)
        .await)
        .is_err()
    {
        return (StatusCode::UNAUTHORIZED).into_response();
    }

    (StatusCode::OK).into_response()
}
