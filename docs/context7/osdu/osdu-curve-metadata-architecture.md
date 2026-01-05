# OSDU Curve Mnemonics, Metadata & Properties Architecture

This document explains how OSDU handles curve mnemonics, curve metadata, and curve properties when ingesting and storing LAS files/curve data, including the call stack and architecture for related workflows.

## Overview

OSDU takes a **two-tier approach** to curve data:

1. **High-level metadata** (WellLog record): Log-level information stored in structured records
2. **Curve-level data** (LAS file): Mnemonics, units, and curve properties remain in the original LAS file
3. **Unit Service**: Provides standardized measurement and unit definitions for curve interpretation

**Key Principles:**

- **Source File Preservation**: Curve data stays in original LAS/DLIS format
- **Metadata Extraction**: High-level log metadata extracted to WellLog records
- **Unit Service Integration**: Standardized measurement/unit lookups via Unit Service
- **Wellbore DDMS**: Optimized access to bulk curve data
- **Measurement Dictionary**: OSDD/Energistics standards for curve identification

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                    OSDU Data Platform                        │
│                                                             │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  LAS File Ingestion                                  │   │
│  │  - Parse LAS file                                    │   │
│  │  - Extract curve mnemonics from ~Curve section      │   │
│  │  - Extract units from curve definitions             │   │
│  │  - Extract depth range, step                        │   │
│  └────────────────────┬────────────────────────────────┘   │
│                       │                                      │
│         ┌─────────────┼─────────────┬──────────────┐        │
│         │             │             │              │        │
│  ┌──────▼──────┐ ┌───▼────┐ ┌─────▼─────┐ ┌──────▼──────┐ │
│  │ WellLog     │ │ LAS    │ │ Unit      │ │ Wellbore    │ │
│  │ Record      │ │ File   │ │ Service   │ │ DDMS        │ │
│  │ (Metadata)  │ │ (Data) │ │ (Lookup)  │ │ (Access)    │ │
│  └─────────────┘ └────────┘ └───────────┘ └─────────────┘ │
│         │             │             │              │        │
│         └─────────────┼─────────────┴──────────────┘        │
│                       │                                      │
│  ┌────────────────────▼────────────────────────────────┐   │
│  │  Storage Service                                     │   │
│  │  - WellLog records (log-level metadata)             │   │
│  │  - Dataset records (file references)                │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                             │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  File Storage (Object Store)                        │   │
│  │  - LAS files with curve data intact                 │   │
│  └─────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
```

## Curve Data Organization

### 1. LAS File Structure (Source Data)

LAS files contain curve information in the `~Curve Information` section:

```
~Curve Information
  DEPT .M              : DEPTH
  GR   .API            : GAMMA RAY
  RHOB .G/CC           : BULK DENSITY
  NPHI .V/V            : NEUTRON POROSITY
  DT   .US/F           : SONIC TRANSIT TIME
  RT   .OHMM           : RESISTIVITY
```

**Curve Definition Format**:

- **Mnemonic**: Short code (e.g., `GR`, `RHOB`, `NPHI`)
- **Unit**: Measurement unit (e.g., `API`, `G/CC`, `V/V`)
- **Description**: Human-readable name (e.g., `GAMMA RAY`, `BULK DENSITY`)

### 2. WellLog Record (High-Level Metadata)

The WellLog record stores log-level metadata but **not individual curve details**:

```json
{
	"kind": "osdu:wks:work-product-component--WellLog:1.0.0",
	"data": {
		"WellboreId": "osdu:master-data--Wellbore:3511023252:",
		"WellLogTypeID": "osdu:reference-data--LogType:Raw:",
		"BottomMeasuredDepth": 12660,
		"TopMeasuredDepth": 48.0,
		"LoggingService": "SLIM CEMENT MAP TOOL",
		"ActivityType": "Wireline",
		"ServiceCompanyId": "osdu:master-data--Organisation:Schlumberger:",
		"Datasets": ["osdu:dataset--File.Generic:abc123"]
	}
}
```

**Note**: Curve mnemonics, units, and properties are **not stored in the WellLog record**. They remain in the LAS file.

### 3. Dataset Record (File Reference)

The Dataset record references the LAS file:

```json
{
	"kind": "osdu:wks:dataset--File.Generic:1.0.0",
	"data": {
		"DatasetProperties": {
			"FileSourceInfo": {
				"FileSource": "gs://bucket/path/to/well_log.las",
				"Name": "well_log_001.las"
			}
		},
		"TotalSize": "13245217",
		"Source": "Data Source"
	}
}
```

## Unit Service: Measurement & Unit Lookup

OSDU provides a **Unit Service** for standardized measurement and unit definitions, based on:

- **OSDD** (Oilfield Services Data Dictionary) - Schlumberger's master database
- **Energistics Units of Measure** - Industry standard definitions

### Measurement Structure

A **Measurement** represents a physical quantity (e.g., Gamma Ray, Bulk Density):

```json
{
	"ancestry": "L.UM",
	"code": "GR",
	"name": "Gamma Ray",
	"namespace": "Energistics_UoM",
	"dimensionCode": "L",
	"unitQuantityCode": "UM",
	"baseMeasurement": true,
	"source": "Energistics"
}
```

**Measurement Properties**:

- `ancestry`: Hierarchical classification (e.g., `L.UM` for Length/Unitless Measurement)
- `code`: Measurement code/mnemonic
- `name`: Full name
- `namespace`: Source namespace (Energistics_UoM, RP66, ECL, etc.)
- `dimensionCode`: Physical dimension (L=Length, M=Mass, etc.)
- `unitQuantityCode`: Unit quantity classification
- `baseMeasurement`: Whether this is a base measurement

### Unit Structure

A **Unit** represents a unit of measurement (e.g., API, G/CC):

```json
{
	"displaySymbol": "API",
	"name": "American Petroleum Institute unit",
	"description": "Unit for gamma ray measurements",
	"namespace": "Energistics_UoM",
	"source": "Energistics",
	"essence": {
		"symbol": "API",
		"baseMeasurement_reference": "{\"ancestry\":\"L.UM\",\"type\":\"UM\"}",
		"persistableReference": "{\"symbol\":\"API\",\"baseMeasurement\":{\"ancestry\":\"L.UM\",\"type\":\"UM\"},\"type\":\"UAD\"}"
	}
}
```

**Unit Properties**:

- `displaySymbol`: Unit symbol (e.g., `API`, `G/CC`, `V/V`)
- `name`: Full unit name
- `namespace`: Source namespace
- `essence`: Core definition with conversion parameters
- `baseMeasurement_reference`: Links to base measurement

### Unit Conversion

Units can have conversion parameters:

**Scale-Offset Conversion**:

```json
{
	"essence": {
		"scaleOffset": {
			"scale": 1.0,
			"offset": 0.0
		}
	}
}
```

**ABCD Conversion** (Energistics):

```json
{
	"essence": {
		"abcd": {
			"a": 0.0,
			"b": 1.0,
			"c": 1.0,
			"d": 0.0
		}
	}
}
```

Formula: `y = (A + B*x) / (C + D*x)`

## Unit Service API

### Search Measurements

```http
POST /v2/measurement/search
Content-Type: application/json
Authorization: Bearer <JWT>

{
  "keywords": [
    {
      "code": "Name",
      "value": "Gamma Ray"
    }
  ]
}
```

**Response**:

```json
{
	"measurements": [
		{
			"ancestry": "L.UM",
			"code": "GR",
			"name": "Gamma Ray",
			"namespace": "Energistics_UoM",
			"dimensionCode": "L",
			"unitQuantityCode": "UM"
		}
	],
	"totalCount": 1
}
```

### Search by Mnemonic Code

```http
POST /v2/measurement/search
Content-Type: application/json

{
  "keywords": [
    {
      "code": "Code",
      "value": "GR"
    },
    {
      "code": "Namespace",
      "value": "Energistics_UoM"
    }
  ]
}
```

### Get Measurement by Ancestry

```http
GET /v2/measurement/{ancestry}
Authorization: Bearer <JWT>
```

**Example**: `GET /v2/measurement/L.UM`

### Search Units

```http
POST /v2/unit/search
Content-Type: application/json

{
  "keywords": [
    {
      "code": "Name",
      "value": "API"
    },
    {
      "code": "Namespace",
      "value": "Energistics_UoM"
    }
  ]
}
```

### Get Unit Assignment

```http
GET /v2/unit-assignment/{measurement-ancestry}/{unit-namespace}
Authorization: Bearer <JWT>
```

Returns the standard unit assignment for a measurement in a given namespace.

## Curve Metadata Extraction Workflow

### Call Stack: LAS File Ingestion with Curve Metadata

```
┌─────────────────────────────────────────────────────────────┐
│  1. LAS File Upload                                          │
│     POST /v2/files/upload                                    │
│     - Upload LAS file to staging                             │
│     - Returns FileSource identifier                          │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  2. LAS File Parsing                                         │
│     - Parse ~Curve Information section                      │
│     - Extract curve mnemonics                               │
│     - Extract units                                         │
│     - Extract descriptions                                  │
│     - Extract depth range (STRT, STOP, STEP)                │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  3. Curve Metadata Mapping                                   │
│     For each curve in LAS file:                             │
│                                                             │
│     Curve: GR .API : GAMMA RAY                              │
│       → Mnemonic: "GR"                                      │
│       → Unit: "API"                                         │
│       → Description: "GAMMA RAY"                            │
│                                                             │
│     - Lookup measurement in Unit Service                    │
│     - Lookup unit in Unit Service                           │
│     - Map to standardized definitions                       │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  4. Unit Service Lookup                                      │
│     POST /v2/measurement/search                              │
│     {                                                        │
│       "keywords": [                                          │
│         {"code": "Code", "value": "GR"},                    │
│         {"code": "Namespace", "value": "Energistics_UoM"}   │
│       ]                                                      │
│     }                                                        │
│                                                             │
│     Response: Measurement definition                        │
│     - ancestry: "L.UM"                                      │
│     - dimensionCode: "L"                                    │
│     - unitQuantityCode: "UM"                                │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  5. Build Manifest                                           │
│     - Create WellLog record with log-level metadata         │
│     - Create Dataset record referencing LAS file            │
│     - Curve details remain in LAS file                      │
│     - Optionally store curve summary in metadata            │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  6. Ingest Records                                           │
│     PUT /api/storage/v2/records                              │
│     - Create WellLog record                                 │
│     - Create Dataset record                                 │
│     - Link WellLog to Dataset                               │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  7. File Service Metadata Ingestion                          │
│     POST /v2/files/metadata                                  │
│     - Move LAS file from staging to persistent storage      │
│     - Create Dataset record                                 │
│     - LAS file with curve data preserved                    │
└─────────────────────────────────────────────────────────────┘
```

## Curve Data Access Workflows

### Workflow 1: Query WellLog and Access Curve Data

```
┌─────────────────────────────────────────────────────────────┐
│  1. Search for WellLog                                       │
│     GET /api/search/v2/query                                 │
│     {                                                        │
│       "kind": "osdu:wks:work-product-component--WellLog:1.0.0",│
│       "query": "WellboreId:osdu:master-data--Wellbore:123"  │
│     }                                                        │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  2. Get WellLog Record                                       │
│     GET /api/storage/v2/records/{welllog-id}                │
│                                                             │
│     Response:                                                │
│     {                                                        │
│       "data": {                                              │
│         "WellboreId": "...",                                 │
│         "BottomMeasuredDepth": 12660,                        │
│         "Datasets": ["osdu:dataset--File.Generic:abc123"]   │
│       }                                                      │
│     }                                                        │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  3. Get Dataset Record                                       │
│     GET /api/storage/v2/records/{dataset-id}                │
│                                                             │
│     Response:                                                │
│     {                                                        │
│       "data": {                                              │
│         "DatasetProperties": {                               │
│           "FileSourceInfo": {                                │
│             "FileSource": "gs://bucket/path/file.las"       │
│           }                                                  │
│         }                                                    │
│       }                                                      │
│     }                                                        │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  4. Get File Download URL                                    │
│     GET /v2/files/{dataset-id}/downloadURL                  │
│                                                             │
│     Response:                                                │
│     {                                                        │
│       "SignedURL": "https://storage.googleapis.com/..."     │
│     }                                                        │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  5. Download and Parse LAS File                              │
│     - Download LAS file using SignedURL                     │
│     - Parse ~Curve Information section                      │
│     - Extract curve mnemonics, units, descriptions          │
│     - Access curve data values                              │
└─────────────────────────────────────────────────────────────┘
```

### Workflow 2: Wellbore DDMS - Optimized Curve Access

The **Wellbore DDMS** provides optimized access to bulk curve data:

```
┌─────────────────────────────────────────────────────────────┐
│  1. Get WellLog via DDMS                                     │
│     GET /api/wellbore/v1/welllogs/{welllog-id}              │
│                                                             │
│     Response includes:                                       │
│     - WellLog metadata                                       │
│     - Dataset references                                     │
│     - Optimized metadata                                     │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  2. Get Curve Data via DDMS                                  │
│     GET /api/wellbore/v1/welllogs/{welllog-id}/data         │
│     ?channels=GR,RHOB,NPHI                                   │
│     &startDepth=1000                                         │
│     &endDepth=2000                                           │
│                                                             │
│     DDMS:                                                    │
│     - Parses LAS file                                        │
│     - Extracts requested channels                            │
│     - Filters by depth range                                 │
│     - Returns structured data                                │
└─────────────────────────────────────────────────────────────┘
```

**DDMS Benefits**:

- **Type-safe access**: Structured API for curve data
- **Optimized queries**: Efficient depth range filtering
- **Channel selection**: Request specific curves
- **Bulk data handling**: Optimized for large datasets

### Workflow 3: Curve Metadata Lookup via Unit Service

```
┌─────────────────────────────────────────────────────────────┐
│  1. Parse LAS File Curve Section                            │
│     - Extract mnemonic: "GR"                                │
│     - Extract unit: "API"                                   │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  2. Lookup Measurement                                       │
│     POST /v2/measurement/search                              │
│     {                                                        │
│       "keywords": [                                          │
│         {"code": "Code", "value": "GR"}                     │
│       ]                                                      │
│     }                                                        │
│                                                             │
│     Response:                                                │
│     {                                                        │
│       "measurements": [{                                     │
│         "ancestry": "L.UM",                                  │
│         "code": "GR",                                        │
│         "name": "Gamma Ray",                                 │
│         "dimensionCode": "L",                                │
│         "unitQuantityCode": "UM"                             │
│       }]                                                     │
│     }                                                        │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  3. Lookup Unit                                              │
│     POST /v2/unit/search                                     │
│     {                                                        │
│       "keywords": [                                          │
│         {"code": "Name", "value": "API"},                   │
│         {"code": "Namespace", "value": "Energistics_UoM"}   │
│       ]                                                      │
│     }                                                        │
│                                                             │
│     Response:                                                │
│     {                                                        │
│       "units": [{                                            │
│         "displaySymbol": "API",                              │
│         "name": "American Petroleum Institute unit",         │
│         "namespace": "Energistics_UoM",                      │
│         "essence": {                                         │
│           "symbol": "API",                                   │
│           "baseMeasurement_reference": "..."                 │
│         }                                                    │
│       }]                                                     │
│     }                                                        │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  4. Validate Unit Assignment                                 │
│     GET /v2/unit-assignment/L.UM/Energistics_UoM            │
│                                                             │
│     Response:                                                │
│     {                                                        │
│       "measurement": { /* GR measurement */ },               │
│       "unit": { /* API unit */ },                            │
│       "lastModified": "20160322"                             │
│     }                                                        │
│                                                             │
│     Validates that API is a valid unit for Gamma Ray        │
└─────────────────────────────────────────────────────────────┘
```

## Curve Property Mapping

### Mnemonic to Measurement Mapping

OSDU uses the **Unit Service** to map LAS curve mnemonics to standardized measurements:

**LAS Curve** → **Unit Service Lookup** → **Standardized Measurement**

```
LAS File:
  GR .API : GAMMA RAY
    ↓
Unit Service Search:
  Code: "GR"
  Namespace: "Energistics_UoM"
    ↓
Measurement:
  {
    "ancestry": "L.UM",
    "code": "GR",
    "name": "Gamma Ray",
    "dimensionCode": "L",
    "unitQuantityCode": "UM"
  }
```

### Unit Validation

Units from LAS files are validated against Unit Service:

```
LAS Unit: "API"
    ↓
Unit Service Search:
  Name: "API"
  Namespace: "Energistics_UoM"
    ↓
Unit:
  {
    "displaySymbol": "API",
    "name": "American Petroleum Institute unit",
    "namespace": "Energistics_UoM",
    "essence": {
      "symbol": "API",
      "baseMeasurement_reference": "L.UM"
    }
  }
```

### Namespace Handling

OSDU supports multiple unit/measurement namespaces:

| Namespace           | Description                       | Use Case           |
| ------------------- | --------------------------------- | ------------------ |
| **Energistics_UoM** | Clean Energistics standard        | Primary standard   |
| **RP66**            | Legacy RP66 standard              | Legacy data        |
| **ECL**             | Eclipse/Petrel extensions         | Schlumberger tools |
| **LIS**             | Log Information Standard          | Very old data      |
| **POSC**            | Petrotechnical Open Software Corp | Legacy standard    |
| **SLB**             | Schlumberger-specific             | SLB internal       |

**Namespace Resolution**:

1. Try Energistics_UoM first (preferred)
2. Fall back to vendor-specific namespaces if needed
3. Support multiple namespaces for interoperability

## Enhanced WellLog Schema (Hypothetical Extension)

While the standard WellLog schema doesn't include curve-level details, an extended schema could include:

```json
{
	"kind": "osdu:wks:work-product-component--WellLog:1.0.0",
	"data": {
		"WellboreId": "...",
		"BottomMeasuredDepth": 12660,
		"Datasets": ["..."],

		// Extended curve metadata (if supported)
		"CurveMetadata": [
			{
				"Mnemonic": "GR",
				"Unit": "API",
				"Description": "GAMMA RAY",
				"MeasurementReference": "Energistics_UoM:GR",
				"UnitReference": "Energistics_UoM:API",
				"DataType": "float",
				"NullValue": -999.25
			},
			{
				"Mnemonic": "RHOB",
				"Unit": "G/CC",
				"Description": "BULK DENSITY",
				"MeasurementReference": "Energistics_UoM:RHOB",
				"UnitReference": "Energistics_UoM:G/CC",
				"DataType": "float",
				"NullValue": -999.25
			}
		]
	}
}
```

**Note**: This is a hypothetical extension. In practice, OSDU keeps curve details in the LAS file.

## Airflow Workflow: Curve Metadata Extraction

### DAG Structure

```python
from airflow.models.dag import DAG
from airflow.operators.python import PythonOperator

def extract_curve_metadata_task(**context):
    """Extract curve metadata from LAS file"""
    las_file_path = context['dag_run'].conf['las_file_path']

    # Parse LAS file
    las_data = parse_las_file(las_file_path)

    # Extract curve information
    curves = []
    for curve_def in las_data.curves:
        mnemonic = curve_def.mnemonic
        unit = curve_def.unit
        description = curve_def.description

        # Lookup in Unit Service
        measurement = lookup_measurement(mnemonic)
        unit_def = lookup_unit(unit)

        curves.append({
            "mnemonic": mnemonic,
            "unit": unit,
            "description": description,
            "measurement_reference": measurement.get("persistableReference"),
            "unit_reference": unit_def.get("persistableReference")
        })

    return {
        "curves": curves,
        "depth_range": {
            "start": las_data.start_depth,
            "stop": las_data.stop_depth,
            "step": las_data.step
        }
    }

def create_welllog_record_task(**context):
    """Create WellLog record with metadata"""
    ti = context['ti']
    curve_metadata = ti.xcom_pull(task_ids='extract_curve_metadata')

    # Build WellLog record
    welllog_record = {
        "kind": "osdu:wks:work-product-component--WellLog:1.0.0",
        "data": {
            "WellboreId": context['dag_run'].conf['wellbore_id'],
            "BottomMeasuredDepth": curve_metadata['depth_range']['stop'],
            "TopMeasuredDepth": curve_metadata['depth_range']['start'],
            "Datasets": [context['dag_run'].conf['dataset_id']]
            # Note: Curve details not stored in WellLog record
        }
    }

    # Ingest record
    ingest_record(welllog_record)

    return welllog_record

with DAG(
    dag_id="las_curve_metadata_extraction",
    schedule=None,
    start_date=pendulum.datetime(2023, 1, 1, tz="UTC"),
) as dag:
    extract_metadata = PythonOperator(
        task_id="extract_curve_metadata",
        python_callable=extract_curve_metadata_task,
    )

    create_welllog = PythonOperator(
        task_id="create_welllog_record",
        python_callable=create_welllog_record_task,
    )

    extract_metadata >> create_welllog
```

## Curve Data Query Patterns

### Pattern 1: Get All Curves for a WellLog

```
1. Get WellLog record
   GET /api/storage/v2/records/{welllog-id}

2. Get Dataset record
   GET /api/storage/v2/records/{dataset-id}

3. Download LAS file
   GET /v2/files/{dataset-id}/downloadURL
   → Download file

4. Parse LAS file
   - Read ~Curve Information section
   - Extract all curve definitions
   - Return list of curves with mnemonics, units, descriptions
```

### Pattern 2: Validate Curve Units

```
1. Parse LAS file curve section
   - Extract mnemonic and unit for each curve

2. For each curve:
   a. Lookup measurement
      POST /v2/measurement/search
      {"keywords": [{"code": "Code", "value": mnemonic}]}

   b. Lookup unit
      POST /v2/unit/search
      {"keywords": [{"code": "Name", "value": unit}]}

   c. Validate unit assignment
      GET /v2/unit-assignment/{measurement-ancestry}/{unit-namespace}

   d. Report validation results
      - Valid: Unit matches measurement
      - Warning: Unit not standard for measurement
      - Error: Unit incompatible with measurement
```

### Pattern 3: Convert Curve Units

```
1. Get curve data with source unit
   - Download LAS file
   - Extract curve values

2. Lookup source unit
   POST /v2/unit/search
   {"keywords": [{"code": "Name", "value": "API"}]}

3. Lookup target unit
   POST /v2/unit/search
   {"keywords": [{"code": "Name", "value": "GAPI"}]}

4. Get conversion parameters
   - From unit.essence.scaleOffset or unit.essence.abcd
   - Calculate conversion formula

5. Apply conversion
   - Convert all curve values
   - Create new dataset with converted values
```

## Wellbore DDMS: Optimized Curve Access

### Channel Data API

The Wellbore DDMS provides optimized access to curve (channel) data:

```http
GET /api/wellbore/v1/welllogs/{welllog-id}/channels
Authorization: Bearer <JWT>
```

**Response**:

```json
{
	"channels": [
		{
			"mnemonic": "GR",
			"unit": "API",
			"description": "GAMMA RAY",
			"dataType": "float",
			"nullValue": -999.25,
			"measurementReference": "Energistics_UoM:GR",
			"unitReference": "Energistics_UoM:API"
		},
		{
			"mnemonic": "RHOB",
			"unit": "G/CC",
			"description": "BULK DENSITY",
			"dataType": "float",
			"nullValue": -999.25,
			"measurementReference": "Energistics_UoM:RHOB",
			"unitReference": "Energistics_UoM:G/CC"
		}
	]
}
```

### Channel Data Retrieval

```http
GET /api/wellbore/v1/welllogs/{welllog-id}/channels/{channel-mnemonic}/data
?startDepth=1000
&endDepth=2000
&samplingRate=1
Authorization: Bearer <JWT>
```

**Response**:

```json
{
	"channel": "GR",
	"unit": "API",
	"data": [
		{ "depth": 1000.0, "value": 45.2 },
		{ "depth": 1000.1524, "value": 46.1 },
		{ "depth": 1000.3048, "value": 47.3 }
	],
	"startDepth": 1000.0,
	"endDepth": 2000.0,
	"samplingRate": 0.1524
}
```

### Multi-Channel Data Retrieval

```http
POST /api/wellbore/v1/welllogs/{welllog-id}/channels/data
Content-Type: application/json

{
  "channels": ["GR", "RHOB", "NPHI"],
  "startDepth": 1000,
  "endDepth": 2000,
  "samplingRate": 1
}
```

**Response**:

```json
{
	"channels": ["GR", "RHOB", "NPHI"],
	"data": [
		{
			"depth": 1000.0,
			"GR": 45.2,
			"RHOB": 2.65,
			"NPHI": 0.15
		},
		{
			"depth": 1000.1524,
			"GR": 46.1,
			"RHOB": 2.66,
			"NPHI": 0.16
		}
	],
	"startDepth": 1000.0,
	"endDepth": 2000.0
}
```

## Complete Workflow: LAS Ingestion with Curve Metadata

### End-to-End Call Stack

```
┌─────────────────────────────────────────────────────────────┐
│  1. LAS File Upload                                          │
│     Client → File Service                                    │
│     POST /v2/files/upload                                    │
│     - Multipart form with LAS file                          │
│     - Returns FileSource: "gs://staging/abc123.las"         │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  2. LAS File Parsing (Airflow Task)                         │
│     Airflow DAG → LAS Parser                                 │
│                                                             │
│     parse_las_file(file_path):                              │
│       - Read ~Version Information                           │
│       - Read ~Well Information                              │
│         * STRT, STOP, STEP                                  │
│         * WELL, UWI                                         │
│       - Read ~Curve Information                             │
│         * For each curve:                                   │
│           mnemonic, unit, description                       │
│       - Read ~ASCII (data values)                           │
│                                                             │
│     Returns:                                                │
│     {                                                        │
│       "well_info": {...},                                   │
│       "curves": [                                           │
│         {"mnemonic": "GR", "unit": "API", ...},            │
│         {"mnemonic": "RHOB", "unit": "G/CC", ...}          │
│       ],                                                     │
│       "depth_range": {"start": 48, "stop": 12660, ...}     │
│     }                                                        │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  3. Curve Metadata Enrichment (Airflow Task)                │
│     Airflow DAG → Unit Service                               │
│                                                             │
│     For each curve:                                         │
│       a. Lookup measurement                                 │
│          POST /v2/measurement/search                        │
│          {"keywords": [{"code": "Code", "value": "GR"}]}   │
│                                                             │
│       b. Lookup unit                                        │
│          POST /v2/unit/search                               │
│          {"keywords": [{"code": "Name", "value": "API"}]}  │
│                                                             │
│       c. Validate assignment                                │
│          GET /v2/unit-assignment/L.UM/Energistics_UoM       │
│                                                             │
│     Returns enriched curve metadata:                        │
│     {                                                        │
│       "curves": [                                           │
│         {                                                    │
│           "mnemonic": "GR",                                 │
│           "unit": "API",                                    │
│           "description": "GAMMA RAY",                       │
│           "measurement": {                                  │
│             "ancestry": "L.UM",                             │
│             "code": "GR",                                   │
│             "name": "Gamma Ray"                             │
│           },                                                │
│           "unit_def": {                                     │
│             "displaySymbol": "API",                         │
│             "namespace": "Energistics_UoM"                  │
│           },                                                │
│           "validated": true                                 │
│         }                                                    │
│       ]                                                      │
│     }                                                        │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  4. Build Manifest                                           │
│     Airflow DAG → Manifest Builder                           │
│                                                             │
│     build_manifest(las_data, curve_metadata):               │
│       - Create Work Product                                 │
│       - Create Work Product Component (WellLog)             │
│         * Use depth_range from LAS                          │
│         * Link to wellbore (from UWI lookup)                │
│         * Note: Curve details NOT in WellLog record         │
│       - Create Dataset reference                            │
│         * FileSource from step 1                            │
│                                                             │
│     Returns manifest JSON                                    │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  5. File Service Metadata Ingestion                          │
│     Airflow DAG → File Service API                           │
│     POST /v2/files/metadata                                  │
│                                                             │
│     {                                                        │
│       "data": {                                              │
│         "DatasetProperties": {                               │
│           "FileSourceInfo": {                                │
│             "FileSource": "gs://staging/abc123.las",        │
│             "Name": "well_log_001.las"                      │
│           }                                                  │
│         }                                                    │
│       },                                                     │
│       "kind": "osdu:wks:dataset--File.Generic:1.0.0",      │
│       ...                                                    │
│     }                                                        │
│                                                             │
│     Response:                                                │
│     {"id": "osdu:dataset--File.Generic:xyz789"}             │
│                                                             │
│     File Service:                                            │
│     - Moves file from staging to persistent storage         │
│     - Creates Dataset record                                │
│     - LAS file with curve data preserved                    │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  6. Storage Service Record Ingestion                         │
│     Airflow DAG → Storage Service API                        │
│     PUT /api/storage/v2/records                              │
│                                                             │
│     [                                                        │
│       {                                                      │
│         "kind": "osdu:wks:work-product--WorkProduct:1.0.0",│
│         "data": {                                            │
│           "Name": "Well Log WP",                            │
│           "Components": ["surrogate-key: wpc1"]             │
│         }                                                    │
│       },                                                     │
│       {                                                      │
│         "kind": "osdu:wks:work-product-component--WellLog:1.0.0",│
│         "data": {                                            │
│           "WellboreId": "osdu:master-data--Wellbore:123",   │
│           "BottomMeasuredDepth": 12660,                      │
│           "TopMeasuredDepth": 48.0,                          │
│           "Datasets": ["osdu:dataset--File.Generic:xyz789"] │
│           // Note: No curve-level details here              │
│         }                                                    │
│       }                                                      │
│     ]                                                        │
│                                                             │
│     Storage Service:                                         │
│     - Validates schema                                      │
│     - Checks referential integrity                          │
│     - Stores records                                        │
│     - Returns record IDs                                    │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  7. Search Service Indexing                                  │
│     Storage Service → Search Service                         │
│                                                             │
│     - Indexes WellLog metadata fields                       │
│     - Indexes Dataset metadata                              │
│     - Curve mnemonics NOT indexed (in LAS file)             │
│     - Full-text search on descriptions                      │
└─────────────────────────────────────────────────────────────┘
```

## Curve Data Retrieval Workflow

### Call Stack: Get Curve Data for Visualization

```
┌─────────────────────────────────────────────────────────────┐
│  1. Search for WellLog                                       │
│     Client → Search Service                                  │
│     POST /api/search/v2/query                                │
│     {                                                        │
│       "kind": "osdu:wks:work-product-component--WellLog:1.0.0",│
│       "query": "WellboreId:osdu:master-data--Wellbore:123"  │
│     }                                                        │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  2. Get WellLog Record                                       │
│     Client → Storage Service                                 │
│     GET /api/storage/v2/records/{welllog-id}                │
│                                                             │
│     Response includes Dataset ID                             │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  3. Option A: Direct File Access                             │
│     Client → File Service                                    │
│     GET /v2/files/{dataset-id}/downloadURL                  │
│                                                             │
│     Response: Signed URL                                    │
│     → Download LAS file                                      │
│     → Parse curve data locally                               │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  3. Option B: Wellbore DDMS Access (Recommended)            │
│     Client → Wellbore DDMS                                   │
│     GET /api/wellbore/v1/welllogs/{welllog-id}/channels     │
│                                                             │
│     Response: List of channels with metadata                │
│                                                             │
│     GET /api/wellbore/v1/welllogs/{welllog-id}/channels/data│
│     ?channels=GR,RHOB,NPHI                                   │
│     &startDepth=1000                                         │
│     &endDepth=2000                                           │
│                                                             │
│     Response: Structured curve data                         │
│     - Pre-parsed from LAS file                              │
│     - Filtered by depth range                               │
│     - Selected channels only                                │
│     - Optimized for visualization                           │
└─────────────────────────────────────────────────────────────┘
```

## Curve Property Validation Workflow

### Call Stack: Validate Curve Units

```
┌─────────────────────────────────────────────────────────────┐
│  1. Get WellLog and Dataset                                  │
│     (Same as retrieval workflow)                             │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  2. Download LAS File                                        │
│     GET /v2/files/{dataset-id}/downloadURL                  │
│     → Download file                                          │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  3. Parse Curve Section                                      │
│     parse_las_curves(las_file):                             │
│       - Extract ~Curve Information                          │
│       - For each curve:                                     │
│         * mnemonic (e.g., "GR")                             │
│         * unit (e.g., "API")                                │
│         * description (e.g., "GAMMA RAY")                   │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  4. Validate Each Curve                                      │
│     For each curve:                                         │
│                                                             │
│     a. Lookup Measurement                                   │
│        POST /v2/measurement/search                          │
│        {                                                     │
│          "keywords": [                                       │
│            {"code": "Code", "value": mnemonic},            │
│            {"code": "Namespace", "value": "Energistics_UoM"}│
│          ]                                                   │
│        }                                                     │
│                                                             │
│        Response:                                            │
│        {                                                     │
│          "measurements": [{                                  │
│            "ancestry": "L.UM",                               │
│            "code": "GR",                                     │
│            "name": "Gamma Ray"                               │
│          }]                                                  │
│        }                                                     │
│                                                             │
│     b. Lookup Unit                                          │
│        POST /v2/unit/search                                 │
│        {                                                     │
│          "keywords": [                                       │
│            {"code": "Name", "value": unit},                │
│            {"code": "Namespace", "value": "Energistics_UoM"}│
│          ]                                                   │
│        }                                                     │
│                                                             │
│        Response:                                            │
│        {                                                     │
│          "units": [{                                         │
│            "displaySymbol": "API",                           │
│            "name": "American Petroleum Institute unit",      │
│            "essence": {                                      │
│              "baseMeasurement_reference": "L.UM"             │
│            }                                                 │
│          }]                                                  │
│        }                                                     │
│                                                             │
│     c. Validate Assignment                                  │
│        GET /v2/unit-assignment/{measurement-ancestry}/{namespace}│
│                                                             │
│        Checks if unit is valid for measurement              │
│                                                             │
│     d. Report Results                                       │
│        {                                                     │
│          "mnemonic": "GR",                                   │
│          "unit": "API",                                      │
│          "valid": true,                                      │
│          "measurement": "Energistics_UoM:GR",                │
│          "unit_reference": "Energistics_UoM:API"             │
│        }                                                     │
└─────────────────────────────────────────────────────────────┘
```

## Key Design Patterns

### 1. Source File Preservation

**Pattern**: Keep curve data in original LAS file format

- **Benefit**: Preserves data lineage and original structure
- **Trade-off**: Requires file parsing for curve access
- **Mitigation**: Wellbore DDMS provides optimized access

### 2. Metadata Extraction

**Pattern**: Extract high-level metadata, preserve detailed data in file

- **WellLog record**: Log-level metadata (depth range, wellbore, etc.)
- **LAS file**: Curve-level data (mnemonics, units, values)
- **Unit Service**: Standardized measurement/unit definitions

### 3. Unit Service Lookup

**Pattern**: Use Unit Service for curve interpretation

- **Mnemonic → Measurement**: Lookup standardized measurement
- **Unit → Unit Definition**: Lookup unit properties
- **Validation**: Verify unit-measurement compatibility

### 4. Wellbore DDMS Optimization

**Pattern**: DDMS provides optimized bulk data access

- **Pre-parsed**: LAS files parsed and cached
- **Filtered**: Depth range and channel selection
- **Structured**: Type-safe API responses

## Comparison with DataForge Approach

| Aspect               | OSDU                                    | DataForge                            |
| -------------------- | --------------------------------------- | ------------------------------------ |
| **Curve Storage**    | In LAS file (original format)           | In Parquet (converted)               |
| **Curve Metadata**   | In LAS file + Unit Service lookup       | In PostgreSQL (curve_metadata table) |
| **Mnemonic Mapping** | Unit Service measurement lookup         | curve_metadata table with mnemonic   |
| **Unit Validation**  | Unit Service unit lookup                | unit_conversions crate               |
| **Curve Access**     | Download LAS or use DDMS                | Direct SQL queries on Parquet        |
| **Schema**           | WellLog (log-level) + LAS (curve-level) | Well + Curve tables (normalized)     |

## Key Takeaways

1. **Two-Tier Storage**: Log-level metadata in WellLog records, curve data in LAS files
2. **Unit Service Integration**: Standardized measurement/unit lookups via Unit Service
3. **Source File Preservation**: LAS files stored in original format for lineage
4. **Wellbore DDMS**: Optimized access to bulk curve data
5. **Measurement Dictionary**: OSDD/Energistics standards for curve identification
6. **Namespace Support**: Multiple unit/measurement namespaces (Energistics_UoM, RP66, etc.)
7. **Validation Workflow**: Unit Service validates unit-measurement compatibility
8. **Access Patterns**: Direct file download or optimized DDMS API access

## References

- [OSDU Unit Service Documentation](https://community.opengroup.org/groups/osdu/platform/-/wikis/uploads/UnitService-f41927ad-44ec-4503-ac2b-c5f82a0e563e)
- [OSDU WellLog Schema](https://gitlab.opengroup.org/osdu/subcommittees/data-def/work-products/schema/-/blob/master/Generated/work-product-component/WellLog.1.0.0.json)
- [OSDU Wellbore DDMS](https://community.opengroup.org/groups/osdu/platform/-/wikis/Core-Services-Overview)
- [Energistics Units of Measure](https://www.energistics.org/unit-of-measure-standard/)
- [OSDD - Oilfield Services Data Dictionary](https://community.opengroup.org/groups/osdu/platform/-/wikis/uploads/UnitService-f41927ad-44ec-4503-ac2b-c5f82a0e563e)
