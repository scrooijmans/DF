<img src="out_scichartv4/typedoc/classes/categorycoordinatecalculator_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/categorycoordinatecalculator.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [CategoryCoordinateCalculator](https://www.scichart.com/documentation/js/v4/typedoc/classes/categorycoordinatecalculator.html)

# Class CategoryCoordinateCalculator

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

Provides an implementation of Numeric [Coordinate Calculator](https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html) which transforms numeric data indexes for [CategoryAxis](https://www.scichart.com/documentation/js/v4/typedoc/enums/eaxistype.html#categoryaxis) to pixel coordinates and vice versa.

remarks  
SciChart's <a href="https://www.scichart.com/javascript-chart-features" class="external">JavaScript Charts</a> perform conversion operations between data-coordinates for all drawing and axis measurements.

You can fetch a {link CategoryCoordinateCalculator} instance by calling [AxisCore.getCurrentCoordinateCalculator](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#getcurrentcoordinatecalculator) on a [CategoryAxis](https://www.scichart.com/documentation/js/v4/typedoc/enums/eaxistype.html#categoryaxis). This will return a unique calculator for the current draw pass.

You can convert pixel to data-indexes and back by using the following code. An additional method for Category calculators transforms between data-value and index:

``` ts
const axis: AxisCore;
const calc = axis.getCurrentCoordinateCalculator();

const pixel = calc.getCoordinate(11); // Gets the pixel coordinate for data at index 11
const dataIndex = calc.getDataValue(pixel); // Performs the inverse operation to get data-value
const dataValue = calc.transformIndexToData(dataIndex); // Converts index to data-value
```

Use the Coordinate calculators when drawing, placing markers, annotations or if you want to place a tooltip over the chart.

### Hierarchy

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html" class="tsd-signature-type">CoordinateCalculatorBase</a>
  - CategoryCoordinateCalculator

### Implements

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html" class="tsd-signature-type">IDeletable</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html" class="tsd-signature-type">IDeletable</a>

## Index

### Constructors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/categorycoordinatecalculator.html#constructor" class="tsd-kind-icon">constructor</a>

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/categorycoordinatecalculator.html#basexvalues" class="tsd-kind-icon">baseXValues</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/categorycoordinatecalculator.html#hasflippedcoordinates" class="tsd-kind-icon">hasFlippedCoordinates</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/categorycoordinatecalculator.html#indexmax" class="tsd-kind-icon">indexMax</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/categorycoordinatecalculator.html#indexmin" class="tsd-kind-icon">indexMin</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/categorycoordinatecalculator.html#iscategorycoordinatecalculator" class="tsd-kind-icon">isCategoryCoordinateCalculator</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/categorycoordinatecalculator.html#nativecalculator" class="tsd-kind-icon">nativeCalculator</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/categorycoordinatecalculator.html#offset" class="tsd-kind-icon">offset</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/categorycoordinatecalculator.html#viewportdimension" class="tsd-kind-icon">viewportDimension</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/categorycoordinatecalculator.html#visiblemax" class="tsd-kind-icon">visibleMax</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/categorycoordinatecalculator.html#visiblemin" class="tsd-kind-icon">visibleMin</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/categorycoordinatecalculator.html#webassemblycontext" class="tsd-kind-icon">webAssemblyContext</a>

### Accessors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/categorycoordinatecalculator.html#isflipped" class="tsd-kind-icon">isFlipped</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/categorycoordinatecalculator.html#delete" class="tsd-kind-icon">delete</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/categorycoordinatecalculator.html#getcoordwidth" class="tsd-kind-icon">getCoordWidth</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/categorycoordinatecalculator.html#getcoordinate" class="tsd-kind-icon">getCoordinate</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/categorycoordinatecalculator.html#getdatavalue" class="tsd-kind-icon">getDataValue</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/categorycoordinatecalculator.html#getdatawidth" class="tsd-kind-icon">getDataWidth</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/categorycoordinatecalculator.html#transformdatatoindex" class="tsd-kind-icon">transformDataToIndex</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/categorycoordinatecalculator.html#transformindextodata" class="tsd-kind-icon">transformIndexToData</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/categorycoordinatecalculator.html#translateby" class="tsd-kind-icon">translateBy</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/categorycoordinatecalculator.html#zoomtranslateby" class="tsd-kind-icon">zoomTranslateBy</a>

## Constructors

### constructor

- new CategoryCoordinateCalculator(webAssemblyContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart" class="tsd-signature-type">TSciChart</a>, viewportDimension: number, visibleMin: number, visibleMax: number, offset?: number): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/categorycoordinatecalculator.html" class="tsd-signature-type">CategoryCoordinateCalculator</a>

<!-- -->

- Creates an instance of CategoryCoordinateCalculator

  #### Parameters

  - ##### webAssemblyContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart" class="tsd-signature-type">TSciChart</a>

    The [SciChart 2D WebAssembly Context](https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart) or {@link TSciChart2D \| SciChart 2D WebAssembly Context} containing native methods and access to our WebGL2 Engine and WebAssembly numerical methods

  - ##### viewportDimension: number

    The size of the associated [Axis](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html) at the time of drawing

  - ##### visibleMin: number

    The [CategoryAxis.visibleRange](https://www.scichart.com/documentation/js/v4/typedoc/classes/categoryaxis.html#visiblerange).min at the time of drawing, corresponding to the minimum data-index visible

  - ##### visibleMax: number

    The [CategoryAxis.visibleRange](https://www.scichart.com/documentation/js/v4/typedoc/classes/categoryaxis.html#visiblerange).max at the time of drawing, corresponding to the maximum data-index visible

  - ##### Default value offset: number = 0

    A constant pixel offset used in coordinate calculations

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/categorycoordinatecalculator.html" class="tsd-signature-type">CategoryCoordinateCalculator</a>

## Properties

### baseXValues

baseXValues: SCRTDoubleVector

The primary chart series X-values, required for category calculations and interpolation / extrapolation

### Readonly hasFlippedCoordinates

hasFlippedCoordinates: boolean

When true, this coordinate calculator has flipped coordinates

### Readonly indexMax

indexMax: number

The indexMax is the [CategoryAxis.visibleRange](https://www.scichart.com/documentation/js/v4/typedoc/classes/categoryaxis.html#visiblerange).max at the time of drawing, corresponding to the maximum data-index visible

### Readonly indexMin

indexMin: number

The indexMin is the [CategoryAxis.visibleRange](https://www.scichart.com/documentation/js/v4/typedoc/classes/categoryaxis.html#visiblerange).min at the time of drawing, corresponding to the minimum data-index visible

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

- Converts the Data-value to a pixel coordinate Performs the inverse operation to [getDataValue](https://www.scichart.com/documentation/js/v4/typedoc/classes/categorycoordinatecalculator.html#getdatavalue)

  #### Parameters

  - ##### dataValue: number

    The data-value

  #### Returns number

  the pixel coordinate

### getDataValue

- getDataValue(coordinate: number): number

<!-- -->

- Converts the pixel coordinate to a Data-value. Performs the inverse operation to [getCoordinate](https://www.scichart.com/documentation/js/v4/typedoc/classes/categorycoordinatecalculator.html#getcoordinate)

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

### transformDataToIndex

- transformDataToIndex(dataValue: number): number

<!-- -->

- Transforms an Data-value to Index, with extrapolation and interpolation for values found outside of [the Primary Chart series X-Values](https://www.scichart.com/documentation/js/v4/typedoc/classes/categorycoordinatecalculator.html#basexvalues)

  #### Parameters

  - ##### dataValue: number

  #### Returns number

  the index

### transformIndexToData

- transformIndexToData(index: number): number

<!-- -->

- Transforms an Index to a Data-value, with extrapolation and interpolation for values found outside of [the Primary Chart series X-Values](https://www.scichart.com/documentation/js/v4/typedoc/classes/categorycoordinatecalculator.html#basexvalues)

  #### Parameters

  - ##### index: number

    the index to transform

  #### Returns number

  the Data-value

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
