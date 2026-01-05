<img src="out_scichartv4/typedoc/classes/heatmapcolormap_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/heatmapcolormap.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [HeatmapColorMap](https://www.scichart.com/documentation/js/v4/typedoc/classes/heatmapcolormap.html)

# Class HeatmapColorMap

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

The HeatmapColorMap is a utility class for managing color mapping in heatmap visualizations.

------------------------------------------------------------------------

ðŸ“š Docs: <a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/uniform-heatmap-renderable-series/color-maps-and-legends/" class="external">https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/uniform-heatmap-renderable-series/color-maps-and-legends/</a>

### Hierarchy

- HeatmapColorMap

### Implements

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icolormapparams.html" class="tsd-signature-type">IColorMapParams</a>

## Index

### Constructors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/heatmapcolormap.html#constructor" class="tsd-kind-icon">constructor</a>

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/heatmapcolormap.html#propertychanged" class="tsd-kind-icon">propertyChanged</a>

### Accessors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/heatmapcolormap.html#gradientstops" class="tsd-kind-icon">gradientStops</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/heatmapcolormap.html#maximum" class="tsd-kind-icon">maximum</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/heatmapcolormap.html#minimum" class="tsd-kind-icon">minimum</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/heatmapcolormap.html#calculatescalefactor" class="tsd-kind-icon">calculateScaleFactor</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/heatmapcolormap.html#notifypropertychanged" class="tsd-kind-icon">notifyPropertyChanged</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/heatmapcolormap.html#tojson" class="tsd-kind-icon">toJSON</a>

## Constructors

### constructor

- new HeatmapColorMap(options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iheatmapcolormapoptions.html" class="tsd-signature-type">IHeatmapColorMapOptions</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/heatmapcolormap.html" class="tsd-signature-type">HeatmapColorMap</a>

<!-- -->

- #### Parameters

  - ##### Optional options: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iheatmapcolormapoptions.html" class="tsd-signature-type">IHeatmapColorMapOptions</a>

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/heatmapcolormap.html" class="tsd-signature-type">HeatmapColorMap</a>

## Properties

### Readonly propertyChanged

propertyChanged: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/eventhandler.html" class="tsd-signature-type">EventHandler</a>\<<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/propertychangedeventargs.html" class="tsd-signature-type">PropertyChangedEventArgs</a>\>

## Accessors

### gradientStops

- get gradientStops(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tgradientstop" class="tsd-signature-type">TGradientStop</a>\[\]
- set gradientStops(gradientStops: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tgradientstop" class="tsd-signature-type">TGradientStop</a>\[\]): void

<!-- -->

- #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tgradientstop" class="tsd-signature-type">TGradientStop</a>\[\]

- #### Parameters

  - ##### gradientStops: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tgradientstop" class="tsd-signature-type">TGradientStop</a>\[\]

  #### Returns void

### maximum

- get maximum(): number
- set maximum(maximum: number): void

<!-- -->

- #### Returns number

- #### Parameters

  - ##### maximum: number

  #### Returns void

### minimum

- get minimum(): number
- set minimum(minimum: number): void

<!-- -->

- #### Returns number

- #### Parameters

  - ##### minimum: number

  #### Returns void

## Methods

### Protected calculateScaleFactor

- calculateScaleFactor(): void

<!-- -->

- #### Returns void

### Protected notifyPropertyChanged

- notifyPropertyChanged(property: string): void

<!-- -->

- #### Parameters

  - ##### property: string

  #### Returns void

### toJSON

- toJSON(): { gradientStops: { color: string; offset: number }\[\]; maximum: number; minimum: number }

<!-- -->

- #### Returns { gradientStops: { color: string; offset: number }\[\]; maximum: number; minimum: number }

  - ##### gradientStops: { color: string; offset: number }\[\]

  - ##### maximum: number

  - ##### minimum: number

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
