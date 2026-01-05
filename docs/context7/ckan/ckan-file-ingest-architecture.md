# CKAN File Ingest Architecture (Context7 Summary)

This document summarizes how **CKAN** handles user uploads, persistent file storage, and structured
ingest into the **FileStore** and **DataStore**, based on Context7 documentation and CKAN guides.

It is a useful reference for DataForge’s upload UX → filestore → structured ingest pipeline
(e.g. LAS → Parquet + metadata).

---

## 1. High-Level Architecture

- **CKAN Core**
  - Python web app (Pylons/Flask stack).
  - PostgreSQL for metadata (datasets, resources, users, etc.).
  - Optional **FileStore** for storing binary uploads.
  - Optional **DataStore** (PostgreSQL) for structured, queryable tabular data.
- **Extensions**
  - `ckanext-filestore`: manages file uploads to local/remote storage.
  - `ckanext-datastore`: provides a SQL‑backed data API and data dictionary.
  - `ckanext-datapusher` (or DataStore ETL): background service that loads files into the DataStore.

Conceptually:

- A **dataset** is a logical collection of **resources**.
- Each **resource** may be:
  - A URL to an external file, or
  - An uploaded file stored in the **FileStore**.
- If enabled, tabular resources can be ingested into the **DataStore**:
  - Rows/columns are stored in a PostgreSQL table.
  - A **data dictionary** describes the schema.
  - The **Data API** exposes query access, including SQL.

---

## 2. Storage Model

### 2.1 FileStore (Binary Storage)

File uploads go through **FileStore** via `resource_create` or `resource_update`:

```bash
curl -H'Authorization: API_KEY' \
  'http://yourhost/api/action/resource_create' \
  --form upload=@filetoupload.csv \
  --form package_id=my_dataset
```

- CKAN:
  - Saves binary content to a configured directory / object store.
  - Stores a **resource** record with:
    - `url_type='upload'`.
    - `url` / storage path.
    - Size, MIME type, checksum, etc.

### 2.2 DataStore (Structured Storage)

If the **DataStore** extension is active and the resource is tabular:

- CKAN (or DataPusher) can:
  - Create a **DataStore table** for the resource (`datastore_create`).
  - Import rows from the uploaded file into that table.
  - Mark the resource as `datastore_active = True`.

Templates check `res.datastore_active` to expose:

- **Download links** (CSV/TSV/JSON/XML) via `/datastore/dump/{resource_id}`.
- A **Data Dictionary** button that links to `datastore.dictionary`.

---

## 3. User Upload UX → Persistent FileStore

### 3.1 Upload via Web UI

**User workflow**

1. User navigates to a dataset page and clicks **“Add resource”** or **“Upload”**.
2. Chooses a file and fills metadata (format, description, etc.).
3. Submits the form.

**Call stack**

1. **Browser**: POST multipart form to `/dataset/new_resource` or `/api/action/resource_create`
   (depending on UI/API).
2. **CKAN Controller**:
   - Validates form.
   - Determines `package_id` (dataset).
   - Calls `resource_create` action.
3. **resource_create**:
   - Handles `upload` field (file object).
   - Writes file to **FileStore**.
   - Creates `resource` row in metadata DB:
     - `id` (resource_id).
     - `package_id`.
     - `url_type='upload'`.
     - `url` or storage path.
     - `format`, `mimetype`, `size`, etc.
4. **Response / UI**:
   - Redirects back to dataset page.
   - New resource appears in resource list with a download link.

CKAN’s FileStore API supports the same flow programmatically via `resource_create` and
`resource_update`.

---

## 4. Structured Ingest into DataStore

Once a file resource exists, CKAN can ingest it into the **DataStore** to support:

- SQL‑like queries via `datastore_search` and `datastore_search_sql`.
- Data dictionary views and schema editing.
- Multiple export formats (CSV/TSV/JSON/XML) from the structured table.

There are two main paths:

1. **Direct ingestion** using `datastore_create` with records provided explicitly.
2. **Automated ingestion** from FileStore via **DataPusher**.

### 4.1 Direct DataStore Ingest (API)

An application can bypass FileStore and write directly to the DataStore:

```bash
curl -X POST http://127.0.0.1:5000/api/3/action/datastore_create \
  -H "Authorization: API_KEY" \
  -d '{
    "resource": { "id": "RESOURCE_ID" },
    "fields": [
      { "id": "a" },
      { "id": "b" }
    ],
    "records": [
      { "a": 1, "b": "xyz" },
      { "a": 2, "b": "zzz" }
    ]
  }'
```

This creates a **DataStore table** linked to the existing resource.

### 4.2 DataPusher: File → DataStore Table

In typical CKAN portals, a background **DataPusher** service watches for new/updated resources:

- When a resource with a suitable format (CSV, TSV, XLSX, etc.) is uploaded:
  - CKAN sends a job to DataPusher (`paster datapusher submit <resource_id>` or webhook).
  - DataPusher downloads the file (using the resource URL).
  - It then:
    1. Parses the file.
    2. Infers the schema (field names/types).
    3. Calls `datastore_create` (or `datastore_upsert`) to:
       - Create/update the DataStore table.
       - Load all rows.
  - On success, `res.datastore_active` is set to `True`.

From a user’s perspective:

- They upload a CSV.
- After background processing, the dataset page shows:
  - **“Data API”** section.
  - **“Data Dictionary”** and structured download links.

---

## 5. Data Dictionary & Data API

When `res.datastore_active` is true, CKAN templates enable:

- **Data Dictionary**:

  ```jinja2
  {% if res.datastore_active %}
    {{ h.build_nav_icon('datastore.dictionary', _('Data Dictionary'),
        id=pkg.name, resource_id=res.id, icon='code') }}
  {% endif %}
  ```

  - Data dictionary shows:
    - Field names, types, labels, descriptions.
    - Custom metadata via `IDataDictionaryForm` plugins.

- **Data API**:
  - `datastore_search` and `datastore_search_sql` endpoints.
  - `datastore/dump/{resource_id}` for full exports in CSV, TSV, JSON, XML.
  - Supports rich filtering and SQL queries.

This is CKAN’s equivalent of:

- DataForge’s normalized tabular/Parquet representation.
- Rich querying and visualization over ingested curves.

---

## 6. Call Stack: “User Uploads CSV → Structured Data in DataStore”

### 6.1 Upload & Resource Creation

1. **User (Browser)** uploads CSV via dataset page.
2. **CKAN Controller**:
   - Calls `resource_create` with `upload` and `package_id`.
3. **FileStore**:
   - Saves binary file.
   - Resource row created (with `id=RESOURCE_ID`).
4. **(Optional) Trigger DataPusher**:
   - CKAN posts a job to DataPusher specifying `RESOURCE_ID`.

### 6.2 DataPusher ETL

5. **DataPusher Service**:
   - Downloads file from CKAN using resource URL and API key.
   - Parses the CSV/TSV/XLSX file.
   - Infers schema (columns + types).
   - Calls `datastore_create` / `datastore_upsert`:
     - Creates/upserts DataStore table `"RESOURCE_ID"`.
     - Inserts all rows as records.

6. **CKAN DataStore**:
   - Stores schema and data in PostgreSQL.
   - Marks resource as `datastore_active`.

### 6.3 User Access to Structured Data

7. **Dataset page reloaded**:
   - Shows:
     - **Download as CSV/TSV/JSON/XML** using `/datastore/dump/{RESOURCE_ID}`.
     - **Data Dictionary** button.
     - **Data API** examples (`datastore_search`, `datastore_search_sql`).

8. **Clients** (scripts, dashboards, etc.) use the Data API to:
   - Query rows (filter, sort, aggregate).
   - Fetch schema (fields, types) to build dynamic UIs.

---

## 7. Patterns Relevant to DataForge

From CKAN’s upload + ingest architecture, DataForge can borrow:

- **Clear separation between FileStore and DataStore**:
  - Always store the original LAS/curve files.
  - Treat normalized tabular/parquet layers as a **separate, queryable store**.

- **Resource-centric ingest**:
  - Each uploaded file maps to a resource-like record with:
    - Storage path.
    - Type, size, checksum.
    - Ingest status (pending, active, failed).

- **Background ETL service**:
  - Similar to DataPusher:
    - A worker process that:
      - Downloads / reads the file.
      - Parses LAS.
      - Writes normalized columns & metadata.
      - Flags the resource as “ingested” so the UI can show advanced tools.

- **Data dictionary & API layer**:
  - Maintain a per‑curve / per‑column dictionary:
    - Units, ranges, curve type, semantic tags.
  - Expose a Data API for curve queries (subset by depth, by curve, etc.), akin to
    `datastore_search` / `datastore_search_sql`.

CKAN is a good reference for designing a robust, user‑friendly pipeline from **upload UX → durable
file store → structured ingest → rich query APIs**.


