# Rust Microservice Template

This repository serves as a production-ready template for building scalable and maintainable microservices in Rust. It is built upon the principles of **Clean Architecture** (also known as Hexagonal or Ports and Adapters Architecture) to ensure a clear separation of concerns, making the codebase easy to test, evolve, and reason about.

## Core Principles

The project's structure is designed to isolate business logic from external concerns like databases, web frameworks, and other services. This is achieved by dividing the code into three main layers:

1.  **Domain (`src/domain`)**: The heart of the application. It contains the core business logic, entities, and abstract interfaces (ports) that define how the domain interacts with the outside world. This layer has **zero** knowledge of external technologies.
2.  **Implementation (`src/implementation`)**: This layer contains the application-specific use cases or services. It orchestrates the flow of data, calling methods on the domain entities and using the ports defined in the domain layer. It acts as a bridge between the infrastructure and the domain.
3.  **Infrastructure (`src/infrastructure`)**: This is the outermost layer. It contains the concrete implementations of the ports defined in the domain (e.g., a MongoDB repository, an Axum web server). It handles all communication with external systems, databases, and message queues.

This separation ensures that the core business logic (`domain`) can be changed and tested independently of the technology choices made in the `infrastructure` layer.

## Getting Started

### Prerequisites

- [Rust and Cargo](https://www.rust-lang.org/tools/install)

### 1. Configuration

The service is configured using environment variables. For local development, you can create a `.env` file in the project root.

Create a `.env` file by copying the example:

```sh
cp .env.example .env
```

Then, fill in the required variables in your new `.env` file:

```dotenv
# .env

# Server Configuration
PORT=8080

# Service Metadata for Tracing
SERVICE_NAME="my-hex-service"
PROJECT_ID="your-gcp-project-id" # Required for Google Cloud logging/tracing

# MongoDB Configuration
MONGO_URL="mongodb://localhost:27017"
MONGO_DB="my_database"

# Log Level (TRACE, DEBUG, INFO, WARN, ERROR)
DEBUG_LEVEL="INFO"
```

### 2. Running the Service

Once your `.env` file is configured, you can run the application with Cargo:

```sh
cargo run
```

The server will start, and you should see a log message indicating it is listening on the configured port.

## Key Features & Concepts

### Dependency Injection

Dependencies are wired up at application startup in `main.rs`. The `AppContext` struct holds shared state, such as service instances, which are then passed to the web handlers. This approach makes dependencies explicit and easy to manage.

```rustlang-template/src/main.rs#L42-44
struct AppContext {
    pub example_srv: Arc<ExampleService>,
}
```

### Advanced Error Handling

The template uses a custom error handling structure (`src/error.rs`) designed for robustness and security. The `AppError` struct separates errors into:

- **`public_message`**: A safe, user-friendly message to be returned in the HTTP response.
- **`internal_message`**: A detailed, internal-only message containing sensitive or technical information. This message is **logged** on the server but never sent to the client.
- **`status_code`**: The appropriate HTTP status code.
- **`metadata`**: Optional, dynamic `serde_json::Value` to provide extra context in logs or responses.

When an `AppError` is returned from a handler, it automatically triggers a `tracing::error!` event with the `internal_message` and sends a clean JSON response with the `public_message`.

### Logging and Tracing

The project uses the `tracing` crate for structured, level-based logging.

#### Log Levels

These are the log levels, ordered from least to most severe.

```
TRACE < DEBUG < INFO < WARN < ERROR
```

The `DEBUG_LEVEL` environment variable controls which log levels are displayed. By default, it is set to `INFO`, meaning that `INFO`, `WARN`, and `ERROR` logs will be visible. To see more detailed logs for debugging, set `DEBUG_LEVEL` to `DEBUG` or `TRACE`.

#### Example Usage

Simple text log:

```rust
tracing::info!("This is an info log");
tracing::warn!("This is a warning log");
```

Log with structured data, which is highly recommended for machine-readable logs:

```rust
tracing::info!(user_id = 42, "User logged in");
tracing::warn!(error = "Database connection failed", "A component failed to start");
```

### Optional Providers

The template includes several optional providers in `src/infrastructure/providers` for common Google Cloud services:

- `prv_pubsub.rs`: For publishing messages to Google Cloud Pub/Sub.
- `prv_storage.rs`: For interacting with Google Cloud Storage.
- `prv_tasks.rs`: For creating tasks in Google Cloud Tasks.

These are not wired into the application by default. To use them, you must expose them in `src/infrastructure/providers/mod.rs` and initialize them in `main.rs`, similar to how `MongoProvider` is handled.

## API Endpoints

This template provides a simple example feature with the following endpoints:

- `GET /api/v1/example`: Retrieves a list of all examples from the database.
- `POST /api/v1/example/random`: Creates a new example with a random name and adds it to the database.
