use std::{net::SocketAddrV4, sync::Arc};

use axum::Router;
use migration::{Migrator, MigratorTrait};
use router::greet;
use sea_orm::Database;
use setting::read_settings;
use state::AppState;
use tokio::net::TcpListener;

mod handler;
mod repository;
mod router;
mod service;
mod setting;
mod state;

#[tokio::main]
async fn main() {
    let settings = read_settings("config").unwrap();

    let db = Database::connect(settings.database.url).await.unwrap();
    Migrator::up(&db, None).await.unwrap();

    let state = Arc::new(AppState::new(db));
    let router = Router::new()
        .nest("/greet", greet::router())
        .with_state(state);

    let addr = format!("{}:{}", settings.server.ip, settings.server.port);
    let socket_addr: SocketAddrV4 = addr.clone().parse().unwrap();
    let listener = TcpListener::bind(socket_addr).await.unwrap();

    axum::serve(listener, router).await.unwrap();
}
