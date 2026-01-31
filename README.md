# 🦀 Rustlang Productivity Template: Layered Architecture + Granular DI

This repository is a **High-Performance Rust Microservice Template** refactored to use a **Layered Architecture (Horizontal Slicing)** pattern. It emphasizes separation of concerns, granular dependency injection, and production-grade robustness using `axum`, `mongodb` (v3), and `tokio`.

## 🏗 Philosophy: Separation of Concerns

Unlike vertical slices, this template organizes code by technical function (Layers). This ensures strict boundaries, testability, and a clear flow of data.

### Pillars of the Template:
1.  **Strict Layering**:
    *   **Presentation**: Entry points (HTTP, OpenAPI). Depends on Application.
    *   **Application**: Business Logic & Orchestration. Depends on Domain & Infrastructure.
    *   **Domain**: Pure Business Entities & Rules. Dependency-free.
    *   **Infrastructure**: External details (DB, Providers). Depends on Domain.
2.  **Granular Dependency Injection**:
    *   Uses `axum::extract::FromRef` to inject *only* the specific service needed by a handler, avoiding monolithic state objects.
3.  **Explicit ID Handling**:
    *   IDs are managed explicitly. The Service generates/retrieves the ID from the Repository, ensuring the Domain Entity is always consistent.

---

## 📂 Project Structure

```text
src/
├── domain/               # Enterprise Logic & Entities
│   ├── users.rs          # Entity definitions (Option<ObjectId>)
│   ├── products.rs
│   └── error.rs          # Domain-wide Error types
├── application/          # Use Cases & Services
│   ├── users.rs          # Business rules & Orchestration
│   └── products.rs
├── infrastructure/       # External Concerns
│   ├── persistence/      # Repository implementations
│   │   ├── users.rs
│   │   └── mongo.rs      # DB Connection logic
│   └── providers/        # 3rd party adapters (Redis, Telemetry)
├── presentation/         # Entry Points
│   ├── http/
│   │   ├── users/        # Routes & DTOs
│   │   ├── validation.rs # Axum Extractors
│   │   └── response.rs   # Standard Response Wrappers
│   ├── server.rs         # Server Setup
│   ├── state.rs          # Dependency Injection State
│   └── openapi.rs        # Swagger Registry
├── config.rs             # Configuration
└── main.rs               # Wiring & Entry Point
```

---

## 🛠 Development Guide: Adding a New Feature (e.g., Payments)

### 1. Domain Layer (`src/domain/payments.rs`)
Define your entity.
```rust
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Payment {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<bson::oid::ObjectId>,
    pub amount: f64,
}
```

### 2. Infrastructure Layer (`src/infrastructure/persistence/payments.rs`)
Implement the repository. `create` must return the generated ID.
```rust
pub struct PaymentRepository { collection: Collection<Payment> }

impl PaymentRepository {
    pub async fn create(&self, p: &Payment) -> Result<ObjectId> {
        let res = self.collection.insert_one(p).await?;
        Ok(res.inserted_id.as_object_id().unwrap_or_default())
    }
}
```

### 3. Application Layer (`src/application/payments.rs`)
Implement the service logic. Orchestrate the ID assignment.
```rust
pub struct PaymentService { repo: Arc<PaymentRepository> }

impl PaymentService {
    pub async fn create(&self, dto: CreatePaymentDto) -> Result<Payment, Error> {
        let mut payment = Payment { id: None, amount: dto.amount };
        let id = self.repo.create(&payment).await?;
        payment.id = Some(id);
        Ok(payment)
    }
}
```

### 4. Presentation Layer (`src/presentation/http/payments/`)
Define DTOs (`dtos.rs`) and Routes (`routes.rs`).
```rust
// routes.rs
#[utoipa::path(...)]
pub async fn create_payment(
    State(service): State<Arc<PaymentService>>, // Granular Injection
    ValidatedJson(req): ValidatedJson<CreatePaymentDto>
) -> ...
```

### 5. Wiring (`src/main.rs` & `src/presentation/state.rs`)
1.  Initialize Repository and Service in `main.rs`.
2.  Add Service to `AppState` in `state.rs`.
3.  Register routes in `src/presentation/http/mod.rs`.

---

## ⚙️ Tech Stack

*   **REST API**: Axum 0.8.
*   **Database**: MongoDB (v3 Driver, BSON 3).
*   **Docs**: Swagger UI (`/api-docs/openapi.json` exposed at `/swagger-ui`).
*   **Observability**: OpenTelemetry + Tracing.
*   **Validation**: Strong typing via `validator`.

## 📡 Local Execution

```bash
cp .env.example .env
cargo run
```
Access interactive documentation at: **`http://localhost:3000/swagger-ui`**