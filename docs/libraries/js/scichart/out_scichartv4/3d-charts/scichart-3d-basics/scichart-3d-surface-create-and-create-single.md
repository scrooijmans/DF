On this page

# SciChart3DSurface.create() vs. createSingle()

Instantiating a newÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html" rel="noopener noreferrer" target="_blank">SciChart3DSurfaceðŸ“˜</a> chart is accomplished with theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#create" rel="noopener noreferrer" target="_blank">SciChart3DSurface.create()ðŸ“˜</a> function. We have some variations on this function which can be used in different scenarios. We'll go through these as well as WebAssembly (wasm) file loading below.

## SciChart3DSurface.create()<a href="https://www.scichart.com/documentation/js/v4/3d-charts/scichart-3d-basics/scichart-3d-surface-create-and-create-single/#scichart3dsurfacecreate" class="hash-link" aria-label="Direct link to SciChart3DSurface.create()" translate="no" title="Direct link to SciChart3DSurface.create()">â€‹</a>

The first function to create a 3D chart in SciChart.js is simplyÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#create" rel="noopener noreferrer" target="_blank">SciChart3DSurface.create()ðŸ“˜</a>. This is an **asynchronous static function** which places a SciChart3DSurface (a single chart with X, Y, Z axis and one to many series) into the HTML Dom. It will also load WebAssembly files and initialise our 3D WebGL engine for the first chart load.

- SciChart3DSurface.create()

``` prism-code
import { SciChart3DSurface } from "scichart";

async function initSciChart1() {
    // Assumes a div in your HTML <div id="scichart-div-id"></div>
    const { sciChart3DSurface, wasmContext } = await SciChart3DSurface.create("scichart-div-id");
    // Now manipulate the sciChart3DSurface, adding axis, series and more
    // When you exit the page and no longer want the chart to draw, call .delete() to free memory
    sciChart3DSurface.delete();
}
```

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#create" rel="noopener noreferrer" target="_blank">SciChart3DSurface.create()ðŸ“˜</a> **uses a single, shared WebGL context for all chart surfaces on the screen**. This bypasses the maximum number of WebGL contexts and you can have 10, 20, 30 or even 100 charts on an HTML page. The only limit is performance of the browser in rendering the chart surfaces. **For a higher performance solution which uses one WebGL context per chart, see**Â <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#createsingle" rel="noopener noreferrer" target="_blank">SciChart3DSurface.createSingle()ðŸ“˜</a>

**Ensure that you await SciChart3DSurface.create().** The return type is an object containing SciChart3DSurface and its wasmContext (WebAssembly Context) which must be passed to other chart parts on this SciChart3DSurface.

## SciChart3DSurface.createSingle()<a href="https://www.scichart.com/documentation/js/v4/3d-charts/scichart-3d-basics/scichart-3d-surface-create-and-create-single/#scichart3dsurfacecreatesingle" class="hash-link" aria-label="Direct link to SciChart3DSurface.createSingle()" translate="no" title="Direct link to SciChart3DSurface.createSingle()">â€‹</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#createsingle" rel="noopener noreferrer" target="_blank">SciChart3DSurface.createSingle()ðŸ“˜</a> is also an asynchronous static function which places a SciChart3DSurface into the DOM. However, this variation forces one WebGL context per chart. This can improve performance in multi-chart scenarios but you must obey the WebGL Context Limits per browser. More on this in ourÂ [Performance Tips](https://www.scichart.com/documentation/js/v4/2d-charts/performance-tips/performance-tips-and-tricks) article.

- SciChart3DSurface.createSingle()

``` prism-code
import { SciChart3DSurface } from "scichart";

async function initSciChart2() {
    // Assumes a div in your HTML <div id="scichart-div-id"></div>
    const { sciChart3DSurface, wasmContext } = await SciChart3DSurface.createSingle("scichart-div-id");
    // Now manipulate the sciChart3DSurface, adding axis, series and more
    // When you exit the page and no longer want the chart to draw, call .delete() to free memory
    sciChart3DSurface.delete();
}
```

### Resolving Wasm errors on load<a href="https://www.scichart.com/documentation/js/v4/3d-charts/scichart-3d-basics/scichart-3d-surface-create-and-create-single/#resolving-wasm-errors-on-load" class="hash-link" aria-label="Direct link to Resolving Wasm errors on load" translate="no" title="Direct link to Resolving Wasm errors on load">â€‹</a>

If you get an error when loading a SciChart3DSurface as follows:

![](out_scichartv4/3d-charts/scichart-3d-basics/scichart-3d-surface-create-and-create-single/index_media/0f0874621de662630fd5d7f09c89455c4ace26e6.svg)warning

Could not load SciChart WebAssembly module. Check your build process and ensure that your scichart3d.wasm and scichart3d.js files are from the same version

If so, find out how to resolve this at the pageÂ [Deploying Wasm and Data files](https://www.scichart.com/documentation/js/v4/2d-charts/surface/deploying-wasm/).

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/3d-charts/scichart-3d-basics/scichart-3d-surface-create-and-create-single/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/3d-charts/scichart-3d-basics/scichart-3d-surface-create-and-create-single/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
