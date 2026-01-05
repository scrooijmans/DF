# How Datashader Handles Dynamic Changes (Zooming, Panning, Filtering)

## Overview

Datashader handles dynamic changes such as zooming, panning, and filtering efficiently by processing only the necessary subsets of data. This is achieved through range-based filtering, spatial partitioning, and integration with interactive frameworks like HoloViews.

## 1. Zooming and Panning via Canvas Range Parameters

Datashader uses `x_range` and `y_range` parameters on the Canvas to control the viewport. Changing these ranges automatically limits which data is aggregated.

### Basic Zooming

```python
# Full view
canvas_full = ds.Canvas(plot_width=600, plot_height=600)
agg_full = canvas_full.points(df, 'x', 'y', agg=ds.count())

# Zoomed view - only processes data in the specified range
canvas_zoom = ds.Canvas(plot_width=600, plot_height=600,
                        x_range=(1.92, 1.95),
                        y_range=(1.92, 1.95))
agg_zoom = canvas_zoom.points(df, 'x', 'y', agg=ds.count())
```

### How It Works

- Only points within `x_range` and `y_range` are considered
- Points outside the range are skipped during aggregation
- The aggregation operates on the filtered subset

### Multiple Zoom Levels

```python
# Different zoom levels reveal different details
zoom_levels = [
    (0.1, 0.8),    # Wide view
    (0.14, 0.4),   # Medium zoom
    (0.15, 0.2)    # High zoom
]

images = [
    tf.shade(ds.Canvas(x_range=r, y_range=r).trimesh(verts, tris, mesh=mesh))
    for r in zoom_levels
]
```

## 2. Efficient Data Filtering

Datashader filters data before aggregation, so only relevant points are processed.

### Manual Filtering

```python
# Filter data before aggregation
filtered_df = df[(df['x'] >= x_min) & (df['x'] <= x_max) &
                 (df['y'] >= y_min) & (df['y'] <= y_max)]

canvas = ds.Canvas(plot_width=600, plot_height=600)
agg = canvas.points(filtered_df, 'x', 'y', agg=ds.count())
```

### Automatic Filtering via Canvas Range

```python
# Canvas automatically filters data based on x_range/y_range
canvas = ds.Canvas(plot_width=600, plot_height=600,
                   x_range=(x_min, x_max),
                   y_range=(y_min, y_max))
# Only points within these ranges are processed
agg = canvas.points(df, 'x', 'y', agg=ds.count())
```

### Conditional Filtering

```python
# Filter by data attributes
filtered_df = df[df['category'] == 'A']
agg = canvas.points(filtered_df, 'x', 'y', agg=ds.count())

# Or combine spatial and attribute filtering
filtered = df[(df['x'].between(x_min, x_max)) &
              (df['y'].between(y_min, y_max)) &
              (df['value'] > threshold)]
agg = canvas.points(filtered, 'x', 'y', agg=ds.count())
```

## 3. Interactive Updates with HoloViews

HoloViews integration enables dynamic recomputation on zoom/pan events.

### Basic Interactive Setup

```python
import holoviews as hv
from holoviews.operation.datashader import datashade
hv.extension('bokeh')

# Create interactive plot - automatically recomputes on zoom/pan
points = hv.Points(df, 'x', 'y')
interactive_plot = datashade(points)
# When user zooms/pans, HoloViews calls Datashader with new ranges
```

### How Dynamic Updates Work

1. User zooms/pans in the Bokeh plot
2. HoloViews captures the new viewport range
3. HoloViews calls Datashader with updated `x_range`/`y_range`
4. Datashader filters and aggregates only the visible subset
5. The new aggregate is rendered

### Example with Custom Ranges

```python
# Set initial viewport
datashaded = hv.datashade(points, aggregator=ds.count_cat('cat'))
datashaded = datashaded.redim.range(x=(-5, 5), y=(-5, 5))

# Interactive - updates automatically on user interaction
hv.render(datashaded)
```

## 4. Dask Integration for Partitioned Data

With Dask DataFrames, Datashader uses spatial partitioning to process only relevant partitions.

### Spatial Partitioning

```python
import dask.dataframe as dd

# Convert to Dask DataFrame with spatial partitioning
ddf = dd.from_pandas(sgeodf, npartitions=2)
ddf = ddf.pack_partitions(npartitions=100).persist()

# Datashader automatically uses only relevant partitions
canvas = ds.Canvas(plot_width=700, plot_height=500)
agg = canvas.polygons(ddf, geometry='geometry', agg=ds.mean('cty_pop200'))
```

### Partition Selection via `cx_partitions`

```python
# Custom callback showing partition usage
def compute_partitions(el):
    # Get partitions needed for current viewport
    n = ddf.cx_partitions[slice(*el.range('x')),
                          slice(*el.range('y'))].npartitions
    return el.opts(title=f'Population by county (npartitions: {n})')

# This shows how many partitions are actually processed
out.apply(compute_partitions).opts(frame_width=700, frame_height=500)
```

### Benefits

- Only partitions intersecting the viewport are loaded
- Parallel processing of relevant partitions
- Reduced memory usage
- Faster updates on zoom/pan

## 5. Recomputing Only Necessary Subsets

Datashader avoids reprocessing everything by:

### A. Range-Based Filtering

```python
# Initial view
canvas1 = ds.Canvas(plot_width=600, plot_height=600,
                    x_range=(-10, 10), y_range=(-10, 10))
agg1 = canvas1.points(df, 'x', 'y', agg=ds.count())

# Zoomed view - only processes subset
canvas2 = ds.Canvas(plot_width=600, plot_height=600,
                    x_range=(0, 5), y_range=(0, 5))
agg2 = canvas2.points(df, 'x', 'y', agg=ds.count())
# Only points in [0,5] x [0,5] are processed
```

### B. Incremental Aggregation

- Single-pass processing
- Each point updates its bin once
- No need to store all points

### C. Efficient Data Structures

- Uses spatial indexing when available (e.g., Dask `cx_partitions`)
- Skips partitions outside the viewport
- Processes only visible data

## 6. Complete Interactive Example

Demonstrating the full workflow:

```python
import holoviews as hv
import datashader as ds
from holoviews.operation.datashader import datashade, rasterize
from holoviews.streams import RangeXY
hv.extension('bokeh')

# Large dataset
df = pd.DataFrame({
    'x': np.random.rand(10000000) * 100 - 50,
    'y': np.random.rand(10000000) * 100 - 50,
    'value': np.random.rand(10000000)
})

# Create HoloViews element
points = hv.Points(df, 'x', 'y', vdims='value')

# Interactive datashading - recomputes on zoom/pan
interactive = datashade(points, aggregator=ds.mean('value'))

# With dynamic spreading for sparse data
final = hv.dynspread(interactive, threshold=0.8, max_px=5)

# Display - updates automatically when user zooms/pans
hv.render(final.opts(width=700, height=500))
```

### What Happens on User Interaction

1. User zooms to `x_range=(10, 20), y_range=(10, 20)`
2. HoloViews detects the change
3. Calls `datashade()` with the new range
4. Datashader filters: `df[(df['x'].between(10, 20)) & (df['y'].between(10, 20))]`
5. Aggregates only the filtered subset
6. Renders the new aggregate
7. Updates the display

## 7. Performance Optimizations

### A. Spatial Indexing

- Dask spatial partitions enable partition-level filtering
- Only intersecting partitions are processed

### B. Lazy Evaluation

- With Dask, computation is deferred until needed
- Supports caching of intermediate results

### C. Fixed Output Size

- Output size is determined by canvas resolution, not data size
- Rendering time is constant regardless of filtered data size

### D. Efficient Filtering

- Vectorized operations (NumPy/Pandas)
- Early filtering before aggregation
- Minimal memory overhead

## 8. Filtering Strategies

### Pre-filtering

```python
# Filter before creating canvas
filtered = df[df['category'] == 'A']
agg = canvas.points(filtered, 'x', 'y', agg=ds.count())
```

### Range-based Filtering

```python
# Let canvas handle spatial filtering
canvas = ds.Canvas(x_range=(x_min, x_max), y_range=(y_min, y_max))
agg = canvas.points(df, 'x', 'y', agg=ds.count())
```

### Combined Filtering

```python
# Combine multiple filters
filtered = df[
    (df['x'].between(x_min, x_max)) &
    (df['y'].between(y_min, y_max)) &
    (df['category'].isin(['A', 'B'])) &
    (df['value'] > threshold)
]
agg = canvas.points(filtered, 'x', 'y', agg=ds.count())
```

## Summary

Datashader handles dynamic changes efficiently by:

1. **Range-Based Filtering**: `x_range`/`y_range` limit processing to the viewport
2. **Early Filtering**: Data is filtered before aggregation
3. **Spatial Partitioning**: With Dask, only relevant partitions are processed
4. **Incremental Updates**: Only the changed viewport is recomputed
5. **Interactive Integration**: HoloViews triggers recomputation on user events

This approach ensures:

- Only visible data is processed
- Memory usage scales with viewport size, not total data size
- Fast updates on zoom/pan
- Scalability to very large datasets
- No unnecessary reprocessing of unchanged regions

The separation of aggregation and rendering, combined with range-based filtering and spatial indexing, enables interactive exploration of datasets with millions or billions of points.
