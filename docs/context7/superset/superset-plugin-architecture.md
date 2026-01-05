# Apache Superset: Plugin Architecture

## Executive Summary

Apache Superset's plugin architecture enables extensibility through **frontend chart plugins** and **backend database engine specs**. Chart plugins follow a standardized lifecycle: **creation → configuration → registration → rendering**, using the `configure()` and `register()` pattern. Plugins communicate through well-defined interfaces: `formData` for configuration, `queriesData` for results, and `SuperChart` for rendering. Database engine specs extend `BaseEngineSpec` to add new data sources. Backward compatibility is maintained through **deprecation warnings**, **migration tools**, and **versioned APIs**. The system supports plugin versioning, automatic migrations, and graceful degradation when plugins are unavailable.

---

## 1. Plugin Architecture Overview

### 1.1 Plugin Types

**Superset supports two main plugin types:**

1. **Frontend Chart Plugins**: Visualization components (React/TypeScript)
2. **Backend Database Engine Specs**: Data source connectors (Python)

**Plugin Categories:**
- **Chart Plugins**: Bar, line, pie, map, etc.
- **Database Engine Specs**: PostgreSQL, MySQL, SQLite, etc.
- **Preset Plugins**: Collections of related plugins (NVD3, Deck.gl, ECharts)

### 1.2 Plugin Architecture Principles

**Key Principles:**
- **Modularity**: Plugins are independent packages
- **Standardization**: Common interfaces and patterns
- **Extensibility**: Easy to add new plugins
- **Backward Compatibility**: Existing plugins continue to work
- **Versioning**: Support for plugin versions and migrations

---

## 2. Frontend Chart Plugins

### 2.1 Plugin Structure

**Chart plugins follow a standard structure:**

```typescript
// Plugin class
class MyChartPlugin {
  configure(config: { key: string }): MyChartPlugin;
  register(): void;
}

// Usage
new MyChartPlugin()
  .configure({ key: 'my-chart' })
  .register();
```

**Key Components:**
- **Plugin Class**: Main plugin definition
- **Configuration**: Unique key for identification
- **Registration**: Adds plugin to Superset registry

### 2.2 Plugin Registration Lifecycle

**Complete registration flow:**

```javascript
// 1. Import plugin
import MyChartPlugin from '@superset-ui/plugin-chart-my-chart';

// 2. Configure with unique key
const plugin = new MyChartPlugin().configure({ 
  key: 'my-chart' 
});

// 3. Register with Superset
plugin.register();

// 4. Plugin available for use
// Can be referenced by key 'my-chart'
```

**Registration Steps:**
1. **Import**: Load plugin package
2. **Configure**: Set unique key and options
3. **Register**: Add to Superset's plugin registry
4. **Available**: Plugin can be used in charts

### 2.3 Plugin Integration

**Integrating into Superset preset:**

```javascript
// MainPreset.js
import { SupersetPluginChartHelloWorld } from 'superset-plugin-chart-hello-world';

// Inside plugins array
new SupersetPluginChartHelloWorld().configure({ 
  key: 'ext-hello-world' 
}),
```

**Integration Points:**
- **MainPreset.js**: Main plugin registry
- **Plugin Array**: List of all registered plugins
- **Build Process**: Plugins bundled with Superset

### 2.4 Plugin Development Workflow

**Creating a new chart plugin:**

```bash
# 1. Install generator
npm install -g @superset-ui/generator-superset

# 2. Create plugin directory
mkdir superset-plugin-chart-hello-world
cd superset-plugin-chart-hello-world

# 3. Generate plugin scaffold
yo @superset-ui/superset

# 4. Develop plugin
npm run dev

# 5. Build plugin
npm run build

# 6. Link to Superset (development)
npm link
# In Superset frontend:
npm link superset-plugin-chart-hello-world
```

**Development Steps:**
1. **Scaffold**: Generate plugin structure
2. **Develop**: Implement chart logic
3. **Test**: Test in development mode
4. **Build**: Compile for production
5. **Link**: Connect to Superset instance

---

## 3. Plugin API Contract

### 3.1 SuperChart Interface

**Plugins render through SuperChart:**

```jsx
<SuperChart
  chartType="my-chart"  // Must match plugin key
  width={600}
  height={400}
  formData={{
    // Chart configuration
    groupby: ['region'],
    metrics: ['sum__sales'],
    viz_type: 'my-chart'
  }}
  queriesData={[{
    // Query results
    data: [
      { region: 'US-West', sum__sales: 10000 },
      { region: 'US-East', sum__sales: 15000 }
    ],
    colnames: ['region', 'sum__sales'],
    coltypes: ['STRING', 'NUMERIC']
  }]}
/>
```

**SuperChart Props:**
- **chartType**: Plugin key (required)
- **width/height**: Chart dimensions
- **formData**: Chart configuration
- **queriesData**: Query results array

### 3.2 FormData Contract

**Standard formData structure:**

```typescript
interface FormData {
  datasource: string;        // "15__table"
  viz_type: string;          // Plugin key
  groupby?: string[];        // Grouping columns
  metrics?: string[];        // Metric expressions
  adhoc_filters?: Filter[];  // Ad-hoc filters
  time_range?: string;       // Time range filter
  row_limit?: number;        // Row limit
  // ... plugin-specific fields
}
```

**Common Fields:**
- **datasource**: Dataset identifier
- **viz_type**: Visualization type (plugin key)
- **groupby**: Columns for grouping
- **metrics**: Aggregations to compute
- **filters**: Applied filters

### 3.3 QueriesData Contract

**Standard queriesData structure:**

```typescript
interface QueriesData {
  data: Array<Record<string, any>>;  // Row data
  colnames: string[];                // Column names
  coltypes: string[];                // Column types
  rowcount: number;                  // Number of rows
}
```

**Data Format:**
- **data**: Array of row objects
- **colnames**: Column name array
- **coltypes**: Generic type array (STRING, NUMERIC, etc.)
- **rowcount**: Total row count

### 3.4 Transform Props Pattern

**Plugins transform props for rendering:**

```typescript
function transformProps(chartProps: ChartProps): TransformedProps {
  const { formData, queriesData, width, height } = chartProps;
  
  // Extract data
  const data = queriesData[0].data;
  const colnames = queriesData[0].colnames;
  
  // Transform for visualization
  const transformedData = data.map(row => ({
    x: row[formData.groupby[0]],
    y: row[formData.metrics[0]]
  }));
  
  return {
    data: transformedData,
    width,
    height,
    // ... other props
  };
}
```

**Transform Process:**
1. **Extract**: Get formData and queriesData
2. **Process**: Transform data for visualization
3. **Return**: Props for rendering component

---

## 4. Plugin Lifecycle

### 4.1 Plugin Lifecycle Stages

**Complete plugin lifecycle:**

```
1. Creation
   ↓
2. Configuration
   ↓
3. Registration
   ↓
4. Discovery
   ↓
5. Rendering
   ↓
6. Updates
   ↓
7. Deprecation (optional)
   ↓
8. Migration (optional)
```

### 4.2 Creation Stage

**Plugin creation:**

```bash
# Generate plugin scaffold
yo @superset-ui/superset

# Plugin structure created:
superset-plugin-chart-hello-world/
  ├── src/
  │   ├── plugin/
  │   │   ├── index.ts
  │   │   └── transformProps.ts
  │   └── HelloWorld.tsx
  ├── package.json
  └── tsconfig.json
```

**Created Files:**
- **Plugin class**: Main plugin definition
- **Transform function**: Data transformation
- **Component**: React visualization component
- **Configuration**: Package and TypeScript config

### 4.3 Configuration Stage

**Plugin configuration:**

```javascript
new MyChartPlugin().configure({
  key: 'my-chart',           // Unique identifier
  name: 'My Chart',          // Display name
  description: '...',        // Description
  thumbnail: '...',          // Thumbnail image
  // ... other config options
});
```

**Configuration Options:**
- **key**: Unique plugin identifier (required)
- **name**: Human-readable name
- **description**: Plugin description
- **thumbnail**: Preview image
- **metadata**: Additional metadata

### 4.4 Registration Stage

**Plugin registration:**

```javascript
// Register single plugin
new MyChartPlugin()
  .configure({ key: 'my-chart' })
  .register();

// Register preset (multiple plugins)
new NVD3ChartPreset().register();

// Register individual plugins from preset
new AreaChartPlugin().configure({ key: 'area' }).register();
new LineChartPlugin().configure({ key: 'line' }).register();
```

**Registration Methods:**
- **Single Plugin**: Register one plugin
- **Preset**: Register collection of plugins
- **Selective**: Register specific plugins from preset

### 4.5 Discovery Stage

**Plugin discovery:**

```typescript
// Superset discovers registered plugins
const plugin = getChartPlugin('my-chart');

// Plugin available for use
if (plugin) {
  // Use plugin
}
```

**Discovery Process:**
1. **Registry Lookup**: Find plugin by key
2. **Validation**: Verify plugin is valid
3. **Availability**: Check if plugin is enabled
4. **Usage**: Plugin can be used in charts

### 4.6 Rendering Stage

**Plugin rendering:**

```jsx
// Superset renders plugin
<SuperChart
  chartType="my-chart"
  formData={formData}
  queriesData={queriesData}
  width={600}
  height={400}
/>

// Plugin's transformProps called
const transformedProps = plugin.transformProps(chartProps);

// Plugin's component rendered
<MyChartComponent {...transformedProps} />
```

**Rendering Flow:**
1. **SuperChart**: Receives chart props
2. **Transform**: Plugin transforms props
3. **Render**: Plugin component renders visualization

### 4.7 Update Stage

**Plugin updates:**

```bash
# Update plugin package
npm update @superset-ui/plugin-chart-my-chart

# Rebuild Superset
npm run build

# Restart Superset
# Plugin updates applied
```

**Update Process:**
1. **Package Update**: Update plugin package
2. **Rebuild**: Rebuild Superset frontend
3. **Restart**: Restart Superset instance
4. **New Version**: New plugin version active

---

## 5. Backend Database Engine Specs

### 5.1 Engine Spec Structure

**Database engine specs extend BaseEngineSpec:**

```python
from superset.db_engine_specs import BaseEngineSpec

class MyDatabaseEngineSpec(BaseEngineSpec):
    engine_name = "my_database"
    engine = "My Database"
    
    # Type mappings
    column_type_mappings = (
        (re.compile(r"^string", re.IGNORECASE), types.String(), GenericDataType.STRING),
        # ... more mappings
    )
    
    # Capabilities
    supports_file_upload = True
    supports_dynamic_schema = False
    supports_subqueries = True
```

**Engine Spec Components:**
- **engine_name**: Unique identifier
- **engine**: Display name
- **column_type_mappings**: Type mappings
- **Capabilities**: Feature flags

### 5.2 Engine Spec Registration

**Engine specs auto-discovered:**

```python
# Engine specs in superset/db_engine_specs/
# Automatically discovered and registered

# Get available engines
from superset.db_engine_specs import get_available_engine_specs

engines = get_available_engine_specs()
for engine in engines:
    print(f"{engine.engine_name}: {engine.engine}")
```

**Registration Process:**
1. **Discovery**: Auto-discovered from `db_engine_specs` module
2. **Registration**: Added to engine registry
3. **Available**: Can be used in database connections

### 5.3 Engine Spec Capabilities

**Configurable capabilities:**

```python
class MyDatabaseEngineSpec(BaseEngineSpec):
    # File operations
    supports_file_upload = True
    supports_csv_upload = True
    
    # Schema operations
    supports_dynamic_schema = False
    supports_catalog = True
    
    # Query features
    supports_subqueries = True
    supports_joins = True
    supports_aliases_overshadowing_source_columns = True
    
    # OAuth2
    supports_oauth2 = False
```

**Capability Flags:**
- **File Operations**: Upload support
- **Schema Operations**: Dynamic schema, catalog
- **Query Features**: Subqueries, joins, aliases
- **Authentication**: OAuth2 support

### 5.4 Custom Type Mappings

**Override default type mappings:**

```python
from sqlalchemy.dialects.mssql.base import SMALLDATETIME

class MssqlEngineSpec(BaseEngineSpec):
    column_type_mappings = (
        # Custom mapping for SMALLDATETIME
        (
            re.compile(r"^smalldatetime.*", re.IGNORECASE),
            SMALLDATETIME(),
            GenericDataType.TEMPORAL,
        ),
        # Inherit defaults from BaseEngineSpec
    )
```

**Type Mapping:**
- **Pattern**: Regex to match database type
- **SQLAlchemy Type**: Type object for SQL generation
- **Generic Type**: Superset's generic type (STRING, NUMERIC, etc.)

---

## 6. Backward Compatibility

### 6.1 Deprecation Warnings

**Marking deprecated functionality:**

```python
from deprecated import deprecated

@deprecated(version="3.0.0", remove_in="4.0.0")
def old_function():
    warnings.warn("Use new_function instead", DeprecationWarning)
```

**Deprecation Process:**
1. **Mark Deprecated**: Add `@deprecated` decorator
2. **Version Info**: Specify deprecation and removal versions
3. **Warning**: Emit deprecation warning
4. **Documentation**: Document migration path

### 6.2 API Versioning

**Versioned endpoints:**

```python
# Old endpoint (deprecated)
@expose("/old-endpoint")
def old_endpoint():
    # Deprecated functionality

# New endpoint
@expose("/api/v1/new-endpoint")
def new_endpoint():
    # New implementation
```

**Versioning Strategy:**
- **URL Versioning**: `/api/v1/`, `/api/v2/`
- **Deprecation Period**: Old endpoints remain during transition
- **Migration Guide**: Documentation for migration

### 6.3 Plugin Migration Tools

**CLI for visualization migrations:**

```bash
# Migrate charts from old to new plugin
superset viz-migrations upgrade

# Downgrade charts
superset viz-migrations downgrade

# Automatic migration during:
# - Dashboard import/export
# - Backup restore
```

**Migration Support:**
- **Upgrade**: Migrate to new plugin version
- **Downgrade**: Rollback to previous version
- **Automatic**: Applied during import/export

### 6.4 Legacy Plugin Support

**Legacy plugins continue to work:**

```javascript
// Legacy plugins still supported
import { NVD3ChartPreset } from '@superset-ui/legacy-preset-chart-nvd3';

new NVD3ChartPreset().register();
```

**Legacy Support:**
- **Backward Compatible**: Old plugins still work
- **No Breaking Changes**: Existing charts continue to function
- **Gradual Migration**: Migrate at own pace

### 6.5 Database Migration Compatibility

**Database schema migrations:**

```python
# Migration with backward compatibility
def upgrade():
    # Add new column (nullable for backward compatibility)
    op.add_column(
        "dashboards",
        sa.Column("new_column", sa.String(255), nullable=True)
    )

def downgrade():
    # Remove column (rollback)
    op.drop_column("dashboards", "new_column")
```

**Migration Strategy:**
- **Additive Changes**: New columns nullable initially
- **Rollback Support**: `downgrade()` function for rollback
- **Version Tracking**: Alembic tracks migration versions

---

## 7. Plugin Versioning

### 7.1 Plugin Package Versioning

**NPM package versioning:**

```json
{
  "name": "@superset-ui/plugin-chart-my-chart",
  "version": "1.2.3",
  "peerDependencies": {
    "@superset-ui/core": "^0.18.0"
  }
}
```

**Version Strategy:**
- **Semantic Versioning**: MAJOR.MINOR.PATCH
- **Peer Dependencies**: Core Superset version requirements
- **Compatibility**: Version ranges for compatibility

### 7.2 Plugin Version Configuration

**Versioning in plugin config:**

```typescript
{
  id: 'my_plugin',
  path: 'my_plugin',
  includeCurrentVersion: true,
  lastVersion: '1.1.0',
  onlyIncludeVersions: ['current', '1.1.0', '1.0.0'],
  versions: {
    current: {
      label: 'Next',
      path: 'next',
      banner: 'unreleased',
    },
    '1.1.0': {
      label: '1.1.0',
      path: '1.1.0',
      banner: 'none',
    },
  },
}
```

**Version Configuration:**
- **Current Version**: Latest development version
- **Stable Versions**: Released versions
- **Version Labels**: Display names for versions

### 7.3 Migration Between Versions

**Handling version changes:**

```bash
# Upgrade Superset
pip install --upgrade apache-superset

# Run migrations
superset db upgrade

# Plugin migrations applied automatically
# Charts migrated to new plugin versions
```

**Migration Process:**
1. **Upgrade**: Update Superset version
2. **Database Migration**: Run schema migrations
3. **Plugin Migration**: Migrate charts to new plugin versions
4. **Validation**: Verify charts still work

---

## 8. Plugin Guarantees

### 8.1 API Stability

**Stable API contracts:**

```typescript
// Stable interfaces
interface ChartProps {
  formData: FormData;
  queriesData: QueriesData[];
  width: number;
  height: number;
}

interface FormData {
  datasource: string;
  viz_type: string;
  // ... stable fields
}
```

**Stability Guarantees:**
- **Interface Stability**: Core interfaces remain stable
- **Backward Compatibility**: Old plugins continue to work
- **Deprecation Period**: Advance notice before breaking changes

### 8.2 Plugin Isolation

**Plugins are isolated:**

```javascript
// Plugin A doesn't affect Plugin B
new PluginA().configure({ key: 'plugin-a' }).register();
new PluginB().configure({ key: 'plugin-b' }).register();

// Each plugin independent
// Failure in one doesn't affect others
```

**Isolation Benefits:**
- **Independent**: Plugins don't interfere
- **Failure Isolation**: One plugin failure doesn't break others
- **Version Independence**: Different versions can coexist

### 8.3 Graceful Degradation

**Handling missing plugins:**

```typescript
// Check if plugin exists
const plugin = getChartPlugin('my-chart');

if (plugin) {
  // Render plugin
  return <SuperChart chartType="my-chart" {...props} />;
} else {
  // Fallback to default
  return <DefaultChart {...props} />;
}
```

**Degradation Strategy:**
- **Plugin Check**: Verify plugin exists
- **Fallback**: Use default visualization
- **Error Handling**: Graceful error messages

### 8.4 Data Contract Guarantees

**Stable data formats:**

```typescript
// QueriesData format stable
interface QueriesData {
  data: Array<Record<string, any>>;  // Always array of objects
  colnames: string[];                // Always string array
  coltypes: string[];                // Always type array
  rowcount: number;                  // Always number
}
```

**Data Guarantees:**
- **Format Stability**: Data format doesn't change
- **Type Safety**: TypeScript interfaces ensure types
- **Backward Compatible**: Old format still supported

---

## 9. Plugin Development Best Practices

### 9.1 Plugin Naming

**Naming conventions:**

```javascript
// Package name
@superset-ui/plugin-chart-my-chart

// Plugin key
'my-chart'  // kebab-case, unique

// Class name
MyChartPlugin  // PascalCase
```

**Naming Rules:**
- **Package**: `@superset-ui/plugin-chart-{name}`
- **Key**: kebab-case, unique
- **Class**: PascalCase with "Plugin" suffix

### 9.2 Plugin Structure

**Recommended structure:**

```
superset-plugin-chart-my-chart/
  ├── src/
  │   ├── plugin/
  │   │   ├── index.ts          # Plugin class
  │   │   ├── transformProps.ts # Data transformation
  │   │   └── controlPanel.ts  # Control panel config
  │   └── MyChart.tsx           # React component
  ├── package.json
  ├── tsconfig.json
  └── README.md
```

**Structure Guidelines:**
- **Plugin Logic**: In `plugin/` directory
- **Component**: Separate React component
- **Configuration**: Control panel configuration
- **Documentation**: README with usage examples

### 9.3 Error Handling

**Robust error handling:**

```typescript
function transformProps(chartProps: ChartProps): TransformedProps {
  try {
    const { formData, queriesData } = chartProps;
    
    // Validate data
    if (!queriesData || queriesData.length === 0) {
      throw new Error('No query data available');
    }
    
    // Transform data
    return transformData(formData, queriesData);
    
  } catch (error) {
    // Log error
    console.error('Plugin error:', error);
    
    // Return safe fallback
    return {
      data: [],
      error: error.message
    };
  }
}
```

**Error Handling:**
- **Validation**: Validate inputs
- **Try-Catch**: Catch and handle errors
- **Fallback**: Return safe default
- **Logging**: Log errors for debugging

### 9.4 Testing

**Plugin testing:**

```typescript
// Unit tests
describe('MyChartPlugin', () => {
  it('transforms props correctly', () => {
    const props = { /* test props */ };
    const transformed = transformProps(props);
    expect(transformed).toMatchSnapshot();
  });
});
```

**Testing Strategy:**
- **Unit Tests**: Test transformation logic
- **Integration Tests**: Test with SuperChart
- **Snapshot Tests**: Visual regression testing

---

## 10. Plugin Examples

### 10.1 Simple Chart Plugin

**Basic plugin example:**

```javascript
import { ChartPlugin } from '@superset-ui/core';

class MyChartPlugin extends ChartPlugin {
  constructor() {
    super({
      name: 'My Chart',
      metadata: {
        thumbnail: '...',
      },
    });
  }
  
  loadChart() {
    return import('./MyChart');
  }
}

new MyChartPlugin()
  .configure({ key: 'my-chart' })
  .register();
```

### 10.2 Preset Plugin

**Preset with multiple charts:**

```javascript
import { PresetChartPlugin } from '@superset-ui/core';

class MyPresetPlugin extends PresetChartPlugin {
  constructor() {
    super({
      name: 'My Preset',
      plugins: [
        new Chart1Plugin().configure({ key: 'chart1' }),
        new Chart2Plugin().configure({ key: 'chart2' }),
      ],
    });
  }
}

new MyPresetPlugin().register();
```

### 10.3 Database Engine Spec

**Custom database engine:**

```python
from superset.db_engine_specs import BaseEngineSpec

class MyDatabaseEngineSpec(BaseEngineSpec):
    engine_name = "my_database"
    engine = "My Database"
    
    supports_file_upload = True
    supports_subqueries = True
    
    @classmethod
    def get_function_names(cls) -> list[str]:
        return ["MY_FUNC1", "MY_FUNC2"]
```

---

## 11. Limitations and Considerations

### 11.1 Current Limitations

**Plugin system limitations:**
- **No Hot Reload**: Must rebuild for changes
- **Version Conflicts**: Multiple versions can conflict
- **Limited Backend Plugins**: Only database engine specs supported
- **No Plugin Marketplace**: No centralized plugin repository

### 11.2 Migration Considerations

**When migrating plugins:**
- **Data Compatibility**: Ensure data format compatible
- **Chart Migration**: Migrate existing charts
- **Testing**: Thoroughly test migrated charts
- **Rollback Plan**: Plan for rollback if needed

### 11.3 Performance Considerations

**Plugin performance:**
- **Bundle Size**: Large plugins increase bundle size
- **Render Performance**: Complex plugins may be slow
- **Memory Usage**: Multiple plugins increase memory
- **Lazy Loading**: Consider lazy loading for large plugins

---

## 12. Conclusion

Superset's plugin architecture provides:

1. **Extensibility**: Easy to add new visualizations and data sources
2. **Standardization**: Common interfaces and patterns
3. **Backward Compatibility**: Existing plugins continue to work
4. **Versioning**: Support for plugin versions and migrations
5. **Isolation**: Plugins don't interfere with each other

**Key Takeaways:**
- **Plugin Lifecycle**: Creation → Configuration → Registration → Rendering
- **API Contracts**: Stable interfaces (formData, queriesData, SuperChart)
- **Backward Compatibility**: Deprecation warnings, migration tools, legacy support
- **Database Engine Specs**: Extend BaseEngineSpec for new data sources
- **Versioning**: Semantic versioning with migration support

**Architecture Strengths:**
- **Modularity**: Independent plugin packages
- **Standardization**: Common patterns and interfaces
- **Extensibility**: Easy to add new plugins
- **Stability**: Backward compatibility guarantees

**Areas for Improvement:**
- Hot reload for development
- Centralized plugin marketplace
- More backend plugin types
- Better version conflict resolution

This architecture prioritizes **extensibility and stability** over **fine-grained control**, enabling users to add custom visualizations and data sources while maintaining compatibility with existing plugins and charts.


