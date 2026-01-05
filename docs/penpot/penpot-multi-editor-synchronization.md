# Penpot Multi-Editor Synchronization Architecture

## Overview

Penpot implements a sophisticated event-driven architecture that synchronizes multiple editors, canvases, and panels while maintaining strict separation between the data model and views. This document explains how Penpot prevents views from owning or diverging from the data model through its event-driven state update system.

## Core Architecture Principles

### 1. Single Source of Truth: The Data Model

Penpot maintains a clear hierarchical data model that serves as the single source of truth:

```
File
├── Pages (Container)
│   └── Shapes (ShapeTree)
├── Components (Container)
│   └── Shapes (ShapeTree)
├── Colors
├── MediaItems
└── Typographies
```

**Key Entities:**

- **Container**: Abstraction for Pages and Components that share common functions for managing trees of shapes
- **Shape**: Core entity with composition relationships to Selrect, Transform, Constraints, Interactions, Fill, Stroke, Shadow, Blur, Font, Content, and Exports
- **ShapeTree**: Hierarchical relationship of shapes where frames contain top-level shapes, and frames/groups may contain non-frame shapes

The data model is **immutable** and **versioned**, ensuring that all views derive from the same authoritative state.

### 2. Event-Driven State Updates

Penpot uses an event-driven architecture where:

1. **User actions generate events** - When a user interacts with the UI (creating, moving, editing shapes), events are dispatched
2. **Events update the data model** - Events are processed by reducers that update the global state store
3. **Views react to state changes** - All views (canvases, panels, editors) subscribe to state changes and re-render accordingly

#### Event Flow Architecture

```
User Interaction
    ↓
Event Dispatch
    ↓
State Store Update (Global)
    ↓
State Change Notification
    ↓
View Subscriptions React
    ↓
Multiple Views Update Simultaneously
```

### 3. WebSocket-Based Real-Time Synchronization

For multi-user collaboration, Penpot uses a **Redis pub/sub mechanism** combined with WebSockets:

#### Backend Architecture

```
Backend creates a subscription in Redis with a topic that has the id of the file.
Backend sends a notification to this topic when a user sends any change to the file.
Notification is received by all subscribers and sent to the user via the websocket.
```

**Configuration:**

- Redis/Valkey is used as the pub/sub broker
- Each file has its own topic (identified by file ID)
- All connected clients subscribe to the file's topic
- Changes are broadcast to all subscribers via WebSocket

#### Frontend WebSocket Integration

The frontend establishes WebSocket connections to receive real-time updates:

```javascript
// WebSocket connection for notifications
const ws = await waitForNotificationsWebSocket();
await ws.mockOpen();

// Receiving presence and change notifications
await ws.mockMessage(JSON.stringify(presenceFixture));
```

### 4. Component Synchronization Logic

Penpot implements sophisticated component synchronization through the `sync-attrs` structure:

#### Attribute Synchronization Groups

The `sync-attrs` structure in `component.cljc` maps shape attributes to synchronization groups:

- **Main components** serve as the source of truth
- **Component copies** (instances) synchronize attributes from the main component
- **Touched state** prevents further synchronization for specific groups when manually overridden

This ensures that:

- Changes to main components propagate to all instances
- Manual overrides in instances are preserved (via "touched" state)
- No divergence occurs between main components and their instances

### 5. Preventing View Ownership of Data

#### Global State Store

Penpot maintains a **global state store** that all views read from:

```javascript
// Debug utilities reveal the global state structure
debug.dump_state();
debug.get_state(':workspace-data :pages 0');
debug.dump_objects();
```

**Key Characteristics:**

- Views **never own** the data - they only read from the global store
- Views **never mutate** data directly - they dispatch events
- All mutations go through the event system and update the global store
- Multiple views can read the same state simultaneously without conflicts

#### State Derivation Pattern

Views derive their display state from the global store:

```clojure
(defc accordion
  [{:keys [^boolean default-open title children] :as props]
  (let [open* (mf/use-state default-open)
        open? (deref open*)]
    [:details {:open open?}
      [:summary title]
      children]))
```

**Important:** Even local component state (like `open*`) is derived from props that come from the global store, maintaining the unidirectional data flow.

### 6. Canvas and Viewport Synchronization

#### Tile-Based Rendering System

Penpot uses a sophisticated tile-based rendering system that ensures multiple canvases stay synchronized:

**On Shape Interaction (create, move, edit):**

1. Compute tiles to re-render using shape's bounding box
2. Update TileHashMap
3. Invalidate affected tiles in cache
4. Re-render only affected tiles

**On Zoom/Pan:**

1. Identify new tiles entering visible_rect
2. Pull tiles from cache or render from scratch if not available
3. All viewports share the same tile cache and data model

**Key Benefits:**

- Multiple viewports can display the same file simultaneously
- All viewports derive from the same tile cache and data model
- Changes to the data model automatically invalidate relevant tiles
- Views cannot diverge because they all read from the same source

### 7. Event System and State Updates

#### Event Types

Penpot defines various events that track changes:

- `contentsave` - Triggered when document content is saved
- `shapechange` - Triggered when a shape's properties are modified
- `pagechange` - Triggered when the active page changes

#### Event Listener Management

```javascript
// Event listeners return IDs for proper cleanup
const id = penpot.on('pagechange', myListener);
penpot.off(id);
```

This ensures:

- Events are properly managed and cleaned up
- Multiple listeners can subscribe to the same events
- Views react to events but don't control the event flow

### 8. Undo/Redo System

The undo/redo system further enforces the single source of truth:

- **State mutations** are recorded in a history stack
- **Undo operations** revert to previous state versions
- **All views** automatically reflect the reverted state
- **No view-specific state** can persist after undo

This prevents views from maintaining divergent state that survives undo operations.

## Synchronization Guarantees

### 1. Consistency Guarantees

- **Single Source of Truth**: All views read from the same global state store
- **Atomic Updates**: State updates are atomic - either all views see the change or none do
- **Event Ordering**: Events are processed in order, ensuring consistent state transitions

### 2. Divergence Prevention

- **No Direct Mutations**: Views cannot directly mutate the data model
- **Event-Only Updates**: All changes must go through the event system
- **State Derivation**: Views derive their display from the global state, not local copies
- **Component Sync**: Component instances sync from main components, preventing divergence

### 3. Real-Time Synchronization

- **WebSocket Notifications**: All clients receive change notifications in real-time
- **Redis Pub/Sub**: Ensures reliable message delivery to all subscribers
- **Automatic Reconciliation**: When a client receives a change, it updates its local state store, which triggers view updates

## Implementation Patterns

### Pattern 1: Event Dispatch → State Update → View Reaction

```clojure
;; User interaction
[:button {:on-click handle-shape-move} "Move Shape"]

;; Event handler dispatches event
(defn handle-shape-move [event]
  (dispatch! :update-shape-position {:id shape-id :x new-x :y new-y}))

;; Reducer updates global state
(defn update-shape-position [state event]
  (update-in state [:workspace-data :pages 0 :objects shape-id]
             assoc :x (:x event) :y (:y event)))

;; Views automatically re-render when state changes
```

### Pattern 2: Component Synchronization

```clojure
;; Main component change
(update-main-component {:id main-id :fill-color new-color})

;; Sync logic propagates to instances
(sync-component-instances main-id {:fill-color new-color})

;; All instances update automatically
;; (unless "touched" for that attribute group)
```

### Pattern 3: Multi-Viewport Synchronization

```clojure
;; Multiple viewports read from same state
(def viewport-1 (get-state :viewport-1))
(def viewport-2 (get-state :viewport-2))

;; Both derive from same page data
(def page-data (get-state [:workspace-data :pages 0]))

;; Shape change updates both viewports
(update-shape shape-id changes)
;; → Invalidates tiles in both viewports
;; → Both viewports re-render affected tiles
```

## Benefits of This Architecture

1. **Consistency**: All views always show the same data because they read from the same source
2. **Predictability**: State changes follow a clear, traceable path through the event system
3. **Scalability**: Multiple editors, canvases, and panels can be added without architectural changes
4. **Collaboration**: Real-time synchronization works seamlessly because all clients follow the same pattern
5. **Maintainability**: Clear separation of concerns makes the codebase easier to understand and modify
6. **Debugging**: Global state can be inspected and events can be traced

## Conclusion

Penpot's architecture ensures that views never own or diverge from the data model through:

1. **Single global state store** that serves as the source of truth
2. **Event-driven updates** that prevent direct view mutations
3. **State derivation** where views read from the global store
4. **Real-time synchronization** via WebSocket and Redis pub/sub
5. **Component synchronization** that maintains consistency between main components and instances
6. **Tile-based rendering** that shares cache and data across multiple viewports

This architecture provides strong guarantees about consistency and prevents the common pitfalls of view-state divergence in complex multi-view applications.
