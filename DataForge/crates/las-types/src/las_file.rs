//! LAS file structure and parsing logic.

use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

use serde::{Deserialize, Serialize};
use tracing::trace;

use crate::sections::{
    curve::CurveSection, parameter::ParameterSection, version::VersionSection, well::WellSection,
};

/// Represents a parsed LAS (Log ASCII Standard) file.
///
/// Contains all sections of a LAS file including version information,
/// well metadata, parameters, and curve data.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LASFile {
    /// Version section (~V)
    pub version: VersionSection,
    /// Well information section (~W)
    pub well: WellSection,
    /// Parameter section (~P)
    pub params: ParameterSection,
    /// Curve data section (~C and ~A)
    pub curves: CurveSection,
    /// Whether the file uses wrapped data format
    pub is_wrapped: bool,
}

impl Default for LASFile {
    fn default() -> Self {
        Self::new()
    }
}

impl LASFile {
    /// Create a new empty LAS file structure.
    pub fn new() -> Self {
        LASFile {
            version: VersionSection::new(),
            well: WellSection::new(),
            params: ParameterSection::new(),
            curves: CurveSection::new(),
            is_wrapped: false,
        }
    }

    /// Read and parse a LAS file from disk.
    ///
    /// Handles encoding issues by converting to UTF-8 lossily.
    /// Supports LAS versions 1.2, 2.0, and 3.0.
    pub fn read<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let mut las = LASFile::new();

        // Read file as bytes first to handle encoding issues
        let mut file = File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        // Convert to UTF-8 lossily, replacing invalid characters
        let content = String::from_utf8_lossy(&buffer);

        let mut current_section = None;
        let mut reading_data = false;
        let mut null_value = -999.25; // Default null value
        let mut all_data_values: Vec<f64> = Vec::new(); // For wrapped mode
        let mut version_detected = false;
        let mut delimiter = ' '; // Default delimiter

        for line in content.lines() {
            let trimmed = line.trim();

            // Skip empty lines
            if trimmed.is_empty() {
                continue;
            }

            // Check for section markers
            if trimmed.starts_with('~') {
                let section_line = trimmed[1..].trim();

                // Handle LAS 3.0 section names
                if section_line.starts_with("Log_Data") || section_line.starts_with("ASCII") {
                    current_section = Some('A');
                    reading_data = true;
                } else if section_line.starts_with("Log_Definition") {
                    current_section = Some('C');
                    reading_data = false;
                } else if section_line.starts_with("Log_Parameter")
                    || section_line.starts_with("Parameter")
                {
                    current_section = Some('P');
                    reading_data = false;
                } else if section_line.starts_with("Drilling_Definition")
                    || section_line.starts_with("Core_Definition")
                    || section_line.starts_with("Inclinometry_Definition")
                    || section_line.starts_with("Test_Definition")
                    || section_line.starts_with("TOPS_Definition")
                    || section_line.starts_with("Perforations_Definition")
                {
                    // Skip these LAS 3.0 specific sections
                    current_section = None;
                    reading_data = false;
                } else {
                    // Handle legacy section names
                    let first_char = section_line.chars().next().unwrap_or(' ');
                    current_section = Some(first_char);
                    reading_data = first_char == 'A';
                }
                continue;
            }

            // Parse section content
            if let Some(section) = current_section {
                match (section, reading_data) {
                    ('V', _) => {
                        if let Err(e) = las.version.parse_line(line) {
                            trace!(error = %e, line = %line, "Failed to parse version section line");
                        }

                        las.is_wrapped = las.version.is_wrapped();
                        delimiter = las.version.get_delimiter_char();

                        if !version_detected {
                            if let Some(version_num) = las.version.get_version_number() {
                                las.well.set_version(version_num);
                                version_detected = true;
                            }
                        }

                        // Update null value if we find it in version section
                        if line.contains("NULL") {
                            if let Some(null_str) = las.well.get_value("NULL") {
                                if let Ok(val) = null_str.parse::<f64>() {
                                    null_value = val;
                                }
                            }
                        }
                    }
                    ('W', _) => {
                        if let Err(e) = las.well.parse_line(line) {
                            trace!(error = %e, line = %line, "Failed to parse well section line");
                        }
                        // Update null value if we find it in well section
                        if line.contains("NULL") {
                            if let Some(null_str) = las.well.get_value("NULL") {
                                if let Ok(val) = null_str.parse::<f64>() {
                                    null_value = val;
                                }
                            }
                        }
                    }
                    ('P', _) => {
                        if let Err(e) = las.params.parse_line(line) {
                            trace!(error = %e, line = %line, "Failed to parse parameter section line");
                        }
                    }
                    ('C', false) => {
                        // Try standard mode first, then legacy mode
                        if las
                            .curves
                            .parse_header_with_mode(
                                line,
                                crate::sections::curve::CurveHeaderParseMode::Standard,
                            )
                            .is_err()
                        {
                            if let Err(e) = las.curves.parse_header_with_mode(
                                line,
                                crate::sections::curve::CurveHeaderParseMode::Legacy,
                            ) {
                                trace!(error = %e, line = %line, "Failed to parse curve header line");
                            }
                        }
                    }
                    ('A', true) => {
                        if las.is_wrapped {
                            // For wrapped mode, accumulate all values
                            let values: Vec<&str> = if delimiter == ',' {
                                line.split(',').collect()
                            } else if delimiter == '\t' {
                                line.split('\t').collect()
                            } else {
                                line.split_whitespace().collect()
                            };

                            for value_str in values {
                                let value = match value_str.trim().parse::<f64>() {
                                    Ok(v) => {
                                        if value_str.trim().is_empty() {
                                            f64::NAN
                                        } else {
                                            v
                                        }
                                    }
                                    Err(_) => {
                                        if !value_str.trim().is_empty() {
                                            trace!(value = %value_str, "Invalid numeric value in wrapped data");
                                        }
                                        f64::NAN
                                    }
                                };
                                all_data_values.push(value);
                            }
                        } else {
                            // Unwrapped mode: parse line by line
                            if let Err(e) = las.curves.parse_data_line(line, null_value, delimiter)
                            {
                                trace!(error = %e, "Failed to parse data line");
                            }
                        }
                    }
                    _ => {
                        // Skip other sections (like ~O for Other)
                    }
                }
            }
        }

        // Post-processing for wrapped mode
        if las.is_wrapped && !all_data_values.is_empty() {
            let num_curves = las.curves.columns.len();
            if num_curves > 0 {
                let num_data_points = all_data_values.len() / num_curves;

                for (curve_idx, curve) in las.curves.columns.iter_mut().enumerate() {
                    curve.values.clear();
                    for data_idx in 0..num_data_points {
                        let value_idx = data_idx * num_curves + curve_idx;
                        if value_idx < all_data_values.len() {
                            curve.values.push(all_data_values[value_idx]);
                        }
                    }
                }
            }
        }

        Ok(las)
    }

    /// Get the detected LAS version number (e.g., 1.2, 2.0, 3.0).
    pub fn get_detected_version(&self) -> Option<f32> {
        self.version.get_version_number()
    }

    /// Check if this is a LAS 1.2 format file.
    pub fn is_las_1_2(&self) -> bool {
        self.get_detected_version()
            .map(|v| v < 2.0)
            .unwrap_or(false)
    }

    /// Check if this is a LAS 2.0 format file.
    pub fn is_las_2_0(&self) -> bool {
        self.get_detected_version()
            .map(|v| v >= 2.0 && v < 3.0)
            .unwrap_or(false)
    }

    /// Check if this is a LAS 3.0 format file.
    pub fn is_las_3_0(&self) -> bool {
        self.version.is_las_3_0()
    }
}
