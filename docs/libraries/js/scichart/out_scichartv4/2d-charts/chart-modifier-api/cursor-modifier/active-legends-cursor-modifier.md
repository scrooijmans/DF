On this page

# Active Legends - CursorModifier output into a legend

![](out_scichartv4/2d-charts/chart-modifier-api/cursor-modifier/active-legends-cursor-modifier/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

Background reading:

If you haven't already, read the articleÂ [The CursorModifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/cursor-modifier/cursor-modifier-overview) typeÂ which will show you how to setup a <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/cursormodifier.html" rel="noopener noreferrer" target="_blank">CursorModifierðŸ“˜</a> with default options for tooltips. This article goes into further detail on customising the tooltip items (formatting, text content).

## CursorModifier tooltipLegendTemplates<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/cursor-modifier/active-legends-cursor-modifier/#cursormodifier-tooltiplegendtemplates" class="hash-link" aria-label="Direct link to CursorModifier tooltipLegendTemplates" translate="no" title="Direct link to CursorModifier tooltipLegendTemplates">â€‹</a>

TheÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/cursormodifier.html" rel="noopener noreferrer" target="_blank">CursorModifierðŸ“˜</a>Â supports a tooltipLegendTemplate property which allows you to specify a function to transform CursorModifier content into a legend which can be placed in the top left of the chart. This active legend updates with series values as you move the mouse.

Perhaps the best example of this is in financial charts, which require placing a legend in the top left of the chart to show the current hovered candle or series.

Here's a worked example below, which extends ourÂ [Candlestick Chart - Volume Bars](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-candlestick-renderable-series) example with an active legend provided byÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/cursormodifier.html" rel="noopener noreferrer" target="_blank">CursorModifierðŸ“˜</a>.

- TS

``` prism-code
// Add a CursorModifier with active legend to the chart
const cursorModifier = new CursorModifier({
    // X,Y offset in pixels for the active legend
    tooltipLegendOffsetX: 5,
    tooltipLegendOffsetY: 5,
    // Callback to format the legend
    tooltipLegendTemplate: (si: SeriesInfo[], svgAnnotation: CursorTooltipSvgAnnotation) => {
        const seriesInfos = si as OhlcSeriesInfo[];
        let outputSvgString = "";

        // Foreach series there will be a seriesInfo supplied by SciChart. This contains info about the series under the mouse
        seriesInfos.forEach((seriesInfo, index) => {
            const y = 20 + index * 20;
            // use the series.stroke for the text color. If the series.stroke is transparent, use white
            let textColor = seriesInfo.stroke;
            if (textColor === undefined || parseColorToTArgb(textColor).opacity === 0) {
                textColor = "#ffffff";
            }
            // Default handling for Xy series
            let legendText = seriesInfo.formattedYValue;
            // Special handling for Ohlc series
            if (seriesInfo.dataSeriesType === EDataSeriesType.Ohlc) {
                legendText =
                    `Open=${seriesInfo.formattedOpenValue} High=${seriesInfo.formattedHighValue} ` +
                    `Low=${seriesInfo.formattedLowValue} Close=${seriesInfo.formattedCloseValue}`;
            }
            // Output one block of text per seriesInfo on the chart. Using seriesName (from dataSeries.dataSeriesName) as a prefix
            outputSvgString += `<text x="8" y="${y}" font-size="13" font-family="Verdana" fill="${textColor}">
                ${seriesInfo.seriesName}: ${legendText}
            </text>`;
        });

        return `<svg width="100%" height="100%">
            ${outputSvgString}
        </svg>`;
    }
});
sciChartSurface.chartModifiers.add(cursorModifier);
```

This results in the following output:

## Using External placementDivId with the CursorModifier<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/cursor-modifier/active-legends-cursor-modifier/#using-external-placementdivid-with-the-cursormodifier" class="hash-link" aria-label="Direct link to Using External placementDivId with the CursorModifier" translate="no" title="Direct link to Using External placementDivId with the CursorModifier">â€‹</a>

Another way you can control the placement of theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/cursormodifier.html" rel="noopener noreferrer" target="_blank">CursorModifierðŸ“˜</a>Â tooltip is using theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/cursormodifier.html#placementdivid" rel="noopener noreferrer" target="_blank">placementDivIdðŸ“˜</a> property. This places the standard CursorModifier tooltip into a div of your choice (which can be anywhere on the app).

Note, it does not currently work withÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/cursormodifier.html#tooltiplegendtemplate" rel="noopener noreferrer" target="_blank">tooltipLegendTemplateðŸ“˜</a>, however we are working on more options for styling, placement and configuration of tooltips soon.

Try the following code in your application:

- TS

``` prism-code
// Add a CursorModifier with external placement div on the chart
// Expects <div id="legend-root" /> to be present in the DOM
const cursorModifier = new CursorModifier({
    placementDivId: "legend-root",
    showTooltip: true,
    tooltipContainerBackground: "#4682b433"
});
sciChartSurface.chartModifiers.add(cursorModifier);
```

- html

``` prism-code
<body>
    <div id="container">
        <div id="scichart-root" ></div>
        <div id="legend-root" ></div>
    </div>
</body>
```

- css

``` prism-code
body { margin: 0; }
#container { width: 100%; height: 100vh; position: relative; }
#scichart-root { width: 100%; height: 100%; position: relative; }
#legend-root { position: absolute; left: 20px; top: 20px; }
```

This results in the following output:

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/cursor-modifier/active-legends-cursor-modifier/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [Formatting CursorModifier Tooltip Items](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/cursor-modifier/formatting-cursor-modifier-tooltip-items)
- [Customizing the CursorModifier Tooltip Container Appearance](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/cursor-modifier/customizing-cursor-modifier-tooltip-container-appearance)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-modifier-api/cursor-modifier/active-legends-cursor-modifier/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-modifier-api/cursor-modifier/active-legends-cursor-modifier/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
