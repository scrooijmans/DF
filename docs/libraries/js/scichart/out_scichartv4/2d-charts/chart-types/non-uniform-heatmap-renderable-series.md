On this page

# The Non-Uniform Heatmap Chart Type

A complementary type to the Uniform Heatmap is the Non-Uniform Heatmap, new to SciChart.js v2.3.

Non-Uniform Heatmaps can be created using theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/nonuniformheatmaprenderableseries.html" rel="noopener noreferrer" target="_blank">NonUniformHeatmapRenderableSeriesðŸ“˜</a> type.

![](out_scichartv4/2d-charts/chart-types/non-uniform-heatmap-renderable-series/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

TheÂ <a href="https://www.scichart.com/demo/javascript-non-uniform-heatmap-chart" rel="noopener noreferrer" target="_blank">JavaScript Non-Uniform Heatmap Chart Example</a> can be found in theÂ <a href="https://github.com/ABTSoftware/SciChart.JS.Examples/tree/master/Examples/src/components/Examples/Charts2D/BasicChartTypes/NonUniformHeatmapChart" rel="noopener noreferrer" target="_blank">SciChart.Js Examples Suite &gt; Non-Uniform Heatmap Chart</a>Â on Github

Above: The JavaScript <a href="https://www.scichart.com/demo/iframe/non-uniform-heatmap-chart" target="_blank">Digital Mountain Series Chart</a> example from the <a href="https://www.scichart.com/demo/react" target="_blank">SciChart.js Demo</a>

  

## Create aÂ Non-Uniform Heatmap<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/non-uniform-heatmap-renderable-series/#create-anon-uniform-heatmap" class="hash-link" aria-label="Direct link to Create aÂ Non-Uniform Heatmap" translate="no" title="Direct link to Create aÂ Non-Uniform Heatmap">â€‹</a>

Non-Uniform heatmaps are a variation on Uniform heatmaps, where you can specify independent sizes for heatmap rows and columns.

The cell sizes are specified either by an array of X,Y cell coordinates or a mapping function passed to the constructor options ofÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/nonuniformheatmapdataseries.html" rel="noopener noreferrer" target="_blank">NonUniformHeatmapDataSeriesðŸ“˜</a>.

For example, you can create a Non-uniform Heatmap with the following code:

- TS
- Builder API (JSON Config)

``` prism-code
// Demonstrates how to create a uniform heatmap chart with SciChart.js
const {
    SciChartSurface,
    NumericAxis,
    HeatmapColorMap,
    NonUniformHeatmapRenderableSeries,
    NonUniformHeatmapDataSeries,
    SciChartJsNavyTheme
} = SciChart;
// or, for npm, import { SciChartSurface, ... } from "scichart"

// Create a SciChartSurface with X & Y Axis
const { wasmContext, sciChartSurface } = await SciChartSurface.create(divElementId, {
    theme: new SciChartJsNavyTheme()
});
sciChartSurface.xAxes.add(new NumericAxis(wasmContext));
sciChartSurface.yAxes.add(new NumericAxis(wasmContext));

// The data for the heatmap is a 2d Array of zValues [height][width]
// for example:
//   const zValues = [
//     [0, 2, 3.4],
//     [5, 3, 4],
//     [3, 1.5, -1],
//   ];
const zValues = generateHeatmapData();

// Create the non-uniform heatmap series
const heatmapSeries = new NonUniformHeatmapRenderableSeries(wasmContext, {
    // Pass in the 2d zValues array and x/yCellOffsets to give x,y positions
    dataSeries: new NonUniformHeatmapDataSeries(wasmContext, { 
        zValues, 
        // arrays with the cell offsets
        xCellOffsets: [0, 10, 20, 26, 36, 60, 72, 84],
        yCellOffsets: [100, 250, 390, 410, 600]
    }),
    // zValues mapped to colours using the colorMap.
    // zValue[y][x] when compared to HeatmapColorMap.maximum corresponds to gradientstop offset=1
    // zValue[y][x] when compared to HeatmapColorMap.minimum corresponds to gradientstop offset=0
    colorMap: new HeatmapColorMap({
        minimum: 0,
        maximum: 100,
        gradientStops: [
            { offset: 0, color: "#14233C" },
            { offset: 0.2, color: "#264B93" },
            { offset: 0.3, color: "#50C7E0" },
            { offset: 0.5, color: "#67BDAF" },
            { offset: 0.7, color: "#DC7969" },
            { offset: 0.9, color: "#F48420" },
            { offset: 1, color: "#EC0F6C" }
        ]
    }),
    // optional settings
    opacity: 0.77,
    // values outside of the colorMap.min/max will be filled with the colours at edge of the colormap
    fillValuesOutOfRange: true,
    // Optional datalabels may be placed in cell
    dataLabels: {
        style: { 
            fontFamily: "Default", 
            fontSize: 16 
        },
        color: "white"
    }
});

sciChartSurface.renderableSeries.add(heatmapSeries);
```

``` prism-code
// Demonstrates how to create a line chart with SciChart.js using the Builder API
const { chartBuilder, ESeriesType, HeatmapColorMap, EThemeProviderType } = SciChart;
// or, for npm, import { chartBuilder, ... } from "scichart"

const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    surface: { theme: { type: EThemeProviderType.Dark } },
    series: {
        type: ESeriesType.NonUniformHeatmapSeries,
        options: {
            colorMap: new HeatmapColorMap({
                minimum: 0,
                maximum: 100,
                gradientStops: [
                    { offset: 0, color: "#14233C" },
                    { offset: 0.2, color: "#264B93" },
                    { offset: 0.3, color: "#50C7E0" },
                    { offset: 0.5, color: "#67BDAF" },
                    { offset: 0.7, color: "#DC7969" },
                    { offset: 0.9, color: "#F48420" },
                    { offset: 1, color: "#EC0F6C" }
                ]
            }),
            dataLabels: {
                style: { fontFamily: "Default", fontSize: 16 },
                color: "White"
            }
        },
        heatmapData: {
            zValues: generateHeatmapData(),
            xCellOffsets: [0, 10, 20, 26, 36, 60, 72, 84],
            yCellOffsets: [100, 250, 390, 410, 600]
        }
    }
});
```

This results in the following output:

**Above:** The Non-Uniform Heatmap allows you to have uneven sizes for columns & rows in a javascript heatmap. In the case where you have equal cell sizes, use theÂ [Uniform Heatmap](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/uniform-heatmap-renderable-series/uniform-heatmap-chart-type) for faster performance.

In the code above:

- We create a 2D array (type `number[][]`). This is filled with heat values of the heatmap.
- AÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/nonuniformheatmapdataseries.html" rel="noopener noreferrer" target="_blank">NonUniformHeatmapDataSeriesðŸ“˜</a> instance is created with `xCellOffsets = []` and `yCellOffsets = []`. This defines the position of the heatmap in X,Y space as well as the position of each column/row.
- We set the Colormap, which maps colors to heat values in the dataseries.
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/nonuniformheatmaprenderableseries.html" rel="noopener noreferrer" target="_blank">NonUniformHeatmapRenderableSeriesðŸ“˜</a> instance is created withÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/inonuniformheatmaprenderableseriesoptions.html#dataseries" rel="noopener noreferrer" target="_blank">INonUniformHeatmapRenderableSeriesOptions.dataSeriesðŸ“˜</a> andÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/inonuniformheatmaprenderableseriesoptions.html#colormap" rel="noopener noreferrer" target="_blank">INonUniformHeatmapRenderableSeriesOptions.colorMapðŸ“˜</a> options and added to theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#renderableseries" rel="noopener noreferrer" target="_blank">sciChartSurface.renderableSeriesðŸ“˜</a> collection.
- Alternatively we can assign aÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/nonuniformheatmaprenderableseries.html#dataseries" rel="noopener noreferrer" target="_blank">NonUniformHeatmapRenderableSeries.dataSeriesðŸ“˜</a> property separately.

### Updating Heatmap Values<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/non-uniform-heatmap-renderable-series/#updating-heatmap-values" class="hash-link" aria-label="Direct link to Updating Heatmap Values" translate="no" title="Direct link to Updating Heatmap Values">â€‹</a>

TheÂ [Uniform Heatmap documentation - Updating Heatmaps](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/uniform-heatmap-renderable-series/updating-realtime)Â shows how you can update a heatmap dynamically, by using theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/nonuniformheatmapdataseries.html#setzvalues" rel="noopener noreferrer" target="_blank">setZValues()ðŸ“˜</a> function. The mechanism for the Non-uniform heatmap is the same.

### Heatmap ColorÂ MapsÂ <a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/non-uniform-heatmap-renderable-series/#heatmap-colormaps" class="hash-link" aria-label="Direct link to Heatmap ColorÂ MapsÂ " translate="no" title="Direct link to Heatmap ColorÂ MapsÂ ">â€‹</a>

TheÂ [Uniform Heatmap documentation - ColorMaps and Legends](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/uniform-heatmap-renderable-series/uniform-heatmap-chart-type)Â shows how you can modify a heatmaps color mapping, which maps zValues to cell colors, by using theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/nonuniformheatmaprenderableseries.html#colormap" rel="noopener noreferrer" target="_blank">colorMapðŸ“˜</a> property. The mechanism for the Non-uniform heatmap is the same.

### Adding Text in Cell to a Non-Uniform Heatmap<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/non-uniform-heatmap-renderable-series/#adding-text-in-cell-to-a-non-uniform-heatmap" class="hash-link" aria-label="Direct link to Adding Text in Cell to a Non-Uniform Heatmap" translate="no" title="Direct link to Adding Text in Cell to a Non-Uniform Heatmap">â€‹</a>

TheÂ [Uniform Heatmap documentation](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/uniform-heatmap-renderable-series/uniform-heatmap-chart-type) shows how you can add text-in cell to a heatmap via theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/nonuniformheatmaprenderableseries.html#dataLabelProvider" rel="noopener noreferrer" target="_blank">dataLabelProviderðŸ“˜</a> property. The mechanism for the Non-uniform heatmap is the same.

### Adding a Heatmap Legend toÂ a Non-Uniform Heatmap<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/non-uniform-heatmap-renderable-series/#adding-a-heatmap-legend-toa-non-uniform-heatmap" class="hash-link" aria-label="Direct link to Adding a Heatmap Legend toÂ a Non-Uniform Heatmap" translate="no" title="Direct link to Adding a Heatmap Legend toÂ a Non-Uniform Heatmap">â€‹</a>

TheÂ [Uniform Heatmap documentation - ColorMaps and Legends](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/uniform-heatmap-renderable-series/uniform-heatmap-chart-type) shows how you can aÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/heatmaplegend.html" rel="noopener noreferrer" target="_blank">HeatmapLegendðŸ“˜</a> with colorMap to the heatmap chart. The mechanism for the Non-uniform heatmap is the same.Â 

## Defining the x,y cell positions<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/non-uniform-heatmap-renderable-series/#defining-the-xy-cell-positions" class="hash-link" aria-label="Direct link to Defining the x,y cell positions" translate="no" title="Direct link to Defining the x,y cell positions">â€‹</a>

As well as passing an array of x/yCellOffsets as a mapping function, it is possible to pass just arrays viaÂ Â <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/inonuniformheatmapseriesoptions.html#xcelloffsets" rel="noopener noreferrer" target="_blank">INonUniformHeatmapSeriesOptions.xCellOffsetsðŸ“˜</a> andÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/inonuniformheatmapseriesoptions.html#ycelloffsets" rel="noopener noreferrer" target="_blank">INonUniformHeatmapSeriesOptions.yCellOffsetsðŸ“˜</a>.

The function should generate cell offsets based on the index. This feature is useful when dataSeries are updated dynamically withÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/nonuniformheatmapdataseries.html#setZValues" rel="noopener noreferrer" target="_blank">NonUniformHeatmapDataSeries.setZValuesðŸ“˜</a>, which will trigger recalculation of the offsets.

### Both the following two code examples are valid:<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/non-uniform-heatmap-renderable-series/#both-the-following-two-code-examples-are-valid" class="hash-link" aria-label="Direct link to Both the following two code examples are valid:" translate="no" title="Direct link to Both the following two code examples are valid:">â€‹</a>

``` prism-code
// Passing just cell Offset Arrays
const xRangeOffsetsSource = [0, 10, 20, 26, 36, 60, 72, 84];
const yRangeOffsetsSource = [100, 250, 390, 410, 600];

const dataSeries = new NonUniformHeatmapDataSeries(wasmContext, {
    zValues,
    xCellOffsets: xRangeOffsetsSource,
    yCellOffsets: yRangeOffsetsSource
});
```

as well as this:

``` prism-code
// Passing just cell Offset Arrays
const xRangeOffsetsSource = [0, 10, 20, 26, 36, 60, 72, 84];
const yRangeOffsetsSource = [100, 250, 390, 410, 600];

const dataSeries = new NonUniformHeatmapDataSeries(wasmContext, {
    zValues,
    xCellOffsets: i => xRangeOffsetsSource[i],
    yCellOffsets: i => yRangeOffsetsSource[i]
});
```

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/non-uniform-heatmap-renderable-series/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [The Uniform Heatmap Chart Type](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/uniform-heatmap-renderable-series/uniform-heatmap-chart-type)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/non-uniform-heatmap-renderable-series/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/non-uniform-heatmap-renderable-series/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
