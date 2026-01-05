<img src="out_scichartv4/typedoc/classes/polarxyscatterrenderableseries_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [PolarXyScatterRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html)

# Class PolarXyScatterRenderableSeries

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

Defines a polar scatter-series or scatter chart type in the SciChart's High Performance Real-time <a href="https://www.scichart.com/javascript-chart-features" class="external">JavaScript Charts</a>

remarks  
To add a scatter series to a [SciChartPolarSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartpolarsurface.html) you need to declare both the [RenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html) and a [DataSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html). Simplified code sample below:

``` ts
const { sciChartSurface, wasmContext } = SciChartPolarSurface.create(rootId);

// Create the renderableSeries
const polarScatterSeries = new PolarXyScatterRenderableSeries(wasmContext, {
     dataSeries: new XyDataSeries(wasmContext, {
         xValues: [1, 2, 3],
         yValues: [3, 2, 4],
     }),
     pointMarker: new EllipsePointMarker(wasmContext, {
         width: 9,
         height: 9,
         fill: "#FF0000",
         stroke: "#0000FF",
         strokeThickness: 1
     })
});
// append to the SciChartSurface
sciChartPolarSurface.renderableSeries.add(polarScatterSeries);
```

------------------------------------------------------------------------

ðŸ“š Docs: <a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-xy-scatter-renderable-series/" class="external">https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-xy-scatter-renderable-series/</a>

### Hierarchy

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyscatterrenderableseries.html" class="tsd-signature-type">XyScatterRenderableSeries</a>
  - PolarXyScatterRenderableSeries

### Implements

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html" class="tsd-signature-type">IDeletable</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries.html" class="tsd-signature-type">IRenderableSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iorderedrenderable.html" class="tsd-signature-type">IOrderedRenderable</a>

## Index

### Constructors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#constructor" class="tsd-kind-icon">constructor</a>

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#animationfsm" class="tsd-kind-icon">animationFSM</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#animationqueue" class="tsd-kind-icon">animationQueue</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#cliptototalangle" class="tsd-kind-icon">clipToTotalAngle</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#currentrenderpassdata" class="tsd-kind-icon">currentRenderPassData</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#datalabelproviderproperty" class="tsd-kind-icon">dataLabelProviderProperty</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#dataseriesproperty" class="tsd-kind-icon">dataSeriesProperty</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#hittestprovider" class="tsd-kind-icon">hitTestProvider</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#hovered" class="tsd-kind-icon">hovered</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#id" class="tsd-kind-icon">id</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#invalidateparentcallback" class="tsd-kind-icon">invalidateParentCallback</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#iscollection" class="tsd-kind-icon">isCollection</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#ispolar" class="tsd-kind-icon">isPolar</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#isstacked" class="tsd-kind-icon">isStacked</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#isvisiblechanged" class="tsd-kind-icon">isVisibleChanged</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#opacityproperty" class="tsd-kind-icon">opacityProperty</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#paletteproviderproperty" class="tsd-kind-icon">paletteProviderProperty</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#pointseries" class="tsd-kind-icon">pointSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#renderdatatransformproperty" class="tsd-kind-icon">renderDataTransformProperty</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#resamplerhelper" class="tsd-kind-icon">resamplerHelper</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#resamplingparams" class="tsd-kind-icon">resamplingParams</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#rollovermodifierprops" class="tsd-kind-icon">rolloverModifierProps</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#rollovermodifierprops1" class="tsd-kind-icon">rolloverModifierProps1</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#selected" class="tsd-kind-icon">selected</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#transformedrenderpassdata" class="tsd-kind-icon">transformedRenderPassData</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#type" class="tsd-kind-icon">type</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#typemap" class="tsd-kind-icon">typeMap</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#webassemblycontext" class="tsd-kind-icon">webAssemblyContext</a>

### Accessors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#animation" class="tsd-kind-icon">animation</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#cliptoyrange" class="tsd-kind-icon">clipToYRange</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#datalabelprovider" class="tsd-kind-icon">dataLabelProvider</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#dataseries" class="tsd-kind-icon">dataSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#drawnanas" class="tsd-kind-icon">drawNaNAs</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#drawingproviders" class="tsd-kind-icon">drawingProviders</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#effect" class="tsd-kind-icon">effect</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#enabledrawingoptimisations" class="tsd-kind-icon">enableDrawingOptimisations</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#isdigitalline" class="tsd-kind-icon">isDigitalLine</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#ishovered" class="tsd-kind-icon">isHovered</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#isrunninganimation" class="tsd-kind-icon">isRunningAnimation</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#isrunningdataanimation" class="tsd-kind-icon">isRunningDataAnimation</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#isselected" class="tsd-kind-icon">isSelected</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#isspline" class="tsd-kind-icon">isSpline</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#isvisible" class="tsd-kind-icon">isVisible</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#opacity" class="tsd-kind-icon">opacity</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#paletteprovider" class="tsd-kind-icon">paletteProvider</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#parentsurface" class="tsd-kind-icon">parentSurface</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#pointmarker" class="tsd-kind-icon">pointMarker</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#renderdatatransform" class="tsd-kind-icon">renderDataTransform</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#resamplingmode" class="tsd-kind-icon">resamplingMode</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#resamplingprecision" class="tsd-kind-icon">resamplingPrecision</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#seriesname" class="tsd-kind-icon">seriesName</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#stroke" class="tsd-kind-icon">stroke</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#strokethickness" class="tsd-kind-icon">strokeThickness</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#supportsresampling" class="tsd-kind-icon">supportsResampling</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#xaxis" class="tsd-kind-icon">xAxis</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#xaxisid" class="tsd-kind-icon">xAxisId</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#yarrayfilter" class="tsd-kind-icon">yArrayFilter</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#yaxis" class="tsd-kind-icon">yAxis</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#yaxisid" class="tsd-kind-icon">yAxisId</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#yrangemode" class="tsd-kind-icon">yRangeMode</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#adddrawingproviders" class="tsd-kind-icon">addDrawingProviders</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#adjustautocolor" class="tsd-kind-icon">adjustAutoColor</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#afteranimationcomplete" class="tsd-kind-icon">afterAnimationComplete</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#applytheme" class="tsd-kind-icon">applyTheme</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#beforeanimationstart" class="tsd-kind-icon">beforeAnimationStart</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#checkisoutofdatarange" class="tsd-kind-icon">checkIsOutOfDataRange</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#dataseriesdatachanged" class="tsd-kind-icon">dataSeriesDataChanged</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#delete" class="tsd-kind-icon">delete</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#draw" class="tsd-kind-icon">draw</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#enqueueanimation" class="tsd-kind-icon">enqueueAnimation</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#getcurrentrenderpassdata" class="tsd-kind-icon">getCurrentRenderPassData</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#getdatapointwidth" class="tsd-kind-icon">getDataPointWidth</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#getdataseriesname" class="tsd-kind-icon">getDataSeriesName</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#getdataseriesvaluescount" class="tsd-kind-icon">getDataSeriesValuesCount</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#getindicesrange" class="tsd-kind-icon">getIndicesRange</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#getnativexvalues" class="tsd-kind-icon">getNativeXValues</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#getnativeyvalues" class="tsd-kind-icon">getNativeYValues</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#getrenderlayer" class="tsd-kind-icon">getRenderLayer</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#getrendernextto" class="tsd-kind-icon">getRenderNextTo</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#getrenderorder" class="tsd-kind-icon">getRenderOrder</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#getresampledpointseries" class="tsd-kind-icon">getResampledPointSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#getresamplingparams" class="tsd-kind-icon">getResamplingParams</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#getseriesinfo" class="tsd-kind-icon">getSeriesInfo</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#getsurfacerenderorder" class="tsd-kind-icon">getSurfaceRenderOrder</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#getxrange" class="tsd-kind-icon">getXRange</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#getyrange" class="tsd-kind-icon">getYRange</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#hasdataseries" class="tsd-kind-icon">hasDataSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#hasdataseriesvalues" class="tsd-kind-icon">hasDataSeriesValues</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#hasfillpaletteprovider" class="tsd-kind-icon">hasFillPaletteProvider</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#haspointmarkerpaletteprovider" class="tsd-kind-icon">hasPointMarkerPaletteProvider</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#hasstrokepaletteprovider" class="tsd-kind-icon">hasStrokePaletteProvider</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#linkaxes" class="tsd-kind-icon">linkAxes</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#needsresampling" class="tsd-kind-icon">needsResampling</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#newhittestprovider" class="tsd-kind-icon">newHitTestProvider</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#notifypropertychanged" class="tsd-kind-icon">notifyPropertyChanged</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#onanimate" class="tsd-kind-icon">onAnimate</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#onattach" class="tsd-kind-icon">onAttach</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#ondetach" class="tsd-kind-icon">onDetach</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#ondpichanged" class="tsd-kind-icon">onDpiChanged</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#pushpalettedcolors" class="tsd-kind-icon">pushPalettedColors</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#resolveautocolors" class="tsd-kind-icon">resolveAutoColors</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#runanimation" class="tsd-kind-icon">runAnimation</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#setanimationvectors" class="tsd-kind-icon">setAnimationVectors</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#setcurrentrenderpassdata" class="tsd-kind-icon">setCurrentRenderPassData</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#setpaletteprovider" class="tsd-kind-icon">setPaletteProvider</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#setrenderlayer" class="tsd-kind-icon">setRenderLayer</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#setrendernextto" class="tsd-kind-icon">setRenderNextTo</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#setrenderorder" class="tsd-kind-icon">setRenderOrder</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#setsurfacerenderorder" class="tsd-kind-icon">setSurfaceRenderOrder</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#tojson" class="tsd-kind-icon">toJSON</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#topointseries" class="tsd-kind-icon">toPointSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#updateanimationproperties" class="tsd-kind-icon">updateAnimationProperties</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#updatetransformedvalues" class="tsd-kind-icon">updateTransformedValues</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#valuechanged" class="tsd-kind-icon">valueChanged</a>

## Constructors

### constructor

- new PolarXyScatterRenderableSeries(webAssemblyContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart" class="tsd-signature-type">TSciChart</a>, options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipolarxyscatterrenderableseriesoptions.html" class="tsd-signature-type">IPolarXyScatterRenderableSeriesOptions</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html" class="tsd-signature-type">PolarXyScatterRenderableSeries</a>

<!-- -->

- Creates an instance of the [PolarXyScatterRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html)

  #### Parameters

  - ##### webAssemblyContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart" class="tsd-signature-type">TSciChart</a>

    The [SciChart WebAssembly Context](https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart) containing native methods and access to our WebGL2 WebAssembly Drawing Engine

  - ##### Optional options: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipolarxyscatterrenderableseriesoptions.html" class="tsd-signature-type">IPolarXyScatterRenderableSeriesOptions</a>

    Optional parameters of type [IPolarXyScatterRenderableSeriesOptions](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipolarxyscatterrenderableseriesoptions.html) to configure the series

    ------------------------------------------------------------------------

    ðŸ“š Docs: <a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-xy-scatter-renderable-series/" class="external">https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-xy-scatter-renderable-series/</a>

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html" class="tsd-signature-type">PolarXyScatterRenderableSeries</a>

## Properties

### Protected animationFSM

animationFSM: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesanimationfinitestatemachine.html" class="tsd-signature-type">SeriesAnimationFiniteStateMachine</a>

### Protected animationQueue

animationQueue: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesanimation.html" class="tsd-signature-type">SeriesAnimation</a>\[\] = \[\]

### clipToTotalAngle

clipToTotalAngle: boolean = false

### Protected currentRenderPassData

currentRenderPassData: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassdata.html" class="tsd-signature-type">RenderPassData</a>

### Protected dataLabelProviderProperty

dataLabelProviderProperty: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basedatalabelprovider.html" class="tsd-signature-type">BaseDataLabelProvider</a>

### Protected dataSeriesProperty

dataSeriesProperty: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idataseries.html" class="tsd-signature-type">IDataSeries</a>

### hitTestProvider

hitTestProvider: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ihittestprovider.html" class="tsd-signature-type">IHitTestProvider</a>

Gets the current [IHitTestProvider](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ihittestprovider.html), used to call methods [IHitTestProvider.hitTest](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ihittestprovider.html#hittest), [IHitTestProvider.hitTestXSlice](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ihittestprovider.html#hittestxslice) and [IHitTestProvider.hitTestDataPoint](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ihittestprovider.html#hittestdatapoint) and provide info about the series data-points at mouse or touch locations

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

isPolar: boolean = true

### Readonly isStacked

isStacked: boolean

Returns true if the series is a stacked series or not

### isVisibleChanged

isVisibleChanged: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/eventhandler.html" class="tsd-signature-type">EventHandler</a>\<<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesvisiblechangedargs.html" class="tsd-signature-type">SeriesVisibleChangedArgs</a>\> = new EventHandler\<SeriesVisibleChangedArgs\>()

An isVisible changed EventHandler. This event fires whenever the [Series](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries.html) isVisible changes.

remarks  
See [EventHandler](https://www.scichart.com/documentation/js/v4/typedoc/classes/eventhandler.html) for how to subscribe

### Protected opacityProperty

opacityProperty: number = 1

### Protected paletteProviderProperty

paletteProviderProperty: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipaletteprovider.html" class="tsd-signature-type">IPaletteProvider</a> = DefaultPaletteProvider.createEmpty()

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

Gets or sets [RolloverModifierRenderableSeriesProps](https://www.scichart.com/documentation/js/v4/typedoc/classes/rollovermodifierrenderableseriesprops.html) for [RolloverModifier](https://www.scichart.com/documentation/js/v4/typedoc/classes/rollovermodifier.html) tooltips

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

type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html" class="tsd-signature-type">ESeriesType</a> = ESeriesType.PolarScatterSeries

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

- How to treat NAN (Not a number) values in the input [dataSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#dataseries). See [ELineDrawMode](https://www.scichart.com/documentation/js/v4/typedoc/enums/elinedrawmode.html) for a list of values.

  inheritdoc  

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/elinedrawmode.html" class="tsd-signature-type">ELineDrawMode</a>

- How to treat NAN (Not a number) values in the input [dataSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#dataseries). See [ELineDrawMode](https://www.scichart.com/documentation/js/v4/typedoc/enums/elinedrawmode.html) for a list of values.

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

- The parent [SciChartSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html) that this RenderableSeries is attached to

  inheritdoc  

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

- Gets or sets the [EResamplingMode](https://www.scichart.com/documentation/js/v4/typedoc/enums/eresamplingmode.html) used when drawing this series. Default value is Auto. To disable resampling for this series set mode = None. Also see [resamplingPrecision](https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#resamplingprecision) which specifies the precision applied when resampling. To globally enable/disable resampling for debug purposes set [SciChartDefaults.debugDisableResampling](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartdefaults.html#debugdisableresampling)

  inheritdoc  

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eresamplingmode.html" class="tsd-signature-type">EResamplingMode</a>

- Gets or sets the [EResamplingMode](https://www.scichart.com/documentation/js/v4/typedoc/enums/eresamplingmode.html) used when drawing this series. Default value is Auto. To disable resampling for this series set mode = None. Also see [resamplingPrecision](https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#resamplingprecision) which specifies the precision applied when resampling. To globally enable/disable resampling for debug purposes set [SciChartDefaults.debugDisableResampling](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartdefaults.html#debugdisableresampling)

  inheritdoc  

  #### Parameters

  - ##### value: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eresamplingmode.html" class="tsd-signature-type">EResamplingMode</a>

  #### Returns void

### resamplingPrecision

- get resamplingPrecision(): number
- set resamplingPrecision(value: number): void

<!-- -->

- Gets or sets the resampling precision for this series.

  Default value is 0.0. Value of 1.0 means double precision: the resampler outputs 2x the number of points. Value of 2.0 means quadruple precision: the resampler outputs 4x the number of points.

  If experiencing visual artefacts, try setting the precision to 1.0 or 2.0. This will come at a minor performance cost of around 20% for large datasets.

  inheritdoc  

  #### Returns number

- Gets or sets the resampling precision for this series.

  Default value is 0.0. Value of 1.0 means double precision: the resampler outputs 2x the number of points. Value of 2.0 means quadruple precision: the resampler outputs 4x the number of points.

  If experiencing visual artefacts, try setting the precision to 1.0 or 2.0. This will come at a minor performance cost of around 20% for large datasets.

  inheritdoc  

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

### supportsResampling

- get supportsResampling(): boolean

<!-- -->

- Returns true if the series supports resampling

  inheritdoc  

  #### Returns boolean

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

### Protected addDrawingProviders

- addDrawingProviders(webAssemblyContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart" class="tsd-signature-type">TSciChart</a>, options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipolarxyscatterrenderableseriesoptions.html" class="tsd-signature-type">IPolarXyScatterRenderableSeriesOptions</a>): void

<!-- -->

- inheritdoc  

  #### Parameters

  - ##### webAssemblyContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart" class="tsd-signature-type">TSciChart</a>

  - ##### Optional options: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipolarxyscatterrenderableseriesoptions.html" class="tsd-signature-type">IPolarXyScatterRenderableSeriesOptions</a>

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

- dataSeriesDataChanged(args: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idatachangeargs.html" class="tsd-signature-type">IDataChangeArgs</a>): void

<!-- -->

- Is being called when the data for the underlying DataSeries changes

  #### Parameters

  - ##### args: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idatachangeargs.html" class="tsd-signature-type">IDataChangeArgs</a>

  #### Returns void

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

- Returns [IDataSeries.count](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idataseries.html#count) for the linked [dataSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#dataseries)

  #### Returns number

### getIndicesRange

- getIndicesRange(xRange: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>, isCategoryData?: boolean): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

<!-- -->

- Returns the indices range of data points in xRange of the associated [IDataSeries](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idataseries.html)

  #### Parameters

  - ##### xRange: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

    The X-Axis Range currently in view

  - ##### Optional isCategoryData: boolean

    If True the renderable series uses [CategoryAxis](https://www.scichart.com/documentation/js/v4/typedoc/enums/eaxistype.html#categoryaxis)

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

### getNativeXValues

- getNativeXValues(): SCRTDoubleVector

<!-- -->

- Returns the [IDataSeries.getNativeXValues](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idataseries.html#getnativexvalues) for the associated [dataSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#dataseries)

  #### Returns SCRTDoubleVector

### getNativeYValues

- getNativeYValues(): SCRTDoubleVector

<!-- -->

- Returns the [IDataSeries.getNativeYValues](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idataseries.html#getnativeyvalues) for the associated [dataSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#dataseries)

  #### Returns SCRTDoubleVector

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

- Returns true if the [BaseRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries.html) has a [dataSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#dataseries)

  #### Returns boolean

### hasDataSeriesValues

- hasDataSeriesValues(): boolean

<!-- -->

- Returns true if the [BaseRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries.html) has a [dataSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/polarxyscatterrenderableseries.html#dataseries) and [IDataSeries.hasValues](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idataseries.html#hasvalues) is true

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

- Links the item to axes

  #### Returns void

### needsResampling

- needsResampling(rp: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/resamplingparams.html" class="tsd-signature-type">ResamplingParams</a>): boolean

<!-- -->

- inheritdoc  

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

- toJSON(excludeData?: boolean): <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tseriesdefinition" class="tsd-signature-type">TSeriesDefinition</a>

<!-- -->

- Convert the object to a definition that can be serialized to JSON, or used directly with the builder api

  #### Parameters

  - ##### Default value excludeData: boolean = false

    if set true, data values will not be included in the json.

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tseriesdefinition" class="tsd-signature-type">TSeriesDefinition</a>

### toPointSeries

- toPointSeries(rp?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/resamplingparams.html" class="tsd-signature-type">ResamplingParams</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointseries.html" class="tsd-signature-type">IPointSeries</a>

<!-- -->

- Returns a dataset for drawing on the viewport

  #### Parameters

  - ##### Optional rp: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/resamplingparams.html" class="tsd-signature-type">ResamplingParams</a>

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
