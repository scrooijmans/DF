On this page

# Tutorial 10 - Vertical Charts

In the [previous tutorial](https://www.scichart.com/documentation/js/v4/get-started/tutorials-js-npm-webpack/tutorial-09-linking-multiple-charts) we explained how to link multiple charts using SciChart.js. In this tutorial, we are going to show you how to add a **Vertical Chart**.

![](out_scichartv4/get-started/tutorials-js-npm-webpack/tutorial-10-vertical-charts/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

Source code for this tutorial can be found atÂ <a href="https://github.com/ABTSoftware/SciChart.JS.Examples/tree/dev_v4.0/Tutorials/2D_Chart_Tutorials_JavaScript/Tutorial_10_Vertical_Charts" rel="noopener noreferrer" target="_blank">SciChart.JS.Examples</a> Github Repository

## Adding a Chart<a href="https://www.scichart.com/documentation/js/v4/get-started/tutorials-js-npm-webpack/tutorial-10-vertical-charts/#adding-a-chart" class="hash-link" aria-label="Direct link to Adding a Chart" translate="no" title="Direct link to Adding a Chart">â€‹</a>

First let's create a simpleÂ [Line Chart](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-line-renderable-series). In this example we generate a data set for sine wave and add [SciChartSurface](https://www.scichart.com/documentation/js/v4/2d-charts/surface/scichart-surface-type-overview)Â with twoÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numericaxis.html" rel="noopener noreferrer" target="_blank">NumericAxisðŸ“˜</a> andÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastlinerenderableseries.html" rel="noopener noreferrer" target="_blank">FastLineRenderableSeriesðŸ“˜</a> to produce theÂ **Line Chart**.

- index.js
- index.html

``` prism-code
import {
  SciChartSurface,
  NumericAxis,
  NumberRange,
  XyDataSeries,
  FastLineRenderableSeries,
} from "scichart";

async function initSciChart() {
  const { wasmContext, sciChartSurface } = await SciChartSurface.create(
    "scichart-root"
  );

  // Generate a data set for sine wave
  const xValues = [];
  const yValues = [];
  for (let i = 0; i <= 100; i++) {
    const x = 0.1 * i;
    xValues.push(x);
    yValues.push(Math.sin(x));
  }

  sciChartSurface.xAxes.add(
    new NumericAxis(wasmContext, {
      axisTitle: "X Axis",
      growBy: new NumberRange(0.1, 0.1),
    })
  );

  sciChartSurface.yAxes.add(
    new NumericAxis(wasmContext, {
      axisTitle: "Y Axis",
      growBy: new NumberRange(0.1, 0.1),
    })
  );

  sciChartSurface.renderableSeries.add(
    new FastLineRenderableSeries(wasmContext, {
      dataSeries: new XyDataSeries(wasmContext, { xValues, yValues }),
      stroke: "orange",
    })
  );
}

initSciChart();
```

``` prism-code
<html lang="en-us">
  <head>
    <meta charset="utf-8" />
    <meta content="text/html; charset=utf-8" http-equiv="Content-Type" />
    <title>SciChart.js Tutorial 10 - Vertical Charts</title>
    <script async type="text/javascript" src="bundle.js"></script>
    <style>
      body {
        font-family: "Arial";
      }
    </style>
  </head>
  <body>
    <h1>Hello SciChart.js world!</h1>
    <p>In this example we setup a basic chart.</p>
    <p>
      In part2 we are going to make the chart render vertically (rotated 90
      degrees)
    </p>
    <!-- the Div where SciChartSurface will reside -->
    <div id="scichart-root" style="width: 800px; height: 600px"></div>
  </body>
</html>
```

This gives us this chart:

<img src="out_scichartv4/get-started/tutorials-js-npm-webpack/tutorial-10-vertical-charts/index_media/5e3280639a4ad95167c799a7d4d78538ab329e2a.png" class="img_ev3q" decoding="async" loading="lazy" width="1037" height="1020" />

## Making the Chart Vertical<a href="https://www.scichart.com/documentation/js/v4/get-started/tutorials-js-npm-webpack/tutorial-10-vertical-charts/#making-the-chart-vertical" class="hash-link" aria-label="Direct link to Making the Chart Vertical" translate="no" title="Direct link to Making the Chart Vertical">â€‹</a>

In fact, just setting <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase2d.html#axisalignment" rel="noopener noreferrer" target="_blank">xAxis.axisAlignmentðŸ“˜</a> toÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eaxisalignment.html" rel="noopener noreferrer" target="_blank">EAxisAlignment.LeftðŸ“˜</a> or Right andÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase2d.html#axisalignment" rel="noopener noreferrer" target="_blank">yAxis.axisAlignmentðŸ“˜</a> toÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eaxisalignment.html" rel="noopener noreferrer" target="_blank">EAxisAlignment.TopðŸ“˜</a> or EAxisAlignment.Bottom makes the chart vertical.

Moreover if the axes orientation needs to be inverted there isÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase2d.html#flippedcoordinates" rel="noopener noreferrer" target="_blank">axis.flippedCoordinatesðŸ“˜</a> property onÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html" rel="noopener noreferrer" target="_blank">AxisCoreðŸ“˜</a>Â class.Â 

In order to make the chart interactiveÂ weÂ add chart modifiers such asÂ [ZoomPanModifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/zooming-and-panning/zoom-pan-modifier),Â [ZoomExtentsModifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/zooming-and-panning/zoom-extents-modifier),Â [MouseWheelZoomModifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/zooming-and-panning/mouse-wheel-zoom-modifier) andÂ [RolloverModifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/rollover-modifier).

- Part2 index.js
- Part2 index.html

``` prism-code
import {
  SciChartSurface,
  NumericAxis,
  NumberRange,
  XyDataSeries,
  FastLineRenderableSeries,
  EAxisAlignment,
  ZoomPanModifier,
  ZoomExtentsModifier,
  MouseWheelZoomModifier,
  RolloverModifier
} from "scichart";

async function initSciChart() {
  const { wasmContext, sciChartSurface } = await SciChartSurface.create(
    "scichart-root"
  );

  // Generate a data set for sine wave
  const xValues = [];
  const yValues = [];
  for (let i = 0; i <= 100; i++) {
    const x = 0.1 * i;
    xValues.push(x);
    yValues.push(Math.sin(x));
  }

  // Make the chart vertical by setting X Axis Alignment to Left or Right
  sciChartSurface.xAxes.add(
    new NumericAxis(wasmContext, {
      axisTitle: "X Axis",
      growBy: new NumberRange(0.1, 0.1),
      axisAlignment: EAxisAlignment.Left
    })
  );

  // Make the chart vertical by setting Y Axis Alignment to Top or Bottom
  sciChartSurface.yAxes.add(
    new NumericAxis(wasmContext, {
      axisTitle: "Y Axis",
      growBy: new NumberRange(0.1, 0.1),
      axisAlignment: EAxisAlignment.Top,
      // Flip the axis orientation with this property
      flippedCoordinates: true,
    })
  );

  sciChartSurface.renderableSeries.add(
    new FastLineRenderableSeries(wasmContext, {
      dataSeries: new XyDataSeries(wasmContext, { xValues, yValues }),
      stroke: "orange",
    })
  );

  // Add some interactivity modifiers. Everything is transposed in a vertical chart
  // so zoom/pan and rollover works vertically
  sciChartSurface.chartModifiers.add(new ZoomPanModifier());
  sciChartSurface.chartModifiers.add(new ZoomExtentsModifier());
  sciChartSurface.chartModifiers.add(new MouseWheelZoomModifier());
  sciChartSurface.chartModifiers.add(new RolloverModifier());
}

initSciChart();
```

``` prism-code
<html lang="en-us">
  <head>
    <meta charset="utf-8" />
    <meta content="text/html; charset=utf-8" http-equiv="Content-Type" />
    <title>SciChart.js Tutorial 10 - Vertical Charts</title>
    <script async type="text/javascript" src="bundle.js"></script>
    <style>
      body {
        font-family: "Arial";
      }
    </style>
  </head>
  <body>
    <h1>Hello SciChart.js world!</h1>
    <p>In this example we create vertical charts (Part2)</p>
    <!-- the Div where SciChartSurface will reside -->
    <div id="scichart-root" style="width: 800px; height: 600px"></div>
  </body>
</html>
```

Â This gives us the **Vertical Chart**:

<img src="out_scichartv4/get-started/tutorials-js-npm-webpack/tutorial-10-vertical-charts/index_media/3283aeaa582e20cfdaf329798292474177c8c8c7.png" class="img_ev3q" decoding="async" loading="lazy" width="1022" height="995" />

## Further Reading<a href="https://www.scichart.com/documentation/js/v4/get-started/tutorials-js-npm-webpack/tutorial-10-vertical-charts/#further-reading" class="hash-link" aria-label="Direct link to Further Reading" translate="no" title="Direct link to Further Reading">â€‹</a>

Here is related documentation for further reading:Â Â Â 

- [Axis Types in SciChart.js](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-types/common-axis-base-type)
- [Axis Docs - Create a Vertical Chart](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/multi-axis-and-layout/vertical-charts-rotate-transpose-axis)
- [Synchronizing Vertical Charts](https://www.scichart.com/documentation/js/v4/2d-charts/chart-synchronization-api/synchronizing-vertical-charts)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/get-started/tutorials-js-npm-webpack/tutorial-10-vertical-charts/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/get-started/tutorials-js-npm-webpack/tutorial-10-vertical-charts/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
