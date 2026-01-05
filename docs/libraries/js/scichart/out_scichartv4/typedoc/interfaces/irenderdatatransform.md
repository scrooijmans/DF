<img src="out_scichartv4/typedoc/interfaces/irenderdatatransform_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderdatatransform.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [IRenderDataTransform](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderdatatransform.html)

# Interface IRenderDataTransform

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

### Hierarchy

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html" class="tsd-signature-type">IDeletable</a>
  - IRenderDataTransform

### Implemented by

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderdatatransform.html" class="tsd-signature-type">BaseRenderDataTransform</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderdatatransformjs.html" class="tsd-signature-type">BaseRenderDataTransformJS</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/bezierrenderdatatransform.html" class="tsd-signature-type">BezierRenderDataTransform</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/ohlcbaserenderdatatransform.html" class="tsd-signature-type">OhlcBaseRenderDataTransform</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarinterpolatebandrenderdatatransform.html" class="tsd-signature-type">PolarInterpolateBandRenderDataTransform</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarinterpolatelinerenderdatatransform.html" class="tsd-signature-type">PolarInterpolateLineRenderDataTransform</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/smoothstackedrenderdatatransform.html" class="tsd-signature-type">SmoothStackedRenderDataTransform</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/splinerenderdatatransform.html" class="tsd-signature-type">SplineRenderDataTransform</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xybaserenderdatatransform.html" class="tsd-signature-type">XyBaseRenderDataTransform</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyybaserenderdatatransform.html" class="tsd-signature-type">XyyBaseRenderDataTransform</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyybezierrenderdatatransform.html" class="tsd-signature-type">XyyBezierRenderDataTransform</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyysplinerenderdatatransform.html" class="tsd-signature-type">XyySplineRenderDataTransform</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyzbaserenderdatatransform.html" class="tsd-signature-type">XyzBaseRenderDataTransform</a>

## Index

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderdatatransform.html#drawingproviders" class="tsd-kind-icon">drawingProviders</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderdatatransform.html#parentseries" class="tsd-kind-icon">parentSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderdatatransform.html#pointseries" class="tsd-kind-icon">pointSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderdatatransform.html#requirestransform" class="tsd-kind-icon">requiresTransform</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderdatatransform.html#useforyrange" class="tsd-kind-icon">useForYRange</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderdatatransform.html#delete" class="tsd-kind-icon">delete</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderdatatransform.html#ondatachange" class="tsd-kind-icon">onDataChange</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderdatatransform.html#runtransform" class="tsd-kind-icon">runTransform</a>

## Properties

### drawingProviders

drawingProviders: Array\<<a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iseriesdrawingprovider.html" class="tsd-signature-type">ISeriesDrawingProvider</a>\>

The drawingProviders on the parentSeries to which this transform applies

### Readonly parentSeries

parentSeries: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries.html" class="tsd-signature-type">BaseRenderableSeries</a>

The series the transform is attached to

### pointSeries

pointSeries: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basepointseriesresampled.html" class="tsd-signature-type">BasePointSeriesResampled</a>

The pointSeries that stores the result of the transform

### requiresTransform

requiresTransform: boolean

Whether then transform will run when the series is drawn. This is set true initially, and when data changes, and is set to false when the transform does run. If your transform depends on any other property, you must set this true if that property changes

### useForYRange

useForYRange: boolean

A flag to tell the parent series if the transformed values should be used when calculating data range

## Methods

### delete

- delete(): void

<!-- -->

- Deletes native (WebAssembly) memory used by this type, after which it cannot be used.

  remarks  
  Call .delete() before finishing with the object to ensure that WebAssmembly memory leaks do not occur.

  All elements within SciChart's High Performance <a href="https://www.scichart.com/javascript-chart-features" class="external">Realtime JavaScript Charts</a> which implement [IDeletable](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html) must be deleted manually to free native (WebAssembly) memory

  #### Returns void

### onDataChange

- onDataChange(args: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idatachangeargs.html" class="tsd-signature-type">IDataChangeArgs</a>): void

<!-- -->

- #### Parameters

  - ##### args: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idatachangeargs.html" class="tsd-signature-type">IDataChangeArgs</a>

  #### Returns void

### runTransform

- runTransform(renderPassData: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassdata.html" class="tsd-signature-type">RenderPassData</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassdata.html" class="tsd-signature-type">RenderPassData</a>

<!-- -->

- The method that is called to run the transform. Do not override this, instead override runTransformInternal on [BaseRenderDataTransform](https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderdatatransform.html)

  #### Parameters

  - ##### renderPassData: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassdata.html" class="tsd-signature-type">RenderPassData</a>

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassdata.html" class="tsd-signature-type">RenderPassData</a>

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
