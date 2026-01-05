<img src="out_scichartv4/typedoc/classes/surfacemeshrenderableseries3d_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [SurfaceMeshRenderableSeries3D](https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html)

# Class SurfaceMeshRenderableSeries3D

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

summary  
A JavaScript 3D Surface Mesh or Surface Plot chart type rendering a 2-dimensional array of data as color and height values SciChart's High Performance Real-time <a href="https://www.scichart.com/javascript-chart-features" class="external">JavaScript 3D Chart Library</a>

description  
Surface meshes are a 3D representation of 2-dimensional arrays of data, rendered as a height-map with optional colors on the chart. The [SurfaceMeshRenderableSeries3D](https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype3d.html#surfacemeshrenderableseries3d) assumes the cells are equal size, and spaced along the X,Z axis according to properties on the [UniformGridDataSeries3D](https://www.scichart.com/documentation/js/v4/typedoc/classes/uniformgriddataseries3d.html).

For a code sample how to initialize a surface mesh, see below

``` ts
// Create an empty 2D array using the helper function zeroArray2D
const heightMapArray: number[][] = zeroArray2D([height, width]);
// todo: fill the zValues 2d array with values

// Create a UniformGridDataSeries3D passing in height values
const dataSeries = new UniformGridDataSeries3D(wasmContext, {
      yValues: heightmapArray,
      xStep: 1,
      zStep: 1,
      dataSeriesName: "Uniform Surface Mesh"
  });

// Create a color map with gradient colors
const colorMap = new GradientColorPalette(wasmContext, {
       gradientStops: [
           { offset: 1, color: "#8B0000" },
           { offset: 0.9, color: "#FF0000" },
           { offset: 0.7, color: "#FF0000" },
           { offset: 0.5, color: "#ADFF2F" },
           { offset: 0.3, color: "#00FFFF" },
           { offset: 0.1, color: "#0000FF" },
           { offset: 0, color: "#1D2C6B" }
       ]
   });

// Create a SurfaceMeshRenderableSeries3D
const series = new SurfaceMeshRenderableSeries3D(wasmContext, {
       dataSeries,
       minimum: 0,
       maximum: 0.5,
       opacity: 0.9,
       cellHardnessFactor: 1.0,
       shininess: 0,
       lightingFactor: 0.8,
       highlight: 1.0,
       stroke: "rgba(24,139,34,0.5)",
       strokeThickness: 2.0,
       contourStroke: "rgba(24,139,34,0.5)",
       contourInterval: 2,
       contourOffset: 0,
       contourStrokeThickness: 2,
       drawSkirt: false,
       drawMeshAs: EDrawMeshAs.SOLID_WIREFRAME,
       meshColorPalette: colorMap,
       isVisible: true
   });

// Add the Surface Mesh to the chart
sciChart3DSurface.renderableSeries.add(series);
```

### Hierarchy

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries3d.html" class="tsd-signature-type">BaseRenderableSeries3D</a>
  - SurfaceMeshRenderableSeries3D

### Implements

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html" class="tsd-signature-type">IDeletable</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries3d.html" class="tsd-signature-type">IRenderableSeries3D</a>

## Index

### Constructors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#constructor" class="tsd-kind-icon">constructor</a>

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#id" class="tsd-kind-icon">id</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#invalidateparentcallback" class="tsd-kind-icon">invalidateParentCallback</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#isvisiblechanged" class="tsd-kind-icon">isVisibleChanged</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#pointmarkerproperty" class="tsd-kind-icon">pointMarkerProperty</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#webassemblycontext" class="tsd-kind-icon">webAssemblyContext</a>

### Accessors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#cellhardnessfactor" class="tsd-kind-icon">cellHardnessFactor</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#colormaptexturesize" class="tsd-kind-icon">colorMapTextureSize</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#contourinterval" class="tsd-kind-icon">contourInterval</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#contouroffset" class="tsd-kind-icon">contourOffset</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#contourstroke" class="tsd-kind-icon">contourStroke</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#contourstrokethickness" class="tsd-kind-icon">contourStrokeThickness</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#dataseries" class="tsd-kind-icon">dataSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#drawmeshas" class="tsd-kind-icon">drawMeshAs</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#drawskirt" class="tsd-kind-icon">drawSkirt</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#heightscalefactor" class="tsd-kind-icon">heightScaleFactor</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#highlight" class="tsd-kind-icon">highlight</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#isvisible" class="tsd-kind-icon">isVisible</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#lightingfactor" class="tsd-kind-icon">lightingFactor</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#maximum" class="tsd-kind-icon">maximum</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#meshcolorpalette" class="tsd-kind-icon">meshColorPalette</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#meshpalettemode" class="tsd-kind-icon">meshPaletteMode</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#meshresolution" class="tsd-kind-icon">meshResolution</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#minimum" class="tsd-kind-icon">minimum</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#opacity" class="tsd-kind-icon">opacity</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#paletteprovider" class="tsd-kind-icon">paletteProvider</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#parentsurface" class="tsd-kind-icon">parentSurface</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#pointmarker" class="tsd-kind-icon">pointMarker</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#sceneentity" class="tsd-kind-icon">sceneEntity</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#seriesname" class="tsd-kind-icon">seriesName</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#shininess" class="tsd-kind-icon">shininess</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#stroke" class="tsd-kind-icon">stroke</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#strokethickness" class="tsd-kind-icon">strokeThickness</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#type" class="tsd-kind-icon">type</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#yoffset" class="tsd-kind-icon">yOffset</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#applytheme" class="tsd-kind-icon">applyTheme</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#attachpointmarker" class="tsd-kind-icon">attachPointMarker</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#dataseriesdatachanged" class="tsd-kind-icon">dataSeriesDataChanged</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#delete" class="tsd-kind-icon">delete</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#detachpointmarker" class="tsd-kind-icon">detachPointMarker</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#enrichhittest" class="tsd-kind-icon">enrichHitTest</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#getdataseriesname" class="tsd-kind-icon">getDataSeriesName</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#getoptions" class="tsd-kind-icon">getOptions</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#hittest" class="tsd-kind-icon">hitTest</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#notifypropertychanged" class="tsd-kind-icon">notifyPropertyChanged</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#onattach" class="tsd-kind-icon">onAttach</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#ondetach" class="tsd-kind-icon">onDetach</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#ondpichanged" class="tsd-kind-icon">onDpiChanged</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#setsceneentity" class="tsd-kind-icon">setSceneEntity</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#tojson" class="tsd-kind-icon">toJSON</a>

## Constructors

### constructor

- new SurfaceMeshRenderableSeries3D(webAssemblyContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart3d" class="tsd-signature-type">TSciChart3D</a>, options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/isurfacemeshrenderableseries3doptions.html" class="tsd-signature-type">ISurfaceMeshRenderableSeries3DOptions</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html" class="tsd-signature-type">SurfaceMeshRenderableSeries3D</a>

<!-- -->

- Creates an instance of a [SurfaceMeshRenderableSeries3D](https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype3d.html#surfacemeshrenderableseries3d)

  #### Parameters

  - ##### webAssemblyContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart3d" class="tsd-signature-type">TSciChart3D</a>

    The [SciChart 3D WebAssembly Context](https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart3d) containing native methods and access to our WebGL2 Engine and WebAssembly numerical methods

  - ##### Optional options: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/isurfacemeshrenderableseries3doptions.html" class="tsd-signature-type">ISurfaceMeshRenderableSeries3DOptions</a>

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html" class="tsd-signature-type">SurfaceMeshRenderableSeries3D</a>

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

pointMarkerProperty: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basepointmarker3d.html" class="tsd-signature-type">BasePointMarker3D</a>

### Protected Readonly webAssemblyContext

webAssemblyContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart3d" class="tsd-signature-type">TSciChart3D</a>

The [SciChart 3D WebAssembly Context](https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart3d) containing native methods and access to our WebGL2 WebAssembly Drawing Engine

## Accessors

### cellHardnessFactor

- get cellHardnessFactor(): number
- set cellHardnessFactor(cellHardnessFactor: number): void

<!-- -->

- Gets or sets the cell hardness factor, a number used to calculate lighting effects.

  remarks  
  Defaults to 1

  #### Returns number

- Gets or sets the cell hardness factor, a number used to calculate lighting effects.

  remarks  
  Defaults to 1

  #### Parameters

  - ##### cellHardnessFactor: number

  #### Returns void

### colorMapTextureSize

- get colorMapTextureSize(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/size.html" class="tsd-signature-type">Size</a>
- set colorMapTextureSize(colorMapTextureSize: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/size.html" class="tsd-signature-type">Size</a>): void

<!-- -->

- Gets or sets the Colormap texture size.

  remarks  
  This defaults to \[1024,1\] and simply sets the resolution of color mapping. Leave default in normal operation

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/size.html" class="tsd-signature-type">Size</a>

- Gets or sets the Colormap texture size.

  remarks  
  This defaults to \[1024,1\] and simply sets the resolution of color mapping. Leave default in normal operation

  #### Parameters

  - ##### colorMapTextureSize: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/size.html" class="tsd-signature-type">Size</a>

  #### Returns void

### contourInterval

- get contourInterval(): number
- set contourInterval(contourInterval: number): void

<!-- -->

- Gets or sets the contour interval, a value for the spacing between contour lines.

  description  
  For example, if you have data in the [SurfaceMeshRenderableSeries3D.dataSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#dataseries) ranging from 0-100.0, and you want to display contour lines every 1/10th then set [SurfaceMeshRenderableSeries3D.contourInterval](https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#contourinterval) = 10

  remarks  
  Defaults to 20, but must be set according to your data in the [SurfaceMeshRenderableSeries3D.dataSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#dataseries)

  #### Returns number

- Gets or sets the contour interval, a value for the spacing between contour lines.

  description  
  For example, if you have data in the [SurfaceMeshRenderableSeries3D.dataSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#dataseries) ranging from 0-100.0, and you want to display contour lines every 1/10th then set [SurfaceMeshRenderableSeries3D.contourInterval](https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#contourinterval) = 10

  remarks  
  Defaults to 20, but must be set according to your data in the [SurfaceMeshRenderableSeries3D.dataSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#dataseries)

  #### Parameters

  - ##### contourInterval: number

  #### Returns void

### contourOffset

- get contourOffset(): number
- set contourOffset(contourOffset: number): void

<!-- -->

- Gets or sets the contour offset, a value for the offsetting contour lines

  description  
  For example, if you have data in the [SurfaceMeshRenderableSeries3D.dataSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#dataseries) ranging from 0-100.0, and you want to display the first contour line at value = 5, then set [SurfaceMeshRenderableSeries3D.contourOffset](https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#contouroffset) = 5

  remarks  
  Defaults to 0.1, but must be set according to your data in the [SurfaceMeshRenderableSeries3D.dataSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#dataseries)

  #### Returns number

- Gets or sets the contour offset, a value for the offsetting contour lines

  description  
  For example, if you have data in the [SurfaceMeshRenderableSeries3D.dataSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#dataseries) ranging from 0-100.0, and you want to display the first contour line at value = 5, then set [SurfaceMeshRenderableSeries3D.contourOffset](https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#contouroffset) = 5

  remarks  
  Defaults to 0.1, but must be set according to your data in the [SurfaceMeshRenderableSeries3D.dataSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#dataseries)

  #### Parameters

  - ##### contourOffset: number

  #### Returns void

### contourStroke

- get contourStroke(): string
- set contourStroke(contourStroke: string): void

<!-- -->

- Gets or sets the contour stroke as an HTML Color Code

  #### Returns string

- Gets or sets the contour stroke as an HTML Color Code

  #### Parameters

  - ##### contourStroke: string

  #### Returns void

### contourStrokeThickness

- get contourStrokeThickness(): number
- set contourStrokeThickness(contourStrokeThickness: number): void

<!-- -->

- Gets or sets the stroke thickness of contour lines on the [SurfaceMeshRenderableSeries3D](https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype3d.html#surfacemeshrenderableseries3d)

  #### Returns number

- Gets or sets the stroke thickness of contour lines on the [SurfaceMeshRenderableSeries3D](https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype3d.html#surfacemeshrenderableseries3d)

  #### Parameters

  - ##### contourStrokeThickness: number

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

### drawMeshAs

- get drawMeshAs(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/edrawmeshas.html" class="tsd-signature-type">EDrawMeshAs</a>
- set drawMeshAs(drawMeshAs: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/edrawmeshas.html" class="tsd-signature-type">EDrawMeshAs</a>): void

<!-- -->

- Gets or sets the [EDrawMeshAs](https://www.scichart.com/documentation/js/v4/typedoc/enums/edrawmeshas.html), whether the mesh is drawn as wireframe, solid, with or without contours etc...

  remarks  
  Default value is SOLID_WIREFRAME. For contours, choose SOLID_WITH_CONTOURS

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/edrawmeshas.html" class="tsd-signature-type">EDrawMeshAs</a>

- Gets or sets the [EDrawMeshAs](https://www.scichart.com/documentation/js/v4/typedoc/enums/edrawmeshas.html), whether the mesh is drawn as wireframe, solid, with or without contours etc...

  remarks  
  Default value is SOLID_WIREFRAME. For contours, choose SOLID_WITH_CONTOURS

  #### Parameters

  - ##### drawMeshAs: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/edrawmeshas.html" class="tsd-signature-type">EDrawMeshAs</a>

  #### Returns void

### drawSkirt

- get drawSkirt(): boolean
- set drawSkirt(drawSkirt: boolean): void

<!-- -->

- When true, draws a skirt from the edge of the mesh to zero (solid walls on the left, right, top, bottom side)

  #### Returns boolean

- When true, draws a skirt from the edge of the mesh to zero (solid walls on the left, right, top, bottom side)

  #### Parameters

  - ##### drawSkirt: boolean

  #### Returns void

### heightScaleFactor

- get heightScaleFactor(): number
- set heightScaleFactor(heightScaleFactor: number): void

<!-- -->

- Gets or sets the height scale factor for the [SurfaceMeshRenderableSeries3D](https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype3d.html#surfacemeshrenderableseries3d).

  remarks  
  Default value is 1. A height scale factor of 0 makes the mesh flat, and 0.5 means all heights are multiplied by 0.5 etc...

  #### Returns number

- Gets or sets the height scale factor for the [SurfaceMeshRenderableSeries3D](https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype3d.html#surfacemeshrenderableseries3d).

  remarks  
  Default value is 1. A height scale factor of 0 makes the mesh flat, and 0.5 means all heights are multiplied by 0.5 etc...

  #### Parameters

  - ##### heightScaleFactor: number

  #### Returns void

### highlight

- get highlight(): number
- set highlight(highlight: number): void

<!-- -->

- Gets or sets the highlight factor, a number used to calculate lighting effects.

  remarks  
  Defaults to 0.05

  #### Returns number

- Gets or sets the highlight factor, a number used to calculate lighting effects.

  remarks  
  Defaults to 0.05

  #### Parameters

  - ##### highlight: number

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

### lightingFactor

- get lightingFactor(): number
- set lightingFactor(lightingFactor: number): void

<!-- -->

- Gets or sets the lighting factor, a number used to calculate lighting effects.

  remarks  
  Defaults to 0.8

  #### Returns number

- Gets or sets the lighting factor, a number used to calculate lighting effects.

  remarks  
  Defaults to 0.8

  #### Parameters

  - ##### lightingFactor: number

  #### Returns void

### maximum

- get maximum(): number
- set maximum(maximum: number): void

<!-- -->

- Gets or sets the maximum value in the [SurfaceMeshRenderableSeries3D.dataSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#dataseries) that we want to map to colours in the [SurfaceMeshRenderableSeries3D.meshColorPalette](https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#meshcolorpalette)

  remarks  
  For example, if data contains values 0..100, and the meshColorPalette is a [GradientColorPalette](https://www.scichart.com/documentation/js/v4/typedoc/classes/gradientcolorpalette.html) from red to blue, and you want 0=red and blue=100 then you should set [SurfaceMeshRenderableSeries3D.minimum](https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#minimum) = 0 and [SurfaceMeshRenderableSeries3D.maximum](https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#maximum) = 1

  #### Returns number

- Gets or sets the maximum value in the [SurfaceMeshRenderableSeries3D.dataSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#dataseries) that we want to map to colours in the [SurfaceMeshRenderableSeries3D.meshColorPalette](https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#meshcolorpalette)

  remarks  
  For example, if data contains values 0..100, and the meshColorPalette is a [GradientColorPalette](https://www.scichart.com/documentation/js/v4/typedoc/classes/gradientcolorpalette.html) from red to blue, and you want 0=red and blue=100 then you should set [SurfaceMeshRenderableSeries3D.minimum](https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#minimum) = 0 and [SurfaceMeshRenderableSeries3D.maximum](https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#maximum) = 1

  #### Parameters

  - ##### maximum: number

  #### Returns void

### meshColorPalette

- get meshColorPalette(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/meshcolorpalette.html" class="tsd-signature-type">MeshColorPalette</a>
- set meshColorPalette(meshColorPalette: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/meshcolorpalette.html" class="tsd-signature-type">MeshColorPalette</a>): void

<!-- -->

- Gets or sets the [MeshColorPalette](https://www.scichart.com/documentation/js/v4/typedoc/classes/meshcolorpalette.html), which defines how values in the [SurfaceMeshRenderableSeries3D.dataSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#dataseries) map to colours

  remarks  
  See concrete types [SolidColorBrushPalette](https://www.scichart.com/documentation/js/v4/typedoc/classes/solidcolorbrushpalette.html) and [GradientColorPalette](https://www.scichart.com/documentation/js/v4/typedoc/classes/gradientcolorpalette.html) plus our examples for more information.

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/meshcolorpalette.html" class="tsd-signature-type">MeshColorPalette</a>

- Gets or sets the [MeshColorPalette](https://www.scichart.com/documentation/js/v4/typedoc/classes/meshcolorpalette.html), which defines how values in the [SurfaceMeshRenderableSeries3D.dataSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#dataseries) map to colours

  remarks  
  See concrete types [SolidColorBrushPalette](https://www.scichart.com/documentation/js/v4/typedoc/classes/solidcolorbrushpalette.html) and [GradientColorPalette](https://www.scichart.com/documentation/js/v4/typedoc/classes/gradientcolorpalette.html) plus our examples for more information.

  #### Parameters

  - ##### meshColorPalette: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/meshcolorpalette.html" class="tsd-signature-type">MeshColorPalette</a>

  #### Returns void

### meshPaletteMode

- get meshPaletteMode(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/emeshpalettemode.html" class="tsd-signature-type">EMeshPaletteMode</a>
- set meshPaletteMode(meshPaletteMode: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/emeshpalettemode.html" class="tsd-signature-type">EMeshPaletteMode</a>): void

<!-- -->

- Gets or sets the [EMeshPaletteMode](https://www.scichart.com/documentation/js/v4/typedoc/enums/emeshpalettemode.html), which defines how heightmaps are treated.

  remarks  
  Defaults to HEIGHT_MAP_INTERPOLATED

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/emeshpalettemode.html" class="tsd-signature-type">EMeshPaletteMode</a>

- Gets or sets the [EMeshPaletteMode](https://www.scichart.com/documentation/js/v4/typedoc/enums/emeshpalettemode.html), which defines how heightmaps are treated.

  remarks  
  Defaults to HEIGHT_MAP_INTERPOLATED

  #### Parameters

  - ##### meshPaletteMode: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/emeshpalettemode.html" class="tsd-signature-type">EMeshPaletteMode</a>

  #### Returns void

### meshResolution

- get meshResolution(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/emeshresolution.html" class="tsd-signature-type">EMeshResolution</a>
- set meshResolution(meshResolution: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/emeshresolution.html" class="tsd-signature-type">EMeshResolution</a>): void

<!-- -->

- Gets or sets the [EMeshResolution](https://www.scichart.com/documentation/js/v4/typedoc/enums/emeshresolution.html), the amount of oversampling when creating a mesh.

  remarks  
  Defaults to 1x.

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/emeshresolution.html" class="tsd-signature-type">EMeshResolution</a>

- Gets or sets the [EMeshResolution](https://www.scichart.com/documentation/js/v4/typedoc/enums/emeshresolution.html), the amount of oversampling when creating a mesh.

  remarks  
  Defaults to 1x.

  #### Parameters

  - ##### meshResolution: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/emeshresolution.html" class="tsd-signature-type">EMeshResolution</a>

  #### Returns void

### minimum

- get minimum(): number
- set minimum(minimum: number): void

<!-- -->

- Gets or sets the minimum value in the [SurfaceMeshRenderableSeries3D.dataSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#dataseries) that we want to map to colours in the [SurfaceMeshRenderableSeries3D.meshColorPalette](https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#meshcolorpalette)

  remarks  
  For example, if data contains values 0..100, and the meshColorPalette is a [GradientColorPalette](https://www.scichart.com/documentation/js/v4/typedoc/classes/gradientcolorpalette.html) from red to blue, and you want 0=red and blue=100 then you should set [SurfaceMeshRenderableSeries3D.minimum](https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#minimum) = 0 and [SurfaceMeshRenderableSeries3D.maximum](https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#maximum) = 1

  #### Returns number

- Gets or sets the minimum value in the [SurfaceMeshRenderableSeries3D.dataSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#dataseries) that we want to map to colours in the [SurfaceMeshRenderableSeries3D.meshColorPalette](https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#meshcolorpalette)

  remarks  
  For example, if data contains values 0..100, and the meshColorPalette is a [GradientColorPalette](https://www.scichart.com/documentation/js/v4/typedoc/classes/gradientcolorpalette.html) from red to blue, and you want 0=red and blue=100 then you should set [SurfaceMeshRenderableSeries3D.minimum](https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#minimum) = 0 and [SurfaceMeshRenderableSeries3D.maximum](https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#maximum) = 1

  #### Parameters

  - ##### minimum: number

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

- get pointMarker(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basepointmarker3d.html" class="tsd-signature-type">BasePointMarker3D</a> \| undefined
- set pointMarker(pointMarker: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basepointmarker3d.html" class="tsd-signature-type">BasePointMarker3D</a> \| undefined): void

<!-- -->

- A [3D Point Marker](https://www.scichart.com/documentation/js/v4/typedoc/classes/basepointmarker3d.html) which is used to draw an optional 3D point-marker at each Xyz data-point. Applicable to some series types only

  inheritdoc  

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basepointmarker3d.html" class="tsd-signature-type">BasePointMarker3D</a> \| undefined

- A [3D Point Marker](https://www.scichart.com/documentation/js/v4/typedoc/classes/basepointmarker3d.html) which is used to draw an optional 3D point-marker at each Xyz data-point. Applicable to some series types only

  inheritdoc  

  #### Parameters

  - ##### pointMarker: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basepointmarker3d.html" class="tsd-signature-type">BasePointMarker3D</a> \| undefined

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

### strokeThickness

- get strokeThickness(): number
- set strokeThickness(strokeThickness: number): void

<!-- -->

- Gets or sets the stroke thickness of mesh wireframe lines on the [SurfaceMeshRenderableSeries3D](https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype3d.html#surfacemeshrenderableseries3d)

  #### Returns number

- Gets or sets the stroke thickness of mesh wireframe lines on the [SurfaceMeshRenderableSeries3D](https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype3d.html#surfacemeshrenderableseries3d)

  #### Parameters

  - ##### strokeThickness: number

  #### Returns void

### type

- get type(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype3d.html" class="tsd-signature-type">ESeriesType3D</a>

<!-- -->

- Gets the Series type. See [ESeriesType3D](https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype3d.html) for a list of values

  inheritdoc  

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype3d.html" class="tsd-signature-type">ESeriesType3D</a>

### yOffset

- get yOffset(): number
- set yOffset(yOffset: number): void

<!-- -->

- Gets or sets a Y-offset or height-offset in world coordinates.

  remarks  
  Defaults to 0. When setting to a value such as 10, the entire [SurfaceMeshRenderableSeries3D](https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype3d.html#surfacemeshrenderableseries3d) will be offset to height=10

  #### Returns number

- Gets or sets a Y-offset or height-offset in world coordinates.

  remarks  
  Defaults to 0. When setting to a value such as 10, the entire [SurfaceMeshRenderableSeries3D](https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype3d.html#surfacemeshrenderableseries3d) will be offset to height=10

  #### Parameters

  - ##### yOffset: number

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

### getDataSeriesName

- getDataSeriesName(): string

<!-- -->

- #### Returns string

### Protected getOptions

- getOptions(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/isurfacemeshrenderableseries3doptions.html" class="tsd-signature-type">ISurfaceMeshRenderableSeries3DOptions</a>

<!-- -->

- #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/isurfacemeshrenderableseries3doptions.html" class="tsd-signature-type">ISurfaceMeshRenderableSeries3DOptions</a>

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

- Notifies listeners to [invalidateParentCallback](https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#invalidateparentcallback) that a property has changed

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
