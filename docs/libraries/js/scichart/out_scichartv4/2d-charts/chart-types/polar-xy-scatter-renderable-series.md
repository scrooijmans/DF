On this page

# The Polar Scatter Chart Type

The <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html" rel="noopener noreferrer" target="_blank">PolarXyScatterRenderableSeriesðŸ“˜</a> visualizes discrete points in a polar (circular) coordinate system, using customizable point markers. Each `(x, y)` data point is mapped as an "angle" (theta, usually in degrees or radians) and a "radius" (distance from center), making this chart type ideal for:

- Visualizing measurement data around a circle (e.g., wind direction/speed)
- Radar, sonar, and astronomy applications
- Medical/engineering polar data, and more

Above: The JavaScript <a href="https://www.scichart.com/demo/iframe/polar-scatter-chart" target="_blank">Polar Xy Scatter Series Chart</a> example from the <a href="https://www.scichart.com/demo/react" target="_blank">SciChart.js Demo</a>

  

## Properties<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-xy-scatter-renderable-series/#properties" class="hash-link" aria-label="Direct link to Properties" translate="no" title="Direct link to Properties">â€‹</a>

Key options for <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipolarxyscatterrenderableseriesoptions.html" rel="noopener noreferrer" target="_blank">IPolarXyScatterRenderableSeriesOptionsðŸ“˜</a>:

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipolarxyscatterrenderableseriesoptions.html#dataseries" rel="noopener noreferrer" target="_blank">dataSeriesðŸ“˜</a>: The <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html" rel="noopener noreferrer" target="_blank">XyDataSeriesðŸ“˜</a> containing your X and Y arrays (angle & radius, respectively)
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipolarxyscatterrenderableseriesoptions.html#pointmarker" rel="noopener noreferrer" target="_blank">pointMarkerðŸ“˜</a>: Type, shape, and color of marker (<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/ellipsepointmarker.html" rel="noopener noreferrer" target="_blank">EllipsePointMarkerðŸ“˜</a>, <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/trianglepointmarker.html" rel="noopener noreferrer" target="_blank">TrianglePointMarkerðŸ“˜</a>, etc.)
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipolarxyscatterrenderableseriesoptions.html#paletteprovider" rel="noopener noreferrer" target="_blank">paletteProviderðŸ“˜</a>: Optional for dynamic/color-by-value marker coloring
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#cliptototalangle" rel="noopener noreferrer" target="_blank">clipToTotalAngleðŸ“˜</a>: If set, clips points outside the angular axis' total angle
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipolarxyscatterrenderableseriesoptions.html#datalabels" rel="noopener noreferrer" target="_blank">dataLabelsðŸ“˜</a>: Optionally show and style labels for individual points

## Examples<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-xy-scatter-renderable-series/#examples" class="hash-link" aria-label="Direct link to Examples" translate="no" title="Direct link to Examples">â€‹</a>

### Basic Polar Scatter Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-xy-scatter-renderable-series/#basic-polar-scatter-series" class="hash-link" aria-label="Direct link to Basic Polar Scatter Series" translate="no" title="Direct link to Basic Polar Scatter Series">â€‹</a>

``` prism-code
// Demonstrates how to create a scatter chart with SciChart.js
const {
    SciChartPolarSurface,
    SciChartJsNavyTheme,
    XyDataSeries,
    PolarNumericAxis,
    EPolarAxisMode, 
    NumberRange, 
    EAxisAlignment, 
    EPolarLabelMode,
    PolarXyScatterRenderableSeries,
    TrianglePointMarker,
} = SciChart;
// or, for npm, import { SciChartSurface, ... } from "scichart"

const { sciChartSurface, wasmContext } = await SciChartPolarSurface.create(rootElement, {
    theme: new SciChartJsNavyTheme()
});

const radialYAxis = new PolarNumericAxis(wasmContext, {
    polarAxisMode: EPolarAxisMode.Radial,
    axisAlignment: EAxisAlignment.Right,
    drawLabels: false,
});
sciChartSurface.yAxes.add(radialYAxis);

const angularXAxis = new PolarNumericAxis(wasmContext, {
    polarAxisMode: EPolarAxisMode.Angular,
    axisAlignment: EAxisAlignment.Top,
    polarLabelMode: EPolarLabelMode.Parallel,

    visibleRange: new NumberRange(0, 360),
    autoTicks: false, // Control the tick intervals manually
    majorDelta: 30, // Go from 0 to 360 in steps of 30 degrees

    labelPrecision: 0,
    labelPostfix: `Â°`, // Degree symbol
});
sciChartSurface.xAxes.add(angularXAxis);

const scatterExample = new PolarXyScatterRenderableSeries(wasmContext, {
    dataSeries: new XyDataSeries(wasmContext, {
        xValues: Array.from({ length: 36 }, (_, i) => i * 10),
        yValues: Array.from({ length: 36 }, () => Math.random()),
    }),
    pointMarker: new TrianglePointMarker(wasmContext, {
        width: 10,
        height: 9,
        strokeThickness: 1.5,
        fill: "#000000",
        stroke: "#FF8800",
    }),
});
sciChartSurface.renderableSeries.add(scatterExample);
```

In the code above:

- The <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html" rel="noopener noreferrer" target="_blank">PolarXyScatterRenderableSeriesðŸ“˜</a> is created and linked to an <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html" rel="noopener noreferrer" target="_blank">XyDataSeriesðŸ“˜</a>.
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/trianglepointmarker.html" rel="noopener noreferrer" target="_blank">TrianglePointMarkerðŸ“˜</a> is used to draw each point, but you can substitute with any point marker class.
- Angular axis is configured to have a visible range from `0` to `360` and this coincides with the default <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarnumericaxis.html#totalangledegrees" rel="noopener noreferrer" target="_blank">totalAngleðŸ“˜</a> of `360 degrees`, thus we added the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarnumericaxis.html#labelpostfix" rel="noopener noreferrer" target="_blank">labelPostfixðŸ“˜</a> as `Â°` to further clarify the angle unit.
- The radial axis is used for "distance from center"; here labels are hidden for a minimalist look.

### Polar Scatter with the BuilderAPI schema<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-xy-scatter-renderable-series/#polar-scatter-with-the-builderapi-schema" class="hash-link" aria-label="Direct link to Polar Scatter with the BuilderAPI schema" translate="no" title="Direct link to Polar Scatter with the BuilderAPI schema">â€‹</a>

The Scatter series's <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipolarxyscatterrenderableseriesoptions.html#pointmarker" rel="noopener noreferrer" target="_blank">pointMarkerðŸ“˜</a> property can be configured using a json type - options schema, great for dynamically assigning point marker types for a large dataset.

``` prism-code
const scatterExample = new PolarXyScatterRenderableSeries(wasmContext, {
    dataSeries: new XyDataSeries(wasmContext, {
        xValues: Array.from({ length: 36 }, (_, i) => i * Math.PI / 18),
        yValues: Array.from({ length: 36 }, () => Math.random()),
    }),
    pointMarker: {
        type: EPointMarkerType.Cross,
        options: {
            width: 20,
            height: 8,
            strokeThickness: 2,
            stroke: "#44AAFF",
        }
    }
});
sciChartSurface.renderableSeries.add(scatterExample);
```

## Point Marker Types & Customization<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-xy-scatter-renderable-series/#point-marker-types--customization" class="hash-link" aria-label="Direct link to Point Marker Types &amp; Customization" translate="no" title="Direct link to Point Marker Types &amp; Customization">â€‹</a>

Any pointmarker type available in SciChart.js can be used for polar scatter charts:

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/ellipsepointmarker.html" rel="noopener noreferrer" target="_blank">EllipsePointMarkerðŸ“˜</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/squarepointmarker.html" rel="noopener noreferrer" target="_blank">SquarePointMarkerðŸ“˜</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/trianglepointmarker.html" rel="noopener noreferrer" target="_blank">TrianglePointMarkerðŸ“˜</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/crosspointmarker.html" rel="noopener noreferrer" target="_blank">CrossPointMarkerðŸ“˜</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xpointmarker.html" rel="noopener noreferrer" target="_blank">XPointMarkerðŸ“˜</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/spritepointmarker.html" rel="noopener noreferrer" target="_blank">SpritePointMarkerðŸ“˜</a> for custom image markers
- Styles such as `fill`, `stroke`, `size`, and `strokeThickness` are all customizable.

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/polar-xy-scatter-renderable-series/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/polar-xy-scatter-renderable-series/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
