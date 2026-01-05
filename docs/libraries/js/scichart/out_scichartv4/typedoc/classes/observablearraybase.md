<img src="out_scichartv4/typedoc/classes/observablearraybase_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/observablearraybase.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [ObservableArrayBase](https://www.scichart.com/documentation/js/v4/typedoc/classes/observablearraybase.html)

# Class ObservableArrayBase\<T\>

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

An Observable array which raises [collectionChanged](https://www.scichart.com/documentation/js/v4/typedoc/classes/observablearraybase.html#collectionchanged) events when an item is added, removed or the collection cleared

### Type parameters

- #### T

### Hierarchy

- ObservableArrayBase
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/observablearray.html" class="tsd-signature-type">ObservableArray</a>

## Index

### Constructors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/observablearraybase.html#constructor" class="tsd-kind-icon">constructor</a>

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/observablearraybase.html#collectionchanged" class="tsd-kind-icon">collectionChanged</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/observablearraybase.html#items" class="tsd-kind-icon">items</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/observablearraybase.html#add" class="tsd-kind-icon">add</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/observablearraybase.html#asarray" class="tsd-kind-icon">asArray</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/observablearraybase.html#clear" class="tsd-kind-icon">clear</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/observablearraybase.html#contains" class="tsd-kind-icon">contains</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/observablearraybase.html#get" class="tsd-kind-icon">get</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/observablearraybase.html#insert" class="tsd-kind-icon">insert</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/observablearraybase.html#remove" class="tsd-kind-icon">remove</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/observablearraybase.html#removeat" class="tsd-kind-icon">removeAt</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/observablearraybase.html#set" class="tsd-kind-icon">set</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/observablearraybase.html#size" class="tsd-kind-icon">size</a>

## Constructors

### constructor

- new ObservableArrayBase(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/observablearraybase.html" class="tsd-signature-type">ObservableArrayBase</a>

<!-- -->

- Creates an instance of the [ObservableArray](https://www.scichart.com/documentation/js/v4/typedoc/classes/observablearray.html)

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/observablearraybase.html" class="tsd-signature-type">ObservableArrayBase</a>

## Properties

### Readonly collectionChanged

collectionChanged: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/eventhandler.html" class="tsd-signature-type">EventHandler</a>\<<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/observablearraychangedargs.html" class="tsd-signature-type">ObservableArrayChangedArgs</a>\>

Event handler which fires when the collection changes. See [ObservableArrayChangedArgs](https://www.scichart.com/documentation/js/v4/typedoc/classes/observablearraychangedargs.html) for args

### Protected items

items: T\[\] = \[\]

## Methods

### add

- add(...items: T\[\]): void

<!-- -->

- Adds items to the array, and raises the [collectionChanged](https://www.scichart.com/documentation/js/v4/typedoc/classes/observablearraybase.html#collectionchanged) event to subscribers

  #### Parameters

  - ##### Rest ...items: T\[\]

  #### Returns void

### asArray

- asArray(): T\[\]

<!-- -->

- Returns the backing array. Do not modify this collection. Use add or remove instead.

  #### Returns T\[\]

### clear

- clear(callDeleteOnChildren?: boolean): void

<!-- -->

- Clears the array. Raises the [collectionChanged](https://www.scichart.com/documentation/js/v4/typedoc/classes/observablearraybase.html#collectionchanged) event to subscribers

  #### Parameters

  - ##### Default value callDeleteOnChildren: boolean = true

    When true, if the items in the array implement the [IDeletable](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html) interface, the delete() function will be called. Defaults to true

  #### Returns void

### contains

- contains(item: T): boolean

<!-- -->

- Returns true if the array contains an item

  #### Parameters

  - ##### item: T

  #### Returns boolean

### get

- get(index: number): T

<!-- -->

- Gets an item at index

  #### Parameters

  - ##### index: number

  #### Returns T

### insert

- insert(index: number, item: T): void

<!-- -->

- Inserts items at the specified index. Raises the [collectionChanged](https://www.scichart.com/documentation/js/v4/typedoc/classes/observablearraybase.html#collectionchanged) event to subscribers

  #### Parameters

  - ##### index: number

  - ##### item: T

  #### Returns void

### remove

- remove(item: T, callDeleteOnChildren?: boolean): void

<!-- -->

- Removes an item by value. Raises the [collectionChanged](https://www.scichart.com/documentation/js/v4/typedoc/classes/observablearraybase.html#collectionchanged) event to subscribers

  #### Parameters

  - ##### item: T

    The item to remove

  - ##### Default value callDeleteOnChildren: boolean = true

    When true, if the items in the array implement the [IDeletable](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html) interface, the delete() function will be called. Defaults to true

  #### Returns void

### removeAt

- removeAt(index: number, callDeleteOnChildren?: boolean): void

<!-- -->

- Removes an item at the specified index. Raises the [collectionChanged](https://www.scichart.com/documentation/js/v4/typedoc/classes/observablearraybase.html#collectionchanged) event to subscribers

  #### Parameters

  - ##### index: number

    The item to remove

  - ##### Default value callDeleteOnChildren: boolean = true

    When true, if the items in the array implement the [IDeletable](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html) interface, the delete() function will be called. Defaults to true

  #### Returns void

### set

- set(index: number, item: T): void

<!-- -->

- Sets an item at index. Raises the [collectionChanged](https://www.scichart.com/documentation/js/v4/typedoc/classes/observablearraybase.html#collectionchanged) event to subscribers

  #### Parameters

  - ##### index: number

  - ##### item: T

  #### Returns void

### size

- size(): number

<!-- -->

- gets the number of elements in the array

  #### Returns number

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
