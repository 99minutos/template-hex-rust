# Rust Microservice Template

Production-ready Rust microservice template — **DDD/Hexagonal Architecture** (Ports & Adapters), strict layer isolation, type-safe domain IDs, distributed tracing, and auto-generated OpenAPI docs.

**Stack**: Rust 2024 · Axum 0.8 · MongoDB 3.x · Tokio · OpenTelemetry · Stackdriver · utoipa (Swagger)

---

## Table of Contents

- [Quick Start](#quick-start)
- [Architecture](#architecture)
- [Project Structure](#project-structure)
- [Layer by Layer](#layer-by-layer)
  - [1. Domain Entity](#1-domain-entity)
  - [2. Domain Port](#2-domain-port)
  - [3. Persistence Model (Document)](#3-persistence-model-document)
  - [4. Repository (Port Implementation)](#4-repository-port-implementation)
  - [5. Service (Application Layer)](#5-service-application-layer)
  - [6. DTOs (Input / Output)](#6-dtos-input--output)
  - [7. Routes (Handlers)](#7-routes-handlers)
  - [8. Wiring](#8-wiring)
- [Dependency Rules](#dependency-rules)
- [Naming Conventions](#naming-conventions)
- [Error Handling](#error-handling)
- [Observability & Telemetry](#observability--telemetry)
- [Patterns](#patterns)
- [Environment Variables](#environment-variables)
- [API Documentation](#api-documentation)
- [Deployment](#deployment)

---

## Quick Start

```bash
# Prerequisites
cargo install sccache          # compile cache (required by Cargo.toml)
# MongoDB running locally or remotely

# Setup
git clone <this-repo> my-service && cd my-service
cp .env.example .env           # fill in MONGO_URL, MONGO_DB, SERVICE_NAME, PROJECT_ID

# Run
cargo run                      # http://localhost:3000
                               # Swagger UI → http://localhost:3000/swagger-ui (non-PRD only)
```

After cloning, remove the example entities included in the template and start fresh with your own domain following the guide below.

---

## Architecture

```text
                                HTTP Request
                                     │
                                     ▼
┌────────────────────────────────────────────────────────────────┐
│  PRESENTATION                                                  │
│  Axum · DTOs · ValidatedJson · OpenAPI                         │
└───────────────────┬────────────────────────────────────────────┘
                    │  direct params (&str, &EntityId)
                    ▼
┌────────────────────────────────────────────────────────────────┐
│  APPLICATION                                                   │
│  Services · Business Rules · #[tracing::instrument]            │
└───────────────────┬────────────────────────────────────────────┘
                    │  DomainResult<T>
                    ▼
┌────────────────────────────────────────────────────────────────┐
│  DOMAIN  ⚠ pure Rust — zero external deps                      │
│  Entities · DomainId<T> · Ports (traits) · DomainError         │
└────────────────────────────────▲───────────────────────────────┘
                                 │  implements Ports
                                 │  From<Entity> ↔ From<Document>
                                 │
┌────────────────────────────────┴───────────────────────────────┐
│  INFRASTRUCTURE                                                │
│  Repositories · Documents · Providers · Telemetry              │
└───────────────┬────────────────────────────────┬───────────────┘
                │                                │
                ▼                                ▼
┌───────────────────────────┐  ┌─────────────────────────────────┐
│  MongoDB / Redis          │  │  GCP                            │
│                           │  │  Cloud Trace · Cloud Logging    │
└───────────────────────────┘  └─────────────────────────────────┘
```

### Layer Responsibilities

| Layer              | Owns                                     | Depends On              | NEVER imports                                    |
| ------------------ | ---------------------------------------- | ----------------------- | ------------------------------------------------ |
| **Domain**         | Entities, Ports (traits), Errors, Values | Nothing (pure Rust)     | `infrastructure`, `presentation`, `bson`, `axum` |
| **Application**    | Services, business orchestration         | `domain`                | `infrastructure`, `presentation`                 |
| **Infrastructure** | Documents, Repositories, Providers       | `domain`                | `application`, `presentation`                    |
| **Presentation**   | DTOs, Handlers, Server, OpenAPI          | `domain`, `application` | `infrastructure`                                 |

---

## Project Structure

```text
src/
├── domain/                          # ⚪ Core — ZERO external deps
│   ├── ports/                       #   🔌 Repository traits (interfaces)
│   │   ├── {entity}.rs              #     trait {Entity}RepositoryPort
│   │   └── mod.rs
│   ├── {entity}.rs                  #   Entity + Marker + typed ID
│   ├── values.rs                    #   DomainId<T> generic type-safe ID
│   ├── error.rs                     #   DomainError enum + DomainResult<T>
│   ├── pagination.rs                #   Shared Pagination struct
│   └── mod.rs
│
├── application/                     # 🔵 Business Logic
│   ├── {entity}.rs                  #   Service (Arc<dyn Port>)
│   └── mod.rs
│
├── infrastructure/                  # 🟢 External I/O
│   ├── persistence/
│   │   ├── {entity}/                #   Singular folder
│   │   │   ├── model.rs             #     {Entity}Document (BSON-aware)
│   │   │   ├── repository.rs        #     impl {Entity}RepositoryPort
│   │   │   └── mod.rs
│   │   └── mod.rs
│   ├── providers/
│   │   ├── mongo.rs                 #   MongoProvider (connection + ping)
│   │   ├── redis.rs                 #   RedisProvider (optional)
│   │   └── telemetry.rs             #   Tracing + OpenTelemetry + Stackdriver
│   ├── serde/
│   │   └── chrono_bson.rs           #   ChronoAsBson (Documents only)
│   └── mod.rs
│
├── presentation/                    # 🟡 HTTP / API
│   ├── http/
│   │   ├── {entity}/                #   Singular folder
│   │   │   ├── dtos/
│   │   │   │   ├── input.rs         #     Request DTO + validation
│   │   │   │   ├── output.rs        #     Response DTO + From<Entity>
│   │   │   │   └── mod.rs
│   │   │   ├── routes.rs            #     Axum handlers
│   │   │   └── mod.rs
│   │   ├── error.rs                 #   ApiError ← DomainError mapping
│   │   ├── response.rs              #   GenericApiResponse<T> + trace_id
│   │   ├── validation.rs            #   ValidatedJson extractor
│   │   └── mod.rs                   #   app_router() — nests entity routes
│   ├── server.rs                    #   Axum server + graceful shutdown
│   ├── state.rs                     #   AppState + FromRef
│   └── openapi.rs                   #   utoipa OpenAPI registry
│
├── config.rs                        #   Env loading (dotenvy + OnceLock)
└── main.rs                          #   DI wiring: Repo → Service → State → Server
```

---

## Layer by Layer

Below is the exact recipe for adding any entity to the template. Replace `{Entity}` with your domain concept (e.g., `Invoice`, `Tenant`, `Shipment`).

### 1. Domain Entity

**File**: `src/domain/{entity}.rs`

```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::domain::values;

// Marker for type-safe ID
#[derive(Debug, Clone)]
pub struct {Entity}Marker;
pub type {Entity}Id = values::DomainId<{Entity}Marker>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct {Entity} {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<{Entity}Id>,
    // ... your fields here (ONLY what is explicitly needed) ...
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<DateTime<Utc>>,
}
```

`DomainId<T>` serializes as a plain string, implements `Deref<Target=str>`, `Display`, `From<String>`, `From<&str>`. Foreign keys to other entities use their typed ID (e.g., `customer_id: CustomerId`) — **never** a raw `String`.

**Register**: add `pub mod {entity};` in `src/domain/mod.rs`.

---

### 2. Domain Port

**File**: `src/domain/ports/{entity}.rs`

```rust
use crate::domain::error::DomainResult;
use crate::domain::pagination::Pagination;
use crate::domain::{entity}::{{Entity}, {Entity}Id};
use async_trait::async_trait;

#[async_trait]
pub trait {Entity}RepositoryPort: Send + Sync {
    async fn create(&self, entity: &{Entity}) -> DomainResult<{Entity}Id>;
    async fn find_by_id(&self, id: &{Entity}Id) -> DomainResult<Option<{Entity}>>;
    async fn find_all(&self, pagination: Pagination) -> DomainResult<Vec<{Entity}>>;
    async fn update(&self, id: &{Entity}Id, entity: &{Entity}) -> DomainResult<bool>;
    async fn delete(&self, id: &{Entity}Id) -> DomainResult<bool>;
    async fn count(&self) -> DomainResult<u64>;
    // Add only the queries your domain actually needs
}
```

**Rules**:

- Trait names **MUST** end with `Port`.
- Only create ports for **Aggregate Roots**, not every struct.
- Return `DomainResult<T>` — never `mongodb::error::Error`.

**Register**: add `pub mod {entity};` in `src/domain/ports/mod.rs`.

---

### 3. Persistence Model (Document)

**File**: `src/infrastructure/persistence/{entity}/model.rs`

```rust
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use crate::domain::{entity}::{{Entity}, {Entity}Id};

#[derive(Debug, Serialize, Deserialize)]
pub struct {Entity}Document {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    // ... mirror entity fields but with BSON types ...
    pub created_at: bson::DateTime,
    pub updated_at: bson::DateTime,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<bson::DateTime>,
}

// Domain → Document (for writes)
impl From<{Entity}> for {Entity}Document {
    fn from(e: {Entity}) -> Self {
        Self {
            id: e.id.as_ref().and_then(|id| ObjectId::parse_str(&**id).ok()),
            // ... map fields ...
            created_at: bson::DateTime::from_chrono(e.created_at),
            updated_at: bson::DateTime::from_chrono(e.updated_at),
            deleted_at: e.deleted_at.map(bson::DateTime::from_chrono),
        }
    }
}

// Document → Domain (for reads)
impl From<{Entity}Document> for {Entity} {
    fn from(doc: {Entity}Document) -> Self {
        Self {
            id: doc.id.map(|oid| {Entity}Id::new(oid.to_hex())),
            // ... map fields ...
            created_at: doc.created_at.to_chrono(),
            updated_at: doc.updated_at.to_chrono(),
            deleted_at: doc.deleted_at.map(|dt| dt.to_chrono()),
        }
    }
}
```

All `DomainId<T>` ↔ `ObjectId` conversion happens **exclusively** in this file.

---

### 4. Repository (Port Implementation)

**File**: `src/infrastructure/persistence/{entity}/repository.rs`

```rust
use async_trait::async_trait;
use bson::{doc, oid::ObjectId};
use futures::stream::TryStreamExt;
use mongodb::{Collection, Database, IndexModel, options::IndexOptions};
use crate::domain::error::{DomainResult, Error};
use crate::domain::pagination::Pagination;
use crate::domain::ports::{entity}::{Entity}RepositoryPort;
use crate::domain::{entity}::{{Entity}, {Entity}Id};
use super::model::{Entity}Document;

#[derive(Clone)]
pub struct {Entity}Repository {
    collection: Collection<{Entity}Document>,
}

impl {Entity}Repository {
    pub fn new(db: &Database) -> Self {
        Self { collection: db.collection("{entities}") }  // plural for DB collection
    }

    /// Idempotent — safe to call on every startup.
    pub async fn create_indexes(&self) -> DomainResult<()> {
        let indexes = vec![
            IndexModel::builder()
                .keys(doc! { "deleted_at": 1, "created_at": -1 })
                .options(IndexOptions::builder().build())
                .build(),
            // ... add unique / compound indexes as needed ...
        ];
        self.collection.create_indexes(indexes).await
            .map_err(|e| Error::database(e.to_string()))?;
        Ok(())
    }
}

#[async_trait]
impl {Entity}RepositoryPort for {Entity}Repository {

    #[tracing::instrument(skip_all)]
    async fn create(&self, entity: &{Entity}) -> DomainResult<{Entity}Id> {
        let doc = {Entity}Document::from(entity.clone());
        let result = self.collection.insert_one(doc).await
            .map_err(|e| Error::database(e.to_string()))?;
        result.inserted_id.as_object_id()
            .map(|oid| {Entity}Id::new(oid.to_hex()))
            .ok_or_else(|| Error::internal("Failed to get inserted ID"))
    }

    #[tracing::instrument(skip_all)]
    async fn find_by_id(&self, id: &{Entity}Id) -> DomainResult<Option<{Entity}>> {
        let oid = ObjectId::parse_str(&**id)
            .map_err(|_| Error::invalid_param("id", "{Entity}", &**id))?;
        let doc = self.collection
            .find_one(doc! { "_id": oid, "deleted_at": { "$exists": false } })
            .await.map_err(|e| Error::database(e.to_string()))?;
        Ok(doc.map({Entity}::from))
    }

    #[tracing::instrument(skip_all)]
    async fn find_all(&self, pagination: Pagination) -> DomainResult<Vec<{Entity}>> {
        let cursor = self.collection
            .find(doc! { "deleted_at": { "$exists": false } })
            .skip(pagination.get_skip())
            .limit(pagination.get_limit())
            .sort(doc! { "created_at": -1 })
            .await.map_err(|e| Error::database(e.to_string()))?;
        let docs: Vec<{Entity}Document> = cursor.try_collect().await
            .map_err(|e| Error::database(e.to_string()))?;
        Ok(docs.into_iter().map({Entity}::from).collect())
    }

    #[tracing::instrument(skip_all)]
    async fn update(&self, id: &{Entity}Id, entity: &{Entity}) -> DomainResult<bool> {
        let oid = ObjectId::parse_str(&**id)
            .map_err(|_| Error::invalid_param("id", "{Entity}", &**id))?;
        let doc = {Entity}Document::from(entity.clone());
        let bson_doc = mongodb::bson::serialize_to_document(&doc)
            .map_err(|e| Error::internal(e.to_string()))?;
        let result = self.collection
            .update_one(
                doc! { "_id": oid, "deleted_at": { "$exists": false } },
                doc! { "$set": bson_doc },
            ).await.map_err(|e| Error::database(e.to_string()))?;
        Ok(result.matched_count > 0)
    }

    #[tracing::instrument(skip_all)]
    async fn delete(&self, id: &{Entity}Id) -> DomainResult<bool> {
        let oid = ObjectId::parse_str(&**id)
            .map_err(|_| Error::invalid_param("id", "{Entity}", &**id))?;
        let now = bson::DateTime::from_chrono(chrono::Utc::now());
        let result = self.collection
            .update_one(
                doc! { "_id": oid, "deleted_at": { "$exists": false } },
                doc! { "$set": { "deleted_at": now } },
            ).await.map_err(|e| Error::database(e.to_string()))?;
        Ok(result.matched_count > 0)
    }

    #[tracing::instrument(skip_all)]
    async fn count(&self) -> DomainResult<u64> {
        self.collection
            .count_documents(doc! { "deleted_at": { "$exists": false } })
            .await.map_err(|e| Error::database(e.to_string()))
    }
}
```

**Key patterns**:

- Struct name is `{Entity}Repository` — **never** `Mongo{Entity}Repository`.
- Every MongoDB call uses `.map_err(|e| Error::database(e.to_string()))`.
- Every `ObjectId::parse_str` uses `.map_err(|_| Error::invalid_param(...))`.
- All queries filter `"deleted_at": { "$exists": false }`.

**Register**: create `src/infrastructure/persistence/{entity}/mod.rs` with `pub mod model; pub mod repository;` and add `pub mod {entity};` in `src/infrastructure/persistence/mod.rs`.

---

### 5. Service (Application Layer)

**File**: `src/application/{entity}.rs`

```rust
use std::sync::Arc;
use crate::domain::error::{DomainResult, Error};
use crate::domain::pagination::Pagination;
use crate::domain::ports::{entity}::{Entity}RepositoryPort;
use crate::domain::{entity}::{{Entity}, {Entity}Id};

#[derive(Clone)]
pub struct {Entity}Service {
    repo: Arc<dyn {Entity}RepositoryPort>,
}

impl {Entity}Service {
    pub fn new(repo: Arc<dyn {Entity}RepositoryPort>) -> Self {
        Self { repo }
    }

    #[tracing::instrument(skip_all)]
    pub async fn create(&self, /* direct params */) -> DomainResult<{Entity}> {
        // 1. Business rules / validations
        // 2. Build entity
        let now = chrono::Utc::now();
        let mut entity = {Entity} {
            id: None,
            // ... fields from params ...
            created_at: now,
            updated_at: now,
            deleted_at: None,
        };
        // 3. Persist
        let id = self.repo.create(&entity).await?;
        entity.id = Some(id);
        Ok(entity)
    }

    #[tracing::instrument(skip_all, fields(%id))]
    pub async fn get(&self, id: &{Entity}Id) -> DomainResult<{Entity}> {
        self.repo.find_by_id(id).await?
            .ok_or_else(|| Error::not_found("{Entity}", id.to_string()))
    }

    #[tracing::instrument(skip_all)]
    pub async fn list(&self, pagination: Pagination) -> DomainResult<Vec<{Entity}>> {
        self.repo.find_all(pagination).await
    }

    #[tracing::instrument(skip_all)]
    pub async fn count(&self) -> DomainResult<u64> {
        self.repo.count().await
    }

    #[tracing::instrument(skip_all, fields(%id))]
    pub async fn delete(&self, id: &{Entity}Id) -> DomainResult<()> {
        if !self.repo.delete(id).await? {
            return Err(Error::not_found("{Entity}", id.to_string()));
        }
        Ok(())
    }
}
```

**Rules**:

- Depends on `Arc<dyn {Entity}RepositoryPort>` — never the concrete repo.
- Accepts **direct params** (`&str`, `&{Entity}Id`, `f64`, etc.), not DTOs.
- ≤6 params → pass directly. >6 params → group with an existing domain struct.
- Every public method annotated with `#[tracing::instrument]`.

**Register**: add `pub mod {entity};` in `src/application/mod.rs`.

---

### 6. DTOs (Input / Output)

**File**: `src/presentation/http/{entity}/dtos/input.rs`

```rust
use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Deserialize, ToSchema, Validate)]
pub struct Create{Entity}Input {
    #[validate(length(min = 1, message = "field cannot be empty"))]
    pub some_field: String,
    // ... only fields the client sends ...
}
```

**File**: `src/presentation/http/{entity}/dtos/output.rs`

```rust
use serde::Serialize;
use utoipa::ToSchema;
use crate::domain::{entity}::{Entity};

#[derive(Serialize, ToSchema)]
pub struct {Entity}Output {
    pub id: String,
    // ... fields to expose ...
    pub created_at: String,
    pub updated_at: String,
}

impl From<{Entity}> for {Entity}Output {
    fn from(e: {Entity}) -> Self {
        Self {
            id: e.id.map(|id| id.into_inner()).unwrap_or_default(),
            // ... map fields ...
            created_at: e.created_at.to_rfc3339(),
            updated_at: e.updated_at.to_rfc3339(),
        }
    }
}
```

**File**: `src/presentation/http/{entity}/dtos/mod.rs`

```rust
mod input;
mod output;
pub use input::*;
pub use output::*;
```

Input DTOs carry **validation only**. There are no `From<DTO> → Command` conversions — no command structs exist.

---

### 7. Routes (Handlers)

**File**: `src/presentation/http/{entity}/routes.rs`

```rust
use std::sync::Arc;
use axum::{Router, extract::{Path, Query, State}, routing::{get, post}};
use crate::application::{entity}::{Entity}Service;
use crate::domain::pagination::Pagination;
use crate::domain::{entity}::{Entity}Id;
use crate::presentation::http::error::ApiError;
use crate::presentation::http::response::{GenericApiResponse, GenericPagination};
use crate::presentation::http::validation::ValidatedJson;
use crate::presentation::state::AppState;
use super::dtos::{Create{Entity}Input, {Entity}Output};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(create).get(list))
        .route("/{id}", get(get_by_id).delete(delete))
}

#[utoipa::path(post, path = "/api/v1/{entities}", tag = "{Entities}", request_body = Create{Entity}Input,
    responses((status = 200, body = GenericApiResponse<{Entity}Output>)))]
#[tracing::instrument(skip_all)]
async fn create(
    State(service): State<Arc<{Entity}Service>>,
    ValidatedJson(input): ValidatedJson<Create{Entity}Input>,
) -> Result<GenericApiResponse<{Entity}Output>, ApiError> {
    let entity = service.create(&input.some_field).await?;   // direct params
    Ok(GenericApiResponse::success(entity.into()))
}

#[utoipa::path(get, path = "/api/v1/{entities}/{id}", tag = "{Entities}",
    responses((status = 200, body = GenericApiResponse<{Entity}Output>)))]
#[tracing::instrument(skip_all)]
async fn get_by_id(
    State(service): State<Arc<{Entity}Service>>,
    Path(id): Path<String>,
) -> Result<GenericApiResponse<{Entity}Output>, ApiError> {
    let typed_id = {Entity}Id::new(id);                      // String → typed ID at boundary
    let entity = service.get(&typed_id).await?;
    Ok(GenericApiResponse::success(entity.into()))
}

#[tracing::instrument(skip_all)]
async fn list(
    State(service): State<Arc<{Entity}Service>>,
    Query(query): Query<Pagination>,
) -> Result<GenericApiResponse<GenericPagination<{Entity}Output>>, ApiError> {
    let pagination = Pagination { page: query.page, limit: query.limit };
    let items = service.list(pagination).await?;
    let total = service.count().await?;
    Ok(GenericApiResponse::paginated(
        items.into_iter().map(Into::into).collect(),
        total, query.page, query.limit,
    ))
}

#[tracing::instrument(skip_all)]
async fn delete(
    State(service): State<Arc<{Entity}Service>>,
    Path(id): Path<String>,
) -> Result<GenericApiResponse<()>, ApiError> {
    service.delete(&{Entity}Id::new(id)).await?;
    Ok(GenericApiResponse::success(()))
}
```

**File**: `src/presentation/http/{entity}/mod.rs`

```rust
pub mod dtos;
pub mod routes;
```

Handler responsibilities — **nothing else**:

1. Validate DTO (`ValidatedJson`).
2. Convert `String` → typed ID (`{Entity}Id::new(id)`).
3. Call service with direct params.
4. Convert `Entity` → `Output` DTO.
5. Wrap in `GenericApiResponse`.

---

### 8. Wiring

**`src/presentation/http/mod.rs`** — nest routes:

```rust
pub mod {entity};

pub fn app_router() -> Router<AppState> {
    Router::new()
        .nest("/{entities}", {entity}::routes::router())
}
```

**`src/presentation/state.rs`** — add service to `AppState`:

```rust
#[derive(Clone)]
pub struct AppState {
    pub {entity}_service: Arc<{Entity}Service>,
}

impl FromRef<AppState> for Arc<{Entity}Service> {
    fn from_ref(state: &AppState) -> Self {
        state.{entity}_service.clone()
    }
}
```

**`src/main.rs`** — dependency injection:

```rust
// 1. Repository (concrete)
let {entity}_repo = Arc::new({Entity}Repository::new(&db));
{entity}_repo.create_indexes().await.ok();

// 2. Service (inject as Port — erases concrete type)
let {entity}_service = Arc::new({Entity}Service::new(
    {entity}_repo as Arc<dyn {Entity}RepositoryPort>,
));

// 3. State
let state = AppState { {entity}_service };
```

**`src/presentation/openapi.rs`** — register paths and schemas:

```rust
#[derive(OpenApi)]
#[openapi(
    paths({entity}::routes::create, {entity}::routes::get_by_id),
    components(schemas(Create{Entity}Input, {Entity}Output)),
    tags((name = "{Entities}", description = "..."))
)]
pub struct ApiDoc;
```

---

## Dependency Rules

```text
  Presentation ──► Application ──► Domain ◄── Infrastructure
                                     ▲              │
                                     │     implements Ports
                                     └──────────────┘
```

**Verify** — these commands must all return zero results:

```bash
grep -r "use crate::infrastructure" src/domain/
grep -r "use crate::presentation"  src/domain/
grep -r "bson::\|mongodb::"        src/domain/
grep -r "use crate::infrastructure" src/application/
grep -r "use crate::presentation"   src/application/
```

---

## Naming Conventions

| Element                | Convention               | Example                                                  |
| ---------------------- | ------------------------ | -------------------------------------------------------- |
| Files & folders        | **Singular**, snake_case | `invoice.rs`, `invoice/`                                 |
| Structs                | PascalCase, Singular     | `Invoice`, `InvoiceRepository`                           |
| Port traits            | `{Entity}RepositoryPort` | `InvoiceRepositoryPort`                                  |
| Infrastructure structs | **No tech prefix**       | `InvoiceRepository` ✅ / ~~`MongoInvoiceRepository`~~ ❌ |
| DB collections         | **Plural**, snake_case   | `invoices`, `order_items`                                |
| API routes             | **Plural**               | `/api/v1/invoices`                                       |
| Typed IDs              | `{Entity}Id`             | `InvoiceId = DomainId<InvoiceMarker>`                    |

---

## Error Handling

### Domain (`src/domain/error.rs`)

All errors are **database-agnostic**. The `DomainError` enum covers every business scenario:

| Variant           | Purpose                   | Infrastructure mapping                              |
| ----------------- | ------------------------- | --------------------------------------------------- | --- | ------------------------------------------- |
| `NotFound`        | Entity doesn't exist      | —                                                   |
| `AlreadyExists`   | Duplicate / conflict      | —                                                   |
| `Invalid`         | Validation failure        | `ObjectId::parse_str` → `Error::invalid_param(...)` |
| `Required`        | Missing field             | —                                                   |
| `Unauthorized`    | Auth failure              | —                                                   |
| `Forbidden`       | Permission denied         | —                                                   |
| `BusinessRule`    | Domain invariant violated | —                                                   |
| `Database`        | Persistence failure       | `.map_err(                                          | e   | Error::database(e.to_string()))`            |
| `ExternalService` | Third-party call failed   | `.map_err(                                          | e   | Error::external("service", e.to_string()))` |
| `Internal`        | Unexpected error          | —                                                   |

### Presentation (`src/presentation/http/error.rs`)

`DomainError` auto-maps to HTTP via `impl From<DomainError> for ApiError`:

| DomainError                             | HTTP Status                 |
| --------------------------------------- | --------------------------- |
| `NotFound`                              | 404                         |
| `AlreadyExists`                         | 409                         |
| `Invalid`/`Required`                    | 400                         |
| `Unauthorized`                          | 401                         |
| `Forbidden`                             | 403                         |
| `BusinessRule`                          | 422                         |
| `Database`/`Internal`/`ExternalService` | 500 (logged, detail hidden) |

### Rules

- **NEVER** use `unwrap()` or `expect()` in business code.
- Always return `DomainResult<T>`.
- Infrastructure errors are **mapped to strings** before crossing boundaries — `mongodb::Error` never leaks upward.

---

## Observability & Telemetry

The template ships with a **production-grade observability stack** wired out of the box.

### Stack

```text
┌──────────────┐     ┌───────────────────┐     ┌──────────────────┐
│  tracing      │────►│ tracing-subscriber │────►│ Stackdriver JSON  │  → Cloud Logging
│  (spans +     │     │ (EnvFilter)        │     │ (structured logs) │
│   events)     │     └────────┬──────────┘     └──────────────────┘
└──────────────┘              │
                              ▼
                    ┌───────────────────┐     ┌──────────────────┐
                    │ tracing-otel      │────►│ GCP Cloud Trace   │  → Distributed Tracing
                    │ (bridge layer)    │     │ (OTLP exporter)   │
                    └───────────────────┘     └──────────────────┘
```

### What's Configured (`infrastructure/providers/telemetry.rs`)

- **Structured JSON logs** via `tracing-stackdriver` — each log entry includes `trace_id`, `span_id`, severity, service metadata.
- **Distributed tracing** via `tracing-opentelemetry` + `opentelemetry-gcloud-trace` — every request gets a full trace exported to GCP Cloud Trace.
- **Resource attributes**: `service.name`, `service.version`, `deployment.environment`, `project.id` — attached to every trace.
- **Noise suppression**: `h2`, `hyper`, `tokio_util`, `tower_http` logs suppressed to `warn` level.
- **Log level control**: set via `DEBUG_LEVEL` env var (`debug`, `info`, `warn`, `error`).

### How to Instrument Your Code

**Services** — use `#[tracing::instrument]` on every public method:

```rust
#[tracing::instrument(skip_all, fields(%id))]
pub async fn get(&self, id: &{Entity}Id) -> DomainResult<{Entity}> {
    // Automatically creates a named span with the `id` field
    tracing::info!("Fetching entity");  // structured log inside the span
    self.repo.find_by_id(id).await?
        .ok_or_else(|| Error::not_found("{Entity}", id.to_string()))
}
```

**Repositories** — same pattern:

```rust
#[tracing::instrument(skip_all)]
async fn create(&self, entity: &{Entity}) -> DomainResult<{Entity}Id> { ... }
```

**Handlers** — same:

```rust
#[tracing::instrument(skip_all)]
async fn create(...) -> Result<..., ApiError> { ... }
```

### Trace ID in Every Response

`GenericApiResponse<T>` automatically extracts the current OpenTelemetry trace ID and includes it in every JSON response:

```json
{
  "trace_id": "0af7651916cd43dd8448eb211c80319c",
  "data": { ... }
}
```

This allows correlating any API response with its full trace in Cloud Trace and its logs in Cloud Logging — **invaluable for debugging production issues**.

### Log Examples

```rust
// Structured fields — appear as searchable attributes in Cloud Logging
tracing::info!(entity_id = %id, action = "created", "Entity persisted");
tracing::warn!(email = %email, "Duplicate email attempt");
tracing::error!(error = %e, "Database connection failed");
```

---

## Patterns

### Soft Deletes

All entities include `deleted_at: Option<DateTime<Utc>>`:

- **Delete** = `$set: { deleted_at: now }` — never `delete_one`.
- **All queries** filter `"deleted_at": { "$exists": false }`.
- **Indexes** include `deleted_at` as first key in compounds.

### Pagination

Every `find_all()` requires `Pagination { page, limit }`. Response uses `GenericPagination<T>`:

```json
{
  "trace_id": "...",
  "data": {
    "data": [ ... ],
    "total": 142,
    "page": 1,
    "limit": 20
  }
}
```

### Validated Input

Use `ValidatedJson<T>` instead of `Json<T>` — it deserializes **and** runs `validator` rules, returning a `400` with details on failure.

### Database Indexes

Every repository **must** implement `create_indexes()`. Called once on startup in `main.rs` — idempotent by MongoDB design.

### Testing (Ports Enable Mocking)

The Ports & Adapters architecture lets you test services without a database:

```rust
struct Mock{Entity}Repo;

#[async_trait]
impl {Entity}RepositoryPort for Mock{Entity}Repo {
    async fn create(&self, _: &{Entity}) -> DomainResult<{Entity}Id> {
        Ok({Entity}Id::new("mock-id"))
    }
    // ...
}

#[tokio::test]
async fn test_create() {
    let service = {Entity}Service::new(Arc::new(Mock{Entity}Repo));
    let result = service.create(/* params */).await;
    assert!(result.is_ok());
}
```

---

## Environment Variables

| Variable         | Required | Default                  | Description                                  |
| ---------------- | -------- | ------------------------ | -------------------------------------------- |
| `SERVICE_NAME`   | ✅       | —                        | Service name (traces + logs)                 |
| `PROJECT_ID`     | ✅       | —                        | GCP project ID (traces)                      |
| `MONGO_URL`      | ✅       | —                        | MongoDB connection string                    |
| `MONGO_DB`       | ✅       | —                        | Database name                                |
| `PORT`           | ❌       | `3000`                   | HTTP listen port                             |
| `APP_ENV`        | ❌       | `DEV`                    | `DEV` / `STG` / `PRD` — controls Swagger UI  |
| `REDIS_URL`      | ❌       | `redis://127.0.0.1:6379` | Redis connection string                      |
| `DEBUG_LEVEL`    | ❌       | `info`                   | Log level (`debug`, `info`, `warn`, `error`) |
| `STORAGE_BUCKET` | ❌       | —                        | GCS bucket name                              |
| `CORS_ORIGINS`   | ❌       | `*`                      | Comma-separated allowed origins              |

---

## API Documentation

Swagger UI auto-generated at **`http://localhost:{PORT}/swagger-ui`**.

> Disabled when `APP_ENV=PRD`.

Annotate handlers with `#[utoipa::path(...)]` and register in `src/presentation/openapi.rs`.

---

## Deployment

### Docker (distroless)

```bash
docker build -f build/Dockerfile -t my-service .
docker run -p 3000:3000 --env-file .env my-service
```

Multi-stage build: Rust builder → `gcr.io/distroless/cc-debian12` (no shell, no package manager, minimal attack surface). Release profile enables **LTO + strip**.

### Google Cloud Run

```bash
gcloud builds submit \
  --config=build/cloudbuild.yaml \
  --substitutions=_SERVICE_NAME=my-service,_REGION=us-central1
```

Pipeline: **Build** → **Push to Artifact Registry** → **Deploy to Cloud Run**.

Configure secrets via Cloud Run console or Cloud Build substitutions.

### Compile Performance

`sccache` is configured as the Rust compiler wrapper. Dev profile uses `incremental = true` + `opt-level = 0`, while all dependencies compile at `opt-level = 3`.
