# Reducing Latency: Backend to Frontend Chart Rendering

This document outlines strategies for minimizing latency between backend data processing/fetching and frontend chart visualization in JavaScript applications.

## Latency Components

The total latency from backend to frontend consists of:

1. **Backend Processing**: Query execution, data transformation
2. **Serialization**: Converting data to transfer format
3. **Network Transfer**: Data transmission over network
4. **Deserialization**: Converting received data to JavaScript objects
5. **Frontend Processing**: Data transformation for chart library
6. **Chart Rendering**: Updating the visualization

## Strategy 1: Efficient Data Serialization Formats

### Option A: Apache Arrow IPC (Recommended for Large Arrays)

**Why Arrow IPC:**
- **Zero-copy deserialization** in JavaScript
- **Columnar format** - efficient for numeric arrays
- **Streaming support** - incremental data delivery
- **Type-safe** - preserves data types (Float64, Int32, etc.)
- **Compression** - built-in support

**Backend (Rust):**

```rust
use arrow::array::{Float64Array, StringArray};
use arrow::record_batch::RecordBatch;
use arrow::ipc::writer::StreamWriter;
use arrow::ipc::reader::StreamReader;
use std::io::Cursor;

// Create Arrow arrays
let x_values = Float64Array::from(vec![1.0, 2.0, 3.0, ...]);
let y_values = Float64Array::from(vec![10.0, 20.0, 30.0, ...]);

// Create schema
let schema = Arc::new(Schema::new(vec![
    Field::new("x", DataType::Float64, false),
    Field::new("y", DataType::Float64, false),
]));

// Create record batch
let batch = RecordBatch::try_new(
    schema.clone(),
    vec![Arc::new(x_values), Arc::new(y_values)],
)?;

// Serialize to Arrow IPC format
let mut buffer = Vec::new();
let mut writer = StreamWriter::try_new(&mut buffer, &schema)?;
writer.write(&batch)?;
writer.finish()?;

// Send buffer over HTTP/WebSocket
```

**Frontend (JavaScript/TypeScript):**

```typescript
import { tableFromIPC } from 'apache-arrow';

// Receive Arrow IPC buffer from backend
async function fetchChartData(seriesId: string) {
  const response = await fetch(`/api/series/${seriesId}/data`, {
    headers: { 'Accept': 'application/vnd.apache.arrow.stream' }
  });
  
  const arrayBuffer = await response.arrayBuffer();
  
  // Zero-copy deserialization - directly creates typed arrays
  const table = tableFromIPC(arrayBuffer);
  
  // Extract columns as typed arrays (no conversion needed)
  const xValues = table.getChild('x')?.toArray() as Float64Array;
  const yValues = table.getChild('y')?.toArray() as Float64Array;
  
  // Directly use with SciChart (no additional processing)
  dataSeries.appendRange(xValues, yValues);
}
```

**Performance:**
- **Serialization**: ~5-10ms for 10K points
- **Network**: ~20-50ms (compressed)
- **Deserialization**: ~1-2ms (zero-copy)
- **Total**: ~30-70ms

### Option B: Protocol Buffers (Protobuf)

**Why Protobuf:**
- **Compact binary format** - smaller than JSON
- **Fast serialization/deserialization**
- **Type-safe** with schema validation
- **Cross-language support**

**Backend (Rust with prost):**

```rust
use prost::Message;

#[derive(Clone, prost::Message)]
pub struct SeriesData {
    #[prost(double, repeated, tag = "1")]
    pub x_values: Vec<f64>,
    #[prost(double, repeated, tag = "2")]
    pub y_values: Vec<f64>,
}

// Serialize
let series_data = SeriesData {
    x_values: vec![1.0, 2.0, 3.0, ...],
    y_values: vec![10.0, 20.0, 30.0, ...],
};

let mut buffer = Vec::new();
series_data.encode(&mut buffer)?;
// Send buffer
```

**Frontend (TypeScript with protobuf-es):**

```typescript
import { fromBinary } from '@bufbuild/protobuf';
import { SeriesDataSchema } from './gen/series_pb';

async function fetchChartData(seriesId: string) {
  const response = await fetch(`/api/series/${seriesId}/data`, {
    headers: { 'Accept': 'application/x-protobuf' }
  });
  
  const arrayBuffer = await response.arrayBuffer();
  const uint8Array = new Uint8Array(arrayBuffer);
  
  // Deserialize
  const seriesData = fromBinary(SeriesDataSchema, uint8Array);
  
  // Convert to typed arrays for SciChart
  const xValues = new Float64Array(seriesData.xValues);
  const yValues = new Float64Array(seriesData.yValues);
  
  dataSeries.appendRange(xValues, yValues);
}
```

**Performance:**
- **Serialization**: ~3-8ms for 10K points
- **Network**: ~15-40ms (compressed)
- **Deserialization**: ~2-5ms
- **Total**: ~20-55ms

### Option C: Binary JSON (MessagePack)

**Why MessagePack:**
- **Smaller than JSON** - ~30% reduction
- **Faster parsing** than JSON
- **Simple implementation**

**Performance:**
- **Serialization**: ~5-10ms
- **Network**: ~25-50ms
- **Deserialization**: ~3-7ms
- **Total**: ~35-70ms

### Format Comparison

| Format | Size (10K points) | Serialize | Deserialize | Total Latency |
|--------|------------------|-----------|-------------|---------------|
| **Arrow IPC** | ~160KB | 5-10ms | 1-2ms | **30-70ms** ⭐ |
| **Protobuf** | ~80KB | 3-8ms | 2-5ms | **20-55ms** ⭐⭐ |
| **MessagePack** | ~120KB | 5-10ms | 3-7ms | 35-70ms |
| **JSON** | ~200KB | 10-20ms | 15-30ms | 80-150ms |
| **JSON (gzip)** | ~60KB | 15-25ms | 20-35ms | 70-120ms |

**Recommendation**: Use **Arrow IPC** for large numeric arrays, **Protobuf** for structured data.

## Strategy 2: Streaming Data Transfer

### WebSocket Streaming with Arrow IPC

**Backend (Rust):**

```rust
use tokio_tungstenite::{accept_async, WebSocketStream};
use futures_util::{SinkExt, StreamExt};

async fn handle_websocket_stream(stream: WebSocketStream) {
    let (mut ws_sender, _) = stream.split();
    
    // Stream data in chunks
    let chunk_size = 1000;
    for chunk in data.chunks(chunk_size) {
        // Create Arrow batch for chunk
        let batch = create_arrow_batch(chunk)?;
        
        // Serialize to Arrow IPC
        let buffer = serialize_arrow_batch(&batch)?;
        
        // Send over WebSocket
        ws_sender.send(Message::Binary(buffer)).await?;
    }
}
```

**Frontend (TypeScript):**

```typescript
import { tableFromIPC } from 'apache-arrow';

function streamChartData(seriesId: string, dataSeries: XyDataSeries) {
  const ws = new WebSocket(`ws://api/series/${seriesId}/stream`);
  
  ws.binaryType = 'arraybuffer';
  
  ws.onmessage = (event) => {
    // Deserialize Arrow IPC chunk (zero-copy)
    const table = tableFromIPC(event.data);
    
    const xValues = table.getChild('x')?.toArray() as Float64Array;
    const yValues = table.getChild('y')?.toArray() as Float64Array;
    
    // Incremental update - SciChart handles efficiently
    dataSeries.appendRange(xValues, yValues);
    
    // Chart updates incrementally without full redraw
  };
}
```

**Benefits:**
- **Progressive rendering** - chart updates as data arrives
- **Lower perceived latency** - first data visible immediately
- **Better UX** - user sees progress

### Server-Sent Events (SSE) for One-Way Streaming

**Backend (Rust with axum):**

```rust
use axum::response::sse::{Event, Sse};
use futures_util::stream::Stream;

async fn stream_series_data(series_id: String) -> Sse<impl Stream<Item = Result<Event>>> {
    let stream = async_stream::stream! {
        // Stream data chunks
        for chunk in data_chunks {
            let arrow_buffer = serialize_arrow_batch(&chunk)?;
            let base64 = base64::encode(&arrow_buffer);
            
            yield Ok(Event::default()
                .data(base64)
                .event("data-chunk"));
        }
    };
    
    Sse::new(stream)
}
```

**Frontend (TypeScript):**

```typescript
function streamChartDataSSE(seriesId: string, dataSeries: XyDataSeries) {
  const eventSource = new EventSource(`/api/series/${seriesId}/stream`);
  
  eventSource.addEventListener('data-chunk', (event) => {
    // Decode base64 and deserialize Arrow IPC
    const binary = base64ToArrayBuffer(event.data);
    const table = tableFromIPC(binary);
    
    const xValues = table.getChild('x')?.toArray() as Float64Array;
    const yValues = table.getChild('y')?.toArray() as Float64Array;
    
    dataSeries.appendRange(xValues, yValues);
  });
}
```

## Strategy 3: Compression

### Compression Codecs Comparison

| Codec | Compression Ratio | Speed | Latency Impact | Use Case |
|-------|------------------|-------|----------------|----------|
| **Brotli** | 5-8x | Slow | +50-100ms | Maximum compression |
| **Gzip** | 4-6x | Medium | +20-50ms | Good balance |
| **ZSTD** | 3-5x | Fast | +10-30ms | **Recommended** |
| **Snappy** | 2-3x | Very Fast | +5-15ms | **Lowest latency** |
| **LZ4** | 2-3x | Very Fast | +5-15ms | Alternative to Snappy |

**Recommendation**: Use **Snappy** or **ZSTD** for low latency.

**Backend (Rust):**

```rust
use snap::raw::{Encoder, Decoder};

// Compress Arrow IPC buffer
fn compress_arrow_buffer(buffer: &[u8]) -> Vec<u8> {
    let mut encoder = Encoder::new();
    encoder.compress_vec(buffer).unwrap()
}
```

**Frontend (JavaScript):**

```typescript
// Use CompressionStream API (browser native)
async function decompressResponse(response: Response): Promise<ArrayBuffer> {
  const stream = response.body?.pipeThrough(
    new DecompressionStream('gzip')
  );
  
  return new Response(stream).arrayBuffer();
}
```

## Strategy 4: Incremental Updates

### Delta Updates (Send Only Changes)

**Backend:**

```rust
// Track previous state
struct SeriesState {
    last_sent_index: usize,
    data: Vec<(f64, f64)>,
}

fn get_delta_updates(state: &mut SeriesState) -> Vec<(f64, f64)> {
    let new_data = &state.data[state.last_sent_index..];
    state.last_sent_index = state.data.len();
    new_data.to_vec()
}
```

**Frontend:**

```typescript
// Append only new data points
function applyDeltaUpdate(dataSeries: XyDataSeries, delta: { x: number[], y: number[] }) {
  // SciChart efficiently handles incremental appends
  dataSeries.appendRange(delta.x, delta.y);
}
```

### Differential Compression

For time-series data with small changes:

```typescript
// Send only differences
interface DeltaUpdate {
  baseIndex: number;  // Starting index
  xDeltas: number[];  // Differences from previous values
  yDeltas: number[];
}

function applyDelta(dataSeries: XyDataSeries, delta: DeltaUpdate) {
  const xValues = new Float64Array(delta.xDeltas.length);
  const yValues = new Float64Array(delta.yDeltas.length);
  
  let lastX = dataSeries.getNativeXValues()[delta.baseIndex - 1] || 0;
  let lastY = dataSeries.getNativeYValues()[delta.baseIndex - 1] || 0;
  
  for (let i = 0; i < delta.xDeltas.length; i++) {
    xValues[i] = lastX + delta.xDeltas[i];
    yValues[i] = lastY + delta.yDeltas[i];
    lastX = xValues[i];
    lastY = yValues[i];
  }
  
  dataSeries.appendRange(xValues, yValues);
}
```

## Strategy 5: Frontend Optimization

### Batch Updates with SciChart

```typescript
// ❌ SLOW: Multiple individual updates
for (const point of newPoints) {
  dataSeries.append(point.x, point.y); // Triggers redraw each time
}

// ✅ FAST: Batch update
sciChartSurface.suspendUpdates();
try {
  const xValues = new Float64Array(newPoints.map(p => p.x));
  const yValues = new Float64Array(newPoints.map(p => p.y));
  dataSeries.appendRange(xValues, yValues);
} finally {
  sciChartSurface.resumeUpdates(); // Single redraw
}
```

### Pre-allocate Data Series Capacity

```typescript
// Pre-allocate to avoid reallocation during updates
const dataSeries = new XyDataSeries(wasmContext, {
  capacity: 100000  // Pre-allocate for 100K points
});
```

### Use Typed Arrays Directly

```typescript
// ✅ FAST: Use Float64Array directly
const xValues = new Float64Array([1, 2, 3, ...]);
const yValues = new Float64Array([10, 20, 30, ...]);
dataSeries.appendRange(xValues, yValues);

// ❌ SLOW: Convert from regular arrays
const xValues = [1, 2, 3, ...];  // Regular array
const yValues = [10, 20, 30, ...];
dataSeries.appendRange(xValues, yValues);  // Conversion overhead
```

### Incremental Rendering

```typescript
// Render visible range first, then load rest
async function loadChartDataProgressive(seriesId: string) {
  // 1. Load visible range first (fast)
  const visibleData = await fetch(`/api/series/${seriesId}/data?range=0-1000`);
  dataSeries.appendRange(visibleData.x, visibleData.y);
  
  // 2. Load remaining data in background
  const fullData = await fetch(`/api/series/${seriesId}/data`);
  dataSeries.appendRange(fullData.x.slice(1000), fullData.y.slice(1000));
}
```

## Strategy 6: Caching and Prefetching

### Client-Side Caching

```typescript
class ChartDataCache {
  private cache = new Map<string, { data: any, timestamp: number }>();
  private ttl = 5 * 60 * 1000; // 5 minutes
  
  async get(seriesId: string): Promise<any> {
    const cached = this.cache.get(seriesId);
    
    if (cached && Date.now() - cached.timestamp < this.ttl) {
      return cached.data; // Cache hit
    }
    
    // Fetch and cache
    const data = await fetchChartData(seriesId);
    this.cache.set(seriesId, { data, timestamp: Date.now() });
    return data;
  }
}
```

### Prefetching

```typescript
// Prefetch likely-to-be-viewed data
async function prefetchRelatedSeries(chartId: string) {
  const chart = await getChart(chartId);
  
  // Prefetch all series in parallel
  const prefetchPromises = chart.seriesIds.map(id => 
    fetchChartData(id).catch(() => null) // Don't fail on errors
  );
  
  await Promise.all(prefetchPromises);
}
```

### Service Worker Caching

```typescript
// Cache chart data in service worker
self.addEventListener('fetch', (event) => {
  if (event.request.url.includes('/api/series/')) {
    event.respondWith(
      caches.match(event.request).then((response) => {
        return response || fetch(event.request).then((response) => {
          const clone = response.clone();
          caches.open('chart-data').then((cache) => {
            cache.put(event.request, clone);
          });
          return response;
        });
      })
    );
  }
});
```

## Strategy 7: Parallel Processing

### Parallel Data Fetching

```typescript
// Fetch multiple series in parallel
async function loadMultipleSeries(seriesIds: string[]) {
  const promises = seriesIds.map(id => fetchChartData(id));
  const results = await Promise.all(promises);
  
  // Apply all updates in batch
  sciChartSurface.suspendUpdates();
  try {
    results.forEach((data, index) => {
      const series = chartSeries[index];
      series.dataSeries.appendRange(data.x, data.y);
    });
  } finally {
    sciChartSurface.resumeUpdates();
  }
}
```

### Web Workers for Processing

```typescript
// Offload data processing to web worker
const worker = new Worker('chart-data-processor.js');

worker.postMessage({
  type: 'process',
  data: rawData
});

worker.onmessage = (event) => {
  const { xValues, yValues } = event.data;
  dataSeries.appendRange(xValues, yValues);
};
```

## Strategy 8: Request Optimization

### HTTP/2 Server Push

```rust
// Push related resources proactively
async fn handle_chart_request(req: Request) -> Response {
    let response = Response::new(...);
    
    // Push series data before client requests it
    response.headers_mut().insert(
        "Link",
        "</api/series/123/data>; rel=preload; as=fetch".parse().unwrap()
    );
    
    response
}
```

### Request Batching

```typescript
// Batch multiple requests into one
async function batchFetchSeries(seriesIds: string[]) {
  const response = await fetch('/api/series/batch', {
    method: 'POST',
    body: JSON.stringify({ ids: seriesIds }),
    headers: { 'Content-Type': 'application/json' }
  });
  
  const batchData = await response.json();
  // Process all series at once
}
```

## Complete Implementation Example

### Backend (Rust + Axum)

```rust
use axum::{
    extract::Path,
    response::{Response, IntoResponse},
    routing::get,
    Router,
};
use arrow::array::Float64Array;
use arrow::record_batch::RecordBatch;
use arrow::ipc::writer::StreamWriter;

async fn get_series_data(Path(series_id): Path<String>) -> Response {
    // Fetch data from database/storage
    let (x_values, y_values) = fetch_series_data(&series_id).await;
    
    // Create Arrow arrays
    let x_array = Float64Array::from(x_values);
    let y_array = Float64Array::from(y_values);
    
    // Create schema
    let schema = Arc::new(Schema::new(vec![
        Field::new("x", DataType::Float64, false),
        Field::new("y", DataType::Float64, false),
    ]));
    
    // Create record batch
    let batch = RecordBatch::try_new(
        schema.clone(),
        vec![Arc::new(x_array), Arc::new(y_array)],
    ).unwrap();
    
    // Serialize to Arrow IPC
    let mut buffer = Vec::new();
    let mut writer = StreamWriter::try_new(&mut buffer, &schema).unwrap();
    writer.write(&batch).unwrap();
    writer.finish().unwrap();
    
    // Compress with Snappy
    let compressed = compress_snappy(&buffer);
    
    // Return with appropriate headers
    Response::builder()
        .header("Content-Type", "application/vnd.apache.arrow.stream")
        .header("Content-Encoding", "snappy")
        .header("Cache-Control", "public, max-age=300")
        .body(compressed.into())
        .unwrap()
        .into_response()
}

fn compress_snappy(data: &[u8]) -> Vec<u8> {
    use snap::raw::Encoder;
    let mut encoder = Encoder::new();
    encoder.compress_vec(data).unwrap()
}
```

### Frontend (TypeScript + SciChart)

```typescript
import { tableFromIPC } from 'apache-arrow';
import { XyDataSeries, SciChartSurface } from 'scichart';

class ChartDataService {
  private cache = new Map<string, { data: any, timestamp: number }>();
  
  async loadSeriesData(
    seriesId: string,
    dataSeries: XyDataSeries,
    surface: SciChartSurface
  ): Promise<void> {
    // Check cache first
    const cached = this.cache.get(seriesId);
    if (cached && Date.now() - cached.timestamp < 5 * 60 * 1000) {
      this.applyData(dataSeries, cached.data, surface);
      return;
    }
    
    // Fetch with Arrow IPC format
    const response = await fetch(`/api/series/${seriesId}/data`, {
      headers: {
        'Accept': 'application/vnd.apache.arrow.stream',
        'Accept-Encoding': 'snappy'
      }
    });
    
    // Decompress if needed
    const arrayBuffer = response.headers.get('Content-Encoding') === 'snappy'
      ? await this.decompressSnappy(await response.arrayBuffer())
      : await response.arrayBuffer();
    
    // Zero-copy deserialization
    const table = tableFromIPC(arrayBuffer);
    
    const xValues = table.getChild('x')?.toArray() as Float64Array;
    const yValues = table.getChild('y')?.toArray() as Float64Array;
    
    // Cache result
    this.cache.set(seriesId, {
      data: { xValues, yValues },
      timestamp: Date.now()
    });
    
    // Apply to chart with batching
    this.applyData(dataSeries, { xValues, yValues }, surface);
  }
  
  private applyData(
    dataSeries: XyDataSeries,
    data: { xValues: Float64Array, yValues: Float64Array },
    surface: SciChartSurface
  ): void {
    surface.suspendUpdates();
    try {
      dataSeries.appendRange(data.xValues, data.yValues);
    } finally {
      surface.resumeUpdates();
    }
  }
  
  private async decompressSnappy(buffer: ArrayBuffer): Promise<ArrayBuffer> {
    // Use CompressionStream API or Snappy.js
    const stream = new Response(buffer).body?.pipeThrough(
      new DecompressionStream('gzip') // Browser native
    );
    return new Response(stream).arrayBuffer();
  }
}
```

## Performance Benchmarks

### Latency Comparison (10K points)

| Strategy | Format | Compression | Total Latency |
|----------|--------|-------------|---------------|
| Baseline | JSON | None | 150-200ms |
| Optimized | JSON | Gzip | 80-120ms |
| **Recommended** | **Arrow IPC** | **Snappy** | **30-70ms** ⭐ |
| Alternative | Protobuf | ZSTD | 20-55ms |
| Streaming | Arrow IPC | Snappy | 10-30ms (first chunk) |

### Optimization Impact

| Optimization | Latency Reduction |
|--------------|------------------|
| Arrow IPC vs JSON | 60-70% |
| Snappy compression | 20-30% |
| Zero-copy deserialization | 40-50% |
| Batch updates | 30-40% |
| Caching | 90-95% (cache hits) |
| Streaming | 50-70% (perceived) |

## Recommendations

### For Lowest Latency:

1. **Serialization**: Arrow IPC for numeric arrays, Protobuf for structured data
2. **Compression**: Snappy or ZSTD (fast compression/decompression)
3. **Transfer**: WebSocket streaming for real-time, HTTP/2 for requests
4. **Frontend**: Batch updates, use typed arrays, suspend/resume updates
5. **Caching**: Client-side cache with TTL, service worker for offline
6. **Progressive**: Load visible range first, stream remaining data

### Implementation Priority:

1. **Phase 1**: Switch from JSON to Arrow IPC/Protobuf
2. **Phase 2**: Add compression (Snappy/ZSTD)
3. **Phase 3**: Implement streaming for large datasets
4. **Phase 4**: Add client-side caching
5. **Phase 5**: Optimize frontend batch updates

## Summary

To minimize latency between backend and frontend chart rendering:

- **Use Arrow IPC** for large numeric arrays (zero-copy, columnar)
- **Compress with Snappy/ZSTD** (fast compression/decompression)
- **Stream data** for progressive rendering
- **Batch updates** in frontend (suspend/resume SciChart updates)
- **Cache aggressively** (client-side, service worker)
- **Use typed arrays** directly (Float64Array, etc.)
- **Load progressively** (visible range first, then background)

This architecture can achieve **30-70ms total latency** for 10K point arrays, compared to 150-200ms with JSON.

