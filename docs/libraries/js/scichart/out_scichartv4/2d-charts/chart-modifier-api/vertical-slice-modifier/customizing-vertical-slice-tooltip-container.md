On this page

# Customizing VerticalSliceModifier Tooltip Containers

![](out_scichartv4/2d-charts/chart-modifier-api/vertical-slice-modifier/customizing-vertical-slice-tooltip-container/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

**Background reading:** If you haven't already, read the articleÂ [The VerticalSliceModifier Type](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/vertical-slice-modifier/vertical-slice-modifier-overview)Â which will show you how to setup aÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticalslicemodifier.html" rel="noopener noreferrer" target="_blank">VerticalSliceModifierðŸ“˜</a> with default options for tooltips.

This article goes into further detail on customising the tooltip items (formatting, text content)

## Basic Customization of VerticalSliceModifier Tooltips via properties<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/vertical-slice-modifier/customizing-vertical-slice-tooltip-container/#basic-customization-of-verticalslicemodifier-tooltips-via-properties" class="hash-link" aria-label="Direct link to Basic Customization of VerticalSliceModifier Tooltips via properties" translate="no" title="Direct link to Basic Customization of VerticalSliceModifier Tooltips via properties">â€‹</a>

Basic customisation of theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticalslicemodifier.html" rel="noopener noreferrer" target="_blank">VerticalSliceModifierðŸ“˜</a>Â appearance can be made through the following properties.

- The vertical line thickness, dash and stroke color can be changed with theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticalslicemodifier.html#rolloverlinestroke" rel="noopener noreferrer" target="_blank">VerticalSliceModifier.rolloverLineStrokeðŸ“˜</a>,Â <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticalslicemodifier.html#rolloverlinestrokethickness" rel="noopener noreferrer" target="_blank">VerticalSliceModifier.rolloverLineStrokeThicknessðŸ“˜</a>Â and <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticalslicemodifier.html#rolloverlinestrokedasharray" rel="noopener noreferrer" target="_blank">VerticalSliceModifier.rolloverLineStrokeDashArrayðŸ“˜</a> properties.
- The line selection color can be set via theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticalslicemodifier.html#lineselectioncolor" rel="noopener noreferrer" target="_blank">VerticalSliceModifier.lineSelectionColorðŸ“˜</a> property.
- The tooltip can be turned on/off (to have a simple cursor) with theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticalslicemodifier.html#showtooltip" rel="noopener noreferrer" target="_blank">VerticalSliceModifier.showTooltipðŸ“˜</a> property.
- The tooltip background and text color can be changed withÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rollovermodifierrenderableseriesprops.html#tooltipcolor" rel="noopener noreferrer" target="_blank">tooltipColorðŸ“˜</a> andÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rollovermodifierrenderableseriesprops.html#tooltiptextcolor" rel="noopener noreferrer" target="_blank">tooltipTextColorðŸ“˜</a> properties, found onÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries.html#rollovermodifierprops" rel="noopener noreferrer" target="_blank">RenderableSeries.rolloverModifierPropsðŸ“˜</a> on a per-series basis.
- Axis labels can be turned on/off via theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/lineannotation.html#showlabel" rel="noopener noreferrer" target="_blank">VerticalSliceModifier.verticalLine.showLabelðŸ“˜</a> property, (whereÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticalslicemodifier.html#verticalline" rel="noopener noreferrer" target="_blank">VerticalSliceModifier.verticalLineðŸ“˜</a> is simply aÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/lineannotation.html" rel="noopener noreferrer" target="_blank">LineAnnotationðŸ“˜</a>).
- Axis Label backgrounds and text color can be changed with the properties found on <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticalslicemodifier.html#verticalline" rel="noopener noreferrer" target="_blank">VerticalSliceModifier.verticalLineðŸ“˜</a>, such asÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/cursormodifier.html#axislabelstroke" rel="noopener noreferrer" target="_blank"></a><a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/lineannotation.html#axislabelstroke" rel="noopener noreferrer" target="_blank">axisLabelStrokeðŸ“˜</a>,Â <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/cursormodifier.html#axislabelfill" rel="noopener noreferrer" target="_blank"></a><a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/lineannotation.html#axislabelfill" rel="noopener noreferrer" target="_blank">axisLabelFillðŸ“˜</a>. Â Â 

## Deep Customization of VerticalSliceModifier Tooltips via SVG Templates<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/vertical-slice-modifier/customizing-vertical-slice-tooltip-container/#deep-customization-of-verticalslicemodifier-tooltips-via-svg-templates" class="hash-link" aria-label="Direct link to Deep Customization of VerticalSliceModifier Tooltips via SVG Templates" translate="no" title="Direct link to Deep Customization of VerticalSliceModifier Tooltips via SVG Templates">â€‹</a>

Deeper customisation of the tooltip appearance can be achieved via the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/cursormodifier.html#tooltipsvgtemplate" rel="noopener noreferrer" target="_blank">tooltipSvgTemplateðŸ“˜</a> property.

This defines the actual SVG used to host the tooltip container. This property expects a function in the following format (seeÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tcursortooltipsvgtemplate" rel="noopener noreferrer" target="_blank">TCursorTooltipSvgTemplateðŸ“˜</a>):

- cursotTooltipSvgTemplate function signature

``` prism-code
tooltipSvgTemplate: (
    seriesInfos: SeriesInfo[],
    svgAnnotation: CursorTooltipSvgAnnotation
) => string
```

TheÂ input/output parameters are:

- **Input**: an array ofÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo.html" rel="noopener noreferrer" target="_blank">SeriesInfoðŸ“˜</a>: a data object which stores info about the series under the mouse.
- **Input**: svgAnnotation (<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tcursortooltipsvgtemplate" rel="noopener noreferrer" target="_blank">CursorTooltipSvgAnnotationðŸ“˜</a>) which is the current tooltip SVG annotation
- **Output**:Â string, containing the new SVG to define the tooltip container.

Let's create a simple example which shows you how to customize the tooltip container.

- TS

``` prism-code

const customTooltipTemplate = (
    id, // : string
    seriesInfo, // : SeriesInfo,
    rolloverTooltip // : RolloverTooltipSvgAnnotation
) => {
    const width = 120;
    const height = 120;
    rolloverTooltip.updateSize(width, height);
    return `
    <svg width="${width}" height="${height}">
        <circle cx="50%" cy="50%" r="50%" fill="${seriesInfo.stroke}"/>
        <text y="40" font-size="13" font-family="Verdana" dy="0" fill="${"white"}">
            <tspan x="15" dy="1.2em">${seriesInfo.seriesName}</tspan>
            <tspan x="15" dy="1.2em">x: ${seriesInfo.formattedXValue} y: ${seriesInfo.formattedYValue}</tspan>
        </text>
    </svg>`;
};

lineSeries1.rolloverModifierProps.tooltipTemplate = (
    id, // : string
    seriesInfo, // : SeriesInfo
    rolloverTooltip // : RolloverTooltipSvgAnnotation
) => {
    return customTooltipTemplate(id, seriesInfo, rolloverTooltip);
};

lineSeries2.rolloverModifierProps.tooltipTemplate = (
    id, // : string
    seriesInfo, // : SeriesInfo
    rolloverTooltip // : RolloverTooltipSvgAnnotation
) => {
    return customTooltipTemplate(id, seriesInfo, rolloverTooltip);
};

const getTooltipLegendTemplate = (seriesInfos, svgAnnotation) => {
    let outputSvgString = "";

    // Foreach series there will be a seriesInfo supplied by SciChart. This contains info about the series under the house
    seriesInfos.forEach((seriesInfo, index) => {
        if (seriesInfo.isWithinDataBounds) {
            const lineHeight = 30;
            const y = 50 + index * lineHeight;
            // Use the series stroke for legend text colour
            const textColor = seriesInfo.stroke;
            // Use the seriesInfo formattedX/YValue for text on the
            outputSvgString += `<text x="8" y="${y}" font-size="16" font-family="Verdana" fill="${textColor}">
                                ${seriesInfo.seriesName}: X=${seriesInfo.formattedXValue}, Y=${seriesInfo.formattedYValue}
                            </text>`;
        }
    });

    // Content here is returned for the custom legend placed in top-left of the chart
    return `<svg width="100%" height="100%">
            <text x="8" y="20" font-size="15" font-family="Verdana" fill="lightblue">Custom Rollover Legend</text>
            ${outputSvgString}
        </svg>`;
};

const vSlice1 = new VerticalSliceModifier({
    x1: 5.06,
    xCoordinateMode: ECoordinateMode.DataValue,
    isDraggable: true,
    // Defines if rollover vertical line is shown
    showRolloverLine: true,
    rolloverLineStrokeThickness: 1,
    rolloverLineStroke: "green",
    lineSelectionColor: "green",
    // Shows the default tooltip
    showTooltip: true,
    // Optional: Overrides the legend template to display additional info top-left of the chart
    tooltipLegendTemplate: getTooltipLegendTemplate
});

const vSlice2 = new VerticalSliceModifier({
    x1: 0.75,
    xCoordinateMode: ECoordinateMode.Relative,
    isDraggable: true,
    // Defines if rollover vertical line is shown
    showRolloverLine: true,
    rolloverLineStrokeThickness: 1,
    rolloverLineStroke: "orange",
    lineSelectionColor: "orange",
    // Shows the default tooltip
    showTooltip: true
});

sciChartSurface.chartModifiers.add(vSlice1, vSlice2);
```

This results in the following output:

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-modifier-api/vertical-slice-modifier/customizing-vertical-slice-tooltip-container/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-modifier-api/vertical-slice-modifier/customizing-vertical-slice-tooltip-container/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
