On this page

# The Fan Charts Type

Fan Charts are provided by using multipleÂ [Band Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-band-renderable-series) on the same chart.

![](out_scichartv4/2d-charts/chart-types/fan-charts-type/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

TheÂ <a href="https://www.scichart.com/demo/javascript-fan-chart" rel="noopener noreferrer" target="_blank">JavaScript Fan Chart Example</a>Â can be found in theÂ <a href="https://github.com/abtsoftware/scichart.js.examples" rel="noopener noreferrer" target="_blank">SciChart.Js Examples Suite</a>Â on Github, or our live demo atÂ <a href="https://www.scichart.com/demo/" rel="noopener noreferrer" target="_blank">scichart.com/demo</a>

Above: The JavaScript <a href="https://www.scichart.com/demo/iframe/fan-chart" target="_blank">Fan Chart</a> example from the <a href="https://www.scichart.com/demo/react" target="_blank">SciChart.js Demo</a>

  

## Create aÂ Fan Chart<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fan-charts-type/#create-afan-chart" class="hash-link" aria-label="Direct link to Create aÂ Fan Chart" translate="no" title="Direct link to Create aÂ Fan Chart">â€‹</a>

There is no Fan Chart type out of the box in SciChart.js, but it is easy to create one using multiple Band series. Start with the following code:

- TS
- Builder API (JSON Config)

``` prism-code
// Demonstrates how to create a fan chart using SciChart.js
const {
    SciChartSurface,
    NumericAxis,
    FastLineRenderableSeries,
    FastBandRenderableSeries,
    XyDataSeries,
    XyyDataSeries,
    SciChartJsNavyTheme
} = SciChart;
// or, for npm, import { SciChartSurface, ... } from "scichart"

const { wasmContext, sciChartSurface } = await SciChartSurface.create(divElementId, {
    theme: new SciChartJsNavyTheme()
});
sciChartSurface.xAxes.add(new NumericAxis(wasmContext));
sciChartSurface.yAxes.add(new NumericAxis(wasmContext));

// get data for the fan chart
// format is [{ date, actual, varMax, var4, var3, var2, var1, varMin }]
const varianceData = getVarianceData();
const xValues = varianceData.map(v => v.date);

// Add a line series with the Xy data (the actual data)
sciChartSurface.renderableSeries.add(
    new FastLineRenderableSeries(wasmContext, {
        dataSeries: new XyDataSeries(wasmContext, { xValues, yValues: varianceData.map(v => v.actual) }),
        stroke: "#3388FF"
    })
);

// Add band series with progressively higher opacity for the fan variance data
sciChartSurface.renderableSeries.add(
    new FastBandRenderableSeries(wasmContext, {
        dataSeries: new XyyDataSeries(wasmContext, {
            xValues,
            yValues: varianceData.map(v => v.varMin),
            y1Values: varianceData.map(v => v.varMax)
        }),
        opacity: 0.15,
        fill: "#3388FF",
        stroke: "#00000000",
        strokeY1: "#00000000"
    })
);
sciChartSurface.renderableSeries.add(
    new FastBandRenderableSeries(wasmContext, {
        dataSeries: new XyyDataSeries(wasmContext, {
            xValues,
            yValues: varianceData.map(v => v.var1),
            y1Values: varianceData.map(v => v.var4)
        }),
        opacity: 0.33,
        fill: "#3388FF",
        stroke: "#00000000",
        strokeY1: "#00000000"
    })
);
sciChartSurface.renderableSeries.add(
    new FastBandRenderableSeries(wasmContext, {
        dataSeries: new XyyDataSeries(wasmContext, {
            xValues,
            yValues: varianceData.map(v => v.var2),
            y1Values: varianceData.map(v => v.var3)
        }),
        opacity: 0.5,
        fill: "#3388FF",
        stroke: "#00000000",
        strokeY1: "#00000000"
    })
);
```

``` prism-code
// Demonstrates how to create a band chart with SciChart.js using the Builder API
const { chartBuilder, ESeriesType, EThemeProviderType } = SciChart;
// or, for npm, import { chartBuilder, ... } from "scichart"

// get data for the fan chart
// format is [{ date, actual, varMax, var4, var3, var2, var1, varMin }]
const varianceData = getVarianceData();

// Convert to arrays expected by scichart.js. There are more efficient ways to do this!
const xValues = varianceData.map(v => v.date);
const yValues = varianceData.map(v => v.actual);
const varMinValues = varianceData.map(v => v.varMin);
const varMaxValues = varianceData.map(v => v.varMax);
const var1Values = varianceData.map(v => v.var1);
const var2Values = varianceData.map(v => v.var2);
const var3Values = varianceData.map(v => v.var3);
const var4Values = varianceData.map(v => v.var4);

const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    surface: { theme: { type: EThemeProviderType.Dark } },
    series: [
        {
            type: ESeriesType.LineSeries,
            xyData: { xValues, yValues },
            options: {
                stroke: "#3388FF"
            }
        },
        {
            type: ESeriesType.BandSeries,
            xyyData: { xValues, yValues: varMinValues, y1Values: varMaxValues },
            options: {
                opacity: 0.15,
                fill: "#3388FF",
                strokeY1: "#00000000"
            }
        },
        {
            type: ESeriesType.BandSeries,
            xyyData: { xValues, yValues: var1Values, y1Values: var4Values },
            options: {
                opacity: 0.33,
                fill: "#3388FF",
                strokeY1: "#00000000"
            }
        },
        {
            type: ESeriesType.BandSeries,
            xyyData: { xValues, yValues: var2Values, y1Values: var3Values },
            options: {
                opacity: 0.5,
                fill: "#3388FF",
                strokeY1: "#00000000"
            }
        }
    ]
});
```

This results in the following output:

In the example above:

- Some variance data is first created and returned as an array of objects.
- A Line series is created to display the actual X,Y value
- Several Band SeriesÂ areÂ created and added to theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#renderableseries" rel="noopener noreferrer" target="_blank">SciChartSurface.renderableSeriesðŸ“˜</a> collection to render the variance bands.
- We set the stroke, fill propertiesÂ and opacity of each seriesÂ (more info over atÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastbandrenderableseries.html" rel="noopener noreferrer" target="_blank">FastBandRenderableSeries in TypeDocðŸ“˜</a>).
- We assign aÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basedataseries.html" rel="noopener noreferrer" target="_blank">DataSeriesðŸ“˜</a> - in this case anÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyydataseries.html" rel="noopener noreferrer" target="_blank">XyyDataSeriesðŸ“˜</a> which stores X, Y1, Y2 data for bands, andÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html" rel="noopener noreferrer" target="_blank">XyDataSeriesðŸ“˜</a> for lines.

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/fan-charts-type/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/fan-charts-type/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
