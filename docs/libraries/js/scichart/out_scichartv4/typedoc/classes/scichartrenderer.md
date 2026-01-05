<img src="out_scichartv4/typedoc/classes/scichartrenderer_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartrenderer.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [SciChartRenderer](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartrenderer.html)

# Class SciChartRenderer

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

A class used internally in SciChart to perform layout, arrangement, data-preparation and rendering on the Cartesian 2D [SciChartSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html)

### Hierarchy

- SciChartRenderer

## Index

### Constructors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartrenderer.html#constructor" class="tsd-kind-icon">constructor</a>

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartrenderer.html#isinvalidated" class="tsd-kind-icon">isInvalidated</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartrenderer.html#prevrect" class="tsd-kind-icon">prevRect</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartrenderer.html#prevsurfacerect" class="tsd-kind-icon">prevSurfaceRect</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartrenderer.html#previoustime" class="tsd-kind-icon">previousTime</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartrenderer.html#scichartsurface" class="tsd-kind-icon">sciChartSurface</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartrenderer.html#displayerrormessage" class="tsd-kind-icon">displayErrorMessage</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartrenderer.html#enqueuerenderable" class="tsd-kind-icon">enqueueRenderable</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartrenderer.html#enqueuerenderables" class="tsd-kind-icon">enqueueRenderables</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartrenderer.html#getannotationdrawfunctions" class="tsd-kind-icon">getAnnotationDrawFunctions</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartrenderer.html#gethtmlannotationdrawfunction" class="tsd-kind-icon">getHTMLAnnotationDrawFunction</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartrenderer.html#render" class="tsd-kind-icon">render</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartrenderer.html#resizeannotationrootelements" class="tsd-kind-icon">resizeAnnotationRootElements</a>

## Constructors

### constructor

- new SciChartRenderer(sciChartSurface: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html" class="tsd-signature-type">SciChartSurface</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartrenderer.html" class="tsd-signature-type">SciChartRenderer</a>

<!-- -->

- Creates an instance of the SciChartRenderer

  #### Parameters

  - ##### sciChartSurface: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html" class="tsd-signature-type">SciChartSurface</a>

    The [SciChartSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html) that we are rendering

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartrenderer.html" class="tsd-signature-type">SciChartRenderer</a>

## Properties

### isInvalidated

isInvalidated: boolean = false

### Protected prevRect

prevRect: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>

### Protected prevSurfaceRect

prevSurfaceRect: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>

### Protected previousTime

previousTime: number

### Protected sciChartSurface

sciChartSurface: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html" class="tsd-signature-type">SciChartSurface</a>

## Methods

### Protected displayErrorMessage

- displayErrorMessage(): void

<!-- -->

- #### Returns void

### Protected enqueueRenderable

- enqueueRenderable(renderContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/webglrendercontext2d.html" class="tsd-signature-type">WebGlRenderContext2D</a>, item: TRenderable, absoluteLayer: number): void

<!-- -->

- #### Parameters

  - ##### renderContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/webglrendercontext2d.html" class="tsd-signature-type">WebGlRenderContext2D</a>

  - ##### item: TRenderable

  - ##### absoluteLayer: number

  #### Returns void

### Protected enqueueRenderables

- enqueueRenderables(renderContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/webglrendercontext2d.html" class="tsd-signature-type">WebGlRenderContext2D</a>, renderPassInfo: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassinfo.html" class="tsd-signature-type">RenderPassInfo</a>): void

<!-- -->

- #### Parameters

  - ##### renderContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/webglrendercontext2d.html" class="tsd-signature-type">WebGlRenderContext2D</a>

  - ##### renderPassInfo: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassinfo.html" class="tsd-signature-type">RenderPassInfo</a>

  #### Returns void

### Protected getAnnotationDrawFunctions

- getAnnotationDrawFunctions(scs: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html" class="tsd-signature-type">SciChartSurface</a>, renderPassInfo: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassinfo.html" class="tsd-signature-type">RenderPassInfo</a>, renderContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/webglrendercontext2d.html" class="tsd-signature-type">WebGlRenderContext2D</a>): void

<!-- -->

- #### Parameters

  - ##### scs: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html" class="tsd-signature-type">SciChartSurface</a>

  - ##### renderPassInfo: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassinfo.html" class="tsd-signature-type">RenderPassInfo</a>

  - ##### renderContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/webglrendercontext2d.html" class="tsd-signature-type">WebGlRenderContext2D</a>

  #### Returns void

### Protected getHTMLAnnotationDrawFunction

- getHTMLAnnotationDrawFunction(annotation: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/domannotationbase.html" class="tsd-signature-type">DomAnnotationBase</a>, coordSvgTranslation: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/point.html" class="tsd-signature-type">Point</a>): void

<!-- -->

- #### Parameters

  - ##### annotation: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/domannotationbase.html" class="tsd-signature-type">DomAnnotationBase</a>

  - ##### coordSvgTranslation: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/point.html" class="tsd-signature-type">Point</a>

  #### Returns void

### render

- render(renderContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/webglrendercontext2d.html" class="tsd-signature-type">WebGlRenderContext2D</a>): void

<!-- -->

- Render loop for the current {@SciChartSurface}

  #### Parameters

  - ##### renderContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/webglrendercontext2d.html" class="tsd-signature-type">WebGlRenderContext2D</a>

    the {@WebGLRenderContext2D} used for drawing

  #### Returns void

### Protected resizeAnnotationRootElements

- resizeAnnotationRootElements(seriesViewRect: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>): void

<!-- -->

- #### Parameters

  - ##### seriesViewRect: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>

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
