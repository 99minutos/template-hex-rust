use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;
use crate::domain::users::User;

#[derive(Deserialize, ToSchema, Validate)]
pub struct CreateUserDto {
    #[validate(length(min = 1, message = "Name cannot be empty"))]
    pub name: String,
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
}

#[derive(Serialize, ToSchema)]
pub struct UserResponseDto {
    pub id: String,
    pub name: String,
    pub email: String,
    pub created_at: String,
    pub updated_at: String,
}

impl From<User> for UserResponseDto {
    fn from(user: User) -> Self {
        Self {
            id: user.id.unwrap().to_hex(),
            name: user.name,
            email: user.email,
            created_at: user.created_at.to_chrono().to_rfc3339(),
            updated_at: user.updated_at.to_chrono().to_rfc3339(),
        }
    }
}
