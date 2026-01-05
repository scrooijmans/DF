//! LAS file parsing library for oil & gas well log data.
//!
//! This crate provides tools for parsing LAS (Log ASCII Standard) files,
//! which are commonly used in the oil and gas industry for well log data.
//!
//! # Features
//!
//! - Parse LAS 1.2, 2.0, and 3.0 file formats
//! - Automatic curve type detection and mapping
//! - Configurable curve name dictionaries
//! - Well metadata extraction
//!
//! # Example
//!
//! ```no_run
//! use las_parser::{LasParser, LasResult};
//!
//! fn main() -> LasResult<()> {
//!     let parser = LasParser::new();
//!     let las_file = parser.parse_las_file("well_log.las")?;
//!
//!     println!("Parsed {} curves", las_file.curves.columns.len());
//!     Ok(())
//! }
//! ```

pub mod las_parser;

// Re-export main parser types
pub use las_parser::{LasParseError, LasParser, LasResult};

// Re-export LASFile from las-types for convenience
pub use las_types::LASFile;

// Re-export commonly used types from las-types
pub use las_types::{
    batch_update_curve_mappings, export_curve_mappings, get_mapped_curves, get_mapping_statistics,
    get_unmapped_curves, update_curve_mapping, validate_curve_mappings, CurveDetection,
    CurveMappingDictionary, CurveMappingExport, CurveMappingStats, LasProcessingResult,
    MainCurveType, SubcurveMapping, WellMetadata,
};
