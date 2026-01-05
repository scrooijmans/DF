# Lapce Multi-Editor Synchronization Architecture

## Executive Summary

Lapce maintains synchronization across multiple editors, splits, and views through a **reactive, event-driven architecture** built on Floem's signal system and asynchronous communication via crossbeam channels. The architecture ensures consistency through a centralized state management system where all UI components react to shared state changes propagated through signals and RPC notifications.

**Key Architectural Components:**

1. **Floem Signal System** - Reactive state management for UI updates
2. **Crossbeam Channels** - Asynchronous message passing between components
3. **JSON-RPC Protocol** - Structured communication between app and proxy
4. **Centralized State** - Single source of truth for editor state
5. **Event-Driven Updates** - Changes propagate automatically to all views

---

## 1. Architecture Overview

### 1.1 Component Architecture

Lapce's architecture consists of four main modules that work together to maintain synchronization:

```
┌─────────────────────────────────────────────────────────────┐
│                    LAPCE-APP (Frontend UI)                   │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │ Editor View 1│  │ Editor View 2│  │ Editor View 3│      │
│  │  (Split 1)   │  │  (Split 2)   │  │  (Split 3)   │      │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘      │
│         │                 │                 │               │
│         └─────────────────┼─────────────────┘               │
│                           │                                 │
│              ┌────────────▼────────────┐                     │
│              │  Floem Signal System   │                     │
│              │  (Reactive State Mgmt) │                     │
│              └────────────┬────────────┘                     │
└──────────────────────────┼──────────────────────────────────┘
                           │ JSON-RPC over stdio
                           │ (crossbeam channels)
┌──────────────────────────▼──────────────────────────────────┐
│                  LAPCE-PROXY (Backend)                       │
│  ┌──────────────────────────────────────────────────────┐   │
│  │         CoreRpcHandler / ProxyRpcHandler             │   │
│  │  - File operations                                    │   │
│  │  - LSP interactions                                   │   │
│  │  - Command execution                                 │   │
│  └──────────────────────────────────────────────────────┘   │
│                                                              │
│  ┌──────────────────────────────────────────────────────┐   │
│  │              LAPCE-CORE (Editor Primitives)          │   │
│  │  - Rope-based text buffers                           │   │
│  │  - Document state management                          │   │
│  │  - Text manipulation utilities                       │   │
│  └──────────────────────────────────────────────────────┘   │
└──────────────────────────────────────────────────────────────┘
```

### 1.2 Communication Flow

All components communicate asynchronously using **crossbeam channels**:

```rust
// Bidirectional communication channels
let (writer_tx, writer_rx): (Sender<RpcMessage>, Receiver<RpcMessage>) = unbounded();
let (reader_tx, reader_rx): (Sender<RpcMessage>, Receiver<RpcMessage>) = unbounded();

// Messages flow through these channels
// - UI → Proxy: User actions, commands
// - Proxy → UI: File changes, LSP responses, terminal output
```

---

## 2. Reactive State Management with Floem Signals

### 2.1 Signal-Based Architecture

Lapce uses **Floem's signal system** for reactive state management. This ensures that when the underlying state changes, all views that depend on that state automatically update.

**Key Principles:**

- **Single Source of Truth**: Editor state is stored in a central location
- **Automatic Propagation**: Changes to state automatically trigger UI updates
- **Dependency Tracking**: Views declare their dependencies on state
- **Efficient Updates**: Only affected views re-render

### 2.2 How Signals Synchronize Multiple Views

When a file is edited in one editor view:

1. **User Action** → Text delta is applied to the buffer
2. **State Change** → Buffer state updates in lapce-core
3. **Signal Emission** → Floem signal notifies all subscribers
4. **View Updates** → All editor views observing that buffer automatically re-render
5. **Consistency** → All views show the same content

**Example Flow:**

```
Editor View 1 (Split 1) edits buffer
    ↓
Buffer.update(delta) in lapce-core
    ↓
Signal emits change notification
    ↓
┌─────────────────────────────────┐
│  All Editor Views subscribed    │
│  to this buffer's signal:       │
│  - Editor View 1 (Split 1) ✓    │
│  - Editor View 2 (Split 2) ✓    │
│  - Editor View 3 (Split 3) ✓    │
└─────────────────────────────────┘
    ↓
All views re-render with updated content
```

### 2.3 Signal Subscription Pattern

Views subscribe to signals for the data they display:

```rust
// Pseudocode representation of how views subscribe
struct EditorView {
    buffer_id: BufferId,
    signal: Signal<BufferState>,
}

impl EditorView {
    fn new(buffer_id: BufferId) -> Self {
        let signal = get_buffer_signal(buffer_id);
        Self { buffer_id, signal }
    }

    // View automatically re-renders when signal changes
    fn render(&self) -> View {
        self.signal.map(|buffer_state| {
            // Render based on current buffer state
            render_editor(buffer_state)
        })
    }
}
```

---

## 3. Event-Driven Communication via RPC

### 3.1 JSON-RPC Protocol

Lapce uses a **JSON-RPC protocol over stdio** for structured communication between the UI (lapce-app) and backend (lapce-proxy). This protocol handles:

- **Requests**: Commands from UI to proxy (file operations, LSP requests)
- **Responses**: Results from proxy to UI (file contents, LSP completions)
- **Notifications**: Asynchronous events (file changes, diagnostics, terminal output)

### 3.2 File Change Notifications

When a file changes (either through user edit or external modification), the system notifies all views:

```rust
use lapce_rpc::core::{CoreRpcHandler, CoreNotification};

// When file changes, notify all views
core_rpc.open_file_changed(
    PathBuf::from("/workspace/src/main.rs"),
    FileChanged::Change("new file content".to_string())
);
```

**Notification Flow:**

1. **File Change Detected** → Proxy detects change (user edit or external)
2. **RPC Notification Sent** → `CoreRpcHandler` sends notification
3. **Channel Broadcast** → Notification sent through crossbeam channel
4. **UI Receives Notification** → All editor views receive the notification
5. **Signal Update** → Buffer state signal updates
6. **View Synchronization** → All views showing that file update automatically

### 3.3 Command Execution and Synchronization

Commands (like split, close tab, save) are executed through a centralized command system:

```rust
use lapce_app::command::{LapceCommand, CommandKind, LapceWorkbenchCommand};

// Split command example
let split_vertical = LapceCommand {
    kind: CommandKind::Workbench(LapceWorkbenchCommand::SplitVertical),
    data: None,
};
```

**Command Execution Flow:**

1. **User Triggers Command** → Command created and dispatched
2. **Command Handler** → Central handler processes command
3. **State Mutation** → Editor state updated (new split created)
4. **Signal Emission** → State change signal emitted
5. **View Updates** → All views react to layout change
6. **Consistency** → All splits show updated layout

---

## 4. Consistency Enforcement Mechanisms

### 4.1 Single Source of Truth

**Buffer State Management:**

- Each file has **one buffer** in lapce-core
- Multiple views can display the same buffer
- All views read from the same buffer state
- Edits modify the buffer, not individual views

**Document State:**

- Document state (cursor position, selection, scroll position) is managed per-view
- Buffer content is shared across all views of the same file
- View-specific state (like cursor position) is independent per split

### 4.2 Atomic Updates

Text edits are applied as **atomic deltas** to the buffer:

```rust
use lapce_core::buffer::{Buffer, rope_text::RopeText};
use lapce_xi_rope::{Rope, RopeDelta};

// Atomic edit operation
let delta = RopeDelta::simple_edit(
    0..0,                           // Delete range
    Rope::from("// Header\n"),      // Insert text
    buffer.len()
);
buffer.update(&delta);  // Atomic update - all views see this change
```

**Atomic Update Benefits:**

- **Consistency**: All views see the same change at once
- **No Partial States**: Views never see intermediate states
- **Transaction Safety**: Edits are atomic operations

### 4.3 Event Ordering

Crossbeam channels provide **ordered message delivery**:

- Messages sent on the same channel are received in order
- This ensures that state changes are applied in the correct sequence
- Prevents race conditions in multi-view updates

### 4.4 View Filtering and Selective Updates

Views can filter notifications to only react to relevant changes:

```rust
// Pseudocode: View only updates when its buffer changes
impl EditorView {
    fn handle_notification(&self, notification: &Notification) {
        match notification {
            Notification::BufferChanged(buffer_id) if buffer_id == self.buffer_id => {
                // Only update if this view's buffer changed
                self.update();
            }
            _ => {
                // Ignore notifications for other buffers
            }
        }
    }
}
```

**Benefits:**

- **Performance**: Views only update when necessary
- **Efficiency**: Unrelated changes don't trigger unnecessary re-renders
- **Scalability**: System scales well with many views

---

## 5. Split and View Management

### 5.1 Split Creation and Synchronization

When a split is created:

1. **Command Triggered** → `SplitVertical` or `SplitHorizontal` command
2. **State Update** → Layout state updated with new split
3. **Signal Emission** → Layout change signal emitted
4. **View Creation** → New editor view created for the split
5. **Buffer Assignment** → View can show same or different buffer
6. **Synchronization** → If same buffer, both views subscribe to same signal

### 5.2 Multiple Views of Same File

When multiple splits show the same file:

```
┌─────────────────┐  ┌─────────────────┐
│  Editor View 1  │  │  Editor View 2  │
│  (Split 1)      │  │  (Split 2)      │
│                 │  │                 │
│  Buffer: main.rs│  │  Buffer: main.rs│
│  Cursor: line 5 │  │  Cursor: line 20│
└─────────────────┘  └─────────────────┘
         │                    │
         └──────────┬─────────┘
                    │
         ┌──────────▼──────────┐
         │  Shared Buffer      │
         │  (Single Source)    │
         └────────────────────┘
```

**Synchronization:**

- Both views subscribe to the **same buffer signal**
- When buffer changes, **both views update**
- Cursor positions are **independent per view**
- Scroll positions are **independent per view**

### 5.3 View-Specific State

Each view maintains its own state for:

- **Cursor Position**: Independent per view
- **Selection**: Independent per view
- **Scroll Position**: Independent per view
- **Viewport**: Independent per view

Shared state (from buffer):

- **File Content**: Shared across all views
- **Syntax Highlighting**: Shared (computed from buffer)
- **LSP Diagnostics**: Shared (per file)

---

## 6. Asynchronous Communication Patterns

### 6.1 Crossbeam Channels

Lapce uses **crossbeam channels** for lock-free, asynchronous communication:

```rust
use crossbeam_channel::{unbounded, Sender, Receiver};

// Create bidirectional channels
let (tx, rx): (Sender<Message>, Receiver<Message>) = unbounded();

// Send message (non-blocking)
tx.send(message).unwrap();

// Receive message (blocking until message available)
let message = rx.recv().unwrap();
```

**Benefits:**

- **Non-blocking**: UI remains responsive
- **Thread-safe**: Safe for concurrent access
- **Ordered**: Messages received in order
- **Efficient**: Lock-free implementation

### 6.2 RPC Request/Response Pattern

For operations requiring responses (like LSP requests):

```rust
use lapce_rpc::proxy::{ProxyRpcHandler, ProxyRequest, ProxyResponse};

// Async request with callback
proxy_rpc.request_async(
    ProxyRequest::GetDefinition {
        request_id: 101,
        path: PathBuf::from("/workspace/src/lib.rs"),
        position: Position { line: 50, character: 20 },
    },
    |result| match result {
        Ok(ProxyResponse::GetDefinitionResponse { definitions }) => {
            // Update UI with results
            update_views_with_definitions(definitions);
        }
        Err(e) => eprintln!("Failed: {}", e.message),
    }
);
```

**Pattern:**

1. **Request Sent** → UI sends request through channel
2. **Processing** → Proxy processes request (non-blocking)
3. **Response Received** → Response arrives through channel
4. **Signal Update** → State updated, signals emitted
5. **View Update** → Views react to state change

---

## 7. Consistency Guarantees

### 7.1 Eventual Consistency

Lapce ensures **eventual consistency** across all views:

- **Immediate Updates**: Local edits update immediately
- **Propagated Updates**: Changes propagate to all views via signals
- **External Changes**: File system changes detected and propagated
- **LSP Updates**: Language server responses update all relevant views

### 7.2 Conflict Resolution

When conflicts occur (e.g., external file change while editing):

1. **Change Detection** → System detects external change
2. **Notification** → All views notified of external change
3. **User Prompt** → User prompted to reload or keep changes
4. **State Resolution** → User choice determines final state
5. **Synchronization** → All views updated to resolved state

### 7.3 State Validation

Before applying changes, the system validates:

- **Buffer Integrity**: Buffer state is valid before update
- **View Consistency**: All views can access the buffer
- **Signal Validity**: Signals are valid before emission
- **RPC Integrity**: RPC messages are well-formed

---

## 8. Performance Optimizations

### 8.1 Selective Re-rendering

Views only re-render when their subscribed signals change:

- **Signal Dependency Tracking**: Floem tracks which views depend on which signals
- **Minimal Updates**: Only affected views re-render
- **Batched Updates**: Multiple changes can be batched together

### 8.2 Lazy Loading

Large files and views are loaded lazily:

- **Viewport Rendering**: Only visible portions rendered
- **Incremental Loading**: Content loaded as needed
- **Virtual Scrolling**: Efficient scrolling for large files

### 8.3 Efficient Data Structures

Lapce uses efficient data structures:

- **Rope Data Structure**: Efficient text manipulation (from Xi-Editor)
- **Delta-Based Updates**: Only changes transmitted, not full content
- **Immutable State**: State updates create new state, enabling efficient comparisons

---

## 9. Comparison with Other Editors

### 9.1 Similarities to VSCode

| Aspect                 | VSCode                | Lapce              |
| ---------------------- | --------------------- | ------------------ |
| **Remote Development** | Remote extension host | Proxy architecture |
| **LSP Support**        | Built-in              | Built-in           |
| **Multi-View**         | Split views           | Split views        |
| **State Management**   | Extension host state  | Floem signals      |

### 9.2 Differences

| Aspect           | VSCode                | Lapce                  |
| ---------------- | --------------------- | ---------------------- |
| **Language**     | TypeScript/Electron   | Rust                   |
| **UI Framework** | Web technologies      | Floem (native)         |
| **State System** | Event emitter pattern | Signal-based reactive  |
| **Performance**  | Web-based rendering   | GPU-accelerated (wgpu) |

---

## 10. Key Takeaways

### 10.1 Architectural Principles

1. **Reactive State Management**: Floem signals provide automatic UI updates
2. **Asynchronous Communication**: Crossbeam channels enable non-blocking operations
3. **Single Source of Truth**: One buffer per file, shared across views
4. **Event-Driven Updates**: Changes propagate through signals and notifications
5. **Type-Safe Communication**: JSON-RPC provides structured, validated communication

### 10.2 Synchronization Mechanisms

1. **Signal Subscriptions**: Views subscribe to buffer state signals
2. **RPC Notifications**: File changes broadcast via RPC
3. **Atomic Updates**: Text edits applied atomically
4. **Ordered Messages**: Channel ordering ensures correct sequence
5. **Selective Updates**: Views only update when relevant

### 10.3 Consistency Enforcement

1. **Centralized State**: Single buffer per file
2. **Atomic Operations**: Edits are atomic
3. **Event Ordering**: Messages processed in order
4. **State Validation**: Changes validated before application
5. **Conflict Resolution**: User-guided resolution for conflicts

---

## 11. Implementation Details

### 11.1 Signal System Integration

The Floem signal system integrates with Lapce's architecture:

```rust
// Pseudocode: How signals work in Lapce
struct BufferState {
    content: Rope,
    version: u64,
}

// Signal for buffer state
let buffer_signal: Signal<BufferState> = create_signal(initial_state);

// Views subscribe to signal
view1.subscribe(buffer_signal.clone());
view2.subscribe(buffer_signal.clone());

// When buffer updates
buffer_signal.set(new_state);  // All subscribed views update automatically
```

### 11.2 RPC Handler Pattern

RPC handlers manage communication:

```rust
// CoreRpcHandler manages UI → Proxy communication
struct CoreRpcHandler {
    tx: Sender<CoreRequest>,
    rx: Receiver<CoreNotification>,
}

// ProxyRpcHandler manages Proxy → UI communication
struct ProxyRpcHandler {
    tx: Sender<ProxyRequest>,
    rx: Receiver<ProxyResponse>,
}
```

### 11.3 Buffer Management

Buffers are managed centrally:

```rust
// Central buffer store
struct BufferStore {
    buffers: HashMap<PathBuf, Buffer>,
    signals: HashMap<PathBuf, Signal<BufferState>>,
}

impl BufferStore {
    fn get_buffer(&self, path: &Path) -> &Buffer {
        // Returns shared buffer reference
    }

    fn update_buffer(&mut self, path: &Path, delta: RopeDelta) {
        // Updates buffer and emits signal
        let buffer = self.buffers.get_mut(path).unwrap();
        buffer.update(&delta);
        self.signals[path].set(buffer.state());
    }
}
```

---

## 12. Conclusion

Lapce's multi-editor synchronization architecture demonstrates a sophisticated approach to maintaining consistency across multiple views through:

1. **Reactive State Management**: Floem signals provide automatic, efficient updates
2. **Asynchronous Communication**: Crossbeam channels enable responsive, non-blocking operations
3. **Centralized State**: Single source of truth ensures consistency
4. **Event-Driven Design**: Changes propagate automatically through the system
5. **Performance Optimization**: Selective updates and efficient data structures ensure responsiveness

This architecture ensures that all editor views, splits, and components remain synchronized while maintaining high performance and responsiveness, even with large files and many simultaneous views.
