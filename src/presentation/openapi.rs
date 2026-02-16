use crate::domain::product::{ProductMetadata, ProductStatus};
use crate::presentation::http::error::ApiError;
use crate::presentation::http::{order, product, user};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        user::routes::create_user,
        user::routes::get_user,
        user::routes::list_users,
        user::routes::delete_user,
        // Products
        product::routes::create_product,
        product::routes::get_product,
        product::routes::list_products,
        product::routes::update_metadata,
        product::routes::delete_product,
        // Orders
        order::routes::create_order,
        order::routes::get_order,
        order::routes::list_orders,
    ),
    components(
        schemas(
            user::dtos::CreateUserInput,
            user::dtos::UserOutput,
            product::dtos::CreateProductInput,
            product::dtos::ProductOutput,
            product::dtos::UpdateProductMetadataInput,
            order::dtos::CreateOrderInput,
            order::dtos::OrderOutput,
            ApiError,
            ProductStatus,
            ProductMetadata,
        )
    ),
    tags(
        (name = "Users", description = "User management endpoints"),
        (name = "Products", description = "Product management endpoints"),
        (name = "Orders", description = "Order management endpoints")
    )
)]
pub struct ApiDoc;
