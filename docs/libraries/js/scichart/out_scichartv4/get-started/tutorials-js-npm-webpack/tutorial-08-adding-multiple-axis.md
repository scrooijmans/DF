On this page

# Tutorial 08 - Adding Multiple Axis

InÂ [Tutorial 7 - Adding Tooltips and Legends](https://www.scichart.com/documentation/js/v4/get-started/tutorials-js-npm-webpack/tutorial-07-adding-tooltips-and-legends), we showed you how to add tooltips and legends to a JavaScriptÂ Chart using SciChart.js. Now we are going to learn how to add a second **YAxis**.

SciChart supports unlimited axes. This unlocks different possibilities, such as **rotated (vertical)** charts.

![](out_scichartv4/get-started/tutorials-js-npm-webpack/tutorial-08-adding-multiple-axis/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

Source code for this tutorial can be found at <a href="https://github.com/ABTSoftware/SciChart.JS.Examples/tree/dev_v4.0/Tutorials/2D_Chart_Tutorials_JavaScript/Tutorial_8_Multiple_Axis" rel="noopener noreferrer" target="_blank">SciChart.JS.Examples Github Repository</a>

# Ein Fehler ist aufgetreten.

JavaScript kann nicht ausgeführt werden.

Video tutorial for version 3. SciChart.js JavaScript Chart Tutorial 08 - Adding Multiple X and Y Axis

  

In thisÂ tutorial, we are going to add a second **YAxis** to the chart. We are going to show how to register line series andÂ annotations on the second axis. We are also going to show how to ensure Axis drag behavior works on both axis.

## Adding a Second Y Axis<a href="https://www.scichart.com/documentation/js/v4/get-started/tutorials-js-npm-webpack/tutorial-08-adding-multiple-axis/#adding-a-second-y-axis" class="hash-link" aria-label="Direct link to Adding a Second Y Axis" translate="no" title="Direct link to Adding a Second Y Axis">â€‹</a>

The procedure to add a second axis to a **SciChartSurface** is pretty much the same as with one axis with one difference.

![](out_scichartv4/get-started/tutorials-js-npm-webpack/tutorial-08-adding-multiple-axis/index_media/0f0874621de662630fd5d7f09c89455c4ace26e6.svg)warning

Prior to version 4 you must assign a **unique string ID** to all axes viaÂ the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#id" rel="noopener noreferrer" target="_blank">axis.idðŸ“˜</a>Â propertyÂ if there is more than one. However, starting from version 4 axis IDs are assigned automatically.

To see the axis to appear to the either side of a chart, you setÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase2d.html#axisalignment" rel="noopener noreferrer" target="_blank">axis.axisAlignmentðŸ“˜</a> toÂ **EAxisAlignment.Left**, **EAxisAlignment.Right**, etc. <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eaxisalignment.html" rel="noopener noreferrer" target="_blank">EAxisAlignmentðŸ“˜</a>Â is anÂ enumeration.

Let's start by placing two Y-Axis on the left and right of the chart, and two X-Axis on the top and bottom.

- index.js
- html

``` prism-code
import {
  SciChartSurface,
  NumericAxis,
  EAxisAlignment,
  FastLineRenderableSeries,
  XyDataSeries,
  YAxisDragModifier,
  XAxisDragModifier,
  TextAnnotation,
} from "scichart";

async function initSciChart() {
  // #region ExampleA

  // Create the SciChartSurface in the div 'scichart-root'
  // The SciChartSurface, and webassembly context 'wasmContext' are paired. This wasmContext
  // instance must be passed to other types that exist on the same surface.
  const { sciChartSurface, wasmContext } = await SciChartSurface.create(
    "scichart-root"
  );

  // Create an X,Y Axis and add to the chart
  const xAxis = new NumericAxis(wasmContext, {
    axisTitle: "Primary XAxis",
    axisAlignment: EAxisAlignment.Bottom,
  });
  const xAxis2 = new NumericAxis(wasmContext, {
    axisTitle: "Secondary XAxis",
    axisAlignment: EAxisAlignment.Top,
  });
  const yAxis = new NumericAxis(wasmContext, {
    axisTitle: "Primary YAxis",
    axisAlignment: EAxisAlignment.Left,
  });
  const yAxis2 = new NumericAxis(wasmContext, {
    axisTitle: "Secondary YAxis",
    axisAlignment: EAxisAlignment.Right,
  });
  sciChartSurface.xAxes.add(xAxis);
  sciChartSurface.xAxes.add(xAxis2);
  sciChartSurface.yAxes.add(yAxis);
  sciChartSurface.yAxes.add(yAxis2);

  // #endregion
}

initSciChart();
```

``` prism-code
<html lang="en-us">
    <head>
        <meta charset="utf-8" />
        <meta content="text/html; charset=utf-8" http-equiv="Content-Type" />
        <title>SciChart.js Tutorial 8 - Multiple Axes</title>
        <script async type="text/javascript" src="bundle.js"></script>
        <style>
            body { font-family: 'Arial'}
        </style>
    </head>
    <body>
        <h1>Hello SciChart.js world!</h1>
        <p>In this example we add multiple X and Y axes and assign series, annotations to them</p>
        <!-- the Div where the SciChartSurface will reside -->
        <div id="scichart-root" style="width: 800px; height: 600px;"></div>
    </body>
</html>
```

Now we can see the primary and secondary X & Y axis in our application:

<img src="out_scichartv4/get-started/tutorials-js-npm-webpack/tutorial-08-adding-multiple-axis/index_media/8a7821ea8451d6c42f0d04e83e45e865cf8e0d86.png" class="img_ev3q" decoding="async" loading="lazy" width="1012" height="996" />

## Registering RenderableSeries on an Axis<a href="https://www.scichart.com/documentation/js/v4/get-started/tutorials-js-npm-webpack/tutorial-08-adding-multiple-axis/#registering-renderableseries-on-an-axis" class="hash-link" aria-label="Direct link to Registering RenderableSeries on an Axis" translate="no" title="Direct link to Registering RenderableSeries on an Axis">â€‹</a>

If there are several Y or X axes, you need to register other chart parts, like **RenderableSeries** and **Annotations**, on a particular axis to be measured against its scale.

We do this by setting the RenderableSeries.xAxisId and yAxisId properties. These must match the axis Id, which has been assigned automatically or set manually. If you leave an xAxisId/yAxisId unset, it will default to the first X/Y axis added to the surface.

- index.js region B

``` prism-code
  // Create first series and bind to the first X and Y axis
  const lineSeries1 = new FastLineRenderableSeries(wasmContext, {
    // If not set, xAxisId, yAxisId will default to the first X and Y axes added to sciChartSurface
    // therefore this series will bind to the Primary XAxis and YAxis
    stroke: "#33F9FF",
    strokeThickness: 6,
    dataSeries: new XyDataSeries(wasmContext, {
      xValues: [0, 1, 2, 3],
      yValues: [0, 60, 160, 300],
    }),
  });
  sciChartSurface.renderableSeries.add(lineSeries1);

  // Create second series and bind to the second Y axis
  const lineSeries2 = new FastLineRenderableSeries(wasmContext, {
    // Specify xAxisId, yAxisId.
    // Therefore this series will bind to the Secondary XAxis and YAxis
    xAxisId: xAxis2.id,
    yAxisId: yAxis2.id,
    stroke: "#33ff33",
    strokeThickness: 2,
    dataSeries: new XyDataSeries(wasmContext, {
      xValues: [0, 1, 2, 3, 4],
      yValues: [0, 101, 240, 500, 600],
    }),
  });
  sciChartSurface.renderableSeries.add(lineSeries2);
```

This results in the chart shown below, where the thicker blue line is bound to the primary axes:

<img src="out_scichartv4/get-started/tutorials-js-npm-webpack/tutorial-08-adding-multiple-axis/index_media/919d351f982bfcecb4a082c74b71a7e6f5654143.png" class="img_ev3q" decoding="async" loading="lazy" width="1012" height="996" />

## Registering Annotations on an Axis<a href="https://www.scichart.com/documentation/js/v4/get-started/tutorials-js-npm-webpack/tutorial-08-adding-multiple-axis/#registering-annotations-on-an-axis" class="hash-link" aria-label="Direct link to Registering Annotations on an Axis" translate="no" title="Direct link to Registering Annotations on an Axis">â€‹</a>

Annotations can also be added to a multi-axis chart, and registered with a specific axis pair. Add aÂ [TextAnnotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/text-annotation)Â as follows.

- index.js region C

``` prism-code
  sciChartSurface.annotations.add(
    new TextAnnotation({
      text: "Annotations on Axis!",
      x1: 2,
      y1: 400,
      // If not set, yAxisId, xAxisId will default to the first X and Y axes
      // This annotation will be bound to the Secondary XAxis and YAxis
      xAxisId: xAxis2.id,
      yAxisId: yAxis2.id,
      horizontalAnchorPoint: EHorizontalAnchorPoint.Center,
      verticalAnchorPoint: EVerticalAnchorPoint.Center
    })
  );
```

Now we can see aÂ [TextAnnotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/text-annotation) in theÂ middle of the chart, bound to the Secondary X,Y axis at `X,Y={2, 400}`

<img src="out_scichartv4/get-started/tutorials-js-npm-webpack/tutorial-08-adding-multiple-axis/index_media/ea1c37377e9d96df69a1fdc8293cc096084d76ff.png" class="img_ev3q" decoding="async" loading="lazy" width="1012" height="996" />

## Adding Axis Drag Behaviour<a href="https://www.scichart.com/documentation/js/v4/get-started/tutorials-js-npm-webpack/tutorial-08-adding-multiple-axis/#adding-axis-drag-behaviour" class="hash-link" aria-label="Direct link to Adding Axis Drag Behaviour" translate="no" title="Direct link to Adding Axis Drag Behaviour">â€‹</a>

If you want to visualize more clearly which series or annotation is bound to which axis pair, add some Axis Drag behaviours.

- index.js region D

``` prism-code
  sciChartSurface.chartModifiers.add(new XAxisDragModifier());
  sciChartSurface.chartModifiers.add(new YAxisDragModifier());
```

Now if we hover overÂ an Y Axis, click the mouse button and dragÂ the scaling occurs.

Moreover we can notice that the scaling only affects the series and annotations attached to that axis.

<img src="out_scichartv4/get-started/tutorials-js-npm-webpack/tutorial-08-adding-multiple-axis/index_media/3d746dc73fb7f11bc35fbae5dc4a4cbd3aa7b109.gif" class="img_ev3q" decoding="async" loading="lazy" width="1008" height="986" />

## Further Reading<a href="https://www.scichart.com/documentation/js/v4/get-started/tutorials-js-npm-webpack/tutorial-08-adding-multiple-axis/#further-reading" class="hash-link" aria-label="Direct link to Further Reading" translate="no" title="Direct link to Further Reading">â€‹</a>

Here is related documentation for further reading:Â Â Â 

- [Axis Types in SciChart.js](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-api-overview)
- [Secondary and Multiple Axis](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/multi-axis-and-layout/secondary-and-multiple-axis-overview)
- [Vertically Stacked Axis](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/multi-axis-and-layout/vertically-stacked-axis-layout)
- [Advanced Options - Custom Layouts](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/multi-axis-and-layout/advanced-options-custom-layout-managers)

#### See Also<a href="https://www.scichart.com/documentation/js/v4/get-started/tutorials-js-npm-webpack/tutorial-08-adding-multiple-axis/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [Tutorial 06 - Adding Annotations](https://www.scichart.com/documentation/js/v4/get-started/tutorials-js-npm-webpack/tutorial-06-adding-annotations)
- [Tutorial 09 - Linking Multiple Charts](https://www.scichart.com/documentation/js/v4/get-started/tutorials-js-npm-webpack/tutorial-09-linking-multiple-charts)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/get-started/tutorials-js-npm-webpack/tutorial-08-adding-multiple-axis/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/get-started/tutorials-js-npm-webpack/tutorial-08-adding-multiple-axis/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
