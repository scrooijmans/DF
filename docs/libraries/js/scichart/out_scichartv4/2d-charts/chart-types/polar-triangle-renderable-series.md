On this page

# The Polar Triangle Series Type

The <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polartrianglerenderableseries.html" rel="noopener noreferrer" target="_blank">PolarTriangleRenderableSeriesðŸ“˜</a> creates triangle strips / lists or polyongs in a polar coordinate system, allowing for flexible data visualization in polar charts. This is useful for creating radar charts, polar area charts, or any custom triangular visualizations.

## Creating a Polar Triangle Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-triangle-renderable-series/#creating-a-polar-triangle-series" class="hash-link" aria-label="Direct link to Creating a Polar Triangle Series" translate="no" title="Direct link to Creating a Polar Triangle Series">â€‹</a>

To create a chart using <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polartrianglerenderableseries.html" rel="noopener noreferrer" target="_blank">PolarTriangleRenderableSeriesðŸ“˜</a> you can use use the following code.

- TS

``` prism-code
const radialYAxis = new PolarNumericAxis(wasmContext, {
    polarAxisMode: EPolarAxisMode.Radial
});

sciChartSurface.yAxes.add(radialYAxis);

const angularXAxis = new PolarNumericAxis(wasmContext, {
    polarAxisMode: EPolarAxisMode.Angular
});

sciChartSurface.xAxes.add(angularXAxis);

const trianglesX = [
    [566.0, 551.0, 581.0],
    [529.0, 514.0, 544.0],
    [483.0, 468.0, 498.0],
    [445.0, 430.0, 460.0],
    [409.0, 394.0, 424.0],
    [399.0, 384.0, 414.0],
    [401.0, 386.0, 416.0],
    [412.0, 397.0, 427.0],
    [437.0, 422.0, 452.0],
    [473.0, 458.0, 488.0],
    [527.0, 512.0, 542.0],
    [574.0, 559.0, 589.0]
].flat(); // xValues and yValues take an array of numbers, and we are using an array of arrays just for readability reasons

const trianglesY = [
    [296.32, 270.34, 270.34],
    [309.32, 283.34, 283.34],
    [309.32, 283.34, 283.34],
    [291.32, 265.34, 265.34],
    [267.32, 241.34, 241.34],
    [225.32, 199.34, 199.34],
    [184.32, 158.34, 158.34],
    [146.32, 120.34, 120.34],
    [108.32, 82.34, 82.34],
    [94.32, 68.34, 68.34],
    [94.32, 68.34, 68.34],
    [107.32, 81.34, 81.34]
].flat();

const triangle = new PolarTriangleRenderableSeries(wasmContext, {
    dataSeries: new XyDataSeries(wasmContext, {
        xValues: trianglesX,
        yValues: trianglesY
    }),
    fill: "cornflowerblue",
    opacity: 0.6,
    drawMode: ETriangleSeriesDrawMode.List
});

sciChartSurface.renderableSeries.add(triangle);
```

## Polar Map Example<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-triangle-renderable-series/#polar-map-example" class="hash-link" aria-label="Direct link to Polar Map Example" translate="no" title="Direct link to Polar Map Example">â€‹</a>

![](out_scichartv4/2d-charts/chart-types/polar-triangle-renderable-series/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

The <a href="https://www.scichart.com/demo/javascript/polar-map-example" rel="noopener noreferrer" target="_blank">JavaScript Polar Map Example</a> using the `PolarTriangleRenderableSeries` can be found in theÂ <a href="https://github.com/ABTSoftware/SciChart.JS.Examples/tree/master/Examples/src/components/Examples/Charts2D/PolarCharts/PolarMapExample" rel="noopener noreferrer" target="_blank">SciChart.Js Examples Suite &gt; Polar Map Example</a>Â on Github, or our live demo atÂ <a href="https://www.scichart.com/demo/react/polar-column-chart" rel="noopener noreferrer" target="_blank">scichart.com/demo</a>.

Above: The JavaScript <a href="https://www.scichart.com/demo/iframe/polar-map-example" target="_blank">Polar Map Example</a> example from the <a href="https://www.scichart.com/demo/react" target="_blank">SciChart.js Demo</a>

  

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/polar-triangle-renderable-series/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/polar-triangle-renderable-series/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
