# ColaNode: Database Request → UI View Architecture (Context7 Summary)

This document explains, at an architectural level, how **ColaNode** issues requests to its
databases (local SQLite and remote Postgres) and presents results to the user, based on the
ColaNode repository and the existing Context7 ColaNode docs in this folder.

It is a useful reference for DataForge when designing “local-first DB → UI table / page” flows.

---

## 1. High-Level Data Flow

ColaNode uses a **local-first + sync** pattern:

- **Client (Electron / Web)**
  - Local **SQLite** database holds the primary working set:
    - Workspaces, pages, blocks, messages, database rows, etc.
  - **Yjs** CRDT documents for collaborative content (pages, records).
  - UI components read almost everything from **local state / SQLite**.

- **Sync layer**
  - Background sync service and **y-websocket** connections:
    - Push local changes to server.
    - Pull remote changes and apply to local DB + Yjs docs.

- **Server**
  - **PostgreSQL** as the main persistent store.
  - API / sync services that:
    - Accept CRDT updates and non-CRDT mutations.
    - Write to Postgres.
    - Broadcast changes to other clients.

Conceptually:

- **User action → Local DB (SQLite + Yjs) → UI updates immediately.**
- **Background sync → Remote DB (Postgres) → Other clients.**

The UI rarely blocks on remote calls; it reads from **local state** that is continuously updated.

---

## 2. Local Database Access (SQLite)

### 2.1 Local-First Write Path

For most entities (workspaces, pages, database rows, messages):

1. **User action** in the UI (e.g. create page, edit block, add row).
2. Frontend state logic:
   - Creates a domain object (e.g. page record, Yjs update).
   - Writes the change to **SQLite** through an internal data access layer:
     - Either via:
       - Direct SQL wrapped in a local DB module, or
       - A higher-level ORM / query builder the ColaNode client uses.
3. UI subscribes to local state:
   - Reacts immediately to the changed SQLite rows / Yjs documents.
   - Renders new rows / page content without waiting for server ACK.

The local DB acts as the **source of truth for the client UI**.

### 2.2 Reading From SQLite for Views

Views such as:

- Workspace sidebar (lists of pages/databases).
- Database/table views (Notion-like tables).
- Message threads / channels.

follow a pattern:

1. **Selector / query function**:
   - Encapsulates a query, e.g. “all pages in workspace X ordered by updated_at”.
   - For structured DB views: “rows for database table Y with filters/sorts”.
2. **Local query execution**:
   - Runs against **SQLite** (often via an abstraction).
   - Returns JS/TS objects representing rows.
3. **UI binding**:
   - Components map rows → UI cells:
     - For tables: column definitions + row data.
     - For lists: list items.
     - For detail views: fields bound to inputs.

Because the DB is local, queries are low-latency and can be executed frequently in response to
state changes or focus changes.

---

## 3. CRDT-Backed Data (Yjs) + SQLite

For collaborative content (pages, some database records), ColaNode combines:

- **Yjs docs** for real-time, conflict-free editing.
- **SQLite** for indexing/searching and non-CRDT fields.

Flow:

1. User edits a page:
   - Keystrokes update a **Yjs document**.
   - Yjs change is persisted locally (e.g. via IndexedDB and/or SQLite snapshotting).
2. Indexed / derived data:
   - Certain derived fields (last edited time, title, etc.) are stored in SQLite to support:
     - Fast sidebar queries.
     - Search.
3. UI reads:
   - For list views: query SQLite for high-level metadata (titles, IDs).
   - For detail editor: hydrate the Yjs doc and bind it to the editor.

The key is that **UI always reads from local state** (Yjs + SQLite), not from the server directly.

---

## 4. Server Communication & Remote Database (Postgres)

ColaNode’s client rarely talks to Postgres directly. Instead:

- **WebSocket sync** (y-websocket and other protocols):
  - Sends CRDT updates and operation logs to the server.
  - Receives remote updates and applies them to local SQLite + Yjs docs.
- **HTTP / RPC APIs**:
  - Used for:
    - Authentication.
    - Initial workspace provisioning.
    - Non-CRDT operations (e.g. file uploads, some admin actions).

On the **server**, handlers:

1. Validate the request (auth, workspace membership, etc.).
2. Apply changes to **Postgres** and server-side Yjs docs / event streams.
3. Broadcast events to other connected clients so their local DBs can update.

But for the **user-facing views**, the important path is:

- **Postgres → sync → client SQLite/Yjs → UI**, not direct Postgres → UI.

---

## 5. Example: Database/Table View (Notion-like)

A ColaNode database (table) view can be summarized as:

1. **User opens a database view**:
   - UI identifies the database ID and view configuration (columns, filters, sorts).
2. **Local query**:
   - A query module builds an SQL statement for SQLite:
     - `SELECT * FROM database_rows WHERE db_id = ? AND ... ORDER BY ...`
   - Runs it locally and gets rows.
3. **UI table rendering**:
   - Columns are configured from metadata:
     - Name, type, formatting, width.
   - Rows are bound to cells, very similar to how AG Grid or DBeaver visualize tables.
4. **Edits**:
   - Inline or dialog-based edits write back to SQLite immediately.
   - For CRDT-backed DBs, edits also mutate Yjs documents linked to rows.
5. **Background sync**:
   - A sync worker detects local changes and sends them to the server.
   - When remote updates arrive, they update SQLite rows and trigger re-render.

This is functionally close to:

- DBeaver’s “JDBC ResultSet → paged grid”.
- AG Grid’s “datasource → row model → table”.

But in ColaNode:

- The “datasource” is **local SQLite + CRDT state**, not a remote DB.

---

## 6. Example: Messages / Channels

Messages are typically **not CRDT-based** (no concurrent text editing). The pattern:

1. **Fetch / subscribe locally**:
   - Messages for channel X are stored in SQLite.
   - A query like `SELECT * FROM messages WHERE channel_id = ? ORDER BY created_at` feeds the view.
2. **New message**:
   - User sends a message.
   - Client writes the message row into SQLite.
   - UI appends it to the message list instantly.
3. **Sync**:
   - Background sync pushes the message to the server.
   - Server persists it in Postgres and broadcasts to other clients.
   - Other clients receive it via sync and insert into their local SQLite, causing their UIs to
     update.

Again: **UI only depends on local DB**, while sync keeps local and server state aligned.

---

## 7. Patterns Relevant to DataForge

ColaNode’s database request and view architecture suggests several patterns for DataForge:

- **Local-first query path**
  - Always read from a **local database** (SQLite) or local state store.
  - Treat remote DBs or cloud services as **replicas / sync targets**, not primary UI sources.

- **Separation of concerns**
  - UI components:
    - Express views in terms of “give me rows for X with filters Y”.
  - Data access layer:
    - Resolves those views by querying SQLite (and maybe CRDT docs).
  - Sync layer:
    - Independently keeps SQLite in sync with any remote systems.

- **Consistent table/view model**
  - Use a DBeaver/AG Grid–style table abstraction:
    - Columns defined from metadata.
    - Rows bound from local query results.
    - Edits go back through a single data access API that writes to SQLite.

- **CRDT + DB composition**
  - For collaborative content (e.g. shared workbooks / notes):
    - Represent live content in a CRDT (like Yjs).
    - Maintain derived/indexed fields in SQLite for fast list/search views.

Adopting these patterns lets DataForge keep **database requests low-latency and predictable** while
still supporting sync, collaboration, and rich local querying, similar to ColaNode.
