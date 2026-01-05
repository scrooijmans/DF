On this page

# The Polar Category Axis

The <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarcategoryaxis.html" rel="noopener noreferrer" target="_blank">PolarCategoryAxisðŸ“˜</a> brings category (discrete, label-based, or index-based) data to polar chartsâ€”enabling radar, spider, and circular bar charts where each sector represents a category or qualitative dimension.

**Key features:**

- Lets you plot non-numeric, ordinal, or named categories around a circle (e.g., "Defense", "Shooting", ...).
- Works in both angular (sweep) and radial (outward) modes.
- Supports custom label lists via <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarcategoryaxis.html#labels" rel="noopener noreferrer" target="_blank">labelsðŸ“˜</a>, <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarcategoryaxis.html#startangle" rel="noopener noreferrer" target="_blank">startAngleðŸ“˜</a>, <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarcategoryaxis.html#totalangle" rel="noopener noreferrer" target="_blank">totalAngleðŸ“˜</a>, gridline style, and many more.
- Collapses â€œgapsâ€? in your data, making it perfect for displaying categorical time-series or performance data.

## Key Options<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-types/polar-category-axis/#key-options" class="hash-link" aria-label="Direct link to Key Options" translate="no" title="Direct link to Key Options">â€‹</a>

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarcategoryaxis.html#labels" rel="noopener noreferrer" target="_blank">labelsðŸ“˜</a> â€“ Array of strings for each category.
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarcategoryaxis.html#polaraaxismode" rel="noopener noreferrer" target="_blank">polarAxisModeðŸ“˜</a> â€“ Set to <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/epolaraxismode.html#angular" rel="noopener noreferrer" target="_blank">AngularðŸ“˜</a> to lay categories around the sweep; or <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/epolaraxismode.html#radial" rel="noopener noreferrer" target="_blank">RadialðŸ“˜</a> for concentric category rings.
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarcategoryaxis.html#startangle" rel="noopener noreferrer" target="_blank">startAngle / startAngleDegreesðŸ“˜</a> â€“ Where the axis begins (e.g., `Math.PI / 2` for 12 oâ€™clock).
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarcategoryaxis.html#flippedcoordinates" rel="noopener noreferrer" target="_blank">flippedCoordinatesðŸ“˜</a> â€“ Set to `true` for clockwise domain.
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarcategoryaxis.html#polarlabelmode" rel="noopener noreferrer" target="_blank">polarLabelModeðŸ“˜</a> â€“ Controls label orientation (parallel, perpendicular, horizontal, etc).

## Example: Category Radar Chart<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-types/polar-category-axis/#example-category-radar-chart" class="hash-link" aria-label="Direct link to Example: Category Radar Chart" translate="no" title="Direct link to Example: Category Radar Chart">â€‹</a>

``` prism-code
const {
    SciChartPolarSurface,
    SciChartJsNavyTheme,
    PolarCategoryAxis,
    PolarLineRenderableSeries,
    XyDataSeries,
    EPolarAxisMode,
    NumberRange,
    EPolarGridlineMode,
    EAxisAlignment,
    EPolarLabelMode
} = SciChart;

const { sciChartSurface, wasmContext } = await SciChartPolarSurface.create(rootElement, {
    theme: new SciChartJsNavyTheme()
});

// Angular axis: goes around the circle, from 0 to 360 degrees
const angularAxis = new PolarCategoryAxis(wasmContext, {
    polarAxisMode: EPolarAxisMode.Angular, // Lines around the center
    axisAlignment: EAxisAlignment.Top,
    drawMinorGridLines: false,

    startAngle: Math.PI / 2, // Start chart at 12 o'clock
    labels: ["Bandwidth", "Latency", "Throughput", "Capacity", "Efficiency", "Scalability", "Reliability"],
});
sciChartSurface.xAxes.add(angularAxis);

// Radial axis: from center outward, with circular gridlines
const radialAxis = new PolarCategoryAxis(wasmContext, {
    polarAxisMode: EPolarAxisMode.Radial,
    axisAlignment: EAxisAlignment.Right,
    polarLabelMode: EPolarLabelMode.Perpendicular,
    innerRadius: 0.1, 
    gridlineMode: EPolarGridlineMode.Polygons, // radar chart look

    flippedCoordinates: true, // from "Low" to "Excellent"
    labels: ["Low", "Medium", "High", "Very High", "Excellent"],
    visibleRange: new NumberRange(0, 4),
    startAngle: Math.PI / 2, // draw radial labels at 12 o'clock
});
sciChartSurface.yAxes.add(radialAxis);

sciChartSurface.renderableSeries.add(
    new PolarLineRenderableSeries(wasmContext, {
        dataSeries: new XyDataSeries(wasmContext, {
            xValues: Array.from({ length: 8 }, (_, i) => i),
            yValues: [1, 2, 3, 4, 0, 4, 3, 1] // Example values for each category
        }),
        stroke: "#FF6600",
        strokeThickness: 3,
    })
);
```

![](out_scichartv4/2d-charts/axis-api/axis-types/polar-category-axis/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

The number of string elements inside the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarcategoryaxis.html#labels" rel="noopener noreferrer" target="_blank">labelsðŸ“˜</a> array determines the number of *possible* text tick labels, it does not force the chart to render that amount of ticks directly, that is still determined by the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarcategoryaxis.html#visibleRange" rel="noopener noreferrer" target="_blank">visibleRangeðŸ“˜</a> of the angular axis, or the x/y-range calculated to include all the renderable series's points, if present.

## Common Use Cases<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-types/polar-category-axis/#common-use-cases" class="hash-link" aria-label="Direct link to Common Use Cases" translate="no" title="Direct link to Common Use Cases">â€‹</a>

- Stats, analytics, comparisons between entities
- Product or feature comparison (radar charts)
- Survey, KPI, and performance dashboards
- Any polar visualization with qualitative or ordinal axes

## See the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarcategoryaxis.html" rel="noopener noreferrer" target="_blank">PolarCategoryAxisðŸ“˜</a> in Action:<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-types/polar-category-axis/#see-the-polarcategoryaxisblue_book-in-action" class="hash-link" aria-label="Direct link to see-the-polarcategoryaxisblue_book-in-action" translate="no" title="Direct link to see-the-polarcategoryaxisblue_book-in-action">â€‹</a>

- <a href="https://www.scichart.com/demo/react/polar-column-category-chart" rel="noopener noreferrer" target="_blank">https://www.scichart.com/demo/react/polar-column-category-chart</a>
- <a href="https://www.scichart.com/demo/react/polar-stacked-radial-column-chart" rel="noopener noreferrer" target="_blank">https://www.scichart.com/demo/react/polar-stacked-radial-column-chart</a>
- <a href="https://www.scichart.com/demo/react/polar-line-temperature-average" rel="noopener noreferrer" target="_blank">https://www.scichart.com/demo/react/polar-line-temperature-average</a>

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/axis-api/axis-types/polar-category-axis/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/axis-api/axis-types/polar-category-axis/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
