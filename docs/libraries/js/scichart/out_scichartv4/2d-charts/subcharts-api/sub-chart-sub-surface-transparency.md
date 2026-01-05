On this page

# SubChart Sub Surface Transparency

The SubCharts API allows you to place nested charts (charts within charts). In the previous exampleÂ [What is the SubCharts API](https://www.scichart.com/documentation/js/v4/2d-charts/subcharts-api/subcharts-api-overview) we covered how to add SubCharts to a SciChartSurface, and also how to position them inÂ [SubCharts Positioning](https://www.scichart.com/documentation/js/v4/2d-charts/subcharts-api/sub-charts-positioning).

In this example we're going to show the difference betweenÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsubsurface.html#istransparent" rel="noopener noreferrer" target="_blank">SciChartSubSurface.isTransparentðŸ“˜</a> true and false.

- TS
- Builder API (JSON Config)

``` prism-code
// Demonstrates the isTransparent property in SubCharts
const {
    SciChartSurface,
    SciChartSubSurface,
    NumericAxis,
    SciChartJsNavyTheme,
    Rect,
    ECoordinateMode,
    FastLineRenderableSeries,
    XyDataSeries,
    MouseWheelZoomModifier,
    ZoomPanModifier
} = SciChart;

// or, for npm, import { SciChartSurface, ... } from "scichart"

// Create a parent (regular) SciChartSurface which will contain the sub-chart
const { wasmContext, sciChartSurface } = await SciChartSurface.create(divElementId, {
    theme: new SciChartJsNavyTheme(),
    title: "Parent Chart",
    titleStyle: { fontSize: 22, color: "#eeeeee" }
});

// Create X,Y axis on the parent chart and programmatically zoom into part of the data
sciChartSurface.xAxes.add(new NumericAxis(wasmContext));
sciChartSurface.yAxes.add(new NumericAxis(wasmContext));

// Add some random data to the parent chart so it can be shown through the subchart
sciChartSurface.renderableSeries.add(
    new FastLineRenderableSeries(wasmContext, {
        strokeThickness: 3,
        stroke: "teal",
        dataSeries: new XyDataSeries(wasmContext, { xValues, yValues })
    })
);

sciChartSurface.chartModifiers.add(new MouseWheelZoomModifier());
sciChartSurface.chartModifiers.add(new ZoomPanModifier());

// Add a Sub-Charts to the main surface with isTransparent=true
const subChart1 = SciChartSubSurface.createSubSurface(sciChartSurface, {
    position: new Rect(0.02, 0.05, 0.4, 0.4),
    isTransparent: true,
    canvasBorder: { border: 3, color: "#77777777" },
    title: "SubChart with isTransparent True",
    titleStyle: { fontSize: 16, color: "#eeeeee99" }
});

// Add x,y axis to the subchart
subChart1.xAxes.add(new NumericAxis(wasmContext));
subChart1.yAxes.add(new NumericAxis(wasmContext));

// Add some random data to the sub chart so it can be shown through the subchart
subChart1.renderableSeries.add(
    new FastLineRenderableSeries(wasmContext, {
        strokeThickness: 5,
        stroke: "Orange",
        dataSeries: new XyDataSeries(wasmContext, { xValues, yValues: yValues1 })
    })
);

// Add a Sub-Charts to the main surface with isTransparent=false
const subChart2 = SciChartSubSurface.createSubSurface(sciChartSurface, {
    position: new Rect(0.02, 0.5, 0.4, 0.4),
    isTransparent: false,
    canvasBorder: { border: 3, color: "#77777777" },
    title: "SubChart with isTransparent false",
    titleStyle: { fontSize: 16, color: "#eeeeee99" }
});

// Add x,y axis to the subchart
subChart2.xAxes.add(new NumericAxis(wasmContext));
subChart2.yAxes.add(new NumericAxis(wasmContext));

// Add some random data to the sub chart so it can be shown through the subchart
subChart2.renderableSeries.add(
    new FastLineRenderableSeries(wasmContext, {
        strokeThickness: 5,
        stroke: "Orange",
        dataSeries: new XyDataSeries(wasmContext, { xValues, yValues: yValues1 })
    })
);
```

``` prism-code
// Demonstrates how to create a line chart with SciChart.js using the Builder API
const {
    chartBuilder,
    ESeriesType,
    EAxisType,
    EThemeProviderType,
    Rect,
    ECoordinateMode,
    EAnnotationType,
    NumberRange,
    EChart2DModifierType
} = SciChart;

// or, for npm, import { chartBuilder, ... } from "scichart"

const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    surface: { theme: { type: EThemeProviderType.Dark } },
    // Main chart definition is here
    xAxes: { type: EAxisType.NumericAxis },
    yAxes: { type: EAxisType.NumericAxis },
    series: [
        {
            type: ESeriesType.LineSeries,
            xyData: {
                xValues,
                yValues
            },
            options: {
                stroke: "#0066FF",
                strokeThickness: 5
            }
        }
    ],
    modifiers: [
        { type: EChart2DModifierType.ZoomPan },
        { type: EChart2DModifierType.ZoomExtents },
        { type: EChart2DModifierType.MouseWheelZoom }
    ],
    // Subchart definition is here
    subCharts: [
        {
            surface: {
                position: new Rect(0.02, 0.05, 0.4, 0.4),
                isTransparent: true,
                canvasBorder: { border: 3, color: "#77777777" },
                title: "SubChart with isTransparent True",
                titleStyle: { fontSize: 16, color: "#eeeeee99" }
            },
            // Define the x,y axis on Subchart
            xAxes: { type: EAxisType.NumericAxis },
            yAxes: { type: EAxisType.NumericAxis },
            // Define the series on Subchart
            series: [
                {
                    type: ESeriesType.LineSeries,
                    xyData: {
                        xValues,
                        yValues: yValues1
                    },
                    options: {
                        stroke: "#0066FF",
                        strokeThickness: 5
                    }
                }
            ]
        },
        {
            surface: {
                position: new Rect(0.02, 0.5, 0.4, 0.4),
                isTransparent: false,
                canvasBorder: { border: 3, color: "#77777777" },
                title: "SubChart with isTransparent false",
                titleStyle: { fontSize: 16, color: "#eeeeee99" }
            },
            // Define the x,y axis on Subchart
            xAxes: { type: EAxisType.NumericAxis },
            yAxes: { type: EAxisType.NumericAxis },
            // Define the series on Subchart
            series: [
                {
                    type: ESeriesType.LineSeries,
                    xyData: {
                        xValues,
                        yValues: yValues1
                    },
                    options: {
                        stroke: "#0066FF",
                        strokeThickness: 5
                    }
                }
            ]
        }
    ]
});
```

This results in the following output:

The example shows that the Line Series on the parent chart will be visible underneath the sub-surface when `isTransparent = true`. This behavior could be changed usingÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/i2dsubsurfaceoptions.html#istransparent" rel="noopener noreferrer" target="_blank">I2DSubSurfaceOptions.isTransparentðŸ“˜</a> option passed toÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsubsurface.html#createsubsurface" rel="noopener noreferrer" target="_blank">SciChartSubSurface.createSubSurface()ðŸ“˜</a> or via theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsubsurface.html#istransparent" rel="noopener noreferrer" target="_blank">SciChartSubSurface.isTransparentðŸ“˜</a> property.

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/subcharts-api/sub-chart-sub-surface-transparency/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [SubCharts Positioning](https://www.scichart.com/documentation/js/v4/2d-charts/subcharts-api/sub-charts-positioning)
- [SubCharts Html Container](https://www.scichart.com/documentation/js/v4/2d-charts/subcharts-api/sub-charts-html-container)
- [Worked Example: Using SubCharts to create a Large Dashboard](https://www.scichart.com/documentation/js/v4/2d-charts/subcharts-api/example-using-sub-charts-to-create-large-dashboard)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/subcharts-api/sub-chart-sub-surface-transparency/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/subcharts-api/sub-chart-sub-surface-transparency/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
