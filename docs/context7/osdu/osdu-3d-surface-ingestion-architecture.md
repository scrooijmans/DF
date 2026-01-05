# OSDU 3D Surface Ingestion & Storage Architecture

This document explains how OSDU handles 3D surfaces (seismic horizons, geological surfaces, property grids) when ingesting and storing different data formats, including the design architecture and call stack for related workflows.

## Overview

OSDU handles 3D surface data through **multiple ingestion pathways** and specialized services:

1. **Seismic DDMS**: Optimized bulk storage for seismic surfaces (SEGY, ZGY, VDS formats)
2. **CSV Parser Ingestion**: Automated CSV file parsing for point-based surfaces
3. **Manifest-based Ingestion**: Structured JSON manifests for surface records
4. **Direct Storage API**: Programmatic record creation via Storage Service API
5. **File Service**: Generic file ingestion for GeoTIFF, RESQML, and other formats
6. **Reservoir DDMS**: Specialized service for reservoir property grids and structural models

**Key Principles:**

- **Seismic DDMS**: Specialized service for bulk seismic surface data (SEGY → ZGY/VDS conversion)
- **Metadata/Bulk Separation**: Metadata in Storage Service, bulk data in DDMS
- **Multi-Format Support**: SEGY, ZGY, VDS, CSV, GeoTIFF, RESQML ingestion
- **Work Product Model**: Surfaces as Work Product Components (SeismicHorizon, Surface, GridProperty)
- **Coordinate System Support**: Multiple CRS support with spatial indexing
- **Grid-Based Storage**: Efficient storage for regular and irregular grids

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                    OSDU Data Platform                        │
│                                                             │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  Ingestion Pathways                                 │   │
│  │  ┌──────────────┐ ┌──────────────┐ ┌──────────────┐ │   │
│  │  │ SEGY→ZGY    │ │ CSV Parser  │ │  Manifest   │ │   │
│  │  │   DAG       │ │   DAG       │ │  Workflow   │ │   │
│  │  └──────┬──────┘ └──────┬───────┘ └──────┬───────┘ │   │
│  │         │               │                 │          │   │
│  │         ▼               ▼                 ▼          │   │
│  │  ┌──────────────────────────────────────────────┐  │   │
│  │  │  Format Conversion & Validation                │  │   │
│  │  │  - SEGY → ZGY/VDS conversion                 │  │   │
│  │  │  - CSV → Grid structure                      │  │   │
│  │  │  - GeoTIFF → Grid extraction                 │  │   │
│  │  │  - RESQML → Surface mapping                  │  │   │
│  │  └──────────────────────┬───────────────────────┘  │   │
│  └─────────┼───────────────┼────────────────┼─────────┘   │
│            │               │                 │              │
│            ▼               ▼                 ▼              │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  Schema Validation & Transformation                  │   │
│  │  - Validate surface geometry                        │   │
│  │  - Resolve reference data (CRS, units)             │   │
│  │  - Transform to standardized format                │   │
│  └──────────────────────┬──────────────────────────────┘   │
│                         │                                    │
│                         ▼                                    │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  Seismic DDMS / Reservoir DDMS                      │   │
│  │  - Optimized bulk surface storage                   │   │
│  │  - Grid-based access patterns                      │   │
│  │  - Spatial indexing                                │   │
│  └──────────────────────┬──────────────────────────────┘   │
│                         │                                    │
│                         ▼                                    │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  Storage Service                                     │   │
│  │  - Surface metadata records                        │   │
│  │  - WorkProduct and WorkProductComponent           │   │
│  │  - Dataset records (file references)             │   │
│  │  - ACLs and legal tags                            │   │
│  └──────────────────────┬──────────────────────────────┘   │
│                         │                                    │
│                         ▼                                    │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  Indexer Service                                     │   │
│  │  - Index surface metadata                          │   │
│  │  - Extract searchable fields                       │   │
│  │  - Spatial indexing                                │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                             │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  File Storage (Object Store)                        │   │
│  │  - Store source SEGY/ZGY/VDS files                 │   │
│  │  - Store GeoTIFF/RESQML files                     │   │
│  │  - Preserve original data format                   │   │
│  └─────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
```

## Surface Data Types

### 1. Seismic Horizons

Seismic horizons represent interpreted geological boundaries from seismic data:

```json
{
	"kind": "{{data-partition-id}}:wks:work-product-component--SeismicHorizon:1.0.0",
	"data": {
		"ResourceTypeID": "srn:type:work-product-component/SeismicHorizon:",
		"Name": "Top Reservoir Horizon",
		"Description": "Interpreted top of reservoir formation",
		"InterpretationProjectID": "{{data-partition-id}}:master-data--InterpretationProject:001:",
		"SeismicGeometryContextID": "{{data-partition-id}}:master-data--SeismicGeometryContext:001:",
		"HorizonTypeID": "{{data-partition-id}}:reference-data--HorizonType:Top:",
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

### 2. Geological Surfaces

Point-based or grid-based geological surfaces:

```json
{
	"kind": "{{data-partition-id}}:wks:work-product-component--Surface:1.0.0",
	"data": {
		"Name": "F3-Horizon-Truncation",
		"SurfaceTypeID": "{{data-partition-id}}:reference-data--SurfaceType:Horizon:",
		"Representation": {
			"Format": "CSV",
			"FileSource": "gs://bucket/surfaces/f3-truncation.csv",
			"PointCount": 10000,
			"CoordinateReferenceSystem": "EPSG:32631",
			"Units": {
				"XY": "{{data-partition-id}}:reference-data--UnitOfMeasure:M:",
				"Z": "{{data-partition-id}}:reference-data--UnitOfMeasure:M:"
			}
		}
	}
}
```

### 3. Property Grids

3D property grids (porosity, permeability, saturation, etc.):

```json
{
	"kind": "{{data-partition-id}}:wks:work-product-component--GridProperty:1.0.0",
	"data": {
		"Name": "Porosity Grid",
		"PropertyTypeID": "{{data-partition-id}}:reference-data--PropertyType:Porosity:",
		"GridID": "{{data-partition-id}}:master-data--Grid:001:",
		"Representation": {
			"Format": "ZGY",
			"FileSource": "gs://bucket/grids/porosity.zgy",
			"GridDimensions": {
				"I": 200,
				"J": 150,
				"K": 50
			},
			"DataRange": {
				"Min": 0.0,
				"Max": 0.35
			},
			"Unit": "{{data-partition-id}}:reference-data--UnitOfMeasure:V/V:"
		}
	}
}
```

## Data Formats

### 1. SEGY Format

SEGY (SEG Y) is the industry standard for seismic data:

**Structure:**

- **Text Header**: 3200-byte EBCDIC text header
- **Binary Header**: 400-byte binary header with survey metadata
- **Trace Headers**: 240-byte header per trace
- **Trace Data**: Amplitude samples (typically 32-bit floats)

**Key Metadata:**

- Survey geometry (inline/crossline spacing)
- Coordinate reference system
- Sample rate and count
- Data format (IEEE float, integer, etc.)

### 2. ZGY Format

ZGY (ZGY Seismic Data Format) is an optimized format for cloud storage:

**Advantages:**

- **Chunked Storage**: Data organized in chunks for efficient access
- **Compression**: Built-in compression support
- **Metadata**: Rich metadata embedded in format
- **Cloud-Optimized**: Designed for object storage (S3, GCS, Azure Blob)

**Conversion Process:**

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

### 3. VDS Format

VDS (Virtual Data Store) is another optimized format for seismic data:

**Advantages:**

- **Virtualization**: Logical view of data without physical reorganization
- **Efficient Access**: Optimized for random access patterns
- **Metadata**: Comprehensive metadata support
- **Cloud-Native**: Designed for distributed storage

### 4. CSV Format

CSV files for point-based surfaces:

```csv
X,Y,Z
605882.732209,6073657.908672,1061.42
605907.722462,6073658.606713,1061.31
605932.712715,6073659.304754,1061.14
605957.702968,6073660.002795,1060.83
```

**Column Mapping:**

- **X** → X-coordinate (easting)
- **Y** → Y-coordinate (northing)
- **Z** → Z-coordinate (depth/elevation)
- **SurfaceName** → Optional surface identifier

### 5. GeoTIFF Format

GeoTIFF files for raster surface data:

**Structure:**

- **TIFF Header**: Standard TIFF metadata
- **GeoTIFF Tags**: Coordinate reference system, geotransform
- **Image Data**: Gridded surface values (single or multi-band)

**Key Tags:**

- `ModelTiepointTag`: Georeferencing tiepoints
- `ModelPixelScaleTag`: Pixel scale in X, Y, Z
- `GeoKeyDirectoryTag`: Coordinate system information

### 6. RESQML Format

RESQML (REServoir Modeling Markup Language) for reservoir surfaces:

**Structure:**

- **XML-based**: Industry standard XML format
- **Surface Representations**: Multiple representation types
- **Grid Definitions**: Structured and unstructured grids
- **Property Arrays**: Property data linked to grids

## Ingestion Pathways

### Pathway 1: SEGY to ZGY Conversion

**Architecture:**

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

**Call Stack:**

```
1. SEGY File Upload
   POST /v2/files/upload
   → Returns FileSource identifier

2. Trigger SEGY to ZGY Conversion DAG
   POST /v1/workflow/Osdu_ingest/workflowRun
   {
     "executionContext": {
       "Payload": {
         "AppKey": "segy-to-zgy-conversion",
         "data-partition-id": "{{data-partition-id}}",
         "FileSource": "srn:type:file/segy:survey-001:"
       }
     }
   }
   → Returns WorkflowID

3. SEGY to ZGY Conversion Processing (Airflow DAG)
   a. Read SEGY file from staging
   b. Parse SEGY headers:
      - Text header (3200 bytes EBCDIC)
      - Binary header (400 bytes)
      - Extract survey geometry
      - Extract coordinate system
   c. Parse trace headers:
      - Inline/crossline numbers
      - X/Y coordinates
      - Sample rate and count
   d. Read trace data:
      - Amplitude samples per trace
      - Data format (IEEE float, integer)
   e. Organize data by inline/crossline
   f. Chunk data for ZGY format:
      - Group traces by spatial proximity
      - Create chunks for efficient access
   g. Compress chunks (optional)
   h. Generate ZGY metadata:
      - Survey geometry
      - Coordinate reference system
      - Data statistics
   i. Write ZGY file to object store

4. Seismic DDMS Storage
   POST /seistore-svc/api/v3/surveys/{surveyId}/volumes
   {
     "volumeId": "volume-001",
     "format": "ZGY",
     "fileSource": "gs://bucket/surveys/survey-001.zgy",
     "geometry": {
       "inlineCount": 1000,
       "crosslineCount": 500,
       "sampleCount": 2000,
       "sampleRate": 0.004,
       "coordinateReferenceSystem": "EPSG:32631"
     }
   }
   → Returns volume ID

5. Storage Service Metadata Ingestion
   PUT /api/storage/v2/records
   [
     {
       "kind": "{{data-partition-id}}:wks:work-product-component--SeismicVolume:1.0.0",
       "data": {
         "SurveyID": "...",
         "VolumeID": "volume-001",
         "Format": "ZGY",
         "Geometry": { ... },
         "FileSource": "gs://bucket/surveys/survey-001.zgy"
       }
     }
   ]
   → Returns record IDs

6. Indexer Service
   - Indexes seismic volume metadata
   - Extracts searchable fields (SurveyID, geometry, CRS)
   - Updates search index
```

**SEGY to ZGY Conversion DAG Implementation:**

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

### Pathway 2: SEGY to VDS Conversion

**Architecture:**

Similar to SEGY to ZGY, but converts to VDS format instead:

```
SEGY File Upload
    ↓
File Service (staging)
    ↓
SEGY to VDS Conversion DAG (Airflow)
    ↓
SEGY Parsing & Validation
    ↓
VDS Conversion
    ↓
Seismic DDMS Storage
    ↓
Storage Service (metadata)
```

**Call Stack:**

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

### Pathway 3: CSV Surface Ingestion

**Architecture:**

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
         "FileSource": "srn:type:file/csv:surface-001:"
       }
     }
   }
   → Returns WorkflowID

3. CSV Parser Processing (Airflow DAG)
   a. Read CSV file from staging
   b. Parse header row (X, Y, Z columns)
   c. For each data row:
      - Extract X, Y, Z coordinates
      - Validate coordinate ranges
      - Validate data types
   d. Generate surface metadata:
      - Calculate spatial extent
      - Count points
      - Determine coordinate reference system
   e. Optionally generate grid structure:
      - Regular grid (if points form grid)
      - Irregular grid (if points are scattered)
   f. Create surface record

4. Storage Service Ingestion
   PUT /api/storage/v2/records
   [
     {
       "kind": "{{data-partition-id}}:wks:work-product-component--Surface:1.0.0",
       "data": {
         "Name": "F3-Horizon-Truncation",
         "SurfaceTypeID": "{{data-partition-id}}:reference-data--SurfaceType:Horizon:",
         "Representation": {
           "Format": "CSV",
           "FileSource": "gs://bucket/surfaces/f3-truncation.csv",
           "PointCount": 10000,
           "SpatialExtent": {
             "MinX": 605882,
             "MaxX": 606000,
             "MinY": 6073657,
             "MaxY": 6073700,
             "MinZ": 1060,
             "MaxZ": 1062
           },
           "CoordinateReferenceSystem": "EPSG:32631"
         }
       }
     }
   ]
   → Returns record IDs

5. Indexer Service
   - Indexes surface metadata
   - Extracts searchable fields (Name, SurfaceType, spatial extent)
   - Updates search index
```

**CSV Parser DAG Implementation:**

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

### Pathway 4: GeoTIFF Surface Ingestion

**Architecture:**

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

**Call Stack:**

```
1. GeoTIFF File Upload
   POST /v2/files/upload
   → Returns FileSource identifier

2. GeoTIFF Processing
   a. Read GeoTIFF file from staging
   b. Parse TIFF headers:
      - Image dimensions (width, height)
      - Data type (float, integer)
      - Compression
   c. Parse GeoTIFF tags:
      - Coordinate reference system
      - Geotransform (affine transformation)
      - Model tiepoints
   d. Extract grid data:
      - Read pixel values
      - Apply geotransform to get coordinates
      - Generate grid structure
   e. Calculate spatial extent
   f. Generate statistics (min, max, mean, std)

3. Storage Service Ingestion
   PUT /api/storage/v2/records
   [
     {
       "kind": "{{data-partition-id}}:wks:work-product-component--Surface:1.0.0",
       "data": {
         "Name": "Surface from GeoTIFF",
         "Representation": {
           "Format": "GeoTIFF",
           "FileSource": "gs://bucket/surfaces/surface.tif",
           "GridDimensions": {
             "Width": 1000,
             "Height": 800
           },
           "SpatialExtent": { ... },
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
   → Returns record IDs
```

### Pathway 5: Manifest-Based Ingestion

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
Seismic DDMS / Storage Service
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
             "kind": "{{data-partition-id}}:wks:work-product-component--SeismicHorizon:1.0.0",
             "data": {
               "Name": "Top Reservoir Horizon",
               "SurfaceRepresentation": {
                 "Format": "ZGY",
                 "FileSource": "gs://bucket/horizons/top-reservoir.zgy",
                 "GridDimensions": { ... }
               }
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
   c. For each surface component:
      - Validate surface geometry
      - Validate coordinate reference system
      - Validate file source exists
   d. Resolve all reference data

4. Seismic DDMS Storage (if ZGY/VDS format)
   POST /seistore-svc/api/v3/surveys/{surveyId}/volumes
   { ... }
   → Returns volume ID

5. Storage Service Batch Ingestion
   PUT /api/storage/v2/records
   [
     WorkProduct record,
     Surface WorkProductComponent record,
     Dataset record (if file source provided)
   ]
   → Returns record IDs with surrogate keys resolved

6. Indexer Service
   - Indexes all ingested records
   - Creates searchable metadata
```

## Seismic DDMS Architecture

### Bulk Data Storage

Seismic DDMS provides optimized storage and access for seismic surface bulk data:

**API Endpoints:**

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

**Storage Benefits:**

- **Optimized Access**: Efficient retrieval of large seismic volumes
- **Chunked Storage**: Data organized in chunks for partial reads
- **Compression**: Built-in compression for storage efficiency
- **Spatial Indexing**: Fast queries by inline/crossline ranges
- **Cloud-Native**: Designed for object storage (S3, GCS, Azure Blob)

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

**Surface Type Resolution:**

```
Input: "Horizon" or "Top" or "Base"
    ↓
Reference Data Service Lookup
    GET /api/reference-data/v2/reference-data/{{data-partition-id}}:reference-data--SurfaceType:Horizon:
    ↓
Response:
    {
      "code": "Horizon",
      "name": "Seismic Horizon",
      "description": "Interpreted geological boundary"
    }
    ↓
Resolved Reference:
    "{{data-partition-id}}:reference-data--SurfaceType:Horizon:"
```

### Grid Structure Generation

**Regular Grid Detection:**

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

## Storage Architecture

### Metadata vs. Bulk Data Separation

**Metadata Storage (Storage Service):**

```json
{
	"id": "{{data-partition-id}}:doc:4ff67ce36ae2452b8ddad3391f1fc08a",
	"version": 1582725123640845,
	"kind": "{{data-partition-id}}:wks:work-product-component--SeismicHorizon:1.0.0",
	"data": {
		"Name": "Top Reservoir Horizon",
		"InterpretationProjectID": "...",
		"SeismicGeometryContextID": "...",
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

**Bulk Data Storage (Seismic DDMS):**

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

### File Storage

**Source File Preservation:**

```
File Service (Object Store)
    ↓
Source File Storage:
    gs://osdu-storage/{{data-partition-id}}/files/{{file-id}}/survey.segy
    gs://osdu-storage/{{data-partition-id}}/files/{{file-id}}/surface.csv
    gs://osdu-storage/{{data-partition-id}}/files/{{file-id}}/horizon.tif
    ↓
Dataset Record Reference:
    {
      "kind": "{{data-partition-id}}:wks:dataset--File.Generic:1.0.0",
      "data": {
        "DatasetProperties": {
          "FileSourceInfo": {
            "FileSource": "gs://osdu-storage/{{data-partition-id}}/files/{{file-id}}/survey.segy"
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

## Search & Discovery

### Indexing Process

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

### Query Examples

**Search by Surface Name:**

```http
POST /api/search/v2/query
{
  "kind": "{{data-partition-id}}:wks:work-product-component--Surface:1.0.0",
  "query": "data.Name:\"Top Reservoir\""
}
```

**Search by Spatial Extent:**

```http
POST /api/search/v2/query
{
  "kind": "{{data-partition-id}}:wks:work-product-component--Surface:1.0.0",
  "query": "data.SurfaceRepresentation.SpatialExtent.MinX:[400000 TO 500000] AND data.SurfaceRepresentation.SpatialExtent.MinY:[6500000 TO 7000000]"
}
```

**Search by Format:**

```http
POST /api/search/v2/query
{
  "kind": "{{data-partition-id}}:wks:work-product-component--Surface:1.0.0",
  "query": "data.SurfaceRepresentation.Format:\"ZGY\""
}
```

## Error Handling & Validation

### Validation Errors

**Common Validation Failures:**

1. **Invalid SEGY Format:**

   ```
   Error: Invalid SEGY header format
   Resolution: Verify SEGY file structure and headers
   ```

2. **Missing Coordinate Reference System:**

   ```
   Error: CoordinateReferenceSystem not specified or invalid
   Resolution: Provide valid CRS identifier (EPSG code)
   ```

3. **Invalid Grid Dimensions:**

   ```
   Error: Grid dimensions do not match data size
   Resolution: Verify grid dimensions match actual data
   ```

4. **CSV Column Mismatch:**

   ```
   Error: Required columns X, Y, Z not found in CSV
   Resolution: Ensure CSV contains X, Y, Z columns
   ```

5. **Coordinate Range Validation:**
   ```
   Error: X coordinate 20000000 exceeds maximum range
   Resolution: Validate coordinate ranges before ingestion
   ```

## Performance Considerations

### Batch Ingestion

**Optimal Batch Size:**

- **Small surfaces (< 10K points)**: Process individually
- **Medium surfaces (10K-100K points)**: Batch by surface type
- **Large surfaces (> 100K points)**: Chunk processing recommended

**Recommended:** Process surfaces individually, batch metadata records (100-200 per batch)

### Chunked Processing

**SEGY to ZGY Chunking:**

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

## Summary

OSDU provides **comprehensive, multi-format ingestion** for 3D surface data through specialized services:

1. **Seismic DDMS**: Optimized bulk storage for seismic surfaces (SEGY → ZGY/VDS conversion)
2. **CSV Parser**: Automated ingestion from CSV files with point-based surfaces
3. **GeoTIFF Support**: Raster surface data ingestion
4. **RESQML Support**: Industry-standard reservoir surface formats
5. **Manifest-based**: Structured JSON manifests for complex workflows
6. **Direct API**: Programmatic record creation for custom applications

**Key Design Decisions:**

- **Metadata/Bulk Separation**: Metadata in Storage Service, bulk data in Seismic DDMS
- **Format Conversion**: SEGY → ZGY/VDS for optimized cloud storage
- **Grid-Based Storage**: Efficient storage for regular and irregular grids
- **Source File Preservation**: Original files stored alongside converted formats
- **Search Integration**: Automatic indexing for discovery and spatial queries
- **Access Control**: ACLs and legal tags enforced at ingestion time

**Call Stack Highlights:**

- **SEGY to ZGY**: File upload → DAG trigger → SEGY parsing → ZGY conversion → Seismic DDMS storage → Metadata ingest → Index
- **CSV Parser**: File upload → DAG trigger → CSV parsing → Grid generation → Storage API → Index
- **GeoTIFF**: File upload → GeoTIFF parsing → Grid extraction → Storage API → Index
- **Manifest**: JSON creation → Workflow API → Parse & validate → Seismic DDMS/Storage → Batch ingest → Index

This architecture ensures **data consistency**, **spatial accuracy**, **format optimization**, **traceability**, and **governance** while supporting multiple ingestion formats and surface types.
