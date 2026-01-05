<img src="out_scichartv4/typedoc/classes/stackedmountaincollection_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [StackedMountainCollection](https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html)

# Class StackedMountainCollection

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

summary  
A [StackedMountainCollection](https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#stackedmountaincollection) allows grouping multiple [StackedMountainRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html) to create a JavaScript Stacked Mountain chart, or 100% Stacked Mountain chart

description  
Multiple [StackedMountainRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html) are required to create a stacked mountain chart type in SciChart. These are grouped with a [StackedMountainCollection](https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#stackedmountaincollection), which implements [IRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries.html) and may be added directly to a [SciChartSurface.renderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#renderableseries) collection.

Code sample below:

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
This type implements [IRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries.html) but it is not a renderable series, instead it wraps multiple [StackedMountainRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html) to create a stacked mountain chart

------------------------------------------------------------------------

ðŸ“š Docs: <a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/stacked-mountain-renderable-series/" class="external">https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/stacked-mountain-renderable-series/</a>

### Hierarchy

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedxycollection.html" class="tsd-signature-type">StackedXyCollection</a>\<<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html" class="tsd-signature-type">StackedMountainRenderableSeries</a>\>
  - StackedMountainCollection
    - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarstackedmountaincollection.html" class="tsd-signature-type">PolarStackedMountainCollection</a>

### Implements

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries.html" class="tsd-signature-type">IRenderableSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iorderedrenderable.html" class="tsd-signature-type">IOrderedRenderable</a>

## Index

### Constructors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#constructor" class="tsd-kind-icon">constructor</a>

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#accumulatedfinalanimationvalues0" class="tsd-kind-icon">accumulatedFinalAnimationValues0</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#accumulatedvalues0" class="tsd-kind-icon">accumulatedValues0</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#animationfsm" class="tsd-kind-icon">animationFSM</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#collectionchanged" class="tsd-kind-icon">collectionChanged</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#currentrenderpassdata" class="tsd-kind-icon">currentRenderPassData</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#datalabelproviderproperty" class="tsd-kind-icon">dataLabelProviderProperty</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#enabledrawingoptimisations" class="tsd-kind-icon">enableDrawingOptimisations</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#firstanimationrender" class="tsd-kind-icon">firstAnimationRender</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#id" class="tsd-kind-icon">id</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#invalidateparentcallback" class="tsd-kind-icon">invalidateParentCallback</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#isaccumulatedvectordirty" class="tsd-kind-icon">isAccumulatedVectorDirty</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#iscollection" class="tsd-kind-icon">isCollection</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#ispolar" class="tsd-kind-icon">isPolar</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#isspline" class="tsd-kind-icon">isSpline</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#isstacked" class="tsd-kind-icon">isStacked</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#items" class="tsd-kind-icon">items</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#parentsurface" class="tsd-kind-icon">parentSurface</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#supportsresampling" class="tsd-kind-icon">supportsResampling</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#type" class="tsd-kind-icon">type</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#webassemblycontext" class="tsd-kind-icon">webAssemblyContext</a>

### Accessors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#animation" class="tsd-kind-icon">animation</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#candraw" class="tsd-kind-icon">canDraw</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#datalabelprovider" class="tsd-kind-icon">dataLabelProvider</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#dataseries" class="tsd-kind-icon">dataSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#drawnanas" class="tsd-kind-icon">drawNaNAs</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#drawingproviders" class="tsd-kind-icon">drawingProviders</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#effect" class="tsd-kind-icon">effect</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#hittestprovider" class="tsd-kind-icon">hitTestProvider</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#hovered" class="tsd-kind-icon">hovered</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#isdigitalline" class="tsd-kind-icon">isDigitalLine</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#ishovered" class="tsd-kind-icon">isHovered</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#isonehundredpercent" class="tsd-kind-icon">isOneHundredPercent</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#isrunninganimation" class="tsd-kind-icon">isRunningAnimation</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#isrunningdataanimation" class="tsd-kind-icon">isRunningDataAnimation</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#isselected" class="tsd-kind-icon">isSelected</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#isvisible" class="tsd-kind-icon">isVisible</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#isvisiblechanged" class="tsd-kind-icon">isVisibleChanged</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#opacity" class="tsd-kind-icon">opacity</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#paletteprovider" class="tsd-kind-icon">paletteProvider</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#pointmarker" class="tsd-kind-icon">pointMarker</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#resamplingmode" class="tsd-kind-icon">resamplingMode</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#resamplingprecision" class="tsd-kind-icon">resamplingPrecision</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#rollovermodifierprops" class="tsd-kind-icon">rolloverModifierProps</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#rollovermodifierprops1" class="tsd-kind-icon">rolloverModifierProps1</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#selected" class="tsd-kind-icon">selected</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#seriesname" class="tsd-kind-icon">seriesName</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#stroke" class="tsd-kind-icon">stroke</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#strokethickness" class="tsd-kind-icon">strokeThickness</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#xaxis" class="tsd-kind-icon">xAxis</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#xaxisid" class="tsd-kind-icon">xAxisId</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#yaxis" class="tsd-kind-icon">yAxis</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#yaxisid" class="tsd-kind-icon">yAxisId</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#yrangemode" class="tsd-kind-icon">yRangeMode</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#add" class="tsd-kind-icon">add</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#adjustautocolor" class="tsd-kind-icon">adjustAutoColor</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#afteranimationcomplete" class="tsd-kind-icon">afterAnimationComplete</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#applytheme" class="tsd-kind-icon">applyTheme</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#asarray" class="tsd-kind-icon">asArray</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#beforeanimationstart" class="tsd-kind-icon">beforeAnimationStart</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#checkiduniqueness" class="tsd-kind-icon">checkIdUniqueness</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#checkisoutofdatarange" class="tsd-kind-icon">checkIsOutOfDataRange</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#clear" class="tsd-kind-icon">clear</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#contains" class="tsd-kind-icon">contains</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#delete" class="tsd-kind-icon">delete</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#draw" class="tsd-kind-icon">draw</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#enqueueanimation" class="tsd-kind-icon">enqueueAnimation</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#get" class="tsd-kind-icon">get</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#getbasexvalues" class="tsd-kind-icon">getBaseXValues</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#getbyid" class="tsd-kind-icon">getById</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#getcurrentrenderpassdata" class="tsd-kind-icon">getCurrentRenderPassData</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#getdatapointwidth" class="tsd-kind-icon">getDataPointWidth</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#getdataseriesname" class="tsd-kind-icon">getDataSeriesName</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#getdataseriesvaluescount" class="tsd-kind-icon">getDataSeriesValuesCount</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#getfirstseries" class="tsd-kind-icon">getFirstSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#getindicesrange" class="tsd-kind-icon">getIndicesRange</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#getnativexvalues" class="tsd-kind-icon">getNativeXValues</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#getparentsurface" class="tsd-kind-icon">getParentSurface</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#getrenderlayer" class="tsd-kind-icon">getRenderLayer</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#getrendernextto" class="tsd-kind-icon">getRenderNextTo</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#getrenderorder" class="tsd-kind-icon">getRenderOrder</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#getresamplingparams" class="tsd-kind-icon">getResamplingParams</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#getseriesinfo" class="tsd-kind-icon">getSeriesInfo</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#getsurfacerenderorder" class="tsd-kind-icon">getSurfaceRenderOrder</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#getvisibleseries" class="tsd-kind-icon">getVisibleSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#getxrange" class="tsd-kind-icon">getXRange</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#getyrange" class="tsd-kind-icon">getYRange</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#hasdataseries" class="tsd-kind-icon">hasDataSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#hasdataseriesvalues" class="tsd-kind-icon">hasDataSeriesValues</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#hasfillpaletteprovider" class="tsd-kind-icon">hasFillPaletteProvider</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#haspointmarkerpaletteprovider" class="tsd-kind-icon">hasPointMarkerPaletteProvider</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#hasstrokepaletteprovider" class="tsd-kind-icon">hasStrokePaletteProvider</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#insert" class="tsd-kind-icon">insert</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#invalidateparent" class="tsd-kind-icon">invalidateParent</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#isalldataseriesset" class="tsd-kind-icon">isAllDataSeriesSet</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#isenoughdatatodraw" class="tsd-kind-icon">isEnoughDataToDraw</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#linkaxes" class="tsd-kind-icon">linkAxes</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#notifypropertychanged" class="tsd-kind-icon">notifyPropertyChanged</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#onanimate" class="tsd-kind-icon">onAnimate</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#onattach" class="tsd-kind-icon">onAttach</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#ondetach" class="tsd-kind-icon">onDetach</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#ondpichanged" class="tsd-kind-icon">onDpiChanged</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#pushpalettedcolors" class="tsd-kind-icon">pushPalettedColors</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#remove" class="tsd-kind-icon">remove</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#removeat" class="tsd-kind-icon">removeAt</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#resolveautocolors" class="tsd-kind-icon">resolveAutoColors</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#runanimation" class="tsd-kind-icon">runAnimation</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#set" class="tsd-kind-icon">set</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#setaccumulatedvaluesdirty" class="tsd-kind-icon">setAccumulatedValuesDirty</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#setcurrentrenderpassdata" class="tsd-kind-icon">setCurrentRenderPassData</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#setrenderlayer" class="tsd-kind-icon">setRenderLayer</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#setrendernextto" class="tsd-kind-icon">setRenderNextTo</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#setrenderorder" class="tsd-kind-icon">setRenderOrder</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#setsurfacerenderorder" class="tsd-kind-icon">setSurfaceRenderOrder</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#size" class="tsd-kind-icon">size</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#tojson" class="tsd-kind-icon">toJSON</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#topointseries" class="tsd-kind-icon">toPointSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#updateaccumulatedvectors" class="tsd-kind-icon">updateAccumulatedVectors</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#updateanimationproperties" class="tsd-kind-icon">updateAnimationProperties</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#updatehittestproviders" class="tsd-kind-icon">updateHitTestProviders</a>

## Constructors

### constructor

- new StackedMountainCollection(webAssemblyContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart" class="tsd-signature-type">TSciChart</a>, options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibasestackedcollectionoptions.html" class="tsd-signature-type">IBaseStackedCollectionOptions</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html" class="tsd-signature-type">StackedMountainCollection</a>

<!-- -->

- Creates an instance of the [StackedMountainCollection](https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#stackedmountaincollection)

  #### Parameters

  - ##### webAssemblyContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart" class="tsd-signature-type">TSciChart</a>

    The [SciChart WebAssembly Context](https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart) containing native methods and access to our WebGL2 WebAssembly Drawing Engine

  - ##### Optional options: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibasestackedcollectionoptions.html" class="tsd-signature-type">IBaseStackedCollectionOptions</a>

    Optional parameters of type [IBaseStackedCollectionOptions](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibasestackedcollectionoptions.html) to configure the series

    ------------------------------------------------------------------------

    ðŸ“š Docs: <a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/stacked-mountain-renderable-series/" class="external">https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/stacked-mountain-renderable-series/</a>

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html" class="tsd-signature-type">StackedMountainCollection</a>

## Properties

### Protected accumulatedFinalAnimationValues0

accumulatedFinalAnimationValues0: SCRTDoubleVector

### accumulatedValues0

accumulatedValues0: SCRTDoubleVector

the accumulated values which are used to draw each column/band for [BaseStackedRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/basestackedrenderableseries.html)

### Protected animationFSM

animationFSM: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesanimationfinitestatemachine.html" class="tsd-signature-type">SeriesAnimationFiniteStateMachine</a>

### Readonly collectionChanged

collectionChanged: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/eventhandler.html" class="tsd-signature-type">EventHandler</a>\<<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/observablearraychangedargs.html" class="tsd-signature-type">ObservableArrayChangedArgs</a>\>

Event handler which fires when the collection changes. See [ObservableArrayChangedArgs](https://www.scichart.com/documentation/js/v4/typedoc/classes/observablearraychangedargs.html) for args

### Protected currentRenderPassData

currentRenderPassData: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassdata.html" class="tsd-signature-type">RenderPassData</a>

### Protected dataLabelProviderProperty

dataLabelProviderProperty: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basedatalabelprovider.html" class="tsd-signature-type">BaseDataLabelProvider</a>

### enableDrawingOptimisations

enableDrawingOptimisations: boolean = true

Readonly. When true, resampling modes are enabled for faster drawing performance.

### Protected firstAnimationRender

firstAnimationRender: boolean = false

### Readonly id

id: string = generateGuid()

A unique Id for the [IRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries.html)

### invalidateParentCallback

invalidateParentCallback: () =\> void

A callback which tells the parent [SciChartSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html) that it must be redrawn, e.g. when a property changes

#### Type declaration

- - (): void

  <!-- -->

  - #### Returns void

### Protected isAccumulatedVectorDirty

isAccumulatedVectorDirty: boolean = true

### Readonly isCollection

isCollection: boolean = true

Returns true if the series is a collection of other series.

### Readonly isPolar

isPolar: boolean = false

### Readonly isSpline

isSpline: false = false

Returns true if the series uses spline interpolation

### Readonly isStacked

isStacked: true = true

Returns true if the series is a stacked series or not

### Protected items

items: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html" class="tsd-signature-type">StackedMountainRenderableSeries</a>\[\] = \[\]

### parentSurface

parentSurface: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html" class="tsd-signature-type">SciChartSurface</a>

The parent [SciChartSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html) that this RenderableSeries is attached to

### Readonly supportsResampling

supportsResampling: false = false

Returns true if the series supports resampling

### Readonly type

type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html" class="tsd-signature-type">ESeriesType</a>

### Protected webAssemblyContext

webAssemblyContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart" class="tsd-signature-type">TSciChart</a>

The [SciChart WebAssembly Context](https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart) containing native methods and access to our WebGL2 WebAssembly Drawing Engine

## Accessors

### animation

- set animation(value: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesanimation.html" class="tsd-signature-type">SeriesAnimation</a>): void

<!-- -->

- Sets a start up animation class, a child class for {@link BaseAnimation}

  #### Parameters

  - ##### value: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesanimation.html" class="tsd-signature-type">SeriesAnimation</a>

  #### Returns void

### Protected canDraw

- get canDraw(): boolean

<!-- -->

- #### Returns boolean

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
- set dataSeries(value: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idataseries.html" class="tsd-signature-type">IDataSeries</a>): void

<!-- -->

- dataSeries property is not supported for BaseStackedCollection

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idataseries.html" class="tsd-signature-type">IDataSeries</a>

- dataSeries property is not supported for BaseStackedCollection

  #### Parameters

  - ##### value: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idataseries.html" class="tsd-signature-type">IDataSeries</a>

  #### Returns void

### drawNaNAs

- get drawNaNAs(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/elinedrawmode.html" class="tsd-signature-type">ELineDrawMode</a>
- set drawNaNAs(value: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/elinedrawmode.html" class="tsd-signature-type">ELineDrawMode</a>): void

<!-- -->

- drawNaNAs property is not supported for BaseStackedCollection

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/elinedrawmode.html" class="tsd-signature-type">ELineDrawMode</a>

- drawNaNAs property is not supported for BaseStackedCollection

  #### Parameters

  - ##### value: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/elinedrawmode.html" class="tsd-signature-type">ELineDrawMode</a>

  #### Returns void

### drawingProviders

- get drawingProviders(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iseriesdrawingprovider.html" class="tsd-signature-type">ISeriesDrawingProvider</a>\[\]
- set drawingProviders(value: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iseriesdrawingprovider.html" class="tsd-signature-type">ISeriesDrawingProvider</a>\[\]): void

<!-- -->

- drawingProviders property is not supported for BaseStackedCollection

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iseriesdrawingprovider.html" class="tsd-signature-type">ISeriesDrawingProvider</a>\[\]

- drawingProviders property is not supported for BaseStackedCollection

  #### Parameters

  - ##### value: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iseriesdrawingprovider.html" class="tsd-signature-type">ISeriesDrawingProvider</a>\[\]

  #### Returns void

### effect

- get effect(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/shadereffect.html" class="tsd-signature-type">ShaderEffect</a>
- set effect(effect: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/shadereffect.html" class="tsd-signature-type">ShaderEffect</a>): void

<!-- -->

- effect property is not supported for BaseStackedCollection

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/shadereffect.html" class="tsd-signature-type">ShaderEffect</a>

- effect property is not supported for BaseStackedCollection

  #### Parameters

  - ##### effect: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/shadereffect.html" class="tsd-signature-type">ShaderEffect</a>

  #### Returns void

### hitTestProvider

- get hitTestProvider(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ihittestprovider.html" class="tsd-signature-type">IHitTestProvider</a>
- set hitTestProvider(value: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ihittestprovider.html" class="tsd-signature-type">IHitTestProvider</a>): void

<!-- -->

- hitTestProvider property is not supported for BaseStackedCollection

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ihittestprovider.html" class="tsd-signature-type">IHitTestProvider</a>

- hitTestProvider property is not supported for BaseStackedCollection

  #### Parameters

  - ##### value: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ihittestprovider.html" class="tsd-signature-type">IHitTestProvider</a>

  #### Returns void

### hovered

- get hovered(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/eventhandler.html" class="tsd-signature-type">EventHandler</a>\<<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/serieshoveredargs.html" class="tsd-signature-type">SeriesHoveredArgs</a>\>

<!-- -->

- A hovered EventHandler. This event fires whenever the [Series](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries.html) is hovered or unhovered by a mouse or pointer.

  inheritdoc  

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/eventhandler.html" class="tsd-signature-type">EventHandler</a>\<<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/serieshoveredargs.html" class="tsd-signature-type">SeriesHoveredArgs</a>\>

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

### isOneHundredPercent

- get isOneHundredPercent(): boolean
- set isOneHundredPercent(value: boolean): void

<!-- -->

- Gets or sets 100% mode. When true, the stacked group becomes a 100% stacked chart

  #### Returns boolean

- Gets or sets 100% mode. When true, the stacked group becomes a 100% stacked chart

  #### Parameters

  - ##### value: boolean

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

### isVisibleChanged

- get isVisibleChanged(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/eventhandler.html" class="tsd-signature-type">EventHandler</a>\<<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesvisiblechangedargs.html" class="tsd-signature-type">SeriesVisibleChangedArgs</a>\>

<!-- -->

- An isVisible changed EventHandler. This event fires whenever the [Series](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries.html) isVisible changes.

  inheritdoc  

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/eventhandler.html" class="tsd-signature-type">EventHandler</a>\<<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesvisiblechangedargs.html" class="tsd-signature-type">SeriesVisibleChangedArgs</a>\>

### opacity

- get opacity(): number
- set opacity(value: number): void

<!-- -->

- opacity property is not supported for BaseStackedCollection

  #### Returns number

- opacity property is not supported for BaseStackedCollection

  #### Parameters

  - ##### value: number

  #### Returns void

### paletteProvider

- get paletteProvider(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipaletteprovider.html" class="tsd-signature-type">IPaletteProvider</a>
- set paletteProvider(value: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipaletteprovider.html" class="tsd-signature-type">IPaletteProvider</a>): void

<!-- -->

- paletteProvider property is not supported for BaseStackedCollection

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipaletteprovider.html" class="tsd-signature-type">IPaletteProvider</a>

- paletteProvider property is not supported for BaseStackedCollection

  #### Parameters

  - ##### value: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipaletteprovider.html" class="tsd-signature-type">IPaletteProvider</a>

  #### Returns void

### pointMarker

- get pointMarker(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmarker.html" class="tsd-signature-type">IPointMarker</a>
- set pointMarker(value: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmarker.html" class="tsd-signature-type">IPointMarker</a>): void

<!-- -->

- pointMarker property is not supported for BaseStackedCollection

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmarker.html" class="tsd-signature-type">IPointMarker</a>

- pointMarker property is not supported for BaseStackedCollection

  #### Parameters

  - ##### value: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmarker.html" class="tsd-signature-type">IPointMarker</a>

  #### Returns void

### resamplingMode

- get resamplingMode(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eresamplingmode.html" class="tsd-signature-type">EResamplingMode</a>
- set resamplingMode(value: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eresamplingmode.html" class="tsd-signature-type">EResamplingMode</a>): void

<!-- -->

- resamplingMode property is not supported for BaseStackedCollection

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eresamplingmode.html" class="tsd-signature-type">EResamplingMode</a>

- resamplingMode property is not supported for BaseStackedCollection

  #### Parameters

  - ##### value: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eresamplingmode.html" class="tsd-signature-type">EResamplingMode</a>

  #### Returns void

### resamplingPrecision

- get resamplingPrecision(): number
- set resamplingPrecision(value: number): void

<!-- -->

- resamplingPrecision property is not supported for BaseStackedCollection

  #### Returns number

- resamplingPrecision property is not supported for BaseStackedCollection

  #### Parameters

  - ##### value: number

  #### Returns void

### rolloverModifierProps

- get rolloverModifierProps(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rollovermodifierrenderableseriesprops.html" class="tsd-signature-type">RolloverModifierRenderableSeriesProps</a>
- set rolloverModifierProps(value: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rollovermodifierrenderableseriesprops.html" class="tsd-signature-type">RolloverModifierRenderableSeriesProps</a>): void

<!-- -->

- rolloverModifierProps property is not supported for BaseStackedCollection

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rollovermodifierrenderableseriesprops.html" class="tsd-signature-type">RolloverModifierRenderableSeriesProps</a>

- rolloverModifierProps property is not supported for BaseStackedCollection

  #### Parameters

  - ##### value: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rollovermodifierrenderableseriesprops.html" class="tsd-signature-type">RolloverModifierRenderableSeriesProps</a>

  #### Returns void

### rolloverModifierProps1

- get rolloverModifierProps1(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rollovermodifierrenderableseriesprops.html" class="tsd-signature-type">RolloverModifierRenderableSeriesProps</a>
- set rolloverModifierProps1(value: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rollovermodifierrenderableseriesprops.html" class="tsd-signature-type">RolloverModifierRenderableSeriesProps</a>): void

<!-- -->

- rolloverModifierProps1() is not supported for BaseStackedCollection

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rollovermodifierrenderableseriesprops.html" class="tsd-signature-type">RolloverModifierRenderableSeriesProps</a>

- rolloverModifierProps1() is not supported for BaseStackedCollection

  #### Parameters

  - ##### value: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rollovermodifierrenderableseriesprops.html" class="tsd-signature-type">RolloverModifierRenderableSeriesProps</a>

  #### Returns void

### selected

- get selected(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/eventhandler.html" class="tsd-signature-type">EventHandler</a>\<<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesselectedargs.html" class="tsd-signature-type">SeriesSelectedArgs</a>\>

<!-- -->

- A selected EventHandler. This event fires whenever the [Series](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries.html) is selected or deselected.

  inheritdoc  

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/eventhandler.html" class="tsd-signature-type">EventHandler</a>\<<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesselectedargs.html" class="tsd-signature-type">SeriesSelectedArgs</a>\>

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
- set stroke(value: string): void

<!-- -->

- stroke property is not supported for BaseStackedCollection

  #### Returns string

- stroke property is not supported for BaseStackedCollection

  #### Parameters

  - ##### value: string

  #### Returns void

### strokeThickness

- get strokeThickness(): number
- set strokeThickness(value: number): void

<!-- -->

- strokeThickness property is not supported for BaseStackedCollection

  #### Returns number

- strokeThickness property is not supported for BaseStackedCollection

  #### Parameters

  - ##### value: number

  #### Returns void

### xAxis

- get xAxis(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase2d.html" class="tsd-signature-type">AxisBase2D</a> \| undefined

<!-- -->

- Gets the bound [XAxis](https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase2d.html) for this [IRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries.html).

  inheritdoc  

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase2d.html" class="tsd-signature-type">AxisBase2D</a> \| undefined

### xAxisId

- get xAxisId(): string \| undefined
- set xAxisId(id: string): void

<!-- -->

- inheritdoc  

  #### Returns string \| undefined

- inheritdoc  

  #### Parameters

  - ##### id: string

  #### Returns void

### yAxis

- get yAxis(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase2d.html" class="tsd-signature-type">AxisBase2D</a> \| undefined

<!-- -->

- Gets the bound [YAxis](https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase2d.html) for this [IRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries.html).

  inheritdoc  

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase2d.html" class="tsd-signature-type">AxisBase2D</a> \| undefined

### yAxisId

- get yAxisId(): string \| undefined
- set yAxisId(id: string): void

<!-- -->

- inheritdoc  

  #### Returns string \| undefined

- inheritdoc  

  #### Parameters

  - ##### id: string

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

### add

- add(...items: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html" class="tsd-signature-type">StackedMountainRenderableSeries</a>\[\]): void

<!-- -->

- Adds items to the array, and raises the [collectionChanged](https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#collectionchanged) event to subscribers

  #### Parameters

  - ##### Rest ...items: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html" class="tsd-signature-type">StackedMountainRenderableSeries</a>\[\]

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

### asArray

- asArray(): T\[\]

<!-- -->

- Returns the backing array. Do not modify this collection. Use add or remove instead.

  #### Returns T\[\]

### Protected beforeAnimationStart

- beforeAnimationStart(): void

<!-- -->

- Runs before the animation starts

  #### Returns void

### checkIdUniqueness

- checkIdUniqueness(id: string): boolean

<!-- -->

- Checks uniqueness of Id

  #### Parameters

  - ##### id: string

  #### Returns boolean

### checkIsOutOfDataRange

- checkIsOutOfDataRange(xValue: number, yValue: number): boolean

<!-- -->

- checkIsOutOfDataRange() is not supported for BaseStackedCollection

  #### Parameters

  - ##### xValue: number

  - ##### yValue: number

  #### Returns boolean

### clear

- clear(callDeleteOnChildren?: boolean): void

<!-- -->

- Clears the array. Raises the [collectionChanged](https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#collectionchanged) event to subscribers

  #### Parameters

  - ##### Default value callDeleteOnChildren: boolean = true

    When true, if the items in the array implement the [IDeletable](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html) interface, the delete() function will be called. Defaults to true

  #### Returns void

### contains

- contains(item: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html" class="tsd-signature-type">StackedMountainRenderableSeries</a>): boolean

<!-- -->

- Returns true if the array contains an item

  #### Parameters

  - ##### item: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html" class="tsd-signature-type">StackedMountainRenderableSeries</a>

  #### Returns boolean

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

### get

- get(index: number): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html" class="tsd-signature-type">StackedMountainRenderableSeries</a>

<!-- -->

- Gets an item at index

  #### Parameters

  - ##### index: number

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html" class="tsd-signature-type">StackedMountainRenderableSeries</a>

### getBaseXValues

- getBaseXValues(): number\[\]

<!-- -->

- inheritdoc  

  #### Returns number\[\]

### getById

- getById(id: string): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html" class="tsd-signature-type">StackedMountainRenderableSeries</a>

<!-- -->

- Gets an item by Id

  #### Parameters

  - ##### id: string

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html" class="tsd-signature-type">StackedMountainRenderableSeries</a>

### getCurrentRenderPassData

- getCurrentRenderPassData(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassdata.html" class="tsd-signature-type">RenderPassData</a>

<!-- -->

- getCurrentRenderPassData method is not supported for BaseStackedCollection

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

- Returns [IDataSeries.count](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idataseries.html#count) for the linked [dataSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#dataseries)

  #### Returns number

### Protected getFirstSeries

- getFirstSeries(): T

<!-- -->

- Gets the first series in the collection, else undefined

  #### Returns T

### getIndicesRange

- getIndicesRange(xRange: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

<!-- -->

- inheritdoc  

  #### Parameters

  - ##### xRange: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

### getNativeXValues

- getNativeXValues(): SCRTDoubleVector

<!-- -->

- Returns the [IDataSeries.getNativeXValues](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idataseries.html#getnativexvalues) for the associated [dataSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#dataseries)

  #### Returns SCRTDoubleVector

### Protected getParentSurface

- getParentSurface(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html" class="tsd-signature-type">SciChartSurface</a>

<!-- -->

- Gets the parent [SciChartSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html)

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html" class="tsd-signature-type">SciChartSurface</a>

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

### getResamplingParams

- getResamplingParams(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/resamplingparams.html" class="tsd-signature-type">ResamplingParams</a>

<!-- -->

- Gets the ResamplingParams for this render. This will be undefined until needsResampling is called.

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/resamplingparams.html" class="tsd-signature-type">ResamplingParams</a>

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

### getVisibleSeries

- getVisibleSeries(): T\[\]

<!-- -->

- Gets visible renderable series array

  #### Returns T\[\]

### getXRange

- getXRange(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

<!-- -->

- Gets the X-Range of the series. Override in derived classes to provide series specific implementations

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

### getYRange

- getYRange(xVisibleRange: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>, isXCategoryAxis: boolean): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

<!-- -->

- Gets the Y-Range of the series for the current X-Range. Override in derived classes to provide series specific implementations

  #### Parameters

  - ##### xVisibleRange: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

    The [AxisCore.visibleRange](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#visiblerange) for the current bound XAxis

  - ##### isXCategoryAxis: boolean

    Whether the current bound [XAxis](https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase2d.html) is a Category axis

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

### hasDataSeries

- hasDataSeries(): boolean

<!-- -->

- Returns true if the [BaseRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries.html) has a [dataSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#dataseries)

  #### Returns boolean

### hasDataSeriesValues

- hasDataSeriesValues(): boolean

<!-- -->

- Returns true if the [BaseRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries.html) has a [dataSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#dataseries) and [IDataSeries.hasValues](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idataseries.html#hasvalues) is true

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

### insert

- insert(index: number, item: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html" class="tsd-signature-type">StackedMountainRenderableSeries</a>): void

<!-- -->

- Inserts items at the specified index. Raises the [collectionChanged](https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#collectionchanged) event to subscribers

  #### Parameters

  - ##### index: number

  - ##### item: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html" class="tsd-signature-type">StackedMountainRenderableSeries</a>

  #### Returns void

### Protected invalidateParent

- invalidateParent(): void

<!-- -->

- notifies listeners to [invalidateParentCallback](https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#invalidateparentcallback) and redraws the [SciChartSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html)

  #### Returns void

### Protected isAllDataSeriesSet

- isAllDataSeriesSet(): boolean

<!-- -->

- #### Returns boolean

### Protected isEnoughDataToDraw

- isEnoughDataToDraw(): boolean

<!-- -->

- #### Returns boolean

### linkAxes

- linkAxes(): void

<!-- -->

- Links the item to axes

  #### Returns void

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

### onDetach

- onDetach(): void

<!-- -->

- Called when the [BaseRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries.html) is detached from a [SciChartSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html)

  #### Returns void

### onDpiChanged

- onDpiChanged(args: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tdpichangedeventargs" class="tsd-signature-type">TDpiChangedEventArgs</a>): void

<!-- -->

- Called when the Dpi changes in the browser. This could be due to user zooming the browser, or changing DPI settings in Windows, or moving the browser containing SciChart to another monitor

  #### Parameters

  - ##### args: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tdpichangedeventargs" class="tsd-signature-type">TDpiChangedEventArgs</a>

    The [TDpiChangedEventArgs](https://www.scichart.com/documentation/js/v4/typedoc/index.html#tdpichangedeventargs) containing info about the Dpi Changed event

  #### Returns void

### Optional pushPalettedColors

- pushPalettedColors(color: number, palettingState: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tpalettingstate" class="tsd-signature-type">TPalettingState</a>): void

<!-- -->

- #### Parameters

  - ##### color: number

    The color for palette

  - ##### palettingState: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tpalettingstate" class="tsd-signature-type">TPalettingState</a>

  #### Returns void

### remove

- remove(item: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html" class="tsd-signature-type">StackedMountainRenderableSeries</a>, callDeleteOnChildren?: boolean): void

<!-- -->

- Removes an item by value. Raises the [collectionChanged](https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#collectionchanged) event to subscribers

  #### Parameters

  - ##### item: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html" class="tsd-signature-type">StackedMountainRenderableSeries</a>

    The item to remove

  - ##### Default value callDeleteOnChildren: boolean = true

    When true, if the items in the array implement the [IDeletable](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html) interface, the delete() function will be called. Defaults to true

  #### Returns void

### removeAt

- removeAt(index: number, callDeleteOnChildren?: boolean): void

<!-- -->

- Removes an item at the specified index. Raises the [collectionChanged](https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#collectionchanged) event to subscribers

  #### Parameters

  - ##### index: number

    The item to remove

  - ##### Default value callDeleteOnChildren: boolean = true

    When true, if the items in the array implement the [IDeletable](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html) interface, the delete() function will be called. Defaults to true

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

### set

- set(index: number, item: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html" class="tsd-signature-type">StackedMountainRenderableSeries</a>): void

<!-- -->

- Sets an item at index. Raises the [collectionChanged](https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountaincollection.html#collectionchanged) event to subscribers

  #### Parameters

  - ##### index: number

  - ##### item: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedmountainrenderableseries.html" class="tsd-signature-type">StackedMountainRenderableSeries</a>

  #### Returns void

### setAccumulatedValuesDirty

- setAccumulatedValuesDirty(): void

<!-- -->

- Notify the collection that the accumulated values need to be recalculated

  #### Returns void

### setCurrentRenderPassData

- setCurrentRenderPassData(renderPassData: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassdata.html" class="tsd-signature-type">RenderPassData</a>): void

<!-- -->

- Sets the RenderPassData instance used for this render pass

  #### Parameters

  - ##### renderPassData: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassdata.html" class="tsd-signature-type">RenderPassData</a>

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

### size

- size(): number

<!-- -->

- gets the number of elements in the array

  #### Returns number

### toJSON

- toJSON(excludeData?: boolean): <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tseriesdefinition" class="tsd-signature-type">TSeriesDefinition</a>

<!-- -->

- Convert the object to a definition that can be serialized to JSON, or used directly with the builder api

  #### Parameters

  - ##### Default value excludeData: boolean = false

    if set true, data values will not be included in the json.

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tseriesdefinition" class="tsd-signature-type">TSeriesDefinition</a>

### toPointSeries

- toPointSeries(resamplingParams?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/resamplingparams.html" class="tsd-signature-type">ResamplingParams</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointseries.html" class="tsd-signature-type">IPointSeries</a>

<!-- -->

- Returns a dataset for drawing on the viewport

  #### Parameters

  - ##### Optional resamplingParams: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/resamplingparams.html" class="tsd-signature-type">ResamplingParams</a>

    The resampling parameters

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointseries.html" class="tsd-signature-type">IPointSeries</a>

### updateAccumulatedVectors

- updateAccumulatedVectors(): void

<!-- -->

- inheritdoc  

  #### Returns void

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

### Protected updateHitTestProviders

- updateHitTestProviders(renderPassData: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassdata.html" class="tsd-signature-type">RenderPassData</a>): void

<!-- -->

- #### Parameters

  - ##### renderPassData: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassdata.html" class="tsd-signature-type">RenderPassData</a>

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
