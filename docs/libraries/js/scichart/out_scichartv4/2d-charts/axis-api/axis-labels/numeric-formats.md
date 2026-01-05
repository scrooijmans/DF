On this page

# Axis Labels - The ENumericFormat Enum

In the previous article we gave you an overview of the LabelProvider feature and how this is used to format labels on axis.

You'll notice in the many axis label code samples, labels are formatted by passing inÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/enumericformat.html" rel="noopener noreferrer" target="_blank">ENumericFormatðŸ“˜</a>Â to the constructor options of the axis. What's actually happening here is parameters are being passed to theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#labelprovider" rel="noopener noreferrer" target="_blank">AxisCore.labelProviderðŸ“˜</a>.

[NumericAxis](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-types/numeric-axis/) is quite versatile, it can format decimals with label prefix and postfix to any number of decimal places or significant figures. It can even format numbers as dates (assuming number is a unix stamp). You can find out all the options of theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/enumericformat.html" rel="noopener noreferrer" target="_blank">ENumericFormatðŸ“˜</a>Â enum below, or on ourÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/enumericformat.html" rel="noopener noreferrer" target="_blank">TypeDocðŸ“˜</a>.

``` prism-code
// ENumericFormat
// import {ENumericFormat} from "scichart";
export enum ENumericFormat {
    /** No format, return the string representation unchanged */
    NoFormat = "NoFormat",
    /** Format to a specified number of decimal places */
    Decimal = "Decimal",
    /** Format to a specified number of significant figures */
    SignificantFigures = "SignificantFigures",
    /** Format as a date in format DD/MM/YYYY */
    Date_DDMMYYYY = "Date_DDMMYYYY",
    /** Format as a date in format DD/MM/YY */
    Date_DDMMYY = "Date_DDMMYY",
    /** Format as a date in format DD/MM HH:MM */
    Date_DDMMHHMM = "Date_DDMMHHMM",
    /** Format as a date in format DD/MM */
    Date_DDMM = "Date_DDMM",
    /** Format as a date in format HH:MM */
    Date_HHMM = "Date_HHMM",
    /** Format as a date in format HH:MM:SS */
    Date_HHMMSS = "Date_HHMMSS",
    /** Format as a date in format SS.ms */
    Date_SSms = "Date_SSms",
    /**
     * Format using Exponential notation to a specified number of significant figures eg 1.0E0, 1.5E1, 2.7E2
     * Note that this will ALWAYS be base 10, even when used on a Logarithmic axis
     */
    Exponential = "Exponential",
    /**
     * Format using Scientific notation to a specified number of significant figures eg 1.0x10^1, 1.5x10^2, 2.7x10^3
     * On a Logarithmic axis, the base will be the same as the axis logarithmic base
     */
    Scientific = "Scientific",
    /**
     * Engineering notation, eg 12.32K, 1.5M, 2.7G
     * @feature You can pass your custom prefixes as {@link IEngineeringPrefix}
     */
    Engineering = "Engineering"
}
```

## Demonstrating the Different ENumericFormats<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-labels/numeric-formats/#demonstrating-the-different-enumericformats" class="hash-link" aria-label="Direct link to Demonstrating the Different ENumericFormats" translate="no" title="Direct link to Demonstrating the Different ENumericFormats">â€‹</a>

Below we've created an example that demonstrates four of theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/enumericformat.html" rel="noopener noreferrer" target="_blank">ENumericFormatðŸ“˜</a>Â values: **Date_DDMMYYYY**, **Engineering**, **Scientific** and **Decimal**.

Try to edit the code in the Codepen below and pan the chart to see the effect of different ENumericFormat values.

- TS
- Builder API (JSON Config)

``` prism-code

// Bottom XAxis has Date formatting to convert unix timestamps
// See also DateTimeAxis and SmartDateLabelProvider
sciChartSurface.xAxes.add(
    new NumericAxis(wasmContext, {
        labelFormat: ENumericFormat.Date_DDMMYYYY,
        axisTitle: "ENumericFormat.Date_DDMMYYYY",
        visibleRange: new NumberRange(1577833200, 1640991600),
        axisTitleStyle: { fontSize: 16, padding: new Thickness(20, 10, 10, 10) }
    })
);

sciChartSurface.xAxes.add(
    new LogarithmicAxis(wasmContext, {
        labelFormat: ENumericFormat.Scientific,
        axisTitle: "ENumericFormat.Scientific",
        visibleRange: new NumberRange(1, 1000000000000),
        axisAlignment: EAxisAlignment.Top,
        axisTitleStyle: { fontSize: 16, padding: new Thickness(10, 10, 20, 10) }
    })
);

// Right YAxis has Decimal formatting with 2 decimal points
sciChartSurface.yAxes.add(
    new NumericAxis(wasmContext, {
        labelFormat: ENumericFormat.Decimal,
        cursorLabelFormat: ENumericFormat.Decimal,
        labelPrecision: 2,
        labelPrefix: "$",
        labelPostfix: " USD",
        axisTitle: "ENumericFormat.Decimal",
        axisTitleStyle: { fontSize: 16 }
    })
);

// Left YAxis has Engineering formatting (1k, 1M, 1B, 1T)
sciChartSurface.yAxes.add(
    new LogarithmicAxis(wasmContext, {
        labelFormat: ENumericFormat.Engineering,
        axisTitle: "ENumericFormat.Engineering",
        visibleRange: new NumberRange(1, 1000000000000),
        labelPrecision: 0,
        axisAlignment: EAxisAlignment.Left,
        axisTitleStyle: { fontSize: 16 }
    })
);
```

``` prism-code
const { sciChartSurface, wasmContext } = await chartBuilder.build2DChart(divElementId, {
    surface: { theme: { type: EThemeProviderType.Dark } },
    xAxes: [
        {
            type: EAxisType.NumericAxis,
            options: {
                labelFormat: ENumericFormat.Date_DDMMYYYY,
                axisTitle: "ENumericFormat.Date_DDMMYYYY",
                visibleRange: new NumberRange(1577833200, 1640991600),
                axisTitleStyle: {
                    fontSize: 16,
                    padding: new Thickness(20, 10, 10, 10)
                }
            }
        },
        {
            type: EAxisType.LogarithmicAxis,
            options: {
                labelFormat: ENumericFormat.Scientific,
                axisTitle: "ENumericFormat.Scientific",
                visibleRange: new NumberRange(1, 1000000000000),
                axisAlignment: EAxisAlignment.Top,
                axisTitleStyle: {
                    fontSize: 16,
                    padding: new Thickness(10, 10, 20, 10)
                }
            }
        }
    ],
    yAxes: [
        {
            type: EAxisType.NumericAxis,
            options: {
                labelFormat: ENumericFormat.Decimal,
                cursorLabelFormat: ENumericFormat.Decimal,
                labelPrecision: 2,
                labelPrefix: "$",
                labelPostfix: " USD",
                axisTitle: "ENumericFormat.Decimal",
                axisTitleStyle: { fontSize: 16 }
            }
        },
        {
            type: EAxisType.NumericAxis,
            options: {
                labelFormat: ENumericFormat.Engineering,
                axisTitle: "ENumericFormat.Engineering",
                visibleRange: new NumberRange(1, 1000000000000),
                labelPrecision: 0,
                axisAlignment: EAxisAlignment.Left,
                axisTitleStyle: { fontSize: 16 }
            }
        }
    ]
});
```

This results in the following output:

![](out_scichartv4/2d-charts/axis-api/axis-labels/numeric-formats/index_media/af50301a53fa555616a9b2279e7e25a1d566cb1a.svg)info

Further enhancement of the NumericAxis labels including custom formatting, string formatting or dynamic formatting can be achieved with theÂ [LabelProvider API](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-labels/label-provider-api-overview/).

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/axis-api/axis-labels/numeric-formats/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/axis-api/axis-labels/numeric-formats/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
