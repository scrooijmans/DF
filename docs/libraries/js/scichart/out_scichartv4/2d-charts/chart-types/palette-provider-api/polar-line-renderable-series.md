On this page

# Per-Point Coloring for Polar Line Series

Polar Line series can be colored per-point or per line-segment using theÂ [PaletteProvider API](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/palette-provider-api-overview). To use this, we must create a class (TS) or object (JS)Â which implements or confirms to theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/istrokepaletteprovider.html" rel="noopener noreferrer" target="_blank">IStrokePaletteProviderðŸ“˜</a> andÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ifillpaletteprovider.html" rel="noopener noreferrer" target="_blank">IFillPaletteProviderðŸ“˜</a> interfaces. Then, apply this to theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarlinerenderableseries.html#paletteprovider" rel="noopener noreferrer" target="_blank">PolarLineRenderableSeries.paletteProviderðŸ“˜</a> property. This allows you to colour data-points based on values, or custom rules with infinite extensiblity.

First, let's create a <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/istrokepaletteprovider.html" rel="noopener noreferrer" target="_blank">PaletteProviderðŸ“˜</a> class like this:

- Creating the PaletteProvider

``` prism-code
const {
    DefaultPaletteProvider,
    EStrokePaletteMode,
    parseColorToUIntArgb
} = SciChart;
// or, for npm, import { DefaultPaletteProvider, ... } from "scichart"

type TRule = (yValue: number, xValue: number) => boolean;

class ThresholdLinePaletteProvider extends DefaultPaletteProvider {
    public rule: TRule;
    public stroke: number;
    public strokePaletteMode = EStrokePaletteMode.SOLID;

    constructor(stroke: string, rule: TRule) {
        super();
        this.strokePaletteMode = EStrokePaletteMode.SOLID;
        this.rule = rule;
        this.stroke = parseColorToUIntArgb(stroke);
    }

    overrideStrokeArgb(xValue: number, yValue: number, index: number, opacity: number, metadata: IPointMetadata) {
        return this.rule(yValue, xValue) 
            ? this.stroke 
            : undefined;
    }
}
```

Next, we can apply the PaletteProvider to the series. This can be done both with the programmatic API and the Builder API:

- TS
- Builder API (JSON Config)

``` prism-code
const polarLine = new PolarLineRenderableSeries(wasmContext, {
    dataSeries: new XyDataSeries(wasmContext, {
        xValues: Array.from({ length: 34 }, (_, i) => i),
        yValues: Array.from({ length: 34 }, (_, i) => 1 + i / 5)
    }),
    stroke: "green",
    strokeThickness: 5,
    interpolateLine: true,
    paletteProvider: new ThresholdLinePaletteProvider(
        "#FFFFFF",
        (yValue, xValue) => Math.floor(xValue / 3) % 2 === 0
    )
});
sciChartSurface.renderableSeries.add(polarLine);
```

``` prism-code
// Demonstrates how to create an interpolated polar line chart with SciChart.js using the Builder API
const {
    EPolarAxisMode,
    EAxisAlignment,
    EPolarLabelMode,
    NumberRange,
    ESeriesType,
    EThemeProviderType,
    chartBuilder,
    EAxisType,
    EBaseType,
    EPaletteProviderType
} = SciChart;
// or, for npm, import { chartBuilder, ... } from "scichart"

// Register the custom ThresholdLinePaletteProvider with the chartBuilder
chartBuilder.registerType(
    EBaseType.PaletteProvider,
    "ThresholdLinePaletteProvider",
    options => new ThresholdLinePaletteProvider(options.stroke, options.rule)
);

const { wasmContext, sciChartSurface } = await chartBuilder.build2DPolarChart(divElementId, {
    surface: { theme: { type: EThemeProviderType.Navy } },
    xAxes: [
        {
            type: EAxisType.PolarNumericAxis,
                options: {
                polarAxisMode: EPolarAxisMode.Angular,
                axisAlignment: EAxisAlignment.Top,
                visibleRange: new NumberRange(0, 12),
                polarLabelMode: EPolarLabelMode.Parallel
            }
        }
    ],
    yAxes: [
        {
            type: SciChart.EAxisType.PolarNumericAxis,
            options: {
                axisAlignment: EAxisAlignment.Right,
                polarAxisMode: EPolarAxisMode.Radial,
                visibleRange: new NumberRange(0, 8),
                labelPrecision: 0,
            }
        }
    ],
    series: [
        {
            type: ESeriesType.PolarLineSeries,
            xyData: {
                xValues: Array.from({ length: 20 }, (_, i) => i),
                yValues: Array.from({ length: 20 }, (_, i) => 1 + i / 3)
            },
            options: {
                stroke: "pink",
                strokeThickness: 4,
                interpolateLine: true,
                paletteProvider: {
                    type: EPaletteProviderType.Custom,
                    customType: "ThresholdLinePaletteProvider",
                    options: {
                        stroke: "#FFFFFF",
                        rule: (yValue: number, xValue: number) => Math.floor(xValue / 3) % 2 === 0
                    }
                }
            }
        }
    ]
});
```

The code above results in a [Polar Line Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-line-renderable-series) with the following rule: **change color on every 3rd point**. The result is shown below:

- We create a `ThresholdLinePaletteProvider` class that extends <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/defaultpaletteprovider.html" rel="noopener noreferrer" target="_blank">DefaultPaletteProviderðŸ“˜</a>
- The <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/defaultpaletteprovider.html#strokepalettemode" rel="noopener noreferrer" target="_blank">strokePaletteModeðŸ“˜</a> is set to <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/estrokepalettemode.html#solid" rel="noopener noreferrer" target="_blank">SOLIDðŸ“˜</a> since we want abrupt color changes based on a condition, not a <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/estrokepalettemode.html#gradient" rel="noopener noreferrer" target="_blank">GRADIENTðŸ“˜</a>
- We override <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/defaultpaletteprovider.html#overridestrokeargb" rel="noopener noreferrer" target="_blank">overrideStrokeArgbðŸ“˜</a> to return another stroke color when our rule is met: `Math.floor(xValue / 3) % 2 === 0`
- When the method returns `undefined`, the default stroke color is used; otherwise, the custom color is applied
- The <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarlinerenderableseries.html#interpolateline" rel="noopener noreferrer" target="_blank">interpolateLineðŸ“˜</a> is set to `true` to create smooth curved segments that follow the polar coordinate system

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/polar-line-renderable-series/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [Line Series PaletteProvider API](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/fast-line-renderable-series) - check out the 2D version of this article for more info

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/palette-provider-api/polar-line-renderable-series/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/palette-provider-api/polar-line-renderable-series/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
