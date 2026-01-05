<img src="out_scichartv4/typedoc/classes/polarmountainserieshittestprovider_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarmountainserieshittestprovider.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [PolarMountainSeriesHitTestProvider](https://www.scichart.com/documentation/js/v4/typedoc/classes/polarmountainserieshittestprovider.html)

# Class PolarMountainSeriesHitTestProvider

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

Hit-test provider for [PolarMountainRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/polarmountainrenderableseries.html). See base class [BaseHitTestProvider](https://www.scichart.com/documentation/js/v4/typedoc/classes/basehittestprovider.html) for further info. Does not support vertical charts

### Hierarchy

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polardatapointhittestprovider.html" class="tsd-signature-type">PolarDataPointHitTestProvider</a>
  - PolarMountainSeriesHitTestProvider

### Implements

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ihittestprovider.html" class="tsd-signature-type">IHitTestProvider</a>

## Index

### Constructors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarmountainserieshittestprovider.html#constructor" class="tsd-kind-icon">constructor</a>

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarmountainserieshittestprovider.html#currentrenderpassdata" class="tsd-kind-icon">currentRenderPassData</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarmountainserieshittestprovider.html#parentseries" class="tsd-kind-icon">parentSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarmountainserieshittestprovider.html#webassemblycontext" class="tsd-kind-icon">webAssemblyContext</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarmountainserieshittestprovider.html#default_hit_test_radius" class="tsd-kind-icon">DEFAULT_HIT_TEST_RADIUS</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarmountainserieshittestprovider.html#gettranslatedhittestpoint" class="tsd-kind-icon">getTranslatedHitTestPoint</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarmountainserieshittestprovider.html#hittest" class="tsd-kind-icon">hitTest</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarmountainserieshittestprovider.html#hittestdatapoint" class="tsd-kind-icon">hitTestDataPoint</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarmountainserieshittestprovider.html#hittestdatapointinternal" class="tsd-kind-icon">hitTestDataPointInternal</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarmountainserieshittestprovider.html#hittestfordatapointselectionmodifier" class="tsd-kind-icon">hitTestForDataPointSelectionModifier</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarmountainserieshittestprovider.html#hittestmountainbody" class="tsd-kind-icon">hitTestMountainBody</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarmountainserieshittestprovider.html#hittestxslice" class="tsd-kind-icon">hitTestXSlice</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarmountainserieshittestprovider.html#update" class="tsd-kind-icon">update</a>

## Constructors

### constructor

- new PolarMountainSeriesHitTestProvider(parentSeries: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries.html" class="tsd-signature-type">BaseRenderableSeries</a>, wasmContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart" class="tsd-signature-type">TSciChart</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarmountainserieshittestprovider.html" class="tsd-signature-type">PolarMountainSeriesHitTestProvider</a>

<!-- -->

- Creates an instance of the [BaseHitTestProvider](https://www.scichart.com/documentation/js/v4/typedoc/classes/basehittestprovider.html)

  #### Parameters

  - ##### parentSeries: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries.html" class="tsd-signature-type">BaseRenderableSeries</a>

    the parent [RenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries.html) that this Hit-Test provider is attached to

  - ##### wasmContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart" class="tsd-signature-type">TSciChart</a>

    the [SciChart WebAssembly Context](https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart) containing native methods and access to our WebGL2 WebAssembly Drawing Engine

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarmountainserieshittestprovider.html" class="tsd-signature-type">PolarMountainSeriesHitTestProvider</a>

## Properties

### Protected currentRenderPassData

currentRenderPassData: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassdata.html" class="tsd-signature-type">RenderPassData</a>

### Readonly parentSeries

parentSeries: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries.html" class="tsd-signature-type">BaseRenderableSeries</a>

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

  - ##### Optional hitTestRadius: number

    The radius in pixels to determine whether a mouse is over a data-point

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/hittestinfo.html" class="tsd-signature-type">HitTestInfo</a>

### Protected hitTestDataPointInternal

- hitTestDataPointInternal(x: number, y: number, hitTestRadius: number): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/hittestinfo.html" class="tsd-signature-type">HitTestInfo</a>

<!-- -->

- #### Parameters

  - ##### x: number

  - ##### y: number

  - ##### hitTestRadius: number

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

### Protected hitTestMountainBody

- hitTestMountainBody(x: number, y: number): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/hittestinfo.html" class="tsd-signature-type">HitTestInfo</a>

<!-- -->

- Does not support ve

  #### Parameters

  - ##### x: number

  - ##### y: number

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
