On this page

# PolarDataPointSelectionModifier

The <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polardatapointselectionmodifier.html" rel="noopener noreferrer" target="_blank">PolarDataPointSelectionModifierðŸ“˜</a> is a modifier that allows users to select data points on a polar chart. It provides visual feedback when a data point is selected, enhancing the user experience by allowing for interaction with the chart.

## Adding a PolarDataPointSelectionModifier to a Chart<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/polar-modifiers/polar-data-point-selection-modifier/#adding-a-polardatapointselectionmodifier-to-a-chart" class="hash-link" aria-label="Direct link to Adding a PolarDataPointSelectionModifier to a Chart" translate="no" title="Direct link to Adding a PolarDataPointSelectionModifier to a Chart">â€‹</a>

- TS
- Builder API (JSON Config)

``` prism-code
const { 
    PolarDataPointSelectionModifier, 
    DataPointSelectionPaletteProvider,
    PolarLineRenderableSeries,
    XyDataSeries,
    EllipsePointMarker,
} = SciChart;
// or for npm: import { PolarDataPointSelectionModifier } from "scichart";

// const { sciChartSurface, wasmContext } = await SciChartPolarSurface.create(divElementId, {})
// ...

// Add a series with point markers & a DataPointSelectionPaletteProvider to see the selection effect
sciChartSurface.renderableSeries.add(
    new PolarLineRenderableSeries(wasmContext, {
        dataSeries: new XyDataSeries(wasmContext, {
            xValues: Array.from({ length: 10 }, (_, i) => i),
            yValues: Array.from({ length: 10 }, (_, i) => Math.sin(i * 0.1))
        }),
        stroke: "#FFAA00",
        strokeThickness: 3,
        pointMarker: new EllipsePointMarker(wasmContext, {
            width: 12,
            height: 12,
            fill: "#000000",
            stroke: "#FFAA00",
            strokeThickness: 2,
        }),
        paletteProvider: new DataPointSelectionPaletteProvider({
            fill: "#FFFFFF",
            stroke: "#FFAA00", // keep the same
        })
    })
);

// Add PolarDataPointSelectionModifier behaviour to the chart
sciChartSurface.chartModifiers.add(
    new PolarDataPointSelectionModifier({
        allowDragSelect: true,
        allowClickSelect: true,
        selectionStroke: "#3388FF",
        selectionFill: "#3388FF44",
        onSelectionChanged: (args) => {
            console.log("seriesSelectionModifier onSelectionChanged", args);
        },
    }),
);
```

``` prism-code
// Demonstrates how to configure the PolarDataPointSelection in SciChart.js using the Builder API
const { 
    chartBuilder, 
    EThemeProviderType, 
    EAxisType, 
    EChart2DModifierType, 
    ESeriesType, 
    EPolarAxisMode,
    DataPointSelectionPaletteProvider
} = SciChart;
// or, for npm, import { chartBuilder, ... } from "scichart"

const { wasmContext, sciChartSurface } = await chartBuilder.build2DPolarChart(divElementId, {
    surface: { theme: { type: EThemeProviderType.Dark } },
    xAxes: { 
        type: EAxisType.PolarNumericAxis, 
        options: { polarAxisMode: EPolarAxisMode.Angular } 
    },
    yAxes: { 
        type: EAxisType.PolarNumericAxis, 
        options: { polarAxisMode: EPolarAxisMode.Angular } 
    },
    series: [
        {
            type: ESeriesType.PolarLineSeries,
            xyData: {
                xValues: Array.from({ length: 10 }, (_, i) => i),
                yValues: Array.from({ length: 10 }, (_, i) => Math.sin(i * 0.1))
            },
            options: {
                stroke: "#FFAA00",
                strokeThickness: 3,
                pointMarker: {
                    type: SciChart.EPointMarkerType.Triangle,
                    options: {
                        width: 12,
                        height: 12,
                        fill: "#000000",
                        stroke: "#FFAA00",
                        strokeThickness: 2
                    }
                },
                paletteProvider: new DataPointSelectionPaletteProvider({
                    fill: "#FFFFFF",
                    stroke: "#FFAA00", // keep the same
                })
            }
        }
    ],
    modifiers: [
        {
            type: EChart2DModifierType.PolarDataPointSelection
        }
    ]
});
```

#### See Also:<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/polar-modifiers/polar-data-point-selection-modifier/#see-also" class="hash-link" aria-label="Direct link to See Also:" translate="no" title="Direct link to See Also:">â€‹</a>

- [What is the ChartModifier API?](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/chart-modifier-api-overview)
- [PolarSeriesSelectionModifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/polar-modifiers/polar-series-selection-modifier)
- [DataPointSelectionModifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/selection/data-point-selection/)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-modifier-api/polar-modifiers/polar-data-point-selection-modifier/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-modifier-api/polar-modifiers/polar-data-point-selection-modifier/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
