<img src="out_scichartv4/typedoc/interfaces/ipen2d_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipen2d.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [IPen2D](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipen2d.html)

# Interface IPen2D

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

Defines the interface to a Brush, used for drawing filled areas, polygons and rectangles on [IRenderContext2D](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irendercontext2d.html)

### Hierarchy

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html" class="tsd-signature-type">IDeletable</a>
  - IPen2D

### Implemented by

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/webglpen.html" class="tsd-signature-type">WebGlPen</a>

## Index

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipen2d.html#delete" class="tsd-kind-icon">delete</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipen2d.html#getpentype" class="tsd-kind-icon">getPenType</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipen2d.html#setopacity" class="tsd-kind-icon">setOpacity</a>

## Methods

### delete

- delete(): void

<!-- -->

- Deletes native (WebAssembly) memory used by this type, after which it cannot be used.

  remarks  
  Call .delete() before finishing with the object to ensure that WebAssmembly memory leaks do not occur.

  All elements within SciChart's High Performance <a href="https://www.scichart.com/javascript-chart-features" class="external">Realtime JavaScript Charts</a> which implement [IDeletable](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html) must be deleted manually to free native (WebAssembly) memory

  #### Returns void

### getPenType

- getPenType(): EDrawingTypes

<!-- -->

- Gets the type of the pen. See {@link EDrawingTypes} for list of values

  #### Returns EDrawingTypes

### setOpacity

- setOpacity(opacity: number): void

<!-- -->

- Allows setting an opacity override for the pen. This will be applied the next time the pen is used to draw

  #### Parameters

  - ##### opacity: number

    An opacity number from 0..1

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
