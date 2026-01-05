# VS Code Drag Preview and Drop Target Rendering

## Overview

This document explains how VS Code renders drag previews and drop targets during panel or editor movement, including how it calculates valid drop zones and avoids layout corruption during interaction.

## 1. Drag Preview Rendering

### Preview Creation

VS Code uses the HTML5 Drag and Drop API with custom preview handling. When a drag operation starts:

1. **Element Capture**: The dragged element's visual state is captured
2. **Custom Preview Creation**: A custom drag preview (ghost image) is created
3. **Metadata Storage**: Editor metadata is stored in `dataTransfer`

### Preview Setup

The `setupDragAndDrop` utility handles event setup:

```typescript
export function setupDragAndDrop(
  element: HTMLElement,
  options: {
    onDragEnter: (e: IDragAndDropData) => void;
    onDragLeave: (e: IDragAndDropData) => void;
    onDragOver: (e: IDragAndDropData) => void;
    onDrop: (e: IDragAndDropData) => void;
  }
): IDisposable;
```

### Preview Creation Process

1. **Clone Element**: The tab/panel element is cloned
2. **Apply Styling**: Opacity and transform are applied for ghost effect
3. **Set Drag Image**: Uses `dataTransfer.setDragImage()` or custom overlay
4. **Position Preview**: Positions preview relative to cursor

### Visual Feedback During Drag

The `GlobalMouseMoveMonitor` tracks mouse position for real-time updates:

```typescript
export class GlobalMouseMoveMonitor extends Disposable {
  public start(element: HTMLElement): void {
    this._mouseMoveListener = addDisposableListener(
      element,
      EventType.MOUSE_MOVE,
      (e) =>
        this._callback({
          clientX: e.clientX,
          clientY: e.clientY,
          // ... other mouse event data
        })
    );
  }
}
```

This enables:

- Real-time preview positioning
- Drop zone calculation
- Visual indicator updates

## 2. Drop Target Calculation and Validation

### Hit Testing Process

During `dragover`, VS Code:

1. **Gets Mouse Coordinates**: Extracts `clientX` and `clientY` from drag event
2. **Performs Hit Testing**: Tests against potential drop targets
3. **Determines Drop Zone Type**: Identifies tab bar, group area, splitter, etc.
4. **Validates Operation**: Checks if the operation is allowed

### Drag Over Implementation

```javascript
function handleInnerDragEvent(e) {
  // Critical: preventDefault() must be called for drop to fire
  e.preventDefault();
  if (!e.dataTransfer) {
    return;
  }
  // Send drag information including modifier keys
  hostMessaging.postMessage('drag', {
    shiftKey: e.shiftKey,
  });
}
```

### Drop Zone Types

VS Code identifies several drop zone types:

1. **Tab Bar Zones**:

   - Before/after a tab (insertion point)
   - Into an existing group (merge)

2. **Editor Group Zones**:

   - Center of a group (add to group)
   - Edge of a group (split/create new group)
   - Splitter between groups (reposition)

3. **Panel Zones**:
   - Panel area (move panel)
   - Panel edge (resize/reposition)

### Zone Calculation Algorithm

The calculation process:

1. **Mouse Position**: Extract `clientX`, `clientY` from event
2. **Element Bounds**: Use `getBoundingClientRect()` for each potential target
3. **Proximity Check**: Calculate distance to edges/centers
4. **Threshold Zones**: Use configurable hit areas (e.g., 10px from edges)
5. **Priority**: Closest valid zone wins

Example logic:

```typescript
// Pseudo-code for drop zone calculation
function calculateDropZone(
  mouseX: number,
  mouseY: number
): DropZone | null {
  const groups = this.getEditorGroups();

  for (const group of groups) {
    const bounds = group.getBoundingClientRect();

    // Check if mouse is over group
    if (
      mouseX >= bounds.left &&
      mouseX <= bounds.right &&
      mouseY >= bounds.top &&
      mouseY <= bounds.bottom
    ) {
      // Check proximity to edges (split zones)
      const edgeThreshold = 10;
      if (mouseX - bounds.left < edgeThreshold) {
        return {
          type: 'split-left',
          group,
          position: bounds.left,
        };
      }
      if (bounds.right - mouseX < edgeThreshold) {
        return {
          type: 'split-right',
          group,
          position: bounds.right,
        };
      }

      // Check tab bar area
      const tabBarBounds =
        group.tabBar.getBoundingClientRect();
      if (
        mouseY >= tabBarBounds.top &&
        mouseY <= tabBarBounds.bottom
      ) {
        return {
          type: 'tab-bar',
          group,
          insertIndex: calculateTabIndex(mouseX),
        };
      }

      // Default: center of group
      return { type: 'group-center', group };
    }
  }

  return null;
}
```

## 3. Visual Drop Target Indicators

### Indicator Rendering

During drag operations, VS Code shows visual feedback:

1. **Drop Zone Highlighting**:

   - Adds CSS classes to valid targets
   - Shows insertion lines/rectangles
   - Highlights group borders

2. **Preview Indicators**:

   - Split preview lines
   - Tab insertion markers
   - Group merge highlights

3. **Cursor Feedback**:
   - Changes cursor based on drop zone
   - Shows move/copy indicators

### Event-Driven Updates

The drag event handlers update indicators:

```typescript
// Drag enter: show indicator
onDragEnter: (e: IDragAndDropData) => {
  const dropZone = this.calculateDropZone(e.dataTransfer);
  if (dropZone) {
    this.showDropIndicator(dropZone);
  }
};

// Drag over: update indicator position
onDragOver: (e: IDragAndDropData) => {
  const newZone = this.calculateDropZone(e.dataTransfer);
  if (newZone !== this.currentDropZone) {
    this.hideDropIndicator(this.currentDropZone);
    this.showDropIndicator(newZone);
    this.currentDropZone = newZone;
  }
};

// Drag leave: hide indicator
onDragLeave: (e: IDragAndDropData) => {
  this.hideDropIndicator(this.currentDropZone);
  this.currentDropZone = null;
};
```

## 4. Layout Corruption Prevention

### State Management

The `SplitView` uses state to prevent concurrent modifications:

```typescript
private doAddView(view: IView<TLayoutContext>, size: number | Sizing, index?: number): void {
  if (this.state !== State.Idle) {
    throw new Error('Cant modify splitview');
  }

  this.state = State.Busy;
  // ... perform modifications ...
}
```

### Prevention Strategies

1. **Read-Only During Drag**:

   - Layout tree is read-only while dragging
   - Only visual indicators change
   - No structural mutations

2. **Deferred Mutations**:

   - Changes are queued until drop
   - Single atomic operation on drop
   - Rollback on cancellation

3. **Validation Before Mutation**:

   ```typescript
   // Before applying drop
   if (this.state !== State.Idle) {
     // Reject drop or queue it
     return;
   }

   // Validate drop target
   if (!this.isValidDropTarget(dropZone)) {
     return;
   }

   // Apply changes atomically
   this.state = State.Busy;
   try {
     this.applyDrop(dropZone);
   } finally {
     this.state = State.Idle;
   }
   ```

4. **Event Coordination**:
   - `preventDefault()` on `dragover` to allow drop
   - No layout changes during `dragover`
   - Layout changes only on `drop`

### SplitView Safety

The `SplitView` enforces safety through:

1. **State Guards**: Operations only proceed when state is `Idle`
2. **Transactional Updates**: Changes are atomic
3. **Disposable Pattern**: Resources are properly cleaned up

```typescript
// DOM manipulation is coordinated with state
if (index === this.viewItems.length) {
  this.viewContainer.appendChild(container);
} else {
  this.viewContainer.insertBefore(
    container,
    this.viewContainer.children.item(index)
  );
}

// Disposables ensure cleanup
const containerDisposable = toDisposable(() =>
  this.viewContainer.removeChild(container)
);
```

4. **Error Handling**: Invalid operations throw errors rather than corrupting state

## 5. Interaction Flow Summary

The complete interaction flow:

1. **Drag Start**:

   - Create drag preview
   - Initialize drag state
   - Start mouse tracking

2. **Drag Over (Continuous)**:

   - Calculate drop zone from mouse position
   - Update visual indicators
   - Validate operation
   - Prevent default to allow drop

3. **Drop**:

   - Validate final drop target
   - Check layout state (`State.Idle`)
   - Apply mutations atomically
   - Update layout tree
   - Clean up indicators

4. **Error Handling**:
   - Reject invalid drops
   - Restore state on errors
   - Clean up on drag cancel

## Key Implementation Details

1. **Always call `preventDefault()`** in `dragover` handlers
2. **Use state management** (`State.Idle`/`State.Busy`) to prevent concurrent modifications
3. **Calculate drop zones** from mouse coordinates and element bounds
4. **Update visual indicators** during `dragover`, not during structural changes
5. **Apply layout mutations** only on `drop` in a single atomic operation

## Summary

VS Code implements drag preview and drop target rendering with:

1. **Custom Preview Creation**: Uses HTML5 Drag and Drop API with custom preview images
2. **Real-Time Feedback**: Uses `GlobalMouseMoveMonitor` for mouse tracking
3. **Hit Testing**: Calculates valid drop zones from mouse position and element bounds
4. **Visual Indicators**: Shows drop zones, insertion points, and split previews
5. **Layout Safety**: Prevents corruption through state management and atomic operations

The system ensures responsive visual feedback during drag operations while maintaining layout stability and preventing corruption.

