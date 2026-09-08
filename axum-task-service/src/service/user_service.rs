use std::sync::Arc;

use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};

use crate::{
    errors::{
        DomainError,
        ServiceError::{self, Repo},
    },
    repository::user_repository::{NewUser, UserRepository},
    structs::{
        claims::Claims,
        user::{LoginRequest, RegisterRequest, User},
    },
};

const TOKEN_LIFETIME_HOURS: i64 = 24;

pub struct UserService {
    repo: Arc<dyn UserRepository>,
    jwt_secret: Arc<str>,
}

impl UserService {
    pub fn new(repo: Arc<dyn UserRepository>, jwt_secret: String) -> Self {
        Self {
            repo,
            jwt_secret: Arc::from(jwt_secret),
        }
    }

    pub async fn get_user_by_id(&self, id: i32) -> Result<User, ServiceError> {
        self.repo
            .find_by_id(id)
            .await?
            .ok_or(ServiceError::NotFound {
                resource: "user".into(),
                id,
            })
    }

    pub async fn get_user_by_phone(&self, phone: String) -> Result<User, ServiceError> {
        self.repo
            .find_by_phone(phone.clone())
            .await?
            .ok_or(ServiceError::NotFoundPhone {
                resource: "user".into(),
                phone,
            })
    }

    pub async fn create_user(&self, user: RegisterRequest) -> Result<User, ServiceError> {
        if user.name.trim().is_empty() {
            return Err(DomainError::EmptyUserName.into());
        }

        if user.phone.trim().is_empty() {
            return Err(DomainError::EmptyPhone.into());
        }

        if user.password.len() < 8 {
            return Err(DomainError::PasswordTooShort.into());
        }

        let existing_user = self.repo.find_by_phone(user.phone.clone()).await?;

        if existing_user.is_some() {
            return Err(DomainError::PhoneAlreadyExists.into());
        }

        let password_hash = hash_password(user.password).await?;

        let new_user = NewUser {
            name: user.name,
            phone: user.phone,
            role: user.role,
            password_hash,
        };

        match self.repo.create_user(new_user).await {
            Ok(created_user) => Ok(created_user),

            Err(sqlx::Error::Database(db_error)) if db_error.code().as_deref() == Some("23505") => {
                Err(DomainError::PhoneAlreadyExists.into())
            }

            Err(error) => Err(ServiceError::Repo(error)),
        }
    }

    pub async fn login(&self, request: LoginRequest) -> Result<String, ServiceError> {
        let user = self
            .repo
            .find_by_phone(request.phone)
            .await?
            .ok_or(DomainError::InvalidCredentials)?;

        let is_password_valid =
            verify_password(request.password, user.password_hash.clone()).await?;

        if !is_password_valid {
            return Err(DomainError::InvalidCredentials.into());
        }

        self.create_token(&user)
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>, ServiceError> {
        Ok(self.repo.get_all_users().await?)
    }

    pub async fn delete_user_by_id(&self, id: i32) -> Result<(), ServiceError> {
        let deleted = self.repo.delete_user_by_id(id).await?;

        if !deleted {
            return Err(ServiceError::NotFound {
                resource: "user".into(),
                id,
            });
        }

        Ok(())
    }

    fn create_token(&self, user: &User) -> Result<String, ServiceError> {
        let exp = (Utc::now() + Duration::hours(TOKEN_LIFETIME_HOURS)).timestamp() as usize;

        let claims = Claims {
            sub: user.id,
            role: user.role.clone(),
            exp,
        };

        let header = Header::new(Algorithm::HS256);

        let token = encode(
            &header,
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )
        .map_err(|error| {
            tracing::error!("JWT create error: {error}");

            DomainError::Unauthorized
        })?;

        Ok(token)
    }
}

async fn hash_password(password: String) -> Result<String, ServiceError> {
    let password_hash =
        tokio::task::spawn_blocking(move || bcrypt::hash(password, bcrypt::DEFAULT_COST))
            .await
            .map_err(|error| {
                tracing::error!("Bcrypt task join error: {error}");

                DomainError::Unauthorized
            })?
            .map_err(|error| {
                tracing::error!("Bcrypt hashing error: {error}");

                DomainError::Unauthorized
            })?;

    Ok(password_hash)
}

async fn verify_password(password: String, password_hash: String) -> Result<bool, ServiceError> {
    let is_valid = tokio::task::spawn_blocking(move || bcrypt::verify(password, &password_hash))
        .await
        .map_err(|error| {
            tracing::error!("Bcrypt verify task join error: {error}");

            DomainError::InvalidCredentials
        })?
        .map_err(|error| {
            tracing::error!("Bcrypt verify error: {error}");

            DomainError::InvalidCredentials
        })?;

    Ok(is_valid)
}
