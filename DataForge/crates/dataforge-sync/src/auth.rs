//! Authentication middleware for DataForge sync server
//!
//! Provides JWT-based authentication for sync endpoints.
//! Following Harbor patterns for API authentication.
//!
//! ## Usage
//!
//! ```rust,ignore
//! use dataforge_sync::auth::{AuthConfig, AuthMiddleware, AuthUser};
//!
//! // Create auth config
//! let config = AuthConfig::new("your-secret-key");
//!
//! // Add to Axum router
//! let app = Router::new()
//!     .route("/api/sync/push", post(push_handler))
//!     .layer(axum::middleware::from_fn_with_state(
//!         config.clone(),
//!         auth_middleware,
//!     ));
//! ```

use axum::{
    extract::{FromRequestParts, State},
    http::{request::Parts, StatusCode},
    middleware::Next,
    response::Response,
};
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::sync::Arc;
use thiserror::Error;
use tracing::{debug, warn};

/// Authentication configuration
#[derive(Debug, Clone)]
pub struct AuthConfig {
    /// Secret key for signing tokens
    pub secret_key: String,
    /// Token expiration duration in seconds (default: 1 hour)
    pub token_expiry_seconds: i64,
    /// Whether to skip authentication (for development only!)
    pub skip_auth: bool,
}

impl AuthConfig {
    /// Create a new auth config with a secret key
    pub fn new(secret_key: impl Into<String>) -> Self {
        Self {
            secret_key: secret_key.into(),
            token_expiry_seconds: 3600,
            skip_auth: false,
        }
    }

    /// Skip authentication (for development/testing ONLY)
    pub fn skip_auth(mut self) -> Self {
        self.skip_auth = true;
        self
    }

    /// Set token expiry duration
    pub fn with_expiry(mut self, seconds: i64) -> Self {
        self.token_expiry_seconds = seconds;
        self
    }
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            secret_key: String::new(),
            token_expiry_seconds: 3600,
            skip_auth: true, // Default to skip for dev
        }
    }
}

/// Authenticated user information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthUser {
    /// Account ID
    pub account_id: String,
    /// Email address
    pub email: String,
    /// Workspace ID (if scoped to a workspace)
    pub workspace_id: Option<String>,
    /// Token expiration timestamp
    pub expires_at: i64,
}

impl AuthUser {
    /// Check if the token has expired
    pub fn is_expired(&self) -> bool {
        Utc::now().timestamp() > self.expires_at
    }
}

/// Authentication token (simplified JWT-like structure)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthToken {
    /// Token payload
    pub payload: AuthUser,
    /// Token signature (HMAC-SHA256)
    pub signature: String,
}

impl AuthToken {
    /// Create a new authentication token
    pub fn new(user: AuthUser, secret_key: &str) -> Self {
        let payload_json = serde_json::to_string(&user).unwrap_or_default();
        let signature = Self::compute_signature(&payload_json, secret_key);
        Self {
            payload: user,
            signature,
        }
    }

    /// Verify the token signature
    pub fn verify(&self, secret_key: &str) -> bool {
        let payload_json = serde_json::to_string(&self.payload).unwrap_or_default();
        let expected_signature = Self::compute_signature(&payload_json, secret_key);
        self.signature == expected_signature
    }

    /// Compute HMAC-SHA256 signature
    fn compute_signature(data: &str, secret: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        hasher.update(secret.as_bytes());
        hex::encode(hasher.finalize())
    }

    /// Encode token to string for transport
    pub fn encode(&self) -> String {
        let json = serde_json::to_string(self).unwrap_or_default();
        base64_encode(&json)
    }

    /// Decode token from string
    pub fn decode(token: &str) -> Result<Self, AuthError> {
        let json = base64_decode(token)?;
        serde_json::from_str(&json).map_err(|_| AuthError::InvalidToken)
    }
}

/// Authentication errors
#[derive(Debug, Error)]
pub enum AuthError {
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
}

/// Authentication middleware for Axum
///
/// Validates Bearer tokens and extracts user information.
pub async fn auth_middleware(
    State(config): State<Arc<AuthConfig>>,
    mut request: axum::http::Request<axum::body::Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    // Skip auth if configured (development only!)
    if config.skip_auth {
        debug!("Auth skipped (skip_auth=true)");
        // Insert a dummy user for development
        request.extensions_mut().insert(AuthUser {
            account_id: "dev-user".to_string(),
            email: "dev@localhost".to_string(),
            workspace_id: None,
            expires_at: i64::MAX,
        });
        return Ok(next.run(request).await);
    }

    // Extract authorization header
    let auth_header = request
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Parse Bearer token
    let token_str = auth_header
        .strip_prefix("Bearer ")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Decode and verify token
    let token = match AuthToken::decode(token_str) {
        Ok(t) => t,
        Err(e) => {
            warn!("Invalid token: {}", e);
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    // Verify signature
    if !token.verify(&config.secret_key) {
        warn!("Invalid token signature");
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Check expiration
    if token.payload.is_expired() {
        warn!("Token has expired");
        return Err(StatusCode::UNAUTHORIZED);
    }

    debug!("Authenticated user: {}", token.payload.email);

    // Insert user into request extensions
    request.extensions_mut().insert(token.payload);

    Ok(next.run(request).await)
}

/// Axum extractor for authenticated user
#[derive(Debug, Clone)]
pub struct ExtractUser(pub AuthUser);

impl<S> FromRequestParts<S> for ExtractUser
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    fn from_request_parts<'life0, 'life1, 'async_trait>(
        parts: &'life0 mut Parts,
        _state: &'life1 S,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<Self, Self::Rejection>> + Send + 'async_trait>,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            parts
                .extensions
                .get::<AuthUser>()
                .cloned()
                .map(ExtractUser)
                .ok_or(StatusCode::UNAUTHORIZED)
        })
    }
}

/// Generate a new auth token for a user
pub fn generate_token(
    account_id: impl Into<String>,
    email: impl Into<String>,
    workspace_id: Option<String>,
    config: &AuthConfig,
) -> AuthToken {
    let user = AuthUser {
        account_id: account_id.into(),
        email: email.into(),
        workspace_id,
        expires_at: (Utc::now() + Duration::seconds(config.token_expiry_seconds)).timestamp(),
    };
    AuthToken::new(user, &config.secret_key)
}

/// Simple base64 encoding (URL-safe)
fn base64_encode(data: &str) -> String {
    use std::io::Write;
    let mut encoder = base64::write::EncoderStringWriter::new(&base64::engine::general_purpose::URL_SAFE_NO_PAD);
    encoder.write_all(data.as_bytes()).unwrap();
    encoder.into_inner()
}

/// Simple base64 decoding (URL-safe)
fn base64_decode(data: &str) -> Result<String, AuthError> {
    use base64::Engine;
    let bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(data)
        .map_err(|_| AuthError::InvalidToken)?;
    String::from_utf8(bytes).map_err(|_| AuthError::InvalidToken)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_creation_and_verification() {
        let config = AuthConfig::new("test-secret");
        let token = generate_token("user-123", "test@example.com", None, &config);

        assert!(token.verify(&config.secret_key));
        assert!(!token.payload.is_expired());
    }

    #[test]
    fn test_token_encode_decode() {
        let config = AuthConfig::new("test-secret");
        let token = generate_token("user-123", "test@example.com", Some("ws-456".to_string()), &config);

        let encoded = token.encode();
        let decoded = AuthToken::decode(&encoded).unwrap();

        assert_eq!(decoded.payload.account_id, "user-123");
        assert_eq!(decoded.payload.email, "test@example.com");
        assert_eq!(decoded.payload.workspace_id, Some("ws-456".to_string()));
    }

    #[test]
    fn test_invalid_signature() {
        let config = AuthConfig::new("test-secret");
        let token = generate_token("user-123", "test@example.com", None, &config);

        // Verify with wrong key
        assert!(!token.verify("wrong-secret"));
    }
}
