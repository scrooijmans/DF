<img src="out_scichartv4/typedoc/classes/rect_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [Rect](https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html)

# Class Rect

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

Class to represent a rectangle in 2D space

### Hierarchy

- Rect

## Index

### Constructors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html#constructor" class="tsd-kind-icon">constructor</a>

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html#height" class="tsd-kind-icon">height</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html#width" class="tsd-kind-icon">width</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html#x" class="tsd-kind-icon">x</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html#y" class="tsd-kind-icon">y</a>

### Accessors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html#bottom" class="tsd-kind-icon">bottom</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html#left" class="tsd-kind-icon">left</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html#right" class="tsd-kind-icon">right</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html#top" class="tsd-kind-icon">top</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html#clippointtorect" class="tsd-kind-icon">clipPointToRect</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html#create" class="tsd-kind-icon">create</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html#createcopy" class="tsd-kind-icon">createCopy</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html#createwithcoords" class="tsd-kind-icon">createWithCoords</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html#createwithpoints" class="tsd-kind-icon">createWithPoints</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html#createzero" class="tsd-kind-icon">createZero</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html#hydrate" class="tsd-kind-icon">hydrate</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html#intersect" class="tsd-kind-icon">intersect</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html#isequal" class="tsd-kind-icon">isEqual</a>

## Constructors

### constructor

- new Rect(x: number, y: number, width: number, height: number): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>

<!-- -->

- Creates a rect with X,Y,Width,Height

  remarks  
  To create a rect with two points, or with left, top right bottom, see the factory functions [Rect.createWithPoints](https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html#createwithpoints) or [Rect.createWithCoords](https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html#createwithcoords)

  #### Parameters

  - ##### x: number

  - ##### y: number

  - ##### width: number

  - ##### height: number

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>

## Properties

### Readonly height

height: number

Gets or sets the height

### Readonly width

width: number

Gets or sets the width

### Readonly x

x: number

Gets or sets the top left X coordinate

### Readonly y

y: number

Gets or sets the top left Y coordinate

## Accessors

### bottom

- get bottom(): number

<!-- -->

- Gets the bottom edge of the rect

  #### Returns number

### left

- get left(): number

<!-- -->

- Gets the left edge of the rect

  #### Returns number

### right

- get right(): number

<!-- -->

- Gets the right edge of the rect

  #### Returns number

### top

- get top(): number

<!-- -->

- Gets the top edge of the rect

  #### Returns number

## Methods

### Static clipPointToRect

- clipPointToRect(point: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/point.html" class="tsd-signature-type">Point</a>, rect: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/point.html" class="tsd-signature-type">Point</a>

<!-- -->

- Clips a point to the rect, so if the point is outside the rect it will be on the boundary of the rect after this operation

  #### Parameters

  - ##### point: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/point.html" class="tsd-signature-type">Point</a>

    The point to clip

  - ##### rect: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>

    The rect

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/point.html" class="tsd-signature-type">Point</a>

### Static create

- create(x: number, y: number, width: number, height: number): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>

<!-- -->

- Creates a rectangle with X,Y top left coordinate and width and height

  #### Parameters

  - ##### x: number

  - ##### y: number

  - ##### width: number

  - ##### height: number

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>

### Static createCopy

- createCopy(rect: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>

<!-- -->

- Clones a rect

  #### Parameters

  - ##### rect: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>

### Static createWithCoords

- createWithCoords(left: number, top: number, right: number, bottom: number): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>

<!-- -->

- Creates a rectangle with left, top ,right, bottom

  #### Parameters

  - ##### left: number

  - ##### top: number

  - ##### right: number

  - ##### bottom: number

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>

### Static createWithPoints

- createWithPoints(point1: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/point.html" class="tsd-signature-type">Point</a>, point2: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/point.html" class="tsd-signature-type">Point</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>

<!-- -->

- Create a rectangle with two points which could be top-left, bottom-right

  #### Parameters

  - ##### point1: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/point.html" class="tsd-signature-type">Point</a>

  - ##### point2: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/point.html" class="tsd-signature-type">Point</a>

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>

### Static createZero

- createZero(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>

<!-- -->

- Creates a zero rect with x,y,w,h = 0

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>

### Static hydrate

- hydrate(input: { height: number; width: number; x: number; y: number }): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>

<!-- -->

- Turns a { x, y, width, height } object into a [NumberRange](https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html), most helpful for JSON deserialization

  #### Parameters

  - ##### input: { height: number; width: number; x: number; y: number }

    - ##### height: number

    - ##### width: number

    - ##### x: number

    - ##### y: number

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>

### Static intersect

- intersect(rect1: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>, rect2: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>

<!-- -->

- #### Parameters

  - ##### rect1: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>

  - ##### rect2: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>

### Static isEqual

- isEqual(rect1: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>, rect2: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>): boolean

<!-- -->

- Returns true if a rect numerically equals another rect

  #### Parameters

  - ##### rect1: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>

  - ##### rect2: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>

  #### Returns boolean

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
