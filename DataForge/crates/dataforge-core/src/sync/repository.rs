//! SQLite implementations of sync repository traits

use crate::models::{
    ConflictResolution, ConflictStrategy, SyncAction, SyncConflict, SyncQueueEntry, SyncState,
    SyncStatus,
};
use crate::Result;
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, OptionalExtension};
use uuid::Uuid;

use super::traits::{ConflictRepository, SyncQueueRepository, SyncStateRepository};

/// Helper to convert rusqlite errors to our Error type
fn db_err(e: rusqlite::Error) -> crate::Error {
    crate::Error::Database(e)
}

// ============================================================
// SQLITE SYNC STATE REPOSITORY
// ============================================================

/// SQLite implementation of SyncStateRepository
pub struct SqliteSyncStateRepo<'a> {
    conn: &'a Connection,
}

impl<'a> SqliteSyncStateRepo<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    /// Parse a datetime string from SQLite
    fn parse_datetime(s: &str) -> Option<DateTime<Utc>> {
        DateTime::parse_from_rfc3339(s)
            .ok()
            .map(|dt| dt.with_timezone(&Utc))
            .or_else(|| {
                // SQLite datetime format: "YYYY-MM-DD HH:MM:SS"
                chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S")
                    .ok()
                    .map(|ndt| ndt.and_utc())
            })
    }
}

impl<'a> SyncStateRepository for SqliteSyncStateRepo<'a> {
    fn get_state(&self, workspace_id: &Uuid) -> Result<Option<SyncState>> {
        let result = self
            .conn
            .query_row(
                r#"
                SELECT workspace_id, server_url, last_sync_version, last_sync_at,
                       last_push_at, last_pull_at, sync_status, last_error,
                       conflict_strategy, created_at, updated_at
                FROM sync_state
                WHERE workspace_id = ?1
                "#,
                params![workspace_id.to_string()],
                |row| {
                    let workspace_id_str: String = row.get(0)?;
                    let server_url: String = row.get(1)?;
                    let last_sync_version: i64 = row.get(2)?;
                    let last_sync_at: Option<String> = row.get(3)?;
                    let last_push_at: Option<String> = row.get(4)?;
                    let last_pull_at: Option<String> = row.get(5)?;
                    let sync_status: String = row.get(6)?;
                    let last_error: Option<String> = row.get(7)?;
                    let conflict_strategy: Option<String> = row.get(8)?;
                    let created_at: String = row.get(9)?;
                    let updated_at: String = row.get(10)?;

                    Ok(SyncState {
                        workspace_id: Uuid::parse_str(&workspace_id_str).unwrap_or_default(),
                        server_url,
                        last_sync_version,
                        last_sync_at: last_sync_at.and_then(|s| Self::parse_datetime(&s)),
                        last_push_at: last_push_at.and_then(|s| Self::parse_datetime(&s)),
                        last_pull_at: last_pull_at.and_then(|s| Self::parse_datetime(&s)),
                        sync_status: sync_status.parse().unwrap_or_default(),
                        last_error,
                        conflict_strategy: conflict_strategy
                            .and_then(|s| s.parse().ok())
                            .unwrap_or_default(),
                        created_at: Self::parse_datetime(&created_at).unwrap_or_else(Utc::now),
                        updated_at: Self::parse_datetime(&updated_at).unwrap_or_else(Utc::now),
                    })
                },
            )
            .optional()
            .map_err(db_err)?;

        Ok(result)
    }

    fn save_state(&self, state: &SyncState) -> Result<()> {
        self.conn
            .execute(
                r#"
                INSERT INTO sync_state (
                    workspace_id, server_url, last_sync_version, last_sync_at,
                    last_push_at, last_pull_at, sync_status, last_error,
                    conflict_strategy, created_at, updated_at
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
                ON CONFLICT(workspace_id) DO UPDATE SET
                    server_url = excluded.server_url,
                    last_sync_version = excluded.last_sync_version,
                    last_sync_at = excluded.last_sync_at,
                    last_push_at = excluded.last_push_at,
                    last_pull_at = excluded.last_pull_at,
                    sync_status = excluded.sync_status,
                    last_error = excluded.last_error,
                    conflict_strategy = excluded.conflict_strategy,
                    updated_at = excluded.updated_at
                "#,
                params![
                    state.workspace_id.to_string(),
                    state.server_url,
                    state.last_sync_version,
                    state.last_sync_at.map(|dt| dt.to_rfc3339()),
                    state.last_push_at.map(|dt| dt.to_rfc3339()),
                    state.last_pull_at.map(|dt| dt.to_rfc3339()),
                    state.sync_status.to_string(),
                    state.last_error,
                    state.conflict_strategy.to_string(),
                    state.created_at.to_rfc3339(),
                    Utc::now().to_rfc3339(),
                ],
            )
            .map_err(db_err)?;

        Ok(())
    }

    fn update_last_sync(&self, workspace_id: &Uuid, version: i64) -> Result<()> {
        let now = Utc::now().to_rfc3339();
        self.conn
            .execute(
                r#"
                UPDATE sync_state
                SET last_sync_version = ?1, last_sync_at = ?2, updated_at = ?2
                WHERE workspace_id = ?3
                "#,
                params![version, now, workspace_id.to_string()],
            )
            .map_err(db_err)?;

        Ok(())
    }

    fn update_status(&self, workspace_id: &Uuid, status: SyncStatus) -> Result<()> {
        let now = Utc::now().to_rfc3339();
        self.conn
            .execute(
                r#"
                UPDATE sync_state
                SET sync_status = ?1, updated_at = ?2
                WHERE workspace_id = ?3
                "#,
                params![status.to_string(), now, workspace_id.to_string()],
            )
            .map_err(db_err)?;

        Ok(())
    }

    fn set_error(&self, workspace_id: &Uuid, error: &str) -> Result<()> {
        let now = Utc::now().to_rfc3339();
        self.conn
            .execute(
                r#"
                UPDATE sync_state
                SET sync_status = 'error', last_error = ?1, updated_at = ?2
                WHERE workspace_id = ?3
                "#,
                params![error, now, workspace_id.to_string()],
            )
            .map_err(db_err)?;

        Ok(())
    }

    fn clear_error(&self, workspace_id: &Uuid) -> Result<()> {
        let now = Utc::now().to_rfc3339();
        self.conn
            .execute(
                r#"
                UPDATE sync_state
                SET sync_status = 'idle', last_error = NULL, updated_at = ?1
                WHERE workspace_id = ?2
                "#,
                params![now, workspace_id.to_string()],
            )
            .map_err(db_err)?;

        Ok(())
    }

    fn delete_state(&self, workspace_id: &Uuid) -> Result<()> {
        self.conn
            .execute(
                "DELETE FROM sync_state WHERE workspace_id = ?1",
                params![workspace_id.to_string()],
            )
            .map_err(db_err)?;

        Ok(())
    }
}

// ============================================================
// SQLITE SYNC QUEUE REPOSITORY
// ============================================================

/// SQLite implementation of SyncQueueRepository
pub struct SqliteSyncQueueRepo<'a> {
    conn: &'a Connection,
}

impl<'a> SqliteSyncQueueRepo<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    /// Parse a datetime string from SQLite
    fn parse_datetime(s: &str) -> Option<DateTime<Utc>> {
        DateTime::parse_from_rfc3339(s)
            .ok()
            .map(|dt| dt.with_timezone(&Utc))
            .or_else(|| {
                chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S")
                    .ok()
                    .map(|ndt| ndt.and_utc())
            })
    }

    /// Parse blob_hashes JSON array
    fn parse_blob_hashes(s: Option<String>) -> Option<Vec<String>> {
        s.and_then(|json| serde_json::from_str(&json).ok())
    }

    /// Serialize blob_hashes to JSON array
    fn serialize_blob_hashes(hashes: Option<&Vec<String>>) -> Option<String> {
        hashes.map(|h| serde_json::to_string(h).unwrap_or_default())
    }
}

impl<'a> SyncQueueRepository for SqliteSyncQueueRepo<'a> {
    fn enqueue(&self, entry: &SyncQueueEntry) -> Result<i64> {
        self.conn
            .execute(
                r#"
                INSERT INTO sync_queue (
                    workspace_id, entity_type, entity_id, action, version,
                    payload, blob_hashes, priority, created_at, attempts,
                    last_attempt_at, last_error, synced_at
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)
                ON CONFLICT(workspace_id, entity_type, entity_id) DO UPDATE SET
                    action = excluded.action,
                    version = excluded.version,
                    payload = excluded.payload,
                    blob_hashes = excluded.blob_hashes,
                    priority = MAX(sync_queue.priority, excluded.priority),
                    synced_at = NULL,
                    last_error = NULL,
                    attempts = 0
                "#,
                params![
                    entry.workspace_id.to_string(),
                    entry.entity_type,
                    entry.entity_id.to_string(),
                    entry.action.to_string(),
                    entry.version,
                    entry.payload,
                    Self::serialize_blob_hashes(entry.blob_hashes.as_ref()),
                    entry.priority,
                    entry.created_at.to_rfc3339(),
                    entry.attempts,
                    entry.last_attempt_at.map(|dt| dt.to_rfc3339()),
                    entry.last_error,
                    entry.synced_at.map(|dt| dt.to_rfc3339()),
                ],
            )
            .map_err(db_err)?;

        let id = self.conn.last_insert_rowid();
        Ok(id)
    }

    fn get_pending(&self, workspace_id: &Uuid, limit: usize) -> Result<Vec<SyncQueueEntry>> {
        let mut stmt = self
            .conn
            .prepare(
                r#"
                SELECT id, workspace_id, entity_type, entity_id, action, version,
                       payload, blob_hashes, priority, created_at, attempts,
                       last_attempt_at, last_error, synced_at
                FROM sync_queue
                WHERE workspace_id = ?1 AND synced_at IS NULL
                ORDER BY priority DESC, created_at ASC
                LIMIT ?2
                "#,
            )
            .map_err(db_err)?;

        let entries = stmt
            .query_map(params![workspace_id.to_string(), limit as i64], |row| {
                let id: i64 = row.get(0)?;
                let workspace_id_str: String = row.get(1)?;
                let entity_type: String = row.get(2)?;
                let entity_id_str: String = row.get(3)?;
                let action: String = row.get(4)?;
                let version: i64 = row.get(5)?;
                let payload: Option<String> = row.get(6)?;
                let blob_hashes: Option<String> = row.get(7)?;
                let priority: i32 = row.get(8)?;
                let created_at: String = row.get(9)?;
                let attempts: i32 = row.get(10)?;
                let last_attempt_at: Option<String> = row.get(11)?;
                let last_error: Option<String> = row.get(12)?;
                let synced_at: Option<String> = row.get(13)?;

                Ok(SyncQueueEntry {
                    id,
                    workspace_id: Uuid::parse_str(&workspace_id_str).unwrap_or_default(),
                    entity_type,
                    entity_id: Uuid::parse_str(&entity_id_str).unwrap_or_default(),
                    action: action.parse().unwrap_or(SyncAction::Update),
                    version,
                    payload,
                    blob_hashes: Self::parse_blob_hashes(blob_hashes),
                    priority,
                    created_at: Self::parse_datetime(&created_at).unwrap_or_else(Utc::now),
                    attempts,
                    last_attempt_at: last_attempt_at.and_then(|s| Self::parse_datetime(&s)),
                    last_error,
                    synced_at: synced_at.and_then(|s| Self::parse_datetime(&s)),
                })
            })
            .map_err(db_err)?
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(db_err)?;

        Ok(entries)
    }

    fn get_all_pending(&self, workspace_id: &Uuid) -> Result<Vec<SyncQueueEntry>> {
        self.get_pending(workspace_id, i64::MAX as usize)
    }

    fn count_pending(&self, workspace_id: &Uuid) -> Result<i64> {
        let count: i64 = self
            .conn
            .query_row(
                "SELECT COUNT(*) FROM sync_queue WHERE workspace_id = ?1 AND synced_at IS NULL",
                params![workspace_id.to_string()],
                |row| row.get(0),
            )
            .map_err(db_err)?;

        Ok(count)
    }

    fn mark_synced(&self, ids: &[i64]) -> Result<()> {
        if ids.is_empty() {
            return Ok(());
        }

        let now = Utc::now().to_rfc3339();
        let placeholders: String = ids.iter().map(|_| "?").collect::<Vec<_>>().join(",");
        let sql = format!(
            "UPDATE sync_queue SET synced_at = ?1 WHERE id IN ({})",
            placeholders
        );

        let mut params: Vec<Box<dyn rusqlite::ToSql>> = vec![Box::new(now)];
        for id in ids {
            params.push(Box::new(*id));
        }

        self.conn
            .execute(&sql, rusqlite::params_from_iter(params.iter().map(|p| p.as_ref())))
            .map_err(db_err)?;

        Ok(())
    }

    fn mark_failed(&self, id: i64, error: &str) -> Result<()> {
        let now = Utc::now().to_rfc3339();
        self.conn
            .execute(
                r#"
                UPDATE sync_queue
                SET last_error = ?1, last_attempt_at = ?2, attempts = attempts + 1
                WHERE id = ?3
                "#,
                params![error, now, id],
            )
            .map_err(db_err)?;

        Ok(())
    }

    fn increment_attempts(&self, id: i64) -> Result<()> {
        let now = Utc::now().to_rfc3339();
        self.conn
            .execute(
                "UPDATE sync_queue SET attempts = attempts + 1, last_attempt_at = ?1 WHERE id = ?2",
                params![now, id],
            )
            .map_err(db_err)?;

        Ok(())
    }

    fn clear_synced(&self, workspace_id: &Uuid) -> Result<()> {
        self.conn
            .execute(
                "DELETE FROM sync_queue WHERE workspace_id = ?1 AND synced_at IS NOT NULL",
                params![workspace_id.to_string()],
            )
            .map_err(db_err)?;

        Ok(())
    }

    fn delete(&self, id: i64) -> Result<()> {
        self.conn
            .execute("DELETE FROM sync_queue WHERE id = ?1", params![id])
            .map_err(db_err)?;

        Ok(())
    }

    fn delete_for_entity(
        &self,
        workspace_id: &Uuid,
        entity_type: &str,
        entity_id: &Uuid,
    ) -> Result<()> {
        self.conn
            .execute(
                r#"
                DELETE FROM sync_queue
                WHERE workspace_id = ?1 AND entity_type = ?2 AND entity_id = ?3
                "#,
                params![workspace_id.to_string(), entity_type, entity_id.to_string()],
            )
            .map_err(db_err)?;

        Ok(())
    }
}

// ============================================================
// SQLITE CONFLICT REPOSITORY
// ============================================================

/// SQLite implementation of ConflictRepository
pub struct SqliteConflictRepo<'a> {
    conn: &'a Connection,
}

impl<'a> SqliteConflictRepo<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    /// Parse a datetime string from SQLite
    fn parse_datetime(s: &str) -> Option<DateTime<Utc>> {
        DateTime::parse_from_rfc3339(s)
            .ok()
            .map(|dt| dt.with_timezone(&Utc))
            .or_else(|| {
                chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S")
                    .ok()
                    .map(|ndt| ndt.and_utc())
            })
    }
}

impl<'a> ConflictRepository for SqliteConflictRepo<'a> {
    fn save_conflict(&self, conflict: &SyncConflict) -> Result<i64> {
        self.conn
            .execute(
                r#"
                INSERT INTO sync_conflicts (
                    workspace_id, entity_type, entity_id, local_version, local_data,
                    remote_version, remote_data, resolution, resolved_by, resolved_at,
                    created_at
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
                "#,
                params![
                    conflict.workspace_id.to_string(),
                    conflict.entity_type,
                    conflict.entity_id.to_string(),
                    conflict.local_version,
                    conflict.local_data,
                    conflict.remote_version,
                    conflict.remote_data,
                    conflict.resolution.to_string(),
                    conflict.resolved_by.map(|id| id.to_string()),
                    conflict.resolved_at.map(|dt| dt.to_rfc3339()),
                    conflict.created_at.to_rfc3339(),
                ],
            )
            .map_err(db_err)?;

        let id = self.conn.last_insert_rowid();
        Ok(id)
    }

    fn get_pending(&self, workspace_id: &Uuid) -> Result<Vec<SyncConflict>> {
        let mut stmt = self
            .conn
            .prepare(
                r#"
                SELECT id, workspace_id, entity_type, entity_id, local_version, local_data,
                       remote_version, remote_data, resolution, resolved_by, resolved_at,
                       created_at
                FROM sync_conflicts
                WHERE workspace_id = ?1 AND resolution = 'pending'
                ORDER BY created_at ASC
                "#,
            )
            .map_err(db_err)?;

        let conflicts = stmt
            .query_map(params![workspace_id.to_string()], |row| {
                let id: i64 = row.get(0)?;
                let workspace_id_str: String = row.get(1)?;
                let entity_type: String = row.get(2)?;
                let entity_id_str: String = row.get(3)?;
                let local_version: i64 = row.get(4)?;
                let local_data: String = row.get(5)?;
                let remote_version: i64 = row.get(6)?;
                let remote_data: String = row.get(7)?;
                let resolution: String = row.get(8)?;
                let resolved_by: Option<String> = row.get(9)?;
                let resolved_at: Option<String> = row.get(10)?;
                let created_at: String = row.get(11)?;

                Ok(SyncConflict {
                    id,
                    workspace_id: Uuid::parse_str(&workspace_id_str).unwrap_or_default(),
                    entity_type,
                    entity_id: Uuid::parse_str(&entity_id_str).unwrap_or_default(),
                    local_version,
                    local_data,
                    remote_version,
                    remote_data,
                    resolution: resolution.parse().unwrap_or(ConflictResolution::Pending),
                    resolved_by: resolved_by.and_then(|s| Uuid::parse_str(&s).ok()),
                    resolved_at: resolved_at.and_then(|s| Self::parse_datetime(&s)),
                    created_at: Self::parse_datetime(&created_at).unwrap_or_else(Utc::now),
                })
            })
            .map_err(db_err)?
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(db_err)?;

        Ok(conflicts)
    }

    fn get_by_id(&self, id: i64) -> Result<Option<SyncConflict>> {
        let result = self
            .conn
            .query_row(
                r#"
                SELECT id, workspace_id, entity_type, entity_id, local_version, local_data,
                       remote_version, remote_data, resolution, resolved_by, resolved_at,
                       created_at
                FROM sync_conflicts
                WHERE id = ?1
                "#,
                params![id],
                |row| {
                    let id: i64 = row.get(0)?;
                    let workspace_id_str: String = row.get(1)?;
                    let entity_type: String = row.get(2)?;
                    let entity_id_str: String = row.get(3)?;
                    let local_version: i64 = row.get(4)?;
                    let local_data: String = row.get(5)?;
                    let remote_version: i64 = row.get(6)?;
                    let remote_data: String = row.get(7)?;
                    let resolution: String = row.get(8)?;
                    let resolved_by: Option<String> = row.get(9)?;
                    let resolved_at: Option<String> = row.get(10)?;
                    let created_at: String = row.get(11)?;

                    Ok(SyncConflict {
                        id,
                        workspace_id: Uuid::parse_str(&workspace_id_str).unwrap_or_default(),
                        entity_type,
                        entity_id: Uuid::parse_str(&entity_id_str).unwrap_or_default(),
                        local_version,
                        local_data,
                        remote_version,
                        remote_data,
                        resolution: resolution.parse().unwrap_or(ConflictResolution::Pending),
                        resolved_by: resolved_by.and_then(|s| Uuid::parse_str(&s).ok()),
                        resolved_at: resolved_at.and_then(|s| Self::parse_datetime(&s)),
                        created_at: Self::parse_datetime(&created_at).unwrap_or_else(Utc::now),
                    })
                },
            )
            .optional()
            .map_err(db_err)?;

        Ok(result)
    }

    fn count_pending(&self, workspace_id: &Uuid) -> Result<i64> {
        let count: i64 = self
            .conn
            .query_row(
                "SELECT COUNT(*) FROM sync_conflicts WHERE workspace_id = ?1 AND resolution = 'pending'",
                params![workspace_id.to_string()],
                |row| row.get(0),
            )
            .map_err(db_err)?;

        Ok(count)
    }

    fn resolve(
        &self,
        id: i64,
        resolution: ConflictResolution,
        resolved_by: &Uuid,
    ) -> Result<()> {
        let now = Utc::now().to_rfc3339();
        self.conn
            .execute(
                r#"
                UPDATE sync_conflicts
                SET resolution = ?1, resolved_by = ?2, resolved_at = ?3
                WHERE id = ?4
                "#,
                params![resolution.to_string(), resolved_by.to_string(), now, id],
            )
            .map_err(db_err)?;

        Ok(())
    }

    fn delete(&self, id: i64) -> Result<()> {
        self.conn
            .execute("DELETE FROM sync_conflicts WHERE id = ?1", params![id])
            .map_err(db_err)?;

        Ok(())
    }

    fn clear_resolved(&self, workspace_id: &Uuid) -> Result<()> {
        self.conn
            .execute(
                "DELETE FROM sync_conflicts WHERE workspace_id = ?1 AND resolution != 'pending'",
                params![workspace_id.to_string()],
            )
            .map_err(db_err)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::open_memory_db;

    #[test]
    fn test_sync_state_crud() {
        let conn = open_memory_db().expect("Failed to create test database");
        let repo = SqliteSyncStateRepo::new(&conn);

        // First, create a workspace to reference
        let workspace_id = Uuid::new_v4();
        let account_id = Uuid::new_v4();

        // Insert test account and workspace
        conn.execute(
            "INSERT INTO accounts (id, email, name, status) VALUES (?1, ?2, ?3, 1)",
            params![account_id.to_string(), "test@example.com", "Test User"],
        )
        .unwrap();

        conn.execute(
            "INSERT INTO workspaces (id, name, owner_account_id) VALUES (?1, ?2, ?3)",
            params![workspace_id.to_string(), "Test Workspace", account_id.to_string()],
        )
        .unwrap();

        // Test create
        let state = SyncState::new(workspace_id, "https://sync.example.com".to_string());
        repo.save_state(&state).expect("Failed to save state");

        // Test read
        let loaded = repo.get_state(&workspace_id).expect("Failed to get state");
        assert!(loaded.is_some());
        let loaded = loaded.unwrap();
        assert_eq!(loaded.workspace_id, workspace_id);
        assert_eq!(loaded.server_url, "https://sync.example.com");
        assert_eq!(loaded.last_sync_version, 0);

        // Test update
        repo.update_last_sync(&workspace_id, 42).expect("Failed to update");
        let loaded = repo.get_state(&workspace_id).unwrap().unwrap();
        assert_eq!(loaded.last_sync_version, 42);

        // Test status update
        repo.update_status(&workspace_id, SyncStatus::Syncing).expect("Failed to update status");
        let loaded = repo.get_state(&workspace_id).unwrap().unwrap();
        assert_eq!(loaded.sync_status, SyncStatus::Syncing);

        // Test error
        repo.set_error(&workspace_id, "Test error").expect("Failed to set error");
        let loaded = repo.get_state(&workspace_id).unwrap().unwrap();
        assert_eq!(loaded.sync_status, SyncStatus::Error);
        assert_eq!(loaded.last_error, Some("Test error".to_string()));

        // Test clear error
        repo.clear_error(&workspace_id).expect("Failed to clear error");
        let loaded = repo.get_state(&workspace_id).unwrap().unwrap();
        assert_eq!(loaded.sync_status, SyncStatus::Idle);
        assert!(loaded.last_error.is_none());

        // Test delete
        repo.delete_state(&workspace_id).expect("Failed to delete");
        let loaded = repo.get_state(&workspace_id).expect("Failed to get state");
        assert!(loaded.is_none());
    }

    #[test]
    fn test_sync_queue_crud() {
        let conn = open_memory_db().expect("Failed to create test database");
        let repo = SqliteSyncQueueRepo::new(&conn);

        // Create test workspace
        let workspace_id = Uuid::new_v4();
        let account_id = Uuid::new_v4();

        conn.execute(
            "INSERT INTO accounts (id, email, name, status) VALUES (?1, ?2, ?3, 1)",
            params![account_id.to_string(), "test@example.com", "Test User"],
        )
        .unwrap();

        conn.execute(
            "INSERT INTO workspaces (id, name, owner_account_id) VALUES (?1, ?2, ?3)",
            params![workspace_id.to_string(), "Test Workspace", account_id.to_string()],
        )
        .unwrap();

        // Test enqueue
        let entity_id = Uuid::new_v4();
        let entry = SyncQueueEntry::new(
            workspace_id,
            "well",
            entity_id,
            SyncAction::Create,
            1,
            Some(r#"{"name": "Test Well"}"#.to_string()),
            Some(vec!["abc123".to_string()]),
        );

        let id = repo.enqueue(&entry).expect("Failed to enqueue");
        assert!(id > 0);

        // Test count
        let count = repo.count_pending(&workspace_id).expect("Failed to count");
        assert_eq!(count, 1);

        // Test get pending
        let pending = repo.get_pending(&workspace_id, 10).expect("Failed to get pending");
        assert_eq!(pending.len(), 1);
        assert_eq!(pending[0].entity_type, "well");
        assert_eq!(pending[0].action, SyncAction::Create);

        // Test mark synced
        repo.mark_synced(&[pending[0].id]).expect("Failed to mark synced");
        let count = repo.count_pending(&workspace_id).expect("Failed to count");
        assert_eq!(count, 0);

        // Test clear synced
        repo.clear_synced(&workspace_id).expect("Failed to clear synced");
    }

    #[test]
    fn test_conflict_crud() {
        let conn = open_memory_db().expect("Failed to create test database");
        let repo = SqliteConflictRepo::new(&conn);

        // Create test workspace
        let workspace_id = Uuid::new_v4();
        let account_id = Uuid::new_v4();

        conn.execute(
            "INSERT INTO accounts (id, email, name, status) VALUES (?1, ?2, ?3, 1)",
            params![account_id.to_string(), "test@example.com", "Test User"],
        )
        .unwrap();

        conn.execute(
            "INSERT INTO workspaces (id, name, owner_account_id) VALUES (?1, ?2, ?3)",
            params![workspace_id.to_string(), "Test Workspace", account_id.to_string()],
        )
        .unwrap();

        // Test save conflict
        let entity_id = Uuid::new_v4();
        let conflict = SyncConflict::new(
            workspace_id,
            "well",
            entity_id,
            2,
            r#"{"name": "Local Well"}"#.to_string(),
            1,
            r#"{"name": "Remote Well"}"#.to_string(),
        );

        let id = repo.save_conflict(&conflict).expect("Failed to save conflict");
        assert!(id > 0);

        // Test count pending
        let count = repo.count_pending(&workspace_id).expect("Failed to count");
        assert_eq!(count, 1);

        // Test get pending
        let pending = repo.get_pending(&workspace_id).expect("Failed to get pending");
        assert_eq!(pending.len(), 1);
        assert_eq!(pending[0].entity_type, "well");
        assert!(pending[0].is_pending());

        // Test resolve
        repo.resolve(id, ConflictResolution::Local, &account_id)
            .expect("Failed to resolve");

        let loaded = repo.get_by_id(id).unwrap().unwrap();
        assert_eq!(loaded.resolution, ConflictResolution::Local);
        assert!(loaded.resolved_at.is_some());

        // Test count after resolution
        let count = repo.count_pending(&workspace_id).expect("Failed to count");
        assert_eq!(count, 0);

        // Test clear resolved
        repo.clear_resolved(&workspace_id).expect("Failed to clear resolved");
        let loaded = repo.get_by_id(id).unwrap();
        assert!(loaded.is_none());
    }
}
