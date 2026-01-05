<img src="out_scichartv4/typedoc/classes/scichart3drenderer_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3drenderer.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [SciChart3DRenderer](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3drenderer.html)

# Class SciChart3DRenderer

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

A class used internally in SciChart to perform layout, arrangement, data-preparation and rendering on the Cartesian 3D [SciChart3DSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html)

### Hierarchy

- SciChart3DRenderer

## Index

### Constructors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3drenderer.html#constructor" class="tsd-kind-icon">constructor</a>

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3drenderer.html#isinvalidated" class="tsd-kind-icon">isInvalidated</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3drenderer.html#render" class="tsd-kind-icon">render</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3drenderer.html#getscenedescriptor" class="tsd-kind-icon">getSceneDescriptor</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3drenderer.html#preparerenderdata" class="tsd-kind-icon">prepareRenderData</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3drenderer.html#tryperformautorangeon" class="tsd-kind-icon">tryPerformAutoRangeOn</a>

## Constructors

### constructor

- new SciChart3DRenderer(scs: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html" class="tsd-signature-type">SciChart3DSurface</a>, wasmContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart3d" class="tsd-signature-type">TSciChart3D</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3drenderer.html" class="tsd-signature-type">SciChart3DRenderer</a>

<!-- -->

- #### Parameters

  - ##### scs: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html" class="tsd-signature-type">SciChart3DSurface</a>

  - ##### wasmContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart3d" class="tsd-signature-type">TSciChart3D</a>

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3drenderer.html" class="tsd-signature-type">SciChart3DRenderer</a>

## Properties

### isInvalidated

isInvalidated: boolean = false

## Methods

### render

- render(): void

<!-- -->

- The main render loop

  #### Returns void

### Static getSceneDescriptor

- getSceneDescriptor(scs: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html" class="tsd-signature-type">SciChart3DSurface</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scenedescriptor.html" class="tsd-signature-type">SceneDescriptor</a>

<!-- -->

- get the [SceneDescriptor](https://www.scichart.com/documentation/js/v4/typedoc/classes/scenedescriptor.html) to define the look & styling of the scene in the current render pass

  #### Parameters

  - ##### scs: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html" class="tsd-signature-type">SciChart3DSurface</a>

    the [SciChart3DSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html) we are drawing

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scenedescriptor.html" class="tsd-signature-type">SceneDescriptor</a>

### Static prepareRenderData

- prepareRenderData(scs: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html" class="tsd-signature-type">SciChart3DSurface</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassinfo3d.html" class="tsd-signature-type">RenderPassInfo3D</a>

<!-- -->

- Prepares render data and returns a [RenderPassInfo3D](https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassinfo3d.html) for the current render pass

  #### Parameters

  - ##### scs: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html" class="tsd-signature-type">SciChart3DSurface</a>

    the [SciChart3DSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html) we are drawing

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassinfo3d.html" class="tsd-signature-type">RenderPassInfo3D</a>

### Static tryPerformAutoRangeOn

- tryPerformAutoRangeOn(axis: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase3d.html" class="tsd-signature-type">AxisBase3D</a>, scs: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html" class="tsd-signature-type">SciChart3DSurface</a>): void

<!-- -->

- Performs autorange on the [AxisBase3D](https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase3d.html) depending on flags such as [AxisBase3D.autoRange](https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase3d.html#autorange)

  #### Parameters

  - ##### axis: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase3d.html" class="tsd-signature-type">AxisBase3D</a>

    The [AxisBase3D](https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase3d.html) we are auto-ranging

  - ##### scs: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html" class="tsd-signature-type">SciChart3DSurface</a>

    the [SciChart3DSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html) we are drawing

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
