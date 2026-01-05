# SciChart.js Data Formats and Series Types

This document explains the data formats required for SciChart.js charts and the types of series that can be rendered.

## Data Formats

SciChart.js uses different data series types depending on the chart type. Each data series type has a specific structure for storing and presenting data.

### 1. XY Data Format (`XyDataSeries`)

Used for basic line, scatter, column, mountain, and impulse series.

**Data Structure:**
- **X values**: Array of numbers (x-axis values)
- **Y values**: Array of numbers (y-axis values)
- Arrays must have the same length

**Example:**
```javascript
const { XyDataSeries } = SciChart;

// Create data series
const xyDataSeries = new XyDataSeries(wasmContext);

// Append single point
xyDataSeries.append(1, 10);  // x=1, y=10
xyDataSeries.append(2, 20);  // x=2, y=20

// Append multiple points (faster for large datasets)
const xValues = [3, 4, 5, 6, 7];
const yValues = [30, 40, 50, 60, 70];
xyDataSeries.appendRange(xValues, yValues);

// Or using Float64Array for better performance
const xArray = new Float64Array([8, 9, 10]);
const yArray = new Float64Array([80, 90, 100]);
xyDataSeries.appendRange(xArray, yArray);
```

**Series Types Using XY Data:**
- `LineSeries` / `FastLineRenderableSeries`
- `ScatterSeries` / `XyScatterRenderableSeries`
- `ColumnSeries` / `FastColumnRenderableSeries`
- `MountainSeries` / `FastMountainRenderableSeries`
- `ImpulseSeries` / `FastImpulseRenderableSeries`
- `SplineLineSeries` / `SplineLineRenderableSeries`
- `SplineMountainSeries` / `SplineMountainRenderableSeries`
- `TriangleSeries` / `FastTriangleRenderableSeries`
- `LineSegmentSeries` / `FastLineSegmentRenderableSeries`

### 2. XYY Data Format (`XyyDataSeries`)

Used for band series (upper and lower bounds).

**Data Structure:**
- **X values**: Array of numbers (x-axis values)
- **Y1 values**: Array of numbers (upper bound values)
- **Y2 values**: Array of numbers (lower bound values)
- All arrays must have the same length

**Example:**
```javascript
const { XyyDataSeries } = SciChart;

const xyyDataSeries = new XyyDataSeries(wasmContext);

// Append single point
xyyDataSeries.append(1, 15, 5);  // x=1, y1=15 (upper), y2=5 (lower)

// Append multiple points
const xValues = [2, 3, 4, 5];
const y1Values = [20, 25, 30, 35];  // Upper bounds
const y2Values = [10, 15, 20, 25];  // Lower bounds
xyyDataSeries.appendRange(xValues, y1Values, y2Values);
```

**Series Types Using XYY Data:**
- `BandSeries` / `FastBandRenderableSeries`
- `SplineBandSeries` / `SplineBandRenderableSeries`
- `PolarBandSeries` / `PolarBandRenderableSeries`

### 3. XYZ Data Format (`XyzDataSeries`)

Used for bubble series where the third dimension (Z) represents bubble size.

**Data Structure:**
- **X values**: Array of numbers (x-axis values)
- **Y values**: Array of numbers (y-axis values)
- **Z values**: Array of numbers (bubble size values)
- All arrays must have the same length

**Example:**
```javascript
const { XyzDataSeries } = SciChart;

const xyzDataSeries = new XyzDataSeries(wasmContext);

// Append single point
xyzDataSeries.append(1, 10, 5);  // x=1, y=10, z=5 (size)

// Append multiple points
const xValues = [2, 3, 4];
const yValues = [20, 30, 40];
const zValues = [10, 15, 20];  // Bubble sizes
xyzDataSeries.appendRange(xValues, yValues, zValues);
```

**Series Types Using XYZ Data:**
- `BubbleSeries` / `FastBubbleRenderableSeries`

### 4. OHLC Data Format (`OhlcDataSeries`)

Used for financial candlestick and OHLC charts.

**Data Structure:**
- **X values**: Array of numbers (typically timestamps or indices)
- **Open values**: Array of numbers (opening prices)
- **High values**: Array of numbers (highest prices)
- **Low values**: Array of numbers (lowest prices)
- **Close values**: Array of numbers (closing prices)
- All arrays must have the same length

**Example:**
```javascript
const { OhlcDataSeries } = SciChart;

const ohlcDataSeries = new OhlcDataSeries(wasmContext, {
    xValues: [1, 2, 3, 4, 5],
    openValues: [100, 105, 110, 108, 112],
    highValues: [102, 107, 112, 110, 115],
    lowValues: [99, 104, 109, 107, 111],
    closeValues: [101, 106, 111, 109, 114]
});

// Or append programmatically
ohlcDataSeries.append(6, 115, 118, 114, 117);  // x, open, high, low, close
```

**Series Types Using OHLC Data:**
- `CandlestickSeries` / `FastCandlestickRenderableSeries`
- `OhlcSeries` / `FastOhlcRenderableSeries`

### 5. HLC Data Format (`HlcDataSeries`)

Used for error bars (High-Low-Close).

**Data Structure:**
- **X values**: Array of numbers
- **High values**: Array of numbers (upper error bounds)
- **Low values**: Array of numbers (lower error bounds)
- **Close values**: Array of numbers (center values)
- All arrays must have the same length

**Example:**
```javascript
const { HlcDataSeries } = SciChart;

const hlcDataSeries = new HlcDataSeries(wasmContext, {
    xValues: [1, 2, 3],
    highValues: [12, 15, 18],   // Upper error
    lowValues: [8, 10, 12],     // Lower error
    closeValues: [10, 12.5, 15] // Center value
});
```

**Series Types Using HLC Data:**
- `ErrorBarsSeries` / `FastErrorBarsRenderableSeries`

### 6. Uniform Heatmap Data Format (`UniformHeatmapDataSeries`)

Used for heatmaps with uniform grid spacing.

**Data Structure:**
- **zValues**: 2D array `number[][]` where `zValues[height][width]`
- **xStart**: Starting X value
- **xStep**: Step size for X axis
- **yStart**: Starting Y value
- **yStep**: Step size for Y axis

**Example:**
```javascript
const { UniformHeatmapDataSeries } = SciChart;

// Create 2D array for z-values
const heatmapWidth = 7;
const heatmapHeight = 4;
const zValues = Array.from(Array(heatmapHeight));
zValues.forEach((row, index, collection) => {
    collection[index] = Array.from(Array(heatmapWidth));
});

// Fill with data (zValues[y][x])
for (let x = 0; x < heatmapWidth; x++) {
    for (let y = 0; y < heatmapHeight; y++) {
        zValues[y][x] = 3.5 * ((heatmapHeight - y) * (heatmapWidth - x));
    }
}

const heatmapDataSeries = new UniformHeatmapDataSeries(wasmContext, {
    zValues,
    xStart: 0,
    xStep: 1,
    yStart: 0,
    yStep: 1
});
```

**Series Types Using Uniform Heatmap Data:**
- `UniformHeatmapSeries` / `UniformHeatmapRenderableSeries`
- `UniformContoursSeries` / `UniformContoursRenderableSeries`
- `PolarUniformHeatmapSeries` / `PolarUniformHeatmapRenderableSeries`

### 7. Non-Uniform Heatmap Data Format (`NonUniformHeatmapDataSeries`)

Used for heatmaps with non-uniform grid spacing.

**Data Structure:**
- **zValues**: 2D array `number[][]`
- **xCellOffsets**: Array of X cell boundary positions
- **yCellOffsets**: Array of Y cell boundary positions

**Example:**
```javascript
const { NonUniformHeatmapDataSeries } = SciChart;

const zValues = [
    [1, 2, 3, 4],
    [5, 6, 7, 8],
    [9, 10, 11, 12]
];

const xCellOffsets = [0, 10, 20, 26, 36];  // 4 cells with varying widths
const yCellOffsets = [100, 250, 390, 410]; // 3 cells with varying heights

const heatmapDataSeries = new NonUniformHeatmapDataSeries(wasmContext, {
    zValues,
    xCellOffsets,
    yCellOffsets
});
```

**Series Types Using Non-Uniform Heatmap Data:**
- `NonUniformHeatmapSeries` / `NonUniformHeatmapRenderableSeries`

### 8. XY Text Data Format (`XyTextDataSeries`)

Used for text annotations at specific points.

**Data Structure:**
- **X values**: Array of numbers
- **Y values**: Array of numbers
- **Text values**: Array of strings

**Example:**
```javascript
const { XyTextDataSeries } = SciChart;

const textDataSeries = new XyTextDataSeries(wasmContext, {
    xValues: [1, 2, 3, 4],
    yValues: [10, 20, 30, 40],
    textValues: ["Point 1", "Point 2", "Point 3", "Point 4"]
});
```

**Series Types Using XY Text Data:**
- `TextSeries` / `TextRenderableSeries`
- `PolarTextSeries` / `PolarTextRenderableSeries`

### 9. Box Plot Data Format (`BoxPlotDataSeries`)

Used for box plot charts showing statistical distributions.

**Data Structure:**
- **X values**: Array of numbers
- **Box plot data**: Array of objects with `{ min, lowerQuartile, median, upperQuartile, max }`

**Example:**
```javascript
const { BoxPlotDataSeries } = SciChart;

const boxPlotData = [
    { min: 5, lowerQuartile: 10, median: 15, upperQuartile: 20, max: 25 },
    { min: 8, lowerQuartile: 12, median: 18, upperQuartile: 22, max: 28 }
];

const boxPlotDataSeries = new BoxPlotDataSeries(wasmContext, {
    xValues: [1, 2],
    boxPlotData
});
```

**Series Types Using Box Plot Data:**
- `BoxPlotSeries` / `FastBoxPlotRenderableSeries`

### 10. Extended Data Formats

Some series support extended data formats:

**XYX Data** (`XyxSeriesData`): X, Y, and X1 (width) values
- Used by: `RectangleSeries`, `PolarColumnSeries`

**XYXY Data** (`XyxySeriesData`): X, Y, X1, Y1 values
- Used by: `RectangleSeries`, `TriangleSeries`, `LineSegmentSeries`, `PolarColumnSeries`, `PolarTriangleSeries`

## Series Types

SciChart.js supports a wide variety of series types. Here's a comprehensive list:

### Basic Series Types

1. **Line Series**
   - `LineSeries` / `FastLineRenderableSeries`
   - `SplineLineSeries` / `SplineLineRenderableSeries`
   - `PolarLineSeries` / `PolarLineRenderableSeries`
   - Data: `XyDataSeries`

2. **Mountain Series** (Area charts)
   - `MountainSeries` / `FastMountainRenderableSeries`
   - `SplineMountainSeries` / `SplineMountainRenderableSeries`
   - `PolarMountainSeries` / `PolarMountainRenderableSeries`
   - Data: `XyDataSeries`

3. **Column Series** (Bar charts)
   - `ColumnSeries` / `FastColumnRenderableSeries`
   - `PolarColumnSeries` / `PolarColumnRenderableSeries`
   - Data: `XyDataSeries` or `XyxSeriesData` or `XyxySeriesData`

4. **Scatter Series**
   - `ScatterSeries` / `XyScatterRenderableSeries`
   - `PolarScatterSeries` / `PolarScatterRenderableSeries`
   - Data: `XyDataSeries`

5. **Bubble Series**
   - `BubbleSeries` / `FastBubbleRenderableSeries`
   - Data: `XyzDataSeries`

6. **Band Series** (Upper/lower bounds)
   - `BandSeries` / `FastBandRenderableSeries`
   - `SplineBandSeries` / `SplineBandRenderableSeries`
   - `PolarBandSeries` / `PolarBandRenderableSeries`
   - Data: `XyyDataSeries`

### Financial Series Types

7. **Candlestick Series**
   - `CandlestickSeries` / `FastCandlestickRenderableSeries`
   - Data: `OhlcDataSeries`

8. **OHLC Series**
   - `OhlcSeries` / `FastOhlcRenderableSeries`
   - Data: `OhlcDataSeries`

9. **Error Bars Series**
   - `ErrorBarsSeries` / `FastErrorBarsRenderableSeries`
   - Data: `HlcDataSeries`

### Specialized Series Types

10. **Impulse Series** (Vertical lines)
    - `ImpulseSeries` / `FastImpulseRenderableSeries`
    - Data: `XyDataSeries`

11. **Text Series**
    - `TextSeries` / `TextRenderableSeries`
    - `PolarTextSeries` / `PolarTextRenderableSeries`
    - Data: `XyTextDataSeries`

12. **Rectangle Series**
    - `RectangleSeries` / `FastRectangleRenderableSeries`
    - Data: `XyxSeriesData` or `XyxySeriesData`

13. **Triangle Series**
    - `TriangleSeries` / `FastTriangleRenderableSeries`
    - `PolarTriangleSeries` / `PolarTriangleRenderableSeries`
    - Data: `XyDataSeries` or `XyxySeriesData`

14. **Line Segment Series**
    - `LineSegmentSeries` / `FastLineSegmentRenderableSeries`
    - Data: `XyDataSeries` or `XyxySeriesData`

15. **Box Plot Series**
    - `BoxPlotSeries` / `FastBoxPlotRenderableSeries`
    - Data: `BoxPlotDataSeries`

### Heatmap Series Types

16. **Uniform Heatmap Series**
    - `UniformHeatmapSeries` / `UniformHeatmapRenderableSeries`
    - `PolarUniformHeatmapSeries` / `PolarUniformHeatmapRenderableSeries`
    - Data: `UniformHeatmapDataSeries`

17. **Non-Uniform Heatmap Series**
    - `NonUniformHeatmapSeries` / `NonUniformHeatmapRenderableSeries`
    - Data: `NonUniformHeatmapDataSeries`

18. **Contours Series**
    - `UniformContoursSeries` / `UniformContoursRenderableSeries`
    - Data: `UniformHeatmapDataSeries`

### Stacked Series Types

19. **Stacked Column Series**
    - `StackedColumnSeries` / `StackedColumnRenderableSeries`
    - `PolarStackedColumnSeries` / `PolarStackedColumnRenderableSeries`
    - Data: `XyDataSeries` (multiple series)

20. **Stacked Mountain Series**
    - `StackedMountainSeries` / `StackedMountainRenderableSeries`
    - `PolarStackedMountainSeries` / `PolarStackedMountainRenderableSeries`
    - Data: `XyDataSeries` (multiple series)

21. **Stacked Collections**
    - `StackedColumnCollection` / `StackedColumnCollection`
    - `PolarStackedColumnCollection` / `PolarStackedColumnCollection`
    - `StackedMountainCollection` / `StackedMountainCollection`
    - `PolarStackedMountainCollection` / `PolarStackedMountainCollection`
    - Data: Array of series definitions

22. **Smooth Stacked Mountain Series**
    - `SmoothStackedMountainSeries` / `SmoothStackedMountainRenderableSeries`
    - Data: `XyDataSeries`

## Common Operations

### Appending Data

```javascript
// Single point
dataSeries.append(x, y);

// Multiple points (recommended for performance)
dataSeries.appendRange(xValues, yValues);

// Using Float64Array for better performance
const xArray = new Float64Array([1, 2, 3]);
const yArray = new Float64Array([10, 20, 30]);
dataSeries.appendRange(xArray, yArray);
```

### Inserting Data

```javascript
// Insert at specific index
dataSeries.insert(index, x, y);

// Insert range
dataSeries.insertRange(index, xValues, yValues);
```

### Updating Data

```javascript
// Update single point
dataSeries.update(index, newY);

// Update range
dataSeries.updateN(startIndex, xValues, yValues);
```

### Removing Data

```javascript
// Remove single point
dataSeries.removeAt(index);

// Remove range
dataSeries.removeRange(startIndex, count);
```

### Clearing Data

```javascript
// Clear all data (keeps capacity)
dataSeries.clear();

// Delete and free memory
dataSeries.delete();
```

## Performance Tips

1. **Use `appendRange` instead of multiple `append` calls** for better performance
2. **Reuse Float64Array buffers** when appending data frequently
3. **Set initial capacity** for large datasets:
   ```javascript
   const dataSeries = new XyDataSeries(wasmContext, { capacity: 100000 });
   ```
4. **Use `clear()` instead of `delete()`** if you plan to reuse the data series
5. **Batch updates** when modifying multiple points

## Complete Example

```javascript
const { 
    SciChartSurface, 
    NumericAxis, 
    FastLineRenderableSeries, 
    XyDataSeries,
    SciChartJsNavyTheme 
} = SciChart;

// Create chart surface
const { wasmContext, sciChartSurface } = await SciChartSurface.create("chart", {
    theme: new SciChartJsNavyTheme()
});

// Add axes
sciChartSurface.xAxes.add(new NumericAxis(wasmContext));
sciChartSurface.yAxes.add(new NumericAxis(wasmContext));

// Create data series
const dataSeries = new XyDataSeries(wasmContext, { capacity: 1000 });

// Populate data
const xValues = [];
const yValues = [];
for (let i = 0; i < 100; i++) {
    xValues.push(i);
    yValues.push(Math.sin(i * 0.1) * 50 + 50);
}
dataSeries.appendRange(xValues, yValues);

// Create renderable series
const lineSeries = new FastLineRenderableSeries(wasmContext, {
    dataSeries,
    stroke: "#FF6600",
    strokeThickness: 2
});

// Add to chart
sciChartSurface.renderableSeries.add(lineSeries);
```

## Summary

- **XY Data**: Most common format for line, scatter, column, mountain series
- **XYY Data**: For band series with upper/lower bounds
- **XYZ Data**: For bubble series with size dimension
- **OHLC Data**: For financial candlestick/OHLC charts
- **HLC Data**: For error bars
- **Heatmap Data**: 2D arrays for heatmap visualizations
- **Text Data**: For text annotations
- **Box Plot Data**: For statistical box plots

Each series type requires its corresponding data series type, and SciChart.js provides optimized methods for appending, inserting, updating, and removing data points.

