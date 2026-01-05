On this page

# Tooltip Modifier 3D

Tooltips in SciChart.js 3D are performed by theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/tooltipmodifier3d.html" rel="noopener noreferrer" target="_blank">TooltipModifier3DðŸ“˜</a>. This is aÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/chartmodifierbase3d.html" rel="noopener noreferrer" target="_blank">ChartModifierBase3DðŸ“˜</a> derived type which executes on touch over the data point andÂ shows tooltips for the data-points under the mouse.

## Declaring a TooltipModifier3D<a href="https://www.scichart.com/documentation/js/v4/3d-charts/chart-modifier-3d-api/tooltip-modifier-3d/#declaring-a-tooltipmodifier3d" class="hash-link" aria-label="Direct link to Declaring a TooltipModifier3D" translate="no" title="Direct link to Declaring a TooltipModifier3D">â€‹</a>

Declaring aÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/tooltipmodifier3d.html" rel="noopener noreferrer" target="_blank">TooltipModifier3DðŸ“˜</a> is as simple as adding one to theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/orbitmodifier3d.html" rel="noopener noreferrer" target="_blank">SciChart3DSurface.chartModifiersðŸ“˜</a>Â property. This can be done as a single modifier, or as part of a group.

- TS
- HTML
- CSS

``` prism-code
// Declare a tooltip and add to the chart like this.
// Optional parameters help define tooltip operation
const tooltipModifier = new TooltipModifier3D({
    isCrosshairVisible: true,
    crosshairStroke: "#83D2F5",
    crosshairStrokeThickness: 3,
    tooltipContainerBackground: "#537ABD",
    tooltipTextStroke: "White",
    tooltipLegendOffsetX: 10,
    tooltipLegendOffsetY: 10
  });

sciChart3DSurface.chartModifiers.add(tooltipModifier);
```

``` prism-code
<div class="wrapper">
    <div id="scichart-root"></div>
    <div class="titleWrapper">
        <p class="title">SciChart.js 3D Chart Example</p>
        <p class="subTitle">Hover the points to see tooltips</p>
    </div>
</div>
```

``` prism-code
body {
    margin: 0;
    font-family: Arial;
}
.wrapper {
    width: 100%;
    height: 100vh;
    position: relative;
}
#scichart-root {
    width: 100%;
    height: 100%;
    position: relative;
}
.titleWrapper {
    position: absolute;
    width: 100%;
    top: 35%;
    text-align: center;
    pointer-events: none;
    color: #ffffff77;
}
.title {
    font-size: 20px;
}
.subTitle {
    font-size: 16px;
}
```

This results in the following behaviour added to the chart.

## Styling the Tooltip Output<a href="https://www.scichart.com/documentation/js/v4/3d-charts/chart-modifier-3d-api/tooltip-modifier-3d/#styling-the-tooltip-output" class="hash-link" aria-label="Direct link to Styling the Tooltip Output" translate="no" title="Direct link to Styling the Tooltip Output">â€‹</a>

### Properties which affect Tooltip style<a href="https://www.scichart.com/documentation/js/v4/3d-charts/chart-modifier-3d-api/tooltip-modifier-3d/#properties-which-affect-tooltip-style" class="hash-link" aria-label="Direct link to Properties which affect Tooltip style" translate="no" title="Direct link to Properties which affect Tooltip style">â€‹</a>

Some simple properties which affect the tooltip style are:

| Property | Description |
|----|----|
| **isCrosshairVisible** | When true (default), a crosshair is drawn from the hovered datapoint to the far axis wall. |
| **crosshairStroke** | The stroke color as a Hex code of the crosshair line. |
| **crosshairStrokeThickness** | The stroke thickness of the crosshair line. |
| **tooltipContainerBackground** | The background color of the tooltip container as a Hex code. |
| **tooltipLegendOffsetX** / **Y** | Offset in pixels of the tooltip from the hovered datapoint. |
| **tooltipTextStroke** | The text color on the tooltip. |

For further customisation of the tooltip content & container, read on.

### Tooltip Text Formatting<a href="https://www.scichart.com/documentation/js/v4/3d-charts/chart-modifier-3d-api/tooltip-modifier-3d/#tooltip-text-formatting" class="hash-link" aria-label="Direct link to Tooltip Text Formatting" translate="no" title="Direct link to Tooltip Text Formatting">â€‹</a>

Tooltips obey formatting rules on the Axis. These can be defined by settingÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/labelprovider.html" rel="noopener noreferrer" target="_blank">axis.labelProvider.cursorPrecisionðŸ“˜</a>,Â <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/labelprovider.html#cursornumericformat" rel="noopener noreferrer" target="_blank">cursorNumericFormatðŸ“˜</a> or overridingÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/labelprovider.html#formatcursorlabel" rel="noopener noreferrer" target="_blank">formatCursorLabelðŸ“˜</a>. For more information on text formatting, see theÂ [LabelProvider documentation](https://www.scichart.com/documentation/js/v4/3d-charts/axis-3d-api/axis-3d-text-label-formatting/).

### Modifying the Tooltip Content<a href="https://www.scichart.com/documentation/js/v4/3d-charts/chart-modifier-3d-api/tooltip-modifier-3d/#modifying-the-tooltip-content" class="hash-link" aria-label="Direct link to Modifying the Tooltip Content" translate="no" title="Direct link to Modifying the Tooltip Content">â€‹</a>

You can modify the content output by tooltip via the **TooltipModifier3D.tooltipDataTemplate** property. This accepts a function with **SeriesInfo3D** andÂ **TooltipSvgAnnotation3D** arguments where you can access data about the series that was hit.

- TS
- HTML
- CSS

``` prism-code
// Declare a tooltip and add to the chart
const tooltipModifier = new TooltipModifier3D({
    crosshairStroke: "#83D2F5",
    crosshairStrokeThickness: 3,
    tooltipContainerBackground: "#537ABD",
    tooltipTextStroke: "White",
    tooltipLegendOffsetX: 10,
    tooltipLegendOffsetY: 10
});

sciChart3DSurface.chartModifiers.add(tooltipModifier);

// Customize the tooltip content
tooltipModifier.tooltipDataTemplate = (seriesInfo, svgAnnotation) => {
    // Create an array to hold strings (lines) to show in the tooltip
    const valuesWithLabels = [];
    if (seriesInfo && seriesInfo.isHit) {
        // You can access the renderableSeries which was hit via the seriesInfo
        const renderableSeries = seriesInfo.renderableSeries;
        // And the parent Chart from that
        const parentSurface = renderableSeries.parentSurface;

        // Push lines to the array to display in the tooltip
        valuesWithLabels.push(`dataSeriesName: "${seriesInfo.dataSeriesName}"`);
        valuesWithLabels.push(` ${parentSurface.xAxis.axisTitle}: ${seriesInfo.xValue.toFixed(2)}`);
        valuesWithLabels.push(` ${parentSurface.yAxis.axisTitle}: ${seriesInfo.yValue.toFixed(2)}`);
        valuesWithLabels.push(` ${parentSurface.zAxis.axisTitle}: ${seriesInfo.zValue.toFixed(2)}`);

        // access the metadata (if exists)". Any JS object on the data-points can be accessed
        // in tooltips
        const md = (seriesInfo as XyzSeriesInfo3D).pointMetadata as TMyMetadata;
        if (md) {
            valuesWithLabels.push(` Metadata: "${md.customString}"`);
        }
    }
    return valuesWithLabels;
};
```

``` prism-code
<div class="wrapper">
    <div id="scichart-root"></div>
    <div class="titleWrapper">
        <p class="title">SciChart.js 3D Chart Example</p>
        <p class="subTitle">Hover the points to see tooltips</p>
    </div>
</div>
```

``` prism-code
body {
    margin: 0;
    font-family: Arial;
}
.wrapper {
    width: 100%;
    height: 100vh;
    position: relative;
}
#scichart-root {
    width: 100%;
    height: 100%;
    position: relative;
}
.titleWrapper {
    position: absolute;
    width: 100%;
    top: 35%;
    text-align: center;
    pointer-events: none;
    color: #ffffff77;
}
.title {
    font-size: 20px;
}
.subTitle {
    font-size: 16px;
}
```

This results in the following output.

The arguments to the **tooltipDataTemplate** function are **SeriesInfo3D** and **TooltipSvgAnnotation3D**. You can access any info about the series, parent chart or axis from SeriesInfo3D. Inspect these types in the TypeDoc to see what properties are available.

### Modifying the Tooltip Container<a href="https://www.scichart.com/documentation/js/v4/3d-charts/chart-modifier-3d-api/tooltip-modifier-3d/#modifying-the-tooltip-container" class="hash-link" aria-label="Direct link to Modifying the Tooltip Container" translate="no" title="Direct link to Modifying the Tooltip Container">â€‹</a>

The container of the tooltip can be modified as well. Extending the example above further, we override **TooltipModifier3D.tooltipSvgTemplate** to customize the background/foreground color before rendering the tooltip.

- TS
- HTML
- CSS

``` prism-code
// Declare a tooltip and add to the chart
const tooltipModifier = new TooltipModifier3D({
    crosshairStroke: "#83D2F5",
    crosshairStrokeThickness: 3,
    tooltipContainerBackground: "#537ABD",
    tooltipTextStroke: "White",
    tooltipLegendOffsetX: 10,
    tooltipLegendOffsetY: 10
});

// Customize the tooltip container like this
const defaultTemplate = tooltipModifier.tooltipSvgTemplate;
tooltipModifier.tooltipSvgTemplate = (seriesInfo, svgAnnotation) => {
    if (seriesInfo) {
        const md = (seriesInfo as XyzSeriesInfo3D).pointMetadata;
        const backgroundColor = md ? parseArgbToHtmlColor(md.vertexColor) : seriesInfo.renderableSeries.stroke;
        svgAnnotation.containerBackground = backgroundColor;
        svgAnnotation.textStroke = "white";
    }
    return defaultTemplate(seriesInfo, svgAnnotation);
};

sciChart3DSurface.chartModifiers.add(tooltipModifier);
```

``` prism-code
<div class="wrapper">
    <div id="scichart-root"></div>
    <div class="titleWrapper">
        <p class="title">SciChart.js 3D Chart Example</p>
        <p class="subTitle">Hover the points to see tooltips</p>
    </div>
</div>
```

``` prism-code
body {
    margin: 0;
    font-family: Arial;
}
.wrapper {
    width: 100%;
    height: 100vh;
    position: relative;
}
#scichart-root {
    width: 100%;
    height: 100%;
    position: relative;
}
.titleWrapper {
    position: absolute;
    width: 100%;
    top: 35%;
    text-align: center;
    pointer-events: none;
    color: #ffffff77;
}
.title {
    font-size: 20px;
}
.subTitle {
    font-size: 16px;
}
```

This results in the following output:

## Placing the Tooltip as a Separate Legend<a href="https://www.scichart.com/documentation/js/v4/3d-charts/chart-modifier-3d-api/tooltip-modifier-3d/#placing-the-tooltip-as-a-separate-legend" class="hash-link" aria-label="Direct link to Placing the Tooltip as a Separate Legend" translate="no" title="Direct link to Placing the Tooltip as a Separate Legend">â€‹</a>

The tooltip can be placed as a legend in the corner of the chart by using the **TooltipModifier3D.placementDivId** property. This simply changes the location in the HTML Dom where tooltips are placed.

Here's a quick example:

- TS
- HTML
- CSS

``` prism-code
// Declare a tooltip and add to the chart like this.
const tooltipModifier = new TooltipModifier3D({
    crosshairStroke: "#83D2F5",
    crosshairStrokeThickness: 3,
    tooltipContainerBackground: "#537ABD",
    tooltipTextStroke: "White",
    tooltipLegendOffsetX: 10,
    tooltipLegendOffsetY: 10,

    // Allows placement of tooltip in a custom div anywhere in your app
    placementDivId: "tooltipContainerDivId"
});

sciChart3DSurface.chartModifiers.add(tooltipModifier);
```

``` prism-code
<div class="wrapper">
    <div id="scichart-root"></div>
    <div class="titleWrapper">
        <p class="title">SciChart.js 3D Chart Example</p>
        <p class="subTitle">Hover the points to see tooltips</p>
    </div>
    <div id="tooltipContainerDivId">
        <!-- Tooltips are placed here -->
    </div>
</div>
```

``` prism-code
body {
    margin: 0;
    font-family: Arial;
}
.wrapper {
    width: 100%;
    height: 100vh;
    position: relative;
}
#scichart-root {
    width: 100%;
    height: 100%;
    position: relative;
}
.titleWrapper {
    position: absolute;
    width: 100%;
    top: 35%;
    text-align: center;
    pointer-events: none;
    color: #ffffff77;
}
.title {
    font-size: 20px;
}
.subTitle {
    font-size: 16px;
}
#tooltipContainerDivId {
    position: absolute;
    top: 10px;
    left: 10px;
    pointer-events: none;
}
```

This results in the following output:

If you hover over data points you will notice that the tooltips appears in the left top part of the chart.

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/3d-charts/chart-modifier-3d-api/tooltip-modifier-3d/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/3d-charts/chart-modifier-3d-api/tooltip-modifier-3d/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
