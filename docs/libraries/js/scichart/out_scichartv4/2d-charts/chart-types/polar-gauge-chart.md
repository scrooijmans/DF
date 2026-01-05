On this page

# The Polar Gauge Chart Type

The Polar Gauge Chart is used to visualize data in a polar coordinate system as a circular gauge or meter, where values are represented by angular position and radial distance. This chart type is ideal for displaying progress indicators, performance metrics, or any data that benefits from a circular visualization.

![](out_scichartv4/2d-charts/chart-types/polar-gauge-chart/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

The <a href="https://www.scichart.com/demo/javascript/polar-gauge-chart" rel="noopener noreferrer" target="_blank">JavaScript Polar Gauge Chart</a> can be found in theÂ <a href="https://github.com/ABTSoftware/SciChart.JS.Examples/blob/release_v4.0/Examples/src/components/Examples/Charts2D/PolarCharts/PolarGaugeChart" rel="noopener noreferrer" target="_blank">SciChart.Js Examples Suite &gt; Polar Gauge Chart</a>Â on Github, or our live demo atÂ <a href="https://www.scichart.com/demo/react/polar-gauge-chart" rel="noopener noreferrer" target="_blank">scichart.com/demo</a>.

Above: The JavaScript <a href="https://www.scichart.com/demo/iframe/polar-gauge-chart" target="_blank">Polar Gauge Chart</a> example from the <a href="https://www.scichart.com/demo/react" target="_blank">SciChart.js Demo</a>

  

In SciChart.js, gauge charts are not specific renderable series, and can be created using either <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polararcannotation.html" rel="noopener noreferrer" target="_blank">PolarArcAnnotationðŸ“˜</a> for simple arc-based gauges or <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarcolumnrenderableseries.html" rel="noopener noreferrer" target="_blank">PolarColumnRenderableSeriesðŸ“˜</a> for column-based gauges. Both approaches offer different advantages depending on your specific use case.

## Properties<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-gauge-chart/#properties" class="hash-link" aria-label="Direct link to Properties" translate="no" title="Direct link to Properties">â€‹</a>

Polar gauge charts typically consist of:

- **Angular Axis** - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarnumericaxis.html" rel="noopener noreferrer" target="_blank">PolarNumericAxisðŸ“˜</a> / <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarcategoryaxis.html" rel="noopener noreferrer" target="_blank">PolarCategoryAxisðŸ“˜</a> with <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/epolaraxismode.html#angular" rel="noopener noreferrer" target="_blank">polarAxisMode.AngularðŸ“˜</a>, which controls the sweep angle and range of the gauge
- **Radial Axis** - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarnumericaxis.html" rel="noopener noreferrer" target="_blank">PolarNumericAxisðŸ“˜</a> / <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarcategoryaxis.html" rel="noopener noreferrer" target="_blank">PolarCategoryAxisðŸ“˜</a> with <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/epolaraxismode.html#radial" rel="noopener noreferrer" target="_blank">polarAxisMode.RadialðŸ“˜</a>, which defines the radial scale and range
- **Background Elements** - Arcs / Columns for optional visual indicators showing the full range as a gradient
- **Value Elements** - The actual data representation (arc or column)
- **Annotations** - Additional elements like pointers, centered dataLabels

## Examples<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-gauge-chart/#examples" class="hash-link" aria-label="Direct link to Examples" translate="no" title="Direct link to Examples">â€‹</a>

### Basic Gauge using PolarArcAnnotation<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-gauge-chart/#basic-gauge-using-polararcannotation" class="hash-link" aria-label="Direct link to Basic Gauge using PolarArcAnnotation" translate="no" title="Direct link to Basic Gauge using PolarArcAnnotation">â€‹</a>

The <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polararcannotation.html" rel="noopener noreferrer" target="_blank">PolarArcAnnotationðŸ“˜</a> approach is ideal for simple gauges with smooth arc representations:

``` prism-code
// Demonstrates how to create a gauge chart using ArcAnnotation & PolarPointerAnnotation using SciChart.js
const {
    SciChartPolarSurface,
    SciChartJsNavyTheme,
    NumberRange,
    PolarArcAnnotation,
    PolarNumericAxis,
    EPolarAxisMode,
} = SciChart;
// or, for npm, import { SciChartSurface, ... } from "scichart"

const { wasmContext, sciChartSurface } = await SciChartPolarSurface.create(divElementId, {
    theme: new SciChartJsNavyTheme(),
});

// The gauge angle
const gaugeTotalAngle = Math.PI * 1.2;
const gaugeRange = new NumberRange(0, 100); // the range of the gauge

// Add the axes
const angularXAxis = new PolarNumericAxis(wasmContext, {
    polarAxisMode: EPolarAxisMode.Angular,
    visibleRange: gaugeRange, // 0 - 100
    flippedCoordinates: true, // go clockwise
    totalAngle: gaugeTotalAngle,
    startAngle: (Math.PI - gaugeTotalAngle) / 2, // to center the bottom gap
    isVisible: false,
});
sciChartSurface.xAxes.add(angularXAxis);

const radialYAxis = new PolarNumericAxis(wasmContext, {
    polarAxisMode: EPolarAxisMode.Radial,
    visibleRange: new NumberRange(0, 10), // 0 - 10
    isVisible: false,
});
sciChartSurface.yAxes.add(radialYAxis);

// (optional) add a gray background Arc
const backgroundArc = new PolarArcAnnotation({
    y1: 10, // outer radius of the arc relative to the center of the gauge
    y2: 8, // inner radius of the arc

    x1: gaugeRange.min, // start angle -> 0
    x2: gaugeRange.max, // end angle -> 100

    fill: "#88888822",
    strokeThickness: 0
});

// The Value Arc
const valueArc = new PolarArcAnnotation({ 
    y1: 10, // outer radius
    y2: 8, // inner radius

    x1: gaugeRange.min, // start angle -> 0
    x2: 50 + Math.random() * 30, // current value (end of arc)

    fill: "#3388FF",
    stroke: "#FFFFFF",
    strokeThickness: 3
});

sciChartSurface.annotations.add(backgroundArc, valueArc);
```

In the code above:

- We configure the angular axis's <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarnumericaxis.html#totalangle" rel="noopener noreferrer" target="_blank">totalAngleðŸ“˜</a> property to define the gauge's sweep angle.
- Also calculate the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarnumericaxis.html#startangle" rel="noopener noreferrer" target="_blank">startAngleðŸ“˜</a> property to set the starting point of the gauge.
- Both axes have <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarnumericaxis.html#isvisible" rel="noopener noreferrer" target="_blank">isVisibleðŸ“˜</a> set to `false` to hide gridlines, ticks, and labels for a clean gauge appearance
- An optional gray **background arc** shows the full potential range of the gauge
- The **value arc** represents the current data value as such: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polararcannotation.html" rel="noopener noreferrer" target="_blank">PolarArcAnnotationðŸ“˜</a>'s `x1`, `x2` properties to set the start / end angle, and `y1`, `y2` properties to set the outer and inner radius (manipulating the arc thickness)

### Basic Gauge using PolarColumnRenderableSeries<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-gauge-chart/#basic-gauge-using-polarcolumnrenderableseries" class="hash-link" aria-label="Direct link to Basic Gauge using PolarColumnRenderableSeries" translate="no" title="Direct link to Basic Gauge using PolarColumnRenderableSeries">â€‹</a>

The <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarcolumnrenderableseries.html" rel="noopener noreferrer" target="_blank">PolarColumnRenderableSeriesðŸ“˜</a> approach offers more flexibility for data-driven gauges and gradient fills:

``` prism-code
// (optional) add a gray column to show the potential full range of the gauge
const grayColumn = new PolarColumnRenderableSeries(wasmContext, {
    dataSeries: new XyxyDataSeries(wasmContext, {
        y1Values: [10], // outer radius
        yValues: [8], // inner radius
        x1Values: [gaugeRange.max], // start of visible range -> 0
        xValues: [gaugeRange.min], // end of visible range -> 100
    }),
    columnXMode: EColumnMode.StartEnd,
    fill: "#88888844",
    strokeThickness: 0,
});
sciChartSurface.renderableSeries.add(grayColumn);

// add the value column:
const columnSeries = new PolarColumnRenderableSeries(wasmContext, {
    dataSeries: new XyxyDataSeries(wasmContext, {
        y1Values: [10], // outer radius
        yValues: [8], // inner radius
        x1Values: [gaugeRange.min], // start of the gauge -> 0
        xValues: [50 + Math.random() * 40], // current value of gauge
    }),
    columnXMode: EColumnMode.StartEnd,
    stroke: "#FFFFFF",
    strokeThickness: 3,

    fillLinearGradient: new GradientParams(
        new Point(0, 0), 
        new Point(1, 0), 
        [
            { offset: 0, color: "#AA2200" },
            { offset: 1, color: "#FFDD00" },
        ]
    ),
});
sciChartSurface.renderableSeries.add(columnSeries);
```

In the code above:

- We use <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyxydataseries.html" rel="noopener noreferrer" target="_blank">XyxyDataSeriesðŸ“˜</a> to define both the radial dimensions (`yValues`, `y1Values`) and angular range (`xValues`, `x1Values`)
- The <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarcolumnrenderableseries.html#columnxmode" rel="noopener noreferrer" target="_blank">columnXModeðŸ“˜</a> is set to <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/ecolumnmode.html#startend" rel="noopener noreferrer" target="_blank">EColumnMode.StartEndðŸ“˜</a> for precise control over the gauge arc
- This approach is better suited for gauges that need to display multiple data points or dynamic updates

### Advanced Multi-Threshold Gauge<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-gauge-chart/#advanced-multi-threshold-gauge" class="hash-link" aria-label="Direct link to Advanced Multi-Threshold Gauge" translate="no" title="Direct link to Advanced Multi-Threshold Gauge">â€‹</a>

For more complex gauge requirements with multiple thresholds, indicators, and styling:

``` prism-code
const GAUGE_THRESHOLDS = [50, 75, 100];
const GAUGE_COLORS = [ "#DD2222", "#FFCC22", "#22AA22"];

const GAUGE_VALUE = 50 + Math.random() * 30; // random val for the gauge

// add thin arcs, outlining the thresholds of the gauge
GAUGE_THRESHOLDS.forEach((threshold, i) => {
    sciChartSurface.annotations.add(
        new PolarArcAnnotation({ 
            y1: 10, // outer radius
            y2: 9.7, // inner radius
            x1: GAUGE_THRESHOLDS[i - 1] || gaugeRange.min, // start angle -> 0 or previous threshold
            x2: threshold, // end angle -> current threshold

            fill: GAUGE_COLORS[i],
            strokeThickness: 0
        })
    );
});

// (optional) gray background Arc
const backgroundArc = new PolarArcAnnotation({
    y1: 9.6, // outer radius
    y2: 7, // inner radius
    x1: gaugeRange.min, // start angle -> 0
    x2: gaugeRange.max, // end angle -> 100

    fill: "#88888822",
    strokeThickness: 0
});

// The Value Arc
const valueArc = new PolarArcAnnotation({ 
    y1: 9.3, // outer radius
    y2: 7, // inner radius
    x1: gaugeRange.min, // start angle -> 0
    x2: GAUGE_VALUE, // current value (end of arc)

    strokeThickness: 0,
    // smart fill color based on the threshold it's in
    fill: GAUGE_COLORS.find((_, i) => GAUGE_VALUE <= GAUGE_THRESHOLDS[i])
});

sciChartSurface.annotations.add(backgroundArc, valueArc);

// For more details, you can use either:
// 1. PolarPointerAnnotation to show the current value
const arrowPointer = new PolarPointerAnnotation({
    x1: GAUGE_VALUE, // pointer angle
    y1: 7, // pointer length
    xCoordinateMode: ECoordinateMode.DataValue,
    yCoordinateMode: ECoordinateMode.DataValue,

    pointerStyle: {
        baseSize: 0.03, 
        stroke: "#F00",
        fill: "#F00",
        backExtensionSize: 0.3, // how much the pointer extends back
    },

    pointerCenterStyle: {
        size: 0.2, // relative to the pointer length
        fill: "#111",
    }
});
sciChartSurface.annotations.add(arrowPointer);

// 2. TextAnnotation to show the value in the center of the gauge
const centeredText = new TextAnnotation({
    x1: 0,
    y1: 0, // centered at (0, 0)
    xCoordinateMode: ECoordinateMode.DataValue,
    yCoordinateMode: ECoordinateMode.DataValue,
    text: `${GAUGE_VALUE.toFixed(2)}%`,
    fontSize: 50,
    textColor: GAUGE_COLORS.find((_, i) => GAUGE_VALUE <= GAUGE_THRESHOLDS[i]), 
    horizontalAnchorPoint: EHorizontalAnchorPoint.Center,
    verticalAnchorPoint: EVerticalAnchorPoint.Center,
});
// sciChartSurface.annotations.add(centeredText); // uncomment to see it
```

In the code above:

- **Multiple threshold <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polararcannotation.html" rel="noopener noreferrer" target="_blank">arcsðŸ“˜</a>** are created using different colors to represent various performance zones
- **Smart color selection** automatically chooses the appropriate color based on the current value and thresholds
- A <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarpointerannotation.html" rel="noopener noreferrer" target="_blank">PolarPointerAnnotationðŸ“˜</a> provides a precise indicator of the current value
- Optional <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/textannotation.html" rel="noopener noreferrer" target="_blank">TextAnnotationðŸ“˜</a> or <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarpointerannotation.html" rel="noopener noreferrer" target="_blank">PolarPointerAnnotationðŸ“˜</a> can further highlight the numeric value in the center

## Choosing the Right Approach<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-gauge-chart/#choosing-the-right-approach" class="hash-link" aria-label="Direct link to Choosing the Right Approach" translate="no" title="Direct link to Choosing the Right Approach">â€‹</a>

**Use <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polararcannotation.html" rel="noopener noreferrer" target="_blank">PolarArcAnnotationðŸ“˜</a> when:**

- Creating simple, static gauges
- The gauge represents a single value
- Minimal code complexity is preferred

**Use <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarcolumnrenderableseries.html" rel="noopener noreferrer" target="_blank">PolarColumnRenderableSeriesðŸ“˜</a> when:**

- You need data-driven gauges
- Multiple data points need to be displayed
- Dynamic updates are required
- You want to leverage series-level features like <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarcolumnrenderableseries.html#filllineargradient" rel="noopener noreferrer" target="_blank">fillLinearGradientðŸ“˜</a>, <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarcolumnrenderableseries.html#paletteproviders" rel="noopener noreferrer" target="_blank">paletteProvidersðŸ“˜</a> or <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarcolumnrenderableseries.html#animation" rel="noopener noreferrer" target="_blank">animationðŸ“˜</a>

![](out_scichartv4/2d-charts/chart-types/polar-gauge-chart/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

Both approaches can be combined in the same chart to create sophisticated gauge visualizations with multiple indicators, thresholds, and interactive elements.

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-gauge-chart/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [Polar Pointer Annotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/polar-pointer-annotation)
- [Polar Arc Annotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/polar-arc-annotation)
- [Polar Column Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-column-renderable-series)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/polar-gauge-chart/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/polar-gauge-chart/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
