On this page

# Metadata and Tooltips

Metadata and Tooltips

Using the ChartModifier API you can add tooltips and cursors to a SciChartSurface. By Combining this with Metadata you can include additional properties or data other than x,y in your tooltip labels.

![](out_scichartv4/2d-charts/chart-types/point-metadata-api/tooltips/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

Background reading:Â 

- If you haven't already, read the articleÂ [DataSeries PointMetadata API](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/point-metadata-api/point-metadata-api-overview)Â which will show you how to setup a DataSeries with point metadata (javascript objects).
- Also take a look at theÂ [RolloverModifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/rollover-modifier) andÂ [CursorModifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/cursor-modifier/cursor-modifier-overview)Â docs to find out how to add tooltips to charts.

## Example: Metadata + RolloverModifier<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/point-metadata-api/tooltips/#example-metadata--rollovermodifier" class="hash-link" aria-label="Direct link to Example: Metadata + RolloverModifier" translate="no" title="Direct link to Example: Metadata + RolloverModifier">â€‹</a>

Maybe you want certain property from Metadata to appear in tooltips. If so, you can use some code like this:

- JS
- Builder API (JSON Config)

``` prism-code
// Demonstrates how to add PointMetadata to a DataSeries and consume it in SciChart.js
const {
    SciChartSurface,
    NumericAxis,
    FastLineRenderableSeries,
    XyDataSeries,
    SciChartJsNavyTheme,
    EllipsePointMarker,
    NumberRange,
    RolloverModifier
} = SciChart;

// or, for npm, import { SciChartSurface, ... } from "scichart"

const { wasmContext, sciChartSurface } = await SciChartSurface.create(divElementId, {
    theme: new SciChartJsNavyTheme()
});

const growBy = new NumberRange(0.1, 0.1);
sciChartSurface.xAxes.add(new NumericAxis(wasmContext, { growBy }));
sciChartSurface.yAxes.add(new NumericAxis(wasmContext, { growBy }));

// Create metadata with initial values. Metadata can be any JS object
const dataSeries = new XyDataSeries(wasmContext, {
    xValues: [1, 2, 3, 4, 5],
    yValues: [4.3, 5.3, 6, 6.3, 6.4],
    metadata: [
        { stringValue: "Here's", customValue: 7 },
        undefined, // nothing at this index
        { stringValue: "Some" },
        {}, // empty object at this index
        { stringValue: "Metadata", customValue: 99 }
    ]
});

// Add a line series with the metadata
sciChartSurface.renderableSeries.add(
    new FastLineRenderableSeries(wasmContext, {
        dataSeries,
        pointMarker: new EllipsePointMarker(wasmContext, {
            width: 11,
            height: 11,
            fill: "#364BA0",
            stroke: "#50C7E0",
            strokeThickness: 2
        })
    })
);

// Add a RolloverModifier configured to output X,Y,Metadata.stringValue and customValue
sciChartSurface.chartModifiers.add(
    new RolloverModifier({
        snapToDataPoint: true,
        tooltipDataTemplate: seriesInfo => [
            `X: ${seriesInfo.formattedXValue}`,
            `Y: ${seriesInfo.formattedYValue}`,
            `Metadata.stringValue: ${seriesInfo.pointMetadata?.stringValue ?? "null"}`,
            `Metadata.customValue: ${seriesInfo.pointMetadata?.customValue ?? "null"}`
        ]
    })
);
```

``` prism-code
// Demonstrates how to add PointMetadata to a DataSeries and consume it in SciChart.js with the BuilderAPI
const { chartBuilder, ESeriesType, EThemeProviderType, EChart2DModifierType, EPointMarkerType } = SciChart;

// or, for npm, import { chartBuilder, ... } from "scichart"

const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    surface: { theme: { type: EThemeProviderType.Dark } },
    series: [
        {
            type: ESeriesType.LineSeries,
            // Metadata is set in xyData property
            xyData: {
                xValues: [1, 2, 3, 4, 5],
                yValues: [4.3, 5.3, 6, 6.3, 6.4],
                metadata: [
                    { stringValue: "Here's", customValue: 7 },
                    undefined, // nothing at this index
                    { stringValue: "Some" },
                    {}, // empty object at this index
                    { stringValue: "Metadata", customValue: 99 }
                ]
            },
            options: {
                stroke: "#C52E60",
                pointMarker: {
                    type: EPointMarkerType.Ellipse,
                    options: {
                        width: 11,
                        height: 11,
                        fill: "White"
                    }
                }
            }
        }
    ],
    // Configure a Rollovermodifier to display metadata
    modifiers: [
        {
            type: EChart2DModifierType.Rollover,
            options: {
                snapToDataPoint: true,
                tooltipDataTemplate: seriesInfo => [
                    `X: ${seriesInfo.formattedXValue}`,
                    `Y: ${seriesInfo.formattedYValue}`,
                    `Metadata.stringValue: ${seriesInfo.pointMetadata?.stringValue ?? "null"}`,
                    `Metadata.customValue: ${seriesInfo.pointMetadata?.customValue ?? "null"}`
                ]
            }
        }
    ]
});
```

This results in the following output.

## Example: Metadata + CursorModifier<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/point-metadata-api/tooltips/#example-metadata--cursormodifier" class="hash-link" aria-label="Direct link to Example: Metadata + CursorModifier" translate="no" title="Direct link to Example: Metadata + CursorModifier">â€‹</a>

The CursorModifier is very similar, however a different template is used to convert an array of SeriesInfo into tooltip lines.

Modify the above code like this to make metadata work with the CursorModifier.

- JS
- Builder API (JSON Config)

``` prism-code
// Demonstrates how to add PointMetadata to a DataSeries and consume it in SciChart.js
const {
    SciChartSurface,
    NumericAxis,
    FastLineRenderableSeries,
    XyDataSeries,
    SciChartJsNavyTheme,
    EllipsePointMarker,
    NumberRange,
    CursorModifier
} = SciChart;

// or, for npm, import { SciChartSurface, ... } from "scichart"

const { wasmContext, sciChartSurface } = await SciChartSurface.create(divElementId, {
    theme: new SciChartJsNavyTheme()
});

const growBy = new NumberRange(0.1, 0.1);
sciChartSurface.xAxes.add(new NumericAxis(wasmContext, { growBy }));
sciChartSurface.yAxes.add(new NumericAxis(wasmContext, { growBy }));

// Create metadata with initial values. Metadata can be any JS object
const dataSeries = new XyDataSeries(wasmContext, {
    xValues: [1, 2, 3, 4, 5],
    yValues: [4.3, 5.3, 6, 6.3, 6.4],
    metadata: [
        { stringValue: "Here's", customValue: 7 },
        undefined, // nothing at this index
        { stringValue: "Some" },
        {}, // empty object at this index
        { stringValue: "Metadata", customValue: 99 }
    ]
});

// Add a line series with the metadata
sciChartSurface.renderableSeries.add(
    new FastLineRenderableSeries(wasmContext, {
        dataSeries,
        pointMarker: new EllipsePointMarker(wasmContext, {
            width: 11,
            height: 11,
            fill: "#364BA0",
            stroke: "#50C7E0",
            strokeThickness: 2
        })
    })
);

// Add a RolloverModifier configured to output X,Y,Metadata.stringValue and customValue
sciChartSurface.chartModifiers.add(
    new CursorModifier({
        // snapToDataPoint: true,
        showTooltip: true,
        hitTestRadius: 10,
        tooltipDataTemplate: (seriesInfos, tooltipTitle) => {
            const seriesInfo = seriesInfos?.find(s => s.isHit);
            if (seriesInfo) {
                // Each element in the array is a line in the tooltip
                // This can be used to show the data from a single data-point where seriesInfo.isHit = true
                // or you can return multiple lines for multiple series (1 seriesInfo = 1 series)
                return [
                    `X: ${seriesInfo.formattedXValue}`,
                    `Y: ${seriesInfo.formattedYValue}`,
                    `Metadata.stringValue: ${seriesInfo.pointMetadata?.stringValue ?? "null"}`,
                    `Metadata.customValue: ${seriesInfo.pointMetadata?.customValue ?? "null"}`
                ];
            }
            return [];
        }
    })
);
```

``` prism-code
// Demonstrates how to add PointMetadata to a DataSeries and consume it in SciChart.js with the BuilderAPI
const { chartBuilder, ESeriesType, EThemeProviderType, EChart2DModifierType, EPointMarkerType } = SciChart;

// or, for npm, import { chartBuilder, ... } from "scichart"

const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    surface: { theme: { type: EThemeProviderType.Dark } },
    series: [
        {
            type: ESeriesType.LineSeries,
            // Metadata is set in xyData property
            xyData: {
                xValues: [1, 2, 3, 4, 5],
                yValues: [4.3, 5.3, 6, 6.3, 6.4],
                metadata: [
                    { stringValue: "Here's", customValue: 7 },
                    undefined, // nothing at this index
                    { stringValue: "Some" },
                    {}, // empty object at this index
                    { stringValue: "Metadata", customValue: 99 }
                ]
            },
            options: {
                stroke: "#C52E60",
                pointMarker: {
                    type: EPointMarkerType.Ellipse,
                    options: {
                        width: 11,
                        height: 11,
                        fill: "White"
                    }
                }
            }
        }
    ],
    // Configure a Rollovermodifier to display metadata
    modifiers: [
        {
            type: EChart2DModifierType.Cursor,
            options: {
                // snapToDataPoint: true,
                showTooltip: true,
                hitTestRadius: 10,
                tooltipDataTemplate: (seriesInfos, tooltipTitle) => {
                    const seriesInfo = seriesInfos?.find(s => s.isHit);
                    if (seriesInfo) {
                        // Each element in the array is a line in the tooltip
                        // This can be used to show the data from a single data-point where seriesInfo.isHit = true
                        // or you can return multiple lines for multiple series (1 seriesInfo = 1 series)
                        return [
                            `X: ${seriesInfo.formattedXValue}`,
                            `Y: ${seriesInfo.formattedYValue}`,
                            `Metadata.stringValue: ${seriesInfo.pointMetadata?.stringValue ?? "null"}`,
                            `Metadata.customValue: ${seriesInfo.pointMetadata?.customValue ?? "null"}`
                        ];
                    }
                    return [];
                }
            }
        }
    ]
});
```

Here's the output from the above:

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/point-metadata-api/tooltips/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/point-metadata-api/tooltips/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
