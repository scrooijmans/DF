On this page

# Per-Point Coloring for Triangle Series

Triangle series can be colored per-point using theÂ [PaletteProvider API](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/palette-provider-api-overview). To use this, we must create a class (typescript) or object (javascript)Â which implements or confirms to theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/istrokepaletteprovider.html" rel="noopener noreferrer" target="_blank">IStrokePaletteProviderðŸ“˜</a> andÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ifillpaletteprovider.html" rel="noopener noreferrer" target="_blank">IFillPaletteProviderðŸ“˜</a>Â interfaces. Then, apply this to theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fasttrianglerenderableseries.html#paletteprovider" rel="noopener noreferrer" target="_blank">FastTriangleRenderableSeries.paletteProviderðŸ“˜</a> property. This allows you to colour data-points based on values, or custom rules with infinite extensiblity.

First, let's create a PaletteProvider class like this:

- TS

``` prism-code
import { EFillPaletteMode, IFillPaletteProvider, parseColorToUIntArgb } from "scichart";

// Custom PaletteProvider for triangle series which colours datapoints above a threshold
const COLORS = ["#f39c12", "#27ae60", "#2980b9", "#8e44ad" ]

class TrianglePaletteProvider implements IFillPaletteProvider {
    public readonly fillPaletteMode = EFillPaletteMode.SOLID;

    public onAttached(): void {}

    public onDetached(): void {}

    public overrideFillArgb(_xValue: number, _yValue: number, index: number, opacity: number): number {
        const opacityFix = Math.round(opacity * 255);

        return parseColorToUIntArgb(COLORS[Math.floor(index / 3)], opacityFix);
    }
}
```

Next, we can apply the PaletteProvider to the series. This can be done both with the programmatic API:

- TS

``` prism-code
const coordinates = [
    [0, 150],
    [0, 50],
    [50, 0],
    [150, 0],
    [200, 50],
    [200, 150],
    [150, 200],
    [50, 200],
    [0, 150],
    [0, 50]
];
const dataSeries = new XyDataSeries(wasmContext, {
    xValues: coordinates.map(p => p[0]),
    yValues: coordinates.map(p => p[1])
});

const triangleSeries = new FastTriangleRenderableSeries(wasmContext, {
    dataSeries,
    drawMode: ETriangleSeriesDrawMode.Strip, // each group of three consecutive points in the list defines a triangle, every point is connected to the last two points
    fill: "cornflowerblue",
    opacity: 0.5,
    paletteProvider: new TrianglePaletteProvider()
});
```

![](out_scichartv4/2d-charts/chart-types/palette-provider-api/fast-triangle-renderable-series/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

For a more detailed example of <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fasttrianglerenderableseries.html" rel="noopener noreferrer" target="_blank">FastTriangleRenderableSeriesðŸ“˜</a>, see the <a href="https://www.scichart.com/demo/iframe/treemap-chart" rel="noopener noreferrer" target="_blank">Javascript Treemap Chart Example</a>. Each rectangle is coloured based on its value, the larger the value, the darker the green, while negative values are coloured red in the same fashion.

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/fast-triangle-renderable-series/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [PaletteProvider API](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/palette-provider-api-overview)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/palette-provider-api/fast-triangle-renderable-series/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/palette-provider-api/fast-triangle-renderable-series/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
