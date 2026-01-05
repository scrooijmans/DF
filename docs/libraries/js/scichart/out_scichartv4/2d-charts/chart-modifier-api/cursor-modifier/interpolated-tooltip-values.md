On this page

# Interpolated Tooltip Values

In SciChart.JS v3 we added some additional properties to theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/hittestinfo.html" rel="noopener noreferrer" target="_blank">hitTestInfoðŸ“˜</a> object so you can now get full information about the points either side of the hit-test location.Â  This allows you to do interpolation for your tooltip values rather than just showing values at the actual data points.Â 

xValue and yValue are always the values nearest the cursor.Â  point2xValue and point2vValue are the points on the other side of the cursor.Â  The interpolate function does linear interpoltion to find the y value for the x coordinate of the line between those points.

This example uses a CursorModifier, but the same principle would apply to RolloverModifier.Â  The difference is that the CursorModifier tooltipDataTemplate takes an array of seriesInfo because it is one tooltip for all series, whereas RolloverModifier does one tooltip per series.

- Interpolated ToolTip

``` prism-code
const interpolate = (x1: number, x2: number, y1: number, y2: number, x: number) => {
    return y1 + ((y2 - y1) * (x - x1)) / (x2 - x1);
};

const interpolatedTooltipDataTemplate: TCursorTooltipDataTemplate = (
    seriesInfos: SeriesInfo[],
    tooltipTitle: string
) => {
    const valuesWithLabels: string[] = [];
    seriesInfos.forEach((si, index) => {
        if (si.isHit) {
            if (index === 0) valuesWithLabels.push("X: " + si.getXCursorFormattedValue(si.hitTestPointValues.x));
            const xySeriesInfo = si as XySeriesInfo;
            const yValue = interpolate (
                xySeriesInfo.xValue,
                xySeriesInfo.point2xValue,
                xySeriesInfo.yValue,
                xySeriesInfo.point2yValue,
                xySeriesInfo.hitTestPointValues.x
            );
            const seriesTitle = si.seriesName ? si.seriesName : `Series #${index + 1}`;
            valuesWithLabels.push(seriesTitle);
            valuesWithLabels.push(`  Nearest: ${xySeriesInfo.formattedYValue}`);
            valuesWithLabels.push(`  Interpolated: ${xySeriesInfo.getYCursorFormattedValue(yValue)}`);
        }
    });
    return valuesWithLabels;
};

// Apply this to a cursorModifier
const cursorModifier = new CursorModifier({
    crosshairStroke: "#ff6600",
    crosshairStrokeThickness: 1,
    tooltipContainerBackground: "#F48420",
    showTooltip: true,
    axisLabelFill: "#F48420",
    axisLabelStroke: "#fff",
    tooltipDataTemplate: interpolatedTooltipDataTemplate
});
```

<img src="out_scichartv4/2d-charts/chart-modifier-api/cursor-modifier/interpolated-tooltip-values/index_media/49de0c864d107ad23735c9a1e965291702771448.gif" style="display:block;margin-left:auto;margin-right:auto;object-fit:contain;height:auto;width:85%;margin:0 auto" />

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/cursor-modifier/interpolated-tooltip-values/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [The CursorModifier Type](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/cursor-modifier/cursor-modifier-overview)
- [Rollover Modifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/rollover-modifier)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-modifier-api/cursor-modifier/interpolated-tooltip-values/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-modifier-api/cursor-modifier/interpolated-tooltip-values/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
