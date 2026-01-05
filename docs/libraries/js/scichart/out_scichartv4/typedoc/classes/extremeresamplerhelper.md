<img src="out_scichartv4/typedoc/classes/extremeresamplerhelper_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/extremeresamplerhelper.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [ExtremeResamplerHelper](https://www.scichart.com/documentation/js/v4/typedoc/classes/extremeresamplerhelper.html)

# Class ExtremeResamplerHelper

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

Helper class for functions which optimise drawing

### Hierarchy

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/deletableentity.html" class="tsd-signature-type">DeletableEntity</a>
  - ExtremeResamplerHelper

### Implements

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html" class="tsd-signature-type">IDeletable</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html" class="tsd-signature-type">IDeletable</a>

## Index

### Constructors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/extremeresamplerhelper.html#constructor" class="tsd-kind-icon">constructor</a>

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/extremeresamplerhelper.html#nativemergeindexparams" class="tsd-kind-icon">nativeMergeIndexParams</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/extremeresamplerhelper.html#copyvaluesbyindexes" class="tsd-kind-icon">copyValuesByIndexes</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/extremeresamplerhelper.html#delete" class="tsd-kind-icon">delete</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/extremeresamplerhelper.html#getnativeresamplingmode" class="tsd-kind-icon">getNativeResamplingMode</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/extremeresamplerhelper.html#mergeindexes" class="tsd-kind-icon">mergeIndexes</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/extremeresamplerhelper.html#needsresampling" class="tsd-kind-icon">needsResampling</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/extremeresamplerhelper.html#resampleintopointseries" class="tsd-kind-icon">resampleIntoPointSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/extremeresamplerhelper.html#resetandfillbasicnativeargs" class="tsd-kind-icon">resetAndFillBasicNativeArgs</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/extremeresamplerhelper.html#calculateresamplinghash" class="tsd-kind-icon">calculateResamplingHash</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/extremeresamplerhelper.html#resampleseries" class="tsd-kind-icon">resampleSeries</a>

## Constructors

### constructor

- new ExtremeResamplerHelper(wasmContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart" class="tsd-signature-type">TSciChart</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/extremeresamplerhelper.html" class="tsd-signature-type">ExtremeResamplerHelper</a>

<!-- -->

- #### Parameters

  - ##### wasmContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart" class="tsd-signature-type">TSciChart</a>

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/extremeresamplerhelper.html" class="tsd-signature-type">ExtremeResamplerHelper</a>

## Properties

### Readonly nativeMergeIndexParams

nativeMergeIndexParams: SCRTDoubleResamplerMergeIndicesParams

## Methods

### copyValuesByIndexes

- copyValuesByIndexes(indices: IntVector, xValues: SCRTDoubleVector, yValues: SCRTDoubleVector, y1Values: SCRTDoubleVector, count: number, isCategoryAxis: boolean, isFifoSweeping: boolean, indicesOut: SCRTDoubleVector, xValuesOut: SCRTDoubleVector, yValuesOut: SCRTDoubleVector, y1ValuesOut: SCRTDoubleVector, y1Offset?: number): void

<!-- -->

- #### Parameters

  - ##### indices: IntVector

  - ##### xValues: SCRTDoubleVector

  - ##### yValues: SCRTDoubleVector

  - ##### y1Values: SCRTDoubleVector

  - ##### count: number

  - ##### isCategoryAxis: boolean

  - ##### isFifoSweeping: boolean

  - ##### indicesOut: SCRTDoubleVector

  - ##### xValuesOut: SCRTDoubleVector

  - ##### yValuesOut: SCRTDoubleVector

  - ##### y1ValuesOut: SCRTDoubleVector

  - ##### Default value y1Offset: number = 0

  #### Returns void

### delete

- delete(): void

<!-- -->

- Deletes native (WebAssembly) memory used by this type, after which it cannot be used.

  remarks  
  Call .delete() before finishing with the object to ensure that WebAssmembly memory leaks do not occur.

  All elements within SciChart's High Performance <a href="https://www.scichart.com/javascript-chart-features" class="external">Realtime JavaScript Charts</a> which implement [IDeletable](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html) must be deleted manually to free native (WebAssembly) memory

  #### Returns void

### getNativeResamplingMode

- getNativeResamplingMode(resamplingMode: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eresamplingmode.html" class="tsd-signature-type">EResamplingMode</a>): ResamplingMode

<!-- -->

- #### Parameters

  - ##### resamplingMode: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eresamplingmode.html" class="tsd-signature-type">EResamplingMode</a>

  #### Returns ResamplingMode

### mergeIndexes

- mergeIndexes(indices: IntVector, size1: number, size2: number, mergedIndicesOut: IntVector): number

<!-- -->

- #### Parameters

  - ##### indices: IntVector

  - ##### size1: number

  - ##### size2: number

  - ##### mergedIndicesOut: IntVector

  #### Returns number

### needsResampling

- needsResampling(rp: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/resamplingparams.html" class="tsd-signature-type">ResamplingParams</a>, xOriginalValues: SCRTDoubleVector, fillBasicNativeArgs?: boolean, updateResamplingMode?: boolean): boolean

<!-- -->

- Calls native RequiresReduction method to calculate if resampling is needed

  #### Parameters

  - ##### rp: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/resamplingparams.html" class="tsd-signature-type">ResamplingParams</a>

    The resampling params

  - ##### xOriginalValues: SCRTDoubleVector

    XValues

  - ##### Default value fillBasicNativeArgs: boolean = true

    Update nativeArgs if True

  - ##### Default value updateResamplingMode: boolean = false

    Update {@link ResamplingArgs.Resampling} property if True

  #### Returns boolean

### resampleIntoPointSeries

- resampleIntoPointSeries(wasmContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart" class="tsd-signature-type">TSciChart</a>, rp: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/resamplingparams.html" class="tsd-signature-type">ResamplingParams</a>, xOriginalValues: SCRTDoubleVector, yOriginalValues: SCRTDoubleVector, indexesOut: IntVector, indexesResampledOut: SCRTDoubleVector, xResampledValuesOut: SCRTDoubleVector, yResampledValuesOut: SCRTDoubleVector, fillBasicNativeArgs?: boolean): { OutputSplitIndex: number }

<!-- -->

- This method calls does resampling by calling the native resampling methods. It does similar things as WPF AvxExtremeResamplerDoubleDouble.ResampleInternal() method The needsResampling() method must be called before, this is done in {@link SciChartRenderer.resampleSeries}

  #### Parameters

  - ##### wasmContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart" class="tsd-signature-type">TSciChart</a>

    The WebAssembly context

  - ##### rp: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/resamplingparams.html" class="tsd-signature-type">ResamplingParams</a>

    The resampling parameters

  - ##### xOriginalValues: SCRTDoubleVector

    The original not resampled X values

  - ##### yOriginalValues: SCRTDoubleVector

    The original not resampled Y values

  - ##### indexesOut: IntVector

    The indices output

  - ##### indexesResampledOut: SCRTDoubleVector

  - ##### xResampledValuesOut: SCRTDoubleVector

    The resampled X values output

  - ##### yResampledValuesOut: SCRTDoubleVector

    The resampled Y values output

  - ##### Default value fillBasicNativeArgs: boolean = true

    If set to True resets and fill nativeArgs with basic properties, you can set it to False if {@link needsResampling()} was called before this method

  #### Returns { OutputSplitIndex: number }

  indicesVector The resampled vector of indices

  - ##### OutputSplitIndex: number

### resetAndFillBasicNativeArgs

- resetAndFillBasicNativeArgs(rp: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/resamplingparams.html" class="tsd-signature-type">ResamplingParams</a>, xOriginalValues: SCRTDoubleVector): void

<!-- -->

- Fills basic native args needed for {@link needsResampling()} and [resampleIntoPointSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/extremeresamplerhelper.html#resampleintopointseries) methods

  #### Parameters

  - ##### rp: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/resamplingparams.html" class="tsd-signature-type">ResamplingParams</a>

  - ##### xOriginalValues: SCRTDoubleVector

  #### Returns void

### Static calculateResamplingHash

- calculateResamplingHash(rs: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries.html" class="tsd-signature-type">IRenderableSeries</a>, rp: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/resamplingparams.html" class="tsd-signature-type">ResamplingParams</a>): number

<!-- -->

- #### Parameters

  - ##### rs: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries.html" class="tsd-signature-type">IRenderableSeries</a>

  - ##### rp: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/resamplingparams.html" class="tsd-signature-type">ResamplingParams</a>

  #### Returns number

### Static resampleSeries

- resampleSeries(xAxis: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase2d.html" class="tsd-signature-type">AxisBase2D</a>, rs: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries.html" class="tsd-signature-type">IRenderableSeries</a>, seriesViewRect: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tseriesrenderpassinfo" class="tsd-signature-type">TSeriesRenderPassInfo</a>

<!-- -->

- Used internally

  #### Parameters

  - ##### xAxis: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase2d.html" class="tsd-signature-type">AxisBase2D</a>

  - ##### rs: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries.html" class="tsd-signature-type">IRenderableSeries</a>

  - ##### seriesViewRect: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tseriesrenderpassinfo" class="tsd-signature-type">TSeriesRenderPassInfo</a>

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
