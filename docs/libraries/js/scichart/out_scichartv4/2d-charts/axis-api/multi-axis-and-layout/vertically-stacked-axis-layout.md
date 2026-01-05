On this page

# Vertically Stacked Axis Layout

The Stacked Axis feature in SciChart allows you to specify the layout of the axis panel. Normally when you have multipleÂ YAxis, they are stacked horizontally. However, you can switch this to stack vertically. Custom and complex layouts are possible allowing for all kinds of chart scenarios.

![](out_scichartv4/2d-charts/axis-api/multi-axis-and-layout/vertically-stacked-axis-layout/index_media/0f0874621de662630fd5d7f09c89455c4ace26e6.svg)warning

[Polar Charts](https://www.scichart.com/documentation/js/v4/2d-charts/surface/scichart-polar-surface-type) do not support stacked axes yet

Above: The JavaScript <a href="https://www.scichart.com/demo/iframe/javascript-vertically-stacked-axes" target="_blank">Vertically Stacked Axis</a> example from the <a href="https://www.scichart.com/demo/react" target="_blank">SciChart.js Demo</a>

  

## Create a Vertically Stacked Axis Chart<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/multi-axis-and-layout/vertically-stacked-axis-layout/#create-a-vertically-stacked-axis-chart" class="hash-link" aria-label="Direct link to Create a Vertically Stacked Axis Chart" translate="no" title="Direct link to Create a Vertically Stacked Axis Chart">â€‹</a>

### Step 1: Create a multiple Y-Axis Chart<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/multi-axis-and-layout/vertically-stacked-axis-layout/#step-1-create-a-multiple-y-axis-chart" class="hash-link" aria-label="Direct link to Step 1: Create a multiple Y-Axis Chart" translate="no" title="Direct link to Step 1: Create a multiple Y-Axis Chart">â€‹</a>

Typically if you create a chart with several Y-Axis, they are stacked on the left or right of the chart.

The following code with 8 YAxis on theÂ left results in this output:

- TS
- Builder API (JSON Config)

``` prism-code
// Create an XAxis on the bottom
sciChartSurface.xAxes.add(
    new NumericAxis(wasmContext, {
        axisTitle: "X Axis",
        axisTitleStyle: { fontSize: 13 },
        backgroundColor: "#50C7E022",
        axisBorder: { color: "#50C7E0", borderTop: 1 }
    })
);

// Create several YAxis on the left
// Creating a NumericAxis as a YAxis on the left
const yAxis0 = new NumericAxis(wasmContext, {
    axisTitle: "Y Axis 0",
    axisAlignment: EAxisAlignment.Left
});
sciChartSurface.yAxes.add(yAxis0);
const yAxis1 = new NumericAxis(wasmContext, {
    axisTitle: "Y Axis 1",
    axisAlignment: EAxisAlignment.Left
});
sciChartSurface.yAxes.add(yAxis1);
const yAxis2 = new NumericAxis(wasmContext, {
    axisTitle: "Y Axis 2",
    axisAlignment: EAxisAlignment.Left
});
sciChartSurface.yAxes.add(yAxis2);
const yAxis3 = new NumericAxis(wasmContext, {
    axisTitle: "Y Axis 3",
    axisAlignment: EAxisAlignment.Left
});
sciChartSurface.yAxes.add(yAxis3);
const yAxis4 = new NumericAxis(wasmContext, {
    axisTitle: "Y Axis 4",
    axisAlignment: EAxisAlignment.Left
});
sciChartSurface.yAxes.add(yAxis4);
const yAxis5 = new NumericAxis(wasmContext, {
    axisTitle: "Y Axis 5",
    axisAlignment: EAxisAlignment.Left
});
sciChartSurface.yAxes.add(yAxis5);
const yAxis6 = new NumericAxis(wasmContext, {
    axisTitle: "Y Axis 6",
    axisAlignment: EAxisAlignment.Left
});
sciChartSurface.yAxes.add(yAxis6);
const yAxis7 = new NumericAxis(wasmContext, {
    axisTitle: "Y Axis 7",
    axisAlignment: EAxisAlignment.Left
});
sciChartSurface.yAxes.add(yAxis7);

// To make it clearer what's happening, colour the axis backgrounds & borders
const axisColors = ["#50C7E0", "#EC0F6C", "#30BC9A", "#F48420", "#364BA0", "#882B91", "#67BDAF", "#C52E60"];
sciChartSurface.yAxes.asArray().forEach((yAxis, index) => {
    yAxis.backgroundColor = axisColors[index] + "22";
    yAxis.axisBorder = { color: axisColors[index], borderRight: 1 };
    yAxis.axisTitleStyle.fontSize = 13;
});

// Let's add some series to the chart to show how they also behave with axis
const getOptions = index => {
    const xValues = Array.from(Array(50).keys());
    const yValues = xValues.map(x => Math.sin(x * 0.4 + index));

    return {
        yAxisId: sciChartSurface.yAxes.asArray()[index].id,
        stroke: axisColors[index] + "77",
        strokeThickness: 2,
        dataSeries: new XyDataSeries(wasmContext, { xValues, yValues })
    };
};

sciChartSurface.renderableSeries.add(new FastLineRenderableSeries(wasmContext, { ...getOptions(0) }));
sciChartSurface.renderableSeries.add(new FastLineRenderableSeries(wasmContext, { ...getOptions(1) }));
sciChartSurface.renderableSeries.add(new FastLineRenderableSeries(wasmContext, { ...getOptions(2) }));
sciChartSurface.renderableSeries.add(new FastLineRenderableSeries(wasmContext, { ...getOptions(3) }));
sciChartSurface.renderableSeries.add(new FastLineRenderableSeries(wasmContext, { ...getOptions(4) }));
sciChartSurface.renderableSeries.add(new FastLineRenderableSeries(wasmContext, { ...getOptions(5) }));
sciChartSurface.renderableSeries.add(new FastLineRenderableSeries(wasmContext, { ...getOptions(6) }));
sciChartSurface.renderableSeries.add(new FastLineRenderableSeries(wasmContext, { ...getOptions(7) }));
```

``` prism-code
const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    surface: { theme: { type: EThemeProviderType.Dark } },
    xAxes: {
        type: EAxisType.NumericAxis,
        options: {
            axisTitle: "X Axis",
            axisTitleStyle: { fontSize: 13 },
            backgroundColor: "#50C7E022",
            axisBorder: { color: "#50C7E0", borderTop: 1 }
        }
    },
    yAxes: [
        {
            type: EAxisType.NumericAxis,
            options: { axisTitle: "Y Axis 0", axisAlignment: EAxisAlignment.Left }
        },
        {
            type: EAxisType.NumericAxis,
            options: { axisTitle: "Y Axis 1", axisAlignment: EAxisAlignment.Left }
        },
        {
            type: EAxisType.NumericAxis,
            options: { axisTitle: "Y Axis 2", axisAlignment: EAxisAlignment.Left }
        },
        {
            type: EAxisType.NumericAxis,
            options: { axisTitle: "Y Axis 3", axisAlignment: EAxisAlignment.Left }
        },
        {
            type: EAxisType.NumericAxis,
            options: { axisTitle: "Y Axis 4", axisAlignment: EAxisAlignment.Left }
        },
        {
            type: EAxisType.NumericAxis,
            options: { axisTitle: "Y Axis 5", axisAlignment: EAxisAlignment.Left }
        },
        {
            type: EAxisType.NumericAxis,
            options: { axisTitle: "Y Axis 6", axisAlignment: EAxisAlignment.Left }
        },
        {
            type: EAxisType.NumericAxis,
            options: { axisTitle: "Y Axis 7", axisAlignment: EAxisAlignment.Left }
        }
    ]
});

const axisColors = ["#50C7E0", "#EC0F6C", "#30BC9A", "#F48420", "#364BA0", "#882B91", "#67BDAF", "#C52E60"];
sciChartSurface.yAxes.asArray().forEach((yAxis, index) => {
    yAxis.backgroundColor = axisColors[index] + "22";
    yAxis.axisBorder = { color: axisColors[index], borderRight: 1 };
    yAxis.axisTitleStyle.fontSize = 13;
});
```

### Step 2: Apply the Layout Strategy<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/multi-axis-and-layout/vertically-stacked-axis-layout/#step-2-apply-the-layout-strategy" class="hash-link" aria-label="Direct link to Step 2: Apply the Layout Strategy" translate="no" title="Direct link to Step 2: Apply the Layout Strategy">â€‹</a>

To change the behaviour of axis stacking you need to set the appropriate layoutStrategy property on theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#layoutmanager" rel="noopener noreferrer" target="_blank">SciChartSurface.LayoutManagerðŸ“˜</a>Â with the stacked version.Â 

SciChart provides the following Outer Axes Layout Strategies:

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/leftalignedouterverticallystackedaxislayoutstrategy.html" rel="noopener noreferrer" target="_blank">LeftAlignedOuterVerticallyStackedAxisLayoutStrategyðŸ“˜</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rightalignedouterverticallystackedaxislayoutstrategy.html" rel="noopener noreferrer" target="_blank">RightAlignedOuterVerticallyStackedAxisLayoutStrategyðŸ“˜</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/topalignedouterhorizontallystackedaxislayoutstrategy.html" rel="noopener noreferrer" target="_blank">TopAlignedOuterHorizontallyStackedAxisLayoutStrategyðŸ“˜</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/bottomalignedouterhorizontallystackedaxislayoutstrategy.html" rel="noopener noreferrer" target="_blank">BottomAlignedOuterHorizontallyStackedAxisLayoutStrategyðŸ“˜</a>

Modify the code above to set this property on theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#layoutmanager" rel="noopener noreferrer" target="_blank">SciChartSurface.LayoutManagerðŸ“˜</a>:

- TS

``` prism-code
sciChartSurface.layoutManager.leftOuterAxesLayoutStrategy =
    new LeftAlignedOuterVerticallyStackedAxisLayoutStrategy();
```

Now the layout isÂ completely changed.

Make sure to assign Layout Strategy to an appropriate property on the Layout Manager accordingly to Axis Alignment.

## Experimenting with different Layout Strategies<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/multi-axis-and-layout/vertically-stacked-axis-layout/#experimenting-with-different-layout-strategies" class="hash-link" aria-label="Direct link to Experimenting with different Layout Strategies" translate="no" title="Direct link to Experimenting with different Layout Strategies">â€‹</a>

The followingÂ vertically stackedÂ layout strategies are available and may be applied to the following properties onÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#layoutmanager" rel="noopener noreferrer" target="_blank">SciChartSurface.LayoutManagerðŸ“˜</a>:

| Layout Strategy | Use With | Apply to LayoutManager Prop | Behavior |
|----|----|----|----|
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/leftalignedouteraxislayoutstrategy.html" rel="noopener noreferrer" target="_blank">LeftAlignedOuterAxisLayoutStrategyðŸ“˜</a> | Y Axis | <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/layoutmanager.html#leftinneraxeslayoutstrategy" rel="noopener noreferrer" target="_blank">leftInnerAxisLayoutStrategyðŸ“˜</a>, <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/layoutmanager.html#leftouteraxeslayoutstrategy" rel="noopener noreferrer" target="_blank">leftOuterAxisLayoutStrategyðŸ“˜</a> | Default behavior |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rightalignedouteraxislayoutstrategy.html" rel="noopener noreferrer" target="_blank">RightAlignedOuterAxisLayoutStrategyðŸ“˜</a> | Y Axis | <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/layoutmanager.html#rightinneraxeslayoutstrategy" rel="noopener noreferrer" target="_blank">rightInnerAxisLayoutStrategyðŸ“˜</a>, <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/layoutmanager.html#rightouteraxeslayoutstrategy" rel="noopener noreferrer" target="_blank">rightOuterAxisLayoutStrategyðŸ“˜</a> | Default behavior |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/leftalignedouterverticallystackedaxislayoutstrategy.html" rel="noopener noreferrer" target="_blank">LeftAlignedOuterVerticallyStackedAxisLayoutStrategyðŸ“˜</a> | Y Axis | <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/layoutmanager.html#rightouteraxeslayoutstrategy" rel="noopener noreferrer" target="_blank">rightOuterAxisLayoutStrategyðŸ“˜</a> | Vertical stacking behavior |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rightalignedouterverticallystackedaxislayoutstrategy.html" rel="noopener noreferrer" target="_blank">RightAlignedOuterVerticallyStackedAxisLayoutStrategyðŸ“˜</a> | Y Axis | <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/layoutmanager.html#leftouteraxeslayoutstrategy" rel="noopener noreferrer" target="_blank">leftOuterAxisLayoutStrategyðŸ“˜</a> | Vertical stacking behavior |

![](out_scichartv4/2d-charts/axis-api/multi-axis-and-layout/vertically-stacked-axis-layout/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

Try experimenting with the Codepen above to see how each of the strategies behave. Note that a **RightLayoutStrategy**Â will requireÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numericaxis.html#axisalignment" rel="noopener noreferrer" target="_blank">Axis.axisAlignmentðŸ“˜</a> =Â <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eaxisalignment.html" rel="noopener noreferrer" target="_blank">EAxisAlignment.RightðŸ“˜</a> and vice versa.

## CustomisingÂ Axis Size when Vertically Stacked<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/multi-axis-and-layout/vertically-stacked-axis-layout/#customisingaxis-size-when-vertically-stacked" class="hash-link" aria-label="Direct link to CustomisingÂ Axis Size when Vertically Stacked" translate="no" title="Direct link to CustomisingÂ Axis Size when Vertically Stacked">â€‹</a>

TheÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase2d.html#stackedaxislength" rel="noopener noreferrer" target="_blank">Axis.stackedAxisLengthðŸ“˜</a>Â property allows you to customize the size of a Vertically Stacked Axis in SciChart.js. This property may be an absolute number, e.g. 50 pixels, or a percentage e.g. "30%". When left undefined, default equal spacing will be used.

Find an updated example below:

- TS

``` prism-code
sciChartSurface.layoutManager.leftOuterAxesLayoutStrategy =
    new LeftAlignedOuterVerticallyStackedAxisLayoutStrategy();

// ...

// Create several YAxis on the left using stackedAxisLength
const yAxis0 = new NumericAxis(wasmContext, {
    axisAlignment: EAxisAlignment.Left,
    axisTitle: "50% Size",
    stackedAxisLength: "50%" // Occupy 50% of available viewport size
});
sciChartSurface.yAxes.add(yAxis0);
const yAxis1 = new NumericAxis(wasmContext, {
    axisAlignment: EAxisAlignment.Left,
    axisTitle: "Default"
});
sciChartSurface.yAxes.add(yAxis1);
const yAxis2 = new NumericAxis(wasmContext, {
    axisAlignment: EAxisAlignment.Left,
    axisTitle: "Default"
});
sciChartSurface.yAxes.add(yAxis2);
const yAxis3 = new NumericAxis(wasmContext, {
    axisAlignment: EAxisAlignment.Left,
    axisTitle: "200 pixels",
    stackedAxisLength: 200 // Occupy exactly 200 pixels
});
sciChartSurface.yAxes.add(yAxis3);
```

The layout and sizes of the Vertically Stacked Axis now updates as follows:

See Also

- [Inner Axis Layout](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/multi-axis-and-layout/inner-axis-layout)
- [Secondary and Multiple Axis](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/multi-axis-and-layout/secondary-and-multiple-axis-overview)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/axis-api/multi-axis-and-layout/vertically-stacked-axis-layout/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/axis-api/multi-axis-and-layout/vertically-stacked-axis-layout/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
