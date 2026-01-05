On this page

# Polar Axis Styling

Polar axes in SciChart.js are specialized axes designed for polar charts, which are circular in nature. They can be used to create various types of polar charts, such as radial bar charts, polar line charts, and more.

There are currently only 2 types of polar axes available: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarnumericaxis.html" rel="noopener noreferrer" target="_blank">PolarNumericAxisðŸ“˜</a> and <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarcategoryaxis.html" rel="noopener noreferrer" target="_blank">PolarCategoryAxisðŸ“˜</a>, which are used for numeric and categorical data, respectively. Most properties of these two are the same, with some differences in the way they handle data and labels.

## Key Polar Axis Concepts<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-styling/polar-axis-styling/#key-polar-axis-concepts" class="hash-link" aria-label="Direct link to Key Polar Axis Concepts" translate="no" title="Direct link to Key Polar Axis Concepts">â€‹</a>

### Angle Sweep & Start<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-styling/polar-axis-styling/#angle-sweep--start" class="hash-link" aria-label="Direct link to Angle Sweep &amp; Start" translate="no" title="Direct link to Angle Sweep &amp; Start">â€‹</a>

> how far the axis sweeps, and where it starts from on the circle, via <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polaraxisbase.html#totalangle" rel="noopener noreferrer" target="_blank">totalAngleðŸ“˜</a> and <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polaraxisbase.html#startangle" rel="noopener noreferrer" target="_blank">startAngleðŸ“˜</a> properties, having values in the range of `0` to `2 * Math.PI`.

![](out_scichartv4/2d-charts/axis-api/axis-styling/polar-axis-styling/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

There is also a **degree mode**, with values in between `0` and `360` degrees but used with <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polaraxisbase.html#totalangledegrees" rel="noopener noreferrer" target="_blank">totalAngleDegreesðŸ“˜</a> and <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polaraxisbase.html#startangledegrees" rel="noopener noreferrer" target="_blank">startAngleDegreesðŸ“˜</a> instead.

### Flipped Coordinates<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-styling/polar-axis-styling/#flipped-coordinates" class="hash-link" aria-label="Direct link to Flipped Coordinates" translate="no" title="Direct link to Flipped Coordinates">â€‹</a>

> whether the axis increments clockwise or counter-clockwise, via <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polaraxisbase.html#flippedcoordinates" rel="noopener noreferrer" target="_blank">flippedCoordinatesðŸ“˜</a> property, can be set on any of the axes.
>
> When `flippedCoordinates` is set to `true` on the Angular axis, the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polaraxisbase.html#startangle" rel="noopener noreferrer" target="_blank">startAngleðŸ“˜</a> will be reversed, so `-Math.PI/2` will not make the axis start at 12 o'clock (as it usually does), but rather at 6 o'clock.

### Polar Labels<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-styling/polar-axis-styling/#polar-labels" class="hash-link" aria-label="Direct link to Polar Labels" translate="no" title="Direct link to Polar Labels">â€‹</a>

> how the labels are displayed, via <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polaraxisbase.html#polarlabelmode" rel="noopener noreferrer" target="_blank">polarLabelModeðŸ“˜</a> property.
>
> The default value is `polarLabelMode`: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/epolarlabelmode.html#horizontal" rel="noopener noreferrer" target="_blank">HorizontalðŸ“˜</a>, which means that the labels will be drawn horizontally (never rotating), while the <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/epolarlabelmode.html#perpendicular" rel="noopener noreferrer" target="_blank">PerpendicularðŸ“˜</a> mode will draw the labels perpendicular to the axis, while the <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/epolarlabelmode.html#parallel" rel="noopener noreferrer" target="_blank">ParallelðŸ“˜</a> mode will draw them parallel to the axis.

![](out_scichartv4/2d-charts/axis-api/axis-styling/polar-axis-styling/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

There is also the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polaraxisbase.html#labelstyle" rel="noopener noreferrer" target="_blank">labelStyleðŸ“˜</a> property, which acts the same way as the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polaraxisbase.html#labelstyle" rel="noopener noreferrer" target="_blank">AxisBase.labelStyleðŸ“˜</a> property, allowing you to set the font, color, and other styles of the labels based on the <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#ttextstyle" rel="noopener noreferrer" target="_blank">TLabelStyleðŸ“˜</a> type.

### Grid mode for Radial Gridlines<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-styling/polar-axis-styling/#grid-mode-for-radial-gridlines" class="hash-link" aria-label="Direct link to Grid mode for Radial Gridlines" translate="no" title="Direct link to Grid mode for Radial Gridlines">â€‹</a>

> The grid mode for radial gridlines can be controlled via the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polaraxisbase.html#gridlineMode" rel="noopener noreferrer" target="_blank">gridlineModeðŸ“˜</a> property. This property determines how the radial gridlines are drawn.
>
> For a Spider/Radar chart look you can set this property to <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/epolargridlinemode.html#polygon" rel="noopener noreferrer" target="_blank">EPolarGridLineMode.PolygonðŸ“˜</a>, which will draw the gridlines as concentric polygons, or you can stick to the default <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/epolargridlinemode.html#circles" rel="noopener noreferrer" target="_blank">EPolarGridLineMode.CirclesðŸ“˜</a> which draws radial gridlines as arcs.

## Other Styling Properties<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-styling/polar-axis-styling/#other-styling-properties" class="hash-link" aria-label="Direct link to Other Styling Properties" translate="no" title="Direct link to Other Styling Properties">â€‹</a>

### Major Gridlines<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-styling/polar-axis-styling/#major-gridlines" class="hash-link" aria-label="Direct link to Major Gridlines" translate="no" title="Direct link to Major Gridlines">â€‹</a>

> aligned with labels having <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polaraxisbase.html#majorglstyle" rel="noopener noreferrer" target="_blank">majorGridLineStyleðŸ“˜</a> and <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polaraxisbase.html#drawmajorgl" rel="noopener noreferrer" target="_blank">drawMajorGridLinesðŸ“˜</a> properties.

### Minor Gridlines<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-styling/polar-axis-styling/#minor-gridlines" class="hash-link" aria-label="Direct link to Minor Gridlines" translate="no" title="Direct link to Minor Gridlines">â€‹</a>

> between labels with <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polaraxisbase.html#minorglstyle" rel="noopener noreferrer" target="_blank">minorGridLineStyleðŸ“˜</a> and <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polaraxisbase.html#drawminorgl" rel="noopener noreferrer" target="_blank">drawMinorGridLinesðŸ“˜</a> properties.

![](out_scichartv4/2d-charts/axis-api/axis-styling/polar-axis-styling/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

Both Major and Minor gridlines **style** objects have the <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tgridlinestyle" rel="noopener noreferrer" target="_blank">TGridLineStyleðŸ“˜</a> type, which allows you to set the `color`, `thickness`, and `strokeDashArray` of the gridlines.

Also, the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polaraxisbase.html#drawmajorgl" rel="noopener noreferrer" target="_blank">drawMajorGridLinesðŸ“˜</a> and <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polaraxisbase.html#drawminorgl" rel="noopener noreferrer" target="_blank">drawMinorGridLinesðŸ“˜</a> properties are `boolean` values that control whether the gridlines are drawn or not.

### Major Ticks<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-styling/polar-axis-styling/#major-ticks" class="hash-link" aria-label="Direct link to Major Ticks" translate="no" title="Direct link to Major Ticks">â€‹</a>

> small marks, outside the axis, aligned with labels with <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polaraxisbase.html#majortickstyle" rel="noopener noreferrer" target="_blank">majorTickLineStyleðŸ“˜</a> and <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polaraxisbase.html#drawmajorticks" rel="noopener noreferrer" target="_blank">drawMajorTicksðŸ“˜</a> properties.

### Minor Ticks<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-styling/polar-axis-styling/#minor-ticks" class="hash-link" aria-label="Direct link to Minor Ticks" translate="no" title="Direct link to Minor Ticks">â€‹</a>

> small marks, outside the axis, between labels with <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polaraxisbase.html#minortickstyle" rel="noopener noreferrer" target="_blank">minorTickLineStyleðŸ“˜</a> and <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polaraxisbase.html#drawminorticks" rel="noopener noreferrer" target="_blank">drawMinorTicksðŸ“˜</a> properties.

![](out_scichartv4/2d-charts/axis-api/axis-styling/polar-axis-styling/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

Both Major and Minor ticks **style** objects have the <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tticklinestyle" rel="noopener noreferrer" target="_blank">TTickLineStyleðŸ“˜</a> type, which allows you to set the `color`, `tickSize`, and `strokeThickess` of the ticks.

## Polar Axis Styling Example<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-styling/polar-axis-styling/#polar-axis-styling-example" class="hash-link" aria-label="Direct link to Polar Axis Styling Example" translate="no" title="Direct link to Polar Axis Styling Example">â€‹</a>

Read the code comments carefully

- TS

``` prism-code
const {
    SciChartPolarSurface,
    PolarNumericAxis,
    NumberRange,
    SciChartJsNavyTheme,
    EPolarAxisMode,
    EPolarGridlineMode,
    PolarCategoryAxis,
    EPolarLabelMode
} = SciChart;

// Create a SciChartPolarSurface
const { wasmContext, sciChartSurface } = await SciChartPolarSurface.create(divElementId, {
    theme: new SciChartJsNavyTheme(),
});

// Add polar axes
const TOTAL_ANGLE = Math.PI * 1.5; // 270 degrees in radians (3 quarters of a circle)

sciChartSurface.xAxes.add(
    new PolarCategoryAxis(wasmContext, {
        polarAxisMode: EPolarAxisMode.Angular,
        visibleRange: new NumberRange(0, 9),

        totalAngle: TOTAL_ANGLE, // in radians
        // totalAngleDegrees: 270, // if you want to work in degrees

        flippedCoordinates: true, // increment clockwise
        startAngle: (Math.PI - TOTAL_ANGLE) / 2, // formula to center incomplete polar surfaces like gauges ("with the missing slice at the bottom")

        autoTicks: false, // take control over tick distance
        majorDelta: 1,    // draw tick every 1 unit

        // minor gridlines turned off look better when radial axis gridlineMode is set to `Polygons`
        drawMinorGridLines: false,

        majorTickLineStyle: { // optionally style major ticks
            color: "#FFFFFF",
            tickSize: 2,
            strokeThickness: 2,
        },

        majorGridLineStyle: { // optionally style major grid lines
            color: "rgba(12,17,53,1)",
            strokeThickness: 1,
            strokeDashArray: [5, 2], // dashed lines
        },

        labels: ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth"],
        labelStyle: {
            color: "#FFFFFF",
        },
        polarLabelMode: EPolarLabelMode.Parallel // can also be "Perpendicular", or (the default) "Horizontal" 
    })
);

sciChartSurface.yAxes.add(
    new PolarNumericAxis(wasmContext, {
        polarAxisMode: EPolarAxisMode.Radial,
        visibleRange: new NumberRange(0, 10),
        gridlineMode: EPolarGridlineMode.Polygons, // results in a radar chart look (straight radial lines in between grid lines)

        startAngle: (Math.PI - TOTAL_ANGLE) / 2, // match the radial labels to the start of the angular axis
        labelPrecision: 0,

        majorGridLineStyle: { // optionally style major grid lines
            color: "rgba(12,17,53,1)",
            strokeThickness: 1,
            strokeDashArray: [5, 2], // dashed lines
        },
    })
);
```

This results in the following output:

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-styling/polar-axis-styling/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [Polar Chart Layout](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/multi-axis-and-layout/polar-chart-layout)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/axis-api/axis-styling/polar-axis-styling/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/axis-api/axis-styling/polar-axis-styling/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
