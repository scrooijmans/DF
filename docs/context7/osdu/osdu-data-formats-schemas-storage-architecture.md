# OSDU Subsurface Data Formats, Schemas & Storage Architecture

This document explains what different subsurface data formats OSDU caters for ingesting and loading, the strict DB schema definitions used, and the file format storage options for different kinds of data.

## Overview

OSDU supports a **comprehensive range of subsurface data formats** through a **schema-driven architecture** with **strict type definitions** and **optimized storage strategies**:

1. **Schema-Driven Architecture**: All data conforms to strict JSON schemas defined by OSDU
2. **Metadata/Bulk Separation**: Metadata in Storage Service (JSON records), bulk data in DDMS (optimized formats)
3. **Work Product Model**: Hierarchical organization (Work Product → Work Product Component → Dataset)
4. **Multi-Format Ingestion**: Industry-standard formats (LAS, WITSML, SEGY, RESQML, CSV, etc.)
5. **Optimized Storage**: Format conversion for cloud-optimized storage (SEGY → ZGY/VDS)

**Key Principles:**

- **Strict Schema Definitions**: Every record type has a defined schema with typed fields
- **Kind-Based Typing**: Records identified by `kind` attribute (e.g., `osdu:wks:work-product-component--WellLog:1.0.0`)
- **Reference Data Governance**: Fixed, Open, and Local reference data types
- **Source File Preservation**: Original files preserved alongside converted/optimized formats
- **Domain Data Management Services**: Specialized DDMS for bulk data (Seismic, Wellbore, Reservoir)

## OSDU Data Type Categories

OSDU organizes data into **four main categories**:

### 1. Master Data

**Definition**: Slow-changing oil & gas entities that don't have direct file associations.

**Characteristics:**

- **Stability**: Changes infrequently (wells, wellbores, fields, facilities)
- **No File Association**: Represents business entities, not files
- **Reference Relationships**: Referenced by Work Product Components

**Examples:**

- Well
- Wellbore
- Field
- Facility
- Organisation
- InterpretationProject
- SeismicGeometryContext

**Schema Pattern:**

```json
{
	"kind": "{{data-partition-id}}:wks:master-data--Wellbore:1.0.0",
	"data": {
		"FacilityID": "well-001",
		"FacilityTypeID": "{{data-partition-id}}:reference-data--FacilityType:Well:",
		"Name": "Wellbore-001",
		"FacilityOperator": [
			{
				"FacilityOperatorID": "operator-001"
			}
		]
	}
}
```

### 2. Reference Data

**Definition**: Controlled vocabularies and taxonomies for data classification.

**Governance Types:**

- **Fixed**: Pre-determined by OSDU forum, cannot be changed (ensures interoperability)
- **Open**: Agreed by OSDU forum, companies can extend with custom values
- **Local**: OSDU provides schema, companies create their own values (limited interoperability)

**Examples:**

- LogType (Raw, Processed, Interpreted)
- SurfaceType (Horizon, Fault, Unconformity)
- ResourceSecurityClassification (Public, Restricted, Confidential)
- UnitOfMeasure (M, FT, API, G/CC)
- HorizonType (Top, Base, Marker)

**Schema Pattern:**

```json
{
	"kind": "{{data-partition-id}}:reference-data--LogType:Raw:",
	"data": {
		"Code": "Raw",
		"Name": "Raw Log",
		"Description": "Unprocessed well log data"
	}
}
```

### 3. Work Product Components (WPC)

**Definition**: Metadata associated with specific files or file collections, representing changing data over time.

**Characteristics:**

- **Versioned**: Can have multiple versions (each version is a snapshot)
- **File Association**: Must reference one or more Dataset records
- **Time-Varying**: Data changes over time (logs, surfaces, interpretations)

**Examples:**

- WellLog
- WellboreMarker (Well Tops)
- Trajectory
- SeismicHorizon
- Surface
- GridProperty
- SeismicVolume

**Schema Pattern:**

```json
{
	"kind": "{{data-partition-id}}:wks:work-product-component--WellLog:1.0.0",
	"data": {
		"WellboreId": "{{data-partition-id}}:master-data--Wellbore:001:",
		"WellLogTypeID": "{{data-partition-id}}:reference-data--LogType:Raw:",
		"TopMeasuredDepth": 0.0,
		"BottomMeasuredDepth": 5000.0,
		"Datasets": ["{{data-partition-id}}:dataset--File.Generic:abc123:"]
	}
}
```

### 4. Datasets

**Definition**: Metadata describing physical files (not the logical entity content).

**Characteristics:**

- **File Metadata**: Describes file properties (size, format, location)
- **No Business Content**: Does not describe the logical entity content
- **File Reference**: Links to physical file location for download

**Types:**

- `File.Generic`: Generic file of any type
- `File.LAS`: LAS file specific metadata
- `File.SEGY`: SEGY file specific metadata
- `FileCollection.Generic`: Collection of files

**Schema Pattern:**

```json
{
	"kind": "{{data-partition-id}}:wks:dataset--File.Generic:1.0.0",
	"data": {
		"DatasetProperties": {
			"FileSourceInfo": {
				"FileSource": "gs://bucket/path/to/file.las",
				"Name": "well_log_001.las"
			}
		},
		"TotalSize": "13245217",
		"Source": "Data Source"
	}
}
```

## Supported Subsurface Data Formats

### Well Data Formats

#### 1. LAS (Log ASCII Standard)

**Ingestion Format:**

- **File Extension**: `.las`
- **Structure**: ASCII text with sections (~Version, ~Well, ~Curve, ~Parameter, ~ASCII)
- **Content**: Well log curves (depth-indexed measurements)

**Storage Format:**

- **Metadata**: WellLog Work Product Component (log-level metadata)
- **Bulk Data**: Original LAS file preserved in object store
- **Optional**: Wellbore DDMS for optimized bulk access

**Schema:**

```json
{
	"kind": "{{data-partition-id}}:wks:work-product-component--WellLog:1.0.0",
	"schema": [
		{
			"path": "data.WellboreId",
			"kind": "core:String:1.0.0"
		},
		{
			"path": "data.WellLogTypeID",
			"kind": "core:String:1.0.0"
		},
		{
			"path": "data.TopMeasuredDepth",
			"kind": "core:Double:1.0.0"
		},
		{
			"path": "data.BottomMeasuredDepth",
			"kind": "core:Double:1.0.0"
		},
		{
			"path": "data.Datasets",
			"kind": "core:Array:1.0.0"
		}
	]
}
```

**Storage Options:**

- **Object Store**: Original LAS file (preserved)
- **Wellbore DDMS**: Optional optimized storage for bulk curve access

#### 2. DLIS (Digital Log Interchange Standard)

**Ingestion Format:**

- **File Extension**: `.dlis`
- **Structure**: Binary format with logical file structure
- **Content**: Well log curves (similar to LAS but binary)

**Storage Format:**

- **Metadata**: WellLog Work Product Component
- **Bulk Data**: Original DLIS file preserved
- **Optional**: Wellbore DDMS for optimized access

**Schema:** Same as LAS (WellLog schema)

#### 3. WITSML (Wellsite Information Transfer Standard Markup Language)

**Ingestion Format:**

- **File Extension**: `.xml` (WITSML format)
- **Structure**: XML-based industry standard
- **Content**: Well data (logs, trajectories, mud logs, etc.)

**Storage Format:**

- **Metadata**: Various WPC types (WellLog, Trajectory, etc.)
- **Bulk Data**: Original WITSML file preserved
- **Conversion**: WITSML Parser DAG converts to OSDU R3 schemas

**Schema Examples:**

**WellLog from WITSML:**

```json
{
	"kind": "{{data-partition-id}}:wks:work-product-component--WellLog:1.0.0",
	"data": {
		"WellboreId": "...",
		"WellLogTypeID": "...",
		"Datasets": ["..."]
	}
}
```

**Trajectory from WITSML:**

```json
{
  "kind": "{{data-partition-id}}:wks:work-product-component--Trajectory:1.0.0",
  "data": {
    "WellboreId": "...",
    "TrajectoryCRS": "EPSG:32631",
    "Stations": [...]
  }
}
```

#### 4. CSV (Comma-Separated Values)

**Ingestion Format:**

- **File Extension**: `.csv`
- **Structure**: Tabular data with headers
- **Content**: Various well data (tops, trajectories, checkshots, etc.)

**Storage Format:**

- **Metadata**: Various WPC types depending on content
- **Bulk Data**: Original CSV file preserved
- **Conversion**: CSV Parser DAG creates flattened schemas

**CSV Types Supported:**

- Well Tops/Markers
- Trajectories
- Checkshots
- Wells metadata
- Surfaces (3D points)

**Schema:** Schema created dynamically by CSV Parser (flattened structure)

### Seismic Data Formats

#### 1. SEGY (SEG Y)

**Ingestion Format:**

- **File Extension**: `.segy`, `.sgy`
- **Structure**:
  - Text Header (3200 bytes EBCDIC)
  - Binary Header (400 bytes)
  - Trace Headers (240 bytes per trace)
  - Trace Data (amplitude samples)

**Storage Format:**

- **Metadata**: SeismicVolume Work Product Component
- **Bulk Data**: Converted to ZGY or VDS (cloud-optimized)
- **Original**: SEGY file preserved in object store
- **Seismic DDMS**: Optimized bulk storage

**Schema:**

```json
{
	"kind": "{{data-partition-id}}:wks:work-product-component--SeismicVolume:1.0.0",
	"schema": [
		{
			"path": "data.SurveyID",
			"kind": "core:String:1.0.0"
		},
		{
			"path": "data.VolumeID",
			"kind": "core:String:1.0.0"
		},
		{
			"path": "data.Format",
			"kind": "core:String:1.0.0"
		},
		{
			"path": "data.Geometry.InlineCount",
			"kind": "core:Integer:1.0.0"
		},
		{
			"path": "data.Geometry.CrosslineCount",
			"kind": "core:Integer:1.0.0"
		},
		{
			"path": "data.Geometry.SampleCount",
			"kind": "core:Integer:1.0.0"
		},
		{
			"path": "data.CoordinateReferenceSystem",
			"kind": "core:String:1.0.0"
		}
	]
}
```

**Storage Options:**

- **Object Store**: Original SEGY file (preserved)
- **Seismic DDMS**: ZGY or VDS format (optimized for cloud storage)

#### 2. ZGY (OpenZGY)

**Ingestion Format:**

- **File Extension**: `.zgy`
- **Structure**: Cloud-optimized format with chunked storage
- **Content**: Seismic trace data (converted from SEGY)

**Storage Format:**

- **Metadata**: SeismicVolume Work Product Component
- **Bulk Data**: ZGY file in Seismic DDMS
- **Optimization**: Chunked storage for efficient access

**Schema:** Same as SEGY (SeismicVolume schema)

**Storage Options:**

- **Seismic DDMS**: Native ZGY format (optimized)

#### 3. VDS (Virtual Data Store)

**Ingestion Format:**

- **File Extension**: `.vds`
- **Structure**: Virtual data organization format
- **Content**: Seismic trace data (converted from SEGY)

**Storage Format:**

- **Metadata**: SeismicVolume Work Product Component
- **Bulk Data**: VDS file in Seismic DDMS
- **Optimization**: Virtualized logical view

**Schema:** Same as SEGY (SeismicVolume schema)

**Storage Options:**

- **Seismic DDMS**: Native VDS format (optimized)

#### 4. Seismic Bin Grid

**Ingestion Format:**

- **Formats**: P6/11, CSV
- **Content**: Surface positions for subsurface nodes

**Storage Format:**

- **Metadata**: SeismicBinGrid Work Product Component
- **Bulk Data**: Original file or converted format

**Schema:**

```json
{
	"kind": "{{data-partition-id}}:wks:work-product-component--SeismicBinGrid:1.0.0",
	"data": {
		"SurveyID": "...",
		"GridDimensions": {
			"InlineCount": 1000,
			"CrosslineCount": 500
		},
		"CoordinateReferenceSystem": "EPSG:32631"
	}
}
```

#### 5. Seismic Line Geometry

**Ingestion Format:**

- **Formats**: P1/11, P1/P90, CSV
- **Content**: 2D processing geometry of seismic lines

**Storage Format:**

- **Metadata**: SeismicLineGeometry Work Product Component
- **Bulk Data**: Original file preserved

### Surface Data Formats

#### 1. CSV (Point-Based Surfaces)

**Ingestion Format:**

- **File Extension**: `.csv`
- **Structure**: X, Y, Z columns
- **Content**: Point-based geological surfaces

**Storage Format:**

- **Metadata**: Surface Work Product Component
- **Bulk Data**: Original CSV file preserved
- **Optional**: Grid structure generation

**Schema:**

```json
{
	"kind": "{{data-partition-id}}:wks:work-product-component--Surface:1.0.0",
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
			"path": "data.Representation.PointCount",
			"kind": "core:Integer:1.0.0"
		},
		{
			"path": "data.Representation.SpatialExtent.MinX",
			"kind": "core:Double:1.0.0"
		},
		{
			"path": "data.Representation.CoordinateReferenceSystem",
			"kind": "core:String:1.0.0"
		}
	]
}
```

#### 2. GeoTIFF

**Ingestion Format:**

- **File Extension**: `.tif`, `.tiff`
- **Structure**: TIFF with GeoTIFF tags
- **Content**: Raster surface data

**Storage Format:**

- **Metadata**: Surface Work Product Component
- **Bulk Data**: Original GeoTIFF file preserved
- **Grid Extraction**: Grid structure extracted from GeoTIFF tags

**Schema:** Same as CSV Surface schema

#### 3. RESQML (Reservoir Characterization Markup Language)

**Ingestion Format:**

- **File Extension**: `.xml` (RESQML format)
- **Structure**: XML-based industry standard
- **Content**: Reservoir models, surfaces, grids, properties

**Storage Format:**

- **Metadata**: Various WPC types (Surface, GridProperty, etc.)
- **Bulk Data**: Original RESQML file preserved
- **Reservoir DDMS**: Specialized storage for reservoir data

**Schema Examples:**

**Surface from RESQML:**

```json
{
	"kind": "{{data-partition-id}}:wks:work-product-component--Surface:1.0.0",
	"data": {
		"Name": "...",
		"SurfaceTypeID": "...",
		"Representation": {
			"Format": "RESQML",
			"FileSource": "..."
		}
	}
}
```

**GridProperty from RESQML:**

```json
{
	"kind": "{{data-partition-id}}:wks:work-product-component--GridProperty:1.0.0",
	"data": {
		"Name": "Porosity Grid",
		"PropertyTypeID": "{{data-partition-id}}:reference-data--PropertyType:Porosity:",
		"GridID": "...",
		"Representation": {
			"Format": "RESQML",
			"GridDimensions": {
				"I": 200,
				"J": 150,
				"K": 50
			}
		}
	}
}
```

### Trajectory Data Formats

#### 1. CSV

**Ingestion Format:**

- **File Extension**: `.csv`
- **Structure**: X, Y, TVD, MD columns (or MD, Inclination, Azimuth)
- **Content**: Well trajectory stations

**Storage Format:**

- **Metadata**: Trajectory Work Product Component
- **Bulk Data**: Wellbore DDMS (optimized station storage)
- **Original**: CSV file preserved

**Schema:**

```json
{
	"kind": "{{data-partition-id}}:wks:work-product-component--Trajectory:1.0.0",
	"schema": [
		{
			"path": "data.WellboreID",
			"kind": "core:String:1.0.0"
		},
		{
			"path": "data.TrajectoryCRS",
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
		}
	]
}
```

**Storage Options:**

- **Object Store**: Original CSV file (preserved)
- **Wellbore DDMS**: Optimized station array storage

#### 2. WITSML

**Ingestion Format:**

- **File Extension**: `.xml` (WITSML format)
- **Structure**: XML with trajectoryStation elements
- **Content**: Well trajectory with MD, inclination, azimuth

**Storage Format:**

- **Metadata**: Trajectory Work Product Component
- **Bulk Data**: Wellbore DDMS (after CRS conversion)
- **Original**: WITSML file preserved

**Schema:** Same as CSV Trajectory schema

## Database Schema Architecture

### Storage Service Schema Structure

OSDU uses a **strict schema definition system** where each record `kind` has an associated schema:

**Schema Definition Format:**

```json
{
	"kind": "{{data-partition-id}}:wks:work-product-component--WellLog:1.0.0",
	"schema": [
		{
			"path": "data.WellboreId",
			"kind": "core:String:1.0.0"
		},
		{
			"path": "data.TopMeasuredDepth",
			"kind": "core:Double:1.0.0"
		},
		{
			"path": "data.BottomMeasuredDepth",
			"kind": "core:Double:1.0.0"
		},
		{
			"path": "data.Datasets",
			"kind": "core:Array:1.0.0"
		}
	]
}
```

**Supported Schema Data Types:**

- `core:String:1.0.0` - String values
- `core:Integer:1.0.0` - Integer numbers
- `core:Long:1.0.0` - Long integers
- `core:Float:1.0.0` - Floating-point numbers
- `core:Double:1.0.0` - Double-precision floating-point
- `core:Boolean:1.0.0` - Boolean values
- `core:Array:1.0.0` - Array of values
- `core:Object:1.0.0` - Nested object
- `core:dl:geopoint:1.0.0` - Geographic point (latitude/longitude)
- `core:dl:date:1.0.0` - Date values
- `core:dl:datetime:1.0.0` - DateTime values

**Schema API:**

```http
# Create Schema
POST /api/storage/v2/schemas
{
  "kind": "{{data-partition-id}}:wks:work-product-component--WellLog:1.0.0",
  "schema": [
    {
      "path": "data.WellboreId",
      "kind": "core:String:1.0.0"
    }
  ]
}

# Get Schema
GET /api/storage/v2/schemas/{{data-partition-id}}:wks:work-product-component--WellLog:1.0.0
```

### Record Structure

**Complete Record Format:**

```json
{
	"id": "{{data-partition-id}}:doc:4ff67ce36ae2452b8ddad3391f1fc08a",
	"version": 1582725123640845,
	"kind": "{{data-partition-id}}:wks:work-product-component--WellLog:1.0.0",
	"acl": {
		"owners": ["data.default.owners@..."],
		"viewers": ["data.default.viewers@..."]
	},
	"legal": {
		"legaltags": ["{{data-partition-id}}-demo-legaltag"],
		"otherRelevantDataCountries": ["US"],
		"status": "compliant"
	},
	"data": {
		"WellboreId": "{{data-partition-id}}:master-data--Wellbore:001:",
		"WellLogTypeID": "{{data-partition-id}}:reference-data--LogType:Raw:",
		"TopMeasuredDepth": 0.0,
		"BottomMeasuredDepth": 5000.0,
		"Datasets": ["{{data-partition-id}}:dataset--File.Generic:abc123:"]
	},
	"createTime": "2024-01-15T10:30:00Z",
	"createUser": "user@example.com",
	"modifyTime": "2024-01-15T10:30:00Z",
	"modifyUser": "user@example.com"
}
```

**Record Fields:**

- **id**: Unique record identifier (format: `{{data-partition-id}}:doc:{{uuid}}`)
- **version**: Version number (monotonically increasing)
- **kind**: Record type identifier (format: `{{data-partition-id}}:{{namespace}}:{{type}}:{{version}}`)
- **acl**: Access Control List (owners, viewers)
- **legal**: Legal tags and compliance information
- **data**: Actual data payload (conforms to schema)
- **createTime/modifyTime**: Timestamps
- **createUser/modifyUser**: User identifiers

### Kind Naming Convention

**Format:** `{{data-partition-id}}:{{namespace}}:{{type}}:{{version}}`

**Examples:**

- `osdu:wks:work-product-component--WellLog:1.0.0`
- `osdu:wks:master-data--Wellbore:1.0.0`
- `osdu:wks:reference-data--LogType:Raw:`
- `osdu:wks:dataset--File.Generic:1.0.0`

**Components:**

- **data-partition-id**: Data partition identifier (e.g., `osdu`, `common`)
- **namespace**: Namespace (e.g., `wks` for work products)
- **type**: Record type (e.g., `work-product-component--WellLog`)
- **version**: Schema version (e.g., `1.0.0`)

## Storage Architecture

### Storage Service (Metadata Storage)

**Purpose**: Stores metadata records (JSON documents)

**Storage Backend:**

- **Database**: Document database (MongoDB, Cosmos DB, Firestore, etc.)
- **Format**: JSON documents
- **Indexing**: Full-text and field-based indexing via Indexer Service

**Record Storage:**

```
Storage Service Database
    ↓
Collection: records
    ↓
Document:
    {
      "_id": "osdu:doc:4ff67ce36ae2452b8ddad3391f1fc08a",
      "version": 1582725123640845,
      "kind": "osdu:wks:work-product-component--WellLog:1.0.0",
      "data": { ... },
      "acl": { ... },
      "legal": { ... }
    }
    ↓
Indexed Fields:
    - id (primary key)
    - kind (indexed)
    - version (indexed)
    - data.* (indexed per schema definition)
    - createTime (indexed)
```

**Schema Enforcement:**

- Schemas must be created before ingesting records
- Only fields with schema definitions are indexed
- Schema validation occurs at ingestion time

### Domain Data Management Services (DDMS) - Bulk Data Storage

**Purpose**: Optimized storage for bulk data (not metadata)

#### Seismic DDMS

**Storage Format:**

- **ZGY**: Chunked, compressed format
- **VDS**: Virtual data store format
- **Original SEGY**: Preserved in object store

**Storage Backend:**

- **Object Store**: S3, GCS, Azure Blob
- **Format**: Binary (ZGY/VDS) or original SEGY
- **Organization**: Survey → Volume → Chunks

**Storage Structure:**

```
Seismic DDMS Backend
    ↓
Survey: survey-001
    ↓
Volume: volume-001
    ↓
Format: ZGY
    ↓
Chunks:
    - chunk-001 (inline 0-100, crossline 0-100)
    - chunk-002 (inline 0-100, crossline 100-200)
    - ...
    ↓
Metadata:
    - Geometry (inline/crossline/sample counts)
    - Coordinate Reference System
    - Sample rate
    - Data statistics
```

#### Wellbore DDMS

**Storage Format:**

- **Trajectories**: Station arrays (JSON or binary)
- **Logs**: Optimized curve storage (Parquet, ZGY, or original LAS)
- **Checkshots**: Station arrays

**Storage Backend:**

- **Object Store**: S3, GCS, Azure Blob
- **Format**: JSON, Parquet, or original formats
- **Organization**: Wellbore → Data Type → Files

**Storage Structure:**

```
Wellbore DDMS Backend
    ↓
Wellbore: wellbore-001
    ↓
Trajectories:
    - trajectory-001.json (station array)
    ↓
Logs:
    - log-001.parquet (optimized curve data)
    - log-001.las (original file)
    ↓
Checkshots:
    - checkshot-001.json (station array)
```

#### Reservoir DDMS

**Storage Format:**

- **Grids**: Structured/unstructured grid data
- **Properties**: Property arrays (ZGY, Parquet, or RESQML)
- **Surfaces**: Grid-based or point-based surfaces

**Storage Backend:**

- **Object Store**: S3, GCS, Azure Blob
- **Format**: RESQML, ZGY, Parquet, or original formats

### File Service (Generic File Storage)

**Purpose**: Stores physical files in object storage

**Storage Backend:**

- **Object Store**: S3, GCS, Azure Blob
- **Format**: Original file format (preserved as-is)
- **Organization**: Partition → Files → File ID

**Storage Structure:**

```
File Service Object Store
    ↓
Bucket: osdu-storage-{{data-partition-id}}
    ↓
Path: files/{{file-id}}/{{filename}}
    ↓
File: well_log_001.las
    - Original format preserved
    - Metadata in Storage Service
    - Downloadable via pre-signed URLs
```

**File Metadata (Dataset Record):**

```json
{
	"kind": "{{data-partition-id}}:wks:dataset--File.Generic:1.0.0",
	"data": {
		"DatasetProperties": {
			"FileSourceInfo": {
				"FileSource": "gs://bucket/files/{{file-id}}/well_log_001.las",
				"Name": "well_log_001.las"
			}
		},
		"TotalSize": "13245217",
		"Source": "Data Source"
	}
}
```

## Data Format Mapping Matrix

| Data Type            | Ingestion Formats      | Storage Format (Metadata) | Storage Format (Bulk Data)                | DDMS                      |
| -------------------- | ---------------------- | ------------------------- | ----------------------------------------- | ------------------------- |
| **Well Logs**        | LAS, DLIS, WITSML      | WellLog WPC (JSON)        | Original file (LAS/DLIS) or Wellbore DDMS | Wellbore DDMS (optional)  |
| **Well Tops**        | CSV, WITSML, JSON      | WellboreMarker WPC (JSON) | Original file (CSV)                       | N/A                       |
| **Trajectories**     | CSV, WITSML, JSON      | Trajectory WPC (JSON)     | Wellbore DDMS (station arrays)            | Wellbore DDMS             |
| **Seismic Volumes**  | SEGY                   | SeismicVolume WPC (JSON)  | ZGY or VDS (converted)                    | Seismic DDMS              |
| **Seismic Horizons** | ZGY, VDS, CSV, GeoTIFF | SeismicHorizon WPC (JSON) | ZGY/VDS or original file                  | Seismic DDMS (if ZGY/VDS) |
| **Surfaces**         | CSV, GeoTIFF, RESQML   | Surface WPC (JSON)        | Original file or Reservoir DDMS           | Reservoir DDMS (optional) |
| **Grid Properties**  | RESQML, ZGY            | GridProperty WPC (JSON)   | RESQML or ZGY                             | Reservoir DDMS            |
| **Checkshots**       | CSV, WITSML            | Checkshot WPC (JSON)      | Wellbore DDMS (station arrays)            | Wellbore DDMS             |

## Schema Definition Examples

### WellLog Schema

```json
{
	"kind": "{{data-partition-id}}:wks:work-product-component--WellLog:1.0.0",
	"schema": [
		{
			"path": "data.WellboreId",
			"kind": "core:String:1.0.0"
		},
		{
			"path": "data.WellLogTypeID",
			"kind": "core:String:1.0.0"
		},
		{
			"path": "data.TopMeasuredDepth",
			"kind": "core:Double:1.0.0"
		},
		{
			"path": "data.BottomMeasuredDepth",
			"kind": "core:Double:1.0.0"
		},
		{
			"path": "data.LoggingService",
			"kind": "core:String:1.0.0"
		},
		{
			"path": "data.ActivityType",
			"kind": "core:String:1.0.0"
		},
		{
			"path": "data.ServiceCompanyId",
			"kind": "core:String:1.0.0"
		},
		{
			"path": "data.Datasets",
			"kind": "core:Array:1.0.0"
		}
	]
}
```

### WellboreMarker Schema

```json
{
	"kind": "{{data-partition-id}}:wks:work-product-component--WellboreMarker:1.0.0",
	"schema": [
		{
			"path": "data.Data.IndividualTypeProperties.WellboreID",
			"kind": "core:String:1.0.0"
		},
		{
			"path": "data.Data.IndividualTypeProperties.DepthUnit",
			"kind": "core:String:1.0.0"
		},
		{
			"path": "data.Data.IndividualTypeProperties.Markers",
			"kind": "core:Array:1.0.0"
		},
		{
			"path": "data.Data.IndividualTypeProperties.Markers[].MarkerName",
			"kind": "core:String:1.0.0"
		},
		{
			"path": "data.Data.IndividualTypeProperties.Markers[].MarkerMeasuredDepth",
			"kind": "core:Double:1.0.0"
		}
	]
}
```

### Trajectory Schema

```json
{
	"kind": "{{data-partition-id}}:wks:work-product-component--Trajectory:1.0.0",
	"schema": [
		{
			"path": "data.WellboreID",
			"kind": "core:String:1.0.0"
		},
		{
			"path": "data.TrajectoryCRS",
			"kind": "core:String:1.0.0"
		},
		{
			"path": "data.TrajectoryID",
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
		}
	]
}
```

### SeismicVolume Schema

```json
{
	"kind": "{{data-partition-id}}:wks:work-product-component--SeismicVolume:1.0.0",
	"schema": [
		{
			"path": "data.SurveyID",
			"kind": "core:String:1.0.0"
		},
		{
			"path": "data.VolumeID",
			"kind": "core:String:1.0.0"
		},
		{
			"path": "data.Format",
			"kind": "core:String:1.0.0"
		},
		{
			"path": "data.Geometry.InlineCount",
			"kind": "core:Integer:1.0.0"
		},
		{
			"path": "data.Geometry.CrosslineCount",
			"kind": "core:Integer:1.0.0"
		},
		{
			"path": "data.Geometry.SampleCount",
			"kind": "core:Integer:1.0.0"
		},
		{
			"path": "data.Geometry.SampleRate",
			"kind": "core:Double:1.0.0"
		},
		{
			"path": "data.CoordinateReferenceSystem",
			"kind": "core:String:1.0.0"
		},
		{
			"path": "data.FileSource",
			"kind": "core:String:1.0.0"
		}
	]
}
```

### Surface Schema

```json
{
	"kind": "{{data-partition-id}}:wks:work-product-component--Surface:1.0.0",
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
			"path": "data.Representation.CoordinateReferenceSystem",
			"kind": "core:String:1.0.0"
		}
	]
}
```

## File Format Storage Options

### Original Format Preservation

**Principle**: OSDU preserves original files in their native format for:

- **Data Lineage**: Traceability to source
- **Compatibility**: Support for legacy tools
- **Audit**: Original data for verification

**Storage Location**: File Service object store

**Examples:**

- LAS files stored as `.las`
- SEGY files stored as `.segy`
- CSV files stored as `.csv`
- WITSML files stored as `.xml`
- RESQML files stored as `.xml`

### Optimized Format Conversion

**Principle**: OSDU converts to cloud-optimized formats for:

- **Performance**: Faster access and querying
- **Storage Efficiency**: Compression and chunking
- **Scalability**: Support for large datasets

**Conversion Examples:**

1. **SEGY → ZGY**
   - **Reason**: Cloud-optimized chunked storage
   - **Benefits**: Efficient partial reads, compression
   - **Storage**: Seismic DDMS

2. **SEGY → VDS**
   - **Reason**: Virtual data organization
   - **Benefits**: Logical view without physical reorganization
   - **Storage**: Seismic DDMS

3. **LAS → Wellbore DDMS**
   - **Reason**: Optimized curve access
   - **Benefits**: Fast queries, efficient storage
   - **Storage**: Wellbore DDMS (optional)

4. **CSV → Parquet**
   - **Reason**: Columnar storage efficiency
   - **Benefits**: Compression, fast queries
   - **Storage**: DDMS or object store

### Storage Format Decision Matrix

| Data Type                    | Original Format     | Optimized Format             | Storage Location              |
| ---------------------------- | ------------------- | ---------------------------- | ----------------------------- |
| **Well Logs (LAS)**          | `.las` (preserved)  | Wellbore DDMS (optional)     | File Service / Wellbore DDMS  |
| **Well Logs (DLIS)**         | `.dlis` (preserved) | Wellbore DDMS (optional)     | File Service / Wellbore DDMS  |
| **Seismic (SEGY)**           | `.segy` (preserved) | `.zgy` or `.vds` (converted) | File Service / Seismic DDMS   |
| **Seismic (ZGY)**            | `.zgy` (native)     | `.zgy` (native)              | Seismic DDMS                  |
| **Seismic (VDS)**            | `.vds` (native)     | `.vds` (native)              | Seismic DDMS                  |
| **Trajectories**             | `.csv` (preserved)  | Wellbore DDMS (JSON/Parquet) | File Service / Wellbore DDMS  |
| **Well Tops**                | `.csv` (preserved)  | N/A                          | File Service                  |
| **Surfaces (CSV)**           | `.csv` (preserved)  | Reservoir DDMS (optional)    | File Service / Reservoir DDMS |
| **Surfaces (GeoTIFF)**       | `.tif` (preserved)  | Reservoir DDMS (optional)    | File Service / Reservoir DDMS |
| **Grid Properties (RESQML)** | `.xml` (preserved)  | Reservoir DDMS (ZGY/Parquet) | File Service / Reservoir DDMS |

## Ingestion Workflow Schema Enforcement

### Schema Creation Workflow

```
1. Define Schema
   POST /api/storage/v2/schemas
   {
     "kind": "{{data-partition-id}}:wks:work-product-component--WellLog:1.0.0",
     "schema": [ ... ]
   }
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
   {
     "kind": "{{data-partition-id}}:wks:work-product-component--WellLog:1.0.0",
     "data": { ... }
   }
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

### Schema Validation Rules

**Required Field Validation:**

- Fields marked as required in schema must be present
- Missing required fields cause ingestion failure

**Data Type Validation:**

- String fields must be strings
- Numeric fields must be numbers
- Array fields must be arrays
- Object fields must be objects

**Reference Validation:**

- Master data references must exist
- Reference data values must be valid
- Dataset references must exist

**Example Validation Error:**

```json
{
	"error": "Schema validation failed",
	"kind": "osdu:wks:work-product-component--WellLog:1.0.0",
	"errors": [
		{
			"path": "data.WellboreId",
			"message": "Required field missing"
		},
		{
			"path": "data.TopMeasuredDepth",
			"message": "Expected Double, got String"
		}
	]
}
```

## Summary

OSDU provides **comprehensive support for subsurface data formats** through:

### Supported Ingestion Formats

1. **Well Data**: LAS, DLIS, WITSML, CSV
2. **Seismic Data**: SEGY, ZGY, VDS, Seismic Bin Grid (P6/11, CSV), Seismic Line Geometry (P1/11, P1/P90, CSV)
3. **Surface Data**: CSV, GeoTIFF, RESQML
4. **Trajectory Data**: CSV, WITSML, JSON
5. **Reservoir Data**: RESQML, ZGY, Parquet

### Storage Architecture

1. **Storage Service**: JSON document database for metadata records
2. **Seismic DDMS**: Optimized bulk storage (ZGY, VDS formats)
3. **Wellbore DDMS**: Optimized bulk storage (trajectories, logs, checkshots)
4. **Reservoir DDMS**: Optimized bulk storage (grids, properties, surfaces)
5. **File Service**: Object store for original file preservation

### Schema System

1. **Strict Typing**: Every record kind has a defined schema
2. **Path-Based Fields**: Schema fields defined by JSON path
3. **Type System**: Core data types (String, Integer, Double, Array, Object, etc.)
4. **Schema API**: Create and retrieve schemas via REST API
5. **Indexing**: Only schema-defined fields are indexed for search

### File Format Strategy

1. **Original Preservation**: All original files preserved in object store
2. **Format Conversion**: SEGY → ZGY/VDS for cloud optimization
3. **DDMS Optimization**: Bulk data stored in optimized formats when beneficial
4. **Dual Storage**: Both original and optimized formats available

This architecture ensures **data consistency**, **format flexibility**, **storage optimization**, and **interoperability** across diverse subsurface data types and formats.
