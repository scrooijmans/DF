<img src="out_scichartv4/typedoc/classes/errorserieshittestprovider_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/errorserieshittestprovider.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [ErrorSeriesHitTestProvider](https://www.scichart.com/documentation/js/v4/typedoc/classes/errorserieshittestprovider.html)

# Class ErrorSeriesHitTestProvider

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

Hit-test provider for [FastColumnRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/fastcolumnrenderableseries.html). See base class [BaseHitTestProvider](https://www.scichart.com/documentation/js/v4/typedoc/classes/basehittestprovider.html) for further info

### Hierarchy

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basehittestprovider.html" class="tsd-signature-type">BaseHitTestProvider</a>\<<a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ierrorrenderableseries.html" class="tsd-signature-type">IErrorRenderableSeries</a>\>
  - ErrorSeriesHitTestProvider

### Implements

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ihittestprovider.html" class="tsd-signature-type">IHitTestProvider</a>

## Index

### Constructors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/errorserieshittestprovider.html#constructor" class="tsd-kind-icon">constructor</a>

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/errorserieshittestprovider.html#currentrenderpassdata" class="tsd-kind-icon">currentRenderPassData</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/errorserieshittestprovider.html#parentseries" class="tsd-kind-icon">parentSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/errorserieshittestprovider.html#webassemblycontext" class="tsd-kind-icon">webAssemblyContext</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/errorserieshittestprovider.html#default_hit_test_radius" class="tsd-kind-icon">DEFAULT_HIT_TEST_RADIUS</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/errorserieshittestprovider.html#gettranslatedhittestpoint" class="tsd-kind-icon">getTranslatedHitTestPoint</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/errorserieshittestprovider.html#hittest" class="tsd-kind-icon">hitTest</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/errorserieshittestprovider.html#hittestdatapoint" class="tsd-kind-icon">hitTestDataPoint</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/errorserieshittestprovider.html#hittestfordatapointselectionmodifier" class="tsd-kind-icon">hitTestForDataPointSelectionModifier</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/errorserieshittestprovider.html#hittestxslice" class="tsd-kind-icon">hitTestXSlice</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/errorserieshittestprovider.html#update" class="tsd-kind-icon">update</a>

## Constructors

### constructor

- new ErrorSeriesHitTestProvider(parentSeries: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ierrorrenderableseries.html" class="tsd-signature-type">IErrorRenderableSeries</a>, wasmContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart" class="tsd-signature-type">TSciChart</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/errorserieshittestprovider.html" class="tsd-signature-type">ErrorSeriesHitTestProvider</a>

<!-- -->

- Creates an instance of the [BaseHitTestProvider](https://www.scichart.com/documentation/js/v4/typedoc/classes/basehittestprovider.html)

  #### Parameters

  - ##### parentSeries: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ierrorrenderableseries.html" class="tsd-signature-type">IErrorRenderableSeries</a>

    the parent [RenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries.html) that this Hit-Test provider is attached to

  - ##### wasmContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart" class="tsd-signature-type">TSciChart</a>

    the [SciChart WebAssembly Context](https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart) containing native methods and access to our WebGL2 WebAssembly Drawing Engine

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/errorserieshittestprovider.html" class="tsd-signature-type">ErrorSeriesHitTestProvider</a>

## Properties

### Protected currentRenderPassData

currentRenderPassData: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassdata.html" class="tsd-signature-type">RenderPassData</a>

### Readonly parentSeries

parentSeries: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ierrorrenderableseries.html" class="tsd-signature-type">IErrorRenderableSeries</a>

### Protected webAssemblyContext

webAssemblyContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart" class="tsd-signature-type">TSciChart</a>

### Static Readonly DEFAULT_HIT_TEST_RADIUS

DEFAULT_HIT_TEST_RADIUS: number = 7.07

## Methods

### getTranslatedHitTestPoint

- getTranslatedHitTestPoint(x: number, y: number): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/point.html" class="tsd-signature-type">Point</a>

<!-- -->

- #### Parameters

  - ##### x: number

  - ##### y: number

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/point.html" class="tsd-signature-type">Point</a>

### hitTest

- hitTest(x: number, y: number): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/hittestinfo.html" class="tsd-signature-type">HitTestInfo</a>

<!-- -->

- inheritdoc  

  #### Parameters

  - ##### x: number

  - ##### y: number

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/hittestinfo.html" class="tsd-signature-type">HitTestInfo</a>

### hitTestDataPoint

- hitTestDataPoint(x: number, y: number, hitTestRadius?: number): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/hittestinfo.html" class="tsd-signature-type">HitTestInfo</a>

<!-- -->

- description  
  Performs a hit-test for the data point at the specific mouse point (X,Y coordinate on the parent SciChartSurface), returning a HitTestInfo type with the results

  remarks  
  For Retina displays and Browser zoom, ensure that X,Y points are scaled by [DpiHelper.PIXEL_RATIO](https://www.scichart.com/documentation/js/v4/typedoc/classes/dpihelper.html#pixel_ratio)

  #### Parameters

  - ##### x: number

    The mouse point X coordinate on the parent SciChartSurface. NOTE: For Retina displays and Browser zoom, ensure that X,Y points are scaled by [DpiHelper.PIXEL_RATIO](https://www.scichart.com/documentation/js/v4/typedoc/classes/dpihelper.html#pixel_ratio)

  - ##### y: number

    The mouse point Y coordinate on the parent SciChartSurface. NOTE: For Retina displays and Browser zoom, ensure that X,Y points are scaled by [DpiHelper.PIXEL_RATIO](https://www.scichart.com/documentation/js/v4/typedoc/classes/dpihelper.html#pixel_ratio)

  - ##### Default value hitTestRadius: number = BaseHitTestProvider.DEFAULT_HIT_TEST_RADIUS

    The radius in pixels to determine whether a mouse is over a data-point

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/hittestinfo.html" class="tsd-signature-type">HitTestInfo</a>

### hitTestForDataPointSelectionModifier

- hitTestForDataPointSelectionModifier(x: number, y: number, hitTestRadius?: number): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/hittestinfo.html" class="tsd-signature-type">HitTestInfo</a>

<!-- -->

- description  
  Performs a hit-test for the [DataPointSelectionModifier](https://www.scichart.com/documentation/js/v4/typedoc/classes/datapointselectionmodifier.html). This calls [IHitTestProvider.hitTestDataPoint](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ihittestprovider.html#hittestdatapoint) by default. The hitTestProvider for the renderableSeries can override this if different behaviour is desired, eg for columSeries we call hitTest instead. returns a HitTestInfo type with the results, Only for sorted values

  remarks  
  For Retina displays and Browser zoom, ensure that X,Y points are scaled by [DpiHelper.PIXEL_RATIO](https://www.scichart.com/documentation/js/v4/typedoc/classes/dpihelper.html#pixel_ratio)

  #### Parameters

  - ##### x: number

    The mouse point X coordinate on the parent SciChartSurface. NOTE: For Retina displays and Browser zoom, ensure that X,Y points are scaled by [DpiHelper.PIXEL_RATIO](https://www.scichart.com/documentation/js/v4/typedoc/classes/dpihelper.html#pixel_ratio)

  - ##### y: number

    The mouse point Y coordinate on the parent SciChartSurface. NOTE: For Retina displays and Browser zoom, ensure that X,Y points are scaled by [DpiHelper.PIXEL_RATIO](https://www.scichart.com/documentation/js/v4/typedoc/classes/dpihelper.html#pixel_ratio)

  - ##### Default value hitTestRadius: number = BaseHitTestProvider.DEFAULT_HIT_TEST_RADIUS

    The radius in pixels to determine whether a mouse is over a data-point

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/hittestinfo.html" class="tsd-signature-type">HitTestInfo</a>

### hitTestXSlice

- hitTestXSlice(x: number, y: number): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/hittestinfo.html" class="tsd-signature-type">HitTestInfo</a>

<!-- -->

- inheritdoc  

  #### Parameters

  - ##### x: number

  - ##### y: number

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/hittestinfo.html" class="tsd-signature-type">HitTestInfo</a>

### update

- update(renderPassData: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassdata.html" class="tsd-signature-type">RenderPassData</a>): void

<!-- -->

- description  
  updates the current HitTestProvider with the latest renderPassData

  #### Parameters

  - ##### renderPassData: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassdata.html" class="tsd-signature-type">RenderPassData</a>

    the latest renderPassData from the parent series last draw operation

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
