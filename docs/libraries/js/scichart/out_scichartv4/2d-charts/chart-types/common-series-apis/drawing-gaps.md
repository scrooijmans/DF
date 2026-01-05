On this page

# Drawing Gaps in Series (null data)

It is possible to have null points or gaps in multiple series types by passing a data point with Y = NaN (Not a Number). The following chart (series) types support NaN gaps.

- Line Series - with gaps or closed-lines where NaN is found
- Scatter Series
- Mountain Series - with gaps or closed-lines
- Column Series
- Candlestick/Ohlc Series
- Bubble Series

## Drawing Null points with NaN<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/common-series-apis/drawing-gaps/#drawing-null-points-with-nan" class="hash-link" aria-label="Direct link to Drawing Null points with NaN" translate="no" title="Direct link to Drawing Null points with NaN">â€‹</a>

The following example shows you how to draw null points by setting Y=NaN.

Setting Y=NaN is considered a 'null point' in SciChart.js and will allow you to hide areas of the data.

![](out_scichartv4/2d-charts/chart-types/common-series-apis/drawing-gaps/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

Note Line-based series such as Line, Mountain, Band and Spline may also be drawn as a closed-line when Y=NaN by settingÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries.html#drawnanas" rel="noopener noreferrer" target="_blank">BaseRenderableSeries.drawNaNAsðŸ“˜</a> =Â <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/elinedrawmode.html" rel="noopener noreferrer" target="_blank">ELineDrawMode.PolyLineðŸ“˜</a>. This property will be ignored for certain series types like Scatter.

Here's an example below:

- TS
- Builder API (JSON Config)

``` prism-code
// Create some data with Y=NAN gaps
let yLast = 100.0;
const xValues = [];
const yValues = [];
for (let i = 0; i <= 250; i++) {
    const y = yLast + (random() - 0.48);
    yLast = y;
    xValues.push(i);
    yValues.push(i % 50 < 15 ? NaN : y);
}

// Create a mountain series
const mountainSeries = new FastMountainRenderableSeries(wasmContext, {
    dataSeries: new XyDataSeries(wasmContext, { xValues, yValues }),
    stroke: "#4682b4",
    strokeThickness: 3,
    zeroLineY: 0.0,
    // when a solid color is required, use fill
    fill: "rgba(176, 196, 222, 0.7)",
    // when a gradient is required, use fillLinearGradient
    fillLinearGradient: new GradientParams(new Point(0, 0), new Point(0, 1), [
        { color: "rgba(70,130,180,0.77)", offset: 0 },
        { color: "rgba(70,130,180,0.0)", offset: 1 }
    ])
});
```

``` prism-code
// Create some data with Y=NAN gaps
let yLast = 100.0;
const xValues = [];
const yValues = [];
for (let i = 0; i <= 250; i++) {
    const y = yLast + (random() - 0.48);
    yLast = y;
    xValues.push(i);
    yValues.push(i % 50 < 15 ? NaN : y);
}

const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    surface: { theme: { type: EThemeProviderType.Dark } },
    series: [
        {
            type: ESeriesType.MountainSeries,
            xyData: {
                xValues,
                yValues
            },
            options: {
                stroke: "#4682b4",
                strokeThickness: 3,
                zeroLineY: 0.0,
                drawNaNAs: ELineDrawMode.DiscontinuousLine,
                fill: "rgba(176, 196, 222, 0.7)", // when a solid color is required, use fill
                fillLinearGradient: {
                    gradientStops: [
                        { color: "rgba(70,130,180,0.77)", offset: 0.0 },
                        { color: "rgba(70,130,180,0.0)", offset: 1 }
                    ],
                    startPoint: { x: 0, y: 0 },
                    endPoint: { x: 0, y: 1 }
                }
            }
        }
    ],
    annotations: [
        {
            type: EAnnotationType.SVGTextAnnotation,
            options: {
                x1: 75,
                y1: 104.1,
                text: "Gaps occur where Y = NaN",
                textColor: "LightSteelBlue",
                fontSize: 16,
                horizontalAnchorPoint: EHorizontalAnchorPoint.Right,
                verticalAnchorPoint: EVerticalAnchorPoint.Bottom
            }
        },
        {
            type: EAnnotationType.RenderContextLineAnnotation,
            options: { x1: 70, x2: 105, y1: 104, y2: 102, stroke: "LightSteelBlue", strokeThickness: 2 }
        }
    ]
});
```

## Closing Lines with NaN<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/common-series-apis/drawing-gaps/#closing-lines-with-nan" class="hash-link" aria-label="Direct link to Closing Lines with NaN" translate="no" title="Direct link to Closing Lines with NaN">â€‹</a>

Lines may also be closed when Y=NaN by settingÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries.html#drawnanas" rel="noopener noreferrer" target="_blank">BaseRenderableSeries.drawNaNAsðŸ“˜</a> =Â <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/elinedrawmode.html" rel="noopener noreferrer" target="_blank">ELineDrawMode.PolyLineðŸ“˜</a>. This property will be ignored for certain series types like Scatter.

## Having Different Styles in the Same Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/common-series-apis/drawing-gaps/#having-different-styles-in-the-same-series" class="hash-link" aria-label="Direct link to Having Different Styles in the Same Series" translate="no" title="Direct link to Having Different Styles in the Same Series">â€‹</a>

Sometimes it is asked 'can we have a different line-color, or dashed line style when Y=NaN'.

Not using the above API, but a workaround can be achieved by simply using two series, e.g. with the following data.

- TS
- Builder API (JSON Config)

``` prism-code
// Create some data with Y=NAN gaps
let yLast = 100.0;
const xValues = [];
const yValues = [];
const yValuesInv = [];
for (let i = 0; i <= 250; i++) {
    const y = yLast + (random() - 0.48);
    yLast = y;
    xValues.push(i);
    const isThisDataNull = i % 50 < 15;
    yValues.push(isThisDataNull ? NaN : y);
    yValuesInv.push(isThisDataNull ? y : NaN);
}

// Create a mountain series
const mountainSeries = new FastMountainRenderableSeries(wasmContext, {
    dataSeries: new XyDataSeries(wasmContext, { xValues, yValues }),
    stroke: "#4682b4",
    strokeThickness: 3,
    zeroLineY: 0.0,
    // when a solid color is required, use fill
    fill: "rgba(176, 196, 222, 0.7)",
    // when a gradient is required, use fillLinearGradient
    fillLinearGradient: new GradientParams(new Point(0, 0), new Point(0, 1), [
        { color: "rgba(70,130,180,0.77)", offset: 0 },
        { color: "rgba(70,130,180,0.0)", offset: 1 }
    ])
});

// Create the 'alternative style' mountain series which renders yValuesInv
const altStyleMountainSeries = new FastMountainRenderableSeries(wasmContext, {
    dataSeries: new XyDataSeries(wasmContext, { xValues, yValues: yValuesInv }),
    stroke: "#F4842077",
    strokeDashArray: [2, 2],
    strokeThickness: 3,
    zeroLineY: 0.0,
    // when a gradient is required, use fillLinearGradient
    fillLinearGradient: new GradientParams(new Point(0, 0), new Point(0, 1), [
        { color: "#F4842033", offset: 0 },
        { color: "#F4842000", offset: 1 }
    ])
});
```

``` prism-code
// Create some data with Y=NAN gaps
let yLast = 100.0;
const xValues = [];
const yValues = [];
for (let i = 0; i <= 250; i++) {
    const y = yLast + (random() - 0.48);
    yLast = y;
    xValues.push(i);
    yValues.push(i % 50 < 15 ? NaN : y);
}

const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    surface: { theme: { type: EThemeProviderType.Dark } },
    series: [
        {
            type: ESeriesType.MountainSeries,
            xyData: {
                xValues,
                yValues
            },
            options: {
                stroke: "#4682b4",
                strokeThickness: 3,
                zeroLineY: 0.0,
                drawNaNAs: ELineDrawMode.DiscontinuousLine,
                fill: "rgba(176, 196, 222, 0.7)", // when a solid color is required, use fill
                fillLinearGradient: {
                    gradientStops: [
                        { color: "rgba(70,130,180,0.77)", offset: 0.0 },
                        { color: "rgba(70,130,180,0.0)", offset: 1 }
                    ],
                    startPoint: { x: 0, y: 0 },
                    endPoint: { x: 0, y: 1 }
                }
            }
        }
    ],
    annotations: [
        {
            type: EAnnotationType.SVGTextAnnotation,
            options: {
                x1: 75,
                y1: 104.1,
                text: "Gaps occur where Y = NaN",
                textColor: "LightSteelBlue",
                fontSize: 16,
                horizontalAnchorPoint: EHorizontalAnchorPoint.Right,
                verticalAnchorPoint: EVerticalAnchorPoint.Bottom
            }
        },
        {
            type: EAnnotationType.RenderContextLineAnnotation,
            options: { x1: 70, x2: 105, y1: 104, y2: 102, stroke: "LightSteelBlue", strokeThickness: 2 }
        }
    ]
});
```

This approach results in some nice visualisations to represent null data (or inaccurate data) like this.

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/common-series-apis/drawing-gaps/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/common-series-apis/drawing-gaps/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
