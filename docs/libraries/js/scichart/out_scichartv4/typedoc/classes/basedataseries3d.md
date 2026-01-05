<img src="out_scichartv4/typedoc/classes/basedataseries3d_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basedataseries3d.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [BaseDataSeries3D](https://www.scichart.com/documentation/js/v4/typedoc/classes/basedataseries3d.html)

# Class BaseDataSeries3D

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

The base class for DataSeries in SciChart's <a href="https://www.scichart.com/javascript-chart-features" class="external">JavaScript 3D Charts</a>

remarks  
A DataSeries stores the data to render. This is independent from the [RenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries3d.html) which defines how that data should be rendered.

See derived types of [BaseDataSeries3D](https://www.scichart.com/documentation/js/v4/typedoc/classes/basedataseries3d.html) to find out what data-series are available. See derived types of [IRenderableSeries3D](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries3d.html) to find out what 3D JavaScript Chart types are available.

### Hierarchy

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/deletableentity.html" class="tsd-signature-type">DeletableEntity</a>
  - BaseDataSeries3D
    - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyzdataseries3d.html" class="tsd-signature-type">XyzDataSeries3D</a>
    - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basegriddataseries3d.html" class="tsd-signature-type">BaseGridDataSeries3D</a>

### Implements

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html" class="tsd-signature-type">IDeletable</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idataseries3d.html" class="tsd-signature-type">IDataSeries3D</a>

## Index

### Constructors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basedataseries3d.html#constructor" class="tsd-kind-icon">constructor</a>

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basedataseries3d.html#datachanged" class="tsd-kind-icon">dataChanged</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basedataseries3d.html#type" class="tsd-kind-icon">type</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basedataseries3d.html#webassemblycontext" class="tsd-kind-icon">webAssemblyContext</a>

### Accessors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basedataseries3d.html#dataseriesname" class="tsd-kind-icon">dataSeriesName</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basedataseries3d.html#ismodified" class="tsd-kind-icon">isModified</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basedataseries3d.html#xrange" class="tsd-kind-icon">xRange</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basedataseries3d.html#yrange" class="tsd-kind-icon">yRange</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basedataseries3d.html#zrange" class="tsd-kind-icon">zRange</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basedataseries3d.html#delete" class="tsd-kind-icon">delete</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basedataseries3d.html#getisdeleted" class="tsd-kind-icon">getIsDeleted</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basedataseries3d.html#getoptions" class="tsd-kind-icon">getOptions</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basedataseries3d.html#notifydatachanged" class="tsd-kind-icon">notifyDataChanged</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basedataseries3d.html#resetmodified" class="tsd-kind-icon">resetModified</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basedataseries3d.html#tojson" class="tsd-kind-icon">toJSON</a>

## Constructors

### constructor

- new BaseDataSeries3D(webAssemblyContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart3d" class="tsd-signature-type">TSciChart3D</a>, options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibasedataseries3doptions.html" class="tsd-signature-type">IBaseDataSeries3DOptions</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basedataseries3d.html" class="tsd-signature-type">BaseDataSeries3D</a>

<!-- -->

- Creates an instance of the [BaseDataSeries3D](https://www.scichart.com/documentation/js/v4/typedoc/classes/basedataseries3d.html)

  #### Parameters

  - ##### webAssemblyContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart3d" class="tsd-signature-type">TSciChart3D</a>

    the [SciChart WebAssembly Context](https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart3d) containing native methods and access to our underlying WebGL2 rendering engine

  - ##### Optional options: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibasedataseries3doptions.html" class="tsd-signature-type">IBaseDataSeries3DOptions</a>

    optional parameters of type [IBaseDataSeries3DOptions](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibasedataseries3doptions.html) to configure the series

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basedataseries3d.html" class="tsd-signature-type">BaseDataSeries3D</a>

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

### yRange

- get yRange(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

<!-- -->

- Gets the total extends of the [DataSeries3D](https://www.scichart.com/documentation/js/v4/typedoc/classes/basedataseries3d.html) in the y-range

  inheritdoc  

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

### zRange

- get zRange(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

<!-- -->

- Gets the total extends of the [DataSeries3D](https://www.scichart.com/documentation/js/v4/typedoc/classes/basedataseries3d.html) in the z-range

  inheritdoc  

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

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

- getOptions(excludeData: boolean): <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibasedataseries3doptions.html" class="tsd-signature-type">IBaseDataSeries3DOptions</a>

<!-- -->

- #### Parameters

  - ##### excludeData: boolean

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibasedataseries3doptions.html" class="tsd-signature-type">IBaseDataSeries3DOptions</a>

### notifyDataChanged

- notifyDataChanged(): void

<!-- -->

- Call this method to notify subscribers of [dataChanged](https://www.scichart.com/documentation/js/v4/typedoc/classes/basedataseries3d.html#datachanged) that the data has changed and <a href="https://www.scichart.com/javascript-chart-features" class="external">3D JavaScript Chart</a> needs redrawing

  #### Returns void

### resetModified

- resetModified(): void

<!-- -->

- Resets the modified flag.

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
