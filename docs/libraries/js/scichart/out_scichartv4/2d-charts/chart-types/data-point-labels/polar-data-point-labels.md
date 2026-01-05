On this page

# Polar Data Point Labels

![](out_scichartv4/2d-charts/chart-types/data-point-labels/polar-data-point-labels/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

Before reading this article, we recommend you read the [DataLabels API Overview](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-point-labels/data-labels-api-overview) to understand the basics of how DataLabels work.

Polar Data Labels allow per data-point text labels to be drawn on series, or arbitrary text labels at `x`, `y` positions on the chart, which for polar charts are conceptually "radius" and "angle".

There are no real differences between polar and cartesian data labels.

## Adding Data Labels<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-point-labels/polar-data-point-labels/#adding-data-labels" class="hash-link" aria-label="Direct link to Adding Data Labels" translate="no" title="Direct link to Adding Data Labels">â€‹</a>

You can configure data labels for almost any series by setting a valid style on theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibaselinerenderableseriesoptions.html#datalabels" rel="noopener noreferrer" target="_blank">dataLabels propertyðŸ“˜</a> in the series options:

- TS
- Builder API (JSON Config)

``` prism-code
// Demonstrates how to add DataLabels to a Polar chart with SciChart.js
const {
    SciChartPolarSurface,
    PolarNumericAxis,
    EPolarAxisMode,
    EHorizontalTextPosition,
    EVerticalTextPosition,
    PolarMountainRenderableSeries,
    EDataLabelSkipMode,
    SciChartJsNavyTheme,
    XyDataSeries,
    NumberRange,
    Thickness
} = SciChart;
// or, for npm, import { SciChartSurface, ... } from "scichart"

const { sciChartSurface, wasmContext } = await SciChartPolarSurface.create(divElementId, {
    theme: new SciChartJsNavyTheme()
});

// Add the yAxis
const radialYAxis = new PolarNumericAxis(wasmContext, {
    polarAxisMode: EPolarAxisMode.Radial,
    visibleRange: new NumberRange(0, 5),
    drawLabels: false,
});
sciChartSurface.yAxes.add(radialYAxis);

// Add the xAxis
const angularXAxis = new PolarNumericAxis(wasmContext, {
    polarAxisMode: EPolarAxisMode.Angular,
});
sciChartSurface.xAxes.add(angularXAxis);

// Add mountain:
const mountain = new PolarMountainRenderableSeries(wasmContext, {
    dataSeries: new XyDataSeries(wasmContext, {
        xValues: Array.from({ length: 9 }, (_, i) => i),
        yValues: [ 4, 4.2, 3.5, 1.6, 3.8, 4, 1.8, 4.7, 4 ]
    }),
    stroke: "#CC7700",
    fill: "#CC770044",
    strokeThickness: 4,
    dataLabels: {
        color: "#FFFFFF",
        style: {
            fontSize: 16,
            padding: new Thickness(0, 0, 20, 0), // raise the labels by adding a bit of bottom padding
        },
        precision: 2, // 2 decimal places
        horizontalTextPosition: EHorizontalTextPosition.Center,
        verticalTextPosition: EVerticalTextPosition.Above,
        skipMode: EDataLabelSkipMode.ShowAll
    },
})
sciChartSurface.renderableSeries.add(mountain);
```

``` prism-code
// Demonstrates how to add DataLabels to a Polar chart with SciChart.js using the Builder API
const { 
    chartBuilder, 
    ESeriesType, 
    EThemeProviderType, 
    EHorizontalTextPosition,
    EVerticalTextPosition,
    EDataLabelSkipMode 
} = SciChart;
// or, for npm, import { chartBuilder, ... } from "scichart"

const { wasmContext, sciChartSurface } = await chartBuilder.build2DPolarChart(divElementId, {
    surface: { theme: { type: EThemeProviderType.Dark } },
    series: [
        {
            type: ESeriesType.PolarMountainSeries,
            xyData: {
                xValues: Array.from({ length: 9 }, (_, i) => i),
                yValues: [4, 4.2, 3.5, 1.6, 3.8, 4, 1.8, 4.7, 4]
            },
            options: {
                dataLabels: {
                    color: "#EEE",
                    style: {
                        fontSize: 16,
                        padding: { top: 0, right: 0, bottom: 20, left: 0 } // raise the labels by adding a bit of bottom padding
                    },
                    precision: 2, // 2 decimal places
                    horizontalTextPosition: EHorizontalTextPosition.Center,
                    verticalTextPosition: EVerticalTextPosition.Above,
                    skipMode: EDataLabelSkipMode.ShowAll
                },
                stroke: "#CC7700",
                fill: "#CC770044",
                strokeThickness: 4,
            }
        }
    ]
});
```

This results in the following output:

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-point-labels/polar-data-point-labels/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [DataLabels API Overview](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-point-labels/data-labels-api-overview)
- <a href="https://www.scichart.com/demo/iframe/polar-line-chart" rel="noopener noreferrer" target="_blank">Polar Data Labels Example</a>

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/data-point-labels/polar-data-point-labels/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/data-point-labels/polar-data-point-labels/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
