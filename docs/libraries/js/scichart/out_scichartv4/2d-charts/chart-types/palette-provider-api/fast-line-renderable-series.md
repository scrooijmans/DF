On this page

# Per-point Colouring for Line Series

Line series can be colored per-point or per line-segment using theÂ [PaletteProvider API](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/palette-provider-api-overview). To use this, we must create a class (typescript) or object (javascript)Â which implements or confirms to theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/istrokepaletteprovider.html" rel="noopener noreferrer" target="_blank">IStrokePaletteProviderðŸ“˜</a> interface. Then, apply this to theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries.html#paletteprovider" rel="noopener noreferrer" target="_blank">FastLineRenderableSeries.paletteProviderðŸ“˜</a> property.

Let's start off with the PaletteProvider class:

- TS

``` prism-code
const { DefaultPaletteProvider, EStrokePaletteMode, parseColorToUIntArgb } = SciChart;

// or, for npm, import { DefaultPaletteProvider, ... } from "scichart"

// Custom PaletteProvider for line series which colours datapoints above a threshold
class ThresholdLinePaletteProvider extends DefaultPaletteProvider {
    rule: any;
    stroke: number;
    constructor(stroke, rule) {
        super();
        this.strokePaletteMode = EStrokePaletteMode.SOLID;
        this.rule = rule;
        this.stroke = parseColorToUIntArgb(stroke);
    }

    // This function is called for every data-point.
    // Return undefined to use the default color for the line,
    // else, return a custom colour as an ARGB color code, e.g. 0xFFFF0000 is red
    overrideStrokeArgb(xValue, yValue, index, opacity, metadata) {
        return this.rule(yValue) ? this.stroke : undefined;
    }
}
```

Next, we can apply the PaletteProvider to the line series. This can be done both with the programmatic API and the Builder API:

- TS
- Builder API (JSON Config)

``` prism-code
// Demonstrates how to create a line chart with PaletteProvider using SciChart.js
const { SciChartSurface, NumericAxis, FastLineRenderableSeries, XyDataSeries, SciChartJsNavyTheme } = SciChart;

// or, for npm, import { SciChartSurface, ... } from "scichart"

const { wasmContext, sciChartSurface } = await SciChartSurface.create(divElementId, {
    theme: new SciChartJsNavyTheme()
});
sciChartSurface.xAxes.add(new NumericAxis(wasmContext));
sciChartSurface.yAxes.add(new NumericAxis(wasmContext));

const xValues = [];
const yValues = [];
for (let i = 0; i < 100; i++) {
    xValues.push(i);
    yValues.push(0.2 * Math.sin(i * 0.1) - Math.cos(i * 0.01));
}

const xyDataSeries = new XyDataSeries(wasmContext, {
    xValues,
    yValues
});

// The ThresholdLinePaletteProvider we created before is applied to a FastLineRenderableSeries
const lineSeries = new FastLineRenderableSeries(wasmContext, {
    stroke: "#F48420",
    strokeThickness: 5,
    dataSeries: xyDataSeries,
    paletteProvider: new ThresholdLinePaletteProvider("#30BC9A", yValue => yValue > -0.8)
});

sciChartSurface.renderableSeries.add(lineSeries);
```

``` prism-code

// Demonstrates how to create a chart with a custom PaletteProvider, using the builder API
const { chartBuilder, EBaseType, ESeriesType, EPaletteProviderType, EThemeProviderType } = SciChart;

// or, for npm, import { chartBuilder, ... } from "scichart"

// Register the custom ThresholdLinePaletteProvider with the chartBuilder
chartBuilder.registerType(
    EBaseType.PaletteProvider,
    "ThresholdLinePaletteProvider",
    options => new ThresholdLinePaletteProvider(options.stroke, options.rule)
);

// Now use the Builder-API to build the chart
const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    surface: { theme: { type: EThemeProviderType.Dark } },
    series: [
        {
            type: ESeriesType.LineSeries,
            xyData: {
                xValues: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
                yValues: [2.5, 3.5, 3.7, 3.99, 4.0, 5.0, 5.5, 5.0, 4.0, 3.0]
            },
            options: {
                stroke: "#FF6600",
                strokeThickness: 5,
                // Now you can instantiate using parameters below
                paletteProvider: {
                    type: EPaletteProviderType.Custom,
                    customType: "ThresholdLinePaletteProvider",
                    options: {
                        stroke: "Green",
                        rule: yValue => yValue >= 4.0
                    }
                }
                // Note: Assigning an instance is also valid, e.g.
                // paletteProvider: new ThresholdLinePaletteProvider("Green", yValue => yValue >= 4.0)
            }
        }
    ]
});
```

This results in the following output:

In TypeScript you only need to implement an interface such asÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/istrokepaletteprovider.html" rel="noopener noreferrer" target="_blank">IStrokePaletteProviderðŸ“˜</a>, whereas in JavaScript you must extend theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/defaultpaletteprovider.html" rel="noopener noreferrer" target="_blank">DefaultPaletteProviderðŸ“˜</a> class.

## Gradient Transitions in Lines<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/fast-line-renderable-series/#gradient-transitions-in-lines" class="hash-link" aria-label="Direct link to Gradient Transitions in Lines" translate="no" title="Direct link to Gradient Transitions in Lines">â€‹</a>

ChangeÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/defaultpaletteprovider.html#strokepalettemode" rel="noopener noreferrer" target="_blank">strokePaletteModeðŸ“˜</a> toÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/estrokepalettemode.html" rel="noopener noreferrer" target="_blank">EStrokePaletteMode.GRADIENTðŸ“˜</a> to get a gradient color transition:

- JS
- TS

``` prism-code
class LinePaletteProvider extends DefaultPaletteProvider {
    constructor(stroke, rule) {
        super();
        this.strokePaletteMode = EStrokePaletteMode.GRADIENT;
        // ..
    }
    // ..
}
```

``` prism-code
class LinePaletteProvider implements IStrokePaletteProvider {
    readonly strokePaletteMode: EStrokePaletteMode = EStrokePaletteMode.GRADIENT;
    // ...
}
```

This now results in gradient color changes between line segments.

<img src="out_scichartv4/2d-charts/chart-types/palette-provider-api/fast-line-renderable-series/index_media/1391ebd071e7da19c1e23fb1e7da9469d4c2f69a.png" style="display:block;margin-left:auto;margin-right:auto;object-fit:contain;height:auto;width:85%;margin:0 auto" />

Note: SciChart won't bisect the line at a threshold value but only changes colour between line segments in the data you already have. If you want to have a perfect transistion from one colour to another at a specific Y-value, you will need to include data-points

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/fast-line-renderable-series/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [PaletteProvider API](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/palette-provider-api-overview)
- [Per-point Colouring of Mountain Segments](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/fast-mountain-renderable-series)
- [Per-Point Colouring of Band Segments](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/fast-band-renderable-series)
- [Per-Point Colouring of Bubble Charts](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/fast-bubble-renderable-series)
- [Per-Point Colouring of Candlestick / OHLC Charts](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/fast-candlestick-ohlc-renderable-series)
- [Per-Point Colouring of Scatter Charts (or PointMarkers)](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/xy-scatter-renderable-series)
- [Per-Point Colouring of Column Charts](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/fast-column-renderable-series)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/palette-provider-api/fast-line-renderable-series/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/palette-provider-api/fast-line-renderable-series/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
