On this page

# The Column Series Type

Column Series can be created using theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastcolumnrenderableseries.html" rel="noopener noreferrer" target="_blank">FastColumnRenderableSeriesðŸ“˜</a> type.

![](out_scichartv4/2d-charts/chart-types/fast-column-renderable-series/column-series-type/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

TheÂ <a href="https://www.scichart.com/demo/javascript/column-chart" rel="noopener noreferrer" target="_blank">JavaScript Column Chart Example</a> can be found in theÂ <a href="https://github.com/ABTSoftware/SciChart.JS.Examples/tree/master/Examples/src/components/Examples/Charts2D/BasicChartTypes/ColumnChart" rel="noopener noreferrer" target="_blank">SciChart.Js Examples Suite &gt; Column Chart</a>Â on Github, or our live demo atÂ <a href="https://www.scichart.com/demo/javascript/column-chart" rel="noopener noreferrer" target="_blank">scichart.com/demo</a>.

Above: The JavaScript <a href="https://www.scichart.com/demo/iframe/column-chart" target="_blank">Column Series Chart</a> example from the <a href="https://www.scichart.com/demo/react" target="_blank">SciChart.js Demo</a>

  

## Create aÂ Column Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-column-renderable-series/column-series-type/#create-acolumn-series" class="hash-link" aria-label="Direct link to Create aÂ Column Series" translate="no" title="Direct link to Create aÂ Column Series">â€‹</a>

To create aÂ <a href="https://www.scichart.com/demo/javascript-column-chart" rel="noopener noreferrer" target="_blank">Javascript Column Chart</a> with SciChart.js, use the following code:

- TS
- Builder API (JSON Config)

``` prism-code
// Demonstrates how to create a Column chart with SciChart.js
const {
    SciChartSurface,
    NumericAxis,
    FastColumnRenderableSeries,
    XyDataSeries,
    SciChartJsNavyTheme,
    GradientParams,
    EDataPointWidthMode,
    Point,
} = SciChart;
// or, for npm, import { SciChartSurface, ... } from "scichart"

const { wasmContext, sciChartSurface } = await SciChartSurface.create(divElementId, {
    theme: new SciChartJsNavyTheme()
});
sciChartSurface.xAxes.add(new NumericAxis(wasmContext));
sciChartSurface.yAxes.add(new NumericAxis(wasmContext));

// Create and add a column series
const columnSeries = new FastColumnRenderableSeries(wasmContext, {
    dataSeries: new XyDataSeries(wasmContext, {
        xValues: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19],
        yValues: [0.1, 0.2, 0.4, 0.8, 1.1, 1.5, 2.4, 4.6, 8.1, 11.7, 14.4, 16, 13.7, 10.1, 6.4, 3.5, 2.5, 1.4, 0.4, 0.1]
    }),
    // When solid fill required, use fill
    fill: "rgba(176, 196, 222, 0.5)", 

    // When gradient fill required, use fillGradient
    fillLinearGradient: new GradientParams(new Point(0, 0), new Point(0, 1), [
        { color: "rgba(70,130,180,0.77)", offset: 0 },
        { color: "rgba(70,130,180,0.0)", offset: 1 }
    ]),
    stroke: "#FFFFFF77",
    strokeThickness: 2,
    cornerRadius: 4, // optional cornerradius

    // Defines the relative width of columns
    dataPointWidth: 0.7,
    dataPointWidthMode: EDataPointWidthMode.Relative
});

sciChartSurface.renderableSeries.add(columnSeries);
```

``` prism-code
// Demonstrates how to create a Column chart with SciChart.js using the Builder API
const { chartBuilder, ESeriesType, EThemeProviderType } = SciChart;
// or, for npm, import { chartBuilder, ... } from "scichart"

const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    surface: { theme: { type: EThemeProviderType.Dark } },
    series: [
        {
            type: ESeriesType.ColumnSeries,
            xyData: {
                xValues: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19],
                yValues: [0.1, 0.2, 0.4, 0.8, 1.1, 1.5, 2.4, 4.6, 8.1, 11.7, 14.4, 16, 13.7, 10.1, 6.4, 3.5, 2.5, 1.4, 0.4, 0.1]
            },
            options: {
                fill: "rgba(176, 196, 222, 0.5)",
                stroke: "rgba(176, 196, 222, 1)",
                strokeThickness: 2,
                dataPointWidth: 0.7
            }
        }
    ]
});
```

This results in the following output:

In the code above:

- AÂ Column Series instance is created and added to theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#renderableseries" rel="noopener noreferrer" target="_blank">SciChartSurface.renderableSeriesðŸ“˜</a> collection.
- We set the stroke, strokethickness and fillÂ properties
- We setÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastcolumnrenderableseries.html#datapointwidth" rel="noopener noreferrer" target="_blank">FastColumnRenderableSeries.dataPointWidthðŸ“˜</a> - which defines the fraction of width to occupy
- We assign aÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastcolumnrenderableseries.html#dataseries" rel="noopener noreferrer" target="_blank">FastColumnRenderableSeries.dataSeriesðŸ“˜</a> - which stores the Xy data to render.

## RenderÂ a GapÂ in a Column Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-column-renderable-series/column-series-type/#rendera-gapin-a-column-series" class="hash-link" aria-label="Direct link to RenderÂ a GapÂ in a Column Series" translate="no" title="Direct link to RenderÂ a GapÂ in a Column Series">â€‹</a>

It is possible to have null points orÂ gapsÂ in aÂ Column Series by passing aÂ data point with a **NaN** valueÂ as the **Y** value.Â Please refer to theÂ [Common Series Features - Draw Gaps in Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/common-series-apis/drawing-gaps)Â article for more details.

## Add Point Markers onto a Column Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-column-renderable-series/column-series-type/#add-point-markers-onto-a-column-series" class="hash-link" aria-label="Direct link to Add Point Markers onto a Column Series" translate="no" title="Direct link to Add Point Markers onto a Column Series">â€‹</a>

It is possible to put scatter point markers of varying type (Ellipse, Square, Triangle, Cross, Custom) onto a Column Series via the PointMarker API. To learn more, see the documentation pageÂ [Drawing PointMarkers on Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/common-series-apis/drawing-point-markers).

To learn more about the types of Point Marker in SciChart.js, see theÂ [Point Markers API documentation](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/common-series-apis/drawing-point-markers).

ThereÂ is also a dedicatedÂ [Scatter Series type](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/xy-scatter-renderable-series)Â and aÂ [Bubble Series type](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-bubble-renderable-series)Â with some more options.

## PaintingÂ Columns with Different Colors<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-column-renderable-series/column-series-type/#paintingcolumns-with-different-colors" class="hash-link" aria-label="Direct link to PaintingÂ Columns with Different Colors" translate="no" title="Direct link to PaintingÂ Columns with Different Colors">â€‹</a>

It is possible to define the colour of column stroke & fill individually using theÂ [PaletteProvider API](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/palette-provider-api-overview).

For more info on how to do this, see theÂ [PaletteProvider - Per-point colouring of Column Charts](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/fast-column-renderable-series) documentation page.

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-column-renderable-series/column-series-type/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [DataPointWidthMode](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-column-renderable-series/data-point-width-mode)
- [Start Here - RenderableSeries Overview](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/renderable-series-api-overview)
- [Common RenderableSeries Properties](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/common-series-apis/drawing-point-markers)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/fast-column-renderable-series/column-series-type/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/fast-column-renderable-series/column-series-type/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
