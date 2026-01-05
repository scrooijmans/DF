On this page

# Rotating Axis Labels

SciChart.js supports rotation of labelsÂ for all 2D axis types andÂ [LabelProviders](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-labels/label-provider-api-overview). This lets you display longer labels, or pack more labels onto an x axis.

To use rotated labels on a chart, or vertical labels, use this code:

- TS
- Builder API (JSON Config)

``` prism-code
const { wasmContext, sciChartSurface } = await SciChartSurface.create(divElementId, {
    theme: new SciChartJsNavyTheme()
});

const xAxis = new NumericAxis(wasmContext, {
    axisTitle: "X Axis / 90 Degree Rotation",
    visibleRange: new NumberRange(1e6, 2e6),
    labelFormat: ENumericFormat.Decimal,
    labelPrecision: 4,
    // Allow more labels for the demo
    maxAutoTicks: 30,
    // Rotation is in degrees clockwise. Negative numbers are OK
    rotation: 90,
    // Turn off minor gridlines, since majors are now closer together
    drawMinorGridLines: false
});

// Add the xAxis to the chart
sciChartSurface.xAxes.add(xAxis);

// Creating a NumericAxis as a YAxis on the left
sciChartSurface.yAxes.add(
    new NumericAxis(wasmContext, {
        axisTitle: "Y Axis, -25 Degree Rotation",
        rotation: -25,
        labelFormat: ENumericFormat.Decimal,
        labelPrecision: 4
    })
);
```

``` prism-code
// If you want to show an Axis with rotated labels. Using a numeric axis for example
const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    surface: { theme: { type: EThemeProviderType.Dark } },
    xAxes: {
        type: EAxisType.NumericAxis,
        options: {
            axisTitle: "X Axis / 90 Degree Rotation",
            visibleRange: new NumberRange(1e6, 2e6),
            labelFormat: ENumericFormat.Decimal,
            labelPrecision: 4,
            // Allow more labels for the demo
            maxAutoTicks: 30,
            // Rotation is in degrees clockwise. Negative numbers are OK
            rotation: 90,
            // Turn off minor gridlines, since majors are now closer together
            drawMinorGridLines: false
        }
    },
    yAxes: {
        type: EAxisType.NumericAxis,
        options: {
            axisTitle: "Y Axis, Numeric",
            rotation: 25,
            labelFormat: ENumericFormat.Decimal,
            labelPrecision: 4
        }
    }
});
```

This results in the following output:

![](out_scichartv4/2d-charts/axis-api/axis-labels/rotating-axis-labels/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

For an example of using rotation with text labels see theÂ <a href="https://www.scichart.com/demo/javascript-multiline-labels" rel="noopener noreferrer" target="_blank">Multiline and Rotated Text Labels</a> demo in our examples suite.

### Further notes on Label Culling & Spacing<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-labels/rotating-axis-labels/#further-notes-on-label-culling--spacing" class="hash-link" aria-label="Direct link to Further notes on Label Culling &amp; Spacing" translate="no" title="Direct link to Further notes on Label Culling &amp; Spacing">â€‹</a>

![](out_scichartv4/2d-charts/axis-api/axis-labels/rotating-axis-labels/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

An axis with rotated labels obeys other rules of axis tick spacing and label culling. Take a look at the section onÂ [Gridline and Label Spacing (Interval)](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-tick-label-interval/gridline-and-label-spacing-interval) for some more information how this works.

When working with rotated labels that are not horizontal or vertical, it may be necessary to turn offÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisrenderer.html#hideoverlappinglabels" rel="noopener noreferrer" target="_blank">axis.axisRenderer.hideOverlappingLabelsðŸ“˜</a>Â as the bounding box of a partially rotated label is much larger than the text itself.This property may also be set via the axis constructor optionÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iaxisbase2doptions.html#hideoverlappinglabels" rel="noopener noreferrer" target="_blank">hideOverlappingLabelsðŸ“˜</a>.

To do this, use the following code:

``` prism-code
// hideOverlappingLabels Â Example
// Either
const xAxis = new NumericAxis(wasmContext, {
    // Allow labels to overlap
    hideOverlappingLabels : false
});
// Or
const xAxis = new NumericAxis(wasmContext);
// Allow rotated labels to overlap
xAxis.axisRenderer.hideOverlappingLabels = false;
```

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/axis-api/axis-labels/rotating-axis-labels/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/axis-api/axis-labels/rotating-axis-labels/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
