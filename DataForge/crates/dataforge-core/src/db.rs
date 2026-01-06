//! SQLite database schema and operations

use rusqlite::{Connection, Result as SqliteResult};
use tracing::info;

/// SQL schema for DataForge database
pub const SCHEMA: &str = r#"
-- ============================================================
-- ACCOUNTS (top-level authentication identity)
-- ============================================================
-- Based on ColaNode's account-workspace-user model:
-- - Account: Top-level identity that can belong to multiple workspaces
-- - Workspace: Container (project) that hosts multiple users
-- - User: Workspace-scoped identity created when account joins workspace

CREATE TABLE IF NOT EXISTS accounts (
    id TEXT PRIMARY KEY,
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT,
    name TEXT NOT NULL,
    avatar_url TEXT,
    status INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- status values:
-- 0 = unverified (pending email verification or admin approval)
-- 1 = verified (active)
-- 2 = suspended

CREATE INDEX IF NOT EXISTS idx_accounts_email ON accounts(email);
CREATE INDEX IF NOT EXISTS idx_accounts_status ON accounts(status);

-- ============================================================
-- WORKSPACES (containers for wells and users)
-- ============================================================

CREATE TABLE IF NOT EXISTS workspaces (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    avatar_url TEXT,
    owner_account_id TEXT NOT NULL REFERENCES accounts(id),
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_workspaces_owner ON workspaces(owner_account_id);

-- ============================================================
-- WORKSPACE MEMBERS (per-workspace user identity)
-- ============================================================
-- When an account joins a workspace, a distinct user identity is created
-- This allows different display names, roles, and permissions per workspace

CREATE TABLE IF NOT EXISTS workspace_members (
    id TEXT PRIMARY KEY,
    workspace_id TEXT NOT NULL REFERENCES workspaces(id) ON DELETE CASCADE,
    account_id TEXT NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    display_name TEXT,
    role TEXT NOT NULL DEFAULT 'member',
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    UNIQUE(workspace_id, account_id)
);

-- role values: 'owner', 'admin', 'member', 'viewer'

CREATE INDEX IF NOT EXISTS idx_workspace_members_workspace ON workspace_members(workspace_id);
CREATE INDEX IF NOT EXISTS idx_workspace_members_account ON workspace_members(account_id);

-- ============================================================
-- SESSIONS (for local authentication state)
-- ============================================================

CREATE TABLE IF NOT EXISTS sessions (
    id TEXT PRIMARY KEY,
    account_id TEXT NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    token_hash TEXT NOT NULL,
    device_name TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    expires_at TEXT NOT NULL,
    last_active_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_sessions_account ON sessions(account_id);
CREATE INDEX IF NOT EXISTS idx_sessions_token ON sessions(token_hash);

-- Legacy alias: users view maps to workspace_members for backwards compatibility
CREATE VIEW IF NOT EXISTS users AS
SELECT
    wm.id,
    a.email,
    COALESCE(wm.display_name, a.name) as name,
    wm.workspace_id as team_id,
    wm.created_at,
    wm.updated_at
FROM workspace_members wm
JOIN accounts a ON wm.account_id = a.id;

-- Legacy alias: teams view maps to workspaces
CREATE VIEW IF NOT EXISTS teams AS
SELECT
    id,
    name,
    owner_account_id as owner_id,
    created_at,
    updated_at
FROM workspaces;

-- Legacy alias: team_members view
CREATE VIEW IF NOT EXISTS team_members AS
SELECT
    id,
    workspace_id as team_id,
    id as user_id,
    role,
    created_at
FROM workspace_members;

-- ============================================================
-- WELLS
-- ============================================================
-- Wells belong directly to workspaces; created_by references workspace_members
-- Each well has a canonical depth grid for consistent curve sampling

CREATE TABLE IF NOT EXISTS wells (
    id TEXT PRIMARY KEY,
    workspace_id TEXT NOT NULL REFERENCES workspaces(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    uwi TEXT,
    field TEXT,
    company TEXT,
    location TEXT,
    x REAL,
    y REAL,
    -- Depth grid configuration (canonical sampling for this well)
    depth_unit TEXT NOT NULL DEFAULT 'ft',
    depth_step REAL NOT NULL DEFAULT 0.5,
    depth_origin REAL NOT NULL DEFAULT 0.0,
    -- Computed depth range (updated when curves added)
    min_depth REAL,
    max_depth REAL,
    created_by TEXT NOT NULL REFERENCES workspace_members(id),
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    version INTEGER NOT NULL DEFAULT 1,
    deleted_at TEXT
);

CREATE INDEX IF NOT EXISTS idx_wells_workspace_id ON wells(workspace_id);
CREATE INDEX IF NOT EXISTS idx_wells_version ON wells(version);

-- ============================================================
-- REFERENCE DATA: LOG TYPES
-- ============================================================
-- Classification of log data (Raw, Processed, Interpreted, etc.)

CREATE TABLE IF NOT EXISTS log_types (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT
);

-- ============================================================
-- REFERENCE DATA: ACQUISITION TYPES
-- ============================================================
-- How the log was acquired (Wireline, LWD, MWD, etc.)

CREATE TABLE IF NOT EXISTS acquisition_types (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT
);

-- ============================================================
-- REFERENCE DATA: MEASUREMENT TYPES (OSDU-inspired Unit Service)
-- ============================================================
-- Classification of physical quantities (length, pressure, etc.)

CREATE TABLE IF NOT EXISTS measurement_types (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    base_unit_id TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- ============================================================
-- REFERENCE DATA: UNITS (OSDU-inspired Unit Service)
-- ============================================================
-- Unit definitions with conversion factors to base unit

CREATE TABLE IF NOT EXISTS units (
    id TEXT PRIMARY KEY,
    measurement_type_id TEXT NOT NULL REFERENCES measurement_types(id),
    symbol TEXT NOT NULL,
    name TEXT NOT NULL,
    to_base_factor REAL NOT NULL DEFAULT 1.0,
    to_base_offset REAL NOT NULL DEFAULT 0.0,
    is_base INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    UNIQUE(measurement_type_id, symbol)
);

CREATE INDEX IF NOT EXISTS idx_units_measurement_type ON units(measurement_type_id);
CREATE INDEX IF NOT EXISTS idx_units_symbol ON units(symbol);

-- ============================================================
-- REFERENCE DATA: CURVE PROPERTIES (PWLS-style dictionary)
-- ============================================================
-- Canonical curve properties that mnemonics map to
-- Enhanced with measurement_type_id for unit validation

CREATE TABLE IF NOT EXISTS curve_properties (
    id TEXT PRIMARY KEY,
    canonical_name TEXT NOT NULL UNIQUE,
    property_class TEXT,
    measurement_type_id TEXT REFERENCES measurement_types(id),
    typical_unit TEXT,
    description TEXT,
    display_color TEXT,
    log_scale INTEGER NOT NULL DEFAULT 0,
    min_valid_value REAL,
    max_valid_value REAL,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- ============================================================
-- REFERENCE DATA: CURVE MNEMONICS
-- ============================================================
-- Maps vendor mnemonics to canonical properties

CREATE TABLE IF NOT EXISTS curve_mnemonics (
    mnemonic TEXT PRIMARY KEY,
    property_id TEXT REFERENCES curve_properties(id),
    priority INTEGER NOT NULL DEFAULT 0,
    vendor TEXT,
    notes TEXT
);

CREATE INDEX IF NOT EXISTS idx_curve_mnemonics_property ON curve_mnemonics(property_id);

-- ============================================================
-- LOG RUNS (one per LAS upload event)
-- ============================================================
-- Each LAS file import creates a log_run record
-- Preserves original file metadata and provenance

CREATE TABLE IF NOT EXISTS log_runs (
    id TEXT PRIMARY KEY,
    well_id TEXT NOT NULL REFERENCES wells(id) ON DELETE CASCADE,
    workspace_id TEXT NOT NULL REFERENCES workspaces(id) ON DELETE CASCADE,
    -- Source file info (immutable)
    source_filename TEXT NOT NULL,
    source_file_hash TEXT,
    raw_file_blob_hash TEXT,
    -- Classification
    log_type_id TEXT REFERENCES log_types(id),
    acquisition_type_id TEXT REFERENCES acquisition_types(id),
    run_number INTEGER,
    -- Service info
    service_company TEXT,
    tool_name TEXT,
    logging_date TEXT,
    -- Original depth info (before resampling)
    original_top_depth REAL,
    original_bottom_depth REAL,
    original_step REAL,
    original_depth_unit TEXT,
    original_null_value REAL,
    las_version TEXT,
    -- Import tracking
    imported_by TEXT REFERENCES workspace_members(id),
    imported_at TEXT NOT NULL DEFAULT (datetime('now')),
    -- Versioning
    version INTEGER NOT NULL DEFAULT 1,
    deleted_at TEXT
);

CREATE INDEX IF NOT EXISTS idx_log_runs_well ON log_runs(well_id);
CREATE INDEX IF NOT EXISTS idx_log_runs_workspace ON log_runs(workspace_id);
CREATE INDEX IF NOT EXISTS idx_log_runs_source_hash ON log_runs(source_file_hash);

-- ============================================================
-- CURVES (enhanced for dual native/gridded storage)
-- ============================================================
-- Each curve belongs to a log_run (LAS upload) and a well
-- Stores both native (original sampling) and gridded (resampled) data

CREATE TABLE IF NOT EXISTS curves (
    id TEXT PRIMARY KEY,
    log_run_id TEXT NOT NULL REFERENCES log_runs(id) ON DELETE CASCADE,
    well_id TEXT NOT NULL REFERENCES wells(id) ON DELETE CASCADE,
    -- Mnemonic and property mapping
    mnemonic TEXT NOT NULL,
    property_id TEXT REFERENCES curve_properties(id),
    unit TEXT,
    unit_id TEXT REFERENCES units(id),
    description TEXT,
    -- Native range (original sampling, before resampling)
    native_top_depth REAL,
    native_bottom_depth REAL,
    native_step REAL,
    native_sample_count INTEGER,
    -- Gridded range (after resampling to well grid)
    gridded_top_depth REAL,
    gridded_bottom_depth REAL,
    gridded_sample_count INTEGER,
    -- Resampling policy (how this curve was resampled)
    resample_method TEXT,
    was_unit_converted INTEGER NOT NULL DEFAULT 0,
    source_depth_unit TEXT,
    -- Statistics
    min_value REAL,
    max_value REAL,
    mean_value REAL,
    null_count INTEGER,
    -- Quality/Validity fields (OSDU-inspired)
    null_value REAL,
    quality_flag TEXT DEFAULT 'raw',
    acquisition_date TEXT,
    service_company TEXT,
    -- Storage references (content-addressed)
    native_parquet_hash TEXT,
    gridded_parquet_hash TEXT,
    -- Tracking
    created_by TEXT REFERENCES workspace_members(id),
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    version INTEGER NOT NULL DEFAULT 1,
    deleted_at TEXT,
    UNIQUE(log_run_id, mnemonic)
);

-- quality_flag values: 'raw', 'edited', 'computed', 'spliced', 'reprocessed'

CREATE INDEX IF NOT EXISTS idx_curves_well_id ON curves(well_id);
CREATE INDEX IF NOT EXISTS idx_curves_log_run ON curves(log_run_id);
CREATE INDEX IF NOT EXISTS idx_curves_property ON curves(property_id);
CREATE INDEX IF NOT EXISTS idx_curves_mnemonic ON curves(mnemonic);
CREATE INDEX IF NOT EXISTS idx_curves_native_hash ON curves(native_parquet_hash);
CREATE INDEX IF NOT EXISTS idx_curves_gridded_hash ON curves(gridded_parquet_hash);
-- Note: idx_curves_quality is created in migrations after quality_flag column exists

-- ============================================================
-- CURVE VERSIONS (for edit tracking)
-- ============================================================
-- Tracks versions when curves are edited/reprocessed

CREATE TABLE IF NOT EXISTS curve_versions (
    id TEXT PRIMARY KEY,
    curve_id TEXT NOT NULL REFERENCES curves(id) ON DELETE CASCADE,
    version INTEGER NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    created_by TEXT REFERENCES workspace_members(id),
    reason TEXT,
    native_parquet_hash TEXT,
    gridded_parquet_hash TEXT,
    min_value REAL,
    max_value REAL,
    mean_value REAL,
    null_count INTEGER,
    UNIQUE(curve_id, version)
);

-- reason values: 'splice', 'environmental_correction', 'reprocess', 'edit', 'merge'

CREATE INDEX IF NOT EXISTS idx_curve_versions_curve ON curve_versions(curve_id);

-- Legacy curve_metadata table (kept for backwards compatibility during migration)
CREATE TABLE IF NOT EXISTS curve_metadata (
    id TEXT PRIMARY KEY,
    curve_mnemonic TEXT UNIQUE NOT NULL,
    main_curve_type TEXT NOT NULL,
    subcurve_name TEXT,
    display_name TEXT NOT NULL,
    description TEXT,
    units TEXT NOT NULL,
    vendor_variations TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- ============================================================
-- UNIFIED VIEW REGISTRY
-- ============================================================
-- Tracks materialized unified views per well (chunked Parquet files)

CREATE TABLE IF NOT EXISTS unified_views (
    id TEXT PRIMARY KEY,
    well_id TEXT NOT NULL REFERENCES wells(id) ON DELETE CASCADE,
    chunk_index INTEGER NOT NULL,
    start_depth REAL NOT NULL,
    end_depth REAL NOT NULL,
    row_count INTEGER NOT NULL,
    column_count INTEGER NOT NULL,
    columns_json TEXT NOT NULL,
    parquet_hash TEXT NOT NULL,
    generated_at TEXT NOT NULL DEFAULT (datetime('now')),
    UNIQUE(well_id, chunk_index)
);

CREATE INDEX IF NOT EXISTS idx_unified_views_well ON unified_views(well_id);

-- ============================================================
-- SURVEY RUNS (one per trajectory CSV upload event)
-- ============================================================
-- Each trajectory CSV import creates a survey_run record
-- Parallel structure to log_runs for curve data

CREATE TABLE IF NOT EXISTS survey_runs (
    id TEXT PRIMARY KEY,
    well_id TEXT NOT NULL REFERENCES wells(id) ON DELETE CASCADE,
    workspace_id TEXT NOT NULL REFERENCES workspaces(id) ON DELETE CASCADE,
    -- Source file info (immutable)
    source_filename TEXT NOT NULL,
    source_file_hash TEXT,
    raw_file_blob_hash TEXT,
    -- Survey metadata
    survey_type TEXT,  -- 'definitive', 'preliminary', 'mwd', 'gyro'
    survey_company TEXT,
    survey_date TEXT,
    survey_run_number INTEGER,
    -- Reference info
    magnetic_declination REAL,
    grid_convergence REAL,
    azimuth_reference TEXT,  -- 'true_north', 'grid_north', 'magnetic_north'
    vertical_reference TEXT,  -- 'kb', 'rt', 'msl', 'ground_level'
    vertical_reference_elevation REAL,
    source_crs TEXT,  -- EPSG code or WKT
    target_crs TEXT,  -- For future CRS transformation
    -- Calculation metadata
    calculation_method TEXT DEFAULT 'minimum_curvature',
    tie_in_md REAL,
    tie_in_tvd REAL,
    tie_in_ns REAL,
    tie_in_ew REAL,
    -- Depth range
    top_md REAL,
    bottom_md REAL,
    station_count INTEGER,
    md_unit TEXT NOT NULL DEFAULT 'ft',
    -- Import tracking
    imported_by TEXT REFERENCES workspace_members(id),
    imported_at TEXT NOT NULL DEFAULT (datetime('now')),
    -- Versioning
    version INTEGER NOT NULL DEFAULT 1,
    deleted_at TEXT
);

CREATE INDEX IF NOT EXISTS idx_survey_runs_well ON survey_runs(well_id);
CREATE INDEX IF NOT EXISTS idx_survey_runs_workspace ON survey_runs(workspace_id);

-- ============================================================
-- TRAJECTORY COLUMNS (individual survey measurements)
-- ============================================================
-- Each column of trajectory data (MD, INC, AZI, TVD, etc.)
-- Parallel structure to curves table

CREATE TABLE IF NOT EXISTS trajectory_columns (
    id TEXT PRIMARY KEY,
    survey_run_id TEXT NOT NULL REFERENCES survey_runs(id) ON DELETE CASCADE,
    well_id TEXT NOT NULL REFERENCES wells(id) ON DELETE CASCADE,
    -- Column identity
    column_type TEXT NOT NULL,  -- 'md', 'inclination', 'azimuth', 'tvd', 'ns', 'ew', 'dls', 'dx', 'dy', 'dz'
    column_name TEXT NOT NULL,  -- Original column name from CSV
    unit TEXT,
    unit_id TEXT REFERENCES units(id),
    description TEXT,
    -- Data characteristics
    is_input INTEGER NOT NULL DEFAULT 0,  -- true for MD, INC, AZI (input stations)
    is_calculated INTEGER NOT NULL DEFAULT 0,  -- true for TVD, NS, EW, DLS (output)
    -- Statistics
    min_value REAL,
    max_value REAL,
    mean_value REAL,
    null_count INTEGER,
    sample_count INTEGER,
    -- Storage reference (content-addressed)
    parquet_hash TEXT,
    -- Tracking
    created_by TEXT REFERENCES workspace_members(id),
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    version INTEGER NOT NULL DEFAULT 1,
    deleted_at TEXT,
    UNIQUE(survey_run_id, column_type)
);

CREATE INDEX IF NOT EXISTS idx_trajectory_columns_survey ON trajectory_columns(survey_run_id);
CREATE INDEX IF NOT EXISTS idx_trajectory_columns_well ON trajectory_columns(well_id);
CREATE INDEX IF NOT EXISTS idx_trajectory_columns_type ON trajectory_columns(column_type);

-- ============================================================
-- MARKER SETS (one per marker CSV upload event per well)
-- ============================================================
-- Each marker CSV import creates marker_set record(s) - one per well
-- Follows OSDU WellboreMarkerSet patterns

CREATE TABLE IF NOT EXISTS marker_sets (
    id TEXT PRIMARY KEY,
    well_id TEXT REFERENCES wells(id) ON DELETE SET NULL,  -- Nullable for unmapped marker sets
    workspace_id TEXT NOT NULL REFERENCES workspaces(id) ON DELETE CASCADE,
    -- Source file info (immutable)
    source_filename TEXT NOT NULL,
    source_file_hash TEXT,
    raw_file_blob_hash TEXT,
    -- Marker set metadata
    name TEXT,                          -- "Formation Tops", "Sequence Boundaries"
    interpretation_type TEXT,           -- 'formation', 'sequence', 'zone', 'horizon'
    interpreter TEXT,
    reference_source TEXT,
    -- Depth info
    depth_unit TEXT NOT NULL DEFAULT 'ft',
    depth_reference TEXT,               -- 'kb', 'rt', 'msl', 'ground_level'
    min_depth REAL,
    max_depth REAL,
    marker_count INTEGER,
    -- Import tracking
    imported_by TEXT REFERENCES workspace_members(id),
    imported_at TEXT NOT NULL DEFAULT (datetime('now')),
    -- Versioning
    version INTEGER NOT NULL DEFAULT 1,
    deleted_at TEXT
);

CREATE INDEX IF NOT EXISTS idx_marker_sets_well ON marker_sets(well_id);
CREATE INDEX IF NOT EXISTS idx_marker_sets_workspace ON marker_sets(workspace_id);

-- ============================================================
-- MARKERS (individual marker/well top entries)
-- ============================================================
-- Each marker in the CSV creates a marker record
-- Follows OSDU WellboreMarkerSet patterns with MarkerName, MarkerMeasuredDepth

CREATE TABLE IF NOT EXISTS markers (
    id TEXT PRIMARY KEY,
    marker_set_id TEXT NOT NULL REFERENCES marker_sets(id) ON DELETE CASCADE,
    well_id TEXT REFERENCES wells(id) ON DELETE SET NULL,  -- Nullable for unmapped markers
    well_name TEXT,                      -- Original well name from CSV (for unmapped markers)
    -- Marker data (OSDU WellboreMarkerSet pattern)
    name TEXT NOT NULL,                  -- Marker/formation name (e.g., "Breda Formation", "MFS10")
    marker_type TEXT,                    -- 'formation', 'sequence_boundary', 'mfs', 'ts', 'horizon', 'zone_top', 'zone_base'
    measured_depth REAL NOT NULL,
    true_vertical_depth REAL,
    true_vertical_depth_ss REAL,
    -- Additional properties
    thickness REAL,
    quality TEXT,                        -- 'confirmed', 'uncertain', 'projected', 'absent'
    picked_by TEXT,
    comments TEXT,
    -- Original data preservation
    original_row_index INTEGER,
    original_values TEXT,                -- JSON of original row values for audit
    -- Tracking
    created_by TEXT REFERENCES workspace_members(id),
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    version INTEGER NOT NULL DEFAULT 1,
    deleted_at TEXT,
    UNIQUE(marker_set_id, name, measured_depth)
);

CREATE INDEX IF NOT EXISTS idx_markers_marker_set ON markers(marker_set_id);
CREATE INDEX IF NOT EXISTS idx_markers_well ON markers(well_id) WHERE well_id IS NOT NULL;
-- Note: idx_markers_well_name is created in migrations after well_name column exists
CREATE INDEX IF NOT EXISTS idx_markers_name ON markers(name);
CREATE INDEX IF NOT EXISTS idx_markers_depth ON markers(measured_depth);

-- ============================================================
-- SURFACES (workspace-level 3D surfaces)
-- ============================================================
-- Surfaces are workspace-level entities (not tied to wells)
-- Represents geological surfaces, seismic horizons, faults, etc.
-- Point data stored in Parquet, metadata in SQLite
-- Follows OSDU work-product-component--Surface pattern

CREATE TABLE IF NOT EXISTS surfaces (
    id TEXT PRIMARY KEY,
    workspace_id TEXT NOT NULL REFERENCES workspaces(id) ON DELETE CASCADE,
    -- Source file info (immutable)
    source_filename TEXT NOT NULL,
    source_file_hash TEXT,
    raw_file_blob_hash TEXT,           -- Original CSV preserved in blob store
    data_blob_hash TEXT,               -- Parquet point data in blob store
    -- Surface metadata (OSDU Surface pattern)
    name TEXT NOT NULL,
    description TEXT,
    surface_type TEXT,                 -- 'horizon', 'fault', 'unconformity', 'contact', 'other'
    -- Spatial extent (bounding box)
    min_x REAL NOT NULL,
    max_x REAL NOT NULL,
    min_y REAL NOT NULL,
    max_y REAL NOT NULL,
    min_z REAL NOT NULL,
    max_z REAL NOT NULL,
    -- Coordinate reference
    crs TEXT,                          -- EPSG code, e.g., "EPSG:32631"
    xy_unit TEXT NOT NULL DEFAULT 'm', -- 'm', 'ft'
    z_unit TEXT NOT NULL DEFAULT 'm',  -- 'm', 'ft'
    z_positive_direction TEXT DEFAULT 'down', -- 'down' (depth) or 'up' (elevation)
    -- Point data info
    point_count INTEGER NOT NULL,
    -- Optional grid info (if regular grid detected)
    is_regular_grid INTEGER DEFAULT 0,
    grid_ni INTEGER,                   -- Number of points in I direction
    grid_nj INTEGER,                   -- Number of points in J direction
    grid_origin_x REAL,
    grid_origin_y REAL,
    grid_spacing_x REAL,
    grid_spacing_y REAL,
    -- Import tracking
    imported_by TEXT REFERENCES workspace_members(id),
    imported_at TEXT NOT NULL DEFAULT (datetime('now')),
    -- Versioning
    version INTEGER NOT NULL DEFAULT 1,
    deleted_at TEXT,
    UNIQUE(workspace_id, name, deleted_at)
);

CREATE INDEX IF NOT EXISTS idx_surfaces_workspace ON surfaces(workspace_id);
CREATE INDEX IF NOT EXISTS idx_surfaces_type ON surfaces(surface_type);
CREATE INDEX IF NOT EXISTS idx_surfaces_name ON surfaces(name);

-- ============================================================
-- CHECKSHOT RUNS (one per checkshot CSV upload event)
-- ============================================================
-- Each checkshot CSV import creates a checkshot_run record
-- Follows OSDU work-product-component--Checkshot pattern
-- Checkshots establish time-depth relationships for velocity calibration

CREATE TABLE IF NOT EXISTS checkshot_runs (
    id TEXT PRIMARY KEY,
    well_id TEXT NOT NULL REFERENCES wells(id) ON DELETE CASCADE,
    workspace_id TEXT NOT NULL REFERENCES workspaces(id) ON DELETE CASCADE,
    -- Source file info (immutable)
    source_filename TEXT NOT NULL,
    source_file_hash TEXT,
    raw_file_blob_hash TEXT,
    -- Checkshot metadata (OSDU Checkshot pattern)
    name TEXT,
    description TEXT,
    survey_type TEXT,              -- 'checkshot', 'vsp', 'sonic_log', 'other'
    survey_company TEXT,
    survey_date TEXT,
    -- Depth/time ranges
    min_md REAL,
    max_md REAL,
    min_tvd REAL,
    max_tvd REAL,
    min_twt REAL,
    max_twt REAL,
    station_count INTEGER,
    -- Units
    depth_unit TEXT NOT NULL DEFAULT 'ft',  -- 'ft', 'm'
    time_unit TEXT NOT NULL DEFAULT 's',    -- 's', 'ms'
    -- Reference info
    datum TEXT,                    -- 'kb', 'msl', 'ground_level'
    datum_elevation REAL,
    -- Import tracking
    imported_by TEXT REFERENCES workspace_members(id),
    imported_at TEXT NOT NULL DEFAULT (datetime('now')),
    -- Versioning
    version INTEGER NOT NULL DEFAULT 1,
    deleted_at TEXT
);

CREATE INDEX IF NOT EXISTS idx_checkshot_runs_well ON checkshot_runs(well_id);
CREATE INDEX IF NOT EXISTS idx_checkshot_runs_workspace ON checkshot_runs(workspace_id);

-- ============================================================
-- CHECKSHOT COLUMNS (MD, TVD, TWT, velocity per run)
-- ============================================================
-- Each column of checkshot data stored separately
-- Follows trajectory_columns pattern for consistency

CREATE TABLE IF NOT EXISTS checkshot_columns (
    id TEXT PRIMARY KEY,
    checkshot_run_id TEXT NOT NULL REFERENCES checkshot_runs(id) ON DELETE CASCADE,
    well_id TEXT NOT NULL REFERENCES wells(id) ON DELETE CASCADE,
    -- Column identity
    column_type TEXT NOT NULL,     -- 'md', 'tvd', 'twt', 'velocity', 'quality'
    column_name TEXT NOT NULL,     -- Original column name from CSV
    unit TEXT,
    -- Data characteristics
    is_input INTEGER NOT NULL DEFAULT 1,      -- true for MD, TVD, TWT
    is_calculated INTEGER NOT NULL DEFAULT 0, -- true for calculated velocity
    -- Statistics
    min_value REAL,
    max_value REAL,
    mean_value REAL,
    null_count INTEGER,
    sample_count INTEGER,
    -- Storage reference (content-addressed)
    parquet_hash TEXT,
    -- Tracking
    created_by TEXT REFERENCES workspace_members(id),
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    version INTEGER NOT NULL DEFAULT 1,
    deleted_at TEXT,
    UNIQUE(checkshot_run_id, column_type)
);

CREATE INDEX IF NOT EXISTS idx_checkshot_columns_run ON checkshot_columns(checkshot_run_id);
CREATE INDEX IF NOT EXISTS idx_checkshot_columns_well ON checkshot_columns(well_id);
CREATE INDEX IF NOT EXISTS idx_checkshot_columns_type ON checkshot_columns(column_type);

-- ============================================================
-- BLOB REGISTRY
-- ============================================================

CREATE TABLE IF NOT EXISTS blob_registry (
    hash TEXT PRIMARY KEY,
    size_bytes INTEGER NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    last_accessed_at TEXT,
    synced_to_server INTEGER NOT NULL DEFAULT 0,
    synced_at TEXT
);

-- ============================================================
-- SYNC STATE (Enhanced for ColaNode-style sync)
-- ============================================================
-- Tracks sync state per workspace (each workspace can have its own server)

CREATE TABLE IF NOT EXISTS sync_state (
    workspace_id TEXT PRIMARY KEY REFERENCES workspaces(id) ON DELETE CASCADE,
    server_url TEXT NOT NULL,
    last_sync_version INTEGER NOT NULL DEFAULT 0,
    last_sync_at TEXT,
    last_push_at TEXT,
    last_pull_at TEXT,
    sync_status TEXT NOT NULL DEFAULT 'idle',
    last_error TEXT,
    conflict_strategy TEXT NOT NULL DEFAULT 'manual',
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- sync_status values: 'idle', 'syncing', 'error', 'offline'
-- conflict_strategy values: 'manual', 'last_write_wins', 'local_wins', 'remote_wins'

CREATE INDEX IF NOT EXISTS idx_sync_state_status ON sync_state(sync_status);

-- ============================================================
-- SYNC QUEUE (Enhanced for offline-first)
-- ============================================================
-- Queues local changes for sync to server

CREATE TABLE IF NOT EXISTS sync_queue (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    workspace_id TEXT NOT NULL REFERENCES workspaces(id) ON DELETE CASCADE,
    entity_type TEXT NOT NULL,
    entity_id TEXT NOT NULL,
    action TEXT NOT NULL,
    version INTEGER NOT NULL DEFAULT 1,
    payload TEXT,
    blob_hashes TEXT,
    priority INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    attempts INTEGER NOT NULL DEFAULT 0,
    last_attempt_at TEXT,
    last_error TEXT,
    synced_at TEXT,
    UNIQUE(workspace_id, entity_type, entity_id)
);

-- action values: 'create', 'update', 'delete'
-- priority: higher = sync first (0 = normal, 1 = high, -1 = low)

CREATE INDEX IF NOT EXISTS idx_sync_queue_workspace ON sync_queue(workspace_id);
CREATE INDEX IF NOT EXISTS idx_sync_queue_status ON sync_queue(synced_at);
CREATE INDEX IF NOT EXISTS idx_sync_queue_priority ON sync_queue(priority DESC, created_at ASC);
CREATE INDEX IF NOT EXISTS idx_sync_queue_entity ON sync_queue(entity_type, entity_id);

-- ============================================================
-- SYNC CONFLICTS (For conflict resolution)
-- ============================================================
-- Records conflicts detected during pull for user review

CREATE TABLE IF NOT EXISTS sync_conflicts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    workspace_id TEXT NOT NULL REFERENCES workspaces(id) ON DELETE CASCADE,
    entity_type TEXT NOT NULL,
    entity_id TEXT NOT NULL,
    local_version INTEGER NOT NULL,
    local_data TEXT NOT NULL,
    remote_version INTEGER NOT NULL,
    remote_data TEXT NOT NULL,
    resolution TEXT,
    resolved_by TEXT REFERENCES accounts(id),
    resolved_at TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- resolution values: 'pending', 'local', 'remote', 'merged'

CREATE INDEX IF NOT EXISTS idx_sync_conflicts_workspace ON sync_conflicts(workspace_id);
CREATE INDEX IF NOT EXISTS idx_sync_conflicts_pending ON sync_conflicts(resolution) WHERE resolution = 'pending';

-- ============================================================
-- WORKSPACE SYNC CONFIGURATION
-- ============================================================
-- Add sync-related columns to workspaces (separate migration for existing DBs)

-- Note: These columns are added via ALTER TABLE in migrations for existing databases
-- sync_server_url TEXT - Custom server URL for this workspace
-- sync_enabled INTEGER NOT NULL DEFAULT 1 - Whether sync is enabled
-- sync_interval_seconds INTEGER NOT NULL DEFAULT 300 - Auto-sync interval
"#;

/// Initialize the database with the schema
pub fn init_db(conn: &Connection) -> SqliteResult<()> {
	info!("Initializing DataForge database schema");
	conn.execute_batch(SCHEMA)?;

	// Run migrations for existing databases
	run_migrations(conn)?;

	// Seed reference data (log types, acquisition types, curve properties, mnemonics)
	insert_default_reference_data(conn)?;

	info!("Database schema initialized successfully");
	Ok(())
}

/// Run database migrations for schema changes
fn run_migrations(conn: &Connection) -> SqliteResult<()> {
	// Migration 1: Add conflict_strategy column to sync_state if missing
	// Added in Phase 4 of sync implementation
	let has_conflict_strategy: bool = conn
		.prepare("SELECT COUNT(*) FROM pragma_table_info('sync_state') WHERE name = 'conflict_strategy'")?
		.query_row([], |row| row.get::<_, i64>(0))
		.map(|count| count > 0)
		.unwrap_or(false);

	if !has_conflict_strategy {
		info!("Running migration: adding conflict_strategy column to sync_state");
		conn.execute(
			"ALTER TABLE sync_state ADD COLUMN conflict_strategy TEXT NOT NULL DEFAULT 'manual'",
			[],
		)?;
	}

	// Migration 2: Check if curves table has old schema (curve_name column) and recreate it
	// This handles the schema change from single blob_hash to native/gridded dual storage
	let has_old_curves_schema: bool = conn
		.prepare("SELECT COUNT(*) FROM pragma_table_info('curves') WHERE name = 'curve_name'")?
		.query_row([], |row| row.get::<_, i64>(0))
		.map(|count| count > 0)
		.unwrap_or(false);

	if has_old_curves_schema {
		info!("Running migration: recreating curves table with new schema (native/gridded storage + quality fields)");
		// Rename old table, create new one, and drop old
		// Note: This will lose existing curve data, but the curves can be re-ingested from LAS files
		conn.execute("ALTER TABLE curves RENAME TO curves_old", [])?;
		conn.execute_batch(
			r#"
			CREATE TABLE curves (
				id TEXT PRIMARY KEY,
				log_run_id TEXT NOT NULL REFERENCES log_runs(id) ON DELETE CASCADE,
				well_id TEXT NOT NULL REFERENCES wells(id) ON DELETE CASCADE,
				mnemonic TEXT NOT NULL,
				property_id TEXT REFERENCES curve_properties(id),
				unit TEXT,
				unit_id TEXT REFERENCES units(id),
				description TEXT,
				native_top_depth REAL,
				native_bottom_depth REAL,
				native_step REAL,
				native_sample_count INTEGER,
				gridded_top_depth REAL,
				gridded_bottom_depth REAL,
				gridded_sample_count INTEGER,
				resample_method TEXT,
				was_unit_converted INTEGER NOT NULL DEFAULT 0,
				source_depth_unit TEXT,
				min_value REAL,
				max_value REAL,
				mean_value REAL,
				null_count INTEGER,
				null_value REAL,
				quality_flag TEXT DEFAULT 'raw',
				acquisition_date TEXT,
				service_company TEXT,
				native_parquet_hash TEXT,
				gridded_parquet_hash TEXT,
				created_by TEXT REFERENCES workspace_members(id),
				created_at TEXT NOT NULL DEFAULT (datetime('now')),
				updated_at TEXT NOT NULL DEFAULT (datetime('now')),
				version INTEGER NOT NULL DEFAULT 1,
				deleted_at TEXT,
				UNIQUE(log_run_id, mnemonic)
			);
			CREATE INDEX IF NOT EXISTS idx_curves_well_id ON curves(well_id);
			CREATE INDEX IF NOT EXISTS idx_curves_log_run ON curves(log_run_id);
			CREATE INDEX IF NOT EXISTS idx_curves_property ON curves(property_id);
			CREATE INDEX IF NOT EXISTS idx_curves_mnemonic ON curves(mnemonic);
			CREATE INDEX IF NOT EXISTS idx_curves_native_hash ON curves(native_parquet_hash);
			CREATE INDEX IF NOT EXISTS idx_curves_gridded_hash ON curves(gridded_parquet_hash);
			CREATE INDEX IF NOT EXISTS idx_curves_quality ON curves(quality_flag);
			DROP TABLE curves_old;
			"#,
		)?;
	}

	// Migration 2b: Add quality fields to existing curves table if missing
	// This handles databases that have the new schema but not the quality fields
	let has_quality_flag: bool = conn
		.prepare("SELECT COUNT(*) FROM pragma_table_info('curves') WHERE name = 'quality_flag'")?
		.query_row([], |row| row.get::<_, i64>(0))
		.map(|count| count > 0)
		.unwrap_or(false);

	if !has_quality_flag {
		info!("Running migration: adding quality fields to curves table");
		conn.execute("ALTER TABLE curves ADD COLUMN null_value REAL", [])?;
		conn.execute("ALTER TABLE curves ADD COLUMN quality_flag TEXT DEFAULT 'raw'", [])?;
		conn.execute("ALTER TABLE curves ADD COLUMN acquisition_date TEXT", [])?;
		conn.execute("ALTER TABLE curves ADD COLUMN service_company TEXT", [])?;
		conn.execute("ALTER TABLE curves ADD COLUMN unit_id TEXT REFERENCES units(id)", [])?;
		// Create the quality index after column exists
		conn.execute("CREATE INDEX IF NOT EXISTS idx_curves_quality ON curves(quality_flag)", [])?;
	}

	// Migration 2c: Add OSDU-inspired columns to curve_properties if missing
	// This handles databases created before the OSDU unit service was added
	let has_measurement_type_id: bool = conn
		.prepare("SELECT COUNT(*) FROM pragma_table_info('curve_properties') WHERE name = 'measurement_type_id'")?
		.query_row([], |row| row.get::<_, i64>(0))
		.map(|count| count > 0)
		.unwrap_or(false);

	if !has_measurement_type_id {
		info!("Running migration: adding OSDU-inspired columns to curve_properties table");
		conn.execute("ALTER TABLE curve_properties ADD COLUMN measurement_type_id TEXT REFERENCES measurement_types(id)", [])?;
		conn.execute("ALTER TABLE curve_properties ADD COLUMN min_valid_value REAL", [])?;
		conn.execute("ALTER TABLE curve_properties ADD COLUMN max_valid_value REAL", [])?;
	}

	// Migration 3: Create unified_views table if it doesn't exist
	// This is handled by the main schema, but we check explicitly for safety
	let unified_views_exists: bool = conn
		.prepare("SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='unified_views'")?
		.query_row([], |row| row.get::<_, i64>(0))
		.map(|count| count > 0)
		.unwrap_or(false);

	if !unified_views_exists {
		info!("Running migration: creating unified_views table");
		conn.execute_batch(
			r#"
			CREATE TABLE unified_views (
				id TEXT PRIMARY KEY,
				well_id TEXT NOT NULL REFERENCES wells(id) ON DELETE CASCADE,
				chunk_index INTEGER NOT NULL,
				start_depth REAL NOT NULL,
				end_depth REAL NOT NULL,
				row_count INTEGER NOT NULL,
				column_count INTEGER NOT NULL,
				columns_json TEXT NOT NULL,
				parquet_hash TEXT NOT NULL,
				generated_at TEXT NOT NULL DEFAULT (datetime('now')),
				UNIQUE(well_id, chunk_index)
			);
			CREATE INDEX IF NOT EXISTS idx_unified_views_well ON unified_views(well_id);
			"#,
		)?;
	}

	// Migration 4: Add well_name column to markers table if missing
	// This column stores the original well name from CSV for unmapped markers
	let markers_has_well_name: bool = conn
		.prepare("SELECT COUNT(*) FROM pragma_table_info('markers') WHERE name = 'well_name'")?
		.query_row([], |row| row.get::<_, i64>(0))
		.map(|count| count > 0)
		.unwrap_or(false);

	if !markers_has_well_name {
		info!("Running migration: adding well_name column to markers table");
		conn.execute("ALTER TABLE markers ADD COLUMN well_name TEXT", [])?;
		// Create the index after column exists
		conn.execute(
			"CREATE INDEX IF NOT EXISTS idx_markers_well_name ON markers(well_name) WHERE well_name IS NOT NULL",
			[],
		)?;
	}

	Ok(())
}

/// Open or create a database at the given path
pub fn open_db(path: &std::path::Path) -> SqliteResult<Connection> {
	let conn = Connection::open(path)?;

	// Enable foreign keys
	conn.execute_batch("PRAGMA foreign_keys = ON;")?;

	// WAL mode for better concurrency
	conn.execute_batch("PRAGMA journal_mode = WAL;")?;

	// Initialize schema
	init_db(&conn)?;

	Ok(conn)
}

/// Open an in-memory database (for testing)
pub fn open_memory_db() -> SqliteResult<Connection> {
	let conn = Connection::open_in_memory()?;
	conn.execute_batch("PRAGMA foreign_keys = ON;")?;
	init_db(&conn)?;
	Ok(conn)
}

/// Seed reference data tables
fn insert_default_reference_data(conn: &Connection) -> SqliteResult<()> {
	// Seed log types
	let log_types = [
		("raw", "Raw", "Unprocessed field data"),
		("processed", "Processed", "Quality-controlled and edited data"),
		("interpreted", "Interpreted", "Derived or computed curves"),
		("composite", "Composite", "Spliced from multiple runs"),
	];

	for (id, name, description) in log_types {
		conn.execute(
			"INSERT OR IGNORE INTO log_types (id, name, description) VALUES (?1, ?2, ?3)",
			rusqlite::params![id, name, description],
		)?;
	}

	// Seed acquisition types
	let acquisition_types = [
		("wireline", "Wireline", "Conventional wireline logging"),
		("lwd", "LWD", "Logging While Drilling"),
		("mwd", "MWD", "Measurement While Drilling"),
		("slickline", "Slickline", "Slickline-conveyed tools"),
		("coiled_tubing", "Coiled Tubing", "Coiled tubing-conveyed logging"),
	];

	for (id, name, description) in acquisition_types {
		conn.execute(
			"INSERT OR IGNORE INTO acquisition_types (id, name, description) VALUES (?1, ?2, ?3)",
			rusqlite::params![id, name, description],
		)?;
	}

	// ================================================================
	// MEASUREMENT TYPES AND UNITS (OSDU-inspired Unit Service)
	// ================================================================
	// Seed measurement types first
	let measurement_types = [
		("length", "Length", "Distance measurements"),
		("time", "Time", "Time duration"),
		("velocity", "Velocity", "Speed or velocity"),
		("density", "Density", "Mass per unit volume"),
		("porosity", "Porosity", "Volume fraction (dimensionless)"),
		("resistivity", "Resistivity", "Electrical resistivity"),
		("conductivity", "Conductivity", "Electrical conductivity"),
		("pressure", "Pressure", "Force per unit area"),
		("temperature", "Temperature", "Thermal measurement"),
		("gamma_ray", "Gamma Ray", "Natural radioactivity (API units)"),
		("slowness", "Slowness", "Time per unit length (acoustic)"),
		("voltage", "Voltage", "Electrical potential difference"),
		("force", "Force", "Mechanical force"),
		("dimensionless", "Dimensionless", "Unitless ratios and indices"),
		("photoelectric", "Photoelectric", "Photoelectric absorption index"),
		// Trajectory-related measurement types
		("angle", "Angle", "Angular measurements (inclination, azimuth)"),
		("angle_per_length", "Angular Rate", "Rate of angle change per length (dogleg severity)"),
	];

	for (id, name, description) in measurement_types {
		conn.execute(
			"INSERT OR IGNORE INTO measurement_types (id, name, description) VALUES (?1, ?2, ?3)",
			rusqlite::params![id, name, description],
		)?;
	}

	// Seed units with conversion factors
	// Format: (id, measurement_type_id, symbol, name, to_base_factor, to_base_offset, is_base)
	let units = [
		// Length units (base: meters)
		("m", "length", "m", "meters", 1.0, 0.0, true),
		("ft", "length", "ft", "feet", 0.3048, 0.0, false),
		("in", "length", "in", "inches", 0.0254, 0.0, false),
		("cm", "length", "cm", "centimeters", 0.01, 0.0, false),
		("mm", "length", "mm", "millimeters", 0.001, 0.0, false),
		// Time units (base: seconds)
		("s", "time", "s", "seconds", 1.0, 0.0, true),
		("ms", "time", "ms", "milliseconds", 0.001, 0.0, false),
		("us", "time", "us", "microseconds", 0.000001, 0.0, false),
		("min", "time", "min", "minutes", 60.0, 0.0, false),
		("hr", "time", "hr", "hours", 3600.0, 0.0, false),
		// Velocity units (base: m/s)
		("m_s", "velocity", "m/s", "meters per second", 1.0, 0.0, true),
		("ft_s", "velocity", "ft/s", "feet per second", 0.3048, 0.0, false),
		// Density units (base: kg/m3)
		("kg_m3", "density", "kg/m3", "kilograms per cubic meter", 1.0, 0.0, true),
		("g_cm3", "density", "g/cm3", "grams per cubic centimeter", 1000.0, 0.0, false),
		("lb_ft3", "density", "lb/ft3", "pounds per cubic foot", 16.0185, 0.0, false),
		// Porosity units (dimensionless, base: fraction)
		("v_v", "porosity", "v/v", "volume fraction", 1.0, 0.0, true),
		("pu", "porosity", "pu", "porosity units (percent)", 0.01, 0.0, false),
		("percent", "porosity", "%", "percent", 0.01, 0.0, false),
		// Resistivity units (base: ohm.m)
		("ohm_m", "resistivity", "ohm.m", "ohm-meters", 1.0, 0.0, true),
		("ohmm", "resistivity", "ohmm", "ohm-meters (alt)", 1.0, 0.0, false),
		// Conductivity units (base: S/m)
		("s_m", "conductivity", "S/m", "siemens per meter", 1.0, 0.0, true),
		("ms_m", "conductivity", "mS/m", "millisiemens per meter", 0.001, 0.0, false),
		// Pressure units (base: Pa)
		("pa", "pressure", "Pa", "pascals", 1.0, 0.0, true),
		("psi", "pressure", "psi", "pounds per square inch", 6894.76, 0.0, false),
		("kpa", "pressure", "kPa", "kilopascals", 1000.0, 0.0, false),
		("mpa", "pressure", "MPa", "megapascals", 1000000.0, 0.0, false),
		("bar", "pressure", "bar", "bar", 100000.0, 0.0, false),
		// Temperature units (base: Kelvin)
		("k", "temperature", "K", "kelvin", 1.0, 0.0, true),
		("degc", "temperature", "degC", "degrees Celsius", 1.0, 273.15, false),
		("degf", "temperature", "degF", "degrees Fahrenheit", 0.5556, 255.37, false),
		// Gamma ray (base: GAPI)
		("gapi", "gamma_ray", "GAPI", "API gamma ray units", 1.0, 0.0, true),
		// Slowness units (base: us/m)
		("us_m", "slowness", "us/m", "microseconds per meter", 1.0, 0.0, true),
		("us_ft", "slowness", "us/ft", "microseconds per foot", 3.28084, 0.0, false),
		// Voltage units (base: V)
		("v", "voltage", "V", "volts", 1.0, 0.0, true),
		("mv", "voltage", "mV", "millivolts", 0.001, 0.0, false),
		// Force units (base: N)
		("n", "force", "N", "newtons", 1.0, 0.0, true),
		("lbf", "force", "lbf", "pounds-force", 4.44822, 0.0, false),
		("klbf", "force", "klbf", "kilopounds-force", 4448.22, 0.0, false),
		// Photoelectric (base: b/e)
		("b_e", "photoelectric", "b/e", "barns per electron", 1.0, 0.0, true),
		// Dimensionless
		("unitless", "dimensionless", "", "unitless", 1.0, 0.0, true),
		("ratio", "dimensionless", "ratio", "ratio", 1.0, 0.0, false),
		// Angle units (base: radians)
		("rad", "angle", "rad", "radians", 1.0, 0.0, true),
		("deg", "angle", "deg", "degrees", 0.0174533, 0.0, false),  // pi/180
		("dega", "angle", "dega", "degrees (alternate)", 0.0174533, 0.0, false),
		("grad", "angle", "grad", "gradians", 0.0157080, 0.0, false),  // pi/200
		// Angular rate units (base: deg/100ft for DLS)
		("deg_100ft", "angle_per_length", "deg/100ft", "degrees per 100 feet", 1.0, 0.0, true),
		("deg_30m", "angle_per_length", "deg/30m", "degrees per 30 meters", 1.0164, 0.0, false),
		("deg_10m", "angle_per_length", "deg/10m", "degrees per 10 meters", 0.3281, 0.0, false),
	];

	for (id, measurement_type_id, symbol, name, to_base_factor, to_base_offset, is_base) in units {
		conn.execute(
			r#"INSERT OR IGNORE INTO units
			   (id, measurement_type_id, symbol, name, to_base_factor, to_base_offset, is_base)
			   VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)"#,
			rusqlite::params![id, measurement_type_id, symbol, name, to_base_factor, to_base_offset, is_base as i32],
		)?;
	}

	// Update measurement_types with base_unit_id
	conn.execute("UPDATE measurement_types SET base_unit_id = 'm' WHERE id = 'length'", [])?;
	conn.execute("UPDATE measurement_types SET base_unit_id = 's' WHERE id = 'time'", [])?;
	conn.execute("UPDATE measurement_types SET base_unit_id = 'm_s' WHERE id = 'velocity'", [])?;
	conn.execute("UPDATE measurement_types SET base_unit_id = 'kg_m3' WHERE id = 'density'", [])?;
	conn.execute("UPDATE measurement_types SET base_unit_id = 'v_v' WHERE id = 'porosity'", [])?;
	conn.execute("UPDATE measurement_types SET base_unit_id = 'ohm_m' WHERE id = 'resistivity'", [])?;
	conn.execute("UPDATE measurement_types SET base_unit_id = 's_m' WHERE id = 'conductivity'", [])?;
	conn.execute("UPDATE measurement_types SET base_unit_id = 'pa' WHERE id = 'pressure'", [])?;
	conn.execute("UPDATE measurement_types SET base_unit_id = 'k' WHERE id = 'temperature'", [])?;
	conn.execute("UPDATE measurement_types SET base_unit_id = 'gapi' WHERE id = 'gamma_ray'", [])?;
	conn.execute("UPDATE measurement_types SET base_unit_id = 'us_m' WHERE id = 'slowness'", [])?;
	conn.execute("UPDATE measurement_types SET base_unit_id = 'v' WHERE id = 'voltage'", [])?;
	conn.execute("UPDATE measurement_types SET base_unit_id = 'n' WHERE id = 'force'", [])?;
	conn.execute("UPDATE measurement_types SET base_unit_id = 'b_e' WHERE id = 'photoelectric'", [])?;
	conn.execute("UPDATE measurement_types SET base_unit_id = 'unitless' WHERE id = 'dimensionless'", [])?;
	conn.execute("UPDATE measurement_types SET base_unit_id = 'rad' WHERE id = 'angle'", [])?;
	conn.execute("UPDATE measurement_types SET base_unit_id = 'deg_100ft' WHERE id = 'angle_per_length'", [])?;

	// ================================================================
	// CURVE PROPERTIES (PWLS-style canonical properties)
	// ================================================================
	// Enhanced with measurement_type_id and valid value ranges
	// Format: (id, name, prop_class, measurement_type_id, unit, color, log_scale, min_val, max_val)
	let properties: [(&str, &str, &str, &str, &str, &str, bool, Option<f64>, Option<f64>); 14] = [
		("gamma_ray", "Gamma Ray", "Natural Radioactivity", "gamma_ray", "GAPI", "#00FF00", false, Some(0.0), Some(300.0)),
		("bulk_density", "Bulk Density", "Density", "density", "g/cm3", "#FF0000", false, Some(1.0), Some(3.5)),
		("neutron_porosity", "Neutron Porosity", "Porosity", "porosity", "v/v", "#0000FF", false, Some(-0.15), Some(0.6)),
		("deep_resistivity", "Deep Resistivity", "Resistivity", "resistivity", "ohm.m", "#FF00FF", true, Some(0.1), Some(10000.0)),
		("medium_resistivity", "Medium Resistivity", "Resistivity", "resistivity", "ohm.m", "#FF00AA", true, Some(0.1), Some(10000.0)),
		("shallow_resistivity", "Shallow Resistivity", "Resistivity", "resistivity", "ohm.m", "#FF0055", true, Some(0.1), Some(10000.0)),
		("caliper", "Caliper", "Borehole Geometry", "length", "in", "#808080", false, Some(0.0), Some(30.0)),
		("compressional_slowness", "Compressional Slowness", "Acoustic", "slowness", "us/ft", "#800080", false, Some(40.0), Some(200.0)),
		("shear_slowness", "Shear Slowness", "Acoustic", "slowness", "us/ft", "#400080", false, Some(60.0), Some(400.0)),
		("spontaneous_potential", "Spontaneous Potential", "Electrical", "voltage", "mV", "#FFFF00", false, Some(-200.0), Some(200.0)),
		("photoelectric", "Photoelectric Effect", "Density", "photoelectric", "b/e", "#FFA500", false, Some(0.0), Some(10.0)),
		("depth", "Depth", "Index", "length", "ft", "#000000", false, None, None),
		("bit_size", "Bit Size", "Borehole Geometry", "length", "in", "#A0A0A0", false, Some(0.0), Some(30.0)),
		("tension", "Cable Tension", "Tool Status", "force", "lbf", "#C0C0C0", false, Some(0.0), Some(10000.0)),
	];

	for (id, name, prop_class, measurement_type_id, unit, color, log_scale, min_val, max_val) in properties {
		conn.execute(
			r#"INSERT OR IGNORE INTO curve_properties
			   (id, canonical_name, property_class, measurement_type_id, typical_unit, display_color, log_scale, min_valid_value, max_valid_value)
			   VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)"#,
			rusqlite::params![id, name, prop_class, measurement_type_id, unit, color, log_scale as i32, min_val, max_val],
		)?;
	}

	// Seed mnemonic mappings (priority: higher = more likely match)
	let mnemonics = [
		// Gamma Ray variants
		("GR", "gamma_ray", 100, None::<&str>),
		("GR1", "gamma_ray", 90, None),
		("GR2", "gamma_ray", 85, None),
		("SGR", "gamma_ray", 80, Some("Spectral")),
		("CGR", "gamma_ray", 80, Some("Computed")),
		("GRD", "gamma_ray", 70, None),
		("GRS", "gamma_ray", 70, None),
		("GAMMA", "gamma_ray", 60, None),
		("HSGR", "gamma_ray", 75, Some("Halliburton")),
		// Bulk Density variants
		("RHOB", "bulk_density", 100, None),
		("RHOZ", "bulk_density", 95, Some("Z-corrected")),
		("DEN", "bulk_density", 80, None),
		("ZDEN", "bulk_density", 85, Some("Z-corrected")),
		("DENSITY", "bulk_density", 70, None),
		// Neutron Porosity variants
		("NPHI", "neutron_porosity", 100, None),
		("TNPH", "neutron_porosity", 95, Some("Thermal")),
		("NPOR", "neutron_porosity", 85, None),
		("NEU", "neutron_porosity", 80, None),
		("PHIN", "neutron_porosity", 75, None),
		("NEUTRON", "neutron_porosity", 60, None),
		// Deep Resistivity variants
		("RT", "deep_resistivity", 100, None),
		("LLD", "deep_resistivity", 95, Some("Laterolog Deep")),
		("ILD", "deep_resistivity", 95, Some("Induction Deep")),
		("HDRS", "deep_resistivity", 90, Some("Halliburton")),
		("RD", "deep_resistivity", 85, None),
		("RILD", "deep_resistivity", 85, None),
		("AT90", "deep_resistivity", 80, Some("Array")),
		// Medium Resistivity variants
		("RXO", "medium_resistivity", 100, None),
		("LLM", "medium_resistivity", 95, Some("Laterolog Medium")),
		("ILM", "medium_resistivity", 95, Some("Induction Medium")),
		("HMRS", "medium_resistivity", 90, Some("Halliburton")),
		// Shallow Resistivity variants
		("RS", "shallow_resistivity", 100, None),
		("LLS", "shallow_resistivity", 95, Some("Laterolog Shallow")),
		("MSFL", "shallow_resistivity", 90, Some("Microspherically Focused")),
		("RFOC", "shallow_resistivity", 85, None),
		// Caliper variants
		("CALI", "caliper", 100, None),
		("CAL", "caliper", 95, None),
		("HCAL", "caliper", 90, Some("Halliburton")),
		("C1", "caliper", 80, None),
		("C2", "caliper", 80, None),
		("CALIPER", "caliper", 70, None),
		// Sonic variants
		("DT", "compressional_slowness", 100, None),
		("DTCO", "compressional_slowness", 95, Some("Compressional")),
		("DTC", "compressional_slowness", 95, None),
		("AC", "compressional_slowness", 90, None),
		("SONIC", "compressional_slowness", 70, None),
		("DTS", "shear_slowness", 100, Some("Shear")),
		("DTSM", "shear_slowness", 95, Some("Shear")),
		// SP variants
		("SP", "spontaneous_potential", 100, None),
		("SSP", "spontaneous_potential", 90, Some("Static")),
		// PE variants
		("PE", "photoelectric", 100, None),
		("PEF", "photoelectric", 95, None),
		("PEFZ", "photoelectric", 90, Some("Z-corrected")),
		// Depth variants
		("DEPT", "depth", 100, None),
		("DEPTH", "depth", 95, None),
		("MD", "depth", 90, Some("Measured Depth")),
		("TVD", "depth", 85, Some("True Vertical Depth")),
		("TVDSS", "depth", 80, Some("TVD Subsea")),
		// Bit size
		("BS", "bit_size", 100, None),
		("BIT", "bit_size", 90, None),
		// Tension
		("TENS", "tension", 100, None),
		("TEN", "tension", 90, None),
	];

	for (mnemonic, property_id, priority, notes) in mnemonics {
		conn.execute(
			r#"INSERT OR IGNORE INTO curve_mnemonics
			   (mnemonic, property_id, priority, notes)
			   VALUES (?1, ?2, ?3, ?4)"#,
			rusqlite::params![mnemonic, property_id, priority, notes],
		)?;
	}

	// Also populate legacy curve_metadata for backwards compatibility
	let legacy_defaults = [
		("GR", "GR", "Gamma Ray", "GAPI", &["SGR", "CGR", "GRD", "GRS"][..]),
		("RHOB", "RHOB", "Bulk Density", "g/cm3", &["RHOZ", "DEN", "ZDEN"][..]),
		("NPHI", "NPHI", "Neutron Porosity", "v/v", &["TNPH", "NPOR", "NEU"][..]),
		("RT", "RT", "True Resistivity", "ohm.m", &["LLD", "ILD", "HDRS"][..]),
		("CALI", "CALI", "Caliper", "in", &["CAL", "HCAL", "C1"][..]),
		("DT", "DT", "Sonic", "us/ft", &["AC", "DTC", "DTCO"][..]),
		("SP", "SP", "Spontaneous Potential", "mV", &["SSP"][..]),
		("PE", "PE", "Photoelectric", "b/e", &["PEF", "PEFZ"][..]),
		("DEPT", "DEPTH", "Depth", "ft", &["DEPTH", "MD", "TVD"][..]),
	];

	for (mnemonic, curve_type, display_name, units, variations) in legacy_defaults {
		let id = uuid::Uuid::new_v4().to_string();
		let variations_json = serde_json::to_string(variations).unwrap_or_default();

		conn.execute(
			r#"INSERT OR IGNORE INTO curve_metadata
			   (id, curve_mnemonic, main_curve_type, display_name, units, vendor_variations)
			   VALUES (?1, ?2, ?3, ?4, ?5, ?6)"#,
			rusqlite::params![id, mnemonic, curve_type, display_name, units, variations_json],
		)?;
	}

	Ok(())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_create_memory_db() {
		let conn = open_memory_db().expect("Failed to create memory database");

		// Verify tables exist
		let tables: Vec<String> = conn
			.prepare("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name")
			.unwrap()
			.query_map([], |row| row.get(0))
			.unwrap()
			.filter_map(|r| r.ok())
			.collect();

		assert!(tables.contains(&"wells".to_string()));
		assert!(tables.contains(&"curves".to_string()));
		assert!(tables.contains(&"blob_registry".to_string()));
		assert!(tables.contains(&"log_runs".to_string()));
		assert!(tables.contains(&"curve_properties".to_string()));
		assert!(tables.contains(&"curve_mnemonics".to_string()));
		assert!(tables.contains(&"log_types".to_string()));
		assert!(tables.contains(&"acquisition_types".to_string()));
		assert!(tables.contains(&"unified_views".to_string()));
		// New OSDU-inspired tables
		assert!(tables.contains(&"measurement_types".to_string()));
		assert!(tables.contains(&"units".to_string()));
		assert!(tables.contains(&"curve_versions".to_string()));
		// Trajectory tables
		assert!(tables.contains(&"survey_runs".to_string()));
		assert!(tables.contains(&"trajectory_columns".to_string()));
		// Marker tables
		assert!(tables.contains(&"marker_sets".to_string()));
		assert!(tables.contains(&"markers".to_string()));
		// Surface tables
		assert!(tables.contains(&"surfaces".to_string()));
		// Checkshot tables
		assert!(tables.contains(&"checkshot_runs".to_string()));
		assert!(tables.contains(&"checkshot_columns".to_string()));
	}

	#[test]
	fn test_checkshot_tables_exist() {
		let conn = open_memory_db().expect("Failed to create memory database");

		// Check checkshot_runs columns
		let run_columns: Vec<String> = conn
			.prepare("SELECT name FROM pragma_table_info('checkshot_runs')")
			.unwrap()
			.query_map([], |row| row.get(0))
			.unwrap()
			.filter_map(|r| r.ok())
			.collect();

		assert!(run_columns.contains(&"id".to_string()));
		assert!(run_columns.contains(&"well_id".to_string()));
		assert!(run_columns.contains(&"workspace_id".to_string()));
		assert!(run_columns.contains(&"min_twt".to_string()));
		assert!(run_columns.contains(&"max_twt".to_string()));
		assert!(run_columns.contains(&"time_unit".to_string()));
		assert!(run_columns.contains(&"depth_unit".to_string()));
		assert!(run_columns.contains(&"survey_type".to_string()));
		assert!(run_columns.contains(&"station_count".to_string()));

		// Check checkshot_columns
		let col_columns: Vec<String> = conn
			.prepare("SELECT name FROM pragma_table_info('checkshot_columns')")
			.unwrap()
			.query_map([], |row| row.get(0))
			.unwrap()
			.filter_map(|r| r.ok())
			.collect();

		assert!(col_columns.contains(&"id".to_string()));
		assert!(col_columns.contains(&"checkshot_run_id".to_string()));
		assert!(col_columns.contains(&"well_id".to_string()));
		assert!(col_columns.contains(&"column_type".to_string()));
		assert!(col_columns.contains(&"parquet_hash".to_string()));
		assert!(col_columns.contains(&"min_value".to_string()));
		assert!(col_columns.contains(&"max_value".to_string()));
	}

	#[test]
	fn test_reference_data_seeded() {
		let conn = open_memory_db().expect("Failed to create memory database");

		// Check log types
		let log_type_count: i64 = conn
			.query_row("SELECT COUNT(*) FROM log_types", [], |row| row.get(0))
			.unwrap();
		assert!(log_type_count >= 4, "Should have at least 4 log types");

		// Check acquisition types
		let acq_type_count: i64 = conn
			.query_row("SELECT COUNT(*) FROM acquisition_types", [], |row| row.get(0))
			.unwrap();
		assert!(acq_type_count >= 5, "Should have at least 5 acquisition types");

		// Check curve properties
		let prop_count: i64 = conn
			.query_row("SELECT COUNT(*) FROM curve_properties", [], |row| row.get(0))
			.unwrap();
		assert!(prop_count >= 10, "Should have at least 10 curve properties");

		// Check curve mnemonics
		let mnemonic_count: i64 = conn
			.query_row("SELECT COUNT(*) FROM curve_mnemonics", [], |row| row.get(0))
			.unwrap();
		assert!(mnemonic_count >= 50, "Should have at least 50 mnemonic mappings");
	}

	#[test]
	fn test_measurement_types_seeded() {
		let conn = open_memory_db().expect("Failed to create memory database");

		// Check measurement types
		let count: i64 = conn
			.query_row("SELECT COUNT(*) FROM measurement_types", [], |row| row.get(0))
			.unwrap();
		assert!(count >= 10, "Should have at least 10 measurement types");

		// Verify base_unit_id is set
		let length_base: String = conn
			.query_row(
				"SELECT base_unit_id FROM measurement_types WHERE id = 'length'",
				[],
				|row| row.get(0),
			)
			.unwrap();
		assert_eq!(length_base, "m");
	}

	#[test]
	fn test_units_seeded() {
		let conn = open_memory_db().expect("Failed to create memory database");

		// Check units
		let count: i64 = conn
			.query_row("SELECT COUNT(*) FROM units", [], |row| row.get(0))
			.unwrap();
		assert!(count >= 30, "Should have at least 30 units");

		// Verify feet to meters conversion
		let (symbol, factor): (String, f64) = conn
			.query_row(
				"SELECT symbol, to_base_factor FROM units WHERE id = 'ft'",
				[],
				|row| Ok((row.get(0)?, row.get(1)?)),
			)
			.unwrap();
		assert_eq!(symbol, "ft");
		assert!((factor - 0.3048).abs() < 0.0001);
	}

	#[test]
	fn test_unit_conversion_lookup() {
		let conn = open_memory_db().expect("Failed to create memory database");

		// Look up all length units with their conversion factors
		let mut stmt = conn
			.prepare(
				r#"SELECT u.symbol, u.to_base_factor, u.is_base
				   FROM units u
				   JOIN measurement_types mt ON u.measurement_type_id = mt.id
				   WHERE mt.id = 'length'
				   ORDER BY u.is_base DESC, u.symbol"#,
			)
			.unwrap();

		let units: Vec<(String, f64, bool)> = stmt
			.query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get::<_, i32>(2)? == 1)))
			.unwrap()
			.filter_map(|r| r.ok())
			.collect();

		// Should have multiple length units
		assert!(units.len() >= 4, "Should have at least 4 length units");

		// Base unit should be meters with factor 1.0
		let base = units.iter().find(|(_, _, is_base)| *is_base).unwrap();
		assert_eq!(base.0, "m");
		assert!((base.1 - 1.0).abs() < 0.0001);
	}

	#[test]
	fn test_curve_property_measurement_type_link() {
		let conn = open_memory_db().expect("Failed to create memory database");

		// Verify curve properties have measurement_type_id set
		let result: (String, String, String) = conn
			.query_row(
				r#"SELECT cp.canonical_name, cp.measurement_type_id, mt.name
				   FROM curve_properties cp
				   JOIN measurement_types mt ON cp.measurement_type_id = mt.id
				   WHERE cp.id = 'gamma_ray'"#,
				[],
				|row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
			)
			.unwrap();

		assert_eq!(result.0, "Gamma Ray");
		assert_eq!(result.1, "gamma_ray");
		assert_eq!(result.2, "Gamma Ray");

		// Verify valid value ranges
		let (min_val, max_val): (Option<f64>, Option<f64>) = conn
			.query_row(
				"SELECT min_valid_value, max_valid_value FROM curve_properties WHERE id = 'gamma_ray'",
				[],
				|row| Ok((row.get(0)?, row.get(1)?)),
			)
			.unwrap();

		assert_eq!(min_val, Some(0.0));
		assert_eq!(max_val, Some(300.0));
	}

	#[test]
	fn test_mnemonic_to_property_lookup() {
		let conn = open_memory_db().expect("Failed to create memory database");

		// Look up GR mnemonic
		let result: (String, String, i32) = conn
			.query_row(
				r#"SELECT m.mnemonic, p.canonical_name, m.priority
				   FROM curve_mnemonics m
				   JOIN curve_properties p ON m.property_id = p.id
				   WHERE m.mnemonic = 'GR'"#,
				[],
				|row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
			)
			.unwrap();

		assert_eq!(result.0, "GR");
		assert_eq!(result.1, "Gamma Ray");
		assert_eq!(result.2, 100);
	}

	#[test]
	fn test_legacy_curve_metadata() {
		let conn = open_memory_db().expect("Failed to create memory database");

		let count: i64 = conn
			.query_row("SELECT COUNT(*) FROM curve_metadata", [], |row| row.get(0))
			.unwrap();

		assert!(count >= 9, "Should have at least 9 default curve types");
	}

	#[test]
	fn test_angle_units_seeded() {
		let conn = open_memory_db().expect("Failed to create memory database");

		// Check that angle measurement type exists
		let angle_type: String = conn
			.query_row(
				"SELECT name FROM measurement_types WHERE id = 'angle'",
				[],
				|row| row.get(0),
			)
			.unwrap();
		assert_eq!(angle_type, "Angle");

		// Check that angle units exist
		let deg_factor: f64 = conn
			.query_row(
				"SELECT to_base_factor FROM units WHERE id = 'deg'",
				[],
				|row| row.get(0),
			)
			.unwrap();
		// pi/180 ≈ 0.0174533
		assert!((deg_factor - 0.0174533).abs() < 0.0001);

		// Check that DLS units exist
		let dls_unit: String = conn
			.query_row(
				"SELECT symbol FROM units WHERE id = 'deg_100ft'",
				[],
				|row| row.get(0),
			)
			.unwrap();
		assert_eq!(dls_unit, "deg/100ft");
	}

	#[test]
	fn test_trajectory_tables_exist() {
		let conn = open_memory_db().expect("Failed to create memory database");

		// Check survey_runs columns
		let survey_columns: Vec<String> = conn
			.prepare("SELECT name FROM pragma_table_info('survey_runs')")
			.unwrap()
			.query_map([], |row| row.get(0))
			.unwrap()
			.filter_map(|r| r.ok())
			.collect();

		assert!(survey_columns.contains(&"id".to_string()));
		assert!(survey_columns.contains(&"well_id".to_string()));
		assert!(survey_columns.contains(&"survey_type".to_string()));
		assert!(survey_columns.contains(&"calculation_method".to_string()));
		assert!(survey_columns.contains(&"magnetic_declination".to_string()));
		assert!(survey_columns.contains(&"tie_in_md".to_string()));

		// Check trajectory_columns columns
		let traj_columns: Vec<String> = conn
			.prepare("SELECT name FROM pragma_table_info('trajectory_columns')")
			.unwrap()
			.query_map([], |row| row.get(0))
			.unwrap()
			.filter_map(|r| r.ok())
			.collect();

		assert!(traj_columns.contains(&"id".to_string()));
		assert!(traj_columns.contains(&"survey_run_id".to_string()));
		assert!(traj_columns.contains(&"column_type".to_string()));
		assert!(traj_columns.contains(&"is_input".to_string()));
		assert!(traj_columns.contains(&"is_calculated".to_string()));
		assert!(traj_columns.contains(&"parquet_hash".to_string()));
	}

	#[test]
	fn test_curves_table_has_quality_fields() {
		let conn = open_memory_db().expect("Failed to create memory database");

		// Check that curves table has new quality fields
		let columns: Vec<String> = conn
			.prepare("SELECT name FROM pragma_table_info('curves')")
			.unwrap()
			.query_map([], |row| row.get(0))
			.unwrap()
			.filter_map(|r| r.ok())
			.collect();

		assert!(columns.contains(&"null_value".to_string()));
		assert!(columns.contains(&"quality_flag".to_string()));
		assert!(columns.contains(&"acquisition_date".to_string()));
		assert!(columns.contains(&"service_company".to_string()));
		assert!(columns.contains(&"unit_id".to_string()));
	}

	#[test]
	fn test_curve_versions_table_exists() {
		let conn = open_memory_db().expect("Failed to create memory database");

		// Check that curve_versions table has expected columns
		let columns: Vec<String> = conn
			.prepare("SELECT name FROM pragma_table_info('curve_versions')")
			.unwrap()
			.query_map([], |row| row.get(0))
			.unwrap()
			.filter_map(|r| r.ok())
			.collect();

		assert!(columns.contains(&"id".to_string()));
		assert!(columns.contains(&"curve_id".to_string()));
		assert!(columns.contains(&"version".to_string()));
		assert!(columns.contains(&"reason".to_string()));
		assert!(columns.contains(&"native_parquet_hash".to_string()));
	}

	#[test]
	fn test_marker_tables_exist() {
		let conn = open_memory_db().expect("Failed to create memory database");

		// Check marker_sets columns
		let marker_set_columns: Vec<String> = conn
			.prepare("SELECT name FROM pragma_table_info('marker_sets')")
			.unwrap()
			.query_map([], |row| row.get(0))
			.unwrap()
			.filter_map(|r| r.ok())
			.collect();

		assert!(marker_set_columns.contains(&"id".to_string()));
		assert!(marker_set_columns.contains(&"well_id".to_string()));
		assert!(marker_set_columns.contains(&"workspace_id".to_string()));
		assert!(marker_set_columns.contains(&"source_filename".to_string()));
		assert!(marker_set_columns.contains(&"interpretation_type".to_string()));
		assert!(marker_set_columns.contains(&"depth_unit".to_string()));
		assert!(marker_set_columns.contains(&"min_depth".to_string()));
		assert!(marker_set_columns.contains(&"max_depth".to_string()));
		assert!(marker_set_columns.contains(&"marker_count".to_string()));

		// Check markers columns
		let marker_columns: Vec<String> = conn
			.prepare("SELECT name FROM pragma_table_info('markers')")
			.unwrap()
			.query_map([], |row| row.get(0))
			.unwrap()
			.filter_map(|r| r.ok())
			.collect();

		assert!(marker_columns.contains(&"id".to_string()));
		assert!(marker_columns.contains(&"marker_set_id".to_string()));
		assert!(marker_columns.contains(&"well_id".to_string()));
		assert!(marker_columns.contains(&"name".to_string()));
		assert!(marker_columns.contains(&"marker_type".to_string()));
		assert!(marker_columns.contains(&"measured_depth".to_string()));
		assert!(marker_columns.contains(&"true_vertical_depth".to_string()));
		assert!(marker_columns.contains(&"quality".to_string()));
		assert!(marker_columns.contains(&"original_values".to_string()));
	}

	#[test]
	fn test_surfaces_table_exists() {
		let conn = open_memory_db().expect("Failed to create memory database");

		// Check surfaces columns
		let surface_columns: Vec<String> = conn
			.prepare("SELECT name FROM pragma_table_info('surfaces')")
			.unwrap()
			.query_map([], |row| row.get(0))
			.unwrap()
			.filter_map(|r| r.ok())
			.collect();

		// Core fields
		assert!(surface_columns.contains(&"id".to_string()));
		assert!(surface_columns.contains(&"workspace_id".to_string()));
		assert!(surface_columns.contains(&"source_filename".to_string()));
		assert!(surface_columns.contains(&"raw_file_blob_hash".to_string()));
		assert!(surface_columns.contains(&"data_blob_hash".to_string()));

		// Surface metadata
		assert!(surface_columns.contains(&"name".to_string()));
		assert!(surface_columns.contains(&"description".to_string()));
		assert!(surface_columns.contains(&"surface_type".to_string()));

		// Spatial extent
		assert!(surface_columns.contains(&"min_x".to_string()));
		assert!(surface_columns.contains(&"max_x".to_string()));
		assert!(surface_columns.contains(&"min_y".to_string()));
		assert!(surface_columns.contains(&"max_y".to_string()));
		assert!(surface_columns.contains(&"min_z".to_string()));
		assert!(surface_columns.contains(&"max_z".to_string()));

		// Coordinate reference
		assert!(surface_columns.contains(&"crs".to_string()));
		assert!(surface_columns.contains(&"xy_unit".to_string()));
		assert!(surface_columns.contains(&"z_unit".to_string()));
		assert!(surface_columns.contains(&"z_positive_direction".to_string()));

		// Point data info
		assert!(surface_columns.contains(&"point_count".to_string()));

		// Grid info
		assert!(surface_columns.contains(&"is_regular_grid".to_string()));
		assert!(surface_columns.contains(&"grid_ni".to_string()));
		assert!(surface_columns.contains(&"grid_nj".to_string()));
		assert!(surface_columns.contains(&"grid_origin_x".to_string()));
		assert!(surface_columns.contains(&"grid_origin_y".to_string()));
		assert!(surface_columns.contains(&"grid_spacing_x".to_string()));
		assert!(surface_columns.contains(&"grid_spacing_y".to_string()));

		// Tracking
		assert!(surface_columns.contains(&"imported_by".to_string()));
		assert!(surface_columns.contains(&"imported_at".to_string()));
		assert!(surface_columns.contains(&"version".to_string()));
		assert!(surface_columns.contains(&"deleted_at".to_string()));
	}
}
