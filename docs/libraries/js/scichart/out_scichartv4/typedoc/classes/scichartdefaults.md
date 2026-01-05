<img src="out_scichartv4/typedoc/classes/scichartdefaults_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartdefaults.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [SciChartDefaults](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartdefaults.html)

# Class SciChartDefaults

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

### Hierarchy

- SciChartDefaults

## Index

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartdefaults.html#createsuspended" class="tsd-kind-icon">createSuspended</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartdefaults.html#debugdisableresampling" class="tsd-kind-icon">debugDisableResampling</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartdefaults.html#defaultloader" class="tsd-kind-icon">defaultLoader</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartdefaults.html#disableaspect" class="tsd-kind-icon">disableAspect</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartdefaults.html#nativefonttimeout" class="tsd-kind-icon">nativeFontTimeout</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartdefaults.html#performancewarnings" class="tsd-kind-icon">performanceWarnings</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartdefaults.html#usenativetext" class="tsd-kind-icon">useNativeText</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartdefaults.html#usesharedcache" class="tsd-kind-icon">useSharedCache</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartdefaults.html#wasmbuffersizeskb" class="tsd-kind-icon">wasmBufferSizesKb</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartdefaults.html#watermarkposition" class="tsd-kind-icon">watermarkPosition</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartdefaults.html#watermarkrelativetocanvas" class="tsd-kind-icon">watermarkRelativeToCanvas</a>

## Properties

### Static createSuspended

createSuspended: boolean = false

Defines if newly created charts should be rendered as soon as possible after initialization. Setting to `true` will require surfaces to be "resumed" in order to perform actual rendering.

### Static debugDisableResampling

debugDisableResampling: boolean = false

NOT RECOMMENDED UNLESS IN DEBUG MODE: Turn on/off adaptive, visually lossless resampling algorithms globally for the entire application.

To do this on a per-series basis use [BaseRenderableSeries.resamplingMode](https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries.html#resamplingmode). For precision issues try adjusting the [BaseRenderableSeries.resamplingPrecision](https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries.html#resamplingprecision) property

### Static defaultLoader

defaultLoader: false \| <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iscichartloader.html" class="tsd-signature-type">ISciChartLoader</a> = new DefaultSciChartLoader()

Allows you to customize the loading elements or animation as part of the HTML page / DOM when a [SciChartSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html) or [SciChart3DSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html) is loading WebAssembly.

Set to false for disabling.

### Static disableAspect

disableAspect: boolean = false

Optional - the option of disabling / enabling scaling of the [SciChartSurfaceBase](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurfacebase.html). If false - the [SciChartSurfaceBase](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurfacebase.html) will take the height and width of parent div without scaling.

### Static nativeFontTimeout

nativeFontTimeout: number = 2000

Time in ms to retry loading a native font. If you need to load a large font over a potentially slow connection await sciChartSurface.registerFont

### Static performanceWarnings

performanceWarnings: boolean = true

Turn on/off warnings to optimize performance

### Static useNativeText

useNativeText: boolean = true

Whether to use WebGL for rendering axis labels. Default true (was false before v4). These are much faster than rendering using canvas text, but do not have quite the same font and style support.

You can set useNativeText on an individual axis using to override this setting.

### Static useSharedCache

useSharedCache: boolean = true

Set this true to allow reuse of label textures across different axes and different charts

### Static wasmBufferSizesKb

wasmBufferSizesKb: number = 1024 \* 8

For 2D charts a number of buffers are created to process data. Buffer size by default grows with usage and caps out at 8,192kb. A total of 10 buffers are created for different scenarios. When SciChartSurface.create() is used this results in a maximum of 80MB memory as a static overhead for the application. When SciChartSurface.createSingle() is used this results in up to 80MB memory *per chart* as a static overhead. Lowering this number to 2048kb (or 1024kb) will reduce memory usage for the SciChart 2D Engine but may impact chart drawing performance.

remarks  
This property needs to be set before charts created and cannot be dynamically adjusted after that. Do not set lower than 1024kb or higher than 32MB (1024 x 32). Values outside this range will be clamped

### Static watermarkPosition

watermarkPosition: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/ewatermarkposition.html" class="tsd-signature-type">EWatermarkPosition</a> = EWatermarkPosition.BottomLeft

The default position of the watermark for trials

### Static watermarkRelativeToCanvas

watermarkRelativeToCanvas: boolean = false

For 2D charts the watermark is normally positioned within the series area. Set this true to place it relative to the overall canvas.

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
