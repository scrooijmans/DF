On this page

# Customizing the CursorModifier Tooltip Container Appearance

![](out_scichartv4/2d-charts/chart-modifier-api/cursor-modifier/customizing-cursor-modifier-tooltip-container-appearance/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

Background reading:

If you haven't already, read the articleÂ [The CursorModifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/cursor-modifier/cursor-modifier-overview) typeÂ which will show you how to setup a CursorModifier with default options for tooltips. This article goes into further detail on customising the tooltip items (formatting, text content)

Basic customisation of the cursor and tooltip appearance can be made through the following properties.

- The crosshair line thickness and stroke color can be changed with theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/cursormodifier.html#crosshairstrokethickness" rel="noopener noreferrer" target="_blank">crosshairStrokeThicknessðŸ“˜</a> andÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/cursormodifier.html#crosshairstroke" rel="noopener noreferrer" target="_blank">crosshairStrokeðŸ“˜</a> properties.
- Axis labels can be turned on/off via theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/cursormodifier.html#showaxislabels" rel="noopener noreferrer" target="_blank">showAxisLabelsðŸ“˜</a> property.
- The tooltip can be turned on/off (to have a simple cursor) with theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/cursormodifier.html#showtooltip" rel="noopener noreferrer" target="_blank">showTooltipðŸ“˜</a> property.
- Axis Label backgrounds and text color can be changed with theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/cursormodifier.html#axislabelstroke" rel="noopener noreferrer" target="_blank">axisLabelStrokeðŸ“˜</a>,Â <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/cursormodifier.html#axislabelfill" rel="noopener noreferrer" target="_blank">axisLabelFillðŸ“˜</a> properties.
- The tooltip background and text color can be changed with theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/cursormodifier.html#tooltipcontainerbackground" rel="noopener noreferrer" target="_blank">tooltipContainerBackgroundðŸ“˜</a> andÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/cursormodifier.html#tooltiptextstroke" rel="noopener noreferrer" target="_blank">tooltipTextStrokeðŸ“˜</a> properties.

Deeper customisation of the tooltip appearance can be achieved via the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/cursormodifier.html#tooltipsvgtemplate" rel="noopener noreferrer" target="_blank">tooltipSvgTemplateðŸ“˜</a> property.

This defines the actual SVG used to host the tooltip container. This property expects a function in the following format (seeÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tcursortooltipsvgtemplate" rel="noopener noreferrer" target="_blank">TCursorTooltipSvgTemplateðŸ“˜</a>):

- cursorTooltipSvgTemplate function signature

``` prism-code
    cursorTooltipSvgTemplate: (
        seriesInfos: SeriesInfo[],
        svgAnnotation: CursorTooltipSvgAnnotation
    ) => string
```

Where theÂ input/output parameters are:

- **Input**: an array ofÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo.html" rel="noopener noreferrer" target="_blank">SeriesInfoðŸ“˜</a>: a data object which stores info about the series under the mouse.
- **Input**: svgAnnotation (<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tcursortooltipsvgtemplate" rel="noopener noreferrer" target="_blank">CursorTooltipSvgAnnotationðŸ“˜</a>) which is the current tooltip SVG annotation
- **Output**:Â string, containing the new SVG to define the tooltip container.

Let's create a simple example which shows you how to customize the tooltip container.

- TS

``` prism-code
const tooltipSvgTemplate = (seriesInfos, svgAnnotation) => {
    const width = 120;
    const height = 120;
    const seriesInfo = seriesInfos[0];

    if (!seriesInfo?.isWithinDataBounds) {
        return "<svg></svg>";
    }

    let seriesName;
    let xValue;
    let yValue;
    let index;
    let yValueFromDS;

    seriesInfos.forEach(si => {
        // If hit (within hitTestRadius of point)
        if (si.isHit) {
            // SeriesInfo.seriesName comes from dataSeries.dataSeriesName
            seriesName = si.seriesName;
            // seriesInfo.xValue, yValue
            xValue = si.xValue.toFixed(2);

            yValue = si.yValue.toFixed(2);

            // index to the dataseries is available
            index = si.dataSeriesIndex;

            // Which can be used to get anything from the dataseries
            yValueFromDS = si.renderableSeries.dataSeries.getNativeYValues().get(si.dataSeriesIndex).toFixed(4);
        }
    });

    if (!seriesName) {
        return "<svg></svg>";
    }

    const x = seriesInfo ? seriesInfo.formattedXValue : "";
    const y = seriesInfo ? seriesInfo.formattedYValue : "";

    // this calculates and sets svgAnnotation.xCoordShift and svgAnnotation.yCoordShift. Do not set x1 or y1 at this point.
    adjustTooltipPosition(width, height, svgAnnotation);

    return `
        <svg width="${width}" height="${height}">
            <rect x="0" y="0" rx="${seriesName === "Sinewave 1" ? "10" : "100"}" ry="${seriesName === "Sinewave 1" ? "10" : "100"}" width="${width}" height="${height}" fill="${seriesName === "Sinewave 1" ? "#FF6600" : "#50C7E0"}"/>
            <text y="35" font-family="Verdana" font-size="12" fill="white">
                <tspan x="50%" text-anchor="middle" font-size="14">${seriesName}</tspan>
                <tspan x="50%" text-anchor="middle" dy="2.4em">x: ${xValue}</tspan>
                <tspan x="50%" text-anchor="middle" dy="1.2em">y: ${yValue}</tspan>
                <tspan x="50%" text-anchor="middle" dy="1.2em">index: ${index}</tspan>
            </text>
        </svg>`;
};

// Add a CursorModifier to the chart
const cursorModifier = new CursorModifier({
    showTooltip: true,
    showAxisLabels: true,
    hitTestRadius: 10,
    tooltipSvgTemplate
});

sciChartSurface.chartModifiers.add(cursorModifier);
```

This results in the following output:

## Svg toolitip demo<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/cursor-modifier/customizing-cursor-modifier-tooltip-container-appearance/#svg-toolitip-demo" class="hash-link" aria-label="Direct link to Svg toolitip demo" translate="no" title="Direct link to Svg toolitip demo">â€‹</a>

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-modifier-api/cursor-modifier/customizing-cursor-modifier-tooltip-container-appearance/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-modifier-api/cursor-modifier/customizing-cursor-modifier-tooltip-container-appearance/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
