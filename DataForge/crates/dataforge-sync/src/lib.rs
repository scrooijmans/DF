//! DataForge Sync
//!
//! Git-like sync protocol for DataForge. Handles push/pull synchronization
//! between client and server.
//!
//! ## Sync Model
//!
//! DataForge uses a pull-based (Git-like) sync model:
//!
//! 1. Client makes changes locally (SQLite + blob store)
//! 2. Changes are tracked with incrementing version numbers
//! 3. On sync: push local changes, pull remote changes
//! 4. Conflicts resolved by version number (last-write-wins)
//!
//! This is simpler than real-time CRDT sync and better suited for
//! enterprise workflows where users don't need Figma-like collaboration.
//!
//! ## Server Module
//!
//! The [`server`] module provides a production-ready Axum-based sync server with:
//! - SQLite database for change persistence
//! - JWT authentication middleware
//! - S3/MinIO blob storage with presigned URLs
//! - Workspace-scoped push/pull endpoints
//!
//! ## Storage Backends
//!
//! Production deployments support S3/MinIO blob storage with presigned URLs:
//!
//! - **Presigned URLs**: Direct client-to-storage downloads/uploads
//! - **S3/MinIO**: Compatible with AWS S3 and MinIO
//! - **Content-Addressed**: Blobs stored by SHA-256 digest for deduplication
//!
//! See [`storage`] module for S3/MinIO presigned URL generation.

pub mod auth;
pub mod client;
pub mod protocol;
pub mod server;
pub mod storage;

pub use client::SyncClient;
pub use protocol::*;
pub use storage::{
	BatchPresignRequest, BatchPresignResponse, BlobMetadata, BlobStorage, PresignRequest,
	PresignResponse, PresignedBlobUrl, PresignedUrl, StorageBackend, StorageConfig, StorageError,
};
pub use auth::{
	auth_middleware, generate_token, AuthConfig, AuthError, AuthToken, AuthUser, ExtractUser,
};
pub use server::{AppState, ServerDb, ServerDbError, create_router};
