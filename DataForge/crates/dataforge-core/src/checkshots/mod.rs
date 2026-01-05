//! Checkshot Data Module
//!
//! Handles parsing, processing, and storing checkshot (time-depth) data.
//! Following OSDU patterns for checkshot stations with MD, TVD, TWT columns.
//!
//! ## Submodules
//!
//! - `detection` - Column type detection from CSV headers
//! - `parser` - CSV parsing with delimiter detection
//! - `ingest` - Main ingestion workflow

pub mod detection;
pub mod ingest;
pub mod parser;

// Re-export commonly used types
pub use detection::{detect_checkshot_column_type, detect_checkshot_column_with_unit, DetectedCheckColumn};
pub use ingest::{
    ingest_checkshot_file, CheckshotColumnMapping, CheckshotIngestOptions, CheckshotIngestResult,
};
pub use parser::{parse_checkshot_csv, ParsedCheckshot, ParsedCheckColumn};
