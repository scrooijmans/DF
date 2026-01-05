# Grafana Data Refresh and Recomputation Lifecycle

## Executive Summary

Grafana's data refresh and recomputation system orchestrates a complex lifecycle from dashboard load through query execution, data frame generation, transformations, and panel rendering. The system supports both **manual refresh** (immediate, user-triggered) and **automatic refresh intervals** (scheduled, time-based), with intelligent query cancellation, caching, and interval calculation based on time range and panel dimensions.

---

## 1. Complete Lifecycle Overview

### 1.1 Lifecycle Stages

The complete data refresh lifecycle consists of the following stages:

```
Dashboard Load
    ↓
Dashboard Initialization
    ↓
Panel Discovery & Initialization
    ↓
Query Request Construction
    ↓
Query Execution (Parallel)
    ↓
Data Frame Generation
    ↓
Data Transformations
    ↓
Field Configuration Application
    ↓
Panel Rendering
    ↓
[Refresh Interval Timer / Manual Refresh]
    ↓
(Repeat from Query Request Construction)
```

### 1.2 Lifecycle Components

**1. Dashboard Load**
- Dashboard JSON loaded from database
- Schema migration applied if needed
- Dashboard model created

**2. Query Execution**
- Data source resolution
- Query request construction
- Parallel query execution
- Response handling

**3. Data Processing**
- Data frame generation
- Transformations applied
- Field configurations applied

**4. Panel Rendering**
- Visualization rendering
- Loading state management
- Error handling

**5. Refresh Management**
- Manual refresh triggers
- Automatic refresh intervals
- Query cancellation

---

## 2. Dashboard Load and Initialization

### 2.1 Dashboard Loading Process

When a dashboard is first loaded:

```typescript
// 1. Load dashboard from database
const dashboardDTO = await dashboardAPI.getDashboardDTO(uid);

// 2. Create dashboard model
const dashboard = new DashboardModel(dashboardDTO.dashboard, dashboardDTO.meta);

// 3. Initialize dashboard state
dashboard.initDashboard({
  // Time range from dashboard JSON or defaults
  time: dashboard.time || { from: 'now-6h', to: 'now' },
  
  // Refresh interval from dashboard JSON
  refresh: dashboard.refresh || '',
  
  // Template variables
  templating: dashboard.templating || { list: [] },
  
  // Panels array
  panels: dashboard.panels || [],
});
```

**Key Initialization Steps:**
1. **Dashboard JSON Parsing**: Dashboard configuration loaded
2. **Schema Migration**: Automatic migration to current schema version
3. **Time Range Setup**: Dashboard time range initialized
4. **Template Variable Resolution**: Variables resolved to actual values
5. **Panel Discovery**: All panels identified and initialized

### 2.2 Panel Initialization

Each panel is initialized with:

```typescript
// Panel initialization
panels.forEach(panel => {
  // 1. Panel configuration loaded
  const panelConfig = panel;
  
  // 2. Data source resolved
  const datasource = resolveDataSource(panel.datasource.uid);
  
  // 3. Queries extracted
  const queries = panel.targets || [];
  
  // 4. Panel state initialized
  panel.state = {
    loading: false,
    data: null,
    error: null,
    lastRefresh: null,
  };
  
  // 5. Panel registered for refresh
  registerPanelForRefresh(panel);
});
```

---

## 3. Query Request Construction

### 3.1 Query Request Structure

Before executing queries, Grafana constructs a `DataQueryRequest` for each panel:

```typescript
interface DataQueryRequest {
  // Query targets (queries to execute)
  targets: Array<{
    refId: string;           // Unique reference ID (A, B, C, etc.)
    datasource: {
      type: string;          // Data source type (prometheus, loki, etc.)
      uid: string;          // Data source UID
    };
    // Query-specific fields (varies by data source)
    expr?: string;          // Prometheus query expression
    query?: string;         // SQL query, etc.
    // ... other query fields
  }>;
  
  // Time range
  range: {
    from: DateTime;          // Start time
    to: DateTime;           // End time
    raw: {
      from: string;         // Raw time string (e.g., "now-6h")
      to: string;           // Raw time string (e.g., "now")
    };
  };
  
  // Query parameters
  interval: string;          // Query interval (e.g., "30s")
  intervalMs: number;        // Interval in milliseconds
  maxDataPoints: number;    // Maximum data points to return
  
  // Context
  scopedVars: object;        // Template variable values
  timezone: string;          // Timezone (e.g., "browser")
  app: string;               // Application context ("dashboard")
  
  // Request tracking
  requestId: string;         // Unique request ID for cancellation
  startTime: number;         // Request start timestamp
}
```

### 3.2 Interval Calculation

The query interval is calculated based on:

**Formula**: `interval = timeRange / maxDataPoints`

**Where:**
- **timeRange**: Dashboard time range duration
- **maxDataPoints**: Calculated from panel width in pixels

**Example Calculation:**
```
Time Range: 7 days (7d = 604,800 seconds)
Panel Width: 1000 pixels
Max Data Points: 1000

Interval = 604,800s / 1000 = 604.8s ≈ 10 minutes
```

**Interval Calculation Code:**
```typescript
function calculateInterval(timeRange: TimeRange, panelWidth: number): string {
  // Calculate max data points from panel width
  const maxDataPoints = Math.max(panelWidth, 100); // Minimum 100
  
  // Calculate time range in milliseconds
  const timeRangeMs = timeRange.to.valueOf() - timeRange.from.valueOf();
  const timeRangeSeconds = timeRangeMs / 1000;
  
  // Calculate interval
  const intervalSeconds = timeRangeSeconds / maxDataPoints;
  
  // Convert to human-readable format
  return formatInterval(intervalSeconds);
}
```

**Key Insight**: Wider panels and shorter time ranges result in more frequent data refreshes, as there are more pixels to fill with data points.

### 3.3 Request ID Generation

Each query request gets a unique `requestId` for cancellation tracking:

```typescript
const requestId = `query-${Date.now()}-${Math.random()}`;
```

This ID is used to:
- Track pending requests
- Cancel requests when refresh is triggered
- Identify request lifecycle

---

## 4. Query Execution

### 4.1 Parallel Query Execution

Grafana executes queries **in parallel** for all panels:

```typescript
async function executeAllQueries(dashboard: DashboardModel) {
  // Collect all queries from all panels
  const queryPromises = dashboard.panels.flatMap(panel => {
    return panel.targets.map(target => {
      return executeQuery(panel, target);
    });
  });
  
  // Execute all queries in parallel
  const results = await Promise.allSettled(queryPromises);
  
  // Process results
  results.forEach((result, index) => {
    if (result.status === 'fulfilled') {
      handleQuerySuccess(result.value);
    } else {
      handleQueryError(result.reason);
    }
  });
}
```

**Benefits of Parallel Execution:**
- **Faster Dashboard Load**: All panels query simultaneously
- **Better User Experience**: Dashboard appears faster
- **Resource Efficiency**: Better utilization of network and data source capacity

### 4.2 Data Source Query Execution

For each query, Grafana:

1. **Resolves Data Source**:
```typescript
const datasource = await getDataSourceSrv().get(target.datasource.uid);
```

2. **Constructs Query Request**:
```typescript
const request: DataQueryRequest = {
  targets: [target],
  range: dashboard.timeRange,
  interval: calculateInterval(dashboard.timeRange, panel.width),
  intervalMs: intervalToMs(interval),
  maxDataPoints: calculateMaxDataPoints(panel.width),
  scopedVars: resolveTemplateVariables(dashboard.templating),
  timezone: dashboard.timezone,
  app: 'dashboard',
  requestId: generateRequestId(),
  startTime: Date.now(),
};
```

3. **Executes Query**:
```typescript
// Data source plugin executes query
const response = await datasource.query(request).toPromise();
```

4. **Handles Response**:
```typescript
if (response.state === 'Done') {
  // Process data frames
  processDataFrames(response.data);
} else if (response.state === 'Error') {
  // Handle error
  handleQueryError(response.error);
} else if (response.state === 'Loading') {
  // Update loading state
  updatePanelLoadingState(panel, true);
}
```

### 4.3 Query Cancellation

Grafana implements **request cancellation** to prevent unnecessary queries:

**Cancellation Triggers:**
1. **Manual Refresh**: User clicks refresh button
2. **Auto Refresh**: Scheduled refresh interval triggers
3. **Time Range Change**: User changes dashboard time range
4. **Dashboard Navigation**: User navigates away from dashboard

**Cancellation Implementation:**

**Before Grafana 7.2:**
```typescript
// Request identified by requestId
const requestId = `query-${Date.now()}`;

// Cancel by requestId
backendSrv.cancelRequest(requestId);
```

**After Grafana 7.2 (RxJS-based):**
```typescript
// Query returns Observable
const queryObservable = datasource.query(request);

// Subscribe with cancellation support
const subscription = queryObservable.subscribe({
  next: (response) => handleResponse(response),
  error: (error) => handleError(error),
});

// Cancel when needed
subscription.unsubscribe();
```

**Cancellation Behavior:**
- **Pending Requests**: All pending requests are cancelled
- **In-Flight Requests**: Network requests are aborted if possible
- **Cleanup**: Resources are cleaned up, loading states reset

---

## 5. Data Frame Generation

### 5.1 Data Frame Structure

Data sources return data in **DataFrame** format, Grafana's unified data structure:

```typescript
interface DataFrame {
  // Frame identification
  name?: string;            // Frame name
  refId?: string;           // Query reference ID
  
  // Data fields (columns)
  fields: Array<Field>;
  
  // Metadata
  meta?: {
    executedQueryString?: string;  // Actual query executed
    preferredVisualisationPluginId?: string;  // Preferred visualization
    // ... other metadata
  };
  
  // Length (number of rows)
  length: number;
}
```

**Field Structure:**
```typescript
interface Field {
  name: string;             // Field name (e.g., "time", "value")
  type: FieldType;          // Field type (time, number, string, etc.)
  values: Vector;           // Field values (array-like)
  labels?: Labels;          // Labels (for time series)
  config?: FieldConfig;     // Field configuration
}
```

### 5.2 Data Source to Data Frame Transformation

Each data source plugin transforms its native data format into DataFrames:

**Example: Prometheus Data Source**
```typescript
// Prometheus returns time series
const prometheusResponse = {
  data: {
    result: [
      {
        metric: { instance: "server1", job: "api" },
        values: [[1609459200, "1.5"], [1609459260, "1.6"]]
      }
    ]
  }
};

// Transform to DataFrame
const frame = new DataFrame({
  name: "http_requests_total",
  fields: [
    {
      name: "time",
      type: FieldType.time,
      values: [1609459200000, 1609459260000]  // Convert to milliseconds
    },
    {
      name: "value",
      type: FieldType.number,
      values: [1.5, 1.6],
      labels: { instance: "server1", job: "api" }
    }
  ]
});
```

**Example: SQL Data Source**
```typescript
// SQL returns table rows
const sqlResponse = {
  rows: [
    { time: "2024-01-01T00:00:00Z", value: 100, region: "US" },
    { time: "2024-01-01T01:00:00Z", value: 110, region: "US" }
  ]
};

// Transform to DataFrame
const frame = new DataFrame({
  fields: [
    {
      name: "time",
      type: FieldType.time,
      values: [1704067200000, 1704070800000]
    },
    {
      name: "value",
      type: FieldType.number,
      values: [100, 110]
    },
    {
      name: "region",
      type: FieldType.string,
      values: ["US", "US"]
    }
  ]
});
```

### 5.3 Multiple Data Frames

A single query can return multiple data frames:

```typescript
// Query response contains multiple frames
const response: DataQueryResponse = {
  data: [
    frame1,  // First series
    frame2,  // Second series
    frame3,  // Third series
  ],
  state: 'Done',
};
```

**Use Cases:**
- **Multiple Series**: One query returns multiple time series
- **Multiple Metrics**: Different metrics in separate frames
- **Table Results**: Multiple result sets from complex queries

---

## 6. Data Transformations

### 6.1 Transformation Pipeline

After data frames are generated, they pass through a **transformation pipeline**:

```
Data Frames (Raw)
    ↓
Transformations (Optional)
    ↓
Field Configuration Application
    ↓
Data Frames (Processed)
    ↓
Panel Rendering
```

### 6.2 Available Transformations

Grafana provides various transformations:

**1. Filter Data**
- Filter rows based on conditions
- Filter fields/columns

**2. Group By**
- Group data by field values
- Aggregate grouped data

**3. Calculate Field**
- Add calculated fields
- Mathematical operations

**4. Join**
- Join multiple data frames
- Merge by time or key

**5. Partition by Values**
- Split data into multiple series
- Create separate visualizations

**6. Organize Fields**
- Rename fields
- Reorder fields
- Hide/show fields

### 6.3 Transformation Execution

Transformations are applied sequentially:

```typescript
// Apply transformations
let processedData = dataFrames;

panel.transformations.forEach(transformation => {
  processedData = applyTransformation(processedData, transformation);
});

// Example: Filter transformation
function applyFilterTransformation(frames: DataFrame[], config: FilterConfig): DataFrame[] {
  return frames.map(frame => {
    const filteredRows = frame.fields[0].values.filter((_, index) => {
      return evaluateFilterCondition(frame, index, config);
    });
    
    return createFilteredFrame(frame, filteredRows);
  });
}
```

**Transformation Order Matters:**
- Transformations are applied in the order they appear
- Each transformation operates on the output of the previous one
- Changing order can produce different results

---

## 7. Field Configuration Application

### 7.1 Field Configuration

Field configurations control how data is displayed:

```typescript
interface FieldConfig {
  // Display settings
  displayName?: string;      // Custom field name
  unit?: string;             // Unit (bytes, percent, etc.)
  decimals?: number;         // Decimal places
  
  // Value mappings
  mappings?: Array<ValueMapping>;  // Value to text mappings
  
  // Thresholds
  thresholds?: ThresholdsConfig;   // Color thresholds
  
  // Custom options (visualization-specific)
  custom?: Record<string, any>;
}
```

### 7.2 Field Overrides

Field overrides allow panel-specific field configuration:

```typescript
// Panel field config
panel.fieldConfig = {
  defaults: {
    unit: 'short',
    decimals: 2,
  },
  overrides: [
    {
      matcher: { id: 'byName', options: 'CPU' },
      properties: [
        { id: 'unit', value: 'percent' },
        { id: 'decimals', value: 1 },
      ],
    },
  ],
};
```

### 7.3 Field Configuration Application

Field configurations are applied to data frames:

```typescript
import { applyFieldOverrides } from '@grafana/data';

// Apply field configurations
const processedData = applyFieldOverrides({
  data: transformedDataFrames,
  fieldConfig: panel.fieldConfig,
  theme: currentTheme,
  replaceVariables: (value) => replaceTemplateVariables(value, dashboard.templating),
});
```

**Application Process:**
1. **Default Configuration**: Apply panel defaults to all fields
2. **Override Matching**: Match fields to override rules
3. **Override Application**: Apply matched overrides
4. **Variable Replacement**: Replace template variables in display names
5. **Unit Formatting**: Format values according to unit settings

---

## 8. Panel Rendering

### 8.1 Panel Rendering Process

Once data is processed, panels render the visualization:

```typescript
// Panel rendering
function renderPanel(panel: PanelModel, data: DataFrame[]) {
  // 1. Set loading state to false
  panel.state.loading = false;
  
  // 2. Update panel data
  panel.data = data;
  
  // 3. Trigger panel render
  panel.render();
}
```

### 8.2 Panel Component Structure

Panels use React components with specific props:

```typescript
interface PanelProps {
  // Data
  data: DataFrame[];        // Processed data frames
  timeRange: TimeRange;      // Current time range
  
  // Configuration
  options: PanelOptions;      // Panel-specific options
  fieldConfig: FieldConfig;  // Field configuration
  
  // Dimensions
  width: number;             // Panel width in pixels
  height: number;            // Panel height in pixels
  
  // Utilities
  replaceVariables: (value: string) => string;
  onOptionsChange: (options: PanelOptions) => void;
}
```

### 8.3 Loading States

Panels manage loading states during data fetching:

```typescript
// Loading state management
<PanelChrome
  title={panel.title}
  loadingState={panel.state.loading ? LoadingState.Loading : LoadingState.Done}
  statusMessage={panel.state.error?.message}
>
  {(innerWidth, innerHeight) => (
    <PanelComponent
      data={panel.data}
      width={innerWidth}
      height={innerHeight}
      options={panel.options}
    />
  )}
</PanelChrome>
```

**Loading States:**
- **Loading**: Data is being fetched
- **Streaming**: Data is streaming (live updates)
- **Done**: Data loaded, ready to render
- **Error**: Query failed, show error message

### 8.4 Rendering Performance

Grafana tracks panel rendering performance:

```typescript
// Performance metrics
const metrics = {
  panelId: panel.id,
  pluginId: panel.type,
  totalQueryTime: queryDuration,
  totalTransformationTime: transformationDuration,
  totalRenderTime: renderDuration,
  totalPanelTime: queryDuration + transformationDuration + renderDuration,
  isSlowPanel: totalPanelTime > 500, // 500ms threshold
};
```

---

## 9. Refresh Mechanisms

### 9.1 Manual Refresh

**Manual refresh** is triggered when the user clicks the refresh button:

```typescript
// Manual refresh handler
function handleManualRefresh() {
  // 1. Cancel all pending requests
  cancelAllPendingRequests();
  
  // 2. Reset panel loading states
  dashboard.panels.forEach(panel => {
    panel.state.loading = true;
    panel.state.error = null;
  });
  
  // 3. Trigger query execution
  executeAllQueries(dashboard);
}
```

**Manual Refresh Behavior:**
- **Immediate**: Queries execute immediately
- **Cancels Pending**: All pending requests are cancelled
- **Full Refresh**: All panels refresh simultaneously
- **User-Controlled**: Only happens when user clicks refresh

### 9.2 Automatic Refresh Intervals

**Automatic refresh** is scheduled based on dashboard refresh interval:

```typescript
// Auto refresh setup
function setupAutoRefresh(dashboard: DashboardModel) {
  const refreshInterval = parseRefreshInterval(dashboard.refresh);
  
  if (refreshInterval > 0) {
    // Set up interval timer
    const intervalId = setInterval(() => {
      handleAutoRefresh();
    }, refreshInterval);
    
    // Store interval ID for cleanup
    dashboard.autoRefreshIntervalId = intervalId;
  }
}

// Auto refresh handler
function handleAutoRefresh() {
  // Same as manual refresh, but triggered by timer
  cancelAllPendingRequests();
  executeAllQueries(dashboard);
}
```

**Refresh Interval Options:**
- **Off**: No automatic refresh (default)
- **5s, 10s, 30s, 1m, 5m, 15m, 30m, 1h, 2h, 1d**: Fixed intervals
- **Auto**: Calculated based on time range and panel width

### 9.3 Auto Refresh Calculation

When refresh interval is set to **"Auto"**:

```typescript
function calculateAutoRefreshInterval(dashboard: DashboardModel): number {
  // Get dashboard time range
  const timeRange = dashboard.timeRange;
  const timeRangeMs = timeRange.to.valueOf() - timeRange.from.valueOf();
  
  // Get average panel width
  const avgPanelWidth = calculateAveragePanelWidth(dashboard.panels);
  
  // Calculate optimal refresh interval
  // Shorter time ranges = more frequent refresh
  // Longer time ranges = less frequent refresh
  const refreshInterval = Math.max(
    timeRangeMs / 100,  // Refresh 100 times over time range
    MIN_REFRESH_INTERVAL // Minimum 5 seconds
  );
  
  return refreshInterval;
}
```

**Auto Refresh Logic:**
- **Short Time Ranges** (minutes/hours): Refresh frequently (every few seconds)
- **Long Time Ranges** (days/weeks): Refresh infrequently (every few minutes)
- **Rationale**: No need to refresh more often than pixels available to draw updates

**Example:**
```
Time Range: 1 hour
Auto Refresh: ~36 seconds (refresh 100 times over 1 hour)

Time Range: 7 days
Auto Refresh: ~10 minutes (refresh 100 times over 7 days)
```

### 9.4 Minimum Refresh Interval

Grafana enforces a **minimum refresh interval** to prevent excessive load:

```ini
[dashboards]
min_refresh_interval = "5s"  # Default: 5 seconds
```

**Behavior:**
- Refresh intervals below minimum are ignored
- Minimum applies to both manual and automatic refresh
- Prevents dashboard from refreshing too frequently
- Protects data sources from excessive query load

### 9.5 Refresh Interaction

**Manual and Automatic Refresh Interaction:**

1. **Manual Refresh During Auto Refresh**:
   - Manual refresh cancels pending auto refresh queries
   - Manual refresh executes immediately
   - Auto refresh timer continues (doesn't reset)

2. **Auto Refresh During Manual Refresh**:
   - Auto refresh cancels any pending manual refresh queries
   - Auto refresh executes on schedule
   - Manual refresh can be triggered again immediately

3. **Time Range Change**:
   - Changing time range cancels all pending queries
   - Triggers immediate refresh with new time range
   - Auto refresh continues with new time range

---

## 10. Complete Refresh Lifecycle Example

### 10.1 Initial Dashboard Load

```
1. User navigates to dashboard
   ↓
2. Dashboard JSON loaded from database
   ↓
3. Dashboard model initialized
   ↓
4. Panels discovered and initialized
   ↓
5. Time range set (from dashboard or default)
   ↓
6. Template variables resolved
   ↓
7. Query requests constructed for all panels
   ↓
8. All queries executed in parallel
   ↓
9. Data frames generated by data source plugins
   ↓
10. Transformations applied (if configured)
   ↓
11. Field configurations applied
   ↓
12. Panels rendered with data
   ↓
13. Auto refresh timer started (if configured)
```

### 10.2 Manual Refresh Flow

```
1. User clicks "Refresh dashboard" button
   ↓
2. All pending requests cancelled
   ↓
3. Panel loading states set to true
   ↓
4. Query requests constructed (with new requestId)
   ↓
5. All queries executed in parallel
   ↓
6. Data frames generated
   ↓
7. Transformations applied
   ↓
8. Field configurations applied
   ↓
9. Panels re-rendered with new data
   ↓
10. Loading states set to false
```

### 10.3 Auto Refresh Flow

```
1. Auto refresh timer fires (e.g., every 30 seconds)
   ↓
2. All pending requests cancelled
   ↓
3. Panel loading states set to true
   ↓
4. Query requests constructed
   ↓
5. All queries executed in parallel
   ↓
6. Data frames generated
   ↓
7. Transformations applied
   ↓
8. Field configurations applied
   ↓
9. Panels re-rendered with new data
   ↓
10. Loading states set to false
   ↓
11. Timer resets for next refresh
```

### 10.4 Time Range Change Flow

```
1. User changes dashboard time range
   ↓
2. All pending requests cancelled
   ↓
3. Dashboard time range updated
   ↓
4. Query intervals recalculated (new time range)
   ↓
5. Query requests constructed with new time range
   ↓
6. All queries executed in parallel
   ↓
7. Data frames generated
   ↓
8. Transformations applied
   ↓
9. Field configurations applied
   ↓
10. Panels re-rendered with new data
   ↓
11. Auto refresh interval recalculated (if "Auto")
```

---

## 11. Query Caching and Optimization

### 11.1 Query Caching

Grafana supports **query caching** to reduce data source load:

**Cache Behavior:**
- **Cache Key**: Based on query, time range, and interval
- **Cache TTL**: Based on panel interval calculation
- **Cache Hit**: Returns cached data if available
- **Cache Miss**: Queries data source and caches result

**Cache Calculation:**
```
Panel Interval: 10 minutes (from time range / max data points)
Cache TTL: 10 minutes

If query executed within 10 minutes:
  → Return cached data
Else:
  → Query data source
  → Cache new result
```

**Cache Benefits:**
- **Reduced Load**: Fewer queries to data sources
- **Faster Refresh**: Cached data returns instantly
- **Better Performance**: Less network traffic

### 11.2 Incremental Querying (Prometheus)

Prometheus data source supports **incremental querying** (beta):

**Traditional Querying:**
```
Every refresh: Query entire time range
  → Fetch all data from start to end
  → High data transfer
  → Slower response
```

**Incremental Querying:**
```
First refresh: Query entire time range
  → Cache data frames

Subsequent refreshes: Query only new data
  → Fetch data since last query
  → Merge with cached data
  → Lower data transfer
  → Faster response
```

**Use Case**: Live dashboards with "now-relative" time ranges

---

## 12. Performance Considerations

### 12.1 Query Performance

**Factors Affecting Query Performance:**
1. **Time Range**: Longer ranges = more data = slower queries
2. **Query Complexity**: Complex queries = slower execution
3. **Data Source Performance**: Backend performance impacts query time
4. **Network Latency**: Network speed affects query duration
5. **Concurrent Queries**: Too many parallel queries can overwhelm data source

### 12.2 Panel Performance

**Panel Performance Metrics:**
```typescript
{
  totalQueryTime: 150,        // ms
  totalTransformationTime: 20, // ms
  totalRenderTime: 30,       // ms
  totalPanelTime: 200,       // ms
  isSlowPanel: false,        // < 500ms threshold
}
```

**Optimization Strategies:**
1. **Reduce Time Range**: Shorter ranges = faster queries
2. **Limit Data Points**: Use maxDataPoints to limit returned data
3. **Optimize Queries**: Write efficient queries
4. **Use Caching**: Enable query caching
5. **Reduce Transformations**: Minimize transformation complexity

### 12.3 Dashboard Performance

**Dashboard Performance Factors:**
1. **Number of Panels**: More panels = more queries = slower load
2. **Refresh Interval**: Too frequent = excessive load
3. **Query Complexity**: Complex queries = slower execution
4. **Data Source Capacity**: Backend capacity limits performance

**Best Practices:**
- **Limit Panel Count**: Keep dashboards focused
- **Appropriate Refresh Intervals**: Don't refresh too frequently
- **Optimize Queries**: Write efficient queries
- **Use Caching**: Enable query caching
- **Monitor Performance**: Track slow panels

---

## 13. Summary

Grafana's data refresh and recomputation lifecycle is a sophisticated system that:

1. **Orchestrates Complex Pipeline**: From dashboard load through query execution, data processing, and panel rendering

2. **Supports Multiple Refresh Modes**:
   - **Manual Refresh**: Immediate, user-triggered
   - **Automatic Refresh**: Scheduled, time-based
   - **Auto Calculation**: Intelligent interval calculation

3. **Manages Query Lifecycle**:
   - Parallel query execution
   - Request cancellation
   - Error handling
   - Loading state management

4. **Processes Data Efficiently**:
   - Data frame generation
   - Transformations
   - Field configuration
   - Panel rendering

5. **Optimizes Performance**:
   - Query caching
   - Incremental querying
   - Interval calculation
   - Performance monitoring

The system balances **responsiveness** (fast dashboard loads), **efficiency** (minimal data source load), and **flexibility** (multiple refresh options) to provide a powerful and user-friendly dashboard experience.


