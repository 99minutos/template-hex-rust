use crate::domain::error::{Error, Result};
use crate::{
    domain::users::User, infrastructure::persistence::users::UsersRepository,
    presentation::http::users::dtos::CreateUserDto,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct UsersService {
    repo: Arc<UsersRepository>,
}

impl UsersService {
    pub fn new(repo: Arc<UsersRepository>) -> Self {
        Self { repo }
    }

    #[tracing::instrument(skip_all)]
    pub async fn create_user(&self, dto: CreateUserDto) -> Result<User> {
        if self.repo.find_by_email(&dto.email).await?.is_some() {
            return Err(Error::duplicate("User", "email", &dto.email));
        }

        let mut user = User {
            id: None,
            name: dto.name,
            email: dto.email,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        let id = self.repo.create(&user).await?;
        user.id = Some(id);
        Ok(user)
    }

    #[tracing::instrument(skip_all)]
    pub async fn get_user(&self, id: &str) -> Result<User> {
        self.repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| Error::not_found("User", id))
    }

    #[tracing::instrument(skip_all)]
    pub async fn list_users(&self) -> Result<Vec<User>> {
        Ok(self.repo.find_all().await?)
    }

    #[tracing::instrument(skip_all)]
    pub async fn delete_user(&self, id: &str) -> Result<()> {
        let deleted = self.repo.delete(id).await?;
        if !deleted {
            return Err(Error::not_found("User", id));
        }
        Ok(())
    }
}
