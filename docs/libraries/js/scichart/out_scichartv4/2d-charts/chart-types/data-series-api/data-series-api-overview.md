On this page

# DataSeries API Overview

## What can you do with the DataSeries in SciChart?<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-series-api/data-series-api-overview/#what-can-you-do-with-the-dataseries-in-scichart" class="hash-link" aria-label="Direct link to What can you do with the DataSeries in SciChart?" translate="no" title="Direct link to What can you do with the DataSeries in SciChart?">â€‹</a>

Most chart libraries are geared towards loading a chart with static data and never modifying it.

With SciChart.js, you can:

- Create a chart initially with X, Y data and optional metadata (objects) per-point
- Store values in floating-point 64 bit numbers
- modify the data: appending new data, removing, inserting
- Update values
- Animate changes of data or new values
- Replace all values - like in a spectrum analyzer
- Scroll valuesÂ  - real-time monitoring scenarios
- Sweep values - wrap around as data reaches the right edge of the viewport.

DataSeries allow you to have fine-grained control over the chart data & enable dynamic updates.

## DataSeries Types<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-series-api/data-series-api-overview/#dataseries-types" class="hash-link" aria-label="Direct link to DataSeries Types" translate="no" title="Direct link to DataSeries Types">â€‹</a>

The following DataSeries types exist in SciChart.js. All DataSeries types store memory in WebAssembly and implement theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html" rel="noopener noreferrer" target="_blank">IDeletableðŸ“˜</a> interface. You must callÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html#delete" rel="noopener noreferrer" target="_blank">IDeletable.delete()ðŸ“˜</a>Â when discarding aÂ DataSeries to free memory.

Internally the DataSeriesÂ wrap theÂ <a href="https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Number#:~:text=The%20JavaScript%20Number%20type%20is,arithmetic%20is%20subject%20to%20rounding." rel="noopener noreferrer" target="_blank">JavaScript number type, which is a double-precision 64-bit floating-point number</a> and expect numeric values. You can also store Dates and render strings on chart axis, more on that below.

Here's the content formatted as a two-column Markdown table with headers:

| DataSeries Type | Applicable Series |
|----|----|
| **<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html" rel="noopener noreferrer" target="_blank">XyDataSeriesðŸ“˜</a>** Stores X,Y Data | <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastlinerenderableseries.html" rel="noopener noreferrer" target="_blank">FastLineRenderableSeriesðŸ“˜</a>, <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastmountainrenderableseries.html" rel="noopener noreferrer" target="_blank">FastMountainRenderableSeriesðŸ“˜</a>, <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyscatterrenderableseries.html" rel="noopener noreferrer" target="_blank">XyScatterRenderableSeriesðŸ“˜</a>, <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastcolumnrenderableseries.html" rel="noopener noreferrer" target="_blank">FastColumnRenderableSeriesðŸ“˜</a> <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastlinesegmentrenderableseries.html" rel="noopener noreferrer" target="_blank">FastLineSegmentRenderableSeriesðŸ“˜</a> |
| **<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyydataseries.html" rel="noopener noreferrer" target="_blank">XyyDataSeriesðŸ“˜</a>** Stores X,Y1,Y2 data | **Required for:** FastBandRenderableSeries **Can also apply to:** <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastlinerenderableseries.html" rel="noopener noreferrer" target="_blank">FastLineRenderableSeriesðŸ“˜</a>, <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastmountainrenderableseries.html" rel="noopener noreferrer" target="_blank">FastMountainRenderableSeriesðŸ“˜</a>, <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyscatterrenderableseries.html" rel="noopener noreferrer" target="_blank">XyScatterRenderableSeriesðŸ“˜</a>, <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastcolumnrenderableseries.html" rel="noopener noreferrer" target="_blank">FastColumnRenderableSeriesðŸ“˜</a> (uses only X,Y1) |
| **<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyzdataseries.html" rel="noopener noreferrer" target="_blank">XyzDataSeriesðŸ“˜</a>** Stores X,Y,Z data | **Required for:** FastBubbleRenderableSeries **Can also apply to:** <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastlinerenderableseries.html" rel="noopener noreferrer" target="_blank">FastLineRenderableSeriesðŸ“˜</a>, <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastmountainrenderableseries.html" rel="noopener noreferrer" target="_blank">FastMountainRenderableSeriesðŸ“˜</a>, <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyscatterrenderableseries.html" rel="noopener noreferrer" target="_blank">XyScatterRenderableSeriesðŸ“˜</a>, <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastcolumnrenderableseries.html" rel="noopener noreferrer" target="_blank">FastColumnRenderableSeriesðŸ“˜</a> (uses only X,Y) |
| **<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/ohlcdataseries.html" rel="noopener noreferrer" target="_blank">OhlcDataSeriesðŸ“˜</a>** Stores X, Open, High, Low, Close data | **Required for:** <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastcandlestickrenderableseries.html" rel="noopener noreferrer" target="_blank">FastCandlestickRenderableSeriesðŸ“˜</a> or <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastohlcrenderableseries.html" rel="noopener noreferrer" target="_blank">FastOhlcRenderableSeriesðŸ“˜</a> **Can also apply to:** <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastlinerenderableseries.html" rel="noopener noreferrer" target="_blank">FastLineRenderableSeriesðŸ“˜</a>, <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastmountainrenderableseries.html" rel="noopener noreferrer" target="_blank">FastMountainRenderableSeriesðŸ“˜</a>, <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyscatterrenderableseries.html" rel="noopener noreferrer" target="_blank">XyScatterRenderableSeriesðŸ“˜</a>, <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastcolumnrenderableseries.html" rel="noopener noreferrer" target="_blank">FastColumnRenderableSeriesðŸ“˜</a> (uses only X,Close) |
| **<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/uniformheatmapdataseries.html" rel="noopener noreferrer" target="_blank">UniformHeatmapDataSeriesðŸ“˜</a>** Stores Z-values as 2D array with positions computed from Start/Step values | **Required for:** <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/uniformheatmaprenderableseries.html" rel="noopener noreferrer" target="_blank">UniformHeatmapRenderableSeriesðŸ“˜</a> **Not applicable** to any other RenderableSeries |
| **<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/boxplotdataseries.html" rel="noopener noreferrer" target="_blank">BoxPlotDataSeriesðŸ“˜</a>** Stores X, Maximum, UpperQuartile, Median, LowerQuartile and Minimum data | **Required for:** <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastboxplotrenderableseries.html" rel="noopener noreferrer" target="_blank">FastBoxPlotRenderableSeriesðŸ“˜</a> |
| **<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyndataseries.html" rel="noopener noreferrer" target="_blank">XyNDataSeriesðŸ“˜</a>** Stores X and and an arbitrary number of Y data | **Applicable to:** <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastlinerenderableseries.html" rel="noopener noreferrer" target="_blank">FastLineRenderableSeriesðŸ“˜</a> |
| **<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xytextdataseries.html" rel="noopener noreferrer" target="_blank">XyTextDataSeriesðŸ“˜</a>** Stores X, Y and Text data | **Required for:** <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fasttextrenderableseries.html" rel="noopener noreferrer" target="_blank">FastTextRenderableSeriesðŸ“˜</a> |
| **<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyxdataseries.html" rel="noopener noreferrer" target="_blank">XyxDataSeriesðŸ“˜</a>** Stores X, Y and X1 data | **Applicable to:** <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastrectanglerenderableseries.html" rel="noopener noreferrer" target="_blank">FastRectangleRenderableSeriesðŸ“˜</a> <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarcolumnrenderableseries.html" rel="noopener noreferrer" target="_blank">PolarColumnRenderableSeriesðŸ“˜</a> |
| **<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyxydataseries.html" rel="noopener noreferrer" target="_blank">XyxyDataSeriesðŸ“˜</a>** Stores X, Y, X1 and Y1 data | **Applicable to:** <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastrectanglerenderableseries.html" rel="noopener noreferrer" target="_blank">FastRectangleRenderableSeriesðŸ“˜</a> <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarcolumnrenderableseries.html" rel="noopener noreferrer" target="_blank">PolarColumnRenderableSeriesðŸ“˜</a> |
| **<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/nonuniformheatmapdataseries.html" rel="noopener noreferrer" target="_blank">NonUniformHeatmapDataSeriesðŸ“˜</a>** Stores xStart, xStep, yStart and yStep data | **Applicable to:** <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/nonuniformheatmaprenderableseries.html" rel="noopener noreferrer" target="_blank">NonUniformHeatmapRenderableSeriesðŸ“˜</a> |
| **<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/hlcdataseries.html" rel="noopener noreferrer" target="_blank">HlcDataSeriesðŸ“˜</a>** Stores X, Y, H and L data | **Applicable to:** <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fasterrorbarsrenderableseries.html" rel="noopener noreferrer" target="_blank">FastErrorBarsRenderableSeriesðŸ“˜</a> |

## Creating, Assigning a DataSeries<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-series-api/data-series-api-overview/#creating-assigning-a-dataseries" class="hash-link" aria-label="Direct link to Creating, Assigning a DataSeries" translate="no" title="Direct link to Creating, Assigning a DataSeries">â€‹</a>

A DataSeries can be created with a single line of code, once you have a wasmContext (WebAssembly Context). The WebAssembly Context is created when you call theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#create" rel="noopener noreferrer" target="_blank">SciChartSurface.create()ðŸ“˜</a> function, and the context should be used for elements on that chart only.

- TS
- Builder API (JSON Config)

``` prism-code
// import { SciChartSurface, XyDataSeries, FastLineRenderableSeries ... } from "scichart"
const xValues = [];
const yValues = [];
for (let i = 0; i < 100; i++) {
    xValues.push(i);
    yValues.push(0.2 * Math.sin(i * 0.1) - Math.cos(i * 0.01));
}

// Create a DataSeries
const xyDataSeries = new XyDataSeries(wasmContext, {
    // Optional: pass X,Y values to DataSeries constructor for fast initialization
    // each are Arrays of numbers or Float64Array (typed array for best performance)
    xValues,
    yValues
});

// Create a renderableSeries and assign the dataSeries
const lineSeries = new FastLineRenderableSeries(wasmContext, {
    dataSeries: xyDataSeries
});

// add to the chart
sciChartSurface.renderableSeries.add(lineSeries);
```

``` prism-code
// Demonstrates how to create and assign a dataSeries with SciChart.js using the Builder API
const { chartBuilder, ESeriesType, EThemeProviderType, XyDataSeries } = SciChart;

// or, for npm, import { chartBuilder, ... } from "scichart"
const xValues = [0, 1, 2, 3, 4, 5, 6, 7, 8];
const yValues = [2.5, 3.5, 3.7, 4.0, 5.0, 5.5, 5.0, 4.0, 3.0];

const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    surface: { theme: { type: EThemeProviderType.Dark } },
    series: [
        {
            type: ESeriesType.LineSeries,
            // This section creates a DataSeries with X,Y values
            xyData: {
                xValues,
                yValues
            },
            options: {
                stroke: "#FF6600",
                strokeThickness: 2
            }
        }
    ]
});

// However this is also valid (either xyData, or new XyDataSeries)
// sciChartSurface.renderableSeries.get(0).dataSeries = new XyDataSeries(wasmContext, { xValues, yValues });
```

Once the DataSeries has been created, it can be assigned to a RenderableSeries by setting theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries.html#dataseries" rel="noopener noreferrer" target="_blank">BaseRenderableSeries.dataSeriesðŸ“˜</a> property. This is true for both the classic JavaScript API or the Builder API. More info on this in the section onÂ [RenderableSeries](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/renderable-series-api-overview).

![](out_scichartv4/2d-charts/chart-types/data-series-api/data-series-api-overview/index_media/0f0874621de662630fd5d7f09c89455c4ace26e6.svg)warning

Once you are finished with the DataSeries, don't forget to callÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html#delete" rel="noopener noreferrer" target="_blank">IDeletable.delete()ðŸ“˜</a>. This frees WebAssembly native memory and releases it back to the operating system. For more info see the related articleÂ [Best Practices when Deleting DataSeries](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-series-api/deleting-memory).

## Setting Data Distribution Flags<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-series-api/data-series-api-overview/#setting-data-distribution-flags" class="hash-link" aria-label="Direct link to Setting Data Distribution Flags" translate="no" title="Direct link to Setting Data Distribution Flags">â€‹</a>

For optimal drawing and correct operation, SciChart.js needs to know the distribution of your data, whether sorted in the x-direction and whether the data contains NaN (Not a Number). These flags will be computed automatically, but can be specified for improved performance.

- TS
- Builder API (JSON Config)

``` prism-code
const xyDataSeries = new XyDataSeries(wasmContext, {
    xValues,
    yValues,
    // Data distribution flags are calculated automatically as you update data.
    // Providing them in advance can improve performance for big-data
    // Note: undefined behaviour will occur if these flags are set incorrectly
    dataIsSortedInX: true,
    dataEvenlySpacedInX: true,
    containsNaN: false
});
```

``` prism-code
const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    surface: { theme: { type: EThemeProviderType.Dark } },
    series: [
        {
            type: ESeriesType.LineSeries,
            // This section creates a DataSeries with X,Y values
            // IDataSeriesOptions are valid here
            xyData: {
                xValues: [0, 1, 2, 3, 4, 5, 6, 7, 8],
                yValues: [2.5, 3.5, 3.7, 4.0, 5.0, 5.5, 5.0, 4.0, 3.0],
                dataIsSortedInX: true,
                dataEvenlySpacedInX: true,
                containsNaN: false
            },
            options: {
                stroke: "#FF6600",
                strokeThickness: 2
            }
        }
    ]
});

// However this is also valid (either xyData, or onew XyDataSeries)
// sciChartSurface.renderableSeries.get(0).dataSeries = new XyDataSeries(wasmContext, { xValues, yValues });
```

![](out_scichartv4/2d-charts/chart-types/data-series-api/data-series-api-overview/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

When you don't specify Data Distribution Flags, SciChart.js will compute them automatically as data is updated. This adds a small performance overhead, only noticeable with very big data.

If you specify flags manually, make sure they are correct, and update them as your data updates. If you don't, undefined behaviour can occur.

## Getting the DataSeries XRange and YRange<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-series-api/data-series-api-overview/#getting-the-dataseries-xrange-and-yrange" class="hash-link" aria-label="Direct link to Getting the DataSeries XRange and YRange" translate="no" title="Direct link to Getting the DataSeries XRange and YRange">â€‹</a>

All DataSeries types expose theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#xrange" rel="noopener noreferrer" target="_blank">XRangeðŸ“˜</a> and YRange (viaÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#getwindowedyrange" rel="noopener noreferrer" target="_blank">getWindowedYRangeðŸ“˜</a>) of the underlying series. If you need to know the min and max of the DataSeries in the X or Y direction, then call one of these properties:

Example Title

- TS

``` prism-code
const xyDataSeries = new XyDataSeries(webAssemblyContext);
xyDataSeries.appendRange([1, 2, 3], [10, 20, 30]);

// XRange will choose the first/last value if isSorted=true, else it will iterate over all values
const xRange = xyDataSeries.xRange; // Type NumberRange
console.log(`XRange: ${xRange.toString()}`);
// yRange requires a window of x-values. To get the entire yRange, pass in xRange
const yRange = xyDataSeries.getWindowedYRange(xRange, true, false);
console.log(`YRange: ${yRange.toString()}`);

// Outputs to console
// XRange: NumberRange (1, 3)
// YRange: NumberRange (10, 30)
```

## Storing Date &Â StringÂ values in DataSeries<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-series-api/data-series-api-overview/#storing-date-stringvalues-in-dataseries" class="hash-link" aria-label="Direct link to Storing Date &amp;Â StringÂ values in DataSeries" translate="no" title="Direct link to Storing Date &amp;Â StringÂ values in DataSeries">â€‹</a>

All DataSeries storeÂ 64-bit double precision numeric values. However, if you want to display a date or a string on an axis, you need to do a small conversion first.

### Storing DatesÂ on DataSeriesÂ in SciChart<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-series-api/data-series-api-overview/#storing-dateson-dataseriesin-scichart" class="hash-link" aria-label="Direct link to Storing DatesÂ on DataSeriesÂ in SciChart" translate="no" title="Direct link to Storing DatesÂ on DataSeriesÂ in SciChart">â€‹</a>

DataSeries don't support dates, but you can store values as a unix timestamp and render them as a date on the axis. The process is:

1.  Store Dates as Unix timestamps in the DataSeries.
2.  Format the Date using our built-inÂ [LabelProvider](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-labels/label-provider-api-overview), or create your own

Examples can be found in theÂ <a href="https://www.scichart.com/demo" rel="noopener noreferrer" target="_blank">SciChart.js examples suite</a>, or in our documentation on theÂ [Label Formatting page](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-labels/numeric-formats).

### Storing Strings in DataSeries in SciChart<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-series-api/data-series-api-overview/#storing-strings-in-dataseries-in-scichart" class="hash-link" aria-label="Direct link to Storing Strings in DataSeries in SciChart" translate="no" title="Direct link to Storing Strings in DataSeries in SciChart">â€‹</a>

Similarly, DataSeries don't support strings, but if you want to render strings, then it's advisable to use X values as sequential integers e.g. 0,1,2,3... and use theÂ [LabelProvider feature](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-labels/label-provider-api-overview) to format labels as strings.

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-series-api/data-series-api-overview/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [Append, Insert, Update, Remove](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-series-api/append-insert-update-remove)
- [DataSeries Get Set value at Index](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-series-api/get-set-value-at-index)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/data-series-api/data-series-api-overview/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/data-series-api/data-series-api-overview/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
