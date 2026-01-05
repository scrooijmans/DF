On this page

# Column Series Data Point Width Mode

### Overview<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-column-renderable-series/data-point-width-mode/#overview" class="hash-link" aria-label="Direct link to Overview" translate="no" title="Direct link to Overview">â€‹</a>

Not just applicable toÂ [Column Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-column-renderable-series/column-series-type), but alsoÂ [Error Bars Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-error-bars-renderable-series),Â [Candlestick](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-candlestick-renderable-series), [Rectangle Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-rectangle-renderable-series) andÂ [OHLC Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-ohlc-renderable-series), theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastcolumnrenderableseries.html#datapointwidth" rel="noopener noreferrer" target="_blank">dataPointWidthðŸ“˜</a> andÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastcolumnrenderableseries.html#datapointwidthmode" rel="noopener noreferrer" target="_blank">dataPointWidthModeðŸ“˜</a> properties allow controlling of spacing of bars on a 2D JavaScript Chart.

In previous articles we've shown howÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastcolumnrenderableseries.html#datapointwidth" rel="noopener noreferrer" target="_blank">dataPointWidthðŸ“˜</a>Â can be used to change the width of bar-style series, for example Columns, Error Bars and Candlesticks/OHLC.

By default,Â <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastcolumnrenderableseries.html#datapointwidth" rel="noopener noreferrer" target="_blank">dataPointWidthðŸ“˜</a>Â sets the width of columns/bars as a fraction of available space, with valid values from 0.0 - 1.0.

New to SciChart.js v3.4 and above, a new property <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastcolumnrenderableseries.html#datapointwidthmode" rel="noopener noreferrer" target="_blank">dataPointWidthModeðŸ“˜</a>Â has been added. This has values of **Absolute**, **Range** and **Relative**. These define how theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastcolumnrenderableseries.html#datapointwidth" rel="noopener noreferrer" target="_blank">dataPointWidthðŸ“˜</a>Â is interpreted on these series types:

### <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/edatapointwidthmode.html" rel="noopener noreferrer" target="_blank">EDataPointWidthMode.AbsoluteðŸ“˜</a><a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-column-renderable-series/data-point-width-mode/#edatapointwidthmodeabsoluteblue_book" class="hash-link" aria-label="Direct link to edatapointwidthmodeabsoluteblue_book" translate="no" title="Direct link to edatapointwidthmodeabsoluteblue_book">â€‹</a>

Interprets Data Point Width as an absolute pixel value

### <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/edatapointwidthmode.html" rel="noopener noreferrer" target="_blank">EDataPointWidthMode.RangeðŸ“˜</a><a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-column-renderable-series/data-point-width-mode/#edatapointwidthmoderangeblue_book" class="hash-link" aria-label="Direct link to edatapointwidthmoderangeblue_book" translate="no" title="Direct link to edatapointwidthmoderangeblue_book">â€‹</a>

Interprets Data Point Width as the x data range per column. This is useful if you are plotting sparse columns on a NumericAxis

### <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/edatapointwidthmode.html" rel="noopener noreferrer" target="_blank">EDataPointWidthMode.RelativeðŸ“˜</a><a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-column-renderable-series/data-point-width-mode/#edatapointwidthmoderelativeblue_book" class="hash-link" aria-label="Direct link to edatapointwidthmoderelativeblue_book" translate="no" title="Direct link to edatapointwidthmoderelativeblue_book">â€‹</a>

Interprets Data Point Width as a relative to the full width which is axis length / number of columns. This assumes that there are no gaps in the data. If you are plotting sparse columns on a NumericAxis, consider Range mode

### Example<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-column-renderable-series/data-point-width-mode/#example" class="hash-link" aria-label="Direct link to Example" translate="no" title="Direct link to Example">â€‹</a>

Here's an example of their use below. They can be very useful to change how sparsely populated column series behave.

Given a dataset with sparse values like this:

- TS

``` prism-code
// Create some data with gaps
const xValues = [0, 10, 30, 70, 80, 90, 110, 120, 150, 180, 190];
const yValues = [0.2, 0.4, 0.8, 1.5, 2.4, 8.1, 13.7, 6.4, 3.5, 1.4, 0.4];

const dataSeries = new XyDataSeries(wasmContext, { xValues, yValues });
```

And 3 column series with different dataPointWidthModes:

- TS

``` prism-code
// Create and add three column series to demonstrate the different EDataPointWidthModes
const columnSeries0 = new FastColumnRenderableSeries(wasmContext, {
    dataSeries,
    fill: "#50C7E077",
    stroke: "#50C7E0",
    strokeThickness: 2,
    yAxisId: "Absolute",
    dataPointWidthMode: EDataPointWidthMode.Absolute,
    // When dataPointWidthMode=Absolute, this is the width of each column in pixels
    dataPointWidth: 8,
});
const columnSeries1 = new FastColumnRenderableSeries(wasmContext, {
    dataSeries,
    fill: "#EC0F6C77",
    stroke: "#EC0F6C",
    strokeThickness: 2,
    yAxisId: "Range",
    dataPointWidthMode: EDataPointWidthMode.Range,
    // When dataPointWidthMode=Range, this is the width of each column in range units
    dataPointWidth: 8,
});
const columnSeries2 = new FastColumnRenderableSeries(wasmContext, {
    dataSeries,
    fill: "#30BC9A77",
    stroke: "#30BC9A",
    strokeThickness: 2,
    yAxisId: "Relative",
    dataPointWidthMode: EDataPointWidthMode.Relative,
    // When dataPointWidthMode=Range, this is the width of each column in relative units of available space
    dataPointWidth: 0.8,
});
sciChartSurface.renderableSeries.add(columnSeries0, columnSeries1, columnSeries2);
```

The result is the following output:

### Breakdown:<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-column-renderable-series/data-point-width-mode/#breakdown" class="hash-link" aria-label="Direct link to Breakdown:" translate="no" title="Direct link to Breakdown:">â€‹</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/edatapointwidthmode.html" rel="noopener noreferrer" target="_blank"><strong>EDataPointWidthMode.Relative</strong>ðŸ“˜</a> was the previous default (and only) value prior to v3.4. This would calculate the available space for a column and render each bar as a fraction of that availble space (from 0.0 - 1.0). The problem with this mode was that when x-values were unevenly spaced or the dataset was sparse, then

<a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/edatapointwidthmode.html" rel="noopener noreferrer" target="_blank"><strong>EDataPointWidthMode.Absolute</strong>ðŸ“˜</a>Â has been added post v3.4 which renders each bar as a pixel width. This is perfect for handling sparse datasets, except the bar will not scale as the chart is zoomed in or out.

<a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/edatapointwidthmode.html" rel="noopener noreferrer" target="_blank"><strong>EDataPointWidthMode.Relative</strong>ðŸ“˜</a>Â has also been added post v3.4. This mode renders each bar as a x-Range, so if the xAxis has a range of 0-200 and you specify a value of 8, then a bar will occupy exactly 8 data units. Using this mode, bars or columns will never overlap.

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/fast-column-renderable-series/data-point-width-mode/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/fast-column-renderable-series/data-point-width-mode/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
