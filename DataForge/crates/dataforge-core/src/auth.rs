//! Authentication module for DataForge
//!
//! Provides email-based authentication with:
//! - Account registration and login
//! - Password hashing with Argon2
//! - Session token management
//! - Workspace membership

use argon2::{
	password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
	Argon2,
};
use chrono::{DateTime, Duration, Utc};
use rusqlite::{params, Connection, OptionalExtension};
use sha2::{Digest, Sha256};
use tracing::info;
use uuid::Uuid;

use crate::models::{
	Account, AccountStatus, AuthResult, Session, Workspace, WorkspaceMemberInfo, WorkspaceRole,
};

/// Authentication errors
#[derive(Debug, thiserror::Error)]
pub enum AuthError {
	#[error("Account not found")]
	AccountNotFound,

	#[error("Invalid email format")]
	InvalidEmail,

	#[error("Invalid password")]
	InvalidPassword,

	#[error("Email already registered")]
	EmailAlreadyExists,

	#[error("Session expired")]
	SessionExpired,

	#[error("Session not found")]
	SessionNotFound,

	#[error("Account suspended")]
	AccountSuspended,

	#[error("Workspace not found")]
	WorkspaceNotFound,

	#[error("Already a member of this workspace")]
	AlreadyMember,

	#[error("Not authorized")]
	NotAuthorized,

	#[error("Cannot modify workspace owner")]
	CannotModifyOwner,

	#[error("Account not found for email")]
	AccountNotFoundForEmail,

	#[error("Cannot transfer ownership to yourself")]
	CannotTransferToSelf,

	#[error("Target must be an admin to receive ownership")]
	TargetNotAdmin,

	#[error("Database error: {0}")]
	Database(#[from] rusqlite::Error),

	#[error("Password hashing error: {0}")]
	PasswordHash(String),
}

pub type AuthResult_ = std::result::Result<AuthResult, AuthError>;

/// Hash a password using Argon2
pub fn hash_password(password: &str) -> Result<String, AuthError> {
	let salt = SaltString::generate(&mut OsRng);
	let argon2 = Argon2::default();

	argon2
		.hash_password(password.as_bytes(), &salt)
		.map(|hash| hash.to_string())
		.map_err(|e| AuthError::PasswordHash(e.to_string()))
}

/// Verify a password against a hash
pub fn verify_password(password: &str, hash: &str) -> Result<bool, AuthError> {
	let parsed_hash =
		PasswordHash::new(hash).map_err(|e| AuthError::PasswordHash(e.to_string()))?;

	Ok(Argon2::default()
		.verify_password(password.as_bytes(), &parsed_hash)
		.is_ok())
}

/// Generate a session token
pub fn generate_session_token() -> String {
	format!("{}_{}", Uuid::new_v4(), Uuid::new_v4())
}

/// Hash a session token for storage
pub fn hash_token(token: &str) -> String {
	let mut hasher = Sha256::new();
	hasher.update(token.as_bytes());
	hex::encode(hasher.finalize())
}

/// Session duration (30 days)
const SESSION_DURATION_DAYS: i64 = 30;

/// Register a new account with email and password
pub fn register_account(
	conn: &Connection,
	email: &str,
	password: &str,
	name: &str,
	device_name: Option<&str>,
) -> Result<AuthResult, AuthError> {
	let email = email.trim().to_lowercase();

	// Basic email validation
	if !email.contains('@') || !email.contains('.') {
		return Err(AuthError::InvalidEmail);
	}

	// Check if email already exists
	let existing: Option<String> = conn
		.query_row(
			"SELECT id FROM accounts WHERE email = ?1",
			params![&email],
			|row| row.get(0),
		)
		.optional()?;

	if existing.is_some() {
		return Err(AuthError::EmailAlreadyExists);
	}

	// Hash password
	let password_hash = hash_password(password)?;

	// Create account
	let account_id = Uuid::new_v4();
	let now = Utc::now();

	conn.execute(
		r#"
		INSERT INTO accounts (id, email, password_hash, name, status, created_at, updated_at)
		VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
		"#,
		params![
			account_id.to_string(),
			&email,
			&password_hash,
			name,
			AccountStatus::Verified as i32, // Auto-verify for MVP
			now.to_rfc3339(),
			now.to_rfc3339(),
		],
	)?;

	info!("Created new account: {} ({})", email, account_id);

	// Create session
	let (session, session_token) = create_session(conn, account_id, device_name)?;

	// Build account object
	let account = Account {
		id: account_id,
		email,
		password_hash: Some(password_hash),
		name: name.to_string(),
		avatar_url: None,
		status: AccountStatus::Verified,
		created_at: now,
		updated_at: now,
	};

	// Get workspaces (will be empty for new account)
	let workspaces = get_account_workspaces(conn, account_id)?;

	Ok(AuthResult {
		account,
		session_token,
		workspaces,
	})
}

/// Login with email and password
pub fn login(
	conn: &Connection,
	email: &str,
	password: &str,
	device_name: Option<&str>,
) -> Result<AuthResult, AuthError> {
	let email = email.trim().to_lowercase();

	// Find account by email
	let row = conn
		.query_row(
			r#"
			SELECT id, email, password_hash, name, avatar_url, status, created_at, updated_at
			FROM accounts WHERE email = ?1
			"#,
			params![&email],
			|row| {
				Ok((
					row.get::<_, String>(0)?,
					row.get::<_, String>(1)?,
					row.get::<_, Option<String>>(2)?,
					row.get::<_, String>(3)?,
					row.get::<_, Option<String>>(4)?,
					row.get::<_, i32>(5)?,
					row.get::<_, String>(6)?,
					row.get::<_, String>(7)?,
				))
			},
		)
		.optional()?
		.ok_or(AuthError::AccountNotFound)?;

	let (id_str, email, password_hash_opt, name, avatar_url, status, created_at, updated_at) = row;

	let account_id = Uuid::parse_str(&id_str).map_err(|_| AuthError::AccountNotFound)?;
	let status = AccountStatus::from(status);

	// Check if suspended
	if status == AccountStatus::Suspended {
		return Err(AuthError::AccountSuspended);
	}

	// Verify password
	let password_hash = password_hash_opt.ok_or(AuthError::InvalidPassword)?;
	if !verify_password(password, &password_hash)? {
		return Err(AuthError::InvalidPassword);
	}

	// Create new session
	let (session, session_token) = create_session(conn, account_id, device_name)?;

	// Build account object
	let account = Account {
		id: account_id,
		email,
		password_hash: Some(password_hash),
		name,
		avatar_url,
		status,
		created_at: DateTime::parse_from_rfc3339(&created_at)
			.map(|dt| dt.with_timezone(&Utc))
			.unwrap_or_else(|_| Utc::now()),
		updated_at: DateTime::parse_from_rfc3339(&updated_at)
			.map(|dt| dt.with_timezone(&Utc))
			.unwrap_or_else(|_| Utc::now()),
	};

	// Get workspaces
	let workspaces = get_account_workspaces(conn, account_id)?;

	info!("Login successful: {} ({})", account.email, account_id);

	Ok(AuthResult {
		account,
		session_token,
		workspaces,
	})
}

/// Create a new session for an account
fn create_session(
	conn: &Connection,
	account_id: Uuid,
	device_name: Option<&str>,
) -> Result<(Session, String), AuthError> {
	let session_id = Uuid::new_v4();
	let token = generate_session_token();
	let token_hash = hash_token(&token);
	let now = Utc::now();
	let expires_at = now + Duration::days(SESSION_DURATION_DAYS);

	conn.execute(
		r#"
		INSERT INTO sessions (id, account_id, token_hash, device_name, created_at, expires_at, last_active_at)
		VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
		"#,
		params![
			session_id.to_string(),
			account_id.to_string(),
			&token_hash,
			device_name,
			now.to_rfc3339(),
			expires_at.to_rfc3339(),
			now.to_rfc3339(),
		],
	)?;

	let session = Session {
		id: session_id,
		account_id,
		token_hash,
		device_name: device_name.map(String::from),
		created_at: now,
		expires_at,
		last_active_at: now,
	};

	Ok((session, token))
}

/// Validate a session token and return the account
pub fn validate_session(conn: &Connection, token: &str) -> Result<AuthResult, AuthError> {
	let token_hash = hash_token(token);
	let now = Utc::now();

	// Find session
	let session_row = conn
		.query_row(
			r#"
			SELECT s.id, s.account_id, s.device_name, s.created_at, s.expires_at, s.last_active_at
			FROM sessions s
			WHERE s.token_hash = ?1
			"#,
			params![&token_hash],
			|row| {
				Ok((
					row.get::<_, String>(0)?,
					row.get::<_, String>(1)?,
					row.get::<_, Option<String>>(2)?,
					row.get::<_, String>(3)?,
					row.get::<_, String>(4)?,
					row.get::<_, String>(5)?,
				))
			},
		)
		.optional()?
		.ok_or(AuthError::SessionNotFound)?;

	let (session_id_str, account_id_str, device_name, created_at, expires_at, _last_active_at) =
		session_row;

	let expires_at = DateTime::parse_from_rfc3339(&expires_at)
		.map(|dt| dt.with_timezone(&Utc))
		.unwrap_or_else(|_| Utc::now());

	// Check expiration
	if now > expires_at {
		return Err(AuthError::SessionExpired);
	}

	let account_id = Uuid::parse_str(&account_id_str).map_err(|_| AuthError::SessionNotFound)?;

	// Update last_active_at
	conn.execute(
		"UPDATE sessions SET last_active_at = ?1 WHERE token_hash = ?2",
		params![now.to_rfc3339(), &token_hash],
	)?;

	// Get account
	let account = get_account_by_id(conn, account_id)?;

	// Check if suspended
	if account.status == AccountStatus::Suspended {
		return Err(AuthError::AccountSuspended);
	}

	// Get workspaces
	let workspaces = get_account_workspaces(conn, account_id)?;

	Ok(AuthResult {
		account,
		session_token: token.to_string(),
		workspaces,
	})
}

/// Logout (invalidate session)
pub fn logout(conn: &Connection, token: &str) -> Result<(), AuthError> {
	let token_hash = hash_token(token);

	conn.execute("DELETE FROM sessions WHERE token_hash = ?1", params![&token_hash])?;

	Ok(())
}

/// Get account by ID
fn get_account_by_id(conn: &Connection, account_id: Uuid) -> Result<Account, AuthError> {
	conn.query_row(
		r#"
		SELECT id, email, password_hash, name, avatar_url, status, created_at, updated_at
		FROM accounts WHERE id = ?1
		"#,
		params![account_id.to_string()],
		|row| {
			Ok(Account {
				id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap_or(account_id),
				email: row.get(1)?,
				password_hash: row.get(2)?,
				name: row.get(3)?,
				avatar_url: row.get(4)?,
				status: AccountStatus::from(row.get::<_, i32>(5)?),
				created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(6)?)
					.map(|dt| dt.with_timezone(&Utc))
					.unwrap_or_else(|_| Utc::now()),
				updated_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(7)?)
					.map(|dt| dt.with_timezone(&Utc))
					.unwrap_or_else(|_| Utc::now()),
			})
		},
	)
	.optional()?
	.ok_or(AuthError::AccountNotFound)
}

/// Get all workspaces for an account
pub fn get_account_workspaces(
	conn: &Connection,
	account_id: Uuid,
) -> Result<Vec<Workspace>, AuthError> {
	let mut stmt = conn.prepare(
		r#"
		SELECT w.id, w.name, w.description, w.avatar_url, w.owner_account_id, w.created_at, w.updated_at
		FROM workspaces w
		INNER JOIN workspace_members wm ON w.id = wm.workspace_id
		WHERE wm.account_id = ?1
		ORDER BY w.name
		"#,
	)?;

	let workspaces = stmt
		.query_map(params![account_id.to_string()], |row| {
			Ok(Workspace {
				id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap_or_default(),
				name: row.get(1)?,
				description: row.get(2)?,
				avatar_url: row.get(3)?,
				owner_account_id: Uuid::parse_str(&row.get::<_, String>(4)?).unwrap_or_default(),
				created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(5)?)
					.map(|dt| dt.with_timezone(&Utc))
					.unwrap_or_else(|_| Utc::now()),
				updated_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(6)?)
					.map(|dt| dt.with_timezone(&Utc))
					.unwrap_or_else(|_| Utc::now()),
			})
		})?
		.filter_map(|r| r.ok())
		.collect();

	Ok(workspaces)
}

/// Create a new workspace
pub fn create_workspace(
	conn: &Connection,
	name: &str,
	description: Option<&str>,
	owner_account_id: Uuid,
) -> Result<Workspace, AuthError> {
	let workspace_id = Uuid::new_v4();
	let now = Utc::now();

	conn.execute(
		r#"
		INSERT INTO workspaces (id, name, description, owner_account_id, created_at, updated_at)
		VALUES (?1, ?2, ?3, ?4, ?5, ?6)
		"#,
		params![
			workspace_id.to_string(),
			name,
			description,
			owner_account_id.to_string(),
			now.to_rfc3339(),
			now.to_rfc3339(),
		],
	)?;

	// Add owner as member with 'owner' role
	let member_id = Uuid::new_v4();
	conn.execute(
		r#"
		INSERT INTO workspace_members (id, workspace_id, account_id, role, created_at, updated_at)
		VALUES (?1, ?2, ?3, ?4, ?5, ?6)
		"#,
		params![
			member_id.to_string(),
			workspace_id.to_string(),
			owner_account_id.to_string(),
			WorkspaceRole::Owner.to_string(),
			now.to_rfc3339(),
			now.to_rfc3339(),
		],
	)?;

	info!(
		"Created workspace '{}' ({}) for account {}",
		name, workspace_id, owner_account_id
	);

	Ok(Workspace {
		id: workspace_id,
		name: name.to_string(),
		description: description.map(String::from),
		avatar_url: None,
		owner_account_id,
		created_at: now,
		updated_at: now,
	})
}

/// Get workspace member ID for an account in a workspace
pub fn get_workspace_member_id(
	conn: &Connection,
	workspace_id: Uuid,
	account_id: Uuid,
) -> Result<Uuid, AuthError> {
	conn.query_row(
		"SELECT id FROM workspace_members WHERE workspace_id = ?1 AND account_id = ?2",
		params![workspace_id.to_string(), account_id.to_string()],
		|row| {
			let id_str: String = row.get(0)?;
			Ok(Uuid::parse_str(&id_str).unwrap_or_default())
		},
	)
	.optional()?
	.ok_or(AuthError::NotAuthorized)
}

/// Get the role of a member in a workspace
pub fn get_member_role(
	conn: &Connection,
	workspace_id: Uuid,
	account_id: Uuid,
) -> Result<WorkspaceRole, AuthError> {
	conn.query_row(
		"SELECT role FROM workspace_members WHERE workspace_id = ?1 AND account_id = ?2",
		params![workspace_id.to_string(), account_id.to_string()],
		|row| {
			let role_str: String = row.get(0)?;
			Ok(role_str
				.parse::<WorkspaceRole>()
				.unwrap_or(WorkspaceRole::Viewer))
		},
	)
	.optional()?
	.ok_or(AuthError::NotAuthorized)
}

/// Get all members of a workspace with account details
pub fn get_workspace_members(
	conn: &Connection,
	workspace_id: Uuid,
) -> Result<Vec<WorkspaceMemberInfo>, AuthError> {
	let mut stmt = conn.prepare(
		r#"
		SELECT
			wm.id as member_id,
			wm.account_id,
			a.email,
			a.name,
			a.avatar_url,
			wm.role,
			wm.created_at as joined_at
		FROM workspace_members wm
		INNER JOIN accounts a ON wm.account_id = a.id
		WHERE wm.workspace_id = ?1
		ORDER BY
			CASE wm.role
				WHEN 'owner' THEN 0
				WHEN 'admin' THEN 1
				WHEN 'member' THEN 2
				WHEN 'viewer' THEN 3
			END,
			wm.created_at ASC
		"#,
	)?;

	let members = stmt
		.query_map(params![workspace_id.to_string()], |row| {
			Ok(WorkspaceMemberInfo {
				member_id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap_or_default(),
				account_id: Uuid::parse_str(&row.get::<_, String>(1)?).unwrap_or_default(),
				email: row.get(2)?,
				name: row.get(3)?,
				avatar_url: row.get(4)?,
				role: row
					.get::<_, String>(5)?
					.parse::<WorkspaceRole>()
					.unwrap_or(WorkspaceRole::Viewer),
				joined_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(6)?)
					.map(|dt| dt.with_timezone(&Utc))
					.unwrap_or_else(|_| Utc::now()),
			})
		})?
		.filter_map(|r| r.ok())
		.collect();

	Ok(members)
}

/// Add a user to a workspace by email (user must already have an account)
pub fn add_workspace_member(
	conn: &Connection,
	workspace_id: Uuid,
	email: &str,
	role: WorkspaceRole,
	_added_by: Uuid, // For audit purposes
) -> Result<WorkspaceMemberInfo, AuthError> {
	let email = email.trim().to_lowercase();

	// Find account by email
	let account_row = conn
		.query_row(
			"SELECT id, email, name, avatar_url FROM accounts WHERE email = ?1",
			params![&email],
			|row| {
				Ok((
					row.get::<_, String>(0)?,
					row.get::<_, String>(1)?,
					row.get::<_, String>(2)?,
					row.get::<_, Option<String>>(3)?,
				))
			},
		)
		.optional()?
		.ok_or(AuthError::AccountNotFoundForEmail)?;

	let (account_id_str, email, name, avatar_url) = account_row;
	let account_id = Uuid::parse_str(&account_id_str).map_err(|_| AuthError::AccountNotFound)?;

	// Check if already a member
	let existing: Option<String> = conn
		.query_row(
			"SELECT id FROM workspace_members WHERE workspace_id = ?1 AND account_id = ?2",
			params![workspace_id.to_string(), account_id.to_string()],
			|row| row.get(0),
		)
		.optional()?;

	if existing.is_some() {
		return Err(AuthError::AlreadyMember);
	}

	// Cannot add as owner (only one owner allowed)
	let mut actual_role = if role == WorkspaceRole::Owner {
		WorkspaceRole::Admin
	} else {
		role
	};

	// Auto-promote first non-owner member to admin
	// This ensures the owner can always transfer ownership
	let has_other_members: bool = conn
		.query_row(
			"SELECT 1 FROM workspace_members WHERE workspace_id = ?1 AND role != 'owner' LIMIT 1",
			params![workspace_id.to_string()],
			|_| Ok(true),
		)
		.optional()?
		.unwrap_or(false);

	if !has_other_members && actual_role == WorkspaceRole::Member {
		actual_role = WorkspaceRole::Admin;
		info!("Auto-promoting first member to admin for workspace {}", workspace_id);
	}

	// Add membership
	let member_id = Uuid::new_v4();
	let now = Utc::now();

	conn.execute(
		r#"
		INSERT INTO workspace_members (id, workspace_id, account_id, role, created_at, updated_at)
		VALUES (?1, ?2, ?3, ?4, ?5, ?6)
		"#,
		params![
			member_id.to_string(),
			workspace_id.to_string(),
			account_id.to_string(),
			actual_role.to_string(),
			now.to_rfc3339(),
			now.to_rfc3339(),
		],
	)?;

	info!(
		"Added {} ({}) to workspace {} with role {}",
		email, account_id, workspace_id, actual_role
	);

	Ok(WorkspaceMemberInfo {
		member_id,
		account_id,
		email,
		name,
		avatar_url,
		role: actual_role,
		joined_at: now,
	})
}

/// Update a workspace member's role
pub fn update_member_role(
	conn: &Connection,
	workspace_id: Uuid,
	target_account_id: Uuid,
	new_role: WorkspaceRole,
	requester_account_id: Uuid,
) -> Result<(), AuthError> {
	// Get requester's role
	let requester_role = get_member_role(conn, workspace_id, requester_account_id)?;

	// Only owners and admins can change roles
	if requester_role != WorkspaceRole::Owner && requester_role != WorkspaceRole::Admin {
		return Err(AuthError::NotAuthorized);
	}

	// Get target's current role
	let target_role = get_member_role(conn, workspace_id, target_account_id)?;

	// Cannot change owner's role
	if target_role == WorkspaceRole::Owner {
		return Err(AuthError::CannotModifyOwner);
	}

	// Admins cannot promote to owner or admin
	if requester_role == WorkspaceRole::Admin
		&& (new_role == WorkspaceRole::Owner || new_role == WorkspaceRole::Admin)
	{
		return Err(AuthError::NotAuthorized);
	}

	// Cannot change role to owner (only one owner allowed)
	if new_role == WorkspaceRole::Owner {
		return Err(AuthError::NotAuthorized);
	}

	let now = Utc::now();

	conn.execute(
		"UPDATE workspace_members SET role = ?1, updated_at = ?2 WHERE workspace_id = ?3 AND account_id = ?4",
		params![
			new_role.to_string(),
			now.to_rfc3339(),
			workspace_id.to_string(),
			target_account_id.to_string(),
		],
	)?;

	info!(
		"Updated role for {} in workspace {} to {}",
		target_account_id, workspace_id, new_role
	);

	Ok(())
}

/// Remove a member from a workspace
pub fn remove_workspace_member(
	conn: &Connection,
	workspace_id: Uuid,
	target_account_id: Uuid,
	requester_account_id: Uuid,
) -> Result<(), AuthError> {
	// Get requester's role
	let requester_role = get_member_role(conn, workspace_id, requester_account_id)?;

	// Only owners and admins can remove members
	if requester_role != WorkspaceRole::Owner && requester_role != WorkspaceRole::Admin {
		return Err(AuthError::NotAuthorized);
	}

	// Get target's role
	let target_role = get_member_role(conn, workspace_id, target_account_id)?;

	// Cannot remove owner
	if target_role == WorkspaceRole::Owner {
		return Err(AuthError::CannotModifyOwner);
	}

	// Admins cannot remove other admins
	if requester_role == WorkspaceRole::Admin && target_role == WorkspaceRole::Admin {
		return Err(AuthError::NotAuthorized);
	}

	conn.execute(
		"DELETE FROM workspace_members WHERE workspace_id = ?1 AND account_id = ?2",
		params![workspace_id.to_string(), target_account_id.to_string()],
	)?;

	info!(
		"Removed {} from workspace {}",
		target_account_id, workspace_id
	);

	Ok(())
}

/// Leave a workspace (member removes themselves)
pub fn leave_workspace(
	conn: &Connection,
	workspace_id: Uuid,
	account_id: Uuid,
) -> Result<(), AuthError> {
	// Get member's role
	let role = get_member_role(conn, workspace_id, account_id)?;

	// Owner cannot leave (must transfer ownership first)
	if role == WorkspaceRole::Owner {
		return Err(AuthError::CannotModifyOwner);
	}

	conn.execute(
		"DELETE FROM workspace_members WHERE workspace_id = ?1 AND account_id = ?2",
		params![workspace_id.to_string(), account_id.to_string()],
	)?;

	info!("{} left workspace {}", account_id, workspace_id);

	Ok(())
}

/// Delete a workspace (owner only)
///
/// CASCADE DELETE rules in the database will automatically delete:
/// - workspace_members
/// - wells (and their curves, log_runs, etc.)
/// - surfaces
/// - sync_state, sync_queue, sync_conflicts
pub fn delete_workspace(
	conn: &Connection,
	workspace_id: Uuid,
	requester_account_id: Uuid,
) -> Result<(), AuthError> {
	// Verify requester is the owner
	let role = get_member_role(conn, workspace_id, requester_account_id)?;

	if role != WorkspaceRole::Owner {
		return Err(AuthError::NotAuthorized);
	}

	// Verify workspace exists
	let exists: bool = conn
		.query_row(
			"SELECT 1 FROM workspaces WHERE id = ?1",
			params![workspace_id.to_string()],
			|_| Ok(true),
		)
		.optional()?
		.unwrap_or(false);

	if !exists {
		return Err(AuthError::WorkspaceNotFound);
	}

	// Delete workspace (CASCADE handles members, wells, etc.)
	conn.execute(
		"DELETE FROM workspaces WHERE id = ?1",
		params![workspace_id.to_string()],
	)?;

	info!(
		"Deleted workspace {} by owner {}",
		workspace_id, requester_account_id
	);

	Ok(())
}

/// Update workspace name and/or description
///
/// Owner can update all fields
/// Admin can update name and description
pub fn update_workspace(
	conn: &Connection,
	workspace_id: Uuid,
	name: Option<&str>,
	description: Option<&str>,
	requester_account_id: Uuid,
) -> Result<Workspace, AuthError> {
	// Verify requester has permission
	let role = get_member_role(conn, workspace_id, requester_account_id)?;

	if role != WorkspaceRole::Owner && role != WorkspaceRole::Admin {
		return Err(AuthError::NotAuthorized);
	}

	// Get current workspace
	let workspace: Workspace = conn
		.query_row(
			r#"
            SELECT id, name, description, avatar_url, owner_account_id, created_at, updated_at
            FROM workspaces WHERE id = ?1
            "#,
			params![workspace_id.to_string()],
			|row| {
				Ok(Workspace {
					id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap_or_default(),
					name: row.get(1)?,
					description: row.get(2)?,
					avatar_url: row.get(3)?,
					owner_account_id: Uuid::parse_str(&row.get::<_, String>(4)?).unwrap_or_default(),
					created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(5)?)
						.map(|dt| dt.with_timezone(&Utc))
						.unwrap_or_else(|_| Utc::now()),
					updated_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(6)?)
						.map(|dt| dt.with_timezone(&Utc))
						.unwrap_or_else(|_| Utc::now()),
				})
			},
		)
		.optional()?
		.ok_or(AuthError::WorkspaceNotFound)?;

	// Build updated values
	let new_name = name.unwrap_or(&workspace.name);
	let new_description = description.or(workspace.description.as_deref());
	let now = Utc::now();

	conn.execute(
		"UPDATE workspaces SET name = ?1, description = ?2, updated_at = ?3 WHERE id = ?4",
		params![
			new_name,
			new_description,
			now.to_rfc3339(),
			workspace_id.to_string(),
		],
	)?;

	info!(
		"Updated workspace {} by {}",
		workspace_id, requester_account_id
	);

	Ok(Workspace {
		id: workspace.id,
		name: new_name.to_string(),
		description: new_description.map(String::from),
		avatar_url: workspace.avatar_url,
		owner_account_id: workspace.owner_account_id,
		created_at: workspace.created_at,
		updated_at: now,
	})
}

/// Transfer workspace ownership to another admin
///
/// Requirements:
/// - Requester must be current owner
/// - Target must be an admin in the workspace
/// - Target cannot be the requester
///
/// After transfer:
/// - Target becomes owner
/// - Previous owner becomes admin
pub fn transfer_workspace_ownership(
	conn: &Connection,
	workspace_id: Uuid,
	new_owner_account_id: Uuid,
	requester_account_id: Uuid,
) -> Result<(), AuthError> {
	// Verify requester is the owner
	let requester_role = get_member_role(conn, workspace_id, requester_account_id)?;

	if requester_role != WorkspaceRole::Owner {
		return Err(AuthError::NotAuthorized);
	}

	// Cannot transfer to yourself
	if new_owner_account_id == requester_account_id {
		return Err(AuthError::CannotTransferToSelf);
	}

	// Verify target is an admin
	let target_role = get_member_role(conn, workspace_id, new_owner_account_id)?;

	if target_role != WorkspaceRole::Admin {
		return Err(AuthError::TargetNotAdmin);
	}

	let now = Utc::now();

	// 1. Demote current owner to admin
	conn.execute(
		"UPDATE workspace_members SET role = ?1, updated_at = ?2 WHERE workspace_id = ?3 AND account_id = ?4",
		params![
			"admin",
			now.to_rfc3339(),
			workspace_id.to_string(),
			requester_account_id.to_string(),
		],
	)?;

	// 2. Promote new owner
	conn.execute(
		"UPDATE workspace_members SET role = ?1, updated_at = ?2 WHERE workspace_id = ?3 AND account_id = ?4",
		params![
			"owner",
			now.to_rfc3339(),
			workspace_id.to_string(),
			new_owner_account_id.to_string(),
		],
	)?;

	// 3. Update workspace owner_account_id
	conn.execute(
		"UPDATE workspaces SET owner_account_id = ?1, updated_at = ?2 WHERE id = ?3",
		params![
			new_owner_account_id.to_string(),
			now.to_rfc3339(),
			workspace_id.to_string(),
		],
	)?;

	info!(
		"Transferred ownership of workspace {} from {} to {}",
		workspace_id, requester_account_id, new_owner_account_id
	);

	Ok(())
}

/// Update account profile information
///
/// Users can update their own name
pub fn update_account(
	conn: &Connection,
	account_id: Uuid,
	name: Option<&str>,
) -> Result<Account, AuthError> {
	// Get current account
	let account: Account = conn
		.query_row(
			r#"
            SELECT id, email, name, avatar_url, status, created_at, updated_at
            FROM accounts WHERE id = ?1
            "#,
			params![account_id.to_string()],
			|row| {
				let status_int: i32 = row.get(4)?;
				Ok(Account {
					id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap_or_default(),
					email: row.get(1)?,
					password_hash: None, // Not needed for profile display
					name: row.get(2)?,
					avatar_url: row.get(3)?,
					status: AccountStatus::try_from(status_int).unwrap_or(AccountStatus::Unverified),
					created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(5)?)
						.map(|dt| dt.with_timezone(&Utc))
						.unwrap_or_else(|_| Utc::now()),
					updated_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(6)?)
						.map(|dt| dt.with_timezone(&Utc))
						.unwrap_or_else(|_| Utc::now()),
				})
			},
		)
		.optional()?
		.ok_or(AuthError::AccountNotFound)?;

	// Build updated values
	let new_name = name.unwrap_or(&account.name);
	let now = Utc::now();

	conn.execute(
		"UPDATE accounts SET name = ?1, updated_at = ?2 WHERE id = ?3",
		params![new_name, now.to_rfc3339(), account_id.to_string(),],
	)?;

	info!("Updated account {} profile", account_id);

	Ok(Account {
		id: account.id,
		email: account.email,
		password_hash: None, // Not returned for security
		name: new_name.to_string(),
		avatar_url: account.avatar_url,
		status: account.status,
		created_at: account.created_at,
		updated_at: now,
	})
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::db::open_memory_db;

	#[test]
	fn test_password_hashing() {
		let password = "test_password_123";
		let hash = hash_password(password).unwrap();

		assert!(verify_password(password, &hash).unwrap());
		assert!(!verify_password("wrong_password", &hash).unwrap());
	}

	#[test]
	fn test_register_and_login() {
		let conn = open_memory_db().unwrap();

		// Register
		let result = register_account(&conn, "test@example.com", "password123", "Test User", None)
			.expect("Registration should succeed");

		assert_eq!(result.account.email, "test@example.com");
		assert_eq!(result.account.name, "Test User");
		assert!(!result.session_token.is_empty());
		assert!(result.workspaces.is_empty());

		// Login
		let login_result = login(&conn, "test@example.com", "password123", None)
			.expect("Login should succeed");

		assert_eq!(login_result.account.email, "test@example.com");
	}

	#[test]
	fn test_duplicate_email() {
		let conn = open_memory_db().unwrap();

		register_account(&conn, "test@example.com", "password123", "Test User", None).unwrap();

		let result = register_account(&conn, "test@example.com", "password456", "Another User", None);

		assert!(matches!(result, Err(AuthError::EmailAlreadyExists)));
	}

	#[test]
	fn test_invalid_password() {
		let conn = open_memory_db().unwrap();

		register_account(&conn, "test@example.com", "password123", "Test User", None).unwrap();

		let result = login(&conn, "test@example.com", "wrong_password", None);

		assert!(matches!(result, Err(AuthError::InvalidPassword)));
	}

	#[test]
	fn test_session_validation() {
		let conn = open_memory_db().unwrap();

		let register_result =
			register_account(&conn, "test@example.com", "password123", "Test User", None).unwrap();

		let validate_result = validate_session(&conn, &register_result.session_token)
			.expect("Session should be valid");

		assert_eq!(validate_result.account.email, "test@example.com");
	}

	#[test]
	fn test_logout() {
		let conn = open_memory_db().unwrap();

		let result =
			register_account(&conn, "test@example.com", "password123", "Test User", None).unwrap();

		logout(&conn, &result.session_token).expect("Logout should succeed");

		let validate_result = validate_session(&conn, &result.session_token);
		assert!(matches!(validate_result, Err(AuthError::SessionNotFound)));
	}

	#[test]
	fn test_create_workspace() {
		let conn = open_memory_db().unwrap();

		let result =
			register_account(&conn, "test@example.com", "password123", "Test User", None).unwrap();

		let workspace =
			create_workspace(&conn, "My Workspace", Some("Test workspace"), result.account.id)
				.expect("Workspace creation should succeed");

		assert_eq!(workspace.name, "My Workspace");
		assert_eq!(workspace.owner_account_id, result.account.id);

		// Verify membership
		let workspaces = get_account_workspaces(&conn, result.account.id).unwrap();
		assert_eq!(workspaces.len(), 1);
		assert_eq!(workspaces[0].name, "My Workspace");
	}
}
