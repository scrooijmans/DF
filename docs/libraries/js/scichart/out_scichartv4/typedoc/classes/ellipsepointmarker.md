<img src="out_scichartv4/typedoc/classes/ellipsepointmarker_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/ellipsepointmarker.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [EllipsePointMarker](https://www.scichart.com/documentation/js/v4/typedoc/classes/ellipsepointmarker.html)

# Class EllipsePointMarker

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

summary  
Point-marker type which renders an ellipse at each x-y datapoint location

remarks  
To apply the EllipsePointMarker to a [IRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries.html), use the following code:

``` ts
const sciChartSurface: SciChartSurface;
const wasmContext: TSciChart;

sciChartSurface.renderableSeries.add(
     new FastLineRenderableSeries(wasmContext, {
         pointMarker: new EllipsePointMarker(wasmContext, {
             width: 9,
             height: 9,
             fill: "#FF0000",
             stroke: "#0000FF",
             strokeThickness: 1
         })
     })
);
```

------------------------------------------------------------------------

ðŸ“š Docs: <a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/common-series-apis/drawing-point-markers/" class="external">https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/common-series-apis/drawing-point-markers/</a>

### Hierarchy

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basepointmarker.html" class="tsd-signature-type">BasePointMarker</a>
  - EllipsePointMarker

### Implements

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html" class="tsd-signature-type">IDeletable</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmarker.html" class="tsd-signature-type">IPointMarker</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icacheable.html" class="tsd-signature-type">ICacheable</a>

## Index

### Constructors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/ellipsepointmarker.html#constructor" class="tsd-kind-icon">constructor</a>

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/ellipsepointmarker.html#invalidateparentcallback" class="tsd-kind-icon">invalidateParentCallback</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/ellipsepointmarker.html#type" class="tsd-kind-icon">type</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/ellipsepointmarker.html#webassemblycontext" class="tsd-kind-icon">webAssemblyContext</a>

### Accessors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/ellipsepointmarker.html#antialias" class="tsd-kind-icon">antiAlias</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/ellipsepointmarker.html#fill" class="tsd-kind-icon">fill</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/ellipsepointmarker.html#height" class="tsd-kind-icon">height</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/ellipsepointmarker.html#lastpointonly" class="tsd-kind-icon">lastPointOnly</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/ellipsepointmarker.html#opacity" class="tsd-kind-icon">opacity</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/ellipsepointmarker.html#stroke" class="tsd-kind-icon">stroke</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/ellipsepointmarker.html#strokethickness" class="tsd-kind-icon">strokeThickness</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/ellipsepointmarker.html#width" class="tsd-kind-icon">width</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/ellipsepointmarker.html#adjustautocolor" class="tsd-kind-icon">adjustAutoColor</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/ellipsepointmarker.html#createcanvastexture" class="tsd-kind-icon">createCanvasTexture</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/ellipsepointmarker.html#delete" class="tsd-kind-icon">delete</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/ellipsepointmarker.html#drawsprite" class="tsd-kind-icon">drawSprite</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/ellipsepointmarker.html#getfillmask" class="tsd-kind-icon">getFillMask</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/ellipsepointmarker.html#getpointmarkerstyle" class="tsd-kind-icon">getPointMarkerStyle</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/ellipsepointmarker.html#getsprite" class="tsd-kind-icon">getSprite</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/ellipsepointmarker.html#getstrokemask" class="tsd-kind-icon">getStrokeMask</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/ellipsepointmarker.html#invalidatecache" class="tsd-kind-icon">invalidateCache</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/ellipsepointmarker.html#notifypropertychanged" class="tsd-kind-icon">notifyPropertyChanged</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/ellipsepointmarker.html#ondpichanged" class="tsd-kind-icon">onDpiChanged</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/ellipsepointmarker.html#resetcache" class="tsd-kind-icon">resetCache</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/ellipsepointmarker.html#resolveautocolors" class="tsd-kind-icon">resolveAutoColors</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/ellipsepointmarker.html#resumeupdates" class="tsd-kind-icon">resumeUpdates</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/ellipsepointmarker.html#suspendupdates" class="tsd-kind-icon">suspendUpdates</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/ellipsepointmarker.html#tojson" class="tsd-kind-icon">toJSON</a>

## Constructors

### constructor

- new EllipsePointMarker(webAssemblyContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart" class="tsd-signature-type">TSciChart</a>, options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmarkeroptions.html" class="tsd-signature-type">IPointMarkerOptions</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/ellipsepointmarker.html" class="tsd-signature-type">EllipsePointMarker</a>

<!-- -->

- Creates an instance of the [EllipsePointMarker](https://www.scichart.com/documentation/js/v4/typedoc/classes/ellipsepointmarker.html)

  #### Parameters

  - ##### webAssemblyContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart" class="tsd-signature-type">TSciChart</a>

    The [SciChart 2D WebAssembly Context](https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart) containing native methods and access to our WebGL2 Engine and WebAssembly numerical methods

  - ##### Optional options: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmarkeroptions.html" class="tsd-signature-type">IPointMarkerOptions</a>

    Optional parameters of type [IPointMarkerOptions](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmarkeroptions.html) used to configure the point-marker at instantiation time

    ------------------------------------------------------------------------

    ðŸ“š Docs: <a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/common-series-apis/drawing-point-markers/" class="external">https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/common-series-apis/drawing-point-markers/</a>

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/ellipsepointmarker.html" class="tsd-signature-type">EllipsePointMarker</a>

## Properties

### invalidateParentCallback

invalidateParentCallback: () =\> void

Callback to invalidate the parent 2D [SciChartSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html)

#### Type declaration

- - (): void

  <!-- -->

  - #### Returns void

### Readonly type

type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/epointmarkertype.html#ellipse" class="tsd-signature-type">Ellipse</a> = EPointMarkerType.Ellipse

Type of the point marker

### Protected webAssemblyContext

webAssemblyContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart" class="tsd-signature-type">TSciChart</a>

The [SciChart 2D WebAssembly Context](https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart) containing native methods and access to our WebGL2 Engine and WebAssembly numerical methods

## Accessors

### antiAlias

- get antiAlias(): boolean
- set antiAlias(antiAlias: boolean): void

<!-- -->

- Gets or sets if anti-aliasing should be applied to the pointmarker. Default true.

  #### Returns boolean

- Gets or sets if anti-aliasing should be applied to the pointmarker. Default true.

  #### Parameters

  - ##### antiAlias: boolean

  #### Returns void

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

### height

- get height(): number
- set height(height: number): void

<!-- -->

- Gets or sets the height of the point-marker in pixels

  #### Returns number

- Gets or sets the height of the point-marker in pixels

  #### Parameters

  - ##### height: number

  #### Returns void

### lastPointOnly

- get lastPointOnly(): boolean
- set lastPointOnly(lastPointOnly: boolean): void

<!-- -->

- Set true to make the point marker render only for the last point on the data series

  #### Returns boolean

- Set true to make the point marker render only for the last point on the data series

  #### Parameters

  - ##### lastPointOnly: boolean

  #### Returns void

### opacity

- get opacity(): number
- set opacity(opacity: number): void

<!-- -->

- Gets or sets the opacity of the point-marker

  #### Returns number

- Gets or sets the opacity of the point-marker

  #### Parameters

  - ##### opacity: number

  #### Returns void

### stroke

- get stroke(): string
- set stroke(stroke: string): void

<!-- -->

- Gets or sets the point-marker stroke as an HTML Color Code

  #### Returns string

- Gets or sets the point-marker stroke as an HTML Color Code

  #### Parameters

  - ##### stroke: string

  #### Returns void

### strokeThickness

- get strokeThickness(): number
- set strokeThickness(strokeThickness: number): void

<!-- -->

- Gets or sets the stroke-thickness of the point-marker in pixels

  #### Returns number

- Gets or sets the stroke-thickness of the point-marker in pixels

  #### Parameters

  - ##### strokeThickness: number

  #### Returns void

### width

- get width(): number
- set width(width: number): void

<!-- -->

- Gets or sets the width of the point-marker in pixels

  #### Returns number

- Gets or sets the width of the point-marker in pixels

  #### Parameters

  - ##### width: number

  #### Returns void

## Methods

### adjustAutoColor

- adjustAutoColor(propertyName: string, color: string): string

<!-- -->

- Replace this to do custom adjustments to the auto color for a particular property

  #### Parameters

  - ##### propertyName: string

  - ##### color: string

  #### Returns string

### createCanvasTexture

- createCanvasTexture(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ispritetextures.html" class="tsd-signature-type">ISpriteTextures</a>

<!-- -->

- Called internally - creates the [CanvasTexture](https://www.scichart.com/documentation/js/v4/typedoc/classes/canvastexture.html) object and calls [drawSprite](https://www.scichart.com/documentation/js/v4/typedoc/classes/ellipsepointmarker.html#drawsprite) for creating the cached texture to draw

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ispritetextures.html" class="tsd-signature-type">ISpriteTextures</a>

### delete

- delete(): void

<!-- -->

- Deletes native (WebAssembly) memory used by this type, after which it cannot be used.

  remarks  
  Call .delete() before finishing with the object to ensure that WebAssmembly memory leaks do not occur.

  All elements within SciChart's High Performance <a href="https://www.scichart.com/javascript-chart-features" class="external">Realtime JavaScript Charts</a> which implement [IDeletable](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html) must be deleted manually to free native (WebAssembly) memory

  #### Returns void

### drawSprite

- drawSprite(context: CanvasRenderingContext2D, spriteWidth: number, spriteHeight: number, stroke: string, strokeThickness: number, fill: string): void

<!-- -->

- When overridden in a derived class, draw once the point-marker to the {@link CanvasRenderingContext2D}. This will create a sprite (image or bitmap) which will be repeated at each xy data-value using our fast WebGL WebAssembly graphics engine

  #### Parameters

  - ##### context: CanvasRenderingContext2D

    the {@link CanvasRenderingContext2D} to draw to

  - ##### spriteWidth: number

    the sprite target width

  - ##### spriteHeight: number

    the sprite target height

  - ##### stroke: string

    the stroke color

  - ##### strokeThickness: number

    the thickness of the stroke

  - ##### fill: string

    the fill color

  #### Returns void

### getFillMask

- getFillMask(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/canvastexture.html" class="tsd-signature-type">CanvasTexture</a>

<!-- -->

- Gets a [CanvasTexture](https://www.scichart.com/documentation/js/v4/typedoc/classes/canvastexture.html) object which represents the fill mask sprite instance to use for points, which appearance is overridden by a Palette Provider

  remarks  
  note [CanvasTexture](https://www.scichart.com/documentation/js/v4/typedoc/classes/canvastexture.html) implements [IDeletable](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html) and must be deleted manually to free memory

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/canvastexture.html" class="tsd-signature-type">CanvasTexture</a>

### getPointMarkerStyle

- getPointMarkerStyle(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basepointmarkerstyle.html" class="tsd-signature-type">BasePointMarkerStyle</a>

<!-- -->

- Creates the [BasePointMarkerStyle](https://www.scichart.com/documentation/js/v4/typedoc/classes/basepointmarkerstyle.html) object from the {@BasePointMarker}

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basepointmarkerstyle.html" class="tsd-signature-type">BasePointMarkerStyle</a>

### getSprite

- getSprite(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/canvastexture.html" class="tsd-signature-type">CanvasTexture</a>

<!-- -->

- Gets a [CanvasTexture](https://www.scichart.com/documentation/js/v4/typedoc/classes/canvastexture.html) object which represents the point-marker sprite instance to draw

  remarks  
  note [CanvasTexture](https://www.scichart.com/documentation/js/v4/typedoc/classes/canvastexture.html) implements [IDeletable](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html) and must be deleted manually to free memory

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/canvastexture.html" class="tsd-signature-type">CanvasTexture</a>

### getStrokeMask

- getStrokeMask(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/canvastexture.html" class="tsd-signature-type">CanvasTexture</a>

<!-- -->

- Gets a [CanvasTexture](https://www.scichart.com/documentation/js/v4/typedoc/classes/canvastexture.html) object which represents the stroke mask sprite instance to use for points, which appearance is overridden by a Palette Provider

  remarks  
  note [CanvasTexture](https://www.scichart.com/documentation/js/v4/typedoc/classes/canvastexture.html) implements [IDeletable](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html) and must be deleted manually to free memory

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/canvastexture.html" class="tsd-signature-type">CanvasTexture</a>

### invalidateCache

- invalidateCache(): void

<!-- -->

- #### Returns void

### Protected notifyPropertyChanged

- notifyPropertyChanged\<PropertyType\>(propertyName: string, newValue: PropertyType, oldValue: PropertyType): void

<!-- -->

- Notifies listeners to [invalidateParentCallback](https://www.scichart.com/documentation/js/v4/typedoc/classes/ellipsepointmarker.html#invalidateparentcallback) that a property has changed and redraw is required

  #### Type parameters

  - #### PropertyType

  #### Parameters

  - ##### propertyName: string

    the property name

  - ##### newValue: PropertyType

    the new value

  - ##### oldValue: PropertyType

    the old value

  #### Returns void

### onDpiChanged

- onDpiChanged(args: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tdpichangedeventargs" class="tsd-signature-type">TDpiChangedEventArgs</a>): void

<!-- -->

- Called when the Dpi changes in the browser. This could be due to user zooming the browser, or changing DPI settings in Windows, or moving the browser containing SciChart to another monitor

  #### Parameters

  - ##### args: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tdpichangedeventargs" class="tsd-signature-type">TDpiChangedEventArgs</a>

    The [TDpiChangedEventArgs](https://www.scichart.com/documentation/js/v4/typedoc/index.html#tdpichangedeventargs) containing info about the Dpi Changed event

  #### Returns void

### resetCache

- resetCache(): void

<!-- -->

- #### Returns void

### resolveAutoColors

- resolveAutoColors(index: number, maxSeries: number, theme: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html" class="tsd-signature-type">IThemeProvider</a>): void

<!-- -->

- Resolve colors marked AUTO_COLOR using the theme's strokePalette and fillPalette To do custom adjustments to the resolved colors, override the adjustAutoColor method

  #### Parameters

  - ##### index: number

  - ##### maxSeries: number

  - ##### theme: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html" class="tsd-signature-type">IThemeProvider</a>

  #### Returns void

### resumeUpdates

- resumeUpdates(): void

<!-- -->

- Resumes recreation of the PointMarker

  #### Returns void

### suspendUpdates

- suspendUpdates(): void

<!-- -->

- Suspends recreation of the PointMarker, used to increase performance when several properties of the PointMarker needs to be updated

  #### Returns void

### toJSON

- toJSON(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tpointmarkerdefinition" class="tsd-signature-type">TPointMarkerDefinition</a>

<!-- -->

- Convert the object to a definition that can be serialized to JSON, or used directly with the builder api

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tpointmarkerdefinition" class="tsd-signature-type">TPointMarkerDefinition</a>

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
