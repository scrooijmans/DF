# Dataverse File Ingest Architecture (Context7 Summary)

This document summarizes how **Dataverse** handles user file uploads and converts them into
archival/tabular representations, based on Context7 documentation and Dataverse guides.

It is particularly relevant to DataForge’s LAS → normalized columns → Parquet + metadata pipeline.

---

## 1. High-Level Architecture

- **Frontend**: Web UI (JSF/PrimeFaces) and REST APIs for dataset/file operations.
- **Application server**: Payara/Glassfish Java EE app running the Dataverse WAR.
- **Storage**:
  - Local filesystem or remote store (e.g. S3) for **binary file storage**.
  - PostgreSQL for **metadata and tabular data tables**.
- **Background processing**:
  - Asynchronous **ingest pipeline** driven by the `DataverseIngest` JMS/MQ queue.
  - Optional external workers (for big data) via S3 direct upload/download.

Conceptually:

- User uploads a file → Dataverse creates a `DataFile` record and stores the **original file**.
- If the file is of an **ingestable type** (tabular, some statistical formats, etc.), an ingest
  job is queued.
- Ingest converts the file into an **internal archival form** (normalized tabular data + summary
  statistics), while preserving the **original** for download.

---

## 2. Storage Model for Uploaded Files

For each uploaded file, Dataverse manages:

- A `DataFile` entity (metadata in PostgreSQL), including:
  - `contentType` (MIME type).
  - `filesize`, checksum, and storage identifier.
  - Link to the parent dataset version.
- A binary object stored in:
  - Local filesystem, or
  - S3/remote store (with optional **direct upload** and **redirected download**).
- For tabular ingest:
  - Derived **tabular storage** in database tables, including:
    - Data variables/columns.
    - Summary statistics.
  - Optional “preprocessed” JSON view accessible via
    `/api/access/datafile/$id?format=prep`.
  - The **original format** remains accessible via
    `/api/access/datafile/$id?format=original`.

This mirrors DataForge’s target of:

- Original LAS file in blob store.
- Normalized columns + metadata in SQLite + Parquet.

---

## 3. Upload & Ingest Workflow

### 3.1 User Upload (Front Door)

Typical paths:

- **Web UI upload**: via browser (multipart/form-data).
- **API upload**:
  - SWORDv2 (e.g. upload a zip of files).
  - Native API.
  - S3 **direct upload** (client sends file directly to S3 using pre-signed URLs).

In all cases, Dataverse:

1. Authenticates the user and checks dataset edit permissions.
2. Creates or updates a **dataset version**.
3. Creates a `DataFile` entry per uploaded file with:
   - `contentType`, `filesize`, checksum, storage ID, etc.
4. Places files into the configured storage backend.

If S3 direct upload is used:

- Client first calls `.../datasets/:persistentId/uploadurls` to obtain pre-signed URLs.
- Client uploads parts with `curl -X PUT` to S3 (`x-amz-tagging: dv-state=temp`).
- Client calls Dataverse “complete” endpoint to finalize upload (send eTags).

### 3.2 Ingest Trigger

After the file is registered as a `DataFile`:

- Dataverse checks **ingestibility** (MIME type / extension).
- If ingest is enabled and the file size is below the configured limit:
  - An ingest job is added to the **`DataverseIngest` queue** (Payara MQ).
  - Admins can inspect this queue (`imqcmd -u admin query dst -t q -n DataverseIngest`)
    and even purge it (`... purge dst -t q -n DataverseIngest`) if needed.

Big-data installs can set per-store ingest size limits:

```sh
./asadmin create-jvm-options "-Ddataverse.files.<id>.ingestsizelimit=<size in bytes>"
```

Setting this low (or zero) effectively disables ingest for very large files, mirroring a
“store original only” mode.

---

## 4. Tabular Ingest Pipeline (Conceptual)

Once an ingest job is dequeued:

1. **File retrieval**:
   - The ingest service retrieves a working copy of the uploaded file from storage
     (filesystem or S3, unless disabled via size limits).

2. **Format detection & plugin selection**:
   - Inspects `contentType` and/or file extension.
   - Selects a tabular ingest plugin (e.g. CSV, TSV, Stata, SPSS, R, etc.).

3. **Parsing & normalization**:
   - Parses the file into rows and columns.
   - Infers variable names, types, formats (numeric, categorical, dates).

4. **Archival representation**:
   - Writes normalized tabular data into an internal archival format:
     - Column-oriented tables in PostgreSQL (variables, variable categories, summary stats).
     - Optionally a derived, canonical tabular file (e.g. tab-delimited) as the “ingested”
       file associated with the `DataFile`.
   - The **original uploaded file** is retained and can be downloaded via `format=original`.

5. **Metadata enrichment**:
   - Populates dataset/file-level metadata (e.g. column labels, units, value labels).
   - Generates a **“preprocessed” JSON** representation (summaries, histograms) used by
     tools like the Data Explorer via `format=prep`.

6. **Finalization**:
   - Marks the `DataFile` as ingested/tabular.
   - Updates dataset version state so UI/API can:
     - Display variable lists.
     - Offer multiple download formats (original, tab-delimited, RData, etc.).

This is close to:

- DataForge reading LAS.
- Extracting depth, curves, and metadata.
- Writing normalized columns into Parquet + metadata rows into SQLite.

---

## 5. Call Stack: “User Uploads Tabular File → Archival Representation”

### 5.1 Request Path (Simplified)

1. **Browser / API client**
   - `POST /api/datasets/:persistentId/add` or SWORDv2, or direct S3 upload init.

2. **REST / SWORD Controller**
   - Validates permissions.
   - Creates/updates dataset version.
   - Registers the file as a `DataFile`.

3. **Storage Layer**
   - Writes file to:
     - File system directory (`FileSystemDirectory` in config), or
     - Remote store (S3) using pre-signed URLs.

4. **Ingest Scheduler**
   - Examines `DataFile`:
     - If ingestable and enabled (size < ingest limit), pushes message onto
       `DataverseIngest` queue (Payara MQ).

5. **Ingest Worker**
   - Dequeues job.
   - Retrieves file from storage.
   - Runs ingest plugin:
     - Parse → normalize → store tabular data + stats.
   - Writes derived tabular/archival representation and metadata.
   - Updates `DataFile` + dataset version state.

6. **User Access**
   - UI and API now present:
     - Original file downloads (`format=original`).
     - Tabular downloads (tab-delimited, RData, etc.).
     - Exploratory tools over the ingested tabular data (`format=prep`).

---

## 6. Relevance & Patterns for DataForge

From this Dataverse ingest architecture, DataForge can adopt:

- **Strict ingest pipeline**:
  - Separate “file registration/storage” from “ingest and normalization”.
  - Treat ingest as an asynchronous job queue (similar to `DataverseIngest`).

- **Dual representation**:
  - Always preserve the **original LAS** file.
  - Produce a normalized **archival representation**:
    - Columnar Parquet for curve data.
    - Metadata rows in SQLite.

- **Size-based ingest policies**:
  - Configurable thresholds beyond which ingest is skipped (store-only mode).

- **Rich metadata & preprocessed views**:
  - Generate summary stats and “prep” JSON for visualization components (charts, explorers).

These patterns align well with a robust, auditable ingest path for “uploaded LAS → normalized
archival form + metadata” in DataForge.


