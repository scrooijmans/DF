# GoldenLayout Architecture: Layout Trees, Component Lifecycles, and Dynamic Panel Management

This document explains how GoldenLayout defines layout trees, manages component lifecycles, and dynamically adds or removes panels while maintaining layout consistency, based on the official GoldenLayout documentation.

## 1. Layout Tree Structure

### Hierarchical Configuration Model

GoldenLayout uses a hierarchical tree structure to represent the layout. The tree is defined through configuration objects that describe the layout's structure:

```typescript
interface ResolvedLayoutConfig {
    readonly root: ResolvedRootItemConfig | undefined;
    readonly dimensions: ResolvedLayoutConfig.Dimensions;
    readonly header: ResolvedLayoutConfig.Header;
    readonly settings: ResolvedLayoutConfig.Settings;
    readonly openPopouts: ResolvedPopoutLayoutConfig[];
    readonly resolved: true;
}
```

### Root Item Types

The root of the layout tree can be one of three types:

```typescript
type ResolvedRootItemConfig = 
    | ResolvedRowOrColumnItemConfig  // Horizontal or vertical split
    | ResolvedStackItemConfig         // Tabbed container
    | ResolvedComponentItemConfig;    // Single component
```

### Item Configuration Hierarchy

All items in the layout tree extend from `ResolvedItemConfig`:

```typescript
interface ResolvedItemConfig {
    readonly type: ItemType;                    // 'row', 'column', 'stack', 'component', 'ground'
    readonly id: string;                        // Unique identifier
    readonly content: readonly ResolvedItemConfig[];  // Child items (recursive)
    readonly size: number;                       // Size in specified unit
    readonly sizeUnit: SizeUnitEnum;            // 'px' or '%'
    readonly minSize?: number;
    readonly minSizeUnit: SizeUnitEnum;
    readonly isClosable: boolean;
}
```

### Ground Item (Root Container)

Internally, GoldenLayout wraps the root item in a "ground" item, which serves as the root container:

```typescript
interface ResolvedGroundItemConfig extends ResolvedItemConfig {
    readonly type: 'ground';
    readonly id: '';
    readonly isClosable: false;
    readonly minSize: 0;
    readonly size: 100;
    readonly sizeUnit: SizeUnitEnum.Percent;
}
```

The ground item is created automatically and contains the user-defined root item as its child.

### ContentItem Class Hierarchy

The runtime representation uses a class hierarchy:

- **`ContentItem`** (abstract base class) - All layout items extend this
  - **`ComponentItem`** - Represents a single component panel
  - **`Stack`** - Tabbed container holding multiple components
  - **`Row`** / **`Column`** - Split containers organizing children horizontally/vertically
  - **`GroundItem`** - Root container (internal)

Each `ContentItem` maintains:
- A reference to its parent (`ContentItem | null`)
- An array of child items (`ContentItem[]`)
- A reference to the `LayoutManager`
- An HTML element for DOM representation

## 2. Component Lifecycle Management

### Component Binding Flow

GoldenLayout uses an event-driven lifecycle for components. When a component needs to be created, the `bindComponentEvent` is fired:

```typescript
type BindComponentEventHandler = (
    container: ComponentContainer,
    itemConfig: ResolvedComponentItemConfig
) => ComponentContainer.BindableComponent;
```

#### Binding Process

1. **Component Creation Request**: When GoldenLayout needs a component, it fires `bindComponentEvent` with:
   - `ComponentContainer` - The container that will hold the component
   - `ResolvedComponentItemConfig` - Configuration including `componentType` and `componentState`

2. **Component Instantiation**: The handler creates the component:
   ```typescript
   private handleBindComponentEvent(
       container: ComponentContainer, 
       itemConfig: ResolvedComponentItemConfig
   ) {
       const componentTypeName = ResolvedComponentItemConfig
           .resolveComponentTypeName(itemConfig);
       const component = this.createVirtualComponent(
           container, 
           componentTypeName, 
           itemConfig.componentState
       );
       
       // Append to layout DOM
       const componentRootElement = component.rootHtmlElement;
       this._layoutElement.appendChild(componentRootElement);
       
       // Store mapping
       this._boundComponentMap.set(container, component);
       
       // Attach event handlers
       container.virtualRectingRequiredEvent = (container, width, height) => 
           this.handleContainerVirtualRectingRequiredEvent(container, width, height);
       container.virtualVisibilityChangeRequiredEvent = (container, visible) => 
           this.handleContainerVisibilityChangeRequiredEvent(container, visible);
       
       return {
           component,
           virtual: true,  // or false for non-virtual components
       };
   }
   ```

3. **Component Container Setup**: The `ComponentContainer` wraps the component and provides:
   - Lifecycle management (show/hide, focus/blur)
   - Size management
   - State management
   - Event emission

### Component Unbinding Flow

When a component is removed, `unbindComponentEvent` is fired:

```typescript
type UnbindComponentEventHandler = (container: ComponentContainer) => void;
```

#### Unbinding Process

```typescript
private handleUnbindComponentEvent(container: ComponentContainer) {
    const component = this._boundComponentMap.get(container);
    if (component === undefined) {
        throw new Error('handleUnbindComponentEvent: Component not found');
    }

    const componentRootElement = component.rootHtmlElement;
    if (componentRootElement === undefined) {
        throw new Error('Component does not have a root HTML element');
    }

    // Remove from DOM
    this._layoutElement.removeChild(componentRootElement);
    
    // Remove from internal mapping
    this._boundComponentMap.delete(container);
}
```

### Virtual Components

GoldenLayout supports "virtual" components for performance optimization. Virtual components:

- Are positioned absolutely outside the normal DOM flow
- Use event handlers for positioning (`virtualRectingRequiredEvent`)
- Can be hidden/shown via `virtualVisibilityChangeRequiredEvent`
- Require manual z-index management via `virtualZIndexChangeRequiredEvent`

#### Virtual Recting (Positioning)

```typescript
private handleContainerVirtualRectingRequiredEvent(
    container: ComponentContainer, 
    width: number, 
    height: number
) {
    const component = this._boundComponentMap.get(container);
    const rootElement = component.rootHtmlElement;
    
    // Calculate position relative to layout root
    const containerBoundingClientRect = container.element.getBoundingClientRect();
    const left = containerBoundingClientRect.left - 
                 this._goldenLayoutBoundingClientRect.left;
    const top = containerBoundingClientRect.top - 
                this._goldenLayoutBoundingClientRect.top;
    
    // Apply styles
    rootElement.style.left = this.numberToPixels(left);
    rootElement.style.top = this.numberToPixels(top);
    rootElement.style.width = this.numberToPixels(width);
    rootElement.style.height = this.numberToPixels(height);
}
```

#### Virtual Visibility Management

```typescript
private handleContainerVisibilityChangeRequiredEvent(
    container: ComponentContainer, 
    visible: boolean
) {
    const component = this._boundComponentMap.get(container);
    const componentRootElement = component.rootHtmlElement;
    
    if (visible) {
        componentRootElement.style.display = '';
    } else {
        componentRootElement.style.display = 'none';
    }
}
```

### ComponentContainer Lifecycle States

The `ComponentContainer` manages several lifecycle states:

- **Initialization**: Component is created and bound
- **Visible/Hidden**: Controlled via `show()` / `hide()` methods
- **Focused/Blurred**: Managed through `focus()` / `blur()` methods
- **Drag Mode**: Entered during drag operations (`enterDragMode()` / `exitDragMode()`)
- **Maximized**: When stack is maximized (`enterStackMaximised()` / `exitStackMaximised()`)
- **Destruction**: Component is unbound and removed

## 3. Dynamic Panel Management

### Adding Panels

Panels are added through the layout configuration tree. The `ContentItem.addChild()` method is used to add items:

```typescript
abstract class ContentItem extends EventEmitter {
    addChild(
        contentItem: ContentItem, 
        index?: number | null, 
        suspendResize?: boolean
    ): number;
}
```

#### Adding Process

1. **Create Item Configuration**: Define the new item's configuration:
   ```typescript
   const newComponentConfig: ComponentItemConfig = {
       type: 'component',
       componentType: 'MyComponent',
       componentState: { /* initial state */ },
       title: 'New Panel',
   };
   ```

2. **Resolve Configuration**: Convert to resolved config:
   ```typescript
   const resolvedConfig = ComponentItemConfig.resolve(
       newComponentConfig, 
       false
   );
   ```

3. **Create ContentItem**: Instantiate the appropriate `ContentItem` subclass:
   ```typescript
   const componentItem = new ComponentItem(
       layoutManager,
       resolvedConfig,
       parentItem
   );
   ```

4. **Add to Parent**: Add to the parent's children array:
   ```typescript
   parentItem.addChild(componentItem, index);
   ```

5. **Initialize**: The item initializes itself, creates DOM elements, and triggers component binding

### Removing Panels

Panels are removed through the `close()` method:

```typescript
class ComponentItem extends ContentItem {
    close(): void;
}
```

#### Removal Process

1. **Close Request**: Call `close()` on the `ComponentItem`
2. **Parent Notification**: The item notifies its parent
3. **Child Removal**: Parent removes the item from its children array
4. **Cleanup**: 
   - Component is unbound (triggers `unbindComponentEvent`)
   - DOM elements are removed
   - Event listeners are cleaned up
   - Item is destroyed

4. **Layout Recalculation**: If the parent becomes empty or needs restructuring:
   - Empty containers may be removed
   - Sibling items may be resized
   - Layout is recalculated

### Maintaining Layout Consistency

GoldenLayout maintains consistency through several mechanisms:

#### 1. Configuration Serialization

The layout can be serialized to/from configuration:

```typescript
// Save current layout
const config: ResolvedLayoutConfig = layoutManager.saveLayout();

// Load layout
layoutManager.loadLayout(config);
```

This ensures the layout tree structure is always representable as a configuration object.

#### 2. Size Management

Each item maintains size constraints:

```typescript
interface ResolvedItemConfig {
    readonly size: number;
    readonly sizeUnit: SizeUnitEnum;  // 'px' or '%'
    readonly minSize?: number;
    readonly minSizeUnit: SizeUnitEnum;
}
```

When items are added or removed:
- Remaining items are proportionally resized
- Minimum size constraints are enforced
- Size units (pixels vs. percentages) are respected

#### 3. Parent-Child Relationships

The tree structure maintains strict parent-child relationships:

- Each `ContentItem` has a `parent: ContentItem | null`
- Each `ContentItem` maintains `children: ContentItem[]`
- When a child is removed, it's removed from the parent's children array
- When a parent becomes empty, it may be removed or replaced

#### 4. DOM Synchronization

The DOM structure mirrors the layout tree:

- Each `ContentItem` has an associated HTML element
- Elements are added/removed as items are added/removed
- CSS classes and styles reflect the item's state
- Resize observers ensure DOM and layout stay synchronized

#### 5. Event-Driven Updates

Changes propagate through events:

- `ContentItem` extends `EventEmitter`
- Size changes trigger resize events
- Visibility changes trigger show/hide events
- Structural changes trigger layout update events

#### 6. Layout Recalculation

When structure changes:

1. **Size Calculation**: Each item calculates its required size
2. **Constraint Resolution**: Minimum sizes and constraints are applied
3. **Proportional Distribution**: Available space is distributed proportionally
4. **DOM Update**: Elements are repositioned and resized
5. **Component Notification**: Components are notified of size/position changes

### Example: Adding a Panel to a Stack

```typescript
// Get the stack item
const stackItem = layoutManager.rootItem.children[0] as Stack;

// Create new component config
const newPanelConfig: ComponentItemConfig = {
    type: 'component',
    componentType: 'DataViewer',
    title: 'Data Panel',
    componentState: { dataSource: 'table1' },
};

// Resolve config
const resolvedConfig = ComponentItemConfig.resolve(newPanelConfig, false);

// Create component item
const componentItem = new ComponentItem(
    layoutManager,
    resolvedConfig,
    stackItem
);

// Add to stack (appears as new tab)
stackItem.addChild(componentItem);
```

### Example: Removing a Panel

```typescript
// Get the component item
const componentItem = layoutManager.rootItem
    .children[0]
    .children[0] as ComponentItem;

// Close it (triggers removal)
componentItem.close();

// The layout automatically:
// 1. Removes the component from its parent
// 2. Unbinds the component (triggers unbindComponentEvent)
// 3. Removes DOM elements
// 4. Recalculates layout
// 5. If parent becomes empty, may remove or restructure parent
```

## Summary

GoldenLayout maintains layout consistency through:

1. **Tree Structure**: Hierarchical configuration model with `ContentItem` classes
2. **Lifecycle Events**: `bindComponentEvent` / `unbindComponentEvent` for component management
3. **Parent-Child Relationships**: Strict tree structure with automatic cleanup
4. **Size Management**: Proportional resizing with constraint enforcement
5. **DOM Synchronization**: DOM structure mirrors layout tree
6. **Event-Driven Updates**: Changes propagate through event system
7. **Configuration Serialization**: Layout can be saved/loaded as JSON

This architecture ensures that adding or removing panels maintains the layout's structural integrity, size constraints, and visual consistency.


