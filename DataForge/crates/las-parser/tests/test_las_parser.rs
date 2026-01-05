use las_parser::LasParser;
use las_types::sections::well::WellSection;
use std::path::PathBuf;

#[test]
fn test_las_parser_parse_single_file() {
    println!("🚀 Testing LAS parser - Parse single file");
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests");
    path.push("test_data");
    path.push("logs");
    path.push("1.2");
    path.push("sample_wrapped.las");
    println!("Reading LAS file: {}", path.display());
    let parser = LasParser::new();
    let result = parser.parse_las_file(path);
    match result {
        Ok(las_file) => {
            println!("✅ Successfully parsed LAS file");
            let well_name = las_file.well.get_value("WELL").unwrap_or("Unknown Well");
            println!("Processing well: {}", well_name);
            assert!(!las_file.curves.columns.is_empty(), "Should have curve data");
            println!("Found {} curves", las_file.curves.columns.len());
        },
        Err(e) => {
            println!("❌ Error parsing LAS file: {}", e);
            panic!("Failed to parse LAS file");
        }
    }
}

#[test]
fn test_las_parser_parse_multiple_files() {
    println!("🚀 Testing LAS parser - Parse multiple files");
    let mut paths = Vec::new();
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests");
    path.push("test_data");
    path.push("logs");
    path.push("1.2");
    path.push("sample_wrapped.las");
    paths.push(path);
    let mut path2 = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path2.push("tests");
    path2.push("test_data");
    path2.push("logs");
    path2.push("1.2");
    path2.push("sample_minimal.las");
    if path2.exists() {
        paths.push(path2);
    }
    let parser = LasParser::new();
    println!("Processing {} LAS files", paths.len());
    let result = parser.parse_multiple_las_files(&paths);
    match result {
        Ok(parsed_files) => {
            println!("✅ Successfully parsed {} files", parsed_files.len());
            for (file_name, las_file) in &parsed_files {
                let well_name = las_file.well.get_value("WELL").unwrap_or("Unknown Well");
                println!("  - {}: {} ({} curves)", file_name, well_name, las_file.curves.columns.len());
                assert!(!las_file.curves.columns.is_empty(), "File {} should have curve data", file_name);
            }
        },
        Err(e) => {
            println!("❌ Error parsing multiple LAS files: {}", e);
            panic!("Failed to parse multiple LAS files");
        }
    }
}

#[test]
fn test_las_parser_parse_invalid_file() {
    println!("🚀 Testing LAS parser - Parse invalid file");
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests");
    path.push("test_data");
    path.push("logs");
    path.push("non_existent_file.las");
    let parser = LasParser::new();
    let result = parser.parse_las_file(path);
    match result {
        Ok(_) => {
            panic!("Should have failed to parse non-existent file");
        },
        Err(e) => {
            println!("✅ Correctly failed to parse non-existent file: {}", e);
        }
    }
}

#[test]
fn test_well_section_parsing() {
    println!("🚀 Testing WellSection parsing logic");
    let mut well_section = WellSection::new();
    
    // Test parsing a valid well section line
    let line = "STRT.m                          635.0000:START DEPTH";
    let result = well_section.parse_line(line);
    
    // Add debug output to help diagnose issues
    println!("Parse result: {:?}", result);
    
    assert!(result.is_ok(), "Failed to parse valid line");
    
    let header_item = well_section.items.get("STRT").expect("STRT item not found");
    
    // Print the actual values for debugging
    println!("Parsed values:");
    println!("Mnemonic: {}", header_item.mnemonic);
    println!("Unit: '{}'", header_item.unit);
    println!("Value: {}", header_item.value);
    println!("Description: {}", header_item.description);
    
    assert_eq!(header_item.mnemonic, "STRT", "Mnemonic mismatch");
    assert_eq!(header_item.unit, "m", "Unit mismatch");
    assert_eq!(header_item.value, "635.0000", "Value mismatch");
    assert_eq!(header_item.description, "START DEPTH", "Description mismatch");

    // Test parsing a comment line
    let comment = "# This is a comment";
    assert!(well_section.parse_line(comment).is_ok(), "Failed to parse comment");

    // Test parsing an empty line
    let empty = "";
    assert!(well_section.parse_line(empty).is_ok(), "Failed to parse empty line");

    // Test parsing an invalid line
    let invalid = "Invalid line format";
    assert!(well_section.parse_line(invalid).is_err(), "Should fail on invalid line");
} 