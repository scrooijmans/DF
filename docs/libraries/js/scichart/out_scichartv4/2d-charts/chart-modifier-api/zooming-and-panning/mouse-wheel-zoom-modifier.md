On this page

# MouseWheelZoomModifier

SciChart.js provides an zooming / panningÂ behavior when scrolling the mouse-wheel, or two-finger drag on touch devices via theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/mousewheelzoommodifier.html" rel="noopener noreferrer" target="_blank">MouseWheelZoomModifierðŸ“˜</a>, available out of the box.

Besides common features which are inherited from theÂ [ChartModifierBase](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/common-features) class, theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/mousewheelzoommodifier.html" rel="noopener noreferrer" target="_blank">MouseWheelZoomModifierðŸ“˜</a>Â allows you to specify how fast the chart zooms in or out via theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/mousewheelzoommodifier.html#growfactor" rel="noopener noreferrer" target="_blank">growFactor propertyðŸ“˜</a>.

## Adding aÂ MouseWheelZoomModifier to a Chart<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/zooming-and-panning/mouse-wheel-zoom-modifier/#adding-amousewheelzoommodifier-to-a-chart" class="hash-link" aria-label="Direct link to Adding aÂ MouseWheelZoomModifier to a Chart" translate="no" title="Direct link to Adding aÂ MouseWheelZoomModifier to a Chart">â€‹</a>

AÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/mousewheelzoommodifier.html" rel="noopener noreferrer" target="_blank">MouseWheelZoomModifierðŸ“˜</a>Â can be added to theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#chartmodifiers" rel="noopener noreferrer" target="_blank">sciChartSurface.chartModifiersðŸ“˜</a> collection to enable scaling or panning behavior. For example:

- TS
- Builder API (JSON Config)

``` prism-code
const { MouseWheelZoomModifier } = SciChart;
// or for npm import { MouseWheelZoomModifier } from "scichart"

// Add MouseWheel Zoom behavior
sciChartSurface.chartModifiers.add(
    new MouseWheelZoomModifier({
        growFactor: 0.001, // each mousewheel click zooms 0.1%
    })
);
```

``` prism-code
// Demonstrates how to configure the MouseWheelZoomModifier in SciChart.js using the Builder API
const { chartBuilder, EThemeProviderType, EChart2DModifierType, EXyDirection } = SciChart;
// or, for npm, import { chartBuilder, ... } from "scichart"

const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    modifiers: [
        {
            type: EChart2DModifierType.MouseWheelZoom,
            options: {
                growFactor: 0.001 // each mousewheel click zooms 0.1%
            }
        }
    ]
});
```

This results in the following behavior:

<img src="out_scichartv4/2d-charts/chart-modifier-api/zooming-and-panning/mouse-wheel-zoom-modifier/index_media/cc207d4d0fe2d00f050ea7824388e9f3cc15fab1.gif" style="display:block;margin-left:auto;margin-right:auto;object-fit:contain;height:auto;width:85%;margin:0 auto" />

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-modifier-api/zooming-and-panning/mouse-wheel-zoom-modifier/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-modifier-api/zooming-and-panning/mouse-wheel-zoom-modifier/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
