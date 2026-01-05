# ColaNode CRDT & Conflict Resolution Architecture (Context7 Summary)

This document dives into how **ColaNode** uses **CRDTs (Yjs)** together with **local SQLite** and
**server-side PostgreSQL** to handle concurrent edits and conflict resolution between local and
remote state.

It builds on:

- `colanode-architecture-tech-stack.md`
- `colanode-database-request-and-view-architecture.md`
- `colanode-architecture-assessment.md`

and adds a deeper look at **how conflicts are avoided or resolved** in different parts of the
system.

---

## 1. Big Picture: Where Conflicts Can Happen

ColaNode’s data breaks down into two broad categories:

- **CRDT-managed data (Yjs)**:
  - Pages (rich text content).
  - Database records (Notion-style structured rows, for collaborative fields).
  - Some UI configuration / layout data.
  - Conflict resolution is **inherent** in Yjs CRDTs; clients merge changes without
    app-specific merge code.

- **Non-CRDT relational data**:
  - Messages (chat-style, append-only).
  - File metadata and operations.
  - Workspace membership, permissions, settings.
  - Conflict handling is more traditional:
    - Primary keys / unique constraints.
    - Timestamps / versions.
    - Last-write-wins or domain-specific rules.

At a high level:

- **Yjs + WebSocket + IndexedDB/SQLite** handle _per-document_ CRDT conflicts.
- **SQLite (local)** and **PostgreSQL (server)** handle relational data with more conventional
  sync semantics.

---

## 2. CRDT Stack: Yjs on Client, Snapshots in Server DB

### 2.1 Libraries & Components

- **Yjs**:
  - Core CRDT framework used on the client and server.
  - Provides types like `Y.Text`, `Y.Map`, `Y.Array` used for pages and structured records.
- **y-websocket**:
  - WebSocket protocol and provider for syncing Yjs documents.
  - Client: `WebsocketProvider`.
  - Server: y-websocket-compatible WebSocket server; often coupled with Redis + Postgres.
- **y-indexeddb**:
  - Client-side persistence for Yjs documents (browser).
  - For Electron/desktop, ColaNode also persists Yjs state via SQLite for robustness.
- **SQLite (client)**:
  - Holds:
    - Local copies of Yjs state, or derived data (page metadata, record indexes).
    - Non-CRDT tables (messages, file ops, etc.).
- **PostgreSQL (server)**:
  - Stores:
    - Yjs document snapshots (binary).
    - Yjs operation logs (optional, inferred from context).
    - Relational metadata: workspaces, users, roles, messages, etc.
- **Redis**:
  - Pub/sub and ephemeral coordination:
    - Broadcast Yjs updates between WebSocket workers.
    - Cache hot data.

Yjs is **the CRDT engine**; SQLite/Postgres are **persistence and indexing layers**. Conflicts are
resolved _before_ data is written to the DBs.

---

## 3. CRDT Data Flow: Local Edit → Local DB → Server → Other Clients

### 3.1 Local-First Edit Flow

For a collaborative page or record:

1. **User types / edits** in the UI (React component).
2. The UI calls an application service (`PageService`, `DatabaseService`) which:
   - Mutates the **Yjs document**:

     ```ts
     const ydoc = getYDoc(pageId);
     const ytext = ydoc.getText('content');
     ytext.insert(index, text); // or delete/replace
     ```

3. Yjs emits an `update` event.
4. Application/infrastructure layer reacts to the Yjs update:
   - Persist updated state locally:
     - Via **y-indexeddb** (browser).
     - Or via a Yjs → SQLite adapter in Electron.
   - Update any **derived tables** in SQLite (e.g. page metadata, search index).
5. UI re-renders based on the updated Yjs state (and any derived SQLite data).

This all happens **without contacting the server**. The page is fully usable offline.

### 3.2 Background Sync to Server

In parallel:

1. The **WebSocket provider** (`WebsocketProvider`) observes Yjs updates:

   ```ts
   const wsProvider = new WebsocketProvider(
   	'wss://server.colanode.com',
   	`workspace-${workspaceId}/doc-${documentId}`,
   	ydoc
   );
   ```

2. It sends Yjs **delta updates** over WebSocket to the server:
   - These are CRDT operations (not entire docs).
   - They are **commutative and idempotent**.
3. Server-side Yjs instance:
   - Receives the update, applies it to the server’s Yjs doc.
   - Optionally writes:
     - A **binary snapshot** of the whole Yjs doc to Postgres (`yjs_documents.snapshot`).
     - A **log entry** per operation into `yjs_operations`.
4. Server uses Redis pub/sub to:
   - Broadcast updates to other WebSocket worker instances.
   - Notify other connected clients.
5. Other clients’ Yjs docs:
   - Apply the same updates.
   - CRDT properties ensure all replicas converge to the same state.

**Conflict resolution** is entirely handled by Yjs:

- If multiple users type concurrently:
  - Each client generates its own local operations.
  - After updates propagate and merge, all nodes see a coherent combined result.
  - No explicit “conflict detection” code is required in ColaNode’s app logic.

---

## 4. Conflict Resolution Semantics in Yjs

From the perspective of ColaNode’s architecture:

- **Yjs guarantees convergence**:
  - For CRDT-managed fields, all clients will eventually agree on a single state.
  - Conflicts (concurrent edits) are resolved deterministically by the CRDT algorithm.

- **Operational behavior**:
  - Yjs uses **logical clocks** and per-client **IDs** to order concurrent operations.
  - Data types (e.g. `Y.Text`, `Y.Map`) define how operations interleave:
    - Text edits interleave by character positions.
    - Map updates follow last-writer semantics with deterministic tiebreakers.
  - Duplicated updates (e.g. replays) are ignored via internal versioning identifiers.

For ColaNode:

- This means **no merge UIs** and **no per-field conflict resolution hooks** are necessary for
  pages and collaborative records.
- The app can treat the Yjs state as “ground truth” and mirror it to SQLite / Postgres.

---

## 5. Relational Data & Sync: Where Conflicts Still Matter

Not all data in ColaNode is CRDT-managed. Some examples:

- **Messages / chat history**:
  - Typically append-only.
  - Stored as rows in SQLite (local) and Postgres (server).
  - Conflicts are rare because:
    - Each message has a unique ID.
    - Operations are mostly inserts; updates/deletes are much less frequent.
  - Sync is closer to “log shipping”:
    - Local queue of unsent messages.
    - Server assigns authoritative timestamps or sequence numbers.

- **File metadata and operations**:
  - Tables for uploads, versions, permissions.
  - Conflicts (e.g. renaming the same file concurrently) are handled by:
    - Last-write-wins.
    - Or domain-specific rules (not deeply CRDT-ified).

- **Workspace membership, roles, settings**:
  - Managed centrally on the server.
  - Clients read from server or from local cache.
  - Writes are validated server-side; conflicts resolved by server business logic.

In these areas, **conflict detection** looks more traditional:

- Unique constraints in Postgres (e.g. `(workspace_id, slug)`).
- `updated_at` / `version` fields for optimistic concurrency.
- Server-side decisions about which update wins, sometimes guided by:
  - “Last modified wins.”
  - “Admins override members.”

SQLite is often just a **cache** or **offline queue**; authoritative conflict resolution happens on
the server.

---

## 6. Local DB vs Server DB: How They Stay in Sync

### 6.1 CRDT-Managed Tables

For entities where the **Yjs doc is the source of truth** (e.g. pages):

- **Client side**:
  - Yjs is primary.
  - SQLite stores:
    - References to the Yjs doc (`doc_id`).
    - Derived metadata (title, updated_at, search index).
  - When Yjs changes:
    - Event handlers update relevant SQLite tables to keep them in sync.

- **Server side**:
  - Yjs doc snapshots and operation log in Postgres.
  - Relational tables store metadata (workspace ID, permissions, tags, etc.).
  - A background process or hooks:
    - Persist Yjs doc snapshots to Postgres.
    - Optionally update relational views/materialized views for querying.

**Conflicts**:

- Between local & server:
  - Are resolved at the Yjs layer, not at the SQL layer.
  - SQL tables only see already-merged Yjs state.

### 6.2 Non-CRDT Tables

For messages, file metadata, etc.:

- **Local SQLite**:
  - Holds a subset of server state for offline use.
  - Maintains an **outbox / sync queue** for pending changes.

- **Server Postgres**:
  - Authoritative source.
  - Applies:
    - Inserts/updates/deletes from clients.
    - Constraints and business rules.

- **Sync protocol** (simplified):
  1. Client pushes a batch of changes (messages, metadata).
  2. Server applies them, returning:
     - Success/fail for each change.
     - Any new/updated rows since a given cursor.
  3. Client updates SQLite to:
     - Mark successful local changes as synced.
     - Store new/updated server rows.

Conflicts (e.g. updating a row from multiple clients) are detected by:

- Version columns, timestamps, or state flags.
- Server decides which change to keep; clients simply reflect server state on next sync.

---

## 7. Example: Page Edit Conflict Between Two Devices

Consider a page open on **Device A** and **Device B**:

1. Both devices have:
   - Local Yjs doc for `page-123`.
   - Local SQLite rows referencing the page.

2. Device A types “Hello”.
   - Yjs on A updates.
   - A’s SQLite + UI update.
   - Yjs delta is sent via WebSocket to server.

3. Device B, still offline, types “World”.
   - Yjs on B updates.
   - B’s SQLite + UI update.
   - No server connectivity; changes remain local.

4. When B comes online:
   - B’s WebSocket reconnects.
   - B sends its Yjs updates to the server.
   - Server merges updates from A and B into a single Yjs document:
     - Final text may be “HelloWorld” or “WorldHello” depending on CRDT order.
   - Server broadcasts the resulting Yjs state to A and B.

5. Both clients apply the merged Yjs doc:
   - Yjs on A and B converge.
   - SQLite on A and B is updated from Yjs state.
   - UIs render the same final content.

**Key point**: Neither client nor server needed to detect “conflict” or present a merge UI.
Conflict resolution is purely **CRDT semantics** inside Yjs.

---

## 8. Example: Message Conflict (Non-CRDT)

For messages in a channel:

1. Device A and B both send messages while offline.
2. Each device stores messages locally in SQLite with:
   - Temporary local IDs.
   - Content, author, timestamp (local).
3. On reconnect:
   - Clients push unsynced messages to server.
   - Server:
     - Assigns canonical IDs and server timestamps.
     - Stores them in Postgres.
4. On subsequent sync:
   - Clients map local temp IDs to server IDs.
   - Ordering in UI is based on server timestamp or sequence.

Conflicts such as two messages with the same temp ID are impossible once normalized; the server is
the single assigner of canonical IDs. There is no need for CRDT semantics here because message
streams are naturally append-only and can be linearly ordered.

---

## 9. Design Takeaways for DataForge

From ColaNode’s architecture:

- **Use CRDTs only where they pay off**:
  - Rich, shared documents, or structured records with concurrent editing.
  - Let Yjs handle all conflict resolution there.

- **Keep relational data simple**:
  - Use classic relational patterns (PKs, FKs, timestamps, versioning).
  - Make the server authoritative and let clients treat SQLite as cache + outbox.

- **Treat Yjs as a single source of truth for CRDT-managed content**:
  - SQLite/Postgres mirror and index that state, but do not participate in merge logic.

- **Align sync strategies with data semantics**:
  - CRDT docs → streaming deltas via WebSocket, automatic conflict resolution.
  - Non-CRDT tables → batched REST/GraphQL sync with explicit conflict policies.

Adopting these patterns lets DataForge build a robust local-first architecture where:

- Local SQLite stays fast and responsive.
- Conflicts on collaborative content are handled by Yjs, not ad-hoc merge code.
- Relational metadata and logs use simpler, well-understood conflict resolution strategies.
