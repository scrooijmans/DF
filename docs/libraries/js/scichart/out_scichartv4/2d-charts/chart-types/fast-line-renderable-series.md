On this page

# The Line Series Type

Line Series can be created using the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastlinerenderableseries.html" rel="noopener noreferrer" target="_blank">FastLineRenderableSeriesðŸ“˜</a> type.

![](out_scichartv4/2d-charts/chart-types/fast-line-renderable-series/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

TheÂ <a href="https://www.scichart.com/demo/javascript/line-chart" rel="noopener noreferrer" target="_blank">JavaScript Line Chart Example</a> can be found in theÂ <a href="https://github.com/ABTSoftware/SciChart.JS.Examples/tree/master/Examples/src/components/Examples/Charts2D/BasicChartTypes/LineChart" rel="noopener noreferrer" target="_blank">SciChart.Js Examples Suite &gt; Line Chart</a>Â on Github, or our live demo atÂ <a href="https://www.scichart.com/demo/javascript/line-chart" rel="noopener noreferrer" target="_blank">scichart.com/demo</a>.

Above: The JavaScript <a href="https://www.scichart.com/demo/iframe/line-chart" target="_blank">Line Series Chart</a> example from the <a href="https://www.scichart.com/demo/react" target="_blank">SciChart.js Demo</a>

  

## Create aÂ Line Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-line-renderable-series/#create-aline-series" class="hash-link" aria-label="Direct link to Create aÂ Line Series" translate="no" title="Direct link to Create aÂ Line Series">â€‹</a>

To create aÂ <a href="https://www.scichart.com/demo/javascript-line-chart" rel="noopener noreferrer" target="_blank">Javascript Line Chart</a> with SciChart.js, use the following code:

- TS
- Builder API (JSON Config)

``` prism-code
// Demonstrates how to create a line chart with SciChart.js
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

// Add X and Y axes to the chart
sciChartSurface.xAxes.add(new NumericAxis(wasmContext));
sciChartSurface.yAxes.add(new NumericAxis(wasmContext));

const xValues = [];
const yValues = [];
for (let i = 0; i < 100; i++) {
    xValues.push(i);
    yValues.push(0.2 * Math.sin(i * 0.1) - Math.cos(i * 0.01));
}

// Create a line series with the XY data we generated
const lineSeries = new FastLineRenderableSeries(wasmContext, {
    dataSeries: new XyDataSeries(wasmContext, {
        xValues,
        yValues
    }),
    stroke: "#FF6600",
    strokeThickness: 5,
});

sciChartSurface.renderableSeries.add(lineSeries);
```

``` prism-code
// Demonstrates how to create a line chart with SciChart.js using the Builder API
const { chartBuilder, ESeriesType, EThemeProviderType } = SciChart;
// or, for npm, import { chartBuilder, ... } from "scichart"

const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    surface: { theme: { type: EThemeProviderType.Dark } },
    series: [
        {
            type: ESeriesType.LineSeries,
            xyData: {
                xValues: [0, 1, 2, 3, 4, 5, 6, 7, 8],
                yValues: [2.5, 3.5, 3.7, 4.0, 5.0, 5.5, 5.0, 4.0, 3.0]
            },
            options: {
                stroke: "#0066FF",
                strokeThickness: 5
            }
        }
    ]
});
```

This results in the following:

In the code above:

- AÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastlinerenderableseries.html" rel="noopener noreferrer" target="_blank">FastLineRenderableSeriesðŸ“˜</a> instance is created and added to theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#renderableseries" rel="noopener noreferrer" target="_blank">SciChartSurface.renderableSeriesðŸ“˜</a> collection.
- We set the stroke, strokeThickness properties
- We assign an <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html" rel="noopener noreferrer" target="_blank">XyDataSeriesðŸ“˜</a> as our line's `dataSeries` - which stores the Xy data to render.

## RenderÂ a GapÂ in a Line Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-line-renderable-series/#rendera-gapin-a-line-series" class="hash-link" aria-label="Direct link to RenderÂ a GapÂ in a Line Series" translate="no" title="Direct link to RenderÂ a GapÂ in a Line Series">â€‹</a>

![](out_scichartv4/2d-charts/chart-types/fast-line-renderable-series/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

It is possible to have null points orÂ gapsÂ in a Line Series by passing aÂ data point with a **NaN** valueÂ as the **Y** value.Â Please refer to theÂ [Common Series Features - Draw Gaps in Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/common-series-apis/drawing-gaps)Â article for more details.

## Add Point Markers onto a Line Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-line-renderable-series/#add-point-markers-onto-a-line-series" class="hash-link" aria-label="Direct link to Add Point Markers onto a Line Series" translate="no" title="Direct link to Add Point Markers onto a Line Series">â€‹</a>

It is possible to put scatter point markers of varying type (Ellipse, Square, Triangle, Cross, Custom) onto a Line Series via the PointMarker API. To learn more, see the documentation pageÂ [Drawing PointMarkers on Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/common-series-apis/drawing-point-markers).

![](out_scichartv4/2d-charts/chart-types/fast-line-renderable-series/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

To learn more about the types of Point Marker in SciChart.js, see theÂ [Point Markers API documentation](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/common-series-apis/drawing-point-markers).

ThereÂ is also a dedicatedÂ [Scatter Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/xy-scatter-renderable-series)Â type and aÂ [Bubble Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-bubble-renderable-series)Â type with some more options.

## Painting Line Segments with Different Colors<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-line-renderable-series/#painting-line-segments-with-different-colors" class="hash-link" aria-label="Direct link to Painting Line Segments with Different Colors" translate="no" title="Direct link to Painting Line Segments with Different Colors">â€‹</a>

![](out_scichartv4/2d-charts/chart-types/fast-line-renderable-series/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

It is possible to define the colour of line segments individually using theÂ [PaletteProvider API](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/palette-provider-api-overview).

For more info on how to do this, see theÂ [PaletteProvider - Per-point colouring of Line Charts](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/fast-line-segment-renderable-series/) documentation page.

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-line-renderable-series/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [Per-point Colouring of Line Segments](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/fast-line-renderable-series)

- [The PaletteProvider API](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/palette-provider-api-overview)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/fast-line-renderable-series/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/fast-line-renderable-series/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
