On this page

# Per-Point Colouring of Bubble Charts

## ColouringÂ Bubble Points Individually with PaletteProvider<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/fast-bubble-renderable-series/#colouringbubble-points-individually-with-paletteprovider" class="hash-link" aria-label="Direct link to ColouringÂ Bubble Points Individually with PaletteProvider" translate="no" title="Direct link to ColouringÂ Bubble Points Individually with PaletteProvider">â€‹</a>

It is possible to override the Bubble series point-marker fill & stroke colour on a per-datapoint basis in SciChart.js using the PaletteProvider API.

To use this, we must create a classÂ which implements or confirms to theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmarkerpaletteprovider.html" rel="noopener noreferrer" target="_blank">IPointMarkerPaletteProviderðŸ“˜</a>Â interface. Then, apply this to theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyscatterrenderableseries.html#paletteprovider" rel="noopener noreferrer" target="_blank">XyScatterRenderableSeries.paletteProviderðŸ“˜</a> property. This allows you to colour data-points based on values, or custom rules with infinite extensiblity.

Let's start off by creating a PaletteProvider class:

- TS

``` prism-code
const { DefaultPaletteProvider, EStrokePaletteMode, parseColorToUIntArgb } = SciChart;

// or, for npm, import { DefaultPaletteProvider, ... } from "scichart"

// Custom PaletteProvider for line series which colours datapoints above a threshold
class BubblePaletteProvider extends DefaultPaletteProvider {
    rule: (yValue: number) => boolean;
    overrideFill: number;

    constructor(fill, rule: (yValue: number) => boolean) {
        super();
        this.strokePaletteMode = EStrokePaletteMode.SOLID;
        this.rule = rule;
        // Use the helper function parseColorToUIntArgb to convert a hex string
        // e.g. #FF00FF77 into ARGB numeric format 0xFF00FF77 expected by scichart
        this.overrideFill = parseColorToUIntArgb(fill);
    }

    // This function is called for every data-point.
    // Return undefined to use the default color for the pointmarker,
    // else, return a custom colour as an ARGB color code, e.g. 0xFFFF0000 is red
    overridePointMarkerArgb(xValue, yValue, index, opacity, metadata) {
        // Draw points outside the range a different color
        if (this.rule(yValue)) {
            return { stroke: this.overrideFill, fill: this.overrideFill };
        }
        // Undefined means use default colors
        return undefined;
    }
}
```

![](out_scichartv4/2d-charts/chart-types/palette-provider-api/fast-bubble-renderable-series/index_media/0f0874621de662630fd5d7f09c89455c4ace26e6.svg)warning

It is important that overridePointMarkerArgb returns an object containingÂ both stroke and fill, even if stroke is ignored on the bubble series.

Next, we can apply the PaletteProvider to a Bubble Series. This can be done both with the programmatic API and the Builder API:

- TS
- Builder API (JSON Config)

``` prism-code
// The BubblePaletteProvider we created before is applied to a FastBubbleRenderableSeries
const bubbleSeries = new FastBubbleRenderableSeries(wasmContext, {
    dataSeries: xyzDataSeries,
    opacity: 1,
    pointMarker: new EllipsePointMarker(wasmContext, {
        // choose a suitably large size for pointmarker. This will  be scaled per-point
        width: 64,
        height: 64,
        strokeThickness: 0,
        fill: "#4682b477"
    }),
    // PaletteProvider feature allows coloring per-point based on a rule
    paletteProvider: new BubblePaletteProvider("Red", yValue => yValue > -0.8)
});

sciChartSurface.renderableSeries.add(bubbleSeries);
```

``` prism-code
// Register the custom BubblePaletteProvider with the chartBuilder
chartBuilder.registerType(
    EBaseType.PaletteProvider,
    "BubblePaletteProvider",
    options => new BubblePaletteProvider(options.fill, options.rule)
);

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
                },
                // Now you can instantiate using parameters below
                paletteProvider: {
                    type: EPaletteProviderType.Custom,
                    customType: "BubblePaletteProvider",
                    options: {
                        fill: "Red",
                        rule: yValue => yValue >= -0.8
                    }
                }
                // Note: Assigning an instance is also valid, e.g.
                // paletteProvider: new BubblePaletteProvider("Green", "Red", yValue => yValue >= 4.0)
            }
        }
    ]
});
```

This results in the following output. Values aboveÂ the thresholdÂ are coloured red, while values below this threshold use the default colour.

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/fast-bubble-renderable-series/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [PaletteProvider API](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/palette-provider-api-overview)
- [Per-point Colouring of Line Segments](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/fast-line-renderable-series)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/palette-provider-api/fast-bubble-renderable-series/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/palette-provider-api/fast-bubble-renderable-series/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
