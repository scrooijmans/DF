On this page

# The Uniform Heatmap Chart Type

Uniform Heatmaps can be created using theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/uniformheatmaprenderableseries.html" rel="noopener noreferrer" target="_blank">UniformHeatmapRenderableSeriesðŸ“˜</a> type.

![](out_scichartv4/2d-charts/chart-types/uniform-heatmap-renderable-series/uniform-heatmap-chart-type/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

TheÂ <a href="https://www.scichart.com/demo/javascript/heatmap-chart" rel="noopener noreferrer" target="_blank">JavaScript Heatmap Chart Example</a> can be found in theÂ <a href="https://github.com/ABTSoftware/SciChart.JS.Examples/tree/master/Examples/src/components/Examples/Charts2D/BasicChartTypes/HeatmapChart" rel="noopener noreferrer" target="_blank">SciChart.Js Examples Suite &gt; Heatmap Chart</a>Â on Github, or our live demo atÂ <a href="https://www.scichart.com/demo/javascript/heatmap-chart" rel="noopener noreferrer" target="_blank">scichart.com/demo</a>.

Above: The JavaScript <a href="https://www.scichart.com/demo/iframe/heatmap-chart" target="_blank">Uniform Heatmap Series Chart</a> example from the <a href="https://www.scichart.com/demo/react" target="_blank">SciChart.js Demo</a>

  

## Create aÂ Uniform Heatmap<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/uniform-heatmap-renderable-series/uniform-heatmap-chart-type/#create-auniform-heatmap" class="hash-link" aria-label="Direct link to Create aÂ Uniform Heatmap" translate="no" title="Direct link to Create aÂ Uniform Heatmap">â€‹</a>

Uniform heatmaps are extremely fast, lightweight series types for rendering two dimensional data as a heatmap or spectrogram. TheÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/uniformheatmaprenderableseries.html" rel="noopener noreferrer" target="_blank">UniformHeatmapRenderableSeriesðŸ“˜</a>Â type should be used in conjunction with aÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/uniformheatmapdataseries.html" rel="noopener noreferrer" target="_blank">UniformHeatmapDataSeriesðŸ“˜</a>Â when you simply want to specify a Step in the X,Y direction (each cell is the same size).

To create aÂ <a href="https://www.scichart.com/demo/javascript-heatmap-chart" rel="noopener noreferrer" target="_blank">Javascript Heatmap Chart</a>Â with SciChart.js, use the following code:

### Creating the Imports<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/uniform-heatmap-renderable-series/uniform-heatmap-chart-type/#creating-the-imports" class="hash-link" aria-label="Direct link to Creating the Imports" translate="no" title="Direct link to Creating the Imports">â€‹</a>

First, let's setup the imports that we need for the heatmap type.

- TS

``` prism-code
// Demonstrates how to create a uniform heatmap chart with SciChart.js
const {
    SciChartSurface,
    NumericAxis,
    HeatmapColorMap,
    UniformHeatmapDataSeries,
    UniformHeatmapRenderableSeries,
    SciChartJsNavyTheme
} = SciChart;
// or, for npm, import { SciChartSurface, ... } from "scichart"
```

### Creating Heatmap ZValues\[ \]\[ \] Array<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/uniform-heatmap-renderable-series/uniform-heatmap-chart-type/#creating-heatmap-zvalues---array" class="hash-link" aria-label="Direct link to Creating Heatmap ZValues[ ][ ] Array" translate="no" title="Direct link to Creating Heatmap ZValues[ ][ ] Array">â€‹</a>

Next, we want to create a 2-dimensional array of data. Heatmap data is a 2D number array (type `number[][]` in Typescript) which contains the heat values. These are later mapped to colours in the heatmap.

The dimensions of the zValues 2D array are \[height\]\[width\]

- TS

``` prism-code
// Create some data for the heatmap as a 2d array
// e.g.
// const zValues = [
//   [0, 2, 3.4, 2, 1, 5],
//   [5, 3, 4, 2.2, 7, 4.4],
//   [3, 1.5, 1, 3, 4, 6.4],
//   [2, 1.2, 5.4, 4, 3, 5],
// ];
//
const heatmapWidth = 7;
const heatmapHeight = 4;
const zValues = Array.from(Array(heatmapHeight));
zValues.forEach((row, index, collection) => {
    collection[index] = Array.from(Array(heatmapWidth));
});
for (let x = 0; x < heatmapWidth; x++) {
    for (let y = 0; y < heatmapHeight; y++) {
        zValues[y][x] = 3.5 * ((heatmapHeight - y) * (heatmapWidth - x));
    }
}
```

### Creating the Heatmap Instance<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/uniform-heatmap-renderable-series/uniform-heatmap-chart-type/#creating-the-heatmap-instance" class="hash-link" aria-label="Direct link to Creating the Heatmap Instance" translate="no" title="Direct link to Creating the Heatmap Instance">â€‹</a>

Finally, we create theÂ UniformHeatmapRenderableSeries type, which has both aÂ UniformHeatmapDataSeries for the data and aÂ HeatmapColorMap to map zValues to colors.

Here's a full example below:

- TS
- Builder API (JSON Config)

``` prism-code
// Create the uniform heatmap series
const heatmapSeries = new UniformHeatmapRenderableSeries(wasmContext, {
    dataSeries: new UniformHeatmapDataSeries(wasmContext, {
        // 2d zValues array. Dimensions [height][width]
        zValues,
        // xStart, xStep, yStart, yStep defines the x,y position
        xStart: 10,
        xStep: 1,
        yStart: 10,
        yStep: 1
    }),
    // zValues mapped to colours using the colorMap.
    // zValue[y][x] when compared to HeatmapColorMap.maximum corresponds to gradientstop offset=1
    // zValue[y][x] when compared to HeatmapColorMap.minimum corresponds to gradientstop offset=0
    colorMap: new HeatmapColorMap({
        minimum: 0,
        maximum: 100,
        gradientStops: [
            { offset: 1, color: "#EC0F6C" },
            { offset: 0.9, color: "#F48420" },
            { offset: 0.7, color: "#DC7969" },
            { offset: 0.5, color: "#67BDAF" },
            { offset: 0.3, color: "#50C7E0" },
            { offset: 0.2, color: "#264B93" },
            { offset: 0, color: "#14233C" }
        ]
    }),
    // Optional datalabels may be placed in cell
    dataLabels: {
        style: { fontFamily: "Default", fontSize: 16 },
        color: "White"
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
        type: ESeriesType.UniformHeatmapSeries,
        options: { 
            colorMap: new HeatmapColorMap({
                minimum: 0,
                maximum: 4,
                gradientStops: [
                    { offset: 0, color: "yellow" },
                    { offset: 0.5, color: "blue" },
                    { offset: 1, color: "red" }
                ]
            }),
            dataLabels: {
                style: { fontFamily: "Default", fontSize: 16 },
                color: "White"
            }
        },
        heatmapData: {
            zValues: [
                [0, 2, 3.4],
                [5, 3, 4],
                [3, 1.5, -1]
            ],
            xStart: 10,
            xStep: 1,
            yStart: 10,
            yStep: 1
        }
    }
});
```

This results in the following output:

![](out_scichartv4/2d-charts/chart-types/uniform-heatmap-renderable-series/uniform-heatmap-chart-type/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

Click **'Edit on CodePen'** in the example above to see the full example in your browser!

## Adding Data Labels (Text Labels) to Heatmap Cells<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/uniform-heatmap-renderable-series/uniform-heatmap-chart-type/#adding-data-labels-text-labels-to-heatmap-cells" class="hash-link" aria-label="Direct link to Adding Data Labels (Text Labels) to Heatmap Cells" translate="no" title="Direct link to Adding Data Labels (Text Labels) to Heatmap Cells">â€‹</a>

![](out_scichartv4/2d-charts/chart-types/uniform-heatmap-renderable-series/uniform-heatmap-chart-type/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

In SciChart.js v3 and above, you can now add data labels (text labels) to heatmap cells. To do this, you specify the HeatmapRenderableSeries.dataLabels property. Data Labels will automatically hide if the cell size is too small. If you can't see a data label, zoom in to ensure that it shows.

``` prism-code
// ...
const heatmapSeries = new UniformHeatmapRenderableSeries(wasmContext, {
    dataSeries: heatmapDataSeries,
    colorMap: heatmapColorMap,
    dataLabels: {
        style: {
            fontFamily: "Arial",
            fontSize: 16,
        },
        color: appTheme.ForegroundColor
    }
});
```

## Max Heatmap Size and Tiling Heatmaps<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/uniform-heatmap-renderable-series/uniform-heatmap-chart-type/#max-heatmap-size-and-tiling-heatmaps" class="hash-link" aria-label="Direct link to Max Heatmap Size and Tiling Heatmaps" translate="no" title="Direct link to Max Heatmap Size and Tiling Heatmaps">â€‹</a>

In SciChart.js the maximum heatmap size (NxM size of the 2-dimensional array) is determined byÂ <a href="https://stackoverflow.com/a/46109824" rel="noopener noreferrer" target="_blank">WebGL gl.MAX_TEXTURE_SIZE</a>. This will be a different value depending on the GPU hardware, the browser and operating system. On a Windows PC Running in Chrome `gl.MAX_TEXTURE_SIZE is 16,384 x 16,384` but could be as low as `2048 x 2048` on other devices.

For viewing massive heatmaps, SciChart.js allows tiling of heatmaps by placing multiple UniformHeatmapRenderableSeries onto the same SciChartSurface. Each heatmap can be positioned using `xStart`, `xStep`, `yStart`, `yStep` constructor parameters. This allows you to render very large datasets in browser and is how one of our users achieved this output: medical imaging using SciChart's heatmap feature.

## How to insert gaps (transparent cells) in heatmap using NaN<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/uniform-heatmap-renderable-series/uniform-heatmap-chart-type/#how-to-insert-gaps-transparent-cells-in-heatmap-using-nan" class="hash-link" aria-label="Direct link to How to insert gaps (transparent cells) in heatmap using NaN" translate="no" title="Direct link to How to insert gaps (transparent cells) in heatmap using NaN">â€‹</a>

This section describes how to insert transparent cells (gaps, not a number, NaNs, null cells, empty cells) into <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/uniformheatmaprenderableseries.html" rel="noopener noreferrer" target="_blank">UniformHeatmapRenderableSeriesðŸ“˜</a>. In order to insert gaps we need to add `NaN` values into `zValues` array and to set flag `dataSeries.hasNaNs = true`.

For <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/uniformheatmapdataseries.html" rel="noopener noreferrer" target="_blank">UniformHeatmapDataSeriesðŸ“˜</a> this can be done by passing <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iuniformheatmapseriesoptions.html#zvalues" rel="noopener noreferrer" target="_blank">zValue optionsðŸ“˜</a> in the constructor.

![](out_scichartv4/2d-charts/chart-types/uniform-heatmap-renderable-series/uniform-heatmap-chart-type/index_media/af50301a53fa555616a9b2279e7e25a1d566cb1a.svg)info

A similar approach works for [Non-Uniform Heatmap Chart Type](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/non-uniform-heatmap-renderable-series/).

``` prism-code
    const { sciChartSurface, wasmContext } = await SciChartSurface.create(divElementId2);
    sciChartSurface.xAxes.add(
        new NumericAxis(wasmContext, {
            axisTitle: "Heatmap X"
        })
    );
    sciChartSurface.yAxes.add(
        new NumericAxis(wasmContext, {
            axisTitle: "Heatmap Y",
            axisAlignment: EAxisAlignment.Left
        })
    );

    const gradientStops = [
        { offset: 0, color: "yellow" },
        { offset: 0.5, color: "green" },
        { offset: 1, color: "red" }
    ];
    const colorMap = new HeatmapColorMap({
        minimum: 1,
        maximum: 3,
        gradientStops
    });

    const zValues = [
        [NaN, NaN, 1, 2],
        [2, 1, 2, 3]
    ];

    const dataSeries = new UniformHeatmapDataSeries(wasmContext, {
        xStart: 0,
        xStep: 1,
        yStart: 3,
        yStep: 3,
        zValues
    });
    dataSeries.hasNaNs = true;

    const heatmapSeries = new UniformHeatmapRenderableSeries(wasmContext, {
        opacity: 0.5,
        dataSeries,
        colorMap
    });

    sciChartSurface.renderableSeries.add(heatmapSeries);
    return { sciChartSurface, wasmContext };
```

This is the result

<img src="out_scichartv4/2d-charts/chart-types/uniform-heatmap-renderable-series/uniform-heatmap-chart-type/index_media/7b133837318fccca1a7cb76b839b88652b7de33b.png" class="img_ev3q" decoding="async" loading="lazy" width="1057" height="715" />

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/uniform-heatmap-renderable-series/uniform-heatmap-chart-type/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [Updating (Realtime) Heatmaps](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/uniform-heatmap-renderable-series/updating-realtime)
- [Color Maps and Legends of Heatmap](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/uniform-heatmap-renderable-series/color-maps-and-legends)
- [Start Here - RenderableSeries Overview](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/renderable-series-api-overview)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/uniform-heatmap-renderable-series/uniform-heatmap-chart-type/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/uniform-heatmap-renderable-series/uniform-heatmap-chart-type/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
