use rusqlite::Connection;
use std::path::PathBuf;
use tracing::{info, warn};
use uuid::Uuid;

/// Current session state (persisted to disk)
#[derive(Debug, Clone, Default)]
pub struct SessionState {
    /// The session token (stored in keychain/secure storage in production)
    pub token: Option<String>,
    /// Currently selected workspace ID
    pub current_workspace_id: Option<Uuid>,
    /// Sync server auth token (for authenticating with remote sync server)
    pub sync_auth_token: Option<String>,
}

/// Crash recovery information
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct CrashRecoveryInfo {
    /// Whether a crash was detected on startup
    pub crash_detected: bool,
    /// Timestamp when the crashed session started (if available)
    pub crashed_session_start: Option<String>,
    /// Whether recovery was performed
    pub recovery_performed: bool,
}

pub struct AppState {
    pub db: Option<Connection>,
    pub data_dir: Option<PathBuf>,
    pub session: SessionState,
    /// Information about crash recovery (populated during initialization)
    pub crash_recovery: CrashRecoveryInfo,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            db: None,
            data_dir: None,
            session: SessionState::default(),
            crash_recovery: CrashRecoveryInfo::default(),
        }
    }
}

/// Running marker file content
#[derive(serde::Serialize, serde::Deserialize)]
struct RunningMarker {
    started_at: String,
    pid: u32,
}

impl AppState {
    pub fn initialize(&mut self, data_dir: PathBuf) -> anyhow::Result<()> {
        std::fs::create_dir_all(&data_dir)?;

        // Check for crash recovery (stale running marker)
        self.crash_recovery = self.check_crash_recovery(&data_dir);
        if self.crash_recovery.crash_detected {
            warn!(
                crashed_at = ?self.crash_recovery.crashed_session_start,
                "Detected previous crash - performing recovery"
            );
            self.perform_crash_recovery(&data_dir)?;
        }

        // Create running marker for this session
        self.create_running_marker(&data_dir)?;

        let db_path = data_dir.join("dataforge.db");
        self.db = Some(dataforge_core::db::open_db(&db_path)?);
        self.data_dir = Some(data_dir.clone());

        // Try to load saved session token
        self.load_session(&data_dir)?;

        Ok(())
    }

    /// Check if a previous session crashed (left behind a running marker)
    fn check_crash_recovery(&self, data_dir: &PathBuf) -> CrashRecoveryInfo {
        let marker_file = data_dir.join(".running");

        if marker_file.exists() {
            // Read the marker to get crash info
            let crashed_session_start = std::fs::read_to_string(&marker_file)
                .ok()
                .and_then(|content| serde_json::from_str::<RunningMarker>(&content).ok())
                .map(|m| m.started_at);

            CrashRecoveryInfo {
                crash_detected: true,
                crashed_session_start,
                recovery_performed: false,
            }
        } else {
            CrashRecoveryInfo::default()
        }
    }

    /// Perform crash recovery actions
    fn perform_crash_recovery(&mut self, data_dir: &PathBuf) -> anyhow::Result<()> {
        // Clean up stale temp files
        let temp_session = data_dir.join("session.json.tmp");
        if temp_session.exists() {
            info!("Cleaning up stale session temp file");
            let _ = std::fs::remove_file(&temp_session);
        }

        // Remove the stale running marker
        let marker_file = data_dir.join(".running");
        if marker_file.exists() {
            let _ = std::fs::remove_file(&marker_file);
        }

        // Check WAL checkpoint (helps recover from interrupted writes)
        let db_path = data_dir.join("dataforge.db");
        if db_path.exists() {
            // The WAL checkpoint will be handled by SQLite when we open the DB
            info!("Database exists, SQLite will handle WAL recovery");
        }

        self.crash_recovery.recovery_performed = true;
        info!("Crash recovery completed");

        Ok(())
    }

    /// Create a running marker to detect crashes
    fn create_running_marker(&self, data_dir: &PathBuf) -> anyhow::Result<()> {
        let marker_file = data_dir.join(".running");
        let marker = RunningMarker {
            started_at: chrono::Utc::now().to_rfc3339(),
            pid: std::process::id(),
        };
        let content = serde_json::to_string(&marker)?;
        std::fs::write(&marker_file, content)?;
        Ok(())
    }

    /// Remove the running marker (call on clean shutdown)
    pub fn remove_running_marker(&self) {
        if let Some(ref data_dir) = self.data_dir {
            let marker_file = data_dir.join(".running");
            let _ = std::fs::remove_file(&marker_file);
        }
    }

    /// Load session from disk
    fn load_session(&mut self, data_dir: &PathBuf) -> anyhow::Result<()> {
        let session_file = data_dir.join("session.json");
        if session_file.exists() {
            let content = std::fs::read_to_string(&session_file)?;
            if let Ok(saved) = serde_json::from_str::<SavedSession>(&content) {
                self.session.token = Some(saved.token);
                self.session.current_workspace_id = saved.workspace_id.and_then(|s| Uuid::parse_str(&s).ok());
                self.session.sync_auth_token = saved.sync_auth_token;
            }
        }
        Ok(())
    }

    /// Save session to disk atomically
    ///
    /// Uses write-to-temp-then-rename pattern for crash safety.
    /// On Unix, sets restrictive file permissions (owner read/write only).
    pub fn save_session(&self) -> anyhow::Result<()> {
        if let Some(ref data_dir) = self.data_dir {
            let session_file = data_dir.join("session.json");
            let temp_file = data_dir.join("session.json.tmp");

            let saved = SavedSession {
                token: self.session.token.clone().unwrap_or_default(),
                workspace_id: self.session.current_workspace_id.map(|id| id.to_string()),
                sync_auth_token: self.session.sync_auth_token.clone(),
            };
            let content = serde_json::to_string_pretty(&saved)?;

            // Write to temp file first
            std::fs::write(&temp_file, content)?;

            // Set secure permissions on Unix (owner read/write only)
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let mut perms = std::fs::metadata(&temp_file)?.permissions();
                perms.set_mode(0o600);
                std::fs::set_permissions(&temp_file, perms)?;
            }

            // Atomic rename (safe on all platforms)
            std::fs::rename(&temp_file, &session_file)?;
        }
        Ok(())
    }

    /// Clear session (logout)
    pub fn clear_session(&mut self) -> anyhow::Result<()> {
        self.session = SessionState::default();
        if let Some(ref data_dir) = self.data_dir {
            let session_file = data_dir.join("session.json");
            if session_file.exists() {
                std::fs::remove_file(&session_file)?;
            }
        }
        Ok(())
    }

    /// Get database connection reference
    pub fn db(&self) -> Option<&Connection> {
        self.db.as_ref()
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
struct SavedSession {
    token: String,
    workspace_id: Option<String>,
    #[serde(default)]
    sync_auth_token: Option<String>,
}
