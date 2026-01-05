<img src="out_scichartv4/typedoc/classes/seriesinfo3d_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo3d.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [SeriesInfo3D](https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo3d.html)

# Class SeriesInfo3D

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

SeriesInfo3D is a data-structure which provides enriched information about a hit-test operation in SciChart 3D. It's derived by calling [BaseRenderableSeries3D.hitTest](https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries3d.html#hittest). There is a class hierachy for [SeriesInfo3D](https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo3d.html) which is a different class depending on series type, e.g. 3D Scatter series has [XyzSeriesInfo3D](https://www.scichart.com/documentation/js/v4/typedoc/classes/xyzseriesinfo3d.html), 3D Surface mesh series has [SurfaceMeshSeriesInfo3D](https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshseriesinfo3d.html) etc.

### Hierarchy

- SeriesInfo3D
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshseriesinfo3d.html" class="tsd-signature-type">SurfaceMeshSeriesInfo3D</a>
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyzseriesinfo3d.html" class="tsd-signature-type">XyzSeriesInfo3D</a>

## Index

### Constructors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo3d.html#constructor" class="tsd-kind-icon">constructor</a>

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo3d.html#dataseriesname" class="tsd-kind-icon">dataSeriesName</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo3d.html#dataseriestype" class="tsd-kind-icon">dataSeriesType</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo3d.html#hittestinfo" class="tsd-kind-icon">hitTestInfo</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo3d.html#hitworldcoords" class="tsd-kind-icon">hitWorldCoords</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo3d.html#isempty" class="tsd-kind-icon">isEmpty</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo3d.html#ishit" class="tsd-kind-icon">isHit</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo3d.html#renderableseries" class="tsd-kind-icon">renderableSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo3d.html#xvalue" class="tsd-kind-icon">xValue</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo3d.html#yvalue" class="tsd-kind-icon">yValue</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo3d.html#zvalue" class="tsd-kind-icon">zValue</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo3d.html#equals" class="tsd-kind-icon">equals</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo3d.html#empty" class="tsd-kind-icon">empty</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo3d.html#isequal" class="tsd-kind-icon">isEqual</a>

## Constructors

### constructor

- new SeriesInfo3D(series: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries3d.html" class="tsd-signature-type">IRenderableSeries3D</a>, hitTestInfo: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/hittestinfo3d.html" class="tsd-signature-type">HitTestInfo3D</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo3d.html" class="tsd-signature-type">SeriesInfo3D</a>

<!-- -->

- #### Parameters

  - ##### series: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries3d.html" class="tsd-signature-type">IRenderableSeries3D</a>

  - ##### hitTestInfo: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/hittestinfo3d.html" class="tsd-signature-type">HitTestInfo3D</a>

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo3d.html" class="tsd-signature-type">SeriesInfo3D</a>

## Properties

### dataSeriesName

dataSeriesName: string

The name of the associated [DataSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/basedataseries3d.html)

### dataSeriesType

dataSeriesType: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/edataseriestype3d.html" class="tsd-signature-type">EDataSeriesType3D</a>

Data series type

### Protected hitTestInfo

hitTestInfo: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/hittestinfo3d.html" class="tsd-signature-type">HitTestInfo3D</a>

### hitWorldCoords

hitWorldCoords: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/vector3.html" class="tsd-signature-type">Vector3</a>

When [isHit](https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo3d.html#ishit) is true, this is the [Vector3](https://www.scichart.com/documentation/js/v4/typedoc/classes/vector3.html) 3D World Coordinates of the data-point that was hit

### isEmpty

isEmpty: boolean = false

When true, the [SeriesInfo3D](https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo3d.html) is empty

### isHit

isHit: boolean

When true, the result of the hit-test operation is hit (mouse over data-point)

### renderableSeries

renderableSeries: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries3d.html" class="tsd-signature-type">IRenderableSeries3D</a>

The associated [RenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries3d.html)

### xValue

xValue: number

When [isHit](https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo3d.html#ishit) is true, this is the X-value of the data-point in the [XyzDataSeries3D](https://www.scichart.com/documentation/js/v4/typedoc/classes/xyzdataseries3d.html) that was hit

### yValue

yValue: number

When [isHit](https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo3d.html#ishit) is true, this is the Y-value of the data-point in the [XyzDataSeries3D](https://www.scichart.com/documentation/js/v4/typedoc/classes/xyzdataseries3d.html) that was hit

### zValue

zValue: number

When [isHit](https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo3d.html#ishit) is true, this is the Z-value of the data-point in the [XyzDataSeries3D](https://www.scichart.com/documentation/js/v4/typedoc/classes/xyzdataseries3d.html) that was hit

## Methods

### equals

- equals(info: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo3d.html" class="tsd-signature-type">SeriesInfo3D</a>): boolean

<!-- -->

- #### Parameters

  - ##### info: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo3d.html" class="tsd-signature-type">SeriesInfo3D</a>

  #### Returns boolean

### Static empty

- empty(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo3d.html" class="tsd-signature-type">SeriesInfo3D</a>

<!-- -->

- The default empty [HitTestInfo3D](https://www.scichart.com/documentation/js/v4/typedoc/classes/hittestinfo3d.html) instance

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo3d.html" class="tsd-signature-type">SeriesInfo3D</a>

### Static isEqual

- isEqual(info1: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo3d.html" class="tsd-signature-type">SeriesInfo3D</a>, info2: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo3d.html" class="tsd-signature-type">SeriesInfo3D</a>): boolean

<!-- -->

- #### Parameters

  - ##### info1: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo3d.html" class="tsd-signature-type">SeriesInfo3D</a>

  - ##### info2: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo3d.html" class="tsd-signature-type">SeriesInfo3D</a>

  #### Returns boolean

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
