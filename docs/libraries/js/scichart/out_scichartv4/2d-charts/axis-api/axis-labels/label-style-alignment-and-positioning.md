On this page

# Label Style, Alignment and Positioning

## The LabelStyle property<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-labels/label-style-alignment-and-positioning/#the-labelstyle-property" class="hash-link" aria-label="Direct link to The LabelStyle property" translate="no" title="Direct link to The LabelStyle property">â€‹</a>

The Axis includes aÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase2d.html#labelstyle" rel="noopener noreferrer" target="_blank">LabelStyleðŸ“˜</a> property. This may be set in theÂ constructor options or set on the axis itself. Apply a labelStyle as follows to an axis:

``` prism-code
// Label Style

import { NumericAxis, ELabelAlignment, Thickness } from "scichart";

const axis = new NumericAxis(wasmContext, {
    labelStyle: {
        alignment: ELabelAlignment.Auto,
        fontFamily: "Arial",
        fontSize: 16,
        color: "White",
    } // type TTextStyle
});
```

The type for LabelStyle isÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#ttextstyle" rel="noopener noreferrer" target="_blank">TTextStyleðŸ“˜</a>. The Definition for TTextStyle is found below:

``` prism-code
// TTextStyle definition

/**
 * A type class to contain information about Axis Label text styles
 * @remarks
 * - Set the fontFamily as a string to set the font
 * - Set the fontSize as you would in HTML/CSS
 * - Set the fontWeight and fontStyle as you would in HTML/CSS
 * - Set the color as an HTML Color code to define the color
 */
export type TTextStyle = {
    fontSize?: number;
    fontFamily?: string;
    fontWeight?: string;
    fontStyle?: string;
    color?: string;
    /** Padding is left 4, right 4, top 2, bottom 0 by default. This is because there is natural space below the text baseline.
     * If you are using text labels rather than just numbers, or when using native text,
     * you may want to increase the bottom padding.
     */
    padding?: Thickness;
    /** Horizontal label alignment for vertical axes.  Default Auto */
    alignment?: ELabelAlignment;
};
```

## Rotated and Multiline Native Text Labels<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-labels/label-style-alignment-and-positioning/#rotated-and-multiline-native-text-labels" class="hash-link" aria-label="Direct link to Rotated and Multiline Native Text Labels" translate="no" title="Direct link to Rotated and Multiline Native Text Labels">â€‹</a>

The standard axis labels supported rotation, but the positioning is poor for angles outside the 0 to 90 range.Â With native text labels, this is fixed.Â Note that rotation is a property on the labelProvider, not the axis itself.

![](out_scichartv4/2d-charts/axis-api/axis-labels/label-style-alignment-and-positioning/index_media/af50301a53fa555616a9b2279e7e25a1d566cb1a.svg)info

When using angles that are not a multiple of 90, you probably want to setÂ **hideOverlappingLabels: false** as the overlap is calculated using the bounding rectangle of the text.Â 

Multiline labels are supported simply by using newline characters (\n)Â in the label text.Â  lineSpacing is a property on the labelProvider.Â  The alignment property on labelStyle also affects the alignment for multiple lines.Â 

![](out_scichartv4/2d-charts/axis-api/axis-labels/label-style-alignment-and-positioning/index_media/5a72c3ee6a9cc14296f901057d6eabf2b9b712b4.svg)note

Note: for more info aboutÂ [Text and MultiLine labels see this article](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-labels/text-and-multi-line-labels). For rotation of labelsÂ [see this article](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-labels/rotating-axis-labels).

## Label Alignment & Padding<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-labels/label-style-alignment-and-positioning/#label-alignment--padding" class="hash-link" aria-label="Direct link to Label Alignment &amp; Padding" translate="no" title="Direct link to Label Alignment &amp; Padding">â€‹</a>

The **labelStyle** option on an axis contains **padding** and **alignment** which can be used to adjust the positioning of axis labels.

Padding refers to the space around the label.Â 

- By default, a label will not be displayed if if would overlap with the previous label, and this overlap includes padding.Â 
- By default the padding is 4 pixels left andÂ right, 2 pixels top,Â and 0 bottom padding.Â This is because the font height includes space below the baseline of the text.Â 
  - For numbers this usually results in nicely centered labels for a vertical axis, but depending on your font size and style, or if you are using text, you may want to adjust the padding to improve the vertical alignment, or to fit in labels that would otherwise be hidden.
- Alignment is anÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/elabelalignment.html" rel="noopener noreferrer" target="_blank">ELabelAlignmentðŸ“˜</a> which can be one of the options below.Â  Auto is the default.Â 

## KeepLabelsWithinAxis property<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-labels/label-style-alignment-and-positioning/#keeplabelswithinaxis-property" class="hash-link" aria-label="Direct link to KeepLabelsWithinAxis property" translate="no" title="Direct link to KeepLabelsWithinAxis property">â€‹</a>

Another property which defines label placement is keepLabelsWithinAxis.

By default the first and last labels on an axis are shifted so that they stay within the bounds of the axis itself.Â  If you want to turn this off so that all labels are centered, you can disableÂ **keepLabelsWithinAxis** as follows:

``` prism-code
// keepLabelsWithinAxis Example

// Either
const xAxis = new NumericAxis(wasmContext, {
    // Allow labels to overlap
    keepLabelsWithinAxis: false
});
// Or
const xAxis = new NumericAxis(wasmContext);
// Allow rotated labels to overlap
xAxis.axisRenderer.keepLabelsWithinAxis= false;
```

## Worked Example: Alignment of labels<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-labels/label-style-alignment-and-positioning/#worked-example-alignment-of-labels" class="hash-link" aria-label="Direct link to Worked Example: Alignment of labels" translate="no" title="Direct link to Worked Example: Alignment of labels">â€‹</a>

In the example below we show how to apply theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/elabelalignment.html" rel="noopener noreferrer" target="_blank">ELabelAlignmentðŸ“˜</a>Â enum to an axis. We've chosenÂ [LogarithmicAxis](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-types/logarithmic-axis/) for this demo to get different length labels, such as "10", "100", "1000". Try editing the label alignment in the sandbox below to see how it affects the chart.

- TS
- Builder API (JSON Config)

``` prism-code
// Demonstrates how to configure label alignment in SciChart.js
const {
    SciChartSurface,
    NumericAxis,
    LogarithmicAxis,
    SciChartJsNavyTheme,
    EAxisAlignment,
    ELabelAlignment,
    NumberRange,
    ENumericFormat
} = SciChart;

// or, for npm, import { SciChartSurface, ... } from "scichart"

const { wasmContext, sciChartSurface } = await SciChartSurface.create(divElementId, {
    theme: new SciChartJsNavyTheme()
});

// Create an XAxis on the bottom
const xAxis = new NumericAxis(wasmContext, {
    axisTitle: "X Axis, center aligned labels",
    keepLabelsWithinAxis: false,
    axisBorder: { color: "#50C7E077", borderTop: 1 },
    backgroundColor: "#50C7E022"
});

// Add the xAxis to the chart
sciChartSurface.xAxes.add(xAxis);

// Creating a NumericAxis as a YAxis on the left
sciChartSurface.yAxes.add(
    new LogarithmicAxis(wasmContext, {
        axisTitle: "Y Axis, left-aligned labels",
        axisAlignment: EAxisAlignment.Left,
        labelFormat: ENumericFormat.Decimal,
        labelStyle: { alignment: ELabelAlignment.Left },
        visibleRange: new NumberRange(0.1, 1e6),
        logBase: 10,
        axisBorder: { color: "#50C7E077", borderRight: 1 },
        backgroundColor: "#50C7E022"
    })
);

// Creating a NumericAxis as a YAxis on the right
sciChartSurface.yAxes.add(
    new LogarithmicAxis(wasmContext, {
        axisTitle: "Y Axis, right-aligned labels",
        axisAlignment: EAxisAlignment.Right,
        labelFormat: ENumericFormat.Decimal,
        labelStyle: { alignment: ELabelAlignment.Right },
        visibleRange: new NumberRange(0.1, 1e6),
        logBase: 10,
        axisBorder: { color: "#50C7E077", borderLeft: 1 },
        backgroundColor: "#50C7E022"
    })
);
```

``` prism-code
// Demonstrates how to configure a numeric axis in SciChart.js using the Builder API
const {
    chartBuilder,
    EThemeProviderType,
    EAxisAlignment,
    ELabelAlignment,
    EAxisType,
    ENumericFormat,
    NumberRange
} = SciChart;

// or, for npm, import { chartBuilder, ... } from "scichart"

const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    surface: { theme: { type: EThemeProviderType.Dark } },
    xAxes: {
        type: EAxisType.NumericAxis,
        options: {
            axisTitle: "X Axis, center aligned labels",
            keepLabelsWithinAxis: false,
            axisBorder: { color: "#50C7E077", borderTop: 1 },
            backgroundColor: "#50C7E022"
        }
    },
    yAxes: [
        {
            type: EAxisType.LogarithmicAxis,
            options: {
                axisTitle: "Y Axis, left-aligned labels",
                axisAlignment: EAxisAlignment.Left,
                labelFormat: ENumericFormat.Decimal,
                labelStyle: { alignment: ELabelAlignment.Left },
                visibleRange: new NumberRange(0.1, 1e6),
                logBase: 10,
                axisBorder: { color: "#50C7E077", borderRight: 1 },
                backgroundColor: "#50C7E022"
            }
        },
        {
            type: EAxisType.LogarithmicAxis,
            options: {
                axisTitle: "Y Axis, right-aligned labels",
                axisAlignment: EAxisAlignment.Right,
                labelFormat: ENumericFormat.Decimal,
                labelStyle: { alignment: ELabelAlignment.Right },
                visibleRange: new NumberRange(0.1, 1e6),
                logBase: 10,
                axisBorder: { color: "#50C7E077", borderLeft: 1 },
                backgroundColor: "#50C7E022"
            }
        }
    ]
});
```

This results in the following output:

![](out_scichartv4/2d-charts/axis-api/axis-labels/label-style-alignment-and-positioning/index_media/5a72c3ee6a9cc14296f901057d6eabf2b9b712b4.svg)note

Label alignment only applies to vertical axis.Â Â Labels for horizontal axesÂ are always centered horizontally.

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/axis-api/axis-labels/label-style-alignment-and-positioning/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/axis-api/axis-labels/label-style-alignment-and-positioning/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
