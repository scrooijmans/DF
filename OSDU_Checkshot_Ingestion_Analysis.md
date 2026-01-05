# OSDU Checkshot Ingestion Analysis: Architecture, Schema, and Ingestion Flow

## Executive Summary

This document provides a comprehensive analysis of how checkshots (time-depth relationships) are designed, stored, processed, and ingested in the OSDU (Open Subsurface Data Universe) platform. It covers the complete architecture, schema definitions, and detailed ingestion workflows with call stacks for all supported formats and methods.

---

## 1. Checkshot Architecture Overview

### 1.1 Conceptual Overview

**Checkshots** (also known as **Time-Depth Relationships** or **Checkshot Surveys**) are measurements that establish the relationship between depth and seismic travel time. They are used for:

- **Velocity Calibration**: Calibrating seismic velocity models
- **Time-to-Depth Conversion**: Converting seismic time data to depth
- **Seismic-to-Well Ties**: Correlating well data with seismic data
- **Velocity Analysis**: Building velocity models for depth conversion

In OSDU, checkshots are represented as **Work Product Components (WPC)** in the OSDU data model, following the Work Product pattern.

### 1.2 OSDU Data Model Hierarchy

Checkshots fit into the OSDU data model hierarchy as follows:

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
│ - Example: Wellbore Survey WP           │
└──────────────┬──────────────────────────┘
               │
               ▼
┌─────────────────────────────────────────┐
│ Work Product Component (WPC)            │
│ - Checkshot                             │
│ - Contains array of stations            │
│ - Each station has:                     │
│   • Measured Depth (MD)                 │
│   • True Vertical Depth (TVD)           │
│   • Two-Way Time (TWT)                  │
└──────────────┬──────────────────────────┘
               │
               ▼
┌─────────────────────────────────────────┐
│ Dataset                                 │
│ - References source files (CSV, WITSML) │
│ - Links to WPC                          │
└─────────────────────────────────────────┘
```

### 1.3 Key Architectural Components

1. **Work Product (WP)**: Container that groups one or more checkshot WPCs
2. **Work Product Component (WPC)**: The actual Checkshot record containing station data
3. **Dataset**: References to source files (typically CSV or WITSML files)
4. **Master Data References**: Links to Wellbore entities
5. **Wellbore DDMS**: Specialized service for optimized bulk storage of checkshot station arrays
6. **Reference Data**: Unit of Measure references for depth and time units

### 1.4 Design Principles

- **Schema First**: Checkshots require schema definition before ingestion
- **Work Product Pattern**: Follows OSDU Work Product/Work Product Component pattern
- **Metadata/Bulk Separation**: Metadata in Storage Service, bulk data in Wellbore DDMS
- **Station-Based Model**: Checkshots represented as arrays of stations (MD, TVD, TWT)
- **Wellbore Association**: Each checkshot is associated with a specific Wellbore
- **Unit Standardization**: Uses OSDU Unit of Measure reference data for depth and time units

---

## 2. Checkshot Schema Definition

### 2.1 Schema Structure

Checkshot is defined as a Work Product Component with the following schema structure:

**Kind Identifier Pattern**:
```
{authority}:wks:work-product-component--Checkshot:{version}
```

**Example**: `osdu:wks:work-product-component--Checkshot:1.0.0`

### 2.2 Schema Fields

#### 2.2.1 Work Product Component Schema

```json
{
  "kind": "osdu:wks:work-product-component--Checkshot:1.0.0",
  "schema": [
    {
      "path": "data.ResourceTypeID",
      "kind": "core:String:1.0.0"
    },
    {
      "path": "data.ResourceSecurityClassification",
      "kind": "core:String:1.0.0"
    },
    {
      "path": "data.WellboreId",
      "kind": "core:String:1.0.0"
    },
    {
      "path": "data.Name",
      "kind": "core:String:1.0.0"
    },
    {
      "path": "data.Description",
      "kind": "core:String:1.0.0"
    },
    {
      "path": "data.SurveyType",
      "kind": "core:String:1.0.0"
    },
    {
      "path": "data.SurveyTypeID",
      "kind": "core:String:1.0.0"
    },
    {
      "path": "data.StationCount",
      "kind": "core:Integer:1.0.0"
    },
    {
      "path": "data.MinMD",
      "kind": "core:Double:1.0.0"
    },
    {
      "path": "data.MaxMD",
      "kind": "core:Double:1.0.0"
    },
    {
      "path": "data.MinTVD",
      "kind": "core:Double:1.0.0"
    },
    {
      "path": "data.MaxTVD",
      "kind": "core:Double:1.0.0"
    },
    {
      "path": "data.MinTWT",
      "kind": "core:Double:1.0.0"
    },
    {
      "path": "data.MaxTWT",
      "kind": "core:Double:1.0.0"
    },
    {
      "path": "data.TimeUnit",
      "kind": "core:String:1.0.0"
    },
    {
      "path": "data.TimeUnitID",
      "kind": "core:String:1.0.0"
    },
    {
      "path": "data.DepthUnit",
      "kind": "core:String:1.0.0"
    },
    {
      "path": "data.DepthUnitID",
      "kind": "core:String:1.0.0"
    },
    {
      "path": "data.DataGroupTypeProperties.Files",
      "kind": "core:Array:1.0.0"
    },
    {
      "path": "data.Datasets",
      "kind": "core:Array:1.0.0"
    }
  ]
}
```

### 2.3 Checkshot Station Data Structure

Checkshot stations contain the following fields:

#### Checkshot Station:
- **md** (float): Measured depth from vertical reference point
- **tvd** (float): True vertical depth
- **twt** (float): Two-way time (seismic travel time)
- **velocity** (float, optional): Calculated velocity (TVD/TWT)

### 2.4 Understanding "Schema First" Approach

OSDU requires **schema definition before ingestion** for checkshots. This ensures:

1. **Type Safety**: Data types are validated at ingestion time
2. **Indexing**: Only schema-defined fields are indexed by Search Service
3. **Validation**: Required fields and data types are enforced
4. **Discovery**: Schema enables structured search and query capabilities
5. **Versioning**: Schema versions enable evolution of data structures

**Schema Creation Workflow**:
```
1. Define Schema
   POST /api/storage/v2/schemas
   ↓
2. Schema Validation
   - Validate schema structure
   - Validate data types
   - Store schema in Schema Service
   ↓
3. Schema Registration
   - Schema linked to kind
   - Available for record validation
   ↓
4. Record Ingestion
   PUT /api/storage/v2/records
   ↓
5. Schema Validation
   - Validate record against schema
   - Check required fields
   - Validate data types
   ↓
6. Indexing
   - Only schema-defined fields indexed
   - Extract searchable metadata
```

### 2.5 Schema API

**Create Schema**:
```http
POST /api/storage/v2/schemas
Content-Type: application/json
Authorization: Bearer <JWT>
Slb-Data-Partition-Id: osdu

{
  "kind": "osdu:wks:work-product-component--Checkshot:1.0.0",
  "schema": [
    {
      "path": "data.WellboreId",
      "kind": "core:String:1.0.0"
    },
    {
      "path": "data.SurveyType",
      "kind": "core:String:1.0.0"
    },
    {
      "path": "data.StationCount",
      "kind": "core:Integer:1.0.0"
    },
    {
      "path": "data.MinMD",
      "kind": "core:Double:1.0.0"
    },
    {
      "path": "data.MaxMD",
      "kind": "core:Double:1.0.0"
    },
    {
      "path": "data.TimeUnitID",
      "kind": "core:String:1.0.0"
    },
    {
      "path": "data.DepthUnitID",
      "kind": "core:String:1.0.0"
    }
  ]
}
```

**Get Schema**:
```http
GET /api/storage/v2/schemas/osdu:wks:work-product-component--Checkshot:1.0.0
Authorization: Bearer <JWT>
Slb-Data-Partition-Id: osdu
```

---

## 3. Supported Data Formats

### 3.1 CSV Format

CSV files for checkshot data:

**Structure**:
```csv
MD,TVD,TWT
0.0,0.0,0.0
100.0,100.0,0.05
200.0,200.0,0.10
300.0,300.0,0.15
```

**Column Mapping**:
- **MD** → Measured Depth (meters)
- **TVD** → True Vertical Depth (meters)
- **TWT** → Two-Way Time (seconds)
- **Velocity** → Optional calculated velocity

**Validation Rules**:
- TVD: Required, number, range: -10000 to 50000
- TWT: Required, number, range: 0 to 100

**Storage Strategy**: CSV files are preserved in File Service, with station data extracted and stored in Wellbore DDMS.

### 3.2 WITSML Format

WITSML (Wellsite Information Transfer Standard Markup Language) for checkshot data:

**Structure**:
- **XML-based**: Industry standard XML format
- **Checkshot Surveys**: WITSML 2.0 checkshot survey objects
- **Station Data**: Time-depth pairs in WITSML format

**Key Elements**:
- `checkshotSurvey`: Root element for checkshot survey
- `wellbore`: Reference to wellbore
- `station`: Individual time-depth measurement
- `md`: Measured depth
- `tvd`: True vertical depth
- `time`: Two-way time

**Storage Strategy**: WITSML files are preserved in File Service, with checkshot data parsed and stored in Wellbore DDMS.

---

## 4. Ingestion Methods

OSDU supports multiple ingestion pathways for checkshots:

1. **CSV Parser Ingestion** (Airflow DAG)
2. **WITSML Parser Ingestion** (Energistics integration)
3. **Manifest-based Ingestion** (Workflow Service)
4. **Direct Storage API Ingestion** (Programmatic)

---

## 5. Ingestion Flow: Complete Walkthrough

### 5.1 Pathway 1: CSV Checkshot Ingestion

**Architecture**:
```
CSV File Upload
    ↓
File Service (staging)
    ↓
CSV Parser DAG (Airflow)
    ↓
CSV Parsing & Validation
    ↓
Station Array Generation
    ↓
Wellbore DDMS Storage (bulk data)
    ↓
Storage Service (metadata)
    ↓
Indexer Service
```

**Complete Call Stack**:

```
┌─────────────────────────────────────────────────────────────┐
│ Phase 1: File Upload                                         │
└─────────────────────────────────────────────────────────────┘

Step 1.1: Upload CSV File
Client → File Service API
POST /v2/files/upload
Content-Type: multipart/form-data
Authorization: Bearer <JWT>
Slb-Data-Partition-Id: osdu

Request Body (multipart/form-data):
  - file: well-001-checkshot.csv (binary)

Response (200 OK):
{
  "FileID": "4ff67ce36ae2452b8ddad3391f1fc08a",
  "FileSource": "srn:type:file/csv:checkshot-001:",
  "Location": "gs://osdu-storage/osdu/files/4ff67ce36ae2452b8ddad3391f1fc08a/well-001-checkshot.csv"
}

┌─────────────────────────────────────────────────────────────┐
│ Phase 2: Trigger CSV Parser Workflow                        │
└─────────────────────────────────────────────────────────────┘

Step 2.1: Trigger CSV Parser DAG
Client → Workflow Service API
POST /v1/workflow/Osdu_ingest/workflowRun
Content-Type: application/json
Authorization: Bearer <JWT>
Slb-Data-Partition-Id: osdu

Request Body:
{
  "executionContext": {
    "Payload": {
      "AppKey": "csv-parser",
      "data-partition-id": "osdu",
      "FileSource": "srn:type:file/csv:checkshot-001:",
      "WellboreId": "osdu:master-data--Wellbore:001:"
    }
  }
}

Response (200 OK):
{
  "workflowId": "workflow-001",
  "status": "RUNNING"
}

┌─────────────────────────────────────────────────────────────┐
│ Phase 3: CSV Parser Processing (Airflow DAG)               │
└─────────────────────────────────────────────────────────────┘

Step 3.1: Read CSV File from Staging
Airflow DAG → File Service
GET /v2/files/{fileId}/download
Authorization: Bearer <JWT>

Response: CSV file content

Step 3.2: Parse CSV Header Row
Airflow DAG (Python):
  - Read first row
  - Validate required columns (MD, TVD, TWT)
  - Detect optional columns (Velocity)

Step 3.3: Parse CSV Data Rows
Airflow DAG (Python):
  - For each data row:
    * Extract MD, TVD, TWT values
    * Validate numeric ranges:
      - TVD: -10000 to 50000
      - TWT: 0 to 100
    * Validate data types (numeric)
    * Store in stations array

Step 3.4: Generate Checkshot Metadata
Airflow DAG (Python):
  - Calculate statistics:
    * StationCount: Number of stations
    * MinMD, MaxMD: Depth range
    * MinTVD, MaxTVD: TVD range
    * MinTWT, MaxTWT: Time range
  - Determine units (from metadata or default)
  - Extract survey type (from metadata or default to "Checkshot")

Step 3.5: Extract Checkshot Name
Airflow DAG (Python):
  - Extract from filename or CSV metadata
  - Default to filename if not specified

┌─────────────────────────────────────────────────────────────┐
│ Phase 4: Wellbore DDMS Storage (Bulk Data)                  │
└─────────────────────────────────────────────────────────────┘

Step 4.1: Store Checkshot Stations in Wellbore DDMS
Airflow DAG → Wellbore DDMS API
POST /api/os-wellbore-ddms/wellbores/{wellboreId}/checkshots
Content-Type: application/json
Authorization: Bearer <JWT>

Request Body:
{
  "wellboreId": "osdu:master-data--Wellbore:001:",
  "stations": [
    {
      "md": 0.0,
      "tvd": 0.0,
      "twt": 0.0
    },
    {
      "md": 100.0,
      "tvd": 100.0,
      "twt": 0.05
    },
    {
      "md": 200.0,
      "tvd": 200.0,
      "twt": 0.10
    },
    {
      "md": 300.0,
      "tvd": 300.0,
      "twt": 0.15
    }
  ],
  "timeUnit": "osdu:reference-data--UnitOfMeasure:S:",
  "depthUnit": "osdu:reference-data--UnitOfMeasure:M:"
}

Response (200 OK):
{
  "checkshotId": "checkshot-001",
  "stationCount": 4,
  "status": "STORED"
}

┌─────────────────────────────────────────────────────────────┐
│ Phase 5: Storage Service Metadata Ingestion                │
└─────────────────────────────────────────────────────────────┘

Step 5.1: Create Checkshot WPC Record
Airflow DAG → Storage Service API
PUT /api/storage/v2/records
Content-Type: application/json
Authorization: Bearer <JWT>
Slb-Data-Partition-Id: osdu

Request Body:
[
  {
    "kind": "osdu:wks:work-product-component--Checkshot:1.0.0",
    "acl": {
      "viewers": ["data.default.viewers@osdu"],
      "owners": ["data.default.owners@osdu"]
    },
    "legal": {
      "legaltags": ["osdu-legaltag-001"],
      "otherRelevantDataCountries": ["US"]
    },
    "data": {
      "ResourceTypeID": "srn:type:work-product-component/Checkshot:",
      "WellboreId": "osdu:master-data--Wellbore:001:",
      "Name": "Well-001 Checkshot Survey",
      "Description": "Checkshot survey for velocity calibration",
      "SurveyType": "Checkshot",
      "SurveyTypeID": "osdu:reference-data--SurveyType:Checkshot:",
      "StationCount": 4,
      "MinMD": 0.0,
      "MaxMD": 300.0,
      "MinTVD": 0.0,
      "MaxTVD": 300.0,
      "MinTWT": 0.0,
      "MaxTWT": 0.15,
      "TimeUnit": "Second",
      "TimeUnitID": "osdu:reference-data--UnitOfMeasure:S:",
      "DepthUnit": "Meter",
      "DepthUnitID": "osdu:reference-data--UnitOfMeasure:M:",
      "DataGroupTypeProperties": {
        "Files": [
          "srn:type:file/csv:checkshot-001:"
        ]
      },
      "Datasets": [
        "osdu:dataset--File.Generic:checkshot-001:"
      ]
    }
  }
]

Response (200 OK):
[
  {
    "id": "osdu:doc:5gg78df47bf3563c9eead4492g2gd09b",
    "version": 1582725123640845
  }
]

┌─────────────────────────────────────────────────────────────┐
│ Phase 6: Indexing                                           │
└─────────────────────────────────────────────────────────────┘

Step 6.1: Indexer Service Processing
Indexer Service (Automatic):
  - Receives record creation event
  - Extracts searchable fields:
    * WellboreId
    * Name
    * SurveyType
    * StationCount
    * MinMD, MaxMD
    * MinTVD, MaxTVD
    * MinTWT, MaxTWT
  - Updates search index
  - Enables depth and time range queries
```

**CSV Parser DAG Implementation**:

```python
# csv_checkshot_ingestion_dag.py (Airflow DAG)
from airflow import DAG
from airflow.operators.python import PythonOperator
from datetime import datetime

def parse_csv_checkshot(file_source, wellbore_id, data_partition_id):
    """
    Parse CSV file and create checkshot records
    """
    # 1. Read CSV from staging
    csv_data = file_service.read(file_source)

    # 2. Parse CSV
    import csv
    reader = csv.DictReader(csv_data)

    # 3. Extract checkshot stations
    stations = []
    for row in reader:
        station = {
            "md": float(row['MD']),
            "tvd": float(row['TVD']),
            "twt": float(row['TWT'])
        }
        # Optional: Calculate velocity
        if station['twt'] > 0:
            station['velocity'] = station['tvd'] / station['twt']
        stations.append(station)

    # 4. Calculate statistics
    station_count = len(stations)
    min_md = min(s['md'] for s in stations)
    max_md = max(s['md'] for s in stations)
    min_tvd = min(s['tvd'] for s in stations)
    max_tvd = max(s['tvd'] for s in stations)
    min_twt = min(s['twt'] for s in stations)
    max_twt = max(s['twt'] for s in stations)

    # 5. Determine units (from metadata or default)
    time_unit_id = determine_time_unit(file_source) or f"{data_partition_id}:reference-data--UnitOfMeasure:S:"
    depth_unit_id = determine_depth_unit(file_source) or f"{data_partition_id}:reference-data--UnitOfMeasure:M:"

    # 6. Extract checkshot name from filename
    checkshot_name = extract_checkshot_name(file_source)

    # 7. Store in Wellbore DDMS
    checkshot_id = wellbore_ddms.store_checkshot(
        wellbore_id=wellbore_id,
        stations=stations,
        time_unit_id=time_unit_id,
        depth_unit_id=depth_unit_id
    )

    # 8. Create checkshot record
    storage_service.ingest_records([{
        "kind": f"{data_partition_id}:wks:work-product-component--Checkshot:1.0.0",
        "data": {
            "ResourceTypeID": "srn:type:work-product-component/Checkshot:",
            "WellboreId": wellbore_id,
            "Name": checkshot_name,
            "SurveyType": "Checkshot",
            "SurveyTypeID": f"{data_partition_id}:reference-data--SurveyType:Checkshot:",
            "StationCount": station_count,
            "MinMD": min_md,
            "MaxMD": max_md,
            "MinTVD": min_tvd,
            "MaxTVD": max_tvd,
            "MinTWT": min_twt,
            "MaxTWT": max_twt,
            "TimeUnitID": time_unit_id,
            "DepthUnitID": depth_unit_id,
            "DataGroupTypeProperties": {
                "Files": [file_source]
            }
        }
    }])

    return {"status": "success", "checkshot_id": checkshot_id, "station_count": station_count}

with DAG(
    dag_id='csv_checkshot_ingestion',
    start_date=datetime(2023, 1, 1),
    schedule_interval=None,
    catchup=False,
    tags=['osdu', 'checkshot', 'csv', 'ingestion'],
) as dag:
    parse_task = PythonOperator(
        task_id='parse_csv_checkshot',
        python_callable=parse_csv_checkshot,
        op_kwargs={
            'file_source': '{{ dag_run.conf.file_source }}',
            'wellbore_id': '{{ dag_run.conf.wellbore_id }}',
            'data_partition_id': '{{ dag_run.conf.data_partition_id }}'
        }
    )
```

### 5.2 Pathway 2: WITSML Checkshot Ingestion

**Architecture**:
```
WITSML File Upload
    ↓
File Service (staging)
    ↓
WITSML Parser (Energistics)
    ↓
WITSML Parsing & Validation
    ↓
Station Array Generation
    ↓
Wellbore DDMS Storage (bulk data)
    ↓
Storage Service (metadata)
    ↓
Indexer Service
```

**Complete Call Stack**:

```
┌─────────────────────────────────────────────────────────────┐
│ Phase 1: File Upload                                         │
└─────────────────────────────────────────────────────────────┘

Step 1.1: Upload WITSML File
Client → File Service API
POST /v2/files/upload
Content-Type: multipart/form-data
Authorization: Bearer <JWT>
Slb-Data-Partition-Id: osdu

Request Body (multipart/form-data):
  - file: well-001-checkshot.xml (binary)

Response (200 OK):
{
  "FileID": "6hh89eg58cg4674d0ggbe5615h3he21e",
  "FileSource": "srn:type:file/witsml:checkshot-002:",
  "Location": "gs://osdu-storage/osdu/files/6hh89eg58cg4674d0ggbe5615h3he21e/well-001-checkshot.xml"
}

┌─────────────────────────────────────────────────────────────┐
│ Phase 2: Trigger WITSML Parser Workflow                     │
└─────────────────────────────────────────────────────────────┘

Step 2.1: Trigger WITSML Parser
Client → Workflow Service API
POST /v1/workflow/Osdu_ingest/workflowRun
Content-Type: application/json
Authorization: Bearer <JWT>
Slb-Data-Partition-Id: osdu

Request Body:
{
  "executionContext": {
    "Payload": {
      "AppKey": "witsml-parser",
      "data-partition-id": "osdu",
      "FileSource": "srn:type:file/witsml:checkshot-002:",
      "WellboreId": "osdu:master-data--Wellbore:001:"
    }
  }
}

Response (200 OK):
{
  "workflowId": "workflow-002",
  "status": "RUNNING"
}

┌─────────────────────────────────────────────────────────────┐
│ Phase 3: WITSML Parser Processing                           │
└─────────────────────────────────────────────────────────────┘

Step 3.1: Read WITSML File from Staging
WITSML Parser → File Service
GET /v2/files/{fileId}/download
Authorization: Bearer <JWT>

Response: WITSML XML file content

Step 3.2: Parse WITSML Structure
WITSML Parser (Energistics):
  - Parse XML structure
  - Extract checkshotSurvey element
  - Extract wellbore reference
  - Extract survey metadata:
    * Survey type
    * Survey date
    * Service company
    * Units

Step 3.3: Parse Station Data
WITSML Parser (Energistics):
  - For each station element:
    * Extract MD (measured depth)
    * Extract TVD (true vertical depth)
    * Extract TWT (two-way time)
    * Validate data types and ranges
    * Store in stations array

Step 3.4: Generate Checkshot Metadata
WITSML Parser:
  - Calculate statistics:
    * StationCount
    * MinMD, MaxMD
    * MinTVD, MaxTVD
    * MinTWT, MaxTWT
  - Extract units from WITSML
  - Extract survey type from WITSML

Step 3.5: Resolve Wellbore Reference
WITSML Parser → Master Data Service:
  - Resolve wellbore reference from WITSML
  - Validate wellbore exists
  - Map to OSDU wellbore ID

┌─────────────────────────────────────────────────────────────┐
│ Phase 4: Wellbore DDMS Storage (Bulk Data)                  │
└─────────────────────────────────────────────────────────────┘

Step 4.1: Store Checkshot Stations in Wellbore DDMS
WITSML Parser → Wellbore DDMS API
POST /api/os-wellbore-ddms/wellbores/{wellboreId}/checkshots
Content-Type: application/json
Authorization: Bearer <JWT>

Request Body:
{
  "wellboreId": "osdu:master-data--Wellbore:001:",
  "stations": [
    {
      "md": 0.0,
      "tvd": 0.0,
      "twt": 0.0
    },
    {
      "md": 100.0,
      "tvd": 100.0,
      "twt": 0.05
    }
  ],
  "timeUnit": "osdu:reference-data--UnitOfMeasure:S:",
  "depthUnit": "osdu:reference-data--UnitOfMeasure:M:"
}

Response (200 OK):
{
  "checkshotId": "checkshot-002",
  "stationCount": 2,
  "status": "STORED"
}

┌─────────────────────────────────────────────────────────────┐
│ Phase 5: Storage Service Metadata Ingestion                │
└─────────────────────────────────────────────────────────────┘

Step 5.1: Create Checkshot WPC Record
WITSML Parser → Storage Service API
PUT /api/storage/v2/records
Content-Type: application/json
Authorization: Bearer <JWT>
Slb-Data-Partition-Id: osdu

Request Body:
[
  {
    "kind": "osdu:wks:work-product-component--Checkshot:1.0.0",
    "acl": {
      "viewers": ["data.default.viewers@osdu"],
      "owners": ["data.default.owners@osdu"]
    },
    "legal": {
      "legaltags": ["osdu-legaltag-001"],
      "otherRelevantDataCountries": ["US"]
    },
    "data": {
      "ResourceTypeID": "srn:type:work-product-component/Checkshot:",
      "WellboreId": "osdu:master-data--Wellbore:001:",
      "Name": "Well-001 Checkshot Survey (WITSML)",
      "Description": "Checkshot survey from WITSML",
      "SurveyType": "VSP",
      "SurveyTypeID": "osdu:reference-data--SurveyType:VSP:",
      "StationCount": 2,
      "MinMD": 0.0,
      "MaxMD": 100.0,
      "MinTVD": 0.0,
      "MaxTVD": 100.0,
      "MinTWT": 0.0,
      "MaxTWT": 0.05,
      "TimeUnitID": "osdu:reference-data--UnitOfMeasure:S:",
      "DepthUnitID": "osdu:reference-data--UnitOfMeasure:M:",
      "DataGroupTypeProperties": {
        "Files": [
          "srn:type:file/witsml:checkshot-002:"
        ]
      },
      "Datasets": [
        "osdu:dataset--File.Generic:checkshot-002:"
      ]
    }
  }
]

Response (200 OK):
[
  {
    "id": "osdu:doc:7ii90fh69dh5785e1hhcf6726i4if32f",
    "version": 1582725123640845
  }
]

┌─────────────────────────────────────────────────────────────┐
│ Phase 6: Indexing                                           │
└─────────────────────────────────────────────────────────────┘

Step 6.1: Indexer Service Processing
Indexer Service (Automatic):
  - Receives record creation event
  - Extracts searchable fields
  - Updates search index
```

### 5.3 Pathway 3: Manifest-Based Ingestion

**Architecture**:
```
JSON Manifest Creation
    ↓
Workflow Service API
    ↓
Manifest Parser
    ↓
Schema Validation
    ↓
Wellbore DDMS / Storage Service
    ↓
Indexer Service
```

**Complete Call Stack**:

```
┌─────────────────────────────────────────────────────────────┐
│ Phase 1: Manifest Creation                                   │
└─────────────────────────────────────────────────────────────┘

Step 1.1: Create Manifest JSON
Client (Application):
{
  "manifest": {
    "kind": "osdu:wks:Manifest:1.0.0",
    "Data": {
      "WorkProductComponents": [
        {
          "kind": "osdu:wks:work-product-component--Checkshot:1.0.0",
          "data": {
            "ResourceTypeID": "srn:type:work-product-component/Checkshot:",
            "WellboreId": "osdu:master-data--Wellbore:001:",
            "Name": "Well-001 Checkshot Survey",
            "SurveyType": "Checkshot",
            "SurveyTypeID": "osdu:reference-data--SurveyType:Checkshot:",
            "StationCount": 4,
            "MinMD": 0.0,
            "MaxMD": 300.0,
            "MinTVD": 0.0,
            "MaxTVD": 300.0,
            "MinTWT": 0.0,
            "MaxTWT": 0.15,
            "TimeUnitID": "osdu:reference-data--UnitOfMeasure:S:",
            "DepthUnitID": "osdu:reference-data--UnitOfMeasure:M:",
            "Stations": [
              {
                "md": 0.0,
                "tvd": 0.0,
                "twt": 0.0
              },
              {
                "md": 100.0,
                "tvd": 100.0,
                "twt": 0.05
              }
            ]
          }
        }
      ]
    }
  }
}

┌─────────────────────────────────────────────────────────────┐
│ Phase 2: Submit Manifest to Workflow Service               │
└─────────────────────────────────────────────────────────────┘

Step 2.1: Submit Manifest
Client → Workflow Service API
POST /v1/workflow/Osdu_ingest/workflowRun
Content-Type: application/json
Authorization: Bearer <JWT>
Slb-Data-Partition-Id: osdu

Request Body:
{
  "executionContext": {
    "Payload": {
      "AppKey": "manifest-ingestion",
      "data-partition-id": "osdu"
    },
    "manifest": {
      "kind": "osdu:wks:Manifest:1.0.0",
      "Data": {
        "WorkProductComponents": [ ... ]
      }
    }
  }
}

Response (200 OK):
{
  "workflowId": "workflow-003",
  "status": "RUNNING"
}

┌─────────────────────────────────────────────────────────────┐
│ Phase 3: Manifest Parser Processing                         │
└─────────────────────────────────────────────────────────────┘

Step 3.1: Validate Manifest Structure
Manifest Parser:
  - Validate manifest kind
  - Validate WorkProductComponents array
  - Validate each component structure

Step 3.2: Extract WorkProduct and WorkProductComponents
Manifest Parser:
  - Extract WorkProduct (if provided)
  - Extract WorkProductComponents
  - Validate component kinds

Step 3.3: Validate Checkshot Components
Manifest Parser:
  - For each checkshot component:
    * Validate WellboreId exists
    * Validate station data structure
    * Validate depth and time ranges
    * Validate units

Step 3.4: Resolve Reference Data
Manifest Parser → Reference Data Service:
  - Resolve SurveyTypeID
  - Resolve TimeUnitID
  - Resolve DepthUnitID
  - Validate all references exist

┌─────────────────────────────────────────────────────────────┐
│ Phase 4: Wellbore DDMS Storage (if stations provided)      │
└─────────────────────────────────────────────────────────────┘

Step 4.1: Store Checkshot Stations in Wellbore DDMS
Manifest Parser → Wellbore DDMS API
POST /api/os-wellbore-ddms/wellbores/{wellboreId}/checkshots
Content-Type: application/json
Authorization: Bearer <JWT>

Request Body:
{
  "wellboreId": "osdu:master-data--Wellbore:001:",
  "stations": [ ... ],
  "timeUnit": "osdu:reference-data--UnitOfMeasure:S:",
  "depthUnit": "osdu:reference-data--UnitOfMeasure:M:"
}

Response (200 OK):
{
  "checkshotId": "checkshot-003",
  "stationCount": 4,
  "status": "STORED"
}

┌─────────────────────────────────────────────────────────────┐
│ Phase 5: Storage Service Batch Ingestion                   │
└─────────────────────────────────────────────────────────────┘

Step 5.1: Batch Ingest Records
Manifest Parser → Storage Service API
PUT /api/storage/v2/records
Content-Type: application/json
Authorization: Bearer <JWT>
Slb-Data-Partition-Id: osdu

Request Body:
[
  {
    "kind": "osdu:wks:work-product:1.0.0",
    "data": {
      "Name": "Wellbore Survey Work Product",
      "WorkProductComponents": [
        "osdu:doc:8jj01gi70ei6896f2iidg7837j5jg43g"
      ]
    }
  },
  {
    "kind": "osdu:wks:work-product-component--Checkshot:1.0.0",
    "data": {
      "ResourceTypeID": "srn:type:work-product-component/Checkshot:",
      "WellboreId": "osdu:master-data--Wellbore:001:",
      "Name": "Well-001 Checkshot Survey",
      "SurveyType": "Checkshot",
      "StationCount": 4,
      "MinMD": 0.0,
      "MaxMD": 300.0,
      "MinTVD": 0.0,
      "MaxTVD": 300.0,
      "MinTWT": 0.0,
      "MaxTWT": 0.15,
      "TimeUnitID": "osdu:reference-data--UnitOfMeasure:S:",
      "DepthUnitID": "osdu:reference-data--UnitOfMeasure:M:"
    }
  }
]

Response (200 OK):
[
  {
    "id": "osdu:doc:9kk12hj81fj7907g3jjhe8948k6kh54h",
    "version": 1582725123640845
  },
  {
    "id": "osdu:doc:8jj01gi70ei6896f2iidg7837j5jg43g",
    "version": 1582725123640845
  }
]

┌─────────────────────────────────────────────────────────────┐
│ Phase 6: Indexing                                           │
└─────────────────────────────────────────────────────────────┘

Step 6.1: Indexer Service Processing
Indexer Service (Automatic):
  - Receives record creation events
  - Indexes all ingested records
  - Creates searchable metadata
```

### 5.4 Pathway 4: Direct Storage API Ingestion

**Architecture**:
```
Application Code
    ↓
Wellbore DDMS (bulk data)
    ↓
Storage Service API (metadata)
    ↓
Indexer Service
```

**Complete Call Stack**:

```
┌─────────────────────────────────────────────────────────────┐
│ Phase 1: Prepare Checkshot Data                            │
└─────────────────────────────────────────────────────────────┘

Step 1.1: Prepare Checkshot Stations
Application Code:
  const stations = [
    {
      md: 0.0,
      tvd: 0.0,
      twt: 0.0
    },
    {
      md: 100.0,
      tvd: 100.0,
      twt: 0.05
    },
    {
      md: 200.0,
      tvd: 200.0,
      twt: 0.10
    }
  ];

  const checkshotMetadata = {
    wellboreId: "osdu:master-data--Wellbore:001:",
    name: "Well-001 Checkshot Survey",
    surveyType: "Checkshot",
    stationCount: 3,
    minMD: 0.0,
    maxMD: 200.0,
    minTVD: 0.0,
    maxTVD: 200.0,
    minTWT: 0.0,
    maxTWT: 0.10,
    timeUnitID: "osdu:reference-data--UnitOfMeasure:S:",
    depthUnitID: "osdu:reference-data--UnitOfMeasure:M:"
  };

┌─────────────────────────────────────────────────────────────┐
│ Phase 2: Store in Wellbore DDMS                            │
└─────────────────────────────────────────────────────────────┘

Step 2.1: Store Checkshot Stations
Application → Wellbore DDMS API
POST /api/os-wellbore-ddms/wellbores/{wellboreId}/checkshots
Content-Type: application/json
Authorization: Bearer <JWT>

Request Body:
{
  "wellboreId": "osdu:master-data--Wellbore:001:",
  "stations": stations,
  "timeUnit": "osdu:reference-data--UnitOfMeasure:S:",
  "depthUnit": "osdu:reference-data--UnitOfMeasure:M:"
}

Response (200 OK):
{
  "checkshotId": "checkshot-004",
  "stationCount": 3,
  "status": "STORED"
}

┌─────────────────────────────────────────────────────────────┐
│ Phase 3: Create Metadata Record                            │
└─────────────────────────────────────────────────────────────┘

Step 3.1: Create Checkshot WPC Record
Application → Storage Service API
PUT /api/storage/v2/records
Content-Type: application/json
Authorization: Bearer <JWT>
Slb-Data-Partition-Id: osdu

Request Body:
[
  {
    "kind": "osdu:wks:work-product-component--Checkshot:1.0.0",
    "acl": {
      "viewers": ["data.default.viewers@osdu"],
      "owners": ["data.default.owners@osdu"]
    },
    "legal": {
      "legaltags": ["osdu-legaltag-001"],
      "otherRelevantDataCountries": ["US"]
    },
    "data": {
      "ResourceTypeID": "srn:type:work-product-component/Checkshot:",
      "WellboreId": checkshotMetadata.wellboreId,
      "Name": checkshotMetadata.name,
      "SurveyType": checkshotMetadata.surveyType,
      "SurveyTypeID": "osdu:reference-data--SurveyType:Checkshot:",
      "StationCount": checkshotMetadata.stationCount,
      "MinMD": checkshotMetadata.minMD,
      "MaxMD": checkshotMetadata.maxMD,
      "MinTVD": checkshotMetadata.minTVD,
      "MaxTVD": checkshotMetadata.maxTVD,
      "MinTWT": checkshotMetadata.minTWT,
      "MaxTWT": checkshotMetadata.maxTWT,
      "TimeUnitID": checkshotMetadata.timeUnitID,
      "DepthUnitID": checkshotMetadata.depthUnitID
    }
  }
]

Response (200 OK):
[
  {
    "id": "osdu:doc:0ll23ik92gk8018h4kkif9059l7li65i",
    "version": 1582725123640845
  }
]

┌─────────────────────────────────────────────────────────────┐
│ Phase 4: Indexing                                           │
└─────────────────────────────────────────────────────────────┘

Step 4.1: Indexer Service Processing
Indexer Service (Automatic):
  - Receives record creation event
  - Extracts searchable fields
  - Updates search index
```

---

## 6. Storage Architecture

### 6.1 Metadata vs. Bulk Data Separation

**Metadata Storage (Storage Service)**:

```json
{
  "id": "osdu:doc:4ff67ce36ae2452b8ddad3391f1fc08a",
  "version": 1582725123640845,
  "kind": "osdu:wks:work-product-component--Checkshot:1.0.0",
  "data": {
    "ResourceTypeID": "srn:type:work-product-component/Checkshot:",
    "WellboreId": "osdu:master-data--Wellbore:001:",
    "Name": "Well-001 Checkshot Survey",
    "Description": "Checkshot survey for velocity calibration",
    "SurveyType": "Checkshot",
    "SurveyTypeID": "osdu:reference-data--SurveyType:Checkshot:",
    "StationCount": 4,
    "MinMD": 0.0,
    "MaxMD": 300.0,
    "MinTVD": 0.0,
    "MaxTVD": 300.0,
    "MinTWT": 0.0,
    "MaxTWT": 0.15,
    "TimeUnitID": "osdu:reference-data--UnitOfMeasure:S:",
    "DepthUnitID": "osdu:reference-data--UnitOfMeasure:M:",
    "DataGroupTypeProperties": {
      "Files": [
        "srn:type:file/csv:checkshot-001:"
      ]
    },
    "Datasets": [
      "osdu:dataset--File.Generic:checkshot-001:"
    ]
  }
}
```

**Bulk Data Storage (Wellbore DDMS)**:

```
Wellbore DDMS Backend
    ↓
Wellbore: wellbore-001
    ↓
Checkshots:
    - checkshot-001.json (station array)
    {
      "checkshotId": "checkshot-001",
      "wellboreId": "osdu:master-data--Wellbore:001:",
      "stations": [
        {
          "md": 0.0,
          "tvd": 0.0,
          "twt": 0.0
        },
        {
          "md": 100.0,
          "tvd": 100.0,
          "twt": 0.05
        },
        {
          "md": 200.0,
          "tvd": 200.0,
          "twt": 0.10
        },
        {
          "md": 300.0,
          "tvd": 300.0,
          "twt": 0.15
        }
      ],
      "timeUnit": "osdu:reference-data--UnitOfMeasure:S:",
      "depthUnit": "osdu:reference-data--UnitOfMeasure:M:"
    }
    ↓
Indexed for Queries:
    - Depth range index (MinMD, MaxMD)
    - Time range index (MinTWT, MaxTWT)
    - Wellbore index for fast lookups
```

### 6.2 File Storage

**Source File Preservation**:

```
File Service (Object Store)
    ↓
Source File Storage:
    gs://osdu-storage/osdu/files/{file-id}/well-001-checkshot.csv
    gs://osdu-storage/osdu/files/{file-id}/well-001-checkshot.xml
    ↓
Dataset Record Reference:
    {
      "kind": "osdu:wks:dataset--File.Generic:1.0.0",
      "data": {
        "DatasetProperties": {
          "FileSourceInfo": {
            "FileSource": "gs://osdu-storage/osdu/files/{file-id}/well-001-checkshot.csv"
          }
        }
      }
    }
    ↓
Linked to Checkshot:
    Checkshot.data.DataGroupTypeProperties.Files = [
      "srn:type:file/csv:checkshot-001:"
    ]
```

### 6.3 Wellbore DDMS Architecture

**Bulk Data Storage**:

Wellbore DDMS provides optimized storage and access for checkshot bulk data:

**API Endpoints**:

```http
# Store checkshot
POST /api/os-wellbore-ddms/wellbores/{wellboreId}/checkshots
Content-Type: application/json

{
  "wellboreId": "osdu:master-data--Wellbore:001:",
  "stations": [
    {
      "md": 0.0,
      "tvd": 0.0,
      "twt": 0.0
    }
  ],
  "timeUnit": "osdu:reference-data--UnitOfMeasure:S:",
  "depthUnit": "osdu:reference-data--UnitOfMeasure:M:"
}

# Retrieve checkshot
GET /api/os-wellbore-ddms/wellbores/{wellboreId}/checkshots/{checkshotId}

# Query checkshots
GET /api/os-wellbore-ddms/wellbores/{wellboreId}/checkshots?minMD=0&maxMD=5000

# Query by time range
GET /api/os-wellbore-ddms/wellbores/{wellboreId}/checkshots?minTWT=0&maxTWT=1.0
```

**Storage Benefits**:

- **Optimized Access**: Efficient retrieval of large station arrays
- **Depth Indexing**: Fast queries by depth range (MD, TVD)
- **Time Indexing**: Fast queries by time range (TWT)
- **Compression**: Efficient storage of repetitive checkshot data
- **Caching**: In-memory caching for frequently accessed checkshots
- **Velocity Calculation**: Supports velocity model calibration

---

## 7. Data Transformation & Validation

### 7.1 Reference Data Resolution

**Survey Type Resolution**:

```
Input: "Checkshot" or "VSP" or "Vertical Seismic Profile"
    ↓
Reference Data Service Lookup
    GET /api/reference-data/v2/reference-data/osdu:reference-data--SurveyType:Checkshot:
    ↓
Response:
    {
      "code": "Checkshot",
      "name": "Checkshot Survey",
      "description": "Time-depth relationship survey"
    }
    ↓
Resolved Reference:
    "osdu:reference-data--SurveyType:Checkshot:"
```

**Unit of Measure Resolution**:

```
Input: "Second" or "S" or "s"
    ↓
Reference Data Service Lookup
    GET /api/reference-data/v2/reference-data/osdu:reference-data--UnitOfMeasure:S:
    ↓
Response:
    {
      "code": "S",
      "name": "Second",
      "symbol": "s"
    }
    ↓
Resolved Reference:
    "osdu:reference-data--UnitOfMeasure:S:"
```

### 7.2 Velocity Calculation

**Velocity from Time-Depth Relationship**:

```python
def calculate_velocity(stations):
    """
    Calculate velocity from time-depth relationship
    """
    velocities = []
    for i in range(1, len(stations)):
        prev_station = stations[i-1]
        curr_station = stations[i]
        
        # Calculate interval velocity
        depth_interval = curr_station['tvd'] - prev_station['tvd']
        time_interval = curr_station['twt'] - prev_station['twt']
        
        if time_interval > 0:
            velocity = depth_interval / time_interval
            velocities.append({
                "md": curr_station['md'],
                "velocity": velocity
            })
    
    return velocities
```

---

## 8. Search & Discovery

### 8.1 Indexing Process

```
Record Creation Event
    ↓
Indexer Service
    ↓
Extract Searchable Fields:
    - WellboreId → Filter by wellbore
    - Name → Full-text search
    - SurveyType → Filter by type
    - StationCount → Filter by size
    - MinMD, MaxMD → Depth range queries
    - MinTVD, MaxTVD → TVD range queries
    - MinTWT, MaxTWT → Time range queries
    ↓
Update Search Index
    ↓
Search Service Available
```

### 8.2 Query Examples

**Search by Wellbore**:

```http
POST /api/search/v2/query
Content-Type: application/json
Authorization: Bearer <JWT>
Slb-Data-Partition-Id: osdu

{
  "kind": "osdu:wks:work-product-component--Checkshot:1.0.0",
  "query": "data.WellboreId:\"osdu:master-data--Wellbore:001:\""
}
```

**Search by Depth Range**:

```http
POST /api/search/v2/query
Content-Type: application/json
Authorization: Bearer <JWT>
Slb-Data-Partition-Id: osdu

{
  "kind": "osdu:wks:work-product-component--Checkshot:1.0.0",
  "query": "data.MinMD:[0 TO 1000] AND data.MaxMD:[500 TO 2000]"
}
```

**Search by Time Range**:

```http
POST /api/search/v2/query
Content-Type: application/json
Authorization: Bearer <JWT>
Slb-Data-Partition-Id: osdu

{
  "kind": "osdu:wks:work-product-component--Checkshot:1.0.0",
  "query": "data.MinTWT:[0 TO 0.5] AND data.MaxTWT:[0.3 TO 1.0]"
}
```

**Search by Survey Type**:

```http
POST /api/search/v2/query
Content-Type: application/json
Authorization: Bearer <JWT>
Slb-Data-Partition-Id: osdu

{
  "kind": "osdu:wks:work-product-component--Checkshot:1.0.0",
  "query": "data.SurveyType:\"Checkshot\""
}
```

---

## 9. Error Handling & Validation

### 9.1 Validation Errors

**Common Validation Failures**:

1. **Missing Required Columns**:
   ```
   Error: Required columns MD, TVD, TWT not found in CSV
   Resolution: Ensure CSV contains MD, TVD, TWT columns
   ```

2. **Invalid Depth Range**:
   ```
   Error: TVD value 60000 exceeds maximum range (50000)
   Resolution: Validate TVD ranges before ingestion
   ```

3. **Invalid Time Range**:
   ```
   Error: TWT value -0.5 is negative
   Resolution: TWT must be non-negative (0 to 100)
   ```

4. **Missing Wellbore Reference**:
   ```
   Error: WellboreId not specified or invalid
   Resolution: Provide valid WellboreId reference
   ```

5. **Invalid Unit Reference**:
   ```
   Error: TimeUnitID reference not found
   Resolution: Ensure unit reference exists in Reference Data Service
   ```

### 9.2 Schema Validation

**Required Field Validation**:
- Fields marked as required in schema must be present
- Missing required fields cause ingestion failure

**Data Type Validation**:
- String fields must be strings
- Numeric fields must be numbers
- Array fields must be arrays
- Object fields must be objects

**Reference Validation**:
- Master data references (WellboreId) must exist
- Reference data values (SurveyTypeID, UnitIDs) must be valid
- Dataset references must exist

**Example Validation Error**:

```json
{
  "error": "Schema validation failed",
  "kind": "osdu:wks:work-product-component--Checkshot:1.0.0",
  "errors": [
    {
      "path": "data.WellboreId",
      "message": "Required field missing"
    },
    {
      "path": "data.StationCount",
      "message": "Expected Integer, got String"
    },
    {
      "path": "data.MinMD",
      "message": "Expected Double, got String"
    }
  ]
}
```

---

## 10. Performance Considerations

### 10.1 Batch Ingestion

**Optimal Batch Size**:

- **Small checkshots (< 100 stations)**: Process individually
- **Medium checkshots (100-1000 stations)**: Batch by wellbore
- **Large checkshots (> 1000 stations)**: Chunk processing recommended

**Recommended**: Process checkshots individually, batch metadata records (100-200 per batch)

### 10.2 Station Array Processing

**Efficient Station Processing**:

```python
def process_checkshot_stations(stations, chunk_size=1000):
    """
    Process checkshot stations in chunks
    """
    # 1. Validate all stations first
    validate_stations(stations)
    
    # 2. Calculate statistics
    stats = calculate_statistics(stations)
    
    # 3. Process in chunks if large
    if len(stations) > chunk_size:
        chunks = [stations[i:i+chunk_size] for i in range(0, len(stations), chunk_size)]
        for chunk in chunks:
            wellbore_ddms.store_checkshot_chunk(chunk)
    else:
        wellbore_ddms.store_checkshot_stations(stations)
    
    return stats
```

---

## 11. Design Patterns and Best Practices

### 11.1 Schema First Approach

1. **Define schemas before ingestion**: Ensures type safety and proper indexing
2. **Version schemas carefully**: Schema changes require data migration
3. **Index only necessary fields**: Reduces index size and improves performance
4. **Validate at ingestion**: Catch errors early in the pipeline

### 11.2 Work Product Pattern

1. **Group related WPCs**: Use Work Products to organize related checkshots
2. **Maintain relationships**: Link checkshots to wellbores and surveys
3. **Preserve lineage**: Track source files and processing history

### 11.3 Wellbore DDMS Usage

1. **Use Wellbore DDMS for bulk data**: Optimized storage for station arrays
2. **Store metadata separately**: Keep metadata in Storage Service for searchability
3. **Leverage indexing**: Use depth and time range indexes for efficient queries

### 11.4 Unit Standardization

1. **Use reference data units**: Always use Unit of Measure reference data
2. **Specify units explicitly**: Always provide TimeUnitID and DepthUnitID
3. **Handle unit conversions**: Convert units when necessary

---

## 12. Important Considerations

### 12.1 Data Partitioning

- All checkshot operations require `data-partition-id` header
- Data is scoped to partitions for multi-tenancy
- Cross-partition queries are not supported

### 12.2 Access Control

- ACLs (Access Control Lists) must be specified at ingestion
- Legal tags are required for compliance
- Viewers and owners must be specified

### 12.3 Versioning

- Records are versioned automatically
- Each update creates a new version
- Version history is maintained

### 12.4 File Source References

- File sources use SRN (Storage Resource Name) format
- Files must be uploaded to File Service before referencing
- File sources are validated during ingestion

### 12.5 Wellbore Association

- Each checkshot must be associated with a Wellbore
- WellboreId must exist in Master Data Service
- Wellbore reference is validated during ingestion

---

## 13. Summary

OSDU provides **comprehensive, multi-format ingestion** for checkshot data through specialized services:

1. **Wellbore DDMS**: Optimized bulk storage for checkshot station arrays
2. **CSV Parser**: Automated ingestion from CSV files with time-depth relationships
3. **WITSML Parser**: Industry-standard WITSML format support
4. **Manifest-based**: Structured JSON manifests for complex workflows
5. **Direct API**: Programmatic record creation for custom applications

**Key Design Decisions**:

- **Metadata/Bulk Separation**: Metadata in Storage Service, bulk data in Wellbore DDMS
- **Station-Based Model**: Checkshots represented as arrays of stations (MD, TVD, TWT)
- **Wellbore Association**: Each checkshot linked to a specific Wellbore
- **Unit Standardization**: Uses OSDU Unit of Measure reference data
- **Source File Preservation**: Original files stored alongside processed data
- **Search Integration**: Automatic indexing for discovery and range queries
- **Access Control**: ACLs and legal tags enforced at ingestion time

**Call Stack Highlights**:

- **CSV Parser**: File upload → DAG trigger → CSV parsing → Station extraction → Wellbore DDMS storage → Metadata ingest → Index
- **WITSML Parser**: File upload → DAG trigger → WITSML parsing → Station extraction → Wellbore DDMS storage → Metadata ingest → Index
- **Manifest**: JSON creation → Workflow API → Parse & validate → Wellbore DDMS/Storage → Batch ingest → Index
- **Direct API**: Application code → Wellbore DDMS → Storage API → Index

This architecture ensures **data consistency**, **velocity accuracy**, **traceability**, and **governance** while supporting multiple ingestion formats and checkshot types.

---

## 14. References

1. OSDU Data Formats, Schemas & Storage Architecture Documentation
2. OSDU Well Trajectory Ingestion & Storage Architecture Documentation
3. OSDU Storage Service API Documentation
4. OSDU Wellbore DDMS API Documentation
5. OSDU Workflow Service API Documentation
6. OSDU File Service API Documentation
7. OSDU Search Service API Documentation
8. OSDU Reference Data Service API Documentation
9. WITSML 2.0 Specification
10. OSDU Major Data Types Overview Documentation

