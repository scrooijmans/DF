# Lumino Architecture: Widget System, Message Loop, and Layout Management

This document explains how Lumino implements its widget system, message loop, and layout management, and how these abstractions support complex multi-panel applications.

## Overview

Lumino is a powerful, flexible, and extensible framework for building interactive scientific applications, particularly in the Jupyter ecosystem. Its architecture is built on three core abstractions:

1. **Widget System**: A hierarchical component model with lifecycle management
2. **Message Loop**: An asynchronous messaging system for inter-component communication
3. **Layout Management**: Flexible layout managers for arranging widgets in complex panel configurations

---

## 1. Widget System

### Base Widget Class

All visual elements in Lumino inherit from the base `Widget` class, which provides:

- **DOM Attachment**: Methods to attach/detach widgets from the DOM
- **Lifecycle Management**: Messages for lifecycle events
- **State Management**: Visibility, activation, and parent-child relationships
- **Message Handling**: Integration with the message loop system

### Widget Lifecycle Messages

The widget system uses a comprehensive set of message types to manage widget lifecycle:

```typescript
Widget.Msg:
  - BeforeAttach: Sent before a widget is attached to the DOM
  - AfterAttach: Sent after a widget is attached to the DOM
  - BeforeDetach: Sent before a widget is detached from the DOM
  - AfterDetach: Sent after a widget is detached from the DOM
  - BeforeShow: Sent before a widget is shown
  - AfterShow: Sent after a widget is shown
  - BeforeHide: Sent before a widget is hidden
  - AfterHide: Sent after a widget is hidden
  - ActivateRequest: Message to request widget activation
  - CloseRequest: Message to request closing a widget
  - FitRequest: Message to request a fit operation (resize to content)
  - UpdateRequest: Message to request a widget update
  - ParentChanged: Message sent when a widget's parent changes
```

### Widget Hierarchy

Widgets form a parent-child hierarchy:
- **Panel**: Base container widget that can hold other widgets
- **LayoutItem**: Represents an item within a layout
- **Title**: Represents the title/header of a widget (used in docked panels)

### Key Widget Types

- **Panel**: Basic container widget
- **BoxPanel**: Panel using BoxLayout (horizontal/vertical arrangement)
- **SplitPanel**: Panel with adjustable splitters
- **DockPanel**: Panel supporting docking and tabbing (key for multi-panel apps)
- **TabPanel**: Panel displaying content in tabs
- **StackedPanel**: Panel displaying one widget at a time from a stack
- **AccordionPanel**: Panel with accordion-style arrangement

---

## 2. Message Loop

### Core Message Loop API

Lumino's `MessageLoop` provides a centralized messaging system for asynchronous inter-component communication:

```typescript
MessageLoop:
  // Message Dispatch
  sendMessage(target: any, message: Message): void
    - Sends a message directly to a target object (synchronous)
  
  postMessage(target: any, message: Message): void
    - Posts a message to be processed asynchronously
  
  flush(): void
    - Processes all pending messages in the loop
  
  // Message Hooks (Interception)
  installMessageHook(hook: (msg: Message) => boolean): void
    - Installs a hook to intercept messages before delivery
    - Returns true to consume/prevent message delivery
  
  removeMessageHook(hook: (msg: Message) => boolean): void
    - Removes a previously installed message hook
  
  // Exception Handling
  setExceptionHandler(handler: ExceptionHandler | null): void
    - Sets exception handler for message processing errors
  
  getExceptionHandler(): ExceptionHandler | null
    - Gets current exception handler
  
  clearData(): void
    - Clears all data from the message loop
```

### Message Types

- **Message**: Base class for all messages
- **ConflatableMessage**: Messages that can be conflated (merged) when multiple are queued
- **ChildMessage**: Base class for messages sent between parent and child widgets
- **ResizeMessage**: Message for widget resize events

### Message Handling Interfaces

- **IMessageHandler**: Interface for objects that can handle messages
- **IMessageHook**: Interface for message hooks that can intercept messages

### How Messages Flow

1. **Sending**: `sendMessage()` delivers immediately (synchronous)
2. **Posting**: `postMessage()` queues for async processing
3. **Hooks**: Message hooks can intercept and optionally consume messages
4. **Processing**: Messages are processed through the target's message handler
5. **Exceptions**: Exception handlers catch errors during message processing

### Integration with Widgets

Widgets automatically integrate with the message loop:
- Lifecycle events (attach, detach, show, hide) are sent as messages
- Widgets can send messages to other widgets
- Parent-child communication uses the message system
- Layout updates trigger messages to affected widgets

---

## 3. Layout Management

### Layout Hierarchy

All layouts inherit from the base `Layout` class:

```
Layout (base class)
├── AccordionLayout
├── BoxLayout
├── DockLayout (key for multi-panel apps)
├── GridLayout
├── PanelLayout
├── SingletonLayout
├── SplitLayout
└── StackedLayout
```

### LayoutItem

Each widget in a layout is wrapped in a `LayoutItem`, which provides:
- Widget reference
- Size constraints and hints
- Alignment and positioning information
- Stretch factors for flexible sizing

### Key Layout Types

#### BoxLayout
Arranges widgets in a horizontal or vertical box:
- **Direction**: 'left-to-right', 'right-to-left', 'top-to-bottom', 'bottom-to-top'
- **Alignment**: Controls widget alignment within the box
- **Stretch**: Controls how widgets expand to fill space
- **Size Basis**: Base size for widgets before stretching

```typescript
BoxLayout:
  getStretch(widget: Widget): number
  setStretch(widget: Widget, stretch: number): void
  getSizeBasis(widget: Widget): number
  setSizeBasis(widget: Widget, size: number): void
```

#### SplitLayout
Arranges widgets with adjustable splitters:
- **Orientation**: Horizontal or vertical splits
- **Alignment**: Alignment of splitter handles
- **Stretch**: Controls relative sizes of split sections

```typescript
SplitLayout:
  getStretch(widget: Widget): number
  setStretch(widget: Widget, value: number): void
```

#### DockLayout
The most powerful layout for multi-panel applications:
- **Docking**: Widgets can be docked to edges (top, bottom, left, right)
- **Tab Areas**: Multiple widgets can be tabbed together
- **Split Areas**: Complex nested split configurations
- **Insert Modes**: Control how widgets are inserted into the layout

```typescript
DockLayout:
  IAddOptions: Options when adding widgets
  ILayoutConfig: Overall layout configuration
  ISplitAreaConfig: Configuration for split areas
  ITabAreaConfig: Configuration for tab areas
  InsertMode: Enum for insertion modes
```

#### StackedLayout
Stacks widgets, showing one at a time:
- Useful for tab-like behavior without tabs
- Widgets are layered, only the top one is visible

#### GridLayout
Arranges widgets in a grid:
- Row and column management
- Cell spanning support
- Alignment per cell

### Layout and Panel Integration

Each panel type uses a specific layout:
- **BoxPanel** → BoxLayout
- **SplitPanel** → SplitLayout
- **DockPanel** → DockLayout
- **StackedPanel** → StackedLayout
- **TabPanel** → Uses tabs with underlying layout
- **AccordionPanel** → AccordionLayout

---

## 4. Supporting Complex Multi-Panel Applications

### DockPanel: The Foundation

`DockPanel` with `DockLayout` is the primary mechanism for complex multi-panel applications:

1. **Docking System**: Widgets can be docked to any edge of the panel
2. **Tabbed Areas**: Multiple widgets can share the same space via tabs
3. **Nested Splits**: Complex arrangements with nested split panes
4. **Dynamic Reconfiguration**: Layout can be modified at runtime
5. **Persistence**: Layout configurations can be saved/restored

### Message-Driven Updates

The message loop ensures layout changes propagate correctly:
- When a widget is added/removed, messages notify affected widgets
- Resize events trigger layout recalculation
- Parent-child relationships are maintained through messages
- Update requests coordinate layout passes

### Layout Calculation Flow

1. **Request**: Widget requests layout update (via `UpdateRequest` message)
2. **Layout Pass**: Layout manager calculates positions and sizes
3. **Messages**: Layout items receive size/position information
4. **DOM Update**: Widgets update their DOM elements
5. **Cascade**: Child layouts recalculate if needed

### Multi-Panel Architecture Example

A typical multi-panel application structure:

```
DockPanel (root)
├── DockLayout
    ├── TabArea (left side)
    │   ├── Widget A (tab 1)
    │   └── Widget B (tab 2)
    ├── SplitArea (center, vertical)
    │   ├── Widget C (top)
    │   └── Widget D (bottom)
    └── TabArea (right side)
        └── Widget E
```

### Key Abstractions for Multi-Panel Apps

1. **Separation of Concerns**:
   - Widgets handle their own rendering
   - Layouts handle positioning
   - Message loop handles communication

2. **Flexibility**:
   - Multiple layout types can be combined
   - Widgets can be moved between layouts
   - Layouts can be nested arbitrarily

3. **Performance**:
   - Messages are queued and batched
   - Layout calculations are deferred until needed
   - Only visible widgets are updated

4. **Extensibility**:
   - Custom layouts can extend base Layout class
   - Message hooks allow interception/customization
   - Renderers can be customized per layout type

---

## 5. Integration Points

### Widget ↔ Message Loop
- Widgets send/receive messages for lifecycle and state changes
- Message handlers are automatically invoked
- Exception handling prevents one widget's errors from crashing the app

### Widget ↔ Layout
- Widgets are wrapped in LayoutItems
- Layouts query widget size preferences
- Layouts assign positions and sizes to widgets
- Widgets update their DOM based on layout assignments

### Layout ↔ Message Loop
- Layout changes trigger messages to affected widgets
- Update requests coordinate layout passes
- Resize messages trigger layout recalculation

---

## Summary

Lumino's architecture provides:

1. **Widget System**: Hierarchical component model with rich lifecycle management
2. **Message Loop**: Asynchronous, hookable messaging system for decoupled communication
3. **Layout Management**: Flexible, composable layout system supporting complex arrangements

Together, these abstractions enable:
- **Complex Multi-Panel Applications**: DockPanel with nested splits and tabs
- **Dynamic UI Reconfiguration**: Runtime layout changes
- **Decoupled Components**: Message-based communication
- **Performance**: Efficient batching and deferred updates
- **Extensibility**: Custom widgets, layouts, and message handlers

This architecture is particularly well-suited for applications like JupyterLab, which require complex, resizable, dockable panel arrangements that can be dynamically reconfigured by users.


