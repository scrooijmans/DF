<img src="out_scichartv4/typedoc/interfaces/ititlerenderer_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ititlerenderer.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [ITitleRenderer](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ititlerenderer.html)

# Interface ITitleRenderer

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

### Hierarchy

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html" class="tsd-signature-type">IDeletable</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icacheable.html" class="tsd-signature-type">ICacheable</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/inotifyondpichanged.html" class="tsd-signature-type">INotifyOnDpiChanged</a>
  - ITitleRenderer
    - <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icharttitlerenderer.html" class="tsd-signature-type">IChartTitleRenderer</a>

### Implemented by

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axistitlerenderer.html" class="tsd-signature-type">AxisTitleRenderer</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/charttitlerenderer.html" class="tsd-signature-type">ChartTitleRenderer</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/titlerendererbase.html" class="tsd-signature-type">TitleRendererBase</a>

## Index

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ititlerenderer.html#viewrect" class="tsd-kind-icon">viewRect</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ititlerenderer.html#delete" class="tsd-kind-icon">delete</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ititlerenderer.html#draw" class="tsd-kind-icon">draw</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ititlerenderer.html#invalidatecache" class="tsd-kind-icon">invalidateCache</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ititlerenderer.html#layout" class="tsd-kind-icon">layout</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ititlerenderer.html#measure" class="tsd-kind-icon">measure</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ititlerenderer.html#ondpichanged" class="tsd-kind-icon">onDpiChanged</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ititlerenderer.html#resetcache" class="tsd-kind-icon">resetCache</a>

## Properties

### viewRect

viewRect: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>

## Methods

### delete

- delete(): void

<!-- -->

- Deletes native (WebAssembly) memory used by this type, after which it cannot be used.

  remarks  
  Call .delete() before finishing with the object to ensure that WebAssmembly memory leaks do not occur.

  All elements within SciChart's High Performance <a href="https://www.scichart.com/javascript-chart-features" class="external">Realtime JavaScript Charts</a> which implement [IDeletable](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html) must be deleted manually to free native (WebAssembly) memory

  #### Returns void

### draw

- draw(...params: any\[\]): void

<!-- -->

- #### Parameters

  - ##### Rest ...params: any\[\]

  #### Returns void

### invalidateCache

- invalidateCache(): void

<!-- -->

- Deletes native (WebAssembly) memory used by a cached resource and removes reference to it, forcing the cached object to be recreated next time it is fetched.

  remarks  
  Call .invalidateCache() to delete currently cached value

  #### Returns void

### layout

- layout(viewRectOrigin: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>): void

<!-- -->

- #### Parameters

  - ##### viewRectOrigin: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>

  #### Returns void

### measure

- measure(...params: any\[\]): void

<!-- -->

- #### Parameters

  - ##### Rest ...params: any\[\]

  #### Returns void

### Protected onDpiChanged

- onDpiChanged(args: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tdpichangedeventargs" class="tsd-signature-type">TDpiChangedEventArgs</a>): void

<!-- -->

- Called when the Dpi changes in the browser. This could be due to user zooming the browser, or changing DPI settings in Windows, or moving the browser containing SciChart to another monitor

  #### Parameters

  - ##### args: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tdpichangedeventargs" class="tsd-signature-type">TDpiChangedEventArgs</a>

    The [TDpiChangedEventArgs](https://www.scichart.com/documentation/js/v4/typedoc/index.html#tdpichangedeventargs) containing info about the Dpi Changed event

  #### Returns void

### resetCache

- resetCache(): void

<!-- -->

- Deletes native (WebAssembly) memory used by a cached resource and removes reference to it. And resets properties to the default values

  remarks  
  Call .resetCache() to clear the cached value and prevent it from being recreated using saved properties.

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
