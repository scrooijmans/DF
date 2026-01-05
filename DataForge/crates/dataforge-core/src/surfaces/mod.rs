//! 3D Surface Module
//!
//! Handles parsing, processing, and storing 3D surface data from CSV files.
//! Following OSDU work-product-component--Surface patterns with X, Y, Z
//! coordinates, spatial extent, and optional CRS information.
//!
//! ## Key Features
//!
//! - **Workspace-level entities**: Surfaces belong to workspaces, not wells
//! - **Multi-surface file support**: One CSV can contain multiple named surfaces
//! - **Automatic column detection**: Headers are analyzed to detect X, Y, Z columns
//! - **Spatial extent calculation**: Bounding box computed from point data
//! - **Parquet storage**: Points stored efficiently in columnar format
//!
//! ## Submodules
//!
//! - `detection` - Column type detection from CSV headers
//! - `parser` - CSV parsing with delimiter detection and multi-surface support
//! - `ingest` - Main ingestion workflow with Parquet storage

pub mod detection;
pub mod ingest;
pub mod parser;

// Re-export commonly used types
pub use detection::{detect_surface_column_type, detect_surface_column_with_unit, DetectedSurfaceColumn};
pub use ingest::{
    ingest_surface_file, SurfaceColumnMapping, SurfaceIngestConfig, SurfaceIngestResult,
};
pub use parser::{parse_surface_csv, ParsedSurfaceFile, ParsedSurfacePoint, SurfaceGroup};
