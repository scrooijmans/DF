<img src="out_scichartv4/typedoc/classes/meshcolorpalette_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/meshcolorpalette.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [MeshColorPalette](https://www.scichart.com/documentation/js/v4/typedoc/classes/meshcolorpalette.html)

# Class MeshColorPalette

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

Provides a base class for color palettes which may be applied to the [SurfaceMeshRenderableSeries3D.meshColorPalette](https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#meshcolorpalette) property

remarks  
See concrete types [SolidColorBrushPalette](https://www.scichart.com/documentation/js/v4/typedoc/classes/solidcolorbrushpalette.html) and [GradientColorPalette](https://www.scichart.com/documentation/js/v4/typedoc/classes/gradientcolorpalette.html) for more details.

### Hierarchy

- MeshColorPalette
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/gradientcolorpalette.html" class="tsd-signature-type">GradientColorPalette</a>
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/solidcolorbrushpalette.html" class="tsd-signature-type">SolidColorBrushPalette</a>

## Index

### Constructors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/meshcolorpalette.html#constructor" class="tsd-kind-icon">constructor</a>

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/meshcolorpalette.html#propertychanged" class="tsd-kind-icon">propertyChanged</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/meshcolorpalette.html#type" class="tsd-kind-icon">type</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/meshcolorpalette.html#webassemblycontext" class="tsd-kind-icon">webAssemblyContext</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/meshcolorpalette.html#getoptions" class="tsd-kind-icon">getOptions</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/meshcolorpalette.html#gettexture" class="tsd-kind-icon">getTexture</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/meshcolorpalette.html#notifypropertychanged" class="tsd-kind-icon">notifyPropertyChanged</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/meshcolorpalette.html#tojson" class="tsd-kind-icon">toJSON</a>

## Constructors

### Protected constructor

- new MeshColorPalette(webAssemblyContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart3d" class="tsd-signature-type">TSciChart3D</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/meshcolorpalette.html" class="tsd-signature-type">MeshColorPalette</a>

<!-- -->

- Creates an instance of a [MeshColorPalette](https://www.scichart.com/documentation/js/v4/typedoc/classes/meshcolorpalette.html)

  #### Parameters

  - ##### webAssemblyContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart3d" class="tsd-signature-type">TSciChart3D</a>

    The [SciChart 3D WebAssembly Context](https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart3d) containing native methods and access to our WebGL2 Engine and WebAssembly numerical methods

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/meshcolorpalette.html" class="tsd-signature-type">MeshColorPalette</a>

## Properties

### propertyChanged

propertyChanged: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/eventhandler.html" class="tsd-signature-type">EventHandler</a>\<<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/propertychangedeventargs.html" class="tsd-signature-type">PropertyChangedEventArgs</a>\>

An event handler which notifies subscribers that a property has changed and scene needs to be redrawn

### Readonly Abstract type

type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/epaletteprovidertype.html" class="tsd-signature-type">EPaletteProviderType</a>

### Protected webAssemblyContext

webAssemblyContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart3d" class="tsd-signature-type">TSciChart3D</a>

The [SciChart 3D WebAssembly Context](https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart3d) containing native methods and access to our WebGL2 Engine and WebAssembly numerical methods

## Methods

### Protected Abstract getOptions

- getOptions(): any

<!-- -->

- #### Returns any

### Abstract getTexture

- getTexture(size: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/size.html" class="tsd-signature-type">Size</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/canvastexture.html" class="tsd-signature-type">CanvasTexture</a>

<!-- -->

- Used internally - returns a [CanvasTexture](https://www.scichart.com/documentation/js/v4/typedoc/classes/canvastexture.html) containing the palette colors which will be sent to SciChart's WebGL WebAssembly rendering engine during rendering

  remarks  
  [CanvasTexture](https://www.scichart.com/documentation/js/v4/typedoc/classes/canvastexture.html) implements [IDeletable](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html) and must be deleted to free native memory

  #### Parameters

  - ##### size: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/size.html" class="tsd-signature-type">Size</a>

    The desired size of the texture. Default / expected value is \[1024,1\]

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/canvastexture.html" class="tsd-signature-type">CanvasTexture</a>

### Protected notifyPropertyChanged

- notifyPropertyChanged(propertyName: string): void

<!-- -->

- Notifies subscribers of [propertyChanged](https://www.scichart.com/documentation/js/v4/typedoc/classes/meshcolorpalette.html#propertychanged)

  #### Parameters

  - ##### propertyName: string

    The property name which changed

  #### Returns void

### toJSON

- toJSON(): { options: any; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/epaletteprovidertype.html" class="tsd-signature-type">EPaletteProviderType</a> }

<!-- -->

- #### Returns { options: any; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/epaletteprovidertype.html" class="tsd-signature-type">EPaletteProviderType</a> }

  - ##### options: any

  - ##### type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/epaletteprovidertype.html" class="tsd-signature-type">EPaletteProviderType</a>

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
