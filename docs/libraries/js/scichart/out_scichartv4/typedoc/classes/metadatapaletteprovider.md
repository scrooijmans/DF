<img src="out_scichartv4/typedoc/classes/metadatapaletteprovider_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/metadatapaletteprovider.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [MetadataPaletteProvider](https://www.scichart.com/documentation/js/v4/typedoc/classes/metadatapaletteprovider.html)

# Class MetadataPaletteProvider

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

### Hierarchy

- MetadataPaletteProvider

### Implements

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ifillpaletteprovider.html" class="tsd-signature-type">IFillPaletteProvider</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/istrokepaletteprovider.html" class="tsd-signature-type">IStrokePaletteProvider</a>

## Index

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/metadatapaletteprovider.html#fillpalettemode" class="tsd-kind-icon">fillPaletteMode</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/metadatapaletteprovider.html#strokepalettemode" class="tsd-kind-icon">strokePaletteMode</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/metadatapaletteprovider.html#onattached" class="tsd-kind-icon">onAttached</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/metadatapaletteprovider.html#ondetached" class="tsd-kind-icon">onDetached</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/metadatapaletteprovider.html#overridefillargb" class="tsd-kind-icon">overrideFillArgb</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/metadatapaletteprovider.html#overridepointmarkerargb" class="tsd-kind-icon">overridePointMarkerArgb</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/metadatapaletteprovider.html#overridestrokeargb" class="tsd-kind-icon">overrideStrokeArgb</a>

## Properties

### fillPaletteMode

fillPaletteMode: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/efillpalettemode.html" class="tsd-signature-type">EFillPaletteMode</a> = EFillPaletteMode.SOLID

### strokePaletteMode

strokePaletteMode: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/estrokepalettemode.html" class="tsd-signature-type">EStrokePaletteMode</a> = EStrokePaletteMode.SOLID

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

- overrideFillArgb(xValue: number, yValue: number, index: number, opacity?: number, metadata?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icolormetadata.html" class="tsd-signature-type">IColorMetadata</a>): number

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

  - ##### Optional metadata: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icolormetadata.html" class="tsd-signature-type">IColorMetadata</a>

    the point metadata

  #### Returns number

  an ARGB color code, e.g. 0xFFFF0000 would be red, or 'undefined' for default colouring

### overridePointMarkerArgb

- overridePointMarkerArgb(xValue: number, yValue: number, index: number, opacity?: number, metadata?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icolormetadata.html" class="tsd-signature-type">IColorMetadata</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tpointmarkerargb" class="tsd-signature-type">TPointMarkerArgb</a>

<!-- -->

- inheritdoc  

  #### Parameters

  - ##### xValue: number

  - ##### yValue: number

  - ##### index: number

  - ##### Optional opacity: number

  - ##### Optional metadata: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icolormetadata.html" class="tsd-signature-type">IColorMetadata</a>

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tpointmarkerargb" class="tsd-signature-type">TPointMarkerArgb</a>

### overrideStrokeArgb

- overrideStrokeArgb(xValue: number, yValue: number, index: number, opacity?: number, metadata?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icolormetadata.html" class="tsd-signature-type">IColorMetadata</a>): number

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

  - ##### Optional metadata: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icolormetadata.html" class="tsd-signature-type">IColorMetadata</a>

    the point metadata

  #### Returns number

  an ARGB color code, e.g. 0xFFFF0000 would be red, or 'undefined' for default colouring

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
