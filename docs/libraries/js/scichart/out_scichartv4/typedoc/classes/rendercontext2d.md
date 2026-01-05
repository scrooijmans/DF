<img src="out_scichartv4/typedoc/classes/rendercontext2d_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rendercontext2d.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [RenderContext2D](https://www.scichart.com/documentation/js/v4/typedoc/classes/rendercontext2d.html)

# Class RenderContext2D

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

The RenderContext2D provides methods for drawing to an {@link HTMLCanvasElement \| Html5 Canvas} This context class is used in SciChart's High Performance Realtime <a href="https://www.scichart.com/javascript-chart-features" class="external">JavaScript Charts</a> to draw shapes, lines, fills, images and more

### Hierarchy

- RenderContext2D

## Index

### Constructors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rendercontext2d.html#constructor" class="tsd-kind-icon">constructor</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rendercontext2d.html#clear" class="tsd-kind-icon">clear</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rendercontext2d.html#drawcircle" class="tsd-kind-icon">drawCircle</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rendercontext2d.html#drawrect" class="tsd-kind-icon">drawRect</a>

## Constructors

### constructor

- new RenderContext2D(canvas2D: HTMLCanvasElement): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rendercontext2d.html" class="tsd-signature-type">RenderContext2D</a>

<!-- -->

- Creates an instance of the RenderContext2D

  #### Parameters

  - ##### canvas2D: HTMLCanvasElement

    the {@link HTMLCanvasElement} we are drawing to

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rendercontext2d.html" class="tsd-signature-type">RenderContext2D</a>

## Methods

### clear

- clear(): void

<!-- -->

- Clears the backing canvas element

  #### Returns void

### drawCircle

- drawCircle(x: number, y: number, radius: number, fillHtmlColor: string): void

<!-- -->

- Draws a circle to the specified location and with provided Htmlcolor string

  #### Parameters

  - ##### x: number

    the X-location of the rectangle

  - ##### y: number

    the Y-location of the rectangle

  - ##### radius: number

    the radius of the circle

  - ##### fillHtmlColor: string

    the Html color code to fill the circle

  #### Returns void

### drawRect

- drawRect(x: number, y: number, width: number, height: number, htmlColor?: string): void

<!-- -->

- Draws a rectangle to the specified location and with provided Htmlcolor string

  #### Parameters

  - ##### x: number

    the X-location of the rectangle

  - ##### y: number

    the Y-location of the rectangle

  - ##### width: number

    the width of the rectangle

  - ##### height: number

    the height of the rectangle

  - ##### Default value htmlColor: string = "rgba(211,211,211,0.5)"

    the Html color code to fill the rectangle

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
