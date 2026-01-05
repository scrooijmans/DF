//! Curve section parsing for LAS files.
//!
//! Handles both curve definitions (~C section) and curve data (~A section).

use super::HeaderItem;
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::trace;

lazy_static! {
    // Pattern for curve section lines like:
    // "DEPT.M                          :DEPTH"
    // "GR  .GAPI                       :GAMMA RAY"
    static ref HEADER_PATTERN: Regex = Regex::new(
        r"^([^.]+)\s*\.\s*([^:]*?)\s*:(.*)$"
    ).unwrap();

    // More flexible regex inspired by lasio's approach
    static ref FLEXIBLE_CURVE_HEADER_REGEX: Regex = Regex::new(
        r"^\s*([^\s.]+)\s*\.\s*([^\s:]+)?\s*([0-9 ]*)?:\s*(.*)$"
    ).unwrap();

    // LAS 3.0 specific regex for format specifications and associations
    static ref LAS3_CURVE_HEADER_REGEX: Regex = Regex::new(
        r"^\s*([^\s.]+)\s*\.\s*([^\s:]+)?\s*([0-9 ]*)?:\s*([^|]*?)(?:\s*\{([^}]*)\})?\s*(?:\|\s*(.*))?$"
    ).unwrap();
}

/// A column of curve data from a LAS file.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CurveDataColumn {
    /// Curve mnemonic (e.g., "GR", "RHOB")
    pub mnemonic: String,
    /// Unit of measurement (e.g., "gAPI", "g/cm3")
    pub unit: String,
    /// Description of the curve
    pub description: String,
    /// Numeric values for this curve
    pub values: Vec<f64>,
    /// LAS 3.0 format specification (e.g., "{F10.4}")
    pub format: Option<String>,
    /// LAS 3.0 associations with other curves
    pub associations: Vec<String>,
}

/// The curve section of a LAS file.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CurveSection {
    /// Header items from the ~C section
    pub items: HashMap<String, HeaderItem>,
    /// Data columns parsed from ~A section
    pub columns: Vec<CurveDataColumn>,
    /// Maps uppercase mnemonic to column index
    #[serde(skip)]
    mnemonic_map: HashMap<String, usize>,
}

impl Default for CurveSection {
    fn default() -> Self {
        Self::new()
    }
}

impl CurveSection {
    /// Create a new empty curve section.
    pub fn new() -> Self {
        CurveSection {
            items: HashMap::new(),
            columns: Vec::new(),
            mnemonic_map: HashMap::new(),
        }
    }

    /// Parse a curve header line using standard mode.
    pub fn parse_header(&mut self, line: &str) -> Result<(), String> {
        self.parse_header_with_mode(line, CurveHeaderParseMode::Standard)
    }

    /// Parse a curve header line with a specific parsing mode.
    pub fn parse_header_with_mode(
        &mut self,
        line: &str,
        mode: CurveHeaderParseMode,
    ) -> Result<(), String> {
        if line.trim().is_empty() || line.trim().starts_with('#') {
            return Ok(());
        }
        let line = line.trim();

        match mode {
            CurveHeaderParseMode::Standard => self.parse_header_standard(line),
            CurveHeaderParseMode::Legacy => self.parse_header_legacy(line),
        }
    }

    fn parse_header_standard(&mut self, line: &str) -> Result<(), String> {
        // Try LAS 3.0 regex first for format and association support
        if let Some(caps) = LAS3_CURVE_HEADER_REGEX.captures(line) {
            let mnemonic = caps.get(1).map_or("", |m| m.as_str()).trim();
            let unit = caps.get(2).map_or("", |m| m.as_str()).trim();
            let description = caps.get(4).map_or("", |m| m.as_str()).trim();
            let format = caps.get(5).map(|m| m.as_str().trim().to_string());
            let associations = caps
                .get(6)
                .map(|m| {
                    m.as_str()
                        .trim()
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .collect()
                })
                .unwrap_or_default();

            self.add_column(mnemonic, unit, description, format, associations);
            return Ok(());
        }

        // Try flexible regex
        if let Some(caps) = FLEXIBLE_CURVE_HEADER_REGEX.captures(line) {
            let mnemonic = caps.get(1).map_or("", |m| m.as_str()).trim();
            let unit = caps.get(2).map_or("", |m| m.as_str()).trim();
            let description = caps.get(4).map_or("", |m| m.as_str()).trim();

            self.add_column(mnemonic, unit, description, None, Vec::new());
            return Ok(());
        }

        // Fallback to original pattern
        if let Some(caps) = HEADER_PATTERN.captures(line) {
            let mnemonic = caps.get(1).map_or("", |m| m.as_str()).trim();
            let unit = caps.get(2).map_or("", |m| m.as_str()).trim();
            let description = caps.get(3).map_or("", |m| m.as_str()).trim();

            self.add_column(mnemonic, unit, description, None, Vec::new());
            return Ok(());
        }

        // Log and continue rather than failing
        trace!(line = %line, "Failed to parse curve header line");
        Ok(())
    }

    fn parse_header_legacy(&mut self, line: &str) -> Result<(), String> {
        let patterns = vec![
            r"^\s*([^\s.]+)\s*\.\s*([^\s:]+)?\s*:\s*(.*)$",
            r"^\s*([^\s.]+)\s*\.\s*([^\s]*?)\s*:\s*(.*)$",
            r"^\s*([^\s:]+)\s*:\s*(.*)$",
        ];

        for pattern_str in patterns {
            if let Ok(pattern) = Regex::new(pattern_str) {
                if let Some(caps) = pattern.captures(line) {
                    let mnemonic = caps.get(1).map_or("", |m| m.as_str()).trim();
                    let unit = caps.get(2).map_or("", |m| m.as_str()).trim();
                    let description = caps.get(3).map_or("", |m| m.as_str()).trim();

                    self.add_column(mnemonic, unit, description, None, Vec::new());
                    return Ok(());
                }
            }
        }

        trace!(line = %line, "Failed to parse curve header line in legacy mode");
        Ok(())
    }

    fn add_column(
        &mut self,
        mnemonic: &str,
        unit: &str,
        description: &str,
        format: Option<String>,
        associations: Vec<String>,
    ) {
        let col = CurveDataColumn {
            mnemonic: mnemonic.to_string(),
            unit: unit.to_string(),
            description: description.to_string(),
            values: Vec::new(),
            format,
            associations,
        };

        // Handle duplicate mnemonics by appending counter
        let mut final_mnemonic = mnemonic.to_uppercase();
        let mut counter = 1;
        while self.mnemonic_map.contains_key(&final_mnemonic) {
            final_mnemonic = format!("{}:{}", mnemonic.to_uppercase(), counter);
            counter += 1;
        }

        self.mnemonic_map.insert(final_mnemonic, self.columns.len());
        self.columns.push(col);
    }

    /// Parse a line of data values.
    pub fn parse_data_line(
        &mut self,
        line: &str,
        null_value: f64,
        delimiter: char,
    ) -> Result<(), String> {
        if line.trim().is_empty() || line.trim().starts_with('#') {
            return Ok(());
        }

        // Parse based on delimiter
        let values: Vec<&str> = if delimiter == ',' {
            line.split(',').collect()
        } else if delimiter == '\t' {
            line.split('\t').collect()
        } else {
            line.split_whitespace().collect()
        };

        // Handle mismatched value counts gracefully
        if values.len() != self.columns.len() {
            trace!(
                values = values.len(),
                columns = self.columns.len(),
                "Data line value count mismatch"
            );

            let mut padded_values = values.clone();
            while padded_values.len() < self.columns.len() {
                padded_values.push("-999.25");
            }
            if padded_values.len() > self.columns.len() {
                padded_values.truncate(self.columns.len());
            }

            for (i, value_str) in padded_values.iter().enumerate() {
                let value = self.parse_value_for_column(i, value_str, null_value);
                if i < self.columns.len() {
                    self.columns[i].values.push(value);
                }
            }
        } else {
            for (i, value_str) in values.iter().enumerate() {
                let value = self.parse_value_for_column(i, value_str, null_value);
                self.columns[i].values.push(value);
            }
        }
        Ok(())
    }

    fn parse_value_for_column(
        &self,
        column_index: usize,
        value_str: &str,
        _null_value: f64,
    ) -> f64 {
        if column_index >= self.columns.len() {
            return f64::NAN;
        }

        let column = &self.columns[column_index];
        let trimmed_value = value_str.trim();

        // Check if this is a string field based on format specification
        if let Some(format_spec) = &column.format {
            if format_spec.contains('S') {
                return f64::NAN;
            }
        }

        match trimmed_value.parse::<f64>() {
            Ok(v) => {
                if trimmed_value.is_empty() {
                    f64::NAN
                } else {
                    v
                }
            }
            Err(_) => {
                if !trimmed_value.is_empty() {
                    trace!(
                        column = %column.mnemonic,
                        value = %trimmed_value,
                        "Invalid numeric value"
                    );
                }
                f64::NAN
            }
        }
    }

    /// Parse a data line using space delimiter (legacy compatibility).
    pub fn parse_data_line_legacy(&mut self, line: &str, null_value: f64) -> Result<(), String> {
        self.parse_data_line(line, null_value, ' ')
    }

    /// Get curve data by mnemonic.
    pub fn get_curve_data(&self, mnemonic: &str) -> Option<&Vec<f64>> {
        self.mnemonic_map
            .get(&mnemonic.to_uppercase())
            .and_then(|&idx| self.columns.get(idx))
            .map(|col| &col.values)
    }

    /// Get a curve column by mnemonic.
    pub fn get_column(&self, mnemonic: &str) -> Option<&CurveDataColumn> {
        self.mnemonic_map
            .get(&mnemonic.to_uppercase())
            .and_then(|&idx| self.columns.get(idx))
    }

    /// Get depth data, trying common depth mnemonics.
    pub fn get_depth_data(&self) -> Option<&Vec<f64>> {
        for mnemonic in &["DEPT", "DEPTH", "MD", "TVD", "DEPTH M"] {
            if let Some(data) = self.get_curve_data(mnemonic) {
                return Some(data);
            }
        }
        None
    }

    /// Get gamma ray data.
    pub fn get_gr_data(&self) -> Option<&Vec<f64>> {
        self.get_curve_data("GR")
    }
}

/// Parsing mode for curve headers.
#[derive(Debug, Clone, Copy)]
pub enum CurveHeaderParseMode {
    /// Standard parsing (LAS 2.0/3.0 compatible)
    Standard,
    /// Legacy parsing with fallback patterns
    Legacy,
}

/// Metadata about a curve type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurveMetadata {
    /// Unique curve identifier
    pub curve_id: String,
    /// Curve name/mnemonic
    pub curve_name: String,
    /// Unit of measurement
    pub unit: Option<String>,
    /// Description of the curve
    pub description: Option<String>,
}

/// Build a HashMap for fast lookup by mnemonic (curve_name).
pub fn build_curve_metadata_map(curves: &[CurveMetadata]) -> HashMap<String, CurveMetadata> {
    let mut map = HashMap::new();
    for curve in curves {
        map.insert(curve.curve_name.to_uppercase(), curve.clone());
    }
    map
}
