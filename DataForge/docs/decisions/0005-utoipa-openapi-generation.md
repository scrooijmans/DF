# Use utoipa for OpenAPI Specification Generation

- **Status**: accepted
- **Date**: 2026-01-05
- **Decision-makers**: Engineering team
- **Consulted**: Rust API documentation best practices
- **Informed**: Stakeholders

## Context and Problem Statement

DataForge's sync server needs API documentation for client developers, SDK generation, and contract validation. How should we generate and maintain OpenAPI specifications?

## Decision Drivers

- **Code-first**: Spec should be generated from code, not manually maintained
- **Type safety**: Rust types should drive the schema
- **Low overhead**: Minimal boilerplate for developers
- **Framework integration**: Must work with Axum
- **Interactive docs**: Swagger UI for exploration

## Considered Options

1. **utoipa** (with utoipa-axum)
2. **aide** (alternative OpenAPI generator)
3. **Manual OpenAPI YAML** (hand-written spec)

## Decision Outcome

**Chosen option**: "utoipa", because it's the most popular Rust OpenAPI library, has dedicated Axum integration, and generates specs directly from code using proc macros. This ensures documentation stays in sync with implementation.

### Consequences

**Positive:**
- Spec automatically generated from code
- Type-safe schema from Rust structs
- Native Axum router integration
- Swagger UI included
- OpenAPI 3.1 support

**Negative:**
- Proc macros add compilation time
- Must annotate all endpoints and types
- Learning curve for macro syntax

## Confirmation

- All sync endpoints annotated with `#[utoipa::path]`
- Request/response types derive `ToSchema`
- Swagger UI served at `/swagger-ui`
- OpenAPI JSON at `/api-docs/openapi.json`

## Pros and Cons of Options

### utoipa (with utoipa-axum)

Code-first OpenAPI generation using proc macros.

- **Good**: Most popular Rust OpenAPI library
- **Good**: Dedicated `utoipa-axum` integration
- **Good**: Generates from code (always in sync)
- **Good**: Type-safe schemas from Rust types
- **Good**: Swagger UI integration included
- **Good**: OpenAPI 3.1 support
- **Bad**: Proc macros add compilation time
- **Bad**: Must annotate every endpoint
- **Neutral**: Some learning curve for macro syntax

### aide

Alternative OpenAPI generator for Rust.

- **Good**: Simpler API in some cases
- **Good**: Axum support
- **Bad**: Smaller community
- **Bad**: Less documentation
- **Bad**: Fewer examples

### Manual OpenAPI YAML

Hand-written OpenAPI specification.

- **Good**: Full control over spec
- **Good**: No compilation overhead
- **Bad**: Must manually keep in sync with code
- **Bad**: Error-prone (spec can drift from implementation)
- **Bad**: No type safety
- **Bad**: Tedious to maintain

## More Information

### Implementation Pattern

**1. Add dependencies to `Cargo.toml`:**
```toml
[dependencies]
utoipa = { version = "5", features = ["axum_extras"] }
utoipa-axum = "0.2"
utoipa-swagger-ui = { version = "8", features = ["axum"] }
```

**2. Derive `ToSchema` on types:**
```rust
use utoipa::ToSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, ToSchema)]
pub struct PushRequest {
    /// Client's last known server version
    pub from_version: i64,
    /// Changes to push
    pub changes: Vec<SyncChange>,
    /// Blob hashes pending upload
    pub pending_blobs: Vec<String>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct PushResponse {
    /// New server version after applying changes
    pub server_version: i64,
    /// IDs of accepted changes
    pub accepted: Vec<Uuid>,
    /// Conflicting changes
    pub conflicts: Vec<SyncConflict>,
}
```

**3. Annotate handlers with `#[utoipa::path]`:**
```rust
/// Push local changes to server
#[utoipa::path(
    post,
    path = "/api/v1/sync/push",
    tag = "sync",
    request_body = PushRequest,
    responses(
        (status = 200, description = "Changes pushed successfully", body = PushResponse),
        (status = 401, description = "Unauthorized"),
        (status = 409, description = "Conflict detected", body = PushResponse)
    )
)]
async fn push_changes(
    State(state): State<AppState>,
    Json(request): Json<PushRequest>,
) -> Result<Json<PushResponse>, StatusCode> {
    // Implementation
}
```

**4. Create OpenAPI document:**
```rust
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "DataForge Sync API",
        version = "1.0.0",
        description = "Git-like sync protocol for well log data"
    ),
    tags(
        (name = "sync", description = "Sync operations"),
        (name = "blobs", description = "Blob storage operations")
    ),
    paths(push_changes, pull_changes, get_blob_urls)
)]
struct ApiDoc;
```

**5. Serve with Swagger UI:**
```rust
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_swagger_ui::SwaggerUi;

let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
    .routes(routes!(push_changes))
    .routes(routes!(pull_changes))
    .with_state(state)
    .split_for_parts();

let app = router
    .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api));
```

### Endpoints to Document

| Endpoint | Tag | Description |
|----------|-----|-------------|
| `POST /api/v1/sync/push` | sync | Push local changes |
| `POST /api/v1/sync/pull` | sync | Pull remote changes |
| `POST /api/v1/blobs/urls` | blobs | Get download URLs |
| `POST /api/v1/blobs/upload-urls` | blobs | Get upload URLs |
| `POST /api/v1/blobs/:hash/uploaded` | blobs | Confirm upload |
| `GET /health` | system | Health check |

### References

- [utoipa GitHub](https://github.com/juhaku/utoipa)
- [utoipa-axum documentation](https://docs.rs/utoipa-axum)
- [Working with OpenAPI using Rust](https://www.shuttle.dev/blog/2024/04/04/using-openapi-rust)
- [Production-ready microservice: OpenAPI](https://apatisandor.hu/blog/production-ready-openapi/)
