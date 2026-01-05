On this page

# Axis Ranging - AutoRange

At the outset,Â theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#visiblerange" rel="noopener noreferrer" target="_blank">Axis.visibleRangeðŸ“˜</a> is adjusted to be equal to theÂ data rangeÂ of an axis. However, an axis won't adjust its VisibleRangeÂ automatically when data changes, unless itÂ is configured to do this.Â The defaultÂ behaviorÂ can be changedÂ using differentÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#autorange" rel="noopener noreferrer" target="_blank">AutoRangeðŸ“˜</a> modes.

## AutoRange Once<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/ranging-scaling/auto-range/#autorange-once" class="hash-link" aria-label="Direct link to AutoRange Once" translate="no" title="Direct link to AutoRange Once">â€‹</a>

This is theÂ **default setting**. The axis will attempt to autorange once to fit the data as you start the chart. This is an one-time action - the VisibleRangeÂ won't adjust to any data changes in future.

Note: SpecifyingÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#visiblerange" rel="noopener noreferrer" target="_blank">axis.visibleRangeðŸ“˜</a> at startup will set that as the first default range. AutoRange.Once is ignored when a visibleRange is set

- TS
- Builder API (JSON Config)

``` prism-code
const { SciChartSurface, NumericAxis, SciChartJsNavyTheme, EAutoRange } = SciChart;

// or, for npm, import { SciChartSurface, ... } from "scichart"

// Create a chart with X,Y axis
const { wasmContext, sciChartSurface } = await SciChartSurface.create(divElementId, {
    theme: new SciChartJsNavyTheme()
});

// Set AutoRange on the yAxis
sciChartSurface.xAxes.add(new NumericAxis(wasmContext));
sciChartSurface.yAxes.add(
    new NumericAxis(wasmContext, {
        autoRange: EAutoRange.Once
    })
);
```

``` prism-code
const { chartBuilder, ESeriesType, EThemeProviderType, EAutoRange, EAxisType } = SciChart;

// or, for npm, import { chartBuilder, ... } from "scichart"

const xValues = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19];
const yValues = xValues.map(x => Math.sin(x * 0.2));

const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    surface: { theme: { type: EThemeProviderType.Dark } },
    xAxes: {
        type: EAxisType.NumericAxis,
        options: { axisTitle: "X Axis" }
    },
    yAxes: {
        type: EAxisType.NumericAxis,
        options: { axisTitle: "Y Axis", autoRange: EAutoRange.Once }
    },
    series: [
        {
            type: ESeriesType.LineSeries,
            xyData: {
                xValues,
                yValues
            },
            options: {
                stroke: "#50C7E0",
                strokeThickness: 3
            }
        }
    ]
});
```

## AutoRange Always<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/ranging-scaling/auto-range/#autorange-always" class="hash-link" aria-label="Direct link to AutoRange Always" translate="no" title="Direct link to AutoRange Always">â€‹</a>

In this mode, the axis will attempt to autorange always to fit the data every time the chart is drawn. The **VisibleRange** **will adjust** to data changes correspondingly.

Please be aware that this **setting will override any other ranging**, including zooming and panning by modifiers, but is useful in situations where you always want to view the extents of the data.

To combine AutoRanging and user-zooming you need to use **ZoomState** - a special technique we will talk about later.

- TS
- Builder API (JSON Config)

``` prism-code
const { SciChartSurface, NumericAxis, SciChartJsNavyTheme, EAutoRange } = SciChart;

// or, for npm, import { SciChartSurface, ... } from "scichart"

// Create a chart with X,Y axis
const { wasmContext, sciChartSurface } = await SciChartSurface.create(divElementId, {
    theme: new SciChartJsNavyTheme()
});

// Set AutoRange on the yAxis
sciChartSurface.xAxes.add(new NumericAxis(wasmContext));
sciChartSurface.yAxes.add(
    new NumericAxis(wasmContext, {
        autoRange: EAutoRange.Always
    })
);
```

``` prism-code
const { chartBuilder, ESeriesType, EThemeProviderType, EAutoRange, EAxisType } = SciChart;

// or, for npm, import { chartBuilder, ... } from "scichart"

const xValues = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19];
const yValues = xValues.map(x => Math.sin(x * 0.2));

const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    surface: { theme: { type: EThemeProviderType.Dark } },
    xAxes: {
        type: EAxisType.NumericAxis,
        options: { axisTitle: "X Axis" }
    },
    yAxes: {
        type: EAxisType.NumericAxis,
        options: {
            axisTitle: "Y Axis",
            autoRange: EAutoRange.Always
        }
    },
    series: [
        {
            type: ESeriesType.LineSeries,
            xyData: {
                xValues,
                yValues
            },
            options: {
                stroke: "#50C7E0",
                strokeThickness: 3
            }
        }
    ]
});
```

## AutoRange Never<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/ranging-scaling/auto-range/#autorange-never" class="hash-link" aria-label="Direct link to AutoRange Never" translate="no" title="Direct link to AutoRange Never">â€‹</a>

The **axis will never autorange**. With this option, you would need to set the VisibleRange manually. The **VisibleRange**Â **won't adjust** to any data changes.

- TS
- Builder API (JSON Config)

``` prism-code
const { SciChartSurface, NumericAxis, SciChartJsNavyTheme, EAutoRange } = SciChart;

// or, for npm, import { SciChartSurface, ... } from "scichart"

// Create a chart with X,Y axis
const { wasmContext, sciChartSurface } = await SciChartSurface.create(divElementId, {
    theme: new SciChartJsNavyTheme()
});

// Set AutoRange on the yAxis
sciChartSurface.xAxes.add(new NumericAxis(wasmContext));
sciChartSurface.yAxes.add(
    new NumericAxis(wasmContext, {
        autoRange: EAutoRange.Never
    })
);
```

``` prism-code
const { chartBuilder, ESeriesType, EThemeProviderType, EAutoRange, EAxisType } = SciChart;

// or, for npm, import { chartBuilder, ... } from "scichart"

const xValues = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19];
const yValues = xValues.map(x => Math.sin(x * 0.2));

const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    surface: { theme: { type: EThemeProviderType.Dark } },
    xAxes: {
        type: EAxisType.NumericAxis,
        options: { axisTitle: "X Axis" }
    },
    yAxes: {
        type: EAxisType.NumericAxis,
        options: { axisTitle: "Y Axis", autoRange: EAutoRange.Never }
    },
    series: [
        {
            type: ESeriesType.LineSeries,
            xyData: {
                xValues,
                yValues
            },
            options: {
                stroke: "#50C7E0",
                strokeThickness: 3
            }
        }
    ]
});
```

## Adding Padding or Spacing with GrowBy<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/ranging-scaling/auto-range/#adding-padding-or-spacing-with-growby" class="hash-link" aria-label="Direct link to Adding Padding or Spacing with GrowBy" translate="no" title="Direct link to Adding Padding or Spacing with GrowBy">â€‹</a>

Also, it is possible toÂ **add spacing** or padding to the visibleRange when the chart autoranges via theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#growby" rel="noopener noreferrer" target="_blank">GrowByðŸ“˜</a> property. It allows to specify two fractions which will be always applied to theÂ Min,Â Max values of visibleRange :

- TS
- Builder API (JSON Config)

``` prism-code
const { SciChartSurface, NumericAxis, SciChartJsNavyTheme, EAutoRange, NumberRange } = SciChart;

// or, for npm, import { SciChartSurface, ... } from "scichart"

// Create a chart with X,Y axis
const { wasmContext, sciChartSurface } = await SciChartSurface.create(divElementId, {
    theme: new SciChartJsNavyTheme()
});

// Set GrowBy on the yAxis to add 20% padding above/below
sciChartSurface.xAxes.add(new NumericAxis(wasmContext));
sciChartSurface.yAxes.add(
    new NumericAxis(wasmContext, {
        autoRange: EAutoRange.Always,
        growBy: new NumberRange(0.2, 0.2)
    })
);
```

``` prism-code
const { chartBuilder, ESeriesType, EThemeProviderType, EAutoRange, EAxisType, NumberRange } = SciChart;

// or, for npm, import { chartBuilder, ... } from "scichart"

const xValues = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19];
const yValues = xValues.map(x => Math.sin(x * 0.2));

const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    surface: { theme: { type: EThemeProviderType.Dark } },
    xAxes: {
        type: EAxisType.NumericAxis,
        options: { axisTitle: "X Axis" }
    },
    yAxes: {
        type: EAxisType.NumericAxis,
        options: {
            axisTitle: "Y Axis",
            autoRange: EAutoRange.Always,
            growBy: new NumberRange(0.2, 0.2)
        }
    },
    series: [
        {
            type: ESeriesType.LineSeries,
            xyData: {
                xValues,
                yValues
            },
            options: {
                stroke: "#50C7E0",
                strokeThickness: 3
            }
        }
    ]
});
```

## Programmatically Ranging an Axis<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/ranging-scaling/auto-range/#programmatically-ranging-an-axis" class="hash-link" aria-label="Direct link to Programmatically Ranging an Axis" translate="no" title="Direct link to Programmatically Ranging an Axis">â€‹</a>

See the section onÂ [Setting and Getting VisibleRange](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/ranging-scaling/set-range-zoom-to-fit) for more details.

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/axis-api/ranging-scaling/auto-range/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/axis-api/ranging-scaling/auto-range/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
