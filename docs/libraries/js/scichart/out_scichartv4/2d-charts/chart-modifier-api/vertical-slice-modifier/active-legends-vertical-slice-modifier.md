On this page

# Active Legends - VerticalSliceModifier output to a Legend

![](out_scichartv4/2d-charts/chart-modifier-api/vertical-slice-modifier/active-legends-vertical-slice-modifier/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

**Background reading:** If you haven't already, read the articleÂ [The VerticalSliceModifier Type](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/vertical-slice-modifier/vertical-slice-modifier-overview)Â which will show you how to setup aÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticalslicemodifier.html" rel="noopener noreferrer" target="_blank">VerticalSliceModifierðŸ“˜</a> with default options for tooltips.

This article goes into further detail on customising the tooltip items (formatting, text content)

## VerticalSliceModifier tooltipLegendTemplates<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/vertical-slice-modifier/active-legends-vertical-slice-modifier/#verticalslicemodifier-tooltiplegendtemplates" class="hash-link" aria-label="Direct link to VerticalSliceModifier tooltipLegendTemplates" translate="no" title="Direct link to VerticalSliceModifier tooltipLegendTemplates">â€‹</a>

TheÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticalslicemodifier.html" rel="noopener noreferrer" target="_blank">VerticalSliceModifierðŸ“˜</a>Â supports aÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticalslicemodifier.html#tooltiplegendtemplate" rel="noopener noreferrer" target="_blank">tooltipLegendTemplateðŸ“˜</a> property which allows you to specify a function to transformÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticalslicemodifier.html" rel="noopener noreferrer" target="_blank">VerticalSliceModifierðŸ“˜</a>Â content into a legend which can be placed in the top left of the chart. This active legend updates with series values as you drag the vertical lines, or when a series updates.

TheÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticalslicemodifier.html#tooltiplegendtemplate" rel="noopener noreferrer" target="_blank">tooltipLegendTemplateðŸ“˜</a>Â property expects a function in the following format (seeÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#trolloverlegendsvgtemplate" rel="noopener noreferrer" target="_blank">TRolloverLegendSvgTemplateðŸ“˜</a>):

- tooltipLegendTemplate function signature

``` prism-code
TRolloverLegendSvgTemplate: (
    seriesInfos: SeriesInfo[],
    svgAnnotation: RolloverLegendSvgAnnotation
) => string
```

TheÂ input/output parameters are:

| **In/Out** | **Parameter** | **Description** |
|----|----|----|
| *Input* | seriesInfos | an array ofÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo.html" rel="noopener noreferrer" target="_blank">SeriesInfoðŸ“˜</a>: a data object which stores info about the series that intersects the Vertical Line |
| *Input* | svgAnnotation | TheÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rolloverlegendsvgannotation.html" rel="noopener noreferrer" target="_blank">RolloverLegendSvgAnnotationðŸ“˜</a> that will be used to render the legend. From here you can access properties of the underlying legend container, such asÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rolloverlegendsvgannotation.html#tooltiplegendoffsetx" rel="noopener noreferrer" target="_blank">tooltipLegendOffsetXðŸ“˜</a> / Y orÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rolloverlegendsvgannotation.html#tooltiplegendtemplate" rel="noopener noreferrer" target="_blank">tooltipLegendTemplateðŸ“˜</a> |
| *Return* | string\[\] | A string containing the result SVG to display inside theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rolloverlegendsvgannotation.html" rel="noopener noreferrer" target="_blank">RolloverLegendSvgAnnotationðŸ“˜</a> |

Let's create a simple example which shows you how to access properties onÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyseriesinfo.html" rel="noopener noreferrer" target="_blank">XySeriesInfoðŸ“˜</a> and output to a custom legend.

Here's a worked example below, which shows how to place the hit-test result from a vertical line into an active legend elsewhere in your application.

- TS

``` prism-code
// Create a tooltip legend template
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
            <text x="8" y="20" font-size="15" font-family="Verdana" fill="lightblue">Custom VerticalSlice Legend</text>
            ${outputSvgString}
        </svg>`;
};

// Apply it to a VerticalSliceModifier
const vSlice = new VerticalSliceModifier({
    x1: 30.0,
    xCoordinateMode: ECoordinateMode.DataValue,
    isDraggable: true,
    showRolloverLine: true,
    rolloverLineStrokeThickness: 1,
    rolloverLineStroke: "#50C7E0",
    lineSelectionColor: "#50C7E0",
    showTooltip: true,
    // Optional: Overrides the legend template to display additional info top-left of the chart
    tooltipLegendTemplate: getTooltipLegendTemplate
});
sciChartSurface.chartModifiers.add(vSlice);
```

This results in the following output:

## Using External placementDivId with the VerticalSliceModifier<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/vertical-slice-modifier/active-legends-vertical-slice-modifier/#using-external-placementdivid-with-the-verticalslicemodifier" class="hash-link" aria-label="Direct link to Using External placementDivId with the VerticalSliceModifier" translate="no" title="Direct link to Using External placementDivId with the VerticalSliceModifier">â€‹</a>

Another way you can control the placement of theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticalslicemodifier.html" rel="noopener noreferrer" target="_blank">VerticalSliceModifierðŸ“˜</a>Â tooltip is using theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticalslicemodifier.html#placementdivid" rel="noopener noreferrer" target="_blank">placementDivIdðŸ“˜</a> property. This places the standard VerticalSliceModifier tooltip into a div of your choice (which can be anywhere on the app).

Note, it does not currently work withÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticalslicemodifier.html#tooltiplegendtemplate" rel="noopener noreferrer" target="_blank">tooltipLegendTemplateðŸ“˜</a>, however we are working on more options for styling, placement and configuration of tooltips soon.

Try the following code in your application:

- TS

``` prism-code
// Create a VerticalSliceModifier and add to the chart
const vSlice = new VerticalSliceModifier({
    x1: 30.0,
    xCoordinateMode: ECoordinateMode.DataValue,
    isDraggable: true,
    showRolloverLine: true,
    rolloverLineStrokeThickness: 1,
    rolloverLineStroke: "#50C7E0",
    lineSelectionColor: "#50C7E0",
    showTooltip: true,
    // Optional: Places the tooltip output in a div with id="legend-root"
    placementDivId: "legend-root"
});
sciChartSurface.chartModifiers.add(vSlice);
```

- HTML

``` prism-code
<body>
  <div id="container">
    <div id="scichart-root"></div>
    <div id="legend-root"></div>
  </div>
</body>
```

- CSS

``` prism-code
body {
  margin: 0;
}
#container {
  width: 100%;
  height: 100vh;
  position: relative;
}
#scichart-root {
  width: 100%;
  height: 100%;
  position: relative;
}
#legend-root {
  position: absolute;
  left: 20px;
  top: 20px;
}
```

This results in the following output.

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-modifier-api/vertical-slice-modifier/active-legends-vertical-slice-modifier/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-modifier-api/vertical-slice-modifier/active-legends-vertical-slice-modifier/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
