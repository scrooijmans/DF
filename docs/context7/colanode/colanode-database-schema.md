# ColaNode Database Schema

This document details ColaNode's database schema, including the node-based data model, relationships between workspaces, spaces, discussions, and users, as well as collaboration tracking mechanisms.

## Overview

ColaNode employs a **node-based data model** to manage all entities (workspaces, spaces, discussions, users). This model provides flexibility and efficient synchronization across devices. The architecture uses:

- **Local Storage**: SQLite database on client devices
- **Server Storage**: PostgreSQL database on the server
- **Binary Assets**: S3-compatible object storage

## Node-Based Data Model

### Core Node Structure

All entities in ColaNode are represented as nodes with a consistent structure:

```sql
-- Core nodes table (SQLite local / PostgreSQL server)
CREATE TABLE nodes (
  id TEXT PRIMARY KEY,              -- ULID (Universally Unique Lexicographically Sortable Identifier)
  type TEXT NOT NULL,               -- Node type: 'workspace', 'space', 'discussion', 'user', 'page', etc.
  parent_id TEXT,                   -- Parent node ID (NULL for root nodes like workspaces)
  attributes JSONB NOT NULL,        -- Type-specific properties stored as JSON
  created_at TIMESTAMPTZ DEFAULT NOW(),
  updated_at TIMESTAMPTZ DEFAULT NOW(),
  deleted_at TIMESTAMPTZ,           -- Soft delete support

  -- Indexes
  INDEX idx_nodes_type ON nodes(type),
  INDEX idx_nodes_parent_id ON nodes(parent_id),
  INDEX idx_nodes_created_at ON nodes(created_at DESC)
);
```

### Node Types

ColaNode supports various node types:

- `workspace` - Top-level container
- `space` - Container within a workspace
- `discussion` - Discussion thread within a space
- `user` - User identity (scoped to workspace)
- `page` - Rich text document (Notion-like)
- `database_record` - Structured data entry
- `message` - Chat message (simpler than discussions)

## Hierarchical Relationships

### Workspace → Space → Discussion Hierarchy

```
Workspace (parent_id = NULL)
  └── Space (parent_id = workspace.id)
      └── Discussion (parent_id = space.id)
          └── Message/Comment (parent_id = discussion.id)
```

### SQL Schema for Relationships

```sql
-- Example: Finding all spaces in a workspace
SELECT * FROM nodes
WHERE type = 'space'
  AND parent_id = 'workspace-ulid-here';

-- Example: Finding all discussions in a space
SELECT * FROM nodes
WHERE type = 'discussion'
  AND parent_id = 'space-ulid-here';

-- Example: Finding entire hierarchy
WITH RECURSIVE node_tree AS (
  -- Start with workspace
  SELECT id, type, parent_id, attributes, 0 as depth
  FROM nodes
  WHERE id = 'workspace-ulid-here'

  UNION ALL

  -- Recursively find children
  SELECT n.id, n.type, n.parent_id, n.attributes, nt.depth + 1
  FROM nodes n
  INNER JOIN node_tree nt ON n.parent_id = nt.id
  WHERE n.deleted_at IS NULL
)
SELECT * FROM node_tree ORDER BY depth, created_at;
```

## Workspace and User Relationships

### Per-Workspace User Model

ColaNode uses a **per-workspace user identity** model. When a user joins a workspace, a distinct user node is created with a unique ID scoped to that workspace.

**Benefits**:

- Simplified authorization (user identity tied to workspace)
- Better auditing (workspace-scoped user actions)
- Data partitioning (easier to isolate workspace data)

### User-Workspace Linking Schema

```sql
-- User nodes (scoped to workspace)
-- Each user has a node with type='user' and parent_id=workspace.id
CREATE TABLE nodes (
  id TEXT PRIMARY KEY,              -- ULID for user in this workspace
  type TEXT NOT NULL,               -- 'user'
  parent_id TEXT NOT NULL,          -- workspace.id
  attributes JSONB NOT NULL,        -- User attributes:
                                    --   {
                                    --     "email": "user@example.com",
                                    --     "name": "John Doe",
                                    --     "avatar_url": "...",
                                    --     "role": "admin" | "member" | "viewer",
                                    --     "global_user_id": "global-user-ulid"  -- Links to global user account
                                    --   }
  created_at TIMESTAMPTZ DEFAULT NOW(),
  updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Global user accounts (for authentication)
CREATE TABLE global_users (
  id TEXT PRIMARY KEY,              -- ULID
  email TEXT UNIQUE NOT NULL,
  password_hash TEXT,               -- Hashed password
  created_at TIMESTAMPTZ DEFAULT NOW(),
  updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Workspace membership tracking
CREATE TABLE workspace_members (
  workspace_id TEXT NOT NULL,       -- References nodes.id where type='workspace'
  user_node_id TEXT NOT NULL,       -- References nodes.id where type='user'
  global_user_id TEXT NOT NULL,     -- References global_users.id
  role TEXT NOT NULL,               -- 'admin', 'member', 'viewer'
  joined_at TIMESTAMPTZ DEFAULT NOW(),

  PRIMARY KEY (workspace_id, user_node_id),
  FOREIGN KEY (workspace_id) REFERENCES nodes(id),
  FOREIGN KEY (user_node_id) REFERENCES nodes(id),
  FOREIGN KEY (global_user_id) REFERENCES global_users(id)
);
```

### Querying User-Workspace Relationships

```sql
-- Find all workspaces a user belongs to
SELECT w.*
FROM nodes w
INNER JOIN workspace_members wm ON w.id = wm.workspace_id
WHERE wm.global_user_id = 'user-global-id-here'
  AND w.type = 'workspace'
  AND w.deleted_at IS NULL;

-- Find all users in a workspace
SELECT u.*, wm.role
FROM nodes u
INNER JOIN workspace_members wm ON u.id = wm.user_node_id
WHERE wm.workspace_id = 'workspace-id-here'
  AND u.type = 'user'
  AND u.deleted_at IS NULL;
```

## Collaboration Tracking

### Node Reactions

Tracks user reactions (emojis, likes, etc.) on nodes:

```sql
CREATE TABLE node_reactions (
  id TEXT PRIMARY KEY,              -- ULID
  node_id TEXT NOT NULL,            -- References nodes.id
  user_node_id TEXT NOT NULL,       -- References nodes.id where type='user'
  reaction_type TEXT NOT NULL,      -- 'emoji', 'like', 'thumbs_up', etc.
  reaction_value TEXT,              -- Emoji character or reaction identifier
  created_at TIMESTAMPTZ DEFAULT NOW(),

  FOREIGN KEY (node_id) REFERENCES nodes(id),
  FOREIGN KEY (user_node_id) REFERENCES nodes(id),

  -- Prevent duplicate reactions from same user
  UNIQUE (node_id, user_node_id, reaction_type)
);

-- Indexes
CREATE INDEX idx_node_reactions_node_id ON node_reactions(node_id);
CREATE INDEX idx_node_reactions_user_node_id ON node_reactions(user_node_id);
```

### Node Interactions

Tracks user interactions with nodes (views, opens, reads):

```sql
CREATE TABLE node_interactions (
  id TEXT PRIMARY KEY,              -- ULID
  node_id TEXT NOT NULL,            -- References nodes.id
  user_node_id TEXT NOT NULL,       -- References nodes.id where type='user'
  interaction_type TEXT NOT NULL,   -- 'viewed', 'opened', 'read', 'edited'
  first_interaction_at TIMESTAMPTZ DEFAULT NOW(),
  last_interaction_at TIMESTAMPTZ DEFAULT NOW(),

  FOREIGN KEY (node_id) REFERENCES nodes(id),
  FOREIGN KEY (user_node_id) REFERENCES nodes(id),

  -- One interaction record per user per node per type
  UNIQUE (node_id, user_node_id, interaction_type)
);

-- Indexes
CREATE INDEX idx_node_interactions_node_id ON node_interactions(node_id);
CREATE INDEX idx_node_interactions_user_node_id ON node_interactions(user_node_id);
CREATE INDEX idx_node_interactions_type ON node_interactions(interaction_type);
```

### Collaboration Features Enabled

**Read Indicators**:

```sql
-- Check who has read a discussion
SELECT u.attributes->>'name' as user_name, ni.last_interaction_at
FROM node_interactions ni
INNER JOIN nodes u ON ni.user_node_id = u.id
WHERE ni.node_id = 'discussion-id-here'
  AND ni.interaction_type = 'read'
ORDER BY ni.last_interaction_at DESC;
```

**Reaction Counts**:

```sql
-- Count reactions per node
SELECT
  node_id,
  reaction_type,
  reaction_value,
  COUNT(*) as count
FROM node_reactions
WHERE node_id = 'node-id-here'
GROUP BY node_id, reaction_type, reaction_value;
```

**"Seen By" Status**:

```sql
-- Get "seen by" list for a node
SELECT
  u.attributes->>'name' as user_name,
  u.attributes->>'avatar_url' as avatar_url,
  ni.last_interaction_at as seen_at
FROM node_interactions ni
INNER JOIN nodes u ON ni.user_node_id = u.id
WHERE ni.node_id = 'node-id-here'
  AND ni.interaction_type = 'viewed'
ORDER BY ni.last_interaction_at DESC;
```

## Yjs Document Storage

### Collaborative Content (Pages, Database Records)

For collaborative content (pages, database records), ColaNode uses Yjs CRDTs:

```sql
-- Yjs document snapshots (PostgreSQL server)
CREATE TABLE yjs_documents (
  id TEXT PRIMARY KEY,              -- Same as nodes.id for collaborative content
  workspace_id TEXT NOT NULL,       -- References nodes.id where type='workspace'
  node_id TEXT NOT NULL,            -- References nodes.id
  document_type TEXT NOT NULL,      -- 'page', 'database_record'
  snapshot BYTEA,                   -- Yjs document binary snapshot
  version BIGINT NOT NULL,          -- Version number for optimistic locking
  updated_at TIMESTAMPTZ DEFAULT NOW(),

  FOREIGN KEY (workspace_id) REFERENCES nodes(id),
  FOREIGN KEY (node_id) REFERENCES nodes(id),

  UNIQUE (node_id)
);

-- Yjs operation log (for recovery and audit)
CREATE TABLE yjs_operations (
  id BIGSERIAL PRIMARY KEY,
  document_id TEXT NOT NULL,        -- References yjs_documents.id
  operation_data BYTEA NOT NULL,    -- Yjs update binary (delta)
  client_id TEXT,                   -- Client identifier
  user_node_id TEXT,                -- References nodes.id where type='user'
  created_at TIMESTAMPTZ DEFAULT NOW(),

  FOREIGN KEY (document_id) REFERENCES yjs_documents(id),
  FOREIGN KEY (user_node_id) REFERENCES nodes(id)
);

-- Indexes
CREATE INDEX idx_yjs_operations_document_id ON yjs_operations(document_id);
CREATE INDEX idx_yjs_operations_created_at ON yjs_operations(created_at DESC);
```

## Complete Schema (SQLite Local / PostgreSQL Server)

### Core Tables

```sql
-- =====================================================
-- 1. NODES TABLE (Core entity storage)
-- =====================================================
CREATE TABLE nodes (
  id TEXT PRIMARY KEY,              -- ULID
  type TEXT NOT NULL,               -- Node type
  parent_id TEXT,                   -- Parent node ID
  attributes JSONB NOT NULL,        -- Type-specific attributes
  created_at TIMESTAMPTZ DEFAULT NOW(),
  updated_at TIMESTAMPTZ DEFAULT NOW(),
  deleted_at TIMESTAMPTZ,           -- Soft delete

  FOREIGN KEY (parent_id) REFERENCES nodes(id),

  -- Indexes
  INDEX idx_nodes_type ON nodes(type),
  INDEX idx_nodes_parent_id ON nodes(parent_id),
  INDEX idx_nodes_created_at ON nodes(created_at DESC),
  INDEX idx_nodes_deleted_at ON nodes(deleted_at) WHERE deleted_at IS NULL
);

-- =====================================================
-- 2. GLOBAL USERS (Authentication)
-- =====================================================
CREATE TABLE global_users (
  id TEXT PRIMARY KEY,              -- ULID
  email TEXT UNIQUE NOT NULL,
  password_hash TEXT,
  created_at TIMESTAMPTZ DEFAULT NOW(),
  updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- =====================================================
-- 3. WORKSPACE MEMBERSHIP
-- =====================================================
CREATE TABLE workspace_members (
  workspace_id TEXT NOT NULL,       -- References nodes.id where type='workspace'
  user_node_id TEXT NOT NULL,       -- References nodes.id where type='user'
  global_user_id TEXT NOT NULL,     -- References global_users.id
  role TEXT NOT NULL,               -- 'admin', 'member', 'viewer'
  joined_at TIMESTAMPTZ DEFAULT NOW(),

  PRIMARY KEY (workspace_id, user_node_id),
  FOREIGN KEY (workspace_id) REFERENCES nodes(id),
  FOREIGN KEY (user_node_id) REFERENCES nodes(id),
  FOREIGN KEY (global_user_id) REFERENCES global_users(id)
);

-- =====================================================
-- 4. NODE REACTIONS
-- =====================================================
CREATE TABLE node_reactions (
  id TEXT PRIMARY KEY,              -- ULID
  node_id TEXT NOT NULL,
  user_node_id TEXT NOT NULL,
  reaction_type TEXT NOT NULL,
  reaction_value TEXT,
  created_at TIMESTAMPTZ DEFAULT NOW(),

  FOREIGN KEY (node_id) REFERENCES nodes(id),
  FOREIGN KEY (user_node_id) REFERENCES nodes(id),

  UNIQUE (node_id, user_node_id, reaction_type),
  INDEX idx_node_reactions_node_id ON node_reactions(node_id)
);

-- =====================================================
-- 5. NODE INTERACTIONS
-- =====================================================
CREATE TABLE node_interactions (
  id TEXT PRIMARY KEY,              -- ULID
  node_id TEXT NOT NULL,
  user_node_id TEXT NOT NULL,
  interaction_type TEXT NOT NULL,   -- 'viewed', 'opened', 'read', 'edited'
  first_interaction_at TIMESTAMPTZ DEFAULT NOW(),
  last_interaction_at TIMESTAMPTZ DEFAULT NOW(),

  FOREIGN KEY (node_id) REFERENCES nodes(id),
  FOREIGN KEY (user_node_id) REFERENCES nodes(id),

  UNIQUE (node_id, user_node_id, interaction_type),
  INDEX idx_node_interactions_node_id ON node_interactions(node_id)
);

-- =====================================================
-- 6. YJS DOCUMENTS (Collaborative content)
-- =====================================================
CREATE TABLE yjs_documents (
  id TEXT PRIMARY KEY,              -- Same as nodes.id
  workspace_id TEXT NOT NULL,
  node_id TEXT NOT NULL,
  document_type TEXT NOT NULL,      -- 'page', 'database_record'
  snapshot BYTEA,
  version BIGINT NOT NULL,
  updated_at TIMESTAMPTZ DEFAULT NOW(),

  FOREIGN KEY (workspace_id) REFERENCES nodes(id),
  FOREIGN KEY (node_id) REFERENCES nodes(id),

  UNIQUE (node_id)
);

-- =====================================================
-- 7. YJS OPERATIONS (Operation log)
-- =====================================================
CREATE TABLE yjs_operations (
  id BIGSERIAL PRIMARY KEY,
  document_id TEXT NOT NULL,
  operation_data BYTEA NOT NULL,
  client_id TEXT,
  user_node_id TEXT,
  created_at TIMESTAMPTZ DEFAULT NOW(),

  FOREIGN KEY (document_id) REFERENCES yjs_documents(id),
  FOREIGN KEY (user_node_id) REFERENCES nodes(id),

  INDEX idx_yjs_operations_document_id ON yjs_operations(document_id)
);
```

## Node Attributes Examples

### Workspace Node Attributes

```json
{
  "name": "Engineering Team",
  "description": "Engineering workspace",
  "settings": {
    "theme": "dark",
    "notifications": true
  },
  "created_by": "user-node-id"
}
```

### Space Node Attributes

```json
{
  "name": "Frontend Development",
  "description": "Frontend team discussions",
  "icon": "💻",
  "color": "#3b82f6"
}
```

### Discussion Node Attributes

```json
{
  "title": "React Performance Optimization",
  "status": "open", // 'open', 'closed', 'archived'
  "tags": ["react", "performance"],
  "pinned": false
}
```

### User Node Attributes

```json
{
  "email": "john@example.com",
  "name": "John Doe",
  "avatar_url": "https://...",
  "role": "admin", // 'admin', 'member', 'viewer'
  "global_user_id": "global-user-ulid",
  "status": "active", // 'active', 'inactive', 'suspended'
  "preferences": {
    "theme": "dark",
    "notifications": true
  }
}
```

### Page Node Attributes

```json
{
  "title": "Project Roadmap",
  "icon": "📋",
  "cover_image": "https://...",
  "yjs_document_id": "same-as-node-id"
}
```

## Common Queries

### Get Workspace Hierarchy

```sql
-- Get workspace with all spaces and discussions
WITH RECURSIVE workspace_tree AS (
  -- Start with workspace
  SELECT
    id,
    type,
    parent_id,
    attributes,
    0 as depth,
    id as workspace_id
  FROM nodes
  WHERE id = 'workspace-id-here'
    AND type = 'workspace'

  UNION ALL

  -- Recursively get children
  SELECT
    n.id,
    n.type,
    n.parent_id,
    n.attributes,
    wt.depth + 1,
    wt.workspace_id
  FROM nodes n
  INNER JOIN workspace_tree wt ON n.parent_id = wt.id
  WHERE n.deleted_at IS NULL
    AND n.type IN ('space', 'discussion', 'page')
)
SELECT * FROM workspace_tree ORDER BY depth, created_at;
```

### Get All Users in Workspace

```sql
SELECT
  u.id as user_node_id,
  u.attributes->>'name' as name,
  u.attributes->>'email' as email,
  u.attributes->>'avatar_url' as avatar_url,
  wm.role,
  wm.joined_at
FROM nodes u
INNER JOIN workspace_members wm ON u.id = wm.user_node_id
WHERE wm.workspace_id = 'workspace-id-here'
  AND u.type = 'user'
  AND u.deleted_at IS NULL
ORDER BY wm.joined_at;
```

### Get Discussion with Reactions and Interactions

```sql
SELECT
  d.id,
  d.attributes->>'title' as title,
  d.attributes->>'status' as status,
  -- Reaction counts
  (
    SELECT COUNT(*)
    FROM node_reactions nr
    WHERE nr.node_id = d.id
  ) as reaction_count,
  -- View count
  (
    SELECT COUNT(*)
    FROM node_interactions ni
    WHERE ni.node_id = d.id
      AND ni.interaction_type = 'viewed'
  ) as view_count,
  -- Last activity
  (
    SELECT MAX(last_interaction_at)
    FROM node_interactions ni
    WHERE ni.node_id = d.id
  ) as last_activity
FROM nodes d
WHERE d.id = 'discussion-id-here'
  AND d.type = 'discussion'
  AND d.deleted_at IS NULL;
```

### Get User's Recent Activity

```sql
SELECT
  n.id,
  n.type,
  n.attributes->>'title' as title,
  ni.interaction_type,
  ni.last_interaction_at
FROM node_interactions ni
INNER JOIN nodes n ON ni.node_id = n.id
WHERE ni.user_node_id = 'user-node-id-here'
  AND n.deleted_at IS NULL
ORDER BY ni.last_interaction_at DESC
LIMIT 50;
```

## IndexedDB Schema (Browser Client)

For browser clients, ColaNode also uses IndexedDB for Yjs document persistence:

```javascript
// IndexedDB structure (managed by y-indexeddb)
const dbName = "yjs-documents";
const storeName = "updates";

// Stores Yjs document updates
// Key: document-id
// Value: Yjs update binary data
```

## Schema Maintenance and Database Operations

### Schema Maintenance

ColaNode's database schema is maintained through:

1. **PostgreSQL Server**: The primary schema is defined in PostgreSQL on the server
2. **SQLite Client**: Clients maintain a matching schema in local SQLite databases
3. **Schema Synchronization**: Schema changes are propagated from server to clients during sync

**ORM Usage**:

ColaNode's server is built with **Node.js/TypeScript** and uses **PostgreSQL** as the primary database. While the documentation doesn't explicitly confirm the use of an ORM, the architecture suggests:

- **Possible ORM**: Prisma or similar TypeScript ORM (based on Node.js/TypeScript stack)
- **Alternative**: Raw SQL queries with a PostgreSQL client library (e.g., `pg` or `postgres`)
- **Migrations**: Schema changes are likely managed through SQL migration files or ORM migration tools

The node-based data model with JSONB attributes provides flexibility that works well with both ORM and raw SQL approaches.

### Database Operations

#### Creating a New Workspace

When a user creates a new workspace, the following database operations occur:

**Server (PostgreSQL)**:

```sql
BEGIN TRANSACTION;

-- 1. Create workspace node
INSERT INTO nodes (
  id,                    -- ULID generated
  type,                  -- 'workspace'
  parent_id,             -- NULL (root node)
  attributes,            -- JSONB with workspace metadata
  created_at,
  updated_at
) VALUES (
  'workspace-ulid-here',
  'workspace',
  NULL,
  '{
    "name": "Engineering Team",
    "description": "Engineering workspace",
    "settings": {
      "theme": "dark",
      "notifications": true
    },
    "created_by": "user-node-id"
  }'::jsonb,
  NOW(),
  NOW()
);

-- 2. Create user node for the creator (workspace-scoped)
INSERT INTO nodes (
  id,                    -- ULID for user in this workspace
  type,                  -- 'user'
  parent_id,             -- workspace.id
  attributes,            -- JSONB with user info
  created_at,
  updated_at
) VALUES (
  'user-workspace-ulid-here',
  'user',
  'workspace-ulid-here',
  '{
    "email": "creator@example.com",
    "name": "John Doe",
    "avatar_url": "...",
    "role": "admin",
    "global_user_id": "global-user-ulid"
  }'::jsonb,
  NOW(),
  NOW()
);

-- 3. Create workspace membership record
INSERT INTO workspace_members (
  workspace_id,
  user_node_id,
  global_user_id,
  role,
  joined_at
) VALUES (
  'workspace-ulid-here',
  'user-workspace-ulid-here',
  'global-user-ulid',
  'admin',
  NOW()
);

COMMIT;
```

**Client (SQLite)**:

The same operations are performed locally in SQLite, then synced to the server in the background.

**Key Points**:

- Workspace creation is a **transactional operation** ensuring atomicity
- Three records are created: workspace node, user node, and workspace membership
- The creator automatically becomes an admin of the workspace
- All operations use ULIDs for unique identifiers

#### Creating a New Space

When a user creates a new space within a workspace:

**Server (PostgreSQL)**:

```sql
BEGIN TRANSACTION;

-- 1. Create space node
INSERT INTO nodes (
  id,                    -- ULID generated
  type,                  -- 'space'
  parent_id,             -- workspace.id
  attributes,            -- JSONB with space metadata
  created_at,
  updated_at
) VALUES (
  'space-ulid-here',
  'space',
  'workspace-ulid-here',
  '{
    "name": "Frontend Development",
    "description": "Frontend team discussions",
    "icon": "💻",
    "color": "#3b82f6"
  }'::jsonb,
  NOW(),
  NOW()
);

-- 2. Optionally create initial interaction record
INSERT INTO node_interactions (
  id,
  node_id,
  user_node_id,
  interaction_type,
  first_interaction_at,
  last_interaction_at
) VALUES (
  'interaction-ulid-here',
  'space-ulid-here',
  'user-workspace-ulid-here',
  'viewed',
  NOW(),
  NOW()
);

COMMIT;
```

**Client (SQLite)**:

Same operations performed locally, then synced to server.

**Key Points**:

- Space creation is simpler than workspace creation (only one node)
- The `parent_id` links the space to its workspace
- Optional interaction tracking can record who created/viewed the space
- Spaces inherit workspace permissions

#### Adding a User to a Workspace

When a user is added to an existing workspace:

**Server (PostgreSQL)**:

```sql
BEGIN TRANSACTION;

-- 1. Verify global user exists
-- (Assuming global_user_id is provided)

-- 2. Create workspace-scoped user node
INSERT INTO nodes (
  id,                    -- ULID for user in this workspace
  type,                  -- 'user'
  parent_id,             -- workspace.id
  attributes,            -- JSONB with user info
  created_at,
  updated_at
) VALUES (
  'user-workspace-ulid-here',
  'user',
  'workspace-ulid-here',
  '{
    "email": "newuser@example.com",
    "name": "Jane Smith",
    "avatar_url": "...",
    "role": "member",              -- or 'admin', 'viewer'
    "global_user_id": "global-user-ulid",
    "status": "active"
  }'::jsonb,
  NOW(),
  NOW()
)
ON CONFLICT (id) DO NOTHING;  -- Prevent duplicates

-- 3. Create workspace membership record
INSERT INTO workspace_members (
  workspace_id,
  user_node_id,
  global_user_id,
  role,
  joined_at
) VALUES (
  'workspace-ulid-here',
  'user-workspace-ulid-here',
  'global-user-ulid',
  'member',              -- or 'admin', 'viewer'
  NOW()
)
ON CONFLICT (workspace_id, user_node_id) DO UPDATE
SET role = EXCLUDED.role,
    joined_at = CASE
      WHEN workspace_members.joined_at IS NULL
      THEN EXCLUDED.joined_at
      ELSE workspace_members.joined_at
    END;

COMMIT;
```

**Client (SQLite)**:

Same operations performed locally, then synced to server.

**Key Points**:

- User addition creates a **workspace-scoped user identity**
- The same global user can have different user nodes in different workspaces
- Membership record links the global user to the workspace-scoped user node
- Conflict handling prevents duplicate memberships
- Role can be updated if user is re-added to workspace

### Transaction Handling

All multi-step database operations (workspace creation, user addition) are wrapped in **database transactions** to ensure:

- **Atomicity**: All operations succeed or all fail
- **Consistency**: Database remains in a valid state
- **Isolation**: Concurrent operations don't interfere
- **Durability**: Changes are persisted

### Synchronization Flow

**Local-First Pattern**:

1. **Client Operation**: User action triggers local SQLite write
2. **Immediate UI Update**: UI reflects change instantly
3. **Background Sync**: Changes queued for server sync
4. **Server Persistence**: Server receives and persists changes to PostgreSQL
5. **Broadcast**: Server broadcasts changes to other connected clients
6. **Client Sync**: Other clients receive and apply changes to local SQLite

**Conflict Resolution**:

- Node-based model with ULIDs minimizes conflicts
- Yjs handles conflicts for collaborative content (pages, database records)
- Last-write-wins for simple node attributes
- Soft deletes (`deleted_at`) allow recovery of accidentally deleted nodes

## Summary

### Key Schema Characteristics

1. **Node-Based Model**: All entities are nodes with consistent structure
2. **Hierarchical Relationships**: Parent-child relationships via `parent_id`
3. **Per-Workspace Users**: User identities scoped to workspaces
4. **Collaboration Tracking**: Reactions and interactions tables
5. **Yjs Integration**: Separate tables for collaborative content (pages, database records)
6. **Soft Deletes**: `deleted_at` timestamp for soft deletion
7. **JSONB Attributes**: Flexible type-specific properties stored as JSON

### Relationship Summary

```
Global User (global_users)
  └── Workspace Membership (workspace_members)
      └── Workspace User Node (nodes, type='user')
          └── Workspace (nodes, type='workspace')
              └── Space (nodes, type='space', parent_id=workspace.id)
                  └── Discussion (nodes, type='discussion', parent_id=space.id)
                      ├── Reactions (node_reactions)
                      └── Interactions (node_interactions)
```

This schema design enables:

- ✅ Flexible hierarchical organization
- ✅ Efficient querying of relationships
- ✅ Collaboration tracking and features
- ✅ Offline-first operation with SQLite
- ✅ Real-time sync with Yjs CRDTs
