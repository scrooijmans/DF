On this page

# PolarMouseWheelZoomModifier

The <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarmousewheelzoommodifier.html" rel="noopener noreferrer" target="_blank">PolarMouseWheelZoomModifierðŸ“˜</a> is special a modifier that allows users to interact with the polar chart in 2 major ways:

## 1. Zooming in and out, using `EActionType.Zoom` (default)<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/polar-modifiers/polar-mouse-wheel-zoom-modifier/#1-zooming-in-and-out-using-eactiontypezoom-default" class="hash-link" aria-label="Direct link to 1-zooming-in-and-out-using-eactiontypezoom-default" translate="no" title="Direct link to 1-zooming-in-and-out-using-eactiontypezoom-default">â€‹</a>

- TS
- Builder API (JSON Config)

``` prism-code
const { PolarMouseWheelZoomModifier, EActionType } = SciChart;
// or for npm: import { PolarMouseWheelZoomModifier, EActionType } from "scichart";

// Add PolarMouseWheelZoomModifier behaviour to the chart
sciChartSurface.chartModifiers.add(
    new PolarMouseWheelZoomModifier({
        growFactor: 0.002,
        zoomSize: false,
        defaultActionType: EActionType.Zoom // DEFAULT value -> for zooming / scaling the polar chart
        // defaultActionType: EActionType.Pan // secondary value -> pans / spins the polar chart
    }),
);
```

``` prism-code
// Demonstrates how to configure the PolarMouseWheelZoomModifier in SciChart.js using the Builder API
const { 
    chartBuilder, 
    EThemeProviderType, 
    EAxisType, 
    EChart2DModifierType, 
    EPolarAxisMode, 
    ESeriesType, 
    EActionType 
} = SciChart;
// or, for npm, import { chartBuilder, ... } from "scichart"

const { wasmContext, sciChartSurface } = await chartBuilder.build2DPolarChart(divElementId, {
    surface: { theme: { type: EThemeProviderType.Dark } },
    xAxes: { 
        type: EAxisType.PolarNumericAxis, 
        options: { 
            polarAxisMode: EPolarAxisMode.Angular 
        } 
    },
    yAxes: { 
        type: EAxisType.PolarNumericAxis, 
        options: { 
            polarAxisMode: EPolarAxisMode.Radial 
        } 
    },
    series: {
        type: ESeriesType.PolarBandSeries,
        options: {
            strokeThickness: 3,
        },
        xyyData: {
            xValues: Array.from({ length: 12 }, (_, i) => i),
            yValues: Array.from({ length: 12 }, (_, i) => Math.sin(i * 0.2)),
            y1Values: Array.from({ length: 12 }, (_, i) => Math.cos(i * 0.2))
        }
    },
    modifiers: [
        {
            type: EChart2DModifierType.PolarMouseWheelZoom,
            options: {
                growFactor: 0.002,
                zoomSize: false,
                defaultActionType: EActionType.Zoom
            }
        }
    ]
});
```

This will result in the following behavior:

## 2. Panning / Spinning the chart, with `EActionType.Pan`<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/polar-modifiers/polar-mouse-wheel-zoom-modifier/#2-panning--spinning-the-chart-with-eactiontypepan" class="hash-link" aria-label="Direct link to 2-panning--spinning-the-chart-with-eactiontypepan" translate="no" title="Direct link to 2-panning--spinning-the-chart-with-eactiontypepan">â€‹</a>

- TS

``` prism-code
const { PolarMouseWheelZoomModifier, EActionType } = SciChart;
// or for npm: import { PolarMouseWheelZoomModifier, EActionType } from "scichart";

// Add PolarMouseWheelZoomModifier behaviour to the chart
sciChartSurface.chartModifiers.add(
    new PolarMouseWheelZoomModifier({
        growFactor: 0.002,
        zoomSize: false,
        defaultActionType: EActionType.Pan
    }),
);
```

Which will result in this:

- use whichever configuration you prefer, depending on the desired interaction with the polar chart.

#### See Also:<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/polar-modifiers/polar-mouse-wheel-zoom-modifier/#see-also" class="hash-link" aria-label="Direct link to See Also:" translate="no" title="Direct link to See Also:">â€‹</a>

- [What is the ChartModifier API?](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/chart-modifier-api-overview)
- [MouseWheelZoomModifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/zooming-and-panning/mouse-wheel-zoom-modifier)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-modifier-api/polar-modifiers/polar-mouse-wheel-zoom-modifier/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-modifier-api/polar-modifiers/polar-mouse-wheel-zoom-modifier/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
