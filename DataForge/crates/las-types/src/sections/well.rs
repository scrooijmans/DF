// Import types from parent module (mod.rs)
// super refers to the parent module (like .. in file paths)
use super::HeaderItem;
// Import Regex for pattern matching LAS file lines
use regex::Regex;
// Import lazy_static to compile regex pattern once at program start
use lazy_static::lazy_static;
use std::collections::HashMap;
use thiserror::Error;
use serde::{Serialize, Deserialize};

// lazy_static ensures the regex is compiled once and reused
// This is more efficient than compiling the regex for each line
lazy_static! {
    // Pattern for well section lines like:
    // "WELL.          1-28  :WELL NAME"
    // "LOC .      C,NW,NE, 28-30S-39W:LOCATION"
    // "STRT.M        1783.5000:START DEPTH"
    // "STOP.M        1784.5000:STOP DEPTH"
    // "STEP.M           0.2500:STEP SIZE"
    // Need to be more flexible with whitespace
    static ref HEADER_PATTERN: Regex = Regex::new(
        r"^([^.]+)\s*\.\s*([^:]*?)\s*([^:\s]+)\s*:(.*)$"
    ).unwrap();
}

// WellSection struct holds all the header items for the well section
// #[derive(Debug)] auto-implements debug printing for this struct
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WellSection {
    pub items: HashMap<String, HeaderItem>,
    pub version: Option<f32>, // Store detected version for version-aware parsing
}

#[derive(Debug, Serialize)]
pub struct WellEntry {
    pub mnemonic: String,
    pub unit: String,
    pub value: String,
    pub description: String,
}

#[derive(Error, Debug)]
pub enum WellSectionError {
    #[error("Invalid line format")]
    InvalidFormat,
}

// Implementation block for WellSection - contains all methods
impl WellSection {
    // Constructor - creates new WellSection with empty vector
    // -> Self means it returns a WellSection
    pub fn new() -> Self {
        WellSection {
            items: HashMap::new(),
            version: None,
        }
    }

    // Set the version for parsing logic
    pub fn set_version(&mut self, version: f32) {
        self.version = Some(version);
    }

    // Parses a single line from the well section of a LAS file
    // &mut self means this method can modify the WellSection
    // Returns Result - Ok(()) for success, Err(message) for parsing failures
    pub fn parse_line(&mut self, line: &str) -> Result<(), WellSectionError> {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            return Ok(());
        }

        // Find the first dot (mnemonic/unit delimiter)
        let dot_pos = line.find('.').ok_or(WellSectionError::InvalidFormat)?;
        let mnemonic = line[..dot_pos].trim();
        let rest = &line[dot_pos + 1..];

        // Find the first space after the dot (unit/value delimiter)
        let space_pos = rest.find(char::is_whitespace).ok_or(WellSectionError::InvalidFormat)?;
        let unit = rest[..space_pos].trim();
        let rest = rest[space_pos..].trim_start();

        // Find the last colon (value/description delimiter)
        let colon_pos = rest.rfind(':').ok_or(WellSectionError::InvalidFormat)?;
        let value = rest[..colon_pos].trim();
        let description = rest[colon_pos + 1..].trim();

        // Handle different field orders based on version and mnemonic
        let (final_value, final_description) = if let Some(version) = self.version {
            if version < 2.0 {
                // LAS 1.2: Use specific field order based on mnemonic
                // According to LAS 1.2 standard and lasio's implementation:
                // - STRT, STOP, STEP, NULL use "value:descr" (value first, then description)
                // - Most other fields use "descr:value" (description first, then value)
                match mnemonic {
                    "STRT" | "STOP" | "STEP" | "NULL" => {
                        // These fields use "value:descr" order
                        (value, description)
                    },
                    _ => {
                        // For other fields in LAS 1.2, use "descr:value" order
                        // The format is: MNEM.UNIT DESCRIPTION: VALUE
                        // So we need to swap value and description
                        (description, value)
                    }
                }
            } else {
                // LAS 2.0: All fields use "value:descr" order
                (value, description)
            }
        } else {
            // Default to LAS 2.0 behavior if version not set
            (value, description)
        };

        self.items.insert(mnemonic.to_string(), HeaderItem {
            mnemonic: mnemonic.to_string(),
            unit: unit.to_string(),
            value: final_value.to_string(),
            description: final_description.to_string(),
        });
        Ok(())
    }

    // Helper method to get a value by its mnemonic
    // Returns Option - Some(value) if found, None if not found
    // &self means this is a read-only method
    pub fn get_value(&self, mnemonic: &str) -> Option<&str> {
        self.items.get(mnemonic).map(|item| item.value.as_str())
    }
    
    /// Get start depth as f64
    pub fn get_start_depth(&self) -> f64 {
        self.get_value("STRT")
            .and_then(|v| v.parse().ok())
            .unwrap_or(0.0)
    }
    
    /// Get stop depth as f64
    pub fn get_stop_depth(&self) -> f64 {
        self.get_value("STOP")
            .and_then(|v| v.parse().ok())
            .unwrap_or(0.0)
    }
    
    /// Get step as f64
    pub fn get_step(&self) -> f64 {
        self.get_value("STEP")
            .and_then(|v| v.parse().ok())
            .unwrap_or(0.1524)
    }
    
    /// Get null value as f64
    pub fn get_null_value(&self) -> f64 {
        self.get_value("NULL")
            .and_then(|v| v.parse().ok())
            .unwrap_or(-999.25)
    }
    
    /// Get well name
    pub fn get_well_name(&self) -> Option<&str> {
        self.get_value("WELL")
    }
    
    /// Get field name
    pub fn get_field(&self) -> Option<&str> {
        self.get_value("FLD")
    }
    
    /// Get company name
    pub fn get_company(&self) -> Option<&str> {
        self.get_value("COMP")
    }
    
    /// Get location
    pub fn get_location(&self) -> Option<&str> {
        self.get_value("LOC")
    }
    
    /// Get UWI
    pub fn get_uwi(&self) -> Option<&str> {
        self.get_value("UWI")
    }
    
    /// Get province
    pub fn get_province(&self) -> Option<&str> {
        self.get_value("PROV")
    }
    
    /// Get service company
    pub fn get_service_company(&self) -> Option<&str> {
        self.get_value("SRVC")
    }
    
    /// Get log date
    pub fn get_log_date(&self) -> Option<&str> {
        self.get_value("DATE")
    }
} 