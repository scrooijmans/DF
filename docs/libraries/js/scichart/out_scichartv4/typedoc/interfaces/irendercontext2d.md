<img src="out_scichartv4/typedoc/interfaces/irendercontext2d_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irendercontext2d.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [IRenderContext2D](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irendercontext2d.html)

# Interface IRenderContext2D

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

The RenderContext provides methods for drawing to a WebGL WebAssembly Canvas or standard HTML5 Canvas. This interface is used in SciChart's High Performance Realtime <a href="https://www.scichart.com/javascript-chart-features" class="external">JavaScript Charts</a> to draw shapes, lines, fills, images and more

### Hierarchy

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html" class="tsd-signature-type">IDeletable</a>
  - IRenderContext2D

### Implemented by

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/batchrendercontext.html" class="tsd-signature-type">BatchRenderContext</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/webglrendercontext2d.html" class="tsd-signature-type">WebGlRenderContext2D</a>

## Index

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irendercontext2d.html#createpen" class="tsd-kind-icon">createPen</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irendercontext2d.html#createsolidbrush" class="tsd-kind-icon">createSolidBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irendercontext2d.html#delete" class="tsd-kind-icon">delete</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irendercontext2d.html#drawline" class="tsd-kind-icon">drawLine</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irendercontext2d.html#drawlines" class="tsd-kind-icon">drawLines</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irendercontext2d.html#drawrect" class="tsd-kind-icon">drawRect</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irendercontext2d.html#drawtrianglestrip" class="tsd-kind-icon">drawTriangleStrip</a>

## Methods

### createPen

- createPen(stroke: string, strokeThickness: number, strokeDashArray?: number\[\], antiAliased?: boolean): <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipen2d.html" class="tsd-signature-type">IPen2D</a>

<!-- -->

- description  
  creates a pen (which you should cache) from the provided stroke and strokeThickness

  #### Parameters

  - ##### stroke: string

    in hex format e.g. \#FF6600 or CSS rgba format rgb(255,70,30,0.8)

  - ##### strokeThickness: number

    in pixels

  - ##### Optional strokeDashArray: number\[\]

  - ##### Optional antiAliased: boolean

    true if the pen draws an Anti-Aliased linez

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipen2d.html" class="tsd-signature-type">IPen2D</a>

### createSolidBrush

- createSolidBrush(fill: string, opacity?: number): <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibrush2d.html" class="tsd-signature-type">IBrush2D</a>

<!-- -->

- description  
  creates a solid color brush (which you should cache) from the provided fillColor and opacity

  #### Parameters

  - ##### fill: string

    color in hex format, e.g. \#FF6600 or CSS rgba format rgb(255,70,30,0.8)

  - ##### Optional opacity: number

    from 0.0 to 1.0

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibrush2d.html" class="tsd-signature-type">IBrush2D</a>

### delete

- delete(): void

<!-- -->

- Deletes native (WebAssembly) memory used by this type, after which it cannot be used.

  remarks  
  Call .delete() before finishing with the object to ensure that WebAssmembly memory leaks do not occur.

  All elements within SciChart's High Performance <a href="https://www.scichart.com/javascript-chart-features" class="external">Realtime JavaScript Charts</a> which implement [IDeletable](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html) must be deleted manually to free native (WebAssembly) memory

  #### Returns void

### drawLine

- drawLine(x1: number, y1: number, x2: number, y2: number, strokePen: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipen2d.html" class="tsd-signature-type">IPen2D</a>, viewRect: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>, clipRect: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>): void

<!-- -->

- description  
  Draws a single line from (x1,y1) to (x2,y2) with the specified Pen

  #### Parameters

  - ##### x1: number

    The X1 coordinate in pixels

  - ##### y1: number

    The Y1 coordinate in pixels

  - ##### x2: number

    The X2 coordinate in pixels

  - ##### y2: number

    The Y2 coordinate in pixels

  - ##### strokePen: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipen2d.html" class="tsd-signature-type">IPen2D</a>

    the pen to draw with

  - ##### viewRect: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>

    the series viewRect, used for translate and clipping

  - ##### clipRect: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>

  #### Returns void

### drawLines

- drawLines(xyValues: number\[\], strokePen: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipen2d.html" class="tsd-signature-type">IPen2D</a>, viewRect: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>, clipRect: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>, lineDrawMode?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/elinedrawmode.html" class="tsd-signature-type">ELineDrawMode</a>): void

<!-- -->

- description  
  Draws a polyline with the specified Pen

  #### Parameters

  - ##### xyValues: number\[\]

    An array of x,y points arranged as x0y0 x1y1 ... xnyn

  - ##### strokePen: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipen2d.html" class="tsd-signature-type">IPen2D</a>

    the pen to draw with

  - ##### viewRect: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>

    the series viewRect, used for translate and clipping

  - ##### clipRect: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>

  - ##### Optional lineDrawMode: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/elinedrawmode.html" class="tsd-signature-type">ELineDrawMode</a>

    whether to draw lines as a polyline, or disconnected lines

  #### Returns void

### drawRect

- drawRect(rect: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>, viewRect: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>, clipRect: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>, strokePen?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipen2d.html" class="tsd-signature-type">IPen2D</a>, fillBrush?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibrush2d.html" class="tsd-signature-type">IBrush2D</a>, bevelCorners?: boolean, strokeThickness?: number): void

<!-- -->

- description  
  Draws a Rect with optional fill and stroke

  #### Parameters

  - ##### rect: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>

    the Rect dimensions to draw

  - ##### viewRect: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>

    the series viewRect, used for translate and clipping

  - ##### clipRect: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>

  - ##### Optional strokePen: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipen2d.html" class="tsd-signature-type">IPen2D</a>

    the stroke pen to draw the outline with

  - ##### Optional fillBrush: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibrush2d.html" class="tsd-signature-type">IBrush2D</a>

    the fill brush to draw the fill with

  - ##### Optional bevelCorners: boolean

  - ##### Optional strokeThickness: number

  #### Returns void

### drawTriangleStrip

- drawTriangleStrip(xValues: number\[\], yValues: number\[\], viewRect: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>, clipRect: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>, fillBrush?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibrush2d.html" class="tsd-signature-type">IBrush2D</a>): void

<!-- -->

- description  
  Draws a triangle strip with the specified Pen

  #### Parameters

  - ##### xValues: number\[\]

    An array of x coordinates

  - ##### yValues: number\[\]

    An array of y coordinates

  - ##### viewRect: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>

    The series viewRect, used for translate

  - ##### clipRect: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>

    The rect, used for clipping

  - ##### Optional fillBrush: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibrush2d.html" class="tsd-signature-type">IBrush2D</a>

    The fill brush to draw the fill with

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
