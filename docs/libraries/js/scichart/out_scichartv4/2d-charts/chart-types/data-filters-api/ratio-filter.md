On this page

# Ratio Filter

The XyRatioFilter returns a series where each point is the ratio of the original series and the given divisor DataSeries.

The original series and divisor series can be different types, and there is a **divisorField** option to specify the field to use from the divisorSeries.

## Calculating a Ratio of Two Chart Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-filters-api/ratio-filter/#calculating-a-ratio-of-two-chart-series" class="hash-link" aria-label="Direct link to Calculating a Ratio of Two Chart Series" translate="no" title="Direct link to Calculating a Ratio of Two Chart Series">â€‹</a>

To create a ratio filter and apply to a chart, use the code below:

- Ratio filter example

``` prism-code
import { 
    SciChartSurface,
    NumericAxis,
    XyDataSeries,
    FastLineRenderableSeries,
    NumberRange,
    XyMovingAverageFilter 
} from "scichart";
...
const { sciChartSurface, wasmContext } = await SciChartSurface.create('scichart-div-id-4');
sciChartSurface.xAxes.add(new NumericAxis(wasmContext, { growBy: new NumberRange(0.1, 0.1) }));
sciChartSurface.yAxes.add(new NumericAxis(wasmContext, { growBy: new NumberRange(0.01, 0.01), autoRange: EAutoRange.Always }));
// A function to get some data - sinewave plus a randm factor
const getData = (start, count) => {
    let xValues = \[\];
    let yValues = \[\];
    for (let i = start; i < start + count; i++) {
        xValues.push(i);
        yValues.push(2 \* Math.sin(i/10) + Math.random());      
    }
    return { xValues, yValues };
};
// Original Data
const dataSeries = new XyDataSeries(wasmContext, getData(0, 100));
const originalLine = new FastLineRenderableSeries(wasmContext, { dataSeries, stroke: "#5555ff", strokeThickness: 3 });
// Create the filter, passing in the original series
const movingAverage = new XyMovingAverageFilter(dataSeries, { length: 10 });
const filteredLine = new FastLineRenderableSeries(wasmContext, { dataSeries: movingAverage, stroke: "#cc6600", strokeThickness: 3 });
// Another filter using the same original data, but different length
const movingAverage30 = new XyMovingAverageFilter(dataSeries, { length: 30});
const filteredLine30 = new FastLineRenderableSeries(wasmContext, { dataSeries: movingAverage30, stroke: "#55dd55", strokeThickness: 3 });
sciChartSurface.renderableSeries.add(originalLine, filteredLine, filteredLine30);
```

This produces the following chart. We also added a LegendModifier to make it clear which line is which:

<img src="out_scichartv4/2d-charts/chart-types/data-filters-api/ratio-filter/index_media/35214542571677da888adf2dc09930ff59cf0ad0.png" class="img_ev3q" decoding="async" loading="lazy" width="798" height="598" />

The ratio filter requires that the original series and divisor series have the same number of elements.

When you add or remove data, the filter will not update until both series have been updated. It does not matter which series you update first.

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-filters-api/ratio-filter/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [What is the Filters API](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-filters-api/data-filters-api-overview)
- [Scale Offset Filters](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-filters-api/scale-offset-filters)
- [Linear Trendline Filter](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-filters-api/linear-trendline-filter)
- [Creating a Custom Filter](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-filters-api/custom-filter)
- [Moving Average Filter](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-filters-api/moving-average-filter)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/data-filters-api/ratio-filter/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/data-filters-api/ratio-filter/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
