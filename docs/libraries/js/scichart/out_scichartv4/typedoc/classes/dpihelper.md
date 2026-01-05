<img src="out_scichartv4/typedoc/classes/dpihelper_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/dpihelper.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [DpiHelper](https://www.scichart.com/documentation/js/v4/typedoc/classes/dpihelper.html)

# Class DpiHelper

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

A helper class with methods and properties for DPI scaling of canvases

### Hierarchy

- DpiHelper

## Index

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/dpihelper.html#isdpiscaleenabled" class="tsd-kind-icon">IsDpiScaleEnabled</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/dpihelper.html#pixel_ratio" class="tsd-kind-icon">PIXEL_RATIO</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/dpihelper.html#dpichanged" class="tsd-kind-icon">dpiChanged</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/dpihelper.html#adjustlinestyle" class="tsd-kind-icon">adjustLineStyle</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/dpihelper.html#adjuststrokesize" class="tsd-kind-icon">adjustStrokeSize</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/dpihelper.html#adjusttextstyle" class="tsd-kind-icon">adjustTextStyle</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/dpihelper.html#adjustthickness" class="tsd-kind-icon">adjustThickness</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/dpihelper.html#createcanvas" class="tsd-kind-icon">createCanvas</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/dpihelper.html#initialize" class="tsd-kind-icon">initialize</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/dpihelper.html#setheight" class="tsd-kind-icon">setHeight</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/dpihelper.html#setsize" class="tsd-kind-icon">setSize</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/dpihelper.html#setwidth" class="tsd-kind-icon">setWidth</a>

## Properties

### Static IsDpiScaleEnabled

IsDpiScaleEnabled: boolean = true

When true, automatically adjust chart resolution for sharper images on high DPI screens

### Static PIXEL_RATIO

PIXEL_RATIO: number

Returns the current DPI scaling factor

remarks  
Normal screen = 1, Retina display = 2, a Windows PC at 125% = 1.25

### Static dpiChanged

dpiChanged: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/eventhandler.html" class="tsd-signature-type">EventHandler</a>\<<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tdpichangedeventargs" class="tsd-signature-type">TDpiChangedEventArgs</a>\> = new EventHandler\<TDpiChangedEventArgs\>()

Event you can subscribe to for Dpi Changes. See [EventHandler](https://www.scichart.com/documentation/js/v4/typedoc/classes/eventhandler.html) for subscription syntax. Remember to unsubscribe to prevent memory leaks!

## Methods

### Static adjustLineStyle

- adjustLineStyle(lineStyle: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ilinestyle.html" class="tsd-signature-type">ILineStyle</a>, dpiScale?: number): <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ilinestyle.html" class="tsd-signature-type">ILineStyle</a>

<!-- -->

- #### Parameters

  - ##### lineStyle: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ilinestyle.html" class="tsd-signature-type">ILineStyle</a>

  - ##### Default value dpiScale: number = DpiHelper.PIXEL_RATIO

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ilinestyle.html" class="tsd-signature-type">ILineStyle</a>

### Static adjustStrokeSize

- adjustStrokeSize(gridLineStyle: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tgridlinestyle" class="tsd-signature-type">TGridLineStyle</a>, dpiScale?: number): <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tgridlinestyle" class="tsd-signature-type">TGridLineStyle</a>

<!-- -->

- #### Parameters

  - ##### gridLineStyle: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tgridlinestyle" class="tsd-signature-type">TGridLineStyle</a>

  - ##### Default value dpiScale: number = DpiHelper.PIXEL_RATIO

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tgridlinestyle" class="tsd-signature-type">TGridLineStyle</a>

### Static adjustTextStyle

- adjustTextStyle(textStyle: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#ttextstyle" class="tsd-signature-type">TTextStyle</a>, dpiScale?: number): <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#ttextstyle" class="tsd-signature-type">TTextStyle</a>

<!-- -->

- #### Parameters

  - ##### textStyle: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#ttextstyle" class="tsd-signature-type">TTextStyle</a>

  - ##### Default value dpiScale: number = DpiHelper.PIXEL_RATIO

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#ttextstyle" class="tsd-signature-type">TTextStyle</a>

### Static adjustThickness

- adjustThickness(thickness: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/thickness.html" class="tsd-signature-type">Thickness</a>, dpiScale?: number): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/thickness.html" class="tsd-signature-type">Thickness</a>

<!-- -->

- #### Parameters

  - ##### thickness: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/thickness.html" class="tsd-signature-type">Thickness</a>

  - ##### Default value dpiScale: number = DpiHelper.PIXEL_RATIO

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/thickness.html" class="tsd-signature-type">Thickness</a>

### Static createCanvas

- createCanvas(desiredWidth?: number, desiredHeight?: number): HTMLCanvasElement

<!-- -->

- Creates a HTML Canvas element and applies the desired width, height using the [PIXEL_RATIO](https://www.scichart.com/documentation/js/v4/typedoc/classes/dpihelper.html#pixel_ratio) DPI scaling factor

  #### Parameters

  - ##### Default value desiredWidth: number = 0

  - ##### Default value desiredHeight: number = 0

  #### Returns HTMLCanvasElement

### Static initialize

- initialize(): void

<!-- -->

- Static initialization function for [DpiHelper](https://www.scichart.com/documentation/js/v4/typedoc/classes/dpihelper.html). Is called once by the framework on startup

  #### Returns void

### Static setHeight

- setHeight(canvas: HTMLCanvasElement, desiredHeight: number): void

<!-- -->

- Sets the desired height on an HTML Canvas element using the [PIXEL_RATIO](https://www.scichart.com/documentation/js/v4/typedoc/classes/dpihelper.html#pixel_ratio) DPI scaling factor

  #### Parameters

  - ##### canvas: HTMLCanvasElement

  - ##### desiredHeight: number

  #### Returns void

### Static setSize

- setSize(canvas: HTMLCanvasElement, desiredWidth: number, desiredHeight: number): void

<!-- -->

- Sets the desired size on an HTML Canvas element using the [PIXEL_RATIO](https://www.scichart.com/documentation/js/v4/typedoc/classes/dpihelper.html#pixel_ratio) DPI scaling factor

  #### Parameters

  - ##### canvas: HTMLCanvasElement

  - ##### desiredWidth: number

  - ##### desiredHeight: number

  #### Returns void

### Static setWidth

- setWidth(canvas: HTMLCanvasElement, desiredWidth: number): void

<!-- -->

- Sets the desired width on an HTML Canvas element using the [PIXEL_RATIO](https://www.scichart.com/documentation/js/v4/typedoc/classes/dpihelper.html#pixel_ratio) DPI scaling factor

  #### Parameters

  - ##### canvas: HTMLCanvasElement

  - ##### desiredWidth: number

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
