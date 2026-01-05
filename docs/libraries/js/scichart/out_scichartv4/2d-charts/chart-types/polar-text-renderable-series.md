On this page

# The Polar Text Series Type

There are several ways to add text to a SciChart.js polar chart. These include the TextAnnotation / NativeTextAnnotation, renderable series DataLabels and also the PolarTextRenderableSeries (Text Series).

Text Series should be used when you want to render a lot of text, not necessarily at X,Y positions of other chart series. The <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polartextrenderableseries.html" rel="noopener noreferrer" target="_blank">PolarTextRenderableSeriesðŸ“˜</a> is an extension of <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fasttextrenderableseries.html" rel="noopener noreferrer" target="_blank">FastTextRenderableSeriesðŸ“˜</a>, so there is some

![](out_scichartv4/2d-charts/chart-types/polar-text-renderable-series/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

TheÂ <a href="https://www.scichart.com/demo/javascript-text-chart" rel="noopener noreferrer" target="_blank">JavaScript Text / Word Cloud Chart Example</a> can be found in theÂ <a href="https://github.com/ABTSoftware/SciChart.JS.Examples/tree/master/Examples/src/components/Examples/Charts2D/BasicChartTypes/TextSeriesChart" rel="noopener noreferrer" target="_blank">SciChart.Js Examples Suite &gt; Text Series Chart</a>Â on Github, or our live demo atÂ <a href="https://www.scichart.com/demo/javascript-line-chart" rel="noopener noreferrer" target="_blank">scichart.com/demo</a>

## Creating a Polar Text Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-text-renderable-series/#creating-a-polar-text-series" class="hash-link" aria-label="Direct link to Creating a Polar Text Series" translate="no" title="Direct link to Creating a Polar Text Series">â€‹</a>

To create a chart usingÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polartextrenderableseries.html" rel="noopener noreferrer" target="_blank">PolarTextRenderableSeriesðŸ“˜</a> use the following code.Â 

**Note** that it is required to set `style: { fontSize: X }` and `color` in the dataLabels property in order for text to be drawn.Â 

PolarTextRenderableSeries uses the specialÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xytextdataseries.html" rel="noopener noreferrer" target="_blank">XyTextDataSeriesðŸ“˜</a> which allows you to supplyÂ text values directly on the dataSeries, rather than having to use metadata.Â 

- TS

``` prism-code
// Demonstrates how to create a basic polar triangle series chart using SciChart.js
const {
    SciChartPolarSurface,
    PolarNumericAxis,
    PolarTextRenderableSeries,
    EPolarAxisMode,
    XyTextDataSeries,
    SciChartJsNavyTheme,
    NumberRange
} = SciChart;
// or, for npm, import { SciChartPolarSurface, ... } from "scichart"

const { sciChartSurface, wasmContext } = await SciChartPolarSurface.create(divElementId, {
    theme: new SciChartJsNavyTheme(),
});

const radialYAxis = new PolarNumericAxis(wasmContext, {
    polarAxisMode: EPolarAxisMode.Radial,
    visibleRange: new NumberRange(0, 7),
    startAngle: Math.PI,
});
sciChartSurface.yAxes.add(radialYAxis);

const angularXAxis = new PolarNumericAxis(wasmContext, {
    polarAxisMode: EPolarAxisMode.Angular,
    visibleRange: new NumberRange(1, 8),
    flippedCoordinates: true,
    startAngle: Math.PI,
});
sciChartSurface.xAxes.add(angularXAxis);

const textSeries = new PolarTextRenderableSeries(wasmContext, {
    dataSeries: new XyTextDataSeries(wasmContext, {
        xValues: [1, 2, 3, 4, 5, 6],
        yValues: [4, 3, 4, 4, 3, 3],
        textValues: ["This", "text", "is", "drawn", "using", "PolarTextRenderableSeries"]
    }),
    // font size & color are required for text to be drawn
    dataLabels: {
        style: {
            fontSize: 18
        },
        color: "#ffffff"
    }
});
sciChartSurface.renderableSeries.add(textSeries);
```

This results in the following output:Â 

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-text-renderable-series/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [(2D) Fast Text Renderable Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-text-renderable-series)
- [Adding DataLabels to a Chart Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-point-labels/data-labels-api-overview)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/polar-text-renderable-series/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/polar-text-renderable-series/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
