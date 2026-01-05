On this page

# Tutorial 04 - Adding Realtime Updates

In the [previous tutorial](https://www.scichart.com/documentation/js/v4/get-started/tutorials-js-npm-webpack/tutorial-03-adding-zooming-panning-behavior) we explained how to add zooming and panning behavior to a JavaScript Chart using SciChart.js. In this tutorial, we are going to show you how to add realtime updates.

Realtime updates is where SciChart excels. Our software is designed to be ultra high-performance and really shines in circumstances where you want to manipulate data in real-time. There are a lot of options for dynamically updating data and getting rich, reactive charts. If you're interested to find out how, read on!

![](out_scichartv4/get-started/tutorials-js-npm-webpack/tutorial-04-adding-realtime-updates/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

The source code for this tutorial can be found atÂ <a href="https://github.com/ABTSoftware/SciChart.JS.Examples/tree/dev_v4.0/Tutorials/2D_Chart_Tutorials_JavaScript/Tutorial_4_Adding_Realtime_Updates" rel="noopener noreferrer" target="_blank">SciChart.Js.Examples Github Repository</a>Â Â 

# Ein Fehler ist aufgetreten.

JavaScript kann nicht ausgeführt werden.

Video tutorial for version 3. SciChart.js JavaScript Chart Tutorial 04 - Realtime Chart Updates

  

## Creating the Base Application<a href="https://www.scichart.com/documentation/js/v4/get-started/tutorials-js-npm-webpack/tutorial-04-adding-realtime-updates/#creating-the-base-application" class="hash-link" aria-label="Direct link to Creating the Base Application" translate="no" title="Direct link to Creating the Base Application">â€‹</a>

SciChart has the concept ofÂ [RenderableSeries](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/renderable-series-api-overview) andÂ [DataSeries](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-series-api/data-series-api-overview). RenderableSeries present the data, while DataSeries hold the X,Y data and manage updates.

Let's start the tutorial with some basic code like this to declare a SciChartSurface with an X,Y axis, and a two series:

- index.js
- index.html

``` prism-code
import {
    SciChartSurface,
    NumericAxis,
    XyDataSeries,
    FastLineRenderableSeries,
    XyScatterRenderableSeries,
    EllipsePointMarker
} from "scichart";

async function initSciChart() {
    // Create the SciChartSurface in the div 'scichart-root'
    // The SciChartSurface, and webassembly context 'wasmContext' are paired. This wasmContext
    // instance must be passed to other types that exist on the same surface.
    const { sciChartSurface, wasmContext} = await SciChartSurface.create("scichart-root");

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
}

initSciChart();
```

``` prism-code
<html lang="en-us">
    <head>
        <meta charset="utf-8" />
        <meta content="text/html; charset=utf-8" http-equiv="Content-Type" />
        <link rel="icon" href="data:," />
        <title>SciChart.js Tutorial 4 - Adding Realtime Updates</title>
        <script async type="text/javascript" src="bundle.js"></script>
        <style>
            body { font-family: 'Arial'}
        </style>
    </head>
    <body>
        <h1>Hello SciChart.js world!</h1>
        <p>In this example we explore charts with real-time updates in SciChart.js</p>

        <!-- the Div where the SciChartSurface will reside -->
        <div id="scichart-root" style="width: 800px; height: 600px;"></div>
    </body>
</html>
```

This code above creates aÂ [Scatter series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/xy-scatter-renderable-series), a [Line series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-line-renderable-series), appends some static data. You should now be looking at this if you npm start and run the application:

<img src="out_scichartv4/get-started/tutorials-js-npm-webpack/tutorial-04-adding-realtime-updates/index_media/0325f926567a5e6a82fab4cea147c07c3adcdd07.png" class="img_ev3q" decoding="async" loading="lazy" width="818" height="807" />

You can learn more about theÂ [Scatter Series, which requires a pointmarker here](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/xy-scatter-renderable-series). Also theÂ [Line Series page](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-line-renderable-series) has further information for this chart type.

## Part 1 - Updating Data Values<a href="https://www.scichart.com/documentation/js/v4/get-started/tutorials-js-npm-webpack/tutorial-04-adding-realtime-updates/#part-1---updating-data-values" class="hash-link" aria-label="Direct link to Part 1 - Updating Data Values" translate="no" title="Direct link to Part 1 - Updating Data Values">â€‹</a>

Now let's update some data-values in JavaScriptÂ to see real-time changes to the chart.

We will use setTimeout to create a timer and callÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#update" rel="noopener noreferrer" target="_blank">xyDataSeries.update()ðŸ“˜</a> to change the data. SciChart automatically redraws when the data is changed.

- index.js

``` prism-code
let phase = 0.0;

const updateDataFunc = () => {

    // update the datapoints in the dataseries
    // Note dataSeries.updateRange() passing in array is
    // higher performance for larger datasets vs. calling dataSeries.update() in a loop
    for(let i = 0; i < 1000; i++) {
        lineData.update(i, Math.sin(i * 0.1 + phase));
        scatterData.update(i, Math.cos(i * 0.1 + phase));
    }

    phase += 0.01;
};

// Update data at 60Hz
setInterval(updateDataFunc, 1000/60);
```

Breaking this down, we're using theÂ [DataSeries API](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-series-api/data-series-api-overview) to manipulate the data.

Specifically,Â <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#update" rel="noopener noreferrer" target="_blank">dataSeries.update()ðŸ“˜</a>, which updates a Y-value at a specific index. There are also functions toÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#append" rel="noopener noreferrer" target="_blank">appendðŸ“˜</a>,Â <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#appendrange" rel="noopener noreferrer" target="_blank">appendRangeðŸ“˜</a>,Â <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#insert" rel="noopener noreferrer" target="_blank">insertðŸ“˜</a>,Â <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#insertrange" rel="noopener noreferrer" target="_blank">insertRangeðŸ“˜</a>,Â <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#removeat" rel="noopener noreferrer" target="_blank">removeðŸ“˜</a>,Â <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#removerange" rel="noopener noreferrer" target="_blank">removeRangeðŸ“˜</a> andÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#clear" rel="noopener noreferrer" target="_blank">clearðŸ“˜</a>, which will remove all data. Any changes to the DataSeries immediately reflect in the chart as SciChart is a fully reactive, dynamic JavaScript chart component.

You can learn more about theÂ [dynamic DataSeries API in SciChart here](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-series-api/data-series-api-overview).

Now run the application, you should see this!

<img src="out_scichartv4/get-started/tutorials-js-npm-webpack/tutorial-04-adding-realtime-updates/index_media/c640d01143dbe77735c143fa5735ebaef04ac6cc.gif" class="img_ev3q" decoding="async" loading="lazy" width="804" height="779" />

## Part 2 - Appending Data Values<a href="https://www.scichart.com/documentation/js/v4/get-started/tutorials-js-npm-webpack/tutorial-04-adding-realtime-updates/#part-2---appending-data-values" class="hash-link" aria-label="Direct link to Part 2 - Appending Data Values" translate="no" title="Direct link to Part 2 - Appending Data Values">â€‹</a>

As well as using dataSeries.update(), you can also useÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#append" rel="noopener noreferrer" target="_blank">dataSeries.append()ðŸ“˜</a> to add new data-values to the end of a DataSeries. Make some changes in your updateDataFunc() as follows:

- index.js

``` prism-code
  // Update the updateDataFunc() to append a point to the end of the data series

  const updateDataFunc = () => {

      // Append another data-point to the chart. We use dataSeries.count()
      // to determine the current length before appending
      const i = lineData.count();
      lineData.append(i, Math.sin(i * 0.1));
      scatterData.append(i, Math.cos(i * 0.1));

      // ZoomExtents after appending data.
      // Also see XAxis.AutoRange, and XAxis.VisibleRange for more options
      sciChartSurface.zoomExtents();
  };

  // Repeat at 60Hz
  setInterval(updateDataFunc, 1000/60);
```

Now run the application again. You sould see the series growing larger as new data is appended.

<img src="out_scichartv4/get-started/tutorials-js-npm-webpack/tutorial-04-adding-realtime-updates/index_media/c94e05321d998be545790003d76dc9bd296ba829.gif" class="img_ev3q" decoding="async" loading="lazy" width="806" height="788" />

![](out_scichartv4/get-started/tutorials-js-npm-webpack/tutorial-04-adding-realtime-updates/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

By default, when you append data, SciChart does not resize the viewport to fit the new data. In the code sample above we have calledÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#zoomextents" rel="noopener noreferrer" target="_blank">sciChartSurface.zoomExtents()ðŸ“˜</a> to fit the data.

Other options to manipulate the viewportÂ can be seen at the pagesÂ [Axis Ranging - AutoRange](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/ranging-scaling/auto-range) andÂ [Axis Ranging - Setting and Getting VisibleRange](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/ranging-scaling/set-range-zoom-to-fit).

## Part 3 - Scrolling Realtime Charts<a href="https://www.scichart.com/documentation/js/v4/get-started/tutorials-js-npm-webpack/tutorial-04-adding-realtime-updates/#part-3---scrolling-realtime-charts" class="hash-link" aria-label="Direct link to Part 3 - Scrolling Realtime Charts" translate="no" title="Direct link to Part 3 - Scrolling Realtime Charts">â€‹</a>

What if you wanted to scroll as new data was appended? You have a few choices.

- If you simply want to scroll the viewport, you can manipulateÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#visiblerange" rel="noopener noreferrer" target="_blank">XAxis.visibleRangeðŸ“˜</a> (seeÂ [Getting and Setting VisibleRange](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/ranging-scaling/set-range-zoom-to-fit))Â as you append new data.
- If you want to be memory efficient and discard old data, you can use dataSeries.removeAt, or dataSeries.removeRange to remove old values as well.

We're going to show you how to scroll data in a JavaScript chart below.

### Scrolling the Chart via XAxis.visibleRange<a href="https://www.scichart.com/documentation/js/v4/get-started/tutorials-js-npm-webpack/tutorial-04-adding-realtime-updates/#scrolling-the-chart-via-xaxisvisiblerange" class="hash-link" aria-label="Direct link to Scrolling the Chart via XAxis.visibleRange" translate="no" title="Direct link to Scrolling the Chart via XAxis.visibleRange">â€‹</a>

To scroll in SciChart.js, all we have to do is manipulate theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#visiblerange" rel="noopener noreferrer" target="_blank">XAxis.visibleRangeðŸ“˜</a>. This accepts a type:Â <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" rel="noopener noreferrer" target="_blank">NumberRangeðŸ“˜</a>, which defines a minimum and maximum value to display on the chart.

Modify the updateData() function as follows:

- index.js

``` prism-code
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
```

This results in the following.

<img src="out_scichartv4/get-started/tutorials-js-npm-webpack/tutorial-04-adding-realtime-updates/index_media/cbd70df8830df9e473f62d6a0c303125b71dccd5.gif" class="img_ev3q" decoding="async" loading="lazy" width="806" height="788" />

The above example is simple, but it never removes points from the XyDataSeries, so memory will grow forever. Consider if you are streaming lots of data-points callingÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#removeat" rel="noopener noreferrer" target="_blank">XyDataSeries.removeAtðŸ“˜</a>, orÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#removerange" rel="noopener noreferrer" target="_blank">removeRangeðŸ“˜</a> or usingÂ [FIFO Modes](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-series-api/realtime-updates) to discard old data.

Also, once you are done with a DataSeries, callÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#delete" rel="noopener noreferrer" target="_blank">DataSeries.delete()ðŸ“˜</a> to remove WebAssembly memory.

We've written live examples with how to Append Data, Update Data, Scroll or Sweep using FIFO Modes at the pageÂ [DataSeries Realtime Updates](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-series-api/realtime-updates). Read this page and theÂ [DataSeries API pages](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-series-api/data-series-api-overview) to learn more about dynamic data updates in SciChart.

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/get-started/tutorials-js-npm-webpack/tutorial-04-adding-realtime-updates/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/get-started/tutorials-js-npm-webpack/tutorial-04-adding-realtime-updates/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
