<img src="out_scichartv4/typedoc/interfaces/ipaletteprovider_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipaletteprovider.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [IPaletteProvider](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipaletteprovider.html)

# Interface IPaletteProvider

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

The PaletteProvider interface allows you to perform per-point paletting or coloring of series or data-points in SciChart's <a href="https://www.scichart.com/javascript-chart-features" class="external">High Performance Realtime JavaScript Charts</a>

remarks  
See derived type [IStrokePaletteProvider](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/istrokepaletteprovider.html) for coloring line series or strokes. See derived type [IFillPaletteProvider](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ifillpaletteprovider.html) for coloring fills or areas.

### Hierarchy

- IPaletteProvider
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iadvancedpaletteprovider.html" class="tsd-signature-type">IAdvancedPaletteProvider</a>
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/istrokepaletteprovider.html" class="tsd-signature-type">IStrokePaletteProvider</a>
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ifillpaletteprovider.html" class="tsd-signature-type">IFillPaletteProvider</a>
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmarkerpaletteprovider.html" class="tsd-signature-type">IPointMarkerPaletteProvider</a>

### Implemented by

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basepaletteprovider.html" class="tsd-signature-type">BasePaletteProvider</a>

## Index

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipaletteprovider.html#israngeindependant" class="tsd-kind-icon">isRangeIndependant</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipaletteprovider.html#onattached" class="tsd-kind-icon">onAttached</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipaletteprovider.html#ondetached" class="tsd-kind-icon">onDetached</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipaletteprovider.html#shouldupdatepalette" class="tsd-kind-icon">shouldUpdatePalette</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipaletteprovider.html#tojson" class="tsd-kind-icon">toJSON</a>

## Properties

### Optional isRangeIndependant

isRangeIndependant: boolean

Set true if the paletting does not depend on the visible Range. This prevents the palette being recalculated if only the visible range changes.

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
