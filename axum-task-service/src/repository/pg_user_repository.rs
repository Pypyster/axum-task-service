use async_trait::async_trait;
use sqlx::PgPool;

use crate::{
    db::user::{create_user, delete_user_by_id, find_user_by_id, find_user_by_phone, list_users},
    repository::user_repository::{NewUser, UserRepository},
    structs::user::User,
};

pub struct PgUserRepos {
    pool: PgPool,
}

impl PgUserRepos {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PgUserRepos {
    async fn create_user(&self, user: NewUser) -> Result<User, sqlx::Error> {
        create_user(
            &self.pool,
            user.name,
            user.phone,
            user.role,
            user.password_hash,
        )
        .await
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<User>, sqlx::Error> {
        find_user_by_id(&self.pool, id).await
    }

    async fn find_by_phone(&self, phone: String) -> Result<Option<User>, sqlx::Error> {
        find_user_by_phone(&self.pool, phone).await
    }

    async fn get_all_users(&self) -> Result<Vec<User>, sqlx::Error> {
        list_users(&self.pool).await
    }

    async fn delete_user_by_id(&self, id: i32) -> Result<bool, sqlx::Error> {
        delete_user_by_id(&self.pool, id).await
    }
}
