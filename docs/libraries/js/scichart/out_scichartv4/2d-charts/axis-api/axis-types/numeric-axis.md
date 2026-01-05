On this page

# The Numeric Axis

TheÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numericaxis.html" rel="noopener noreferrer" target="_blank">NumericAxisðŸ“˜</a>Â is a Value axis and is suitable for X and Y Axis when the data on that axis is numeric (e.g. number in TypeScript). It is not suitable for non-numeric data types.

![](out_scichartv4/2d-charts/axis-api/axis-types/numeric-axis/index_media/af50301a53fa555616a9b2279e7e25a1d566cb1a.svg)info

Learn more about theÂ [commonalities between axis here](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-types/common-axis-base-type).

## Create and Configure a NumericAxis<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-types/numeric-axis/#create-and-configure-a-numericaxis" class="hash-link" aria-label="Direct link to Create and Configure a NumericAxis" translate="no" title="Direct link to Create and Configure a NumericAxis">â€‹</a>

There are lots of options that can be passed to the constructor of a NumericAxis to configure it. Some of these are in theÂ [common AxisBase2D type](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-types/common-axis-base-type).

To create and configureÂ aÂ NumericAxis, use the following code:Â 

- TS
- Builder API (JSON Config)

``` prism-code
// Demonstrates how to configure a numeric axis in SciChart.js
const { SciChartSurface, NumericAxis, SciChartJsNavyTheme, EAutoRange, EAxisAlignment, ENumericFormat } = SciChart;

// or, for npm, import { SciChartSurface, ... } from "scichart"

const { wasmContext, sciChartSurface } = await SciChartSurface.create(divElementId, {
    theme: new SciChartJsNavyTheme()
});

// Create an XAxis on the bottom
const xAxis = new NumericAxis(wasmContext, {
    // All these properties are optional
    // ...
    // Enable flags like drawing gridlines
    drawMajorGridLines: true,
    drawMinorGridLines: true,
    drawLabels: true,
    // Set multiline title
    axisTitle: ["X Axis, Bottom", "2 decimal places"],
    // Set the alignment and autoRange
    axisAlignment: EAxisAlignment.Bottom,
    autoRange: EAutoRange.Once,
    // Enable decision labels with 4 significant figures
    labelFormat: ENumericFormat.Decimal,
    cursorLabelFormat: ENumericFormat.Decimal,
    labelPrecision: 4
});

// Add the xAxis to the chart
sciChartSurface.xAxes.add(xAxis);

// Creating a NumericAxis as a YAxis on the left
sciChartSurface.yAxes.add(
    new NumericAxis(wasmContext, {
        axisTitle: "Y Axis, Left, 4 dp",
        axisAlignment: EAxisAlignment.Left,
        labelFormat: ENumericFormat.Decimal,
        cursorLabelFormat: ENumericFormat.Decimal,
        labelPrecision: 4,
        labelPrefix: "$",
        labelPostfix: " USD"
    })
);
```

``` prism-code
// Demonstrates how to configure a numeric axis in SciChart.js using the Builder API
const { chartBuilder, ESeriesType, EThemeProviderType, EAutoRange, EAxisAlignment, ENumericFormat, EAxisType } =
    SciChart;

// or, for npm, import { chartBuilder, ... } from "scichart"

const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    surface: { theme: { type: EThemeProviderType.Dark } },
    xAxes: {
        type: EAxisType.NumericAxis,
        options: {
            // All these properties are optional
            // ...
            // Enable flags like drawing gridlines
            drawMajorGridLines: true,
            drawMinorGridLines: true,
            drawLabels: true,
            // Set title, alignment and autorange
            axisTitle: "X Axis, Bottom, 2 decimal places",
            axisAlignment: EAxisAlignment.Bottom,
            autoRange: EAutoRange.Once,
            // Enable decision labels with 4 significant figures
            labelFormat: ENumericFormat.Decimal,
            cursorLabelFormat: ENumericFormat.Decimal,
            labelPrecision: 2
        }
    },
    yAxes: {
        type: EAxisType.NumericAxis,
        options: {
            // axisTitle: "Y Axis, Left, default formatting",
            axisAlignment: EAxisAlignment.Left,
            axisTitle: "Y Axis, Left, 4 dp",
            labelFormat: ENumericFormat.Decimal,
            cursorLabelFormat: ENumericFormat.Decimal,
            labelPrecision: 4,
            labelPrefix: "$",
            labelPostfix: " USD"
        }
    },
    series: [
        {
            type: ESeriesType.LineSeries,
            xyData: {
                xValues: [0, 1, 2, 3, 4, 5, 6, 7, 8],
                yValues: [2.5, 3.5, 3.7, 4.0, 5.0, 5.5, 5.0, 4.0, 3.0]
            },
            options: {
                stroke: "#0066FF",
                strokeThickness: 5
            }
        }
    ]
});
```

This results in the following output:

![](out_scichartv4/2d-charts/axis-api/axis-types/numeric-axis/index_media/af50301a53fa555616a9b2279e7e25a1d566cb1a.svg)info

Further enhancement of the NumericAxis labels including custom formatting, string formatting or dynamic formatting can be achieved with theÂ [LabelProvider API](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-labels/label-provider-api-overview).

Also see the documentation page on ENumericFormat

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/axis-api/axis-types/numeric-axis/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/axis-api/axis-types/numeric-axis/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
