<img src="out_scichartv4/typedoc/interfaces/ipointmarkerpaletteprovider_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmarkerpaletteprovider.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [IPointMarkerPaletteProvider](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmarkerpaletteprovider.html)

# Interface IPointMarkerPaletteProvider

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

The IPointMarkerPaletteProvider interface allows you to perform per-point paletting or coloring of series or data-points in Scatter Charts and JavaScript chart types which have a stroke line or outline as well as a fill or body

remarks  
See type [IStrokePaletteProvider](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/istrokepaletteprovider.html) for per data-point coloring of strokes or outlines See type [IFillPaletteProvider](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ifillpaletteprovider.html) for per data-point coloring of fills or areas.

### Hierarchy

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipaletteprovider.html" class="tsd-signature-type">IPaletteProvider</a>
  - IPointMarkerPaletteProvider
    - <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iallpaletteproviders.html" class="tsd-signature-type">IAllPaletteProviders</a>

### Implemented by

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datapointselectionpaletteprovider.html" class="tsd-signature-type">DataPointSelectionPaletteProvider</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/defaultpaletteprovider.html" class="tsd-signature-type">DefaultPaletteProvider</a>

## Index

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmarkerpaletteprovider.html#israngeindependant" class="tsd-kind-icon">isRangeIndependant</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmarkerpaletteprovider.html#strokepalettemode" class="tsd-kind-icon">strokePaletteMode</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmarkerpaletteprovider.html#onattached" class="tsd-kind-icon">onAttached</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmarkerpaletteprovider.html#ondetached" class="tsd-kind-icon">onDetached</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmarkerpaletteprovider.html#overridepointmarkerargb" class="tsd-kind-icon">overridePointMarkerArgb</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmarkerpaletteprovider.html#shouldupdatepalette" class="tsd-kind-icon">shouldUpdatePalette</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmarkerpaletteprovider.html#tojson" class="tsd-kind-icon">toJSON</a>

## Properties

### Optional isRangeIndependant

isRangeIndependant: boolean

Set true if the paletting does not depend on the visible Range. This prevents the palette being recalculated if only the visible range changes.

### strokePaletteMode

strokePaletteMode: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/estrokepalettemode.html" class="tsd-signature-type">EStrokePaletteMode</a>

Gets the stroke palette mode

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

### Optional shouldUpdatePalette

- shouldUpdatePalette(): boolean

<!-- -->

- Called once before the per-vertex loop starts.

  #### Returns boolean

  true if paletting should be forced to run. If this exists and returns false, the existing paletting state is reused if possible. If this does NOT exist, the palette will be recalculated on every render. This default will change in v4. Use this to force the palette to be recalculated if some external parameter to it changes.

### Optional toJSON

- toJSON(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tpaletteproviderdefinition" class="tsd-signature-type">TPaletteProviderDefinition</a>

<!-- -->

- Convert the object to a definition that can be serialized to JSON, or used directly with the builder api

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tpaletteproviderdefinition" class="tsd-signature-type">TPaletteProviderDefinition</a>

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
