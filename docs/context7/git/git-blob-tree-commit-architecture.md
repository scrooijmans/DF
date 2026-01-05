# Git's Blob, Tree, and Commit Architecture

This document explains Git's internal architecture focusing on how file edits are represented through blobs, trees, and commits. It covers how modifying a single file creates new blobs and trees, how packfiles optimize storage, and why Git's immutability model inspired modern data systems.

## Overview

Git is a distributed version control system built on a content-addressable object database. Its architecture is based on four immutable object types:

- **Blobs**: Store file content
- **Trees**: Represent directory structures
- **Commits**: Record snapshots with metadata
- **Tags**: Symbolic references to objects

All objects are identified by their SHA-1 hash (or SHA-256 in newer versions), ensuring immutability and integrity.

## Core Object Types

### Blob Objects

A **blob** (Binary Large Object) stores the actual content of a file. It contains:

- **No metadata**: No filename, path, or permissions
- **Just content**: Raw file data
- **Content-addressable**: Identified by SHA-1 hash of content

**Blob Structure:**

```
Header: "blob <size>\0"
Content: <file content>
Hash: SHA-1(header + content)
```

**Example:**

```bash
# File: hello.txt
# Content: "Hello, World!"

# Blob creation:
echo "Hello, World!" | git hash-object -w --stdin
# Output: 8ab686eafeb1f44702738c8b0f24f2567c36da6d

# Blob stored at:
.git/objects/8a/b686eafeb1f44702738c8b0f24f2567c36da6d
```

**Blob Properties:**

- **Immutable**: Once created, never modified
- **Deduplicated**: Same content = same hash = same blob
- **Location-independent**: Same file in different paths shares blob

### Tree Objects

A **tree** object represents a directory structure. It contains:

- **Entries**: List of files and subdirectories
- **Metadata**: Mode (permissions), type, hash, and name for each entry
- **Hierarchical**: Can reference other trees (subdirectories)

**Tree Structure:**

```
Header: "tree <size>\0"
Entries:
  <mode> <name>\0<20-byte-sha1>
  <mode> <name>\0<20-byte-sha1>
  ...
```

**Tree Entry Format:**

- **Mode**: File permissions (100644 for regular file, 040000 for directory)
- **Name**: Filename or directory name
- **SHA-1**: Hash of blob (for files) or tree (for subdirectories)

**Example:**

```bash
# Tree object contents:
100644 blob 8ab686eafeb1f44702738c8b0f24f2567c36da6d    hello.txt
100644 blob 3b18e512dba79e4c8300dd08aeb37f8e728b8dad    world.txt
040000 tree 0123456789abcdef0123456789abcdef01234567    subdir
```

**Tree Properties:**

- **Immutable**: Represents exact directory state
- **Content-addressable**: Hash depends on all entries
- **Efficient comparison**: Two trees with same hash = identical structure

### Commit Objects

A **commit** object represents a snapshot of the repository at a point in time. It contains:

- **Tree reference**: SHA-1 of root tree object
- **Parent commits**: SHA-1(s) of parent commit(s)
- **Metadata**: Author, committer, timestamp, commit message

**Commit Structure:**

```
Header: "commit <size>\0"
Content:
tree <tree-sha1>
parent <parent-sha1>
parent <parent-sha1>  # For merge commits
author <name> <email> <timestamp> <timezone>
committer <name> <email> <timestamp> <timezone>

<commit message>
```

**Example:**

```
tree 1a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b
parent 9a8b7c6d5e4f3a2b1c0d9e8f7a6b5c4d3e2f1a0b
author John Doe <john@example.com> 1515488792 -0500
committer John Doe <john@example.com> 1515488792 -0500

Initial commit
```

**Commit Properties:**

- **Immutable**: Once created, never modified
- **Linked history**: Forms directed acyclic graph (DAG)
- **Complete snapshot**: References entire tree structure

## Object Storage

### Loose Objects

Initially, objects are stored as **loose objects**:

```
.git/objects/
├── 8a/
│   └── b686eafeb1f44702738c8b0f24f2567c36da6d  # Blob
├── 1a/
│   └── 2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b  # Tree
└── 9a/
    └── 8b7c6d5e4f3a2b1c0d9e8f7a6b5c4d3e2f1a0b  # Commit
```

**Structure:**

- Two-level directory: First 2 chars of hash
- Filename: Remaining 38 chars of hash
- Content: Zlib-compressed object data

### Object Format

```python
# Conceptual object creation
def create_object(obj_type, content):
    # 1. Create header
    header = f"{obj_type} {len(content)}\0"

    # 2. Combine header and content
    full_content = header.encode() + content

    # 3. Compute SHA-1 hash
    obj_hash = sha1(full_content).hexdigest()

    # 4. Compress with zlib
    compressed = zlib.compress(full_content)

    # 5. Determine storage path
    obj_dir = f".git/objects/{obj_hash[:2]}"
    obj_file = f"{obj_dir}/{obj_hash[2:]}"

    # 6. Write to disk
    mkdir_p(obj_dir)
    write_file(obj_file, compressed)

    return obj_hash
```

## How File Edits Create New Objects

### Single File Edit Process

When you modify a single file:

```
1. Edit file in working directory
   ↓
2. git add (stage changes)
   - Compute new blob hash
   - Create new blob object
   - Update index
   ↓
3. git commit
   - Create new tree (with new blob reference)
   - Create new commit (with new tree reference)
   - Update branch reference
```

### Detailed Process: Editing a File

**Initial State:**

```
Commit A (abc123...)
  └── Tree A (def456...)
      ├── file1.txt → Blob A (111aaa...)
      └── file2.txt → Blob B (222bbb...)
```

**Step 1: Edit file1.txt**

```bash
# User edits file1.txt
echo "new content" > file1.txt
```

**Step 2: Stage Changes (git add)**

```python
# Conceptual git add process
def git_add(file_path):
    # 1. Read file content
    content = read_file(file_path)

    # 2. Compute blob hash
    blob_hash = create_blob(content)
    # Result: New blob C (333ccc...) created

    # 3. Update index
    index.update_entry(file_path, blob_hash, get_mode(file_path))

    # 4. Blob stored at:
    # .git/objects/33/3ccc...
```

**Result After git add:**

```
Index:
  file1.txt → Blob C (333ccc...)  # New blob
  file2.txt → Blob B (222bbb...)  # Unchanged

Objects:
  Blob A (111aaa...)  # Old version (still exists)
  Blob B (222bbb...)  # Unchanged
  Blob C (333ccc...)  # New version
```

**Step 3: Commit (git commit)**

```python
# Conceptual git commit process
def git_commit(message, author, committer):
    # 1. Read index
    index_entries = read_index()

    # 2. Create tree from index
    # For each directory level:
    #   - Collect entries in that directory
    #   - Create tree object
    root_tree_hash = create_tree_from_index(index_entries)
    # Result: New Tree B (444ddd...) created

    # 3. Get parent commit
    parent_commit = get_head_commit()

    # 4. Create commit object
    commit_hash = create_commit(
        tree=root_tree_hash,
        parent=parent_commit.hash,
        message=message,
        author=author,
        committer=committer
    )
    # Result: New Commit B (555eee...) created

    # 5. Update HEAD
    update_ref("HEAD", commit_hash)

    return commit_hash
```

**Result After git commit:**

```
Commit B (555eee...)  # New commit
  └── Tree B (444ddd...)  # New tree
      ├── file1.txt → Blob C (333ccc...)  # New blob
      └── file2.txt → Blob B (222bbb...)  # Unchanged (reused)

Objects (all still exist):
  Commit A (abc123...)
    └── Tree A (def456...)
        ├── file1.txt → Blob A (111aaa...)  # Old version
        └── file2.txt → Blob B (222bbb...)

  Commit B (555eee...)  # New
    └── Tree B (444ddd...)  # New
        ├── file1.txt → Blob C (333ccc...)  # New
        └── file2.txt → Blob B (222bbb...)  # Reused
```

### Tree Creation Process

```python
# How trees are created from index
def create_tree_from_index(index_entries):
    # Group entries by directory
    dirs = {}
    for entry in index_entries:
        dir_path = dirname(entry.path)
        if dir_path not in dirs:
            dirs[dir_path] = []
        dirs[dir_path].append(entry)

    # Create trees bottom-up
    tree_hashes = {}

    # Process directories in depth order
    for dir_path in sorted_dirs_by_depth(dirs):
        entries = []

        # Add files in this directory
        for entry in dirs[dir_path]:
            if dirname(entry.path) == dir_path:
                entries.append({
                    "mode": entry.mode,
                    "name": basename(entry.path),
                    "hash": entry.blob_hash
                })

        # Add subdirectories (already created)
        for subdir in get_subdirs(dir_path):
            subdir_hash = tree_hashes[subdir]
            entries.append({
                "mode": "040000",  # Directory
                "name": basename(subdir),
                "hash": subdir_hash
            })

        # Create tree object
        tree_hash = create_tree_object(entries)
        tree_hashes[dir_path] = tree_hash

    # Return root tree
    return tree_hashes[""]
```

### Why New Objects Are Created

**Immutability Principle:**

- Objects are never modified
- New content = new hash = new object
- Old objects remain for history

**Example:**

```python
# Edit file1.txt
# Old content: "Hello"
# New content: "Hello, World!"

# Old blob hash: 8ab686eafeb1f44702738c8b0f24f2567c36da6d
# New blob hash: 3b18e512dba79e4c8300dd08aeb37f8e728b8dad

# Both blobs exist in .git/objects/
# Tree references new blob
# Old tree (with old blob) still exists in old commit
```

## Packfiles: Storage Optimization

### Why Packfiles?

As repositories grow, loose objects become inefficient:

- **Many small files**: One file per object
- **No compression across objects**: Each object compressed independently
- **No delta compression**: Similar objects stored separately

### Packfile Structure

A **packfile** is a compressed collection of objects:

```
.git/objects/pack/
├── pack-abc123.pack    # Pack file (compressed objects)
└── pack-abc123.idx     # Index file (object lookup)
```

**Packfile Format:**

```
Header: "PACK" + version + object_count
Objects:
  Object 1 (compressed, possibly delta)
  Object 2 (compressed, possibly delta)
  ...
Checksum: SHA-1 of all objects
```

### Delta Compression

Packfiles use **delta compression** to store similar objects efficiently:

**Delta Types:**

1. **REF_DELTA**: References another object by SHA-1
2. **OFS_DELTA**: References another object by offset in packfile

**Delta Example:**

```python
# Original blob: "Hello, World!"
# Modified blob: "Hello, World! How are you?"

# Instead of storing full content:
# Store as delta:
delta = {
    "base": original_blob_hash,
    "instructions": [
        "copy 13 bytes",  # "Hello, World!"
        "insert: How are you?"
    ]
}

# Delta is much smaller than full content
```

### Packfile Creation Process

```python
# Conceptual packfile creation
def create_packfile(objects):
    # 1. Sort objects for delta compression
    sorted_objects = sort_for_delta_compression(objects)
    # Sort by:
    #   - Type (blobs together, trees together)
    #   - Filename (similar names together)
    #   - Size (larger objects first)

    # 2. For each object, find best delta base
    packed_objects = []
    for obj in sorted_objects:
        # Find similar objects to delta against
        candidates = find_delta_candidates(obj, sorted_objects)

        if candidates:
            # Create delta
            best_base = find_best_delta_base(obj, candidates)
            delta = create_delta(obj, best_base)

            if len(delta) < len(obj.content):
                # Delta is smaller, use it
                packed_objects.append({
                    "type": "OFS_DELTA",
                    "base_offset": best_base.offset,
                    "delta": delta
                })
            else:
                # Full object is smaller
                packed_objects.append({
                    "type": "full",
                    "content": obj.content
                })
        else:
            # No good delta candidate
            packed_objects.append({
                "type": "full",
                "content": obj.content
            })

    # 3. Write packfile
    write_packfile(packed_objects)

    # 4. Create index for fast lookup
    create_pack_index(packed_objects)
```

### Packfile Heuristics

Git uses heuristics to optimize delta compression:

```python
# Delta compression heuristics
def find_delta_candidates(obj, all_objects):
    candidates = []

    for candidate in all_objects:
        # 1. Same type only
        if obj.type != candidate.type:
            continue

        # 2. Prefer same filename
        if obj.name == candidate.name:
            candidates.append((candidate, priority=HIGH))

        # 3. Prefer larger objects as base
        if candidate.size > obj.size:
            candidates.append((candidate, priority=MEDIUM))

        # 4. Size similarity
        size_ratio = min(obj.size, candidate.size) / max(obj.size, candidate.size)
        if size_ratio > 0.5:
            candidates.append((candidate, priority=LOW))

    # Sort by priority and limit window
    return sorted(candidates, key=lambda x: x[1])[:DELTA_WINDOW]
```

### Packfile Benefits

1. **Space Efficiency**: Delta compression reduces storage
2. **Network Efficiency**: Smaller transfers
3. **Fast Lookup**: Index enables O(1) object access
4. **Incremental Updates**: New objects can be added to packs

### Repacking

```bash
# Manual repack
git repack -a -d

# Process:
# 1. Collect all objects
# 2. Create new packfile with delta compression
# 3. Delete old loose objects and packfiles
# 4. Update references
```

## Complete Call Stack: File Edit and Commit

### Call Stack Overview

```
1. User: Edit file
   ↓
2. User: git add <file>
   ↓
3. Git: hash-object (compute blob hash)
   ↓
4. Git: write-object (store blob)
   ↓
5. Git: update-index (update staging area)
   ↓
6. User: git commit
   ↓
7. Git: write-tree (create tree from index)
   ↓
8. Git: commit-tree (create commit object)
   ↓
9. Git: update-ref (update HEAD/branch)
```

### Detailed Call Stack

#### Step 1: Edit File

```bash
# User edits file
$ echo "new content" > file.txt
```

#### Step 2: git add

```python
# git add file.txt

# 1. hash-object.c: hash_object_file()
def hash_object_file(file_path, obj_type="blob"):
    # Read file content
    content = read_file(file_path)

    # Create object header
    header = f"{obj_type} {len(content)}\0"
    full_content = header.encode() + content

    # Compute SHA-1
    obj_hash = sha1(full_content).hexdigest()

    return obj_hash

# 2. object.c: write_object_file()
def write_object_file(obj_hash, content):
    # Compress with zlib
    compressed = zlib.compress(content)

    # Determine path
    obj_dir = f".git/objects/{obj_hash[:2]}"
    obj_file = f"{obj_dir}/{obj_hash[2:]}"

    # Write atomically
    write_file_atomic(obj_file, compressed)

    return obj_hash

# 3. read-cache.c: add_to_index()
def add_to_index(file_path, obj_hash, mode):
    # Read current index
    index = read_index()

    # Update or add entry
    index.update_entry(
        path=file_path,
        sha1=obj_hash,
        mode=mode,
        mtime=get_mtime(file_path),
        size=get_size(file_path)
    )

    # Write index
    write_index(index)
```

#### Step 3: git commit

```python
# git commit -m "message"

# 1. tree.c: write_tree_from_index()
def write_tree_from_index():
    # Read index
    index = read_index()

    # Group entries by directory
    dirs = group_by_directory(index.entries)

    # Create trees bottom-up
    tree_hashes = {}
    for dir_path in sorted_dirs_by_depth(dirs):
        entries = []

        # Add files
        for entry in dirs[dir_path]:
            if is_file_in_dir(entry, dir_path):
                entries.append(create_tree_entry(entry))

        # Add subdirectories
        for subdir in get_subdirs(dir_path):
            entries.append({
                "mode": "040000",
                "name": basename(subdir),
                "hash": tree_hashes[subdir]
            })

        # Create tree object
        tree_hash = write_tree_object(entries)
        tree_hashes[dir_path] = tree_hash

    return tree_hashes[""]  # Root tree

# 2. commit.c: commit_tree()
def commit_tree(tree_hash, parent_hashes, message, author, committer):
    # Build commit content
    content = f"tree {tree_hash}\n"

    for parent in parent_hashes:
        content += f"parent {parent}\n"

    content += f"author {author.name} <{author.email}> {author.timestamp} {author.timezone}\n"
    content += f"committer {committer.name} <{committer.email}> {committer.timestamp} {committer.timezone}\n"
    content += f"\n{message}\n"

    # Create commit object
    commit_hash = write_object("commit", content)

    return commit_hash

# 3. refs.c: update_ref()
def update_ref(ref_name, new_hash):
    # Read current ref
    old_hash = read_ref(ref_name)

    # Write new ref
    ref_path = f".git/{ref_name}"
    write_file_atomic(ref_path, new_hash + "\n")

    # Update reflog
    update_reflog(ref_name, old_hash, new_hash)
```

### Complete Example: Single File Edit

```bash
# Initial state
$ git log --oneline
abc123 Initial commit

$ git ls-tree abc123
100644 blob 111aaa... file1.txt
100644 blob 222bbb... file2.txt

# Edit file1.txt
$ echo "new content" > file1.txt

# Stage
$ git add file1.txt
# Process:
# 1. Compute hash: 333ccc...
# 2. Create blob: .git/objects/33/3ccc...
# 3. Update index: file1.txt → 333ccc...

# Commit
$ git commit -m "Update file1"
# Process:
# 1. Create tree: 444ddd...
#    - file1.txt → 333ccc... (new)
#    - file2.txt → 222bbb... (reused)
# 2. Create commit: 555eee...
#    - tree: 444ddd...
#    - parent: abc123...
# 3. Update HEAD: 555eee...

# Final state
$ git log --oneline
555eee Update file1
abc123 Initial commit

# All objects still exist:
$ ls .git/objects/
11/1aaa...  # Old blob (from commit abc123)
22/2bbb...  # Unchanged blob
33/3ccc...  # New blob (from commit 555eee)
44/4ddd...  # New tree
55/5eee...  # New commit
```

## Git's Immutability Model

### Immutability Principles

1. **Objects Never Change**: Once created, object content is immutable
2. **Content-Addressable**: Object identity = content hash
3. **Append-Only**: New content creates new objects
4. **History Preservation**: All versions remain accessible

### Why Immutability Matters

**1. Integrity**

- Object hash = content hash
- Any modification = different hash
- Detects corruption automatically

**2. Deduplication**

- Same content = same hash = same object
- Automatic deduplication
- Efficient storage

**3. Time Travel**

- All versions preserved
- Can access any historical state
- Complete audit trail

**4. Concurrent Access**

- Readers never see partial writes
- No locking needed for reads
- Safe parallel operations

### Immutability in Practice

```python
# Example: Multiple edits to same file
# Edit 1: "Hello"
blob1_hash = create_blob("Hello")  # abc123...

# Edit 2: "Hello, World"
blob2_hash = create_blob("Hello, World")  # def456...

# Edit 3: "Hello, World!"
blob3_hash = create_blob("Hello, World!")  # ghi789...

# All three blobs exist in .git/objects/
# Each commit references different blob
# Can access any version via commit history
```

## Influence on Modern Data Systems

### Git's Design Principles

1. **Content-Addressable Storage**: Objects identified by content hash
2. **Immutability**: Objects never modified
3. **Efficient Storage**: Deduplication and compression
4. **Distributed**: Each repository is complete
5. **Fast Operations**: Hash-based lookups

### Systems Inspired by Git

#### 1. DVC (Data Version Control)

- Uses Git-like blob storage for large files
- Content-addressable cache
- `.dvc` files track data versions (like Git tracks code)

#### 2. Delta Lake

- Transaction log similar to Git's commit history
- Immutable Parquet files
- Snapshot-based reads

#### 3. Apache Iceberg

- Snapshot-based table format
- Immutable data files
- Manifest lists (similar to Git trees)

#### 4. TileDB

- Fragment-based storage (like Git commits)
- Immutable fragments
- Time-travel queries

#### 5. Zarr

- Chunk-based storage
- Content-addressable chunks
- Immutable chunk storage

### Common Patterns

**1. Content-Addressable Storage**

```python
# Git pattern
object_hash = sha1(content)
store_at(object_hash, content)

# Modern systems
blob_id = hash(content)
store_blob(blob_id, content)
```

**2. Immutable Snapshots**

```python
# Git: Commit = snapshot
commit = {
    "tree": tree_hash,
    "parent": parent_hash,
    "timestamp": now()
}

# Modern: Snapshot = state
snapshot = {
    "manifest": manifest_hash,
    "parent": parent_snapshot,
    "timestamp": now()
}
```

**3. Append-Only Logs**

```python
# Git: Commit history
commits = [commit1, commit2, commit3, ...]

# Modern: Transaction log
transactions = [tx1, tx2, tx3, ...]
```

**4. Delta Compression**

```python
# Git: Packfile deltas
delta = create_delta(new_object, base_object)

# Modern: Similar delta strategies
delta = compute_diff(new_data, base_data)
```

## Storage Optimization Strategies

### 1. Object Deduplication

```python
# Git automatically deduplicates
def store_object(content):
    obj_hash = sha1(content)

    # Check if already exists
    if object_exists(obj_hash):
        return obj_hash  # Reuse existing

    # Store new object
    write_object(obj_hash, content)
    return obj_hash

# Example: Same file in different commits
commit1_tree = {
    "file.txt": blob_hash_abc  # "Hello"
}

commit2_tree = {
    "file.txt": blob_hash_abc  # Same content, same blob
}

# Only one blob stored, both trees reference it
```

### 2. Tree Reuse

```python
# Unchanged subtrees are reused
def create_tree(entries):
    # If tree with same entries exists, reuse it
    tree_hash = compute_tree_hash(entries)

    if tree_exists(tree_hash):
        return tree_hash  # Reuse

    # Create new tree
    return write_tree(entries)

# Example: Only one file changed
# Old tree: {file1: hash1, file2: hash2, subdir: tree_hash}
# New tree: {file1: hash3, file2: hash2, subdir: tree_hash}
#          file2 and subdir unchanged, same hashes reused
```

### 3. Packfile Optimization

```python
# Packfiles optimize storage
def repack_objects():
    # 1. Collect objects
    objects = collect_all_objects()

    # 2. Sort for delta compression
    sorted_objects = sort_for_deltas(objects)

    # 3. Create deltas
    packed = []
    for obj in sorted_objects:
        base = find_best_delta_base(obj)
        if base:
            delta = create_delta(obj, base)
            if len(delta) < len(obj.content):
                packed.append(delta)
            else:
                packed.append(obj.content)
        else:
            packed.append(obj.content)

    # 4. Write packfile
    write_packfile(packed)
```

## Best Practices

### 1. Regular Repacking

```bash
# Automatic repacking (Git does this)
git gc --auto

# Manual repacking
git repack -a -d
```

### 2. Efficient Commits

- **Batch changes**: Commit related changes together
- **Logical commits**: One logical change per commit
- **Avoid large files**: Use Git LFS for large binaries

### 3. Understanding Object Model

- **Know what creates objects**: Each edit creates new objects
- **Understand storage**: Loose objects vs packfiles
- **Monitor size**: Use `git count-objects -v`

## Summary

Git's architecture is built on immutable, content-addressable objects:

1. **Blobs**: Store file content, identified by SHA-1 hash
2. **Trees**: Represent directory structures, reference blobs and other trees
3. **Commits**: Record snapshots with metadata, form history graph
4. **Immutability**: Objects never modified, new content = new objects
5. **Packfiles**: Optimize storage with delta compression
6. **Deduplication**: Same content stored only once
7. **Efficient Updates**: Only changed files create new blobs

This architecture provides:

- **Integrity**: Hash-based verification
- **Efficiency**: Deduplication and compression
- **History**: Complete version history
- **Concurrency**: Safe parallel operations
- **Distribution**: Each repo is complete

Git's design has inspired modern data systems like DVC, Delta Lake, Iceberg, and TileDB, which apply similar principles of immutability, content-addressable storage, and efficient versioning to large-scale data management.
