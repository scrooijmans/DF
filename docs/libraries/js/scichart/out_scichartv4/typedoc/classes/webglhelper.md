<img src="out_scichartv4/typedoc/classes/webglhelper_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/webglhelper.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [WebGlHelper](https://www.scichart.com/documentation/js/v4/typedoc/classes/webglhelper.html)

# Class WebGlHelper

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

Helper class to extract WebGL1/2 detection and context creation

### Hierarchy

- WebGlHelper

## Index

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/webglhelper.html#getcontext" class="tsd-kind-icon">getContext</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/webglhelper.html#getwebglsupport" class="tsd-kind-icon">getWebGlSupport</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/webglhelper.html#initialize" class="tsd-kind-icon">initialize</a>

## Methods

### Static getContext

- getContext(canvas: HTMLCanvasElement, options?: WebGLContextAttributes): WebGLRenderingContext \| null

<!-- -->

- Calls canvas.getContext() passing in "webgl" or "webgl2" depending on current browser WebGL Support.

  #### Parameters

  - ##### canvas: HTMLCanvasElement

  - ##### Optional options: WebGLContextAttributes

  #### Returns WebGLRenderingContext \| null

### Static getWebGlSupport

- getWebGlSupport(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/ewebglsupport.html" class="tsd-signature-type">EWebGLSupport</a>

<!-- -->

- Gets the WebGL support by the browser. See [EWebGLSupport](https://www.scichart.com/documentation/js/v4/typedoc/enums/ewebglsupport.html) for options

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/ewebglsupport.html" class="tsd-signature-type">EWebGLSupport</a>

### Static initialize

- initialize(): void

<!-- -->

- Static initializer function. Is called once by the framework on startup

  #### Returns void

## Legend

- Constructor
- Property
- Method
- Accessor

<!-- -->

- Inherited constructor
- Inherited property
- Inherited method
- Inherited accessor

<!-- -->

- Property
- Method

<!-- -->

- Protected property
- Protected method

<!-- -->

- Static property
- Static method

Generated using <a href="https://typedoc.org/" target="_blank">TypeDoc</a>
