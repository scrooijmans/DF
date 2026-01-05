# Apache Superset: Units of Truth, Storage, References, and Versioning

## Executive Summary

In Apache Superset, **Datasets** serve as the foundational unit of truth for data definitions. **Charts** (also called "slices") reference datasets and define visualization configurations. **Dashboards** reference charts and define layout and presentation. All three object types are stored in Superset's metadata database using SQLAlchemy ORM models, with relationships maintained through foreign keys and JSON structures.

---

## 1. The Unit of Truth: Datasets

### 1.1 What is a Dataset?

A **Dataset** (historically called a "datasource" or "table") is the primary unit of truth in Superset. It represents either:

- A physical database table/view
- A virtual dataset defined by a SQL query

Datasets encapsulate:

- **Schema information**: Database, schema, and table name
- **Column definitions**: Metadata about each column (name, type, filterable, groupable)
- **Metrics**: Pre-defined SQL aggregations (SUM, COUNT, AVG, etc.)
- **Configuration**: Cache timeouts, descriptions, ownership

### 1.2 Dataset Storage

Datasets are stored in the metadata database using the `Dataset` SQLAlchemy model:

```python
class Dataset(Model, AuditMixinNullable):
    __tablename__ = "tables"  # Note: table name is "tables" not "datasets"

    id = Column(Integer, primary_key=True)
    table_name = Column(String(250))
    schema = Column(String(255))
    database_id = Column(Integer, ForeignKey('dbs.id'))
    # ... other fields
```

**Key Storage Characteristics:**

- **Primary Key**: Integer `id` field
- **UUID**: Each dataset has a unique UUID for external references
- **Foreign Keys**: References `dbs` table for database connection
- **Audit Fields**: Inherits from `AuditMixinNullable` (created_on, changed_on, created_by_fk, changed_by_fk)

### 1.3 Dataset Components

#### Columns (TableColumn)

Columns are stored in the `table_columns` table with a foreign key to the dataset:

```python
class TableColumn(Model, AuditMixinNullable):
    __tablename__ = "table_columns"

    id = Column(Integer, primary_key=True)
    dataset_id = Column(Integer, ForeignKey('tables.id'))
    column_name = Column(String(255))
    type = Column(String(32))
    filterable = Column(Boolean)
    groupby = Column(Boolean)
    is_active = Column(Boolean)
```

**Relationship**: One Dataset → Many TableColumns

#### Metrics (SqlMetric)

Metrics are stored in the `sql_metrics` table:

```python
class SqlMetric(Model, AuditMixinNullable):
    __tablename__ = "sql_metrics"

    id = Column(Integer, primary_key=True)
    dataset_id = Column(Integer, ForeignKey('tables.id'))
    metric_name = Column(String(255))
    expression = Column(Text)  # SQL expression like "SUM(amount)"
    metric_type = Column(String(32))
```

**Relationship**: One Dataset → Many SqlMetrics

### 1.4 Dataset Versioning

Datasets do **not** have explicit versioning. Instead:

- **Schema Refresh**: Datasets can refresh their schema to sync with database changes via `/api/v1/dataset/{id}/refresh`
- **Audit Trail**: `AuditMixinNullable` tracks creation and modification timestamps
- **Change Tracking**: The `changed_on` and `changed_by_fk` fields record who modified the dataset and when

---

## 2. Charts (Slices)

### 2.1 What is a Chart?

A **Chart** (internally called a "slice") is a visualization definition that:

- References a dataset (via `datasource_id`)
- Defines visualization type (`viz_type`)
- Stores query configuration (`params`/`formData` as JSON)
- Contains rendering metadata

### 2.2 Chart Storage

Charts are stored in the `slices` table:

```python
class Slice(Model, AuditMixinNullable):
    __tablename__ = "slices"

    id = Column(Integer, primary_key=True)
    slice_name = Column(String(250))
    datasource_id = Column(Integer)  # References dataset.id
    datasource_type = Column(String(200))  # Usually "table"
    viz_type = Column(String(250))  # e.g., "bar", "line", "table"
    params = Column(Text)  # JSON string with formData
    query_context = Column(Text)  # JSON string with query configuration
    cache_timeout = Column(Integer)
```

**Key Storage Characteristics:**

- **Primary Key**: Integer `id` field
- **UUID**: Each chart has a unique UUID
- **JSON Storage**: `params` and `query_context` are stored as JSON strings
- **Foreign Key Reference**: `datasource_id` references `tables.id` (the dataset)

### 2.3 Chart-to-Dataset Relationship

**Relationship**: Many Charts → One Dataset

- Charts reference datasets via `datasource_id` and `datasource_type`
- The `datasource_type` is typically `"table"` (referring to the Dataset)
- Charts can be created from the same dataset with different visualizations
- When a dataset is deleted, charts referencing it may become invalid

### 2.4 Chart Configuration Storage

The `params` field stores chart configuration as JSON:

```json
{
  "adhoc_filters": [],
  "columns": ["region"],
  "groupby": ["region"],
  "metrics": ["sum__sales"],
  "row_limit": 50,
  "color_scheme": "supersetColors",
  "viz_type": "dist_bar"
}
```

The `query_context` field stores query execution metadata:

```json
{
  "datasource": { "id": 15, "type": "table" },
  "force": false,
  "queries": [
    {
      "columns": ["region"],
      "metrics": ["sum__sales"],
      "row_limit": 50
    }
  ],
  "result_format": "json",
  "result_type": "full"
}
```

### 2.5 Chart Versioning

Charts have **limited versioning**:

- **Migration System**: Charts can be migrated between visualization types (e.g., legacy NVD3 to ECharts) via `viz-migrations` CLI command
- **Audit Trail**: `AuditMixinNullable` tracks creation and modification
- **No Historical Versions**: Chart changes overwrite previous configurations

---

## 3. Dashboards

### 3.1 What is a Dashboard?

A **Dashboard** is a collection of charts arranged in a layout. It contains:

- Layout configuration (which charts, their positions, sizes)
- Metadata (refresh frequency, filters, color schemes)
- Presentation settings (CSS, publication status)

### 3.2 Dashboard Storage

Dashboards are stored in the `dashboards` table:

```python
class Dashboard(Model, AuditMixinNullable):
    __tablename__ = "dashboards"

    id = Column(Integer, primary_key=True)
    dashboard_title = Column(String(500))
    slug = Column(String(255), unique=True)
    uuid = Column(String(36), unique=True)
    position_json = Column(Text)  # JSON layout definition
    json_metadata = Column(Text)  # JSON metadata
    css = Column(Text)  # Custom CSS
    published = Column(Boolean)
```

**Key Storage Characteristics:**

- **Primary Key**: Integer `id` field
- **Unique Identifiers**: Both `slug` (human-readable) and `uuid` (machine-readable)
- **JSON Storage**: Layout and metadata stored as JSON strings
- **No Direct Foreign Keys**: Charts are referenced indirectly via `position_json`

### 3.3 Dashboard-to-Chart Relationship

**Relationship**: Many Dashboards → Many Charts (via JSON structure)

Dashboards reference charts through the `position_json` field, which contains a nested JSON structure:

```json
{
  "DASHBOARD_VERSION_KEY": "v2",
  "GRID_ID": {
    "type": "GRID",
    "id": "GRID_ID",
    "children": ["CHART-1", "CHART-2"],
    "meta": {}
  },
  "CHART-1": {
    "type": "CHART",
    "id": "CHART-1",
    "children": [],
    "meta": {
      "chartId": 42,
      "width": 6,
      "height": 4,
      "sliceName": "Sales by Region"
    }
  },
  "CHART-2": {
    "type": "CHART",
    "id": "CHART-2",
    "children": [],
    "meta": {
      "chartId": 43,
      "width": 6,
      "height": 4
    }
  }
}
```

**Key Points:**

- Charts are referenced by their integer `id` in the `meta.chartId` field
- The layout is versioned via `DASHBOARD_VERSION_KEY` (currently "v2")
- Multiple dashboards can reference the same chart
- Charts can be added/removed/repositioned without modifying the chart itself

### 3.4 Dashboard Metadata

The `json_metadata` field stores dashboard-level configuration:

```json
{
  "refresh_frequency": 30,
  "color_scheme": "supersetColors",
  "native_filter_configuration": [],
  "timed_refresh_immune_slices": [324],
  "stagger_refresh": false,
  "stagger_time": 2500,
  "filter_immune_slices": [],
  "expanded_slices": {}
}
```

### 3.5 Dashboard Versioning

Dashboards have **layout versioning**:

- **Layout Version**: `DASHBOARD_VERSION_KEY` in `position_json` tracks layout format version
- **Migration System**: Dashboard layouts are automatically migrated between versions (e.g., v1 → v2 in Superset 0.28)
- **Audit Trail**: `AuditMixinNullable` tracks creation and modification
- **No Historical Versions**: Dashboard changes overwrite previous configurations

---

## 4. Object Relationships and Data Flow

### 4.1 Hierarchical Structure

```
Database (dbs)
    └── Dataset (tables)
        ├── Columns (table_columns)
        ├── Metrics (sql_metrics)
        └── Charts (slices)
            └── Dashboards (dashboards)
                └── position_json references chart IDs
```

### 4.2 Reference Chain

1. **Database** → **Dataset**: Foreign key `database_id` in `tables`
2. **Dataset** → **Columns/Metrics**: Foreign keys `dataset_id` in `table_columns` and `sql_metrics`
3. **Dataset** → **Chart**: Foreign key `datasource_id` in `slices`
4. **Chart** → **Dashboard**: Indirect reference via `position_json.meta.chartId`

### 4.3 Data Flow Example

When a dashboard is rendered:

1. **Dashboard Load**: Fetch dashboard by ID/slug
2. **Extract Chart IDs**: Parse `position_json` to get all `chartId` values
3. **Load Charts**: Fetch chart records using chart IDs
4. **Load Datasets**: For each chart, fetch the dataset via `datasource_id`
5. **Execute Queries**: Use chart's `query_context` and dataset's column/metric definitions
6. **Render**: Display charts in positions defined by `position_json`

### 4.4 Dependency Resolution

The API endpoint `/api/v1/dashboard/{id}/datasets` demonstrates relationship traversal:

- Starts with dashboard
- Extracts chart IDs from `position_json`
- Loads charts to get `datasource_id` values
- Returns unique datasets used by all charts

---

## 5. Versioning and Change Management

### 5.1 Audit Trail (All Objects)

All three object types inherit from `AuditMixinNullable`, which provides:

```python
class AuditMixinNullable:
    created_on = Column(DateTime)
    changed_on = Column(DateTime)
    created_by_fk = Column(Integer, ForeignKey('ab_user.id'))
    changed_by_fk = Column(Integer, ForeignKey('ab_user.id'))
```

**Limitations:**

- Tracks **who** and **when**, but not **what** changed
- No historical snapshots
- Changes overwrite previous state

### 5.2 Migration System

#### Dataset Migrations

- **Schema Refresh**: `/api/v1/dataset/{id}/refresh` syncs columns/types from database
- **Import/Export**: Datasets can be exported/imported via CLI commands

#### Chart Migrations

- **Viz Type Migration**: `superset viz-migrations` command migrates charts to new visualization types
- **Automatic Migration**: Charts are automatically migrated when dashboards are imported/exported

#### Dashboard Migrations

- **Layout Migration**: Automatic migration between layout versions (e.g., 0.27 → 0.28)
- **Chart Type Migration**: When dashboards are migrated, charts are upgraded to latest types

### 5.3 Versioning Limitations

**No Explicit Versioning:**

- No version numbers for datasets, charts, or dashboards
- No rollback capability
- No branching or merging
- No diff/comparison tools

**Workarounds:**

- Export/import for backup
- Git-based version control for exported YAML/JSON
- Database backups for full state preservation

---

## 6. Storage Architecture

### 6.1 Metadata Database

All objects are stored in Superset's **metadata database**:

- **Recommended**: PostgreSQL or MySQL for production
- **Default**: SQLite for development (not recommended for production)
- **Purpose**: Stores all Superset configuration, not the actual data being visualized

### 6.2 Table Structure Summary

| Object    | Table Name      | Primary Key | Key Foreign Keys                  |
| --------- | --------------- | ----------- | --------------------------------- |
| Database  | `dbs`           | `id`        | None                              |
| Dataset   | `tables`        | `id`        | `database_id` → `dbs.id`          |
| Column    | `table_columns` | `id`        | `dataset_id` → `tables.id`        |
| Metric    | `sql_metrics`   | `id`        | `dataset_id` → `tables.id`        |
| Chart     | `slices`        | `id`        | `datasource_id` → `tables.id`     |
| Dashboard | `dashboards`    | `id`        | None (charts referenced via JSON) |

### 6.3 JSON Storage Patterns

**Why JSON?**

- Flexibility for evolving schemas
- Complex nested structures (dashboard layouts)
- Query configuration that varies by chart type

**Trade-offs:**

- Not queryable via SQL (must parse JSON)
- No referential integrity for chart references in dashboards
- Potential for orphaned references

---

## 7. Best Practices and Implications

### 7.1 Dataset as Source of Truth

**Datasets are the foundation:**

- Define columns and metrics once, reuse in many charts
- Changes to dataset schema propagate to all charts
- Deleting a dataset invalidates dependent charts

**Recommendations:**

- Use descriptive dataset names
- Document columns and metrics
- Refresh schemas after database changes
- Avoid deleting datasets with active charts

### 7.2 Chart Independence

**Charts are reusable:**

- Same chart can appear in multiple dashboards
- Chart configuration is independent of dashboard layout
- Changing a chart affects all dashboards using it

**Recommendations:**

- Use clear chart names
- Document chart purposes
- Consider chart duplication if different dashboards need different configurations

### 7.3 Dashboard Composition

**Dashboards are compositions:**

- Reference charts, don't own them
- Layout changes don't affect charts
- Can be published/unpublished independently

**Recommendations:**

- Use version control for dashboard exports
- Document dashboard purposes and audiences
- Test dashboard after chart updates

---

## 8. API Access Patterns

### 8.1 Dataset API

```bash
# Create dataset
POST /api/v1/dataset/
{
  "database": 1,
  "schema": "public",
  "table_name": "sales_data",
  "owners": [1]
}

# Get dataset
GET /api/v1/dataset/{id}

# Update dataset
PUT /api/v1/dataset/{id}

# Refresh schema
POST /api/v1/dataset/{id}/refresh
```

### 8.2 Chart API

```bash
# Create chart
POST /api/v1/chart/
{
  "slice_name": "Sales by Region",
  "datasource_id": 15,
  "datasource_type": "table",
  "viz_type": "dist_bar",
  "params": "{...}",
  "query_context": "{...}"
}

# Get chart
GET /api/v1/chart/{id}
```

### 8.3 Dashboard API

```bash
# Create dashboard
POST /api/v1/dashboard/
{
  "dashboard_title": "Sales Dashboard",
  "slug": "sales-dashboard",
  "position_json": "{...}",
  "json_metadata": "{...}"
}

# Get dashboard
GET /api/v1/dashboard/{id_or_slug}

# Get dashboard datasets
GET /api/v1/dashboard/{id}/datasets
```

---

## 9. Conclusion

### Key Takeaways

1. **Dataset is the Unit of Truth**: Datasets define data structure and are referenced by charts
2. **Charts Reference Datasets**: Charts are visualization configurations that query datasets
3. **Dashboards Reference Charts**: Dashboards compose charts into layouts via JSON
4. **Storage is Relational + JSON**: Core relationships use foreign keys; complex configs use JSON
5. **Limited Versioning**: Audit trails exist, but no version history or rollback
6. **Metadata Database**: All objects stored in Superset's metadata database (not the data sources)

### Architecture Strengths

- Clear separation of concerns (data definition → visualization → presentation)
- Reusability (one dataset → many charts → many dashboards)
- Flexibility (JSON allows evolving schemas)

### Architecture Limitations

- No explicit versioning system
- JSON references lack referential integrity
- No built-in change tracking beyond audit timestamps
- Dashboard layout changes can break if charts are deleted

This architecture prioritizes flexibility and ease of use over strict versioning and referential integrity, which aligns with Superset's goal of being an agile business intelligence tool.

