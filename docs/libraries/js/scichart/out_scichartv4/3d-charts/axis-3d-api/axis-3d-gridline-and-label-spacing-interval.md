On this page

# Axis3D Gridline and Label Spacing (Interval)

Axis 3D Gridline and Label Spacing obeys the same rules as SciChart 2D. Here are the key principles.

- Each axis has aÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#majordelta" rel="noopener noreferrer" target="_blank">axisCore.MajorDeltaðŸ“˜</a>Â andÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#minordelta" rel="noopener noreferrer" target="_blank">axisCore.MinorDeltaðŸ“˜</a>, which specify the interval of major gridlines / labels and minor gridlines respectively. These are normally calculated automatically. They can beÂ set manually along withÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase3d.html#autoticks" rel="noopener noreferrer" target="_blank">axis.autoTicksðŸ“˜</a> = false to achieve user-defined intervals.
- You can also set hints such asÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase3d.html#maxautoticks" rel="noopener noreferrer" target="_blank">axis.maxAutoTicksðŸ“˜</a> orÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase3d.html#minorspermajor" rel="noopener noreferrer" target="_blank">axis.minorsPerMajorðŸ“˜</a> to adjust the number of gridlines using the automatic built-in intervals.
- Or, if you want fine-grained control over gridline spacing and to apply custom or dynamic rules, you can create aÂ [TickProvider plugin](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-labels/label-provider-api-overview) to do it

![](out_scichartv4/3d-charts/axis-3d-api/axis-3d-gridline-and-label-spacing-interval/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

Background reading: Read theÂ [Axis Ticks - Gridline and Label Spacing](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-tick-label-interval/gridline-and-label-spacing-interval/) and the advanced articleÂ [Axis Ticks - Programmatic Gridline Spacing](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-tick-label-interval/tick-provider-api/) to learn more about this powerful API.

## Simple Example of spacing Gridlines<a href="https://www.scichart.com/documentation/js/v4/3d-charts/axis-3d-api/axis-3d-gridline-and-label-spacing-interval/#simple-example-of-spacing-gridlines" class="hash-link" aria-label="Direct link to Simple Example of spacing Gridlines" translate="no" title="Direct link to Simple Example of spacing Gridlines">â€‹</a>

Here is a code sample that demonstrates the three ways to space gridlines.

### Automatic Spacing<a href="https://www.scichart.com/documentation/js/v4/3d-charts/axis-3d-api/axis-3d-gridline-and-label-spacing-interval/#automatic-spacing" class="hash-link" aria-label="Direct link to Automatic Spacing" translate="no" title="Direct link to Automatic Spacing">â€‹</a>

Automatic gridline and label spacing (default) can be adjusted by setting theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase3d.html#maxautoticks" rel="noopener noreferrer" target="_blank">axis.maxAutoTicksðŸ“˜</a> and <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase3d.html#minorspermajor" rel="noopener noreferrer" target="_blank">axis.minorsPerMajorðŸ“˜</a>Â properties.

- TS

``` prism-code
// Define the X Axis with automatic spacing, and optional hint to set the max
// number of gridlines, labels and minor gridlines
sciChart3DSurface.xAxis = new NumericAxis3D(wasmContext, {
    axisTitle: "X [Automatic Spacing]",
    visibleRange: new NumberRange(0, 10),
    autoTicks: true, // default value is true. Major/Minor Deltas are calculated automatically
    maxAutoTicks: 5, // Hint: 5 major gridlines and labels
    minorsPerMajor: 4 // Exact: 4 minor gridlines per major gridline
});
```

### Manual Spacing<a href="https://www.scichart.com/documentation/js/v4/3d-charts/axis-3d-api/axis-3d-gridline-and-label-spacing-interval/#manual-spacing" class="hash-link" aria-label="Direct link to Manual Spacing" translate="no" title="Direct link to Manual Spacing">â€‹</a>

To manually specify gridline and label intervals, setÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase3d.html#autoticks" rel="noopener noreferrer" target="_blank">axis.autoTicksðŸ“˜</a> = falseÂ then setÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#majordelta" rel="noopener noreferrer" target="_blank">axisCore.MajorDeltaðŸ“˜</a>Â andÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#minordelta" rel="noopener noreferrer" target="_blank">axisCore.MinorDeltaðŸ“˜</a>.

- TS

``` prism-code
sciChart3DSurface.yAxis = new NumericAxis3D(wasmContext, {
    axisTitle: "Y [Manual Spacing]",
    visibleRange: new NumberRange(0, 10),
    autoTicks: false, // Major/Minor Deltas are specified manually
    majorDelta: 5,
    minorDelta: 1
});
```

### Custom Spacing<a href="https://www.scichart.com/documentation/js/v4/3d-charts/axis-3d-api/axis-3d-gridline-and-label-spacing-interval/#custom-spacing" class="hash-link" aria-label="Direct link to Custom Spacing" translate="no" title="Direct link to Custom Spacing">â€‹</a>

Finally, to specify custom spacing or irregular spacing, you can create a class which inherits from NumericTickProvider and attach to the axis like this.

- TS

``` prism-code
// Custom TickProvider implementation
//
class CustomTickProvider extends NumericTickProvider {
    constructor(wasmContext) {
        super(wasmContext);
    }

    // returns an array of minor gridline positions in data space
    // Called once per draw. Can be dynamic
    getMinorTicks(minorDelta, majorDelta, visibleRange) {
        // Todo here: calculate your tick spacing based on axis minorDelta, majorDelta and visibleRange
        // Note we do not return major ticks here, so minor ticks exclude the majors
        return [
            0.2, 0.4, 0.6, 0.8, 1.2, 1.4, 1.6, 1.8, 2.2, 2.4, 2.6, 2.8, 3.0, 3.2, 3.4, 3.6, 3.8, 4.2, 4.4, 4.6, 4.8,
            5.0, 5.2, 5.4, 5.6, 5.8, 6.0, 6.2, 6.4, 6.6, 6.8, 7.0, 7.2, 7.4, 7.6, 7.8, 8.2, 8.4, 8.6, 8.8, 9.0, 9.2,
            9.4, 9.6, 9.8
        ];
    }

    // returns an array of major gridline positions in data space
    // Called once per draw. Can be dynamic
    getMajorTicks(minorDelta, majorDelta, visibleRange) {
        // Todo here: calculate your tick spacing based on axis minorDelta, majorDelta and visibleRange
        // Note we return the major tick intervals and label intervals here
        return [0, 1, 2, 4, 8];
    }
}

// Create the X-Axis
sciChart3DSurface.zAxis = new NumericAxis3D(wasmContext, {
    axisTitle: "Z [Custom Spacing]",
    visibleRange: new NumberRange(0, 10)
});

// Apply the tick provider
sciChart3DSurface.zAxis.tickProvider = new CustomTickProvider(wasmContext);
```

Putting this all together, we've created an example to show you all three spacing methods in one 3D chart.

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/3d-charts/axis-3d-api/axis-3d-gridline-and-label-spacing-interval/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/3d-charts/axis-3d-api/axis-3d-gridline-and-label-spacing-interval/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
