<img src="out_scichartv4/typedoc/classes/palettefactory_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/palettefactory.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [PaletteFactory](https://www.scichart.com/documentation/js/v4/typedoc/classes/palettefactory.html)

# Class PaletteFactory

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

The PaletteFactory allows easy creation of palettes for some chart types

### Hierarchy

- PaletteFactory

## Index

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/palettefactory.html#precision" class="tsd-kind-icon">precision</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/palettefactory.html#createcolormap" class="tsd-kind-icon">createColorMap</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/palettefactory.html#creategradient" class="tsd-kind-icon">createGradient</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/palettefactory.html#createygradient" class="tsd-kind-icon">createYGradient</a>

## Properties

### Static Readonly precision

precision: number = 500

## Methods

### Static createColorMap

- createColorMap(webAssemblyContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart" class="tsd-signature-type">TSciChart</a>, gradientStops: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tgradientstop" class="tsd-signature-type">TGradientStop</a>\[\]): number\[\]

<!-- -->

- #### Parameters

  - ##### webAssemblyContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart" class="tsd-signature-type">TSciChart</a>

  - ##### gradientStops: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tgradientstop" class="tsd-signature-type">TGradientStop</a>\[\]

  #### Returns number\[\]

### Static createGradient

- createGradient(webAssemblyContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart" class="tsd-signature-type">TSciChart</a>, gradientBrush: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/gradientparams.html" class="tsd-signature-type">GradientParams</a>, options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/igradientpaletteoptions.html" class="tsd-signature-type">IGradientPaletteOptions</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iallpaletteproviders.html" class="tsd-signature-type">IAllPaletteProviders</a>

<!-- -->

- Creates a multi purpose Gradient Palette for line series, scatter, bubble or column and mountain series returning an [IPaletteProvider](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipaletteprovider.html) implementation which colors data-points of charts depending on the x-index of the data according to the Gradient Brush passed in

  #### Parameters

  - ##### webAssemblyContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart" class="tsd-signature-type">TSciChart</a>

    the [SciChart WebAssembly Context](https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart) containing native methods

  - ##### gradientBrush: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/gradientparams.html" class="tsd-signature-type">GradientParams</a>

    the [GradientParams](https://www.scichart.com/documentation/js/v4/typedoc/classes/gradientparams.html) containing information about the Gradient Brush

  - ##### Optional options: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/igradientpaletteoptions.html" class="tsd-signature-type">IGradientPaletteOptions</a>

    the [IGradientPaletteOptions](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/igradientpaletteoptions.html) containing additional options to turn stroke, fill or pointmarker sections on or off, and opacity per option

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iallpaletteproviders.html" class="tsd-signature-type">IAllPaletteProviders</a>

### Static createYGradient

- createYGradient(webAssemblyContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart" class="tsd-signature-type">TSciChart</a>, gradientBrush: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/gradientparams.html" class="tsd-signature-type">GradientParams</a>, yRange: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>, options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/igradientpaletteoptions.html" class="tsd-signature-type">IGradientPaletteOptions</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iallpaletteproviders.html" class="tsd-signature-type">IAllPaletteProviders</a>

<!-- -->

- #### Parameters

  - ##### webAssemblyContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart" class="tsd-signature-type">TSciChart</a>

  - ##### gradientBrush: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/gradientparams.html" class="tsd-signature-type">GradientParams</a>

  - ##### yRange: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

  - ##### Optional options: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/igradientpaletteoptions.html" class="tsd-signature-type">IGradientPaletteOptions</a>

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iallpaletteproviders.html" class="tsd-signature-type">IAllPaletteProviders</a>

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
