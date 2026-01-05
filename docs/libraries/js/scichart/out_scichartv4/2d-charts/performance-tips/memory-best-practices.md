On this page

# Memory Best Practices

Related toÂ [Performance Tips](https://www.scichart.com/documentation/js/v4/2d-charts/performance-tips/performance-tips-and-tricks/), this article covers Memory Best Practices in SciChart.js.

## Wasm Memory vs. JavaScript Memory Model<a href="https://www.scichart.com/documentation/js/v4/2d-charts/performance-tips/memory-best-practices/#wasm-memory-vs-javascript-memory-model" class="hash-link" aria-label="Direct link to Wasm Memory vs. JavaScript Memory Model" translate="no" title="Direct link to Wasm Memory vs. JavaScript Memory Model">â€‹</a>

SciChart.js uses WebAssembly, also known as Wasm, to achieve high performance 2D/3D chart rendering in the browser. Using Wasm, we've compiled our in-house C++ codebase, tested in enterprise and embedded environments for many years on Windows and mobile into a JavaScript friendly library.

A key difference between Wasm and JavaScript is that JS has a garbage collector and Wasm does not. Any type created in WebAssembly must be expliclty deleted to reclaim memory. Below we have some guidelines on how to do that, and what to do to detect problems.

![](out_scichartv4/2d-charts/performance-tips/memory-best-practices/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

Failure to call .delete() on a WebAssembly type can result in a memory leak. But don't worry, we haveÂ [memory debugging tools](https://www.scichart.com/documentation/js/v4/2d-charts/performance-tips/memory-best-practices/) to help track that.

### Deletable Entities in SciChart.js<a href="https://www.scichart.com/documentation/js/v4/2d-charts/performance-tips/memory-best-practices/#deletable-entities-in-scichartjs" class="hash-link" aria-label="Direct link to Deletable Entities in SciChart.js" translate="no" title="Direct link to Deletable Entities in SciChart.js">â€‹</a>

A number of types in SciChart.js implement theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html" rel="noopener noreferrer" target="_blank">IDeletableðŸ“˜</a> interface. This has a single function:Â **<a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html#delete" rel="noopener noreferrer" target="_blank">.delete()ðŸ“˜</a>** which deletes underlying WebAssembly memory.

The following types implement IDeletable in SciChart.js:

- **SciChartSurface**
- **SciChart3DSurface**
- Axis types e.g **NumericAxis**, **CategoryAxis**
- Series types e.g. **FastLineRenderableSeries**
- DataSeries types e.g. **XyDataSeries**
- Annotations e.g. **LineAnnotation**
- Miscellaneous types e.g. **EllipsePointMarker**

When calling delete on a parent, all child items are deleted so to properly clean-up an entire chart, you must only callÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#delete" rel="noopener noreferrer" target="_blank">sciChartSurface.delete()ðŸ“˜</a> once. The best place to do this is component unmount in React, Vue or onDestroy in Angular.

## Best Practices to call .delete()<a href="https://www.scichart.com/documentation/js/v4/2d-charts/performance-tips/memory-best-practices/#best-practices-to-call-delete" class="hash-link" aria-label="Direct link to Best Practices to call .delete()" translate="no" title="Direct link to Best Practices to call .delete()">â€‹</a>

### Deleting Charts on Component Unmount<a href="https://www.scichart.com/documentation/js/v4/2d-charts/performance-tips/memory-best-practices/#deleting-charts-on-component-unmount" class="hash-link" aria-label="Direct link to Deleting Charts on Component Unmount" translate="no" title="Direct link to Deleting Charts on Component Unmount">â€‹</a>

Deleting a ScIChartSurface by callingÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#delete" rel="noopener noreferrer" target="_blank">sciChartSurface.delete()ðŸ“˜</a> once on component unmount will cascade down to all child objects such as DataSeries, Annotations and RenderableSeries.

![](out_scichartv4/2d-charts/performance-tips/memory-best-practices/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

React, Vue and Angular all have component unmount or onDestroy callbacks. In our React, Vue and Angular boilerplates atÂ <a href="https://www.scichart.com/getting-started/scichart-javascript/" rel="noopener noreferrer" target="_blank">scichart.com/getting-started-scichart-js</a> we show you how to do this. We've also created a [Tutorial for Creating a Reusable React Component](https://www.scichart.com/documentation/js/v4/get-started/tutorials-react/tutorial-01-setting-up-project-with-scichart-react/) showing you how to correctly delete the chart on unmount.

### Deleting DataSeries memory<a href="https://www.scichart.com/documentation/js/v4/2d-charts/performance-tips/memory-best-practices/#deleting-dataseries-memory" class="hash-link" aria-label="Direct link to Deleting DataSeries memory" translate="no" title="Direct link to Deleting DataSeries memory">â€‹</a>

IfÂ you remove or re-assign a DataSeries from a chart (use case: adding or removing chart series), don't forget to callÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html#delete" rel="noopener noreferrer" target="_blank">series.delete()ðŸ“˜</a>. This frees WebAssembly native memory and releases it back to the browser.

For some examples of how to do this, see the articleÂ [Deleting DataSeries Memory](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-series-api/deleting-memory/).

### Deleting RendeableSeries, Annotations or Modifiers<a href="https://www.scichart.com/documentation/js/v4/2d-charts/performance-tips/memory-best-practices/#deleting-rendeableseries-annotations-or-modifiers" class="hash-link" aria-label="Direct link to Deleting RendeableSeries, Annotations or Modifiers" translate="no" title="Direct link to Deleting RendeableSeries, Annotations or Modifiers">â€‹</a>

RenderableSeries, Annotations and AxisÂ must also be deleted in the case where you remove, or reassign series on a chart. Some code examples of how to do this can alsoÂ be seen in the pageÂ [Deleting DataSeries Memory](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-series-api/deleting-memory/).

### Detecting Leaks by forgetting to call .delete()<a href="https://www.scichart.com/documentation/js/v4/2d-charts/performance-tips/memory-best-practices/#detecting-leaks-by-forgetting-to-call-delete" class="hash-link" aria-label="Direct link to Detecting Leaks by forgetting to call .delete()" translate="no" title="Direct link to Detecting Leaks by forgetting to call .delete()">â€‹</a>

Suspect a memory leak? If you forget to call .delete(), don't worry! We have a set of tools to detect memory leaks and inform you of which objects need to be properly cleaned up. See the articleÂ [Memory Leak Debugging](https://www.scichart.com/documentation/js/v4/2d-charts/performance-tips/memory-leak-debugging/) for more info.

## Other Memory Optimisations / Footnotes<a href="https://www.scichart.com/documentation/js/v4/2d-charts/performance-tips/memory-best-practices/#other-memory-optimisations--footnotes" class="hash-link" aria-label="Direct link to Other Memory Optimisations / Footnotes" translate="no" title="Direct link to Other Memory Optimisations / Footnotes">â€‹</a>

### Memory Differences between create() and createSingle()<a href="https://www.scichart.com/documentation/js/v4/2d-charts/performance-tips/memory-best-practices/#memory-differences-between-create-and-createsingle" class="hash-link" aria-label="Direct link to Memory Differences between create() and createSingle()" translate="no" title="Direct link to Memory Differences between create() and createSingle()">â€‹</a>

There are two factory functions to create a chart:Â <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#create" rel="noopener noreferrer" target="_blank">SciChartSurface.create()ðŸ“˜</a> andÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#createsingle" rel="noopener noreferrer" target="_blank">SciChartSurface.createSingle()ðŸ“˜</a>.

- WhenÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#create" rel="noopener noreferrer" target="_blank">SciChartSurface.create()ðŸ“˜</a> is called, a single shared instance of our WebGL graphics engine is instantiated with a single one-time memory cost.
- WhenÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#createsingle" rel="noopener noreferrer" target="_blank">SciChartSurface.createSingle()ðŸ“˜</a> is called, one instance of our WebGL engine is instantiated per-chart, which uses additional RAM per chart.

Therefore,

- Charts created with **create()** have lower memory usage but marginally slower drawing performance.
- Charts created with **createSingle()** have higher memory usage but faster drawing performance. They are also subject to WebGL context limitsÂ [specified here](https://www.scichart.com/documentation/js/v4/2d-charts/performance-tips/performance-tips-and-tricks).

### Completely Releasing Memory back to the OS<a href="https://www.scichart.com/documentation/js/v4/2d-charts/performance-tips/memory-best-practices/#completely-releasing-memory-back-to-the-os" class="hash-link" aria-label="Direct link to Completely Releasing Memory back to the OS" translate="no" title="Direct link to Completely Releasing Memory back to the OS">â€‹</a>

When deleting charts created withÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#createsingle" rel="noopener noreferrer" target="_blank">SciChartSurface.createSingle()ðŸ“˜</a>, and when the last reference to sciChartSurface falls out of scope, the JavaScript Garbage Collector releases all memory held by WebAssembly and JavaScript for this chart back to the host OS.

When deleting charts created withÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#create" rel="noopener noreferrer" target="_blank">SciChartSurface.create()ðŸ“˜</a>, and when the last reference to all SciChartSurfaces fall out of scope, the JavaScript Garbage Collector will release all JS/Wasm memory only if the following flag is set.

``` prism-code
// Set static properties to define memory cleanup behaviour after the last chart is garbage collected
SciChartSurface.autoDisposeWasmContext = true; // default false
SciChartSurface.wasmContextDisposeTimeout = 100; // default 0 mlliseconds
```

This default behaviour is there to avoid unintential garbage collection and re-instanitation of our WebGL engine on switching page in a Single Page Application. SettingÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#autodisposewasmcontext" rel="noopener noreferrer" target="_blank">SciChartSurface.autoDisposeWasmContextðŸ“˜</a> to true and a suitable non-zero timeout gives SPA apps change to switch page before automatically cleaning up WebAssembly memory.

#### Summary of Options to Force Wasm Context disposal<a href="https://www.scichart.com/documentation/js/v4/2d-charts/performance-tips/memory-best-practices/#summary-of-options-to-force-wasm-context-disposal" class="hash-link" aria-label="Direct link to Summary of Options to Force Wasm Context disposal" translate="no" title="Direct link to Summary of Options to Force Wasm Context disposal">â€‹</a>

The Wasm engine can be disposed in several ways to release memory back to the host OS.

- for charts instantiated withÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#createsingle" rel="noopener noreferrer" target="_blank">SciChartSurface.createSingle()ðŸ“˜</a> wasmContext is destroyed automatically upon surface deletion and nullifying all of the references to the SciChartSurface/wasmContext.
- for charts instantiated withÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#create" rel="noopener noreferrer" target="_blank">SciChartSurface.create()ðŸ“˜</a> wasmContext is not destroyed automatically by default; to destroy it
  - callÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#disposesharedwasmcontext" rel="noopener noreferrer" target="_blank">SciChartSurface.disposeSharedWasmContext()ðŸ“˜</a>
  - or enable auto disposal after a specified timeout usingÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#autodisposewasmcontext" rel="noopener noreferrer" target="_blank">SciChartSurface.autoDisposeWasmContextðŸ“˜</a>Â flag andÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#wasmcontextdisposetimeout" rel="noopener noreferrer" target="_blank">SciChartSurface.wasmContextDisposeTimeoutðŸ“˜</a>Â property.

### Setting WebGL Buffer Sizes<a href="https://www.scichart.com/documentation/js/v4/2d-charts/performance-tips/memory-best-practices/#setting-webgl-buffer-sizes" class="hash-link" aria-label="Direct link to Setting WebGL Buffer Sizes" translate="no" title="Direct link to Setting WebGL Buffer Sizes">â€‹</a>

We're into esoteric memory improvements here, but this small optimisation is worth mentioning.

If you are operating in an extremely memory constrained environment and every megabyte counts, you can trade drawing performance for lower memory by adjusting maximum WebGL buffer size. To do this, setÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartdefaults.html#wasmbuffersizeskb" rel="noopener noreferrer" target="_blank">SciChartDefaults.wasmBufferSizesKbðŸ“˜</a> once in your app before showing a chart.

``` prism-code
// Specifies the maximum buffer size to use PER PRIMITIVE TYPE
// If all chart types are shown in application, theoretical maximum = 8 x wasmBufferSizesKb
SciChartDefaults.wasmBufferSizesKb = 1024;

// minimum = 1024 kb
// default = 8192 kb
// maximum = 32,768 kb
```

### Debugging Memory Leaks when using SciChart.js<a href="https://www.scichart.com/documentation/js/v4/2d-charts/performance-tips/memory-best-practices/#debugging-memory-leaks-when-using-scichartjs" class="hash-link" aria-label="Direct link to Debugging Memory Leaks when using SciChart.js" translate="no" title="Direct link to Debugging Memory Leaks when using SciChart.js">â€‹</a>

We've created a set of tools and a guide on how to debug and eliminate memory leaks in your application when using SciChart.js. Find out more atÂ [Memory Leak Debugging](https://www.scichart.com/documentation/js/v4/2d-charts/performance-tips/memory-leak-debugging/).

### Best Practies when using SciChart.js in React Components<a href="https://www.scichart.com/documentation/js/v4/2d-charts/performance-tips/memory-best-practices/#best-practies-when-using-scichartjs-in-react-components" class="hash-link" aria-label="Direct link to Best Practies when using SciChart.js in React Components" translate="no" title="Direct link to Best Practies when using SciChart.js in React Components">â€‹</a>

Additional guidelines on the best practices including memory cleanup when using SciChart.js in React Components can be found at the pageÂ [Tutorial - Creating a Reusable React Component](https://www.scichart.com/documentation/js/v4/get-started/tutorials-react/tutorial-01-setting-up-project-with-scichart-react/).

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/performance-tips/memory-best-practices/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/performance-tips/memory-best-practices/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
