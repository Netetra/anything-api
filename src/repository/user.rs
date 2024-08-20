use sea_orm::{DatabaseConnection, DbErr, EntityTrait};

use crate::entity::user;

pub struct UserRepository {
    db: DatabaseConnection,
}

pub trait UserRepositoryTrait {
    async fn get_list(&self) -> Result<Vec<user::Model>, DbErr>;
}

impl UserRepository {
    pub fn new(db: DatabaseConnection) -> UserRepository {
        UserRepository { db }
    }
}

impl UserRepositoryTrait for UserRepository {
    async fn get_list(&self) -> Result<Vec<user::Model>, DbErr> {
        user::Entity::find().all(&self.db).await
    }
}
