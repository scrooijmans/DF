On this page

# Secondary and Multiple Axes

SciChart.js supports **unlimited, multiple X or YÂ axis** which can be aligned to theÂ Right, Left, Top, Bottom sides of a chart.

Above: The JavaScript <a href="https://www.scichart.com/demo/iframe/javascript-chart-with-multiple-x-axis" target="_blank">Multiple X Axes</a> example from the <a href="https://www.scichart.com/demo/react" target="_blank">SciChart.js Demo</a>

  

## How to Setup a Chart with Multiple Axes<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/multi-axis-and-layout/secondary-and-multiple-axis-overview/#how-to-setup-a-chart-with-multiple-axes" class="hash-link" aria-label="Direct link to How to Setup a Chart with Multiple Axes" translate="no" title="Direct link to How to Setup a Chart with Multiple Axes">â€‹</a>

- Axis may be placed by setting theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase2d.html#axisalignment" rel="noopener noreferrer" target="_blank">AxisBase2D.axisAlignmentðŸ“˜</a> property.
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#id" rel="noopener noreferrer" target="_blank">Axis.IdðŸ“˜</a> identifies an axis in multi-axis scenarios
- Series, Annotations and some Modifiers have **yAxisId**, **yAxisId** properties. These are used to assign chart items to an axis in multi-axis scenarios.

When you create an axis it automatically gets a unique id, which you can use to assign to Series, Annotations and some Modifiers.

- TS

``` prism-code
renderableSeries.xAxisId = xAxis.id;
renderableSeries.yAxisId = yAxis.id;
...
annotation.xAxisId = xAxis.id;
annotation.yAxisId = yAxis.id;
```

There's little more to it than that. However, there are many configurations in SciChart.js which we will get into later.

Here's a worked example:

- TS
- Builder API (JSON Config)

``` prism-code
const { wasmContext, sciChartSurface } = await SciChartSurface.create(divElementId, {
    theme: new SciChartJsNavyTheme()
});

// Add a primary X,Y Axis pair
sciChartSurface.xAxes.add(
    new NumericAxis(wasmContext, {
        axisAlignment: EAxisAlignment.Bottom,
        axisTitle: "X Axis Bottom",
        axisTitleStyle: titleStyle1,
        labelStyle: labelStyle1,
        backgroundColor: "#50C7E022",
        axisBorder: {
            borderTop: 1,
            color: "#50C7E0"
        }
    })
);

sciChartSurface.yAxes.add(
    new NumericAxis(wasmContext, {
        axisAlignment: EAxisAlignment.Left,
        axisTitle: "Y Axis Left",
        axisTitleStyle: titleStyle1,
        labelStyle: labelStyle1,
        growBy: new NumberRange(0.1, 0.1),
        backgroundColor: "#50C7E022",
        axisBorder: {
            borderRight: 1,
            color: "#50C7E0"
        }
    })
);

// Add a secondary X,Y Axis pair
// Capture the axis instance as you may need it's Id which is auto generated
const xAxis2 = new NumericAxis(wasmContext, {
    axisTitleStyle: titleStyle2,
    labelStyle: labelStyle2,
    axisAlignment: EAxisAlignment.Top,
    axisTitle: "X Axis Top",
    backgroundColor: "#F4842022",
    axisBorder: {
        borderBottom: 1,
        color: "#F48420"
    }
});
sciChartSurface.xAxes.add(xAxis2);

const yAxis2 = new NumericAxis(wasmContext, {
    axisTitleStyle: titleStyle2,
    labelStyle: labelStyle2,
    axisAlignment: EAxisAlignment.Right,
    axisTitle: "Y Axis Right",
    labelFormat: ENumericFormat.Decimal,
    labelPrecision: 2,
    growBy: new NumberRange(0.1, 0.1),
    backgroundColor: "#F4842022",
    axisBorder: {
        borderLeft: 1,
        color: "#F48420"
    }
});
sciChartSurface.yAxes.add(yAxis2);
```

``` prism-code
const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    surface: { theme: { type: EThemeProviderType.Dark } },
    xAxes: [
        {
            type: EAxisType.NumericAxis,
            options: {
                axisAlignment: EAxisAlignment.Bottom,
                axisTitle: "X Axis Bottom",
                axisTitleStyle: titleStyle1,
                labelStyle: labelStyle1,
                backgroundColor: "#50C7E022",
                axisBorder: {
                    borderTop: 1,
                    color: "#50C7E0"
                }
            }
        },
        {
            type: EAxisType.NumericAxis,
            options: {
                axisTitleStyle: titleStyle2,
                labelStyle: labelStyle2,
                axisAlignment: EAxisAlignment.Top,
                axisTitle: "X Axis Top",
                backgroundColor: "#F4842022",
                axisBorder: {
                    borderBottom: 1,
                    color: "#F48420"
                }
            }
        }
    ],
    yAxes: [
        {
            type: EAxisType.NumericAxis,
            options: {
                axisAlignment: EAxisAlignment.Left,
                axisTitle: "Y Axis Left",
                axisTitleStyle: titleStyle1,
                labelStyle: labelStyle1,
                growBy: new NumberRange(0.1, 0.1),
                backgroundColor: "#50C7E022",
                axisBorder: {
                    borderRight: 1,
                    color: "#50C7E0"
                }
            }
        },
        {
            type: EAxisType.NumericAxis,
            options: {
                axisTitleStyle: titleStyle2,
                labelStyle: labelStyle2,
                axisAlignment: EAxisAlignment.Right,
                axisTitle: "Y Axis Right",
                labelFormat: ENumericFormat.Decimal,
                labelPrecision: 2,
                growBy: new NumberRange(0.1, 0.1),
                backgroundColor: "#F4842022",
                axisBorder: {
                    borderLeft: 1,
                    color: "#F48420"
                }
            }
        }
    ]
});
```

This code results in the following configuration of axis. Also seen in ourÂ <a href="https://www.scichart.com/demo/javascript-chart-with-multiple-x-axis" rel="noopener noreferrer" target="_blank">Multiple Axis Demo</a>.

## Attaching Chart SeriesÂ to anÂ AxisÂ <a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/multi-axis-and-layout/secondary-and-multiple-axis-overview/#attaching-chart-seriesto-anaxis" class="hash-link" aria-label="Direct link to Attaching Chart SeriesÂ to anÂ AxisÂ " translate="no" title="Direct link to Attaching Chart SeriesÂ to anÂ AxisÂ ">â€‹</a>

EveryÂ [RenderableSeries](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/renderable-series-api-overview) (the chart types in SciChart.js e.g.Â Line,Â Candlestick,Â Column) and everyÂ [Annotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/annotations-api-overview) (Trendlines, text or markers laid over the chart) and someÂ [ChartModifiers](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/chart-modifier-api-overview) (zoom,Â pan behaviours) need to be attached to a particular axis.

The link between series and axis is done viaÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#id" rel="noopener noreferrer" target="_blank">AxisCore.idðŸ“˜</a>, andÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries.html#xaxisid" rel="noopener noreferrer" target="_blank">BaseRenderableSeries.xAxisIdðŸ“˜</a> andÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries.html#yaxisid" rel="noopener noreferrer" target="_blank">BaseRenderableSeries.yAxisIdðŸ“˜</a> properties.

With a single X,Y Axis you never have to set these properties as they gets set automatically. When a series, annotation orÂ modifier gets attached to SciChartSurface, xAxis.id and yAxis.id get values from the first X and Y axes.

However, in a multiple axis scenario, series must be attached to an axis. To do this, ensure that you set theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries.html#xaxisid" rel="noopener noreferrer" target="_blank">BaseRenderableSeries.xAxisIdðŸ“˜</a> andÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries.html#yaxisid" rel="noopener noreferrer" target="_blank">BaseRenderableSeries.yAxisIdðŸ“˜</a> equal to theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#id" rel="noopener noreferrer" target="_blank">YAxis.idðŸ“˜</a> orÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#id" rel="noopener noreferrer" target="_blank">XAxis.idðŸ“˜</a> you wish to attach to.

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/axis-api/multi-axis-and-layout/secondary-and-multiple-axis-overview/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/axis-api/multi-axis-and-layout/secondary-and-multiple-axis-overview/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
