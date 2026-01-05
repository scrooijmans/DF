On this page

# Creating a Pie Chart

SciChart provides a powerful API for creating various types of charts, including **Pie Charts**.

The <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#chartbuilder.buildchart" rel="noopener noreferrer" target="_blank">buildChartðŸ“˜</a> function can be used to build both 2D Charts, **Pie Charts**, 2D Polar Charts & 3D Charts, so the returned object type will differ depending on the chart type.

## Using <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#chartbuilder.buildchart" rel="noopener noreferrer" target="_blank">buildChartðŸ“˜</a> to create a Pie Chart<a href="https://www.scichart.com/documentation/js/v4/2d-charts/builder-api/pie-chart/#using-buildchartblue_book-to-create-a-pie-chart" class="hash-link" aria-label="Direct link to using-buildchartblue_book-to-create-a-pie-chart" translate="no" title="Direct link to using-buildchartblue_book-to-create-a-pie-chart">â€‹</a>

- TS

``` prism-code
const { chartBuilder, ESciChartSurfaceType } = SciChart;

const pieSurface = chartBuilder.buildChart(divElementId, {
    type: ESciChartSurfaceType.Pie2D,
    options: {
        segments: [
            { text: "This", value: 10, color: "red" },
            { text: "That", value: 5, color: "blue" },
            { text: "Other", value: 7, color: "green" }
        ]
    }
});
```

## Using <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#buildpiechart" rel="noopener noreferrer" target="_blank">buildPieChartðŸ“˜</a> to explicitly create a Pie Chart:<a href="https://www.scichart.com/documentation/js/v4/2d-charts/builder-api/pie-chart/#using-buildpiechartblue_book-to-explicitly-create-a-pie-chart" class="hash-link" aria-label="Direct link to using-buildpiechartblue_book-to-explicitly-create-a-pie-chart" translate="no" title="Direct link to using-buildpiechartblue_book-to-explicitly-create-a-pie-chart">â€‹</a>

- TS

``` prism-code
const { chartBuilder } = SciChart;

const pieSurface = chartBuilder.buildPieChart(divElementId, {
    segments: [
        { text: "This", value: 10, color: "red" },
        { text: "That", value: 5, color: "blue" },
        { text: "Other", value: 7, color: "green" }
    ]
});
```

Both of these methods will result in this output:

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/builder-api/pie-chart/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [Intro to the Builder API](https://www.scichart.com/documentation/js/v4/2d-charts/builder-api/builder-api-overview)
- [Working with Data](https://www.scichart.com/documentation/js/v4/2d-charts/builder-api/working-with-data)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/builder-api/pie-chart/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/builder-api/pie-chart/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
