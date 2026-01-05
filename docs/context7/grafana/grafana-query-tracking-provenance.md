# Grafana Query Tracking and Data Provenance Architecture

## Executive Summary

Grafana provides comprehensive tracking of query origins, data source references, and panel configuration history through multiple mechanisms: **DataFrame metadata** stores executed queries and data source information, **Panel Inspector** provides runtime inspection of queries and data, **Dashboard version history** tracks configuration changes over time, **Query History** records executed queries for reuse, and **Panel JSON** contains complete configuration including data source references. These mechanisms work together to provide full **data lineage** and **provenance tracking**, allowing users to understand exactly where rendered data comes from, how queries were executed, and how panel configurations have evolved.

---

## 1. Query Origin Tracking

### 1.1 Query Request Tracking

Every query execution includes tracking information in the **DataQueryRequest**:

```typescript
interface DataQueryRequest {
  // Query identification
  requestId: string;            // Unique request ID for tracking
  startTime: number;            // Request start timestamp
  
  // Query targets
  targets: Array<{
    refId: string;              // Query reference ID (A, B, C, etc.)
    datasource: {
      type: string;             // Data source type
      uid: string;              // Data source UID
    };
    // Query-specific fields
    expr?: string;              // Prometheus query
    query?: string;             // SQL query, etc.
  }>;
  
  // Context
  range: TimeRange;             // Time range for query
  interval: string;             // Query interval
  maxDataPoints: number;        // Maximum data points
  scopedVars: object;           // Template variable values
  app: string;                  // Application context ("dashboard", "explore")
}
```

**Tracking Elements:**
- **requestId**: Unique identifier for each query request
- **refId**: Reference ID linking queries to results
- **startTime**: Timestamp for performance tracking
- **app**: Context (dashboard vs. explore)

### 1.2 Query Response Tracking

Query responses include tracking metadata in **DataFrameMeta**:

```typescript
interface DataFrameMeta {
  // Query tracking
  executedQueryString?: string;  // Actual query executed (after variable replacement)
  requestId?: string;            // Request ID from query request
  
  // Data source information
  datasource?: {
    type: string;                // Data source type
    uid: string;                 // Data source UID
    name?: string;               // Data source name
  };
  
  // Query statistics
  stats?: Array<{
    displayName: string;         // Stat name (e.g., "Query time")
    value: number;                // Stat value
  }>;
  
  // Custom metadata
  custom?: Record<string, any>;
}
```

**Example Metadata:**
```typescript
{
  executedQueryString: "rate(http_requests_total{host=\"server1\"}[5m])",
  requestId: "query-1704067200000-abc123",
  datasource: {
    type: "prometheus",
    uid: "prometheus-uid",
    name: "Prometheus Production",
  },
  stats: [
    { displayName: "Query time", value: 150 },  // milliseconds
    { displayName: "Data points", value: 1000 },
  ],
}
```

### 1.3 Query Execution Flow with Tracking

```
1. Panel constructs query request
   ↓
2. Query request created with:
   - requestId: "query-1704067200000-abc123"
   - refId: "A"
   - datasource: { type: "prometheus", uid: "prometheus-uid" }
   - targets: [{ expr: "rate(http_requests_total[5m])", refId: "A" }]
   ↓
3. Query sent to data source plugin
   ↓
4. Data source executes query
   ↓
5. Data source creates DataFrame with metadata:
   - frame.meta.executedQueryString = "rate(http_requests_total[5m])"
   - frame.meta.requestId = "query-1704067200000-abc123"
   - frame.meta.datasource = { type: "prometheus", uid: "prometheus-uid" }
   - frame.refId = "A"
   ↓
6. DataFrame returned to panel
   ↓
7. Panel stores DataFrame with metadata
   ↓
8. Panel Inspector can display metadata
```

---

## 2. Data Source Reference Tracking

### 2.1 Panel Configuration References

Panels store data source references in their configuration:

```json
{
  "id": 1,
  "type": "graph",
  "title": "CPU Usage",
  "datasource": {
    "type": "prometheus",
    "uid": "prometheus-uid"
  },
  "targets": [
    {
      "refId": "A",
      "expr": "rate(cpu_usage_total[5m])",
      "datasource": {
        "type": "prometheus",
        "uid": "prometheus-uid"
      }
    }
  ]
}
```

**Reference Levels:**
1. **Panel-level**: `panel.datasource` - Default data source for panel
2. **Query-level**: `target.datasource` - Data source for specific query (can override panel default)

### 2.2 Data Source Resolution

When a panel executes queries, data source references are resolved:

```typescript
class PanelComponent {
  async executeQueries() {
    const targets = this.panel.targets;
    
    for (const target of targets) {
      // Resolve data source reference
      const datasourceRef = target.datasource || this.panel.datasource;
      
      // Get data source instance
      const datasource = await getDataSourceSrv().get(datasourceRef.uid);
      
      // Build query request
      const request: DataQueryRequest = {
        targets: [target],
        range: this.props.timeRange,
        requestId: `query-${Date.now()}-${target.refId}`,
        // ... other parameters
      };
      
      // Execute query
      const response = await datasource.query(request).toPromise();
      
      // Metadata includes data source reference
      response.data.forEach(frame => {
        frame.meta = {
          ...frame.meta,
          datasource: {
            type: datasource.type,
            uid: datasource.uid,
            name: datasource.name,
          },
        };
      });
    }
  }
}
```

### 2.3 Data Source Reference Tracking in Metadata

DataFrames include data source information in metadata:

```typescript
// DataFrame with data source tracking
const frame: DataFrame = {
  name: "http_requests_total",
  refId: "A",
  fields: [
    { name: "time", type: FieldType.time, values: [...] },
    { name: "value", type: FieldType.number, values: [...] },
  ],
  meta: {
    executedQueryString: "rate(http_requests_total[5m])",
    datasource: {
      type: "prometheus",
      uid: "prometheus-uid",
      name: "Prometheus Production",
    },
    requestId: "query-1704067200000-abc123",
  },
};
```

**Benefits:**
- **Provenance**: Know which data source produced the data
- **Debugging**: Identify data source issues
- **Auditing**: Track data source usage
- **Documentation**: Understand data lineage

---

## 3. Panel Inspector: Runtime Query Inspection

### 3.1 Panel Inspector Overview

The **Panel Inspector** provides runtime inspection of panel data, queries, and configuration:

**Access:**
- Panel menu → **Inspect**
- Query editor → **Query Inspector** button

**Inspector Tabs:**
1. **Data**: Raw data returned by queries
2. **Stats**: Query performance statistics
3. **Query**: Request and response data
4. **JSON**: Panel JSON, panel data JSON, and DataFrame structure

### 3.2 Data Tab

The **Data tab** shows raw data returned by queries:

```typescript
// Data tab displays
interface DataTabContent {
  // Data frames
  frames: DataFrame[];
  
  // Field information
  fields: Array<{
    name: string;
    type: FieldType;
    values: any[];
    labels?: Labels;
  }>;
  
  // Download options
  downloadCSV: () => void;
  downloadJSON: () => void;
}
```

**Features:**
- **Raw Data View**: See actual data values
- **Field Details**: Field names, types, labels
- **Data Export**: Download as CSV or JSON
- **Transformations**: View data before/after transformations

### 3.3 Stats Tab

The **Stats tab** shows query performance statistics:

```typescript
interface StatsTabContent {
  // Query statistics
  queryTime: number;            // Query execution time (ms)
  dataProcessingTime: number;   // Data processing time (ms)
  dataPoints: number;            // Number of data points
  seriesCount: number;           // Number of series
  
  // Frame statistics
  frames: Array<{
    name: string;
    fields: number;
    rows: number;
    size: number;                // Size in bytes
  }>;
}
```

**Example Stats:**
```
Query time: 150ms
Data processing time: 20ms
Data points: 1000
Series: 5
Frames: 1
```

### 3.4 Query Tab

The **Query tab** shows request and response data:

```typescript
interface QueryTabContent {
  // Request data
  request: {
    targets: Array<{
      refId: string;
      datasource: {
        type: string;
        uid: string;
      };
      expr?: string;
      query?: string;
    }>;
    range: TimeRange;
    interval: string;
    maxDataPoints: number;
    requestId: string;
    startTime: number;
  };
  
  // Response data
  response: {
    data: DataFrame[];
    state: 'Done' | 'Loading' | 'Error';
    error?: Error;
  };
  
  // Timing information
  timing: {
    queryTime: number;
    dataProcessingTime: number;
    totalTime: number;
  };
}
```

**Features:**
- **Request Inspection**: See exact query sent to data source
- **Response Inspection**: See raw response from data source
- **Timing Analysis**: Understand query performance
- **Error Debugging**: See error details if query failed

### 3.5 JSON Tab

The **JSON tab** shows three types of JSON:

**1. Panel JSON:**
```json
{
  "id": 1,
  "type": "graph",
  "title": "CPU Usage",
  "datasource": {
    "type": "prometheus",
    "uid": "prometheus-uid"
  },
  "targets": [
    {
      "refId": "A",
      "expr": "rate(cpu_usage_total[5m])"
    }
  ],
  "fieldConfig": {
    "defaults": {
      "unit": "short"
    }
  }
}
```

**2. Panel Data JSON:**
```json
{
  "series": [
    {
      "name": "CPU Usage",
      "fields": [
        {
          "name": "time",
          "type": "time",
          "values": [1704067200000, 1704067260000, ...]
        },
        {
          "name": "value",
          "type": "number",
          "values": [45.2, 46.1, ...]
        }
      ]
    }
  ]
}
```

**3. DataFrame Structure JSON:**
```json
{
  "schema": {
    "refId": "A",
    "fields": [
      {
        "name": "time",
        "type": "time",
        "typeInfo": {
          "frame": "time.Time"
        }
      },
      {
        "name": "value",
        "type": "number",
        "typeInfo": {
          "frame": "float64"
        }
      }
    ]
  },
  "meta": {
    "executedQueryString": "rate(cpu_usage_total[5m])",
    "datasource": {
      "type": "prometheus",
      "uid": "prometheus-uid"
    }
  }
}
```

**Use Cases:**
- **Provisioning**: Copy panel JSON for dashboard provisioning
- **Debugging**: Inspect panel configuration
- **Documentation**: Understand panel structure
- **Troubleshooting**: Identify configuration issues

---

## 4. Query Inspector in Query Editor

### 4.1 Query Inspector Access

The **Query Inspector** is accessible from the query editor:

**Access:**
- Query editor → **Query Inspector** button
- Available during query editing

**Inspector Tabs:**
1. **Stats**: Query performance statistics
2. **Query**: Request and response data
3. **JSON**: DataFrame structure JSON
4. **Data**: Raw data returned by query
5. **Error**: Error information (if query failed)

### 4.2 Query Inspector Features

**Real-time Query Inspection:**
- **Live Updates**: Inspector updates when query executes
- **Request/Response**: See exact request sent and response received
- **Performance Metrics**: Query time, data processing time
- **Data Preview**: Preview data before applying to panel

**Query Optimization:**
- **Identify Slow Queries**: See query execution time
- **Data Volume Analysis**: See data points returned
- **Query Validation**: Verify query syntax and results

---

## 5. Panel Configuration History

### 5.1 Dashboard Version History

**Dashboard version history** tracks all changes to dashboard configuration, including panel configurations:

**Version Storage:**
```typescript
interface DashboardVersion {
  id: number;                   // Version ID
  dashboardId: number;          // Dashboard ID
  version: number;              // Version number
  data: DashboardJSON;          // Complete dashboard JSON snapshot
  created: number;              // Timestamp
  createdBy: number;            // User ID
  message?: string;             // Commit message
}
```

**Version History Features:**
- **Complete Snapshots**: Each version contains full dashboard JSON
- **Panel Configuration**: Panel configs included in each version
- **Immutable Versions**: Versions never change once created
- **Comparison**: Compare any two versions
- **Restore**: Restore any previous version

### 5.2 Panel Configuration in Versions

Each dashboard version includes complete panel configurations:

```json
{
  "version": 5,
  "data": {
    "panels": [
      {
        "id": 1,
        "type": "graph",
        "title": "CPU Usage",
        "datasource": {
          "type": "prometheus",
          "uid": "prometheus-uid"
        },
        "targets": [
          {
            "refId": "A",
            "expr": "rate(cpu_usage_total[5m])"
          }
        ],
        "fieldConfig": {
          "defaults": {
            "unit": "short"
          }
        }
      }
    ]
  },
  "created": 1704067200000,
  "createdBy": 1,
  "message": "Updated CPU query"
}
```

**Tracking Capabilities:**
- **Configuration Changes**: See how panel configs changed over time
- **Query Evolution**: Track query changes across versions
- **Data Source Changes**: See when data sources were changed
- **Field Config Changes**: Track visualization setting changes

### 5.3 Version Comparison

Grafana allows comparing dashboard versions:

**Comparison Features:**
- **Text Diff**: Human-readable description of changes
- **JSON Diff**: Raw JSON diff for technical analysis
- **Panel-Level Changes**: See which panels changed
- **Query Changes**: See query modifications

**Example Comparison:**
```
Version 4 → Version 5:
- Panel "CPU Usage" (ID: 1):
  - Query A: Changed from "cpu_usage" to "rate(cpu_usage_total[5m])"
  - Field config: Changed unit from "none" to "short"
- Added panel "Memory Usage" (ID: 2)
```

### 5.4 Panel Configuration Tracking

**What Gets Tracked:**
- **Panel Properties**: Title, type, position, size
- **Data Source References**: Data source type and UID
- **Query Definitions**: All queries in targets array
- **Field Configuration**: Display settings, units, thresholds
- **Panel Options**: Visualization-specific options
- **Transformations**: Data transformation configurations

**Tracking Mechanism:**
- **Automatic**: Every dashboard save creates new version
- **Complete**: Full panel configuration included
- **Immutable**: Versions never modified
- **Queryable**: Can search and filter versions

---

## 6. Query History

### 6.1 Query History Overview

**Query History** tracks queries executed in Explore mode:

**Storage:**
```typescript
interface QueryHistoryEntry {
  uid: string;                  // Unique identifier
  datasourceUid: string;        // Data source UID
  queries: Array<{
    refId: string;
    datasource: {
      type: string;
      uid: string;
    };
    // Query-specific fields
    expr?: string;
    query?: string;
  }>;
  createdAt: number;            // Timestamp
  createdBy: number;            // User ID
  starred: boolean;             // Starred status
  comment?: string;             // User comment
}
```

**Features:**
- **Query Tracking**: Records all queries executed in Explore
- **Reuse**: Add queries from history to panels
- **Comments**: Add comments to queries
- **Starring**: Mark important queries
- **Search**: Search query history

### 6.2 Query History vs. Panel Configuration

**Query History:**
- **Scope**: Explore mode queries only
- **Purpose**: Reuse queries across sessions
- **Storage**: Separate database table
- **Lifecycle**: Independent of dashboards

**Panel Configuration:**
- **Scope**: Dashboard panel queries
- **Purpose**: Panel configuration persistence
- **Storage**: Part of dashboard JSON
- **Lifecycle**: Tied to dashboard versions

**Relationship:**
- Queries from history can be added to panels
- Panel queries are not automatically added to history
- Query history provides query templates/reuse

---

## 7. Data Provenance Mechanisms

### 7.1 Complete Data Lineage

Grafana provides complete data lineage through multiple tracking mechanisms:

**Lineage Chain:**
```
Dashboard (UID, Version)
    ↓
Panel (ID, Type)
    ↓
Data Source Reference (Type, UID)
    ↓
Query Definition (refId, expr/query)
    ↓
Query Execution (requestId, startTime)
    ↓
DataFrame (refId, meta.executedQueryString)
    ↓
Rendered Visualization
```

**Tracking at Each Level:**
1. **Dashboard Level**: Dashboard UID, version, creation/modification timestamps
2. **Panel Level**: Panel ID, type, configuration JSON
3. **Data Source Level**: Data source type, UID, name
4. **Query Level**: Query string, refId, execution metadata
5. **Data Level**: DataFrame metadata, executed query string

### 7.2 Metadata Propagation

Metadata flows through the entire data pipeline:

```typescript
// 1. Query Request
const request: DataQueryRequest = {
  requestId: "query-1704067200000-abc123",
  targets: [{
    refId: "A",
    datasource: { type: "prometheus", uid: "prometheus-uid" },
    expr: "rate(http_requests_total[5m])",
  }],
  // ... other parameters
};

// 2. Data Source Execution
const response = await datasource.query(request).toPromise();

// 3. DataFrame Creation (in data source plugin)
const frame: DataFrame = {
  refId: "A",
  fields: [...],
  meta: {
    executedQueryString: "rate(http_requests_total[5m])",
    requestId: "query-1704067200000-abc123",
    datasource: {
      type: "prometheus",
      uid: "prometheus-uid",
      name: "Prometheus Production",
    },
  },
};

// 4. Panel Receives DataFrame
panel.data = [frame];

// 5. Panel Inspector Can Display
// - Query: request + response
// - Metadata: frame.meta
// - Data: frame.fields
```

### 7.3 Provenance Information Available

**At Runtime (Panel Inspector):**
- **Query Request**: Exact query sent to data source
- **Query Response**: Raw response from data source
- **Executed Query**: Query after variable replacement
- **Data Source**: Type, UID, name
- **Timing**: Query time, processing time
- **Data**: Raw data values

**In Configuration (Panel JSON):**
- **Panel Configuration**: Complete panel JSON
- **Data Source Reference**: Type and UID
- **Query Definitions**: All queries in targets
- **Field Configuration**: Display settings

**In History (Dashboard Versions):**
- **Configuration Evolution**: How panel config changed
- **Query Evolution**: How queries changed
- **Data Source Changes**: When data sources changed
- **Timeline**: When changes occurred

---

## 8. Understanding Rendered Data Origins

### 8.1 Data Origin Inspection Workflow

**Step 1: Panel Inspector → Data Tab**
- View raw data returned by queries
- See data frame structure
- Identify data source from metadata

**Step 2: Panel Inspector → Query Tab**
- See exact query request sent
- See query response received
- View executed query string (after variable replacement)
- See data source information

**Step 3: Panel Inspector → JSON Tab**
- View panel JSON (configuration)
- View panel data JSON (processed data)
- View DataFrame structure JSON (raw data structure)

**Step 4: Query Editor → Query Inspector**
- Inspect individual queries
- See query performance
- Validate query syntax

**Step 5: Dashboard Settings → Versions**
- View panel configuration history
- Compare panel configurations across versions
- See when queries/data sources changed

### 8.2 Data Origin Information Available

**For Each Data Point:**
1. **Query Origin**: Which query produced the data (refId)
2. **Data Source**: Which data source executed the query
3. **Executed Query**: Actual query executed (after variable replacement)
4. **Time Range**: Time range used for query
5. **Variables**: Template variable values used
6. **Transformations**: Transformations applied to data
7. **Field Configuration**: Display settings applied

**Example Data Origin Trace:**
```
Data Point: 45.2 at 2024-01-01 12:00:00

Origin:
- Panel: "CPU Usage" (ID: 1)
- Query: refId "A"
- Data Source: Prometheus (UID: prometheus-uid)
- Executed Query: "rate(cpu_usage_total{host=\"server1\"}[5m])"
- Original Query: "rate(cpu_usage_total{host=\"$host\"}[5m])"
- Variable Replacement: $host → "server1"
- Time Range: 2024-01-01 11:55:00 to 2024-01-01 12:00:00
- Transformations: None
- Field Config: Unit: "short", Decimals: 1
```

### 8.3 Data Lineage Visualization

**Complete Lineage:**
```
Dashboard: "Production Overview" (UID: dashboard-uid, Version: 5)
    ↓
Panel: "CPU Usage" (ID: 1, Type: graph)
    ↓
Data Source: Prometheus (UID: prometheus-uid, Name: "Prometheus Production")
    ↓
Query: refId "A"
  - Original: "rate(cpu_usage_total{host=\"$host\"}[5m])"
  - Variables: { host: "server1" }
  - Executed: "rate(cpu_usage_total{host=\"server1\"}[5m])"
    ↓
Request: requestId "query-1704067200000-abc123"
  - Time Range: 2024-01-01 11:55:00 to 2024-01-01 12:00:00
  - Interval: 30s
  - Max Data Points: 1000
    ↓
Response: DataFrame
  - refId: "A"
  - Fields: time, value
  - Meta: {
      executedQueryString: "rate(cpu_usage_total{host=\"server1\"}[5m])",
      datasource: { type: "prometheus", uid: "prometheus-uid" },
      requestId: "query-1704067200000-abc123"
    }
    ↓
Transformations: None
    ↓
Field Config: Unit "short", Decimals 1
    ↓
Rendered: Time Series Chart
```

---

## 9. Tracking Mechanisms Summary

### 9.1 Query Origin Tracking

| Mechanism | Location | Information Tracked |
|-----------|----------|-------------------|
| **requestId** | DataQueryRequest | Unique request identifier |
| **refId** | Query target, DataFrame | Links queries to results |
| **executedQueryString** | DataFrameMeta | Actual query executed |
| **startTime** | DataQueryRequest | Query start timestamp |
| **Query Inspector** | Runtime UI | Request/response inspection |

### 9.2 Data Source Reference Tracking

| Mechanism | Location | Information Tracked |
|-----------|----------|-------------------|
| **Panel datasource** | Panel JSON | Panel-level data source |
| **Target datasource** | Query target | Query-level data source |
| **DataFrameMeta.datasource** | DataFrame metadata | Data source in response |
| **Data Source Resolution** | Runtime | Resolved data source instance |

### 9.3 Panel Configuration History

| Mechanism | Location | Information Tracked |
|-----------|----------|-------------------|
| **Dashboard Versions** | Database | Complete dashboard snapshots |
| **Version Comparison** | UI | Changes between versions |
| **Panel JSON** | Dashboard JSON | Complete panel configuration |
| **Version Metadata** | Version table | Creation time, user, message |

### 9.4 Data Provenance

| Mechanism | Location | Information Tracked |
|-----------|----------|-------------------|
| **Panel Inspector** | Runtime UI | Query, data, metadata |
| **Query Inspector** | Query editor | Request/response details |
| **DataFrame Metadata** | DataFrame | Executed query, data source |
| **Query History** | Database | Explore queries for reuse |

---

## 10. Practical Use Cases

### 10.1 Debugging Data Issues

**Scenario**: Data looks incorrect in panel

**Investigation Steps:**
1. **Panel Inspector → Data Tab**: View raw data values
2. **Panel Inspector → Query Tab**: See executed query
3. **Panel Inspector → JSON Tab**: View panel configuration
4. **Query Inspector**: Inspect individual queries
5. **Dashboard Versions**: Check if query changed recently

**Information Gathered:**
- What query was executed
- What data source was used
- What data was returned
- How data was transformed
- When configuration changed

### 10.2 Understanding Data Lineage

**Scenario**: Need to understand where data comes from

**Investigation Steps:**
1. **Panel Inspector → Query Tab**: See query and data source
2. **Panel JSON**: View complete panel configuration
3. **DataFrame Metadata**: See executed query string
4. **Dashboard Versions**: See query evolution

**Information Gathered:**
- Data source used
- Query executed
- Variable values used
- Transformations applied
- Configuration history

### 10.3 Auditing Data Source Usage

**Scenario**: Need to find all panels using a data source

**Investigation Steps:**
1. **Dashboard JSON**: Search for data source UID
2. **Panel Inspector**: Check data source in metadata
3. **Query Inspector**: See data source in query requests
4. **API Queries**: Use API to find dashboards by data source

**Information Gathered:**
- Which dashboards use data source
- Which panels use data source
- Which queries use data source
- How data source is configured

### 10.4 Tracking Configuration Changes

**Scenario**: Need to see how panel configuration changed

**Investigation Steps:**
1. **Dashboard Settings → Versions**: View version history
2. **Version Comparison**: Compare versions
3. **JSON Diff**: See raw JSON changes
4. **Text Diff**: See human-readable changes

**Information Gathered:**
- When configuration changed
- What changed (queries, data sources, settings)
- Who made changes
- Why changes were made (commit message)

---

## 11. Summary

Grafana provides comprehensive tracking of query origins, data source references, and panel configuration history through:

1. **Query Origin Tracking**:
   - requestId for unique request identification
   - refId for linking queries to results
   - executedQueryString in DataFrame metadata
   - Query Inspector for runtime inspection

2. **Data Source Reference Tracking**:
   - Panel-level and query-level data source references
   - Data source information in DataFrame metadata
   - Data source resolution at query time

3. **Panel Configuration History**:
   - Dashboard version history with complete snapshots
   - Version comparison for change tracking
   - Panel JSON in each version

4. **Data Provenance Mechanisms**:
   - Panel Inspector for runtime inspection
   - Query Inspector for query debugging
   - DataFrame metadata for query tracking
   - Complete data lineage from dashboard to visualization

These mechanisms work together to provide:
- **Full Traceability**: Know exactly where data comes from
- **Debugging Capabilities**: Understand query execution
- **Configuration History**: Track changes over time
- **Data Lineage**: Complete provenance from source to visualization

The system enables users to understand, debug, and audit their dashboards with complete visibility into query origins, data source usage, and configuration evolution.


