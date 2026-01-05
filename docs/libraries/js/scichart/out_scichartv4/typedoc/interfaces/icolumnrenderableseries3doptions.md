<img src="out_scichartv4/typedoc/interfaces/icolumnrenderableseries3doptions_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icolumnrenderableseries3doptions.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [IColumnRenderableSeries3DOptions](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icolumnrenderableseries3doptions.html)

# Interface IColumnRenderableSeries3DOptions

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

### Hierarchy

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibaserenderableseries3doptions.html" class="tsd-signature-type">IBaseRenderableSeries3DOptions</a>
  - IColumnRenderableSeries3DOptions

## Index

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icolumnrenderableseries3doptions.html#datapointwidthx" class="tsd-kind-icon">dataPointWidthX</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icolumnrenderableseries3doptions.html#datapointwidthz" class="tsd-kind-icon">dataPointWidthZ</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icolumnrenderableseries3doptions.html#dataseries" class="tsd-kind-icon">dataSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icolumnrenderableseries3doptions.html#fill" class="tsd-kind-icon">fill</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icolumnrenderableseries3doptions.html#id" class="tsd-kind-icon">id</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icolumnrenderableseries3doptions.html#isvisible" class="tsd-kind-icon">isVisible</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icolumnrenderableseries3doptions.html#opacity" class="tsd-kind-icon">opacity</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icolumnrenderableseries3doptions.html#pointmarker" class="tsd-kind-icon">pointMarker</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icolumnrenderableseries3doptions.html#seriesname" class="tsd-kind-icon">seriesName</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icolumnrenderableseries3doptions.html#shininess" class="tsd-kind-icon">shininess</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icolumnrenderableseries3doptions.html#stroke" class="tsd-kind-icon">stroke</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icolumnrenderableseries3doptions.html#usemetadatacolors" class="tsd-kind-icon">useMetadataColors</a>

## Properties

### Optional dataPointWidthX

dataPointWidthX: number

Sets the data point width in X direction in Data Space.

### Optional dataPointWidthZ

dataPointWidthZ: number

Sets the data point width in Z direction in Data Space.

### Optional dataSeries

dataSeries: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basedataseries3d.html" class="tsd-signature-type">BaseDataSeries3D</a>

The [DataSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/basedataseries3d.html) which provides a datasource for this [IRenderableSeries3D](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries3d.html) to draw

### Optional fill

fill: string

Sets the column fill as an HTML Color Code. This will override the pointMarker fill if set.

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

pointMarker: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basemeshpointmarker3d.html" class="tsd-signature-type">BaseMeshPointMarker3D</a> \| <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tpointmarkerdefinition3d" class="tsd-signature-type">TPointMarkerDefinition3D</a>

A point marker [3D Point Marker](https://www.scichart.com/documentation/js/v4/typedoc/classes/basemeshpointmarker3d.html) which is used to draw columns at each Xyz data-point.

### Optional seriesName

seriesName: string

Series name

### Optional shininess

shininess: number

Gets or sets an optional Shininess factor, passed to 3D rendering shaders to make shiny objects

### Optional stroke

stroke: string

Gets or sets the stroke color as an HTML Color code

### Optional useMetadataColors

useMetadataColors: boolean

Sets flag to do coloring per data-point using metadata vertexColor

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
