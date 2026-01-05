On this page

# The Bubble Series Type

Bubble Series can be created using theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastbubblerenderableseries.html" rel="noopener noreferrer" target="_blank">FastBubbleRenderableSeriesðŸ“˜</a> type.

![](out_scichartv4/2d-charts/chart-types/fast-bubble-renderable-series/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

TheÂ <a href="https://www.scichart.com/demo/javascript-bubble-chart" rel="noopener noreferrer" target="_blank">JavaScript Bubble Chart Example</a>Â can be found in theÂ <a href="https://github.com/ABTSoftware/SciChart.JS.Examples/tree/master/Examples/src/components/Examples/Charts2D/BasicChartTypes/BubbleChart" rel="noopener noreferrer" target="_blank">SciChart.Js Examples Suite &gt; Bubble Series</a>Â on Github, or our live demo atÂ <a href="https://www.scichart.com/demo/javascript/bubble-chart" rel="noopener noreferrer" target="_blank">scichart.com/demo</a>.

Above: The JavaScript <a href="https://www.scichart.com/demo/iframe/bubble-chart" target="_blank">Bubble Series Chart</a> example from the <a href="https://www.scichart.com/demo/react" target="_blank">SciChart.js Demo</a>

  

## Create aÂ Bubble Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-bubble-renderable-series/#create-abubble-series" class="hash-link" aria-label="Direct link to Create aÂ Bubble Series" translate="no" title="Direct link to Create aÂ Bubble Series">â€‹</a>

To create aÂ <a href="https://www.scichart.com/demo/javascript-bubble-chart" rel="noopener noreferrer" target="_blank">Javascript Bubble Chart</a> with SciChart.js, use the following code:

- TS
- Builder API (JSON Config)

``` prism-code
// Demonstrates how to create a bubble chart with SciChart.js
const {
    SciChartSurface,
    NumericAxis,
    XyzDataSeries,
    FastBubbleRenderableSeries,
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
const sizes = [];
for (let i = 0; i < 30; i++) {
    xValues.push(i);
    yValues.push(0.2 * Math.sin(i * 0.2) - Math.cos(i * 0.04));
    sizes.push(Math.sin(i) * 60 + 3);
}

const xyzDataSeries = new XyzDataSeries(wasmContext, {
    xValues,
    yValues,
    zValues: sizes
});

const bubbleSeries = new FastBubbleRenderableSeries(wasmContext, {
    dataSeries: xyzDataSeries,
    pointMarker: new EllipsePointMarker(wasmContext, {
        // choose a suitably large size for pointmarker. This will  be scaled per-point
        width: 64,
        height: 64,
        strokeThickness: 0,
        fill: "#4682b477"
    })
});

sciChartSurface.renderableSeries.add(bubbleSeries);
```

``` prism-code
// Demonstrates how to create a scatter with SciChart.js using the Builder API
const { chartBuilder, ESeriesType, EPointMarkerType, EThemeProviderType } = SciChart;
// or, for npm, import { chartBuilder, ... } from "scichart"

const xValues = [];
const yValues = [];
const sizes = [];
for (let i = 0; i < 30; i++) {
    xValues.push(i);
    yValues.push(0.2 * Math.sin(i * 0.2) - Math.cos(i * 0.04));
    sizes.push(Math.sin(i) * 60 + 3);
}

const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    surface: { theme: { type: EThemeProviderType.Dark } },
    series: [
        {
            type: ESeriesType.BubbleSeries,
            xyzData: {
                xValues,
                yValues,
                zValues: sizes
            },
            options: {
                pointMarker: {
                    type: EPointMarkerType.Ellipse,
                    options: {
                        // choose a suitably large size for pointmarker. This will  be scaled per-point
                        width: 64,
                        height: 64,
                        strokeThickness: 0,
                        fill: "#4682b477"
                    }
                }
            }
        }
    ]
});
```

In the code above:

- AÂ Bubble Series instance is created and added to theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#renderableseries" rel="noopener noreferrer" target="_blank">SciChartSurface.renderableSeriesðŸ“˜</a> collection.
- We set aÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basepointmarker.html" rel="noopener noreferrer" target="_blank">PointMarkerðŸ“˜</a> with a width, height = 64. Note that this pointmarker will be scaled up or down relative to bubble size. HavingÂ a <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basepointmarker.html#strokethickness" rel="noopener noreferrer" target="_blank">strokeThicknessðŸ“˜</a> of 0 can create a better visual.
- We assign aÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basedataseries.html" rel="noopener noreferrer" target="_blank">DataSeriesðŸ“˜</a> - which stores the Xyz data to render, where X,Y is position and Z is scale factor.

This results in the following output:

![](out_scichartv4/2d-charts/chart-types/fast-bubble-renderable-series/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

Because the Bubble Series renders a single point-market but scales for each point, **it is advisable not to use a Stroke on the PointMarker**, as this could get pixellated when the bubble is scaled up or down. This approach of ours results in extremely high performance charts - hundreds of thousands of data-points are possible with SciChart.js.

## Scaling Bubble sizes per-point<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-bubble-renderable-series/#scaling-bubble-sizes-per-point" class="hash-link" aria-label="Direct link to Scaling Bubble sizes per-point" translate="no" title="Direct link to Scaling Bubble sizes per-point">â€‹</a>

![](out_scichartv4/2d-charts/chart-types/fast-bubble-renderable-series/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

The Bubble chart sizes are scaled using the zValue on theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyzdataseries.html" rel="noopener noreferrer" target="_blank">XyzDataSeriesðŸ“˜</a>. By default, the z-value is pixels.

You can scale up/down the entire bubble series by setting theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastbubblerenderableseries.html#zmultiplier" rel="noopener noreferrer" target="_blank">FastBubbleRenderableSeries.zMultiplierðŸ“˜</a> property. Default value=1.

You can modify or edit sizes by adjusting the zValues viaÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyzdataseries.html#updatexyz" rel="noopener noreferrer" target="_blank">xyzDataSeries.updateXyz()ðŸ“˜</a> or similar. See theÂ [DataSeries Documentation](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-series-api/data-series-api-overview) pages for more info about data updates.

Bubble sizes can be scaled using the DataSeries zValue, or the zMultiplier property (see above). What if you wanted to scale a bubble series depending on the zoom level of the viewport? Here's a quick worked example:

- TS

``` prism-code
const bubbleSeries = new FastBubbleRenderableSeries(wasmContext, {
    dataSeries: new XyzDataSeries(wasmContext, { xValues, yValues, zValues: sizes }),
    pointMarker: new EllipsePointMarker(wasmContext, {
        width: 256,
        height: 256,
        strokeThickness: 0,
        fill: "#4682b477"
    })
});

// Adjust zMultiplier based on zoom level
const adjustSeriesStyle = () => {
    const xAxis = sciChartSurface.xAxes.get(0);
    // Get the max range of the xAxis and calculate how zoomed in we are
    const seriesRange = xAxis.getMaximumRange();
    const zoomMultiplier = seriesRange.diff / xAxis.visibleRange.diff;

    // Calculate & apply a zoom factor
    const size =
        (Math.round(sciChartSurface.seriesViewRect.width) * zoomMultiplier) / bubbleSeries.dataSeries.count();
    bubbleSeries.zMultiplier = size * 0.05;
};

const usePreRenderCallback = (sciChartSurface, callback) => {
    let wasRendered = false;

    // initial setup - trigger on first redraw
    sciChartSurface.rendered.subscribe(() => {
        if (!wasRendered) {
            wasRendered = true;
            callback();
        }
    });

    // subsequent calls - trigger before render
    sciChartSurface.preRender.subscribe(() => {
        if (wasRendered) {
            callback();
        }
    });
};

// Callback called before render on SciChartSurface
usePreRenderCallback(sciChartSurface, () => {
    adjustSeriesStyle();
});

sciChartSurface.renderableSeries.add(bubbleSeries);
```

This results in the following output:

## RenderÂ a GapÂ in a Bubble Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-bubble-renderable-series/#rendera-gapin-a-bubble-series" class="hash-link" aria-label="Direct link to RenderÂ a GapÂ in a Bubble Series" translate="no" title="Direct link to RenderÂ a GapÂ in a Bubble Series">â€‹</a>

![](out_scichartv4/2d-charts/chart-types/fast-bubble-renderable-series/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

It is possible to have null points orÂ gapsÂ in aÂ Bubble Series by passing aÂ data point with a **NaN** valueÂ as the **Y** value.Â Or, by simply skipping a point if using a value-axis. Please refer to theÂ [Common Series Features - Draw Gaps in Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/common-series-apis/drawing-gaps)Â article for more details.

## Different Point-Markers on a Bubble Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-bubble-renderable-series/#different-point-markers-on-a-bubble-series" class="hash-link" aria-label="Direct link to Different Point-Markers on a Bubble Series" translate="no" title="Direct link to Different Point-Markers on a Bubble Series">â€‹</a>

Every data point of a Bubble SeriesÂ is marked with aÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basepointmarker.html" rel="noopener noreferrer" target="_blank">PointMarkerðŸ“˜</a>. Several different types of PointMarker are available in SciChart.js:

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/ellipsepointmarker.html" rel="noopener noreferrer" target="_blank">EllipsePointMarkerðŸ“˜</a> - Renders a circle at each point
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/squarepointmarker.html" rel="noopener noreferrer" target="_blank">SquarePointMarkerðŸ“˜</a> - Renders a square at each point
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/trianglepointmarker.html" rel="noopener noreferrer" target="_blank">TrianglePointMarkerðŸ“˜</a> - Renders a triangle at each point
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/crosspointmarker.html" rel="noopener noreferrer" target="_blank">CrossPointMarkerðŸ“˜</a> - Renders a plus sign '+' at each point
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xpointmarker.html" rel="noopener noreferrer" target="_blank">XPointMarkerðŸ“˜</a> - Renders an 'x' at each point
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/spritepointmarker.html" rel="noopener noreferrer" target="_blank">SpritePointMarkerðŸ“˜</a> - Allows an image to be used at each point to create custom pointmarkers

Any of these can be used to create a bubble chart.

![](out_scichartv4/2d-charts/chart-types/fast-bubble-renderable-series/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

To learn more about the types of Point Marker in SciChart.js, see theÂ [Point Markers API documentation](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/common-series-apis/drawing-point-markers).

There is also a TypeScript example of custom pointmarkers in theÂ <a href="https://www.scichart.com/demo/javascript-chart-custom-pointmarkers" rel="noopener noreferrer" target="_blank">SciChart.js Demo.</a>

## Painting Bubbles with Different Colors<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-bubble-renderable-series/#painting-bubbles-with-different-colors" class="hash-link" aria-label="Direct link to Painting Bubbles with Different Colors" translate="no" title="Direct link to Painting Bubbles with Different Colors">â€‹</a>

![](out_scichartv4/2d-charts/chart-types/fast-bubble-renderable-series/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

It is possible to define the colour of PointMarkers individually using theÂ [PaletteProvider API](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/palette-provider-api-overview).

For more info on how to do this, see theÂ [PaletteProvider - Per-point colouring of Scatter Charts](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/fast-bubble-renderable-series) documentation page.

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/fast-bubble-renderable-series/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/fast-bubble-renderable-series/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
