On this page

# Linear Trendline Filter

The Linear Trend filter calculates a straight line best-fit for your DataSeries. This can then be plotted as a line series on the chart.

## ApplyingÂ a Linear TrendlineÂ to Chart Data<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-filters-api/linear-trendline-filter/#applyinga-linear-trendlineto-chart-data" class="hash-link" aria-label="Direct link to ApplyingÂ a Linear TrendlineÂ to Chart Data" translate="no" title="Direct link to ApplyingÂ a Linear TrendlineÂ to Chart Data">â€‹</a>

To calculate a Linear Trendline use the following code.

- Linear Trendline

``` prism-code
import {
    SciChartSurface,
    NumericAxis,
    XyDataSeries,
    FastLineRenderableSeries,
    XyScatterRenderableSeries,
    NumberRange,
    XyLinearTrendFilter 
} from "scichart";
...
const { sciChartSurface, wasmContext } = await SciChartSurface.create('scichart-div-id-2');
sciChartSurface.xAxes.add(new NumericAxis(wasmContext, { growBy: new NumberRange(0.1, 0.1) }));
sciChartSurface.yAxes.add(new NumericAxis(wasmContext, { growBy: new NumberRange(0.1, 0.1) }));
// Original Data
const dataSeries = new XyDataSeries(wasmContext, {
    xValues: [1, 2, 3, 4, 5],
    yValues: [1, 3, 2, 4, 6],
});
const originalLine = new XyScatterRenderableSeries(wasmContext, { dataSeries });
// Create the filter, passing in the original series
const linearTrendFilter = new XyLinearTrendFilter(dataSeries);
const filteredLine = new FastLineRenderableSeries(wasmContext, { dataSeries: linearTrendFilter, stroke: "#cc6600" });
sciChartSurface.renderableSeries.add(originalLine, filteredLine);
```

This results in the following output:

<img src="out_scichartv4/2d-charts/chart-types/data-filters-api/linear-trendline-filter/index_media/c29a7e3bf0d7a89abdc8b56ca6a501aa02012309.png" class="img_ev3q" decoding="async" loading="lazy" width="799" height="551" />

## Accessing Trendline Properties<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-filters-api/linear-trendline-filter/#accessing-trendline-properties" class="hash-link" aria-label="Direct link to Accessing Trendline Properties" translate="no" title="Direct link to Accessing Trendline Properties">â€‹</a>

The **XyLinearTrendFilter** has properties for slope, intercept and correlation which you can use to access the parameters of the trend line.

For example you could display them using an annotation like this:

- Trendline properties

``` prism-code
import { TextAnnotation, EHorizontalAnchorPoint, EVerticalAnchorPoint } from "scichart";

const textAnnotation = new TextAnnotation({
    x1: 1,
    y1: 5,
    fontSize: 20,
    text: `Slope: ${linearTrendFilter.slope}, y-intercept: ${linearTrendFilter.intercept}, correlation: ${linearTrendFilter.correlation.toFixed(3)}`
});
sciChartSurface.annotations.add(textAnnotation);
```

<img src="out_scichartv4/2d-charts/chart-types/data-filters-api/linear-trendline-filter/index_media/89c58c0a3a27d009b5ae66d609664e002c99cd29.png" class="img_ev3q" decoding="async" loading="lazy" width="800" height="550" />

## Specifying the Input Field<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-filters-api/linear-trendline-filter/#specifying-the-input-field" class="hash-link" aria-label="Direct link to Specifying the Input Field" translate="no" title="Direct link to Specifying the Input Field">â€‹</a>

The XyLinearTrendFilter produces an XyDataSeries, but it can accept any series type as input. The options includes aÂ **field** property of type **EDataSeriesField**, which determines which field on the original series will be the input.

This example takes a OHLC Series as input and plots separate trendlines for the High and Low values:

- Trendline properties

``` prism-code
import { 
    OhlcDataSeries,
    FastLineRenderableSeries,
    FastCandlestickRenderableSeries,
    XyLinearTrendFilter,
    EDataSeriesField 
} from "scichart";
...
// Original Data
const dataSeries = new OhlcDataSeries(wasmContext, {
    xValues: [1, 2, 3, 4, 5],
    openValues: [1, 3, 2, 4, 6],
    highValues: [5, 4, 7, 6.5, 8],
    lowValues: [0.5, 2, 1.5, 2.5, 4],
    closeValues: [3, 2.5, 5, 3, 5],
});
const rsCandles = new FastCandlestickRenderableSeries(wasmContext, { dataSeries });
// Create the filters, passing in the original series and specifying the input field
const linearTrendHigh = new XyLinearTrendFilter(dataSeries, { field: EDataSeriesField.High });
const linearTrendLow = new XyLinearTrendFilter(dataSeries, { field: EDataSeriesField.Low });
const rsHigh = new FastLineRenderableSeries(wasmContext, { dataSeries: linearTrendHigh, stroke: "#ddff33", strokeThickness: 3 });
const rsLow = new FastLineRenderableSeries(wasmContext, { dataSeries: linearTrendLow, stroke: "#ff5599", strokeThickness: 3 });
sciChartSurface.renderableSeries.add(rsCandles, rsHigh, rsLow);
```

<img src="out_scichartv4/2d-charts/chart-types/data-filters-api/linear-trendline-filter/index_media/d8198e8546a2faedcff9f27528aa6b7197ac931e.png" class="img_ev3q" decoding="async" loading="lazy" width="798" height="598" />

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-filters-api/linear-trendline-filter/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [What is the Filters API](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-filters-api/data-filters-api-overview)
- [Scale Offset Filters](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-filters-api/scale-offset-filters)
- [Creating a Custom Filter](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-filters-api/custom-filter)
- [Ratio Filter](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-filters-api/ratio-filter)
- [Moving Average Filter](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-filters-api/moving-average-filter)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/data-filters-api/linear-trendline-filter/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/data-filters-api/linear-trendline-filter/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
