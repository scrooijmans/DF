On this page

# The Lollipop (Impulse or Stem) Chart Type

Lollipop Charts, otherwise known as Impulse or Stem charts,Â can be created using theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastimpulserenderableseries.html" rel="noopener noreferrer" target="_blank">FastImpulseRenderableSeriesðŸ“˜</a> type.

![](out_scichartv4/2d-charts/chart-types/fast-impulse-renderable-series/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

The <a href="https://www.scichart.com/demo/javascript/impulse-chart" rel="noopener noreferrer" target="_blank">JavaScript Impulse Series Example</a> can be found in theÂ <a href="https://github.com/abtsoftware/scichart.js.examples" rel="noopener noreferrer" target="_blank">SciChart.Js Examples Suite</a>Â on Github, or our live demo atÂ <a href="https://www.scichart.com/demo/javascript/impulse-chart" rel="noopener noreferrer" target="_blank">scichart.com/demo</a>.

Above: The JavaScript <a href="https://www.scichart.com/demo/iframe/impulse-chart" target="_blank">JavaScript Impulse Chart example</a> example from the <a href="https://www.scichart.com/demo/react" target="_blank">SciChart.js Demo</a>

  

## Create an Impulse Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-impulse-renderable-series/#create-an-impulse-series" class="hash-link" aria-label="Direct link to Create an Impulse Series" translate="no" title="Direct link to Create an Impulse Series">â€‹</a>

To create aÂ <a href="https://www.scichart.com/demo/javascript/impulse-chart" rel="noopener noreferrer" target="_blank">Javascript Impulse Chart</a> with SciChart.js, use the following code:

- TS
- Builder API (JSON Config)

``` prism-code
// Demonstrates how to create an Impulse (or Stem, Lollipop) chart with SciChart.js
const {
    SciChartSurface,
    NumericAxis,
    FastImpulseRenderableSeries,
    XyDataSeries,
    EllipsePointMarker,
    SciChartJsNavyTheme,
    NumberRange
} = SciChart;
// or, for npm, import { SciChartSurface, ... } from "scichart"

const { wasmContext, sciChartSurface } = await SciChartSurface.create(divElementId, {
    theme: new SciChartJsNavyTheme()
});
sciChartSurface.xAxes.add(new NumericAxis(wasmContext));
sciChartSurface.yAxes.add(new NumericAxis(wasmContext, { growBy: new NumberRange(0, 0.1) }));

// Create some data
const xValues = [];
const yValues = [];
for (let i = 0; i < 100; i++) {
    xValues.push(i);
    yValues.push(Math.sin(i * 0.2) * Math.log(i / 100));
}

// Create and add a column series
const impulseSeries = new FastImpulseRenderableSeries(wasmContext, {
    fill: "rgba(176, 196, 222, 0.5)",
    stroke: "rgba(176, 196, 222, 1)",
    strokeThickness: 2,
    size: 10,
    dataSeries: new XyDataSeries(wasmContext, { xValues, yValues }),
    // Optional: define the pointmarker type. Note: size, stroke, fill properties are on the parent series
    pointMarker: new EllipsePointMarker(wasmContext)
});

sciChartSurface.renderableSeries.add(impulseSeries);
```

``` prism-code
// Demonstrates how to create a line chart with SciChart.js using the Builder API
const { chartBuilder, ESeriesType, EThemeProviderType, EPointMarkerType } = SciChart;
// or, for npm, import { chartBuilder, ... } from "scichart"

// Create some data
const xValues = [];
const yValues = [];
for (let i = 0; i < 100; i++) {
    xValues.push(i);
    yValues.push(Math.sin(i * 0.2) * Math.log(i / 100));
}

const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    surface: { theme: { type: EThemeProviderType.Dark } },
    series: [
        {
            type: ESeriesType.ImpulseSeries,
            xyData: {
                xValues,
                yValues
            },
            options: {
                fill: "rgba(176, 196, 222, 0.5)",
                stroke: "rgba(176, 196, 222, 1)",
                strokeThickness: 2,
                size: 10,
                pointMarker: { type: EPointMarkerType.Ellipse }
            }
        }
    ]
});
```

This results in the following output:

In the code above:

- A Impulse Series instance is created and added to the `SciChartSurface.renderableSeries` collection.
- We set the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastimpulserenderableseries.html#fill" rel="noopener noreferrer" target="_blank">fillðŸ“˜</a> property that controls the color of connector and point of each dataset
- We can update the size of each point by updating <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastimpulserenderableseries.html#size" rel="noopener noreferrer" target="_blank">sizeðŸ“˜</a> property (default value is `10.0`)
- We assign aÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastimpulserenderableseries.html#dataseries" rel="noopener noreferrer" target="_blank">FastImpulseRenderableSeries.dataSeriesðŸ“˜</a> - which stores the Xy data to render.

## Setting the PointMarker on an Impulse Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-impulse-renderable-series/#setting-the-pointmarker-on-an-impulse-series" class="hash-link" aria-label="Direct link to Setting the PointMarker on an Impulse Series" translate="no" title="Direct link to Setting the PointMarker on an Impulse Series">â€‹</a>

Every data point of an Impulse Series is marked with aÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries.html#pointmarker" rel="noopener noreferrer" target="_blank">PointMarkerðŸ“˜</a>. To change the pointmarker type, or size, use the following code. Note that the fill, size property on `FastImpulseRenderableSeries` overrides the width, height, fill, stroke on the `TrianglePointMarker`.

``` prism-code
const impulseSeries = new FastImpulseRenderableSeries(wasmContext, {
    fill: "#26c6da",
    strokeThickness: 1,
    dataSeries,
    pointMarker: new TrianglePointMarker(wasmContext),
    size: 10,
});
```

![](out_scichartv4/2d-charts/chart-types/fast-impulse-renderable-series/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

Different pointmarkers are supported including Ellipse, Box, Triangle, Cross or custom markers. See theÂ [Scatter Chart documentation](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/xy-scatter-renderable-series) for more information on supported pointmarkers.

## RenderÂ a GapÂ in an Impulse Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-impulse-renderable-series/#rendera-gapin-an-impulse-series" class="hash-link" aria-label="Direct link to RenderÂ a GapÂ in an Impulse Series" translate="no" title="Direct link to RenderÂ a GapÂ in an Impulse Series">â€‹</a>

![](out_scichartv4/2d-charts/chart-types/fast-impulse-renderable-series/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

It is possible to have null points orÂ gapsÂ in aÂ Impulse Series by passing aÂ data point with a **NaN** valueÂ as the **Y** value.Â Please refer to theÂ [Common Series Features - Draw Gaps in Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/common-series-apis/drawing-gaps)Â article for more details.

## Painting Impulse Series Points with Different Colors<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-impulse-renderable-series/#painting-impulse-series-points-with-different-colors" class="hash-link" aria-label="Direct link to Painting Impulse Series Points with Different Colors" translate="no" title="Direct link to Painting Impulse Series Points with Different Colors">â€‹</a>

![](out_scichartv4/2d-charts/chart-types/fast-impulse-renderable-series/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

It is possible to define the colourÂ individual datapoints differently using theÂ [PaletteProvider API](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/palette-provider-api-overview).

For more info on how to do this, see theÂ [PaletteProvider - Per-point colouring of Impulse Charts](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/fast-impulse-renderable-series) documentation page.

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/fast-impulse-renderable-series/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/fast-impulse-renderable-series/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
