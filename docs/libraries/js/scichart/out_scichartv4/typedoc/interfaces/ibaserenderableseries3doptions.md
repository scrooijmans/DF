<img src="out_scichartv4/typedoc/interfaces/ibaserenderableseries3doptions_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibaserenderableseries3doptions.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [IBaseRenderableSeries3DOptions](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibaserenderableseries3doptions.html)

# Interface IBaseRenderableSeries3DOptions

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

Options to pass to the [BaseRenderableSeries3D](https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries3d.html) constructor

### Hierarchy

- IBaseRenderableSeries3DOptions
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icolumnrenderableseries3doptions.html" class="tsd-signature-type">IColumnRenderableSeries3DOptions</a>
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointlinerenderableseries3doptions.html" class="tsd-signature-type">IPointLineRenderableSeries3DOptions</a>
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/isurfacemeshrenderableseries3doptions.html" class="tsd-signature-type">ISurfaceMeshRenderableSeries3DOptions</a>

## Index

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibaserenderableseries3doptions.html#dataseries" class="tsd-kind-icon">dataSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibaserenderableseries3doptions.html#id" class="tsd-kind-icon">id</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibaserenderableseries3doptions.html#isvisible" class="tsd-kind-icon">isVisible</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibaserenderableseries3doptions.html#opacity" class="tsd-kind-icon">opacity</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibaserenderableseries3doptions.html#pointmarker" class="tsd-kind-icon">pointMarker</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibaserenderableseries3doptions.html#seriesname" class="tsd-kind-icon">seriesName</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibaserenderableseries3doptions.html#shininess" class="tsd-kind-icon">shininess</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibaserenderableseries3doptions.html#stroke" class="tsd-kind-icon">stroke</a>

## Properties

### Optional dataSeries

dataSeries: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basedataseries3d.html" class="tsd-signature-type">BaseDataSeries3D</a>

The [DataSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/basedataseries3d.html) which provides a datasource for this [IRenderableSeries3D](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries3d.html) to draw

### Optional Readonly id

id: string

A unique Id for the [IRenderableSeries3D](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries3d.html)

### Optional isVisible

isVisible: boolean

Gets or sets whether the [IRenderableSeries3D](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries3d.html) is visible or not

### Optional opacity

opacity: number

Gets or sets the opacity of the [IRenderableSeries3D](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries3d.html)

remarks  
Value range 0.0 to 1.0. Default = 1.

### Optional pointMarker

pointMarker: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmarker3d.html" class="tsd-signature-type">IPointMarker3D</a> \| <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tpointmarkerdefinition3d" class="tsd-signature-type">TPointMarkerDefinition3D</a>

A [3D Point Marker](https://www.scichart.com/documentation/js/v4/typedoc/classes/basepointmarker3d.html) which is used to draw an optional 3D point-marker at each Xyz data-point. Applicable to some series types only

### Optional seriesName

seriesName: string

Series name

### Optional shininess

shininess: number

Gets or sets an optional Shininess factor, passed to 3D rendering shaders to make shiny objects

### Optional stroke

stroke: string

Gets or sets the stroke color as an HTML Color code

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
