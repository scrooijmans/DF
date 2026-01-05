# How Datashader Separates Data Aggregation from Rendering

## Overview

Datashader separates data aggregation from rendering through a two-stage pipeline architecture. This separation is fundamental to its scalability and enables efficient visualization of datasets with millions or billions of points.

## 1. The Two-Stage Pipeline Architecture

Datashader uses a clear separation between two stages:

**Stage 1: Data Aggregation** → Produces an `xarray.DataArray`
**Stage 2: Visual Rendering** → Converts the aggregate to an image

These stages are completely independent and can be executed separately:

```python
# Stage 1: Aggregation (data processing)
canvas = ds.Canvas(plot_width=850, plot_height=500)
agg = canvas.points(df, 'longitude', 'latitude', agg=ds.count())
# Result: xarray.DataArray with aggregated values

# Stage 2: Rendering (visualization)
img = ds.tf.shade(agg, cmap=colorcet.fire, how='log')
# Result: Image object ready for display
```

## 2. Stage 1: Data Aggregation

The aggregation stage produces a structured aggregate array:

**Output Format:**

- **Type**: `xarray.DataArray`
- **Dimensions**: `(y, x)` matching canvas pixel dimensions
- **Coordinates**: Data space coordinates for each pixel
- **Values**: Aggregated values (counts, means, sums, etc.)

**Example showing the aggregate structure:**

```python
canvas = ds.Canvas(plot_width=300, plot_height=300)
agg = canvas.points(df, 'x', 'y', agg=ds.count())
# agg is an xarray.DataArray with shape (300, 300)
# Each element represents the count of points in that pixel bin
```

**Key Characteristics:**

- The aggregate is purely data - no colors or visual properties
- Can be inspected and analyzed
- Can be transformed with NumPy/xarray operations
- Can be reused for different visualizations
- Can be stored for later rendering

## 3. Stage 2: Visual Rendering via Transfer Functions

Transfer functions convert aggregates into images. The primary function is `tf.shade()`:

```python
import datashader.transfer_functions as tf

# Basic shading
img = tf.shade(agg, cmap=['lightblue', 'darkblue'])

# With different colormaps and transformations
img = tf.shade(agg, cmap=colorcet.fire, how='log')
```

**How `tf.shade()` Works:**

1. **Value Normalization**: Maps aggregate values to a 0-1 range using methods like:

   - `'linear'`: Linear scaling
   - `'log'`: Logarithmic scaling
   - `'eq_hist'`: Histogram equalization
   - `'cbrt'`: Cube root scaling
   - Custom callable functions

2. **Color Mapping**: Maps normalized values to colors using:

   - Colormaps (sequential, diverging, categorical)
   - Color keys for categorical data
   - Custom color lists

3. **Alpha Channel**: Controls transparency based on data presence or value

4. **Output**: Produces an RGBA image object

**Example with Different Rendering Options:**

```python
# Same aggregate, different visualizations
agg = canvas.points(df, 'x', 'y', agg=ds.count())

tf.Images(
    tf.shade(agg, how='linear', name="Linear scaling"),
    tf.shade(agg, how='log', name="Logarithmic scaling"),
    tf.shade(agg, how='eq_hist', name="Histogram equalization"),
    tf.shade(agg, cmap=["darkred", "yellow"], name="Custom colors")
)
```

## 4. Separation Benefits: Intermediate Processing

The separation allows operations between aggregation and rendering:

**Transform the Aggregate:**

```python
agg = canvas.points(df, 'x', 'y', agg=ds.count())

# Apply mathematical transformations
agg_squared = np.power(agg, 2)
agg_filtered = agg.where(agg >= np.percentile(agg, 99))

# Then render the transformed aggregate
tf.Images(
    tf.shade(agg, name="Original"),
    tf.shade(agg_squared, name="Squared"),
    tf.shade(agg_filtered, name="99th Percentile")
)
```

**Multiple Aggregations, Single Rendering:**

```python
# Compute multiple aggregates
agg_count = canvas.points(df, 'x', 'y', agg=ds.count())
agg_mean = canvas.points(df, 'x', 'y', agg=ds.mean('value'))
agg_sum = canvas.points(df, 'x', 'y', agg=ds.sum('value'))

# Render each with appropriate colormaps
tf.Images(
    tf.shade(agg_count, name="Count"),
    tf.shade(agg_mean, name="Mean"),
    tf.shade(agg_sum, name="Sum")
)
```

**Categorical Aggregation with Different Renderings:**

```python
# Single aggregation with categories
agg_cat = canvas.points(df, 'x', 'y', ds.by('category', ds.count()))

# Different visualizations of the same aggregate
color_key = dict(cat1='blue', cat2='green', cat3='red')
tf.Images(
    tf.shade(agg_cat, name="Default colors"),
    tf.shade(agg_cat, color_key=color_key, name="Custom colors")
)
```

## 5. How This Design Enables Scalability

### Fixed Memory Footprint

The aggregation stage produces a fixed-size output regardless of input size:

```python
# 1 million points → 360,000 aggregate values (600×600 canvas)
canvas = ds.Canvas(plot_width=600, plot_height=600)
agg_1M = canvas.points(df_1M, 'x', 'y', agg=ds.count())

# 100 million points → same 360,000 aggregate values
agg_100M = canvas.points(df_100M, 'x', 'y', agg=ds.count())

# 1 billion points → still 360,000 aggregate values
agg_1B = canvas.points(df_1B, 'x', 'y', agg=ds.count())
```

The memory required is determined by canvas resolution, not data size.

### Single-Pass Processing

Aggregation is incremental and single-pass:

- Each data point is processed once
- Points update their pixel bin's aggregate value
- No need to store all raw points in memory

**Example with 100 million points:**

```python
# Can handle 100 million points efficiently
distb = gaussians(specs=[...], num=20000000)  # 20 million points
canvas = ds.Canvas(plot_width=850, plot_height=500)
agg = canvas.points(distb, 'x', 'y', agg=ds.count())
# Only 425,000 values stored (850×500), not 20 million points
```

### Rendering is Independent of Data Size

Rendering operates on the fixed-size aggregate:

- **Input**: `xarray.DataArray` with shape `(height, width)`
- **Processing**: Color mapping and normalization
- **Output**: Image with same dimensions

The rendering time depends on canvas resolution, not the original data size:

```python
# Rendering 1 million points: ~same time as rendering 1 billion points
# because both produce the same-sized aggregate
img = tf.shade(agg, cmap=colorcet.fire, how='log')
```

### Reusability and Efficiency

The same aggregate can be rendered multiple times with different visualizations:

```python
# Compute aggregate once (expensive operation)
agg = canvas.points(large_df, 'x', 'y', agg=ds.count())

# Render multiple times with different styles (cheap operation)
img1 = tf.shade(agg, cmap=colorcet.fire, how='log')
img2 = tf.shade(agg, cmap=colorcet.viridis, how='linear')
img3 = tf.shade(agg, cmap=colorcet.coolwarm, how='eq_hist')
```

## 6. Complete Pipeline Example

Demonstrating the full separation:

```python
import datashader as ds
import datashader.transfer_functions as tf
import pandas as pd
import colorcet

# Load large dataset
df = pd.read_csv('census.csv')  # Millions of points

# STAGE 1: AGGREGATION (data processing)
canvas = ds.Canvas(plot_width=850, plot_height=500)
agg = canvas.points(df, 'longitude', 'latitude', agg=ds.count())
# Result: xarray.DataArray(850, 500) - fixed size regardless of df size

# Optional: Transform aggregate
agg_log = np.log1p(agg)  # Apply log transformation

# STAGE 2: RENDERING (visualization)
img = tf.shade(agg_log, cmap=colorcet.fire, how='linear')
# Result: Image object ready for display

# Alternative renderings of the same aggregate
img2 = tf.shade(agg, cmap=colorcet.viridis, how='eq_hist')
img3 = tf.shade(agg, cmap=['lightblue', 'darkblue'], how='log')
```

## 7. Additional Transfer Functions

Beyond `tf.shade()`, other functions operate on aggregates or images:

**Spreading (for sparse data):**

```python
# Apply spreading to aggregate (better than RGB spreading)
agg_spread = tf.spread(agg, px=3)
img = tf.shade(agg_spread, name="Aggregate spreading")
```

**Dynamic Spreading:**

```python
# Automatically adjusts spreading based on data density
img = tf.dynspread(tf.shade(agg), threshold=0.5)
```

## Summary

Datashader separates aggregation from rendering through:

1. **Aggregation Stage**: Processes raw data into a fixed-size `xarray.DataArray` aggregate
2. **Rendering Stage**: Converts the aggregate to an image using transfer functions

This design enables scalability because:

- Memory usage is bounded by canvas resolution, not data size
- Aggregation is single-pass and incremental
- Rendering operates on fixed-size aggregates
- Aggregates can be reused for multiple visualizations
- The pipeline can handle millions or billions of points efficiently

The separation allows data processing and visualization to be optimized independently, making it practical to visualize datasets that would be impossible to render directly.
