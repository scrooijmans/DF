On this page

# The Polar Band Series Type

The <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarbandrenderableseries.html" rel="noopener noreferrer" target="_blank">PolarBandRenderableSeriesðŸ“˜</a> creates a band or area between two polar curves, displaying the relationship between two sets of yValues on a polar coordinate system.

![](out_scichartv4/2d-charts/chart-types/polar-band-renderable-series/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

The <a href="https://www.scichart.com/demo/javascript/polar-band-chart" rel="noopener noreferrer" target="_blank">JavaScript Polar Band Chart</a> can be found in theÂ <a href="https://github.com/ABTSoftware/SciChart.JS.Examples/blob/release_v4.0/Examples/src/components/Examples/Charts2D/PolarCharts/PolarBandChart" rel="noopener noreferrer" target="_blank">SciChart.Js Examples Suite &gt; Polar Band Chart</a>Â on Github, or our live demo atÂ <a href="https://www.scichart.com/demo/react/polar-band-chart" rel="noopener noreferrer" target="_blank">scichart.com/demo</a>.

Above: The JavaScript <a href="https://www.scichart.com/demo/iframe/polar-band-chart" target="_blank">Polar Band Series Chart</a> example from the <a href="https://www.scichart.com/demo/react" target="_blank">SciChart.js Demo</a>

  

Some of <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipolarbandrenderableseriesoptions.html" rel="noopener noreferrer" target="_blank">IPolarBandRenderableSeriesOptionsðŸ“˜</a>'s key properties include:

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipolarbandrenderableseries.html#dataseries" rel="noopener noreferrer" target="_blank">dataSeriesðŸ“˜</a> - The <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyydataseries.html" rel="noopener noreferrer" target="_blank">XyyDataSeriesðŸ“˜</a> containing `xValues`, `yValues`, and `y1Values` arrays.
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipolarbandrenderableseries.html#fill" rel="noopener noreferrer" target="_blank">fillðŸ“˜</a> - Fill color where Y is greater than Y1
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipolarbandrenderableseries.html#filly1" rel="noopener noreferrer" target="_blank">fillY1ðŸ“˜</a> - Fill color where Y1 is greater than Y
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipolarbandrenderableseries.html#stroke" rel="noopener noreferrer" target="_blank">strokeðŸ“˜</a> - Stroke color for the Y line
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipolarbandrenderableseries.html#strokey1" rel="noopener noreferrer" target="_blank">strokeY1ðŸ“˜</a> - Stroke color for the Y1 line
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipolarbandrenderableseries.html#strokethickness" rel="noopener noreferrer" target="_blank">strokeThicknessðŸ“˜</a> - Thickness of the stroke lines
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipolarbandrenderableseries.html#interpolateline" rel="noopener noreferrer" target="_blank">interpolateLineðŸ“˜</a> - When true, line segments draw as arcs instead of straight lines
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipolarbandrenderableseries.html#scalegradienttoyrange" rel="noopener noreferrer" target="_blank">scaleGradientToYRangeðŸ“˜</a> - Controls gradient scaling behavior for radial axis
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipolarbandrenderableseries.html#filllineargradient" rel="noopener noreferrer" target="_blank">fillLinearGradientðŸ“˜</a> - Linear gradient params where Y is greater than Y1
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipolarbandrenderableseries.html#filllineargradienty1" rel="noopener noreferrer" target="_blank">fillLinearGradientY1ðŸ“˜</a> - Linear gradient params where Y1 is greater than Y
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipolarbandrenderableseries.html#paletteprovider" rel="noopener noreferrer" target="_blank">paletteProviderðŸ“˜</a> - Custom coloring provider for dynamic styling

## Examples<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-band-renderable-series/#examples" class="hash-link" aria-label="Direct link to Examples" translate="no" title="Direct link to Examples">â€‹</a>

### Basic Polar Band Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-band-renderable-series/#basic-polar-band-series" class="hash-link" aria-label="Direct link to Basic Polar Band Series" translate="no" title="Direct link to Basic Polar Band Series">â€‹</a>

``` prism-code
const { sciChartSurface, wasmContext } = await SciChartPolarSurface.create(divElementId, {
    theme: new SciChartJsNavyTheme()
});

const angularXAxis = new PolarNumericAxis(wasmContext, {
    polarAxisMode: EPolarAxisMode.Angular,
    visibleRange: new NumberRange(0, 8),
    drawMinorGridLines: false
});
sciChartSurface.xAxes.add(angularXAxis);

const radialYAxis = new PolarNumericAxis(wasmContext, {
    polarAxisMode: EPolarAxisMode.Radial,
    drawMinorGridLines: false,
    labelPrecision: 0,
    autoTicks: false,
    majorDelta: 1,
    innerRadius: 0.2
});
sciChartSurface.yAxes.add(radialYAxis);

// Define the polar band series
const baiscBand = new PolarBandRenderableSeries(wasmContext, {
    dataSeries: new XyyDataSeries(wasmContext, {
        xValues: [0, 1, 3, 4, 5, 6],
        yValues: [1, 2, 3, 4, 5, 6],
        y1Values: [6, 5, 1, 5, 4, 2]
    }),
    stroke: "rgba(200,200,30,1)", // yellow
    fill: "rgba(200,200,30,0.3)", // thin yellow
    strokeY1: "rgba(200,120,160,1)", // pink
    fillY1: "rgba(200,120,160,0.3)", // thin pink
    strokeThickness: 4,
    interpolateLine: false
});
sciChartSurface.renderableSeries.add(baiscBand);
```

In the code above:

- 2 Polar Band Series instances are created and added to the `sciChartSurface.renderableSeries` collection.
- We set the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarbandrenderableseries.html#stroke" rel="noopener noreferrer" target="_blank">strokeðŸ“˜</a>, <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarbandrenderableseries.html#strokey1" rel="noopener noreferrer" target="_blank">strokeY1ðŸ“˜</a> for the yValues and y1Values respectively, and then <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarbandrenderableseries.html#fill" rel="noopener noreferrer" target="_blank">fillðŸ“˜</a> and <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarbandrenderableseries.html#filly1" rel="noopener noreferrer" target="_blank">fillY1ðŸ“˜</a>, for when `Y > Y1` and vice versa.
- We assign an <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyydataseries.html" rel="noopener noreferrer" target="_blank">XyyDataSeriesðŸ“˜</a> which stores X, Y and Y1 value arrays.

### Gradient Fills in Polar Band Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-band-renderable-series/#gradient-fills-in-polar-band-series" class="hash-link" aria-label="Direct link to Gradient Fills in Polar Band Series" translate="no" title="Direct link to Gradient Fills in Polar Band Series">â€‹</a>

To use Gradient Fills with a <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarbandrenderableseries.html" rel="noopener noreferrer" target="_blank">PolarBandRenderableSeriesðŸ“˜</a>, you need to set the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarbandrenderableseries.html#filllineargradient" rel="noopener noreferrer" target="_blank">fillLinearGradientðŸ“˜</a> & <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarbandrenderableseries.html#filllineargradienty1" rel="noopener noreferrer" target="_blank">fillLinearGradientY1ðŸ“˜</a> properties with a <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/gradientparams.html" rel="noopener noreferrer" target="_blank">GradientParamsðŸ“˜</a> object: a type which defines gradients by a number of <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/gradientparams.html#gradientstops" rel="noopener noreferrer" target="_blank">TGradientStopðŸ“˜</a> objects inside an array and a start and end <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/gradientparams.html#startpoint" rel="noopener noreferrer" target="_blank">PointðŸ“˜</a>.

``` prism-code
const gradientBand = new PolarBandRenderableSeries(wasmContext, {
    dataSeries: new XyyDataSeries(wasmContext, {
        xValues: [0, 1, 2, 3, 4, 5],
        yValues: [7, 4, 1, 5, 6, 7],
        y1Values: [2, 5, 7, 3, 8, 2]
    }),
    stroke: "white",
    strokeY1: "white",
    strokeThickness: 2,
    fillLinearGradient: new GradientParams(new Point(0, 0), new Point(0, 1), [
        { color: "#FF9999", offset: 0 },
        { color: "#FF2222", offset: 1 }
    ]),
    // This one is for gradient where Y1 values are greater than Y2 values
    fillLinearGradientY1: new GradientParams(new Point(0, 0), new Point(0, 1), [
        { color: "#2222FF", offset: 0 },
        { color: "#9999FF", offset: 1 }
    ]),
    interpolateLine: true,
    scaleGradientToYRange: false // set to true to have global gradient depending on Y axis range
});
sciChartSurface.renderableSeries.add(gradientBand);
```

In the code above:

- We create a <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/gradientparams.html" rel="noopener noreferrer" target="_blank">GradientParamsðŸ“˜</a> object with 2 gradient stops, and set the start and end points of the gradient.
- We set <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarbandrenderableseries.html#scalegradienttoyrange" rel="noopener noreferrer" target="_blank">scaleGradientToYRangeðŸ“˜</a> to `true` to make the gradient scale to the Y range of the data for each band segment.
- The <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarbandrenderableseries.html#interpolateline" rel="noopener noreferrer" target="_blank">interpolateLineðŸ“˜</a> flag is set to `true` to make the band wrap around the angular axes in a circular fashion.

### PaletteProvider for Polar Band Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-band-renderable-series/#paletteprovider-for-polar-band-series" class="hash-link" aria-label="Direct link to PaletteProvider for Polar Band Series" translate="no" title="Direct link to PaletteProvider for Polar Band Series">â€‹</a>

By extending <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/defaultpaletteprovider.html" rel="noopener noreferrer" target="_blank">DefaultPaletteProviderðŸ“˜</a> you can create a custom palette for your Polar Band Series, to achieve dynamic coloring based on data values. See more about this topic here [Palette Provider API - Polar Band Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/polar-band-renderable-series).

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/polar-band-renderable-series/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/polar-band-renderable-series/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
