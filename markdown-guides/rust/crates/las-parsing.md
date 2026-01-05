# LAS Parser Crate

## Overview

The `las-parser` crate is the main entry point for LAS file processing in MudRock. It provides a high-level API for parsing LAS files, managing curve mappings, and preparing data for the petrophysical schema conversion pipeline.

## Core Responsibilities

### 1. LAS File Processing
- **Parse LAS files** (versions 1.2, 2.0, 3.0) using the `las-types` crate
- **Extract well metadata** (well name, field, company, location, UWI, etc.)
- **Process curve data** with proper depth indexing and null value handling
- **Handle multiple LAS files** for batch processing operations

### 2. Curve Mapping Management
- **Dictionary-based curve detection** using configurable `CurveMappingDictionary`
- **Automatic curve type detection** for common petrophysical measurements
- **User-configurable mappings** for custom curve names and vendor variations
- **Batch curve mapping operations** for efficient processing

### 3. Data Preparation for Schema Conversion
- **Generate confirmation data** for user review and approval
- **Identify unmapped curves** requiring user input
- **Extract curve metadata** for schema validation
- **Prepare data structures** for downstream DuckDB processing

## Key Components

### LasParser
```rust
pub struct LasParser {
    curve_mapping_dict: CurveMappingDictionary,
}
```

**Methods:**
- `new()` - Create parser with default curve mappings
- `new_with_mappings()` - Create parser with custom mappings
- `parse_las_file()` - Parse single LAS file
- `parse_multiple_las_files()` - Batch parse multiple files
- `process_las_file_for_confirmation()` - Prepare data for user confirmation
- `update_mappings()` - Update curve mapping dictionary

### Curve Mapping Operations
- `update_curve_mapping()` - Update single curve mapping
- `batch_update_curve_mappings()` - Update multiple curves
- `get_mapped_curves()` - Get all mapped curves
- `get_unmapped_curves()` - Get curves requiring user input
- `get_mapping_statistics()` - Get mapping statistics and metrics

## Integration with Petrophysical Flow

### Phase 1: LAS File Ingestion
```
LAS File → LasParser.parse_las_file() → LASFile object
```

### Phase 2: Curve Detection & Mapping
```
LASFile → LasParser.process_las_file_for_confirmation() → LasProcessingResult
```

### Phase 3: User Confirmation
```
LasProcessingResult → UI → User Mappings → LasParser.update_mappings()
```

### Phase 4: Schema Conversion
```
Updated Mappings → LasParser → Normalized Data → DuckDB Parquet
```

## Performance Characteristics

- **Parsing Speed**: ~37ms for large LAS files (29K+ lines)
- **Mapping Operations**: < 20µs for batch curve updates
- **Memory Efficiency**: Minimal allocations, reuses existing data structures
- **Scalability**: Handles 100+ wells in batch operations

## Error Handling

- **Graceful degradation** for malformed LAS files
- **Detailed error messages** for debugging
- **Validation** of curve mappings and metadata
- **Recovery** from partial parsing failures

## Testing

Comprehensive test suite covering:
- **Unit tests** for individual parsing functions
- **Integration tests** with real LAS files
- **Performance tests** for batch operations
- **Error handling tests** for edge cases

## Dependencies

- **las-types**: Core LAS file parsing and data structures
- **serde**: Serialization for data exchange
- **std::collections**: HashMap for efficient lookups
- **std::time**: Performance measurement

## Future Enhancements

- **Parallel processing** for large batch operations
- **Caching** for frequently accessed mappings
- **Advanced curve detection** using machine learning
- **Real-time validation** during parsing
