use crate::domain::error::{Error, Result};
use crate::domain::users::User;
use crate::infrastructure::persistence::Pagination;
use crate::infrastructure::persistence::users::UsersRepository;
use std::sync::Arc;

// ===== Application Commands =====

#[derive(Debug, Clone)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Clone)]
pub struct UpdateUser {
    pub name: String,
    pub email: String,
}

// ===== Service =====

#[derive(Clone)]
pub struct UsersService {
    repo: Arc<UsersRepository>,
}

impl UsersService {
    pub fn new(repo: Arc<UsersRepository>) -> Self {
        Self { repo }
    }

    #[tracing::instrument(skip_all, fields(email = %cmd.email))]
    pub async fn create_user(&self, cmd: CreateUser) -> Result<User> {
        if self.repo.find_by_email(&cmd.email).await?.is_some() {
            return Err(Error::duplicate("User", "email", &cmd.email));
        }

        let now = chrono::Utc::now();
        let mut user = User {
            id: None,
            name: cmd.name,
            email: cmd.email,
            created_at: now,
            updated_at: now,
            deleted_at: None,
        };

        let id = self.repo.create(&user).await?;
        user.id = Some(id);

        tracing::info!(user_id = %user.id.as_deref().unwrap_or("unknown"), "User created");
        Ok(user)
    }

    #[tracing::instrument(skip_all, fields(%id))]
    pub async fn get_user(&self, id: &str) -> Result<User> {
        self.repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| Error::not_found("User", id))
    }

    #[tracing::instrument(skip_all)]
    pub async fn list_users(&self, pagination: Pagination) -> Result<Vec<User>> {
        self.repo.find_all(pagination).await
    }

    #[tracing::instrument(skip_all, fields(%id))]
    pub async fn update_user(&self, id: &str, cmd: UpdateUser) -> Result<User> {
        let mut user = self.get_user(id).await?;

        // Business rule: cannot change email to one already in use
        if cmd.email != user.email {
            if self.repo.find_by_email(&cmd.email).await?.is_some() {
                return Err(Error::duplicate("User", "email", &cmd.email));
            }
        }

        user.name = cmd.name;
        user.email = cmd.email;
        user.updated_at = chrono::Utc::now();

        self.repo.update(id, &user).await?;

        tracing::info!("User updated");
        Ok(user)
    }

    #[tracing::instrument(skip_all, fields(%id))]
    pub async fn delete_user(&self, id: &str) -> Result<()> {
        let deleted = self.repo.delete(id).await?;
        if !deleted {
            return Err(Error::not_found("User", id));
        }
        tracing::info!("User soft-deleted");
        Ok(())
    }
}
