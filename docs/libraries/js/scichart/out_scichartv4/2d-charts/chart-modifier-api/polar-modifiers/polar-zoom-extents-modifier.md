On this page

# PolarZoomExtentsModifier

SciChart.js provides the ability to Zoom Extents the polar chart (zoom to fit data) by double-clicking the chart area with the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarzoomextentsmodifier.html" rel="noopener noreferrer" target="_blank">PolarZoomExtentsModifierðŸ“˜</a>, available out of the box.

Here is how to define the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarzoomextentsmodifier.html" rel="noopener noreferrer" target="_blank">PolarZoomExtentsModifierðŸ“˜</a> in your code:

- TS
- Builder API (JSON Config)

``` prism-code
const { PolarZoomExtentsModifier, Point } = SciChart;
// or for npm: import { PolarZoomExtentsModifier } from "scichart";

sciChartSurface.chartModifiers.add(
    // Zoom Extents Modifier:
    new PolarZoomExtentsModifier({
        centerPoint: new Point(0, 0),
        animationDuration: 1000,
        innerRadius: 0.2
    })
);
```

``` prism-code
// Demonstrates how to configure the PolarZoomExtentsModifier in SciChart.js using the Builder API
const {
    chartBuilder,
    EAxisAlignment,
    EThemeProviderType,
    EAxisType,
    EChart2DModifierType,
    EPolarAxisMode,
    Point,
    ESeriesType
} = SciChart;
// or, for npm, import { chartBuilder, ... } from "scichart"

const { wasmContext, sciChartSurface } = await chartBuilder.build2DPolarChart(divElementId, {
    surface: { theme: { type: EThemeProviderType.Dark } },
    xAxes: [
        {
            type: EAxisType.PolarNumericAxis,
            options: {
                polarAxisMode: EPolarAxisMode.Radial,
                axisAlignment: EAxisAlignment.Left,
                innerRadius: 0.1,
                startAngleDegrees: 90
            }
        }
    ],
    yAxes: [
        {
            type: EAxisType.PolarNumericAxis,
            options: {
                polarAxisMode: EPolarAxisMode.Angular,
                axisAlignment: EAxisAlignment.Bottom,
                startAngleDegrees: 90
            }
        }
    ],
    series: {
        type: ESeriesType.PolarColumnSeries,
        options: {
            stroke: "#50C7E0",
            fill: "#50C7E044",
            strokeThickness: 2
        },
        xyData: {
            xValues: Array.from({ length: 5 }, (_, i) => i),
            yValues: Array.from({ length: 5 }, (_, i) => 1 + Math.sin(i * 0.3))
        }
    },
    modifiers: [
        {
            type: EChart2DModifierType.PolarZoomExtents,
            options: {
                xStartAngle: Math.PI / 4,
                yStartAngle: 0,
                totalAngle: Math.PI * 2,
                centerPoint: new Point(0, 0),
                animationDuration: 1000
            }
        }
    ]
});
```

## Zoom to a Preset Range<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/polar-modifiers/polar-zoom-extents-modifier/#zoom-to-a-preset-range" class="hash-link" aria-label="Direct link to Zoom to a Preset Range" translate="no" title="Direct link to Zoom to a Preset Range">â€‹</a>

If you would like the double-click to zoom to some preset range, rather than the data range, you can set `zoomExtentsRange` on the axes. In addition, if you are setting an initial visibleRange on an axis and would like zoomExtents to return to this range, you can just set `zoomExtentsToInitialRange` true, which will set `zoomExtentsRange` to the `visibleRange` passed in.

Besides common features which are inherited from the base <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/zoomextentsmodifier.html" rel="noopener noreferrer" target="_blank">ZoomExtentsModifierðŸ“˜</a> class, the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarzoomextentsmodifier.html" rel="noopener noreferrer" target="_blank">PolarZoomExtentsModifierðŸ“˜</a> has many more polar-specific features, such as:

| Property | Type | Description |
|----|----|----|
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarzoomextentsmodifier.html#xstartangle" rel="noopener noreferrer" target="_blank">xStartAngleðŸ“˜</a> | number | The start angle for the X-axis in the polar chart. |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarzoomextentsmodifier.html#ystartangle" rel="noopener noreferrer" target="_blank">yStartAngleðŸ“˜</a> | number | The start angle for the Y-axis in the polar chart. |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarzoomextentsmodifier.html#totalangle" rel="noopener noreferrer" target="_blank">totalAngleðŸ“˜</a> | number | The total angle of the polar chart, which defines the range of angles covered by the chart. |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarzoomextentsmodifier.html#lengthscale" rel="noopener noreferrer" target="_blank">lengthScaleðŸ“˜</a> | number | The scale factor for the radian axis |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarzoomextentsmodifier.html#innerradius" rel="noopener noreferrer" target="_blank">innerRadiusðŸ“˜</a> | number | The inner radius of the polar chart, which defines the minimum distance from the center to the edge of the chart. |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarzoomextentsmodifier.html#centerpoint" rel="noopener noreferrer" target="_blank">centerPointðŸ“˜</a> | <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/point.html" rel="noopener noreferrer" target="_blank">PointðŸ“˜</a> | Center point of the polar chart, which defines the origin of the polar coordinates. |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarzoomextentsmodifier.html#resetstartangles" rel="noopener noreferrer" target="_blank">resetStartAnglesðŸ“˜</a> | boolean | Whether to reset the start angles for both the X and Y axes to their initial values. |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarzoomextentsmodifier.html#resettotalangle" rel="noopener noreferrer" target="_blank">resetTotalAngleðŸ“˜</a> | boolean | Whether to reset the total angle to its initial value. |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarzoomextentsmodifier.html#resetranges" rel="noopener noreferrer" target="_blank">resetRangesðŸ“˜</a> | boolean | Whether to reset the ranges for both the radial and angular axes to their initial values. |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarzoomextentsmodifier.html#resetlengthscale" rel="noopener noreferrer" target="_blank">resetLengthScaleðŸ“˜</a> | boolean | Whether to reset the length scale to its initial value. |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarzoomextentsmodifier.html#resetcenterpoint" rel="noopener noreferrer" target="_blank">resetCenterPointðŸ“˜</a> | boolean | Whether to reset the center point to its initial value. |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarzoomextentsmodifier.html#resetinnerradius" rel="noopener noreferrer" target="_blank">resetInnerRadiusðŸ“˜</a> | boolean | Whether to reset the inner radius to its initial value. |

See all at <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipolarzoomextentsmodifieroptions.html" rel="noopener noreferrer" target="_blank">IPolarZoomExtentsModifierOptionsðŸ“˜</a>.

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/polar-modifiers/polar-zoom-extents-modifier/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [What is the ChartModifier API?](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/chart-modifier-api-overview)
- [ZoomExtentsModifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/zooming-and-panning/zoom-extents-modifier)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-modifier-api/polar-modifiers/polar-zoom-extents-modifier/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-modifier-api/polar-modifiers/polar-zoom-extents-modifier/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
