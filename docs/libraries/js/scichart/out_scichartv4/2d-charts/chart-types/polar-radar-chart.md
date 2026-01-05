On this page

# The Polar Radar Chart Type

The Radar chart (or Spider chart) type displays data in a circular layout, where each axis represents a different variable, most often a category. The axes are connected drawn to form a polygon, allowing for easy comparison of multiple variables.

The data is usually plotted using the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarlinerenderableseries.html" rel="noopener noreferrer" target="_blank">PolarLineRenderableSeriesðŸ“˜</a> or <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarmountainrenderableseries.html" rel="noopener noreferrer" target="_blank">PolarMountainRenderableSeriesðŸ“˜</a> classes, which are specialized for polar coordinates.

Above: The JavaScript <a href="https://www.scichart.com/demo/iframe/polar-radar-chart" target="_blank">Polar Radar Chart</a> example from the <a href="https://www.scichart.com/demo/react" target="_blank">SciChart.js Demo</a>

  

## Create a Basic Polar Radar Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-radar-chart/#create-a-basic-polar-radar-series" class="hash-link" aria-label="Direct link to Create a Basic Polar Radar Series" translate="no" title="Direct link to Create a Basic Polar Radar Series">â€‹</a>

To create a Javascript Polar Radar Series with SciChart.js, use the following code:

``` prism-code
const {
    SciChartPolarSurface,
    SciChartJsNavyTheme,
    PolarNumericAxis,
    NumberRange,
    PolarCategoryAxis,
    PolarMountainRenderableSeries,
    EPolarAxisMode,
    EPolarGridlineMode,
    XyDataSeries,
} = SciChart;
// or for npm import { SciChartPolarSurface, ... } from "scichart"

const { sciChartSurface, wasmContext } = await SciChartPolarSurface.create(rootElement, {
    theme: new SciChartJsNavyTheme()
});

const angularXAxis = new PolarCategoryAxis(wasmContext, {
    polarAxisMode: EPolarAxisMode.Angular,
    labels: [ "Offense", "Shooting", "Defense", "Rebounds", "Passing", "Bench" ], // categories
    startAngle: Math.PI / 2, // start at 12 o'clock
    flippedCoordinates: true, // go clockwise

    majorGridLineStyle: { color: "#88888844" },
    drawMinorGridLines: false,
});
sciChartSurface.xAxes.add(angularXAxis);

const radialYAxis = new PolarNumericAxis(wasmContext, {
    polarAxisMode: EPolarAxisMode.Radial,
    gridlineMode: EPolarGridlineMode.Polygons, // this creates the radar chart look
    visibleRange: new NumberRange(0, 10), 
    startAngle: Math.PI / 2, // start at 12 o'clock
    
    labelPrecision: 0,
    majorGridLineStyle: { color: "#88888844" },
    drawMinorGridLines: false,
    drawMajorTickLines: false,
    drawMinorTickLines: false,
});
sciChartSurface.yAxes.add(radialYAxis);

const xValues = [0, 1, 2, 3, 4, 5];
const yValues = [9, 10, 7, 5, 8, 6]; // values for: "Offense", "Shooting", "Defense", "Rebounds", "Passing", "Bench"

// Radar / Spider Charts may also work with `PolarLineRenderableSeries`
const polarMountain = new PolarMountainRenderableSeries(wasmContext, {
    dataSeries: new XyDataSeries(wasmContext, {
        xValues: [...xValues, xValues[xValues.length] + 1], // + 1 to close the loop
        yValues: [...yValues, yValues[0]], // re-plot first point to close the loop
        dataSeriesName: "Golden State Warriors",
    }),
    stroke: "#FFC72C", // Golden State Warriors gold
    fill: "#1D428A80", // Golden State Warriors blue with 50% opacity
    strokeThickness: 4,
});
sciChartSurface.renderableSeries.add(polarMountain);
```

Key aspects of the code above:

- This **radar** / **spider** / **cobweb** chart look is achieved by using `gridlineMode`: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/epolargridlinemode.html" rel="noopener noreferrer" target="_blank">EPolarGridlineMode.PolygonsðŸ“˜</a> on the radial axis (yAxis), which draws straight lines in between gridlines instead of arcs.
- We set `labels: [ "Offense", "Shooting", ... ]` on the angular axis (xAxis) to display the categories.
- While defining the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html" rel="noopener noreferrer" target="_blank">XyDataSeriesðŸ“˜</a> for the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarmountainrenderableseries.html" rel="noopener noreferrer" target="_blank">PolarMountainRenderableSeriesðŸ“˜</a>, we use this trick on the `xValues` array to reach the 1st point from the end of the series, so that the polygon closes correctly:

``` prism-code
xValues: [...xValues, xValues[xValues.length] + 1], // assuming step = 1
```

- For the `yValues` array we do this by re-using the 1st value:

``` prism-code
yValues: [...yValues, yValues[0]]
```

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/polar-radar-chart/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/polar-radar-chart/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
