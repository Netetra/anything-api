use std::sync::Arc;

use axum::extract::State;
use axum::response::IntoResponse;

use crate::state::AppState;

use crate::service::greet::GreetServiceTrait;

pub async fn hello(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let service = &state.clone().greet;
    service.hello_world().await
}

pub async fn morning(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let service = &state.clone().greet;
    service.good_morning().await
}

pub async fn noon(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let service = &state.clone().greet;
    service.good_afternoon().await
}

pub async fn night(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let service = &state.clone().greet;
    service.good_night().await
}
