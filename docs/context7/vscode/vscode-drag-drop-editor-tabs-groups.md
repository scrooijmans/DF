# VS Code Drag-and-Drop Implementation for Editor Tabs and Groups

## Overview

This document explains in detail how VS Code implements drag-and-drop of editor tabs and editor groups, including how drag start, hover, preview, and drop are handled, and how the layout tree is mutated safely when a drop occurs.

## Architecture Overview

VS Code uses a layered approach for drag-and-drop:

1. **Browser Drag-and-Drop Utilities**: Low-level HTML5 Drag and Drop API wrappers
2. **Abstract Drag-and-Drop Handlers**: High-level interfaces for drag operations
3. **Layout Tree Management**: SplitView component for managing the layout structure
4. **Editor Group Services**: State management and coordination

## 1. Drag Start Handling

### Browser Drag-and-Drop Setup

VS Code uses a utility function to set up drag-and-drop on elements:

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

### Drag Start Process

When a drag operation starts on an editor tab:

1. **Event Capture**: The tab element captures the drag event
2. **Metadata Storage**: Editor metadata (group ID, editor input) is stored in `dataTransfer`
3. **Visual Indicator**: A ghost image or visual indicator is created
4. **State Initialization**: Drag state is initialized

### Drag Handler Interface

VS Code uses an abstract handler pattern:

```typescript
export declare abstract class DragAndDropHandler {
  abstract handleDragStart(
    data: DragAndDropData,
    event: DragEvent
  ): void;
  abstract handleDragOver(
    data: DragAndDropData,
    event: DragEvent
  ): void;
  abstract handleDrop(
    data: DragAndDropData,
    event: DragEvent
  ): void;
}
```

## 2. Hover and Preview Handling

### Drag Over Event Processing

During drag operations, the `dragover` event is continuously fired. VS Code handles this by:

1. **Preventing Default**: Always calls `preventDefault()` to allow drops
2. **Drop Target Calculation**: Determines what element is under the cursor
3. **Visual Feedback**: Shows appropriate visual indicators
4. **Modifier Key Detection**: Checks for Shift, Ctrl, etc. to determine move vs. copy

Example implementation:

```javascript
function handleInnerDragEvent(e) {
  // Critical: preventDefault() must be called for drop to fire
  e.preventDefault();
  if (!e.dataTransfer) {
    return;
  }
  // Handle drag with shiftKey state
  hostMessaging.postMessage('drag', {
    shiftKey: e.shiftKey,
  });
}
```

### Preview Logic

The preview system:

1. **Calculates Drop Position**: Determines where the item would be dropped

   - Before/after a tab (insertion point)
   - Into an existing group (merge)
   - To create a new group (split)

2. **Shows Visual Indicators**:

   - Drop zone highlights
   - Insertion lines
   - Split preview indicators

3. **Updates Cursor**: Changes cursor based on valid drop targets

## 3. Layout Tree Structure

### SplitView Component

The layout uses a `SplitView` tree structure:

```typescript
export declare class SplitView {
  constructor(
    container: HTMLElement,
    options?: SplitViewOptions
  );
  layout(size: number): void;
  add(
    view: ISplitViewView,
    size: number,
    index?: number
  ): void;
  remove(index: number, size?: number): void;
  moveView(index: number, toIndex: number): void;
  getViews(): ISplitViewView[];
  resizeView(index: number, size: number): void;
  dispose(): void;
}
```

### Tree Structure

The layout tree represents:

- **Root**: Editor area container
- **Internal Nodes**: Split containers (vertical/horizontal)
- **Leaf Nodes**: Editor groups (containing tabs)

## 4. Safe Layout Tree Mutation on Drop

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

### Drop Handling Process

When a drop occurs, the layout is mutated safely through these steps:

#### 1. Validation

- Verify the drop target is valid
- Check if the operation is allowed (e.g., moving within the same group)

#### 2. Atomic Operations

- All layout changes are batched
- The tree is modified in a single transaction
- State is set to `Busy` during mutation

#### 3. Tree Mutation Steps

```typescript
// Example: Moving a tab to a new group
// 1. Remove editor from source group
sourceGroup.removeEditor(editor);

// 2. If target group doesn't exist, create it
if (!targetGroup) {
  const newGroup = this.createEditorGroup();
  this.splitView.add(newGroup.view, size, index);
}

// 3. Add editor to target group
targetGroup.addEditor(editor, index);

// 4. Update layout
this.layout(size);
```

#### 4. DOM Synchronization

DOM updates follow the tree mutation:

- Views are inserted/removed in the correct order
- DOM structure matches the layout tree

```typescript
if (index === this.viewItems.length) {
  this.viewContainer.appendChild(container);
} else {
  this.viewContainer.insertBefore(
    container,
    this.viewContainer.children.item(index)
  );
}
```

#### 5. Event Notification

- Services are notified of changes
- UI updates to reflect the new layout
- State returns to `Idle`

### Safety Mechanisms

1. **State Guards**: Operations only proceed when state is `Idle`
2. **Transactional Updates**: Changes are atomic
3. **Disposable Pattern**: Resources are properly cleaned up

```typescript
const onChangeDisposable = view.onDidChange((size) =>
  this.onViewChange(item, size)
);
const containerDisposable = toDisposable(() =>
  this.viewContainer.removeChild(container)
);
const disposable = combinedDisposable(
  onChangeDisposable,
  containerDisposable
);
```

4. **Error Handling**: Invalid operations throw errors rather than corrupting state

## 5. Editor Group Service Integration

The `GroupService` coordinates:

- Editor group lifecycle
- Tab management within groups
- Layout persistence
- Cross-group operations (move/copy)

Commands like `moveActiveEditor` use this service:

```typescript
commands.executeCommand('moveActiveEditor', {
  to: 'right',
  by: 'group',
  value: 1,
});
```

## 6. Complete Drag-and-Drop Flow

### Phase 1: Drag Start

1. User initiates drag on tab/group
2. Drag metadata is stored
3. Visual preview is created
4. Drag state is initialized

### Phase 2: Drag Over (Continuous)

1. Mouse position is tracked
2. Drop target is calculated
3. Visual feedback is updated
4. `preventDefault()` is called

### Phase 3: Drop

1. Drop target is validated
2. Layout state is checked (`State.Idle`)
3. Layout tree is mutated atomically
4. DOM is updated
5. Services are notified
6. State returns to `Idle`

### Phase 4: Cleanup

1. Visual indicators are removed
2. Drag state is cleared
3. Event listeners are cleaned up

## Summary

VS Code implements drag-and-drop with:

1. **Event Handling**: Uses `setupDragAndDrop` utility and `DragAndDropHandler` pattern
2. **Visual Feedback**: Provides preview indicators during hover
3. **Layout Tree**: Uses `SplitView` component for managing editor groups
4. **Safe Mutations**: Uses state management, atomic operations, and transactional updates
5. **Service Integration**: Coordinates with editor group services for state management

The system ensures consistency by using state management, atomic operations, and proper cleanup, preventing layout corruption during drag-and-drop operations.

