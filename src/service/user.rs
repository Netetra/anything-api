use sea_orm::DatabaseConnection;

use crate::repository::user::UserRepository;

pub struct UserService {
    repo: UserRepository,
}

impl UserService {
    pub fn new(db: DatabaseConnection) -> UserService {
        UserService {
            repo: UserRepository::new(db.clone()),
        }
    }
}
