On this page

# The Mountain (Area) Series Type

Mountain (or Area) Series can be created using theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastmountainrenderableseries.html" rel="noopener noreferrer" target="_blank">FastMountainRenderableSeriesðŸ“˜</a> type.

![](out_scichartv4/2d-charts/chart-types/fast-mountain-area-renderable-series/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

TheÂ <a href="https://www.scichart.com/demo/javascript/mountain-chart" rel="noopener noreferrer" target="_blank">JavaScript Mountain Chart Example</a> can be found in theÂ <a href="https://github.com/ABTSoftware/SciChart.JS.Examples/tree/master/Examples/src/components/Examples/Charts2D/BasicChartTypes/MountainChart" rel="noopener noreferrer" target="_blank">SciChart.Js Examples Suite &gt; Mountain Chart</a>Â on Github, or our live demo atÂ <a href="https://www.scichart.com/demo/javascript/mountain-chart" rel="noopener noreferrer" target="_blank">scichart.com/demo</a>.

Above: The JavaScript <a href="https://www.scichart.com/demo/iframe/mountain-chart" target="_blank">Mountain Series Chart</a> example from the <a href="https://www.scichart.com/demo/react" target="_blank">SciChart.js Demo</a>

  

## Create aÂ Mountain Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-mountain-area-renderable-series/#create-amountain-series" class="hash-link" aria-label="Direct link to Create aÂ Mountain Series" translate="no" title="Direct link to Create aÂ Mountain Series">â€‹</a>

To create aÂ <a href="https://www.scichart.com/demo/javascript-mountain-chart" rel="noopener noreferrer" target="_blank">Javascript Mountain Chart</a> with SciChart.js, use the following code:

- TS
- Builder API (JSON Config)

``` prism-code
// Demonstrates how to create a Mountain (Area) chart with SciChart.js
const {
    SciChartSurface,
    NumericAxis,
    FastMountainRenderableSeries,
    GradientParams,
    XyDataSeries,
    Point,
    SciChartJsNavyTheme
} = SciChart;
// or, for npm, import { SciChartSurface, ... } from "scichart"

const { wasmContext, sciChartSurface } = await SciChartSurface.create(divElementId, {
    theme: new SciChartJsNavyTheme()
});
sciChartSurface.xAxes.add(new NumericAxis(wasmContext));
sciChartSurface.yAxes.add(new NumericAxis(wasmContext));

// Create some data
let yLast = 100.0;
const xValues = [];
const yValues = [];
for (let i = 0; i <= 250; i++) {
    const y = yLast + (Math.random() - 0.48);
    yLast = y;
    xValues.push(i);
    yValues.push(y);
}

// Create a mountain series & add to the chart
const mountainSeries = new FastMountainRenderableSeries(wasmContext, {
    dataSeries: new XyDataSeries(wasmContext, { xValues, yValues }),
    stroke: "#4682b4",
    strokeThickness: 3,
    zeroLineY: 0.0,
    // when a solid color is required, use fill
    fill: "rgba(176, 196, 222, 0.7)",
    // when a gradient is required, use fillLinearGradient
    fillLinearGradient: new GradientParams(new Point(0, 0), new Point(0, 1), [
        { color: "rgba(70,130,180,0.77)", offset: 0 },
        { color: "rgba(70,130,180,0.0)", offset: 1 }
    ])
});

sciChartSurface.renderableSeries.add(mountainSeries);
```

``` prism-code
// Demonstrates how to create a line chart with SciChart.js using the Builder API
const { chartBuilder, ESeriesType, EThemeProviderType } = SciChart;
// or, for npm, import { chartBuilder, ... } from "scichart"

// Create some data
let yLast = 100.0;
const xValues = [];
const yValues = [];
for (let i = 0; i <= 250; i++) {
    const y = yLast + (Math.random() - 0.48);
    yLast = y;
    xValues.push(i);
    yValues.push(y);
}

const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    series: [
        {
            type: ESeriesType.MountainSeries,
            xyData: {
                xValues,
                yValues
            },
            options: {
                stroke: "#4682b4",
                strokeThickness: 3,
                zeroLineY: 0.0,
                fill: "rgba(176, 196, 222, 0.7)", // when a solid color is required, use fill
                fillLinearGradient: {
                    gradientStops: [
                        { color: "rgba(70,130,180,0.77)", offset: 0.0 },
                        { color: "rgba(70,130,180,0.0)", offset: 1 }
                    ],
                    startPoint: { x: 0, y: 0 },
                    endPoint: { x: 0, y: 1 }
                }
            }
        }
    ]
});
```

This results in the following output:

In the code above:

- AÂ Mountain Series instance is created and added to theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#renderableseries" rel="noopener noreferrer" target="_blank">SciChartSurface.renderableSeriesðŸ“˜</a> collection.
- We set the stroke, strokethickness and fillÂ properties
- ZeroLineY defines where the zero crossing is. The default is `0.0`
- We assign aÂ [DataSeries](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-series-api/data-series-api-overview) - which stores the Xy data to render.

## RenderÂ a GapÂ in a Mountain Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-mountain-area-renderable-series/#rendera-gapin-a-mountain-series" class="hash-link" aria-label="Direct link to RenderÂ a GapÂ in a Mountain Series" translate="no" title="Direct link to RenderÂ a GapÂ in a Mountain Series">â€‹</a>

![](out_scichartv4/2d-charts/chart-types/fast-mountain-area-renderable-series/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

It is possible to have null points orÂ gapsÂ in a Mountain Series by passing aÂ data point with a **NaN** valueÂ as the **Y** value.Â Please refer to theÂ [Common Series Features - Draw Gaps in Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/common-series-apis/drawing-gaps)Â article for more details.

## Add Point Markers onto a Mountain Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-mountain-area-renderable-series/#add-point-markers-onto-a-mountain-series" class="hash-link" aria-label="Direct link to Add Point Markers onto a Mountain Series" translate="no" title="Direct link to Add Point Markers onto a Mountain Series">â€‹</a>

It is possible to put scatter point markers of varying type (Ellipse, Square, Triangle, Cross, Custom) onto a Mountain Series via the PointMarker API. To learn more, see the documentation pageÂ [Drawing PointMarkers on Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/common-series-apis/drawing-point-markers).

![](out_scichartv4/2d-charts/chart-types/fast-mountain-area-renderable-series/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

To learn more about the types of Point Marker in SciChart.js, see theÂ [Point Markers API documentation](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/common-series-apis/drawing-point-markers).

ThereÂ is also a dedicatedÂ [Scatter Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/xy-scatter-renderable-series) typeÂ and aÂ [Bubble Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-bubble-renderable-series) typeÂ with some more options.

## Painting Mountain Segments with Different Colors<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-mountain-area-renderable-series/#painting-mountain-segments-with-different-colors" class="hash-link" aria-label="Direct link to Painting Mountain Segments with Different Colors" translate="no" title="Direct link to Painting Mountain Segments with Different Colors">â€‹</a>

![](out_scichartv4/2d-charts/chart-types/fast-mountain-area-renderable-series/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

It is possible to define the colour of line segments as well as mountain fills individually using theÂ [PaletteProvider API](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/palette-provider-api-overview).

For more info on how to do this, see theÂ [PaletteProvider - Per-point colouring of Mountain Charts](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/fast-mountain-renderable-series) documentation page.

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-mountain-area-renderable-series/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [Start Here - RenderableSeries Overview](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/renderable-series-api-overview)

- [Common RenderableSeries Properties](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/common-series-apis/is-visible)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/fast-mountain-area-renderable-series/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/fast-mountain-area-renderable-series/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
