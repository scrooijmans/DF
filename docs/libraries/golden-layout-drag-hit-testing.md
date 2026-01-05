# GoldenLayout: Drag Intent, Hit Testing, and Drop Zones

This document explains how GoldenLayout tracks drag intent, performs hit testing, and manages drop zones to differentiate between tab reordering, pane splitting, and window creation.

## Overview

GoldenLayout uses a sophisticated system of drag listeners, segment-based hit testing, and area-based drop zones to determine user intent during drag operations. The system differentiates between three main operations:

1. **Tab Reordering**: Moving tabs within the same stack
2. **Pane Splitting**: Creating new panes by dropping on edges
3. **Window Creation**: Creating popout windows

## Drag Intent Tracking

### Drag Start Event

Drag intent is initiated through the `Tab.DragStartEvent`:

```typescript
export namespace Tab {
    export type DragStartEvent = (
        x: number, 
        y: number, 
        dragListener: DragListener, 
        componentItem: ComponentItem
    ) => void;
}
```

Key aspects:
- **Initial Position**: The `x` and `y` coordinates capture where the drag started
- **DragListener**: The `DragListener` instance manages the entire drag operation lifecycle
- **ComponentItem**: The component being dragged is tracked throughout the operation

### ComponentItem Drag Methods

The `ComponentItem` class provides drag-related methods:

```typescript
class ComponentItem extends ContentItem {
    // @internal
    drag(): void;
    enterDragMode(width: number, height: number): void;
    exitDragMode(): void;
}
```

- `enterDragMode()`: Transitions the component into drag mode, setting up visual feedback
- `exitDragMode()`: Cleans up after drag completion
- The dragged component uses a special z-index (`defaultComponentDragZIndex = "32"`) to appear above other elements

## Hit Testing: Segment-Based Detection

### Stack Segments

GoldenLayout uses a **segment-based hit testing system** defined by the `Stack.Segment` enum:

```typescript
export namespace Stack {
    export const enum Segment {
        Body = "body",
        Bottom = "bottom",
        Header = "header",
        Left = "left",
        Right = "right",
        Top = "top"
    }
}
```

Each segment represents a different drop zone area:
- **Header**: The tab bar area - used for tab reordering
- **Body**: The main content area - used for replacing/swapping components
- **Left/Right/Top/Bottom**: Edge areas - used for pane splitting

### Content Area Dimensions

Each segment has associated drop zone dimensions:

```typescript
export namespace Stack {
    export interface ContentAreaDimension {
        highlightArea: AreaLinkedRect;
        hoverArea: AreaLinkedRect;
    }
    
    export type ContentAreaDimensions = {
        [segment: string]: ContentAreaDimension;
    };
}
```

- **hoverArea**: The larger area that triggers hover feedback
- **highlightArea**: The smaller area that shows the actual drop target
- Both use `AreaLinkedRect` (defined by x1, y1, x2, y2 coordinates)

### AreaLinkedRect

The drop zones are defined as rectangles:

```typescript
export interface AreaLinkedRect {
    x1: number;
    x2: number;
    y1: number;
    y2: number;
}
```

This allows precise hit testing by checking if the current mouse position falls within these rectangular bounds.

## Drop Zone Differentiation

### 1. Tab Reordering

**Detection**: When the drag cursor is over the **Header** segment

**Behavior**:
- The dragged tab can be reordered within the same stack
- Visual feedback shows insertion points between tabs
- The `reorderEnabled` property on `Tab` controls whether reordering is allowed

```typescript
class Tab {
    get reorderEnabled(): boolean;
    set reorderEnabled(value: boolean);
}
```

### 2. Pane Splitting

**Detection**: When the drag cursor is over **Left**, **Right**, **Top**, or **Bottom** segments

**Behavior**:
- Creates a new pane by splitting the existing pane
- The direction (left/right/top/bottom) determines the split orientation
- The `highlightArea` shows exactly where the split will occur
- Results in a new row or column container with the dragged component

### 3. Window Creation (Popout)

**Detection**: Special handling when dragging outside the layout container or to specific areas

**Behavior**:
- Creates a `BrowserPopout` instance
- Opens a new browser window with the component
- Managed through `ResolvedPopoutLayoutConfig`:

```typescript
export interface ResolvedPopoutLayoutConfig extends ResolvedLayoutConfig {
    readonly indexInParent: number | null;
    readonly parentId: string | null;
    readonly window: ResolvedPopoutLayoutConfig.Window;
}
```

**Settings Control**:
```typescript
export namespace ResolvedLayoutConfig {
    export interface Settings {
        readonly popoutWholeStack: boolean;  // Whether to popout entire stack or just component
        readonly popInOnClose: boolean;       // Whether to pop back in when window closes
        readonly blockedPopoutsThrowError: boolean;
        readonly closePopoutsOnUnload: boolean;
    }
}
```

## Hit Testing Flow

The hit testing process works as follows:

1. **Mouse Move During Drag**: The `DragListener` tracks mouse position
2. **Element Under Cursor**: Determines which `ContentItem` is under the cursor
3. **Segment Calculation**: Calculates which segment of that item the cursor is in
4. **Area Check**: Checks if cursor is within the `hoverArea` or `highlightArea` of that segment
5. **Visual Feedback**: Updates highlight/hover indicators based on the detected segment
6. **Drop Action**: On drop, performs the appropriate action based on the segment

## Visual Feedback System

### Z-Index Management

Different states use different z-index values:

```typescript
export namespace StyleConstants {
    const defaultComponentBaseZIndex = "auto";
    const defaultComponentDragZIndex = "32";              // Dragged components
    const defaultComponentStackMaximisedZIndex = "41";     // Maximized stacks
}
```

### Highlight Areas

- **hoverArea**: Larger, less precise area for initial hover detection
- **highlightArea**: Smaller, precise area showing exact drop location
- Both are updated in real-time as the mouse moves during drag

## Implementation Details

### ContentItem Hierarchy

All layout items extend `ContentItem`:

```typescript
export abstract class ContentItem extends EventEmitter {
    addChild(contentItem: ContentItem, index?: number | null, suspendResize?: boolean): number;
    // ... other methods
}
```

This allows any item in the hierarchy to participate in drag and drop operations.

### Location Selectors

For programmatic component placement, GoldenLayout uses `LocationSelector`:

```typescript
export interface LocationSelector {
    typeId: LocationSelector.TypeId;
    index?: number;
}
```

This provides a way to specify drop locations algorithmically, complementing the visual drag-and-drop system.

## Summary

GoldenLayout's drag system uses:

1. **Drag Intent**: Tracked through `DragListener` and initial coordinates
2. **Hit Testing**: Segment-based system (header, body, edges) to determine drop target
3. **Drop Zones**: Rectangular areas (`AreaLinkedRect`) with hover and highlight zones
4. **Differentiation**: 
   - **Header segment** → Tab reordering
   - **Edge segments** (left/right/top/bottom) → Pane splitting
   - **Special handling** → Window creation (popout)

The system provides real-time visual feedback through highlight areas and uses precise coordinate-based hit testing to determine user intent, enabling smooth and intuitive drag-and-drop operations.

