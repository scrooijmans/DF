# Pijul Push/Pull Synchronization Architecture Analysis

## Executive Summary

This document analyzes Pijul's push/pull synchronization model, focusing on how it represents state, handles patches, manages conflicts, and enables efficient synchronization. The analysis covers protocol flow, internal state transitions, and architectural lessons applicable to REST-based sync APIs for Parquet datasets.

---

## 1. Client and Server State Representation

### Patch-Based State Model

Pijul represents repository state as a **collection of patches** rather than snapshots or sequential commits. This fundamental difference enables more flexible synchronization.

#### Client State Structure

```rust
// Conceptual representation of Pijul's state model

/// A patch represents a single atomic change
struct Patch {
    /// Unique identifier for this patch
    id: PatchHash,
    
    /// Dependencies on other patches (graph edges)
    dependencies: Vec<PatchHash>,
    
    /// The actual change (file modifications, additions, deletions)
    change: Change,
    
    /// Metadata (author, timestamp, description)
    metadata: PatchMetadata,
}

/// Repository state is a graph of patches
struct Repository {
    /// All patches in the repository
    patches: HashMap<PatchHash, Patch>,
    
    /// Current channel state (which patches are applied)
    channel: Channel,
    
    /// File tree state (derived from applied patches)
    file_tree: FileTree,
}

/// Channel represents a view of the repository
struct Channel {
    /// Set of patch hashes that are applied in this channel
    applied_patches: HashSet<PatchHash>,
    
    /// Name of the channel (e.g., "main", "feature-branch")
    name: String,
}
```

#### Server State Structure

The server maintains similar state but with additional metadata:

```rust
/// Server-side repository state
struct ServerRepository {
    /// All patches known to the server
    patches: HashMap<PatchHash, Patch>,
    
    /// Multiple channels (branches)
    channels: HashMap<String, Channel>,
    
    /// Access control and permissions
    permissions: Permissions,
    
    /// Patch metadata (who pushed, when, etc.)
    patch_metadata: HashMap<PatchHash, ServerPatchMetadata>,
}
```

### State Representation Principles

**Key Design Principles:**

1. **Patches as First-Class Entities**: Patches are independent, atomic units of change
2. **Graph-Based Dependencies**: Patches form a directed acyclic graph (DAG) of dependencies
3. **Channel-Based Views**: Different views (branches) are represented as different sets of applied patches
4. **Derived State**: File tree and content are derived from applying patches, not stored directly

### State Comparison

**Traditional VCS (Git-like):**
```
State = Snapshot of files at commit
History = Linear sequence of commits
Sync = Exchange commits
```

**Pijul:**
```
State = Set of applied patches
History = Graph of patches with dependencies
Sync = Exchange patches
```

---

## 2. Push and Pull Operations Structure

### Push Operation

#### Push Request Structure

```rust
/// Push request from client to server
struct PushRequest {
    /// Patches the client wants to push
    patches: Vec<Patch>,
    
    /// Channel name (e.g., "main")
    channel: String,
    
    /// Client's current state (set of patch hashes)
    client_state: HashSet<PatchHash>,
}
```

#### Push Execution Flow

```
1. Client prepares push request
   │
   │  1.1. Identify patches not on server
   │       - Compare client_state with server_state
   │       - Find patches in client but not in server
   │
   │  1.2. Check dependencies
   │       - Ensure all patch dependencies are either:
   │         a) Already on server, OR
   │         b) Included in push request
   │
   │  1.3. Validate patches
   │       - Verify patch integrity
   │       - Check patch signatures (if signed)
   │
   ▼
2. Client sends push request
   │
   │  POST /api/push
   │  {
   │    "channel": "main",
   │    "patches": [...],
   │    "client_state": [...]
   │  }
   │
   ▼
3. Server processes push
   │
   │  3.1. Validate patches
   │       - Check patch format
   │       - Verify dependencies exist
   │       - Validate permissions
   │
   │  3.2. Check for conflicts
   │       - Compare with server's current state
   │       - Detect conflicting patches
   │
   │  3.3. Apply patches (if no conflicts)
   │       - Add patches to server's patch store
   │       - Update channel state
   │       - Recompute file tree
   │
   │  3.4. Generate response
   │       - Accepted patches
   │       - Rejected patches (with reasons)
   │       - Conflicts (if any)
   │
   ▼
4. Server sends push response
   │
   │  {
   │    "accepted": [...],
   │    "rejected": [...],
   │    "conflicts": [...],
   │    "server_state": [...]
   │  }
   │
   ▼
5. Client updates local state
   │
   │  5.1. Mark accepted patches as synced
   │  5.2. Handle rejected patches
   │  5.3. Update client_state with server_state
   │  5.4. Resolve conflicts (if any)
```

### Pull Operation

#### Pull Request Structure

```rust
/// Pull request from client to server
struct PullRequest {
    /// Channel to pull from
    channel: String,
    
    /// Client's current state (set of patch hashes)
    client_state: HashSet<PatchHash>,
    
    /// Optional: specific patches to pull
    requested_patches: Option<Vec<PatchHash>>,
}
```

#### Pull Execution Flow

```
1. Client prepares pull request
   │
   │  1.1. Get current client state
   │       - Read channel state
   │       - Collect all applied patch hashes
   │
   │  1.2. Determine what to request
   │       - If client_state is empty: pull all patches
   │       - Otherwise: pull patches in server but not client
   │
   ▼
2. Client sends pull request
   │
   │  POST /api/pull
   │  {
   │    "channel": "main",
   │    "client_state": [...]
   │  }
   │
   ▼
3. Server processes pull
   │
   │  3.1. Compare states
   │       - Find patches in server but not in client
   │       - Respect dependency order
   │
   │  3.2. Select patches to send
   │       - Include all missing patches
   │       - Include all dependencies (even if client has them)
   │       - Order by dependency graph
   │
   │  3.3. Generate response
   │       - List of patches
   │       - Server's current state
   │       - Dependency information
   │
   ▼
4. Server sends pull response
   │
   │  {
   │    "patches": [...],
   │    "server_state": [...],
   │    "dependencies": {...}
   │  }
   │
   ▼
5. Client applies patches
   │
   │  5.1. Validate patches
   │       - Check patch integrity
   │       - Verify dependencies
   │
   │  5.2. Apply patches in dependency order
   │       - Topological sort of patch graph
   │       - Apply each patch sequentially
   │       - Update file tree after each patch
   │
   │  5.3. Update channel state
   │       - Mark patches as applied
   │       - Update client_state
   │
   │  5.4. Handle conflicts (if any)
   │       - Detect conflicts during application
   │       - Resolve or queue for resolution
```

---

## 3. Content Identity and Versioning

### Patch Identity

**Patch Hash**: Each patch is identified by a cryptographic hash of its content:

```rust
/// Patch hash (content-addressed identifier)
type PatchHash = [u8; 32];  // SHA-256 hash

impl Patch {
    fn hash(&self) -> PatchHash {
        // Hash includes:
        // - Change content
        // - Dependencies
        // - Metadata (author, timestamp)
        sha256_hash(self)
    }
}
```

**Properties:**
- **Content-Addressed**: Same patch content → same hash
- **Deterministic**: Hash is computed deterministically
- **Collision-Resistant**: Cryptographic hash prevents collisions

### Versioning Model

Pijul uses a **graph-based versioning model** rather than linear version numbers:

```rust
/// Version is represented as a set of patch hashes
type Version = HashSet<PatchHash>;

/// Two repositories are at the same version if they have the same set of patches
fn versions_equal(v1: &Version, v2: &Version) -> bool {
    v1 == v2
}

/// Version comparison (partial ordering)
fn version_contains(v1: &Version, v2: &Version) -> bool {
    // v1 contains v2 if v2 is a subset of v1
    v2.is_subset(v1)
}
```

**Key Properties:**
- **No Linear Version Numbers**: Versions are sets, not numbers
- **Partial Ordering**: Some versions are comparable (subset/superset), others are not
- **Merge-Friendly**: Merging is just set union

### Dependency Graph

```rust
/// Patch dependency graph
struct PatchGraph {
    /// Map from patch to its dependencies
    dependencies: HashMap<PatchHash, Vec<PatchHash>>,
    
    /// Map from patch to patches that depend on it
    dependents: HashMap<PatchHash, Vec<PatchHash>>,
}

impl PatchGraph {
    /// Topological sort of patches (dependency order)
    fn topological_sort(&self, patches: &[PatchHash]) -> Vec<PatchHash> {
        // Kahn's algorithm or DFS-based topological sort
        // Ensures dependencies are applied before dependents
    }
    
    /// Check if patch can be applied (all dependencies present)
    fn can_apply(&self, patch: &PatchHash, state: &Version) -> bool {
        let deps = self.dependencies.get(patch).unwrap_or(&vec![]);
        deps.iter().all(|dep| state.contains(dep))
    }
}
```

---

## 4. Execution Flow for a Push Request

### Detailed Push Flow

```
┌─────────────────────────────────────────────────────────────────┐
│                    CLIENT SIDE                                  │
└─────────────────────────────────────────────────────────────────┘

Step 1: Identify Patches to Push
─────────────────────────────────
│
│  client_state = get_current_channel_state()
│  server_state = get_last_known_server_state()  // From last sync
│
│  patches_to_push = client_state - server_state
│
│  // Validate all dependencies are available
│  for patch in patches_to_push:
│      deps = patch.dependencies
│      missing_deps = deps - (client_state ∪ server_state)
│      if missing_deps:
│          error("Missing dependencies: {missing_deps}")
│
▼

Step 2: Build Push Request
───────────────────────────
│
│  request = PushRequest {
│      channel: "main",
│      patches: patches_to_push,
│      client_state: client_state,
│  }
│
▼

Step 3: Send HTTP Request
──────────────────────────
│
│  POST /api/push
│  Authorization: Bearer <token>
│  Content-Type: application/json
│
│  Body: {
│      "channel": "main",
│      "patches": [
│          {
│              "hash": "abc123...",
│              "dependencies": ["def456..."],
│              "change": {...},
│              "metadata": {...}
│          },
│          ...
│      ],
│      "client_state": ["abc123...", "def456...", ...]
│  }
│
▼

┌─────────────────────────────────────────────────────────────────┐
│                    SERVER SIDE                                  │
└─────────────────────────────────────────────────────────────────┘

Step 4: Receive and Validate Request
─────────────────────────────────────
│
│  // Authentication
│  user = authenticate(request.headers["Authorization"])
│  if !user.has_permission(channel, "push"):
│      return 403 Forbidden
│
│  // Parse request
│  push_request = parse_json(request.body)
│
▼

Step 5: Validate Patches
─────────────────────────
│
│  server_state = get_channel_state(push_request.channel)
│  conflicts = []
│  accepted = []
│
│  for patch in push_request.patches:
│      // Check if patch already exists
│      if server_state.contains(patch.hash):
│          accepted.append(patch.hash)  // Already have it
│          continue
│
│      // Validate dependencies
│      deps = patch.dependencies
│      missing_deps = deps - server_state
│      if missing_deps:
│          conflicts.append({
│              "patch": patch.hash,
│              "reason": "missing_dependencies",
│              "missing": missing_deps
│          })
│          continue
│
│      // Check for conflicts with existing patches
│      if has_conflict(patch, server_state):
│          conflicts.append({
│              "patch": patch.hash,
│              "reason": "conflict",
│              "conflicting_patches": find_conflicting_patches(patch, server_state)
│          })
│          continue
│
│      // Patch is valid
│      accepted.append(patch.hash)
│
▼

Step 6: Apply Accepted Patches
───────────────────────────────
│
│  // Transaction: apply all accepted patches atomically
│  begin_transaction()
│
│  try:
│      for patch_hash in accepted:
│          patch = find_patch(patch_hash)
│          
│          // Store patch
│          store_patch(patch)
│          
│          // Update channel state
│          add_patch_to_channel(push_request.channel, patch_hash)
│          
│          // Update file tree
│          apply_patch_to_file_tree(patch)
│      
│      // Update server state
│      new_server_state = server_state ∪ accepted
│      update_channel_state(push_request.channel, new_server_state)
│      
│      commit_transaction()
│  except:
│      rollback_transaction()
│      return 500 Internal Server Error
│
▼

Step 7: Generate Response
──────────────────────────
│
│  response = PushResponse {
│      accepted: accepted,
│      rejected: conflicts,
│      server_state: new_server_state,
│      conflicts: conflicts,  // Detailed conflict information
│  }
│
│  return 200 OK, response
│
▼

┌─────────────────────────────────────────────────────────────────┐
│                    CLIENT SIDE                                  │
└─────────────────────────────────────────────────────────────────┘

Step 8: Process Response
─────────────────────────
│
│  response = await http_response
│
│  // Update local state
│  for patch_hash in response.accepted:
│      mark_patch_as_synced(patch_hash)
│
│  // Update known server state
│  update_last_known_server_state(response.server_state)
│
│  // Handle conflicts
│  if response.conflicts:
│      for conflict in response.conflicts:
│          queue_conflict_for_resolution(conflict)
│
│  // Handle rejected patches
│  for rejected in response.rejected:
│      log_rejection(rejected)
│      // May need to pull and retry
│
▼

Step 9: Update UI/State
────────────────────────
│
│  notify_ui("push_complete", {
│      "accepted": response.accepted.len(),
│      "conflicts": response.conflicts.len()
│  })
│
│  if response.conflicts:
│      show_conflict_resolution_ui()
```

### State Transitions

**Client State Transitions:**

```
Initial State:
  client_state = {patch1, patch2, patch3}
  server_state_known = {patch1, patch2}

After Creating New Patch:
  client_state = {patch1, patch2, patch3, patch4}
  server_state_known = {patch1, patch2}  // Unchanged

After Push (Success):
  client_state = {patch1, patch2, patch3, patch4}  // Unchanged
  server_state_known = {patch1, patch2, patch3, patch4}  // Updated

After Push (Conflict):
  client_state = {patch1, patch2, patch3, patch4}
  server_state_known = {patch1, patch2, patch5}  // Server has different patches
  conflicts = [conflict_info]
```

**Server State Transitions:**

```
Initial State:
  server_state = {patch1, patch2}

After Receiving Push:
  new_patches = {patch3, patch4}
  server_state = {patch1, patch2, patch3, patch4}

After Conflict Detection:
  server_state = {patch1, patch2}  // Unchanged
  conflicts = [conflict_info]  // Patches rejected
```

---

## 5. Conflict and Partial Update Management

### Conflict Detection

Pijul detects conflicts at the **patch level** rather than file level:

```rust
/// Conflict detection algorithm
fn has_conflict(
    new_patch: &Patch,
    existing_patches: &HashSet<PatchHash>,
) -> bool {
    // Two patches conflict if:
    // 1. They modify the same lines in the same file, AND
    // 2. They are not in a dependency relationship
    
    for existing_hash in existing_patches {
        let existing_patch = get_patch(existing_hash);
        
        // Check if patches touch overlapping regions
        if patches_overlap(new_patch, existing_patch) {
            // Check dependency relationship
            if !are_dependent(new_patch, existing_patch) {
                return true;  // Conflict!
            }
        }
    }
    
    false
}

/// Check if two patches overlap (modify same lines)
fn patches_overlap(p1: &Patch, p2: &Patch) -> bool {
    // Compare file paths and line ranges
    for (file, lines1) in p1.changes {
        if let Some(lines2) = p2.changes.get(file) {
            if lines_intersect(lines1, lines2) {
                return true;
            }
        }
    }
    false
}
```

### Conflict Representation

```rust
/// Conflict information
struct Conflict {
    /// The patch that conflicted
    conflicting_patch: PatchHash,
    
    /// Patches it conflicts with
    conflicts_with: Vec<PatchHash>,
    
    /// File and line information
    location: ConflictLocation,
    
    /// Conflict type
    conflict_type: ConflictType,
}

enum ConflictType {
    /// Two patches modify the same lines
    OverlappingChanges,
    
    /// One patch deletes what another modifies
    DeleteModify,
    
    /// Dependencies cannot be satisfied
    MissingDependencies,
}
```

### Conflict Resolution Strategies

**1. Automatic Resolution (Commutative Patches)**

If patches are **commutative** (order-independent), they can be automatically merged:

```rust
/// Check if patches can be automatically merged
fn can_auto_merge(p1: &Patch, p2: &Patch) -> bool {
    // Patches are commutative if they:
    // - Modify different files, OR
    // - Modify non-overlapping lines in the same file
    
    !patches_overlap(p1, p2)
}

/// Automatically merge commutative patches
fn auto_merge(p1: &Patch, p2: &Patch) -> Vec<Patch> {
    // Return both patches (they can be applied in any order)
    vec![p1.clone(), p2.clone()]
}
```

**2. Manual Resolution**

For non-commutative patches, manual resolution is required:

```rust
/// Manual conflict resolution
struct ConflictResolution {
    /// Which version to keep
    resolution: ResolutionChoice,
    
    /// Optional: merged patch (if user creates one)
    merged_patch: Option<Patch>,
}

enum ResolutionChoice {
    /// Keep local patch, discard remote
    KeepLocal,
    
    /// Keep remote patch, discard local
    KeepRemote,
    
    /// Use manually created merged patch
    UseMerged(Patch),
}
```

### Partial Update Handling

Pijul handles partial updates through **patch dependencies**:

```rust
/// Partial update scenario
// Client has: {patch1, patch2}
// Server has: {patch1, patch2, patch3, patch4}
// patch4 depends on patch3

// Pull request includes both patch3 and patch4
// Client applies patch3 first (dependency), then patch4

fn apply_partial_update(
    patches: Vec<Patch>,
    current_state: &Version,
) -> Result<Version> {
    // Topological sort ensures dependencies are applied first
    let sorted = topological_sort(patches);
    
    let mut new_state = current_state.clone();
    
    for patch in sorted {
        // Verify all dependencies are present
        if !dependencies_satisfied(&patch, &new_state) {
            return Err("Missing dependencies");
        }
        
        // Apply patch
        apply_patch(&patch)?;
        new_state.insert(patch.hash());
    }
    
    Ok(new_state)
}
```

---

## 6. Adapting to Parquet Datasets and Metadata over HTTP

### Mapping Pijul Concepts to Parquet Sync

| Pijul Concept | Parquet Sync Equivalent |
|---------------|------------------------|
| **Patch** | **Change Record** (create/update/delete of dataset or metadata) |
| **Patch Hash** | **Change ID** (UUID or content hash) |
| **Patch Dependencies** | **Change Dependencies** (e.g., metadata depends on dataset) |
| **Channel** | **Workspace/Branch** |
| **File Tree** | **Dataset Registry** (list of available datasets) |
| **Patch Application** | **Change Application** (apply to local database) |

### REST API Design for Parquet Sync

#### Push Endpoint

```rust
/// Push changes to server
POST /api/sync/push

Request:
{
    "workspace_id": "uuid",
    "from_version": 123,  // Client's last known server version
    "changes": [
        {
            "id": "change-uuid",
            "entity_type": "dataset" | "metadata",
            "entity_id": "dataset-uuid",
            "action": "create" | "update" | "delete",
            "version": 5,
            "timestamp": "2024-01-01T10:00:00Z",
            "dependencies": ["change-uuid-1", "change-uuid-2"],  // Pijul-inspired
            "data": {...},  // Entity data (for create/update)
            "blob_hash": "sha256-hash"  // For dataset changes
        }
    ],
    "pending_blobs": ["hash1", "hash2"]  // Blobs that need uploading
}

Response:
{
    "server_version": 125,
    "accepted": ["change-uuid-1", "change-uuid-2"],
    "rejected": [
        {
            "change_id": "change-uuid-3",
            "reason": "conflict",
            "conflicting_changes": ["server-change-uuid"],
            "server_version": 124,
            "server_data": {...}
        }
    ],
    "blob_upload_urls": [
        {
            "hash": "hash1",
            "url": "https://s3.../presigned-upload-url",
            "expires_at": "2024-01-01T11:00:00Z"
        }
    ]
}
```

#### Pull Endpoint

```rust
/// Pull changes from server
POST /api/sync/pull

Request:
{
    "workspace_id": "uuid",
    "from_version": 120,  // Client's current version
    "limit": 100  // Optional pagination
}

Response:
{
    "server_version": 125,
    "changes": [
        {
            "id": "change-uuid",
            "entity_type": "dataset",
            "entity_id": "dataset-uuid",
            "action": "create",
            "version": 6,
            "timestamp": "2024-01-01T11:00:00Z",
            "dependencies": ["change-uuid-1"],
            "data": {...},
            "blob_hash": "sha256-hash"
        }
    ],
    "required_blobs": ["hash1", "hash2"],  // Blobs client needs to download
    "has_more": false  // Pagination indicator
}
```

### State Representation for Parquet Sync

```rust
/// Client state (similar to Pijul's channel state)
struct ClientSyncState {
    /// Workspace ID
    workspace_id: Uuid,
    
    /// Last known server version
    last_sync_version: i64,
    
    /// Set of change IDs that have been applied locally
    applied_changes: HashSet<Uuid>,
    
    /// Local entity versions (for conflict detection)
    entity_versions: HashMap<(EntityType, Uuid), i64>,
}

/// Server state
struct ServerSyncState {
    /// Workspace ID
    workspace_id: Uuid,
    
    /// Current server version (monotonically increasing)
    current_version: i64,
    
    /// All changes in the workspace
    changes: HashMap<Uuid, Change>,
    
    /// Change dependencies graph
    dependency_graph: ChangeGraph,
    
    /// Entity versions (for conflict detection)
    entity_versions: HashMap<(EntityType, Uuid), i64>,
}
```

### Change Dependencies for Parquet

```rust
/// Example: Metadata change depends on dataset change
let dataset_change = Change {
    id: uuid1,
    entity_type: EntityType::Dataset,
    entity_id: dataset_id,
    action: SyncAction::Create,
    dependencies: vec![],  // No dependencies
    blob_hash: Some("parquet-file-hash"),
};

let metadata_change = Change {
    id: uuid2,
    entity_type: EntityType::Metadata,
    entity_id: metadata_id,
    action: SyncAction::Create,
    dependencies: vec![uuid1],  // Depends on dataset creation
    data: Some(metadata_json),
};
```

### Conflict Detection for Parquet

```rust
/// Conflict detection (adapted from Pijul)
fn detect_conflict(
    new_change: &Change,
    existing_changes: &[Change],
    entity_versions: &HashMap<(EntityType, Uuid), i64>,
) -> Option<Conflict> {
    // Check version conflicts
    let key = (new_change.entity_type, new_change.entity_id);
    if let Some(&existing_version) = entity_versions.get(&key) {
        if new_change.version <= existing_version {
            return Some(Conflict {
                change_id: new_change.id,
                reason: ConflictReason::VersionConflict {
                    client_version: new_change.version,
                    server_version: existing_version,
                },
            });
        }
    }
    
    // Check dependency conflicts
    for dep in &new_change.dependencies {
        if !existing_changes.iter().any(|c| c.id == *dep) {
            return Some(Conflict {
                change_id: new_change.id,
                reason: ConflictReason::MissingDependency {
                    missing: *dep,
                },
            });
        }
    }
    
    None
}
```

---

## 7. Architectural Lessons for REST-Based Sync API

### Lesson 1: Change-Based Synchronization

**Pijul's Approach:**
- Synchronize changes (patches), not entire state
- Changes are atomic and independent
- Changes can be applied in dependency order

**REST API Application:**
```rust
// ✅ GOOD: Sync changes, not full state
POST /api/sync/push
{
    "changes": [
        {"id": "uuid1", "action": "create", "data": {...}},
        {"id": "uuid2", "action": "update", "data": {...}}
    ]
}

// ❌ BAD: Syncing full state
POST /api/sync/push
{
    "full_state": {
        "datasets": [...],  // Entire dataset list
        "metadata": [...]   // Entire metadata
    }
}
```

**Benefits:**
- **Efficiency**: Only transfer what changed
- **Incremental**: Can sync large datasets incrementally
- **Resumable**: Can retry failed changes individually

### Lesson 2: Dependency Management

**Pijul's Approach:**
- Patches declare dependencies on other patches
- Dependencies form a DAG
- Patches applied in topological order

**REST API Application:**
```rust
// Changes declare dependencies
{
    "id": "change-2",
    "dependencies": ["change-1"],  // Must apply change-1 first
    "action": "create",
    "data": {...}
}

// Server validates dependencies
fn validate_push(request: &PushRequest) -> Result<()> {
    for change in &request.changes {
        for dep in &change.dependencies {
            // Check dependency exists in server or request
            if !server_has_change(dep) && !request_has_change(dep) {
                return Err("Missing dependency");
            }
        }
    }
    Ok(())
}
```

**Benefits:**
- **Order Independence**: Changes can be sent in any order
- **Partial Updates**: Can apply subset of changes safely
- **Conflict Prevention**: Dependencies prevent invalid states

### Lesson 3: Conflict-Free Replicated Data Types (CRDT)

**Pijul's Approach:**
- Patches are commutative when they don't overlap
- Conflicts are explicit and manageable
- State converges automatically for commutative patches

**REST API Application:**
```rust
// Commutative changes (no conflict)
let change1 = Change {
    entity_id: dataset1_id,
    action: SyncAction::Update,
    field: "name",
    value: "New Name",
};

let change2 = Change {
    entity_id: dataset2_id,  // Different entity
    action: SyncAction::Update,
    field: "description",
    value: "New Description",
};
// These can be applied in any order

// Non-commutative changes (conflict)
let change1 = Change {
    entity_id: dataset1_id,
    action: SyncAction::Update,
    field: "name",
    value: "Name A",
};

let change2 = Change {
    entity_id: dataset1_id,  // Same entity, same field
    action: SyncAction::Update,
    field: "name",
    value: "Name B",
};
// These conflict and need resolution
```

**Benefits:**
- **Automatic Merging**: Commutative changes merge automatically
- **Explicit Conflicts**: Non-commutative changes are clearly identified
- **User Control**: Users resolve conflicts when needed

### Lesson 4: State Comparison

**Pijul's Approach:**
- Compare states as sets of patch hashes
- Efficient set difference operations
- No need for linear version numbers

**REST API Application:**
```rust
// Client sends its state
POST /api/sync/pull
{
    "from_version": 120,  // Linear version (simple)
    "applied_changes": ["uuid1", "uuid2", "uuid3"]  // Set-based (Pijul-style)
}

// Server computes difference
fn compute_changes_to_send(
    client_state: &HashSet<Uuid>,
    server_state: &HashSet<Uuid>,
) -> Vec<Change> {
    let missing = server_state - client_state;
    let changes = missing.iter()
        .map(|id| get_change(*id))
        .collect();
    
    // Topological sort by dependencies
    topological_sort(changes)
}
```

**Benefits:**
- **Efficient**: Set operations are fast
- **Accurate**: No version number drift
- **Flexible**: Works with branching/merging

### Lesson 5: Idempotent Operations

**Pijul's Approach:**
- Applying the same patch twice has no effect
- Patches are content-addressed (hash-based)

**REST API Application:**
```rust
// Idempotent push
POST /api/sync/push
{
    "changes": [
        {"id": "uuid1", ...},  // If server already has this, ignore it
        {"id": "uuid2", ...}
    ]
}

// Server handles idempotency
fn apply_change(change: &Change) -> Result<()> {
    if server_has_change(change.id) {
        return Ok(());  // Already applied, no-op
    }
    
    // Apply change
    apply_change_internal(change)?;
    mark_change_applied(change.id);
    Ok(())
}
```

**Benefits:**
- **Retry-Safe**: Can retry failed requests safely
- **Network Resilience**: Handles duplicate requests
- **Simplified Logic**: No need to track "already sent" state

### Lesson 6: Separation of Metadata and Bulk Data

**Pijul's Approach:**
- Patches contain change metadata
- Large files are stored separately
- Patches reference files by hash

**REST API Application:**
```rust
// Change contains metadata only
{
    "id": "uuid1",
    "entity_type": "dataset",
    "action": "create",
    "blob_hash": "sha256-hash",  // Reference to Parquet file
    "metadata": {
        "name": "Well Logs",
        "size": 1048576,
        "row_count": 10000
    }
}

// Blob uploaded separately
PUT /api/blobs/{hash}
Content-Type: application/parquet
<parquet file data>
```

**Benefits:**
- **Efficiency**: Metadata is small, blobs are large
- **Deduplication**: Same blob hash = same content
- **Caching**: Blobs can be cached independently

---

## 8. Complete Example: Parquet Dataset Sync

### Scenario: Syncing a New Dataset

```
1. Client creates new dataset locally
   │
   │  Local Database:
   │  - INSERT INTO datasets (id, name, blob_hash, version)
   │    VALUES (uuid1, "Well Logs", "hash123", 1)
   │
   │  Sync Queue:
   │  - INSERT INTO sync_queue (entity_type, entity_id, action, version)
   │    VALUES ("dataset", uuid1, "create", 1)
   │
   ▼

2. Client pushes change to server
   │
   │  POST /api/sync/push
   │  {
   │    "workspace_id": "ws-123",
   │    "from_version": 100,
   │    "changes": [
   │      {
   │        "id": "change-uuid-1",
   │        "entity_type": "dataset",
   │        "entity_id": "uuid1",
   │        "action": "create",
   │        "version": 1,
   │        "dependencies": [],
   │        "data": {
   │          "name": "Well Logs",
   │          "blob_hash": "hash123"
   │        },
   │        "blob_hash": "hash123"
   │      }
   │    ],
   │    "pending_blobs": ["hash123"]
   │  }
   │
   ▼

3. Server processes push
   │
   │  3.1. Validate change
   │       - Check workspace permissions
   │       - Verify change format
   │       - Check dependencies (none in this case)
   │
   │  3.2. Check for blob
   │       - Server checks if blob "hash123" exists
   │       - If not, generates presigned upload URL
   │
   │  3.3. Accept change
   │       - Store change in database
   │       - Update entity version
   │       - Increment server version to 101
   │
   │  3.4. Generate response
   │       {
   │         "server_version": 101,
   │         "accepted": ["change-uuid-1"],
   │         "rejected": [],
   │         "blob_upload_urls": [
   │           {
   │             "hash": "hash123",
   │             "url": "https://s3.../presigned-url",
   │             "expires_at": "..."
   │           }
   │         ]
   │       }
   │
   ▼

4. Client uploads blob
   │
   │  PUT <presigned-url>
   │  Content-Type: application/parquet
   │  <parquet file data>
   │
   ▼

5. Client updates local state
   │
   │  - Mark change as synced
   │  - Update last_sync_version to 101
   │  - Remove from sync_queue
   │
   ▼

6. Another client pulls changes
   │
   │  POST /api/sync/pull
   │  {
   │    "workspace_id": "ws-123",
   │    "from_version": 100
   │  }
   │
   │  Response:
   │  {
   │    "server_version": 101,
   │    "changes": [
   │      {
   │        "id": "change-uuid-1",
   │        "entity_type": "dataset",
   │        "entity_id": "uuid1",
   │        "action": "create",
   │        "version": 1,
   │        "data": {...},
   │        "blob_hash": "hash123"
   │      }
   │    ],
   │    "required_blobs": ["hash123"]
   │  }
   │
   ▼

7. Client downloads blob and applies change
   │
   │  7.1. Download blob
   │       GET /api/blobs/urls
   │       Response: {presigned download URL}
   │       Download blob, verify hash
   │
   │  7.2. Apply change
   │       - INSERT INTO datasets (...)
   │       - Store blob locally
   │       - Update sync_state
   │
   ▼

8. Both clients now synchronized
   │
   │  Client 1: last_sync_version = 101
   │  Client 2: last_sync_version = 101
   │  Both have dataset uuid1 with blob hash123
```

---

## 9. Key Takeaways and Recommendations

### Core Principles from Pijul

1. **Change-Based Sync**: Sync changes, not full state
2. **Dependency Management**: Explicit dependencies enable safe partial updates
3. **Content Addressing**: Use hashes for identity and deduplication
4. **Set-Based Versioning**: Versions as sets enable flexible merging
5. **Idempotency**: Operations should be safe to retry
6. **Separation of Concerns**: Separate metadata from bulk data

### REST API Design Recommendations

1. **Use Change IDs**: Each change has a unique ID (UUID)
2. **Support Dependencies**: Changes can declare dependencies
3. **Version Tracking**: Track both linear versions and change sets
4. **Conflict Detection**: Detect conflicts at change level
5. **Blob Separation**: Handle large Parquet files separately
6. **Idempotent Endpoints**: All sync endpoints should be idempotent

### Implementation Checklist

- [ ] Change-based sync protocol (not full state)
- [ ] Dependency graph support
- [ ] Content-addressed blob storage
- [ ] Conflict detection and resolution
- [ ] Idempotent push/pull operations
- [ ] Efficient state comparison (set operations)
- [ ] Separation of metadata and bulk data
- [ ] Support for partial updates
- [ ] Retry-safe operations
- [ ] Pagination for large change sets

---

## References

- Pijul Manual: https://pijul.org/manual/
- Pijul Theory: https://pijul.org/manual/theory.html
- CRDTs: Conflict-Free Replicated Data Types
- Git Internals: For comparison with traditional VCS

