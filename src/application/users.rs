use crate::domain::error::{Error, Result};
use crate::domain::users::{User, UserId};
use crate::infrastructure::persistence::Pagination;
use crate::infrastructure::persistence::users::UsersRepository;
use std::sync::Arc;

#[derive(Clone)]
pub struct UsersService {
    repo: Arc<UsersRepository>,
}

impl UsersService {
    pub fn new(repo: Arc<UsersRepository>) -> Self {
        Self { repo }
    }

    #[tracing::instrument(skip_all, fields(%email))]
    pub async fn create_user(&self, name: &str, email: &str) -> Result<User> {
        if self.repo.find_by_email(email).await?.is_some() {
            return Err(Error::duplicate("User", "email", email));
        }

        let now = chrono::Utc::now();
        let mut user = User {
            id: None,
            name: name.to_string(),
            email: email.to_string(),
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
    pub async fn get_user(&self, id: &UserId) -> Result<User> {
        self.repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| Error::not_found("User", id.to_string()))
    }

    #[tracing::instrument(skip_all)]
    pub async fn list_users(&self, pagination: Pagination) -> Result<Vec<User>> {
        self.repo.find_all(pagination).await
    }

    #[tracing::instrument(skip_all, fields(%id, %email))]
    pub async fn update_user(&self, id: &UserId, name: &str, email: &str) -> Result<User> {
        let mut user = self.get_user(id).await?;

        // Business rule: cannot change email to one already in use
        if email != user.email {
            if self.repo.find_by_email(email).await?.is_some() {
                return Err(Error::duplicate("User", "email", email));
            }
        }

        user.name = name.to_string();
        user.email = email.to_string();
        user.updated_at = chrono::Utc::now();

        self.repo.update(id, &user).await?;

        tracing::info!("User updated");
        Ok(user)
    }

    #[tracing::instrument(skip_all, fields(%id))]
    pub async fn delete_user(&self, id: &UserId) -> Result<()> {
        let deleted = self.repo.delete(id).await?;
        if !deleted {
            return Err(Error::not_found("User", id.to_string()));
        }
        tracing::info!("User soft-deleted");
        Ok(())
    }
}
