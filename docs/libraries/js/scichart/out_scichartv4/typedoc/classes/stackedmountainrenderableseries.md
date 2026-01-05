<img src="out_scichartv4/typedoc/classes/stackedmountainrenderableseries_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [StackedMountainRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html)

# Class StackedMountainRenderableSeries

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

summary  
The [StackedMountainRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html) allows creating JavaScript Stacked Mountain charts

description  
Multiple [StackedMountainRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html) are required to create a stacked mountain chart type in SciChart. These are grouped with a [StackedMountainCollection](https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#stackedmountaincollection), which implements [IRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries.html) and may be added directly to a [SciChartSurface.renderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#renderableseries) collection.

Code sample below for creating a stacked mountain chart

``` ts
const stackedMountain0 = new StackedMountainRenderableSeries(wasmContext);
// .. configure mountain 1, including set dataSeries
const stackedMountain1 = new StackedMountainRenderableSeries(wasmContext);
// .. configure mountain 2, including set dataSeries
const stackedMountain2 = new StackedMountainRenderableSeries(wasmContext);
// .. configure mountain 3, including set dataSeries
const stackedMountainCollection = new StackedMountainCollection(wasmContext);
stackedMountainCollection.add(stackedMountain0, stackedMountain1, stackedMountain2);

sciChartSurface.renderableSeries.add(stackedMountainCollection);
```

remarks  
Do not add the [StackedMountainRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html) directly to [SciChartSurface.renderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#renderableseries) array, instead, use a [StackedMountainCollection](https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#stackedmountaincollection) to group / stack the mountains.

------------------------------------------------------------------------

ðŸ“š Docs: <a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/stacked-mountain-renderable-series/" class="external">https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/stacked-mountain-renderable-series/</a>

### Hierarchy

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basestackedmountainrenderableseries.html" class="tsd-signature-type">BaseStackedMountainRenderableSeries</a>
  - StackedMountainRenderableSeries
    - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/smoothstackedmountainrenderableseries.html" class="tsd-signature-type">SmoothStackedMountainRenderableSeries</a>
    - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarstackedmountainrenderableseries.html" class="tsd-signature-type">PolarStackedMountainRenderableSeries</a>

### Implements

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html" class="tsd-signature-type">IDeletable</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries.html" class="tsd-signature-type">IRenderableSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iorderedrenderable.html" class="tsd-signature-type">IOrderedRenderable</a>

## Index

### Constructors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#constructor" class="tsd-kind-icon">constructor</a>

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#accumulatedfinalanimationvalues" class="tsd-kind-icon">accumulatedFinalAnimationValues</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#accumulatedvalues" class="tsd-kind-icon">accumulatedValues</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#animationfsm" class="tsd-kind-icon">animationFSM</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#animationqueue" class="tsd-kind-icon">animationQueue</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#currentrenderpassdata" class="tsd-kind-icon">currentRenderPassData</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#datalabelproviderproperty" class="tsd-kind-icon">dataLabelProviderProperty</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#dataseriesproperty" class="tsd-kind-icon">dataSeriesProperty</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#hittestprovider" class="tsd-kind-icon">hitTestProvider</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#hovered" class="tsd-kind-icon">hovered</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#id" class="tsd-kind-icon">id</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#invalidateparentcallback" class="tsd-kind-icon">invalidateParentCallback</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#iscollection" class="tsd-kind-icon">isCollection</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#ispolar" class="tsd-kind-icon">isPolar</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#isstacked" class="tsd-kind-icon">isStacked</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#isvisiblechanged" class="tsd-kind-icon">isVisibleChanged</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#notifyparentpropertychangedfn" class="tsd-kind-icon">notifyParentPropertyChangedFn</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#opacityproperty" class="tsd-kind-icon">opacityProperty</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#paletteproviderproperty" class="tsd-kind-icon">paletteProviderProperty</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#parentcollection" class="tsd-kind-icon">parentCollection</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#pointseries" class="tsd-kind-icon">pointSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#renderdatatransformproperty" class="tsd-kind-icon">renderDataTransformProperty</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#resamplerhelper" class="tsd-kind-icon">resamplerHelper</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#resamplingparams" class="tsd-kind-icon">resamplingParams</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#rollovermodifierprops" class="tsd-kind-icon">rolloverModifierProps</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#rollovermodifierprops1" class="tsd-kind-icon">rolloverModifierProps1</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#selected" class="tsd-kind-icon">selected</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#transformedrenderpassdata" class="tsd-kind-icon">transformedRenderPassData</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#type" class="tsd-kind-icon">type</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#typemap" class="tsd-kind-icon">typeMap</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#webassemblycontext" class="tsd-kind-icon">webAssemblyContext</a>

### Accessors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#animation" class="tsd-kind-icon">animation</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#cliptoyrange" class="tsd-kind-icon">clipToYRange</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#customtextureoptions" class="tsd-kind-icon">customTextureOptions</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#datalabelprovider" class="tsd-kind-icon">dataLabelProvider</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#dataseries" class="tsd-kind-icon">dataSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#drawnanas" class="tsd-kind-icon">drawNaNAs</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#drawingproviders" class="tsd-kind-icon">drawingProviders</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#effect" class="tsd-kind-icon">effect</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#enabledrawingoptimisations" class="tsd-kind-icon">enableDrawingOptimisations</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#fill" class="tsd-kind-icon">fill</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#filly1" class="tsd-kind-icon">fillY1</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#isdigitalline" class="tsd-kind-icon">isDigitalLine</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#ishovered" class="tsd-kind-icon">isHovered</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#isrunninganimation" class="tsd-kind-icon">isRunningAnimation</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#isrunningdataanimation" class="tsd-kind-icon">isRunningDataAnimation</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#isselected" class="tsd-kind-icon">isSelected</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#isspline" class="tsd-kind-icon">isSpline</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#isvisible" class="tsd-kind-icon">isVisible</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#opacity" class="tsd-kind-icon">opacity</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#paletteprovider" class="tsd-kind-icon">paletteProvider</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#parentsurface" class="tsd-kind-icon">parentSurface</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#pointmarker" class="tsd-kind-icon">pointMarker</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#renderdatatransform" class="tsd-kind-icon">renderDataTransform</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#resamplingmode" class="tsd-kind-icon">resamplingMode</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#resamplingprecision" class="tsd-kind-icon">resamplingPrecision</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#seriesname" class="tsd-kind-icon">seriesName</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#stroke" class="tsd-kind-icon">stroke</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#strokedasharray" class="tsd-kind-icon">strokeDashArray</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#strokethickness" class="tsd-kind-icon">strokeThickness</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#strokey1" class="tsd-kind-icon">strokeY1</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#strokey1dasharray" class="tsd-kind-icon">strokeY1DashArray</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#supportsresampling" class="tsd-kind-icon">supportsResampling</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#xaxis" class="tsd-kind-icon">xAxis</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#xaxisid" class="tsd-kind-icon">xAxisId</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#yarrayfilter" class="tsd-kind-icon">yArrayFilter</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#yaxis" class="tsd-kind-icon">yAxis</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#yaxisid" class="tsd-kind-icon">yAxisId</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#yrangemode" class="tsd-kind-icon">yRangeMode</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#adddrawingproviders" class="tsd-kind-icon">addDrawingProviders</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#adjustautocolor" class="tsd-kind-icon">adjustAutoColor</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#afteranimationcomplete" class="tsd-kind-icon">afterAnimationComplete</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#applytheme" class="tsd-kind-icon">applyTheme</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#beforeanimationstart" class="tsd-kind-icon">beforeAnimationStart</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#checkisoutofdatarange" class="tsd-kind-icon">checkIsOutOfDataRange</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#dataseriesdatachanged" class="tsd-kind-icon">dataSeriesDataChanged</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#delete" class="tsd-kind-icon">delete</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#draw" class="tsd-kind-icon">draw</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#enqueueanimation" class="tsd-kind-icon">enqueueAnimation</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#getaccumulatedvalues" class="tsd-kind-icon">getAccumulatedValues</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#getbasexvalues" class="tsd-kind-icon">getBaseXValues</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#getcurrentrenderpassdata" class="tsd-kind-icon">getCurrentRenderPassData</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#getdatapointwidth" class="tsd-kind-icon">getDataPointWidth</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#getdataseriesname" class="tsd-kind-icon">getDataSeriesName</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#getdataseriesvaluescount" class="tsd-kind-icon">getDataSeriesValuesCount</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#getindicesrange" class="tsd-kind-icon">getIndicesRange</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#getnativexvalues" class="tsd-kind-icon">getNativeXValues</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#getnativeyvalues" class="tsd-kind-icon">getNativeYValues</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#getparentsurfacefn" class="tsd-kind-icon">getParentSurfaceFn</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#getrenderlayer" class="tsd-kind-icon">getRenderLayer</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#getrendernextto" class="tsd-kind-icon">getRenderNextTo</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#getrenderorder" class="tsd-kind-icon">getRenderOrder</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#getresampledpointseries" class="tsd-kind-icon">getResampledPointSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#getresamplingparams" class="tsd-kind-icon">getResamplingParams</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#getseriesinfo" class="tsd-kind-icon">getSeriesInfo</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#getsurfacerenderorder" class="tsd-kind-icon">getSurfaceRenderOrder</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#getxrange" class="tsd-kind-icon">getXRange</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#getyrange" class="tsd-kind-icon">getYRange</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#hasdataseries" class="tsd-kind-icon">hasDataSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#hasdataseriesvalues" class="tsd-kind-icon">hasDataSeriesValues</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#hasfillpaletteprovider" class="tsd-kind-icon">hasFillPaletteProvider</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#haspointmarkerpaletteprovider" class="tsd-kind-icon">hasPointMarkerPaletteProvider</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#hasstrokepaletteprovider" class="tsd-kind-icon">hasStrokePaletteProvider</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#linkaxes" class="tsd-kind-icon">linkAxes</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#needsresampling" class="tsd-kind-icon">needsResampling</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#newhittestprovider" class="tsd-kind-icon">newHitTestProvider</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#notifypropertychanged" class="tsd-kind-icon">notifyPropertyChanged</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#onanimate" class="tsd-kind-icon">onAnimate</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#onattach" class="tsd-kind-icon">onAttach</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#onattachtoparentcollection" class="tsd-kind-icon">onAttachToParentCollection</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#ondetach" class="tsd-kind-icon">onDetach</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#ondetachfromparentcollection" class="tsd-kind-icon">onDetachFromParentCollection</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#ondpichanged" class="tsd-kind-icon">onDpiChanged</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#pushpalettedcolors" class="tsd-kind-icon">pushPalettedColors</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#resolveautocolors" class="tsd-kind-icon">resolveAutoColors</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#runanimation" class="tsd-kind-icon">runAnimation</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#setanimationvectors" class="tsd-kind-icon">setAnimationVectors</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#setcurrentrenderpassdata" class="tsd-kind-icon">setCurrentRenderPassData</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#setpaletteprovider" class="tsd-kind-icon">setPaletteProvider</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#setrenderlayer" class="tsd-kind-icon">setRenderLayer</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#setrendernextto" class="tsd-kind-icon">setRenderNextTo</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#setrenderorder" class="tsd-kind-icon">setRenderOrder</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#setsurfacerenderorder" class="tsd-kind-icon">setSurfaceRenderOrder</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#tojson" class="tsd-kind-icon">toJSON</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#topointseries" class="tsd-kind-icon">toPointSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#updateanimationproperties" class="tsd-kind-icon">updateAnimationProperties</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#updatetransformedvalues" class="tsd-kind-icon">updateTransformedValues</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#valuechanged" class="tsd-kind-icon">valueChanged</a>

## Constructors

### constructor

- new StackedMountainRenderableSeries(webAssemblyContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart" class="tsd-signature-type">TSciChart</a>, options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/istackedmountainrenderableseriesoptions.html" class="tsd-signature-type">IStackedMountainRenderableSeriesOptions</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html" class="tsd-signature-type">StackedMountainRenderableSeries</a>

<!-- -->

- #### Parameters

  - ##### webAssemblyContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart" class="tsd-signature-type">TSciChart</a>

  - ##### Optional options: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/istackedmountainrenderableseriesoptions.html" class="tsd-signature-type">IStackedMountainRenderableSeriesOptions</a>

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html" class="tsd-signature-type">StackedMountainRenderableSeries</a>

## Properties

### Protected accumulatedFinalAnimationValues

accumulatedFinalAnimationValues: SCRTDoubleVector

### accumulatedValues

accumulatedValues: SCRTDoubleVector

the accumulated values which are used to draw each column/band for [BaseStackedRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/basestackedrenderableseries.html)

### Protected animationFSM

animationFSM: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesanimationfinitestatemachine.html" class="tsd-signature-type">SeriesAnimationFiniteStateMachine</a>

### Protected animationQueue

animationQueue: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesanimation.html" class="tsd-signature-type">SeriesAnimation</a>\[\] = \[\]

### Protected currentRenderPassData

currentRenderPassData: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassdata.html" class="tsd-signature-type">RenderPassData</a>

### Protected dataLabelProviderProperty

dataLabelProviderProperty: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basedatalabelprovider.html" class="tsd-signature-type">BaseDataLabelProvider</a>

### Protected dataSeriesProperty

dataSeriesProperty: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idataseries.html" class="tsd-signature-type">IDataSeries</a>

### hitTestProvider

hitTestProvider: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ihittestprovider.html" class="tsd-signature-type">IHitTestProvider</a>

### hovered

hovered: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/eventhandler.html" class="tsd-signature-type">EventHandler</a>\<<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/serieshoveredargs.html" class="tsd-signature-type">SeriesHoveredArgs</a>\> = new EventHandler\<SeriesHoveredArgs\>()

A hovered EventHandler. This event fires whenever the [Series](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries.html) is hovered or unhovered by a mouse or pointer.

remarks  
See [EventHandler](https://www.scichart.com/documentation/js/v4/typedoc/classes/eventhandler.html) for how to subscribe

### Readonly id

id: string

A unique Id for the [IRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries.html)

### invalidateParentCallback

invalidateParentCallback: () =\> void

A callback which tells the parent [SciChartSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html) that it must be redrawn, e.g. when a property changes

#### Type declaration

- - (): void

  <!-- -->

  - #### Returns void

### Readonly isCollection

isCollection: boolean = false

Returns true if the series is a collection of other series.

### Readonly isPolar

isPolar: boolean = false

Returns true if the series can be used on a polar surface

### Readonly isStacked

isStacked: boolean = true

Returns true if the series is a stacked series or not

### isVisibleChanged

isVisibleChanged: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/eventhandler.html" class="tsd-signature-type">EventHandler</a>\<<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesvisiblechangedargs.html" class="tsd-signature-type">SeriesVisibleChangedArgs</a>\> = new EventHandler\<SeriesVisibleChangedArgs\>()

An isVisible changed EventHandler. This event fires whenever the [Series](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries.html) isVisible changes.

remarks  
See [EventHandler](https://www.scichart.com/documentation/js/v4/typedoc/classes/eventhandler.html) for how to subscribe

### Protected notifyParentPropertyChangedFn

notifyParentPropertyChangedFn: (propertyName: string) =\> void

#### Type declaration

- - (propertyName: string): void

  <!-- -->

  - #### Parameters

    - ##### propertyName: string

    #### Returns void

### Protected opacityProperty

opacityProperty: number = 1

### Protected paletteProviderProperty

paletteProviderProperty: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipaletteprovider.html" class="tsd-signature-type">IPaletteProvider</a> = DefaultPaletteProvider.createEmpty()

### Protected parentCollection

parentCollection: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basestackedcollection.html" class="tsd-signature-type">BaseStackedCollection</a>\<<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basestackedrenderableseries.html" class="tsd-signature-type">BaseStackedRenderableSeries</a>\>

### Protected pointSeries

pointSeries: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basepointseriesresampled.html" class="tsd-signature-type">BasePointSeriesResampled</a>

### Protected renderDataTransformProperty

renderDataTransformProperty: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderdatatransform.html" class="tsd-signature-type">IRenderDataTransform</a>

### Protected resamplerHelper

resamplerHelper: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/extremeresamplerhelper.html" class="tsd-signature-type">ExtremeResamplerHelper</a>

### Protected resamplingParams

resamplingParams: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/resamplingparams.html" class="tsd-signature-type">ResamplingParams</a>

### Readonly rolloverModifierProps

rolloverModifierProps: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rollovermodifierrenderableseriesprops.html" class="tsd-signature-type">RolloverModifierRenderableSeriesProps</a> = new RolloverModifierRenderableSeriesProps(this)

### Readonly rolloverModifierProps1

rolloverModifierProps1: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rollovermodifierrenderableseriesprops.html" class="tsd-signature-type">RolloverModifierRenderableSeriesProps</a> = new RolloverModifierRenderableSeriesProps(this, true)

Gets or sets [RolloverModifierRenderableSeriesProps](https://www.scichart.com/documentation/js/v4/typedoc/classes/rollovermodifierrenderableseriesprops.html) for [RolloverModifier](https://www.scichart.com/documentation/js/v4/typedoc/classes/rollovermodifier.html) tooltips Is being used for Y1 tooltips for [FastBandRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/fastbandrenderableseries.html)

### selected

selected: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/eventhandler.html" class="tsd-signature-type">EventHandler</a>\<<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesselectedargs.html" class="tsd-signature-type">SeriesSelectedArgs</a>\> = new EventHandler\<SeriesSelectedArgs\>()

A selected EventHandler. This event fires whenever the [Series](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries.html) is selected or deselected.

remarks  
See [EventHandler](https://www.scichart.com/documentation/js/v4/typedoc/classes/eventhandler.html) for how to subscribe

### Protected transformedRenderPassData

transformedRenderPassData: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassdata.html" class="tsd-signature-type">RenderPassData</a>

### Readonly type

type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html" class="tsd-signature-type">ESeriesType</a> = ESeriesType.StackedMountainSeries

### Protected typeMap

typeMap: Map\<string, string\> = new Map\<string, string\>()

### Protected webAssemblyContext

webAssemblyContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart" class="tsd-signature-type">TSciChart</a>

## Accessors

### animation

- set animation(value: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesanimation.html" class="tsd-signature-type">SeriesAnimation</a>): void

<!-- -->

- Sets a start up animation class, a child class for [SeriesAnimation](https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesanimation.html)

  #### Parameters

  - ##### value: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesanimation.html" class="tsd-signature-type">SeriesAnimation</a>

  #### Returns void

### clipToYRange

- get clipToYRange(): boolean
- set clipToYRange(value: boolean): void

<!-- -->

- If true, the drawing will be clipped to the visibleRange of the associated Y Axis. This is only really relevant if you are using Stacked Y Axes and do not want the series to be drawn outside that axis range

  #### Returns boolean

- If true, the drawing will be clipped to the visibleRange of the associated Y Axis. This is only really relevant if you are using Stacked Y Axes and do not want the series to be drawn outside that axis range

  #### Parameters

  - ##### value: boolean

  #### Returns void

### customTextureOptions

- get customTextureOptions(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icustomtextureoptions.html" class="tsd-signature-type">ICustomTextureOptions</a>
- set customTextureOptions(customTextureOptions: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icustomtextureoptions.html" class="tsd-signature-type">ICustomTextureOptions</a>): void

<!-- -->

- Gets or sets options to use a custom texture brush

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icustomtextureoptions.html" class="tsd-signature-type">ICustomTextureOptions</a>

- Gets or sets options to use a custom texture brush

  #### Parameters

  - ##### customTextureOptions: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icustomtextureoptions.html" class="tsd-signature-type">ICustomTextureOptions</a>

  #### Returns void

### dataLabelProvider

- get dataLabelProvider(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basedatalabelprovider.html" class="tsd-signature-type">BaseDataLabelProvider</a>
- set dataLabelProvider(provider: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basedatalabelprovider.html" class="tsd-signature-type">BaseDataLabelProvider</a>): void

<!-- -->

- Gets or sets the [BaseDataLabelProvider](https://www.scichart.com/documentation/js/v4/typedoc/classes/basedatalabelprovider.html) used for creating and drawing per-point text

  inheritdoc  

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basedatalabelprovider.html" class="tsd-signature-type">BaseDataLabelProvider</a>

- Gets or sets the [BaseDataLabelProvider](https://www.scichart.com/documentation/js/v4/typedoc/classes/basedatalabelprovider.html) used for creating and drawing per-point text

  inheritdoc  

  #### Parameters

  - ##### provider: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basedatalabelprovider.html" class="tsd-signature-type">BaseDataLabelProvider</a>

  #### Returns void

### dataSeries

- get dataSeries(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idataseries.html" class="tsd-signature-type">IDataSeries</a>
- set dataSeries(dataSeries: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idataseries.html" class="tsd-signature-type">IDataSeries</a>): void

<!-- -->

- The [DataSeries](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idataseries.html) which provides a datasource for this [IRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries.html) to draw

  inheritdoc  

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idataseries.html" class="tsd-signature-type">IDataSeries</a>

- The [DataSeries](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idataseries.html) which provides a datasource for this [IRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries.html) to draw

  inheritdoc  

  #### Parameters

  - ##### dataSeries: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idataseries.html" class="tsd-signature-type">IDataSeries</a>

  #### Returns void

### drawNaNAs

- get drawNaNAs(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/elinedrawmode.html" class="tsd-signature-type">ELineDrawMode</a>
- set drawNaNAs(drawNaNAs: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/elinedrawmode.html" class="tsd-signature-type">ELineDrawMode</a>): void

<!-- -->

- How to treat NAN (Not a number) values in the input [dataSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#dataseries). See [ELineDrawMode](https://www.scichart.com/documentation/js/v4/typedoc/enums/elinedrawmode.html) for a list of values.

  inheritdoc  

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/elinedrawmode.html" class="tsd-signature-type">ELineDrawMode</a>

- How to treat NAN (Not a number) values in the input [dataSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#dataseries). See [ELineDrawMode](https://www.scichart.com/documentation/js/v4/typedoc/enums/elinedrawmode.html) for a list of values.

  inheritdoc  

  #### Parameters

  - ##### drawNaNAs: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/elinedrawmode.html" class="tsd-signature-type">ELineDrawMode</a>

  #### Returns void

### drawingProviders

- get drawingProviders(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iseriesdrawingprovider.html" class="tsd-signature-type">ISeriesDrawingProvider</a>\[\]
- set drawingProviders(value: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iseriesdrawingprovider.html" class="tsd-signature-type">ISeriesDrawingProvider</a>\[\]): void

<!-- -->

- Gets a list of [Series Drawing Providers](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iseriesdrawingprovider.html), which perform specific drawing operations in the series

  inheritdoc  

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iseriesdrawingprovider.html" class="tsd-signature-type">ISeriesDrawingProvider</a>\[\]

- Gets a list of [Series Drawing Providers](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iseriesdrawingprovider.html), which perform specific drawing operations in the series

  inheritdoc  

  #### Parameters

  - ##### value: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iseriesdrawingprovider.html" class="tsd-signature-type">ISeriesDrawingProvider</a>\[\]

  #### Returns void

### effect

- get effect(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/shadereffect.html" class="tsd-signature-type">ShaderEffect</a>
- set effect(effect: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/shadereffect.html" class="tsd-signature-type">ShaderEffect</a>): void

<!-- -->

- Gets an optional [ShaderEffect](https://www.scichart.com/documentation/js/v4/typedoc/enums/ebasetype.html#shadereffect) for modifying the render output of this [IRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries.html)

  remarks  
  Options include [GlowEffect](https://www.scichart.com/documentation/js/v4/typedoc/classes/gloweffect.html) and [ShadowEffect](https://www.scichart.com/documentation/js/v4/typedoc/classes/shadoweffect.html). Apply an effect to see how it modifies rendering!

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/shadereffect.html" class="tsd-signature-type">ShaderEffect</a>

- Sets an optional [ShaderEffect](https://www.scichart.com/documentation/js/v4/typedoc/enums/ebasetype.html#shadereffect) for modifying the render output of this [IRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries.html)

  remarks  
  Options include [GlowEffect](https://www.scichart.com/documentation/js/v4/typedoc/classes/gloweffect.html) and [ShadowEffect](https://www.scichart.com/documentation/js/v4/typedoc/classes/shadoweffect.html). Apply an effect to see how it modifies rendering!

  #### Parameters

  - ##### effect: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/shadereffect.html" class="tsd-signature-type">ShaderEffect</a>

  #### Returns void

### enableDrawingOptimisations

- get enableDrawingOptimisations(): boolean

<!-- -->

- Readonly. When true, resampling modes are enabled for faster drawing performance.

  inheritdoc  

  #### Returns boolean

### fill

- get fill(): string
- set fill(fill: string): void

<!-- -->

- Gets or sets the fill brush of the mountain as an HTML color code

  #### Returns string

- Gets or sets the fill brush of the mountain as an HTML color code

  #### Parameters

  - ##### fill: string

  #### Returns void

### fillY1

- get fillY1(): string
- set fillY1(fillY1: string): void

<!-- -->

- Gets or sets the fill color for when Y1 is less than Y as an HTML Color code

  #### Returns string

- Gets or sets the fill color for when Y1 is less than Y as an HTML Color code

  #### Parameters

  - ##### fillY1: string

  #### Returns void

### isDigitalLine

- get isDigitalLine(): boolean
- set isDigitalLine(isDigitalLine: boolean): void

<!-- -->

- When true, if this series draws a line, the line will be a digital (step) line

  inheritdoc  

  #### Returns boolean

- When true, if this series draws a line, the line will be a digital (step) line

  inheritdoc  

  #### Parameters

  - ##### isDigitalLine: boolean

  #### Returns void

### isHovered

- get isHovered(): boolean
- set isHovered(isHovered: boolean): void

<!-- -->

- Gets or sets whether the Series is hovered by a mouse or pointer device. Setting programmatically will trigger hovered logic

  inheritdoc  

  #### Returns boolean

- Gets or sets whether the Series is hovered by a mouse or pointer device. Setting programmatically will trigger hovered logic

  inheritdoc  

  #### Parameters

  - ##### isHovered: boolean

  #### Returns void

### isRunningAnimation

- get isRunningAnimation(): boolean

<!-- -->

- gets if the animation is currently running

  inheritdoc  

  #### Returns boolean

### isRunningDataAnimation

- get isRunningDataAnimation(): boolean

<!-- -->

- gets if a data animation is currently running

  #### Returns boolean

### isSelected

- get isSelected(): boolean
- set isSelected(isSelected: boolean): void

<!-- -->

- Gets or sets whether the Series is selected. Setting programmatically will trigger selection logic

  inheritdoc  

  #### Returns boolean

- Gets or sets whether the Series is selected. Setting programmatically will trigger selection logic

  inheritdoc  

  #### Parameters

  - ##### isSelected: boolean

  #### Returns void

### isSpline

- get isSpline(): boolean

<!-- -->

- Returns true if the series uses spline interpolation

  inheritdoc  

  #### Returns boolean

### isVisible

- get isVisible(): boolean
- set isVisible(isVisible: boolean): void

<!-- -->

- When true, the series is visible and drawn

  inheritdoc  

  #### Returns boolean

- When true, the series is visible and drawn

  inheritdoc  

  #### Parameters

  - ##### isVisible: boolean

  #### Returns void

### opacity

- get opacity(): number
- set opacity(value: number): void

<!-- -->

- An Opacity factor of the Series that controls its semi-transparency level, where value 1 means the Series is opaque; 0 - transparent.

  inheritdoc  

  #### Returns number

- An Opacity factor of the Series that controls its semi-transparency level, where value 1 means the Series is opaque; 0 - transparent.

  inheritdoc  

  #### Parameters

  - ##### value: number

  #### Returns void

### paletteProvider

- get paletteProvider(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipaletteprovider.html" class="tsd-signature-type">IPaletteProvider</a>
- set paletteProvider(paletteProvider: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipaletteprovider.html" class="tsd-signature-type">IPaletteProvider</a>): void

<!-- -->

- An optional [IPaletteProvider](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipaletteprovider.html) which is used to provide per data-point coloring or paletting.

  inheritdoc  

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipaletteprovider.html" class="tsd-signature-type">IPaletteProvider</a>

- An optional [IPaletteProvider](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipaletteprovider.html) which is used to provide per data-point coloring or paletting.

  inheritdoc  

  #### Parameters

  - ##### paletteProvider: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipaletteprovider.html" class="tsd-signature-type">IPaletteProvider</a>

  #### Returns void

### parentSurface

- get parentSurface(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html" class="tsd-signature-type">SciChartSurface</a>
- set parentSurface(value: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html" class="tsd-signature-type">SciChartSurface</a>): void

<!-- -->

- The parent [SciChartSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html) that this RenderableSeries is attached to

  inheritdoc  

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html" class="tsd-signature-type">SciChartSurface</a>

- set parentSurface property is not supported for BaseStackedRenderableSeries

  #### Parameters

  - ##### value: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html" class="tsd-signature-type">SciChartSurface</a>

  #### Returns void

### pointMarker

- get pointMarker(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmarker.html" class="tsd-signature-type">IPointMarker</a> \| undefined
- set pointMarker(pointMarker: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmarker.html" class="tsd-signature-type">IPointMarker</a> \| undefined): void

<!-- -->

- A [Point Marker](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmarker.html) which is used to draw an optional point-marker at each data-point. Applicable to some series types only

  inheritdoc  

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmarker.html" class="tsd-signature-type">IPointMarker</a> \| undefined

- A [Point Marker](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmarker.html) which is used to draw an optional point-marker at each data-point. Applicable to some series types only

  inheritdoc  

  #### Parameters

  - ##### pointMarker: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmarker.html" class="tsd-signature-type">IPointMarker</a> \| undefined

  #### Returns void

### renderDataTransform

- get renderDataTransform(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderdatatransform.html" class="tsd-signature-type">IRenderDataTransform</a>
- set renderDataTransform(transform: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderdatatransform.html" class="tsd-signature-type">IRenderDataTransform</a>): void

<!-- -->

- inheritdoc  

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderdatatransform.html" class="tsd-signature-type">IRenderDataTransform</a>

- inheritdoc  

  #### Parameters

  - ##### transform: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderdatatransform.html" class="tsd-signature-type">IRenderDataTransform</a>

  #### Returns void

### resamplingMode

- get resamplingMode(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eresamplingmode.html" class="tsd-signature-type">EResamplingMode</a>
- set resamplingMode(value: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eresamplingmode.html" class="tsd-signature-type">EResamplingMode</a>): void

<!-- -->

- resamplingMode property is not supported for BaseStackedRenderableSeries

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eresamplingmode.html" class="tsd-signature-type">EResamplingMode</a>

- resamplingMode property is not supported for BaseStackedRenderableSeries

  #### Parameters

  - ##### value: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eresamplingmode.html" class="tsd-signature-type">EResamplingMode</a>

  #### Returns void

### resamplingPrecision

- get resamplingPrecision(): number
- set resamplingPrecision(value: number): void

<!-- -->

- resamplingPrecision property is not supported for BaseStackedRenderableSeries

  #### Returns number

- resamplingPrecision property is not supported for BaseStackedRenderableSeries

  #### Parameters

  - ##### value: number

  #### Returns void

### seriesName

- get seriesName(): string \| undefined
- set seriesName(value: string): void

<!-- -->

- Gets or sets series name

  inheritdoc  

  #### Returns string \| undefined

- Gets or sets series name

  #### Parameters

  - ##### value: string

  #### Returns void

### stroke

- get stroke(): string
- set stroke(htmlColorCode: string): void

<!-- -->

- A Stroke for lines, outlines and edges of this RenderableSeries

  inheritdoc  

  #### Returns string

- A Stroke for lines, outlines and edges of this RenderableSeries

  inheritdoc  

  #### Parameters

  - ##### htmlColorCode: string

  #### Returns void

### strokeDashArray

- get strokeDashArray(): number\[\]
- set strokeDashArray(strokeDashArray: number\[\]): void

<!-- -->

- The StrokeDashArray defines the stroke or dash pattern for the Y0 line. Accepts an array of values, e.g. \[2,2\] will have a line of length 2 and a gap of length 2.

  #### Returns number\[\]

- The StrokeDashArray defines the stroke or dash pattern for the Y0 line. Accepts an array of values, e.g. \[2,2\] will have a line of length 2 and a gap of length 2.

  #### Parameters

  - ##### strokeDashArray: number\[\]

  #### Returns void

### strokeThickness

- get strokeThickness(): number
- set strokeThickness(value: number): void

<!-- -->

- The Stroke Thickness for lines, outlines and edges of this RenderableSeries

  inheritdoc  

  #### Returns number

- The Stroke Thickness for lines, outlines and edges of this RenderableSeries

  inheritdoc  

  #### Parameters

  - ##### value: number

  #### Returns void

### strokeY1

- get strokeY1(): string
- set strokeY1(strokeY1: string): void

<!-- -->

- Gets or sets the stroke color the Y1 values in the data-series. See associated [XyyDataSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/xyydataseries.html) for further information

  remarks  
  This property is set internally to the value of a previous StackedMountainSeries.

  #### Returns string

- Gets or sets the stroke color the Y1 values in the data-series. See associated [XyyDataSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/xyydataseries.html) for further information

  remarks  
  This property is set internally to the value of a previous StackedMountainSeries.

  #### Parameters

  - ##### strokeY1: string

  #### Returns void

### strokeY1DashArray

- get strokeY1DashArray(): number\[\]
- set strokeY1DashArray(strokeY1DashArray: number\[\]): void

<!-- -->

- The strokeY1DashArray defines the stroke or dash pattern for the Y1 line. Accepts an array of values, e.g. \[2,2\] will have a line of length 2 and a gap of length 2.

  remarks  
  This property is set internally to the value of a previous StackedMountainSeries.

  #### Returns number\[\]

- The strokeY1DashArray defines the stroke or dash pattern for the Y1 line. Accepts an array of values, e.g. \[2,2\] will have a line of length 2 and a gap of length 2.

  remarks  
  This property is set internally to the value of a previous StackedMountainSeries.

  #### Parameters

  - ##### strokeY1DashArray: number\[\]

  #### Returns void

### supportsResampling

- get supportsResampling(): boolean

<!-- -->

- Returns true if the series supports resampling

  inheritdoc  

  #### Returns boolean

### xAxis

- get xAxis(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase2d.html" class="tsd-signature-type">AxisBase2D</a>

<!-- -->

- Gets the bound [XAxis](https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase2d.html) for this [IRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries.html).

  inheritdoc  

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase2d.html" class="tsd-signature-type">AxisBase2D</a>

### xAxisId

- get xAxisId(): string \| undefined
- set xAxisId(value: string): void

<!-- -->

- xAxisId property is not supported for BaseStackedRenderableSeries, instead set on the [StackedColumnCollection](https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#stackedcolumncollection) or [StackedMountainCollection](https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#stackedmountaincollection)

  #### Returns string \| undefined

- xAxisId property is not supported for BaseStackedRenderableSeries, instead set on the [StackedColumnCollection](https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#stackedcolumncollection) or [StackedMountainCollection](https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#stackedmountaincollection)

  #### Parameters

  - ##### value: string

  #### Returns void

### yArrayFilter

- get yArrayFilter(): string \| number
- set yArrayFilter(value: string \| number): void

<!-- -->

- inheritdoc  

  #### Returns string \| number

- inheritdoc  

  #### Parameters

  - ##### value: string \| number

  #### Returns void

### yAxis

- get yAxis(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase2d.html" class="tsd-signature-type">AxisBase2D</a>

<!-- -->

- Gets the bound [YAxis](https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase2d.html) for this [IRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries.html).

  inheritdoc  

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase2d.html" class="tsd-signature-type">AxisBase2D</a>

### yAxisId

- get yAxisId(): string \| undefined
- set yAxisId(value: string): void

<!-- -->

- yAxisId property is not supported for BaseStackedRenderableSeries, instead set on the [StackedColumnCollection](https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#stackedcolumncollection) or [StackedMountainCollection](https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#stackedmountaincollection)

  #### Returns string \| undefined

- yAxisId property is not supported for BaseStackedRenderableSeries, instead set on the [StackedColumnCollection](https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#stackedcolumncollection) or [StackedMountainCollection](https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#stackedmountaincollection)

  #### Parameters

  - ##### value: string

  #### Returns void

### yRangeMode

- get yRangeMode(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eyrangemode.html" class="tsd-signature-type">EYRangeMode</a>
- set yRangeMode(value: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eyrangemode.html" class="tsd-signature-type">EYRangeMode</a>): void

<!-- -->

- Determines whether the y range for this series should consider only the visible data (the default), or include the drawn points just outside the visible range

  inheritdoc  

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eyrangemode.html" class="tsd-signature-type">EYRangeMode</a>

- Determines whether the y range for this series should consider only the visible data (the default), or include the drawn points just outside the visible range

  #### Parameters

  - ##### value: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eyrangemode.html" class="tsd-signature-type">EYRangeMode</a>

  #### Returns void

## Methods

### Protected addDrawingProviders

- addDrawingProviders(webAssemblyContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart" class="tsd-signature-type">TSciChart</a>): void

<!-- -->

- #### Parameters

  - ##### webAssemblyContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart" class="tsd-signature-type">TSciChart</a>

  #### Returns void

### adjustAutoColor

- adjustAutoColor(propertyName: string, color: string): string

<!-- -->

- Replace this to do custom adjustments to the auto color for a particular property

  #### Parameters

  - ##### propertyName: string

  - ##### color: string

  #### Returns string

### Protected afterAnimationComplete

- afterAnimationComplete(): void

<!-- -->

- Runs after the animation is complete

  #### Returns void

### applyTheme

- applyTheme(themeProvider: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html" class="tsd-signature-type">IThemeProvider</a>): void

<!-- -->

- inheritdoc  

  #### Parameters

  - ##### themeProvider: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html" class="tsd-signature-type">IThemeProvider</a>

  #### Returns void

### Protected beforeAnimationStart

- beforeAnimationStart(): void

<!-- -->

- Runs before the animation starts

  #### Returns void

### checkIsOutOfDataRange

- checkIsOutOfDataRange(xValue: number, yValue: number): boolean

<!-- -->

- Checks is the point is out of the data range. For sorted data only. Is used to hide tooltips for [RolloverModifier](https://www.scichart.com/documentation/js/v4/typedoc/classes/rollovermodifier.html)

  #### Parameters

  - ##### xValue: number

    The X value of the point

  - ##### yValue: number

    The Y value of the point

  #### Returns boolean

### Protected dataSeriesDataChanged

- dataSeriesDataChanged(): void

<!-- -->

- #### Returns void

### delete

- delete(): void

<!-- -->

- Deletes native (WebAssembly) memory used by this type, after which it cannot be used.

  remarks  
  Call .delete() before finishing with the object to ensure that WebAssmembly memory leaks do not occur.

  All elements within SciChart's High Performance <a href="https://www.scichart.com/javascript-chart-features" class="external">Realtime JavaScript Charts</a> which implement [IDeletable](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html) must be deleted manually to free native (WebAssembly) memory

  #### Returns void

### draw

- draw(renderContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/webglrendercontext2d.html" class="tsd-signature-type">WebGlRenderContext2D</a>, renderPassData: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassdata.html" class="tsd-signature-type">RenderPassData</a>): void

<!-- -->

- Called when the [BaseRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries.html) must be drawn

  #### Parameters

  - ##### renderContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/webglrendercontext2d.html" class="tsd-signature-type">WebGlRenderContext2D</a>

    The {@link WebGL2RenderingContext} with methods for drawing on the WebGL Canvas via our WebAssembly Rendering Engine

  - ##### renderPassData: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassdata.html" class="tsd-signature-type">RenderPassData</a>

    The [RenderPassData](https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassdata.html) containing data about the current rendering pass

  #### Returns void

### enqueueAnimation

- enqueueAnimation(animation: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesanimation.html" class="tsd-signature-type">SeriesAnimation</a>): void

<!-- -->

- Add the animation into the queue

  #### Parameters

  - ##### animation: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesanimation.html" class="tsd-signature-type">SeriesAnimation</a>

  #### Returns void

### getAccumulatedValues

- getAccumulatedValues(finalValues: boolean): SCRTDoubleVector

<!-- -->

- Returns current of final animation accumulated values. Is used for visible range calculation with animation

  #### Parameters

  - ##### finalValues: boolean

    Flag to return final animation accumulated values

  #### Returns SCRTDoubleVector

### getBaseXValues

- getBaseXValues(): number\[\]

<!-- -->

- getBaseXValues() is not supported for BaseStackedRenderableSeries

  #### Returns number\[\]

### getCurrentRenderPassData

- getCurrentRenderPassData(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassdata.html" class="tsd-signature-type">RenderPassData</a>

<!-- -->

- Gets the RenderPassData instance used for this render pass

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassdata.html" class="tsd-signature-type">RenderPassData</a>

### getDataPointWidth

- getDataPointWidth(xCoordCalc: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html" class="tsd-signature-type">CoordinateCalculatorBase</a>, widthFraction: number, widthMode?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/edatapointwidthmode.html" class="tsd-signature-type">EDataPointWidthMode</a>): number

<!-- -->

- description  
  Calculates data point width in pixels

  #### Parameters

  - ##### xCoordCalc: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html" class="tsd-signature-type">CoordinateCalculatorBase</a>

  - ##### widthFraction: number

  - ##### Optional widthMode: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/edatapointwidthmode.html" class="tsd-signature-type">EDataPointWidthMode</a>

  #### Returns number

### getDataSeriesName

- getDataSeriesName(): string

<!-- -->

- Returns the associated [IDataSeries.dataSeriesName](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idataseries.html#dataseriesname)

  #### Returns string

### getDataSeriesValuesCount

- getDataSeriesValuesCount(): number

<!-- -->

- Returns [IDataSeries.count](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idataseries.html#count) for the linked [dataSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#dataseries)

  #### Returns number

### getIndicesRange

- getIndicesRange(xRange: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>, isCategoryData?: boolean): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

<!-- -->

- Returns the indices range of data points in xRange of the associated [IDataSeries](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idataseries.html)

  #### Parameters

  - ##### xRange: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

    The X-Axis Range currently in view

  - ##### Default value isCategoryData: boolean = false

    If True the renderable series uses [CategoryAxis](https://www.scichart.com/documentation/js/v4/typedoc/enums/eaxistype.html#categoryaxis)

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

### getNativeXValues

- getNativeXValues(): SCRTDoubleVector

<!-- -->

- Returns the [IDataSeries.getNativeXValues](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idataseries.html#getnativexvalues) for the associated [dataSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#dataseries)

  #### Returns SCRTDoubleVector

### getNativeYValues

- getNativeYValues(): SCRTDoubleVector

<!-- -->

- Returns the [IDataSeries.getNativeYValues](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idataseries.html#getnativeyvalues) for the associated [dataSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#dataseries)

  #### Returns SCRTDoubleVector

### Protected getParentSurfaceFn

- getParentSurfaceFn(): any

<!-- -->

- #### Returns any

### getRenderLayer

- getRenderLayer(): number

<!-- -->

- The render layer grouping within which the series will be draw. Defaults to EDefaultRenderLayer.SeriesLayer (5)

  #### Returns number

### getRenderNextTo

- getRenderNextTo(): { offset: number; renderable: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iorderedrenderable.html" class="tsd-signature-type">IOrderedRenderable</a> \| string }

<!-- -->

- inheritdoc  

  #### Returns { offset: number; renderable: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iorderedrenderable.html" class="tsd-signature-type">IOrderedRenderable</a> \| string }

  - ##### offset: number

  - ##### renderable: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iorderedrenderable.html" class="tsd-signature-type">IOrderedRenderable</a> \| string

### getRenderOrder

- getRenderOrder(): number

<!-- -->

- inheritdoc  

  #### Returns number

### Protected getResampledPointSeries

- getResampledPointSeries(isXCategoryAxis?: boolean): <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointseries.html" class="tsd-signature-type">IPointSeries</a>

<!-- -->

- #### Parameters

  - ##### Default value isXCategoryAxis: boolean = false

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointseries.html" class="tsd-signature-type">IPointSeries</a>

### getResamplingParams

- getResamplingParams(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/resamplingparams.html" class="tsd-signature-type">ResamplingParams</a>

<!-- -->

- #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/resamplingparams.html" class="tsd-signature-type">ResamplingParams</a>

### getSeriesInfo

- getSeriesInfo(hitTestInfo: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/hittestinfo.html" class="tsd-signature-type">HitTestInfo</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo.html" class="tsd-signature-type">SeriesInfo</a>

<!-- -->

- Get a SeiesInfo object for this series based on the given hitTest

  #### Parameters

  - ##### hitTestInfo: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/hittestinfo.html" class="tsd-signature-type">HitTestInfo</a>

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo.html" class="tsd-signature-type">SeriesInfo</a>

### getSurfaceRenderOrder

- getSurfaceRenderOrder(): number

<!-- -->

- inheritdoc  

  #### Returns number

### getXRange

- getXRange(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

<!-- -->

- Gets the X-Range of the series. Override in derived classes to provide series specific implementations

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

### getYRange

- getYRange(xVisibleRange: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>, isXCategoryAxis?: boolean): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

<!-- -->

- Gets the Y-Range of the series for the current X-Range. Override in derived classes to provide series specific implementations

  #### Parameters

  - ##### xVisibleRange: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

    The [AxisCore.visibleRange](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#visiblerange) for the current bound XAxis

  - ##### Default value isXCategoryAxis: boolean = false

    Whether the current bound [XAxis](https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase2d.html) is a Category axis

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

### hasDataSeries

- hasDataSeries(): boolean

<!-- -->

- Returns true if the [BaseRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries.html) has a [dataSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#dataseries)

  #### Returns boolean

### hasDataSeriesValues

- hasDataSeriesValues(): boolean

<!-- -->

- Returns true if the [BaseRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries.html) has a [dataSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html#dataseries) and [IDataSeries.hasValues](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idataseries.html#hasvalues) is true

  #### Returns boolean

### hasFillPaletteProvider

- hasFillPaletteProvider(): boolean

<!-- -->

- Returns true if the [BaseRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries.html) has an [](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ifillpaletteprovider.html)

  #### Returns boolean

### hasPointMarkerPaletteProvider

- hasPointMarkerPaletteProvider(): boolean

<!-- -->

- Returns true if the [BaseRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries.html) has an [IPointMarkerPaletteProvider](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmarkerpaletteprovider.html)

  #### Returns boolean

### hasStrokePaletteProvider

- hasStrokePaletteProvider(): boolean

<!-- -->

- Returns true if the [BaseRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries.html) has an [IStrokePaletteProvider](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/istrokepaletteprovider.html)

  #### Returns boolean

### linkAxes

- linkAxes(): void

<!-- -->

- The linking is ignored because axes from the parent collection are used for stacked renderable series

  #### Returns void

### Protected needsResampling

- needsResampling(rp: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/resamplingparams.html" class="tsd-signature-type">ResamplingParams</a>): boolean

<!-- -->

- Used internally to check if resampling is needed for the renderable series

  #### Parameters

  - ##### rp: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/resamplingparams.html" class="tsd-signature-type">ResamplingParams</a>

  #### Returns boolean

### Protected newHitTestProvider

- newHitTestProvider(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ihittestprovider.html" class="tsd-signature-type">IHitTestProvider</a>

<!-- -->

- inheritdoc  

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ihittestprovider.html" class="tsd-signature-type">IHitTestProvider</a>

### notifyPropertyChanged

- notifyPropertyChanged(propertyName: string): void

<!-- -->

- inheritdoc  

  #### Parameters

  - ##### propertyName: string

  #### Returns void

### onAnimate

- onAnimate(timeElapsed: number): void

<!-- -->

- Is called for each render

  #### Parameters

  - ##### timeElapsed: number

  #### Returns void

### onAttach

- onAttach(scs: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html" class="tsd-signature-type">SciChartSurface</a>): void

<!-- -->

- Called when the [BaseRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries.html) is attached to a parent [SciChartSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html)

  #### Parameters

  - ##### scs: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html" class="tsd-signature-type">SciChartSurface</a>

    the [SciChartSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html) that this series has been attached to

  #### Returns void

### onAttachToParentCollection

- onAttachToParentCollection(parentCollection: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basestackedcollection.html" class="tsd-signature-type">BaseStackedCollection</a>\<<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basestackedrenderableseries.html" class="tsd-signature-type">BaseStackedRenderableSeries</a>\>, getParentSurfaceFn: () =\> <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html" class="tsd-signature-type">SciChartSurface</a>, notifyPropertyChangedFn: (propertyName: string) =\> void): void

<!-- -->

- Called internally when the [StackedMountainRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html) is attached to a parent [StackedMountainCollection](https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#stackedmountaincollection)

  #### Parameters

  - ##### parentCollection: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basestackedcollection.html" class="tsd-signature-type">BaseStackedCollection</a>\<<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basestackedrenderableseries.html" class="tsd-signature-type">BaseStackedRenderableSeries</a>\>

    the parent [BaseStackedCollection](https://www.scichart.com/documentation/js/v4/typedoc/classes/basestackedcollection.html)

  - ##### getParentSurfaceFn: () =\> <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html" class="tsd-signature-type">SciChartSurface</a>

    function to get the parent [SciChartSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html)

    - - (): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html" class="tsd-signature-type">SciChartSurface</a>

      <!-- -->

      - #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html" class="tsd-signature-type">SciChartSurface</a>

  - ##### notifyPropertyChangedFn: (propertyName: string) =\> void

    function to notify property has changed

    - - (propertyName: string): void

      <!-- -->

      - #### Parameters

        - ##### propertyName: string

        #### Returns void

  #### Returns void

### onDetach

- onDetach(): void

<!-- -->

- Called when the [BaseRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries.html) is detached from a [SciChartSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html)

  #### Returns void

### onDetachFromParentCollection

- onDetachFromParentCollection(): void

<!-- -->

- Called when the [BaseStackedRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/basestackedrenderableseries.html) is detached from its parent [BaseStackedCollection](https://www.scichart.com/documentation/js/v4/typedoc/classes/basestackedcollection.html)

  #### Returns void

### onDpiChanged

- onDpiChanged(args: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tdpichangedeventargs" class="tsd-signature-type">TDpiChangedEventArgs</a>): void

<!-- -->

- Called when the Dpi changes in the browser. This could be due to user zooming the browser, or changing DPI settings in Windows, or moving the browser containing SciChart to another monitor

  #### Parameters

  - ##### args: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tdpichangedeventargs" class="tsd-signature-type">TDpiChangedEventArgs</a>

    The [TDpiChangedEventArgs](https://www.scichart.com/documentation/js/v4/typedoc/index.html#tdpichangedeventargs) containing info about the Dpi Changed event

  #### Returns void

### pushPalettedColors

- pushPalettedColors(color: number, palettingState: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tpalettingstate" class="tsd-signature-type">TPalettingState</a>): void

<!-- -->

- adds palette colors

  #### Parameters

  - ##### color: number

  - ##### palettingState: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tpalettingstate" class="tsd-signature-type">TPalettingState</a>

  #### Returns void

### resolveAutoColors

- resolveAutoColors(index: number, maxSeries: number, theme: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html" class="tsd-signature-type">IThemeProvider</a>): void

<!-- -->

- Resolve colors marked AUTO_COLOR using the theme's strokePalette and fillPalette To do custom adjustments to the resolved colors, override the adjustAutoColor method

  #### Parameters

  - ##### index: number

  - ##### maxSeries: number

  - ##### theme: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html" class="tsd-signature-type">IThemeProvider</a>

  #### Returns void

### runAnimation

- runAnimation(animation: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesanimation.html" class="tsd-signature-type">SeriesAnimation</a>): void

<!-- -->

- Cancel all previous animations and run the current one

  #### Parameters

  - ##### animation: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesanimation.html" class="tsd-signature-type">SeriesAnimation</a>

  #### Returns void

### Protected setAnimationVectors

- setAnimationVectors(animation: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesanimation.html" class="tsd-signature-type">SeriesAnimation</a>): void

<!-- -->

- Sets initial and end animation vectors

  #### Parameters

  - ##### animation: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesanimation.html" class="tsd-signature-type">SeriesAnimation</a>

  #### Returns void

### setCurrentRenderPassData

- setCurrentRenderPassData(renderPassData: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassdata.html" class="tsd-signature-type">RenderPassData</a>): void

<!-- -->

- Sets the RenderPassData instance used for this render pass

  #### Parameters

  - ##### renderPassData: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassdata.html" class="tsd-signature-type">RenderPassData</a>

  #### Returns void

### Protected setPaletteProvider

- setPaletteProvider(paletteProvider: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipaletteprovider.html" class="tsd-signature-type">IPaletteProvider</a>): void

<!-- -->

- #### Parameters

  - ##### paletteProvider: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipaletteprovider.html" class="tsd-signature-type">IPaletteProvider</a>

  #### Returns void

### setRenderLayer

- setRenderLayer(value: number \| <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/edefaultrenderlayer.html" class="tsd-signature-type">EDefaultRenderLayer</a>): void

<!-- -->

- The render layer grouping within which the series will be draw. Defaults to EDefaultRenderLayer.SeriesLayer (5)

  #### Parameters

  - ##### value: number \| <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/edefaultrenderlayer.html" class="tsd-signature-type">EDefaultRenderLayer</a>

  #### Returns void

### setRenderNextTo

- setRenderNextTo(renderable: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iorderedrenderable.html" class="tsd-signature-type">IOrderedRenderable</a> \| string, offset?: number): void

<!-- -->

- inheritdoc  

  #### Parameters

  - ##### renderable: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iorderedrenderable.html" class="tsd-signature-type">IOrderedRenderable</a> \| string

  - ##### Default value offset: number = 0

  #### Returns void

### setRenderOrder

- setRenderOrder(value: number): void

<!-- -->

- inheritdoc  

  #### Parameters

  - ##### value: number

  #### Returns void

### setSurfaceRenderOrder

- setSurfaceRenderOrder(value: number): void

<!-- -->

- inheritdoc  

  #### Parameters

  - ##### value: number

  #### Returns void

### toJSON

- toJSON(excludeData?: boolean): { options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibandrenderableseriesoptions.html" class="tsd-signature-type">IBandRenderableSeriesOptions</a>; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#bandseries" class="tsd-signature-type">BandSeries</a>; xyyData?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#txyyseriesdata" class="tsd-signature-type">TXyySeriesData</a> } \| { options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipolarbandrenderableseriesoptions.html" class="tsd-signature-type">IPolarBandRenderableSeriesOptions</a>; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#polarbandseries" class="tsd-signature-type">PolarBandSeries</a>; xyyData?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#txyyseriesdata" class="tsd-signature-type">TXyySeriesData</a> } \| { options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibubblerenderableseriesoptions.html" class="tsd-signature-type">IBubbleRenderableSeriesOptions</a>; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#bubbleseries" class="tsd-signature-type">BubbleSeries</a>; xyzData?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#txyzseriesdata" class="tsd-signature-type">TXyzSeriesData</a> } \| { options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icolumnrenderableseriesoptions.html" class="tsd-signature-type">IColumnRenderableSeriesOptions</a>; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#columnseries" class="tsd-signature-type">ColumnSeries</a>; xyData?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#txyseriesdata" class="tsd-signature-type">TXySeriesData</a> } \| { options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iimpulserenderableseries.html" class="tsd-signature-type">IImpulseRenderableSeries</a>; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#impulseseries" class="tsd-signature-type">ImpulseSeries</a>; xyData?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#txyseriesdata" class="tsd-signature-type">TXySeriesData</a> } \| { ohlcData?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tohlcseriesdata" class="tsd-signature-type">TOhlcSeriesData</a>; options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icandlestickrenderableseriesoptions.html" class="tsd-signature-type">ICandlestickRenderableSeriesOptions</a>; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#candlestickseries" class="tsd-signature-type">CandlestickSeries</a> } \| { options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ifastlinerenderableseriesoptions.html" class="tsd-signature-type">IFastLineRenderableSeriesOptions</a>; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#lineseries" class="tsd-signature-type">LineSeries</a>; xyData?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#txyseriesdata" class="tsd-signature-type">TXySeriesData</a> } \| { options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipolarlinerenderableseriesoptions.html" class="tsd-signature-type">IPolarLineRenderableSeriesOptions</a>; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#polarlineseries" class="tsd-signature-type">PolarLineSeries</a>; xyData?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#txyseriesdata" class="tsd-signature-type">TXySeriesData</a> } \| { options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/imountainrenderableseriesoptions.html" class="tsd-signature-type">IMountainRenderableSeriesOptions</a>; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#mountainseries" class="tsd-signature-type">MountainSeries</a>; xyData?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#txyseriesdata" class="tsd-signature-type">TXySeriesData</a> } \| { options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipolarmountainrenderableseriesoptions.html" class="tsd-signature-type">IPolarMountainRenderableSeriesOptions</a>; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#polarmountainseries" class="tsd-signature-type">PolarMountainSeries</a>; xyData?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#txyseriesdata" class="tsd-signature-type">TXySeriesData</a> } \| { hlcData?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#thlcseriesdata" class="tsd-signature-type">THlcSeriesData</a>; options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ifasterrorbarsrenderableseriesoptions.html" class="tsd-signature-type">IFastErrorBarsRenderableSeriesOptions</a>; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#errorbarsseries" class="tsd-signature-type">ErrorBarsSeries</a> } \| { ohlcData?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tohlcseriesdata" class="tsd-signature-type">TOhlcSeriesData</a>; options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iohlcrenderableseriesoptions.html" class="tsd-signature-type">IOhlcRenderableSeriesOptions</a>; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#ohlcseries" class="tsd-signature-type">OhlcSeries</a> } \| { options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ixyscatterrenderableseriesoptions.html" class="tsd-signature-type">IXyScatterRenderableSeriesOptions</a>; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#scatterseries" class="tsd-signature-type">ScatterSeries</a>; xyData?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#txyseriesdata" class="tsd-signature-type">TXySeriesData</a> } \| { options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ixyscatterrenderableseriesoptions.html" class="tsd-signature-type">IXyScatterRenderableSeriesOptions</a>; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#polarscatterseries" class="tsd-signature-type">PolarScatterSeries</a>; xyData?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#txyseriesdata" class="tsd-signature-type">TXySeriesData</a> } \| { options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/itextrenderableseriesoptions.html" class="tsd-signature-type">ITextRenderableSeriesOptions</a>; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#textseries" class="tsd-signature-type">TextSeries</a>; xyTextData?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#txytextseriesdata" class="tsd-signature-type">TXyTextSeriesData</a> } \| { options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/isplinebandrenderableseriesoptions.html" class="tsd-signature-type">ISplineBandRenderableSeriesOptions</a>; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#splinebandseries" class="tsd-signature-type">SplineBandSeries</a>; xyyData?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#txyyseriesdata" class="tsd-signature-type">TXyySeriesData</a> } \| { options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/isplinelinerenderableseriesoptions.html" class="tsd-signature-type">ISplineLineRenderableSeriesOptions</a>; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#splinelineseries" class="tsd-signature-type">SplineLineSeries</a>; xyData?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#txyseriesdata" class="tsd-signature-type">TXySeriesData</a> } \| { options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/isplinemountainrenderableseriesoptions.html" class="tsd-signature-type">ISplineMountainRenderableSeriesOptions</a>; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#splinemountainseries" class="tsd-signature-type">SplineMountainSeries</a>; xyData?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#txyseriesdata" class="tsd-signature-type">TXySeriesData</a> } \| { options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ismoothstackedmountainrenderableseriesoptions.html" class="tsd-signature-type">ISmoothStackedMountainRenderableSeriesOptions</a>; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#smoothstackedmountainseries" class="tsd-signature-type">SmoothStackedMountainSeries</a>; xyData?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#txyseriesdata" class="tsd-signature-type">TXySeriesData</a> } \| { heatmapData?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iuniformheatmapseriesoptions.html" class="tsd-signature-type">IUniformHeatmapSeriesOptions</a>; options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iheatmaprenderableseriesoptions.html" class="tsd-signature-type">IHeatmapRenderableSeriesOptions</a>; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#uniformheatmapseries" class="tsd-signature-type">UniformHeatmapSeries</a> } \| { heatmapData?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iuniformheatmapseriesoptions.html" class="tsd-signature-type">IUniformHeatmapSeriesOptions</a>; options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipolaruniformheatmaprenderableseriesoptions.html" class="tsd-signature-type">IPolarUniformHeatmapRenderableSeriesOptions</a>; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#polaruniformheatmapseries" class="tsd-signature-type">PolarUniformHeatmapSeries</a> } \| { heatmapData?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/inonuniformheatmapseriesoptions.html" class="tsd-signature-type">INonUniformHeatmapSeriesOptions</a>; options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/inonuniformheatmaprenderableseriesoptions.html" class="tsd-signature-type">INonUniformHeatmapRenderableSeriesOptions</a>; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#nonuniformheatmapseries" class="tsd-signature-type">NonUniformHeatmapSeries</a> } \| { heatmapData?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iuniformheatmapseriesoptions.html" class="tsd-signature-type">IUniformHeatmapSeriesOptions</a>; options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icontoursrenderableseriesoptions.html" class="tsd-signature-type">IContoursRenderableSeriesOptions</a>; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#uniformcontoursseries" class="tsd-signature-type">UniformContoursSeries</a> } \| { options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/istackedcolumnrenderableseriesoptions.html" class="tsd-signature-type">IStackedColumnRenderableSeriesOptions</a>; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#stackedcolumnseries" class="tsd-signature-type">StackedColumnSeries</a>; xyData?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#txyseriesdata" class="tsd-signature-type">TXySeriesData</a> } \| { options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipolarcolumnrenderableseriesoptions.html" class="tsd-signature-type">IPolarColumnRenderableSeriesOptions</a>; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#polarstackedcolumnseries" class="tsd-signature-type">PolarStackedColumnSeries</a>; xyData?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#txyseriesdata" class="tsd-signature-type">TXySeriesData</a> } \| { options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/istackedmountainrenderableseriesoptions.html" class="tsd-signature-type">IStackedMountainRenderableSeriesOptions</a>; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#stackedmountainseries" class="tsd-signature-type">StackedMountainSeries</a>; xyData?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#txyseriesdata" class="tsd-signature-type">TXySeriesData</a> } \| { options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipolarstackedmountainrenderableseriesoptions.html" class="tsd-signature-type">IPolarStackedMountainRenderableSeriesOptions</a>; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#polarstackedmountainseries" class="tsd-signature-type">PolarStackedMountainSeries</a>; xyData?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#txyseriesdata" class="tsd-signature-type">TXySeriesData</a> } \| { options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/istackedcolumncollectionoptions.html" class="tsd-signature-type">IStackedColumnCollectionOptions</a>; series?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tseriesdefinition" class="tsd-signature-type">TSeriesDefinition</a>\[\]; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#stackedcolumncollection" class="tsd-signature-type">StackedColumnCollection</a> } \| { options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibasestackedcollectionoptions.html" class="tsd-signature-type">IBaseStackedCollectionOptions</a>; series?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tseriesdefinition" class="tsd-signature-type">TSeriesDefinition</a>\[\]; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#polarstackedcolumncollection" class="tsd-signature-type">PolarStackedColumnCollection</a> } \| { options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibasestackedcollectionoptions.html" class="tsd-signature-type">IBaseStackedCollectionOptions</a>; series?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tseriesdefinition" class="tsd-signature-type">TSeriesDefinition</a>\[\]; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#stackedmountaincollection" class="tsd-signature-type">StackedMountainCollection</a> } \| { options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibasestackedcollectionoptions.html" class="tsd-signature-type">IBaseStackedCollectionOptions</a>; series?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tseriesdefinition" class="tsd-signature-type">TSeriesDefinition</a>\[\]; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#polarstackedmountaincollection" class="tsd-signature-type">PolarStackedMountainCollection</a> } \| { options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ifastrectanglerenderableseriesoptions.html" class="tsd-signature-type">IFastRectangleRenderableSeriesOptions</a>; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#rectangleseries" class="tsd-signature-type">RectangleSeries</a>; xyxData?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#txyxseriesdata" class="tsd-signature-type">TXyxSeriesData</a>; xyxyData?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#txyxyseriesdata" class="tsd-signature-type">TXyxySeriesData</a> } \| { boxPlotData?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tboxplotseriesdata" class="tsd-signature-type">TBoxPlotSeriesData</a>; options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ifastboxplotrenderableseriesoptions.html" class="tsd-signature-type">IFastBoxPlotRenderableSeriesOptions</a>; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#boxplotseries" class="tsd-signature-type">BoxPlotSeries</a> } \| { options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ifasttrianglerenderableseriesoptions.html" class="tsd-signature-type">IFastTriangleRenderableSeriesOptions</a>; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#triangleseries" class="tsd-signature-type">TriangleSeries</a>; xyData?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#txyseriesdata" class="tsd-signature-type">TXySeriesData</a>; xyxyData?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#txyxyseriesdata" class="tsd-signature-type">TXyxySeriesData</a> } \| { options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ifastlinesegmentrenderableseriesoptions.html" class="tsd-signature-type">IFastLineSegmentRenderableSeriesOptions</a>; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#linesegmentseries" class="tsd-signature-type">LineSegmentSeries</a>; xyData?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#txyseriesdata" class="tsd-signature-type">TXySeriesData</a>; xyxyData?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#txyxyseriesdata" class="tsd-signature-type">TXyxySeriesData</a> } \| { options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipolarcolumnrenderableseriesoptions.html" class="tsd-signature-type">IPolarColumnRenderableSeriesOptions</a>; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#polarcolumnseries" class="tsd-signature-type">PolarColumnSeries</a>; xyData?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#txyseriesdata" class="tsd-signature-type">TXySeriesData</a>; xyxData?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#txyxseriesdata" class="tsd-signature-type">TXyxSeriesData</a>; xyxyData?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#txyxyseriesdata" class="tsd-signature-type">TXyxySeriesData</a> } \| { options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ifasttrianglerenderableseriesoptions.html" class="tsd-signature-type">IFastTriangleRenderableSeriesOptions</a>; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#polartriangleseries" class="tsd-signature-type">PolarTriangleSeries</a>; xyData?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#txyseriesdata" class="tsd-signature-type">TXySeriesData</a>; xyxyData?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#txyxyseriesdata" class="tsd-signature-type">TXyxySeriesData</a> } \| { options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/itextrenderableseriesoptions.html" class="tsd-signature-type">ITextRenderableSeriesOptions</a>; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#polartextseries" class="tsd-signature-type">PolarTextSeries</a>; xyTextData?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#txytextseriesdata" class="tsd-signature-type">TXyTextSeriesData</a> } \| { customType?: string; options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibaserenderableseriesoptions.html" class="tsd-signature-type">IBaseRenderableSeriesOptions</a>; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#custom" class="tsd-signature-type">Custom</a> }

<!-- -->

- #### Parameters

  - ##### Default value excludeData: boolean = false

  #### Returns { options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibandrenderableseriesoptions.html" class="tsd-signature-type">IBandRenderableSeriesOptions</a>; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#bandseries" class="tsd-signature-type">BandSeries</a>; xyyData?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#txyyseriesdata" class="tsd-signature-type">TXyySeriesData</a> } \| { options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipolarbandrenderableseriesoptions.html" class="tsd-signature-type">IPolarBandRenderableSeriesOptions</a>; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#polarbandseries" class="tsd-signature-type">PolarBandSeries</a>; xyyData?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#txyyseriesdata" class="tsd-signature-type">TXyySeriesData</a> } \| { options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibubblerenderableseriesoptions.html" class="tsd-signature-type">IBubbleRenderableSeriesOptions</a>; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#bubbleseries" class="tsd-signature-type">BubbleSeries</a>; xyzData?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#txyzseriesdata" class="tsd-signature-type">TXyzSeriesData</a> } \| { options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icolumnrenderableseriesoptions.html" class="tsd-signature-type">IColumnRenderableSeriesOptions</a>; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#columnseries" class="tsd-signature-type">ColumnSeries</a>; xyData?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#txyseriesdata" class="tsd-signature-type">TXySeriesData</a> } \| { options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iimpulserenderableseries.html" class="tsd-signature-type">IImpulseRenderableSeries</a>; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#impulseseries" class="tsd-signature-type">ImpulseSeries</a>; xyData?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#txyseriesdata" class="tsd-signature-type">TXySeriesData</a> } \| { ohlcData?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tohlcseriesdata" class="tsd-signature-type">TOhlcSeriesData</a>; options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icandlestickrenderableseriesoptions.html" class="tsd-signature-type">ICandlestickRenderableSeriesOptions</a>; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#candlestickseries" class="tsd-signature-type">CandlestickSeries</a> } \| { options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ifastlinerenderableseriesoptions.html" class="tsd-signature-type">IFastLineRenderableSeriesOptions</a>; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#lineseries" class="tsd-signature-type">LineSeries</a>; xyData?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#txyseriesdata" class="tsd-signature-type">TXySeriesData</a> } \| { options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipolarlinerenderableseriesoptions.html" class="tsd-signature-type">IPolarLineRenderableSeriesOptions</a>; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#polarlineseries" class="tsd-signature-type">PolarLineSeries</a>; xyData?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#txyseriesdata" class="tsd-signature-type">TXySeriesData</a> } \| { options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/imountainrenderableseriesoptions.html" class="tsd-signature-type">IMountainRenderableSeriesOptions</a>; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#mountainseries" class="tsd-signature-type">MountainSeries</a>; xyData?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#txyseriesdata" class="tsd-signature-type">TXySeriesData</a> } \| { options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipolarmountainrenderableseriesoptions.html" class="tsd-signature-type">IPolarMountainRenderableSeriesOptions</a>; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#polarmountainseries" class="tsd-signature-type">PolarMountainSeries</a>; xyData?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#txyseriesdata" class="tsd-signature-type">TXySeriesData</a> } \| { hlcData?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#thlcseriesdata" class="tsd-signature-type">THlcSeriesData</a>; options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ifasterrorbarsrenderableseriesoptions.html" class="tsd-signature-type">IFastErrorBarsRenderableSeriesOptions</a>; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#errorbarsseries" class="tsd-signature-type">ErrorBarsSeries</a> } \| { ohlcData?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tohlcseriesdata" class="tsd-signature-type">TOhlcSeriesData</a>; options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iohlcrenderableseriesoptions.html" class="tsd-signature-type">IOhlcRenderableSeriesOptions</a>; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#ohlcseries" class="tsd-signature-type">OhlcSeries</a> } \| { options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ixyscatterrenderableseriesoptions.html" class="tsd-signature-type">IXyScatterRenderableSeriesOptions</a>; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#scatterseries" class="tsd-signature-type">ScatterSeries</a>; xyData?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#txyseriesdata" class="tsd-signature-type">TXySeriesData</a> } \| { options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ixyscatterrenderableseriesoptions.html" class="tsd-signature-type">IXyScatterRenderableSeriesOptions</a>; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#polarscatterseries" class="tsd-signature-type">PolarScatterSeries</a>; xyData?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#txyseriesdata" class="tsd-signature-type">TXySeriesData</a> } \| { options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/itextrenderableseriesoptions.html" class="tsd-signature-type">ITextRenderableSeriesOptions</a>; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#textseries" class="tsd-signature-type">TextSeries</a>; xyTextData?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#txytextseriesdata" class="tsd-signature-type">TXyTextSeriesData</a> } \| { options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/isplinebandrenderableseriesoptions.html" class="tsd-signature-type">ISplineBandRenderableSeriesOptions</a>; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#splinebandseries" class="tsd-signature-type">SplineBandSeries</a>; xyyData?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#txyyseriesdata" class="tsd-signature-type">TXyySeriesData</a> } \| { options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/isplinelinerenderableseriesoptions.html" class="tsd-signature-type">ISplineLineRenderableSeriesOptions</a>; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#splinelineseries" class="tsd-signature-type">SplineLineSeries</a>; xyData?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#txyseriesdata" class="tsd-signature-type">TXySeriesData</a> } \| { options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/isplinemountainrenderableseriesoptions.html" class="tsd-signature-type">ISplineMountainRenderableSeriesOptions</a>; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#splinemountainseries" class="tsd-signature-type">SplineMountainSeries</a>; xyData?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#txyseriesdata" class="tsd-signature-type">TXySeriesData</a> } \| { options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ismoothstackedmountainrenderableseriesoptions.html" class="tsd-signature-type">ISmoothStackedMountainRenderableSeriesOptions</a>; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#smoothstackedmountainseries" class="tsd-signature-type">SmoothStackedMountainSeries</a>; xyData?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#txyseriesdata" class="tsd-signature-type">TXySeriesData</a> } \| { heatmapData?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iuniformheatmapseriesoptions.html" class="tsd-signature-type">IUniformHeatmapSeriesOptions</a>; options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iheatmaprenderableseriesoptions.html" class="tsd-signature-type">IHeatmapRenderableSeriesOptions</a>; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#uniformheatmapseries" class="tsd-signature-type">UniformHeatmapSeries</a> } \| { heatmapData?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iuniformheatmapseriesoptions.html" class="tsd-signature-type">IUniformHeatmapSeriesOptions</a>; options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipolaruniformheatmaprenderableseriesoptions.html" class="tsd-signature-type">IPolarUniformHeatmapRenderableSeriesOptions</a>; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#polaruniformheatmapseries" class="tsd-signature-type">PolarUniformHeatmapSeries</a> } \| { heatmapData?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/inonuniformheatmapseriesoptions.html" class="tsd-signature-type">INonUniformHeatmapSeriesOptions</a>; options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/inonuniformheatmaprenderableseriesoptions.html" class="tsd-signature-type">INonUniformHeatmapRenderableSeriesOptions</a>; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#nonuniformheatmapseries" class="tsd-signature-type">NonUniformHeatmapSeries</a> } \| { heatmapData?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iuniformheatmapseriesoptions.html" class="tsd-signature-type">IUniformHeatmapSeriesOptions</a>; options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icontoursrenderableseriesoptions.html" class="tsd-signature-type">IContoursRenderableSeriesOptions</a>; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#uniformcontoursseries" class="tsd-signature-type">UniformContoursSeries</a> } \| { options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/istackedcolumnrenderableseriesoptions.html" class="tsd-signature-type">IStackedColumnRenderableSeriesOptions</a>; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#stackedcolumnseries" class="tsd-signature-type">StackedColumnSeries</a>; xyData?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#txyseriesdata" class="tsd-signature-type">TXySeriesData</a> } \| { options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipolarcolumnrenderableseriesoptions.html" class="tsd-signature-type">IPolarColumnRenderableSeriesOptions</a>; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#polarstackedcolumnseries" class="tsd-signature-type">PolarStackedColumnSeries</a>; xyData?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#txyseriesdata" class="tsd-signature-type">TXySeriesData</a> } \| { options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/istackedmountainrenderableseriesoptions.html" class="tsd-signature-type">IStackedMountainRenderableSeriesOptions</a>; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#stackedmountainseries" class="tsd-signature-type">StackedMountainSeries</a>; xyData?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#txyseriesdata" class="tsd-signature-type">TXySeriesData</a> } \| { options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipolarstackedmountainrenderableseriesoptions.html" class="tsd-signature-type">IPolarStackedMountainRenderableSeriesOptions</a>; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#polarstackedmountainseries" class="tsd-signature-type">PolarStackedMountainSeries</a>; xyData?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#txyseriesdata" class="tsd-signature-type">TXySeriesData</a> } \| { options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/istackedcolumncollectionoptions.html" class="tsd-signature-type">IStackedColumnCollectionOptions</a>; series?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tseriesdefinition" class="tsd-signature-type">TSeriesDefinition</a>\[\]; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#stackedcolumncollection" class="tsd-signature-type">StackedColumnCollection</a> } \| { options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibasestackedcollectionoptions.html" class="tsd-signature-type">IBaseStackedCollectionOptions</a>; series?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tseriesdefinition" class="tsd-signature-type">TSeriesDefinition</a>\[\]; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#polarstackedcolumncollection" class="tsd-signature-type">PolarStackedColumnCollection</a> } \| { options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibasestackedcollectionoptions.html" class="tsd-signature-type">IBaseStackedCollectionOptions</a>; series?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tseriesdefinition" class="tsd-signature-type">TSeriesDefinition</a>\[\]; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#stackedmountaincollection" class="tsd-signature-type">StackedMountainCollection</a> } \| { options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibasestackedcollectionoptions.html" class="tsd-signature-type">IBaseStackedCollectionOptions</a>; series?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tseriesdefinition" class="tsd-signature-type">TSeriesDefinition</a>\[\]; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#polarstackedmountaincollection" class="tsd-signature-type">PolarStackedMountainCollection</a> } \| { options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ifastrectanglerenderableseriesoptions.html" class="tsd-signature-type">IFastRectangleRenderableSeriesOptions</a>; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#rectangleseries" class="tsd-signature-type">RectangleSeries</a>; xyxData?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#txyxseriesdata" class="tsd-signature-type">TXyxSeriesData</a>; xyxyData?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#txyxyseriesdata" class="tsd-signature-type">TXyxySeriesData</a> } \| { boxPlotData?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tboxplotseriesdata" class="tsd-signature-type">TBoxPlotSeriesData</a>; options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ifastboxplotrenderableseriesoptions.html" class="tsd-signature-type">IFastBoxPlotRenderableSeriesOptions</a>; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#boxplotseries" class="tsd-signature-type">BoxPlotSeries</a> } \| { options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ifasttrianglerenderableseriesoptions.html" class="tsd-signature-type">IFastTriangleRenderableSeriesOptions</a>; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#triangleseries" class="tsd-signature-type">TriangleSeries</a>; xyData?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#txyseriesdata" class="tsd-signature-type">TXySeriesData</a>; xyxyData?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#txyxyseriesdata" class="tsd-signature-type">TXyxySeriesData</a> } \| { options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ifastlinesegmentrenderableseriesoptions.html" class="tsd-signature-type">IFastLineSegmentRenderableSeriesOptions</a>; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#linesegmentseries" class="tsd-signature-type">LineSegmentSeries</a>; xyData?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#txyseriesdata" class="tsd-signature-type">TXySeriesData</a>; xyxyData?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#txyxyseriesdata" class="tsd-signature-type">TXyxySeriesData</a> } \| { options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipolarcolumnrenderableseriesoptions.html" class="tsd-signature-type">IPolarColumnRenderableSeriesOptions</a>; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#polarcolumnseries" class="tsd-signature-type">PolarColumnSeries</a>; xyData?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#txyseriesdata" class="tsd-signature-type">TXySeriesData</a>; xyxData?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#txyxseriesdata" class="tsd-signature-type">TXyxSeriesData</a>; xyxyData?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#txyxyseriesdata" class="tsd-signature-type">TXyxySeriesData</a> } \| { options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ifasttrianglerenderableseriesoptions.html" class="tsd-signature-type">IFastTriangleRenderableSeriesOptions</a>; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#polartriangleseries" class="tsd-signature-type">PolarTriangleSeries</a>; xyData?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#txyseriesdata" class="tsd-signature-type">TXySeriesData</a>; xyxyData?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#txyxyseriesdata" class="tsd-signature-type">TXyxySeriesData</a> } \| { options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/itextrenderableseriesoptions.html" class="tsd-signature-type">ITextRenderableSeriesOptions</a>; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#polartextseries" class="tsd-signature-type">PolarTextSeries</a>; xyTextData?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#txytextseriesdata" class="tsd-signature-type">TXyTextSeriesData</a> } \| { customType?: string; options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibaserenderableseriesoptions.html" class="tsd-signature-type">IBaseRenderableSeriesOptions</a>; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#custom" class="tsd-signature-type">Custom</a> }

### toPointSeries

- toPointSeries(resamplingParams?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/resamplingparams.html" class="tsd-signature-type">ResamplingParams</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointseries.html" class="tsd-signature-type">IPointSeries</a>

<!-- -->

- Returns a dataset for drawing on the viewport

  #### Parameters

  - ##### Optional resamplingParams: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/resamplingparams.html" class="tsd-signature-type">ResamplingParams</a>

    The resampling parameters

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointseries.html" class="tsd-signature-type">IPointSeries</a>

### Protected updateAnimationProperties

- updateAnimationProperties(progress: number, animationFSM: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesanimationfinitestatemachine.html" class="tsd-signature-type">SeriesAnimationFiniteStateMachine</a>): void

<!-- -->

- Internal method that runs on each animation tick

  #### Parameters

  - ##### progress: number

    The current animation progress, a value from 0 to 1

  - ##### animationFSM: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesanimationfinitestatemachine.html" class="tsd-signature-type">SeriesAnimationFiniteStateMachine</a>

    The animation finite state machine

  #### Returns void

### Protected updateTransformedValues

- updateTransformedValues(valueType?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/edataseriesvaluetype.html" class="tsd-signature-type">EDataSeriesValueType</a>): void

<!-- -->

- Runs renderdataTransform to set transformedRenderPassData, usually for use with ranging.

  #### Parameters

  - ##### Optional valueType: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/edataseriesvaluetype.html" class="tsd-signature-type">EDataSeriesValueType</a>

  #### Returns void

### Protected valueChanged

- valueChanged(oldValue: any, newValue: any): boolean

<!-- -->

- #### Parameters

  - ##### oldValue: any

  - ##### newValue: any

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
