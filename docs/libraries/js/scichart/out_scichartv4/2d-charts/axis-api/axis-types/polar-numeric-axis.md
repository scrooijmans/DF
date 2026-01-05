On this page

# The Polar Numeric Axis

The <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarnumericaxis.html" rel="noopener noreferrer" target="_blank">PolarNumericAxisðŸ“˜</a> is a specialized axis type for polar charts (radar, spider, and polar area plots) in SciChart.js.  
Polar axes map values either around a circle (**Angular**, or "theta") or outward from the center (**Radial**, or "r"). Their unique properties make them essential for any circular, gauge, or radar visualization.

## What Makes Polar Axes Special?<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-types/polar-numeric-axis/#what-makes-polar-axes-special" class="hash-link" aria-label="Direct link to What Makes Polar Axes Special?" translate="no" title="Direct link to What Makes Polar Axes Special?">â€‹</a>

- **Dual Modes using the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarnumericaxis.html#polaraaxismode" rel="noopener noreferrer" target="_blank">PolarNumericAxis.polarAxisModeðŸ“˜</a> to either:**
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/epolaraxismode.html#angular" rel="noopener noreferrer" target="_blank">EPolarAxisMode.AngularðŸ“˜</a>: The axis is mapped to sweeps (angles) around the circle. Typically the **xAxis**.
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/epolaraxismode.html#radial" rel="noopener noreferrer" target="_blank">EPolarAxisMode.RadialðŸ“˜</a>: The axis is mapped from the center outward. Typically the **yAxis**.
- **Fully Customizable Sweep:**
  - Start point using <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarnumericaxis.html#startangle" rel="noopener noreferrer" target="_blank">startAngleðŸ“˜</a>
  - Sweep size on the **Angular** axis via <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarnumericaxis.html#totalangle" rel="noopener noreferrer" target="_blank">totalAngleðŸ“˜</a>.
  - Direction (clockwise/counterclockwise) - manipulated by <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarnumericaxis.html#flippedcoordinates" rel="noopener noreferrer" target="_blank">flippedCoordinatesðŸ“˜</a> set to **true**/**false**. By default, the angular axis starts at 3 o'clock (0 radians) and goes counterclockwise. For a 12 o'clock and clockwise increment set up, use these properties:
    ``` prism-code
    const angularAxis = new PolarNumericAxis(wasmContext, {
      ...
      flippedCoordinates: true, // increment clockwise
      startAngle: -Math.PI / 2, // start at 12 o'clock
      // or `startAngleDegrees: -90 degrees`, the minus is there since we flip the coordinate calculator
    })
    ```
- **Multiple Angular Gridline Styles using <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarnumericaxis.html#gridlinemode" rel="noopener noreferrer" target="_blank">gridlineModeðŸ“˜</a>:**
  - Choose between <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/epolargridlinemode.html#circles" rel="noopener noreferrer" target="_blank">EGridLineMode.CirclesðŸ“˜</a> - which is the default, or <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/epolargridlinemode.html#polygons" rel="noopener noreferrer" target="_blank">EGridLineMode.PolygonsðŸ“˜</a> (spider/radar/cobweb).
- **Support for Donut Charts:**
  - Use <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarnumericaxis.html#innerradius" rel="noopener noreferrer" target="_blank">innerRadiusðŸ“˜</a> to hollow out the centerâ€”great for gauges and donuts. Takes in values between `0` and `1`.
- **Multiple axis label appearances using <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarnumericaxis.html#polarlabelmode" rel="noopener noreferrer" target="_blank">polarLabelModeðŸ“˜</a>:**
  - Options for angular label orientation: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/epolarlabelmode.html" rel="noopener noreferrer" target="_blank">HorizontalðŸ“˜</a>, <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/epolarlabelmode.html#perpendicular" rel="noopener noreferrer" target="_blank">PerpendicularðŸ“˜</a>, <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/epolarlabelmode.html#parallel" rel="noopener noreferrer" target="_blank">ParallelðŸ“˜</a>

## PolarNumericAxis in Action<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-types/polar-numeric-axis/#polarnumericaxis-in-action" class="hash-link" aria-label="Direct link to PolarNumericAxis in Action" translate="no" title="Direct link to PolarNumericAxis in Action">â€‹</a>

Here is how to configure both **angular** and **radial** axes in a polar chart:

``` prism-code
const {
    SciChartPolarSurface,
    SciChartJsNavyTheme,
    PolarNumericAxis,
    PolarLineRenderableSeries,
    XyDataSeries,
    EPolarAxisMode,
    NumberRange,
    EAxisAlignment,
    EPolarLabelMode
} = SciChart;

const { sciChartSurface, wasmContext } = await SciChartPolarSurface.create(rootElement, {
    theme: new SciChartJsNavyTheme()
});

// Angular axis: goes around the circle, from 0 to 360 degrees
const angularAxis = new PolarNumericAxis(wasmContext, {
    polarAxisMode: EPolarAxisMode.Angular, // Lines around the center
    axisAlignment: EAxisAlignment.Top,
    polarLabelMode: EPolarLabelMode.Perpendicular,

    visibleRange: new NumberRange(0, 270),

    startAngleDegrees: 90,  // Start at 12 o'clock
    totalAngleDegrees: 270, // Sweep 270 degrees (3/4 circle)
    flippedCoordinates: true, // Clockwise

    labelPrecision: 0,
    autoTicks: false, // Take control over tick management
    majorDelta: 30,   // set Major ticks at every 30 units (degrees)

    majorGridLineStyle: {
        strokeThickness: 1,
    },
    minorGridLineStyle: {
        strokeThickness: 1,
        color: "rgba(50, 100, 255, 0.2)",
        strokeDashArray: [5, 3]
    }
});
sciChartSurface.xAxes.add(angularAxis);

// Radial axis: from center outward, with circular gridlines
const radialAxis = new PolarNumericAxis(wasmContext, {
    polarAxisMode: EPolarAxisMode.Radial,
    axisAlignment: EAxisAlignment.Right,
    gridlineMode: SciChart.EPolarGridlineMode.Circles, // Or Polygons for spider/radar

    innerRadius: 0.1, // 10% donut hole
    visibleRange: new NumberRange(0, 5),
    drawLabels: true,

    drawMinorGridLines: false,
});
sciChartSurface.yAxes.add(radialAxis);

sciChartSurface.renderableSeries.add(
    new PolarLineRenderableSeries(wasmContext, {
        dataSeries: new XyDataSeries(wasmContext, {
            xValues: Array.from({ length: 28 }, (_, i) => i * 10), // 0, 10, ..., 270
            yValues: Array.from({ length: 28 }, (_, i) => 1 + 3 * Math.abs(Math.sin((i * 10 * Math.PI) / 180)))
        }),
        stroke: "#FF6600",
        strokeThickness: 3,
    })
);
```

## Tips<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-types/polar-numeric-axis/#tips" class="hash-link" aria-label="Direct link to Tips" translate="no" title="Direct link to Tips">â€‹</a>

![](out_scichartv4/2d-charts/axis-api/axis-types/polar-numeric-axis/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

Always pair an **Angular** and **Radial**. You can have multiple axes in a polar chart, but make sure you have one of each first.

![](out_scichartv4/2d-charts/axis-api/axis-types/polar-numeric-axis/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

For a **vertical polar chart**, set the yAxis's <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarnumericaxis.html#polaraxismode" rel="noopener noreferrer" target="_blank">polarAxisModeðŸ“˜</a> to <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/epolaraxismode.html#angular" rel="noopener noreferrer" target="_blank">AngularðŸ“˜</a> and the xAxis's to <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/epolaraxismode.html#radial" rel="noopener noreferrer" target="_blank">RadialðŸ“˜</a>

## Best Practices<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-types/polar-numeric-axis/#best-practices" class="hash-link" aria-label="Direct link to Best Practices" translate="no" title="Direct link to Best Practices">â€‹</a>

- Use <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarnumericaxis.html#innerradius" rel="noopener noreferrer" target="_blank">innerRadiusðŸ“˜</a> for donut/ring charts, or <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarnumericaxis.html#gridlinemode" rel="noopener noreferrer" target="_blank">gridlineModeðŸ“˜</a>: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/epolargridlinemode.html#polygons" rel="noopener noreferrer" target="_blank">PolygonsðŸ“˜</a> for radar/spider style.

- Customize tick intervals by setting <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polaraxisbase.html#autoticks" rel="noopener noreferrer" target="_blank">autoTicksðŸ“˜</a> to `false` and playing with <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polaraxisbase.html#majordelta" rel="noopener noreferrer" target="_blank">majorDeltaðŸ“˜</a> and <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polaraxisbase.html#minordelta" rel="noopener noreferrer" target="_blank">minorDeltaðŸ“˜</a>

- Use <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarnumericaxis.html#xcenteroffset" rel="noopener noreferrer" target="_blank">xCenterOffsetðŸ“˜</a> and <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarnumericaxis.html#ycenteroffset" rel="noopener noreferrer" target="_blank">yCenterOffsetðŸ“˜</a> for fine layout control, great for dashboards and overlays.

- The <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polaraxisbase.html#labelpostfix" rel="noopener noreferrer" target="_blank">labelPostfixðŸ“˜</a> is often set to `Â°`, along with <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polaraxisbase.html#labelprecision" rel="noopener noreferrer" target="_blank">labelPrecisionðŸ“˜</a> = `0` to indicate degrees.

- For smaller polar charts, setting <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polaraxisbase.html#drawminorgirdlines" rel="noopener noreferrer" target="_blank">drawMinorGridLinesðŸ“˜</a> to `false` can help improve readability by only keeping the major grid lines.

- By default, <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polaraxisbase.html#labelprecision" rel="noopener noreferrer" target="_blank">labelPrecisionðŸ“˜</a> is set to `1`, but if you work with degrees or just larger datasets, you may want to set it to `0` and format the label as `270Â°` instead of `270.0Â°`.

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/axis-api/axis-types/polar-numeric-axis/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/axis-api/axis-types/polar-numeric-axis/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
