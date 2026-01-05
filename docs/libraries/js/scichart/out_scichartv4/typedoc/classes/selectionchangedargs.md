<img src="out_scichartv4/typedoc/classes/selectionchangedargs_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/selectionchangedargs.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [SelectionChangedArgs](https://www.scichart.com/documentation/js/v4/typedoc/classes/selectionchangedargs.html)

# Class SelectionChangedArgs

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

Arguments passed into the callback for [SeriesSelectionModifier.selectionChanged](https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesselectionmodifier.html#selectionchanged) event

### Hierarchy

- SelectionChangedArgs

## Index

### Constructors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/selectionchangedargs.html#constructor" class="tsd-kind-icon">constructor</a>

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/selectionchangedargs.html#allseries" class="tsd-kind-icon">allSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/selectionchangedargs.html#hittestinfo" class="tsd-kind-icon">hitTestInfo</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/selectionchangedargs.html#selectedseries" class="tsd-kind-icon">selectedSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/selectionchangedargs.html#source" class="tsd-kind-icon">source</a>

## Constructors

### constructor

- new SelectionChangedArgs(source: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesselectionmodifier.html" class="tsd-signature-type">SeriesSelectionModifier</a>, selectedSeries: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries.html" class="tsd-signature-type">IRenderableSeries</a>\[\], allSeries: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries.html" class="tsd-signature-type">IRenderableSeries</a>\[\], hitTestInfo: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/hittestinfo.html" class="tsd-signature-type">HitTestInfo</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/selectionchangedargs.html" class="tsd-signature-type">SelectionChangedArgs</a>

<!-- -->

- Creates an instance of SelectionChangedArgs - arguments passed into the callback for [SeriesSelectionModifier.selectionChanged](https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesselectionmodifier.html#selectionchanged) event

  #### Parameters

  - ##### source: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesselectionmodifier.html" class="tsd-signature-type">SeriesSelectionModifier</a>

  - ##### selectedSeries: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries.html" class="tsd-signature-type">IRenderableSeries</a>\[\]

  - ##### allSeries: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries.html" class="tsd-signature-type">IRenderableSeries</a>\[\]

  - ##### hitTestInfo: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/hittestinfo.html" class="tsd-signature-type">HitTestInfo</a>

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/selectionchangedargs.html" class="tsd-signature-type">SelectionChangedArgs</a>

## Properties

### Readonly allSeries

allSeries: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries.html" class="tsd-signature-type">IRenderableSeries</a>\[\]

The array of all series at the time of the event. Query [IRenderableSeries.isSelected](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries.html#isselected) to determine if it is selected or not

### Readonly hitTestInfo

hitTestInfo: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/hittestinfo.html" class="tsd-signature-type">HitTestInfo</a>

The hitTestInfo for this event, if available.

### Readonly selectedSeries

selectedSeries: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries.html" class="tsd-signature-type">IRenderableSeries</a>\[\]

The array of selected series at the time of the event

### Readonly source

source: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesselectionmodifier.html" class="tsd-signature-type">SeriesSelectionModifier</a>

The source [SeriesSelectionModifier](https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesselectionmodifier.html) which raised the event

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
