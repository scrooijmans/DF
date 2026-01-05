On this page

# The CursorModifier Type

SciChart.js provides a cursors / crosshairsÂ behavior via theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/cursormodifier.html" rel="noopener noreferrer" target="_blank">CursorModifierðŸ“˜</a>, available out of the box. Besides common features which are inherited from theÂ [ChartModifierBase](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/common-features/) class, theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/cursormodifier.html" rel="noopener noreferrer" target="_blank">CursorModifierðŸ“˜</a> allows toÂ you to:

- **Place a crosshair (cursor) on theÂ chart which tracks theÂ mouse**
- **Place a single aggregated tooltip for all series at the crosshair site**
- Optionally show/hide and style vertical/horizontal line in the crosshair
- Optionally show/hide axis labels on the X,Y axis
- Format the axis labels
- Allow customisation of the tooltip style and contents
- Place a legend at an external `<div>` with tooltip info
- Configure when the tooltip is shown (always, only on hover of a point)
- Configure which series react to the Tooltip (all, some, or specific series)

The <a href="https://www.scichart.com/demo/javascript-chart-cursormodifier-crosshairs" rel="noopener noreferrer" target="_blank">Using CursorModifier Example</a> can be found in theÂ <a href="https://github.com/abtsoftware/scichart.js.examples" rel="noopener noreferrer" target="_blank">SciChart.Js Examples Suite</a>Â on Github, or our live demo atÂ <a href="https://www.scichart.com/demo/javascript-line-chart" rel="noopener noreferrer" target="_blank">scichart.com/demo</a>

## Adding aÂ CursorModifierÂ to a Chart<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/cursor-modifier/cursor-modifier-overview/#adding-acursormodifierto-a-chart" class="hash-link" aria-label="Direct link to Adding aÂ CursorModifierÂ to a Chart" translate="no" title="Direct link to Adding aÂ CursorModifierÂ to a Chart">â€‹</a>

A CursorModifier can be added to theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#chartmodifiers" rel="noopener noreferrer" target="_blank">sciChartSurface.chartModifiersðŸ“˜</a> collection to enable crosshair/cursor behavior. For example, this code adds a crosshair, enables defaultÂ tooltips and axis labels.

- TS
- Builder API (JSON Config)

``` prism-code
const {
    SciChartSurface,
    NumericAxis,
    FastLineRenderableSeries,
    XyDataSeries,
    SciChartJsNavyTheme,
    EAutoRange,
    NumberRange,
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

// For the example to work, axis must have EAutoRange.Always
sciChartSurface.xAxes.add(
    new NumericAxis(wasmContext, {
        autoRange: EAutoRange.Always,
        axisTitle: "X Axis"
    })
);
sciChartSurface.yAxes.add(
    new NumericAxis(wasmContext, {
        visibleRange: new NumberRange(-2, 0.5),
        axisTitle: "Y Axis"
    })
);

// Add a CursorModifier to the chart
const cursorModifier = new CursorModifier({
    // Optional properties to configure what parts are shown
    showTooltip: true,
    showAxisLabels: true,
    showXLine: true,
    showYLine: true,
    // How close to a datapoint to show the tooltip? 10 = 10 pixels. 0 means always
    hitTestRadius: 10,
    // Optional properties to configure the axis labels
    axisLabelFill: "#b36200",
    axisLabelStroke: "#fff",
    // Optional properties to configure line and tooltip style
    crosshairStroke: "#ff6600",
    crosshairStrokeThickness: 1,
    tooltipContainerBackground: "#000",
    tooltipTextStroke: "#ff6600"
});
sciChartSurface.chartModifiers.add(cursorModifier);
```

``` prism-code
// Demonstrates how to configure the CursorModifier in SciChart.js using the Builder API
const { chartBuilder, EThemeProviderType, EAxisType, EChart2DModifierType } = SciChart;
// or, for npm, import { chartBuilder, ... } from "scichart"

const { sciChartSurface, wasmContext } = await chartBuilder.build2DChart(divElementId, {
    surface: { theme: { type: EThemeProviderType.Dark } },
    modifiers: [
        {
            type: EChart2DModifierType.Cursor,
            options: {
                // Optional properties to configure what parts are shown
                showTooltip: true,
                showAxisLabels: true,
                showXLine: true,
                showYLine: true,
                // How close to a datapoint to show the tooltip? 10 = 10 pixels. 0 means always
                hitTestRadius: 10,
                // Optional properties to configure the axis labels
                axisLabelFill: "#b36200",
                axisLabelStroke: "#fff",
                // Optional properties to configure line and tooltip style
                crosshairStroke: "#ff6600",
                crosshairStrokeThickness: 1,
                tooltipContainerBackground: "#000",
                tooltipTextStroke: "#ff6600"
            }
        }
    ]
});
```

This results in the following output:Â 

Many of the properties here are optional - they have been included to show the configuration possibilities for the cursor. SeeÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icursormodifieroptions.html" rel="noopener noreferrer" target="_blank">ICursorModifierOptionsðŸ“˜</a> for more.

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/cursor-modifier/cursor-modifier-overview/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [Formatting CursorModifier Tooltip Items](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/cursor-modifier/formatting-cursor-modifier-tooltip-items)
- [Customizing the CursorModifier Tooltip Container Appearance](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/cursor-modifier/customizing-cursor-modifier-tooltip-container-appearance)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-modifier-api/cursor-modifier/cursor-modifier-overview/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-modifier-api/cursor-modifier/cursor-modifier-overview/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
