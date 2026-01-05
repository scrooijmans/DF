# GoldenLayout Component Communication, State Sharing, and Event Routing

This document explains how components in GoldenLayout communicate with each other, share state without tight coupling, and how events are routed through the layout system, based on the official GoldenLayout documentation.

## 1. Event-Driven Architecture

### EventEmitter Foundation

GoldenLayout uses an event-driven architecture built on `EventEmitter`. Multiple classes in the hierarchy extend `EventEmitter`, creating a distributed event system:

```typescript
// Base classes that extend EventEmitter
abstract class ContentItem extends EventEmitter { }
class ComponentContainer extends EventEmitter { }
class BrowserPopout extends EventEmitter { }
```

This design allows components to communicate without direct references, promoting loose coupling.

### Event Hub for Global Communication

The `LayoutManager` provides an `eventHub` property that serves as a central event bus for cross-component communication:

```typescript
// Listening for user broadcasts
layoutManager.eventHub.on('userBroadcast', (...ev: EventEmitter.UnknownParams) => {
  // respond to user broadcast event
});

// Emitting user broadcasts (from any component)
layoutManager.eventHub.emit('userBroadcast', data);
```

The EventHub enables:
- **Decoupled Communication**: Components don't need direct references to each other
- **Cross-Window Communication**: Events propagate to popout windows
- **Global Event Broadcasting**: Any component can emit/listen to events

## 2. Component Communication Patterns

### Pattern 1: EventHub Broadcasting

The most decoupled communication pattern uses the EventHub:

```typescript
// Component A: Emitting an event
class DataViewerComponent {
    constructor(container: ComponentContainer) {
        this.container = container;
        this.setupEventListeners();
    }
    
    private setupEventListeners() {
        // Listen for data updates
        this.container.layoutManager.eventHub.on('dataUpdated', (data) => {
            this.updateView(data);
        });
    }
    
    public updateData(newData: any) {
        // Broadcast to all components
        this.container.layoutManager.eventHub.emit('dataUpdated', newData);
    }
}

// Component B: Listening to the same event
class ChartComponent {
    constructor(container: ComponentContainer) {
        this.container = container;
        this.container.layoutManager.eventHub.on('dataUpdated', (data) => {
            this.updateChart(data);
        });
    }
}
```

**Benefits:**
- No direct coupling between components
- Multiple listeners can respond to the same event
- Works across popout windows
- Easy to add/remove components without breaking communication

### Pattern 2: ComponentContainer Events

Components can listen to events on their own `ComponentContainer`:

```typescript
class MyComponent {
    constructor(container: ComponentContainer) {
        this.container = container;
        
        // Listen to container lifecycle events
        this.container.on('show', () => this.onShow());
        this.container.on('hide', () => this.onHide());
        this.container.on('focus', () => this.onFocus());
        this.container.on('blur', () => this.onBlur());
    }
    
    private onShow() {
        // Component became visible
    }
    
    private onHide() {
        // Component was hidden
    }
    
    private onFocus() {
        // Component received focus
    }
    
    private onBlur() {
        // Component lost focus
    }
}
```

### Pattern 3: ContentItem Tree Events

Since `ContentItem` extends `EventEmitter`, events can propagate through the layout tree:

```typescript
// Emit event on a ContentItem
const componentItem = layoutManager.rootItem.children[0] as ComponentItem;
componentItem.emit('customEvent', data);

// Listen on parent or child items
const parent = componentItem.parent;
if (parent) {
    parent.on('customEvent', (data) => {
        // Handle event from child
    });
}
```

### Pattern 4: Direct Container Access (Tightly Coupled - Not Recommended)

While possible, direct access creates tight coupling:

```typescript
// Get all component containers
const allContainers = layoutManager.rootItem
    .getAllComponents()
    .map(item => item.container);

// Direct method call (creates coupling)
allContainers[0].component.someMethod();
```

**This pattern should be avoided** in favor of event-based communication.

## 3. State Sharing Without Tight Coupling

### Component State in Configuration

Each component can have initial state defined in its configuration:

```typescript
interface ResolvedComponentItemConfig {
    readonly componentState?: JsonValue;  // Initial state
    readonly componentType: JsonValue;
    // ...
}
```

State is passed during component creation:

```typescript
const config: ComponentItemConfig = {
    type: 'component',
    componentType: 'DataViewer',
    componentState: {
        dataSource: 'table1',
        filters: { status: 'active' },
        viewMode: 'grid'
    }
};
```

### State Request Event

Components can request their current state through the `stateRequestEvent`:

```typescript
class ComponentContainer {
    // Event handler that components can set
    stateRequestEvent: ComponentContainer.StateRequestEventHandler | undefined;
    
    // Get current state
    get state(): JsonValue | undefined {
        if (this.stateRequestEvent) {
            return this.stateRequestEvent();
        }
        return undefined;
    }
}
```

**Usage:**

```typescript
class MyComponent {
    constructor(container: ComponentContainer) {
        this.container = container;
        
        // Register state request handler
        this.container.stateRequestEvent = () => {
            return {
                selectedItems: this.selectedItems,
                viewMode: this.viewMode,
                filters: this.filters
            };
        };
    }
    
    // State is automatically requested when layout is saved
    // No need to manually call anything
}
```

### State Sharing via EventHub

Components can share state changes through events:

```typescript
// State management service (optional pattern)
class StateManager {
    private state: Map<string, any> = new Map();
    
    constructor(private eventHub: EventEmitter) {
        // Listen for state updates
        this.eventHub.on('stateUpdate', (key: string, value: any) => {
            this.state.set(key, value);
            // Notify all listeners
            this.eventHub.emit('stateChanged', key, value);
        });
        
        // Listen for state requests
        this.eventHub.on('stateRequest', (key: string) => {
            const value = this.state.get(key);
            this.eventHub.emit('stateResponse', key, value);
        });
    }
    
    public updateState(key: string, value: any) {
        this.eventHub.emit('stateUpdate', key, value);
    }
}

// Component A: Updates shared state
class EditorComponent {
    constructor(container: ComponentContainer) {
        this.container = container;
        const stateManager = new StateManager(container.layoutManager.eventHub);
        
        this.onDataChange = (data) => {
            stateManager.updateState('editorData', data);
        };
    }
}

// Component B: Reacts to shared state changes
class PreviewComponent {
    constructor(container: ComponentContainer) {
        this.container = container;
        
        container.layoutManager.eventHub.on('stateChanged', (key, value) => {
            if (key === 'editorData') {
                this.updatePreview(value);
            }
        });
    }
}
```

### State Persistence

State can be persisted by saving the layout configuration:

```typescript
// Save layout with component states
const config = layoutManager.saveLayout();
// config.root contains all component states in componentState properties

// Load layout with states
layoutManager.loadLayout(config);
// Components receive their state during bindComponentEvent
```

## 4. Event Routing Through the Layout System

### Event Flow Hierarchy

Events flow through multiple layers in GoldenLayout:

```
LayoutManager (eventHub)
    ↓
ContentItem (root → children)
    ↓
ComponentItem
    ↓
ComponentContainer
    ↓
Component (your code)
```

### Routing Path 1: Top-Down (Layout → Component)

Events flow from layout system to components:

```typescript
// 1. LayoutManager triggers layout change
layoutManager.updateSize();

// 2. ContentItem propagates to children
class ContentItem {
    updateSize(force: boolean) {
        // Emit internal event
        this.emit('sizeChanged', this.width, this.height);
        
        // Update children
        this.children.forEach(child => {
            child.updateSize(force);
        });
    }
}

// 3. ComponentItem updates ComponentContainer
class ComponentItem {
    updateSize(force: boolean) {
        this.container.setSize(this.width, this.height);
    }
}

// 4. ComponentContainer notifies component
class ComponentContainer {
    setSize(width: number, height: number) {
        // Trigger virtual recting if needed
        if (this.virtual) {
            this.virtualRectingRequiredEvent?.(this, width, height);
        }
        
        // Emit size change event
        this.emit('sizeChanged', width, height);
    }
}
```

### Routing Path 2: Bottom-Up (Component → Layout)

Events flow from components to layout system:

```typescript
// 1. Component emits event
class MyComponent {
    requestClose() {
        // Access container
        this.container.close();
    }
}

// 2. ComponentContainer handles close
class ComponentContainer {
    close() {
        // Emit close event
        this.emit('close');
        
        // Notify parent
        this.parent.close();
    }
}

// 3. ComponentItem removes itself
class ComponentItem {
    close() {
        // Remove from parent
        this.parent.removeChild(this);
        
        // Trigger unbind
        this.layoutManager.unbindComponent(this.container);
    }
}
```

### Routing Path 3: Horizontal (Component → Component)

Events flow between sibling components:

```typescript
// Component A emits to EventHub
class ComponentA {
    notifySiblings(data: any) {
        this.container.layoutManager.eventHub.emit('sharedData', data);
    }
}

// Component B listens on EventHub
class ComponentB {
    constructor(container: ComponentContainer) {
        this.container = container;
        
        // Listen for sibling events
        container.layoutManager.eventHub.on('sharedData', (data) => {
            this.handleData(data);
        });
    }
}
```

### Event Handler Types

GoldenLayout defines specific event handler types for different scenarios:

```typescript
namespace ComponentContainer {
    // Lifecycle events
    export type ShowEventHandler = (this: void) => void;
    export type HideEventHandler = (this: void) => void;
    export type FocusEventHandler = (this: void, suppressEvent: boolean) => void;
    export type BlurEventHandler = (this: void, suppressEvent: boolean) => void;
    
    // State management
    export type StateRequestEventHandler = (this: void) => JsonValue | undefined;
    export type UpdateItemConfigEventHandler = (itemConfig: ResolvedComponentItemConfig) => void;
    
    // Virtual component events
    export type VirtualRectingRequiredEvent = (
        this: void, 
        container: ComponentContainer, 
        width: number, 
        height: number
    ) => void;
    export type VirtualVisibilityChangeRequiredEvent = (
        this: void, 
        container: ComponentContainer, 
        visible: boolean
    ) => void;
    export type VirtualZIndexChangeRequiredEvent = (
        this: void, 
        container: ComponentContainer, 
        logicalZIndex: LogicalZIndex, 
        defaultZIndex: string
    ) => void;
}

namespace Tab {
    export type CloseEvent = (componentItem: ComponentItem) => void;
    export type DragStartEvent = (
        x: number, 
        y: number, 
        dragListener: DragListener, 
        componentItem: ComponentItem
    ) => void;
    export type FocusEvent = (componentItem: ComponentItem) => void;
}
```

### Focus and Blur Event Routing

Focus events demonstrate event routing through the system:

```typescript
// 1. User clicks on component tab
// Tab emits focus event
tab.emit('focus', componentItem);

// 2. ComponentItem handles focus
class ComponentItem {
    focus(suppressEvent?: boolean) {
        // Set focused state
        this.setFocused(suppressEvent);
        
        // Blur previously focused item
        if (this.layoutManager.focusedComponentItem) {
            this.layoutManager.focusedComponentItem.blur(true);
        }
        
        // Update layout manager
        this.layoutManager.focusedComponentItem = this;
        
        // Notify container
        this.container.focus(suppressEvent);
    }
}

// 3. ComponentContainer emits focus event
class ComponentContainer {
    focus(suppressEvent?: boolean) {
        if (this._focusEvent) {
            this._focusEvent(suppressEvent);
        }
        
        // Emit to component
        this.emit('focus', suppressEvent);
    }
}

// 4. Component receives focus event
class MyComponent {
    constructor(container: ComponentContainer) {
        container.on('focus', () => {
            this.onFocus();
        });
    }
}
```

## 5. Best Practices for Decoupled Communication

### 1. Use EventHub for Cross-Component Communication

```typescript
// ✅ Good: Decoupled via EventHub
class ComponentA {
    notify(data: any) {
        this.container.layoutManager.eventHub.emit('dataChanged', data);
    }
}

class ComponentB {
    constructor(container: ComponentContainer) {
        container.layoutManager.eventHub.on('dataChanged', (data) => {
            this.update(data);
        });
    }
}

// ❌ Bad: Direct reference
class ComponentA {
    constructor(public componentB: ComponentB) {}
    
    notify(data: any) {
        this.componentB.update(data);  // Tight coupling
    }
}
```

### 2. Use Typed Event Names

```typescript
// Define event names as constants
const Events = {
    DATA_UPDATED: 'dataUpdated',
    SELECTION_CHANGED: 'selectionChanged',
    FILTER_APPLIED: 'filterApplied'
} as const;

// Use typed events
layoutManager.eventHub.emit(Events.DATA_UPDATED, data);
layoutManager.eventHub.on(Events.DATA_UPDATED, (data) => {
    // Handle event
});
```

### 3. Clean Up Event Listeners

```typescript
class MyComponent {
    private eventHandlers: Array<() => void> = [];
    
    constructor(container: ComponentContainer) {
        // Store cleanup functions
        const handler = (data: any) => this.handleData(data);
        container.layoutManager.eventHub.on('dataUpdated', handler);
        
        this.eventHandlers.push(() => {
            container.layoutManager.eventHub.off('dataUpdated', handler);
        });
    }
    
    destroy() {
        // Clean up all listeners
        this.eventHandlers.forEach(cleanup => cleanup());
    }
}
```

### 4. Use State Request Pattern for Persistence

```typescript
class MyComponent {
    constructor(container: ComponentContainer) {
        // Register state request handler
        container.stateRequestEvent = () => {
            return {
                selectedItems: this.selectedItems,
                viewMode: this.viewMode
            };
        };
        
        // State is automatically requested when layout is saved
        // No manual state management needed
    }
}
```

### 5. Avoid Direct Component References

```typescript
// ✅ Good: Access via layout manager
const allComponents = layoutManager.rootItem.getAllComponents();

// ❌ Bad: Store direct references
const componentRefs: ComponentItem[] = [];  // Can become stale
```

## 6. Example: Multi-Component Data Flow

Here's a complete example showing decoupled communication:

```typescript
// Data Service (manages shared state)
class DataService {
    private data: any = null;
    
    constructor(private eventHub: EventEmitter) {
        // Listen for data updates
        eventHub.on('dataUpdate', (newData: any) => {
            this.data = newData;
            // Broadcast to all components
            eventHub.emit('dataChanged', newData);
        });
    }
    
    public updateData(newData: any) {
        this.eventHub.emit('dataUpdate', newData);
    }
    
    public getData() {
        return this.data;
    }
}

// Editor Component (updates data)
class EditorComponent {
    private dataService: DataService;
    
    constructor(container: ComponentContainer) {
        this.dataService = new DataService(container.layoutManager.eventHub);
        
        // Listen for external data changes
        container.layoutManager.eventHub.on('dataChanged', (data) => {
            this.loadData(data);
        });
    }
    
    public saveChanges(data: any) {
        // Update shared data
        this.dataService.updateData(data);
        // Other components will be notified via eventHub
    }
}

// Viewer Component (displays data)
class ViewerComponent {
    constructor(container: ComponentContainer) {
        // Listen for data changes
        container.layoutManager.eventHub.on('dataChanged', (data) => {
            this.render(data);
        });
        
        // Request initial data
        const dataService = new DataService(container.layoutManager.eventHub);
        const initialData = dataService.getData();
        if (initialData) {
            this.render(initialData);
        }
    }
}

// Chart Component (visualizes data)
class ChartComponent {
    constructor(container: ComponentContainer) {
        // Listen for data changes
        container.layoutManager.eventHub.on('dataChanged', (data) => {
            this.updateChart(data);
        });
    }
}
```

## Summary

GoldenLayout enables decoupled component communication through:

1. **EventHub**: Central event bus for global communication
2. **EventEmitter Inheritance**: All layout items can emit/listen to events
3. **State Management**: Component state in configuration + state request events
4. **Event Routing**: Hierarchical event flow through layout tree
5. **Lifecycle Events**: Show/hide/focus/blur events for component coordination

This architecture ensures:
- **Loose Coupling**: Components don't need direct references
- **Scalability**: Easy to add/remove components
- **Maintainability**: Clear communication patterns
- **Flexibility**: Multiple communication patterns available
- **Cross-Window Support**: Events work across popout windows

Components communicate through events rather than direct method calls, creating a flexible and maintainable architecture.


