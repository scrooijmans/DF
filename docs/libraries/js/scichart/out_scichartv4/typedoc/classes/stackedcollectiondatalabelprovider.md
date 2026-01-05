<img src="out_scichartv4/typedoc/classes/stackedcollectiondatalabelprovider_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedcollectiondatalabelprovider.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [StackedCollectionDataLabelProvider](https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedcollectiondatalabelprovider.html)

# Class StackedCollectionDataLabelProvider

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

### Hierarchy

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basedatalabelprovider.html" class="tsd-signature-type">BaseDataLabelProvider</a>
  - StackedCollectionDataLabelProvider

## Index

### Constructors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedcollectiondatalabelprovider.html#constructor" class="tsd-kind-icon">constructor</a>

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedcollectiondatalabelprovider.html#colorproperty" class="tsd-kind-icon">colorProperty</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedcollectiondatalabelprovider.html#datalabels" class="tsd-kind-icon">dataLabels</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedcollectiondatalabelprovider.html#drawimmediate" class="tsd-kind-icon">drawImmediate</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedcollectiondatalabelprovider.html#parentseries" class="tsd-kind-icon">parentSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedcollectiondatalabelprovider.html#styleproperty" class="tsd-kind-icon">styleProperty</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedcollectiondatalabelprovider.html#type" class="tsd-kind-icon">type</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedcollectiondatalabelprovider.html#webassemblycontext" class="tsd-kind-icon">webAssemblyContext</a>

### Accessors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedcollectiondatalabelprovider.html#color" class="tsd-kind-icon">color</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedcollectiondatalabelprovider.html#style" class="tsd-kind-icon">style</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedcollectiondatalabelprovider.html#userotation" class="tsd-kind-icon">useRotation</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedcollectiondatalabelprovider.html#delete" class="tsd-kind-icon">delete</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedcollectiondatalabelprovider.html#draw" class="tsd-kind-icon">draw</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedcollectiondatalabelprovider.html#generatedatalabels" class="tsd-kind-icon">generateDataLabels</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedcollectiondatalabelprovider.html#invalidateparent" class="tsd-kind-icon">invalidateParent</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedcollectiondatalabelprovider.html#onattach" class="tsd-kind-icon">onAttach</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedcollectiondatalabelprovider.html#ondetach" class="tsd-kind-icon">onDetach</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedcollectiondatalabelprovider.html#resolveautocolors" class="tsd-kind-icon">resolveAutoColors</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedcollectiondatalabelprovider.html#tojson" class="tsd-kind-icon">toJSON</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedcollectiondatalabelprovider.html#updatestyle" class="tsd-kind-icon">updateStyle</a>

## Constructors

### constructor

- new StackedCollectionDataLabelProvider(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedcollectiondatalabelprovider.html" class="tsd-signature-type">StackedCollectionDataLabelProvider</a>

<!-- -->

- #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedcollectiondatalabelprovider.html" class="tsd-signature-type">StackedCollectionDataLabelProvider</a>

## Properties

### Protected colorProperty

colorProperty: string

### dataLabels

dataLabels: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tdatalabel" class="tsd-signature-type">TDataLabel</a>\[\] = \[\]

The text to draw, along with the sizes and positions. Usually generated, but can be updated before final drawing

### drawImmediate

drawImmediate: boolean = false

Draws immediately, set this True when using sub-surfaces for proper data label layering

### Protected parentSeries

parentSeries: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries.html" class="tsd-signature-type">IRenderableSeries</a>

The Parent [RenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries.html)

### Protected styleProperty

styleProperty: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tdatalabelstyle" class="tsd-signature-type">TDataLabelStyle</a>

### Readonly type

type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/edatalabelprovidertype.html" class="tsd-signature-type">EDataLabelProviderType</a>

### Protected webAssemblyContext

webAssemblyContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart" class="tsd-signature-type">TSciChart</a>

The [SciChart 2D WebAssembly Context](https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart) containing native methods and access to our WebGL2 Engine and WebAssembly numerical methods

## Accessors

### color

- get color(): string
- set color(value: string): void

<!-- -->

- Gets or sets the color for data labels. Defaults to axis label color

  #### Returns string

- Gets or sets the color for data labels. Defaults to axis label color

  #### Parameters

  - ##### value: string

  #### Returns void

### style

- get style(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tdatalabelstyle" class="tsd-signature-type">TDataLabelStyle</a>
- set style(value: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tdatalabelstyle" class="tsd-signature-type">TDataLabelStyle</a>): void

<!-- -->

- Gets or sets the text style used for data labels. The style must be set, with fontFamily and fontSize set, in order for text to be drawn.

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tdatalabelstyle" class="tsd-signature-type">TDataLabelStyle</a>

- Gets or sets the text style used for data labels. The style must be set, with fontFamily and fontSize set, in order for text to be drawn.

  #### Parameters

  - ##### value: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tdatalabelstyle" class="tsd-signature-type">TDataLabelStyle</a>

  #### Returns void

### Protected useRotation

- get useRotation(): boolean

<!-- -->

- #### Returns boolean

## Methods

### delete

- delete(): void

<!-- -->

- #### Returns void

### draw

- draw(renderContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/webglrendercontext2d.html" class="tsd-signature-type">WebGlRenderContext2D</a>): void

<!-- -->

- #### Parameters

  - ##### renderContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/webglrendercontext2d.html" class="tsd-signature-type">WebGlRenderContext2D</a>

  #### Returns void

### generateDataLabels

- generateDataLabels(renderContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/webglrendercontext2d.html" class="tsd-signature-type">WebGlRenderContext2D</a>, renderPassData: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassdata.html" class="tsd-signature-type">RenderPassData</a>): void

<!-- -->

- #### Parameters

  - ##### renderContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/webglrendercontext2d.html" class="tsd-signature-type">WebGlRenderContext2D</a>

  - ##### renderPassData: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassdata.html" class="tsd-signature-type">RenderPassData</a>

  #### Returns void

### Protected invalidateParent

- invalidateParent(): void

<!-- -->

- #### Returns void

### onAttach

- onAttach(webAssemblyContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart" class="tsd-signature-type">TSciChart</a>, parentSeries: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries.html" class="tsd-signature-type">IRenderableSeries</a>): void

<!-- -->

- Called when a DataLabelProvider is attached to a parent [RenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries.html)

  #### Parameters

  - ##### webAssemblyContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart" class="tsd-signature-type">TSciChart</a>

  - ##### parentSeries: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries.html" class="tsd-signature-type">IRenderableSeries</a>

  #### Returns void

### onDetach

- onDetach(): void

<!-- -->

- #### Returns void

### resolveAutoColors

- resolveAutoColors(index: number, maxSeries: number, theme: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html" class="tsd-signature-type">IThemeProvider</a>): void

<!-- -->

- #### Parameters

  - ##### index: number

  - ##### maxSeries: number

  - ##### theme: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html" class="tsd-signature-type">IThemeProvider</a>

  #### Returns void

### toJSON

- toJSON(): { type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/edatalabelprovidertype.html" class="tsd-signature-type">EDataLabelProviderType</a>; options: object }

<!-- -->

- #### Returns { type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/edatalabelprovidertype.html" class="tsd-signature-type">EDataLabelProviderType</a>; options: object }

  - ##### type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/edatalabelprovidertype.html" class="tsd-signature-type">EDataLabelProviderType</a>

  - ##### options: object

    - ##### color: string

    - ##### style: { fontFamily?: string; fontSize?: number; lineSpacing?: number; multiLineAlignment?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/emultilinealignment.html" class="tsd-signature-type">EMultiLineAlignment</a>; padding?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/thickness.html" class="tsd-signature-type">Thickness</a> }

      - ##### Optional fontFamily?: string

      - ##### Optional fontSize?: number

      - ##### Optional lineSpacing?: number

      - ##### Optional multiLineAlignment?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/emultilinealignment.html" class="tsd-signature-type">EMultiLineAlignment</a>

      - ##### Optional padding?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/thickness.html" class="tsd-signature-type">Thickness</a>

### updateStyle

- updateStyle(value: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tdatalabelstyle" class="tsd-signature-type">TDataLabelStyle</a>): void

<!-- -->

- Update the style. Only the properties passed will be updated

  #### Parameters

  - ##### value: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tdatalabelstyle" class="tsd-signature-type">TDataLabelStyle</a>

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
