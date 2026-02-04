use crate::presentation::http::response::GenericApiResponse;
use crate::presentation::http::{orders, products, users};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Rust Microservice Template API",
        version = "0.1.0",
        description = "Clean Architecture Template.\n\nFlow:\n1. Create User -> Get ID\n2. Create Product -> Get ID\n3. Create Order (requires UserID + ProductID)"
    ),
    paths(
        // Users
        users::routes::create_user,
        users::routes::get_user,
        users::routes::list_users,
        users::routes::delete_user,
        // Products
        products::routes::create_product,
        products::routes::get_product,
        products::routes::list_products,
        products::routes::update_metadata,
        products::routes::delete_product,
        // Orders
        orders::routes::create_order,
        orders::routes::get_order,
        orders::routes::list_orders,
    ),
    components(
        schemas(
            // Users
            users::dtos::CreateUserInput,
            users::dtos::UserOutput,
            // Products
            products::dtos::CreateProductInput,
            products::dtos::UpdateProductMetadataInput,
            products::dtos::ProductOutput,
            crate::domain::products::ProductStatus,
            crate::domain::products::ProductMetadata,
            // Orders
            orders::dtos::CreateOrderInput,
            orders::dtos::OrderOutput,
            // Response Wrapper
            GenericApiResponse<users::dtos::UserOutput>,
            GenericApiResponse<products::dtos::ProductOutput>,
            GenericApiResponse<orders::dtos::OrderOutput>
        )
    ),
    tags(
        (name = "Users", description = "User management"),
        (name = "Products", description = "Product management"),
        (name = "Orders", description = "Order processing")
    )
)]
pub struct ApiDoc;
