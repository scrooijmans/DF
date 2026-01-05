On this page

# Creating a 3D Chart

SciChart provides a powerful API for creating various types of charts, including **3D Charts**.

The <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#chartbuilder.buildchart" rel="noopener noreferrer" target="_blank">buildChartðŸ“˜</a> function can be used to build both 2D Charts, Pie Charts, 2D Polar Charts & **3D Charts**, so the returned object type will differ depending on the chart type.

## Using <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#chartbuilder.buildchart" rel="noopener noreferrer" target="_blank">buildChartðŸ“˜</a> to create a 3D Chart<a href="https://www.scichart.com/documentation/js/v4/3d-charts/builder-api/default-3d-chart/#using-buildchartblue_book-to-create-a-3d-chart" class="hash-link" aria-label="Direct link to using-buildchartblue_book-to-create-a-3d-chart" translate="no" title="Direct link to using-buildchartblue_book-to-create-a-3d-chart">â€‹</a>

- TS

``` prism-code
// Demonstrates how to create a basic 3D chart using the Builder API in SciChart.js
const {
    chartBuilder,
    EThemeProviderType,
    ESciChartSurfaceType,
    EAxisType,
    ESeriesType3D,
    EChart3DModifierType,
    EPointMarker3DType,
    Vector3
} = SciChart;
// or, for npm, import { SciChartSurface, ... } from "scichart"

const { sciChart3DSurface, wasmContext } = await chartBuilder.buildChart(divElementId, {
    type: ESciChartSurfaceType.Default3D,
    options: {
        surface: {
            theme: { type: EThemeProviderType.Navy },
            cameraOptions: {
                position: new Vector3(300, 300, 300),
            }
        },
        zAxis: {
            type: EAxisType.NumericAxis3D,
            options: {
                axisTitle: "Z Axis",
                labelPrecision: 0
            }
        },
        series: [
            {
                type: ESeriesType3D.ScatterRenderableSeries3D,
                xyzData: {
                    xValues: Array.from({ length: 10 }, (_, i) => Math.round(Math.random() * 10)),
                    yValues: Array.from({ length: 10 }, (_, i) => Math.round(Math.random() * 10)),
                    zValues: Array.from({ length: 10 }, (_, i) => Math.round(Math.random() * 10)),
                },
                options: {
                    pointMarker: {
                        type: EPointMarker3DType.Sphere,
                        options: {
                            size: 20,
                            fill: "#3388FF",
                        }
                    }
                }
            }
        ],
        modifiers: [
            { type: EChart3DModifierType.MouseWheelZoom },
            { type: EChart3DModifierType.Orbit },
            { type: EChart3DModifierType.ZoomExtents }
        ]
    }
});
```

## Using <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#chartbuilder.build3dchart" rel="noopener noreferrer" target="_blank">build3DChartðŸ“˜</a> to explicitly create a 3D Chart.<a href="https://www.scichart.com/documentation/js/v4/3d-charts/builder-api/default-3d-chart/#using-build3dchartblue_book-to-explicitly-create-a-3d-chart" class="hash-link" aria-label="Direct link to using-build3dchartblue_book-to-explicitly-create-a-3d-chart" translate="no" title="Direct link to using-build3dchartblue_book-to-explicitly-create-a-3d-chart">â€‹</a>

- TS

``` prism-code
// Demonstrates how to create a basic 3D chart using the Builder API in SciChart.js
const {
    build3DChart,
    EThemeProviderType,
    EAxisType,
    ESeriesType3D,
    EChart3DModifierType,
    EPointMarker3DType,
    Vector3
} = SciChart;
// or, for npm, import { SciChartSurface, ... } from "scichart"

const { sciChart3DSurface, wasmContext } = await build3DChart(divElementId, {
    // no need to specify `type` here, as build3DChart always returns a 3D surface
    surface: {
        theme: { type: EThemeProviderType.Navy },
        cameraOptions: {
            position: new Vector3(300, 300, 300),
        }
    },
    zAxis: {
        type: EAxisType.NumericAxis3D,
        options: {
            axisTitle: "Z Axis",
            labelPrecision: 0
        }
    },
    series: [
        {
            type: ESeriesType3D.ScatterRenderableSeries3D,
            xyzData: {
                xValues: Array.from({ length: 10 }, (_, i) => Math.round(Math.random() * 10)),
                yValues: Array.from({ length: 10 }, (_, i) => Math.round(Math.random() * 10)),
                zValues: Array.from({ length: 10 }, (_, i) => Math.round(Math.random() * 10)),
            },
            options: {
                pointMarker: {
                    type: EPointMarker3DType.Sphere,
                    options: {
                        size: 20,
                        fill: "#3388FF",
                    }
                }
            }
        }
    ],
    modifiers: [
        { type: EChart3DModifierType.MouseWheelZoom },
        { type: EChart3DModifierType.Orbit },
        { type: EChart3DModifierType.ZoomExtents }
    ]
});
```

Both of these methods will result in this output:

#### See Also<a href="https://www.scichart.com/documentation/js/v4/3d-charts/builder-api/default-3d-chart/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [Intro to the Builder API](https://www.scichart.com/documentation/js/v4/2d-charts/builder-api/builder-api-overview)
- [Working with Data](https://www.scichart.com/documentation/js/v4/2d-charts/builder-api/working-with-data)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/3d-charts/builder-api/default-3d-chart/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/3d-charts/builder-api/default-3d-chart/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
