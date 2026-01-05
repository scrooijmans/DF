// Parameter section implementation will go here
use super::HeaderItem;
use regex::Regex;
use lazy_static::lazy_static;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

lazy_static! {
    // Pattern for parameter section lines like:
    // "BS.                            216 mm                   :BS"
    // "PURP.                          Cased hole stratigraphy  :PURP"
    // "X.                             0560160                  :X"
    static ref HEADER_PATTERN: Regex = Regex::new(
        r"^([^.]+)\s*\.\s*([^:]*?)(?:\s*([^:]+))?\s*:(.*)$"
    ).unwrap();
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ParameterSection {
    pub items: HashMap<String, HeaderItem>,
    // Case-insensitive lookup map
    #[serde(skip)]
    mnemonic_map: HashMap<String, String>,
}

impl ParameterSection {
    pub fn new() -> Self {
        ParameterSection {
            items: HashMap::new(),
            mnemonic_map: HashMap::new(),
        }
    }

    pub fn parse_line(&mut self, line: &str) -> Result<(), String> {
        // Skip blank lines and comments
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            return Ok(());
        }

        // Find the first dot (mnemonic/unit delimiter)
        let dot_pos = line.find('.').ok_or("Invalid parameter section line format: missing dot")?;
        let mnemonic = line[..dot_pos].trim();
        let rest = &line[dot_pos + 1..];

        // Find the first space after the dot (unit/value delimiter)
        let space_pos = rest.find(char::is_whitespace).ok_or("Invalid parameter section line format: missing space after unit")?;
        let unit = rest[..space_pos].trim();
        let rest = rest[space_pos..].trim_start();

        // Find the last colon (value/description delimiter)
        let colon_pos = rest.rfind(':').ok_or("Invalid parameter section line format: missing colon")?;
        let mut value = rest[..colon_pos].trim().to_string();
        let description = rest[colon_pos + 1..].trim();

        // Special handling for coordinate values: strip leading zeros
        if mnemonic == "X" || mnemonic == "Y" {
            value = value.trim_start_matches('0').to_string();
            if value.is_empty() { value = "0".to_string(); }
        }

        // Store both original and uppercase versions for case-insensitive lookup
        self.mnemonic_map.insert(mnemonic.to_uppercase(), mnemonic.to_string());
        let header_item = HeaderItem::new(mnemonic, unit, &value, description);
        self.items.insert(mnemonic.to_string(), header_item);
        Ok(())
    }

    // Helper methods for common parameters
    pub fn get_value(&self, mnemonic: &str) -> Option<&str> {
        // Try case-sensitive lookup first
        if let Some(item) = self.items.get(mnemonic) {
            return Some(item.value.as_str());
        }
        // Try case-insensitive lookup
        self.mnemonic_map.get(&mnemonic.to_uppercase())
            .and_then(|m| self.items.get(m))
            .map(|item| item.value.as_str())
    }

    // Common parameter getters
    pub fn get_borehole_size(&self) -> Option<&str> {
        self.get_value("BS")
    }

    pub fn get_mud_type(&self) -> Option<&str> {
        self.get_value("MUD")
    }

    pub fn get_depth_reference(&self) -> Option<&str> {
        self.get_value("DREF")
    }

    pub fn get_fluid_level(&self) -> Option<&str> {
        self.get_value("FLUIDLEVEL")
    }
} 