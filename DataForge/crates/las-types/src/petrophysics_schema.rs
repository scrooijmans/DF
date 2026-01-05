use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fmt;

/// Main curve types for petrophysical well log data.
///
/// These represent the primary categories of well log measurements
/// commonly found in LAS files.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
pub enum MainCurveType {
    /// Gamma Ray - natural radioactivity measurement
    GR,
    /// Bulk Density - formation density measurement
    RHOB,
    /// Neutron Porosity - hydrogen index measurement
    NPHI,
    /// Resistivity - electrical resistance measurement
    RT,
    /// Caliper - borehole diameter measurement
    CALI,
    /// Sonic - acoustic travel time measurement
    DT,
    /// Spontaneous Potential - natural electrical potential
    SP,
    /// Photo-electric Factor - lithology indicator
    PE,
    /// Undefined - for curves that don't match any known pattern
    #[default]
    Undefined,
}

impl fmt::Display for MainCurveType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_display_name())
    }
}

impl MainCurveType {
    pub fn get_standard_units(&self) -> &'static str {
        match self {
            MainCurveType::GR => "gAPI",
            MainCurveType::RHOB => "g/cm³",
            MainCurveType::NPHI => "v/v",
            MainCurveType::RT => "ohm-m",
            MainCurveType::CALI => "inches",
            MainCurveType::DT => "μs/ft",
            MainCurveType::SP => "mV",
            MainCurveType::PE => "b/e",
            MainCurveType::Undefined => "",
        }
    }
    
    pub fn get_curve_id(&self) -> u32 {
        match self {
            MainCurveType::GR => 1,
            MainCurveType::RHOB => 2,
            MainCurveType::NPHI => 3,
            MainCurveType::RT => 4,
            MainCurveType::CALI => 5,
            MainCurveType::DT => 6,
            MainCurveType::SP => 7,
            MainCurveType::PE => 8,
            MainCurveType::Undefined => 999, // Special ID for undefined curves
        }
    }
    
    pub fn get_display_name(&self) -> &'static str {
        match self {
            MainCurveType::GR => "Gamma Ray",
            MainCurveType::RHOB => "Bulk Density",
            MainCurveType::NPHI => "Neutron Porosity",
            MainCurveType::RT => "Resistivity",
            MainCurveType::CALI => "Caliper",
            MainCurveType::DT => "Sonic",
            MainCurveType::SP => "Spontaneous Potential",
            MainCurveType::PE => "Photo-electric Factor",
            MainCurveType::Undefined => "Undefined",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubcurveMapping {
    pub subcurve_id: u32,
    pub main_curve_type: MainCurveType,
    pub subcurve_name: String,
    pub display_name: String,
    pub description: Option<String>,
    pub units: String,
    pub vendor_variations: Vec<String>,
    pub is_primary: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurveDetection {
    pub raw_name: String,
    pub detected_main_type: Option<MainCurveType>,
    pub suggested_subcurve: Option<String>,
    pub units: String,
    pub confidence: f32,
    pub vendor_variations: Vec<String>,
    pub is_mapped: bool, // Whether this curve was found in the mapping dictionary
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LasProcessingResult {
    pub well_metadata: WellMetadata,
    pub curve_detections: Vec<CurveDetection>,
    pub requires_user_confirmation: bool,
    pub unmapped_curves: Vec<String>, // Curves that need user mapping
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WellMetadata {
    pub well_name: String,
    pub field: String,
    pub company: String,
    pub location: String,
    pub uwi: String,
    pub start_depth: f64,
    pub stop_depth: f64,
    pub step: f64,
    pub null_value: f64,
    pub province: Option<String>,
    pub service_company: Option<String>,
    pub log_date: Option<String>,
}

/// User-configurable curve name mapping dictionary.
///
/// This dictionary maps raw curve mnemonics from LAS files to their
/// standardized [`MainCurveType`] classification. It comes pre-populated
/// with common vendor curve names but can be extended with custom mappings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurveMappingDictionary {
    /// Maps uppercase curve mnemonics to their main curve type
    pub mappings: HashMap<String, MainCurveType>,
    /// Maps uppercase curve mnemonics to detailed subcurve information
    pub subcurve_mappings: HashMap<String, SubcurveMapping>,
}

impl Default for CurveMappingDictionary {
    fn default() -> Self {
        Self::new()
    }
}

impl CurveMappingDictionary {
    pub fn new() -> Self {
        let mut mappings = HashMap::new();
        let mut subcurve_mappings = HashMap::new();
        
        // Default curve name mappings based on common LAS patterns
        // Gamma Ray variations
        mappings.insert("GR".to_string(), MainCurveType::GR);
        mappings.insert("GAMMA".to_string(), MainCurveType::GR);
        mappings.insert("GAMN".to_string(), MainCurveType::GR);
        mappings.insert("IDGR".to_string(), MainCurveType::GR);
        mappings.insert("GST".to_string(), MainCurveType::GR);
        mappings.insert("GSTK".to_string(), MainCurveType::GR);
        mappings.insert("GSTH".to_string(), MainCurveType::GR);
        mappings.insert("GSUR".to_string(), MainCurveType::GR);
        mappings.insert("SGR".to_string(), MainCurveType::GR);
        mappings.insert("CGR".to_string(), MainCurveType::GR);
        
        // Resistivity variations
        mappings.insert("RT".to_string(), MainCurveType::RT);
        mappings.insert("RES".to_string(), MainCurveType::RT);
        mappings.insert("RESISTIVITY".to_string(), MainCurveType::RT);
        mappings.insert("ILD".to_string(), MainCurveType::RT);
        mappings.insert("ILM".to_string(), MainCurveType::RT);
        mappings.insert("SFLA".to_string(), MainCurveType::RT);
        mappings.insert("SFLU".to_string(), MainCurveType::RT);
        mappings.insert("MSFL".to_string(), MainCurveType::RT);
        mappings.insert("IDIM".to_string(), MainCurveType::RT);
        mappings.insert("IDID".to_string(), MainCurveType::RT);
        mappings.insert("IDL3".to_string(), MainCurveType::RT);
        mappings.insert("IDIDC".to_string(), MainCurveType::RT);
        mappings.insert("MEL1".to_string(), MainCurveType::RT);
        mappings.insert("PR".to_string(), MainCurveType::RT);
        mappings.insert("COND".to_string(), MainCurveType::RT);
        
        // Density variations
        mappings.insert("RHOB".to_string(), MainCurveType::RHOB);
        mappings.insert("DENSITY".to_string(), MainCurveType::RHOB);
        mappings.insert("DLDN".to_string(), MainCurveType::RHOB);
        mappings.insert("DFAR".to_string(), MainCurveType::RHOB);
        mappings.insert("DNEAR".to_string(), MainCurveType::RHOB);
        mappings.insert("RHGF".to_string(), MainCurveType::RHOB);
        mappings.insert("RHGA".to_string(), MainCurveType::RHOB);
        mappings.insert("DRHO".to_string(), MainCurveType::RHOB);
        
        // Neutron Porosity variations
        mappings.insert("NPHI".to_string(), MainCurveType::NPHI);
        mappings.insert("NEUTRON".to_string(), MainCurveType::NPHI);
        mappings.insert("NCNPL".to_string(), MainCurveType::NPHI);
        mappings.insert("DLDPL".to_string(), MainCurveType::NPHI);
        mappings.insert("ACAPL".to_string(), MainCurveType::NPHI);
        mappings.insert("NEUT".to_string(), MainCurveType::NPHI);
        mappings.insert("PHIA".to_string(), MainCurveType::NPHI);
        mappings.insert("PHID".to_string(), MainCurveType::NPHI);
        mappings.insert("PHIE".to_string(), MainCurveType::NPHI);
        mappings.insert("PHIN".to_string(), MainCurveType::NPHI);
        mappings.insert("PHIC".to_string(), MainCurveType::NPHI);
        
        // Caliper variations
        mappings.insert("CALI".to_string(), MainCurveType::CALI);
        mappings.insert("CALIPER".to_string(), MainCurveType::CALI);
        mappings.insert("DLCL".to_string(), MainCurveType::CALI);
        mappings.insert("ACCL1".to_string(), MainCurveType::CALI);
        mappings.insert("ACCL2".to_string(), MainCurveType::CALI);
        mappings.insert("DCAL".to_string(), MainCurveType::CALI);
        
        // Sonic variations
        mappings.insert("DT".to_string(), MainCurveType::DT);
        mappings.insert("SONIC".to_string(), MainCurveType::DT);
        mappings.insert("ACTC".to_string(), MainCurveType::DT);
        mappings.insert("TPL".to_string(), MainCurveType::DT);
        
        // Spontaneous Potential variations
        mappings.insert("SP".to_string(), MainCurveType::SP);
        mappings.insert("POTENTIAL".to_string(), MainCurveType::SP);
        mappings.insert("IDSP".to_string(), MainCurveType::SP);
        mappings.insert("SPBL".to_string(), MainCurveType::SP);
        
        // Photo-electric factor variations
        mappings.insert("PE".to_string(), MainCurveType::PE);
        mappings.insert("PHOTO".to_string(), MainCurveType::PE);
        mappings.insert("DLPE".to_string(), MainCurveType::PE);
        mappings.insert("PEF".to_string(), MainCurveType::PE);
        
        // Create default subcurve mappings
        subcurve_mappings.insert("ILD".to_string(), SubcurveMapping {
            subcurve_id: 1,
            main_curve_type: MainCurveType::RT,
            subcurve_name: "RESDEEP".to_string(),
            display_name: "Deep Resistivity".to_string(),
            description: Some("Deep laterolog resistivity".to_string()),
            units: "ohm-m".to_string(),
            vendor_variations: vec!["ILD".to_string(), "RESDEEP".to_string()],
            is_primary: true,
        });
        
        subcurve_mappings.insert("SFLA".to_string(), SubcurveMapping {
            subcurve_id: 2,
            main_curve_type: MainCurveType::RT,
            subcurve_name: "RESSHALLOW".to_string(),
            display_name: "Shallow Resistivity".to_string(),
            description: Some("Shallow laterolog resistivity".to_string()),
            units: "ohm-m".to_string(),
            vendor_variations: vec!["SFLA".to_string(), "RESSHALLOW".to_string()],
            is_primary: false,
        });
        
        subcurve_mappings.insert("ILM".to_string(), SubcurveMapping {
            subcurve_id: 3,
            main_curve_type: MainCurveType::RT,
            subcurve_name: "RESMEDIUM".to_string(),
            display_name: "Medium Resistivity".to_string(),
            description: Some("Medium laterolog resistivity".to_string()),
            units: "ohm-m".to_string(),
            vendor_variations: vec!["ILM".to_string(), "RESMEDIUM".to_string()],
            is_primary: false,
        });
        
        subcurve_mappings.insert("GSTH".to_string(), SubcurveMapping {
            subcurve_id: 10,
            main_curve_type: MainCurveType::GR,
            subcurve_name: "GR_THORIUM".to_string(),
            display_name: "GR Thorium Component".to_string(),
            description: Some("Gamma ray thorium component".to_string()),
            units: "gAPI".to_string(),
            vendor_variations: vec!["GSTH".to_string(), "GR_THORIUM".to_string()],
            is_primary: false,
        });
        
        subcurve_mappings.insert("GSUR".to_string(), SubcurveMapping {
            subcurve_id: 11,
            main_curve_type: MainCurveType::GR,
            subcurve_name: "GR_URANIUM".to_string(),
            display_name: "GR Uranium Component".to_string(),
            description: Some("Gamma ray uranium component".to_string()),
            units: "gAPI".to_string(),
            vendor_variations: vec!["GSUR".to_string(), "GR_URANIUM".to_string()],
            is_primary: false,
        });
        
        CurveMappingDictionary {
            mappings,
            subcurve_mappings,
        }
    }
    
    /// Add a new curve mapping
    pub fn add_mapping(&mut self, curve_name: String, main_type: MainCurveType) {
        self.mappings.insert(curve_name.to_uppercase(), main_type);
    }
    
    /// Remove a curve mapping
    pub fn remove_mapping(&mut self, curve_name: &str) {
        self.mappings.remove(&curve_name.to_uppercase());
    }
    
    /// Get main curve type for a curve name
    pub fn get_main_curve_type(&self, curve_name: &str) -> MainCurveType {
        self.mappings
            .get(&curve_name.to_uppercase())
            .cloned()
            .unwrap_or(MainCurveType::Undefined)
    }
    
    /// Get subcurve mapping for a curve name
    pub fn get_subcurve_mapping(&self, curve_name: &str) -> Option<&SubcurveMapping> {
        self.subcurve_mappings.get(&curve_name.to_uppercase())
    }
    
    /// Check if a curve is mapped
    pub fn is_mapped(&self, curve_name: &str) -> bool {
        self.mappings.contains_key(&curve_name.to_uppercase())
    }
    
    /// Get all unmapped curves from a list
    pub fn get_unmapped_curves(&self, curve_names: &[String]) -> Vec<String> {
        curve_names
            .iter()
            .filter(|name| !self.is_mapped(name))
            .cloned()
            .collect()
    }
    
    /// Update the dictionary from a user configuration
    pub fn update_from_config(&mut self, config: &CurveMappingConfig) {
        for mapping in &config.curve_mappings {
            self.add_mapping(mapping.curve_name.clone(), mapping.main_curve_type.clone());
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurveMappingConfig {
    pub curve_mappings: Vec<CurveMappingEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurveMappingEntry {
    pub curve_name: String,
    pub main_curve_type: MainCurveType,
    pub subcurve_name: Option<String>,
    pub units: Option<String>,
    pub description: Option<String>,
}
