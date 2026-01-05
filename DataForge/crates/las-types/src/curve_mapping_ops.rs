use crate::{LASFile, MainCurveType, CurveMappingDictionary};
use std::collections::HashMap;

/// Update all mapped curves in a LAS file based on the curve mapping dictionary
/// This function modifies the LAS file in-place to avoid unnecessary allocations
pub fn update_all_mapped_curves(
    las_file: &mut LASFile,
    mapping_dict: &CurveMappingDictionary,
) -> Result<(), String> {
    // Create a mapping from curve mnemonic to main curve type for efficient lookup
    let curve_mappings: HashMap<String, MainCurveType> = las_file.curves.columns
        .iter()
        .filter_map(|col| {
            if mapping_dict.is_mapped(&col.mnemonic) {
                Some((col.mnemonic.clone(), mapping_dict.get_main_curve_type(&col.mnemonic)))
            } else {
                None
            }
        })
        .collect();

    // Update curve metadata if needed (this would be stored in a separate metadata structure)
    // For now, we'll just validate that the mappings are consistent
    for (mnemonic, main_type) in &curve_mappings {
        if *main_type == MainCurveType::Undefined {
            return Err(format!("Curve '{}' is mapped but has undefined main curve type", mnemonic));
        }
    }

    Ok(())
}

/// Update a specific curve's main curve type mapping
/// Returns the previous mapping if it existed
pub fn update_curve_mapping(
    las_file: &LASFile,
    curve_name: &str,
    new_main_type: MainCurveType,
    mapping_dict: &mut CurveMappingDictionary,
) -> Result<Option<MainCurveType>, String> {
    // Validate that the curve exists in the LAS file
    if !las_file.curves.columns.iter().any(|col| col.mnemonic == curve_name) {
        return Err(format!("Curve '{}' not found in LAS file", curve_name));
    }

    // Get the previous mapping
    let previous_mapping = mapping_dict.get_main_curve_type(curve_name);
    
    // Update the mapping
    mapping_dict.add_mapping(curve_name.to_string(), new_main_type);
    
    // Return the previous mapping (if it wasn't undefined)
    if previous_mapping != MainCurveType::Undefined {
        Ok(Some(previous_mapping))
    } else {
        Ok(None)
    }
}

/// Remove a curve mapping (set it back to undefined)
/// Returns the previous mapping if it existed
pub fn remove_curve_mapping(
    curve_name: &str,
    mapping_dict: &mut CurveMappingDictionary,
) -> Option<MainCurveType> {
    let previous_mapping = mapping_dict.get_main_curve_type(curve_name);
    mapping_dict.remove_mapping(curve_name);
    
    if previous_mapping != MainCurveType::Undefined {
        Some(previous_mapping)
    } else {
        None
    }
}

/// Get all curve mappings for a LAS file
/// Returns a vector of (curve_name, main_curve_type) tuples
pub fn get_curve_mappings(
    las_file: &LASFile,
    mapping_dict: &CurveMappingDictionary,
) -> Vec<(String, MainCurveType)> {
    las_file.curves.columns
        .iter()
        .map(|col| (col.mnemonic.clone(), mapping_dict.get_main_curve_type(&col.mnemonic)))
        .collect()
}

/// Get only mapped curves (excluding undefined ones)
pub fn get_mapped_curves(
    las_file: &LASFile,
    mapping_dict: &CurveMappingDictionary,
) -> Vec<(String, MainCurveType)> {
    las_file.curves.columns
        .iter()
        .filter_map(|col| {
            let main_type = mapping_dict.get_main_curve_type(&col.mnemonic);
            if main_type != MainCurveType::Undefined {
                Some((col.mnemonic.clone(), main_type))
            } else {
                None
            }
        })
        .collect()
}

/// Get only unmapped curves (undefined ones)
pub fn get_unmapped_curves(
    las_file: &LASFile,
    mapping_dict: &CurveMappingDictionary,
) -> Vec<String> {
    las_file.curves.columns
        .iter()
        .filter_map(|col| {
            if mapping_dict.get_main_curve_type(&col.mnemonic) == MainCurveType::Undefined {
                Some(col.mnemonic.clone())
            } else {
                None
            }
        })
        .collect()
}

/// Batch update multiple curve mappings
/// Returns a vector of (curve_name, previous_mapping, success) tuples
pub fn batch_update_curve_mappings(
    las_file: &LASFile,
    updates: &[(String, MainCurveType)],
    mapping_dict: &mut CurveMappingDictionary,
) -> Vec<(String, Option<MainCurveType>, bool)> {
    let mut results = Vec::with_capacity(updates.len());
    
    for (curve_name, new_main_type) in updates {
        match update_curve_mapping(las_file, curve_name, new_main_type.clone(), mapping_dict) {
            Ok(previous_mapping) => {
                results.push((curve_name.clone(), previous_mapping, true));
            }
            Err(_) => {
                results.push((curve_name.clone(), None, false));
            }
        }
    }
    
    results
}

/// Validate curve mappings for consistency
/// Returns a vector of validation errors
pub fn validate_curve_mappings(
    las_file: &LASFile,
    mapping_dict: &CurveMappingDictionary,
) -> Vec<String> {
    let mut errors = Vec::new();
    
    // Check for duplicate main curve types that might conflict
    let mut main_type_counts: HashMap<MainCurveType, Vec<String>> = HashMap::new();
    
    for col in &las_file.curves.columns {
        let main_type = mapping_dict.get_main_curve_type(&col.mnemonic);
        if main_type != MainCurveType::Undefined {
            main_type_counts.entry(main_type)
                .or_insert_with(Vec::new)
                .push(col.mnemonic.clone());
        }
    }
    
    // Check for potential conflicts (multiple curves mapped to same main type)
    for (main_type, curve_names) in main_type_counts {
        if curve_names.len() > 1 {
            // This is not necessarily an error, but worth noting
            // For example, multiple resistivity curves (ILD, ILM, SFLA) can all map to RT
            if main_type == MainCurveType::RT || main_type == MainCurveType::GR {
                // These are expected to have multiple subcurves
                continue;
            } else {
                errors.push(format!(
                    "Multiple curves mapped to {:?}: {:?}",
                    main_type, curve_names
                ));
            }
        }
    }
    
    errors
}

/// Get curve mapping statistics
pub fn get_mapping_statistics(
    las_file: &LASFile,
    mapping_dict: &CurveMappingDictionary,
) -> CurveMappingStats {
    let total_curves = las_file.curves.columns.len();
    let mapped_curves = get_mapped_curves(las_file, mapping_dict).len();
    let unmapped_curves = get_unmapped_curves(las_file, mapping_dict).len();
    
    // Count by main curve type
    let mut type_counts: HashMap<MainCurveType, usize> = HashMap::new();
    for (_, main_type) in get_mapped_curves(las_file, mapping_dict) {
        *type_counts.entry(main_type).or_insert(0) += 1;
    }
    
    CurveMappingStats {
        total_curves,
        mapped_curves,
        unmapped_curves,
        type_counts,
    }
}

#[derive(Debug, Clone)]
pub struct CurveMappingStats {
    pub total_curves: usize,
    pub mapped_curves: usize,
    pub unmapped_curves: usize,
    pub type_counts: HashMap<MainCurveType, usize>,
}

impl CurveMappingStats {
    pub fn mapping_percentage(&self) -> f64 {
        if self.total_curves == 0 {
            0.0
        } else {
            (self.mapped_curves as f64 / self.total_curves as f64) * 100.0
        }
    }
    
    pub fn unmapped_percentage(&self) -> f64 {
        if self.total_curves == 0 {
            0.0
        } else {
            (self.unmapped_curves as f64 / self.total_curves as f64) * 100.0
        }
    }
}

/// Create a curve mapping from a user selection
/// This is a helper function for UI interactions
pub fn create_curve_mapping_from_selection(
    curve_name: String,
    main_curve_type: MainCurveType,
    _subcurve_name: Option<String>,
    _units: Option<String>,
    _description: Option<String>,
) -> Result<(), String> {
    if curve_name.is_empty() {
        return Err("Curve name cannot be empty".to_string());
    }
    
    if main_curve_type == MainCurveType::Undefined {
        return Err("Cannot create mapping for undefined curve type".to_string());
    }
    
    // This would typically be used to create a new mapping entry
    // The actual implementation would depend on how you want to store user selections
    Ok(())
}

/// Export curve mappings to a configuration format
pub fn export_curve_mappings(
    las_file: &LASFile,
    mapping_dict: &CurveMappingDictionary,
) -> CurveMappingExport {
    let mappings = get_curve_mappings(las_file, mapping_dict);
    let stats = get_mapping_statistics(las_file, mapping_dict);
    
    CurveMappingExport {
        well_name: las_file.well.get_well_name().unwrap_or("Unknown").to_string(),
        field: las_file.well.get_field().unwrap_or("Unknown").to_string(),
        company: las_file.well.get_company().unwrap_or("Unknown").to_string(),
        mappings,
        statistics: stats,
    }
}

#[derive(Debug, Clone)]
pub struct CurveMappingExport {
    pub well_name: String,
    pub field: String,
    pub company: String,
    pub mappings: Vec<(String, MainCurveType)>,
    pub statistics: CurveMappingStats,
}
