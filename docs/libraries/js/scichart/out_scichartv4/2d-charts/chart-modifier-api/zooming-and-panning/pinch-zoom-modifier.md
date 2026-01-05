On this page

# PinchZoomModifier

SciChart.js provides Pinch zooming on touch devices via the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/pinchzoommodifier.html" rel="noopener noreferrer" target="_blank">PinchZoomModifierðŸ“˜</a> type.

![](out_scichartv4/2d-charts/chart-modifier-api/zooming-and-panning/pinch-zoom-modifier/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

As of SciChart.js v3.2, **ZoomPanModifier** now inherits **PinchZoomModifier**, allowing you to configure zooming, panning and touch-to-zoom interaction via a single modifier.

## Adding a PinchZoomModifier to a Chart<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/zooming-and-panning/pinch-zoom-modifier/#adding-a-pinchzoommodifier-to-a-chart" class="hash-link" aria-label="Direct link to Adding a PinchZoomModifier to a Chart" translate="no" title="Direct link to Adding a PinchZoomModifier to a Chart">â€‹</a>

A **PinchZoomModifier** may be added to theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#chartmodifiers" rel="noopener noreferrer" target="_blank">sciChartSurface.chartModifiersðŸ“˜</a> colletion to enable pinch to zoom behaviour. For example:

- TS
- Builder API (JSON Config)

``` prism-code
// Demonstrates how to configure PinchZoomModifier SciChart.js
const { SciChartSurface, NumericAxis, PinchZoomModifier } = SciChart;
// or, for npm, import { SciChartSurface, ... } from "scichart"

const { wasmContext, sciChartSurface } = await SciChartSurface.create(divElementId);

// Create an X and Y Axis
sciChartSurface.yAxes.add(new NumericAxis(wasmContext));
sciChartSurface.xAxes.add(new NumericAxis(wasmContext));

const pinchZoomModifier = new PinchZoomModifier({
    horizontalGrowFactor: 0.001,
    verticalGrowFactor: 0.001,
    excludedXAxisIds: ["xAxis2"],
    includedYAxisIds: ["yAxis1"]
});
sciChartSurface.chartModifiers.add(pinchZoomModifier);
```

``` prism-code
// Demonstrates how to configure a PinchZoom Modifier in SciChart.js using the Builder API
const { chartBuilder, EChart2DModifierType } = SciChart;
// or, for npm, import { chartBuilder, ... } from "scichart"

const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    modifiers: [
        {
            type: EChart2DModifierType.PinchZoom,
            options: {
                horizontalGrowFactor: 0.001,
                verticalGrowFactor: 0.001,
                excludedXAxisIds: ["xAxis2"],
                includedYAxisIds: ["yAxis1"]
            }
        }
    ]
});
```

This results in the following output

## Additional Properties<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/zooming-and-panning/pinch-zoom-modifier/#additional-properties" class="hash-link" aria-label="Direct link to Additional Properties" translate="no" title="Direct link to Additional Properties">â€‹</a>

### Adjust ZoomingÂ / Scale Factor<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/zooming-and-panning/pinch-zoom-modifier/#adjust-zooming-scale-factor" class="hash-link" aria-label="Direct link to Adjust ZoomingÂ / Scale Factor" translate="no" title="Direct link to Adjust ZoomingÂ / Scale Factor">â€‹</a>

Horizontal and vertical zoom scale factor can be adjusted via the following properties:

- <a href="https://www.scichart.com/documentation/js/current/typedoc/classes/pinchzoommodifier.html#horizontalgrowfactor" rel="noopener noreferrer" target="_blank">PinchZoomModifier.horizontalGrowFactorðŸ“˜</a>
- <a href="https://www.scichart.com/documentation/js/current/typedoc/classes/pinchzoommodifier.html#verticalgrowfactor" rel="noopener noreferrer" target="_blank">PinchZoomModifier.verticalGrowFactorðŸ“˜</a>

The default value for both is set to `0.005`.

### Allow Pinch Zoom in only one direction<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/zooming-and-panning/pinch-zoom-modifier/#allow-pinch-zoom-in-only-one-direction" class="hash-link" aria-label="Direct link to Allow Pinch Zoom in only one direction" translate="no" title="Direct link to Allow Pinch Zoom in only one direction">â€‹</a>

If you want to enable pinch zooming in only one direction,Â e.g. horizontal only, modify the **PinchZoomModifier.verticalGrowFactor** to equal `0`.

### Include/Exclude Certain Axis from Pinch Zoom<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/zooming-and-panning/pinch-zoom-modifier/#includeexclude-certain-axis-from-pinch-zoom" class="hash-link" aria-label="Direct link to Include/Exclude Certain Axis from Pinch Zoom" translate="no" title="Direct link to Include/Exclude Certain Axis from Pinch Zoom">â€‹</a>

The PinchZoomModifier allows you to include or exclude certain axis by axis.id from the zoom operation.

By default all axis are included, to exclude one or more X or Y axis, set the following property:

- Exclude Axis

``` prism-code
// Exclude a specific axis from the pinch zoom operation
pinchZoomModifier.includeXAxis(axisXInstance, false);
pinchZoomModifier.includeYAxis(axisYInstance, false);

// Include specific axis from the pinch zoom operation
pinchZoomModifier.includeXAxis(axisXInstance, true);
pinchZoomModifier.includeYAxis(axisYInstance, true);

// Reset flags
pinchZoomModifier.includeAllAxes();
```

### Allow Pinch Zoom in only one direction<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/zooming-and-panning/pinch-zoom-modifier/#allow-pinch-zoom-in-only-one-direction-1" class="hash-link" aria-label="Direct link to Allow Pinch Zoom in only one direction" translate="no" title="Direct link to Allow Pinch Zoom in only one direction">â€‹</a>

If you want to enable pinch zooming in only one direction,Â e.g. horizontal only, modify the **PinchZoomModifier.verticalGrowFactor** to equal `0`.

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-modifier-api/zooming-and-panning/pinch-zoom-modifier/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-modifier-api/zooming-and-panning/pinch-zoom-modifier/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
