<img src="out_scichartv4/typedoc/classes/surfacemeshsceneentitystate_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshsceneentitystate.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [SurfaceMeshSceneEntityState](https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshsceneentitystate.html)

# Class SurfaceMeshSceneEntityState

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

State object for [SurfaceMeshSceneEntity](https://www.scichart.com/documentation/js/v4/typedoc/enums/esceneentitytype.html#surfacemeshsceneentity)

### Hierarchy

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderableseriessceneentitystate.html" class="tsd-signature-type">RenderableSeriesSceneEntityState</a>
  - SurfaceMeshSceneEntityState

## Index

### Constructors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshsceneentitystate.html#constructor" class="tsd-kind-icon">constructor</a>

### Accessors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshsceneentitystate.html#isaxiscubestatechanged" class="tsd-kind-icon">isAxisCubeStateChanged</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshsceneentitystate.html#iscolormaptextureinvalid" class="tsd-kind-icon">isColorMapTextureInvalid</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshsceneentitystate.html#isdataseriesmodified" class="tsd-kind-icon">isDataSeriesModified</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshsceneentitystate.html#isinitialstate" class="tsd-kind-icon">isInitialState</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshsceneentitystate.html#isrenderableseriespropertychanged" class="tsd-kind-icon">isRenderableSeriesPropertyChanged</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshsceneentitystate.html#reset" class="tsd-kind-icon">reset</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshsceneentitystate.html#setcolormaptextureinvalid" class="tsd-kind-icon">setColorMapTextureInvalid</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshsceneentitystate.html#setinitialstate" class="tsd-kind-icon">setInitialState</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshsceneentitystate.html#setrenderableseriespropertychanged" class="tsd-kind-icon">setRenderableSeriesPropertyChanged</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshsceneentitystate.html#validate" class="tsd-kind-icon">validate</a>

## Constructors

### constructor

- new SurfaceMeshSceneEntityState(isInitialStateProperty?: boolean, isDataSeriesModifiedProperty?: boolean, isRenderableSeriesPropertyChangedProperty?: boolean, isAxisCubeStateChangedProperty?: boolean, axisCubeState?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#taxiscubestate" class="tsd-signature-type">TAxisCubeState</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshsceneentitystate.html" class="tsd-signature-type">SurfaceMeshSceneEntityState</a>

<!-- -->

- #### Parameters

  - ##### Default value isInitialStateProperty: boolean = true

  - ##### Default value isDataSeriesModifiedProperty: boolean = true

  - ##### Default value isRenderableSeriesPropertyChangedProperty: boolean = true

  - ##### Default value isAxisCubeStateChangedProperty: boolean = true

  - ##### Default value axisCubeState: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#taxiscubestate" class="tsd-signature-type">TAxisCubeState</a> = {xVisibleMin: 0,xVisibleMax: 0,yVisibleMin: 0,yVisibleMax: 0,zVisibleMin: 0,zVisibleMax: 0,xWorldDimension: 0,yWorldDimension: 0,zWorldDimension: 0}

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshsceneentitystate.html" class="tsd-signature-type">SurfaceMeshSceneEntityState</a>

## Accessors

### isAxisCubeStateChanged

- get isAxisCubeStateChanged(): boolean

<!-- -->

- Gets whether an Axis Cube state has been changed since last update.

  #### Returns boolean

### isColorMapTextureInvalid

- get isColorMapTextureInvalid(): boolean

<!-- -->

- Gets or sets whether the color map texture is valid

  #### Returns boolean

### isDataSeriesModified

- get isDataSeriesModified(): boolean

<!-- -->

- Gets whether the Data Series has been modified since last update.

  #### Returns boolean

### isInitialState

- get isInitialState(): boolean

<!-- -->

- Gets whether the Scene Entity is in the initial state, meaning all the underlying resources must be (re-)created.

  #### Returns boolean

### isRenderableSeriesPropertyChanged

- get isRenderableSeriesPropertyChanged(): boolean

<!-- -->

- Gets whether a property of the Renderable Series has been changed since last update.

  #### Returns boolean

## Methods

### reset

- reset(rs?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries3d.html" class="tsd-signature-type">IRenderableSeries3D</a>, rpi?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassinfo3d.html" class="tsd-signature-type">RenderPassInfo3D</a>): void

<!-- -->

- inheritdoc  

  #### Parameters

  - ##### Optional rs: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries3d.html" class="tsd-signature-type">IRenderableSeries3D</a>

  - ##### Optional rpi: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassinfo3d.html" class="tsd-signature-type">RenderPassInfo3D</a>

  #### Returns void

### setColorMapTextureInvalid

- setColorMapTextureInvalid(): void

<!-- -->

- Gets or sets whether the color map texture is valid

  #### Returns void

### setInitialState

- setInitialState(): void

<!-- -->

- Sets whether the Scene Entity is in the initial state, meaning all the underlying resources must be (re-)created.

  #### Returns void

### setRenderableSeriesPropertyChanged

- setRenderableSeriesPropertyChanged(): void

<!-- -->

- Sets whether a property of the Renderable Series has been changed.

  #### Returns void

### validate

- validate(rs: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries3d.html" class="tsd-signature-type">IRenderableSeries3D</a>, rpi: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassinfo3d.html" class="tsd-signature-type">RenderPassInfo3D</a>): boolean

<!-- -->

- inheritdoc  

  #### Parameters

  - ##### rs: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries3d.html" class="tsd-signature-type">IRenderableSeries3D</a>

  - ##### rpi: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassinfo3d.html" class="tsd-signature-type">RenderPassInfo3D</a>

  #### Returns boolean

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
