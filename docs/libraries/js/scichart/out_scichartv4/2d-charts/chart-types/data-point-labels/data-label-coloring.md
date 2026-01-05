On this page

# Data Label Coloring

DataLabels allow per data-point text labels to be drawn on series, or arbitrary text labels at x,y positions on the chart.

You can see several datalabel examples on the SciChart.js demo:

- <a href="https://www.scichart.com/demo/javascript-line-chart" rel="noopener noreferrer" target="_blank">The Line Chart example</a>
- <a href="https://www.scichart.com/demo/javascript-column-chart" rel="noopener noreferrer" target="_blank">The Column Chart example</a>
- <a href="https://www.scichart.com/demo/javascript-chart-color-points-individually-with-paletteprovider" rel="noopener noreferrer" target="_blank">The PaletteProvider example</a>
- <a href="https://www.scichart.com/demo/javascript-datalabels" rel="noopener noreferrer" target="_blank">The DataLabels demo</a>
- <a href="https://www.scichart.com/demo/javascript/stacked-column-chart" rel="noopener noreferrer" target="_blank">The Stacked Column Chart demo</a>
- <a href="https://www.scichart.com/demo/javascript/population-pyramid" rel="noopener noreferrer" target="_blank">The Population Pyramid demo</a>

Explore these for some rich examples of how to use this API.

## Label Colouring<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-point-labels/data-label-coloring/#label-colouring" class="hash-link" aria-label="Direct link to Label Colouring" translate="no" title="Direct link to Label Colouring">â€‹</a>

The color property on the dataLabels option sets the color for all labels, but you can do per-label coloring by overriding theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#getcolor" rel="noopener noreferrer" target="_blank">getColor()ðŸ“˜</a> function on theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html" rel="noopener noreferrer" target="_blank">dataLabelProviderðŸ“˜</a>.

This function hsa to return the integer color codes used by SciChart's engine, so you need to useÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#parsecolortouintargb" rel="noopener noreferrer" target="_blank">parseColorToUIntArgbðŸ“˜</a> helper function to convert from html colors. It is a good idea to pre-calculate integer colour codes, rather than compute them each time labels are drawn, as in the example below.

- TS
- JS

``` prism-code
// Create a column series and add dataLabels
const columnSeries = new FastColumnRenderableSeries(wasmContext, {
    stroke: "SteelBlue",
    fill: "LightSteelBlue",
    strokeThickness: 1,
    dataSeries: new XyDataSeries(wasmContext, {
        xValues: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
        yValues: [-3, -4, 0, 2, 6.3, 3, 4, 8, 7, 5, 6, 8]
    }),
    dataLabels: {
        positionMode: EColumnDataLabelPosition.Outside,
        style: {
            fontFamily: "Default",
            fontSize: 18,
            padding: new Thickness(3, 0, 3, 0)
        },
        color: "#EEE"
    }
});
sciChartSurface.renderableSeries.add(columnSeries);

// Override the colouring using dataLabelProvider.getColor
// import { parseColorToUIntArgb } from "scichart";
const red = parseColorToUIntArgb("red");
const yellow = parseColorToUIntArgb("yellow");
const green = parseColorToUIntArgb("green");


(columnSeries.dataLabelProvider as ColumnSeriesDataLabelProvider).getColor  = (dataLabelState, text) => {
    const y = dataLabelState.yVal();
    if (y <= 0) return red;
    if (y <= 5) return yellow;
    return green;
};
```

``` prism-code
// Create a column series and add dataLabels
const columnSeries = new FastColumnRenderableSeries(wasmContext, {
    stroke: "SteelBlue",
    fill: "LightSteelBlue",
    strokeThickness: 1,
    dataSeries: new XyDataSeries(wasmContext, {
        xValues: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
        yValues: [-3, -4, 0, 2, 6.3, 3, 4, 8, 7, 5, 6, 8]
    }),
    dataLabels: {
        positionMode: EColumnDataLabelPosition.Outside,
        style: {
            fontFamily: "Default",
            fontSize: 18,
            padding: new Thickness(3, 0, 3, 0)
        },
        color: "#EEE"
    }
});
sciChartSurface.renderableSeries.add(columnSeries);
// Override the colouring using dataLabelProvider.getColor
// import { parseColorToUIntArgb } from "scichart";
const red = parseColorToUIntArgb("red");
const yellow = parseColorToUIntArgb("yellow");
const green = parseColorToUIntArgb("green");
columnSeries.dataLabelProvider.getColor = (dataLabelState, text) => {
    const y = dataLabelState.yVal();
    if (y <= 0)
        return red;
    if (y <= 5)
        return yellow;
    return green;
};
```

This results in the following output:

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/data-point-labels/data-label-coloring/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/data-point-labels/data-label-coloring/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
