# deck.gl Efficient Data Updates: Partial Updates, Attribute Diffing, and GPU Buffer Optimization

## Overview

deck.gl provides sophisticated mechanisms for efficiently updating layer data, minimizing both CPU computation and GPU buffer uploads. The system uses multiple strategies: shallow comparison for change detection, selective attribute invalidation, partial buffer updates, and incremental data loading. These optimizations enable smooth 60fps performance even when updating large datasets.

## Change Detection and Data Comparison

### Shallow Comparison

deck.gl uses **shallow comparison** to detect data changes:

- **Reference equality**: Compares data array reference, not contents
- **Fast detection**: O(1) operation instead of O(N) deep comparison
- **Performance**: Avoids expensive deep equality checks on large datasets

```javascript
// Shallow comparison example
const oldData = [
  { x: 1, y: 2 },
  { x: 3, y: 4 },
];
const newData = oldData; // Same reference

// Shallow compare: oldData === newData → true
// No update triggered

// vs.
const newData2 = [...oldData]; // New reference, same contents
// Shallow compare: oldData === newData2 → false
// Update triggered (even though contents are identical)
```

**Important**: Mutating array elements doesn't trigger updates:

```javascript
// This does NOT trigger an update
data[0].x = 100; // Mutating element
deck.setProps({ layers: [new ScatterplotLayer({ data })] });

// This DOES trigger an update
const newData = [...data];
newData[0] = { ...newData[0], x: 100 };
deck.setProps({
  layers: [new ScatterplotLayer({ data: newData })],
});
```

### Custom Data Comparison

For complex data structures, use `dataComparator`:

```javascript
new ScatterplotLayer({
  data: myData,
  dataComparator: (newData, oldData) => {
    // Custom comparison logic
    if (newData.length !== oldData.length) {
      return false; // Different length → update needed
    }

    // Compare timestamps to detect actual changes
    return (
      newData[0].timestamp === oldData[0].timestamp &&
      newData[newData.length - 1].timestamp ===
        oldData[oldData.length - 1].timestamp
    );
  },
});
```

This allows:

- **Deep comparison**: When shallow comparison isn't sufficient
- **Selective comparison**: Only check relevant fields
- **Performance**: Avoid unnecessary updates when data hasn't meaningfully changed

## Attribute Diffing with updateTriggers

### The Problem

When `data` changes, all attributes are recalculated. But sometimes only specific attributes need updates:

```javascript
// Problem: Only colors changed, but positions are recalculated too
const layer = new ScatterplotLayer({
  data: myData,
  getPosition: (d) => [d.lon, d.lat], // Unchanged
  getColor: (d) =>
    d.colorScheme === 'red' ? [255, 0, 0] : [0, 0, 255], // Changed
});

// When colorScheme changes, both positions and colors are recalculated
// This is wasteful if positions haven't changed
```

### Solution: updateTriggers

`updateTriggers` provides fine-grained control over which attributes update:

```javascript
const [colorScheme, setColorScheme] = useState('red');

const layer = new ScatterplotLayer({
  data: myData,
  getPosition: (d) => [d.lon, d.lat],
  getColor: (d) =>
    colorScheme === 'red' ? [255, 0, 0] : [0, 0, 255],

  // Only update colors when colorScheme changes
  updateTriggers: {
    getColor: [colorScheme], // Array of dependencies
  },
});

// When colorScheme changes:
// - getColor attribute is invalidated and recalculated
// - getPosition attribute is NOT recalculated (saved work!)
```

### updateTriggers Mechanism

1. **Dependency tracking**: Each accessor can have dependencies
2. **Shallow comparison**: Dependencies are shallow-compared each render
3. **Selective invalidation**: Only attributes with changed dependencies are invalidated
4. **Automatic update**: Invalidated attributes are recalculated during `updateState`

```javascript
// Multiple dependencies
updateTriggers: {
  getColor: ['colorScheme', 'opacity'],
  getRadius: ['sizeScale'],
  getPosition: ['projection']  // Only if projection changes
}

// When colorScheme changes:
// - getColor invalidated ✓
// - getRadius not invalidated ✗
// - getPosition not invalidated ✗
```

### Manual Attribute Invalidation

Layers can manually invalidate attributes:

```javascript
updateState({props, oldProps, changeFlags}) {
  if (props.customParam !== oldProps.customParam) {
    // Manually invalidate specific attribute
    this.state.attributeManager.invalidate('colors');
  }
}
```

## Partial Data Updates

### The Challenge

When only a few items in a large dataset change, recalculating all attributes is wasteful:

```javascript
// Problem: Updating 1 item in 1 million items
const data = [
  /* 1 million items */
];
data[500000] = { ...data[500000], x: 100 }; // Update one item

// Current behavior: All 1 million items' attributes recalculated
// Time: ~1000ms (very slow!)
```

### Solution: \_dataDiff

The `_dataDiff` prop (v7.2+) enables partial updates:

```javascript
function updateSingleItem(index, newItem) {
  // Create shallow copy
  const newData = data.slice();
  newData[index] = newItem;

  const layer = new ScatterplotLayer({
    data: newData,
    // Only update attributes for changed range
    _dataDiff: (newData, oldData) => {
      // Return array of changed ranges
      return [{ startRow: index, endRow: index + 1 }];
    },
    getPosition: (d) => [d.lon, d.lat],
    getColor: (d) => d.color,
  });

  // Only items in range [index, index+1) are recalculated
  // Time: ~0.001ms (1000x faster!)
}
```

### Partial Update Mechanism

1. **Range detection**: `_dataDiff` returns array of `{startRow, endRow}` ranges
2. **Selective iteration**: AttributeManager only iterates over changed ranges
3. **Partial buffer update**: Only affected buffer regions are updated
4. **GPU upload**: `gl.bufferSubData` uploads only changed bytes

```javascript
// Multiple changed ranges
_dataDiff: (newData, oldData) => {
  const ranges = [];

  // Find all changed items
  for (let i = 0; i < newData.length; i++) {
    if (newData[i] !== oldData[i]) {
      ranges.push({ startRow: i, endRow: i + 1 });
    }
  }

  return ranges;
};

// Or contiguous range
_dataDiff: (newData, oldData) => {
  // Update items 100-200
  return [{ startRow: 100, endRow: 200 }];
};
```

### Partial Update Constraints

Partial updates have constraints:

1. **Contiguous ranges only**: Non-overlapping `[startRow, endRow)` ranges
2. **Data array identity**: Data array reference must not change (mutation allowed)
3. **Array length**: Length must remain constant
4. **Fallback**: If constraints violated, falls back to full update

```javascript
// ✅ Valid: Contiguous range, same array reference
const newData = data.slice();
newData[100] = { ...newData[100], x: 100 };
_dataDiff: () => [{ startRow: 100, endRow: 101 }];

// ❌ Invalid: Array length changed
const newData = data.filter((_, i) => i !== 100);
// Falls back to full update

// ❌ Invalid: New array reference without mutation
const newData = [...data];
// Falls back to full update (warns once)
```

### Partial Updates for Non-Instanced Attributes

For layers with variable vertex counts (e.g., `PathLayer`):

```javascript
// PathLayer: Each path has different number of vertices
class PathLayer extends Layer {
  initializeState() {
    this.state.attributeManager.add({
      // Non-instanced: size varies per object
      positions: {
        size: 3,
        accessor: 'getPath',
        update: this.calculatePositions,
        // Support for partial updates
        getVertexCount: (index, object) => object.length,
        getStartPositions: ({
          index,
          object,
          value,
          offset,
        }) => {
          // Fill positions for this specific path
          for (let i = 0; i < object.length; i++) {
            value[offset++] = object[i][0];
            value[offset++] = object[i][1];
            value[offset++] = object[i][2] || 0;
          }
        },
      },
    });
  }
}
```

## GPU Buffer Upload Optimization

### Buffer Upload Strategies

deck.gl uses multiple strategies to minimize GPU buffer uploads:

#### 1. Full Buffer Upload (Default)

When data changes completely:

```javascript
// Full buffer recreation
attributeManager.update({
  data: newData,
  numInstances: newData.length,
});

// Process:
// 1. Iterate all data
// 2. Build typed array
// 3. Create/update GPU buffer
// 4. Upload entire buffer: gl.bufferData()
```

#### 2. Partial Buffer Update (Partial Updates)

When only a range changes:

```javascript
// Partial buffer update
attributeManager.invalidate('positions', {
  startRow: 100,
  endRow: 200,
});
attributeManager.update({
  data: newData,
  numInstances: newData.length,
  startIndex: 100,
  endIndex: 200,
});

// Process:
// 1. Iterate only range [100, 200)
// 2. Build typed array for range
// 3. Update buffer sub-range: gl.bufferSubData(offset, data)
// 4. Only affected bytes uploaded
```

**Performance**: `bufferSubData` is much faster than `bufferData`:

- **Full upload**: ~10-50ms for 1M items
- **Partial upload**: ~0.1-1ms for 100 items

#### 3. Direct Buffer Supply

Pre-allocated buffers bypass AttributeManager:

```javascript
import { Buffer } from 'luma.gl';

// Pre-create GPU buffer
const gpuBuffer = new Buffer(gl, {
  data: typedArrayData,
  usage: gl.STATIC_DRAW,
});

// Supply directly to layer
new ScatterplotLayer({
  attributes: {
    positions: {
      buffer: gpuBuffer, // Direct GPU buffer
      size: 3,
    },
  },
});

// No CPU iteration, no buffer upload
// Maximum performance
```

#### 4. Buffer Reuse

deck.gl reuses buffers when possible:

```javascript
// Buffer lifecycle
1. Check if buffer exists and is correct size
2. If yes: Reuse existing buffer
3. If no: Create new buffer
4. Upload data to buffer

// Buffer expansion
- WebGL buffers cannot be expanded
- Must recreate buffer if size increases
- WebGL2: Can use gl.copyBufferSubData for GPU-to-GPU copy
```

### WebGL Buffer Update Methods

deck.gl uses different WebGL methods based on update type:

```javascript
// Full buffer update
gl.bufferData(target, data, usage);
// - Replaces entire buffer
// - Can change buffer size
// - More expensive

// Partial buffer update
gl.bufferSubData(target, offset, data);
// - Updates sub-range
// - Cannot change buffer size
// - Much faster

// WebGL2: GPU-to-GPU copy
gl.copyBufferSubData(
  readTarget,
  writeTarget,
  readOffset,
  writeOffset,
  size
);
// - No CPU involvement
// - Very fast for buffer expansion
```

## Incremental Data Loading

### Problem: Concatenating Data

Concatenating new chunks forces full recalculation:

```javascript
// ❌ Bad: Concatenating data
let loadedData = [];
while ((chunk = await fetchNextChunk())) {
  loadedData = loadedData.concat(chunk); // New array reference
  render();
}

// Problem: Every chunk arrival triggers full buffer recalculation
// 1M items loaded → 100K new items → Recalculate all 1.1M items
```

### Solution 1: Separate Layers per Chunk

Create a layer for each chunk:

```javascript
// ✅ Good: Separate layers
let dataChunks = [];
while ((chunk = await fetchNextChunk())) {
  dataChunks.push(chunk);
  render();
}

function render() {
  const layers = dataChunks.map(
    (chunk, index) =>
      new ScatterplotLayer({
        id: `chunk-${index}`, // Unique, stable ID
        data: chunk,
        getPosition: (d) => d.position,
      })
  );

  // Only new chunk's buffers are calculated
  // Existing layers are reused (no recalculation)
}
```

**Benefits**:

- Only new chunks trigger buffer calculation
- Existing layers remain unchanged
- Efficient layer matching by ID

### Solution 2: Async Iterables (v7.2+)

Use async iterables for automatic incremental updates:

```javascript
// ✅ Best: Async iterables
async function* getData() {
  let chunk;
  while ((chunk = await fetchNextChunk())) {
    yield chunk;
  }
}

const layer = new ScatterplotLayer({
  data: getData(), // Async iterable
  getPosition: (d) => d.position,
});

// deck.gl automatically:
// 1. Detects new chunks from iterable
// 2. Updates only sub-buffers for new rows
// 3. No full buffer recalculation
```

**How it works**:

- Iterable yields chunks incrementally
- deck.gl tracks current position
- Only new rows trigger partial updates
- Automatic buffer expansion when needed

## Attribute Manager Update Flow

### Complete Update Flow

```javascript
// 1. Data change detected
shouldUpdateState({changeFlags}) {
  return changeFlags.dataChanged;  // true
}

// 2. State update triggered
updateState({props, oldProps, changeFlags}) {
  if (changeFlags.dataChanged) {
    // Invalidate all attributes
    this.state.attributeManager.invalidateAll();
  }
}

// 3. Attribute update
attributeManager.update({
  data: props.data,
  numInstances: props.data.length,
  props: props
});

// 4. For each invalidated attribute:
//    a. Check if pre-allocated buffer exists
//    b. If yes: Skip update
//    c. If no: Calculate attribute values
//    d. Build typed array
//    e. Upload to GPU buffer
```

### Selective Update Flow

```javascript
// 1. Only specific attribute changed
updateTriggers: {
  getColor: ['colorScheme'];
}

// 2. Only colors invalidated
attributeManager.invalidate('colors');

// 3. Only colors updated
attributeManager.update({
  data: props.data,
  numInstances: props.data.length,
  props: props,
  // Only 'colors' attribute recalculated
  // Other attributes skipped
});
```

### Partial Update Flow

```javascript
// 1. Partial data change
_dataDiff: () => [{ startRow: 100, endRow: 200 }];

// 2. Partial invalidation
attributeManager.invalidate('positions', {
  startRow: 100,
  endRow: 200,
});

// 3. Partial update
attributeManager.update({
  data: props.data,
  numInstances: props.data.length,
  startIndex: 100,
  endIndex: 200,
});

// 4. Only range [100, 200) processed
//    a. Iterate only data[100..200]
//    b. Build typed array for range
//    c. Calculate buffer offset
//    d. gl.bufferSubData(offset, rangeData)
```

## Performance Characteristics

### Update Performance Comparison

| Update Type          | CPU Cost   | GPU Upload | Total Time | Use Case                 |
| -------------------- | ---------- | ---------- | ---------- | ------------------------ |
| **Full update**      | 100-1000ms | 10-50ms    | 110-1050ms | Complete data change     |
| **Selective update** | 10-100ms   | 1-5ms      | 11-105ms   | Single attribute changed |
| **Partial update**   | 0.1-1ms    | 0.1-0.5ms  | 0.2-1.5ms  | Few items changed        |
| **Direct buffer**    | 0ms        | 0ms        | 0ms        | Pre-allocated buffers    |

### Real-World Example

Updating 1 item in 1M item dataset:

```javascript
// Full update
// - Iterate 1M items: ~500ms
// - Build buffers: ~200ms
// - Upload: ~50ms
// Total: ~750ms (causes stutter)

// Partial update
// - Iterate 1 item: ~0.001ms
// - Build buffer slice: ~0.01ms
// - bufferSubData: ~0.1ms
// Total: ~0.11ms (smooth 60fps)
```

**Improvement**: ~6800x faster!

## Best Practices

### 1. Use updateTriggers for Accessor Dependencies

```javascript
// ✅ Good: Track dependencies
const layer = new ScatterplotLayer({
  data: myData,
  getColor: (d) =>
    colorScheme === 'red' ? [255, 0, 0] : [0, 0, 255],
  updateTriggers: {
    getColor: [colorScheme], // Only update colors when scheme changes
  },
});

// ❌ Bad: No updateTriggers
// Colors recalculated on every data change, even if data unchanged
```

### 2. Use \_dataDiff for Frequent Small Updates

```javascript
// ✅ Good: Partial updates for drag operations
function onDrag(index, newPosition) {
  const newData = data.slice();
  newData[index] = {
    ...newData[index],
    position: newPosition,
  };

  const layer = new ScatterplotLayer({
    data: newData,
    _dataDiff: () => [
      { startRow: index, endRow: index + 1 },
    ],
  });
}

// ❌ Bad: Full update on every drag
// Causes stutter during interaction
```

### 3. Use Separate Layers for Incremental Loading

```javascript
// ✅ Good: Separate layers
const layers = dataChunks.map(
  (chunk, i) =>
    new ScatterplotLayer({
      id: `chunk-${i}`, // Stable ID
      data: chunk,
    })
);

// ❌ Bad: Concatenating
let allData = [];
allData = allData.concat(newChunk); // Triggers full recalculation
```

### 4. Pre-allocate Buffers for Maximum Performance

```javascript
// ✅ Best: Direct buffer supply
const buffer = new Buffer(gl, { data: typedArray });
new ScatterplotLayer({
  attributes: {
    positions: { buffer, size: 3 },
  },
});

// Bypasses all CPU work
```

### 5. Avoid Unnecessary Shallow Changes

```javascript
// ❌ Bad: Unnecessary shallow change
function render() {
  const layer = new ScatterplotLayer({
    data: [...data], // New array every render
  });
}

// ✅ Good: Same reference when unchanged
function render() {
  const layer = new ScatterplotLayer({
    data: data, // Same reference
  });
}

// Or use dataComparator
dataComparator: (newData, oldData) => {
  // Custom comparison to avoid false positives
};
```

## Advanced Techniques

### Combining Strategies

```javascript
// Combine partial updates + updateTriggers
const layer = new ScatterplotLayer({
  data: myData,
  getPosition: (d) => [d.lon, d.lat],
  getColor: (d) =>
    colorScheme === 'red' ? [255, 0, 0] : [0, 0, 255],

  // Partial update for positions
  _dataDiff: (newData, oldData) => {
    // Find changed items
    const ranges = [];
    for (let i = 0; i < newData.length; i++) {
      if (
        newData[i].lon !== oldData[i].lon ||
        newData[i].lat !== oldData[i].lat
      ) {
        ranges.push({ startRow: i, endRow: i + 1 });
      }
    }
    return ranges;
  },

  // Selective update for colors
  updateTriggers: {
    getColor: [colorScheme],
  },
});

// Result:
// - Only changed positions recalculated (partial)
// - Only colors recalculated when scheme changes (selective)
// - Maximum efficiency
```

### Custom Update Logic

```javascript
class CustomLayer extends Layer {
  updateState({ props, oldProps, changeFlags }) {
    // Custom invalidation logic
    if (props.customParam !== oldProps.customParam) {
      // Only invalidate specific attributes
      this.state.attributeManager.invalidate(
        'customAttribute'
      );
    }

    // Partial invalidation
    if (props.partialUpdate) {
      this.state.attributeManager.invalidate('positions', {
        startRow: props.updateStart,
        endRow: props.updateEnd,
      });
    }
  }
}
```

## Conclusion

deck.gl's data update system achieves high performance through:

1. **Shallow comparison**: Fast change detection without deep equality
2. **Selective invalidation**: `updateTriggers` update only changed attributes
3. **Partial updates**: `_dataDiff` updates only changed data ranges
4. **Efficient GPU uploads**: `bufferSubData` for partial updates, direct buffers for maximum performance
5. **Incremental loading**: Separate layers or async iterables for growing datasets

These optimizations enable:

- **Smooth interactions**: 60fps during drag operations with partial updates
- **Efficient updates**: 1000x+ faster for small changes in large datasets
- **Scalability**: Handle millions of items with responsive updates
- **Flexibility**: Multiple strategies for different use cases

The key insight is that **most updates are small** - only a few items or attributes change. By optimizing for these common cases, deck.gl maintains excellent performance even with very large datasets.

