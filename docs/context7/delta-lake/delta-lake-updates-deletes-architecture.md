# Delta Lake Architecture: Updates and Deletes on Immutable Parquet Files

This document explains how Delta Lake implements updates and deletes on immutable Parquet files, covering the transaction log, snapshot isolation, file writing, reader state determination, and the complete call flow for update operations.

## Overview

Delta Lake is a storage layer that brings ACID transactions to data lakes. It manages immutable Parquet files through:

- **Transaction Log**: JSON/Parquet files recording all changes
- **Snapshot Isolation**: Readers see consistent snapshots
- **Immutable Files**: Parquet files are never modified in-place
- **Logical Updates**: Updates create new files and mark old ones for removal
- **Deletion Vectors**: Efficient row-level deletions (Delta Lake 2.3.0+)

## Transaction Log Structure

### Delta Log Location

The transaction log is stored in the `_delta_log` directory:

```
my_table/
  ├── part-00000-xxx.parquet
  ├── part-00001-xxx.parquet
  ├── part-00002-xxx.parquet
  └── _delta_log/
      ├── 00000000000000000000.json    # Version 0
      ├── 00000000000000000001.json    # Version 1
      ├── 00000000000000000002.json    # Version 2
      ├── 00000000000000000003.json    # Version 3
      ├── 00000000000000000003.checkpoint.parquet  # Checkpoint
      └── ...
```

### Transaction Log Entry Format

Each transaction log entry (JSON file) contains a series of actions:

```json
{
	"commitInfo": {
		"timestamp": 1515488792485,
		"operation": "UPDATE",
		"operationParameters": {},
		"readVersion": 2,
		"isolationLevel": "WriteSerializable",
		"isBlindAppend": false,
		"operationMetrics": {},
		"engineInfo": "Apache-Spark/3.0.0",
		"txnId": "abc123"
	},
	"metaData": {
		"id": "table-id",
		"name": "my_table",
		"description": "Example table",
		"format": {
			"provider": "parquet",
			"options": {}
		},
		"schemaString": "{\"type\":\"struct\",\"fields\":[...]}",
		"partitionColumns": ["date"],
		"configuration": {},
		"createdTime": 1515488792485
	},
	"add": {
		"path": "part-00003-xxx.parquet",
		"partitionValues": { "date": "2017-12-10" },
		"size": 841454,
		"modificationTime": 1515488792485,
		"dataChange": true,
		"stats": "{\"numRecords\":1000,\"minValues\":{...},\"maxValues\":{...}}"
	},
	"remove": {
		"path": "part-00001-xxx.parquet",
		"deletionTimestamp": 1515488792485,
		"dataChange": true
	}
}
```

### Action Types

**1. Protocol Action**

```json
{
	"protocol": {
		"minReaderVersion": 1,
		"minWriterVersion": 2
	}
}
```

**2. Metadata Action**

```json
{
	"metaData": {
		"id": "table-id",
		"name": "table_name",
		"schemaString": "...",
		"partitionColumns": ["col1", "col2"],
		"configuration": {}
	}
}
```

**3. Add File Action**

```json
{
	"add": {
		"path": "part-00003-xxx.parquet",
		"partitionValues": { "date": "2017-12-10" },
		"size": 841454,
		"modificationTime": 1515488792485,
		"dataChange": true,
		"stats": "...",
		"deletionVector": {
			// Optional (Delta 2.3.0+)
			"storageType": "u",
			"pathOrInlineDv": "...",
			"offset": 0,
			"sizeInBytes": 1024,
			"cardinality": 100
		}
	}
}
```

**4. Remove File Action**

```json
{
	"remove": {
		"path": "part-00001-xxx.parquet",
		"deletionTimestamp": 1515488792485,
		"dataChange": true,
		"extendedFileMetadata": true,
		"partitionValues": { "date": "2017-12-10" },
		"size": 841454
	}
}
```

**5. Commit Info Action**

```json
{
	"commitInfo": {
		"timestamp": 1515488792485,
		"operation": "UPDATE",
		"operationParameters": {},
		"readVersion": 2,
		"isolationLevel": "WriteSerializable",
		"isBlindAppend": false
	}
}
```

## Snapshot Isolation

### Snapshot Concept

A **snapshot** represents the state of a Delta table at a specific version. It is computed by:

1. Reading all transaction log entries up to a version
2. Applying reconciliation rules to determine active files
3. Presenting a consistent view of the table

### Snapshot Reconciliation

```java
// Conceptual snapshot reconciliation
class Snapshot {
    private long version;
    private Metadata metadata;
    private List<AddFile> activeFiles;

    Snapshot reconcile(List<LogEntry> logEntries) {
        // Latest protocol wins
        Protocol latestProtocol = null;

        // Latest metadata wins
        Metadata latestMetadata = null;

        // Track files: add adds, remove removes
        Map<String, AddFile> fileMap = new HashMap<>();

        for (LogEntry entry : logEntries) {
            if (entry.hasProtocol()) {
                latestProtocol = entry.getProtocol();
            }

            if (entry.hasMetadata()) {
                latestMetadata = entry.getMetadata();
            }

            if (entry.hasAddFile()) {
                AddFile addFile = entry.getAddFile();
                fileMap.put(addFile.getPath(), addFile);
            }

            if (entry.hasRemoveFile()) {
                RemoveFile removeFile = entry.getRemoveFile();
                fileMap.remove(removeFile.getPath());
            }
        }

        return new Snapshot(version, latestMetadata, new ArrayList<>(fileMap.values()));
    }
}
```

### Isolation Levels

**1. Serializable (Default)**

- Highest isolation
- Prevents write-write conflicts
- Readers see committed writes

**2. WriteSerializable**

- Allows concurrent writes to different partitions
- Better performance for partitioned tables
- Still prevents conflicts

### Snapshot Reading

```java
// Reader opens snapshot at specific version
DeltaLog deltaLog = DeltaLog.forTable(spark, "/path/to/table");
Snapshot snapshot = deltaLog.getSnapshotForVersionAsOf(version);

// Or get latest snapshot
Snapshot snapshot = deltaLog.update();

// Read active files
List<AddFile> activeFiles = snapshot.getAllFiles();

// Read with predicate pushdown
DeltaScan scan = snapshot.scan(predicate);
```

## Updates: Creating New Parquet Files

### Update Process Overview

When updating data in Delta Lake:

1. **Read current state**: Load snapshot to identify affected files
2. **Filter affected files**: Determine which Parquet files contain rows to update
3. **Read and modify**: Read affected files, apply updates
4. **Write new files**: Write updated data to new Parquet files
5. **Commit transaction**: Record add/remove actions in transaction log

### Traditional Update (Without Deletion Vectors)

```java
// Conceptual update process
void updateTable(String tablePath, Predicate condition, Map<String, Object> updates) {
    // 1. Open Delta log and get current snapshot
    DeltaLog deltaLog = DeltaLog.forTable(spark, tablePath);
    Snapshot snapshot = deltaLog.update();

    // 2. Identify affected files using statistics
    List<AddFile> affectedFiles = snapshot.scan(condition).getFiles();

    // 3. Read affected files
    Dataset<Row> affectedData = spark.read()
        .parquet(affectedFiles.stream()
            .map(AddFile::getPath)
            .toArray(String[]::new));

    // 4. Apply updates
    Dataset<Row> updatedData = affectedData
        .withColumn("col1", when(condition, updates.get("col1")).otherwise(col("col1")))
        .withColumn("col2", when(condition, updates.get("col2")).otherwise(col("col2")));

    // 5. Write new Parquet files
    String outputPath = "/tmp/updated_data";
    updatedData.write()
        .mode("overwrite")
        .parquet(outputPath);

    // 6. Get new file information
    List<AddFile> newFiles = getNewFiles(outputPath);

    // 7. Start transaction
    OptimisticTransaction txn = deltaLog.startTransaction();

    // 8. Prepare actions
    List<Action> actions = new ArrayList<>();

    // Remove old files
    for (AddFile oldFile : affectedFiles) {
        actions.add(oldFile.remove());
    }

    // Add new files
    actions.addAll(newFiles);

    // 9. Commit transaction
    try {
        txn.commit(actions, new Operation(Operation.Name.UPDATE), "Spark/3.0.0");
    } catch (DeltaConcurrentModificationException e) {
        // Retry or handle conflict
    }
}
```

### Update with Deletion Vectors (Delta 2.3.0+)

Deletion vectors enable more efficient updates by avoiding full file rewrites:

```java
// Conceptual update with deletion vectors
void updateWithDeletionVectors(String tablePath,
                                Predicate condition,
                                Map<String, Object> updates) {
    DeltaLog deltaLog = DeltaLog.forTable(spark, tablePath);
    Snapshot snapshot = deltaLog.update();

    // 1. Identify affected files
    List<AddFile> affectedFiles = snapshot.scan(condition).getFiles();

    // 2. For each affected file:
    for (AddFile file : affectedFiles) {
        // Read file and find matching rows
        Dataset<Row> fileData = spark.read().parquet(file.getPath());
        Dataset<Row> matchingRows = fileData.filter(condition);

        if (matchingRows.count() == 0) {
            continue;  // No rows to update in this file
        }

        // 3. Create deletion vector for old rows
        DeletionVector dv = createDeletionVector(matchingRows);

        // 4. Write updated rows to new file
        Dataset<Row> updatedRows = matchingRows
            .withColumn("col1", lit(updates.get("col1")))
            .withColumn("col2", lit(updates.get("col2")));

        String newFilePath = writeNewParquetFile(updatedRows);
        AddFile newFile = createAddFile(newFilePath);
        newFile.setDeletionVector(null);  // New file has no deletions

        // 5. Update old file's deletion vector
        AddFile updatedOldFile = file.copy();
        updatedOldFile.setDeletionVector(mergeDeletionVectors(
            file.getDeletionVector(),
            dv
        ));

        // 6. Commit: add new file, update old file's deletion vector
        List<Action> actions = Arrays.asList(
            updatedOldFile,  // Updated with deletion vector
            newFile          // New file with updated rows
        );

        OptimisticTransaction txn = deltaLog.startTransaction();
        txn.commit(actions, new Operation(Operation.Name.UPDATE), "Spark/3.0.0");
    }
}
```

## Deletes: Marking Files for Removal

### Delete Process

```java
// Conceptual delete process
void deleteRows(String tablePath, Predicate condition) {
    DeltaLog deltaLog = DeltaLog.forTable(spark, tablePath);
    Snapshot snapshot = deltaLog.update();

    // 1. Identify affected files
    List<AddFile> affectedFiles = snapshot.scan(condition).getFiles();

    // 2. For each file, determine action:
    List<Action> actions = new ArrayList<>();

    for (AddFile file : affectedFiles) {
        Dataset<Row> fileData = spark.read().parquet(file.getPath());
        Dataset<Row> matchingRows = fileData.filter(condition);
        long matchingCount = matchingRows.count();
        long totalCount = fileData.count();

        if (matchingCount == totalCount) {
            // All rows match: remove entire file
            actions.add(file.remove());
        } else if (matchingCount == 0) {
            // No rows match: no action needed
            continue;
        } else {
            // Partial match: use deletion vector or rewrite
            if (useDeletionVectors) {
                DeletionVector dv = createDeletionVector(matchingRows);
                AddFile updatedFile = file.copy();
                updatedFile.setDeletionVector(mergeDeletionVectors(
                    file.getDeletionVector(),
                    dv
                ));
                actions.add(updatedFile);
            } else {
                // Rewrite file without deleted rows
                Dataset<Row> remainingRows = fileData.filter(not(condition));
                String newFilePath = writeNewParquetFile(remainingRows);
                AddFile newFile = createAddFile(newFilePath);

                actions.add(file.remove());
                actions.add(newFile);
            }
        }
    }

    // 3. Commit transaction
    OptimisticTransaction txn = deltaLog.startTransaction();
    txn.commit(actions, new Operation(Operation.Name.DELETE), "Spark/3.0.0");
}
```

## How Readers Determine Active Dataset State

### Reader Process

```java
// Conceptual reader process
class DeltaReader {
    Snapshot readSnapshot(String tablePath, Long version) {
        DeltaLog deltaLog = DeltaLog.forTable(spark, tablePath);

        // 1. Get snapshot at version (or latest)
        Snapshot snapshot = (version != null)
            ? deltaLog.getSnapshotForVersionAsOf(version)
            : deltaLog.update();

        // 2. Get all active files
        List<AddFile> activeFiles = snapshot.getAllFiles();

        // 3. Apply deletion vectors if present
        List<AddFile> filesToRead = new ArrayList<>();
        for (AddFile file : activeFiles) {
            if (file.hasDeletionVector()) {
                // File has deletions, but still active
                filesToRead.add(file);
            } else {
                // File is fully active
                filesToRead.add(file);
            }
        }

        return new ReadSnapshot(filesToRead, snapshot.getMetadata());
    }

    Dataset<Row> readData(Snapshot snapshot, Predicate predicate) {
        // 1. Filter files using statistics
        List<AddFile> relevantFiles = snapshot.scan(predicate).getFiles();

        // 2. Read Parquet files
        String[] filePaths = relevantFiles.stream()
            .map(AddFile::getPath)
            .toArray(String[]::new);

        Dataset<Row> data = spark.read().parquet(filePaths);

        // 3. Apply deletion vectors during read
        for (AddFile file : relevantFiles) {
            if (file.hasDeletionVector()) {
                DeletionVector dv = loadDeletionVector(file.getDeletionVector());
                // Filter out deleted rows
                data = data.filter(notInDeletionVector(dv));
            }
        }

        // 4. Apply predicate
        return data.filter(predicate);
    }
}
```

### Snapshot State Determination

```java
// How snapshot determines active files
class SnapshotReconciler {
    Snapshot reconcile(long version, List<LogEntry> logEntries) {
        Map<String, AddFile> activeFiles = new HashMap<>();
        Protocol protocol = null;
        Metadata metadata = null;

        // Process log entries in order
        for (LogEntry entry : logEntries) {
            // Latest protocol wins
            if (entry.hasProtocol()) {
                protocol = entry.getProtocol();
            }

            // Latest metadata wins
            if (entry.hasMetadata()) {
                metadata = entry.getMetadata();
            }

            // Add file: add to active set
            if (entry.hasAddFile()) {
                AddFile addFile = entry.getAddFile();
                activeFiles.put(addFile.getPath(), addFile);
            }

            // Remove file: remove from active set
            if (entry.hasRemoveFile()) {
                RemoveFile removeFile = entry.getRemoveFile();
                activeFiles.remove(removeFile.getPath());
            }
        }

        return new Snapshot(version, protocol, metadata,
                           new ArrayList<>(activeFiles.values()));
    }
}
```

## Complete Call Flow: Single-Row Update

### Call Stack for Update Operation

```
1. User Code
   UPDATE my_table SET col1 = 'new_value' WHERE id = 123;
   ↓
2. DeltaTable.update() / DeltaTable.updateExpr()
   ↓
3. DeltaTableOperations.update()
   ↓
4. OptimisticTransaction.start()
   - Read current snapshot
   - Get read version
   ↓
5. DeltaTableOperations.updateFiles()
   - Identify files containing matching rows
   - Use statistics for file pruning
   ↓
6. For each affected file:
   a. ReadParquetFile(file)
      - Load Parquet file
      - Apply existing deletion vectors (if any)
   ↓
   b. FilterMatchingRows(data, condition)
      - Apply WHERE clause
      - Identify rows to update
   ↓
   c. ApplyUpdates(matchingRows, updates)
      - Modify column values
      - Create updated rows
   ↓
   d. WriteNewParquetFile(updatedRows)
      - Serialize to Parquet
      - Write to storage
      - Collect statistics
      - Create AddFile action
   ↓
   e. CreateRemoveFileAction(oldFile)
      - Create RemoveFile action for old file
   ↓
7. OptimisticTransaction.commit()
   ↓
8. DeltaLog.commit()
   a. Validate no concurrent modifications
      - Check read version hasn't changed
   ↓
   b. Write transaction log entry
      - Serialize actions to JSON
      - Write to _delta_log/00000...{version}.json
      - Atomic write operation
   ↓
   c. Update in-memory state
      - Increment version
      - Update snapshot
   ↓
9. Return success / Handle conflicts
```

### Detailed Call Flow: Code Example

```java
// Step-by-step call flow

// 1. User initiates update
DeltaTable deltaTable = DeltaTable.forPath(spark, "/path/to/table");
deltaTable.update(
    col("id").equalTo(123),
    Map.of("col1", lit("new_value"))
);

// 2. Internal update process
class DeltaTableOperations {
    void update(Column condition, Map<String, Column> updates) {
        // 2a. Start transaction
        OptimisticTransaction txn = deltaLog.startTransaction();

        // 2b. Get current snapshot
        Snapshot snapshot = deltaLog.update();

        // 2c. Find affected files using statistics
        Expression predicate = condition.expr();
        DeltaScan scan = snapshot.scan(predicate);
        List<AddFile> affectedFiles = scan.getFiles();

        // 2d. Process each file
        List<Action> actions = new ArrayList<>();

        for (AddFile file : affectedFiles) {
            // 2e. Read file
            Dataset<Row> fileData = spark.read()
                .parquet(file.getPath());

            // 2f. Apply existing deletion vectors
            if (file.hasDeletionVector()) {
                DeletionVector dv = loadDeletionVector(file.getDeletionVector());
                fileData = applyDeletionVector(fileData, dv);
            }

            // 2g. Filter matching rows
            Dataset<Row> matchingRows = fileData.filter(condition);
            long matchCount = matchingRows.count();

            if (matchCount == 0) {
                continue;  // No matches in this file
            }

            // 2h. Apply updates
            Dataset<Row> updatedRows = matchingRows;
            for (Map.Entry<String, Column> update : updates.entrySet()) {
                updatedRows = updatedRows.withColumn(
                    update.getKey(),
                    update.getValue()
                );
            }

            // 2i. Get remaining rows (not updated)
            Dataset<Row> remainingRows = fileData.filter(not(condition));

            // 2j. Combine and write new file
            Dataset<Row> newFileData = remainingRows.union(updatedRows);
            String newFilePath = writeParquetFile(newFileData);

            // 2k. Create actions
            AddFile newFile = createAddFile(newFilePath, snapshot.getMetadata());
            RemoveFile removeFile = file.remove();

            actions.add(newFile);
            actions.add(removeFile);
        }

        // 2l. Commit transaction
        try {
            txn.commit(actions,
                      new Operation(Operation.Name.UPDATE),
                      "Spark/3.0.0");
        } catch (DeltaConcurrentModificationException e) {
            // Retry logic or error handling
            handleConflict(e);
        }
    }
}
```

## Scaling to Large Datasets

### Performance Optimizations

#### 1. Statistics-Based File Pruning

```java
// Use statistics to skip irrelevant files
DeltaScan scan = snapshot.scan(predicate);

// Statistics stored in AddFile:
// {
//   "numRecords": 1000,
//   "minValues": {"id": 1, "date": "2020-01-01"},
//   "maxValues": {"id": 1000, "date": "2020-12-31"},
//   "nullCount": {"id": 0, "date": 0}
// }

// Files are skipped if:
// - minValues > predicate.maxValue
// - maxValues < predicate.minValue
// - nullCount == numRecords (for NOT NULL predicates)
```

#### 2. Deletion Vectors for Small Updates

```java
// For small updates, use deletion vectors instead of rewriting
if (matchingRowCount < REWRITE_THRESHOLD) {
    // Use deletion vector
    DeletionVector dv = createDeletionVector(matchingRows);
    updateFileWithDeletionVector(file, dv);
} else {
    // Rewrite file
    rewriteFile(file, updatedData);
}
```

#### 3. Partition Pruning

```java
// Only process files in relevant partitions
if (predicate.hasPartitionFilter()) {
    List<String> relevantPartitions = extractPartitions(predicate);
    affectedFiles = affectedFiles.stream()
        .filter(file -> relevantPartitions.contains(file.getPartitionValues()))
        .collect(Collectors.toList());
}
```

#### 4. Parallel File Processing

```java
// Process multiple files in parallel
affectedFiles.parallelStream().forEach(file -> {
    processFile(file, condition, updates);
});
```

#### 5. Checkpointing

```java
// Checkpoints consolidate transaction log entries
// Instead of reading 1000 JSON files, read 1 Parquet checkpoint

// Checkpoint structure:
// _delta_log/
//   ├── 00000000000000000000.json
//   ├── ...
//   ├── 00000000000000000100.json
//   ├── 00000000000000000100.checkpoint.parquet  // Checkpoint
//   ├── 00000000000000000101.json
//   └── ...

// Readers:
// 1. Read checkpoint (fast, Parquet format)
// 2. Read incremental JSON files after checkpoint
// 3. Reconcile to get snapshot
```

### Scalability Characteristics

**1. File-Level Granularity**

- Updates only affect files containing matching rows
- Statistics enable skipping irrelevant files
- Scales linearly with number of affected files

**2. Deletion Vectors**

- Avoid rewriting entire files for small updates
- Deletion vectors are compressed bitmaps
- Efficient for sparse updates

**3. Checkpointing**

- Reduces transaction log read overhead
- Checkpoints written periodically (every 10 commits by default)
- Enables fast snapshot reconstruction

**4. Partitioning**

- Partition pruning limits files to process
- Updates isolated to relevant partitions
- Enables parallel processing

**5. Z-Ordering / Clustering**

- Co-locates related data
- Improves statistics accuracy
- Better file pruning

## Example: Complete Update Flow

```java
// Complete example: Update single row in large table

// 1. Table state
// - 10,000 Parquet files
// - 1 billion rows
// - Partitioned by date
// - Update: SET status = 'active' WHERE id = 12345

// 2. Process
DeltaTable table = DeltaTable.forPath(spark, "/large_table");

// 2a. Start transaction
OptimisticTransaction txn = table.deltaLog().startTransaction();

// 2b. Get snapshot
Snapshot snapshot = table.deltaLog().update();

// 2c. Use statistics to find relevant files
// Statistics show: id ranges from 1 to 1,000,000,000
// File with id=12345 likely in files with minValues.id <= 12345 <= maxValues.id
Expression predicate = new EqualTo(new Column("id"), lit(12345));
DeltaScan scan = snapshot.scan(predicate);

// Result: Only 1-2 files contain id=12345 (thanks to statistics)
List<AddFile> affectedFiles = scan.getFiles();  // ~1-2 files

// 2d. Read only affected files
Dataset<Row> data = spark.read()
    .parquet(affectedFiles.stream()
        .map(AddFile::getPath)
        .toArray(String[]::new));

// 2e. Apply update
Dataset<Row> updated = data.withColumn("status",
    when(col("id").equalTo(12345), lit("active"))
    .otherwise(col("status")));

// 2f. Write new file
String newFile = writeParquetFile(updated);

// 2g. Create actions
List<Action> actions = Arrays.asList(
    affectedFiles.get(0).remove(),  // Remove old file
    createAddFile(newFile)          // Add new file
);

// 2h. Commit
txn.commit(actions, new Operation(Operation.Name.UPDATE), "Spark/3.0.0");

// Result:
// - Only 1-2 files processed (not 10,000)
// - Only 1-2 files rewritten
// - Transaction log updated with 1 entry
// - Total time: seconds (not hours)
```

## Best Practices

### 1. Optimize for Updates

- **Use partitioning**: Partition by columns frequently used in WHERE clauses
- **Collect statistics**: Enable statistics collection for better pruning
- **Use deletion vectors**: Enable for tables with frequent small updates
- **Z-order/cluster**: Co-locate related data for better statistics

### 2. Manage Transaction Log

- **Regular checkpoints**: Ensure checkpoints are created
- **Vacuum old logs**: Clean up old transaction log entries
- **Monitor log size**: Large logs slow down snapshot reads

### 3. Handle Concurrency

- **Retry logic**: Implement retry for concurrent modification exceptions
- **Isolation levels**: Choose appropriate isolation level
- **Batch updates**: Group related updates in single transaction

### 4. Performance Tuning

- **File size**: Optimal file size (128MB - 1GB)
- **Statistics**: Collect statistics on frequently filtered columns
- **Compression**: Use appropriate Parquet compression
- **Parallelism**: Process files in parallel when possible

## Summary

Delta Lake implements updates and deletes on immutable Parquet files through:

1. **Transaction Log**: Records all changes as JSON/Parquet entries
2. **Snapshot Isolation**: Readers see consistent table state at a version
3. **Immutable Files**: Parquet files are never modified, new files are created
4. **Logical Updates**: Updates create new files and mark old ones for removal
5. **Deletion Vectors**: Efficient row-level deletions without full file rewrites
6. **Statistics Pruning**: Skip irrelevant files using min/max statistics
7. **Checkpointing**: Consolidate transaction log for fast snapshot reads

This architecture provides ACID transactions, time-travel queries, and scalable performance for large datasets while maintaining the immutability benefits of Parquet files.
