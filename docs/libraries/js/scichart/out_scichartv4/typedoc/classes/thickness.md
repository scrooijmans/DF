<img src="out_scichartv4/typedoc/classes/thickness_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/thickness.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [Thickness](https://www.scichart.com/documentation/js/v4/typedoc/classes/thickness.html)

# Class Thickness

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

### Hierarchy

- Thickness

## Index

### Constructors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/thickness.html#constructor" class="tsd-kind-icon">constructor</a>

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/thickness.html#bottom" class="tsd-kind-icon">bottom</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/thickness.html#left" class="tsd-kind-icon">left</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/thickness.html#right" class="tsd-kind-icon">right</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/thickness.html#top" class="tsd-kind-icon">top</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/thickness.html#areequal" class="tsd-kind-icon">areEqual</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/thickness.html#fromnumber" class="tsd-kind-icon">fromNumber</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/thickness.html#fromstring" class="tsd-kind-icon">fromString</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/thickness.html#mergeadd" class="tsd-kind-icon">mergeAdd</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/thickness.html#mergemax" class="tsd-kind-icon">mergeMax</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/thickness.html#mergesubtract" class="tsd-kind-icon">mergeSubtract</a>

## Constructors

### constructor

- new Thickness(top: number, right: number, bottom: number, left: number): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/thickness.html" class="tsd-signature-type">Thickness</a>

<!-- -->

- Creates an instance of a Thickness object, with top, right, bottom and left

  #### Parameters

  - ##### top: number

  - ##### right: number

  - ##### bottom: number

  - ##### left: number

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/thickness.html" class="tsd-signature-type">Thickness</a>

## Properties

### bottom

bottom: number

Gets or sets the bottom amount

### left

left: number

Gets or sets the left amount

### right

right: number

Gets or sets the right amount

### top

top: number

Gets or sets the top amount

## Methods

### Static areEqual

- areEqual(first: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/thickness.html" class="tsd-signature-type">Thickness</a>, second: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/thickness.html" class="tsd-signature-type">Thickness</a>): boolean

<!-- -->

- Returns whether two [Thickness](https://www.scichart.com/documentation/js/v4/typedoc/classes/thickness.html) instances are equal or not

  example  
  const t1 = new Thickness(4,4,4,4); const t2 = new Thickness.fromNumber(4); console.log(Thickness.areEqual(t1, t2)); // True

  #### Parameters

  - ##### first: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/thickness.html" class="tsd-signature-type">Thickness</a>

  - ##### second: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/thickness.html" class="tsd-signature-type">Thickness</a>

  #### Returns boolean

### Static fromNumber

- fromNumber(value: number): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/thickness.html" class="tsd-signature-type">Thickness</a>

<!-- -->

- Creates a Thickness (margin or padding) from a single value, e.g. 10, would return a thickness with top, right, bottom left = 10

  #### Parameters

  - ##### value: number

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/thickness.html" class="tsd-signature-type">Thickness</a>

### Static fromString

- fromString(str: string): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/thickness.html" class="tsd-signature-type">Thickness</a>

<!-- -->

- Creates a Thickness (margin or padding) from string, e.g. "25 50 75 100". Order is top, right, bottom left. Same as in Css

  #### Parameters

  - ##### str: string

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/thickness.html" class="tsd-signature-type">Thickness</a>

### Static mergeAdd

- mergeAdd(first: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/thickness.html" class="tsd-signature-type">Thickness</a>, second: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/thickness.html" class="tsd-signature-type">Thickness</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/thickness.html" class="tsd-signature-type">Thickness</a>

<!-- -->

- Returns the sum of each side of 2 components in a new [Thickness](https://www.scichart.com/documentation/js/v4/typedoc/classes/thickness.html) object

  example  
  const t1 = new Thickness(1,2,3,4); const t2 = new Thickness(4,3,2,1); console.log(Thickness.mergeAdd(t1, t2)); // Thickness { top: 5, right: 5, bottom: 5, left: 5 }

  #### Parameters

  - ##### first: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/thickness.html" class="tsd-signature-type">Thickness</a>

  - ##### second: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/thickness.html" class="tsd-signature-type">Thickness</a>

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/thickness.html" class="tsd-signature-type">Thickness</a>

### Static mergeMax

- mergeMax(first: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/thickness.html" class="tsd-signature-type">Thickness</a>, second: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/thickness.html" class="tsd-signature-type">Thickness</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/thickness.html" class="tsd-signature-type">Thickness</a>

<!-- -->

- Returns the max of each side of 2 components in a new [Thickness](https://www.scichart.com/documentation/js/v4/typedoc/classes/thickness.html) object

  example  
  const t1 = new Thickness(1,2,3,4); const t2 = new Thickness(4,3,2,1); console.log(Thickness.mergeMax(t1, t2)); // Thickness { top: 4, right: 3, bottom: 3, left: 4 }

  #### Parameters

  - ##### first: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/thickness.html" class="tsd-signature-type">Thickness</a>

  - ##### second: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/thickness.html" class="tsd-signature-type">Thickness</a>

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/thickness.html" class="tsd-signature-type">Thickness</a>

### Static mergeSubtract

- mergeSubtract(first: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/thickness.html" class="tsd-signature-type">Thickness</a>, second: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/thickness.html" class="tsd-signature-type">Thickness</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/thickness.html" class="tsd-signature-type">Thickness</a>

<!-- -->

- Returns the difference of each side of 2 components in a new [Thickness](https://www.scichart.com/documentation/js/v4/typedoc/classes/thickness.html) object

  example  
  const t1 = new Thickness(1,2,3,4); const t2 = new Thickness(4,3,2,1); console.log(Thickness.mergeSubtract(t1, t2)); // Thickness { top: -3, right: -1, bottom: 1, left: 3 }

  #### Parameters

  - ##### first: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/thickness.html" class="tsd-signature-type">Thickness</a>

  - ##### second: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/thickness.html" class="tsd-signature-type">Thickness</a>

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/thickness.html" class="tsd-signature-type">Thickness</a>

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
