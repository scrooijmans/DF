On this page

# The Band Series type

A Band Series, or High-Low Fill between two lines can be created using theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastbandrenderableseries.html" rel="noopener noreferrer" target="_blank">FastBandRenderableSeriesðŸ“˜</a> type.

![](out_scichartv4/2d-charts/chart-types/fast-band-renderable-series/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

The <a href="https://www.scichart.com/demo/javascript-band-chart" rel="noopener noreferrer" target="_blank">JavaScript Band Chart Example</a>Â can be found in theÂ <a href="https://github.com/ABTSoftware/SciChart.JS.Examples/tree/master/Examples/src/components/Examples/Charts2D/BasicChartTypes/BandSeriesChart" rel="noopener noreferrer" target="_blank">SciChart.Js Examples Suite &gt; Band Series</a>Â on Github, or our live demo atÂ <a href="https://www.scichart.com/demo/javascript/band-chart" rel="noopener noreferrer" target="_blank">scichart.com/demo</a>.

Above: The JavaScript <a href="https://www.scichart.com/demo/iframe/javascript-band-chart" target="_blank">Band Series Chart</a> example from the <a href="https://www.scichart.com/demo/react" target="_blank">SciChart.js Demo</a>

  

## Create aÂ Band Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-band-renderable-series/#create-aband-series" class="hash-link" aria-label="Direct link to Create aÂ Band Series" translate="no" title="Direct link to Create aÂ Band Series">â€‹</a>

To create aÂ <a href="https://www.scichart.com/demo/javascript-band-chart" rel="noopener noreferrer" target="_blank">Javascript Band Chart</a> with SciChart.js, use the following code:

- TS
- Builder API (JSON Config)

``` prism-code
// Demonstrates how to create a band chart using SciChart.js
const { SciChartSurface, NumericAxis, FastBandRenderableSeries, XyyDataSeries, SciChartJsNavyTheme } = SciChart;
// or, for npm, import { SciChartSurface, ... } from "scichart"

const { wasmContext, sciChartSurface } = await SciChartSurface.create(divElementId, {
    theme: new SciChartJsNavyTheme()
});

sciChartSurface.xAxes.add(new NumericAxis(wasmContext));
sciChartSurface.yAxes.add(new NumericAxis(wasmContext));

// Generate some Xyy data for the band series
const xValues = [];
const yValues = [];
const y1Values = [];
const POINTS = 1000;
const STEP = (3 * Math.PI) / POINTS;
for (let i = 0; i <= POINTS; i++) {
    const k = 1 - i / 2000;
    xValues.push(i);
    yValues.push(Math.sin(i * STEP) * k * 0.7);
    y1Values.push(Math.cos(i * STEP) * k);
}

// Create & configure the band series
const bandSeries1 = new FastBandRenderableSeries(wasmContext, {
    dataSeries: new XyyDataSeries(wasmContext, { 
        xValues, 
        yValues, 
        y1Values 
    }),
    stroke: "#F48420",
    strokeY1: "#50C7E0",
    fill: "#F4842033",   // the Fill for a data slice where `yVal < y1Val`
    fillY1: "#50C7E033", // the Fill for a data slice where `yVal > y1Val`
    strokeThickness: 2
});
sciChartSurface.renderableSeries.add(bandSeries1);
```

``` prism-code
// Demonstrates how to create a band chart with SciChart.js using the Builder API
const { chartBuilder, ESeriesType, EThemeProviderType } = SciChart;
// or, for npm, import { chartBuilder, ... } from "scichart"

const xValues = [];
const yValues = [];
const y1Values = [];
const POINTS = 1000;
const STEP = (3 * Math.PI) / POINTS;
for (let i = 0; i <= 1000; i++) {
    const k = 1 - i / 2000;
    xValues.push(i);
    yValues.push(Math.sin(i * STEP) * k * 0.7);
    y1Values.push(Math.cos(i * STEP) * k);
}

const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    surface: { theme: { type: EThemeProviderType.Dark } },
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
                fill: "#279B2733",   // the Fill for a data slice where `yVal < y1Val`
                fillY1: "#FF191933", // the Fill for a data slice where `yVal > y1Val`
                strokeThickness: 2
            }
        }
    ]
});
```

In the code above:

- AÂ Band Series instance is created and added to theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#renderableseries" rel="noopener noreferrer" target="_blank">SciChartSurface.renderableSeriesðŸ“˜</a> collection.
- We set the stroke, fill properties for when Y1 \> Y2 andÂ vice versaÂ (more info over atÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastbandrenderableseries.html" rel="noopener noreferrer" target="_blank">FastBandRenderableSeriesðŸ“˜</a> in TypeDoc).
- We assign aÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastbandrenderableseries.html#dataseries" rel="noopener noreferrer" target="_blank">DataSeriesðŸ“˜</a> - in this case anÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyydataseries.html" rel="noopener noreferrer" target="_blank">XyyDataSeriesðŸ“˜</a> which stores X, Y1, Y2 data.

## Gradient Fills in Band Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-band-renderable-series/#gradient-fills-in-band-series" class="hash-link" aria-label="Direct link to Gradient Fills in Band Series" translate="no" title="Direct link to Gradient Fills in Band Series">â€‹</a>

New to SciChart.js v3.4, the Band Series chart type now supports gradient fills as well as solid color fills.

To use this, you need to set theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastbandrenderableseries.html#filllineargradient" rel="noopener noreferrer" target="_blank">FastBandRenderableSeries.fillLinearGradientðŸ“˜</a> &Â <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastbandrenderableseries.html#filllineargradienty1" rel="noopener noreferrer" target="_blank">fillLinearGradientY1ðŸ“˜</a> properties with aÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/gradientparams.html" rel="noopener noreferrer" target="_blank">GradientParamsðŸ“˜</a> object: a type which defines gradients by a number of gradient stops, with a start and end point.

Here's a code example:

- TS
- Builder API (JSON Config)

``` prism-code
const bandSeries = new FastBandRenderableSeries(wasmContext, {
    dataSeries: new XyyDataSeries(wasmContext, { xValues, yValues, y1Values }),
    stroke: "#F48420",
    strokeY1: "#50C7E0",

    // use fillLinearGradient and fillLinearGradientY1 to set a gradient fill
    fillLinearGradient: new GradientParams(new Point(0, 0.6), new Point(0, 0.9), [
        { color: "#F48420ff", offset: 0 },
        { color: "#F4842033", offset: 1 }
    ]),
    // instead of fill and fillY1
    fillLinearGradientY1: new GradientParams(new Point(0, 0.6), new Point(0, 0.9), [
        { color: "#50C7E033", offset: 0 },
        { color: "#50C7E0ff", offset: 1 }
    ]),
    strokeThickness: 4
});

sciChartSurface.renderableSeries.add(bandSeries);
```

``` prism-code
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
                // use fillLinearGradient and fillLinearGradientY1 to set a gradient fill
                fillLinearGradient: new GradientParams(new Point(0, 0.6), new Point(0, 0.9), [
                    { color: "#F48420ff", offset: 0 },
                    { color: "#F4842033", offset: 1 }
                ]),
                // instead of fill and fillY1
                fillLinearGradientY1: new GradientParams(new Point(0, 0.6), new Point(0, 0.9), [
                    { color: "#50C7E033", offset: 0 },
                    { color: "#50C7E0ff", offset: 1 }
                ]),
                strokeThickness: 4
            }
        }
    ]
});
```

Try adjusting the CodePen above with the code to set a fillLinearGradient and see the result!

## RenderÂ a GapÂ in a Band Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-band-renderable-series/#rendera-gapin-a-band-series" class="hash-link" aria-label="Direct link to RenderÂ a GapÂ in a Band Series" translate="no" title="Direct link to RenderÂ a GapÂ in a Band Series">â€‹</a>

![](out_scichartv4/2d-charts/chart-types/fast-band-renderable-series/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

It is possible to have null points orÂ gapsÂ in a Band Series by passing aÂ data point with a **NaN** valueÂ as the **Y** value.Â Please refer to theÂ [Common Series Features - Draw Gaps in Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/common-series-apis/drawing-gaps)Â article for more details.

## Add Point Markers onto a Band Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-band-renderable-series/#add-point-markers-onto-a-band-series" class="hash-link" aria-label="Direct link to Add Point Markers onto a Band Series" translate="no" title="Direct link to Add Point Markers onto a Band Series">â€‹</a>

It is possible to put scatter point markers of varying type (Ellipse, Square, Triangle, Cross, Custom) onto a Band Series via the PointMarker API. To learn more, see the documentation pageÂ [Drawing PointMarkers on Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/common-series-apis/drawing-point-markers).

![](out_scichartv4/2d-charts/chart-types/fast-band-renderable-series/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

To learn more about the types of Point Marker in SciChart.js, see theÂ [Point Markers API documentation](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/common-series-apis/drawing-point-markers).

ThereÂ is also a dedicatedÂ [Scatter Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/xy-scatter-renderable-series) typeÂ and aÂ [Bubble Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-bubble-renderable-series)Â type with some more options.

## Painting Band Segments with Different Colors<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-band-renderable-series/#painting-band-segments-with-different-colors" class="hash-link" aria-label="Direct link to Painting Band Segments with Different Colors" translate="no" title="Direct link to Painting Band Segments with Different Colors">â€‹</a>

![](out_scichartv4/2d-charts/chart-types/fast-band-renderable-series/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

It is possible to define the colour of line segments individually using theÂ [PaletteProvider API](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/fast-line-segment-renderable-series/)

For more info on how to do this, see theÂ [PaletteProvider - Per-point colouring of Band Charts](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/fast-band-renderable-series) documentation page.

## Use Cases<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-band-renderable-series/#use-cases" class="hash-link" aria-label="Direct link to Use Cases" translate="no" title="Direct link to Use Cases">â€‹</a>

### Displaying horizontal thresholds<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-band-renderable-series/#displaying-horizontal-thresholds" class="hash-link" aria-label="Direct link to Displaying horizontal thresholds" translate="no" title="Direct link to Displaying horizontal thresholds">â€‹</a>

The Band Series Chart can be used to display a mountain or area with a horizontal threshold. For example, if you want to see a series which has a green mountain above zero and red below, to track PnL or profit and loss, you can also use a band series.

For instance, the following code:

- TS
- Builder API (JSON Config)

``` prism-code
// Use a Band series to render a mountain chart with horizontal threshold.
const xValues = [];
const yValues = [];
const y1Values = [];
const POINTS = 1000;
const threshold = 0.4;
const STEP = (3 * Math.PI) / POINTS;
for (let i = 0; i <= 1000; i++) {
    const k = 1 - i / 2000;
    xValues.push(i);
    yValues.push(threshold); // constant value for Y
    y1Values.push(Math.cos(i * STEP) * k);
}

const bandSeries = new FastBandRenderableSeries(wasmContext, {
    dataSeries: new XyyDataSeries(wasmContext, { xValues, yValues, y1Values }),
    stroke: "Transparent", // render Y stroke as transparent
    strokeY1: "#50C7E0", // render Y1 transparent as blue
    fill: "#FF191933", // have different fills above/below the threshold. This is fill above
    fillY1: "#50C7E033", // fill below
    strokeThickness: 2
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
const POINTS = 1000;
const threshold = 0.4;
const STEP = (3 * Math.PI) / POINTS;
for (let i = 0; i <= 1000; i++) {
    const k = 1 - i / 2000;
    xValues.push(i);
    yValues.push(threshold); // constant value for Y
    y1Values.push(Math.cos(i * STEP) * k);
}

const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    surface: { theme: { type: EThemeProviderType.Dark } },
    series: [
        {
            type: ESeriesType.BandSeries,
            xyyData: {
                xValues,
                yValues,
                y1Values
            },
            options: {
                stroke: "Transparent", // render Y stroke as transparent
                strokeY1: "#50C7E0", // render Y1 transparent as blue
                fill: "#FF191933", // have different fills above/below the threshold. This is fill above
                fillY1: "#50C7E033", // fill below
                strokeThickness: 2
            }
        }
    ]
});
```

Resulting in this:

### Bollinger Bands & MACD Indicators<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-band-renderable-series/#bollinger-bands--macd-indicators" class="hash-link" aria-label="Direct link to Bollinger Bands &amp; MACD Indicators" translate="no" title="Direct link to Bollinger Bands &amp; MACD Indicators">â€‹</a>

The Band Series chart primary use is in financial markets, when you want to display things like Bollinger Bands, MACD. For example, our Create Multi Pane Stock Charts demo has a band series for the MACD indicator, which you can see below.

<img src="out_scichartv4/2d-charts/chart-types/fast-band-renderable-series/index_media/1841b941ef8a39ac9b663d026e743e44de811509.png" title="Bollinger Bands &amp; MACD Indicators" style="display:block;margin-left:auto;margin-right:auto;object-fit:contain;height:auto;width:85%;margin:0 auto" alt="Bollinger Bands &amp; MACD Indicators" />

Bollinger Bands & MACD Indicators

<img src="out_scichartv4/2d-charts/chart-types/fast-band-renderable-series/index_media/1bae938ea8b6d32df3b3e24f7cf497a114aed07e.png" title="A complex visualisation using Band Series" style="display:block;margin-left:auto;margin-right:auto;object-fit:contain;height:auto;width:85%;margin:0 auto" alt="A complex visualisation using Band Series" />

A complex visualisation using Band Series

***Above**: a complex visualisation using Band Series. This image is taken from a SciChart WPF sample but all the features are available in SciChart.js as well.*

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-band-renderable-series/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [Start Here - RenderableSeries Overview](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/renderable-series-api-overview)

- [Common RenderableSeries Properties](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/common-series-apis/is-visible)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/fast-band-renderable-series/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/fast-band-renderable-series/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
