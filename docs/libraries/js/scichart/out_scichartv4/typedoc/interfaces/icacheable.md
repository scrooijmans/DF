<img src="out_scichartv4/typedoc/interfaces/icacheable_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icacheable.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [ICacheable](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icacheable.html)

# Interface ICacheable

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

A type which implements [ICacheable](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icacheable.html) should allow managing the state of cached resources

remarks  
It should be used to prevent recreation of objects properties of which were unmodified, and to allow a correct handling of WebGL context loss by recreating WebGL resources.

### Hierarchy

- ICacheable
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ititlerenderer.html" class="tsd-signature-type">ITitleRenderer</a>

### Implemented by

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basepointmarker.html" class="tsd-signature-type">BasePointMarker</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/crosspointmarker.html" class="tsd-signature-type">CrossPointMarker</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datelabelprovider.html" class="tsd-signature-type">DateLabelProvider</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/ellipsepointmarker.html" class="tsd-signature-type">EllipsePointMarker</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/labelproviderbase2d.html" class="tsd-signature-type">LabelProviderBase2D</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/logarithmiclabelprovider.html" class="tsd-signature-type">LogarithmicLabelProvider</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numericlabelprovider.html" class="tsd-signature-type">NumericLabelProvider</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/radianlabelprovider.html" class="tsd-signature-type">RadianLabelProvider</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/smartdatelabelprovider.html" class="tsd-signature-type">SmartDateLabelProvider</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/spritepointmarker.html" class="tsd-signature-type">SpritePointMarker</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/squarepointmarker.html" class="tsd-signature-type">SquarePointMarker</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/textlabelprovider.html" class="tsd-signature-type">TextLabelProvider</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/trianglepointmarker.html" class="tsd-signature-type">TrianglePointMarker</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xpointmarker.html" class="tsd-signature-type">XPointMarker</a>

## Index

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icacheable.html#invalidatecache" class="tsd-kind-icon">invalidateCache</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icacheable.html#resetcache" class="tsd-kind-icon">resetCache</a>

## Methods

### invalidateCache

- invalidateCache(): void

<!-- -->

- Deletes native (WebAssembly) memory used by a cached resource and removes reference to it, forcing the cached object to be recreated next time it is fetched.

  remarks  
  Call .invalidateCache() to delete currently cached value

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
