//! LAS file parser with curve type detection and mapping.
//!
//! This module provides the main [`LasParser`] struct for parsing LAS well log files
//! and detecting curve types based on configurable mapping dictionaries.

use las_types::{
    CurveDetection, CurveMappingDictionary, LASFile, LasProcessingResult, MainCurveType,
    WellMetadata,
};
use tracing::{debug, info, warn};

/// Result type for LAS parsing operations.
pub type LasResult<T> = Result<T, LasParseError>;

/// Errors that can occur during LAS file parsing.
#[derive(Debug, thiserror::Error)]
pub enum LasParseError {
    /// I/O error reading the file
    #[error("Failed to read LAS file: {0}")]
    Io(#[from] std::io::Error),

    /// File parsing error
    #[error("Failed to parse LAS file: {0}")]
    Parse(String),

    /// Invalid file path
    #[error("Invalid file path: {0}")]
    InvalidPath(String),
}

/// LAS file parser with configurable curve type detection.
///
/// The parser uses a [`CurveMappingDictionary`] to identify and classify
/// curve mnemonics found in LAS files.
///
/// # Example
///
/// ```no_run
/// use las_parser::LasParser;
///
/// let parser = LasParser::new();
/// let las_file = parser.parse_las_file("well_log.las")?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub struct LasParser {
    /// User-configurable curve mapping dictionary
    curve_mapping_dict: CurveMappingDictionary,
}

impl Default for LasParser {
    fn default() -> Self {
        Self::new()
    }
}

impl LasParser {
    /// Create a new parser with default curve mappings.
    pub fn new() -> Self {
        LasParser {
            curve_mapping_dict: CurveMappingDictionary::default(),
        }
    }

    /// Create a parser with a custom curve mapping dictionary.
    pub fn new_with_mappings(mappings: CurveMappingDictionary) -> Self {
        LasParser {
            curve_mapping_dict: mappings,
        }
    }

    /// Update the curve mapping dictionary.
    pub fn update_mappings(&mut self, mappings: CurveMappingDictionary) {
        self.curve_mapping_dict = mappings;
    }

    /// Get a reference to the current curve mapping dictionary.
    pub fn get_mappings(&self) -> &CurveMappingDictionary {
        &self.curve_mapping_dict
    }

    /// Parse a LAS file and return the parsed structure.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the LAS file
    ///
    /// # Returns
    ///
    /// The parsed [`LASFile`] structure or an error.
    pub fn parse_las_file<P: AsRef<std::path::Path>>(&self, path: P) -> LasResult<LASFile> {
        let path_ref = path.as_ref();
        debug!(path = ?path_ref, "Parsing LAS file");

        let las_file = LASFile::read(path_ref).map_err(|e| LasParseError::Io(e))?;

        info!(
            path = ?path_ref,
            curves = las_file.curves.columns.len(),
            "Successfully parsed LAS file"
        );
        Ok(las_file)
    }

    /// Parse multiple LAS files, collecting results and skipping failures.
    ///
    /// Files that fail to parse are logged as warnings but don't stop processing.
    ///
    /// # Arguments
    ///
    /// * `paths` - Slice of paths to LAS files
    ///
    /// # Returns
    ///
    /// Vector of (filename, parsed file) tuples for successfully parsed files.
    pub fn parse_multiple_las_files<P: AsRef<std::path::Path>>(
        &self,
        paths: &[P],
    ) -> LasResult<Vec<(String, LASFile)>> {
        debug!(count = paths.len(), "Parsing multiple LAS files");
        let mut results = Vec::new();

        for path in paths {
            let file_name = path
                .as_ref()
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown")
                .to_string();

            match self.parse_las_file(path) {
                Ok(las_file) => {
                    results.push((file_name.clone(), las_file));
                    debug!(file = %file_name, "Parsed successfully");
                }
                Err(e) => {
                    warn!(file = %file_name, error = %e, "Failed to parse LAS file");
                }
            }
        }

        info!(
            successful = results.len(),
            total = paths.len(),
            "Completed parsing multiple LAS files"
        );
        Ok(results)
    }

    /// Process a LAS file for user confirmation with curve detection.
    ///
    /// This analyzes the curves in the file and attempts to match them
    /// to known curve types using the mapping dictionary.
    ///
    /// # Arguments
    ///
    /// * `las_file` - The parsed LAS file
    /// * `well_name` - Name to use for the well in metadata
    ///
    /// # Returns
    ///
    /// Processing result including detected curves and any that need user mapping.
    pub fn process_las_file_for_confirmation(
        &self,
        las_file: &LASFile,
        well_name: &str,
    ) -> LasResult<LasProcessingResult> {
        let mut curve_detections = Vec::new();
        let mut unmapped_curves = Vec::new();

        for column in &las_file.curves.columns {
            let detected_main_type = self
                .curve_mapping_dict
                .get_main_curve_type(&column.mnemonic);
            let is_mapped = self.curve_mapping_dict.is_mapped(&column.mnemonic);

            if !is_mapped {
                unmapped_curves.push(column.mnemonic.clone());
            }

            let suggested_subcurve = if is_mapped {
                self.curve_mapping_dict
                    .get_subcurve_mapping(&column.mnemonic)
                    .map(|s| s.subcurve_name.clone())
            } else {
                None
            };

            let vendor_variations = if is_mapped {
                self.curve_mapping_dict
                    .get_subcurve_mapping(&column.mnemonic)
                    .map(|s| s.vendor_variations.clone())
                    .unwrap_or_default()
            } else {
                vec![column.mnemonic.clone()]
            };

            let detection = CurveDetection {
                raw_name: column.mnemonic.clone(),
                detected_main_type: if is_mapped {
                    Some(detected_main_type)
                } else {
                    None
                },
                suggested_subcurve,
                units: column.unit.clone(),
                confidence: if is_mapped { 0.9 } else { 0.0 },
                vendor_variations,
                is_mapped,
            };
            curve_detections.push(detection);
        }

        Ok(LasProcessingResult {
            well_metadata: self.extract_well_metadata(las_file, well_name),
            curve_detections,
            requires_user_confirmation: !unmapped_curves.is_empty(),
            unmapped_curves,
        })
    }

    /// Extract well metadata from a LAS file.
    fn extract_well_metadata(&self, las_file: &LASFile, well_name: &str) -> WellMetadata {
        WellMetadata {
            well_name: well_name.to_string(),
            field: las_file.well.get_field().unwrap_or_default().to_string(),
            company: las_file.well.get_company().unwrap_or_default().to_string(),
            location: las_file.well.get_location().unwrap_or_default().to_string(),
            uwi: las_file.well.get_uwi().unwrap_or_default().to_string(),
            start_depth: las_file.well.get_start_depth(),
            stop_depth: las_file.well.get_stop_depth(),
            step: las_file.well.get_step(),
            null_value: las_file.well.get_null_value(),
            province: las_file.well.get_province().map(|s| s.to_string()),
            service_company: las_file.well.get_service_company().map(|s| s.to_string()),
            log_date: las_file.well.get_log_date().map(|s| s.to_string()),
        }
    }

    /// Add a curve mapping to the dictionary.
    pub fn add_curve_mapping(&mut self, curve_name: String, main_type: MainCurveType) {
        self.curve_mapping_dict.add_mapping(curve_name, main_type);
    }

    /// Remove a curve mapping from the dictionary.
    pub fn remove_curve_mapping(&mut self, curve_name: &str) {
        self.curve_mapping_dict.remove_mapping(curve_name);
    }

    /// Get statistics about curve mappings for a LAS file.
    ///
    /// # Returns
    ///
    /// Tuple of (mapped_count, total_count, unmapped_curve_names)
    pub fn get_mapping_stats(&self, las_file: &LASFile) -> (usize, usize, Vec<String>) {
        let total_curves = las_file.curves.columns.len();
        let mapped_curves = las_file
            .curves
            .columns
            .iter()
            .filter(|col| self.curve_mapping_dict.is_mapped(&col.mnemonic))
            .count();
        let unmapped_curves = las_file
            .curves
            .columns
            .iter()
            .filter(|col| !self.curve_mapping_dict.is_mapped(&col.mnemonic))
            .map(|col| col.mnemonic.clone())
            .collect();

        (mapped_curves, total_curves, unmapped_curves)
    }
}
