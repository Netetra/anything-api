use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait};

use crate::entity::user;

pub struct UserRepository {
    db: DatabaseConnection,
}

pub trait UserRepositoryTrait {
    async fn save(&self, model: user::ActiveModel) -> Result<(), DbErr>;
    async fn get_all(&self) -> Result<Vec<user::Model>, DbErr>;
}

impl UserRepository {
    pub fn new(db: DatabaseConnection) -> UserRepository {
        UserRepository { db }
    }
}

impl UserRepositoryTrait for UserRepository {
    async fn save(&self, model: user::ActiveModel) -> Result<(), DbErr> {
        let _ = model.insert(&self.db).await?;
        Ok(())
    }
    async fn get_all(&self) -> Result<Vec<user::Model>, DbErr> {
        user::Entity::find().all(&self.db).await
    }
}
