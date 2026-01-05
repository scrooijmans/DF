<img src="out_scichartv4/typedoc/classes/scichart3dsurface_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [SciChart3DSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html)

# Class SciChart3DSurface

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

summary  
The [SciChart3DSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html) is the root 3D Chart control in SciChart's High Performance Real-time <a href="https://www.scichart.com/javascript-chart-features" class="external">JavaScript 3D Chart Library</a>

description  
To create a 3D chart using SciChart, declare a [SciChart3DSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html) using [SciChart3DSurface.create](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#create), add X,Y,Z axis via the [SciChart3DSurface.xAxis](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#xaxis) [SciChart3DSurface.yAxis](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#yaxis) and [SciChart3DSurface.zAxis](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#zaxis) properties.

Next, add a series or chart type by adding a [BaseRenderableSeries3D](https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries3d.html) to the [SciChart3DSurface.renderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#renderableseries) collection.

Position the camera in the 3D scene by adjusting the [SciChart3DSurface.camera](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#camera) property.

To redraw a [SciChart3DSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html) at any time, call [SciChart3DSurface.invalidateElement](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#invalidateelement), however all properties are reactive and the chart will automatically redraw if data or properties change.

remarks  
[SciChartSurfaces](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html) scale to fit the parent DIV where they are hosted. Use CSS to position the DIV.

### Hierarchy

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurfacebase.html" class="tsd-signature-type">SciChartSurfaceBase</a>
  - SciChart3DSurface

### Implements

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html" class="tsd-signature-type">IDeletable</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iscichartsurfacebase.html" class="tsd-signature-type">ISciChartSurfaceBase</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/isuspendable.html" class="tsd-signature-type">ISuspendable</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/inotifyondpichanged.html" class="tsd-signature-type">INotifyOnDpiChanged</a>

## Index

### Constructors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#constructor" class="tsd-kind-icon">constructor</a>

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#adornerlayer" class="tsd-kind-icon">adornerLayer</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#animationlist" class="tsd-kind-icon">animationList</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#annotations" class="tsd-kind-icon">annotations</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#axiscubeentity" class="tsd-kind-icon">axisCubeEntity</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#backgroundproperty" class="tsd-kind-icon">backgroundProperty</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#chartmodifiers" class="tsd-kind-icon">chartModifiers</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#cleanuplocktoken" class="tsd-kind-icon">cleanupLockToken</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#createsuspended" class="tsd-kind-icon">createSuspended</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#destinations" class="tsd-kind-icon">destinations</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#disableaspect" class="tsd-kind-icon">disableAspect</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#dombackgroundsvgcontainer" class="tsd-kind-icon">domBackgroundSvgContainer</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#domcanvas2d" class="tsd-kind-icon">domCanvas2D</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#domcanvaswebgl" class="tsd-kind-icon">domCanvasWebGL</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#domchartroot" class="tsd-kind-icon">domChartRoot</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#domdivcontainer" class="tsd-kind-icon">domDivContainer</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#domseriesbackground" class="tsd-kind-icon">domSeriesBackground</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#domsvgadornerlayer" class="tsd-kind-icon">domSvgAdornerLayer</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#domsvgcontainer" class="tsd-kind-icon">domSvgContainer</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#genericanimationsrun" class="tsd-kind-icon">genericAnimationsRun</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#hasinvalidstate" class="tsd-kind-icon">hasInvalidState</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#heightaspect" class="tsd-kind-icon">heightAspect</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#idproperty" class="tsd-kind-icon">idProperty</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#isdeletedproperty" class="tsd-kind-icon">isDeletedProperty</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#isinitializedproperty" class="tsd-kind-icon">isInitializedProperty</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#layoutmeasured" class="tsd-kind-icon">layoutMeasured</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#loaderjson" class="tsd-kind-icon">loaderJson</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#modifierannotations" class="tsd-kind-icon">modifierAnnotations</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#mousemanager" class="tsd-kind-icon">mouseManager</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#oncreatedname" class="tsd-kind-icon">onCreatedName</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#painted" class="tsd-kind-icon">painted</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#prerender" class="tsd-kind-icon">preRender</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#previousthemeproviderproperty" class="tsd-kind-icon">previousThemeProviderProperty</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#propertychanged" class="tsd-kind-icon">propertyChanged</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#redrawrequested" class="tsd-kind-icon">redrawRequested</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#renderableseries" class="tsd-kind-icon">renderableSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#rendered" class="tsd-kind-icon">rendered</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#renderedtodestination" class="tsd-kind-icon">renderedToDestination</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#renderedtowebgl" class="tsd-kind-icon">renderedToWebGl</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#rootentity" class="tsd-kind-icon">rootEntity</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#showerrors" class="tsd-kind-icon">showErrors</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#suspender" class="tsd-kind-icon">suspender</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#themeproviderproperty" class="tsd-kind-icon">themeProviderProperty</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#touchactionproperty" class="tsd-kind-icon">touchActionProperty</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#visibilityobserver" class="tsd-kind-icon">visibilityObserver</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#watermarkposition" class="tsd-kind-icon">watermarkPosition</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#webassemblycontext3d" class="tsd-kind-icon">webAssemblyContext3D</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#widthaspect" class="tsd-kind-icon">widthAspect</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#antialiaswebglbackbuffer" class="tsd-kind-icon">AntiAliasWebGlBackbuffer</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#default_theme" class="tsd-kind-icon">DEFAULT_THEME</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#autodisposewasmcontext" class="tsd-kind-icon">autoDisposeWasmContext</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#dommastercanvas" class="tsd-kind-icon">domMasterCanvas</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#invalidateontabvisible" class="tsd-kind-icon">invalidateOnTabVisible</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#wasmcontextdisposetimeout" class="tsd-kind-icon">wasmContextDisposeTimeout</a>

### Accessors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#background" class="tsd-kind-icon">background</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#camera" class="tsd-kind-icon">camera</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#chartmodifiergroups" class="tsd-kind-icon">chartModifierGroups</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#chartviewrect" class="tsd-kind-icon">chartViewRect</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#cliprect" class="tsd-kind-icon">clipRect</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#enablegizmo" class="tsd-kind-icon">enableGizmo</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#freezewhenoutofview" class="tsd-kind-icon">freezeWhenOutOfView</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#id" class="tsd-kind-icon">id</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#isaxiscuberendered" class="tsd-kind-icon">isAxisCubeRendered</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#iscopycanvassurface" class="tsd-kind-icon">isCopyCanvasSurface</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#isdeleted" class="tsd-kind-icon">isDeleted</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#ishittestenabled" class="tsd-kind-icon">isHitTestEnabled</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#isinitialized" class="tsd-kind-icon">isInitialized</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#isinvalidated" class="tsd-kind-icon">isInvalidated</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#ispolar" class="tsd-kind-icon">isPolar</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#isrunninganimation" class="tsd-kind-icon">isRunningAnimation</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#issuspended" class="tsd-kind-icon">isSuspended</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#isxyplanevisible" class="tsd-kind-icon">isXYPlaneVisible</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#iszxplanevisible" class="tsd-kind-icon">isZXPlaneVisible</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#iszyplanevisible" class="tsd-kind-icon">isZYPlaneVisible</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#othersurfaces" class="tsd-kind-icon">otherSurfaces</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#prerenderall" class="tsd-kind-icon">preRenderAll</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#previousthemeprovider" class="tsd-kind-icon">previousThemeProvider</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#seriesviewrect" class="tsd-kind-icon">seriesViewRect</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#surfacetype" class="tsd-kind-icon">surfaceType</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#themeprovider" class="tsd-kind-icon">themeProvider</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#viewrect" class="tsd-kind-icon">viewRect</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#viewportmanager" class="tsd-kind-icon">viewportManager</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#worlddimensions" class="tsd-kind-icon">worldDimensions</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#xaxis" class="tsd-kind-icon">xAxis</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#yaxis" class="tsd-kind-icon">yAxis</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#zaxis" class="tsd-kind-icon">zAxis</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#addanimation" class="tsd-kind-icon">addAnimation</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#adddeletable" class="tsd-kind-icon">addDeletable</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#applyoptions" class="tsd-kind-icon">applyOptions</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#applyscichartbackground" class="tsd-kind-icon">applySciChartBackground</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#applytheme" class="tsd-kind-icon">applyTheme</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#attachchartmodifier" class="tsd-kind-icon">attachChartModifier</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#changemastercanvasviewportsize" class="tsd-kind-icon">changeMasterCanvasViewportSize</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#changeviewportsize" class="tsd-kind-icon">changeViewportSize</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#changewebglcanvasviewportsize" class="tsd-kind-icon">changeWebGLCanvasViewportSize</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#clearrootelement" class="tsd-kind-icon">clearRootElement</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#delete" class="tsd-kind-icon">delete</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#detachchartmodifier" class="tsd-kind-icon">detachChartModifier</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#dodrawingloop" class="tsd-kind-icon">doDrawingLoop</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#getanimations" class="tsd-kind-icon">getAnimations</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#getdefaultxaxis" class="tsd-kind-icon">getDefaultXAxis</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#getdefaultyaxis" class="tsd-kind-icon">getDefaultYAxis</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#getmaincanvas" class="tsd-kind-icon">getMainCanvas</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#getnextstate" class="tsd-kind-icon">getNextState</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#getoptions" class="tsd-kind-icon">getOptions</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#getsceneworld" class="tsd-kind-icon">getSceneWorld</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#getseriesviewrectpadding" class="tsd-kind-icon">getSeriesViewRectPadding</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#getxaxisbyid" class="tsd-kind-icon">getXAxisById</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#getyaxisbyid" class="tsd-kind-icon">getYAxisById</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#invalidateelement" class="tsd-kind-icon">invalidateElement</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#nextstaterender" class="tsd-kind-icon">nextStateRender</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#notifypropertychanged" class="tsd-kind-icon">notifyPropertyChanged</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#onanimate" class="tsd-kind-icon">onAnimate</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#ondpichanged" class="tsd-kind-icon">onDpiChanged</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#onscichartrendered" class="tsd-kind-icon">onSciChartRendered</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#registerfont" class="tsd-kind-icon">registerFont</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#resumeupdates" class="tsd-kind-icon">resumeUpdates</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#setdestinations" class="tsd-kind-icon">setDestinations</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#setisaxiscuberendered" class="tsd-kind-icon">setIsAxisCubeRendered</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#setisinitialized" class="tsd-kind-icon">setIsInitialized</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#setseriesviewrect" class="tsd-kind-icon">setSeriesViewRect</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#setsvgclippathdefinitions" class="tsd-kind-icon">setSvgClipPathDefinitions</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#suspendupdates" class="tsd-kind-icon">suspendUpdates</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#tojson" class="tsd-kind-icon">toJSON</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#updatebackground" class="tsd-kind-icon">updateBackground</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#updatewatermark" class="tsd-kind-icon">updateWatermark</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#worldtoscreencoord" class="tsd-kind-icon">worldToScreenCoord</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#usecommunitylicense" class="tsd-kind-icon">UseCommunityLicense</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#configure" class="tsd-kind-icon">configure</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#create" class="tsd-kind-icon">create</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#createsingle" class="tsd-kind-icon">createSingle</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#disposesharedwasmcontext" class="tsd-kind-icon">disposeSharedWasmContext</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#loadwasmfromcdn" class="tsd-kind-icon">loadWasmFromCDN</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#loadwasmlocal" class="tsd-kind-icon">loadWasmLocal</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#resolveoptions" class="tsd-kind-icon">resolveOptions</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#setlicensecallback" class="tsd-kind-icon">setLicenseCallback</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#setruntimelicensekey" class="tsd-kind-icon">setRuntimeLicenseKey</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#setserverlicenseendpoint" class="tsd-kind-icon">setServerLicenseEndpoint</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#usewasmfromcdn" class="tsd-kind-icon">useWasmFromCDN</a>

## Constructors

### constructor

- new SciChart3DSurface(webAssemblyContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart3d" class="tsd-signature-type">TSciChart3D</a>, options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iscichart3dsurfaceoptions.html" class="tsd-signature-type">ISciChart3DSurfaceOptions</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html" class="tsd-signature-type">SciChart3DSurface</a>

<!-- -->

- Creates an instance of [SciChart3DSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html)

  #### Parameters

  - ##### webAssemblyContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart3d" class="tsd-signature-type">TSciChart3D</a>

    The [SciChart 3D WebAssembly Context](https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart3d) containing native methods and access to our WebGL2 Engine and WebAssembly numerical methods

  - ##### Optional options: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iscichart3dsurfaceoptions.html" class="tsd-signature-type">ISciChart3DSurfaceOptions</a>

    Optional parameters of type [ISciChart3DSurfaceOptions](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iscichart3dsurfaceoptions.html) to configure the chart

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html" class="tsd-signature-type">SciChart3DSurface</a>

## Properties

### adornerLayer

adornerLayer: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/adornerlayer.html" class="tsd-signature-type">AdornerLayer</a>

### Protected animationList

animationList: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/igenericanimation.html" class="tsd-signature-type">IGenericAnimation</a>\[\] = \[\]

### Readonly annotations

annotations: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/observablearray.html" class="tsd-signature-type">ObservableArray</a>\<<a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iannotation.html" class="tsd-signature-type">IAnnotation</a>\>

summary  
Gets the collection of [IAnnotation](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iannotation.html) - annotations, markers or shapes drawn over the top of a [SciChartSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html)

description  
A [SciChartSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html) can have zero to many [Annotations](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iannotation.html).

The Annotations are drawn using our WebGL / WebAssembly rendering engine, but some use SVG for maximum configurability. See derived types of [IAnnotation](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iannotation.html) such as [BoxAnnotation](https://www.scichart.com/documentation/js/v4/typedoc/classes/boxannotation.html), [LineAnnotation](https://www.scichart.com/documentation/js/v4/typedoc/classes/lineannotation.html) etc...

Use this collection to add and remove Annotations to the chart.

remarks  
Adding an Annotation to the chart causes it to automatically redraw. Note that annotations do not pariticpate in autoranging, meaning a chart will zoom to fit data and chart series but not annotations

### Readonly axisCubeEntity

axisCubeEntity: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscubeentity.html" class="tsd-signature-type">AxisCubeEntity</a>

The [AxisCubeEntity](https://www.scichart.com/documentation/js/v4/typedoc/enums/esceneentitytype.html#axiscubeentity) is a 3D Scene Entity (inherits [BaseSceneEntity3D](https://www.scichart.com/documentation/js/v4/typedoc/classes/basesceneentity3d.html)) which renders the 3D X,Y,Z axis cube, axis walls and labels in a [SciChart3DSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html)

### Protected backgroundProperty

backgroundProperty: string

### Readonly chartModifiers

chartModifiers: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/observablearray.html" class="tsd-signature-type">ObservableArray</a>\<<a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ichartmodifierbase.html" class="tsd-signature-type">IChartModifierBase</a>\>

An [ObservableArray](https://www.scichart.com/documentation/js/v4/typedoc/classes/observablearray.html) of [IChartModifierBase](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ichartmodifierbase.html) derived types. Chart Modifiers provide behavior such as zooming, panning, tooltips, legends and more in SciChart's High Performance Realtime <a href="https://www.scichart.com/javascript-chart-features" class="external">JavaScript Charts</a>. You can use our built in modifiers (see derived types of [ChartModifierBase](https://www.scichart.com/documentation/js/v4/typedoc/classes/chartmodifierbase.html), or create your own for custom interaction behavior.

### Protected cleanupLockToken

cleanupLockToken: () =\> boolean

#### Type declaration

- - (): boolean

  <!-- -->

  - #### Returns boolean

### Protected createSuspended

createSuspended: boolean

### Protected destinations

destinations: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichartdestination" class="tsd-signature-type">TSciChartDestination</a>\[\]

### Protected disableAspect

disableAspect: boolean

### domBackgroundSvgContainer

domBackgroundSvgContainer: SVGSVGElement

The {@link SVGSVGElement} placed on the background and could be used instead of [domSvgContainer](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#domsvgcontainer)

### domCanvas2D

domCanvas2D: HTMLCanvasElement

The {@link HTMLCanvasElement} which is the HTML5 canvas which SciChart draws overlays (cursors, tooltips) to

### domCanvasWebGL

domCanvasWebGL: HTMLCanvasElement

The {@link HTMLCanvasElement} which is the WebGL canvas that SciChart draws to

### domChartRoot

domChartRoot: HTMLDivElement

The {@link HTMLDivElement} which is the dom chart root

### domDivContainer

domDivContainer: HTMLDivElement

The inner {@link HTMLDivElement} div element

### domSeriesBackground

domSeriesBackground: HTMLDivElement

The inner {@link HTMLDivElement} div element placed on the background

### domSvgAdornerLayer

domSvgAdornerLayer: SVGSVGElement

The {@link SVGSVGElement} which is the SVG adorner layer canvas, is used for annotation adorners

### domSvgContainer

domSvgContainer: SVGSVGElement

The {@link SVGSVGElement} which is the SVG canvas which SciChart adds elements (tooltips, annotations) to

### genericAnimationsRun

genericAnimationsRun: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/eventhandler.html" class="tsd-signature-type">EventHandler</a>\<any\> = new EventHandler\<any\>()

An event handler which notifies its subscribers when animations stage in render process has finished.

### hasInvalidState

hasInvalidState: boolean = false

### Protected heightAspect

heightAspect: number

### Protected idProperty

idProperty: string = generateGuid()

### Protected isDeletedProperty

isDeletedProperty: boolean = false

### Protected isInitializedProperty

isInitializedProperty: boolean = false

### layoutMeasured

layoutMeasured: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/eventhandler.html" class="tsd-signature-type">EventHandler</a>\<any\> = new EventHandler\<any\>()

An event handler which notifies its subscribers when a layout stage in render process has finished.

### Protected loaderJson

loaderJson: any

### Readonly modifierAnnotations

modifierAnnotations: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/observablearray.html" class="tsd-signature-type">ObservableArray</a>\<<a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iannotation.html" class="tsd-signature-type">IAnnotation</a>\>

summary  
Gets the collection of [IAnnotation](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iannotation.html) - modifier annotations, markers or shapes drawn over the top of a [SciChartSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html)

description  
A [SciChartSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html) can have zero to many [Annotations](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iannotation.html).

The Annotations are drawn using our WebGL / WebAssembly rendering engine, but some use SVG for maximum configurability. See derived types of [IAnnotation](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iannotation.html) such as [BoxAnnotation](https://www.scichart.com/documentation/js/v4/typedoc/classes/boxannotation.html), [LineAnnotation](https://www.scichart.com/documentation/js/v4/typedoc/classes/lineannotation.html) etc...

Use this collection to add and remove Modifier Annotations to the chart.

remarks  
Adding an Modifier Annotation to the chart causes it to automatically redraw. Note that annotations do not pariticpate in autoranging, meaning a chart will zoom to fit data and chart series but not annotations

### mouseManager

mouseManager: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/mousemanager.html" class="tsd-signature-type">MouseManager</a>

The [MouseManager](https://www.scichart.com/documentation/js/v4/typedoc/classes/mousemanager.html) subscribes to mouse events on the [domChartRoot](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#domchartroot) and routes them to components within SciChart

### onCreatedName

onCreatedName: string

For serialization Only. The name of the onCreated function applied by the builder api

### painted

painted: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/eventhandler.html" class="tsd-signature-type">EventHandler</a>\<any\> = new EventHandler\<any\>()

An event handler which notifies its subscribers when a chart was visually painted a display canvas.

remarks  
Not applicable to sub-charts

### preRender

preRender: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/eventhandler.html" class="tsd-signature-type">EventHandler</a>\<void\> = new EventHandler\<void\>()

An event handler which notifies its subscribers when a render operation starts. Use this to update elements of the chart for the current render. Any updates made here will not trigger a subsequent render.

### Protected previousThemeProviderProperty

previousThemeProviderProperty: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html" class="tsd-signature-type">IThemeProvider</a> = SciChartSurfaceBase.DEFAULT_THEME

### Readonly propertyChanged

propertyChanged: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/eventhandler.html" class="tsd-signature-type">EventHandler</a>\<<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/propertychangedeventargs.html" class="tsd-signature-type">PropertyChangedEventArgs</a>\>

A propertyChanged EventHandler. See [EventHandler](https://www.scichart.com/documentation/js/v4/typedoc/classes/eventhandler.html) for how to subscribe to and be notified when a property changes on the [SciChartSurfaceBase](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurfacebase.html)

### redrawRequested

redrawRequested: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/eventhandler.html" class="tsd-signature-type">EventHandler</a>\<boolean\> = new EventHandler\<boolean\>()

An event handler which notifies its subscribers when a chart was requested to redraw initially.

### Readonly renderableSeries

renderableSeries: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/observablearray.html" class="tsd-signature-type">ObservableArray</a>\<<a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries3d.html" class="tsd-signature-type">IRenderableSeries3D</a>\>

summary  
Gets the collection of [IRenderableSeries3D](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries3d.html) - the chart types or seres on this [SciChart3DSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html)

description  
A [SciChart3DSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html) can have zero to many [RenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries3d.html).

The RenderableSeries are drawn as chart types, e.g. [3D Scatter series](https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype3d.html#scatterrenderableseries3d), [Surface Mesh series](https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype3d.html#surfacemeshrenderableseries3d). Each RenderableSeries must have a [DataSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/basedataseries3d.html).

Use this collection to add and remove series to the chart.

remarks  
Adding a series to the chart causes it to automatically redraw. To zoom to fit the data after adding a series, either set [AxisCore.autoRange](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#autorange) or call {@link SciChart3DSurface.zoomExtents}

### rendered

rendered: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/eventhandler.html" class="tsd-signature-type">EventHandler</a>\<boolean\> = new EventHandler\<boolean\>()

An event handler which notifies its subscribers when a render operation has finished.

### renderedToDestination

renderedToDestination: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/eventhandler.html" class="tsd-signature-type">EventHandler</a>\<any\> = new EventHandler\<any\>()

An event handler which notifies its subscribers when a chart was rendered to a display canvas.

remarks  
Not applicable to sub-charts

### renderedToWebGl

renderedToWebGl: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/eventhandler.html" class="tsd-signature-type">EventHandler</a>\<any\> = new EventHandler\<any\>()

An event handler which notifies its subscribers when a chart was rendered to WebgL Canvas.

remarks  
Not applicable to sub-charts

### rootEntity

rootEntity: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rootsceneentity.html" class="tsd-signature-type">RootSceneEntity</a>

summary  
The [RootSceneEntity](https://www.scichart.com/documentation/js/v4/typedoc/enums/esceneentitytype.html#rootsceneentity) is a special [BaseSceneEntity3D](https://www.scichart.com/documentation/js/v4/typedoc/classes/basesceneentity3d.html) type which is the root of the entire scene in in SciChart's High Performance <a href="https://www.scichart.com/javascript-chart-features" class="external">JavaScript 3D Charts</a>

remarks  
Add and remove entities to the scene using the property [SciChart3DSurface.rootEntity](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#rootentity) and calling [SceneEntity.children.add](https://www.scichart.com/documentation/js/v4/typedoc/classes/rootsceneentity.html#children).

When a [BaseRenderableSeries3D](https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries3d.html) is added to [SciChart3DSurface.renderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#renderableseries), it's entity is automatically added to the scene.

### showErrors

showErrors: boolean = true

Whether to show errors that occur during the render process. Defaults true.

### Readonly suspender

suspender: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/updatesuspender.html" class="tsd-signature-type">UpdateSuspender</a> = new UpdateSuspender()

### Protected themeProviderProperty

themeProviderProperty: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html" class="tsd-signature-type">IThemeProvider</a> = SciChartSurfaceBase.DEFAULT_THEME

### Protected touchActionProperty

touchActionProperty: string

### Protected visibilityObserver

visibilityObserver: VisibilityObserver

### watermarkPosition

watermarkPosition: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/ewatermarkposition.html" class="tsd-signature-type">EWatermarkPosition</a> = SciChartDefaults.watermarkPosition

The position of the watermark for trials and community licenses

### Readonly webAssemblyContext3D

webAssemblyContext3D: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart3d" class="tsd-signature-type">TSciChart3D</a>

The [SciChart 3D WebAssembly Context](https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart3d) containing native methods and access to our WebGL2 Engine and WebAssembly numerical methods

### Protected widthAspect

widthAspect: number

### Static AntiAliasWebGlBackbuffer

AntiAliasWebGlBackbuffer: boolean = false

Global property defining whether the WebGL render target is anti-aliased or not. This will affect all SciChartSurfaces (2D & 3D) in the application.

remarks  
Defaults to FALSE for crisp gridlines and lines. Individual line series and text labels are chart parts are automatically anti-aliased

### Static DEFAULT_THEME

DEFAULT_THEME: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html" class="tsd-signature-type">IThemeProvider</a> = new SciChartJSDarkv2Theme()

Gets or sets the application-wide default theme. See [IThemeProvider](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html) for details

### Static autoDisposeWasmContext

autoDisposeWasmContext: boolean = false

Defines if the shared wasmContext ([WebAssembly Context](https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart)) should be deleted after all of the surfaces that use it are deleted.

### Static domMasterCanvas

domMasterCanvas: HTMLCanvasElement

The master WebGL canvas

remarks  
WARNING: Do not set this property, this is set internally by SciChart when initializing mutliple charts

### Static invalidateOnTabVisible

invalidateOnTabVisible: boolean = true

Defines if charts should rerender when the tab becomes active.

remarks  
Enabled by default. Purpose of this behavior is to deal with the issue of canvas data being cleared on an inactive tab .

### Static wasmContextDisposeTimeout

wasmContextDisposeTimeout: number = 0

Defines a delay of the shared wasmContext auto-dispose if [autoDisposeWasmContext](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#autodisposewasmcontext) is enabled.

## Accessors

### background

- get background(): string
- set background(background: string): void

<!-- -->

- Gets or sets the SciChartSurface Background as an HTML color code

  remarks  
  Allowable values are colors e.g. 'Red', '#FF00FF', 'rgba(255,0,0,1)'. Also, main surface background supports gradients 'linear-gradient(45deg, rgb(255,0,0,1), rgb(0,0,255,1))', or background images e.g. 'url('<https://somewebsite.com/someimage.png')'>. Sub-charts, however, support only basic formats.

  #### Returns string

- Gets or sets the SciChartSurface Background as an HTML color code

  remarks  
  Allowable values are colors e.g. 'Red', '#FF00FF', 'rgba(255,0,0,1)'. Also, main surface background supports gradients 'linear-gradient(45deg, rgb(255,0,0,1), rgb(0,0,255,1))', or background images e.g. 'url('<https://somewebsite.com/someimage.png')'>. Sub-charts, however, support only basic formats.

  #### Parameters

  - ##### background: string

    The HTML color code

  #### Returns void

### camera

- get camera(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icameracontroller.html" class="tsd-signature-type">ICameraController</a>
- set camera(value: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icameracontroller.html" class="tsd-signature-type">ICameraController</a>): void

<!-- -->

- The [ICameraController](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icameracontroller.html) is a 3D Camera which allows choosing perspective, orthogonal projections, camera position, target, orientation such as Pitch, Yaw and Roll etc...

  remarks  
  See [CameraController](https://www.scichart.com/documentation/js/v4/typedoc/classes/cameracontroller.html) for a concrete implementation of [ICameraController](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icameracontroller.html)

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icameracontroller.html" class="tsd-signature-type">ICameraController</a>

- The [ICameraController](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icameracontroller.html) is a 3D Camera which allows choosing perspective, orthogonal projections, camera position, target, orientation such as Pitch, Yaw and Roll etc...

  remarks  
  See [CameraController](https://www.scichart.com/documentation/js/v4/typedoc/classes/cameracontroller.html) for a concrete implementation of [ICameraController](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icameracontroller.html)

  #### Parameters

  - ##### value: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icameracontroller.html" class="tsd-signature-type">ICameraController</a>

  #### Returns void

### chartModifierGroups

- get chartModifierGroups(): string\[\]

<!-- -->

- Gets Chart Modifier Groups, is used for sharing events between chart modifiers

  #### Returns string\[\]

### chartViewRect

- get chartViewRect(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>

<!-- -->

- #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>

### clipRect

- get clipRect(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>

<!-- -->

- #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>

### enableGizmo

- get enableGizmo(): boolean
- set enableGizmo(isEnabled: boolean): void

<!-- -->

- Gets or sets whether the Xyz gizmo is enabled - a small 3D Xyz axis on the bottom left of the 3D Chart

  #### Returns boolean

- Gets or sets whether the Xyz gizmo is enabled - a small 3D Xyz axis on the bottom left of the 3D Chart

  #### Parameters

  - ##### isEnabled: boolean

  #### Returns void

### freezeWhenOutOfView

- get freezeWhenOutOfView(): boolean
- set freezeWhenOutOfView(freezeWhenOutOfView: boolean): void

<!-- -->

- When true, charts that are out of the viewport will be frozen (pausing rendering). Data updates can resume Once the chart is in view again, rendering will resume. This can be useful for performance optimization.

  #### Returns boolean

- When true, charts that are out of the viewport will be frozen (pausing rendering). Data updates can resume Once the chart is in view again, rendering will resume. This can be useful for performance optimization.

  #### Parameters

  - ##### freezeWhenOutOfView: boolean

  #### Returns void

### id

- get id(): string
- set id(value: string): void

<!-- -->

- Gets or sets the SciChartSurface Id

  #### Returns string

- Gets or sets the SciChartSurface Id

  #### Parameters

  - ##### value: string

  #### Returns void

### isAxisCubeRendered

- get isAxisCubeRendered(): boolean

<!-- -->

- Called internally Gets isAxisCubeRenderedProperty flag

  #### Returns boolean

### isCopyCanvasSurface

- get isCopyCanvasSurface(): HTMLCanvasElement

<!-- -->

- #### Returns HTMLCanvasElement

### isDeleted

- get isDeleted(): boolean

<!-- -->

- Used internally - gets isDeleted flag

  #### Returns boolean

### isHitTestEnabled

- get isHitTestEnabled(): boolean
- set isHitTestEnabled(isEnabled: boolean): void

<!-- -->

- Required to enable Hit-Test if any of the following functions are needed in SciChart3DSurface:

  - [BaseRenderableSeries3D.hitTest](https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries3d.html#hittest)
  - {@link TooltipModiifer3D}

  Enabling hit-test adds minor a performance overhead for drawing and should be disabled if not required.

  #### Returns boolean

- Required to enable Hit-Test if any of the following functions are needed in SciChart3DSurface:

  - [BaseRenderableSeries3D.hitTest](https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries3d.html#hittest)
  - {@link TooltipModiifer3D}

  Enabling hit-test adds minor a performance overhead for drawing and should be disabled if not required.

  #### Parameters

  - ##### isEnabled: boolean

  #### Returns void

### isInitialized

- get isInitialized(): boolean

<!-- -->

- Used internally - gets isInitialized flag

  #### Returns boolean

### isInvalidated

- get isInvalidated(): boolean

<!-- -->

- #### Returns boolean

### isPolar

- get isPolar(): boolean

<!-- -->

- #### Returns boolean

### isRunningAnimation

- get isRunningAnimation(): boolean

<!-- -->

- Returns true if an animation is running

  #### Returns boolean

### isSuspended

- get isSuspended(): boolean

<!-- -->

- Gets a value indicating whether updates for the target are currently suspended

  inheritdoc  

  #### Returns boolean

### isXYPlaneVisible

- get isXYPlaneVisible(): boolean
- set isXYPlaneVisible(value: boolean): void

<!-- -->

- Gets / sets visibility of the XY axis plane

  #### Returns boolean

- Gets / sets visibility of the XY axis plane

  #### Parameters

  - ##### value: boolean

  #### Returns void

### isZXPlaneVisible

- get isZXPlaneVisible(): boolean
- set isZXPlaneVisible(value: boolean): void

<!-- -->

- Gets / sets visibility of the ZX axis plane

  #### Returns boolean

- Gets / sets visibility of the ZX axis plane

  #### Parameters

  - ##### value: boolean

  #### Returns void

### isZYPlaneVisible

- get isZYPlaneVisible(): boolean
- set isZYPlaneVisible(value: boolean): void

<!-- -->

- Gets / sets visibility of the ZY axis plane

  #### Returns boolean

- Gets / sets visibility of the ZY axis plane

  #### Parameters

  - ##### value: boolean

  #### Returns void

### otherSurfaces

- get otherSurfaces(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurfacebase.html" class="tsd-signature-type">SciChartSurfaceBase</a>\[\]

<!-- -->

- Used internally - gets other SciChartSurfaces

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurfacebase.html" class="tsd-signature-type">SciChartSurfaceBase</a>\[\]

### preRenderAll

- get preRenderAll(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/eventhandler.html" class="tsd-signature-type">EventHandler</a>\<void\>

<!-- -->

- An event handler which notifies its subscribers when a render operation starts. Use this to update elements of the chart for the current render. Any updates made here will not trigger a subsequent render.

  remarks  
  this is an alias for [preRender](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#prerender).

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/eventhandler.html" class="tsd-signature-type">EventHandler</a>\<void\>

### previousThemeProvider

- get previousThemeProvider(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html" class="tsd-signature-type">IThemeProvider</a>

<!-- -->

- Used internally - gets the previous [IThemeProvider](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html)

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html" class="tsd-signature-type">IThemeProvider</a>

### seriesViewRect

- get seriesViewRect(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>

<!-- -->

- Gets the Series View [Rect](https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html), a rectangle relative to the entire size of the [SciChartSurfaceBase](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurfacebase.html)

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>

### surfaceType

- get surfaceType(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/esurfacetype.html" class="tsd-signature-type">ESurfaceType</a>

<!-- -->

- inheritdoc  

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/esurfacetype.html" class="tsd-signature-type">ESurfaceType</a>

### themeProvider

- get themeProvider(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html" class="tsd-signature-type">IThemeProvider</a>

<!-- -->

- Used internally - gets the previous [IThemeProvider](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html)

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html" class="tsd-signature-type">IThemeProvider</a>

### viewRect

- get viewRect(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>

<!-- -->

- #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>

### viewportManager

- get viewportManager(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/viewportmanager3dbase.html" class="tsd-signature-type">ViewportManager3DBase</a>
- set viewportManager(viewportManager: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/viewportmanager3dbase.html" class="tsd-signature-type">ViewportManager3DBase</a>): void

<!-- -->

- Gets or sets the [Viewport Manager](https://www.scichart.com/documentation/js/v4/typedoc/classes/viewportmanager3dbase.html) - a class that allows managing of viewport axis ranges

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/viewportmanager3dbase.html" class="tsd-signature-type">ViewportManager3DBase</a>

- Gets or sets the [Viewport Manager](https://www.scichart.com/documentation/js/v4/typedoc/classes/viewportmanager3dbase.html) - a class that allows managing of viewport axis ranges

  #### Parameters

  - ##### viewportManager: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/viewportmanager3dbase.html" class="tsd-signature-type">ViewportManager3DBase</a>

  #### Returns void

### worldDimensions

- get worldDimensions(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/vector3.html" class="tsd-signature-type">Vector3</a>
- set worldDimensions(worldDimensions: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/vector3.html" class="tsd-signature-type">Vector3</a>): void

<!-- -->

- The WorldDimensions defines the size of the world in 3D space. Series and objects can exist outside of this world however the Axis cube will conform to this size.

  remarks  
  See our <a href="https://www.scichart.com/javascript-chart-documentation" class="external">Documentation</a> online to see how the World Dimensions property configures the size of the chart.

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/vector3.html" class="tsd-signature-type">Vector3</a>

- The WorldDimensions defines the size of the world in 3D space. Series and objects can exist outside of this world however the Axis cube will conform to this size.

  remarks  
  See our <a href="https://www.scichart.com/javascript-chart-documentation" class="external">Documentation</a> online to see how the World Dimensions property configures the size of the chart.

  #### Parameters

  - ##### worldDimensions: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/vector3.html" class="tsd-signature-type">Vector3</a>

  #### Returns void

### xAxis

- get xAxis(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase3d.html" class="tsd-signature-type">AxisBase3D</a>
- set xAxis(xAxis: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase3d.html" class="tsd-signature-type">AxisBase3D</a>): void

<!-- -->

- Gets or sets the XAxis in the 3D Chart.

  remarks  
  Axis types which derive from [AxisBase3D](https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase3d.html) or concrete type [NumericAxis3D](https://www.scichart.com/documentation/js/v4/typedoc/enums/eaxistype.html#numericaxis3d) are valid

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase3d.html" class="tsd-signature-type">AxisBase3D</a>

- Gets or sets the XAxis in the 3D Chart.

  remarks  
  Axis types which derive from [AxisBase3D](https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase3d.html) or concrete type [NumericAxis3D](https://www.scichart.com/documentation/js/v4/typedoc/enums/eaxistype.html#numericaxis3d) are valid

  #### Parameters

  - ##### xAxis: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase3d.html" class="tsd-signature-type">AxisBase3D</a>

  #### Returns void

### yAxis

- get yAxis(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase3d.html" class="tsd-signature-type">AxisBase3D</a>
- set yAxis(yAxis: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase3d.html" class="tsd-signature-type">AxisBase3D</a>): void

<!-- -->

- Gets or sets the YAxis in the 3D Chart.

  remarks  
  Axis types which derive from [AxisBase3D](https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase3d.html) or concrete type [NumericAxis3D](https://www.scichart.com/documentation/js/v4/typedoc/enums/eaxistype.html#numericaxis3d) are valid

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase3d.html" class="tsd-signature-type">AxisBase3D</a>

- Gets or sets the YAxis in the 3D Chart.

  remarks  
  Axis types which derive from [AxisBase3D](https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase3d.html) or concrete type [NumericAxis3D](https://www.scichart.com/documentation/js/v4/typedoc/enums/eaxistype.html#numericaxis3d) are valid

  #### Parameters

  - ##### yAxis: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase3d.html" class="tsd-signature-type">AxisBase3D</a>

  #### Returns void

### zAxis

- get zAxis(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase3d.html" class="tsd-signature-type">AxisBase3D</a>
- set zAxis(zAxis: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase3d.html" class="tsd-signature-type">AxisBase3D</a>): void

<!-- -->

- Gets or sets the ZAxis in the 3D Chart.

  remarks  
  Axis types which derive from [AxisBase3D](https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase3d.html) or concrete type [NumericAxis3D](https://www.scichart.com/documentation/js/v4/typedoc/enums/eaxistype.html#numericaxis3d) are valid

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase3d.html" class="tsd-signature-type">AxisBase3D</a>

- Gets or sets the ZAxis in the 3D Chart.

  remarks  
  Axis types which derive from [AxisBase3D](https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase3d.html) or concrete type [NumericAxis3D](https://www.scichart.com/documentation/js/v4/typedoc/enums/eaxistype.html#numericaxis3d) are valid

  #### Parameters

  - ##### zAxis: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase3d.html" class="tsd-signature-type">AxisBase3D</a>

  #### Returns void

## Methods

### addAnimation

- addAnimation(...animations: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/igenericanimation.html" class="tsd-signature-type">IGenericAnimation</a>\[\]): void

<!-- -->

- Add a [GenericAnimation](https://www.scichart.com/documentation/js/v4/typedoc/classes/genericanimation.html) to the surface. Multiple animations will be run in parallel, so if you want to run one after another, use the onCompleted callback to add another animation after the first completes

  #### Parameters

  - ##### Rest ...animations: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/igenericanimation.html" class="tsd-signature-type">IGenericAnimation</a>\[\]

  #### Returns void

### addDeletable

- addDeletable(deletable: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html" class="tsd-signature-type">IDeletable</a>): void

<!-- -->

- #### Parameters

  - ##### deletable: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html" class="tsd-signature-type">IDeletable</a>

  #### Returns void

### Protected applyOptions

- applyOptions(options: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/i3dsurfaceoptions.html" class="tsd-signature-type">I3DSurfaceOptions</a>): void

<!-- -->

- #### Parameters

  - ##### options: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/i3dsurfaceoptions.html" class="tsd-signature-type">I3DSurfaceOptions</a>

  #### Returns void

### Protected applySciChartBackground

- applySciChartBackground(background: string): void

<!-- -->

- inheritdoc  

  #### Parameters

  - ##### background: string

  #### Returns void

### applyTheme

- applyTheme(themeProvider: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html" class="tsd-signature-type">IThemeProvider</a>): void

<!-- -->

- Applies a theme (defined by IThemeProvider) to the current element

  #### Parameters

  - ##### themeProvider: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html" class="tsd-signature-type">IThemeProvider</a>

    The theme data to apply

  #### Returns void

### Protected attachChartModifier

- attachChartModifier(chartModifier: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ichartmodifierbase.html" class="tsd-signature-type">IChartModifierBase</a>): void

<!-- -->

- inheritdoc  

  #### Parameters

  - ##### chartModifier: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ichartmodifierbase.html" class="tsd-signature-type">IChartModifierBase</a>

  #### Returns void

### Protected changeMasterCanvasViewportSize

- changeMasterCanvasViewportSize(wasmContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart" class="tsd-signature-type">TSciChart</a> \| <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart3d" class="tsd-signature-type">TSciChart3D</a>, pixelWidth: number, pixelHeight: number): void

<!-- -->

- #### Parameters

  - ##### wasmContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart" class="tsd-signature-type">TSciChart</a> \| <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart3d" class="tsd-signature-type">TSciChart3D</a>

  - ##### pixelWidth: number

  - ##### pixelHeight: number

  #### Returns void

### changeViewportSize

- changeViewportSize(pixelWidth: number, pixelHeight: number): void

<!-- -->

- Changes the Viewport Size of the [SciChartSurfaceBase](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurfacebase.html)

  #### Parameters

  - ##### pixelWidth: number

  - ##### pixelHeight: number

  #### Returns void

### Protected changeWebGLCanvasViewportSize

- changeWebGLCanvasViewportSize(wasmContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart" class="tsd-signature-type">TSciChart</a> \| <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart3d" class="tsd-signature-type">TSciChart3D</a>, pixelWidth: number, pixelHeight: number): void

<!-- -->

- #### Parameters

  - ##### wasmContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart" class="tsd-signature-type">TSciChart</a> \| <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart3d" class="tsd-signature-type">TSciChart3D</a>

  - ##### pixelWidth: number

  - ##### pixelHeight: number

  #### Returns void

### Protected clearRootElement

- clearRootElement(clearHtml: boolean): void

<!-- -->

- #### Parameters

  - ##### clearHtml: boolean

  #### Returns void

### delete

- delete(clearHtml?: boolean): void

<!-- -->

- Deletes native (WebAssembly) memory used by this type, after which it cannot be used.

  #### Parameters

  - ##### Default value clearHtml: boolean = true

  #### Returns void

### Protected detachChartModifier

- detachChartModifier(chartModifier: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ichartmodifierbase.html" class="tsd-signature-type">IChartModifierBase</a>): void

<!-- -->

- Detaches a [ChartModifierBase2D](https://www.scichart.com/documentation/js/v4/typedoc/classes/chartmodifierbase2d.html) from the [SciChartSurfaceBase](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurfacebase.html)

  #### Parameters

  - ##### chartModifier: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ichartmodifierbase.html" class="tsd-signature-type">IChartModifierBase</a>

  #### Returns void

### doDrawingLoop

- doDrawingLoop(): void

<!-- -->

- Called internally - the main drawing loop

  #### Returns void

### getAnimations

- getAnimations(): ReadonlyArray\<<a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/igenericanimation.html" class="tsd-signature-type">IGenericAnimation</a>\>

<!-- -->

- Gets the generic animations currently on the surface. Do not manipulate this array directly. To add, use addAnimation. To remove, find an animation and call .cancel() on it.

  #### Returns ReadonlyArray\<<a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/igenericanimation.html" class="tsd-signature-type">IGenericAnimation</a>\>

### getDefaultXAxis

- getDefaultXAxis(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase3d.html" class="tsd-signature-type">AxisBase3D</a> \| undefined

<!-- -->

- inheritdoc  

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase3d.html" class="tsd-signature-type">AxisBase3D</a> \| undefined

### getDefaultYAxis

- getDefaultYAxis(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase3d.html" class="tsd-signature-type">AxisBase3D</a> \| undefined

<!-- -->

- inheritdoc  

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase3d.html" class="tsd-signature-type">AxisBase3D</a> \| undefined

### getMainCanvas

- getMainCanvas(): HTMLCanvasElement

<!-- -->

- Gets the main WebGL or 2D canvas

  #### Returns HTMLCanvasElement

### getNextState

- getNextState(excludeData?: boolean): Promise\<{ type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/escichartsurfacetype.html#default3d" class="tsd-signature-type">Default3D</a> } & <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iscichart3ddefinition.html" class="tsd-signature-type">ISciChart3DDefinition</a>\>

<!-- -->

- Triggers the rerendering of the surface and after the chart rerendering is completed, returns its serialized state retrieved with [SciChartSurface.toJSON](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#tojson).

  #### Parameters

  - ##### Default value excludeData: boolean = false

    if set true, data values will not be included in the json.

  #### Returns Promise\<{ type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/escichartsurfacetype.html#default3d" class="tsd-signature-type">Default3D</a> } & <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iscichart3ddefinition.html" class="tsd-signature-type">ISciChart3DDefinition</a>\>

  JSON-like object [ISciChart3DDefinition](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iscichart3ddefinition.html)

### Protected getOptions

- getOptions(): {}

<!-- -->

- #### Returns {}

### getSceneWorld

- getSceneWorld(): SCRTSceneWorld

<!-- -->

- Used internally: Gets the {@link SCRTSceneWorld} object at the root of the 3d scene graph

  #### Returns SCRTSceneWorld

### getSeriesViewRectPadding

- getSeriesViewRectPadding(scaled: boolean): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/thickness.html" class="tsd-signature-type">Thickness</a>

<!-- -->

- #### Parameters

  - ##### scaled: boolean

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/thickness.html" class="tsd-signature-type">Thickness</a>

### getXAxisById

- getXAxisById(axisId: string): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase3d.html" class="tsd-signature-type">AxisBase3D</a> \| undefined

<!-- -->

- inheritdoc  

  #### Parameters

  - ##### axisId: string

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase3d.html" class="tsd-signature-type">AxisBase3D</a> \| undefined

### getYAxisById

- getYAxisById(axisId: string): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase3d.html" class="tsd-signature-type">AxisBase3D</a> \| undefined

<!-- -->

- inheritdoc  

  #### Parameters

  - ##### axisId: string

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase3d.html" class="tsd-signature-type">AxisBase3D</a> \| undefined

### invalidateElement

- invalidateElement(options?: { force?: boolean }): void

<!-- -->

- inheritdoc  

  #### Parameters

  - ##### Optional options: { force?: boolean }

    - ##### Optional force?: boolean

  #### Returns void

### nextStateRender

- nextStateRender(options?: { invalidateOnResume?: boolean; resumeBefore?: boolean; suspendAfter?: boolean }): Promise\<void\>

<!-- -->

- Creates a promise which resolves when the chart is updated to the next fully rendered state

  remarks  
  If the surface is initialized with `createSingle` the promise resolves after the main `render` function is executed. Otherwise, if it is initialized with `create` - the promise resolves after image data is copied to the 2D canvas.

  #### Parameters

  - ##### Optional options: { invalidateOnResume?: boolean; resumeBefore?: boolean; suspendAfter?: boolean }

    - ##### Optional invalidateOnResume?: boolean

    - ##### Optional resumeBefore?: boolean

    - ##### Optional suspendAfter?: boolean

  #### Returns Promise\<void\>

### Protected notifyPropertyChanged

- notifyPropertyChanged(propertyName: string): void

<!-- -->

- summary  
  Notifies subscribers of [SciChartSurfaceBase.propertyChanged](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurfacebase.html#propertychanged) that a property has changed and the chart requires redrawing

  description  
  SciChart provides fully reactive components, changing any property or changing data will cause the [SciChartSurfaceBase](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurfacebase.html) to redraw where necessary. This method notifies subscribers of the [SciChartSurfaceBase.propertyChanged](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurfacebase.html#propertychanged) [EventHandler](https://www.scichart.com/documentation/js/v4/typedoc/classes/eventhandler.html) that a property has changed.

  #### Parameters

  - ##### propertyName: string

    The name of the property which has changed

  #### Returns void

### onAnimate

- onAnimate(timeElapsed: number): void

<!-- -->

- Is being called on each render, to run animations

  #### Parameters

  - ##### timeElapsed: number

  #### Returns void

### onDpiChanged

- onDpiChanged(args: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tdpichangedeventargs" class="tsd-signature-type">TDpiChangedEventArgs</a>): void

<!-- -->

- Called when the Dpi changes in the browser. This could be due to user zooming the browser, or changing DPI settings in Windows, or moving the browser containing SciChart to another monitor

  #### Parameters

  - ##### args: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tdpichangedeventargs" class="tsd-signature-type">TDpiChangedEventArgs</a>

    The [TDpiChangedEventArgs](https://www.scichart.com/documentation/js/v4/typedoc/index.html#tdpichangedeventargs) containing info about the Dpi Changed event

  #### Returns void

### onSciChartRendered

- onSciChartRendered(): void

<!-- -->

- Called after the [SciChart3DSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html) has rendered.

  #### Returns void

### registerFont

- registerFont(fontName: string, url: string): Promise\<boolean\>

<!-- -->

- #### Parameters

  - ##### fontName: string

    Register a font to be used with native text.

  - ##### url: string

  #### Returns Promise\<boolean\>

### resumeUpdates

- resumeUpdates(options?: { force?: boolean; invalidateOnResume?: boolean }): void

<!-- -->

- inheritdoc  

  #### Parameters

  - ##### Optional options: { force?: boolean; invalidateOnResume?: boolean }

    - ##### Optional force?: boolean

    - ##### Optional invalidateOnResume?: boolean

  #### Returns void

### setDestinations

- setDestinations(destinations: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichartdestination" class="tsd-signature-type">TSciChartDestination</a>\[\]): void

<!-- -->

- Used internally - sets destinations

  #### Parameters

  - ##### destinations: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichartdestination" class="tsd-signature-type">TSciChartDestination</a>\[\]

  #### Returns void

### setIsAxisCubeRendered

- setIsAxisCubeRendered(): void

<!-- -->

- Called internally Sets isAxisCubeRenderedProperty flag after Axis Cube is rendered

  #### Returns void

### setIsInitialized

- setIsInitialized(): void

<!-- -->

- Used internally, the flag is set after [SciChartSurfaceBase](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurfacebase.html) is initialized

  #### Returns void

### setSeriesViewRect

- setSeriesViewRect(seriesViewRect: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>): void

<!-- -->

- Sets the Series View [Rect](https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html), a rectangle relative to the entire size of the [SciChartSurfaceBase](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurfacebase.html)

  #### Parameters

  - ##### seriesViewRect: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>

    a [Rect](https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html) which defines the portion of the view for drawing series

  #### Returns void

### setSvgClipPathDefinitions

- setSvgClipPathDefinitions(svgElement: SVGSVGElement, seriesViewRect: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>, surfaceViewRect: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>): void

<!-- -->

- Adds or updates clipPath definitions on an SVG canvas

  #### Parameters

  - ##### svgElement: SVGSVGElement

  - ##### seriesViewRect: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>

  - ##### surfaceViewRect: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>

  #### Returns void

### suspendUpdates

- suspendUpdates(): void

<!-- -->

- Suspends updates on the target

  #### Returns void

### toJSON

- toJSON(excludeData?: boolean): { type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/escichartsurfacetype.html#default3d" class="tsd-signature-type">Default3D</a> } & <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iscichart3ddefinition.html" class="tsd-signature-type">ISciChart3DDefinition</a>

<!-- -->

- Convert the object to a definition that can be serialized to JSON, or used directly with the builder api

  #### Parameters

  - ##### Default value excludeData: boolean = false

    if set true, data values will not be included in the json.

  #### Returns { type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/escichartsurfacetype.html#default3d" class="tsd-signature-type">Default3D</a> } & <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iscichart3ddefinition.html" class="tsd-signature-type">ISciChart3DDefinition</a>

### updateBackground

- updateBackground(): void

<!-- -->

- #### Returns void

### updateWatermark

- updateWatermark(left: number, bottom: number): void

<!-- -->

- inheritdoc  

  #### Parameters

  - ##### left: number

  - ##### bottom: number

  #### Returns void

### worldToScreenCoord

- worldToScreenCoord(worldCoordXyz: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/vector3.html" class="tsd-signature-type">Vector3</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/point.html" class="tsd-signature-type">Point</a>

<!-- -->

- Converts a 3D Xyz coordinate in world coordinates space to a screen coordinate (2d) in pixels. This allows you to get the 2D screen coordinate of any object or vertex in the 3D scene.

  remarks  
  Note: Conversions to/from world/data space must be performed using the {@link AxisBase3D.getCurrentCoordinateCalculator()} API, which returns [CoordinateCalculatorBase](https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html). Functions [CoordinateCalculatorBase.getDataValue](https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html#getdatavalue) and [CoordinateCalculatorBase.getCoordinate](https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html#getcoordinate) convert to/from world coords/data space

  #### Parameters

  - ##### worldCoordXyz: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/vector3.html" class="tsd-signature-type">Vector3</a>

    The 3D Xyz coordinate

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/point.html" class="tsd-signature-type">Point</a>

  The 2D screen coordinate in pixels

### Static UseCommunityLicense

- UseCommunityLicense(): void

<!-- -->

- Causes SciChart to always use its built in community non-commercial license. This stops it attempting to look for the license wizard Usage of the community license constitutes acceptance of the terms at <https://www.scichart.com/community-licensing/>

  #### Returns void

### Static configure

- configure(config: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichartconfig" class="tsd-signature-type">TSciChartConfig</a>): void

<!-- -->

- Allows setting of web URL for Wasm and Data files, in the case you are loading SciChart outside of npm/webpack environment. Note if loading from CDN the version number of data/wasm Urls must match the version number of SciChart.js you are using.

  example  
  ``` ts
  // 3D Charts
  SciChart.SciChart3DSurface.configure({
   dataUrl: "https://cdn.jsdelivr.net/npm/scichart@2.2.2378/_wasm/scichart3d.data",
   wasmUrl: "https://cdn.jsdelivr.net/npm/scichart@2.2.2378/_wasm/scichart3d.wasm"
  });
  ```

  #### Parameters

  - ##### config: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichartconfig" class="tsd-signature-type">TSciChartConfig</a>

  #### Returns void

### Static create

- create(divElementId: string \| HTMLDivElement, options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/i3dsurfaceoptions.html" class="tsd-signature-type">I3DSurfaceOptions</a>): Promise\<<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#twebassemblychart3d" class="tsd-signature-type">TWebAssemblyChart3D</a>\>

<!-- -->

- Creates a [SciChart3DSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html) and [WebAssembly Context](https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart3d) to occupy the div by element ID in your DOM.

  remarks  
  This method is async and must be awaited

  #### Parameters

  - ##### divElementId: string \| HTMLDivElement

    The ID or reference of Div Element where the [SciChart3DSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html) will reside

  - ##### Optional options: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/i3dsurfaceoptions.html" class="tsd-signature-type">I3DSurfaceOptions</a>

    Optional parameters of type [I3DSurfaceOptions](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/i3dsurfaceoptions.html)

  #### Returns Promise\<<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#twebassemblychart3d" class="tsd-signature-type">TWebAssemblyChart3D</a>\>

### Static createSingle

- createSingle(divElement: string \| HTMLDivElement, options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/i3dsurfaceoptions.html" class="tsd-signature-type">I3DSurfaceOptions</a>): Promise\<<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#twebassemblychart3d" class="tsd-signature-type">TWebAssemblyChart3D</a>\>

<!-- -->

- USED INTERNALLY - performs a similar operation to [SciChart3DSurface.create](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#create) but used internally for testing

  #### Parameters

  - ##### divElement: string \| HTMLDivElement

    The Div Element ID or reference where the [SciChartSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html) will reside

  - ##### Optional options: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/i3dsurfaceoptions.html" class="tsd-signature-type">I3DSurfaceOptions</a>

    Optional parameters of type [I3DSurfaceOptions](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/i3dsurfaceoptions.html)

  #### Returns Promise\<<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#twebassemblychart3d" class="tsd-signature-type">TWebAssemblyChart3D</a>\>

### Static disposeSharedWasmContext

- disposeSharedWasmContext(): void

<!-- -->

- #### Returns void

### Static loadWasmFromCDN

- loadWasmFromCDN(): void

<!-- -->

- Tell SciChart to load the Wasm and Data files from CDN, rather than expecting them to be served by the host server.

  #### Returns void

### Static loadWasmLocal

- loadWasmLocal(): void

<!-- -->

- Tell SciChart to load the Wasm and Data files from the local server, rather than from CDN.

  #### Returns void

### Static resolveOptions

- resolveOptions(options: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/isurfaceoptionsbase.html" class="tsd-signature-type">ISurfaceOptionsBase</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/isurfaceoptionsbase.html" class="tsd-signature-type">ISurfaceOptionsBase</a>

<!-- -->

- #### Parameters

  - ##### options: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/isurfaceoptionsbase.html" class="tsd-signature-type">ISurfaceOptionsBase</a>

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/isurfaceoptionsbase.html" class="tsd-signature-type">ISurfaceOptionsBase</a>

### Static setLicenseCallback

- setLicenseCallback(callback: (queryString: string) =\> Promise\<Response\>): void

<!-- -->

- Sets function that will be called by the framework to validate a runtime license from the server, if you need to add additional handling, such as custom authentication. The request sent to the server must include the queryString section passed in, which does not come with a leading ?

  #### Parameters

  - ##### callback: (queryString: string) =\> Promise\<Response\>

    - - (queryString: string): Promise\<Response\>

      <!-- -->

      - #### Parameters

        - ##### queryString: string

        #### Returns Promise\<Response\>

  #### Returns void

### Static setRuntimeLicenseKey

- setRuntimeLicenseKey(keyCode: string): void

<!-- -->

- Sets the runtime license key. Use for full licenses or trials only, not developer licenses.

  #### Parameters

  - ##### keyCode: string

  #### Returns void

### Static setServerLicenseEndpoint

- setServerLicenseEndpoint(endpoint: string): void

<!-- -->

- Sets the endpoint for validating a runtime license key with the server. Must be a relative path.

  default  
  api/license

  #### Parameters

  - ##### endpoint: string

  #### Returns void

### Static useWasmFromCDN

- useWasmFromCDN(): void

<!-- -->

- Tell SciChart to load the Wasm and Data files from CDN, rather than expecting them to be served by the host server.

  deprecated  
  the method name breaks [eslint react-hooks/rules-of-hooks](https://legacy.reactjs.org/docs/hooks-rules.html). To avoid this error in React, use [loadWasmFromCDN](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#loadwasmfromcdn) instead.

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
