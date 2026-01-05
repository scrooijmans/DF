<img src="out_scichartv4/typedoc/interfaces/idataseries3d_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idataseries3d.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [IDataSeries3D](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idataseries3d.html)

# Interface IDataSeries3D

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

Defines the interface to a DataSeries in SciChart's High Performance Real-time <a href="https://www.scichart.com/javascript-chart-features" class="external">JavaScript 3D Charts</a>

### Hierarchy

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html" class="tsd-signature-type">IDeletable</a>
  - IDataSeries3D
    - <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/igriddataseries3d.html" class="tsd-signature-type">IGridDataSeries3D</a>

### Implemented by

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basedataseries3d.html" class="tsd-signature-type">BaseDataSeries3D</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basegriddataseries3d.html" class="tsd-signature-type">BaseGridDataSeries3D</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/uniformgriddataseries3d.html" class="tsd-signature-type">UniformGridDataSeries3D</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyzdataseries3d.html" class="tsd-signature-type">XyzDataSeries3D</a>

## Index

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idataseries3d.html#datachanged" class="tsd-kind-icon">dataChanged</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idataseries3d.html#dataseriesname" class="tsd-kind-icon">dataSeriesName</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idataseries3d.html#ismodified" class="tsd-kind-icon">isModified</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idataseries3d.html#type" class="tsd-kind-icon">type</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idataseries3d.html#xrange" class="tsd-kind-icon">xRange</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idataseries3d.html#yrange" class="tsd-kind-icon">yRange</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idataseries3d.html#zrange" class="tsd-kind-icon">zRange</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idataseries3d.html#delete" class="tsd-kind-icon">delete</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idataseries3d.html#resetmodified" class="tsd-kind-icon">resetModified</a>

## Properties

### dataChanged

dataChanged: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/eventhandler.html" class="tsd-signature-type">EventHandler</a>\<void\>

An [EventHandler](https://www.scichart.com/documentation/js/v4/typedoc/classes/eventhandler.html) which is raised when the data changes.

remarks  
To subscribe to dataChanged, use the following code:

``` ts
const dataSeries = new XyDataSeries(wasmContext);
const callback = () => {
   // Data has changed
};
dataSeries.dataChanged.subscribe(callback);
```

To unsubscribe from dataChanged, use the following code:

``` ts
const dataSeries = new XyDataSeries(wasmContext);
dataSeries.dataChanged.unsubscribe(callback);
```

### dataSeriesName

dataSeriesName: string

Gets the DataSeries name. This is used in legend controls and tooltips to identify the series

### Readonly isModified

isModified: boolean

Determines whether the Data Series has been modified since last resetModified() call.

### Readonly type

type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/edataseriestype3d.html" class="tsd-signature-type">EDataSeriesType3D</a>

Gets the [EDataSeriesType3D](https://www.scichart.com/documentation/js/v4/typedoc/enums/edataseriestype3d.html) type of the DataSeries

### Readonly xRange

xRange: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

Gets the total extends of the [DataSeries3D](https://www.scichart.com/documentation/js/v4/typedoc/classes/basedataseries3d.html) in the x-range

### Readonly yRange

yRange: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

Gets the total extends of the [DataSeries3D](https://www.scichart.com/documentation/js/v4/typedoc/classes/basedataseries3d.html) in the y-range

### Readonly zRange

zRange: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

Gets the total extends of the [DataSeries3D](https://www.scichart.com/documentation/js/v4/typedoc/classes/basedataseries3d.html) in the z-range

## Methods

### delete

- delete(): void

<!-- -->

- Deletes native (WebAssembly) memory used by this type, after which it cannot be used.

  remarks  
  Call .delete() before finishing with the object to ensure that WebAssmembly memory leaks do not occur.

  All elements within SciChart's High Performance <a href="https://www.scichart.com/javascript-chart-features" class="external">Realtime JavaScript Charts</a> which implement [IDeletable](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html) must be deleted manually to free native (WebAssembly) memory

  #### Returns void

### resetModified

- resetModified(): void

<!-- -->

- Resets the modified flag.

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
