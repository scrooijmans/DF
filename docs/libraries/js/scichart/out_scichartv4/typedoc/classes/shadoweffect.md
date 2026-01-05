<img src="out_scichartv4/typedoc/classes/shadoweffect_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/shadoweffect.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [ShadowEffect](https://www.scichart.com/documentation/js/v4/typedoc/classes/shadoweffect.html)

# Class ShadowEffect

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

Defines a drop-shadow shader effect that may be applied to a [RenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries.html)

remarks  
To apply a Shadow effect to a chart series, use the following code:

``` ts
const effect = new ShadowEffect(wasmContext, {
  range: 1,
  brightness: 100,
  offset: new Point(10, 10)
});
const lineSeries = new FastLineRenderableSeries(wasmContext, { stroke, effect });
```

Note that effects implement [IDeletable](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html) and must be manually deleted to free webassembly / native memory

### Hierarchy

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/shadereffect.html" class="tsd-signature-type">ShaderEffect</a>
  - ShadowEffect

### Implements

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html" class="tsd-signature-type">IDeletable</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html" class="tsd-signature-type">IDeletable</a>

## Index

### Constructors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/shadoweffect.html#constructor" class="tsd-kind-icon">constructor</a>

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/shadoweffect.html#propertychanged" class="tsd-kind-icon">propertyChanged</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/shadoweffect.html#type" class="tsd-kind-icon">type</a>

### Accessors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/shadoweffect.html#color" class="tsd-kind-icon">color</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/shadoweffect.html#intensity" class="tsd-kind-icon">intensity</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/shadoweffect.html#offset" class="tsd-kind-icon">offset</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/shadoweffect.html#range" class="tsd-kind-icon">range</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/shadoweffect.html#delete" class="tsd-kind-icon">delete</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/shadoweffect.html#getnativeeffect" class="tsd-kind-icon">getNativeEffect</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/shadoweffect.html#notifypropertychanged" class="tsd-kind-icon">notifyPropertyChanged</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/shadoweffect.html#tojson" class="tsd-kind-icon">toJSON</a>

## Constructors

### constructor

- new ShadowEffect(webAssemblyContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart" class="tsd-signature-type">TSciChart</a>, options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ishadoweffectoptions.html" class="tsd-signature-type">IShadowEffectOptions</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/shadoweffect.html" class="tsd-signature-type">ShadowEffect</a>

<!-- -->

- Creates an instance of the [ShadowEffect](https://www.scichart.com/documentation/js/v4/typedoc/classes/shadoweffect.html)

  #### Parameters

  - ##### webAssemblyContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart" class="tsd-signature-type">TSciChart</a>

    The [SciChart WebAssembly Context](https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart) containing native methods and access to our WebGL2 WebAssembly Drawing Engine

  - ##### Optional options: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ishadoweffectoptions.html" class="tsd-signature-type">IShadowEffectOptions</a>

    Optional parameters of type [IShaderEffectOptions](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ishadereffectoptions.html) to configure the effect

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/shadoweffect.html" class="tsd-signature-type">ShadowEffect</a>

## Properties

### propertyChanged

propertyChanged: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/eventhandler.html" class="tsd-signature-type">EventHandler</a>\<<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/propertychangedeventargs.html" class="tsd-signature-type">PropertyChangedEventArgs</a>\>

Event handler for when properties change, signalling that the parent [SciChartSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html) needs to redraw

### Readonly type

type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eshadereffecttype.html#shadow" class="tsd-signature-type">Shadow</a> = EShaderEffectType.Shadow

## Accessors

### color

- get color(): string
- set color(color: string): void

<!-- -->

- Gets or sets the color property as an HTML Color code

  #### Returns string

- Gets or sets the color property as an HTML Color code

  #### Parameters

  - ##### color: string

  #### Returns void

### intensity

- get intensity(): number
- set intensity(intensity: number): void

<!-- -->

- Gets or sets the intensity property

  #### Returns number

- Gets or sets the intensity property

  #### Parameters

  - ##### intensity: number

  #### Returns void

### offset

- get offset(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/point.html" class="tsd-signature-type">Point</a>
- set offset(offset: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/point.html" class="tsd-signature-type">Point</a>): void

<!-- -->

- Gets or sets the offset property

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/point.html" class="tsd-signature-type">Point</a>

- Gets or sets the offset property

  #### Parameters

  - ##### offset: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/point.html" class="tsd-signature-type">Point</a>

  #### Returns void

### range

- get range(): number
- set range(range: number): void

<!-- -->

- Gets or sets the range property

  #### Returns number

- Gets or sets the range property

  #### Parameters

  - ##### range: number

  #### Returns void

## Methods

### delete

- delete(): void

<!-- -->

- #### Returns void

### getNativeEffect

- getNativeEffect(): SCRTSeriesEffect

<!-- -->

- Gets the native {@link SCRTSeriesEffect} which contains the shader effect that will be applied in WebGL

  #### Returns SCRTSeriesEffect

### notifyPropertyChanged

- notifyPropertyChanged(propertyName: string): void

<!-- -->

- Notifies subscribers to [propertyChanged](https://www.scichart.com/documentation/js/v4/typedoc/classes/shadoweffect.html#propertychanged) that a property has changed and the parent [SciChartSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html) needs to redraw

  #### Parameters

  - ##### propertyName: string

  #### Returns void

### toJSON

- toJSON(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#teffectdefinition" class="tsd-signature-type">TEffectDefinition</a>

<!-- -->

- Convert the object to a definition that can be serialized to JSON, or used directly with the builder api

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#teffectdefinition" class="tsd-signature-type">TEffectDefinition</a>

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
