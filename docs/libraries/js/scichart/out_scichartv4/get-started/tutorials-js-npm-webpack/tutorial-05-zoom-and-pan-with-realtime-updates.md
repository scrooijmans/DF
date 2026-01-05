On this page

# Tutorial 05 - Zoom and Pan with Realtime Updates

InÂ [Tutorial 04 - Adding Realtime Updates](https://www.scichart.com/documentation/js/v4/get-started/tutorials-js-npm-webpack/tutorial-04-adding-realtime-updates), we showed you how to dynamically update DataSeries to enable Real-time updates in SciChart.js. In this tutorial, were going to show you how to allow zooming and panning while scrolling data.

If you haven't read it already, also check outÂ [Tutorial 03 - Adding Zooming Panning Behavior](https://www.scichart.com/documentation/js/v4/get-started/tutorials-js-npm-webpack/tutorial-03-adding-zooming-panning-behavior) as we will assume you have the knowledge to add zoom and pan behaviors to a SciChart.js JavaScript chart.

![](out_scichartv4/get-started/tutorials-js-npm-webpack/tutorial-05-zoom-and-pan-with-realtime-updates/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

The source code for this tutorial can be found atÂ <a href="https://github.com/ABTSoftware/SciChart.JS.Examples/tree/dev_v4.0/Tutorials/2D_Chart_Tutorials_JavaScript/Tutorial_5_Zoom_and_Pan_With_Realtime_Updates" rel="noopener noreferrer" target="_blank">SciChart.JS.Examples Github Repository</a>

# Ein Fehler ist aufgetreten.

JavaScript kann nicht ausgeführt werden.

Video tutorial for version 3. SciChart.js JavaScript Chart Tutorial 05 - Zooming, Panning a Realtime Chart

  

## Creating the Base Application<a href="https://www.scichart.com/documentation/js/v4/get-started/tutorials-js-npm-webpack/tutorial-05-zoom-and-pan-with-realtime-updates/#creating-the-base-application" class="hash-link" aria-label="Direct link to Creating the Base Application" translate="no" title="Direct link to Creating the Base Application">â€‹</a>

We're going to start off with the code we created in the previousÂ [Tutorial 04 - Adding Realtime Updates](https://www.scichart.com/documentation/js/v4/get-started/tutorials-js-npm-webpack/tutorial-04-adding-realtime-updates). If you haven't already started that tutorial, please run through it first so you can understand the concepts.

**Start with this code to begin with**. This will create a real-time chart with scrolling data, **but no zooming or panning yet.**

- index.js
- index.html

``` prism-code
import {
    SciChartSurface,
    NumericAxis,
    XyDataSeries,
    FastLineRenderableSeries,
    XyScatterRenderableSeries,
    EllipsePointMarker,
    NumberRange
} from "scichart";

async function initSciChart() {
    // Create the SciChartSurface in the div 'scichart-root'
    // The SciChartSurface, and webassembly context 'wasmContext' are paired. This wasmContext
    // instance must be passed to other types that exist on the same surface.
    const {sciChartSurface, wasmContext} = await SciChartSurface.create("scichart-root");

    // Create an X,Y Axis and add to the chart
    sciChartSurface.xAxes.add(new NumericAxis(wasmContext));
    sciChartSurface.yAxes.add(new NumericAxis(wasmContext));

    // Create a Scatter series, and Line series and add to chart
    const scatterSeries = new XyScatterRenderableSeries(wasmContext, {
        pointMarker: new EllipsePointMarker(wasmContext, { width: 7, height: 7, fill: "White", stroke: "SteelBlue" }),
    });
    const lineSeries = new FastLineRenderableSeries(wasmContext, { stroke: "#4083B7", strokeThickness: 2 });
    sciChartSurface.renderableSeries.add(lineSeries, scatterSeries);

    // Create and populate some XyDataSeries with static data
    // Note: you can pass xValues, yValues arrays to constructors, and you can use appendRange for bigger datasets
    const scatterData = new XyDataSeries(wasmContext, { dataSeriesName: "Cos(x)" });
    const lineData = new XyDataSeries(wasmContext, { dataSeriesName: "Sin(x)" });

    for(let i = 0; i < 1000; i++) {
        lineData.append(i, Math.sin(i*0.1));
        scatterData.append(i, Math.cos(i*0.1));
    }

    // Assign these dataseries to the line/scatter renderableseries
    scatterSeries.dataSeries = scatterData;
    lineSeries.dataSeries = lineData;

    // SciChart will now redraw with static data
    //

    // #region ExampleA
    // Scrolling the chart by appending and manipulating xAxis.visibleRange

    const updateDataFunc = () => {

        // Append another data-point to the chart. We use dataSeries.count()
        // to determine the current length before appending
        const i = lineData.count();
        lineData.append(i, Math.sin(i * 0.1));
        scatterData.append(i, Math.cos(i * 0.1));

        // Apply scrolling to the chart by updating xAxis.visibleRange
        // Also see dataSeries.fifoCapacity and dataSeries.fifoSweeping for more options
        const xAxis = sciChartSurface.xAxes.get(0);
        xAxis.visibleRange = new NumberRange(i-1000, i);
    };

    // Repeat at 60Hz
    setInterval(updateDataFunc, 1000/60);
    // #endregion
}

initSciChart();
```

``` prism-code
<html lang="en-us">
<head>
    <meta charset="utf-8" />
    <meta content="text/html; charset=utf-8" http-equiv="Content-Type" />
    <link rel="icon" href="data:," />
    <title>SciChart.js Tutorial 4 - Part 3, Realtime Scrolling</title>
    <script async type="text/javascript" src="bundle.js"></script>
    <style>
    body {
        font-family: "Arial";
    }
    </style>
</head>
<body>
    <h1>Hello SciChart.js world!</h1>
    <p>
    In this example we explore real-time updates by scrolling data in
    SciChart.js
    </p>

    <!-- the Div where the SciChartSurface will reside -->
    <div id="scichart-root" style="width: 800px; height: 600px"></div>
</body>
</html>
```

## Adding Zooming Behavior<a href="https://www.scichart.com/documentation/js/v4/get-started/tutorials-js-npm-webpack/tutorial-05-zoom-and-pan-with-realtime-updates/#adding-zooming-behavior" class="hash-link" aria-label="Direct link to Adding Zooming Behavior" translate="no" title="Direct link to Adding Zooming Behavior">â€‹</a>

FromÂ [Tutorial 03 - Adding Zooming, Panning Behavior](https://www.scichart.com/documentation/js/v4/get-started/tutorials-js-npm-webpack/tutorial-03-adding-zooming-panning-behavior), we learned that we can add ChartModifiers to theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#chartmodifiers" rel="noopener noreferrer" target="_blank">sciChartSurface.chartModifiersðŸ“˜</a> collection to add specific zoom, or pan behaviors to the chart.

However, the code we added to scroll the chart on update is going to conflict with the user mouse-zooming behaviors. Take a look below:

- Zooming and Panning Step 1

``` prism-code
  ...
  import {
      RubberBandXyZoomModifier,
      ZoomExtentsModifier,
  } from "scichart";

  async function initSciChart() {

      // Add this code to enable zooming by mouse-drag and double-click to zoom extents
      //
      sciChartSurface.chartModifiers.add(new ZoomExtentsModifier({isAnimated: false}));
      sciChartSurface.chartModifiers.add(new RubberBandXyZoomModifier());
      ...
      const updateDataFunc = () => {
          // Append another data-point to the chart. We use dataSeries.count()
          // to determine the current length before appending
          const i = lineData.count();
          lineData.append(i, Math.sin(i * 0.1));
          scatterData.append(i, Math.cos(i * 0.1));
          // However, user-zoom will conflict with this code which scrolls the chart on update
          xAxis.visibleRange = new NumberRange(i-1000, i);
      };
      // Repeat at 60Hz
      setInterval(updateDataFunc, 1000 / 60);
      ...
  }
  initSciChart();
```

If we want to enable user-zoom, and also scroll the chart, we need to selectively implement that scroll. To do so we can use theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#zoomstate" rel="noopener noreferrer" target="_blank">sciChartSurface.zoomStateðŸ“˜</a> property.

## The sciChartSurface.zoomState Property<a href="https://www.scichart.com/documentation/js/v4/get-started/tutorials-js-npm-webpack/tutorial-05-zoom-and-pan-with-realtime-updates/#the-scichartsurfacezoomstate-property" class="hash-link" aria-label="Direct link to The sciChartSurface.zoomState Property" translate="no" title="Direct link to The sciChartSurface.zoomState Property">â€‹</a>

TheÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#zoomstate" rel="noopener noreferrer" target="_blank">sciChartSurface.zoomStateðŸ“˜</a> property allows us to detect if the chart has been zoomed or panned by the user, or if the chart is at extents of the data. You can take a look at the values of theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/ezoomstate.html" rel="noopener noreferrer" target="_blank">EZoomState Enum hereðŸ“˜</a>.

If we modified our code, we can selectively use this property to detect if the user is zooming and halt any automatic scrolling. For example, try modifying the updateDataFunc as follows:

- Part1/index.js

``` prism-code
// import {
//   ZoomExtentsModifier, RubberBandZoomModifier,
// } from "scichart";

// Add ZoomExtentsModifier and disable extents animation
sciChartSurface.chartModifiers.add(
  new ZoomExtentsModifier({ isAnimated: false })
);
// Add RubberBandZoomModifier
sciChartSurface.chartModifiers.add(new RubberBandXyZoomModifier());

// Part 2: Appending data in realtime
//
const updateDataFunc = () => {
  // Append another data-point to the chart. We use dataSeries.count()
  // to determine the current length before appending
  const i = lineData.count();
  lineData.append(i, Math.sin(i * 0.1));
  scatterData.append(i, Math.cos(i * 0.1));

  // import { ZoomState } from "scichart";
  //
  // Using zoomState, we only scroll if the state is not userZooming
  // This property is set internally whenever the user mouse-down drags on the chart or
  // performs a zoom operation, and can be used to selectively enable or disable scrolling
  if (sciChartSurface.zoomState !== EZoomState.UserZooming) {
    xAxis.visibleRange = new NumberRange(i - 1000, i);
  }
};

// Repeat at 60Hz
setInterval(updateDataFunc, 1000 / 60);
```

Now run the application again, click leftÂ mouse button and move itÂ to select an area. After releasing the button the chart will be zoomed in. To resume realtime updates perform double click.

## Adding Panning Behavior to a Realtime Chart<a href="https://www.scichart.com/documentation/js/v4/get-started/tutorials-js-npm-webpack/tutorial-05-zoom-and-pan-with-realtime-updates/#adding-panning-behavior-to-a-realtime-chart" class="hash-link" aria-label="Direct link to Adding Panning Behavior to a Realtime Chart" translate="no" title="Direct link to Adding Panning Behavior to a Realtime Chart">â€‹</a>

In order to addÂ [ZoomPanModifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/zooming-and-panning/zoom-pan-modifier), update the code as follows.Â Don't forget to include the same ZoomState logic as we had before.

- Part2/index.js

``` prism-code
// import {
//   ZoomExtentsModifier, RubberBandZoomModifier,
//   ZoomPanModifier, EExecuteOn, EModifierMouseArgKey
// } from "scichart";

// Add ZoomExtentsModifier and RubberBandZoomModifier
sciChartSurface.chartModifiers.add(
  new ZoomExtentsModifier({ isAnimated: false })
);
sciChartSurface.chartModifiers.add(new RubberBandXyZoomModifier());

// Add ZoomPanModifier enabled on right-mouse button
sciChartSurface.chartModifiers.add(
  new ZoomPanModifier({
    executeCondition: { button: EExecuteOn.MouseRightButton, key: EModifierMouseArgKey.None }
  })
);

// Part 2: Appending data in realtime
//
const updateDataFunc = () => {
  // Append another data-point to the chart. We use dataSeries.count()
  // to determine the current length before appending
  const i = lineData.count();
  lineData.append(i, Math.sin(i * 0.1));
  scatterData.append(i, Math.cos(i * 0.1));

  // Prevent changing visibleRange if user is zooming or panning
  if (sciChartSurface.zoomState !== EZoomState.UserZooming) {
    xAxis.visibleRange = new NumberRange(i - 1000, i);
  }
};
```

![](out_scichartv4/get-started/tutorials-js-npm-webpack/tutorial-05-zoom-and-pan-with-realtime-updates/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

You can use <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/chartmodifierbase.html#executecondition" rel="noopener noreferrer" target="_blank">ChartModifierBase.executeConditionðŸ“˜</a> to make it work with different mouse button or mouse button + Ctrl/Alt/Shift button. Like we did in the example above

``` prism-code
new ZoomPanModifier({
    executeCondition: { button: EExecuteOn.MouseRightButton, key: EModifierMouseArgKey.None }
});
```

Now run the application again,Â left click the chartÂ and move the mouse. As a resultÂ theÂ chart will moving with the mouse. To pan the chart, use the right mouse button. To resume realtime updates perform double click.

## Further Examples - the Realtime Ticking Stock Chart demo<a href="https://www.scichart.com/documentation/js/v4/get-started/tutorials-js-npm-webpack/tutorial-05-zoom-and-pan-with-realtime-updates/#further-examples---the-realtime-ticking-stock-chart-demo" class="hash-link" aria-label="Direct link to Further Examples - the Realtime Ticking Stock Chart demo" translate="no" title="Direct link to Further Examples - the Realtime Ticking Stock Chart demo">â€‹</a>

In theÂ <a href="https://www.scichart.com/demo" rel="noopener noreferrer" target="_blank">SciChart.js Examples Suite - viewable at scichart.com/demo</a>, we have an example of realtime updates with zooming & panning built into the chart. This is theÂ <a href="https://www.scichart.com/demo/react/realtime-ticking-stock-charts" rel="noopener noreferrer" target="_blank">JavaScript Realtime Ticking Stock Charts example</a>.

Above: The JavaScript <a href="https://www.scichart.com/demo/iframe/realtime-ticking-stock-charts" target="_blank">Realtime Ticking Stock Charts</a> example from the <a href="https://www.scichart.com/demo/react" target="_blank">SciChart.js Demo</a> showing how to combine zooming, panning with a realtime updated chart.

  

In this exampleÂ as newÂ candle data is added, the chart advances by 1 to automatically keep the new candle in the same place in the viewport. However, if you scroll back in time so that the latest candle is out of the viewport, the advancing by 1 does not occur.

To achieve this, weÂ use techniques similar to the above tutorial to selectively advance the chart by one candle **only if the latest data-point is inside the viewport**.

This allows you to create an intuitive user zooming, panning experience while advancing the chart or scrolling as new data comes in.

- TS

``` prism-code
  // Is the latest candle in the viewport?
  if (xAxis.visibleRange.max > getLatestCandleDate) {
      // If so, shift the xAxis by one candle
      const dateDifference = priceBar.date / 1000 - getLatestCandleDate;
      const shiftedRange = new NumberRange(
          xAxis.visibleRange.min + dateDifference,
          xAxis.visibleRange.max + dateDifference
      );
      xAxis.animateVisibleRange(shiftedRange, 250, easing.inOutQuad);
  }
```

And the Source-code for how we achieved it at our Github repository,Â <a href="https://github.com/ABTSoftware/SciChart.JS.Examples/blob/dev_v4.0/Examples/src/components/Examples/Charts2D/CreateStockCharts/RealtimeTickingStockCharts/createCandlestickChart.ts" rel="noopener noreferrer" target="_blank">in file createCandlestickChart.ts</a>, function **onNewTrade()**.

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/get-started/tutorials-js-npm-webpack/tutorial-05-zoom-and-pan-with-realtime-updates/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/get-started/tutorials-js-npm-webpack/tutorial-05-zoom-and-pan-with-realtime-updates/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
