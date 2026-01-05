# Grafana Panel Synchronization Architecture: Shared State Propagation

## Executive Summary

Grafana synchronizes multiple panels on a dashboard through a **centralized state management** system where the **DashboardModel** acts as the single source of truth for shared state (time range, template variables, filters). Panels subscribe to state changes through **reactive patterns** and **prop-based updates**, receiving new state as props when shared state changes. This architecture enables **loose coupling** between panels—panels don't know about each other, only about the dashboard state they consume. Changes propagate through **query re-execution** and **prop updates** rather than direct panel-to-panel communication.

---

## 1. Centralized State Architecture

### 1.1 DashboardModel: Single Source of Truth

The **DashboardModel** is the central state container for all shared dashboard state:

```typescript
class DashboardModel {
  // Shared state
  time: TimeRange;              // Dashboard time range
  templating: {
    list: VariableModel[];      // Template variables
  };
  refresh: string;              // Refresh interval
  timezone: string;             // Timezone setting
  
  // Panel collection
  panels: PanelModel[];         // Array of panel models
  
  // State change tracking
  private eventBus: EventEmitter;
  
  // State change methods
  updateTimeRange(newTimeRange: TimeRange): void;
  updateVariable(variableName: string, value: any): void;
  refreshDashboard(): void;
}
```

**Key Characteristics:**
- **Single Source of Truth**: All shared state lives in DashboardModel
- **Immutable Updates**: State changes create new state objects
- **Event Emission**: Changes emit events for subscribers
- **Panel Independence**: Panels don't reference each other

### 1.2 Shared State Components

**1. Time Range:**
```typescript
interface TimeRange {
  from: DateTime;               // Start time
  to: DateTime;                 // End time
  raw: {
    from: string;              // Raw string (e.g., "now-6h")
    to: string;                // Raw string (e.g., "now")
  };
}
```

**2. Template Variables:**
```typescript
interface VariableModel {
  name: string;                 // Variable name (e.g., "$host")
  current: {
    value: any;                 // Current value
    text: string;               // Display text
  };
  options: Array<{
    value: any;
    text: string;
  }>;
  type: 'query' | 'textbox' | 'custom' | 'interval';
}
```

**3. Refresh State:**
```typescript
interface RefreshState {
  interval: string;             // Refresh interval ("5s", "1m", etc.)
  lastRefresh: number;          // Timestamp of last refresh
  isRefreshing: boolean;        // Currently refreshing
}
```

---

## 2. State Change Propagation Mechanism

### 2.1 State Change Flow

When shared state changes, the following flow occurs:

```
User Action (Time Range Change / Variable Change)
    ↓
DashboardModel.updateTimeRange() / updateVariable()
    ↓
DashboardModel State Updated
    ↓
Event Emitted (timeRangeChanged / variableChanged)
    ↓
Dashboard Component Receives Event
    ↓
Dashboard Component Updates All Panel Props
    ↓
Each Panel Receives New Props
    ↓
Panels Re-execute Queries with New State
    ↓
Panels Re-render with New Data
```

### 2.2 Time Range Change Propagation

**1. User Changes Time Range:**
```typescript
// User selects new time range in time picker
const newTimeRange: TimeRange = {
  from: dateTime().subtract(1, 'hour'),
  to: dateTime(),
  raw: { from: 'now-1h', to: 'now' },
};

// DashboardModel updates
dashboard.updateTimeRange(newTimeRange);
```

**2. DashboardModel Updates State:**
```typescript
class DashboardModel {
  updateTimeRange(newTimeRange: TimeRange): void {
    // Update state
    this.time = newTimeRange;
    
    // Emit event
    this.eventBus.emit('timeRangeChanged', {
      timeRange: newTimeRange,
      oldTimeRange: this.time,
    });
    
    // Trigger refresh
    this.refreshDashboard();
  }
}
```

**3. Dashboard Component Listens and Updates:**
```typescript
class DashboardComponent extends React.Component {
  componentDidMount() {
    // Subscribe to time range changes
    this.dashboard.eventBus.on('timeRangeChanged', (event) => {
      this.handleTimeRangeChange(event.timeRange);
    });
  }
  
  handleTimeRangeChange(newTimeRange: TimeRange): void {
    // Update all panel props
    this.setState({
      timeRange: newTimeRange,
    });
    
    // React will re-render all panels with new props
  }
  
  render() {
    return (
      <div>
        {this.dashboard.panels.map(panel => (
          <PanelComponent
            key={panel.id}
            panel={panel}
            timeRange={this.state.timeRange}  // New time range passed as prop
            variables={this.state.variables}   // Current variables
            onRefresh={this.handleRefresh}
          />
        ))}
      </div>
    );
  }
}
```

**4. Panel Receives New Props:**
```typescript
class PanelComponent extends React.Component<PanelProps> {
  componentDidUpdate(prevProps: PanelProps) {
    // Check if time range changed
    if (prevProps.timeRange !== this.props.timeRange) {
      // Time range changed, re-execute queries
      this.executeQueries();
    }
  }
  
  executeQueries(): void {
    // Build query request with current time range
    const request: DataQueryRequest = {
      targets: this.panel.targets,
      range: this.props.timeRange,  // Use prop from dashboard
      // ... other query parameters
    };
    
    // Execute queries
    this.dataSource.query(request).then(response => {
      this.setState({ data: response.data });
    });
  }
}
```

### 2.3 Variable Change Propagation

**1. User Changes Variable:**
```typescript
// User selects new value in variable dropdown
const newValue = "server2";

// DashboardModel updates variable
dashboard.updateVariable("host", newValue);
```

**2. DashboardModel Updates Variable:**
```typescript
class DashboardModel {
  updateVariable(variableName: string, value: any): void {
    // Find variable
    const variable = this.templating.list.find(v => v.name === variableName);
    if (!variable) return;
    
    // Update variable
    variable.current.value = value;
    variable.current.text = this.getVariableText(variable, value);
    
    // Emit event
    this.eventBus.emit('variableChanged', {
      variable: variableName,
      value: value,
      oldValue: variable.current.value,
    });
    
    // Trigger refresh (variables affect queries)
    this.refreshDashboard();
  }
}
```

**3. Variable Replacement in Queries:**
```typescript
class DashboardComponent {
  // Replace variables in panel queries
  replaceVariablesInQueries(panels: PanelModel[], variables: VariableModel[]): void {
    panels.forEach(panel => {
      panel.targets.forEach(target => {
        // Replace $variable syntax with actual values
        if (target.expr) {
          target.expr = this.replaceVariableSyntax(target.expr, variables);
        }
      });
    });
  }
  
  replaceVariableSyntax(query: string, variables: VariableModel[]): string {
    let result = query;
    
    variables.forEach(variable => {
      const pattern = new RegExp(`\\$${variable.name}`, 'g');
      const value = this.formatVariableValue(variable.current.value);
      result = result.replace(pattern, value);
    });
    
    return result;
  }
}
```

**4. Panel Queries Updated:**
```typescript
// Panel receives updated queries with variables replaced
const query = "rate(http_requests_total{host=\"$host\"}[5m])";
// After variable replacement:
const resolvedQuery = "rate(http_requests_total{host=\"server2\"}[5m])";

// Query executed with resolved variable
const request: DataQueryRequest = {
  targets: [{
    expr: resolvedQuery,  // Variable already replaced
  }],
  range: this.props.timeRange,
  scopedVars: this.props.variables,  // Variables also passed for reference
};
```

---

## 3. Loose Coupling Mechanisms

### 3.1 Prop-Based Communication

Panels receive state through **props**, not direct references:

```typescript
interface PanelProps {
  // State from dashboard (passed as props)
  timeRange: TimeRange;
  variables: VariableModel[];
  refreshInterval: string;
  
  // Panel-specific
  panel: PanelModel;
  width: number;
  height: number;
  
  // Callbacks (one-way communication)
  onOptionsChange: (options: PanelOptions) => void;
}
```

**Benefits:**
- **No Direct References**: Panels don't hold references to other panels
- **Immutable Props**: Props are read-only, preventing accidental mutations
- **Clear Dependencies**: Dependencies are explicit in prop types
- **Testability**: Easy to test panels with mock props

### 3.2 Event-Driven Architecture

State changes propagate through **events**, not direct method calls:

```typescript
// DashboardModel emits events
class DashboardModel {
  private eventBus: EventEmitter;
  
  updateTimeRange(newTimeRange: TimeRange): void {
    this.time = newTimeRange;
    this.eventBus.emit('timeRangeChanged', { timeRange: newTimeRange });
  }
}

// Dashboard component subscribes
class DashboardComponent {
  componentDidMount() {
    this.dashboard.eventBus.on('timeRangeChanged', this.handleTimeRangeChange);
  }
  
  handleTimeRangeChange = (event: TimeRangeChangedEvent) => {
    // Update state, which triggers re-render
    this.setState({ timeRange: event.timeRange });
  };
}
```

**Benefits:**
- **Decoupling**: DashboardModel doesn't know about panels
- **Extensibility**: Easy to add new subscribers
- **Flexibility**: Multiple components can subscribe to same events
- **Testability**: Events can be mocked and tested independently

### 3.3 Query Re-execution Pattern

Panels re-execute queries when state changes, rather than sharing query results:

```typescript
class PanelComponent extends React.Component<PanelProps> {
  componentDidUpdate(prevProps: PanelProps) {
    // Check what changed
    const timeRangeChanged = prevProps.timeRange !== this.props.timeRange;
    const variablesChanged = prevProps.variables !== this.props.variables;
    
    if (timeRangeChanged || variablesChanged) {
      // Re-execute queries with new state
      this.executeQueries();
    }
  }
  
  executeQueries(): void {
    // Each panel executes its own queries
    // No sharing of query results between panels
    const request = this.buildQueryRequest();
    this.dataSource.query(request).then(response => {
      this.setState({ data: response.data });
    });
  }
}
```

**Benefits:**
- **Independence**: Each panel queries independently
- **No Shared State**: Panels don't share query results
- **Consistency**: All panels use same time range/variables
- **Flexibility**: Panels can have different data sources

---

## 4. Cursor Synchronization (Shared Crosshair/Tooltip)

### 4.1 Cursor Sync Mechanism

Grafana supports **cursor synchronization** where hovering over one panel shows crosshair/tooltip on all panels:

**Configuration:**
```typescript
// Dashboard settings
dashboard.graphTooltip = 1;  // 0: Default, 1: Shared crosshair, 2: Shared tooltip
```

**Implementation:**
```typescript
class DashboardComponent {
  // Cursor position state
  state = {
    cursorTime: null as number | null,  // Timestamp of cursor position
  };
  
  // Handle cursor move on any panel
  handlePanelCursorMove = (timestamp: number) => {
    // Update cursor position
    this.setState({ cursorTime: timestamp });
    
    // All panels receive cursor time as prop
    // They render crosshair/tooltip at that time
  };
  
  render() {
    return (
      <div>
        {this.dashboard.panels.map(panel => (
          <PanelComponent
            key={panel.id}
            panel={panel}
            timeRange={this.state.timeRange}
            cursorTime={this.state.cursorTime}  // Cursor position passed to all panels
            onCursorMove={this.handlePanelCursorMove}
          />
        ))}
      </div>
    );
  }
}
```

**Panel Cursor Handling:**
```typescript
class PanelComponent extends React.Component<PanelProps> {
  handleMouseMove = (event: MouseEvent) => {
    // Calculate timestamp from mouse position
    const timestamp = this.getTimestampFromPosition(event.clientX);
    
    // Notify dashboard of cursor move
    this.props.onCursorMove(timestamp);
  };
  
  render() {
    const { cursorTime, data } = this.props;
    
    return (
      <div onMouseMove={this.handleMouseMove}>
        <TimeSeriesChart
          data={data}
          cursorTime={cursorTime}  // Render crosshair at cursor time
        />
      </div>
    );
  }
}
```

**Benefits:**
- **Synchronized Viewing**: All panels show same time point
- **No Direct Coupling**: Panels communicate through dashboard state
- **Optional Feature**: Can be disabled per dashboard

---

## 5. Variable Dependency Chains

### 5.1 Chained Variables

Variables can depend on other variables, creating **dependency chains**:

```typescript
// Variable 1: Region
const regionVariable: VariableModel = {
  name: "region",
  type: "query",
  query: "SELECT region FROM regions",
  current: { value: "us-east", text: "US East" },
};

// Variable 2: Server (depends on region)
const serverVariable: VariableModel = {
  name: "server",
  type: "query",
  query: "SELECT server FROM servers WHERE region = '$region'",  // Uses $region
  current: { value: "server1", text: "Server 1" },
};
```

**Dependency Resolution:**
```typescript
class DashboardModel {
  updateVariable(variableName: string, value: any): void {
    // Update variable
    const variable = this.templating.list.find(v => v.name === variableName);
    variable.current.value = value;
    
    // Check for dependent variables
    const dependentVariables = this.findDependentVariables(variableName);
    
    // Update dependent variables
    dependentVariables.forEach(depVar => {
      // Re-execute query with new parent value
      this.refreshVariable(depVar);
    });
    
    // Emit events
    this.eventBus.emit('variableChanged', { variable: variableName, value });
    dependentVariables.forEach(depVar => {
      this.eventBus.emit('variableChanged', { variable: depVar.name });
    });
    
    // Refresh dashboard
    this.refreshDashboard();
  }
  
  findDependentVariables(variableName: string): VariableModel[] {
    return this.templating.list.filter(variable => {
      // Check if variable query uses parent variable
      if (variable.query && variable.query.includes(`$${variableName}`)) {
        return true;
      }
      return false;
    });
  }
}
```

**Propagation Flow:**
```
User Changes Region Variable
    ↓
Region Variable Updated
    ↓
Dependent Variables Detected (Server)
    ↓
Server Variable Query Re-executed (with new region)
    ↓
Server Variable Options Updated
    ↓
All Panels Receive Updated Variables
    ↓
All Panel Queries Re-executed (with new variables)
```

---

## 6. URL Synchronization

### 6.1 State to URL Sync

Dashboard state is synchronized to the URL for sharing and bookmarking:

```typescript
class DashboardComponent {
  componentDidUpdate(prevProps: any, prevState: any) {
    // Check if state changed
    if (this.state.timeRange !== prevState.timeRange ||
        this.state.variables !== prevState.variables) {
      // Update URL
      this.updateURL();
    }
  }
  
  updateURL(): void {
    const params = new URLSearchParams();
    
    // Add time range
    params.set('from', this.state.timeRange.raw.from);
    params.set('to', this.state.timeRange.raw.to);
    
    // Add variables
    this.state.variables.forEach(variable => {
      params.set(`var-${variable.name}`, variable.current.value);
    });
    
    // Update URL without page reload
    window.history.pushState({}, '', `?${params.toString()}`);
  }
}
```

**URL Format:**
```
https://grafana.example.com/d/dashboard-uid?from=now-6h&to=now&var-host=server1&var-region=us-east
```

### 6.2 URL to State Sync

When dashboard loads, URL parameters restore state:

```typescript
class DashboardComponent {
  componentDidMount() {
    // Parse URL parameters
    const urlParams = new URLSearchParams(window.location.search);
    
    // Restore time range
    const from = urlParams.get('from') || 'now-6h';
    const to = urlParams.get('to') || 'now';
    const timeRange = {
      from: parseTime(from),
      to: parseTime(to),
      raw: { from, to },
    };
    
    // Restore variables
    const variables = this.dashboard.templating.list.map(variable => {
      const urlValue = urlParams.get(`var-${variable.name}`);
      if (urlValue) {
        variable.current.value = urlValue;
      }
      return variable;
    });
    
    // Update dashboard state
    this.dashboard.updateTimeRange(timeRange);
    variables.forEach(v => {
      this.dashboard.updateVariable(v.name, v.current.value);
    });
  }
}
```

---

## 7. Refresh Synchronization

### 7.1 Dashboard-Level Refresh

When refresh is triggered, all panels refresh simultaneously:

```typescript
class DashboardModel {
  refreshDashboard(): void {
    // Cancel pending requests
    this.cancelPendingRequests();
    
    // Emit refresh event
    this.eventBus.emit('refresh', {
      timestamp: Date.now(),
    });
    
    // Update refresh state
    this.refreshState.isRefreshing = true;
    this.refreshState.lastRefresh = Date.now();
  }
}
```

**Panel Refresh Handling:**
```typescript
class PanelComponent extends React.Component<PanelProps> {
  componentDidMount() {
    // Subscribe to refresh events
    this.props.dashboard.eventBus.on('refresh', this.handleRefresh);
  }
  
  handleRefresh = () => {
    // Re-execute queries
    this.executeQueries();
  };
  
  executeQueries(): void {
    // Build query request with current state
    const request: DataQueryRequest = {
      targets: this.panel.targets,
      range: this.props.timeRange,      // Current time range
      scopedVars: this.props.variables, // Current variables
      // ... other parameters
    };
    
    // Execute
    this.dataSource.query(request).then(response => {
      this.setState({ data: response.data });
    });
  }
}
```

### 7.2 Auto-Refresh Synchronization

Auto-refresh triggers dashboard-level refresh:

```typescript
class DashboardComponent {
  componentDidMount() {
    // Set up auto-refresh timer
    if (this.dashboard.refresh) {
      const interval = parseRefreshInterval(this.dashboard.refresh);
      this.autoRefreshTimer = setInterval(() => {
        this.dashboard.refreshDashboard();
      }, interval);
    }
  }
  
  componentWillUnmount() {
    // Clean up timer
    if (this.autoRefreshTimer) {
      clearInterval(this.autoRefreshTimer);
    }
  }
}
```

---

## 8. Panel Independence Guarantees

### 8.1 No Direct Panel References

Panels never reference each other directly:

```typescript
// ❌ Wrong: Panel references another panel
class PanelComponent {
  render() {
    const otherPanel = this.props.dashboard.panels.find(p => p.id === 'other');
    return <div>{otherPanel.data}</div>;
  }
}

// ✅ Correct: Panel only uses its own data
class PanelComponent {
  render() {
    return <div>{this.state.data}</div>;
  }
}
```

### 8.2 Shared State Through Props Only

Panels receive shared state only through props:

```typescript
// ✅ Correct: State passed as props
<PanelComponent
  timeRange={dashboard.timeRange}      // Shared state
  variables={dashboard.templating.list} // Shared state
  panel={panel}                        // Panel-specific
/>

// ❌ Wrong: Panel accesses dashboard directly
class PanelComponent {
  render() {
    const timeRange = this.props.dashboard.timeRange;  // Direct access
    // ...
  }
}
```

### 8.3 Independent Query Execution

Each panel executes queries independently:

```typescript
// Each panel has its own query execution
class PanelComponent {
  executeQueries(): void {
    // Panel-specific queries
    const request: DataQueryRequest = {
      targets: this.panel.targets,  // This panel's queries only
      range: this.props.timeRange,   // Shared time range
      // ...
    };
    
    // Execute independently
    this.dataSource.query(request).then(response => {
      // Store in panel's own state
      this.setState({ data: response.data });
    });
  }
}
```

---

## 9. Complete Synchronization Flow Example

### 9.1 Time Range Change Flow

```
1. User changes time range in time picker
   ↓
2. TimeRangeInput component calls onChange(newTimeRange)
   ↓
3. DashboardComponent.handleTimeRangeChange(newTimeRange)
   ↓
4. DashboardModel.updateTimeRange(newTimeRange)
   ↓
5. DashboardModel.time = newTimeRange
   ↓
6. DashboardModel.eventBus.emit('timeRangeChanged', { timeRange })
   ↓
7. DashboardComponent receives event
   ↓
8. DashboardComponent.setState({ timeRange: newTimeRange })
   ↓
9. React re-renders DashboardComponent
   ↓
10. All PanelComponents receive new timeRange prop
   ↓
11. Each PanelComponent.componentDidUpdate() detects prop change
   ↓
12. Each PanelComponent.executeQueries() with new timeRange
   ↓
13. Each panel queries data source independently
   ↓
14. Each panel receives query response
   ↓
15. Each panel updates its own state with new data
   ↓
16. Each panel re-renders with new data
```

### 9.2 Variable Change Flow

```
1. User selects new variable value in dropdown
   ↓
2. VariableDropdown calls onChange(newValue)
   ↓
3. DashboardComponent.handleVariableChange(variableName, newValue)
   ↓
4. DashboardModel.updateVariable(variableName, newValue)
   ↓
5. DashboardModel finds dependent variables
   ↓
6. DashboardModel refreshes dependent variables
   ↓
7. DashboardModel.eventBus.emit('variableChanged', { variable, value })
   ↓
8. DashboardComponent receives event
   ↓
9. DashboardComponent.replaceVariablesInQueries(panels, variables)
   ↓
10. DashboardComponent.setState({ variables: updatedVariables })
   ↓
11. React re-renders DashboardComponent
   ↓
12. All PanelComponents receive new variables prop
   ↓
13. Each PanelComponent.componentDidUpdate() detects prop change
   ↓
14. Each PanelComponent.executeQueries() with new variables
   ↓
15. Each panel queries with resolved variable values
   ↓
16. Each panel receives query response
   ↓
17. Each panel updates state and re-renders
```

---

## 10. Benefits of Loose Coupling

### 10.1 Maintainability

**Benefits:**
- **Isolated Changes**: Changes to one panel don't affect others
- **Clear Dependencies**: Dependencies are explicit in props
- **Easy Testing**: Panels can be tested in isolation
- **Reduced Complexity**: No complex interdependencies

### 10.2 Extensibility

**Benefits:**
- **Easy to Add Panels**: New panels automatically receive shared state
- **Plugin Support**: Third-party panels work seamlessly
- **Flexible Architecture**: Easy to add new state types
- **No Breaking Changes**: Adding panels doesn't break existing ones

### 10.3 Performance

**Benefits:**
- **Parallel Execution**: Panels query independently in parallel
- **Selective Updates**: Only affected panels re-render
- **No Shared State Overhead**: No complex state synchronization
- **Efficient Rendering**: React optimizes re-renders based on prop changes

### 10.4 User Experience

**Benefits:**
- **Consistent State**: All panels always show same time range/variables
- **Synchronized Updates**: All panels update together
- **URL Sharing**: State can be shared via URL
- **Predictable Behavior**: Clear, consistent behavior across panels

---

## 11. Summary

Grafana's panel synchronization architecture provides:

1. **Centralized State**: DashboardModel as single source of truth
2. **Prop-Based Communication**: Panels receive state through props
3. **Event-Driven Updates**: State changes propagate through events
4. **Independent Execution**: Each panel queries independently
5. **Loose Coupling**: Panels don't know about each other
6. **Automatic Synchronization**: All panels stay in sync automatically
7. **URL Integration**: State synchronized with URL for sharing

This architecture enables:
- **Consistency**: All panels always show same time range/variables
- **Flexibility**: Easy to add/remove panels
- **Performance**: Parallel query execution
- **Maintainability**: Clear, simple architecture
- **Extensibility**: Easy to add new state types

The system balances **synchronization** (all panels stay in sync) with **independence** (panels don't depend on each other), making Grafana dashboards both powerful and maintainable.


