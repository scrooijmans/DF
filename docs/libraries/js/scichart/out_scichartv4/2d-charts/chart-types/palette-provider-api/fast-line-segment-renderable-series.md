On this page

# Per-Point Coloring for Line Segment Series

Line Segment series can be colored per line-segment using theÂ [PaletteProvider API](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/palette-provider-api-overview). To use this, we must create a class (typescript) or object (javascript)Â which implements or confirms to theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/istrokepaletteprovider.html" rel="noopener noreferrer" target="_blank">IStrokePaletteProviderðŸ“˜</a> interface. Then, apply this to theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries.html#paletteprovider" rel="noopener noreferrer" target="_blank">FastLineSegmentRenderableSeries.paletteProviderðŸ“˜</a> property.

Let's start off with the PaletteProvider class:

- TS

``` prism-code
const { DefaultPaletteProvider, EStrokePaletteMode, parseColorToUIntArgb, EPaletteProviderType } = SciChart;
// or, for npm, import { DefaultPaletteProvider, ... } from "scichart"

// Custom PaletteProvider for line segment series which colours datapoints differently on odd and even points (differently on both ends)
class LineSegmentPaletteProvider extends DefaultPaletteProvider {
    public readonly strokePaletteMode = EStrokePaletteMode.GRADIENT;
    private readonly palettedStart = parseColorToUIntArgb("orange");
    private readonly palettedEnd = parseColorToUIntArgb("cyan");

    // tslint:disable-next-line:no-empty
    public onAttached(parentSeries: IRenderableSeries): void {}

    // tslint:disable-next-line:no-empty
    public onDetached(): void {}

    public overrideStrokeArgb(xValue: number, yValue: number, index: number): number {
        return index % 2 === 0 ? this.palettedStart : this.palettedEnd;
    }
}
```

Next, we can apply the PaletteProvider to the line segment series. This can be done both with the programmatic API and the Builder API:

- TS

``` prism-code
function generateVectorFieldSegments(gridSize = 30) {
    const xValues = [];
    const yValues = [];

    const spacing = 10 / (gridSize - 1); // spacing between grid points
    const scale = 0.6; // scale for vector length

    for (let i = 0; i < gridSize; i++) {
        for (let j = 0; j < gridSize; j++) {
            const x = i * spacing;
            const y = j * spacing;

            // Vector field direction (can modify here)
            const angle = Math.sin(x) + Math.cos(y);
            const dx = Math.cos(angle) * scale;
            const dy = Math.sin(angle) * scale;

            const x1 = x;
            const y1 = y;
            const x2 = x + dx;
            const y2 = y + dy;

            // Ensure segments stay in [0, 10]
            if (x2 < 0 || x2 > 10 || y2 < 0 || y2 > 10) continue;

            xValues.push(x1, x2);
            yValues.push(y1, y2);
        }
    }

    return { xValues, yValues };
}
const { xValues, yValues } = generateVectorFieldSegments(30);

const lineSegment1 = new FastLineSegmentRenderableSeries(wasmContext, {
    dataSeries: new XyDataSeries(wasmContext, {
        xValues: xValues,
        yValues: yValues,
    }),
    strokeThickness: 3,
    paletteProvider: new LineSegmentPaletteProvider(),
});
sciChartSurface.renderableSeries.add(lineSegment1);
```

This results in the following output:

In TypeScript you only need to implement an interface such asÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/istrokepaletteprovider.html" rel="noopener noreferrer" target="_blank">IStrokePaletteProviderðŸ“˜</a>, whereas in JavaScript you must extend theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/defaultpaletteprovider.html" rel="noopener noreferrer" target="_blank">DefaultPaletteProviderðŸ“˜</a> class.

![](out_scichartv4/2d-charts/chart-types/palette-provider-api/fast-line-segment-renderable-series/index_media/5a72c3ee6a9cc14296f901057d6eabf2b9b712b4.svg)note

SciChart won't bisect the line at a threshold value but only changes colour between line segments in the data you already have.

That being said, a <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/estrokepalettemode.html#solid" rel="noopener noreferrer" target="_blank">EStrokePaletteMode.SOLIDðŸ“˜</a> transition will not work with 2 points and a line drawn in between them (as it is done within the segment) - the result will only show 1 colour, so try to stick to <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/estrokepalettemode.html#gradient" rel="noopener noreferrer" target="_blank">EStrokePaletteMode.GRADIENTðŸ“˜</a> for line segments' PaletteProviders.

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/fast-line-segment-renderable-series/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [PaletteProvider API](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/palette-provider-api-overview)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/palette-provider-api/fast-line-segment-renderable-series/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/palette-provider-api/fast-line-segment-renderable-series/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
