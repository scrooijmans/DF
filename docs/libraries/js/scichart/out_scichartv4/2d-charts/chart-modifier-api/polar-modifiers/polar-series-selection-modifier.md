On this page

# PolarSeriesSelectionModifier

The <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarseriesselectionmodifier.html" rel="noopener noreferrer" target="_blank">PolarSeriesSelectionModifierðŸ“˜</a> is a modifier that allows users to select a series on a polar chart. It provides visual feedback when a series is selected or hovered, enhancing the user experience by allowing for interaction with the chart.

## Adding a PolarSeriesSelectionModifier to a Chart<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/polar-modifiers/polar-series-selection-modifier/#adding-a-polarseriesselectionmodifier-to-a-chart" class="hash-link" aria-label="Direct link to Adding a PolarSeriesSelectionModifier to a Chart" translate="no" title="Direct link to Adding a PolarSeriesSelectionModifier to a Chart">â€‹</a>

- TS
- Builder API (JSON Config)

``` prism-code
const { 
    PolarSeriesSelectionModifier, 
    PolarLineRenderableSeries,
    XyDataSeries,
} = SciChart;
// or for npm: import { PolarSeriesSelectionModifier } from "scichart";

// const { sciChartSurface, wasmContext } = await SciChartPolarSurface.create(divElementId, {})
// ...

// Add 2 series
sciChartSurface.renderableSeries.add(
    new PolarLineRenderableSeries(wasmContext, {
        dataSeries: new XyDataSeries(wasmContext, {
            xValues: Array.from({ length: 10 }, (_, i) => i),
            yValues: Array.from({ length: 10 }, (_, i) => Math.sin(i * 0.1))
        }),
        stroke: "#FFAA00",
        strokeThickness: 3,
        onSelectedChanged: (sourceSeries) => {
            sourceSeries.stroke = sourceSeries.isSelected ? "white" : "#FFAA00";
        },
        onHoveredChanged: (sourceSeries) => {
            sourceSeries.stroke = sourceSeries.isHovered ? "gray" : "#FFAA00";
        },
    }),
    new PolarLineRenderableSeries(wasmContext, {
        dataSeries: new XyDataSeries(wasmContext, {
            xValues: Array.from({ length: 10 }, (_, i) => i),
            yValues: Array.from({ length: 10 }, (_, i) => Math.cos(i * 0.1))
        }),
        stroke: "#3388FF",
        strokeThickness: 3,
        onSelectedChanged: (sourceSeries) => {
            sourceSeries.stroke = sourceSeries.isSelected ? "white" : "#3388FF";
        },
        onHoveredChanged: (sourceSeries) => {
            sourceSeries.stroke = sourceSeries.isHovered ? "gray" : "#3388FF";
        },
    })
);

// Add PolarSeriesSelectionModifier behaviour to the chart
sciChartSurface.chartModifiers.add(
    new PolarSeriesSelectionModifier(),
);
```

``` prism-code
// Demonstrates how to configure the PolarSeriesSelection in SciChart.js using the Builder API
const { 
    chartBuilder, 
    EThemeProviderType, 
    EAxisType, 
    EChart2DModifierType, 
    ESeriesType, 
    EPolarAxisMode,
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
                onSelectedChanged: (sourceSeries) => {
                    sourceSeries.stroke = sourceSeries.isSelected ? "white" : "#FFAA00";
                },
                onHoveredChanged: (sourceSeries) => {
                    sourceSeries.stroke = sourceSeries.isHovered ? "gray" : "#FFAA00";
                }
            }
        },
        {
            type: ESeriesType.PolarLineSeries,
            xyData: {
                xValues: Array.from({ length: 10 }, (_, i) => i),
                yValues: Array.from({ length: 10 }, (_, i) => Math.cos(i * 0.1))
            },
            options: {
                stroke: "#3388FF",
                strokeThickness: 3,
                onSelectedChanged: (sourceSeries) => {
                    sourceSeries.stroke = sourceSeries.isSelected ? "white" : "#3388FF";
                },
                onHoveredChanged: (sourceSeries) => {
                    sourceSeries.stroke = sourceSeries.isHovered ? "gray" : "#3388FF";
                }
            }
        },
    ],
    modifiers: [
        {
            type: EChart2DModifierType.PolarSeriesSelection
        }
    ]
});
```

# TODO - UNCOMMENT snippet

#### See Also:<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/polar-modifiers/polar-series-selection-modifier/#see-also" class="hash-link" aria-label="Direct link to See Also:" translate="no" title="Direct link to See Also:">â€‹</a>

- [What is the ChartModifier API?](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/chart-modifier-api-overview)
- [PolarDataPointSelectionModifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/polar-modifiers/polar-data-point-selection-modifier)
- [SeriesSelectionModifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/selection/series-selection/)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-modifier-api/polar-modifiers/polar-series-selection-modifier/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-modifier-api/polar-modifiers/polar-series-selection-modifier/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
