On this page

# Drawing PointMarkers on Series (Scatter markers)

Every data point of a Scatter, Line, Bubble, Mountain, Spline, Error or Column SeriesÂ may be marked with aÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries.html#pointmarker" rel="noopener noreferrer" target="_blank">PointMarkerðŸ“˜</a>. So, not just limited to scatter series, you can apply a pointmarker to line series, or error bars to display a repeated marker at the X,Y point.

![](out_scichartv4/2d-charts/chart-types/common-series-apis/drawing-point-markers/index_media/af50301a53fa555616a9b2279e7e25a1d566cb1a.svg)info

Simply setÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries.html#pointmarker" rel="noopener noreferrer" target="_blank">BaseRenderableSeries.pointMarkerðŸ“˜</a> = newÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basepointmarker.html" rel="noopener noreferrer" target="_blank">EllipsePointMarker()ðŸ“˜</a> to apply a scatter point to most series types.

Several different types of PointMarker are available in SciChart.js:

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/ellipsepointmarker.html" rel="noopener noreferrer" target="_blank">EllipsePointMarkerðŸ“˜</a> - Renders a circle at each point
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/squarepointmarker.html" rel="noopener noreferrer" target="_blank">SquarePointMarkerðŸ“˜</a> - Renders a square at each point
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/trianglepointmarker.html" rel="noopener noreferrer" target="_blank">TrianglePointMarkerðŸ“˜</a> - Renders a triangle at each point
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/crosspointmarker.html" rel="noopener noreferrer" target="_blank">CrossPointMarkerðŸ“˜</a> - Renders a plus sign '+' at each point
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xpointmarker.html" rel="noopener noreferrer" target="_blank">XPointMarkerðŸ“˜</a> - Renders an 'x' at each point
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/spritepointmarker.html" rel="noopener noreferrer" target="_blank">SpritePointMarkerðŸ“˜</a> - Allows an image to be used at each point to create custom pointmarkers

Below we're going to show some options how to use different types of PointMarker in SciChart.

- TS
- Builder API (JSON Config)

``` prism-code
// Create a chart with X,Y axis
const { sciChartSurface, wasmContext } = await SciChartSurface.create(divElementId, {
    theme: new SciChartJsNavyTheme()
});
sciChartSurface.xAxes.add(new NumericAxis(wasmContext));
sciChartSurface.yAxes.add(new NumericAxis(wasmContext, { growBy: new NumberRange(0.0, 0.4) }));

// Create some data
const { xValues, yValues1, yValues2, yValues3, yValues4, yValues5 } = createData();

const commonOptions = { width: 11, height: 11, strokeThickness: 2 };

// Add a line series with EllipsePointMarker
sciChartSurface.renderableSeries.add(
    new XyScatterRenderableSeries(wasmContext, {
        pointMarker: new EllipsePointMarker(wasmContext, {
            ...commonOptions,
            fill: "#0077FF99",
            stroke: "LightSteelBlue"
        }),
        dataSeries: new XyDataSeries(wasmContext, { xValues, yValues: yValues1 })
    })
);

// Add a scatter series with SquarePointMarker
sciChartSurface.renderableSeries.add(
    new XyScatterRenderableSeries(wasmContext, {
        pointMarker: new SquarePointMarker(wasmContext, {
            ...commonOptions,
            fill: "#FF000099",
            stroke: "Red"
        }),
        dataSeries: new XyDataSeries(wasmContext, { xValues, yValues: yValues2 })
    })
);

// Add a scatter series with TrianglePointMarker
sciChartSurface.renderableSeries.add(
    new XyScatterRenderableSeries(wasmContext, {
        pointMarker: new TrianglePointMarker(wasmContext, {
            ...commonOptions,
            fill: "#FFDD00",
            stroke: "#FF6600"
        }),
        dataSeries: new XyDataSeries(wasmContext, { xValues, yValues: yValues3 })
    })
);

// Add a scatter series with CrossPointMarker
sciChartSurface.renderableSeries.add(
    new XyScatterRenderableSeries(wasmContext, {
        pointMarker: new CrossPointMarker(wasmContext, {
            ...commonOptions,
            stroke: "#FF00FF"
        }),
        dataSeries: new XyDataSeries(wasmContext, { xValues, yValues: yValues4 })
    })
);

// Add a scatter series with Custom Image using SpritePointMarker
const imageBitmap = await createImageAsync("https://www.scichart.com/demo/images/CustomMarkerImage.png");

sciChartSurface.renderableSeries.add(
    new XyScatterRenderableSeries(wasmContext, {
        pointMarker: new SpritePointMarker(wasmContext, {
            image: imageBitmap
        }),
        dataSeries: new XyDataSeries(wasmContext, { xValues, yValues: yValues5 })
    })
);
```

``` prism-code
const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    surface: { theme: { type: EThemeProviderType.Dark } },
    xAxes: { type: EAxisType.NumericAxis },
    yAxes: { type: EAxisType.NumericAxis, options: { growBy: new NumberRange(0, 0.15) } },
    series: [
        {
            type: ESeriesType.ScatterSeries,
            xyData: {
                xValues,
                yValues: yValues1
            },
            options: {
                pointMarker: {
                    type: EPointMarkerType.Ellipse,
                    options: {
                        width: 11,
                        height: 11,
                        strokeThickness: 2,
                        fill: "#0077FF99",
                        stroke: "LightSteelBlue"
                    }
                }
            }
        },
        {
            type: ESeriesType.ScatterSeries,
            xyData: {
                xValues,
                yValues: yValues2
            },
            options: {
                pointMarker: {
                    type: EPointMarkerType.Square,
                    options: {
                        width: 11,
                        height: 11,
                        strokeThickness: 2,
                        fill: "#FF000099",
                        stroke: "Red"
                    }
                }
            }
        },
        {
            type: ESeriesType.ScatterSeries,
            xyData: {
                xValues,
                yValues: yValues3
            },
            options: {
                pointMarker: {
                    type: EPointMarkerType.Triangle,
                    options: {
                        width: 11,
                        height: 11,
                        strokeThickness: 2,
                        fill: "#FFDD00",
                        stroke: "#FF6600"
                    }
                }
            }
        },
        {
            type: ESeriesType.ScatterSeries,
            xyData: {
                xValues,
                yValues: yValues4
            },
            options: {
                pointMarker: {
                    type: EPointMarkerType.Cross,
                    options: {
                        width: 11,
                        height: 11,
                        strokeThickness: 2,
                        stroke: "#FF00FF"
                    }
                }
            }
        },
        {
            type: ESeriesType.ScatterSeries,
            xyData: {
                xValues,
                yValues: yValues5
            },
            options: {
                pointMarker: {
                    type: EPointMarkerType.Sprite,
                    options: {
                        image: await createImageAsync("https://www.scichart.com/demo/images/CustomMarkerImage.png")
                    }
                }
            }
        }
    ]
});
```

This results in the following output:

## IsLastPointOnly mode for Pointmarkers<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/common-series-apis/drawing-point-markers/#islastpointonly-mode-for-pointmarkers" class="hash-link" aria-label="Direct link to IsLastPointOnly mode for Pointmarkers" translate="no" title="Direct link to IsLastPointOnly mode for Pointmarkers">â€‹</a>

The PointMarker type has a propertyÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basepointmarker.html#lastpointonly" rel="noopener noreferrer" target="_blank">isLastPointOnlyðŸ“˜</a>. When true, only the last point of a scatter series is drawn. This can be useful to highlight a point in say a sweeping ECG chart.

## Additional Tips for PointMarkers<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/common-series-apis/drawing-point-markers/#additional-tips-for-pointmarkers" class="hash-link" aria-label="Direct link to Additional Tips for PointMarkers" translate="no" title="Direct link to Additional Tips for PointMarkers">â€‹</a>

Custom markers can be created using theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/spritepointmarker.html" rel="noopener noreferrer" target="_blank">SpritePointMarkerðŸ“˜</a>Â type, which allows loading a custom image as a marker. This uses the helper functionÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#createimageasync" rel="noopener noreferrer" target="_blank">createImageAsyncðŸ“˜</a> which allows loading of a PNG file either from URL, or locally hosted / imported image.

![](out_scichartv4/2d-charts/chart-types/common-series-apis/drawing-point-markers/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

For a TypeScript / npm & webpack example you can see theÂ <a href="https://www.scichart.com/demo/javascript-chart-custom-pointmarkers" rel="noopener noreferrer" target="_blank">JavaScript Custom PointMarkers Chart</a> example in the SciChart.js demo.

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/common-series-apis/drawing-point-markers/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/common-series-apis/drawing-point-markers/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
