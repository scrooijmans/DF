<img src="out_scichartv4/typedoc/classes/basegriddataseries3d_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basegriddataseries3d.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [BaseGridDataSeries3D](https://www.scichart.com/documentation/js/v4/typedoc/classes/basegriddataseries3d.html)

# Class BaseGridDataSeries3D

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

summary  
The base class for a Grid (two-dimensional array) DataSeries in SciChart's <a href="https://www.scichart.com/javascript-chart-features" class="external">JavaScript 3D Charts</a>

description  
A DataSeries stores the data to render. This is independent from the [RenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries3d.html) which defines how that data should be rendered.

The Grid DataSeries type is used for 3D Charts that have a uniform grid of values, for example [SurfaceMeshRenderableSeries3D](https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype3d.html#surfacemeshrenderableseries3d), which draws a 3D surface-plot or mesh chart with contours.

remarks  
See derived type [UniformGridDataSeries3D](https://www.scichart.com/documentation/js/v4/typedoc/classes/uniformgriddataseries3d.html) for a concrete implementation

### Hierarchy

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basedataseries3d.html" class="tsd-signature-type">BaseDataSeries3D</a>
  - BaseGridDataSeries3D
    - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/uniformgriddataseries3d.html" class="tsd-signature-type">UniformGridDataSeries3D</a>

### Implements

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html" class="tsd-signature-type">IDeletable</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idataseries3d.html" class="tsd-signature-type">IDataSeries3D</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/igriddataseries3d.html" class="tsd-signature-type">IGridDataSeries3D</a>

## Index

### Constructors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basegriddataseries3d.html#constructor" class="tsd-kind-icon">constructor</a>

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basegriddataseries3d.html#datachanged" class="tsd-kind-icon">dataChanged</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basegriddataseries3d.html#type" class="tsd-kind-icon">type</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basegriddataseries3d.html#webassemblycontext" class="tsd-kind-icon">webAssemblyContext</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basegriddataseries3d.html#yrangecached" class="tsd-kind-icon">yRangeCached</a>

### Accessors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basegriddataseries3d.html#dataseriesname" class="tsd-kind-icon">dataSeriesName</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basegriddataseries3d.html#ismodified" class="tsd-kind-icon">isModified</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basegriddataseries3d.html#xrange" class="tsd-kind-icon">xRange</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basegriddataseries3d.html#xsize" class="tsd-kind-icon">xSize</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basegriddataseries3d.html#yrange" class="tsd-kind-icon">yRange</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basegriddataseries3d.html#zrange" class="tsd-kind-icon">zRange</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basegriddataseries3d.html#zsize" class="tsd-kind-icon">zSize</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basegriddataseries3d.html#delete" class="tsd-kind-icon">delete</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basegriddataseries3d.html#getisdeleted" class="tsd-kind-icon">getIsDeleted</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basegriddataseries3d.html#getoptions" class="tsd-kind-icon">getOptions</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basegriddataseries3d.html#getyvalue" class="tsd-kind-icon">getYValue</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basegriddataseries3d.html#getyvalues" class="tsd-kind-icon">getYValues</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basegriddataseries3d.html#notifydatachanged" class="tsd-kind-icon">notifyDataChanged</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basegriddataseries3d.html#resetmodified" class="tsd-kind-icon">resetModified</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basegriddataseries3d.html#setyvalue" class="tsd-kind-icon">setYValue</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basegriddataseries3d.html#setyvalues" class="tsd-kind-icon">setYValues</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basegriddataseries3d.html#tojson" class="tsd-kind-icon">toJSON</a>

## Constructors

### Protected constructor

- new BaseGridDataSeries3D(webAssemblyContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart3d" class="tsd-signature-type">TSciChart3D</a>, options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibasegriddataseries3doptions.html" class="tsd-signature-type">IBaseGridDataSeries3DOptions</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basegriddataseries3d.html" class="tsd-signature-type">BaseGridDataSeries3D</a>

<!-- -->

- Creates an instance of the [BaseGridDataSeries3D](https://www.scichart.com/documentation/js/v4/typedoc/classes/basegriddataseries3d.html)

  #### Parameters

  - ##### webAssemblyContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart3d" class="tsd-signature-type">TSciChart3D</a>

    the [SciChart WebAssembly Context](https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart3d) containing native methods and access to our underlying WebGL2 rendering engine

  - ##### Optional options: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibasegriddataseries3doptions.html" class="tsd-signature-type">IBaseGridDataSeries3DOptions</a>

    optional parameters of type [IBaseGridDataSeries3DOptions](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibasegriddataseries3doptions.html) to configure the series

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basegriddataseries3d.html" class="tsd-signature-type">BaseGridDataSeries3D</a>

## Properties

### Readonly dataChanged

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

### Readonly Abstract type

type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/edataseriestype3d.html" class="tsd-signature-type">EDataSeriesType3D</a>

Gets the [EDataSeriesType3D](https://www.scichart.com/documentation/js/v4/typedoc/enums/edataseriestype3d.html) type of the DataSeries

### Protected webAssemblyContext

webAssemblyContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart3d" class="tsd-signature-type">TSciChart3D</a>

The [SciChart WebAssembly Context](https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart3d) containing native methods and access to our WebGL2 Engine

### Protected yRangeCached

yRangeCached: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

## Accessors

### dataSeriesName

- get dataSeriesName(): string
- set dataSeriesName(dataSeriesName: string): void

<!-- -->

- Gets the DataSeries name. This is used in legend controls and tooltips to identify the series

  inheritdoc  

  #### Returns string

- Gets the DataSeries name. This is used in legend controls and tooltips to identify the series

  inheritdoc  

  #### Parameters

  - ##### dataSeriesName: string

  #### Returns void

### isModified

- get isModified(): boolean

<!-- -->

- Determines whether the Data Series has been modified since last resetModified() call.

  #### Returns boolean

### xRange

- get xRange(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

<!-- -->

- Gets the total extends of the GridDataSeries3D in the x-range

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

### xSize

- get xSize(): number

<!-- -->

- The xSize is the WIDTH or number of elements in each or of the 2-dimensional array, e.g. \[\[1, 2\]\[3, 4\]\[5, 6\]\] has a xSize of 2

  #### Returns number

### yRange

- get yRange(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

<!-- -->

- Gets the total extends of the GridDataSeries3D in the y-range

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

### zRange

- get zRange(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

<!-- -->

- Gets the total extends of the GridDataSeries3D in the z-range

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

### zSize

- get zSize(): number

<!-- -->

- The zSize is the HEIGHT or number of rows of the 2-dimensional array, e.g. \[\[1, 2\]\[3, 4\]\[5, 6\]\] has a height of 3

  #### Returns number

## Methods

### delete

- delete(): void

<!-- -->

- Deletes native (WebAssembly) memory used by this type, after which it cannot be used.

  remarks  
  Call .delete() before finishing with the object to ensure that WebAssmembly memory leaks do not occur.

  All elements within SciChart's High Performance <a href="https://www.scichart.com/javascript-chart-features" class="external">Realtime JavaScript Charts</a> which implement [IDeletable](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html) must be deleted manually to free native (WebAssembly) memory

  #### Returns void

### getIsDeleted

- getIsDeleted(): boolean

<!-- -->

- Returns true if this DataSeries has been deleted and native memory destroyed

  #### Returns boolean

### Protected getOptions

- getOptions(excludeData: boolean): <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibasegriddataseries3doptions.html" class="tsd-signature-type">IBaseGridDataSeries3DOptions</a>

<!-- -->

- #### Parameters

  - ##### excludeData: boolean

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibasegriddataseries3doptions.html" class="tsd-signature-type">IBaseGridDataSeries3DOptions</a>

### getYValue

- getYValue(zIndex: number, xIndex: number): number

<!-- -->

- Gets the YValue at the specific Z,X index where Z must be within 0-zSize and X must be within 0-xSize

  #### Parameters

  - ##### zIndex: number

    the z-index from 0 to zSize

  - ##### xIndex: number

    the x-index from 0 to xSize

  #### Returns number

### getYValues

- getYValues(): number\[\]\[\]

<!-- -->

- Gets the Y-Values array as a two dimensional array. Output is in the format YValues\[z\]\[x\] where z is 0 to zSize and X is 0 to xSize.

  #### Returns number\[\]\[\]

### notifyDataChanged

- notifyDataChanged(): void

<!-- -->

- inheritdoc  

  #### Returns void

### resetModified

- resetModified(): void

<!-- -->

- Resets the modified flag.

  #### Returns void

### setYValue

- setYValue(zIndex: number, xIndex: number, yValue: number): void

<!-- -->

- Sets the YValue at the specific Z,X index where Z must be within 0-zSize and X must be within 0-xSize

  #### Parameters

  - ##### zIndex: number

    the z-index from 0 to zSize

  - ##### xIndex: number

    the x-index from 0 to xSize

  - ##### yValue: number

  #### Returns void

### setYValues

- setYValues(YValues: number\[\]\[\]): void

<!-- -->

- Sets a 2D array of YValues. Input is in the format YValues\[z\]\[x\] where z is 0 to zSize and X is 0 to xSize Note that setting the YValues involves a clone. Once the array has been set you cannot manipulate the input array and expect changes on the chart.

  #### Parameters

  - ##### YValues: number\[\]\[\]

  #### Returns void

### toJSON

- toJSON(excludeData?: boolean): <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tdataseriesdefinition3d" class="tsd-signature-type">TDataSeriesDefinition3D</a>

<!-- -->

- inheritdoc  

  #### Parameters

  - ##### Default value excludeData: boolean = false

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tdataseriesdefinition3d" class="tsd-signature-type">TDataSeriesDefinition3D</a>

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
