# TileDB Architecture: Editing Dense and Sparse Arrays

This document explains TileDB's internal architecture for editing dense and sparse arrays, covering fragment creation, immutable writes, logical overwrites, fragment merging, and the complete call stack for update operations.

## Overview

TileDB is a storage engine designed for managing dense and sparse multi-dimensional arrays. Its architecture emphasizes:

- **Immutability**: Fragments are never modified in-place
- **Concurrency**: Simultaneous reads and writes without conflicts
- **Consistency**: Eventual consistency with time-travel capabilities
- **Performance**: Optimized for both read and write operations

## Core Architecture: Fragment-Based Storage

### Fragment Concept

A **fragment** is an immutable, self-contained unit of data written to a TileDB array. Each write operation creates a new fragment, never modifying existing ones.

```
Array Structure:
my_array/
  ├── __schema                    # Array schema
  ├── __fragments                 # Fragment directory
  │   ├── __tiledb_timestamp_start_1_end_1
  │   │   ├── __fragment_metadata.tdb
  │   │   ├── attr1.tdb           # Attribute data
  │   │   └── coords.tdb          # Coordinate data (sparse)
  │   ├── __tiledb_timestamp_start_2_end_2
  │   │   └── ...
  │   └── __tiledb_timestamp_start_3_end_3
  │       └── ...
  └── __meta                      # Array metadata
```

### Fragment Properties

Each fragment contains:

- **Timestamp range**: When the fragment was created
- **Domain**: Spatial/temporal coverage of the fragment
- **Data tiles**: Compressed data organized in tiles
- **Metadata**: Fragment-level metadata for efficient querying

### Fragment Naming

Fragments are named with timestamps:

```
__tiledb_timestamp_start_{start}_end_{end}
```

Example:

```
__tiledb_timestamp_start_1000000000_end_1000000001
```

## Write Operations: Creating New Fragments

### Write Process Overview

When writing to a TileDB array, the process is:

1. **Create new fragment**: Generate unique fragment URI with timestamp
2. **Write data**: Write data to the new fragment (immutable)
3. **Update array metadata**: Add fragment reference to array metadata
4. **Complete**: Fragment is now part of the array

### Dense Array Write

```cpp
// Conceptual write process for dense array
void write_dense_array(Array* array, void* data, Subarray* subarray) {
    // 1. Generate fragment URI with timestamp
    uint64_t timestamp = get_current_timestamp();
    std::string fragment_uri = generate_fragment_uri(timestamp);

    // 2. Create new fragment writer
    FragmentWriter* writer = create_fragment_writer(fragment_uri);

    // 3. Write data to fragment (organized in tiles)
    for (auto tile : compute_tiles(subarray)) {
        void* tile_data = extract_tile_data(data, tile);
        compress_tile(tile_data);
        writer->write_tile(tile);
    }

    // 4. Write fragment metadata
    FragmentMetadata metadata;
    metadata.set_domain(subarray);
    metadata.set_timestamp_range(timestamp, timestamp);
    writer->write_metadata(metadata);

    // 5. Close fragment
    writer->close();

    // 6. Update array metadata (add fragment reference)
    array->add_fragment(fragment_uri);
}
```

### Sparse Array Write

```cpp
// Conceptual write process for sparse array
void write_sparse_array(Array* array,
                        void* coordinates,
                        void* attributes,
                        uint64_t num_cells) {
    // 1. Generate fragment URI
    uint64_t timestamp = get_current_timestamp();
    std::string fragment_uri = generate_fragment_uri(timestamp);

    // 2. Create fragment writer
    FragmentWriter* writer = create_fragment_writer(fragment_uri);

    // 3. Sort data (if not in global order)
    if (query_layout != GLOBAL_ORDER) {
        sort_by_coordinates(coordinates, attributes, num_cells);
    }

    // 4. Write coordinates and attributes
    writer->write_coordinates(coordinates, num_cells);
    for (auto attr : array->attributes()) {
        writer->write_attribute(attr, attributes[attr], num_cells);
    }

    // 5. Write fragment metadata
    FragmentMetadata metadata;
    metadata.set_non_empty_domain(compute_domain(coordinates, num_cells));
    metadata.set_timestamp_range(timestamp, timestamp);
    writer->write_metadata(metadata);

    // 6. Close and register fragment
    writer->close();
    array->add_fragment(fragment_uri);
}
```

## Logical Overwrites: Immutability in Action

### How Overwrites Work

When updating existing array values, TileDB does **not** modify existing fragments. Instead:

1. **New fragment created**: Contains the updated data
2. **Old fragments remain**: Original data is unchanged
3. **Read-time resolution**: During reads, newer fragments override older ones

### Example: Overwriting Data

```cpp
// Initial state: Fragment 1 contains data at [100:200, 100:200]
// Fragment 1: __tiledb_timestamp_start_1_end_1
//   - Domain: [100:200, 100:200]
//   - Data: original values

// Write operation: Update [150:160, 150:160]
array[150:160, 150:160] = new_data;

// Result: New fragment created
// Fragment 2: __tiledb_timestamp_start_2_end_2
//   - Domain: [150:160, 150:160]
//   - Data: new values

// Fragment 1 remains unchanged!
// During reads, Fragment 2's data overrides Fragment 1's data
// for the overlapping region [150:160, 150:160]
```

### Overwrite Resolution Logic

```cpp
// Conceptual read logic for handling overwrites
void resolve_overwrites(std::vector<Fragment*> fragments,
                        Subarray* query_range,
                        ResultBuffer* result) {
    // Sort fragments by timestamp (newest first)
    std::sort(fragments.begin(), fragments.end(),
              [](Fragment* a, Fragment* b) {
                  return a->timestamp_end() > b->timestamp_end();
              });

    // Track which regions have been covered
    std::vector<Subarray> covered_regions;

    // Process fragments in timestamp order (newest first)
    for (auto fragment : fragments) {
        Subarray fragment_range = fragment->domain();
        Subarray overlap = compute_overlap(fragment_range, query_range);

        // Skip if already covered by newer fragment
        if (is_covered(overlap, covered_regions)) {
            continue;
        }

        // Read data from fragment
        void* fragment_data = read_fragment_data(fragment, overlap);

        // Merge into result (newer data overwrites older)
        merge_into_result(result, fragment_data, overlap);

        // Mark region as covered
        covered_regions.push_back(overlap);
    }
}
```

## Fragment Merge Process During Reads

### Merge Algorithm Overview

When reading from an array with multiple fragments, TileDB merges fragments to present a unified view:

1. **Fragment selection**: Identify fragments overlapping query range
2. **Timestamp ordering**: Sort fragments by timestamp (newest first)
3. **Merge operation**: Combine data, with newer fragments overriding older ones
4. **Result assembly**: Build final result from merged data

### Dense Array Fragment Merge

```cpp
// Conceptual merge for dense arrays
void merge_dense_fragments(std::vector<Fragment*> fragments,
                           Subarray* query_range,
                           ResultBuffer* result) {
    // Initialize result with fill value
    initialize_result(result, query_range, array->fill_value());

    // Process fragments in reverse timestamp order (oldest first for dense)
    // This allows newer fragments to overwrite older ones
    std::sort(fragments.begin(), fragments.end(),
              [](Fragment* a, Fragment* b) {
                  return a->timestamp_start() < b->timestamp_start();
              });

    for (auto fragment : fragments) {
        Subarray overlap = compute_overlap(fragment->domain(), query_range);

        if (overlap.is_empty()) {
            continue;
        }

        // Read fragment data for overlap region
        void* fragment_data = read_fragment_tiles(fragment, overlap);

        // Overwrite result with fragment data
        copy_to_result(result, fragment_data, overlap);
    }
}
```

### Sparse Array Fragment Merge

```cpp
// Conceptual merge for sparse arrays
void merge_sparse_fragments(std::vector<Fragment*> fragments,
                             Subarray* query_range,
                             ResultBuffer* result) {
    // For sparse arrays, we need to merge coordinate lists
    std::vector<Cell> all_cells;

    // Collect cells from all fragments
    for (auto fragment : fragments) {
        Subarray overlap = compute_overlap(fragment->domain(), query_range);

        if (overlap.is_empty()) {
            continue;
        }

        // Read cells from fragment
        std::vector<Cell> fragment_cells = read_fragment_cells(fragment, overlap);
        all_cells.insert(all_cells.end(),
                        fragment_cells.begin(),
                        fragment_cells.end());
    }

    // Sort by coordinates
    std::sort(all_cells.begin(), all_cells.end(),
              [](const Cell& a, const Cell& b) {
                  return compare_coordinates(a.coords, b.coords) < 0;
              });

    // Remove duplicates (keep newest)
    remove_duplicate_cells(all_cells, fragments);

    // Write to result
    result->write_cells(all_cells);
}

void remove_duplicate_cells(std::vector<Cell>& cells,
                            std::vector<Fragment*> fragments) {
    // Create map: coordinate -> (cell, fragment_timestamp)
    std::map<Coordinates, std::pair<Cell, uint64_t>> cell_map;

    for (auto& cell : cells) {
        uint64_t cell_timestamp = get_cell_timestamp(cell, fragments);

        auto it = cell_map.find(cell.coords);
        if (it == cell_map.end() ||
            cell_timestamp > it->second.second) {
            // New cell or newer timestamp
            cell_map[cell.coords] = {cell, cell_timestamp};
        }
    }

    // Rebuild cells list with only newest
    cells.clear();
    for (auto& pair : cell_map) {
        cells.push_back(pair.second.first);
    }
}
```

## Complete Call Stack: Update Operation

### Update Operation Flow

```cpp
// User code
array[100:200, 100:200] = new_data;
```

**Complete Call Stack:**

```
1. Array::operator[] / Array::set_subarray()
   ↓
2. Query::set_subarray()
   ↓
3. Query::set_data_buffer() / Query::set_buffer()
   ↓
4. Query::submit()
   ↓
5. Query::submit_write()
   ↓
6. WriterBase::init()
   - Determine write layout (UNORDERED, GLOBAL_ORDER, ROW_MAJOR, COL_MAJOR)
   - Allocate fragment URI with timestamp
   ↓
7. WriterBase::create_fragment()
   - Generate fragment URI: __tiledb_timestamp_start_{ts}_end_{ts}
   - Create FragmentWriter
   ↓
8. WriterBase::write()
   ↓
9. DenseWriter::write() or SparseWriter::write()
   ↓
10. For each tile in subarray:
    a. TileWriter::write_tile()
       - Compress tile data
       - Write tile to fragment
    ↓
11. WriterBase::finalize()
    - Write fragment metadata
    - Close fragment
    ↓
12. Array::add_fragment()
    - Update array metadata
    - Add fragment URI to fragment list
    - Update array non-empty domain
    ↓
13. Query::finalize()
    - Complete write operation
```

### Detailed Call Stack: Dense Array Update

```cpp
// Step-by-step call stack for dense array update

// 1. User initiates write
array[100:200, 100:200] = new_data;

// 2. Query creation and configuration
Query query(ctx, array, TILEDB_WRITE);
query.set_subarray({100, 200, 100, 200});
query.set_buffer("attr1", buffer, buffer_size);

// 3. Query submission
query.submit();
  → Query::submit_write()
    → WriterBase::init()
      → generate_fragment_uri(timestamp)
      → FragmentWriter::create(fragment_uri)

    → DenseWriter::write()
      → compute_tiles(subarray)
      → for each tile:
          TileWriter::write_tile(tile_data)
            → compress_tile(tile_data)
            → write_to_storage(tile_blob)

    → WriterBase::finalize()
      → FragmentMetadata::create()
      → FragmentMetadata::set_domain(subarray)
      → FragmentMetadata::set_timestamp_range(start, end)
      → FragmentWriter::write_metadata(metadata)
      → FragmentWriter::close()

    → Array::add_fragment(fragment_uri)
      → ArrayMetadata::add_fragment(fragment_uri)
      → ArrayMetadata::update_non_empty_domain()
      → ArrayMetadata::write()
```

### Detailed Call Stack: Sparse Array Update

```cpp
// Step-by-step call stack for sparse array update

// 1. User initiates write
std::vector<Coordinates> coords = {...};
std::vector<Value> values = {...};
query.set_coordinates(coords);
query.set_buffer("attr1", values);

// 2. Query submission
query.submit();
  → Query::submit_write()
    → WriterBase::init()
      → generate_fragment_uri(timestamp)
      → FragmentWriter::create(fragment_uri)

    → SparseWriter::write()
      → if (layout != GLOBAL_ORDER):
          sort_by_coordinates(coords, values)

      → FragmentWriter::write_coordinates(coords)
        → compress_coordinates(coords)
        → write_to_storage(coords_blob)

      → for each attribute:
          FragmentWriter::write_attribute(attr, values)
            → compress_attribute(values)
            → write_to_storage(attr_blob)

    → WriterBase::finalize()
      → compute_non_empty_domain(coords)
      → FragmentMetadata::set_non_empty_domain(domain)
      → FragmentMetadata::set_timestamp_range(start, end)
      → FragmentWriter::write_metadata(metadata)
      → FragmentWriter::close()

    → Array::add_fragment(fragment_uri)
      → ArrayMetadata::add_fragment(fragment_uri)
      → ArrayMetadata::update_non_empty_domain()
      → ArrayMetadata::write()
```

## Fragment Consolidation

### Why Consolidate?

As fragments accumulate, read performance can degrade:

- More fragments to process during reads
- More metadata to load
- More merge operations required

### Consolidation Process

```cpp
// Conceptual consolidation process
void consolidate_fragments(Array* array,
                           std::vector<Fragment*> fragments_to_consolidate) {
    // 1. Create new consolidated fragment
    uint64_t timestamp = get_current_timestamp();
    std::string consolidated_uri = generate_fragment_uri(timestamp);
    FragmentWriter* writer = create_fragment_writer(consolidated_uri);

    // 2. Merge data from all fragments
    // For dense: merge tiles, newer overwrites older
    // For sparse: merge cells, remove duplicates (keep newest)
    void* consolidated_data = merge_fragment_data(fragments_to_consolidate);

    // 3. Write consolidated data
    writer->write_data(consolidated_data);

    // 4. Write consolidated metadata
    FragmentMetadata metadata;
    metadata.set_domain(compute_union_domain(fragments_to_consolidate));
    metadata.set_timestamp_range(
        min_timestamp(fragments_to_consolidate),
        max_timestamp(fragments_to_consolidate)
    );
    writer->write_metadata(metadata);
    writer->close();

    // 5. Update array metadata
    array->remove_fragments(fragments_to_consolidate);
    array->add_fragment(consolidated_uri);

    // 6. Old fragments remain (for time-travel)
    // Vacuum operation needed to delete them
}
```

### Consolidation Strategies

**1. Fragment Count Consolidation**

```cpp
// Consolidate when fragment count exceeds threshold
if (array->fragment_count() > MAX_FRAGMENTS) {
    consolidate_fragments(array, select_fragments_to_consolidate());
}
```

**2. Size-Based Consolidation**

```cpp
// Consolidate small fragments
std::vector<Fragment*> small_fragments;
for (auto fragment : array->fragments()) {
    if (fragment->size() < MIN_FRAGMENT_SIZE) {
        small_fragments.push_back(fragment);
    }
}
if (small_fragments.size() > 1) {
    consolidate_fragments(array, small_fragments);
}
```

**3. Time-Based Consolidation**

```cpp
// Consolidate old fragments
std::vector<Fragment*> old_fragments;
uint64_t cutoff_time = get_current_timestamp() - MAX_FRAGMENT_AGE;
for (auto fragment : array->fragments()) {
    if (fragment->timestamp_end() < cutoff_time) {
        old_fragments.push_back(fragment);
    }
}
consolidate_fragments(array, old_fragments);
```

## Balancing Performance, Immutability, and Consistency

### Design Tradeoffs

#### 1. Immutability vs. Performance

**Immutability Benefits:**

- ✅ Concurrent reads and writes
- ✅ Time-travel queries
- ✅ Crash recovery (no partial writes)
- ✅ Simple concurrency model

**Performance Costs:**

- ❌ More fragments over time
- ❌ More storage space (until consolidation)
- ❌ More complex read logic (merge required)

**Mitigation:**

- Regular consolidation
- Efficient merge algorithms
- Fragment metadata caching

#### 2. Write Size vs. Fragment Count

**Large Writes:**

- ✅ Fewer fragments
- ✅ Better compression
- ✅ Faster consolidation
- ❌ Less granular updates

**Small Writes:**

- ✅ More granular updates
- ✅ Better for incremental updates
- ❌ More fragments
- ❌ More overhead

**Recommendation:**

- Batch small updates when possible
- Use appropriate write layout (GLOBAL_ORDER for efficiency)

#### 3. Consistency Model

**Eventual Consistency:**

- ✅ High performance
- ✅ No locking required
- ✅ Scales well
- ❌ Readers may see stale data temporarily

**Strong Consistency (with locking):**

- ✅ Immediate consistency
- ✅ Predictable behavior
- ❌ Lower performance
- ❌ Lock contention

**TileDB Approach:**

- Default: Eventual consistency (no locking)
- Optional: Explicit locking for strong consistency
- Timestamp-based ordering ensures eventual consistency

### Performance Optimizations

#### 1. Fragment Metadata Caching

```cpp
// Cache fragment metadata to avoid repeated reads
class FragmentMetadataCache {
    std::map<std::string, FragmentMetadata> cache;

    FragmentMetadata* get(const std::string& fragment_uri) {
        auto it = cache.find(fragment_uri);
        if (it != cache.end()) {
            return &it->second;
        }

        // Load from storage
        FragmentMetadata metadata = load_metadata(fragment_uri);
        cache[fragment_uri] = metadata;
        return &cache[fragment_uri];
    }
};
```

#### 2. Consolidated Fragment Metadata

```cpp
// Store all fragment metadata in single file
void consolidate_fragment_metadata(Array* array) {
    ConsolidatedMetadata metadata;

    for (auto fragment : array->fragments()) {
        metadata.add_fragment_metadata(fragment->metadata());
    }

    write_consolidated_metadata(array, metadata);
}

// Reading consolidated metadata is faster
FragmentMetadata* get_fragment_metadata(Array* array,
                                        const std::string& fragment_uri) {
    ConsolidatedMetadata* consolidated =
        load_consolidated_metadata(array);
    return consolidated->get_metadata(fragment_uri);
}
```

#### 3. Parallel Fragment Processing

```cpp
// Process fragments in parallel during reads
void parallel_fragment_merge(std::vector<Fragment*> fragments,
                             Subarray* query_range,
                             ResultBuffer* result) {
    // Partition fragments across threads
    std::vector<std::vector<Fragment*>> partitions =
        partition_fragments(fragments, num_threads);

    // Process each partition in parallel
    #pragma omp parallel for
    for (auto partition : partitions) {
        ResultBuffer partition_result;
        merge_fragments(partition, query_range, &partition_result);

        // Merge partition results
        #pragma omp critical
        {
            merge_results(result, &partition_result);
        }
    }
}
```

## Concurrency Model

### Read-Write Concurrency

```cpp
// Multiple readers and writers can operate concurrently
// No locking required for reads

// Reader 1
Query read_query1(ctx, array, TILEDB_READ);
read_query1.set_subarray({0, 100, 0, 100});
read_query1.submit();  // Reads fragments 1, 2, 3

// Writer (concurrent)
Query write_query(ctx, array, TILEDB_WRITE);
write_query.set_subarray({50, 150, 50, 150});
write_query.submit();  // Creates fragment 4

// Reader 2 (concurrent)
Query read_query2(ctx, array, TILEDB_READ);
read_query2.set_subarray({0, 100, 0, 100});
read_query2.submit();  // Reads fragments 1, 2, 3, 4 (if visible)

// All operations can proceed concurrently
// Readers see consistent snapshot based on array metadata timestamp
```

### Write-Write Concurrency

```cpp
// Multiple writers can write concurrently
// Each creates its own fragment

// Writer 1
Query write1(ctx, array, TILEDB_WRITE);
write1.set_subarray({0, 100, 0, 100});
write1.submit();  // Creates fragment 4

// Writer 2 (concurrent)
Query write2(ctx, array, TILEDB_WRITE);
write2.set_subarray({50, 150, 50, 150});
write2.submit();  // Creates fragment 5

// Both fragments are created independently
// Array metadata updated atomically
// Overlapping regions resolved by timestamp during reads
```

## Time-Travel Queries

### Querying Historical Data

```cpp
// Query array at specific timestamp
Array array(ctx, "my_array", TILEDB_READ);
array.set_open_timestamp(1000000000);  // Open at timestamp

Query query(ctx, array, TILEDB_READ);
query.set_subarray({0, 100, 0, 100});
query.submit();

// Only fragments with timestamp <= 1000000000 are considered
// Provides time-travel capability
```

### Fragment Timestamp Filtering

```cpp
// During read, filter fragments by timestamp
std::vector<Fragment*> select_fragments(
    std::vector<Fragment*> all_fragments,
    uint64_t query_timestamp) {

    std::vector<Fragment*> selected;
    for (auto fragment : all_fragments) {
        if (fragment->timestamp_end() <= query_timestamp) {
            selected.push_back(fragment);
        }
    }
    return selected;
}
```

## Best Practices

### 1. Write Optimization

- **Use GLOBAL_ORDER layout** when possible (avoids sorting)
- **Batch writes** to reduce fragment count
- **Align writes with tile boundaries** for dense arrays

### 2. Fragment Management

- **Regular consolidation** to maintain performance
- **Monitor fragment count** and consolidate when needed
- **Use vacuum** to clean up old fragments after consolidation

### 3. Read Optimization

- **Use consolidated fragment metadata** for faster array opening
- **Cache fragment metadata** when repeatedly querying same array
- **Use appropriate query layouts** (ROW_MAJOR, COL_MAJOR) based on access patterns

### 4. Concurrency

- **Leverage immutability** for concurrent reads and writes
- **Use timestamps** for consistent snapshots
- **Consider locking** only when strong consistency is required

## Summary

TileDB's architecture balances performance, immutability, and consistency through:

1. **Fragment-based immutability**: Each write creates a new fragment, never modifying existing data
2. **Logical overwrites**: Newer fragments logically override older ones during reads
3. **Efficient merging**: Sophisticated merge algorithms combine fragments during reads
4. **Consolidation**: Periodic merging of fragments maintains performance
5. **Concurrency**: Immutability enables safe concurrent reads and writes
6. **Time-travel**: Timestamp-based fragment management enables historical queries

This design provides a robust foundation for managing large-scale array data with excellent performance characteristics while maintaining data integrity and enabling advanced features like time-travel queries.
