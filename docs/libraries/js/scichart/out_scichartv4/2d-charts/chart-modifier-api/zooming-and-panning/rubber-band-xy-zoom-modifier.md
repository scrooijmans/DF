On this page

# RubberBandXyZoomModifier

SciChart.js provides the ability to Drag an area to zoom theÂ chart (known as Rubber-band zoom)Â using theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rubberbandxyzoommodifier.html" rel="noopener noreferrer" target="_blank">RubberBandXyZoomModifierðŸ“˜</a>, available out of the box.

Besides common features which are inherited from theÂ [ChartModifierBase](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/common-features) class, theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rubberbandxyzoommodifier.html" rel="noopener noreferrer" target="_blank">RubberBandXyZoomModifierðŸ“˜</a>Â allows animated zooming via theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/zoomextentsmodifier.html#isanimated" rel="noopener noreferrer" target="_blank">isAnimatedðŸ“˜</a>,Â <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/zoomextentsmodifier.html#animationduration" rel="noopener noreferrer" target="_blank">animationDurationðŸ“˜</a> andÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/zoomextentsmodifier.html#easingfunction" rel="noopener noreferrer" target="_blank">easingFunctionðŸ“˜</a> properties. The drag rectangle can be styled via theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rubberbandxyzoommodifier.html#fill" rel="noopener noreferrer" target="_blank">fillðŸ“˜</a>,Â <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rubberbandxyzoommodifier.html#stroke" rel="noopener noreferrer" target="_blank">strokeðŸ“˜</a> andÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rubberbandxyzoommodifier.html#strokethickness" rel="noopener noreferrer" target="_blank">strokeThicknessðŸ“˜</a> properties.

## Adding aÂ RubberBandXyZoomModifier to a Chart<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/zooming-and-panning/rubber-band-xy-zoom-modifier/#adding-arubberbandxyzoommodifier-to-a-chart" class="hash-link" aria-label="Direct link to Adding aÂ RubberBandXyZoomModifier to a Chart" translate="no" title="Direct link to Adding aÂ RubberBandXyZoomModifier to a Chart">â€‹</a>

AÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rubberbandxyzoommodifier.html" rel="noopener noreferrer" target="_blank">RubberBandXyZoomModifierðŸ“˜</a>Â can be added to theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#chartmodifiers" rel="noopener noreferrer" target="_blank">sciChartSurface.chartModifiersðŸ“˜</a> collection to enable zoom to fit behavior. For example:

- TS
- Builder API (JSON Config)

``` prism-code
import { RubberBandXyZoomModifier, easing } from "scichart";

// Add Zoom Extents behavior
const rubberBandXyZoomModifier = new RubberBandXyZoomModifier({
   isAnimated: true,
   animationDuration: 400,
   easingFunction: easing.outExpo,
   fill: "#FFFFFF33",
   stroke: "#FFFFFF77",
   strokeThickness: 1,
});
sciChartSurface.chartModifiers.add(rubberBandXyZoomModifier);
```

``` prism-code
// Demonstrates how to configure the RubberBand Zoom Modifier in SciChart.js using the Builder API
const { chartBuilder, EChart2DModifierType, easing } = SciChart;
// or, for npm, import { chartBuilder, ... } from "scichart"

const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
   modifiers: [
      {
         type: EChart2DModifierType.RubberBandXYZoom,
         options: {
            isAnimated: true,
            animationDuration: 400,
            easingFunction: easing.outExpo,
            fill: "#FFFFFF33",
            stroke: "#FFFFFF77",
            strokeThickness: 1,
         }
      }
   ]
});
```

This results in the following behavior when dragging the chart:

<img src="out_scichartv4/2d-charts/chart-modifier-api/zooming-and-panning/rubber-band-xy-zoom-modifier/index_media/b2b61aeacd32643b11894e5e644f7cfc67c54361.gif" style="display:block;margin-left:auto;margin-right:auto;object-fit:contain;height:auto;width:85%;margin:0 auto" />

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/zooming-and-panning/rubber-band-xy-zoom-modifier/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [What is the ChartModifier API](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/chart-modifier-api-overview)
- [Common ChartModifiers Features](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/common-features)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-modifier-api/zooming-and-panning/rubber-band-xy-zoom-modifier/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-modifier-api/zooming-and-panning/rubber-band-xy-zoom-modifier/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
