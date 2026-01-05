<img src="out_scichartv4/typedoc/interfaces/ixypointseries_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ixypointseries.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [IXyPointSeries](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ixypointseries.html)

# Interface IXyPointSeries

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

### Hierarchy

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointseries.html" class="tsd-signature-type">IPointSeries</a>
  - IXyPointSeries

### Implemented by

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xypointseriesresampled.html" class="tsd-signature-type">XyPointSeriesResampled</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xypointserieswrapped.html" class="tsd-signature-type">XyPointSeriesWrapped</a>

## Index

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ixypointseries.html#arraycount" class="tsd-kind-icon">arrayCount</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ixypointseries.html#count" class="tsd-kind-icon">count</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ixypointseries.html#fifostartindex" class="tsd-kind-icon">fifoStartIndex</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ixypointseries.html#indexes" class="tsd-kind-icon">indexes</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ixypointseries.html#resampled" class="tsd-kind-icon">resampled</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ixypointseries.html#type" class="tsd-kind-icon">type</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ixypointseries.html#xrange" class="tsd-kind-icon">xRange</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ixypointseries.html#xvalues" class="tsd-kind-icon">xValues</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ixypointseries.html#yvalues" class="tsd-kind-icon">yValues</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ixypointseries.html#getyvalues" class="tsd-kind-icon">getYValues</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ixypointseries.html#getyvaluesbyname" class="tsd-kind-icon">getYValuesByName</a>

## Properties

### Readonly arrayCount

arrayCount: number

### Readonly count

count: number

### fifoStartIndex

fifoStartIndex: number

### Readonly indexes

indexes: SCRTDoubleVector

### Readonly resampled

resampled: boolean

### Readonly type

type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/edataseriestype.html" class="tsd-signature-type">EDataSeriesType</a>

### xRange

xRange: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

### Readonly xValues

xValues: SCRTDoubleVector

### Readonly yValues

yValues: SCRTDoubleVector

## Methods

### getYValues

- getYValues(i?: number, undefinedIfMissing?: boolean): SCRTDoubleVector

<!-- -->

- #### Parameters

  - ##### Optional i: number

  - ##### Optional undefinedIfMissing: boolean

  #### Returns SCRTDoubleVector

### getYValuesByName

- getYValuesByName(name: string, undefinedIfMissing?: boolean): SCRTDoubleVector

<!-- -->

- #### Parameters

  - ##### name: string

  - ##### Optional undefinedIfMissing: boolean

  #### Returns SCRTDoubleVector

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
