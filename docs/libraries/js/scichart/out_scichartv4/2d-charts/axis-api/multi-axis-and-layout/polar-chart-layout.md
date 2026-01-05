On this page

# Polar Chart Layout

Polar charts - charts created with <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartpolarsurface.html#create" rel="noopener noreferrer" target="_blank">SciChartPolarSurface.create()ðŸ“˜</a>, have a different axis layout compared to Cartesian charts. In polar charts, the axes are arranged in a circular manner, which allows for `radial` and `angular` measurements.

The easiest way of thinking about polar axes is this:

- **Radial Axis**: Measures distance from the center of the chart up until the border of it, **like a radius**, and is typically used in the same logic an `Y Axis` is used in 2D Cartesian charts.
- **Angular Axis**: Measures the angle around the center of the chart, **like a circle**, and is typically used in the same logic an `X Axis` is used in 2D Cartesian charts.

![](out_scichartv4/2d-charts/axis-api/multi-axis-and-layout/polar-chart-layout/index_media/5a72c3ee6a9cc14296f901057d6eabf2b9b712b4.svg)note

This is just a suggestion, and in the same way you can have [Vertical Charts](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/multi-axis-and-layout/vertical-charts-rotate-transpose-axis/) on Cartesian surfaces, (where the traditional X and Y axes are interchanged).

### Regular vs Vertical Polar Charts<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/multi-axis-and-layout/polar-chart-layout/#regular-vs-vertical-polar-charts" class="hash-link" aria-label="Direct link to Regular vs Vertical Polar Charts" translate="no" title="Direct link to Regular vs Vertical Polar Charts">â€‹</a>

Notice the following 2 examples, where the first one is a regular, horizontal polar chart, using the `Angular` axis as an x-axis, representing the separate columns of the chart and the `Radial` axis as a y-axis representing the height of each column in the chart, while the second chart is a vertical polar chart, where the roles are reversed.

Above: The JavaScript <a href="https://www.scichart.com/demo/iframe/polar-column-chart" target="_blank">Polar Column Chart Example</a> example from the <a href="https://www.scichart.com/demo/react" target="_blank">SciChart.js Demo</a>

  

Above: The JavaScript <a href="https://www.scichart.com/demo/iframe/polar-radial-column-chart" target="_blank">Polar Radial Column Chart Example</a> example from the <a href="https://www.scichart.com/demo/react" target="_blank">SciChart.js Demo</a>

  

### Creating the 2 axes of a Polar surface in SciChart.js can be done like so:<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/multi-axis-and-layout/polar-chart-layout/#creating-the-2-axes-of-a-polar-surface-in-scichartjs-can-be-done-like-so" class="hash-link" aria-label="Direct link to Creating the 2 axes of a Polar surface in SciChart.js can be done like so:" translate="no" title="Direct link to Creating the 2 axes of a Polar surface in SciChart.js can be done like so:">â€‹</a>

- Creating the Radial and Angular Axes

``` prism-code
const { sciChartSurface, wasmContext } = await SciChartPolarSurface.create(divElementId, {
    theme: new SciChartJsNavyTheme(),
    title: "Polar Chart Layout Example"
});

// Add polar axes
const angularXAxis = new PolarNumericAxis(wasmContext, {
    polarAxisMode: EPolarAxisMode.Angular,
    visibleRange: new NumberRange(0, 360) // 0 to 360 degrees
});
sciChartSurface.xAxes.add(angularXAxis);

const radialYAxis = new PolarNumericAxis(wasmContext, {
    polarAxisMode: EPolarAxisMode.Radial,
    visibleRange: new NumberRange(0, 10) // 0 to 10 units of height
});
sciChartSurface.yAxes.add(radialYAxis);
```

This will result in the following layout:

### Advanced Polar Chart Layout<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/multi-axis-and-layout/polar-chart-layout/#advanced-polar-chart-layout" class="hash-link" aria-label="Direct link to Advanced Polar Chart Layout" translate="no" title="Direct link to Advanced Polar Chart Layout">â€‹</a>

For the Polar Axes, we have created a special label provider, available out of the box, called <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/radianlabelprovider.html" rel="noopener noreferrer" target="_blank">RadianLabelProviderðŸ“˜</a>, which formats the labels in radians. Make sure to read the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/radianlabelprovider.html" rel="noopener noreferrer" target="_blank">TSDoc indicationsðŸ“˜</a> before using it, and observe how the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/radianlabelprovider.html#errortolerance" rel="noopener noreferrer" target="_blank">errorToleranceðŸ“˜</a> and <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/radianlabelprovider.html#maxdenominator" rel="noopener noreferrer" target="_blank">maxDenominatorðŸ“˜</a> pair with <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polaraxisbase.html#autoticks" rel="noopener noreferrer" target="_blank">AxisBase.autoTicksðŸ“˜</a> and <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polaraxisbase.html#majordelta" rel="noopener noreferrer" target="_blank">AxisBase.majorDeltaðŸ“˜</a> to determine the label values.

- Advanced Polar Chart Layout

``` prism-code
const {
    SciChartPolarSurface,
    PolarNumericAxis,
    NumberRange,
    RadianLabelProvider,
    SciChartJsNavyTheme,
    EPolarAxisMode,
} = SciChart;

// Create a SciChartPolarSurface
const { wasmContext, sciChartSurface } = await SciChartPolarSurface.create(divElementId, {
    theme: new SciChartJsNavyTheme(),
});

// Add polar axes
const angularXAxis = new PolarNumericAxis(wasmContext, {
    polarAxisMode: EPolarAxisMode.Angular,
    visibleRange: new NumberRange(0, Math.PI * 2), // 0 to 2Ï radians

    labelProvider: new RadianLabelProvider({
        maxDenominator: 6,   // 6 is the maximum denominator for fractions, e.g. Ï/6, 2Ï/3, but NOT 5Ï/12
        errorTolerance: 0.1, // since PI divisions are not exact, we allow a small error tolerance
    }),
    autoTicks: false,        // take control over tick distance
    majorDelta: Math.PI / 6, // 30 degrees in radians

    totalAngle: Math.PI * 2, // this is the default value, but can be set to a different value if needed (e.g. Math.PI for a half-circle chart)
    startAngle: 0,           // this is the default value, but can be set to a different value if needed (e.g. Math.PI / 2 to start at 12 o'clock)

    isInnerAxis: false,      // whether to draw labels inside or outside of the polar chart
    flippedCoordinates: false,// whether to go clockwise or counter-clockwise (default is counter-clockwise)
})
sciChartSurface.xAxes.add(angularXAxis);

const radialYAxis = new PolarNumericAxis(wasmContext, {
    polarAxisMode: EPolarAxisMode.Radial,
    visibleRange: new NumberRange(0, 10),

    innerRadius: 0.2,        // donut hole in the middle
    labelPrecision: 0,
    drawMinorGridLines: false,
})
sciChartSurface.yAxes.add(radialYAxis);
```

And this is how it looks like:

### Partial Polar Chart Layout<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/multi-axis-and-layout/polar-chart-layout/#partial-polar-chart-layout" class="hash-link" aria-label="Direct link to Partial Polar Chart Layout" translate="no" title="Direct link to Partial Polar Chart Layout">â€‹</a>

By changing the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polaraxisbase.html#totalAngle" rel="noopener noreferrer" target="_blank">totalAngleðŸ“˜</a> property on your angular axis, you can control the sweeping angle of your polar surface, e.g. you can have half-circles, quarter-circles, etc. It expects values in between `0` and `Math.PI * 2`

![](out_scichartv4/2d-charts/axis-api/multi-axis-and-layout/polar-chart-layout/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

There is also a <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polaraxisbase.html#totalAngleDegrees" rel="noopener noreferrer" target="_blank">totalAngleDegreesðŸ“˜</a> property available for convenience, with values in between `0` and `360`

- Partial Polar Layout

``` prism-code
const {
    SciChartPolarSurface,
    PolarNumericAxis,
    NumberRange,
    RadianLabelProvider,
    SciChartJsNavyTheme,
    EPolarAxisMode,
} = SciChart;

// Create a SciChartPolarSurface
const { wasmContext, sciChartSurface } = await SciChartPolarSurface.create(divElementId, {
    theme: new SciChartJsNavyTheme(),
});

// Add polar axes
const angularXAxis = new PolarNumericAxis(wasmContext, {
    polarAxisMode: EPolarAxisMode.Angular,
    visibleRange: new NumberRange(0, Math.PI * 1.5), // 0 to 1.5Ï radians

    labelProvider: new RadianLabelProvider({
        maxDenominator: 3,      // 3 is the maximum denominator for fractions, e.g. 2Ï/3, but NOT Ï/6
        errorTolerance: 0.001,  // since PI divisions are not exact, we allow a small error tolerance
    }),
    autoTicks: false,        // take control over tick distance
    majorDelta: Math.PI / 4, // 45 degrees in radians

    totalAngle: Math.PI * 1.5, 
    // totalAngleDegrees: 270,  // same thing as Math.PI * 1.5 radians, but in degrees

    startAngle: -Math.PI / 4,   // this is the default value, but can be set to a different value if needed (e.g. Math.PI / 2 to start at 12 o'clock)
    // startAngleDegrees: -45,  // same thing as -Math.PI/4 radians, but in degrees

    isInnerAxis: false,         // whether to draw labels inside or outside of the polar chart
    flippedCoordinates: false,  // whether to go clockwise or counter-clockwise (default is counter-clockwise)
})
sciChartSurface.xAxes.add(angularXAxis);

const radialYAxis = new PolarNumericAxis(wasmContext, {
    polarAxisMode: EPolarAxisMode.Radial,
    visibleRange: new NumberRange(0, 10),

    labelPrecision: 0,
    drawMinorGridLines: false,
    startAngle: -Math.PI / 4,    // also match the radial labels to the start of the angular axis
})
sciChartSurface.yAxes.add(radialYAxis);

// optionally add some polar arcs to showcase the functionality of startAngle and totalAngle
const arcThresholds = [1.8, 2.9, Math.PI * 1.5];
const innerRadius = 7.5;
const outerRadius = 9.5;

sciChartSurface.annotations.add(
    new SciChart.PolarArcAnnotation({
        isEditable: true,
        x1: 0,
        x2: arcThresholds[0],

        y1: outerRadius,
        y2: innerRadius,

        fill: "rgba(213, 76, 96, 0.5)",
        stroke: "rgba(213, 76, 96, 1)",
        strokeThickness: 3,
    }),
    new SciChart.PolarArcAnnotation({
        isEditable: true,
        x1: arcThresholds[0],
        x2: arcThresholds[1],

        y1: outerRadius,
        y2: innerRadius,

        fill: "rgba(250, 252, 90, 0.5)",
        stroke: "rgba(250, 252, 90, 1)",
        strokeThickness: 3,
    }),
    new SciChart.PolarArcAnnotation({
        isEditable: true,
        x1: arcThresholds[1],
        x2: arcThresholds[2],

        y1: outerRadius,
        y2: innerRadius,

        fill: "rgba(136, 242, 136, 0.5)",
        stroke: "rgba(136, 242, 136, 1)",
        strokeThickness: 3,
    })
);
```

Resulting in the following layout:

### More Tips:<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/multi-axis-and-layout/polar-chart-layout/#more-tips" class="hash-link" aria-label="Direct link to More Tips:" translate="no" title="Direct link to More Tips:">â€‹</a>

![](out_scichartv4/2d-charts/axis-api/multi-axis-and-layout/polar-chart-layout/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

If using <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarzoomextentsmodifier.html" rel="noopener noreferrer" target="_blank">PolarZoomExtentsModifierðŸ“˜</a> on a polar chart, you will likely want to set <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polaraxisbase.html#zoomextentstoinitialrange" rel="noopener noreferrer" target="_blank">zoomExtentsToInitialRangeðŸ“˜</a> to `true`, so that the zoom extents modifier will not try and squash the first and last data points together.

![](out_scichartv4/2d-charts/axis-api/multi-axis-and-layout/polar-chart-layout/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

If the gridlines are too often for your liking, you can either:

1.  Set <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polaraxisbase.html#drawminorgirdlines" rel="noopener noreferrer" target="_blank">drawMinorGridLinesðŸ“˜</a>: `false` on any axis
2.  Or set <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polaraxisbase.html#minorsPerMajor" rel="noopener noreferrer" target="_blank">minorsPerMajorðŸ“˜</a> to something lower than `5` (the default). Use `2` for a minimal look.

- You can also customize the `color`, `thickness` and `strokeDashArray` of both major and minor gridlines via <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polaraxisbase.html#majorgridlinestyle" rel="noopener noreferrer" target="_blank">MajorGridLineStyleðŸ“˜</a> and <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polaraxisbase.html#minorgridlinestyle" rel="noopener noreferrer" target="_blank">MinorGridLineStyleðŸ“˜</a>, both having the <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tgridlinestyle" rel="noopener noreferrer" target="_blank">TGridLineStyleðŸ“˜</a> type.

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/axis-api/multi-axis-and-layout/polar-chart-layout/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/axis-api/multi-axis-and-layout/polar-chart-layout/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
