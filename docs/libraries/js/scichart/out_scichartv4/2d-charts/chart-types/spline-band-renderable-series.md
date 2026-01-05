On this page

# The Spline (Smoothed) Band Series Type

Spline Band or Smoothed High/Low Fill Series can be created using theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/splinebandrenderableseries.html" rel="noopener noreferrer" target="_blank">SplineBandRenderableSeriesðŸ“˜</a> type.

![](out_scichartv4/2d-charts/chart-types/spline-band-renderable-series/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

The <a href="https://www.scichart.com/demo/javascript/spline-band-chart" rel="noopener noreferrer" target="_blank">JavaScript Spline Band Chart Example</a> can be found in theÂ <a href="https://github.com/abtsoftware/scichart.js.examples" rel="noopener noreferrer" target="_blank">SciChart.Js Examples Suite &gt; Spline Band Series</a>Â on Github, or our live demo atÂ <a href="https://www.scichart.com/demo/javascript/spline-band-chart" rel="noopener noreferrer" target="_blank">scichart.com/demo</a>

Above: The JavaScript <a href="https://www.scichart.com/demo/iframe/spline-band-chart" target="_blank">Spline Band Chart</a> example from the <a href="https://www.scichart.com/demo/react" target="_blank">SciChart.js Demo</a>

  

## Create aÂ Spline Band Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/spline-band-renderable-series/#create-aspline-band-series" class="hash-link" aria-label="Direct link to Create aÂ Spline Band Series" translate="no" title="Direct link to Create aÂ Spline Band Series">â€‹</a>

To create aÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/splinebandrenderableseries.html" rel="noopener noreferrer" target="_blank">Javascript Spline Band ChartðŸ“˜</a> with SciChart.js, use the following code:

- TS
- Builder API (JSON Config)

``` prism-code
// Demonstrates how to create a Spline Band chart using SciChart.js
const {
    SciChartSurface,
    NumericAxis,
    SplineBandRenderableSeries,
    XyyDataSeries,
    SciChartJsNavyTheme,
    EllipsePointMarker
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
for (let i = 0; i < 10; i++) {
    xValues.push(i);
    yValues.push(2 + 0.2 * Math.sin(i) - Math.cos(i * 0.12));
    y1Values.push(1.8 + 0.19 * Math.sin(i * 2) - Math.cos(i * 0.24));
}
const dataSeries = new XyyDataSeries(wasmContext, { xValues, yValues, y1Values });

const bandSeries = new SplineBandRenderableSeries(wasmContext, {
    dataSeries,
    stroke: "#F48420",
    strokeY1: "#50C7E0",
    fill: "#F4842033",
    fillY1: "#50C7E033",
    strokeThickness: 2,
    interpolationPoints: 20, // the larger the number, the smoother the band
    // Add a pointmarker to show where the datapoints are
    pointMarker: new EllipsePointMarker(wasmContext, {
        width: 7,
        height: 7,
        fill: "white",
        strokeThickness: 0
    })
});

sciChartSurface.renderableSeries.add(bandSeries);
```

``` prism-code
// Demonstrates how to create a band chart with SciChart.js using the Builder API
const { chartBuilder, ESeriesType, EThemeProviderType, EPointMarkerType } = SciChart;
// or, for npm, import { chartBuilder, ... } from "scichart"

const xValues = [];
const yValues = [];
const y1Values = [];
for (let i = 0; i < 10; i++) {
    xValues.push(i);
    yValues.push(2 + 0.2 * Math.sin(i) - Math.cos(i * 0.12));
    y1Values.push(1.8 + 0.19 * Math.sin(i * 2) - Math.cos(i * 0.24));
}

const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    surface: { theme: { type: EThemeProviderType.Dark } },
    series: [
        {
            type: ESeriesType.SplineBandSeries,
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
                interpolationPoints: 20, // the larger the number, the smoother the band
                pointMarker: {
                    type: EPointMarkerType.Ellipse,
                    options: {
                        width: 7,
                        height: 7,
                        strokeThickness: 1,
                        fill: "steelblue",
                        stroke: "LightSteelBlue"
                    }
                }
            }
        }
    ]
});
```

In the code above:

- AÂ Spline Band Series instance is created and added to theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#renderableseries" rel="noopener noreferrer" target="_blank">SciChartSurface.renderableSeriesðŸ“˜</a> collection.
- We set the stroke, strokeY1,Â strokethickness properties
- We assign anÂ XyyDataSeries - which stores the Xyy data to render.
- We set the number ofÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/splinebandrenderableseries.html#interpolationpoints" rel="noopener noreferrer" target="_blank">interpolationPointsðŸ“˜</a> - how many points between real Xy data points will be interpolated using a Spline interpolation algorithm.

This results in the following output:

## Performance Tips in Spline Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/spline-band-renderable-series/#performance-tips-in-spline-series" class="hash-link" aria-label="Direct link to Performance Tips in Spline Series" translate="no" title="Direct link to Performance Tips in Spline Series">â€‹</a>

![](out_scichartv4/2d-charts/chart-types/spline-band-renderable-series/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

When theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/splinemountainrenderableseries.html#interpolationpoints" rel="noopener noreferrer" target="_blank">SplineBandRenderableSeries.interpolationPointsðŸ“˜</a> property is set to zero, then this series renders and displays exactly like a FastLineRenderableSeries.

![](out_scichartv4/2d-charts/chart-types/spline-band-renderable-series/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

When the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/splinemountainrenderableseries.html#interpolationpoints" rel="noopener noreferrer" target="_blank">interpolationPointsðŸ“˜</a> property is set to another number, e.g. 10, then SciChart.js will calculate 10 points for each Xy datapoint you add to the XyDataSeries. This means you will be displaying 10x the number of datapoints.

*SciChart.js can handle millions of datapoints, but this is something to be aware of. You might want to adjust down the interpolationPoints depending on amount of data on the chart, or zoom level.*

## RenderÂ a GapÂ in a Spline Band Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/spline-band-renderable-series/#rendera-gapin-a-spline-band-series" class="hash-link" aria-label="Direct link to RenderÂ a GapÂ in a Spline Band Series" translate="no" title="Direct link to RenderÂ a GapÂ in a Spline Band Series">â€‹</a>

![](out_scichartv4/2d-charts/chart-types/spline-band-renderable-series/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

It is possible to have null points orÂ gapsÂ in a Spline Band Series by passing aÂ data point with a **NaN** valueÂ as the **Y** value.Â Please refer to theÂ [Common Series Features - Draw Gaps in Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/common-series-apis/drawing-gaps)Â article for more details.

## Add Point Markers onto a Spline Band Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/spline-band-renderable-series/#add-point-markers-onto-a-spline-band-series" class="hash-link" aria-label="Direct link to Add Point Markers onto a Spline Band Series" translate="no" title="Direct link to Add Point Markers onto a Spline Band Series">â€‹</a>

Every data point of a Spline Band Series can be marked with aÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries.html#pointmarker" rel="noopener noreferrer" target="_blank">PointMarkerðŸ“˜</a>.Â To add Point Markers to the Spline Mountain Series, see theÂ [PointMarkers API Documentation](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/common-series-apis/drawing-point-markers).

![](out_scichartv4/2d-charts/chart-types/spline-band-renderable-series/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

***Note:** PointMarkers are only applied to the original data-points, not the spline interpolated points which are for display purposes only.*

To learn more about the types of Point Marker in SciChart.js, see theÂ [Point Markers API documentation](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/common-series-apis/drawing-point-markers).

ThereÂ is also a dedicatedÂ [Scatter Series type](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/xy-scatter-renderable-series)Â and aÂ [Bubble Series type](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-bubble-renderable-series)Â with some more options.

## Painting Spline Band Segments with Different Colors<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/spline-band-renderable-series/#painting-spline-band-segments-with-different-colors" class="hash-link" aria-label="Direct link to Painting Spline Band Segments with Different Colors" translate="no" title="Direct link to Painting Spline Band Segments with Different Colors">â€‹</a>

It is possible to define the colour of line and band segments individually using theÂ [PaletteProvider API](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/palette-provider-api-overview).

For more info on how to do this, see theÂ [PaletteProvider - Per-point colouring of Band Charts](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/fast-band-renderable-series) documentation page. The same technique applies to spline line charts.

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/spline-band-renderable-series/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [Fast Band Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-band-renderable-series)
- [Spline Line Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/spline-line-renderable-series)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/spline-band-renderable-series/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/spline-band-renderable-series/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
