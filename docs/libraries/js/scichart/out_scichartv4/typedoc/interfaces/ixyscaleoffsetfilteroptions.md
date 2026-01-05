<img src="out_scichartv4/typedoc/interfaces/ixyscaleoffsetfilteroptions_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ixyscaleoffsetfilteroptions.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [IXyScaleOffsetFilterOptions](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ixyscaleoffsetfilteroptions.html)

# Interface IXyScaleOffsetFilterOptions

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

Options for the [XyScaleOffsetFilter](https://www.scichart.com/documentation/js/v4/typedoc/classes/xyscaleoffsetfilter.html)

### Hierarchy

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ixyfilteroptions.html" class="tsd-signature-type">IXyFilterOptions</a>
  - IXyScaleOffsetFilterOptions

## Index

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ixyscaleoffsetfilteroptions.html#arraycount" class="tsd-kind-icon">arrayCount</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ixyscaleoffsetfilteroptions.html#capacity" class="tsd-kind-icon">capacity</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ixyscaleoffsetfilteroptions.html#containsnan" class="tsd-kind-icon">containsNaN</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ixyscaleoffsetfilteroptions.html#dataevenlyspacedinx" class="tsd-kind-icon">dataEvenlySpacedInX</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ixyscaleoffsetfilteroptions.html#dataissortedinx" class="tsd-kind-icon">dataIsSortedInX</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ixyscaleoffsetfilteroptions.html#dataseriesname" class="tsd-kind-icon">dataSeriesName</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ixyscaleoffsetfilteroptions.html#field" class="tsd-kind-icon">field</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ixyscaleoffsetfilteroptions.html#fifocapacity" class="tsd-kind-icon">fifoCapacity</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ixyscaleoffsetfilteroptions.html#fifostartindex" class="tsd-kind-icon">fifoStartIndex</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ixyscaleoffsetfilteroptions.html#fifosweeping" class="tsd-kind-icon">fifoSweeping</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ixyscaleoffsetfilteroptions.html#fifosweepinggap" class="tsd-kind-icon">fifoSweepingGap</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ixyscaleoffsetfilteroptions.html#id" class="tsd-kind-icon">id</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ixyscaleoffsetfilteroptions.html#includeinyrange" class="tsd-kind-icon">includeInYRange</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ixyscaleoffsetfilteroptions.html#issorted" class="tsd-kind-icon">isSorted</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ixyscaleoffsetfilteroptions.html#metadata" class="tsd-kind-icon">metadata</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ixyscaleoffsetfilteroptions.html#offset" class="tsd-kind-icon">offset</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ixyscaleoffsetfilteroptions.html#scale" class="tsd-kind-icon">scale</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ixyscaleoffsetfilteroptions.html#valuenames" class="tsd-kind-icon">valueNames</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ixyscaleoffsetfilteroptions.html#xfield" class="tsd-kind-icon">xField</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ixyscaleoffsetfilteroptions.html#xvalues" class="tsd-kind-icon">xValues</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ixyscaleoffsetfilteroptions.html#yvalues" class="tsd-kind-icon">yValues</a>

## Properties

### Optional arrayCount

arrayCount: number

The number of y values arrays. Eg Xy = 1, Xyy = 2 xOHLC = 4

### Optional capacity

capacity: number

Gets or sets the capacity of the data series. This is the amount of memory reserved for the data. For a normal dataSeries this will grow as data is appended. You can avoid memory fragmentation by creating your series with a larger capacity if you know it will grow to that.

### Optional containsNaN

containsNaN: boolean

Gets or sets whether the Y data contains NaN values.

### Optional dataEvenlySpacedInX

dataEvenlySpacedInX: boolean

When true, the Data is evenly spaced in X.

remarks  
The user must specify this flag (defaults to true) in order to choose the correct, and fastest algorithms for drawing, indexing and ranging. If you experience glitches or strange drawing, it may be because you have set data with uneven spacing in X but not set this flag.

### Optional dataIsSortedInX

dataIsSortedInX: boolean

When true, the Data is sorted in X. Same as isSorted.

remarks  
The user must specify this parameter if the data is not sorted in X in order to have correct rendering. This parameter is used to choose the correct algorithms for zooming, panning and ranging and ensure best performance.

### Optional dataSeriesName

dataSeriesName: string

The DataSeries name, used in legends, tooltips to identify the chart series

### Optional field

field: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/edataseriesfield.html" class="tsd-signature-type">EDataSeriesField</a>

The field that will be returned by getOriginalYValues.

### Optional fifoCapacity

fifoCapacity: number

Set the maximum size of the dataSeries in FIFO (First In First Out) mode. This can only be set in the constructor options. If set, the dataSeries does not support insert/insertRange or remove/removeRange. Any data that is appended once the dataSeries has reached fifoCapacity will cause the oldest data to be discarded. This is much more efficient than appending and removing for achieving scrolling data. Spline series and Stacked series currently do not support fifo mode. To get the scrolling effect, you need to consider the behaviour of your X Axis. You can either Use a [CategoryAxis](https://www.scichart.com/documentation/js/v4/typedoc/enums/eaxistype.html#categoryaxis) Use a [NumericAxis](https://www.scichart.com/documentation/js/v4/typedoc/enums/eaxistype.html#numericaxis) with increasing x values, and update the visibleRange (or use zoomExtents)

### Optional fifoStartIndex

fifoStartIndex: number

Sets the starting index of data for fifo mode.

### Optional fifoSweeping

fifoSweeping: boolean

If true, data in fifo mode will not be "unwrapped" before drawing, giving ecg style sweeping mode. To get the sweeping effect, you need to consider the behaviour of your X Axis. You can either Use a [CategoryAxis](https://www.scichart.com/documentation/js/v4/typedoc/enums/eaxistype.html#categoryaxis) Use a [NumericAxis](https://www.scichart.com/documentation/js/v4/typedoc/enums/eaxistype.html#numericaxis) and make your x values an offset from the first value, eg by doing x % fifoCapcity

### Optional fifoSweepingGap

fifoSweepingGap: number

In fifo sweeping mode, the number of earliest points to skip to create a gap between the latest and earliest data

### Optional id

id: string

A unique Id for the [IDataSeries](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idataseries.html)

### Optional includeInYRange

includeInYRange: boolean\[\]

Whether each set of Y values should be included when calculating y Range

### Optional isSorted

isSorted: boolean

When true, the Data is sorted in X. Same as dataIsSortedInX.

remarks  
The user must specify this parameter if the data is not sorted in X in order to have correct rendering. This parameter is used to choose the correct algorithms for zooming, panning and ranging and ensure best performance.

### Optional metadata

metadata: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmetadata.html" class="tsd-signature-type">IPointMetadata</a>\[\] \| <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmetadata.html" class="tsd-signature-type">IPointMetadata</a> \| { data?: any; type: string }

The Metadata values of type [IPointMetadata](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmetadata.html) to pre-populate the [IDataSeries](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idataseries.html) If a single metadata value is supplied, this will be used as a template for all data values. If type is specified, it should refer to a registered metadataGenerator [IMetadataGenerator](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/imetadatagenerator.html), which can provide all metadata, based on the data provided, or a single object that will be used when adding data if no metadata is provided.

### Optional offset

offset: number

### Optional scale

scale: number

### Optional valueNames

valueNames: string\[\]

The names for the y values arrays. Defaults to y, y1, y2 etc

### Optional xField

xField: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/edataseriesfield.html" class="tsd-signature-type">EDataSeriesField</a>

The field that will be returned by getOriginalXValues.

### Optional xValues

xValues: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#numberarray" class="tsd-signature-type">NumberArray</a>

The X-values array to pre-populate the [XyDataSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html)

### Optional yValues

yValues: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#numberarray" class="tsd-signature-type">NumberArray</a>

The Y-values array to pre-populate the [XyDataSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html)

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
