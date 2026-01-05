On this page

# The Logarithmic Axis

TheÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/logarithmicaxis.html" rel="noopener noreferrer" target="_blank">LogarithicAxisðŸ“˜</a> is a Value axis similar to the normal NumericAxis, but where the tick values increase exponentially. Plotting data on such an axis is equivalent to plotting the log of that data. You can set the logarithmic base using the logBase property. eg logBase: 10 (the default) will result in ticks like 1, 10, 100, 1000. logBase 2 will result in ticks like 2, 4, 8, 16, 32.

![](out_scichartv4/2d-charts/axis-api/axis-types/logarithmic-axis/index_media/af50301a53fa555616a9b2279e7e25a1d566cb1a.svg)info

Learn more about theÂ [commonalities between axis here](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-types/common-axis-base-type).

## Create and Configure a LogarithmicAxis<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-types/logarithmic-axis/#create-and-configure-a-logarithmicaxis" class="hash-link" aria-label="Direct link to Create and Configure a LogarithmicAxis" translate="no" title="Direct link to Create and Configure a LogarithmicAxis">â€‹</a>

To create and configureÂ aÂ LogarithmicAxis, use the following code:Â 

- TS
- Builder API (JSON Config)

``` prism-code
// Demonstrates how to configure a logarithmic axis in SciChart.js
const { SciChartSurface, LogarithmicAxis, SciChartJsNavyTheme, ENumericFormat, NumberRange } = SciChart;

// or, for npm, import { SciChartSurface, ... } from "scichart"

const { wasmContext, sciChartSurface } = await SciChartSurface.create(divElementId, {
    theme: new SciChartJsNavyTheme()
});

// Create an X and Y Axis
const xAxisLogarithmic = new LogarithmicAxis(wasmContext, {
    logBase: 10,
    // Format with E
    labelFormat: ENumericFormat.Exponential,
    labelPrecision: 2,
    minorsPerMajor: 10,
    // Adjust major/minor gridline style to make it clearer for the demo
    majorGridLineStyle: { color: "#50C7E077" },
    minorGridLineStyle: { color: "#50C7E033" },
    axisTitle: "Log(10) Axis with Exponential Format",
    visibleRange: new NumberRange(1, 10_000_000)
});
sciChartSurface.xAxes.add(xAxisLogarithmic);

// The LogarithmicAxis will apply logarithmic scaling and labelling to your data.
// Simply replace a NumericAxis for a LogarithmicAxis on X or Y to apply this scaling
// Note options logBase, labelFormat which lets you specify exponent on labels
const yAxisLogarithmic = new LogarithmicAxis(wasmContext, {
    logBase: 10,
    // Format with superscript
    labelFormat: ENumericFormat.Scientific,
    labelPrecision: 2,
    minorsPerMajor: 10,
    majorGridLineStyle: { color: "#50C7E077" },
    minorGridLineStyle: { color: "#50C7E033" },
    axisTitle: "Log(10) Axis with Scientific Format",
    visibleRange: new NumberRange(0.1, 1_000_000)
});
sciChartSurface.yAxes.add(yAxisLogarithmic);
```

``` prism-code
// Demonstrates how to create a line chart with SciChart.js using the Builder API
const { chartBuilder, EThemeProviderType, NumberRange, ENumericFormat, EAxisType } = SciChart;

// or, for npm, import { chartBuilder, ... } from "scichart"

const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    surface: { theme: { type: EThemeProviderType.Dark } },
    xAxes: {
        type: EAxisType.LogarithmicAxis,
        options: {
            logBase: 10,
            // Format with E
            labelFormat: ENumericFormat.Exponential,
            labelPrecision: 2,
            minorsPerMajor: 10,
            // Adjust major/minor gridline style to make it clearer for the demo
            majorGridLineStyle: { color: "#EEEEEE77" },
            minorGridLineStyle: { color: "#EEEEEE33" },
            axisTitle: "Log(10) Axis with Exponential Format",
            visibleRange: new NumberRange(1, 10_000_000)
        }
    },
    yAxes: {
        type: EAxisType.LogarithmicAxis,
        options: {
            logBase: 10,
            // Format with superscript
            labelFormat: ENumericFormat.Scientific,
            labelPrecision: 2,
            minorsPerMajor: 10,
            majorGridLineStyle: { color: "#EEEEEE77" },
            minorGridLineStyle: { color: "#EEEEEE33" },
            axisTitle: "Log(10) Axis with Scientific Format",
            visibleRange: new NumberRange(0.1, 1_000_000)
        }
    }
});
```

This results in the following output:

## Configuration Options for Log Axis<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-types/logarithmic-axis/#configuration-options-for-log-axis" class="hash-link" aria-label="Direct link to Configuration Options for Log Axis" translate="no" title="Direct link to Configuration Options for Log Axis">â€‹</a>

### labelFormat property<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-types/logarithmic-axis/#labelformat-property" class="hash-link" aria-label="Direct link to labelFormat property" translate="no" title="Direct link to labelFormat property">â€‹</a>

When using logarithmicAxis and the labelFormatÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/enumericformat.html" rel="noopener noreferrer" target="_blank">ENumericFormat.ScientificðŸ“˜</a>, the logBase of the axis will be used as the base for the label. This is NOT the case forÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/enumericformat.html" rel="noopener noreferrer" target="_blank">ENumericFormat.ExponentialðŸ“˜</a> which is always base 10.

``` prism-code
const logAxis = new LogarithmicAxis(wasmContext, {
    // Format with Scientific notation e.g. 1x10^3
    labelFormat: ENumericFormat.Scientific,
    labelPrecision: 2,
    minorsPerMajor: 10,
  });
```

### Plotting Negative Numbers<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-types/logarithmic-axis/#plotting-negative-numbers" class="hash-link" aria-label="Direct link to Plotting Negative Numbers" translate="no" title="Direct link to Plotting Negative Numbers">â€‹</a>

LogarithmicAxis cannot show both positive and negative numbers on the same axis, so if your data is negative you need to set isNegative on the axis.Â  If you need to show positive and negative log data, you need to split it into positive and negative sets and plot them on seperateÂ [vertically stacked axes](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/multi-axis-and-layout/vertically-stacked-axis-layout).

### MinorÂ Tick Mode<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-types/logarithmic-axis/#minortick-mode" class="hash-link" aria-label="Direct link to MinorÂ Tick Mode" translate="no" title="Direct link to MinorÂ Tick Mode">â€‹</a>

By default, Major gridlines are spaced logarithmically, and Minor gridlines are spaced linearly between them. If your visible range is extremely large, you may want to switch to logarithmic spacing for minor gridlines, which you can do with theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/logarithmicaxis.html#minortickmode" rel="noopener noreferrer" target="_blank">LogarithmicAxis.minorTickModeðŸ“˜</a> property which is anÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/elogarithmicmajortickmode.html" rel="noopener noreferrer" target="_blank">ELogarithmicMinorTickModeðŸ“˜</a>Â which can be Logarithmic, Linear or Auto

Auto mode meansÂ it switches from linear to Logarithmic when the visible range is such that the first linear minor tick would be more than 70% of the major tick

### Major Tick Mode - Financial Log Charts<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-types/logarithmic-axis/#major-tick-mode---financial-log-charts" class="hash-link" aria-label="Direct link to Major Tick Mode - Financial Log Charts" translate="no" title="Direct link to Major Tick Mode - Financial Log Charts">â€‹</a>

For financial charts you often want base 2, with a relatively small range, but you don't want your tick labels to be powers of 2. In this case setÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/logarithmicaxis.html#majortickmode" rel="noopener noreferrer" target="_blank">LogarithmicAxis.majorTickModeðŸ“˜</a> toÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/elogarithmicmajortickmode.html" rel="noopener noreferrer" target="_blank">ELogarithmicMajorTickMode.RoundNumbersðŸ“˜</a>. This will give you labels with nice round numbers, at the expense of gridlines that are not exactly equally spaced.

### LabelFormat<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-types/logarithmic-axis/#labelformat" class="hash-link" aria-label="Direct link to LabelFormat" translate="no" title="Direct link to LabelFormat">â€‹</a>

**labelFormat:Â <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/enumericformat.html" rel="noopener noreferrer" target="_blank">ENumericFormat.SignificantFiguresðŸ“˜</a>** is also helpful as it retains precision for very small values, while not resulting in unnecessary decimal places for large values.

## Worked Example - LogAxis Configuration Options<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-types/logarithmic-axis/#worked-example---logaxis-configuration-options" class="hash-link" aria-label="Direct link to Worked Example - LogAxis Configuration Options" translate="no" title="Direct link to Worked Example - LogAxis Configuration Options">â€‹</a>

Here's a worked example that combines some of the techniques above.

- TS
- Builder API (JSON Config)

``` prism-code
// Create Log(10) axis with options
sciChartSurface.xAxes.add(
    new LogarithmicAxis(wasmContext, {
        logBase: 10,
        // Format with E
        labelFormat: ENumericFormat.SignificantFigures,
        majorTickMode: ELogarithmicMajorTickMode.EqualSpacing,
        minorTickMode: ELogarithmicMinorTickMode.Logarithmic,
        // Adjust major/minor gridline style to make it clearer for the demo
        majorGridLineStyle: { color: "#50C7E077" },
        minorGridLineStyle: { color: "#50C7E033" },
        axisTitle: "Log(10) Axis with equally spaced gridlines",
        visibleRange: new NumberRange(1, 10_000_000)
    })
);

// Creating a Log(2) Axis with options
sciChartSurface.yAxes.add(
    new LogarithmicAxis(wasmContext, {
        logBase: 2,
        // Format with 2 decimal places
        labelFormat: ENumericFormat.Decimal,
        labelPrecision: 2,
        labelPrefix: "$",
        majorTickMode: ELogarithmicMajorTickMode.RoundNumbers,
        minorTickMode: ELogarithmicMinorTickMode.Linear,
        // Adjust major/minor gridline style to make it clearer for the demo
        majorGridLineStyle: { color: "#50C7E077" },
        minorGridLineStyle: { color: "#50C7E033" },
        axisTitle: "Log(2) Axis configured for financial",
        visibleRange: new NumberRange(100, 1000)
    })
);
```

``` prism-code
const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    surface: { theme: { type: EThemeProviderType.Dark } },
    xAxes: {
        type: EAxisType.LogarithmicAxis,
        options: {
            logBase: 10,
            // Format with E
            labelFormat: ENumericFormat.SignificantFigures,
            majorTickMode: ELogarithmicMajorTickMode.EqualSpacing,
            minorTickMode: ELogarithmicMinorTickMode.Logarithmic,
            // Adjust major/minor gridline style to make it clearer for the demo
            majorGridLineStyle: { color: "#50C7E077" },
            minorGridLineStyle: { color: "#50C7E033" },
            axisTitle: "Log(10) Axis with equally spaced gridlines",
            visibleRange: new NumberRange(1, 10_000_000)
        }
    },
    yAxes: {
        type: EAxisType.LogarithmicAxis,
        options: {
            logBase: 2,
            // Format with 2 decimal places
            labelFormat: ENumericFormat.Decimal,
            labelPrecision: 2,
            labelPrefix: "$",
            majorTickMode: ELogarithmicMajorTickMode.RoundNumbers,
            minorTickMode: ELogarithmicMinorTickMode.Linear,
            // Adjust major/minor gridline style to make it clearer for the demo
            majorGridLineStyle: { color: "#50C7E077" },
            minorGridLineStyle: { color: "#50C7E033" },
            axisTitle: "Log(2) Axis configured for financial",
            visibleRange: new NumberRange(100, 1000)
        }
    }
});
```

This produces something like this.Â  If you want even more control over the tick values and gridlines seeÂ [Axis Ticks](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-tick-label-interval/gridline-and-label-spacing-interval).

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-types/logarithmic-axis/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [Gridline and Label Spacing (Interval)](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-tick-label-interval/gridline-and-label-spacing-interval)
- [Vertically Stacked Axis Layout](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/multi-axis-and-layout/vertically-stacked-axis-layout)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/axis-api/axis-types/logarithmic-axis/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/axis-api/axis-types/logarithmic-axis/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
