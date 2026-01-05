On this page

# Per-Point Coloring for Rectangle Series

Rectangle series can be colored per-point using theÂ [PaletteProvider API](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/palette-provider-api-overview). To use this, we must create a class (typescript) or object (javascript)Â which implements or confirms to theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/istrokepaletteprovider.html" rel="noopener noreferrer" target="_blank">IStrokePaletteProviderðŸ“˜</a> andÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ifillpaletteprovider.html" rel="noopener noreferrer" target="_blank">IFillPaletteProviderðŸ“˜</a>Â interfaces. Then, apply this to theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastrectanglerenderableseries.html#paletteprovider" rel="noopener noreferrer" target="_blank">FastRectangleRenderableSeries.paletteProviderðŸ“˜</a> property. This allows you to colour data-points based on values, or custom rules with infinite extensiblity.

First, let's create a PaletteProvider class like this:

- TS

``` prism-code
import { DefaultPaletteProvider, EStrokePaletteMode, parseColorToUIntArgb } from "scichart";

// Custom PaletteProvider for rectangle series which colours datapoints above a threshold
class RectanglePaletteProvider extends DefaultPaletteProvider {
    threshold: number;
    stroke: number;
    fillColor: number;

    constructor(threshold: number) {
        super();
        this.strokePaletteMode = EStrokePaletteMode.SOLID;
        this.threshold = threshold;
        this.stroke = parseColorToUIntArgb("#FF0000");
        this.fillColor = parseColorToUIntArgb("#FF000077");
    }

    // This function is called for every data-point.
    // Return undefined to use the default color for the rectangle,
    // else, return a custom colour as an ARGB color code, e.g. 0xFFFF0000 is red
    overrideStrokeArgb(xValue: number, yValue: number, index: number, opacity: number, metadata: any) {
        return yValue > this.threshold 
            ? this.fillColor 
            : undefined;
    }

    // This function is called for every data-point
    // Return undefined to use the default color for the fill, else, return
    // a custom color as ARGB color code e.g. 0xFFFF0000 is red
    overrideFillArgb(xValue: number, yValue: number, index: number, opacity: number, metadata: any) {
        return yValue > this.threshold 
            ? this.fillColor 
            : undefined;
    }
}
```

Next, we can apply the PaletteProvider to the series. This can be done both with the programmatic API:

- TS

``` prism-code
const rectangleSeries = new FastRectangleRenderableSeries(wasmContext, {
    dataSeries: new XyxyDataSeries(wasmContext, {
        xValues: [0, 6, 10, 17, 23],
        yValues: [0, 6, 2, 5, 8],
        x1Values: [5, 9, 15, 25, 30],
        y1Values: [5, 9, 8, 10, 15]
    }),
    columnXMode: EColumnMode.StartEnd, // x, x1
    columnYMode: EColumnYMode.TopBottom, // y, y1
    fill: "skyblue",
    stroke: "white",
    strokeThickness: 2,
    opacity: 0.5,
    paletteProvider: new RectanglePaletteProvider(4.5) // rectangles with yValue > 4.5 will be coloured red
});
```

![](out_scichartv4/2d-charts/chart-types/palette-provider-api/fast-rectangle-renderable-series/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

For a more detailed example of <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastrectanglerenderableseries.html" rel="noopener noreferrer" target="_blank">FastRectangleRenderableSeriesðŸ“˜</a>, see the <a href="https://www.scichart.com/demo/iframe/treemap-chart" rel="noopener noreferrer" target="_blank">Javascript Treemap Chart Example</a>. Each rectangle is coloured based on its value, the larger the value, the darker the green, while negative values are coloured red in the same fashion.

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/fast-rectangle-renderable-series/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [PaletteProvider API](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/palette-provider-api-overview)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/palette-provider-api/fast-rectangle-renderable-series/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/palette-provider-api/fast-rectangle-renderable-series/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
