On this page

# The VerticalSliceModifier Type

With theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticalslicemodifier.html" rel="noopener noreferrer" target="_blank">VerticalSliceModifierðŸ“˜</a>, SciChart.js provides the ability to place multiple vertical lines on the chart, which can show tooltips intersecting chart series.

This provides a similar behaviour to theÂ [RolloverModifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/rollover-modifier), except instead of a single vertical line plus tooltips which track the mouse, you can place multiple draggable vertical lines on the chart, which intersect line series and display tooltips.

TheÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticalslicemodifier.html" rel="noopener noreferrer" target="_blank">VerticalSliceModifierðŸ“˜</a> allows you to:

- Place one or more vertical lines on the chart at data-values, relative coordinates or pixel coordinates
- Vertical lines intersect chart series
- Tooltips may be displayed at the intersections
- Optionally show/hide or customize the tooltip content
- Vertical lines may be dragged or edited
- Vertical lines may be removed from the chart.

TheÂ <a href="https://www.scichart.com/demo/javascript/chart-vertical-slice-modifier" rel="noopener noreferrer" target="_blank">Using VerticalSliceModifier Example</a>Â can be found in theÂ <a href="https://github.com/abtsoftware/scichart.js.examples" rel="noopener noreferrer" target="_blank">SciChart.Js Examples Suite</a>Â on Github, or our live demo atÂ <a href="https://www.scichart.com/demo/javascript-line-chart" rel="noopener noreferrer" target="_blank">scichart.com/demo</a>

## Adding a VerticalSliceModifier to a Chart<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/vertical-slice-modifier/vertical-slice-modifier-overview/#adding-a-verticalslicemodifier-to-a-chart" class="hash-link" aria-label="Direct link to Adding a VerticalSliceModifier to a Chart" translate="no" title="Direct link to Adding a VerticalSliceModifier to a Chart">â€‹</a>

One or moreÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticalslicemodifier.html" rel="noopener noreferrer" target="_blank">VerticalSliceModifiersðŸ“˜</a> can be added to theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#chartmodifiers" rel="noopener noreferrer" target="_blank">sciChartSurface.chartModifiersðŸ“˜</a>Â collection to enable draggable lines with crosshair/cursor behavior. For example, this code adds a crosshair, enables defaultÂ tooltips and allows dragging of the vertical lines.

- TS
- Builder API (JSON Config)

``` prism-code
// Create a chart surface
const { sciChartSurface, wasmContext } = await SciChartSurface.create(divElementId, {
    theme: new SciChartJsNavyTheme(),
    titleStyle: { fontSize: 16 }
});

// For the example to work, axis must have EAutoRange.Always
sciChartSurface.xAxes.add(new NumericAxis(wasmContext, { axisTitle: "X Axis" }));
sciChartSurface.yAxes.add(
    new NumericAxis(wasmContext, {
        visibleRange: new NumberRange(-2, 0.5),
        axisTitle: "Y Axis"
    })
);

// Add some vertical slices to the chart
const vSlice1 = new VerticalSliceModifier({
    x1: 10.1,
    xCoordinateMode: ECoordinateMode.DataValue,
    isDraggable: true,
    // Defines if rollover vertical line is shown
    showRolloverLine: true,
    rolloverLineStrokeThickness: 1,
    rolloverLineStroke: "#FF6600",
    lineSelectionColor: "#FF6600",
    // Shows the default tooltip
    showTooltip: true
});
const vSlice2 = new VerticalSliceModifier({
    x1: 30.0,
    xCoordinateMode: ECoordinateMode.DataValue,
    isDraggable: true,
    // Defines if rollover vertical line is shown
    showRolloverLine: true,
    rolloverLineStrokeThickness: 1,
    rolloverLineStroke: "#50C7E0",
    lineSelectionColor: "#50C7E0",
    // Shows the default tooltip
    showTooltip: true
});
sciChartSurface.chartModifiers.add(vSlice1, vSlice2);
```

``` prism-code
// Demonstrates how to configure the VerticalSliceModifier in SciChart.js using the Builder API
const { chartBuilder, EThemeProviderType, ECoordinateMode, EChart2DModifierType } = SciChart;
// or, for npm, import { chartBuilder, ... } from "scichart"

const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    surface: { theme: { type: EThemeProviderType.Dark } },
    modifiers: [
        {
            type: EChart2DModifierType.VerticalSlice,
            options: {
                x1: 10.1,
                xCoordinateMode: ECoordinateMode.DataValue,
                isDraggable: true,
                // Defines if rollover vertical line is shown
                showRolloverLine: true,
                rolloverLineStrokeThickness: 1,
                rolloverLineStroke: "#FF6600",
                lineSelectionColor: "#FF6600",
                // Shows the default tooltip
                showTooltip: true
            }
        },
        {
            type: EChart2DModifierType.VerticalSlice,
            options: {
                x1: 30.0,
                xCoordinateMode: ECoordinateMode.DataValue,
                isDraggable: true,
                // Defines if rollover vertical line is shown
                showRolloverLine: true,
                rolloverLineStrokeThickness: 1,
                rolloverLineStroke: "#50C7E0",
                lineSelectionColor: "#50C7E0",
                // Shows the default tooltip
                showTooltip: true
            }
        }
    ]
});
```

This results in the following output:Â 

Many of the properties here are optional - they have been included to show the configuration possibilities for the cursor. SeeÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iverticalsliceoptions.html" rel="noopener noreferrer" target="_blank">IVerticalSliceOptionsðŸ“˜</a> for more.

## Removing a VerticalSliceModifier from the Chart<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/vertical-slice-modifier/vertical-slice-modifier-overview/#removing-a-verticalslicemodifier-from-the-chart" class="hash-link" aria-label="Direct link to Removing a VerticalSliceModifier from the Chart" translate="no" title="Direct link to Removing a VerticalSliceModifier from the Chart">â€‹</a>

``` prism-code
sciChartSurface.chartModifiers.remove(vSlice);
```

## Styling & Visibility ofÂ VerticalSliceModifier Elements<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/vertical-slice-modifier/vertical-slice-modifier-overview/#styling--visibility-ofverticalslicemodifier-elements" class="hash-link" aria-label="Direct link to Styling &amp; Visibility ofÂ VerticalSliceModifier Elements" translate="no" title="Direct link to Styling &amp; Visibility ofÂ VerticalSliceModifier Elements">â€‹</a>

The following elements can be turned on or off when using theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticalslicemodifier.html" rel="noopener noreferrer" target="_blank">VerticalSliceModifierðŸ“˜</a>.

- VerticalSliceModifier visibility

``` prism-code
verticalSliceModifier.showTooltip = true; // Show or hide the tooltip
verticalSliceModifier.showRolloverLine = true; // Show or hide the vertical line
verticalSliceModifier.verticalLine.showLabel = true; // Show or hide the X-Axis label
```

In addition, the line may be made editable (draggable) by setting theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticalslicemodifier.html#isdraggable" rel="noopener noreferrer" target="_blank">isDraggableðŸ“˜</a> property

- VerticalSliceModifier editing

``` prism-code
verticalSliceModifier.isDraggable = true; // Enable or disable dragging of the line
```

The line colour, dash pattern, strokethickness and selection color of theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticalslicemodifier.html" rel="noopener noreferrer" target="_blank">VerticalSliceModifierðŸ“˜</a>Â can also be set. e.g.

| **Property** | **Description** |
|----|----|
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticalslicemodifier.html#rolloverlinestroke" rel="noopener noreferrer" target="_blank">VerticalSliceModifier.rolloverLineStrokeðŸ“˜</a> | Sets the colour of the rollover line as an HTML colour code |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticalslicemodifier.html#rolloverlinestrokedasharray" rel="noopener noreferrer" target="_blank">VerticalSliceModifier.rolloverLineStrokeDashArrayðŸ“˜</a> | Sets the dash pattern (see [Dash Line Styling](https://www.scichart.com/documentation/js/v4/2d-charts/styling-and-theming/dash-line-patterns) for guidelines) |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticalslicemodifier.html#rolloverlinestrokethickness" rel="noopener noreferrer" target="_blank">VerticalSliceModifier.rolloverLineStrokeThicknessðŸ“˜</a> | Sets the rollover line thickness |

- VerticalSliceModifier styling

``` prism-code
verticalSliceModifier.rolloverLineStrokeThickness = 1; // Sets the line thickness
verticalSliceModifier.rolloverLineStroke = "Orange"; // Sets the line colour
verticalSliceModifier.lineSelectionColor = "Red"; // Change the highlight color when selected
```

Properties of the tooltip can be controlled on a per-series basis as per theÂ [RolloverModifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/rollover-modifier) via theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rollovermodifierrenderableseriesprops.html" rel="noopener noreferrer" target="_blank">RenderableSeries.rolloverModifierPropsðŸ“˜</a> property

| **Property** | **Description** |
|----|----|
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rollovermodifierrenderableseriesprops.html#tooltiptextcolor" rel="noopener noreferrer" target="_blank">series.rolloverModifierProps.tooltipTextColorðŸ“˜</a> | The text foreground color of the tooltip, on a per-series basis |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rollovermodifierrenderableseriesprops.html#tooltipcolor" rel="noopener noreferrer" target="_blank">series.rolloverModifierProps.tooltipColorðŸ“˜</a> | The tooltip container color on a per-series basis |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rollovermodifierrenderableseriesprops.html#tooltiplabelx" rel="noopener noreferrer" target="_blank">series.rolloverModifierProps.tooltipLabelXðŸ“˜</a> | Prefix label in the tooltip for X values. Defaults to 'X: ' |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rollovermodifierrenderableseriesprops.html#tooltiplabely" rel="noopener noreferrer" target="_blank">series.rolloverModifierProps.tooltipLabelYðŸ“˜</a> | Prefix label in the tooltip forÂ Y values. Defaults to 'Y: ' |

- VerticalSliceModifier Tooltip styling

``` prism-code
// On a per series basis, you can control the tooltip background/foreground
series.rolloverModifierProps.tooltipTextColor = "White";
series.rolloverModifierProps.tooltipColor = "Red";
series.rolloverModifierProps.tooltipLabelX = "X Value:";
series.rolloverModifierProps.tooltipLabelY = "Y Value:";
```

Finally, the vertical lineÂ itself is simply aÂ [LineAnnotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/line-annotation)Â so all the properties there may be accessed via theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticalslicemodifier.html#verticalline" rel="noopener noreferrer" target="_blank">verticalSliceModifier.verticalLineðŸ“˜</a> property after instantiation.

- VerticalSliceModifier.verticalLine Styling

``` prism-code
// Add some vertical slices to the chart
const vSlice1 = new VerticalSliceModifier({
    // options ...
});
vSlice1.verticalLine.showLabel = true; // Show axis label
vSlice1.verticalLine.axisLabelFill = "#FF6600"; // Style axis label outline and font
vSlice1.verticalLine.axisLabelStroke = "White";
```

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/vertical-slice-modifier/vertical-slice-modifier-overview/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [Formatting VerticalSlice Tooltip Items](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/vertical-slice-modifier/formatting-vertical-slice-tooltip-items)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-modifier-api/vertical-slice-modifier/vertical-slice-modifier-overview/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-modifier-api/vertical-slice-modifier/vertical-slice-modifier-overview/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
