<img src="out_scichartv4/typedoc/classes/uniformgriddataseries3d_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/uniformgriddataseries3d.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [UniformGridDataSeries3D](https://www.scichart.com/documentation/js/v4/typedoc/classes/uniformgriddataseries3d.html)

# Class UniformGridDataSeries3D

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

summary  
The [UniformGridDataSeries3D](https://www.scichart.com/documentation/js/v4/typedoc/classes/uniformgriddataseries3d.html) wraps a 2D array of numbers which become the Y-values (heights) in various [BaseRenderableSeries3D](https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries3d.html) in SciChart's High Performance <a href="https://www.scichart.com/javascript-chart-features" class="external">JavaScript 3D Charts</a>.

description  
The [SurfaceMeshRenderableSeries3D](https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype3d.html#surfacemeshrenderableseries3d) requires a 2D array of numbers to map to Y-values (heights).

The [xStart](https://www.scichart.com/documentation/js/v4/typedoc/classes/uniformgriddataseries3d.html#xstart), [xStep](https://www.scichart.com/documentation/js/v4/typedoc/classes/uniformgriddataseries3d.html#xstep) properties define the extents of the data in the X-direction, and [yStart](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iuniformheatmapseriesoptions.html#ystart), [yStep](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iuniformheatmapseriesoptions.html#ystep) define the extents of the data in the Y-direction.

Y-values may be updated via manipulating the array returned by [getYValues](https://www.scichart.com/documentation/js/v4/typedoc/classes/uniformgriddataseries3d.html#getyvalues), or by setting a new array to [setYValues](https://www.scichart.com/documentation/js/v4/typedoc/classes/uniformgriddataseries3d.html#setyvalues). When manpulating data directly, be sure to call [notifyDataChanged](https://www.scichart.com/documentation/js/v4/typedoc/classes/uniformgriddataseries3d.html#notifydatachanged) to inform SciChart to redraw.

### Hierarchy

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basegriddataseries3d.html" class="tsd-signature-type">BaseGridDataSeries3D</a>
  - UniformGridDataSeries3D

### Implements

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html" class="tsd-signature-type">IDeletable</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idataseries3d.html" class="tsd-signature-type">IDataSeries3D</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/igriddataseries3d.html" class="tsd-signature-type">IGridDataSeries3D</a>

## Index

### Constructors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/uniformgriddataseries3d.html#constructor" class="tsd-kind-icon">constructor</a>

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/uniformgriddataseries3d.html#datachanged" class="tsd-kind-icon">dataChanged</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/uniformgriddataseries3d.html#type" class="tsd-kind-icon">type</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/uniformgriddataseries3d.html#webassemblycontext" class="tsd-kind-icon">webAssemblyContext</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/uniformgriddataseries3d.html#yrangecached" class="tsd-kind-icon">yRangeCached</a>

### Accessors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/uniformgriddataseries3d.html#dataseriesname" class="tsd-kind-icon">dataSeriesName</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/uniformgriddataseries3d.html#ismodified" class="tsd-kind-icon">isModified</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/uniformgriddataseries3d.html#xrange" class="tsd-kind-icon">xRange</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/uniformgriddataseries3d.html#xsize" class="tsd-kind-icon">xSize</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/uniformgriddataseries3d.html#xstart" class="tsd-kind-icon">xStart</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/uniformgriddataseries3d.html#xstep" class="tsd-kind-icon">xStep</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/uniformgriddataseries3d.html#yrange" class="tsd-kind-icon">yRange</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/uniformgriddataseries3d.html#zrange" class="tsd-kind-icon">zRange</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/uniformgriddataseries3d.html#zsize" class="tsd-kind-icon">zSize</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/uniformgriddataseries3d.html#zstart" class="tsd-kind-icon">zStart</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/uniformgriddataseries3d.html#zstep" class="tsd-kind-icon">zStep</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/uniformgriddataseries3d.html#delete" class="tsd-kind-icon">delete</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/uniformgriddataseries3d.html#getisdeleted" class="tsd-kind-icon">getIsDeleted</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/uniformgriddataseries3d.html#getoptions" class="tsd-kind-icon">getOptions</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/uniformgriddataseries3d.html#getx" class="tsd-kind-icon">getX</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/uniformgriddataseries3d.html#getyvalue" class="tsd-kind-icon">getYValue</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/uniformgriddataseries3d.html#getyvalues" class="tsd-kind-icon">getYValues</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/uniformgriddataseries3d.html#getz" class="tsd-kind-icon">getZ</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/uniformgriddataseries3d.html#notifydatachanged" class="tsd-kind-icon">notifyDataChanged</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/uniformgriddataseries3d.html#resetmodified" class="tsd-kind-icon">resetModified</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/uniformgriddataseries3d.html#setyvalue" class="tsd-kind-icon">setYValue</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/uniformgriddataseries3d.html#setyvalues" class="tsd-kind-icon">setYValues</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/uniformgriddataseries3d.html#tojson" class="tsd-kind-icon">toJSON</a>

## Constructors

### constructor

- new UniformGridDataSeries3D(webAssemblyContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart3d" class="tsd-signature-type">TSciChart3D</a>, options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iuniformgriddataseries3doptions.html" class="tsd-signature-type">IUniformGridDataSeries3DOptions</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/uniformgriddataseries3d.html" class="tsd-signature-type">UniformGridDataSeries3D</a>

<!-- -->

- Creates an instance of a [UniformGridDataSeries3D](https://www.scichart.com/documentation/js/v4/typedoc/classes/uniformgriddataseries3d.html)

  #### Parameters

  - ##### webAssemblyContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart3d" class="tsd-signature-type">TSciChart3D</a>

    the [SciChart WebAssembly Context](https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart3d) containing native methods and access to our underlying WebGL2 rendering engine

  - ##### Optional options: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iuniformgriddataseries3doptions.html" class="tsd-signature-type">IUniformGridDataSeries3DOptions</a>

    optional parameters of type [IUniformGridDataSeries3DOptions](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iuniformgriddataseries3doptions.html) to configure the series

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/uniformgriddataseries3d.html" class="tsd-signature-type">UniformGridDataSeries3D</a>

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

### type

type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/edataseriestype3d.html" class="tsd-signature-type">EDataSeriesType3D</a> = EDataSeriesType3D.UniformGrid3D

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

- Gets the total extends of the [DataSeries3D](https://www.scichart.com/documentation/js/v4/typedoc/classes/basedataseries3d.html) in the x-range

  inheritdoc  

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

### xSize

- get xSize(): number

<!-- -->

- The xSize is the WIDTH or number of elements in each or of the 2-dimensional array, e.g. \[\[1, 2\]\[3, 4\]\[5, 6\]\] has a xSize of 2

  #### Returns number

### xStart

- get xStart(): number
- set xStart(xStart: number): void

<!-- -->

- xStart defines the Start point on the [XAxis](https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase3d.html) where this grid or mesh will be drawn

  #### Returns number

- xStart defines the Start point on the [XAxis](https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase3d.html) where this grid or mesh will be drawn

  #### Parameters

  - ##### xStart: number

  #### Returns void

### xStep

- get xStep(): number
- set xStep(xStep: number): void

<!-- -->

- xStep defines Step on the [XAxis](https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase3d.html) for each cell in the grid or mesh

  #### Returns number

- xStep defines Step on the [XAxis](https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase3d.html) for each cell in the grid or mesh

  #### Parameters

  - ##### xStep: number

  #### Returns void

### yRange

- get yRange(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

<!-- -->

- Gets the total extends of the GridDataSeries3D in the y-range

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

### zRange

- get zRange(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

<!-- -->

- Gets the total extends of the [DataSeries3D](https://www.scichart.com/documentation/js/v4/typedoc/classes/basedataseries3d.html) in the z-range

  inheritdoc  

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

### zSize

- get zSize(): number

<!-- -->

- The zSize is the HEIGHT or number of rows of the 2-dimensional array, e.g. \[\[1, 2\]\[3, 4\]\[5, 6\]\] has a height of 3

  #### Returns number

### zStart

- get zStart(): number
- set zStart(zStart: number): void

<!-- -->

- zStart defines the Start point on the [ZAxis](https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase3d.html) where this grid or mesh will be drawn

  #### Returns number

- zStart defines the Start point on the [ZAxis](https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase3d.html) where this grid or mesh will be drawn

  #### Parameters

  - ##### zStart: number

  #### Returns void

### zStep

- get zStep(): number
- set zStep(zStep: number): void

<!-- -->

- zStep defines Step on the [ZAxis](https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase3d.html) for each cell in the grid or mesh

  #### Returns number

- zStep defines Step on the [ZAxis](https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase3d.html) for each cell in the grid or mesh

  #### Parameters

  - ##### zStep: number

  #### Returns void

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

- getOptions(excludeData: boolean): <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iuniformgriddataseries3doptions.html" class="tsd-signature-type">IUniformGridDataSeries3DOptions</a>

<!-- -->

- #### Parameters

  - ##### excludeData: boolean

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iuniformgriddataseries3doptions.html" class="tsd-signature-type">IUniformGridDataSeries3DOptions</a>

### getX

- getX(xIndex: number): number

<!-- -->

- inheritdoc  

  #### Parameters

  - ##### xIndex: number

  #### Returns number

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

### getZ

- getZ(zIndex: number): number

<!-- -->

- inheritdoc  

  #### Parameters

  - ##### zIndex: number

  #### Returns number

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
