<img src="out_scichartv4/typedoc/classes/radianlabelprovider_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/radianlabelprovider.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [RadianLabelProvider](https://www.scichart.com/documentation/js/v4/typedoc/classes/radianlabelprovider.html)

# Class RadianLabelProvider

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

The [RadianLabelProvider](https://www.scichart.com/documentation/js/v4/typedoc/classes/radianlabelprovider.html) formats Axis Labels and Cursor / Tooltips for [NumericAxis](https://www.scichart.com/documentation/js/v4/typedoc/enums/eaxistype.html#numericaxis) types as multiples/fractions of PI

note  
It is highly recommended to use this label provider with these axis options:

``` ts
const xAxis = new NumericAxis(wasmContext, {
         labelProvider: new RadianLabelProvider({
            maxDenominator: 4, // or any other non-zero integer
            errorTolerance: 0.0001,

            labelPrecision: 2
             // for values that cannot be expressed as fractions of PI within `maxDenominator` constraint,
             // thus default to decimal format
         }),
         autoTicks: false, // to manually set tick distance
         majorDelta: Math.PI / 4, // or any other PI fraction
});
```

### Hierarchy

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numericlabelprovider.html" class="tsd-signature-type">NumericLabelProvider</a>
  - RadianLabelProvider

### Implements

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html" class="tsd-signature-type">IDeletable</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icacheable.html" class="tsd-signature-type">ICacheable</a>

## Index

### Constructors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/radianlabelprovider.html#constructor" class="tsd-kind-icon">constructor</a>

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/radianlabelprovider.html#formatcursorlabelproperty" class="tsd-kind-icon">formatCursorLabelProperty</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/radianlabelprovider.html#formatlabelproperty" class="tsd-kind-icon">formatLabelProperty</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/radianlabelprovider.html#parentaxis" class="tsd-kind-icon">parentAxis</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/radianlabelprovider.html#providerid" class="tsd-kind-icon">providerId</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/radianlabelprovider.html#styleid" class="tsd-kind-icon">styleId</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/radianlabelprovider.html#textvariesforsametick" class="tsd-kind-icon">textVariesForSameTick</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/radianlabelprovider.html#ticktotext" class="tsd-kind-icon">tickToText</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/radianlabelprovider.html#type" class="tsd-kind-icon">type</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/radianlabelprovider.html#usecache" class="tsd-kind-icon">useCache</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/radianlabelprovider.html#usenativetext" class="tsd-kind-icon">useNativeText</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/radianlabelprovider.html#usesharedcache" class="tsd-kind-icon">useSharedCache</a>

### Accessors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/radianlabelprovider.html#cursornumericformat" class="tsd-kind-icon">cursorNumericFormat</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/radianlabelprovider.html#cursorprecision" class="tsd-kind-icon">cursorPrecision</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/radianlabelprovider.html#engineeringprefix" class="tsd-kind-icon">engineeringPrefix</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/radianlabelprovider.html#errortolerance" class="tsd-kind-icon">errorTolerance</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/radianlabelprovider.html#formatcursorlabel" class="tsd-kind-icon">formatCursorLabel</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/radianlabelprovider.html#formatlabel" class="tsd-kind-icon">formatLabel</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/radianlabelprovider.html#linespacing" class="tsd-kind-icon">lineSpacing</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/radianlabelprovider.html#maxdenominator" class="tsd-kind-icon">maxDenominator</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/radianlabelprovider.html#numericformat" class="tsd-kind-icon">numericFormat</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/radianlabelprovider.html#postfix" class="tsd-kind-icon">postfix</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/radianlabelprovider.html#precision" class="tsd-kind-icon">precision</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/radianlabelprovider.html#prefix" class="tsd-kind-icon">prefix</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/radianlabelprovider.html#rotation" class="tsd-kind-icon">rotation</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/radianlabelprovider.html#adjustlabel" class="tsd-kind-icon">adjustLabel</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/radianlabelprovider.html#applyformat" class="tsd-kind-icon">applyFormat</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/radianlabelprovider.html#attachedtoaxis" class="tsd-kind-icon">attachedToAxis</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/radianlabelprovider.html#clearcache" class="tsd-kind-icon">clearCache</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/radianlabelprovider.html#delete" class="tsd-kind-icon">delete</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/radianlabelprovider.html#detachedfromaxis" class="tsd-kind-icon">detachedFromAxis</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/radianlabelprovider.html#getcachedlabeltexture" class="tsd-kind-icon">getCachedLabelTexture</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/radianlabelprovider.html#getcachedstyle" class="tsd-kind-icon">getCachedStyle</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/radianlabelprovider.html#getlabelheight" class="tsd-kind-icon">getLabelHeight</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/radianlabelprovider.html#getlabelsizesnative" class="tsd-kind-icon">getLabelSizesNative</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/radianlabelprovider.html#getlabeltexture" class="tsd-kind-icon">getLabelTexture</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/radianlabelprovider.html#getlabeltextureasync" class="tsd-kind-icon">getLabelTextureAsync</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/radianlabelprovider.html#getlabelwidth" class="tsd-kind-icon">getLabelWidth</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/radianlabelprovider.html#getlabels" class="tsd-kind-icon">getLabels</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/radianlabelprovider.html#getmaxlabelheightforhorizontalaxis" class="tsd-kind-icon">getMaxLabelHeightForHorizontalAxis</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/radianlabelprovider.html#getmaxlabelwidthforverticalaxis" class="tsd-kind-icon">getMaxLabelWidthForVerticalAxis</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/radianlabelprovider.html#getnativelabelinfo" class="tsd-kind-icon">getNativeLabelInfo</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/radianlabelprovider.html#invalidatecache" class="tsd-kind-icon">invalidateCache</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/radianlabelprovider.html#invalidateparent" class="tsd-kind-icon">invalidateParent</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/radianlabelprovider.html#onbeginaxisdraw" class="tsd-kind-icon">onBeginAxisDraw</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/radianlabelprovider.html#pruneticktextcache" class="tsd-kind-icon">pruneTickTextCache</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/radianlabelprovider.html#resetcache" class="tsd-kind-icon">resetCache</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/radianlabelprovider.html#tojson" class="tsd-kind-icon">toJSON</a>

## Constructors

### constructor

- new RadianLabelProvider(options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iradianlabelprovideroptions.html" class="tsd-signature-type">IRadianLabelProviderOptions</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/radianlabelprovider.html" class="tsd-signature-type">RadianLabelProvider</a>

<!-- -->

- #### Parameters

  - ##### Default value options: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iradianlabelprovideroptions.html" class="tsd-signature-type">IRadianLabelProviderOptions</a> = {}

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/radianlabelprovider.html" class="tsd-signature-type">RadianLabelProvider</a>

## Properties

### Protected formatCursorLabelProperty

formatCursorLabelProperty: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tformatlabelfn" class="tsd-signature-type">TFormatLabelFn</a>

### Protected formatLabelProperty

formatLabelProperty: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tformatlabelfn" class="tsd-signature-type">TFormatLabelFn</a>

### parentAxis

parentAxis: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html" class="tsd-signature-type">AxisCore</a>

The parent [AxisCore](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html). This will be set once [attachedToAxis](https://www.scichart.com/documentation/js/v4/typedoc/classes/radianlabelprovider.html#attachedtoaxis) is called

### Protected providerId

providerId: string = generateGuid()

### Protected styleId

styleId: string

This is the id for the text style used by this axis, as stored in the label cache. Cached labels are accessed by text and styleId. If you have useSharedCache = true and are overriding getLabelTexture or getLabelTextureAsync and do not ensure the style is unique, you might not get the labels you expect. You can either set useSharedCache = false, set this to some unique value, or override getCachedStyle

### textVariesForSameTick

textVariesForSameTick: boolean = false

Set this true if the format function could return different results for the same input (eg SmartDateLabelprovider)

### Protected tickToText

tickToText: Map\<number, string\> = new Map\<number, string\>()

### Readonly type

type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/elabelprovidertype.html#numeric" class="tsd-signature-type">Numeric</a> = ELabelProviderType.Numeric

### useCache

useCache: boolean = !IS_TEST_ENV

Used internally only for testing

### useNativeText

useNativeText: boolean

Whether to use WebGL for rendering axis labels. Default true (was false before v4). These are much faster than rendering using canvas text, but do not have quite the same font and style support.

Set [SciChartDefaults.useNativeText](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartdefaults.html#usenativetext) to change the global default.

### useSharedCache

useSharedCache: boolean

Whether to use cached labels from other axes that have the same style. You may need to set this false if you are overriding getLabelTexture without setting a unique style. Can be set globally using SciChartDefaults. Currently default false.

## Accessors

### cursorNumericFormat

- get cursorNumericFormat(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/enumericformat.html" class="tsd-signature-type">ENumericFormat</a>
- set cursorNumericFormat(value: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/enumericformat.html" class="tsd-signature-type">ENumericFormat</a>): void

<!-- -->

- Gets or sets numeric format to use for cursor labels. For a list of values, see [ENumericFormat](https://www.scichart.com/documentation/js/v4/typedoc/enums/enumericformat.html)

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/enumericformat.html" class="tsd-signature-type">ENumericFormat</a>

- Gets or sets numeric format to use for cursor labels. For a list of values, see [ENumericFormat](https://www.scichart.com/documentation/js/v4/typedoc/enums/enumericformat.html)

  #### Parameters

  - ##### value: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/enumericformat.html" class="tsd-signature-type">ENumericFormat</a>

  #### Returns void

### cursorPrecision

- get cursorPrecision(): number
- set cursorPrecision(value: number): void

<!-- -->

- Gets or sets the precision to use for cursors labels

  #### Returns number

- Gets or sets the precision to use for cursors labels

  #### Parameters

  - ##### value: number

  #### Returns void

### engineeringPrefix

- get engineeringPrefix(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iengineeringprefix.html" class="tsd-signature-type">IEngineeringPrefix</a>
- set engineeringPrefix(value: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iengineeringprefix.html" class="tsd-signature-type">IEngineeringPrefix</a>): void

<!-- -->

- Gets or sets the engineering prefixes to use when formatting values to text. Default - `['K','M','B,'T']` for "large" prefixes, `['m','u','n','p']` for small prefixes

  remarks  
  Only works when [ENumericFormat.Engineering](https://www.scichart.com/documentation/js/v4/typedoc/enums/enumericformat.html#engineering) is selected

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iengineeringprefix.html" class="tsd-signature-type">IEngineeringPrefix</a>

- Gets or sets the engineering prefixes to use when formatting values to text. Default - `['K','M','B,'T']` for "large" prefixes, `['m','u','n','p']` for small prefixes

  remarks  
  Only works when [ENumericFormat.Engineering](https://www.scichart.com/documentation/js/v4/typedoc/enums/enumericformat.html#engineering) is selected

  #### Parameters

  - ##### value: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iengineeringprefix.html" class="tsd-signature-type">IEngineeringPrefix</a>

  #### Returns void

### errorTolerance

- get errorTolerance(): number
- set errorTolerance(value: number): void

<!-- -->

- Gets or sets the maximum error tolerance when formatting values as fractions of PI

  #### Returns number

- Gets or sets the maximum error tolerance when formatting values as fractions of PI

  #### Parameters

  - ##### value: number

  #### Returns void

### formatCursorLabel

- get formatCursorLabel(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tformatlabelfn" class="tsd-signature-type">TFormatLabelFn</a>
- set formatCursorLabel(value: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tformatlabelfn" class="tsd-signature-type">TFormatLabelFn</a>): void

<!-- -->

- Gets or sets a formatCursorLabel function which is used for formatting a data-value into a string for display on a cursor or tooltip If you are creating a custom LabelProvider, you should override formatCursorLabelProperty, not the formatCursorLabel property! See our <a href="https://www.scichart.com/javascript-chart-documentation" class="external">Documentation</a>

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tformatlabelfn" class="tsd-signature-type">TFormatLabelFn</a>

- Gets or sets a formatCursorLabel function which is used for formatting a data-value into a string for display on a cursor or tooltip If you are creating a custom LabelProvider, you should override formatCursorLabelProperty, not the formatCursorLabel property! See our <a href="https://www.scichart.com/javascript-chart-documentation" class="external">Documentation</a>

  #### Parameters

  - ##### value: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tformatlabelfn" class="tsd-signature-type">TFormatLabelFn</a>

  #### Returns void

### formatLabel

- get formatLabel(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tformatlabelfn" class="tsd-signature-type">TFormatLabelFn</a>

<!-- -->

- inheritdoc  

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tformatlabelfn" class="tsd-signature-type">TFormatLabelFn</a>

### lineSpacing

- get lineSpacing(): number
- set lineSpacing(value: number): void

<!-- -->

- Line spacing to use if text is wrapped, as a multiple of the text height. Defaults to 1.1

  #### Returns number

- Line spacing to use if text is wrapped, as a multiple of the text height. Defaults to 1.1

  #### Parameters

  - ##### value: number

  #### Returns void

### maxDenominator

- get maxDenominator(): number
- set maxDenominator(value: number): void

<!-- -->

- Gets or sets the maximum denominator to use when formatting values as fractions of PI

  #### Returns number

- Gets or sets the maximum denominator to use when formatting values as fractions of PI

  #### Parameters

  - ##### value: number

  #### Returns void

### numericFormat

- get numericFormat(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/enumericformat.html" class="tsd-signature-type">ENumericFormat</a>
- set numericFormat(value: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/enumericformat.html" class="tsd-signature-type">ENumericFormat</a>): void

<!-- -->

- Gets or sets numeric format to use. For a list of values, see [ENumericFormat](https://www.scichart.com/documentation/js/v4/typedoc/enums/enumericformat.html)

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/enumericformat.html" class="tsd-signature-type">ENumericFormat</a>

- Gets or sets numeric format to use. For a list of values, see [ENumericFormat](https://www.scichart.com/documentation/js/v4/typedoc/enums/enumericformat.html)

  #### Parameters

  - ##### value: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/enumericformat.html" class="tsd-signature-type">ENumericFormat</a>

  #### Returns void

### postfix

- get postfix(): string
- set postfix(v: string): void

<!-- -->

- Gets or sets a string to add to the end of each label

  #### Returns string

- Gets or sets a string to add to the end of each label

  #### Parameters

  - ##### v: string

  #### Returns void

### precision

- get precision(): number
- set precision(value: number): void

<!-- -->

- Gets or sets the precision to use when formatting

  #### Returns number

- Gets or sets the precision to use when formatting

  #### Parameters

  - ##### value: number

  #### Returns void

### prefix

- get prefix(): string
- set prefix(v: string): void

<!-- -->

- Gets or sets a string to add to the beginning of each label

  #### Returns string

- Gets or sets a string to add to the beginning of each label

  #### Parameters

  - ##### v: string

  #### Returns void

### rotation

- get rotation(): number
- set rotation(value: number): void

<!-- -->

- #### Returns number

- #### Parameters

  - ##### value: number

  #### Returns void

## Methods

### adjustLabel

- adjustLabel(index: number, text: string, width: number, height: number, x: number, y: number, rx: number, ry: number, rotationRadians: number): { color: number; rotationRadians: number; rx: number; ry: number; text: string; x: number; y: number }

<!-- -->

- This method is called for each label just before drawing and can be used to make final adjustments to the label Note that if native text is NOT used, only x and y of the return values will be used. Only color can be returned undefined to use label default

  #### Parameters

  - ##### index: number

  - ##### text: string

  - ##### width: number

  - ##### height: number

  - ##### x: number

  - ##### y: number

  - ##### rx: number

  - ##### ry: number

  - ##### rotationRadians: number

  #### Returns { color: number; rotationRadians: number; rx: number; ry: number; text: string; x: number; y: number }

  - ##### color: number

  - ##### rotationRadians: number

  - ##### rx: number

  - ##### ry: number

  - ##### text: string

  - ##### x: number

  - ##### y: number

### applyFormat

- applyFormat(value: string): string

<!-- -->

- #### Parameters

  - ##### value: string

  #### Returns string

### attachedToAxis

- attachedToAxis(axis: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html" class="tsd-signature-type">AxisCore</a>): void

<!-- -->

- Called when the [LabelProvider](https://www.scichart.com/documentation/js/v4/typedoc/enums/ebasetype.html#labelprovider) is attached to an [Axis](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html)

  #### Parameters

  - ##### axis: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html" class="tsd-signature-type">AxisCore</a>

    The Axis we are attached to.

  #### Returns void

### Protected clearCache

- clearCache(): void

<!-- -->

- #### Returns void

### delete

- delete(): void

<!-- -->

- #### Returns void

### detachedFromAxis

- detachedFromAxis(): void

<!-- -->

- Called when the [LabelProvider](https://www.scichart.com/documentation/js/v4/typedoc/enums/ebasetype.html#labelprovider) is detached from an [Axis](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html)

  #### Returns void

### getCachedLabelTexture

- getCachedLabelTexture(labelText: string, textureManager: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/texturemanager.html" class="tsd-signature-type">TextureManager</a>, labelStyle: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#ttextstyle" class="tsd-signature-type">TTextStyle</a>): TTextureObject

<!-- -->

- Get a texture for the given label text. By default the textures are created first and then the resulting sizes are used by the layout functions

  #### Parameters

  - ##### labelText: string

    The required text

  - ##### textureManager: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/texturemanager.html" class="tsd-signature-type">TextureManager</a>

    A textureManager instance which contains methods for creating textures

  - ##### labelStyle: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#ttextstyle" class="tsd-signature-type">TTextStyle</a>

    The style for the text

  #### Returns TTextureObject

  A TTextureObject containing the bitmapTexture and the size

### Protected getCachedStyle

- getCachedStyle(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tcachedlabelstyle" class="tsd-signature-type">TCachedLabelStyle</a>

<!-- -->

- This method creates the text style to be stored in the label cache. When useSharedCache = true, the label cache will generate a new styleId if this style does not match any existing style. Cached labels are accessed by text and styleId. If you are overriding getLabelTexture or getLabelTextureAsync and do not ensure the style is unique, you might not get the labels you expect. You can either set useSharedCache = false, override this and set the extras field in [TCachedLabelStyle](https://www.scichart.com/documentation/js/v4/typedoc/index.html#tcachedlabelstyle), or set styleId directly

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tcachedlabelstyle" class="tsd-signature-type">TCachedLabelStyle</a>

### getLabelHeight

- getLabelHeight(ctx: CanvasRenderingContext2D, labelText: string, labelStyle: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#ttextstyle" class="tsd-signature-type">TTextStyle</a>): number

<!-- -->

- Called during axis layout to get the height of the label

  #### Parameters

  - ##### ctx: CanvasRenderingContext2D

    the CanvasRenderingContext2D which can be used to perform text measurment

  - ##### labelText: string

    the text of the label

  - ##### labelStyle: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#ttextstyle" class="tsd-signature-type">TTextStyle</a>

    the style of the label

  #### Returns number

  the label height in pixels

### Protected getLabelSizesNative

- getLabelSizesNative(labels: string\[\], textStyle: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#ttextstyle" class="tsd-signature-type">TTextStyle</a>): void

<!-- -->

- #### Parameters

  - ##### labels: string\[\]

  - ##### textStyle: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#ttextstyle" class="tsd-signature-type">TTextStyle</a>

  #### Returns void

### getLabelTexture

- getLabelTexture(labelText: string, textureManager: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/texturemanager.html" class="tsd-signature-type">TextureManager</a>, labelStyle: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#ttextstyle" class="tsd-signature-type">TTextStyle</a>): { bitmapTexture: TSRTexture; textureHeight: number; textureWidth: number }

<!-- -->

- Create a texture for the given label text. This method is called if useNativeText is false. If overriding this method with useSharedCache = true, consider setting it false for this LabelProvider, otherwise other axes using the same style and text may see your custom texture. Alternatively you can override getCachedStyle or set styleId directly

  #### Parameters

  - ##### labelText: string

  - ##### textureManager: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/texturemanager.html" class="tsd-signature-type">TextureManager</a>

  - ##### labelStyle: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#ttextstyle" class="tsd-signature-type">TTextStyle</a>

  #### Returns { bitmapTexture: TSRTexture; textureHeight: number; textureWidth: number }

  - ##### bitmapTexture: TSRTexture

  - ##### textureHeight: number

  - ##### textureWidth: number

### getLabelTextureAsync

- getLabelTextureAsync(labelText: string, textureManager: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/texturemanager.html" class="tsd-signature-type">TextureManager</a>, labelStyle: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#ttextstyle" class="tsd-signature-type">TTextStyle</a>): Promise\<{ bitmapTexture: TSRTexture; textureHeight: number; textureWidth: number }\>

<!-- -->

- deprecated  
  AsyncLabels have been removed. useNativeText: true provides much greater performance benefit. If using texture labels override getLabelTexture instead

  #### Parameters

  - ##### labelText: string

  - ##### textureManager: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/texturemanager.html" class="tsd-signature-type">TextureManager</a>

  - ##### labelStyle: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#ttextstyle" class="tsd-signature-type">TTextStyle</a>

  #### Returns Promise\<{ bitmapTexture: TSRTexture; textureHeight: number; textureWidth: number }\>

### getLabelWidth

- getLabelWidth(ctx: CanvasRenderingContext2D, labelText: string, labelStyle?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#ttextstyle" class="tsd-signature-type">TTextStyle</a>): number

<!-- -->

- Called during axis layout to get the width of the label

  #### Parameters

  - ##### ctx: CanvasRenderingContext2D

    the CanvasRenderingContext2D which can be used to perform text measurment

  - ##### labelText: string

    the text of the label

  - ##### Optional labelStyle: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#ttextstyle" class="tsd-signature-type">TTextStyle</a>

    the style of the label

  #### Returns number

  the label width in pixels

### getLabels

- getLabels(majorTicks: number\[\]): string\[\]

<!-- -->

- Returns an array of label strings for an array of major tick numeric values

  #### Parameters

  - ##### majorTicks: number\[\]

    The major tick numeric values

  #### Returns string\[\]

### getMaxLabelHeightForHorizontalAxis

- getMaxLabelHeightForHorizontalAxis(majorTickLabels: string\[\], ctx: CanvasRenderingContext2D, labelStyle: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#ttextstyle" class="tsd-signature-type">TTextStyle</a>): number

<!-- -->

- Called during axis layout to get the maximum height of labels on a horizontal axis. Normally this calls getLabelHeight for each label and returns the largest.

  #### Parameters

  - ##### majorTickLabels: string\[\]

    an array of text labels

  - ##### ctx: CanvasRenderingContext2D

    the CanvasRenderingContext2D which can be used to perform text measurment

  - ##### labelStyle: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#ttextstyle" class="tsd-signature-type">TTextStyle</a>

    the style of the labels

  #### Returns number

  the maximum label height in pixels

### getMaxLabelWidthForVerticalAxis

- getMaxLabelWidthForVerticalAxis(majorTickLabels: string\[\], ctx: CanvasRenderingContext2D, labelStyle: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#ttextstyle" class="tsd-signature-type">TTextStyle</a>): number

<!-- -->

- Called during axis layout to get the maximum width of labels on a vertical axis. Normally this calls getLabelWidth for each label and returns the largest.

  #### Parameters

  - ##### majorTickLabels: string\[\]

    an array of text labels

  - ##### ctx: CanvasRenderingContext2D

    the CanvasRenderingContext2D which can be used to perform text measurment

  - ##### labelStyle: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#ttextstyle" class="tsd-signature-type">TTextStyle</a>

    the style of the labels

  #### Returns number

  the maximum label width in pixels

### getNativeLabelInfo

- getNativeLabelInfo(labelText: string): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/labelinfo.html" class="tsd-signature-type">LabelInfo</a>

<!-- -->

- #### Parameters

  - ##### labelText: string

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/labelinfo.html" class="tsd-signature-type">LabelInfo</a>

### invalidateCache

- invalidateCache(): void

<!-- -->

- #### Returns void

### Protected invalidateParent

- invalidateParent(): void

<!-- -->

- #### Returns void

### onBeginAxisDraw

- onBeginAxisDraw(): void

<!-- -->

- inheritdoc  

  #### Returns void

### Protected pruneTickTextCache

- pruneTickTextCache(): void

<!-- -->

- #### Returns void

### resetCache

- resetCache(): void

<!-- -->

- #### Returns void

### toJSON

- toJSON(): { options: {}; type: string }

<!-- -->

- #### Returns { options: {}; type: string }

  - ##### options: {}

  - ##### type: string

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
