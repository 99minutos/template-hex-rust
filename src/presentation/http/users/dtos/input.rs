use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Deserialize, ToSchema, Validate)]
pub struct CreateUserInput {
    #[validate(length(min = 1, message = "Name cannot be empty"))]
    pub name: String,
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
}
