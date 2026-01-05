On this page

# Per-Point Colouring of Column Charts

Column series can be colored per-point or per line-segment using theÂ [PaletteProvider API](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/palette-provider-api-overview). To use this, we must create a class (typescript) or object (javascript)Â which implements or confirms to theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/istrokepaletteprovider.html" rel="noopener noreferrer" target="_blank">IStrokePaletteProviderðŸ“˜</a> andÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ifillpaletteprovider.html" rel="noopener noreferrer" target="_blank">IFillPaletteProviderðŸ“˜</a> interfaces. Then, apply this to theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastcolumnrenderableseries.html#paletteprovider" rel="noopener noreferrer" target="_blank">FastColumnRenderableSeries.paletteProviderðŸ“˜</a> property. This allows you to colour data-points based on values, or custom rules with infinite extensiblity.

First, let's create a PaletteProvider class like this:

- TS

``` prism-code
const { DefaultPaletteProvider, EStrokePaletteMode, parseColorToUIntArgb } = SciChart;

// or, for npm, import { DefaultPaletteProvider, ... } from "scichart"

// Custom PaletteProvider for column series which colours datapoints above a threshold
class ColumnPaletteProvider extends DefaultPaletteProvider {
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
        return yValue > this.threshold ? this.fillColor : undefined;
    }

    // This function is called for every data-point
    // Return undefined to use the default color for the fill, else, return
    // a custom color as ARGB color code e.g. 0xFFFF0000 is red
    overrideFillArgb(xValue, yValue, index, opacity, metadata) {
        return yValue > this.threshold ? this.fillColor : undefined;
    }
}
```

Next, we can apply the PaletteProvider to the series. This can be done both with the programmatic API and the Builder API:

- TS
- Builder API (JSON Config)

``` prism-code
// Create some data
const xValues = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19];
const yValues = [
    0.1, 0.2, 0.4, 0.8, 1.1, 1.5, 2.4, 4.6, 8.1, 11.7, 14.4, 16.0, 13.7, 10.1, 6.4, 3.5, 2.5, 1.4, 0.4, 0.1
];

// Create and add a column series
const columnSeries = new FastColumnRenderableSeries(wasmContext, {
    fill: "rgba(176, 196, 222, 0.5)",
    stroke: "rgba(176, 196, 222, 1)",
    strokeThickness: 2,
    dataPointWidth: 0.7,
    dataSeries: new XyDataSeries(wasmContext, { xValues, yValues }),
    paletteProvider: new ColumnPaletteProvider(10)
});

sciChartSurface.renderableSeries.add(columnSeries);
```

``` prism-code

// Demonstrates how to create a chart with a custom PaletteProvider, using the builder API
const { chartBuilder, EBaseType, ESeriesType, EPaletteProviderType, EThemeProviderType } = SciChart;

// or, for npm, import { chartBuilder, ... } from "scichart"

// Register the custom ColumnPaletteProvider with the chartBuilder
chartBuilder.registerType(
    EBaseType.PaletteProvider,
    "ColumnPaletteProvider",
    options => new ColumnPaletteProvider(options.threshold)
);

// Create some data
const xValues = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19];
const yValues = [
    0.1, 0.2, 0.4, 0.8, 1.1, 1.5, 2.4, 4.6, 8.1, 11.7, 14.4, 16.0, 13.7, 10.1, 6.4, 3.5, 2.5, 1.4, 0.4, 0.1
];

// Now use the Builder-API to build the chart
const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    surface: { theme: { type: EThemeProviderType.Dark } },
    series: [
        {
            type: ESeriesType.ColumnSeries,
            xyData: {
                xValues,
                yValues
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

The code above results in aÂ <a href="https://www.scichart.com/demo/javascript-column-chart" rel="noopener noreferrer" target="_blank">JavaScript Column Chart</a> with the following output.Â YValues \> 10 are colored red, and YValues \< 10 are the default series stroke and fill colors.

## Troubleshooting<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/fast-column-renderable-series/#troubleshooting" class="hash-link" aria-label="Direct link to Troubleshooting" translate="no" title="Direct link to Troubleshooting">â€‹</a>

For <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastcolumnrenderableseries.html" rel="noopener noreferrer" target="_blank">FastColumnRenderableSeriesðŸ“˜</a> you may notice that **palette provider stops working when zoomed out to the point that columns are 1px wide**. It may happens when a palette provider class overrides only fill and do not override stroke. When we zoom out far enough to reach one pixel width the drawing engine does not use fill any more it uses stroke. Therefore in order to make it look always the same color you would need to override both fill and stroke. For example:

``` prism-code
import {
  EFillPaletteMode,
  IFillPaletteProvider,
  IStrokePaletteProvider,
  IPointMetadata,
  IRenderableSeries,
  EStrokePaletteMode,
} from 'scichart';

export class BarPaletteProvider
  implements IFillPaletteProvider, IStrokePaletteProvider
{
  fillPaletteMode = EFillPaletteMode.SOLID;
  strokePaletteMode = EStrokePaletteMode.SOLID;
  private color: number;

  constructor(color: string) {
    this.color = BarPaletteProvider.argbColorToNumber(color);
  }

  onAttached(parentSeries: IRenderableSeries): void {}

  onDetached(): void {}

  overrideFillArgb(
    xValue: number,
    yValue: number,
    index: number,
    opacity?: number,
    metadata?: IPointMetadata
  ): number {
    return this.color;
  }

  overrideStrokeArgb(
    xValue: number,
    yValue: number,
    index: number,
    opacity?: number,
    metadata?: IPointMetadata
  ): number | undefined {
    return this.color;
  }

  private static argbColorToNumber(color: string) {
    return parseInt(color.substring(1), 16);
  }
}
```

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/palette-provider-api/fast-column-renderable-series/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/palette-provider-api/fast-column-renderable-series/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
