<img src="out_scichartv4/typedoc/interfaces/ipointlinerenderableseries3doptions_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointlinerenderableseries3doptions.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [IPointLineRenderableSeries3DOptions](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointlinerenderableseries3doptions.html)

# Interface IPointLineRenderableSeries3DOptions

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

### Hierarchy

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibaserenderableseries3doptions.html" class="tsd-signature-type">IBaseRenderableSeries3DOptions</a>
  - IPointLineRenderableSeries3DOptions

## Index

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointlinerenderableseries3doptions.html#dataseries" class="tsd-kind-icon">dataSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointlinerenderableseries3doptions.html#id" class="tsd-kind-icon">id</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointlinerenderableseries3doptions.html#isantialiased" class="tsd-kind-icon">isAntiAliased</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointlinerenderableseries3doptions.html#islinestrip" class="tsd-kind-icon">isLineStrip</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointlinerenderableseries3doptions.html#isvisible" class="tsd-kind-icon">isVisible</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointlinerenderableseries3doptions.html#opacity" class="tsd-kind-icon">opacity</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointlinerenderableseries3doptions.html#pointmarker" class="tsd-kind-icon">pointMarker</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointlinerenderableseries3doptions.html#seriesname" class="tsd-kind-icon">seriesName</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointlinerenderableseries3doptions.html#shininess" class="tsd-kind-icon">shininess</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointlinerenderableseries3doptions.html#stroke" class="tsd-kind-icon">stroke</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointlinerenderableseries3doptions.html#strokethickness" class="tsd-kind-icon">strokeThickness</a>

## Properties

### Optional dataSeries

dataSeries: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basedataseries3d.html" class="tsd-signature-type">BaseDataSeries3D</a>

The [DataSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/basedataseries3d.html) which provides a datasource for this [IRenderableSeries3D](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries3d.html) to draw

### Optional Readonly id

id: string

A unique Id for the [IRenderableSeries3D](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries3d.html)

### Optional isAntiAliased

isAntiAliased: boolean

When true, the line will be anti-aliased. Default true

### Optional isLineStrip

isLineStrip: boolean

When true, creates a polyline, else each pair of points in [XyzDataSeries3D](https://www.scichart.com/documentation/js/v4/typedoc/classes/xyzdataseries3d.html) will be connected with a line with a break before the next pair. Default true

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

### Optional strokeThickness

strokeThickness: number

The strokethickness of the 3D line series in world units. Set a strokeThickness of 0 to hide the line

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
