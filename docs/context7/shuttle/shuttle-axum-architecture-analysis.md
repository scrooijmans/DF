# Shuttle Axum Architecture Analysis

## Executive Summary

This document analyzes how Shuttle uses Axum as its HTTP API framework. While Shuttle's internal gateway/server source code is not publicly documented in detail, this analysis is based on:
1. Shuttle's public documentation and examples
2. Standard Axum patterns used in production Rust APIs
3. Inferred architectural patterns from Shuttle's design philosophy
4. Best practices for building production Axum applications

**Note**: Some aspects of this analysis are inferred from Shuttle's public API and documentation patterns. Where information is inferred rather than directly documented, it is explicitly noted.

---

## 1. High-Level Server Architecture

### Core Subsystems

```
┌─────────────────────────────────────────────────────────────────┐
│                      HTTP Entry Point                            │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐         │
│  │   Hyper      │  │   Tower      │  │   Axum       │         │
│  │   Server     │  │   Service    │  │   Router     │         │
│  └──────────────┘  └──────────────┘  └──────────────┘         │
└────────────────────────────┬────────────────────────────────────┘
                             │
                             ▼
┌─────────────────────────────────────────────────────────────────┐
│                    Middleware Layer                              │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐         │
│  │   CORS       │  │   Tracing    │  │   Auth       │         │
│  │   Layer      │  │   Layer      │  │   Layer      │         │
│  └──────────────┘  └──────────────┘  └──────────────┘         │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐         │
│  │   Timeout    │  │   Compression│  │   Rate Limit │         │
│  │   Layer      │  │   Layer      │  │   Layer      │         │
│  └──────────────┘  └──────────────┘  └──────────────┘         │
└────────────────────────────┬────────────────────────────────────┘
                             │
                             ▼
┌─────────────────────────────────────────────────────────────────┐
│                    Router Composition Layer                      │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │              Main Router                                  │  │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐              │  │
│  │  │  API     │  │  Admin   │  │  Health  │              │  │
│  │  │  Router  │  │  Router  │  │  Router  │              │  │
│  │  └────┬─────┘  └────┬─────┘  └────┬─────┘              │  │
│  │       │              │              │                      │  │
│  │       └──────────────┼──────────────┘                      │  │
│  │                      │                                      │  │
│  │       ┌──────────────▼──────────────┐                      │  │
│  │       │    Nested Routers            │                      │  │
│  │       │  (Projects, Deployments, etc)│                      │  │
│  │       └─────────────────────────────┘                      │  │
│  └──────────────────────────────────────────────────────────┘  │
└────────────────────────────┬────────────────────────────────────┘
                             │
                             ▼
┌─────────────────────────────────────────────────────────────────┐
│                    Handler Layer                                 │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐         │
│  │   Request    │  │   Extractors │  │   Business   │         │
│  │   Handlers   │  │   (Path,     │  │   Logic      │         │
│  │              │  │    Query,    │  │   Services   │         │
│  │              │  │    Body)     │  │              │         │
│  └──────────────┘  └──────────────┘  └──────────────┘         │
└────────────────────────────┬────────────────────────────────────┘
                             │
                             ▼
┌─────────────────────────────────────────────────────────────────┐
│                    Application State Layer                       │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │              AppState (Shared State)                      │  │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐              │  │
│  │  │Database  │  │  Config  │  │  Cache   │              │  │
│  │  │  Pool    │  │  Service │  │  Service │              │  │
│  │  └──────────┘  └──────────┘  └──────────┘              │  │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐              │  │
│  │  │  Auth    │  │  Logger  │  │  Metrics │              │  │
│  │  │ Service  │  │  Service │  │  Service │              │  │
│  │  └──────────┘  └──────────┘  └──────────┘              │  │
│  └──────────────────────────────────────────────────────────┘  │
└────────────────────────────┬────────────────────────────────────┘
                             │
                             ▼
┌─────────────────────────────────────────────────────────────────┐
│                    Business Logic Layer                          │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐         │
│  │   Service    │  │   Repository │  │   Domain     │         │
│  │   Layer      │  │   Layer      │  │   Models     │         │
│  └──────────────┘  └──────────────┘  └──────────────┘         │
└─────────────────────────────────────────────────────────────────┘
```

### Responsibility Separation

1. **HTTP Layer**: Hyper server, Tower service, Axum router
2. **Middleware Layer**: Cross-cutting concerns (CORS, auth, tracing, etc.)
3. **Router Layer**: Route composition and organization
4. **Handler Layer**: Request processing and extraction
5. **State Layer**: Shared application state (database, config, services)
6. **Business Logic Layer**: Domain logic and data access

**Key Design Principle**: Clear separation of concerns enables testability, maintainability, and scalability.

---

## 2. Router Construction and Composition

### Basic Router Structure

Based on Shuttle's documentation patterns and standard Axum practices:

```rust
use axum::{
    Router,
    routing::{get, post, put, delete},
    middleware,
};

// Main router composition
pub fn create_router(state: AppState) -> Router {
    Router::new()
        // Health check (no state needed)
        .route("/health", get(health_check))
        
        // API routes (nested)
        .nest("/api/v1", api_router(state.clone()))
        
        // Admin routes (nested, with auth middleware)
        .nest("/admin", admin_router(state.clone()))
        
        // Apply global middleware
        .layer(middleware::from_fn_with_state(
            state.clone(),
            logging_middleware
        ))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware
        ))
        
        // Attach application state
        .with_state(state)
}
```

### Nested Router Composition

```rust
// API router (nested under /api/v1)
fn api_router(state: AppState) -> Router {
    Router::new()
        .nest("/projects", projects_router(state.clone()))
        .nest("/deployments", deployments_router(state.clone()))
        .nest("/resources", resources_router(state.clone()))
        .with_state(state)
}

// Projects router (nested under /api/v1/projects)
fn projects_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(list_projects).post(create_project))
        .route("/:id", get(get_project).put(update_project).delete(delete_project))
        .route("/:id/deployments", get(list_project_deployments))
        .with_state(state)
}

// Deployments router (nested under /api/v1/deployments)
fn deployments_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(list_deployments).post(create_deployment))
        .route("/:id", get(get_deployment).delete(delete_deployment))
        .route("/:id/logs", get(get_deployment_logs))
        .with_state(state)
}
```

### Router Composition Pattern

**Pattern**: Hierarchical router composition with nested routers

**Benefits**:
- Clear route organization
- Modular route definitions
- Easy to test individual route groups
- Supports versioning (e.g., `/api/v1`, `/api/v2`)

**Implementation**:
- Use `Router::nest()` for path prefixes
- Each nested router can have its own middleware
- State is cloned and passed to each router
- Final router has all routes composed

---

## 3. Application State Injection and Sharing

### State Definition

```rust
use std::sync::Arc;
use sqlx::PgPool;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct AppState {
    // Database connection pool
    pub db: PgPool,
    
    // Configuration
    pub config: Arc<Config>,
    
    // Services (wrapped in Arc for sharing)
    pub auth_service: Arc<AuthService>,
    pub deployment_service: Arc<DeploymentService>,
    pub project_service: Arc<ProjectService>,
    
    // Cache (if needed)
    pub cache: Arc<RwLock<Cache>>,
    
    // Logger
    pub logger: Arc<Logger>,
}

// Configuration struct
#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub api_key: String,
    pub environment: String,
}
```

### State Initialization

```rust
pub async fn create_app_state() -> Result<AppState, Error> {
    // Load configuration
    let config = Arc::new(load_config()?);
    
    // Initialize database pool
    let db = PgPool::connect(&config.database_url).await?;
    
    // Initialize services
    let auth_service = Arc::new(AuthService::new(config.clone()));
    let deployment_service = Arc::new(DeploymentService::new(db.clone()));
    let project_service = Arc::new(ProjectService::new(db.clone()));
    
    // Initialize cache
    let cache = Arc::new(RwLock::new(Cache::new()));
    
    // Initialize logger
    let logger = Arc::new(Logger::new());
    
    Ok(AppState {
        db,
        config,
        auth_service,
        deployment_service,
        project_service,
        cache,
        logger,
    })
}
```

### State Injection in Handlers

```rust
use axum::extract::State;

// Handler with state injection
async fn list_projects(
    State(state): State<AppState>,
) -> Result<Json<Vec<Project>>, AppError> {
    let projects = state.project_service.list_all().await?;
    Ok(Json(projects))
}

// Handler with state and path extraction
async fn get_project(
    State(state): State<AppState>,
    Path(project_id): Path<Uuid>,
) -> Result<Json<Project>, AppError> {
    let project = state.project_service.get_by_id(project_id).await?;
    Ok(Json(project))
}

// Handler with state, path, and body extraction
async fn create_deployment(
    State(state): State<AppState>,
    Path(project_id): Path<Uuid>,
    Json(payload): Json<CreateDeploymentRequest>,
) -> Result<(StatusCode, Json<Deployment>), AppError> {
    let deployment = state.deployment_service
        .create(project_id, payload)
        .await?;
    Ok((StatusCode::CREATED, Json(deployment)))
}
```

### State Sharing Pattern

**Pattern**: State cloned and shared via `Arc` for thread-safe access

**Benefits**:
- Thread-safe shared state
- Efficient cloning (Arc is cheap)
- Services can be shared across handlers
- Easy to test (can mock services)

**Implementation**:
- Wrap services in `Arc` for sharing
- Clone state when passing to routers
- Use `State` extractor in handlers
- State is available to all handlers and middleware

### FromRef Pattern for Subset State

```rust
use axum::extract::FromRef;

// Subset of state for specific router
#[derive(Clone)]
pub struct ApiState {
    pub db: PgPool,
    pub project_service: Arc<ProjectService>,
}

// Implement FromRef to extract subset from AppState
impl FromRef<AppState> for ApiState {
    fn from_ref(state: &AppState) -> ApiState {
        ApiState {
            db: state.db.clone(),
            project_service: state.project_service.clone(),
        }
    }
}

// Router with subset state
fn api_router(state: AppState) -> Router {
    Router::new()
        .route("/projects", get(list_projects))
        .with_state(state)  // Full state passed, but handlers can use ApiState
}

// Handler using subset state
async fn list_projects(
    State(api_state): State<ApiState>,  // Extracts subset via FromRef
) -> Result<Json<Vec<Project>>, AppError> {
    // Only has access to db and project_service
    let projects = api_state.project_service.list_all().await?;
    Ok(Json(projects))
}
```

---

## 4. Request Handling Flow

### Complete Request Flow

```
1. HTTP Request arrives at Hyper server
   │
   ▼
2. Hyper → Tower Service
   │   └─> Tower service wraps Axum router
   │
   ▼
3. Tower Middleware Stack
   │   ├─> Timeout middleware
   │   ├─> Compression middleware
   │   ├─> CORS middleware
   │   ├─> Tracing middleware
   │   └─> Custom middleware
   │
   ▼
4. Axum Router
   │   ├─> Route matching (path, method)
   │   ├─> Extractors (Path, Query, Body, State)
   │   └─> Handler execution
   │
   ▼
5. Handler Function
   │   ├─> Extract request data (Path, Query, Body)
   │   ├─> Extract state (State<AppState>)
   │   ├─> Validate input
   │   ├─> Call business logic (service layer)
   │   ├─> Handle errors
   │   └─> Return response
   │
   ▼
6. Response Conversion
   │   ├─> Handler returns impl IntoResponse
   │   ├─> Axum converts to HTTP response
   │   └─> Middleware processes response
   │
   ▼
7. HTTP Response sent to client
```

### Call Stack Example

```rust
// 1. HTTP Request: GET /api/v1/projects/123

// 2. Router matches route
Router::new()
    .nest("/api/v1", api_router(state))
    // Matches: /api/v1/projects/:id

// 3. Handler is called
async fn get_project(
    State(state): State<AppState>,           // State extractor
    Path(project_id): Path<Uuid>,            // Path extractor
) -> Result<Json<Project>, AppError> {
    // 4. Business logic
    let project = state.project_service
        .get_by_id(project_id)               // Service call
        .await?;                              // Database query
    
    // 5. Response
    Ok(Json(project))                        // IntoResponse
}

// 6. Error handling (if error occurs)
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // Convert error to HTTP response
        match self {
            AppError::NotFound => (StatusCode::NOT_FOUND, ...),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, ...),
            // ...
        }
    }
}
```

### Extractor Execution Order

Axum extractors execute in this order:

1. **Path Extractors**: Extract from URL path (`Path<T>`)
2. **Query Extractors**: Extract from query string (`Query<T>`)
3. **Header Extractors**: Extract from headers (`HeaderMap`, custom headers)
4. **Body Extractors**: Extract from request body (`Json<T>`, `Form<T>`)
5. **State Extractors**: Extract application state (`State<T>`)
6. **Request Extractors**: Extract full request (`Request`)

**Important**: If any extractor fails, the handler is not called, and an error response is returned.

---

## 5. Error Propagation and HTTP Response Conversion

### Error Type Definition

```rust
use axum::{
    response::{Response, IntoResponse},
    http::StatusCode,
    Json,
};
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    
    #[error("Bad request: {0}")]
    BadRequest(String),
    
    #[error("Internal server error: {0}")]
    Internal(#[from] anyhow::Error),
    
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Validation error: {0}")]
    Validation(String),
}

// Implement IntoResponse for automatic error conversion
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::NotFound(msg) => (
                StatusCode::NOT_FOUND,
                msg,
            ),
            AppError::Unauthorized(msg) => (
                StatusCode::UNAUTHORIZED,
                msg,
            ),
            AppError::BadRequest(msg) => (
                StatusCode::BAD_REQUEST,
                msg,
            ),
            AppError::Validation(msg) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                msg,
            ),
            AppError::Database(e) => {
                tracing::error!("Database error: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Database error occurred".to_string(),
                )
            },
            AppError::Internal(e) => {
                tracing::error!("Internal error: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            },
        };
        
        let body = Json(json!({
            "error": error_message,
            "status": status.as_u16(),
        }));
        
        (status, body).into_response()
    }
}
```

### Error Propagation in Handlers

```rust
// Handler with error propagation
async fn get_project(
    State(state): State<AppState>,
    Path(project_id): Path<Uuid>,
) -> Result<Json<Project>, AppError> {
    // Service returns Result<T, AppError>
    let project = state.project_service
        .get_by_id(project_id)
        .await?;  // ? operator propagates error
    
    Ok(Json(project))
}

// Service layer error handling
impl ProjectService {
    pub async fn get_by_id(&self, id: Uuid) -> Result<Project, AppError> {
        let project = sqlx::query_as::<_, Project>(
            "SELECT * FROM projects WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.db)
        .await?  // sqlx::Error -> AppError::Database
        .ok_or_else(|| AppError::NotFound(format!("Project {} not found", id)))?;
        
        Ok(project)
    }
}
```

### Error Handling Middleware

```rust
use tower::ServiceBuilder;
use tower_http::catch_panic::CatchPanicLayer;

// Add error handling middleware
let app = Router::new()
    .route("/api/projects", get(get_project))
    .layer(
        ServiceBuilder::new()
            .layer(CatchPanicLayer::new())  // Catch panics
            .into_inner()
    )
    .with_state(state);
```

### Result Type Pattern

**Pattern**: Handlers return `Result<T, E>` where `E: IntoResponse`

**Benefits**:
- Explicit error handling
- Type-safe error propagation
- Automatic HTTP response conversion
- Easy to test error cases

**Implementation**:
- Define custom error type
- Implement `IntoResponse` for error type
- Use `?` operator for error propagation
- Axum automatically converts errors to responses

---

## 6. Design Patterns Worth Copying

### 1. State Injection Pattern

**Pattern**: Application state injected via `State<T>` extractor

**Benefits**:
- Type-safe state access
- No global state
- Easy to test (can inject mock state)
- Thread-safe (Arc-based sharing)

**Implementation**:
```rust
// Define state
#[derive(Clone)]
struct AppState { ... }

// Inject in handler
async fn handler(State(state): State<AppState>) -> Response {
    // Use state
}
```

### 2. Router Composition Pattern

**Pattern**: Build router from smaller, nested routers

**Benefits**:
- Modular route organization
- Easy to test individual route groups
- Supports versioning
- Clear separation of concerns

**Implementation**:
```rust
Router::new()
    .nest("/api/v1", api_router(state))
    .nest("/admin", admin_router(state))
```

### 3. Extractor Pattern

**Pattern**: Use Axum extractors for request data

**Benefits**:
- Type-safe request parsing
- Automatic validation
- Declarative API
- Reusable extractors

**Implementation**:
```rust
async fn handler(
    Path(id): Path<Uuid>,
    Query(params): Query<QueryParams>,
    Json(body): Json<RequestBody>,
    State(state): State<AppState>,
) -> Response {
    // All data extracted and validated
}
```

### 4. Error Type Pattern

**Pattern**: Custom error type with `IntoResponse` implementation

**Benefits**:
- Consistent error responses
- Type-safe error handling
- Automatic HTTP conversion
- Easy error propagation

**Implementation**:
```rust
#[derive(Debug, thiserror::Error)]
enum AppError { ... }

impl IntoResponse for AppError {
    fn into_response(self) -> Response { ... }
}
```

### 5. Middleware Composition Pattern

**Pattern**: Compose middleware using Tower layers

**Benefits**:
- Reusable middleware
- Composable layers
- Works with any Tower service
- Easy to test

**Implementation**:
```rust
Router::new()
    .layer(middleware::from_fn(logging))
    .layer(middleware::from_fn(auth))
    .layer(CorsLayer::new())
```

### 6. Service Layer Pattern

**Pattern**: Business logic in service layer, handlers delegate to services

**Benefits**:
- Separation of concerns
- Reusable business logic
- Easy to test
- Clear responsibilities

**Implementation**:
```rust
// Handler delegates to service
async fn handler(State(state): State<AppState>) -> Response {
    state.service.do_work().await
}

// Service contains business logic
impl Service {
    pub async fn do_work(&self) -> Result<T, Error> {
        // Business logic here
    }
}
```

### 7. FromRef Pattern

**Pattern**: Extract subset of state using `FromRef` trait

**Benefits**:
- Limit state exposure
- Type-safe subset extraction
- Clear dependencies
- Easy to test

**Implementation**:
```rust
impl FromRef<AppState> for ApiState {
    fn from_ref(state: &AppState) -> ApiState {
        ApiState { ... }
    }
}
```

---

## 7. Complete Example: Production Axum API

### Main Application Setup

```rust
use axum::{
    Router,
    routing::{get, post},
    middleware,
    extract::State,
    response::Json,
    http::StatusCode,
};
use std::sync::Arc;
use sqlx::PgPool;
use tower::ServiceBuilder;
use tower_http::{
    cors::CorsLayer,
    trace::TraceLayer,
    timeout::TimeoutLayer,
    compression::CompressionLayer,
};

#[derive(Clone)]
struct AppState {
    db: PgPool,
    config: Arc<Config>,
    project_service: Arc<ProjectService>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();
    
    // Initialize state
    let state = create_app_state().await?;
    
    // Build router
    let app = create_router(state);
    
    // Start server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    tracing::info!("Server listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;
    
    Ok(())
}

fn create_router(state: AppState) -> Router {
    Router::new()
        // Health check
        .route("/health", get(health_check))
        
        // API routes
        .nest("/api/v1", api_router(state.clone()))
        
        // Apply middleware
        .layer(
            ServiceBuilder::new()
                .layer(TimeoutLayer::new(Duration::from_secs(30)))
                .layer(CompressionLayer::new())
                .layer(CorsLayer::permissive())
                .layer(TraceLayer::new_for_http())
                .layer(middleware::from_fn_with_state(
                    state.clone(),
                    auth_middleware
                ))
                .into_inner()
        )
        .with_state(state)
}

fn api_router(state: AppState) -> Router {
    Router::new()
        .nest("/projects", projects_router(state.clone()))
        .nest("/deployments", deployments_router(state.clone()))
        .with_state(state)
}

fn projects_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(list_projects).post(create_project))
        .route("/:id", get(get_project).put(update_project).delete(delete_project))
        .with_state(state)
}

// Handlers
async fn health_check() -> &'static str {
    "OK"
}

async fn list_projects(
    State(state): State<AppState>,
) -> Result<Json<Vec<Project>>, AppError> {
    let projects = state.project_service.list_all().await?;
    Ok(Json(projects))
}

async fn get_project(
    State(state): State<AppState>,
    Path(project_id): Path<Uuid>,
) -> Result<Json<Project>, AppError> {
    let project = state.project_service.get_by_id(project_id).await?;
    Ok(Json(project))
}

async fn create_project(
    State(state): State<AppState>,
    Json(payload): Json<CreateProjectRequest>,
) -> Result<(StatusCode, Json<Project>), AppError> {
    let project = state.project_service.create(payload).await?;
    Ok((StatusCode::CREATED, Json(project)))
}
```

### Middleware Example

```rust
use axum::{
    middleware::Next,
    extract::Request,
    response::Response,
};

async fn auth_middleware(
    State(state): State<AppState>,
    request: Request,
    next: Next,
) -> Result<Response, AppError> {
    // Extract auth token from header
    let auth_header = request.headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| AppError::Unauthorized("Missing Authorization header".to_string()))?;
    
    // Validate token
    let user = state.auth_service.validate_token(auth_header).await?;
    
    // Add user to request extensions
    request.extensions_mut().insert(user);
    
    // Continue to next middleware/handler
    Ok(next.run(request).await)
}

// Handler can access user from extensions
async fn get_project(
    State(state): State<AppState>,
    Path(project_id): Path<Uuid>,
    user: User,  // Extracted from extensions
) -> Result<Json<Project>, AppError> {
    // User is available from middleware
    let project = state.project_service.get_by_id(project_id, user.id).await?;
    Ok(Json(project))
}
```

---

## 8. Key Takeaways for Production Axum APIs

### Essential Patterns

1. **State Injection**: Use `State<T>` for shared application state
2. **Router Composition**: Build routers from smaller, nested routers
3. **Error Types**: Custom error types with `IntoResponse` implementation
4. **Service Layer**: Business logic in services, handlers delegate
5. **Middleware Stack**: Compose middleware using Tower layers
6. **Extractors**: Use Axum extractors for type-safe request parsing

### Best Practices

1. **State Management**:
   - Wrap services in `Arc` for sharing
   - Clone state when passing to routers
   - Use `FromRef` for subset state when needed

2. **Error Handling**:
   - Define custom error types
   - Implement `IntoResponse` for errors
   - Use `?` operator for error propagation
   - Log errors appropriately

3. **Router Organization**:
   - Group related routes in nested routers
   - Use versioning for API routes (`/api/v1`, `/api/v2`)
   - Separate admin routes from public routes
   - Keep route definitions close to handlers

4. **Middleware**:
   - Apply middleware in correct order
   - Use `from_fn_with_state` for stateful middleware
   - Compose middleware using ServiceBuilder
   - Test middleware independently

5. **Testing**:
   - Test handlers with mock state
   - Test routers independently
   - Test error cases
   - Use `axum_test` or similar for integration tests

---

## 9. Inferred vs. Documented Patterns

### Documented Patterns (from Shuttle examples):
- Basic router construction
- State injection via `with_state()`
- Handler function signatures
- Resource injection via attributes

### Inferred Patterns (based on production Axum best practices):
- Router composition structure
- Error handling implementation
- Middleware composition
- Service layer organization
- State management patterns

**Note**: While Shuttle's internal gateway implementation is not fully documented, the patterns described here are standard for production Axum applications and would be consistent with how Shuttle likely structures its API.

---

## 10. Conclusion

Shuttle's use of Axum follows standard production patterns for building Rust HTTP APIs:

1. **Router Composition**: Hierarchical router structure with nested routers
2. **State Injection**: Shared state via `State<T>` extractor
3. **Error Handling**: Custom error types with `IntoResponse`
4. **Middleware**: Tower-based middleware composition
5. **Service Layer**: Business logic separated from handlers

These patterns provide a solid foundation for building maintainable, testable, and scalable Axum applications.

---

## References

- Axum Documentation: https://docs.rs/axum
- Shuttle Documentation: https://docs.shuttle.dev
- Shuttle Examples: https://github.com/shuttle-hq/shuttle
- Tower Documentation: https://docs.rs/tower

