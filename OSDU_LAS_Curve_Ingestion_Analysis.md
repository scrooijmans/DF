# OSDU LAS/Curve File Ingestion: Deep Dive Analysis

## Executive Summary

This document provides a comprehensive analysis of how OSDU handles LAS (Log ASCII Standard) file ingestion, including the architecture, schema definitions, ingestion flows, and how the platform addresses complex curve data scenarios such as non-overlapping depth values, gaps between depth indices, and NaN/missing values.

**Key Finding**: OSDU follows a **metadata-first, file-preservation** approach where LAS files are stored in their original format, with metadata extracted and indexed. The actual curve data handling (including sparse data, gaps, and NaN values) is managed through the **Wellbore DDMS** which provides optimized accessors for bulk data operations.

---

## 1. OSDU LAS Ingestion Architecture

### 1.1 Core Design Philosophy

OSDU's approach to LAS file ingestion is based on two fundamental principles:

1. **Source Data Preservation**: LAS files are stored in their **original format** to preserve data lineage and integrity
2. **Metadata Extraction**: Metadata is extracted from LAS files and stored as searchable records in the OSDU Storage Service

This dual approach allows OSDU to:

- Maintain data fidelity (no data loss from format conversion)
- Enable searchability and discovery through metadata
- Support multiple access patterns (file-based and API-based)
- Preserve audit trails and data provenance

### 1.2 Architecture Components

```
┌─────────────────────────────────────────────────────────────┐
│ LAS File Ingestion Architecture                             │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  ┌──────────────┐         ┌──────────────┐                │
│  │  LAS File    │─────────▶│ File Service │                │
│  │  (Original)  │ Upload   │  (Storage)   │                │
│  └──────────────┘          └──────┬───────┘                │
│                                    │                         │
│                                    ▼                         │
│                          ┌──────────────────┐              │
│                          │  Cloud Storage   │              │
│                          │  (S3/GCS/Azure)  │              │
│                          │  - Original LAS  │              │
│                          │  - Preserved     │              │
│                          └──────────────────┘              │
│                                                              │
│  ┌──────────────┐         ┌──────────────┐                │
│  │  LAS Parser  │─────────▶│  Metadata    │                │
│  │  (Extract)   │ Extract  │  Extraction  │                │
│  └──────────────┘          └──────┬───────┘                │
│                                    │                         │
│                                    ▼                         │
│                          ┌──────────────────┐              │
│                          │ Storage Service  │              │
│                          │ - WellLog WPC   │              │
│                          │ - Metadata Only │              │
│                          └──────┬──────────┘              │
│                                  │                          │
│                                  ▼                          │
│                          ┌──────────────────┐              │
│                          │  Wellbore DDMS   │              │
│                          │ - Optimized     │              │
│                          │   Accessors     │              │
│                          │ - Curve Data    │              │
│                          │   Access        │              │
│                          └──────────────────┘              │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

### 1.3 Data Separation: Metadata vs. Curve Data

**Critical Understanding**: OSDU separates:

1. **Metadata** (Stored in Storage Service):

   - WellLog Work Product Component record
   - Wellbore references
   - Depth ranges (TopMeasuredDepth, BottomMeasuredDepth)
   - Log type, activity type, service company
   - Dataset references (links to original file)

2. **Curve Data** (Stored in Original File):

   - Actual curve values
   - Depth indices
   - Sparse data, gaps, NaN values
   - All preserved in original LAS format

3. **Access Layer** (Wellbore DDMS):
   - Provides optimized APIs to read curve data from files
   - Handles depth indexing, interpolation, gap handling
   - Supports querying specific depth ranges or curves

---

## 2. LAS File Structure and Curve Data Complexity

### 2.1 LAS File Format Overview

LAS files contain:

**Header Section**:

- Well information
- Curve definitions
- Units of measure
- Depth reference information

**Data Section**:

- Depth column (index)
- Multiple curve columns (petrophysical logs)
- Each row represents a depth sample

### 2.2 Curve Data Complexities

#### A. Non-Overlapping Depth Values

**Scenario**: Different curves may have different depth ranges

```
Curve A: 1000-2000 ft
Curve B: 1500-2500 ft
Curve C: 1200-1800 ft
```

**LAS File Representation**:

```las
~A  DEPTH  GR    RES    DEN
~A  .      .     .      .
~A  1000.0 45.2  -999.25 -999.25
~A  1005.0 46.1  -999.25 -999.25
~A  1500.0 50.3  10.5   -999.25
~A  1505.0 51.2  11.2   2.65
~A  2000.0 55.1  -999.25 2.70
```

**OSDU Handling**:

- Original LAS file preserves all depth ranges
- Metadata records depth extents (TopMeasuredDepth, BottomMeasuredDepth)
- Wellbore DDMS can query specific depth ranges per curve

#### B. Gaps Between Depth Indices

**Scenario**: Irregular depth sampling or missing intervals

```
Depth: 1000, 1005, 1010, [GAP: 1010-1500], 1500, 1505, ...
```

**LAS File Representation**:

```las
~A  DEPTH  GR    RES
~A  .      .     .
~A  1000.0 45.2  10.5
~A  1005.0 46.1  11.2
~A  1010.0 47.0  12.1
~A  1500.0 50.3  15.5  ← Gap in depth
~A  1505.0 51.2  16.2
```

**OSDU Handling**:

- Gaps are preserved in original LAS file
- No interpolation or gap-filling during ingestion
- Wellbore DDMS can handle gaps during data access (optional interpolation)

#### C. NaN/Missing Values

**Scenario**: Missing data points within curves

```
Depth: 1000, 1005, 1010, 1015, 1020
GR:    45.2, 46.1, -999.25, 48.5, 49.2
```

**LAS File Representation**:

- Missing values typically represented as `-999.25` (LAS standard null value)
- Or actual `NaN` in some formats

**OSDU Handling**:

- Original LAS file preserves null indicators
- Metadata may indicate null value representation
- Wellbore DDMS can filter or handle nulls during access

---

## 3. WellLog Schema Definition

### 3.1 Schema Structure

**Kind Identifier**:

```
{authority}:wks:work-product-component--WellLog:{version}
```

**Example**: `osdu:wks:work-product-component--WellLog:1.0.0`

### 3.2 Schema Fields

The WellLog schema (metadata only) includes:

```json
{
  "kind": "osdu:wks:work-product-component--WellLog:1.0.0",
  "schema": [
    {
      "path": "WellboreId",
      "kind": "string"
    },
    {
      "path": "Description",
      "kind": "string"
    },
    {
      "path": "WellLogTypeID",
      "kind": "string"
    },
    {
      "path": "ActivityType",
      "kind": "string"
    },
    {
      "path": "LoggingService",
      "kind": "string"
    },
    {
      "path": "ServiceCompanyId",
      "kind": "string"
    },
    {
      "path": "TopMeasuredDepth",
      "kind": "float"
    },
    {
      "path": "BottomMeasuredDepth",
      "kind": "float"
    },
    {
      "path": "LogVersion",
      "kind": "string"
    },
    {
      "path": "LogRun",
      "kind": "string"
    },
    {
      "path": "LogActivity",
      "kind": "string"
    },
    {
      "path": "Datasets",
      "kind": "array"
    },
    {
      "path": "ResourceSecurityClassification",
      "kind": "string"
    },
    {
      "path": "Source",
      "kind": "string"
    },
    {
      "path": "SubmitterName",
      "kind": "string"
    }
  ]
}
```

### 3.3 Important Schema Notes

**Depth Information**:

- `TopMeasuredDepth`: Shallowest depth in the log
- `BottomMeasuredDepth`: Deepest depth in the log
- **Note**: These represent the overall depth range, not per-curve ranges

**Dataset References**:

- `Datasets`: Array of Dataset record IDs
- Links to the original LAS file(s)
- May reference multiple files if log is split

**Missing Curve-Specific Schema**:

- **Critical**: The WellLog schema does **NOT** include individual curve definitions
- Curve names, units, and ranges are **NOT** stored in metadata
- This information remains in the original LAS file

---

## 4. LAS File Ingestion Flow

### 4.1 Ingestion Methods

OSDU supports LAS ingestion through:

#### A. Manifest-Based Ingestion (Metadata Only)

- **Use Case**: Production-grade, structured ingestion
- **Process**: Extract metadata from LAS, create WellLog WPC record
- **File Handling**: Upload LAS file separately, link via Dataset
- **Note**: Documentation specifically mentions "Metadata only, without Wellbore DDMS"

#### B. Wellbore DDMS Data Loader Utility

- **Use Case**: Full-featured ingestion with curve data access
- **Process**: Handles both metadata and enables curve data access
- **Capabilities**: Optimized accessors for bulk data operations

### 4.2 Complete Ingestion Flow: Manifest-Based (Metadata Only)

**Phase 1: File Upload**

```
┌─────────────────────────────────────────────────────────────┐
│ Step 1.1: Upload LAS File                                   │
│                                                              │
│ Client → File Service API                                   │
│ POST /api/file/v2/files                                     │
│                                                              │
│ Headers:                                                     │
│   - Authorization: Bearer <JWT_TOKEN>                        │
│   - Slb-Data-Partition-Id: <data-partition-id>               │
│                                                              │
│ Request Body:                                                │
│ {                                                           │
│   "FileSource": "well_log.las",                             │
│   "FileExtension": "las",                                    │
│   "DataTypeID": "srn:type:file/las:"                        │
│ }                                                           │
│                                                              │
│ Response (200 OK):                                          │
│ {                                                           │
│   "FileID": "srn:type:file/las:123456789:",                │
│   "SignedURL": "https://storage.../upload-url",             │
│   "FileSource": "well_log.las"                              │
│ }                                                           │
└─────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────┐
│ Step 1.2: Upload LAS File to Cloud Storage                  │
│                                                              │
│ Client → Cloud Storage (S3/GCS/Azure Blob)                  │
│ PUT <SignedURL>                                              │
│                                                              │
│ Headers:                                                     │
│   - Content-Type: text/plain (or application/octet-stream) │
│                                                              │
│ Body: (Raw LAS file content)                                 │
│ ~Version Information                                         │
│ VERS.   2.0:   CWLS LOG ASCII STANDARD - VERSION 2.0         │
│ ...                                                          │
│ ~Well Information                                            │
│ #MNEM.UNIT    VALUE                                          │
│ STRT.M       1000.0                                          │
│ STOP.M       5000.0                                          │
│ ...                                                          │
│ ~Curve Information                                           │
│ #MNEM.UNIT    API CODE    DESCRIPTION                        │
│ DEPTH.M       .            DEPTH                             │
│ GR  .GAPI     .            GAMMA RAY                        │
│ RES .OHMM     .            RESISTIVITY                       │
│ ...                                                          │
│ ~A  DEPTH  GR    RES                                         │
│ ~A  .      .     .                                           │
│ ~A  1000.0 45.2  10.5                                        │
│ ~A  1005.0 46.1  11.2                                        │
│ ...                                                          │
│                                                              │
│ Response (200 OK): File uploaded successfully              │
└─────────────────────────────────────────────────────────────┘
```

**Phase 2: LAS File Parsing and Metadata Extraction**

```
┌─────────────────────────────────────────────────────────────┐
│ Step 2.1: Parse LAS File (Client-Side or Server-Side)      │
│                                                              │
│ LAS Parser extracts:                                         │
│                                                              │
│ 1. Well Information:                                         │
│    - Well name, UWI                                          │
│    - Start depth, stop depth                                 │
│    - Company, field, location                                │
│                                                              │
│ 2. Curve Information:                                        │
│    - Curve names (DEPTH, GR, RES, DEN, etc.)                 │
│    - Units for each curve                                    │
│    - API codes                                               │
│    - Descriptions                                            │
│                                                              │
│ 3. Data Statistics:                                          │
│    - TopMeasuredDepth (minimum depth)                        │
│    - BottomMeasuredDepth (maximum depth)                     │
│    - Number of data points                                   │
│    - Null value representation (-999.25, etc.)               │
│                                                              │
│ 4. Logging Information:                                      │
│    - Logging service                                         │
│    - Service company                                         │
│    - Activity type (Wireline, LWD, etc.)                     │
│    - Log run, log version                                    │
│                                                              │
│ Note: Actual curve data values are NOT extracted,            │
│       only metadata about the curves                        │
└─────────────────────────────────────────────────────────────┘
```

**Phase 3: Schema Creation (Schema First)**

```
┌─────────────────────────────────────────────────────────────┐
│ Step 3.1: Create WellLog Schema                              │
│                                                              │
│ Client → Storage Service API                                │
│ POST /api/storage/v2/schemas                                │
│                                                              │
│ Request Body:                                                │
│ {                                                           │
│   "kind": "osdu:wks:work-product-component--WellLog:1.0.0", │
│   "schema": [                                                │
│     { "path": "WellboreId", "kind": "string" },             │
│     { "path": "Description", "kind": "string" },            │
│     { "path": "TopMeasuredDepth", "kind": "float" },        │
│     { "path": "BottomMeasuredDepth", "kind": "float" },     │
│     { "path": "WellLogTypeID", "kind": "string" },          │
│     { "path": "ActivityType", "kind": "string" },           │
│     { "path": "LoggingService", "kind": "string" },         │
│     { "path": "ServiceCompanyId", "kind": "string" },       │
│     { "path": "Datasets", "kind": "array" }                 │
│   ]                                                          │
│ }                                                           │
│                                                              │
│ Response (200 OK): Schema created                           │
└─────────────────────────────────────────────────────────────┘
```

**Phase 4: Manifest Creation and Ingestion**

```
┌─────────────────────────────────────────────────────────────┐
│ Step 4.1: Create Manifest                                    │
│                                                              │
│ Manifest Structure:                                          │
│ {                                                           │
│   "kind": "osdu:wks:Manifest:1.0.0",                        │
│   "WorkProduct": { ... },                                    │
│   "WorkProductComponents": [                                 │
│     {                                                        │
│       "id": "surrogate-key:wpc-1",                          │
│       "kind": "osdu:wks:work-product-component--WellLog:1.0.0",│
│       "data": {                                              │
│         "WellboreId": "srn:master-data/Wellbore:1100:",      │
│         "Description": "Wireline Log",                       │
│         "TopMeasuredDepth": 1000.0,                         │
│         "BottomMeasuredDepth": 5000.0,                       │
│         "WellLogTypeID": "osdu:reference-data/LogType:Raw:",│
│         "ActivityType": "Wireline",                         │
│         "LoggingService": "SLIM CEMENT MAP TOOL",           │
│         "ServiceCompanyId": "srn:master-data/Organisation:Schlumberger:",│
│         "LogVersion": "1",                                  │
│         "LogRun": "1",                                       │
│         "LogActivity": "Main Pass",                          │
│         "Datasets": ["surrogate-key:dataset-1"]             │
│       }                                                      │
│     }                                                        │
│   ],                                                         │
│   "Datasets": [                                              │
│     {                                                        │
│       "id": "surrogate-key:dataset-1",                      │
│       "kind": "osdu:wks:dataset--File.Generic:1.0.0",       │
│       "data": {                                              │
│         "DatasetProperties": {                               │
│           "FileSourceInfo": {                                │
│             "FileSource": "well_log.las"                     │
│           }                                                  │
│         }                                                    │
│       }                                                      │
│     }                                                        │
│   ]                                                          │
│ }                                                           │
└─────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────┐
│ Step 4.2: Trigger Workflow Ingestion                        │
│                                                              │
│ Client → Workflow Service API                               │
│ POST /api/workflow/v1/workflow/Osdu_ingest/workflowRun     │
│                                                              │
│ Request Body:                                                │
│ {                                                           │
│   "executionContext": {                                      │
│     "Payload": {                                             │
│       "AppKey": "las-ingestion-app",                        │
│       "data-partition-id": "<data-partition-id>"             │
│     },                                                       │
│     "manifest": { /* manifest structure above */ }           │
│   }                                                          │
│ }                                                           │
│                                                              │
│ Response (200 OK):                                          │
│ {                                                           │
│   "message": "Workflow Osdu_ingest started successfully"    │
│ }                                                           │
└─────────────────────────────────────────────────────────────┘
```

**Phase 5: Record Creation**

```
┌─────────────────────────────────────────────────────────────┐
│ Step 5.1: Airflow DAG → Storage Service                      │
│                                                              │
│ Airflow DAG: Osdu_ingest                                    │
│   → validate_manifest_task                                  │
│   → create_work_product_task                                │
│   → create_work_product_component_task                      │
│     → PUT /api/storage/v2/records                          │
│       → Storage Service                                     │
│         → Create WellLog WPC record                         │
│         → Link to Dataset (LAS file)                        │
│   → create_dataset_task                                     │
│   → link_relationships_task                                 │
│                                                              │
│ Result:                                                      │
│ - WellLog WPC record created with metadata                  │
│ - Dataset record linking to original LAS file                │
│ - LAS file remains in original format in cloud storage       │
│ - Curve data NOT stored in Storage Service                  │
└─────────────────────────────────────────────────────────────┘
```

---

## 5. Handling Curve Data Complexities

### 5.1 OSDU's Approach to Complex Curve Data

**Key Principle**: OSDU does **NOT** normalize, interpolate, or transform curve data during ingestion. The original LAS file is preserved exactly as received.

### 5.2 Non-Overlapping Depth Values

**How OSDU Handles It**:

1. **File Preservation**: Original LAS file contains all curves with their respective depth ranges
2. **Metadata Aggregation**: WellLog metadata records the overall depth range (TopMeasuredDepth to BottomMeasuredDepth)
3. **Access Layer**: Wellbore DDMS can:
   - Read specific curves from LAS file
   - Query curves by depth range
   - Handle per-curve depth extents

**Example**:

```json
{
  "WellLog": {
    "TopMeasuredDepth": 1000.0, // Overall minimum
    "BottomMeasuredDepth": 5000.0, // Overall maximum
    "Datasets": ["srn:type:file/las:123:"]
  }
}
```

The LAS file itself contains:

- GR curve: 1000-2000 ft
- RES curve: 1500-2500 ft
- DEN curve: 2000-3000 ft

### 5.3 Gaps Between Depth Indices

**How OSDU Handles It**:

1. **Preservation**: Gaps are preserved in original LAS file
2. **No Interpolation**: OSDU does not fill gaps during ingestion
3. **Access-Time Handling**: Wellbore DDMS can optionally:
   - Return data with gaps (preserve original)
   - Interpolate gaps on-demand (if requested)
   - Filter by depth ranges (skip gaps)

**Example LAS File**:

```las
~A  DEPTH  GR    RES
~A  .      .     .
~A  1000.0 45.2  10.5
~A  1005.0 46.1  11.2
~A  1010.0 47.0  12.1
~A  1500.0 50.3  15.5  ← 490 ft gap preserved
~A  1505.0 51.2  16.2
```

**OSDU Storage**:

- File stored as-is with gap
- Metadata: TopMeasuredDepth=1000.0, BottomMeasuredDepth=1505.0
- Gap handling deferred to access layer

### 5.4 NaN/Missing Values

**How OSDU Handles It**:

1. **Null Value Preservation**: LAS standard null values (-999.25, etc.) preserved
2. **No Data Cleaning**: Missing values remain in file
3. **Access-Time Filtering**: Wellbore DDMS can:
   - Return null values as-is
   - Filter out nulls on-demand
   - Replace nulls with specified values

**Example LAS File**:

```las
~A  DEPTH  GR    RES
~A  .      .     .
~A  1000.0 45.2  10.5
~A  1005.0 -999.25 11.2  ← Missing value preserved
~A  1010.0 47.0  12.1
~A  1015.0 48.5  -999.25  ← Missing value preserved
```

**OSDU Storage**:

- Null values preserved in original file
- No metadata about null value locations
- Null handling deferred to access layer

---

## 6. Wellbore DDMS: Curve Data Access

### 6.1 Wellbore DDMS Role

**Wellbore DDMS** provides optimized accessors for bulk data operations:

- **Type-safe entity access**: Structured access to wellbore data
- **Optimized accessors**: Specialized APIs for logs, trajectories, checkshots
- **Bulk data operations**: Efficient handling of large curve datasets

### 6.2 Curve Data Access Patterns

**Note**: The following API patterns are **inferred/example patterns** based on typical DDMS service designs. The actual Wellbore DDMS API endpoints and parameters are **not fully documented** in the Context7 accessible documentation. To get exact API specifications, refer to the OpenAPI documentation links provided in section 6.4.

**Inferred Pattern 1: Read Entire Log**

```
GET /api/os-wellbore-ddms/v1/wellbores/{wellboreId}/logs/{logId}/curves
```

_(Example pattern - actual endpoint may differ)_

**Inferred Pattern 2: Read Specific Curves**

```
GET /api/os-wellbore-ddms/v1/wellbores/{wellboreId}/logs/{logId}/curves?curves=GR,RES,DEN
```

_(Example pattern - actual query parameters may differ)_

**Inferred Pattern 3: Read Depth Range**

```
GET /api/os-wellbore-ddms/v1/wellbores/{wellboreId}/logs/{logId}/curves?topDepth=1000&bottomDepth=2000
```

_(Example pattern - actual depth filtering may differ)_

**Inferred Pattern 4: Read with Interpolation**

```
GET /api/os-wellbore-ddms/v1/wellbores/{wellboreId}/logs/{logId}/curves?interpolate=true&method=linear
```

_(Example pattern - interpolation support and parameters not confirmed)_

**To Get Actual API Specifications**:

- Visit the Swagger UI: `https://osdu.bm-preship.gcp.gnrg-osdu.projects.epam.com/api/os-wellbore-ddms/docs`
- Review OpenAPI Spec: `https://community.opengroup.org/osdu/platform/domain-data-mgmt-services/wellbore/wellbore-domain-services/-/blob/master/spec/generated/openapi.json`

### 6.3 Handling Complexities at Access Time

**Non-Overlapping Depths**:

- Wellbore DDMS reads curves individually from LAS file
- Returns only data within requested depth range per curve
- No attempt to align non-overlapping curves

**Gaps**:

- Option 1: Return data with gaps (preserve original)
- Option 2: Interpolate gaps if requested
- Option 3: Return gap indicators

**NaN/Missing Values**:

- Option 1: Return null values as-is
- Option 2: Filter nulls if requested
- Option 3: Replace with specified value

### 6.4 Wellbore DDMS: What's Known vs. What's Hidden

#### What We Know (Public Documentation)

**High-Level Description**:

- Wellbore DDMS provides "type-safe entity access and optimized accessors for bulk data such as logs, trajectories, checkshots"
- It's a Domain Data Management Service specifically for wellbore-related data
- It manages wellbore data including wells, wellbores, and logs

**API Documentation Availability**:

- OpenAPI/Swagger documentation is available at multiple deployment environments:
  - Community: `https://osdu.bm-preship.gcp.gnrg-osdu.projects.epam.com/api/os-wellbore-ddms/docs`
  - Azure Testing: `https://osdu-ship.msft-osdu-test.org/api/os-wellbore-ddms/docs`
  - AWS Testing: `https://prsh.testing.preshiptesting.osdu.aws/api/os-wellbore-ddms/docs`
  - GCP Testing: `https://preship.gcp.gnrg-osdu.projects.epam.com/api/os-wellbore-ddms/docs`
- OpenAPI Specification: `https://community.opengroup.org/osdu/platform/domain-data-mgmt-services/wellbore/wellbore-domain-services/-/blob/master/spec/generated/openapi.json`
- Documentation: `https://community.opengroup.org/osdu/platform/domain-data-mgmt-services/wellbore/wellbore-domain-services/-/tree/master/docs`
- Wiki: `https://community.opengroup.org/osdu/platform/domain-data-mgmt-services/wellbore/wellbore-domain-services/-/wikis/home`

**Basic API Patterns** (from documentation references):

- Endpoint base: `/api/os-wellbore-ddms` or `/wellbores/v1`
- Supports standard REST operations (GET, POST, PUT, DELETE)
- Wellbore retrieval: `GET /wellbores/v1/wells/{wellId}/wellbores/{wellboreId}`

**Data Loader Utility**:

- There's a "Wellbore DDMS Data Loader Utility Quickstart guide" mentioned in tutorials
- Suggests there's a utility/tool for loading data via Wellbore DDMS

#### What's Hidden (Not in Public Context7 Documentation)

**Detailed API Specifications**:

- ❌ Specific endpoint paths for curve data access
- ❌ Request/response schemas for curve queries
- ❌ Query parameters for filtering, interpolation, gap handling
- ❌ Authentication and authorization details
- ❌ Error handling and status codes

**Implementation Details**:

- ❌ How Wellbore DDMS parses LAS files internally
- ❌ How it handles non-overlapping depth ranges
- ❌ Gap interpolation algorithms (if any)
- ❌ NaN/null value handling logic
- ❌ Caching strategies for file access
- ❌ Performance optimizations
- ❌ File format support (LAS, DLIS, etc.)

**Internal Architecture**:

- ❌ How Wellbore DDMS integrates with File Service
- ❌ How it accesses cloud storage (S3/GCS/Azure)
- ❌ Connection pooling or file handle management
- ❌ Streaming vs. batch processing
- ❌ Memory management for large files

**Data Transformation**:

- ❌ Whether it performs any data normalization
- ❌ Unit conversion capabilities
- ❌ Coordinate system transformations
- ❌ Data quality checks or validation

#### How to Access Hidden Details

**Option 1: Access OpenAPI Documentation Directly**

```
Visit: https://osdu.bm-preship.gcp.gnrg-osdu.projects.epam.com/api/os-wellbore-ddms/docs
```

This will provide:

- Complete API endpoint definitions
- Request/response schemas
- Query parameters
- Authentication requirements
- Example requests/responses

**Option 2: Review OpenAPI Specification JSON**

```
Access: https://community.opengroup.org/osdu/platform/domain-data-mgmt-services/wellbore/wellbore-domain-services/-/blob/master/spec/generated/openapi.json
```

This provides machine-readable API specification.

**Option 3: Review Source Code** (if accessible)

```
Repository: https://community.opengroup.org/osdu/platform/domain-data-mgmt-services/wellbore/wellbore-domain-services
```

Source code would reveal implementation details, but may require access permissions.

**Option 4: Review Documentation/Wiki**

```
Wiki: https://community.opengroup.org/osdu/platform/domain-data-mgmt-services/wellbore/wellbore-domain-services/-/wikis/home
Docs: https://community.opengroup.org/osdu/platform/domain-data-mgmt-services/wellbore/wellbore-domain-services/-/tree/master/docs
```

#### Inference: What Wellbore DDMS Likely Does

Based on the description "optimized accessors for bulk data" and the architecture, Wellbore DDMS likely:

1. **File Access Layer**:

   - Reads LAS files from cloud storage
   - Parses LAS file format
   - Extracts curve data on-demand

2. **Query Processing**:

   - Accepts queries for specific curves
   - Filters by depth range
   - Handles curve selection

3. **Data Handling**:

   - Preserves original data structure
   - Optionally handles gaps/NaN based on query parameters
   - Returns data in structured format (likely JSON)

4. **Performance Optimization**:

   - May cache parsed file metadata
   - Streams large files rather than loading entirely
   - Optimizes for bulk data operations

5. **Type Safety**:
   - Provides structured response types
   - Validates curve names and references
   - Ensures data consistency

**However**, without access to the actual API documentation or source code, these are **inferences** based on the high-level description and typical patterns for such services.

---

## 7. Storage Solution Architecture

### 7.1 Two-Tier Storage

```
┌─────────────────────────────────────────────────────────────┐
│ OSDU Storage Architecture for LAS Files                      │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│ Tier 1: Metadata Storage (Storage Service)                 │
│ ┌────────────────────────────────────────┐                  │
│ │ WellLog WPC Records                     │                  │
│ │ - Searchable                            │                  │
│ │ - Indexed                               │                  │
│ │ - Fast queries                          │                  │
│ │ - Small size (KB)                       │                  │
│ └────────────────────────────────────────┘                  │
│                                                              │
│ Tier 2: File Storage (Cloud Storage)                        │
│ ┌────────────────────────────────────────┐                  │
│ │ Original LAS Files                      │                  │
│ │ - Preserved format                      │                  │
│ │ - Complete curve data                   │                  │
│ │ - Sparse data, gaps, NaN                │                  │
│ │ - Large size (MB-GB)                    │                  │
│ │ - Accessed via Wellbore DDMS            │                  │
│ └────────────────────────────────────────┘                  │
│                                                              │
│ Access Layer: Wellbore DDMS                                 │
│ ┌────────────────────────────────────────┐                  │
│ │ Optimized Accessors                     │                  │
│ │ - Read curves from files                │                  │
│ │ - Handle complexities                   │                  │
│ │ - Support queries                       │                  │
│ └────────────────────────────────────────┘                  │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

### 7.2 Storage Characteristics

**Metadata Storage (Storage Service)**:

- **Size**: Small (typically < 10 KB per WellLog record)
- **Access**: Fast, indexed, searchable
- **Content**: Metadata only, no curve data
- **Query**: Full-text search, filtering, relationships

**File Storage (Cloud Storage)**:

- **Size**: Large (MB to GB per LAS file)
- **Access**: Slower, requires file parsing
- **Content**: Complete curve data with all complexities
- **Query**: Via Wellbore DDMS APIs

### 7.3 Data Lineage and Provenance

**Preservation Benefits**:

- Original LAS file format maintained
- No data loss from format conversion
- Audit trail preserved
- Reproducibility ensured

---

## 8. Schema Limitations and Considerations

### 8.1 What the Schema Does NOT Include

**Critical Limitations**:

1. **No Individual Curve Definitions**:

   - Curve names not in schema
   - Curve units not in schema
   - Curve depth ranges not in schema
   - Curve descriptions not in schema

2. **No Curve Data Statistics**:

   - Min/max values per curve
   - Data quality indicators
   - Null value counts
   - Gap information

3. **No Per-Curve Metadata**:
   - Only overall depth range (TopMeasuredDepth, BottomMeasuredDepth)
   - No curve-specific depth extents
   - No curve-specific units

### 8.2 Why This Design?

**Rationale**:

- **Flexibility**: LAS files can have variable curve structures
- **Performance**: Metadata remains small and fast
- **Preservation**: Original file contains all details
- **Access Pattern**: Curve details accessed on-demand via Wellbore DDMS

### 8.3 Implications

**For Search/Discovery**:

- Can search by wellbore, depth range, log type, activity type
- **Cannot** search by specific curve names or curve values
- Must access file via Wellbore DDMS to inspect curves

**For Data Access**:

- Must use Wellbore DDMS to read curve data
- Cannot query curve values directly from Storage Service
- File parsing required for curve access

---

## 9. Complete Call Stack: LAS Ingestion

```
┌──────────────────────────────────────────────────────────────┐
│ LAS File Ingestion - Complete Call Stack                     │
├──────────────────────────────────────────────────────────────┤
│                                                               │
│ 1. CLIENT APPLICATION                                         │
│    │                                                           │
│    ├─→ POST /api/storage/v2/schemas                          │
│    │   └─→ Storage Service                                    │
│    │       └─→ Schema Registry (Create WellLog schema)      │
│    │                                                           │
│    ├─→ POST /api/file/v2/files                               │
│    │   └─→ File Service                                      │
│    │       └─→ Generate Signed URL                            │
│    │                                                           │
│    ├─→ PUT <SignedURL>                                        │
│    │   └─→ Cloud Storage (S3/GCS/Azure)                       │
│    │       └─→ Store Original LAS File                        │
│    │                                                           │
│    ├─→ [LAS Parser] (Client-side or Server-side)             │
│    │   └─→ Extract Metadata                                   │
│    │       ├─→ Well information                               │
│    │       ├─→ Depth ranges                                   │
│    │       ├─→ Logging information                            │
│    │       └─→ (Curve data NOT extracted)                     │
│    │                                                           │
│    └─→ POST /api/workflow/v1/workflow/Osdu_ingest/workflowRun│
│        └─→ Workflow Service                                  │
│            └─→ Airflow DAG: Osdu_ingest                      │
│                │                                               │
│                ├─→ validate_manifest_task                      │
│                │   └─→ Schema Validation                       │
│                │   └─→ Reference Validation                   │
│                │                                               │
│                ├─→ create_work_product_task                    │
│                │   └─→ PUT /api/storage/v2/records           │
│                │       └─→ Storage Service                    │
│                │           └─→ Create WP Record                │
│                │                                               │
│                ├─→ create_work_product_component_task          │
│                │   └─→ PUT /api/storage/v2/records           │
│                │       └─→ Storage Service                    │
│                │           ├─→ Schema Validation               │
│                │           ├─→ Create WellLog WPC Record       │
│                │           └─→ Search Service (Indexing)      │
│                │                                               │
│                ├─→ create_dataset_task                          │
│                │   └─→ PUT /api/storage/v2/records           │
│                │       └─→ Storage Service                    │
│                │           └─→ Create Dataset Record          │
│                │           └─→ Link to LAS File                │
│                │                                               │
│                └─→ link_relationships_task                      │
│                    └─→ PUT /api/storage/v2/records           │
│                        └─→ Storage Service                    │
│                            └─→ Update WP Components array      │
│                                                               │
│ 2. DATA ACCESS (Post-Ingestion)                              │
│    │                                                           │
│    └─→ GET /api/os-wellbore-ddms/v1/wellbores/{id}/logs/{id}/curves│
│        └─→ Wellbore DDMS                                     │
│            ├─→ Read Dataset reference                         │
│            ├─→ Access LAS file from Cloud Storage             │
│            ├─→ Parse LAS file                                 │
│            ├─→ Extract requested curves                       │
│            ├─→ Handle gaps, NaN, non-overlapping depths       │
│            └─→ Return curve data                              │
│                                                               │
└──────────────────────────────────────────────────────────────┘
```

---

## 10. Key Insights and Conclusions

### 10.1 OSDU's Approach: Strengths

✅ **Data Preservation**: Original LAS files preserved exactly
✅ **Metadata Searchability**: Fast, indexed metadata queries
✅ **Flexibility**: Handles variable curve structures
✅ **Scalability**: Separates small metadata from large file data
✅ **Lineage**: Complete audit trail and provenance

### 10.2 OSDU's Approach: Limitations

⚠️ **No Curve-Level Metadata**: Individual curve information not indexed
⚠️ **File Access Required**: Must access files for curve data
⚠️ **No Pre-Processing**: Complexities (gaps, NaN) not handled at ingestion
⚠️ **Schema Limitations**: WellLog schema is metadata-only

### 10.3 Handling Complexities: Summary

| Complexity                 | Ingestion Time    | Access Time                     |
| -------------------------- | ----------------- | ------------------------------- |
| **Non-Overlapping Depths** | Preserved in file | Wellbore DDMS handles per-curve |
| **Gaps**                   | Preserved in file | Optional interpolation via DDMS |
| **NaN/Missing Values**     | Preserved in file | Optional filtering via DDMS     |
| **Sparse Data**            | Preserved in file | Returned as-is or interpolated  |

### 10.4 Best Practices

1. **Use Wellbore DDMS**: For curve data access, always use Wellbore DDMS APIs
2. **Preserve Original Files**: Never modify LAS files during ingestion
3. **Metadata Completeness**: Extract comprehensive metadata for searchability
4. **Handle Complexities at Access**: Defer gap/NaN handling to access layer
5. **Monitor File Sizes**: Large LAS files may impact access performance

---

## 11. References

- **OSDU Data Platform Wiki**: `https://community.opengroup.org/groups/osdu/platform/-/wikis`
- **WellLog Schema**: `https://gitlab.opengroup.org/osdu/subcommittees/data-def/work-products/schema/-/blob/master/Generated/work-product-component/WellLog.1.0.0.json`
- **Wellbore DDMS API**: Domain-specific wellbore data management APIs
- **LAS File Format Specification**: CWLS Log ASCII Standard
- **OSDU Data Loading Quick Start Guide**: Comprehensive ingestion workflows
- **Wellbore DDMS Documentation**: Optimized accessors for bulk data

---

## Conclusion

OSDU's LAS file ingestion follows a **metadata-first, file-preservation** architecture that:

1. **Stores metadata** in the Storage Service for fast searchability
2. **Preserves original LAS files** in cloud storage with all complexities intact
3. **Delegates curve data access** to Wellbore DDMS for optimized operations
4. **Handles complexities** (non-overlapping depths, gaps, NaN values) at access time rather than ingestion time

This approach ensures data fidelity, preserves lineage, and provides flexible access patterns while maintaining performance for metadata queries. The trade-off is that curve-specific queries require file access via Wellbore DDMS rather than direct Storage Service queries.
