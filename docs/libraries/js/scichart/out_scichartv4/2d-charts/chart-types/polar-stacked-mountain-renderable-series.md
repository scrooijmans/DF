On this page

# The Polar Stacked Mountain Chart Type

The Polar Stacked Mountain Chart Type is created using a <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarstackedmountaincollection.html" rel="noopener noreferrer" target="_blank">PolarStackedMountainCollectionðŸ“˜</a> to manage multiple series of <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarstackedmountainrenderableseries.html" rel="noopener noreferrer" target="_blank">PolarStackedMountainRenderableSeriesðŸ“˜</a>, which represent the individual stacked mountains in the chart.

![](out_scichartv4/2d-charts/chart-types/polar-stacked-mountain-renderable-series/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

The <a href="https://www.scichart.com/demo/javascript/polar-stacked-mountain-chart" rel="noopener noreferrer" target="_blank">JavaScript Polar Stacked Mountain Chart</a> can be found in theÂ <a href="https://github.com/ABTSoftware/SciChart.JS.Examples/blob/release_v4.0/Examples/src/components/Examples/Charts2D/PolarCharts/PolarStackedMountainChart" rel="noopener noreferrer" target="_blank">SciChart.Js Examples Suite &gt; Polar Stacked Mountain Chart</a>Â on Github, or our live demo atÂ <a href="https://www.scichart.com/demo/react/polar-stacked-mountain-chart" rel="noopener noreferrer" target="_blank">scichart.com/demo</a>.

Above: The JavaScript <a href="https://www.scichart.com/demo/iframe/polar-stacked-mountain-chart" target="_blank">Polar Stacked Mountain Series Chart</a> example from the <a href="https://www.scichart.com/demo/react" target="_blank">SciChart.js Demo</a>

  

## Create a Basic Polar Stacked Mountain Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-stacked-mountain-renderable-series/#create-a-basic-polar-stacked-mountain-series" class="hash-link" aria-label="Direct link to Create a Basic Polar Stacked Mountain Series" translate="no" title="Direct link to Create a Basic Polar Stacked Mountain Series">â€‹</a>

To create a Javascript <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarstackedmountainrenderableseries.html" rel="noopener noreferrer" target="_blank">Polar Stacked Mountain SeriesðŸ“˜</a> with SciChart.js, use the following code:

- TS
- Builder API (JSON Config)

``` prism-code
// Demonstrates how to create a basic polar mountain chart using SciChart.js
const { 
    SciChartPolarSurface, 
    PolarNumericAxis, 
    SciChartJsNavyTheme,
    PolarStackedMountainCollection,
    PolarStackedMountainRenderableSeries,
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
    visibleRange: new NumberRange(0, 6),
    flippedCoordinates: true, // go clockwise
});
sciChartSurface.xAxes.add(angularXAxis);

const radialYAxis = new PolarNumericAxis(wasmContext, {
    polarAxisMode: EPolarAxisMode.Radial, // Radial == "goes from center out, drawn by straight lines"
    visibleRange: new NumberRange(0, 10),
    drawLabels: false, // hide radial labels
});
sciChartSurface.yAxes.add(radialYAxis);

// Create the collection the stacked mountains will be added to
const polarCollection = new PolarStackedMountainCollection(wasmContext);

const xValues = [0, 1, 2, 3, 4, 5]; // x values for the mountains
const yValues1 = Array.from({ length: 6 }, (_, i) => Math.random() * 6 + 2);
const yValues2 = Array.from({ length: 6 }, (_, i) => Math.random() * 1 + 1);

const polarMountain1 = new PolarStackedMountainRenderableSeries(wasmContext, {
    dataSeries: new XyDataSeries(wasmContext, {
        xValues: [...xValues, xValues[0]], // +1 to close the loop
        yValues: [...yValues1, yValues1[0]]  // connect first and last points
    }),
    fill: "#FF3344AA",
    stroke: "white",
    strokeThickness: 3,
});

const polarMountain2 = new PolarStackedMountainRenderableSeries(wasmContext, {
    dataSeries: new XyDataSeries(wasmContext, {
        xValues: [...xValues, xValues[0]], // +1 to close the loop
        yValues: [...yValues2, yValues2[0]] // connect first and last points
    }),
    fill: "#5588FFAA",
    stroke: "white",
    strokeThickness: 3,
});

// Add the mountains to the collection
polarCollection.add(polarMountain1, polarMountain2);

// Add the mountains to the collection
sciChartSurface.renderableSeries.add(polarCollection);
```

``` prism-code
// Demonstrates how to create a band chart with SciChart.js using the Builder API
const { 
    chartBuilder,
    SciChartJsNavyTheme,
    EAxisType,
    ESeriesType,
    EPolarAxisMode,
    EAxisAlignment,
    NumberRange,
} = SciChart;
// or, for npm, import { chartBuilder, ... } from "scichart"

const { wasmContext, sciChartSurface } = await chartBuilder.build2DPolarChart(divElementId, {
    surface: { theme: new SciChartJsNavyTheme() },
    xAxes: [
        {
            type: EAxisType.PolarNumericAxis,
            options: {
                polarAxisMode: EPolarAxisMode.Angular,
                axisAlignment: EAxisAlignment.Top,
                visibleRange: new NumberRange(0, 6),
            }
        }
    ],
    yAxes: [
        {
            type: EAxisType.PolarNumericAxis,
            options: {
                axisAlignment: EAxisAlignment.Right,
                polarAxisMode: EPolarAxisMode.Radial,
                visibleRange: new NumberRange(0, 10),
            }
        }
    ],
    series: [
        {
            type: ESeriesType.PolarMountainSeries,
            xyData: {
                xValues: [0, 1, 2, 3, 4, 5, 0], // +1 to close the loop
                yValues: [2, 3, 5, 4, 6, 5, 2] // connect first and last points
            },
            options: {
                fill: "#FF3344AA",
                stroke: "white",
                strokeThickness: 3,
            }
        },
        {
            type: ESeriesType.PolarMountainSeries,
            xyData: {
                xValues: [0, 1, 2, 3, 4, 5, 0], // +1 to close the loop
                yValues: [1, 1.5, 2, 2.5, 3, 2.5, 1] // connect first and last points
            },
            options: {
                fill: "#5588FFAA",
                stroke: "white",
                strokeThickness: 3,
            }
        }
    ]
});

return { sciChartSurface, wasmContext };
```

Above:

We created 2 <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarstackedmountainrenderableseries.html" rel="noopener noreferrer" target="_blank">PolarStackedMountainRenderableSeriesðŸ“˜</a> and added them to a <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarstackedmountaincollection.html" rel="noopener noreferrer" target="_blank">PolarStackedMountainCollectionðŸ“˜</a>. Each <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarstackedmountainrenderableseries.html" rel="noopener noreferrer" target="_blank">PolarStackedMountainRenderableSeriesðŸ“˜</a> represents a single mountain in the chart, and they are stacked on top of each other. The StackedMountainCollection itself is added to <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#renderableseries" rel="noopener noreferrer" target="_blank">sciChartSurface.renderableSeriesðŸ“˜</a> collection, not the individual mountain series.

This results in the following output:

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-stacked-mountain-renderable-series/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [Polar Mountain Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-mountain-renderable-series)
- [Stacked Mountain Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/stacked-mountain-renderable-series)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/polar-stacked-mountain-renderable-series/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/polar-stacked-mountain-renderable-series/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
