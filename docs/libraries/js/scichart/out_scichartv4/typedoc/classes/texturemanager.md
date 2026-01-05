<img src="out_scichartv4/typedoc/classes/texturemanager_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/texturemanager.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [TextureManager](https://www.scichart.com/documentation/js/v4/typedoc/classes/texturemanager.html)

# Class TextureManager

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

### Hierarchy

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/deletableentity.html" class="tsd-signature-type">DeletableEntity</a>
  - TextureManager

### Implements

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html" class="tsd-signature-type">IDeletable</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html" class="tsd-signature-type">IDeletable</a>

## Index

### Constructors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/texturemanager.html#constructor" class="tsd-kind-icon">constructor</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/texturemanager.html#createaxismarkertexture" class="tsd-kind-icon">createAxisMarkerTexture</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/texturemanager.html#createsimpletexttexture" class="tsd-kind-icon">createSimpleTextTexture</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/texturemanager.html#createtexttexture" class="tsd-kind-icon">createTextTexture</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/texturemanager.html#createtexttexturenative" class="tsd-kind-icon">createTextTextureNative</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/texturemanager.html#createtexturefromctxbuffer" class="tsd-kind-icon">createTextureFromCtxBuffer</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/texturemanager.html#createtexturefromimage" class="tsd-kind-icon">createTextureFromImage</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/texturemanager.html#createtexturefromimagedata" class="tsd-kind-icon">createTextureFromImageData</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/texturemanager.html#delete" class="tsd-kind-icon">delete</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/texturemanager.html#gettexturecontext" class="tsd-kind-icon">getTextureContext</a>

## Constructors

### constructor

- new TextureManager(webAssemblyContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart" class="tsd-signature-type">TSciChart</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/texturemanager.html" class="tsd-signature-type">TextureManager</a>

<!-- -->

- #### Parameters

  - ##### webAssemblyContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart" class="tsd-signature-type">TSciChart</a>

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/texturemanager.html" class="tsd-signature-type">TextureManager</a>

## Methods

### createAxisMarkerTexture

- createAxisMarkerTexture(axisAlignment: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eaxisalignment.html" class="tsd-signature-type">EAxisAlignment</a>, text: string, fontStyle: string, fontWeight: string, fontSizePx: number, fontFamily: string, color: string, padding?: number, backgroundColor?: string, opacity?: number): TTextureObject

<!-- -->

- #### Parameters

  - ##### axisAlignment: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eaxisalignment.html" class="tsd-signature-type">EAxisAlignment</a>

  - ##### text: string

  - ##### fontStyle: string

  - ##### fontWeight: string

  - ##### fontSizePx: number

  - ##### fontFamily: string

  - ##### color: string

  - ##### Default value padding: number = 0

  - ##### Optional backgroundColor: string

  - ##### Optional opacity: number

  #### Returns TTextureObject

### createSimpleTextTexture

- createSimpleTextTexture(text: string, textStyle: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#ttextstyle" class="tsd-signature-type">TTextStyle</a>, backgroundColor?: string, displayVertically?: boolean, displayMirrored?: boolean, opacity?: number): { bitmapTexture: TSRTexture; textureHeight: number; textureWidth: number }

<!-- -->

- #### Parameters

  - ##### text: string

  - ##### textStyle: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#ttextstyle" class="tsd-signature-type">TTextStyle</a>

  - ##### Optional backgroundColor: string

  - ##### Optional displayVertically: boolean

  - ##### Optional displayMirrored: boolean

  - ##### Optional opacity: number

  #### Returns { bitmapTexture: TSRTexture; textureHeight: number; textureWidth: number }

  - ##### bitmapTexture: TSRTexture

  - ##### textureHeight: number

  - ##### textureWidth: number

### createTextTexture

- createTextTexture(text: string\[\], textStyle: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#ttextstyle" class="tsd-signature-type">TTextStyle</a>, rotation?: number, lineSpacing?: number, backgroundColor?: string, opacity?: number): TTextureObject

<!-- -->

- Create a text texture supporting multiline and arbitrary rotation

  #### Parameters

  - ##### text: string\[\]

  - ##### textStyle: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#ttextstyle" class="tsd-signature-type">TTextStyle</a>

  - ##### Optional rotation: number

  - ##### Optional lineSpacing: number

    Expressed as a fraction of the font size. Default 1.1

  - ##### Optional backgroundColor: string

  - ##### Optional opacity: number

  #### Returns TTextureObject

### createTextTextureNative

- createTextTextureNative(text: string\[\], textStyle: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#ttextstyle" class="tsd-signature-type">TTextStyle</a>, rotation?: number, lineSpacing?: number, backgroundColor?: string, opacity?: number): TTextureObject

<!-- -->

- Create a text texture supporting multiline and arbitrary rotation

  #### Parameters

  - ##### text: string\[\]

  - ##### textStyle: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#ttextstyle" class="tsd-signature-type">TTextStyle</a>

  - ##### Optional rotation: number

  - ##### Optional lineSpacing: number

    Expressed as a fraction of the font size. Default 1.1

  - ##### Optional backgroundColor: string

  - ##### Optional opacity: number

  #### Returns TTextureObject

### createTextureFromCtxBuffer

- createTextureFromCtxBuffer(textureWidth: number, textureHeight: number): TTextureObject

<!-- -->

- #### Parameters

  - ##### textureWidth: number

  - ##### textureHeight: number

  #### Returns TTextureObject

### createTextureFromImage

- createTextureFromImage(image: HTMLImageElement, imageWidth: number, imageHeight: number): TTextureObject

<!-- -->

- Creates {@link TSRTexture} from image

  #### Parameters

  - ##### image: HTMLImageElement

    The image

  - ##### imageWidth: number

    The image width, not premultiplied value

  - ##### imageHeight: number

    The image height, not premultiplied value

  #### Returns TTextureObject

### createTextureFromImageData

- createTextureFromImageData(imageData: ImageData, textureWidth: number, textureHeight: number): TTextureObject

<!-- -->

- #### Parameters

  - ##### imageData: ImageData

  - ##### textureWidth: number

  - ##### textureHeight: number

  #### Returns TTextureObject

### delete

- delete(): void

<!-- -->

- #### Returns void

### getTextureContext

- getTextureContext(width: number, height: number): CanvasRenderingContext2D

<!-- -->

- #### Parameters

  - ##### width: number

  - ##### height: number

  #### Returns CanvasRenderingContext2D

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
