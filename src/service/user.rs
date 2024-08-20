use sea_orm::{ActiveValue, DatabaseConnection, DbErr};

use crate::{
    entity::user,
    repository::user::{UserRepository, UserRepositoryTrait},
};

pub struct UserService {
    repo: UserRepository,
}

pub trait UserServiceTrait {
    async fn create_user(&self, model: user::ActiveModel) -> Result<(), DbErr>;
    async fn get_user_list(&self) -> Result<Vec<user::Model>, DbErr>;
}

impl UserService {
    pub fn new(db: DatabaseConnection) -> UserService {
        UserService {
            repo: UserRepository::new(db.clone()),
        }
    }
}

impl UserServiceTrait for UserService {
    async fn create_user(&self, model: user::ActiveModel) -> Result<(), DbErr> {
        self.repo.save(model).await?;
        Ok(())
    }
    async fn get_user_list(&self) -> Result<Vec<user::Model>, DbErr> {
        self.repo.get_all().await
    }
}
