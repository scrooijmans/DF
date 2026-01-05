<img src="out_scichartv4/typedoc/interfaces/ifilterbase_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ifilterbase.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [IFilterBase](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ifilterbase.html)

# Interface IFilterBase

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

Defines the interface to a DataSeries in SciChart's High Performance Real-time <a href="https://www.scichart.com/javascript-chart-features" class="external">JavaScript Charts</a>

remarks  
A DataSeries stores the data to render. This is independent from the [RenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries.html) which defines how that data should be rendered.

See derived types of [BaseDataSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/basedataseries.html) to find out what data-series are available. See derived types of [IRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries.html) to find out what 2D JavaScript Chart types are available.

### Hierarchy

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html" class="tsd-signature-type">IDeletable</a>
  - IFilterBase

### Implemented by

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/hlccustomfilter.html" class="tsd-signature-type">HlcCustomFilter</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/hlcfilterbase.html" class="tsd-signature-type">HlcFilterBase</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/hlcscaleoffsetfilter.html" class="tsd-signature-type">HlcScaleOffsetFilter</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/ohlccustomfilter.html" class="tsd-signature-type">OhlcCustomFilter</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/ohlcfilterbase.html" class="tsd-signature-type">OhlcFilterBase</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/ohlcscaleoffsetfilter.html" class="tsd-signature-type">OhlcScaleOffsetFilter</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xycustomfilter.html" class="tsd-signature-type">XyCustomFilter</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyfilterbase.html" class="tsd-signature-type">XyFilterBase</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xylineartrendfilter.html" class="tsd-signature-type">XyLinearTrendFilter</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xymovingaveragefilter.html" class="tsd-signature-type">XyMovingAverageFilter</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyratiofilter.html" class="tsd-signature-type">XyRatioFilter</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyscaleoffsetfilter.html" class="tsd-signature-type">XyScaleOffsetFilter</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyycustomfilter.html" class="tsd-signature-type">XyyCustomFilter</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyyfilterbase.html" class="tsd-signature-type">XyyFilterBase</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyyscaleoffsetfilter.html" class="tsd-signature-type">XyyScaleOffsetFilter</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyzcustomfilter.html" class="tsd-signature-type">XyzCustomFilter</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyzfilterbase.html" class="tsd-signature-type">XyzFilterBase</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyzscaleoffsetfilter.html" class="tsd-signature-type">XyzScaleOffsetFilter</a>

## Index

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ifilterbase.html#originalseries" class="tsd-kind-icon">originalSeries</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ifilterbase.html#delete" class="tsd-kind-icon">delete</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ifilterbase.html#detachfromoriginalseries" class="tsd-kind-icon">detachFromOriginalSeries</a>

## Properties

### Readonly originalSeries

originalSeries: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idataseries.html" class="tsd-signature-type">IDataSeries</a>

The [IDataSeries](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idataseries.html) to be filtered

## Methods

### delete

- delete(): void

<!-- -->

- Deletes native (WebAssembly) memory used by this type, after which it cannot be used.

  remarks  
  Call .delete() before finishing with the object to ensure that WebAssmembly memory leaks do not occur.

  All elements within SciChart's High Performance <a href="https://www.scichart.com/javascript-chart-features" class="external">Realtime JavaScript Charts</a> which implement [IDeletable](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html) must be deleted manually to free native (WebAssembly) memory

  #### Returns void

### detachFromOriginalSeries

- detachFromOriginalSeries(): void

<!-- -->

- Removes reference to the original [IDataSeries](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idataseries.html) and prevents them from being deleted as an effect of the filter deletion.

  remarks  
  Makes the filter unusable, but allows deleting it separately from the original data series.

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
