# Zarr Architecture: Mutable Scientific Arrays

This document explains the architecture of Zarr for mutable scientific arrays, covering chunked blob storage, partial updates, metadata management, read/write call stacks, design tradeoffs, copy-on-write behavior, and implications for concurrency and versioning.

## Overview

Zarr is a format designed for storing large, N-dimensional arrays with emphasis on:

- **Efficient storage and retrieval** of large datasets
- **Mutability** - ability to modify arrays in-place
- **Partial updates** - modify subsets without loading entire arrays
- **Chunked storage** - divide arrays into independent, manageable blocks
- **Compression** - optional compression per chunk
- **Concurrency** - support for parallel read/write operations

## Core Architecture

### Chunked Storage Model

Zarr divides arrays into smaller, contiguous blocks called **chunks**. Each chunk is stored as an independent blob in the storage backend.

```
Array Shape: (10000, 10000)
Chunk Shape: (1000, 1000)

Result: 100 chunks (10x10 grid of chunks)
Each chunk stored as separate blob: "0.0", "0.1", ..., "9.9"
```

#### Chunk Storage Structure

```python
import zarr

# Create array with specific chunking
arr = zarr.create_array(
    store='data/array.zarr',
    shape=(10000, 10000),
    chunks=(1000, 1000),  # Chunk shape
    dtype='float32'
)

# Storage structure:
# data/array.zarr/
#   ├── zarr.json          # Metadata file
#   ├── 0.0                # Chunk at (0,0)
#   ├── 0.1                # Chunk at (0,1)
#   ├── 1.0                # Chunk at (1,0)
#   └── ...
```

#### Chunk Naming Convention

Chunks are named using their logical coordinates:

```python
# For 2D array with chunks (100, 100)
# Chunk at logical position (i, j) is stored as: "{i}.{j}"

# For 3D array with chunks (100, 100, 100)
# Chunk at (i, j, k) is stored as: "{i}.{j}.{k}"

# Example: 2D array, chunk at row 5, column 3
# Storage key: "5.3"
```

#### Chunk as Independent Blob

Each chunk is:

- **Self-contained**: Contains all data for that region
- **Compressed independently**: Optional compression per chunk
- **Stored separately**: Can be read/written independently
- **Cacheable**: Can be cached independently

### Metadata Files

Zarr stores metadata separately from data chunks, allowing quick access to array structure without reading data.

#### Metadata Structure (zarr.json)

```json
{
	"zarr_format": 3,
	"shape": [10000, 10000],
	"data_type": "float32",
	"chunk_grid": {
		"type": "regular",
		"chunk_shape": [1000, 1000]
	},
	"chunk_key_encoding": {
		"type": "default",
		"separator": "/"
	},
	"fill_value": 0.0,
	"compressor": {
		"id": "blosc",
		"cname": "lz4",
		"clevel": 5,
		"shuffle": 1
	},
	"filters": [],
	"codecs": [
		{
			"name": "bytes",
			"configuration": {
				"endian": "little"
			}
		}
	],
	"attributes": {}
}
```

#### Metadata Location

```python
# For array at path "data/array.zarr"
# Metadata stored at: "data/array.zarr/zarr.json"

# For array in group: "data/group.zarr/array"
# Metadata stored at: "data/group.zarr/array/zarr.json"
```

#### Consolidated Metadata

For read-heavy workloads, metadata can be consolidated:

```python
import zarr

# Create group with multiple arrays
store = zarr.storage.MemoryStore()
group = zarr.create_group(store=store)
group.create_array(shape=(1000,), name='a', dtype='float64')
group.create_array(shape=(2000,), name='b', dtype='float64')

# Consolidate metadata (stores all metadata in root)
zarr.consolidate_metadata(store)

# After consolidation, accessing arrays doesn't require
# individual metadata reads
consolidated = zarr.open_group(store=store)
print(consolidated['a'])  # No additional store read needed
```

**Benefits:**

- Faster access to child arrays
- Single metadata read for entire hierarchy
- Better for read-heavy, slowly-changing data

## Partial Updates

### How Partial Updates Work

When updating a subset of an array, Zarr:

1. Identifies which chunks contain the update region
2. Reads only those chunks from storage
3. Modifies the chunks in memory
4. Writes only the modified chunks back

```python
import zarr
import numpy as np

# Create array
arr = zarr.create_array(
    store='data/array.zarr',
    shape=(10000, 10000),
    chunks=(1000, 1000),
    dtype='float32'
)

# Initialize with zeros
arr[:] = 0.0

# Partial update: Only affects chunks containing [5000:6000, 2000:3000]
arr[5000:6000, 2000:3000] = 42.0

# What happens:
# 1. Identifies chunks: (5,2), (5,3), (6,2), (6,3)
# 2. Reads chunks: "5.2", "5.3", "6.2", "6.3"
# 3. Modifies chunks in memory
# 4. Writes back: "5.2", "5.3", "6.2", "6.3"
# 5. Other chunks remain untouched
```

### Chunk Boundary Alignment

Updates that align with chunk boundaries are most efficient:

```python
# Efficient: Update aligns with chunk boundaries
arr[0:1000, 0:1000] = data  # Updates exactly one chunk (0,0)

# Less efficient: Update spans multiple chunks
arr[500:1500, 500:1500] = data  # Updates 4 chunks, requires read-modify-write
```

### Read-Modify-Write Cycle

When an update doesn't align with chunk boundaries:

```python
# Update spans chunk boundaries
arr[500:1500, 500:1500] = new_data

# Process:
# 1. Read chunks: (0,0), (0,1), (1,0), (1,1)
# 2. For each chunk:
#    a. Load chunk into memory
#    b. Modify overlapping region
#    c. Compress chunk
#    d. Write chunk back
# 3. 4 chunks written (even though update is smaller)
```

## Read/Write Call Stacks

### Reading a Single Array Value

```python
value = arr[5000, 2000]
```

**Call Stack:**

```
1. arr.__getitem__((5000, 2000))
   ↓
2. Calculate chunk coordinates:
   - chunk_i = 5000 // 1000 = 5
   - chunk_j = 2000 // 1000 = 2
   - chunk_key = "5.2"
   ↓
3. Check chunk cache
   ↓
4. If not cached:
   a. store.get("5.2")  # Read chunk blob
   b. Decompress chunk (if compressed)
   c. Deserialize bytes to numpy array
   d. Cache chunk
   ↓
5. Extract value from chunk:
   - local_i = 5000 % 1000 = 0
   - local_j = 2000 % 1000 = 0
   - value = chunk[0, 0]
   ↓
6. Return value
```

### Writing a Single Array Value

```python
arr[5000, 2000] = 42.0
```

**Call Stack:**

```
1. arr.__setitem__((5000, 2000), 42.0)
   ↓
2. Calculate chunk coordinates:
   - chunk_i = 5000 // 1000 = 5
   - chunk_j = 2000 // 1000 = 2
   - chunk_key = "5.2"
   ↓
3. Load chunk (read-modify-write):
   a. Check cache
   b. If not cached:
      - store.get("5.2")  # Read existing chunk
      - Decompress (if compressed)
      - Deserialize to numpy array
      - Cache chunk
   ↓
4. Modify chunk in memory:
   - local_i = 5000 % 1000 = 0
   - local_j = 2000 % 1000 = 0
   - chunk[0, 0] = 42.0
   ↓
5. Write chunk back:
   a. Serialize chunk to bytes
   b. Compress (if configured)
   c. store.set("5.2", compressed_bytes)
   ↓
6. Update cache
```

### Writing a Slice (Multiple Values)

```python
arr[5000:6000, 2000:3000] = new_data
```

**Call Stack:**

```
1. arr.__setitem__(slice(5000:6000, 2000:3000), new_data)
   ↓
2. Calculate affected chunks:
   - Chunks: (5,2), (5,3), (6,2), (6,3)
   ↓
3. For each chunk:
   a. Calculate overlap region
   b. Load chunk (if not in cache)
   c. Copy overlapping data from new_data to chunk
   d. Serialize and compress chunk
   e. Write chunk back
   ↓
4. Update caches for all modified chunks
```

## Storage Backends

### Store Interface

Zarr uses a pluggable storage backend system:

```python
from zarr.abc.store import Store

# Store must implement:
class Store(ABC):
    def get(self, key: str) -> bytes: ...
    def set(self, key: str, value: bytes) -> None: ...
    def delete(self, key: str) -> None: ...
    def list(self, prefix: str = "") -> List[str]: ...
    # ... other methods
```

### Common Storage Backends

```python
# Local filesystem
store = zarr.storage.LocalStore('data/array.zarr')

# In-memory
store = zarr.storage.MemoryStore()

# ZIP file
store = zarr.storage.ZipStore('data.zip', mode='w')

# S3 (via fsspec)
store = zarr.storage.FSStore('s3://bucket/array.zarr')

# Custom backend
class MyStore(zarr.abc.store.Store):
    def get(self, key: str) -> bytes: ...
    def set(self, key: str, value: bytes) -> None: ...
    # ... implement interface
```

## Design Tradeoffs

### 1. Chunk Size Tradeoffs

**Small Chunks:**

- ✅ Better for partial updates (fewer chunks to read/write)
- ✅ Lower memory usage
- ✅ Better parallelization
- ❌ More chunks to manage
- ❌ More metadata overhead
- ❌ Lower compression ratios

**Large Chunks:**

- ✅ Fewer chunks to manage
- ✅ Better compression ratios
- ✅ Less metadata overhead
- ❌ Higher memory usage
- ❌ More data read/written for small updates
- ❌ Less granular parallelization

```python
# Small chunks: Good for frequent small updates
arr = zarr.create_array(..., chunks=(100, 100))

# Large chunks: Good for sequential reads
arr = zarr.create_array(..., chunks=(10000, 10000))
```

### 2. Compression Tradeoffs

**Compression Enabled:**

- ✅ Reduced storage space
- ✅ Faster I/O (less data to transfer)
- ❌ CPU overhead for compression/decompression
- ❌ Slower writes

**No Compression:**

- ✅ Faster read/write (no CPU overhead)
- ✅ Lower latency
- ❌ More storage space required
- ❌ More I/O bandwidth needed

```python
# With compression
arr = zarr.create_array(
    ...,
    compressor=zarr.Blosc(cname='lz4', clevel=5)
)

# Without compression
arr = zarr.create_array(..., compressor=None)
```

### 3. Write Empty Chunks

```python
# Skip writing chunks filled with fill_value
arr = zarr.create_array(
    ...,
    fill_value=0.0,
    config={'write_empty_chunks': False}
)
```

**Tradeoff:**

- ✅ Saves storage space
- ✅ Faster writes for sparse data
- ❌ Must handle missing chunks during reads
- ❌ Slightly more complex read logic

### 4. Metadata Consolidation

**Consolidated:**

- ✅ Faster access to child arrays
- ✅ Single metadata read
- ❌ Must rewrite entire metadata on any change
- ❌ Not suitable for frequently changing hierarchies

**Non-Consolidated:**

- ✅ Independent metadata per array
- ✅ Faster updates (only affected metadata)
- ❌ Multiple metadata reads for hierarchy traversal

## Copy-on-Write Behavior

### Default Behavior

Zarr does **not** implement copy-on-write by default. Updates modify chunks in-place:

```python
# Direct modification
arr[0:1000, 0:1000] = new_data
# Chunk "0.0" is overwritten directly
```

### Implementing Copy-on-Write

Some implementations (e.g., Icechunk) add copy-on-write for versioning:

```python
# Conceptual copy-on-write behavior
def update_with_cow(arr, region, data):
    # 1. Create new chunk version
    old_chunk = read_chunk("0.0")
    new_chunk = old_chunk.copy()
    new_chunk[region] = data

    # 2. Write new version
    write_chunk("0.0.v2", new_chunk)

    # 3. Update metadata to point to new version
    update_metadata({"0.0": "0.0.v2"})

    # 4. Old version remains (for versioning)
```

**Benefits:**

- Time travel (access previous versions)
- Atomic updates
- Better concurrency control

**Costs:**

- More storage space
- More complex metadata management
- Garbage collection needed for old versions

## Concurrency and Versioning

### Concurrent Reads

Multiple readers can safely read the same array:

```python
# Process 1
arr1 = zarr.open_array('data/array.zarr', mode='r')
data1 = arr1[0:1000, 0:1000]

# Process 2 (concurrent)
arr2 = zarr.open_array('data/array.zarr', mode='r')
data2 = arr2[5000:6000, 5000:6000]

# Safe: Reading different chunks
```

### Concurrent Writes

**Challenge:** Multiple writers modifying same chunks can cause corruption.

**Solution 1: Chunk-Level Locking**

```python
# Conceptual locking
def write_chunk_safe(key, data):
    with chunk_lock(key):
        chunk = read_chunk(key)
        chunk.update(data)
        write_chunk(key, chunk)
```

**Solution 2: Transactional Storage (Icechunk)**

```python
# Transactional model
with transaction():
    arr[0:1000, 0:1000] = data1
    arr[5000:6000, 5000:6000] = data2
    # All changes committed atomically
    # Reads see consistent state
```

### Versioning Strategies

**1. Metadata Versioning**

```python
# Store version in metadata
metadata = {
    "version": 2,
    "chunks": {
        "0.0": "0.0.v2",  # Points to versioned chunk
        "0.1": "0.1.v1"
    }
}
```

**2. Chunk Versioning**

```python
# Store multiple versions of chunks
# "0.0.v1" - original
# "0.0.v2" - updated
# "0.0.v3" - latest

# Metadata points to latest
metadata["chunks"]["0.0"] = "0.0.v3"
```

**3. Snapshot-Based Versioning**

```python
# Create snapshot of entire array
snapshot = create_snapshot('data/array.zarr', version='v1')

# Updates go to new version
arr = open_array('data/array.zarr', version='v2')
arr[0:1000, 0:1000] = new_data

# Can access old version
old_arr = open_array('data/array.zarr', version='v1')
```

### Isolation Levels

**Read Uncommitted:**

- Readers may see partial writes
- Fast but inconsistent

**Read Committed (Default):**

- Readers see committed writes
- Writers may conflict

**Serializable (Transactional):**

- All operations serializable
- Highest consistency, lower performance

## Performance Considerations

### Chunk Size Selection

```python
# Rule of thumb: Chunk size should be:
# - Large enough for good compression (1MB - 10MB)
# - Small enough to fit in memory
# - Aligned with access patterns

# For row-wise access
arr = zarr.create_array(..., chunks=(1000, 10000))

# For column-wise access
arr = zarr.create_array(..., chunks=(10000, 1000))

# For balanced access
arr = zarr.create_array(..., chunks=(1000, 1000))
```

### Caching Strategy

```python
# Zarr automatically caches recently accessed chunks
# Cache size can be configured

arr = zarr.create_array(
    ...,
    cache_metadata=True,  # Cache metadata
    cache_attrs=True      # Cache attributes
)

# For custom caching
from zarr.storage import LRUStoreCache
store = LRUStoreCache(underlying_store, max_size=2**30)  # 1GB cache
```

### Parallel Access

```python
# Zarr supports parallel reads/writes
# Chunks can be processed in parallel

from concurrent.futures import ThreadPoolExecutor

def process_chunk(chunk_key):
    chunk = arr.get_chunk(chunk_key)
    # Process chunk
    return result

# Process chunks in parallel
with ThreadPoolExecutor() as executor:
    results = executor.map(process_chunk, chunk_keys)
```

## Example: Complete Update Flow

```python
import zarr
import numpy as np

# 1. Create array
arr = zarr.create_array(
    store='data/array.zarr',
    shape=(10000, 10000),
    chunks=(1000, 1000),
    dtype='float32',
    compressor=zarr.Blosc(cname='lz4', clevel=5)
)

# 2. Initialize
arr[:] = 0.0

# 3. Partial update
update_region = arr[5000:6000, 2000:3000]
new_data = np.random.rand(1000, 1000).astype('float32')

# Internal process:
# - Identifies chunks: (5,2), (5,3), (6,2), (6,3)
# - For each chunk:
#   a. Read chunk blob from store
#   b. Decompress
#   c. Deserialize to numpy
#   d. Modify overlapping region
#   e. Serialize to bytes
#   f. Compress
#   g. Write back to store
# - Update metadata if needed

arr[5000:6000, 2000:3000] = new_data

# 4. Read back
result = arr[5000:6000, 2000:3000]
```

## Best Practices

1. **Choose chunk size based on access patterns**
   - Align with common read/write regions
   - Balance between granularity and overhead

2. **Use compression for storage efficiency**
   - LZ4 for speed, Zstd for compression ratio
   - Consider data characteristics

3. **Align updates with chunk boundaries when possible**
   - Reduces read-modify-write overhead
   - Better performance

4. **Use consolidated metadata for read-heavy workloads**
   - Single metadata read
   - Better for slowly-changing hierarchies

5. **Implement proper concurrency control**
   - Use locking or transactions for concurrent writes
   - Consider versioning for complex workflows

6. **Monitor chunk cache performance**
   - Adjust cache size based on access patterns
   - Consider cache eviction policies

## References

- [Zarr Specification](https://zarr.dev/)
- [Zarr Python Documentation](https://zarr.readthedocs.io/)
- [Icechunk (Transactional Zarr)](https://icechunk.io/)
