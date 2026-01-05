//! Domain models for DataForge
//!
//! Based on ColaNode's account-workspace-user model:
//! - Account: Top-level identity for authentication
//! - Workspace: Container (project) that hosts multiple users
//! - WorkspaceMember: Workspace-scoped user identity

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ============================================================
// AUTHENTICATION MODELS
// ============================================================

/// Account status values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AccountStatus {
	Unverified = 0,
	Verified = 1,
	Suspended = 2,
}

impl From<i32> for AccountStatus {
	fn from(value: i32) -> Self {
		match value {
			1 => AccountStatus::Verified,
			2 => AccountStatus::Suspended,
			_ => AccountStatus::Unverified,
		}
	}
}

impl From<AccountStatus> for i32 {
	fn from(status: AccountStatus) -> Self {
		status as i32
	}
}

/// Top-level authentication identity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
	pub id: Uuid,
	pub email: String,
	#[serde(skip_serializing)]
	pub password_hash: Option<String>,
	pub name: String,
	pub avatar_url: Option<String>,
	pub status: AccountStatus,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
}

/// Workspace (container for wells and users)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workspace {
	pub id: Uuid,
	pub name: String,
	pub description: Option<String>,
	pub avatar_url: Option<String>,
	pub owner_account_id: Uuid,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
}

/// Workspace member role
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WorkspaceRole {
	Owner,
	Admin,
	Member,
	Viewer,
}

impl std::fmt::Display for WorkspaceRole {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			WorkspaceRole::Owner => write!(f, "owner"),
			WorkspaceRole::Admin => write!(f, "admin"),
			WorkspaceRole::Member => write!(f, "member"),
			WorkspaceRole::Viewer => write!(f, "viewer"),
		}
	}
}

impl std::str::FromStr for WorkspaceRole {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_lowercase().as_str() {
			"owner" => Ok(WorkspaceRole::Owner),
			"admin" => Ok(WorkspaceRole::Admin),
			"member" => Ok(WorkspaceRole::Member),
			"viewer" => Ok(WorkspaceRole::Viewer),
			_ => Err(format!("Unknown workspace role: {}", s)),
		}
	}
}

/// Workspace-scoped user identity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceMember {
	pub id: Uuid,
	pub workspace_id: Uuid,
	pub account_id: Uuid,
	pub display_name: Option<String>,
	pub role: WorkspaceRole,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
}

/// Workspace member info for UI display (includes account details)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceMemberInfo {
	pub member_id: Uuid,
	pub account_id: Uuid,
	pub email: String,
	pub name: String,
	pub avatar_url: Option<String>,
	pub role: WorkspaceRole,
	pub joined_at: DateTime<Utc>,
}

/// Workspace invite for adding new members
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceInvite {
	pub id: Uuid,
	pub workspace_id: Uuid,
	pub email: String,
	pub role: WorkspaceRole,
	pub invited_by: Uuid,
	pub token: String,
	pub expires_at: DateTime<Utc>,
	pub accepted_at: Option<DateTime<Utc>>,
	pub created_at: DateTime<Utc>,
}

/// Session for local authentication state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
	pub id: Uuid,
	pub account_id: Uuid,
	#[serde(skip_serializing)]
	pub token_hash: String,
	pub device_name: Option<String>,
	pub created_at: DateTime<Utc>,
	pub expires_at: DateTime<Utc>,
	pub last_active_at: DateTime<Utc>,
}

/// Authentication result returned after login/register
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResult {
	pub account: Account,
	pub session_token: String,
	pub workspaces: Vec<Workspace>,
}

// ============================================================
// WELL MODELS
// ============================================================

use crate::wellgrid::{DepthUnit, ResampleMethod, WellDepthGrid};

/// A well containing curve data (belongs directly to a workspace)
///
/// Each well has a canonical depth grid that defines the sampling
/// interval for all curves. When curves are imported, they are
/// resampled to align with this grid.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Well {
	pub id: Uuid,
	pub workspace_id: Uuid,
	pub name: String,
	pub uwi: Option<String>,
	pub field: Option<String>,
	pub company: Option<String>,
	pub location: Option<String>,
	pub x: Option<f64>,
	pub y: Option<f64>,
	/// Depth unit for this well's canonical grid (ft or m)
	pub depth_unit: DepthUnit,
	/// Depth step size (e.g., 0.5 for half-foot sampling)
	pub depth_step: f64,
	/// Depth grid origin (typically 0.0)
	pub depth_origin: f64,
	/// Minimum depth across all curves (computed)
	pub min_depth: Option<f64>,
	/// Maximum depth across all curves (computed)
	pub max_depth: Option<f64>,
	pub created_by: Uuid,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
	pub version: i64,
	pub deleted_at: Option<DateTime<Utc>>,
}

impl Well {
	/// Get the depth grid configuration for this well
	pub fn depth_grid(&self) -> crate::wellgrid::WellDepthGrid {
		crate::wellgrid::WellDepthGrid::new(self.depth_unit, self.depth_step, self.depth_origin)
	}
}

// ============================================================
// LOG RUN MODELS (LAS Upload Events)
// ============================================================

/// A log run represents a single LAS file upload event
///
/// Each LAS file import creates a log_run record that preserves
/// the original file metadata and provenance. Multiple curves
/// from the same LAS file share the same log_run_id.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogRun {
	pub id: Uuid,
	pub well_id: Uuid,
	pub workspace_id: Uuid,
	/// Original filename
	pub source_filename: String,
	/// SHA-256 hash of original file (for deduplication)
	pub source_file_hash: Option<String>,
	/// Blob hash of stored raw LAS file
	pub raw_file_blob_hash: Option<String>,
	/// Log type (raw, processed, interpreted, composite)
	pub log_type_id: Option<String>,
	/// Acquisition type (wireline, lwd, mwd, etc.)
	pub acquisition_type_id: Option<String>,
	/// Run number within the well
	pub run_number: Option<i32>,
	/// Service company name
	pub service_company: Option<String>,
	/// Logging tool name
	pub tool_name: Option<String>,
	/// Date of logging operation
	pub logging_date: Option<String>,
	/// Original top depth (before resampling)
	pub original_top_depth: Option<f64>,
	/// Original bottom depth (before resampling)
	pub original_bottom_depth: Option<f64>,
	/// Original step size
	pub original_step: Option<f64>,
	/// Original depth unit
	pub original_depth_unit: Option<String>,
	/// Original null value from LAS file
	pub original_null_value: Option<f64>,
	/// LAS file version
	pub las_version: Option<String>,
	/// Who imported this file
	pub imported_by: Option<Uuid>,
	pub imported_at: DateTime<Utc>,
	pub version: i64,
	pub deleted_at: Option<DateTime<Utc>>,
}

// ============================================================
// MEASUREMENT TYPES AND UNITS (OSDU-inspired Unit Service)
// ============================================================

/// Measurement type classification (e.g., "length", "pressure", "resistivity")
///
/// Following OSDU patterns, this provides a centralized registry of
/// physical quantity types that can be used to validate unit compatibility.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeasurementType {
	pub id: String,
	pub name: String,
	pub description: Option<String>,
	/// ID of the base unit for this measurement type
	pub base_unit_id: Option<String>,
	pub created_at: DateTime<Utc>,
}

/// Unit definition with conversion factors
///
/// Units are linked to measurement types and include conversion factors
/// to allow automatic unit conversions within the same measurement type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Unit {
	pub id: String,
	pub measurement_type_id: String,
	/// Unit symbol (e.g., "ft", "m", "psi")
	pub symbol: String,
	/// Full unit name (e.g., "feet", "meters", "pounds per square inch")
	pub name: String,
	/// Multiply by this factor to convert to base unit
	pub to_base_factor: f64,
	/// Add this offset after multiplying (for temperature conversions)
	pub to_base_offset: f64,
	/// Whether this is the base unit for its measurement type
	pub is_base: bool,
	pub created_at: DateTime<Utc>,
}

impl Unit {
	/// Convert a value from this unit to the base unit
	pub fn to_base(&self, value: f64) -> f64 {
		value * self.to_base_factor + self.to_base_offset
	}

	/// Convert a value from the base unit to this unit
	pub fn from_base(&self, value: f64) -> f64 {
		(value - self.to_base_offset) / self.to_base_factor
	}
}

/// Curve quality/validity flag
///
/// Tracks the processing state of a curve, following OSDU patterns
/// for data quality indicators.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum CurveQualityFlag {
	/// Raw, unprocessed field data
	#[default]
	Raw,
	/// Quality-controlled and edited data
	Edited,
	/// Derived or computed from other curves
	Computed,
	/// Spliced from multiple sources
	Spliced,
	/// Reprocessed data
	Reprocessed,
}

impl std::fmt::Display for CurveQualityFlag {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			CurveQualityFlag::Raw => write!(f, "raw"),
			CurveQualityFlag::Edited => write!(f, "edited"),
			CurveQualityFlag::Computed => write!(f, "computed"),
			CurveQualityFlag::Spliced => write!(f, "spliced"),
			CurveQualityFlag::Reprocessed => write!(f, "reprocessed"),
		}
	}
}

impl std::str::FromStr for CurveQualityFlag {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_lowercase().as_str() {
			"raw" => Ok(CurveQualityFlag::Raw),
			"edited" => Ok(CurveQualityFlag::Edited),
			"computed" => Ok(CurveQualityFlag::Computed),
			"spliced" => Ok(CurveQualityFlag::Spliced),
			"reprocessed" => Ok(CurveQualityFlag::Reprocessed),
			_ => Err(format!("Unknown quality flag: {}", s)),
		}
	}
}

/// Reason for creating a curve version
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CurveVersionReason {
	/// Curve was spliced with another
	Splice,
	/// Environmental correction applied
	EnvironmentalCorrection,
	/// Curve was reprocessed
	Reprocess,
	/// Manual edit
	Edit,
	/// Curves were merged
	Merge,
}

impl std::fmt::Display for CurveVersionReason {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			CurveVersionReason::Splice => write!(f, "splice"),
			CurveVersionReason::EnvironmentalCorrection => write!(f, "environmental_correction"),
			CurveVersionReason::Reprocess => write!(f, "reprocess"),
			CurveVersionReason::Edit => write!(f, "edit"),
			CurveVersionReason::Merge => write!(f, "merge"),
		}
	}
}

impl std::str::FromStr for CurveVersionReason {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_lowercase().replace('-', "_").as_str() {
			"splice" => Ok(CurveVersionReason::Splice),
			"environmental_correction" => Ok(CurveVersionReason::EnvironmentalCorrection),
			"reprocess" => Ok(CurveVersionReason::Reprocess),
			"edit" => Ok(CurveVersionReason::Edit),
			"merge" => Ok(CurveVersionReason::Merge),
			_ => Err(format!("Unknown version reason: {}", s)),
		}
	}
}

/// A curve version record for tracking edits/reprocessing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurveVersion {
	pub id: Uuid,
	pub curve_id: Uuid,
	pub version: i32,
	pub created_at: DateTime<Utc>,
	pub created_by: Option<Uuid>,
	pub reason: Option<CurveVersionReason>,
	pub native_parquet_hash: Option<String>,
	pub gridded_parquet_hash: Option<String>,
	pub min_value: Option<f64>,
	pub max_value: Option<f64>,
	pub mean_value: Option<f64>,
	pub null_count: Option<i64>,
}

// ============================================================
// CURVE PROPERTY MODELS (PWLS-style Dictionary)
// ============================================================

/// Canonical curve property (e.g., "Gamma Ray", "Bulk Density")
///
/// Maps multiple vendor mnemonics to a single canonical property.
/// Enhanced with measurement_type_id for unit validation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurveProperty {
	pub id: String,
	pub canonical_name: String,
	pub property_class: Option<String>,
	/// Link to measurement type for unit validation
	pub measurement_type_id: Option<String>,
	pub typical_unit: Option<String>,
	pub description: Option<String>,
	pub display_color: Option<String>,
	pub log_scale: bool,
	/// Expected minimum valid value for this property
	pub min_valid_value: Option<f64>,
	/// Expected maximum valid value for this property
	pub max_valid_value: Option<f64>,
	pub created_at: DateTime<Utc>,
}

/// Mnemonic to property mapping
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurveMnemonic {
	pub mnemonic: String,
	pub property_id: Option<String>,
	pub priority: i32,
	pub vendor: Option<String>,
	pub notes: Option<String>,
}

/// Log type classification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogType {
	pub id: String,
	pub name: String,
	pub description: Option<String>,
}

/// Acquisition type classification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcquisitionType {
	pub id: String,
	pub name: String,
	pub description: Option<String>,
}

/// Metadata about a curve type (e.g., GR, RHOB, NPHI)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurveMetadata {
	pub id: Uuid,
	pub curve_mnemonic: String,
	pub main_curve_type: MainCurveType,
	pub subcurve_name: Option<String>,
	pub display_name: String,
	pub description: Option<String>,
	pub units: String,
	pub vendor_variations: Vec<String>,
	pub created_at: DateTime<Utc>,
}

/// Main curve type categories
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MainCurveType {
	GR,    // Gamma Ray
	RT,    // Resistivity
	RHOB,  // Bulk Density
	NPHI,  // Neutron Porosity
	CALI,  // Caliper
	DT,    // Sonic
	SP,    // Spontaneous Potential
	PE,    // Photoelectric
	DEPTH, // Depth index
	OTHER, // Other/Unknown
}

impl std::fmt::Display for MainCurveType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			MainCurveType::GR => write!(f, "GR"),
			MainCurveType::RT => write!(f, "RT"),
			MainCurveType::RHOB => write!(f, "RHOB"),
			MainCurveType::NPHI => write!(f, "NPHI"),
			MainCurveType::CALI => write!(f, "CALI"),
			MainCurveType::DT => write!(f, "DT"),
			MainCurveType::SP => write!(f, "SP"),
			MainCurveType::PE => write!(f, "PE"),
			MainCurveType::DEPTH => write!(f, "DEPTH"),
			MainCurveType::OTHER => write!(f, "OTHER"),
		}
	}
}

impl std::str::FromStr for MainCurveType {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_uppercase().as_str() {
			"GR" => Ok(MainCurveType::GR),
			"RT" => Ok(MainCurveType::RT),
			"RHOB" => Ok(MainCurveType::RHOB),
			"NPHI" => Ok(MainCurveType::NPHI),
			"CALI" => Ok(MainCurveType::CALI),
			"DT" => Ok(MainCurveType::DT),
			"SP" => Ok(MainCurveType::SP),
			"PE" => Ok(MainCurveType::PE),
			"DEPTH" => Ok(MainCurveType::DEPTH),
			"OTHER" => Ok(MainCurveType::OTHER),
			_ => Err(format!("Unknown curve type: {}", s)),
		}
	}
}

/// A curve instance linked to a well and log run
///
/// Each curve stores two representations:
/// - **Native**: Original data at original sampling, stored immutably
/// - **Gridded**: Resampled to well's canonical depth grid for analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Curve {
	pub id: Uuid,
	/// Log run this curve belongs to (LAS import event)
	pub log_run_id: Uuid,
	/// Well this curve belongs to (denormalized for queries)
	pub well_id: Uuid,
	/// Original mnemonic from LAS file (e.g., "GR", "GR1", "SGR")
	pub mnemonic: String,
	/// Mapped canonical property (e.g., "gamma_ray")
	pub property_id: Option<String>,
	/// Original unit from LAS file
	pub original_unit: Option<String>,
	/// Linked unit ID for unit conversion support
	pub unit_id: Option<String>,

	// ===== Native storage (original data) =====
	/// Top depth in native data
	pub native_top_depth: Option<f64>,
	/// Bottom depth in native data
	pub native_bottom_depth: Option<f64>,
	/// Original step size (may be irregular)
	pub native_step: Option<f64>,
	/// Number of samples in native data
	pub native_sample_count: Option<i64>,
	/// SHA-256 hash of native Parquet file
	pub native_parquet_hash: Option<String>,

	// ===== Gridded storage (resampled data) =====
	/// Top depth after resampling to well grid
	pub gridded_top_depth: Option<f64>,
	/// Bottom depth after resampling to well grid
	pub gridded_bottom_depth: Option<f64>,
	/// Number of samples in gridded data
	pub gridded_sample_count: Option<i64>,
	/// Resampling method used
	pub resample_method: Option<ResampleMethod>,
	/// Whether unit conversion was applied
	pub was_unit_converted: bool,
	/// Original depth unit before conversion
	pub source_depth_unit: Option<String>,
	/// SHA-256 hash of gridded Parquet file
	pub gridded_parquet_hash: Option<String>,

	// ===== Statistics =====
	pub null_count: Option<i64>,
	pub min_value: Option<f64>,
	pub max_value: Option<f64>,
	pub mean_value: Option<f64>,

	// ===== Quality/Validity fields (OSDU-inspired) =====
	/// The null/missing value marker from LAS file
	pub null_value: Option<f64>,
	/// Quality flag indicating processing state
	pub quality_flag: CurveQualityFlag,
	/// Date when the data was acquired (if known)
	pub acquisition_date: Option<String>,
	/// Service company that acquired the data
	pub service_company: Option<String>,

	pub created_by: Option<Uuid>,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
	pub version: i64,
	pub deleted_at: Option<DateTime<Utc>>,
}

/// Registry entry for a stored blob
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlobEntry {
	pub hash: String,
	pub size_bytes: i64,
	pub created_at: DateTime<Utc>,
	pub last_accessed_at: Option<DateTime<Utc>>,
	pub synced_to_server: bool,
	pub synced_at: Option<DateTime<Utc>>,
}

// ============================================================
// UNIFIED VIEW MODELS (Chunked Analysis Views)
// ============================================================

/// Status of a unified view chunk
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum UnifiedViewStatus {
	#[default]
	Pending,
	Building,
	Ready,
	Stale,
	Error,
}

impl std::fmt::Display for UnifiedViewStatus {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			UnifiedViewStatus::Pending => write!(f, "pending"),
			UnifiedViewStatus::Building => write!(f, "building"),
			UnifiedViewStatus::Ready => write!(f, "ready"),
			UnifiedViewStatus::Stale => write!(f, "stale"),
			UnifiedViewStatus::Error => write!(f, "error"),
		}
	}
}

impl std::str::FromStr for UnifiedViewStatus {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_lowercase().as_str() {
			"pending" => Ok(UnifiedViewStatus::Pending),
			"building" => Ok(UnifiedViewStatus::Building),
			"ready" => Ok(UnifiedViewStatus::Ready),
			"stale" => Ok(UnifiedViewStatus::Stale),
			"error" => Ok(UnifiedViewStatus::Error),
			_ => Err(format!("Unknown unified view status: {}", s)),
		}
	}
}

/// A unified view chunk for fast analysis queries
///
/// Unified views combine multiple curves from a well into a single
/// wide Parquet file aligned to the well's depth grid. They are
/// chunked by depth range for efficient memory usage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedView {
	pub id: Uuid,
	pub well_id: Uuid,
	pub workspace_id: Uuid,
	/// Chunk number (0-indexed)
	pub chunk_index: i32,
	/// Start depth of this chunk
	pub start_depth: f64,
	/// End depth of this chunk
	pub end_depth: f64,
	/// Curve IDs included in this chunk
	pub curve_ids: Vec<Uuid>,
	/// Number of rows in the chunk
	pub row_count: i64,
	/// SHA-256 hash of the Parquet file
	pub parquet_hash: String,
	/// View status
	pub status: UnifiedViewStatus,
	/// Last error message (if status is Error)
	pub last_error: Option<String>,
	/// Version of source data when view was built
	pub source_version: i64,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
}

impl UnifiedView {
	/// Check if this view needs rebuilding
	pub fn needs_rebuild(&self, current_version: i64) -> bool {
		self.status == UnifiedViewStatus::Stale || self.source_version < current_version
	}
}

// ============================================================
// SYNC MODELS (ColaNode-style Git-like sync)
// ============================================================

/// Sync status for a workspace
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum SyncStatus {
	#[default]
	Idle,
	Syncing,
	Error,
	Offline,
}

impl std::fmt::Display for SyncStatus {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			SyncStatus::Idle => write!(f, "idle"),
			SyncStatus::Syncing => write!(f, "syncing"),
			SyncStatus::Error => write!(f, "error"),
			SyncStatus::Offline => write!(f, "offline"),
		}
	}
}

impl std::str::FromStr for SyncStatus {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_lowercase().as_str() {
			"idle" => Ok(SyncStatus::Idle),
			"syncing" => Ok(SyncStatus::Syncing),
			"error" => Ok(SyncStatus::Error),
			"offline" => Ok(SyncStatus::Offline),
			_ => Err(format!("Unknown sync status: {}", s)),
		}
	}
}

/// Sync state for a workspace (tracks sync progress per workspace)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncState {
	pub workspace_id: Uuid,
	pub server_url: String,
	pub last_sync_version: i64,
	pub last_sync_at: Option<DateTime<Utc>>,
	pub last_push_at: Option<DateTime<Utc>>,
	pub last_pull_at: Option<DateTime<Utc>>,
	pub sync_status: SyncStatus,
	pub last_error: Option<String>,
	/// Conflict resolution strategy for this workspace
	pub conflict_strategy: ConflictStrategy,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
}

impl SyncState {
	/// Create a new sync state for a workspace
	pub fn new(workspace_id: Uuid, server_url: String) -> Self {
		let now = Utc::now();
		Self {
			workspace_id,
			server_url,
			last_sync_version: 0,
			last_sync_at: None,
			last_push_at: None,
			last_pull_at: None,
			sync_status: SyncStatus::Idle,
			last_error: None,
			conflict_strategy: ConflictStrategy::default(),
			created_at: now,
			updated_at: now,
		}
	}

	/// Create a new sync state with a specific conflict strategy
	pub fn with_strategy(workspace_id: Uuid, server_url: String, strategy: ConflictStrategy) -> Self {
		let mut state = Self::new(workspace_id, server_url);
		state.conflict_strategy = strategy;
		state
	}
}

/// Sync action type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SyncAction {
	Create,
	Update,
	Delete,
}

impl std::fmt::Display for SyncAction {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			SyncAction::Create => write!(f, "create"),
			SyncAction::Update => write!(f, "update"),
			SyncAction::Delete => write!(f, "delete"),
		}
	}
}

impl std::str::FromStr for SyncAction {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_lowercase().as_str() {
			"create" => Ok(SyncAction::Create),
			"update" => Ok(SyncAction::Update),
			"delete" => Ok(SyncAction::Delete),
			_ => Err(format!("Unknown sync action: {}", s)),
		}
	}
}

/// Entry in the sync queue for offline changes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncQueueEntry {
	pub id: i64,
	pub workspace_id: Uuid,
	pub entity_type: String,
	pub entity_id: Uuid,
	pub action: SyncAction,
	pub version: i64,
	pub payload: Option<String>,
	pub blob_hashes: Option<Vec<String>>,
	pub priority: i32,
	pub created_at: DateTime<Utc>,
	pub attempts: i32,
	pub last_attempt_at: Option<DateTime<Utc>>,
	pub last_error: Option<String>,
	pub synced_at: Option<DateTime<Utc>>,
}

impl SyncQueueEntry {
	/// Create a new sync queue entry
	pub fn new(
		workspace_id: Uuid,
		entity_type: impl Into<String>,
		entity_id: Uuid,
		action: SyncAction,
		version: i64,
		payload: Option<String>,
		blob_hashes: Option<Vec<String>>,
	) -> Self {
		Self {
			id: 0, // Will be set by database
			workspace_id,
			entity_type: entity_type.into(),
			entity_id,
			action,
			version,
			payload,
			blob_hashes,
			priority: 0,
			created_at: Utc::now(),
			attempts: 0,
			last_attempt_at: None,
			last_error: None,
			synced_at: None,
		}
	}

	/// Check if this entry has been synced
	pub fn is_synced(&self) -> bool {
		self.synced_at.is_some()
	}

	/// Check if this entry is pending (not synced)
	pub fn is_pending(&self) -> bool {
		self.synced_at.is_none()
	}
}

/// Conflict resolution strategy (how a specific conflict was resolved)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConflictResolution {
	Pending,
	Local,
	Remote,
	Merged,
}

/// Conflict resolution strategy for automatic handling
///
/// Following ColaNode patterns, workspaces can configure how conflicts
/// are automatically resolved during pull operations:
/// - Manual: Queue conflicts for user review (default, safest)
/// - LastWriteWins: Automatically keep the newer change based on timestamp
/// - LocalWins: Always keep local changes
/// - RemoteWins: Always accept remote changes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ConflictStrategy {
	/// Queue all conflicts for manual resolution (default, safest)
	#[default]
	Manual,
	/// Automatically resolve using timestamps - newer change wins
	LastWriteWins,
	/// Always keep local changes
	LocalWins,
	/// Always accept remote changes
	RemoteWins,
}

impl std::fmt::Display for ConflictResolution {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			ConflictResolution::Pending => write!(f, "pending"),
			ConflictResolution::Local => write!(f, "local"),
			ConflictResolution::Remote => write!(f, "remote"),
			ConflictResolution::Merged => write!(f, "merged"),
		}
	}
}

impl std::str::FromStr for ConflictResolution {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_lowercase().as_str() {
			"pending" => Ok(ConflictResolution::Pending),
			"local" => Ok(ConflictResolution::Local),
			"remote" => Ok(ConflictResolution::Remote),
			"merged" => Ok(ConflictResolution::Merged),
			_ => Err(format!("Unknown conflict resolution: {}", s)),
		}
	}
}

impl std::fmt::Display for ConflictStrategy {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			ConflictStrategy::Manual => write!(f, "manual"),
			ConflictStrategy::LastWriteWins => write!(f, "last_write_wins"),
			ConflictStrategy::LocalWins => write!(f, "local_wins"),
			ConflictStrategy::RemoteWins => write!(f, "remote_wins"),
		}
	}
}

impl std::str::FromStr for ConflictStrategy {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_lowercase().replace('-', "_").as_str() {
			"manual" => Ok(ConflictStrategy::Manual),
			"last_write_wins" | "lastwritewins" => Ok(ConflictStrategy::LastWriteWins),
			"local_wins" | "localwins" => Ok(ConflictStrategy::LocalWins),
			"remote_wins" | "remotewins" => Ok(ConflictStrategy::RemoteWins),
			_ => Err(format!("Unknown conflict strategy: {}", s)),
		}
	}
}

/// A sync conflict requiring resolution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncConflict {
	pub id: i64,
	pub workspace_id: Uuid,
	pub entity_type: String,
	pub entity_id: Uuid,
	pub local_version: i64,
	pub local_data: String,
	pub remote_version: i64,
	pub remote_data: String,
	pub resolution: ConflictResolution,
	pub resolved_by: Option<Uuid>,
	pub resolved_at: Option<DateTime<Utc>>,
	pub created_at: DateTime<Utc>,
}

impl SyncConflict {
	/// Create a new pending conflict
	pub fn new(
		workspace_id: Uuid,
		entity_type: impl Into<String>,
		entity_id: Uuid,
		local_version: i64,
		local_data: String,
		remote_version: i64,
		remote_data: String,
	) -> Self {
		Self {
			id: 0, // Will be set by database
			workspace_id,
			entity_type: entity_type.into(),
			entity_id,
			local_version,
			local_data,
			remote_version,
			remote_data,
			resolution: ConflictResolution::Pending,
			resolved_by: None,
			resolved_at: None,
			created_at: Utc::now(),
		}
	}

	/// Check if this conflict is pending resolution
	pub fn is_pending(&self) -> bool {
		self.resolution == ConflictResolution::Pending
	}
}

/// Summary of sync state for frontend display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncSummary {
	pub status: SyncStatus,
	pub last_sync_at: Option<DateTime<Utc>>,
	pub pending_changes: i64,
	pub conflicts: i64,
	pub error: Option<String>,
}

// ============================================================
// TRAJECTORY MODELS (Survey/Directional Data)
// ============================================================

/// Azimuth reference type for trajectory surveys
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum AzimuthReference {
	/// True North (geographic north)
	#[default]
	TrueNorth,
	/// Grid North (map projection north)
	GridNorth,
	/// Magnetic North (compass reading)
	MagneticNorth,
}

impl std::fmt::Display for AzimuthReference {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			AzimuthReference::TrueNorth => write!(f, "true_north"),
			AzimuthReference::GridNorth => write!(f, "grid_north"),
			AzimuthReference::MagneticNorth => write!(f, "magnetic_north"),
		}
	}
}

impl std::str::FromStr for AzimuthReference {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_lowercase().replace('-', "_").as_str() {
			"true_north" | "truenorth" | "tn" => Ok(AzimuthReference::TrueNorth),
			"grid_north" | "gridnorth" | "gn" => Ok(AzimuthReference::GridNorth),
			"magnetic_north" | "magneticnorth" | "mn" => Ok(AzimuthReference::MagneticNorth),
			_ => Err(format!("Unknown azimuth reference: {}", s)),
		}
	}
}

/// Vertical reference type for trajectory surveys
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum VerticalReference {
	/// Kelly Bushing (drilling floor reference)
	#[default]
	KellyBushing,
	/// Rotary Table
	RotaryTable,
	/// Mean Sea Level
	MeanSeaLevel,
	/// Ground Level
	GroundLevel,
}

impl std::fmt::Display for VerticalReference {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			VerticalReference::KellyBushing => write!(f, "kb"),
			VerticalReference::RotaryTable => write!(f, "rt"),
			VerticalReference::MeanSeaLevel => write!(f, "msl"),
			VerticalReference::GroundLevel => write!(f, "ground_level"),
		}
	}
}

impl std::str::FromStr for VerticalReference {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_lowercase().replace('-', "_").as_str() {
			"kb" | "kelly_bushing" | "kellybushing" => Ok(VerticalReference::KellyBushing),
			"rt" | "rotary_table" | "rotarytable" => Ok(VerticalReference::RotaryTable),
			"msl" | "mean_sea_level" | "meansealevel" => Ok(VerticalReference::MeanSeaLevel),
			"ground_level" | "groundlevel" | "gl" => Ok(VerticalReference::GroundLevel),
			_ => Err(format!("Unknown vertical reference: {}", s)),
		}
	}
}

/// Survey type classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum SurveyType {
	/// Definitive survey (final, QC'd data)
	#[default]
	Definitive,
	/// Preliminary survey (not fully QC'd)
	Preliminary,
	/// MWD (Measurement While Drilling) survey
	Mwd,
	/// Gyro survey (gyroscopic measurement)
	Gyro,
	/// Composite of multiple surveys
	Composite,
}

impl std::fmt::Display for SurveyType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			SurveyType::Definitive => write!(f, "definitive"),
			SurveyType::Preliminary => write!(f, "preliminary"),
			SurveyType::Mwd => write!(f, "mwd"),
			SurveyType::Gyro => write!(f, "gyro"),
			SurveyType::Composite => write!(f, "composite"),
		}
	}
}

impl std::str::FromStr for SurveyType {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_lowercase().as_str() {
			"definitive" | "def" => Ok(SurveyType::Definitive),
			"preliminary" | "prelim" => Ok(SurveyType::Preliminary),
			"mwd" => Ok(SurveyType::Mwd),
			"gyro" | "gyroscopic" => Ok(SurveyType::Gyro),
			"composite" | "comp" => Ok(SurveyType::Composite),
			_ => Err(format!("Unknown survey type: {}", s)),
		}
	}
}

/// Calculation method for trajectory computations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum CalculationMethod {
	/// Minimum curvature method (industry standard)
	#[default]
	MinimumCurvature,
	/// Balanced tangential method
	BalancedTangential,
	/// Average angle method
	AverageAngle,
	/// Radius of curvature method
	RadiusOfCurvature,
	/// Tangential method (simple but less accurate)
	Tangential,
}

impl std::fmt::Display for CalculationMethod {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			CalculationMethod::MinimumCurvature => write!(f, "minimum_curvature"),
			CalculationMethod::BalancedTangential => write!(f, "balanced_tangential"),
			CalculationMethod::AverageAngle => write!(f, "average_angle"),
			CalculationMethod::RadiusOfCurvature => write!(f, "radius_of_curvature"),
			CalculationMethod::Tangential => write!(f, "tangential"),
		}
	}
}

impl std::str::FromStr for CalculationMethod {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_lowercase().replace('-', "_").as_str() {
			"minimum_curvature" | "minimumcurvature" | "min_curv" => {
				Ok(CalculationMethod::MinimumCurvature)
			}
			"balanced_tangential" | "balancedtangential" => Ok(CalculationMethod::BalancedTangential),
			"average_angle" | "averageangle" => Ok(CalculationMethod::AverageAngle),
			"radius_of_curvature" | "radiusofcurvature" => Ok(CalculationMethod::RadiusOfCurvature),
			"tangential" => Ok(CalculationMethod::Tangential),
			_ => Err(format!("Unknown calculation method: {}", s)),
		}
	}
}

/// Trajectory column type classification
///
/// Following OSDU patterns, trajectory data consists of input stations
/// (measured values) and output stations (calculated values).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
#[serde(rename_all = "snake_case")]
pub enum TrajectoryColumnType {
	// ===== Input columns (measured values) =====
	/// Measured Depth
	MeasuredDepth,
	/// Inclination angle (degrees from vertical)
	Inclination,
	/// Azimuth angle (degrees from north reference)
	Azimuth,
	/// Incremental X offset (optional input)
	DeltaX,
	/// Incremental Y offset (optional input)
	DeltaY,
	/// Incremental Z offset (optional input)
	DeltaZ,

	// ===== Calculated columns (derived values) =====
	/// True Vertical Depth
	TrueVerticalDepth,
	/// North-South offset (positive = north)
	NorthSouth,
	/// East-West offset (positive = east)
	EastWest,
	/// Dogleg Severity (degrees per 100ft or 30m)
	DoglegSeverity,
	/// X coordinate in local CRS
	LocalX,
	/// Y coordinate in local CRS
	LocalY,
	/// Latitude (WGS84)
	Latitude,
	/// Longitude (WGS84)
	Longitude,

	/// Unknown/other column type
	Unknown,
}

impl TrajectoryColumnType {
	/// Check if this is an input (measured) column
	pub fn is_input(&self) -> bool {
		matches!(
			self,
			TrajectoryColumnType::MeasuredDepth
				| TrajectoryColumnType::Inclination
				| TrajectoryColumnType::Azimuth
				| TrajectoryColumnType::DeltaX
				| TrajectoryColumnType::DeltaY
				| TrajectoryColumnType::DeltaZ
		)
	}

	/// Check if this is a calculated (derived) column
	pub fn is_calculated(&self) -> bool {
		matches!(
			self,
			TrajectoryColumnType::TrueVerticalDepth
				| TrajectoryColumnType::NorthSouth
				| TrajectoryColumnType::EastWest
				| TrajectoryColumnType::DoglegSeverity
				| TrajectoryColumnType::LocalX
				| TrajectoryColumnType::LocalY
				| TrajectoryColumnType::Latitude
				| TrajectoryColumnType::Longitude
		)
	}

	/// Get the canonical name for this column type
	pub fn canonical_name(&self) -> &'static str {
		match self {
			TrajectoryColumnType::MeasuredDepth => "MD",
			TrajectoryColumnType::Inclination => "INC",
			TrajectoryColumnType::Azimuth => "AZI",
			TrajectoryColumnType::DeltaX => "DX",
			TrajectoryColumnType::DeltaY => "DY",
			TrajectoryColumnType::DeltaZ => "DZ",
			TrajectoryColumnType::TrueVerticalDepth => "TVD",
			TrajectoryColumnType::NorthSouth => "NS",
			TrajectoryColumnType::EastWest => "EW",
			TrajectoryColumnType::DoglegSeverity => "DLS",
			TrajectoryColumnType::LocalX => "X",
			TrajectoryColumnType::LocalY => "Y",
			TrajectoryColumnType::Latitude => "LAT",
			TrajectoryColumnType::Longitude => "LON",
			TrajectoryColumnType::Unknown => "UNKNOWN",
		}
	}
}

impl std::fmt::Display for TrajectoryColumnType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		// Output snake_case to match #[serde(rename_all = "snake_case")] and TypeScript types
		match self {
			TrajectoryColumnType::MeasuredDepth => write!(f, "measured_depth"),
			TrajectoryColumnType::Inclination => write!(f, "inclination"),
			TrajectoryColumnType::Azimuth => write!(f, "azimuth"),
			TrajectoryColumnType::DeltaX => write!(f, "delta_x"),
			TrajectoryColumnType::DeltaY => write!(f, "delta_y"),
			TrajectoryColumnType::DeltaZ => write!(f, "delta_z"),
			TrajectoryColumnType::TrueVerticalDepth => write!(f, "true_vertical_depth"),
			TrajectoryColumnType::NorthSouth => write!(f, "north_south"),
			TrajectoryColumnType::EastWest => write!(f, "east_west"),
			TrajectoryColumnType::DoglegSeverity => write!(f, "dogleg_severity"),
			TrajectoryColumnType::LocalX => write!(f, "local_x"),
			TrajectoryColumnType::LocalY => write!(f, "local_y"),
			TrajectoryColumnType::Latitude => write!(f, "latitude"),
			TrajectoryColumnType::Longitude => write!(f, "longitude"),
			TrajectoryColumnType::Unknown => write!(f, "unknown"),
		}
	}
}

impl std::str::FromStr for TrajectoryColumnType {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_lowercase().replace(['-', ' '], "_").as_str() {
			"md" | "measured_depth" | "measureddepth" | "depth" => {
				Ok(TrajectoryColumnType::MeasuredDepth)
			}
			"inc" | "incl" | "inclination" | "theta" | "dip" => {
				Ok(TrajectoryColumnType::Inclination)
			}
			"azi" | "azim" | "azimuth" | "phi" | "bearing" => Ok(TrajectoryColumnType::Azimuth),
			"dx" | "delta_x" | "deltax" => Ok(TrajectoryColumnType::DeltaX),
			"dy" | "delta_y" | "deltay" => Ok(TrajectoryColumnType::DeltaY),
			"dz" | "delta_z" | "deltaz" => Ok(TrajectoryColumnType::DeltaZ),
			"tvd" | "true_vertical_depth" | "trueverticaldepth" => {
				Ok(TrajectoryColumnType::TrueVerticalDepth)
			}
			"ns" | "north_south" | "northsouth" | "departure_ns" | "n_s" => {
				Ok(TrajectoryColumnType::NorthSouth)
			}
			"ew" | "east_west" | "eastwest" | "departure_ew" | "e_w" => {
				Ok(TrajectoryColumnType::EastWest)
			}
			"dls" | "dogleg" | "dogleg_severity" | "doglegseverity" => {
				Ok(TrajectoryColumnType::DoglegSeverity)
			}
			"local_x" | "localx" | "x" | "easting" => Ok(TrajectoryColumnType::LocalX),
			"local_y" | "localy" | "y" | "northing" => Ok(TrajectoryColumnType::LocalY),
			"lat" | "latitude" => Ok(TrajectoryColumnType::Latitude),
			"lon" | "lng" | "longitude" => Ok(TrajectoryColumnType::Longitude),
			_ => Ok(TrajectoryColumnType::Unknown),
		}
	}
}

// ============================================================
// CHECKSHOT MODELS
// ============================================================

/// Checkshot column type classification
///
/// Checkshots establish time-depth relationships for velocity calibration.
/// Following OSDU Checkshot pattern with MD, TVD, TWT as core inputs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
#[serde(rename_all = "snake_case")]
pub enum CheckshotColumnType {
	// ===== Core input columns =====
	/// Measured Depth (along hole)
	MeasuredDepth,
	/// True Vertical Depth (vertical distance)
	TrueVerticalDepth,
	/// Two-Way Time (seismic travel time)
	TwoWayTime,

	// ===== Optional/derived columns =====
	/// Interval Velocity (calculated from TVD/TWT)
	Velocity,
	/// One-Way Time (optional, TWT/2)
	OneWayTime,
	/// Data quality flag
	Quality,

	/// Unknown/other column type
	Unknown,
}

impl CheckshotColumnType {
	/// Check if this is a required input column
	pub fn is_required(&self) -> bool {
		matches!(
			self,
			CheckshotColumnType::MeasuredDepth
				| CheckshotColumnType::TrueVerticalDepth
				| CheckshotColumnType::TwoWayTime
		)
	}

	/// Check if this is a calculated (derived) column
	pub fn is_calculated(&self) -> bool {
		matches!(
			self,
			CheckshotColumnType::Velocity | CheckshotColumnType::OneWayTime
		)
	}

	/// Get the canonical name for this column type
	pub fn canonical_name(&self) -> &'static str {
		match self {
			CheckshotColumnType::MeasuredDepth => "MD",
			CheckshotColumnType::TrueVerticalDepth => "TVD",
			CheckshotColumnType::TwoWayTime => "TWT",
			CheckshotColumnType::Velocity => "VINT",
			CheckshotColumnType::OneWayTime => "OWT",
			CheckshotColumnType::Quality => "QUALITY",
			CheckshotColumnType::Unknown => "UNKNOWN",
		}
	}
}

impl std::fmt::Display for CheckshotColumnType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		// Output snake_case to match #[serde(rename_all = "snake_case")] and TypeScript types
		match self {
			CheckshotColumnType::MeasuredDepth => write!(f, "measured_depth"),
			CheckshotColumnType::TrueVerticalDepth => write!(f, "true_vertical_depth"),
			CheckshotColumnType::TwoWayTime => write!(f, "two_way_time"),
			CheckshotColumnType::Velocity => write!(f, "velocity"),
			CheckshotColumnType::OneWayTime => write!(f, "one_way_time"),
			CheckshotColumnType::Quality => write!(f, "quality"),
			CheckshotColumnType::Unknown => write!(f, "unknown"),
		}
	}
}

impl std::str::FromStr for CheckshotColumnType {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_lowercase().replace(['-', ' '], "_").as_str() {
			"md" | "measured_depth" | "measureddepth" | "depth" | "dept" => {
				Ok(CheckshotColumnType::MeasuredDepth)
			}
			"tvd" | "true_vertical_depth" | "trueverticaldepth" | "tvdss" | "tvd_ss" => {
				Ok(CheckshotColumnType::TrueVerticalDepth)
			}
			"twt" | "two_way_time" | "twowaytime" | "twt_s" | "twt_ms" | "time" => {
				Ok(CheckshotColumnType::TwoWayTime)
			}
			"velocity" | "vel" | "vint" | "interval_velocity" | "intervalvelocity" | "vavg" => {
				Ok(CheckshotColumnType::Velocity)
			}
			"owt" | "one_way_time" | "onewaytime" | "owt_s" | "owt_ms" => {
				Ok(CheckshotColumnType::OneWayTime)
			}
			"quality" | "qual" | "qc" | "flag" | "confidence" => Ok(CheckshotColumnType::Quality),
			_ => Ok(CheckshotColumnType::Unknown),
		}
	}
}

/// A survey run represents a single trajectory CSV upload event
///
/// Parallel structure to LogRun for curve data. Each trajectory CSV import
/// creates a survey_run record that preserves the original file metadata
/// and survey parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurveyRun {
	pub id: Uuid,
	pub well_id: Uuid,
	pub workspace_id: Uuid,

	// ===== Source file info (immutable) =====
	/// Original filename
	pub source_filename: String,
	/// SHA-256 hash of original file (for deduplication)
	pub source_file_hash: Option<String>,
	/// Blob hash of stored raw file
	pub raw_file_blob_hash: Option<String>,

	// ===== Survey metadata =====
	/// Type of survey (definitive, mwd, gyro, etc.)
	pub survey_type: Option<SurveyType>,
	/// Survey company name
	pub survey_company: Option<String>,
	/// Date of survey
	pub survey_date: Option<String>,
	/// Run number within the well
	pub survey_run_number: Option<i32>,

	// ===== Reference info =====
	/// Magnetic declination (degrees, positive = east)
	pub magnetic_declination: Option<f64>,
	/// Grid convergence (degrees)
	pub grid_convergence: Option<f64>,
	/// Azimuth reference (true north, grid north, magnetic north)
	pub azimuth_reference: Option<AzimuthReference>,
	/// Vertical reference (KB, RT, MSL, etc.)
	pub vertical_reference: Option<VerticalReference>,
	/// Vertical reference elevation (e.g., KB elevation above MSL)
	pub vertical_reference_elevation: Option<f64>,
	/// Source coordinate reference system (EPSG code or WKT)
	pub source_crs: Option<String>,
	/// Target coordinate reference system (for transformation)
	pub target_crs: Option<String>,

	// ===== Calculation metadata =====
	/// Calculation method used (minimum curvature, etc.)
	pub calculation_method: CalculationMethod,
	/// Tie-in point measured depth
	pub tie_in_md: Option<f64>,
	/// Tie-in point true vertical depth
	pub tie_in_tvd: Option<f64>,
	/// Tie-in point north-south offset
	pub tie_in_ns: Option<f64>,
	/// Tie-in point east-west offset
	pub tie_in_ew: Option<f64>,

	// ===== Depth range =====
	/// Top measured depth
	pub top_md: Option<f64>,
	/// Bottom measured depth
	pub bottom_md: Option<f64>,
	/// Number of survey stations
	pub station_count: Option<i32>,
	/// Depth unit (ft or m)
	pub md_unit: String,

	// ===== Tracking =====
	/// Who imported this file
	pub imported_by: Option<Uuid>,
	pub imported_at: DateTime<Utc>,
	pub version: i64,
	pub deleted_at: Option<DateTime<Utc>>,
}

/// A trajectory column instance linked to a survey run
///
/// Each column of trajectory data (MD, INC, AZI, TVD, etc.) is stored
/// separately as a Parquet file, following the same pattern as curves.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrajectoryColumn {
	pub id: Uuid,
	/// Survey run this column belongs to
	pub survey_run_id: Uuid,
	/// Well this column belongs to (denormalized for queries)
	pub well_id: Uuid,

	// ===== Column identity =====
	/// Type of trajectory data (md, inclination, azimuth, tvd, etc.)
	pub column_type: TrajectoryColumnType,
	/// Original column name from CSV
	pub column_name: String,
	/// Unit of this column (e.g., "ft", "m", "deg")
	pub unit: Option<String>,
	/// Linked unit ID for unit conversion support
	pub unit_id: Option<String>,
	/// Description of this column
	pub description: Option<String>,

	// ===== Data characteristics =====
	/// True if this is an input (measured) column
	pub is_input: bool,
	/// True if this is a calculated (derived) column
	pub is_calculated: bool,

	// ===== Statistics =====
	pub min_value: Option<f64>,
	pub max_value: Option<f64>,
	pub mean_value: Option<f64>,
	pub null_count: Option<i64>,
	pub sample_count: Option<i64>,

	// ===== Storage reference (content-addressed) =====
	/// SHA-256 hash of Parquet file containing this column's data
	pub parquet_hash: Option<String>,

	// ===== Tracking =====
	pub created_by: Option<Uuid>,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
	pub version: i64,
	pub deleted_at: Option<DateTime<Utc>>,
}

/// A single trajectory station (row of survey data)
///
/// Used for displaying trajectory data in the UI and for calculations.
/// This is a convenience struct that combines multiple column values.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrajectoryStation {
	/// Measured depth
	pub md: f64,
	/// Inclination angle (degrees)
	pub inclination: Option<f64>,
	/// Azimuth angle (degrees)
	pub azimuth: Option<f64>,
	/// True vertical depth (calculated)
	pub tvd: Option<f64>,
	/// North-South offset (calculated)
	pub ns: Option<f64>,
	/// East-West offset (calculated)
	pub ew: Option<f64>,
	/// Dogleg severity (calculated)
	pub dls: Option<f64>,
	/// Whether this is an original station vs interpolated
	pub is_original: bool,
}

/// Summary of a survey run for display in lists
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurveyRunSummary {
	pub id: Uuid,
	pub well_id: Uuid,
	pub source_filename: String,
	pub survey_type: Option<SurveyType>,
	pub top_md: Option<f64>,
	pub bottom_md: Option<f64>,
	pub station_count: Option<i32>,
	pub md_unit: String,
	pub imported_at: DateTime<Utc>,
}

// ============================================================
// MARKER MODELS (Well Tops / Formation Markers)
// ============================================================

/// Marker column type classification for CSV parsing
///
/// Following OSDU WellboreMarkerSet patterns with MarkerName and
/// MarkerMeasuredDepth as core required fields.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
#[serde(rename_all = "snake_case")]
pub enum MarkerColumnType {
	/// Well name/identifier (for multi-well files)
	WellName,
	/// Well UWI/API number
	WellUwi,
	/// Marker/formation name
	MarkerName,
	/// Measured depth of marker
	MeasuredDepth,
	/// True vertical depth
	TrueVerticalDepth,
	/// TVD sub-sea
	TrueVerticalDepthSS,
	/// Marker type classification
	MarkerType,
	/// Thickness to base
	Thickness,
	/// Quality/confidence indicator
	Quality,
	/// Interpreter name
	Interpreter,
	/// Pick date
	PickDate,
	/// Comments/notes
	Comments,
	/// Display color
	Color,
	/// Unknown column type
	Unknown,
}

impl MarkerColumnType {
	/// Check if this column is required for basic marker ingestion
	pub fn is_required(&self) -> bool {
		matches!(
			self,
			MarkerColumnType::MarkerName | MarkerColumnType::MeasuredDepth
		)
	}

	/// Get the canonical name for this column type
	pub fn canonical_name(&self) -> &'static str {
		match self {
			MarkerColumnType::WellName => "well_name",
			MarkerColumnType::WellUwi => "well_uwi",
			MarkerColumnType::MarkerName => "marker_name",
			MarkerColumnType::MeasuredDepth => "measured_depth",
			MarkerColumnType::TrueVerticalDepth => "true_vertical_depth",
			MarkerColumnType::TrueVerticalDepthSS => "true_vertical_depth_ss",
			MarkerColumnType::MarkerType => "marker_type",
			MarkerColumnType::Thickness => "thickness",
			MarkerColumnType::Quality => "quality",
			MarkerColumnType::Interpreter => "interpreter",
			MarkerColumnType::PickDate => "pick_date",
			MarkerColumnType::Comments => "comments",
			MarkerColumnType::Color => "color",
			MarkerColumnType::Unknown => "unknown",
		}
	}
}

impl std::fmt::Display for MarkerColumnType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.canonical_name())
	}
}

impl std::str::FromStr for MarkerColumnType {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_lowercase().replace(['-', ' '], "_").as_str() {
			"well" | "well_name" | "wellname" | "well_id" | "wellid" => {
				Ok(MarkerColumnType::WellName)
			}
			"uwi" | "api" | "api_number" | "well_uwi" => Ok(MarkerColumnType::WellUwi),
			"marker" | "marker_name" | "markername" | "formation" | "formation_name"
			| "top" | "top_name" | "horizon" | "zone" | "surface" => {
				Ok(MarkerColumnType::MarkerName)
			}
			"md" | "depth" | "measured_depth" | "measureddepth" | "pick_depth" | "top_depth" => {
				Ok(MarkerColumnType::MeasuredDepth)
			}
			"tvd" | "true_vertical_depth" | "trueverticaldepth" => {
				Ok(MarkerColumnType::TrueVerticalDepth)
			}
			"tvdss" | "tvd_ss" | "tvd_subsea" => Ok(MarkerColumnType::TrueVerticalDepthSS),
			"type" | "marker_type" | "markertype" | "top_type" | "pick_type" => {
				Ok(MarkerColumnType::MarkerType)
			}
			"thickness" | "thick" | "isochore" => Ok(MarkerColumnType::Thickness),
			"quality" | "confidence" | "certainty" | "status" => Ok(MarkerColumnType::Quality),
			"interpreter" | "picked_by" | "pickedby" | "analyst" | "author" => {
				Ok(MarkerColumnType::Interpreter)
			}
			"date" | "pick_date" | "pickdate" | "interpretation_date" => {
				Ok(MarkerColumnType::PickDate)
			}
			"comment" | "comments" | "notes" | "remark" | "remarks" => {
				Ok(MarkerColumnType::Comments)
			}
			"color" | "colour" | "display_color" => Ok(MarkerColumnType::Color),
			_ => Ok(MarkerColumnType::Unknown),
		}
	}
}

/// Interpretation type for marker sets
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum InterpretationType {
	/// Formation top picks
	#[default]
	Formation,
	/// Sequence stratigraphic boundaries
	Sequence,
	/// Zone boundaries
	Zone,
	/// Seismic horizons
	Horizon,
	/// Other interpretation type
	Other,
}

impl std::fmt::Display for InterpretationType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			InterpretationType::Formation => write!(f, "formation"),
			InterpretationType::Sequence => write!(f, "sequence"),
			InterpretationType::Zone => write!(f, "zone"),
			InterpretationType::Horizon => write!(f, "horizon"),
			InterpretationType::Other => write!(f, "other"),
		}
	}
}

impl std::str::FromStr for InterpretationType {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_lowercase().as_str() {
			"formation" | "fm" => Ok(InterpretationType::Formation),
			"sequence" | "seq" => Ok(InterpretationType::Sequence),
			"zone" => Ok(InterpretationType::Zone),
			"horizon" | "seismic" => Ok(InterpretationType::Horizon),
			"other" => Ok(InterpretationType::Other),
			_ => Err(format!("Unknown interpretation type: {}", s)),
		}
	}
}

/// Marker quality/confidence indicator
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum MarkerQuality {
	/// Confirmed pick with high confidence
	#[default]
	Confirmed,
	/// Uncertain pick
	Uncertain,
	/// Projected from nearby wells
	Projected,
	/// Formation absent at this location
	Absent,
}

impl std::fmt::Display for MarkerQuality {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			MarkerQuality::Confirmed => write!(f, "confirmed"),
			MarkerQuality::Uncertain => write!(f, "uncertain"),
			MarkerQuality::Projected => write!(f, "projected"),
			MarkerQuality::Absent => write!(f, "absent"),
		}
	}
}

impl std::str::FromStr for MarkerQuality {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_lowercase().as_str() {
			"confirmed" | "high" | "definite" => Ok(MarkerQuality::Confirmed),
			"uncertain" | "medium" | "low" => Ok(MarkerQuality::Uncertain),
			"projected" | "inferred" => Ok(MarkerQuality::Projected),
			"absent" | "missing" | "not_present" => Ok(MarkerQuality::Absent),
			_ => Err(format!("Unknown marker quality: {}", s)),
		}
	}
}

/// A marker set represents a single marker CSV upload event per well
///
/// Following OSDU WellboreMarkerSet patterns. Each marker CSV import
/// creates one or more marker_set records (one per well in multi-well files).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkerSet {
	pub id: Uuid,
	pub well_id: Uuid,
	pub workspace_id: Uuid,

	// ===== Source file info (immutable) =====
	/// Original filename
	pub source_filename: String,
	/// SHA-256 hash of original file
	pub source_file_hash: Option<String>,
	/// Blob hash of stored raw file
	pub raw_file_blob_hash: Option<String>,

	// ===== Marker set metadata =====
	/// Name of this marker set (e.g., "Formation Tops", "Sequence Boundaries")
	pub name: Option<String>,
	/// Type of interpretation
	pub interpretation_type: Option<InterpretationType>,
	/// Who interpreted/picked these markers
	pub interpreter: Option<String>,
	/// Reference source for the interpretation
	pub reference_source: Option<String>,

	// ===== Depth info =====
	/// Depth unit (ft or m)
	pub depth_unit: String,
	/// Depth reference (KB, RT, MSL, etc.)
	pub depth_reference: Option<String>,
	/// Minimum depth in this set
	pub min_depth: Option<f64>,
	/// Maximum depth in this set
	pub max_depth: Option<f64>,
	/// Number of markers in this set
	pub marker_count: Option<i32>,

	// ===== Tracking =====
	/// Who imported this file
	pub imported_by: Option<Uuid>,
	pub imported_at: DateTime<Utc>,
	pub version: i64,
	pub deleted_at: Option<DateTime<Utc>>,
}

/// An individual marker (well top / formation pick)
///
/// Following OSDU WellboreMarkerSet patterns with MarkerName and
/// MarkerMeasuredDepth as core fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Marker {
	pub id: Uuid,
	/// Marker set this marker belongs to
	pub marker_set_id: Uuid,
	/// Well this marker belongs to (denormalized for queries)
	pub well_id: Uuid,

	// ===== Marker data (OSDU pattern) =====
	/// Marker/formation name
	pub name: String,
	/// Type classification
	pub marker_type: Option<String>,
	/// Measured depth
	pub measured_depth: f64,
	/// True vertical depth (optional)
	pub true_vertical_depth: Option<f64>,
	/// TVD sub-sea (optional)
	pub true_vertical_depth_ss: Option<f64>,

	// ===== Additional properties =====
	/// Thickness to base (if zone marker)
	pub thickness: Option<f64>,
	/// Quality/confidence indicator
	pub quality: Option<MarkerQuality>,
	/// Who picked this marker
	pub picked_by: Option<String>,
	/// Comments/notes
	pub comments: Option<String>,

	// ===== Original data preservation =====
	/// Row index in source file
	pub original_row_index: Option<i32>,
	/// JSON of original row values for audit
	pub original_values: Option<String>,

	// ===== Tracking =====
	pub created_by: Option<Uuid>,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
	pub version: i64,
	pub deleted_at: Option<DateTime<Utc>>,
}

/// Summary of a marker set for display in lists
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkerSetSummary {
	pub id: Uuid,
	pub well_id: Uuid,
	pub source_filename: String,
	pub name: Option<String>,
	pub interpretation_type: Option<InterpretationType>,
	pub marker_count: Option<i32>,
	pub min_depth: Option<f64>,
	pub max_depth: Option<f64>,
	pub depth_unit: String,
	pub imported_at: DateTime<Utc>,
}

/// Summary of an individual marker for display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkerSummary {
	pub id: Uuid,
	pub name: String,
	pub measured_depth: f64,
	pub true_vertical_depth: Option<f64>,
	pub marker_type: Option<String>,
	pub quality: Option<MarkerQuality>,
}

// =====================================================================
// SURFACE TYPES (Workspace-level 3D surfaces)
// =====================================================================

/// Column types detected in surface CSV files
///
/// Used for auto-detection of X, Y, Z coordinate columns and surface name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SurfaceColumnType {
	/// X coordinate (easting, longitude)
	X,
	/// Y coordinate (northing, latitude)
	Y,
	/// Z coordinate (depth, elevation, TWT)
	Z,
	/// Surface name identifier (for multi-surface files)
	SurfaceName,
	/// Quality/confidence indicator
	Quality,
	/// Generic numeric attribute (preserved but not interpreted)
	Attribute,
	/// Unknown column type
	Unknown,
}

impl SurfaceColumnType {
	/// Check if this column is required for basic surface ingestion
	pub fn is_required(&self) -> bool {
		matches!(self, SurfaceColumnType::X | SurfaceColumnType::Y | SurfaceColumnType::Z)
	}

	/// Get the canonical name for this column type
	pub fn canonical_name(&self) -> &'static str {
		match self {
			SurfaceColumnType::X => "x",
			SurfaceColumnType::Y => "y",
			SurfaceColumnType::Z => "z",
			SurfaceColumnType::SurfaceName => "surface_name",
			SurfaceColumnType::Quality => "quality",
			SurfaceColumnType::Attribute => "attribute",
			SurfaceColumnType::Unknown => "unknown",
		}
	}
}

impl std::fmt::Display for SurfaceColumnType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.canonical_name())
	}
}

impl std::str::FromStr for SurfaceColumnType {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_lowercase().replace(['-', ' '], "_").as_str() {
			"x" | "easting" | "east" | "e" | "longitude" | "lon" => Ok(SurfaceColumnType::X),
			"y" | "northing" | "north" | "n" | "latitude" | "lat" => Ok(SurfaceColumnType::Y),
			"z" | "depth" | "elevation" | "elev" | "twt" | "time" | "value" => Ok(SurfaceColumnType::Z),
			"surface" | "surface_name" | "surfacename" | "horizon" | "name" => {
				Ok(SurfaceColumnType::SurfaceName)
			}
			"quality" | "confidence" | "uncertainty" => Ok(SurfaceColumnType::Quality),
			"attribute" => Ok(SurfaceColumnType::Attribute),
			_ => Ok(SurfaceColumnType::Unknown),
		}
	}
}

/// Type classification for surfaces
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum SurfaceType {
	/// Seismic or geological horizon
	#[default]
	Horizon,
	/// Fault surface
	Fault,
	/// Unconformity surface
	Unconformity,
	/// Contact surface (e.g., fluid contact)
	Contact,
	/// Other surface type
	Other,
}

impl std::fmt::Display for SurfaceType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			SurfaceType::Horizon => write!(f, "horizon"),
			SurfaceType::Fault => write!(f, "fault"),
			SurfaceType::Unconformity => write!(f, "unconformity"),
			SurfaceType::Contact => write!(f, "contact"),
			SurfaceType::Other => write!(f, "other"),
		}
	}
}

impl std::str::FromStr for SurfaceType {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_lowercase().as_str() {
			"horizon" | "seismic" | "geological" => Ok(SurfaceType::Horizon),
			"fault" => Ok(SurfaceType::Fault),
			"unconformity" | "unconform" => Ok(SurfaceType::Unconformity),
			"contact" | "fluid_contact" | "fluidcontact" => Ok(SurfaceType::Contact),
			"other" => Ok(SurfaceType::Other),
			_ => Err(format!("Unknown surface type: {}", s)),
		}
	}
}

/// Direction of positive Z values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum ZPositiveDirection {
	/// Positive Z is downward (depth)
	#[default]
	Down,
	/// Positive Z is upward (elevation)
	Up,
}

impl std::fmt::Display for ZPositiveDirection {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			ZPositiveDirection::Down => write!(f, "down"),
			ZPositiveDirection::Up => write!(f, "up"),
		}
	}
}

impl std::str::FromStr for ZPositiveDirection {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_lowercase().as_str() {
			"down" | "depth" => Ok(ZPositiveDirection::Down),
			"up" | "elevation" => Ok(ZPositiveDirection::Up),
			_ => Err(format!("Unknown Z direction: {}", s)),
		}
	}
}

/// Spatial extent (bounding box) for a surface
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SpatialExtent {
	pub min_x: f64,
	pub max_x: f64,
	pub min_y: f64,
	pub max_y: f64,
	pub min_z: f64,
	pub max_z: f64,
}

impl SpatialExtent {
	/// Create a new empty extent (will be expanded when points are added)
	pub fn empty() -> Self {
		Self {
			min_x: f64::INFINITY,
			max_x: f64::NEG_INFINITY,
			min_y: f64::INFINITY,
			max_y: f64::NEG_INFINITY,
			min_z: f64::INFINITY,
			max_z: f64::NEG_INFINITY,
		}
	}

	/// Expand this extent to include a point
	pub fn expand(&mut self, x: f64, y: f64, z: f64) {
		self.min_x = self.min_x.min(x);
		self.max_x = self.max_x.max(x);
		self.min_y = self.min_y.min(y);
		self.max_y = self.max_y.max(y);
		self.min_z = self.min_z.min(z);
		self.max_z = self.max_z.max(z);
	}

	/// Check if this extent is valid (has been expanded with at least one point)
	pub fn is_valid(&self) -> bool {
		self.min_x <= self.max_x && self.min_y <= self.max_y && self.min_z <= self.max_z
	}
}

/// A surface represents a 3D geological or seismic surface
///
/// Following OSDU work-product-component--Surface patterns.
/// Surfaces are workspace-level entities (not tied to wells).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Surface {
	pub id: Uuid,
	pub workspace_id: Uuid,

	// ===== Source file info (immutable) =====
	/// Original filename
	pub source_filename: String,
	/// SHA-256 hash of original file
	pub source_file_hash: Option<String>,
	/// Blob hash of stored raw CSV file
	pub raw_file_blob_hash: Option<String>,
	/// Blob hash of stored Parquet point data
	pub data_blob_hash: Option<String>,

	// ===== Surface metadata =====
	/// Surface name
	pub name: String,
	/// Description
	pub description: Option<String>,
	/// Type of surface
	pub surface_type: Option<SurfaceType>,

	// ===== Spatial extent (bounding box) =====
	pub min_x: f64,
	pub max_x: f64,
	pub min_y: f64,
	pub max_y: f64,
	pub min_z: f64,
	pub max_z: f64,

	// ===== Coordinate reference =====
	/// Coordinate Reference System (e.g., "EPSG:32631")
	pub crs: Option<String>,
	/// XY coordinate unit (m or ft)
	pub xy_unit: String,
	/// Z coordinate unit (m or ft)
	pub z_unit: String,
	/// Direction of positive Z
	pub z_positive_direction: ZPositiveDirection,

	// ===== Point data info =====
	/// Total number of points
	pub point_count: i64,

	// ===== Optional grid info =====
	/// Whether this surface is a regular grid
	pub is_regular_grid: bool,
	/// Number of points in I direction (if regular grid)
	pub grid_ni: Option<i32>,
	/// Number of points in J direction (if regular grid)
	pub grid_nj: Option<i32>,
	/// Grid origin X (if regular grid)
	pub grid_origin_x: Option<f64>,
	/// Grid origin Y (if regular grid)
	pub grid_origin_y: Option<f64>,
	/// Grid spacing in X direction (if regular grid)
	pub grid_spacing_x: Option<f64>,
	/// Grid spacing in Y direction (if regular grid)
	pub grid_spacing_y: Option<f64>,

	// ===== Tracking =====
	pub imported_by: Option<Uuid>,
	pub imported_at: DateTime<Utc>,
	pub version: i64,
	pub deleted_at: Option<DateTime<Utc>>,
}

/// Summary of a surface for display in lists
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurfaceSummary {
	pub id: Uuid,
	pub workspace_id: Uuid,
	pub name: String,
	pub surface_type: Option<SurfaceType>,
	pub point_count: i64,
	pub extent: SpatialExtent,
	pub crs: Option<String>,
	pub xy_unit: String,
	pub z_unit: String,
	pub is_regular_grid: bool,
	pub imported_at: DateTime<Utc>,
}
