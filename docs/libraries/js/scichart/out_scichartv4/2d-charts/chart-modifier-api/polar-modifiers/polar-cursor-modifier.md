On this page

# PolarCursorModifier

The <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarcursormodifier.html" rel="noopener noreferrer" target="_blank">PolarCursorModifierðŸ“˜</a> is a modifier that provides a crosshair cursor on a polar chart. It allows users to hover over data points and see their values, enhancing the interactivity of the chart.

## Adding a PolarCursorModifier to a Chart<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/polar-modifiers/polar-cursor-modifier/#adding-a-polarcursormodifier-to-a-chart" class="hash-link" aria-label="Direct link to Adding a PolarCursorModifier to a Chart" translate="no" title="Direct link to Adding a PolarCursorModifier to a Chart">â€‹</a>

Similarly to the [CursorModifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/cursor-modifier/cursor-modifier-overview) for a Cartesian 2D chart, a <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarcursormodifier.html" rel="noopener noreferrer" target="_blank">PolarCursorModifierðŸ“˜</a> can be added to the `sciChartSurface.chartModifiers` collection of a <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartpolarsurface.html" rel="noopener noreferrer" target="_blank">SciChartPolarSurfaceðŸ“˜</a> to enable crosshair behavior. For example:

- TS
- Builder API (JSON Config)

``` prism-code
const { PolarCursorModifier, EAngularAxisLabelPlacement, ERadialAxisLabelPlacement } = SciChart;
// or for npm: import { PolarCursorModifier, ... } from "scichart";

// const { sciChartSurface, wasmContext } = await SciChartPolarSurface.create(divElementId, {})
// ...

// Add PolarCursorModifier behaviour to the chart
sciChartSurface.chartModifiers.add(
    new PolarCursorModifier({
        lineColor: "#55aaff",
        lineThickness: 2,
        axisLabelFill: "#55aaff",
        angularAxisLabelPlacement: EAngularAxisLabelPlacement.Center,
        radialAxisLabelPlacement: ERadialAxisLabelPlacement.Center,
        showRadialLine: true,
        showCircularLine: true
    })
);
```

``` prism-code
// Demonstrates how to configure the PolarCursorModifier in SciChart.js using the Builder API
const {
    chartBuilder,
    EAxisType,
    EChart2DModifierType,
    EPolarAxisMode,
    EAngularAxisLabelPlacement,
    ERadialAxisLabelPlacement,
    ESeriesType
} = SciChart;
// or, for npm, import { chartBuilder, ... } from "scichart"

const { wasmContext, sciChartSurface } = await chartBuilder.build2DPolarChart(divElementId, {
    xAxes: { 
        type: EAxisType.PolarNumericAxis, 
        options: { polarAxisMode: EPolarAxisMode.Angular } 
    },
    yAxes: { 
        type: EAxisType.PolarNumericAxis, 
        options: { polarAxisMode: EPolarAxisMode.Radial } 
    },
    series: [
        {
            type: ESeriesType.PolarLineSeries,
            options: {
                stroke: "#50C7E0",
                strokeThickness: 5,
            },
            xyData: {
                xValues: Array.from({ length: 10 }, (_, i) => i),
                yValues: Array.from({ length: 10 }, (_, i) => Math.sin(i * 0.1))
            }
        }
    ],
    modifiers: [
        {
            type: EChart2DModifierType.PolarCursor,
            options: {
                lineColor: "#55aaff",
                lineThickness: 3,
                axisLabelFill: "#55aaff",
                angularAxisLabelPlacement: EAngularAxisLabelPlacement.Center,
                radialAxisLabelPlacement: ERadialAxisLabelPlacement.Top,
                showRadialLine: true,
                showCircularLine: true
            }
        }
    ]
});
```

This results in the following behavior:

#### See Also:<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/polar-modifiers/polar-cursor-modifier/#see-also" class="hash-link" aria-label="Direct link to See Also:" translate="no" title="Direct link to See Also:">â€‹</a>

- [What is the ChartModifier API?](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/chart-modifier-api-overview)
- [CursorModifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/cursor-modifier/cursor-modifier-overview)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-modifier-api/polar-modifiers/polar-cursor-modifier/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-modifier-api/polar-modifiers/polar-cursor-modifier/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
