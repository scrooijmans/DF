On this page

# Creating a Simple Chart

## Creating a Series with the Builder API<a href="https://www.scichart.com/documentation/js/v4/2d-charts/builder-api/simple-chart/#creating-a-series-with-the-builder-api" class="hash-link" aria-label="Direct link to Creating a Series with the Builder API" translate="no" title="Direct link to Creating a Series with the Builder API">â€‹</a>

Letâ€™s add a series. A definition must have a type property which is an <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html" rel="noopener noreferrer" target="_blank">ESeriesTypeðŸ“˜</a>. Weâ€™ll go into options in more detail later. The data property tells us the shape of data we need. There are a few different ways to supply data, but the simplest is to set the values directly here.

- TS

``` prism-code
const {
    chartBuilder,
    ESeriesType
} = SciChart;

const { sciChartSurface, wasmContext } = await chartBuilder.buildChart(divElementId, {
    series: {
        type: ESeriesType.LineSeries,
        xyData: {
            xValues: [1, 3, 4, 7, 9],
            yValues: [10, 6, 7, 2, 16]
        }
    }
});
```

This is all we need for a working SciChart chart with our BuilderAPI.

Notice that the return type of the **chartBuilder.buildChart()** function call is a Promise, which returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html" rel="noopener noreferrer" target="_blank">SciChartSurfaceðŸ“˜</a> and **wasmContext** just like a call to **SciChartSurface.create()**, except it will already be populated with series, default axes and data provided in the builder options.

Run the code sample above and you will get this output:

<img src="out_scichartv4/2d-charts/builder-api/simple-chart/index_media/8fca14ba9497b8b828afcddd22495ffd751997b3.png" style="display:block;margin-left:auto;margin-right:auto;object-fit:contain;height:auto;width:85%;margin:0 auto" />

## Customizing Axis, Theme and Modifiers with the Builder API<a href="https://www.scichart.com/documentation/js/v4/2d-charts/builder-api/simple-chart/#customizing-axis-theme-and-modifiers-with-the-builder-api" class="hash-link" aria-label="Direct link to Customizing Axis, Theme and Modifiers with the Builder API" translate="no" title="Direct link to Customizing Axis, Theme and Modifiers with the Builder API">â€‹</a>

To customise the axes you again need to specify the axis type, then you can set options. The theme can be set in the surface. The theme can be a full theme class, or you can just refer to it by type.

Use the following code sample to try these out:

- TS

``` prism-code
const {
    chartBuilder,
    ESeriesType,
    EAxisType,
    EChart2DModifierType
} = SciChart;

const { sciChartSurface, wasmContext } = await chartBuilder.buildChart(divElementId, {
    surface: {
        theme: {
            type: EThemeProviderType.Light
        }
    },
    series: {
        type: ESeriesType.LineSeries,
        options: { stroke: "blue" },
        xyData: {
            xValues: [1, 3, 4, 7, 9],
            yValues: [10, 6, 7, 2, 16]
        }
    },
    yAxes: {
        type: EAxisType.NumericAxis,
        options: {
            axisTitle: "Number of things",
            visibleRange: new NumberRange(0, 20),
            labelPrecision: 0
        }
    },
    modifiers: [
        { type: EChart2DModifierType.Rollover },
        { type: EChart2DModifierType.ZoomPan }
    ]
});
```

This results in the following chart output:

<img src="out_scichartv4/2d-charts/builder-api/simple-chart/index_media/1a42ad43d107d2836f7d89a2704da3a0983793d7.png" style="display:block;margin-left:auto;margin-right:auto;object-fit:contain;height:auto;width:85%;margin:0 auto" />

Note that visible range for an axis needs to be supplied as a <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" rel="noopener noreferrer" target="_blank">NumberRangeðŸ“˜</a> class, however, in a text definition you can write `visibleRange: { min: 0, max: 20 }` .

Chart Modifiers, like pretty much everything, are `{ type, options }`. Note that series, axes, annotations and modifiers can all be a single definition, or an array.

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/builder-api/simple-chart/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [Intro to the Builder API](https://www.scichart.com/documentation/js/v4/2d-charts/builder-api/builder-api-overview)
- [Working with Data](https://www.scichart.com/documentation/js/v4/2d-charts/builder-api/working-with-data)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/builder-api/simple-chart/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/builder-api/simple-chart/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
