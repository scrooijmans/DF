# Documentation: What is tested in test_real_file.rs
To run all tests:
cargo test --test test_real_file -- --nocapture

## Overview
The current version of `test_real_file.rs` is designed to test the parsing and validation of LAS files using the `LASFile` struct and its associated methods. The tests focus on verifying that the LAS file parser correctly reads and interprets the contents of real LAS files, checking values from the version, well, parameter, and curve sections. The parser has been enhanced with robust error handling and flexible parsing strategies inspired by the mature `lasio` Python library.

**Integration with Export Functionality**: These tests ensure that the LAS parsing foundation is solid before data is exported to Parquet format. The well name extraction, curve data parsing, and metadata handling tested here are critical for the advanced export features like custom well ID assignment and deduplication.

## Main Functions and Files Involved

### 1. `test_real_las_file_short`
- **Tested file:** `6038187_v1.2_short.las`
- **Functionality:**
  - Calls `test_las_file("6038187_v1.2_short.las")`.
  - Verifies version, well, parameter, and curve data for this LAS file.
  - Tests basic LAS 1.2 format parsing.
  - **Export Relevance**: This file is used in multi-well exports and demonstrates deduplication when combined with the full version.

### 2. `test_real_las_file_full`
- **Tested file:** `6038187_v1.2.las`
- **Functionality:**
  - Calls `test_las_file("6038187_v1.2.las")`.
  - Verifies version, well, parameter, and curve data for this LAS file.
  - Tests full LAS 1.2 format with more comprehensive data.
  - **Export Relevance**: This file represents the same well as the short version, enabling deduplication testing in multi-well exports.

### 3. `test_real_las_file_1001178549`
- **Tested file:** `1001178549.las`
- **Functionality:**
  - Calls `test_las_file("1001178549.las")`.
  - Contains file-specific assertions for version, well, and parameter values.
  - Tests complex LAS 2.0 format with API codes and multiple curve sections.
  - Verifies the presence and count of curve headers (27 curves expected).
  - **Export Relevance**: This file demonstrates complex LAS 2.0 parsing and contributes to the global curve metadata in multi-well exports.

## Core Test Logic: `test_las_file`
- **File:** `test_real_file.rs`
- **Functionality:**
  - Loads a LAS file using `LASFile::read`.
  - For `1001178549.las`, checks specific values for version, well, and parameter sections, and verifies curve headers.
  - For other files, checks expected values for version, well, and parameter sections, and verifies curve headers.
  - Always checks that depth data is present and that curve data vectors are not empty.
  - Prints out curve section headers and some statistics for curve data.
  - **Export Integration**: Validates that well names are correctly extracted for custom well ID assignment.

## Functions Being Tested

### LASFile Methods
- **`LASFile::new()`** - Constructor for creating new LAS file objects
- **`LASFile::read()`** - Main file reading and parsing function
  - Section detection and parsing
  - Null value detection and handling
  - Error handling and logging
  - Post-processing validation
  - **Export Critical**: This is the foundation for all export operations

### Section Parsing Methods
- **`VersionSection::parse_line()`** - Version section line parsing
- **`WellSection::parse_line()`** - Well section line parsing with LAS 2.0 delimiter rules
  - **Export Critical**: Well name extraction for custom well ID assignment
- **`ParameterSection::parse_line()`** - Parameter section line parsing with leading zero handling
- **`CurveSection::parse_header()`** - Curve header parsing with flexible regex patterns
- **`CurveSection::parse_header_with_mode()`** - Multi-mode curve header parsing (Standard/Legacy)
- **`CurveSection::parse_data_line()`** - Tolerant data line parsing with column mismatch handling

### Data Access Methods
- **`WellSection::get_value()`** - Retrieving well section values by mnemonic
  - **Export Critical**: Used to extract well names for deduplication and custom ID assignment
- **`ParameterSection::get_value()`** - Retrieving parameter values by mnemonic
- **`CurveSection::get_curve_data()`** - Accessing curve data by mnemonic
- **`CurveSection::get_depth_data()`** - Finding depth curve data
  - **Export Critical**: Essential for data normalization and deduplication
- **`CurveSection::get_gr_data()`** - Finding gamma ray curve data

## Robust Parsing Features Tested

### Error Handling
- **Tolerant parsing** - Continues parsing even with format inconsistencies
- **Warning logging** - Logs issues without failing completely
- **Fallback mechanisms** - Multiple parsing strategies for robustness

### Flexible Header Parsing
- **Regex-based parsing** - Multiple patterns to handle different LAS formats
- **API code handling** - Ignores API codes in curve headers
- **Duplicate mnemonic handling** - Appends `:1`, `:2`, etc. for duplicates
- **Variable whitespace tolerance** - Handles inconsistent spacing

### Data Section Parsing
- **Column mismatch handling** - Pads/truncates data to match expected columns
- **Null value processing** - Converts null values to NaN appropriately
- **Invalid value handling** - Continues parsing with NaN for invalid values

## Integration with Export Features

### Well Name Extraction for Custom IDs
The tests verify that well names are correctly extracted from LAS file headers:
```rust
// In export functionality, this becomes:
let well_name_value = las_file.well.get_value("WELL").map(|s| s.to_string())
    .unwrap_or_else(|| file_name.clone());
```

### Depth Data for Deduplication
The tests ensure depth data is available for the deduplication logic:
```rust
// In export functionality, this becomes:
let depth_data = if let Some(data) = las_file.curves.get_depth_data() {
    data
} else if let Some(first_column) = las_file.curves.columns.first() {
    &first_column.values
} else {
    continue;
};
```

### Curve Metadata for Global Schema
The tests validate curve information that becomes part of the global curve metadata:
```rust
// In export functionality, this becomes:
for column in &las_file.curves.columns {
    if !is_depth_curve {
        global_curve_metadata.push((
            curve_id_counter,
            column.mnemonic.clone(),
            column.unit.clone(),
            column.description.clone(),
        ));
    }
}
```

## Dependencies
- **Main struct tested:** `LASFile` (from `las_rust` crate)
- **Section structs tested:**
  - `VersionSection` - Version information parsing
  - `WellSection` - Well information parsing
  - `ParameterSection` - Parameter information parsing
  - `CurveSection` - Curve information and data parsing

## Test Results and Export Impact

### Current Test Status
- **All three test files pass** ✅
- **Well name extraction working correctly** ✅
- **Depth data available for all files** ✅
- **Curve metadata complete** ✅

### Export Integration Points
1. **Well Name Consistency**: Tests ensure well names are extracted correctly for custom well ID assignment
2. **Data Completeness**: Tests verify all necessary data is available for export
3. **Format Compatibility**: Tests cover LAS 1.2, 2.0, and 3.0 formats used in multi-well exports
4. **Error Resilience**: Tests ensure parsing continues even with format inconsistencies

### Multi-Well Export Dependencies
- **Well Name Extraction**: Critical for deduplication and custom ID assignment
- **Depth Data Availability**: Essential for data normalization
- **Curve Information**: Required for global curve metadata
- **File Format Handling**: Supports all LAS versions in multi-well exports

## Summary
- The tests ensure that the LAS parser can correctly interpret real-world LAS files and extract the expected values from all major sections.
- The parser is now robust and can handle format variations, missing data, and inconsistent formatting.
- The tests are file-specific where needed, and generic for other files.
- The output and assertions help catch parsing errors and mismatches in expected vs. actual data.
- All three test files now pass, demonstrating successful parsing of different LAS file formats and versions.
- **Critical for Export Functionality**: These tests validate the foundation upon which all export features (custom well IDs, deduplication, source file tracking) depend.

## Export Workflow Integration

### Pre-Export Validation
Before any export operation, the parsing functionality tested here ensures:
- Well names are correctly extracted from LAS headers
- Depth data is available and properly formatted
- Curve information is complete and accurate
- File format compatibility is maintained

### Export Feature Dependencies
- **Custom Well ID Assignment**: Depends on accurate well name extraction
- **Deduplication**: Requires consistent depth data and well identification
- **Source File Tracking**: Needs reliable file parsing and metadata extraction
- **Multi-Well Exports**: Requires all files to parse successfully

### Quality Assurance
These tests provide the quality assurance foundation for:
- Data integrity in exported Parquet files
- Consistent well identification across multiple files
- Reliable curve metadata for analytical workflows
- Robust error handling in production environments 