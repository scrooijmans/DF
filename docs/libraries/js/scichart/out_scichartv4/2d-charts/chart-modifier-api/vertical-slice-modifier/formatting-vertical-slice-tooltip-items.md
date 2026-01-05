On this page

# Formatting VerticalSlice Tooltip Items

![](out_scichartv4/2d-charts/chart-modifier-api/vertical-slice-modifier/formatting-vertical-slice-tooltip-items/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

**Background reading:** If you haven't already, read the articleÂ [The VerticalSliceModifier Type](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/vertical-slice-modifier/vertical-slice-modifier-overview)Â which will show you how to setup aÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticalslicemodifier.html" rel="noopener noreferrer" target="_blank">VerticalSliceModifierðŸ“˜</a> with default options for tooltips.

This article goes into further detail on customising the tooltip items (formatting, text content)

## Basic VerticalSliceModifier Tooltip Formatting Options<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/vertical-slice-modifier/formatting-vertical-slice-tooltip-items/#basic-verticalslicemodifier-tooltip-formatting-options" class="hash-link" aria-label="Direct link to Basic VerticalSliceModifier Tooltip Formatting Options" translate="no" title="Direct link to Basic VerticalSliceModifier Tooltip Formatting Options">â€‹</a>

TheÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticalslicemodifier.html" rel="noopener noreferrer" target="_blank">VerticalSliceModifierðŸ“˜</a> obeys similar rules to theÂ [CursorModifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/cursor-modifier/cursor-modifier-overview) andÂ [RolloverModifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/rollover-modifier) for customizing the tooltip content and appearance.

Tooltip and Axis Label formatting comes from theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/labelprovider.html#formatcursorlabel" rel="noopener noreferrer" target="_blank">axis.labelprovider.formatCursorLabel()ðŸ“˜</a> functionÂ and is axis-specific. You can read more about theÂ [Axis.LabelProvider API here](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-labels/label-provider-api-overview), including how to specify formats from Enums and override formatting programmatically.

Below we're going to show you how to apply tooltip formatting to enable four-decimal places onÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticalslicemodifier.html" rel="noopener noreferrer" target="_blank">VerticalSliceModifierðŸ“˜</a> tooltips.

- TS
- Builder API (JSON Config)

``` prism-code
// Create a chart surface
const { sciChartSurface, wasmContext } = await SciChartSurface.create(divElementId, {
    theme: new SciChartJsNavyTheme(),
    titleStyle: { fontSize: 16 }
});

// For the example to work, axis must have EAutoRange.Always
sciChartSurface.xAxes.add(
    new NumericAxis(wasmContext, {
        axisTitle: "X Axis",
        // label format options applied to the X-Axis
        labelPrecision: 1,
        labelFormat: ENumericFormat.Decimal,
        // label format options applied to cursors & tooltips
        cursorLabelPrecision: 2,
        cursorLabelFormat: ENumericFormat.Decimal
    })
);
sciChartSurface.yAxes.add(
    new NumericAxis(wasmContext, {
        visibleRange: new NumberRange(-2, 0.5),
        axisTitle: "Y Axis",
        // label format options applied to the X-Axis
        labelPrecision: 1,
        labelFormat: ENumericFormat.Decimal,
        // label format options applied to cursors & tooltips
        cursorLabelPrecision: 6,
        cursorLabelFormat: ENumericFormat.Decimal
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
vSlice1.verticalLine.showLabel = true;
vSlice1.verticalLine.axisLabelFill = "#FF6600";
vSlice1.verticalLine.axisLabelStroke = "White";
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
vSlice2.verticalLine.showLabel = true;
vSlice2.verticalLine.axisLabelFill = "#50C7E0";
vSlice2.verticalLine.axisLabelStroke = "White";
sciChartSurface.chartModifiers.add(vSlice1, vSlice2);
```

``` prism-code
// Demonstrates how to configure the PinchZoomModifier in SciChart.js using the Builder API
const {
    chartBuilder,
    EThemeProviderType,
    ECoordinateMode,
    EChart2DModifierType,
    ENumericFormat,
    EAxisType,
    NumberRange
} = SciChart;
// or, for npm, import { chartBuilder, ... } from "scichart"

const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    surface: { theme: { type: EThemeProviderType.Dark } },
    xAxes: {
        type: EAxisType.NumericAxis,
        options: {
            axisTitle: "X Axis",
            // label format options applied to the X-Axis
            labelPrecision: 1,
            labelFormat: ENumericFormat.Decimal,
            // label format options applied to cursors & tooltips
            cursorLabelPrecision: 2,
            cursorLabelFormat: ENumericFormat.Decimal
        }
    },
    yAxes: {
        type: EAxisType.NumericAxis,
        options: {
            visibleRange: new NumberRange(-2, 0.5),
            axisTitle: "Y Axis",
            // label format options applied to the X-Axis
            labelPrecision: 1,
            labelFormat: ENumericFormat.Decimal,
            // label format options applied to cursors & tooltips
            cursorLabelPrecision: 6,
            cursorLabelFormat: ENumericFormat.Decimal
        }
    },
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

Here's a **Codepen** which shows the effect of these properties on the axis on cursor tooltips.

For further customisation on a per-axis basis, consider using theÂ [LabelProvider feature](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-labels/label-provider-api-overview) to create a custom labelprovider, and override formatCursorLabel.

## Tooltip DataTemplates<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/vertical-slice-modifier/formatting-vertical-slice-tooltip-items/#tooltip-datatemplates" class="hash-link" aria-label="Direct link to Tooltip DataTemplates" translate="no" title="Direct link to Tooltip DataTemplates">â€‹</a>

Further customisation ofÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticalslicemodifier.html" rel="noopener noreferrer" target="_blank">VerticalSliceModifierðŸ“˜</a> tooltip content can be achieved with the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticalslicemodifier.html#tooltipdatatemplate" rel="noopener noreferrer" target="_blank">VerticalSliceModifier.tooltipDataTemplateðŸ“˜</a> property. This defines the content inside the tooltip e.g. what values are shown (x, y, values from metadata), if the series name is shown and so on.

This property expects a function in the following format (seeÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#trollovertooltipdatatemplate" rel="noopener noreferrer" target="_blank">TRolloverTooltipDataTemplateðŸ“˜</a>):

- tooltipDataTemplateFunction

``` prism-code
TRolloverTooltipDataTemplate: (
    seriesInfo: SeriesInfo,
    tooltipTitle: string,
    tooltipLabelX: string,
    tooltipLabelY: string
) => string[]
```

TheÂ input/output parameters are:

| **In/Out** | **Parameter** | **Description** |
|----|----|----|
| *Input* | seriesInfo | anÂ instance ofÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo.html" rel="noopener noreferrer" target="_blank">SeriesInfoðŸ“˜</a>: a data object which stores info about the series that intersects the Vertical Line |
| *Input* | tooltipTitle | a tooltipTitle (string) which comes fromÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rollovermodifierrenderableseriesprops.html#tooltiptitle" rel="noopener noreferrer" target="_blank">renderableSeries.rolloverModifierProps.tooltipTitleðŸ“˜</a>. |
| *Input* | tooltipLabelX | A prefix (string) which comes fromÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rollovermodifierrenderableseriesprops.html#tooltiplabelx" rel="noopener noreferrer" target="_blank">renderableSeries.rolloverModifierProps.tooltipLabelXðŸ“˜</a> |
| *Input* | tooltipLabelY | A prefix (string) which comes fromÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rollovermodifierrenderableseriesprops.html#tooltiplabely" rel="noopener noreferrer" target="_blank">renderableSeries.rolloverModifierProps.tooltipLabelYðŸ“˜</a> |
| *Return* | string\[\] | An array of strings, each one corresponding to a line in the tooltip |

Let's create a simple example which shows you how to access properties onÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyseriesinfo.html" rel="noopener noreferrer" target="_blank">XySeriesInfoðŸ“˜</a> and output to tooltips.

- TS
- Builder API (JSON Config)

``` prism-code
// Add a custom tooltip data template
const tooltipDataTemplate = (seriesInfo, tooltipTitle, tooltipLabelX, tooltipLabelY) => {
    // each element in this array = 1 line in the tooltip
    const lineItems = [];
    // See SeriesInfo docs at https://www.scichart.com/documentation/js/v4/typedoc/classes/xyseriesinfo.html
    // SeriesInfo.seriesName comes from dataSeries.dataSeriesName
    lineItems.push(`${seriesInfo.seriesName}`);
    // seriesInfo.xValue, yValue are available to be formatted
    // Or, preformatted values are available as si.formattedXValue, si.formattedYValue
    lineItems.push(`X: ${seriesInfo.xValue.toFixed(2)}`);
    lineItems.push(`Y: ${seriesInfo.yValue.toFixed(2)}`);
    // index to the dataseries is available
    lineItems.push(`Index: ${seriesInfo.dataSeriesIndex}`);
    // Which can be used to get anything from the dataseries
    lineItems.push(
        `Y-value from dataSeries: ${seriesInfo.renderableSeries.dataSeries
            .getNativeYValues()
            .get(seriesInfo.dataSeriesIndex)
            .toFixed(4)}`
    );
    // Location of the hit in pixels is available
    lineItems.push(`Location: ${seriesInfo.xCoordinate.toFixed(0)}, ${seriesInfo.yCoordinate.toFixed(0)}`);
    return lineItems;
};

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
    showTooltip: true,
    // The tooltip data template
    tooltipDataTemplate
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
    showTooltip: true,
    // The tooltip data template
    tooltipDataTemplate
});
sciChartSurface.chartModifiers.add(vSlice1, vSlice2);
```

``` prism-code
// Demonstrates how to configure the PinchZoomModifier in SciChart.js using the Builder API
const {
    chartBuilder,
    EThemeProviderType,
    ECoordinateMode,
    EChart2DModifierType,
} = SciChart;
// or, for npm, import { chartBuilder, ... } from "scichart"

// Add a custom tooltip data template
const tooltipDataTemplate = (seriesInfo, tooltipTitle, tooltipLabelX, tooltipLabelY) => {
    // each element in this array = 1 line in the tooltip
    const lineItems = [];
    // See SeriesInfo docs at https://www.scichart.com/documentation/js/v4/typedoc/classes/xyseriesinfo.html
    // SeriesInfo.seriesName comes from dataSeries.dataSeriesName
    lineItems.push(`${seriesInfo.seriesName}`);
    // seriesInfo.xValue, yValue are available to be formatted
    // Or, preformatted values are available as si.formattedXValue, si.formattedYValue
    lineItems.push(`X: ${seriesInfo.xValue.toFixed(2)}`);
    lineItems.push(`Y: ${seriesInfo.yValue.toFixed(2)}`);
    // index to the dataseries is available
    lineItems.push(`Index: ${seriesInfo.dataSeriesIndex}`);
    // Which can be used to get anything from the dataseries
    lineItems.push(
        `Y-value from dataSeries: ${seriesInfo.renderableSeries.dataSeries
            .getNativeYValues()
            .get(seriesInfo.dataSeriesIndex)
            .toFixed(4)}`
    );
    // Location of the hit in pixels is available
    lineItems.push(`Location: ${seriesInfo.xCoordinate.toFixed(0)}, ${seriesInfo.yCoordinate.toFixed(0)}`);
    return lineItems;
};

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
                showTooltip: true,
                // Add the tooltip data template
                tooltipDataTemplate
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
                showTooltip: true,
                // Add the tooltip data template
                tooltipDataTemplate
            }
        }
    ]
});
```

This results in the following output

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-modifier-api/vertical-slice-modifier/formatting-vertical-slice-tooltip-items/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-modifier-api/vertical-slice-modifier/formatting-vertical-slice-tooltip-items/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
