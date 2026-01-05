On this page

# Axis Offset and OverrideOffset

## Default Axis Offset<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/multi-axis-and-layout/axis-offset-and-overrideOffset/#default-axis-offset" class="hash-link" aria-label="Direct link to Default Axis Offset" translate="no" title="Direct link to Default Axis Offset">â€‹</a>

In SciChart.js, each axis has an offset parameter that defines its position along the chart layout. By default, this offset is set internally (commonly as -1 for normal axes). This default lets the layout engine manage axis positioning automatically so axes do not crowd or overlap with the chart or each other.

Regular axes use the internal offset assigned by the chartâ€™s layout system. This ensures enough spacing for labels, ticks, gridlines, and keeps the chart visually organized.

## Why Use overrideOffset and Not offset<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/multi-axis-and-layout/axis-offset-and-overrideOffset/#why-use-overrideoffset-and-not-offset" class="hash-link" aria-label="Direct link to Why Use overrideOffset and Not offset" translate="no" title="Direct link to Why Use overrideOffset and Not offset">â€‹</a>

To manually control axis positioning, you should use <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numericaxis.html#overrideoffset" rel="noopener noreferrer" target="_blank">overrideOffsetðŸ“˜</a> rather than trying to set <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numericaxis.html#offset" rel="noopener noreferrer" target="_blank">offsetðŸ“˜</a> directly.

The offset property is managed by SciChartâ€™s layout strategies (especially when you have stacked or multi-axis layouts). Overriding this manually can cause unexpected results if the internal layout logic adjusts it later.

Setting <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numericaxis.html#overrideoffset" rel="noopener noreferrer" target="_blank">overrideOffsetðŸ“˜</a> ensures that the value you supply takes precedence, overriding whatever offset value the layout system would otherwise use.

## How overrideOffset Creates the 3D Waterfall Effect<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/multi-axis-and-layout/axis-offset-and-overrideOffset/#how-overrideoffset-creates-the-3d-waterfall-effect" class="hash-link" aria-label="Direct link to How overrideOffset Creates the 3D Waterfall Effect" translate="no" title="Direct link to How overrideOffset Creates the 3D Waterfall Effect">â€‹</a>

Above: The JavaScript <a href="https://www.scichart.com/demo/iframe/interactive-waterfall-chart" target="_blank">Interactive Waterfall Spectral Chart</a> example from the <a href="https://www.scichart.com/demo/react" target="_blank">SciChart.js Demo</a>

  

Here is the code from the example that shows how <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numericaxis.html#overrideoffset" rel="noopener noreferrer" target="_blank">overrideOffsetðŸ“˜</a> is used:

``` prism-code
        for (let i = 0; i < seriesCount; i++) {
            // Create one yAxis per series
            const yAxis = new NumericAxis(wasmContext, {
                id: "Y" + i,
                axisAlignment: EAxisAlignment.Left,
                maxAutoTicks: 5,
                drawMinorGridLines: false,
                visibleRange: new NumberRange(-50, 60),
                isVisible: i === seriesCount - 1,
                overrideOffset: 3 * -i,
            });
            sciChartSurface.yAxes.add(yAxis);

            // Create a shared, default xaxis
            const xAxis = new NumericAxis(wasmContext, {
                id: "X" + i,
                axisAlignment: EAxisAlignment.Bottom,
                maxAutoTicks: 5,
                drawMinorGridLines: false,
                growBy: new NumberRange(0, 0.2),
                isVisible: i === seriesCount - 1,
                overrideOffset: 2 * i,
            });
            sciChartSurface.xAxes.add(xAxis);
            ...
```

In interactive waterfall charts where several series are visually stacked in an overlapping â€œ3Dâ€? style <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numericaxis.html#overrideoffset" rel="noopener noreferrer" target="_blank">overrideOffsetðŸ“˜</a> is critical for achieving the desired offset between axes.

Each â€œlayerâ€? or â€œsliceâ€? in a waterfall chart typically gets its own axis with a unique <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numericaxis.html#overrideoffset" rel="noopener noreferrer" target="_blank">overrideOffsetðŸ“˜</a>. By increasing the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numericaxis.html#overrideoffset" rel="noopener noreferrer" target="_blank">overrideOffsetðŸ“˜</a> value for successive axes, you move each axis and corresponding series further away from the previous one, creating the illusion of depth making the chart look like a 3D waterfall.

Manual control via <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numericaxis.html#overrideoffset" rel="noopener noreferrer" target="_blank">overrideOffsetðŸ“˜</a> allows precise tuning: you control how much each series/axis is visually offset, stacking the axes apart and making each new series appear "further back" in the chart layout.

This method is especially useful in interactive waterfall charts where multiple stacked axes and series must not overlap and each needs unique spacing for clarity and aesthetics.

![](out_scichartv4/2d-charts/axis-api/multi-axis-and-layout/axis-offset-and-overrideOffset/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

The <a href="https://www.scichart.com/demo/react/interactive-waterfall-chart" rel="noopener noreferrer" target="_blank">Interactive Waterfall Spectral Chart</a> can be found in theÂ <a href="https://github.com/ABTSoftware/SciChart.JS.Examples/tree/master/Examples/src/components/Examples/FeaturedApps/ScientificCharts/InteractiveWaterfallChart" rel="noopener noreferrer" target="_blank">SciChart.Js Examples Suite &gt; Interactive Waterfall Chart Example</a>Â on Github.

## Summary Table<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/multi-axis-and-layout/axis-offset-and-overrideOffset/#summary-table" class="hash-link" aria-label="Direct link to Summary Table" translate="no" title="Direct link to Summary Table">â€‹</a>

| Property | Default Behavior | When to Use | Key Use-case |
|----|----|----|----|
| offset | Managed by layout engine (default usually -1) | Never set directly | Used internally for positioning |
| overrideOffset | Unset; enables manual control | Set explicitly to control axis position | Essential for stacking (e.g., 3D waterfall charts) |

In SciChart.js, always use overrideOffset to manually set axis positions especially in interactive, multi axis layouts (like waterfall charts) for visual separation and to achieve advanced effects such as 3D stacking. Directly setting offset is not recommended as itâ€™s controlled by the internal engine and may not persist or behave as expected.

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/axis-api/multi-axis-and-layout/axis-offset-and-overrideOffset/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/axis-api/multi-axis-and-layout/axis-offset-and-overrideOffset/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
