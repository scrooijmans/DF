<img src="out_scichartv4/typedoc/classes/defaultpaletteprovider_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/defaultpaletteprovider.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [DefaultPaletteProvider](https://www.scichart.com/documentation/js/v4/typedoc/classes/defaultpaletteprovider.html)

# Class DefaultPaletteProvider

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

A Default Palette provider is applied to the [BaseRenderableSeries.paletteProvider](https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries.html#paletteprovider) property in the constructor however all its functions such as [DefaultPaletteProvider.overrideFillArgb](https://www.scichart.com/documentation/js/v4/typedoc/classes/defaultpaletteprovider.html#overridefillargb), [DefaultPaletteProvider.overrideStrokeArgb](https://www.scichart.com/documentation/js/v4/typedoc/classes/defaultpaletteprovider.html#overridestrokeargb) etc... are set to undefined. This allows a user to do quick one-line paletteproviders in vanilla Javascript like this:

``` javascript
const series = new FastLineRenderableSeries(wasmContext);
series.paletteProvider.overrideFillArgb = (xValue, yValue, index) => {
    return yValue > 0 ? parseColorToUIntArgb("white") : undefined;
}
```

### Hierarchy

- DefaultPaletteProvider

### Implements

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/istrokepaletteprovider.html" class="tsd-signature-type">IStrokePaletteProvider</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ifillpaletteprovider.html" class="tsd-signature-type">IFillPaletteProvider</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmarkerpaletteprovider.html" class="tsd-signature-type">IPointMarkerPaletteProvider</a>

## Index

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/defaultpaletteprovider.html#fillpalettemode" class="tsd-kind-icon">fillPaletteMode</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/defaultpaletteprovider.html#parentseries" class="tsd-kind-icon">parentSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/defaultpaletteprovider.html#strokepalettemode" class="tsd-kind-icon">strokePaletteMode</a>

### Accessors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/defaultpaletteprovider.html#israngeindependant" class="tsd-kind-icon">isRangeIndependant</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/defaultpaletteprovider.html#onattached" class="tsd-kind-icon">onAttached</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/defaultpaletteprovider.html#ondetached" class="tsd-kind-icon">onDetached</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/defaultpaletteprovider.html#overridefillargb" class="tsd-kind-icon">overrideFillArgb</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/defaultpaletteprovider.html#overridepointmarkerargb" class="tsd-kind-icon">overridePointMarkerArgb</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/defaultpaletteprovider.html#overridestrokeargb" class="tsd-kind-icon">overrideStrokeArgb</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/defaultpaletteprovider.html#shouldupdatepalette" class="tsd-kind-icon">shouldUpdatePalette</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/defaultpaletteprovider.html#tojson" class="tsd-kind-icon">toJSON</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/defaultpaletteprovider.html#createempty" class="tsd-kind-icon">createEmpty</a>

## Properties

### fillPaletteMode

fillPaletteMode: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/efillpalettemode.html" class="tsd-signature-type">EFillPaletteMode</a> = EFillPaletteMode.GRADIENT

### Protected parentSeries

parentSeries: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries.html" class="tsd-signature-type">IRenderableSeries</a>

### strokePaletteMode

strokePaletteMode: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/estrokepalettemode.html" class="tsd-signature-type">EStrokePaletteMode</a> = EStrokePaletteMode.GRADIENT

## Accessors

### isRangeIndependant

- get isRangeIndependant(): boolean

<!-- -->

- Set true if the paletting does not depend on the visible Range. This prevents the palette being recalculated if only the visible range changes.

  inheritdoc  

  #### Returns boolean

## Methods

### onAttached

- onAttached(parentSeries: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries.html" class="tsd-signature-type">IRenderableSeries</a>): void

<!-- -->

- Called when the PaletteProvider instance is attached to a [RenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries.html). Use this to be notified when attached and keep a reference to the parent series

  #### Parameters

  - ##### parentSeries: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries.html" class="tsd-signature-type">IRenderableSeries</a>

  #### Returns void

### onDetached

- onDetached(): void

<!-- -->

- Called when the PaletteProvider instance is detached from a [RenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries.html).

  #### Returns void

### overrideFillArgb

- overrideFillArgb(xValue: number, yValue: number, index: number, opacity?: number, metadata?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmetadata.html" class="tsd-signature-type">IPointMetadata</a>): number \| undefined

<!-- -->

- Called by SciChart and may be used to override the color of filled polygon in various chart types.

  remarks  
  WARNING: CALLED PER-VERTEX, MAY RESULT IN PERFORMANCE DEGREDATION IF COMPLEX CODE EXECUTED HERE

  #### Parameters

  - ##### xValue: number

    the current XValue

  - ##### yValue: number

    the current YValue

  - ##### index: number

    the current index to the data

  - ##### Optional opacity: number

    the current opacity

  - ##### Optional metadata: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmetadata.html" class="tsd-signature-type">IPointMetadata</a>

    the point metadata

  #### Returns number \| undefined

  an ARGB color code, e.g. 0xFFFF0000 would be red, or 'undefined' for default colouring

### overridePointMarkerArgb

- overridePointMarkerArgb(xValue: number, yValue: number, index: number, opacity?: number, metadata?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmetadata.html" class="tsd-signature-type">IPointMetadata</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tpointmarkerargb" class="tsd-signature-type">TPointMarkerArgb</a> \| undefined

<!-- -->

- Called by SciChart and may be used to override the color of fill/stroke on pointmarkers WARNING: CALLED PER-VERTEX, MAY RESULT IN PERFORMANCE DEGREDATION IF COMPLEX CODE EXECUTED HERE

  #### Parameters

  - ##### xValue: number

    the current XValue

  - ##### yValue: number

    the current YValue

  - ##### index: number

    the current index to the data

  - ##### Optional opacity: number

    the current opacity

  - ##### Optional metadata: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmetadata.html" class="tsd-signature-type">IPointMetadata</a>

    the point metadata

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tpointmarkerargb" class="tsd-signature-type">TPointMarkerArgb</a> \| undefined

  TPointMarkerArgb type with ARGB stroke & fill color codes, e.g. 0xFFFF0000 would be red, or 'undefined' for default colouring

### overrideStrokeArgb

- overrideStrokeArgb(xValue: number, yValue: number, index: number, opacity?: number, metadata?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmetadata.html" class="tsd-signature-type">IPointMetadata</a>): number \| undefined

<!-- -->

- Called by SciChart and may be used to override the color of a line segment or stroke outline in various chart types.

  remarks  
  WARNING: CALLED PER-VERTEX, MAY RESULT IN PERFORMANCE DEGREDATION IF COMPLEX CODE EXECUTED HERE

  #### Parameters

  - ##### xValue: number

    the current XValue

  - ##### yValue: number

    the current YValue

  - ##### index: number

    the current index to the data

  - ##### Optional opacity: number

    the current opacity

  - ##### Optional metadata: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmetadata.html" class="tsd-signature-type">IPointMetadata</a>

    the point metadata

  #### Returns number \| undefined

  an ARGB color code, e.g. 0xFFFF0000 would be red, or 'undefined' for default colouring

### shouldUpdatePalette

- shouldUpdatePalette(): boolean

<!-- -->

- Called once before the per-vertex loop starts.

  #### Returns boolean

  true if paletting should be forced to run. If this exists and returns false, the existing paletting state is reused if possible. If this does NOT exist, the palette will be recalculated on every render. This default will change in v4. Use this to force the palette to be recalculated if some external parameter to it changes.

### toJSON

- toJSON(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tpaletteproviderdefinition" class="tsd-signature-type">TPaletteProviderDefinition</a>

<!-- -->

- #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tpaletteproviderdefinition" class="tsd-signature-type">TPaletteProviderDefinition</a>

### Static createEmpty

- createEmpty(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/defaultpaletteprovider.html" class="tsd-signature-type">DefaultPaletteProvider</a>

<!-- -->

- #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/defaultpaletteprovider.html" class="tsd-signature-type">DefaultPaletteProvider</a>

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
