<img src="out_scichartv4/typedoc/classes/datapointselectionchangedargs_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datapointselectionchangedargs.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [DataPointSelectionChangedArgs](https://www.scichart.com/documentation/js/v4/typedoc/classes/datapointselectionchangedargs.html)

# Class DataPointSelectionChangedArgs

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

Arguments passed into the callback for [DataPointSelectionModifier.selectionChanged](https://www.scichart.com/documentation/js/v4/typedoc/classes/datapointselectionmodifier.html#selectionchanged) event

### Hierarchy

- DataPointSelectionChangedArgs

## Index

### Constructors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datapointselectionchangedargs.html#constructor" class="tsd-kind-icon">constructor</a>

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datapointselectionchangedargs.html#selecteddatapoints" class="tsd-kind-icon">selectedDataPoints</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datapointselectionchangedargs.html#source" class="tsd-kind-icon">source</a>

## Constructors

### constructor

- new DataPointSelectionChangedArgs(source: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datapointselectionmodifier.html" class="tsd-signature-type">DataPointSelectionModifier</a>, selectedDataPoints: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datapointinfo.html" class="tsd-signature-type">DataPointInfo</a>\[\]): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datapointselectionchangedargs.html" class="tsd-signature-type">DataPointSelectionChangedArgs</a>

<!-- -->

- Creates an instance of DataPointSelectionChangedArgs - arguments passed into the callback for [DataPointSelectionModifier.selectionChanged](https://www.scichart.com/documentation/js/v4/typedoc/classes/datapointselectionmodifier.html#selectionchanged) event

  #### Parameters

  - ##### source: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datapointselectionmodifier.html" class="tsd-signature-type">DataPointSelectionModifier</a>

  - ##### selectedDataPoints: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datapointinfo.html" class="tsd-signature-type">DataPointInfo</a>\[\]

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datapointselectionchangedargs.html" class="tsd-signature-type">DataPointSelectionChangedArgs</a>

## Properties

### Readonly selectedDataPoints

selectedDataPoints: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datapointinfo.html" class="tsd-signature-type">DataPointInfo</a>\[\]

The array of selected datapoints at the time of the event. These are datapoints on any [BaseDataSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/basedataseries.html) which have metadata with [IPointMetadata.isSelected](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmetadata.html#isselected) = true. Query [DataPointInfo](https://www.scichart.com/documentation/js/v4/typedoc/classes/datapointinfo.html) for more details about the series and data-point index that was selected

### Readonly source

source: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datapointselectionmodifier.html" class="tsd-signature-type">DataPointSelectionModifier</a>

The source [DataPointSelectionModifier](https://www.scichart.com/documentation/js/v4/typedoc/classes/datapointselectionmodifier.html) which raised the event

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
