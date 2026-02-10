# Rust Microservice Template

Production-ready Rust microservice using **DDD/Hexagonal Architecture** with strict layer isolation.

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
```

## Project Structure

```text
src/
├── domain/                          # ⚪ Core Business (ZERO external deps)
│   ├── {entity}.rs                  #   Entities: String IDs, chrono dates, deleted_at
│   ├── error.rs                     #   DomainError + Result<T> alias + helpers
│   └── mod.rs
│
├── application/                     # 🔵 Business Logic
│   ├── {entity}.rs                  #   Command structs + Service impl
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
│   │   │   │   ├── input.rs         #   Input DTOs + From<DTO> → Command
│   │   │   │   ├── output.rs        #   Output DTOs + From<Entity> → Output
│   │   │   │   └── mod.rs
│   │   │   ├── routes.rs            #   Handlers + query params
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

### Pure Domain Layer

Domain entities use **only** standard Rust types — no `bson::ObjectId`, no `mongodb`, no `serde_with`:

```rust
// src/domain/users.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Option<String>,           // String, NOT ObjectId
    pub name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,    // chrono native, NOT bson::DateTime
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,  // Soft delete (mandatory)
}
```

### Persistence Models (Documents)

Each entity has a `{Entity}Document` in infrastructure that handles BSON serialization:

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

impl From<User> for UserDocument { /* String → ObjectId */ }
impl From<UserDocument> for User { /* ObjectId → String */ }
```

### Application Commands (Not DTOs)

Services define their **own input types**. They never import from Presentation:

```rust
// src/application/users.rs
#[derive(Debug, Clone)]
pub struct CreateUser {       // ← Application Command
    pub name: String,
    pub email: String,
}

impl UsersService {
    pub async fn create_user(&self, cmd: CreateUser) -> Result<User> { /* ... */ }
}
```

### DTO → Command Conversion

Presentation converts validated DTOs to application commands via `From`:

```rust
// src/presentation/http/users/dtos/input.rs
impl From<CreateUserInput> for CreateUser {
    fn from(dto: CreateUserInput) -> Self {
        Self { name: dto.name, email: dto.email }
    }
}

// src/presentation/http/users/routes.rs
pub async fn create_user(
    State(service): State<Arc<UsersService>>,
    ValidatedJson(input): ValidatedJson<CreateUserInput>,
) -> Result<GenericApiResponse<UserOutput>, ApiError> {
    let user = service.create_user(input.into()).await?;  // DTO.into() → Command
    Ok(GenericApiResponse::success(user.into()))           // Entity.into() → Output
}
```

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

## How to Add a New Feature

Example: Adding **Payments**.

### 1. Domain (`src/domain/payments.rs`)

```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Payment {
    pub id: Option<String>,
    pub user_id: String,
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

### 3. Service + Commands (`src/application/payments.rs`)

```rust
#[derive(Debug, Clone)]
pub struct CreatePayment {
    pub user_id: String,
    pub amount: f64,
}

pub struct PaymentsService { repo: Arc<PaymentsRepository> }

impl PaymentsService {
    pub async fn create_payment(&self, cmd: CreatePayment) -> Result<Payment> {
        // Business rules, then persist
    }
}
```

Register in `src/application/mod.rs`.

### 4. API Layer

```bash
mkdir -p src/presentation/http/payments/dtos
```

- `dtos/input.rs` — `CreatePaymentInput` + `impl From<CreatePaymentInput> for CreatePayment`
- `dtos/output.rs` — `PaymentOutput` + `impl From<Payment> for PaymentOutput`
- `routes.rs` — handlers with `ValidatedJson`, pagination via `Query<PaymentQuery>`

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
