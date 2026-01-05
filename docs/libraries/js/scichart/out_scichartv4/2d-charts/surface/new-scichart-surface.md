On this page

# Creating a new SciChartSurface and loading Wasm

Instantiating a new SciChartSurface (a new Chart) is accomplished with theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#create" rel="noopener noreferrer" target="_blank">SciChartSurface.create()ðŸ“˜</a> function. We have some variations on this function which can be used in different scenarios. We'll go through these as well as WebAssembly (wasm) file loading below.

## SciChartSurface.create()<a href="https://www.scichart.com/documentation/js/v4/2d-charts/surface/new-scichart-surface/#scichartsurfacecreate" class="hash-link" aria-label="Direct link to SciChartSurface.create()" translate="no" title="Direct link to SciChartSurface.create()">â€‹</a>

The first function to create a chart in SciChart.js is simplyÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#create" rel="noopener noreferrer" target="_blank">SciChartSurface.create()ðŸ“˜</a>. This is an **asynchronous static function** which places a SciChartSurface (a single chart with X, Y axis and one to many series) into the HTML Dom. It will also load WebAssembly files and initialise our 2D/3D WebGL engine for the first chart load.

``` prism-code
import {SciChartSurface} from "scichart";
async function initSciChart1() {
    // Assumes a div in your HTML <div id="scichart-div-1"></div>
    const { sciChartSurface, wasmContext } = await SciChartSurface.create("scichart-div-id");
    // Now manipulate the SciChartSurface, adding axis, series and more
    // When you exit the page and no longer want the chart to draw, call .delete() to free memory
    sciChartSurface.delete();
}
```

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#create" rel="noopener noreferrer" target="_blank"><strong>SciChartSurface.create()</strong>ðŸ“˜</a> **uses a single, shared WebGL context for all chart surfaces on the screen**. This bypasses the maximum number of WebGL contexts and you can have 10, 20, 30 or even 100 charts on an HTML page. The only limit is performance of the browser in rendering the chart surfaces. **For a higher performance solution which uses one WebGL context per chart, see**Â <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#createsingle" rel="noopener noreferrer" target="_blank"><strong>SciChartSurface.createSingle()</strong>ðŸ“˜</a>

![](out_scichartv4/2d-charts/surface/new-scichart-surface/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

**Ensure that you await SciChartSurface.create().** The return type is an object containing SciChartSurface and its wasmContext (WebAssembly Context) which must be passed to other chart parts on this SciChartSurface.

## SciChartSurface.createSingle()<a href="https://www.scichart.com/documentation/js/v4/2d-charts/surface/new-scichart-surface/#scichartsurfacecreatesingle" class="hash-link" aria-label="Direct link to SciChartSurface.createSingle()" translate="no" title="Direct link to SciChartSurface.createSingle()">â€‹</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#createsingle" rel="noopener noreferrer" target="_blank">SciChartSurface.createSingle()ðŸ“˜</a> is also an asynchronous static function which places a SciChartSurface into the DOM. However, this variation forces one WebGL context per chart. This can improve performance in multi-chart scenarios but you must obey the WebGL Context Limits per browser. More on this in ourÂ [Performance Tips](https://www.scichart.com/documentation/js/v4/2d-charts/performance-tips/performance-tips-and-tricks) article.

``` prism-code
import {SciChartSurface} from "scichart";
async function initSciChart1() {
    // Assumes a div in your HTML <div id="scichart-div-1"></div>
    const { sciChartSurface, wasmContext } = await SciChartSurface.createSingle("scichart-div-id");
    // Now manipulate the SciChartSurface, adding axis, series and more
    // When you exit the page and no longer want the chart to draw, call .delete() to free memory
    sciChartSurface.delete();
}
```

### Resolving Wasm errors on load<a href="https://www.scichart.com/documentation/js/v4/2d-charts/surface/new-scichart-surface/#resolving-wasm-errors-on-load" class="hash-link" aria-label="Direct link to Resolving Wasm errors on load" translate="no" title="Direct link to Resolving Wasm errors on load">â€‹</a>

If you get an error when loading a SciChartSurface as follows:

![](out_scichartv4/2d-charts/surface/new-scichart-surface/index_media/0f0874621de662630fd5d7f09c89455c4ace26e6.svg)warning

**Error**: Could not load SciChart WebAssembly module. Check your build process and ensure that your scichart2d.wasm, scichart2d.data and scichart2d.js files are from the same version

Please see our related articleÂ [Deploying Wasm or WebAssembly Data Files with your app](https://www.scichart.com/documentation/js/v4/2d-charts/surface/deploying-wasm)

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/surface/new-scichart-surface/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [Setting a Runtime License on a SciChartSurface](https://www.scichart.com/documentation/js/v4/2d-charts/surface/runtime-license)
- [The SciChartSurface Type](https://www.scichart.com/documentation/js/v4/2d-charts/surface/scichart-surface-type-overview)

#### JS Webpack npm Tutorials<a href="https://www.scichart.com/documentation/js/v4/2d-charts/surface/new-scichart-surface/#js-webpack-npm-tutorials" class="hash-link" aria-label="Direct link to JS Webpack npm Tutorials" translate="no" title="Direct link to JS Webpack npm Tutorials">â€‹</a>

[Tutorial 01 - Setting up a npm Project with SciChart.js](https://www.scichart.com/documentation/js/v4/get-started/tutorials-js-npm-webpack/tutorial-01-setting-up-npm-project-with-scichart-js)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/surface/new-scichart-surface/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/surface/new-scichart-surface/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
