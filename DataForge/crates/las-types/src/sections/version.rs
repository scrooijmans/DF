use super::HeaderItem;
use regex::Regex;
use lazy_static::lazy_static;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

lazy_static! {
    // Pattern for version section lines like:
    // "VERS.          2.0  :CWLS LOG ASCII STANDARD - VERSION 2.0"
    // "VERS.          1.2  :CWLS LOG ASCII STANDARD - VERSION 1.2"
    // "WRAP.          NO   :ONE LINE PER DEPTH STEP"
    // "DLM .        COMMA :DELIMITING CHARACTER BETWEEN DATA COLUMNS"
    // Need to be more flexible with whitespace
    static ref HEADER_PATTERN: Regex = Regex::new(
        r"^([^.]+)\s*\.\s*([^:]*?)\s*([^:\s]+)\s*:(.*)$"
    ).unwrap();
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VersionSection {
    pub items: HashMap<String, HeaderItem>,
}

impl VersionSection {
    pub fn new() -> Self {
        VersionSection {
            items: HashMap::new(),
        }
    }

    pub fn parse_line(&mut self, line: &str) -> Result<(), String> {
        // Skip blank lines and comments
        if line.trim().is_empty() || line.trim().starts_with('#') {
            return Ok(());
        }

        // Try to match the line against our regex pattern
        if let Some(captures) = HEADER_PATTERN.captures(line.trim()) {
            let mnemonic = captures.get(1).unwrap().as_str().trim();
            let unit = captures.get(2).unwrap().as_str().trim();
            let raw_value = captures.get(3).unwrap().as_str().trim();
            let description = captures.get(4).unwrap().as_str().trim();

            // Extract the appropriate value based on the mnemonic
            let value = match mnemonic {
                "VERS" => {
                    // For VERS, we want the version number (1.2, 2.0, 3.0, etc.)
                    // First try to find version number in the raw_value field
                    if let Some(cap) = Regex::new(r"\d+\.\d+").unwrap().find(raw_value) {
                        cap.as_str()
                    } else if let Some(cap) = Regex::new(r"\d+").unwrap().find(raw_value) {
                        // If no decimal, just use the raw value as-is
                        cap.as_str()
                    } else {
                        // Fallback: try to find version in description
                        if let Some(cap) = Regex::new(r"\d+\.\d+").unwrap().find(description) {
                            cap.as_str()
                        } else if let Some(cap) = Regex::new(r"\d+").unwrap().find(description) {
                            cap.as_str()
                        } else {
                            raw_value
                        }
                    }
                },
                "WRAP" => {
                    // For WRAP, we want YES or NO
                    if raw_value.to_uppercase().contains("NO") {
                        "NO"
                    } else if raw_value.to_uppercase().contains("YES") {
                        "YES"
                    } else {
                        raw_value
                    }
                },
                "DLM" => {
                    // For DLM, we want SPACE, COMMA, or TAB
                    let upper_value = raw_value.to_uppercase();
                    if upper_value.contains("SPACE") {
                        "SPACE"
                    } else if upper_value.contains("COMMA") {
                        "COMMA"
                    } else if upper_value.contains("TAB") {
                        "TAB"
                    } else {
                        raw_value
                    }
                },
                _ => raw_value
            };

            let header_item = HeaderItem::new(mnemonic, unit, value, description);
            self.items.insert(mnemonic.to_string(), header_item);
            Ok(())
        } else {
            Err(format!("Invalid version section line format: {}", line))
        }
    }

    // Helper methods for common version information
    pub fn get_version(&self) -> Option<&str> {
        self.items.get("VERS").map(|item| item.value.as_str())
    }

    pub fn get_wrap(&self) -> Option<&str> {
        self.items.get("WRAP").map(|item| item.value.as_str())
    }

    pub fn get_delimiter(&self) -> Option<&str> {
        self.items.get("DLM").map(|item| item.value.as_str())
    }

    pub fn is_wrapped(&self) -> bool {
        self.get_wrap()
            .map(|wrap| wrap.trim().to_uppercase() == "YES")
            .unwrap_or(false)
    }

    // Helper method to get the version as a float for comparison
    pub fn get_version_number(&self) -> Option<f32> {
        self.get_version().and_then(|v| v.parse::<f32>().ok())
    }

    // Helper method to check if this is LAS 3.0
    pub fn is_las_3_0(&self) -> bool {
        self.get_version_number()
            .map(|v| v >= 3.0)
            .unwrap_or(false)
    }

    // Helper method to get the delimiter character
    pub fn get_delimiter_char(&self) -> char {
        match self.get_delimiter() {
            Some("COMMA") => ',',
            Some("TAB") => '\t',
            Some("SPACE") | None => ' ', // Default to space
            _ => ' ', // Default to space for unknown delimiters
        }
    }
} 