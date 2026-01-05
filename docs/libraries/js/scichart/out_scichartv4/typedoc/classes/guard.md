<img src="out_scichartv4/typedoc/classes/guard_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/guard.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [Guard](https://www.scichart.com/documentation/js/v4/typedoc/classes/guard.html)

# Class Guard

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

Guard class for sanity-checking (null checking, check if property is true, check arrays same length etc...)

### Hierarchy

- Guard

## Index

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/guard.html#argumentisrealinteger" class="tsd-kind-icon">argumentIsRealInteger</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/guard.html#argumentisrealnumber" class="tsd-kind-icon">argumentIsRealNumber</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/guard.html#arrayssamelength" class="tsd-kind-icon">arraysSameLength</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/guard.html#arrayssamelengtharr" class="tsd-kind-icon">arraysSameLengthArr</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/guard.html#istrue" class="tsd-kind-icon">isTrue</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/guard.html#notnull" class="tsd-kind-icon">notNull</a>

## Methods

### Static argumentIsRealInteger

- argumentIsRealInteger(d: number, name: string): void

<!-- -->

- Asserts a numeric argument is a real number: not null (undefined), not NaN and not infinite

  #### Parameters

  - ##### d: number

  - ##### name: string

  #### Returns void

### Static argumentIsRealNumber

- argumentIsRealNumber(d: number, name: string): void

<!-- -->

- Asserts a numeric argument is a real number: not null (undefined), not NaN and not infinite

  #### Parameters

  - ##### d: number

  - ##### name: string

  #### Returns void

### Static arraysSameLength

- arraysSameLength(arg: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#numberarray" class="tsd-signature-type">NumberArray</a> \| any\[\], name1: string, arg2: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#numberarray" class="tsd-signature-type">NumberArray</a> \| any\[\], name2: string): void

<!-- -->

- Asserts two arrays are not null (undefined) and are the same legnth

  throws  
  Error - an error when the arrays are not the same length

  #### Parameters

  - ##### arg: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#numberarray" class="tsd-signature-type">NumberArray</a> \| any\[\]

    The first array

  - ##### name1: string

    The first array parameter name

  - ##### arg2: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#numberarray" class="tsd-signature-type">NumberArray</a> \| any\[\]

    The second array

  - ##### name2: string

    The second array parameter name

  #### Returns void

### Static arraysSameLengthArr

- arraysSameLengthArr(args?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#telement" class="tsd-signature-type">TElement</a>\[\]): void

<!-- -->

- Asserts multiple arrays are not null (undefined) and are the same legnth

  throws  
  Error - an error when the arrays are not the same length

  #### Parameters

  - ##### Default value args: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#telement" class="tsd-signature-type">TElement</a>\[\] = \[\]

    The array of arrays

  #### Returns void

### Static isTrue

- isTrue(value: boolean, message: string): void

<!-- -->

- Asserts a parameter is true

  throws  
  Error - an error when the parameter is false

  #### Parameters

  - ##### value: boolean

  - ##### message: string

    The message to show if not true

  #### Returns void

### Static notNull

- notNull(arg: any, name: string): void

<!-- -->

- Asserts the argument is not null

  throws  
  Error - an error when the argument is null or undefined

  #### Parameters

  - ##### arg: any

    The argument

  - ##### name: string

    The argument name

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
