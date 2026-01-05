<img src="out_scichartv4/typedoc/classes/renderableseriessceneentitystate_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderableseriessceneentitystate.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [RenderableSeriesSceneEntityState](https://www.scichart.com/documentation/js/v4/typedoc/classes/renderableseriessceneentitystate.html)

# Class RenderableSeriesSceneEntityState

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

State object for a 3D [RenderableSeriesSceneEntity](https://www.scichart.com/documentation/js/v4/typedoc/classes/renderableseriessceneentity.html) that contains a set of boolean flags and other state types (e.g. TAxisCubeState) that determine whether underlying graphics resources of the Scene Entity are up-to-date and reflect the current state of its properties and the date it visualizes.

### Hierarchy

- RenderableSeriesSceneEntityState
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshsceneentitystate.html" class="tsd-signature-type">SurfaceMeshSceneEntityState</a>

## Index

### Constructors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderableseriessceneentitystate.html#constructor" class="tsd-kind-icon">constructor</a>

### Accessors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderableseriessceneentitystate.html#isaxiscubestatechanged" class="tsd-kind-icon">isAxisCubeStateChanged</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderableseriessceneentitystate.html#isdataseriesmodified" class="tsd-kind-icon">isDataSeriesModified</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderableseriessceneentitystate.html#isinitialstate" class="tsd-kind-icon">isInitialState</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderableseriessceneentitystate.html#isrenderableseriespropertychanged" class="tsd-kind-icon">isRenderableSeriesPropertyChanged</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderableseriessceneentitystate.html#reset" class="tsd-kind-icon">reset</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderableseriessceneentitystate.html#setinitialstate" class="tsd-kind-icon">setInitialState</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderableseriessceneentitystate.html#setrenderableseriespropertychanged" class="tsd-kind-icon">setRenderableSeriesPropertyChanged</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderableseriessceneentitystate.html#validate" class="tsd-kind-icon">validate</a>

## Constructors

### constructor

- new RenderableSeriesSceneEntityState(isInitialStateProperty?: boolean, isDataSeriesModifiedProperty?: boolean, isRenderableSeriesPropertyChangedProperty?: boolean, isAxisCubeStateChangedProperty?: boolean, axisCubeState?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#taxiscubestate" class="tsd-signature-type">TAxisCubeState</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderableseriessceneentitystate.html" class="tsd-signature-type">RenderableSeriesSceneEntityState</a>

<!-- -->

- #### Parameters

  - ##### Default value isInitialStateProperty: boolean = true

  - ##### Default value isDataSeriesModifiedProperty: boolean = true

  - ##### Default value isRenderableSeriesPropertyChangedProperty: boolean = true

  - ##### Default value isAxisCubeStateChangedProperty: boolean = true

  - ##### Default value axisCubeState: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#taxiscubestate" class="tsd-signature-type">TAxisCubeState</a> = {xVisibleMin: 0,xVisibleMax: 0,yVisibleMin: 0,yVisibleMax: 0,zVisibleMin: 0,zVisibleMax: 0,xWorldDimension: 0,yWorldDimension: 0,zWorldDimension: 0}

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderableseriessceneentitystate.html" class="tsd-signature-type">RenderableSeriesSceneEntityState</a>

## Accessors

### isAxisCubeStateChanged

- get isAxisCubeStateChanged(): boolean

<!-- -->

- Gets whether an Axis Cube state has been changed since last update.

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

- reset(rs: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries3d.html" class="tsd-signature-type">IRenderableSeries3D</a>, rpi: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassinfo3d.html" class="tsd-signature-type">RenderPassInfo3D</a>): void

<!-- -->

- Resets all the state changed flags and saves the current state of Rederable Series and state of other objects in the Render Pass (e.g. Axis Cube).

  #### Parameters

  - ##### rs: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries3d.html" class="tsd-signature-type">IRenderableSeries3D</a>

    The Rendrable Series, whose state gets save

  - ##### rpi: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassinfo3d.html" class="tsd-signature-type">RenderPassInfo3D</a>

    The current [RenderPassInfo3D](https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassinfo3d.html), whose state gets save

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

- Checks if a state of the Renderable Series and a state of other objects in the Render Pass (e.g. Axis Cube) has been changed. Sets corresponding flags, if any.

  #### Parameters

  - ##### rs: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries3d.html" class="tsd-signature-type">IRenderableSeries3D</a>

    The Rendrable Series to check

  - ##### rpi: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassinfo3d.html" class="tsd-signature-type">RenderPassInfo3D</a>

    The current [RenderPassInfo3D](https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassinfo3d.html) to check

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
