On this page

# The Digital (Step) Line Series

Digital, or Step Line Series can be created using the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastlinerenderableseries.html" rel="noopener noreferrer" target="_blank">FastLineRenderableSeriesðŸ“˜</a> type, and setting the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastlinerenderableseries.html#isdigitalline" rel="noopener noreferrer" target="_blank">isDigitalLineðŸ“˜</a> flag to `true`.

![](out_scichartv4/2d-charts/chart-types/fast-line-digital-renderable-series/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

TheÂ <a href="https://www.scichart.com/demo/javascript/digital-line-chart" rel="noopener noreferrer" target="_blank">JavaScript Digital Line Chart Example</a> can be found in theÂ <a href="https://github.com/ABTSoftware/SciChart.JS.Examples/tree/master/Examples/src/components/Examples/Charts2D/BasicChartTypes/DigitalLineChart" rel="noopener noreferrer" target="_blank">SciChart.Js Examples Suite &gt; Digital Line Chart</a>Â on Github, or our live demo atÂ <a href="https://www.scichart.com/demo/javascript/digital-line-chart" rel="noopener noreferrer" target="_blank">scichart.com/demo</a>.

Above: The JavaScript <a href="https://www.scichart.com/demo/iframe/digital-line-chart" target="_blank">Digital Line Chart</a> example from the <a href="https://www.scichart.com/demo/react" target="_blank">SciChart.js Demo</a>

  

## Create aÂ Digital (Step) Line Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-line-digital-renderable-series/#create-adigital-step-line-series" class="hash-link" aria-label="Direct link to Create aÂ Digital (Step) Line Series" translate="no" title="Direct link to Create aÂ Digital (Step) Line Series">â€‹</a>

To create aÂ <a href="https://www.scichart.com/demo/javascript-digital-line-chart" rel="noopener noreferrer" target="_blank">JavaScript Digital Line Chart</a> with SciChart.js, use the following code:

- TS
- Builder API (JSON Config)

``` prism-code
// Demonstrates how to create a digitral line chart with SciChart.js
const { 
    SciChartSurface, 
    NumericAxis, 
    FastLineRenderableSeries, 
    XyDataSeries, 
    SciChartJsNavyTheme 
} = SciChart;
// or, for npm, import { SciChartSurface, ... } from "scichart"

const { wasmContext, sciChartSurface } = await SciChartSurface.create(divElementId, {
    theme: new SciChartJsNavyTheme()
});

sciChartSurface.xAxes.add(new NumericAxis(wasmContext));
sciChartSurface.yAxes.add(new NumericAxis(wasmContext));

const xValues = [];
const yValues = [];
for (let i = 0; i < 100; i++) {
    xValues.push(i);
    yValues.push(Math.sin(i * 0.1));
}

const lineSeries = new FastLineRenderableSeries(wasmContext, {
    dataSeries: new XyDataSeries(wasmContext, {
        xValues,
        yValues
    }),
    stroke: "#FF6600",
    strokeThickness: 5,
    isDigitalLine: true // enable a digital (step) line
});
sciChartSurface.renderableSeries.add(lineSeries);
```

``` prism-code
// Demonstrates how to create a digital line chart with SciChart.js using the Builder API
const { chartBuilder, ESeriesType, EThemeProviderType } = SciChart;
// or, for npm, import { chartBuilder, ... } from "scichart"

const xValues = [];
const yValues = [];
for (let i = 0; i < 100; i++) {
    xValues.push(i);
    yValues.push(Math.sin(i * 0.1));
}

const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    surface: { theme: { type: EThemeProviderType.Dark } },
    series: [
        {
            type: ESeriesType.LineSeries,
            xyData: {
                xValues,
                yValues
            },
            options: {
                stroke: "#0066FF",
                strokeThickness: 5,
                // set flag isDigitalLine = true to enable a digital (step) line
                isDigitalLine: true
            }
        }
    ]
});
```

This results in the following:

In the code above:

- AÂ Line Series instance is created and added to theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#renderableseries" rel="noopener noreferrer" target="_blank">SciChartSurface.renderableSeriesðŸ“˜</a> collection.
- We set the stroke, strokeThickness properties
- We set the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastlinerenderableseries.html#isdigitalline" rel="noopener noreferrer" target="_blank">isDigitalLineðŸ“˜</a> property to `true` to enable a digital (step) line mode.
- We assign aÂ [DataSeries](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-series-api/data-series-api-overview) - which stores the Xy data to render.

## RenderÂ a GapÂ in a Digital (Step) Line Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-line-digital-renderable-series/#rendera-gapin-a-digital-step-line-series" class="hash-link" aria-label="Direct link to RenderÂ a GapÂ in a Digital (Step) Line Series" translate="no" title="Direct link to RenderÂ a GapÂ in a Digital (Step) Line Series">â€‹</a>

![](out_scichartv4/2d-charts/chart-types/fast-line-digital-renderable-series/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

It is possible to have null points orÂ gapsÂ in aÂ Digital Line Series by passing aÂ data point with a **NaN** valueÂ as the **Y** value.Â Please refer to theÂ [Common Series Features - Draw Gaps in Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/common-series-apis/drawing-gaps)Â article for more details.

## Add Point Markers onto a Digital (Step) Line Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-line-digital-renderable-series/#add-point-markers-onto-a-digital-step-line-series" class="hash-link" aria-label="Direct link to Add Point Markers onto a Digital (Step) Line Series" translate="no" title="Direct link to Add Point Markers onto a Digital (Step) Line Series">â€‹</a>

It is possible to put scatter point markers of varying type (Ellipse, Square, Triangle, Cross, Custom) onto a Digital Line via the PointMarker API. To learn more, see the documentation pageÂ [Drawing PointMarkers on Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/common-series-apis/drawing-point-markers).

![](out_scichartv4/2d-charts/chart-types/fast-line-digital-renderable-series/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

To learn more about the types of Point Marker in SciChart.js, see theÂ [Point Markers API documentation](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/common-series-apis/drawing-point-markers).

ThereÂ is also a dedicatedÂ [Scatter Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/xy-scatter-renderable-series) typeÂ and aÂ [Bubble Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-bubble-renderable-series) typeÂ with some more options.

## Painting Digital Line Segments with Different Colors<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-line-digital-renderable-series/#painting-digital-line-segments-with-different-colors" class="hash-link" aria-label="Direct link to Painting Digital Line Segments with Different Colors" translate="no" title="Direct link to Painting Digital Line Segments with Different Colors">â€‹</a>

![](out_scichartv4/2d-charts/chart-types/fast-line-digital-renderable-series/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

It is possible to define the colour of line segments individually using theÂ [PaletteProvider API](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/palette-provider-api-overview).

For more info on how to do this, see theÂ [PaletteProvider - Per-point colouring of Line Charts](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/fast-line-segment-renderable-series/) documentation page.

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/fast-line-digital-renderable-series/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/fast-line-digital-renderable-series/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
