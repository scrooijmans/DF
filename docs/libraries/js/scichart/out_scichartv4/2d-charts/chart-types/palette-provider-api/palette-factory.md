On this page

# The PaletteFactory Helper Class

We've created a helper class calledÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/palettefactory.html" rel="noopener noreferrer" target="_blank">PaletteFactoryðŸ“˜</a> to create some commonly used PaletteProviders. These work with all series types in SciChart.js that support the PaletteProvider API.

## PaletteFactory.createYGradient<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/palette-factory/#palettefactorycreateygradient" class="hash-link" aria-label="Direct link to PaletteFactory.createYGradient" translate="no" title="Direct link to PaletteFactory.createYGradient">â€‹</a>

The functionÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/palettefactory.html#createygradient" rel="noopener noreferrer" target="_blank">PaletteFactory.createYGradientðŸ“˜</a> generates aÂ [PaletteProvider API](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/palette-provider-api-overview) for use in renderable series which applies a gradient fill dependent on Y-value.

Below find an example of usage:

- TS

``` prism-code
const yGradientPalette = PaletteFactory.createYGradient(
    wasmContext,
    new GradientParams(new Point(0, 0), new Point(0, 1), [
        { offset: 0, color: "#3333FF" },
        { offset: 0.5, color: "#33FFAA" },
        { offset: 1, color: "#FF6600" }
    ]),
    // the range of y-values to apply the gradient to
    new NumberRange(0, 1.5),
    // Optional parameters to control which elements of the palette are enabled
    {
        enableFill: false, // Applies to fills on mountain, column
        enableStroke: true, // Applies to stroke on all series
        enablePointMarkers: true, // Applies to pointmarkers if present
        strokeOpacity: 1.0,
        pointMarkerOpacity: 0.7,
        fillOpacity: 0.0
    }
);

sciChartSurface.renderableSeries.add(
    new FastLineRenderableSeries(wasmContext, {
        strokeThickness: 5,
        dataSeries: new XyDataSeries(wasmContext, { xValues, yValues }),
        pointMarker: new EllipsePointMarker(wasmContext, {
            width: 20,
            height: 20,
            strokeThickness: 0
        }),
        paletteProvider: yGradientPalette
    })
);
```

This creates a Y-Gradient from blue, to green to red for Y-values ranging fromÂ 0 to +1.5. Values outside that range are clamped to the colours at the start/end of the list of gradient stops.

Here's an example output & codepen you can edit to try this out:Â 

To separately control the output of the generatedÂ [PaletteProvider](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/palette-provider-api-overview), check theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/igradientpaletteoptions.html" rel="noopener noreferrer" target="_blank">IGradientPaletteOptionsðŸ“˜</a> parameter passed in. Using this, you can enable fill, stroke, pointmarkers and opacity for different elements of the series.

## PaletteFactory.createGradient<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/palette-factory/#palettefactorycreategradient" class="hash-link" aria-label="Direct link to PaletteFactory.createGradient" translate="no" title="Direct link to PaletteFactory.createGradient">â€‹</a>

Another helper functionÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/palettefactory.html#creategradient" rel="noopener noreferrer" target="_blank">PaletteFactory.createGradientðŸ“˜</a> allows you to create gradient fills in the X-Direction. The parameters for this are largely the same.

Below find an example of usage:

- TS

``` prism-code
const gradientPalette = PaletteFactory.createGradient(
    wasmContext,
    new GradientParams(new Point(0, 0), new Point(1, 1), [
        { color: "red", offset: 0 },
        { color: "pink", offset: 0.2 },
        { color: "yellow", offset: 0.5 },
        { color: "purple", offset: 0.7 },
        { color: "green", offset: 1 }
    ]),
    // Optional parameters to control which elements of the palette are enabled
    {
        enableFill: false, // Applies to fills on mountain, column
        enableStroke: true, // Applies to stroke on all series
        enablePointMarkers: true, // Applies to pointmarkers if present
        strokeOpacity: 1.0,
        pointMarkerOpacity: 0.7,
        fillOpacity: 0.0
    }
);

sciChartSurface.renderableSeries.add(
    new FastLineRenderableSeries(wasmContext, {
        strokeThickness: 5,
        dataSeries: new XyDataSeries(wasmContext, { xValues, yValues }),
        pointMarker: new EllipsePointMarker(wasmContext, {
            width: 20,
            height: 20,
            strokeThickness: 0
        }),
        paletteProvider: gradientPalette
    })
);
```

This creates a X-Gradient from Red, to Yellow, Purple to Green for X-values ranging fromÂ the start of the series to the end. Values outside that range are clamped to the colours at the start/end of the list of gradient stops.

Here's an example output & codepen you can edit to try this out:Â 

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/palette-provider-api/palette-factory/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/palette-provider-api/palette-factory/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
