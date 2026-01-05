On this page

# Custom DataLabel Formatting with getText()

DataLabels allow per data-point text labels to be drawn on series, or arbitrary text labels at x,y positions on the chart.

You can see several datalabel examples on the SciChart.js demo:

- <a href="https://www.scichart.com/demo/javascript-line-chart" rel="noopener noreferrer" target="_blank">The Line Chart example</a>
- <a href="https://www.scichart.com/demo/javascript-column-chart" rel="noopener noreferrer" target="_blank">The Column Chart example</a>
- <a href="https://www.scichart.com/demo/javascript-chart-color-points-individually-with-paletteprovider" rel="noopener noreferrer" target="_blank">The PaletteProvider example</a>
- <a href="https://www.scichart.com/demo/javascript-datalabels" rel="noopener noreferrer" target="_blank">The DataLabels demo</a>
- <a href="https://www.scichart.com/demo/javascript/stacked-column-chart" rel="noopener noreferrer" target="_blank">The Stacked Column Chart demo</a>
- <a href="https://www.scichart.com/demo/javascript/population-pyramid" rel="noopener noreferrer" target="_blank">The Population Pyramid demo</a>

Explore these for some rich examples of how to use this API.

In the articleÂ [DataLabels API Overview](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-point-labels/data-labels-api-overview) we showed you how to apply simple formatting rules to change the number of decimal places on Data Labels. What if you needed more complex Data Label formatting rules? Enter custom label formatting.

## Custom Label Formatting<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-point-labels/custom-data-label-formatting/#custom-label-formatting" class="hash-link" aria-label="Direct link to Custom Label Formatting" translate="no" title="Direct link to Custom Label Formatting">â€‹</a>

To take full control of the label text, override theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#gettext" rel="noopener noreferrer" target="_blank">dataLabelProvider.getText()ðŸ“˜</a> function on the seriesÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries.html#datalabelprovider" rel="noopener noreferrer" target="_blank">renderableSeries.dataLabelProviderðŸ“˜</a>.Â 

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#gettext" rel="noopener noreferrer" target="_blank">getTextðŸ“˜</a> has a single parameter of typeÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelstate.html" rel="noopener noreferrer" target="_blank">DataLabelStateðŸ“˜</a>. This has a number of helper functions that allow you to get the x and y values and coordinates without having to worry about which way the axes run or if you are using a vertical chart.

This example outputs both X and Y Values. DataLabels are rendered using the new native text system, so they support multiline using \n for newlines. The dataLabels style option has **multiLineAlignment** and **lineSpacing** properties for controlling multiline text.

- TS
- Builder API (JSON Config)

``` prism-code
// Create a line series with dataLabels
const lineSeries = new FastLineRenderableSeries(wasmContext, {
    stroke: "SteelBlue",
    strokeThickness: 3,
    pointMarker: new EllipsePointMarker(wasmContext, {
        width: 10,
        height: 10,
        strokeThickness: 2,
        stroke: "SteelBlue",
        fill: "LightSteelBlue"
    }),
    dataSeries: new XyDataSeries(wasmContext, {
        xValues: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
        yValues: [4.3, 5.3, 6, 6.3, 6, 5.2, 4.5, 4.6, 5, 6, 7, 8]
    }),
    // dataLabels style must be specified to show labels
    dataLabels: {
        style: {
            fontFamily: "Arial",
            fontSize: 16,
            lineSpacing: 4,
            multiLineAlignment: EMultiLineAlignment.Left
        },
        color: "#EEE"
    }
});

// Override default dataLabelProvider.getText() function
// See type DataLabelState for available data
(lineSeries.dataLabelProvider as DataLabelProvider).getText = dataLabelState => {
    return `Point index ${dataLabelState.index}\n[x: ${dataLabelState.xVal()}, y: ${dataLabelState.yVal()}]`;
};
```

``` prism-code
// Demonstrates how to add DataLabels to a chart with SciChart.js using the Builder API

const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    surface: { theme: { type: EThemeProviderType.Dark } },
    series: [
        {
            type: ESeriesType.LineSeries,
            xyData: {
                xValues: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
                yValues: [4.3, 5.3, 6, 6.3, 6, 5.2, 4.5, 4.6, 5, 6, 7, 8]
            },
            options: {
                stroke: "#0066FF",
                strokeThickness: 5,
                pointMarker: {
                    type: EPointMarkerType.Ellipse,
                    options: {
                        width: 10,
                        height: 10,
                        strokeThickness: 2,
                        stroke: "SteelBlue",
                        fill: "LightSteelBlue"
                    }
                },
                // Data labels are enabled here. Simply set style, color
                dataLabels: {
                    style: {
                        fontFamily: "Arial",
                        fontSize: 16
                    },
                    color: "#EEE"
                }
            }
        }
    ]
});

// Note you can access dataLabelProvider from a constructed chart as follows
(sciChartSurface.renderableSeries.get(0).dataLabelProvider as DataLabelProvider).getText = dataLabelState => {
    return `Point index ${dataLabelState.index}\n[x: ${dataLabelState.xVal()}, y: ${dataLabelState.yVal()}]`;
};
```

![](out_scichartv4/2d-charts/chart-types/data-point-labels/custom-data-label-formatting/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

See theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelstate.html" rel="noopener noreferrer" target="_blank">DataLabelStateðŸ“˜</a> type for what data is passed into theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#gettext" rel="noopener noreferrer" target="_blank">getText()ðŸ“˜</a> function for each label

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/data-point-labels/custom-data-label-formatting/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/data-point-labels/custom-data-label-formatting/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
