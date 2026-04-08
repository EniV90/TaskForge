use argon2::password_hash::rand_core::OsRng;
use argon2::{
    Argon2, PasswordHasher, PasswordVerifier,
    password_hash::{PasswordHash, SaltString},
};
use glue::errors::{SchedulerServiceError, SchedulerServiceErrorStatus};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct NewUser {
    pub email: String,
    pub password: String,
    pub unique_id: String,
}

impl NewUser {
    fn new(email: String, password: String) -> Result<NewUser, SchedulerServiceError> {
        let unique_id = uuid::Uuid::new_v4().to_string();
        let salt = SaltString::generate(&mut OsRng);
        let argon2_hasher = Argon2::default();

        let hash = argon2_hasher
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| {
                SchedulerServiceError::new(
                    format!("Failed to hash password: {}", e),
                    SchedulerServiceErrorStatus::Unknown,
                )
            })?
            .to_string();
        Ok(NewUser {
            email,
            password: hash,
            unique_id,
        })
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub unique_id: String,
}

impl User {
    fn verify_password(&self, password: String) -> Result<bool, SchedulerServiceError> {
        let argon2_hasher = Argon2::default();
        let parsed_hash = PasswordHash::new(&self.password).map_err(|e| {
            SchedulerServiceError::new(
                format!("Failed to parse password hash: {}", e),
                SchedulerServiceErrorStatus::Unknown,
            )
        })?;
        let is_valid = argon2_hasher
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok();
        Ok(is_valid)
    }
}

// To avoid exposing the password of the user outside the auth, i implemeted the Trimmeduser

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, sqlx::FromRow)]
pub struct TrimmedUser {
    pub id: i32,
    pub email: String,
    pub unique_id: String,
}

impl From<User> for TrimmedUser {
    fn from(user: User) -> Self {
        TrimmedUser {
            id: user.id,
            email: user.email,
            unique_id: user.unique_id,
        }
    }
}
