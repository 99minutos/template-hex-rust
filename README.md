# Rust Microservice Template

This is a template for building Rust microservices using a layered architecture. It uses `axum` for the web server and `mongodb` for the database.

## Architecture

The project is organized into layers to separate concerns:

```text
      Request
         |
         v
+------------------+
|   Presentation   |  <-- Decodes HTTP requests, Validates input
+--------+---------+
         | Calls Service
         v
+--------+---------+      +----------------+
|   Application    |----->|     Domain     |
| (Business Logic) |      |    (Entities)  |
+--------+---------+      +-------^--------+
         | Calls Repo             |
         v                        |
+--------+---------+              |
|  Infrastructure  |--------------+
|   (Persistence)  |  Returns Entities
+------------------+
```

## Folder Structure

Here is where everything is located:

```text
src/
├── domain/               # Core business entities and logic (No external dependencies)
│   ├── users.rs
│   ├── products.rs
│   └── error.rs
├── application/          # Business logic and coordination (Services)
│   ├── users.rs
│   └── products.rs
├── infrastructure/       # Database access and external tools
│   ├── persistence/      # Repositories (Database operations)
│   │   ├── users.rs
│   │   └── mongo.rs
│   └── providers/        # External services (Redis, etc.)
├── presentation/         # API Layer
│   ├── http/
│   │   ├── users/        # Routes and DTOs (Data Transfer Objects)
│   │   ├── validation.rs # Input validation
│   │   └── response.rs   # API responses
│   ├── server.rs         # Server configuration
│   ├── state.rs          # Dependency Injection setup
│   └── openapi.rs        # API Documentation setup
├── config.rs             # Configuration loading
└── main.rs               # Application entry point
```

## How to Add a New Feature

Example: Adding a **Payments** feature.

### 1. Domain Layer (`src/domain/payments.rs`)

Define the data structure.

```rust
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Payment {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<bson::oid::ObjectId>,
    pub amount: f64,
}
```

### 2. Infrastructure Layer (`src/infrastructure/persistence/payments.rs`)

Create the repository to handle database operations.

```rust
pub struct PaymentRepository { collection: Collection<Payment> }

impl PaymentRepository {
    #[tracing::instrument(skip_all)]
    pub async fn create(&self, p: &Payment) -> Result<ObjectId> {
        let res = self.collection.insert_one(p).await?;
        Ok(res.inserted_id.as_object_id().unwrap_or_default())
    }
}
```

### 3. Application Layer (`src/application/payments.rs`)

Create the service to handle business logic.

```rust
pub struct PaymentService { repo: Arc<PaymentRepository> }

impl PaymentService {
    #[tracing::instrument(skip_all)]
    pub async fn create(&self, dto: CreatePaymentDto) -> Result<Payment, Error> {
        let mut payment = Payment {
            id: None,
            amount: dto.amount,
        };
        let id = self.repo.create(&payment).await?;
        payment.id = Some(id);
        Ok(payment)
    }
}
```

### 4. Presentation Layer (`src/presentation/http/payments/`)

Define input data (`dtos.rs`) and API routes (`routes.rs`).

```rust
// routes.rs
#[utoipa::path(...)]
#[tracing::instrument(skip_all)]
pub async fn create_payment(
    State(service): State<Arc<PaymentService>>, // Inject Service
    ValidatedJson(req): ValidatedJson<CreatePaymentDto>
) -> ...
```

### 5. Register Components

1.  Initialize Repository and Service in `src/main.rs`.
2.  Add the Service to `AppState` in `src/presentation/state.rs`.
3.  Register the new routes in `src/presentation/http/mod.rs`.

## Running the Project

1.  Copy the environment file:
    ```bash
    cp .env.example .env
    ```
2.  Run the application:
    ```bash
    cargo run
    ```

You can view the API documentation at: **`http://localhost:3000/swagger-ui`**
