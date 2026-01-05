# Svelte Memory Management & Performance Best Practices

This guide covers JavaScript memory management, garbage collection, and performance optimization patterns for Svelte applications, with a focus on handling large datasets from Parquet files and Rust backend operations.

## 🧠 JavaScript Memory Management Basics (For C++ Developers)

### Key Differences from C++

**C++**:

- Manual memory management (`new`/`delete`, smart pointers)
- Stack vs heap allocation is explicit
- Destructors run deterministically
- No garbage collector

**JavaScript**:

- **Automatic garbage collection** - memory is reclaimed automatically
- **Reference counting** + **mark-and-sweep** GC algorithm
- Objects are allocated on the heap (except primitives)
- No explicit destructors - GC runs when memory pressure is high

### How JavaScript GC Works

1. **Reference Counting**: Objects are tracked by how many references point to them
2. **Mark-and-Sweep**: Periodically, GC marks all reachable objects, then sweeps unreachable ones
3. **Generational GC**: Modern engines (V8) use generational GC - young objects are collected more frequently

**Key Insight**: Objects are garbage collected when they become **unreachable** (no references point to them).

### Memory Allocation Patterns

```typescript
// ✅ GOOD: Reuse arrays/objects when possible
const buffer = new Float64Array(1000); // Pre-allocated buffer
function processData(data: number[]) {
  // Reuse buffer instead of creating new array
  for (let i = 0; i < data.length; i++) {
    buffer[i] = data[i] * 2;
  }
  return buffer.slice(0, data.length); // Only create new array when needed
}

// ❌ BAD: Creating new arrays/objects in hot loops
function processData(data: number[]) {
  const result = []; // New array allocation
  for (let i = 0; i < data.length; i++) {
    result.push(data[i] * 2); // Multiple allocations
  }
  return result;
}
```

## 📊 Svelte-Specific Memory Patterns

### 1. Class-Based State vs Functional Approaches

#### **Question: Does rendering from a class consume more memory?**

**Answer: No, not significantly.** Here's why:

**Class-Based State (Current Pattern)**:

```typescript
// src/lib/state/postgres/postgres-wells-state.svelte.ts
export class PostgresWellsState {
  wells = $state<Well[]>([]); // Single array instance

  async loadWells() {
    this.wells = await fetchWells(); // Replaces array reference
  }
}

// Component usage
const wellsState = getPostgresWellsState();
// Component reads: wellsState.wells
```

**Memory Characteristics**:

- ✅ **Single instance** of state class (created once, reused)
- ✅ **Single array reference** - `wells` array is shared across components
- ✅ **No duplication** - all components read from same array
- ✅ **GC-friendly** - when `wells` is replaced, old array becomes unreachable and is GC'd

**Functional Approach**:

```typescript
// Helper function approach
async function loadWells(): Promise<Well[]> {
  return await fetchWells(); // Returns new array
}

// Component usage
let wells = $state<Well[]>([]);
async function refreshWells() {
  wells = await loadWells(); // New array allocation
}
```

**Memory Characteristics**:

- ⚠️ **New array allocation** on each fetch
- ⚠️ **Potential duplication** if multiple components call `loadWells()` independently
- ✅ **GC-friendly** - old arrays become unreachable when replaced

**Verdict**: **Class-based state is MORE memory-efficient** because:

1. **Single source of truth** - one array instance shared across all components
2. **No duplication** - components don't create their own copies
3. **Better cache locality** - data is loaded once and reused

### 2. When to Use Each Pattern

**Use Class-Based State When**:

- ✅ Data is **shared across multiple components**
- ✅ Data needs to **persist across navigation**
- ✅ Data needs **reactive updates** (realtime subscriptions)
- ✅ You want **single source of truth**

**Use Functional Approach When**:

- ✅ Data is **component-specific** (not shared)
- ✅ Data is **temporary** (component lifecycle only)
- ✅ Data is **computed/derived** (use `$derived` instead)

### 3. Memory-Efficient State Management Patterns

#### **Pattern 1: Clear Old Data Before Loading New**

```typescript
// ✅ GOOD: Clear immediately to allow GC
async loadWells() {
  this.wells = []; // Old array becomes unreachable immediately
  this.wells = await fetchWells(); // New array allocation
}

// ❌ BAD: Keep old data until new data arrives
async loadWells() {
  const newWells = await fetchWells();
  this.wells = newWells; // Old array stays in memory during fetch
}
```

**Why**: Clearing immediately allows GC to reclaim memory sooner, especially important for large datasets.

#### **Pattern 2: Use `$derived` for Computed Values**

```typescript
// ✅ GOOD: Memoized computation
const filteredWells = $derived(wellsState.wells.filter((w) => w.isActive));

// ❌ BAD: Recomputes on every render
let filteredWells = wellsState.wells.filter((w) => w.isActive);
```

**Why**: `$derived` only recalculates when dependencies change, avoiding unnecessary array allocations.

#### **Pattern 3: Avoid Creating New Array References**

```typescript
// ✅ GOOD: Preserve reference when data hasn't changed
const selectedWells = $derived(
  wellsState.wells.filter(w => w.selected)
);

// ❌ BAD: New array on every render
selectedValues={wellsState.wells.map(w => w.id)} // New array each render!
```

**Why**: New array references trigger unnecessary reactivity and prevent reference equality checks.

## 🚀 Large Data Rendering Strategies

### 1. Streaming Data from Rust Backend

**From `dag-execution.md`**: Data flows as Arrow `RecordBatch` from Rust backend.

**Strategy**: **Stream data in chunks**, don't load everything at once.

```typescript
// ✅ GOOD: Stream large datasets
async function* streamParquetData(
  projectId: string,
  wellIds: string[],
  chunkSize: number = 10000,
): AsyncGenerator<Float64Array[]> {
  for (const wellId of wellIds) {
    // Fetch data in chunks from Rust backend
    const response = await invoke<{ data: number[]; hasMore: boolean }>(
      "fetch_parquet_chunk",
      { projectId, wellId, offset: 0, limit: chunkSize },
    );

    yield new Float64Array(response.data);

    // Continue streaming if more data exists
    if (response.hasMore) {
      // Fetch next chunk...
    }
  }
}

// Component usage
let chartData = $state<Float64Array[]>([]);

async function loadChartData() {
  chartData = []; // Clear old data
  const chunks: Float64Array[] = [];

  for await (const chunk of streamParquetData(projectId, wellIds)) {
    chunks.push(chunk);
    // Update chart incrementally
    chartData = [...chunks]; // Trigger reactivity
  }
}
```

### 2. Typed Arrays for Performance

**From SciCharts docs**: Use `Float64Array` instead of regular arrays for numeric data.

```typescript
// ✅ GOOD: Typed arrays for numeric data
const xValues = new Float64Array(1_000_000);
const yValues = new Float64Array(1_000_000);

// Fill from Parquet data
for (let i = 0; i < depthData.length; i++) {
  xValues[i] = depthData[i];
  yValues[i] = curveData[i];
}

// Pass to SciCharts
dataSeries.appendRange(xValues, yValues);

// ❌ BAD: Regular arrays (slower, more allocations)
const xValues: number[] = [];
const yValues: number[] = [];
for (let i = 0; i < depthData.length; i++) {
  xValues.push(depthData[i]); // Multiple allocations
  yValues.push(curveData[i]);
}
```

**Benefits**:

- ✅ **Faster** - typed arrays are optimized by JS engines
- ✅ **Less memory** - contiguous memory layout
- ✅ **Better GC** - single allocation vs many small allocations

### 3. Virtual Scrolling for Large Lists

**For AG Grid tables** (already implemented):

- ✅ AG Grid handles virtual scrolling automatically
- ✅ Only renders visible rows
- ✅ Memory usage stays constant regardless of data size

**For custom lists**:

```typescript
// Use Svelte's built-in virtual scrolling or libraries like:
// - @tanstack/svelte-virtual (recommended)
// - svelte-virtual-list
```

### 4. Web Workers for Heavy Processing

**For large data transformations**:

```typescript
// worker.ts
self.onmessage = (
  e: MessageEvent<{ data: Float64Array; operation: string }>,
) => {
  const { data, operation } = e.data;

  let result: Float64Array;
  switch (operation) {
    case "filter":
      result = filterData(data);
      break;
    case "transform":
      result = transformData(data);
      break;
  }

  self.postMessage(result);
};

// Component
const worker = new Worker("/worker.js", { type: "module" });

async function processLargeDataset(data: Float64Array) {
  return new Promise<Float64Array>((resolve) => {
    worker.onmessage = (e) => resolve(e.data);
    worker.postMessage({ data, operation: "filter" });
  });
}
```

**Why**: Keeps main thread responsive, prevents UI freezing.

## 📈 SciCharts Performance Best Practices

### 1. Use Float64Array Views (Zero-Copy)

**From SciCharts docs**: Use `vectorToArrayViewF64()` for zero-copy access.

```typescript
import { vectorToArrayViewF64 } from "scichart";

// ✅ GOOD: Zero-copy view into WebAssembly memory
const dataSeries = new XyDataSeries(wasmContext, {
  xValues: new Float64Array(1_000_000),
  yValues: new Float64Array(1_000_000),
});

// Create view (not a copy!)
const xView = vectorToArrayViewF64(dataSeries.getNativeXValues(), wasmContext);

// Operate on view directly
for (let i = 0; i < xView.length; i++) {
  xView[i] = calculateValue(i);
}

// ⚠️ IMPORTANT: Re-map view if dataSeries size changes
// The view becomes invalid if underlying memory moves
```

### 2. Reuse Buffers

**From SciCharts docs**: Pre-allocate buffers and reuse them.

```typescript
// ✅ GOOD: Reuse buffers
const BUFFER_SIZE = 10_000;
const xBuffer = new Float64Array(BUFFER_SIZE);
const yBuffer = new Float64Array(BUFFER_SIZE);

function appendDataToChart(dataSeries: XyDataSeries, newData: number[]) {
  // Reuse buffers
  for (let i = 0; i < newData.length; i++) {
    xBuffer[i] = i;
    yBuffer[i] = newData[i];
  }

  // Append from buffer
  dataSeries.appendRange(
    xBuffer.slice(0, newData.length),
    yBuffer.slice(0, newData.length),
  );
}

// ❌ BAD: Allocate new arrays each time
function appendDataToChart(dataSeries: XyDataSeries, newData: number[]) {
  const xValues = new Float64Array(newData.length); // New allocation
  const yValues = new Float64Array(newData.length); // New allocation
  // ...
}
```

### 3. Batch Updates

```typescript
// ✅ GOOD: Batch multiple updates
dataSeries.suspendUpdates();
try {
  for (const chunk of dataChunks) {
    dataSeries.appendRange(chunk.x, chunk.y);
  }
} finally {
  dataSeries.resumeUpdates(); // Single redraw
}

// ❌ BAD: Individual updates trigger redraws
for (const chunk of dataChunks) {
  dataSeries.appendRange(chunk.x, chunk.y); // Redraws each time!
}
```

### 4. Memory Cleanup

```typescript
// ✅ GOOD: Clean up when component unmounts
import { onDestroy } from "svelte";

let chart: SciChartSurface;

onDestroy(() => {
  if (chart) {
    chart.delete(); // Free WebAssembly memory
  }
});
```

## 🎯 Component Memory Patterns

### 1. Avoid Storing Large Data in Component State

```typescript
// ✅ GOOD: Store reference, not copy
const wellsState = getPostgresWellsState();
const wells = $derived(wellsState.wells); // Reference, not copy

// ❌ BAD: Copy large arrays into component state
let wells = $state<Well[]>([]);
wells = [...wellsState.wells]; // Creates copy!
```

### 2. Use `untrack()` for Non-Reactive Reads

```typescript
import { untrack } from "svelte";

// ✅ GOOD: Read without creating reactive dependency
function processData() {
  const currentWells = untrack(() => wellsState.wells);
  // Process without triggering reactivity
}

// ❌ BAD: Creates unnecessary reactive dependency
function processData() {
  const currentWells = wellsState.wells; // Tracked by Svelte
}
```

### 3. Lazy Loading for Large Components

```typescript
// ✅ GOOD: Lazy load heavy components
import { onMount } from "svelte";

let ChartComponent;
let showChart = $state(false);

onMount(async () => {
  // Only load when needed
  if (showChart) {
    ChartComponent = (await import("./HeavyChart.svelte")).default;
  }
});
```

## 📋 Memory Management Checklist

### ✅ Do's

- ✅ **Use class-based state** for shared data (single source of truth)
- ✅ **Clear old data immediately** before loading new data
- ✅ **Use `$derived`** for computed values (memoization)
- ✅ **Use `Float64Array`** for numeric data (typed arrays)
- ✅ **Stream large datasets** in chunks
- ✅ **Reuse buffers** when possible
- ✅ **Batch updates** to charts/components
- ✅ **Clean up resources** in `onDestroy`
- ✅ **Use `untrack()`** for non-reactive reads

### ❌ Don'ts

- ❌ **Don't create new arrays** on every render
- ❌ **Don't copy large arrays** into component state
- ❌ **Don't store large data** in component props
- ❌ **Don't use regular arrays** for numeric data (use typed arrays)
- ❌ **Don't load entire datasets** at once (stream instead)
- ❌ **Don't forget to clean up** WebAssembly memory (SciCharts)
- ❌ **Don't create reactive dependencies** unnecessarily

## 🔍 Memory Profiling Tools

### Chrome DevTools

1. **Memory Tab**: Take heap snapshots to see memory usage
2. **Performance Tab**: Record performance and check memory timeline
3. **Coverage Tab**: See unused code that can be removed

### Svelte DevTools

- Shows component tree and state
- Helps identify unnecessary re-renders

### Best Practices for Profiling

```typescript
// Add memory markers for debugging
if (typeof performance !== "undefined" && performance.mark) {
  performance.mark("data-load-start");
  await loadLargeDataset();
  performance.mark("data-load-end");
  performance.measure("data-load", "data-load-start", "data-load-end");
}
```

## 📊 Performance Benchmarks

### Expected Performance (Typical Hardware)

- **1M data points**: < 100ms to load and render in SciCharts
- **10M data points**: < 1s with streaming
- **Typed arrays**: 2-3x faster than regular arrays
- **Zero-copy views**: 10-100x faster than copying data

## 🎯 Summary: Memory-Efficient Architecture

**For MudRock's Use Case**:

1. **State Management**: ✅ **Class-based** (single source of truth, shared across components)
2. **Data Loading**: ✅ **Streaming** from Rust backend in chunks
3. **Data Storage**: ✅ **Typed arrays** (`Float64Array`) for numeric data
4. **Chart Rendering**: ✅ **SciCharts** with zero-copy views and batched updates
5. **Memory Cleanup**: ✅ **Explicit cleanup** in `onDestroy` hooks

**Key Principle**: **Minimize allocations, maximize reuse, stream when possible.**
