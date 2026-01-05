On this page

# XAxisDragModifier

SciChart.js provides an zooming / panningÂ behavior when dragging the Axis via theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xaxisdragmodifier.html" rel="noopener noreferrer" target="_blank">XAxisDragModifierðŸ“˜</a>, available out of the box.

Besides common features which are inherited from theÂ [ChartModifierBase](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/common-features) class, theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xaxisdragmodifier.html" rel="noopener noreferrer" target="_blank">XAxisDragModifierðŸ“˜</a> allows you to chooseÂ panning or scaling via theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/yaxisdragmodifier.html#dragmode" rel="noopener noreferrer" target="_blank">dragModeðŸ“˜</a> property.

## Adding aÂ XAxisDragModifier to a Chart<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/zooming-and-panning/x-axis-drag-modifier/#adding-axaxisdragmodifier-to-a-chart" class="hash-link" aria-label="Direct link to Adding aÂ XAxisDragModifier to a Chart" translate="no" title="Direct link to Adding aÂ XAxisDragModifier to a Chart">â€‹</a>

AÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xaxisdragmodifier.html" rel="noopener noreferrer" target="_blank">XAxisDragModifierðŸ“˜</a> can be added to theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#chartmodifiers" rel="noopener noreferrer" target="_blank">sciChartSurface.chartModifiersðŸ“˜</a> collection to enable scaling or panning behavior. For example:

- TS
- Builder API (JSON Config)

``` prism-code
import { XAxisDragModifier, EDragMode } from "scichart";

// Add XAxis Drag behavior
sciChartSurface.chartModifiers.add(
    new XAxisDragModifier({
        dragMode: EDragMode.Scaling,
    })
);
```

``` prism-code
// Demonstrates how to configure the XAxisDrag Modifier in SciChart.js using the Builder API
const { chartBuilder, EChart2DModifierType, EDragMode } = SciChart;
// or, for npm, import { chartBuilder, ... } from "scichart"

const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    modifiers: [
        {
            type: EChart2DModifierType.XAxisDrag,
            options: {
                dragMode: EDragMode.Scaling,
            }
        }
    ]
});
```

This results in the following behavior:

<img src="out_scichartv4/2d-charts/chart-modifier-api/zooming-and-panning/x-axis-drag-modifier/index_media/35d8e763edf2ac56b6776a6b8065891426a8977f.gif" title="X and Y Axis Drag Modifier GIF" style="display:block;margin-left:auto;margin-right:auto;object-fit:contain;height:auto;width:85%;margin:0 auto" alt="X and Y Axis Drag Modifier GIF" />

X and Y Axis Drag Modifier GIF

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/zooming-and-panning/x-axis-drag-modifier/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [What is the ChartModifier API](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/chart-modifier-api-overview)
- [Common ChartModifiers Features](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/common-features)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-modifier-api/zooming-and-panning/x-axis-drag-modifier/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-modifier-api/zooming-and-panning/x-axis-drag-modifier/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
