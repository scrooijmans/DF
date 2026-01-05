# OSDU Well Delivery/Planning Ingestion Analysis: Architecture, Schema, and Ingestion Flow

## Executive Summary

This document provides a comprehensive analysis of how well delivery and planning data types (Well Planning, Well Execution, and Well Completions) are designed, stored, processed, and ingested in the OSDU (Open Subsurface Data Universe) platform. It covers the complete architecture, schema definitions, and detailed ingestion workflows with call stacks for all supported formats and methods.

---

## 1. Well Delivery/Planning Architecture Overview

### 1.1 Conceptual Overview

**Well Delivery and Planning** data types in OSDU represent the lifecycle of well operations from planning through execution to completion:

1. **Well Planning**: Well design, drilling plans, and operational planning
2. **Well Execution**: Drilling operations, daily drilling reports, and operational events
3. **Well Completions**: Completion design, casing, tubing, and completion equipment

These data types are represented as **Work Product Components (WPC)** in the OSDU data model, following the Work Product pattern, and are managed by the **Well Delivery DDMS** (Domain Data Management Service).

### 1.2 OSDU Data Model Hierarchy

Well delivery/planning data fits into the OSDU data model hierarchy as follows:

```
┌─────────────────────────────────────────┐
│ Master Data                             │
│ - Well                                  │
│ - Wellbore                              │
│ - Wellbore Activity                     │
└──────────────┬──────────────────────────┘
               │
               ▼
┌─────────────────────────────────────────┐
│ Work Product (WP)                       │
│ - Groups related WPCs                   │
│ - Example: Well Delivery WP              │
└──────────────┬──────────────────────────┘
               │
               ▼
┌─────────────────────────────────────────┐
│ Work Product Component (WPC)            │
│ - WellPlanning                          │
│ - WellExecution                         │
│ - WellCompletion                        │
└──────────────┬──────────────────────────┘
               │
               ▼
┌─────────────────────────────────────────┐
│ Dataset                                 │
│ - References source files                │
│   (WITSML, CSV, Documents)               │
│ - Links to WPC                          │
└─────────────────────────────────────────┘
```

### 1.3 Key Architectural Components

1. **Work Product (WP)**: Container that groups one or more well delivery WPCs
2. **Work Product Component (WPC)**: The actual records (WellPlanning, WellExecution, WellCompletion)
3. **Dataset**: References to source files (WITSML, CSV, PDF, Word documents)
4. **Master Data References**: Links to Well, Wellbore, and WellboreActivity entities
5. **Well Delivery DDMS**: Specialized service for optimized bulk storage of well delivery data
6. **Reference Data**: Activity types, completion types, equipment types

### 1.4 Design Principles

- **Schema First**: Well delivery data requires schema definition before ingestion
- **Work Product Pattern**: Follows OSDU Work Product/Work Product Component pattern
- **Metadata/Bulk Separation**: Metadata in Storage Service, bulk data in Well Delivery DDMS
- **Lifecycle Management**: Supports well lifecycle from planning through execution to completion
- **Document Management**: Handles both structured data (CSV, WITSML) and unstructured documents (PDF, Word)
- **Time-Based Data**: Supports time-series operational data and event-based records
- **Multi-Format Support**: WITSML, CSV, JSON, and document formats

---

## 2. Well Delivery Data Types

### 2.1 Well Planning

**Description**: Well planning data including well design, drilling plans, and operational planning.

**Kind Identifier Pattern**:
```
{authority}:wks:work-product-component--WellPlanning:{version}
```

**Example**: `osdu:wks:work-product-component--WellPlanning:1.0.0`

**Key Characteristics**:
- **Planning Data**: Well design and drilling plans
- **Operational Planning**: Drilling sequence and procedures
- **Time-Based**: Planning phases and milestones
- **Documentation**: Planning documents and reports

**Key Fields**:
- `WellboreId`: Reference to wellbore
- `PlanningPhase`: Planning phase (Conceptual, Detailed, etc.)
- `PlanningDate`: Date of planning
- `WellDesign`: Well design details
- `DrillingPlan`: Drilling plan information
- `Documents`: References to planning documents

### 2.2 Well Execution

**Description**: Well execution data including drilling operations, daily drilling reports, and operational events.

**Kind Identifier Pattern**:
```
{authority}:wks:work-product-component--WellExecution:{version}
```

**Example**: `osdu:wks:work-product-component--WellExecution:1.0.0`

**Key Characteristics**:
- **Operational Data**: Drilling operations and events
- **Time Series**: Daily or event-based data
- **Operations**: Drilling, completion, workover operations
- **Reports**: Daily drilling reports (DDR)

**Key Fields**:
- `WellboreId`: Reference to wellbore
- `ActivityType`: Type of activity (Drilling, Completion, Workover)
- `StartDate`: Activity start date
- `EndDate`: Activity end date
- `OperationalEvents`: Array of operational events
- `DailyReports`: References to daily drilling reports
- `NonProductiveTime`: NPT information

### 2.3 Well Completions

**Description**: Well completion design and configuration data including casing, tubing, and completion equipment.

**Kind Identifier Pattern**:
```
{authority}:wks:work-product-component--WellCompletion:{version}
```

**Example**: `osdu:wks:work-product-component--WellCompletion:1.0.0`

**Key Characteristics**:
- **Completion Design**: Casing, tubing, packers, etc.
- **Depth Intervals**: Completion intervals and equipment depths
- **Equipment Details**: Completion equipment specifications
- **Time-Varying**: Completion changes over time

**Key Fields**:
- `WellboreId`: Reference to wellbore
- `CompletionType`: Type of completion (Open Hole, Cased Hole, etc.)
- `CompletionIntervals`: Array of completion intervals
- `EquipmentDetails`: Completion equipment specifications
- `EffectiveDate`: Date when completion became effective
- `CasingDetails`: Casing string details
- `TubingDetails`: Tubing string details

---

## 3. Schema Definitions

### 3.1 Understanding "Schema First" Approach

OSDU requires **schema definition before ingestion** for all well delivery data types. This ensures:

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

### 3.2 WellPlanning Schema

```json
{
  "kind": "osdu:wks:work-product-component--WellPlanning:1.0.0",
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
      "path": "data.PlanningPhase",
      "kind": "core:String:1.0.0"
    },
    {
      "path": "data.PlanningPhaseID",
      "kind": "core:String:1.0.0"
    },
    {
      "path": "data.PlanningDate",
      "kind": "core:dl:date:1.0.0"
    },
    {
      "path": "data.WellDesign",
      "kind": "core:Object:1.0.0"
    },
    {
      "path": "data.DrillingPlan",
      "kind": "core:Object:1.0.0"
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

### 3.3 WellExecution Schema

```json
{
  "kind": "osdu:wks:work-product-component--WellExecution:1.0.0",
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
      "path": "data.ActivityType",
      "kind": "core:String:1.0.0"
    },
    {
      "path": "data.ActivityTypeID",
      "kind": "core:String:1.0.0"
    },
    {
      "path": "data.StartDate",
      "kind": "core:dl:datetime:1.0.0"
    },
    {
      "path": "data.EndDate",
      "kind": "core:dl:datetime:1.0.0"
    },
    {
      "path": "data.OperationalEvents",
      "kind": "core:Array:1.0.0"
    },
    {
      "path": "data.NonProductiveTime",
      "kind": "core:Object:1.0.0"
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

### 3.4 WellCompletion Schema

```json
{
  "kind": "osdu:wks:work-product-component--WellCompletion:1.0.0",
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
      "path": "data.CompletionType",
      "kind": "core:String:1.0.0"
    },
    {
      "path": "data.CompletionTypeID",
      "kind": "core:String:1.0.0"
    },
    {
      "path": "data.CompletionIntervals",
      "kind": "core:Array:1.0.0"
    },
    {
      "path": "data.EquipmentDetails",
      "kind": "core:Object:1.0.0"
    },
    {
      "path": "data.EffectiveDate",
      "kind": "core:dl:date:1.0.0"
    },
    {
      "path": "data.CasingDetails",
      "kind": "core:Object:1.0.0"
    },
    {
      "path": "data.TubingDetails",
      "kind": "core:Object:1.0.0"
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

### 3.5 Schema API

**Create Schema**:
```http
POST /api/storage/v2/schemas
Content-Type: application/json
Authorization: Bearer <JWT>
Slb-Data-Partition-Id: osdu

{
  "kind": "osdu:wks:work-product-component--WellPlanning:1.0.0",
  "schema": [
    {
      "path": "data.WellboreId",
      "kind": "core:String:1.0.0"
    },
    {
      "path": "data.PlanningPhase",
      "kind": "core:String:1.0.0"
    },
    {
      "path": "data.PlanningDate",
      "kind": "core:dl:date:1.0.0"
    }
  ]
}
```

**Get Schema**:
```http
GET /api/storage/v2/schemas/osdu:wks:work-product-component--WellPlanning:1.0.0
Authorization: Bearer <JWT>
Slb-Data-Partition-Id: osdu
```

---

## 4. Supported Data Formats

### 4.1 WITSML Format

WITSML (Wellsite Information Transfer Standard Markup Language) for well delivery data:

**Structure**:
- **XML-based**: Industry standard XML format
- **Well Planning**: WITSML 2.0 well planning objects
- **Well Execution**: WITSML 2.0 drilling operations and daily reports
- **Well Completions**: WITSML 2.0 completion objects

**Key Elements**:
- `wellPlanning`: Root element for well planning
- `drillingOperation`: Drilling operation data
- `dailyReport`: Daily drilling report
- `completion`: Completion design and equipment
- `wellbore`: Reference to wellbore
- `activity`: Activity information

**Storage Strategy**: WITSML files are preserved in File Service, with structured data parsed and stored in Well Delivery DDMS.

### 4.2 CSV Format

CSV files for well delivery data:

**Well Planning CSV Structure**:
```csv
WellboreId,PlanningPhase,PlanningDate,WellDesign,DrillingPlan
wellbore-001,Detailed,2024-01-15,Design JSON,Plan JSON
```

**Well Execution CSV Structure**:
```csv
WellboreId,ActivityType,StartDate,EndDate,EventType,EventDescription
wellbore-001,Drilling,2024-01-20,2024-01-25,Drilling,Drilling section 1
```

**Well Completion CSV Structure**:
```csv
WellboreId,CompletionType,EffectiveDate,TopDepth,BottomDepth,EquipmentType
wellbore-001,Cased Hole,2024-02-01,0,2000,Casing
```

**Storage Strategy**: CSV files are preserved in File Service, with structured data extracted and stored in Well Delivery DDMS.

### 4.3 Document Formats

Document files for well delivery data:

**Supported Formats**:
- **PDF**: Planning documents, daily reports, completion reports
- **Word (.doc, .docx)**: Planning documents, operational procedures
- **Excel (.xls, .xlsx)**: Planning spreadsheets, operational data

**Storage Strategy**: Documents are preserved in File Service, with metadata extracted and stored in Storage Service. Documents may be indexed for full-text search.

### 4.4 JSON Format

JSON files for structured well delivery data:

**Structure**:
- **Well Planning**: Structured JSON with planning data
- **Well Execution**: Structured JSON with operational events
- **Well Completions**: Structured JSON with completion design

**Storage Strategy**: JSON files are preserved in File Service, with data parsed and stored in Well Delivery DDMS.

---

## 5. Ingestion Methods

OSDU supports multiple ingestion pathways for well delivery/planning data:

1. **CSV Parser Ingestion** (Airflow DAG)
2. **WITSML Parser Ingestion** (Energistics integration)
3. **Manifest-based Ingestion** (Workflow Service)
4. **Direct Storage API Ingestion** (Programmatic)
5. **Document Ingestion** (File Service + Metadata extraction)

---

## 6. Ingestion Flow: Complete Walkthrough

### 6.1 Pathway 1: CSV Well Planning Ingestion

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
Well Delivery DDMS Storage (bulk data)
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
  - file: well-001-planning.csv (binary)

Response (200 OK):
{
  "FileID": "4ff67ce36ae2452b8ddad3391f1fc08a",
  "FileSource": "srn:type:file/csv:planning-001:",
  "Location": "gs://osdu-storage/osdu/files/4ff67ce36ae2452b8ddad3391f1fc08a/well-001-planning.csv"
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
      "FileSource": "srn:type:file/csv:planning-001:",
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
  - Validate required columns (WellboreId, PlanningPhase, PlanningDate)
  - Detect optional columns (WellDesign, DrillingPlan)

Step 3.3: Parse CSV Data Rows
Airflow DAG (Python):
  - For each data row:
    * Extract WellboreId, PlanningPhase, PlanningDate
    * Parse WellDesign JSON (if provided)
    * Parse DrillingPlan JSON (if provided)
    * Validate data types and formats
    * Store in planning records array

Step 3.4: Generate Planning Metadata
Airflow DAG (Python):
  - Extract planning name from filename or CSV
  - Resolve PlanningPhaseID from reference data
  - Validate WellboreId exists
  - Generate description if not provided

┌─────────────────────────────────────────────────────────────┐
│ Phase 4: Well Delivery DDMS Storage (Bulk Data)            │
└─────────────────────────────────────────────────────────────┘

Step 4.1: Store Planning Data in Well Delivery DDMS
Airflow DAG → Well Delivery DDMS API
POST /api/os-well-delivery-ddms/wellbores/{wellboreId}/planning
Content-Type: application/json
Authorization: Bearer <JWT>

Request Body:
{
  "wellboreId": "osdu:master-data--Wellbore:001:",
  "planningData": {
    "planningPhase": "Detailed",
    "planningDate": "2024-01-15",
    "wellDesign": {
      "trajectory": "...",
      "casing": "..."
    },
    "drillingPlan": {
      "sections": [...],
      "procedures": [...]
    }
  }
}

Response (200 OK):
{
  "planningId": "planning-001",
  "status": "STORED"
}

┌─────────────────────────────────────────────────────────────┐
│ Phase 5: Storage Service Metadata Ingestion                │
└─────────────────────────────────────────────────────────────┘

Step 5.1: Create WellPlanning WPC Record
Airflow DAG → Storage Service API
PUT /api/storage/v2/records
Content-Type: application/json
Authorization: Bearer <JWT>
Slb-Data-Partition-Id: osdu

Request Body:
[
  {
    "kind": "osdu:wks:work-product-component--WellPlanning:1.0.0",
    "acl": {
      "viewers": ["data.default.viewers@osdu"],
      "owners": ["data.default.owners@osdu"]
    },
    "legal": {
      "legaltags": ["osdu-legaltag-001"],
      "otherRelevantDataCountries": ["US"]
    },
    "data": {
      "ResourceTypeID": "srn:type:work-product-component/WellPlanning:",
      "WellboreId": "osdu:master-data--Wellbore:001:",
      "Name": "Well-001 Detailed Planning",
      "Description": "Detailed well planning for Well-001",
      "PlanningPhase": "Detailed",
      "PlanningPhaseID": "osdu:reference-data--PlanningPhase:Detailed:",
      "PlanningDate": "2024-01-15",
      "WellDesign": {
        "trajectory": "...",
        "casing": "..."
      },
      "DrillingPlan": {
        "sections": [...],
        "procedures": [...]
      },
      "DataGroupTypeProperties": {
        "Files": [
          "srn:type:file/csv:planning-001:"
        ]
      },
      "Datasets": [
        "osdu:dataset--File.Generic:planning-001:"
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
    * PlanningPhase
    * PlanningDate
  - Updates search index
```

**CSV Parser DAG Implementation**:

```python
# csv_well_planning_ingestion_dag.py (Airflow DAG)
from airflow import DAG
from airflow.operators.python import PythonOperator
from datetime import datetime

def parse_csv_well_planning(file_source, wellbore_id, data_partition_id):
    """
    Parse CSV file and create well planning records
    """
    # 1. Read CSV from staging
    csv_data = file_service.read(file_source)

    # 2. Parse CSV
    import csv
    import json
    reader = csv.DictReader(csv_data)

    # 3. Extract planning records
    planning_records = []
    for row in reader:
        planning_record = {
            "wellboreId": row['WellboreId'],
            "planningPhase": row['PlanningPhase'],
            "planningDate": row['PlanningDate'],
            "wellDesign": json.loads(row['WellDesign']) if row.get('WellDesign') else None,
            "drillingPlan": json.loads(row['DrillingPlan']) if row.get('DrillingPlan') else None
        }
        planning_records.append(planning_record)

    # 4. Resolve reference data
    planning_phase_id = reference_data_service.resolve(
        f"{data_partition_id}:reference-data--PlanningPhase:{planning_record['planningPhase']}:"
    )

    # 5. Extract planning name from filename
    planning_name = extract_planning_name(file_source)

    # 6. Store in Well Delivery DDMS
    planning_id = well_delivery_ddms.store_planning(
        wellbore_id=wellbore_id,
        planning_data=planning_records[0]
    )

    # 7. Create planning record
    storage_service.ingest_records([{
        "kind": f"{data_partition_id}:wks:work-product-component--WellPlanning:1.0.0",
        "data": {
            "ResourceTypeID": "srn:type:work-product-component/WellPlanning:",
            "WellboreId": wellbore_id,
            "Name": planning_name,
            "PlanningPhase": planning_records[0]['planningPhase'],
            "PlanningPhaseID": planning_phase_id,
            "PlanningDate": planning_records[0]['planningDate'],
            "WellDesign": planning_records[0]['wellDesign'],
            "DrillingPlan": planning_records[0]['drillingPlan'],
            "DataGroupTypeProperties": {
                "Files": [file_source]
            }
        }
    }])

    return {"status": "success", "planning_id": planning_id}

with DAG(
    dag_id='csv_well_planning_ingestion',
    start_date=datetime(2023, 1, 1),
    schedule_interval=None,
    catchup=False,
    tags=['osdu', 'well-planning', 'csv', 'ingestion'],
) as dag:
    parse_task = PythonOperator(
        task_id='parse_csv_well_planning',
        python_callable=parse_csv_well_planning,
        op_kwargs={
            'file_source': '{{ dag_run.conf.file_source }}',
            'wellbore_id': '{{ dag_run.conf.wellbore_id }}',
            'data_partition_id': '{{ dag_run.conf.data_partition_id }}'
        }
    )
```

### 6.2 Pathway 2: WITSML Well Execution Ingestion

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
Well Delivery DDMS Storage (bulk data)
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
  - file: well-001-execution.xml (binary)

Response (200 OK):
{
  "FileID": "6hh89eg58cg4674d0ggbe5615h3he21e",
  "FileSource": "srn:type:file/witsml:execution-001:",
  "Location": "gs://osdu-storage/osdu/files/6hh89eg58cg4674d0ggbe5615h3he21e/well-001-execution.xml"
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
      "FileSource": "srn:type:file/witsml:execution-001:",
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
  - Extract drillingOperation elements
  - Extract dailyReport elements
  - Extract wellbore reference
  - Extract activity metadata:
    * Activity type
    * Start date
    * End date
    * Operational events

Step 3.3: Parse Operational Events
WITSML Parser (Energistics):
  - For each operational event:
    * Extract event type
    * Extract event description
    * Extract event timestamp
    * Extract event details
    * Store in events array

Step 3.4: Parse Daily Reports
WITSML Parser (Energistics):
  - For each daily report:
    * Extract report date
    * Extract report content
    * Extract operational metrics
    * Store in reports array

Step 3.5: Generate Execution Metadata
WITSML Parser:
  - Calculate statistics:
    * Total duration
    * Number of events
    * Non-productive time
  - Extract activity type from WITSML
  - Resolve ActivityTypeID from reference data

Step 3.6: Resolve Wellbore Reference
WITSML Parser → Master Data Service:
  - Resolve wellbore reference from WITSML
  - Validate wellbore exists
  - Map to OSDU wellbore ID

┌─────────────────────────────────────────────────────────────┐
│ Phase 4: Well Delivery DDMS Storage (Bulk Data)            │
└─────────────────────────────────────────────────────────────┘

Step 4.1: Store Execution Data in Well Delivery DDMS
WITSML Parser → Well Delivery DDMS API
POST /api/os-well-delivery-ddms/wellbores/{wellboreId}/execution
Content-Type: application/json
Authorization: Bearer <JWT>

Request Body:
{
  "wellboreId": "osdu:master-data--Wellbore:001:",
  "executionData": {
    "activityType": "Drilling",
    "startDate": "2024-01-20T00:00:00Z",
    "endDate": "2024-01-25T00:00:00Z",
    "operationalEvents": [
      {
        "eventType": "Drilling",
        "eventDescription": "Drilling section 1",
        "timestamp": "2024-01-20T08:00:00Z"
      }
    ],
    "dailyReports": [
      {
        "reportDate": "2024-01-20",
        "content": "..."
      }
    ],
    "nonProductiveTime": {
      "totalHours": 12,
      "events": [...]
    }
  }
}

Response (200 OK):
{
  "executionId": "execution-001",
  "status": "STORED"
}

┌─────────────────────────────────────────────────────────────┐
│ Phase 5: Storage Service Metadata Ingestion                │
└─────────────────────────────────────────────────────────────┘

Step 5.1: Create WellExecution WPC Record
WITSML Parser → Storage Service API
PUT /api/storage/v2/records
Content-Type: application/json
Authorization: Bearer <JWT>
Slb-Data-Partition-Id: osdu

Request Body:
[
  {
    "kind": "osdu:wks:work-product-component--WellExecution:1.0.0",
    "acl": {
      "viewers": ["data.default.viewers@osdu"],
      "owners": ["data.default.owners@osdu"]
    },
    "legal": {
      "legaltags": ["osdu-legaltag-001"],
      "otherRelevantDataCountries": ["US"]
    },
    "data": {
      "ResourceTypeID": "srn:type:work-product-component/WellExecution:",
      "WellboreId": "osdu:master-data--Wellbore:001:",
      "Name": "Well-001 Drilling Execution",
      "Description": "Drilling execution for Well-001",
      "ActivityType": "Drilling",
      "ActivityTypeID": "osdu:reference-data--ActivityType:Drilling:",
      "StartDate": "2024-01-20T00:00:00Z",
      "EndDate": "2024-01-25T00:00:00Z",
      "OperationalEvents": [
        {
          "eventType": "Drilling",
          "eventDescription": "Drilling section 1",
          "timestamp": "2024-01-20T08:00:00Z"
        }
      ],
      "NonProductiveTime": {
        "totalHours": 12,
        "events": [...]
      },
      "DataGroupTypeProperties": {
        "Files": [
          "srn:type:file/witsml:execution-001:"
        ]
      },
      "Datasets": [
        "osdu:dataset--File.Generic:execution-001:"
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

### 6.3 Pathway 3: Document Ingestion (Well Planning)

**Architecture**:
```
Document Upload (PDF/Word)
    ↓
File Service (staging)
    ↓
Document Parser
    ↓
Metadata Extraction
    ↓
Storage Service (metadata)
    ↓
Indexer Service (full-text search)
```

**Complete Call Stack**:

```
┌─────────────────────────────────────────────────────────────┐
│ Phase 1: Document Upload                                      │
└─────────────────────────────────────────────────────────────┘

Step 1.1: Upload Document
Client → File Service API
POST /v2/files/upload
Content-Type: multipart/form-data
Authorization: Bearer <JWT>
Slb-Data-Partition-Id: osdu

Request Body (multipart/form-data):
  - file: well-001-planning.pdf (binary)

Response (200 OK):
{
  "FileID": "8jj01gi70ei6896f2iidg7837j5jg43g",
  "FileSource": "srn:type:file/pdf:planning-doc-001:",
  "Location": "gs://osdu-storage/osdu/files/8jj01gi70ei6896f2iidg7837j5jg43g/well-001-planning.pdf"
}

┌─────────────────────────────────────────────────────────────┐
│ Phase 2: Document Processing                                 │
└─────────────────────────────────────────────────────────────┘

Step 2.1: Extract Document Metadata
Document Parser:
  - Extract document title
  - Extract document author
  - Extract creation date
  - Extract wellbore reference (if in metadata)
  - Extract planning phase (if in metadata)

Step 2.2: Extract Document Content (Optional)
Document Parser:
  - Extract text content for full-text search
  - Extract structured data (if available)
  - Generate document summary

┌─────────────────────────────────────────────────────────────┐
│ Phase 3: Storage Service Metadata Ingestion                │
└─────────────────────────────────────────────────────────────┘

Step 3.1: Create WellPlanning WPC Record
Document Parser → Storage Service API
PUT /api/storage/v2/records
Content-Type: application/json
Authorization: Bearer <JWT>
Slb-Data-Partition-Id: osdu

Request Body:
[
  {
    "kind": "osdu:wks:work-product-component--WellPlanning:1.0.0",
    "acl": {
      "viewers": ["data.default.viewers@osdu"],
      "owners": ["data.default.owners@osdu"]
    },
    "legal": {
      "legaltags": ["osdu-legaltag-001"],
      "otherRelevantDataCountries": ["US"]
    },
    "data": {
      "ResourceTypeID": "srn:type:work-product-component/WellPlanning:",
      "WellboreId": "osdu:master-data--Wellbore:001:",
      "Name": "Well-001 Planning Document",
      "Description": "Well planning document for Well-001",
      "PlanningPhase": "Detailed",
      "PlanningPhaseID": "osdu:reference-data--PlanningPhase:Detailed:",
      "PlanningDate": "2024-01-15",
      "DataGroupTypeProperties": {
        "Files": [
          "srn:type:file/pdf:planning-doc-001:"
        ]
      },
      "Datasets": [
        "osdu:dataset--File.Generic:planning-doc-001:"
      ]
    }
  }
]

Response (200 OK):
[
  {
    "id": "osdu:doc:9kk12hj81fj7907g3jjhe8948k6kh54h",
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
  - Indexes document content for full-text search
  - Updates search index
```

### 6.4 Pathway 4: Manifest-Based Ingestion (Well Completion)

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
Well Delivery DDMS / Storage Service
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
          "kind": "osdu:wks:work-product-component--WellCompletion:1.0.0",
          "data": {
            "ResourceTypeID": "srn:type:work-product-component/WellCompletion:",
            "WellboreId": "osdu:master-data--Wellbore:001:",
            "Name": "Well-001 Completion",
            "CompletionType": "Cased Hole",
            "CompletionTypeID": "osdu:reference-data--CompletionType:CasedHole:",
            "EffectiveDate": "2024-02-01",
            "CompletionIntervals": [
              {
                "topDepth": 0,
                "bottomDepth": 2000,
                "intervalType": "Production"
              }
            ],
            "EquipmentDetails": {
              "casing": {
                "strings": [...]
              },
              "tubing": {
                "strings": [...]
              }
            }
          }
        }
      ]
    }
  }
}

┌─────────────────────────────────────────────────────────────┐
│ Phase 2: Submit Manifest to Workflow Service                │
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

Step 3.3: Validate Completion Components
Manifest Parser:
  - For each completion component:
    * Validate WellboreId exists
    * Validate completion intervals
    * Validate equipment details
    * Validate effective date

Step 3.4: Resolve Reference Data
Manifest Parser → Reference Data Service:
  - Resolve CompletionTypeID
  - Resolve ActivityTypeID (if applicable)
  - Validate all references exist

┌─────────────────────────────────────────────────────────────┐
│ Phase 4: Well Delivery DDMS Storage (if bulk data provided) │
└─────────────────────────────────────────────────────────────┘

Step 4.1: Store Completion Data in Well Delivery DDMS
Manifest Parser → Well Delivery DDMS API
POST /api/os-well-delivery-ddms/wellbores/{wellboreId}/completions
Content-Type: application/json
Authorization: Bearer <JWT>

Request Body:
{
  "wellboreId": "osdu:master-data--Wellbore:001:",
  "completionData": {
    "completionType": "Cased Hole",
    "effectiveDate": "2024-02-01",
    "completionIntervals": [...],
    "equipmentDetails": {...}
  }
}

Response (200 OK):
{
  "completionId": "completion-001",
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
      "Name": "Well Delivery Work Product",
      "WorkProductComponents": [
        "osdu:doc:0ll23ik92gk8018h4kkif9059l7li65i"
      ]
    }
  },
  {
    "kind": "osdu:wks:work-product-component--WellCompletion:1.0.0",
    "data": {
      "ResourceTypeID": "srn:type:work-product-component/WellCompletion:",
      "WellboreId": "osdu:master-data--Wellbore:001:",
      "Name": "Well-001 Completion",
      "CompletionType": "Cased Hole",
      "CompletionTypeID": "osdu:reference-data--CompletionType:CasedHole:",
      "EffectiveDate": "2024-02-01",
      "CompletionIntervals": [...],
      "EquipmentDetails": {...}
    }
  }
]

Response (200 OK):
[
  {
    "id": "osdu:doc:1mm34jl03hl9129i5lljg0160m8mj76j",
    "version": 1582725123640845
  },
  {
    "id": "osdu:doc:0ll23ik92gk8018h4kkif9059l7li65i",
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

### 6.5 Pathway 5: Direct Storage API Ingestion

**Architecture**:
```
Application Code
    ↓
Well Delivery DDMS (bulk data, if needed)
    ↓
Storage Service API (metadata)
    ↓
Indexer Service
```

**Complete Call Stack**:

```
┌─────────────────────────────────────────────────────────────┐
│ Phase 1: Prepare Well Delivery Data                        │
└─────────────────────────────────────────────────────────────┘

Step 1.1: Prepare Well Planning Record
Application Code:
  const wellPlanningRecord = {
    "kind": "osdu:wks:work-product-component--WellPlanning:1.0.0",
    "acl": {
      "viewers": ["data.default.viewers@osdu"],
      "owners": ["data.default.owners@osdu"]
    },
    "legal": {
      "legaltags": ["osdu-legaltag-001"],
      "otherRelevantDataCountries": ["US"]
    },
    "data": {
      "ResourceTypeID": "srn:type:work-product-component/WellPlanning:",
      "WellboreId": "osdu:master-data--Wellbore:001:",
      "Name": "Well-001 Detailed Planning",
      "PlanningPhase": "Detailed",
      "PlanningPhaseID": "osdu:reference-data--PlanningPhase:Detailed:",
      "PlanningDate": "2024-01-15",
      "WellDesign": {...},
      "DrillingPlan": {...}
    }
  };

┌─────────────────────────────────────────────────────────────┐
│ Phase 2: Ingest via Storage Service API                    │
└─────────────────────────────────────────────────────────────┘

Step 2.1: Create Well Planning Record
Application → Storage Service API
PUT /api/storage/v2/records
Content-Type: application/json
Authorization: Bearer <JWT>
Slb-Data-Partition-Id: osdu

Request Body:
[
  {
    "kind": "osdu:wks:work-product-component--WellPlanning:1.0.0",
    "acl": {
      "viewers": ["data.default.viewers@osdu"],
      "owners": ["data.default.owners@osdu"]
    },
    "legal": {
      "legaltags": ["osdu-legaltag-001"],
      "otherRelevantDataCountries": ["US"]
    },
    "data": {
      "ResourceTypeID": "srn:type:work-product-component/WellPlanning:",
      "WellboreId": "osdu:master-data--Wellbore:001:",
      "Name": "Well-001 Detailed Planning",
      "PlanningPhase": "Detailed",
      "PlanningPhaseID": "osdu:reference-data--PlanningPhase:Detailed:",
      "PlanningDate": "2024-01-15",
      "WellDesign": {...},
      "DrillingPlan": {...}
    }
  }
]

Response (200 OK):
[
  {
    "id": "osdu:doc:2nn45km14im0230j6mmkh1271n9nk87k",
    "version": 1582725123640845
  }
]

┌─────────────────────────────────────────────────────────────┐
│ Phase 3: Indexing                                           │
└─────────────────────────────────────────────────────────────┘

Step 3.1: Indexer Service Processing
Indexer Service (Automatic):
  - Receives record creation event
  - Extracts searchable fields
  - Updates search index
```

---

## 7. Storage Architecture

### 7.1 Metadata vs. Bulk Data Separation

**Metadata Storage (Storage Service)**:

```json
{
  "id": "osdu:doc:4ff67ce36ae2452b8ddad3391f1fc08a",
  "version": 1582725123640845,
  "kind": "osdu:wks:work-product-component--WellPlanning:1.0.0",
  "data": {
    "ResourceTypeID": "srn:type:work-product-component/WellPlanning:",
    "WellboreId": "osdu:master-data--Wellbore:001:",
    "Name": "Well-001 Detailed Planning",
    "Description": "Detailed well planning for Well-001",
    "PlanningPhase": "Detailed",
    "PlanningPhaseID": "osdu:reference-data--PlanningPhase:Detailed:",
    "PlanningDate": "2024-01-15",
    "WellDesign": {
      "trajectory": "...",
      "casing": "..."
    },
    "DrillingPlan": {
      "sections": [...],
      "procedures": [...]
    },
    "DataGroupTypeProperties": {
      "Files": [
        "srn:type:file/csv:planning-001:"
      ]
    },
    "Datasets": [
      "osdu:dataset--File.Generic:planning-001:"
    ]
  }
}
```

**Bulk Data Storage (Well Delivery DDMS)**:

```
Well Delivery DDMS Backend
    ↓
Wellbore: wellbore-001
    ↓
Planning:
    - planning-001.json (planning data)
    {
      "planningId": "planning-001",
      "wellboreId": "osdu:master-data--Wellbore:001:",
      "planningData": {
        "planningPhase": "Detailed",
        "planningDate": "2024-01-15",
        "wellDesign": {...},
        "drillingPlan": {...}
      }
    }
    ↓
Execution:
    - execution-001.json (execution data)
    {
      "executionId": "execution-001",
      "wellboreId": "osdu:master-data--Wellbore:001:",
      "executionData": {
        "activityType": "Drilling",
        "startDate": "2024-01-20T00:00:00Z",
        "endDate": "2024-01-25T00:00:00Z",
        "operationalEvents": [...],
        "dailyReports": [...]
      }
    }
    ↓
Completions:
    - completion-001.json (completion data)
    {
      "completionId": "completion-001",
      "wellboreId": "osdu:master-data--Wellbore:001:",
      "completionData": {
        "completionType": "Cased Hole",
        "effectiveDate": "2024-02-01",
        "completionIntervals": [...],
        "equipmentDetails": {...}
      }
    }
    ↓
Indexed for Queries:
    - Wellbore index for fast lookups
    - Date range index (StartDate, EndDate, EffectiveDate)
    - Activity type index
    - Completion type index
```

### 7.2 File Storage

**Source File Preservation**:

```
File Service (Object Store)
    ↓
Source File Storage:
    gs://osdu-storage/osdu/files/{file-id}/well-001-planning.csv
    gs://osdu-storage/osdu/files/{file-id}/well-001-execution.xml
    gs://osdu-storage/osdu/files/{file-id}/well-001-planning.pdf
    gs://osdu-storage/osdu/files/{file-id}/well-001-completion.json
    ↓
Dataset Record Reference:
    {
      "kind": "osdu:wks:dataset--File.Generic:1.0.0",
      "data": {
        "DatasetProperties": {
          "FileSourceInfo": {
            "FileSource": "gs://osdu-storage/osdu/files/{file-id}/well-001-planning.csv"
          }
        }
      }
    }
    ↓
Linked to Well Delivery WPC:
    WellPlanning.data.DataGroupTypeProperties.Files = [
      "srn:type:file/csv:planning-001:"
    ]
```

### 7.3 Well Delivery DDMS Architecture

**Bulk Data Storage**:

Well Delivery DDMS provides optimized storage and access for well delivery bulk data:

**API Endpoints**:

```http
# Store well planning
POST /api/os-well-delivery-ddms/wellbores/{wellboreId}/planning
Content-Type: application/json

{
  "wellboreId": "osdu:master-data--Wellbore:001:",
  "planningData": {
    "planningPhase": "Detailed",
    "planningDate": "2024-01-15",
    "wellDesign": {...},
    "drillingPlan": {...}
  }
}

# Store well execution
POST /api/os-well-delivery-ddms/wellbores/{wellboreId}/execution
Content-Type: application/json

{
  "wellboreId": "osdu:master-data--Wellbore:001:",
  "executionData": {
    "activityType": "Drilling",
    "startDate": "2024-01-20T00:00:00Z",
    "endDate": "2024-01-25T00:00:00Z",
    "operationalEvents": [...],
    "dailyReports": [...]
  }
}

# Store well completion
POST /api/os-well-delivery-ddms/wellbores/{wellboreId}/completions
Content-Type: application/json

{
  "wellboreId": "osdu:master-data--Wellbore:001:",
  "completionData": {
    "completionType": "Cased Hole",
    "effectiveDate": "2024-02-01",
    "completionIntervals": [...],
    "equipmentDetails": {...}
  }
}

# Retrieve well planning
GET /api/os-well-delivery-ddms/wellbores/{wellboreId}/planning/{planningId}

# Retrieve well execution
GET /api/os-well-delivery-ddms/wellbores/{wellboreId}/execution/{executionId}

# Retrieve well completion
GET /api/os-well-delivery-ddms/wellbores/{wellboreId}/completions/{completionId}

# Query by date range
GET /api/os-well-delivery-ddms/wellbores/{wellboreId}/execution?startDate=2024-01-01&endDate=2024-01-31

# Query by activity type
GET /api/os-well-delivery-ddms/wellbores/{wellboreId}/execution?activityType=Drilling
```

**Storage Benefits**:

- **Optimized Access**: Efficient retrieval of large operational datasets
- **Time Indexing**: Fast queries by date ranges (StartDate, EndDate, EffectiveDate)
- **Activity Indexing**: Fast queries by activity type
- **Completion Indexing**: Fast queries by completion type
- **Compression**: Efficient storage of repetitive operational data
- **Caching**: In-memory caching for frequently accessed data
- **Lifecycle Management**: Supports well lifecycle from planning through execution to completion

---

## 8. Data Transformation & Validation

### 8.1 Reference Data Resolution

**Planning Phase Resolution**:

```
Input: "Detailed" or "Conceptual" or "Preliminary"
    ↓
Reference Data Service Lookup
    GET /api/reference-data/v2/reference-data/osdu:reference-data--PlanningPhase:Detailed:
    ↓
Response:
    {
      "code": "Detailed",
      "name": "Detailed Planning",
      "description": "Detailed well planning phase"
    }
    ↓
Resolved Reference:
    "osdu:reference-data--PlanningPhase:Detailed:"
```

**Activity Type Resolution**:

```
Input: "Drilling" or "Completion" or "Workover"
    ↓
Reference Data Service Lookup
    GET /api/reference-data/v2/reference-data/osdu:reference-data--ActivityType:Drilling:
    ↓
Response:
    {
      "code": "Drilling",
      "name": "Drilling Activity",
      "description": "Drilling operations"
    }
    ↓
Resolved Reference:
    "osdu:reference-data--ActivityType:Drilling:"
```

**Completion Type Resolution**:

```
Input: "Cased Hole" or "Open Hole" or "Multilateral"
    ↓
Reference Data Service Lookup
    GET /api/reference-data/v2/reference-data/osdu:reference-data--CompletionType:CasedHole:
    ↓
Response:
    {
      "code": "CasedHole",
      "name": "Cased Hole Completion",
      "description": "Cased hole completion type"
    }
    ↓
Resolved Reference:
    "osdu:reference-data--CompletionType:CasedHole:"
```

---

## 9. Search & Discovery

### 9.1 Indexing Process

```
Record Creation Event
    ↓
Indexer Service
    ↓
Extract Searchable Fields:
    - WellboreId → Filter by wellbore
    - Name → Full-text search
    - PlanningPhase → Filter by phase
    - ActivityType → Filter by activity
    - CompletionType → Filter by completion type
    - PlanningDate → Date range queries
    - StartDate, EndDate → Date range queries
    - EffectiveDate → Date range queries
    ↓
Update Search Index
    ↓
Search Service Available
```

### 9.2 Query Examples

**Search by Wellbore**:

```http
POST /api/search/v2/query
Content-Type: application/json
Authorization: Bearer <JWT>
Slb-Data-Partition-Id: osdu

{
  "kind": "osdu:wks:work-product-component--WellPlanning:1.0.0",
  "query": "data.WellboreId:\"osdu:master-data--Wellbore:001:\""
}
```

**Search by Planning Phase**:

```http
POST /api/search/v2/query
Content-Type: application/json
Authorization: Bearer <JWT>
Slb-Data-Partition-Id: osdu

{
  "kind": "osdu:wks:work-product-component--WellPlanning:1.0.0",
  "query": "data.PlanningPhase:\"Detailed\""
}
```

**Search by Date Range**:

```http
POST /api/search/v2/query
Content-Type: application/json
Authorization: Bearer <JWT>
Slb-Data-Partition-Id: osdu

{
  "kind": "osdu:wks:work-product-component--WellExecution:1.0.0",
  "query": "data.StartDate:[2024-01-01 TO 2024-01-31]"
}
```

**Search by Activity Type**:

```http
POST /api/search/v2/query
Content-Type: application/json
Authorization: Bearer <JWT>
Slb-Data-Partition-Id: osdu

{
  "kind": "osdu:wks:work-product-component--WellExecution:1.0.0",
  "query": "data.ActivityType:\"Drilling\""
}
```

---

## 10. Error Handling & Validation

### 10.1 Validation Errors

**Common Validation Failures**:

1. **Missing Wellbore Reference**:
   ```
   Error: WellboreId not specified or invalid
   Resolution: Provide valid WellboreId reference
   ```

2. **Invalid Date Format**:
   ```
   Error: PlanningDate format invalid
   Resolution: Use ISO 8601 date format (YYYY-MM-DD)
   ```

3. **Invalid Reference Data**:
   ```
   Error: PlanningPhaseID reference not found
   Resolution: Ensure reference exists in Reference Data Service
   ```

4. **Missing Required Fields**:
   ```
   Error: Required field Name missing
   Resolution: Provide all required fields
   ```

5. **Invalid Completion Intervals**:
   ```
   Error: Completion interval topDepth (2000) greater than bottomDepth (1000)
   Resolution: Validate depth intervals before ingestion
   ```

### 10.2 Schema Validation

**Required Field Validation**:
- Fields marked as required in schema must be present
- Missing required fields cause ingestion failure

**Data Type Validation**:
- String fields must be strings
- Date fields must be valid dates
- Array fields must be arrays
- Object fields must be objects

**Reference Validation**:
- Master data references (WellboreId) must exist
- Reference data values (PlanningPhaseID, ActivityTypeID, CompletionTypeID) must be valid
- Dataset references must exist

**Example Validation Error**:

```json
{
  "error": "Schema validation failed",
  "kind": "osdu:wks:work-product-component--WellPlanning:1.0.0",
  "errors": [
    {
      "path": "data.WellboreId",
      "message": "Required field missing"
    },
    {
      "path": "data.PlanningDate",
      "message": "Expected Date, got String"
    },
    {
      "path": "data.PlanningPhaseID",
      "message": "Reference not found"
    }
  ]
}
```

---

## 11. Performance Considerations

### 11.1 Batch Ingestion

**Optimal Batch Size**:

- **Small datasets (< 100 records)**: Process individually
- **Medium datasets (100-1000 records)**: Batch by wellbore
- **Large datasets (> 1000 records)**: Chunk processing recommended

**Recommended**: Process well delivery records individually, batch metadata records (100-200 per batch)

### 11.2 Document Processing

**Efficient Document Processing**:

```python
def process_documents(documents, chunk_size=10):
    """
    Process documents in chunks
    """
    # 1. Extract metadata from all documents first
    metadata_list = []
    for doc in documents:
        metadata = extract_document_metadata(doc)
        metadata_list.append(metadata)
    
    # 2. Process in chunks if large
    if len(documents) > chunk_size:
        chunks = [documents[i:i+chunk_size] for i in range(0, len(documents), chunk_size)]
        for chunk in chunks:
            process_document_chunk(chunk)
    else:
        process_documents_batch(documents)
    
    return metadata_list
```

---

## 12. Design Patterns and Best Practices

### 12.1 Schema First Approach

1. **Define schemas before ingestion**: Ensures type safety and proper indexing
2. **Version schemas carefully**: Schema changes require data migration
3. **Index only necessary fields**: Reduces index size and improves performance
4. **Validate at ingestion**: Catch errors early in the pipeline

### 12.2 Work Product Pattern

1. **Group related WPCs**: Use Work Products to organize related well delivery data
2. **Maintain relationships**: Link planning, execution, and completion records
3. **Preserve lineage**: Track source files and processing history

### 12.3 Well Delivery DDMS Usage

1. **Use Well Delivery DDMS for bulk data**: Optimized storage for operational data
2. **Store metadata separately**: Keep metadata in Storage Service for searchability
3. **Leverage indexing**: Use date and activity type indexes for efficient queries

### 12.4 Document Management

1. **Preserve original documents**: Keep documents in File Service for traceability
2. **Extract metadata**: Extract structured metadata for searchability
3. **Enable full-text search**: Index document content for full-text search

---

## 13. Important Considerations

### 13.1 Data Partitioning

- All well delivery operations require `data-partition-id` header
- Data is scoped to partitions for multi-tenancy
- Cross-partition queries are not supported

### 13.2 Access Control

- ACLs (Access Control Lists) must be specified at ingestion
- Legal tags are required for compliance
- Viewers and owners must be specified

### 13.3 Versioning

- Records are versioned automatically
- Each update creates a new version
- Version history is maintained

### 13.4 File Source References

- File sources use SRN (Storage Resource Name) format
- Files must be uploaded to File Service before referencing
- File sources are validated during ingestion

### 13.5 Wellbore Association

- Each well delivery record must be associated with a Wellbore
- WellboreId must exist in Master Data Service
- Wellbore reference is validated during ingestion

### 13.6 Lifecycle Management

- Well delivery data supports well lifecycle from planning through execution to completion
- Records can be linked to track well lifecycle progression
- Time-based queries enable tracking of well lifecycle over time

---

## 14. Summary

OSDU provides **comprehensive, multi-format ingestion** for well delivery and planning data through specialized services:

1. **Well Delivery DDMS**: Optimized bulk storage for well planning, execution, and completion data
2. **CSV Parser**: Automated ingestion from CSV files with structured data
3. **WITSML Parser**: Industry-standard WITSML format support
4. **Document Ingestion**: Support for PDF, Word, and other document formats
5. **Manifest-based**: Structured JSON manifests for complex workflows
6. **Direct API**: Programmatic record creation for custom applications

**Key Design Decisions**:

- **Metadata/Bulk Separation**: Metadata in Storage Service, bulk data in Well Delivery DDMS
- **Lifecycle Management**: Supports well lifecycle from planning through execution to completion
- **Multi-Format Support**: WITSML, CSV, JSON, and document formats
- **Document Management**: Handles both structured data and unstructured documents
- **Time-Based Data**: Supports time-series operational data and event-based records
- **Source File Preservation**: Original files stored alongside processed data
- **Search Integration**: Automatic indexing for discovery and full-text search
- **Access Control**: ACLs and legal tags enforced at ingestion time

**Call Stack Highlights**:

- **CSV Parser**: File upload → DAG trigger → CSV parsing → Data extraction → Well Delivery DDMS storage → Metadata ingest → Index
- **WITSML Parser**: File upload → DAG trigger → WITSML parsing → Data extraction → Well Delivery DDMS storage → Metadata ingest → Index
- **Document Ingestion**: File upload → Document parsing → Metadata extraction → Storage API → Index (with full-text search)
- **Manifest**: JSON creation → Workflow API → Parse & validate → Well Delivery DDMS/Storage → Batch ingest → Index
- **Direct API**: Application code → Well Delivery DDMS (if needed) → Storage API → Index

This architecture ensures **data consistency**, **lifecycle traceability**, **document preservation**, **traceability**, and **governance** while supporting multiple ingestion formats and well delivery data types.

---

## 15. References

1. OSDU Data Formats, Schemas & Storage Architecture Documentation
2. OSDU Major Data Types Overview Documentation
3. OSDU Storage Service API Documentation
4. OSDU Well Delivery DDMS API Documentation
5. OSDU Workflow Service API Documentation
6. OSDU File Service API Documentation
7. OSDU Search Service API Documentation
8. OSDU Reference Data Service API Documentation
9. WITSML 2.0 Specification
10. Well Lifecycle Integration Platform (WLIP) Documentation

