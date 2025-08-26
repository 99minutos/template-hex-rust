# Project Guidelines & Prompt (Rust Service Template)

Audience: Human contributors AND AI assistants collaborating on this repository.
Target Outcomes: Consistent architecture, predictable error semantics, robust observability, safe evolution, and high-quality automated + AI-assisted code changes.

---

## 1. Purpose & Vision

This template provides a production-leaning Rust HTTP microservice foundation with:

- Explicit layered architecture (domain-centric, ports/adapters).
- Strong error typing (`DomainError`) and uniform response envelope (`GenericApiResponse`).
- First-class observability (tracing, trace id propagation, GCP-friendly exporters).
- Extensible provider model (MongoDB, Pub/Sub, Redis, Cloud Tasks, Cloud Storage).
- Deterministic DTO mapping (isolation between internal entities and external schema).
- Safe async patterns without uncontrolled blocking or panics in request paths.

Principle: **Make the correct path the shortest path** (low-friction correctness).

---

## 2. Architectural Layers (Inner → Outer)

| Layer                   | Responsibility                                                                    | Allowed Dependencies                |
| ----------------------- | --------------------------------------------------------------------------------- | ----------------------------------- |
| Domain                  | Pure business types, entities, ports (traits), error model, serialization helpers | Standard lib, pure crates, NO infra |
| Implementation          | Application services orchestrating domain ports                                   | Domain                              |
| Infrastructure          | HTTP adapters, repositories, provider clients, middleware, route assembly         | Implementation + Domain             |
| Tools                   | Tracer + logger initialization                                                    | External libs only                  |
| Main (Composition Root) | Wire environment, providers, repositories, services, routes, server start         | All outer layers                    |

Rules:

1. Domain NEVER imports infrastructure.
2. Application services depend ONLY on traits (ports).
3. External systems touched ONLY in infrastructure.
4. DTOs are boundary-specific; never leak raw entities directly.
5. Avoid tight coupling: implement trait, inject via `Arc<Box<dyn Port...>>`.

---

## 3. Core Types & Contracts

### 3.1 `DomainError`

Variants:

- `NotFound(String)`
- `Conflict(String)`
- `Validation(String)`
- `Transient(String)` (temporary / retryable underlying cause)
- `Unknown(String)` (uncategorized / unexpected)

Factory helpers: `DomainError::not_found`, `::conflict`, `::validation`, `::transient`, `::unknown`.

### 3.2 `DomainWrapper<T>`

Type alias: `Result<T, DomainError>`.
All service & repository async functions return this wrapper.

### 3.3 HTTP Response: `GenericApiResponse<T>`

Structure (serialized):

```
{
  "success": bool,
  "trace_id": "string",
  "data": <optional payload>,
  "cause": <optional error message>
}
```

Internal (non-serialized) field: HTTP `StatusCode` (derived from result).

Mapping (DomainError → HTTP Status):

- NotFound → 404
- Conflict → 409
- Validation → 422
- Transient → 503
- Unknown (fallback) → 500
- Success → 200

Supports:

- `GenericApiResponse::from(DomainWrapper<T>)`
- `GenericApiResponse::from(T)` for direct success (any `T: Serialize`)
- Works with any serializable payload; no longer requires a custom marker trait (`OutputDto` no longer enforced for response serialization logic, though DTO usage is still recommended for boundaries).

Design Goals:

- Uniform envelope for clients.
- Trace correlation always present.
- Consistent status derivation.

---

## 4. Typical Flow (Happy Path)

`HTTP Handler` → calls `Service` → service calls a `Port` → adapter (repository/provider) talks to external system → returns `DomainWrapper<T>` → handler maps to DTO(s) → wraps in `GenericApiResponse` → serialized JSON.

Error path: fail early in adapter → map external error → return `DomainError::<Variant>` → automatic status code + cause rendered.

---

## 5. Handler Patterns

### 5.1 Success + Error Mapping (Vector DTO)

```
let result = service.list().await.map(|entities| {
    entities.into_iter().map(Dto::from).collect::<Vec<Dto>>()
});
Json(GenericApiResponse::from(result))
```

### 5.2 Single Entity Creation

```
let created = service.create(input).await.map(Dto::from);
Json(GenericApiResponse::from(created))
```

### 5.3 Direct Success (no domain call)

```
let info = HealthDto { status: "ok".into() };
Json(GenericApiResponse::from(info))
```

### 5.4 Decoding External Message (e.g. Pub/Sub)

```
let decoded: DomainWrapper<MyEvent> =
    request.decode_data::<MyEvent>().map_err(|e| {
        tracing::warn!(error = %e, "Decode failure");
        DomainError::validation("Invalid message")
    });

Json(GenericApiResponse::from(decoded))
```

---

## 6. DTO & Mapping Guidelines

1. DTOs live in `infrastructure/http/dto/`.
2. Use explicit `From<Entity> for Dto` and `From<Dto> for Entity` (no blanket generics).
3. Convert internal IDs (e.g. `ObjectId`) to string at boundary.
4. Keep DTO naming consistent (`<Thing>Dto` or domain boundary-specific naming).
5. Perform input validation BEFORE converting input DTO → domain entity.
6. Avoid mirroring domain entity 1:1 if external semantics differ (e.g. hide internal fields).

---

## 7. Mongo Repository Rules

- Set `id`, `created_at`, `updated_at` inside repository (persistence metadata authority).
- Index creation performed during repository initialization.
- Convert Mongo driver errors → `DomainError::Transient` (unless clearly another class).
- Avoid embedding business logic—repositories perform persistence only.
- Use `#[tracing::instrument(skip_all)]` on async public methods.

---

## 8. Error Handling Discipline

| Situation                                | Variant      |
| ---------------------------------------- | ------------ |
| Resource not found                       | `NotFound`   |
| Unique constraint / state violation      | `Conflict`   |
| Input invalid / decode / semantic rule   | `Validation` |
| Network / external temporary failure     | `Transient`  |
| Unanticipated panic boundary / logic gap | `Unknown`    |

Guidelines:

- Convert as close to the edge (adapter) as possible.
- Avoid losing context; keep high-level message user-safe.
- Do not propagate raw driver/internal error messages to clients (only sanitized message).

---

## 9. Observability

### Tracing:

- Every handler + repository method instrumented with `#[tracing::instrument(skip_all)]`.
- Use structured fields, e.g.:
  `tracing::debug!(filter=?filter_doc, "Executing Mongo find")`
- Avoid logging entire payloads containing user data—log shape or counts instead.

### Trace ID:

- Propagated via custom `RequestIdLayer`.
- Included in every response as `trace_id`.
- Exported to Google Cloud Trace if configured.

### Logging Levels:

- info: startup, major lifecycle events.
- debug: repository filters, branch decisions (safe info).
- warn: recoverable irregularities (retryable, user input issues).
- error: unexpected or serious domain failures (should map to `Unknown` or `Transient`).

---

## 10. Concurrency & Async Practices

- Avoid blocking code inside async tasks (if CPU-heavy, use `tokio::task::spawn_blocking`).
- Avoid cloning large data—prefer `Arc`.
- Futures must remain cancellation-safe (no partial external side-effects without idempotency).
- Repositories and services must be `Send + Sync`.

---

## 11. Configuration & Environment

Centralized via `EnvConfig`:

- Single initialization with `OnceLock`.
- Mandatory: `SERVICE_NAME`, `MONGO_URL`, `MONGO_DB`.
- Optional: `PROJECT_ID`.

Do NOT re-read `std::env` outside `EnvConfig`.

---

## 12. Testing Strategy (Planned)

(Placeholder until implemented)

- Unit: Service layer with mocked `Port` traits.
- Integration: Against ephemeral Mongo container (seed + verify).
- Contract (HTTP): Validate `GenericApiResponse` shape.
- Observability: Snapshot tests for emitted spans/log fields (optional advanced).

When adding tests:

- Avoid sleeping for timing—prefer deterministic hooks.
- Use `cargo clippy -- -D warnings` adoption gradually.

---

## 13. Naming Conventions

| Concept            | Pattern                             |
| ------------------ | ----------------------------------- |
| Port trait         | `Port<Entity>Repo` / `Port<Action>` |
| Repository adapter | `<Entity>Repository`                |
| Provider           | `<Thing>Provider`                   |
| Service            | `<Entity>Service`                   |
| DTO                | `<Entity>Dto`                       |
| Error enum         | `DomainError`                       |
| Config struct      | `EnvConfig`                         |

---

## 14. Panics Policy

Permitted only during startup initialization (e.g. missing mandatory env var).
Never panic or use `.unwrap()` / `.expect()` in request handling path.
Preferred pattern: `?` + `From` conversions → `DomainError`.

---

## 15. Extending Providers

Checklist for a new provider (e.g., cache, messaging):

1. Add file under `infrastructure/providers`.
2. Provide async constructor with explicit config arguments.
3. Add `Debug` implementation (hide sensitive internal clients).
4. Expose minimal API; map external errors early.
5. Inject in `main` and optionally wrap in trait if needed by domain/service.

---

## 16. Security Baseline

- CORS currently permissive: lock down allowed origins for production.
- Sanitize external inputs (length limits, restricted character sets where applicable).
- Never log secrets, tokens, or full raw payloads.
- Validate all externally supplied IDs; avoid trusting unparsed strings.
- Treat any external decode failure as `Validation`.

---

## 17. Performance & Scalability Notes

- Minimize allocation: reuse buffers where feasible (future optimization).
- Batch DB queries rather than N sequential calls.
- Add indexes before introducing query patterns at scale.
- Use metrics (future addition) for latency + error rates to inform scaling decisions.
- Consider connection pooling and backpressure when adding new providers.

---

## 18. Evolution Rules (Adding a Feature)

1. Domain impact? Update or create entity + port.
2. Implement repository/provider method (infrastructure).
3. Extend service (business orchestration).
4. Add handler + route; apply DTO mapping.
5. Add instrumentation.
6. Ensure `DomainError` mapping covers new error scenarios.
7. Return `GenericApiResponse`.
8. Update docs if architecture principles shift.

---

## 19. `GenericApiResponse` Deep Dive

Current capabilities:

- Accepts any `T: Serialize`.
- Derives HTTP status automatically from `DomainWrapper` error variant.
- Direct success shortcut via `GenericApiResponse::from(payload)` for raw payloads.
- Consistent `trace_id` inclusion.

Examples:

Success from domain call:

```
let result = svc.fetch_all().await.map(|v| v.into_iter().map(Dto::from).collect::<Vec<_>>());
Json(GenericApiResponse::from(result))
```

Direct success (no domain error context):

```
Json(GenericApiResponse::from(HealthDto { status: "ok".into() }))
```

Manual error (rare; prefer `DomainWrapper` normally):

```
let resp: GenericApiResponse<()> = GenericApiResponse::from(Err(DomainError::validation("bad input")));
```

Rationale for internal `status`:

- Keeps serialization simple (no duplicate status echo).
- Ensures one source of truth for code mapping logic.

---

## 20. Troubleshooting Guide

| Symptom                             | Likely Cause                                                   | Resolution                                             |
| ----------------------------------- | -------------------------------------------------------------- | ------------------------------------------------------ |
| All errors returning 500            | Using raw `Err(String)` or incorrect mapping                   | Wrap as `DomainError::<Variant>`                       |
| Missing `trace_id`                  | Handler not within traced span / middleware order issue        | Validate middleware order: RequestIdLayer + TraceLayer |
| Mongo time fields `null`            | Not set in repository                                          | Ensure repository sets timestamps before insert        |
| Panics in handler                   | Use of `unwrap()` somewhere in request path                    | Replace with `?` + conversion into `DomainError`       |
| Status code not matching error      | Bypassed `DomainWrapper` / manual response build               | Use `GenericApiResponse::from(domain_result)`          |
| DTO shows internal `_id` field name | Forgot `#[serde(rename = "_id")]` management or custom mapping | Ensure domain entity has rename, DTO uses string id    |
| Validation not triggered            | No input validation implemented                                | Add explicit validation before mapping DTO → domain    |
| Repeated trace IDs                  | Middlewares mis-ordered                                        | Ensure RequestIdLayer precedes TraceLayer              |

---

## 21. Anti-Patterns (Avoid)

- Passing raw DB models directly in HTTP responses.
- Embedding business logic inside repository adapters.
- Using `unwrap`/`expect` after startup.
- Silent catches (swallowing errors without logging).
- Creating ad-hoc JSON responses bypassing `GenericApiResponse`.
- Logging full user payloads (PII risk).
- Wide trait objects leaking infrastructure concerns into domain.

---

## 22. AI Assistant Prompting Guidelines

When requesting changes, always specify:

1. Objective: (e.g., "Add pagination to GET /api/v1/example").
2. Layers affected: (e.g., domain entity + repository + handler).
3. Constraints: performance,
   validation rules, error semantics.
4. Output format: diff vs full file vs explanation.
5. Test expectations (if applicable).

Good prompt example:
"Add a GET /api/v1/example/{id} endpoint. If not found return 404. Include repository method, service method, handler, route registration, and DTO. Show full new/changed files."

Bad prompt:
"Add endpoint."

Assistant Must:

- Preserve layering rules.
- Map errors via `DomainError`.
- Include `#[tracing::instrument]` on exposed async functions.
- Return `GenericApiResponse`.
- Avoid panics / unwrap in request path.
- Use explicit DTO mapping.

---

## 23. Code Review Checklist (Expanded)

Functional:
[ ] Domain invariants preserved.
[ ] Proper `DomainError` variant usage (no misuse of `Unknown`).
[ ] Input validation added where new input is parsed.

Architecture:
[ ] No domain → infrastructure imports.
[ ] New providers isolated behind traits if used by services.
[ ] DTO separation maintained.

Error / Response:
[ ] `GenericApiResponse` used uniformly.
[ ] Correct HTTP status mapping exercised in tests (if present).
[ ] No leaking internal IDs or raw driver errors.

Observability:
[ ] `#[tracing::instrument(skip_all)]` present on new async public functions.
[ ] Structured logs (no string concatenation clumps).
[ ] Warnings or errors logged with context fields.

Quality:
[ ] No `unwrap` / `expect` outside startup code.
[ ] Reasonable cloning (no unnecessary large data duplication).
[ ] Feature naturally composes with existing patterns.

Security:
[ ] No secrets in logs.
[ ] Input sanitized.
[ ] CORS or auth considerations acknowledged (if relevant).

---

## 24. Extensibility Strategy

When scaling:

- Split domain into sub-modules (`domain/entities/<bounded_context>`).
- Introduce feature flags or cfg gates if optional providers balloon dependencies.
- Add caching layer (e.g., Redis provider) behind trait: `PortCache`.

---

## 25. Future Enhancements (Roadmap Guidance)

- Add metrics (OpenTelemetry metrics) for request latency + error rate.
- Introduce rate limiting middleware.
- Implement structured validation errors (aggregation) → `Validation` variant with detail list.
- Add pagination pattern (cursor or offset) consistently across list endpoints.
- Implement generic repository trait if multiple aggregates arise.

---

## 26. Example Feature Prompt Library

Pagination:
"Add cursor-based pagination to GET /api/v1/example: accept ?after=<id>, limit (max 100). Update repository (query filter + sort), service, handler. Return metadata { next_cursor }. Show file diffs."

Soft Delete:
"Implement soft delete for Example: add deleted_at (optional) in domain + repository filter excluding deleted. Add DELETE endpoint returning 204 or 404. Include migration notes."

Pub/Sub Handler:
"Add POST /internal/pubsub/shipment that accepts PubSubRequest, decodes payload into ShipmentEvent DTO, validates required fields (id, status), maps errors to 422, and returns GenericApiResponse."

---

## 27. Glossary (Extended)

- Domain Layer: Pure logic boundary; no side-effects.
- Port: Trait abstraction defining required capability from external system.
- Adapter: Concrete implementation fulfilling a port.
- DTO: Transform boundary object for external communication.
- Trace ID: Correlation identifier across spans and logs.
- DomainWrapper: Canonical success/error envelope inside server.
- Provider: Infrastructure integration not tightly coupled to domain (e.g. messaging, storage).

---

## 28. Quick Reference (Copy/Paste Snippets)

Basic Handler Skeleton:

```
#[tracing::instrument(skip_all)]
async fn list(State(ctx): State<AppContext>) -> impl IntoResponse {
    let result = ctx.some_service.list().await.map(|items| {
        items.into_iter().map(Dto::from).collect::<Vec<Dto>>()
    });
    Json(GenericApiResponse::from(result))
}
```

Single Fetch with NotFound:

```
let result = svc.get(id).await.map(Dto::from);
Json(GenericApiResponse::from(result))
```

Repository Error Mapping:

```
let cursor = self.db.find(filter).await.map_err(|e| DomainError::transient(e.to_string()))?;
```

Validation Fail Fast:

```
if input.name.trim().is_empty() {
    return Json(GenericApiResponse::from(Err(DomainError::validation("name required"))));
}
```

---

## 29. Design Rationale Summary

- Strong typing reduces runtime ambiguity.
- Centralized error → status mapping ensures consistent client semantics.
- DTO boundary prevents leaking persistence/implementation details.
- Layer separation enforces testability and replaceability (swap Mongo, add caching, etc.).
- Observability baked in to accelerate debugging and production operations.

---

## 30. Final Principles (Memorable Shortlist)

1. Pure domain; effects at the edge.
2. One error enum to rule responses.
3. No panics in request flow.
4. Structured, trace-rich logs.
5. DTOs always deliberate.
6. Short path to correctness; helpers where needed.
7. Explicit mappings, no magic.

---

End of Guidelines.
