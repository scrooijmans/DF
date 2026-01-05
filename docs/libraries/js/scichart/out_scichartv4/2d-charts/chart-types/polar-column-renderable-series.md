On this page

# The Polar Column Chart Type

The <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarcolumnrenderableseries.html" rel="noopener noreferrer" target="_blank">PolarColumnRenderableSeriesðŸ“˜</a> creates columns in a polar coordinate system, displaying data as vertical bars positioned at specific angles and radial distances from the center, or as angular bars drawn around the center. This chart type is ideal for visualizing cyclic data or data with angular relationships.

![](out_scichartv4/2d-charts/chart-types/polar-column-renderable-series/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

The <a href="https://www.scichart.com/demo/javascript/polar-column-chart" rel="noopener noreferrer" target="_blank">JavaScript Polar Column Chart</a> can be found in theÂ <a href="https://github.com/ABTSoftware/SciChart.JS.Examples/blob/release_v4.0/Examples/src/components/Examples/Charts2D/PolarCharts/PolarColumnChart" rel="noopener noreferrer" target="_blank">SciChart.Js Examples Suite &gt; Polar Column Chart</a>Â on Github, or our live demo atÂ <a href="https://www.scichart.com/demo/react/polar-column-chart" rel="noopener noreferrer" target="_blank">scichart.com/demo</a>.

Above: The JavaScript <a href="https://www.scichart.com/demo/iframe/polar-column-chart" target="_blank">Polar Column Series Chart</a> example from the <a href="https://www.scichart.com/demo/react" target="_blank">SciChart.js Demo</a>

  

## Properties<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-column-renderable-series/#properties" class="hash-link" aria-label="Direct link to Properties" translate="no" title="Direct link to Properties">â€‹</a>

Some of <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipolarcolumnrenderableseriesoptions.html" rel="noopener noreferrer" target="_blank">IPolarColumnRenderableSeriesOptionsðŸ“˜</a>'s key properties include:

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipolarcolumnrenderableseriesoptions.html#dataseries" rel="noopener noreferrer" target="_blank">dataSeriesðŸ“˜</a> - The <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html" rel="noopener noreferrer" target="_blank">XyDataSeriesðŸ“˜</a>, <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyxdataseries.html" rel="noopener noreferrer" target="_blank">XyxDataSeriesðŸ“˜</a> or <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyxydataseries.html" rel="noopener noreferrer" target="_blank">XyxyDataSeriesðŸ“˜</a> defining the data points of the columns.
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipolarcolumnrenderableseriesoptions.html#fill" rel="noopener noreferrer" target="_blank">fillðŸ“˜</a> - Fill color for the columns
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipolarcolumnrenderableseriesoptions.html#stroke" rel="noopener noreferrer" target="_blank">strokeðŸ“˜</a> - Stroke color for column borders
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipolarcolumnrenderableseriesoptions.html#strokethickness" rel="noopener noreferrer" target="_blank">strokeThicknessðŸ“˜</a> - Thickness of the column borders
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipolarcolumnrenderableseriesoptions.html#datapointwidth" rel="noopener noreferrer" target="_blank">dataPointWidthðŸ“˜</a> - Width of each column
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipolarcolumnrenderableseriesoptions.html#datapointwidthmode" rel="noopener noreferrer" target="_blank">dataPointWidthModeðŸ“˜</a> - How the `dataPointWidth` is applied, see <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/edatapointwidthmode.html" rel="noopener noreferrer" target="_blank">EDataPointWidthModeðŸ“˜</a> for all options.
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipolarcolumnrenderableseriesoptions.html#columnxmode" rel="noopener noreferrer" target="_blank">columnXModeðŸ“˜</a> - How x values are interpreted for column positioning. See <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/ecolumnmode.html" rel="noopener noreferrer" target="_blank">EColumnModeðŸ“˜</a> for more info.
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipolarcolumnrenderableseriesoptions.html#defaulty1" rel="noopener noreferrer" target="_blank">defaultY1ðŸ“˜</a> - Sets the zero line - where the column starts at defaulting at `0`, but is only needed for <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html" rel="noopener noreferrer" target="_blank">XyDataSeriesðŸ“˜</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipolarcolumnrenderableseriesoptions.html#datalabels" rel="noopener noreferrer" target="_blank">dataLabelsðŸ“˜</a> - Configuration for data labels on columns
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipolarcolumnrenderableseriesoptions.html#paletteprovider" rel="noopener noreferrer" target="_blank">paletteProviderðŸ“˜</a> - Custom coloring provider for dynamic styling

## Examples<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-column-renderable-series/#examples" class="hash-link" aria-label="Direct link to Examples" translate="no" title="Direct link to Examples">â€‹</a>

### Basic Angular Polar Column Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-column-renderable-series/#basic-angular-polar-column-series" class="hash-link" aria-label="Direct link to Basic Angular Polar Column Series" translate="no" title="Direct link to Basic Angular Polar Column Series">â€‹</a>

``` prism-code
// Demonstrates how to create a basic polar column chart using SciChart.js
const { 
    SciChartPolarSurface, 
    PolarNumericAxis, 
    SciChartJsNavyTheme,
    PolarColumnRenderableSeries,
    EPolarAxisMode,
    EAxisAlignment,
    EPolarLabelMode,
    NumberRange,
    XyDataSeries, 
} = SciChart;
// or, for npm, import { SciChartSurface, ... } from "scichart"

const { sciChartSurface, wasmContext } = await SciChartPolarSurface.create(divElementId, {
    theme: new SciChartJsNavyTheme(),
});

const angularXAxis = new PolarNumericAxis(wasmContext, {
    polarAxisMode: EPolarAxisMode.Angular, // Angular == "goes around the center, drawn by arcs"
    axisAlignment: EAxisAlignment.Top,
    visibleRange: new NumberRange(0, 20),
    flippedCoordinates: true, // go clockwise
});
sciChartSurface.xAxes.add(angularXAxis);

const radialYAxis = new PolarNumericAxis(wasmContext, {
    polarAxisMode: EPolarAxisMode.Radial, // Radial == "goes from center out, drawn by straight lines"
    axisAlignment: EAxisAlignment.Right,
    visibleRange: new NumberRange(0, 6),
    drawLabels: false, // don't draw labels
    innerRadius: 0.1, // donut hole
});
sciChartSurface.yAxes.add(radialYAxis);

const polarColumn = new PolarColumnRenderableSeries(wasmContext, {
    dataSeries: new XyDataSeries(wasmContext, {
        xValues: Array.from({ length: 20 }, (_, i) => i),
        yValues: Array.from({ length: 20 }, (_, i) => Math.random() * 5 + 1),
    }),
    stroke: "white",
    fill: "#0088FF66",
    strokeThickness: 2,
    dataPointWidth: 1,
    dataLabels: { // optionally - add data labels
        color: "white",
        style: {
            fontSize: 14,
            fontFamily: "Default",
        },
        polarLabelMode: EPolarLabelMode.Parallel,
    },
});
sciChartSurface.renderableSeries.add(polarColumn);
```

In the code above:

- A <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarcolumnrenderableseries.html" rel="noopener noreferrer" target="_blank">PolarColumnRenderableSeriesðŸ“˜</a> instance is created and added to the `sciChartSurface.renderableSeries` collection.
- We assign an <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html" rel="noopener noreferrer" target="_blank">XyDataSeriesðŸ“˜</a> which stores X and Y value arrays.
- We set the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarcolumnrenderableseries.html#stroke" rel="noopener noreferrer" target="_blank">strokeðŸ“˜</a>, <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarcolumnrenderableseries.html#fill" rel="noopener noreferrer" target="_blank">fillðŸ“˜</a>, <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarcolumnrenderableseries.html#strokethickness" rel="noopener noreferrer" target="_blank">strokeThicknessðŸ“˜</a>, and <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarcolumnrenderableseries.html#datapointwidth" rel="noopener noreferrer" target="_blank">dataPointWidthðŸ“˜</a> properties.
- Optional <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarcolumnrenderableseries.html#datalabels" rel="noopener noreferrer" target="_blank">dataLabelsðŸ“˜</a> are configured to display values on each column.

### Polar Radial Polar Column Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-column-renderable-series/#polar-radial-polar-column-series" class="hash-link" aria-label="Direct link to Polar Radial Polar Column Series" translate="no" title="Direct link to Polar Radial Polar Column Series">â€‹</a>

The same renderable series can be used as radial columns by swapping the axis configurations. This creates columns that extend radially outward from the center:

``` prism-code
const radialXAxis = new PolarNumericAxis(wasmContext, {
    polarAxisMode: EPolarAxisMode.Radial, // radial axis -> xAxis
    axisAlignment: EAxisAlignment.Right,
    innerRadius: 0.1,
    startAngle: 0,
});
sciChartSurface.xAxes.add(radialXAxis);

const angularYAxis = new PolarNumericAxis(wasmContext, {
    polarAxisMode: EPolarAxisMode.Angular, // angular axis -> yAxis
    axisAlignment: EAxisAlignment.Top,
    visibleRange: new NumberRange(0, 10),
    useNativeText: true,
    startAngle: 0,
    totalAngle: Math.PI, // 180 degrees
    flippedCoordinates: true, // go clockwise
});
sciChartSurface.yAxes.add(angularYAxis);

// The Polar renderable series do not require extra config, only control the Angular / Radial look.
const polarColumn = new PolarColumnRenderableSeries(wasmContext, {
    dataSeries: new XyDataSeries(wasmContext, {
        xValues: [1, 2, 3, 4, 5, 6],
        yValues: [6.6, 8.7, 3.5, 5.7, 3.8, 6.8]
    }),
    stroke: "white",
    fill: "#0088FF66",
    strokeThickness: 2,
    dataPointWidth: 0.8,
    dataLabels: { // optionally - add data labels
        color: "white",
        style: {
            fontSize: 14,
        },
        polarLabelMode: EPolarLabelMode.Parallel,
        labelYPositionMode: EColumnDataLabelPosition.Outside,
    },
});
sciChartSurface.renderableSeries.add(polarColumn);
```

In the code above:

- The <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarcolumnrenderableseries.html#xaxis" rel="noopener noreferrer" target="_blank">xAxisðŸ“˜</a> is configured with <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/epolaraxismode.html#radial" rel="noopener noreferrer" target="_blank">EPolarAxisMode.RadialðŸ“˜</a> to control the radial positioning.
- The <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarcolumnrenderableseries.html#yaxis" rel="noopener noreferrer" target="_blank">yAxisðŸ“˜</a> is configured with <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/epolaraxismode.html#angular" rel="noopener noreferrer" target="_blank">EPolarAxisMode.AngularðŸ“˜</a> to control the angular positioning.
- The <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarcolumnrenderableseries.html#axisalignment" rel="noopener noreferrer" target="_blank">axisAlignmentðŸ“˜</a> is also swapped.
- (optional) The angular axis now has 180 degrees (1 \* PI radians), meaning a half-circle, due to the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarcolumnrenderableseries.html#totalAngle" rel="noopener noreferrer" target="_blank">totalAngleðŸ“˜</a>.
- (optional) The angular axis also grows clockwise, via <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarcolumnrenderableseries.html#flippedcoordinates" rel="noopener noreferrer" target="_blank">flippedCoordinatesðŸ“˜</a>.

### Column Mode Configuration<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-column-renderable-series/#column-mode-configuration" class="hash-link" aria-label="Direct link to Column Mode Configuration" translate="no" title="Direct link to Column Mode Configuration">â€‹</a>

The <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarcolumnrenderableseries.html#columnxmode" rel="noopener noreferrer" target="_blank">columnXModeðŸ“˜</a> property controls how columns are positioned and sized along the X-axis:

``` prism-code
const polarColumn = new PolarColumnRenderableSeries(wasmContext, {
    dataSeries: new XyxDataSeries(wasmContext, {
        xValues:  [  1,   2,   3, 5, 6.5, 9.5], 
        x1Values: [1.5, 2.5, 4.5, 6,   9, 10 ], // columns go from 1 -> 1.5 // 2 -> 2.5, etc
        yValues: [6.6, 8.7, 3.5, 5.7, 3.8, 6.8], // dictates the height of the column
    }),
    columnXMode: EColumnMode.StartEnd, // go from start to end (x to x1)
    stroke: "white",
    fill: "#0088FF66",
    strokeThickness: 2,
});
sciChartSurface.renderableSeries.add(polarColumn);
```

In the code above:

- We use an <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyxdataseries.html" rel="noopener noreferrer" target="_blank">XyxDataSeriesðŸ“˜</a> with `xValues`, `x1Values`, and `yValues` arrays.
- The <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarcolumnrenderableseries.html#columnxmode" rel="noopener noreferrer" target="_blank">columnXModeðŸ“˜</a> is set to <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/ecolumnmode.html#startend" rel="noopener noreferrer" target="_blank">EColumnMode.StartEndðŸ“˜</a> to define column start and end positions.
- Each column extends from its `xValue` to its `x1Value`, allowing for variable-width columns.

![](out_scichartv4/2d-charts/chart-types/polar-column-renderable-series/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)Different column modes require different data series types:

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/ecolumnmode.html#start" rel="noopener noreferrer" target="_blank">StartðŸ“˜</a>, <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/ecolumnmode.html#mid" rel="noopener noreferrer" target="_blank">MidðŸ“˜</a> - work with <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html" rel="noopener noreferrer" target="_blank">XyDataSeriesðŸ“˜</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/ecolumnmode.html#startend" rel="noopener noreferrer" target="_blank">StartEndðŸ“˜</a>, <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/ecolumnmode.html#startwidth" rel="noopener noreferrer" target="_blank">StartWidthðŸ“˜</a>, and <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/ecolumnmode.html#midwidth" rel="noopener noreferrer" target="_blank">MidWidthðŸ“˜</a> - require <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyxdataseries.html" rel="noopener noreferrer" target="_blank">XyxDataSeriesðŸ“˜</a> or <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyxydataseries.html" rel="noopener noreferrer" target="_blank">XyxyDataSeriesðŸ“˜</a>

### PaletteProvider for Polar Column Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-column-renderable-series/#paletteprovider-for-polar-column-series" class="hash-link" aria-label="Direct link to PaletteProvider for Polar Column Series" translate="no" title="Direct link to PaletteProvider for Polar Column Series">â€‹</a>

By extending <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/defaultpaletteprovider.html" rel="noopener noreferrer" target="_blank">DefaultPaletteProviderðŸ“˜</a> you can create a custom palette for your Polar Column Series, to achieve dynamic coloring based on data values. See more about this topic here [Palette Provider API - Polar Column Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/polar-column-renderable-series).

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/polar-column-renderable-series/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/polar-column-renderable-series/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
