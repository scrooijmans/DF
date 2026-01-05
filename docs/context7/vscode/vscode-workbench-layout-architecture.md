# VS Code Workbench Layout Architecture

## Overview

This document explains in detail how VS Code models its workbench layout, including editors, editor groups, panels, and split views, and how these are dynamically created, resized, destroyed, and persisted.

## Core Architecture Overview

VS Code uses a layered architecture:
- **Base Layer**: Utilities and UI building blocks
- **Platform Layer**: Service injection and base services
- **Editor Layer**: Monaco Editor Core
- **Workbench Layer**: UI framework (Explorer, Status Bar, Menu Bar, editors)
- **Code Layer**: Entry point integrating all components

The workbench uses a service-oriented architecture with dependency injection. Layout components are managed through services like `IEditorService`, `IPartService`, and `IWorkbenchLayoutService`.

## 1. Editors and Editor Groups

**Editor Groups** are containers that hold one or more editors. They form the primary layout unit in the editor area.

**Key Components:**
- **Editor Service** (`IEditorService`): Manages opening, closing, and interacting with editors
- **Group Service**: Manages editor groups within the workbench
- **Editor Input**: Represents what an editor displays (file, untitled document, etc.)
- **Editor Pane**: The visual component that renders the editor

**Editor Registration:**
Registering a new editor type involves:
1. Registering an **Editor Input Factory** for resolving resources (e.g., by `glob` patterns or URI schemes)
2. Registering an **Editor Pane Factory** for the editor input type

The workbench editor service handles common states and events (display name, capabilities, content changes, dirty state) but not model resolution or rendering.

## 2. Split Views - The Layout Engine

**SplitView** is the component that manages resizable panes. It supports both vertical and horizontal orientations.

**Core SplitView API:**
```typescript
export declare class SplitView {
    constructor(container: HTMLElement, options?: SplitViewOptions);
    layout(size: number): void;
    add(view: ISplitViewView, size: number, index?: number): void;
    remove(index: number, size?: number): void;
    moveView(index: number, toIndex: number): void;
    getViews(): ISplitViewView[];
    resizeView(index: number, size: number): void;
    dispose(): void;
}
```

**View Interface:**
```typescript
export interface ISplitViewView {
    element: HTMLElement;
    minimumObserver: () => void;
    maximumObserver: () => void;
    onDidChange: Event;
    onDidScroll: Event;
}
```

**Adding Views:**
The `doAddView` method shows how views are added:

1. **State Management**: SplitView must be in `State.Idle` before modifications
2. **DOM Manipulation**: Creates a container element (`.split-view-view`) and inserts it at the specified index
3. **Size Calculation**: Supports multiple sizing strategies:
   - **Fixed size**: Direct number value
   - **Split sizing**: Divides an existing view's size in half
   - **Auto sizing**: Automatically distributes or splits based on current layout
   - **Invisible sizing**: Caches size when hidden
4. **Orientation Handling**: Creates either `VerticalViewItem` or `HorizontalViewItem` based on orientation
5. **Event Handling**: Sets up change listeners and disposal logic

**Sizing Strategies:**
- `'split'`: Divides an existing view's size in half
- `'distribute'`: Evenly distributes space among all views
- `'auto'`: Automatically chooses between split or distribute
- `'invisible'`: Stores cached size when view is hidden

## 3. Panels

**Panels** are workbench parts positioned at the bottom or side of the editor area (e.g., Terminal, Problems, Output).

**Panel Interface:**
```typescript
export declare abstract class Panel {
    protected readonly _id: string;
    protected readonly _name: string;
    
    abstract create(parent: HTMLElement): void;
    abstract layout(dimension: Dimension): void;
    abstract focus(): void;
    abstract dispose(): void;
}
```

**PanelView Component:**
```typescript
export declare class PanelView {
    constructor(container: HTMLElement, options?: PanelViewOptions);
    open(size: number): void;
    close(): void;
    toggle(): void;
    layout(size: number): void;
    dispose(): void;
}
```

Panels can be collapsed, expanded, and resized, and they integrate with SplitView for layout management.

## 4. Dynamic Creation, Resizing, and Destruction

**Creation:**
- Editor groups are created when splitting editors or opening files in new groups
- SplitView manages the creation through `add()` with size and index
- The system maintains state (`State.Idle` vs `State.Busy`) to prevent concurrent modifications

**Resizing:**
- Handled through `resizeView(index, size)` on SplitView
- Views emit `onDidChange` events when their size requirements change
- The layout recalculates and redistributes space as needed
- Supports minimum/maximum size constraints through observers

**Destruction:**
- Views are removed via `remove(index, size?)`
- Cleanup includes:
  - Removing DOM elements
  - Disposing event listeners
  - Cleaning up view items
  - Redistributing space to remaining views

**Layout Actions:**
VS Code provides actions for layout manipulation:
- `SplitEditorAction`: Splits the current editor
- `ToggleEditorLayoutAction`: Toggles between horizontal/vertical layouts
- `ToggleCenteredLayoutAction`: Toggles centered layout mode

## 5. Persistence and State Management

**State Service:**
VS Code uses `IStateService` for persistence:

```typescript
export interface IStateService {
    get<T>(key: string, defaultValue: T): T;
    set(key: string, value: any): void;
    remove(key: string): void;
}
```

**Backup Service:**
The `IBackupMainService` handles:
- Persisting editor content
- Saving workspace backups
- Restoring unsaved work

**Workbench Lifecycle:**
The restoration process is marked with performance marks:
- `code/willStartWorkbench` / `code/didStartWorkbench`: Workbench creation and restoration
- `code/willRestoreEditors` / `code/didRestoreEditors`: Editor restoration
- `code/willRestoreViewlet` / `code/didRestoreViewlet`: Viewlet restoration
- `code/willRestorePanel` / `code/didRestorePanel`: Panel restoration

**Part Service:**
The `IPartService` manages workbench parts (sidebar, editor, status bar) and their visibility/state:
```typescript
export interface IPartService {
    getActivePanel(): string;
    setEditorAreaVisible(visible: boolean): void;
    // ... other methods for managing workbench parts
}
```

**Storage:**
- Layout state (editor groups, sizes, positions) is serialized and stored
- Editor states (open files, cursor positions, selections) are preserved
- Panel states (visibility, size) are persisted
- The system restores this state on startup

## 6. Advanced Features

**Grid Layout Support:**
VS Code has explored grid-based editor layouts (beyond simple splits), allowing more complex arrangements.

**Flexible Panel/Sidebar Layout:**
- Panels can be moved between bottom and side positions
- Sidebars can be placed on left or right
- Views can be moved between panel and sidebar

**Custom Editors:**
The system supports custom editor types through:
- Editor input factories
- Editor pane factories
- Integration with the workbench editor service

## Summary

This architecture enables VS Code to maintain a flexible, persistent layout system that adapts to user needs while preserving state across sessions. The combination of SplitView for layout management, service-oriented architecture for coordination, and comprehensive state persistence creates a robust foundation for the workbench experience.


