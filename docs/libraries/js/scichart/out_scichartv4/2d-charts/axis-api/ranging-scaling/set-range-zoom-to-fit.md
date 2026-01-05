On this page

# Axis Ranging - Set Range and Zoom to Fit

## What is VisibleRange?<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/ranging-scaling/set-range-zoom-to-fit/#what-is-visiblerange" class="hash-link" aria-label="Direct link to What is VisibleRange?" translate="no" title="Direct link to What is VisibleRange?">â€‹</a>

**<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#visiblerange" rel="noopener noreferrer" target="_blank">VisibleRangeðŸ“˜</a>** is an actualÂ axis range, measuredÂ in chart units.Â This is a partÂ of a chartÂ that is currently visible in a viewport.

This is a different concept to the Data Range, which is the extents of the data on the chart.

The diagram below explains the concept of the VisibleRange:

<img src="out_scichartv4/2d-charts/axis-api/ranging-scaling/set-range-zoom-to-fit/index_media/79075f9b99fb069ac44a327fc230fd50c6ab3a40.png" class="img_ev3q" decoding="async" loading="lazy" width="624" height="419" alt="Adjusting XAxis and YAxis visible range in a JavaScript Chart" />

## Setting Axis.VisibleRange Programmatically<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/ranging-scaling/set-range-zoom-to-fit/#setting-axisvisiblerange-programmatically" class="hash-link" aria-label="Direct link to Setting Axis.VisibleRange Programmatically" translate="no" title="Direct link to Setting Axis.VisibleRange Programmatically">â€‹</a>

To programmatically range an axis, set theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#visiblerange" rel="noopener noreferrer" target="_blank">AxisCore.visibleRangeðŸ“˜</a> propertyÂ with aÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" rel="noopener noreferrer" target="_blank">NumberRangeðŸ“˜</a> type.

- TS
- Builder API (JSON Config)

``` prism-code
const { SciChartSurface, NumericAxis, SciChartJsNavyTheme, NumberRange } = SciChart;

// or, for npm, import { SciChartSurface, ... } from "scichart"

// Create a chart with X,Y axis
const { wasmContext, sciChartSurface } = await SciChartSurface.create(divElementId, {
    theme: new SciChartJsNavyTheme()
});
sciChartSurface.xAxes.add(new NumericAxis(wasmContext));
sciChartSurface.yAxes.add(new NumericAxis(wasmContext));

// Allow updating visibleRange
document.getElementById("update-chart-button").addEventListener("click", () => {
    const yAxis = sciChartSurface.yAxes.get(0);

    const min = Math.random() * 0.5;
    yAxis.visibleRange = new NumberRange(min, min + 1);
    const range = yAxis.visibleRange;
    const message = `YAxis VisibleRange is ${range.min.toFixed(2)}, ${range.max.toFixed(2)}`;
    document.getElementById("update-range-label").textContent = message;
    console.log(message);
});
```

``` prism-code
const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    surface: { theme: { type: EThemeProviderType.Dark } },
    xAxes: {
        type: EAxisType.NumericAxis,
        options: { axisTitle: "X Axis" }
    },
    yAxes: {
        type: EAxisType.NumericAxis,
        options: {
            axisTitle: "Y Axis",
            visibleRange: new NumberRange(0, 1)
        }
    },
    series: [
        {
            type: ESeriesType.LineSeries,
            xyData: {
                xValues,
                yValues
            },
            options: {
                stroke: "#50C7E0",
                strokeThickness: 3
            }
        }
    ]
});

document.getElementById("update-chart-button").addEventListener("click", () => {
    const yAxis = sciChartSurface.yAxes.get(0);

    console.log(`Setting Axis.VisibleRange = -0.2, 1.2`);
    yAxis.visibleRange = new NumberRange(-0.2, 1.2);
    const range = yAxis.visibleRange;
    console.log(`Axis VisibleRange is ${range.min}, ${range.max}`);
});
```

This results in the following output:Â 

Click the button above to update yAxis.visibleRange. View the codepen in full to see how this works.

Note theÂ [CategoryAxis](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-types/category-axis)Â is treated as a special case. Although it has the same propertyÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/categoryaxis.html#visiblerange" rel="noopener noreferrer" target="_blank">CategoryAxis.visibleRangeðŸ“˜</a> of typeÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" rel="noopener noreferrer" target="_blank">NumberRangeðŸ“˜</a>, it expects values to be indices, not data-values.

The reason for this is thatÂ this axis type works with withÂ **data indexes, not actual data values.** So aÂ **NumberRange** should be applied instead, with lower data index as Min and Upper data index as Max.

To learn more about **how to convert values** from Data-values to Indexes and back in a CategoryAxis, please refer to theÂ [Convert Pixels to Data Coordinates](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/misc/pixel-and-data-coordinates) article.

## Zooming to fitÂ all the Data<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/ranging-scaling/set-range-zoom-to-fit/#zooming-to-fitall-the-data" class="hash-link" aria-label="Direct link to Zooming to fitÂ all the Data" translate="no" title="Direct link to Zooming to fitÂ all the Data">â€‹</a>

Sometimes it is required toÂ makeÂ an axis to **show the fullÂ extent ofÂ the data**Â associated with it. There are several ways to achieve this in code:

1.  Set theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase2d.html#visiblerange" rel="noopener noreferrer" target="_blank">visibleRangeðŸ“˜</a> equal toÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase2d.html#getmaximumrange" rel="noopener noreferrer" target="_blank">axis.getMaximumRange()ðŸ“˜</a>
2.  ConfigureÂ the axis to auto adjustÂ correspondingly toÂ data changes. See the articleÂ [AxisRanging - AutoRange](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/ranging-scaling/auto-range).
3.  Call theÂ functions fromÂ [SciChartSurface](https://www.scichart.com/documentation/js/v4/2d-charts/surface/scichart-surface-type-overview)Â such asÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#zoomextents" rel="noopener noreferrer" target="_blank">ZoomExtents()ðŸ“˜</a>,Â <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#zoomextentsy" rel="noopener noreferrer" target="_blank">ZoomExtentsY()ðŸ“˜</a> andÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#zoomextentsx" rel="noopener noreferrer" target="_blank">ZoomExtentsX()ðŸ“˜</a>Â to force a zoom to fit operation once.

Try this example below:

- TS
- Builder API (JSON Config)

``` prism-code
const { SciChartSurface, NumericAxis, SciChartJsNavyTheme, NumberRange } = SciChart;

// or, for npm, import { SciChartSurface, ... } from "scichart"

// Create a chart with X,Y axis
const { wasmContext, sciChartSurface } = await SciChartSurface.create(divElementId, {
    theme: new SciChartJsNavyTheme()
});
sciChartSurface.xAxes.add(new NumericAxis(wasmContext, { visibleRange: new NumberRange(-2, 20) }));
sciChartSurface.yAxes.add(new NumericAxis(wasmContext, { visibleRange: new NumberRange(-2, 2) }));

document.getElementById("update-chart-button").addEventListener("click", () => {
    // Zoom to fit the chart
    const optionalAnimationDelay = 2000;
    sciChartSurface.zoomExtents(optionalAnimationDelay);

    // See also zoomExtentsY(), zoomExtentsX()
});
```

``` prism-code
const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    surface: { theme: { type: EThemeProviderType.Dark } },
    xAxes: {
        type: EAxisType.NumericAxis,
        options: { axisTitle: "X Axis", visibleRange: new NumberRange(-2, 20) }
    },
    yAxes: {
        type: EAxisType.NumericAxis,
        options: {
            axisTitle: "Y Axis",
            visibleRange: new NumberRange(-2, 2)
        }
    },
    series: [
        {
            type: ESeriesType.LineSeries,
            xyData: {
                xValues,
                yValues
            },
            options: {
                stroke: "#50C7E0",
                strokeThickness: 3
            }
        }
    ]
});

document.getElementById("update-chart-button").addEventListener("click", () => {
    // Zoom to fit the chart
    const optionalAnimationDelay = 2000;
    sciChartSurface.zoomExtents(optionalAnimationDelay);

    // See also zoomExtentsY(), zoomExtentsX()
});
```

This results in the following output:

What if you want to allow the user to zoom to fit using the mouse? All the zooming, panning operations via mouse or touch are handled by ChartModifiers in SciChart.js. See theÂ [ZoomPanModifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/zooming-and-panning/zoom-pan-modifier),Â [ZoomExtentsModifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/zooming-and-panning/zoom-extents-modifier) for information.

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/ranging-scaling/set-range-zoom-to-fit/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [Axis Ranging - AutoRange](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/ranging-scaling/auto-range)
- [Axis Ranging - How to Listen to VisibleRange Changes](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/ranging-scaling/listen-to-visible-range-changes)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/axis-api/ranging-scaling/set-range-zoom-to-fit/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/axis-api/ranging-scaling/set-range-zoom-to-fit/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
