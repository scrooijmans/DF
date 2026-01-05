# How Datashader Determines Aggregation Level Based on Screen Resolution

## Overview

Datashader determines the appropriate level of data aggregation based on screen resolution through its Canvas-based pixel grid system. The aggregation level is directly controlled by the canvas dimensions (`plot_width` and `plot_height`), which define a fixed pixel grid onto which all data points are binned.

## 1. Canvas-Based Pixel Grid Definition

The aggregation level is determined by the `Canvas` dimensions:

```python
canvas = ds.Canvas(plot_width=600, plot_height=600,
                   x_range=(x_min, x_max), y_range=(y_min, y_max))
```

**Key Points:**

- `plot_width` and `plot_height` define the pixel grid (e.g., 600×600 = 360,000 bins)
- `x_range` and `y_range` define the data space mapped to that grid
- Each pixel bin represents a rectangular region in data space

## 2. Pixel-Based Binning Process

The binning process maps data coordinates to pixel indices:

1. **Coordinate Transformation**: For each point `(x, y)`, Datashader computes:

   - `pixel_x = int((x - x_min) / (x_max - x_min) * plot_width)`
   - `pixel_y = int((y - y_min) / (y_max - y_min) * plot_height)`

2. **Pixel Bin Assignment**: The point is assigned to bin `(pixel_x, pixel_y)`

3. **Multiple Points per Bin**: Multiple data points can map to the same bin

**Example:**

```python
canvas = ds.Canvas(plot_width=300, plot_height=300)
agg = canvas.points(df, 'x', 'y', agg=ds.count())
```

This creates a 300×300 grid. Points are assigned to bins, and the aggregation function (e.g., `count()`) combines values within each bin.

## 3. Aggregation Computation

Aggregates are computed incrementally in a single pass:

**Incremental Processing:**

- Each data point is processed once
- Points update their pixel bin's aggregate value
- No need to store all raw points in memory

**Supported Aggregations:**

- `ds.count()`: Counts points per bin
- `ds.sum(column)`: Sums a column per bin
- `ds.mean(column)`: Mean per bin
- `ds.min(column)`, `ds.max(column)`: Min/max per bin
- `ds.std(column)`: Standard deviation per bin
- `ds.any()`: Presence flag per bin
- `ds.summary(...)`: Multiple aggregates in one pass

**Example with Different Aggregations:**

```python
# Count aggregation
agg_count = canvas.points(df, 'x', 'y', agg=ds.count())

# Mean aggregation
agg_mean = canvas.points(df, 'x', 'y', agg=ds.mean('value'))

# Multiple aggregations
agg_multi = canvas.points(df, 'x', 'y',
                         agg=ds.summary(count=ds.count(),
                                       mean_val=ds.mean('value')))
```

## 4. How This Avoids Rendering All Raw Points

**Fixed Output Size:**

- The result is always `plot_width × plot_height` pixels
- 1 billion points on a 600×600 canvas = 360,000 aggregate values
- Memory usage is constant regardless of input size

**One Value Per Pixel:**

- Each bin produces one aggregate value
- Multiple points in the same bin are combined
- No individual point rendering needed

**Single-Pass Processing:**

- Points are processed once
- Each point updates its bin's aggregate
- No need to store all points

**Memory Efficiency:**

- Only the aggregate array is stored
- Raw points can be discarded after processing
- Enables handling of datasets larger than memory

**Example with Large Datasets:**

```python
# Can handle 100 million points efficiently
distb = gaussians(specs=[...], num=20000000)
canvas = ds.Canvas(plot_width=850, plot_height=500)
agg = canvas.points(distb, 'x', 'y', agg=ds.count())
img = ds.tf.shade(agg, cmap=colorcet.fire)
```

Even with 20 million points, only 425,000 aggregate values (850×500) are computed and stored.

## 5. Resolution-Dependent Aggregation

**Higher Resolution (More Pixels):**

- Finer binning
- More detail visible
- More computation required

**Lower Resolution (Fewer Pixels):**

- Coarser binning
- Less detail
- Faster computation

**Example Showing Resolution Impact:**

```python
# Low resolution - coarse binning
canvas_low = ds.Canvas(plot_width=150, plot_height=150)
agg_low = canvas_low.points(df, 'x', 'y', agg=ds.count())

# High resolution - fine binning
canvas_high = ds.Canvas(plot_width=1200, plot_height=1200)
agg_high = canvas_high.points(df, 'x', 'y', agg=ds.count())
```

The same dataset produces different aggregation levels: 22,500 bins vs. 1,440,000 bins.

## 6. Downsampling for Raster Data

For raster/array data, downsampling uses aggregation when the canvas is smaller than the source:

```python
canvas = ds.Canvas(plot_width=150, plot_height=150)
# When downsampling, multiple grid cells map to one pixel
agg = canvas.raster(large_array, agg=ds.mean())  # Default: mean
# Or use other aggregations:
agg_min = canvas.raster(large_array, agg=ds.min())
agg_max = canvas.raster(large_array, agg=ds.max())
agg_std = canvas.raster(large_array, agg=ds.std())
```

## Summary

Datashader uses screen resolution (canvas dimensions) to determine the aggregation level. Each pixel is a bin, and aggregates are computed incrementally in a single pass. This yields a fixed-size output regardless of input size, avoiding the need to render all raw points and enabling efficient visualization of very large datasets.
