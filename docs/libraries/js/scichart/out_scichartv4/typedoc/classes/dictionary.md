<img src="out_scichartv4/typedoc/classes/dictionary_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/dictionary.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [Dictionary](https://www.scichart.com/documentation/js/v4/typedoc/classes/dictionary.html)

# Class Dictionary\<T\>

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

Generic dictionary class keyed by string

### Type parameters

- #### T

### Hierarchy

- Dictionary

### Implements

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idictionary.html" class="tsd-signature-type">IDictionary</a>\<T\>

## Index

### Accessors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/dictionary.html#count" class="tsd-kind-icon">count</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/dictionary.html#keys" class="tsd-kind-icon">keys</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/dictionary.html#values" class="tsd-kind-icon">values</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/dictionary.html#add" class="tsd-kind-icon">add</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/dictionary.html#clear" class="tsd-kind-icon">clear</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/dictionary.html#containskey" class="tsd-kind-icon">containsKey</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/dictionary.html#item" class="tsd-kind-icon">item</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/dictionary.html#remove" class="tsd-kind-icon">remove</a>

## Accessors

### count

- get count(): number

<!-- -->

- Gets the number of items in the dictionary

  inheritdoc  

  #### Returns number

### keys

- get keys(): string\[\]

<!-- -->

- Gets an array of string keys in the dictionary

  inheritdoc  

  #### Returns string\[\]

### values

- get values(): T\[\]

<!-- -->

- Gets the array of values in the dictionary

  inheritdoc  

  #### Returns T\[\]

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

### clear

- clear(): void

<!-- -->

- Clears the dictionary

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
