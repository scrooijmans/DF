On this page

# The Scatter Series Type

Scatter Series can be created using theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyscatterrenderableseries.html" rel="noopener noreferrer" target="_blank">XyScatterRenderableSeriesðŸ“˜</a> type.

![](out_scichartv4/2d-charts/chart-types/xy-scatter-renderable-series/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

The <a href="https://www.scichart.com/demo/javascript/scatter-chart" rel="noopener noreferrer" target="_blank">JavaScript Scatter Chart Example</a> can be found in theÂ <a href="https://github.com/ABTSoftware/SciChart.JS.Examples/tree/master/Examples/src/components/Examples/Charts2D/BasicChartTypes/ScatterChart" rel="noopener noreferrer" target="_blank">SciChart.Js Examples Suite &gt; Scatter Chart</a>Â on Github, or our live demo atÂ <a href="https://www.scichart.com/demo/javascript/scatter-chart" rel="noopener noreferrer" target="_blank">scichart.com/demo</a>.

Above: The JavaScript <a href="https://www.scichart.com/demo/iframe/scatter-chart" target="_blank">Scatter Series Chart</a> example from the <a href="https://www.scichart.com/demo/react" target="_blank">SciChart.js Demo</a>

  

## Create aÂ Scatter Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/xy-scatter-renderable-series/#create-ascatter-series" class="hash-link" aria-label="Direct link to Create aÂ Scatter Series" translate="no" title="Direct link to Create aÂ Scatter Series">â€‹</a>

To create aÂ <a href="https://www.scichart.com/demo/javascript-scatter-chart" rel="noopener noreferrer" target="_blank">Javascript Scatter Chart</a> with SciChart.js, use the following code:

- TS
- Builder API (JSON Config)

``` prism-code
// Demonstrates how to create a scatter chart with SciChart.js
const {
    SciChartSurface,
    NumericAxis,
    XyDataSeries,
    XyScatterRenderableSeries,
    EllipsePointMarker,
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
    yValues.push(0.2 * Math.sin(i * 0.1) - Math.cos(i * 0.01));
}

const scatterSeries = new XyScatterRenderableSeries(wasmContext, {
    dataSeries: new XyDataSeries(wasmContext, {
        xValues, 
        yValues
    }),
    pointMarker: new EllipsePointMarker(wasmContext, {
        width: 7,
        height: 7,
        strokeThickness: 2,
        fill: "steelblue",
        stroke: "LightSteelBlue"
    })
});
sciChartSurface.renderableSeries.add(scatterSeries);
```

``` prism-code
// Demonstrates how to create a scatter with SciChart.js using the Builder API
const { chartBuilder, ESeriesType, EPointMarkerType, EThemeProviderType } = SciChart;
// or, for npm, import { chartBuilder, ... } from "scichart"

const xValues = [];
const yValues = [];
for (let i = 0; i < 100; i++) {
    xValues.push(i);
    yValues.push(0.2 * Math.sin(i * 0.1) - Math.cos(i * 0.01));
}

const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    surface: { theme: { type: EThemeProviderType.Dark } },
    series: [
        {
            type: ESeriesType.ScatterSeries,
            xyData: {
                xValues,
                yValues
            },
            options: {
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

This results in the following:

In the code above:

- AÂ Scatter Series instance is created and added to theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#renderableseries" rel="noopener noreferrer" target="_blank">SciChartSurface.renderableSeriesðŸ“˜</a> collection.
- We set aÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries.html#pointmarker" rel="noopener noreferrer" target="_blank">PointMarkerðŸ“˜</a>. Several types such as Ellipse, Triangle, Cross and Custom are available ([see here for more info](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/common-series-apis/drawing-point-markers))
- We assign aÂ [DataSeries](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-series-api/data-series-api-overview) - which stores the Xy data to render.

## RenderÂ a GapÂ in a Scatter Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/xy-scatter-renderable-series/#rendera-gapin-a-scatter-series" class="hash-link" aria-label="Direct link to RenderÂ a GapÂ in a Scatter Series" translate="no" title="Direct link to RenderÂ a GapÂ in a Scatter Series">â€‹</a>

![](out_scichartv4/2d-charts/chart-types/xy-scatter-renderable-series/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

It is possible to have null points orÂ gapsÂ in aÂ Scatter Series by passing aÂ data point with a **NaN** valueÂ as the **Y** value.Â Or, by simply skipping a point if using a value-axis. Please refer to theÂ [Common Series Features - Draw Gaps in Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/common-series-apis/drawing-gaps)Â article for more details.

## Drawing Last Point only in a Scatter Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/xy-scatter-renderable-series/#drawing-last-point-only-in-a-scatter-series" class="hash-link" aria-label="Direct link to Drawing Last Point only in a Scatter Series" translate="no" title="Direct link to Drawing Last Point only in a Scatter Series">â€‹</a>

![](out_scichartv4/2d-charts/chart-types/xy-scatter-renderable-series/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

New to SciChart.js v3.2! The PointMarker type has a propertyÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basepointmarker.html#lastpointonly" rel="noopener noreferrer" target="_blank">isLastPointOnlyðŸ“˜</a>. When true, only the last point of a scatter series is drawn. This can be useful to highlight a point in say a sweeping ECG chart.

## Different Point-Markers on a Scatter Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/xy-scatter-renderable-series/#different-point-markers-on-a-scatter-series" class="hash-link" aria-label="Direct link to Different Point-Markers on a Scatter Series" translate="no" title="Direct link to Different Point-Markers on a Scatter Series">â€‹</a>

Every data point of a Scatter SeriesÂ is marked with aÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries.html#pointmarker" rel="noopener noreferrer" target="_blank">PointMarkerðŸ“˜</a>. Several different types of PointMarker are available in SciChart.js.

![](out_scichartv4/2d-charts/chart-types/xy-scatter-renderable-series/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

To learn more about the types of Point Marker in SciChart.js, see theÂ [Point Markers API documentation](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/common-series-apis/drawing-point-markers).

There is also a TypeScript example of custom pointmarkers in theÂ <a href="https://www.scichart.com/demo/javascript-chart-custom-pointmarkers" rel="noopener noreferrer" target="_blank">SciChart.js Demo.</a>

Finally, there isÂ a dedicatedÂ [Bubble Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-bubble-renderable-series) typeÂ with some more options such as per-point sizing.

## Painting Scatter Points with Different Colors<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/xy-scatter-renderable-series/#painting-scatter-points-with-different-colors" class="hash-link" aria-label="Direct link to Painting Scatter Points with Different Colors" translate="no" title="Direct link to Painting Scatter Points with Different Colors">â€‹</a>

![](out_scichartv4/2d-charts/chart-types/xy-scatter-renderable-series/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

It is possible to define the colour of PointMarkers individually using theÂ [PaletteProvider API](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/palette-provider-api-overview).

For more info on how to do this, see theÂ [PaletteProvider - Per-point colouring of Scatter Charts](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/xy-scatter-renderable-series) documentation page.

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/xy-scatter-renderable-series/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [Start Here - RenderableSeries Overview](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/renderable-series-api-overview)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/xy-scatter-renderable-series/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/xy-scatter-renderable-series/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
