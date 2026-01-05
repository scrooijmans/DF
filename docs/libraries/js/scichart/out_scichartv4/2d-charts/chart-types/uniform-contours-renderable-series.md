On this page

# The Contours Series Type

Contour maps or Contour-plots can be created using theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/uniformcontoursrenderableseries.html" rel="noopener noreferrer" target="_blank">UniformContoursRenderableSeriesðŸ“˜</a> type.

![](out_scichartv4/2d-charts/chart-types/uniform-contours-renderable-series/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

TheÂ <a href="https://www.scichart.com/demo/javascript/heatmap-chart" rel="noopener noreferrer" target="_blank">JavaScript Heatmap Chart Example</a>Â can be found in theÂ <a href="https://github.com/ABTSoftware/SciChart.JS.Examples/tree/master/Examples/src/components/Examples/Charts2D/BasicChartTypes/ContoursChart" rel="noopener noreferrer" target="_blank">SciChart.Js Examples Suite &gt; Contours Chart</a>Â on Github, or our live demo atÂ <a href="https://www.scichart.com/demo/javascript/heatmap-chart" rel="noopener noreferrer" target="_blank">scichart.com/demo</a>.

Above: The JavaScript <a href="https://www.scichart.com/demo/iframe/heatmap-chart-with-contours" target="_blank">Uniform Contours Series Chart</a> example from the <a href="https://www.scichart.com/demo/react" target="_blank">SciChart.js Demo</a>

  

## Create aÂ Contours Plot<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/uniform-contours-renderable-series/#create-acontours-plot" class="hash-link" aria-label="Direct link to Create aÂ Contours Plot" translate="no" title="Direct link to Create aÂ Contours Plot">â€‹</a>

SciChart's Contour series is an extremely fast, lightweight chart types for rendering two dimensional data as a contour plot. TheÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/uniformcontoursrenderableseries.html" rel="noopener noreferrer" target="_blank">UniformContoursRenderableSeriesðŸ“˜</a>Â type should be used in conjunction with aÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/uniformheatmapdataseries.html" rel="noopener noreferrer" target="_blank">UniformHeatmapDataSeriesðŸ“˜</a>Â when you simply want to specify a Step in the X,Y direction (each cell is the same size).

To create aÂ <a href="https://www.scichart.com/demo/javascript-heatmap-chart-with-contours" rel="noopener noreferrer" target="_blank">Javascript Contours Chart</a>Â with SciChart.js, use the following code:

- TS
- Builder API (JSON Config)

``` prism-code
// Demonstrates how to create a contour plot with SciChart.js
const {
    SciChartSurface,
    NumericAxis,
    HeatmapColorMap,
    UniformHeatmapDataSeries,
    UniformHeatmapRenderableSeries,
    UniformContoursRenderableSeries,
    SciChartJsNavyTheme
} = SciChart;
// or, for npm, import { SciChartSurface, ... } from "scichart"

// Create a SciChartSurface with X & Y Axis
const { wasmContext, sciChartSurface } = await SciChartSurface.create(divElementId, {
    theme: new SciChartJsNavyTheme()
});
sciChartSurface.xAxes.add(new NumericAxis(wasmContext));
sciChartSurface.yAxes.add(new NumericAxis(wasmContext));

const WIDTH = 300;
const HEIGHT = 200;
const colorPaletteMax = 200;

// Create a Heatmap Data-series. zValues are heatValues as a 2D Array (number[][])
// Open the Codepen below to see the definition of this function
const zValues = generateExampleData(3, WIDTH, HEIGHT, colorPaletteMax);

// Create the uniform heatmap data series. Pass heatValues as number[][]
const heatmapDataSeries = new UniformHeatmapDataSeries(wasmContext, {
    // 2d zValues array. Dimensions [height][width]
    zValues,
    // xStart, xStep, yStart, yStep defines the x,y position
    xStart: 0,
    xStep: 1,
    yStart: 0,
    yStep: 1
});

// Create a Contours RenderableSeries with the same data
const contourSeries = new UniformContoursRenderableSeries(wasmContext, {
    dataSeries: heatmapDataSeries,
    zMin: 20,
    zMax: colorPaletteMax,
    zStep: 20
});

// Add it to the scichartsurface
sciChartSurface.renderableSeries.add(contourSeries);

// Create a background heatmap series with the same data and add to the chart
const heatmapSeries = new UniformHeatmapRenderableSeries(wasmContext, {
    dataSeries: heatmapDataSeries,
    opacity: 0.5,
    useLinearTextureFiltering: false,
    // See heatmap documentation for description of how colormaps work
    colorMap: new HeatmapColorMap({
        minimum: 0,
        maximum: colorPaletteMax,
        gradientStops: [
            { offset: 1, color: "#EC0F6C" },
            { offset: 0.9, color: "#F48420" },
            { offset: 0.7, color: "#DC7969" },
            { offset: 0.5, color: "#67BDAF" },
            { offset: 0.3, color: "#50C7E0" },
            { offset: 0.2, color: "#264B93" },
            { offset: 0, color: "#14233C" }
        ]
    })
});

// Add to the SciChartSurface
sciChartSurface.renderableSeries.add(heatmapSeries);
```

``` prism-code
// Demonstrates how to create a line chart with SciChart.js using the Builder API
const { chartBuilder, ESeriesType, HeatmapColorMap, EThemeProviderType } = SciChart;
// or, for npm, import { chartBuilder, ... } from "scichart"

const WIDTH = 300;
const HEIGHT = 200;
const colorPaletteMax = 200;

// Create a Heatmap Data-series. zValues are heatValues as a 2D Array (number[][])
// Open the Codepen below to see the definition of this function
const zValues = generateExampleData(3, WIDTH, HEIGHT, colorPaletteMax);

const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    surface: { theme: { type: EThemeProviderType.Dark } },
    series: [
        {
            type: ESeriesType.UniformContoursSeries,
            options: {
                zMin: 20,
                zMax: colorPaletteMax,
                zStep: 20
            },
            heatmapData: {
                zValues,
                xStart: 0,
                xStep: 1,
                yStart: 0,
                yStep: 1
            }
        },
        {
            type: ESeriesType.UniformHeatmapSeries,
            options: { 
                colorMap: new HeatmapColorMap({
                    minimum: 0,
                    maximum: colorPaletteMax,
                    gradientStops: [
                        { offset: 1, color: "#EC0F6C" },
                        { offset: 0.9, color: "#F48420" },
                        { offset: 0.7, color: "#DC7969" },
                        { offset: 0.5, color: "#67BDAF" },
                        { offset: 0.3, color: "#50C7E0" },
                        { offset: 0.2, color: "#264B93" },
                        { offset: 0, color: "#14233C" }
                    ]
                }), 
                opacity: 0.5 
            },
            heatmapData: {
                zValues,
                xStart: 0,
                xStep: 1,
                yStart: 0,
                yStep: 1
            }
        }
    ]
});
```

In the code above:

- We create an empty 2D array `number[][]` using the helper function `zeroArray2D`. This is filled with values in the generateData function
- AÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/uniformheatmapdataseries.html" rel="noopener noreferrer" target="_blank">UniformHeatmapDataSeriesðŸ“˜</a>Â instance is created with `xStart`, `xStep`, `yStart`, `yStep` values = `0`, `1`, `0`, `1`. This means the heatmap starts at `(X, Y)` =Â `(0, 0)` and each cell is `1` on the axis.
- We set the contour `stroke` and `strokeThickness`.
- AÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/uniformcontoursrenderableseries.html" rel="noopener noreferrer" target="_blank">UniformContoursRenderableSeriesðŸ“˜</a>Â instance is created and added to theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#renderableseries" rel="noopener noreferrer" target="_blank">sciChartSurface.renderableSeriesðŸ“˜</a> collection.

This results in the following output:

## Updating Data in a Contour map<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/uniform-contours-renderable-series/#updating-data-in-a-contour-map" class="hash-link" aria-label="Direct link to Updating Data in a Contour map" translate="no" title="Direct link to Updating Data in a Contour map">â€‹</a>

The contour map is supposed to be fully dynamic, enabling real-time graphics. TheÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/uniformcontoursrenderableseries.html" rel="noopener noreferrer" target="_blank">Contours SeriesðŸ“˜</a>Â however does not support append, insert, update, remove functions like other DataSeries do. You can however update the data and force a refresh simply by updating the data passed in. To do this, use the following code:

``` prism-code
import { UniformHeatmapDataSeries, zeroArray2D  } from "scichart";

const height = 10; // Set the height of the heatmap
const width = 20; // Set the width of the heatmap

// Create an empty 2D array of size height & width
const initialZValues: number[][] = zeroArray2D([height, width]);
// Create a Heatmap Data-series. Pass the heatValues as a number[][] to the UniformHeatmapDataSeries
const heatmapDataSeries = new UniformHeatmapDataSeries({
    xStart: 0,
    xStep: 1,
    yStart: 0,
    yStep: 1,
    zValues: initialZValues
});

// ...
// Later, update the data
initialZValues[5][6] = 123.4;
heatmapDataSeries.notifyDataChanged() // Notify SciChart that the data has changed

// You can also load an entirely new array with the function UniformHeatmapDataSeries.setZValues
const newZValues; // type number[][]
heatmapDataSeries.setZValues(newZValues);
```

For more details, including a live example of how to update 2D array data for heatmaps and contours, see theÂ [Uniform Heatmap documentation - Updating Heatmaps](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/uniform-heatmap-renderable-series/updating-realtime)Â documentation page. The mechanism for contour plots is the same.

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/uniform-contours-renderable-series/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [The Uniform Heatmap Chart Type](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/uniform-heatmap-renderable-series/uniform-heatmap-chart-type)
- [The Non-Uniform Heatmap Chart Type](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/non-uniform-heatmap-renderable-series)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/uniform-contours-renderable-series/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/uniform-contours-renderable-series/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
