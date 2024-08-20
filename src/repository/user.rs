use sea_orm::DatabaseConnection;

pub struct UserRepository {
    db: DatabaseConnection,
}

pub trait UserRepositoryTrait {}

impl UserRepository {
    pub fn new(db: DatabaseConnection) -> UserRepository {
        UserRepository { db }
    }
}

impl UserRepositoryTrait for UserRepository {}
