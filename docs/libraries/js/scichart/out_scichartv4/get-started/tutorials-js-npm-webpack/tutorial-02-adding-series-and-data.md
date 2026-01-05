On this page

# Tutorial 02 - Adding Series and Data

In the [previous tutorial](https://www.scichart.com/documentation/js/v4/get-started/tutorials-js-npm-webpack/tutorial-01-setting-up-npm-project-with-scichart-js) we explained how to setup a project with a simple chart using SciChart.js. In this tutorial, we are going to show you how to add some dataÂ and a line series to the chart.

# Ein Fehler ist aufgetreten.

JavaScript kann nicht ausgeführt werden.

Video tutorial for version 3. SciChart.js JavaScript Chart Tutorial 02 - Adding Series and Data

  

![](out_scichartv4/get-started/tutorials-js-npm-webpack/tutorial-02-adding-series-and-data/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

Source code for this tutorial can be found at <a href="https://github.com/ABTSoftware/SciChart.JS.Examples/tree/dev_v4.0/Tutorials/2D_Chart_Tutorials_JavaScript/Tutorial_2_Adding_Series_and_Data" rel="noopener noreferrer" target="_blank">SciChart.Js.Examples Github Repository</a>

## Adding Series to the Chart<a href="https://www.scichart.com/documentation/js/v4/get-started/tutorials-js-npm-webpack/tutorial-02-adding-series-and-data/#adding-series-to-the-chart" class="hash-link" aria-label="Direct link to Adding Series to the Chart" translate="no" title="Direct link to Adding Series to the Chart">â€‹</a>

In SciChart, there are special classes calledÂ [RenderableSeries](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/renderable-series-api-overview) that are responsible for drawing different chart types, such as lines ([FastLineRenderableSeries](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-line-renderable-series)), columns ([FastColumnsRenderableSeries](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-column-renderable-series/column-series-type)), candlestick series ([FastCandlestickRenderableSeries](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-candlestick-renderable-series)), filled area ([FastMountainRenderableSeries](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-mountain-area-renderable-series)), heat maps ([FastUniformHeatmapRenderableSeries](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/uniform-heatmap-renderable-series/uniform-heatmap-chart-type)) etc...

### Adding a Line Plot to the Chart<a href="https://www.scichart.com/documentation/js/v4/get-started/tutorials-js-npm-webpack/tutorial-02-adding-series-and-data/#adding-a-line-plot-to-the-chart" class="hash-link" aria-label="Direct link to Adding a Line Plot to the Chart" translate="no" title="Direct link to Adding a Line Plot to the Chart">â€‹</a>

In this tutorial, we are going to addÂ some Line series onto the chart.

First,Â we create aÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html" rel="noopener noreferrer" target="_blank">XyDataSeriesðŸ“˜</a> whichÂ isÂ the type which stores the data, and can accept dynamic updates (real-time updates) and manipulation of data. We will assign the dataseries to theÂ [FastLineRenderableSeries](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-line-renderable-series).

Next, we create aÂ [FastLineRenderableSeries](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-line-renderable-series) and add this to theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#renderableseries" rel="noopener noreferrer" target="_blank">SciChartSurface.renderableSeriesðŸ“˜</a> collection.

Try the code below:

- JS

``` prism-code
import {
  SciChartSurface,
  NumericAxis,
  FastLineRenderableSeries,
  XyDataSeries,
} from "scichart";

async function initSciChart() {
  // Create the SciChartSurface in the div 'scichart-root'
  // The SciChartSurface, and webassembly context 'wasmContext' are paired. This wasmContext
  // instance must be passed to other types that exist on the same surface.
  const { sciChartSurface, wasmContext } = await SciChartSurface.create(
    "scichart-root"
  );

  // Create an X,Y Axis and add to the chart
  sciChartSurface.xAxes.add(new NumericAxis(wasmContext));
  sciChartSurface.yAxes.add(new NumericAxis(wasmContext));

  // Declare a DataSeries
  const xyDataSeries = new XyDataSeries(wasmContext);
  xyDataSeries.append(1, 2);
  xyDataSeries.append(3, 4);

  // Add a line series to the SciChartSurface
  sciChartSurface.renderableSeries.add(
    new FastLineRenderableSeries(wasmContext, {
      strokeThickness: 3,
      stroke: "rgba(255, 0, 0, 1)",
      dataSeries: xyDataSeries,
    })
  );

  // zoom to fit (optional, will occur automatically once on startup)
  sciChartSurface.zoomExtents();
}

initSciChart();
```

Ensure you also have the index.html set, which must contain a div with id="scichart-root" (or whatever you pass toÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#create" rel="noopener noreferrer" target="_blank">SciChartSurface.createðŸ“˜</a>)

- HTML

``` prism-code
<html lang="en-us">
<head>
  <meta charset="utf-8" />
  <meta content="text/html; charset=utf-8" http-equiv="Content-Type" />
  <link rel="icon" href="data:," />
  <title>SciChart.js Tutorial 2</title>
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
    In this example we add a line series to the chart, using
    FastLineRenderableSeries and XyDataSeries.
  </p>
  <!-- the Div where the SciChartSurface will reside -->
  <div id="scichart-root" style="width: 100%; height: 600px"></div>
</body>
</html>
```

You should get this result:Â 

<img src="out_scichartv4/get-started/tutorials-js-npm-webpack/tutorial-02-adding-series-and-data/index_media/ac0a8e48db8837655d7b4a6b272cc688c5f9867c.png" class="img_ev3q" decoding="async" loading="lazy" width="902" height="806" alt="Simple JavaScript Line Chart with SciChart.js" />

We've added a line series to the chart, styled it red, and added two data-points. Hardly ground-breaking, but it's a start!

Let's take this up a notch in the second part of the tutorial, by adding 100 series, each with 10,000 data-points.

## Adding 100 Series, with 10,000 Datapoints to the Chart<a href="https://www.scichart.com/documentation/js/v4/get-started/tutorials-js-npm-webpack/tutorial-02-adding-series-and-data/#adding-100-series-with-10000-datapoints-to-the-chart" class="hash-link" aria-label="Direct link to Adding 100 Series, with 10,000 Datapoints to the Chart" translate="no" title="Direct link to Adding 100 Series, with 10,000 Datapoints to the Chart">â€‹</a>

We can take this a little bit further, by adding 100 series each with 10,000 datapoints to the Chart, for a total ofÂ one million data-points. SciChart's specialty is High Performance, Realtime Charts, and that means you can add large amounts of data to our JavaScript chart component with ease.

Modify the code in index.js to the following:

- JS

``` prism-code
import {
  SciChartSurface,
  NumericAxis,
  XyDataSeries,
  FastLineRenderableSeries,
} from "scichart";

async function initSciChart() {
  // Create the SciChartSurface in the div 'scichart-root'
  // The SciChartSurface, and webassembly context 'wasmContext' are paired. This wasmContext
  // instance must be passed to other types that exist on the same surface.
  const { sciChartSurface, wasmContext } = await SciChartSurface.create(
    "scichart-root"
  );

  // Create an X,Y Axis and add to the chart
  sciChartSurface.xAxes.add(new NumericAxis(wasmContext));
  sciChartSurface.yAxes.add(new NumericAxis(wasmContext));

  // Create 100 dataseries, each with 10k points
  for (let seriesCount = 0; seriesCount < 100; seriesCount++) {
    const xyDataSeries = new XyDataSeries(wasmContext);

    const opacity = (1 - seriesCount / 120).toFixed(2);

    // Populate with some data
    for (let i = 0; i < 10000; i++) {
      xyDataSeries.append(i, Math.sin(i * 0.01) * Math.exp(i * (0.00001 * (seriesCount + 1))));
    }

    // Add and create a line series with this data to the chart
    // Create a line series
    const lineSeries = new FastLineRenderableSeries(wasmContext, {
      dataSeries: xyDataSeries,
      stroke: `rgba(176,196,222,${opacity})`,
      strokeThickness: 2,
    });
    sciChartSurface.renderableSeries.add(lineSeries);
  }
}

initSciChart();
```

This code adds 100 series in a loop, each with 10,000 data-points using theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#append" rel="noopener noreferrer" target="_blank">XyDataSeries.appendðŸ“˜</a> method. The mathematical function in there is just to create a nice looking waveform.

We create aÂ [FastLineRenderableSeries](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-line-renderable-series) for each trip around the outer loop and this time use the constructor parameters to set theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastlinerenderableseries.html#dataseries" rel="noopener noreferrer" target="_blank">dataSeriesðŸ“˜</a>,Â <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastlinerenderableseries.html#stroke" rel="noopener noreferrer" target="_blank">strokeðŸ“˜</a> andÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastlinerenderableseries.html#strokethickness" rel="noopener noreferrer" target="_blank">strokeThicknessðŸ“˜</a> properties.

This is the result below:

<img src="out_scichartv4/get-started/tutorials-js-npm-webpack/tutorial-02-adding-series-and-data/index_media/7ba2dd6bf0cb40fcf0485cb3bf9392994c85091e.png" class="img_ev3q" decoding="async" loading="lazy" width="902" height="806" alt="JavaScript Chart with Big Data (1 million points) with SciChart.js" />

There you go! One million data-points in a JavaScript Chart using SciChart.js!

Join us forÂ [Tutorial 3](https://www.scichart.com/documentation/js/v4/get-started/tutorials-js-npm-webpack/tutorial-03-adding-zooming-panning-behavior) where we will be adding zooming and panning behaviour to the chart.

![](out_scichartv4/get-started/tutorials-js-npm-webpack/tutorial-02-adding-series-and-data/index_media/0f0874621de662630fd5d7f09c89455c4ace26e6.svg)warning

**A Note on Licensing SciChart.**

The SciChart.js control comes with a community license which is watermarked. This can be used for commercial trial use for a reasonable time period.

For commercial licenses, a license key can be applied following the instructions at <a href="https://www.scichart.com/licensing-scichart-js" rel="noopener noreferrer" target="_blank">www.scichart.com/licensing-scichart-js</a>.

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/get-started/tutorials-js-npm-webpack/tutorial-02-adding-series-and-data/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/get-started/tutorials-js-npm-webpack/tutorial-02-adding-series-and-data/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
