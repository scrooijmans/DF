# DVC: Blob Storage and Metadata Tracking Architecture

This document explains how DVC (Data Version Control) manages edits to large data files using blob storage and metadata tracking, covering file change detection, blob version creation, deduplication, pipeline references, and the complete end-to-end flow for modifying and committing datasets.

## Overview

DVC is a version control system designed for large files and datasets. It integrates with Git to provide:

- **Content-addressable storage**: Files stored by content hash, not path
- **Deduplication**: Same content stored only once
- **Metadata tracking**: `.dvc` files track file metadata in Git
- **Blob storage**: Actual data stored in cache and remote storage
- **Pipeline integration**: Pipelines reference specific data versions

## Core Architecture

### DVC Project Structure

```
my_project/
  ├── data/
  │   ├── dataset.csv          # Large data file (not in Git)
  │   └── dataset.csv.dvc     # Metadata file (in Git)
  ├── models/
  │   ├── model.pkl           # Model file (not in Git)
  │   └── model.pkl.dvc       # Metadata file (in Git)
  ├── .dvc/
  │   ├── cache/              # Local blob storage
  │   │   └── files/
  │   │       └── md5/
  │   │           ├── a8/
  │   │           │   └── d60da582524dac805fc7b64d762e58
  │   │           └── 8e/
  │   │               └── 4ed00d7118e31340db6c0ba572658e
  │   ├── config              # DVC configuration
  │   └── tmp/                # Temporary files
  ├── dvc.yaml                # Pipeline definition
  ├── dvc.lock                 # Locked pipeline state
  └── .gitignore              # Excludes data files from Git
```

### Component Overview

1. **Data Files**: Large files tracked by DVC (excluded from Git)
2. **`.dvc` Files**: Metadata files tracked by Git
3. **Cache**: Local blob storage (`.dvc/cache/`)
4. **Remote Storage**: Remote blob storage (S3, GCS, SSH, etc.)
5. **Metadata**: File hashes, sizes, paths stored in `.dvc` files

## Blob Storage Architecture

### Content-Addressable Storage

DVC uses content-addressable storage where files are identified by their content hash (MD5), not by path:

```
File: data/dataset.csv
Content: "Hello, World!"
MD5 Hash: a8d60da582524dac805fc7b64d762e58

Cache Location: .dvc/cache/files/md5/a8/d60da582524dac805fc7b64d762e58
```

### Cache Structure

```
.dvc/cache/
└── files/
    └── md5/
        ├── a8/                    # First 2 chars of hash
        │   └── d60da582524dac805fc7b64d762e58  # Full hash as filename
        ├── 8e/
        │   └── 4ed00d7118e31340db6c0ba572658e
        └── ec/
            └── 1d2935f811b77cc49b031b999cbf17
```

**Benefits:**

- **Deduplication**: Same content = same hash = same blob
- **Fast lookup**: Two-level directory structure
- **Immutable**: Blobs never modified (new content = new hash)

### Directory Structure

For directories, DVC creates a `.dir` file containing a JSON mapping:

```json
// .dvc/cache/files/md5/6f/db5336fce0dbfd669f83065f107551.dir
[
	{ "md5": "de7371b0119f4f75f9de703c7c3bac16", "relpath": "cat.jpeg" },
	{ "md5": "402e97968614f583ece3b35555971f64", "relpath": "index.jpeg" }
]
```

**Directory Hash Calculation:**

1. Hash each file in directory
2. Create JSON array mapping hashes to relative paths
3. Hash the JSON array → directory hash

## File Change Detection

### Change Detection Mechanism

DVC detects file changes by comparing:

1. **File hash (MD5)**: Primary method for detecting content changes
2. **File size**: Quick check before computing hash
3. **File modification time**: Optimization to avoid recomputing hash
4. **Inode number**: Filesystem-level optimization

### Change Detection Process

```python
# Conceptual change detection
class DVCFileTracker:
    def detect_changes(self, file_path):
        # 1. Read .dvc file to get expected hash
        dvc_file = read_dvc_file(file_path + ".dvc")
        expected_hash = dvc_file.outs[0].md5
        expected_size = dvc_file.outs[0].size

        # 2. Check file exists
        if not exists(file_path):
            return ChangeType.DELETED

        # 3. Quick check: file size
        actual_size = get_file_size(file_path)
        if actual_size != expected_size:
            return ChangeType.MODIFIED

        # 4. Check modification time (optimization)
        # If mtime hasn't changed and inode is same, skip hash
        if (get_mtime(file_path) == dvc_file.mtime and
            get_inode(file_path) == dvc_file.inode):
            return ChangeType.UNCHANGED

        # 5. Compute hash (only if needed)
        actual_hash = compute_md5(file_path)

        # 6. Compare
        if actual_hash != expected_hash:
            return ChangeType.MODIFIED
        else:
            # Update metadata (mtime, inode) in .dvc file
            update_metadata(file_path, dvc_file)
            return ChangeType.UNCHANGED
```

### Status Check

```bash
# Check for changes
$ dvc status

# Output shows:
# - Modified files (hash changed)
# - Deleted files (file missing)
# - New files (not tracked)
# - Files not in cache (need to fetch)
```

**Status Output Example:**

```
data/data.xml:
    changed: md5
    changed: size
    new:     cache

models/model.pkl:
    deleted
```

## Creating New Blob Versions

### Blob Creation Process

When a file is modified:

```python
# Conceptual blob creation
def create_new_blob(file_path):
    # 1. Compute file hash
    file_hash = compute_md5(file_path)

    # 2. Check if blob already exists in cache
    cache_path = get_cache_path(file_hash)
    if exists_in_cache(cache_path):
        # Blob already exists (deduplication)
        return file_hash

    # 3. Determine cache path
    hash_prefix = file_hash[:2]  # First 2 characters
    cache_dir = f".dvc/cache/files/md5/{hash_prefix}"
    cache_file = f"{cache_dir}/{file_hash}"

    # 4. Copy file to cache
    # Use appropriate link type (copy, symlink, hardlink, reflink)
    link_type = get_link_type()
    if link_type == "copy":
        copy_file(file_path, cache_file)
    elif link_type == "symlink":
        create_symlink(cache_file, file_path)
    elif link_type == "hardlink":
        create_hardlink(cache_file, file_path)
    elif link_type == "reflink":
        create_reflink(cache_file, file_path)

    # 5. Update .dvc file
    update_dvc_file(file_path, file_hash)

    return file_hash
```

### Link Types

DVC supports different link types to optimize storage:

```bash
# Copy (default, most compatible)
dvc config cache.type copy

# Symlinks (saves space, requires support)
dvc config cache.type symlinks

# Hardlinks (saves space, same filesystem)
dvc config cache.type hardlinks

# Reflinks (copy-on-write, best performance)
dvc config cache.type reflinks
```

**Link Type Comparison:**

- **Copy**: Full copy, most compatible, uses most space
- **Symlink**: Points to cache, saves space, may break if cache moved
- **Hardlink**: Same inode, saves space, same filesystem only
- **Reflink**: Copy-on-write, best of both worlds (if supported)

## Deduplication

### How Deduplication Works

DVC automatically deduplicates files based on content hash:

```python
# Deduplication example
def add_file_to_cache(file_path, content_hash):
    cache_path = get_cache_path(content_hash)

    # Check if blob already exists
    if exists(cache_path):
        # File with same content already cached
        # No need to copy again
        print(f"File already in cache: {content_hash}")
        return content_hash

    # New content, create new blob
    copy_to_cache(file_path, cache_path)
    return content_hash

# Example: Same file in different locations
add_file("data/images/cat.jpg", hash="abc123")  # Creates blob
add_file("data/photos/cat.jpg", hash="abc123")  # Reuses blob
# Result: Only one blob stored, both files reference it
```

### Deduplication Benefits

1. **Storage Efficiency**: Same content stored only once
2. **Network Efficiency**: Same file not uploaded multiple times
3. **Cache Efficiency**: Same file not downloaded multiple times

### Example: Deduplication in Action

```bash
# Add same file from different locations
$ dvc add data/train/images/cat.jpg
# Hash: abc123def456...
# Cache: .dvc/cache/files/md5/ab/c123def456...

$ dvc add data/test/images/cat.jpg  # Same file, different location
# Hash: abc123def456...  # Same hash!
# Cache: .dvc/cache/files/md5/ab/c123def456...  # Same blob reused

# Result: Only one copy in cache, both .dvc files reference same hash
```

## Metadata Tracking

### .dvc File Structure

A `.dvc` file contains metadata about the tracked file:

```yaml
# data/dataset.csv.dvc
outs:
  - md5: a8d60da582524dac805fc7b64d762e58
    size: 33471
    nfiles: 1
    path: dataset.csv
    cache: true
    metric: false
    persist: false
```

**Fields:**

- **md5**: Content hash of the file
- **size**: File size in bytes
- **nfiles**: Number of files (1 for single file, N for directory)
- **path**: Relative path to the file
- **cache**: Whether file is cached
- **metric**: Whether file is a metric
- **persist**: Whether file persists across runs

### Directory .dvc File

For directories:

```yaml
# data/images.dvc
outs:
  - md5: 6fdb5336fce0dbfd669f83065f107551
    size: 5242880
    nfiles: 2
    path: images
    cache: true
    dir: true
```

The directory hash represents the directory structure:

- Each file in directory has its own hash
- Directory hash = hash of JSON mapping file hashes to paths

## Pipeline References

### Pipeline Definition (dvc.yaml)

```yaml
# dvc.yaml
stages:
  prepare:
    cmd: python prepare.py
    deps:
      - data/raw_data.csv
      - prepare.py
    outs:
      - data/prepared.csv
    metrics:
      - metrics.json

  train:
    cmd: python train.py
    deps:
      - data/prepared.csv
      - train.py
    outs:
      - models/model.pkl
    metrics:
      - metrics.json
```

### Lock File (dvc.lock)

The lock file captures exact versions of all dependencies:

```yaml
# dvc.lock
stages:
  prepare:
    cmd: python prepare.py
    deps:
      - path: data/raw_data.csv
        md5: a8d60da582524dac805fc7b64d762e58
        size: 33471
      - path: prepare.py
        md5: b9e71cb693582525d916fc8b75d873e69
        size: 2048
    outs:
      - path: data/prepared.csv
        md5: c0f82ec804693636e027fc9c86e984f7a
        size: 45678
        size: 45678

  train:
    cmd: python train.py
    deps:
      - path: data/prepared.csv
        md5: c0f82ec804693636e027fc9c86e984f7a
        size: 45678
      - path: train.py
        md5: d1g93fd915704747f138gd0d97e095g8b
        size: 3072
    outs:
      - path: models/model.pkl
        md5: e2ha04ge026815858g249he1e88f106i9c
        size: 1048576
```

### Pipeline Execution

```python
# Conceptual pipeline execution
def execute_pipeline(stage_name):
    # 1. Load dvc.lock to get exact versions
    lock_file = load_lock_file()
    stage = lock_file.stages[stage_name]

    # 2. Check if stage needs to run
    if is_up_to_date(stage):
        print(f"Stage {stage_name} is up to date")
        return

    # 3. Ensure dependencies are available
    for dep in stage.deps:
        ensure_file_available(dep.path, dep.md5)

    # 4. Execute command
    execute_command(stage.cmd)

    # 5. Verify outputs match expected hashes
    for out in stage.outs:
        verify_output_hash(out.path, out.md5)

    # 6. Stage is complete
    mark_stage_complete(stage_name)

def ensure_file_available(file_path, expected_hash):
    # Check if file exists and hash matches
    if exists(file_path):
        actual_hash = compute_md5(file_path)
        if actual_hash == expected_hash:
            return  # File is correct

    # File missing or wrong, fetch from cache
    fetch_from_cache(expected_hash, file_path)
```

## End-to-End Flow: Modifying and Committing Dataset

### Complete Workflow

```
1. Modify Data File
   ↓
2. Detect Changes (dvc status)
   ↓
3. Compute New Hash
   ↓
4. Create New Blob (if not exists)
   ↓
5. Update .dvc File
   ↓
6. Commit .dvc File to Git
   ↓
7. Push Blob to Remote (dvc push)
   ↓
8. Push Git Changes (git push)
```

### Detailed Step-by-Step Flow

#### Step 1: Modify Data File

```bash
# User modifies the data file
$ echo "new data" >> data/dataset.csv
```

#### Step 2: Detect Changes

```bash
$ dvc status
data/dataset.csv:
    changed: md5
    changed: size
```

**Internal Process:**

```python
# DVC checks file status
def check_status():
    # Read .dvc file
    dvc_file = read_dvc_file("data/dataset.csv.dvc")
    expected_hash = dvc_file.outs[0].md5

    # Compute actual hash
    actual_hash = compute_md5("data/dataset.csv")

    # Compare
    if actual_hash != expected_hash:
        return Status.MODIFIED
```

#### Step 3: Add Modified File

```bash
$ dvc add data/dataset.csv
```

**Internal Process:**

```python
def dvc_add(file_path):
    # 1. Compute new hash
    new_hash = compute_md5(file_path)
    file_size = get_file_size(file_path)

    # 2. Check if blob exists in cache
    cache_path = get_cache_path(new_hash)
    if not exists(cache_path):
        # 3. Create new blob
        copy_to_cache(file_path, cache_path)

    # 4. Update .dvc file
    dvc_file = {
        "outs": [{
            "md5": new_hash,
            "size": file_size,
            "path": file_path,
            "cache": True
        }]
    }
    write_dvc_file(file_path + ".dvc", dvc_file)

    # 5. Update .gitignore (if needed)
    add_to_gitignore(file_path)
```

#### Step 4: Commit to Git

```bash
$ git add data/dataset.csv.dvc
$ git commit -m "Update dataset"
```

**What Gets Committed:**

- `data/dataset.csv.dvc` (metadata file with new hash)
- **NOT** `data/dataset.csv` (excluded by .gitignore)

#### Step 5: Push to Remote Storage

```bash
$ dvc push
```

**Internal Process:**

```python
def dvc_push():
    # 1. Read all .dvc files
    dvc_files = find_all_dvc_files()

    # 2. Collect all hashes
    hashes_to_push = set()
    for dvc_file in dvc_files:
        for out in dvc_file.outs:
            hashes_to_push.add(out.md5)

    # 3. Check remote for each hash
    remote = get_remote_storage()
    for file_hash in hashes_to_push:
        if not remote.has_file(file_hash):
            # 4. Upload blob
            cache_path = get_cache_path(file_hash)
            remote.upload(cache_path, file_hash)
            print(f"Uploaded: {file_hash}")
        else:
            print(f"Already exists: {file_hash}")
```

#### Step 6: Push Git Changes

```bash
$ git push
```

This pushes the `.dvc` file (metadata) to Git, allowing others to:

1. Pull Git changes (get `.dvc` file)
2. Run `dvc pull` to fetch actual data blobs

### Complete Example

```bash
# 1. Initial state
$ dvc add data/dataset.csv
# Creates: data/dataset.csv.dvc
# Hash: a8d60da582524dac805fc7b64d762e58
# Cache: .dvc/cache/files/md5/a8/d60da582524dac805fc7b64d762e58

$ git add data/dataset.csv.dvc .gitignore
$ git commit -m "Add dataset"
$ dvc push
$ git push

# 2. Modify dataset
$ echo "new row" >> data/dataset.csv

# 3. Detect and track changes
$ dvc status
# Output: data/dataset.csv: changed md5, changed size

$ dvc add data/dataset.csv
# Computes new hash: 8e4ed00d7118e31340db6c0ba572658e
# Creates new blob: .dvc/cache/files/md5/8e/4ed00d7118e31340db6c0ba572658e
# Updates: data/dataset.csv.dvc

# 4. Commit metadata
$ git add data/dataset.csv.dvc
$ git commit -m "Update dataset with new row"

# 5. Push data blob
$ dvc push
# Uploads: 8e4ed00d7118e31340db6c0ba572658e
# (Old blob a8d60da582524dac805fc7b64d762e58 remains in cache/remote)

# 6. Push Git changes
$ git push
```

## Pipeline Version References

### How Pipelines Reference Data Versions

```yaml
# dvc.yaml defines pipeline structure
stages:
  process:
    cmd: python process.py
    deps:
      - data/raw.csv  # References .dvc file
    outs:
      - data/processed.csv

# dvc.lock captures exact versions
stages:
  process:
    deps:
      - path: data/raw.csv
        md5: a8d60da582524dac805fc7b64d762e58  # Exact version
    outs:
      - path: data/processed.csv
        md5: c0f82ec804693636e027fc9c86e984f7a
```

### Pipeline Execution with Versioning

```python
# Conceptual pipeline execution
def run_pipeline():
    # 1. Load dvc.lock
    lock = load_lock_file()

    # 2. For each stage
    for stage_name, stage in lock.stages.items():
        # 3. Check dependencies
        for dep in stage.deps:
            # Ensure correct version is available
            ensure_version(dep.path, dep.md5)

        # 4. Check if stage needs to run
        if is_stage_up_to_date(stage):
            continue

        # 5. Execute stage
        execute_command(stage.cmd)

        # 6. Verify outputs
        for out in stage.outs:
            verify_output(out.path, out.md5)

        # 7. Update lock file
        update_lock_file(stage_name, stage)

def ensure_version(file_path, expected_hash):
    # Check if file exists with correct hash
    if exists(file_path):
        actual_hash = compute_md5(file_path)
        if actual_hash == expected_hash:
            return  # Correct version

    # Wrong version or missing, fetch from cache
    fetch_from_cache(expected_hash, file_path)
```

### Reproducing Pipelines

```bash
# Reproduce pipeline
$ dvc repro

# Process:
# 1. Load dvc.lock
# 2. Check each stage's dependencies
# 3. If dependencies changed, re-run stage
# 4. Update dvc.lock with new output hashes
# 5. Commit dvc.lock to Git
```

## Remote Storage

### Remote Storage Structure

Remote storage mirrors the cache structure:

```
Remote Storage (S3/GCS/etc.):
  files/
    md5/
      ├── a8/
      │   └── d60da582524dac805fc7b64d762e58
      ├── 8e/
      │   └── 4ed00d7118e31340db6c0ba572658e
      └── ec/
          └── 1d2935f811b77cc49b031b999cbf17
```

### Push/Pull Operations

```python
# Push: Upload blobs to remote
def dvc_push():
    # Find all hashes referenced in .dvc files
    hashes = collect_referenced_hashes()

    for file_hash in hashes:
        if not remote.has_file(file_hash):
            # Upload blob
            local_path = get_cache_path(file_hash)
            remote.upload(local_path, file_hash)

# Pull: Download blobs from remote
def dvc_pull():
    # Find all hashes referenced in .dvc files
    hashes = collect_referenced_hashes()

    for file_hash in hashes:
        if not exists_in_cache(file_hash):
            # Download blob
            remote_path = get_remote_path(file_hash)
            local_path = get_cache_path(file_hash)
            remote.download(remote_path, local_path)
```

## File Change Detection: Detailed Process

### Change Detection Algorithm

```python
class FileChangeDetector:
    def detect_changes(self, file_path):
        # 1. Load .dvc file
        dvc_file_path = file_path + ".dvc"
        if not exists(dvc_file_path):
            return ChangeType.NEW

        dvc_file = load_dvc_file(dvc_file_path)
        expected_hash = dvc_file.outs[0].md5
        expected_size = dvc_file.outs[0].size
        expected_mtime = dvc_file.mtime
        expected_inode = dvc_file.inode

        # 2. Check file exists
        if not exists(file_path):
            return ChangeType.DELETED

        # 3. Quick checks (optimizations)
        actual_size = get_file_size(file_path)
        if actual_size != expected_size:
            return ChangeType.MODIFIED

        actual_mtime = get_mtime(file_path)
        actual_inode = get_inode(file_path)

        # 4. Fast path: mtime and inode unchanged
        if (actual_mtime == expected_mtime and
            actual_inode == expected_inode):
            # File hasn't changed (filesystem-level check)
            return ChangeType.UNCHANGED

        # 5. Compute hash (expensive operation)
        actual_hash = compute_md5(file_path)

        # 6. Compare hashes
        if actual_hash != expected_hash:
            return ChangeType.MODIFIED
        else:
            # Hash matches, update metadata
            update_dvc_metadata(dvc_file, actual_mtime, actual_inode)
            return ChangeType.UNCHANGED
```

### Optimization: Avoiding Hash Recalculation

DVC uses filesystem metadata to avoid recomputing hashes:

```python
# Optimization: Use mtime and inode
def is_file_unchanged(file_path, dvc_file):
    # If mtime and inode match, file hasn't changed
    if (get_mtime(file_path) == dvc_file.mtime and
        get_inode(file_path) == dvc_file.inode):
        return True

    # Otherwise, need to compute hash
    return compute_md5(file_path) == dvc_file.outs[0].md5
```

## Blob Version Creation: Detailed Process

### Creating New Blob

```python
def create_blob_version(file_path):
    # 1. Compute content hash
    file_hash = compute_md5(file_path)
    file_size = get_file_size(file_path)

    # 2. Determine cache location
    hash_prefix = file_hash[:2]
    cache_dir = f".dvc/cache/files/md5/{hash_prefix}"
    cache_file = f"{cache_dir}/{file_hash}"

    # 3. Check if blob already exists (deduplication)
    if exists(cache_file):
        print(f"Blob already exists: {file_hash}")
        return file_hash

    # 4. Create cache directory
    mkdir_p(cache_dir)

    # 5. Copy file to cache (using configured link type)
    link_type = get_config("cache.type", "copy")

    if link_type == "copy":
        copy_file(file_path, cache_file)
    elif link_type == "symlink":
        # Create symlink from workspace to cache
        create_symlink(cache_file, file_path)
    elif link_type == "hardlink":
        # Create hardlink (same inode)
        create_hardlink(cache_file, file_path)
    elif link_type == "reflink":
        # Create reflink (copy-on-write)
        create_reflink(cache_file, file_path)

    # 6. Update .dvc file
    update_dvc_file(file_path, file_hash, file_size)

    return file_hash
```

### Directory Blob Creation

```python
def create_directory_blob(dir_path):
    # 1. Hash each file in directory
    file_hashes = {}
    for file_path in list_files(dir_path):
        rel_path = get_relative_path(file_path, dir_path)
        file_hash = compute_md5(file_path)
        file_hashes[rel_path] = file_hash

        # Ensure file blob exists
        ensure_blob_exists(file_path, file_hash)

    # 2. Create directory structure JSON
    dir_structure = [
        {"md5": hash, "relpath": path}
        for path, hash in file_hashes.items()
    ]

    # 3. Hash the directory structure
    dir_json = json.dumps(dir_structure, sort_keys=True)
    dir_hash = compute_md5_string(dir_json)

    # 4. Store .dir file in cache
    cache_path = get_cache_path(dir_hash) + ".dir"
    write_file(cache_path, dir_json)

    return dir_hash
```

## Deduplication: Detailed Mechanism

### Automatic Deduplication

```python
# Deduplication happens automatically
def add_file(file_path):
    # Compute hash
    file_hash = compute_md5(file_path)

    # Check cache
    cache_path = get_cache_path(file_hash)

    if exists(cache_path):
        # Same content already cached
        # No need to copy again
        print(f"Deduplicated: {file_hash}")
    else:
        # New content, create blob
        copy_to_cache(file_path, cache_path)

    # Update .dvc file
    update_dvc_file(file_path, file_hash)

    return file_hash

# Example: Same file, different locations
hash1 = add_file("data/train/cat.jpg")  # Creates blob abc123
hash2 = add_file("data/test/cat.jpg")   # Reuses blob abc123

# Result: Only one blob stored
# Both .dvc files reference same hash: abc123
```

### Cross-Project Deduplication

DVC can deduplicate across projects if they share cache:

```bash
# Project 1
$ dvc add data/dataset.csv
# Hash: abc123, Cache: .dvc/cache/files/md5/ab/c123

# Project 2 (different location, same file)
$ dvc add data/dataset.csv
# Hash: abc123, Reuses cache from Project 1 (if same cache location)
```

## Pipeline References: Complete Flow

### Pipeline Definition and Execution

```yaml
# dvc.yaml
stages:
  prepare:
    cmd: python prepare.py
    deps:
      - data/raw.csv
      - prepare.py
    outs:
      - data/prepared.csv

  train:
    cmd: python train.py
    deps:
      - data/prepared.csv
      - train.py
    outs:
      - models/model.pkl
```

### Lock File Generation

```python
# When dvc repro runs
def generate_lock_file():
    lock = {}

    for stage_name, stage_def in load_dvc_yaml().stages.items():
        # Execute stage
        execute_stage(stage_def)

        # Capture exact versions
        lock[stage_name] = {
            "cmd": stage_def.cmd,
            "deps": [],
            "outs": []
        }

        # Capture dependency hashes
        for dep_path in stage_def.deps:
            dep_hash = get_file_hash(dep_path)
            lock[stage_name]["deps"].append({
                "path": dep_path,
                "md5": dep_hash,
                "size": get_file_size(dep_path)
            })

        # Capture output hashes
        for out_path in stage_def.outs:
            out_hash = get_file_hash(out_path)
            lock[stage_name]["outs"].append({
                "path": out_path,
                "md5": out_hash,
                "size": get_file_size(out_path)
            })

    # Write lock file
    write_lock_file(lock)
```

### Pipeline Reproducibility

```python
def reproduce_pipeline():
    # 1. Load dvc.yaml and dvc.lock
    pipeline = load_dvc_yaml()
    lock = load_lock_file()

    # 2. For each stage
    for stage_name in pipeline.stages:
        stage_def = pipeline.stages[stage_name]
        stage_lock = lock.stages[stage_name]

        # 3. Check if stage needs to run
        if is_stage_up_to_date(stage_def, stage_lock):
            print(f"Stage {stage_name} is up to date")
            continue

        # 4. Ensure dependencies are available
        for dep in stage_lock.deps:
            ensure_file_version(dep.path, dep.md5)

        # 5. Execute stage
        execute_command(stage_def.cmd)

        # 6. Verify outputs
        for out in stage_lock.outs:
            verify_output_version(out.path, out.md5)

        # 7. Update lock file
        update_stage_lock(stage_name, stage_def)

def is_stage_up_to_date(stage_def, stage_lock):
    # Check if all outputs exist with correct hashes
    for out in stage_lock.outs:
        if not exists(out.path):
            return False

        actual_hash = compute_md5(out.path)
        if actual_hash != out.md5:
            return False

    # Check if any dependency changed
    for dep in stage_lock.deps:
        actual_hash = get_file_hash(dep.path)
        if actual_hash != dep.md5:
            return False  # Dependency changed, need to rerun

    return True  # Stage is up to date
```

## Complete End-to-End Example

### Scenario: Modify Dataset and Commit

```bash
# Initial state
$ dvc add data/dataset.csv
# Creates: data/dataset.csv.dvc
# Hash: a8d60da582524dac805fc7b64d762e58
# Cache: .dvc/cache/files/md5/a8/d60da582524dac805fc7b64d762e58

$ git add data/dataset.csv.dvc .gitignore
$ git commit -m "Add dataset"
$ dvc push
$ git push

# Modify dataset
$ echo "new data row" >> data/dataset.csv

# Check status
$ dvc status
# Output:
# data/dataset.csv:
#     changed: md5
#     changed: size

# Add modified file
$ dvc add data/dataset.csv
# Process:
# 1. Compute new hash: 8e4ed00d7118e31340db6c0ba572658e
# 2. Check cache: blob doesn't exist
# 3. Create new blob: .dvc/cache/files/md5/8e/4ed00d7118e31340db6c0ba572658e
# 4. Update: data/dataset.csv.dvc with new hash
# 5. Old blob (a8d60da582524dac805fc7b64d762e58) remains in cache

# Commit metadata
$ git add data/dataset.csv.dvc
$ git commit -m "Update dataset with new row"

# Push new blob
$ dvc push
# Process:
# 1. Read data/dataset.csv.dvc
# 2. Get hash: 8e4ed00d7118e31340db6c0ba572658e
# 3. Check remote: blob doesn't exist
# 4. Upload: .dvc/cache/files/md5/8e/4ed00d7118e31340db6c0ba572658e
# 5. Old blob remains in remote (for time travel)

# Push Git changes
$ git push
# Pushes: data/dataset.csv.dvc (with new hash)
```

### Internal Call Stack

```
1. User: dvc add data/dataset.csv
   ↓
2. DVCFileTracker.add_file()
   ↓
3. FileChangeDetector.detect_changes()
   - Compute MD5 hash
   - Check cache for existing blob
   ↓
4. BlobManager.create_blob()
   - Determine cache path from hash
   - Copy/link file to cache
   ↓
5. DVCFileWriter.update_dvc_file()
   - Write/update .dvc file with hash
   - Update .gitignore if needed
   ↓
6. Return success

7. User: git add data/dataset.csv.dvc
   ↓
8. Git stages .dvc file

9. User: git commit
   ↓
10. Git commits .dvc file (metadata only)

11. User: dvc push
    ↓
12. RemoteManager.push()
    - Read all .dvc files
    - Collect all hashes
    - For each hash:
      a. Check if exists in remote
      b. If not, upload blob from cache
    ↓
13. Return success

14. User: git push
    ↓
15. Git pushes .dvc file to remote
```

## Best Practices

### 1. Cache Management

- **Regular cleanup**: Use `dvc gc` to remove unused blobs
- **Cache location**: Use shared cache for multiple projects
- **Link types**: Use reflinks when supported for best performance

### 2. Remote Storage

- **Configure early**: Set up remote storage before pushing
- **Multiple remotes**: Use different remotes for different data types
- **Regular sync**: Push/pull regularly to keep remotes in sync

### 3. Pipeline Management

- **Commit dvc.lock**: Always commit `dvc.lock` to Git
- **Reproduce regularly**: Run `dvc repro` to ensure reproducibility
- **Tag versions**: Use Git tags to mark important data versions

### 4. File Organization

- **Logical grouping**: Group related files in directories
- **Naming conventions**: Use clear, descriptive names
- **.dvcignore**: Exclude unnecessary files from tracking

## Summary

DVC manages edits to large data files through:

1. **Content-Addressable Storage**: Files identified by MD5 hash, not path
2. **Blob Storage**: Files stored in cache and remote storage by hash
3. **Metadata Tracking**: `.dvc` files track file metadata in Git
4. **Change Detection**: Hash-based detection with filesystem optimizations
5. **Deduplication**: Same content stored only once
6. **Pipeline References**: `dvc.lock` captures exact data versions
7. **Immutable Blobs**: Blobs never modified, new content = new blob
8. **Atomic Operations**: Metadata updates are atomic via Git commits

This architecture provides efficient versioning of large files, deduplication, and seamless integration with Git workflows while avoiding the need to store large files in Git repositories.
