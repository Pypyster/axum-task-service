use crate::structs::{user::User, user_role::UserRole};
use async_trait::async_trait;

pub struct NewUser {
    pub name: String,
    pub phone: String,
    pub role: UserRole,
    pub password_hash: String,
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create_user(&self, user: NewUser) -> Result<User, sqlx::Error>;
    async fn find_by_id(&self, id: i32) -> Result<Option<User>, sqlx::Error>;
    async fn find_by_phone(&self, phone: String) -> Result<Option<User>, sqlx::Error>;
    async fn get_all_users(&self) -> Result<Vec<User>, sqlx::Error>;
    async fn delete_user_by_id(&self, id: i32) -> Result<bool, sqlx::Error>;
}
