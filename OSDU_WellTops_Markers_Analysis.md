# OSDU Well Tops/Markers Analysis: Architecture, Schema, and Ingestion Flow

## Executive Summary

This document provides a comprehensive analysis of how well tops/markers (WellboreMarkerSet) are designed, stored, processed, and ingested in the OSDU (Open Subsurface Data Universe) platform. It covers the complete architecture, schema definitions, and detailed ingestion workflows with call stacks.

---

## 1. Well Tops/Markers Architecture

### 1.1 Conceptual Overview

**Well Tops** (also known as **Wellbore Markers** or **Formation Tops**) are geological markers that identify specific formations, zones, or horizons encountered during drilling operations. In OSDU, these are represented as **WellboreMarkerSet** entities, which are Work Product Components (WPC) that group multiple markers together.

### 1.2 OSDU Data Model Hierarchy

WellboreMarkerSet fits into the OSDU data model hierarchy as follows:

```
┌─────────────────────────────────────────┐
│ Master Data                             │
│ - Well                                  │
│ - Wellbore                              │
└──────────────┬──────────────────────────┘
               │
               ▼
┌─────────────────────────────────────────┐
│ Work Product (WP)                       │
│ - Groups related WPCs                   │
│ - Example: WellboreMarkerSet WP         │
└──────────────┬──────────────────────────┘
               │
               ▼
┌─────────────────────────────────────────┐
│ Work Product Component (WPC)            │
│ - WellboreMarkerSet                      │
│ - Contains array of Markers             │
│ - Each Marker has:                       │
│   • MarkerName                           │
│   • MarkerMeasuredDepth                  │
└──────────────┬──────────────────────────┘
               │
               ▼
┌─────────────────────────────────────────┐
│ Dataset                                 │
│ - References source files (CSV, etc.)   │
│ - Links to WPC                          │
└─────────────────────────────────────────┘
```

### 1.3 Key Architectural Components

1. **Work Product (WP)**: Container that groups one or more WellboreMarkerSet WPCs
2. **Work Product Component (WPC)**: The actual WellboreMarkerSet record containing marker data
3. **Dataset**: References to source files (typically CSV files containing marker data)
4. **Master Data References**: Links to Wellbore entities
5. **Reference Data**: Unit of Measure references for depth units

### 1.4 Design Principles

- **Schema First**: WellboreMarkerSet requires schema definition before ingestion
- **Work Product Pattern**: Follows OSDU Work Product/Work Product Component pattern
- **File Association**: Can be associated with source files (CSV) via Dataset records
- **Unit Standardization**: Uses OSDU Unit of Measure reference data for depth units
- **Wellbore Association**: Each marker set is associated with a specific Wellbore

---

## 2. WellboreMarkerSet Schema Definition

### 2.1 Schema Structure

WellboreMarkerSet is defined as a Work Product Component with the following schema structure:

**Kind Identifier Pattern**:
```
{authority}:wks:work-product-component--WellboreMarkerSet:{version}
```

**Example**: `osdu:wks:work-product-component--WellboreMarkerSet:1.0.0`

### 2.2 Schema Fields

#### 2.2.1 Work Product Component Schema

```json
{
  "kind": "osdu:wks:work-product-component--WellboreMarkerSet:1.0.0",
  "schema": [
    {
      "path": "ResourceTypeID",
      "kind": "string"
    },
    {
      "path": "ResourceSecurityClassification",
      "kind": "string"
    },
    {
      "path": "Data.GroupTypeProperties.Files",
      "kind": "array"
    },
    {
      "path": "Data.GroupTypeProperties.Artefacts",
      "kind": "array"
    },
    {
      "path": "Data.IndividualTypeProperties.Name",
      "kind": "string"
    },
    {
      "path": "Data.IndividualTypeProperties.Description",
      "kind": "string"
    },
    {
      "path": "Data.IndividualTypeProperties.WellboreID",
      "kind": "string"
    },
    {
      "path": "Data.IndividualTypeProperties.DepthUnit",
      "kind": "string"
    },
    {
      "path": "Data.IndividualTypeProperties.Markers",
      "kind": "array"
    },
    {
      "path": "Data.IndividualTypeProperties.Markers[].MarkerName",
      "kind": "string"
    },
    {
      "path": "Data.IndividualTypeProperties.Markers[].MarkerMeasuredDepth",
      "kind": "float"
    },
    {
      "path": "ComponentsAssociativeIDs",
      "kind": "array"
    },
    {
      "path": "ResourceID",
      "kind": "string"
    }
  ]
}
```

### 2.3 Data Structure Details

#### 2.3.1 Individual Type Properties

The core marker data is stored in `Data.IndividualTypeProperties`:

- **Name** (string): Name of the marker set (often matches source file name)
- **Description** (string): Description of the marker set
- **WellboreID** (string): Reference to the associated Wellbore (SRN format)
- **DepthUnit** (string): Unit of Measure reference for depth (SRN format)
  - Example: `srn:reference-data/UnitOfMeasure:M:` (Meters)
- **Markers** (array): Array of marker objects
  - **MarkerName** (string): Name of the geological marker/formation
  - **MarkerMeasuredDepth** (float): Measured depth of the marker

#### 2.3.2 Group Type Properties

- **Files** (array): Array of file references (SRN format)
  - Example: `srn:type:file/csv:66153425987012541:`
- **Artefacts** (array): Array of artifact references

#### 2.3.3 Resource References

- **ResourceTypeID**: Type identifier for the resource
  - Example: `srn:type:work-product-component/WellboreMarker:`
- **ResourceSecurityClassification**: Security classification reference
  - Example: `srn:reference-data/ResourceSecurityClassification:RESTRICTED:`
- **ResourceID**: Unique resource identifier (SRN format)

### 2.4 Example Marker Data Structure

```json
{
  "Markers": [
    {
      "MarkerName": "QUATER. UNDIFF.",
      "MarkerMeasuredDepth": 0.0
    },
    {
      "MarkerName": "Breda Formation",
      "MarkerMeasuredDepth": 133.0
    },
    {
      "MarkerName": "Rupel Formation",
      "MarkerMeasuredDepth": 215.0
    },
    {
      "MarkerName": "Brussels Sand Member",
      "MarkerMeasuredDepth": 307.0
    },
    {
      "MarkerName": "Ieper Member",
      "MarkerMeasuredDepth": 410.0
    },
    {
      "MarkerName": "Basal Dongen Tuffite Member",
      "MarkerMeasuredDepth": 573.0
    },
    {
      "MarkerName": "Landen Clay Member",
      "MarkerMeasuredDepth": 594.0
    },
    {
      "MarkerName": "Ommelanden Formation",
      "MarkerMeasuredDepth": 608.0
    }
  ]
}
```

---

## 3. WellboreMarkerSet Storage Architecture

### 3.1 Storage Service Integration

WellboreMarkerSet records are stored using the OSDU Storage Service API, following the standard OSDU record structure:

```json
{
  "id": "opendes:doc:e876f8a2963e4a1b9886e11f9b6634cb",
  "version": 1582725123640845,
  "kind": "opendes:osdu:wellmarker-wp:2.2.0",
  "acl": {
    "viewers": ["data.default.viewers@..."],
    "owners": ["data.default.owners@..."]
  },
  "legal": {
    "legaltags": ["..."],
    "otherRelevantDataCountries": ["US"],
    "status": "compliant"
  },
  "data": {
    // WellboreMarkerSet data structure
  }
}
```

### 3.2 Record Relationships

WellboreMarkerSet records maintain relationships with:

1. **Work Product**: Parent WP that groups the marker set
2. **Wellbore**: Master data reference via WellboreID
3. **Dataset**: File references via Files array
4. **Reference Data**: Unit of Measure via DepthUnit

### 3.3 Schema-First Requirement

**Critical**: The schema for WellboreMarkerSet must be created **before** attempting to ingest any marker records. This follows OSDU's "Schema First" principle.

**Schema Creation Endpoint**:
```bash
POST /api/storage/v2/schemas
```

---

## 4. WellboreMarkerSet Ingestion Methods

### 4.1 Ingestion Approaches

OSDU supports multiple methods for ingesting WellboreMarkerSet data:

#### A. Manifest-Based Ingestion (Recommended)
- **Use Case**: Structured, production-grade ingestion
- **Workflow**: Uses OSDU Manifest schema
- **Endpoint**: `POST /api/workflow/v1/workflow/Osdu_ingest/workflowRun`
- **Advantages**: 
  - Validates data structure
  - Supports Work Product/Work Product Component relationships
  - Handles file associations
  - Batch processing support

#### B. Direct Storage API Ingestion
- **Use Case**: Programmatic ingestion, custom workflows
- **Endpoint**: `PUT /api/storage/v2/records`
- **Advantages**:
  - Direct control over record creation
  - Faster for single records
  - No workflow overhead

#### C. CSV Parser Ingestion
- **Use Case**: CSV file-based marker data
- **Workflow**: CSV Parser DAG
- **Note**: Creates flattened schema (future: R3-style schema support)

### 4.2 Wellbore DDMS Integration

WellboreMarkerSet is part of the **Wellbore DDMS** (Domain Data Management Service), which provides:
- **Type-safe entity access**: Structured access to wellbore-related data
- **Optimized accessors**: Specialized APIs for bulk data operations
- **Domain-specific operations**: Wellbore-specific data management

---

## 5. Complete Ingestion Flow: Manifest-Based Approach

### 5.1 Prerequisites

Before starting WellboreMarkerSet ingestion:

- ✅ Wellbore master data record exists
- ✅ Unit of Measure reference data configured
- ✅ Legal tags created
- ✅ ACL groups configured
- ✅ Schema defined for WellboreMarkerSet kind
- ✅ Source file (CSV) prepared (if using file-based ingestion)

### 5.2 Sample CSV File Structure

**wellbore_markers.csv**:
```csv
MarkerName,MarkerMeasuredDepth
QUATER. UNDIFF.,0.0
Breda Formation,133.0
Rupel Formation,215.0
Brussels Sand Member,307.0
Ieper Member,410.0
Basal Dongen Tuffite Member,573.0
Landen Clay Member,594.0
Ommelanden Formation,608.0
```

### 5.3 Complete Ingestion Flow with Call Stack

**Phase 1: Schema Definition (Schema First)**

```
┌─────────────────────────────────────────────────────────────┐
│ Step 1.1: Create WellboreMarkerSet Schema                   │
│                                                              │
│ Client → Storage Service API                                │
│ POST /api/storage/v2/schemas                                │
│                                                              │
│ Headers:                                                     │
│   - Authorization: Bearer <JWT_TOKEN>                        │
│   - Content-Type: application/json                          │
│   - Slb-Data-Partition-Id: <data-partition-id>              │
│                                                              │
│ Request Body:                                                │
│ {                                                           │
│   "kind": "osdu:wks:work-product-component--WellboreMarkerSet:1.0.0",│
│   "schema": [                                                │
│     { "path": "Data.IndividualTypeProperties.Name",         │
│       "kind": "string" },                                    │
│     { "path": "Data.IndividualTypeProperties.WellboreID",  │
│       "kind": "string" },                                    │
│     { "path": "Data.IndividualTypeProperties.DepthUnit",    │
│       "kind": "string" },                                    │
│     { "path": "Data.IndividualTypeProperties.Markers",      │
│       "kind": "array" },                                     │
│     { "path": "Data.IndividualTypeProperties.Markers[].MarkerName",│
│       "kind": "string" },                                    │
│     { "path": "Data.IndividualTypeProperties.Markers[].MarkerMeasuredDepth",│
│       "kind": "float" }                                      │
│   ]                                                          │
│ }                                                           │
│                                                              │
│ Response (200 OK):                                          │
│ {                                                           │
│   "kind": "osdu:wks:work-product-component--WellboreMarkerSet:1.0.0",│
│   "schema": [...]                                            │
│ }                                                           │
└─────────────────────────────────────────────────────────────┘
```

**Phase 2: File Upload (If Using File-Based Ingestion)**

```
┌─────────────────────────────────────────────────────────────┐
│ Step 2.1: Request Upload URL                                │
│                                                              │
│ Client → File Service API                                   │
│ POST /api/file/v2/files                                     │
│                                                              │
│ Headers:                                                     │
│   - Authorization: Bearer <JWT_TOKEN>                      │
│   - Slb-Data-Partition-Id: <data-partition-id>              │
│                                                              │
│ Request Body:                                                │
│ {                                                           │
│   "FileSource": "wellbore_markers.csv",                     │
│   "FileExtension": "csv",                                    │
│   "DataTypeID": "srn:type:file/csv:"                        │
│ }                                                           │
│                                                              │
│ Response (200 OK):                                          │
│ {                                                           │
│   "FileID": "srn:type:file/csv:66153425987012541:",         │
│   "SignedURL": "https://storage.../upload-url",            │
│   "FileSource": "wellbore_markers.csv"                      │
│ }                                                           │
└─────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────┐
│ Step 2.2: Upload CSV File to Signed URL                    │
│                                                              │
│ Client → Cloud Storage (S3/GCS/Azure Blob)                  │
│ PUT <SignedURL>                                              │
│                                                              │
│ Headers:                                                     │
│   - Content-Type: text/csv                                  │
│                                                              │
│ Body: (Raw CSV file content)                                │
│ MarkerName,MarkerMeasuredDepth                              │
│ QUATER. UNDIFF.,0.0                                         │
│ Breda Formation,133.0                                       │
│ ...                                                          │
│                                                              │
│ Response (200 OK): File uploaded successfully              │
└─────────────────────────────────────────────────────────────┘
```

**Phase 3: Manifest Creation**

```
┌─────────────────────────────────────────────────────────────┐
│ Step 3.1: Create Manifest Structure                         │
│                                                              │
│ The manifest follows OSDU Manifest schema structure:       │
│                                                              │
│ {                                                           │
│   "kind": "osdu:wks:Manifest:1.0.0",                       │
│   "WorkProduct": { ... },                                    │
│   "WorkProductComponents": [ ... ],                         │
│   "Datasets": [ ... ]                                       │
│ }                                                           │
│                                                              │
│ Key Points:                                                  │
│ - Use surrogate-keys for WP, WPC, and Dataset IDs          │
│ - IDs should end with colon (:)                             │
│ - All WPC IDs must be listed in WP Components array         │
└─────────────────────────────────────────────────────────────┘
```

**Phase 4: Workflow Ingestion**

```
┌─────────────────────────────────────────────────────────────┐
│ Step 4.1: Trigger Manifest Ingestion Workflow              │
│                                                              │
│ Client → Workflow Service API                               │
│ POST /api/workflow/v1/workflow/Osdu_ingest/workflowRun     │
│                                                              │
│ Headers:                                                     │
│   - Authorization: Bearer <JWT_TOKEN>                      │
│   - Content-Type: application/json                         │
│   - Slb-Data-Partition-Id: <data-partition-id>              │
│                                                              │
│ Request Body:                                                │
│ {                                                           │
│   "executionContext": {                                      │
│     "Payload": {                                             │
│       "AppKey": "wellbore-marker-ingestion",                │
│       "data-partition-id": "<data-partition-id>"             │
│     },                                                       │
│     "manifest": {                                            │
│       "kind": "osdu:wks:Manifest:1.0.0",                   │
│       "WorkProduct": {                                       │
│         "id": "surrogate-key:wp-1",                        │
│         "kind": "osdu:wks:work-product--WorkProduct:1.0.0",│
│         "acl": { ... },                                      │
│         "legal": { ... },                                    │
│         "data": {                                            │
│           "Name": "Wellbore Marker Set WP",                │
│           "Components": ["surrogate-key:wpc-1:"]            │
│         }                                                    │
│       },                                                      │
│       "WorkProductComponents": [                             │
│         {                                                    │
│           "id": "surrogate-key:wpc-1",                      │
│           "kind": "osdu:wks:work-product-component--WellboreMarkerSet:1.0.0",│
│           "acl": { ... },                                    │
│           "legal": { ... },                                  │
│           "data": {                                          │
│             "ResourceTypeID": "srn:type:work-product-component/WellboreMarker:",│
│             "ResourceSecurityClassification": "srn:reference-data/ResourceSecurityClassification:RESTRICTED:",│
│             "Data": {                                        │
│               "GroupTypeProperties": {                        │
│                 "Files": [                                   │
│                   "srn:type:file/csv:66153425987012541:"     │
│                 ],                                            │
│                 "Artefacts": []                               │
│               },                                              │
│               "IndividualTypeProperties": {                   │
│                 "Name": "wellbore_markers.csv",              │
│                 "Description": "Wellbore Marker Set",        │
│                 "WellboreID": "srn:master-data/Wellbore:1100:",│
│                 "DepthUnit": "srn:reference-data/UnitOfMeasure:M:",│
│                 "Markers": [                                 │
│                   {                                           │
│                     "MarkerName": "QUATER. UNDIFF.",         │
│                     "MarkerMeasuredDepth": 0.0                │
│                   },                                          │
│                   {                                           │
│                     "MarkerName": "Breda Formation",          │
│                     "MarkerMeasuredDepth": 133.0               │
│                   }                                           │
│                   // ... more markers                         │
│                 ]                                             │
│               }                                               │
│             }                                                 │
│           }                                                   │
│         }                                                     │
│       ],                                                       │
│       "Datasets": [                                           │
│         {                                                     │
│           "id": "surrogate-key:dataset-1",                  │
│           "kind": "osdu:wks:dataset--File.Generic:1.0.0",  │
│           "data": {                                           │
│             "DatasetProperties": {                            │
│               "FileSourceInfo": {                             │
│                 "FileSource": "wellbore_markers.csv"          │
│               }                                                │
│             }                                                  │
│           }                                                    │
│         }                                                      │
│       ]                                                        │
│     }                                                          │
│   }                                                            │
│ }                                                             │
│                                                               │
│ Response (200 OK):                                           │
│ {                                                            │
│   "message": "Workflow Osdu_ingest started successfully"    │
│ }                                                            │
└─────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────┐
│ Step 4.2: Airflow DAG Execution (Backend)                    │
│                                                               │
│ Workflow Service → Airflow DAG: Osdu_ingest                  │
│                                                               │
│ DAG Tasks:                                                    │
│   1. validate_manifest_task:                                │
│      - Validates manifest structure                         │
│      - Checks schema compliance                             │
│      - Verifies references (Wellbore, UnitOfMeasure)         │
│                                                               │
│   2. create_work_product_task:                              │
│      - Creates Work Product record                           │
│      - Links to Storage Service                              │
│                                                               │
│   3. create_work_product_component_task:                    │
│      - Creates WellboreMarkerSet WPC record                  │
│      - Validates marker data                                 │
│      - Associates with WP                                    │
│                                                               │
│   4. create_dataset_task:                                    │
│      - Creates Dataset record (if file-based)                │
│      - Links file to WPC                                     │
│                                                               │
│   5. link_relationships_task:                               │
│      - Links WPC to WP                                       │
│      - Links Dataset to WPC                                   │
│      - Updates WP Components array                           │
└─────────────────────────────────────────────────────────────┘
```

**Phase 5: Record Creation and Indexing**

```
┌─────────────────────────────────────────────────────────────┐
│ Step 5.1: Airflow → Storage Service                         │
│                                                               │
│ Airflow DAG → Storage Service API                           │
│ PUT /api/storage/v2/records                                  │
│                                                               │
│ Headers:                                                      │
│   - Authorization: Bearer <SERVICE_ACCOUNT_JWT>              │
│   - Content-Type: application/json                           │
│   - Slb-Data-Partition-Id: <data-partition-id>                │
│                                                               │
│ Request Body (Array of records):                             │
│ [                                                            │
│   {                                                           │
│     "kind": "osdu:wks:work-product--WorkProduct:1.0.0",    │
│     "id": "opendes:doc:4ff67ce36ae2452b8ddad3391f1fc08a",   │
│     "acl": { ... },                                           │
│     "legal": { ... },                                         │
│     "data": {                                                 │
│       "ResourceTypeID": "srn:type:work-product/WellboreMarker:",│
│       "Data": {                                               │
│         "GroupTypeProperties": {                              │
│           "Components": []                                    │
│         },                                                    │
│         "IndividualTypeProperties": {                         │
│           "Name": "wellbore_markers.csv",                     │
│           "Description": "Wellbore Marker"                    │
│         }                                                      │
│       }                                                        │
│     }                                                          │
│   },                                                           │
│   {                                                            │
│     "kind": "osdu:wks:work-product-component--WellboreMarkerSet:1.0.0",│
│     "id": "opendes:doc:e876f8a2963e4a1b9886e11f9b6634cb",    │
│     "acl": { ... },                                            │
│     "legal": { ... },                                          │
│     "data": {                                                  │
│       "ResourceTypeID": "srn:type:work-product-component/WellboreMarker:",│
│       "Data": {                                                │
│         "GroupTypeProperties": {                               │
│           "Files": ["srn:type:file/csv:66153425987012541:"],   │
│           "Artefacts": []                                      │
│         },                                                     │
│         "IndividualTypeProperties": {                          │
│           "Name": "wellbore_markers.csv",                      │
│           "Description": "Wellbore Marker",                    │
│           "WellboreID": "srn:master-data/Wellbore:1100:",      │
│           "DepthUnit": "srn:reference-data/UnitOfMeasure:M:", │
│           "Markers": [                                         │
│             {                                                  │
│               "MarkerName": "QUATER. UNDIFF.",                  │
│               "MarkerMeasuredDepth": 0.0                        │
│             },                                                 │
│             {                                                  │
│               "MarkerName": "Breda Formation",                  │
│               "MarkerMeasuredDepth": 133.0                       │
│             }                                                  │
│             // ... more markers                                │
│           ]                                                    │
│         }                                                      │
│       }                                                        │
│     }                                                          │
│   }                                                            │
│ ]                                                              │
│                                                               │
│ Response (200 OK):                                           │
│ [                                                            │
│   {                                                           │
│     "id": "opendes:doc:4ff67ce36ae2452b8ddad3391f1fc08a",    │
│     "version": 1582725123640845,                             │
│     "kind": "osdu:wks:work-product--WorkProduct:1.0.0"       │
│   },                                                           │
│   {                                                            │
│     "id": "opendes:doc:e876f8a2963e4a1b9886e11f9b6634cb",     │
│     "version": 1582725123640845,                              │
│     "kind": "osdu:wks:work-product-component--WellboreMarkerSet:1.0.0"│
│   }                                                            │
│ ]                                                              │
└─────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────┐
│ Step 5.2: Storage Service → Search Service (Indexing)       │
│                                                               │
│ Storage Service → Search Service API                         │
│ (Internal, automatic after record creation)                  │
│                                                               │
│ Process:                                                      │
│   1. Storage Service validates record against schema         │
│   2. Record stored in storage backend                        │
│   3. Storage Service triggers Search Service indexing        │
│   4. Only schematized fields are indexed                      │
│   5. Record becomes searchable                                │
│   6. Relationships (WP-WPC, WPC-Wellbore) indexed            │
│                                                               │
│ Note: Indexing is asynchronous and may take time             │
└─────────────────────────────────────────────────────────────┘
```

### 5.4 Complete Call Stack Summary

```
┌──────────────────────────────────────────────────────────────┐
│ WellboreMarkerSet Ingestion - Complete Call Stack           │
├──────────────────────────────────────────────────────────────┤
│                                                               │
│ 1. CLIENT APPLICATION                                         │
│    │                                                           │
│    ├─→ POST /api/storage/v2/schemas                          │
│    │   └─→ Storage Service                                    │
│    │       └─→ Schema Registry (Create WellboreMarkerSet schema)│
│    │                                                           │
│    ├─→ POST /api/file/v2/files (if file-based)              │
│    │   └─→ File Service                                      │
│    │       └─→ Generate Signed URL                            │
│    │                                                           │
│    ├─→ PUT <SignedURL>                                        │
│    │   └─→ Cloud Storage (S3/GCS/Azure)                       │
│    │       └─→ File Upload                                     │
│    │                                                           │
│    └─→ POST /api/workflow/v1/workflow/Osdu_ingest/workflowRun│
│        └─→ Workflow Service                                  │
│            └─→ Airflow DAG: Osdu_ingest                      │
│                │                                               │
│                ├─→ validate_manifest_task                     │
│                │   └─→ Schema Validation                      │
│                │   └─→ Reference Validation (Wellbore, Units)│
│                │                                               │
│                ├─→ create_work_product_task                    │
│                │   └─→ PUT /api/storage/v2/records            │
│                │       └─→ Storage Service                    │
│                │           └─→ Create WP Record                │
│                │                                               │
│                ├─→ create_work_product_component_task         │
│                │   └─→ PUT /api/storage/v2/records            │
│                │       └─→ Storage Service                    │
│                │           ├─→ Schema Validation              │
│                │           ├─→ Create WPC Record             │
│                │           └─→ Search Service (Indexing)      │
│                │                                               │
│                ├─→ create_dataset_task                          │
│                │   └─→ PUT /api/storage/v2/records            │
│                │       └─→ Storage Service                    │
│                │           └─→ Create Dataset Record         │
│                │                                               │
│                └─→ link_relationships_task                      │
│                    └─→ PUT /api/storage/v2/records            │
│                        └─→ Storage Service                    │
│                            └─→ Update WP Components array      │
│                                                               │
└──────────────────────────────────────────────────────────────┘
```

---

## 6. Direct Storage API Ingestion (Alternative)

### 6.1 Use Case

Direct Storage API ingestion is useful for:
- Programmatic ingestion from custom applications
- Single record ingestion (no workflow overhead)
- Custom data transformation pipelines
- Testing and development

### 6.2 Direct Ingestion Flow

```
┌─────────────────────────────────────────────────────────────┐
│ Step 1: Ensure Schema Exists                                │
│ GET /api/storage/v2/schemas/{kind}                          │
│ (Verify schema exists before proceeding)                    │
└─────────────────────────────────────────────────────────────┘
         │
         ▼
┌─────────────────────────────────────────────────────────────┐
│ Step 2: Create WellboreMarkerSet Record                     │
│ PUT /api/storage/v2/records                                  │
│                                                              │
│ Request Body:                                                │
│ [                                                           │
│   {                                                         │
│     "kind": "osdu:wks:work-product-component--WellboreMarkerSet:1.0.0",│
│     "acl": { ... },                                         │
│     "legal": { ... },                                        │
│     "data": {                                                │
│       "ResourceTypeID": "srn:type:work-product-component/WellboreMarker:",│
│       "Data": {                                              │
│         "IndividualTypeProperties": {                        │
│           "Name": "Direct Ingestion Marker Set",            │
│           "WellboreID": "srn:master-data/Wellbore:1100:",    │
│           "DepthUnit": "srn:reference-data/UnitOfMeasure:M:",│
│           "Markers": [ ... ]                                 │
│         }                                                    │
│       }                                                      │
│     }                                                        │
│   }                                                          │
│ ]                                                           │
│                                                              │
│ Response: Record created with ID and version                │
└─────────────────────────────────────────────────────────────┘
```

---

## 7. Key Design Patterns and Best Practices

### 7.1 Work Product Pattern

WellboreMarkerSet follows the OSDU Work Product pattern:

1. **Work Product (WP)**: Container that groups related WPCs
2. **Work Product Component (WPC)**: The actual WellboreMarkerSet record
3. **Components Array**: WP maintains array of WPC IDs
4. **Surrogate Keys**: Use surrogate keys for WP/WPC/Dataset IDs in manifests

### 7.2 Reference Data Usage

- **WellboreID**: Must reference existing Wellbore master data (SRN format)
- **DepthUnit**: Must reference Unit of Measure reference data (SRN format)
- **ResourceSecurityClassification**: Must reference ResourceSecurityClassification reference data

### 7.3 File Association Pattern

When associating source files:

1. Upload file via File Service → Get FileID (SRN)
2. Add FileID to `Data.GroupTypeProperties.Files` array
3. Create Dataset record linking to file
4. Reference Dataset in WPC if needed

### 7.4 Schema First Principle

**Always**:
1. ✅ Create schema before ingestion
2. ✅ Verify schema exists before creating records
3. ✅ Use correct kind identifier matching schema

**Never**:
1. ❌ Attempt to ingest without schema
2. ❌ Use incorrect kind identifier
3. ❌ Skip schema validation

### 7.5 Manifest ID Patterns

- **Surrogate Keys**: Use for WP, WPC, Dataset IDs in manifests
  - Format: `surrogate-key:wp-1`, `surrogate-key:wpc-1`
- **IDs Must End with Colon**: `surrogate-key:wpc-1:` (note the trailing colon)
- **System-Generated IDs**: For Master/Reference data, omit ID field

---

## 8. Important Considerations

### 8.1 Schema Requirements

- **Schema must be created before ingestion**: Records cannot be ingested without a defined schema
- **Only schematized fields are indexed**: Fields without schema definitions are not searchable
- **Schema versioning**: Schemas are versioned and must match record kinds
- **Nested path support**: Schema supports nested paths like `Data.IndividualTypeProperties.Markers[].MarkerName`

### 8.2 Data Validation

- **WellboreID validation**: Must reference existing Wellbore master data
- **Unit validation**: DepthUnit must reference valid Unit of Measure
- **Marker depth ordering**: Typically markers are ordered by depth (ascending)
- **Required fields**: Name, WellboreID, DepthUnit, Markers array are typically required

### 8.3 Performance Considerations

- **Batch processing**: Use manifest-based ingestion for multiple marker sets
- **File size**: Large marker sets may require chunking
- **Indexing delay**: Records may not be immediately searchable after ingestion
- **Relationship queries**: WP-WPC relationships enable efficient querying

### 8.4 Error Handling

- **Schema validation errors**: Check schema definition matches record structure
- **Reference validation errors**: Verify WellboreID and DepthUnit references exist
- **Manifest validation errors**: Check manifest structure against OSDU Manifest schema
- **Airflow DAG monitoring**: Monitor DAG execution for ingestion errors

---

## 9. References

- **OSDU Data Platform Wiki**: `https://community.opengroup.org/groups/osdu/platform/-/wikis`
- **Schema Repository**: `https://gitlab.opengroup.org/osdu/subcommittees/data-def/work-products/schema`
- **WellboreMarkerSet Schema**: `https://gitlab.opengroup.org/osdu/subcommittees/data-def/work-products/schema/-/tree/master/Generated/work-product-component/WellboreMarkerSet.1.0.0.json`
- **Manifest Schema**: `https://gitlab.opengroup.org/osdu/subcommittees/data-def/work-products/schema/-/blob/master/Generated/manifest/Manifest.1.0.0.json`
- **Wellbore DDMS API**: Domain-specific wellbore data management APIs
- **OSDU Data Loading Quick Start Guide**: Comprehensive ingestion workflows

---

## 10. Conclusion

WellboreMarkerSet in OSDU is well-architected with:

- ✅ **Clear data model**: Work Product/Work Product Component pattern
- ✅ **Comprehensive schema**: Supports nested marker arrays with depth information
- ✅ **Multiple ingestion pathways**: Manifest-based, Direct API, CSV Parser
- ✅ **File association**: Supports linking source CSV files via Dataset records
- ✅ **Reference data integration**: Uses OSDU reference data for units and classifications
- ✅ **Wellbore DDMS integration**: Optimized access through domain-specific services
- ✅ **Relationship management**: Proper WP-WPC-Wellbore relationships

The platform provides a robust foundation for wellbore marker data management, with clear separation between storage, processing, and domain-specific services, following OSDU's schema-first and work product patterns.

