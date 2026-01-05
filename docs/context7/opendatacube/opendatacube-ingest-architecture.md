# Open Data Cube Ingestion Architecture (Context7 Summary)

This document summarizes how **Open Data Cube (ODC)** handles domain ingestion configuration,
storage scheme transformation, and product indexing, based on Context7 documentation and ODC guides.

It is particularly relevant to DataForge’s **LAS → Parquet dataset + catalog entry** workflow, where
ingestion maps datasets into a new storage scheme, records/indexes them as products, and writes out
transformed data.

---

## 1. High-Level Architecture

- **ODC Core**:
  - Python library for managing Earth observation data.
  - **PostgreSQL index** for product definitions and dataset metadata.
  - **Storage drivers** (filesystem, S3, etc.) for actual data files.
- **Product definitions** (YAML):
  - Define the **storage scheme** (CRS, resolution, tile size, path templates).
  - Specify **measurements** (bands/variables with dtype, units, nodata).
  - Reference a **metadata type** (schema for dataset properties).
- **Dataset indexing**:
  - Datasets are indexed with **metadata documents** (YAML) that reference a product.
  - The index stores dataset locations, properties, and lineage.
- **Ingestion/transformation**:
  - Can rewrite data into the product’s storage scheme using **write plugins**.
  - Transforms source data (e.g., GeoTIFF) into the configured CRS, resolution, and tile structure.

Conceptually:

- A **product** defines a **storage scheme** and measurement schema.
- **Datasets** are indexed instances of a product, with metadata and storage locations.
- **Ingestion** maps source data into the product’s storage scheme and indexes it.

This mirrors DataForge’s approach:

- **Product** = Well/Curve type definition (storage scheme: Parquet structure, column schema).
- **Dataset** = A specific LAS file → Parquet conversion + catalog entry.
- **Ingestion** = LAS parsing → normalized columns → Parquet + metadata DB rows.

---

## 2. Product Definition: Storage Scheme Configuration

### 2.1 Product Definition Structure

A product definition (YAML) specifies:

```yaml
name: landsat_8_oli_tirs
description: Landsat 8 OLI/TIRS Collection 2 Level-2 data.
metadata_type: datacube

# Storage scheme configuration
storage:
  crs: EPSG:32660
  tile_size:
    x: 100000.0
    y: 100000.0
  resolution:
    longitude: 30.0
    latitude: -30.0
  # Path template (Jinja2) for organizing stored data
  path: "{time.year}/{time.month}/{time.day}"

# Measurement definitions
measurements:
  - name: "blue"
    dtype: "uint16"
    units: "reflectance"
    nodata: 0
  - name: "green"
    dtype: "uint16"
    units: "reflectance"
    nodata: 0
  # ... more measurements
```

Key aspects:

- **`storage`**: Defines the target storage scheme:
  - `crs`: Coordinate Reference System.
  - `tile_size`: Spatial tiling dimensions.
  - `resolution`: Pixel resolution.
  - `path`: Template for organizing files (can use dataset properties like `time`, `platform`).
- **`measurements`**: Schema for data variables (name, dtype, units, nodata).
- **`metadata_type`**: References a metadata schema (e.g., `eo3`, `datacube`) that defines
  searchable properties.

### 2.2 Storage Driver Configuration

For S3-based storage, the product definition can specify a write plugin:

```yaml
storage:
  driver: s3aio
  bucket: my_s3_bucket
  path: "{time.year}/{time.month}/{time.day}"
  crs: EPSG:32660
  resolution:
    longitude: 30.0
    latitude: -30.0
```

The `storage` section is passed to the write plugin, allowing driver-specific configuration.

---

## 3. Dataset Indexing: Metadata Documents

### 3.1 Dataset Metadata Document

A dataset is indexed with a metadata document (YAML) that references a product:

```yaml
id: f884df9b-4458-47fd-a9d2-1a52a2db8a1a
$schema: 'https://schemas.opendatacube.org/dataset'

# Reference to product
product:
  name: landsat8_example_product

# Native CRS and geometry
crs: "epsg:32660"
geometry:
  type: Polygon
  coordinates: [[...]]

# Grid definitions (image size, geo-registration)
grids:
  default:
    shape: [7811, 7691]
    transform: [30, 0, 618285, 0, -30, -1642485, 0, 0, 1]

# Measurement storage locations
measurements:
  red:
    path: red.tif
  blue:
    path: blue.tif
  # Multi-band file example
  multiband_example:
    path: multi_band.tif
    band: 2  # 1-based index

# Dataset location (URI)
location: https://landsatonaws.com/L8/099/072/LC08_L1GT_099072_20200523/metadata.yaml

# Properties (searchable metadata)
properties:
  eo:platform: landsat-8
  eo:instrument: OLI_TIRS
  datetime: 2020-01-01T07:02:54.188Z
  odc:file_format: GeoTIFF
  odc:region_code: "074071"

# Lineage (source datasets)
lineage: {}
```

Key aspects:

- **`product.name`**: Links the dataset to a product definition.
- **`grids`**: Defines spatial grids (size, transform) for measurements.
- **`measurements`**: Maps measurement names to file paths (relative to `location`).
- **`properties`**: Searchable metadata (time, platform, region, etc.).
- **`location`**: Base URI for the dataset (can be a directory or single file).

### 3.2 Indexing Process

Datasets are indexed into the ODC catalog using:

```bash
# Add product definition first
datacube product add product-definition.yaml

# Index datasets (from S3, filesystem, etc.)
s3-to-dc --no-sign-request 's3://bucket/path/**/*.yaml' product_name
```

Or programmatically:

```python
from datacube import Datacube

dc = Datacube(app="ingestion")

# Index a dataset
dc.index.datasets.add(dataset_metadata_doc)
```

The index stores:

- Product definitions (name, storage scheme, measurements).
- Dataset metadata (ID, product reference, properties, location, lineage).
- Spatial/temporal indexes for efficient querying.

---

## 4. Ingestion: Storage Scheme Transformation

### 4.1 Ingestion Configuration

Ingestion can rewrite source data into the product’s storage scheme. An ingestion configuration
(YAML) specifies:

```yaml
# Ingestion configuration
output_product: landsat_8_oli_tirs
source_type: landsat_raw

# Storage configuration (passed to write plugin)
storage:
  driver: s3aio
  bucket: my_s3_bucket
  path: "{time.year}/{time.month}/{time.day}"
  crs: EPSG:32660
  resolution:
    longitude: 30.0
    latitude: -30.0
  tile_size:
    x: 100000.0
    y: 100000.0
```

### 4.2 Ingestion Workflow

1. **Source data discovery**:
   - Find source datasets (e.g., raw Landsat scenes).
   - Match them to an input product or source type.

2. **Transformation**:
   - Load source data (e.g., GeoTIFF files).
   - Reproject to target CRS (from product definition).
   - Resample to target resolution.
   - Tile according to `tile_size`.

3. **Write to storage**:
   - Use the configured **write plugin** (e.g., `s3aio`, filesystem driver).
   - Write transformed data to paths defined by the `storage.path` template.
   - Organize files by dataset properties (time, region, etc.).

4. **Index the transformed dataset**:
   - Create a dataset metadata document referencing the output product.
   - Set `location` to the transformed data location.
   - Populate `properties` (time, platform, region, etc.).
   - Add to the ODC index.

5. **Catalog entry**:
   - The dataset is now queryable via `dc.find_datasets(product="...", time=..., lat=..., lon=...)`.
   - Data can be loaded via `dc.load(product="...", ...)`.

---

## 5. Call Stack: "Source Data → Transformed Storage Scheme + Catalog Entry"

### 5.1 Product Definition

1. **Define product** (YAML):
   - Specify storage scheme (`crs`, `resolution`, `tile_size`, `path`).
   - Define measurements (name, dtype, units, nodata).
   - Reference metadata type.

2. **Add product to index**:
   ```bash
   datacube product add product-definition.yaml
   ```
   - Product definition is stored in PostgreSQL.
   - Validates schema and storage configuration.

### 5.2 Dataset Indexing (Direct)

1. **Prepare dataset metadata document** (YAML):
   - Reference product name.
   - Specify `location` (URI to data files).
   - Define `grids` and `measurements` mappings.
   - Populate `properties` (time, platform, region, etc.).

2. **Index dataset**:
   ```bash
   s3-to-dc 's3://bucket/path/*.yaml' product_name
   ```
   Or:
   ```python
   dc.index.datasets.add(metadata_doc)
   ```

3. **Catalog entry created**:
   - Dataset metadata stored in PostgreSQL.
   - Spatial/temporal indexes updated.
   - Dataset is now queryable.

### 5.3 Ingestion with Transformation

1. **Configure ingestion** (YAML):
   - Specify `output_product`.
   - Define `storage` configuration (driver, bucket, path template, CRS, resolution).

2. **Run ingestion**:
   - Discover source datasets.
   - Load source data (e.g., via `dc.load()` with source product).
   - Transform:
     - Reproject to target CRS.
     - Resample to target resolution.
     - Tile according to `tile_size`.

3. **Write transformed data**:
   - Use write plugin (e.g., `s3aio`) to write to storage.
   - Files organized by `storage.path` template.

4. **Index transformed dataset**:
   - Create dataset metadata document for output product.
   - Set `location` to transformed data.
   - Add to index.

5. **Query and load**:
   ```python
   # Find datasets
   datasets = dc.find_datasets(
       product="landsat_8_oli_tirs",
       time=("2020-01-01", "2020-12-31"),
       lat=(-35.3, -35.1),
       lon=(149.0, 149.2)
   )

   # Load data
   data = dc.load(
       product="landsat_8_oli_tirs",
       time=("2020-01-01", "2020-12-31"),
       lat=(-35.3, -35.1),
       lon=(149.0, 149.2),
       measurements=["red", "green", "blue"]
   )
   ```

---

## 6. Relevance & Patterns for DataForge

From ODC’s ingestion architecture, DataForge can adopt:

### 6.1 Product Definition Pattern

- **Well/Curve type definition** (similar to ODC product):
  - Define storage scheme: Parquet structure, column schema, compression.
  - Specify measurements: curve names, dtypes, units, nodata.
  - Reference metadata schema (well properties, curve metadata).

Example (conceptual):

```yaml
# Well type definition
name: standard_well
description: Standard well with depth and curve data

storage:
  format: parquet
  compression: snappy
  schema:
    depth: float64
    gr: float32
    rhob: float32
    # ... more curves

measurements:
  - name: "gr"
    dtype: "float32"
    units: "API"
    nodata: -999.25
  - name: "rhob"
    dtype: "float32"
    units: "g/cm³"
    nodata: -999.25
```

### 6.2 Dataset Indexing Pattern

- **LAS file → Parquet dataset + catalog entry**:
  - Parse LAS file (depth, curves, metadata).
  - Transform into normalized columns (Parquet).
  - Create dataset metadata document:
    - Reference well/curve type (product).
    - Set `location` to Parquet file path.
    - Populate `properties` (well name, UWI, field, company, date, etc.).
  - Index in SQLite catalog.

### 6.3 Storage Scheme Transformation

- **LAS → Parquet rewrite**:
  - Source: LAS file (proprietary format, variable structure).
  - Target: Parquet (normalized columns, standardized schema).
  - Transformation:
    - Extract depth and curves.
    - Normalize column names and types.
    - Handle missing values (nodata).
    - Write to Parquet with compression.

### 6.4 Catalog/Index Pattern

- **PostgreSQL index → SQLite catalog**:
  - ODC uses PostgreSQL for product definitions and dataset metadata.
  - DataForge can use SQLite for:
    - Well/curve type definitions (products).
    - Dataset metadata (wells, curves, Parquet locations).
    - Spatial/temporal indexes (for querying by location, date, etc.).

### 6.5 Query Pattern

- **ODC query → DataForge query**:
  - ODC: `dc.find_datasets(product="...", time=..., lat=..., lon=...)`.
  - DataForge: Query SQLite catalog for wells/curves by:
    - Well type (product).
    - Time range (date).
    - Location (field, company, UWI).
    - Curve names (measurements).

These patterns align well with DataForge’s goal of **"LAS → Parquet dataset + catalog entry"**,
where ingestion maps source LAS files into a normalized storage scheme (Parquet) and indexes them
for efficient querying and loading.

