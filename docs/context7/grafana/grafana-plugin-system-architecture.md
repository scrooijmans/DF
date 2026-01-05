# Grafana Plugin System Architecture

## Executive Summary

Grafana's plugin system is architected around **three core plugin types** (panel, data source, transformation), **strict isolation boundaries** (frontend sandbox, backend process separation), **well-defined lifecycle hooks** (discovery, registration, initialization, execution, cleanup), and **invariant-preserving data model interactions** (DataFrame interface, immutable transformations). The system enables extensibility while maintaining security, stability, and data integrity through process isolation, sandboxed JavaScript contexts, type-safe interfaces, and validation at every boundary. Plugins interact with Grafana's core through **well-defined APIs** that enforce data model invariants, ensuring that plugin code cannot corrupt global state or break core functionality.

---

## 1. Plugin Types and Architecture

### 1.1 Plugin Type Classification

Grafana supports three primary plugin types:

**1. Panel Plugins**
- **Purpose**: Custom visualizations for displaying data
- **Location**: Frontend (React/TypeScript)
- **Interface**: `PanelPlugin<T>`
- **Lifecycle**: Mount → Render → Unmount
- **Data Access**: Receives `DataFrame[]` via props

**2. Data Source Plugins**
- **Purpose**: Connect to external data sources and query data
- **Location**: Frontend (query editor) + Optional Backend (Go)
- **Interface**: `DataSourceApi` (frontend), `backend.QueryDataHandler` (backend)
- **Lifecycle**: Discovery → Registration → Instance Creation → Query Execution
- **Data Access**: Returns `DataFrame[]` from queries

**3. Transformation Plugins**
- **Purpose**: Transform data between query and visualization
- **Location**: Frontend (TypeScript)
- **Interface**: `DataTransformer`
- **Lifecycle**: Registration → Transformation Application
- **Data Access**: Receives and returns `DataFrame[]`

### 1.2 Plugin Manifest (plugin.json)

Every plugin must define a manifest file:

```json
{
  "type": "panel" | "datasource" | "app",
  "name": "Plugin Display Name",
  "id": "my-org-plugin-id",
  "info": {
    "description": "Plugin description",
    "author": {
      "name": "Author Name",
      "url": "https://example.com"
    },
    "version": "1.0.0",
    "updated": "2023-10-08"
  },
  "dependencies": {
    "grafanaDependency": ">=9.0.0",
    "plugins": []
  },
  "backend": false,  // true for backend plugins
  "executable": "",  // executable name for backend
  "signature": "valid" | "invalid" | "unsigned"
}
```

**Key Fields:**
- **type**: Determines plugin category and behavior
- **id**: Unique identifier (must match directory name)
- **grafanaDependency**: Minimum Grafana version required
- **backend**: Whether plugin has backend component
- **signature**: Plugin signature status (security)

---

## 2. Plugin Discovery and Registration

### 2.1 Plugin Discovery Process

**Discovery Flow:**
```
1. Grafana Startup
   ↓
2. Scan Plugin Directories
   - /var/lib/grafana/plugins
   - /usr/share/grafana/data/plugins
   - Custom plugin paths
   ↓
3. Read plugin.json for each directory
   ↓
4. Validate Plugin Manifest
   - Check required fields
   - Validate signature (if enabled)
   - Check Grafana version compatibility
   ↓
5. Register Plugin
   - Add to plugin registry
   - Load plugin metadata
   - Initialize plugin loader
```

**Discovery Mechanisms:**
- **File System Scanning**: Scan plugin directories on startup
- **Manifest Validation**: Validate plugin.json structure
- **Signature Verification**: Check plugin signatures (if enabled)
- **Version Compatibility**: Verify Grafana version requirements

### 2.2 Plugin Registration

**Registration Process:**

```typescript
// Plugin Registry (simplified)
class PluginRegistry {
  private plugins: Map<string, PluginMeta> = new Map();
  
  register(plugin: PluginMeta): void {
    // Validate plugin
    this.validatePlugin(plugin);
    
    // Check for conflicts
    if (this.plugins.has(plugin.id)) {
      throw new Error(`Plugin ${plugin.id} already registered`);
    }
    
    // Register plugin
    this.plugins.set(plugin.id, plugin);
    
    // Initialize plugin loader
    this.loader.load(plugin);
  }
  
  private validatePlugin(plugin: PluginMeta): void {
    // Validate required fields
    if (!plugin.id || !plugin.type || !plugin.info) {
      throw new Error('Invalid plugin manifest');
    }
    
    // Validate signature
    if (plugin.signature === 'invalid') {
      throw new Error(`Plugin ${plugin.id} has invalid signature`);
    }
    
    // Check version compatibility
    if (!this.isVersionCompatible(plugin.dependencies.grafanaDependency)) {
      throw new Error(`Plugin ${plugin.id} requires incompatible Grafana version`);
    }
  }
}
```

**Registration Hooks:**
- **onPluginRegistered**: Called when plugin is registered
- **onPluginLoadError**: Called when plugin fails to load
- **onPluginUnregistered**: Called when plugin is removed

### 2.3 Plugin Loader

**Loading Process:**

```typescript
class PluginLoader {
  async load(plugin: PluginMeta): Promise<void> {
    try {
      // Load plugin module
      const module = await this.loadModule(plugin);
      
      // Extract plugin export
      const pluginExport = module.plugin || module.default;
      
      // Initialize plugin
      await this.initializePlugin(plugin, pluginExport);
      
      // Register plugin with appropriate registry
      this.registerPlugin(plugin, pluginExport);
      
    } catch (error) {
      console.error(`Failed to load plugin ${plugin.id}:`, error);
      throw error;
    }
  }
  
  private async loadModule(plugin: PluginMeta): Promise<any> {
    // Load from plugin directory
    const modulePath = `/public/plugins/${plugin.id}/module.js`;
    return await import(modulePath);
  }
  
  private async initializePlugin(plugin: PluginMeta, pluginExport: any): Promise<void> {
    // Call plugin initialization hook if present
    if (pluginExport.onInit) {
      await pluginExport.onInit();
    }
  }
}
```

---

## 3. Plugin Lifecycle Hooks

### 3.1 Frontend Plugin Lifecycle

**Panel Plugin Lifecycle:**

```typescript
// Panel Plugin Lifecycle
class PanelPlugin<T = any> {
  // 1. Plugin Registration
  static create<T>(panel: React.ComponentType<PanelProps<T>>): PanelPlugin<T> {
    return new PanelPlugin(panel);
  }
  
  // 2. Plugin Initialization
  onInit(): void {
    // Called when plugin is first loaded
    // Setup global state, register services, etc.
  }
  
  // 3. Panel Mount
  render(props: PanelProps<T>): React.ReactElement {
    // Called when panel is mounted
    // props.data contains DataFrame[]
    // props.options contains panel options
    return <PanelComponent {...props} />;
  }
  
  // 4. Panel Update
  onUpdate(props: PanelProps<T>): void {
    // Called when panel props change
    // Data updated, options changed, etc.
  }
  
  // 5. Panel Unmount
  onUnmount(): void {
    // Called when panel is removed
    // Cleanup resources, cancel subscriptions, etc.
  }
}
```

**Data Source Plugin Lifecycle:**

```typescript
// Data Source Plugin Lifecycle
class DataSourcePlugin<TQuery extends DataQuery = DataQuery> {
  // 1. Plugin Registration
  static create<TQuery>(config: DataSourcePluginMeta): DataSourcePlugin<TQuery> {
    return new DataSourcePlugin(config);
  }
  
  // 2. Plugin Initialization
  onInit(): void {
    // Called when plugin is first loaded
  }
  
  // 3. Data Source Instance Creation
  async getInstance(settings: DataSourceInstanceSettings): Promise<DataSourceApi<TQuery>> {
    // Called when data source instance is created
    // Return data source API instance
    return new MyDataSource(settings);
  }
  
  // 4. Query Execution
  async query(request: DataQueryRequest<TQuery>): Promise<DataQueryResponse> {
    // Called when queries are executed
    // Return DataFrame[] responses
  }
  
  // 5. Health Check
  async testDatasource(): Promise<TestDataSourceResponse> {
    // Called to test data source connection
  }
  
  // 6. Instance Cleanup
  onInstanceUnmount(): void {
    // Called when data source instance is removed
    // Cleanup connections, resources, etc.
  }
}
```

### 3.2 Backend Plugin Lifecycle

**Backend Plugin Lifecycle (Go):**

```go
// Backend Plugin Lifecycle
package main

import (
    "context"
    "github.com/grafana/grafana-plugin-sdk-go/backend"
    "github.com/grafana/grafana-plugin-sdk-go/backend/instancemgmt"
)

type MyDataSource struct {
    im instancemgmt.InstanceManager
}

// 1. Plugin Startup
func main() {
    ds := NewDataSource()
    
    // Register plugin with Grafana
    backend.Manage("my-datasource-plugin", ds.im, backend.ManageOpts{
        QueryDataFunc:    ds.QueryData,
        CallResourceFunc: ds.CallResource,
        CheckHealthFunc:  ds.CheckHealth,
    })
}

// 2. Instance Creation
func newInstanceSettings(
    ctx context.Context,
    settings backend.DataSourceInstanceSettings,
) (instancemgmt.Instance, error) {
    // Called when data source instance is created
    // Initialize connections, clients, etc.
    return &instanceSettings{
        httpClient: createHTTPClient(settings),
    }, nil
}

// 3. Query Execution
func (ds *MyDataSource) QueryData(
    ctx context.Context,
    req *backend.QueryDataRequest,
) (*backend.QueryDataResponse, error) {
    // Called when queries are executed
    // Process queries and return DataFrames
    response := backend.NewQueryDataResponse()
    
    for _, query := range req.Queries {
        frame := executeQuery(query)
        response.Responses[query.RefID] = backend.DataResponse{
            Frames: data.Frames{frame},
        }
    }
    
    return response, nil
}

// 4. Health Check
func (ds *MyDataSource) CheckHealth(
    ctx context.Context,
    req *backend.CheckHealthRequest,
) (*backend.CheckHealthResult, error) {
    // Called to test data source connection
    return &backend.CheckHealthResult{
        Status:  backend.HealthStatusOk,
        Message: "Data source is healthy",
    }, nil
}

// 5. Instance Cleanup
func (instance *instanceSettings) Dispose() {
    // Called when instance is removed
    // Cleanup connections, resources, etc.
    instance.httpClient.CloseIdleConnections()
}
```

**Lifecycle Hooks Summary:**

| Hook | Panel Plugin | Data Source Plugin | Backend Plugin |
|------|-------------|-------------------|---------------|
| **Registration** | `PanelPlugin.create()` | `DataSourcePlugin.create()` | `backend.Manage()` |
| **Initialization** | `onInit()` | `onInit()` | `newInstanceSettings()` |
| **Execution** | `render()` | `query()` | `QueryData()` |
| **Update** | `onUpdate()` | N/A | N/A |
| **Cleanup** | `onUnmount()` | `onInstanceUnmount()` | `Dispose()` |

---

## 4. Isolation Boundaries

### 4.1 Frontend Plugin Sandbox

**Plugin Frontend Sandbox** provides JavaScript context isolation:

**Sandbox Architecture:**
```
Main Grafana Application (Main Realm)
    ↓
Plugin Sandbox (Child Realm)
    - Separate JavaScript context
    - Isolated global scope
    - Restricted DOM access
    - Controlled API access
```

**Sandbox Features:**
- **Separate JavaScript Context**: Plugins run in isolated JavaScript realm
- **Global Scope Isolation**: Plugins cannot access main application globals
- **DOM Access Control**: Restricted DOM manipulation
- **API Distortion**: Controlled access to browser APIs

**Sandbox Configuration:**

```typescript
// Sandbox Configuration
interface SandboxConfig {
  enabled: boolean;                    // Enable sandbox
  pluginIds: string[];                 // Plugins to sandbox
  distortions: DistortionMap;         // API distortions
  allowedAPIs: string[];               // Allowed APIs
}

// Distortion Example
const distortions = {
  // Prevent iframe creation
  'HTMLIFrameElement.prototype.createElement': (original) => {
    return function(...args) {
      throw new Error('Creating iframes is not allowed in sandbox');
    };
  },
  
  // Fix Web Workers
  'Worker': (original) => {
    return function(...args) {
      // Provide sandbox-compatible Worker implementation
      return new SandboxWorker(...args);
    };
  },
};
```

**Sandbox Benefits:**
- **Security**: Prevents plugins from accessing sensitive data
- **Stability**: Prevents plugins from breaking main application
- **Isolation**: Plugins cannot interfere with each other

**Sandbox Limitations:**
- **Performance**: Slight performance overhead
- **Compatibility**: Some plugins may not work in sandbox
- **API Restrictions**: Limited access to browser APIs

### 4.2 Backend Plugin Isolation

**Backend plugins run in separate processes:**

**Process Isolation:**
```
Grafana Main Process
    ↓
Plugin Manager
    ↓
Plugin Process 1 (Data Source A)
Plugin Process 2 (Data Source B)
Plugin Process 3 (Panel Backend)
```

**Isolation Mechanisms:**
- **Separate Processes**: Each plugin runs in its own process
- **gRPC Communication**: Plugins communicate via gRPC
- **Resource Limits**: CPU and memory limits per plugin
- **Crash Isolation**: Plugin crashes don't affect main process

**Backend Plugin Communication:**

```go
// Backend Plugin Communication (gRPC)
type BackendPlugin interface {
    // Query execution
    QueryData(ctx context.Context, req *QueryDataRequest) (*QueryDataResponse, error)
    
    // Resource calls
    CallResource(ctx context.Context, req *CallResourceRequest) (*CallResourceResponse, error)
    
    // Health checks
    CheckHealth(ctx context.Context, req *CheckHealthRequest) (*CheckHealthResult, error)
    
    // Stream data
    SubscribeStream(ctx context.Context, req *SubscribeStreamRequest) (*SubscribeStreamResponse, error)
    PublishStream(ctx context.Context, req *PublishStreamRequest) (*PublishStreamResponse, error)
    RunStream(ctx context.Context, req *RunStreamRequest, sender StreamSender) error
}
```

**Isolation Benefits:**
- **Crash Isolation**: Plugin crashes don't affect Grafana
- **Resource Control**: Limit plugin resource usage
- **Security**: Plugins cannot access Grafana's memory
- **Scalability**: Plugins can be distributed across machines

### 4.3 Data Model Isolation

**Plugins interact with data through well-defined interfaces:**

**DataFrame Interface:**
```typescript
interface DataFrame {
  name?: string;
  refId?: string;
  fields: Field[];
  length: number;
  meta?: DataFrameMeta;
}

interface Field {
  name: string;
  type: FieldType;
  values: FieldValueVector;
  labels?: Labels;
  config?: FieldConfig;
}
```

**Invariant Preservation:**
- **Immutable Data**: DataFrames are immutable (plugins cannot mutate)
- **Type Safety**: Field types are enforced
- **Validation**: Data is validated at boundaries
- **Copy-on-Write**: Transformations create new DataFrames

**Data Isolation Example:**

```typescript
// Plugin receives data (immutable)
function PanelComponent(props: PanelProps) {
  const { data } = props;  // DataFrame[] (immutable)
  
  // Plugin cannot mutate data
  // data[0].fields[0].values = [];  // ❌ Error: Cannot assign to read-only property
  
  // Plugin can only read data
  const values = data[0].fields[0].values.toArray();  // ✅ OK: Read-only access
  
  // Plugin returns new data (if transformation)
  return <Visualization data={values} />;
}
```

---

## 5. Plugin Interaction with Data Model

### 5.1 Data Source Plugin → DataFrame

**Data Source plugins must return DataFrames:**

```typescript
// Data Source Plugin Implementation
class MyDataSource extends DataSourceApi<MyQuery> {
  async query(request: DataQueryRequest<MyQuery>): Promise<DataQueryResponse> {
    // Execute query against external data source
    const rawData = await this.fetchData(request.targets);
    
    // Transform to DataFrame
    const frames = rawData.map(data => {
      const frame = new MutableDataFrame({
        refId: data.refId,
        fields: [
          { name: 'time', type: FieldType.time, values: data.timestamps },
          { name: 'value', type: FieldType.number, values: data.values },
        ],
        meta: {
          executedQueryString: data.query,
          datasource: {
            type: this.type,
            uid: this.uid,
          },
        },
      });
      
      return frame;
    });
    
    // Return immutable DataFrames
    return { data: frames };
  }
}
```

**DataFrame Creation Rules:**
- **Required Fields**: Time field for time series
- **Type Consistency**: Field types must match values
- **Metadata**: Include query and data source information
- **Immutability**: DataFrames are immutable once created

### 5.2 Transformation Plugin → DataFrame

**Transformation plugins receive and return DataFrames:**

```typescript
// Transformation Plugin Implementation
export const myTransformation: DataTransformer = {
  id: 'my-transformation',
  name: 'My Transformation',
  
  operator: (options: MyTransformationOptions) => {
    return (source: Observable<DataFrame[]>) => {
      return source.pipe(
        map((frames: DataFrame[]) => {
          // Create new DataFrame (immutable)
          const transformedFrames = frames.map(frame => {
            const newFrame = new MutableDataFrame({
              refId: frame.refId,
              fields: frame.fields.map(field => {
                // Transform field
                return {
                  ...field,
                  name: options.newName || field.name,
                  values: field.values.map(v => transformValue(v, options)),
                };
              }),
              meta: frame.meta,
            });
            
            return newFrame;
          });
          
          return transformedFrames;
        })
      );
    };
  },
};
```

**Transformation Rules:**
- **Immutability**: Never mutate input DataFrames
- **New DataFrames**: Always create new DataFrames
- **Preserve Metadata**: Copy metadata from input
- **Type Safety**: Maintain field type consistency

### 5.3 Panel Plugin → Visualization

**Panel plugins receive DataFrames and render visualizations:**

```typescript
// Panel Plugin Implementation
export class MyPanel extends PureComponent<PanelProps<MyOptions>> {
  render() {
    const { data, options, width, height } = this.props;
    
    // Extract data from DataFrames
    const series = data.series.map(frame => {
      const timeField = frame.fields.find(f => f.type === FieldType.time);
      const valueField = frame.fields.find(f => f.type === FieldType.number);
      
      return {
        time: timeField?.values.toArray() || [],
        values: valueField?.values.toArray() || [],
      };
    });
    
    // Render visualization (read-only data)
    return (
      <div style={{ width, height }}>
        <MyVisualization data={series} options={options} />
      </div>
    );
  }
}
```

**Panel Data Access Rules:**
- **Read-Only**: DataFrames are immutable
- **Field Access**: Access fields by name or type
- **Value Extraction**: Use `.toArray()` or `.get(index)`
- **No Mutation**: Never mutate DataFrame or Field objects

### 5.4 Invariant Preservation

**Grafana enforces data model invariants:**

**1. Immutability Invariant:**
```typescript
// DataFrames are immutable
const frame: DataFrame = getDataFrame();

// ❌ Cannot mutate
frame.fields[0].values = [];  // Error

// ✅ Must create new DataFrame
const newFrame = new MutableDataFrame({
  fields: frame.fields.map(f => ({ ...f, values: transform(f.values) })),
});
```

**2. Type Consistency Invariant:**
```typescript
// Field types must match values
const field: Field = {
  name: 'value',
  type: FieldType.number,  // Must match values type
  values: new ArrayVector([1, 2, 3]),  // Numbers, not strings
};
```

**3. Structure Invariant:**
```typescript
// All fields must have same length
const frame: DataFrame = {
  fields: [
    { name: 'time', values: [1, 2, 3] },      // Length: 3
    { name: 'value', values: [10, 20, 30] },  // Length: 3 ✅
    // { name: 'label', values: [1, 2] },    // Length: 2 ❌ Error
  ],
};
```

**4. Metadata Invariant:**
```typescript
// Metadata must be consistent
const frame: DataFrame = {
  meta: {
    executedQueryString: 'SELECT * FROM table',
    datasource: {
      type: 'mysql',
      uid: 'mysql-uid',
    },
  },
  // Frame data must match metadata
};
```

**Invariant Enforcement:**
- **Validation**: DataFrames validated at creation
- **Type Checking**: TypeScript enforces types
- **Runtime Checks**: Runtime validation in critical paths
- **Error Handling**: Invalid data rejected with errors

---

## 6. Plugin System Architecture Summary

### 6.1 Architecture Layers

```
┌─────────────────────────────────────────────────────────┐
│              Grafana Core Application                    │
│  ┌──────────────────────────────────────────────────┐  │
│  │           Plugin Manager                          │  │
│  │  - Discovery                                      │  │
│  │  - Registration                                   │  │
│  │  - Lifecycle Management                           │  │
│  └──────────────────────────────────────────────────┘  │
│                                                         │
│  ┌──────────────────────────────────────────────────┐  │
│  │           Plugin Registry                         │  │
│  │  - Panel Plugins                                 │  │
│  │  - Data Source Plugins                            │  │
│  │  - Transformation Plugins                        │  │
│  └──────────────────────────────────────────────────┘  │
│                                                         │
│  ┌──────────────────────────────────────────────────┐  │
│  │           Data Model Layer                        │  │
│  │  - DataFrame Interface                            │  │
│  │  - Field Types                                    │  │
│  │  - Metadata                                       │  │
│  └──────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────┐
│              Plugin Isolation Layer                     │
│  ┌──────────────────┐  ┌──────────────────┐           │
│  │ Frontend Sandbox │  │ Backend Process  │           │
│  │  - JS Context    │  │  - gRPC          │           │
│  │  - DOM Control   │  │  - Isolation     │           │
│  └──────────────────┘  └──────────────────┘           │
└─────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────┐
│              Plugin Implementations                      │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  │
│  │ Panel Plugin │  │ Data Source  │  │ Transform   │  │
│  │              │  │   Plugin     │  │   Plugin    │  │
│  └──────────────┘  └──────────────┘  └──────────────┘  │
└─────────────────────────────────────────────────────────┘
```

### 6.2 Key Architectural Principles

**1. Isolation:**
- **Frontend**: JavaScript sandbox for frontend plugins
- **Backend**: Separate processes for backend plugins
- **Data**: Immutable DataFrames prevent mutation

**2. Lifecycle Management:**
- **Discovery**: Automatic plugin discovery
- **Registration**: Validated plugin registration
- **Initialization**: Controlled plugin initialization
- **Execution**: Isolated plugin execution
- **Cleanup**: Resource cleanup on unmount

**3. Invariant Preservation:**
- **Immutability**: DataFrames are immutable
- **Type Safety**: Field types enforced
- **Structure**: Data structure validated
- **Metadata**: Metadata consistency checked

**4. Extensibility:**
- **Plugin Types**: Multiple plugin types supported
- **SDK Support**: Frontend (TypeScript) and Backend (Go) SDKs
- **API Stability**: Stable plugin APIs
- **Versioning**: Plugin versioning support

### 6.3 Security and Stability

**Security Mechanisms:**
- **Plugin Signing**: Signature verification
- **Sandbox Isolation**: JavaScript context isolation
- **Process Isolation**: Backend plugin process separation
- **API Restrictions**: Controlled API access

**Stability Mechanisms:**
- **Crash Isolation**: Plugin crashes don't affect Grafana
- **Resource Limits**: CPU and memory limits
- **Error Handling**: Graceful error handling
- **Validation**: Input validation at boundaries

**Data Integrity:**
- **Immutable Data**: DataFrames cannot be mutated
- **Type Safety**: Type checking prevents errors
- **Validation**: Data validated at every boundary
- **Error Recovery**: Graceful handling of invalid data

---

## 7. Summary

Grafana's plugin system architecture provides:

1. **Three Plugin Types**: Panel, Data Source, and Transformation plugins
2. **Lifecycle Hooks**: Discovery, registration, initialization, execution, cleanup
3. **Isolation Boundaries**: Frontend sandbox and backend process isolation
4. **Data Model Interaction**: Immutable DataFrame interface with invariant preservation
5. **Security**: Plugin signing, sandbox isolation, process separation
6. **Stability**: Crash isolation, resource limits, error handling
7. **Extensibility**: Multiple plugin types, SDK support, API stability

The system enables extensibility while maintaining security, stability, and data integrity through well-defined interfaces, strict isolation, and invariant-preserving data model interactions.


