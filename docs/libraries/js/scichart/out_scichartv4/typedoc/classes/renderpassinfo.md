<img src="out_scichartv4/typedoc/classes/renderpassinfo_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassinfo.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [RenderPassInfo](https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassinfo.html)

# Class RenderPassInfo

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

### Hierarchy

- RenderPassInfo

## Index

### Constructors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassinfo.html#constructor" class="tsd-kind-icon">constructor</a>

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassinfo.html#relativeitems" class="tsd-kind-icon">relativeItems</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassinfo.html#renderableseriesarray" class="tsd-kind-icon">renderableSeriesArray</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassinfo.html#rendermap" class="tsd-kind-icon">rendermap</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassinfo.html#seriesviewrect" class="tsd-kind-icon">seriesViewRect</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassinfo.html#addrenderable" class="tsd-kind-icon">addRenderable</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassinfo.html#resolvenextto" class="tsd-kind-icon">resolveNextTo</a>

## Constructors

### constructor

- new RenderPassInfo(seriesCount: number, seriesViewRect: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassinfo.html" class="tsd-signature-type">RenderPassInfo</a>

<!-- -->

- #### Parameters

  - ##### seriesCount: number

  - ##### seriesViewRect: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassinfo.html" class="tsd-signature-type">RenderPassInfo</a>

## Properties

### Readonly relativeItems

relativeItems: { drawFunction: () =\> void; renderable: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iorderedrenderable.html" class="tsd-signature-type">IOrderedRenderable</a>; tries: number }\[\] = \[\]

### renderableSeriesArray

renderableSeriesArray: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries.html" class="tsd-signature-type">IRenderableSeries</a>\[\] = \[\]

### Readonly rendermap

rendermap: Map\<number, Map\<number, TRenderable\[\]\>\> = new Map\<number, Map\<number, TRenderable\[\]\>\>()

### seriesViewRect

seriesViewRect: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>

## Methods

### addRenderable

- addRenderable(renderable: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iorderedrenderable.html" class="tsd-signature-type">IOrderedRenderable</a>, drawFunction: () =\> void): void

<!-- -->

- #### Parameters

  - ##### renderable: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iorderedrenderable.html" class="tsd-signature-type">IOrderedRenderable</a>

  - ##### drawFunction: () =\> void

    - - (): void

      <!-- -->

      - #### Returns void

  #### Returns void

### resolveNextTo

- resolveNextTo(): void

<!-- -->

- #### Returns void

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
