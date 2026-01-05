# OSDU Well Tops Ingestion & Storage Architecture

This document explains how OSDU handles well tops (wellbore markers, picks, horizons, formations) when ingesting and storing different data formats, including the design architecture and call stack for related workflows.

## Overview

OSDU handles well tops data through **multiple ingestion pathways** depending on the source format:

1. **Manifest-based Ingestion**: Structured JSON manifests for wellbore marker records
2. **CSV Parser Ingestion**: Automated CSV file parsing into OSDU schemas
3. **Direct Storage API**: Programmatic record creation via Storage Service API
4. **Workflow-based Ingestion**: Orchestrated ingestion via Apache Airflow DAGs

**Key Principles:**

- **Schema-Driven**: Well tops conform to OSDU Work Product Component schemas
- **Work Product Model**: Wellbore markers are Work Product Components within Work Products
- **Multi-Format Support**: CSV, JSON, WITSML, and direct API ingestion
- **Metadata Preservation**: Source file references maintained alongside structured records
- **Legal & Access Control**: ACLs and legal tags enforced at ingestion time

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                    OSDU Data Platform                        │
│                                                             │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  Ingestion Pathways                                 │   │
│  │  ┌──────────────┐ ┌──────────────┐ ┌──────────────┐ │   │
│  │  │ CSV Parser  │ │  Manifest   │ │ Direct API   │ │   │
│  │  │   DAG       │ │  Workflow   │ │  Storage     │ │   │
│  │  └──────┬──────┘ └──────┬───────┘ └──────┬───────┘ │   │
│  └─────────┼───────────────┼────────────────┼─────────┘   │
│            │               │                 │              │
│            ▼               ▼                 ▼              │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  Schema Validation & Transformation                  │   │
│  │  - Validate against WellboreMarker schema           │   │
│  │  - Map CSV columns to OSDU fields                   │   │
│  │  - Resolve reference data (units, classifications)  │   │
│  └──────────────────────┬──────────────────────────────┘   │
│                         │                                    │
│                         ▼                                    │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  Storage Service                                     │   │
│  │  - Create WorkProduct record                        │   │
│  │  - Create WellboreMarker WorkProductComponent       │   │
│  │  - Create Dataset record (if source file exists)     │   │
│  │  - Store ACLs and legal tags                        │   │
│  └──────────────────────┬──────────────────────────────┘   │
│                         │                                    │
│                         ▼                                    │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  Indexer Service                                     │   │
│  │  - Index wellbore markers for search                │   │
│  │  - Extract searchable metadata                      │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                             │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  File Storage (Object Store)                        │   │
│  │  - Store source CSV/JSON files                      │   │
│  │  - Preserve original data format                    │   │
│  └─────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
```

## Well Tops Data Formats

### 1. Wellbore Marker Schema Structure

OSDU represents well tops as **WellboreMarker** Work Product Components. The schema includes:

```json
{
	"kind": "{{data-partition-id}}:wks:work-product-component--WellboreMarker:1.0.0",
	"data": {
		"ResourceTypeID": "srn:type:work-product-component/WellboreMarker:",
		"ResourceSecurityClassification": "srn:reference-data/ResourceSecurityClassification:RESTRICTED:",
		"Data": {
			"GroupTypeProperties": {
				"Files": ["srn:type:file/csv:66153425987012541:"],
				"Artefacts": []
			},
			"IndividualTypeProperties": {
				"Name": "1100.csv",
				"Description": "Wellbore Marker",
				"WellboreID": "srn:master-data/Wellbore:1100:",
				"DepthUnit": "srn:reference-data/UnitOfMeasure:M:",
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
			},
			"ExtensionProperties": {}
		},
		"ComponentsAssociativeIDs": ["wpc-1"],
		"ResourceID": "srn:type:work-product-component/WellboreMarker:66153425987012541:"
	},
	"acl": {
		"viewers": ["data.default.viewers@{{data-partition-id}}.osdu-gcp.go3-nrg.projects.epam.com"],
		"owners": ["data.default.owners@{{data-partition-id}}.osdu-gcp.go3-nrg.projects.epam.com"]
	},
	"legal": {
		"legaltags": ["{{data-partition-id}}-demo-legaltag"],
		"otherRelevantDataCountries": ["US"],
		"status": "compliant"
	}
}
```

**Key Fields:**

- **WellboreID**: Reference to the wellbore master data record
- **DepthUnit**: Reference to unit of measure (e.g., meters, feet)
- **Markers**: Array of marker objects with:
  - **MarkerName**: Formation/horizon name (e.g., "Breda Formation")
  - **MarkerMeasuredDepth**: Depth value in the specified unit
- **Files**: References to source files (CSV, JSON, etc.)
- **ResourceSecurityClassification**: Data classification level

### 2. CSV Format Example

CSV files for well tops typically contain:

```csv
WellboreID,MarkerName,MarkerMeasuredDepth,DepthUnit,Description
1100,QUATER. UNDIFF.,0.0,M,Quaternary Undifferentiated
1100,Breda Formation,133.0,M,Breda Formation
1100,Rupel Formation,215.0,M,Rupel Formation
1100,Brussels Sand Member,307.0,M,Brussels Sand Member
1100,Ieper Member,410.0,M,Ieper Member
1100,Basal Dongen Tuffite Member,573.0,M,Basal Dongen Tuffite Member
1100,Landen Clay Member,594.0,M,Landen Clay Member
1100,Ommelanden Formation,608.0,M,Ommelanden Formation
```

**CSV Column Mapping:**

- **WellboreID** → `WellboreID` (must reference existing wellbore)
- **MarkerName** → `Markers[].MarkerName`
- **MarkerMeasuredDepth** → `Markers[].MarkerMeasuredDepth`
- **DepthUnit** → `DepthUnit` (reference data lookup)
- **Description** → `Description` (optional)

### 3. JSON Manifest Format

For manifest-based ingestion, well tops are defined within a Work Product manifest:

```json
{
	"manifest": {
		"kind": "{{data-partition-id}}:wks:Manifest:1.0.0",
		"Data": {
			"WorkProduct": {
				"kind": "{{data-partition-id}}:wks:work-product--WorkProduct:1.0.0",
				"data": {
					"Name": "Well Tops Work Product",
					"Description": "Wellbore markers for well 1100",
					"Components": ["surrogate-key: wpc1"]
				}
			},
			"WorkProductComponents": [
				{
					"id": "surrogate-key: wpc1",
					"kind": "{{data-partition-id}}:wks:work-product-component--WellboreMarker:1.0.0",
					"data": {
						"WellboreID": "{{data-partition-id}}:master-data--Wellbore:1100:",
						"DepthUnit": "{{data-partition-id}}:reference-data--UnitOfMeasure:M:",
						"Markers": [
							{
								"MarkerName": "Breda Formation",
								"MarkerMeasuredDepth": 133.0
							}
						]
					}
				}
			],
			"Datasets": [
				{
					"id": "surrogate-key: dataset1",
					"kind": "{{data-partition-id}}:wks:dataset--File.Generic:1.0.0",
					"data": {
						"DatasetProperties": {
							"FileSourceInfo": {
								"FileSource": "gs://bucket/well-tops/1100.csv"
							}
						}
					}
				}
			]
		}
	}
}
```

## Ingestion Pathways

### Pathway 1: CSV Parser Ingestion

**Architecture:**

```
CSV File Upload
    ↓
File Service (staging)
    ↓
CSV Parser DAG (Airflow)
    ↓
Column Mapping & Validation
    ↓
Schema Transformation
    ↓
Storage Service (record creation)
    ↓
Indexer Service (search indexing)
```

**Call Stack:**

```
1. CSV File Upload
   POST /v2/files/upload
   → Returns FileSource identifier

2. Trigger CSV Parser DAG
   POST /v1/workflow/Osdu_ingest/workflowRun
   {
     "executionContext": {
       "Payload": {
         "AppKey": "csv-parser",
         "data-partition-id": "{{data-partition-id}}",
         "FileSource": "srn:type:file/csv:66153425987012541:"
       }
     }
   }
   → Returns WorkflowID

3. CSV Parser Processing (Airflow DAG)
   a. Read CSV file from staging
   b. Parse header row (column names)
   c. For each data row:
      - Extract WellboreID, MarkerName, MarkerMeasuredDepth, DepthUnit
      - Validate WellboreID exists in master data
      - Resolve DepthUnit reference data
      - Group markers by WellboreID
   d. Transform to WellboreMarker schema
   e. Create WorkProduct and WorkProductComponent records

4. Storage Service Ingestion
   PUT /api/storage/v2/records
   [
     {
       "kind": "{{data-partition-id}}:wks:work-product--WorkProduct:1.0.0",
       "data": { ... }
     },
     {
       "kind": "{{data-partition-id}}:wks:work-product-component--WellboreMarker:1.0.0",
       "data": { ... }
     }
   ]
   → Returns record IDs

5. Indexer Service
   - Indexes WellboreMarker records
   - Extracts searchable fields (WellboreID, MarkerName, depths)
   - Updates search index
```

**CSV Parser DAG Configuration:**

```python
# csv_ingestion_all_steps.py (Airflow DAG)
from airflow import DAG
from airflow.operators.python import PythonOperator

def parse_csv_well_tops(file_source, data_partition_id):
    """
    Parse CSV file and create WellboreMarker records
    """
    # 1. Read CSV from staging
    csv_data = file_service.read(file_source)

    # 2. Parse CSV
    import csv
    reader = csv.DictReader(csv_data)

    # 3. Group by WellboreID
    wellbore_markers = {}
    for row in reader:
        wellbore_id = row['WellboreID']
        if wellbore_id not in wellbore_markers:
            wellbore_markers[wellbore_id] = []

        wellbore_markers[wellbore_id].append({
            "MarkerName": row['MarkerName'],
            "MarkerMeasuredDepth": float(row['MarkerMeasuredDepth'])
        })

    # 4. Create records for each wellbore
    records = []
    for wellbore_id, markers in wellbore_markers.items():
        # Validate wellbore exists
        wellbore = storage_service.get_record(wellbore_id)
        if not wellbore:
            raise ValueError(f"Wellbore {wellbore_id} not found")

        # Resolve depth unit
        depth_unit = reference_data_service.resolve_unit(row['DepthUnit'])

        # Create WellboreMarker record
        marker_record = {
            "kind": f"{data_partition_id}:wks:work-product-component--WellboreMarker:1.0.0",
            "data": {
                "WellboreID": wellbore_id,
                "DepthUnit": depth_unit,
                "Markers": markers
            }
        }
        records.append(marker_record)

    # 5. Ingest records
    storage_service.ingest_records(records)

    return {"status": "success", "records_created": len(records)}
```

### Pathway 2: Manifest-Based Ingestion

**Architecture:**

```
JSON Manifest Creation
    ↓
Workflow Service API
    ↓
Manifest Parser
    ↓
Schema Validation
    ↓
Storage Service (batch ingestion)
    ↓
Indexer Service
```

**Call Stack:**

```
1. Create Manifest JSON
   {
     "manifest": {
       "kind": "{{data-partition-id}}:wks:Manifest:1.0.0",
       "Data": {
         "WorkProduct": { ... },
         "WorkProductComponents": [
           {
             "kind": "{{data-partition-id}}:wks:work-product-component--WellboreMarker:1.0.0",
             "data": { ... }
           }
         ]
       }
     }
   }

2. Submit Manifest to Workflow Service
   POST /v1/workflow/Osdu_ingest/workflowRun
   {
     "executionContext": {
       "Payload": {
         "AppKey": "manifest-ingestion",
         "data-partition-id": "{{data-partition-id}}"
       },
       "manifest": { ... }
     }
   }
   → Returns WorkflowID

3. Manifest Parser Processing
   a. Validate manifest structure
   b. Extract WorkProduct and WorkProductComponents
   c. For each WellboreMarker component:
      - Validate WellboreID reference
      - Validate DepthUnit reference
      - Validate marker array structure
   d. Resolve all reference data

4. Storage Service Batch Ingestion
   PUT /api/storage/v2/records
   [
     WorkProduct record,
     WellboreMarker WorkProductComponent record,
     Dataset record (if file source provided)
   ]
   → Returns record IDs with surrogate keys resolved

5. Indexer Service
   - Indexes all ingested records
   - Creates searchable metadata
```

**Manifest Validation Rules:**

- **WellboreID**: Must reference existing `master-data--Wellbore` record
- **DepthUnit**: Must reference existing `reference-data--UnitOfMeasure` record
- **Markers Array**: Each marker must have `MarkerName` and `MarkerMeasuredDepth`
- **ACL**: Must include valid `owners` and `viewers` arrays
- **Legal Tags**: Must include valid legal tag from legal service

### Pathway 3: Direct Storage API Ingestion

**Architecture:**

```
Application Code
    ↓
Storage Service API
    ↓
Schema Validation
    ↓
Record Creation
    ↓
Indexer Service
```

**Call Stack:**

```
1. Prepare WellboreMarker Record
   const markerRecord = {
     "kind": "{{data-partition-id}}:wks:work-product-component--WellboreMarker:1.0.0",
     "acl": {
       "owners": ["data.default.owners@{{data-partition-id}}.osdu-gcp.go3-nrg.projects.epam.com"],
       "viewers": ["data.default.viewers@{{data-partition-id}}.osdu-gcp.go3-nrg.projects.epam.com"]
     },
     "legal": {
       "legaltags": ["{{data-partition-id}}-demo-legaltag"],
       "otherRelevantDataCountries": ["US"]
     },
     "data": {
       "WellboreID": "{{data-partition-id}}:master-data--Wellbore:1100:",
       "DepthUnit": "{{data-partition-id}}:reference-data--UnitOfMeasure:M:",
       "Markers": [
         {
           "MarkerName": "Breda Formation",
           "MarkerMeasuredDepth": 133.0
         }
       ]
     }
   };

2. Ingest Record via Storage API
   PUT /api/storage/v2/records
   Authorization: Bearer <JWT>
   Content-Type: application/json
   Slb-Data-Partition-Id: {{data-partition-id}}

   [markerRecord]

   → Response:
   {
     "recordIds": [
       "{{data-partition-id}}:doc:4ff67ce36ae2452b8ddad3391f1fc08a"
     ],
     "skippedRecordIds": [],
     "recordIdVersions": [
       {
         "id": "{{data-partition-id}}:doc:4ff67ce36ae2452b8ddad3391f1fc08a",
         "version": 1582725123640845
       }
     ]
   }

3. Storage Service Processing
   a. Validate schema against WellboreMarker kind
   b. Validate ACL format and permissions
   c. Validate legal tags exist and user has permission
   d. Validate WellboreID reference exists
   e. Validate DepthUnit reference exists
   f. Create record with version
   g. Store in storage backend

4. Indexer Service (async)
   - Receives record creation event
   - Extracts searchable fields
   - Updates search index
   - Makes record discoverable via Search Service
```

## Data Transformation & Validation

### Reference Data Resolution

**Depth Unit Resolution:**

```
Input: "M" or "meters" or "m"
    ↓
Unit Service Lookup
    POST /v2/unit/search
    {
      "keywords": [
        {"code": "Symbol", "value": "M"},
        {"code": "Namespace", "value": "Energistics_UoM"}
      ]
    }
    ↓
Response:
    {
      "units": [
        {
          "ancestry": "L.M",
          "code": "M",
          "name": "meter",
          "namespace": "Energistics_UoM"
        }
      ]
    }
    ↓
Resolved Reference:
    "{{data-partition-id}}:reference-data--UnitOfMeasure:M:"
```

**Wellbore Reference Validation:**

```
Input: WellboreID from CSV/manifest
    ↓
Storage Service Lookup
    GET /api/storage/v2/records/{{wellbore-id}}
    ↓
Validation:
    - Record exists
    - Record kind is "master-data--Wellbore"
    - User has read permission (ACL check)
    ↓
If valid: Use WellboreID as-is
If invalid: Reject ingestion with error
```

### CSV Column Mapping

**Automatic Mapping:**

The CSV parser performs automatic column mapping based on common naming conventions:

```python
COLUMN_MAPPINGS = {
    "WellboreID": ["WellboreID", "Wellbore", "Wellbore_ID", "Well ID"],
    "MarkerName": ["MarkerName", "Marker", "Formation", "Horizon", "Top Name"],
    "MarkerMeasuredDepth": ["MarkerMeasuredDepth", "Depth", "MD", "Measured Depth", "TVD"],
    "DepthUnit": ["DepthUnit", "Unit", "Depth Unit", "Unit of Measure"],
    "Description": ["Description", "Desc", "Comments", "Notes"]
}
```

**Mapping Process:**

```
1. Read CSV header row
2. For each expected field:
   a. Try exact match first
   b. Try case-insensitive match
   c. Try mapping from COLUMN_MAPPINGS
   d. If no match found, use column index (if known)
3. Validate required fields present:
   - WellboreID (required)
   - MarkerName (required)
   - MarkerMeasuredDepth (required)
4. Optional fields:
   - DepthUnit (defaults to "M" if not provided)
   - Description (optional)
```

## Storage Architecture

### Record Structure

**WellboreMarker Record Storage:**

```
Storage Service Backend
    ↓
Record Document:
    {
      "id": "{{data-partition-id}}:doc:4ff67ce36ae2452b8ddad3391f1fc08a",
      "version": 1582725123640845,
      "kind": "{{data-partition-id}}:wks:work-product-component--WellboreMarker:1.0.0",
      "acl": { ... },
      "legal": { ... },
      "data": {
        "WellboreID": "...",
        "DepthUnit": "...",
        "Markers": [ ... ]
      },
      "createTime": "2024-01-15T10:30:00Z",
      "createUser": "user@example.com"
    }
    ↓
Indexed Fields (for search):
    - WellboreID
    - MarkerName (from Markers array)
    - MarkerMeasuredDepth (from Markers array)
    - DepthUnit
    - CreateTime
    - CreateUser
```

### File Storage

**Source File Preservation:**

```
File Service (Object Store)
    ↓
Source File Storage:
    gs://osdu-storage/{{data-partition-id}}/files/{{file-id}}/1100.csv
    ↓
Dataset Record Reference:
    {
      "kind": "{{data-partition-id}}:wks:dataset--File.Generic:1.0.0",
      "data": {
        "DatasetProperties": {
          "FileSourceInfo": {
            "FileSource": "gs://osdu-storage/{{data-partition-id}}/files/{{file-id}}/1100.csv"
          }
        }
      }
    }
    ↓
Linked to WellboreMarker:
    WellboreMarker.data.Data.GroupTypeProperties.Files = [
      "srn:type:file/csv:66153425987012541:"
    ]
```

## Search & Discovery

### Indexing Process

```
Record Creation Event
    ↓
Indexer Service
    ↓
Extract Searchable Fields:
    - WellboreID → Searchable by wellbore name
    - MarkerName → Full-text search
    - MarkerMeasuredDepth → Range queries
    - DepthUnit → Filter by unit
    ↓
Update Search Index
    ↓
Search Service Available
```

### Query Examples

**Search by Wellbore:**

```http
POST /api/search/v2/query
{
  "kind": "{{data-partition-id}}:wks:work-product-component--WellboreMarker:1.0.0",
  "query": "data.WellboreID:\"{{data-partition-id}}:master-data--Wellbore:1100:\""
}
```

**Search by Marker Name:**

```http
POST /api/search/v2/query
{
  "kind": "{{data-partition-id}}:wks:work-product-component--WellboreMarker:1.0.0",
  "query": "data.Data.IndividualTypeProperties.Markers.MarkerName:\"Breda Formation\""
}
```

**Search by Depth Range:**

```http
POST /api/search/v2/query
{
  "kind": "{{data-partition-id}}:wks:work-product-component--WellboreMarker:1.0.0",
  "query": "data.Data.IndividualTypeProperties.Markers.MarkerMeasuredDepth:[100 TO 200]"
}
```

## Error Handling & Validation

### Validation Errors

**Common Validation Failures:**

1. **Missing WellboreID Reference:**

   ```
   Error: WellboreID "{{data-partition-id}}:master-data--Wellbore:9999:" not found
   Resolution: Create wellbore master data record first
   ```

2. **Invalid Depth Unit:**

   ```
   Error: DepthUnit "INVALID_UNIT" not found in reference data
   Resolution: Use valid unit from Unit Service (e.g., "M", "FT")
   ```

3. **Missing Required Fields:**

   ```
   Error: Required field "MarkerName" missing in marker at index 2
   Resolution: Ensure all markers have MarkerName and MarkerMeasuredDepth
   ```

4. **ACL Permission Denied:**
   ```
   Error: User does not have permission to create records with legal tag "restricted-tag"
   Resolution: Request access to legal tag or use different tag
   ```

### Retry Logic

**CSV Parser Retry Strategy:**

```python
def ingest_with_retry(records, max_retries=3):
    for attempt in range(max_retries):
        try:
            response = storage_service.ingest_records(records)
            return response
        except ValidationError as e:
            # Don't retry validation errors
            raise
        except TransientError as e:
            if attempt < max_retries - 1:
                time.sleep(2 ** attempt)  # Exponential backoff
                continue
            raise
```

## Performance Considerations

### Batch Ingestion

**Optimal Batch Size:**

- **Small batches (10-50 records)**: Lower memory usage, more API calls
- **Medium batches (100-500 records)**: Balanced performance
- **Large batches (1000+ records)**: Higher memory usage, fewer API calls, risk of timeout

**Recommended:** 100-200 records per batch for well tops ingestion

### Parallel Processing

**CSV Parser Parallelization:**

```python
# Group markers by wellbore
wellbore_groups = group_by_wellbore(csv_rows)

# Process each wellbore in parallel
with ThreadPoolExecutor(max_workers=10) as executor:
    futures = [
        executor.submit(process_wellbore_markers, wellbore_id, markers)
        for wellbore_id, markers in wellbore_groups.items()
    ]

    results = [future.result() for future in as_completed(futures)]
```

## Summary

OSDU provides **flexible, schema-driven ingestion** for well tops data through multiple pathways:

1. **CSV Parser**: Automated ingestion from CSV files with column mapping
2. **Manifest-based**: Structured JSON manifests for complex workflows
3. **Direct API**: Programmatic record creation for custom applications

**Key Design Decisions:**

- **Work Product Model**: Well tops are Work Product Components, enabling versioning and lineage
- **Reference Data**: Units and classifications resolved via Reference Data Service
- **Source File Preservation**: Original files stored alongside structured records
- **Search Integration**: Automatic indexing for discovery and querying
- **Access Control**: ACLs and legal tags enforced at ingestion time

**Call Stack Highlights:**

- **CSV Parser**: File upload → DAG trigger → Parse & validate → Transform → Ingest → Index
- **Manifest**: JSON creation → Workflow API → Parse & validate → Batch ingest → Index
- **Direct API**: Record preparation → Storage API → Validate → Create → Index

This architecture ensures **data consistency**, **traceability**, and **governance** while supporting multiple ingestion formats and use cases.
