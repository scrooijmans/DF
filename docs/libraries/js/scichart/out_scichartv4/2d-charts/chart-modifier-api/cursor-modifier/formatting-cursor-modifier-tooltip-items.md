On this page

# Formatting CursorModifier Tooltip Items

![](out_scichartv4/2d-charts/chart-modifier-api/cursor-modifier/formatting-cursor-modifier-tooltip-items/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

Background reading:

If you haven't already, read the articleÂ [The CursorModifier Type](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/cursor-modifier/cursor-modifier-overview)Â which will show you how to setup a **CursorModifier** with default options for tooltips. This article goes into further detail on customising the tooltip items (formatting, text content)

## Basic CursorModifier Tooltip Formatting Options<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/cursor-modifier/formatting-cursor-modifier-tooltip-items/#basic-cursormodifier-tooltip-formatting-options" class="hash-link" aria-label="Direct link to Basic CursorModifier Tooltip Formatting Options" translate="no" title="Direct link to Basic CursorModifier Tooltip Formatting Options">â€‹</a>

Tooltip and Axis Label formatting comes from theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/labelprovider.html#formatcursorlabel" rel="noopener noreferrer" target="_blank">axis.labelprovider.formatCursorLabel()ðŸ“˜</a> functionÂ and is axis-specific. You can read more about theÂ [Axis.LabelProvider API here](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-labels/label-provider-api-overview), including how to specify formats from Enums and override formatting programmatically.

Below we're going to show you how to apply cursor formatting to enable four-decimal places on tooltips.

- TS
- Builder API (JSON Config)

``` prism-code
const {
    SciChartSurface,
    NumericAxis,
    FastLineRenderableSeries,
    XyDataSeries,
    SciChartJsNavyTheme,
    ENumericFormat,
    CursorModifier,
    TextAnnotation,
    EHorizontalAnchorPoint,
    ECoordinateMode,
    EllipsePointMarker
} = SciChart;
// or for npm import { SciChartSurface, ... } from "scichart"

// Create a chart surface
const { sciChartSurface, wasmContext } = await SciChartSurface.create(divElementId, {
    theme: new SciChartJsNavyTheme(),
    titleStyle: { fontSize: 16 }
});

sciChartSurface.xAxes.add(
    new NumericAxis(wasmContext, {
        // label format options applied to the X-Axis
        labelPrecision: 2,
        labelFormat: ENumericFormat.Decimal,
        // label format options applied to cursors & tooltips
        cursorLabelPrecision: 4,
        cursorLabelFormat: ENumericFormat.Decimal
    })
);
sciChartSurface.yAxes.add(
    new NumericAxis(wasmContext, {
        // label format options applied to the X-Axis
        labelPrecision: 1,
        labelFormat: ENumericFormat.Decimal,
        // label format options applied to cursors & tooltips
        cursorLabelPrecision: 4,
        cursorLabelFormat: ENumericFormat.Decimal
    })
);

// Add a CursorModifier to the chart
const cursorModifier = new CursorModifier({
    showTooltip: true,
    showAxisLabels: true,
    hitTestRadius: 10
});
sciChartSurface.chartModifiers.add(cursorModifier);
```

``` prism-code
const { chartBuilder, EThemeProviderType, EAxisType, EChart2DModifierType, ENumericFormat } = SciChart;
// or, for npm, import { chartBuilder, ... } from "scichart"

const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    surface: { theme: { type: EThemeProviderType.Dark } },
    xAxes: {
        type: EAxisType.NumericAxis,
        options: {
            // label format options applied to the X-Axis
            labelPrecision: 2,
            labelFormat: ENumericFormat.Decimal,
            // label format options applied to cursors & tooltips
            cursorLabelPrecision: 4,
            cursorLabelFormat: ENumericFormat.Decimal
        }
    },
    yAxes: {
        type: EAxisType.NumericAxis,
        options: {
            // label format options applied to the X-Axis
            labelPrecision: 2,
            labelFormat: ENumericFormat.Decimal,
            // label format options applied to cursors & tooltips
            cursorLabelPrecision: 4,
            cursorLabelFormat: ENumericFormat.Decimal
        }
    },
    modifiers: [
        {
            type: EChart2DModifierType.Cursor,
            options: {
                showTooltip: true,
                showAxisLabels: true,
                hitTestRadius: 10
            }
        }
    ]
});
```

Here's a **Codepen** which shows the effect of these properties on the axis on cursor tooltips:

For further customisation on a per-axis basis, consider using theÂ [LabelProvider feature](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-labels/custom-label-providers-readable-numbers) to create a custom labelprovider, and override formatCursorLabel.

## Tooltip DataTemplates<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/cursor-modifier/formatting-cursor-modifier-tooltip-items/#tooltip-datatemplates" class="hash-link" aria-label="Direct link to Tooltip DataTemplates" translate="no" title="Direct link to Tooltip DataTemplates">â€‹</a>

Further customisation of tooltip content can be achieved with the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/cursormodifier.html#tooltipdatatemplate" rel="noopener noreferrer" target="_blank">CursorModifier.tooltipDataTemplateðŸ“˜</a> property. This defines the content inside the tooltip e.g. what values are shown (x, y, values from metadata), if the series name is shown and so on.

This property expects a function in the following format (seeÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tcursortooltipdatatemplate" rel="noopener noreferrer" target="_blank">TCursorTooltipDataTemplateðŸ“˜</a>):

- tooltipDataTemplate function signature

``` prism-code
tooltipDataTemplate: (
    seriesInfos: SeriesInfo[],
    tooltipTitle: string
) => string[];
```

TheÂ input/output parameters are:

- **Input**: an array ofÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo.html" rel="noopener noreferrer" target="_blank">SeriesInfoðŸ“˜</a>: a data object which stores info about the series under the mouse.
- **Input**: a tooltipTitle (string) which comes fromÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rollovermodifierrenderableseriesprops.html#tooltiptitle" rel="noopener noreferrer" target="_blank">renderableSeries.rolloverModifierProps.tooltipTitleðŸ“˜</a>.
- **Output**:Â an array of strings, each one corresponding to a line in the tooltip.

Let's create a simple example which shows you how to access properties onÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyseriesinfo.html" rel="noopener noreferrer" target="_blank">XySeriesInfoðŸ“˜</a> and output to tooltips.

- TS
- Builder API (JSON Config)

``` prism-code
// Create a chart surface
const { sciChartSurface, wasmContext } = await SciChartSurface.create(divElementId, {
    theme: new SciChartJsNavyTheme(),
    titleStyle: { fontSize: 16 }
});

sciChartSurface.xAxes.add(new NumericAxis(wasmContext));
sciChartSurface.yAxes.add(new NumericAxis(wasmContext));

// Add a CursorModifier to the chart
const cursorModifier = new CursorModifier({
    showTooltip: true,
    showAxisLabels: true,
    hitTestRadius: 10,
    // Add a custom tooltip data template
    tooltipDataTemplate: (seriesInfos, tooltipTitle) => {
        // each element in this array = 1 line in the tooltip
        const lineItems = [];
        // See SeriesInfo docs at https://www.scichart.com/documentation/js/v4/typedoc/classes/xyseriesinfo.html
        seriesInfos.forEach(si => {
            // If hit (within hitTestRadius of point)
            if (si.isHit) {
                // SeriesInfo.seriesName comes from dataSeries.dataSeriesName
                lineItems.push(`${si.seriesName}`);
                // seriesInfo.xValue, yValue are available to be formatted
                // Or, preformatted values are available as si.formattedXValue, si.formattedYValue
                lineItems.push(`X: ${si.xValue.toFixed(2)}`);
                lineItems.push(`Y: ${si.yValue.toFixed(2)}`);
                // index to the dataseries is available
                lineItems.push(`Index: ${si.dataSeriesIndex}`);
                // Which can be used to get anything from the dataseries
                lineItems.push(
                    `Y-value from dataSeries: ${si.renderableSeries.dataSeries
                        .getNativeYValues()
                        .get(si.dataSeriesIndex)
                        .toFixed(4)}`
                );
                // Location of the hit in pixels is available
                lineItems.push(`Location: ${si.xCoordinate.toFixed(0)}, ${si.yCoordinate.toFixed(0)}`);
            }
        });

        return lineItems;
    }
});
sciChartSurface.chartModifiers.add(cursorModifier);
```

``` prism-code
// Demonstrates how to configure the PinchZoomModifier in SciChart.js using the Builder API
const { chartBuilder, EThemeProviderType, EAxisType, EChart2DModifierType } = SciChart;
// or, for npm, import { chartBuilder, ... } from "scichart"

const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    surface: { theme: { type: EThemeProviderType.Dark } },
    modifiers: [
        {
            type: EChart2DModifierType.Cursor,
            options: {
                showTooltip: true,
                showAxisLabels: true,
                hitTestRadius: 10,
                // Add a custom tooltip data template
                tooltipDataTemplate: (seriesInfos, tooltipTitle) => {
                    // each element in this array = 1 line in the tooltip
                    const lineItems = [];
                    // See SeriesInfo docs at https://www.scichart.com/documentation/js/v4/typedoc/classes/xyseriesinfo.html
                    seriesInfos.forEach(si => {
                        // If hit (within hitTestRadius of point)
                        if (si.isHit) {
                            // SeriesInfo.seriesName comes from dataSeries.dataSeriesName
                            lineItems.push(`${si.seriesName}`);
                            // seriesInfo.xValue, yValue are available to be formatted
                            // Or, preformatted values are available as si.formattedXValue, si.formattedYValue
                            lineItems.push(`X: ${si.xValue.toFixed(2)}`);
                            lineItems.push(`Y: ${si.yValue.toFixed(2)}`);
                            // index to the dataseries is available
                            lineItems.push(`Index: ${si.dataSeriesIndex}`);
                            // Which can be used to get anything from the dataseries
                            lineItems.push(
                                `Y-value from dataSeries: ${si.renderableSeries.dataSeries
                                    .getNativeYValues()
                                    .get(si.dataSeriesIndex)
                                    .toFixed(4)}`
                            );
                            // Location of the hit in pixels is available
                            lineItems.push(`Location: ${si.xCoordinate.toFixed(0)}, ${si.yCoordinate.toFixed(0)}`);
                        }
                    });

                    return lineItems;
                }
            }
        }
    ]
});
```

This results in the following output

## Accessing Metadata in Tooltip DataTemplates<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/cursor-modifier/formatting-cursor-modifier-tooltip-items/#accessing-metadata-in-tooltip-datatemplates" class="hash-link" aria-label="Direct link to Accessing Metadata in Tooltip DataTemplates" translate="no" title="Direct link to Accessing Metadata in Tooltip DataTemplates">â€‹</a>

In the above example we access properties ofÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyseriesinfo.html" rel="noopener noreferrer" target="_blank">XySeriesInfoðŸ“˜</a> to format lines in the CursorModifier tooltip.

You can also access metadata to store any custom object in your X,Y data, then read that data out in tooltips.

For a worked example seeÂ [PointMetadata API - Metadata and Tooltips](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/point-metadata-api/tooltips).

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/cursor-modifier/formatting-cursor-modifier-tooltip-items/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [Customizing the CursorModifier Tooltip Container Appearance](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/cursor-modifier/customizing-cursor-modifier-tooltip-container-appearance)
- [Active Legends - CursorModifier output into a legend](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/cursor-modifier/active-legends-cursor-modifier)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-modifier-api/cursor-modifier/formatting-cursor-modifier-tooltip-items/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-modifier-api/cursor-modifier/formatting-cursor-modifier-tooltip-items/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
