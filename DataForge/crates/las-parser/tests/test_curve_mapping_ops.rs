use las_parser::LasParser;
use las_types::{MainCurveType, CurveMappingDictionary, update_curve_mapping, get_mapped_curves, get_unmapped_curves, get_mapping_statistics, batch_update_curve_mappings};

#[test]
fn test_curve_mapping_operations() {
    let parser = LasParser::new();
    let mut las_file = parser.parse_las_file("tests/test_data/logs/1.2/sample_big.las").unwrap();
    let mut mapping_dict = CurveMappingDictionary::new();
    
    println!("=== Initial Curve Mappings ===");
    let initial_mappings = get_mapped_curves(&las_file, &mapping_dict);
    println!("Mapped curves: {:?}", initial_mappings);
    
    let unmapped = get_unmapped_curves(&las_file, &mapping_dict);
    println!("Unmapped curves: {:?}", unmapped);
    
    // Test updating a specific curve mapping
    println!("\n=== Updating DEPT curve to DT ===");
    match update_curve_mapping(&las_file, "DEPT", MainCurveType::DT, &mut mapping_dict) {
        Ok(previous) => {
            println!("Updated DEPT -> DT, previous mapping: {:?}", previous);
        }
        Err(e) => {
            println!("Error updating DEPT: {}", e);
        }
    }
    
    // Test batch updates
    println!("\n=== Batch Updates ===");
    let batch_updates = vec![
        ("DEPT".to_string(), MainCurveType::DT),
        ("UNKNOWN_CURVE".to_string(), MainCurveType::GR), // This should fail
    ];
    
    let results = batch_update_curve_mappings(&las_file, &batch_updates, &mut mapping_dict);
    for (curve_name, previous, success) in results {
        println!("{} -> success: {}, previous: {:?}", curve_name, success, previous);
    }
    
    // Get updated mappings
    println!("\n=== Updated Mappings ===");
    let updated_mappings = get_mapped_curves(&las_file, &mapping_dict);
    println!("Mapped curves: {:?}", updated_mappings);
    
    let updated_unmapped = get_unmapped_curves(&las_file, &mapping_dict);
    println!("Unmapped curves: {:?}", updated_unmapped);
    
    // Get statistics
    println!("\n=== Mapping Statistics ===");
    let stats = get_mapping_statistics(&las_file, &mapping_dict);
    println!("Total curves: {}", stats.total_curves);
    println!("Mapped: {} ({:.1}%)", stats.mapped_curves, stats.mapping_percentage());
    println!("Unmapped: {} ({:.1}%)", stats.unmapped_curves, stats.unmapped_percentage());
    println!("Type counts: {:?}", stats.type_counts);
}

#[test]
fn test_curve_mapping_performance() {
    let parser = LasParser::new();
    let las_file = parser.parse_las_file("tests/test_data/logs/1.2/sample_big.las").unwrap();
    let mut mapping_dict = CurveMappingDictionary::new();
    
    // Test performance with many updates
    let start = std::time::Instant::now();
    
    // Simulate updating all curves
    for col in &las_file.curves.columns {
        let _ = update_curve_mapping(&las_file, &col.mnemonic, MainCurveType::GR, &mut mapping_dict);
    }
    
    let duration = start.elapsed();
    println!("Updated {} curves in {:?}", las_file.curves.columns.len(), duration);
    
    // Test batch performance
    let start = std::time::Instant::now();
    
    let updates: Vec<(String, MainCurveType)> = las_file.curves.columns
        .iter()
        .map(|col| (col.mnemonic.clone(), MainCurveType::RT))
        .collect();
    
    let _ = batch_update_curve_mappings(&las_file, &updates, &mut mapping_dict);
    
    let duration = start.elapsed();
    println!("Batch updated {} curves in {:?}", updates.len(), duration);
}

#[test]
fn test_curve_mapping_validation() {
    let parser = LasParser::new();
    let las_file = parser.parse_las_file("tests/test_data/logs/1.2/sample_big.las").unwrap();
    let mut mapping_dict = CurveMappingDictionary::new();
    
    // Set up some mappings that might cause conflicts
    mapping_dict.add_mapping("CURVE1".to_string(), MainCurveType::GR);
    mapping_dict.add_mapping("CURVE2".to_string(), MainCurveType::GR);
    mapping_dict.add_mapping("CURVE3".to_string(), MainCurveType::RHOB);
    
    // This should be fine - multiple resistivity curves
    mapping_dict.add_mapping("ILD".to_string(), MainCurveType::RT);
    mapping_dict.add_mapping("ILM".to_string(), MainCurveType::RT);
    mapping_dict.add_mapping("SFLA".to_string(), MainCurveType::RT);
    
    println!("Mapping dictionary set up with test mappings");
    println!("Total mappings: {}", mapping_dict.mappings.len());
}
