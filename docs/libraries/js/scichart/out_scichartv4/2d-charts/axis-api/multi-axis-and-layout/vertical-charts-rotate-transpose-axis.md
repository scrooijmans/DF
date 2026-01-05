On this page

# Vertical Charts (Rotate, Transpose Axis)

It is possible to createÂ Vertical (Rotated) ChartsÂ with SciChart. This transposes the entire chart, swapping X-Axis for Y and renders series top to bottom intead of left to right. Tooltips and markers also are transposed to the final effect is like a vertical chart.

Above: The JavaScript <a href="https://www.scichart.com/demo/iframe/javascript-oil-and-gas-dashboard-showcase" target="_blank">JavaScript Oil and Gas Dashboard</a> example from the <a href="https://www.scichart.com/demo/react" target="_blank">SciChart.js Demo</a> showing a use-case of transposing the X,Y axis to achieveÂ a vertical chart, visualising well drill depth.

  

ToÂ achieve this, simply setÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase2d.html#axisalignment" rel="noopener noreferrer" target="_blank">axis.axisAlignmentðŸ“˜</a> toÂ Left orÂ Right for X Axis andÂ Top orÂ Bottom for Y Axis. And that's it - SciChart takes care of the rest:

- TS
- Builder API (JSON Config)

``` prism-code
// Demonstrates how to configure a vertical chart in SciChart.js
const {
    SciChartSurface,
    NumericAxis,
    SciChartJsNavyTheme,
    EAxisAlignment,
    HorizontalLineAnnotation,
    ELabelPlacement,
    FastLineRenderableSeries,
    XyDataSeries
} = SciChart;

// or, for npm, import { SciChartSurface, ... } from "scichart"

const { wasmContext, sciChartSurface } = await SciChartSurface.create(divElementId, {
    theme: new SciChartJsNavyTheme()
});

// Add the xAxis to the chart
sciChartSurface.xAxes.add(
    new NumericAxis(wasmContext, {
        axisTitle: "X Axis",
        axisAlignment: EAxisAlignment.Left
    })
);

// Creating a NumericAxis as a YAxis on the left
sciChartSurface.yAxes.add(
    new NumericAxis(wasmContext, {
        axisTitle: "Y Axis",
        axisAlignment: EAxisAlignment.Top
    })
);

// Show how a line series responds to vertical chart
const xValues = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19];
const yValues = xValues.map(x => Math.sin(x * 0.4));
sciChartSurface.renderableSeries.add(
    new FastLineRenderableSeries(wasmContext, {
        dataSeries: new XyDataSeries(wasmContext, {
            xValues,
            yValues
        }),
        stroke: "#0066FF",
        strokeThickness: 3
    })
);

// Show how a HorizontalLineAnnotation responds to vertical chart
sciChartSurface.annotations.add(
    new HorizontalLineAnnotation({
        // normally we set y1 but with vertical charts, we set annotation.x1
        x1: 10,
        labelValue: "HorizontalLineAnnotation with x1 = 10",
        showLabel: true,
        stroke: "#F48420",
        strokeThickness: 2,
        labelPlacement: ELabelPlacement.TopLeft
    })
);
```

``` prism-code
// Demonstrates how to configure a vertical chart in SciChart.js using the Builder API
const { chartBuilder, EThemeProviderType, EAxisType, EAxisAlignment, ESeriesType } = SciChart;

// or, for npm, import { chartBuilder, ... } from "scichart"

const xValues = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19];
const yValues = xValues.map(x => Math.sin(x * 0.4));

const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    surface: { theme: { type: EThemeProviderType.Dark } },
    xAxes: {
        type: EAxisType.NumericAxis,
        options: {
            axisTitle: "X Axis",
            axisAlignment: EAxisAlignment.Left
        }
    },
    yAxes: {
        type: EAxisType.NumericAxis,
        options: {
            axisTitle: "Y Axis",
            axisAlignment: EAxisAlignment.Top
        }
    },
    series: [
        {
            type: ESeriesType.LineSeries,
            options: {
                stroke: "#0066FF",
                strokeThickness: 3
            },
            xyData: {
                xValues,
                yValues
            }
        }
    ]
});
```

This results in the following output, where the XAxis is on the left, the YAxis is on the top. The chart series is rotated automatically and now draws top to bottom, rather than left to right.

## Flipping the Axis when Horizontal or Vertical.<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/multi-axis-and-layout/vertical-charts-rotate-transpose-axis/#flipping-the-axis-when-horizontal-or-vertical" class="hash-link" aria-label="Direct link to Flipping the Axis when Horizontal or Vertical." translate="no" title="Direct link to Flipping the Axis when Horizontal or Vertical.">â€‹</a>

An Axis may be flipped when horizontal or vertical (coordinates drawn in opposite directions) by setting theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#flippedcoordinates" rel="noopener noreferrer" target="_blank">AxisCore.flippedCoordinatesðŸ“˜</a> property.

For example, taking the code sample above, and setting <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#flippedcoordinates" rel="noopener noreferrer" target="_blank">xAxis.flippedCoordinatesðŸ“˜</a> = true, we get the following result. Notice the XAxis is now drawn in reverse and the series is now drawn from bottom to top..

<img src="out_scichartv4/2d-charts/axis-api/multi-axis-and-layout/vertical-charts-rotate-transpose-axis/index_media/6badb707c51100873e0edc32f6b287fbf4a20524.png" class="img_ev3q" decoding="async" loading="lazy" width="1195" height="890" />

## Considerations when using Vertical Charts<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/multi-axis-and-layout/vertical-charts-rotate-transpose-axis/#considerations-when-using-vertical-charts" class="hash-link" aria-label="Direct link to Considerations when using Vertical Charts" translate="no" title="Direct link to Considerations when using Vertical Charts">â€‹</a>

This Flexibility of SciChart allows for some pretty interesting configurations of charts. However, here are some considerations when using a Vertical Chart.

- Tooltips,Â [Cursors](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/cursor-modifier/cursor-modifier-overview) and theÂ [RolloverModifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/rollover-modifier) will also be transposed (rotated 90 degrees). When applying a RolloverModifier the cursor line is usually vertical, but in a vertical chart the cursor line will be horizontal.
- Annotations will behave differently. For example aÂ [HorizontalLineAnnotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/horizontal-line-annotation) will still draw horizontally but instead of setting the y1 property to place on the YAxis, now you must set x1 property to place on the XAxis.

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/multi-axis-and-layout/vertical-charts-rotate-transpose-axis/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- <a href="https://www.scichart.com/demo/javascript-vertical-charts" rel="noopener noreferrer" target="_blank">Vertical (Rotated) Chart Example</a>

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/axis-api/multi-axis-and-layout/vertical-charts-rotate-transpose-axis/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/axis-api/multi-axis-and-layout/vertical-charts-rotate-transpose-axis/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
