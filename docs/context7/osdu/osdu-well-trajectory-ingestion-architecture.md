# OSDU Well Trajectory Ingestion & Storage Architecture

This document explains how OSDU handles well trajectories (survey stations, deviation data, 3D well paths) when ingesting and storing different data formats, including the design architecture and call stack for related workflows.

## Overview

OSDU handles well trajectory data through **multiple ingestion pathways** and specialized services:

1. **Wellbore DDMS**: Optimized bulk data access for trajectories, logs, and checkshots
2. **CSV Parser Ingestion**: Automated CSV file parsing into OSDU schemas
3. **WITSML Parser Ingestion**: WITSML trajectory data parsing via Energistics integration
4. **Manifest-based Ingestion**: Structured JSON manifests for trajectory records
5. **Direct Storage API**: Programmatic record creation via Storage Service API
6. **CRS Conversion Service**: Coordinate reference system transformations for trajectories

**Key Principles:**

- **Wellbore DDMS**: Specialized service for bulk trajectory data access
- **Station-Based Model**: Trajectories represented as arrays of survey stations
- **Coordinate System Support**: Multiple CRS support with conversion capabilities
- **Multi-Format Support**: CSV, WITSML, JSON, and direct API ingestion
- **Metadata & Bulk Separation**: Metadata in Storage Service, bulk data in DDMS
- **Unit Standardization**: Unit Service integration for depth, angle, and distance units

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                    OSDU Data Platform                        │
│                                                             │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  Ingestion Pathways                                 │   │
│  │  ┌──────────────┐ ┌──────────────┐ ┌──────────────┐ │   │
│  │  │ CSV Parser  │ │ WITSML Parser│ │  Manifest   │ │   │
│  │  │   DAG       │ │     DAG      │ │  Workflow   │ │   │
│  │  └──────┬──────┘ └──────┬───────┘ └──────┬───────┘ │   │
│  │         │               │                 │          │   │
│  │         ▼               ▼                 ▼          │   │
│  │  ┌──────────────────────────────────────────────┐  │   │
│  │  │  CRS Conversion Service                       │  │   │
│  │  │  - Coordinate system transformation          │  │   │
│  │  │  - MD/Inclination/Azimuth → X/Y/Z conversion │  │   │
│  │  │  - True North / Grid North conversion       │  │   │
│  │  └──────────────────────┬───────────────────────┘  │   │
│  └─────────┼───────────────┼────────────────┼─────────┘   │
│            │               │                 │              │
│            ▼               ▼                 ▼              │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  Schema Validation & Transformation                  │   │
│  │  - Validate trajectory station data                │   │
│  │  - Resolve reference data (units, CRS)            │   │
│  │  - Transform to standardized format               │   │
│  └──────────────────────┬──────────────────────────────┘   │
│                         │                                    │
│                         ▼                                    │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  Wellbore DDMS                                     │   │
│  │  - Optimized bulk trajectory storage              │   │
│  │  - Station array management                       │   │
│  │  - Efficient access patterns                     │   │
│  └──────────────────────┬──────────────────────────────┘   │
│                         │                                    │
│                         ▼                                    │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  Storage Service                                     │   │
│  │  - Trajectory metadata records                     │   │
│  │  - WorkProduct and WorkProductComponent           │   │
│  │  - Dataset records (file references)             │   │
│  │  - ACLs and legal tags                            │   │
│  └──────────────────────┬──────────────────────────────┘   │
│                         │                                    │
│                         ▼                                    │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  Indexer Service                                     │   │
│  │  - Index trajectory metadata                       │   │
│  │  - Extract searchable fields                       │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                             │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  File Storage (Object Store)                        │   │
│  │  - Store source CSV/WITSML files                   │   │
│  │  - Preserve original data format                   │   │
│  └─────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
```

## Trajectory Data Structure

### Trajectory Station Properties

A trajectory station represents a single point along the wellbore path with the following properties:

```json
{
	"md": 1500.5,
	"inclination": 85.2,
	"azimuthTN": 180.0,
	"azimuthGN": 175.5,
	"dxTN": 10.2,
	"dyTN": -5.5,
	"point": {
		"x": 100.0,
		"y": 200.0,
		"z": 1500.5,
		"trajectoryCRS": "EPSG:32631"
	},
	"wgs84Longitude": -105.2345,
	"wgs84Latitude": 39.1234,
	"dls": 2.5,
	"original": true,
	"dz": 0.1
}
```

**Key Properties:**

- **md** (float): Measured Depth from vertical reference point in `unitZ`
- **inclination** (float): Inclination angle in degrees (0.0 = vertical, 90.0 = horizontal)
- **azimuthTN** (float): True North azimuth angle in degrees (0.0/360.0 = North)
- **azimuthGN** (float): Grid North azimuth angle in degrees
- **dxTN** (float): True E-W deviation in local Cartesian CRS (unit: `unitXY`)
- **dyTN** (float): True N-S deviation in local Cartesian CRS (unit: `unitXY`)
- **point** (object): 3D coordinates in trajectory CRS
  - **x** (number): X-coordinate
  - **y** (number): Y-coordinate
  - **z** (number): Z-coordinate (depth)
  - **trajectoryCRS** (string): Coordinate Reference System identifier
- **wgs84Longitude** (float, optional): WGS 84 longitude in degrees
- **wgs84Latitude** (float, optional): WGS 84 latitude in degrees
- **dls** (float, optional): Dog Leg Severity (curvature) in `unitDls`
- **original** (bool, optional): `true` if original station, `false` if interpolated
- **dz** (float, optional): Vertical separation from reference point

### Trajectory Container Structure

A complete trajectory includes metadata and station array:

```json
{
	"trajectoryCRS": "EPSG:32631",
	"unitXY": "{\"symbol\":\"m\",\"baseMeasurement\":{\"ancestry\":\"Length\"}}",
	"unitZ": "{\"symbol\":\"m\",\"baseMeasurement\":{\"ancestry\":\"Length\"}}",
	"unitDls": "{\"symbol\":\"deg/30m\",\"baseMeasurement\":{\"ancestry\":\"Rotation_Per_Length\"}}",
	"stations": [
		{
			"md": 0,
			"inclination": 0,
			"azimuthTN": 0,
			"point": { "x": 400000, "y": 6500000, "z": 0 },
			"original": true
		},
		{
			"md": 1000,
			"inclination": 0,
			"azimuthTN": 0,
			"point": { "x": 400000, "y": 6500000, "z": -1000 },
			"original": true
		}
	],
	"method": "AzimuthalEquidistant",
	"operationsApplied": ["CRS_Conversion", "Interpolation"]
}
```

## Data Formats

### 1. CSV Format

CSV trajectory files typically contain coordinate and depth information:

```csv
X,Y,TVD,MD
606554,6080126,-30,0
606554,6080126,1665,1695
606554,6080126,2000,2030
```

**CSV Column Mapping:**

- **X** → `point.x` (easting coordinate)
- **Y** → `point.y` (northing coordinate)
- **TVD** → `point.z` (True Vertical Depth)
- **MD** → `md` (Measured Depth)
- **WellboreID** → Reference to wellbore master data (optional, can be in filename)

**Alternative CSV Formats:**

Some CSV files may include additional columns:

- **Inclination** → `inclination` (degrees)
- **Azimuth** → `azimuthTN` (degrees, True North)
- **DLS** → `dls` (Dog Leg Severity)

### 2. WITSML Format

WITSML (Wellsite Information Transfer Standard Markup Language) trajectory data:

```xml
<trajectorys xmlns="http://www.witsml.org/schemas/1series" version="1.4.1.1">
  <trajectory uidWell="well-001" uidWellbore="wellbore-001">
    <nameWell>Example Well</nameWell>
    <nameWellbore>Main Wellbore</nameWellbore>
    <trajectoryStation uid="station-001">
      <md>0.0</md>
      <inclination>0.0</inclination>
      <azimuth>0.0</azimuth>
      <location>
        <latitude>58.62877104866187</latitude>
        <longitude>1.277806753183437</longitude>
      </location>
    </trajectoryStation>
    <trajectoryStation uid="station-002">
      <md>1000.0</md>
      <inclination>0.0</inclination>
      <azimuth>0.0</azimuth>
      <location>
        <latitude>58.62877104866187</latitude>
        <longitude>1.277806753183437</longitude>
      </location>
    </trajectoryStation>
  </trajectory>
</trajectorys>
```

**WITSML Elements:**

- **trajectoryStation**: Individual survey station
- **md**: Measured Depth
- **inclination**: Inclination angle
- **azimuth**: Azimuth angle
- **location**: Geographic coordinates (latitude/longitude)

### 3. JSON Manifest Format

For manifest-based ingestion, trajectories are defined within Work Product manifests:

```json
{
	"manifest": {
		"kind": "{{data-partition-id}}:wks:Manifest:1.0.0",
		"Data": {
			"WorkProduct": {
				"kind": "{{data-partition-id}}:wks:work-product--WorkProduct:1.0.0",
				"data": {
					"Name": "Well Trajectory Work Product",
					"Description": "Trajectory for well 1100",
					"Components": ["surrogate-key: wpc1"]
				}
			},
			"WorkProductComponents": [
				{
					"id": "surrogate-key: wpc1",
					"kind": "{{data-partition-id}}:wks:work-product-component--Trajectory:1.0.0",
					"data": {
						"WellboreID": "{{data-partition-id}}:master-data--Wellbore:1100:",
						"TrajectoryCRS": "EPSG:32631",
						"UnitXY": "{{data-partition-id}}:reference-data--UnitOfMeasure:M:",
						"UnitZ": "{{data-partition-id}}:reference-data--UnitOfMeasure:M:",
						"Stations": [
							{
								"MD": 0.0,
								"Inclination": 0.0,
								"AzimuthTN": 0.0,
								"Point": {
									"X": 400000,
									"Y": 6500000,
									"Z": 0
								}
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
								"FileSource": "gs://bucket/trajectories/1100.csv"
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
Coordinate System Resolution
    ↓
Wellbore DDMS (bulk storage)
    ↓
Storage Service (metadata)
    ↓
Indexer Service
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
         "FileSource": "srn:type:file/csv:trajectory-001:"
       }
     }
   }
   → Returns WorkflowID

3. CSV Parser Processing (Airflow DAG)
   a. Read CSV file from staging
   b. Parse header row (X, Y, TVD, MD)
   c. For each data row:
      - Extract X, Y, TVD, MD values
      - Validate coordinate ranges
      - Validate depth ranges
      - Group stations by WellboreID (if provided)
   d. Resolve coordinate reference system
   e. Transform to trajectory station format
   f. Create trajectory stations array

4. CRS Conversion (if needed)
   POST /convertTrajectory
   {
     "inputKind": "MD_X_Y_Z",
     "inputStations": [
       { "md": 0, "x": 606554, "y": 6080126, "z": -30 }
     ],
     "trajectoryCRS": "EPSG:32631",
     "referencePoint": { "x": 606554, "y": 6080126, "z": 0 },
     "unitXY": "...",
     "unitZ": "...",
     "method": "AzimuthalEquidistant"
   }
   → Returns stations with computed inclination, azimuth, deviations

5. Wellbore DDMS Storage
   POST /api/os-wellbore-ddms/wellbores/{wellboreId}/trajectories
   {
     "trajectoryCRS": "EPSG:32631",
     "unitXY": "...",
     "unitZ": "...",
     "stations": [ ... ]
   }
   → Returns trajectory ID

6. Storage Service Metadata Ingestion
   PUT /api/storage/v2/records
   [
     {
       "kind": "{{data-partition-id}}:wks:work-product-component--Trajectory:1.0.0",
       "data": {
         "WellboreID": "...",
         "TrajectoryID": "...",
         "TrajectoryCRS": "EPSG:32631",
         "StationCount": 100,
         "MinMD": 0,
         "MaxMD": 5000
       }
     }
   ]
   → Returns record IDs

7. Indexer Service
   - Indexes trajectory metadata
   - Extracts searchable fields (WellboreID, CRS, depth ranges)
   - Updates search index
```

**CSV Parser DAG Implementation:**

```python
# csv_ingestion_all_steps.py (Airflow DAG)
from airflow import DAG
from airflow.operators.python import PythonOperator

def parse_csv_trajectory(file_source, data_partition_id):
    """
    Parse CSV file and create trajectory records
    """
    # 1. Read CSV from staging
    csv_data = file_service.read(file_source)

    # 2. Parse CSV
    import csv
    reader = csv.DictReader(csv_data)

    # 3. Extract trajectory stations
    stations = []
    for row in reader:
        station = {
            "md": float(row['MD']),
            "point": {
                "x": float(row['X']),
                "y": float(row['Y']),
                "z": float(row['TVD'])
            }
        }
        stations.append(station)

    # 4. If only X/Y/Z/MD provided, compute inclination/azimuth via CRS service
    if not all('Inclination' in row and 'Azimuth' in row for row in stations):
        # Call CRS Conversion Service
        crs_response = crs_service.convert_trajectory(
            input_kind="MD_X_Y_Z",
            input_stations=stations,
            trajectory_crs="EPSG:32631",
            method="AzimuthalEquidistant"
        )
        stations = crs_response['stations']

    # 5. Store in Wellbore DDMS
    wellbore_id = extract_wellbore_id(file_source)
    trajectory_id = wellbore_ddms.store_trajectory(
        wellbore_id=wellbore_id,
        stations=stations,
        trajectory_crs="EPSG:32631"
    )

    # 6. Create metadata record
    storage_service.ingest_records([{
        "kind": f"{data_partition_id}:wks:work-product-component--Trajectory:1.0.0",
        "data": {
            "WellboreID": wellbore_id,
            "TrajectoryID": trajectory_id,
            "StationCount": len(stations),
            "MinMD": min(s['md'] for s in stations),
            "MaxMD": max(s['md'] for s in stations)
        }
    }])

    return {"status": "success", "trajectory_id": trajectory_id}
```

### Pathway 2: WITSML Parser Ingestion

**Architecture:**

```
WITSML File Upload
    ↓
File Service (staging)
    ↓
WITSML Parser DAG (Airflow)
    ↓
XML Parsing & Validation
    ↓
WITSML → OSDU Schema Mapping
    ↓
CRS Conversion (if needed)
    ↓
Wellbore DDMS Storage
    ↓
Storage Service (metadata)
```

**Call Stack:**

```
1. WITSML File Upload
   POST /v2/files/upload
   → Returns FileSource identifier

2. Trigger WITSML Parser DAG
   POST /v1/workflow/Osdu_ingest/workflowRun
   {
     "executionContext": {
       "Payload": {
         "AppKey": "witsml-parser",
         "data-partition-id": "{{data-partition-id}}",
         "FileSource": "srn:type:file/witsml:trajectory-001:"
       }
     }
   }
   → Returns WorkflowID

3. WITSML Parser Processing (Airflow DAG)
   a. Parse WITSML XML file
   b. Extract trajectory elements
   c. For each trajectoryStation:
      - Extract MD, inclination, azimuth
      - Extract location (latitude/longitude)
      - Map to OSDU trajectory station format
   d. Resolve wellbore reference
   e. Convert geographic coordinates to projected CRS

4. CRS Conversion
   POST /convertTrajectory
   {
     "inputKind": "MD_Incl_Azim",
     "inputStations": [
       {
         "md": 0,
         "inclination": 0,
         "azimuth": 0
       }
     ],
     "referencePoint": {
       "x": 400000,
       "y": 6500000,
       "z": 0
     },
     "trajectoryCRS": "EPSG:32631",
     "azimuthReference": "TN",
     "method": "AzimuthalEquidistant"
   }
   → Returns stations with computed X/Y/Z coordinates

5. Wellbore DDMS Storage
   POST /api/os-wellbore-ddms/wellbores/{wellboreId}/trajectories
   {
     "trajectoryCRS": "EPSG:32631",
     "stations": [ ... ]
   }
   → Returns trajectory ID

6. Storage Service Metadata Ingestion
   PUT /api/storage/v2/records
   [
     {
       "kind": "{{data-partition-id}}:wks:work-product-component--Trajectory:1.0.0",
       "data": { ... }
     }
   ]
   → Returns record IDs
```

**WITSML Parser DAG Implementation:**

```python
# witsml_parser_dag.py (Airflow DAG)
import xml.etree.ElementTree as ET

def parse_witsml_trajectory(file_source, data_partition_id):
    """
    Parse WITSML file and create trajectory records
    """
    # 1. Read WITSML XML from staging
    witsml_data = file_service.read(file_source)

    # 2. Parse XML
    root = ET.fromstring(witsml_data)
    ns = {'witsml': 'http://www.witsml.org/schemas/1series'}

    # 3. Extract trajectory stations
    stations = []
    for station_elem in root.findall('.//witsml:trajectoryStation', ns):
        station = {
            "md": float(station_elem.find('witsml:md', ns).text),
            "inclination": float(station_elem.find('witsml:inclination', ns).text),
            "azimuth": float(station_elem.find('witsml:azimuth', ns).text)
        }

        # Extract location if available
        location = station_elem.find('witsml:location', ns)
        if location is not None:
            station["wgs84Latitude"] = float(location.find('witsml:latitude', ns).text)
            station["wgs84Longitude"] = float(location.find('witsml:longitude', ns).text)

        stations.append(station)

    # 4. Get wellbore reference
    wellbore_uid = root.find('.//witsml:uidWellbore', ns).text
    wellbore_id = resolve_wellbore_id(wellbore_uid)

    # 5. Convert to projected CRS via CRS Conversion Service
    reference_point = get_wellbore_reference_point(wellbore_id)
    crs_response = crs_service.convert_trajectory(
        input_kind="MD_Incl_Azim",
        input_stations=stations,
        reference_point=reference_point,
        trajectory_crs="EPSG:32631",
        azimuth_reference="TN",
        method="AzimuthalEquidistant"
    )

    # 6. Store in Wellbore DDMS
    trajectory_id = wellbore_ddms.store_trajectory(
        wellbore_id=wellbore_id,
        stations=crs_response['stations'],
        trajectory_crs="EPSG:32631"
    )

    # 7. Create metadata record
    storage_service.ingest_records([{
        "kind": f"{data_partition_id}:wks:work-product-component--Trajectory:1.0.0",
        "data": {
            "WellboreID": wellbore_id,
            "TrajectoryID": trajectory_id,
            "SourceFormat": "WITSML",
            "StationCount": len(stations)
        }
    }])

    return {"status": "success", "trajectory_id": trajectory_id}
```

### Pathway 3: Manifest-Based Ingestion

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
CRS Conversion (if needed)
    ↓
Wellbore DDMS Storage
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
         "WorkProductComponents": [
           {
             "kind": "{{data-partition-id}}:wks:work-product-component--Trajectory:1.0.0",
             "data": {
               "WellboreID": "...",
               "Stations": [ ... ]
             }
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
   c. For each Trajectory component:
      - Validate WellboreID reference
      - Validate TrajectoryCRS
      - Validate station array structure
      - Validate units (unitXY, unitZ, unitDls)
   d. Resolve all reference data

4. CRS Conversion (if input format differs)
   POST /convertTrajectory
   {
     "inputKind": "MD_Incl_Azim",
     "inputStations": [ ... ],
     "trajectoryCRS": "EPSG:32631",
     "method": "AzimuthalEquidistant"
   }
   → Returns converted stations

5. Wellbore DDMS Storage
   POST /api/os-wellbore-ddms/wellbores/{wellboreId}/trajectories
   {
     "trajectoryCRS": "EPSG:32631",
     "stations": [ ... ]
   }
   → Returns trajectory ID

6. Storage Service Batch Ingestion
   PUT /api/storage/v2/records
   [
     WorkProduct record,
     Trajectory WorkProductComponent record,
     Dataset record (if file source provided)
   ]
   → Returns record IDs with surrogate keys resolved

7. Indexer Service
   - Indexes all ingested records
   - Creates searchable metadata
```

### Pathway 4: Direct Storage API Ingestion

**Architecture:**

```
Application Code
    ↓
CRS Conversion Service (if needed)
    ↓
Wellbore DDMS (bulk storage)
    ↓
Storage Service API (metadata)
    ↓
Indexer Service
```

**Call Stack:**

```
1. Prepare Trajectory Data
   const stations = [
     {
       md: 0,
       inclination: 0,
       azimuthTN: 0,
       point: { x: 400000, y: 6500000, z: 0 }
     },
     {
       md: 1000,
       inclination: 0,
       azimuthTN: 0,
       point: { x: 400000, y: 6500000, z: -1000 }
     }
   ];

2. Store in Wellbore DDMS
   POST /api/os-wellbore-ddms/wellbores/{wellboreId}/trajectories
   Authorization: Bearer <JWT>
   Content-Type: application/json

   {
     "trajectoryCRS": "EPSG:32631",
     "unitXY": "{\"symbol\":\"m\"}",
     "unitZ": "{\"symbol\":\"m\"}",
     "stations": stations
   }

   → Response:
   {
     "trajectoryId": "trajectory-001",
     "stationCount": 2
   }

3. Create Metadata Record
   PUT /api/storage/v2/records
   Authorization: Bearer <JWT>
   Content-Type: application/json
   Slb-Data-Partition-Id: {{data-partition-id}}

   [
     {
       "kind": "{{data-partition-id}}:wks:work-product-component--Trajectory:1.0.0",
       "acl": {
         "owners": ["data.default.owners@..."],
         "viewers": ["data.default.viewers@..."]
       },
       "legal": {
         "legaltags": ["{{data-partition-id}}-demo-legaltag"],
         "otherRelevantDataCountries": ["US"]
       },
       "data": {
         "WellboreID": "{{data-partition-id}}:master-data--Wellbore:1100:",
         "TrajectoryID": "trajectory-001",
         "TrajectoryCRS": "EPSG:32631",
         "StationCount": 2,
         "MinMD": 0,
         "MaxMD": 1000
       }
     }
   ]

   → Response:
   {
     "recordIds": [
       "{{data-partition-id}}:doc:4ff67ce36ae2452b8ddad3391f1fc08a"
     ],
     "recordIdVersions": [
       {
         "id": "{{data-partition-id}}:doc:4ff67ce36ae2452b8ddad3391f1fc08a",
         "version": 1582725123640845
       }
     ]
   }

4. Storage Service Processing
   a. Validate schema against Trajectory kind
   b. Validate ACL format and permissions
   c. Validate legal tags exist and user has permission
   d. Validate WellboreID reference exists
   e. Validate TrajectoryID exists in Wellbore DDMS
   f. Create record with version
   g. Store in storage backend

5. Indexer Service (async)
   - Receives record creation event
   - Extracts searchable fields
   - Updates search index
   - Makes record discoverable via Search Service
```

## CRS Conversion Service

### Coordinate Reference System Transformation

The CRS Conversion Service converts trajectory stations between different coordinate systems and input formats.

**Supported Input Formats:**

1. **MD_Incl_Azim**: Measured Depth, Inclination, Azimuth
2. **MD_X_Y_Z**: Measured Depth, X, Y, Z coordinates

**Conversion Process:**

```
Input Stations (MD, Inclination, Azimuth)
    ↓
Reference Point (where MD=0)
    ↓
Azimuthal Equidistant Projection
    ↓
Compute X/Y/Z Coordinates
    ↓
Compute Deviations (dxTN, dyTN, dz)
    ↓
Compute Dog Leg Severity (DLS)
    ↓
Output Stations (Complete)
```

**API Call Example:**

```http
POST /convertTrajectory
Content-Type: application/json
Authorization: Bearer <JWT>

{
  "azimuthReference": "TN",
  "interpolate": true,
  "referencePoint": {
    "x": 400000,
    "y": 6500000,
    "z": 0
  },
  "unitZ": "{\"scaleOffset\":{\"scale\":1.0,\"offset\":0.0},\"symbol\":\"m\",\"baseMeasurement\":{\"ancestry\":\"Length\",\"type\":\"UM\"},\"type\":\"USO\"}",
  "inputStations": [
    {
      "md": 0,
      "azimuth": 0,
      "inclination": 0
    },
    {
      "md": 1000,
      "azimuth": 0,
      "inclination": 0
    },
    {
      "md": 2000,
      "azimuth": 0,
      "inclination": 90
    }
  ],
  "trajectoryCRS": "EPSG:32631",
  "inputKind": "MD_Incl_Azim",
  "unitXY": "{\"scaleOffset\":{\"scale\":1.0,\"offset\":0.0},\"symbol\":\"m\",\"baseMeasurement\":{\"ancestry\":\"Length\",\"type\":\"UM\"},\"type\":\"USO\"}",
  "method": "AzimuthalEquidistant"
}
```

**Response:**

```json
{
	"trajectoryCRS": "EPSG:32631",
	"unitXY": "{\"symbol\":\"m\"}",
	"unitZ": "{\"symbol\":\"m\"}",
	"unitDls": "{\"symbol\":\"deg/30m\"}",
	"stations": [
		{
			"md": 0,
			"inclination": 0,
			"azimuthTN": 0,
			"azimuthGN": 1.4705504660493034,
			"dxTN": 0,
			"dyTN": 0,
			"point": {
				"x": 400000,
				"y": 6500000,
				"z": 0
			},
			"wgs84Longitude": 1.277806753183437,
			"wgs84Latitude": 58.62877104866187,
			"dls": 0,
			"original": true,
			"dz": 0
		},
		{
			"md": 1000,
			"inclination": 0,
			"azimuthTN": 0,
			"azimuthGN": 1.4705504660493034,
			"dxTN": 0,
			"dyTN": 0,
			"point": {
				"x": 400000,
				"y": 6500000,
				"z": -1000
			},
			"wgs84Longitude": 1.277806753183437,
			"wgs84Latitude": 58.62877104866187,
			"dls": 0,
			"original": true,
			"dz": 1000
		},
		{
			"md": 2000,
			"inclination": 90,
			"azimuthTN": 0,
			"azimuthGN": 1.4705504660493034,
			"dxTN": 0,
			"dyTN": 0,
			"point": {
				"x": 400000,
				"y": 6500000,
				"z": -2000
			},
			"wgs84Longitude": 1.277806753183437,
			"wgs84Latitude": 58.62877104866187,
			"dls": 0,
			"original": true,
			"dz": 2000
		}
	]
}
```

## Wellbore DDMS Architecture

### Bulk Data Storage

Wellbore DDMS provides optimized storage and access for trajectory bulk data:

**API Endpoints:**

```http
# Store trajectory
POST /api/os-wellbore-ddms/wellbores/{wellboreId}/trajectories
Content-Type: application/json

{
  "trajectoryCRS": "EPSG:32631",
  "unitXY": "...",
  "unitZ": "...",
  "stations": [ ... ]
}

# Retrieve trajectory
GET /api/os-wellbore-ddms/wellbores/{wellboreId}/trajectories/{trajectoryId}

# Query trajectories
GET /api/os-wellbore-ddms/wellbores/{wellboreId}/trajectories?minMD=0&maxMD=5000
```

**Storage Benefits:**

- **Optimized Access**: Efficient retrieval of large station arrays
- **Spatial Indexing**: Fast queries by depth range or spatial extent
- **Compression**: Efficient storage of repetitive trajectory data
- **Caching**: In-memory caching for frequently accessed trajectories

## Data Transformation & Validation

### Reference Data Resolution

**Coordinate Reference System Resolution:**

```
Input: "EPSG:32631" or "WGS_1984_UTM_Zone_31N"
    ↓
CRS Service Lookup
    ↓
Validate CRS exists and is supported
    ↓
Resolved CRS: Full WKT definition
```

**Unit Resolution:**

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

The CSV parser performs automatic column mapping:

```python
COLUMN_MAPPINGS = {
    "X": ["X", "Easting", "East", "E"],
    "Y": ["Y", "Northing", "North", "N"],
    "TVD": ["TVD", "True Vertical Depth", "Vertical Depth", "VD"],
    "MD": ["MD", "Measured Depth", "Depth"],
    "Inclination": ["Inclination", "Incl", "Inc"],
    "Azimuth": ["Azimuth", "Azi", "Az"],
    "DLS": ["DLS", "Dog Leg Severity", "Dogleg"]
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
   - MD (required)
   - Either (X, Y, TVD) OR (Inclination, Azimuth) required
4. Optional fields:
   - DLS (computed if not provided)
   - Inclination/Azimuth (computed via CRS service if X/Y/Z provided)
```

## Storage Architecture

### Metadata vs. Bulk Data Separation

**Metadata Storage (Storage Service):**

```json
{
	"id": "{{data-partition-id}}:doc:4ff67ce36ae2452b8ddad3391f1fc08a",
	"version": 1582725123640845,
	"kind": "{{data-partition-id}}:wks:work-product-component--Trajectory:1.0.0",
	"data": {
		"WellboreID": "{{data-partition-id}}:master-data--Wellbore:1100:",
		"TrajectoryID": "trajectory-001",
		"TrajectoryCRS": "EPSG:32631",
		"UnitXY": "{{data-partition-id}}:reference-data--UnitOfMeasure:M:",
		"UnitZ": "{{data-partition-id}}:reference-data--UnitOfMeasure:M:",
		"StationCount": 100,
		"MinMD": 0,
		"MaxMD": 5000,
		"SourceFormat": "CSV"
	}
}
```

**Bulk Data Storage (Wellbore DDMS):**

```
Wellbore DDMS Backend
    ↓
Trajectory Document:
    {
      "trajectoryId": "trajectory-001",
      "wellboreId": "wellbore-1100",
      "trajectoryCRS": "EPSG:32631",
      "unitXY": "...",
      "unitZ": "...",
      "stations": [
        {
          "md": 0,
          "inclination": 0,
          "azimuthTN": 0,
          "point": { "x": 400000, "y": 6500000, "z": 0 },
          ...
        },
        ...
      ]
    }
    ↓
Indexed for Spatial Queries:
    - Spatial index on point coordinates
    - Depth index on MD values
    - Wellbore index for fast lookups
```

### File Storage

**Source File Preservation:**

```
File Service (Object Store)
    ↓
Source File Storage:
    gs://osdu-storage/{{data-partition-id}}/files/{{file-id}}/trajectory.csv
    ↓
Dataset Record Reference:
    {
      "kind": "{{data-partition-id}}:wks:dataset--File.Generic:1.0.0",
      "data": {
        "DatasetProperties": {
          "FileSourceInfo": {
            "FileSource": "gs://osdu-storage/{{data-partition-id}}/files/{{file-id}}/trajectory.csv"
          }
        }
      }
    }
    ↓
Linked to Trajectory:
    Trajectory.data.Data.GroupTypeProperties.Files = [
      "srn:type:file/csv:trajectory-001:"
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
    - TrajectoryCRS → Filter by coordinate system
    - MinMD / MaxMD → Range queries
    - StationCount → Filter by station count
    - SourceFormat → Filter by source format
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
  "kind": "{{data-partition-id}}:wks:work-product-component--Trajectory:1.0.0",
  "query": "data.WellboreID:\"{{data-partition-id}}:master-data--Wellbore:1100:\""
}
```

**Search by Depth Range:**

```http
POST /api/search/v2/query
{
  "kind": "{{data-partition-id}}:wks:work-product-component--Trajectory:1.0.0",
  "query": "data.MinMD:[0 TO 1000] AND data.MaxMD:[4000 TO 5000]"
}
```

**Search by Coordinate System:**

```http
POST /api/search/v2/query
{
  "kind": "{{data-partition-id}}:wks:work-product-component--Trajectory:1.0.0",
  "query": "data.TrajectoryCRS:\"EPSG:32631\""
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

2. **Invalid Coordinate Reference System:**

   ```
   Error: TrajectoryCRS "INVALID_CRS" not found or not supported
   Resolution: Use valid EPSG code or CRS identifier
   ```

3. **Missing Required Station Fields:**

   ```
   Error: Required field "MD" missing in station at index 5
   Resolution: Ensure all stations have MD value
   ```

4. **Invalid Coordinate Ranges:**

   ```
   Error: X coordinate 20000000 exceeds maximum range
   Resolution: Validate coordinate ranges before ingestion
   ```

5. **CRS Conversion Failure:**
   ```
   Error: Failed to convert trajectory: Invalid reference point
   Resolution: Ensure reference point is valid and in correct CRS
   ```

### Retry Logic

**CSV Parser Retry Strategy:**

```python
def ingest_with_retry(trajectory_data, max_retries=3):
    for attempt in range(max_retries):
        try:
            # Store in Wellbore DDMS
            trajectory_id = wellbore_ddms.store_trajectory(trajectory_data)

            # Create metadata record
            response = storage_service.ingest_records([{
                "kind": "...",
                "data": { "TrajectoryID": trajectory_id, ... }
            }])
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

- **Small batches (10-50 stations)**: Lower memory usage, more API calls
- **Medium batches (100-500 stations)**: Balanced performance
- **Large batches (1000+ stations)**: Higher memory usage, fewer API calls, risk of timeout

**Recommended:** 200-500 stations per batch for trajectory ingestion

### Parallel Processing

**CSV Parser Parallelization:**

```python
# Group trajectories by wellbore
wellbore_groups = group_by_wellbore(csv_rows)

# Process each wellbore in parallel
with ThreadPoolExecutor(max_workers=10) as executor:
    futures = [
        executor.submit(process_wellbore_trajectory, wellbore_id, stations)
        for wellbore_id, stations in wellbore_groups.items()
    ]

    results = [future.result() for future in as_completed(futures)]
```

### Spatial Indexing

**Wellbore DDMS Spatial Index:**

```
Trajectory Stations
    ↓
Spatial Index (R-tree or similar)
    - Index by point coordinates (X, Y, Z)
    - Index by depth range (MinMD, MaxMD)
    - Index by wellbore ID
    ↓
Fast Spatial Queries:
    - Find trajectories within bounding box
    - Find trajectories intersecting depth range
    - Find trajectories near point
```

## Summary

OSDU provides **comprehensive, multi-format ingestion** for well trajectory data through specialized services:

1. **Wellbore DDMS**: Optimized bulk storage and access for trajectory stations
2. **CSV Parser**: Automated ingestion from CSV files with coordinate mapping
3. **WITSML Parser**: Industry-standard WITSML trajectory parsing
4. **Manifest-based**: Structured JSON manifests for complex workflows
5. **Direct API**: Programmatic record creation for custom applications
6. **CRS Conversion Service**: Coordinate system transformations and computations

**Key Design Decisions:**

- **Metadata/Bulk Separation**: Metadata in Storage Service, bulk data in Wellbore DDMS
- **Station-Based Model**: Trajectories as arrays of survey stations
- **CRS Support**: Multiple coordinate reference systems with conversion capabilities
- **Source File Preservation**: Original files stored alongside structured records
- **Search Integration**: Automatic indexing for discovery and querying
- **Access Control**: ACLs and legal tags enforced at ingestion time

**Call Stack Highlights:**

- **CSV Parser**: File upload → DAG trigger → Parse & validate → CRS conversion → DDMS storage → Metadata ingest → Index
- **WITSML Parser**: File upload → DAG trigger → XML parse → WITSML mapping → CRS conversion → DDMS storage → Metadata ingest → Index
- **Manifest**: JSON creation → Workflow API → Parse & validate → CRS conversion → DDMS storage → Batch ingest → Index
- **Direct API**: Data preparation → CRS conversion (if needed) → DDMS storage → Storage API → Index

This architecture ensures **data consistency**, **spatial accuracy**, **traceability**, and **governance** while supporting multiple ingestion formats and coordinate systems.
