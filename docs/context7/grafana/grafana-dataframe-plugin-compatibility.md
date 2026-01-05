# Grafana DataFrame Representation and Plugin Compatibility Architecture

## Executive Summary

Grafana uses a flexible, schema-based data representation system built around **DataFrames** (columnar data structures) with **field schemas** and **metadata** to enable compatibility between queries and visualizations without rigid typing. The system relies on **structural validation** (field types, names, data shapes), **metadata hints** (preferredVisualizationType, preferredVisualisationPluginId), and **runtime compatibility checks** rather than compile-time type enforcement, allowing plugins to work with diverse data sources while maintaining data integrity.

---

## 1. DataFrame: The Unified Data Structure

### 1.1 DataFrame Architecture

**DataFrame** is Grafana's unified data structure that represents all query results, regardless of source:

```typescript
interface DataFrame {
  // Frame identification
  name?: string;            // Frame name (optional)
  refId?: string;           // Query reference ID (A, B, C, etc.)
  
  // Data structure
  fields: Field[];          // Array of fields (columns)
  length: number;           // Number of rows
  
  // Metadata
  meta?: DataFrameMeta;     // Frame-level metadata
  
  // Labels (for time series)
  labels?: Labels;          // Key-value pairs
}
```

**Key Characteristics:**
- **Columnar Structure**: Data organized by columns (fields), not rows
- **Unified Format**: Same structure for time series, tables, logs, traces
- **Flexible Schema**: Fields can have different types and structures
- **Metadata Rich**: Contains hints for visualization and processing

### 1.2 Columnar vs. Row-Based

**Columnar Structure (Grafana's Approach):**
```
Time Series Example:
Fields: [
  { name: "time", values: [t1, t2, t3, ...] },
  { name: "value", values: [v1, v2, v3, ...] }
]

Table Example:
Fields: [
  { name: "host", values: ["server1", "server2", ...] },
  { name: "cpu", values: [45, 67, ...] },
  { name: "memory", values: [32, 48, ...] }
]
```

**Benefits:**
- **Efficient Processing**: Operations on entire columns
- **Memory Efficiency**: Better cache locality
- **Type Safety**: Each field has a consistent type
- **Transformations**: Easy to apply column-wise operations

---

## 2. Field Schema and Types

### 2.1 Field Structure

Each field in a DataFrame has a well-defined schema:

```typescript
interface Field {
  // Field identification
  name: string;             // Field name (e.g., "time", "value", "host")
  
  // Type information
  type: FieldType;          // Field type (time, number, string, boolean, etc.)
  
  // Data values
  values: Vector<T>;        // Field values (array-like, type T)
  
  // Labels (for time series)
  labels?: Labels;          // Key-value pairs (e.g., {instance: "server1"})
  
  // Configuration
  config?: FieldConfig;      // Display configuration (unit, decimals, etc.)
  
  // Display name
  displayName?: string;      // Display name (may differ from name)
}
```

### 2.2 Field Types

Grafana defines a set of **FieldType** enums that represent the semantic type of data:

```typescript
enum FieldType {
  // Time types
  time = 'time',            // Timestamp values (milliseconds since epoch)
  
  // Numeric types
  number = 'number',        // Numeric values (integer or float)
  
  // String types
  string = 'string',        // Text values
  other = 'other',          // Other/unknown types
  
  // Boolean
  boolean = 'boolean',      // True/false values
  
  // Special types
  enum = 'enum',            // Enumerated values
  trace = 'trace',         // Trace data
  geo = 'geo',             // Geographic data
}
```

**Type Semantics:**
- **time**: Represents timestamps (always in milliseconds since Unix epoch)
- **number**: Represents numeric values (integers or floats)
- **string**: Represents text data
- **boolean**: Represents true/false values
- **other**: Catch-all for unknown or complex types

### 2.3 Field Type Examples

**Time Series Field:**
```typescript
{
  name: "time",
  type: FieldType.time,
  values: [1609459200000, 1609459260000, 1609459320000],  // Milliseconds
  config: {
    unit: undefined,  // Time doesn't have units
  }
}

{
  name: "value",
  type: FieldType.number,
  values: [1.5, 1.6, 1.7],
  labels: { instance: "server1", job: "api" },
  config: {
    unit: "short",  // Auto-format units
    decimals: 2,
  }
}
```

**Table Field:**
```typescript
{
  name: "host",
  type: FieldType.string,
  values: ["server1", "server2", "server3"],
  config: {
    displayName: "Hostname",
  }
}

{
  name: "cpu_usage",
  type: FieldType.number,
  values: [45.2, 67.8, 32.1],
  config: {
    unit: "percent",
    decimals: 1,
    min: 0,
    max: 100,
  }
}
```

**Log Field:**
```typescript
{
  name: "time",
  type: FieldType.time,
  values: [1609459200000, 1609459201000],
}

{
  name: "msg",
  type: FieldType.string,
  values: [
    "level=error msg=connection failed",
    "level=info msg=request processed"
  ],
}
```

### 2.4 Field Configuration

**FieldConfig** controls how fields are displayed and processed:

```typescript
interface FieldConfig {
  // Display
  displayName?: string;      // Custom display name
  unit?: string;             // Unit (bytes, percent, short, etc.)
  decimals?: number;         // Decimal places
  
  // Value mapping
  mappings?: ValueMapping[]; // Value to text/color mappings
  
  // Thresholds
  thresholds?: ThresholdsConfig;  // Color thresholds
  
  // Custom options (visualization-specific)
  custom?: Record<string, any>;
}
```

**Example Field Config:**
```typescript
{
  name: "status",
  type: FieldType.number,
  values: [0, 1, 2],
  config: {
    displayName: "Status",
    mappings: [
      { type: "value", options: { 0: { text: "OK", color: "green" } } },
      { type: "value", options: { 1: { text: "Warning", color: "yellow" } } },
      { type: "value", options: { 2: { text: "Error", color: "red" } } },
    ],
  }
}
```

---

## 3. DataFrame Metadata

### 3.1 Metadata Structure

**DataFrameMeta** contains frame-level metadata that provides hints and context:

```typescript
interface DataFrameMeta {
  // Visualization hints
  preferredVisualisationType?: string;        // Suggested visualization type
  preferredVisualisationPluginId?: string;   // Suggested plugin ID (experimental)
  
  // Query information
  executedQueryString?: string;               // Actual query executed
  custom?: Record<string, any>;              // Custom metadata
  
  // Data source information
  datasource?: {
    type: string;
    uid: string;
  };
  
  // Statistics
  stats?: Array<{
    displayName: string;
    value: number;
  }>;
  
  // Notices (warnings, errors)
  notices?: Array<{
    severity: 'info' | 'warning' | 'error';
    text: string;
  }>;
}
```

### 3.2 Visualization Hints

**preferredVisualisationType** suggests which visualization type to use:

```typescript
// Time series data
frame.meta = {
  preferredVisualisationType: 'timeseries',
};

// Table data
frame.meta = {
  preferredVisualisationType: 'table',
};

// Log data
frame.meta = {
  preferredVisualisationType: 'logs',
};

// Node graph data
frame.meta = {
  preferredVisualisationType: 'nodeGraph',
};
```

**preferredVisualisationPluginId** (experimental) suggests a specific plugin:

```typescript
frame.meta = {
  preferredVisualisationPluginId: 'grafana-piechart-panel',
};
```

**Use Cases:**
- **Data Source Hints**: Data sources can suggest appropriate visualizations
- **Explore Mode**: Automatically selects visualization in Explore
- **Panel Suggestions**: Grafana suggests panel types based on data

### 3.3 Metadata Examples

**Prometheus Query Result:**
```typescript
{
  name: "http_requests_total",
  fields: [
    { name: "time", type: FieldType.time, values: [...] },
    { name: "value", type: FieldType.number, values: [...] },
  ],
  meta: {
    preferredVisualisationType: "timeseries",
    executedQueryString: "rate(http_requests_total[5m])",
    datasource: {
      type: "prometheus",
      uid: "prometheus-uid",
    },
  },
}
```

**SQL Query Result:**
```typescript
{
  name: "users",
  fields: [
    { name: "time", type: FieldType.time, values: [...] },
    { name: "region", type: FieldType.string, values: [...] },
    { name: "count", type: FieldType.number, values: [...] },
  ],
  meta: {
    preferredVisualisationType: "table",
    executedQueryString: "SELECT time, region, COUNT(*) as count FROM users GROUP BY time, region",
    datasource: {
      type: "postgres",
      uid: "postgres-uid",
    },
  },
}
```

**Loki Log Query Result:**
```typescript
{
  name: "logs",
  fields: [
    { name: "time", type: FieldType.time, values: [...] },
    { name: "msg", type: FieldType.string, values: [...] },
    { name: "level", type: FieldType.string, values: [...] },
  ],
  meta: {
    preferredVisualisationType: "logs",
    executedQueryString: '{job="api"} |= "error"',
    datasource: {
      type: "loki",
      uid: "loki-uid",
    },
  },
}
```

---

## 4. Visualization Plugin Validation

### 4.1 Plugin Data Input

Visualization plugins receive data through **PanelProps**:

```typescript
interface PanelProps<T = {}> {
  // Data
  data: DataFrame[];        // Array of data frames
  
  // Configuration
  options: T;              // Panel-specific options
  fieldConfig: FieldConfigSource;  // Field configuration
  
  // Context
  timeRange: TimeRange;     // Current time range
  width: number;           // Panel width
  height: number;          // Panel height
  
  // Utilities
  replaceVariables: (value: string) => string;
  onOptionsChange: (options: T) => void;
}
```

### 4.2 Validation Patterns

Plugins validate data using **structural checks** rather than type checking:

#### Pattern 1: Field Type Validation

```typescript
export class TimeSeriesPanel extends PureComponent<PanelProps<TimeSeriesOptions>> {
  render() {
    const { data } = this.props;
    
    // Get first data frame
    const frame = data.series[0];
    if (!frame) {
      return <div>No data</div>;
    }
    
    // Validate required fields
    const timeField = frame.fields.find(f => f.type === FieldType.time);
    const valueField = frame.fields.find(f => f.type === FieldType.number);
    
    if (!timeField || !valueField) {
      return <div>Invalid data: requires time and number fields</div>;
    }
    
    // Render visualization
    return <TimeSeriesChart time={timeField.values} values={valueField.values} />;
  }
}
```

#### Pattern 2: Field Name Validation

```typescript
export class TablePanel extends PureComponent<PanelProps<TableOptions>> {
  render() {
    const { data } = this.props;
    
    const frame = data.series[0];
    if (!frame) {
      return <div>No data</div>;
    }
    
    // Validate field names (if required)
    const requiredFields = ['host', 'cpu', 'memory'];
    const fieldNames = frame.fields.map(f => f.name);
    const missingFields = requiredFields.filter(f => !fieldNames.includes(f));
    
    if (missingFields.length > 0) {
      return <div>Missing required fields: {missingFields.join(', ')}</div>;
    }
    
    // Render table
    return <Table data={frame} />;
  }
}
```

#### Pattern 3: Data Shape Validation

```typescript
export class NodeGraphPanel extends PureComponent<PanelProps<NodeGraphOptions>> {
  render() {
    const { data } = this.props;
    
    // Node graph requires edges frame
    const edgesFrame = data.series.find(
      frame => frame.meta?.preferredVisualisationType === 'nodeGraph' ||
               frame.name === 'edges'
    );
    
    if (!edgesFrame) {
      return <div>Node graph requires edges data frame</div>;
    }
    
    // Validate edges frame structure
    const sourceField = edgesFrame.fields.find(f => f.name === 'source');
    const targetField = edgesFrame.fields.find(f => f.name === 'target');
    
    if (!sourceField || !targetField) {
      return <div>Edges frame must have 'source' and 'target' fields</div>;
    }
    
    // Optional: nodes frame
    const nodesFrame = data.series.find(
      frame => frame.name === 'nodes'
    );
    
    // Render node graph
    return <NodeGraph edges={edgesFrame} nodes={nodesFrame} />;
  }
}
```

#### Pattern 4: Graceful Degradation

```typescript
export class StatPanel extends PureComponent<PanelProps<StatOptions>> {
  render() {
    const { data } = this.props;
    
    const frame = data.series[0];
    if (!frame) {
      return <StatDisplay value={null} text="No data" />;
    }
    
    // Try to find value field
    let valueField = frame.fields.find(f => f.type === FieldType.number);
    
    // Fallback: use first numeric field
    if (!valueField) {
      valueField = frame.fields.find(f => 
        f.type === FieldType.number || 
        typeof f.values[0] === 'number'
      );
    }
    
    // Fallback: use first field
    if (!valueField) {
      valueField = frame.fields[0];
    }
    
    // Get latest value
    const value = valueField?.values[valueField.values.length - 1];
    
    return <StatDisplay value={value} text={this.props.options.text} />;
  }
}
```

### 4.3 Validation Best Practices

**1. Check for Data Existence:**
```typescript
if (!data.series || data.series.length === 0) {
  return <div>No data available</div>;
}
```

**2. Validate Field Types:**
```typescript
const timeField = frame.fields.find(f => f.type === FieldType.time);
if (!timeField) {
  return <div>Time field required</div>;
}
```

**3. Handle Missing Fields Gracefully:**
```typescript
const valueField = frame.fields.find(f => f.name === 'value') ||
                   frame.fields.find(f => f.type === FieldType.number) ||
                   frame.fields[0];
```

**4. Validate Data Shape:**
```typescript
if (frame.length === 0) {
  return <div>Empty data frame</div>;
}
```

**5. Check Field Compatibility:**
```typescript
// Ensure time and value fields have same length
if (timeField.values.length !== valueField.values.length) {
  return <div>Data length mismatch</div>;
}
```

---

## 5. Compatibility Mechanisms Without Rigid Typing

### 5.1 Structural Compatibility

Grafana enforces compatibility through **structural validation** rather than type checking:

**Structural Requirements:**
- **Field Types**: Plugins check for required field types (time, number, etc.)
- **Field Names**: Plugins can check for specific field names (optional)
- **Data Shape**: Plugins validate data shape (length, structure)
- **Metadata Hints**: Plugins can use metadata for compatibility

**Example: Time Series Compatibility**
```typescript
// Plugin checks structure, not type
function isTimeSeriesCompatible(frame: DataFrame): boolean {
  // Check for time field
  const hasTime = frame.fields.some(f => f.type === FieldType.time);
  
  // Check for numeric field
  const hasNumber = frame.fields.some(f => f.type === FieldType.number);
  
  // Check data shape
  const hasData = frame.length > 0;
  
  return hasTime && hasNumber && hasData;
}
```

**Benefits:**
- **Flexible**: Works with any data source that produces compatible structure
- **Runtime Validation**: Errors caught at runtime, not compile time
- **Graceful Degradation**: Plugins can handle partial data

### 5.2 Metadata-Based Compatibility

**preferredVisualisationType** provides compatibility hints:

```typescript
// Data source sets hint
frame.meta = {
  preferredVisualisationType: 'timeseries',
};

// Grafana uses hint for panel suggestions
function suggestPanels(frames: DataFrame[]): string[] {
  const types = frames
    .map(f => f.meta?.preferredVisualisationType)
    .filter(Boolean);
  
  // Return compatible panel types
  return types;
}
```

**preferredVisualisationPluginId** (experimental) suggests specific plugins:

```typescript
// Data source suggests specific plugin
frame.meta = {
  preferredVisualisationPluginId: 'grafana-piechart-panel',
};

// Grafana can auto-select plugin in Explore
function selectPlugin(frames: DataFrame[]): string | null {
  return frames[0]?.meta?.preferredVisualisationPluginId || null;
}
```

### 5.3 Runtime Compatibility Checking

Grafana performs compatibility checks at **runtime**, not compile time:

**Panel Selection Process:**
```typescript
function isPanelCompatible(panelType: string, frames: DataFrame[]): boolean {
  // Get panel plugin
  const panel = getPanelPlugin(panelType);
  
  // Check if panel can handle data
  // This is done by the panel itself during render
  // Panel returns error/fallback if incompatible
  
  return true;  // Always allow, panel handles validation
}
```

**Panel Rendering with Validation:**
```typescript
function renderPanel(panel: PanelModel, data: DataFrame[]) {
  try {
    // Panel validates data during render
    const result = panel.render(data);
    return result;
  } catch (error) {
    // Panel handles incompatible data gracefully
    return <ErrorDisplay error={error} />;
  }
}
```

### 5.4 Field Type Coercion

Grafana allows **implicit type coercion** for compatibility:

**Number Coercion:**
```typescript
// String field with numeric values
const field = {
  name: "value",
  type: FieldType.string,
  values: ["1.5", "2.3", "3.7"],
};

// Plugin can coerce to number
const numericValues = field.values.map(v => parseFloat(v));
// Result: [1.5, 2.3, 3.7]
```

**Time Coercion:**
```typescript
// String field with timestamps
const field = {
  name: "time",
  type: FieldType.string,
  values: ["2024-01-01T00:00:00Z", "2024-01-01T01:00:00Z"],
};

// Plugin can coerce to time
const timeValues = field.values.map(v => new Date(v).getTime());
// Result: [1704067200000, 1704070800000]
```

**Benefits:**
- **Flexibility**: Works with data sources that return strings instead of native types
- **User-Friendly**: No need to transform data manually
- **Plugin Responsibility**: Plugins handle coercion as needed

### 5.5 Multiple Data Frame Handling

Plugins can handle **multiple data frames** with different structures:

```typescript
export class MultiFramePanel extends PureComponent<PanelProps> {
  render() {
    const { data } = this.props;
    
    // Handle different frame types
    const timeSeriesFrames = data.series.filter(f => 
      f.fields.some(field => field.type === FieldType.time)
    );
    
    const tableFrames = data.series.filter(f =>
      !f.fields.some(field => field.type === FieldType.time)
    );
    
    // Render based on frame type
    if (timeSeriesFrames.length > 0) {
      return <TimeSeriesChart frames={timeSeriesFrames} />;
    } else if (tableFrames.length > 0) {
      return <Table frames={tableFrames} />;
    } else {
      return <div>No compatible data</div>;
    }
  }
}
```

---

## 6. Panel Suggestions and Auto-Selection

### 6.1 Panel Suggestions

Grafana suggests compatible panels based on data structure:

```typescript
function suggestPanels(frames: DataFrame[]): PanelSuggestion[] {
  const suggestions: PanelSuggestion[] = [];
  
  // Check metadata hints
  const preferredType = frames[0]?.meta?.preferredVisualisationType;
  if (preferredType) {
    suggestions.push({
      type: preferredType,
      reason: 'Data source suggestion',
      score: 100,
    });
  }
  
  // Analyze field structure
  const hasTimeField = frames.some(f => 
    f.fields.some(field => field.type === FieldType.time)
  );
  const hasNumberField = frames.some(f =>
    f.fields.some(field => field.type === FieldType.number)
  );
  
  if (hasTimeField && hasNumberField) {
    suggestions.push({
      type: 'timeseries',
      reason: 'Time series data detected',
      score: 90,
    });
  }
  
  if (!hasTimeField && hasNumberField) {
    suggestions.push({
      type: 'stat',
      reason: 'Numeric data without time',
      score: 80,
    });
  }
  
  // Check for table structure
  if (frames.some(f => f.fields.length > 2)) {
    suggestions.push({
      type: 'table',
      reason: 'Multiple fields detected',
      score: 70,
    });
  }
  
  return suggestions.sort((a, b) => b.score - a.score);
}
```

### 6.2 Explore Mode Auto-Selection

In Explore mode, Grafana auto-selects visualization based on data:

```typescript
function autoSelectVisualization(frames: DataFrame[]): string {
  // Check metadata hint
  const preferredPluginId = frames[0]?.meta?.preferredVisualisationPluginId;
  if (preferredPluginId) {
    return preferredPluginId;
  }
  
  // Check preferred type
  const preferredType = frames[0]?.meta?.preferredVisualisationType;
  if (preferredType) {
    return getDefaultPluginForType(preferredType);
  }
  
  // Analyze structure
  const suggestions = suggestPanels(frames);
  return suggestions[0]?.type || 'table';
}
```

---

## 7. Data Source to Visualization Flow

### 7.1 Complete Flow

```
Data Source Query
    ↓
Data Source Plugin
    ↓
Transform to DataFrame
    ↓
Set Metadata (preferredVisualisationType)
    ↓
Return DataQueryResponse
    ↓
Grafana Core
    ↓
Apply Transformations
    ↓
Apply Field Configurations
    ↓
Pass to Panel Plugin
    ↓
Panel Validates Data Structure
    ↓
Panel Renders Visualization
```

### 7.2 Data Source Responsibility

**Data sources set metadata hints:**

```typescript
// Prometheus data source
function queryPrometheus(query: string): DataFrame {
  const frame = new DataFrame({
    fields: [
      { name: "time", type: FieldType.time, values: [...] },
      { name: "value", type: FieldType.number, values: [...] },
    ],
    meta: {
      preferredVisualisationType: "timeseries",  // Hint for panels
      executedQueryString: query,
    },
  });
  
  return frame;
}

// SQL data source
function querySQL(query: string): DataFrame {
  const frame = new DataFrame({
    fields: [
      { name: "host", type: FieldType.string, values: [...] },
      { name: "cpu", type: FieldType.number, values: [...] },
      { name: "memory", type: FieldType.number, values: [...] },
    ],
    meta: {
      preferredVisualisationType: "table",  // Hint for panels
      executedQueryString: query,
    },
  });
  
  return frame;
}
```

### 7.3 Panel Plugin Responsibility

**Panels validate and render:**

```typescript
export class MyPanel extends PureComponent<PanelProps> {
  render() {
    const { data } = this.props;
    
    // 1. Validate data structure
    if (!this.isDataValid(data)) {
      return <ErrorDisplay message="Invalid data structure" />;
    }
    
    // 2. Extract required fields
    const fields = this.extractFields(data);
    
    // 3. Process data
    const processedData = this.processData(fields);
    
    // 4. Render visualization
    return <MyVisualization data={processedData} />;
  }
  
  private isDataValid(data: DataFrame[]): boolean {
    // Structural validation
    if (!data || data.length === 0) return false;
    
    const frame = data[0];
    if (!frame.fields || frame.fields.length === 0) return false;
    
    // Type validation
    const hasRequiredTypes = this.checkFieldTypes(frame);
    return hasRequiredTypes;
  }
  
  private extractFields(data: DataFrame[]): Field[] {
    // Extract fields based on type/name
    const frame = data[0];
    return frame.fields.filter(f => 
      f.type === FieldType.number || 
      f.type === FieldType.time
    );
  }
}
```

---

## 8. Benefits of Non-Rigid Typing

### 8.1 Flexibility

**Benefits:**
- **Any Data Source**: Works with any data source that produces DataFrames
- **Dynamic Compatibility**: Compatibility determined at runtime
- **Plugin Independence**: Plugins don't need to know about all data sources
- **Evolution**: System can evolve without breaking changes

### 8.2 Extensibility

**Benefits:**
- **New Data Sources**: Easy to add new data sources
- **New Visualizations**: Easy to add new visualization types
- **Custom Types**: Plugins can define custom field types
- **Metadata Extensions**: Metadata can be extended without breaking changes

### 8.3 User Experience

**Benefits:**
- **Panel Suggestions**: Users get helpful suggestions
- **Graceful Degradation**: Errors are user-friendly
- **Flexible Data**: Users can use any data source with any panel
- **No Type Errors**: No compile-time type errors to confuse users

### 8.4 Developer Experience

**Benefits:**
- **Simple API**: Plugins work with simple DataFrame structure
- **Runtime Validation**: Errors caught early with clear messages
- **No Type System**: No complex type system to learn
- **Documentation**: Clear documentation on expected structures

---

## 9. Summary

Grafana's data representation and compatibility system provides:

1. **Unified Data Structure**: DataFrame with field schemas and metadata
2. **Flexible Type System**: Field types (time, number, string, etc.) with runtime validation
3. **Rich Metadata**: Visualization hints and query information
4. **Structural Validation**: Plugins validate data structure, not types
5. **Compatibility Hints**: Metadata suggests compatible visualizations
6. **Runtime Checking**: Compatibility determined at runtime, not compile time
7. **Graceful Degradation**: Plugins handle incompatible data gracefully

This architecture enables:
- **Flexibility**: Any data source can work with any visualization
- **Extensibility**: Easy to add new data sources and visualizations
- **User Experience**: Helpful suggestions and clear error messages
- **Developer Experience**: Simple API without complex type systems

The system balances **flexibility** (works with diverse data) and **safety** (validates structure) without requiring rigid compile-time typing, making Grafana both powerful and accessible.


