use std::sync::Arc;

use sea_orm::DbErr;

use crate::{
    entity::user, repository::user::UserRepository, repository::user::UserRepositoryTrait,
};

pub struct UserService {
    user_repo: Arc<UserRepository>,
}

pub trait UserServiceTrait {
    async fn create_user(&self, model: user::ActiveModel) -> Result<(), DbErr>;
    async fn get_user_list(&self) -> Result<Vec<user::Model>, DbErr>;
    async fn find_by_name(&self, name: &str) -> Result<Option<user::Model>, DbErr>;
}

impl UserService {
    pub fn new(user_repo: Arc<UserRepository>) -> UserService {
        UserService { user_repo }
    }
}

impl UserServiceTrait for UserService {
    async fn create_user(&self, model: user::ActiveModel) -> Result<(), DbErr> {
        self.user_repo.save(model).await
    }
    async fn get_user_list(&self) -> Result<Vec<user::Model>, DbErr> {
        self.user_repo.get_all().await
    }
    async fn find_by_name(&self, name: &str) -> Result<Option<user::Model>, DbErr> {
        self.user_repo.find_by_name(name).await
    }
}
