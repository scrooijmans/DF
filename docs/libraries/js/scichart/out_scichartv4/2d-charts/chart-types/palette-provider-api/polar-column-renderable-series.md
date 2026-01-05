On this page

# Per-Point Coloring for Polar Column Series

Polar Column series can be colored per-point or per line-segment using theÂ [PaletteProvider API](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/palette-provider-api-overview). To use this, we must create a class (TS) or object (JS)Â which implements or confirms to theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/istrokepaletteprovider.html" rel="noopener noreferrer" target="_blank">IStrokePaletteProviderðŸ“˜</a> andÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ifillpaletteprovider.html" rel="noopener noreferrer" target="_blank">IFillPaletteProviderðŸ“˜</a> interfaces. Then, apply this to theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarcolumnrenderableseries.html#paletteprovider" rel="noopener noreferrer" target="_blank">PolarColumnRenderableSeries.paletteProviderðŸ“˜</a> property. This allows you to colour data-points based on values, or custom rules with infinite extensiblity.

First, let's create a <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/istrokepaletteprovider.html" rel="noopener noreferrer" target="_blank">PaletteProviderðŸ“˜</a> class like this:

- Creating the PaletteProvider

``` prism-code
const {
    DefaultPaletteProvider,
    EStrokePaletteMode,
    parseColorToUIntArgb
} = SciChart;
// or, for npm, import { DefaultPaletteProvider, ... } from "scichart"

// Custom PaletteProvider for column series which colours datapoints above a threshold
class ColumnPaletteProvider extends DefaultPaletteProvider {
    public strokePaletteMode = EStrokePaletteMode.SOLID;
    public threshold: number;
    public stroke: number;
    public fillColor: number;

    constructor(threshold: number) {
        super();
        this.strokePaletteMode = EStrokePaletteMode.SOLID;
        this.threshold = threshold;
        this.stroke = parseColorToUIntArgb("#FF0000");
        this.fillColor = parseColorToUIntArgb("#FF000077");
    }

    // This function is called for every data-point.
    // Return undefined to use the default color for the line,
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

Next, we can apply the PaletteProvider to the series. This can be done both with the programmatic API and the Builder API:

- TS
- Builder API (JSON Config)

``` prism-code
// Create and add a column series
const columnSeries = new PolarColumnRenderableSeries(wasmContext, {
    fill: "rgba(176, 196, 222, 0.5)",
    stroke: "rgba(176, 196, 222, 1)",
    strokeThickness: 2,
    dataPointWidth: 0.7,
    dataSeries: new XyDataSeries(wasmContext, { 
        xValues: Array.from({ length: 20 }, (_, i) => i),
        yValues: Array.from({ length: 20 }).map((_) => Math.random() * 10 + 5)
    }),
    paletteProvider: new ColumnPaletteProvider(10)
});

sciChartSurface.renderableSeries.add(columnSeries);
```

``` prism-code
// Demonstrates how to create a polar column chart with a custom PaletteProvider, using the builder API
const { 
    chartBuilder, 
    EBaseType, 
    ESeriesType, 
    EPaletteProviderType, 
    EThemeProviderType,
    EAxisType,
    EPolarAxisMode,
    EAxisAlignment,
    EPolarLabelMode,
    NumberRange,
} = SciChart;
// or, for npm, import { chartBuilder, ... } from "scichart"

// Register the custom ColumnPaletteProvider with the chartBuilder
chartBuilder.registerType(
    EBaseType.PaletteProvider,
    "ColumnPaletteProvider",
    options => new ColumnPaletteProvider(options.threshold)
);

// Now use the Builder-API to build the chart
const { wasmContext, sciChartSurface } = await chartBuilder.build2DPolarChart(divElementId, {
    surface: { theme: { type: EThemeProviderType.Dark } },
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
            }
        }
    ],
    series: [
        {
            type: ESeriesType.PolarColumnSeries,
            xyData: {
                xValues: Array.from({ length: 20 }, (_, i) => i),
                yValues: Array.from({ length: 20 }).map((_) => Math.random() * 10 + 5),
            },
            options: {
                fill: "rgba(176, 196, 222, 0.5)",
                stroke: "rgba(176, 196, 222, 1)",
                strokeThickness: 2,
                dataPointWidth: 0.7,
                // Now you can instantiate using parameters below
                paletteProvider: {
                    type: EPaletteProviderType.Custom,
                    customType: "ColumnPaletteProvider",
                    options: {
                        threshold: 10
                    }
                }
                // Note: Assigning an instance is also valid, e.g.
                // paletteProvider: new ColumnPaletteProvider(10)
            }
        }
    ]
});
```

The code above results in a [Polar Column Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-column-renderable-series) with the following rule: **change color if point is above yValue 10**. The result is shown below:

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/polar-column-renderable-series/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [Column Series PaletteProvider API](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/fast-column-renderable-series) - check out the 2D version of this article for more info

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/palette-provider-api/polar-column-renderable-series/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/palette-provider-api/polar-column-renderable-series/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
