On this page

# Tutorial 02 - Including index.min.js and WebAssembly Files offline

This set of tutorials will be limited to how to use SciChart.js by including index.min.js in a vanilla JS application (without npm and webpack).

We recommend going through the following tutorials which explain the API first.Â Â Â Â Â Â Â Â Â Â Â Â  Â Â Â Â Â Â Â Â Â Â Â 

- [Adding Series and Data](https://www.scichart.com/documentation/js/v4/get-started/tutorials-js-npm-webpack/tutorial-02-adding-series-and-data)
- [Adding Zooing and Panning behaviour](https://www.scichart.com/documentation/js/v4/get-started/tutorials-js-npm-webpack/tutorial-03-adding-zooming-panning-behavior)

## How to add SciChart.Browser.js to your project<a href="https://www.scichart.com/documentation/js/v4/get-started/tutorials-cdn/tutorial-02-offline/#how-to-add-scichartbrowserjs-to-your-project" class="hash-link" aria-label="Direct link to How to add SciChart.Browser.js to your project" translate="no" title="Direct link to How to add SciChart.Browser.js to your project">â€‹</a>

If you haven't seen our tutorial on how to add SciChart.js to a plain HTML page, then please first readÂ [Tutorial 01 - Including SciChart.js in an HTML page](https://www.scichart.com/documentation/js/v4/get-started/tutorials-cdn/tutorial-01-using-cdn).

This shows you how to reference the browser bundle script, initialize WebAssembly files and create your first basic chart.

## Adding Series and Data to a SciChart.js Chart<a href="https://www.scichart.com/documentation/js/v4/get-started/tutorials-cdn/tutorial-02-offline/#adding-series-and-data-to-a-scichartjs-chart" class="hash-link" aria-label="Direct link to Adding Series and Data to a SciChart.js Chart" translate="no" title="Direct link to Adding Series and Data to a SciChart.js Chart">â€‹</a>

![](out_scichartv4/get-started/tutorials-cdn/tutorial-02-offline/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

Source code for this tutorial can be found at <a href="https://github.com/ABTSoftware/SciChart.JS.Examples/tree/dev_v4.0/Tutorials/2D_Browser_CDN_Tutorials_JavaScript/Tutorial_2_Include_IndexMin_Wasm_Files_Offline" rel="noopener noreferrer" target="_blank">SciChart.Js.Examples Github Repository</a>

In this tutorial, we are going to addÂ some Line series onto the chart and to use downloaded js and wasm files to load SciChart.js offline (without internet access).

First, we will add aÂ [FastLineRenderableSeries](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-line-renderable-series) and add this to theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#renderableseries" rel="noopener noreferrer" target="_blank">SciChartSurface.renderableSeriesðŸ“˜</a> collection.

Next,Â we create anÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html" rel="noopener noreferrer" target="_blank">XyDataSeriesðŸ“˜</a> whichÂ isÂ the type which stores the data, and can accept dynamic updates (real-time updates) and manipulation of data. We will assign the dataseries to the FastLineRenderableSeries.

Try the code below:

- index.js
- index.html

``` prism-code
// Equivalent of imports when using index.min.js is to declare global variables like this
const { SciChartSurface, NumericAxis, FastLineRenderableSeries, XyDataSeries } =
  SciChart;

async function initSciChart() {
  // Create the SciChartSurface in the div 'scichart-root'
  const { sciChartSurface, wasmContext } = await SciChartSurface.create(
    "scichart-root"
  );

  // Create an X,Y Axis and add to the chart
  const xAxis = new NumericAxis(wasmContext);
  const yAxis = new NumericAxis(wasmContext);

  sciChartSurface.xAxes.add(xAxis);
  sciChartSurface.yAxes.add(yAxis);

  // Add a series
  sciChartSurface.renderableSeries.add(
    new FastLineRenderableSeries(wasmContext, {
      dataSeries: new XyDataSeries(wasmContext, {
        xValues: [0, 1, 2, 3, 4],
        yValues: [2, 1, 4, 3, 2],
      }),
    })
  );
}

SciChartSurface.configure({
  wasmUrl: `scichart/4.0.0-beta.734/scichart2d.wasm`,
});

initSciChart();
```

``` prism-code
<html lang="en-us">
  <head>
    <meta charset="utf-8" />
    <meta content="text/html; charset=utf-8" http-equiv="Content-Type" />
    <title>SciChart.js Locally Hosted Files</title>
    <!-- #region Include_index.min.js -->
    <!-- Ensure you have the correct path to the SciChart.js library locally -->
    <!-- and use crossorigin=anonymous to avoid CORS issues -->
    <script
      src="scichart/4.0.0-beta.734/index.min.js"
      crossorigin="anonymous"
    ></script>
    <!-- #endregion -->
    <script async type="text/javascript" src="index.js"></script>
    <style>
      body {
        font-family: "Arial";
      }
    </style>
  </head>
  <body>
    <h1>Hello SciChart.js world!</h1>
    <p>
      In this example we show how to host index.min.js and wasm files offline
    </p>

    <!-- the Div where the SciChartSurface will reside -->
    <div id="scichart-root" style="width: 800px; height: 600px"></div>
  </body>
</html>
```

Second, we need to create `scichart` folder and download `index.min.js` and `scichart2d.wasm` into it.

You can use these links to download files:

- <a href="https://cdn.jsdelivr.net/npm/scichart@4.0.0-beta.734/index.min.js" rel="noopener noreferrer" target="_blank">https://cdn.jsdelivr.net/npm/scichart@4.0.0-beta.734/index.min.js</a>
- <a href="https://cdn.jsdelivr.net/npm/scichart@4.0.0-beta.734/_wasm/scichart2d.wasm" rel="noopener noreferrer" target="_blank">https://cdn.jsdelivr.net/npm/scichart@4.0.0-beta.734/_wasm/scichart2d.wasm</a>
- <a href="https://cdn.jsdelivr.net/npm/scichart@4.0.0-beta.734/_wasm/scichart3d.wasm" rel="noopener noreferrer" target="_blank">https://cdn.jsdelivr.net/npm/scichart@4.0.0-beta.734/_wasm/scichart3d.wasm</a> (in case you need 3D charts)

![](out_scichartv4/get-started/tutorials-cdn/tutorial-02-offline/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

Use your version instead of `4.0.0-beta.734`

This is the folder structure:

<img src="out_scichartv4/get-started/tutorials-cdn/tutorial-02-offline/index_media/1d7a6b829c9ed42f2551252b09e01c7a62ca0c26.png" class="img_ev3q" decoding="async" loading="lazy" width="417" height="202" />

In order to run the example you will need a server. The simplest option would be to open directory with index.js and index.html files in cmd or terminal and to start python server `python3 -m http.server`. The app will be running here `http://localhost:8000/`.

<img src="out_scichartv4/get-started/tutorials-cdn/tutorial-02-offline/index_media/0d23b50769c0957c187777d9370d790d3b1c76aa.png" class="img_ev3q" decoding="async" loading="lazy" width="1053" height="632" />

This results in the following output:

<img src="out_scichartv4/get-started/tutorials-cdn/tutorial-02-offline/index_media/977783ea4c5ffa73fe5644baa1c2e701bf281191.png" class="img_ev3q" decoding="async" loading="lazy" width="1562" height="996" />

## Breaking the Code Down<a href="https://www.scichart.com/documentation/js/v4/get-started/tutorials-cdn/tutorial-02-offline/#breaking-the-code-down" class="hash-link" aria-label="Direct link to Breaking the Code Down" translate="no" title="Direct link to Breaking the Code Down">â€‹</a>

1.  <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#configure" rel="noopener noreferrer" target="_blank">SciChartSurface.configure()ðŸ“˜</a> sets from where to load wasm file.
2.  Next, ensure that the \*.wasm files are served with MIME type application/octet-stream.
3.  Verify the file that was downloaded, that it is actually the correct file & file type.
4.  Finally, ensure that the version numbers of \*.wasm files and index.min.js match. The best way to ensure this isÂ download a specific version again and overwriteÂ files in your source-code repository.

If after that you still are having problems, contactÂ <a href="https://www.scichart.com/contact-us" rel="noopener noreferrer" target="_blank">scichart support</a> and we will be glad to help.

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/get-started/tutorials-cdn/tutorial-02-offline/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/get-started/tutorials-cdn/tutorial-02-offline/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
