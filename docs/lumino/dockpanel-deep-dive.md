# Lumino DockPanel: Docking, Splitting, Resizing, Persistence, and Extension

This document explains how Lumino's `DockPanel` supports docking, splitting, resizing, and persistence of widgets, and how developers can safely extend it.

## Overview

`DockPanel` is Lumino's most powerful panel widget, designed for complex multi-panel applications. It provides:
- **Docking**: Widgets can be docked to edges (top, bottom, left, right)
- **Splitting**: Space can be split with adjustable handles
- **Tabbed Areas**: Multiple widgets can share space via tabs
- **Resizing**: Interactive resize handles for split panes
- **Persistence**: Layout configurations can be saved and restored
- **Extensibility**: Safe extension through renderer interfaces and configuration options

---

## 1. Docking System

### Insert Modes

DockPanel uses `InsertMode` enum to control how widgets are inserted:

```typescript
DockPanel.InsertMode:
  - BEFORE: Insert before a reference widget
  - AFTER: Insert after a reference widget
  - SPLIT_TOP: Split and insert above
  - SPLIT_BOTTOM: Split and insert below
  - SPLIT_LEFT: Split and insert to the left
  - SPLIT_RIGHT: Split and insert to the right
```

### Adding Widgets

Widgets are added using `IAddOptions`:

```typescript
IAddOptions:
  - ref?: Widget          // Reference widget for positioning
  - mode?: InsertMode     // How to insert the widget
  - rank?: number         // Z-order/priority for docking
  - insertMode?: InsertMode  // Alternative insert mode specification
```

### Docking Behavior

1. **Edge Docking**: Widgets can be docked to any edge of the panel
2. **Tab Areas**: Multiple widgets can be grouped in tabbed areas
3. **Split Areas**: Complex nested split configurations
4. **Rank System**: The `rank` property controls docking priority and ordering

### DockPanel Modes

```typescript
DockPanel.Mode:
  - SINGLE_DOCUMENT: Single document interface (like VS Code)
  - MULTIPLE_DOCUMENT: Multiple document interface (like JupyterLab)
```

The mode affects how widgets are arranged and how docking behaves.

---

## 2. Splitting System

### Split Area Configuration

Splits are managed through `ISplitAreaConfig`:

```typescript
ISplitAreaConfig:
  - orientation: 'horizontal' | 'vertical'
  - children: AreaConfig[]  // Nested areas
  - sizes: number[]         // Relative sizes of split sections
```

### Split Layout Structure

The DockLayout maintains a tree structure:
- **Tab Areas**: Contain multiple widgets in tabs
- **Split Areas**: Contain nested areas with splitters
- **Nested Splits**: Splits can contain other splits or tab areas

### Split Handles

Split handles are created by the renderer:
- Handles are interactive DOM elements
- They allow dragging to resize split sections
- The renderer's `createHandle()` method creates handle elements
- Handles are positioned between split sections

### Resize Behavior

1. **Drag to Resize**: Users drag handles to adjust sizes
2. **Proportional Resizing**: Sizes are maintained proportionally
3. **Minimum Sizes**: Widgets can specify minimum sizes
4. **Stretch Factors**: Controls how space is distributed

---

## 3. Resizing System

### Resize Messages

Widgets receive resize information through messages:

```typescript
Widget.ResizeMessage:
  - UnknownSize: Special value for uninitialized sizes
  - Contains width and height information
```

### Resize Flow

1. **User Interaction**: User drags a split handle
2. **Layout Calculation**: DockLayout calculates new sizes
3. **Message Dispatch**: ResizeMessage sent to affected widgets
4. **Widget Update**: Widgets update their DOM based on new sizes
5. **Layout Pass**: Layout manager recalculates positions

### Size Constraints

- **Minimum Sizes**: Widgets can specify minimum dimensions
- **Maximum Sizes**: Widgets can specify maximum dimensions
- **Size Basis**: Base size before stretching
- **Stretch Factors**: How widgets expand to fill space

### Overlay System

During drag operations, an overlay shows where the widget will be placed:

```typescript
DockPanel.Overlay:
  - show(geo: IOverlayGeometry): void
  - hide(delay: number): void
  - node: HTMLDivElement
```

The overlay:
- Shows preview of docking position
- Uses `IOverlayGeometry` for positioning
- Has configurable delay for hiding
- Provides visual feedback during drag operations

---

## 4. Persistence System

### Layout Configuration

Layouts can be serialized to `ILayoutConfig`:

```typescript
ILayoutConfig:
  - main?: AreaConfig        // Main area configuration
  - top?: AreaConfig         // Top docked area
  - bottom?: AreaConfig      // Bottom docked area
  - left?: AreaConfig        // Left docked area
  - right?: AreaConfig       // Right docked area
```

### Area Configuration

```typescript
AreaConfig:
  - type: 'tab' | 'split'     // Type of area
  - widgets?: Array<{        // Widgets in tab area
      ref: Widget,
      rank: number,
      insertMode?: InsertMode
    }>
  - children?: AreaConfig[]  // Nested areas (for splits)
  - sizes?: number[]         // Split sizes
  - orientation?: 'horizontal' | 'vertical'
```

### Saving State

To save a layout:
1. Get the layout configuration from DockLayout
2. Serialize widget references (typically by ID)
3. Store the configuration (JSON, localStorage, etc.)

### Restoring State

To restore a layout:
1. Deserialize the layout configuration
2. Resolve widget references from IDs
3. Reconstruct the layout using `ILayoutConfig`
4. Apply the configuration to DockLayout

### Persistence Best Practices

1. **Widget Identification**: Use stable IDs for widgets
2. **Versioning**: Include version info in saved layouts
3. **Validation**: Validate restored layouts before applying
4. **Fallback**: Provide default layout if restoration fails
5. **Incremental Updates**: Support partial layout updates

---

## 5. Safe Extension Patterns

### Renderer Interface

The primary extension point is the `IRenderer` interface:

```typescript
DockPanel.IRenderer:
  - renderTab(data: IRenderData): HTMLElement
  - renderTabCloseIcon(data: IRenderData): HTMLElement
  - renderOverlay(overlay: Overlay, geometry: IOverlayGeometry): void
  - createHandle(): HTMLElement  // For split handles
```

### Default Renderer

```typescript
DockPanel.defaultRenderer: IRenderer
```

The default renderer provides standard implementation. Developers can:
- Use the default renderer as-is
- Extend the default renderer class
- Implement a completely custom renderer

### Extension Strategies

#### 1. Custom Renderer

```typescript
class CustomDockPanelRenderer implements DockPanel.IRenderer {
  renderTab(data: IRenderData): HTMLElement {
    // Custom tab rendering
  }
  
  renderTabCloseIcon(data: IRenderData): HTMLElement {
    // Custom close icon
  }
  
  renderOverlay(overlay: Overlay, geometry: IOverlayGeometry): void {
    // Custom overlay rendering
  }
  
  createHandle(): HTMLElement {
    // Custom split handle
  }
}

// Use custom renderer
const panel = new DockPanel({
  renderer: new CustomDockPanelRenderer()
});
```

#### 2. Extending Default Renderer

```typescript
class ExtendedRenderer extends DockPanel.Renderer {
  renderTab(data: IRenderData): HTMLElement {
    const tab = super.renderTab(data);
    // Add custom styling or behavior
    return tab;
  }
}
```

#### 3. Custom Overlay

```typescript
class CustomOverlay implements DockPanel.IOverlay {
  node: HTMLDivElement;
  
  show(geo: IOverlayGeometry): void {
    // Custom show logic
  }
  
  hide(delay: number): void {
    // Custom hide logic
  }
}
```

### Configuration Options

Safe extension through `IOptions`:

```typescript
DockPanel.IOptions:
  - mode?: Mode              // Panel mode
  - renderer?: IRenderer     // Custom renderer
  - layout?: ILayoutConfig   // Initial layout
```

### Interface-Based Design

Lumino uses interfaces extensively for extension:
- **IRenderer**: Rendering customization
- **IOverlay**: Overlay behavior customization
- **ILayoutConfig**: Layout configuration
- **IAddOptions**: Widget addition options

This interface-based approach ensures:
- Type safety
- Clear contracts
- Backward compatibility
- Multiple implementations

### Safe Extension Guidelines

1. **Implement Interfaces**: Always implement required interface methods
2. **Preserve Behavior**: Maintain expected behavior when extending
3. **Handle Edge Cases**: Account for null/undefined values
4. **Test Thoroughly**: Test custom renderers with various layouts
5. **Document Changes**: Document any behavioral differences
6. **Version Compatibility**: Consider Lumino version compatibility

### Common Extension Points

1. **Tab Rendering**: Customize tab appearance and behavior
2. **Split Handles**: Customize handle appearance and interaction
3. **Overlay**: Customize drag-and-drop preview
4. **Layout Calculation**: Extend DockLayout for custom arrangements
5. **Widget Integration**: Integrate custom widgets seamlessly

---

## 6. Architecture Integration

### Message Loop Integration

DockPanel integrates with Lumino's message loop:
- Layout changes trigger messages
- Resize events propagate through messages
- Widget lifecycle managed via messages

### Layout Manager

DockPanel uses `DockLayout` internally:
- Handles layout calculations
- Manages split areas and tab areas
- Coordinates resize operations
- Maintains layout state

### Widget Hierarchy

```
DockPanel (Widget)
  └── DockLayout (Layout)
      ├── TabArea
      │   └── Widget[]
      └── SplitArea
          ├── TabArea | SplitArea
          └── TabArea | SplitArea
```

### Event Flow

1. **User Action**: User drags, clicks, or resizes
2. **Event Handling**: DockPanel handles the event
3. **Layout Update**: DockLayout recalculates
4. **Message Dispatch**: Messages sent to widgets
5. **DOM Update**: Widgets update their DOM

---

## 7. Best Practices

### Docking

- Use appropriate `InsertMode` for desired behavior
- Set `rank` values for consistent ordering
- Provide visual feedback during drag operations
- Handle edge cases (empty panels, single widget, etc.)

### Splitting

- Maintain minimum sizes for usability
- Use proportional sizing for better UX
- Handle nested splits carefully
- Test with various screen sizes

### Resizing

- Respect widget size constraints
- Provide smooth resize animations
- Handle resize edge cases gracefully
- Optimize for performance

### Persistence

- Use stable widget identifiers
- Validate restored layouts
- Handle missing widgets gracefully
- Support layout migration/versioning

### Extension

- Follow interface contracts
- Maintain backward compatibility
- Test with various scenarios
- Document custom behavior
- Consider performance implications

---

## Summary

Lumino's DockPanel provides a comprehensive system for:

1. **Docking**: Flexible widget positioning with edge docking and tab areas
2. **Splitting**: Nested split areas with interactive resize handles
3. **Resizing**: Message-driven resize system with size constraints
4. **Persistence**: Serializable layout configurations for state management
5. **Extension**: Safe extension through interfaces and renderer pattern

The architecture supports complex multi-panel applications while maintaining:
- Type safety through interfaces
- Extensibility through renderer pattern
- Performance through efficient layout calculations
- Reliability through message-driven updates

Developers can safely extend DockPanel by:
- Implementing `IRenderer` for custom rendering
- Using `IOptions` for configuration
- Extending default implementations
- Following interface contracts
- Testing thoroughly

This makes DockPanel suitable for applications like JupyterLab, VS Code extensions, and other complex multi-panel interfaces.


