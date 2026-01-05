On this page

# Tutorial 01 - Including SciChart.js in an HTML Page using CDN

This set of tutorials will be limited to how to use SciChart.js by including index.min.js in a vanilla JS application (without npm and webpack).

We recommend going through the following tutorials which explain the API first.Â Â Â Â Â Â Â Â Â Â Â Â  Â Â Â Â Â Â Â Â Â Â Â 

- [Adding Series and Data](https://www.scichart.com/documentation/js/v4/get-started/tutorials-js-npm-webpack/tutorial-02-adding-series-and-data)
- [Adding Zooming and Panning behaviour](https://www.scichart.com/documentation/js/v4/get-started/tutorials-js-npm-webpack/tutorial-03-adding-zooming-panning-behavior)
- [Adding Realtime Updates](https://www.scichart.com/documentation/js/v4/get-started/tutorials-js-npm-webpack/tutorial-04-adding-realtime-updates)

![](out_scichartv4/get-started/tutorials-cdn/tutorial-01-using-cdn/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

Source code for this tutorial can be found at <a href="https://github.com/ABTSoftware/SciChart.JS.Examples/tree/dev_v4.0/Tutorials/2D_Browser_CDN_Tutorials_JavaScript/Tutorial_1_Setting_up_html_page_with_SciChart_CDN" rel="noopener noreferrer" target="_blank">SciChart.Js.Examples Github Repository</a>

## Including SciChart.js in an HTML Page using CDN<a href="https://www.scichart.com/documentation/js/v4/get-started/tutorials-cdn/tutorial-01-using-cdn/#including-scichartjs-in-an-html-page-using-cdn" class="hash-link" aria-label="Direct link to Including SciChart.js in an HTML Page using CDN" translate="no" title="Direct link to Including SciChart.js in an HTML Page using CDN">â€‹</a>

The following tutorial shows you how to include SciChart.js in an HTML page via a CDN, without the need for setup of npm and a module bundler such as webpack.

This methods is suitable for users who have JavaScript applications but still want to use SciChart.js to show charts & graphs in their applications.

This method is also useful as a fallback in application frameworks like Blazor, which don't have Npm support but do need to interop with JavaScript charts in a webpage.

## How to add SciChart index.min.js to your project<a href="https://www.scichart.com/documentation/js/v4/get-started/tutorials-cdn/tutorial-01-using-cdn/#how-to-add-scichart-indexminjs-to-your-project" class="hash-link" aria-label="Direct link to How to add SciChart index.min.js to your project" translate="no" title="Direct link to How to add SciChart index.min.js to your project">â€‹</a>

SciChart.js is hosted as a javascript file over atÂ <a href="https://www.jsdelivr.com/package/npm/scichart" rel="noopener noreferrer" target="_blank">jsdelivr.com/package/npm/scichart</a>. This can be included in a webpage either by linking direct to the script on CDN or downloading it.

1.  Choose what version of SciChart you want to use. To find out which versions are available, head over to <a href="https://www.npmjs.com/package/scichart" rel="noopener noreferrer" target="_blank">npmjs.com/package/scichart</a> and click on versions.

2.  Add script pointing to SciChart.js version into the head section of your html file.

    a\. For instance to use version `4.0.0-beta.734` add this script:

    - Include specific version index.html

    ``` prism-code
    <html lang="en-us">
    <head>
        <meta charset="utf-8" />
        <meta content="text/html; charset=utf-8" http-equiv="Content-Type" />
        <title>SciChart.js Browser Bundle Tutorial 1</title>
        <script
        src="https://cdn.jsdelivr.net/npm/scichart@4.0.0-beta.734/index.min.js"
        crossorigin="anonymous"
        ></script>
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
        In this example we scichart using pure JavaScript from CDN and create a
        simple chart with one X and Y axis
        </p>

        <!-- the Div where the SciChartSurface will reside -->
        <div id="scichart-root" style="width: 800px; height: 600px"></div>
    </body>
    </html>
    ```

    b\. You can if you wish, include latest minor version by using this syntax

    - Include latest minor version

    ``` prism-code
        <script
            src="https://cdn.jsdelivr.net/npm/scichart@4.0/index.min.js"
            crossorigin="anonymous"
        ></script>
    ```

    c\. Or, latest major/minor version (unrecommended for production) by using this syntax

    - Include latest version

    ``` prism-code
        <script
          src="https://cdn.jsdelivr.net/npm/scichart/index.min.js"
          crossorigin="anonymous"
        ></script>
    ```

3.  Next, add some series, axis and behaviours to the SciChartSurface (more on that below)

## Worked Example using index.min.js<a href="https://www.scichart.com/documentation/js/v4/get-started/tutorials-cdn/tutorial-01-using-cdn/#worked-example-using-indexminjs" class="hash-link" aria-label="Direct link to Worked Example using index.min.js" translate="no" title="Direct link to Worked Example using index.min.js">â€‹</a>

![](out_scichartv4/get-started/tutorials-cdn/tutorial-01-using-cdn/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

In order to run the example you will need a server. The simplest option would be to open directory with index.js and index.html files in cmd or terminal and to start python server `python3 -m http.server`. The app will be running here `http://localhost:8000/`.

We have a worked example over atÂ <a href="https://github.com/ABTSoftware/SciChart.JS.Examples/tree/dev_v4.0/Tutorials/2D_Browser_CDN_Tutorials_JavaScript/Tutorial_1_Setting_up_html_page_with_SciChart_CDN" rel="noopener noreferrer" target="_blank">Github</a> of how to use this, but for the sake of ease let's include the code below.

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
initSciChart();
```

``` prism-code
<html lang="en-us">
  <head>
    <meta charset="utf-8" />
    <meta content="text/html; charset=utf-8" http-equiv="Content-Type" />
    <title>SciChart.js Browser Bundle Tutorial 1</title>
    <script
      src="https://cdn.jsdelivr.net/npm/scichart@4.0.0-beta.734/index.min.js"
      crossorigin="anonymous"
    ></script>
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
      In this example we scichart using pure JavaScript from CDN and create a
      simple chart with one X and Y axis
    </p>

    <!-- the Div where the SciChartSurface will reside -->
    <div id="scichart-root" style="width: 800px; height: 600px"></div>
  </body>
</html>
```

This results in the following output:

<img src="out_scichartv4/get-started/tutorials-cdn/tutorial-01-using-cdn/index_media/d27798eada1d3d85f9cdcf80ed58ac959374f5c2.png" class="img_ev3q" decoding="async" loading="lazy" width="1021" height="997" />

## Breaking the Code Down<a href="https://www.scichart.com/documentation/js/v4/get-started/tutorials-cdn/tutorial-01-using-cdn/#breaking-the-code-down" class="hash-link" aria-label="Direct link to Breaking the Code Down" translate="no" title="Direct link to Breaking the Code Down">â€‹</a>

1.  We included the `<script>` tag to load `index.min.js` with a specific version
2.  We have an async function to setup SciChart. This is necessary because SciChart itself uses async functions to load, and we don't want to block the browser loading.
3.  Inside this async function, we callÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#create" rel="noopener noreferrer" target="_blank">SciChartSurface.create()ðŸ“˜</a> to instantiate a chart surface. This function must be awaited.
4.  Lastly, we can use the SciChart API to add a X and Y Axis and series with some data.

![](out_scichartv4/get-started/tutorials-cdn/tutorial-01-using-cdn/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

When using npm we have

``` prism-code
import { SciChartSurface, NumericAxis ..} from "scichart";
```

When using index.min.js all types are global variables and we use

``` prism-code
const { SciChartSurface, NumericAxis ..} = SciChart;
```

## How the Wasm Files get Initialized<a href="https://www.scichart.com/documentation/js/v4/get-started/tutorials-cdn/tutorial-01-using-cdn/#how-the-wasm-files-get-initialized" class="hash-link" aria-label="Direct link to How the Wasm Files get Initialized" translate="no" title="Direct link to How the Wasm Files get Initialized">â€‹</a>

If you've watched our [Tutorial 01 - Setting up an npm project with WebPack](https://www.scichart.com/documentation/js/v4/get-started/tutorials-js-npm-webpack/tutorial-01-setting-up-npm-project-with-scichart-js), you will have read about wasm (WebAssembly)Â files which must be served to load the chart.

SciChart.js when served from CDN automatically defaults to load the wasm from CDN as well.

When we load SciChart.js via index.min.js implicitly this line of code is called

``` prism-code
SciChartSurface.useWasmFromCDN();
```

which is equivalent to calling SciChartSurface.configure() with the URL from the CDN for wasm file

``` prism-code
const libraryVersion = "4.0.0";
SciChartSurface.configure({
  wasmUrl: `https://cdn.jsdelivr.net/npm/scichart@${libraryVersion}/_wasm/scichart2d.wasm`,
});
```

This code doesn't actually need to be called when using `index.min.js`, however its good to be aware that it is happening in the background.

It's important to note whenÂ initializing SciChart.js from CDN that the wasm files will also be loaded from CDN. **So your application will require an internet connection to work**.

![](out_scichartv4/get-started/tutorials-cdn/tutorial-01-using-cdn/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

If you want to host the index.min.js and wasm files locally, then they must be downloaded and included in your app. ReadÂ [Tutorial 02 - Including index.min.js and WebAssembly Files offline](https://www.scichart.com/documentation/js/v4/get-started/tutorials-cdn/tutorial-02-offline)Â to find out how.

Further details onÂ [Wasm file deployment can also be found here](https://www.scichart.com/documentation/js/v4/2d-charts/surface/deploying-wasm).

## Further Notes<a href="https://www.scichart.com/documentation/js/v4/get-started/tutorials-cdn/tutorial-01-using-cdn/#further-notes" class="hash-link" aria-label="Direct link to Further Notes" translate="no" title="Direct link to Further Notes">â€‹</a>

All of the SciChart APIs can be used in theÂ index.min.js method where you include a script directly in HTML.

There is no intellisense or type discovery when using index.min.js. For professional or enterprise grade apps we recommend using npm, a bundler like WebPackÂ and TypeScript which provides a far superior development experience.

![](out_scichartv4/get-started/tutorials-cdn/tutorial-01-using-cdn/index_media/0f0874621de662630fd5d7f09c89455c4ace26e6.svg)warning

**A Note on Licensing SciChart.**

The SciChart.js control comes with a community license which is watermarked. This can be used for commercial trial use for a reasonable time period.

For commercial licenses, a license key can be applied following the instructions at <a href="https://www.scichart.com/licensing-scichart-js" rel="noopener noreferrer" target="_blank">www.scichart.com/licensing-scichart-js</a>.

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/get-started/tutorials-cdn/tutorial-01-using-cdn/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/get-started/tutorials-cdn/tutorial-01-using-cdn/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
