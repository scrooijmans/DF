<img src="out_scichartv4/typedoc/classes/basesceneentity3d_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basesceneentity3d.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [BaseSceneEntity3D](https://www.scichart.com/documentation/js/v4/typedoc/classes/basesceneentity3d.html)

# Class BaseSceneEntity3D\<TNativeEntity\>

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

The [BaseSceneEntity3D](https://www.scichart.com/documentation/js/v4/typedoc/classes/basesceneentity3d.html) provides a base class for entities, or 3D objects in the 3D scene within SciChart's High Performance <a href="https://www.scichart.com/javascript-chart-features" class="external">JavaScript 3D Charts</a>

remarks  
Each [BaseSceneEntity3D](https://www.scichart.com/documentation/js/v4/typedoc/classes/basesceneentity3d.html) wraps a native WebAssembly [SCRTSceneEntity](https://www.scichart.com/documentation/js/v4/typedoc/enums/esceneentitytype.html#scrtsceneentity) which is returned by the [nativeEntity](https://www.scichart.com/documentation/js/v4/typedoc/classes/basesceneentity3d.html#nativeentity) property. This is passed to SciChart's 3D engine and inserted into the scene when added to the [SciChart3DSurface.rootEntity](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#rootentity) collection.

### Type parameters

- #### TNativeEntity: SCRTSceneEntity

### Hierarchy

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/deletableentity.html" class="tsd-signature-type">DeletableEntity</a>
  - BaseSceneEntity3D
    - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rootsceneentity.html" class="tsd-signature-type">RootSceneEntity</a>
    - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderableseriessceneentity.html" class="tsd-signature-type">RenderableSeriesSceneEntity</a>
    - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscubeentity.html" class="tsd-signature-type">AxisCubeEntity</a>
    - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/gizmoentity.html" class="tsd-signature-type">GizmoEntity</a>
    - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/crosshairlinessceneentity.html" class="tsd-signature-type">CrosshairLinesSceneEntity</a>

### Implements

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html" class="tsd-signature-type">IDeletable</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibasesceneentity.html" class="tsd-signature-type">IBaseSceneEntity</a>

## Index

### Constructors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basesceneentity3d.html#constructor" class="tsd-kind-icon">constructor</a>

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basesceneentity3d.html#children" class="tsd-kind-icon">children</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basesceneentity3d.html#currentrenderpassdata" class="tsd-kind-icon">currentRenderPassData</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basesceneentity3d.html#id" class="tsd-kind-icon">id</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basesceneentity3d.html#parent" class="tsd-kind-icon">parent</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basesceneentity3d.html#rootsceneentity" class="tsd-kind-icon">rootSceneEntity</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basesceneentity3d.html#type" class="tsd-kind-icon">type</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basesceneentity3d.html#webassemblycontext" class="tsd-kind-icon">webAssemblyContext</a>

### Accessors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basesceneentity3d.html#entityid" class="tsd-kind-icon">entityId</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basesceneentity3d.html#entityidprovider" class="tsd-kind-icon">entityIdProvider</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basesceneentity3d.html#isvisible" class="tsd-kind-icon">isVisible</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basesceneentity3d.html#nativeentity" class="tsd-kind-icon">nativeEntity</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basesceneentity3d.html#world" class="tsd-kind-icon">world</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basesceneentity3d.html#render" class="tsd-kind-icon">Render</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basesceneentity3d.html#update" class="tsd-kind-icon">Update</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basesceneentity3d.html#attachchild" class="tsd-kind-icon">attachChild</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basesceneentity3d.html#delete" class="tsd-kind-icon">delete</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basesceneentity3d.html#detachchild" class="tsd-kind-icon">detachChild</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basesceneentity3d.html#getentity" class="tsd-kind-icon">getEntity</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basesceneentity3d.html#getroot" class="tsd-kind-icon">getRoot</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basesceneentity3d.html#invalidatescene" class="tsd-kind-icon">invalidateScene</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basesceneentity3d.html#onattached" class="tsd-kind-icon">onAttached</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basesceneentity3d.html#ondetached" class="tsd-kind-icon">onDetached</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basesceneentity3d.html#ondpichanged" class="tsd-kind-icon">onDpiChanged</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basesceneentity3d.html#onenginerestart" class="tsd-kind-icon">onEngineRestart</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basesceneentity3d.html#setnativeentity" class="tsd-kind-icon">setNativeEntity</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basesceneentity3d.html#setrenderpassdata" class="tsd-kind-icon">setRenderPassData</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basesceneentity3d.html#visitentities" class="tsd-kind-icon">visitEntities</a>

## Constructors

### Protected constructor

- new BaseSceneEntity3D(webAssemblyContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart3d" class="tsd-signature-type">TSciChart3D</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basesceneentity3d.html" class="tsd-signature-type">BaseSceneEntity3D</a>

<!-- -->

- Creates an instance of the [BaseSceneEntity3D](https://www.scichart.com/documentation/js/v4/typedoc/classes/basesceneentity3d.html)

  #### Parameters

  - ##### webAssemblyContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart3d" class="tsd-signature-type">TSciChart3D</a>

    The [SciChart 3D WebAssembly Context](https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart3d) containing native methods and access to our WebGL2 Engine and WebAssembly numerical methods

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basesceneentity3d.html" class="tsd-signature-type">BaseSceneEntity3D</a>

## Properties

### Readonly children

children: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/observablearray.html" class="tsd-signature-type">ObservableArray</a>\<<a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibasesceneentity.html" class="tsd-signature-type">IBaseSceneEntity</a>\>

summary  
Gets the collection of [IBaseSceneEntity](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibasesceneentity.html) - child entities or 3D Objects which belong to this entity.

description  
A [SciChart3DSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html) has a [SciChart3DSurface.rootEntity](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#rootentity) property. You can add [BaseSceneEntity3D](https://www.scichart.com/documentation/js/v4/typedoc/classes/basesceneentity3d.html) to this property, and each entity can have a collection of child entities.

Use grouping to control visibility of many entities at once, or to create more complex scenes.

remarks  
Adding a [BaseSceneEntity3D](https://www.scichart.com/documentation/js/v4/typedoc/classes/basesceneentity3d.html) to the children collection will cause the 3D Scene to be redrawn.

### Protected currentRenderPassData

currentRenderPassData: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassinfo3d.html" class="tsd-signature-type">RenderPassInfo3D</a>

The [RenderPassInfo3D](https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassinfo3d.html) containing data about the current rendering pass

### Readonly id

id: string = generateGuid()

A unique Id for the [IBaseSceneEntity](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibasesceneentity.html)

### parent

parent: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibasesceneentity.html" class="tsd-signature-type">IBaseSceneEntity</a>

Gets the parent [Entity](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibasesceneentity.html)

### Readonly rootSceneEntity

rootSceneEntity: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rootsceneentity.html" class="tsd-signature-type">RootSceneEntity</a>

inheritdoc  

### Readonly Abstract type

type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/esceneentitytype.html" class="tsd-signature-type">ESceneEntityType</a>

Gets the type of Scene Entity. See [ESceneEntityType](https://www.scichart.com/documentation/js/v4/typedoc/enums/esceneentitytype.html) for a list of values

### Protected webAssemblyContext

webAssemblyContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart3d" class="tsd-signature-type">TSciChart3D</a>

The [SciChart 3D WebAssembly Context](https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart3d) containing native methods and access to our WebGL2 Engine and WebAssembly numerical methods

## Accessors

### entityId

- get entityId(): number
- set entityId(value: number): void

<!-- -->

- Gets or sets a unique Id for the [IBaseSceneEntity](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibasesceneentity.html). The SciChart3D engine requires Ids fit into UInt32 (4 billion)

  inheritdoc  

  #### Returns number

- Gets or sets a unique Id for the [IBaseSceneEntity](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibasesceneentity.html). The SciChart3D engine requires Ids fit into UInt32 (4 billion)

  inheritdoc  

  #### Parameters

  - ##### value: number

  #### Returns void

### entityIdProvider

- get entityIdProvider(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ientityidprovider.html" class="tsd-signature-type">IEntityIdProvider</a>
- set entityIdProvider(value: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ientityidprovider.html" class="tsd-signature-type">IEntityIdProvider</a>): void

<!-- -->

- Gets or sets the [IEntityIdProvider](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ientityidprovider.html) which generates unique mesh Ids for {@link IBaseSceneEntity

  inheritdoc  

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ientityidprovider.html" class="tsd-signature-type">IEntityIdProvider</a>

- Gets or sets the [IEntityIdProvider](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ientityidprovider.html) which generates unique mesh Ids for {@link IBaseSceneEntity

  inheritdoc  

  #### Parameters

  - ##### value: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ientityidprovider.html" class="tsd-signature-type">IEntityIdProvider</a>

  #### Returns void

### isVisible

- get isVisible(): boolean
- set isVisible(value: boolean): void

<!-- -->

- Gets or Sets whether the entity and all its children are visible

  #### Returns boolean

- Gets or Sets whether the entity and all its children are visible

  #### Parameters

  - ##### value: boolean

  #### Returns void

### nativeEntity

- get nativeEntity(): TNativeEntity

<!-- -->

- Gets the native [SCRTSceneEntity](https://www.scichart.com/documentation/js/v4/typedoc/enums/esceneentitytype.html#scrtsceneentity) - a WebAssembly 3D Scene Entity type which is passed to our WebGL WebAssembly 3D Engine

  inheritdoc  

  #### Returns TNativeEntity

### Protected world

- get world(): SCRTSceneWorld

<!-- -->

- Called internally - gets the world entity

  #### Returns SCRTSceneWorld

## Methods

### Render

- Render(): void

<!-- -->

- Render method called from WebAssembly engine. Use this to do immediate-mode 3D drawing, or to draw created meshes When overriding, you must call super.Update() for the object to draw in the scene

  #### Returns void

### Update

- Update(deltaTime: number): void

<!-- -->

- Update method called from WebAssembly engine. Use this to update meshes, properties, geometry before draw. When overriding, you must call super.Update() for the object to draw in the scene

  #### Parameters

  - ##### deltaTime: number

  #### Returns void

### Protected attachChild

- attachChild(childEntity: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibasesceneentity.html" class="tsd-signature-type">IBaseSceneEntity</a>): void

<!-- -->

- Called internally - Attach a child to the current entity

  #### Parameters

  - ##### childEntity: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibasesceneentity.html" class="tsd-signature-type">IBaseSceneEntity</a>

  #### Returns void

### delete

- delete(): void

<!-- -->

- Deletes native (WebAssembly) memory used by this type, after which it cannot be used.

  remarks  
  Call .delete() before finishing with the object to ensure that WebAssmembly memory leaks do not occur.

  All elements within SciChart's High Performance <a href="https://www.scichart.com/javascript-chart-features" class="external">Realtime JavaScript Charts</a> which implement [IDeletable](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html) must be deleted manually to free native (WebAssembly) memory

  #### Returns void

### Protected detachChild

- detachChild(childEntity: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibasesceneentity.html" class="tsd-signature-type">IBaseSceneEntity</a>): void

<!-- -->

- Called internally - detach a child from the current entity

  #### Parameters

  - ##### childEntity: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibasesceneentity.html" class="tsd-signature-type">IBaseSceneEntity</a>

  #### Returns void

### getEntity

- getEntity(type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/esceneentitytype.html" class="tsd-signature-type">ESceneEntityType</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibasesceneentity.html" class="tsd-signature-type">IBaseSceneEntity</a>

<!-- -->

- Gets the first child [IBaseSceneEntity](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibasesceneentity.html) of type specified by [ESceneEntityType](https://www.scichart.com/documentation/js/v4/typedoc/enums/esceneentitytype.html)

  #### Parameters

  - ##### type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/esceneentitytype.html" class="tsd-signature-type">ESceneEntityType</a>

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibasesceneentity.html" class="tsd-signature-type">IBaseSceneEntity</a>

### getRoot

- getRoot(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rootsceneentity.html" class="tsd-signature-type">RootSceneEntity</a>

<!-- -->

- Gets the root entity in the 3D Scene

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rootsceneentity.html" class="tsd-signature-type">RootSceneEntity</a>

### invalidateScene

- invalidateScene(): void

<!-- -->

- Call this to inform SciChart that data or properties have changed and the 3D Scene must be redrawn

  #### Returns void

### onAttached

- onAttached(): void

<!-- -->

- Called when the [IBaseSceneEntity](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibasesceneentity.html) is attached to the parent scene

  #### Returns void

### onDetached

- onDetached(): void

<!-- -->

- Called when the [IBaseSceneEntity](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibasesceneentity.html) is detached from the parent scene

  #### Returns void

### onDpiChanged

- onDpiChanged(args: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tdpichangedeventargs" class="tsd-signature-type">TDpiChangedEventArgs</a>): void

<!-- -->

- Called when the Dpi changes in the browser. This could be due to user zooming the browser, or changing DPI settings in Windows, or moving the browser containing SciChart to another monitor

  #### Parameters

  - ##### args: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tdpichangedeventargs" class="tsd-signature-type">TDpiChangedEventArgs</a>

    The [TDpiChangedEventArgs](https://www.scichart.com/documentation/js/v4/typedoc/index.html#tdpichangedeventargs) containing info about the Dpi Changed event

  #### Returns void

### onEngineRestart

- onEngineRestart(): void

<!-- -->

- Called when the WebGL 3D Engine restarts. Use this to perform clean-up operations

  #### Returns void

### Protected setNativeEntity

- setNativeEntity(entity: TNativeEntity): void

<!-- -->

- Called internally - sets the native entity

  #### Parameters

  - ##### entity: TNativeEntity

  #### Returns void

### setRenderPassData

- setRenderPassData(rpd: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassinfo3d.html" class="tsd-signature-type">RenderPassInfo3D</a>): void

<!-- -->

- Sets the [RenderPassInfo3D](https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassinfo3d.html) - render pass info, properties and data for the current rendering pass

  #### Parameters

  - ##### rpd: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassinfo3d.html" class="tsd-signature-type">RenderPassInfo3D</a>

  #### Returns void

### visitEntities

- visitEntities(operation: (e: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibasesceneentity.html" class="tsd-signature-type">IBaseSceneEntity</a>) =\> void): void

<!-- -->

- inheritdoc  

  #### Parameters

  - ##### operation: (e: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibasesceneentity.html" class="tsd-signature-type">IBaseSceneEntity</a>) =\> void

    - - (e: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibasesceneentity.html" class="tsd-signature-type">IBaseSceneEntity</a>): void

      <!-- -->

      - #### Parameters

        - ##### e: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibasesceneentity.html" class="tsd-signature-type">IBaseSceneEntity</a>

        #### Returns void

  #### Returns void

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
