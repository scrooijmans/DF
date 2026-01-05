On this page

# Creating a Polar Chart

SciChart provides a powerful API for creating various types of charts, including **Polar Charts**.

The <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#chartbuilder.buildchart" rel="noopener noreferrer" target="_blank">buildChartðŸ“˜</a> function can be used to build both 2D Charts, Pie Charts, **2D Polar Charts** & 3D Charts, so the returned object type will differ depending on the chart type.

## Using <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#chartbuilder.buildchart" rel="noopener noreferrer" target="_blank">buildChartðŸ“˜</a> to create a Polar Chart<a href="https://www.scichart.com/documentation/js/v4/2d-charts/builder-api/polar-chart/#using-buildchartblue_book-to-create-a-polar-chart" class="hash-link" aria-label="Direct link to using-buildchartblue_book-to-create-a-polar-chart" translate="no" title="Direct link to using-buildchartblue_book-to-create-a-polar-chart">â€‹</a>

- TS

``` prism-code
const {
    chartBuilder,
    ESciChartSurfaceType,
    EThemeProviderType,
    EAxisType,
    EPolarAxisMode,
    EChart2DModifierType,
    ESeriesType,
} = SciChart;
// or, for npm, import { SciChartSurface, ... } from "scichart"

const { sciChartSurface, wasmContext } = await chartBuilder.buildChart(divElementId, {
    type: ESciChartSurfaceType.Polar2D,
    options: {
        surface: {
            theme: { type: EThemeProviderType.Navy } 
        },
        xAxes: [{
            type: EAxisType.PolarNumericAxis, 
            options: {
                polarAxisMode: EPolarAxisMode.Angular,
            }
        }],
        yAxes: [{
            type: EAxisType.PolarNumericAxis, 
            options: {
                polarAxisMode: EPolarAxisMode.Radial,
                labelPrecision: 0,
            }
        }],
        series: [
            {
                type: ESeriesType.PolarColumnSeries,
                xyData: {
                    xValues: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
                    yValues: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
                },
                options: {
                    stroke: "rgba(255, 0, 0, 1)",
                    fill: "rgba(255, 0, 0, 0.3)",
                    dataPointWidth: 0.8
                }
            }
        ],
        modifiers: [
            { type: EChart2DModifierType.PolarMouseWheelZoom },
            { type: EChart2DModifierType.PolarZoomExtents },
            { type: EChart2DModifierType.PolarCursor }
        ]
    }
});
```

## Using <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#build2dpolarchart" rel="noopener noreferrer" target="_blank">build2DPolarChartðŸ“˜</a> to explicitly create a Polar Chart:<a href="https://www.scichart.com/documentation/js/v4/2d-charts/builder-api/polar-chart/#using-build2dpolarchartblue_book-to-explicitly-create-a-polar-chart" class="hash-link" aria-label="Direct link to using-build2dpolarchartblue_book-to-explicitly-create-a-polar-chart" translate="no" title="Direct link to using-build2dpolarchartblue_book-to-explicitly-create-a-polar-chart">â€‹</a>

- TS

``` prism-code
const {
    build2DPolarChart,
    EThemeProviderType,
    EAxisType,
    EPolarAxisMode,
    EChart2DModifierType,
    ESeriesType,
} = SciChart;
// or, for npm, import { SciChartSurface, ... } from "scichart"

const { sciChartSurface, wasmContext } = await build2DPolarChart(divElementId, {
    surface: {
        theme: { type: EThemeProviderType.Navy } 
    },
    xAxes: [{
        type: EAxisType.PolarNumericAxis, 
        options: {
            polarAxisMode: EPolarAxisMode.Angular,
        }
    }],
    yAxes: [{
        type: EAxisType.PolarNumericAxis, 
        options: {
            polarAxisMode: EPolarAxisMode.Radial,
            labelPrecision: 0,
        }
    }],
    series: [
        {
            type: ESeriesType.PolarColumnSeries,
            xyData: {
                xValues: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
                yValues: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
            },
            options: {
                stroke: "rgba(255, 0, 0, 1)",
                fill: "rgba(255, 0, 0, 0.3)",
                dataPointWidth: 0.8
            }
        }
    ],
    modifiers: [
        { type: EChart2DModifierType.PolarMouseWheelZoom },
        { type: EChart2DModifierType.PolarZoomExtents },
        { type: EChart2DModifierType.PolarCursor }
    ]
});
```

Both of these methods will result in this output:

![](out_scichartv4/2d-charts/builder-api/polar-chart/index_media/5a72c3ee6a9cc14296f901057d6eabf2b9b712b4.svg)note

The options that the polar chart builder accepts are the same as the 2D surface, and can be seen here <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iscichart2ddefinition.html" rel="noopener noreferrer" target="_blank">ISciChart2DDefinitionðŸ“˜</a>, but you must choose options with `Polar` in their name.

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/builder-api/polar-chart/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [Intro to the Builder API](https://www.scichart.com/documentation/js/v4/2d-charts/builder-api/builder-api-overview)
- [Working with Data](https://www.scichart.com/documentation/js/v4/2d-charts/builder-api/working-with-data)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/builder-api/polar-chart/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/builder-api/polar-chart/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
