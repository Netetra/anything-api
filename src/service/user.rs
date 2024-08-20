use sea_orm::{ActiveValue, DatabaseConnection, DbErr};

use crate::{
    entity::user,
    repository::user::{UserRepository, UserRepositoryTrait},
};

pub struct UserService {
    repo: UserRepository,
}

pub trait UserServiceTrait {
    async fn register(&self, name: String, password: String) -> Result<(), DbErr>;
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
    async fn register(&self, name: String, password: String) -> Result<(), DbErr> {
        let model = user::ActiveModel {
            id: ActiveValue::NotSet,
            name: ActiveValue::Set(name),
            password: ActiveValue::Set(password),
        };
        self.repo.register(model).await?;
        Ok(())
    }
    async fn get_list(&self) -> Result<Vec<user::Model>, DbErr> {
        self.repo.get_list().await
    }
}
