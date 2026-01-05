# Grafana Frontend Architecture: Dynamic Multi-Panel Chart Systems

## Executive Summary

Grafana's frontend architecture for dynamic multi-panel chart systems is built on React, TypeScript, and a centralized state management model. The system uses **DashboardModel** as the single source of truth, **PanelModel** for individual panel configuration, and **DataFrame** as the unified data structure. Panels are dynamically added/removed through React component lifecycle, data flows through a pipeline (fetch → transform → render), and shared axes/interactions are synchronized through prop-based communication. The architecture separates concerns cleanly: data fetching (data source plugins), data transformation (transformation plugins), and visualization rendering (panel plugins).

---

## 1. Core Internal Representations

### 1.1 Dashboard Structure

**DashboardModel** is the central state container:

```typescript
class DashboardModel {
  // Dashboard metadata
  id: number | null;
  uid: string;
  title: string;
  tags: string[];
  timezone: string;
  schemaVersion: number;
  version: number;
  
  // Shared state (single source of truth)
  time: TimeRange;              // Dashboard time range
  templating: {
    list: VariableModel[];      // Template variables
  };
  refresh: string;              // Refresh interval ("5s", "1m", "auto")
  
  // Panel collection
  panels: PanelModel[];         // Array of panel models
  
  // State management
  private eventBus: EventEmitter;
  
  // Methods
  updateTimeRange(newTimeRange: TimeRange): void;
  updateVariable(variableName: string, value: any): void;
  refreshDashboard(): void;
  addPanel(panel: PanelModel): void;
  removePanel(panelId: number): void;
}
```

**Key Characteristics:**
- **Single Source of Truth**: All shared state lives in DashboardModel
- **Immutable Updates**: State changes create new state objects
- **Event-Driven**: Changes emit events for subscribers
- **Panel Independence**: Panels don't reference each other directly

### 1.2 Panel Structure

**PanelModel** represents individual panels:

```typescript
interface PanelModel {
  // Panel identification
  id: number;                   // Unique panel ID
  type: string;                  // Panel plugin type ("graph", "stat", etc.)
  title: string;                 // Panel title
  
  // Data source reference (not embedded)
  datasource: {
    type: string;                // Data source type ("prometheus", "loki", etc.)
    uid: string;                 // Data source UID
  };
  
  // Query configuration
  targets: Array<{
    refId: string;               // Query reference ID ("A", "B", "C")
    datasource: {
      type: string;
      uid: string;
    };
    // Query-specific fields (varies by data source)
    expr?: string;              // Prometheus query
    query?: string;             // SQL query
    // ...
  }>;
  
  // Visualization configuration
  options: PanelOptions;         // Panel-specific options
  fieldConfig: FieldConfig;     // Field display configuration
  
  // Layout
  gridPos: {
    x: number;                   // Grid X position
    y: number;                   // Grid Y position
    w: number;                   // Width in grid units
    h: number;                   // Height in grid units
  };
  
  // Transformations
  transformations: DataTransformer[];  // Data transformations
  
  // Runtime state
  state: {
    loading: boolean;
    data: DataFrame[] | null;
    error: Error | null;
    lastRefresh: number | null;
  };
}
```

**Key Characteristics:**
- **Data Source Reference**: Panels reference data sources by UID, don't own them
- **Query Configuration**: Targets array defines queries to execute
- **Grid Layout**: Position and size in grid units (24-column grid)
- **Runtime State**: Loading, data, and error state managed per panel

### 1.3 Visualization Structure

**DataFrame** is the unified data structure:

```typescript
interface DataFrame {
  // Frame identification
  name?: string;
  refId?: string;               // Query reference ID
  
  // Data fields (columns)
  fields: Array<Field>;
  
  // Metadata
  meta?: {
    executedQueryString?: string;
    preferredVisualisationPluginId?: string;
    datasource?: {
      type: string;
      uid: string;
    };
  };
  
  // Length (number of rows)
  length: number;
}

interface Field {
  name: string;                  // Field name ("time", "value", etc.)
  type: FieldType;              // Field type (time, number, string, etc.)
  values: Vector;                // Field values (array-like)
  labels?: Labels;              // Labels (for time series)
  config?: FieldConfig;        // Field display configuration
}
```

**Key Characteristics:**
- **Unified Format**: All data sources transform to DataFrame format
- **Columnar Structure**: Fields are columns, length is row count
- **Immutable**: DataFrames are immutable once created
- **Type-Safe**: Field types are enforced

---

## 2. Dynamic Panel Management

### 2.1 Adding Panels at Runtime

**Process Flow:**

```typescript
// 1. User clicks "Add Panel" button
function handleAddPanel() {
  // Create new panel model
  const newPanel: PanelModel = {
    id: generatePanelId(),
    type: 'graph',  // Default panel type
    title: 'New Panel',
    datasource: {
      type: dashboard.defaultDatasource?.type || 'prometheus',
      uid: dashboard.defaultDatasource?.uid || '',
    },
    targets: [],
    options: getDefaultPanelOptions('graph'),
    fieldConfig: getDefaultFieldConfig(),
    gridPos: {
      x: 0,
      y: calculateNextYPosition(),
      w: 12,
      h: 8,
    },
    transformations: [],
    state: {
      loading: false,
      data: null,
      error: null,
      lastRefresh: null,
    },
  };
  
  // Add to dashboard
  dashboard.addPanel(newPanel);
  
  // Trigger React re-render
  setDashboardState({ ...dashboard });
}

// 2. DashboardModel.addPanel()
class DashboardModel {
  addPanel(panel: PanelModel): void {
    // Add to panels array
    this.panels.push(panel);
    
    // Emit event
    this.eventBus.emit('panelAdded', { panel });
    
    // Trigger refresh if dashboard is active
    if (this.isActive) {
      this.refreshDashboard();
    }
  }
}
```

**React Component Integration:**

```typescript
// Dashboard component renders panels
function DashboardComponent({ dashboard }: { dashboard: DashboardModel }) {
  const [panels, setPanels] = useState(dashboard.panels);
  
  // Subscribe to panel changes
  useEffect(() => {
    const handler = () => {
      setPanels([...dashboard.panels]);
    };
    
    dashboard.eventBus.on('panelAdded', handler);
    dashboard.eventBus.on('panelRemoved', handler);
    
    return () => {
      dashboard.eventBus.off('panelAdded', handler);
      dashboard.eventBus.off('panelRemoved', handler);
    };
  }, [dashboard]);
  
  return (
    <div className="dashboard">
      {panels.map(panel => (
        <PanelComponent
          key={panel.id}
          panel={panel}
          timeRange={dashboard.time}
          variables={dashboard.templating.list}
          onPanelChange={handlePanelChange}
        />
      ))}
    </div>
  );
}
```

### 2.2 Removing Panels at Runtime

```typescript
// User clicks "Remove Panel" button
function handleRemovePanel(panelId: number) {
  // Remove from dashboard
  dashboard.removePanel(panelId);
  
  // Trigger React re-render
  setDashboardState({ ...dashboard });
}

// DashboardModel.removePanel()
class DashboardModel {
  removePanel(panelId: number): void {
    // Find panel index
    const index = this.panels.findIndex(p => p.id === panelId);
    if (index === -1) return;
    
    // Remove panel
    const panel = this.panels[index];
    this.panels.splice(index, 1);
    
    // Emit event
    this.eventBus.emit('panelRemoved', { panel });
    
    // Cleanup panel resources
    cleanupPanel(panel);
  }
}
```

### 2.3 Reconfiguring Panels

```typescript
// User edits panel configuration
function handlePanelOptionsChange(panelId: number, options: PanelOptions) {
  // Find panel
  const panel = dashboard.panels.find(p => p.id === panelId);
  if (!panel) return;
  
  // Update panel options
  panel.options = { ...panel.options, ...options };
  
  // Trigger refresh
  dashboard.refreshDashboard();
}
```

---

## 3. Data Series Management

### 3.1 Adding Data Series (Queries)

**Process Flow:**

```typescript
// User adds query to panel
function handleAddQuery(panelId: number) {
  const panel = dashboard.panels.find(p => p.id === panelId);
  if (!panel) return;
  
  // Create new query target
  const newTarget = {
    refId: getNextRefId(panel.targets),  // "A", "B", "C", etc.
    datasource: panel.datasource,
    expr: '',  // Empty query (user will fill in)
  };
  
  // Add to panel targets
  panel.targets.push(newTarget);
  
  // Trigger query execution
  executePanelQueries(panel);
}

// Query execution
async function executePanelQueries(panel: PanelModel) {
  // Set loading state
  panel.state.loading = true;
  
  // Build query request
  const request: DataQueryRequest = {
    targets: panel.targets,
    range: dashboard.time,
    interval: calculateInterval(dashboard.time, panel.width),
    scopedVars: resolveVariables(dashboard.templating.list),
    // ...
  };
  
  // Resolve data source
  const datasource = await getDataSourceSrv().get(panel.datasource.uid);
  
  // Execute queries
  try {
    const response = await datasource.query(request).toPromise();
    
    // Update panel data
    panel.state.data = response.data;  // DataFrame[]
    panel.state.loading = false;
    panel.state.lastRefresh = Date.now();
  } catch (error) {
    panel.state.error = error;
    panel.state.loading = false;
  }
  
  // Trigger panel re-render
  forcePanelUpdate(panel);
}
```

### 3.2 Removing Data Series

```typescript
// User removes query from panel
function handleRemoveQuery(panelId: number, refId: string) {
  const panel = dashboard.panels.find(p => p.id === panelId);
  if (!panel) return;
  
  // Remove target
  panel.targets = panel.targets.filter(t => t.refId !== refId);
  
  // Re-execute queries (with remaining targets)
  executePanelQueries(panel);
}
```

### 3.3 Updating Data Series

```typescript
// User modifies query
function handleUpdateQuery(panelId: number, refId: string, query: any) {
  const panel = dashboard.panels.find(p => p.id === panelId);
  if (!panel) return;
  
  // Find and update target
  const target = panel.targets.find(t => t.refId === refId);
  if (target) {
    Object.assign(target, query);
    
    // Re-execute queries
    executePanelQueries(panel);
  }
}
```

---

## 4. Shared Axes and Synchronized Interactions

### 4.1 Time Axis Synchronization

**Implementation:**

```typescript
// DashboardModel manages shared time range
class DashboardModel {
  time: TimeRange;  // Shared across all panels
  
  updateTimeRange(newTimeRange: TimeRange): void {
    // Update state
    this.time = newTimeRange;
    
    // Emit event
    this.eventBus.emit('timeRangeChanged', {
      timeRange: newTimeRange,
    });
    
    // Trigger refresh (all panels re-query)
    this.refreshDashboard();
  }
}

// Dashboard component passes time range as prop
function DashboardComponent({ dashboard }: { dashboard: DashboardModel }) {
  const [timeRange, setTimeRange] = useState(dashboard.time);
  
  // Subscribe to time range changes
  useEffect(() => {
    const handler = (event: { timeRange: TimeRange }) => {
      setTimeRange(event.timeRange);
    };
    
    dashboard.eventBus.on('timeRangeChanged', handler);
    
    return () => {
      dashboard.eventBus.off('timeRangeChanged', handler);
    };
  }, [dashboard]);
  
  return (
    <div>
      {dashboard.panels.map(panel => (
        <PanelComponent
          key={panel.id}
          panel={panel}
          timeRange={timeRange}  // Shared time range passed as prop
          // ...
        />
      ))}
    </div>
  );
}

// Panel receives time range as prop
function PanelComponent({ panel, timeRange }: PanelProps) {
  // Use time range for query execution
  useEffect(() => {
    if (timeRange) {
      executeQueries(panel, timeRange);
    }
  }, [timeRange]);
  
  // Render with time range
  return (
    <TimeSeriesChart
      data={panel.state.data}
      timeRange={timeRange}  // Shared time range
    />
  );
}
```

### 4.2 Zoom Synchronization

**Implementation:**

```typescript
// Dashboard manages zoom state
class DashboardModel {
  zoom: {
    from: DateTime;
    to: DateTime;
  } | null = null;
  
  setZoom(from: DateTime, to: DateTime): void {
    this.zoom = { from, to };
    this.eventBus.emit('zoomChanged', { from, to });
  }
  
  clearZoom(): void {
    this.zoom = null;
    this.eventBus.emit('zoomCleared');
  }
}

// Panel handles zoom interaction
function PanelComponent({ panel, zoom, onZoomChange }: PanelProps) {
  const handleZoom = (from: DateTime, to: DateTime) => {
    // Notify dashboard of zoom
    onZoomChange(from, to);
  };
  
  return (
    <TimeSeriesChart
      data={panel.state.data}
      zoom={zoom}  // Shared zoom state
      onZoom={handleZoom}
    />
  );
}
```

### 4.3 Pan Synchronization

Similar to zoom, pan interactions update dashboard state and propagate to all panels:

```typescript
// Dashboard manages pan state
class DashboardModel {
  panOffset: number = 0;  // Time offset in milliseconds
  
  setPanOffset(offset: number): void {
    this.panOffset = offset;
    this.eventBus.emit('panChanged', { offset });
  }
}
```

### 4.4 Hover/Crosshair Synchronization

**Implementation:**

```typescript
// Dashboard manages cursor position
class DashboardModel {
  cursorTime: number | null = null;  // Timestamp of cursor position
  
  setCursorTime(timestamp: number | null): void {
    this.cursorTime = timestamp;
    this.eventBus.emit('cursorTimeChanged', { timestamp });
  }
}

// Dashboard component passes cursor time to all panels
function DashboardComponent({ dashboard }: { dashboard: DashboardModel }) {
  const [cursorTime, setCursorTime] = useState<number | null>(null);
  
  useEffect(() => {
    const handler = (event: { timestamp: number | null }) => {
      setCursorTime(event.timestamp);
    };
    
    dashboard.eventBus.on('cursorTimeChanged', handler);
    return () => dashboard.eventBus.off('cursorTimeChanged', handler);
  }, [dashboard]);
  
  return (
    <div>
      {dashboard.panels.map(panel => (
        <PanelComponent
          key={panel.id}
          panel={panel}
          cursorTime={cursorTime}  // Shared cursor time
          onCursorMove={(timestamp) => dashboard.setCursorTime(timestamp)}
        />
      ))}
    </div>
  );
}

// Panel renders crosshair at cursor time
function PanelComponent({ panel, cursorTime }: PanelProps) {
  return (
    <TimeSeriesChart
      data={panel.state.data}
      cursorTime={cursorTime}  // Render crosshair at cursor time
    />
  );
}
```

---

## 5. React Components and State Management

### 5.1 Component Hierarchy

```
DashboardComponent (React)
├── DashboardModel (State Container)
│   ├── TimeRangePicker
│   ├── VariableControls
│   └── PanelGrid
│       ├── PanelComponent (Panel 1)
│       │   ├── PanelChrome (Header, Menu)
│       │   └── PanelPlugin (Visualization)
│       ├── PanelComponent (Panel 2)
│       │   └── ...
│       └── PanelComponent (Panel N)
│           └── ...
```

### 5.2 Panel Component Structure

```typescript
interface PanelProps {
  // Panel configuration
  panel: PanelModel;
  
  // Shared state (from dashboard)
  timeRange: TimeRange;
  variables: VariableModel[];
  refreshInterval: string;
  cursorTime: number | null;
  zoom: { from: DateTime; to: DateTime } | null;
  
  // Dimensions
  width: number;
  height: number;
  
  // Callbacks
  onOptionsChange: (options: PanelOptions) => void;
  onCursorMove: (timestamp: number) => void;
  onZoomChange: (from: DateTime, to: DateTime) => void;
}

// Panel component implementation
class PanelComponent extends React.Component<PanelProps> {
  componentDidMount() {
    // Initial query execution
    this.executeQueries();
  }
  
  componentDidUpdate(prevProps: PanelProps) {
    // Check for prop changes that require re-query
    if (
      prevProps.timeRange !== this.props.timeRange ||
      prevProps.variables !== this.props.variables ||
      prevProps.panel.targets !== this.props.panel.targets
    ) {
      this.executeQueries();
    }
  }
  
  componentWillUnmount() {
    // Cleanup
    this.cancelPendingQueries();
  }
  
  executeQueries = async () => {
    const { panel, timeRange, variables } = this.props;
    
    // Build query request
    const request: DataQueryRequest = {
      targets: panel.targets,
      range: timeRange,
      scopedVars: resolveVariables(variables),
      // ...
    };
    
    // Execute
    const response = await this.dataSource.query(request).toPromise();
    
    // Update panel state
    panel.state.data = response.data;
    panel.state.loading = false;
    
    // Force re-render
    this.forceUpdate();
  };
  
  render() {
    const { panel, width, height, cursorTime } = this.props;
    const { data, loading, error } = panel.state;
    
    return (
      <PanelChrome
        title={panel.title}
        loading={loading}
        error={error}
      >
        <PanelPlugin
          data={data}
          width={width}
          height={height}
          options={panel.options}
          cursorTime={cursorTime}
        />
      </PanelChrome>
    );
  }
}
```

### 5.3 Panel Lifecycle Hooks

**Panel Plugin Lifecycle:**

```typescript
// Panel plugin definition
export const panelPlugin = new PanelPlugin<MyPanelOptions>(MyPanelComponent)
  .setPanelOptions(builder => {
    // Options editor configuration
    builder.addTextInput({
      path: 'title',
      name: 'Title',
      defaultValue: 'My Panel',
    });
  })
  .setDataSupport({
    annotations: true,
    alertStates: true,
  });

// Panel component with lifecycle hooks
class MyPanelComponent extends React.Component<PanelProps<MyPanelOptions>> {
  // Mount: Panel is added to dashboard
  componentDidMount() {
    // Initialize visualization library
    this.initChart();
  }
  
  // Update: Props changed (data, options, etc.)
  componentDidUpdate(prevProps: PanelProps) {
    if (prevProps.data !== this.props.data) {
      // Data changed, update visualization
      this.updateChart(this.props.data);
    }
    
    if (prevProps.options !== this.props.options) {
      // Options changed, update visualization
      this.updateChartOptions(this.props.options);
    }
  }
  
  // Unmount: Panel is removed from dashboard
  componentWillUnmount() {
    // Cleanup
    this.destroyChart();
  }
  
  render() {
    const { data, options, width, height } = this.props;
    
    return (
      <div ref={this.chartRef} style={{ width, height }} />
    );
  }
}
```

---

## 6. Data Pipeline Separation

### 6.1 Three-Stage Pipeline

Grafana separates data processing into three distinct stages:

```
1. Data Fetching (Data Source Plugins)
   ↓
2. Data Transformation (Transformation Plugins)
   ↓
3. Visualization Rendering (Panel Plugins)
```

### 6.2 Stage 1: Data Fetching

**Data Source Plugins** are responsible for fetching data:

```typescript
// Data source plugin interface
interface DataSourceApi<TQuery extends DataQuery = DataQuery> {
  // Query execution
  query(request: DataQueryRequest<TQuery>): Observable<DataQueryResponse>;
  
  // Data source metadata
  name: string;
  type: string;
  uid: string;
  
  // Test connection
  testDatasource(): Promise<TestDataSourceResponse>;
}

// Example: Prometheus data source
class PrometheusDataSource extends DataSourceApi<PrometheusQuery> {
  async query(request: DataQueryRequest<PrometheusQuery>): Promise<DataQueryResponse> {
    // 1. Build Prometheus query
    const promQuery = this.buildPrometheusQuery(request.targets);
    
    // 2. Execute HTTP request
    const response = await fetch(`${this.url}/api/v1/query_range`, {
      method: 'POST',
      body: JSON.stringify({
        query: promQuery,
        start: request.range.from.unix(),
        end: request.range.to.unix(),
        step: request.intervalMs / 1000,
      }),
    });
    
    // 3. Transform to DataFrame
    const data = await response.json();
    const frames = this.transformToDataFrames(data);
    
    // 4. Return DataFrame[]
    return { data: frames };
  }
  
  private transformToDataFrames(data: any): DataFrame[] {
    return data.data.result.map((series: any) => {
      return new DataFrame({
        name: series.metric.__name__,
        fields: [
          {
            name: 'time',
            type: FieldType.time,
            values: series.values.map(([t, _]) => t * 1000),  // Convert to ms
          },
          {
            name: 'value',
            type: FieldType.number,
            values: series.values.map(([_, v]) => parseFloat(v)),
            labels: series.metric,
          },
        ],
      });
    });
  }
}
```

**Key Characteristics:**
- **Source-Specific**: Each data source knows how to query its backend
- **DataFrame Output**: All data sources return DataFrame[]
- **Async**: Queries are asynchronous (Observable/Promise)
- **Error Handling**: Data sources handle connection errors

### 6.3 Stage 2: Data Transformation

**Transformation Plugins** process data between fetch and render:

```typescript
// Transformation plugin interface
interface DataTransformer {
  id: string;
  name: string;
  operator: (options: any) => (source: Observable<DataFrame[]>) => Observable<DataFrame[]>;
}

// Example: Filter transformation
export const filterTransformer: DataTransformer = {
  id: 'filter',
  name: 'Filter',
  operator: (options: FilterOptions) => {
    return (source: Observable<DataFrame[]>) => {
      return source.pipe(
        map((frames: DataFrame[]) => {
          return frames.filter(frame => {
            // Apply filter condition
            return evaluateFilter(frame, options);
          });
        })
      );
    };
  },
};

// Transformation pipeline execution
function applyTransformations(
  data: DataFrame[],
  transformations: DataTransformer[]
): DataFrame[] {
  let processed = data;
  
  for (const transformation of transformations) {
    // Apply transformation
    processed = transformation.operator(transformation.options)(of(processed)).pipe(
      first()
    ).toPromise();
  }
  
  return processed;
}
```

**Key Characteristics:**
- **Immutable**: Transformations create new DataFrames, don't mutate input
- **Composable**: Transformations can be chained
- **Reactive**: Uses RxJS Observables for streaming
- **Optional**: Panels can have zero or more transformations

### 6.4 Stage 3: Visualization Rendering

**Panel Plugins** render the final visualization:

```typescript
// Panel plugin receives processed data
class MyPanelComponent extends React.Component<PanelProps<MyOptions>> {
  render() {
    const { data, options, width, height } = this.props;
    
    // Extract data from DataFrames
    const series = data.map(frame => {
      const timeField = frame.fields.find(f => f.type === FieldType.time);
      const valueField = frame.fields.find(f => f.type === FieldType.number);
      
      return {
        time: timeField?.values.toArray() || [],
        values: valueField?.values.toArray() || [],
        name: frame.name,
      };
    });
    
    // Render visualization (read-only data)
    return (
      <div style={{ width, height }}>
        <MyChart
          series={series}
          options={options}
        />
      </div>
    );
  }
}
```

**Key Characteristics:**
- **Read-Only**: Panels receive immutable DataFrames
- **Visualization-Specific**: Each panel type renders differently
- **Options-Driven**: Panel options control visualization appearance
- **Responsive**: Panels adapt to width/height changes

---

## 7. Plugin Integration

### 7.1 Panel Plugin Integration

**Panel plugins** integrate into the rendering pipeline:

```typescript
// Panel plugin registration
export const panelPlugin = new PanelPlugin<MyPanelOptions>(MyPanelComponent)
  .setPanelOptions(builder => {
    // Options editor
    builder.addTextInput({
      path: 'title',
      name: 'Title',
    });
  })
  .setDataSupport({
    annotations: true,
  });

// Panel plugin receives data via props
interface PanelProps<TOptions = any> {
  data: DataFrame[];        // Processed data (after transformations)
  options: TOptions;        // Panel options
  timeRange: TimeRange;    // Shared time range
  width: number;
  height: number;
  // ...
}

// Panel component implementation
class MyPanelComponent extends React.Component<PanelProps<MyPanelOptions>> {
  render() {
    const { data, options } = this.props;
    // Render visualization
    return <MyVisualization data={data} options={options} />;
  }
}
```

### 7.2 Data Source Plugin Integration

**Data source plugins** integrate into the query pipeline:

```typescript
// Data source plugin registration
export const datasourcePlugin = new DataSourcePlugin<MyQuery>(MyDataSource)
  .setQueryEditor(MyQueryEditor)
  .setConfigEditor(MyConfigEditor);

// Data source executes queries
class MyDataSource extends DataSourceApi<MyQuery> {
  async query(request: DataQueryRequest<MyQuery>): Promise<DataQueryResponse> {
    // Execute query against backend
    const response = await this.executeQuery(request);
    
    // Transform to DataFrame[]
    const frames = this.transformToDataFrames(response);
    
    return { data: frames };
  }
}
```

### 7.3 Transformation Plugin Integration

**Transformation plugins** integrate into the transformation pipeline:

```typescript
// Transformation plugin registration
export const transformationPlugin: DataTransformer = {
  id: 'my-transformation',
  name: 'My Transformation',
  operator: (options: MyTransformationOptions) => {
    return (source: Observable<DataFrame[]>) => {
      return source.pipe(
        map((frames: DataFrame[]) => {
          // Transform frames
          return transformFrames(frames, options);
        })
      );
    };
  },
};
```

---

## 8. Mapping to Multi-Well Correlation Chart

### 8.1 Architecture Mapping

**Grafana Architecture → Well Log Architecture:**

| Grafana Concept | Well Log Equivalent | Mapping Notes |
|----------------|---------------------|---------------|
| **Dashboard** | **Well Correlation Chart** | Container for all tracks |
| **Panel** | **Track** | Each track is a panel |
| **Data Series (Query)** | **Curve** | Each curve is a data series |
| **Time Axis** | **Depth Axis** | Shared axis across tracks |
| **Time Range** | **Depth Range** | Shared depth domain |
| **Data Source** | **Well Data Source** | Source of well log data |
| **Panel Plugin** | **Track Visualization** | How tracks are rendered |
| **Transformation** | **Data Processing** | Curve filtering, scaling, etc. |

### 8.2 Implementation Example

**Well Correlation Chart Structure:**

```typescript
// Well correlation chart (like Dashboard)
class WellCorrelationChart {
  // Shared depth range (like time range)
  depthRange: {
    min: number;
    max: number;
  };
  
  // Tracks (like panels)
  tracks: TrackModel[];
  
  // Methods
  updateDepthRange(min: number, max: number): void;
  addTrack(track: TrackModel): void;
  removeTrack(trackId: string): void;
}

// Track model (like PanelModel)
interface TrackModel {
  id: string;
  title: string;
  curves: CurveModel[];  // Like targets/queries
  scale: 'linear' | 'logarithmic';
  width: number;
  position: number;
}

// Curve model (like query target)
interface CurveModel {
  id: string;
  mnemonic: string;
  data: DataFrame;  // Depth-indexed data
  color: string;
  linestyle: string;
}

// Track component (like PanelComponent)
function TrackComponent({ track, depthRange, cursorDepth }: TrackProps) {
  // Render track with curves
  return (
    <div className="track">
      <TrackHeader title={track.title} />
      <TrackVisualization
        curves={track.curves}
        depthRange={depthRange}  // Shared depth range
        cursorDepth={cursorDepth}  // Shared cursor
      />
    </div>
  );
}
```

### 8.3 Shared Depth Axis Implementation

```typescript
// Well correlation chart manages shared depth
class WellCorrelationChart {
  depthRange: { min: number; max: number };
  cursorDepth: number | null = null;
  
  updateDepthRange(min: number, max: number): void {
    this.depthRange = { min, max };
    this.eventBus.emit('depthRangeChanged', { min, max });
    this.refreshAllTracks();
  }
  
  setCursorDepth(depth: number | null): void {
    this.cursorDepth = depth;
    this.eventBus.emit('cursorDepthChanged', { depth });
  }
}

// Track component receives shared depth
function TrackComponent({ track, depthRange, cursorDepth }: TrackProps) {
  return (
    <TrackVisualization
      curves={track.curves}
      depthRange={depthRange}  // Shared depth range
      cursorDepth={cursorDepth}  // Shared cursor
      onDepthZoom={(min, max) => chart.updateDepthRange(min, max)}
      onCursorMove={(depth) => chart.setCursorDepth(depth)}
    />
  );
}
```

---

## 9. Key Source Files and Directories

### 9.1 Core Architecture Files

**Dashboard Management:**
- `packages/grafana-data/src/types/dashboard.ts` - DashboardModel, PanelModel types
- `public/app/features/dashboard/state/DashboardModel.ts` - DashboardModel implementation
- `public/app/features/dashboard/components/Dashboard.tsx` - Dashboard React component

**Panel Management:**
- `public/app/features/dashboard/components/Panel/Panel.tsx` - Panel component
- `public/app/features/dashboard/components/Panel/PanelChrome.tsx` - Panel wrapper
- `public/app/features/dashboard/state/PanelModel.ts` - PanelModel implementation

**Data Pipeline:**
- `packages/grafana-data/src/types/dataFrame.ts` - DataFrame interface
- `packages/grafana-data/src/transformations/` - Transformation plugins
- `public/app/features/transformers/` - Transformation UI

**Plugin System:**
- `packages/grafana-runtime/src/services/plugin/` - Plugin registry
- `packages/grafana-ui/src/components/PanelPlugin/` - Panel plugin base
- `packages/grafana-data/src/types/datasource.ts` - Data source API

### 9.2 State Management Files

**Shared State:**
- `public/app/features/dashboard/state/DashboardModel.ts` - Centralized state
- `public/app/features/dashboard/state/TimeModel.ts` - Time range management
- `public/app/features/variables/state/` - Template variable management

**Event System:**
- `packages/grafana-data/src/events/` - Event bus implementation
- `public/app/core/events.ts` - Event types

### 9.3 Rendering Files

**Panel Rendering:**
- `public/app/features/dashboard/components/Panel/PanelRenderer.tsx` - Panel renderer
- `public/app/features/panel/components/PanelPluginRenderer.tsx` - Plugin renderer

**Visualization:**
- `packages/grafana-ui/src/components/Visualizations/` - Built-in visualizations
- `public/app/plugins/panel/` - Panel plugin implementations

---

## 10. Call Stack and Data Flow

### 10.1 "User Adds Panel" → "Panel Renders Data" → "Interaction Updates Visualization"

**Complete Call Stack:**

```
┌─────────────────────────────────────────────────────────────┐
│ 1. USER ADDS PANEL                                           │
└─────────────────────────────────────────────────────────────┘
│
├─> User clicks "Add Panel" button
│   │
│   ├─> DashboardComponent.handleAddPanel()
│   │   │
│   │   ├─> createNewPanel()
│   │   │   ├─> new PanelModel({
│   │   │   │     id: generateId(),
│   │   │   │     type: 'graph',
│   │   │   │     targets: [],
│   │   │   │     options: defaultOptions,
│   │   │   │     gridPos: calculatePosition(),
│   │   │   │   })
│   │   │   │
│   │   │   └─> dashboard.addPanel(newPanel)
│   │   │       │
│   │   │       ├─> DashboardModel.addPanel(panel)
│   │   │       │   ├─> this.panels.push(panel)
│   │   │       │   └─> this.eventBus.emit('panelAdded', { panel })
│   │   │       │
│   │   │       └─> setDashboardState({ ...dashboard })
│   │   │
│   │   └─> React re-renders DashboardComponent
│   │       │
│   │       └─> DashboardComponent.render()
│   │           │
│   │           └─> dashboard.panels.map(panel =>
│   │               <PanelComponent
│   │                 key={panel.id}
│   │                 panel={panel}
│   │                 timeRange={dashboard.time}
│   │                 variables={dashboard.templating.list}
│   │               />
│   │             )
│   │
│   └─> PanelComponent.componentDidMount()
│       │
│       └─> PanelComponent.executeQueries()
│           │
│           └─> (See "Panel Renders Data" flow)

┌─────────────────────────────────────────────────────────────┐
│ 2. PANEL RENDERS DATA                                        │
└─────────────────────────────────────────────────────────────┘
│
├─> PanelComponent.executeQueries()
│   │
│   ├─> Build DataQueryRequest
│   │   ├─> request = {
│   │   │     targets: panel.targets,
│   │   │     range: dashboard.time,
│   │   │     interval: calculateInterval(),
│   │   │     scopedVars: resolveVariables(),
│   │   │   }
│   │   │
│   │   └─> Resolve data source
│   │       └─> datasource = await getDataSourceSrv().get(panel.datasource.uid)
│   │
│   ├─> Execute query
│   │   └─> response = await datasource.query(request).toPromise()
│   │       │
│   │       ├─> DataSourceApi.query()
│   │       │   ├─> Build backend query (data source specific)
│   │       │   ├─> Execute HTTP request / API call
│   │       │   └─> Transform response to DataFrame[]
│   │       │
│   │       └─> Return { data: DataFrame[] }
│   │
│   ├─> Apply transformations
│   │   └─> processedData = applyTransformations(response.data, panel.transformations)
│   │       │
│   │       └─> For each transformation:
│   │           └─> transformation.operator(options)(source)
│   │               └─> Returns new DataFrame[]
│   │
│   ├─> Apply field configurations
│   │   └─> finalData = applyFieldOverrides({
│   │         data: processedData,
│   │         fieldConfig: panel.fieldConfig,
│   │       })
│   │
│   ├─> Update panel state
│   │   └─> panel.state = {
│   │         data: finalData,
│   │         loading: false,
│   │         lastRefresh: Date.now(),
│   │       }
│   │
│   └─> PanelComponent.render()
│       │
│       └─> <PanelPlugin
│             data={panel.state.data}
│             options={panel.options}
│             width={width}
│             height={height}
│           />
│           │
│           └─> PanelPlugin.render()
│               │
│               └─> <VisualizationComponent
│                     data={data}
│                     options={options}
│                   />
│                   │
│                   └─> Visualization renders (Chart.js, D3, etc.)

┌─────────────────────────────────────────────────────────────┐
│ 3. INTERACTION UPDATES VISUALIZATION                       │
└─────────────────────────────────────────────────────────────┘
│
├─> User interacts with panel (zoom, pan, hover)
│   │
│   ├─> Example: User zooms on panel
│   │   │
│   │   ├─> PanelComponent.handleZoom(from, to)
│   │   │   │
│   │   │   └─> onZoomChange(from, to)  // Callback prop
│   │   │       │
│   │   │       └─> DashboardComponent.handleZoom(from, to)
│   │   │           │
│   │   │           └─> dashboard.setZoom(from, to)
│   │   │               │
│   │   │               ├─> DashboardModel.setZoom(from, to)
│   │   │               │   ├─> this.zoom = { from, to }
│   │   │               │   └─> this.eventBus.emit('zoomChanged', { from, to })
│   │   │               │
│   │   │               └─> setDashboardState({ ...dashboard })
│   │   │
│   │   └─> React re-renders all panels with new zoom prop
│   │       │
│   │       └─> All PanelComponents receive zoom prop
│   │           │
│   │           └─> PanelComponent.componentDidUpdate()
│   │               │
│   │               └─> if (prevProps.zoom !== this.props.zoom)
│   │                   │
│   │                   └─> Visualization updates zoom
│   │                       │
│   │                       └─> <VisualizationComponent
│   │                             zoom={zoom}  // New zoom prop
│   │                           />
│   │                           │
│   │                           └─> Visualization re-renders with new zoom
│   │
│   ├─> Example: User hovers over panel
│   │   │
│   │   ├─> PanelComponent.handleMouseMove(event)
│   │   │   │
│   │   │   ├─> timestamp = getTimestampFromPosition(event.clientX)
│   │   │   │
│   │   │   └─> onCursorMove(timestamp)  // Callback prop
│   │   │       │
│   │   │       └─> DashboardComponent.handleCursorMove(timestamp)
│   │   │           │
│   │   │           └─> dashboard.setCursorTime(timestamp)
│   │   │               │
│   │   │               ├─> DashboardModel.setCursorTime(timestamp)
│   │   │               │   ├─> this.cursorTime = timestamp
│   │   │               │   └─> this.eventBus.emit('cursorTimeChanged', { timestamp })
│   │   │               │
│   │   │               └─> setDashboardState({ ...dashboard })
│   │   │
│   │   └─> React re-renders all panels with new cursorTime prop
│   │       │
│   │       └─> All PanelComponents receive cursorTime prop
│   │           │
│   │           └─> Visualization renders crosshair at cursorTime
│   │
│   └─> Example: User changes time range
│       │
│       ├─> TimeRangePicker.onChange(newTimeRange)
│       │   │
│       │   └─> dashboard.updateTimeRange(newTimeRange)
│       │       │
│       │       ├─> DashboardModel.updateTimeRange(newTimeRange)
│       │       │   ├─> this.time = newTimeRange
│       │       │   └─> this.eventBus.emit('timeRangeChanged', { timeRange })
│       │       │
│       │       └─> dashboard.refreshDashboard()
│       │           │
│       │           └─> All panels re-execute queries (see "Panel Renders Data")
```

### 10.2 Data Flow Diagram

```
User Action
    ↓
Dashboard State Update (DashboardModel)
    ↓
Event Emission (EventBus)
    ↓
React State Update (setState)
    ↓
React Re-render
    ↓
Props Passed to Panels
    ↓
Panel Lifecycle Hooks (componentDidUpdate)
    ↓
Query Execution (if needed)
    ↓
Data Source Plugin
    ↓
DataFrame[] Generation
    ↓
Transformations
    ↓
Field Configuration
    ↓
Panel Rendering
    ↓
Visualization Update
```

---

## 11. Summary

Grafana's frontend architecture for dynamic multi-panel chart systems provides:

1. **Centralized State Management**: DashboardModel as single source of truth for shared state
2. **Dynamic Panel Management**: Panels can be added/removed/reconfigured at runtime
3. **Data Series Management**: Queries (series) can be added/removed/updated dynamically
4. **Shared Axes Synchronization**: Time range, zoom, pan, and cursor synchronized across panels
5. **React Component Architecture**: Component lifecycle hooks manage panel state
6. **Three-Stage Pipeline**: Clear separation between data fetching, transformation, and rendering
7. **Plugin System**: Extensible architecture through panel, data source, and transformation plugins
8. **Prop-Based Communication**: Panels receive state through props, ensuring loose coupling

**Key Architectural Principles:**
- **Loose Coupling**: Panels don't know about each other
- **Single Source of Truth**: DashboardModel manages all shared state
- **Immutable Data**: DataFrames are immutable
- **Event-Driven**: State changes propagate through events
- **Reactive Updates**: React handles re-rendering based on prop changes

**Mapping to Well Log Visualization:**
- **Dashboard → Well Correlation Chart**: Container for tracks
- **Panel → Track**: Individual visualization sections
- **Data Series → Curve**: Individual measurement series
- **Time Axis → Depth Axis**: Shared axis across tracks
- **Time Range → Depth Range**: Shared depth domain

This architecture provides a robust foundation for building dynamic, interactive multi-panel visualizations with synchronized interactions and flexible data management.

