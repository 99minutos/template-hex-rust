# Rust Microservice Template

Production-ready Rust microservice using **DDD/Hexagonal Architecture** with strict layer isolation and **type-safe domain IDs**.

**Stack**: Rust 2024 · Axum 0.8 · MongoDB 3.x · Tokio · OpenTelemetry · utoipa (Swagger)

## Architecture

```text
      Request
         │
         ▼
┌────────────────┐   From<DTO> → Cmd
│  Presentation   │───────────────────┐
│  (HTTP/Axum)    │                   ▼
└───────┬────────┘            ┌──────────────┐
        │ uses                │  Application  │ ← owns Command structs
        │                     │  (Services)   │
        │                     └──────┬───────┘
        │                            │ uses
        ▼                            ▼
┌────────────────┐  From/TryFrom  ┌──────────┐
│ Infrastructure  │◄─────────────►│  Domain   │
│  (Documents +  │                │  (pure)   │
│   Repositories)│                └──────────┘
└───────┬────────┘
        │
        ▼
     MongoDB
```

### Dependency Rules (Enforced)

| Direction                                | Status       |
| ---------------------------------------- | ------------ |
| Presentation → Application → Domain      | ✅ Allowed   |
| Infrastructure ↔ Domain (via From/Into) | ✅ Allowed   |
| Domain → Infrastructure                  | ❌ Forbidden |
| Domain → Presentation                    | ❌ Forbidden |
| Application → Presentation               | ❌ Forbidden |

Verify with:

```bash
grep -r "use crate::infrastructure" src/domain/      # Must return 0 results
grep -r "use crate::presentation" src/application/   # Must return 0 results
grep -r "bson::\|mongodb::" src/domain/              # Must return 0 results
grep -rE "struct Create|struct Update" src/application/  # Must return 0 results (no command structs)
```

## Project Structure

```text
src/
├── domain/                          # ⚪ Core Business (ZERO external deps)
│   ├── {entity}.rs                  #   Entities + Marker + typed ID (DomainId<Marker>)
│   ├── values.rs                    #   DomainId<T> generic type-safe ID
│   ├── error.rs                     #   DomainError + Result<T> alias + helpers
│   └── mod.rs
│
├── application/                     # 🔵 Business Logic
│   ├── {entity}.rs                  #   Services (direct params, NO command structs)
│   └── mod.rs
│
├── infrastructure/                  # 🟢 External I/O
│   ├── persistence/
│   │   ├── {entity}/
│   │   │   ├── model.rs             #   {Entity}Document (BSON-aware)
│   │   │   ├── repository.rs        #   Collection<Document>, returns Domain entities
│   │   │   └── mod.rs
│   │   └── mod.rs                   #   Pagination struct
│   ├── providers/
│   │   ├── mongo.rs                 #   MongoProvider (connection)
│   │   ├── redis.rs                 #   RedisProvider
│   │   └── telemetry.rs             #   Tracing + OpenTelemetry + Stackdriver
│   └── serde/
│       └── chrono_bson.rs           #   ChronoAsBson (used ONLY by Documents)
│
├── presentation/                    # 🟡 API Layer
│   ├── http/
│   │   ├── {entity}/
│   │   │   ├── dtos/
│   │   │   │   ├── input.rs         #   Input DTOs (validation only, no From→Command)
│   │   │   │   ├── output.rs        #   Output DTOs + From<Entity> → Output
│   │   │   │   └── mod.rs
│   │   │   ├── routes.rs            #   Handlers: validate, build typed IDs, call service
│   │   │   └── mod.rs
│   │   ├── error.rs                 #   DomainError → HTTP status mapping
│   │   ├── response.rs              #   GenericApiResponse<T> with trace_id
│   │   └── validation.rs            #   ValidatedJson extractor
│   ├── server.rs                    #   Axum app + graceful shutdown
│   ├── state.rs                     #   AppState + FromRef impls
│   └── openapi.rs                   #   utoipa registry
│
├── config.rs                        #   Env loading (dotenvy + OnceLock)
└── main.rs                          #   DI wiring: Repo → Service → State → Server
```

## Key Design Decisions

### Type-Safe Domain IDs (`DomainId<T>`)

Every entity defines a **marker type** and a typed ID alias using `DomainId<T>`. This prevents accidentally passing a `UserId` where a `ProductId` is expected:

```rust
// src/domain/values.rs — generic type-safe ID
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DomainId<T> {
    id: String,
    _marker: PhantomData<T>,
}
// Serializes as plain string "abc123", not {"id": "abc123"}
// Implements: Deref<Target=str>, Display, AsRef<str>, From<String>
```

```rust
// src/domain/users.rs — each entity defines Marker + type alias
use crate::domain::values;

#[derive(Debug, Clone)]
pub struct UserMarker;
pub type UserId = values::DomainId<UserMarker>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Option<UserId>,           // Typed ID, NOT Option<String>
    pub name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}
```

Foreign keys are also typed — `Order` uses `UserId` and `ProductId`, not plain `String`:

```rust
pub struct Order {
    pub id: Option<OrderId>,
    pub user_id: UserId,       // Type-safe FK — can't mix with ProductId
    pub product_id: ProductId, // Type-safe FK
    // ...
}
```

### Persistence Models (Documents)

Each entity has a `{Entity}Document` in infrastructure that handles BSON serialization. Conversion between `DomainId<T>` and `ObjectId` happens here:

```rust
// src/infrastructure/persistence/users/model.rs
#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserDocument {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    #[serde_as(as = "Option<IfIsHumanReadable<serde_with::DisplayFromStr>>")]
    pub id: Option<ObjectId>,    // ← BSON ObjectId here
    // ...
}

impl From<User> for UserDocument { /* DomainId → ObjectId */ }
impl From<UserDocument> for User { /* ObjectId → DomainId */ }
```

Repositories return typed IDs and accept typed IDs:

```rust
pub async fn create(&self, user: &User) -> Result<UserId> { /* ... */ }
pub async fn find_by_id(&self, id: &UserId) -> Result<Option<User>> { /* ... */ }
```

### Direct Parameters, No Command Structs

Services accept **typed IDs and direct parameters** — no intermediate command structs. This keeps the application layer lean:

```rust
// src/application/users.rs — NO CreateUser struct
impl UsersService {
    // 2 params (≤6) → pass directly
    pub async fn create_user(&self, name: &str, email: &str) -> Result<User> { /* ... */ }

    // Typed ID ensures you can't pass a ProductId here
    pub async fn get_user(&self, id: &UserId) -> Result<User> { /* ... */ }
    pub async fn update_user(&self, id: &UserId, name: &str, email: &str) -> Result<User> { /* ... */ }
}

// For >6 params, group using an existing domain type:
impl ProductsService {
    pub async fn create_product(
        &self, name: &str, price: f64, stock: i32,
        metadata: ProductMetadata,  // ← existing domain type groups 4 fields
    ) -> Result<Product> { /* ... */ }
}
```

**Rule**: ≤6 params → pass directly. >6 params → group using an existing domain struct.

### Handlers: Typed IDs at the Boundary

Handlers build typed IDs from raw strings and pass DTO fields directly to services:

```rust
// src/presentation/http/users/routes.rs
pub async fn create_user(
    State(service): State<Arc<UsersService>>,
    ValidatedJson(input): ValidatedJson<CreateUserInput>,
) -> Result<GenericApiResponse<UserOutput>, ApiError> {
    let user = service.create_user(&input.name, &input.email).await?;  // direct params
    Ok(GenericApiResponse::success(user.into()))
}

pub async fn get_user(
    State(service): State<Arc<UsersService>>,
    Path(id): Path<String>,
) -> Result<GenericApiResponse<UserOutput>, ApiError> {
    let user_id = UserId::new(id);                     // String → typed ID at boundary
    let user = service.get_user(&user_id).await?;
    Ok(GenericApiResponse::success(user.into()))
}
```

**No `From<DTO> for Command`** — there are no command structs. Input DTOs only carry validation.

### Error Handling

Domain errors are **database-agnostic** — `Database(String)`, not `Database(#[from] mongodb::error::Error)`:

```rust
// Domain: no mongodb dependency
DomainError::Database(String)

// Infrastructure: explicit conversion in every repo method
self.collection.find_one(doc! { ... }).await
    .map_err(|e| Error::database(e.to_string()))?;
```

### Soft Deletes

All entities have `deleted_at: Option<DateTime<Utc>>`. Repositories:

- **Delete**: `$set: { deleted_at: now }` (never `delete_one`)
- **Query**: always filter `"deleted_at": { "$exists": false }`
- **Indexes**: include `deleted_at` as first key in compound indexes

### Pagination

All `find_all()` methods require a `Pagination` parameter:

```rust
let users = service.list_users(Pagination { page: 1, page_size: 20 }).await?;
```

### Output DTOs

`DomainId<T>` converts to `String` via `into_inner()` in output DTOs:

```rust
impl From<User> for UserOutput {
    fn from(user: User) -> Self {
        Self {
            id: user.id.map(|id| id.into_inner()).unwrap_or_default(),
            // ...
        }
    }
}
```

## How to Add a New Feature

Example: Adding **Payments**.

### 1. Domain (`src/domain/payments.rs`)

```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::domain::values;
use crate::domain::users::UserId;

#[derive(Debug, Clone)]
pub struct PaymentMarker;
pub type PaymentId = values::DomainId<PaymentMarker>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Payment {
    pub id: Option<PaymentId>,     // Typed ID
    pub user_id: UserId,           // Typed FK
    pub amount: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}
```

Register in `src/domain/mod.rs`.

### 2. Persistence Model + Repository

```bash
mkdir -p src/infrastructure/persistence/payments
```

- `model.rs` — `PaymentDocument` with `ObjectId`, `ChronoAsBson`, `From`/`TryFrom` conversions
- `repository.rs` — `Collection<PaymentDocument>`, CRUD + indexes + soft delete + pagination
- `mod.rs` — re-exports

Register in `src/infrastructure/persistence/mod.rs`.

### 3. Service (`src/application/payments.rs`)

```rust
// NO command structs — 2 params (≤6), pass directly with typed IDs

pub struct PaymentsService { repo: Arc<PaymentsRepository> }

impl PaymentsService {
    pub async fn create_payment(
        &self,
        user_id: &UserId,   // typed ID, not &str
        amount: f64,
    ) -> Result<Payment> {
        // Business rules, then persist
    }
}
```

Register in `src/application/mod.rs`.

### 4. API Layer

```bash
mkdir -p src/presentation/http/payments/dtos
```

- `dtos/input.rs` — `CreatePaymentInput` (validation only, no `From` → command)
- `dtos/output.rs` — `PaymentOutput` + `impl From<Payment> for PaymentOutput` (uses `id.into_inner()`)
- `routes.rs` — handlers build `UserId::new(req.user_id)`, call `service.create_payment(&user_id, req.amount)`

Register in `src/presentation/http/mod.rs`.

### 5. Wire Everything

In `src/main.rs`:

```rust
let payments_repo = Arc::new(PaymentsRepository::new(&db));
payments_repo.create_indexes().await?;
let payments_service = Arc::new(PaymentsService::new(payments_repo));
```

In `src/presentation/state.rs` — add to `AppState` + `impl FromRef`.

In `src/presentation/http/mod.rs`:

```rust
.nest("/payments", payments::routes::router())
```

In `src/presentation/openapi.rs` — add paths + schemas.

## Prerequisites

```bash
cargo install sccache
```

## Running

```bash
cp .env.example .env   # Configure environment variables
cargo run
```

### Required Environment Variables

| Variable       | Required | Default                  | Description                       |
| -------------- | -------- | ------------------------ | --------------------------------- |
| `SERVICE_NAME` | ✅       | —                        | Service identifier                |
| `PROJECT_ID`   | ✅       | —                        | GCP project ID                    |
| `MONGO_URL`    | ✅       | —                        | MongoDB connection string         |
| `MONGO_DB`     | ✅       | —                        | Database name                     |
| `PORT`         | ❌       | `3000`                   | HTTP listen port                  |
| `APP_ENV`      | ❌       | `DEV`                    | Environment (`DEV`, `STG`, `PRD`) |
| `REDIS_URL`    | ❌       | `redis://127.0.0.1:6379` | Redis connection string           |
| `DEBUG_LEVEL`  | ❌       | `info`                   | Log level                         |
| `CORS_ORIGINS` | ❌       | `*`                      | Comma-separated CORS origins      |

## API Documentation

Swagger UI available at: **`http://localhost:3000/swagger-ui`** (disabled in `PRD` environment).

## Deployment

The project includes a Cloud Build pipeline (`build/cloudbuild.yaml`) that builds a distroless Docker image and deploys to **Google Cloud Run**.

```bash
# Manual Docker build
docker build -f build/Dockerfile -t service .
```
