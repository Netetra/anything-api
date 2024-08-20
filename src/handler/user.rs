use std::sync::Arc;

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use sea_orm::ActiveValue;
use serde::Deserialize;

use crate::entity::user;
use crate::service::user::UserServiceTrait;
use crate::state::AppState;

pub async fn user_list(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let service = &state.clone().user;
    match service.get_user_list().await {
        Ok(users) => (StatusCode::OK, Json(users)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    }
}

#[derive(Deserialize)]
pub struct RegisterPayload {
    name: String,
    password: String,
}

pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RegisterPayload>,
) -> impl IntoResponse {
    let service = &state.clone().user;

    let model = user::ActiveModel {
        id: ActiveValue::NotSet,
        name: ActiveValue::Set(payload.name),
        password: ActiveValue::Set(payload.password),
    };

    match service.create_user(model).await {
        Ok(_) => (StatusCode::OK).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    }
}
