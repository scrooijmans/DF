# JupyterLab Widget Lifecycle Management

This document explains how JupyterLab manages widget lifecycles, including creation, disposal, rehydration, and how application state is synchronized across sessions.

## Overview

JupyterLab uses a sophisticated widget lifecycle management system built on:
- **Lumino Widgets**: Core widget infrastructure
- **Widget Trackers**: Track and manage collections of widgets
- **State Database (StateDB)**: Persistent storage for widget state
- **Layout Restorer**: Coordinates restoration of widgets and layouts
- **Document Registry**: Manages document models and contexts

## Widget Creation

### Factory Pattern

Widgets are created through factory classes that implement `IWidgetFactory`:

```typescript
interface IWidgetFactory<T extends IDocumentWidget> {
    createNew(
        context: IContext<DocumentRegistry.IModel>,
        source?: T
    ): T;
}
```

**Key Points:**
- Factories receive a document context (which includes the model)
- An optional source widget can be provided for cloning/duplication
- Factories emit a `widgetCreated` signal upon successful creation
- The factory pattern allows different widget types for different document types

### Example: Creating a Widget

```typescript
// Widget factory creates a new widget
const widget = factory.createNew(context, sourceWidget);

// Factory emits widgetCreated signal
factory.widgetCreated.connect((factory, widget) => {
    // Handle newly created widget
});
```

### Document Context

Every widget is associated with a `DocumentRegistry.Context` that provides:
- **Model**: The underlying data model (notebook, file, etc.)
- **Path**: File path or identifier
- **Session**: Kernel/session connection (for notebooks)
- **Save State**: Signal for monitoring save operations

```typescript
interface IContext<T extends IModel> {
    model: T;
    path: string;
    sessionContext?: ISessionContext;
    saveState: ISignal<IContext<T>, SaveState>;
    addSibling(widget: Widget, options?: IOpenOptions): IDisposable;
}
```

## Widget Tracking

### WidgetTracker

Widgets are tracked using `WidgetTracker` instances, which maintain collections of widgets and provide lifecycle management:

```typescript
class WidgetTracker<T extends Widget> implements IWidgetTracker<T> {
    add(widget: T): Promise<void>;
    restore(options?: IRestorable.IOptions<T>): Promise<any>;
    inject(obj: T): void;
    defer(options: IRestorable.IOptions<T>): void;
    
    // Signals
    widgetAdded: ISignal<this, T>;
    currentChanged: ISignal<this, null | T>;
    
    // Properties
    restored: Promise<void>;
    current: T | null;
}
```

### Adding Widgets to Trackers

When a widget is added to a tracker:

```typescript
await tracker.add(widget: T): Promise<void>
```

**Process:**
1. Widget is added synchronously to the tracker's collection
2. Widget becomes the current widget (unless tracker already has focus)
3. `widgetAdded` signal is emitted
4. Widget state is saved to the restoration connector (if available)
5. Promise resolves after save completes

**Important:** The `add()` method saves widget state automatically, enabling restoration later.

### Widget Injection

Widgets can be injected into trackers without triggering restoration lifecycle:

```typescript
tracker.inject(widget: T): void
```

**Use Cases:**
- Adding widgets that don't need restoration
- Programmatically adding widgets without state persistence
- `widgetAdded` signal does NOT fire for injected widgets

### Tracker Signals

Trackers emit signals for lifecycle events:

```typescript
// Emitted when a widget is explicitly added (not injected)
tracker.widgetAdded.connect((tracker, widget) => {
    // Handle new widget
});

// Emitted when the current widget changes
tracker.currentChanged.connect((tracker, widget) => {
    // widget is null if last tracked widget was disposed
});
```

## Widget Disposal

### Disposal Process

Widgets implement the `IDisposable` interface and must be properly disposed:

```typescript
widget.dispose(): void
```

**Disposal Behavior:**
1. Widget and all descendant widgets are disposed
2. Resources are released (event listeners, timers, etc.)
3. Widget is removed from parent containers
4. `disposed` signal is emitted
4. Subsequent calls to `dispose()` are no-ops
5. Widget becomes unsafe to use after disposal

### Disposal Status

Check if a widget has been disposed:

```typescript
const isDisposed: boolean = widget.isDisposed
```

### Disposal Signal

React to widget disposal:

```typescript
widget.disposed.connect((widget) => {
    // Cleanup code
    // Remove from trackers, close connections, etc.
});
```

### Automatic Disposal

Widgets are automatically disposed when:
- Parent widget is disposed (cascading disposal)
- Layout is restored and widget is not in the new layout
- Document context is closed
- Application shuts down

### Best Practices

1. **Always dispose widgets**: Prevent memory leaks
2. **Check disposal status**: Before using widgets, verify `!widget.isDisposed`
3. **Listen to disposed signal**: Clean up external resources
4. **Remove from trackers**: Trackers should handle this, but verify
5. **Dispose in reverse order**: Dispose children before parents when manual cleanup is needed

## State Persistence

### State Database (StateDB)

JupyterLab uses `StateDB` for persistent storage of widget and application state:

```typescript
class StateDB<T extends ReadonlyPartialJSONValue> {
    save(id: string, value: T): Promise<void>;
    fetch(id: string): Promise<undefined | T>;
    remove(id: string): Promise<void>;
    list(namespace: string): Promise<{ ids: string[]; values: T[] }>;
    toJSON(): Promise<{ readonly [id: string]: T }>;
    clear(): Promise<void>;
    
    changed: ISignal<this, Change>;
}
```

### State Storage Keys

State keys follow the `namespace:identifier` convention:

```typescript
// Examples
"notebook:path/to/notebook.ipynb"
"filebrowser:listing"
"layout:main-area"
"widget-tracker:notebook"
```

**Benefits:**
- Namespace organization
- Enables `list(namespace)` to retrieve all items in a namespace
- Prevents key collisions
- Aligns with JupyterLab command identifier conventions

### Saving State

```typescript
await stateDB.save("namespace:identifier", {
    widgetId: "unique-id",
    layout: { /* layout config */ },
    preferences: { /* user prefs */ }
});
```

### Fetching State

```typescript
const state = await stateDB.fetch("namespace:identifier");
if (state) {
    // Restore widget from state
}
```

### Listing by Namespace

```typescript
const { ids, values } = await stateDB.list("widget-tracker");
// Returns all widgets in the tracker namespace
```

### State Change Monitoring

Monitor state changes:

```typescript
stateDB.changed.connect((db, change) => {
    // change.type: 'save' | 'remove'
    // change.id: string
    // change.newValue: T | undefined
    // change.oldValue: T | undefined
});
```

## Widget Rehydration

### IRestorable Interface

Widgets that can be restored implement `IRestorable`:

```typescript
interface IRestorable<T extends IObservableDisposable, U = any> {
    restored: Promise<U>;
    restore(options: IRestorable.IOptions<T>): Promise<U>;
}
```

**Restoration Options:**

```typescript
interface IRestorable.IOptions<T> {
    command: string;              // Command to create new widgets
    name: (widget: T) => string;  // Function to generate widget name
    when?: Promise<any>;          // Optional promise to wait for
    args?: (widget: T) => any;    // Function to generate command args
}
```

### Layout Restorer

The `LayoutRestorer` coordinates widget restoration:

```typescript
interface ILayoutRestorer {
    restored: Promise<void>;
    add(widget: Widget, name: string): void;
    restore<T extends Widget>(
        tracker: WidgetTracker<T>,
        options: IRestorer.IOptions<T>
    ): Promise<any>;
}
```

### Widget Registration

Before restoration, widgets must be registered:

```typescript
layoutRestorer.add(widget: Widget, name: string): void
```

**Purpose:**
- Associates widget with a unique name
- Enables layout restorer to track widget position
- Links widget to saved layout state

### Restoration Process

Restore widgets for a tracker:

```typescript
await layoutRestorer.restore(
    tracker: WidgetTracker<T>,
    options: IRestorer.IOptions<T>
): Promise<any>
```

**Restoration Steps:**

1. **Fetch Saved State**: Retrieve widget state from StateDB
2. **Create Widgets**: Use `options.command` to create widgets
3. **Rehydrate State**: Apply saved state to widgets
4. **Restore Layout**: Position widgets according to saved layout
5. **Resolve Promise**: Signal completion

**Restoration Options:**

```typescript
{
    command: "docmanager:open",           // Command to create widget
    name: (widget) => widget.context.path, // Unique identifier
    args: (widget) => ({ path: widget.context.path }), // Command args
    when: app.restored                   // Wait for app restoration
}
```

### Tracker Restoration

Trackers can restore their widgets:

```typescript
await tracker.restore(options?: IRestorable.IOptions<T>): Promise<any>
```

**Process:**
1. Fetches saved state for tracker namespace
2. Creates widgets using restoration options
3. Applies saved state to each widget
4. Resolves `tracker.restored` promise
5. Emits `widgetAdded` signals for restored widgets

**Note:** This is typically called by `LayoutRestorer`, not directly by client code.

### Deferred Restoration

Restoration can be deferred (useful for single-document mode):

```typescript
tracker.defer(options: IRestorable.IOptions<T>): void
```

**Use Case:**
- Avoids restoring widgets in single-document mode
- Saves restoration options for later
- Should not be called by client code in normal scenarios

### Restoration Promises

Multiple restoration promises are available:

```typescript
// Layout restorer ready
await layoutRestorer.restored

// Tracker restored
await tracker.restored

// Application restored
await app.restored

// Shell restored
await shell.restored
```

**Usage:**
- Coordinate actions after restoration
- Ensure dependencies are ready
- Chain restoration operations

## Application State Synchronization

### Application Startup

The application startup process coordinates restoration:

```typescript
await app.start(options?: IStartOptions): Promise<void>
```

**Startup Sequence:**

1. **Activate Startup Plugins**: Load and activate core plugins
2. **Wait for Activation**: Ensure plugins are ready
3. **Attach Shell to DOM**: Mount the application shell
4. **Add Event Listeners**: Set up application event handling
5. **Restore Layout**: Begin layout restoration process

### Layout Restoration

The shell restores layout state:

```typescript
await shell.restoreLayout(
    mode: Mode,
    layoutRestorer: LayoutRestorer,
    configuration?: { [m: string]: IUserLayout }
): Promise<void>
```

**Process:**

1. **Determine Mode**: `multiple-document` or `single-document`
2. **Fetch Layout State**: Retrieve saved layout from StateDB
3. **Restore Widget Trackers**: Restore widgets for each tracker
4. **Restore Layout Structure**: Rebuild dock panel layout
5. **Apply User Configuration**: Apply user customizations
6. **Resolve Restoration Promise**: Signal completion

**Important:** This should only be called once during initialization.

### State Synchronization Flow

```
┌─────────────────┐
│  Application    │
│     Start       │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  Load Plugins   │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  Initialize     │
│  StateDB        │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  Restore Layout │
└────────┬────────┘
         │
         ├─────────────────┐
         │                 │
         ▼                 ▼
┌─────────────────┐  ┌─────────────────┐
│ Restore Widget  │  │ Restore Widget  │
│   Trackers      │  │   Trackers      │
└────────┬────────┘  └────────┬────────┘
         │                    │
         └─────────┬──────────┘
                   │
                   ▼
         ┌─────────────────┐
         │ Rebuild Layout  │
         │   Structure     │
         └────────┬────────┘
                  │
                  ▼
         ┌─────────────────┐
         │  Apply User     │
         │  Configuration  │
         └────────┬────────┘
                  │
                  ▼
         ┌─────────────────┐
         │  Resolve        │
         │  Promises       │
         └─────────────────┘
```

### Saving Application State

The shell can save its current state:

```typescript
const layout: ILayout = shell.saveLayout()
```

**Returns:**
- Main area layout configuration
- Side panel layouts
- Widget states
- Mode information

This can be saved to StateDB for later restoration.

### State Persistence Across Sessions

**Saving State:**

1. **Widget State**: Saved when widgets are added to trackers
2. **Layout State**: Saved when layout changes
3. **User Preferences**: Saved when preferences change
4. **Document State**: Saved with document context

**State Storage:**

- **Browser Storage**: StateDB uses IndexedDB/localStorage
- **Server Storage**: Some state can be stored on Jupyter Server
- **Session Storage**: Temporary state for current session

**Restoration on Reload:**

1. Application starts
2. StateDB is initialized
3. Saved state is fetched
4. Widgets are recreated
5. Layout is restored
6. State is rehydrated

### Document Context State

Document widgets maintain state through their context:

```typescript
context.saveState: ISignal<IContext<T>, SaveState>
```

**Save States:**
- `'started'`: Save operation began
- `'completed'`: Save operation completed
- `'failed'`: Save operation failed

**State Management:**

```typescript
// Monitor save state
context.saveState.connect((context, state) => {
    if (state === 'completed') {
        // State persisted successfully
    }
});

// Save document
await context.save();
```

### Session Synchronization

For notebooks and interactive documents:

```typescript
context.sessionContext: ISessionContext
```

**Session Lifecycle:**

1. **Initialize**: Connect to or start kernel session
2. **Ready**: Session is ready for use
3. **Dispose**: Clean up session resources

**State Synchronization:**

- Kernel state is maintained by Jupyter Server
- Widget state is maintained by client StateDB
- Layout state is maintained by client StateDB
- Document content is maintained by Jupyter Server

## RestorablePool

For managing pools of restorable objects:

```typescript
class RestorablePool<T extends IObservableDisposable> {
    add(obj: T): void;
    save(obj: T): Promise<void>;
    restore(options: IRestorable.IOptions<T>): Promise<any>;
    
    currentChanged: ISignal<this, null | T>;
    restored: Promise<any>;
}
```

**Use Cases:**
- Managing collections of restorable widgets
- Coordinating restoration of related objects
- Providing current object tracking

## Best Practices

### Widget Creation

1. **Use Factories**: Always create widgets through factories
2. **Register with Tracker**: Add widgets to appropriate trackers
3. **Register with Restorer**: For widgets that need restoration
4. **Emit Signals**: Factories should emit `widgetCreated` signal

### Widget Disposal

1. **Always Dispose**: Prevent memory leaks
2. **Check Before Use**: Verify `!widget.isDisposed`
3. **Listen to Signals**: React to `disposed` signal
4. **Remove from Trackers**: Ensure trackers handle disposal
5. **Clean Up Resources**: Remove event listeners, timers, etc.

### State Management

1. **Use Namespaces**: Follow `namespace:identifier` convention
2. **Save Regularly**: Save state when widgets change
3. **Handle Missing State**: Gracefully handle `undefined` from `fetch()`
4. **Monitor Changes**: Use `changed` signal for reactive updates
5. **Serialize Properly**: Only store JSON-serializable data

### Restoration

1. **Wait for Ready**: Use `restored` promises before accessing widgets
2. **Provide Commands**: Restoration options must include valid commands
3. **Generate Unique Names**: Name functions must return unique identifiers
4. **Handle Failures**: Restoration may fail - handle gracefully
5. **Coordinate Dependencies**: Use `when` option for dependency ordering

### Session Synchronization

1. **Monitor Save State**: Track document save operations
2. **Handle Errors**: Gracefully handle save/restore failures
3. **Sync State**: Keep client and server state synchronized
4. **Handle Disconnections**: Manage state during network issues
5. **Clean Up Sessions**: Properly dispose session contexts

## Summary

JupyterLab's widget lifecycle management provides:

1. **Creation**: Factory pattern with document contexts
2. **Tracking**: WidgetTrackers manage collections and lifecycle
3. **Disposal**: Proper cleanup with signals and cascading disposal
4. **Persistence**: StateDB for storing widget and layout state
5. **Rehydration**: LayoutRestorer coordinates restoration
6. **Synchronization**: State persists across sessions via StateDB
7. **Signals**: Event-driven architecture for lifecycle events

The system ensures widgets are properly created, tracked, persisted, and restored across application sessions, providing a seamless user experience while maintaining state consistency.


