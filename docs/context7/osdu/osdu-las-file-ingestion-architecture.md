# OSDU LAS File Ingestion & Well Log Schema Architecture

This document explains how the Open Subsurface Data Universe (OSDU) platform handles LAS file ingestion and well log schemas, including the architecture, code design, and call stack details.

## Overview

OSDU uses a **manifest-based ingestion approach** where LAS files (and other well log formats like DLIS) are stored in their original format, with metadata extracted and structured according to OSDU schemas. The platform preserves data lineage by keeping source files intact while making them searchable through structured metadata.

**Key Principles:**
- **Source Data Preservation**: LAS/DLIS files stored in original format
- **Metadata Extraction**: Structured metadata extracted for searchability
- **Schema-Based Organization**: Well logs organized using OSDU Work Product schemas
- **Manifest-Driven Ingestion**: JSON manifest files define the ingestion structure
- **Workflow Orchestration**: Apache Airflow orchestrates ingestion workflows

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                    OSDU Data Platform                        │
│                                                             │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  Ingestion Layer                                     │   │
│  │  - Manifest Parser                                   │   │
│  │  - File Service (upload/staging)                     │   │
│  │  - Metadata Extractor (LAS/DLIS parser)              │   │
│  └────────────────────┬────────────────────────────────┘   │
│                       │                                      │
│  ┌────────────────────▼────────────────────────────────┐   │
│  │  Storage Service (Metadata)                          │   │
│  │  - Work Product Records                              │   │
│  │  - Work Product Component Records (WellLog)          │   │
│  │  - Dataset Records (File references)                 │   │
│  │  - Schema Definitions                                │   │
│  └────────────────────┬────────────────────────────────┘   │
│                       │                                      │
│  ┌────────────────────▼────────────────────────────────┐   │
│  │  File Storage (Object Store)                         │   │
│  │  - LAS files (original format)                       │   │
│  │  - DLIS files                                        │   │
│  │  - Other well log formats                            │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                             │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  Search Service                                      │   │
│  │  - Indexed metadata fields                           │   │
│  │  - Full-text search                                  │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                             │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  Domain Data Management Services (DDMS)              │   │
│  │  - Wellbore DDMS (optimized log access)              │   │
│  │  - Type-safe entity access                           │   │
│  └─────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
```

## Data Model: Work Products, Components, and Datasets

OSDU organizes well log data using a hierarchical structure:

### 1. Work Product (WP)

A **Work Product** represents a collection of related data products. For well logs, this might be a complete logging run or a set of related logs.

**Schema Kind**: `{{data-partition-id}}:wks:work-product--WorkProduct:1.0.0`

**Example Structure**:
```json
{
  "kind": "osdu:wks:work-product--WorkProduct:1.0.0",
  "acl": {
    "owners": ["data.default.owners@osdu.example.com"],
    "viewers": ["data.default.viewers@osdu.example.com"]
  },
  "legal": {
    "legaltags": ["osdu-demo-legaltag"],
    "otherRelevantDataCountries": ["US"]
  },
  "data": {
    "Name": "Raster Well Log WP",
    "Description": "Raster Well Log WP for ML POC",
    "Components": ["surrogate-key: wpc1"]
  }
}
```

### 2. Work Product Component (WPC) - WellLog

A **Work Product Component** represents a specific well log. This is where well log metadata is stored.

**Schema Kind**: `{{data-partition-id}}:wks:work-product-component--WellLog:1.0.0`

**WellLog Schema Fields**:
```json
{
  "id": "surrogate-key: wpc1",
  "kind": "osdu:wks:work-product-component--WellLog:1.0.0",
  "acl": { /* ... */ },
  "legal": { /* ... */ },
  "data": {
    "ResourceSecurityClassification": "osdu:reference-data--ResourceSecurityClassification:Public:",
    "Source": "Oklahoma public registry",
    "Description": "Raster Well Log",
    "SubmitterName": "GCP Ingestion team",
    "WellLogTypeID": "osdu:reference-data--LogType:Raw:",
    "LogActivity": "Main Pass",
    "LogRun": "1",
    "WellboreId": "osdu:master-data--Wellbore:3511023252:",
    "BottomMeasuredDepth": 12660,
    "LoggingService": "SLIM CEMENT MAP TOOL",
    "LogVersion": "1",
    "ActivityType": "Wireline",
    "ServiceCompanyId": "osdu:master-data--Organisation:Schlumberger:",
    "Datasets": ["surrogate-key: dataset1"]
  }
}
```

**Key WellLog Fields**:
- `WellboreId`: Reference to the wellbore master data record
- `WellLogTypeID`: Type of log (Raw, Processed, Interpreted, etc.)
- `BottomMeasuredDepth`: Maximum depth of the log
- `LoggingService`: Tool/service used for logging
- `ActivityType`: Type of logging activity (Wireline, LWD, etc.)
- `Datasets`: Array of dataset IDs containing the actual log files

### 3. Dataset

A **Dataset** represents the actual file (LAS, DLIS, etc.) stored in object storage.

**Schema Kind**: `{{data-partition-id}}:wks:dataset--File.Generic:1.0.0`

**Example Structure**:
```json
{
  "id": "surrogate-key: dataset1",
  "kind": "osdu:wks:dataset--File.Generic:1.0.0",
  "acl": { /* ... */ },
  "legal": { /* ... */ },
  "data": {
    "Description": "Raster Well Log",
    "DatasetProperties": {
      "FileSourceInfo": {
        "FileSource": "gs://bucket/path/to/file.las",
        "Name": "well_log_001.las",
        "PreLoadFilePath": "/staging/well_log_001.las",
        "PreloadFileCreateUser": "Data Loading Team",
        "PreloadFileModifyDate": "Apr 15 2021",
        "PreloadFileModifyUser": "Data Loading Team"
      }
    },
    "TotalSize": "13245217273",
    "Source": "Example Data Source",
    "Name": "Dataset X221/15"
  }
}
```

## Ingestion Workflow

### Manifest-Based Ingestion Process

OSDU uses **manifest files** (JSON) to define the complete ingestion structure:

```
┌─────────────────────────────────────────────────────────────┐
│  1. Prepare Source Data                                      │
│     - LAS/DLIS files in staging area                        │
│     - Metadata mapping document (Excel/CSV)                 │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  2. Create Mapping Document                                  │
│     - Map LAS file metadata to OSDU WellLog schema          │
│     - Identify required reference/master data               │
│     - Define hardcoded values                               │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  3. Build Manifest File                                      │
│     - Work Product structure                                │
│     - Work Product Component (WellLog)                      │
│     - Dataset references                                    │
│     - ACL and legal tags                                    │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  4. Upload Files to Staging                                  │
│     POST /v2/files/upload                                    │
│     - Files uploaded to staging area                        │
│     - Returns FileSource identifier                         │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  5. Submit Manifest                                          │
│     - Airflow DAG processes manifest                        │
│     - Validates referential integrity                       │
│     - Creates records in Storage Service                    │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  6. File Service Metadata Ingestion                          │
│     POST /v2/files/metadata                                  │
│     - Creates Dataset record                                │
│     - Moves file from staging to persistent storage         │
│     - Returns Dataset record ID                             │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  7. Storage Service Record Creation                          │
│     PUT /api/storage/v2/records                              │
│     - Creates Work Product record                           │
│     - Creates Work Product Component (WellLog) record       │
│     - Links to Dataset record                               │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  8. Search Service Indexing                                  │
│     - Metadata fields indexed for search                    │
│     - Full-text search enabled                              │
└─────────────────────────────────────────────────────────────┘
```

## Complete Manifest Example

```json
{
  "manifest": {
    "kind": "osdu:wks:Manifest:1.0.0",
    "ReferenceData": [],
    "MasterData": [],
    "Data": {
      "WorkProduct": {
        "kind": "osdu:wks:work-product--WorkProduct:1.0.0",
        "acl": {
          "owners": ["data.default.owners@osdu.example.com"],
          "viewers": ["data.default.viewers@osdu.example.com"]
        },
        "legal": {
          "legaltags": ["osdu-demo-legaltag"],
          "otherRelevantDataCountries": ["US"]
        },
        "data": {
          "Name": "Raster Well Log WP",
          "Description": "Raster Well Log WP for ML POC",
          "Components": ["surrogate-key: wpc1"]
        }
      },
      "WorkProductComponents": [
        {
          "id": "surrogate-key: wpc1",
          "kind": "osdu:wks:work-product-component--WellLog:1.0.0",
          "acl": {
            "owners": ["data.default.owners@osdu.example.com"],
            "viewers": ["data.default.viewers@osdu.example.com"]
          },
          "legal": {
            "legaltags": ["osdu-demo-legaltag"],
            "otherRelevantDataCountries": ["US"]
          },
          "createUser": "GCP ML POC",
          "data": {
            "ResourceSecurityClassification": "osdu:reference-data--ResourceSecurityClassification:Public:",
            "Source": "Oklahoma public registry",
            "Description": "Raster Well Log",
            "SubmitterName": "GCP Ingestion team",
            "WellLogTypeID": "osdu:reference-data--LogType:Raw:",
            "LogActivity": "Main Pass",
            "LogRun": "1",
            "WellboreId": "osdu:master-data--Wellbore:3511023252:",
            "BottomMeasuredDepth": 12660,
            "LoggingService": "SLIM CEMENT MAP TOOL",
            "LogVersion": "1",
            "ActivityType": "Wireline",
            "ServiceCompanyId": "osdu:master-data--Organisation:Schlumberger:",
            "Datasets": ["surrogate-key: dataset1"]
          }
        }
      ],
      "Datasets": [
        {
          "id": "surrogate-key: dataset1",
          "kind": "osdu:wks:dataset--File.Generic:1.0.0",
          "acl": {
            "owners": ["data.default.owners@osdu.example.com"],
            "viewers": ["data.default.viewers@osdu.example.com"]
          },
          "legal": {
            "legaltags": ["osdu-demo-legaltag"],
            "otherRelevantDataCountries": ["US"]
          },
          "createUser": "ML POC",
          "data": {
            "Description": "Raster Well Log",
            "DatasetProperties": {
              "FileSourceInfo": {
                "FileSource": "gs://alexd-test-dataset/Demo/Demo/OKI.NW.13.13.04.XX.6B287FFB.D-N01_SLB_3501123504.las"
              }
            }
          }
        }
      ]
    }
  }
}
```

## Call Stack: LAS File Ingestion

### Detailed Call Stack

```
┌─────────────────────────────────────────────────────────────┐
│  1. Client Initiates Ingestion                               │
│     - User uploads LAS file                                  │
│     - Provides metadata mapping                              │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  2. File Upload to Staging                                   │
│     POST /v2/files/upload                                    │
│                                                             │
│     Request:                                                 │
│     - Multipart form data with LAS file                     │
│     - Headers: Authorization, Data-Partition-Id             │
│                                                             │
│     File Service:                                            │
│     - Validates file                                        │
│     - Stores in staging area                                │
│     - Returns FileSource identifier                         │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  3. LAS File Parsing (Optional)                              │
│     - Extract metadata from LAS file                        │
│     - Parse curve information                               │
│     - Extract well information                              │
│     - Generate mapping document                             │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  4. Manifest Generation                                      │
│     - Build Work Product structure                          │
│     - Build Work Product Component (WellLog)                │
│     - Build Dataset reference                               │
│     - Add ACL and legal tags                                │
│     - Use surrogate keys for WP/WPC/Dataset IDs             │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  5. Airflow DAG Execution                                    │
│     - process_manifest_r3.py                                │
│     - Validates manifest structure                          │
│     - Checks referential integrity                          │
│     - Processes records in order                            │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ├─────────────────────┬─────────────────┐
                       ▼                     ▼                 ▼
        ┌──────────────────┐  ┌──────────────────┐  ┌──────────────────┐
        │  Reference Data  │  │  Master Data     │  │  Data Records    │
        │  Ingestion       │  │  Ingestion       │  │  Ingestion       │
        └──────────────────┘  └──────────────────┘  └──────────────────┘
                       │                     │                 │
                       └─────────────────────┴─────────────────┘
                                           │
                                           ▼
┌─────────────────────────────────────────────────────────────┐
│  6. File Service Metadata Ingestion                          │
│     POST /v2/files/metadata                                  │
│                                                             │
│     Request Body:                                            │
│     {                                                       │
│       "data": {                                             │
│         "DatasetProperties": {                              │
│           "FileSourceInfo": {                               │
│             "FileSource": "<from step 2>",                  │
│             "Name": "well_log_001.las"                      │
│           }                                                 │
│         }                                                   │
│       },                                                    │
│       "kind": "osdu:wks:dataset--File.Generic:1.0.0",      │
│       "acl": { /* ... */ },                                 │
│       "legal": { /* ... */ }                                │
│     }                                                       │
│                                                             │
│     File Service:                                            │
│     - Creates Dataset record                                │
│     - Moves file from staging to persistent storage         │
│     - Returns Dataset record ID                             │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  7. Storage Service Record Creation                          │
│     PUT /api/storage/v2/records                              │
│                                                             │
│     Request Body (Array of Records):                        │
│     [                                                       │
│       {                                                     │
│         "kind": "osdu:wks:work-product--WorkProduct:1.0.0",│
│         "acl": { /* ... */ },                               │
│         "legal": { /* ... */ },                             │
│         "data": { /* Work Product data */ }                 │
│       },                                                    │
│       {                                                     │
│         "kind": "osdu:wks:work-product-component--WellLog:1.0.0",│
│         "acl": { /* ... */ },                               │
│         "legal": { /* ... */ },                             │
│         "data": {                                           │
│           "WellboreId": "...",                              │
│           "WellLogTypeID": "...",                           │
│           "BottomMeasuredDepth": 12660,                     │
│           "Datasets": ["<dataset-id-from-step-6>"]          │
│         }                                                   │
│       }                                                     │
│     ]                                                       │
│                                                             │
│     Storage Service:                                         │
│     - Validates schema                                      │
│     - Checks ACL permissions                                │
│     - Validates legal tags                                  │
│     - Stores records                                        │
│     - Returns record IDs                                    │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  8. Schema Indexing                                          │
│     - Storage Service indexes fields defined in schema      │
│     - Search Service builds search index                    │
│     - Metadata available for queries                        │
└─────────────────────────────────────────────────────────────┘
```

## Schema Management

### Schema Definition

OSDU uses schemas to define the structure of records. Schemas must be created before ingesting records.

**Create Schema API**:
```http
POST /api/storage/v2/schemas
Content-Type: application/json
Authorization: Bearer <JWT>
Slb-Data-Partition-Id: common

{
  "kind": "osdu:wks:work-product-component--WellLog:1.0.0",
  "schema": [
    {
      "path": "WellboreId",
      "kind": "string"
    },
    {
      "path": "WellLogTypeID",
      "kind": "string"
    },
    {
      "path": "BottomMeasuredDepth",
      "kind": "float"
    },
    {
      "path": "LoggingService",
      "kind": "string"
    },
    {
      "path": "ActivityType",
      "kind": "string"
    },
    {
      "path": "Datasets",
      "kind": "array"
    }
  ]
}
```

**Key Points**:
- Only fields with schema definitions are indexed by Search Service
- Schema defines data types for validation
- Schema must be created before record ingestion
- Schema supports nested structures and arrays

### WellLog Schema Structure

The WellLog schema (defined by OSDU Data Definitions team) includes:

**Core Fields**:
- `WellboreId`: Reference to wellbore master data
- `WellLogTypeID`: Type of log (Raw, Processed, Interpreted)
- `BottomMeasuredDepth`: Maximum depth (float)
- `TopMeasuredDepth`: Minimum depth (float, optional)
- `LoggingService`: Tool/service name
- `ActivityType`: Wireline, LWD, etc.
- `ServiceCompanyId`: Reference to organization
- `Datasets`: Array of dataset record IDs

**Metadata Fields**:
- `ResourceSecurityClassification`: Security level
- `Source`: Data source
- `Description`: Human-readable description
- `SubmitterName`: Who submitted the data
- `LogActivity`: Activity description
- `LogRun`: Run number
- `LogVersion`: Version identifier

## File Service API

### Upload File to Staging

```http
POST /v2/files/upload
Content-Type: multipart/form-data
Authorization: Bearer <JWT>
Slb-Data-Partition-Id: common

Form Data:
  file: <LAS file binary>
```

**Response**:
```json
{
  "FileSource": "gs://staging-bucket/temp/abc123.las"
}
```

### Ingest File Metadata

```http
POST /v2/files/metadata
Content-Type: application/json
Authorization: Bearer <JWT>
Slb-Data-Partition-Id: common

{
  "data": {
    "Description": "LAS Well Log",
    "DatasetProperties": {
      "FileSourceInfo": {
        "FileSource": "gs://staging-bucket/temp/abc123.las",
        "Name": "well_log_001.las"
      }
    },
    "TotalSize": "13245217",
    "Source": "Data Source",
    "Name": "Dataset X221/15"
  },
  "kind": "osdu:wks:dataset--File.Generic:1.0.0",
  "acl": {
    "viewers": ["data.default.viewers@osdu.example.com"],
    "owners": ["data.default.owners@osdu.example.com"]
  },
  "legal": {
    "legaltags": ["osdu-demo-legaltag"],
    "otherRelevantDataCountries": ["US"]
  }
}
```

**Response**:
```json
{
  "id": "osdu:dataset--File.Generic:ce6fe9fd-ab46-4358-ae27-8631e6cf8ae4"
}
```

## Storage Service API

### Ingest Records

```http
PUT /api/storage/v2/records
Content-Type: application/json
Authorization: Bearer <JWT>
Slb-Data-Partition-Id: common

[
  {
    "kind": "osdu:wks:work-product--WorkProduct:1.0.0",
    "acl": {
      "viewers": ["data.default.viewers@osdu.example.com"],
      "owners": ["data.default.owners@osdu.example.com"]
    },
    "legal": {
      "legaltags": ["osdu-demo-legaltag"],
      "otherRelevantDataCountries": ["US"]
    },
    "data": {
      "Name": "Raster Well Log WP",
      "Description": "Raster Well Log WP for ML POC",
      "Components": ["surrogate-key: wpc1"]
    }
  },
  {
    "kind": "osdu:wks:work-product-component--WellLog:1.0.0",
    "acl": {
      "viewers": ["data.default.viewers@osdu.example.com"],
      "owners": ["data.default.owners@osdu.example.com"]
    },
    "legal": {
      "legaltags": ["osdu-demo-legaltag"],
      "otherRelevantDataCountries": ["US"]
    },
    "data": {
      "WellboreId": "osdu:master-data--Wellbore:3511023252:",
      "WellLogTypeID": "osdu:reference-data--LogType:Raw:",
      "BottomMeasuredDepth": 12660,
      "LoggingService": "SLIM CEMENT MAP TOOL",
      "ActivityType": "Wireline",
      "ServiceCompanyId": "osdu:master-data--Organisation:Schlumberger:",
      "Datasets": ["osdu:dataset--File.Generic:ce6fe9fd-ab46-4358-ae27-8631e6cf8ae4"]
    }
  }
]
```

## Referential Integrity

OSDU enforces referential integrity during ingestion. All referenced records must exist:

**Required References for WellLog**:
- `WellboreId`: Must exist in master-data--Wellbore
- `WellLogTypeID`: Must exist in reference-data--LogType
- `ServiceCompanyId`: Must exist in master-data--Organisation
- `ResourceSecurityClassification`: Must exist in reference-data--ResourceSecurityClassification

**Validation Process**:
```
┌─────────────────────────────────────────────────────────────┐
│  Referential Integrity Check                                 │
│                                                             │
│  1. Extract all reference IDs from manifest                 │
│  2. Query Storage Service for each reference                │
│  3. If any reference missing:                               │
│     - Log warning                                           │
│     - Reject record                                         │
│     - Add to skipped_ids list                               │
│  4. If all references exist:                                │
│     - Proceed with ingestion                                │
│     - Add to processed_ids list                             │
└─────────────────────────────────────────────────────────────┘
```

**Example Error Log**:
```
WARNING - Resource with kind osdu:wks:master-data--Well:1.0.0 
and id: 'osdu:master-data--Well:1000' was rejected.

Missing ids {
  'osdu:master-data--GeoPoliticalEntity:Netherlands:',
  'osdu:reference-data--FacilityEventType:SPUD:',
  'osdu:reference-data--LogType:Raw:',
  ...
}
```

## Domain Data Management Services (DDMS)

### Wellbore DDMS

The **Wellbore Domain Data Management Service** provides optimized access to well log data:

**Features**:
- Type-safe entity access
- Optimized accessors for bulk data (logs, trajectories, checkshots)
- Efficient querying of well log metadata
- Direct access to log files

**API Pattern**:
```http
GET /api/wellbore/v1/welllogs/{welllog-id}
GET /api/wellbore/v1/welllogs/{welllog-id}/data
GET /api/wellbore/v1/wellbores/{wellbore-id}/logs
```

## Airflow Workflow Orchestration

### Manifest Processing DAG

OSDU uses Apache Airflow to orchestrate ingestion workflows:

```python
from airflow.models.dag import DAG
from airflow.operators.python import PythonOperator

def process_manifest_task(**context):
    manifest_file = context['dag_run'].conf['manifest_file']
    
    # Parse manifest
    manifest = parse_manifest(manifest_file)
    
    # Validate referential integrity
    validate_references(manifest)
    
    # Process records
    processed_ids = []
    skipped_ids = []
    
    for record in manifest['Data']['WorkProductComponents']:
        try:
            record_id = ingest_record(record)
            processed_ids.append(record_id)
        except Exception as e:
            skipped_ids.append(record['id'])
            log_warning(f"Failed to ingest {record['id']}: {e}")
    
    return {
        'processed_ids': processed_ids,
        'skipped_ids': skipped_ids
    }

with DAG(
    dag_id="osdu_manifest_ingestion",
    schedule=None,
    start_date=pendulum.datetime(2023, 1, 1, tz="UTC"),
) as dag:
    process_manifest = PythonOperator(
        task_id="process_manifest_task",
        python_callable=process_manifest_task,
    )
```

## LAS File Metadata Extraction

While OSDU stores LAS files in their original format, metadata can be extracted for the manifest:

**LAS File Structure**:
```
~Version Information
~Well Information
  STRT .M    48.0000 : START DEPTH
  STOP .M   12660.0000 : STOP DEPTH
  STEP .M      0.1524 : STEP
  NULL .      -999.25 : NULL VALUE
  WELL .     WELL-001 : WELL NAME
  UWI  .   1234567890 : UNIQUE WELL IDENTIFIER
~Curve Information
  DEPT .M              : DEPTH
  GR   .API            : GAMMA RAY
  RHOB .G/CC           : BULK DENSITY
~ASCII
  [Data values]
```

**Mapping to WellLog Schema**:
- `WellboreId`: Lookup by UWI or well name
- `BottomMeasuredDepth`: From STOP value
- `TopMeasuredDepth`: From STRT value
- `LoggingService`: From service company in header
- `ActivityType`: Determine from log type
- Curve information: Stored in Dataset metadata (not in WellLog schema)

## Key Design Patterns

### 1. Surrogate Keys

For Work Products, Work Product Components, and Datasets in manifests, use surrogate keys:

```json
{
  "id": "surrogate-key: wpc1",
  "kind": "osdu:wks:work-product-component--WellLog:1.0.0",
  ...
}
```

The system generates actual record IDs during ingestion.

### 2. Reference Data Pattern

Reference data (like LogType, ResourceSecurityClassification) must be ingested first:

```
1. Ingest Reference Data
2. Ingest Master Data (Wellbore, Organisation)
3. Ingest Data Records (Work Product, WellLog, Dataset)
```

### 3. File Preservation Pattern

- Source files (LAS/DLIS) stored in original format
- Metadata extracted and structured
- Files referenced via Dataset records
- Enables data lineage and auditability

### 4. ACL and Legal Tags

Every record requires:
- **ACL**: Access control (owners, viewers)
- **Legal Tags**: Legal/compliance metadata
- **Data Partition**: Tenant isolation

## Comparison with DataForge Approach

| Aspect | OSDU | DataForge |
|--------|------|-----------|
| **File Storage** | Original format (LAS/DLIS) | Converted to Parquet |
| **Metadata** | Structured in WellLog schema | Structured in PostgreSQL tables |
| **Schema** | OSDU WellLog schema | Custom well/curve schema |
| **Ingestion** | Manifest-based (JSON) | Direct upload + mapping UI |
| **Lineage** | Preserved via file references | Tracked in database |
| **Search** | Indexed metadata fields | SQL queries on PostgreSQL |
| **Access** | DDMS APIs | Direct database queries |

## Key Takeaways

1. **Manifest-Based Ingestion**: JSON manifests define complete ingestion structure
2. **Hierarchical Organization**: Work Product → Work Product Component (WellLog) → Dataset
3. **Source File Preservation**: LAS files stored in original format
4. **Schema-Driven**: Well-defined schemas for all record types
5. **Referential Integrity**: Strict validation of references
6. **Workflow Orchestration**: Airflow DAGs orchestrate ingestion
7. **Domain Services**: DDMS provides optimized access to well log data
8. **ACL and Legal**: Every record requires access control and legal tags

## References

- [OSDU Data Platform Documentation](https://community.opengroup.org/groups/osdu/platform)
- [OSDU WellLog Schema](https://gitlab.opengroup.org/osdu/subcommittees/data-def/work-products/schema/-/blob/master/Generated/work-product-component/WellLog.1.0.0.json)
- [OSDU Manifest Schema](https://gitlab.opengroup.org/osdu/subcommittees/data-def/work-products/schema/-/blob/master/Generated/manifest/Manifest.1.0.0.json)
- [OSDU Data Loading Quick Start Guide](https://community.opengroup.org/groups/osdu/platform/-/wikis/OSDU-Data-Platform-Data-Loading-Quick-Start-Guide)
- [OSDU API Quick Start Guide](https://community.opengroup.org/groups/osdu/platform/-/wikis/OSDU-API-Quick-start-guide)

