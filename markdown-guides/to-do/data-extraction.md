# Data Extraction Architecture Plan for MudRock MVP & Rust DAG

This document outlines a comprehensive data extraction architecture that integrates with our current modular storage system and optimizes for high-performance operations using Arrow and DuckDB.

## 🎯 Current Architecture Analysis

### **Current LAS File Upload & Storage Process**

Based on the LAS file upload implementation, we currently have:

1. **File Structure**: `project-{id}/wells/{well-id}/logs_{log-type}.parquet`
2. **Schema**: Wide format with `well_name`, `depth`, `curve_id`, `value` columns
3. **Storage**: OpenDAL integration with MinIO/S3-compatible storage
4. **Metadata**: PostgreSQL tables for well and curve metadata

### **Current Schema Analysis**

```rust
// Current parquet schema (from las-to-parquet conversion)
Schema::new(vec![
    Field::new("well_name", DataType::Utf8, false),
    Field::new("depth", DataType::Float64, false), 
    Field::new("curve_id", DataType::UInt32, false),
    Field::new("value", DataType::Float64, false),
])
```

**Issues with Current Schema for High-Performance Extraction:**

1. **Normalized Structure**: Current schema stores each depth/curve combination as a separate row
2. **No Columnar Optimization**: Curve data is not stored in separate columns for efficient filtering
3. **Limited DuckDB Optimization**: Current structure doesn't leverage DuckDB's columnar advantages
4. **Cross-Well Queries**: Difficult to efficiently query multiple wells simultaneously

## 🚀 Optimized Data Extraction Architecture

### **Phase 1: Hybrid Schema Design (Well-by-Well + Cross-Well Optimization)**

Given the reality of well log data:
- **5,000m depth** with **0.1524m spacing** = ~32,800 data points per well
- **5-100 curves** per well with **different curve types** across wells
- **Different depth ranges** per well

#### **A) Per-Well Wide-Format Schema (Primary Storage)**

```rust
// Optimized wide-format schema PER WELL (not cross-well)
// File: project-{id}/wells/{well-id}/logs_{log-type}.parquet
Schema::new(vec![
    Field::new("depth", DataType::Float64, false),           // Primary key
    Field::new("gr", DataType::Float64, true),               // Gamma Ray
    Field::new("rhob", DataType::Float64, true),             // Density  
    Field::new("nphi", DataType::Float64, true),             // Neutron Porosity
    Field::new("rt", DataType::Float64, true),               // Resistivity
    Field::new("cali", DataType::Float64, true),             // Caliper
    Field::new("sp", DataType::Float64, true),               // Spontaneous Potential
    Field::new("dt", DataType::Float64, true),                // Sonic
    // ... other curves as needed for this specific well
])
```

**Benefits:**
- **No Sparse Matrix Problem**: Each well only stores curves it actually has
- **Optimal Compression**: Parquet compression works well with per-well data
- **Fast Single-Well Queries**: Direct access to well-specific data
- **Memory Efficient**: Only load required wells into memory

#### **B) Cross-Well Query Schema (On-Demand)**

```rust
// Virtual schema for cross-well queries (generated dynamically)
Schema::new(vec![
    Field::new("well_id", DataType::Utf8, false),            // Well identifier
    Field::new("depth", DataType::Float64, false),           // Depth
    Field::new("curve_name", DataType::Utf8, false),         // Curve name
    Field::new("value", DataType::Float64, true),            // Curve value
    Field::new("unit", DataType::Utf8, true),                // Curve unit
])
```

**Benefits:**
- **Flexible Cross-Well Analysis**: Can query any combination of wells/curves
- **No NULL Overhead**: Only stores actual data points
- **Dynamic Schema**: Adapts to available curves per well

#### **C) Well Curve Registry Schema**

```rust
// Registry of available curves per well (stored in PostgreSQL)
Schema::new(vec![
    Field::new("well_id", DataType::Utf8, false),
    Field::new("curve_name", DataType::Utf8, false),
    Field::new("unit", DataType::Utf8, true),
    Field::new("description", DataType::Utf8, true),
    Field::new("min_depth", DataType::Float64, true),
    Field::new("max_depth", DataType::Float64, true),
    Field::new("data_points", DataType::UInt32, true),
    Field::new("null_count", DataType::UInt32, true),
    Field::new("min_value", DataType::Float64, true),
    Field::new("max_value", DataType::Float64, true),
])
```

#### **D) Well Metadata Schema**

```rust
// Well metadata schema
Schema::new(vec![
    Field::new("well_id", DataType::Utf8, false),
    Field::new("well_name", DataType::Utf8, false),
    Field::new("field", DataType::Utf8, true),
    Field::new("company", DataType::Utf8, true),
    Field::new("location", DataType::Utf8, true),
    Field::new("uwi", DataType::Utf8, true),
    Field::new("total_depth", DataType::Float64, true),
    Field::new("start_depth", DataType::Float64, true),
    Field::new("stop_depth", DataType::Float64, true),
    Field::new("curve_count", DataType::UInt32, true),
    Field::new("created_at", DataType::Timestamp(TimeUnit::Millisecond, None), false),
])
```

### **Phase 2: Modular Data Extraction Components**

#### **A) Core Extraction Engine**

```rust
// crates/data-extraction/
├── src/
│   ├── lib.rs
│   ├── extractor/
│   │   ├── mod.rs
│   │   ├── well_extractor.rs          // Single well extraction
│   │   ├── multi_well_extractor.rs    // Cross-well extraction
│   │   └── curve_extractor.rs         // Curve-specific extraction
│   ├── filters/
│   │   ├── mod.rs
│   │   ├── depth_filter.rs           // Depth range filtering
│   │   ├── curve_filter.rs           // Curve value filtering
│   │   └── well_filter.rs            // Well selection filtering
│   ├── aggregators/
│   │   ├── mod.rs
│   │   ├── statistics.rs             // Statistical aggregations
│   │   ├── interpolators.rs          // Data interpolation
│   │   └── quality_checkers.rs       // Data quality assessment
│   ├── exporters/
│   │   ├── mod.rs
│   │   ├── parquet_exporter.rs       // Export to parquet
│   │   ├── arrow_exporter.rs         // Export to Arrow RecordBatch
│   │   └── csv_exporter.rs           // Export to CSV
│   └── cache/
│       ├── mod.rs
│       ├── query_cache.rs            // Query result caching
│       └── metadata_cache.rs         // Metadata caching
```

#### **B) Extraction Query Builder**

```rust
pub struct DataExtractionQuery {
    pub project_id: String,
    pub well_ids: Vec<String>,
    pub log_types: Vec<String>,
    pub depth_range: Option<(f64, f64)>,
    pub curve_filters: HashMap<String, CurveFilter>,
    pub aggregation: Option<AggregationConfig>,
    pub output_format: OutputFormat,
    pub cache_key: Option<String>,
}

pub struct CurveFilter {
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
    pub exclude_nulls: bool,
    pub interpolation_method: Option<InterpolationMethod>,
}
```

### **Phase 3: High-Performance Query Engine**

#### **A) DuckDB Integration Layer**

```rust
// crates/data-extraction/src/query_engine/
pub struct DuckDBExtractionEngine {
    connection: Connection,
    s3_config: S3Config,
    cache_manager: QueryCacheManager,
}

impl DuckDBExtractionEngine {
    /// Register parquet files as virtual tables
    pub async fn register_well_tables(&mut self, project_id: &str, well_ids: &[String]) -> Result<()> {
        for well_id in well_ids {
            let s3_uri = format!("s3://project-{}/wells/{}/logs_*.parquet", project_id, well_id);
            let table_name = format!("well_{}", well_id);
            self.connection.execute(&format!("CREATE VIEW {} AS SELECT * FROM read_parquet('{}')", table_name, s3_uri))?;
        }
        Ok(())
    }
    
    /// Execute optimized extraction query
    pub async fn execute_extraction(&mut self, query: &DataExtractionQuery) -> Result<RecordBatch> {
        let sql = self.build_optimized_sql(query)?;
        let result = self.connection.execute(&sql)?;
        Ok(result)
    }
}
```

#### **B) Optimized SQL Generation for Well-by-Well Storage**

```sql
-- Example 1: Single well query (fast - direct parquet access)
SELECT 
    depth,
    gr,
    rhob,
    nphi,
    rt
FROM read_parquet('s3://project-123/wells/well_001/logs_composite.parquet')
WHERE depth BETWEEN 1000.0 AND 2000.0
  AND gr IS NOT NULL
  AND rhob IS NOT NULL;

-- Example 2: Cross-well analysis (normalized on-demand)
WITH well_001_data AS (
    SELECT 'well_001' as well_id, depth, 'gr' as curve_name, gr as value, 'gAPI' as unit
    FROM read_parquet('s3://project-123/wells/well_001/logs_composite.parquet')
    WHERE depth BETWEEN 1000.0 AND 2000.0 AND gr IS NOT NULL
),
well_002_data AS (
    SELECT 'well_002' as well_id, depth, 'gr' as curve_name, gr as value, 'gAPI' as unit
    FROM read_parquet('s3://project-123/wells/well_002/logs_composite.parquet')
    WHERE depth BETWEEN 1000.0 AND 2000.0 AND gr IS NOT NULL
),
combined_data AS (
    SELECT * FROM well_001_data
    UNION ALL
    SELECT * FROM well_002_data
)
SELECT 
    well_id,
    AVG(value) as avg_gr,
    MIN(depth) as min_depth,
    MAX(depth) as max_depth,
    COUNT(*) as data_points
FROM combined_data
GROUP BY well_id
ORDER BY avg_gr DESC;

-- Example 3: Multi-curve cross-well analysis
WITH well_curves AS (
    -- Dynamically generated based on available curves per well
    SELECT 'well_001' as well_id, depth, 'gr' as curve_name, gr as value FROM read_parquet('s3://project-123/wells/well_001/logs_composite.parquet') WHERE gr IS NOT NULL
    UNION ALL
    SELECT 'well_001' as well_id, depth, 'rhob' as curve_name, rhob as value FROM read_parquet('s3://project-123/wells/well_001/logs_composite.parquet') WHERE rhob IS NOT NULL
    UNION ALL
    SELECT 'well_002' as well_id, depth, 'gr' as curve_name, gr as value FROM read_parquet('s3://project-123/wells/well_002/logs_composite.parquet') WHERE gr IS NOT NULL
    -- ... more wells and curves as needed
)
SELECT 
    well_id,
    curve_name,
    AVG(value) as avg_value,
    MIN(depth) as min_depth,
    MAX(depth) as max_depth,
    COUNT(*) as data_points
FROM well_curves
WHERE depth BETWEEN 1000.0 AND 2000.0
GROUP BY well_id, curve_name
ORDER BY well_id, curve_name;
```

### **Phase 4: Performance Optimizations for Well-by-Well Storage**

#### **A) Parquet File Optimization for Well Log Data**

Based on DuckDB performance guidelines and well log characteristics:

```rust
pub struct WellLogParquetConfig {
    pub row_group_size: usize,        // Optimized for ~32K data points per well
    pub compression: CompressionType, // Optimized for well log data
    pub file_size_mb: usize,         // Per-well file size target
    pub column_encoding: ColumnEncoding,
    pub depth_precision: u8,         // Decimal places for depth values
}

// Optimal settings for well log data (5K depth, 0.1524m spacing, 5-100 curves)
impl Default for WellLogParquetConfig {
    fn default() -> Self {
        Self {
            row_group_size: 32_000,   // ~1 well worth of data per row group
            compression: CompressionType::Zstd, // Better compression for repetitive data
            file_size_mb: 50,         // Smaller files for better parallelization
            column_encoding: ColumnEncoding::Delta, // Excellent for depth data
            depth_precision: 4,       // 0.0001m precision (0.1mm)
        }
    }
}

// For wells with many curves (50-100), use larger row groups
impl WellLogParquetConfig {
    pub fn for_high_curve_count() -> Self {
        Self {
            row_group_size: 16_000,   // Smaller row groups for many columns
            compression: CompressionType::Zstd,
            file_size_mb: 100,        // Larger files acceptable for high curve count
            column_encoding: ColumnEncoding::Delta,
            depth_precision: 4,
        }
    }
}
```

#### **B) Storage Strategy Optimization**

```rust
pub struct WellStorageStrategy {
    pub max_curves_per_file: usize,  // Split files if too many curves
    pub depth_chunking: bool,        // Split by depth ranges for very deep wells
    pub curve_grouping: CurveGrouping, // Group related curves together
}

pub enum CurveGrouping {
    AllInOne,           // All curves in single file
    ByType,             // Separate files by curve type (gr, resistivity, etc.)
    ByDepthRange,       // Split by depth ranges (0-2500m, 2500-5000m)
    Hybrid,             // Combination based on well characteristics
}

impl WellStorageStrategy {
    pub fn determine_strategy(&self, curve_count: usize, depth_range: f64) -> CurveGrouping {
        match (curve_count, depth_range) {
            (curves, _) if curves <= 20 => CurveGrouping::AllInOne,
            (curves, depth) if curves > 50 || depth > 4000.0 => CurveGrouping::ByType,
            _ => CurveGrouping::Hybrid,
        }
    }
}
```

#### **C) Query Optimization Strategies for Well-by-Well Storage**

1. **Well-Level Filtering**: Query only required wells first, then filter curves
2. **Curve Registry Lookup**: Use PostgreSQL metadata to determine available curves per well
3. **Dynamic SQL Generation**: Build optimized queries based on available curves
4. **Parallel Well Processing**: Process multiple wells in parallel
5. **Depth Range Optimization**: Use Parquet statistics for depth filtering
6. **Curve-Specific Projection**: Only read required curve columns per well

```rust
pub struct WellQueryOptimizer {
    curve_registry: CurveRegistry,
    well_metadata: WellMetadataCache,
}

impl WellQueryOptimizer {
    /// Generate optimized query for cross-well analysis
    pub fn build_cross_well_query(
        &self,
        well_ids: &[String],
        curve_names: &[String],
        depth_range: Option<(f64, f64)>,
    ) -> Result<String> {
        let mut query_parts = Vec::new();
        
        for well_id in well_ids {
            // Check which curves are available for this well
            let available_curves = self.curve_registry.get_curves_for_well(well_id)?;
            let requested_curves: Vec<_> = curve_names
                .iter()
                .filter(|curve| available_curves.contains(curve))
                .collect();
            
            if !requested_curves.is_empty() {
                let well_query = self.build_single_well_query(
                    well_id,
                    &requested_curves,
                    depth_range,
                )?;
                query_parts.push(well_query);
            }
        }
        
        Ok(format!(
            "WITH well_data AS ({}) SELECT * FROM well_data ORDER BY well_id, depth",
            query_parts.join(" UNION ALL ")
        ))
    }
    
    fn build_single_well_query(
        &self,
        well_id: &str,
        curves: &[&String],
        depth_range: Option<(f64, f64)>,
    ) -> Result<String> {
        let curve_selects: Vec<String> = curves
            .iter()
            .map(|curve| {
                format!(
                    "SELECT '{}' as well_id, depth, '{}' as curve_name, {} as value FROM read_parquet('s3://project-{}/wells/{}/logs_composite.parquet')",
                    well_id, curve, curve, self.project_id, well_id
                )
            })
            .collect();
        
        let mut query = curve_selects.join(" UNION ALL ");
        
        if let Some((min_depth, max_depth)) = depth_range {
            query = format!(
                "SELECT * FROM ({}) WHERE depth BETWEEN {} AND {}",
                query, min_depth, max_depth
            );
        }
        
        Ok(query)
    }
}
```

#### **C) Memory Management**

```rust
pub struct MemoryOptimizationConfig {
    pub max_memory_mb: usize,        // Limit memory usage
    pub spill_to_disk: bool,         // Enable out-of-core processing
    pub batch_size: usize,           // Process data in batches
    pub cache_size_mb: usize,       // Query result cache size
}
```

### **Phase 5: Integration with Current Architecture**

#### **A) Storage Layer Integration**

```rust
// Extend existing WellStorageManager
impl WellStorageManager {
    /// Extract data using optimized extraction engine
    pub async fn extract_well_data(
        &self,
        project_id: &str,
        extraction_query: DataExtractionQuery,
    ) -> Result<ExtractionResult> {
        let mut engine = DuckDBExtractionEngine::new(self.s3_config.clone())?;
        engine.register_well_tables(project_id, &extraction_query.well_ids).await?;
        let result = engine.execute_extraction(&extraction_query).await?;
        Ok(ExtractionResult::new(result, extraction_query))
    }
}
```

#### **B) Tauri Command Integration**

```rust
// src-tauri/src/main.rs
#[tauri::command]
async fn extract_well_data(
    project_id: String,
    well_ids: Vec<String>,
    log_types: Vec<String>,
    depth_range: Option<(f64, f64)>,
    curve_filters: HashMap<String, CurveFilter>,
) -> Result<ExtractionResult, String> {
    let storage_manager = WellStorageManager::new().await?;
    let query = DataExtractionQuery {
        project_id,
        well_ids,
        log_types,
        depth_range,
        curve_filters,
        aggregation: None,
        output_format: OutputFormat::Arrow,
        cache_key: None,
    };
    
    storage_manager.extract_well_data(&query.project_id, query).await
        .map_err(|e| e.to_string())
}
```

### **Phase 6: Rust DAG Integration**

#### **A) DAG Node for Data Extraction**

```rust
// crates/dags/src/nodes/
pub struct DataExtractionNode {
    pub node_id: String,
    pub extraction_query: DataExtractionQuery,
    pub output_schema: SchemaRef,
}

impl DAGNode for DataExtractionNode {
    async fn execute(&self, context: &DAGContext) -> Result<RecordBatch> {
        let storage_manager = context.get_storage_manager()?;
        let result = storage_manager.extract_well_data(
            &self.extraction_query.project_id,
            self.extraction_query.clone(),
        ).await?;
        
        Ok(result.into_record_batch())
    }
    
    fn input_schemas(&self) -> Vec<SchemaRef> {
        vec![] // No inputs - this is a source node
    }
    
    fn output_schema(&self) -> SchemaRef {
        self.output_schema.clone()
    }
}
```

#### **B) Pipeline Integration**

```rust
// Example DAG pipeline for petrophysical analysis
pub fn create_petrophysical_analysis_pipeline() -> DAGPipeline {
    let mut pipeline = DAGPipeline::new();
    
    // Node 1: Extract well data
    let extraction_node = DataExtractionNode {
        node_id: "extract_wells".to_string(),
        extraction_query: DataExtractionQuery {
            project_id: "project_123".to_string(),
            well_ids: vec!["well_001".to_string(), "well_002".to_string()],
            log_types: vec!["composite".to_string()],
            depth_range: Some((1000.0, 2000.0)),
            curve_filters: HashMap::new(),
            aggregation: None,
            output_format: OutputFormat::Arrow,
            cache_key: None,
        },
        output_schema: create_well_logs_schema(),
    };
    
    // Node 2: Calculate shale volume
    let shale_volume_node = ShaleVolumeCalculationNode {
        node_id: "shale_volume".to_string(),
        gr_threshold: 75.0,
        input_schema: create_well_logs_schema(),
        output_schema: create_shale_volume_schema(),
    };
    
    // Node 3: Calculate porosity
    let porosity_node = PorosityCalculationNode {
        node_id: "porosity".to_string(),
        method: PorosityMethod::NeutronDensity,
        input_schema: create_well_logs_schema(),
        output_schema: create_porosity_schema(),
    };
    
    // Build pipeline
    pipeline.add_node(extraction_node);
    pipeline.add_node(shale_volume_node);
    pipeline.add_node(porosity_node);
    
    // Connect nodes
    pipeline.add_edge("extract_wells", "shale_volume")?;
    pipeline.add_edge("extract_wells", "porosity")?;
    
    pipeline
}
```

## 🎯 Implementation Roadmap

### **Week 1-2: Foundation**
- [ ] Implement enhanced parquet schema with wide format
- [ ] Create basic data extraction engine with DuckDB integration
- [ ] Implement depth and curve filtering

### **Week 3-4: Core Features**
- [ ] Add multi-well extraction capabilities
- [ ] Implement query optimization (filter pushdown, projection)
- [ ] Add caching layer for performance

### **Week 5-6: Advanced Features**
- [ ] Integrate with existing storage layer
- [ ] Add Tauri command integration
- [ ] Implement DAG node for data extraction

### **Week 7-8: Optimization & Testing**
- [ ] Performance optimization based on DuckDB guidelines
- [ ] Memory management and out-of-core processing
- [ ] Comprehensive testing and benchmarking

## 📊 Expected Performance Benefits for Well-by-Well Storage

### **Query Performance Improvements**
- **5-15x faster** single-well queries (direct parquet access, no joins)
- **3-8x faster** cross-well queries (normalized on-demand, no sparse matrices)
- **2-5x faster** depth range filtering (optimized row groups)
- **10-20x faster** curve-specific queries (projection pushdown per well)

### **Storage Efficiency**
- **60-90% reduction** in storage size (no NULL values, optimized compression)
- **Better compression** with Zstd on repetitive well log data
- **Smaller file sizes** (50-100MB per well) for better parallelization
- **No sparse matrix overhead** (each well only stores its actual curves)

### **Memory Efficiency**
- **70-90% reduction** in memory usage (load only required wells)
- **Per-well processing** (32K rows × curves vs. millions of sparse rows)
- **Intelligent caching** at well level reduces repeated overhead
- **Out-of-core processing** for large cross-well analyses

### **Scalability**
- **Linear scaling** with number of wells (process wells in parallel)
- **Independent well processing** (no cross-well dependencies)
- **Efficient metadata lookup** (PostgreSQL curve registry)
- **Dynamic query generation** (only query available curves per well)

## 🔧 Configuration & Usage

### **Basic Usage Example**

```rust
// Example 1: Single well extraction (fastest)
let single_well_query = DataExtractionQuery {
    project_id: "project_123".to_string(),
    well_ids: vec!["well_001".to_string()],
    curve_names: vec!["gr".to_string(), "rhob".to_string(), "nphi".to_string()],
    depth_range: Some((1000.0, 2000.0)),
    output_format: OutputFormat::Arrow,
    cache_key: Some("well_001_analysis".to_string()),
};

let single_result = storage_manager.extract_well_data(&single_well_query).await?;

// Example 2: Cross-well analysis (normalized on-demand)
let cross_well_query = DataExtractionQuery {
    project_id: "project_123".to_string(),
    well_ids: vec!["well_001".to_string(), "well_002".to_string(), "well_003".to_string()],
    curve_names: vec!["gr".to_string()], // Only GR available across all wells
    depth_range: Some((1000.0, 2000.0)),
    output_format: OutputFormat::Arrow,
    cache_key: Some("cross_well_gr_analysis".to_string()),
};

let cross_result = storage_manager.extract_well_data(&cross_well_query).await?;

// Example 3: Multi-curve analysis with curve registry lookup
let multi_curve_query = DataExtractionQuery {
    project_id: "project_123".to_string(),
    well_ids: vec!["well_001".to_string(), "well_002".to_string()],
    curve_names: vec!["gr".to_string(), "rhob".to_string(), "rt".to_string()],
    depth_range: Some((1000.0, 2000.0)),
    curve_filters: {
        let mut filters = HashMap::new();
        filters.insert("gr".to_string(), CurveFilter {
            min_value: Some(0.0),
            max_value: Some(200.0),
            exclude_nulls: true,
            interpolation_method: None,
        });
        filters
    },
    aggregation: Some(AggregationConfig::Statistics),
    output_format: OutputFormat::Arrow,
    cache_key: Some("multi_curve_analysis".to_string()),
};

let multi_result = storage_manager.extract_well_data(&multi_curve_query).await?;
```

### **DAG Integration Example**

```rust
// Use in DAG pipeline
let extraction_node = DataExtractionNode::new(query);
let pipeline = DAGPipeline::new()
    .add_node(extraction_node)
    .add_node(shale_volume_calculation)
    .add_edge("extract_wells", "shale_volume")?;

let result = pipeline.execute().await?;
```

## 🎯 **Key Decision: Well-by-Well Storage is Optimal**

### **Why Well-by-Well Storage is Better for Your Use Case**

Given your specifications:
- **5,000m depth** with **0.1524m spacing** = ~32,800 data points per well
- **5-100 curves** per well with **different curve types** across wells
- **Different depth ranges** per well

**Well-by-Well Storage Advantages:**
1. **No Sparse Matrix Problem**: Avoids 60-90% NULL values that would occur in combined storage
2. **Optimal Compression**: Parquet compression works much better with dense data
3. **Memory Efficiency**: Load only required wells (32K rows vs. millions of sparse rows)
4. **Parallel Processing**: Process wells independently in parallel
5. **Flexible Schema**: Each well can have its own curve schema
6. **Fast Single-Well Queries**: Direct parquet access without joins

**Cross-Well Analysis Solution:**
- **Normalize on-demand**: Convert to normalized format only when needed for cross-well analysis
- **Dynamic SQL Generation**: Build queries based on available curves per well
- **Curve Registry**: Use PostgreSQL metadata to determine available curves per well
- **Intelligent Caching**: Cache normalized results for repeated cross-well queries

### **Storage Strategy Summary**

```
project-{id}/
├── wells/
│   ├── well_001/
│   │   ├── logs_composite.parquet    # All curves for well_001 (wide format)
│   │   ├── logs_gr.parquet          # GR-specific data (if needed)
│   │   └── metadata.parquet         # Well-specific metadata
│   ├── well_002/
│   │   ├── logs_composite.parquet    # All curves for well_002 (different curves)
│   │   └── metadata.parquet
│   └── well_003/
│       ├── logs_composite.parquet    # All curves for well_003
│       └── metadata.parquet
└── catalogs/
    ├── wells_catalog.parquet        # Well metadata registry
    └── curves_catalog.parquet       # Curve availability per well
```

This architecture provides a solid foundation for high-performance data extraction that integrates seamlessly with your current modular approach while optimizing for DuckDB and Arrow performance characteristics, specifically addressing the sparse matrix problem you identified.
