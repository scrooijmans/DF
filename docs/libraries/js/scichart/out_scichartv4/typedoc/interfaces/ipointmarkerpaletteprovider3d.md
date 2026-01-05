<img src="out_scichartv4/typedoc/interfaces/ipointmarkerpaletteprovider3d_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmarkerpaletteprovider3d.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [IPointMarkerPaletteProvider3D](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmarkerpaletteprovider3d.html)

# Interface IPointMarkerPaletteProvider3D

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

The IPointMarkerPaletteProvider3D interface allows you to perform per-point paletting or coloring of series or data-points in 3D Scatter Charts and JavaScript 3D chart types which have a point-marker

### Hierarchy

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipaletteprovider3d.html" class="tsd-signature-type">IPaletteProvider3D</a>
  - IPointMarkerPaletteProvider3D

## Index

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmarkerpaletteprovider3d.html#onattached" class="tsd-kind-icon">onAttached</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmarkerpaletteprovider3d.html#ondetached" class="tsd-kind-icon">onDetached</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmarkerpaletteprovider3d.html#overridecolorabgr" class="tsd-kind-icon">overrideColorAbgr</a>

## Methods

### onAttached

- onAttached(parentSeries: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries3d.html" class="tsd-signature-type">IRenderableSeries3D</a>): void

<!-- -->

- Called when the PaletteProvider3D instance is attached to a 3D RenderableSeries. Use this to be notified when attached and keep a reference to the parent series

  #### Parameters

  - ##### parentSeries: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries3d.html" class="tsd-signature-type">IRenderableSeries3D</a>

  #### Returns void

### onDetached

- onDetached(): void

<!-- -->

- Called when the PaletteProvider3D instance is detached from a 3D RenderableSeries

  #### Returns void

### overrideColorAbgr

- overrideColorAbgr(xValue: number, yValue: number, zValue: number, index: number, metadata?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmetadata3d.html" class="tsd-signature-type">IPointMetadata3D</a>): number

<!-- -->

- Called by SciChart and may be used to override a 3D Point-marker in Scatter or Bubble 3D Charts on a per-point basis

  #### Parameters

  - ##### xValue: number

    the current XValue

  - ##### yValue: number

    the current YValue

  - ##### zValue: number

    the current ZValue

  - ##### index: number

    the current index to the data

  - ##### Optional metadata: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmetadata3d.html" class="tsd-signature-type">IPointMetadata3D</a>

    the current metadata

  #### Returns number

  an ABGR color code, e.g. e.g. 0xFFFF0000 would be red, or 'undefined' for default colouring

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
