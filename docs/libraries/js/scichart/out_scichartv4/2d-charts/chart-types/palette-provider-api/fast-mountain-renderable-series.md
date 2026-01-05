On this page

# Per-point Colouring of Mountain Segments

Mountain series can be colored per-point using theÂ [PaletteProvider API](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/palette-provider-api-overview). To use this, we must create a class (typescript) or object (javascript)Â which implements or confirms to theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/istrokepaletteprovider.html" rel="noopener noreferrer" target="_blank">IStrokePaletteProviderðŸ“˜</a> andÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ifillpaletteprovider.html" rel="noopener noreferrer" target="_blank">IFillPaletteProviderðŸ“˜</a>Â interfaces. Then, apply this to theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastmountainrenderableseries.html#paletteprovider" rel="noopener noreferrer" target="_blank">FastMountainRenderableSeries.paletteProviderðŸ“˜</a> property. This allows you to colour data-points based on values, or custom rules with infinite extensiblity.

First, let's create a PaletteProvider class like this:

- TS

``` prism-code
const { DefaultPaletteProvider, EStrokePaletteMode, parseColorToUIntArgb } = SciChart;

// or, for npm, import { DefaultPaletteProvider, ... } from "scichart"

// Custom PaletteProvider for line series which colours datapoints above a threshold
class MountainPaletteProvider extends DefaultPaletteProvider {
    threshold: number;
    stroke: number;
    fillColor: number;
    constructor(threshold) {
        super();
        this.strokePaletteMode = EStrokePaletteMode.SOLID;
        this.threshold = threshold;
        this.stroke = parseColorToUIntArgb("#FF0000");
        this.fillColor = parseColorToUIntArgb("#FF000077");
    }

    // This function is called for every data-point.
    // Return undefined to use the default color for the line,
    // else, return a custom colour as an ARGB color code, e.g. 0xFFFF0000 is red
    overrideStrokeArgb(xValue, yValue, index, opacity, metadata) {
        return xValue > this.threshold ? this.fillColor : undefined;
    }

    // This function is called for every data-point
    // Return undefined to use the default color for the fill, else, return
    // a custom color as ARGB color code e.g. 0xFFFF0000 is red
    overrideFillArgb(xValue, yValue, index, opacity, metadata) {
        return xValue > this.threshold ? this.fillColor : undefined;
    }
}
```

Next, we can apply the PaletteProvider to the series. This can be done both with the programmatic API and the Builder API:

- TS
- Builder API (JSON Config)

``` prism-code
const threshold = 75;
// Create a mountain series & add to the chart
const mountainSeries = new FastMountainRenderableSeries(wasmContext, {
    dataSeries: new XyDataSeries(wasmContext, { xValues, yValues }),
    stroke: "#4682b4",
    strokeThickness: 3,
    zeroLineY: 0.0,
    // when a solid color is required, use fill
    fill: "rgba(176, 196, 222, 0.7)",
    // when a gradient is required, use fillLinearGradient
    fillLinearGradient: new GradientParams(new Point(0, 0), new Point(0, 1), [
        { color: "rgba(70,130,180,0.77)", offset: 0 },
        { color: "rgba(70,130,180,0.0)", offset: 1 }
    ]),
    // Apply the paletteprovider
    paletteProvider: new MountainPaletteProvider(threshold)
});

sciChartSurface.renderableSeries.add(mountainSeries);
```

``` prism-code

// Demonstrates how to create a chart with a custom PaletteProvider, using the builder API
const { chartBuilder, EBaseType, ESeriesType, EPaletteProviderType, EThemeProviderType } = SciChart;

// or, for npm, import { chartBuilder, ... } from "scichart"

// Register the custom ThresholdLinePaletteProvider with the chartBuilder
chartBuilder.registerType(
    EBaseType.PaletteProvider,
    "MountainPaletteProvider",
    options => new MountainPaletteProvider(options.threshold)
);

// Create some data
let yLast = 100.0;
const xValues = [];
const yValues = [];
for (let i = 0; i <= 100; i++) {
    const y = yLast + (Math.random() - 0.48);
    yLast = y;
    xValues.push(i);
    yValues.push(y);
}

// Now use the Builder-API to build the chart
const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    surface: { theme: { type: EThemeProviderType.Dark } },
    series: [
        {
            type: ESeriesType.MountainSeries,
            xyData: {
                xValues,
                yValues
            },
            options: {
                stroke: "#4682b4",
                strokeThickness: 3,
                zeroLineY: 0.0,
                fill: "rgba(176, 196, 222, 0.7)", // when a solid color is required, use fill
                fillLinearGradient: {
                    gradientStops: [
                        { color: "rgba(70,130,180,0.77)", offset: 0.0 },
                        { color: "rgba(70,130,180,0.0)", offset: 1 }
                    ],
                    startPoint: { x: 0, y: 0 },
                    endPoint: { x: 0, y: 1 }
                },
                // Now you can instantiate using parameters below
                paletteProvider: {
                    type: EPaletteProviderType.Custom,
                    customType: "MountainPaletteProvider",
                    options: {
                        threshold: 75
                    }
                }
                // Note: Assigning an instance is also valid, e.g.
                // paletteProvider: new ThresholdLinePaletteProvider("Green", yValue => yValue >= 4.0)
            }
        }
    ]
});
```

The code above results in aÂ <a href="https://www.scichart.com/demo/javascript-mountain-chart" rel="noopener noreferrer" target="_blank">JavaScript Mountain Chart</a> with the following output. XValues \> 200 are colored red, and XValues \< 200 are the default series colors.

![](out_scichartv4/2d-charts/chart-types/palette-provider-api/fast-mountain-renderable-series/index_media/5a72c3ee6a9cc14296f901057d6eabf2b9b712b4.svg)note

SciChart won't bisect the line at a threshold value but only changes colour between line segments in the data you already have. If you want to have a perfect transistion from one colour to another at a specific Y-value, you will need to include data-points

## Colouring Mountain SeriesÂ Point-Markers with PaletteProvider<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/fast-mountain-renderable-series/#colouring-mountain-seriespoint-markers-with-paletteprovider" class="hash-link" aria-label="Direct link to Colouring Mountain SeriesÂ Point-Markers with PaletteProvider" translate="no" title="Direct link to Colouring Mountain SeriesÂ Point-Markers with PaletteProvider">â€‹</a>

If applying PointMarkers to the FastMountainRenderableSeries, and you want to adjust per-point coloring of the markers, then you need to implement overridePointMarkerArgb in your paletteprovider.

Find out how in the documentation pageÂ [Per-Point Colouring of Scatter Charts (or PointMarkers)](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/xy-scatter-renderable-series).

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/fast-mountain-renderable-series/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [PaletteProvider API](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/palette-provider-api-overview)
- [Per-point Colouring of Line Segments](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/fast-line-renderable-series)
- [Per-Point Colouring of Band Segments](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/fast-band-renderable-series)
- [Per-Point Colouring of Bubble Charts](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/fast-bubble-renderable-series)
- [Per-Point Colouring of Candlestick / OHLC Charts](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/fast-candlestick-ohlc-renderable-series)
- [Per-Point Colouring of Scatter Charts (or PointMarkers)](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/xy-scatter-renderable-series)
- [Per-Point Colouring of Column Charts](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/fast-column-renderable-series)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/palette-provider-api/fast-mountain-renderable-series/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/palette-provider-api/fast-mountain-renderable-series/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
