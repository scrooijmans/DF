# OSDU Surface Ingestion Analysis: Architecture, Schema, and Ingestion Flow

## Executive Summary

This document provides a comprehensive analysis of how surfaces (seismic horizons, geological surfaces, property grids) are designed, stored, processed, and ingested in the OSDU (Open Subsurface Data Universe) platform. It covers the complete architecture, schema definitions, and detailed ingestion workflows with call stacks for all supported formats and methods.

---

## 1. Surface Architecture Overview

### 1.1 Conceptual Overview

**Surfaces** in OSDU represent 3D geological or geophysical surfaces that can be:

- **Seismic Horizons**: Interpreted geological boundaries from seismic data
- **Geological Surfaces**: Point-based or grid-based geological surfaces (horizons, faults, unconformities)
- **Property Grids**: 3D property grids (porosity, permeability, saturation, etc.)

Surfaces are represented as **Work Product Components (WPC)** in the OSDU data model, following the Work Product pattern.

### 1.2 OSDU Data Model Hierarchy

Surfaces fit into the OSDU data model hierarchy as follows:

```
┌─────────────────────────────────────────┐
│ Master Data                             │
│ - InterpretationProject                 │
│ - SeismicGeometryContext                │
│ - Grid                                  │
└──────────────┬──────────────────────────┘
               │
               ▼
┌─────────────────────────────────────────┐
│ Work Product (WP)                       │
│ - Groups related WPCs                   │
│ - Example: Seismic Interpretation WP    │
└──────────────┬──────────────────────────┘
               │
               ▼
┌─────────────────────────────────────────┐
│ Work Product Component (WPC)            │
│ - SeismicHorizon                        │
│ - Surface                               │
│ - GridProperty                          │
└──────────────┬──────────────────────────┘
               │
               ▼
┌─────────────────────────────────────────┐
│ Dataset                                 │
│ - References source files                │
│   (SEGY, ZGY, VDS, CSV, GeoTIFF, RESQML)│
│ - Links to WPC                          │
└─────────────────────────────────────────┘
```

### 1.3 Key Architectural Components

1. **Work Product (WP)**: Container that groups one or more surface WPCs
2. **Work Product Component (WPC)**: The actual surface record (SeismicHorizon, Surface, or GridProperty)
3. **Dataset**: References to source files (SEGY, ZGY, VDS, CSV, GeoTIFF, RESQML)
4. **Seismic DDMS**: Specialized service for optimized bulk storage of seismic surfaces (ZGY, VDS formats)
5. **Reservoir DDMS**: Specialized service for reservoir property grids and structural models
6. **Storage Service**: Metadata storage for all surface types
7. **File Service**: Generic file storage for source files

### 1.4 Design Principles

- **Schema First**: Surfaces require schema definition before ingestion
- **Work Product Pattern**: Follows OSDU Work Product/Work Product Component pattern
- **Metadata/Bulk Separation**: Metadata in Storage Service, bulk data in DDMS or File Service
- **Multi-Format Support**: SEGY, ZGY, VDS, CSV, GeoTIFF, RESQML ingestion
- **Format Optimization**: SEGY → ZGY/VDS conversion for cloud-optimized storage
- **Coordinate System Support**: Multiple CRS support with spatial indexing
- **Grid-Based Storage**: Efficient storage for regular and irregular grids

---

## 2. Surface Data Types

### 2.1 Seismic Horizons

Seismic horizons represent interpreted geological boundaries from seismic data:

**Kind Identifier Pattern**:

```
{authority}:wks:work-product-component--SeismicHorizon:{version}
```

**Example**: `osdu:wks:work-product-component--SeismicHorizon:1.0.0`

**Key Fields**:

- `Name`: Horizon name
- `InterpretationProjectID`: Reference to interpretation project
- `SeismicGeometryContextID`: Reference to seismic geometry context
- `HorizonTypeID`: Reference to horizon type (Top, Base, etc.)
- `SurfaceRepresentation`: Format, file source, grid dimensions, spatial extent, CRS

### 2.2 Geological Surfaces

Point-based or grid-based geological surfaces:

**Kind Identifier Pattern**:

```
{authority}:wks:work-product-component--Surface:{version}
```

**Example**: `osdu:wks:work-product-component--Surface:1.0.0`

**Key Fields**:

- `Name`: Surface name
- `SurfaceTypeID`: Reference to surface type (Horizon, Fault, Unconformity)
- `Representation`: Format, file source, point count, spatial extent, CRS, units

### 2.3 Property Grids

3D property grids (porosity, permeability, saturation, etc.):

**Kind Identifier Pattern**:

```
{authority}:wks:work-product-component--GridProperty:{version}
```

**Example**: `osdu:wks:work-product-component--GridProperty:1.0.0`

**Key Fields**:

- `Name`: Property grid name
- `PropertyTypeID`: Reference to property type (Porosity, Permeability, etc.)
- `GridID`: Reference to structural grid
- `Representation`: Format, file source, grid dimensions, data range, unit

---

## 3. Schema Definitions

### 3.1 Understanding "Schema First" Approach

OSDU requires **schema definition before ingestion** for all surface types. This ensures:

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

### 3.2 SeismicHorizon Schema

```json
{
  "kind": "osdu:wks:work-product-component--SeismicHorizon:1.0.0",
  "schema": [
    {
      "path": "data.ResourceTypeID",
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
      "path": "data.InterpretationProjectID",
      "kind": "core:String:1.0.0"
    },
    {
      "path": "data.SeismicGeometryContextID",
      "kind": "core:String:1.0.0"
    },
    {
      "path": "data.HorizonTypeID",
      "kind": "core:String:1.0.0"
    },
    {
      "path": "data.SurfaceRepresentation.Format",
      "kind": "core:String:1.0.0"
    },
    {
      "path": "data.SurfaceRepresentation.FileSource",
      "kind": "core:String:1.0.0"
    },
    {
      "path": "data.SurfaceRepresentation.GridDimensions.InlineCount",
      "kind": "core:Integer:1.0.0"
    },
    {
      "path": "data.SurfaceRepresentation.GridDimensions.CrosslineCount",
      "kind": "core:Integer:1.0.0"
    },
    {
      "path": "data.SurfaceRepresentation.GridDimensions.SampleCount",
      "kind": "core:Integer:1.0.0"
    },
    {
      "path": "data.SurfaceRepresentation.SpatialExtent.MinX",
      "kind": "core:Double:1.0.0"
    },
    {
      "path": "data.SurfaceRepresentation.SpatialExtent.MaxX",
      "kind": "core:Double:1.0.0"
    },
    {
      "path": "data.SurfaceRepresentation.SpatialExtent.MinY",
      "kind": "core:Double:1.0.0"
    },
    {
      "path": "data.SurfaceRepresentation.SpatialExtent.MaxY",
      "kind": "core:Double:1.0.0"
    },
    {
      "path": "data.SurfaceRepresentation.SpatialExtent.MinZ",
      "kind": "core:Double:1.0.0"
    },
    {
      "path": "data.SurfaceRepresentation.SpatialExtent.MaxZ",
      "kind": "core:Double:1.0.0"
    },
    {
      "path": "data.SurfaceRepresentation.CoordinateReferenceSystem",
      "kind": "core:String:1.0.0"
    }
  ]
}
```

### 3.3 Surface Schema

```json
{
  "kind": "osdu:wks:work-product-component--Surface:1.0.0",
  "schema": [
    {
      "path": "data.Name",
      "kind": "core:String:1.0.0"
    },
    {
      "path": "data.SurfaceTypeID",
      "kind": "core:String:1.0.0"
    },
    {
      "path": "data.Representation.Format",
      "kind": "core:String:1.0.0"
    },
    {
      "path": "data.Representation.FileSource",
      "kind": "core:String:1.0.0"
    },
    {
      "path": "data.Representation.PointCount",
      "kind": "core:Integer:1.0.0"
    },
    {
      "path": "data.Representation.SpatialExtent.MinX",
      "kind": "core:Double:1.0.0"
    },
    {
      "path": "data.Representation.SpatialExtent.MaxX",
      "kind": "core:Double:1.0.0"
    },
    {
      "path": "data.Representation.SpatialExtent.MinY",
      "kind": "core:Double:1.0.0"
    },
    {
      "path": "data.Representation.SpatialExtent.MaxY",
      "kind": "core:Double:1.0.0"
    },
    {
      "path": "data.Representation.SpatialExtent.MinZ",
      "kind": "core:Double:1.0.0"
    },
    {
      "path": "data.Representation.SpatialExtent.MaxZ",
      "kind": "core:Double:1.0.0"
    },
    {
      "path": "data.Representation.CoordinateReferenceSystem",
      "kind": "core:String:1.0.0"
    },
    {
      "path": "data.Representation.Units.XY",
      "kind": "core:String:1.0.0"
    },
    {
      "path": "data.Representation.Units.Z",
      "kind": "core:String:1.0.0"
    }
  ]
}
```

### 3.4 GridProperty Schema

```json
{
  "kind": "osdu:wks:work-product-component--GridProperty:1.0.0",
  "schema": [
    {
      "path": "data.Name",
      "kind": "core:String:1.0.0"
    },
    {
      "path": "data.PropertyTypeID",
      "kind": "core:String:1.0.0"
    },
    {
      "path": "data.GridID",
      "kind": "core:String:1.0.0"
    },
    {
      "path": "data.Representation.Format",
      "kind": "core:String:1.0.0"
    },
    {
      "path": "data.Representation.FileSource",
      "kind": "core:String:1.0.0"
    },
    {
      "path": "data.Representation.GridDimensions.I",
      "kind": "core:Integer:1.0.0"
    },
    {
      "path": "data.Representation.GridDimensions.J",
      "kind": "core:Integer:1.0.0"
    },
    {
      "path": "data.Representation.GridDimensions.K",
      "kind": "core:Integer:1.0.0"
    },
    {
      "path": "data.Representation.DataRange.Min",
      "kind": "core:Double:1.0.0"
    },
    {
      "path": "data.Representation.DataRange.Max",
      "kind": "core:Double:1.0.0"
    },
    {
      "path": "data.Representation.Unit",
      "kind": "core:String:1.0.0"
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
  "kind": "osdu:wks:work-product-component--Surface:1.0.0",
  "schema": [
    {
      "path": "data.Name",
      "kind": "core:String:1.0.0"
    },
    {
      "path": "data.SurfaceTypeID",
      "kind": "core:String:1.0.0"
    }
  ]
}
```

**Get Schema**:

```http
GET /api/storage/v2/schemas/osdu:wks:work-product-component--Surface:1.0.0
Authorization: Bearer <JWT>
Slb-Data-Partition-Id: osdu
```

---

## 4. Supported Data Formats

### 4.1 SEGY Format

**SEGY (SEG Y)** is the industry standard for seismic data:

**Structure**:

- **Text Header**: 3200-byte EBCDIC text header
- **Binary Header**: 400-byte binary header with survey metadata
- **Trace Headers**: 240-byte header per trace
- **Trace Data**: Amplitude samples (typically 32-bit floats)

**Key Metadata**:

- Survey geometry (inline/crossline spacing)
- Coordinate reference system
- Sample rate and count
- Data format (IEEE float, integer, etc.)

**Storage Strategy**: SEGY files are typically converted to ZGY or VDS format for optimized cloud storage, with original SEGY preserved in File Service.

### 4.2 ZGY Format

**ZGY (ZGY Seismic Data Format)** is an optimized format for cloud storage:

**Advantages**:

- **Chunked Storage**: Data organized in chunks for efficient access
- **Compression**: Built-in compression support
- **Metadata**: Rich metadata embedded in format
- **Cloud-Optimized**: Designed for object storage (S3, GCS, Azure Blob)

**Conversion Process**:

```
SEGY File
    ↓
Parse SEGY Headers
    ↓
Extract Survey Geometry
    ↓
Chunk Data by Inline/Crossline
    ↓
Compress Chunks
    ↓
Generate ZGY Metadata
    ↓
Write ZGY File
```

### 4.3 VDS Format

**VDS (Virtual Data Store)** is another optimized format for seismic data:

**Advantages**:

- **Virtualization**: Logical view of data without physical reorganization
- **Efficient Access**: Optimized for random access patterns
- **Metadata**: Comprehensive metadata support
- **Cloud-Native**: Designed for distributed storage

### 4.4 CSV Format

CSV files for point-based surfaces:

**Structure**:

```csv
X,Y,Z
605882.732209,6073657.908672,1061.42
605907.722462,6073658.606713,1061.31
605932.712715,6073659.304754,1061.14
605957.702968,6073660.002795,1060.83
```

**Column Mapping**:

- **X** → X-coordinate (easting)
- **Y** → Y-coordinate (northing)
- **Z** → Z-coordinate (depth/elevation)
- **SurfaceName** → Optional surface identifier

**Storage Strategy**: CSV files are preserved in File Service, with metadata extracted and stored in Storage Service.

### 4.5 GeoTIFF Format

GeoTIFF files for raster surface data:

**Structure**:

- **TIFF Header**: Standard TIFF metadata
- **GeoTIFF Tags**: Coordinate reference system, geotransform
- **Image Data**: Gridded surface values (single or multi-band)

**Key Tags**:

- `ModelTiepointTag`: Georeferencing tiepoints
- `ModelPixelScaleTag`: Pixel scale in X, Y, Z
- `GeoKeyDirectoryTag`: Coordinate system information

**Storage Strategy**: GeoTIFF files are preserved in File Service, with grid structure extracted and stored in metadata.

### 4.6 RESQML Format

**RESQML (REServoir Modeling Markup Language)** for reservoir surfaces:

**Structure**:

- **XML-based**: Industry standard XML format
- **Surface Representations**: Multiple representation types
- **Grid Definitions**: Structured and unstructured grids
- **Property Arrays**: Property data linked to grids

**Storage Strategy**: RESQML files are preserved in File Service, with surface and grid information extracted and stored as WPC records. Reservoir DDMS may be used for optimized access.

---

## 5. Ingestion Methods

OSDU supports multiple ingestion pathways for surfaces:

1. **SEGY to ZGY Conversion** (Airflow DAG)
2. **SEGY to VDS Conversion** (Airflow DAG)
3. **CSV Surface Ingestion** (Airflow DAG)
4. **GeoTIFF Surface Ingestion** (File Service + Processing)
5. **Manifest-based Ingestion** (Workflow Service)
6. **Direct Storage API Ingestion** (Programmatic)

---

## 6. Ingestion Flow: Complete Walkthrough

### 6.1 Pathway 1: SEGY to ZGY Conversion

**Architecture**:

```
SEGY File Upload
    ↓
File Service (staging)
    ↓
SEGY to ZGY Conversion DAG (Airflow)
    ↓
SEGY Parsing & Validation
    ↓
ZGY Conversion
    ↓
Seismic DDMS Storage
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

Step 1.1: Upload SEGY File
Client → File Service API
POST /v2/files/upload
Content-Type: multipart/form-data
Authorization: Bearer <JWT>
Slb-Data-Partition-Id: osdu

Request Body (multipart/form-data):
  - file: survey-001.segy (binary)

Response (200 OK):
{
  "FileID": "4ff67ce36ae2452b8ddad3391f1fc08a",
  "FileSource": "srn:type:file/segy:survey-001:",
  "Location": "gs://osdu-storage/osdu/files/4ff67ce36ae2452b8ddad3391f1fc08a/survey-001.segy"
}

┌─────────────────────────────────────────────────────────────┐
│ Phase 2: Trigger Conversion Workflow                        │
└─────────────────────────────────────────────────────────────┘

Step 2.1: Trigger SEGY to ZGY Conversion DAG
Client → Workflow Service API
POST /v1/workflow/Osdu_ingest/workflowRun
Content-Type: application/json
Authorization: Bearer <JWT>
Slb-Data-Partition-Id: osdu

Request Body:
{
  "executionContext": {
    "Payload": {
      "AppKey": "segy-to-zgy-conversion",
      "data-partition-id": "osdu",
      "FileSource": "srn:type:file/segy:survey-001:"
    }
  }
}

Response (200 OK):
{
  "workflowId": "workflow-001",
  "status": "RUNNING"
}

┌─────────────────────────────────────────────────────────────┐
│ Phase 3: SEGY to ZGY Conversion Processing (Airflow DAG)   │
└─────────────────────────────────────────────────────────────┘

Step 3.1: Read SEGY File from Staging
Airflow DAG → File Service
GET /v2/files/{fileId}/download
Authorization: Bearer <JWT>

Response: SEGY file binary data

Step 3.2: Parse SEGY Headers
Airflow DAG (Python):
  - Parse text header (3200 bytes EBCDIC)
  - Parse binary header (400 bytes)
  - Extract survey geometry:
    * Inline count
    * Crossline count
    * Sample count
    * Sample rate
  - Extract coordinate system
  - Extract data format

Step 3.3: Parse Trace Headers
Airflow DAG (Python):
  - For each trace:
    * Read 240-byte trace header
    * Extract inline/crossline numbers
    * Extract X/Y coordinates
    * Extract sample rate and count

Step 3.4: Read Trace Data
Airflow DAG (Python):
  - For each trace:
    * Read amplitude samples
    * Determine data format (IEEE float, integer)
    * Convert to standard format

Step 3.5: Organize Data by Inline/Crossline
Airflow DAG (Python):
  - Group traces by inline/crossline
  - Create spatial organization
  - Validate geometry consistency

Step 3.6: Chunk Data for ZGY Format
Airflow DAG (Python):
  - Group traces by spatial proximity
  - Create chunks for efficient access
  - Calculate chunk boundaries

Step 3.7: Compress Chunks (Optional)
Airflow DAG (Python):
  - Apply compression algorithm
  - Calculate compression ratios

Step 3.8: Generate ZGY Metadata
Airflow DAG (Python):
  - Survey geometry
  - Coordinate reference system
  - Data statistics (min, max, mean)
  - Chunk index

Step 3.9: Write ZGY File to Object Store
Airflow DAG → File Service
POST /v2/files/upload
Content-Type: multipart/form-data

Request Body (multipart/form-data):
  - file: survey-001.zgy (binary)

Response (200 OK):
{
  "FileID": "5aa78df47bf3563c9eead4492g2gd09b",
  "FileSource": "srn:type:file/zgy:survey-001:",
  "Location": "gs://osdu-storage/osdu/files/5aa78df47bf3563c9eead4492g2gd09b/survey-001.zgy"
}

┌─────────────────────────────────────────────────────────────┐
│ Phase 4: Seismic DDMS Storage                               │
└─────────────────────────────────────────────────────────────┘

Step 4.1: Store Volume in Seismic DDMS
Airflow DAG → Seismic DDMS API
POST /seistore-svc/api/v3/surveys/{surveyId}/volumes
Content-Type: application/json
Authorization: Bearer <JWT>

Request Body:
{
  "volumeId": "volume-001",
  "format": "ZGY",
  "fileSource": "gs://osdu-storage/osdu/files/5aa78df47bf3563c9eead4492g2gd09b/survey-001.zgy",
  "geometry": {
    "inlineCount": 1000,
    "crosslineCount": 500,
    "sampleCount": 2000,
    "sampleRate": 0.004
  },
  "coordinateReferenceSystem": "EPSG:32631"
}

Response (200 OK):
{
  "volumeId": "volume-001",
  "status": "STORED"
}

┌─────────────────────────────────────────────────────────────┐
│ Phase 5: Storage Service Metadata Ingestion                │
└─────────────────────────────────────────────────────────────┘

Step 5.1: Create SeismicVolume WPC Record
Airflow DAG → Storage Service API
PUT /api/storage/v2/records
Content-Type: application/json
Authorization: Bearer <JWT>
Slb-Data-Partition-Id: osdu

Request Body:
[
  {
    "kind": "osdu:wks:work-product-component--SeismicVolume:1.0.0",
    "acl": {
      "viewers": ["data.default.viewers@osdu"],
      "owners": ["data.default.owners@osdu"]
    },
    "legal": {
      "legaltags": ["osdu-legaltag-001"],
      "otherRelevantDataCountries": ["US"]
    },
    "data": {
      "ResourceTypeID": "srn:type:work-product-component/SeismicVolume:",
      "SurveyID": "survey-001",
      "VolumeID": "volume-001",
      "Format": "ZGY",
      "SourceFormat": "SEGY",
      "Geometry": {
        "InlineCount": 1000,
        "CrosslineCount": 500,
        "SampleCount": 2000,
        "SampleRate": 0.004
      },
      "CoordinateReferenceSystem": "EPSG:32631",
      "FileSource": "srn:type:file/zgy:survey-001:"
    }
  }
]

Response (200 OK):
[
  {
    "id": "osdu:doc:6bb79ef58cf4564d0ffbe4503h3he10c",
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
    * SurveyID
    * VolumeID
    * Format
    * Geometry (inline/crossline ranges)
    * CoordinateReferenceSystem
  - Updates search index
  - Enables spatial queries
```

**SEGY to ZGY Conversion DAG Implementation**:

```python
# segy_to_zgy_conversion_dag.py (Airflow DAG)
from airflow import DAG
from airflow.operators.python import PythonOperator
from datetime import datetime

def convert_segy_to_zgy(file_source, data_partition_id):
    """
    Convert SEGY file to ZGY format
    """
    # 1. Read SEGY file from staging
    segy_data = file_service.read(file_source)

    # 2. Parse SEGY headers
    segy_parser = SegyParser()
    headers = segy_parser.parse_headers(segy_data)

    # Extract metadata
    inline_count = headers['inline_count']
    crossline_count = headers['crossline_count']
    sample_count = headers['sample_count']
    sample_rate = headers['sample_rate']
    crs = headers['coordinate_reference_system']

    # 3. Parse trace data
    traces = segy_parser.parse_traces(segy_data)

    # 4. Organize by inline/crossline
    organized_data = organize_traces_by_geometry(traces, headers)

    # 5. Convert to ZGY format
    zgy_converter = ZgyConverter()
    zgy_file = zgy_converter.convert(
        data=organized_data,
        geometry={
            'inline_count': inline_count,
            'crossline_count': crossline_count,
            'sample_count': sample_count,
            'sample_rate': sample_rate
        },
        crs=crs,
        compression=True
    )

    # 6. Upload ZGY file to object store
    zgy_file_source = file_service.upload(zgy_file)

    # 7. Store in Seismic DDMS
    survey_id = extract_survey_id(file_source)
    volume_id = seismic_ddms.store_volume(
        survey_id=survey_id,
        format="ZGY",
        file_source=zgy_file_source,
        geometry={
            'inline_count': inline_count,
            'crossline_count': crossline_count,
            'sample_count': sample_count,
            'sample_rate': sample_rate
        },
        crs=crs
    )

    # 8. Create metadata record
    storage_service.ingest_records([{
        "kind": f"{data_partition_id}:wks:work-product-component--SeismicVolume:1.0.0",
        "data": {
            "SurveyID": survey_id,
            "VolumeID": volume_id,
            "Format": "ZGY",
            "SourceFormat": "SEGY",
            "Geometry": {
                "InlineCount": inline_count,
                "CrosslineCount": crossline_count,
                "SampleCount": sample_count,
                "SampleRate": sample_rate
            },
            "CoordinateReferenceSystem": crs,
            "FileSource": zgy_file_source
        }
    }])

    return {"status": "success", "volume_id": volume_id}

with DAG(
    dag_id='segy_to_zgy_conversion',
    start_date=datetime(2023, 1, 1),
    schedule_interval=None,
    catchup=False,
    tags=['osdu', 'seismic', 'conversion', 'segy', 'zgy'],
) as dag:
    convert_task = PythonOperator(
        task_id='convert_segy_to_zgy',
        python_callable=convert_segy_to_zgy,
        op_kwargs={
            'file_source': '{{ dag_run.conf.file_source }}',
            'data_partition_id': '{{ dag_run.conf.data_partition_id }}'
        }
    )
```

### 6.2 Pathway 2: SEGY to VDS Conversion

**Architecture**: Similar to SEGY to ZGY, but converts to VDS format instead.

**Call Stack** (similar to Pathway 1, with VDS-specific steps):

```
1. SEGY File Upload
   POST /v2/files/upload
   → Returns FileSource identifier

2. Trigger SEGY to VDS Conversion DAG
   POST /v1/workflow/Osdu_ingest/workflowRun
   {
     "executionContext": {
       "Payload": {
         "AppKey": "segy-to-vds-conversion",
         "data-partition-id": "{{data-partition-id}}",
         "FileSource": "srn:type:file/segy:survey-001:"
       }
     }
   }
   → Returns WorkflowID

3. SEGY to VDS Conversion Processing
   a. Read SEGY file from staging
   b. Parse SEGY headers (same as ZGY conversion)
   c. Parse trace data
   d. Create VDS structure:
      - Virtual data organization
      - Logical view of data
      - Metadata generation
   e. Write VDS file to object store

4. Seismic DDMS Storage
   POST /seistore-svc/api/v3/surveys/{surveyId}/volumes
   {
     "volumeId": "volume-001",
     "format": "VDS",
     "fileSource": "gs://bucket/surveys/survey-001.vds",
     ...
   }
   → Returns volume ID

5. Storage Service Metadata Ingestion
   PUT /api/storage/v2/records
   [ ... ]
   → Returns record IDs
```

### 6.3 Pathway 3: CSV Surface Ingestion

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
Grid Structure Generation
    ↓
Storage Service (metadata + data)
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
  - file: f3-truncation.csv (binary)

Response (200 OK):
{
  "FileID": "7cc80fg69dg5675e1ggcf5614i4if21d",
  "FileSource": "srn:type:file/csv:surface-001:",
  "Location": "gs://osdu-storage/osdu/files/7cc80fg69dg5675e1ggcf5614i4if21d/f3-truncation.csv"
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
      "FileSource": "srn:type:file/csv:surface-001:"
    }
  }
}

Response (200 OK):
{
  "workflowId": "workflow-002",
  "status": "RUNNING"
}

┌─────────────────────────────────────────────────────────────┐
│ Phase 3: CSV Parser Processing (Airflow DAG)                │
└─────────────────────────────────────────────────────────────┘

Step 3.1: Read CSV File from Staging
Airflow DAG → File Service
GET /v2/files/{fileId}/download
Authorization: Bearer <JWT>

Response: CSV file content

Step 3.2: Parse CSV Header Row
Airflow DAG (Python):
  - Read first row
  - Validate required columns (X, Y, Z)
  - Detect optional columns (SurfaceName)

Step 3.3: Parse CSV Data Rows
Airflow DAG (Python):
  - For each data row:
    * Extract X, Y, Z coordinates
    * Validate coordinate ranges
    * Validate data types (numeric)
    * Store in points array

Step 3.4: Generate Surface Metadata
Airflow DAG (Python):
  - Calculate spatial extent:
    * MinX, MaxX
    * MinY, MaxY
    * MinZ, MaxZ
  - Count points
  - Determine coordinate reference system (from metadata or default)

Step 3.5: Optionally Generate Grid Structure
Airflow DAG (Python):
  - Detect if points form regular grid
  - If regular:
    * Calculate grid dimensions (I, J)
    * Calculate spacing (X, Y)
    * Calculate origin
  - If irregular:
    * Store as point cloud

Step 3.6: Extract Surface Name
Airflow DAG (Python):
  - Extract from filename or CSV metadata
  - Default to filename if not specified

┌─────────────────────────────────────────────────────────────┐
│ Phase 4: Storage Service Ingestion                          │
└─────────────────────────────────────────────────────────────┘

Step 4.1: Create Surface WPC Record
Airflow DAG → Storage Service API
PUT /api/storage/v2/records
Content-Type: application/json
Authorization: Bearer <JWT>
Slb-Data-Partition-Id: osdu

Request Body:
[
  {
    "kind": "osdu:wks:work-product-component--Surface:1.0.0",
    "acl": {
      "viewers": ["data.default.viewers@osdu"],
      "owners": ["data.default.owners@osdu"]
    },
    "legal": {
      "legaltags": ["osdu-legaltag-001"],
      "otherRelevantDataCountries": ["US"]
    },
    "data": {
      "ResourceTypeID": "srn:type:work-product-component/Surface:",
      "Name": "F3-Horizon-Truncation",
      "SurfaceTypeID": "osdu:reference-data--SurfaceType:Horizon:",
      "Representation": {
        "Format": "CSV",
        "FileSource": "srn:type:file/csv:surface-001:",
        "PointCount": 10000,
        "SpatialExtent": {
          "MinX": 605882,
          "MaxX": 606000,
          "MinY": 6073657,
          "MaxY": 6073700,
          "MinZ": 1060,
          "MaxZ": 1062
        },
        "CoordinateReferenceSystem": "EPSG:32631",
        "Units": {
          "XY": "osdu:reference-data--UnitOfMeasure:M:",
          "Z": "osdu:reference-data--UnitOfMeasure:M:"
        }
      }
    }
  }
]

Response (200 OK):
[
  {
    "id": "osdu:doc:8dd91gh70eh6786f2hhdg6725j5jg32e",
    "version": 1582725123640845
  }
]

┌─────────────────────────────────────────────────────────────┐
│ Phase 5: Indexing                                            │
└─────────────────────────────────────────────────────────────┘

Step 5.1: Indexer Service Processing
Indexer Service (Automatic):
  - Receives record creation event
  - Extracts searchable fields:
    * Name
    * SurfaceType
    * SpatialExtent (for spatial queries)
    * CoordinateReferenceSystem
    * Format
  - Updates search index
  - Enables spatial queries
```

**CSV Parser DAG Implementation**:

```python
# csv_ingestion_all_steps.py (Airflow DAG)
def parse_csv_surface(file_source, data_partition_id):
    """
    Parse CSV file and create surface records
    """
    # 1. Read CSV from staging
    csv_data = file_service.read(file_source)

    # 2. Parse CSV
    import csv
    reader = csv.DictReader(csv_data)

    # 3. Extract surface points
    points = []
    for row in reader:
        point = {
            "x": float(row['X']),
            "y": float(row['Y']),
            "z": float(row['Z'])
        }
        points.append(point)

    # 4. Calculate spatial extent
    min_x = min(p['x'] for p in points)
    max_x = max(p['x'] for p in points)
    min_y = min(p['y'] for p in points)
    max_y = max(p['y'] for p in points)
    min_z = min(p['z'] for p in points)
    max_z = max(p['z'] for p in points)

    # 5. Determine coordinate reference system (from metadata or default)
    crs = determine_crs(file_source) or "EPSG:32631"

    # 6. Extract surface name from filename
    surface_name = extract_surface_name(file_source)

    # 7. Create surface record
    storage_service.ingest_records([{
        "kind": f"{data_partition_id}:wks:work-product-component--Surface:1.0.0",
        "data": {
            "Name": surface_name,
            "SurfaceTypeID": f"{data_partition_id}:reference-data--SurfaceType:Horizon:",
            "Representation": {
                "Format": "CSV",
                "FileSource": file_source,
                "PointCount": len(points),
                "SpatialExtent": {
                    "MinX": min_x,
                    "MaxX": max_x,
                    "MinY": min_y,
                    "MaxY": max_y,
                    "MinZ": min_z,
                    "MaxZ": max_z
                },
                "CoordinateReferenceSystem": crs
            }
        }
    }])

    return {"status": "success", "point_count": len(points)}
```

### 6.4 Pathway 4: GeoTIFF Surface Ingestion

**Architecture**:

```
GeoTIFF File Upload
    ↓
File Service (staging)
    ↓
GeoTIFF Parser
    ↓
Grid Extraction
    ↓
Storage Service (metadata + data)
    ↓
Indexer Service
```

**Complete Call Stack**:

```
┌─────────────────────────────────────────────────────────────┐
│ Phase 1: File Upload                                         │
└─────────────────────────────────────────────────────────────┘

Step 1.1: Upload GeoTIFF File
Client → File Service API
POST /v2/files/upload
Content-Type: multipart/form-data
Authorization: Bearer <JWT>
Slb-Data-Partition-Id: osdu

Request Body (multipart/form-data):
  - file: surface.tif (binary)

Response (200 OK):
{
  "FileID": "9ee02hi81fi7897g3iihe7836k6kh43f",
  "FileSource": "srn:type:file/tif:surface-002:",
  "Location": "gs://osdu-storage/osdu/files/9ee02hi81fi7897g3iihe7836k6kh43f/surface.tif"
}

┌─────────────────────────────────────────────────────────────┐
│ Phase 2: GeoTIFF Processing                                 │
└─────────────────────────────────────────────────────────────┘

Step 2.1: Read GeoTIFF File from Staging
GeoTIFF Parser → File Service
GET /v2/files/{fileId}/download
Authorization: Bearer <JWT>

Response: GeoTIFF file binary data

Step 2.2: Parse TIFF Headers
GeoTIFF Parser (Python):
  - Read TIFF header
  - Extract image dimensions (width, height)
  - Extract data type (float, integer)
  - Extract compression information

Step 2.3: Parse GeoTIFF Tags
GeoTIFF Parser (Python):
  - Read ModelTiepointTag (georeferencing tiepoints)
  - Read ModelPixelScaleTag (pixel scale in X, Y, Z)
  - Read GeoKeyDirectoryTag (coordinate system information)
  - Extract coordinate reference system

Step 2.4: Extract Grid Data
GeoTIFF Parser (Python):
  - Read pixel values
  - Apply geotransform to get coordinates
  - Generate grid structure
  - Calculate spatial extent

Step 2.5: Generate Statistics
GeoTIFF Parser (Python):
  - Calculate min, max, mean, std
  - Generate data range

┌─────────────────────────────────────────────────────────────┐
│ Phase 3: Storage Service Ingestion                          │
└─────────────────────────────────────────────────────────────┘

Step 3.1: Create Surface WPC Record
GeoTIFF Parser → Storage Service API
PUT /api/storage/v2/records
Content-Type: application/json
Authorization: Bearer <JWT>
Slb-Data-Partition-Id: osdu

Request Body:
[
  {
    "kind": "osdu:wks:work-product-component--Surface:1.0.0",
    "acl": {
      "viewers": ["data.default.viewers@osdu"],
      "owners": ["data.default.owners@osdu"]
    },
    "legal": {
      "legaltags": ["osdu-legaltag-001"],
      "otherRelevantDataCountries": ["US"]
    },
    "data": {
      "ResourceTypeID": "srn:type:work-product-component/Surface:",
      "Name": "Surface from GeoTIFF",
      "SurfaceTypeID": "osdu:reference-data--SurfaceType:Horizon:",
      "Representation": {
        "Format": "GeoTIFF",
        "FileSource": "srn:type:file/tif:surface-002:",
        "GridDimensions": {
          "Width": 1000,
          "Height": 800
        },
        "SpatialExtent": {
          "MinX": 400000,
          "MaxX": 500000,
          "MinY": 6500000,
          "MaxY": 7000000,
          "MinZ": 1000,
          "MaxZ": 2000
        },
        "CoordinateReferenceSystem": "EPSG:32631",
        "DataStatistics": {
          "Min": 1000,
          "Max": 2000,
          "Mean": 1500
        }
      }
    }
  }
]

Response (200 OK):
[
  {
    "id": "osdu:doc:0ff13ij92gj8908h4jjif8947l7li54g",
    "version": 1582725123640845
  }
]

┌─────────────────────────────────────────────────────────────┐
│ Phase 4: Indexing                                            │
└─────────────────────────────────────────────────────────────┘

Step 4.1: Indexer Service Processing
Indexer Service (Automatic):
  - Receives record creation event
  - Extracts searchable fields
  - Updates search index
```

### 6.5 Pathway 5: Manifest-Based Ingestion

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
Seismic DDMS / Storage Service
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
          "kind": "osdu:wks:work-product-component--SeismicHorizon:1.0.0",
          "data": {
            "ResourceTypeID": "srn:type:work-product-component/SeismicHorizon:",
            "Name": "Top Reservoir Horizon",
            "Description": "Interpreted top of reservoir formation",
            "InterpretationProjectID": "osdu:master-data--InterpretationProject:001:",
            "SeismicGeometryContextID": "osdu:master-data--SeismicGeometryContext:001:",
            "HorizonTypeID": "osdu:reference-data--HorizonType:Top:",
            "SurfaceRepresentation": {
              "Format": "ZGY",
              "FileSource": "gs://bucket/horizons/top-reservoir.zgy",
              "GridDimensions": {
                "InlineCount": 1000,
                "CrosslineCount": 500,
                "SampleCount": 1
              },
              "SpatialExtent": {
                "MinX": 400000,
                "MaxX": 500000,
                "MinY": 6500000,
                "MaxY": 7000000,
                "MinZ": 1000,
                "MaxZ": 2000
              },
              "CoordinateReferenceSystem": "EPSG:32631"
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

Step 3.3: Validate Surface Components
Manifest Parser:
  - For each surface component:
    * Validate surface geometry
    * Validate coordinate reference system
    * Validate file source exists
    * Validate grid dimensions (if provided)

Step 3.4: Resolve Reference Data
Manifest Parser → Reference Data Service:
  - Resolve InterpretationProjectID
  - Resolve SeismicGeometryContextID
  - Resolve HorizonTypeID
  - Resolve SurfaceTypeID
  - Resolve UnitOfMeasure references

┌─────────────────────────────────────────────────────────────┐
│ Phase 4: Seismic DDMS Storage (if ZGY/VDS format)           │
└─────────────────────────────────────────────────────────────┘

Step 4.1: Store Volume in Seismic DDMS (if applicable)
Manifest Parser → Seismic DDMS API
POST /seistore-svc/api/v3/surveys/{surveyId}/volumes
Content-Type: application/json
Authorization: Bearer <JWT>

Request Body:
{
  "volumeId": "volume-002",
  "format": "ZGY",
  "fileSource": "gs://bucket/horizons/top-reservoir.zgy",
  "geometry": {
    "inlineCount": 1000,
    "crosslineCount": 500,
    "sampleCount": 1
  },
  "coordinateReferenceSystem": "EPSG:32631"
}

Response (200 OK):
{
  "volumeId": "volume-002",
  "status": "STORED"
}

┌─────────────────────────────────────────────────────────────┐
│ Phase 5: Storage Service Batch Ingestion                    │
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
      "Name": "Seismic Interpretation Work Product",
      "WorkProductComponents": [
        "osdu:doc:1gg24jk03hk9019i5kkjg9058m8mj65h"
      ]
    }
  },
  {
    "kind": "osdu:wks:work-product-component--SeismicHorizon:1.0.0",
    "data": {
      "ResourceTypeID": "srn:type:work-product-component/SeismicHorizon:",
      "Name": "Top Reservoir Horizon",
      "InterpretationProjectID": "osdu:master-data--InterpretationProject:001:",
      "SeismicGeometryContextID": "osdu:master-data--SeismicGeometryContext:001:",
      "HorizonTypeID": "osdu:reference-data--HorizonType:Top:",
      "SurfaceRepresentation": {
        "Format": "ZGY",
        "FileSource": "gs://bucket/horizons/top-reservoir.zgy",
        "GridDimensions": {
          "InlineCount": 1000,
          "CrosslineCount": 500,
          "SampleCount": 1
        },
        "SpatialExtent": {
          "MinX": 400000,
          "MaxX": 500000,
          "MinY": 6500000,
          "MaxY": 7000000,
          "MinZ": 1000,
          "MaxZ": 2000
        },
        "CoordinateReferenceSystem": "EPSG:32631"
      }
    }
  },
  {
    "kind": "osdu:wks:dataset--File.Generic:1.0.0",
    "data": {
      "DatasetProperties": {
        "FileSourceInfo": {
          "FileSource": "gs://bucket/horizons/top-reservoir.zgy"
        }
      }
    }
  }
]

Response (200 OK):
[
  {
    "id": "osdu:doc:2hh35kl14il0120j6llkh0169n9nk76i",
    "version": 1582725123640845
  },
  {
    "id": "osdu:doc:1gg24jk03hk9019i5kkjg9058m8mj65h",
    "version": 1582725123640845
  },
  {
    "id": "osdu:doc:3ii46lm25jm1231k7mmli1270o0ol87j",
    "version": 1582725123640845
  }
]

┌─────────────────────────────────────────────────────────────┐
│ Phase 6: Indexing                                            │
└─────────────────────────────────────────────────────────────┘

Step 6.1: Indexer Service Processing
Indexer Service (Automatic):
  - Receives record creation events
  - Indexes all ingested records
  - Creates searchable metadata
  - Enables spatial queries
```

### 6.6 Pathway 6: Direct Storage API Ingestion

**Architecture**:

```
Application Code
    ↓
Storage Service API (metadata)
    ↓
File Service (bulk data, if needed)
    ↓
Indexer Service
```

**Complete Call Stack**:

```
┌─────────────────────────────────────────────────────────────┐
│ Phase 1: Prepare Surface Data                              │
└─────────────────────────────────────────────────────────────┘

Step 1.1: Prepare Surface Record
Application Code:
  const surfaceRecord = {
    "kind": "osdu:wks:work-product-component--Surface:1.0.0",
    "acl": {
      "viewers": ["data.default.viewers@osdu"],
      "owners": ["data.default.owners@osdu"]
    },
    "legal": {
      "legaltags": ["osdu-legaltag-001"],
      "otherRelevantDataCountries": ["US"]
    },
    "data": {
      "ResourceTypeID": "srn:type:work-product-component/Surface:",
      "Name": "F3-Horizon-Truncation",
      "SurfaceTypeID": "osdu:reference-data--SurfaceType:Horizon:",
      "Representation": {
        "Format": "CSV",
        "FileSource": "srn:type:file/csv:surface-001:",
        "PointCount": 10000,
        "SpatialExtent": {
          "MinX": 605882,
          "MaxX": 606000,
          "MinY": 6073657,
          "MaxY": 6073700,
          "MinZ": 1060,
          "MaxZ": 1062
        },
        "CoordinateReferenceSystem": "EPSG:32631",
        "Units": {
          "XY": "osdu:reference-data--UnitOfMeasure:M:",
          "Z": "osdu:reference-data--UnitOfMeasure:M:"
        }
      }
    }
  };

┌─────────────────────────────────────────────────────────────┐
│ Phase 2: Ingest via Storage Service API                     │
└─────────────────────────────────────────────────────────────┘

Step 2.1: Create Surface Record
Application → Storage Service API
PUT /api/storage/v2/records
Content-Type: application/json
Authorization: Bearer <JWT>
Slb-Data-Partition-Id: osdu

Request Body:
[
  {
    "kind": "osdu:wks:work-product-component--Surface:1.0.0",
    "acl": {
      "viewers": ["data.default.viewers@osdu"],
      "owners": ["data.default.owners@osdu"]
    },
    "legal": {
      "legaltags": ["osdu-legaltag-001"],
      "otherRelevantDataCountries": ["US"]
    },
    "data": {
      "ResourceTypeID": "srn:type:work-product-component/Surface:",
      "Name": "F3-Horizon-Truncation",
      "SurfaceTypeID": "osdu:reference-data--SurfaceType:Horizon:",
      "Representation": {
        "Format": "CSV",
        "FileSource": "srn:type:file/csv:surface-001:",
        "PointCount": 10000,
        "SpatialExtent": {
          "MinX": 605882,
          "MaxX": 606000,
          "MinY": 6073657,
          "MaxY": 6073700,
          "MinZ": 1060,
          "MaxZ": 1062
        },
        "CoordinateReferenceSystem": "EPSG:32631",
        "Units": {
          "XY": "osdu:reference-data--UnitOfMeasure:M:",
          "Z": "osdu:reference-data--UnitOfMeasure:M:"
        }
      }
    }
  }
]

Response (200 OK):
[
  {
    "id": "osdu:doc:4jj57mn36kn2342l8nnmj2381p1pm98k",
    "version": 1582725123640845
  }
]

┌─────────────────────────────────────────────────────────────┐
│ Phase 3: Indexing                                            │
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
  "kind": "osdu:wks:work-product-component--SeismicHorizon:1.0.0",
  "data": {
    "ResourceTypeID": "srn:type:work-product-component/SeismicHorizon:",
    "Name": "Top Reservoir Horizon",
    "InterpretationProjectID": "osdu:master-data--InterpretationProject:001:",
    "SeismicGeometryContextID": "osdu:master-data--SeismicGeometryContext:001:",
    "HorizonTypeID": "osdu:reference-data--HorizonType:Top:",
    "SurfaceRepresentation": {
      "Format": "ZGY",
      "FileSource": "gs://bucket/horizons/top-reservoir.zgy",
      "GridDimensions": {
        "InlineCount": 1000,
        "CrosslineCount": 500,
        "SampleCount": 1
      },
      "SpatialExtent": {
        "MinX": 400000,
        "MaxX": 500000,
        "MinY": 6500000,
        "MaxY": 7000000,
        "MinZ": 1000,
        "MaxZ": 2000
      },
      "CoordinateReferenceSystem": "EPSG:32631"
    }
  }
}
```

**Bulk Data Storage (Seismic DDMS)**:

```
Seismic DDMS Backend
    ↓
Volume Document:
    {
      "volumeId": "volume-001",
      "surveyId": "survey-001",
      "format": "ZGY",
      "fileSource": "gs://bucket/surveys/survey-001.zgy",
      "geometry": {
        "inlineCount": 1000,
        "crosslineCount": 500,
        "sampleCount": 2000
      },
      "chunks": [
        {
          "chunkId": "chunk-001",
          "inlineRange": [0, 100],
          "crosslineRange": [0, 100],
          "fileOffset": 0,
          "size": 1048576
        },
        ...
      ]
    }
    ↓
Indexed for Spatial Queries:
    - Spatial index on inline/crossline ranges
    - Survey index for fast lookups
    - Format index for format-based queries
```

### 7.2 File Storage

**Source File Preservation**:

```
File Service (Object Store)
    ↓
Source File Storage:
    gs://osdu-storage/osdu/files/{file-id}/survey.segy
    gs://osdu-storage/osdu/files/{file-id}/surface.csv
    gs://osdu-storage/osdu/files/{file-id}/horizon.tif
    ↓
Dataset Record Reference:
    {
      "kind": "osdu:wks:dataset--File.Generic:1.0.0",
      "data": {
        "DatasetProperties": {
          "FileSourceInfo": {
            "FileSource": "gs://osdu-storage/osdu/files/{file-id}/survey.segy"
          }
        }
      }
    }
    ↓
Linked to Surface:
    Surface.data.Data.GroupTypeProperties.Files = [
      "srn:type:file/segy:survey-001:"
    ]
```

### 7.3 Seismic DDMS Architecture

**Bulk Data Storage**:

Seismic DDMS provides optimized storage and access for seismic surface bulk data:

**API Endpoints**:

```http
# Store seismic volume
POST /seistore-svc/api/v3/surveys/{surveyId}/volumes
Content-Type: application/json

{
  "volumeId": "volume-001",
  "format": "ZGY",
  "fileSource": "gs://bucket/surveys/survey-001.zgy",
  "geometry": {
    "inlineCount": 1000,
    "crosslineCount": 500,
    "sampleCount": 2000,
    "sampleRate": 0.004
  },
  "coordinateReferenceSystem": "EPSG:32631"
}

# Retrieve volume metadata
GET /seistore-svc/api/v3/surveys/{surveyId}/volumes/{volumeId}

# Query volumes
GET /seistore-svc/api/v3/surveys/{surveyId}/volumes?format=ZGY

# Extract horizon slice
POST /seistore-svc/api/v3/volumes/{volumeId}/extract-horizon
{
  "inlineRange": [100, 200],
  "crosslineRange": [50, 150],
  "sampleIndex": 500
}
```

**Storage Benefits**:

- **Optimized Access**: Efficient retrieval of large seismic volumes
- **Chunked Storage**: Data organized in chunks for partial reads
- **Compression**: Built-in compression for storage efficiency
- **Spatial Indexing**: Fast queries by inline/crossline ranges
- **Cloud-Native**: Designed for object storage (S3, GCS, Azure Blob)

### 7.4 Reservoir DDMS Architecture

**Property Grid Storage**:

Reservoir DDMS provides optimized storage for reservoir property grids:

**API Endpoints** (Inferred):

```http
# Store property grid
POST /reservoir-ddms/api/v1/grids/{gridId}/properties
Content-Type: application/json

{
  "propertyId": "property-001",
  "propertyType": "Porosity",
  "format": "ZGY",
  "fileSource": "gs://bucket/grids/porosity.zgy",
  "gridDimensions": {
    "I": 200,
    "J": 150,
    "K": 50
  },
  "dataRange": {
    "Min": 0.0,
    "Max": 0.35
  }
}

# Retrieve property grid
GET /reservoir-ddms/api/v1/grids/{gridId}/properties/{propertyId}

# Query property grids
GET /reservoir-ddms/api/v1/grids/{gridId}/properties?propertyType=Porosity
```

---

## 8. Data Transformation & Validation

### 8.1 Reference Data Resolution

**Coordinate Reference System Resolution**:

```
Input: "EPSG:32631" or "WGS_1984_UTM_Zone_31N"
    ↓
CRS Service Lookup
    ↓
Validate CRS exists and is supported
    ↓
Resolved CRS: Full WKT definition
```

**Surface Type Resolution**:

```
Input: "Horizon" or "Top" or "Base"
    ↓
Reference Data Service Lookup
    GET /api/reference-data/v2/reference-data/osdu:reference-data--SurfaceType:Horizon:
    ↓
Response:
    {
      "code": "Horizon",
      "name": "Seismic Horizon",
      "description": "Interpreted geological boundary"
    }
    ↓
Resolved Reference:
    "osdu:reference-data--SurfaceType:Horizon:"
```

### 8.2 Grid Structure Generation

**Regular Grid Detection**:

```python
def detect_regular_grid(points):
    """
    Detect if points form a regular grid
    """
    # 1. Sort points by X, then Y
    sorted_points = sorted(points, key=lambda p: (p['x'], p['y']))

    # 2. Calculate X and Y spacing
    x_coords = sorted(set(p['x'] for p in points))
    y_coords = sorted(set(p['y'] for p in points))

    # 3. Check if spacing is regular
    x_spacing = calculate_spacing(x_coords)
    y_spacing = calculate_spacing(y_coords)

    # 4. If regular, generate grid structure
    if is_regular_spacing(x_spacing) and is_regular_spacing(y_spacing):
        return {
            "type": "regular",
            "dimensions": {
                "I": len(x_coords),
                "J": len(y_coords)
            },
            "spacing": {
                "X": x_spacing,
                "Y": y_spacing
            },
            "origin": {
                "X": min(x_coords),
                "Y": min(y_coords)
            }
        }
    else:
        return {
            "type": "irregular",
            "pointCount": len(points)
        }
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
    - Name → Full-text search
    - SurfaceType → Filter by type
    - SpatialExtent → Spatial queries
    - CoordinateReferenceSystem → Filter by CRS
    - GridDimensions → Filter by size
    - Format → Filter by format
    ↓
Update Search Index
    ↓
Search Service Available
```

### 9.2 Query Examples

**Search by Surface Name**:

```http
POST /api/search/v2/query
Content-Type: application/json
Authorization: Bearer <JWT>
Slb-Data-Partition-Id: osdu

{
  "kind": "osdu:wks:work-product-component--Surface:1.0.0",
  "query": "data.Name:\"Top Reservoir\""
}
```

**Search by Spatial Extent**:

```http
POST /api/search/v2/query
Content-Type: application/json
Authorization: Bearer <JWT>
Slb-Data-Partition-Id: osdu

{
  "kind": "osdu:wks:work-product-component--Surface:1.0.0",
  "query": "data.SurfaceRepresentation.SpatialExtent.MinX:[400000 TO 500000] AND data.SurfaceRepresentation.SpatialExtent.MinY:[6500000 TO 7000000]"
}
```

**Search by Format**:

```http
POST /api/search/v2/query
Content-Type: application/json
Authorization: Bearer <JWT>
Slb-Data-Partition-Id: osdu

{
  "kind": "osdu:wks:work-product-component--Surface:1.0.0",
  "query": "data.SurfaceRepresentation.Format:\"ZGY\""
}
```

---

## 10. Error Handling & Validation

### 10.1 Validation Errors

**Common Validation Failures**:

1. **Invalid SEGY Format**:

   ```
   Error: Invalid SEGY header format
   Resolution: Verify SEGY file structure and headers
   ```

2. **Missing Coordinate Reference System**:

   ```
   Error: CoordinateReferenceSystem not specified or invalid
   Resolution: Provide valid CRS identifier (EPSG code)
   ```

3. **Invalid Grid Dimensions**:

   ```
   Error: Grid dimensions do not match data size
   Resolution: Verify grid dimensions match actual data
   ```

4. **CSV Column Mismatch**:

   ```
   Error: Required columns X, Y, Z not found in CSV
   Resolution: Ensure CSV contains X, Y, Z columns
   ```

5. **Coordinate Range Validation**:
   ```
   Error: X coordinate 20000000 exceeds maximum range
   Resolution: Validate coordinate ranges before ingestion
   ```

### 10.2 Schema Validation

**Required Field Validation**:

- Fields marked as required in schema must be present
- Missing required fields cause ingestion failure

**Data Type Validation**:

- String fields must be strings
- Numeric fields must be numbers
- Array fields must be arrays
- Object fields must be objects

**Reference Validation**:

- Master data references must exist
- Reference data values must be valid
- Dataset references must exist

**Example Validation Error**:

```json
{
  "error": "Schema validation failed",
  "kind": "osdu:wks:work-product-component--Surface:1.0.0",
  "errors": [
    {
      "path": "data.Name",
      "message": "Required field missing"
    },
    {
      "path": "data.Representation.SpatialExtent.MinX",
      "message": "Expected Double, got String"
    }
  ]
}
```

---

## 11. Performance Considerations

### 11.1 Batch Ingestion

**Optimal Batch Size**:

- **Small surfaces (< 10K points)**: Process individually
- **Medium surfaces (10K-100K points)**: Batch by surface type
- **Large surfaces (> 100K points)**: Chunk processing recommended

**Recommended**: Process surfaces individually, batch metadata records (100-200 per batch)

### 11.2 Chunked Processing

**SEGY to ZGY Chunking**:

```python
def convert_segy_to_zgy_chunked(segy_file, chunk_size=100):
    """
    Convert SEGY to ZGY with chunked processing
    """
    # 1. Parse SEGY headers
    headers = parse_segy_headers(segy_file)

    # 2. Calculate chunk boundaries
    inline_count = headers['inline_count']
    crossline_count = headers['crossline_count']

    inline_chunks = (inline_count + chunk_size - 1) // chunk_size
    crossline_chunks = (crossline_count + chunk_size - 1) // chunk_size

    # 3. Process each chunk
    zgy_chunks = []
    for i_chunk in range(inline_chunks):
        for j_chunk in range(crossline_chunks):
            # Extract chunk from SEGY
            chunk_data = extract_chunk(
                segy_file,
                inline_range=[i_chunk * chunk_size, (i_chunk + 1) * chunk_size],
                crossline_range=[j_chunk * chunk_size, (j_chunk + 1) * chunk_size]
            )

            # Convert chunk to ZGY format
            zgy_chunk = convert_chunk_to_zgy(chunk_data)
            zgy_chunks.append(zgy_chunk)

    # 4. Combine chunks into ZGY file
    zgy_file = combine_zgy_chunks(zgy_chunks, headers)

    return zgy_file
```

---

## 12. Design Patterns and Best Practices

### 12.1 Schema First Approach

1. **Define schemas before ingestion**: Ensures type safety and proper indexing
2. **Version schemas carefully**: Schema changes require data migration
3. **Index only necessary fields**: Reduces index size and improves performance
4. **Validate at ingestion**: Catch errors early in the pipeline

### 12.2 Work Product Pattern

1. **Group related WPCs**: Use Work Products to organize related surfaces
2. **Maintain relationships**: Link surfaces to interpretation projects and geometry contexts
3. **Preserve lineage**: Track source files and processing history

### 12.3 Format Optimization

1. **Convert SEGY to ZGY/VDS**: Optimize for cloud storage and access
2. **Preserve source files**: Keep original files for traceability
3. **Use appropriate formats**: Choose format based on access patterns

### 12.4 Spatial Data Handling

1. **Specify CRS explicitly**: Always provide coordinate reference system
2. **Validate spatial extent**: Ensure coordinates are within valid ranges
3. **Use spatial indexing**: Enable efficient spatial queries
4. **Handle CRS transformations**: Convert between CRS when needed

---

## 13. Important Considerations

### 13.1 Data Partitioning

- All surface operations require `data-partition-id` header
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

---

## 14. Summary

OSDU provides **comprehensive, multi-format ingestion** for 3D surface data through specialized services:

1. **Seismic DDMS**: Optimized bulk storage for seismic surfaces (SEGY → ZGY/VDS conversion)
2. **CSV Parser**: Automated ingestion from CSV files with point-based surfaces
3. **GeoTIFF Support**: Raster surface data ingestion
4. **RESQML Support**: Industry-standard reservoir surface formats
5. **Manifest-based**: Structured JSON manifests for complex workflows
6. **Direct API**: Programmatic record creation for custom applications

**Key Design Decisions**:

- **Metadata/Bulk Separation**: Metadata in Storage Service, bulk data in Seismic DDMS
- **Format Conversion**: SEGY → ZGY/VDS for optimized cloud storage
- **Grid-Based Storage**: Efficient storage for regular and irregular grids
- **Source File Preservation**: Original files stored alongside converted formats
- **Search Integration**: Automatic indexing for discovery and spatial queries
- **Access Control**: ACLs and legal tags enforced at ingestion time

**Call Stack Highlights**:

- **SEGY to ZGY**: File upload → DAG trigger → SEGY parsing → ZGY conversion → Seismic DDMS storage → Metadata ingest → Index
- **CSV Parser**: File upload → DAG trigger → CSV parsing → Grid generation → Storage API → Index
- **GeoTIFF**: File upload → GeoTIFF parsing → Grid extraction → Storage API → Index
- **Manifest**: JSON creation → Workflow API → Parse & validate → Seismic DDMS/Storage → Batch ingest → Index

This architecture ensures **data consistency**, **spatial accuracy**, **format optimization**, **traceability**, and **governance** while supporting multiple ingestion formats and surface types.

---

## 15. References

1. OSDU 3D Surface Ingestion & Storage Architecture Documentation
2. OSDU Data Formats, Schemas & Storage Architecture Documentation
3. OSDU Storage Service API Documentation
4. OSDU Seismic DDMS API Documentation
5. OSDU Workflow Service API Documentation
6. OSDU File Service API Documentation
7. OSDU Search Service API Documentation
8. OSDU Reference Data Service API Documentation
9. SEGY File Format Specification
10. ZGY Format Specification
11. VDS Format Specification
12. GeoTIFF Specification
13. RESQML Specification
