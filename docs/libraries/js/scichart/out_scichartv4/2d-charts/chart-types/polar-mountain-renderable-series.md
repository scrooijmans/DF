On this page

# The Polar Mountain Chart Type

The <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarmountainrenderableseries.html" rel="noopener noreferrer" target="_blank">PolarMountainRenderableSeriesðŸ“˜</a> is a type of renderable series that displays data in a polar mountain format.

![](out_scichartv4/2d-charts/chart-types/polar-mountain-renderable-series/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

The <a href="https://www.scichart.com/demo/javascript/polar-mountain-chart" rel="noopener noreferrer" target="_blank">JavaScript Polar Mountain Chart</a> can be found in theÂ <a href="https://github.com/ABTSoftware/SciChart.JS.Examples/blob/release_v4.0/Examples/src/components/Examples/Charts2D/PolarCharts/PolarMountainChart" rel="noopener noreferrer" target="_blank">SciChart.Js Examples Suite &gt; Polar Mountain Chart</a>Â on Github, or our live demo atÂ <a href="https://www.scichart.com/demo/react/polar-mountain-chart" rel="noopener noreferrer" target="_blank">scichart.com/demo</a>.

Above: The JavaScript <a href="https://www.scichart.com/demo/iframe/polar-mountain-chart" target="_blank">Polar Mountain Series Chart</a> example from the <a href="https://www.scichart.com/demo/react" target="_blank">SciChart.js Demo</a>

  

## Properties<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-mountain-renderable-series/#properties" class="hash-link" aria-label="Direct link to Properties" translate="no" title="Direct link to Properties">â€‹</a>

Some of <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipolarmountainrenderableseriesoptions.html" rel="noopener noreferrer" target="_blank">IPolarMountainRenderableSeriesOptionsðŸ“˜</a>'s key properties include:

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipolarmountainrenderableseriesoptions.html#dataseries" rel="noopener noreferrer" target="_blank">dataSeriesðŸ“˜</a> - The <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html" rel="noopener noreferrer" target="_blank">XyDataSeriesðŸ“˜</a> containing `xValues` and `yValues` arrays
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipolarmountainrenderableseriesoptions.html#stroke" rel="noopener noreferrer" target="_blank">strokeðŸ“˜</a> - Stroke color for the line
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipolarmountainrenderableseriesoptions.html#strokethickness" rel="noopener noreferrer" target="_blank">strokeThicknessðŸ“˜</a> - Thickness of the line
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipolarmountainrenderableseriesoptions.html#interpolateline" rel="noopener noreferrer" target="_blank">interpolateLineðŸ“˜</a> - When true, line segments draw as arcs instead of straight lines
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipolarmountainrenderableseriesoptions.html#cliptototalangle" rel="noopener noreferrer" target="_blank">clipToTotalAngleðŸ“˜</a> - When true, clips data outside the total angle range
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipolarmountainrenderableseriesoptions.html#pointmarker" rel="noopener noreferrer" target="_blank">pointMarkerðŸ“˜</a> - Optional markers to display at data points
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipolarmountainrenderableseriesoptions.html#paletteprovider" rel="noopener noreferrer" target="_blank">paletteProviderðŸ“˜</a> - Custom coloring provider for dynamic styling
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipolarmountainrenderableseriesoptions.html#datalabels" rel="noopener noreferrer" target="_blank">dataLabelsðŸ“˜</a> - Configuration for optional data labels on points

## Create a Basic Polar Mountain Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-mountain-renderable-series/#create-a-basic-polar-mountain-series" class="hash-link" aria-label="Direct link to Create a Basic Polar Mountain Series" translate="no" title="Direct link to Create a Basic Polar Mountain Series">â€‹</a>

To create a Javascript <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarmountainrenderableseries.html" rel="noopener noreferrer" target="_blank">Polar Mountain SeriesðŸ“˜</a> with SciChart.js, use the following code:

- Creating a Polar Mountain Series

``` prism-code
// Demonstrates how to create a basic polar mountain chart using SciChart.js
const { 
    SciChartPolarSurface, 
    SciChartJsNavyTheme,
    PolarNumericAxis, 
    PolarMountainRenderableSeries,
    EPolarAxisMode,
    EAxisAlignment,
    NumberRange,
    XyDataSeries, 
    EPolarLabelMode
} = SciChart;
// or, for npm, import { SciChartSurface, ... } from "scichart"

const { sciChartSurface, wasmContext } = await SciChartPolarSurface.create(divElementId, {
    theme: new SciChartJsNavyTheme(),
});

const angularXAxis = new PolarNumericAxis(wasmContext, {
    polarAxisMode: EPolarAxisMode.Angular,
    axisAlignment: EAxisAlignment.Top,
    visibleRange: new NumberRange(0, 19),
    polarLabelMode: EPolarLabelMode.Parallel,
});
sciChartSurface.xAxes.add(angularXAxis);

const radialYAxis = new PolarNumericAxis(wasmContext, {
    axisAlignment: EAxisAlignment.Right,
    polarAxisMode: EPolarAxisMode.Radial,
    visibleRange: new NumberRange(0, 8),
    labelPrecision: 0,
});
sciChartSurface.yAxes.add(radialYAxis);

const polarMountain = new PolarMountainRenderableSeries(wasmContext, {
    dataSeries: new XyDataSeries(wasmContext, {
        xValues: Array.from({ length: 20 }, (_, i) => i),
        yValues: Array.from({ length: 20 }, (_, i) => Math.sin(i) * 3 + 4),
    }),
    stroke: "pink",
    strokeThickness: 4,
    interpolateLine: false, // set to true for rounded lines
});
sciChartSurface.renderableSeries.add(polarMountain);
```

In the code above:

- We create a <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarmountainrenderableseries.html" rel="noopener noreferrer" target="_blank">PolarMountainRenderableSeriesðŸ“˜</a> instance and append it to the renderableSeries collection.
- Add an <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html" rel="noopener noreferrer" target="_blank">XyDataSeriesðŸ“˜</a> to the series, which stores the Xy data to render.
- Note that the line wraps for 1 and a half turns around the angular axis, since it calculates xValues as `xVal % visibleRange.max` and visible range is fixed to (0, 8)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/polar-mountain-renderable-series/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/polar-mountain-renderable-series/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
