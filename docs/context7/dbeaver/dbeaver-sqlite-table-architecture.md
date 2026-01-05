# DBeaver + SQLite: Request/Result → Table View Architecture (Context7 Summary)

This document summarizes how **DBeaver** talks to **SQLite** and presents results in its
spreadsheet-style **data grid**, based on Context7 documentation and DBeaver guides.

It’s a useful reference for DataForge when thinking about “SQLite query → paged table UI”.

---

## 1. High-Level Architecture

- **Core runtime**
  - DBeaver is a **Java/Eclipse RCP** desktop app.
  - All database access goes through **JDBC drivers** (including SQLite JDBC).
  - The UI is SWT/JFace-based; data is shown via a custom **data editor grid**.

- **Driver layer (SQLite)**
  - Uses a standard **JDBC connection URL** (e.g. `jdbc:sqlite:/path/to/file.db`).
  - Connection properties can enable extras like:
    - `enable_load_extension=true` (to allow `load_extension()`).

- **SQL execution layer**
  - SQL text is sent to the JDBC driver via standard `Connection` / `Statement` / `ResultSet`.
  - DBeaver wraps this in its own **execution context** so it can:
    - Track running queries.
    - Cancel/abort.
    - Stream results to the UI in pages.

- **Data editor / grid**
  - A reusable **Result Set Viewer** widget:
    - Displays rows in a table/grid.
    - Supports paging, filtering, sorting, in-place editing.
    - Uses lazy fetch to avoid loading the entire result set.

Conceptually:

- **SQLite file** ←→ JDBC connection.
- **SQL query** → JDBC `ResultSet`.
- `ResultSet` → **paged in‑memory model** → **UI grid cells**.

---

## 2. SQLite Request Path

### 2.1 Connection Setup

1. User configures a **SQLite connection** in DBeaver:
   - Selects the `.db` file path.
   - Optionally sets driver properties (e.g. `enable_load_extension=true`).
2. DBeaver builds a JDBC URL (using driver URL template, e.g. `jdbc:sqlite:{file}`).
3. DBeaver opens a JDBC connection:
   - Manages it in a **DB session** tied to the UI workspace.

From this point, any SQL editor or data viewer operations use that JDBC connection.

### 2.2 SQL Execution

When the user runs a query, e.g.:

```sql
SELECT * FROM my_table;
```

the flow is:

1. **SQL Editor**:
   - Computes the statement under cursor / selection.
   - Sends it to the **SQL execution engine**.
2. **Execution engine**:
   - Uses JDBC to execute the SQL:
     - `PreparedStatement` or `Statement`.
     - Gets a JDBC `ResultSet`.
   - Wraps the `ResultSet` in DBeaver’s internal result model.
3. **Result Set Viewer**:
   - Binds the result model to the grid.
   - Starts fetching the first “page” of rows.

For SQLite, this is the same pipeline as for other relational DBs; the driver is the only
SQLite‑specific part.

---

## 3. Result Paging & Lazy Loading

DBeaver **never loads an entire huge result set at once** by default. Instead it:

- Has a configurable **maximum result‑set size** / page size (default ~200 rows).
- Fetches rows **on demand** as the user scrolls.

From the docs:

> By default, DBeaver limits the number of rows fetched to **200** …  
> Once you scroll to the last row of the current result portion, DBeaver fetches the next portion.  
> You can also manually fetch the next portion of data or fetch all rows.

Mechanically:

1. **Initial load**
   - Executes the query.
   - Reads up to **N rows** from the `ResultSet` into an in‑memory buffer.
   - Renders those rows into the grid.
2. **Scrolling to bottom**
   - When the user hits the last buffered row:
     - Viewer asks execution layer to **fetch next page** (another N rows).
     - Data model appends them; grid repaints rows.
3. **Manual fetch**
   - Toolbar actions:
     - “Fetch next page of results” → fetch one more page.
     - “Fetch all rows” → consume the full `ResultSet` into memory
       (with warnings about large results).

This architecture:

- Keeps memory use bounded for large SQLite tables.
- Keeps UI responsive—rows stream in as needed.

---

## 4. Data Grid / Presentation Model

The **Data Editor** (result grid) acts as a layered adapter over the raw `ResultSet`:

- **Core responsibilities**
  - Map JDBC column metadata → grid columns (names, types, nullable, PK/FK hints).
  - Map each row in the buffer → grid row object.
  - Handle:
    - Sorting / filtering (often by re‑running queries, not client‑side).
    - Cell editing (for updatable `ResultSet`s).
    - Copy/paste, export, and data transforms.

- **Presentation features** (from DBeaver docs):
  - Color by value / data type.
  - Transform representations (e.g. view as URL, binary, epoch time).
  - Grouping panel → DBeaver builds a `GROUP BY` SQL query and executes it.
  - Export results directly from query or from the grid (Data transfer wizard).

For SQLite, these features still operate on the **same ResultSet model**; there’s no special
SQLite grid implementation—the differences are in:

- The JDBC driver (SQLite semantics).
- What SQL features (e.g. window functions, extensions) the driver supports.

---

## 5. Internal SQLite Use (JSON/MultiSource Drivers)

DBeaver also uses **SQLite internally** for some file‑based “virtual” drivers (JSON, MultiSource):

- JSON / file drivers:
  - Simple queries read data directly from the files.
  - Complex queries (with `WHERE`, `JOIN`, `ORDER BY`, `GROUP BY`, etc.) trigger an
    **import into an internal SQLite database** for processing.
  - A driver property like `internalDbFilePath` can control where this internal DB lives.

Example from JSON driver docs:

```properties
# Persistent internal SQLite DB for complex queries
internalDbFilePath=C:\\User\\database.db
```

Flow for those virtual drivers:

1. Parse source file(s) (e.g. JSON).
2. Materialize them as **SQLite tables** in an internal DB.
3. Run user SQL via SQLite.
4. Present the `ResultSet` in the **same data grid UI** as any other database.

This shows DBeaver’s pattern:

- Use **SQLite as an internal query/compute engine** when needed.
- Always surface results through the **same Result Set Viewer and paging**.

---

## 6. Patterns Relevant to DataForge

For DataForge’s SQLite‑backed views, DBeaver suggests several patterns:

- **Standard DB pipeline**
  - Treat SQLite like any other DB:
    - Single query abstraction.
    - Result stream → paged view model → grid.

- **Paged fetching**
  - Limit rows per page (e.g. 200–1,000).
  - Fetch next/prev pages on scroll or explicit “Next page” actions.
  - Expose “Fetch all” carefully with warnings.

- **Stable, reusable grid**
  - One generic grid component for all tables/views, configured by:
    - Column metadata (name, type, units).
    - Data source (query or stored view).

- **Internal SQLite for virtual sources**
  - For non‑SQL or file sources (e.g. LAS/Parquet previews), consider:
    - Loading subsets into an internal SQLite/temp table.
    - Running SQL over that for complex filters/joins.
    - Rendering via the same grid/paging model.

These match DataForge’s goal of **SQLite as the local truth** with a **rich, DBeaver‑style table
viewer** on top.
