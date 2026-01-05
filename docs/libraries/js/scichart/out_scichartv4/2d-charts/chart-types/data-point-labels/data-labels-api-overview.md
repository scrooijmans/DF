On this page

# DataLabels API Overview

DataLabels allow per data-point text labels to be drawn on series, or arbitrary text labels at x,y positions on the chart.

You can see several datalabel examples on the SciChart.js demo:

- <a href="https://www.scichart.com/demo/javascript-line-chart" rel="noopener noreferrer" target="_blank">The Line Chart example</a>
- <a href="https://www.scichart.com/demo/javascript-column-chart" rel="noopener noreferrer" target="_blank">The Column Chart example</a>
- <a href="https://www.scichart.com/demo/javascript-chart-color-points-individually-with-paletteprovider" rel="noopener noreferrer" target="_blank">The PaletteProvider example</a>
- <a href="https://www.scichart.com/demo/javascript-datalabels" rel="noopener noreferrer" target="_blank">The DataLabels demo</a>
- <a href="https://www.scichart.com/demo/javascript/stacked-column-chart" rel="noopener noreferrer" target="_blank">The Stacked Column Chart demo</a>
- <a href="https://www.scichart.com/demo/javascript/population-pyramid" rel="noopener noreferrer" target="_blank">The Population Pyramid demo</a>

Explore these for some rich examples of how to use this API.

## The DataLabels API<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-point-labels/data-labels-api-overview/#the-datalabels-api" class="hash-link" aria-label="Direct link to The DataLabels API" translate="no" title="Direct link to The DataLabels API">â€‹</a>

Each RenderableSeries as aÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries.html#datalabelprovider" rel="noopener noreferrer" target="_blank">dataLabelProviderðŸ“˜</a> property. Many also accept Data Label configuration viaÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibaserenderableseriesoptions.html#datalabelprovider" rel="noopener noreferrer" target="_blank">constructor optionsðŸ“˜</a>.

This defines whether text labels are rendered for data-points, and the style and positioning of these text labels.

Above: The JavaScript <a href="https://www.scichart.com/demo/iframe/datalabels" target="_blank">Data Labels Example</a> example from the <a href="https://www.scichart.com/demo/react" target="_blank">SciChart.js Demo</a>

  

## Adding Data Labels<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-point-labels/data-labels-api-overview/#adding-data-labels" class="hash-link" aria-label="Direct link to Adding Data Labels" translate="no" title="Direct link to Adding Data Labels">â€‹</a>

You an configure data labels for almost any series by setting a valid style on theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibaselinerenderableseriesoptions.html#datalabels" rel="noopener noreferrer" target="_blank">dataLabels propertyðŸ“˜</a> in the series options:

- TS
- Builder API (JSON Config)

``` prism-code
// Demonstrates how to add DataLabels to a chart with SciChart.js
const {
    SciChartSurface,
    NumericAxis,
    FastLineRenderableSeries,
    EllipsePointMarker,
    XyDataSeries,
    SciChartJsNavyTheme
} = SciChart;

// or, for npm, import { SciChartSurface, ... } from "scichart"

// Create a chart with X, Y axis
const { sciChartSurface, wasmContext } = await SciChartSurface.create(divElementId, {
    theme: new SciChartJsNavyTheme()
});

sciChartSurface.xAxes.add(new NumericAxis(wasmContext));
sciChartSurface.yAxes.add(new NumericAxis(wasmContext));

// Create a Line series with a pointmarker & some data
// We add dataLabels by setting the dataLabels constructor option
sciChartSurface.renderableSeries.add(
    new FastLineRenderableSeries(wasmContext, {
        stroke: "SteelBlue",
        strokeThickness: 3,
        pointMarker: new EllipsePointMarker(wasmContext, {
            width: 10,
            height: 10,
            strokeThickness: 2,
            stroke: "SteelBlue",
            fill: "LightSteelBlue"
        }),
        dataSeries: new XyDataSeries(wasmContext, {
            xValues: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            yValues: [4.3, 5.3, 6, 6.3, 6, 5.2, 4.5, 4.6, 5, 6, 7, 8]
        }),
        // Data labels are enabled here. Simply set style, color
        dataLabels: {
            style: {
                fontFamily: "Default",
                fontSize: 16
            },
            color: "#EEE"
        }
    })
);
```

``` prism-code
// Demonstrates how to add DataLabels to a chart with SciChart.js using the Builder API
const { chartBuilder, ESeriesType, EThemeProviderType, EPointMarkerType } = SciChart;

// or, for npm, import { chartBuilder, ... } from "scichart"

const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    surface: { theme: { type: EThemeProviderType.Dark } },
    series: [
        {
            type: ESeriesType.LineSeries,
            xyData: {
                xValues: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
                yValues: [4.3, 5.3, 6, 6.3, 6, 5.2, 4.5, 4.6, 5, 6, 7, 8]
            },
            options: {
                stroke: "#0066FF",
                strokeThickness: 5,
                pointMarker: {
                    type: EPointMarkerType.Ellipse,
                    options: {
                        width: 10,
                        height: 10,
                        strokeThickness: 2,
                        stroke: "SteelBlue",
                        fill: "LightSteelBlue"
                    }
                },
                // Data labels are enabled here. Simply set style, color
                dataLabels: {
                    style: {
                        fontFamily: "Default",
                        fontSize: 16
                    },
                    color: "#EEE"
                }
            }
        }
    ]
});
```

This results in the following output:

## Standard Label Formatting<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-point-labels/data-labels-api-overview/#standard-label-formatting" class="hash-link" aria-label="Direct link to Standard Label Formatting" translate="no" title="Direct link to Standard Label Formatting">â€‹</a>

Datalabels supports the same numeric format and precision options as axis labels. By default the Y-value is printed to the label. The numericFormat option is one of theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/enumericformat.html" rel="noopener noreferrer" target="_blank">ENumericFormatðŸ“˜</a> values.

- TS
- Builder API (JSON Config)

``` prism-code
sciChartSurface.renderableSeries.add(
    new FastLineRenderableSeries(wasmContext, {
        stroke: "SteelBlue",
        strokeThickness: 3,
        pointMarker,
        dataSeries,
        // Configure datalabel formatting using similar
        // numericFormat and precision options to the axis label formatting
        dataLabels: {
            numericFormat: ENumericFormat.Decimal,
            precision: 4,
            style: {
                fontFamily: "Default",
                fontSize: 16
            },
            color: "#EEE"
        }
    })
);
```

``` prism-code
const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    surface: { theme: { type: EThemeProviderType.Dark } },
    series: [
        {
            type: ESeriesType.LineSeries,
            xyData: {
                xValues: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
                yValues: [4.3, 5.3, 6, 6.3, 6, 5.2, 4.5, 4.6, 5, 6, 7, 8]
            },
            options: {
                stroke: "#0066FF",
                strokeThickness: 5,
                pointMarker,
                // ...

                // Configure datalabel formatting using similar
                // numericFormat and precision options to the axis label formatting
                dataLabels: {
                    numericFormat: ENumericFormat.Decimal,
                    precision: 4,
                    style: {
                        fontFamily: "Default",
                        fontSize: 16
                    },
                    color: "#EEE"
                }
            }
        }
    ]
});
```

The precision is now increased to 4 decimal places

Data Labels formatting uses similar code to theÂ [LabelProvider](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-labels/label-provider-api-overview) for axis labels. This means that labels can be formatted as dates, exponents or scientific notation.

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/data-point-labels/data-labels-api-overview/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/data-point-labels/data-labels-api-overview/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
