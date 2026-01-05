# Reading Parquet Data from a Binary Blob with DuckDB

This guide explains how to read Parquet data from a binary blob (in-memory buffer) using DuckDB, based on the official DuckDB documentation.

## Overview

DuckDB provides several methods to read Parquet files, including from binary blobs in memory. The approach depends on your programming language and environment.

## Method 1: Using DuckDB-Wasm (JavaScript/TypeScript)

For web applications, DuckDB-Wasm supports registering binary buffers directly:

```javascript
// Fetch Parquet file as binary data
const res = await fetch('https://origin/remote.parquet');
const parquetBuffer = new Uint8Array(await res.arrayBuffer());

// Register the buffer with DuckDB
await db.registerFileBuffer('buffer.parquet', parquetBuffer);

// Query the registered buffer
const result = await conn.query(`
    SELECT * FROM read_parquet('buffer.parquet')
`);
```

### Alternative: Direct URL Query

You can also query Parquet files directly from URLs without downloading:

```javascript
await conn.query(`
    CREATE TABLE direct AS
        SELECT * FROM 'https://origin/remote.parquet'
`);
```

## Method 2: Using DuckDB Python

In Python, you can use `BytesIO` or similar approaches to work with in-memory Parquet data:

```python
import duckdb
from io import BytesIO
import pyarrow.parquet as pq

# Assuming you have a parquet blob as bytes
parquet_blob: bytes = get_parquet_blob()  # Your binary data

# Option 1: Write to temporary file and read
with open('temp.parquet', 'wb') as f:
    f.write(parquet_blob)

con = duckdb.connect()
result = con.execute("SELECT * FROM read_parquet('temp.parquet')").fetchall()

# Option 2: Use PyArrow to read from memory, then convert
parquet_file = BytesIO(parquet_blob)
table = pq.read_table(parquet_file)
result = con.execute("SELECT * FROM table").fetchall()
```

### Using DuckDB's `read_parquet` Function

```python
import duckdb

con = duckdb.connect()

# Read from file path
relation = con.read_parquet('path/to/file.parquet')

# Read multiple files with glob pattern
relation = con.read_parquet('path/to/files/*.parquet')

# Read from list of files
relation = con.read_parquet(['file1.parquet', 'file2.parquet'])

# Query the relation
result = relation.fetchall()
```

## Method 3: Using DuckDB Rust (duckdb-rs)

For Rust applications:

```rust
use duckdb::{Connection, Result};
use duckdb::arrow::record_batch::RecordBatch;

fn read_parquet_from_blob(parquet_blob: &[u8]) -> Result<Vec<RecordBatch>> {
    let conn = Connection::open_in_memory()?;
    
    // Install and load Parquet extension
    conn.execute_batch("INSTALL parquet; LOAD parquet;")?;
    
    // Write blob to temporary file (DuckDB needs file path)
    std::fs::write("temp.parquet", parquet_blob)?;
    
    // Read Parquet file
    let batches: Vec<RecordBatch> = conn
        .prepare("SELECT * FROM read_parquet('temp.parquet')")? 
        .query_arrow([])? 
        .collect();
    
    // Clean up temp file
    std::fs::remove_file("temp.parquet")?;
    
    Ok(batches)
}
```

### Reading with Filters and Projections

```rust
let mut stmt = conn.prepare(
    "SELECT id, name, amount
     FROM read_parquet('data.parquet')
     WHERE amount > ?
     ORDER BY amount DESC"
)?;

let results: Vec<RecordBatch> = stmt.query_arrow([1000])?.collect();
```

## Method 4: SQL Direct Query

If you have the Parquet file accessible via file path, URL, or registered buffer:

```sql
-- Read single Parquet file
SELECT * FROM read_parquet('data.parquet');

-- Read from HTTPS URL
SELECT * FROM read_parquet('https://some.url/some_file.parquet');

-- Read multiple files with glob pattern
SELECT * FROM read_parquet('s3://your-bucket/*.parquet');

-- Read with filename column (useful for multi-file queries)
SELECT * FROM read_parquet('s3://your-bucket/*.parquet', filename = true);
```

## Method 5: Creating a View from Parquet

You can create a view to treat Parquet data as a regular table:

```sql
CREATE VIEW people AS SELECT * FROM read_parquet('people.parquet');

-- Then query as normal
SELECT * FROM people WHERE age > 25;
```

## Inspecting Parquet Files

### Get Schema Information

```sql
SELECT * FROM parquet_schema('test.parquet');
```

### Get File Metadata

```sql
SELECT * FROM parquet_file_metadata('test.parquet');
```

## Best Practices

1. **For Large Blobs**: Consider writing to a temporary file if your environment supports it, as DuckDB is optimized for file-based access.

2. **Memory Management**: When working with large Parquet files in memory, be mindful of memory usage. DuckDB can stream data efficiently.

3. **Error Handling**: Always handle cases where the Parquet blob might be corrupted or invalid.

4. **Performance**: For repeated queries on the same data, consider creating a view or materializing the data into a DuckDB table.

## Example: Complete Python Workflow

```python
import duckdb
from io import BytesIO

def query_parquet_blob(parquet_blob: bytes) -> list:
    """
    Query a Parquet file from a binary blob.
    
    Args:
        parquet_blob: Binary data containing Parquet file
        
    Returns:
        Query results as a list of tuples
    """
    # Create in-memory database
    con = duckdb.connect(database=':memory:')
    
    # Write blob to temporary file
    temp_path = 'temp_query.parquet'
    try:
        with open(temp_path, 'wb') as f:
            f.write(parquet_blob)
        
        # Query the Parquet file
        result = con.execute("SELECT * FROM read_parquet(?)", [temp_path]).fetchall()
        return result
    finally:
        # Clean up
        import os
        if os.path.exists(temp_path):
            os.remove(temp_path)
        con.close()
```

## Notes

- DuckDB's `read_parquet` function is highly optimized and supports column pruning, predicate pushdown, and other optimizations.
- The Parquet extension is typically included by default in most DuckDB distributions.
- For cloud storage (S3, GCS, Azure), DuckDB supports direct access via URLs when the appropriate extensions are loaded.

## References

- [DuckDB Parquet Documentation](https://duckdb.org/docs/data/parquet/overview)
- [DuckDB Python API](https://duckdb.org/docs/api/python/overview)
- [DuckDB-Wasm Documentation](https://duckdb.org/docs/api/wasm/overview)

