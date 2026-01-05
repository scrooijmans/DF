On this page

# Deploying Wasm (WebAssembly) with your app

If you receive an error message when running your app, you may not have deployed the Wasm (WebAssembly) or data files correctly. Below are some steps on how to resolve that.

![](out_scichartv4/2d-charts/surface/deploying-wasm/index_media/0f0874621de662630fd5d7f09c89455c4ace26e6.svg)warning

**Error**: Could not load SciChart WebAssembly module. Check your build process and ensure that your scichart2d.wasm and scichart2d.js files are from the same version

### Option 1: Package Wasm & Data Files with Webpack (or similar)Â <a href="https://www.scichart.com/documentation/js/v4/2d-charts/surface/deploying-wasm/#option-1-package-wasm--data-files-with-webpack-or-similar" class="hash-link" aria-label="Direct link to Option 1: Package Wasm &amp; Data Files with Webpack (or similar)Â " translate="no" title="Direct link to Option 1: Package Wasm &amp; Data Files with Webpack (or similar)Â ">â€‹</a>

In our tutorials and boilerplate examples we show you how to package the Wasm files to load them in a variety of JavaScript frameworks including React, Angular, Vue, Vite, Electron, Tauri, Svelte, Blazor, Next, Nuxt and more. Find the links to setting up a JavaScript project below:

| JS Project Framework | Boilerplate Project or Setup Instructions |
|----|----|
| npm / webpack | [Tutorial - Setting up a project with Webpack](https://www.scichart.com/documentation/js/v4/get-started/tutorials-js-npm-webpack/tutorial-01-setting-up-npm-project-with-scichart-js) |
| Vanilla Javascript CDN (no npm, webpack) | [Tutorial - Including index.min.js and wasm files using CDN](https://www.scichart.com/documentation/js/v4/get-started/tutorials-cdn/tutorial-01-using-cdn) |
| Vanilla Javascript offline (no npm, webpack) | [Tutorial - Including index.min.js and wasm files offline](https://www.scichart.com/documentation/js/v4/get-started/tutorials-cdn/tutorial-02-offline) |
| React (scichart-react) | <a href="https://github.com/ABTSoftware/SciChart.JS.Examples/tree/master/BoilerPlates/scichart-react" rel="noopener noreferrer" target="_blank">code sample</a> |
| vue.js | <a href="https://github.com/ABTSoftware/SciChart.JS.Examples/tree/master/BoilerPlates/vue" rel="noopener noreferrer" target="_blank">code sample</a> |
| svelte-vite | <a href="https://github.com/ABTSoftware/SciChart.JS.Examples/tree/master/BoilerPlates/svelte-vite" rel="noopener noreferrer" target="_blank">code sample</a> |
| svelte-rollup | <a href="https://github.com/ABTSoftware/SciChart.JS.Examples/tree/master/BoilerPlates/svelte-rollup" rel="noopener noreferrer" target="_blank">code sample</a> |
| react-vite | <a href="https://github.com/ABTSoftware/SciChart.JS.Examples/tree/master/BoilerPlates/react-vite" rel="noopener noreferrer" target="_blank">code sample</a> |
| nextjs | <a href="https://github.com/ABTSoftware/SciChart.JS.Examples/tree/master/BoilerPlates/next" rel="noopener noreferrer" target="_blank">code sample</a> |
| Nuxt.js | <a href="https://github.com/ABTSoftware/SciChart.JS.Examples/tree/master/BoilerPlates/nuxt" rel="noopener noreferrer" target="_blank">code sample</a> |
| Angular | <a href="https://github.com/ABTSoftware/SciChart.JS.Examples/tree/master/BoilerPlates/angular" rel="noopener noreferrer" target="_blank">code sample</a> |
| Angular (scichart-angular) | <a href="https://github.com/ABTSoftware/SciChart.JS.Examples/tree/master/BoilerPlates/scichart-angular" rel="noopener noreferrer" target="_blank">code sample</a> |
| blazor via JS Interop | <a href="https://github.com/ABTSoftware/SciChart.JS.Examples/tree/master/BoilerPlates/blazor" rel="noopener noreferrer" target="_blank">code sample</a> |
| Electron | <a href="https://github.com/ABTSoftware/SciChart.JS.Examples/tree/master/BoilerPlates/electron" rel="noopener noreferrer" target="_blank">code sample</a> |
| Tauri React Vite | <a href="https://github.com/ABTSoftware/SciChart.JS.Examples/tree/master/BoilerPlates/tauri-vite-react" rel="noopener noreferrer" target="_blank">code sample</a> |
| Tauri Javascript Vite | <a href="https://github.com/ABTSoftware/SciChart.JS.Examples/tree/master/BoilerPlates/tauri-vite-vanilla" rel="noopener noreferrer" target="_blank">code sample</a> |
| Web components | <a href="https://github.com/ABTSoftware/SciChart.JS.Examples/tree/master/BoilerPlates/web-components" rel="noopener noreferrer" target="_blank">code sample</a> |

![](out_scichartv4/2d-charts/surface/deploying-wasm/index_media/af50301a53fa555616a9b2279e7e25a1d566cb1a.svg)info

The above projects have been updated for SciChart.js v4, which has only a \*.wasm file to serve. SciChart.js v3.x had \*.wasm and \*.data files. For older versions of SciChart, see the boilerplates folder in the <a href="https://github.com/ABTSoftware/SciChart.JS.Examples/tree/dev_v3.5/BoilerPlates" rel="noopener noreferrer" target="_blank">dev 3.5</a> branch.

![](out_scichartv4/2d-charts/surface/deploying-wasm/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

See more boilerplate examples for JavaScript frameworksÂ at our Github repository:Â <a href="https://github.com/ABTSoftware/SciChart.JS.Examples/tree/master/Sandbox" rel="noopener noreferrer" target="_blank">github.com/abtsoftware/scichart.js.examples</a> under the Boilerplates folder Â 

### Option 2: Load Wasm from URL with SciChartSurface.configure() or useWasmFromCDN()<a href="https://www.scichart.com/documentation/js/v4/2d-charts/surface/deploying-wasm/#option-2-load-wasm-from-url-with-scichartsurfaceconfigure-or-usewasmfromcdn" class="hash-link" aria-label="Direct link to Option 2: Load Wasm from URL with SciChartSurface.configure() or useWasmFromCDN()" translate="no" title="Direct link to Option 2: Load Wasm from URL with SciChartSurface.configure() or useWasmFromCDN()">â€‹</a>

The easiest way for SciChart.js to load WebAssembly and Data files are to load them from our CDNÂ (see <a href="https://www.jsdelivr.com/package/npm/scichart" rel="noopener noreferrer" target="_blank">jsdelivr.com/package/npm/scichart</a>). This method is particularly useful in projects or frameworks that don't have a package manager or module bundler.

To do load SciChart's Wasm and Data file from CDN, callÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#configure" rel="noopener noreferrer" target="_blank">SciChartSurface.configure()ðŸ“˜</a> once before any SciChartSurface is shown:

**Configure Wasm and Data File URLs**

``` prism-code
import { SciChartSurface, libraryVersion } from "scichart";
// Load Wasm from URL
// This URL can be anything, but for example purposes we are loading from JSDelivr CDN
SciChartSurface.configure({
   wasmUrl: `https://cdn.jsdelivr.net/npm/scichart@${libraryVersion}/_wasm/scichart2d.wasm`
});
```

We've packaged a helpful function that automatically loads the latest & correct version of SciChart's Wasm & Data files from CDN. To use this, instead of callingÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#configure" rel="noopener noreferrer" target="_blank">SciChartSurface.configure()ðŸ“˜</a>Â passing in a URL, call <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#usewasmfromcdn" rel="noopener noreferrer" target="_blank">SciChartSurface.useWasmFromCDN()ðŸ“˜</a>.

**Load Wasm from CDN**

``` prism-code
import { SciChartSurface } from "scichart";

export async function initSciChart() {
    // Call this once before any SciChartSurface is shown.
    // This is equivalent to calling SciChartSurface.configure() with the CDN URL (JSDelivr)
    SciChartSurface.useWasmFromCDN();
}
```

## Loading Wasm files offline<a href="https://www.scichart.com/documentation/js/v4/2d-charts/surface/deploying-wasm/#loading-wasm-files-offline" class="hash-link" aria-label="Direct link to Loading Wasm files offline" translate="no" title="Direct link to Loading Wasm files offline">â€‹</a>

If your application must load wasm files offline (does not have an internet connection), you can download the files and serve them and useÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#configure" rel="noopener noreferrer" target="_blank">SciChartSurface.configure()ðŸ“˜</a> to fetch the local file.

To find out how to do this, seeÂ [Tutorial 02 - Including index.min.js and WebAssembly Files offline](https://www.scichart.com/documentation/js/v4/get-started/tutorials-cdn/tutorial-02-offline).

## Loading Wasm for 3D Charts<a href="https://www.scichart.com/documentation/js/v4/2d-charts/surface/deploying-wasm/#loading-wasm-for-3d-charts" class="hash-link" aria-label="Direct link to Loading Wasm for 3D Charts" translate="no" title="Direct link to Loading Wasm for 3D Charts">â€‹</a>

The process for loading Wasm files for 3D Charts is exactly the same, except you must configure SciChart.js to load scichart3d.wasm.

This can be done via Webpack/npm or a bundlers, or by callingÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#usewasmfromcdn" rel="noopener noreferrer" target="_blank">SciChart3DSurface.useWasmFromCDN()ðŸ“˜</a> orÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#configure" rel="noopener noreferrer" target="_blank">SciChart3DSurface.configure()ðŸ“˜</a> as before.

**Configure Wasm and Data File URLs**

``` prism-code
import { SciChartSurface } from "scichart";

// Call this once before any SciChart3DSurface is shown to load wasm from CDN
SciChart3DSurface.useWasmFromCDN();
// Alternatively, if you want to host wasm files and serve them locally
// 1.) Ensure you are serving your wasm files
// 2.) Call to SciChart3DSurface.configure specifying the relative URL of the files
SciChart3DSurface.configure({
   wasmUrl: `relative/path/to/scichart3d.wasm`
});
```

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/surface/deploying-wasm/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/surface/deploying-wasm/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
