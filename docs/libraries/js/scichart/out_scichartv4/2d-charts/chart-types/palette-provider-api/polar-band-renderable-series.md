On this page

# Per-Point Coloring for Polar Band Series

[Polar Band series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-band-renderable-series) can be colored per-point or per line-segment using theÂ [PaletteProvider API](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/palette-provider-api-overview). To use this, we must create a class (TS) or object (JS)Â which implements or confirms to theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/istrokepaletteprovider.html" rel="noopener noreferrer" target="_blank">IStrokePaletteProviderðŸ“˜</a> andÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ifillpaletteprovider.html" rel="noopener noreferrer" target="_blank">IFillPaletteProviderðŸ“˜</a> interfaces. Then, apply this to theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarbandrenderableseries.html#paletteprovider" rel="noopener noreferrer" target="_blank">PolarBandRenderableSeries.paletteProviderðŸ“˜</a> property. This allows you to colour data-points based on values, or custom rules with infinite extensiblity.

First, let's create a <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/istrokepaletteprovider.html" rel="noopener noreferrer" target="_blank">PaletteProviderðŸ“˜</a> class like this:

- Creating the PaletteProvider

``` prism-code
const {
    DefaultPaletteProvider,
    EStrokePaletteMode,
    EFillPaletteMode,
    parseColorToUIntArgb
} = SciChart;
// or, for npm, import { DefaultPaletteProvider, ... } from "scichart"

// Custom PaletteProvider for line series which colours datapoints above a threshold
class PolarBandPaletteProvider extends DefaultPaletteProvider {
    strokePaletteMode = EStrokePaletteMode.SOLID;
    fillPaletteMode = EFillPaletteMode.SOLID;

    orange = parseColorToUIntArgb("#DD8800");
    lightOrange = parseColorToUIntArgb("#DD880044");

    overrideFillArgb(xValue: number, yValue: number, index: number, opacity: number, metadata: any) {
        if ((xValue >= 3 && xValue < 6) || (xValue >= 9 && xValue < 12)) {
            return this.lightOrange;
        } else {
            return undefined;
        }
    }

    overrideStrokeArgb(xValue: number, yValue: number, index: number, opacity: number, metadata: any) {
        if ((xValue > 3 && xValue <= 6) || (xValue > 9 && xValue <= 12)) {
            return this.orange;
        } else {
            return undefined; // use the default stroke color
        }
    }
}
```

Next, we can apply the PaletteProvider to the series. This can be done both with the programmatic API and the Builder API:

- TS
- Builder API (JSON Config)

``` prism-code
const polarBand1 = new PolarBandRenderableSeries(wasmContext, {
    dataSeries: new XyyDataSeries(wasmContext, {
        xValues: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
        yValues: [1, 2.5, 3, 1, 2.5, 3, 1, 2.5, 3, 1, 2.5, 3, 1],
        y1Values: [2, 5, 6, 2, 5, 6, 2, 5, 6, 2, 5, 6, 2]
    }),
    stroke: "#FF0000",
    fill: "#FF000044",
    strokeThickness: 3,
    paletteProvider: new PolarBandPaletteProvider()
});
```

``` prism-code
// Demonstrates how to create a band chart with SciChart.js using the Builder API
const {
    EThemeProviderType,
    chartBuilder,
    EPolarAxisMode,
    EAxisAlignment,
    EPolarLabelMode,
    EAxisType,
    ESeriesType,
} = SciChart;
// or, for npm, import { chartBuilder, ... } from "scichart"

const { wasmContext, sciChartSurface } = await chartBuilder.build2DPolarChart(divElementId, {
    surface: { theme: { type: EThemeProviderType.Navy } },
    xAxes: [
        {
            type: EAxisType.PolarNumericAxis,
                options: {
                polarAxisMode: EPolarAxisMode.Angular,
                axisAlignment: EAxisAlignment.Top,
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
                labelPrecision: 0,
                innerRadius: 0.1
            }
        }
    ],
    series: [
        {
            type: ESeriesType.PolarBandSeries,
            xyyData: {
                xValues: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
                yValues: [1, 2.5, 3, 1, 2.5, 3, 1, 2.5, 3, 1, 2.5, 3, 1],
                y1Values: [2, 5, 6, 2, 5, 6, 2, 5, 6, 2, 5, 6, 2]
            },
            options: {
                stroke: "#FF0000",
                fill: "#FF000044",
                strokeThickness: 3,
                paletteProvider: new PolarBandPaletteProvider()
            }
        }
    ]
});

return { sciChartSurface, wasmContext };
```

The code above results in a [Polar Band Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-band-renderable-series) with the following rule: **change color once every 3 points**. The result is shown below:

In the code above:

- We create a class that extends <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/defaultpaletteprovider.html" rel="noopener noreferrer" target="_blank">DefaultPaletteProviderðŸ“˜</a> to override <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/defaultpaletteprovider.html#overridefillargb" rel="noopener noreferrer" target="_blank">overrideFillArgbðŸ“˜</a> and <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/defaultpaletteprovider.html#overridestrokeargb" rel="noopener noreferrer" target="_blank">overrideStrokeArgbðŸ“˜</a> methods by `xValue`, more specifically, the orange fill & stroke when this custom rule is met:

``` prism-code
((xValue > 3 && xValue <= 6) || (xValue > 9 && xValue <= 12))
```

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/polar-band-renderable-series/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [Band Series PaletteProvider API](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/fast-band-renderable-series) - check out the 2D version of this article for more info

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/palette-provider-api/polar-band-renderable-series/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/palette-provider-api/polar-band-renderable-series/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
