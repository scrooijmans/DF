<img src="out_scichartv4/typedoc/classes/logarithmiccoordinatecalculator_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/logarithmiccoordinatecalculator.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [LogarithmicCoordinateCalculator](https://www.scichart.com/documentation/js/v4/typedoc/classes/logarithmiccoordinatecalculator.html)

# Class LogarithmicCoordinateCalculator

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

Provides an implementation of Numeric [Coordinate Calculator](https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html) which transforms numeric data-values to pixel coordinates using logarithmic scaling and vice versa.

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

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html" class="tsd-signature-type">CoordinateCalculatorBase</a>
  - LogarithmicCoordinateCalculator

### Implements

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html" class="tsd-signature-type">IDeletable</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html" class="tsd-signature-type">IDeletable</a>

## Index

### Constructors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/logarithmiccoordinatecalculator.html#constructor" class="tsd-kind-icon">constructor</a>

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/logarithmiccoordinatecalculator.html#hasflippedcoordinates" class="tsd-kind-icon">hasFlippedCoordinates</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/logarithmiccoordinatecalculator.html#iscategorycoordinatecalculator" class="tsd-kind-icon">isCategoryCoordinateCalculator</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/logarithmiccoordinatecalculator.html#nativecalculator" class="tsd-kind-icon">nativeCalculator</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/logarithmiccoordinatecalculator.html#offset" class="tsd-kind-icon">offset</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/logarithmiccoordinatecalculator.html#viewportdimension" class="tsd-kind-icon">viewportDimension</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/logarithmiccoordinatecalculator.html#visiblemax" class="tsd-kind-icon">visibleMax</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/logarithmiccoordinatecalculator.html#visiblemin" class="tsd-kind-icon">visibleMin</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/logarithmiccoordinatecalculator.html#webassemblycontext" class="tsd-kind-icon">webAssemblyContext</a>

### Accessors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/logarithmiccoordinatecalculator.html#isflipped" class="tsd-kind-icon">isFlipped</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/logarithmiccoordinatecalculator.html#logbase" class="tsd-kind-icon">logBase</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/logarithmiccoordinatecalculator.html#delete" class="tsd-kind-icon">delete</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/logarithmiccoordinatecalculator.html#getcoordwidth" class="tsd-kind-icon">getCoordWidth</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/logarithmiccoordinatecalculator.html#getcoordinate" class="tsd-kind-icon">getCoordinate</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/logarithmiccoordinatecalculator.html#getdatavalue" class="tsd-kind-icon">getDataValue</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/logarithmiccoordinatecalculator.html#getdatawidth" class="tsd-kind-icon">getDataWidth</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/logarithmiccoordinatecalculator.html#translateby" class="tsd-kind-icon">translateBy</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/logarithmiccoordinatecalculator.html#zoomtranslateby" class="tsd-kind-icon">zoomTranslateBy</a>

## Constructors

### constructor

- new LogarithmicCoordinateCalculator(webAssemblyContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart" class="tsd-signature-type">TSciChart</a> \| <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart3d" class="tsd-signature-type">TSciChart3D</a>, viewportDimension: number, visibleMin: number, visibleMax: number, xyDirection: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/exydirection.html" class="tsd-signature-type">EXyDirection</a>, logBase: number, flipCoordinates: boolean, offset?: number): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/logarithmiccoordinatecalculator.html" class="tsd-signature-type">LogarithmicCoordinateCalculator</a>

<!-- -->

- Creates an instance of LogarithmicCoordinateCalculator

  #### Parameters

  - ##### webAssemblyContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart" class="tsd-signature-type">TSciChart</a> \| <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart3d" class="tsd-signature-type">TSciChart3D</a>

    The [SciChart 2D WebAssembly Context](https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart) or {@link TSciChart2D \| SciChart 2D WebAssembly Context} containing native methods and access to our WebGL2 Engine and WebAssembly numerical methods

  - ##### viewportDimension: number

    The size of the associated [Axis](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html) at the time of drawing

  - ##### visibleMin: number

    The [AxisCore.visibleRange](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#visiblerange).min at the time of drawing

  - ##### visibleMax: number

    The [AxisCore.visibleRange](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#visiblerange).max at the time of drawing

  - ##### xyDirection: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/exydirection.html" class="tsd-signature-type">EXyDirection</a>

    Whether the parent axis is an X or Y axis

  - ##### logBase: number

    The Logarithmic base, e.g. 10, for calculating log coordinates

  - ##### flipCoordinates: boolean

    Whether the flip-coordinates flag is true on the associated axis

  - ##### Default value offset: number = 0

    A constant pixel offset used in coordinate calculations

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/logarithmiccoordinatecalculator.html" class="tsd-signature-type">LogarithmicCoordinateCalculator</a>

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

### logBase

- get logBase(): number

<!-- -->

- #### Returns number

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

- Converts the Data-value to a pixel coordinate Performs the inverse operation to [getDataValue](https://www.scichart.com/documentation/js/v4/typedoc/classes/logarithmiccoordinatecalculator.html#getdatavalue)

  #### Parameters

  - ##### dataValue: number

    The data-value

  #### Returns number

  the pixel coordinate

### getDataValue

- getDataValue(coordinate: number): number

<!-- -->

- Converts the pixel coordinate to a Data-value. Performs the inverse operation to [getCoordinate](https://www.scichart.com/documentation/js/v4/typedoc/classes/logarithmiccoordinatecalculator.html#getcoordinate)

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

- #### Parameters

  - ##### pixels: number

  - ##### range: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

### zoomTranslateBy

- zoomTranslateBy(minFraction: number, maxFraction: number, inputRange: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

<!-- -->

- #### Parameters

  - ##### minFraction: number

  - ##### maxFraction: number

  - ##### inputRange: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

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
