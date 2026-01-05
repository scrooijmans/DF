On this page

# The Digital (Step) Band Series Type

A Digital Band Series, or High-Low Fill between two Digital or Step lines can be created using theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastbandrenderableseries.html" rel="noopener noreferrer" target="_blank">FastBandRenderableSeriesðŸ“˜</a> type.

![](out_scichartv4/2d-charts/chart-types/fast-band-digital-renderable-series/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

The <a href="https://www.scichart.com/demo/react/digital-band-chart" rel="noopener noreferrer" target="_blank">JavaScript Digital Band Chart Example</a>Â can be found in theÂ <a href="https://github.com/ABTSoftware/SciChart.JS.Examples/tree/master/Examples/src/components/Examples/Charts2D/BasicChartTypes/DigitalBandSeriesChart" rel="noopener noreferrer" target="_blank">SciChart.Js Examples Suite &gt; Band Series</a>Â on Github, or our live demo atÂ <a href="https://www.scichart.com/demo/javascript/digital-band-chart" rel="noopener noreferrer" target="_blank">scichart.com/demo</a>.

Above: The JavaScript <a href="https://www.scichart.com/demo/iframe/digital-band-chart" target="_blank">Digital Band Chart</a> example from the <a href="https://www.scichart.com/demo/react" target="_blank">SciChart.js Demo</a>

  

## Create aÂ Digital Band Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-band-digital-renderable-series/#create-adigital-band-series" class="hash-link" aria-label="Direct link to Create aÂ Digital Band Series" translate="no" title="Direct link to Create aÂ Digital Band Series">â€‹</a>

To create aÂ <a href="https://www.scichart.com/demo/javascript-digital-band-chart" rel="noopener noreferrer" target="_blank">Javascript Digital Band Chart</a> with SciChart.js, use the following code:

- TS
- Builder API (JSON Config)

``` prism-code
// Demonstrates how to create a band chart using SciChart.js
const { 
    SciChartSurface, 
    NumericAxis, 
    FastBandRenderableSeries, 
    XyyDataSeries, 
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
const y1Values = [];
const POINTS = 100;
const STEP = (3 * Math.PI) / POINTS;
for (let i = 0; i <= POINTS; i++) {
    const k = 1 - i / (POINTS * 2);
    xValues.push(i);
    yValues.push(Math.sin(i * STEP) * k * 0.7);
    y1Values.push(Math.cos(i * STEP) * k);
}

const bandSeries = new FastBandRenderableSeries(wasmContext, {
    dataSeries: new XyyDataSeries(wasmContext, { 
        xValues, 
        yValues,
        y1Values 
    }),
    stroke: "#F48420",
    strokeY1: "#50C7E0",
    fill: "#F4842033",
    fillY1: "#50C7E033",
    strokeThickness: 2,
    // optional parameter defines a step-line
    isDigitalLine: true
});
sciChartSurface.renderableSeries.add(bandSeries);
```

``` prism-code
// Demonstrates how to create a band chart with SciChart.js using the Builder API
const { chartBuilder, ESeriesType, EThemeProviderType } = SciChart;
// or, for npm, import { chartBuilder, ... } from "scichart"

const xValues = [];
const yValues = [];
const y1Values = [];
const POINTS = 100;
const STEP = (3 * Math.PI) / POINTS;
for (let i = 0; i <= POINTS; i++) {
    const k = 1 - i / (POINTS * 2);
    xValues.push(i);
    yValues.push(Math.sin(i * STEP) * k * 0.7);
    y1Values.push(Math.cos(i * STEP) * k);
}

const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    series: [
        {
            type: ESeriesType.BandSeries,
            xyyData: {
                xValues,
                yValues,
                y1Values
            },
            options: {
                stroke: "#FF1919FF",
                strokeY1: "#279B27FF",
                fill: "#279B2733",
                fillY1: "#FF191933",
                strokeThickness: 2,
                // optional parameter defines a step-line
                isDigitalLine: true
            }
        }
    ]
});
```

This results in the following output:

In the code above:

- AÂ Band Series instance is created and added to theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#renderableseries" rel="noopener noreferrer" target="_blank">SciChartSurface.renderableSeriesðŸ“˜</a> collection.
- We set the stroke, fill properties for when Y \> Y1 andÂ vice versaÂ (more info over atÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastbandrenderableseries.html" rel="noopener noreferrer" target="_blank">FastBandRenderableSeriesðŸ“˜</a> in TypeDoc).
- We set the **isDigitalLine** property to true.
- We assign aÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastbandrenderableseries.html#dataseries" rel="noopener noreferrer" target="_blank">DataSeriesðŸ“˜</a> - in this case anÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyydataseries.html" rel="noopener noreferrer" target="_blank">XyyDataSeriesðŸ“˜</a> which stores X, Y, Y1 data.

## RenderÂ a GapÂ in a Digital Band Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-band-digital-renderable-series/#rendera-gapin-a-digital-band-series" class="hash-link" aria-label="Direct link to RenderÂ a GapÂ in a Digital Band Series" translate="no" title="Direct link to RenderÂ a GapÂ in a Digital Band Series">â€‹</a>

![](out_scichartv4/2d-charts/chart-types/fast-band-digital-renderable-series/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

It is possible to have null points orÂ gapsÂ in a Digital Band Series by passing aÂ data point with a **NaN** valueÂ as the **Y** value.Â Please refer to theÂ [Common Series Features - Draw Gaps in Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/common-series-apis/drawing-gaps)Â article for more details.

## Add Point Markers onto a Band Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-band-digital-renderable-series/#add-point-markers-onto-a-band-series" class="hash-link" aria-label="Direct link to Add Point Markers onto a Band Series" translate="no" title="Direct link to Add Point Markers onto a Band Series">â€‹</a>

It is possible to put scatter point markers of varying type (Ellipse, Square, Triangle, Cross, Custom) onto a Band Series via the PointMarker API. To learn more, see the documentation pageÂ [Drawing PointMarkers on Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/common-series-apis/drawing-point-markers).

![](out_scichartv4/2d-charts/chart-types/fast-band-digital-renderable-series/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

To learn more about the types of Point Marker in SciChart.js, see theÂ [Point Markers API documentation](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/common-series-apis/drawing-point-markers).

ThereÂ is also a dedicatedÂ [Scatter Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/xy-scatter-renderable-series) typeÂ and aÂ [Bubble Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-bubble-renderable-series)Â type with some more options.

## Painting Band Segments with Different Colors<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-band-digital-renderable-series/#painting-band-segments-with-different-colors" class="hash-link" aria-label="Direct link to Painting Band Segments with Different Colors" translate="no" title="Direct link to Painting Band Segments with Different Colors">â€‹</a>

![](out_scichartv4/2d-charts/chart-types/fast-band-digital-renderable-series/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

It is possible to define the colour of band segments individually using theÂ [PaletteProvider API](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/palette-provider-api-overview).

For more info on how to do this, see theÂ [PaletteProvider - Per-point colouring of Band Charts](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/fast-band-renderable-series) documentation page.

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-band-digital-renderable-series/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [The Band Series type](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-band-renderable-series)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/fast-band-digital-renderable-series/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/fast-band-digital-renderable-series/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
