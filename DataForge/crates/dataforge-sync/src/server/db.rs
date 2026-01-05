//! Server-side database for sync state persistence
//!
//! Stores sync changes and workspace state for the sync server.
//! This is separate from the client-side database schema.

use chrono::{DateTime, Utc};
use rusqlite::{params, Connection};
use std::path::Path;
use std::sync::{Arc, Mutex};
use thiserror::Error;
use tracing::info;
use uuid::Uuid;

use crate::protocol::{EntityType, SyncAction, SyncChange, SyncConflict};

/// Server database errors
#[derive(Debug, Error)]
pub enum ServerDbError {
	#[error("Database error: {0}")]
	Database(#[from] rusqlite::Error),
	#[error("Lock error: {0}")]
	Lock(String),
	#[error("Not found: {0}")]
	NotFound(String),
}

/// Server database schema for sync state
const SERVER_SCHEMA: &str = r#"
-- ============================================================
-- SERVER SYNC CHANGES
-- ============================================================
-- Stores all changes pushed by clients, indexed by workspace

CREATE TABLE IF NOT EXISTS sync_changes (
	id TEXT PRIMARY KEY,
	workspace_id TEXT NOT NULL,
	entity_type TEXT NOT NULL,
	entity_id TEXT NOT NULL,
	action TEXT NOT NULL,
	version INTEGER NOT NULL,
	timestamp TEXT NOT NULL,
	user_id TEXT NOT NULL,
	data TEXT,
	server_version INTEGER NOT NULL,
	created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_sync_changes_workspace ON sync_changes(workspace_id);
CREATE INDEX IF NOT EXISTS idx_sync_changes_server_version ON sync_changes(workspace_id, server_version);
CREATE INDEX IF NOT EXISTS idx_sync_changes_entity ON sync_changes(workspace_id, entity_type, entity_id);

-- ============================================================
-- WORKSPACE VERSIONS
-- ============================================================
-- Tracks the current version for each workspace

CREATE TABLE IF NOT EXISTS workspace_versions (
	workspace_id TEXT PRIMARY KEY,
	current_version INTEGER NOT NULL DEFAULT 0,
	updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- ============================================================
-- BLOB REGISTRY (server-side)
-- ============================================================
-- Tracks blobs that have been uploaded to storage

CREATE TABLE IF NOT EXISTS blob_registry (
	hash TEXT PRIMARY KEY,
	workspace_id TEXT NOT NULL,
	size_bytes INTEGER,
	uploaded_by TEXT NOT NULL,
	uploaded_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_blob_registry_workspace ON blob_registry(workspace_id);
"#;

/// Server database wrapper
pub struct ServerDb {
	conn: Arc<Mutex<Connection>>,
}

impl ServerDb {
	/// Open or create a server database at the given path
	pub fn open(path: &Path) -> Result<Self, ServerDbError> {
		let conn = Connection::open(path)?;

		// Enable foreign keys and WAL mode
		conn.execute_batch("PRAGMA foreign_keys = ON; PRAGMA journal_mode = WAL;")?;

		// Initialize schema
		conn.execute_batch(SERVER_SCHEMA)?;

		info!("Server database initialized at {:?}", path);

		Ok(Self {
			conn: Arc::new(Mutex::new(conn)),
		})
	}

	/// Open an in-memory database (for testing)
	pub fn open_memory() -> Result<Self, ServerDbError> {
		let conn = Connection::open_in_memory()?;
		conn.execute_batch("PRAGMA foreign_keys = ON;")?;
		conn.execute_batch(SERVER_SCHEMA)?;

		Ok(Self {
			conn: Arc::new(Mutex::new(conn)),
		})
	}

	/// Get current version for a workspace
	pub fn get_workspace_version(&self, workspace_id: &str) -> Result<i64, ServerDbError> {
		let conn = self.conn.lock().map_err(|e| ServerDbError::Lock(e.to_string()))?;

		let version: Option<i64> = conn
			.query_row(
				"SELECT current_version FROM workspace_versions WHERE workspace_id = ?1",
				params![workspace_id],
				|row| row.get(0),
			)
			.ok();

		Ok(version.unwrap_or(0))
	}

	/// Store a batch of changes and return (accepted, conflicts)
	pub fn push_changes(
		&self,
		workspace_id: &str,
		changes: &[SyncChange],
		_client_version: i64,
	) -> Result<(Vec<Uuid>, Vec<SyncConflict>, i64), ServerDbError> {
		let mut conn = self.conn.lock().map_err(|e| ServerDbError::Lock(e.to_string()))?;
		let tx = conn.transaction()?;

		let mut accepted = Vec::new();
		let mut conflicts = Vec::new();

		for change in changes {
			// Check for conflicts: is there a newer version of this entity on the server?
			let server_change: Option<(i64, Option<String>)> = tx
				.query_row(
					r#"SELECT version, data FROM sync_changes
					   WHERE workspace_id = ?1 AND entity_type = ?2 AND entity_id = ?3
					   ORDER BY version DESC LIMIT 1"#,
					params![
						workspace_id,
						entity_type_to_string(change.entity_type),
						change.entity_id.to_string()
					],
					|row| Ok((row.get(0)?, row.get(1)?)),
				)
				.ok();

			if let Some((server_version, server_data)) = server_change {
				if server_version > change.version {
					// Conflict detected
					conflicts.push(SyncConflict {
						change_id: change.id,
						entity_type: change.entity_type,
						entity_id: change.entity_id,
						server_version,
						client_version: change.version,
						server_data,
					});
					continue;
				}
			}

			// No conflict - accept the change
			let new_server_version = Self::increment_version_tx(&tx, workspace_id)?;

			tx.execute(
				r#"INSERT INTO sync_changes
				   (id, workspace_id, entity_type, entity_id, action, version, timestamp, user_id, data, server_version)
				   VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)"#,
				params![
					change.id.to_string(),
					workspace_id,
					entity_type_to_string(change.entity_type),
					change.entity_id.to_string(),
					action_to_string(change.action),
					change.version,
					change.timestamp.to_rfc3339(),
					change.user_id.to_string(),
					change.data,
					new_server_version
				],
			)?;

			accepted.push(change.id);
		}

		let final_version = tx
			.query_row(
				"SELECT current_version FROM workspace_versions WHERE workspace_id = ?1",
				params![workspace_id],
				|row| row.get(0),
			)
			.unwrap_or(0);

		tx.commit()?;

		Ok((accepted, conflicts, final_version))
	}

	/// Internal version increment within transaction
	fn increment_version_tx(tx: &rusqlite::Transaction, workspace_id: &str) -> Result<i64, ServerDbError> {
		tx.execute(
			r#"INSERT INTO workspace_versions (workspace_id, current_version, updated_at)
			   VALUES (?1, 1, datetime('now'))
			   ON CONFLICT(workspace_id) DO UPDATE SET
			   current_version = current_version + 1,
			   updated_at = datetime('now')"#,
			params![workspace_id],
		)?;

		let version: i64 = tx.query_row(
			"SELECT current_version FROM workspace_versions WHERE workspace_id = ?1",
			params![workspace_id],
			|row| row.get(0),
		)?;

		Ok(version)
	}

	/// Pull changes since a given version
	pub fn pull_changes(
		&self,
		workspace_id: &str,
		from_version: i64,
		limit: Option<i64>,
	) -> Result<(Vec<SyncChange>, i64, bool), ServerDbError> {
		let conn = self.conn.lock().map_err(|e| ServerDbError::Lock(e.to_string()))?;

		let limit = limit.unwrap_or(100);

		let mut stmt = conn.prepare(
			r#"SELECT id, entity_type, entity_id, action, version, timestamp, user_id, data
			   FROM sync_changes
			   WHERE workspace_id = ?1 AND server_version > ?2
			   ORDER BY server_version ASC
			   LIMIT ?3"#,
		)?;

		let changes: Vec<SyncChange> = stmt
			.query_map(params![workspace_id, from_version, limit + 1], |row| {
				Ok(SyncChange {
					id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap_or_default(),
					entity_type: string_to_entity_type(&row.get::<_, String>(1)?),
					entity_id: Uuid::parse_str(&row.get::<_, String>(2)?).unwrap_or_default(),
					action: string_to_action(&row.get::<_, String>(3)?),
					version: row.get(4)?,
					timestamp: DateTime::parse_from_rfc3339(&row.get::<_, String>(5)?)
						.map(|dt| dt.with_timezone(&Utc))
						.unwrap_or_else(|_| Utc::now()),
					user_id: Uuid::parse_str(&row.get::<_, String>(6)?).unwrap_or_default(),
					data: row.get(7)?,
				})
			})?
			.filter_map(|r| r.ok())
			.collect();

		// Check if there are more results
		let has_more = changes.len() as i64 > limit;
		let changes: Vec<SyncChange> = changes.into_iter().take(limit as usize).collect();

		// Get current version (query inline to avoid deadlock)
		let current_version: i64 = conn
			.query_row(
				"SELECT current_version FROM workspace_versions WHERE workspace_id = ?1",
				params![workspace_id],
				|row| row.get(0),
			)
			.unwrap_or(0);

		Ok((changes, current_version, has_more))
	}

	/// Register a blob as uploaded
	pub fn register_blob(
		&self,
		hash: &str,
		workspace_id: &str,
		size_bytes: Option<i64>,
		uploaded_by: &str,
	) -> Result<(), ServerDbError> {
		let conn = self.conn.lock().map_err(|e| ServerDbError::Lock(e.to_string()))?;

		conn.execute(
			r#"INSERT OR REPLACE INTO blob_registry (hash, workspace_id, size_bytes, uploaded_by, uploaded_at)
			   VALUES (?1, ?2, ?3, ?4, datetime('now'))"#,
			params![hash, workspace_id, size_bytes, uploaded_by],
		)?;

		Ok(())
	}

	/// Check if a blob exists
	pub fn blob_exists(&self, hash: &str) -> Result<bool, ServerDbError> {
		let conn = self.conn.lock().map_err(|e| ServerDbError::Lock(e.to_string()))?;

		let exists: bool = conn
			.query_row(
				"SELECT 1 FROM blob_registry WHERE hash = ?1",
				params![hash],
				|_| Ok(true),
			)
			.unwrap_or(false);

		Ok(exists)
	}

	/// Get list of missing blobs from a list of hashes
	pub fn get_missing_blobs(&self, hashes: &[String]) -> Result<Vec<String>, ServerDbError> {
		if hashes.is_empty() {
			return Ok(Vec::new());
		}

		let conn = self.conn.lock().map_err(|e| ServerDbError::Lock(e.to_string()))?;

		let mut missing = Vec::new();
		for hash in hashes {
			let exists: bool = conn
				.query_row(
					"SELECT 1 FROM blob_registry WHERE hash = ?1",
					params![hash],
					|_| Ok(true),
				)
				.unwrap_or(false);

			if !exists {
				missing.push(hash.clone());
			}
		}

		Ok(missing)
	}
}

// Helper functions for entity type serialization
fn entity_type_to_string(et: EntityType) -> &'static str {
	match et {
		EntityType::Project => "project",
		EntityType::Well => "well",
		EntityType::Curve => "curve",
		EntityType::CurveMetadata => "curve_metadata",
	}
}

fn string_to_entity_type(s: &str) -> EntityType {
	match s {
		"project" => EntityType::Project,
		"well" => EntityType::Well,
		"curve" => EntityType::Curve,
		"curve_metadata" => EntityType::CurveMetadata,
		_ => EntityType::Project, // Default fallback
	}
}

fn action_to_string(action: SyncAction) -> &'static str {
	match action {
		SyncAction::Create => "create",
		SyncAction::Update => "update",
		SyncAction::Delete => "delete",
	}
}

fn string_to_action(s: &str) -> SyncAction {
	match s {
		"create" => SyncAction::Create,
		"update" => SyncAction::Update,
		"delete" => SyncAction::Delete,
		_ => SyncAction::Update, // Default fallback
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use chrono::Utc;

	fn create_test_change(entity_type: EntityType, entity_id: Uuid, version: i64) -> SyncChange {
		SyncChange {
			id: Uuid::new_v4(),
			entity_type,
			entity_id,
			action: SyncAction::Create,
			version,
			timestamp: Utc::now(),
			user_id: Uuid::new_v4(),
			data: Some(r#"{"name": "test"}"#.to_string()),
		}
	}

	#[test]
	fn test_open_memory_db() {
		let db = ServerDb::open_memory().expect("Failed to open memory db");
		let version = db.get_workspace_version("test-workspace").unwrap();
		assert_eq!(version, 0);
	}

	#[test]
	fn test_push_and_pull_changes() {
		let db = ServerDb::open_memory().expect("Failed to open memory db");
		let workspace_id = "test-workspace";

		// Push some changes
		let entity_id = Uuid::new_v4();
		let changes = vec![
			create_test_change(EntityType::Well, entity_id, 1),
		];

		let (accepted, conflicts, version) = db.push_changes(workspace_id, &changes, 0).unwrap();

		assert_eq!(accepted.len(), 1);
		assert!(conflicts.is_empty());
		assert_eq!(version, 1);

		// Pull changes
		let (pulled, current_version, has_more) = db.pull_changes(workspace_id, 0, None).unwrap();

		assert_eq!(pulled.len(), 1);
		assert_eq!(current_version, 1);
		assert!(!has_more);
	}

	#[test]
	fn test_conflict_detection() {
		let db = ServerDb::open_memory().expect("Failed to open memory db");
		let workspace_id = "test-workspace";
		let entity_id = Uuid::new_v4();

		// Push first change
		let changes1 = vec![create_test_change(EntityType::Well, entity_id, 1)];
		let (accepted, _, _) = db.push_changes(workspace_id, &changes1, 0).unwrap();
		assert_eq!(accepted.len(), 1);

		// Try to push older change - should conflict
		let changes2 = vec![create_test_change(EntityType::Well, entity_id, 0)];
		let (accepted, conflicts, _) = db.push_changes(workspace_id, &changes2, 0).unwrap();

		assert!(accepted.is_empty());
		assert_eq!(conflicts.len(), 1);
		assert_eq!(conflicts[0].server_version, 1);
	}

	#[test]
	fn test_blob_registry() {
		let db = ServerDb::open_memory().expect("Failed to open memory db");

		// Register a blob
		db.register_blob("abc123", "workspace-1", Some(1024), "user-1").unwrap();

		// Check it exists
		assert!(db.blob_exists("abc123").unwrap());
		assert!(!db.blob_exists("xyz789").unwrap());

		// Check missing blobs
		let missing = db.get_missing_blobs(&["abc123".to_string(), "xyz789".to_string()]).unwrap();
		assert_eq!(missing, vec!["xyz789"]);
	}

	#[test]
	fn test_pull_pagination() {
		let db = ServerDb::open_memory().expect("Failed to open memory db");
		let workspace_id = "test-workspace";

		// Push 5 changes
		for i in 0..5 {
			let changes = vec![create_test_change(EntityType::Well, Uuid::new_v4(), i)];
			db.push_changes(workspace_id, &changes, 0).unwrap();
		}

		// Pull with limit of 2
		let (pulled, _, has_more) = db.pull_changes(workspace_id, 0, Some(2)).unwrap();

		assert_eq!(pulled.len(), 2);
		assert!(has_more);

		// Pull next batch
		let (pulled2, _, has_more2) = db.pull_changes(workspace_id, 2, Some(2)).unwrap();

		assert_eq!(pulled2.len(), 2);
		assert!(has_more2);
	}
}
