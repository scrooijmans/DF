# VS Code Post-Drag UI State Reconciliation

## Overview

This document explains how VS Code reconciles UI state after a drag operation completes, including how it updates internal layout models, restores focus, and rebinds views to their new containers.

## 1. Drop Event Handling and Initial State Capture

### Drop Event Processing

When a drop completes, VS Code:

1. **Validates Drop Target**: Ensures the drop location is valid
2. **Captures Current State**: Records active editor, focus, selection
3. **Prepares for Mutation**: Sets up for atomic layout changes

The drop handler structure:

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

### State Capture

Before making changes, VS Code captures:

- Active editor reference
- Current focus element
- Editor selection state
- Scroll positions
- Group relationships

## 2. Internal Layout Model Updates

### State Transition and Atomic Updates

VS Code updates the layout model atomically using the `SplitView` state machine:

```typescript
private doAddView(view: IView<TLayoutContext>, size: number | Sizing, index?: number): void {
  if (this.state !== State.Idle) {
    throw new Error('Cant modify splitview');
  }

  this.state = State.Busy;
  // ... perform layout modifications ...
}
```

### Reconciliation Process

The reconciliation process follows these steps:

#### 1. State Validation

```typescript
// Ensure layout is in Idle state before modification
if (this.state !== State.Idle) {
  // Queue or reject the operation
  return;
}
```

#### 2. Atomic Layout Mutation

```typescript
this.state = State.Busy;

// 1. Update internal data structures
this.viewItems.splice(index, 0, item);

// 2. Update DOM structure
if (index === this.viewItems.length) {
  this.viewContainer.appendChild(container);
} else {
  this.viewContainer.insertBefore(
    container,
    this.viewContainer.children.item(index)
  );
}

// 3. Recalculate layout
this.layout(size);

// 4. Return to Idle state
this.state = State.Idle;
```

#### 3. Service Notification

After layout changes:

- `EditorService` updates editor-to-group mappings
- `GroupService` updates group relationships
- `PartService` updates workbench layout state

### Model Service Updates

The `IModelService` maintains editor models independently of their container:

```typescript
export interface IModelService {
  createModel(
    value: string,
    languageIdentifier: LanguageIdentifier,
    resource: URI
  ): ITextModel;
  getModel(resource: URI): ITextModel | null;
  // Models persist across container moves
}
```

**Key Principle**: Models remain stable during drag operations; only container associations change.

## 3. View Rebinding to New Containers

### Container Reattachment Process

Views are reattached to new containers with proper lifecycle management. The `doAddView` function demonstrates the rebinding pattern:

```typescript
private doAddView(view: IView<TLayoutContext>, size: number | Sizing, index?: number): void {
  // Create new container element
  const container = $('.split-view-view');

  // Insert into DOM at correct position
  if (index === this.viewItems.length) {
    this.viewContainer.appendChild(container);
  } else {
    this.viewContainer.insertBefore(container, this.viewContainer.children.item(index));
  }

  // Rebind view to new container with proper lifecycle management
  const onChangeDisposable = view.onDidChange(size => this.onViewChange(item, size));
  const containerDisposable = toDisposable(() =>
    this.viewContainer.removeChild(container)
  );
  const disposable = combinedDisposable(onChangeDisposable, containerDisposable);

  // Create view item that binds view to container
  const item = this.orientation === Orientation.VERTICAL
    ? new VerticalViewItem(container, view, viewSize, disposable)
    : new HorizontalViewItem(container, view, viewSize, disposable);
}
```

### Rebinding Steps

1. **Container Creation**: New DOM container is created for the view
2. **DOM Insertion**: Container is inserted at the correct position
3. **Event Rebinding**: View's `onDidChange` and other events are reattached
4. **Disposable Management**: Cleanup is tied to container lifecycle
5. **View Item Creation**: Wrapper object links view to container

### Disposable Pattern for Cleanup

The disposable pattern ensures proper cleanup:

```typescript
const containerDisposable = toDisposable(() =>
  this.viewContainer.removeChild(container)
);
const disposable = combinedDisposable(
  onChangeDisposable,
  containerDisposable
);
```

When a view is removed:

- Disposables are disposed
- Event listeners are removed
- DOM elements are cleaned up
- Resources are released

## 4. Focus Restoration

### Focus Tracking

VS Code tracks focus state using polling:

```javascript
const trackFocus = ({ onFocus, onBlur }) => {
  const interval = 250;
  let isFocused = document.hasFocus();

  setInterval(() => {
    const target = getActiveFrame();
    const isCurrentlyFocused =
      document.hasFocus() ||
      !!(
        target &&
        target.contentDocument &&
        target.contentDocument.body.classList.contains(
          'vscode-context-menu-visible'
        )
      );

    if (isCurrentlyFocused === isFocused) {
      return;
    }

    isFocused = isCurrentlyFocused;
    if (isCurrentlyFocused) {
      onFocus();
    } else {
      onBlur();
    }
  }, interval);
};
```

### Focus Restoration After Drop

After a drop operation:

1. **Identify Moved Element**: Determine which editor/view was moved
2. **Restore Focus**: Set focus to the moved element

```typescript
// Focus restoration logic
if (document.activeElement === activeFrame) {
  // Already focused, no need to refocus
  return;
}
activeFrame.contentWindow.focus();
```

3. **Restore Editor Selection**: If applicable, restore selection state

```javascript
// Editor selection is preserved and restored
editor.selection = newSelection;

// Internally:
// 1. Set value on API object
// 2. Make async call to main side (e.g., trySetSelection)
// 3. Upon return, update API object with actual selection
```

### Active Editor Management

The `EditorService` manages active editor state:

```typescript
export class EditorService implements IEditorService {
  public openEditor(input: any): Promise<any> {
    // Logic to open an editor
    return Promise.resolve(null);
  }
}
```

After a drop:

- The moved editor becomes active in its new group
- Previous group's active editor is updated if needed
- Focus is set to the new active editor

## 5. State Synchronization

### Service Coordination

Multiple services coordinate state updates:

1. **Editor Service**: Updates editor-to-group mappings
2. **Group Service**: Updates group layout and relationships
3. **Part Service**: Updates workbench layout state
4. **Model Service**: Maintains model references (unchanged)

### Event-Driven Updates

Services emit events when state changes:

```typescript
// Services emit events when state changes
onDidDisposeModel: Event<IModel>;
onDidModelChange: Event<IModel>;
onDidShowContextView: Event<void>;
onDidHideContextView: Event<void>;
```

Listeners update:

- UI components
- Extension host
- Other dependent services

### Layout Recalculation

After container changes:

```typescript
// Layout is recalculated for all affected views
this.layout(size);

// Each view's layout method is called
view.layout(dimension: Dimension): void;
```

This ensures:

- Correct sizing
- Proper positioning
- Updated scroll positions
- Validated constraints

## 6. Complete Reconciliation Flow

The complete post-drop reconciliation sequence:

```typescript
// 1. Drop event received
handleDrop(data: DragAndDropData, event: DragEvent): void {
  // 2. Validate drop target
  if (!isValidDropTarget(data)) {
    return;
  }

  // 3. Capture current state
  const previousActiveEditor = this.getActiveEditor();
  const previousFocus = document.activeElement;
  const previousSelection = previousActiveEditor?.selection;

  // 4. Update layout model (atomic)
  this.state = State.Busy;
  try {
    // Remove from source
    this.removeView(sourceIndex);

    // Add to target
    this.addView(view, size, targetIndex);

    // Recalculate layout
    this.layout(size);

    // 5. Rebind views to containers
    this.rebindViewToContainer(view, newContainer);

    // 6. Update service state
    this.editorService.updateEditorGroup(editor, newGroup);
    this.groupService.updateLayout();

    // 7. Restore focus
    this.restoreFocus(movedEditor, previousSelection);

    // 8. Emit events
    this.emitStateChangeEvents();

  } finally {
    this.state = State.Idle;
  }
}
```

## 7. Error Handling and Rollback

If reconciliation fails:

1. **State Rollback**: Revert to previous layout state
2. **Focus Restoration**: Restore previous focus
3. **Error Notification**: Inform the user
4. **Cleanup**: Dispose of partially created resources

## Summary

VS Code reconciles UI state after drag operations through:

1. **Atomic Layout Updates**: Uses state guards (`State.Idle` → `State.Busy` → `State.Idle`)
2. **View Rebinding**: Properly reattaches views to new containers with lifecycle management
3. **Focus Restoration**: Restores focus to moved elements
4. **Service Synchronization**: Coordinates updates across multiple services via events
5. **Layout Recalculation**: Ensures all affected views are properly sized and positioned

This approach ensures UI consistency, preserves user state (focus, selection), and maintains proper resource cleanup.

