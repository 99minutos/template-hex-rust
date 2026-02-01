use crate::domain::error::Error;
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
    pub async fn create_user(&self, dto: CreateUserDto) -> Result<User, Error> {
        // Business Logic: Validate Uniqueness
        if self.repo.find_by_email(&dto.email).await?.is_some() {
            return Err(Error::Conflict(format!(
                "User with email {} already exists",
                dto.email
            )));
        }

        let mut user = User {
            id: None,
            name: dto.name,
            email: dto.email,
            created_at: bson::DateTime::now(),
            updated_at: bson::DateTime::now(),
        };

        let id = self.repo.create(&user).await?;
        user.id = Some(id);
        Ok(user)
    }

    #[tracing::instrument(skip_all)]
    pub async fn get_user(&self, id: &str) -> Result<User, Error> {
        self.repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| Error::NotFound(format!("User {} not found", id)))
    }

    #[tracing::instrument(skip_all)]
    pub async fn list_users(&self) -> Result<Vec<User>, Error> {
        Ok(self.repo.find_all().await?)
    }

    #[tracing::instrument(skip_all)]
    pub async fn delete_user(&self, id: &str) -> Result<(), Error> {
        let deleted = self.repo.delete(id).await?;
        if !deleted {
            return Err(Error::NotFound(format!("User {} not found", id)));
        }
        Ok(())
    }
}
