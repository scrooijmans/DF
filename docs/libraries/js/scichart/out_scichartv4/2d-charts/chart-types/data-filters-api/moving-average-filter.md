On this page

# Moving Average Filter

The XyMovingAverageFilter performs a simple moving average with a specified period length, resulting in a smoothed waveform derived from your data.

## Applying a Moving Average to Chart Data<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-filters-api/moving-average-filter/#applying-a-moving-average-to-chart-data" class="hash-link" aria-label="Direct link to Applying a Moving Average to Chart Data" translate="no" title="Direct link to Applying a Moving Average to Chart Data">â€‹</a>

To calculate a moving average and apply to a chart, use the following code.

- Moving average

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
    let xValues = [];
    let yValues = [];
    for (let i = start; i < start + count; i++) {
        xValues.push(i);
        yValues.push(2 * Math.sin(i/10) + Math.random());      
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

This results in the following output:

<img src="out_scichartv4/2d-charts/chart-types/data-filters-api/moving-average-filter/index_media/f753895404812d7aa9e93c3631b3297d92e19a83.png" class="img_ev3q" decoding="async" loading="lazy" width="799" height="600" />

## Updating Moving Averages Dynamically<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-filters-api/moving-average-filter/#updating-moving-averages-dynamically" class="hash-link" aria-label="Direct link to Updating Moving Averages Dynamically" translate="no" title="Direct link to Updating Moving Averages Dynamically">â€‹</a>

When the underlying data updates, the filter automatically updates. There is no need to recalculate your filter: SciChart.js does this for you!

Where possible, only the changed points are recalculated. In addition, updating the parameters of the filter, in this case the length, will also recalulate the filter and redraw.

If we add the following to the above example:

- Moving average

``` prism-code
// Add some additional data every 100ms
const updateFunc = () => {
    if (dataSeries.count() < 300) {
        const { xValues, yValues } = getData(dataSeries.count(), 10);
        dataSeries.appendRange(xValues, yValues);
        setTimeout(updateFunc, 100);
    }
};
// Start the update
setTimeout(updateFunc, 1000);
```

We get this output.

<img src="out_scichartv4/2d-charts/chart-types/data-filters-api/moving-average-filter/index_media/e89efeb7df690262ed89b57ab42fbae5489b0ec7.gif" class="img_ev3q" decoding="async" loading="lazy" width="394" height="292" />

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-filters-api/moving-average-filter/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [What is the Filters API](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-filters-api/data-filters-api-overview)
- [Scale Offset Filters](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-filters-api/scale-offset-filters)
- [Linear Trendline Filter](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-filters-api/linear-trendline-filter)
- [Ratio Filter](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-filters-api/ratio-filter)
- [Creating a Custom Filter](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-filters-api/custom-filter)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/data-filters-api/moving-average-filter/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/data-filters-api/moving-average-filter/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
