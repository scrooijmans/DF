# DuckDB Petrophysics Schema Design

## Overview

This document defines the optimal Parquet schema design for petrophysical data in MudRock, leveraging DuckDB's performance characteristics while handling the complex hierarchical nature of well log data.

## Data Profile

- **Depth spacing**: 0.1524m (6 inches)
- **Typical depth**: 3000m
- **Samples per well**: ~19,685 data points (3000m ÷ 0.1524m)
- **Curves per well**: 2-50 curves
- **Total records per well**: 39,370 to 984,250 records

## Architecture Design

### Three-Layer Hierarchical Structure

```
1. Main Curve Types (Rigid)     → GR, RHOB, NPHI, RT, CALI, DT, SP, PE
2. Subcurve Categories (Flexible) → RESDEEP, RESSHALLOW, RESMED, GR_THORIUM, GR_URANIUM
3. Raw Curve Names (User-defined) → "Deep Resistivity", "Shallow Res", "GR_Thorium_Component"
```

## Schema Definitions

### 1. Main Curve Types (Canonical Schema)

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MainCurveType {
    GR,      // Gamma Ray
    RHOB,    // Bulk Density
    NPHI,    // Neutron Porosity
    RT,      // Resistivity
    CALI,    // Caliper
    DT,      // Sonic
    SP,      // Spontaneous Potential
    PE,      // Photo-electric factor
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
        }
    }
}
```

### 2. Subcurve Mapping System

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubcurveMapping {
    pub subcurve_id: u32,
    pub main_curve_type: MainCurveType,
    pub subcurve_name: String,           // e.g., "RESDEEP", "RESSHALLOW"
    pub display_name: String,            // e.g., "Deep Resistivity"
    pub description: Option<String>,     // e.g., "Deep laterolog resistivity"
    pub units: String,                   // e.g., "ohm-m"
    pub vendor_variations: Vec<String>,  // e.g., ["RESDEEP", "RT_DEEP", "DEEP_RES"]
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurveMetadata {
    pub well_id: u32,
    pub subcurve_id: u32,
    pub raw_curve_name: String,          // User-defined name from LAS file
    pub main_curve_type: MainCurveType,
    pub subcurve_name: String,
    pub units: String,
    pub min_depth: f64,
    pub max_depth: f64,
    pub data_points: u32,
    pub null_value: f64,
    pub step: f64,
}
```

### 3. Optimized Parquet Schema

```rust
// Main well data (per-well single file)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WellDataRecord {
    // Primary keys
    pub well_id: u32,
    pub depth: f64,
    
    // Main curve data (sparse columns for common curves)
    pub gr: Option<f32>,               // Gamma Ray (any subcurve)
    pub rhob: Option<f32>,             // Bulk Density
    pub nphi: Option<f32>,             // Neutron Porosity
    pub rt: Option<f32>,               // Resistivity (any subcurve)
    pub cali: Option<f32>,             // Caliper
    pub dt: Option<f32>,               // Sonic
    pub sp: Option<f32>,               // Spontaneous Potential
    pub pe: Option<f32>,               // Photo-electric factor
    
    // Subcurve identification
    pub gr_subcurve_id: Option<u32>,   // Which GR subcurve this represents
    pub rt_subcurve_id: Option<u32>,   // Which RT subcurve this represents
    
    // Quality flags
    pub qc_flags: Option<u16>,
}

// Detailed subcurve data (separate table)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubcurveDataRecord {
    pub well_id: u32,
    pub depth: f64,
    pub subcurve_id: u32,
    pub value: f32,
    pub qc_flag: i8,
}
```

### 4. Metadata Tables

```rust
// Global subcurve definitions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubcurveDefinition {
    pub subcurve_id: u32,
    pub main_curve_type: MainCurveType,
    pub subcurve_name: String,              // e.g., "RESDEEP"
    pub display_name: String,               // e.g., "Deep Resistivity"
    pub description: String,                // e.g., "Deep laterolog resistivity"
    pub standard_units: String,             // e.g., "ohm-m"
    pub vendor_variations: Vec<String>,     // e.g., ["RESDEEP", "RT_DEEP", "DEEP_RES"]
    pub expected_range: Option<(f32, f32)>, // e.g., (0.1, 1000.0) for resistivity
}

// Well-specific curve mappings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WellCurveMapping {
    pub well_id: u32,
    pub raw_curve_name: String,             // From LAS file
    pub subcurve_id: u32,                   // Maps to SubcurveDefinition
    pub units: String,                      // Actual units in this well
    pub conversion_factor: f64,             // Convert to standard units
    pub min_depth: f64,
    pub max_depth: f64,
    pub data_points: u32,
    pub null_value: f64,
    pub step: f64,
}
```

## Storage Architecture

### File Structure

```
s3://mudrock-storage/
├── wells/
│   ├── well_id=1/
│   │   ├── data.parquet                    # Main curves (GR, RHOB, etc.)
│   │   ├── subcurves.parquet              # Detailed subcurve data
│   │   └── metadata.parquet               # Curve mappings + well info
│   ├── well_id=2/
│   │   ├── data.parquet
│   │   ├── subcurves.parquet
│   │   └── metadata.parquet
│   └── well_id=3/
│       ├── data.parquet
│       ├── subcurves.parquet
│       └── metadata.parquet
├── curve_mappings/
│   ├── subcurve_definitions.parquet       # Global subcurve definitions
│   └── vendor_variations.parquet          # Curve name mappings
```

### Parquet Optimization Settings

- **Row group size**: 100,000 rows (optimal for DuckDB)
- **Compression**: SNAPPY (fast decompression)
- **File size**: 100MB - 10GB per file (DuckDB recommendation)
- **Partitioning**: By well_id for parallel processing

## Query Performance Examples

### 1. Single Well, Depth Range Query

```sql
-- Very fast: DuckDB can skip entire row groups
SELECT depth, gr, rhob, nphi
FROM 's3://mudrock-storage/wells/well_id=1/data.parquet'
WHERE depth BETWEEN 1000 AND 2000
ORDER BY depth;
```

### 2. Multi-Well, Single Curve Query

```sql
-- Fast: DuckDB processes multiple files in parallel
SELECT well_id, depth, gr
FROM 's3://mudrock-storage/wells/*/data.parquet'
WHERE depth BETWEEN 1000 AND 2000
ORDER BY well_id, depth;
```

### 3. All Resistivity Subcurves Query

```sql
-- Efficient: Join with metadata to get all resistivity variants
SELECT 
    s.well_id, 
    s.depth, 
    s.value, 
    s.subcurve_id,
    c.subcurve_name,
    c.display_name
FROM 's3://mudrock-storage/wells/*/subcurves.parquet' s
JOIN 's3://mudrock-storage/curve_mappings/subcurve_definitions.parquet' c
ON s.subcurve_id = c.subcurve_id
WHERE c.main_curve_type = 'RT' AND s.depth BETWEEN 1000 AND 2000
ORDER BY s.well_id, s.depth, s.subcurve_id;
```

## Benefits

### 1. Performance
- ✅ **No depth duplication**: Single depth column per well
- ✅ **Optimal row groups**: 1M records fits perfectly in 1-2 row groups
- ✅ **Fast depth queries**: DuckDB can skip entire row groups based on depth ranges
- ✅ **Parallel processing**: DuckDB processes multiple wells simultaneously

### 2. Flexibility
- ✅ **User-defined curve names**: LAS files can have any curve names
- ✅ **Automatic mapping**: System detects and maps to standard subcurves
- ✅ **Extensible**: Easy to add new subcurve types

### 3. Storage Efficiency
- ✅ **No empty columns**: Unlike wide format
- ✅ **Better compression**: Normalized data compresses better
- ✅ **Smaller file sizes**: DuckDB's bitpacking optimization

### 4. Data Integrity
- ✅ **Standardized units**: Automatic unit conversion
- ✅ **Quality tracking**: QC flags for each measurement
- ✅ **Metadata consistency**: Centralized curve definitions

## LAS File Upload Flow

### Current CSV Flow (DuckDB-based)
```
CSV File → DuckDB read_csv_auto() → DuckDB COPY TO PARQUET → MinIO Storage
```

### Implemented LAS Flow (Rust + DuckDB)
```
LAS File → las-parser crate → CurveMappingDictionary → User Confirmation UI → Schema Conversion → DuckDB Parquet → MinIO Storage
```

**Current Implementation Status:**
- ✅ **LAS File Parsing**: `las-types` crate handles LAS 1.2, 2.0, 3.0 files
- ✅ **Curve Detection**: Dictionary-based mapping with `CurveMappingDictionary`
- ✅ **Functional Operations**: Pure functions for curve mapping operations
- ✅ **Performance**: < 20µs for batch curve updates, 37ms for large LAS parsing
- ⚠️ **User Confirmation UI**: Pending frontend implementation
- ⚠️ **DuckDB Integration**: Pending integration with existing Parquet generation

### Detailed LAS Processing Pipeline

#### Phase 1: LAS File Processing (Rust) - ✅ IMPLEMENTED
```rust
// crates/search/document_loading/las-parser/src/las_parser.rs
impl LasParser {
    pub fn process_las_file_for_confirmation(
        &self,
        las_file: &LASFile,
        well_name: &str,
    ) -> Result<LasProcessingResult, Box<dyn std::error::Error>> {
        let mut curve_detections = Vec::new();
        let mut unmapped_curves = Vec::new();
        
        for column in &las_file.curves.columns {
            let is_mapped = self.curve_mapping_dict.is_mapped(&column.mnemonic);
            let detected_main_type = self.curve_mapping_dict.get_main_curve_type(&column.mnemonic);
            
            if !is_mapped {
                unmapped_curves.push(column.mnemonic.clone());
            }
            
            let detection = CurveDetection {
                raw_name: column.mnemonic.clone(),
                detected_main_type: if is_mapped { Some(detected_main_type) } else { None },
                suggested_subcurve: self.suggest_subcurve_name(&column.mnemonic),
                units: column.unit.clone(),
                confidence: if is_mapped { 0.9 } else { 0.0 },
                vendor_variations: self.get_vendor_variations(&column.mnemonic),
                is_mapped,
            };
            curve_detections.push(detection);
        }
        
        Ok(LasProcessingResult {
            well_metadata: self.extract_well_metadata(las_file, well_name),
            curve_detections,
            requires_user_confirmation: !unmapped_curves.is_empty(),
            unmapped_curves,
        })
    }
}
```

**Key Features Implemented:**
- ✅ **Dictionary-based curve detection** using `CurveMappingDictionary`
- ✅ **Automatic curve type mapping** for common petrophysical measurements
- ✅ **Unmapped curve identification** for user confirmation
- ✅ **Performance optimization** with functional approach
- ✅ **Comprehensive metadata extraction** from LAS files

#### Curve Mapping Operations - ✅ IMPLEMENTED
```rust
// crates/search/document_loading/las-types/src/curve_mapping_ops.rs

// Update single curve mapping
pub fn update_curve_mapping(
    las_file: &LASFile,
    curve_name: &str,
    new_main_type: MainCurveType,
    mapping_dict: &mut CurveMappingDictionary,
) -> Result<Option<MainCurveType>, String>

// Batch update multiple curves
pub fn batch_update_curve_mappings(
    las_file: &LASFile,
    updates: &[(String, MainCurveType)],
    mapping_dict: &mut CurveMappingDictionary,
) -> Vec<(String, Option<MainCurveType>, bool)>

// Get mapping statistics
pub fn get_mapping_statistics(
    las_file: &LASFile,
    mapping_dict: &CurveMappingDictionary,
) -> CurveMappingStats
```

**Performance Metrics:**
- **Single curve update**: < 1µs
- **Batch operations**: 8 curves in 19.2µs
- **Large file parsing**: 29K+ lines in 37ms
- **Memory efficiency**: Minimal allocations, functional approach

#### Phase 2: User Confirmation UI (SvelteKit)
```svelte
<!-- src/lib/components/ingestion/las-confirmation.svelte -->
<script lang="ts">
  export let processingResult: LasProcessingResult;
  export let onConfirmation: (mapping: LasMapping) => void;
  
  let wellName: string = processingResult.well_metadata.suggested_name;
  let curveMappings: Map<string, CurveMapping> = new Map();
  
  function confirmMapping() {
    const mapping: LasMapping = {
      well_name: wellName,
      curve_mappings: Object.fromEntries(curveMappings),
      units_conversions: getUnitsConversions(),
    };
    onConfirmation(mapping);
  }
</script>

<div class="las-confirmation-container">
  <h3>Confirm LAS File Processing</h3>
  
  <!-- Well Information -->
  <div class="well-section">
    <label>Well Name:</label>
    <input bind:value={wellName} />
  </div>
  
  <!-- Curve Mapping -->
  <div class="curve-mapping-section">
    <h4>Map Your Curves</h4>
    {#each processingResult.curve_detections as detection}
      <div class="curve-mapping-row">
        <div class="detected-curve">
          <strong>{detection.raw_name}</strong>
          <span class="units">({detection.units})</span>
        </div>
        <div class="mapping-select">
          <select bind:value={curveMappings[detection.raw_name]}>
            <option value="">Skip this curve</option>
            <option value="gr">Gamma Ray (GR)</option>
            <option value="rhob">Bulk Density (RHOB)</option>
            <option value="nphi">Neutron Porosity (NPHI)</option>
            <option value="rt">Resistivity (RT)</option>
            <!-- Add more options -->
          </select>
        </div>
        <div class="subcurve-mapping">
          <select bind:value={curveMappings[detection.raw_name].subcurve}>
            <option value="primary">Primary</option>
            <option value="resdeep">Deep Resistivity</option>
            <option value="resshallow">Shallow Resistivity</option>
            <option value="gr_thorium">GR Thorium</option>
            <!-- Add more subcurve options -->
          </select>
        </div>
      </div>
    {/each}
  </div>
  
  <button on:click={confirmMapping}>Confirm & Process</button>
</div>
```

#### Phase 3: Schema Conversion (Rust)
```rust
// src-tauri/src/ingestion/las_to_schema_converter.rs
impl LasToSchemaConverter {
    pub async fn convert_las_to_normalized_data(
        &self,
        las_file: &LASFile,
        mapping: &LasMapping,
    ) -> Result<NormalizedWellData, Box<dyn std::error::Error + Send + Sync>> {
        let mut main_curve_data = Vec::new();
        let mut subcurve_data = Vec::new();
        let mut curve_metadata = Vec::new();
        
        // Get depth array (single source of truth)
        let depth_count = las_file.curves.columns.first()
            .map(|col| col.values.len())
            .unwrap_or(0);
        
        let depths: Vec<f64> = (0..depth_count)
            .map(|i| las_file.well.start_depth + (i as f64 * las_file.well.step.unwrap_or(0.1524)))
            .collect();
        
        // Process each curve according to user mapping
        for (raw_curve_name, curve_data) in &las_file.curves.columns {
            if let Some(curve_mapping) = mapping.curve_mappings.get(raw_curve_name) {
                let subcurve_info = self.get_subcurve_info(curve_mapping)?;
                
                // Convert data points
                for (depth_idx, depth) in depths.iter().enumerate() {
                    if depth_idx < curve_data.values.len() {
                        let value = curve_data.values[depth_idx];
                        if !value.is_nan() && value != las_file.well.null_value {
                            // Add to subcurve data
                            subcurve_data.push(SubcurveDataRecord {
                                well_id: 0, // Will be assigned later
                                depth: *depth,
                                subcurve_id: subcurve_info.subcurve_id,
                                value: (value * curve_mapping.conversion_factor) as f32,
                                qc_flag: 0,
                            });
                            
                            // Add to main curve data if primary
                            if subcurve_info.is_primary {
                                self.add_to_main_curve_data(
                                    &mut main_curve_data,
                                    *depth,
                                    subcurve_info.main_curve_type,
                                    subcurve_info.subcurve_id,
                                    value as f32,
                                );
                            }
                        }
                    }
                }
                
                // Create curve metadata
                curve_metadata.push(WellCurveMapping {
                    well_id: 0, // Will be assigned later
                    raw_curve_name: raw_curve_name.clone(),
                    subcurve_id: subcurve_info.subcurve_id,
                    units: curve_mapping.units.clone(),
                    conversion_factor: curve_mapping.conversion_factor,
                    min_depth: las_file.well.start_depth,
                    max_depth: las_file.well.stop_depth,
                    data_points: curve_data.values.len() as u32,
                    null_value: las_file.well.null_value,
                    step: las_file.well.step.unwrap_or(0.1524),
                });
            }
        }
        
        Ok(NormalizedWellData {
            main_curve_data,
            subcurve_data,
            curve_metadata,
        })
    }
}
```

#### Phase 4: DuckDB Parquet Generation (Leveraging Current Flow)
```rust
// src-tauri/src/ingestion/las_parquet_generator.rs
impl LasParquetGenerator {
    pub async fn generate_parquet_files(
        &self,
        normalized_data: &NormalizedWellData,
        well_id: u32,
        well_name: &str,
    ) -> Result<ParquetFiles, Box<dyn std::error::Error + Send + Sync>> {
        let conn = Connection::open_in_memory()?;
        
        // 1. Create main curve data table
        self.create_main_curve_table(&conn)?;
        self.insert_main_curve_data(&conn, &normalized_data.main_curve_data, well_id)?;
        
        // 2. Create subcurve data table
        self.create_subcurve_table(&conn)?;
        self.insert_subcurve_data(&conn, &normalized_data.subcurve_data, well_id)?;
        
        // 3. Create metadata table
        self.create_metadata_table(&conn)?;
        self.insert_metadata(&conn, &normalized_data.curve_metadata, well_id, well_name)?;
        
        // 4. Generate Parquet files using DuckDB (similar to current CSV flow)
        let main_data_parquet = self.export_to_parquet(&conn, "main_curve_data", "main_data.parquet")?;
        let subcurve_parquet = self.export_to_parquet(&conn, "subcurve_data", "subcurves.parquet")?;
        let metadata_parquet = self.export_to_parquet(&conn, "curve_metadata", "metadata.parquet")?;
        
        Ok(ParquetFiles {
            main_data: main_data_parquet,
            subcurves: subcurve_parquet,
            metadata: metadata_parquet,
        })
    }
    
    fn export_to_parquet(
        &self,
        conn: &Connection,
        table_name: &str,
        file_name: &str,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
        // Use DuckDB's optimized Parquet export (similar to current CSV flow)
        conn.execute(
            &format!(
                "COPY {} TO '{}' (FORMAT PARQUET, ROW_GROUP_SIZE 100000, COMPRESSION SNAPPY)",
                table_name, file_name
            ),
            [],
        )?;
        
        let parquet_data = std::fs::read(file_name)?;
        let _ = std::fs::remove_file(file_name);
        Ok(parquet_data)
    }
}
```

#### Phase 5: Storage Upload (Reuse Current MinIO Flow)
```rust
// src-tauri/src/ingestion/las_upload_manager.rs
impl LasUploadManager {
    pub async fn upload_las_to_storage(
        &self,
        parquet_files: &ParquetFiles,
        well_id: u32,
        bucket_name: &str,
    ) -> Result<LasUploadResult, Box<dyn std::error::Error + Send + Sync>> {
        let upload_manager = MinioUploadManager::new().await?;
        
        // Upload main data
        let main_data_result = upload_manager.upload_file(
            bucket_name,
            &format!("wells/well_id={}/data.parquet", well_id),
            &parquet_files.main_data,
        ).await?;
        
        // Upload subcurves
        let subcurves_result = upload_manager.upload_file(
            bucket_name,
            &format!("wells/well_id={}/subcurves.parquet", well_id),
            &parquet_files.subcurves,
        ).await?;
        
        // Upload metadata
        let metadata_result = upload_manager.upload_file(
            bucket_name,
            &format!("wells/well_id={}/metadata.parquet", well_id),
            &parquet_files.metadata,
        ).await?;
        
        Ok(LasUploadResult {
            success: true,
            well_id,
            main_data_path: main_data_result.file_path,
            subcurves_path: subcurves_result.file_path,
            metadata_path: metadata_result.file_path,
        })
    }
}
```

### Key Benefits of LAS Flow

#### 1. Leverages Current Infrastructure
- ✅ **Reuses DuckDB Parquet generation** from current CSV flow
- ✅ **Reuses MinIO upload logic** from current system
- ✅ **Maintains performance optimizations** (row group sizing, compression)

#### 2. Handles LAS Complexity
- ✅ **Rust LAS parser** handles LAS file format intricacies
- ✅ **User confirmation** ensures correct curve mapping
- ✅ **Schema validation** ensures data quality

#### 3. DuckDB Performance Benefits
- ✅ **Optimal row group sizing** (100K rows per group)
- ✅ **SNAPPY compression** for fast decompression
- ✅ **Columnar efficiency** for analytics queries

#### 4. Flexible Architecture
- ✅ **Modular design** - each phase can be tested independently
- ✅ **Extensible** - easy to add new file formats
- ✅ **User-friendly** - clear confirmation UI

## Implementation Phases

### ✅ Phase 1: Core Schema Implementation (COMPLETED)
- [x] Define main curve types and subcurve mapping structures
- [x] Implement LAS file processing with subcurve detection
- [x] Create `CurveMappingDictionary` with user-configurable mappings
- [x] Implement functional curve mapping operations
- [x] Test with real LAS files (8 curves, 29K+ lines)

### ✅ Phase 2: LAS Parser Integration (COMPLETED)
- [x] Implement Rust LAS parser integration (`las-parser` crate)
- [x] Create curve detection and mapping system
- [x] Add batch curve mapping operations
- [x] Implement performance-optimized functional approach
- [x] Test with comprehensive test suite

### Phase 3: User Confirmation UI (PENDING)
- [ ] Create user confirmation UI for curve mapping
- [ ] Build schema conversion logic (LAS → normalized data)
- [ ] Integrate with existing DuckDB Parquet generation
- [ ] Test with medium dataset (10-20 wells)

### Phase 4: Storage Architecture
- [ ] Implement partitioned storage structure (per-well files)
- [ ] Create metadata management system
- [ ] Add curve name mapping and vendor variation detection
- [ ] Test with medium dataset (10-20 wells)

### Phase 5: Query Engine
- [ ] Implement hierarchical query engine
- [ ] Add main curve and subcurve-specific query methods
- [ ] Create batch operations for multi-well analytics
- [ ] Performance testing and optimization

### Phase 6: Advanced Features
- [ ] Add data quality validation and QC flagging
- [ ] Implement real-time curve mapping UI
- [ ] Add cross-well analytics and statistics
- [ ] Create visualization-ready data export formats

### Phase 7: Production Optimization
- [ ] Implement caching for frequently accessed data
- [ ] Add monitoring and performance metrics
- [ ] Create backup and recovery procedures
- [ ] Scale testing with large datasets (100+ wells)

## Success Metrics

- **Query Performance**: < 100ms for single-well depth range queries
- **Storage Efficiency**: < 10MB per well for typical 3000m well
- **Scalability**: Support for 1000+ wells with < 1s query times
- **Data Integrity**: 100% accurate curve mapping and unit conversion
- **User Experience**: Intuitive queries for both main curves and subcurves
