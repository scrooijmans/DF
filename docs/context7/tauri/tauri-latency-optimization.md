# Tauri-Specific Latency Optimization Strategies

This document outlines Tauri-specific strategies for reducing latency between frontend and backend, leveraging Tauri's unique IPC architecture, channels, and binary data transfer capabilities.

## Tauri IPC Architecture

Tauri uses a custom IPC (Inter-Process Communication) system between the Rust backend and JavaScript frontend. Understanding this architecture is key to optimization:

- **Commands**: Synchronous or async function calls via `invoke()`
- **Events**: One-way messaging via `emit()` / `listen()`
- **Channels**: Streaming binary data with backpressure support
- **Response Types**: JSON (default) vs Binary (optimized)

## Strategy 1: Use Binary Response for Large Data

### Problem: JSON Serialization Overhead

By default, Tauri serializes command responses as JSON, which is slow for large numeric arrays:

```rust
// ❌ SLOW: JSON serialization for large arrays
#[tauri::command]
fn get_series_data() -> Vec<f64> {
    vec![1.0, 2.0, 3.0, ...] // 10,000+ values
}
// Serialization: ~20-50ms for 10K points
```

### Solution: Use `tauri::ipc::Response` for Binary Data

```rust
use tauri::ipc::Response;

#[tauri::command]
fn get_series_data() -> Response {
    let data: Vec<f64> = vec![1.0, 2.0, 3.0, ...];
    
    // Convert to bytes (little-endian)
    let bytes: Vec<u8> = data
        .iter()
        .flat_map(|&f| f.to_le_bytes())
        .collect();
    
    // Return as binary response (no JSON serialization)
    Response::new(bytes)
}
```

**Frontend:**

```typescript
import { invoke } from '@tauri-apps/api/core';

// Response automatically handles binary data
const binaryData = await invoke<number[]>('get_series_data');

// Convert to Float64Array (zero-copy in modern browsers)
const floatArray = new Float64Array(binaryData);
```

**Performance Improvement:**
- **JSON**: 20-50ms serialization + 15-30ms deserialization = **35-80ms**
- **Binary**: 2-5ms conversion + 1-2ms deserialization = **3-7ms**
- **Improvement**: **90-95% reduction**

### Optimized Binary Response with Arrow IPC

For even better performance, use Arrow IPC format:

```rust
use arrow::array::Float64Array;
use arrow::record_batch::RecordBatch;
use arrow::ipc::writer::StreamWriter;
use tauri::ipc::Response;

#[tauri::command]
fn get_series_data_arrow() -> Response {
    let x_values = Float64Array::from(vec![1.0, 2.0, 3.0, ...]);
    let y_values = Float64Array::from(vec![10.0, 20.0, 30.0, ...]);
    
    let schema = Arc::new(Schema::new(vec![
        Field::new("x", DataType::Float64, false),
        Field::new("y", DataType::Float64, false),
    ]));
    
    let batch = RecordBatch::try_new(
        schema.clone(),
        vec![Arc::new(x_values), Arc::new(y_values)],
    ).unwrap();
    
    // Serialize to Arrow IPC
    let mut buffer = Vec::new();
    let mut writer = StreamWriter::try_new(&mut buffer, &schema).unwrap();
    writer.write(&batch).unwrap();
    writer.finish().unwrap();
    
    Response::new(buffer)
}
```

**Frontend:**

```typescript
import { tableFromIPC } from 'apache-arrow';

const binaryData = await invoke<number[]>('get_series_data_arrow');
const arrayBuffer = new Uint8Array(binaryData).buffer;

// Zero-copy deserialization
const table = tableFromIPC(arrayBuffer);
const xValues = table.getChild('x')?.toArray() as Float64Array;
const yValues = table.getChild('y')?.toArray() as Float64Array;
```

## Strategy 2: Use Tauri Channels for Streaming

### Problem: Large Payloads Block IPC

Sending very large datasets (100K+ points) in a single `invoke()` call can block the IPC channel.

### Solution: Tauri Channels for Streaming

**Backend (Rust):**

```rust
use tauri::{command, AppHandle};
use tauri::ipc::Channel;

#[command]
async fn stream_series_data(
    app: AppHandle,
    series_id: String,
    on_event: Channel<Vec<u8>>,
) -> Result<(), String> {
    // Stream data in chunks
    let chunk_size = 10_000;
    let data = fetch_series_data(&series_id).await?;
    
    for chunk in data.chunks(chunk_size) {
        // Convert chunk to bytes
        let bytes: Vec<u8> = chunk
            .iter()
            .flat_map(|&f| f.to_le_bytes())
            .collect();
        
        // Send chunk via channel (non-blocking)
        on_event.send(bytes).await.map_err(|e| e.to_string())?;
        
        // Small delay to prevent overwhelming frontend
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
    }
    
    Ok(())
}
```

**Frontend (TypeScript):**

```typescript
import { invoke, Channel } from '@tauri-apps/api/core';

const onEvent = new Channel<number[]>();

onEvent.onmessage = (chunk) => {
  // Convert to Float64Array
  const floatArray = new Float64Array(chunk);
  
  // Incremental update - SciChart handles efficiently
  dataSeries.appendRange(floatArray, floatArray); // Example
};

// Start streaming
await invoke('stream_series_data', {
  seriesId: 'series-123',
  onEvent,
});
```

**Benefits:**
- **Progressive rendering** - chart updates as data arrives
- **Lower memory usage** - don't load all data at once
- **Better UX** - user sees progress immediately
- **Non-blocking** - doesn't freeze IPC channel

## Strategy 3: Optimize Async Commands

### Use `spawn_blocking` for CPU-Intensive Tasks

```rust
use tauri::command;
use tauri::async_runtime;

#[command]
async fn process_large_dataset(data: Vec<f64>) -> Result<Vec<f64>, String> {
    // ❌ BAD: Blocks main thread
    // let result = heavy_computation(data);
    
    // ✅ GOOD: Offload to worker thread
    let result = async_runtime::spawn_blocking(move || {
        heavy_computation(data) // CPU-intensive work
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?;
    
    Ok(result)
}
```

**Why This Matters:**
- Prevents UI freezing during computation
- Allows other commands to execute concurrently
- Better resource utilization

### Parallel Processing with Rayon

```rust
use rayon::prelude::*;

#[command]
async fn process_multiple_series(series_ids: Vec<String>) -> Result<Vec<Vec<f64>>, String> {
    async_runtime::spawn_blocking(move || {
        // Process in parallel
        series_ids
            .par_iter()
            .map(|id| process_single_series(id))
            .collect()
    })
    .await
    .map_err(|e| format!("Task error: {}", e))
}
```

## Strategy 4: Event System vs Channels

### When to Use Events

**Events are best for:**
- Small payloads (< 1KB)
- Infrequent updates
- Status notifications
- Progress updates

```rust
use tauri::{AppHandle, Emitter};

#[tauri::command]
async fn long_running_task(app: AppHandle) -> Result<(), String> {
    // Emit progress events
    for progress in 0..=100 {
        app.emit("task-progress", progress).unwrap();
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    
    Ok(())
}
```

```typescript
import { listen } from '@tauri-apps/api/event';

listen<number>('task-progress', (event) => {
  updateProgressBar(event.payload);
});
```

### When to Use Channels

**Channels are best for:**
- Large binary data (> 1KB)
- Streaming data
- High-frequency updates
- Binary payloads

```rust
#[command]
async fn stream_data(on_event: Channel<Vec<u8>>) -> Result<(), String> {
    for chunk in data_chunks {
        on_event.send(chunk).await?;
    }
    Ok(())
}
```

**Performance Comparison:**

| Method | Payload Size | Latency | Use Case |
|--------|-------------|---------|----------|
| **Events** | < 1KB | 1-5ms | Status, progress |
| **Channels** | > 1KB | 2-10ms | Binary streaming |
| **Commands (JSON)** | Any | 5-50ms | Simple requests |
| **Commands (Binary)** | Large | 3-7ms | Large arrays |

## Strategy 5: Shared State Management

### Avoid Repeated IPC Calls

**❌ BAD: Multiple IPC calls**

```typescript
// Frontend
const config = await invoke('get_config');
const data = await invoke('get_data', { config });
const processed = await invoke('process_data', { data });
// 3 round trips = 15-150ms
```

**✅ GOOD: Single IPC call with shared state**

```rust
use std::sync::Arc;
use tauri::State;

struct AppState {
    config: Arc<Mutex<Config>>,
    cache: Arc<Mutex<HashMap<String, Vec<f64>>>>,
}

#[tauri::command]
async fn get_processed_data(
    state: State<'_, AppState>,
) -> Result<Vec<f64>, String> {
    // Access shared state (no IPC overhead)
    let config = state.config.lock().unwrap();
    let cache = state.cache.lock().unwrap();
    
    // Check cache first
    if let Some(cached) = cache.get("processed_data") {
        return Ok(cached.clone());
    }
    
    // Process and cache
    let result = process_data(&config);
    cache.insert("processed_data".to_string(), result.clone());
    
    Ok(result)
}
```

**Performance Improvement:**
- **Multiple calls**: 15-150ms (3 round trips)
- **Single call**: 5-20ms (1 round trip)
- **Improvement**: **60-85% reduction**

## Strategy 6: Batch Operations

### Batch Multiple Operations

**❌ BAD: Individual calls**

```typescript
for (const id of seriesIds) {
  const data = await invoke('get_series', { id });
  // 10 calls = 50-500ms
}
```

**✅ GOOD: Batch call**

```rust
#[tauri::command]
async fn get_multiple_series(ids: Vec<String>) -> Result<Vec<Vec<f64>>, String> {
    // Process all in parallel
    let results: Vec<_> = ids
        .into_iter()
        .map(|id| fetch_series_data(&id))
        .collect();
    
    futures::future::try_join_all(results).await
}
```

```typescript
// Single call for all series
const allData = await invoke('get_multiple_series', { ids: seriesIds });
// 1 call = 10-50ms
```

## Strategy 7: Prefer Direct Memory Access

### Use Shared Memory When Possible

For very large datasets, consider using shared memory or memory-mapped files:

```rust
use memmap2::MmapOptions;
use std::fs::File;

#[tauri::command]
fn get_series_data_mmap(series_id: String) -> Result<Response, String> {
    let file_path = format!("data/{}.bin", series_id);
    let file = File::open(&file_path)
        .map_err(|e| format!("Failed to open file: {}", e))?;
    
    // Memory-map the file (zero-copy)
    let mmap = unsafe {
        MmapOptions::new()
            .map(&file)
            .map_err(|e| format!("Failed to mmap: {}", e))?
    };
    
    // Return as binary response (no copy)
    Ok(Response::new(mmap.to_vec()))
}
```

**Benefits:**
- **Zero-copy** for file-backed data
- **Lower memory usage** - OS handles paging
- **Fast access** - direct memory access

## Strategy 8: Optimize Serialization

### Use Efficient Data Structures

**❌ BAD: Nested structures**

```rust
#[derive(Serialize)]
struct SeriesData {
    metadata: HashMap<String, String>, // Slow to serialize
    points: Vec<Point>, // Nested structs
}

#[derive(Serialize)]
struct Point {
    x: f64,
    y: f64,
}
```

**✅ GOOD: Flat arrays**

```rust
#[tauri::command]
fn get_series_data() -> Response {
    // Flat arrays serialize faster
    let x_values: Vec<f64> = ...;
    let y_values: Vec<f64> = ...;
    
    // Convert to binary
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&(x_values.len() as u32).to_le_bytes());
    bytes.extend_from_slice(unsafe {
        std::slice::from_raw_parts(
            x_values.as_ptr() as *const u8,
            x_values.len() * 8,
        )
    });
    bytes.extend_from_slice(unsafe {
        std::slice::from_raw_parts(
            y_values.as_ptr() as *const u8,
            y_values.len() * 8,
        )
    });
    
    Response::new(bytes)
}
```

## Strategy 9: Debounce and Throttle IPC Calls

### Frontend: Debounce Frequent Calls

```typescript
import { debounce } from 'lodash-es';

// Debounce rapid updates
const debouncedUpdate = debounce(async (data: number[]) => {
  await invoke('update_series', { data });
}, 100); // Wait 100ms after last call

// Multiple rapid calls become one
debouncedUpdate([1, 2, 3]);
debouncedUpdate([1, 2, 3, 4]);
debouncedUpdate([1, 2, 3, 4, 5]);
// Only last call executes
```

### Backend: Batch Updates

```rust
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};

struct UpdateQueue {
    pending: Arc<Mutex<Vec<Update>>>,
}

impl UpdateQueue {
    async fn add_update(&self, update: Update) {
        self.pending.lock().await.push(update);
    }
    
    async fn flush(&self) {
        let updates = {
            let mut pending = self.pending.lock().await;
            pending.drain(..).collect::<Vec<_>>()
        };
        
        // Process all updates at once
        process_batch(updates).await;
    }
}

// Auto-flush every 100ms
tokio::spawn(async move {
    loop {
        sleep(Duration::from_millis(100)).await;
        queue.flush().await;
    }
});
```

## Strategy 10: Use Tauri's Built-in Optimizations

### Enable Release Optimizations

**Cargo.toml:**

```toml
[profile.release]
codegen-units = 1      # Better optimization
lto = true             # Link-time optimization
opt-level = "s"        # Optimize for size (faster)
panic = "abort"        # Smaller binary
strip = true           # Remove debug symbols
```

### Use Native WebView

Tauri uses system-native WebViews, which are faster than bundled Chromium:

- **macOS**: WKWebView (WebKit)
- **Windows**: WebView2 (Edge)
- **Linux**: WebKitGTK

**Benefits:**
- Smaller binary size
- Better system integration
- Faster startup
- Lower memory usage

## Complete Optimization Example

### Backend (Rust)

```rust
use tauri::{command, AppHandle, State};
use tauri::ipc::{Response, Channel};
use std::sync::Arc;
use tokio::sync::Mutex;

struct AppState {
    cache: Arc<Mutex<HashMap<String, Vec<u8>>>>,
}

#[command]
async fn get_series_data_optimized(
    series_id: String,
    state: State<'_, AppState>,
) -> Result<Response, String> {
    // Check cache first
    {
        let cache = state.cache.lock().await;
        if let Some(cached) = cache.get(&series_id) {
            return Ok(Response::new(cached.clone()));
        }
    }
    
    // Fetch data (offload to worker thread)
    let data = async_runtime::spawn_blocking(move || {
        fetch_series_data(&series_id)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?;
    
    // Convert to binary (no JSON serialization)
    let bytes: Vec<u8> = data
        .iter()
        .flat_map(|&f| f.to_le_bytes())
        .collect();
    
    // Cache result
    {
        let mut cache = state.cache.lock().await;
        cache.insert(series_id, bytes.clone());
    }
    
    Ok(Response::new(bytes))
}

#[command]
async fn stream_series_data(
    series_id: String,
    on_event: Channel<Vec<u8>>,
) -> Result<(), String> {
    // Stream in chunks for progressive rendering
    let chunk_size = 10_000;
    let data = fetch_series_data(&series_id).await?;
    
    for chunk in data.chunks(chunk_size) {
        let bytes: Vec<u8> = chunk
            .iter()
            .flat_map(|&f| f.to_le_bytes())
            .collect();
        
        on_event.send(bytes).await.map_err(|e| e.to_string())?;
    }
    
    Ok(())
}
```

### Frontend (TypeScript)

```typescript
import { invoke, Channel } from '@tauri-apps/api/core';
import { XyDataSeries, SciChartSurface } from 'scichart';

class OptimizedChartDataService {
  private cache = new Map<string, Float64Array[]>();
  
  // Single optimized call
  async loadSeriesData(
    seriesId: string,
    dataSeries: XyDataSeries,
    surface: SciChartSurface
  ): Promise<void> {
    // Check cache
    const cached = this.cache.get(seriesId);
    if (cached) {
      this.applyData(dataSeries, cached, surface);
      return;
    }
    
    // Fetch binary data
    const binaryData = await invoke<number[]>('get_series_data_optimized', {
      seriesId,
    });
    
    // Convert to typed arrays
    const arrayBuffer = new Uint8Array(binaryData).buffer;
    const floatArray = new Float64Array(arrayBuffer);
    
    // Split into x and y (assuming interleaved)
    const xValues = new Float64Array(floatArray.length / 2);
    const yValues = new Float64Array(floatArray.length / 2);
    
    for (let i = 0; i < floatArray.length / 2; i++) {
      xValues[i] = floatArray[i * 2];
      yValues[i] = floatArray[i * 2 + 1];
    }
    
    // Cache
    this.cache.set(seriesId, [xValues, yValues]);
    
    // Apply with batching
    this.applyData(dataSeries, [xValues, yValues], surface);
  }
  
  // Streaming version
  async streamSeriesData(
    seriesId: string,
    dataSeries: XyDataSeries,
    surface: SciChartSurface
  ): Promise<void> {
    const onEvent = new Channel<number[]>();
    
    surface.suspendUpdates();
    
    onEvent.onmessage = (chunk) => {
      const arrayBuffer = new Uint8Array(chunk).buffer;
      const floatArray = new Float64Array(arrayBuffer);
      
      // Split and append incrementally
      const xValues = new Float64Array(floatArray.length / 2);
      const yValues = new Float64Array(floatArray.length / 2);
      
      for (let i = 0; i < floatArray.length / 2; i++) {
        xValues[i] = floatArray[i * 2];
        yValues[i] = floatArray[i * 2 + 1];
      }
      
      dataSeries.appendRange(xValues, yValues);
    };
    
    await invoke('stream_series_data', { seriesId, onEvent });
    
    surface.resumeUpdates();
  }
  
  private applyData(
    dataSeries: XyDataSeries,
    [xValues, yValues]: [Float64Array, Float64Array],
    surface: SciChartSurface
  ): void {
    surface.suspendUpdates();
    try {
      dataSeries.appendRange(xValues, yValues);
    } finally {
      surface.resumeUpdates();
    }
  }
}
```

## Performance Benchmarks

### Latency Comparison (10K points)

| Method | Format | Latency | Improvement |
|--------|--------|---------|-------------|
| Baseline | JSON | 50-100ms | - |
| Binary Response | Binary | 5-15ms | **70-85%** ⭐ |
| Arrow IPC | Arrow | 3-10ms | **80-90%** ⭐⭐ |
| Channels (Streaming) | Binary | 2-5ms (first chunk) | **90-95%** ⭐⭐⭐ |
| Cached | Any | 0.1-1ms | **99%** |

### Optimization Impact

| Optimization | Latency Reduction |
|--------------|------------------|
| Binary vs JSON | 70-85% |
| Channels vs Commands | 50-70% |
| Caching | 90-99% |
| Batch operations | 60-85% |
| `spawn_blocking` | Prevents blocking |
| Shared state | 60-85% |

## Best Practices Summary

### 1. Use Binary Responses for Large Arrays
```rust
Response::new(bytes) // Instead of JSON
```

### 2. Use Channels for Streaming
```rust
Channel<Vec<u8>> // For large datasets
```

### 3. Offload CPU Work
```rust
async_runtime::spawn_blocking(|| { ... })
```

### 4. Cache Aggressively
```rust
State<'_, AppState> // Shared state
```

### 5. Batch Operations
```rust
get_multiple_series(ids: Vec<String>) // Single call
```

### 6. Use Events for Small Updates
```rust
app.emit("progress", value) // < 1KB
```

### 7. Debounce Frontend Calls
```typescript
debounce(invoke, 100) // Prevent spam
```

### 8. Enable Release Optimizations
```toml
lto = true
opt-level = "s"
```

## Summary

Tauri-specific optimizations can achieve:

- **70-90% latency reduction** using binary responses
- **90-95% reduction** with streaming channels
- **99% reduction** with caching
- **Non-blocking** UI with `spawn_blocking`
- **Progressive rendering** with channels

Key takeaways:
- **Use `Response::new()`** for binary data instead of JSON
- **Use Channels** for streaming large datasets
- **Use `spawn_blocking`** for CPU-intensive work
- **Cache in shared state** to avoid repeated IPC
- **Batch operations** to reduce round trips
- **Use Events** for small, infrequent updates

These Tauri-specific optimizations, combined with the general strategies from the previous document, can achieve **sub-10ms latency** for most operations.

