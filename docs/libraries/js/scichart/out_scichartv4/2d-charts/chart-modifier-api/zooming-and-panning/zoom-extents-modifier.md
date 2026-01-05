On this page

# ZoomExtentsModifier

SciChart.js provides the ability to Zoom Extents the entire chart (zoom to fit data) by double-clicking the chart area withÂ theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/zoomextentsmodifier.html" rel="noopener noreferrer" target="_blank">ZoomExtentsModifierðŸ“˜</a>, available out of the box.

Besides common features which are inherited from theÂ [ChartModifierBase](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/common-features) class, theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/zoomextentsmodifier.html" rel="noopener noreferrer" target="_blank">ZoomExtentsModifierðŸ“˜</a>Â allows animated zooming via theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/zoomextentsmodifier.html#isanimated" rel="noopener noreferrer" target="_blank">isAnimatedðŸ“˜</a>,Â <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/zoomextentsmodifier.html#animationduration" rel="noopener noreferrer" target="_blank">animationDurationðŸ“˜</a> andÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/zoomextentsmodifier.html#easingfunction" rel="noopener noreferrer" target="_blank">easingFunctionðŸ“˜</a> properties.

## Adding aÂ ZoomExtentsModifierÂ to a Chart<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/zooming-and-panning/zoom-extents-modifier/#adding-azoomextentsmodifierto-a-chart" class="hash-link" aria-label="Direct link to Adding aÂ ZoomExtentsModifierÂ to a Chart" translate="no" title="Direct link to Adding aÂ ZoomExtentsModifierÂ to a Chart">â€‹</a>

AÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/zoomextentsmodifier.html" rel="noopener noreferrer" target="_blank">ZoomExtentsModifierðŸ“˜</a>Â can be added to theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#chartmodifiers" rel="noopener noreferrer" target="_blank">sciChartSurface.chartModifiersðŸ“˜</a> collection to enable zoom to fit behavior.

For example:

- TS
- Builder API (JSON Config)

``` prism-code
import { ZoomExtentsModifier, easing } from "scichart";

// Add Zoom Extents behavior
const zoomExtentsModifier = new ZoomExtentsModifier({
   isAnimated: true,
   animationDuration: 400,
   easingFunction: easing.outExpo
});
sciChartSurface.chartModifiers.add(zoomExtentsModifier);
```

``` prism-code
// Demonstrates how to configure the ZoomExtentsModifier in SciChart.js using the Builder API
const { chartBuilder, EChart2DModifierType, easing } = SciChart;
// or, for npm, import { chartBuilder, ... } from "scichart"

const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
   modifiers: [
      {
         type: EChart2DModifierType.ZoomExtents,
         options: {
            isAnimated: true,
            animationDuration: 400,
            easingFunction: easing.outExpo
         }
      }
   ]
});
```

This results in the following behavior when double-clicking the chart:

<img src="out_scichartv4/2d-charts/chart-modifier-api/zooming-and-panning/zoom-extents-modifier/index_media/f79b4a87e71073004e42584b86ed224667b14d62.gif" style="display:block;margin-left:auto;margin-right:auto;object-fit:contain;height:auto;width:85%;margin:0 auto" />

## Zoom to a Preset Range<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/zooming-and-panning/zoom-extents-modifier/#zoom-to-a-preset-range" class="hash-link" aria-label="Direct link to Zoom to a Preset Range" translate="no" title="Direct link to Zoom to a Preset Range">â€‹</a>

If you would like the double-click to zoom to some preset range, rather than the data range, you can set <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numericaxis.html#zoomextentsrange" rel="noopener noreferrer" target="_blank">zoomExtentsRangeðŸ“˜</a> on the axes.Â  In addition, if you are setting an initial visibleRange on an axis and would like zoomExtents to return to this range, you can just setÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numericaxis.html#zoomextentstoinitialrange" rel="noopener noreferrer" target="_blank">zoomExtentsToInitialRangeðŸ“˜</a> true, which will set <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numericaxis.html#zoomextentsrange" rel="noopener noreferrer" target="_blank">zoomExtentsRangeðŸ“˜</a> to the visibleRange passed in.

If you just want to have some space around your data, set <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numericaxis.html#growby" rel="noopener noreferrer" target="_blank">growByðŸ“˜</a> instead.

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/zooming-and-panning/zoom-extents-modifier/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [What is the ChartModifier API](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/chart-modifier-api-overview)
- [Common ChartModifiers Features](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/common-features)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-modifier-api/zooming-and-panning/zoom-extents-modifier/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-modifier-api/zooming-and-panning/zoom-extents-modifier/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
