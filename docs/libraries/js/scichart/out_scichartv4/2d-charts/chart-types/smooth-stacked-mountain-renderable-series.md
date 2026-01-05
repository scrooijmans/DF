On this page

# The Bezier (Smoothed) Stacked Mountain Series Type

A variation on theÂ [Stacked Mountain Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/stacked-mountain-renderable-series) is the SmoothStackedMountainRenderableSeries. This uses a custom Bezier smoothing algorithm to create a smoothed line (spline line) to interpolate between datapoints.

Smoothed Stacked MountainÂ Charts can be created by a combination of theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/smoothstackedmountainrenderableseries.html" rel="noopener noreferrer" target="_blank">SmoothStackedMountainRenderableSeriesðŸ“˜</a> andÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html" rel="noopener noreferrer" target="_blank">StackedMountainCollectionðŸ“˜</a>Â types.Â 

![](out_scichartv4/2d-charts/chart-types/smooth-stacked-mountain-renderable-series/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

TheÂ <a href="https://www.scichart.com/demo/javascript/smooth-stacked-mountain-chart" rel="noopener noreferrer" target="_blank">JavaScript Smooth Stacked Mountain Chart Example</a> can be found in theÂ <a href="https://github.com/ABTSoftware/SciChart.JS.Examples/tree/master/Examples/src/components/Examples/Charts2D/BasicChartTypes/SplineMountainChart" rel="noopener noreferrer" target="_blank">SciChart.Js Examples Suite &gt; Spline Mountain Chart</a>Â on Github

Above: The JavaScript <a href="https://www.scichart.com/demo/iframe/smooth-stacked-mountain-chart" target="_blank">Smooth Stacked Mountain Chart</a> example from the <a href="https://www.scichart.com/demo/react" target="_blank">SciChart.js Demo</a>

  

## Create a Bezier Smoothed Stacked Mountain Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/smooth-stacked-mountain-renderable-series/#create-a-bezier-smoothed-stacked-mountain-series" class="hash-link" aria-label="Direct link to Create a Bezier Smoothed Stacked Mountain Series" translate="no" title="Direct link to Create a Bezier Smoothed Stacked Mountain Series">â€‹</a>

To create aÂ <a href="https://www.scichart.com/demo/javascript/smooth-stacked-mountain-chart" rel="noopener noreferrer" target="_blank">JavaScript Smoothed Stacked Mountain Series</a>, use the following code:

- TS
- Builder API (JSON Config)

``` prism-code
// Demonstrates how to create a Column chart with SciChart.js
const {
    SciChartSurface,
    NumericAxis,
    SmoothStackedMountainRenderableSeries,
    StackedMountainCollection,
    XyDataSeries,
    SciChartJsNavyTheme
} = SciChart;
// or, for npm, import { SciChartSurface, ... } from "scichart"

const { wasmContext, sciChartSurface } = await SciChartSurface.create(divElementId, {
    theme: new SciChartJsNavyTheme()
});
sciChartSurface.xAxes.add(new NumericAxis(wasmContext));
sciChartSurface.yAxes.add(new NumericAxis(wasmContext));

// Data for the example
const xValues = [1992, 1993, 1994, 1995, 1996, 1997, 1998, 1999, 2000, 2001, 2002, 2003];
const yValues1 = [10, 13, 7, 16, 4, 6, 20, 14, 16, 10, 24, 11];
const yValues2 = [12, 17, 21, 15, 19, 18, 13, 21, 22, 20, 5, 10];
const yValues3 = [7, 30, 27, 24, 21, 15, 17, 26, 22, 28, 21, 22];
const yValues4 = [16, 10, 9, 8, 22, 14, 12, 27, 25, 23, 17, 17];
const yValues5 = [7, 24, 21, 11, 19, 17, 14, 27, 26, 22, 28, 16];

// Create some RenderableSeries - for each part of the stacked m mountain
// Notice the stackedGroupId. This defines if series are stacked (same), or grouped side by side (different)
const rendSeries1 = new SmoothStackedMountainRenderableSeries(wasmContext, {
    dataSeries: new XyDataSeries(wasmContext, {
        xValues,
        yValues: yValues1,
        dataSeriesName: "EU"
    }),
    fill: "#882B91",
    stroke: "#E4F5FC",
    strokeThickness: 2,
    opacity: 0.8
});

const rendSeries2 = new SmoothStackedMountainRenderableSeries(wasmContext, {
    dataSeries: new XyDataSeries(wasmContext, {
        xValues,
        yValues: yValues2,
        dataSeriesName: "Asia"
    }),
    fill: "#EC0F6C",
    stroke: "#E4F5FC",
    strokeThickness: 2,
    opacity: 0.8
});

const rendSeries3 = new SmoothStackedMountainRenderableSeries(wasmContext, {
    dataSeries: new XyDataSeries(wasmContext, {
        xValues,
        yValues: yValues3,
        dataSeriesName: "USA"
    }),
    fill: "#F48420",
    stroke: "#E4F5FC",
    strokeThickness: 2,
    opacity: 0.8
});

const rendSeries4 = new SmoothStackedMountainRenderableSeries(wasmContext, {
    dataSeries: new XyDataSeries(wasmContext, {
        xValues,
        yValues: yValues4,
        dataSeriesName: "UK"
    }),
    fill: "#50C7E0",
    stroke: "#E4F5FC",
    strokeThickness: 2,
    opacity: 0.8
});

const rendSeries5 = new SmoothStackedMountainRenderableSeries(wasmContext, {
    dataSeries: new XyDataSeries(wasmContext, {
        xValues,
        yValues: yValues5,
        dataSeriesName: "Latam"
    }),
    fill: "#30BC9A",
    stroke: "#E4F5FC",
    strokeThickness: 2,
    opacity: 0.8
});

// To add the series to the chart, put them in a StackedMountainCollection
const stackedMountainCollection = new StackedMountainCollection(wasmContext);
stackedMountainCollection.add(rendSeries1, rendSeries2, rendSeries3, rendSeries4, rendSeries5);

// Add the Stacked Mountain collection to the chart
sciChartSurface.renderableSeries.add(stackedMountainCollection);
```

``` prism-code
// Demonstrates how to create a Mountain chart with SciChart.js using the Builder API
const { chartBuilder, ESeriesType, EThemeProviderType } = SciChart;
// or, for npm, import { chartBuilder, ... } from "scichart"

// Data for the example
const xValues = [1992, 1993, 1994, 1995, 1996, 1997, 1998, 1999, 2000, 2001, 2002, 2003];
const yValues1 = [10, 13, 7, 16, 4, 6, 20, 14, 16, 10, 24, 11];
const yValues2 = [12, 17, 21, 15, 19, 18, 13, 21, 22, 20, 5, 10];
const yValues3 = [7, 30, 27, 24, 21, 15, 17, 26, 22, 28, 21, 22];
const yValues4 = [16, 10, 9, 8, 22, 14, 12, 27, 25, 23, 17, 17];
const yValues5 = [7, 24, 21, 11, 19, 17, 14, 27, 26, 22, 28, 16];

const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    surface: { theme: { type: EThemeProviderType.Dark } },
    series: [
        // Group StackedMountain into one StackedMountainCollection and pass into the series object
        {
            type: ESeriesType.StackedMountainCollection,
            series: [
                {
                    type: ESeriesType.SmoothStackedMountainSeries,
                    options: {
                        fill: "#882B91",
                        stroke: "#E4F5FC"
                    },
                    xyData: { xValues, yValues: yValues1 }
                },
                {
                    type: ESeriesType.SmoothStackedMountainSeries,
                    options: {
                        fill: "#EC0F6C",
                        stroke: "#E4F5FC"
                    },
                    xyData: { xValues, yValues: yValues2 }
                },
                {
                    type: ESeriesType.SmoothStackedMountainSeries,
                    options: {
                        fill: "#F48420",
                        stroke: "#E4F5FC"
                    },
                    xyData: { xValues, yValues: yValues3 }
                },
                {
                    type: ESeriesType.SmoothStackedMountainSeries,
                    options: {
                        fill: "#50C7E0",
                        stroke: "#E4F5FC"
                    },
                    xyData: { xValues, yValues: yValues4 }
                },
                {
                    type: ESeriesType.SmoothStackedMountainSeries,
                    options: {
                        fill: "#30BC9A",
                        stroke: "#E4F5FC"
                    },
                    xyData: { xValues, yValues: yValues5 }
                }
            ]
        }
    ]
});
```

Above:

- We created 3Â <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html" rel="noopener noreferrer" target="_blank">StackedMountainRenderableSeriesðŸ“˜</a> and added them to aÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html" rel="noopener noreferrer" target="_blank">StackedMountainCollectionðŸ“˜</a>
- The StackedMountainCollection itself is added toÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#renderableseries" rel="noopener noreferrer" target="_blank">sciChartSurface.renderableSeriesðŸ“˜</a> collection, not the individual mountain series.

This results in the following output:

## Creating 100% Stacked Mountain Charts<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/smooth-stacked-mountain-renderable-series/#creating-100-stacked-mountain-charts" class="hash-link" aria-label="Direct link to Creating 100% Stacked Mountain Charts" translate="no" title="Direct link to Creating 100% Stacked Mountain Charts">â€‹</a>

SciChart.js also supports aÂ JavaScript 100% Smoothed Stacked Mountain chart, which can be enabled by setting a single flag:Â <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#isonehundredpercent" rel="noopener noreferrer" target="_blank">StackedMountainCollection.isOneHundredPercentðŸ“˜</a>.

- TS

``` prism-code
// ...
const rendSeries5 = new SmoothStackedMountainRenderableSeries(wasmContext, {
    dataSeries: new XyDataSeries(wasmContext, {
        xValues,
        yValues: yValues5,
        dataSeriesName: "Latam"
    }),
    fill: "#30BC9A",
    stroke: "#E4F5FC",
    strokeThickness: 2,
    opacity: 0.8
});

// To add the series to the chart, put them in a StackedMountainCollection
const stackedMountainCollection = new StackedMountainCollection(wasmContext);
// Set the isOneHundredPercent option to enable 100% stacking
stackedMountainCollection.isOneHundredPercent = true;
stackedMountainCollection.add(rendSeries1, rendSeries2, rendSeries3, rendSeries4, rendSeries5);

// Add the Stacked Mountain collection to the chart
sciChartSurface.renderableSeries.add(stackedMountainCollection);
// ...
```

Setting this flag results in the following output.

***Above:** aÂ JavaScript 100% Smoothed Stacked Mountain chart whenÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#isonehundredpercent" rel="noopener noreferrer" target="_blank">StackedMountainCollection.isOneHundredPercentðŸ“˜</a> is true*

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/smooth-stacked-mountain-renderable-series/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [Stacked Mountain Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/stacked-mountain-renderable-series)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/smooth-stacked-mountain-renderable-series/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/smooth-stacked-mountain-renderable-series/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
