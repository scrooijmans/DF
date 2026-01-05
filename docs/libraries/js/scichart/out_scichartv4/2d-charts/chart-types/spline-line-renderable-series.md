On this page

# The Spline (Smoothed) Line Series Type

Spline Line or Smoothed Series can be created using theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/splinelinerenderableseries.html" rel="noopener noreferrer" target="_blank">SplineLineRenderableSeriesðŸ“˜</a> type.

![](out_scichartv4/2d-charts/chart-types/spline-line-renderable-series/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

The <a href="https://www.scichart.com/demo/javascript/spline-line-chart" rel="noopener noreferrer" target="_blank">JavaScript Spline Line Chart Example</a> can be found in theÂ <a href="https://github.com/ABTSoftware/SciChart.JS.Examples/tree/master/Examples/src/components/Examples/Charts2D/BasicChartTypes/SplineLineChart" rel="noopener noreferrer" target="_blank">SciChart.Js Examples Suite &gt; Spline Line Chart</a>Â on Github, or our live demo atÂ <a href="https://www.scichart.com/demo/javascript/spline-line-chart" rel="noopener noreferrer" target="_blank">scichart.com/demo</a>

Above: The JavaScript <a href="https://www.scichart.com/demo/iframe/spline-line-chart" target="_blank">Spline Line Chart</a> example from the <a href="https://www.scichart.com/demo/react" target="_blank">SciChart.js Demo</a>

  

## Create aÂ Spline Line Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/spline-line-renderable-series/#create-aspline-line-series" class="hash-link" aria-label="Direct link to Create aÂ Spline Line Series" translate="no" title="Direct link to Create aÂ Spline Line Series">â€‹</a>

To create aÂ <a href="https://www.scichart.com/demo/javascript-spline-line-chart" rel="noopener noreferrer" target="_blank">Javascript Spline Line Chart</a> with SciChart.js, use the following code:

- TS
- Builder API (JSON Config)

``` prism-code
// Demonstrates how to create a line chart with SciChart.js
const {
    SciChartSurface,
    NumericAxis,
    SplineLineRenderableSeries,
    EllipsePointMarker,
    XyDataSeries,
    SciChartJsNavyTheme,
    NumberRange
} = SciChart;
// or, for npm, import { SciChartSurface, ... } from "scichart"

const { wasmContext, sciChartSurface } = await SciChartSurface.create(divElementId, {
    theme: new SciChartJsNavyTheme()
});
sciChartSurface.xAxes.add(new NumericAxis(wasmContext, { growBy: new NumberRange(0.1, 0.1) }));
sciChartSurface.yAxes.add(new NumericAxis(wasmContext, { growBy: new NumberRange(0.1, 0.1) }));

// Create a spline line series
const splineLineSeries = new SplineLineRenderableSeries(wasmContext, {
    dataSeries: new XyDataSeries(wasmContext, {
        xValues: Array.from({length: 10}, (_, i) => i), // [0, 1, 2, ..., 9]
        yValues: Array.from({length: 10}, (_, i) => 0.2 * Math.sin(i) - Math.cos(i * 0.12))
    }),
    stroke: "#FF6600",
    strokeThickness: 5,
    // Set interpolation points to decide the amount of smoothing - the larger the number, the smoother the line
    interpolationPoints: 20,
    // Add a pointmarker to show where the datapoints are
    pointMarker: new EllipsePointMarker(wasmContext, {
        width: 7,
        height: 7,
        fill: "white",
        strokeThickness: 0
    })
});

sciChartSurface.renderableSeries.add(splineLineSeries);
```

``` prism-code
// Demonstrates how to create a line chart with SciChart.js using the Builder API
const { chartBuilder, ESeriesType, EThemeProviderType, EPointMarkerType } = SciChart;
// or, for npm, import { chartBuilder, ... } from "scichart"

const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    surface: { theme: { type: EThemeProviderType.Dark } },
    series: [
        {
            type: ESeriesType.SplineLineSeries,
            xyData: {
                xValues: Array.from({length: 10}, (_, i) => i), // [0, 1, 2, ..., 9]
                yValues: Array.from({length: 10}, (_, i) => 0.2 * Math.sin(i) - Math.cos(i * 0.12))
            },
            options: {
                stroke: "#FF6600",
                strokeThickness: 5,
                interpolationPoints: 20, // the larger the number, the smoother the line
                pointMarker: {
                    type: EPointMarkerType.Ellipse,
                    options: {
                        width: 7,
                        height: 7,
                        fill: "white",
                        strokeThickness: 0
                    }
                }
            }
        }
    ]
});
```

In the code above:

- AÂ Spline Line Series instance is created and added to theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#renderableseries" rel="noopener noreferrer" target="_blank">SciChartSurface.renderableSeriesðŸ“˜</a> collection.
- We set the stroke, strokethickness properties
- We assign aÂ [DataSeries](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-series-api/data-series-api-overview) - which stores the Xy data to render.
- We set the number ofÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/splinelinerenderableseries.html#interpolationpoints" rel="noopener noreferrer" target="_blank">interpolationPointsðŸ“˜</a> - how many points between real Xy data points will be interpolated using a Spline interpolation algorithm.

This results in the following output:

## Performance Tips in Spline Lines<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/spline-line-renderable-series/#performance-tips-in-spline-lines" class="hash-link" aria-label="Direct link to Performance Tips in Spline Lines" translate="no" title="Direct link to Performance Tips in Spline Lines">â€‹</a>

![](out_scichartv4/2d-charts/chart-types/spline-line-renderable-series/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

When theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/splinelinerenderableseries.html#interpolationpoints" rel="noopener noreferrer" target="_blank">SplineLineRenderableSeries.interpolationPointsðŸ“˜</a> property is set to zero, then this series renders and displays exactly like a FastLineRenderableSeries.

![](out_scichartv4/2d-charts/chart-types/spline-line-renderable-series/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

When theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/splinelinerenderableseries.html#interpolationpoints" rel="noopener noreferrer" target="_blank">interpolationPointsðŸ“˜</a> property is set to another number, e.g. 10, then SciChart.js will calculate 10 points for each Xy datapoint you add to the XyDataSeries. This means you will be displaying 10x the number of datapoints.

*SciChart.js can handle millions of datapoints, but this is something to be aware of. You might want to adjust down the interpolationPoints depending on amount of data on the chart, or zoom level.*

## RenderÂ a GapÂ in a Spline Line Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/spline-line-renderable-series/#rendera-gapin-a-spline-line-series" class="hash-link" aria-label="Direct link to RenderÂ a GapÂ in a Spline Line Series" translate="no" title="Direct link to RenderÂ a GapÂ in a Spline Line Series">â€‹</a>

![](out_scichartv4/2d-charts/chart-types/spline-line-renderable-series/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

It is possible to have null points orÂ gapsÂ in a Line Series by passing aÂ data point with a **NaN** valueÂ as the **Y** value.Â Please refer to theÂ [Common Series Features - Draw Gaps in Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/common-series-apis/drawing-gaps)Â article for more details.

## Add Point Markers onto a Line Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/spline-line-renderable-series/#add-point-markers-onto-a-line-series" class="hash-link" aria-label="Direct link to Add Point Markers onto a Line Series" translate="no" title="Direct link to Add Point Markers onto a Line Series">â€‹</a>

Every data point of a Spline Line Series can be marked with aÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries.html#pointmarker" rel="noopener noreferrer" target="_blank">PointMarkerðŸ“˜</a>. To add Point Markers to the Spline Line Series, see theÂ [PointMarkers API Documentation](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/common-series-apis/drawing-point-markers).

![](out_scichartv4/2d-charts/chart-types/spline-line-renderable-series/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

PointMarkers are only applied to the original data-points, not the spline interpolated points which are for display purposes only.

To learn more about the types of Point Marker in SciChart.js, see theÂ [Point Markers API documentation](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/common-series-apis/drawing-point-markers).

ThereÂ is also a dedicatedÂ [Scatter Series type](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/xy-scatter-renderable-series)Â and aÂ [Bubble Series type](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-bubble-renderable-series)Â with some more options.

## Painting Spline Line Segments with Different Colors<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/spline-line-renderable-series/#painting-spline-line-segments-with-different-colors" class="hash-link" aria-label="Direct link to Painting Spline Line Segments with Different Colors" translate="no" title="Direct link to Painting Spline Line Segments with Different Colors">â€‹</a>

![](out_scichartv4/2d-charts/chart-types/spline-line-renderable-series/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

It is possible to define the colour of line segments individually using theÂ [PaletteProvider API](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/palette-provider-api-overview).

For more info on how to do this, see theÂ [PaletteProvider - Per-point colouring of Line Charts](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/fast-line-renderable-series) documentation page. The same technique applies to spline line charts.

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/spline-line-renderable-series/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [The Line Series Type](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-line-renderable-series)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/spline-line-renderable-series/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/spline-line-renderable-series/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
