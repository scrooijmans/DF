# Rust Axum JWT Authentication Architecture Analysis

## Executive Summary

This document analyzes JWT-based authentication implementation with Axum, based on a production-ready implementation found in the codebase. The analysis covers the complete authentication flow from login to protected route access, including token creation, validation, middleware integration, and error handling.

---

## 1. High-Level Architecture

### Authentication Flow Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                    Client Application                            │
└────────────────────────────┬────────────────────────────────────┘
                             │
                             │ 1. POST /auth/login
                             │    { email, password }
                             ▼
┌─────────────────────────────────────────────────────────────────┐
│                    Auth Routes (Public)                          │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐         │
│  │   /login     │  │   /refresh   │  │   /register  │         │
│  │   Handler    │  │   Handler    │  │   Handler    │         │
│  └──────┬───────┘  └──────┬───────┘  └──────────────┘         │
│         │                  │                                    │
│         │ 2. Validate      │ 4. Validate                       │
│         │    credentials   │    refresh token                  │
│         │                  │                                    │
│         │ 3. Generate JWT  │ 5. Generate new                   │
│         │    tokens        │    access token                   │
│         └──────────────────┴────────────────────────────────────┘
└────────────────────────────┬────────────────────────────────────┘
                             │
                             │ Returns: { access_token, refresh_token }
                             ▼
┌─────────────────────────────────────────────────────────────────┐
│                    Client Stores Tokens                          │
│  ┌──────────────┐  ┌──────────────┐                            │
│  │ Access Token │  │Refresh Token │                            │
│  │ (short-lived)│  │ (long-lived) │                            │
│  └──────────────┘  └──────────────┘                            │
└────────────────────────────┬────────────────────────────────────┘
                             │
                             │ 6. Request with Authorization: Bearer <token>
                             ▼
┌─────────────────────────────────────────────────────────────────┐
│                    Authentication Middleware                     │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │  1. Extract Authorization header                         │  │
│  │  2. Parse Bearer token                                   │  │
│  │  3. Decode JWT token                                     │  │
│  │  4. Verify signature                                     │  │
│  │  5. Check expiration                                     │  │
│  │  6. Extract user claims                                  │  │
│  │  7. Insert user into request extensions                  │  │
│  └──────────────────────────────────────────────────────────┘  │
└────────────────────────────┬────────────────────────────────────┘
                             │
                             │ Request with authenticated user
                             ▼
┌─────────────────────────────────────────────────────────────────┐
│                    Protected Route Handlers                      │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐         │
│  │   Extract    │  │   Business   │  │   Return     │         │
│  │   User from  │  │   Logic      │  │   Response   │         │
│  │   Extensions │  │              │  │              │         │
│  └──────────────┘  └──────────────┘  └──────────────┘         │
└─────────────────────────────────────────────────────────────────┘
```

### Core Components

1. **Auth Routes**: Public endpoints for login, refresh, registration
2. **JWT Service**: Token creation, signing, validation
3. **Auth Middleware**: Request interception and token validation
4. **User Extractor**: Type-safe user extraction from request
5. **Error Handling**: Consistent error responses

---

## 2. Auth Routes Definition

### Router Setup

```rust
use axum::{
    Router,
    routing::{post, get},
    middleware,
    Json,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
struct AppState {
    auth_config: Arc<AuthConfig>,
    db: PgPool,
    // ... other services
}

fn create_router(state: AppState) -> Router {
    Router::new()
        // Public auth routes (no middleware)
        .route("/auth/login", post(login_handler))
        .route("/auth/refresh", post(refresh_handler))
        .route("/auth/register", post(register_handler))
        
        // Protected routes (with auth middleware)
        .route("/api/protected", get(protected_handler))
        .route("/api/user/profile", get(get_profile))
        
        // Apply auth middleware to protected routes
        .route_layer(middleware::from_fn_with_state(
            state.auth_config.clone(),
            auth_middleware,
        ))
        .with_state(state)
}
```

### Login Handler

```rust
#[derive(Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    access_token: String,
    refresh_token: String,
    token_type: String,
    expires_in: i64,
    user: UserInfo,
}

#[derive(Serialize)]
struct UserInfo {
    id: String,
    email: String,
    // ... other user fields
}

async fn login_handler(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    // 1. Validate credentials (check against database)
    let user = state
        .user_service
        .authenticate(&payload.email, &payload.password)
        .await
        .map_err(|_| AppError::InvalidCredentials)?;
    
    // 2. Generate access token (short-lived, e.g., 15 minutes)
    let access_token = generate_token(
        &user.id,
        &user.email,
        None, // workspace_id
        &state.auth_config,
    );
    
    // 3. Generate refresh token (long-lived, e.g., 7 days)
    let refresh_config = AuthConfig {
        token_expiry_seconds: 7 * 24 * 3600, // 7 days
        ..state.auth_config.clone()
    };
    let refresh_token = generate_token(
        &user.id,
        &user.email,
        None,
        &refresh_config,
    );
    
    // 4. Store refresh token in database (optional, for revocation)
    state
        .token_service
        .store_refresh_token(&user.id, &refresh_token.encode())
        .await?;
    
    // 5. Return tokens
    Ok(Json(LoginResponse {
        access_token: access_token.encode(),
        refresh_token: refresh_token.encode(),
        token_type: "Bearer".to_string(),
        expires_in: state.auth_config.token_expiry_seconds,
        user: UserInfo {
            id: user.id,
            email: user.email,
        },
    }))
}
```

### Refresh Handler

```rust
#[derive(Deserialize)]
struct RefreshRequest {
    refresh_token: String,
}

async fn refresh_handler(
    State(state): State<AppState>,
    Json(payload): Json<RefreshRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    // 1. Decode and validate refresh token
    let token = AuthToken::decode(&payload.refresh_token)
        .map_err(|_| AppError::InvalidToken)?;
    
    // 2. Verify signature
    let refresh_config = AuthConfig {
        token_expiry_seconds: 7 * 24 * 3600,
        ..state.auth_config.clone()
    };
    if !token.verify(&refresh_config.secret_key) {
        return Err(AppError::InvalidToken);
    }
    
    // 3. Check expiration
    if token.payload.is_expired() {
        return Err(AppError::TokenExpired);
    }
    
    // 4. Verify refresh token exists in database (optional, for revocation)
    let is_valid = state
        .token_service
        .verify_refresh_token(&token.payload.account_id, &payload.refresh_token)
        .await?;
    
    if !is_valid {
        return Err(AppError::InvalidToken);
    }
    
    // 5. Generate new access token
    let access_token = generate_token(
        &token.payload.account_id,
        &token.payload.email,
        token.payload.workspace_id.clone(),
        &state.auth_config,
    );
    
    // 6. Return new access token (refresh token remains the same)
    Ok(Json(LoginResponse {
        access_token: access_token.encode(),
        refresh_token: payload.refresh_token, // Return same refresh token
        token_type: "Bearer".to_string(),
        expires_in: state.auth_config.token_expiry_seconds,
        user: UserInfo {
            id: token.payload.account_id,
            email: token.payload.email,
        },
    }))
}
```

---

## 3. JWT Creation, Signing, and Validation

### Token Structure

```rust
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use chrono::{Duration, Utc};

/// Authenticated user information (JWT payload)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthUser {
    /// Account ID
    pub account_id: String,
    /// Email address
    pub email: String,
    /// Workspace ID (if scoped to a workspace)
    pub workspace_id: Option<String>,
    /// Token expiration timestamp (Unix timestamp)
    pub expires_at: i64,
}

impl AuthUser {
    /// Check if the token has expired
    pub fn is_expired(&self) -> bool {
        Utc::now().timestamp() > self.expires_at
    }
}

/// Authentication token (JWT-like structure)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthToken {
    /// Token payload (user claims)
    pub payload: AuthUser,
    /// Token signature (HMAC-SHA256)
    pub signature: String,
}
```

### Token Creation and Signing

```rust
impl AuthToken {
    /// Create a new authentication token
    pub fn new(user: AuthUser, secret_key: &str) -> Self {
        // 1. Serialize payload to JSON
        let payload_json = serde_json::to_string(&user)
            .expect("Failed to serialize user");
        
        // 2. Compute HMAC-SHA256 signature
        let signature = Self::compute_signature(&payload_json, secret_key);
        
        Self {
            payload: user,
            signature,
        }
    }
    
    /// Compute HMAC-SHA256 signature
    fn compute_signature(data: &str, secret: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        hasher.update(secret.as_bytes());
        hex::encode(hasher.finalize())
    }
    
    /// Encode token to string for transport (Base64 URL-safe)
    pub fn encode(&self) -> String {
        let json = serde_json::to_string(self)
            .expect("Failed to serialize token");
        base64_encode(&json)
    }
}
```

### Token Generation Helper

```rust
/// Generate a new auth token for a user
pub fn generate_token(
    account_id: impl Into<String>,
    email: impl Into<String>,
    workspace_id: Option<String>,
    config: &AuthConfig,
) -> AuthToken {
    // 1. Create user claims with expiration
    let user = AuthUser {
        account_id: account_id.into(),
        email: email.into(),
        workspace_id,
        expires_at: (Utc::now() + Duration::seconds(config.token_expiry_seconds))
            .timestamp(),
    };
    
    // 2. Create and sign token
    AuthToken::new(user, &config.secret_key)
}
```

### Token Validation

```rust
impl AuthToken {
    /// Decode token from string
    pub fn decode(token: &str) -> Result<Self, AuthError> {
        // 1. Decode Base64
        let json = base64_decode(token)?;
        
        // 2. Deserialize JSON
        serde_json::from_str(&json)
            .map_err(|_| AuthError::InvalidToken)
    }
    
    /// Verify the token signature
    pub fn verify(&self, secret_key: &str) -> bool {
        // 1. Serialize payload
        let payload_json = serde_json::to_string(&self.payload)
            .ok()?;
        
        // 2. Compute expected signature
        let expected_signature = Self::compute_signature(&payload_json, secret_key);
        
        // 3. Compare signatures (constant-time comparison recommended)
        self.signature == expected_signature
    }
}
```

### Base64 Encoding/Decoding

```rust
use base64::Engine;

/// Simple base64 encoding (URL-safe, no padding)
fn base64_encode(data: &str) -> String {
    use std::io::Write;
    let mut encoder = base64::write::EncoderStringWriter::new(
        &base64::engine::general_purpose::URL_SAFE_NO_PAD
    );
    encoder.write_all(data.as_bytes()).unwrap();
    encoder.into_inner()
}

/// Simple base64 decoding (URL-safe, no padding)
fn base64_decode(data: &str) -> Result<String, AuthError> {
    let bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(data)
        .map_err(|_| AuthError::InvalidToken)?;
    String::from_utf8(bytes)
        .map_err(|_| AuthError::InvalidToken)
}
```

---

## 4. Authentication Middleware Integration

### Middleware Implementation

```rust
use axum::{
    extract::State,
    middleware::Next,
    response::Response,
    http::{Request, StatusCode},
};
use std::sync::Arc;

/// Authentication middleware for Axum
///
/// Validates Bearer tokens and extracts user information.
pub async fn auth_middleware(
    State(config): State<Arc<AuthConfig>>,
    mut request: Request<axum::body::Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    // Development mode: skip auth if configured
    if config.skip_auth {
        tracing::debug!("Auth skipped (skip_auth=true)");
        request.extensions_mut().insert(AuthUser {
            account_id: "dev-user".to_string(),
            email: "dev@localhost".to_string(),
            workspace_id: None,
            expires_at: i64::MAX,
        });
        return Ok(next.run(request).await);
    }
    
    // 1. Extract Authorization header
    let auth_header = request
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;
    
    // 2. Parse Bearer token
    let token_str = auth_header
        .strip_prefix("Bearer ")
        .ok_or(StatusCode::UNAUTHORIZED)?;
    
    // 3. Decode token
    let token = match AuthToken::decode(token_str) {
        Ok(t) => t,
        Err(e) => {
            tracing::warn!("Invalid token: {}", e);
            return Err(StatusCode::UNAUTHORIZED);
        }
    };
    
    // 4. Verify signature
    if !token.verify(&config.secret_key) {
        tracing::warn!("Invalid token signature");
        return Err(StatusCode::UNAUTHORIZED);
    }
    
    // 5. Check expiration
    if token.payload.is_expired() {
        tracing::warn!("Token has expired");
        return Err(StatusCode::UNAUTHORIZED);
    }
    
    tracing::debug!("Authenticated user: {}", token.payload.email);
    
    // 6. Insert user into request extensions
    request.extensions_mut().insert(token.payload);
    
    // 7. Continue to next middleware/handler
    Ok(next.run(request).await)
}
```

### Middleware Wiring

```rust
use axum::{
    Router,
    middleware,
    routing::get,
};

fn create_router(state: AppState) -> Router {
    Router::new()
        // Public routes (no auth required)
        .route("/auth/login", post(login_handler))
        .route("/health", get(health_check))
        
        // Protected routes (auth required)
        .route("/api/protected", get(protected_handler))
        .route("/api/user/profile", get(get_profile))
        
        // Apply auth middleware to all routes below this point
        .route_layer(middleware::from_fn_with_state(
            state.auth_config.clone(),
            auth_middleware,
        ))
        .with_state(state)
}
```

### Selective Middleware Application

```rust
// Apply middleware only to specific routes
fn create_router(state: AppState) -> Router {
    Router::new()
        // Public routes
        .route("/auth/login", post(login_handler))
        .route("/health", get(health_check))
        
        // Protected API routes
        .nest(
            "/api",
            Router::new()
                .route("/protected", get(protected_handler))
                .route("/user/profile", get(get_profile))
                // Middleware only applies to /api/* routes
                .route_layer(middleware::from_fn_with_state(
                    state.auth_config.clone(),
                    auth_middleware,
                ))
        )
        .with_state(state)
}
```

---

## 5. Request Call Stack for Protected Endpoint

### Complete Call Stack

```
1. HTTP Request arrives
   │
   │   GET /api/protected
   │   Authorization: Bearer <token>
   │
   ▼
2. Hyper Server receives request
   │
   ▼
3. Tower Service (Axum Router)
   │
   ▼
4. Route Matching
   │   Router matches: /api/protected → protected_handler
   │
   ▼
5. Middleware Stack Execution (before handler)
   │
   │   ┌─────────────────────────────────────┐
   │   │ auth_middleware()                   │
   │   │                                     │
   │   │ 5.1. Extract State<AuthConfig>     │
   │   │ 5.2. Extract Request               │
   │   │ 5.3. Extract Next                  │
   │   │                                     │
   │   │ 5.4. Get Authorization header      │
   │   │      request.headers()             │
   │   │      .get("Authorization")         │
   │   │                                     │
   │   │ 5.5. Parse Bearer token            │
   │   │      strip_prefix("Bearer ")       │
   │   │                                     │
   │   │ 5.6. Decode token                  │
   │   │      AuthToken::decode(token_str)  │
   │   │        → base64_decode()           │
   │   │        → serde_json::from_str()    │
   │   │                                     │
   │   │ 5.7. Verify signature              │
   │   │      token.verify(secret_key)      │
   │   │        → compute_signature()       │
   │   │        → compare signatures        │
   │   │                                     │
   │   │ 5.8. Check expiration              │
   │   │      token.payload.is_expired()    │
   │   │                                     │
   │   │ 5.9. Insert user into extensions   │
   │   │      request.extensions_mut()      │
   │   │      .insert(token.payload)        │
   │   │                                     │
   │   │ 5.10. Call next.run(request)       │
   │   └─────────────────────────────────────┘
   │
   ▼
6. Handler Execution
   │
   │   ┌─────────────────────────────────────┐
   │   │ protected_handler()                 │
   │   │                                     │
   │   │ 6.1. Extract user from extensions   │
   │   │      ExtractUser(user)              │
   │   │        → parts.extensions           │
   │   │        .get::<AuthUser>()           │
   │   │                                     │
   │   │ 6.2. Execute business logic         │
   │   │      // Use user.account_id, etc.   │
   │   │                                     │
   │   │ 6.3. Return response                │
   │   │      Json(response_data)            │
   │   └─────────────────────────────────────┘
   │
   ▼
7. Response Conversion
   │   Handler returns impl IntoResponse
   │   Axum converts to HTTP Response
   │
   ▼
8. Middleware Stack Execution (after handler)
   │   (if any response middleware exists)
   │
   ▼
9. HTTP Response sent to client
```

### Detailed Code Flow

```rust
// Step 1-4: Request arrives, route matched
// GET /api/protected
// Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...

// Step 5: Middleware executes
pub async fn auth_middleware(
    State(config): State<Arc<AuthConfig>>,  // Extract config from state
    mut request: Request<Body>,              // Extract request
    next: Next,                              // Extract next middleware/handler
) -> Result<Response, StatusCode> {
    // Extract header
    let auth_header = request.headers()
        .get("Authorization")                // Get header
        .and_then(|h| h.to_str().ok())       // Convert to string
        .ok_or(StatusCode::UNAUTHORIZED)?;   // Return 401 if missing
    
    // Parse Bearer token
    let token_str = auth_header
        .strip_prefix("Bearer ")             // Remove "Bearer " prefix
        .ok_or(StatusCode::UNAUTHORIZED)?;   // Return 401 if invalid format
    
    // Decode token
    let token = AuthToken::decode(token_str)?;  // Base64 decode + JSON parse
    
    // Verify signature
    if !token.verify(&config.secret_key) {
        return Err(StatusCode::UNAUTHORIZED);
    }
    
    // Check expiration
    if token.payload.is_expired() {
        return Err(StatusCode::UNAUTHORIZED);
    }
    
    // Insert user into extensions
    request.extensions_mut()
        .insert(token.payload);              // Store AuthUser in extensions
    
    // Continue to handler
    Ok(next.run(request).await)              // Call next middleware/handler
}

// Step 6: Handler executes
async fn protected_handler(
    ExtractUser(user): ExtractUser,          // Extract user from extensions
) -> Json<ProtectedResponse> {
    // Business logic using user.account_id, user.email, etc.
    Json(ProtectedResponse {
        message: format!("Hello, {}!", user.email),
        user_id: user.account_id,
    })
}
```

---

## 6. User Identity Extraction and Passing to Handlers

### Custom Extractor Implementation

```rust
use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};

/// Axum extractor for authenticated user
#[derive(Debug, Clone)]
pub struct ExtractUser(pub AuthUser);

impl<S> FromRequestParts<S> for ExtractUser
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        // Extract AuthUser from request extensions
        parts
            .extensions
            .get::<AuthUser>()                // Get from extensions
            .cloned()                         // Clone (AuthUser is Clone)
            .map(ExtractUser)                 // Wrap in ExtractUser
            .ok_or(StatusCode::UNAUTHORIZED)  // Return 401 if not found
    }
}
```

### Using the Extractor in Handlers

```rust
// Simple handler with user extraction
async fn protected_handler(
    ExtractUser(user): ExtractUser,
) -> Json<ProtectedResponse> {
    Json(ProtectedResponse {
        message: format!("Hello, {}!", user.email),
        user_id: user.account_id,
    })
}

// Handler with multiple extractors
async fn get_user_profile(
    ExtractUser(user): ExtractUser,
    State(state): State<AppState>,
    Path(user_id): Path<String>,
) -> Result<Json<UserProfile>, AppError> {
    // Verify user can access this profile
    if user.account_id != user_id {
        return Err(AppError::Forbidden);
    }
    
    // Fetch profile from database
    let profile = state.user_service
        .get_profile(&user.account_id)
        .await?;
    
    Ok(Json(profile))
}

// Handler with user and query parameters
async fn list_user_resources(
    ExtractUser(user): ExtractUser,
    Query(params): Query<ResourceQuery>,
) -> Json<Vec<Resource>> {
    // Use user.account_id to filter resources
    let resources = fetch_user_resources(&user.account_id, &params).await;
    Json(resources)
}
```

### Alternative: Direct Extension Access

```rust
use axum::extract::Extension;

// Alternative approach using Extension extractor
async fn protected_handler_alt(
    Extension(user): Extension<AuthUser>,
) -> Json<ProtectedResponse> {
    Json(ProtectedResponse {
        message: format!("Hello, {}!", user.email),
        user_id: user.account_id,
    })
}
```

**Note**: `Extension` is deprecated in favor of `FromRequestParts` custom extractors, but it still works.

---

## 7. Error Handling and Response Surfacing

### Error Type Definition

```rust
use axum::{
    response::{Response, IntoResponse},
    http::StatusCode,
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Invalid credentials")]
    InvalidCredentials,
    
    #[error("Missing authorization header")]
    MissingHeader,
    
    #[error("Invalid authorization header format")]
    InvalidHeader,
    
    #[error("Invalid token")]
    InvalidToken,
    
    #[error("Token has expired")]
    TokenExpired,
    
    #[error("Invalid signature")]
    InvalidSignature,
    
    #[error("Insufficient permissions")]
    InsufficientPermissions,
    
    #[error("Forbidden")]
    Forbidden,
    
    #[error("Internal server error: {0}")]
    Internal(#[from] anyhow::Error),
    
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
}

// Implement IntoResponse for automatic error conversion
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::InvalidCredentials => (
                StatusCode::UNAUTHORIZED,
                "Invalid credentials",
            ),
            AppError::MissingHeader => (
                StatusCode::UNAUTHORIZED,
                "Missing authorization header",
            ),
            AppError::InvalidHeader => (
                StatusCode::UNAUTHORIZED,
                "Invalid authorization header format",
            ),
            AppError::InvalidToken => (
                StatusCode::UNAUTHORIZED,
                "Invalid token",
            ),
            AppError::TokenExpired => (
                StatusCode::UNAUTHORIZED,
                "Token has expired",
            ),
            AppError::InvalidSignature => (
                StatusCode::UNAUTHORIZED,
                "Invalid token signature",
            ),
            AppError::InsufficientPermissions => (
                StatusCode::FORBIDDEN,
                "Insufficient permissions",
            ),
            AppError::Forbidden => (
                StatusCode::FORBIDDEN,
                "Forbidden",
            ),
            AppError::Internal(e) => {
                tracing::error!("Internal error: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error",
                )
            },
            AppError::Database(e) => {
                tracing::error!("Database error: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Database error",
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

### Middleware Error Handling

```rust
pub async fn auth_middleware(
    State(config): State<Arc<AuthConfig>>,
    mut request: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    // All errors return StatusCode::UNAUTHORIZED
    // Axum automatically converts StatusCode to HTTP response
    
    let auth_header = request
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;  // Returns 401
    
    let token_str = auth_header
        .strip_prefix("Bearer ")
        .ok_or(StatusCode::UNAUTHORIZED)?;  // Returns 401
    
    let token = AuthToken::decode(token_str)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;  // Returns 401
    
    if !token.verify(&config.secret_key) {
        return Err(StatusCode::UNAUTHORIZED);  // Returns 401
    }
    
    if token.payload.is_expired() {
        return Err(StatusCode::UNAUTHORIZED);  // Returns 401
    }
    
    request.extensions_mut().insert(token.payload);
    Ok(next.run(request).await)
}
```

### Handler Error Handling

```rust
async fn login_handler(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    // Errors automatically converted to HTTP responses via IntoResponse
    let user = state
        .user_service
        .authenticate(&payload.email, &payload.password)
        .await
        .map_err(|_| AppError::InvalidCredentials)?;  // Returns 401 JSON
    
    // ... rest of handler
    Ok(Json(response))
}
```

### Custom Error Response Format

```rust
// More detailed error responses
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_code, error_message) = match self {
            AppError::InvalidCredentials => (
                StatusCode::UNAUTHORIZED,
                "INVALID_CREDENTIALS",
                "Invalid email or password",
            ),
            AppError::TokenExpired => (
                StatusCode::UNAUTHORIZED,
                "TOKEN_EXPIRED",
                "The access token has expired. Please refresh your token.",
            ),
            // ... other errors
        };
        
        let body = Json(json!({
            "error": {
                "code": error_code,
                "message": error_message,
            },
            "status": status.as_u16(),
        }));
        
        (status, body).into_response()
    }
}
```

---

## 8. Design Patterns and Scalability

### Patterns That Scale Well

#### 1. **Stateless JWT Tokens**
- ✅ No server-side session storage required
- ✅ Works well with horizontal scaling
- ✅ No shared state between servers
- ⚠️ Cannot revoke tokens without additional infrastructure

#### 2. **Middleware-Based Authentication**
- ✅ Centralized auth logic
- ✅ Easy to apply to multiple routes
- ✅ Consistent error handling
- ✅ Testable in isolation

#### 3. **Custom Extractors**
- ✅ Type-safe user extraction
- ✅ Compile-time guarantees
- ✅ Clean handler signatures
- ✅ Reusable across handlers

#### 4. **Error Type with IntoResponse**
- ✅ Consistent error responses
- ✅ Type-safe error handling
- ✅ Automatic HTTP conversion
- ✅ Easy to extend

### Patterns That Need Adaptation for Larger Systems

#### 1. **Symmetric Key Signing (HMAC-SHA256)**

**Current Implementation:**
```rust
// Single secret key for all tokens
pub struct AuthConfig {
    pub secret_key: String,  // Shared secret
}
```

**Scaling Issues:**
- ❌ All servers must share the same secret
- ❌ Key rotation requires coordination
- ❌ Cannot verify tokens without the secret

**Better for Scale:**
```rust
// Asymmetric key signing (RS256)
use jsonwebtoken::{encode, decode, Header, Algorithm, EncodingKey, DecodingKey};

pub struct AuthConfig {
    pub private_key: EncodingKey,   // Private key for signing
    pub public_key: DecodingKey,    // Public key for verification
}

// Benefits:
// - Public key can be shared (JWKS endpoint)
// - Private key only needed for token generation
// - Multiple servers can verify without sharing secrets
```

#### 2. **In-Memory Token Validation**

**Current Implementation:**
```rust
// Token validation is purely in-memory
if !token.verify(&config.secret_key) {
    return Err(StatusCode::UNAUTHORIZED);
}
```

**Scaling Issues:**
- ❌ Cannot revoke tokens
- ❌ Cannot track token usage
- ❌ No audit trail

**Better for Scale:**
```rust
// Token validation with database check
async fn validate_token(
    token: &AuthToken,
    config: &AuthConfig,
    db: &PgPool,
) -> Result<(), AppError> {
    // 1. Verify signature (in-memory)
    if !token.verify(&config.secret_key) {
        return Err(AppError::InvalidSignature);
    }
    
    // 2. Check expiration (in-memory)
    if token.payload.is_expired() {
        return Err(AppError::TokenExpired);
    }
    
    // 3. Check revocation list (database)
    let is_revoked = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM revoked_tokens WHERE token_id = $1)"
    )
    .bind(&token.payload.account_id)
    .fetch_one(db)
    .await?;
    
    if is_revoked {
        return Err(AppError::TokenRevoked);
    }
    
    Ok(())
}
```

#### 3. **Simple Token Structure**

**Current Implementation:**
```rust
// Simple custom token structure
pub struct AuthToken {
    pub payload: AuthUser,
    pub signature: String,
}
```

**Scaling Issues:**
- ❌ Not standard JWT format
- ❌ Cannot use standard JWT libraries
- ❌ Limited interoperability

**Better for Scale:**
```rust
// Standard JWT using jsonwebtoken crate
use jsonwebtoken::{encode, decode, Header, Algorithm, EncodingKey, DecodingKey, Validation, Claims};

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,           // Subject (user ID)
    email: String,
    exp: usize,            // Expiration
    iat: usize,            // Issued at
    workspace_id: Option<String>,
}

// Generate token
let token = encode(
    &Header::new(Algorithm::HS256),
    &claims,
    &EncodingKey::from_secret(secret.as_ref()),
)?;

// Verify token
let token_data = decode::<Claims>(
    &token,
    &DecodingKey::from_secret(secret.as_ref()),
    &Validation::new(Algorithm::HS256),
)?;
```

#### 4. **No Refresh Token Rotation**

**Current Implementation:**
```rust
// Refresh token remains the same
refresh_token: payload.refresh_token, // Return same refresh token
```

**Scaling Issues:**
- ❌ Refresh tokens never expire (security risk)
- ❌ Cannot detect token theft
- ❌ No token rotation

**Better for Scale:**
```rust
// Refresh token rotation
async fn refresh_handler(
    State(state): State<AppState>,
    Json(payload): Json<RefreshRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    // 1. Validate old refresh token
    let old_token = validate_refresh_token(&payload.refresh_token, &state).await?;
    
    // 2. Revoke old refresh token
    state.token_service.revoke_token(&old_token.id).await?;
    
    // 3. Generate new access token
    let access_token = generate_access_token(&old_token.user_id, &state).await?;
    
    // 4. Generate new refresh token (rotation)
    let refresh_token = generate_refresh_token(&old_token.user_id, &state).await?;
    
    Ok(Json(LoginResponse {
        access_token,
        refresh_token,  // New refresh token
        // ...
    }))
}
```

### Recommended Architecture for Production

```rust
// Production-ready JWT auth architecture

// 1. Use standard JWT library
use jsonwebtoken::{encode, decode, Header, Algorithm, EncodingKey, DecodingKey, Validation};

// 2. Asymmetric key signing (RS256)
pub struct AuthConfig {
    pub private_key: EncodingKey,   // For signing
    pub public_key: DecodingKey,    // For verification
    pub issuer: String,             // Token issuer
    pub audience: String,           // Token audience
}

// 3. Token validation with database
pub async fn validate_token(
    token: &str,
    config: &AuthConfig,
    db: &PgPool,
) -> Result<Claims, AppError> {
    // Verify signature
    let token_data = decode::<Claims>(
        token,
        &config.public_key,
        &Validation::new(Algorithm::RS256)
            .iss(&[&config.issuer])
            .aud(&[&config.audience]),
    )?;
    
    // Check revocation
    if is_token_revoked(&token_data.claims.jti, db).await? {
        return Err(AppError::TokenRevoked);
    }
    
    Ok(token_data.claims)
}

// 4. Refresh token rotation
// 5. Token blacklisting/revocation
// 6. Rate limiting on auth endpoints
// 7. Audit logging
```

---

## 9. Complete Example: Production-Ready Implementation

```rust
use axum::{
    Router,
    routing::{post, get},
    middleware,
    extract::{State, FromRequestParts},
    http::{request::Parts, StatusCode},
    response::{Response, IntoResponse},
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

// ============================================================================
// Types
// ============================================================================

#[derive(Clone)]
struct AppState {
    auth_config: Arc<AuthConfig>,
    db: PgPool,
    user_service: Arc<UserService>,
}

#[derive(Clone)]
struct AuthConfig {
    secret_key: String,
    token_expiry_seconds: i64,
    refresh_token_expiry_seconds: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AuthUser {
    account_id: String,
    email: String,
    workspace_id: Option<String>,
    expires_at: i64,
}

// ============================================================================
// Routes
// ============================================================================

fn create_router(state: AppState) -> Router {
    Router::new()
        // Public routes
        .route("/auth/login", post(login_handler))
        .route("/auth/refresh", post(refresh_handler))
        .route("/health", get(health_check))
        
        // Protected routes
        .route("/api/protected", get(protected_handler))
        .route("/api/user/profile", get(get_profile))
        
        // Apply auth middleware
        .route_layer(middleware::from_fn_with_state(
            state.auth_config.clone(),
            auth_middleware,
        ))
        .with_state(state)
}

// ============================================================================
// Handlers
// ============================================================================

async fn login_handler(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    let user = state.user_service
        .authenticate(&payload.email, &payload.password)
        .await
        .map_err(|_| AppError::InvalidCredentials)?;
    
    let access_token = generate_token(
        &user.id,
        &user.email,
        None,
        &state.auth_config,
    );
    
    let refresh_token = generate_refresh_token(
        &user.id,
        &user.email,
        &state.auth_config,
    );
    
    Ok(Json(LoginResponse {
        access_token: access_token.encode(),
        refresh_token: refresh_token.encode(),
        token_type: "Bearer".to_string(),
        expires_in: state.auth_config.token_expiry_seconds,
    }))
}

async fn protected_handler(
    ExtractUser(user): ExtractUser,
) -> Json<ProtectedResponse> {
    Json(ProtectedResponse {
        message: format!("Hello, {}!", user.email),
        user_id: user.account_id,
    })
}

// ============================================================================
// Middleware
// ============================================================================

async fn auth_middleware(
    State(config): State<Arc<AuthConfig>>,
    mut request: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = request
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;
    
    let token_str = auth_header
        .strip_prefix("Bearer ")
        .ok_or(StatusCode::UNAUTHORIZED)?;
    
    let token = AuthToken::decode(token_str)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;
    
    if !token.verify(&config.secret_key) {
        return Err(StatusCode::UNAUTHORIZED);
    }
    
    if token.payload.is_expired() {
        return Err(StatusCode::UNAUTHORIZED);
    }
    
    request.extensions_mut().insert(token.payload);
    Ok(next.run(request).await)
}

// ============================================================================
// Extractor
// ============================================================================

struct ExtractUser(AuthUser);

impl<S> FromRequestParts<S> for ExtractUser
where
    S: Send + Sync,
{
    type Rejection = StatusCode;
    
    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<AuthUser>()
            .cloned()
            .map(ExtractUser)
            .ok_or(StatusCode::UNAUTHORIZED)
    }
}
```

---

## 10. Key Takeaways

### Best Practices

1. **Use Middleware for Auth**: Centralizes authentication logic
2. **Custom Extractors**: Type-safe user extraction
3. **Error Types**: Consistent error handling with `IntoResponse`
4. **Stateless Tokens**: Scales horizontally
5. **Refresh Tokens**: Long-lived tokens for better UX

### Scaling Considerations

1. **Asymmetric Keys**: Use RS256 for better key management
2. **Token Revocation**: Implement token blacklisting
3. **Refresh Rotation**: Rotate refresh tokens on use
4. **Standard JWT**: Use standard JWT format for interoperability
5. **Rate Limiting**: Protect auth endpoints from abuse
6. **Audit Logging**: Track authentication events

### Security Considerations

1. **HTTPS Only**: Always use HTTPS in production
2. **Secure Storage**: Store tokens securely on client
3. **Token Expiration**: Use short-lived access tokens
4. **Signature Verification**: Always verify token signatures
5. **Input Validation**: Validate all inputs
6. **Error Messages**: Don't leak sensitive information

---

## References

- Axum Documentation: https://docs.rs/axum
- JWT Specification: https://tools.ietf.org/html/rfc7519
- jsonwebtoken Crate: https://docs.rs/jsonwebtoken
- Tower Middleware: https://docs.rs/tower

