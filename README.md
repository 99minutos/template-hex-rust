## Logs

These are the log levels, ordered from least to most severe.

```
TRACE < DEBUG < INFO < WARN < ERROR < CRITICAL
```

The `DEBUG_LEVEL` environment variable controls which log levels are displayed.

By default, `DEBUG_LEVEL` is set to "INFO", meaning that INFO, WARN, ERROR, and CRITICAL logs will be visible in the system. To see more detailed logs, set `DEBUG_LEVEL` to a lower level, such as "DEBUG" or "TRACE".

### Example Usage

Simple text

```rust
tracing::trace!("This is a trace log");
tracing::debug!("This is a debug log");
tracing::info!("This is an info log");
tracing::warn!("This is a warning log");
tracing::error!("This is an error log");
```

With structured data

```rust
tracing::info!(user_id = 42, "User logged in");
tracing::warn!(error = "Database connection failed", "Failed to connect to the database");
```

With complex data types

```rust

#derive(Valuable)
struct User {
    id: u32,
    name: String,
}

let user = User { id: 42, name: "Alice".to_string() };
tracing::info!(user = user.as_value(), "User logged in");
```
