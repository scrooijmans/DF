On this page

# PolarPanModifier

The <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarpanmodifier.html" rel="noopener noreferrer" target="_blank">PolarPanModifierðŸ“˜</a> is a modifier that allows users to pan (drag) the polar chart. This modifier is useful for enhancing the user experience by providing an intuitive way to navigate through the chart data.

![](out_scichartv4/2d-charts/chart-modifier-api/polar-modifiers/polar-pan-modifier/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

You can set the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarpanmodifier.html#primarypanmode" rel="noopener noreferrer" target="_blank">primaryPanModeðŸ“˜</a> to `EPolarPanModifierPanMode.Cartesian` to pan the chart in Cartesian coordinates, or keep it to either `EPolarPanModifierPanMode.PolarStartAngle` or `EPolarPanModifierPanMode.PolarVisibleRange` to pan the chart in polar coordinates.

- TS
- Builder API (JSON Config)

``` prism-code
const { PolarPanModifier, EPolarPanModifierPanMode } = SciChart;
// or for npm: import { PolarPanModifier } from "scichart";

// Add PolarPanModifier behaviour to the chart
sciChartSurface.chartModifiers.add(
    new PolarPanModifier({
        primaryPanMode: EPolarPanModifierPanMode.PolarVisibleRange,

        secondaryPanMode: EPolarPanModifierPanMode.Cartesian,

        secondaryExecuteCondition: {
            key: SciChart.EModifierMouseArgKey.Ctrl
        }
    }),
);
```

``` prism-code
// Demonstrates how to configure the PolarPanModifier in SciChart.js using the Builder API
const { 
    chartBuilder, 
    EThemeProviderType, 
    EAxisType, 
    EChart2DModifierType, 
    EActionType, 
    EPolarAxisMode 
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
        options: { polarAxisMode: EPolarAxisMode.Radial } 
    },
    modifiers: [
        {
            type: EChart2DModifierType.PolarPan,
            options: {
                growFactor: 0.002,
                zoomSize: false,
            }
        }
    ]
});
```

This results in the following behavior:

#### See Also:<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/polar-modifiers/polar-pan-modifier/#see-also" class="hash-link" aria-label="Direct link to See Also:" translate="no" title="Direct link to See Also:">â€‹</a>

- [What is the ChartModifier API?](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/chart-modifier-api-overview)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-modifier-api/polar-modifiers/polar-pan-modifier/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-modifier-api/polar-modifiers/polar-pan-modifier/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
