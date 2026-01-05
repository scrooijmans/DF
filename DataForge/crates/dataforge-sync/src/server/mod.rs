//! Production sync server module
//!
//! Provides a complete Axum-based sync server with:
//! - SQLite database for change persistence
//! - JWT authentication middleware
//! - S3/MinIO blob storage with presigned URLs
//! - Push/pull sync endpoints
//!
//! ## Architecture
//!
//! Following patterns from:
//! - Shuttle/Axum: Router composition, State injection
//! - Harbor: Presigned URL generation for blob storage
//! - Pijul: Change-based sync with dependency tracking
//! - ColaNode: Workspace-scoped operations

pub mod db;
pub mod handlers;
pub mod state;

pub use db::{ServerDb, ServerDbError};
pub use handlers::create_router;
pub use state::AppState;
