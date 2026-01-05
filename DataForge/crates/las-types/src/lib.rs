//! Core data structures and types for LAS well log files.
//!
//! This crate provides the foundational types for working with LAS
//! (Log ASCII Standard) files used in the oil and gas industry.
//!
//! # Main Types
//!
//! - [`LASFile`] - The main structure representing a parsed LAS file
//! - [`CurveSection`] - Curve definitions and data
//! - [`WellSection`] - Well metadata
//! - [`MainCurveType`] - Standardized curve type classifications
//! - [`CurveMappingDictionary`] - Configurable curve name mappings

pub mod curve_mapping_ops;
pub mod las_file;
pub mod petrophysics_schema;
pub mod sections;

// Re-export main file type
pub use las_file::LASFile;

// Re-export petrophysical types
pub use petrophysics_schema::{
    CurveDetection, CurveMappingConfig, CurveMappingDictionary, CurveMappingEntry,
    LasProcessingResult, MainCurveType, SubcurveMapping, WellMetadata,
};

// Re-export curve mapping operations
pub use curve_mapping_ops::{
    batch_update_curve_mappings, export_curve_mappings, get_mapped_curves, get_mapping_statistics,
    get_unmapped_curves, update_curve_mapping, validate_curve_mappings, CurveMappingExport,
    CurveMappingStats,
};

// Re-export section types
pub use sections::{
    curve::{build_curve_metadata_map, CurveDataColumn, CurveHeaderParseMode, CurveMetadata, CurveSection},
    parameter::ParameterSection,
    version::VersionSection,
    well::{WellEntry, WellSection, WellSectionError},
    HeaderItem, SectionItems,
};
