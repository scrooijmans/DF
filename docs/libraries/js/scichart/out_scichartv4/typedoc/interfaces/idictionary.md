<img src="out_scichartv4/typedoc/interfaces/idictionary_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idictionary.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [IDictionary](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idictionary.html)

# Interface IDictionary\<T\>

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

Generic dictionary interface keyed by string

### Type parameters

- #### T

### Hierarchy

- IDictionary

### Implemented by

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/dictionary.html" class="tsd-signature-type">Dictionary</a>

## Index

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idictionary.html#count" class="tsd-kind-icon">count</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idictionary.html#keys" class="tsd-kind-icon">keys</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idictionary.html#values" class="tsd-kind-icon">values</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idictionary.html#add" class="tsd-kind-icon">add</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idictionary.html#containskey" class="tsd-kind-icon">containsKey</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idictionary.html#item" class="tsd-kind-icon">item</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idictionary.html#remove" class="tsd-kind-icon">remove</a>

## Properties

### Readonly count

count: number

Gets the number of items in the dictionary

### Readonly keys

keys: string\[\]

Gets an array of string keys in the dictionary

### Readonly values

values: T\[\]

Gets the array of values in the dictionary

## Methods

### add

- add(key: string, value: T): void

<!-- -->

- Adds a key-value pair to the dictionary

  #### Parameters

  - ##### key: string

    The string key

  - ##### value: T

    The object value

  #### Returns void

### containsKey

- containsKey(key: string): boolean

<!-- -->

- Checks if the dictionary contains an item key

  #### Parameters

  - ##### key: string

  #### Returns boolean

### item

- item(key: string): T

<!-- -->

- Gets the item by string key

  #### Parameters

  - ##### key: string

  #### Returns T

### remove

- remove(key: string): T

<!-- -->

- Removes an item by string key

  #### Parameters

  - ##### key: string

  #### Returns T

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
