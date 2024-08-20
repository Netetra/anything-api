use std::sync::Arc;

use sea_orm::DatabaseConnection;

use crate::{
    repository::user::UserRepository,
    service::{auth::AuthService, greet::GreetService, user::UserService},
};

pub struct AppState {
    pub auth: AuthService,
    pub user: UserService,
    pub greet: GreetService,
}

impl AppState {
    pub fn new(db: DatabaseConnection) -> AppState {
        let user_repo = Arc::new(UserRepository::new(db.clone()));
        AppState {
            auth: AuthService::new(),
            user: UserService::new(user_repo.clone()),
            greet: GreetService::new(),
        }
    }
}
