# GoldenLayout: Internal Layout Tree Restructuring and Validation

This document explains how GoldenLayout restructures its internal layout tree when components are dragged, split, or merged, and how it ensures layout validity throughout these operations.

## Layout Tree Structure

### ContentItem Hierarchy

GoldenLayout uses a tree structure where all layout items extend `ContentItem`:

```typescript
export abstract class ContentItem extends EventEmitter {
    constructor(
        layoutManager: LayoutManager, 
        config: ResolvedItemConfig,
        _parent: ContentItem | null,
        _element: HTMLElement
    );
    addChild(contentItem: ContentItem, index?: number | null, suspendResize?: boolean): number;
}
```

**Tree Node Types**:
- **Ground**: Root container (always present, never closable)
- **Row/Column**: Container items that arrange children horizontally/vertically
- **Stack**: Container that holds multiple components as tabs
- **Component**: Leaf nodes containing actual UI components

### Tree Structure Example

```
Ground (root)
└── Row
    ├── Stack
    │   ├── Component A
    │   └── Component B
    └── Column
        ├── Component C
        └── Component D
```

## Tree Restructuring Operations

### 1. Adding Components (Drag & Drop)

#### Process Flow

1. **Drag Start**: Component enters drag mode via `ComponentItem.enterDragMode()`
2. **Hit Testing**: Determines target location using segment-based detection
3. **Tree Modification**: Based on drop target:

#### Tab Reordering (Same Stack)

**Operation**: Moving within the same stack
- **No tree restructuring**: Only changes the order within the stack's `content` array
- **Stack maintains same parent-child relationships**
- Uses `addChild()` with a new index to reorder

```typescript
// Pseudo-code for tab reordering
stack.addChild(componentItem, newIndex);
// Component's parent remains the same stack
```

#### Pane Splitting (Creating New Containers)

**Operation**: Dropping on edge segments creates new containers

**Tree Restructuring**:
1. **Identify Target**: Determine which container to split
2. **Create New Container**: Create a new Row or Column based on split direction
3. **Reorganize Tree**:
   - Original container becomes child of new container
   - Dragged component becomes sibling of original container
   - New container replaces original in parent

**Example: Horizontal Split (Right Edge)**

```
Before:
Ground
└── Stack
    └── Component A

After (dropping Component B on right edge):
Ground
└── Row
    ├── Stack (original, now child of Row)
    │   └── Component A
    └── Stack (new, contains dragged component)
        └── Component B
```

**Implementation Pattern**:
```typescript
// Pseudo-code for split operation
const targetItem = hitTestResult.item;
const parent = targetItem.parent;
const newContainer = createRowOrColumn(direction);
const originalIndex = parent.children.indexOf(targetItem);

// Remove original from parent
parent.removeChild(targetItem);

// Add new container at original position
parent.addChild(newContainer, originalIndex);

// Add original as first child of new container
newContainer.addChild(targetItem, 0);

// Add dragged component as second child
newContainer.addChild(draggedComponent, 1);
```

#### Component Replacement (Body Drop)

**Operation**: Dropping on body segment replaces/swaps components

**Tree Restructuring**:
- If target is a Stack: Add component to stack (no structural change)
- If target is a Component: May create a Stack to hold both components

### 2. Removing Components (Close/Merge)

#### Component Removal

When a component is closed:

1. **Remove from Parent**: Component removed from parent's children array
2. **Unbind Component**: Triggers `unbindComponentEvent` to clean up component instance
3. **Cleanup Empty Containers**: Check if parent becomes empty or has only one child

#### Empty Container Cleanup

**Merging Single-Child Containers**:

When a container (Row/Column/Stack) has only one child after removal:

1. **Identify Single-Child Container**: Check if `children.length === 1`
2. **Promote Child**: Move the single child up to replace the container
3. **Update Parent**: Replace container with its child in parent's children array
4. **Update Parent Reference**: Child's parent becomes the container's parent
5. **Destroy Container**: Clean up the now-empty container

**Example: Container Merging**

```
Before (closing Component B):
Ground
└── Row
    ├── Stack
    │   └── Component A
    └── Stack
        └── Component B  ← closing this

After cleanup:
Ground
└── Row
    └── Stack
        └── Component A
```

**Further cleanup** (if Row now has only one child):
```
Ground
└── Stack
    └── Component A
```

**Implementation Pattern**:
```typescript
// Pseudo-code for cleanup after removal
function cleanupAfterRemove(item: ContentItem) {
    const parent = item.parent;
    if (parent && parent.children.length === 1) {
        const grandparent = parent.parent;
        const onlyChild = parent.children[0];
        
        // Remove parent from grandparent
        const parentIndex = grandparent.children.indexOf(parent);
        grandparent.removeChild(parent);
        
        // Add only child to grandparent at same position
        grandparent.addChild(onlyChild, parentIndex);
        
        // Update child's parent reference
        onlyChild.setParent(grandparent);
        
        // Destroy empty container
        parent.destroy();
        
        // Recursively check if grandparent needs cleanup
        cleanupAfterRemove(grandparent);
    }
}
```

### 3. Parent-Child Relationship Management

#### setParent() Method

Components use `setParent()` to maintain correct parent references:

```typescript
class ComponentItem extends ContentItem {
    protected setParent(parent: ContentItem): void;
}
```

**Critical Operations**:
- **When adding child**: Child's parent is set to the container
- **When moving child**: Parent reference is updated
- **When removing child**: Parent reference is cleared (set to null)

#### addChild() Method

The `addChild()` method handles:
- **Index Management**: Inserts at specified index or appends
- **Parent Reference**: Sets child's parent to this container
- **DOM Updates**: Updates DOM structure
- **Size Recalculation**: Triggers layout recalculation (unless `suspendResize` is true)

```typescript
addChild(
    contentItem: ContentItem, 
    index?: number | null, 
    suspendResize?: boolean
): number;
```

## Layout Validity Enforcement

### 1. Configuration-Based Validation

#### ResolvedItemConfig Structure

All items have a strict configuration structure:

```typescript
export interface ResolvedItemConfig {
    readonly content: readonly ResolvedItemConfig[];
    readonly id: string;
    readonly isClosable: boolean;
    readonly minSize: number | undefined;
    readonly minSizeUnit: SizeUnitEnum;
    readonly size: number;
    readonly sizeUnit: SizeUnitEnum;
    readonly type: ItemType;
}
```

**Validation Rules**:
- **Type Safety**: Each item has a specific type (`'ground'`, `'row'`, `'column'`, `'stack'`, `'component'`)
- **Content Constraints**: 
  - Components have empty `content: []`
  - Containers have non-empty `content` arrays
  - Stacks can only contain components
- **ID Uniqueness**: Each item has a unique ID

#### Configuration Error Handling

```typescript
export class ConfigurationError extends ExternalError {
    constructor(message: string, node?: string | undefined);
    readonly node?: string | undefined;
}
```

Invalid configurations throw `ConfigurationError` with node identification.

### 2. Type Checking Functions

GoldenLayout provides type guards for validation:

```typescript
export namespace ResolvedItemConfig {
    export function isComponentItem(itemConfig: ResolvedItemConfig): itemConfig is ResolvedComponentItemConfig;
    export function isGroundItem(itemConfig: ResolvedItemConfig): itemConfig is ResolvedGroundItemConfig;
    export function isStackItem(itemConfig: ResolvedItemConfig): itemConfig is ResolvedStackItemConfig;
}
```

These ensure type safety during tree operations.

### 3. Size and Constraint Validation

#### Size Properties

Each item has size constraints:

```typescript
interface ResolvedItemConfig {
    readonly size: number;
    readonly sizeUnit: SizeUnitEnum;
    readonly minSize: number | undefined;
    readonly minSizeUnit: SizeUnitEnum;
}
```

**Validation**:
- Sizes must be positive
- Minimum sizes are enforced
- Size units must be valid (`Pixel` or `Percent`)

#### Size Recalculation

After tree modifications, sizes are recalculated:

```typescript
class ComponentItem extends ContentItem {
    updateSize(force: boolean): void;
}
```

**Process**:
1. **Propagate from Root**: Size changes propagate from root (Ground) down
2. **Calculate Available Space**: Each container calculates space for children
3. **Distribute Sizes**: Sizes distributed based on `size` and `sizeUnit` properties
4. **Enforce Minimums**: Minimum sizes are enforced, adjusting other children if needed

### 4. Ground Item Invariants

The Ground item (root) has fixed properties that cannot be violated:

```typescript
export interface ResolvedGroundItemConfig extends ResolvedItemConfig {
    readonly id: '';
    readonly isClosable: false;
    readonly minSize: 0;
    readonly minSizeUnit: SizeUnitEnum.Pixel;
    readonly reorderEnabled: false;
    readonly size: 100;
    readonly sizeUnit: SizeUnitEnum.Percent;
    readonly type: 'ground';
}
```

**Invariants**:
- **Always exists**: Ground cannot be removed
- **Always 100% size**: Ground always takes full container
- **Never closable**: Ground cannot be closed
- **Single root**: Only one Ground exists per layout

### 5. Component Binding Validation

#### Component Lifecycle

Components go through binding/unbinding cycles:

```typescript
// Binding
bindComponentEvent: (container, itemConfig) => BindableComponent;

// Unbinding
unbindComponentEvent: (container) => void;
```

**Validation**:
- Components must be bound before use
- Unbinding must occur before removal
- Component types must be valid

### 6. Tree Structure Invariants

**Structural Rules**:
1. **Single Root**: Exactly one Ground item exists
2. **No Orphans**: Every item (except Ground) has a parent
3. **No Cycles**: Parent-child relationships form a tree (no cycles)
4. **Type Constraints**:
   - Stacks can only contain Components
   - Rows/Columns can contain Stacks, Rows, Columns, or Components
   - Components are always leaves (no children)
5. **Non-Empty Containers**: Containers (except Ground) should have at least one child
   - Empty containers are cleaned up automatically

## Restructuring Process Summary

### Complete Drag & Drop Flow

1. **Pre-Drag Validation**:
   - Verify component can be dragged (`reorderEnabled`)
   - Check source container validity

2. **Drag Operation**:
   - Enter drag mode (`enterDragMode()`)
   - Track mouse position
   - Perform hit testing

3. **Drop Validation**:
   - Verify drop target is valid
   - Check if operation is allowed
   - Validate target container type

4. **Tree Restructuring**:
   - Remove component from source (if moving)
   - Create new containers if splitting
   - Add component to target location
   - Update all parent-child references

5. **Cleanup**:
   - Remove empty containers
   - Merge single-child containers
   - Update DOM structure

6. **Size Recalculation**:
   - Propagate size changes from root
   - Enforce minimum sizes
   - Update all item dimensions

7. **Post-Operation Validation**:
   - Verify tree structure is valid
   - Ensure all references are correct
   - Validate configuration can be serialized

### Configuration Serialization

The `toConfig()` method ensures the tree can be serialized:

```typescript
class ComponentItem extends ContentItem {
    toConfig(): ResolvedComponentItemConfig;
}
```

**Validation**:
- All items must be able to serialize to valid config
- Config must be able to recreate the same tree structure
- Circular references are impossible (tree structure prevents this)

## Error Handling

### Configuration Errors

Invalid operations throw `ConfigurationError`:

```typescript
export class ConfigurationError extends ExternalError {
    constructor(message: string, node?: string | undefined);
    readonly node?: string | undefined;
}
```

### Validation Failures

Common validation failures:
- **Invalid parent type**: Trying to add component to invalid container
- **Missing required properties**: Config missing required fields
- **Size constraint violations**: Sizes don't add up correctly
- **Type mismatches**: Wrong item type for operation

## Summary

GoldenLayout ensures layout validity through:

1. **Strict Type System**: Type-safe item configurations
2. **Parent-Child Management**: Proper reference maintenance via `setParent()` and `addChild()`
3. **Automatic Cleanup**: Empty and single-child containers are merged automatically
4. **Size Validation**: Size constraints enforced during recalculation
5. **Ground Invariants**: Root container has fixed, unchangeable properties
6. **Configuration Validation**: All operations must result in serializable, valid configurations
7. **Error Handling**: Invalid operations throw descriptive errors

The system maintains a valid tree structure at all times, automatically cleaning up unnecessary containers and ensuring all parent-child relationships are correct throughout drag, split, and merge operations.

