//! DataForge Core
//!
//! Core data types, database schema, and business logic for the DataForge MVP.
//!
//! ## Modules
//!
//! - `auth` - Authentication and session management
//! - `db` - SQLite database schema and operations
//! - `blob` - Content-addressed blob storage
//! - `models` - Domain models (Well, Curve, Project, etc.)
//! - `las` - LAS file processing (wraps las-parser)
//! - `parquet` - Parquet file generation and querying
//! - `sync` - Sync infrastructure (repositories, traits) for offline-first sync
//! - `wellgrid` - Well depth grid and resampling utilities
//! - `trajectory` - Trajectory/survey data parsing, detection, and calculation
//! - `markers` - Well tops/markers parsing, detection, and ingestion
//! - `surfaces` - 3D surface data (horizons, faults) parsing, detection, and ingestion
//! - `checkshots` - Checkshot/time-depth data parsing, detection, and ingestion

pub mod auth;
pub mod blob;
pub mod checkshots;
pub mod db;
pub mod error;
pub mod ingest;
pub mod las;
pub mod markers;
pub mod models;
pub mod parquet;
pub mod surfaces;
pub mod sync;
pub mod trajectory;
pub mod unified_view;
pub mod units;
pub mod wellgrid;

pub use auth::AuthError;
pub use error::{Error, Result};
pub use ingest::{ingest_las_file, IngestOptions, IngestResult};
pub use ingest::{
	update_curve_data, add_curve_rows, delete_curve_rows,
	get_curve_versions, revert_curve_to_version,
	CurveUpdateResult, CurveVersion,
};
pub use parquet::CellEdit;
pub use unified_view::{build_unified_views, UnifiedViewOptions, UnifiedViewResult};
pub use units::{UnitService, detect_unit_id, normalize_unit_symbol};
pub use wellgrid::{DepthUnit, ResampleMethod, WellDepthGrid};

// Trajectory exports
pub use trajectory::{
    calculate_dls, minimum_curvature, CalculatedStation, TieInPoint, TrajectoryCalculationResult,
    detect_column_type, DetectedColumn,
    parse_trajectory_csv, ParsedTrajectory, ParsedTrajectoryColumn,
    ingest_trajectory_file, ColumnMapping, TrajectoryIngestOptions, TrajectoryIngestResult,
    read_parquet_column_values,
};

// Markers exports
pub use markers::{
    detect_marker_column_type, detect_marker_column_with_unit, DetectedMarkerColumn,
    parse_marker_csv, ParsedMarkerColumn, ParsedMarkerFile, ParsedMarkerRow,
    ingest_marker_file, MarkerColumnMapping, MarkerIngestOptions, MarkerIngestResult,
    WellMarkerGroup, WellMatchMode,
};

// Surfaces exports
pub use surfaces::{
    detect_surface_column_type, detect_surface_column_with_unit, DetectedSurfaceColumn,
    parse_surface_csv, ParsedSurfaceFile, ParsedSurfacePoint, SurfaceGroup,
    ingest_surface_file, SurfaceColumnMapping, SurfaceIngestConfig, SurfaceIngestResult,
};

// Checkshots exports
pub use checkshots::{
    detect_checkshot_column_type, detect_checkshot_column_with_unit, DetectedCheckColumn,
    parse_checkshot_csv, ParsedCheckshot, ParsedCheckColumn,
    ingest_checkshot_file, CheckshotColumnMapping, CheckshotIngestOptions, CheckshotIngestResult,
};
