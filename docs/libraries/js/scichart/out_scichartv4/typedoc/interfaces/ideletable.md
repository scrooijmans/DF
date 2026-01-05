<img src="out_scichartv4/typedoc/interfaces/ideletable_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [IDeletable](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html)

# Interface IDeletable

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

A type which implements [IDeletable](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html) must be deleted manually to free webassembly memory

remarks  
Call .delete() before finishing with the object to ensure that WebAssmembly memory leaks do not occur.

All elements within SciChart's High Performance <a href="https://www.scichart.com/javascript-chart-features" class="external">Realtime JavaScript Charts</a> which implement [IDeletable](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html) must be deleted manually to free native (WebAssembly) memory

### Hierarchy

- IDeletable
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idataseries.html" class="tsd-signature-type">IDataSeries</a>
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ichartmodifierbase.html" class="tsd-signature-type">IChartModifierBase</a>
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmarker.html" class="tsd-signature-type">IPointMarker</a>
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipen2d.html" class="tsd-signature-type">IPen2D</a>
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibrush2d.html" class="tsd-signature-type">IBrush2D</a>
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irendercontext2d.html" class="tsd-signature-type">IRenderContext2D</a>
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iseriesdrawingprovider.html" class="tsd-signature-type">ISeriesDrawingProvider</a>
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderdatatransform.html" class="tsd-signature-type">IRenderDataTransform</a>
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ieventsubscriptionitem.html" class="tsd-signature-type">IEventSubscriptionItem</a>
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idataseries3d.html" class="tsd-signature-type">IDataSeries3D</a>
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibasesceneentity.html" class="tsd-signature-type">IBaseSceneEntity</a>
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries3d.html" class="tsd-signature-type">IRenderableSeries3D</a>
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ititlerenderer.html" class="tsd-signature-type">ITitleRenderer</a>
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ifilterbase.html" class="tsd-signature-type">IFilterBase</a>
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iannotation.html" class="tsd-signature-type">IAnnotation</a>
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iscichartsurfacebase.html" class="tsd-signature-type">ISciChartSurfaceBase</a>
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries.html" class="tsd-signature-type">IRenderableSeries</a>

### Implemented by

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationbase.html" class="tsd-signature-type">AnnotationBase</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationhovermodifier.html" class="tsd-signature-type">AnnotationHoverModifier</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/arcannotation.html" class="tsd-signature-type">ArcAnnotation</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/arcannotationbase.html" class="tsd-signature-type">ArcAnnotationBase</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase2d.html" class="tsd-signature-type">AxisBase2D</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase2d.html" class="tsd-signature-type">AxisBase2D</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase3d.html" class="tsd-signature-type">AxisBase3D</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase3d.html" class="tsd-signature-type">AxisBase3D</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html" class="tsd-signature-type">AxisCore</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html" class="tsd-signature-type">AxisCore</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscubeentity.html" class="tsd-signature-type">AxisCubeEntity</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axismarkerannotation.html" class="tsd-signature-type">AxisMarkerAnnotation</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisrenderer.html" class="tsd-signature-type">AxisRenderer</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisrenderer.html" class="tsd-signature-type">AxisRenderer</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axistitlerenderer.html" class="tsd-signature-type">AxisTitleRenderer</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/bandseriesdrawingprovider.html" class="tsd-signature-type">BandSeriesDrawingProvider</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basebandrenderableseries.html" class="tsd-signature-type">BaseBandRenderableSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basedataseries.html" class="tsd-signature-type">BaseDataSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basedataseries3d.html" class="tsd-signature-type">BaseDataSeries3D</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basegriddataseries3d.html" class="tsd-signature-type">BaseGridDataSeries3D</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/baseheatmaprenderableseries.html" class="tsd-signature-type">BaseHeatmapRenderableSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/baselinerenderableseries.html" class="tsd-signature-type">BaseLineRenderableSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basemountainrenderableseries.html" class="tsd-signature-type">BaseMountainRenderableSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/baseohlcrenderableseries.html" class="tsd-signature-type">BaseOhlcRenderableSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basepointmarker.html" class="tsd-signature-type">BasePointMarker</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basepointseriesresampled.html" class="tsd-signature-type">BasePointSeriesResampled</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basepointseriesresampled.html" class="tsd-signature-type">BasePointSeriesResampled</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderdatatransform.html" class="tsd-signature-type">BaseRenderDataTransform</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderdatatransformjs.html" class="tsd-signature-type">BaseRenderDataTransformJS</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries.html" class="tsd-signature-type">BaseRenderableSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries3d.html" class="tsd-signature-type">BaseRenderableSeries3D</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basesceneentity3d.html" class="tsd-signature-type">BaseSceneEntity3D</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/baseseriesdrawingprovider.html" class="tsd-signature-type">BaseSeriesDrawingProvider</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basestackedmountainrenderableseries.html" class="tsd-signature-type">BaseStackedMountainRenderableSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basestackedrenderableseries.html" class="tsd-signature-type">BaseStackedRenderableSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/batchrendercontext.html" class="tsd-signature-type">BatchRenderContext</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/bezierrenderdatatransform.html" class="tsd-signature-type">BezierRenderDataTransform</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/boxannotation.html" class="tsd-signature-type">BoxAnnotation</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/boxplotdataseries.html" class="tsd-signature-type">BoxPlotDataSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/boxplotseriesdrawingprovider.html" class="tsd-signature-type">BoxPlotSeriesDrawingProvider</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/bubbleseriesdrawingprovider.html" class="tsd-signature-type">BubbleSeriesDrawingProvider</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/canvastexture.html" class="tsd-signature-type">CanvasTexture</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/canvastexture.html" class="tsd-signature-type">CanvasTexture</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/categoryaxis.html" class="tsd-signature-type">CategoryAxis</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/categoryaxis.html" class="tsd-signature-type">CategoryAxis</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/categoryaxisbase.html" class="tsd-signature-type">CategoryAxisBase</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/categoryaxisbase.html" class="tsd-signature-type">CategoryAxisBase</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/categorycoordinatecalculator.html" class="tsd-signature-type">CategoryCoordinateCalculator</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/categorycoordinatecalculator.html" class="tsd-signature-type">CategoryCoordinateCalculator</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/chartmodifierbase.html" class="tsd-signature-type">ChartModifierBase</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/chartmodifierbase2d.html" class="tsd-signature-type">ChartModifierBase2D</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/chartmodifierbase3d.html" class="tsd-signature-type">ChartModifierBase3D</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/charttitlerenderer.html" class="tsd-signature-type">ChartTitleRenderer</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/columnrenderableseries3d.html" class="tsd-signature-type">ColumnRenderableSeries3D</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/columnsceneentity.html" class="tsd-signature-type">ColumnSceneEntity</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/columnseriesdrawingprovider.html" class="tsd-signature-type">ColumnSeriesDrawingProvider</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html" class="tsd-signature-type">CoordinateCalculatorBase</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html" class="tsd-signature-type">CoordinateCalculatorBase</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/crosspointmarker.html" class="tsd-signature-type">CrossPointMarker</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/crosshairlinessceneentity.html" class="tsd-signature-type">CrosshairLinesSceneEntity</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/cursormodifier.html" class="tsd-signature-type">CursorModifier</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/cursortooltipsvgannotation.html" class="tsd-signature-type">CursorTooltipSvgAnnotation</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/customannotation.html" class="tsd-signature-type">CustomAnnotation</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/customchartmodifier2d.html" class="tsd-signature-type">CustomChartModifier2D</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/customchartmodifier3d.html" class="tsd-signature-type">CustomChartModifier3D</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datapointselectionmodifier.html" class="tsd-signature-type">DataPointSelectionModifier</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datelabelprovider.html" class="tsd-signature-type">DateLabelProvider</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datetimenumericaxis.html" class="tsd-signature-type">DateTimeNumericAxis</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datetimenumericaxis.html" class="tsd-signature-type">DateTimeNumericAxis</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/deletableentity.html" class="tsd-signature-type">DeletableEntity</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/domannotationbase.html" class="tsd-signature-type">DomAnnotationBase</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/ellipsepointmarker.html" class="tsd-signature-type">EllipsePointMarker</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/errorseriesdrawingprovider.html" class="tsd-signature-type">ErrorSeriesDrawingProvider</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/extremeresamplerhelper.html" class="tsd-signature-type">ExtremeResamplerHelper</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/extremeresamplerhelper.html" class="tsd-signature-type">ExtremeResamplerHelper</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastbandrenderableseries.html" class="tsd-signature-type">FastBandRenderableSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastboxplotrenderableseries.html" class="tsd-signature-type">FastBoxPlotRenderableSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastbubblerenderableseries.html" class="tsd-signature-type">FastBubbleRenderableSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastcandlestickrenderableseries.html" class="tsd-signature-type">FastCandlestickRenderableSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastcolumnrenderableseries.html" class="tsd-signature-type">FastColumnRenderableSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fasterrorbarsrenderableseries.html" class="tsd-signature-type">FastErrorBarsRenderableSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastimpulserenderableseries.html" class="tsd-signature-type">FastImpulseRenderableSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastlinerenderableseries.html" class="tsd-signature-type">FastLineRenderableSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastlinesegmentrenderableseries.html" class="tsd-signature-type">FastLineSegmentRenderableSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastmountainrenderableseries.html" class="tsd-signature-type">FastMountainRenderableSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastohlcrenderableseries.html" class="tsd-signature-type">FastOhlcRenderableSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastrectanglerenderableseries.html" class="tsd-signature-type">FastRectangleRenderableSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fasttextrenderableseries.html" class="tsd-signature-type">FastTextRenderableSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fasttrianglerenderableseries.html" class="tsd-signature-type">FastTriangleRenderableSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/flippedcategorycoordinatecalculator.html" class="tsd-signature-type">FlippedCategoryCoordinateCalculator</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/flippedcategorycoordinatecalculator.html" class="tsd-signature-type">FlippedCategoryCoordinateCalculator</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/flippednumericcoordinatecalculator.html" class="tsd-signature-type">FlippedNumericCoordinateCalculator</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/flippednumericcoordinatecalculator.html" class="tsd-signature-type">FlippedNumericCoordinateCalculator</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fontkey.html" class="tsd-signature-type">FontKey</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fontkey.html" class="tsd-signature-type">FontKey</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/gizmoentity.html" class="tsd-signature-type">GizmoEntity</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/gloweffect.html" class="tsd-signature-type">GlowEffect</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/gloweffect.html" class="tsd-signature-type">GlowEffect</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/heatmaplegend.html" class="tsd-signature-type">HeatmapLegend</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/heatmaplegend.html" class="tsd-signature-type">HeatmapLegend</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/heightseriesdrawingprovider.html" class="tsd-signature-type">HeightSeriesDrawingProvider</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/hlccustomfilter.html" class="tsd-signature-type">HlcCustomFilter</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/hlcdataseries.html" class="tsd-signature-type">HlcDataSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/hlcfilterbase.html" class="tsd-signature-type">HlcFilterBase</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/hlcscaleoffsetfilter.html" class="tsd-signature-type">HlcScaleOffsetFilter</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/horizontallineannotation.html" class="tsd-signature-type">HorizontalLineAnnotation</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/htmlcustomannotation.html" class="tsd-signature-type">HtmlCustomAnnotation</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/htmltextannotation.html" class="tsd-signature-type">HtmlTextAnnotation</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/labelprovider.html" class="tsd-signature-type">LabelProvider</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/labelproviderbase2d.html" class="tsd-signature-type">LabelProviderBase2D</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/legendmodifier.html" class="tsd-signature-type">LegendModifier</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/legendmodifier3d.html" class="tsd-signature-type">LegendModifier3D</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/lineannotation.html" class="tsd-signature-type">LineAnnotation</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/linearrowannotation.html" class="tsd-signature-type">LineArrowAnnotation</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/linesegmentseriesdrawingprovider.html" class="tsd-signature-type">LineSegmentSeriesDrawingProvider</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/lineseriesdrawingprovider.html" class="tsd-signature-type">LineSeriesDrawingProvider</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/logarithmicaxis.html" class="tsd-signature-type">LogarithmicAxis</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/logarithmicaxis.html" class="tsd-signature-type">LogarithmicAxis</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/logarithmiccoordinatecalculator.html" class="tsd-signature-type">LogarithmicCoordinateCalculator</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/logarithmiccoordinatecalculator.html" class="tsd-signature-type">LogarithmicCoordinateCalculator</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/logarithmiclabelprovider.html" class="tsd-signature-type">LogarithmicLabelProvider</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/manuallegend.html" class="tsd-signature-type">ManualLegend</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/manuallegend.html" class="tsd-signature-type">ManualLegend</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/mountainseriesdrawingprovider.html" class="tsd-signature-type">MountainSeriesDrawingProvider</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/mousewheelzoommodifier.html" class="tsd-signature-type">MouseWheelZoomModifier</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/mousewheelzoommodifier3d.html" class="tsd-signature-type">MouseWheelZoomModifier3D</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/nativetextannotation.html" class="tsd-signature-type">NativeTextAnnotation</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/nonuniformheatmapdrawingprovider.html" class="tsd-signature-type">NonUniformHeatmapDrawingProvider</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/nonuniformheatmaprenderableseries.html" class="tsd-signature-type">NonUniformHeatmapRenderableSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numericaxis.html" class="tsd-signature-type">NumericAxis</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numericaxis.html" class="tsd-signature-type">NumericAxis</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numericaxis3d.html" class="tsd-signature-type">NumericAxis3D</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numericaxis3d.html" class="tsd-signature-type">NumericAxis3D</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numericcoordinatecalculator.html" class="tsd-signature-type">NumericCoordinateCalculator</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numericcoordinatecalculator.html" class="tsd-signature-type">NumericCoordinateCalculator</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numericlabelprovider.html" class="tsd-signature-type">NumericLabelProvider</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/ohlcbaserenderdatatransform.html" class="tsd-signature-type">OhlcBaseRenderDataTransform</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/ohlccustomfilter.html" class="tsd-signature-type">OhlcCustomFilter</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/ohlcdataseries.html" class="tsd-signature-type">OhlcDataSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/ohlcfilterbase.html" class="tsd-signature-type">OhlcFilterBase</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/ohlcpointseriesresampled.html" class="tsd-signature-type">OhlcPointSeriesResampled</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/ohlcpointseriesresampled.html" class="tsd-signature-type">OhlcPointSeriesResampled</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/ohlcscaleoffsetfilter.html" class="tsd-signature-type">OhlcScaleOffsetFilter</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/ohlcseriesdrawingprovider.html" class="tsd-signature-type">OhlcSeriesDrawingProvider</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/orbitmodifier3d.html" class="tsd-signature-type">OrbitModifier3D</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/overviewcustomresizableannotation.html" class="tsd-signature-type">OverviewCustomResizableAnnotation</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/overviewrangeselectionmodifier.html" class="tsd-signature-type">OverviewRangeSelectionModifier</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/pielabelprovider.html" class="tsd-signature-type">PieLabelProvider</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/pinchzoommodifier.html" class="tsd-signature-type">PinchZoomModifier</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/pinchzoommodifier3d.html" class="tsd-signature-type">PinchZoomModifier3D</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/pointline3dsceneentity.html" class="tsd-signature-type">PointLine3DSceneEntity</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/pointlinerenderableseries3d.html" class="tsd-signature-type">PointLineRenderableSeries3D</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/pointmarkerdrawingprovider.html" class="tsd-signature-type">PointMarkerDrawingProvider</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/pointereventsmediatormodifier.html" class="tsd-signature-type">PointerEventsMediatorModifier</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polararcannotation.html" class="tsd-signature-type">PolarArcAnnotation</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polararczoommodifier.html" class="tsd-signature-type">PolarArcZoomModifier</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polaraxisbase.html" class="tsd-signature-type">PolarAxisBase</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polaraxisbase.html" class="tsd-signature-type">PolarAxisBase</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polaraxisrenderer.html" class="tsd-signature-type">PolarAxisRenderer</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polaraxisrenderer.html" class="tsd-signature-type">PolarAxisRenderer</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarbandrenderableseries.html" class="tsd-signature-type">PolarBandRenderableSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarbandseriesdrawingprovider.html" class="tsd-signature-type">PolarBandSeriesDrawingProvider</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarcategoryaxis.html" class="tsd-signature-type">PolarCategoryAxis</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarcategoryaxis.html" class="tsd-signature-type">PolarCategoryAxis</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarcolumnrenderableseries.html" class="tsd-signature-type">PolarColumnRenderableSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarcolumnseriesdrawingprovider.html" class="tsd-signature-type">PolarColumnSeriesDrawingProvider</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarcursormodifier.html" class="tsd-signature-type">PolarCursorModifier</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polardatapointselectionmodifier.html" class="tsd-signature-type">PolarDataPointSelectionModifier</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarheatmapdrawingprovider.html" class="tsd-signature-type">PolarHeatmapDrawingProvider</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarinterpolatebandrenderdatatransform.html" class="tsd-signature-type">PolarInterpolateBandRenderDataTransform</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarinterpolatelinerenderdatatransform.html" class="tsd-signature-type">PolarInterpolateLineRenderDataTransform</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarlegendmodifier.html" class="tsd-signature-type">PolarLegendModifier</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarlinerenderableseries.html" class="tsd-signature-type">PolarLineRenderableSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarlineseriesdrawingprovider.html" class="tsd-signature-type">PolarLineSeriesDrawingProvider</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarmountainrenderableseries.html" class="tsd-signature-type">PolarMountainRenderableSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarmousewheelzoommodifier.html" class="tsd-signature-type">PolarMouseWheelZoomModifier</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarnumericaxis.html" class="tsd-signature-type">PolarNumericAxis</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarnumericaxis.html" class="tsd-signature-type">PolarNumericAxis</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarpanmodifier.html" class="tsd-signature-type">PolarPanModifier</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarpointmarkerdrawingprovider.html" class="tsd-signature-type">PolarPointMarkerDrawingProvider</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarpointerannotation.html" class="tsd-signature-type">PolarPointerAnnotation</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarseriesselectionmodifier.html" class="tsd-signature-type">PolarSeriesSelectionModifier</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarstackedcolumnrenderableseries.html" class="tsd-signature-type">PolarStackedColumnRenderableSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarstackedmountainrenderableseries.html" class="tsd-signature-type">PolarStackedMountainRenderableSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polartextrenderableseries.html" class="tsd-signature-type">PolarTextRenderableSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polartrianglerenderableseries.html" class="tsd-signature-type">PolarTriangleRenderableSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polartriangleseriesdrawingprovider.html" class="tsd-signature-type">PolarTriangleSeriesDrawingProvider</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polaruniformheatmaprenderableseries.html" class="tsd-signature-type">PolarUniformHeatmapRenderableSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html" class="tsd-signature-type">PolarXyScatterRenderableSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarzoomextentsmodifier.html" class="tsd-signature-type">PolarZoomExtentsModifier</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/radianlabelprovider.html" class="tsd-signature-type">RadianLabelProvider</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rectangleseriesdrawingprovider.html" class="tsd-signature-type">RectangleSeriesDrawingProvider</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rendercontextannotationbase.html" class="tsd-signature-type">RenderContextAnnotationBase</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderableseriessceneentity.html" class="tsd-signature-type">RenderableSeriesSceneEntity</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/resetcamera3dmodifier.html" class="tsd-signature-type">ResetCamera3DModifier</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rolloverlegendsvgannotation.html" class="tsd-signature-type">RolloverLegendSvgAnnotation</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rollovermarkersvgannotation.html" class="tsd-signature-type">RolloverMarkerSvgAnnotation</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rollovermodifier.html" class="tsd-signature-type">RolloverModifier</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rollovertooltipsvgannotation.html" class="tsd-signature-type">RolloverTooltipSvgAnnotation</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rootsceneentity.html" class="tsd-signature-type">RootSceneEntity</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rubberbandsvgrect.html" class="tsd-signature-type">RubberBandSvgRect</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rubberbandsvgrect.html" class="tsd-signature-type">RubberBandSvgRect</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rubberbandxyzoommodifier.html" class="tsd-signature-type">RubberBandXyZoomModifier</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scatterpointssceneentity.html" class="tsd-signature-type">ScatterPointsSceneEntity</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scatterrenderableseries3d.html" class="tsd-signature-type">ScatterRenderableSeries3D</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dlegend.html" class="tsd-signature-type">SciChart3DLegend</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dlegend.html" class="tsd-signature-type">SciChart3DLegend</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html" class="tsd-signature-type">SciChart3DSurface</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartlegend.html" class="tsd-signature-type">SciChartLegend</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartlegend.html" class="tsd-signature-type">SciChartLegend</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartlegendbase.html" class="tsd-signature-type">SciChartLegendBase</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartlegendbase.html" class="tsd-signature-type">SciChartLegendBase</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartoverview.html" class="tsd-signature-type">SciChartOverview</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartoverview.html" class="tsd-signature-type">SciChartOverview</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartpielegend.html" class="tsd-signature-type">SciChartPieLegend</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartpielegend.html" class="tsd-signature-type">SciChartPieLegend</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartpiesurface.html" class="tsd-signature-type">SciChartPieSurface</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartpolarsubsurface.html" class="tsd-signature-type">SciChartPolarSubSurface</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartpolarsurface.html" class="tsd-signature-type">SciChartPolarSurface</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsubsurface.html" class="tsd-signature-type">SciChartSubSurface</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html" class="tsd-signature-type">SciChartSurface</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurfacebase.html" class="tsd-signature-type">SciChartSurfaceBase</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesselectionmodifier.html" class="tsd-signature-type">SeriesSelectionModifier</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/shadereffect.html" class="tsd-signature-type">ShaderEffect</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/shadereffect.html" class="tsd-signature-type">ShaderEffect</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/shadoweffect.html" class="tsd-signature-type">ShadowEffect</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/shadoweffect.html" class="tsd-signature-type">ShadowEffect</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/smartdatelabelprovider.html" class="tsd-signature-type">SmartDateLabelProvider</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/smearseriesdrawingprovider.html" class="tsd-signature-type">SmearSeriesDrawingProvider</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/smoothstackedmountainrenderableseries.html" class="tsd-signature-type">SmoothStackedMountainRenderableSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/smoothstackedrenderdatatransform.html" class="tsd-signature-type">SmoothStackedRenderDataTransform</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/splinebandrenderableseries.html" class="tsd-signature-type">SplineBandRenderableSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/splinelinerenderableseries.html" class="tsd-signature-type">SplineLineRenderableSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/splinemountainrenderableseries.html" class="tsd-signature-type">SplineMountainRenderableSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/splinerenderdatatransform.html" class="tsd-signature-type">SplineRenderDataTransform</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/spritepointmarker.html" class="tsd-signature-type">SpritePointMarker</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/squarepointmarker.html" class="tsd-signature-type">SquarePointMarker</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedcolumnrenderableseries.html" class="tsd-signature-type">StackedColumnRenderableSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html" class="tsd-signature-type">StackedMountainRenderableSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshrenderableseries3d.html" class="tsd-signature-type">SurfaceMeshRenderableSeries3D</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/surfacemeshsceneentity.html" class="tsd-signature-type">SurfaceMeshSceneEntity</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/svgannotationbase.html" class="tsd-signature-type">SvgAnnotationBase</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/textannotation.html" class="tsd-signature-type">TextAnnotation</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/textlabelprovider.html" class="tsd-signature-type">TextLabelProvider</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/texturemanager.html" class="tsd-signature-type">TextureManager</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/texturemanager.html" class="tsd-signature-type">TextureManager</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/titlerendererbase.html" class="tsd-signature-type">TitleRendererBase</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/tooltipmodifier3d.html" class="tsd-signature-type">TooltipModifier3D</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/tooltipsvgannotation3d.html" class="tsd-signature-type">TooltipSvgAnnotation3D</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/trianglepointmarker.html" class="tsd-signature-type">TrianglePointMarker</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/triangleseriesdrawingprovider.html" class="tsd-signature-type">TriangleSeriesDrawingProvider</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/uniformcontoursdrawingprovider.html" class="tsd-signature-type">UniformContoursDrawingProvider</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/uniformcontoursrenderableseries.html" class="tsd-signature-type">UniformContoursRenderableSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/uniformgriddataseries3d.html" class="tsd-signature-type">UniformGridDataSeries3D</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/uniformheatmapdrawingprovider.html" class="tsd-signature-type">UniformHeatmapDrawingProvider</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/uniformheatmaprenderableseries.html" class="tsd-signature-type">UniformHeatmapRenderableSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html" class="tsd-signature-type">VerticalLineAnnotation</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticalslicemodifier.html" class="tsd-signature-type">VerticalSliceModifier</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/webglbrush.html" class="tsd-signature-type">WebGlBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/webglpen.html" class="tsd-signature-type">WebGlPen</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/webglrendercontext2d.html" class="tsd-signature-type">WebGlRenderContext2D</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xaxisdragmodifier.html" class="tsd-signature-type">XAxisDragModifier</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xdataseries.html" class="tsd-signature-type">XDataSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xpointmarker.html" class="tsd-signature-type">XPointMarker</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xybaserenderdatatransform.html" class="tsd-signature-type">XyBaseRenderDataTransform</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xycustomfilter.html" class="tsd-signature-type">XyCustomFilter</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html" class="tsd-signature-type">XyDataSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyfilterbase.html" class="tsd-signature-type">XyFilterBase</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xylineartrendfilter.html" class="tsd-signature-type">XyLinearTrendFilter</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xymovingaveragefilter.html" class="tsd-signature-type">XyMovingAverageFilter</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyndataseries.html" class="tsd-signature-type">XyNDataSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xynpointseriesresampled.html" class="tsd-signature-type">XyNPointSeriesResampled</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xynpointseriesresampled.html" class="tsd-signature-type">XyNPointSeriesResampled</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xypointseriesresampled.html" class="tsd-signature-type">XyPointSeriesResampled</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xypointseriesresampled.html" class="tsd-signature-type">XyPointSeriesResampled</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyratiofilter.html" class="tsd-signature-type">XyRatioFilter</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyscaleoffsetfilter.html" class="tsd-signature-type">XyScaleOffsetFilter</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyscatterrenderableseries.html" class="tsd-signature-type">XyScatterRenderableSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xytextdataseries.html" class="tsd-signature-type">XyTextDataSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyxdataseries.html" class="tsd-signature-type">XyxDataSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyxydataseries.html" class="tsd-signature-type">XyxyDataSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyybaserenderdatatransform.html" class="tsd-signature-type">XyyBaseRenderDataTransform</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyybezierrenderdatatransform.html" class="tsd-signature-type">XyyBezierRenderDataTransform</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyycustomfilter.html" class="tsd-signature-type">XyyCustomFilter</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyydataseries.html" class="tsd-signature-type">XyyDataSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyyfilterbase.html" class="tsd-signature-type">XyyFilterBase</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyypointseriesresampled.html" class="tsd-signature-type">XyyPointSeriesResampled</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyypointseriesresampled.html" class="tsd-signature-type">XyyPointSeriesResampled</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyyscaleoffsetfilter.html" class="tsd-signature-type">XyyScaleOffsetFilter</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyysplinerenderdatatransform.html" class="tsd-signature-type">XyySplineRenderDataTransform</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyzbaserenderdatatransform.html" class="tsd-signature-type">XyzBaseRenderDataTransform</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyzcustomfilter.html" class="tsd-signature-type">XyzCustomFilter</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyzdataseries.html" class="tsd-signature-type">XyzDataSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyzdataseries3d.html" class="tsd-signature-type">XyzDataSeries3D</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyzfilterbase.html" class="tsd-signature-type">XyzFilterBase</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyzpointseriesresampled.html" class="tsd-signature-type">XyzPointSeriesResampled</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyzpointseriesresampled.html" class="tsd-signature-type">XyzPointSeriesResampled</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyzscaleoffsetfilter.html" class="tsd-signature-type">XyzScaleOffsetFilter</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/yaxisdragmodifier.html" class="tsd-signature-type">YAxisDragModifier</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/zoomextentsmodifier.html" class="tsd-signature-type">ZoomExtentsModifier</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/zoompanmodifier.html" class="tsd-signature-type">ZoomPanModifier</a>

## Index

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html#delete" class="tsd-kind-icon">delete</a>

## Methods

### delete

- delete(): void

<!-- -->

- Deletes native (WebAssembly) memory used by this type, after which it cannot be used.

  remarks  
  Call .delete() before finishing with the object to ensure that WebAssmembly memory leaks do not occur.

  All elements within SciChart's High Performance <a href="https://www.scichart.com/javascript-chart-features" class="external">Realtime JavaScript Charts</a> which implement [IDeletable](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html) must be deleted manually to free native (WebAssembly) memory

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
