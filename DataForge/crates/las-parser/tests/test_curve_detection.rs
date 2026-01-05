use las_parser::LasParser;
use las_types::{MainCurveType, CurveMappingDictionary};

#[test]
fn test_curve_detection_with_dictionary() {
    let parser = LasParser::new();
    
    // Test with the sample big LAS file
    let las_file = parser.parse_las_file("tests/test_data/logs/1.2/sample_big.las").unwrap();
    
    // Process for confirmation
    let result = parser.process_las_file_for_confirmation(&las_file, "Test Well").unwrap();
    
    println!("Well: {}", result.well_metadata.well_name);
    println!("Field: {}", result.well_metadata.field);
    println!("Company: {}", result.well_metadata.company);
    println!("Location: {}", result.well_metadata.location);
    println!("UWI: {}", result.well_metadata.uwi);
    println!("Start Depth: {}", result.well_metadata.start_depth);
    println!("Stop Depth: {}", result.well_metadata.stop_depth);
    println!("Step: {}", result.well_metadata.step);
    println!("Null Value: {}", result.well_metadata.null_value);
    
    println!("\nCurve Detection Results:");
    for detection in &result.curve_detections {
        println!("  {} -> {:?} (mapped: {}, confidence: {:.2})", 
                detection.raw_name, 
                detection.detected_main_type,
                detection.is_mapped,
                detection.confidence);
    }
    
    println!("\nUnmapped Curves: {:?}", result.unmapped_curves);
    println!("Requires User Confirmation: {}", result.requires_user_confirmation);
    
    // Get mapping statistics
    let (mapped, total, unmapped) = parser.get_mapping_stats(&las_file);
    println!("\nMapping Statistics:");
    println!("  Mapped: {}/{}", mapped, total);
    println!("  Unmapped: {:?}", unmapped);
}

#[test]
fn test_custom_curve_mapping() {
    let mut parser = LasParser::new();
    
    // Add custom mappings
    parser.add_curve_mapping("CUSTOM_GR".to_string(), MainCurveType::GR);
    parser.add_curve_mapping("CUSTOM_RT".to_string(), MainCurveType::RT);
    
    // Test the mappings
    let mappings = parser.get_mappings();
    assert_eq!(mappings.get_main_curve_type("CUSTOM_GR"), MainCurveType::GR);
    assert_eq!(mappings.get_main_curve_type("CUSTOM_RT"), MainCurveType::RT);
    assert_eq!(mappings.get_main_curve_type("UNKNOWN_CURVE"), MainCurveType::Undefined);
    
    println!("Custom mappings work correctly!");
}

#[test]
fn test_curve_mapping_dictionary() {
    let mut dict = CurveMappingDictionary::new();
    
    // Test default mappings
    assert_eq!(dict.get_main_curve_type("GR"), MainCurveType::GR);
    assert_eq!(dict.get_main_curve_type("ILD"), MainCurveType::RT);
    assert_eq!(dict.get_main_curve_type("RHOB"), MainCurveType::RHOB);
    assert_eq!(dict.get_main_curve_type("NPHI"), MainCurveType::NPHI);
    assert_eq!(dict.get_main_curve_type("CALI"), MainCurveType::CALI);
    assert_eq!(dict.get_main_curve_type("DT"), MainCurveType::DT);
    assert_eq!(dict.get_main_curve_type("SP"), MainCurveType::SP);
    assert_eq!(dict.get_main_curve_type("PE"), MainCurveType::PE);
    
    // Test undefined curve
    assert_eq!(dict.get_main_curve_type("UNKNOWN_CURVE"), MainCurveType::Undefined);
    
    // Test case insensitivity
    assert_eq!(dict.get_main_curve_type("gr"), MainCurveType::GR);
    assert_eq!(dict.get_main_curve_type("Gr"), MainCurveType::GR);
    
    // Test subcurve mappings
    if let Some(subcurve) = dict.get_subcurve_mapping("ILD") {
        assert_eq!(subcurve.subcurve_name, "RESDEEP");
        assert_eq!(subcurve.is_primary, true);
    }
    
    if let Some(subcurve) = dict.get_subcurve_mapping("SFLA") {
        assert_eq!(subcurve.subcurve_name, "RESSHALLOW");
        assert_eq!(subcurve.is_primary, false);
    }
    
    println!("Curve mapping dictionary works correctly!");
}
