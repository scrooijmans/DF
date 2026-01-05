<img src="out_scichartv4/typedoc/classes/observablearraychangedargs_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/observablearraychangedargs.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [ObservableArrayChangedArgs](https://www.scichart.com/documentation/js/v4/typedoc/classes/observablearraychangedargs.html)

# Class ObservableArrayChangedArgs

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

Event args used by the [ObservableArray.collectionChanged](https://www.scichart.com/documentation/js/v4/typedoc/classes/observablearray.html#collectionchanged) event

### Hierarchy

- ObservableArrayChangedArgs

## Index

### Constructors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/observablearraychangedargs.html#constructor" class="tsd-kind-icon">constructor</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/observablearraychangedargs.html#getaction" class="tsd-kind-icon">getAction</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/observablearraychangedargs.html#getnewitems" class="tsd-kind-icon">getNewItems</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/observablearraychangedargs.html#getolditems" class="tsd-kind-icon">getOldItems</a>

## Constructors

### constructor

- new ObservableArrayChangedArgs(action: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eobservablearraychangedaction.html" class="tsd-signature-type">EObservableArrayChangedAction</a>, newItems: any\[\], oldItems: any\[\]): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/observablearraychangedargs.html" class="tsd-signature-type">ObservableArrayChangedArgs</a>

<!-- -->

- Creates an instance of the [ObservableArrayChangedArgs](https://www.scichart.com/documentation/js/v4/typedoc/classes/observablearraychangedargs.html)

  #### Parameters

  - ##### action: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eobservablearraychangedaction.html" class="tsd-signature-type">EObservableArrayChangedAction</a>

  - ##### newItems: any\[\]

  - ##### oldItems: any\[\]

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/observablearraychangedargs.html" class="tsd-signature-type">ObservableArrayChangedArgs</a>

## Methods

### getAction

- getAction(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eobservablearraychangedaction.html" class="tsd-signature-type">EObservableArrayChangedAction</a>

<!-- -->

- Gets the operation that occurred on the [ObservableArray](https://www.scichart.com/documentation/js/v4/typedoc/classes/observablearray.html). See [EObservableArrayChangedAction](https://www.scichart.com/documentation/js/v4/typedoc/enums/eobservablearraychangedaction.html) for a list of values

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eobservablearraychangedaction.html" class="tsd-signature-type">EObservableArrayChangedAction</a>

### getNewItems

- getNewItems(): any\[\]

<!-- -->

- Gets new items added if the [EObservableArrayChangedAction](https://www.scichart.com/documentation/js/v4/typedoc/enums/eobservablearraychangedaction.html) was Add or Replace

  #### Returns any\[\]

### getOldItems

- getOldItems(): any\[\]

<!-- -->

- Gets old items removed if the [EObservableArrayChangedAction](https://www.scichart.com/documentation/js/v4/typedoc/enums/eobservablearraychangedaction.html) was Remove or Replace

  #### Returns any\[\]

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
