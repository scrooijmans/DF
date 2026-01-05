<img src="out_scichartv4/typedoc/classes/canvastexture_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/canvastexture.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [CanvasTexture](https://www.scichart.com/documentation/js/v4/typedoc/classes/canvastexture.html)

# Class CanvasTexture

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

summary  
The [CanvasTexture](https://www.scichart.com/documentation/js/v4/typedoc/classes/canvastexture.html) is used internally to map an {@link HTMLCanvasElement} to a WebGL Texture. Use this when you want to create a WebGL texture and draw on it.

remarks  
To use a canvas texture, declare one, draw on the HTML canvas, then call [CanvasTexture.copyTexture](https://www.scichart.com/documentation/js/v4/typedoc/classes/canvastexture.html#copytexture). Code example below

``` ts
// Create a canvasTexture
const canvasTexture = new CanvasTexture(wasmContext, width, height);
canvasTexture.clear();

// do some drawing with html5Context
const html5Context = canvasTexture.getContext();
// todo - your drawing here

// Copy and get the texture
canvasTexture.copyTexture();
const webAssemblyTexture: TSRTexture = canvasTexture.getTexture();

// After use, delete the CanvasTexture
canvasTexture.delete();
```

### Hierarchy

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/deletableentity.html" class="tsd-signature-type">DeletableEntity</a>
  - CanvasTexture

### Implements

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html" class="tsd-signature-type">IDeletable</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html" class="tsd-signature-type">IDeletable</a>

## Index

### Constructors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/canvastexture.html#constructor" class="tsd-kind-icon">constructor</a>

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/canvastexture.html#canvas" class="tsd-kind-icon">canvas</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/canvastexture.html#height" class="tsd-kind-icon">height</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/canvastexture.html#width" class="tsd-kind-icon">width</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/canvastexture.html#applyopacity" class="tsd-kind-icon">applyOpacity</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/canvastexture.html#clear" class="tsd-kind-icon">clear</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/canvastexture.html#copytexture" class="tsd-kind-icon">copyTexture</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/canvastexture.html#delete" class="tsd-kind-icon">delete</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/canvastexture.html#getcontext" class="tsd-kind-icon">getContext</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/canvastexture.html#gettexture" class="tsd-kind-icon">getTexture</a>

## Constructors

### constructor

- new CanvasTexture(webAssemblyContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart" class="tsd-signature-type">TSciChart</a> \| <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart3d" class="tsd-signature-type">TSciChart3D</a>, textureWidth: number, textureHeight: number): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/canvastexture.html" class="tsd-signature-type">CanvasTexture</a>

<!-- -->

- Creates an instance of a [CanvasTexture](https://www.scichart.com/documentation/js/v4/typedoc/classes/canvastexture.html)

  remarks  
  The [CanvasTexture](https://www.scichart.com/documentation/js/v4/typedoc/classes/canvastexture.html) implements [IDeletable](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html), and must be manually deleted to free WebAssembly / native memory

  #### Parameters

  - ##### webAssemblyContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart" class="tsd-signature-type">TSciChart</a> \| <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart3d" class="tsd-signature-type">TSciChart3D</a>

    The [SciChart 2D WebAssembly Context](https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart) or {@link TSciChart2D \| SciChart 2D WebAssembly Context} containing native methods and access to our WebGL2 Engine and WebAssembly numerical methods

  - ##### textureWidth: number

    The width of the texture

  - ##### textureHeight: number

    The height of the texture

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/canvastexture.html" class="tsd-signature-type">CanvasTexture</a>

## Properties

### canvas

canvas: HTMLCanvasElement

### Readonly height

height: number

### Readonly width

width: number

## Methods

### applyOpacity

- applyOpacity(opacity: number): void

<!-- -->

- #### Parameters

  - ##### opacity: number

  #### Returns void

### clear

- clear(): void

<!-- -->

- Clears the texture and the canvas

  #### Returns void

### copyTexture

- copyTexture(): void

<!-- -->

- After you have finished drawing, copy the canvas to the destination {@link TSRTexture}

  #### Returns void

### delete

- delete(): void

<!-- -->

- Deletes native (WebAssembly) memory used by this type, after which it cannot be used.

  remarks  
  Call .delete() before finishing with the object to ensure that WebAssmembly memory leaks do not occur.

  All elements within SciChart's High Performance <a href="https://www.scichart.com/javascript-chart-features" class="external">Realtime JavaScript Charts</a> which implement [IDeletable](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html) must be deleted manually to free native (WebAssembly) memory

  #### Returns void

### getContext

- getContext(): CanvasRenderingContext2D

<!-- -->

- Get an HTML5 {@link CanvasRenderingContext2D} to draw on.

  #### Returns CanvasRenderingContext2D

### getTexture

- getTexture(): TSRTexture

<!-- -->

- Get the SciChart WebAssembly / WebGL {@link TSRTexture \| Texture}

  #### Returns TSRTexture

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
