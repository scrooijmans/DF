<img src="out_scichartv4/typedoc/classes/gradientcolorpalette_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/gradientcolorpalette.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [GradientColorPalette](https://www.scichart.com/documentation/js/v4/typedoc/classes/gradientcolorpalette.html)

# Class GradientColorPalette

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

Provides a gradient color palette which may be applied to the [SurfaceMeshRenderableSeries3D.meshColorPalette](https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html#meshcolorpalette) property

### Hierarchy

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/meshcolorpalette.html" class="tsd-signature-type">MeshColorPalette</a>
  - GradientColorPalette

## Index

### Constructors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/gradientcolorpalette.html#constructor" class="tsd-kind-icon">constructor</a>

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/gradientcolorpalette.html#propertychanged" class="tsd-kind-icon">propertyChanged</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/gradientcolorpalette.html#type" class="tsd-kind-icon">type</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/gradientcolorpalette.html#webassemblycontext" class="tsd-kind-icon">webAssemblyContext</a>

### Accessors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/gradientcolorpalette.html#gradientstops" class="tsd-kind-icon">gradientStops</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/gradientcolorpalette.html#getoptions" class="tsd-kind-icon">getOptions</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/gradientcolorpalette.html#gettexture" class="tsd-kind-icon">getTexture</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/gradientcolorpalette.html#notifypropertychanged" class="tsd-kind-icon">notifyPropertyChanged</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/gradientcolorpalette.html#tojson" class="tsd-kind-icon">toJSON</a>

## Constructors

### constructor

- new GradientColorPalette(webAssemblyContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart3d" class="tsd-signature-type">TSciChart3D</a>, options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/igradientcolorpaletteoptions.html" class="tsd-signature-type">IGradientColorPaletteOptions</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/gradientcolorpalette.html" class="tsd-signature-type">GradientColorPalette</a>

<!-- -->

- Creates an instance of the [GradientColorPalette](https://www.scichart.com/documentation/js/v4/typedoc/classes/gradientcolorpalette.html)

  #### Parameters

  - ##### webAssemblyContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart3d" class="tsd-signature-type">TSciChart3D</a>

    The [SciChart 3D WebAssembly Context](https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart3d) containing native methods and access to our WebGL2 Engine and WebAssembly numerical methods

  - ##### Optional options: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/igradientcolorpaletteoptions.html" class="tsd-signature-type">IGradientColorPaletteOptions</a>

    optional parameters of type [IGradientColorPaletteOptions](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/igradientcolorpaletteoptions.html) passed to the constructor

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/gradientcolorpalette.html" class="tsd-signature-type">GradientColorPalette</a>

## Properties

### propertyChanged

propertyChanged: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/eventhandler.html" class="tsd-signature-type">EventHandler</a>\<<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/propertychangedeventargs.html" class="tsd-signature-type">PropertyChangedEventArgs</a>\>

An event handler which notifies subscribers that a property has changed and scene needs to be redrawn

### Readonly type

type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/epaletteprovidertype.html" class="tsd-signature-type">EPaletteProviderType</a> = EPaletteProviderType.Gradient3D

### Protected webAssemblyContext

webAssemblyContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart3d" class="tsd-signature-type">TSciChart3D</a>

The [SciChart 3D WebAssembly Context](https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart3d) containing native methods and access to our WebGL2 Engine and WebAssembly numerical methods

## Accessors

### gradientStops

- get gradientStops(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tgradientstop" class="tsd-signature-type">TGradientStop</a>\[\]
- set gradientStops(gradientStops: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tgradientstop" class="tsd-signature-type">TGradientStop</a>\[\]): void

<!-- -->

- The array of [Gradient Stops](https://www.scichart.com/documentation/js/v4/typedoc/index.html#tgradientstop) to apply with offsets from 0.0 - 1.0

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tgradientstop" class="tsd-signature-type">TGradientStop</a>\[\]

- The array of [Gradient Stops](https://www.scichart.com/documentation/js/v4/typedoc/index.html#tgradientstop) to apply with offsets from 0.0 - 1.0

  #### Parameters

  - ##### gradientStops: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tgradientstop" class="tsd-signature-type">TGradientStop</a>\[\]

  #### Returns void

## Methods

### Protected getOptions

- getOptions(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#requiredownprops" class="tsd-signature-type">RequiredOwnProps</a>\<<a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/igradientcolorpaletteoptions.html" class="tsd-signature-type">IGradientColorPaletteOptions</a>\>

<!-- -->

- #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#requiredownprops" class="tsd-signature-type">RequiredOwnProps</a>\<<a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/igradientcolorpaletteoptions.html" class="tsd-signature-type">IGradientColorPaletteOptions</a>\>

### getTexture

- getTexture(size: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/size.html" class="tsd-signature-type">Size</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/canvastexture.html" class="tsd-signature-type">CanvasTexture</a>

<!-- -->

- inheritdoc  

  #### Parameters

  - ##### size: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/size.html" class="tsd-signature-type">Size</a>

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/canvastexture.html" class="tsd-signature-type">CanvasTexture</a>

### Protected notifyPropertyChanged

- notifyPropertyChanged(propertyName: string): void

<!-- -->

- Notifies subscribers of [propertyChanged](https://www.scichart.com/documentation/js/v4/typedoc/classes/gradientcolorpalette.html#propertychanged)

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
