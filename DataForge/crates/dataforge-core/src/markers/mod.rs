//! Well Tops/Markers Module
//!
//! Handles parsing, processing, and storing wellbore marker (well tops) data.
//! Following OSDU WellboreMarkerSet patterns with MarkerName, MarkerMeasuredDepth,
//! WellboreID, and DepthUnit as core fields.
//!
//! ## Key Features
//!
//! - **Multi-well file support**: One CSV can contain markers for multiple wells
//! - **Automatic column detection**: Headers are analyzed to detect marker columns
//! - **Well matching**: Markers are matched to existing wells or new wells are created
//! - **Flexible parsing**: Supports tab, comma, semicolon delimiters
//!
//! ## Submodules
//!
//! - `detection` - Column type detection from CSV headers
//! - `parser` - CSV parsing with delimiter detection and multi-well support
//! - `ingest` - Main ingestion workflow with well matching/creation

pub mod detection;
pub mod ingest;
pub mod parser;

// Re-export commonly used types
pub use detection::{detect_marker_column_type, detect_marker_column_with_unit, DetectedMarkerColumn};
pub use ingest::{
    ingest_marker_file, MarkerColumnMapping, MarkerIngestOptions, MarkerIngestResult,
    WellMarkerGroup, WellMatchMode,
};
pub use parser::{parse_marker_csv, ParsedMarkerColumn, ParsedMarkerFile, ParsedMarkerRow};
