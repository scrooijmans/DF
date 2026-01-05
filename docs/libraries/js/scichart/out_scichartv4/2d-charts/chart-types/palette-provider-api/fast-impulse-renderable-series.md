# Per-Point Colouring of Impulse Charts

Impulse series can be colored per-point or per data-point using theÂ [PaletteProvider API](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/palette-provider-api-overview). To use this, we must create a class (typescript) or object (javascript)Â which implements or confirms to theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/istrokepaletteprovider.html" rel="noopener noreferrer" target="_blank">IStrokePaletteProviderðŸ“˜</a> andÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ifillpaletteprovider.html" rel="noopener noreferrer" target="_blank">IFillPaletteProviderðŸ“˜</a> interfaces. Then, apply this to theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastimpulserenderableseries.html#paletteprovider" rel="noopener noreferrer" target="_blank">FastImpulseRenderableSeries.paletteProviderðŸ“˜</a> property. This allows you to colour data-points based on values, or custom rules with infinite extensiblity.

First, let's create a PaletteProvider class like this:

- TS

``` prism-code
const { DefaultPaletteProvider, EStrokePaletteMode, parseColorToUIntArgb } = SciChart;

// or, for npm, import { DefaultPaletteProvider, ... } from "scichart"

// Custom PaletteProvider for impulse series which colours data-points above a threshold
class LineAndPointMarkerPaletteProvider extends DefaultPaletteProvider {
    rule: any;
    stroke: number;
    constructor(stroke, rule) {
        super();
        this.strokePaletteMode = EStrokePaletteMode.SOLID;
        this.rule = rule;
        this.stroke = parseColorToUIntArgb(stroke);
    }

    overrideStrokeArgb(xValue, yValue, index, opacity, metadata) {
        // Conditional logic for coloring here. Returning 'undefined' means 'use default renderableSeries.stroke'
        // else, we can return a color of choice.
        //
        // Note that colors returned are Argb format as number. There are helper functions which can convert from Html
        // color codes to Argb format.
        //
        // Performance considerations: overrideStrokeArgb is called per-point on the series when drawing.
        // Caching color values and doing minimal logic in this function will help performance
        return this.rule(yValue) ? this.stroke : undefined;
    }

    overridePointMarkerArgb(xValue, yValue, index, opacity, metadata) {
        if (this.rule(yValue)) {
            // Override pointmarker color here
            return {
                stroke: this.stroke,
                fill: this.stroke
            };
        }
        // Default color here
        return undefined;
    }
}
```

Next, we can apply the PaletteProvider to the series. This can be done both with the programmatic API and the Builder API:

- TS
- Builder API (JSON Config)

``` prism-code
// Demonstrates how to create an Impulse (or Stem, Lollipop) chart with SciChart.js
const {
    SciChartSurface,
    NumericAxis,
    FastImpulseRenderableSeries,
    XyDataSeries,
    EllipsePointMarker,
    SciChartJsNavyTheme,
    NumberRange
} = SciChart;

// or, for npm, import { SciChartSurface, ... } from "scichart"

const { wasmContext, sciChartSurface } = await SciChartSurface.create(divElementId, {
    theme: new SciChartJsNavyTheme()
});
sciChartSurface.xAxes.add(new NumericAxis(wasmContext));
sciChartSurface.yAxes.add(new NumericAxis(wasmContext, { growBy: new NumberRange(0, 0.1) }));

// Create some data
const xValues = [];
const yValues = [];
for (let i = 0; i < 100; i++) {
    xValues.push(i);
    yValues.push(Math.sin(i * 0.2) * Math.log(i / 100));
}

// Create and add a column series
const impulseSeries = new FastImpulseRenderableSeries(wasmContext, {
    fill: "#50C7E0",
    strokeThickness: 2,
    size: 10,
    dataSeries: new XyDataSeries(wasmContext, { xValues, yValues }),
    // Apply the PaletteProvider to the impulse series
    paletteProvider: new LineAndPointMarkerPaletteProvider("#F48420", y => y < 0.0)
});

sciChartSurface.renderableSeries.add(impulseSeries);
```

``` prism-code
// Demonstrates how to create a line chart with SciChart.js using the Builder API
const { chartBuilder, ESeriesType, EThemeProviderType, EPointMarkerType, EPaletteProviderType, EBaseType } =
    SciChart;

// or, for npm, import { chartBuilder, ... } from "scichart"

// Create some data
const xValues = [];
const yValues = [];
for (let i = 0; i < 100; i++) {
    xValues.push(i);
    yValues.push(Math.sin(i * 0.2) * Math.log(i / 100));
}

// Register the custom LineAndPointMarkerPaletteProvider with the chartBuilder
chartBuilder.registerType(
    EBaseType.PaletteProvider,
    "LineAndPointMarkerPaletteProvider",
    options => new LineAndPointMarkerPaletteProvider(options.stroke, options.rule)
);

const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    surface: { theme: { type: EThemeProviderType.Dark } },
    series: [
        {
            type: ESeriesType.ImpulseSeries,
            xyData: {
                xValues,
                yValues
            },
            options: {
                fill: "#50C7E0",
                strokeThickness: 2,
                size: 10,
                pointMarker: { type: EPointMarkerType.Ellipse },
                // Now you can instantiate using parameters below
                paletteProvider: {
                    type: EPaletteProviderType.Custom,
                    customType: "LineAndPointMarkerPaletteProvider",
                    options: {
                        stroke: "#F48420",
                        rule: y => y < 0.0
                    }
                }
                // Note: Assigning an instance is also valid, e.g.
                // paletteProvider: new LineAndPointMarkerPaletteProvider()
            }
        }
    ]
});
```

The code above results in aÂ <a href="https://www.scichart.com/demo/javascript-column-chart" rel="noopener noreferrer" target="_blank">JavaScript Column Chart</a> with the following output.Â YValues \> 10 are colored red, and YValues \< 10 are the default series stroke and fill colors.

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/palette-provider-api/fast-impulse-renderable-series/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/palette-provider-api/fast-impulse-renderable-series/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
