use sea_orm::DatabaseConnection;

use crate::service::{greet::GreetService, user::UserService};

pub struct AppState {
    pub user: UserService,
    pub greet: GreetService,
}

impl AppState {
    pub fn new(db: DatabaseConnection) -> AppState {
        AppState {
            user: UserService::new(db.clone()),
            greet: GreetService::new(),
        }
    }
}
