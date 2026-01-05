<img src="out_scichartv4/typedoc/classes/renderpassdata_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassdata.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [RenderPassData](https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassdata.html)

# Class RenderPassData

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

summary  
RenderPassData contains properties which are passed to [BaseRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries.html) at the time of drawing

### Hierarchy

- RenderPassData

## Index

### Constructors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassdata.html#constructor" class="tsd-kind-icon">constructor</a>

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassdata.html#getxcoordinatecalculator" class="tsd-kind-icon">getxCoordinateCalculator</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassdata.html#getycoordinatecalculator" class="tsd-kind-icon">getyCoordinateCalculator</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassdata.html#indexrange" class="tsd-kind-icon">indexRange</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassdata.html#isverticalchart" class="tsd-kind-icon">isVerticalChart</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassdata.html#pointseries" class="tsd-kind-icon">pointSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassdata.html#resamplinghash" class="tsd-kind-icon">resamplingHash</a>

### Accessors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassdata.html#xcoordinatecalculator" class="tsd-kind-icon">xCoordinateCalculator</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassdata.html#ycoordinatecalculator" class="tsd-kind-icon">yCoordinateCalculator</a>

## Constructors

### constructor

- new RenderPassData(indexRange: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a> \| undefined, getxCoordinateCalculator: () =\> <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html" class="tsd-signature-type">CoordinateCalculatorBase</a>, getyCoordinateCalculator: () =\> <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html" class="tsd-signature-type">CoordinateCalculatorBase</a>, isVerticalChart: boolean, pointSeries?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointseries.html" class="tsd-signature-type">IPointSeries</a>, resamplingHash?: number): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassdata.html" class="tsd-signature-type">RenderPassData</a>

<!-- -->

- Creates an instance of RenderPassData

  #### Parameters

  - ##### indexRange: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a> \| undefined

    The min and max index to data-range currently visible on the [SciChartSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html)

  - ##### getxCoordinateCalculator: () =\> <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html" class="tsd-signature-type">CoordinateCalculatorBase</a>

    A function to get the XAxis [Coordinate Calculator](https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html), used to transform between pixel and data-coordinates

    - - (): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html" class="tsd-signature-type">CoordinateCalculatorBase</a>

      <!-- -->

      - #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html" class="tsd-signature-type">CoordinateCalculatorBase</a>

  - ##### getyCoordinateCalculator: () =\> <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html" class="tsd-signature-type">CoordinateCalculatorBase</a>

    A function to get the YAxis [Coordinate Calculator](https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html), used to transform between pixel and data-coordinates

    - - (): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html" class="tsd-signature-type">CoordinateCalculatorBase</a>

      <!-- -->

      - #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html" class="tsd-signature-type">CoordinateCalculatorBase</a>

  - ##### isVerticalChart: boolean

    A flag indicating if the chart is currently vertically arranged (XAxis on the left, YAxis on the top/bottom)

  - ##### Optional pointSeries: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointseries.html" class="tsd-signature-type">IPointSeries</a>

    The point series

  - ##### Optional resamplingHash: number

    The resampling hash value, used for caching

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassdata.html" class="tsd-signature-type">RenderPassData</a>

## Properties

### Readonly getxCoordinateCalculator

getxCoordinateCalculator: () =\> <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html" class="tsd-signature-type">CoordinateCalculatorBase</a>

A function to get the XAxis [Coordinate Calculator](https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html), used to transform between pixel and data-coordinates

#### Type declaration

- - (): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html" class="tsd-signature-type">CoordinateCalculatorBase</a>

  <!-- -->

  - #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html" class="tsd-signature-type">CoordinateCalculatorBase</a>

### Readonly getyCoordinateCalculator

getyCoordinateCalculator: () =\> <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html" class="tsd-signature-type">CoordinateCalculatorBase</a>

A function to get the YAxis [Coordinate Calculator](https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html), used to transform between pixel and data-coordinates

#### Type declaration

- - (): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html" class="tsd-signature-type">CoordinateCalculatorBase</a>

  <!-- -->

  - #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html" class="tsd-signature-type">CoordinateCalculatorBase</a>

### Readonly indexRange

indexRange: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a> \| undefined

The min and max index to data-range currently visible on the [SciChartSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html)

### Readonly isVerticalChart

isVerticalChart: boolean

A flag indicating if the chart is currently vertically arranged (XAxis on the left, YAxis on the top/bottom)

### Optional Readonly pointSeries

pointSeries: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointseries.html" class="tsd-signature-type">IPointSeries</a>

The point series

### Optional Readonly resamplingHash

resamplingHash: number

The resampling hash value, used for caching

## Accessors

### xCoordinateCalculator

- get xCoordinateCalculator(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html" class="tsd-signature-type">CoordinateCalculatorBase</a>

<!-- -->

- Get the XAxis [Coordinate Calculator](https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html), used to transform between pixel and data-coordinates

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html" class="tsd-signature-type">CoordinateCalculatorBase</a>

### yCoordinateCalculator

- get yCoordinateCalculator(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html" class="tsd-signature-type">CoordinateCalculatorBase</a>

<!-- -->

- Get the YAxis [Coordinate Calculator](https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html), used to transform between pixel and data-coordinates

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html" class="tsd-signature-type">CoordinateCalculatorBase</a>

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
