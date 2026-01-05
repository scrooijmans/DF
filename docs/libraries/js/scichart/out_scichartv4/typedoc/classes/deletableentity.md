<img src="out_scichartv4/typedoc/classes/deletableentity_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/deletableentity.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [DeletableEntity](https://www.scichart.com/documentation/js/v4/typedoc/classes/deletableentity.html)

# Class DeletableEntity

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

Utility class responsible for adding its instance to [MemoryUsageHelper.objectRegistry](https://www.scichart.com/documentation/js/v4/typedoc/classes/memoryusagehelper.html#objectregistry) when Memory Usage Debug Mode is enabled

remarks  
It wraps the returned instance into a proxy object, so internal reference comparisons may fail

### Hierarchy

- DeletableEntity
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/chartmodifierbase.html" class="tsd-signature-type">ChartModifierBase</a>
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html" class="tsd-signature-type">CoordinateCalculatorBase</a>
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basepointseriesresampled.html" class="tsd-signature-type">BasePointSeriesResampled</a>
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/canvastexture.html" class="tsd-signature-type">CanvasTexture</a>
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/shadereffect.html" class="tsd-signature-type">ShaderEffect</a>
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/webglpen.html" class="tsd-signature-type">WebGlPen</a>
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/labelprovider.html" class="tsd-signature-type">LabelProvider</a>
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/texturemanager.html" class="tsd-signature-type">TextureManager</a>
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderdatatransform.html" class="tsd-signature-type">BaseRenderDataTransform</a>
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rubberbandsvgrect.html" class="tsd-signature-type">RubberBandSvgRect</a>
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartlegendbase.html" class="tsd-signature-type">SciChartLegendBase</a>
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartpiesurface.html" class="tsd-signature-type">SciChartPieSurface</a>
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basedataseries3d.html" class="tsd-signature-type">BaseDataSeries3D</a>
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basesceneentity3d.html" class="tsd-signature-type">BaseSceneEntity3D</a>
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries3d.html" class="tsd-signature-type">BaseRenderableSeries3D</a>
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/webglbrush.html" class="tsd-signature-type">WebGlBrush</a>
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/titlerendererbase.html" class="tsd-signature-type">TitleRendererBase</a>
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xynpointseriesresampled.html" class="tsd-signature-type">XyNPointSeriesResampled</a>
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/heatmaplegend.html" class="tsd-signature-type">HeatmapLegend</a>
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartoverview.html" class="tsd-signature-type">SciChartOverview</a>
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisrenderer.html" class="tsd-signature-type">AxisRenderer</a>
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries.html" class="tsd-signature-type">BaseRenderableSeries</a>
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationbase.html" class="tsd-signature-type">AnnotationBase</a>
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurfacebase.html" class="tsd-signature-type">SciChartSurfaceBase</a>
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basepointmarker.html" class="tsd-signature-type">BasePointMarker</a>
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basedataseries.html" class="tsd-signature-type">BaseDataSeries</a>
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/baseseriesdrawingprovider.html" class="tsd-signature-type">BaseSeriesDrawingProvider</a>
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fontkey.html" class="tsd-signature-type">FontKey</a>
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/webglrendercontext2d.html" class="tsd-signature-type">WebGlRenderContext2D</a>
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html" class="tsd-signature-type">AxisCore</a>
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/extremeresamplerhelper.html" class="tsd-signature-type">ExtremeResamplerHelper</a>

### Implements

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html" class="tsd-signature-type">IDeletable</a>

## Index

### Constructors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/deletableentity.html#constructor" class="tsd-kind-icon">constructor</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/deletableentity.html#delete" class="tsd-kind-icon">delete</a>

## Constructors

### constructor

- new DeletableEntity(entity?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html" class="tsd-signature-type">IDeletable</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/deletableentity.html" class="tsd-signature-type">DeletableEntity</a>

<!-- -->

- #### Parameters

  - ##### Optional entity: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html" class="tsd-signature-type">IDeletable</a>

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/deletableentity.html" class="tsd-signature-type">DeletableEntity</a>

## Methods

### Abstract delete

- delete(): void

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
