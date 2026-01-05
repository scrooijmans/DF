use rusqlite::Connection;
use std::path::PathBuf;
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

pub struct AppState {
    pub db: Option<Connection>,
    pub data_dir: Option<PathBuf>,
    pub session: SessionState,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            db: None,
            data_dir: None,
            session: SessionState::default(),
        }
    }
}

impl AppState {
    pub fn initialize(&mut self, data_dir: PathBuf) -> anyhow::Result<()> {
        std::fs::create_dir_all(&data_dir)?;

        let db_path = data_dir.join("dataforge.db");
        self.db = Some(dataforge_core::db::open_db(&db_path)?);
        self.data_dir = Some(data_dir.clone());

        // Try to load saved session token
        self.load_session(&data_dir)?;

        Ok(())
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

    /// Save session to disk
    pub fn save_session(&self) -> anyhow::Result<()> {
        if let Some(ref data_dir) = self.data_dir {
            let session_file = data_dir.join("session.json");
            let saved = SavedSession {
                token: self.session.token.clone().unwrap_or_default(),
                workspace_id: self.session.current_workspace_id.map(|id| id.to_string()),
                sync_auth_token: self.session.sync_auth_token.clone(),
            };
            let content = serde_json::to_string_pretty(&saved)?;
            std::fs::write(&session_file, content)?;
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
