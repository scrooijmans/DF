<img src="out_scichartv4/typedoc/classes/trianglepointmarker3d_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/trianglepointmarker3d.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [TrianglePointMarker3D](https://www.scichart.com/documentation/js/v4/typedoc/classes/trianglepointmarker3d.html)

# Class TrianglePointMarker3D

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

summary  
The [TrianglePointMarker3D](https://www.scichart.com/documentation/js/v4/typedoc/classes/trianglepointmarker3d.html) renders a flat billboarded textured triangle to each XYZ point in a 3D Scatter or Bubble series.

remarks  
See related class [ScatterRenderableSeries3D](https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype3d.html#scatterrenderableseries3d) which renders a [BasePointMarker3D](https://www.scichart.com/documentation/js/v4/typedoc/classes/basepointmarker3d.html) at each Xyz point in an [XyzDataSeries3D](https://www.scichart.com/documentation/js/v4/typedoc/classes/xyzdataseries3d.html) Data Series.

### Hierarchy

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basetexturepointmarker3d.html" class="tsd-signature-type">BaseTexturePointMarker3D</a>
  - TrianglePointMarker3D

### Implements

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmarker3d.html" class="tsd-signature-type">IPointMarker3D</a>

## Index

### Constructors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/trianglepointmarker3d.html#constructor" class="tsd-kind-icon">constructor</a>

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/trianglepointmarker3d.html#propertychanged" class="tsd-kind-icon">propertyChanged</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/trianglepointmarker3d.html#type" class="tsd-kind-icon">type</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/trianglepointmarker3d.html#webassemblycontext" class="tsd-kind-icon">webAssemblyContext</a>

### Accessors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/trianglepointmarker3d.html#fill" class="tsd-kind-icon">fill</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/trianglepointmarker3d.html#markertype" class="tsd-kind-icon">markerType</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/trianglepointmarker3d.html#pointstexture" class="tsd-kind-icon">pointsTexture</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/trianglepointmarker3d.html#size" class="tsd-kind-icon">size</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/trianglepointmarker3d.html#notifypropertychanged" class="tsd-kind-icon">notifyPropertyChanged</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/trianglepointmarker3d.html#tojson" class="tsd-kind-icon">toJSON</a>

## Constructors

### Protected constructor

- new TrianglePointMarker3D(webAssemblyContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart3d" class="tsd-signature-type">TSciChart3D</a>, options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibasepointmarker3doptions.html" class="tsd-signature-type">IBasePointMarker3DOptions</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/trianglepointmarker3d.html" class="tsd-signature-type">TrianglePointMarker3D</a>

<!-- -->

- Creates an instance of [TrianglePointMarker3D](https://www.scichart.com/documentation/js/v4/typedoc/classes/trianglepointmarker3d.html)

  #### Parameters

  - ##### webAssemblyContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart3d" class="tsd-signature-type">TSciChart3D</a>

    The [SciChart 3D WebAssembly Context](https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart3d) containing native methods and access to our WebGL2 Engine and WebAssembly numerical methods

  - ##### Optional options: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibasepointmarker3doptions.html" class="tsd-signature-type">IBasePointMarker3DOptions</a>

    Optional parameters of type [IBasePointMarker3DOptions](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibasepointmarker3doptions.html) used to configure the point-marker

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/trianglepointmarker3d.html" class="tsd-signature-type">TrianglePointMarker3D</a>

## Properties

### Readonly propertyChanged

propertyChanged: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/eventhandler.html" class="tsd-signature-type">EventHandler</a>\<<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/propertychangedeventargs.html" class="tsd-signature-type">PropertyChangedEventArgs</a>\>

Event handler which informs subscribers that a property has changed and the surface needs redrawing

### type

type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/epointmarker3dtype.html" class="tsd-signature-type">EPointMarker3DType</a> = EPointMarker3DType.Triangle

### Protected Readonly webAssemblyContext

webAssemblyContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart3d" class="tsd-signature-type">TSciChart3D</a>

The [SciChart 3D WebAssembly Context](https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart3d) containing native methods and access to our WebGL2 Engine and WebAssembly numerical methods

## Accessors

### fill

- get fill(): string
- set fill(fill: string): void

<!-- -->

- Gets or sets the point-marker fill as an HTML Color Code

  #### Returns string

- Gets or sets the point-marker fill as an HTML Color Code

  #### Parameters

  - ##### fill: string

  #### Returns void

### markerType

- get markerType(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/emarkertype.html" class="tsd-signature-type">EMarkerType</a>

<!-- -->

- Defines the MarkerType, e.g. pixel point marker, Mesh (3d object) or textured-quad

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/emarkertype.html" class="tsd-signature-type">EMarkerType</a>

### pointsTexture

- get pointsTexture(): TSRTexture

<!-- -->

- inheritdoc  

  #### Returns TSRTexture

### size

- get size(): number
- set size(size: number): void

<!-- -->

- Gets or sets the size of the point-marker in world coordinates

  #### Returns number

- Gets or sets the size of the point-marker in world coordinates

  #### Parameters

  - ##### size: number

  #### Returns void

## Methods

### Protected notifyPropertyChanged

- notifyPropertyChanged(propertyName: string): void

<!-- -->

- Notifies listeners to [propertyChanged](https://www.scichart.com/documentation/js/v4/typedoc/classes/trianglepointmarker3d.html#propertychanged) that a property has changed and redraw is required

  #### Parameters

  - ##### propertyName: string

    the property name

  #### Returns void

### toJSON

- toJSON(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tpointmarkerdefinition3d" class="tsd-signature-type">TPointMarkerDefinition3D</a>

<!-- -->

- #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tpointmarkerdefinition3d" class="tsd-signature-type">TPointMarkerDefinition3D</a>

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
