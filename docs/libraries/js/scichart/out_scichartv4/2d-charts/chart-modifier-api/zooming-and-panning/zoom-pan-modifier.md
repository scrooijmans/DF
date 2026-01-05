On this page

# ZoomPanModifier

SciChart.js provides an scrolling / panningÂ behavior via theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/zoompanmodifier.html" rel="noopener noreferrer" target="_blank">ZoomPanModifierðŸ“˜</a>, available out of the box.

![](out_scichartv4/2d-charts/chart-modifier-api/zooming-and-panning/zoom-pan-modifier/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

As of SciChart.js v3.2, <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/zoompanmodifier.html" rel="noopener noreferrer" target="_blank">ZoomPanModifierðŸ“˜</a> now inherits <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/pinchzoommodifier.html" rel="noopener noreferrer" target="_blank">PinchZoomModifierðŸ“˜</a>, allowing you to configure zooming, panning and touch-to-zoom interaction via a single modifier.

All the properties for theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/pinchzoommodifier.html" rel="noopener noreferrer" target="_blank">PinchZoomModifierðŸ“˜</a> may be set to control vertical/horizontal zooming, include/exclude axis from pinch zooming etc..

Besides common features which are inherited from theÂ [ChartModifierBase](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/common-features) class, theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/zoompanmodifier.html" rel="noopener noreferrer" target="_blank">ZoomPanModifierðŸ“˜</a>Â allows toÂ **restrict scrolling** to the horizontal or vertical direction only, via theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/zoompanmodifier.html#xydirection" rel="noopener noreferrer" target="_blank">xyDirectionðŸ“˜</a> property.

## Adding aÂ ZoomPanModifierÂ to a Chart<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/zooming-and-panning/zoom-pan-modifier/#adding-azoompanmodifierto-a-chart" class="hash-link" aria-label="Direct link to Adding aÂ ZoomPanModifierÂ to a Chart" translate="no" title="Direct link to Adding aÂ ZoomPanModifierÂ to a Chart">â€‹</a>

A ZoomPanModifier can be added to theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#chartmodifiers" rel="noopener noreferrer" target="_blank">sciChartSurface.chartModifiersðŸ“˜</a> collection to enable panning behavior. For example:

- TS
- Builder API (JSON Config)

``` prism-code
const { ZoomPanModifier, EXyDirection } = SciChart;
// or for npm: import { PinchZoomModifier } from "scichart";

// Add Zoom Pan and Pinch behaviour to the chart. All parameters are optional
sciChartSurface.chartModifiers.add(new ZoomPanModifier({
    // Specifies Panning in X,Y direction or both
    xyDirection: EXyDirection.XyDirection,
    // Enables Pinch Zoom functionality
    enableZoom: true,
    // Optional parameters specify the amount of pinch zooming in X/Y  Default is 0.005
    horizontalGrowFactor: 0.005,
    verticalGrowFactor: 0.005
    // Optional parameters to include/exclude X/Y axis from zooming by axis.id
    // If not specified, by default, all axis are included in zooming
    // either use:
    // excludedXAxisIds: ["XAxis1"],
    // excludedYAxisIds: ["YAxis1"],
    // or:
    // includedXAxisIds: ["XAxis2"],
    // includedYAxisIds: ["YAxis2"],
}));
```

``` prism-code
// Demonstrates how to configure the ZoomPanModifier in SciChart.js using the Builder API
const { chartBuilder, EThemeProviderType, EAxisType, EChart2DModifierType, EXyDirection } = SciChart;
// or, for npm, import { chartBuilder, ... } from "scichart"

const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    surface: { theme: { type: EThemeProviderType.Dark } },
    xAxes: { type: EAxisType.NumericAxis },
    yAxes: { type: EAxisType.NumericAxis },
    modifiers: [
        {
            type: EChart2DModifierType.ZoomPan,
            options: {
                // Specifies Panning in X,Y direction or both
                xyDirection: EXyDirection.XyDirection,
                // Enables Pinch Zoom functionality
                enableZoom: true,
                // Optional parameters specify the amount of pinch zooming in X/Y  Default is 0.005
                horizontalGrowFactor: 0.005,
                verticalGrowFactor: 0.005
            }
        }
    ]
});
```

This results in the following behavior:

## Additional Properties<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/zooming-and-panning/zoom-pan-modifier/#additional-properties" class="hash-link" aria-label="Direct link to Additional Properties" translate="no" title="Direct link to Additional Properties">â€‹</a>

### Allow Panning in only one direction (X or Y)<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/zooming-and-panning/zoom-pan-modifier/#allow-panning-in-only-one-direction-x-or-y" class="hash-link" aria-label="Direct link to Allow Panning in only one direction (X or Y)" translate="no" title="Direct link to Allow Panning in only one direction (X or Y)">â€‹</a>

Panning can be restricted to X or Y by setting the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/zoompanmodifier.html#xydirection" rel="noopener noreferrer" target="_blank">ZoomPanModifier.xyDirectionðŸ“˜</a> property.

### Allow Panning on only one X/Y axis<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/zooming-and-panning/zoom-pan-modifier/#allow-panning-on-only-one-xy-axis" class="hash-link" aria-label="Direct link to Allow Panning on only one X/Y axis" translate="no" title="Direct link to Allow Panning on only one X/Y axis">â€‹</a>

Panning can be restricted to a single X or Y axis by setting the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/zoompanmodifier.html#xaxisid" rel="noopener noreferrer" target="_blank">ZoomPanModifier.xAxisIdðŸ“˜</a> or <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/zoompanmodifier.html#yaxisid" rel="noopener noreferrer" target="_blank">ZoomPanModifier.yAxisIdðŸ“˜</a> properties.

### Adjust Pinch ZoomingÂ / Scale Factor<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/zooming-and-panning/zoom-pan-modifier/#adjust-pinch-zooming-scale-factor" class="hash-link" aria-label="Direct link to Adjust Pinch ZoomingÂ / Scale Factor" translate="no" title="Direct link to Adjust Pinch ZoomingÂ / Scale Factor">â€‹</a>

![](out_scichartv4/2d-charts/chart-modifier-api/zooming-and-panning/zoom-pan-modifier/index_media/af50301a53fa555616a9b2279e7e25a1d566cb1a.svg)info

The following is inherited from **PinchZoomModifier**

Horizontal and vertical pinch zoom scale factor can be adjusted via the following properties. The default value is set to `0.005`.

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/zoompanmodifier.html#horizontalgrowfactor" rel="noopener noreferrer" target="_blank">ZoomPanModifier.horizontalGrowFactorðŸ“˜</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/zoompanmodifier.html#verticalgrowfactor" rel="noopener noreferrer" target="_blank">ZoomPanModifier.verticalGrowFactorðŸ“˜</a>

### Include/Exclude Certain Axis from Pinch Zoom<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/zooming-and-panning/zoom-pan-modifier/#includeexclude-certain-axis-from-pinch-zoom" class="hash-link" aria-label="Direct link to Include/Exclude Certain Axis from Pinch Zoom" translate="no" title="Direct link to Include/Exclude Certain Axis from Pinch Zoom">â€‹</a>

The ZoomPanModifier allows you to include or exclude certain axis by axis.id from the pinch zoom operation.

By default all axis are included, to exclude one or more X or Y axis, set the following property:

- Exclude Axis

``` prism-code
// Exclude a specific axis from the pinch zoom operation
zoomPanModifier.includeXAxis(axisXInstance, false);
zoomPanModifier.includeYAxis(axisYInstance, false);

// Include specific axis from the pinch zoom operation
zoomPanModifier.includeXAxis(axisXInstance, true);
zoomPanModifier.includeYAxis(axisYInstance, true);

// Reset flags
zoomPanModifier.includeAllAxes();
```

### Allow Pinch Zoom in only one direction<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/zooming-and-panning/zoom-pan-modifier/#allow-pinch-zoom-in-only-one-direction" class="hash-link" aria-label="Direct link to Allow Pinch Zoom in only one direction" translate="no" title="Direct link to Allow Pinch Zoom in only one direction">â€‹</a>

If you want to enable pinch zooming in only one direction,Â  e.g. horizontal only, modify the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/zoompanmodifier.html#verticalgrowfactor" rel="noopener noreferrer" target="_blank">ZoomPanModifier.verticalGrowFactorðŸ“˜</a> to equal `0`.

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/zooming-and-panning/zoom-pan-modifier/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [What is the ChartModifier API](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/chart-modifier-api-overview)
- [Common ChartModifiers Features](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/common-features)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-modifier-api/zooming-and-panning/zoom-pan-modifier/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-modifier-api/zooming-and-panning/zoom-pan-modifier/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
