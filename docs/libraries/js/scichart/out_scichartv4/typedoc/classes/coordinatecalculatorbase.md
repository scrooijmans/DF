<img src="out_scichartv4/typedoc/classes/coordinatecalculatorbase_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [CoordinateCalculatorBase](https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html)

# Class CoordinateCalculatorBase

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

The CoordinateCalculatorBase class provides methods for converting between Pixel and Data coordinates

remarks  
SciChart's <a href="https://www.scichart.com/javascript-chart-features" class="external">JavaScript Charts</a> perform conversion operations between data-coordinates for all drawing and axis measurements.

You can fetch a CoordinateCalculator instance by calling [AxisCore.getCurrentCoordinateCalculator](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#getcurrentcoordinatecalculator). This will return a unique calculator for the current draw pass.

You can convert pixel to data-coordinates and back by using the following code:

``` ts
const axis: AxisCore;
const calc = axis.getCurrentCoordinateCalculator();

const pixel = calc.getCoordinate(1.23); // Gets the pixel coordinate for data-value 1.23
const dataValue = cald.getDataValue(pixel); // Performs the inverse operation to get data-value
```

Use the Coordinate calculators when drawing, placing markers, annotations or if you want to place a tooltip over the chart.

### Hierarchy

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/deletableentity.html" class="tsd-signature-type">DeletableEntity</a>
  - CoordinateCalculatorBase
    - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/categorycoordinatecalculator.html" class="tsd-signature-type">CategoryCoordinateCalculator</a>
    - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/flippedcategorycoordinatecalculator.html" class="tsd-signature-type">FlippedCategoryCoordinateCalculator</a>
    - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/flippednumericcoordinatecalculator.html" class="tsd-signature-type">FlippedNumericCoordinateCalculator</a>
    - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/logarithmiccoordinatecalculator.html" class="tsd-signature-type">LogarithmicCoordinateCalculator</a>
    - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numericcoordinatecalculator.html" class="tsd-signature-type">NumericCoordinateCalculator</a>

### Implements

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html" class="tsd-signature-type">IDeletable</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html" class="tsd-signature-type">IDeletable</a>

## Index

### Constructors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html#constructor" class="tsd-kind-icon">constructor</a>

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html#hasflippedcoordinates" class="tsd-kind-icon">hasFlippedCoordinates</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html#iscategorycoordinatecalculator" class="tsd-kind-icon">isCategoryCoordinateCalculator</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html#nativecalculator" class="tsd-kind-icon">nativeCalculator</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html#offset" class="tsd-kind-icon">offset</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html#viewportdimension" class="tsd-kind-icon">viewportDimension</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html#visiblemax" class="tsd-kind-icon">visibleMax</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html#visiblemin" class="tsd-kind-icon">visibleMin</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html#webassemblycontext" class="tsd-kind-icon">webAssemblyContext</a>

### Accessors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html#isflipped" class="tsd-kind-icon">isFlipped</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html#delete" class="tsd-kind-icon">delete</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html#getcoordwidth" class="tsd-kind-icon">getCoordWidth</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html#getcoordinate" class="tsd-kind-icon">getCoordinate</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html#getdatavalue" class="tsd-kind-icon">getDataValue</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html#getdatawidth" class="tsd-kind-icon">getDataWidth</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html#translateby" class="tsd-kind-icon">translateBy</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html#zoomtranslateby" class="tsd-kind-icon">zoomTranslateBy</a>

## Constructors

### Protected constructor

- new CoordinateCalculatorBase(webAssemblyContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart" class="tsd-signature-type">TSciChart</a> \| <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart3d" class="tsd-signature-type">TSciChart3D</a>, viewportDimension: number, visibleMin: number, visibleMax: number, offset: number, hasFlippedCoordinates: boolean, isCategoryCoordinateCalculator?: boolean): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html" class="tsd-signature-type">CoordinateCalculatorBase</a>

<!-- -->

- Creates an instance of the CoordinateCalculatorBase

  #### Parameters

  - ##### webAssemblyContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart" class="tsd-signature-type">TSciChart</a> \| <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart3d" class="tsd-signature-type">TSciChart3D</a>

    The [SciChart 2D WebAssembly Context](https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart) or {@link TSciChart2D \| SciChart 2D WebAssembly Context} containing native methods and access to our WebGL2 Engine and WebAssembly numerical methods

  - ##### viewportDimension: number

    The size of the associated [Axis](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html) at the time of drawing

  - ##### visibleMin: number

    The [AxisCore.visibleRange](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#visiblerange).min at the time of drawing

  - ##### visibleMax: number

    The [AxisCore.visibleRange](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#visiblerange).max at the time of drawing

  - ##### offset: number

    A constant pixel offset used in coordinate calculations

  - ##### hasFlippedCoordinates: boolean

    When true, this calculator has flipped coordinates

  - ##### Default value isCategoryCoordinateCalculator: boolean = false

    When true, this calculator behaves as a Category coordinate calculator, using index not x-value for measuring

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html" class="tsd-signature-type">CoordinateCalculatorBase</a>

## Properties

### Readonly hasFlippedCoordinates

hasFlippedCoordinates: boolean

When true, this coordinate calculator has flipped coordinates

### Readonly isCategoryCoordinateCalculator

isCategoryCoordinateCalculator: boolean

When true, this coordinate calculator behaves as a Category coordinate calculator, using index not x-value for measuring

### nativeCalculator

nativeCalculator: CoordinateCalculator

Gets the native (WebAssembly) {@link CoordinateCalculator} instance

### offset

offset: number

Gets or sets a constant offset in pixels for all generated coordinates

### Readonly viewportDimension

viewportDimension: number

Gets or sets the ViewportDimension, corresponding to the size of the associated [Axis](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html) at the time of drawing

### Readonly visibleMax

visibleMax: number

Gets or sets the Visible maximum value, corresponding to [AxisCore.visibleRange](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#visiblerange).max at the time of drawing

### Readonly visibleMin

visibleMin: number

Gets or sets the Visible minimum value, corresponding to [AxisCore.visibleRange](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#visiblerange).min at the time of drawing

### Protected webAssemblyContext

webAssemblyContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart" class="tsd-signature-type">TSciChart</a> \| <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart3d" class="tsd-signature-type">TSciChart3D</a>

The [SciChart 2D WebAssembly Context](https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart) or {@link TSciChart2D \| SciChart 2D WebAssembly Context} containing native methods and access to our WebGL2 Engine and WebAssembly numerical methods

## Accessors

### isFlipped

- get isFlipped(): boolean

<!-- -->

- Returns isFlipped flag. By default for cartesian chart X axis is not flipped and Y axis is flipped. By default for polar chart both axes are not flipped

  #### Returns boolean

## Methods

### delete

- delete(): void

<!-- -->

- Deletes native (WebAssembly) memory used by this type, after which it cannot be used.

  remarks  
  Call .delete() before finishing with the object to ensure that WebAssmembly memory leaks do not occur.

  All elements within SciChart's High Performance <a href="https://www.scichart.com/javascript-chart-features" class="external">Realtime JavaScript Charts</a> which implement [IDeletable](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html) must be deleted manually to free native (WebAssembly) memory

  #### Returns void

### getCoordWidth

- getCoordWidth(dataWidth: number): number

<!-- -->

- Converts data width into coordinate width

  #### Parameters

  - ##### dataWidth: number

  #### Returns number

### getCoordinate

- getCoordinate(dataValue: number): number

<!-- -->

- Converts the Data-value to a pixel coordinate Performs the inverse operation to [getDataValue](https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html#getdatavalue)

  #### Parameters

  - ##### dataValue: number

    The data-value

  #### Returns number

  the pixel coordinate

### getDataValue

- getDataValue(coordinate: number): number

<!-- -->

- Converts the pixel coordinate to a Data-value. Performs the inverse operation to [getCoordinate](https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html#getcoordinate)

  #### Parameters

  - ##### coordinate: number

    The pixel coordiante

  #### Returns number

  the data value

### getDataWidth

- getDataWidth(coordWidth: number): number

<!-- -->

- Converts coordinate width into data width

  #### Parameters

  - ##### coordWidth: number

  #### Returns number

### translateBy

- translateBy(pixels: number, range: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

<!-- -->

- Translates a [NumberRange](https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html) in Data-coordinates by a specified number of pixels, performing intermediate calculations from data-value to pixel and back to perform the translation

  #### Parameters

  - ##### pixels: number

    The pixels to translate

  - ##### range: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

    The [NumberRange](https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html) to translate. For example this could be an [Axis.visibleRange](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#visiblerange)

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

  The translated range

### zoomTranslateBy

- zoomTranslateBy(minFraction: number, maxFraction: number, inputRange: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

<!-- -->

- Zooms a [NumberRange](https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html) by a specified fractional amount

  #### Parameters

  - ##### minFraction: number

    The fraction to zoom the [NumberRange.min](https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html#min) by. A fraction of 0.1 zooms the minimum by 10%

  - ##### maxFraction: number

    The fraction to zoom the [NumberRange.max](https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html#max) by. A fraction of 0.1 zooms the maximum by 10%

  - ##### inputRange: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

    The [NumberRange](https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html) to zoom

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

  The zoomed range

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
