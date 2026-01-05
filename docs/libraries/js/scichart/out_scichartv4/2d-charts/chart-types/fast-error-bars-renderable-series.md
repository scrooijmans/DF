On this page

# The Error Bars Chart Type

Error Bars can be added to a SciChart.js chart using theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fasterrorbarsrenderableseries.html" rel="noopener noreferrer" target="_blank">FastErrorBarsRenderableSeriesðŸ“˜</a> type.

![](out_scichartv4/2d-charts/chart-types/fast-error-bars-renderable-series/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

The <a href="https://www.scichart.com/demo/javascript-error-bars-chart" rel="noopener noreferrer" target="_blank">JavaScript Error Bars Chart Example</a> can be found in theÂ <a href="https://github.com/ABTSoftware/SciChart.JS.Examples/tree/master/Examples/src/components/Examples/Charts2D/BasicChartTypes/ErrorBarsChart" rel="noopener noreferrer" target="_blank">SciChart.Js Examples Suite &gt; Error Bars Chart</a>Â on Github, or our live demo atÂ <a href="https://www.scichart.com/demo/javascript/error-bars-chart" rel="noopener noreferrer" target="_blank">scichart.com/demo</a>.

Above: The JavaScript <a href="https://www.scichart.com/demo/iframe/error-bars-chart" target="_blank">Error Bars Series Chart</a> example from the <a href="https://www.scichart.com/demo/react" target="_blank">SciChart.js Demo</a>

  

## Creating Error Bars Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-error-bars-renderable-series/#creating-error-bars-series" class="hash-link" aria-label="Direct link to Creating Error Bars Series" translate="no" title="Direct link to Creating Error Bars Series">â€‹</a>

To create aÂ <a href="https://www.scichart.com/demo/javascript-error-bars-chart" rel="noopener noreferrer" target="_blank">Javascript Error Bars Chart</a> with SciChart.js, use the following code:

- TS
- Builder API (JSON Config)

``` prism-code
// Demonstrates how to create a chart with error bars using SciChart.js
const {
    SciChartSurface,
    NumericAxis,
    FastErrorBarsRenderableSeries,
    HlcDataSeries,
    EErrorMode,
    EErrorDirection,
    EDataPointWidthMode,
    SciChartJsNavyTheme
} = SciChart;
// or, for npm, import { SciChartSurface, ... } from "scichart"

const { wasmContext, sciChartSurface } = await SciChartSurface.create(divElementId, {
    theme: new SciChartJsNavyTheme()
});

// Add X and Y axes to the chart
sciChartSurface.xAxes.add(new NumericAxis(wasmContext));
sciChartSurface.yAxes.add(new NumericAxis(wasmContext));

// Create a FastErrorBarsRenderableSeries with HLC data
const errorBarsSeries = new FastErrorBarsRenderableSeries(wasmContext, {
    dataSeries: new HlcDataSeries(wasmContext, {
        xValues: [0, 1, 2, 3, 4, 5, 6, 7, 8],
        yValues: [2.5, 3.5, 3.7, 4.0, 5.0, 5.5, 5.0, 4.0, 3.0], // "Mid" points (pointMarkers are placed here)
        highValues: [3.7, 3.8, 4.0, 5.3, 5.9, 5.7, 5.0, 4.3, 3.2], // highest point of the error bars
        lowValues: [2.2, 3.4, 3.3, 3.8, 5.0, 4.8, 3.5, 3.0, 1.8] // lowest point of the error bars
    }),
    stroke: "#50C7E0",
    strokeThickness: 2,

    // Optional parameters. Defaults are Both, Vertical
    errorMode: EErrorMode.Both,
    errorDirection: EErrorDirection.Vertical,

    // More optional parameters. Defaults are 0.5, Relative
    dataPointWidth: 0.5,
    dataPointWidthMode: EDataPointWidthMode.Relative
});
sciChartSurface.renderableSeries.add(errorBarsSeries);
```

``` prism-code
// Demonstrates how to create a chart with error bars in SciChart.js using the Builder API
const { 
    chartBuilder, 
    ESeriesType, 
    EThemeProviderType,
    EErrorMode,
    EErrorDirection,
    EDataPointWidthMode
} = SciChart;
// or, for npm, import { chartBuilder, ... } from "scichart"

const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    surface: { theme: { type: EThemeProviderType.Dark } },
    series: [
        {
            type: ESeriesType.ErrorBarsSeries,
            hlcData: {
                xValues: [0, 1, 2, 3, 4, 5, 6, 7, 8],
                yValues: [2.5, 3.5, 3.7, 4.0, 5.0, 5.5, 5.0, 4.0, 3.0],
                highValues: [3.7, 3.8, 4.0, 5.3, 5.9, 5.7, 5.0, 4.3, 3.2],
                lowValues: [2.2, 3.4, 3.3, 3.8, 5.0, 4.8, 3.5, 3.0, 1.8]
            },
            options: {
                stroke: "#50C7E0",
                strokeThickness: 3,
                errorMode: EErrorMode.Both,
                errorDirection: EErrorDirection.Vertical,
                dataPointWidth: 0.5,
                dataPointWidthMode: EDataPointWidthMode.Relative
            }
        }
    ]
});
```

This results in the following:

In the code above:

- We defineÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/hlcdataseries.html" rel="noopener noreferrer" target="_blank">HlcDataSeriesðŸ“˜</a>, passing arrays with X, Y, High, and Low values
- AÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fasterrorbarsrenderableseries.html" rel="noopener noreferrer" target="_blank">FastErrorBarsRenderableSeriesðŸ“˜</a> is created with dataSeries option and added to theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#renderableseries" rel="noopener noreferrer" target="_blank">SciChartSurface.renderableSeriesðŸ“˜</a> collection.
- Alternatively we can assign aÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fasterrorbarsrenderableseries.html#dataseries" rel="noopener noreferrer" target="_blank">FastErrorBarsRenderableSeries.dataSeriesðŸ“˜</a> property, which stores the HLC data to render.

## Error Mode<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-error-bars-renderable-series/#error-mode" class="hash-link" aria-label="Direct link to Error Mode" translate="no" title="Direct link to Error Mode">â€‹</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fasterrorbarsrenderableseries.html#errormode" rel="noopener noreferrer" target="_blank">FastErrorBarsRenderableSeries.errorModeðŸ“˜</a> property defines whether high and low caps should be displayed on error bars. Available values are defined by enumÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eerrormode.html" rel="noopener noreferrer" target="_blank">EErrorModeðŸ“˜</a>. We can set this property either via the series itself, or passing in to constructor options (see typeÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ifasterrorbarsrenderableseriesoptions.html" rel="noopener noreferrer" target="_blank">IFastErrorBarsRenderableSeriesOptionsðŸ“˜</a>).

The following example shows how to use only **high** error on bars.

``` prism-code
const errorBarsSeries = new FastErrorBarsRenderableSeries(wasmContext, {
    dataSeries: hlcDataSeries,
    errorMode: EErrorMode.High,
});
```

This results in the following output:

<img src="out_scichartv4/2d-charts/chart-types/fast-error-bars-renderable-series/index_media/d15d7736df7d4ac7832aa86e928a782a6098d1fe.png" title="Error Bars Series with High Error Segments Only" style="display:block;margin-left:auto;margin-right:auto;object-fit:contain;height:auto;width:85%;margin:0 auto" alt="Error Bars Series with High Error Segments Only" />

Error Bars Series with High Error Segments Only

## Horizontal Error Bars<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-error-bars-renderable-series/#horizontal-error-bars" class="hash-link" aria-label="Direct link to Horizontal Error Bars" translate="no" title="Direct link to Horizontal Error Bars">â€‹</a>

It is possible to change the direction of Error Bars to horizontal usingÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fasterrorbarsrenderableseries.html#errordirection" rel="noopener noreferrer" target="_blank">FastErrorBarsRenderableSeries.errorDirectionðŸ“˜</a> property. In this case the High & Low values inÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/hlcdataseries.html" rel="noopener noreferrer" target="_blank">HlcDataSeriesðŸ“˜</a>Â are applied in the X-direction.

``` prism-code
import { EErrorDirection } from "scichart";
// ...

const horizontalBars = new FastErrorBarsRenderableSeries(wasmContext, {
    dataSeries: new HlcDataSeries(wasmContext, {
        xValues: [0, 1, 2, 2.5, 4.5, 5, 6, 7, 8],
        yValues: [2.5, 3.5, 3.7, 4.0, 5.0, 5.5, 5.0, 4.0, 3.0],
        // High, Low becomes left-right as absolute values
        highValues: [-0.5, 0.6, 1.1, 2.3, 4.0, 4.9, 5.8, 6.8, 7.5],
        lowValues: [0.4, 1.2, 2.1, 3.0, 4.7, 5.7, 6.5, 7.3, 8.9],
    }),
    errorDirection: EErrorDirection.Horizontal,
});
```

This results in the following output

<img src="out_scichartv4/2d-charts/chart-types/fast-error-bars-renderable-series/index_media/ae9d35d5e47fc597d4f1cf50c10467dcdb8cc5c1.png" title="Horizontal ErrorBars" style="display:block;margin-left:auto;margin-right:auto;object-fit:contain;height:auto;width:85%;margin:0 auto" alt="Horizontal ErrorBars" />

Horizontal ErrorBars

## Advanced Properties<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-error-bars-renderable-series/#advanced-properties" class="hash-link" aria-label="Direct link to Advanced Properties" translate="no" title="Direct link to Advanced Properties">â€‹</a>

### Setting Stroke, StrokeThickness and StrokeDash<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-error-bars-renderable-series/#setting-stroke-strokethickness-and-strokedash" class="hash-link" aria-label="Direct link to Setting Stroke, StrokeThickness and StrokeDash" translate="no" title="Direct link to Setting Stroke, StrokeThickness and StrokeDash">â€‹</a>

Error Bars in SciChart.js support settingÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fasterrorbarsrenderableseries.html#stroke" rel="noopener noreferrer" target="_blank">strokeðŸ“˜</a>,Â <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fasterrorbarsrenderableseries.html#strokethickness" rel="noopener noreferrer" target="_blank">strokeThicknessðŸ“˜</a> andÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fasterrorbarsrenderableseries.html#strokeddasharray" rel="noopener noreferrer" target="_blank">strokeDashArrayðŸ“˜</a> to set a dashed line style.

For example, this code:

``` prism-code
const errorBarsSeries = new FastErrorBarsRenderableSeries(wasmContext, {
    dataSeries: hlcDataSeries,
    stroke: "Aqua",
    strokeDashArray: [4, 2],
    strokeThickness: 4,
});
```

Results in this:

<img src="out_scichartv4/2d-charts/chart-types/fast-error-bars-renderable-series/index_media/886dfa166d947c8fdf5d42ddc9a19238ad815e09.png" title="Advanced properties on Error Bars - stroke, strokeThickness, strokeDashArray" style="display:block;margin-left:auto;margin-right:auto;object-fit:contain;height:auto;width:85%;margin:0 auto" alt="Advanced properties on Error Bars - stroke, strokeThickness, strokeDashArray" />

Advanced properties on Error Bars - stroke, strokeThickness, strokeDashArray

![](out_scichartv4/2d-charts/chart-types/fast-error-bars-renderable-series/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

Learn more about Dashed Line Styling at thisÂ [documentation page](https://www.scichart.com/documentation/js/v4/2d-charts/styling-and-theming/style-chart-parts-in-code).

### Drawing a Line and PointMarker through Error Bars<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-error-bars-renderable-series/#drawing-a-line-and-pointmarker-through-error-bars" class="hash-link" aria-label="Direct link to Drawing a Line and PointMarker through Error Bars" translate="no" title="Direct link to Drawing a Line and PointMarker through Error Bars">â€‹</a>

You can add a PointMarker to highlight the Y-value of anÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/hlcdataseries.html" rel="noopener noreferrer" target="_blank">HlcDataSeriesðŸ“˜</a>. The data can also be used twice to display both lines / scatter points and error bars.

Try this code out:

``` prism-code
import { FastLineRenderableSeries, FastErrorBarsRenderableSeries, HlcDataSeries, EllipsePointMarker  } from "scichart";

// Many RenderableSeries can share one DataSeries in SciChart

// Use the HlcDataSeries on a FastLineRenderableSeries
const lineSeries = new FastLineRenderableSeries(wasmContext, {
    dataSeries: hlcDataSeries, // will render XY values, and ignore high/low values
    stroke: "SteelBlue"
});

// Use the same HlcDataSeries on a FastErrorBarsRenderableSeries. It will render high/lows for error bars
const errorBarsSeries = new FastErrorBarsRenderableSeries(wasmContext, {
    dataSeries: hlcDataSeries,
    stroke: "SteelBlue",
    strokeThickness: 3,
    opacity: 0.77,
    pointMarker: new EllipsePointMarker(wasmContext, {
        width: 10,
        height: 10,
        fill: "SteelBlue",
        stroke: "#333"
    })
});

sciChartSurface.renderableSeries.add(lineSeries);
sciChartSurface.renderableSeries.add(errorBarsSeries);
```

This results in the folllowing output:

<img src="out_scichartv4/2d-charts/chart-types/fast-error-bars-renderable-series/index_media/8dd73f119880d364ebbb4c74764aab536c38d769.png" title="Drawing a Line and PointMarkers through Error Bars" style="display:block;margin-left:auto;margin-right:auto;object-fit:contain;height:auto;width:85%;margin:0 auto" alt="Drawing a Line and PointMarkers through Error Bars" />

Drawing a Line and PointMarkers through Error Bars

### Setting the Width of Error Bars Whiskers<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-error-bars-renderable-series/#setting-the-width-of-error-bars-whiskers" class="hash-link" aria-label="Direct link to Setting the Width of Error Bars Whiskers" translate="no" title="Direct link to Setting the Width of Error Bars Whiskers">â€‹</a>

You can define the width of Error Bars usingÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fasterrorbarsrenderableseries.html#datapointwidth" rel="noopener noreferrer" target="_blank">dataPointWidthðŸ“˜</a> andÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fasterrorbarsrenderableseries.html#datapointwidthmode" rel="noopener noreferrer" target="_blank">dataPointWidthModeðŸ“˜</a>Â which supports values fromÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/edatapointwidthmode.html" rel="noopener noreferrer" target="_blank">EDataPointModeðŸ“˜</a>.

For example. setting dataPointWidthMode to Absolute and dataPointWidth to 50 ensures 50 pixels always.

``` prism-code
const errorBarsSeries = new FastErrorBarsRenderableSeries(wasmContext, {
    dataSeries: hlcDataSeries,
    stroke: "LightSteelBlue",
    strokeThickness: 3,
    dataPointWidth: 50,
    dataPointWidthMode: EDataPointWidthMode.Absolute,
});
sciChartSurface.renderableSeries.add(errorBarsSeries);
```

<img src="out_scichartv4/2d-charts/chart-types/fast-error-bars-renderable-series/index_media/971c1a8071033a4360a9f5582f200ed15add2f00.png" title="Setting the Width of Error Bars Whiskers" style="display:block;margin-left:auto;margin-right:auto;object-fit:contain;height:auto;width:85%;margin:0 auto" alt="Setting the Width of Error Bars Whiskers" />

Setting the Width of Error Bars Whiskers

The alternative mode isÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/edatapointwidthmode.html" rel="noopener noreferrer" target="_blank">EDataPointWidthMode.RelativeðŸ“˜</a>.

In this mode a dataPointWidth value of `0.0` - `1.0` occupies 0% to 100% of the available space as calculated by SciChart.js.

``` prism-code
const errorBarsSeries = new FastErrorBarsRenderableSeries(wasmContext, {
    dataSeries: hlcDataSeries,
    stroke: "LightSteelBlue",
    strokeThickness: 3,
    dataPointWidth: 0.2,
    dataPointWidthMode: EDataPointWidthMode.Relative,
});
sciChartSurface.renderableSeries.add(errorBarsSeries);
```

<img src="out_scichartv4/2d-charts/chart-types/fast-error-bars-renderable-series/index_media/969f86be508e8a3a630be88b21590c949e4ead37.png" title="The Relative Width of Error Bars Whiskers" style="display:block;margin-left:auto;margin-right:auto;object-fit:contain;height:auto;width:85%;margin:0 auto" alt="The Relative Width of Error Bars Whiskers" />

The Relative Width of Error Bars Whiskers

### Showing/Hiding Error Bars Connector or WhiskersÂ <a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-error-bars-renderable-series/#showinghiding-error-bars-connector-or-whiskers" class="hash-link" aria-label="Direct link to Showing/Hiding Error Bars Connector or WhiskersÂ " translate="no" title="Direct link to Showing/Hiding Error Bars Connector or WhiskersÂ ">â€‹</a>

You can specify which parts of an Error Bar should be visible usingÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fasterrorbarsrenderableseries.html#drawconnector" rel="noopener noreferrer" target="_blank">drawConnectorðŸ“˜</a> andÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fasterrorbarsrenderableseries.html#drawwhiskers" rel="noopener noreferrer" target="_blank">drawWhiskersðŸ“˜</a> and finally;

``` prism-code
const errorBarsSeries = new FastErrorBarsRenderableSeries(wasmContext, {
    dataSeries: hlcDataSeries,
    stroke: "LightSteelBlue",
    strokeThickness: 3,
    drawConnector: false, // Draw the horizontal connector
    drawWhiskers: true,   // Draw the top/bottom whiskers
});
sciChartSurface.renderableSeries.add(errorBarsSeries);
```

Results in this:

<img src="out_scichartv4/2d-charts/chart-types/fast-error-bars-renderable-series/index_media/a0c1eff3868356bda7e409900c4a5279b4447eb4.png" title="Hiding Parts of Error Bars - Connector / Whiskers" style="display:block;margin-left:auto;margin-right:auto;object-fit:contain;height:auto;width:85%;margin:0 auto" alt="Hiding Parts of Error Bars - Connector / Whiskers" />

Hiding Parts of Error Bars - Connector / Whiskers

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/fast-error-bars-renderable-series/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/fast-error-bars-renderable-series/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
