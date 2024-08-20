use sea_orm::{DatabaseConnection, DbErr};

use crate::{
    entity::user,
    repository::user::{UserRepository, UserRepositoryTrait},
};

pub struct UserService {
    repo: UserRepository,
}

pub trait UserServiceTrait {
    async fn get_list(&self) -> Result<Vec<user::Model>, DbErr>;
}

impl UserService {
    pub fn new(db: DatabaseConnection) -> UserService {
        UserService {
            repo: UserRepository::new(db.clone()),
        }
    }
}

impl UserServiceTrait for UserService {
    async fn get_list(&self) -> Result<Vec<user::Model>, DbErr> {
        self.repo.get_list().await
    }
}
