On this page

# Axis Borders and Background

In SciChart.js we have properties to let you style the axis border and background. This is the line between the axis and the main SciChartSurface and the background area of the axis itself.

To style the axis border and background, use this code.

- TS
- Builder API (JSON Config)

``` prism-code
// Demonstrates how to style axis borders and background in SciChart.js
const { SciChartSurface, NumericAxis, SciChartJsNavyTheme, EAxisAlignment } = SciChart;

// or, for npm, import { SciChartSurface, ... } from "scichart"

const { wasmContext, sciChartSurface } = await SciChartSurface.create(divElementId, {
    theme: new SciChartJsNavyTheme()
});

const yAxis = new NumericAxis(wasmContext, {
    axisTitleStyle: { color: "#368BC1" },
    id: "RightAxis",
    axisTitle: "Right Axis",
    axisBorder: {
        borderLeft: 1,
        color: "#368BC1" // Blue color
    },
    backgroundColor: "#368BC111"
});

const leftYAxis = new NumericAxis(wasmContext, {
    axisAlignment: EAxisAlignment.Left,
    axisTitleStyle: { color: "#228B22" },
    axisTitle: "Left Axis",
    axisBorder: {
        borderRight: 1,
        color: "#228B22" // Green color
    },
    backgroundColor: "#228B2222"
});

const xAxis = new NumericAxis(wasmContext, {
    axisTitleStyle: { color: "#EEEEEE" },
    axisTitle: "X Axis",
    axisBorder: {
        borderTop: 1,
        color: "#EEEEEE" // Green color
    },
    backgroundColor: "#EEEEEE11"
});

sciChartSurface.yAxes.add(yAxis, leftYAxis);
sciChartSurface.xAxes.add(xAxis);
```

``` prism-code
// Demonstrates how to style a numeric axis in SciChart.js using the Builder API
const { chartBuilder, EThemeProviderType, EAxisAlignment, EAxisType } = SciChart;

// or, for npm, import { chartBuilder, ... } from "scichart"

const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    surface: { theme: { type: EThemeProviderType.Dark } },
    xAxes: {
        type: EAxisType.NumericAxis,
        options: {
            axisTitleStyle: { color: "#EEEEEE" },
            axisTitle: "X Axis",
            axisBorder: {
                borderTop: 1,
                color: "#EEEEEE" // Green color
            },
            backgroundColor: "#EEEEEE11"
        }
    },
    yAxes: [
        {
            type: EAxisType.NumericAxis,
            options: {
                axisTitleStyle: { color: "#368BC1" },
                id: "RightAxis",
                axisTitle: "Right Axis",
                axisBorder: {
                    borderLeft: 1,
                    color: "#368BC1" // Blue color
                },
                backgroundColor: "#368BC111"
            }
        },
        {
            type: EAxisType.NumericAxis,
            options: {
                axisAlignment: EAxisAlignment.Left,
                axisTitleStyle: { color: "#228B22" },
                axisTitle: "Left Axis",
                axisBorder: {
                    borderRight: 1,
                    color: "#228B22" // Green color
                },
                backgroundColor: "#228B2222"
            }
        }
    ]
});
```

This results in the following output:

***Note:** All colors in SciChart.js are strings, which are HTML color codes. Supported values are 6-digit hex codes e.g. "#ADFF2F", 8-digit hex codes in RGBA format where the last two digits are opacity e.g. "#AAFF2F33" and rgba CSS color codes e.g. "rgba(173, 255, 47, 0.3)" as well as "Red" or "White"*

## The Axis Background<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-styling/axis-borders-and-background/#the-axis-background" class="hash-link" aria-label="Direct link to The Axis Background" translate="no" title="Direct link to The Axis Background">â€‹</a>

The background of the axis can also be set to a solid color using theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase2d.html#backgroundcolor" rel="noopener noreferrer" target="_blank">AxisBase2D.backgroundColorðŸ“˜</a> property. This supports an HTML color code as above.

## The TBorderType<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-styling/axis-borders-and-background/#the-tbordertype" class="hash-link" aria-label="Direct link to The TBorderType" translate="no" title="Direct link to The TBorderType">â€‹</a>

Axis Borders can be set on the Left, Right, Bottom or Top of the Axis. The thickness of the border can be set individually on each side of the axis. For more information, see theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase2d.html#axisborder" rel="noopener noreferrer" target="_blank">AxisBase2D.axisBorderðŸ“˜</a> property, which is typeÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tborder" rel="noopener noreferrer" target="_blank">TBorderðŸ“˜</a>.

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/axis-api/axis-styling/axis-borders-and-background/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/axis-api/axis-styling/axis-borders-and-background/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
