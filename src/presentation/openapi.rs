use crate::presentation::http::response::GenericApiResponse;
use crate::presentation::http::{orders, products, users};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Rust Microservice Template API",
        version = "0.1.0",
        description = "Vertical Slice Architecture Demo.\n\nFlow:\n1. Create User -> Get ID\n2. Create Product -> Get ID\n3. Create Order (requires UserID + ProductID)"
    ),
    paths(
        // Users
        users::routes::create_user,
        users::routes::get_user,
        users::routes::list_users,
        users::routes::delete_user,
        // Products
        products::routes::create_product,
        products::routes::list_products,
        products::routes::update_product_metadata,
        // Orders
        orders::routes::create_order,
    ),
    components(
        schemas(
            // Users
            users::dtos::CreateUserDto,
            users::dtos::UserResponseDto,
            // Products
            products::dtos::CreateProductDto,
            products::dtos::UpdateProductMetadataDto,
            products::dtos::ProductResponseDto,
            crate::domain::products::ProductStatus,
            crate::domain::products::ProductMetadata,
            crate::presentation::http::response::MessageResponseDto,
            // Orders
            orders::dtos::CreateOrderDto,
            orders::dtos::OrderResponseDto,
            // Generics
            GenericApiResponse<users::dtos::UserResponseDto>,
            GenericApiResponse<Vec<users::dtos::UserResponseDto>>,
            GenericApiResponse<products::dtos::ProductResponseDto>,
            GenericApiResponse<Vec<products::dtos::ProductResponseDto>>,
            GenericApiResponse<crate::presentation::http::response::MessageResponseDto>,
            GenericApiResponse<orders::dtos::OrderResponseDto>,
        )
    ),
    tags(
        (name = "Users", description = "User management"),
        (name = "Products", description = "Product management"),
        (name = "Orders", description = "Order processing (Depends on User + Product)")
    )
)]
pub struct ApiDoc;
