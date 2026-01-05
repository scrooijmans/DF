On this page

# Horizontally Stacked Axis Layout

The Stacked Axis feature in SciChart allows you to specify the layout of the axis panel. Normally when you have multipleÂ XAxis, they are stacked vertically. However, you can switch this to stack horizontally. Custom and complex layouts are possible allowing for all kinds of chart scenarios.

![](out_scichartv4/2d-charts/axis-api/multi-axis-and-layout/horizontally-stacked-axis-layout/index_media/0f0874621de662630fd5d7f09c89455c4ace26e6.svg)warning

[Polar Charts](https://www.scichart.com/documentation/js/v4/2d-charts/surface/scichart-polar-surface-type) do not support stacked axes yet

In theÂ [previous article we demonstrated Vertically Stacked Axis](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/multi-axis-and-layout/vertically-stacked-axis-layout). This is where you specify a layout strategy for Y Axis on the left or right of the chart to stack axis above each other.

## Create a Horizontally Stacked Axis Chart<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/multi-axis-and-layout/horizontally-stacked-axis-layout/#create-a-horizontally-stacked-axis-chart" class="hash-link" aria-label="Direct link to Create a Horizontally Stacked Axis Chart" translate="no" title="Direct link to Create a Horizontally Stacked Axis Chart">â€‹</a>

### Step 1: Create a Multi X-Axis Chart<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/multi-axis-and-layout/horizontally-stacked-axis-layout/#step-1-create-a-multi-x-axis-chart" class="hash-link" aria-label="Direct link to Step 1: Create a Multi X-Axis Chart" translate="no" title="Direct link to Step 1: Create a Multi X-Axis Chart">â€‹</a>

Typically if you create a chart with several X-Axis, they are stacked on the top or bottom of the chart.

The following code withÂ 4 XAxis on theÂ bottom results in this output:

- TS
- Builder API (JSON Config)

``` prism-code
// Create an YAxis on the Left
sciChartSurface.yAxes.add(
    new NumericAxis(wasmContext, {
        axisTitle: "Y Axis",
        axisTitleStyle: { fontSize: 13 },
        backgroundColor: "#50C7E022",
        axisBorder: { color: "#50C7E0", borderRight: 1 },
        axisAlignment: EAxisAlignment.Left,
        growBy: new NumberRange(0.1, 0.1)
    })
);

// Create several XAxis on the bottom
sciChartSurface.xAxes.add(new NumericAxis(wasmContext, { axisTitle: "X Axis 0" }));
sciChartSurface.xAxes.add(new NumericAxis(wasmContext, { axisTitle: "X Axis 1" }));
sciChartSurface.xAxes.add(new NumericAxis(wasmContext, { axisTitle: "X Axis 2" }));
sciChartSurface.xAxes.add(new NumericAxis(wasmContext, { axisTitle: "X Axis 3" }));

// To make it clearer what's happening, colour the axis backgrounds & borders
const axisColors = ["#50C7E0", "#EC0F6C", "#30BC9A", "#F48420"];
sciChartSurface.xAxes.asArray().forEach((xAxis, index) => {
    xAxis.backgroundColor = axisColors[index] + "22";
    xAxis.axisBorder = { color: axisColors[index], borderTop: 1 };
    xAxis.axisTitleStyle.fontSize = 13;
});

// Let's add some series to the chart to show how they also behave with axis
const getOptions = index => {
    const xValues = Array.from(Array(50).keys());
    const yValues = xValues.map(x => Math.sin(x * 0.4 + index));

    return {
        xAxisId: sciChartSurface.xAxes.asArray()[index].id,
        stroke: axisColors[index],
        strokeThickness: 2,
        dataSeries: new XyDataSeries(wasmContext, { xValues, yValues })
    };
};

sciChartSurface.renderableSeries.add(new FastLineRenderableSeries(wasmContext, { ...getOptions(0) }));
sciChartSurface.renderableSeries.add(new FastLineRenderableSeries(wasmContext, { ...getOptions(1) }));
sciChartSurface.renderableSeries.add(new FastLineRenderableSeries(wasmContext, { ...getOptions(2) }));
sciChartSurface.renderableSeries.add(new FastLineRenderableSeries(wasmContext, { ...getOptions(3) }));
```

``` prism-code
const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    surface: { theme: { type: EThemeProviderType.Dark } },
    yAxes: {
        type: EAxisType.NumericAxis,
        options: {
            axisTitle: "Y Axis",
            axisTitleStyle: { fontSize: 13 },
            backgroundColor: "#50C7E022",
            axisBorder: { color: "#50C7E0", borderRight: 1 },
            axisAlignment: EAxisAlignment.Left
        }
    },
    xAxes: [
        {
            type: EAxisType.NumericAxis,
            options: {
                id: "XAxis0",
                axisTitle: "X Axis 0",
                backgroundColor: "#50C7E022",
                axisBorder: { borderTop: 1, color: "#50C7E0" }
            }
        },
        {
            type: EAxisType.NumericAxis,
            options: {
                id: "XAxis1",
                axisTitle: "X Axis 1",
                backgroundColor: "#EC0F6C22",
                axisBorder: { borderTop: 1, color: "#EC0F6C" }
            }
        },
        {
            type: EAxisType.NumericAxis,
            options: {
                id: "XAxis2",
                axisTitle: "X Axis 2",
                backgroundColor: "#30BC9A22",
                axisBorder: { borderTop: 1, color: "#30BC9A" }
            }
        },
        {
            type: EAxisType.NumericAxis,
            options: {
                id: "XAxis3",
                axisTitle: "X Axis 3",
                backgroundColor: "#F4842022",
                axisBorder: { borderTop: 1, color: "#F48420" }
            }
        }
    ],
    series: [
        {
            type: ESeriesType.LineSeries,
            options: { stroke: "#50C7E0", strokeThickness: 2, xAxisId: "XAxis0" },
            xyData: { xValues, yValues }
        },
        {
            type: ESeriesType.LineSeries,
            options: { stroke: "#EC0F6C", strokeThickness: 2, xAxisId: "XAxis1" },
            xyData: { xValues, yValues: yValues1 }
        },
        {
            type: ESeriesType.LineSeries,
            options: { stroke: "#30BC9A", strokeThickness: 2, xAxisId: "XAxis2" },
            xyData: { xValues, yValues: yValues2 }
        },
        {
            type: ESeriesType.LineSeries,
            options: { stroke: "#F48420", strokeThickness: 2, xAxisId: "XAxis3" },
            xyData: { xValues, yValues: yValues3 }
        }
    ]
});
```

### Step 2: Apply the Layout Strategy<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/multi-axis-and-layout/horizontally-stacked-axis-layout/#step-2-apply-the-layout-strategy" class="hash-link" aria-label="Direct link to Step 2: Apply the Layout Strategy" translate="no" title="Direct link to Step 2: Apply the Layout Strategy">â€‹</a>

To change the behaviour of axis stacking you need to set the appropriate layoutStrategy property on theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#layoutmanager" rel="noopener noreferrer" target="_blank">SciChartSurface.LayoutManagerðŸ“˜</a>Â with the stacked version.Â 

SciChart provides the following Outer Axes Layout Strategies:

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/leftalignedouterverticallystackedaxislayoutstrategy.html" rel="noopener noreferrer" target="_blank">LeftAlignedOuterVerticallyStackedAxisLayoutStrategyðŸ“˜</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rightalignedouterverticallystackedaxislayoutstrategy.html" rel="noopener noreferrer" target="_blank">RightAlignedOuterVerticallyStackedAxisLayoutStrategyðŸ“˜</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/topalignedouterhorizontallystackedaxislayoutstrategy.html" rel="noopener noreferrer" target="_blank">TopAlignedOuterHorizontallyStackedAxisLayoutStrategyðŸ“˜</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/bottomalignedouterhorizontallystackedaxislayoutstrategy.html" rel="noopener noreferrer" target="_blank">BottomAlignedOuterHorizontallyStackedAxisLayoutStrategyðŸ“˜</a>

Modify the code above to set this property on theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#layoutmanager" rel="noopener noreferrer" target="_blank">SciChartSurface.LayoutManagerðŸ“˜</a>:

- TS

``` prism-code
// Enable stacking of axis
sciChartSurface.layoutManager.bottomOuterAxesLayoutStrategy =
    new BottomAlignedOuterHorizontallyStackedAxisLayoutStrategy();
```

Now the layout isÂ completely changed.Â 

## LayoutStrategies Applicable to X-Axis<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/multi-axis-and-layout/horizontally-stacked-axis-layout/#layoutstrategies-applicable-to-x-axis" class="hash-link" aria-label="Direct link to LayoutStrategies Applicable to X-Axis" translate="no" title="Direct link to LayoutStrategies Applicable to X-Axis">â€‹</a>

The followingÂ horizontally stackedÂ layout strategies are available and may be applied to the following properties onÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#layoutmanager" rel="noopener noreferrer" target="_blank">SciChartSurface.LayoutManagerðŸ“˜</a>:

| Layout Strategy | Use With | Apply to LayoutManager Prop | Behavior |
|----|----|----|----|
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/topalignedouteraxislayoutstrategy.html" rel="noopener noreferrer" target="_blank">TopAlignedOuterAxisLayoutStrategyðŸ“˜</a> | X Axis | <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/layoutmanager.html#topinneraxeslayoutstrategy" rel="noopener noreferrer" target="_blank">topInnerAxisLayoutStrategyðŸ“˜</a>, <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/layoutmanager.html#topouteraxeslayoutstrategy" rel="noopener noreferrer" target="_blank">topOuterAxisLayoutStrategyðŸ“˜</a> | Default behavior |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/bottomalignedouteraxislayoutstrategy.html" rel="noopener noreferrer" target="_blank">BottomAlignedOuterAxisLayoutStrategyðŸ“˜</a> | X Axis | <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/layoutmanager.html#bottominneraxeslayoutstrategy" rel="noopener noreferrer" target="_blank">bottomInnerAxisLayoutStrategyðŸ“˜</a>, <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/layoutmanager.html#bottomouteraxeslayoutstrategy" rel="noopener noreferrer" target="_blank">bottomOuterAxisLayoutStrategyðŸ“˜</a> | Default behavior |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/topalignedouterhorizontallystackedaxislayoutstrategy.html" rel="noopener noreferrer" target="_blank">TopAlignedOuterHorizontallyStackedAxisLayoutStrategyðŸ“˜</a> | X Axis | <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/layoutmanager.html#topouteraxeslayoutstrategy" rel="noopener noreferrer" target="_blank">topOuterAxisLayoutStrategyðŸ“˜</a> | Horizontal stacking behavior |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/bottomalignedouterhorizontallystackedaxislayoutstrategy.html" rel="noopener noreferrer" target="_blank">BottomAlignedOuterHorizontallyStackedAxisLayoutStrategyðŸ“˜</a> | X Axis | <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/layoutmanager.html#bottomouteraxeslayoutstrategy" rel="noopener noreferrer" target="_blank">bottomOuterAxisLayoutStrategyðŸ“˜</a> | Horizontal stacking behavior |

![](out_scichartv4/2d-charts/axis-api/multi-axis-and-layout/horizontally-stacked-axis-layout/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

Try experimenting with the Codepen above to see how each of the strategies behave. Note that a **TopLayoutStrategy**Â will requireÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numericaxis.html#axisalignment" rel="noopener noreferrer" target="_blank">Axis.axisAlignmentðŸ“˜</a> =Â <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eaxisalignment.html" rel="noopener noreferrer" target="_blank">EAxisAlignment.TopðŸ“˜</a> and vice versa.

## CustomisingÂ Axis Size when Horizontally Stacked<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/multi-axis-and-layout/horizontally-stacked-axis-layout/#customisingaxis-size-when-horizontally-stacked" class="hash-link" aria-label="Direct link to CustomisingÂ Axis Size when Horizontally Stacked" translate="no" title="Direct link to CustomisingÂ Axis Size when Horizontally Stacked">â€‹</a>

TheÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase2d.html#stackedaxislength" rel="noopener noreferrer" target="_blank">Axis.stackedAxisLengthðŸ“˜</a>Â property allows you to customize the size of a Horizontally Stacked Axis in SciChart.js. This property may be an absolute number, e.g. 50 pixels, or a percentage e.g. "30%". When left undefined, default equal spacing will be used.

Try the following code to see how it affects stacked axis size.

- stackedAxisLength

``` prism-code
// This stacked axis has 100 pixel length
sciChartSurface.xAxes.add(new NumericAxis(wasmContext, { axisTitle: "X Axis 0", stackedAxisLength: 100 }));
// This stacked axis occupies 50% of available space
sciChartSurface.xAxes.add(new NumericAxis(wasmContext, { axisTitle: "X Axis 1", stackedAxisLength: "50%" }));
// These stacked axis obey default spacing
sciChartSurface.xAxes.add(new NumericAxis(wasmContext, { axisTitle: "X Axis 2" }));
sciChartSurface.xAxes.add(new NumericAxis(wasmContext, { axisTitle: "X Axis 3" }));
```

## Combining Vertical (rotated) Charts & Stacked Axis<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/multi-axis-and-layout/horizontally-stacked-axis-layout/#combining-vertical-rotated-charts--stacked-axis" class="hash-link" aria-label="Direct link to Combining Vertical (rotated) Charts &amp; Stacked Axis" translate="no" title="Direct link to Combining Vertical (rotated) Charts &amp; Stacked Axis">â€‹</a>

Part of the magic of SciChart.js is the sheer number of combinations you can have for chart and axis layout!

If we combine theÂ [Vertical Chart feature](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/multi-axis-and-layout/vertical-charts-rotate-transpose-axis) where you set **XAxis.axisAlignment** = Left and **YAxis.axisAlignment** = Top with the Horizontally Stacked Axis feature where we can re-arrange the layout of axis on the top/bottom of the chart, we can achieve things like this:

- TS

``` prism-code
sciChartSurface.layoutManager.topOuterAxesLayoutStrategy =
    new TopAlignedOuterHorizontallyStackedAxisLayoutStrategy();

// Create an XAxis on the left
sciChartSurface.xAxes.add(
    new NumericAxis(wasmContext, {
        axisTitle: "Rotated X Axis",
        axisTitleStyle: { fontSize: 13 },
        backgroundColor: "#50C7E022",
        axisBorder: { color: "#50C7E0", borderRight: 1 },
        axisAlignment: EAxisAlignment.Left
    })
);

// Create several Y-Axis on the Top
const axisAlignment = EAxisAlignment.Top;
sciChartSurface.yAxes.add(
    new NumericAxis(wasmContext, { axisTitle: "Rotated, Stacked Y Axis 0", axisAlignment })
);
sciChartSurface.yAxes.add(
    new NumericAxis(wasmContext, { axisTitle: "Rotated, Stacked Y Axis 1", axisAlignment })
);
sciChartSurface.yAxes.add(
    new NumericAxis(wasmContext, { axisTitle: "Rotated, Stacked Y Axis 2", axisAlignment })
);
sciChartSurface.yAxes.add(
    new NumericAxis(wasmContext, { axisTitle: "Rotated, Stacked Y Axis 3", axisAlignment })
);
```

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/multi-axis-and-layout/horizontally-stacked-axis-layout/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [Inner Axis Layout](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/multi-axis-and-layout/inner-axis-layout)
- [Secondary and Multiple Axis](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/multi-axis-and-layout/secondary-and-multiple-axis-overview)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/axis-api/multi-axis-and-layout/horizontally-stacked-axis-layout/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/axis-api/multi-axis-and-layout/horizontally-stacked-axis-layout/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
