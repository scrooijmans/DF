On this page

# The DateTimeNumericAxis

TheÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datetimenumericaxis.html" rel="noopener noreferrer" target="_blank">DateTimeNumericAxisðŸ“˜</a>Â is a Value axis (subclass ofÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numericaxis.html" rel="noopener noreferrer" target="_blank">NumericAxisðŸ“˜</a>) and has some extra formatting options and features for handling date formatting.

![](out_scichartv4/2d-charts/axis-api/axis-types/date-time-numeric-axis/index_media/af50301a53fa555616a9b2279e7e25a1d566cb1a.svg)info

Learn more about theÂ [commonalities between axis here](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-types/common-axis-base-type).

## Create and Configure a DateTimeNumericAxis<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-types/date-time-numeric-axis/#create-and-configure-a-datetimenumericaxis" class="hash-link" aria-label="Direct link to Create and Configure a DateTimeNumericAxis" translate="no" title="Direct link to Create and Configure a DateTimeNumericAxis">â€‹</a>

Dates in SciChart.js are treated as Linux timestamps divided by 1000 (to get seconds from milliseconds). e.g. to create a DateTimeNumericAxis in SciChart.js, use the following code:

- TS
- Builder API (JSON Config)

``` prism-code
const { wasmContext, sciChartSurface } = await SciChartSurface.create(divElementId, {
    theme: new SciChartJsNavyTheme()
});

// If you want to show an XAxis with dates between 1st March 2023 and 10th March 2023
const minDate = new Date("2023-03-01");
const maxDate = new Date("2023-03-10");

// Create the axis. SmartDateLabelProvider is automatically applied to labelProvider property
const xAxis = new DateTimeNumericAxis(wasmContext, {
    axisTitle: "X Axis / DateTime",
    // We need to specify some visibleRange to see these two dates
    // SciChart.js expects linux timestamp / 1000
    visibleRange: new NumberRange(minDate.getTime() / 1000, maxDate.getTime() / 1000)
});

// Add the xAxis to the chart
sciChartSurface.xAxes.add(xAxis);

// Creating a NumericAxis as a YAxis on the left
sciChartSurface.yAxes.add(
    new NumericAxis(wasmContext, {
        axisTitle: "Y Axis, Numeric",
        axisAlignment: EAxisAlignment.Left
    })
);

// Add a series to the chart with X-data as dates using unix Timestamp / 1000
//
const xValues = [];
const yValues = [];
let startDate = minDate.getTime() / 1000;
for (let i = 0; i <= 10; i++) {
    xValues.push(startDate);
    yValues.push(Math.random() * 0.1 + (i > 0 ? yValues[i - 1] : 0));
    startDate += 86400; // number of seconds in a day
}
sciChartSurface.renderableSeries.add(
    new FastLineRenderableSeries(wasmContext, {
        dataSeries: new XyDataSeries(wasmContext, { xValues, yValues }),
        strokeThickness: 3,
        stroke: "#50C7E0"
    })
);
// Note console log out xValues to see they are unix timestamps / 1000
console.log("xValues: " + xValues);
```

``` prism-code
// If you want to show an XAxis with dates between 1st March 2023 and 10th March 2023
const minDate = new Date("2023-03-01");
const maxDate = new Date("2023-03-10");

// Create data for the chart with X-data as dates using unix Timestamp / 1000
const xValues = [];
const yValues = [];
let startDate = minDate.getTime() / 1000;
for (let i = 0; i <= 10; i++) {
    xValues.push(startDate);
    yValues.push(Math.random() * 0.1 + (i > 0 ? yValues[i - 1] : 0));
    startDate += 86400; // number of seconds in a day
}

const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    surface: { theme: { type: EThemeProviderType.Dark } },
    xAxes: {
        type: EAxisType.DateTimeNumericAxis,
        options: {
            axisTitle: "X Axis / DateTime",
            // We need to specify some visibleRange to see these two dates
            // SciChart.js expects linux timestamp / 1000
            visibleRange: new NumberRange(minDate.getTime() / 1000, maxDate.getTime() / 1000)
        }
    },
    yAxes: {
        type: EAxisType.NumericAxis,
        options: {
            axisTitle: "Y Axis, Left, default formatting",
            axisAlignment: EAxisAlignment.Left
        }
    },
    series: [
        {
            type: ESeriesType.LineSeries,
            options: {
                strokeThickness: 3,
                stroke: "#50C7E0"
            },
            xyData: { xValues, yValues }
        }
    ]
});
```

This results in the following output:

Two fundamental differences of DateTimeNumericAxis and NumericAxis are that aÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/smartdatelabelprovider.html" rel="noopener noreferrer" target="_blank">SmartDateLabelProviderðŸ“˜</a> is applied to the labelProvider property andÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datetimedeltacalculator.html" rel="noopener noreferrer" target="_blank">DateTimeDeltaCalculatorðŸ“˜</a> is applied to the deltaCalculator property. This allows for more intuitive date formatting & handling when zooming the chart. Try it!

## Date / Label Formatting Options<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-types/date-time-numeric-axis/#date--label-formatting-options" class="hash-link" aria-label="Direct link to Date / Label Formatting Options" translate="no" title="Direct link to Date / Label Formatting Options">â€‹</a>

### Configuring Default behaviour with the SmartDateLabelProvider<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-types/date-time-numeric-axis/#configuring-default-behaviour-with-the-smartdatelabelprovider" class="hash-link" aria-label="Direct link to Configuring Default behaviour with the SmartDateLabelProvider" translate="no" title="Direct link to Configuring Default behaviour with the SmartDateLabelProvider">â€‹</a>

You'll notice above the Date formatting is quite intuitive out of the box, and dynamically changes on zoom. The more zoomed in you are, the finer grained the date labels e.g. Month/Day becomes Day/Hour, and Day/Hour becomes Hour/Minute. This behaviour is provided by theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/smartdatelabelprovider.html" rel="noopener noreferrer" target="_blank">SmartDateLabelProviderðŸ“˜</a>Â which is assigned to theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase2d.html#labelprovider" rel="noopener noreferrer" target="_blank">Axis.LabelProviderðŸ“˜</a> property by default.

This behaviour is pretty fixed, however some options of theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/smartdatelabelprovider.html" rel="noopener noreferrer" target="_blank">SmartDateLabelProviderðŸ“˜</a>Â are below:

![](out_scichartv4/2d-charts/axis-api/axis-types/date-time-numeric-axis/index_media/af50301a53fa555616a9b2279e7e25a1d566cb1a.svg)info

The properties on SmartDateLabelProvider can be found in theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/smartdatelabelprovider.html" rel="noopener noreferrer" target="_blank">TypeDoc API documentationðŸ“˜</a>.

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/smartdatelabelprovider.html#cursornumericformat" rel="noopener noreferrer" target="_blank">cursorNumericFormatðŸ“˜</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/smartdatelabelprovider.html#cursorprecision" rel="noopener noreferrer" target="_blank">cursorPrecisionðŸ“˜</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/smartdatelabelprovider.html#formatcursorlabel" rel="noopener noreferrer" target="_blank">formatCursorLabelðŸ“˜</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/smartdatelabelprovider.html#formatlabel" rel="noopener noreferrer" target="_blank">formatLabelðŸ“˜</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/smartdatelabelprovider.html#linespacing" rel="noopener noreferrer" target="_blank">lineSpacingðŸ“˜</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/smartdatelabelprovider.html#numericformat" rel="noopener noreferrer" target="_blank">numericFormatðŸ“˜</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/smartdatelabelprovider.html#postfix" rel="noopener noreferrer" target="_blank">postfixðŸ“˜</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/smartdatelabelprovider.html#precision" rel="noopener noreferrer" target="_blank">precisionðŸ“˜</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/smartdatelabelprovider.html#prefix" rel="noopener noreferrer" target="_blank">prefixðŸ“˜</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/smartdatelabelprovider.html#rotation" rel="noopener noreferrer" target="_blank">rotationðŸ“˜</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/smartdatelabelprovider.html#showwiderdateonfirstlabel" rel="noopener noreferrer" target="_blank">showWiderDateOnFirstLabelðŸ“˜</a>

### Further customising the DateTimeNumericAxis Label Output<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-types/date-time-numeric-axis/#further-customising-the-datetimenumericaxis-label-output" class="hash-link" aria-label="Direct link to Further customising the DateTimeNumericAxis Label Output" translate="no" title="Direct link to Further customising the DateTimeNumericAxis Label Output">â€‹</a>

There isn't much option at the moment for customising theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datetimenumericaxis.html" rel="noopener noreferrer" target="_blank">DateTimeNumericAxisðŸ“˜</a> label formatting when using the defaultÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/smartdatelabelprovider.html" rel="noopener noreferrer" target="_blank">SmartDateLabelProviderðŸ“˜</a>, however, it is possible to substitute the simplerÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datelabelprovider.html" rel="noopener noreferrer" target="_blank">DateLabelProviderðŸ“˜</a>Â which doesn't have dynamic updating labels on zoom, and to specify your own formats.

It is also possible to create a custom labelprovider class and have complete control over axis label output. More on that in theÂ [Custom Label Providers documentation page](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-labels/custom-label-providers-dynamic-dates-on-zoom).

Here's a quick example:

- TS
- Builder API (JSON Config)

``` prism-code
// If you want to show an XAxis with custom label formats
const minDate = new Date("2023-03-01");
const maxDate = new Date("2023-03-03");

// Create the axis. SmartDateLabelProvider is automatically applied to labelProvider property
const xAxis = new DateTimeNumericAxis(wasmContext, {
    axisTitle: "X Axis / DateTime",
    visibleRange: new NumberRange(minDate.getTime() / 1000, maxDate.getTime() / 1000),
    // Specify a DateLabelProvider with format to override the built-in behaviour
    labelProvider: new DateLabelProvider({ labelFormat: ENumericFormat.Date_DDMMYYYY })
});

// When zoomed in to less than one day, switch the date format
xAxis.visibleRangeChanged.subscribe(arg => {
    const SECONDS_IN_DAY = 86400;
    const SECONDS_IN_HOUR = 3600;
    if (arg.visibleRange.max - arg.visibleRange.min < SECONDS_IN_HOUR) {
        xAxis.labelProvider.numericFormat = ENumericFormat.Date_HHMMSS;
    } else if (arg.visibleRange.max - arg.visibleRange.min < SECONDS_IN_DAY) {
        xAxis.labelProvider.numericFormat = ENumericFormat.Date_HHMM;
    } else {
        xAxis.labelProvider.numericFormat = ENumericFormat.Date_DDMMYYYY;
    }
});

// Note other options include overriding labelProvider.formatLabel,
// or custom labelproviders

// Add the xAxis to the chart
sciChartSurface.xAxes.add(xAxis);
```

``` prism-code
// If you want to show an XAxis with dates and dynamic label formats
const minDate = new Date("2023-03-01");
const maxDate = new Date("2023-03-03");

const { sciChartSurface, wasmContext } = await chartBuilder.build2DChart(divElementId, {
    surface: { theme: { type: EThemeProviderType.Dark } },
    xAxes: {
        type: EAxisType.DateTimeNumericAxis,
        options: {
            axisTitle: "X Axis / DateTime",
            // We need to specify some visibleRange to see these two dates
            // SciChart.js expects linux timestamp / 1000
            visibleRange: new NumberRange(minDate.getTime() / 1000, maxDate.getTime() / 1000),
            labelProvider: {
                type: ELabelProviderType.Date,
                options: {
                    labelFormat: ENumericFormat.Date_DDMMYYYY
                }
            }
        }
    },
    yAxes: {
        type: EAxisType.NumericAxis,
        options: {
            axisTitle: "Y Axis, Left, default formatting",
            axisAlignment: EAxisAlignment.Left
        }
    },
    modifiers: [{ type: EChart2DModifierType.MouseWheelZoom }]
});

const xAxis = sciChartSurface.xAxes.get(0);
// When zoomed in to less than one day, switch the date format
xAxis.visibleRangeChanged.subscribe(arg => {
    const SECONDS_IN_DAY = 86400;
    const SECONDS_IN_HOUR = 3600;
    if (arg.visibleRange.max - arg.visibleRange.min < SECONDS_IN_HOUR) {
        xAxis.labelProvider.numericFormat = ENumericFormat.Date_HHMMSS;
    } else if (arg.visibleRange.max - arg.visibleRange.min < SECONDS_IN_DAY) {
        xAxis.labelProvider.numericFormat = ENumericFormat.Date_HHMM;
    } else {
        xAxis.labelProvider.numericFormat = ENumericFormat.Date_DDMMYYYY;
    }
});
```

This code example above shows how you can swap the defaultÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/smartdatelabelprovider.html" rel="noopener noreferrer" target="_blank">SmartDateLabelProviderðŸ“˜</a>Â on theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datetimenumericaxis.html" rel="noopener noreferrer" target="_blank">DateTimeNumericAxisðŸ“˜</a>Â for a simplerÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datelabelprovider.html" rel="noopener noreferrer" target="_blank">DateLabelProviderðŸ“˜</a>, then subscribe to axis.visibleRangeChanged to dynamically change the labelformat.

This results in the following output:

![](out_scichartv4/2d-charts/axis-api/axis-types/date-time-numeric-axis/index_media/af50301a53fa555616a9b2279e7e25a1d566cb1a.svg)info

Other options are available, such as implementing aÂ [custom LabelProvider](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-labels/custom-label-providers-dynamic-dates-on-zoom). OverridingÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/labelprovider.html#formatlabel" rel="noopener noreferrer" target="_blank">LabelProvider.formatLabelðŸ“˜</a> and formatCursorLabel allows for complete control over axis labels.

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/axis-api/axis-types/date-time-numeric-axis/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/axis-api/axis-types/date-time-numeric-axis/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
