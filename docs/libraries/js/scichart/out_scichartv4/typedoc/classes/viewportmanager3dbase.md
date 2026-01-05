<img src="out_scichartv4/typedoc/classes/viewportmanager3dbase_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/viewportmanager3dbase.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [ViewportManager3DBase](https://www.scichart.com/documentation/js/v4/typedoc/classes/viewportmanager3dbase.html)

# Class ViewportManager3DBase

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

The Viewport Manager performs certain functions such as axis ranging and viewport manipulation on a [SciChart3DSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html)

### Hierarchy

- ViewportManager3DBase
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/defaultviewportmanager3d.html" class="tsd-signature-type">DefaultViewportManager3D</a>

## Index

### Constructors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/viewportmanager3dbase.html#constructor" class="tsd-kind-icon">constructor</a>

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/viewportmanager3dbase.html#height" class="tsd-kind-icon">height</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/viewportmanager3dbase.html#isattached" class="tsd-kind-icon">isAttached</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/viewportmanager3dbase.html#parentsurface" class="tsd-kind-icon">parentSurface</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/viewportmanager3dbase.html#width" class="tsd-kind-icon">width</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/viewportmanager3dbase.html#attachscichartsurface" class="tsd-kind-icon">attachSciChartSurface</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/viewportmanager3dbase.html#calculateautorange" class="tsd-kind-icon">calculateAutoRange</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/viewportmanager3dbase.html#detachscichartsurface" class="tsd-kind-icon">detachSciChartSurface</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/viewportmanager3dbase.html#setsize" class="tsd-kind-icon">setSize</a>

## Constructors

### constructor

- new ViewportManager3DBase(width: number, height: number): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/viewportmanager3dbase.html" class="tsd-signature-type">ViewportManager3DBase</a>

<!-- -->

- #### Parameters

  - ##### width: number

  - ##### height: number

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/viewportmanager3dbase.html" class="tsd-signature-type">ViewportManager3DBase</a>

## Properties

### height

height: number

### isAttached

isAttached: boolean

When true, currently attached to a [SciChart3DSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html)

### parentSurface

parentSurface: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html" class="tsd-signature-type">SciChart3DSurface</a>

The parent [SciChart3DSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html) when attached

### width

width: number

## Methods

### attachSciChartSurface

- attachSciChartSurface(scs: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html" class="tsd-signature-type">SciChart3DSurface</a>): void

<!-- -->

- Called when attached to a [SciChart3DSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html)

  #### Parameters

  - ##### scs: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html" class="tsd-signature-type">SciChart3DSurface</a>

    The [SciChart3DSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html)

  #### Returns void

### calculateAutoRange

- calculateAutoRange(axis: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase3d.html" class="tsd-signature-type">AxisBase3D</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

<!-- -->

- Calculates the visibleRange for an [AxisBase3D](https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase3d.html) depending on the current parameters such as [AxisCore.autoRange](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#autorange) and data on the axis.

  #### Parameters

  - ##### axis: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase3d.html" class="tsd-signature-type">AxisBase3D</a>

    The [AxisBase3D](https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase3d.html) we are calculating for

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

  The auto-fitted range

### detachSciChartSurface

- detachSciChartSurface(): void

<!-- -->

- Called when detached from a [SciChart3DSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html)

  #### Returns void

### setSize

- setSize(width: number, height: number): void

<!-- -->

- #### Parameters

  - ##### width: number

  - ##### height: number

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
