# OSDU Well Execution Ingestion Analysis: Architecture, Schema, and Ingestion Flow

## Executive Summary

This document provides a comprehensive analysis of how well execution data (drilling operations, daily drilling reports, operational events) is designed, stored, processed, and ingested in the OSDU (Open Subsurface Data Universe) platform. It covers the complete architecture, schema definitions, and detailed ingestion workflows with call stacks for all supported formats (WITSML, CSV, and Documents).

---

## 1. Well Execution Architecture Overview

### 1.1 Conceptual Overview

**Well Execution** data in OSDU represents operational data from well drilling, completion, and workover activities, including:

- **Drilling Operations**: Drilling activities, operations, and procedures
- **Daily Drilling Reports (DDR)**: Daily operational reports with drilling metrics
- **Operational Events**: Time-stamped events during well operations
- **Non-Productive Time (NPT)**: Tracking of non-productive time events
- **Operational Metrics**: Performance metrics, drilling parameters, and measurements

In OSDU, well execution data is represented as **Work Product Components (WPC)** in the OSDU data model, following the Work Product pattern, and is managed by the **Well Delivery DDMS** (Domain Data Management Service).

### 1.2 OSDU Data Model Hierarchy

Well execution data fits into the OSDU data model hierarchy as follows:

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
│ - WellExecution                         │
│   • ActivityType                        │
│   • StartDate, EndDate                  │
│   • OperationalEvents                   │
│   • DailyReports                        │
│   • NonProductiveTime                   │
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

1. **Work Product (WP)**: Container that groups one or more well execution WPCs
2. **Work Product Component (WPC)**: The actual WellExecution record containing operational data
3. **Dataset**: References to source files (WITSML, CSV, PDF, Word documents)
4. **Master Data References**: Links to Well, Wellbore, and WellboreActivity entities
5. **Well Delivery DDMS**: Specialized service for optimized bulk storage of well execution data
6. **Reference Data**: Activity types, event types, operational metrics

### 1.4 Design Principles

- **Schema First**: Well execution requires schema definition before ingestion
- **Work Product Pattern**: Follows OSDU Work Product/Work Product Component pattern
- **Metadata/Bulk Separation**: Metadata in Storage Service, bulk data in Well Delivery DDMS
- **Time-Based Data**: Supports time-series operational data and event-based records
- **Document Management**: Handles both structured data (CSV, WITSML) and unstructured documents (PDF, Word)
- **Multi-Format Support**: WITSML, CSV, JSON, and document formats
- **Event Tracking**: Supports detailed operational event tracking with timestamps

---

## 2. Well Execution Schema Definition

### 2.1 Schema Structure

WellExecution is defined as a Work Product Component with the following schema structure:

**Kind Identifier Pattern**:
```
{authority}:wks:work-product-component--WellExecution:{version}
```

**Example**: `osdu:wks:work-product-component--WellExecution:1.0.0`

### 2.2 Schema Fields

#### 2.2.1 Work Product Component Schema

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
      "path": "data.Duration",
      "kind": "core:Double:1.0.0"
    },
    {
      "path": "data.DurationUnit",
      "kind": "core:String:1.0.0"
    },
    {
      "path": "data.OperationalEvents",
      "kind": "core:Array:1.0.0"
    },
    {
      "path": "data.DailyReports",
      "kind": "core:Array:1.0.0"
    },
    {
      "path": "data.NonProductiveTime",
      "kind": "core:Object:1.0.0"
    },
    {
      "path": "data.OperationalMetrics",
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

### 2.3 Operational Event Data Structure

Operational events contain the following fields:

#### Operational Event:
- **eventType** (string): Type of event (Drilling, Tripping, Casing, etc.)
- **eventDescription** (string): Description of the event
- **timestamp** (datetime): Event timestamp
- **duration** (float, optional): Event duration in hours
- **depth** (float, optional): Depth at which event occurred
- **details** (object, optional): Additional event details

### 2.4 Daily Report Data Structure

Daily reports contain the following fields:

#### Daily Report:
- **reportDate** (date): Date of the report
- **content** (string): Report content/text
- **drillingMetrics** (object, optional): Drilling metrics (ROP, WOB, RPM, etc.)
- **depthProgress** (object, optional): Depth progress information
- **weatherConditions** (object, optional): Weather conditions
- **personnel** (array, optional): Personnel on site
- **equipment** (array, optional): Equipment used

### 2.5 Non-Productive Time (NPT) Structure

NPT data contains the following fields:

#### Non-Productive Time:
- **totalHours** (float): Total NPT hours
- **events** (array): Array of NPT events
  - **eventType** (string): Type of NPT event
  - **startTime** (datetime): NPT start time
  - **endTime** (datetime): NPT end time
  - **duration** (float): Duration in hours
  - **description** (string): Description of NPT event
  - **category** (string): NPT category (Equipment, Weather, Personnel, etc.)

### 2.6 Understanding "Schema First" Approach

OSDU requires **schema definition before ingestion** for well execution data. This ensures:

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

### 2.7 Schema API

**Create Schema**:
```http
POST /api/storage/v2/schemas
Content-Type: application/json
Authorization: Bearer <JWT>
Slb-Data-Partition-Id: osdu

{
  "kind": "osdu:wks:work-product-component--WellExecution:1.0.0",
  "schema": [
    {
      "path": "data.WellboreId",
      "kind": "core:String:1.0.0"
    },
    {
      "path": "data.ActivityType",
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
    }
  ]
}
```

**Get Schema**:
```http
GET /api/storage/v2/schemas/osdu:wks:work-product-component--WellExecution:1.0.0
Authorization: Bearer <JWT>
Slb-Data-Partition-Id: osdu
```

---

## 3. Supported Data Formats

### 3.1 WITSML Format

WITSML (Wellsite Information Transfer Standard Markup Language) for well execution data:

**Structure**:
- **XML-based**: Industry standard XML format
- **Drilling Operations**: WITSML 2.0 drilling operation objects
- **Daily Reports**: WITSML 2.0 daily report objects
- **Operational Events**: WITSML 2.0 operational event objects

**Key Elements**:
- `drillingOperation`: Root element for drilling operations
- `dailyReport`: Daily drilling report data
- `operationalEvent`: Operational event information
- `wellbore`: Reference to wellbore
- `activity`: Activity information
- `npt`: Non-productive time information
- `drillingParams`: Drilling parameters (ROP, WOB, RPM, etc.)

**Storage Strategy**: WITSML files are preserved in File Service, with structured data parsed and stored in Well Delivery DDMS.

### 3.2 CSV Format

CSV files for well execution data:

**Well Execution CSV Structure**:
```csv
WellboreId,ActivityType,StartDate,EndDate,EventType,EventDescription,Timestamp,Depth
wellbore-001,Drilling,2024-01-20T00:00:00Z,2024-01-25T00:00:00Z,Drilling,Drilling section 1,2024-01-20T08:00:00Z,1000
wellbore-001,Drilling,2024-01-20T00:00:00Z,2024-01-25T00:00:00Z,Tripping,Tripping out,2024-01-21T10:00:00Z,1500
```

**Daily Report CSV Structure**:
```csv
WellboreId,ReportDate,Content,ROP,WOB,RPM,Depth
wellbore-001,2024-01-20,Drilling section 1,25.5,50000,120,1000
wellbore-001,2024-01-21,Drilling section 1 continued,24.8,48000,120,1500
```

**NPT CSV Structure**:
```csv
WellboreId,EventType,StartTime,EndTime,Duration,Description,Category
wellbore-001,Equipment Failure,2024-01-22T14:00:00Z,2024-01-22T18:00:00Z,4.0,Pump failure,Equipment
```

**Storage Strategy**: CSV files are preserved in File Service, with structured data extracted and stored in Well Delivery DDMS.

### 3.3 Document Formats

Document files for well execution data:

**Supported Formats**:
- **PDF**: Daily drilling reports, operational reports
- **Word (.doc, .docx)**: Daily drilling reports, operational procedures
- **Excel (.xls, .xlsx)**: Daily drilling reports, operational data spreadsheets

**Daily Drilling Report (DDR) Structure**:
- Report header (well name, date, rig information)
- Daily summary (activities, progress, issues)
- Drilling parameters (ROP, WOB, RPM, torque, etc.)
- Depth progress (current depth, footage drilled)
- Weather conditions
- Personnel and equipment
- Non-productive time events

**Storage Strategy**: Documents are preserved in File Service, with metadata extracted and stored in Storage Service. Documents may be indexed for full-text search.

---

## 4. Ingestion Methods

OSDU supports multiple ingestion pathways for well execution data:

1. **WITSML Parser Ingestion** (Energistics integration)
2. **CSV Parser Ingestion** (Airflow DAG)
3. **Document Ingestion** (File Service + Metadata extraction)
4. **Manifest-based Ingestion** (Workflow Service)
5. **Direct Storage API Ingestion** (Programmatic)

---

## 5. Ingestion Flow: Complete Walkthrough

### 5.1 Pathway 1: WITSML Well Execution Ingestion

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
Operational Data Extraction
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
  "FileID": "4ff67ce36ae2452b8ddad3391f1fc08a",
  "FileSource": "srn:type:file/witsml:execution-001:",
  "Location": "gs://osdu-storage/osdu/files/4ff67ce36ae2452b8ddad3391f1fc08a/well-001-execution.xml"
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
  "workflowId": "workflow-001",
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
  - Extract operationalEvent elements
  - Extract npt elements
  - Extract wellbore reference
  - Extract activity metadata:
    * Activity type
    * Start date
    * End date
    * Duration

Step 3.3: Parse Operational Events
WITSML Parser (Energistics):
  - For each operationalEvent element:
    * Extract event type
    * Extract event description
    * Extract event timestamp
    * Extract event duration (if available)
    * Extract event depth (if available)
    * Extract event details
    * Store in events array

Step 3.4: Parse Daily Reports
WITSML Parser (Energistics):
  - For each dailyReport element:
    * Extract report date
    * Extract report content/text
    * Extract drilling parameters:
      - ROP (Rate of Penetration)
      - WOB (Weight on Bit)
      - RPM (Revolutions Per Minute)
      - Torque
      - Flow rate
    * Extract depth progress
    * Extract weather conditions (if available)
    * Extract personnel (if available)
    * Extract equipment (if available)
    * Store in reports array

Step 3.5: Parse Non-Productive Time
WITSML Parser (Energistics):
  - For each npt element:
    * Extract NPT event type
    * Extract start time
    * Extract end time
    * Calculate duration
    * Extract description
    * Extract category
    * Store in npt array
  - Calculate total NPT hours

Step 3.6: Parse Drilling Parameters
WITSML Parser (Energistics):
  - Extract drilling parameters from drillingOperation:
    * Average ROP
    * Average WOB
    * Average RPM
    * Maximum torque
    * Flow rates
  - Store in operational metrics

Step 3.7: Generate Execution Metadata
WITSML Parser:
  - Calculate statistics:
    * Total duration (EndDate - StartDate)
    * Number of events
    * Number of daily reports
    * Total NPT hours
    * Average drilling parameters
  - Extract activity type from WITSML
  - Resolve ActivityTypeID from reference data

Step 3.8: Resolve Wellbore Reference
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
    "duration": 120.0,
    "durationUnit": "Hour",
    "operationalEvents": [
      {
        "eventType": "Drilling",
        "eventDescription": "Drilling section 1",
        "timestamp": "2024-01-20T08:00:00Z",
        "depth": 1000.0
      },
      {
        "eventType": "Tripping",
        "eventDescription": "Tripping out",
        "timestamp": "2024-01-21T10:00:00Z",
        "depth": 1500.0
      }
    ],
    "dailyReports": [
      {
        "reportDate": "2024-01-20",
        "content": "Drilling section 1. Progress: 1000 ft. ROP: 25.5 ft/hr.",
        "drillingMetrics": {
          "rop": 25.5,
          "wob": 50000,
          "rpm": 120,
          "torque": 4500,
          "flowRate": 800
        },
        "depthProgress": {
          "currentDepth": 1000,
          "footageDrilled": 1000
        }
      }
    ],
    "nonProductiveTime": {
      "totalHours": 12.0,
      "events": [
        {
          "eventType": "Equipment Failure",
          "startTime": "2024-01-22T14:00:00Z",
          "endTime": "2024-01-22T18:00:00Z",
          "duration": 4.0,
          "description": "Pump failure",
          "category": "Equipment"
        }
      ]
    },
    "operationalMetrics": {
      "averageROP": 25.2,
      "averageWOB": 49000,
      "averageRPM": 120,
      "maxTorque": 5000
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
      "Duration": 120.0,
      "DurationUnit": "Hour",
      "OperationalEvents": [
        {
          "eventType": "Drilling",
          "eventDescription": "Drilling section 1",
          "timestamp": "2024-01-20T08:00:00Z",
          "depth": 1000.0
        }
      ],
      "DailyReports": [
        {
          "reportDate": "2024-01-20",
          "content": "Drilling section 1. Progress: 1000 ft."
        }
      ],
      "NonProductiveTime": {
        "totalHours": 12.0,
        "events": [
          {
            "eventType": "Equipment Failure",
            "startTime": "2024-01-22T14:00:00Z",
            "endTime": "2024-01-22T18:00:00Z",
            "duration": 4.0,
            "description": "Pump failure",
            "category": "Equipment"
          }
        ]
      },
      "OperationalMetrics": {
        "averageROP": 25.2,
        "averageWOB": 49000,
        "averageRPM": 120
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
    * ActivityType
    * StartDate, EndDate
    * Duration
    * OperationalEvents (event types, timestamps)
    * NonProductiveTime (total hours, categories)
  - Updates search index
  - Enables date range and activity type queries
```

**WITSML Parser Implementation**:

```python
# witsml_well_execution_parser.py
from energistics.witsml import WitsmlParser
from datetime import datetime

def parse_witsml_execution(witsml_file, wellbore_id, data_partition_id):
    """
    Parse WITSML file and create well execution records
    """
    # 1. Read WITSML file
    witsml_content = file_service.read(witsml_file)
    
    # 2. Parse WITSML
    parser = WitsmlParser()
    witsml_data = parser.parse(witsml_content)
    
    # 3. Extract drilling operations
    drilling_operations = witsml_data.get('drillingOperations', [])
    
    # 4. Extract operational events
    operational_events = []
    for operation in drilling_operations:
        events = operation.get('operationalEvents', [])
        for event in events:
            operational_events.append({
                "eventType": event.get('eventType'),
                "eventDescription": event.get('description'),
                "timestamp": event.get('timestamp'),
                "depth": event.get('depth'),
                "details": event.get('details')
            })
    
    # 5. Extract daily reports
    daily_reports = []
    for report in witsml_data.get('dailyReports', []):
        daily_reports.append({
            "reportDate": report.get('date'),
            "content": report.get('content'),
            "drillingMetrics": {
                "rop": report.get('rop'),
                "wob": report.get('wob'),
                "rpm": report.get('rpm'),
                "torque": report.get('torque'),
                "flowRate": report.get('flowRate')
            },
            "depthProgress": {
                "currentDepth": report.get('currentDepth'),
                "footageDrilled": report.get('footageDrilled')
            }
        })
    
    # 6. Extract NPT
    npt_events = []
    total_npt_hours = 0.0
    for npt in witsml_data.get('npt', []):
        start_time = datetime.fromisoformat(npt.get('startTime'))
        end_time = datetime.fromisoformat(npt.get('endTime'))
        duration = (end_time - start_time).total_seconds() / 3600.0
        total_npt_hours += duration
        
        npt_events.append({
            "eventType": npt.get('eventType'),
            "startTime": npt.get('startTime'),
            "endTime": npt.get('endTime'),
            "duration": duration,
            "description": npt.get('description'),
            "category": npt.get('category')
        })
    
    # 7. Calculate duration
    start_date = datetime.fromisoformat(witsml_data.get('startDate'))
    end_date = datetime.fromisoformat(witsml_data.get('endDate'))
    duration = (end_date - start_date).total_seconds() / 3600.0
    
    # 8. Store in Well Delivery DDMS
    execution_id = well_delivery_ddms.store_execution(
        wellbore_id=wellbore_id,
        execution_data={
            "activityType": witsml_data.get('activityType'),
            "startDate": witsml_data.get('startDate'),
            "endDate": witsml_data.get('endDate'),
            "duration": duration,
            "operationalEvents": operational_events,
            "dailyReports": daily_reports,
            "nonProductiveTime": {
                "totalHours": total_npt_hours,
                "events": npt_events
            }
        }
    )
    
    # 9. Create execution record
    storage_service.ingest_records([{
        "kind": f"{data_partition_id}:wks:work-product-component--WellExecution:1.0.0",
        "data": {
            "ResourceTypeID": "srn:type:work-product-component/WellExecution:",
            "WellboreId": wellbore_id,
            "Name": f"Well-001 {witsml_data.get('activityType')} Execution",
            "ActivityType": witsml_data.get('activityType'),
            "ActivityTypeID": f"{data_partition_id}:reference-data--ActivityType:{witsml_data.get('activityType')}:",
            "StartDate": witsml_data.get('startDate'),
            "EndDate": witsml_data.get('endDate'),
            "Duration": duration,
            "OperationalEvents": operational_events,
            "DailyReports": daily_reports,
            "NonProductiveTime": {
                "totalHours": total_npt_hours,
                "events": npt_events
            },
            "DataGroupTypeProperties": {
                "Files": [witsml_file]
            }
        }
    }])
    
    return {"status": "success", "execution_id": execution_id}
```

### 5.2 Pathway 2: CSV Well Execution Ingestion

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
Operational Data Extraction
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
  - file: well-001-execution.csv (binary)

Response (200 OK):
{
  "FileID": "6hh89eg58cg4674d0ggbe5615h3he21e",
  "FileSource": "srn:type:file/csv:execution-002:",
  "Location": "gs://osdu-storage/osdu/files/6hh89eg58cg4674d0ggbe5615h3he21e/well-001-execution.csv"
}

┌─────────────────────────────────────────────────────────────┐
│ Phase 2: Trigger CSV Parser Workflow                       │
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
      "FileSource": "srn:type:file/csv:execution-002:",
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
│ Phase 3: CSV Parser Processing (Airflow DAG)              │
└─────────────────────────────────────────────────────────────┘

Step 3.1: Read CSV File from Staging
Airflow DAG → File Service
GET /v2/files/{fileId}/download
Authorization: Bearer <JWT>

Response: CSV file content

Step 3.2: Parse CSV Header Row
Airflow DAG (Python):
  - Read first row
  - Detect CSV type (execution events, daily reports, or NPT)
  - Validate required columns based on type

Step 3.3: Parse CSV Data Rows
Airflow DAG (Python):
  - For execution events CSV:
    * Extract WellboreId, ActivityType, StartDate, EndDate
    * Extract EventType, EventDescription, Timestamp, Depth
    * Validate data types and formats
    * Group events by execution period
  - For daily reports CSV:
    * Extract WellboreId, ReportDate, Content
    * Extract drilling metrics (ROP, WOB, RPM, etc.)
    * Extract depth progress
    * Store in reports array
  - For NPT CSV:
    * Extract NPT event details
    * Calculate durations
    * Store in NPT array

Step 3.4: Generate Execution Metadata
Airflow DAG (Python):
  - Calculate statistics:
    * Total duration
    * Number of events
    * Number of daily reports
    * Total NPT hours
  - Extract activity type from CSV or metadata
  - Resolve ActivityTypeID from reference data

┌─────────────────────────────────────────────────────────────┐
│ Phase 4: Well Delivery DDMS Storage (Bulk Data)            │
└─────────────────────────────────────────────────────────────┘

Step 4.1: Store Execution Data in Well Delivery DDMS
Airflow DAG → Well Delivery DDMS API
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
    "operationalEvents": [...],
    "dailyReports": [...],
    "nonProductiveTime": {...}
  }
}

Response (200 OK):
{
  "executionId": "execution-002",
  "status": "STORED"
}

┌─────────────────────────────────────────────────────────────┐
│ Phase 5: Storage Service Metadata Ingestion                │
└─────────────────────────────────────────────────────────────┘

Step 5.1: Create WellExecution WPC Record
Airflow DAG → Storage Service API
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
      "Name": "Well-001 Drilling Execution (CSV)",
      "ActivityType": "Drilling",
      "ActivityTypeID": "osdu:reference-data--ActivityType:Drilling:",
      "StartDate": "2024-01-20T00:00:00Z",
      "EndDate": "2024-01-25T00:00:00Z",
      "OperationalEvents": [...],
      "DailyReports": [...],
      "NonProductiveTime": {...},
      "DataGroupTypeProperties": {
        "Files": [
          "srn:type:file/csv:execution-002:"
        ]
      }
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

**CSV Parser DAG Implementation**:

```python
# csv_well_execution_ingestion_dag.py (Airflow DAG)
from airflow import DAG
from airflow.operators.python import PythonOperator
from datetime import datetime

def parse_csv_well_execution(file_source, wellbore_id, data_partition_id):
    """
    Parse CSV file and create well execution records
    """
    # 1. Read CSV from staging
    csv_data = file_service.read(file_source)
    
    # 2. Parse CSV
    import csv
    reader = csv.DictReader(csv_data)
    
    # 3. Detect CSV type
    csv_type = detect_csv_type(reader.fieldnames)
    
    # 4. Extract data based on type
    if csv_type == "execution_events":
        operational_events = []
        for row in reader:
            operational_events.append({
                "eventType": row['EventType'],
                "eventDescription": row['EventDescription'],
                "timestamp": row['Timestamp'],
                "depth": float(row['Depth']) if row.get('Depth') else None
            })
        
        # Group by execution period
        start_date = min(e['timestamp'] for e in operational_events)
        end_date = max(e['timestamp'] for e in operational_events)
        activity_type = determine_activity_type(operational_events)
        
    elif csv_type == "daily_reports":
        daily_reports = []
        for row in reader:
            daily_reports.append({
                "reportDate": row['ReportDate'],
                "content": row['Content'],
                "drillingMetrics": {
                    "rop": float(row['ROP']) if row.get('ROP') else None,
                    "wob": float(row['WOB']) if row.get('WOB') else None,
                    "rpm": float(row['RPM']) if row.get('RPM') else None
                },
                "depthProgress": {
                    "currentDepth": float(row['Depth']) if row.get('Depth') else None
                }
            })
        
        start_date = min(r['reportDate'] for r in daily_reports)
        end_date = max(r['reportDate'] for r in daily_reports)
        activity_type = "Drilling"  # Default or from metadata
    
    elif csv_type == "npt":
        npt_events = []
        total_npt_hours = 0.0
        for row in reader:
            start_time = datetime.fromisoformat(row['StartTime'])
            end_time = datetime.fromisoformat(row['EndTime'])
            duration = (end_time - start_time).total_seconds() / 3600.0
            total_npt_hours += duration
            
            npt_events.append({
                "eventType": row['EventType'],
                "startTime": row['StartTime'],
                "endTime": row['EndTime'],
                "duration": duration,
                "description": row['Description'],
                "category": row['Category']
            })
    
    # 5. Resolve reference data
    activity_type_id = reference_data_service.resolve(
        f"{data_partition_id}:reference-data--ActivityType:{activity_type}:"
    )
    
    # 6. Store in Well Delivery DDMS
    execution_id = well_delivery_ddms.store_execution(
        wellbore_id=wellbore_id,
        execution_data={
            "activityType": activity_type,
            "startDate": start_date,
            "endDate": end_date,
            "operationalEvents": operational_events if csv_type == "execution_events" else [],
            "dailyReports": daily_reports if csv_type == "daily_reports" else [],
            "nonProductiveTime": {
                "totalHours": total_npt_hours if csv_type == "npt" else 0.0,
                "events": npt_events if csv_type == "npt" else []
            }
        }
    )
    
    # 7. Create execution record
    storage_service.ingest_records([{
        "kind": f"{data_partition_id}:wks:work-product-component--WellExecution:1.0.0",
        "data": {
            "ResourceTypeID": "srn:type:work-product-component/WellExecution:",
            "WellboreId": wellbore_id,
            "Name": f"Well-001 {activity_type} Execution (CSV)",
            "ActivityType": activity_type,
            "ActivityTypeID": activity_type_id,
            "StartDate": start_date,
            "EndDate": end_date,
            "OperationalEvents": operational_events if csv_type == "execution_events" else [],
            "DailyReports": daily_reports if csv_type == "daily_reports" else [],
            "NonProductiveTime": {
                "totalHours": total_npt_hours if csv_type == "npt" else 0.0,
                "events": npt_events if csv_type == "npt" else []
            },
            "DataGroupTypeProperties": {
                "Files": [file_source]
            }
        }
    }])
    
    return {"status": "success", "execution_id": execution_id}

def detect_csv_type(fieldnames):
    """
    Detect CSV type based on column names
    """
    if 'EventType' in fieldnames and 'Timestamp' in fieldnames:
        return "execution_events"
    elif 'ReportDate' in fieldnames and 'Content' in fieldnames:
        return "daily_reports"
    elif 'StartTime' in fieldnames and 'EndTime' in fieldnames and 'Category' in fieldnames:
        return "npt"
    else:
        return "execution_events"  # Default

with DAG(
    dag_id='csv_well_execution_ingestion',
    start_date=datetime(2023, 1, 1),
    schedule_interval=None,
    catchup=False,
    tags=['osdu', 'well-execution', 'csv', 'ingestion'],
) as dag:
    parse_task = PythonOperator(
        task_id='parse_csv_well_execution',
        python_callable=parse_csv_well_execution,
        op_kwargs={
            'file_source': '{{ dag_run.conf.file_source }}',
            'wellbore_id': '{{ dag_run.conf.wellbore_id }}',
            'data_partition_id': '{{ dag_run.conf.data_partition_id }}'
        }
    )
```

### 5.3 Pathway 3: Document Ingestion (Daily Drilling Reports)

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
Content Extraction (Optional)
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
  - file: well-001-ddr-2024-01-20.pdf (binary)

Response (200 OK):
{
  "FileID": "8jj01gi70ei6896f2iidg7837j5jg43g",
  "FileSource": "srn:type:file/pdf:ddr-001:",
  "Location": "gs://osdu-storage/osdu/files/8jj01gi70ei6896f2iidg7837j5jg43g/well-001-ddr-2024-01-20.pdf"
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
  - Extract report date (from filename or document)
  - Extract activity type (if in metadata)

Step 2.2: Extract Document Content
Document Parser:
  - Extract text content for full-text search
  - Extract structured data (if available):
    * Drilling parameters (ROP, WOB, RPM, etc.)
    * Depth progress
    * Weather conditions
    * Personnel
    * Equipment
  - Generate document summary

Step 2.3: Parse Daily Drilling Report Structure
Document Parser:
  - Parse report header:
    * Well name
    * Report date
    * Rig information
  - Parse daily summary:
    * Activities
    * Progress
    * Issues
  - Parse drilling parameters:
    * ROP, WOB, RPM, torque, flow rate
  - Parse depth progress:
    * Current depth
    * Footage drilled
  - Parse NPT events (if present)

┌─────────────────────────────────────────────────────────────┐
│ Phase 3: Storage Service Metadata Ingestion                │
└─────────────────────────────────────────────────────────────┘

Step 3.1: Create WellExecution WPC Record
Document Parser → Storage Service API
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
      "Name": "Well-001 Daily Drilling Report - 2024-01-20",
      "Description": "Daily drilling report for Well-001 on 2024-01-20",
      "ActivityType": "Drilling",
      "ActivityTypeID": "osdu:reference-data--ActivityType:Drilling:",
      "StartDate": "2024-01-20T00:00:00Z",
      "EndDate": "2024-01-20T23:59:59Z",
      "DailyReports": [
        {
          "reportDate": "2024-01-20",
          "content": "Extracted report content...",
          "drillingMetrics": {
            "rop": 25.5,
            "wob": 50000,
            "rpm": 120
          },
          "depthProgress": {
            "currentDepth": 1000,
            "footageDrilled": 100
          }
        }
      ],
      "DataGroupTypeProperties": {
        "Files": [
          "srn:type:file/pdf:ddr-001:"
        ]
      },
      "Datasets": [
        "osdu:dataset--File.Generic:ddr-001:"
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

**Document Parser Implementation**:

```python
# document_well_execution_parser.py
from pdfminer.high_level import extract_text
from docx import Document
import re
from datetime import datetime

def parse_ddr_document(document_file, wellbore_id, data_partition_id):
    """
    Parse Daily Drilling Report document and create well execution records
    """
    # 1. Read document
    file_extension = document_file.split('.')[-1].lower()
    
    if file_extension == 'pdf':
        text_content = extract_text(document_file)
    elif file_extension in ['doc', 'docx']:
        doc = Document(document_file)
        text_content = '\n'.join([para.text for para in doc.paragraphs])
    else:
        raise ValueError(f"Unsupported document format: {file_extension}")
    
    # 2. Extract metadata
    report_date = extract_report_date(document_file, text_content)
    well_name = extract_well_name(text_content)
    
    # 3. Extract structured data
    drilling_metrics = extract_drilling_metrics(text_content)
    depth_progress = extract_depth_progress(text_content)
    npt_events = extract_npt_events(text_content)
    
    # 4. Create daily report
    daily_report = {
        "reportDate": report_date,
        "content": text_content[:1000],  # First 1000 characters
        "drillingMetrics": drilling_metrics,
        "depthProgress": depth_progress
    }
    
    # 5. Store in Well Delivery DDMS (optional, for structured data)
    if drilling_metrics or depth_progress:
        execution_id = well_delivery_ddms.store_execution(
            wellbore_id=wellbore_id,
            execution_data={
                "activityType": "Drilling",
                "startDate": f"{report_date}T00:00:00Z",
                "endDate": f"{report_date}T23:59:59Z",
                "dailyReports": [daily_report],
                "nonProductiveTime": {
                    "totalHours": sum(e['duration'] for e in npt_events),
                    "events": npt_events
                }
            }
        )
    
    # 6. Create execution record
    storage_service.ingest_records([{
        "kind": f"{data_partition_id}:wks:work-product-component--WellExecution:1.0.0",
        "data": {
            "ResourceTypeID": "srn:type:work-product-component/WellExecution:",
            "WellboreId": wellbore_id,
            "Name": f"{well_name} Daily Drilling Report - {report_date}",
            "ActivityType": "Drilling",
            "ActivityTypeID": f"{data_partition_id}:reference-data--ActivityType:Drilling:",
            "StartDate": f"{report_date}T00:00:00Z",
            "EndDate": f"{report_date}T23:59:59Z",
            "DailyReports": [daily_report],
            "NonProductiveTime": {
                "totalHours": sum(e['duration'] for e in npt_events),
                "events": npt_events
            },
            "DataGroupTypeProperties": {
                "Files": [document_file]
            }
        }
    }])
    
    return {"status": "success"}

def extract_drilling_metrics(text):
    """
    Extract drilling metrics from text
    """
    metrics = {}
    
    # Extract ROP
    rop_match = re.search(r'ROP[:\s]+(\d+\.?\d*)', text, re.IGNORECASE)
    if rop_match:
        metrics['rop'] = float(rop_match.group(1))
    
    # Extract WOB
    wob_match = re.search(r'WOB[:\s]+(\d+)', text, re.IGNORECASE)
    if wob_match:
        metrics['wob'] = float(wob_match.group(1))
    
    # Extract RPM
    rpm_match = re.search(r'RPM[:\s]+(\d+)', text, re.IGNORECASE)
    if rpm_match:
        metrics['rpm'] = float(rpm_match.group(1))
    
    return metrics

def extract_depth_progress(text):
    """
    Extract depth progress from text
    """
    progress = {}
    
    # Extract current depth
    depth_match = re.search(r'(?:current\s+)?depth[:\s]+(\d+\.?\d*)', text, re.IGNORECASE)
    if depth_match:
        progress['currentDepth'] = float(depth_match.group(1))
    
    # Extract footage drilled
    footage_match = re.search(r'footage[:\s]+(\d+\.?\d*)', text, re.IGNORECASE)
    if footage_match:
        progress['footageDrilled'] = float(footage_match.group(1))
    
    return progress
```

---

## 6. Storage Architecture

### 6.1 Metadata vs. Bulk Data Separation

**Metadata Storage (Storage Service)**:

```json
{
  "id": "osdu:doc:4ff67ce36ae2452b8ddad3391f1fc08a",
  "version": 1582725123640845,
  "kind": "osdu:wks:work-product-component--WellExecution:1.0.0",
  "data": {
    "ResourceTypeID": "srn:type:work-product-component/WellExecution:",
    "WellboreId": "osdu:master-data--Wellbore:001:",
    "Name": "Well-001 Drilling Execution",
    "Description": "Drilling execution for Well-001",
    "ActivityType": "Drilling",
    "ActivityTypeID": "osdu:reference-data--ActivityType:Drilling:",
    "StartDate": "2024-01-20T00:00:00Z",
    "EndDate": "2024-01-25T00:00:00Z",
    "Duration": 120.0,
    "DurationUnit": "Hour",
    "OperationalEvents": [
      {
        "eventType": "Drilling",
        "eventDescription": "Drilling section 1",
        "timestamp": "2024-01-20T08:00:00Z",
        "depth": 1000.0
      }
    ],
    "DailyReports": [
      {
        "reportDate": "2024-01-20",
        "content": "Drilling section 1. Progress: 1000 ft."
      }
    ],
    "NonProductiveTime": {
      "totalHours": 12.0,
      "events": [
        {
          "eventType": "Equipment Failure",
          "startTime": "2024-01-22T14:00:00Z",
          "endTime": "2024-01-22T18:00:00Z",
          "duration": 4.0,
          "description": "Pump failure",
          "category": "Equipment"
        }
      ]
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
```

**Bulk Data Storage (Well Delivery DDMS)**:

```
Well Delivery DDMS Backend
    ↓
Wellbore: wellbore-001
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
        "duration": 120.0,
        "operationalEvents": [
          {
            "eventType": "Drilling",
            "eventDescription": "Drilling section 1",
            "timestamp": "2024-01-20T08:00:00Z",
            "depth": 1000.0
          }
        ],
        "dailyReports": [
          {
            "reportDate": "2024-01-20",
            "content": "...",
            "drillingMetrics": {...},
            "depthProgress": {...}
          }
        ],
        "nonProductiveTime": {
          "totalHours": 12.0,
          "events": [...]
        },
        "operationalMetrics": {...}
      }
    }
    ↓
Indexed for Queries:
    - Wellbore index for fast lookups
    - Date range index (StartDate, EndDate)
    - Activity type index
    - Event type index
    - NPT category index
```

### 6.2 File Storage

**Source File Preservation**:

```
File Service (Object Store)
    ↓
Source File Storage:
    gs://osdu-storage/osdu/files/{file-id}/well-001-execution.xml
    gs://osdu-storage/osdu/files/{file-id}/well-001-execution.csv
    gs://osdu-storage/osdu/files/{file-id}/well-001-ddr-2024-01-20.pdf
    ↓
Dataset Record Reference:
    {
      "kind": "osdu:wks:dataset--File.Generic:1.0.0",
      "data": {
        "DatasetProperties": {
          "FileSourceInfo": {
            "FileSource": "gs://osdu-storage/osdu/files/{file-id}/well-001-execution.xml"
          }
        }
      }
    }
    ↓
Linked to WellExecution:
    WellExecution.data.DataGroupTypeProperties.Files = [
      "srn:type:file/witsml:execution-001:"
    ]
```

### 6.3 Well Delivery DDMS Architecture

**Bulk Data Storage**:

Well Delivery DDMS provides optimized storage and access for well execution bulk data:

**API Endpoints**:

```http
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
    "dailyReports": [...],
    "nonProductiveTime": {...}
  }
}

# Retrieve well execution
GET /api/os-well-delivery-ddms/wellbores/{wellboreId}/execution/{executionId}

# Query by date range
GET /api/os-well-delivery-ddms/wellbores/{wellboreId}/execution?startDate=2024-01-01&endDate=2024-01-31

# Query by activity type
GET /api/os-well-delivery-ddms/wellbores/{wellboreId}/execution?activityType=Drilling

# Query operational events
GET /api/os-well-delivery-ddms/wellbores/{wellboreId}/execution/{executionId}/events?eventType=Drilling

# Query daily reports
GET /api/os-well-delivery-ddms/wellbores/{wellboreId}/execution/{executionId}/reports?startDate=2024-01-20&endDate=2024-01-25

# Query NPT
GET /api/os-well-delivery-ddms/wellbores/{wellboreId}/execution/{executionId}/npt?category=Equipment
```

**Storage Benefits**:

- **Optimized Access**: Efficient retrieval of large operational datasets
- **Time Indexing**: Fast queries by date ranges (StartDate, EndDate)
- **Activity Indexing**: Fast queries by activity type
- **Event Indexing**: Fast queries by event type
- **NPT Indexing**: Fast queries by NPT category
- **Compression**: Efficient storage of repetitive operational data
- **Caching**: In-memory caching for frequently accessed execution data
- **Time-Series Support**: Optimized for time-series operational data

---

## 7. Data Transformation & Validation

### 7.1 Reference Data Resolution

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

**Event Type Resolution**:

```
Input: "Drilling" or "Tripping" or "Casing"
    ↓
Reference Data Service Lookup
    GET /api/reference-data/v2/reference-data/osdu:reference-data--EventType:Drilling:
    ↓
Response:
    {
      "code": "Drilling",
      "name": "Drilling Event",
      "description": "Drilling operation event"
    }
    ↓
Resolved Reference:
    "osdu:reference-data--EventType:Drilling:"
```

### 7.2 Duration Calculation

**Duration from Start/End Dates**:

```python
def calculate_duration(start_date, end_date):
    """
    Calculate duration from start and end dates
    """
    start = datetime.fromisoformat(start_date)
    end = datetime.fromisoformat(end_date)
    duration_seconds = (end - start).total_seconds()
    duration_hours = duration_seconds / 3600.0
    return duration_hours
```

### 7.3 NPT Aggregation

**NPT Event Aggregation**:

```python
def aggregate_npt_events(npt_events):
    """
    Aggregate NPT events and calculate totals
    """
    total_hours = 0.0
    by_category = {}
    
    for event in npt_events:
        duration = event.get('duration', 0.0)
        total_hours += duration
        
        category = event.get('category', 'Other')
        if category not in by_category:
            by_category[category] = 0.0
        by_category[category] += duration
    
    return {
        "totalHours": total_hours,
        "byCategory": by_category,
        "events": npt_events
    }
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
    - ActivityType → Filter by activity
    - StartDate, EndDate → Date range queries
    - Duration → Filter by duration
    - OperationalEvents → Event type queries
    - NonProductiveTime → NPT queries
    - DailyReports → Report content search
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
  "kind": "osdu:wks:work-product-component--WellExecution:1.0.0",
  "query": "data.WellboreId:\"osdu:master-data--Wellbore:001:\""
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

**Search by NPT Category**:

```http
POST /api/search/v2/query
Content-Type: application/json
Authorization: Bearer <JWT>
Slb-Data-Partition-Id: osdu

{
  "kind": "osdu:wks:work-product-component--WellExecution:1.0.0",
  "query": "data.NonProductiveTime.events.category:\"Equipment\""
}
```

**Full-Text Search in Daily Reports**:

```http
POST /api/search/v2/query
Content-Type: application/json
Authorization: Bearer <JWT>
Slb-Data-Partition-Id: osdu

{
  "kind": "osdu:wks:work-product-component--WellExecution:1.0.0",
  "query": "data.DailyReports.content:\"pump failure\""
}
```

---

## 9. Error Handling & Validation

### 9.1 Validation Errors

**Common Validation Failures**:

1. **Missing Wellbore Reference**:
   ```
   Error: WellboreId not specified or invalid
   Resolution: Provide valid WellboreId reference
   ```

2. **Invalid Date Format**:
   ```
   Error: StartDate format invalid
   Resolution: Use ISO 8601 datetime format (YYYY-MM-DDTHH:mm:ssZ)
   ```

3. **Invalid Date Range**:
   ```
   Error: EndDate (2024-01-20) is before StartDate (2024-01-25)
   Resolution: Ensure EndDate is after StartDate
   ```

4. **Invalid Reference Data**:
   ```
   Error: ActivityTypeID reference not found
   Resolution: Ensure reference exists in Reference Data Service
   ```

5. **Invalid Event Data**:
   ```
   Error: Operational event timestamp missing
   Resolution: Provide timestamp for all operational events
   ```

### 9.2 Schema Validation

**Required Field Validation**:
- Fields marked as required in schema must be present
- Missing required fields cause ingestion failure

**Data Type Validation**:
- String fields must be strings
- Date fields must be valid datetimes
- Array fields must be arrays
- Object fields must be objects
- Numeric fields must be numbers

**Reference Validation**:
- Master data references (WellboreId) must exist
- Reference data values (ActivityTypeID) must be valid
- Dataset references must exist

**Example Validation Error**:

```json
{
  "error": "Schema validation failed",
  "kind": "osdu:wks:work-product-component--WellExecution:1.0.0",
  "errors": [
    {
      "path": "data.WellboreId",
      "message": "Required field missing"
    },
    {
      "path": "data.StartDate",
      "message": "Expected DateTime, got String"
    },
    {
      "path": "data.OperationalEvents[0].timestamp",
      "message": "Required field missing"
    }
  ]
}
```

---

## 10. Performance Considerations

### 10.1 Batch Ingestion

**Optimal Batch Size**:

- **Small executions (< 100 events)**: Process individually
- **Medium executions (100-1000 events)**: Batch by wellbore
- **Large executions (> 1000 events)**: Chunk processing recommended

**Recommended**: Process well execution records individually, batch metadata records (100-200 per batch)

### 10.2 Event Processing

**Efficient Event Processing**:

```python
def process_operational_events(events, chunk_size=1000):
    """
    Process operational events in chunks
    """
    # 1. Validate all events first
    validate_events(events)
    
    # 2. Sort by timestamp
    sorted_events = sorted(events, key=lambda e: e['timestamp'])
    
    # 3. Process in chunks if large
    if len(sorted_events) > chunk_size:
        chunks = [sorted_events[i:i+chunk_size] for i in range(0, len(sorted_events), chunk_size)]
        for chunk in chunks:
            well_delivery_ddms.store_events_chunk(chunk)
    else:
        well_delivery_ddms.store_events(sorted_events)
    
    return sorted_events
```

### 10.3 Document Processing

**Efficient Document Processing**:

```python
def process_ddr_documents(documents, chunk_size=10):
    """
    Process daily drilling report documents in chunks
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

## 11. Design Patterns and Best Practices

### 11.1 Schema First Approach

1. **Define schemas before ingestion**: Ensures type safety and proper indexing
2. **Version schemas carefully**: Schema changes require data migration
3. **Index only necessary fields**: Reduces index size and improves performance
4. **Validate at ingestion**: Catch errors early in the pipeline

### 11.2 Work Product Pattern

1. **Group related WPCs**: Use Work Products to organize related execution records
2. **Maintain relationships**: Link execution records to wellbores and activities
3. **Preserve lineage**: Track source files and processing history

### 11.3 Well Delivery DDMS Usage

1. **Use Well Delivery DDMS for bulk data**: Optimized storage for operational data
2. **Store metadata separately**: Keep metadata in Storage Service for searchability
3. **Leverage indexing**: Use date, activity type, and event type indexes for efficient queries

### 11.4 Time-Based Data Handling

1. **Use ISO 8601 format**: Always use ISO 8601 datetime format
2. **Validate date ranges**: Ensure EndDate is after StartDate
3. **Index by date ranges**: Enable efficient date range queries
4. **Handle time zones**: Normalize to UTC for consistency

### 11.5 Document Management

1. **Preserve original documents**: Keep documents in File Service for traceability
2. **Extract metadata**: Extract structured metadata for searchability
3. **Enable full-text search**: Index document content for full-text search
4. **Parse structured data**: Extract drilling parameters and metrics when possible

---

## 12. Important Considerations

### 12.1 Data Partitioning

- All well execution operations require `data-partition-id` header
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

- Each well execution record must be associated with a Wellbore
- WellboreId must exist in Master Data Service
- Wellbore reference is validated during ingestion

### 12.6 Time-Based Data

- Well execution data is inherently time-based
- StartDate and EndDate define the execution period
- Operational events are timestamped
- Daily reports are date-based
- NPT events have start and end times

---

## 13. Summary

OSDU provides **comprehensive, multi-format ingestion** for well execution data through specialized services:

1. **Well Delivery DDMS**: Optimized bulk storage for well execution operational data
2. **WITSML Parser**: Industry-standard WITSML format support with full operational data extraction
3. **CSV Parser**: Automated ingestion from CSV files with structured operational data
4. **Document Ingestion**: Support for PDF, Word, and Excel documents with metadata extraction
5. **Manifest-based**: Structured JSON manifests for complex workflows
6. **Direct API**: Programmatic record creation for custom applications

**Key Design Decisions**:

- **Metadata/Bulk Separation**: Metadata in Storage Service, bulk data in Well Delivery DDMS
- **Time-Based Data**: Supports time-series operational data and event-based records
- **Multi-Format Support**: WITSML, CSV, JSON, and document formats
- **Document Management**: Handles both structured data and unstructured documents
- **Event Tracking**: Supports detailed operational event tracking with timestamps
- **NPT Tracking**: Comprehensive non-productive time tracking and analysis
- **Source File Preservation**: Original files stored alongside processed data
- **Search Integration**: Automatic indexing for discovery, full-text search, and time-based queries
- **Access Control**: ACLs and legal tags enforced at ingestion time

**Call Stack Highlights**:

- **WITSML Parser**: File upload → DAG trigger → WITSML parsing → Operational data extraction → Well Delivery DDMS storage → Metadata ingest → Index
- **CSV Parser**: File upload → DAG trigger → CSV parsing → Data extraction → Well Delivery DDMS storage → Metadata ingest → Index
- **Document Ingestion**: File upload → Document parsing → Metadata/content extraction → Storage API → Index (with full-text search)
- **Manifest**: JSON creation → Workflow API → Parse & validate → Well Delivery DDMS/Storage → Batch ingest → Index
- **Direct API**: Application code → Well Delivery DDMS (if needed) → Storage API → Index

This architecture ensures **data consistency**, **operational traceability**, **document preservation**, **time-series support**, **traceability**, and **governance** while supporting multiple ingestion formats and comprehensive well execution data management.

---

## 14. References

1. OSDU Data Formats, Schemas & Storage Architecture Documentation
2. OSDU Well Delivery/Planning Ingestion Analysis Documentation
3. OSDU Storage Service API Documentation
4. OSDU Well Delivery DDMS API Documentation
5. OSDU Workflow Service API Documentation
6. OSDU File Service API Documentation
7. OSDU Search Service API Documentation
8. OSDU Reference Data Service API Documentation
9. WITSML 2.0 Specification
10. Daily Drilling Report (DDR) Standards

