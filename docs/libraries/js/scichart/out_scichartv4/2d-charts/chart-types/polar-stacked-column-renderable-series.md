On this page

# The Polar Stacked Column Chart Type

![](out_scichartv4/2d-charts/chart-types/polar-stacked-column-renderable-series/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

The <a href="https://www.scichart.com/demo/javascript/polar-stacked-column-chart" rel="noopener noreferrer" target="_blank">JavaScript Polar Stacked Column Chart</a> can be found in theÂ <a href="https://github.com/ABTSoftware/SciChart.JS.Examples/blob/release_v4.0/Examples/src/components/Examples/Charts2D/PolarCharts/PolarWindroseColumnChart" rel="noopener noreferrer" target="_blank">SciChart.Js Examples Suite &gt; Polar Windrose Chart</a>Â on Github, or our live demo atÂ <a href="https://www.scichart.com/demo/react/polar-windrose-column-chart" rel="noopener noreferrer" target="_blank">scichart.com/demo</a>.

Above: The JavaScript <a href="https://www.scichart.com/demo/iframe/polar-windrose-column-chart" target="_blank">Polar Stacked Column Series Chart</a> example from the <a href="https://www.scichart.com/demo/react" target="_blank">SciChart.js Demo</a>

  

The Polar Stacked Column Chart Type is created using a <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarstackedcolumncollection.html" rel="noopener noreferrer" target="_blank">PolarStackedColumnCollectionðŸ“˜</a> to manage multiple series of <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarstackedcolumnrenderableseries.html" rel="noopener noreferrer" target="_blank">PolarStackedColumnRenderableSeriesðŸ“˜</a>, which represent the individual stacked columns in the chart.

## Create a Basic Polar Stacked Column Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-stacked-column-renderable-series/#create-a-basic-polar-stacked-column-series" class="hash-link" aria-label="Direct link to Create a Basic Polar Stacked Column Series" translate="no" title="Direct link to Create a Basic Polar Stacked Column Series">â€‹</a>

To create a Javascript <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarstackedcolumnrenderableseries.html" rel="noopener noreferrer" target="_blank">Polar Stacked Column SeriesðŸ“˜</a> with SciChart.js, use the following code:

- TS

``` prism-code
// Demonstrates how to create a basic polar column chart using SciChart.js
const { 
    SciChartPolarSurface, 
    PolarNumericAxis, 
    SciChartJsNavyTheme,
    PolarStackedColumnCollection,
    PolarStackedColumnRenderableSeries,
    EPolarAxisMode,
    NumberRange,
    XyDataSeries, 
} = SciChart;
// or, for npm, import { SciChartSurface, ... } from "scichart"

const { sciChartSurface, wasmContext } = await SciChartPolarSurface.create(divElementId, {
    theme: new SciChartJsNavyTheme(),
});

const angularXAxis = new PolarNumericAxis(wasmContext, {
    polarAxisMode: EPolarAxisMode.Angular, // Angular == "goes around the center, drawn by arcs"
    visibleRange: new NumberRange(0, 10), // to keep column #1 and #10 from touching
    flippedCoordinates: true, // go clockwise
});
sciChartSurface.xAxes.add(angularXAxis);

const radialYAxis = new PolarNumericAxis(wasmContext, {
    polarAxisMode: EPolarAxisMode.Radial, // Radial == "goes from center out, drawn by straight lines"
    innerRadius: 0.1, // donut hole
    labelPrecision: 0, // no decimals
});
sciChartSurface.yAxes.add(radialYAxis);

// Create the collection the stacked columns will be added to
const polarCollection = new PolarStackedColumnCollection(wasmContext, {
    isOneHundredPercent: false // set to true to make columns stack to 100%
});

const polarColumn1 = new PolarStackedColumnRenderableSeries(wasmContext, {
    dataSeries: new XyDataSeries(wasmContext, {
        xValues: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
        yValues: Array.from({ length: 10 }, (_, i) => Math.random() * 11 + 1)
    }),
    fill: "#FF3344AA",
    stroke: "white",
    strokeThickness: 2,
});

const polarColumn2 = new PolarStackedColumnRenderableSeries(wasmContext, {
    dataSeries: new XyDataSeries(wasmContext, {
        xValues: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
        yValues: Array.from({ length: 10 }, (_, i) => Math.random() * 2 + 1)
    }),
    fill: "#5588FFAA",
});

// Add the columns to the collection
polarCollection.add(polarColumn1, polarColumn2);

// Add the columns to the collection
sciChartSurface.renderableSeries.add(polarCollection);
```

In the code above:

- Set xAxis' <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polaraxisbase.html#polarmode" rel="noopener noreferrer" target="_blank">polarAxisModeðŸ“˜</a> to `EPolarAxisMode.Angular` to create an Angular axis - the way most Polar charts are created.
- Set yAxis' <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polaraxisbase.html#polarmode" rel="noopener noreferrer" target="_blank">polarAxisModeðŸ“˜</a> to `EPolarAxisMode.Radial` to create a Radial axis which represents the height of the columns.
- We create a <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarstackedcolumncollection.html" rel="noopener noreferrer" target="_blank">PolarStackedColumnCollectionðŸ“˜</a> to hold the renderable series.
- We add 2 <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarstackedcolumnrenderableseries.html" rel="noopener noreferrer" target="_blank">PolarStackedColumnRenderableSeriesðŸ“˜</a> to the collection, each with its own data and styling.

> This type of plot is called a **WindRose**, or **Rose** chart, as it is often used to visualize wind speed and direction data.

## Create a Basic **Radial** Polar Stacked Column Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-stacked-column-renderable-series/#create-a-basic-radial-polar-stacked-column-series" class="hash-link" aria-label="Direct link to create-a-basic-radial-polar-stacked-column-series" translate="no" title="Direct link to create-a-basic-radial-polar-stacked-column-series">â€‹</a>

To create a Javascript **Radial** <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarstackedcolumnrenderableseries.html" rel="noopener noreferrer" target="_blank">Polar Stacked Column SeriesðŸ“˜</a> with SciChart.js, use the code from above, but replace the **xAxis** and **yAxis** config with this snippet:

``` prism-code
const radialXAxis = new PolarNumericAxis(wasmContext, {
    polarAxisMode: EPolarAxisMode.Radial, // radial axis -> xAxis
    axisAlignment: EAxisAlignment.Right,
    innerRadius: 0.1, // donut hole
    labelPrecision: 0, // no decimals
});
sciChartSurface.xAxes.add(radialXAxis);
const angularYAxis = new PolarNumericAxis(wasmContext, {
    polarAxisMode: EPolarAxisMode.Angular, // angular axis -> yAxis
    axisAlignment: EAxisAlignment.Top,
    visibleRange: new NumberRange(0, 15),
    useNativeText: true,
    flippedCoordinates: true, // go clockwise
});
sciChartSurface.yAxes.add(angularYAxis);
```

In the code above:

- Set xAxis' <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polaraxisbase.html#polarmode" rel="noopener noreferrer" target="_blank">polarAxisModeðŸ“˜</a> to `EPolarAxisMode.Radial` to create a Radial axis.

- Set yAxis' <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polaraxisbase.html#polarmode" rel="noopener noreferrer" target="_blank">polarAxisModeðŸ“˜</a> to `EPolarAxisMode.Angular` to create an Angular axis.

  > Both of these settings now create a `vertical` or `radial` chart, where x-axis represents the height of the columns, and y-axis represents the angle. If this is something you want to replicate,

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-stacked-column-renderable-series/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [Stacked Column Chart](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/stacked-column-renderable-series)
- [Polar Column Chart](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-column-renderable-series)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/polar-stacked-column-renderable-series/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/polar-stacked-column-renderable-series/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
