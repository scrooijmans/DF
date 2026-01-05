On this page

# Text / String Axis

There is no specific Text / String Axis in SciChart.js, however, with a combination ofÂ [the LabelProvider API](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-labels/label-provider-api-overview) and theÂ [NumericAxis](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-types/numeric-axis), it's possible to create one.

Use this technique if you want to use strings instead of numbers for the axis labels.

![](out_scichartv4/2d-charts/axis-api/axis-types/text-string-axis/index_media/af50301a53fa555616a9b2279e7e25a1d566cb1a.svg)info

Learn more about theÂ [commonalities between axis here](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-types/common-axis-base-type).

## Create and Configure a Text Axis<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-types/text-string-axis/#create-and-configure-a-text-axis" class="hash-link" aria-label="Direct link to Create and Configure a Text Axis" translate="no" title="Direct link to Create and Configure a Text Axis">â€‹</a>

To create a string axis in SciChart.js, we're going to use aÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/textlabelprovider.html" rel="noopener noreferrer" target="_blank">TextLabelProviderðŸ“˜</a>Â on an ordinaryÂ [NumericAxis](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-types/numeric-axis). This allows you to transform numbers \[0, 1, 2, 3, 4\] into string labels.

- TS
- Builder API (JSON Config)

``` prism-code
// Demonstrates how to configure a text axis in SciChart.js
// using TextLabelProvider & NumericAxis
const {
    SciChartSurface,
    NumericAxis,
    SciChartJsNavyTheme,
    TextLabelProvider,
    FastColumnRenderableSeries,
    XyDataSeries,
    GradientParams,
    Point
} = SciChart;

// or, for npm, import { SciChartSurface, ... } from "scichart"

const { wasmContext, sciChartSurface } = await SciChartSurface.create(divElementId, {
    theme: new SciChartJsNavyTheme()
});

// Create the labelProvider
const labelProvider = new TextLabelProvider({
    // When passed as an array, labels will be used in order
    labels: ["Bananas", "Apples", "Oranges", "Strawberries", "Plums"]
});

// Create an XAxis with a TextLabelProvider
const xAxis = new NumericAxis(wasmContext, { labelProvider });
sciChartSurface.xAxes.add(xAxis);

// Create a YAxis
sciChartSurface.yAxes.add(new NumericAxis(wasmContext));

// Create a column chart with the data. Labels are mapped to sequential X-values
sciChartSurface.renderableSeries.add(
    new FastColumnRenderableSeries(wasmContext, {
        dataSeries: new XyDataSeries(wasmContext, {
            xValues: [0, 1, 2, 3, 4],
            yValues: [0.1, 0.2, 0.4, 0.8, 1.1]
        }),
        fillLinearGradient: new GradientParams(new Point(0, 0), new Point(0, 1), [
            { color: "rgba(70,130,180,0.77)", offset: 0 },
            { color: "rgba(70,130,180,0.0)", offset: 1 }
        ]),
        stroke: "#FFFFFF77",
        strokeThickness: 2
    })
);
```

``` prism-code
// Demonstrates how to configure a text axis in SciChart.js using the Builder API
const { chartBuilder, ESeriesType, EThemeProviderType, ELabelProviderType, EAxisType } = SciChart;

// or, for npm, import { chartBuilder, ... } from "scichart"

const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    surface: { theme: { type: EThemeProviderType.Dark } },
    xAxes: {
        type: EAxisType.NumericAxis,
        options: {
            labelProvider: {
                type: ELabelProviderType.Text,
                options: {
                    // When passed as an array, labels will be used in order
                    labels: ["Bananas", "Apples", "Oranges", "Strawberries", "Plums"]
                }
            }
        }
    },
    yAxes: {
        type: EAxisType.NumericAxis,
        options: {}
    },
    series: [
        {
            type: ESeriesType.ColumnSeries,
            xyData: {
                xValues: [0, 1, 2, 3, 4],
                yValues: [0.1, 0.2, 0.4, 0.8, 1.1]
            },
            options: {
                fillLinearGradient: {
                    gradientStops: [
                        { color: "rgba(70,130,180,0.77)", offset: 0.0 },
                        { color: "rgba(70,130,180,0.0)", offset: 1 }
                    ],
                    startPoint: { x: 0, y: 0 },
                    endPoint: { x: 0, y: 1 }
                },
                stroke: "#FFFFFF77",
                strokeThickness: 2
            }
        }
    ]
});
```

This results in the following output:

## Controlling the Order of Labels<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-types/text-string-axis/#controlling-the-order-of-labels" class="hash-link" aria-label="Direct link to Controlling the Order of Labels" translate="no" title="Direct link to Controlling the Order of Labels">â€‹</a>

If you want to control the order of labels with data, pass the labels as an object, using numbers as fields:

- TS
- Builder API (JSON Config)

``` prism-code
// Create the labelProvider
const labelProvider = new TextLabelProvider({
    // When passed as an object, x values will be mapped to fields
    labels: {
        0: "Plums",
        1: "Strawberries",
        2: "Oranges",
        3: "Apples",
        4: "Bananas"
    }
});

// Create an XAxis with a TextLabelProvider
const xAxis = new NumericAxis(wasmContext, { labelProvider });
sciChartSurface.xAxes.add(xAxis);

// Data values are
//   xValues: [0,1,2,3,4],
//   yValues: [0.1, 0.2, 0.4, 0.8, 1.1],
```

``` prism-code
const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    surface: { theme: { type: EThemeProviderType.Dark } },
    xAxes: {
        type: EAxisType.NumericAxis,
        options: {
            labelProvider: {
                type: ELabelProviderType.Text,
                options: {
                    // When passed as an object, x values will be mapped to fields
                    labels: {
                        0: "Plums",
                        1: "Strawberries",
                        2: "Oranges",
                        3: "Apples",
                        4: "Bananas"
                    }
                }
            }
        }
    },
    // ... );
```

With the same data as before, the label order is reversed.

## Multiline Text Labels<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-types/text-string-axis/#multiline-text-labels" class="hash-link" aria-label="Direct link to Multiline Text Labels" translate="no" title="Direct link to Multiline Text Labels">â€‹</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/textlabelprovider.html" rel="noopener noreferrer" target="_blank">TextLabelProviderðŸ“˜</a> has aÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/textlabelprovider.html#maxlength" rel="noopener noreferrer" target="_blank">maxLengthðŸ“˜</a> option which if greater than 0 will do basic word-wrap to that number of characters. The text will only be split at spaces. Words will not be split.

You can ensure the lines appear exactly as you want by passing them as an array. Note that this can be used in conjunction with word wrap using maxLength. A label given as text will be split according to the maxLength, but one passed as an array will be displayed exactly as given, as shown in the following example:

- TS
- Builder API (JSON Config)

``` prism-code
// Create an XAxis with 90-degree rotated labels
const xAxis = new NumericAxis(wasmContext, {
    labelProvider: new TextLabelProvider({
        // When passed as an object, x values will be mapped to fields
        labels: [
            // Provide multiple lines directly
            ["Apples", "and Bananas"],
            ["Strawberries", "and Raspberries"],
            ["Lemons, Limes", "and Oranges"],
            // These will be auto-wrapped
            "Apples and Bananas",
            "Strawberries and Raspberries",
            "Lemons Limes and Oranges"
        ],
        maxLength: 7
    }),
    labelStyle: {
        alignment: ELabelAlignment.Center
    }
});
sciChartSurface.xAxes.add(xAxis);
```

``` prism-code
const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    surface: { theme: { type: EThemeProviderType.Dark } },
    xAxes: {
        type: EAxisType.NumericAxis,
        options: {
            labelProvider: {
                type: ELabelProviderType.Text,
                options: {
                    labels: [
                        // Provide multiple lines directly
                        ["Apples", "and Bananas"],
                        ["Strawberries", "and Raspberries"],
                        ["Lemons, Limes", "and Oranges"],
                        // These will be auto-wrapped
                        "Apples and Bananas",
                        "Strawberries and Raspberries",
                        "Lemons Limes and Oranges"
                    ],
                    maxLength: 7
                }
            }
        }
    },
    // ... );
```

This results in the following output:

Note the difference between the way the first three labels are wrapped, compared to the second three.

![](out_scichartv4/2d-charts/axis-api/axis-types/text-string-axis/index_media/5a72c3ee6a9cc14296f901057d6eabf2b9b712b4.svg)note

You can provide an alternative wrapping function by overriding theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/textlabelprovider.html#wraptext" rel="noopener noreferrer" target="_blank">TextLabelProvider.wrapTextðŸ“˜</a> method which takes the label text and returns an array of lines.

When using multiline, theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/textlabelprovider.html#linespacing" rel="noopener noreferrer" target="_blank">TextLabelProvider.lineSpacingðŸ“˜</a> option controls the line spacing. It is expressed as a fraction of the normal line height and defaults to 1.1, ie 10% spacing between lines.

### Further notes on Label Culling & Spacing<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-types/text-string-axis/#further-notes-on-label-culling--spacing" class="hash-link" aria-label="Direct link to Further notes on Label Culling &amp; Spacing" translate="no" title="Direct link to Further notes on Label Culling &amp; Spacing">â€‹</a>

![](out_scichartv4/2d-charts/axis-api/axis-types/text-string-axis/index_media/5a72c3ee6a9cc14296f901057d6eabf2b9b712b4.svg)note

The TextLabelProvider obeys other rules of axis tick spacing and label culling. Take a look at the section onÂ [Gridline and Label Spacing (Interval)](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-tick-label-interval/gridline-and-label-spacing-interval) for some more information how this works.

Finally, the propertyÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisrenderer.html#hideoverlappinglabels" rel="noopener noreferrer" target="_blank">axis.axisRenderer.hideOverlappingLabelsðŸ“˜</a> may be set to false if you wish to disable culling of labels which overlap. This property may also be set via the axis constructor optionÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iaxisbase2doptions.html#hideoverlappinglabels" rel="noopener noreferrer" target="_blank">hideOverlappingLabelsðŸ“˜</a>.

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/axis-api/axis-types/text-string-axis/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/axis-api/axis-types/text-string-axis/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
