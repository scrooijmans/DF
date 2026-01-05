# Grafana Architecture: Unit of Truth and Data Source References

## Executive Summary

Grafana's architecture is built around a clear separation of concerns with distinct units of truth: **Dashboard JSON** serves as the primary unit of truth for dashboard configuration, **Data Frames** serve as the unified data structure for all data flowing through the system, and **Data Sources** are stored independently and referenced (not owned) by panels. This architecture enables multiple panels to share the same data source without data duplication, while maintaining version history and schema compatibility.

---

## 1. Units of Truth in Grafana

### 1.1 Dashboard JSON: The Primary Configuration Unit

**Dashboard JSON** is the fundamental unit of truth for dashboard configuration in Grafana. It is a complete JSON object that stores:

- **Dashboard metadata**: ID, UID, title, tags, timezone, schema version
- **Panel definitions**: Array of panel JSON objects, each containing visualization configuration
- **Template variables**: Reusable variables for dynamic queries
- **Time settings**: Time range, refresh intervals, timezone
- **Annotations**: Event markers and overlays
- **Layout configuration**: Panel positioning and organization

#### Dashboard JSON Structure

```json
{
  "id": null,  // Assigned when saved
  "uid": "cLV5GDCkz",  // Unique identifier
  "title": "New dashboard",
  "tags": [],
  "timezone": "browser",
  "editable": true,
  "graphTooltip": 1,
  "panels": [],  // Array of panel definitions
  "time": {
    "from": "now-6h",
    "to": "now"
  },
  "templating": {
    "list": []
  },
  "annotations": {
    "list": []
  },
  "refresh": "5s",
  "schemaVersion": 17,  // Schema version for migration compatibility
  "version": 0,  // Dashboard version number
  "links": []
}
```

**Key Characteristics:**
- The `id` field is `null` until the dashboard is saved, at which point Grafana assigns an integer value
- The `uid` (unique identifier) is the primary reference for dashboards in APIs
- `schemaVersion` tracks the dashboard schema format for migration purposes
- `version` tracks the dashboard's revision history

### 1.2 Panel Models: Component-Level Configuration

**Panel models** are JSON objects within the dashboard's `panels` array. Each panel contains:

- **Panel type**: Visualization type (graph, stat, table, etc.)
- **Data source reference**: Reference to the data source (by UID and type)
- **Query configuration**: Targets/queries to execute against the data source
- **Visualization options**: Display settings, thresholds, legends, etc.
- **Grid position**: Layout coordinates (x, y, width, height)

#### Panel JSON Example

```json
{
  "type": "graph",
  "title": "CPU Usage",
  "gridPos": {
    "x": 0,
    "y": 0,
    "w": 24,
    "h": 8
  },
  "id": 2,
  "datasource": {
    "type": "prometheus",
    "uid": "kLtEtcRGk"
  },
  "targets": [
    {
      "expr": "cpu_usage_percent",
      "refId": "A"
    }
  ],
  "legend": {
    "show": true
  }
}
```

**Critical Point**: Panels **reference** data sources via `datasource.uid` and `datasource.type`; they do **not** own or embed the data source configuration.

### 1.3 Data Frames: The Unified Data Structure

**Data Frames** are the unified data structure that serves as the unit of truth for **all data** flowing through Grafana, regardless of the source data model. This abstraction is fundamental to Grafana's architecture.

#### Data Frame Architecture

- **Columnar structure**: Data frames are columnar-oriented tables
- **Fields**: Columns are called "fields" (e.g., time field, value field)
- **Unified model**: Both time series and table query results are represented as data frames
- **Time series representation**: A time series is a data frame with two fields: time and value

#### How Data Frames Work

1. **Data Source Plugins** retrieve data from external sources (Prometheus, Loki, SQL databases, APIs, etc.)
2. **Transformation**: The plugin transforms the source data into Grafana's data frame format
3. **Unified Interface**: All panels receive data in the same data frame format, regardless of source
4. **Visualization**: Panels interpret data frames based on their visualization type

**Key Insight**: The data frame abstraction allows Grafana to support 150+ data source plugins while maintaining a consistent internal data model. This is the "unified data structure" that reconciles differences between various data source models.

---

## 2. Storage Architecture

### 2.1 Dashboard Storage

Dashboards are stored in Grafana's **database** (typically SQLite, PostgreSQL, or MySQL):

- **Primary storage**: Dashboard JSON is persisted in the database
- **Version history**: Each save creates a new version entry, preserving all previous versions
- **Organization-scoped**: Dashboards belong to organizations, providing isolation
- **Folder hierarchy**: Dashboards can be organized in folders, preserving structure

#### Version History Mechanism

- **Automatic versioning**: Every dashboard save creates a new version
- **Immutable versions**: Previous versions are never modified
- **Version comparison**: Users can compare any two versions (JSON diff or structured diff)
- **Restore capability**: Any version can be restored, creating a new version with that content

**Storage Structure:**
```
Dashboard Table:
- id (integer, primary key)
- uid (string, unique identifier)
- org_id (integer, organization)
- folder_id (integer, optional)
- title (string)
- data (JSON, the dashboard JSON)
- version (integer, dashboard version)
- created (timestamp)
- updated (timestamp)

Dashboard Version History Table:
- id (integer)
- dashboard_id (integer, foreign key)
- version (integer)
- data (JSON, snapshot of dashboard JSON)
- created (timestamp)
- created_by (integer, user ID)
- message (string, optional commit message)
```

### 2.2 Data Source Storage

**Data sources are stored separately** from dashboards in their own database tables:

- **Independent storage**: Data source configuration is stored in a dedicated table
- **Organization-scoped**: Data sources belong to organizations
- **Shared resource**: Multiple dashboards and panels can reference the same data source
- **Version tracking**: Data sources have their own version numbers for configuration changes

#### Data Source Storage Structure

```
Data Source Table:
- id (integer, primary key)
- uid (string, unique identifier)  // Primary reference mechanism
- org_id (integer, organization)
- name (string)
- type (string, e.g., "prometheus", "loki", "mysql")
- access (string, "proxy" or "direct")
- url (string, data source endpoint)
- json_data (JSON, type-specific configuration)
- secure_json_fields (JSON, encrypted sensitive fields)
- version (integer, configuration version)
- is_default (boolean)
- read_only (boolean)
```

**Critical Separation**: Data sources are **never embedded** in dashboard JSON. They are always referenced by UID.

### 2.3 Schema Versioning

Grafana maintains **schema versioning** to ensure backward compatibility:

- **Schema version**: Each dashboard has a `schemaVersion` field (currently up to v42+)
- **Migration system**: Grafana automatically migrates dashboards from older schema versions to newer ones
- **Backward compatibility**: Older dashboards continue to work with newer Grafana versions
- **Migration testing**: Comprehensive test suite ensures migrations don't break existing dashboards

**Migration Process:**
1. Dashboard loaded from database
2. Schema version detected
3. Sequential migrations applied (v14→v15→v16→...→current)
4. Dashboard updated to current schema version
5. Dashboard rendered/displayed

---

## 3. Referencing Mechanism: How Panels Reference Data Sources

### 3.1 Data Source Reference Model

Panels reference data sources through a **lightweight reference object**:

```json
{
  "datasource": {
    "type": "prometheus",      // Plugin type identifier
    "uid": "kLtEtcRGk"        // Unique identifier of the data source instance
  }
}
```

**Alternative Reference Formats:**
- **By UID only**: `{ "uid": "kLtEtcRGk" }`
- **By type only** (for default data source): `{ "type": "prometheus" }`
- **Legacy format** (deprecated): `"datasource": "Prometheus"` (name-based)

### 3.2 Resolution Process

When a panel needs to query a data source:

1. **Reference extraction**: Panel extracts `datasource.uid` and `datasource.type` from panel JSON
2. **Data source lookup**: Grafana queries the data source table using the UID
3. **Configuration retrieval**: Full data source configuration (URL, credentials, type-specific settings) is loaded
4. **Query execution**: Panel queries are executed against the resolved data source
5. **Data frame generation**: Data source plugin returns data in data frame format
6. **Panel rendering**: Panel visualizes the data frames

**Key Point**: The panel never stores the data source configuration. It only stores a reference, and the actual configuration is resolved at query time.

### 3.3 Multiple Panels, One Data Source

The reference model enables **data source sharing**:

```
Data Source: Prometheus (UID: "kLtEtcRGk")
├── Panel 1: CPU Usage Graph
│   └── References: { "type": "prometheus", "uid": "kLtEtcRGk" }
├── Panel 2: Memory Usage Stat
│   └── References: { "type": "prometheus", "uid": "kLtEtcRGk" }
└── Panel 3: Network Traffic Table
    └── References: { "type": "prometheus", "uid": "kLtEtcRGk" }
```

**Benefits:**
- **Single source of truth**: Data source configuration is stored once
- **Centralized management**: Update data source URL/credentials in one place, all panels automatically use the new configuration
- **No data duplication**: Data source configuration is not copied into each panel
- **Efficient storage**: Dashboard JSON remains lightweight

### 3.4 Data Source Ownership and Permissions

**Data sources are organization-level resources**, not panel-level:

- **Organization ownership**: Data sources belong to organizations, not individual dashboards or panels
- **Permission model**: Data source permissions are managed at the organization/data source level
- **Access control**: Users with appropriate permissions can query a data source from any panel
- **Enterprise features**: Grafana Enterprise supports Label-Based Access Control (LBAC) for fine-grained data source access

**Permission Hierarchy:**
```
Organization
└── Data Source (UID: "kLtEtcRGk")
    ├── Permissions (who can query/edit)
    └── Referenced by:
        ├── Dashboard A, Panel 1
        ├── Dashboard A, Panel 2
        ├── Dashboard B, Panel 1
        └── Dashboard C, Panel 3
```

---

## 4. Versioning System

### 4.1 Dashboard Versioning

**Dashboard versioning** is comprehensive and automatic:

- **Version number**: Each dashboard has an incrementing `version` field (0, 1, 2, ...)
- **Version history table**: All versions are preserved in a separate table
- **Immutable versions**: Once created, a version is never modified
- **Restore creates new version**: Restoring a previous version creates a new version (e.g., restoring v5 creates v10, preserving v5)

**Version Management:**
- **View versions**: Users can see all versions in the dashboard settings
- **Compare versions**: Side-by-side comparison with JSON diff
- **Restore versions**: Restore any previous version
- **Version metadata**: Each version stores creator, timestamp, and optional commit message

### 4.2 Data Source Versioning

**Data source versioning** tracks configuration changes:

- **Version field**: Each data source has a `version` integer field
- **Configuration tracking**: Version increments when data source configuration changes
- **Provisioning compatibility**: When provisioning data sources via configuration files, version numbers prevent older configurations from overwriting newer ones

**Provisioning Example:**
```yaml
# datasources.yaml
datasources:
  - name: Prometheus
    type: prometheus
    uid: kLtEtcRGk
    url: http://prometheus:9090
    version: 2  # Prevents overwriting if database version is higher
```

### 4.3 Schema Versioning

**Schema versioning** ensures backward compatibility:

- **Schema version field**: Each dashboard has a `schemaVersion` field (e.g., 17, 18, 19)
- **Migration system**: Automatic migration from older schema versions to current
- **Migration tests**: Comprehensive test suite validates migrations
- **Breaking changes**: Documented breaking changes guide migration paths

**Migration Flow:**
```
Dashboard v14 (schemaVersion: 14)
    ↓ Migration v14→v15
Dashboard v15 (schemaVersion: 15)
    ↓ Migration v15→v16
Dashboard v16 (schemaVersion: 16)
    ↓ Migration v16→v17
Dashboard v17 (schemaVersion: 17)  // Current
```

---

## 5. Data Flow: From Source to Visualization

### 5.1 Complete Data Flow Architecture

```
External Data Source (Prometheus/Loki/SQL/etc.)
    ↓
Data Source Plugin (transforms to data frames)
    ↓
Data Frames (unified structure)
    ↓
Panel Query Processing
    ↓
Panel Visualization
```

### 5.2 Detailed Flow

1. **User Interaction**: User views dashboard or panel refreshes
2. **Panel Activation**: Panel reads its configuration from dashboard JSON
3. **Data Source Resolution**: Panel extracts `datasource.uid`, Grafana looks up data source configuration
4. **Query Construction**: Panel builds queries from `targets` array in panel JSON
5. **Query Execution**: Data source plugin executes queries against external data source
6. **Data Transformation**: Plugin transforms source data into data frames
7. **Data Frame Delivery**: Data frames returned to panel
8. **Visualization**: Panel renders data frames according to visualization type

**Key Insight**: At no point does the panel "own" the data source. The data source is resolved dynamically, queried, and data is returned in a standardized format (data frames).

---

## 6. Key Architectural Principles

### 6.1 Separation of Concerns

- **Configuration vs. Data**: Dashboard JSON stores configuration, not data
- **Reference vs. Embed**: Panels reference data sources, they don't embed them
- **Storage vs. Runtime**: Data sources are stored separately, resolved at runtime

### 6.2 Single Source of Truth

- **Dashboard JSON**: Single source of truth for dashboard configuration
- **Data Frames**: Single source of truth for data structure
- **Data Source Table**: Single source of truth for data source configuration

### 6.3 Referential Integrity

- **UID-based references**: All references use UIDs for stability
- **Validation**: Grafana validates data source references when dashboards are loaded
- **Error handling**: Missing data sources are handled gracefully (error messages, fallbacks)

### 6.4 Versioning and Immutability

- **Immutable versions**: Previous versions are never modified
- **Version history**: Complete audit trail of changes
- **Schema compatibility**: Automatic migration ensures compatibility

---

## 7. Practical Implications

### 7.1 Dashboard Provisioning

When provisioning dashboards via configuration files:

- **Data source references**: Must use UIDs that exist in the target Grafana instance
- **Version handling**: Provisioned dashboards can overwrite database versions (with caution)
- **Schema compatibility**: Provisioned dashboards must use compatible schema versions

### 7.2 Data Source Management

- **Centralized updates**: Update data source URL/credentials once, all panels automatically use new configuration
- **Permission management**: Control access at the data source level, not panel level
- **Multi-dashboard impact**: Changes to shared data sources affect all referencing dashboards

### 7.3 Migration and Upgrades

- **Automatic migration**: Grafana automatically migrates dashboards to current schema
- **Version preservation**: All historical versions are preserved during migration
- **Rollback capability**: Can restore any previous version if needed

---

## 8. Summary

Grafana's architecture is built on three fundamental units of truth:

1. **Dashboard JSON**: The complete configuration unit for dashboards, stored in the database with full version history
2. **Data Frames**: The unified data structure that all data sources transform their data into, enabling consistent visualization across 150+ data source types
3. **Data Source Configuration**: Stored independently in the database, referenced by UID from panels

**Key Architectural Features:**

- **Reference Model**: Panels reference data sources by UID, they don't own or embed them
- **Shared Resources**: Multiple panels across multiple dashboards can reference the same data source
- **Version History**: Complete version history for dashboards, with immutable versions
- **Schema Migration**: Automatic migration ensures backward compatibility
- **Separation of Concerns**: Clear separation between configuration (dashboard JSON), data structure (data frames), and data source configuration

This architecture enables Grafana to be both flexible (supporting many data sources) and efficient (avoiding data duplication), while maintaining a clear audit trail and enabling centralized data source management.


