# AG Grid JS + SQL: Request/Result → Table View Architecture (Context7 Summary)

This document summarizes how **AG Grid (JavaScript, vanilla)** can display data from a **SQL
database** in a table, based on AG Grid’s row model and datasource APIs.

It is a useful reference for DataForge’s future “SQL-backed tables in Svelte/JS frontends”.

---

## 1. High-Level Architecture

AG Grid itself does **not** talk directly to SQL databases. The typical architecture is:

- **Backend**
  - SQL database (Postgres, SQLite, etc.).
  - HTTP API layer (REST or GraphQL).
  - Endpoints that:
    - Accept paging / sort / filter parameters from AG Grid.
    - Execute SQL queries.
    - Return JSON rows and total row count.

- **Frontend**
  - AG Grid JS in the browser.
  - Uses either:
    - **Client-Side Row Model**: load all rows into memory (`rowData`) and let the grid handle
      filtering, sorting, grouping on the client.
    - **Server-Side / Infinite Row Model**: stream data in **blocks** via a **datasource** that
      calls backend endpoints which in turn query SQL.

Conceptually:

- SQL DB ←→ Backend API ←→ AG Grid **datasource** ←→ Grid **row model** ←→ Table UI.

---

## 2. Simple Case: Load SQL Results via REST into Client-Side Row Model

For smaller result sets, you can:

1. Backend exposes an endpoint, e.g.:
   - `GET /api/orders?limit=1000` → returns JSON:
     - `rows: Order[]` (array of objects).
2. Frontend calls the endpoint (`fetch`) and sets `rowData`.

### 2.1 Example (vanilla JS/TS)

```ts
import {
  ClientSideRowModelModule,
  ColDef,
  GridApi,
  GridOptions,
  ModuleRegistry,
  createGrid,
} from 'ag-grid-community';

ModuleRegistry.registerModules([ClientSideRowModelModule]);

// Column definitions must match the JSON row fields
const columnDefs: ColDef[] = [
  { field: 'id' },
  { field: 'customer_name' },
  { field: 'status' },
  { field: 'created_at' },
];

let gridApi: GridApi;

const gridOptions: GridOptions = {
  columnDefs,
  rowData: [], // will be filled from SQL-backed API
  defaultColDef: {
    flex: 1,
    sortable: true,
    filter: true,
  },
};

async function loadDataFromSqlApi() {
  const res = await fetch('/api/orders?limit=1000');
  const data = await res.json(); // expecting { rows: [...] }
  gridApi.setGridOption('rowData', data.rows);
}

const gridDiv = document.querySelector<HTMLElement>('#myGrid')!;
gridApi = createGrid(gridDiv, gridOptions);

loadDataFromSqlApi();
```

This matches AG Grid’s **Client-Side Row Model** pattern: data is provided via `rowData`, and all
sorting/filtering happens in the browser.

---

## 3. Scalable Case: Server-Side Row Model with SQL-backed Datasource

For large tables, AG Grid recommends **Server-Side Row Model** (Enterprise) or **Infinite Row
Model** (Community). Both:

- Use a **datasource** interface with a `getRows(params)` method.
- The grid asks for **blocks** of rows (with start/end indexes, filters, sorts).
- The datasource calls the backend, which turns that into SQL.

### 3.1 Conceptual Flow

1. User scrolls / grid initializes.
2. AG Grid calls `datasource.getRows(request)` where `request` contains:
   - `startRow`, `endRow` (paging).
   - `sortModel` (column + direction).
   - `filterModel` (per-column filters).
3. Datasource sends a request to backend REST endpoint, e.g.:

   ```http
   POST /api/orders/query
   Content-Type: application/json
   {
     "startRow": 0,
     "endRow": 100,
     "sortModel": [{ "colId": "created_at", "sort": "desc" }],
     "filterModel": {
       "status": { "filterType": "text", "type": "equals", "filter": "OPEN" }
     }
   }
   ```

4. Backend builds SQL, e.g.:

   ```sql
   SELECT *
   FROM orders
   WHERE status = 'OPEN'
   ORDER BY created_at DESC
   LIMIT 100 OFFSET 0;
   ```

   and also gets **total row count** for pagination:

   ```sql
   SELECT count(*) FROM orders WHERE status = 'OPEN';
   ```

5. Backend responds with:

   ```json
   {
     "rows": [ /* array of order objects */ ],
     "lastRow": 1234
   }
   ```

6. Datasource calls `params.success({ rowData: rows, rowCount: lastRow })`.
7. AG Grid updates the table and continues to request more blocks as needed.

### 3.2 Example Datasource (Server-Side Row Model)

Borrowing AG Grid’s `FakeServer` examples (which use AlaSQL) but replacing them with a real HTTP
call:

```ts
import {
  ColDef,
  GridApi,
  GridOptions,
  IServerSideDatasource,
  IServerSideGetRowsParams,
  ModuleRegistry,
  ServerSideRowModelModule,
  createGrid,
} from 'ag-grid-community';

ModuleRegistry.registerModules([ServerSideRowModelModule]);

const columnDefs: ColDef[] = [
  { field: 'id' },
  { field: 'customer_name' },
  { field: 'status' },
  { field: 'created_at' },
];

let gridApi: GridApi;

const gridOptions: GridOptions = {
  columnDefs,
  defaultColDef: { flex: 1, sortable: true, filter: true },
  rowModelType: 'serverSide', // key: use Server-Side Row Model
};

function createSqlBackedDatasource(): IServerSideDatasource {
  return {
    async getRows(params: IServerSideGetRowsParams) {
      // AG Grid sends sort/filter/paging info in params.request
      const body = JSON.stringify(params.request);

      try {
        const res = await fetch('/api/orders/query', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body,
        });

        if (!res.ok) {
          throw new Error(`HTTP ${res.status}`);
        }

        const { rows, lastRow } = await res.json();

        params.success({
          rowData: rows,
          rowCount: lastRow,
        });
      } catch (err) {
        console.error('Failed to load rows from SQL backend', err);
        params.fail();
      }
    },
  };
}

const gridDiv = document.querySelector<HTMLElement>('#myGrid')!;
gridApi = createGrid(gridDiv, gridOptions);

// Register datasource on grid ready
gridApi.setGridOption('serverSideDatasource', createSqlBackedDatasource());
```

On the backend, you implement the equivalent of AG Grid’s `FakeServer`:

- Map `filterModel` to `WHERE` clauses.
- Map `sortModel` to `ORDER BY`.
- Use `startRow` / `endRow` to compute `LIMIT/OFFSET`.

AG Grid’s docs show this with AlaSQL in the browser; for a real SQL DB you move that logic to your
server.

---

## 4. Applying This in Svelte / DataForge

Although AG Grid’s examples are in TypeScript/vanilla, the same pattern applies in Svelte:

- Create a Svelte component that:
  - Renders a `<div id="myGrid">`.
  - On mount, calls `createGrid` with `gridOptions`.
  - For **client-side** mode:
    - Fetch rows from a SQL-backed API once and set `rowData`.
  - For **server-side** mode:
    - Provide a datasource whose `getRows()` calls a SQL-backed API endpoint.

Architecturally:

- Keep **SQL specifics** on the backend:
  - Build safe SQL from AG Grid’s request.
  - Apply tenancy / security filtering.
- Keep the frontend **purely grid-focused**:
  - Map JSON rows → AG Grid rows.
  - Let the grid worry about paging, virtualization, and rendering.

This mirrors the AG Grid server-side row model samples (which use a `FakeServer` with SQL built via
Alasql) but swaps the in-browser SQL engine for a **real SQL database behind an HTTP API**.


