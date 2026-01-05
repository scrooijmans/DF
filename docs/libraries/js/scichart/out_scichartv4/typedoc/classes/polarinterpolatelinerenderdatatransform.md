<img src="out_scichartv4/typedoc/classes/polarinterpolatelinerenderdatatransform_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarinterpolatelinerenderdatatransform.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [PolarInterpolateLineRenderDataTransform](https://www.scichart.com/documentation/js/v4/typedoc/classes/polarinterpolatelinerenderdatatransform.html)

# Class PolarInterpolateLineRenderDataTransform

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

### Hierarchy

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderdatatransform.html" class="tsd-signature-type">BaseRenderDataTransform</a>\<<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xypointseriesresampled.html" class="tsd-signature-type">XyPointSeriesResampled</a>\>
  - PolarInterpolateLineRenderDataTransform

### Implements

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html" class="tsd-signature-type">IDeletable</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderdatatransform.html" class="tsd-signature-type">IRenderDataTransform</a>

## Index

### Constructors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarinterpolatelinerenderdatatransform.html#constructor" class="tsd-kind-icon">constructor</a>

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarinterpolatelinerenderdatatransform.html#drawingproviders" class="tsd-kind-icon">drawingProviders</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarinterpolatelinerenderdatatransform.html#lastindexrange" class="tsd-kind-icon">lastIndexRange</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarinterpolatelinerenderdatatransform.html#lastresamplinghash" class="tsd-kind-icon">lastResamplingHash</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarinterpolatelinerenderdatatransform.html#parentseries" class="tsd-kind-icon">parentSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarinterpolatelinerenderdatatransform.html#pointseries" class="tsd-kind-icon">pointSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarinterpolatelinerenderdatatransform.html#requirestransform" class="tsd-kind-icon">requiresTransform</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarinterpolatelinerenderdatatransform.html#useforyrange" class="tsd-kind-icon">useForYRange</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarinterpolatelinerenderdatatransform.html#wasmcontext" class="tsd-kind-icon">wasmContext</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarinterpolatelinerenderdatatransform.html#createpointseries" class="tsd-kind-icon">createPointSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarinterpolatelinerenderdatatransform.html#delete" class="tsd-kind-icon">delete</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarinterpolatelinerenderdatatransform.html#makerenderpassdata" class="tsd-kind-icon">makeRenderPassData</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarinterpolatelinerenderdatatransform.html#ondatachange" class="tsd-kind-icon">onDataChange</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarinterpolatelinerenderdatatransform.html#runtransform" class="tsd-kind-icon">runTransform</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarinterpolatelinerenderdatatransform.html#runtransforminternal" class="tsd-kind-icon">runTransformInternal</a>

## Constructors

### constructor

- new PolarInterpolateLineRenderDataTransform(parentSeries: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries.html" class="tsd-signature-type">BaseRenderableSeries</a>, wasmContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart" class="tsd-signature-type">TSciChart</a>, drawingProviders?: Array\<<a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iseriesdrawingprovider.html" class="tsd-signature-type">ISeriesDrawingProvider</a>\>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarinterpolatelinerenderdatatransform.html" class="tsd-signature-type">PolarInterpolateLineRenderDataTransform</a>

<!-- -->

- #### Parameters

  - ##### parentSeries: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries.html" class="tsd-signature-type">BaseRenderableSeries</a>

  - ##### wasmContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart" class="tsd-signature-type">TSciChart</a>

  - ##### Optional drawingProviders: Array\<<a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iseriesdrawingprovider.html" class="tsd-signature-type">ISeriesDrawingProvider</a>\>

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarinterpolatelinerenderdatatransform.html" class="tsd-signature-type">PolarInterpolateLineRenderDataTransform</a>

## Properties

### Readonly drawingProviders

drawingProviders: Array\<<a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iseriesdrawingprovider.html" class="tsd-signature-type">ISeriesDrawingProvider</a>\>

The drawingProviders on the parentSeries to which this transform applies

### Protected lastIndexRange

lastIndexRange: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

### Protected lastResamplingHash

lastResamplingHash: number

### Readonly parentSeries

parentSeries: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries.html" class="tsd-signature-type">BaseRenderableSeries</a>

The series the transform is attached to

### pointSeries

pointSeries: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xypointseriesresampled.html" class="tsd-signature-type">XyPointSeriesResampled</a>

The pointSeries that stores the result of the transform

### requiresTransform

requiresTransform: boolean = true

Whether then transform will run when the series is drawn. This is set true initially, and when data changes, and is set to false when the transform does run. If your transform depends on any other property, you must set this true if that property changes

### useForYRange

useForYRange: boolean = false

A flag to tell the parent series if the transformed values should be used when calculating data range

### Protected wasmContext

wasmContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart" class="tsd-signature-type">TSciChart</a>

## Methods

### Protected createPointSeries

- createPointSeries(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xypointseriesresampled.html" class="tsd-signature-type">XyPointSeriesResampled</a>

<!-- -->

- #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xypointseriesresampled.html" class="tsd-signature-type">XyPointSeriesResampled</a>

### delete

- delete(): void

<!-- -->

- #### Returns void

### Protected makeRenderPassData

- makeRenderPassData(originalRPD: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassdata.html" class="tsd-signature-type">RenderPassData</a>, pointSeries: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointseries.html" class="tsd-signature-type">IPointSeries</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassdata.html" class="tsd-signature-type">RenderPassData</a>

<!-- -->

- #### Parameters

  - ##### originalRPD: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassdata.html" class="tsd-signature-type">RenderPassData</a>

  - ##### pointSeries: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointseries.html" class="tsd-signature-type">IPointSeries</a>

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassdata.html" class="tsd-signature-type">RenderPassData</a>

### onDataChange

- onDataChange(args: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idatachangeargs.html" class="tsd-signature-type">IDataChangeArgs</a>): void

<!-- -->

- #### Parameters

  - ##### args: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idatachangeargs.html" class="tsd-signature-type">IDataChangeArgs</a>

  #### Returns void

### runTransform

- runTransform(renderPassData: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassdata.html" class="tsd-signature-type">RenderPassData</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassdata.html" class="tsd-signature-type">RenderPassData</a>

<!-- -->

- #### Parameters

  - ##### renderPassData: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassdata.html" class="tsd-signature-type">RenderPassData</a>

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassdata.html" class="tsd-signature-type">RenderPassData</a>

### Protected runTransformInternal

- runTransformInternal(renderPassData: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassdata.html" class="tsd-signature-type">RenderPassData</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointseries.html" class="tsd-signature-type">IPointSeries</a>

<!-- -->

- #### Parameters

  - ##### renderPassData: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassdata.html" class="tsd-signature-type">RenderPassData</a>

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointseries.html" class="tsd-signature-type">IPointSeries</a>

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
