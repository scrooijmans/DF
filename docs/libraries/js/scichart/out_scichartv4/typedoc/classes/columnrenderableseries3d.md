<img src="out_scichartv4/typedoc/classes/columnrenderableseries3d_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/columnrenderableseries3d.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [ColumnRenderableSeries3D](https://www.scichart.com/documentation/js/v4/typedoc/classes/columnrenderableseries3d.html)

# Class ColumnRenderableSeries3D

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

Defines a 3D column-series in the SciChart's High Performance Real-time <a href="https://www.scichart.com/javascript-chart-features" class="external">JavaScript 3D Charts</a>

remarks  
To add a column 3D series to a [SciChart3DSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html) you need to declare both the [ColumnRenderableSeries3D](https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype3d.html#columnrenderableseries3d) and a [XyzDataSeries3D](https://www.scichart.com/documentation/js/v4/typedoc/classes/xyzdataseries3d.html). Simplified code sample below:

``` ts
const sciChart3DSurface: SciChart3DSurface;
const wasmContext: TSciChart3D;
// Create and fill the dataseries
const dataSeries = new XyzDataSeries3D(wasmContext);
dataSeries.append(1,2,3);
dataSeries.append(3,4,5);
// Create the renderableSeries
const columnSeries = new ColumnRenderableSeries3D(wasmContext);
columnSeries .dataSeries = dataSeries;
columnSeries .pointMarker = new SpherePointMarker3D(wasmContext, {
    size: 3,
    fill: "#FF0000"
});
// append to the SciChartSurface
sciChart3DSurface.renderableSeries.add(columnSeries );
```

### Hierarchy

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries3d.html" class="tsd-signature-type">BaseRenderableSeries3D</a>
  - ColumnRenderableSeries3D

### Implements

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html" class="tsd-signature-type">IDeletable</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries3d.html" class="tsd-signature-type">IRenderableSeries3D</a>

## Index

### Constructors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/columnrenderableseries3d.html#constructor" class="tsd-kind-icon">constructor</a>

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/columnrenderableseries3d.html#id" class="tsd-kind-icon">id</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/columnrenderableseries3d.html#invalidateparentcallback" class="tsd-kind-icon">invalidateParentCallback</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/columnrenderableseries3d.html#isvisiblechanged" class="tsd-kind-icon">isVisibleChanged</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/columnrenderableseries3d.html#pointmarkerproperty" class="tsd-kind-icon">pointMarkerProperty</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/columnrenderableseries3d.html#type" class="tsd-kind-icon">type</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/columnrenderableseries3d.html#webassemblycontext" class="tsd-kind-icon">webAssemblyContext</a>

### Accessors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/columnrenderableseries3d.html#datapointwidthx" class="tsd-kind-icon">dataPointWidthX</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/columnrenderableseries3d.html#datapointwidthz" class="tsd-kind-icon">dataPointWidthZ</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/columnrenderableseries3d.html#dataseries" class="tsd-kind-icon">dataSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/columnrenderableseries3d.html#fill" class="tsd-kind-icon">fill</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/columnrenderableseries3d.html#isvisible" class="tsd-kind-icon">isVisible</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/columnrenderableseries3d.html#opacity" class="tsd-kind-icon">opacity</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/columnrenderableseries3d.html#paletteprovider" class="tsd-kind-icon">paletteProvider</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/columnrenderableseries3d.html#parentsurface" class="tsd-kind-icon">parentSurface</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/columnrenderableseries3d.html#pointmarker" class="tsd-kind-icon">pointMarker</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/columnrenderableseries3d.html#sceneentity" class="tsd-kind-icon">sceneEntity</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/columnrenderableseries3d.html#seriesname" class="tsd-kind-icon">seriesName</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/columnrenderableseries3d.html#shininess" class="tsd-kind-icon">shininess</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/columnrenderableseries3d.html#stroke" class="tsd-kind-icon">stroke</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/columnrenderableseries3d.html#usemetadatacolors" class="tsd-kind-icon">useMetadataColors</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/columnrenderableseries3d.html#applytheme" class="tsd-kind-icon">applyTheme</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/columnrenderableseries3d.html#attachpointmarker" class="tsd-kind-icon">attachPointMarker</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/columnrenderableseries3d.html#dataseriesdatachanged" class="tsd-kind-icon">dataSeriesDataChanged</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/columnrenderableseries3d.html#delete" class="tsd-kind-icon">delete</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/columnrenderableseries3d.html#detachpointmarker" class="tsd-kind-icon">detachPointMarker</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/columnrenderableseries3d.html#enrichhittest" class="tsd-kind-icon">enrichHitTest</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/columnrenderableseries3d.html#getdatapointwidth" class="tsd-kind-icon">getDataPointWidth</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/columnrenderableseries3d.html#getdataseriesname" class="tsd-kind-icon">getDataSeriesName</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/columnrenderableseries3d.html#getoptions" class="tsd-kind-icon">getOptions</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/columnrenderableseries3d.html#hittest" class="tsd-kind-icon">hitTest</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/columnrenderableseries3d.html#notifypropertychanged" class="tsd-kind-icon">notifyPropertyChanged</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/columnrenderableseries3d.html#onattach" class="tsd-kind-icon">onAttach</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/columnrenderableseries3d.html#ondetach" class="tsd-kind-icon">onDetach</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/columnrenderableseries3d.html#ondpichanged" class="tsd-kind-icon">onDpiChanged</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/columnrenderableseries3d.html#setsceneentity" class="tsd-kind-icon">setSceneEntity</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/columnrenderableseries3d.html#tojson" class="tsd-kind-icon">toJSON</a>

## Constructors

### constructor

- new ColumnRenderableSeries3D(webAssemblyContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart3d" class="tsd-signature-type">TSciChart3D</a>, options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icolumnrenderableseries3doptions.html" class="tsd-signature-type">IColumnRenderableSeries3DOptions</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/columnrenderableseries3d.html" class="tsd-signature-type">ColumnRenderableSeries3D</a>

<!-- -->

- Creates an instance of a [ColumnRenderableSeries3D](https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype3d.html#columnrenderableseries3d)

  #### Parameters

  - ##### webAssemblyContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart3d" class="tsd-signature-type">TSciChart3D</a>

    The [SciChart 3D WebAssembly Context](https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart3d) containing native methods and access to our WebGL2 WebAssembly Drawing Engine

  - ##### Optional options: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icolumnrenderableseries3doptions.html" class="tsd-signature-type">IColumnRenderableSeries3DOptions</a>

    Optional parameters of type [IColumnRenderableSeries3DOptions](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icolumnrenderableseries3doptions.html) to configure the series

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/columnrenderableseries3d.html" class="tsd-signature-type">ColumnRenderableSeries3D</a>

## Properties

### Readonly id

id: string

A unique Id for the [IRenderableSeries3D](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries3d.html)

### invalidateParentCallback

invalidateParentCallback: () =\> void

A callback function which notifies the parent [SciChart3DSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html) that data or properties have changed and the 3D Scene needs redrawing

#### Type declaration

- - (): void

  <!-- -->

  - #### Returns void

### isVisibleChanged

isVisibleChanged: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/eventhandler.html" class="tsd-signature-type">EventHandler</a>\<<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/series3dvisiblechangedargs.html" class="tsd-signature-type">Series3DVisibleChangedArgs</a>\> = new EventHandler\<Series3DVisibleChangedArgs\>()

### Protected pointMarkerProperty

pointMarkerProperty: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basemeshpointmarker3d.html" class="tsd-signature-type">BaseMeshPointMarker3D</a>

### Readonly type

type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype3d.html" class="tsd-signature-type">ESeriesType3D</a> = ESeriesType3D.ColumnRenderableSeries3D

Gets the Series type. See [ESeriesType3D](https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype3d.html) for a list of values

### Protected Readonly webAssemblyContext

webAssemblyContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart3d" class="tsd-signature-type">TSciChart3D</a>

The [SciChart 3D WebAssembly Context](https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart3d) containing native methods and access to our WebGL2 WebAssembly Drawing Engine

## Accessors

### dataPointWidthX

- get dataPointWidthX(): number
- set dataPointWidthX(value: number): void

<!-- -->

- Gets or sets the data point width in X direction in Data Space

  #### Returns number

- Gets or sets the data point width in X direction in Data Space

  #### Parameters

  - ##### value: number

  #### Returns void

### dataPointWidthZ

- get dataPointWidthZ(): number
- set dataPointWidthZ(value: number): void

<!-- -->

- Gets or sets the data point width in Z direction in Data Space

  #### Returns number

- Gets or sets the data point width in Z direction in Data Space

  #### Parameters

  - ##### value: number

  #### Returns void

### dataSeries

- get dataSeries(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basedataseries3d.html" class="tsd-signature-type">BaseDataSeries3D</a>
- set dataSeries(dataSeries: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basedataseries3d.html" class="tsd-signature-type">BaseDataSeries3D</a>): void

<!-- -->

- The [DataSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/basedataseries3d.html) which provides a datasource for this [BaseRenderableSeries3D](https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries3d.html) to draw

  inheritdoc  

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basedataseries3d.html" class="tsd-signature-type">BaseDataSeries3D</a>

- The [DataSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/basedataseries3d.html) which provides a datasource for this [BaseRenderableSeries3D](https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries3d.html) to draw

  inheritdoc  

  #### Parameters

  - ##### dataSeries: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basedataseries3d.html" class="tsd-signature-type">BaseDataSeries3D</a>

  #### Returns void

### fill

- get fill(): string
- set fill(fill: string): void

<!-- -->

- Gets or sets the column fill as an HTML Color Code. This will override the pointMarker fill if set.

  #### Returns string

- Gets or sets the column fill as an HTML Color Code. This will override the pointMarker fill if set.

  #### Parameters

  - ##### fill: string

  #### Returns void

### isVisible

- get isVisible(): boolean
- set isVisible(isVisible: boolean): void

<!-- -->

- Gets or sets whether the [BaseDataSeries3D](https://www.scichart.com/documentation/js/v4/typedoc/classes/basedataseries3d.html) is visible or not

  inheritdoc  

  #### Returns boolean

- Gets or sets whether the [BaseDataSeries3D](https://www.scichart.com/documentation/js/v4/typedoc/classes/basedataseries3d.html) is visible or not

  inheritdoc  

  #### Parameters

  - ##### isVisible: boolean

  #### Returns void

### opacity

- get opacity(): number
- set opacity(opacity: number): void

<!-- -->

- Gets or sets an optional Opacity from 0.0 (fully transparent) - 1.0 (fully opaque)

  inheritdoc  

  #### Returns number

- Gets or sets an optional Opacity from 0.0 (fully transparent) - 1.0 (fully opaque)

  inheritdoc  

  #### Parameters

  - ##### opacity: number

  #### Returns void

### paletteProvider

- get paletteProvider(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipaletteprovider3d.html" class="tsd-signature-type">IPaletteProvider3D</a>
- set paletteProvider(paletteProvider: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipaletteprovider3d.html" class="tsd-signature-type">IPaletteProvider3D</a>): void

<!-- -->

- Gets or sets an optional [IPaletteProvider3D](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipaletteprovider3d.html) - a PaletteProvider class which allows for per-point data-point coloring on some 3D [BaseRenderableSeries3D](https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries3d.html) types.

  inheritdoc  

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipaletteprovider3d.html" class="tsd-signature-type">IPaletteProvider3D</a>

- Gets or sets an optional [IPaletteProvider3D](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipaletteprovider3d.html) - a PaletteProvider class which allows for per-point data-point coloring on some 3D [BaseRenderableSeries3D](https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries3d.html) types.

  inheritdoc  

  #### Parameters

  - ##### paletteProvider: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipaletteprovider3d.html" class="tsd-signature-type">IPaletteProvider3D</a>

  #### Returns void

### parentSurface

- get parentSurface(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html" class="tsd-signature-type">SciChart3DSurface</a>
- set parentSurface(parentSurface: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html" class="tsd-signature-type">SciChart3DSurface</a>): void

<!-- -->

- Used internally - Gets or sets the parent [SciChart3DSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html)

  inheritdoc  

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html" class="tsd-signature-type">SciChart3DSurface</a>

- Used internally - Gets or sets the parent [SciChart3DSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html)

  inheritdoc  

  #### Parameters

  - ##### parentSurface: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html" class="tsd-signature-type">SciChart3DSurface</a>

  #### Returns void

### pointMarker

- get pointMarker(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basemeshpointmarker3d.html" class="tsd-signature-type">BaseMeshPointMarker3D</a>
- set pointMarker(pointMarker: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basemeshpointmarker3d.html" class="tsd-signature-type">BaseMeshPointMarker3D</a>): void

<!-- -->

- A [3D Point Marker](https://www.scichart.com/documentation/js/v4/typedoc/classes/basepointmarker3d.html) which is used to draw an optional 3D point-marker at each Xyz data-point. Applicable to some series types only

  inheritdoc  

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basemeshpointmarker3d.html" class="tsd-signature-type">BaseMeshPointMarker3D</a>

- A [3D Point Marker](https://www.scichart.com/documentation/js/v4/typedoc/classes/basepointmarker3d.html) which is used to draw an optional 3D point-marker at each Xyz data-point. Applicable to some series types only

  inheritdoc  

  #### Parameters

  - ##### pointMarker: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basemeshpointmarker3d.html" class="tsd-signature-type">BaseMeshPointMarker3D</a>

  #### Returns void

### sceneEntity

- get sceneEntity(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibasesceneentity.html" class="tsd-signature-type">IBaseSceneEntity</a>

<!-- -->

- Used internally - Gets the [3D Scene Entity](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibasesceneentity.html) which renders the geometry in the 3D Scene

  inheritdoc  

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibasesceneentity.html" class="tsd-signature-type">IBaseSceneEntity</a>

### seriesName

- get seriesName(): string
- set seriesName(value: string): void

<!-- -->

- Gets or sets series name

  inheritdoc  

  #### Returns string

- Gets or sets series name

  #### Parameters

  - ##### value: string

  #### Returns void

### shininess

- get shininess(): number
- set shininess(shininess: number): void

<!-- -->

- Gets or sets an optional Shininess factor, passed to 3D rendering shaders to make shiny objects

  inheritdoc  

  #### Returns number

- Gets or sets an optional Shininess factor, passed to 3D rendering shaders to make shiny objects

  inheritdoc  

  #### Parameters

  - ##### shininess: number

  #### Returns void

### stroke

- get stroke(): string
- set stroke(stroke: string): void

<!-- -->

- Gets or sets the stroke color as an HTML Color code

  inheritdoc  

  #### Returns string

- Gets or sets the stroke color as an HTML Color code

  inheritdoc  

  #### Parameters

  - ##### stroke: string

  #### Returns void

### useMetadataColors

- get useMetadataColors(): boolean
- set useMetadataColors(value: boolean): void

<!-- -->

- Gets or sets flag to do coloring per data-point using metadata vertexColor

  #### Returns boolean

- Gets or sets flag to do coloring per data-point using metadata vertexColor

  #### Parameters

  - ##### value: boolean

  #### Returns void

## Methods

### applyTheme

- applyTheme(themeProvider: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html" class="tsd-signature-type">IThemeProvider</a>): void

<!-- -->

- Applies a theme (defined by [IThemeProvider](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html)) to the current element

  #### Parameters

  - ##### themeProvider: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html" class="tsd-signature-type">IThemeProvider</a>

    The theme data to apply

  #### Returns void

### Protected attachPointMarker

- attachPointMarker(pointMarker: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basepointmarker3d.html" class="tsd-signature-type">BasePointMarker3D</a>): void

<!-- -->

- #### Parameters

  - ##### pointMarker: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basepointmarker3d.html" class="tsd-signature-type">BasePointMarker3D</a>

  #### Returns void

### Protected dataSeriesDataChanged

- dataSeriesDataChanged(): void

<!-- -->

- Is being called when the data for the underlying DataSeries changes

  #### Returns void

### delete

- delete(): void

<!-- -->

- Deletes native (WebAssembly) memory used by this type, after which it cannot be used.

  remarks  
  Call .delete() before finishing with the object to ensure that WebAssmembly memory leaks do not occur.

  All elements within SciChart's High Performance <a href="https://www.scichart.com/javascript-chart-features" class="external">Realtime JavaScript Charts</a> which implement [IDeletable](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html) must be deleted manually to free native (WebAssembly) memory

  #### Returns void

### Protected detachPointMarker

- detachPointMarker(pointMarker: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basepointmarker3d.html" class="tsd-signature-type">BasePointMarker3D</a>): void

<!-- -->

- #### Parameters

  - ##### pointMarker: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basepointmarker3d.html" class="tsd-signature-type">BasePointMarker3D</a>

  #### Returns void

### Protected enrichHitTest

- enrichHitTest(hitTestInfo: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/hittestinfo3d.html" class="tsd-signature-type">HitTestInfo3D</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo3d.html" class="tsd-signature-type">SeriesInfo3D</a>

<!-- -->

- #### Parameters

  - ##### hitTestInfo: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/hittestinfo3d.html" class="tsd-signature-type">HitTestInfo3D</a>

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo3d.html" class="tsd-signature-type">SeriesInfo3D</a>

### getDataPointWidth

- getDataPointWidth(rpd: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassinfo3d.html" class="tsd-signature-type">RenderPassInfo3D</a>, dataPointWidthX: number, dataPointWidthZ: number): number

<!-- -->

- Calculate the dataPointWidth in pixels.

  #### Parameters

  - ##### rpd: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassinfo3d.html" class="tsd-signature-type">RenderPassInfo3D</a>

  - ##### dataPointWidthX: number

  - ##### dataPointWidthZ: number

  #### Returns number

### getDataSeriesName

- getDataSeriesName(): string

<!-- -->

- #### Returns string

### Protected getOptions

- getOptions(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icolumnrenderableseries3doptions.html" class="tsd-signature-type">IColumnRenderableSeries3DOptions</a>

<!-- -->

- #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icolumnrenderableseries3doptions.html" class="tsd-signature-type">IColumnRenderableSeries3DOptions</a>

### hitTest

- hitTest(screenPoint: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/point.html" class="tsd-signature-type">Point</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo3d.html" class="tsd-signature-type">SeriesInfo3D</a>

<!-- -->

- Performs a HitTest operation on the series, returning the [SeriesInfo3D](https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo3d.html) containing the enriched Hit-Test result. This contains information about the hit-test operation such as the values of the data under the mouse and location of the data under the mouse in 3D world coordinates. This may be used for tooltips, selection or inspection of the 3d scene through mouse-clicks

  #### Parameters

  - ##### screenPoint: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/point.html" class="tsd-signature-type">Point</a>

    The screen point (X,Y pixel coordinate in 2D space)

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo3d.html" class="tsd-signature-type">SeriesInfo3D</a>

### Protected notifyPropertyChanged

- notifyPropertyChanged(propertyName: string): void

<!-- -->

- Notifies listeners to [invalidateParentCallback](https://www.scichart.com/documentation/js/v4/typedoc/classes/columnrenderableseries3d.html#invalidateparentcallback) that a property has changed

  #### Parameters

  - ##### propertyName: string

  #### Returns void

### onAttach

- onAttach(scs: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html" class="tsd-signature-type">SciChart3DSurface</a>): void

<!-- -->

- Called when the [IRenderableSeries3D](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries3d.html) is attached to a [SciChart3DSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html)

  #### Parameters

  - ##### scs: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html" class="tsd-signature-type">SciChart3DSurface</a>

    the parent [SciChart3DSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html)

  #### Returns void

### onDetach

- onDetach(): void

<!-- -->

- Called when the [IRenderableSeries3D](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries3d.html) is detached from a [SciChart3DSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html)

  #### Returns void

### onDpiChanged

- onDpiChanged(args: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tdpichangedeventargs" class="tsd-signature-type">TDpiChangedEventArgs</a>): void

<!-- -->

- Called when the Dpi changes in the browser. This could be due to user zooming the browser, or changing DPI settings in Windows, or moving the browser containing SciChart to another monitor

  #### Parameters

  - ##### args: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tdpichangedeventargs" class="tsd-signature-type">TDpiChangedEventArgs</a>

    The [TDpiChangedEventArgs](https://www.scichart.com/documentation/js/v4/typedoc/index.html#tdpichangedeventargs) containing info about the Dpi Changed event

  #### Returns void

### Protected setSceneEntity

- setSceneEntity(sceneEntity: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseriessceneentity.html" class="tsd-signature-type">IRenderableSeriesSceneEntity</a>): void

<!-- -->

- Used internally - sets the [3D Scene Entity](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseriessceneentity.html)

  #### Parameters

  - ##### sceneEntity: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseriessceneentity.html" class="tsd-signature-type">IRenderableSeriesSceneEntity</a>

  #### Returns void

### toJSON

- toJSON(excludeData?: boolean): <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tseriesdefinition3d" class="tsd-signature-type">TSeriesDefinition3D</a>

<!-- -->

- Convert the object to a definition that can be serialized to JSON, or used directly with the builder api

  #### Parameters

  - ##### Default value excludeData: boolean = false

    if set true, data values will not be included in the json.

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tseriesdefinition3d" class="tsd-signature-type">TSeriesDefinition3D</a>

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
