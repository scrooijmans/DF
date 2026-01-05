add # LAS Types Crate

## Overview

The `las-types` crate provides the foundational data structures and parsing logic for LAS (Log ASCII Standard) files in MudRock. It handles the low-level parsing of LAS files and defines the core types used throughout the petrophysical data processing pipeline.

## Core Responsibilities

### 1. LAS File Parsing
- **Parse LAS file sections** (Version, Well, Curve, Parameter, Data)
- **Handle multiple LAS versions** (1.2, 2.0, 3.0) with version-specific logic
- **Extract curve data** with proper type conversion and null value handling
- **Validate file structure** and report parsing errors

### 2. Data Structure Definitions
- **LASFile** - Main container for parsed LAS data
- **Section structs** - VersionSection, WellSection, CurveSection, ParameterSection
- **Curve data types** - CurveDataColumn with values, units, descriptions
- **Metadata extraction** - Well information, curve headers, parameters

### 3. Petrophysical Schema Support
- **MainCurveType enum** - Canonical curve types (GR, RHOB, NPHI, RT, etc.)
- **SubcurveMapping** - Hierarchical curve structure with vendor variations
- **CurveMappingDictionary** - User-configurable curve name mappings
- **WellMetadata** - Standardized well information extraction

### 4. Curve Mapping Operations
- **Functional approach** - Pure functions for curve mapping operations
- **Memory efficient** - Minimal allocations, reuses existing data
- **Batch operations** - Efficient processing of multiple curves
- **Validation** - Schema consistency and mapping validation

## Key Components

### LAS File Structure
```rust
pub struct LASFile {
    pub version: VersionSection,
    pub well: WellSection,
    pub curves: CurveSection,
    pub parameters: ParameterSection,
    pub data: Vec<Vec<f64>>,
}
```

### Petrophysical Types
```rust
pub enum MainCurveType {
    GR, RHOB, NPHI, RT, CALI, DT, SP, PE, Undefined
}

pub struct SubcurveMapping {
    pub subcurve_id: u32,
    pub main_curve_type: MainCurveType,
    pub subcurve_name: String,
    pub display_name: String,
    pub units: String,
    pub vendor_variations: Vec<String>,
}
```

### Curve Mapping Dictionary
```rust
pub struct CurveMappingDictionary {
    pub mappings: HashMap<String, MainCurveType>,
    pub subcurve_mappings: HashMap<String, SubcurveMapping>,
}
```

## Parsing Capabilities

### LAS Version Support
- **LAS 1.2**: Standard format with basic curve information
- **LAS 2.0**: Enhanced with additional metadata fields
- **LAS 3.0**: Modern format with improved structure

### Curve Data Handling
- **Automatic type detection** for numeric values
- **Null value handling** with configurable null indicators
- **Unit extraction** from curve headers
- **Description parsing** for curve metadata

### Error Recovery
- **Graceful degradation** for malformed sections
- **Partial parsing** when some sections are corrupted
- **Detailed error reporting** for debugging

## Integration with Petrophysical Flow

### Phase 1: Raw LAS Parsing
```
LAS File → las-types → LASFile object
```

### Phase 2: Metadata Extraction
```
LASFile → WellSection.get_*() → WellMetadata
```

### Phase 3: Curve Processing
```
LASFile → CurveSection → CurveDataColumn[] → MainCurveType mapping
```

### Phase 4: Schema Conversion
```
CurveDataColumn + MainCurveType → Normalized data → DuckDB Parquet
```

## Performance Characteristics

- **Parsing Speed**: ~37ms for large LAS files (29K+ lines)
- **Memory Usage**: Efficient data structures with minimal overhead
- **Scalability**: Handles files with 50+ curves and 20K+ data points
- **Error Handling**: Fast failure with detailed error messages

## Functional Design

### Curve Mapping Operations
All curve mapping operations are implemented as pure functions:
- `update_curve_mapping()` - Update single curve mapping
- `batch_update_curve_mappings()` - Batch update operations
- `get_mapped_curves()` - Get mapped curves
- `get_unmapped_curves()` - Get unmapped curves
- `validate_curve_mappings()` - Schema validation

### Benefits of Functional Approach
- **Testability** - Easy to unit test individual functions
- **Reusability** - Functions can be composed and reused
- **Immutability** - No side effects, predictable behavior
- **Performance** - Minimal allocations, efficient operations

## Testing

Comprehensive test coverage:
- **Unit tests** for each parsing function
- **Integration tests** with real LAS files
- **Error handling tests** for malformed data
- **Performance tests** for large files

## Dependencies

- **serde** - Serialization for data exchange
- **std::collections** - HashMap for efficient lookups
- **std::time** - Performance measurement
- **std::error** - Error handling

## Future Enhancements

- **Enhanced curve detection** using pattern matching
- **Automatic unit conversion** between different unit systems
- **Data quality validation** during parsing
- **Parallel parsing** for large batch operations
