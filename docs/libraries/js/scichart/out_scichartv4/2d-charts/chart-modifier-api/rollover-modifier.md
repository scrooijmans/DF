On this page

# Rollover Modifier

SciChart.js provides aÂ mouse-over tooltipsÂ behavior via theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rollovermodifier.html" rel="noopener noreferrer" target="_blank">RolloverModifierðŸ“˜</a>, available out of the box.

Besides common features which are inherited from theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/chartmodifierbase.html" rel="noopener noreferrer" target="_blank">ChartModifierBaseðŸ“˜</a> class, theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rollovermodifier.html" rel="noopener noreferrer" target="_blank">RolloverModifierðŸ“˜</a> allows adding custom or standard tooltips to a chart (per series), and consuming the tooltip data in another area of your UI (custom legends).

## Adding aÂ RolloverModifierÂ to a Chart<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/rollover-modifier/#adding-arollovermodifierto-a-chart" class="hash-link" aria-label="Direct link to Adding aÂ RolloverModifierÂ to a Chart" translate="no" title="Direct link to Adding aÂ RolloverModifierÂ to a Chart">â€‹</a>

A RolloverModifier can be added to theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#chartmodifiers" rel="noopener noreferrer" target="_blank">sciChartSurface.chartModifiersðŸ“˜</a> collection to enable tooltip behavior. For example:

``` prism-code
// Create a SciChartSurface
const { sciChartSurface, wasmContext } = await SciChartSurface.create(divElementId);

// Add RolloverModifier behavior
sciChartSurface.chartModifiers.add(new RolloverModifier());
```

This results in the following behavior:

<img src="out_scichartv4/2d-charts/chart-modifier-api/rollover-modifier/index_media/e9729f6387b467a421d8022af5424c875587a3e5.gif" style="display:block;margin-left:auto;margin-right:auto;object-fit:contain;height:auto;width:85%;margin:0 auto" />

This is a basic implementation of the tooltip. You will notice that the text color on the White series is not visible. We will describe how to customise the tooltip including style and appearance below.

## Customizing the Rollover Appearance<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/rollover-modifier/#customizing-the-rollover-appearance" class="hash-link" aria-label="Direct link to Customizing the Rollover Appearance" translate="no" title="Direct link to Customizing the Rollover Appearance">â€‹</a>

### Styling the Vertical Line<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/rollover-modifier/#styling-the-vertical-line" class="hash-link" aria-label="Direct link to Styling the Vertical Line" translate="no" title="Direct link to Styling the Vertical Line">â€‹</a>

The Rollover Line itself can be styled by setting theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rollovermodifier.html#rolloverlinestrokethickness" rel="noopener noreferrer" target="_blank">rolloverLineStrokeThicknessðŸ“˜</a> orÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rollovermodifier.html#rolloverlinestroke" rel="noopener noreferrer" target="_blank">rolloverLineStrokeðŸ“˜</a> properties as follows. It can also be hidden with theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rollovermodifier.html#showrolloverline" rel="noopener noreferrer" target="_blank">showRolloverLine propertyðŸ“˜</a>.

### Styling the RolloverLine<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/rollover-modifier/#styling-the-rolloverline" class="hash-link" aria-label="Direct link to Styling the RolloverLine" translate="no" title="Direct link to Styling the RolloverLine">â€‹</a>

``` prism-code
const rolloverModifier = new RolloverModifier({
    rolloverLineStroke: "SteelBlue",
    rolloverLineStrokeThickness: 5,
    showRolloverLine: true
});
sciChartSurface.chartModifiers.add(rolloverModifier);
```

Which results in the following.

<img src="out_scichartv4/2d-charts/chart-modifier-api/rollover-modifier/index_media/eef6a6834e3fca0844afb92221d54c62523eb15e.png" style="display:block;margin-left:auto;margin-right:auto;object-fit:contain;height:auto;width:85%;margin:0 auto" />

### Styling the Tooltip<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/rollover-modifier/#styling-the-tooltip" class="hash-link" aria-label="Direct link to Styling the Tooltip" translate="no" title="Direct link to Styling the Tooltip">â€‹</a>

The Tooltip itself can be styled by setting the properties as follows.

- Styling the Tooltip

``` prism-code
const rendSeries = new FastBandRenderableSeries(wasmContext, { dataSeries, strokeThickness: 2 });
rendSeries.rolloverModifierProps.tooltipTextColor = "SteelBlue";
rendSeries.rolloverModifierProps.tooltipColor = "Pink";
rendSeries.rolloverModifierProps.tooltipLabelX = "X"; // X value will be hidden if X label is not set
rendSeries.rolloverModifierProps.tooltipLabelY = "Label Y";
// For Band series in order to set Y1 Tooltip use  rolloverModifierProps1 like
rendSeries.rolloverModifierProps1.width = 70; // here you can set the width manually
rendSeries.rolloverModifierProps1.tooltipLabelY = "Y1";
```

This results in the following:

<img src="out_scichartv4/2d-charts/chart-modifier-api/rollover-modifier/index_media/9282ab7119d5c0a9c256fe84235a97da342c1e00.png" style="display:block;margin-left:auto;margin-right:auto;object-fit:contain;height:auto;width:85%;margin:0 auto" />

### Placing the tooltip outside the chart<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/rollover-modifier/#placing-the-tooltip-outside-the-chart" class="hash-link" aria-label="Direct link to Placing the tooltip outside the chart" translate="no" title="Direct link to Placing the tooltip outside the chart">â€‹</a>

The tooltip can be placed anywhere on the chart by setting theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rollovermodifier.html#placementdivid" rel="noopener noreferrer" target="_blank">RolloverModifier.placementDivIdðŸ“˜</a> property.

Simply add this code when declaring the rollover:

``` prism-code
new RolloverModifier({ placementDivId: "tooltip-div-id" });
```

### Styling the Series RolloverMarker<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/rollover-modifier/#styling-the-series-rollovermarker" class="hash-link" aria-label="Direct link to Styling the Series RolloverMarker" translate="no" title="Direct link to Styling the Series RolloverMarker">â€‹</a>

The RolloverMarker is the small dot which intersects the rollover line and the series itself. By default this is set to the series stroke color.

You can change the market color by setting the propertyÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rollovermodifierrenderableseriesprops.html#markercolor" rel="noopener noreferrer" target="_blank">renderableSeries.rolloverModifierProps.markerColorðŸ“˜</a> to an HTML color code. e.g.:

``` prism-code
const rendSeries = new FastBandRenderableSeries(wasmContext, { dataSeries, strokeThickness: 2 });
rendSeries.rolloverModifierProps.markerColor = "Orange";
```

Results in:

<img src="out_scichartv4/2d-charts/chart-modifier-api/rollover-modifier/index_media/7c146ba9415c478fa86e787252c8bea9ad08a8bd.png" style="display:block;margin-left:auto;margin-right:auto;object-fit:contain;height:auto;width:85%;margin:0 auto" />

### Customizing the Tooltip Content<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/rollover-modifier/#customizing-the-tooltip-content" class="hash-link" aria-label="Direct link to Customizing the Tooltip Content" translate="no" title="Direct link to Customizing the Tooltip Content">â€‹</a>

The Tooltip Content can be customised in the RolloverModifier. There are two levels of this.Â  You can customise just the content for the tooltip, or you can supply your own svg to customise the appearance as well.

To customise the content, create aÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rollovermodifier.html#tooltipdatatemplate" rel="noopener noreferrer" target="_blank">tooltipDataTemplateðŸ“˜</a>. This is a function which takes a seriesInfo (which is the results of the hit-test) and returns an array of strings which are the lines that will appear in the tooltip.

- Tooltip Content

``` prism-code
const tooltipDataTemplate: TRolloverTooltipDataTemplate = (seriesInfo: XySeriesInfo): string[] => {
    const valuesWithLabels: string[] = [];
    // Line Series
    const xySeriesInfo = seriesInfo as XySeriesInfo;
    valuesWithLabels.push(`X: ${xySeriesInfo.formattedXValue} Y: ${xySeriesInfo.formattedYValue}`);
    return valuesWithLabels;
};
```

You can apply this template function to the rolloverModifier itself, in which case it will apply to all series, or you can set it per-series using renderableSeries.rolloverModifierProps.tooltipDataTemplate = tooltipDataTemplate;

ToÂ change the shape and layout of the tooltip, you can create an SVG element to be hosted inside the tooltip:

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

// Add a CursorModifier to the chart
const rolloverModifier = new RolloverModifier({});

sciChartSurface.chartModifiers.add(rolloverModifier);
```

This results in the following output:

### Consuming Tooltip Data in a Legend<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/rollover-modifier/#consuming-tooltip-data-in-a-legend" class="hash-link" aria-label="Direct link to Consuming Tooltip Data in a Legend" translate="no" title="Direct link to Consuming Tooltip Data in a Legend">â€‹</a>

Rollover data can also be consumed in a custom legend in the top-left of the chart. Try some code like this:

``` prism-code
const lineSeries = new FastLineRenderableSeries(wasmContext, { stroke: EColor.Orange });
lineSeries.rolloverModifierProps.tooltipLegendTemplate = (
    tooltipProps: RolloverModifierRenderableSeriesProps,
    seriesInfo: SeriesInfo
) => {
    return `<svg width="340" height="25">
        <rect width="100%" height="100%" fill="#000000DD" stroke="grey" stroke-width="2" />
        <svg width="100%">
            <text x="8" y="16" font-size="13" font-family="Verdana" fill="red">Custom Legend Tooltip</text>
            <text x="180" y="16" font-size="13" font-family="Verdana" fill="lightblue">X: ${seriesInfo.formattedXValue}</text>
            <text x="260" y="16" font-size="13" font-family="Verdana" fill="green">Y: ${seriesInfo.formattedYValue}</text>
        </svg>
    </svg>`;
};
```

This results in the following active-legend which updates as you move the mouse over the chart.

<img src="out_scichartv4/2d-charts/chart-modifier-api/rollover-modifier/index_media/fb772c6b0db939acc1fec38bcf786e76d9ff22d5.png" style="display:block;margin-left:auto;margin-right:auto;object-fit:contain;height:auto;width:85%;margin:0 auto" />

## Getting tooltips for the nearest point only<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/rollover-modifier/#getting-tooltips-for-the-nearest-point-only" class="hash-link" aria-label="Direct link to Getting tooltips for the nearest point only" translate="no" title="Direct link to Getting tooltips for the nearest point only">â€‹</a>

By default, RolloverModifier returns data for all series at the x-value the mouse is over, regardless of y-value.Â If you want to only see information for the points that are near the cursor in both x and y, then set theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rollovermodifier.html#hittestradius" rel="noopener noreferrer" target="_blank">hitTestRadiusðŸ“˜</a> property to a non-zero value, either on the modifier instance, or in the constructor options.Â This will cause it to report only on points that number of pixels away from the cursor.

### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/rollover-modifier/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [Interpolated Tooltip Values](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/cursor-modifier/interpolated-tooltip-values)
- [What is the ChartModifier API](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/chart-modifier-api-overview)
- [Common ChartModifiers Features](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/common-features/)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-modifier-api/rollover-modifier/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-modifier-api/rollover-modifier/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
