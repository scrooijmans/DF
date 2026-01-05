# Database Design for Large Arrays: Low-Latency Storage and Retrieval

This document outlines optimal strategies for storing large array values (10,000s of points) referenced by PostgreSQL tables, with a focus on achieving the lowest possible latency for data retrieval.

## Problem Statement

For SciChart.js applications, we need to store large arrays of numeric data (X/Y values, time series, etc.) that:

- Contain 10,000+ data points per array
- Are referenced by PostgreSQL tables (metadata, relationships)
- Require low-latency retrieval for real-time visualization
- May be queried selectively (specific ranges, columns)

## Storage Architecture Options

### Option 1: Object Storage with Parquet (Recommended)

**Architecture:**

- **PostgreSQL**: Stores metadata and references (file paths, IDs)
- **Object Storage (MinIO/S3)**: Stores actual array data in Parquet format
- **Query Engine**: DuckDB or Apache Arrow for efficient Parquet queries

**Why This is Optimal:**

1. **Columnar Storage**: Parquet's columnar format enables:
   - Column pruning (read only needed columns)
   - Predicate pushdown (filter at storage level)
   - Efficient compression (10-100x compression ratios)
2. **Low Latency**:
   - Parallel column decoding
   - Selective column reading
   - Efficient compression (Snappy, ZSTD)
3. **Scalability**: Object storage scales independently from database

**Implementation:**

```sql
-- PostgreSQL table for metadata
CREATE TABLE chart_series (
    id SERIAL PRIMARY KEY,
    chart_id INTEGER REFERENCES charts(id),
    series_name TEXT NOT NULL,
    data_file_path TEXT NOT NULL,  -- Path in object storage
    data_format TEXT NOT NULL,      -- 'parquet', 'arrow', etc.
    point_count INTEGER,
    created_at TIMESTAMP DEFAULT NOW()
);

-- Index for fast lookups
CREATE INDEX idx_chart_series_chart_id ON chart_series(chart_id);
CREATE INDEX idx_chart_series_data_file ON chart_series(data_file_path);
```

**Parquet File Structure:**

```rust
// Using Apache Arrow Rust
use arrow::array::{Float64Array, StringArray};
use arrow::record_batch::RecordBatch;
use parquet::arrow::ArrowWriter;
use parquet::file::properties::WriterProperties;

// Create optimized writer properties
let writer_props = WriterProperties::builder()
    .set_compression(parquet::basic::Compression::SNAPPY)  // Fast compression
    .set_write_batch_size(64 * 1024)  // 64KB row groups
    .set_max_row_group_length(128 * 1024)  // 128K rows per group
    .set_dictionary_enabled(true)  // Dictionary encoding for better compression
    .build();

// Create schema: one column per array dimension
let schema = Schema::new(vec![
    Field::new("x", DataType::Float64, false),
    Field::new("y", DataType::Float64, false),
]);

// Write data
let x_values = Float64Array::from(vec![1.0, 2.0, 3.0, ...]);
let y_values = Float64Array::from(vec![10.0, 20.0, 30.0, ...]);
let batch = RecordBatch::try_new(schema, vec![Arc::new(x_values), Arc::new(y_values)])?;

let file = File::create("series_data.parquet")?;
let mut writer = ArrowWriter::try_new(file, schema, Some(writer_props))?;
writer.write(&batch)?;
writer.close()?;
```

**Retrieval with DuckDB:**

```rust
use duckdb::Connection;

// Query Parquet file directly from S3/MinIO
let conn = Connection::open_in_memory()?;
conn.execute("INSTALL httpfs", [])?;
conn.execute("LOAD httpfs", [])?;

// Configure S3 access
conn.execute(
    "CREATE SECRET s3_storage (
        TYPE S3,
        KEY_ID 'access_key',
        SECRET 'secret_key',
        ENDPOINT 'minio:9000',
        REGION 'us-east-1',
        URL_STYLE 'path',
        USE_SSL false
    )",
    [],
)?;

// Query specific columns only (column pruning)
let query = format!(
    "SELECT x, y FROM read_parquet('s3://bucket/series_data.parquet')
     WHERE x BETWEEN {} AND {}
     LIMIT {}",
    start_x, end_x, limit
);

let mut stmt = conn.prepare(&query)?;
let rows = stmt.query_map([], |row| {
    Ok((row.get::<_, f64>(0)?, row.get::<_, f64>(1)?))
})?;
```

### Option 2: PostgreSQL TOAST with Compressed Arrays

**Architecture:**

- Store arrays directly in PostgreSQL using `bytea` or `jsonb`
- PostgreSQL TOAST automatically handles large values
- Use compression (gzip, lz4) before storage

**Pros:**

- Simple implementation
- ACID transactions
- No external dependencies

**Cons:**

- Higher latency (full array retrieval)
- Limited query capabilities
- Database size grows significantly

**Implementation:**

```sql
-- Store compressed arrays in PostgreSQL
CREATE TABLE chart_series (
    id SERIAL PRIMARY KEY,
    chart_id INTEGER REFERENCES charts(id),
    series_name TEXT NOT NULL,
    x_values BYTEA NOT NULL,  -- Compressed Float64Array
    y_values BYTEA NOT NULL,  -- Compressed Float64Array
    compression_type TEXT DEFAULT 'snappy',
    point_count INTEGER,
    created_at TIMESTAMP DEFAULT NOW()
);

-- Use TOAST storage strategy
ALTER TABLE chart_series ALTER COLUMN x_values SET STORAGE EXTERNAL;
ALTER TABLE chart_series ALTER COLUMN y_values SET STORAGE EXTERNAL;
```

**Compression in Rust:**

```rust
use snap::raw::{Decoder, Encoder};

// Compress array before storage
fn compress_array(data: &[f64]) -> Vec<u8> {
    let bytes: Vec<u8> = data.iter()
        .flat_map(|&x| x.to_le_bytes())
        .collect();

    let mut encoder = Encoder::new();
    encoder.compress_vec(&bytes).unwrap()
}

// Decompress on retrieval
fn decompress_array(compressed: &[u8]) -> Vec<f64> {
    let mut decoder = Decoder::new();
    let decompressed = decoder.decompress_vec(compressed).unwrap();

    decompressed.chunks(8)
        .map(|chunk| f64::from_le_bytes(chunk.try_into().unwrap()))
        .collect()
}
```

### Option 3: Hybrid Approach (Metadata in PG, Data in Object Storage)

**Architecture:**

- PostgreSQL: Lightweight metadata and references
- Object Storage: Parquet files with full data
- Cache Layer: Redis/Memcached for frequently accessed data

**Implementation:**

```sql
-- Minimal metadata in PostgreSQL
CREATE TABLE chart_series (
    id SERIAL PRIMARY KEY,
    chart_id INTEGER REFERENCES charts(id),
    series_name TEXT NOT NULL,
    storage_path TEXT NOT NULL,  -- s3://bucket/path/to/file.parquet
    storage_type TEXT DEFAULT 'parquet',
    metadata JSONB,  -- Additional metadata (bounds, stats, etc.)
    cache_key TEXT,  -- For cache invalidation
    created_at TIMESTAMP DEFAULT NOW()
);

-- JSONB metadata for fast queries
CREATE INDEX idx_series_metadata ON chart_series USING GIN(metadata);
```

## Compression Strategies

### Compression Codec Comparison

Based on Apache Parquet benchmarks:

| Codec      | Compression Ratio | Speed     | Latency | Use Case                        |
| ---------- | ----------------- | --------- | ------- | ------------------------------- |
| **SNAPPY** | 2-3x              | Very Fast | Lowest  | **Recommended for low latency** |
| **ZSTD**   | 3-5x              | Fast      | Low     | Best balance                    |
| **GZIP**   | 4-6x              | Medium    | Medium  | Higher compression needed       |
| **LZ4**    | 2-3x              | Very Fast | Low     | Alternative to Snappy           |
| **BROTLI** | 5-8x              | Slow      | High    | Maximum compression             |

**Recommendation: Use SNAPPY for lowest latency**

```rust
use parquet::basic::Compression;

let writer_props = WriterProperties::builder()
    .set_compression(Compression::SNAPPY)  // Best for low latency
    .set_dictionary_enabled(true)  // Improves compression for repeated values
    .set_write_batch_size(64 * 1024)  // Optimize for read performance
    .build();
```

### Encoding Strategies

**1. Dictionary Encoding:**

- Best for columns with repeated values
- Reduces storage size significantly
- Minimal performance impact

**2. Delta Encoding:**

- Best for sorted/sequential data (time series)
- Excellent compression for incremental values
- Fast encoding/decoding

**3. Byte Stream Split:**

- Best for floating-point data
- Improves compression ratio
- No size reduction, but better compression

```rust
// Enable dictionary encoding for categorical data
let writer_props = WriterProperties::builder()
    .set_dictionary_enabled(true)
    .set_dictionary_page_size_limit(1024 * 1024)  // 1MB max dictionary
    .build();
```

## Low-Latency Retrieval Strategies

### 1. Column Pruning

Read only the columns you need:

```rust
// Read only Y values (skip X values)
let query = "SELECT y FROM read_parquet('s3://bucket/data.parquet')";
// Reduces I/O by ~50% for XY data
```

### 2. Predicate Pushdown

Filter at storage level:

```rust
// Filter before reading full dataset
let query = format!(
    "SELECT x, y FROM read_parquet('s3://bucket/data.parquet')
     WHERE x >= {} AND x <= {}",
    min_x, max_x
);
// Only reads relevant row groups
```

### 3. Row Group Optimization

Configure row groups for your access patterns:

```rust
// Smaller row groups = better for selective queries
// Larger row groups = better for full scans
let writer_props = WriterProperties::builder()
    .set_max_row_group_length(64 * 1024)  // 64K rows per group
    .set_write_batch_size(64 * 1024)
    .build();
```

### 4. Parallel Column Decoding

Enable multi-threaded decoding:

```rust
use parquet::arrow::ArrowReaderProperties;

let reader_props = ArrowReaderProperties::builder()
    .set_use_threads(true)  // Parallel column decoding
    .set_prebuffer(true)  // Prefetch for remote files
    .build();
```

### 5. Caching Strategy

**Multi-Level Caching:**

```rust
// Level 1: In-memory cache (Redis/Memcached)
// Level 2: Local disk cache
// Level 3: Object storage

struct CachedParquetReader {
    redis_client: redis::Client,
    local_cache: PathBuf,
    s3_client: S3Client,
}

impl CachedParquetReader {
    async fn read_with_cache(&self, path: &str) -> Result<Vec<u8>> {
        // Check Redis cache first
        if let Ok(cached) = self.redis_client.get(path).await {
            return Ok(cached);
        }

        // Check local disk cache
        let local_path = self.local_cache.join(path);
        if local_path.exists() {
            let data = std::fs::read(&local_path)?;
            // Populate Redis cache
            self.redis_client.set(path, &data).await?;
            return Ok(data);
        }

        // Fetch from S3
        let data = self.s3_client.get_object(path).await?;

        // Populate both caches
        std::fs::write(&local_path, &data)?;
        self.redis_client.set(path, &data).await?;

        Ok(data)
    }
}
```

### 6. Streaming/Chunked Retrieval

For very large arrays, stream data in chunks:

```rust
use arrow::record_batch::RecordBatchReader;

// Stream row groups instead of loading all at once
let reader = ParquetFileReader::try_new(file)?;
let mut row_group_reader = reader.get_row_group(0)?;

// Read in batches
let batch_size = 10_000;
let mut batches = Vec::new();
while let Some(batch) = row_group_reader.next() {
    batches.push(batch?);
    if batches.len() >= batch_size {
        process_batches(&batches);
        batches.clear();
    }
}
```

## File Format Comparison

### Parquet vs Arrow vs Other Formats

| Format      | Latency  | Compression | Query Support | Use Case                  |
| ----------- | -------- | ----------- | ------------- | ------------------------- |
| **Parquet** | Low      | Excellent   | Excellent     | **Storage (Recommended)** |
| **Arrow**   | Very Low | Good        | Excellent     | In-memory processing      |
| **JSON**    | High     | Poor        | Limited       | Not recommended           |
| **CSV**     | Medium   | Poor        | Limited       | Not recommended           |
| **HDF5**    | Medium   | Good        | Limited       | Scientific computing      |

**Recommendation:**

- **Storage**: Parquet (compressed, queryable)
- **In-Memory**: Arrow (zero-copy, fast)
- **Transfer**: Arrow IPC (efficient serialization)

## Complete Implementation Example

### Database Schema

```sql
-- Main chart table
CREATE TABLE charts (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);

-- Series metadata table
CREATE TABLE chart_series (
    id SERIAL PRIMARY KEY,
    chart_id INTEGER REFERENCES charts(id) ON DELETE CASCADE,
    series_name TEXT NOT NULL,
    series_type TEXT NOT NULL,  -- 'line', 'scatter', 'candlestick', etc.

    -- Storage reference
    storage_path TEXT NOT NULL,  -- s3://bucket/path/to/file.parquet
    storage_format TEXT DEFAULT 'parquet',

    -- Metadata for fast queries
    metadata JSONB,  -- { "point_count": 10000, "x_min": 0, "x_max": 100, ... }

    -- Indexing
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

-- Indexes for fast lookups
CREATE INDEX idx_series_chart_id ON chart_series(chart_id);
CREATE INDEX idx_series_metadata ON chart_series USING GIN(metadata);
CREATE INDEX idx_series_storage_path ON chart_series(storage_path);

-- Function to update metadata
CREATE OR REPLACE FUNCTION update_series_metadata()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER series_metadata_update
    BEFORE UPDATE ON chart_series
    FOR EACH ROW
    EXECUTE FUNCTION update_series_metadata();
```

### Rust Implementation

```rust
use arrow::array::{Float64Array, StringArray};
use arrow::record_batch::RecordBatch;
use parquet::arrow::{ArrowReader, ArrowWriter};
use parquet::file::properties::WriterProperties;
use parquet::basic::Compression;
use std::sync::Arc;

pub struct SeriesDataManager {
    s3_client: S3Client,
    cache: RedisClient,
}

impl SeriesDataManager {
    /// Store series data as Parquet in object storage
    pub async fn store_series_data(
        &self,
        series_id: i32,
        x_values: Vec<f64>,
        y_values: Vec<f64>,
    ) -> Result<String> {
        // Create Arrow schema
        let schema = Arc::new(Schema::new(vec![
            Field::new("x", DataType::Float64, false),
            Field::new("y", DataType::Float64, false),
        ]));

        // Create Arrow arrays
        let x_array = Arc::new(Float64Array::from(x_values));
        let y_array = Arc::new(Float64Array::from(y_values));

        // Create record batch
        let batch = RecordBatch::try_new(
            schema.clone(),
            vec![x_array, y_array],
        )?;

        // Optimized writer properties for low latency
        let writer_props = WriterProperties::builder()
            .set_compression(Compression::SNAPPY)  // Fast compression
            .set_dictionary_enabled(true)  // Better compression
            .set_max_row_group_length(64 * 1024)  // 64K rows per group
            .set_write_batch_size(64 * 1024)
            .build();

        // Write to temporary file
        let temp_path = format!("/tmp/series_{}.parquet", series_id);
        let file = File::create(&temp_path)?;
        let mut writer = ArrowWriter::try_new(file, schema, Some(writer_props))?;
        writer.write(&batch)?;
        writer.close()?;

        // Upload to S3
        let s3_path = format!("series/{}/data.parquet", series_id);
        self.s3_client.upload_file(&temp_path, &s3_path).await?;

        // Clean up
        std::fs::remove_file(&temp_path)?;

        Ok(s3_path)
    }

    /// Retrieve series data with caching
    pub async fn get_series_data(
        &self,
        series_id: i32,
        x_range: Option<(f64, f64)>,
        limit: Option<usize>,
    ) -> Result<(Vec<f64>, Vec<f64>)> {
        let s3_path = format!("series/{}/data.parquet", series_id);

        // Check cache first
        let cache_key = format!("series:{}:data", series_id);
        if let Ok(cached) = self.cache.get::<_, Vec<u8>>(&cache_key).await {
            return self.deserialize_from_cache(cached);
        }

        // Query Parquet file using DuckDB
        let conn = Connection::open_in_memory()?;
        conn.execute("INSTALL httpfs", [])?;
        conn.execute("LOAD httpfs", [])?;

        // Configure S3
        self.configure_s3_for_duckdb(&conn).await?;

        // Build query with column pruning and predicate pushdown
        let mut query = format!(
            "SELECT x, y FROM read_parquet('s3://bucket/{}')",
            s3_path
        );

        if let Some((min_x, max_x)) = x_range {
            query.push_str(&format!(" WHERE x >= {} AND x <= {}", min_x, max_x));
        }

        if let Some(limit) = limit {
            query.push_str(&format!(" LIMIT {}", limit));
        }

        // Execute query
        let mut stmt = conn.prepare(&query)?;
        let rows = stmt.query_map([], |row| {
            Ok((row.get::<_, f64>(0)?, row.get::<_, f64>(1)?))
        })?;

        let mut x_values = Vec::new();
        let mut y_values = Vec::new();

        for row in rows {
            let (x, y) = row?;
            x_values.push(x);
            y_values.push(y);
        }

        // Cache result
        let serialized = self.serialize_for_cache(&x_values, &y_values)?;
        self.cache.set(&cache_key, serialized).await?;

        Ok((x_values, y_values))
    }
}
```

## Performance Benchmarks

### Expected Latency (10,000 point arrays)

| Storage Method           | Read Latency | Write Latency | Compression |
| ------------------------ | ------------ | ------------- | ----------- |
| **Parquet (S3, Snappy)** | **10-50ms**  | 100-200ms     | 3-5x        |
| Parquet (S3, ZSTD)       | 20-80ms      | 150-300ms     | 5-8x        |
| PostgreSQL TOAST         | 50-200ms     | 50-100ms      | 2-3x        |
| PostgreSQL JSONB         | 100-500ms    | 100-200ms     | 1x          |
| Arrow IPC (S3)           | 5-30ms       | 80-150ms      | 1.5-2x      |

### Optimization Impact

| Optimization       | Latency Improvement     |
| ------------------ | ----------------------- |
| Column Pruning     | 40-50% reduction        |
| Predicate Pushdown | 60-80% reduction        |
| Snappy Compression | 20-30% faster than ZSTD |
| Parallel Decoding  | 30-50% faster           |
| Caching (Redis)    | 90-95% reduction        |

## Recommendations

### For Lowest Latency:

1. **Storage Format**: Parquet with Snappy compression
2. **Storage Location**: Object Storage (MinIO/S3) with local caching
3. **Query Engine**: DuckDB with column pruning and predicate pushdown
4. **Caching**: Multi-level (Redis → Local Disk → S3)
5. **Row Groups**: 64K rows per group for optimal selectivity
6. **Encoding**: Dictionary encoding for categorical data, delta for time series

### Database Design:

```sql
-- Minimal metadata in PostgreSQL
CREATE TABLE chart_series (
    id SERIAL PRIMARY KEY,
    chart_id INTEGER REFERENCES charts(id),
    series_name TEXT NOT NULL,
    storage_path TEXT NOT NULL,  -- Reference to Parquet file
    metadata JSONB,  -- Fast queries on bounds, stats
    created_at TIMESTAMP DEFAULT NOW()
);

-- Fast lookups
CREATE INDEX idx_series_chart ON chart_series(chart_id);
CREATE INDEX idx_series_metadata ON chart_series USING GIN(metadata);
```

### Implementation Priority:

1. **Phase 1**: Parquet + S3 + DuckDB (baseline)
2. **Phase 2**: Add Redis caching layer
3. **Phase 3**: Add local disk cache
4. **Phase 4**: Optimize row groups and compression
5. **Phase 5**: Add parallel decoding and streaming

## Summary

For storing large arrays (10,000s of points) with lowest latency:

- **Storage**: Parquet files in object storage (MinIO/S3)
- **Compression**: Snappy (best latency/compression balance)
- **Database**: PostgreSQL for metadata only
- **Query**: DuckDB with column pruning and predicate pushdown
- **Caching**: Multi-level (Redis → Disk → S3)
- **Format**: Columnar (Parquet) for efficient queries

This architecture provides:

- **10-50ms read latency** for 10K point arrays
- **3-5x compression** with Snappy
- **Scalable** to millions of series
- **Queryable** with SQL-like syntax
- **Cost-effective** object storage
