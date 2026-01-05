<img src="out_scichartv4/typedoc/classes/numberrange_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [NumberRange](https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html)

# Class NumberRange

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

Defines a number range with numeric min, max

### Hierarchy

- NumberRange

## Index

### Constructors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html#constructor" class="tsd-kind-icon">constructor</a>

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html#max" class="tsd-kind-icon">max</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html#min" class="tsd-kind-icon">min</a>

### Accessors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html#diff" class="tsd-kind-icon">diff</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html#clip" class="tsd-kind-icon">clip</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html#equals" class="tsd-kind-icon">equals</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html#growby" class="tsd-kind-icon">growBy</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html#growbylog" class="tsd-kind-icon">growByLog</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html#isdefined" class="tsd-kind-icon">isDefined</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html#iszero" class="tsd-kind-icon">isZero</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html#tostring" class="tsd-kind-icon">toString</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html#union" class="tsd-kind-icon">union</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html#areequal" class="tsd-kind-icon">areEqual</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html#hydrate" class="tsd-kind-icon">hydrate</a>

## Constructors

### constructor

- new NumberRange(min?: number, max?: number): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

<!-- -->

- #### Parameters

  - ##### Default value min: number = 0

  - ##### Default value max: number = 10

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

## Properties

### Readonly max

max: number

### Readonly min

min: number

## Accessors

### diff

- get diff(): number

<!-- -->

- Returns a difference between max and min

  #### Returns number

## Methods

### clip

- clip(range: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

<!-- -->

- Clips a range to a min, max value

  remarks  
  E.g. if the current range is \[1,5\] and input is \[2,6\] then result will be \[2,5\]

  #### Parameters

  - ##### range: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

  The new clipped range

### equals

- equals(other: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>): boolean

<!-- -->

- Returns true if the range equals another by value

  #### Parameters

  - ##### other: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

  #### Returns boolean

### growBy

- growBy(range: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

<!-- -->

- Grows a range by a min and max factor

  remarks  
  If the current range is \[5,10\] and the input range is \[0.1, 0.1\] the current range will be grown by 10%, so \[4.5, 10.5\]

  #### Parameters

  - ##### range: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

    The grow factor

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

### growByLog

- growByLog(range: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>, logBase: number): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

<!-- -->

- #### Parameters

  - ##### range: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

  - ##### logBase: number

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

### isDefined

- isDefined(): boolean

<!-- -->

- Returns true if the range is defined (is a real number, not NaN, not infinite, and not undefined)

  #### Returns boolean

### isZero

- isZero(): boolean

<!-- -->

- Returns true if the range min === range max

  #### Returns boolean

### toString

- toString(): string

<!-- -->

- Returns a string representation of a [NumberRange](https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html) for easy debugging

  #### Returns string

### union

- union(range: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

<!-- -->

- Returns a new [NumberRange](https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html) which is the union of two ranges

  remarks  
  E.g. if current range is \[1,2\] and input is \[2,3\] the result range will be \[1,3\]

  #### Parameters

  - ##### range: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

  the new union range

### Static areEqual

- areEqual(range1: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>, range2: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>): boolean

<!-- -->

- #### Parameters

  - ##### range1: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

  - ##### range2: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

  #### Returns boolean

### Static hydrate

- hydrate(range: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a> \| { max: number; min: number }): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

<!-- -->

- Turns a { min, max } object into a [NumberRange](https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html), most helpful for JSON deserialization

  #### Parameters

  - ##### range: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a> \| { max: number; min: number }

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

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
