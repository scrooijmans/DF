<img src="out_scichartv4/typedoc/classes/xyybezierrenderdatatransform_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyybezierrenderdatatransform.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [XyyBezierRenderDataTransform](https://www.scichart.com/documentation/js/v4/typedoc/classes/xyybezierrenderdatatransform.html)

# Class XyyBezierRenderDataTransform

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

A RenderDataTransform that calculates a Cubic Bezier curve over the an Xyy dataSeries, while respecting the bounds of the data.

### Hierarchy

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderdatatransform.html" class="tsd-signature-type">BaseRenderDataTransform</a>\<<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyypointseriesresampled.html" class="tsd-signature-type">XyyPointSeriesResampled</a>\>
  - XyyBezierRenderDataTransform
    - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/smoothstackedrenderdatatransform.html" class="tsd-signature-type">SmoothStackedRenderDataTransform</a>

### Implements

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html" class="tsd-signature-type">IDeletable</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderdatatransform.html" class="tsd-signature-type">IRenderDataTransform</a>

## Index

### Constructors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyybezierrenderdatatransform.html#constructor" class="tsd-kind-icon">constructor</a>

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyybezierrenderdatatransform.html#drawingproviders" class="tsd-kind-icon">drawingProviders</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyybezierrenderdatatransform.html#forceygreaterthany1" class="tsd-kind-icon">forceYGreaterThanY1</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyybezierrenderdatatransform.html#lastindexrange" class="tsd-kind-icon">lastIndexRange</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyybezierrenderdatatransform.html#lastresamplinghash" class="tsd-kind-icon">lastResamplingHash</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyybezierrenderdatatransform.html#parentseries" class="tsd-kind-icon">parentSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyybezierrenderdatatransform.html#pointseries" class="tsd-kind-icon">pointSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyybezierrenderdatatransform.html#requirestransform" class="tsd-kind-icon">requiresTransform</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyybezierrenderdatatransform.html#useforyrange" class="tsd-kind-icon">useForYRange</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyybezierrenderdatatransform.html#wasmcontext" class="tsd-kind-icon">wasmContext</a>

### Accessors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyybezierrenderdatatransform.html#curvature" class="tsd-kind-icon">curvature</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyybezierrenderdatatransform.html#interpolationpoints" class="tsd-kind-icon">interpolationPoints</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyybezierrenderdatatransform.html#createpointseries" class="tsd-kind-icon">createPointSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyybezierrenderdatatransform.html#delete" class="tsd-kind-icon">delete</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyybezierrenderdatatransform.html#makerenderpassdata" class="tsd-kind-icon">makeRenderPassData</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyybezierrenderdatatransform.html#ondatachange" class="tsd-kind-icon">onDataChange</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyybezierrenderdatatransform.html#runtransform" class="tsd-kind-icon">runTransform</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyybezierrenderdatatransform.html#runtransforminternal" class="tsd-kind-icon">runTransformInternal</a>

## Constructors

### constructor

- new XyyBezierRenderDataTransform(parentSeries: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries.html" class="tsd-signature-type">BaseRenderableSeries</a>, wasmContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart" class="tsd-signature-type">TSciChart</a>, drawingProviders?: Array\<<a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iseriesdrawingprovider.html" class="tsd-signature-type">ISeriesDrawingProvider</a>\>, options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibezierrenderdatatransformoptions.html" class="tsd-signature-type">IBezierRenderDataTransformOptions</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyybezierrenderdatatransform.html" class="tsd-signature-type">XyyBezierRenderDataTransform</a>

<!-- -->

- #### Parameters

  - ##### parentSeries: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries.html" class="tsd-signature-type">BaseRenderableSeries</a>

  - ##### wasmContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart" class="tsd-signature-type">TSciChart</a>

  - ##### Optional drawingProviders: Array\<<a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iseriesdrawingprovider.html" class="tsd-signature-type">ISeriesDrawingProvider</a>\>

  - ##### Optional options: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibezierrenderdatatransformoptions.html" class="tsd-signature-type">IBezierRenderDataTransformOptions</a>

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyybezierrenderdatatransform.html" class="tsd-signature-type">XyyBezierRenderDataTransform</a>

## Properties

### Readonly drawingProviders

drawingProviders: Array\<<a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iseriesdrawingprovider.html" class="tsd-signature-type">ISeriesDrawingProvider</a>\>

The drawingProviders on the parentSeries to which this transform applies

### Protected forceYGreaterThanY1

forceYGreaterThanY1: boolean = false

### Protected lastIndexRange

lastIndexRange: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

### Protected lastResamplingHash

lastResamplingHash: number

### Readonly parentSeries

parentSeries: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries.html" class="tsd-signature-type">BaseRenderableSeries</a>

The series the transform is attached to

### pointSeries

pointSeries: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyypointseriesresampled.html" class="tsd-signature-type">XyyPointSeriesResampled</a>

The pointSeries that stores the result of the transform

### requiresTransform

requiresTransform: boolean = true

Whether then transform will run when the series is drawn. This is set true initially, and when data changes, and is set to false when the transform does run. If your transform depends on any other property, you must set this true if that property changes

### useForYRange

useForYRange: boolean = false

A flag to tell the parent series if the transformed values should be used when calculating data range

### Protected wasmContext

wasmContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart" class="tsd-signature-type">TSciChart</a>

## Accessors

### curvature

- get curvature(): number
- set curvature(value: number): void

<!-- -->

- A scale factor for the tightness of the curves. Valid values 0 to 1. Lower = tighter curves

  #### Returns number

- A scale factor for the tightness of the curves. Valid values 0 to 1. Lower = tighter curves

  #### Parameters

  - ##### value: number

  #### Returns void

### interpolationPoints

- get interpolationPoints(): number
- set interpolationPoints(value: number): void

<!-- -->

- The number of points to add between each data point. Default 20 These are Not uniformly distributed, but clutered around the data points to give smoother curves

  #### Returns number

- The number of points to add between each data point. Default 20 These are Not uniformly distributed, but clutered around the data points to give smoother curves

  #### Parameters

  - ##### value: number

  #### Returns void

## Methods

### Protected createPointSeries

- createPointSeries(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyypointseriesresampled.html" class="tsd-signature-type">XyyPointSeriesResampled</a>

<!-- -->

- #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyypointseriesresampled.html" class="tsd-signature-type">XyyPointSeriesResampled</a>

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

- The method that is called to run the transform. Do not override this, instead override runTransformInternal on [BaseRenderDataTransform](https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderdatatransform.html)

  #### Parameters

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
