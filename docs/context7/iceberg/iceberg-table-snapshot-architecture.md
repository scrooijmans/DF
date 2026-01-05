# Apache Iceberg: Table and Snapshot Architecture

This document explains Apache Iceberg's table and snapshot architecture, covering how immutable data files, metadata files, and manifests work together to enable logical edits, the call stack for overwriting data and handling deletes, and how Iceberg avoids rewriting entire datasets.

## Overview

Apache Iceberg is a table format for large-scale data lakes that provides:

- **ACID transactions**: Atomic, consistent, isolated, durable operations
- **Time travel**: Query historical snapshots
- **Schema evolution**: Change schema without rewriting data
- **Partition evolution**: Change partitioning without rewriting data
- **Efficient updates/deletes**: Logical modifications without full rewrites

## Table Architecture

### Table Structure

An Iceberg table consists of:

```
my_table/
  ├── data/
  │   ├── year=2020/month=01/
  │   │   ├── 00000-0-abc123.parquet
  │   │   └── 00001-0-def456.parquet
  │   └── year=2020/month=02/
  │       └── 00002-0-ghi789.parquet
  └── metadata/
      ├── v1.metadata.json
      ├── v2.metadata.json
      ├── snap-1234567890123456789-1-abc.avro    # Manifest list
      ├── snap-2345678901234567890-1-def.avro
      ├── manifest-00000-abc123.avro              # Manifest file
      ├── manifest-00001-def456.avro
      └── delete-00000-xyz789.avro               # Delete file
```

### Core Components

1. **Data Files**: Immutable Parquet files containing table data
2. **Metadata Files**: JSON files tracking table state
3. **Manifest Lists**: Avro files listing manifests for a snapshot
4. **Manifests**: Avro files listing data files and delete files
5. **Delete Files**: Files marking deleted rows (position or equality deletes)

## Snapshot Architecture

### Snapshot Concept

A **snapshot** is an immutable view of a table at a specific point in time. Each snapshot:

- References a manifest list
- Contains all data files active at that time
- Includes delete files for logical deletions
- Has a unique snapshot ID and timestamp

### Snapshot Structure

```json
{
	"snapshot-id": 1234567890123456789,
	"timestamp-ms": 1515488792485,
	"sequence-number": 1,
	"manifest-list": "s3://bucket/table/metadata/snap-1234567890123456789-1-abc.avro",
	"summary": {
		"operation": "append",
		"added-data-files": "5",
		"added-records": "1000",
		"added-files-size": "52428800"
	},
	"schema-id": 0
}
```

### Snapshot Hierarchy

```
Table Metadata (v2.metadata.json)
  └── Current Snapshot: 1234567890123456789
      └── Manifest List: snap-1234567890123456789-1-abc.avro
          ├── Manifest 1: manifest-00000-abc123.avro
          │   ├── Data File 1: data/part-00000.parquet
          │   ├── Data File 2: data/part-00001.parquet
          │   └── Delete File 1: delete-00000-xyz789.avro
          └── Manifest 2: manifest-00001-def456.avro
              └── Data File 3: data/part-00002.parquet
```

## Immutable Data Files

### Data File Properties

Data files in Iceberg are:

- **Immutable**: Never modified after creation
- **Self-contained**: Complete data for their partition/range
- **Parquet format**: Columnar storage with compression
- **Tracked in manifests**: Referenced by manifest entries

### Data File Structure

```parquet
// Data file: data/year=2020/month=01/00000-0-abc123.parquet
// Contains actual table data in Parquet format
// Never modified after creation
```

## Metadata Files

### Metadata File Structure

Metadata files are JSON files that track:

- Table schema
- Partition specification
- Current snapshot reference
- Snapshot history
- Table properties

```json
{
	"format-version": 2,
	"table-uuid": "9c12d241-149e-49d1-bb56-1273e81491cc",
	"location": "s3://bucket/table",
	"last-sequence-number": 1,
	"last-updated-ms": 1515488792485,
	"last-column-id": 5,
	"current-schema-id": 0,
	"schemas": [
		{
			"schema-id": 0,
			"type": "struct",
			"fields": [
				{ "id": 1, "name": "id", "type": "long", "required": true },
				{ "id": 2, "name": "name", "type": "string", "required": false }
			]
		}
	],
	"partition-specs": [
		{
			"spec-id": 0,
			"fields": [{ "source-id": 3, "field-id": 1000, "name": "year", "transform": "identity" }]
		}
	],
	"current-snapshot-id": 1234567890123456789,
	"snapshots": [
		{
			"snapshot-id": 1234567890123456789,
			"timestamp-ms": 1515488792485,
			"sequence-number": 1,
			"manifest-list": "s3://bucket/table/metadata/snap-1234567890123456789-1-abc.avro",
			"summary": {
				"operation": "append"
			}
		}
	],
	"snapshot-log": [
		{
			"snapshot-id": 1234567890123456789,
			"timestamp-ms": 1515488792485
		}
	]
}
```

### Metadata File Updates

When table state changes:

1. New metadata file is created (v3.metadata.json)
2. Old metadata file remains (for time travel)
3. Atomic swap: Table pointer updated to new metadata file
4. Old metadata can be cleaned up later

## Manifests

### Manifest Structure

A manifest is an immutable Avro file that lists:

- Data files with their metadata
- Delete files (position or equality deletes)
- Partition information
- File statistics (min/max values, null counts)

### Manifest Entry Schema

```avro
// Manifest entry structure
{
  "status": 1,  // 0 = EXISTING, 1 = ADDED, 2 = DELETED
  "snapshot_id": 1234567890123456789,
  "sequence_number": 1,
  "data_file": {
    "content": 0,  // 0 = DATA, 1 = POSITION_DELETES, 2 = EQUALITY_DELETES
    "file_path": "data/year=2020/month=01/00000-0-abc123.parquet",
    "file_format": "PARQUET",
    "partition": {"year": 2020, "month": 1},
    "record_count": 1000,
    "file_size_in_bytes": 52428800,
    "column_sizes": {...},
    "value_counts": {...},
    "null_value_counts": {...},
    "nan_value_counts": {...},
    "lower_bounds": {"id": 1, "name": "Alice"},
    "upper_bounds": {"id": 1000, "name": "Zoe"},
    "key_metadata": null,
    "split_offsets": [0, 26214400, 52428800],
    "equality_ids": null,
    "sort_order_id": 0
  }
}
```

### Manifest List

A manifest list is an Avro file that lists all manifests for a snapshot:

```avro
// Manifest list entry
{
  "manifest_path": "s3://bucket/table/metadata/manifest-00000-abc123.avro",
  "manifest_length": 1024000,
  "partition_spec_id": 0,
  "added_snapshot_id": 1234567890123456789,
  "added_data_files_count": 5,
  "existing_data_files_count": 0,
  "deleted_data_files_count": 0,
  "added_rows_count": 1000,
  "existing_rows_count": 0,
  "deleted_rows_count": 0,
  "partitions": [
    {
      "contains_null": false,
      "contains_nan": false,
      "lower_bound": {"year": 2020, "month": 1},
      "upper_bound": {"year": 2020, "month": 1}
    }
  ],
  "added_delete_files_count": 0,
  "existing_delete_files_count": 0,
  "deleted_delete_files_count": 0
}
```

## How Components Work Together for Logical Edits

### Logical Edit Process

When editing data in Iceberg:

1. **Read current snapshot**: Load manifest list and manifests
2. **Identify affected files**: Use statistics to find files containing target data
3. **Create new files**: Write updated data to new Parquet files
4. **Create delete files**: Mark old rows as deleted (if using MoR)
5. **Create new manifest**: List new data files and delete files
6. **Create new manifest list**: Reference new manifests
7. **Create new snapshot**: Point to new manifest list
8. **Update metadata**: Create new metadata file with new snapshot
9. **Atomic commit**: Swap table pointer to new metadata file

### Example: Update Operation

```
Initial State:
  Snapshot 1
    └── Manifest List 1
        └── Manifest 1
            └── Data File A (contains row id=123)

Update: SET name='NewName' WHERE id=123

Process:
  1. Read Data File A
  2. Filter row id=123
  3. Update row
  4. Write new Data File B (with updated row)
  5. Create Delete File (marking old row in File A)
  6. Create Manifest 2 (references File B and Delete File)
  7. Create Manifest List 2 (references Manifest 2 + unchanged manifests)
  8. Create Snapshot 2 (references Manifest List 2)
  9. Update metadata (v3.metadata.json) with Snapshot 2
  10. Atomic swap: table pointer → v3.metadata.json

Result:
  Snapshot 2
    └── Manifest List 2
        ├── Manifest 1 (unchanged, still references File A)
        └── Manifest 2 (new)
            ├── Data File B (new, with updated row)
            └── Delete File (marks row in File A as deleted)

Readers:
  - Read Snapshot 2
  - Read Data File A, apply Delete File → row id=123 excluded
  - Read Data File B → row id=123 with new name included
  - Result: Logical update achieved without modifying File A
```

## Overwrite Data: Complete Call Stack

### Overwrite Operation Flow

```java
// User code
INSERT OVERWRITE table PARTITION (year=2020) SELECT ...;
```

**Complete Call Stack:**

```
1. SparkSQL INSERT OVERWRITE
   ↓
2. IcebergTable.insertOverwrite()
   ↓
3. OverwriteFiles.newOverwrite()
   ↓
4. OverwriteFiles.overwriteByFilter() / overwriteByPartition()
   ↓
5. BaseRewriteFiles.plan()
   a. Load current snapshot
   b. Identify files to replace using filter/partition
   c. Plan rewrite operation
   ↓
6. BaseRewriteFiles.execute()
   ↓
7. RewriteDataFiles.rewrite()
   a. Read files to be replaced
   b. Filter out data in overwrite range
   c. Write new data files
   d. Create new manifest entries
   ↓
8. BaseRewriteFiles.commit()
   ↓
9. OverwriteFiles.commit()
   a. Create new manifest with:
      - New data files (ADDED)
      - Old data files (DELETED)
      - Unchanged data files (EXISTING)
   b. Create new manifest list
   c. Create new snapshot
   d. Update metadata
   ↓
10. TableOperations.commit()
    a. Read current metadata
    b. Validate no concurrent changes
    c. Write new metadata file
    d. Atomic swap: update table pointer
    ↓
11. Return success
```

### Detailed Overwrite Call Stack

```java
// Step-by-step overwrite process

// 1. User initiates overwrite
spark.sql("INSERT OVERWRITE table PARTITION (year=2020) SELECT * FROM source");

// 2. Iceberg table operations
class IcebergTable {
    void insertOverwrite(PartitionSpec spec, Map<String, String> partition, Dataset<Row> data) {
        // 2a. Start overwrite operation
        OverwriteFiles overwrite = newOverwrite();

        // 2b. Set partition filter
        overwrite.overwriteByPartition(partition);

        // 2c. Execute overwrite
        overwrite.commit();
    }
}

// 3. Overwrite planning
class OverwriteFiles {
    void commit() {
        // 3a. Get current snapshot
        Snapshot currentSnapshot = table.currentSnapshot();

        // 3b. Load manifest list
        ManifestList manifestList = loadManifestList(currentSnapshot.manifestListLocation());

        // 3c. Identify files to replace
        List<ManifestEntry> filesToReplace = new ArrayList<>();
        for (ManifestFile manifestFile : manifestList.manifests()) {
            Manifest manifest = loadManifest(manifestFile.path());
            for (ManifestEntry entry : manifest.entries()) {
                if (matchesPartition(entry, partitionFilter)) {
                    filesToReplace.add(entry);
                }
            }
        }

        // 3d. Read and rewrite data
        RewriteDataFiles rewrite = newRewriteDataFiles();
        for (ManifestEntry entry : filesToReplace) {
            // Read old file
            Dataset<Row> oldData = spark.read().parquet(entry.dataFile().path());

            // Filter out overwritten partition
            Dataset<Row> remainingData = oldData.filter(
                not(matchesPartition(oldData, partitionFilter))
            );

            // Combine with new data
            Dataset<Row> newData = remainingData.union(sourceData);

            // Write new file
            String newFilePath = writeParquetFile(newData);
            DataFile newFile = createDataFile(newFilePath);

            // Mark old file for deletion
            rewrite.deleteFile(entry.dataFile());
            rewrite.addFile(newFile);
        }

        // 3e. Commit rewrite
        rewrite.commit();
    }
}

// 4. Manifest creation
class RewriteDataFiles {
    void commit() {
        // 4a. Create new manifest entries
        List<ManifestEntry> newEntries = new ArrayList<>();

        // Add new files
        for (DataFile newFile : newFiles) {
            newEntries.add(ManifestEntry.added(
                currentSnapshot.snapshotId(),
                newFile
            ));
        }

        // Mark old files as deleted
        for (DataFile oldFile : deletedFiles) {
            newEntries.add(ManifestEntry.deleted(
                currentSnapshot.snapshotId(),
                oldFile
            ));
        }

        // Keep unchanged files
        for (DataFile unchangedFile : unchangedFiles) {
            newEntries.add(ManifestEntry.existing(
                currentSnapshot.snapshotId(),
                unchangedFile
            ));
        }

        // 4b. Write new manifest
        ManifestWriter writer = new ManifestWriter(spec, outputFile());
        for (ManifestEntry entry : newEntries) {
            writer.add(entry);
        }
        ManifestFile newManifest = writer.close();

        // 4c. Create manifest list
        ManifestListWriter listWriter = new ManifestListWriter(spec, outputFile());
        listWriter.add(newManifest);
        // Add unchanged manifests
        for (ManifestFile unchanged : unchangedManifests) {
            listWriter.add(unchanged);
        }
        String manifestListPath = listWriter.close();

        // 4d. Create new snapshot
        Snapshot newSnapshot = Snapshot.builder()
            .snapshotId(generateSnapshotId())
            .timestampMs(System.currentTimeMillis())
            .sequenceNumber(currentSnapshot.sequenceNumber() + 1)
            .manifestList(manifestListPath)
            .summary(createSummary("overwrite"))
            .build();

        // 4e. Update metadata
        TableMetadata newMetadata = currentMetadata()
            .replaceCurrentSnapshot(newSnapshot)
            .addSnapshot(newSnapshot)
            .incrementSequenceNumber();

        // 4f. Commit
        tableOperations.commit(currentMetadata, newMetadata);
    }
}

// 5. Atomic commit
class TableOperations {
    void commit(TableMetadata base, TableMetadata metadata) {
        // 5a. Validate no concurrent changes
        TableMetadata current = loadMetadata();
        if (!current.uuid().equals(base.uuid()) ||
            current.lastSequenceNumber() != base.lastSequenceNumber()) {
            throw new CommitFailedException("Concurrent modification detected");
        }

        // 5b. Write new metadata file
        String newMetadataPath = writeMetadataFile(metadata);

        // 5c. Atomic swap
        catalog.updateTablePointer(tableName, newMetadataPath);

        // 5d. Update in-memory cache
        refresh();
    }
}
```

## Handling Deletes

### Delete File Types

Iceberg supports two types of delete files:

**1. Position Delete Files**

- Mark rows by position within a data file
- Efficient for deleting specific rows
- Structure: `(file_path, pos, row)`

**2. Equality Delete Files**

- Mark rows by matching column values
- Efficient for deleting by key
- Structure: `(equality_columns, row_values)`

### Delete Process

```java
// Conceptual delete process
void deleteRows(Table table, Expression filter) {
    // 1. Get current snapshot
    Snapshot snapshot = table.currentSnapshot();

    // 2. Identify affected files using statistics
    List<DataFile> affectedFiles = findFiles(snapshot, filter);

    // 3. For each affected file
    for (DataFile file : affectedFiles) {
        // 3a. Read file
        Dataset<Row> data = spark.read().parquet(file.path());

        // 3b. Find matching rows
        Dataset<Row> matchingRows = data.filter(filter);

        if (matchingRows.count() == data.count()) {
            // All rows match: mark entire file for deletion
            markFileForDeletion(file);
        } else {
            // Partial match: create delete file
            DeleteFile deleteFile = createDeleteFile(file, matchingRows);
            addDeleteFile(deleteFile);
        }
    }

    // 4. Create new manifest with delete files
    Manifest newManifest = createManifest();
    for (DeleteFile deleteFile : deleteFiles) {
        newManifest.addDeleteFile(deleteFile);
    }

    // 5. Create new snapshot
    Snapshot newSnapshot = createSnapshot(newManifest);

    // 6. Commit
    table.manageSnapshots()
        .replaceSnapshot(snapshot.id(), newSnapshot)
        .commit();
}
```

### Position Delete File Example

```avro
// Position delete file structure
{
  "file_path": "data/year=2020/month=01/00000-0-abc123.parquet",
  "pos": 42,  // Row position 42 in the file
  "row": {    // Optional: full row data
    "id": 123,
    "name": "Old Name"
  }
}
```

### Equality Delete File Example

```avro
// Equality delete file structure
{
  "equality_ids": [1],  // Column ID 1 (id column)
  "row": {
    "id": 123  // Delete rows where id = 123
  }
}
```

### Delete During Read

```java
// How deletes are applied during reads
Dataset<Row> readWithDeletes(Table table, Expression filter) {
    Snapshot snapshot = table.currentSnapshot();

    // 1. Load manifest list
    ManifestList manifestList = loadManifestList(snapshot.manifestListLocation());

    // 2. Collect data files and delete files
    List<DataFile> dataFiles = new ArrayList<>();
    Map<String, List<DeleteFile>> deleteFilesByDataFile = new HashMap<>();

    for (ManifestFile manifestFile : manifestList.manifests()) {
        Manifest manifest = loadManifest(manifestFile.path());

        for (ManifestEntry entry : manifest.entries()) {
            if (entry.content() == Content.DATA) {
                dataFiles.add(entry.dataFile());
            } else if (entry.content() == Content.POSITION_DELETES) {
                DeleteFile deleteFile = entry.deleteFile();
                String targetFile = deleteFile.path();
                deleteFilesByDataFile.computeIfAbsent(targetFile, k -> new ArrayList<>())
                    .add(deleteFile);
            }
        }
    }

    // 3. Read data files and apply deletes
    Dataset<Row> result = spark.emptyDataFrame();

    for (DataFile dataFile : dataFiles) {
        Dataset<Row> fileData = spark.read().parquet(dataFile.path());

        // Apply position deletes
        List<DeleteFile> deletes = deleteFilesByDataFile.get(dataFile.path());
        if (deletes != null) {
            for (DeleteFile deleteFile : deletes) {
                if (deleteFile.content() == Content.POSITION_DELETES) {
                    // Filter out deleted positions
                    Set<Long> deletedPositions = loadDeletePositions(deleteFile);
                    fileData = fileData.filter(
                        row -> !deletedPositions.contains(row.position())
                    );
                } else if (deleteFile.content() == Content.EQUALITY_DELETES) {
                    // Filter out matching rows
                    fileData = applyEqualityDeletes(fileData, deleteFile);
                }
            }
        }

        result = result.union(fileData);
    }

    // 4. Apply query filter
    return result.filter(filter);
}
```

## Committing a New Snapshot

### Snapshot Commit Process

```java
// Complete snapshot commit process
class SnapshotCommit {
    void commit(Table table, List<DataFile> newFiles, List<DataFile> deletedFiles) {
        // 1. Load current metadata
        TableMetadata currentMetadata = table.operations().current();
        Snapshot currentSnapshot = currentMetadata.currentSnapshot();

        // 2. Create new manifest entries
        List<ManifestEntry> entries = new ArrayList<>();

        // Add new files
        for (DataFile file : newFiles) {
            entries.add(ManifestEntry.added(
                generateSnapshotId(),
                file
            ));
        }

        // Mark deleted files
        for (DataFile file : deletedFiles) {
            entries.add(ManifestEntry.deleted(
                generateSnapshotId(),
                file
            ));
        }

        // 3. Write new manifest
        ManifestWriter writer = ManifestFiles.write(
            table.spec(),
            outputFile("manifest-" + UUID.randomUUID() + ".avro")
        );

        for (ManifestEntry entry : entries) {
            writer.add(entry);
        }

        ManifestFile newManifest = writer.close();

        // 4. Create manifest list
        ManifestListWriter listWriter = ManifestListFiles.write(
            table.spec(),
            outputFile("snap-" + generateSnapshotId() + ".avro")
        );

        // Add new manifest
        listWriter.add(newManifest);

        // Add unchanged manifests from current snapshot
        ManifestList currentManifestList = loadManifestList(
            currentSnapshot.manifestListLocation()
        );
        for (ManifestFile existing : currentManifestList.manifests()) {
            if (!isAffected(existing, deletedFiles)) {
                listWriter.add(existing);
            }
        }

        String manifestListPath = listWriter.close();

        // 5. Create new snapshot
        long snapshotId = generateSnapshotId();
        Snapshot newSnapshot = Snapshot.builder()
            .snapshotId(snapshotId)
            .timestampMs(System.currentTimeMillis())
            .sequenceNumber(currentSnapshot.sequenceNumber() + 1)
            .manifestList(manifestListPath)
            .parentSnapshotId(currentSnapshot.snapshotId())
            .summary(createSummary("append", newFiles.size()))
            .build();

        // 6. Create new metadata
        TableMetadata newMetadata = currentMetadata
            .replaceCurrentSnapshot(newSnapshot)
            .addSnapshot(newSnapshot)
            .incrementSequenceNumber()
            .updateLastUpdatedMs(System.currentTimeMillis());

        // 7. Atomic commit
        table.operations().commit(currentMetadata, newMetadata);
    }
}
```

### Atomic Commit Mechanism

```java
// Atomic commit using optimistic concurrency
class TableOperations {
    void commit(TableMetadata base, TableMetadata metadata) {
        // 1. Read current metadata
        TableMetadata current = loadMetadata();

        // 2. Validate no concurrent changes
        if (current.uuid() != base.uuid() ||
            current.lastSequenceNumber() != base.lastSequenceNumber()) {
            throw new CommitFailedException("Concurrent modification");
        }

        // 3. Write new metadata file
        String newMetadataPath = writeMetadataFile(metadata);

        // 4. Atomic swap (depends on storage system)
        if (isS3()) {
            // S3: Use atomic rename
            atomicRename(tempFile, newMetadataPath);
        } else if (isHDFS()) {
            // HDFS: Use atomic rename
            fs.rename(tempFile, newMetadataPath);
        } else {
            // Other: Use two-phase commit
            writeMetadataFile(newMetadataPath);
            updateTablePointer(newMetadataPath);
        }

        // 5. Update cache
        refreshMetadata();
    }
}
```

## How Iceberg Avoids Rewriting Entire Datasets

### Key Strategies

#### 1. File-Level Granularity

Iceberg operates at the file level, not dataset level:

```java
// Update affects only specific files
UPDATE table SET name='New' WHERE id=123;

// Process:
// 1. Find files containing id=123 (using statistics)
// 2. Only those files are read and rewritten
// 3. Other files remain unchanged
// 4. New snapshot references unchanged files + new files
```

#### 2. Statistics-Based Pruning

Manifest entries contain statistics enabling file skipping:

```java
// Manifest entry statistics
{
  "lower_bounds": {"id": 1, "date": "2020-01-01"},
  "upper_bounds": {"id": 1000, "date": "2020-01-31"},
  "null_value_counts": {"id": 0, "date": 0}
}

// Query: WHERE id=500 AND date='2020-01-15'
// File can be skipped if:
// - upper_bounds.id < 500 OR lower_bounds.id > 500
// - OR date range doesn't overlap
```

#### 3. Manifest Reuse

Unchanged manifests are reused across snapshots:

```java
// Snapshot 1
ManifestList 1
  ├── Manifest A (files 1-100)
  └── Manifest B (files 101-200)

// Update: Modify file 50
// Snapshot 2
ManifestList 2
  ├── Manifest A' (files 1-49, 51-100, new file 50')
  └── Manifest B (reused, unchanged)

// Only Manifest A is rewritten, Manifest B is reused
```

#### 4. Delete Files (Merge-on-Read)

Instead of rewriting files, use delete files:

```java
// Delete: WHERE id=123
// Instead of rewriting file containing id=123:
// 1. Create position delete file marking row position
// 2. New snapshot includes delete file
// 3. During read: filter out deleted positions
// 4. Original data file never modified
```

#### 5. Partition Pruning

Only process files in affected partitions:

```java
// Overwrite: PARTITION (year=2020, month=01)
// Process:
// 1. Only files in year=2020/month=01 are affected
// 2. Files in other partitions remain unchanged
// 3. New snapshot references:
//    - New files for year=2020/month=01
//    - Unchanged files for other partitions
```

#### 6. Incremental Processing

Process only changed data:

```java
// MERGE INTO operation
MERGE INTO target
USING source
ON target.id = source.id
WHEN MATCHED THEN UPDATE
WHEN NOT MATCHED THEN INSERT

// Process:
// 1. Join target and source to find matches
// 2. Only matched rows are updated
// 3. Only new rows are inserted
// 4. Unmatched target rows remain unchanged
// 5. Only affected files are rewritten
```

### Example: Large Dataset Update

```java
// Scenario:
// - Table with 1 billion rows
// - 10,000 Parquet files
// - Update: SET status='active' WHERE id=12345

// Traditional approach (rewrite entire dataset):
// - Read all 10,000 files
// - Process 1 billion rows
// - Write 10,000 new files
// - Time: hours

// Iceberg approach:
// 1. Use statistics to find file containing id=12345
//    - Check lower_bounds and upper_bounds
//    - Result: 1 file contains id=12345
// 2. Read only that 1 file
// 3. Update the row
// 4. Write 1 new file
// 5. Create new manifest (1 new file, 1 deleted file)
// 6. Create new manifest list (reuse 9,999 other manifests)
// 7. Create new snapshot
// 8. Commit metadata
// - Time: seconds

// Result:
// - Only 1 file rewritten (not 10,000)
// - Only 1 row processed (not 1 billion)
// - 9,999 files unchanged and reused
// - New snapshot references:
//   - 1 new file (with updated row)
//   - 9,999 unchanged files (reused from previous snapshot)
```

## Copy-on-Write vs Merge-on-Read

### Copy-on-Write (CoW)

**Process:**

- Updates rewrite affected data files
- Delete files not used
- All data in data files

**Pros:**

- Optimal read performance
- No delete file processing
- Simpler read logic

**Cons:**

- Slower writes (file rewriting)
- More I/O for updates

**Use Case:** Read-heavy workloads

### Merge-on-Read (MoR)

**Process:**

- Updates create delete files
- New data in new files
- Deletes applied during read

**Pros:**

- Faster writes (no file rewriting)
- Less I/O for updates
- Better for frequent small updates

**Cons:**

- Slower reads (delete file processing)
- More complex read logic

**Use Case:** Write-heavy workloads, frequent updates

## Best Practices

### 1. Optimize File Size

- Target file size: 128MB - 1GB
- Too small: Many files, more metadata
- Too large: Less granular updates

### 2. Use Appropriate Delete Strategy

- **Position deletes**: For specific row deletions
- **Equality deletes**: For key-based deletions
- **File deletion**: When entire file is obsolete

### 3. Regular Maintenance

- **Rewrite data files**: Compact small files
- **Rewrite manifests**: Optimize manifest structure
- **Expire snapshots**: Clean up old metadata

### 4. Leverage Statistics

- Collect statistics on frequently filtered columns
- Use Z-order/clustering for better statistics
- Enable automatic statistics collection

### 5. Partitioning Strategy

- Partition by frequently filtered columns
- Avoid over-partitioning (too many small partitions)
- Use hidden partitioning when possible

## Summary

Apache Iceberg's architecture enables efficient logical edits through:

1. **Immutable Components**: Data files, manifests, and metadata files are never modified
2. **Snapshot-Based State**: Each snapshot represents a consistent table state
3. **Manifest Hierarchy**: Manifest lists → Manifests → Data files
4. **Logical Edits**: Updates create new files and mark old ones, not in-place modifications
5. **Statistics Pruning**: Skip irrelevant files using min/max statistics
6. **Manifest Reuse**: Unchanged manifests reused across snapshots
7. **Delete Files**: Logical deletions without rewriting data files
8. **Atomic Commits**: Metadata updates are atomic via pointer swaps

This architecture provides ACID transactions, time-travel queries, and efficient updates/deletes without requiring full dataset rewrites, making it highly scalable for large data lakes.
