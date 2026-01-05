use crate::state::AppState;
use dataforge_core::auth;
use dataforge_core::models::{Account, AccountStatus, Workspace, WorkspaceMemberInfo, WorkspaceRole};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::State;
use tracing::info;

// ============================================================
// LAS FILE PARSING
// ============================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct LasParseResult {
    pub well_name: Option<String>,
    pub curve_count: usize,
    pub row_count: usize,
    pub curves: Vec<String>,
}

#[tauri::command]
pub fn parse_las_file(path: String) -> Result<LasParseResult, String> {
    let path = PathBuf::from(path);

    let las = dataforge_core::las::parse_las_file(&path)
        .map_err(|e| format!("Failed to parse LAS: {}", e))?;

    let curves: Vec<String> = las
        .curves
        .iter()
        .map(|c| c.mnemonic.clone())
        .collect();

    Ok(LasParseResult {
        well_name: las.well_name.clone(),
        curve_count: las.curves.len(),
        row_count: las.curves.first().map(|c| c.row_count).unwrap_or(0),
        curves,
    })
}

// ============================================================
// AUTHENTICATION
// ============================================================

/// Account data returned to frontend
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountResponse {
    pub id: String,
    pub email: String,
    pub name: String,
    pub avatar_url: Option<String>,
    pub status: String,
}

impl From<Account> for AccountResponse {
    fn from(account: Account) -> Self {
        Self {
            id: account.id.to_string(),
            email: account.email,
            name: account.name,
            avatar_url: account.avatar_url,
            status: match account.status {
                AccountStatus::Unverified => "unverified",
                AccountStatus::Verified => "verified",
                AccountStatus::Suspended => "suspended",
            }
            .to_string(),
        }
    }
}

/// Workspace data returned to frontend
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkspaceResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub avatar_url: Option<String>,
    pub owner_id: String,
    pub role: Option<String>,
}

impl From<Workspace> for WorkspaceResponse {
    fn from(workspace: Workspace) -> Self {
        Self {
            id: workspace.id.to_string(),
            name: workspace.name,
            description: workspace.description,
            avatar_url: workspace.avatar_url,
            owner_id: workspace.owner_account_id.to_string(),
            role: None, // Will be set separately
        }
    }
}

/// Authentication response
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub account: AccountResponse,
    pub workspaces: Vec<WorkspaceResponse>,
    pub current_workspace_id: Option<String>,
}

/// Session state response (for checking auth on app launch)
#[derive(Debug, Serialize, Deserialize)]
pub struct SessionStateResponse {
    pub authenticated: bool,
    pub account: Option<AccountResponse>,
    pub workspaces: Vec<WorkspaceResponse>,
    pub current_workspace_id: Option<String>,
}

/// Register a new account
#[tauri::command]
pub fn register(
    email: String,
    password: String,
    name: String,
    state: State<'_, Mutex<AppState>>,
) -> Result<AuthResponse, String> {
    let mut state = state.lock().map_err(|e| format!("Lock error: {}", e))?;

    let db = state.db.as_ref().ok_or("Database not initialized")?;

    let result = auth::register_account(db, &email, &password, &name, Some("Desktop App"))
        .map_err(|e| e.to_string())?;

    // Save session
    state.session.token = Some(result.session_token.clone());
    state.save_session().map_err(|e| e.to_string())?;

    info!("Registered new account: {}", email);

    Ok(AuthResponse {
        account: result.account.into(),
        workspaces: result.workspaces.into_iter().map(|w| w.into()).collect(),
        current_workspace_id: state.session.current_workspace_id.map(|id| id.to_string()),
    })
}

/// Login with email and password
#[tauri::command]
pub fn login(
    email: String,
    password: String,
    state: State<'_, Mutex<AppState>>,
) -> Result<AuthResponse, String> {
    let mut state = state.lock().map_err(|e| format!("Lock error: {}", e))?;

    let db = state.db.as_ref().ok_or("Database not initialized")?;

    let result = auth::login(db, &email, &password, Some("Desktop App"))
        .map_err(|e| e.to_string())?;

    // Save session
    state.session.token = Some(result.session_token.clone());
    state.save_session().map_err(|e| e.to_string())?;

    info!("Logged in: {}", email);

    Ok(AuthResponse {
        account: result.account.into(),
        workspaces: result.workspaces.into_iter().map(|w| w.into()).collect(),
        current_workspace_id: state.session.current_workspace_id.map(|id| id.to_string()),
    })
}

/// Logout current session
#[tauri::command]
pub fn logout(state: State<'_, Mutex<AppState>>) -> Result<(), String> {
    let mut state = state.lock().map_err(|e| format!("Lock error: {}", e))?;

    if let (Some(db), Some(token)) = (state.db.as_ref(), state.session.token.as_ref()) {
        let _ = auth::logout(db, token);
    }

    state.clear_session().map_err(|e| e.to_string())?;

    info!("Logged out");

    Ok(())
}

/// Check if backend is ready (ColaNode pattern: readiness check)
#[tauri::command]
pub fn is_ready(state: State<'_, Mutex<AppState>>) -> Result<bool, String> {
    let state = state.lock().map_err(|e| format!("Lock error: {}", e))?;
    Ok(state.db.is_some())
}

/// Get current session state (called on app launch)
/// Follows ColaNode pattern: backend validates session before frontend queries
#[tauri::command]
pub fn get_session(
    state: State<'_, Mutex<AppState>>,
) -> Result<SessionStateResponse, String> {
    let mut state = state.lock().map_err(|e| format!("Lock error: {}", e))?;

    // If database is not initialized, return unauthenticated state
    // This should rarely happen as we check readiness first, but handle gracefully
    let db = match state.db.as_ref() {
        Some(db) => db,
        None => {
            tracing::warn!("⚠️ Database not initialized yet, returning unauthenticated state");
            return Ok(SessionStateResponse {
                authenticated: false,
                account: None,
                workspaces: vec![],
                current_workspace_id: None,
            });
        }
    };

    // Check if we have a saved session token (ColaNode pattern: check persisted session)
    let Some(ref token) = state.session.token.clone() else {
        info!("ℹ️ No session token found");
        return Ok(SessionStateResponse {
            authenticated: false,
            account: None,
            workspaces: vec![],
            current_workspace_id: None,
        });
    };

    // Validate the session (ColaNode pattern: validate on every query)
    match auth::validate_session(db, &token) {
        Ok(result) => {
            info!("✅ Session validated successfully for account: {}", result.account.email);
            Ok(SessionStateResponse {
                authenticated: true,
                account: Some(result.account.into()),
                workspaces: result.workspaces.into_iter().map(|w| w.into()).collect(),
                current_workspace_id: state.session.current_workspace_id.map(|id| id.to_string()),
            })
        }
        Err(e) => {
            // Session invalid - clear the invalid token (ColaNode pattern: auto-cleanup)
            info!("⚠️ Session validation failed: {}, clearing session", e);
            let _ = state.clear_session();
            Ok(SessionStateResponse {
                authenticated: false,
                account: None,
                workspaces: vec![],
                current_workspace_id: None,
            })
        }
    }
}

// ============================================================
// WORKSPACES
// ============================================================

/// Create a new workspace
#[tauri::command]
pub fn create_workspace(
    name: String,
    description: Option<String>,
    state: State<'_, Mutex<AppState>>,
) -> Result<WorkspaceResponse, String> {
    let mut state = state.lock().map_err(|e| format!("Lock error: {}", e))?;

    let db = state.db.as_ref().ok_or("Database not initialized")?;
    let token = state.session.token.as_ref().ok_or("Not authenticated")?;

    // Validate session and get account
    let auth_result = auth::validate_session(db, token).map_err(|e| e.to_string())?;

    // Create workspace
    let workspace = auth::create_workspace(
        db,
        &name,
        description.as_deref(),
        auth_result.account.id,
    )
    .map_err(|e| e.to_string())?;

    // Set as current workspace
    state.session.current_workspace_id = Some(workspace.id);
    state.save_session().map_err(|e| e.to_string())?;

    info!("Created workspace: {}", name);

    let mut response: WorkspaceResponse = workspace.into();
    response.role = Some(WorkspaceRole::Owner.to_string());

    Ok(response)
}

/// Select a workspace as current
#[tauri::command]
pub fn select_workspace(
    workspace_id: String,
    state: State<'_, Mutex<AppState>>,
) -> Result<(), String> {
    let mut state = state.lock().map_err(|e| format!("Lock error: {}", e))?;

    let id = uuid::Uuid::parse_str(&workspace_id)
        .map_err(|_| "Invalid workspace ID")?;

    // Verify the workspace exists and user is a member
    let db = state.db.as_ref().ok_or("Database not initialized")?;
    let token = state.session.token.as_ref().ok_or("Not authenticated")?;

    let auth_result = auth::validate_session(db, token).map_err(|e| e.to_string())?;

    // Check if workspace is in the list
    if !auth_result.workspaces.iter().any(|w| w.id == id) {
        return Err("Not a member of this workspace".to_string());
    }

    state.session.current_workspace_id = Some(id);
    state.save_session().map_err(|e| e.to_string())?;

    info!("Selected workspace: {}", workspace_id);

    Ok(())
}

/// Get all workspaces for current user
#[tauri::command]
pub fn get_workspaces(
    state: State<'_, Mutex<AppState>>,
) -> Result<Vec<WorkspaceResponse>, String> {
    let state = state.lock().map_err(|e| format!("Lock error: {}", e))?;

    let db = state.db.as_ref().ok_or("Database not initialized")?;
    let token = state.session.token.as_ref().ok_or("Not authenticated")?;

    let auth_result = auth::validate_session(db, token).map_err(|e| e.to_string())?;

    Ok(auth_result.workspaces.into_iter().map(|w| w.into()).collect())
}

// ============================================================
// WORKSPACE MEMBERS
// ============================================================

/// Workspace member data returned to frontend
#[derive(Debug, Serialize, Deserialize)]
pub struct MemberResponse {
    pub member_id: String,
    pub account_id: String,
    pub email: String,
    pub name: String,
    pub avatar_url: Option<String>,
    pub role: String,
    pub joined_at: String,
}

impl From<WorkspaceMemberInfo> for MemberResponse {
    fn from(member: WorkspaceMemberInfo) -> Self {
        Self {
            member_id: member.member_id.to_string(),
            account_id: member.account_id.to_string(),
            email: member.email,
            name: member.name,
            avatar_url: member.avatar_url,
            role: member.role.to_string(),
            joined_at: member.joined_at.to_rfc3339(),
        }
    }
}

/// Get all members of a workspace
#[tauri::command]
pub fn get_workspace_members(
    workspace_id: String,
    state: State<'_, Mutex<AppState>>,
) -> Result<Vec<MemberResponse>, String> {
    let state = state.lock().map_err(|e| format!("Lock error: {}", e))?;

    let db = state.db.as_ref().ok_or("Database not initialized")?;
    let token = state.session.token.as_ref().ok_or("Not authenticated")?;

    // Validate session
    let auth_result = auth::validate_session(db, token).map_err(|e| e.to_string())?;

    let workspace_id = uuid::Uuid::parse_str(&workspace_id)
        .map_err(|_| "Invalid workspace ID")?;

    // Verify user is a member of this workspace
    if !auth_result.workspaces.iter().any(|w| w.id == workspace_id) {
        return Err("Not a member of this workspace".to_string());
    }

    let members = auth::get_workspace_members(db, workspace_id)
        .map_err(|e| e.to_string())?;

    Ok(members.into_iter().map(|m| m.into()).collect())
}

/// Add a member to a workspace by email
#[tauri::command]
pub fn add_workspace_member(
    workspace_id: String,
    email: String,
    role: String,
    state: State<'_, Mutex<AppState>>,
) -> Result<MemberResponse, String> {
    let state = state.lock().map_err(|e| format!("Lock error: {}", e))?;

    let db = state.db.as_ref().ok_or("Database not initialized")?;
    let token = state.session.token.as_ref().ok_or("Not authenticated")?;

    // Validate session
    let auth_result = auth::validate_session(db, token).map_err(|e| e.to_string())?;

    let workspace_id = uuid::Uuid::parse_str(&workspace_id)
        .map_err(|_| "Invalid workspace ID")?;

    // Parse role
    let role: WorkspaceRole = role.parse().map_err(|e: String| e)?;

    // Check if user has permission (owner or admin)
    let requester_role = auth::get_member_role(db, workspace_id, auth_result.account.id)
        .map_err(|e| e.to_string())?;

    if requester_role != WorkspaceRole::Owner && requester_role != WorkspaceRole::Admin {
        return Err("Only owners and admins can add members".to_string());
    }

    let member = auth::add_workspace_member(
        db,
        workspace_id,
        &email,
        role,
        auth_result.account.id,
    )
    .map_err(|e| e.to_string())?;

    info!("Added member {} to workspace {}", email, workspace_id);

    Ok(member.into())
}

/// Update a workspace member's role
#[tauri::command]
pub fn update_workspace_member_role(
    workspace_id: String,
    target_account_id: String,
    new_role: String,
    state: State<'_, Mutex<AppState>>,
) -> Result<(), String> {
    let state = state.lock().map_err(|e| format!("Lock error: {}", e))?;

    let db = state.db.as_ref().ok_or("Database not initialized")?;
    let token = state.session.token.as_ref().ok_or("Not authenticated")?;

    // Validate session
    let auth_result = auth::validate_session(db, token).map_err(|e| e.to_string())?;

    let workspace_id = uuid::Uuid::parse_str(&workspace_id)
        .map_err(|_| "Invalid workspace ID")?;

    let target_account_id = uuid::Uuid::parse_str(&target_account_id)
        .map_err(|_| "Invalid target account ID")?;

    let new_role: WorkspaceRole = new_role.parse().map_err(|e: String| e)?;

    auth::update_member_role(
        db,
        workspace_id,
        target_account_id,
        new_role,
        auth_result.account.id,
    )
    .map_err(|e| e.to_string())?;

    info!(
        "Updated member {} role to {} in workspace {}",
        target_account_id, new_role, workspace_id
    );

    Ok(())
}

/// Remove a member from a workspace
#[tauri::command]
pub fn remove_workspace_member(
    workspace_id: String,
    target_account_id: String,
    state: State<'_, Mutex<AppState>>,
) -> Result<(), String> {
    let state = state.lock().map_err(|e| format!("Lock error: {}", e))?;

    let db = state.db.as_ref().ok_or("Database not initialized")?;
    let token = state.session.token.as_ref().ok_or("Not authenticated")?;

    // Validate session
    let auth_result = auth::validate_session(db, token).map_err(|e| e.to_string())?;

    let workspace_id = uuid::Uuid::parse_str(&workspace_id)
        .map_err(|_| "Invalid workspace ID")?;

    let target_account_id = uuid::Uuid::parse_str(&target_account_id)
        .map_err(|_| "Invalid target account ID")?;

    auth::remove_workspace_member(
        db,
        workspace_id,
        target_account_id,
        auth_result.account.id,
    )
    .map_err(|e| e.to_string())?;

    info!(
        "Removed member {} from workspace {}",
        target_account_id, workspace_id
    );

    Ok(())
}

/// Get current user's role in a workspace
#[tauri::command]
pub fn get_my_workspace_role(
    workspace_id: String,
    state: State<'_, Mutex<AppState>>,
) -> Result<String, String> {
    let state = state.lock().map_err(|e| format!("Lock error: {}", e))?;

    let db = state.db.as_ref().ok_or("Database not initialized")?;
    let token = state.session.token.as_ref().ok_or("Not authenticated")?;

    // Validate session
    let auth_result = auth::validate_session(db, token).map_err(|e| e.to_string())?;

    let workspace_id = uuid::Uuid::parse_str(&workspace_id)
        .map_err(|_| "Invalid workspace ID")?;

    let role = auth::get_member_role(db, workspace_id, auth_result.account.id)
        .map_err(|e| e.to_string())?;

    Ok(role.to_string())
}

/// Delete a workspace (owner only)
#[tauri::command]
pub fn delete_workspace(
    workspace_id: String,
    state: State<'_, Mutex<AppState>>,
) -> Result<(), String> {
    let mut state = state.lock().map_err(|e| format!("Lock error: {}", e))?;

    let db = state.db.as_ref().ok_or("Database not initialized")?;
    let token = state.session.token.as_ref().ok_or("Not authenticated")?;

    // Validate session
    let auth_result = auth::validate_session(db, token).map_err(|e| e.to_string())?;

    let workspace_id = uuid::Uuid::parse_str(&workspace_id)
        .map_err(|_| "Invalid workspace ID")?;

    // Delete workspace
    auth::delete_workspace(db, workspace_id, auth_result.account.id)
        .map_err(|e| e.to_string())?;

    // If deleted workspace was current, clear selection
    if state.session.current_workspace_id == Some(workspace_id) {
        state.session.current_workspace_id = None;
        state.save_session().map_err(|e| e.to_string())?;
    }

    info!("Deleted workspace: {}", workspace_id);

    Ok(())
}

/// Update workspace name and/or description
#[tauri::command]
pub fn update_workspace(
    workspace_id: String,
    name: Option<String>,
    description: Option<String>,
    state: State<'_, Mutex<AppState>>,
) -> Result<WorkspaceResponse, String> {
    let state = state.lock().map_err(|e| format!("Lock error: {}", e))?;

    let db = state.db.as_ref().ok_or("Database not initialized")?;
    let token = state.session.token.as_ref().ok_or("Not authenticated")?;

    // Validate session
    let auth_result = auth::validate_session(db, token).map_err(|e| e.to_string())?;

    let workspace_id = uuid::Uuid::parse_str(&workspace_id)
        .map_err(|_| "Invalid workspace ID")?;

    let workspace = auth::update_workspace(
        db,
        workspace_id,
        name.as_deref(),
        description.as_deref(),
        auth_result.account.id,
    )
    .map_err(|e| e.to_string())?;

    // Get user's role in workspace
    let role = auth::get_member_role(db, workspace_id, auth_result.account.id)
        .map_err(|e| e.to_string())?;

    info!("Updated workspace: {}", workspace_id);

    Ok(WorkspaceResponse {
        id: workspace.id.to_string(),
        name: workspace.name,
        description: workspace.description,
        avatar_url: workspace.avatar_url,
        owner_id: workspace.owner_account_id.to_string(),
        role: Some(role.to_string()),
    })
}

/// Transfer workspace ownership to an admin
#[tauri::command]
pub fn transfer_workspace_ownership(
    workspace_id: String,
    new_owner_account_id: String,
    state: State<'_, Mutex<AppState>>,
) -> Result<(), String> {
    let state = state.lock().map_err(|e| format!("Lock error: {}", e))?;

    let db = state.db.as_ref().ok_or("Database not initialized")?;
    let token = state.session.token.as_ref().ok_or("Not authenticated")?;

    // Validate session
    let auth_result = auth::validate_session(db, token).map_err(|e| e.to_string())?;

    let workspace_id = uuid::Uuid::parse_str(&workspace_id)
        .map_err(|_| "Invalid workspace ID")?;

    let new_owner_account_id = uuid::Uuid::parse_str(&new_owner_account_id)
        .map_err(|_| "Invalid target account ID")?;

    auth::transfer_workspace_ownership(
        db,
        workspace_id,
        new_owner_account_id,
        auth_result.account.id,
    )
    .map_err(|e| e.to_string())?;

    info!(
        "Transferred ownership of workspace {} to {}",
        workspace_id, new_owner_account_id
    );

    Ok(())
}

/// Update account profile information
#[tauri::command]
pub fn update_account(
    name: Option<String>,
    state: State<'_, Mutex<AppState>>,
) -> Result<AccountResponse, String> {
    let state = state.lock().map_err(|e| format!("Lock error: {}", e))?;

    let db = state.db.as_ref().ok_or("Database not initialized")?;
    let token = state.session.token.as_ref().ok_or("Not authenticated")?;

    // Validate session
    let auth_result = auth::validate_session(db, token).map_err(|e| e.to_string())?;

    let account = auth::update_account(db, auth_result.account.id, name.as_deref())
        .map_err(|e| e.to_string())?;

    info!("Updated account profile: {}", account.id);

    Ok(account.into())
}

// ============================================================
// PROJECTS (Placeholder - will be expanded)
// ============================================================

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to DataForge.", name)
}
