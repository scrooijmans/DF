<img src="out_scichartv4/typedoc/classes/verticallineannotation_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [VerticalLineAnnotation](https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html)

# Class VerticalLineAnnotation

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

summary  
The [VerticalLineAnnotation](https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html) provides an [Annotation](https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationbase.html) which draws a vertical line at specific x1 (or y1 for Vertical Chart) over the [SciChartSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html)

description  
To add a [VerticalLineAnnotation](https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html) to a [SciChartSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html), use the following code:

``` ts
const sciChartSurface: SciChartSurface;
const verticalLineAnnotation = new VerticalLineAnnotation( { x1: 1, y1: 3 fill: "#FF000077", stroke: "#FF0000"});
sciChartSurface.annotations.add(verticalLineAnnotation);
```

remarks  
Uses the fast WebGL/WebAssembly {@link WebGL2RenderingContext} for rendering

### Hierarchy

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/lineannotation.html" class="tsd-signature-type">LineAnnotation</a>
  - VerticalLineAnnotation

### Implements

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html" class="tsd-signature-type">IDeletable</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iannotation.html" class="tsd-signature-type">IAnnotation</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iadornerprovider.html" class="tsd-signature-type">IAdornerProvider</a>

## Index

### Constructors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#constructor" class="tsd-kind-icon">constructor</a>

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#adornerclippingproperty" class="tsd-kind-icon">adornerClippingProperty</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#adornerdraggingpointproperty" class="tsd-kind-icon">adornerDraggingPointProperty</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#annotationsgripsfillproperty" class="tsd-kind-icon">annotationsGripsFillProperty</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#annotationsgripsradiusproperty" class="tsd-kind-icon">annotationsGripsRadiusProperty</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#annotationsgripsstrokeproperty" class="tsd-kind-icon">annotationsGripsStrokeProperty</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#axisfontfamilyproperty" class="tsd-kind-icon">axisFontFamilyProperty</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#axisfontsizeproperty" class="tsd-kind-icon">axisFontSizeProperty</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#clicked" class="tsd-kind-icon">clicked</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#clippingproperty" class="tsd-kind-icon">clippingProperty</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#dragdelta" class="tsd-kind-icon">dragDelta</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#dragended" class="tsd-kind-icon">dragEnded</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#dragonlabel" class="tsd-kind-icon">dragOnLabel</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#dragonline" class="tsd-kind-icon">dragOnLine</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#dragpointsproperty" class="tsd-kind-icon">dragPointsProperty</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#dragstarted" class="tsd-kind-icon">dragStarted</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#hovered" class="tsd-kind-icon">hovered</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#id" class="tsd-kind-icon">id</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#invalidateparentcallback" class="tsd-kind-icon">invalidateParentCallback</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#invalidatestate" class="tsd-kind-icon">invalidateState</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#isdomannotation" class="tsd-kind-icon">isDomAnnotation</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#ishiddenproperty" class="tsd-kind-icon">isHiddenProperty</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#issvgannotation" class="tsd-kind-icon">isSvgAnnotation</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#parentsurfaceproperty" class="tsd-kind-icon">parentSurfaceProperty</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#previsselected" class="tsd-kind-icon">prevIsSelected</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#prevvalue" class="tsd-kind-icon">prevValue</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#renderlayerproperty" class="tsd-kind-icon">renderLayerProperty</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#rendernexttoproperty" class="tsd-kind-icon">renderNextToProperty</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#renderorderproperty" class="tsd-kind-icon">renderOrderProperty</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#selectedchanged" class="tsd-kind-icon">selectedChanged</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#selectionboxdeltaproperty" class="tsd-kind-icon">selectionBoxDeltaProperty</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#selectionboxstrokeproperty" class="tsd-kind-icon">selectionBoxStrokeProperty</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#selectionboxthicknessproperty" class="tsd-kind-icon">selectionBoxThicknessProperty</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#strokepencache" class="tsd-kind-icon">strokePenCache</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#surfacerenderorderproperty" class="tsd-kind-icon">surfaceRenderOrderProperty</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#surfacetypes" class="tsd-kind-icon">surfaceTypes</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#svgadorner" class="tsd-kind-icon">svgAdorner</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#type" class="tsd-kind-icon">type</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#typemap" class="tsd-kind-icon">typeMap</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#verticalalignment" class="tsd-kind-icon">verticalAlignment</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#x1property" class="tsd-kind-icon">x1Property</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#xcoordinatemodeproperty" class="tsd-kind-icon">xCoordinateModeProperty</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#y1property" class="tsd-kind-icon">y1Property</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#ycoordinatemodeproperty" class="tsd-kind-icon">yCoordinateModeProperty</a>

### Accessors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#adornerclipping" class="tsd-kind-icon">adornerClipping</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#adornerdraggingpoint" class="tsd-kind-icon">adornerDraggingPoint</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#annotationlayer" class="tsd-kind-icon">annotationLayer</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#annotationsgripsfill" class="tsd-kind-icon">annotationsGripsFill</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#annotationsgripsradius" class="tsd-kind-icon">annotationsGripsRadius</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#annotationsgripsstroke" class="tsd-kind-icon">annotationsGripsStroke</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#axisfontfamily" class="tsd-kind-icon">axisFontFamily</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#axisfontsize" class="tsd-kind-icon">axisFontSize</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#axislabelfill" class="tsd-kind-icon">axisLabelFill</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#axislabelstroke" class="tsd-kind-icon">axisLabelStroke</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#clipping" class="tsd-kind-icon">clipping</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#dragpoints" class="tsd-kind-icon">dragPoints</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#isdraggingstarted" class="tsd-kind-icon">isDraggingStarted</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#iseditable" class="tsd-kind-icon">isEditable</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#ishidden-1" class="tsd-kind-icon">isHidden</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#ishovered" class="tsd-kind-icon">isHovered</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#isselected" class="tsd-kind-icon">isSelected</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#isverticalchart" class="tsd-kind-icon">isVerticalChart</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#isvisible" class="tsd-kind-icon">isVisible</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#labelplacement" class="tsd-kind-icon">labelPlacement</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#labelvalue" class="tsd-kind-icon">labelValue</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#opacity" class="tsd-kind-icon">opacity</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#parentsurface" class="tsd-kind-icon">parentSurface</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#resizedirections" class="tsd-kind-icon">resizeDirections</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#selectionboxdelta" class="tsd-kind-icon">selectionBoxDelta</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#selectionboxstroke" class="tsd-kind-icon">selectionBoxStroke</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#selectionboxthickness" class="tsd-kind-icon">selectionBoxThickness</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#showlabel" class="tsd-kind-icon">showLabel</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#stroke" class="tsd-kind-icon">stroke</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#strokedasharray" class="tsd-kind-icon">strokeDashArray</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#strokethickness" class="tsd-kind-icon">strokeThickness</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#svgadornerroot" class="tsd-kind-icon">svgAdornerRoot</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#x1-1" class="tsd-kind-icon">x1</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#x2" class="tsd-kind-icon">x2</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#xaxis" class="tsd-kind-icon">xAxis</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#xaxisid" class="tsd-kind-icon">xAxisId</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#xcoordinatemode" class="tsd-kind-icon">xCoordinateMode</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#y1-1" class="tsd-kind-icon">y1</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#y2" class="tsd-kind-icon">y2</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#yaxis" class="tsd-kind-icon">yAxis</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#yaxisid" class="tsd-kind-icon">yAxisId</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#ycoordinatemode" class="tsd-kind-icon">yCoordinateMode</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#applypixelratiotodragdist" class="tsd-kind-icon">applyPixelRatioToDragDist</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#applysvgclipping" class="tsd-kind-icon">applySvgClipping</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#calcdragdistance" class="tsd-kind-icon">calcDragDistance</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#candragpoint" class="tsd-kind-icon">canDragPoint</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#checkisclickedonannotation" class="tsd-kind-icon">checkIsClickedOnAnnotation</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#checkisclickedonannotationinternal" class="tsd-kind-icon">checkIsClickedOnAnnotationInternal</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#checkispointwithincliparea" class="tsd-kind-icon">checkIsPointWithinClipArea</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#checkiswithinbounds" class="tsd-kind-icon">checkIsWithinBounds</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#click" class="tsd-kind-icon">click</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#clicktoselect" class="tsd-kind-icon">clickToSelect</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#convertcartesiantopolar" class="tsd-kind-icon">convertCartesianToPolar</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#convertfromcoordinate" class="tsd-kind-icon">convertFromCoordinate</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#convertpolartocartesian" class="tsd-kind-icon">convertPolarToCartesian</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#delete" class="tsd-kind-icon">delete</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#deleteadorner" class="tsd-kind-icon">deleteAdorner</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#drawwithcontext" class="tsd-kind-icon">drawWithContext</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#getabsolutecoordinates" class="tsd-kind-icon">getAbsoluteCoordinates</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#getabsolutehorizontalcoordinate" class="tsd-kind-icon">getAbsoluteHorizontalCoordinate</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#getabsoluteverticalcoordinate" class="tsd-kind-icon">getAbsoluteVerticalCoordinate</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#getadornerannotationborders" class="tsd-kind-icon">getAdornerAnnotationBorders</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#getannotationborders" class="tsd-kind-icon">getAnnotationBorders</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#getannotationgripsvg" class="tsd-kind-icon">getAnnotationGripSvg</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#getclippath" class="tsd-kind-icon">getClipPath</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#getclippingrect" class="tsd-kind-icon">getClippingRect</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#getcoordinate" class="tsd-kind-icon">getCoordinate</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#getrenderlayer" class="tsd-kind-icon">getRenderLayer</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#getrendernextto" class="tsd-kind-icon">getRenderNextTo</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#getrenderorder" class="tsd-kind-icon">getRenderOrder</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#getresolvedcoordinate" class="tsd-kind-icon">getResolvedCoordinate</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#getsurfacerenderorder" class="tsd-kind-icon">getSurfaceRenderOrder</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#getvalue" class="tsd-kind-icon">getValue</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#getvaluesfromcoordinates" class="tsd-kind-icon">getValuesFromCoordinates</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#getx1coordinate" class="tsd-kind-icon">getX1Coordinate</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#getx2coordinate" class="tsd-kind-icon">getX2Coordinate</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#getxycoordinatesfromvalues" class="tsd-kind-icon">getXYCoordinatesFromValues</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#gety1coordinate" class="tsd-kind-icon">getY1Coordinate</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#gety2coordinate" class="tsd-kind-icon">getY2Coordinate</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#hover" class="tsd-kind-icon">hover</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#linkaxes" class="tsd-kind-icon">linkAxes</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#notifypropertychanged" class="tsd-kind-icon">notifyPropertyChanged</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#onattach" class="tsd-kind-icon">onAttach</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#ondetach" class="tsd-kind-icon">onDetach</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#ondpichanged" class="tsd-kind-icon">onDpiChanged</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#ondragadorner" class="tsd-kind-icon">onDragAdorner</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#ondragended" class="tsd-kind-icon">onDragEnded</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#ondragstarted" class="tsd-kind-icon">onDragStarted</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#resumeinvalidate" class="tsd-kind-icon">resumeInvalidate</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#setannotationborders" class="tsd-kind-icon">setAnnotationBorders</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#setrenderlayer" class="tsd-kind-icon">setRenderLayer</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#setrendernextto" class="tsd-kind-icon">setRenderNextTo</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#setrenderorder" class="tsd-kind-icon">setRenderOrder</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#setsurfacerenderorder" class="tsd-kind-icon">setSurfaceRenderOrder</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#suspendinvalidate" class="tsd-kind-icon">suspendInvalidate</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#svgstringadornertemplate" class="tsd-kind-icon">svgStringAdornerTemplate</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#tojson" class="tsd-kind-icon">toJSON</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#updateadornerinner" class="tsd-kind-icon">updateAdornerInner</a>

### Object literals

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#annotationborders" class="tsd-kind-icon">annotationBorders</a>

## Constructors

### constructor

- new VerticalLineAnnotation(options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ihvlineannotationoptions.html" class="tsd-signature-type">IHVLineAnnotationOptions</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html" class="tsd-signature-type">VerticalLineAnnotation</a>

<!-- -->

- Create an instance of a LineAnnotation

  #### Parameters

  - ##### Optional options: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ihvlineannotationoptions.html" class="tsd-signature-type">IHVLineAnnotationOptions</a>

    Optional parameters of type [ILineAnnotationOptions](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ilineannotationoptions.html) which configure the annotation upon construction

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html" class="tsd-signature-type">VerticalLineAnnotation</a>

## Properties

### Protected adornerClippingProperty

adornerClippingProperty: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eannotationclippingmode.html" class="tsd-signature-type">EAnnotationClippingMode</a> \| string = EAnnotationClippingMode.Chart

### Protected adornerDraggingPointProperty

adornerDraggingPointProperty: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/edragginggrippoint.html" class="tsd-signature-type">EDraggingGripPoint</a>

### Protected annotationsGripsFillProperty

annotationsGripsFillProperty: string = SciChartSurfaceBase.DEFAULT_THEME.annotationsGripsBackgroundBrush

### Protected annotationsGripsRadiusProperty

annotationsGripsRadiusProperty: number = ADORNER_GRIP_RADIUS

### Protected annotationsGripsStrokeProperty

annotationsGripsStrokeProperty: string = SciChartSurfaceBase.DEFAULT_THEME.annotationsGripsBorderBrush

### Protected Optional axisFontFamilyProperty

axisFontFamilyProperty: string = DEFAULT_FONT_FAMILY

### Protected Optional axisFontSizeProperty

axisFontSizeProperty: number = 14

### clicked

clicked: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/eventhandler.html" class="tsd-signature-type">EventHandler</a>\<<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationclickeventargs.html" class="tsd-signature-type">AnnotationClickEventArgs</a>\>

### Protected clippingProperty

clippingProperty: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eannotationclippingmode.html" class="tsd-signature-type">EAnnotationClippingMode</a> \| string = EAnnotationClippingMode.SeriesViewRect

### dragDelta

dragDelta: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/eventhandler.html" class="tsd-signature-type">EventHandler</a>\<<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationdragdeltaeventargs.html" class="tsd-signature-type">AnnotationDragDeltaEventArgs</a>\>

### dragEnded

dragEnded: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/eventhandler.html" class="tsd-signature-type">EventHandler</a>\<void\>

### dragOnLabel

dragOnLabel: boolean = true

### dragOnLine

dragOnLine: boolean = true

### Protected dragPointsProperty

dragPointsProperty: keyof <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/edragginggrippoint.html" class="tsd-signature-type">EDraggingGripPoint</a>\[\] = \[EDraggingGripPoint.Body,EDraggingGripPoint.x1y1,EDraggingGripPoint.x2y2,EDraggingGripPoint.x2y1,EDraggingGripPoint.x1y2\]

### dragStarted

dragStarted: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/eventhandler.html" class="tsd-signature-type">EventHandler</a>\<void\>

### hovered

hovered: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/eventhandler.html" class="tsd-signature-type">EventHandler</a>\<<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationhovereventargs.html" class="tsd-signature-type">AnnotationHoverEventArgs</a>\>

### Readonly id

id: string

A unique Id for the [IAnnotation](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iannotation.html)

### invalidateParentCallback

invalidateParentCallback: () =\> void

description  
callback which notifies the parent [SciChartSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html) it's time to draw

#### Type declaration

- - (): void

  <!-- -->

  - #### Returns void

### Protected invalidateState

invalidateState: { isHidden: boolean; x1: number; y1: number }

#### Type declaration

- ##### isHidden: boolean

- ##### x1: number

- ##### y1: number

### Readonly isDomAnnotation

isDomAnnotation: boolean = false

### Protected isHiddenProperty

isHiddenProperty: boolean = false

### Readonly isSvgAnnotation

isSvgAnnotation: boolean = false

description  
defines if the annotation is SVG annotation

### Protected parentSurfaceProperty

parentSurfaceProperty: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurfacebase.html" class="tsd-signature-type">SciChartSurfaceBase</a>

### Protected prevIsSelected

prevIsSelected: boolean = true

### Protected prevValue

prevValue: { x: number; y: number }

#### Type declaration

- ##### x: number

- ##### y: number

### Protected renderLayerProperty

renderLayerProperty: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/edefaultrenderlayer.html" class="tsd-signature-type">EDefaultRenderLayer</a> \| number

### Protected renderNextToProperty

renderNextToProperty: { offset: number; renderable: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iorderedrenderable.html" class="tsd-signature-type">IOrderedRenderable</a> \| string }

#### Type declaration

- ##### offset: number

- ##### renderable: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iorderedrenderable.html" class="tsd-signature-type">IOrderedRenderable</a> \| string

### Protected renderOrderProperty

renderOrderProperty: number = undefined

### selectedChanged

selectedChanged: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/eventhandler.html" class="tsd-signature-type">EventHandler</a>\<boolean\>

### Protected selectionBoxDeltaProperty

selectionBoxDeltaProperty: number = 1.5

### Protected selectionBoxStrokeProperty

selectionBoxStrokeProperty: string = SciChartSurfaceBase.DEFAULT_THEME.annotationSelectionStroke

### Protected selectionBoxThicknessProperty

selectionBoxThicknessProperty: number = 6

### Protected strokePenCache

strokePenCache: Pen2DCache

### Protected surfaceRenderOrderProperty

surfaceRenderOrderProperty: number = undefined

### Readonly surfaceTypes

surfaceTypes: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/esurfacetype.html" class="tsd-signature-type">ESurfaceType</a>\[\] = \[ESurfaceType.SciChartSurfaceType\]

description  
compatible surface types. See [EAnnotationType](https://www.scichart.com/documentation/js/v4/typedoc/enums/eannotationtype.html) for a list of values

### Protected svgAdorner

svgAdorner: SVGElement

### Readonly type

type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eannotationtype.html" class="tsd-signature-type">EAnnotationType</a> = EAnnotationType.RenderContextVerticalLineAnnotation

description  
annotation type. See [EAnnotationType](https://www.scichart.com/documentation/js/v4/typedoc/enums/eannotationtype.html) for a list of values

### Protected typeMap

typeMap: Map\<string, string\> = new Map\<string, string\>()

### verticalAlignment

verticalAlignment: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/everticalalignment.html" class="tsd-signature-type">EVerticalAlignment</a>

### Protected x1Property

x1Property: number

### Protected xCoordinateModeProperty

xCoordinateModeProperty: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/ecoordinatemode.html" class="tsd-signature-type">ECoordinateMode</a> = ECoordinateMode.DataValue

### Protected y1Property

y1Property: number

### Protected yCoordinateModeProperty

yCoordinateModeProperty: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/ecoordinatemode.html" class="tsd-signature-type">ECoordinateMode</a> = ECoordinateMode.DataValue

## Accessors

### adornerClipping

- get adornerClipping(): string
- set adornerClipping(value: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eannotationclippingmode.html" class="tsd-signature-type">EAnnotationClippingMode</a> \| string): void

<!-- -->

- Gets or sets mode or custom rule for adorners clipping

  #### Returns string

- Gets or sets mode or custom rule for adorners clipping

  #### Parameters

  - ##### value: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eannotationclippingmode.html" class="tsd-signature-type">EAnnotationClippingMode</a> \| string

  #### Returns void

### adornerDraggingPoint

- get adornerDraggingPoint(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/edragginggrippoint.html" class="tsd-signature-type">EDraggingGripPoint</a>
- set adornerDraggingPoint(value: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/edragginggrippoint.html" class="tsd-signature-type">EDraggingGripPoint</a>): void

<!-- -->

- Gets or sets current [EDraggingGripPoint](https://www.scichart.com/documentation/js/v4/typedoc/enums/edragginggrippoint.html)

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/edragginggrippoint.html" class="tsd-signature-type">EDraggingGripPoint</a>

- Gets or sets current [EDraggingGripPoint](https://www.scichart.com/documentation/js/v4/typedoc/enums/edragginggrippoint.html)

  #### Parameters

  - ##### value: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/edragginggrippoint.html" class="tsd-signature-type">EDraggingGripPoint</a>

  #### Returns void

### annotationLayer

- get annotationLayer(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eannotationlayer.html" class="tsd-signature-type">EAnnotationLayer</a>
- set annotationLayer(annotationCanvas: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eannotationlayer.html" class="tsd-signature-type">EAnnotationLayer</a>): void

<!-- -->

- inheritdoc  

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eannotationlayer.html" class="tsd-signature-type">EAnnotationLayer</a>

- inheritdoc  

  #### Parameters

  - ##### annotationCanvas: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eannotationlayer.html" class="tsd-signature-type">EAnnotationLayer</a>

  #### Returns void

### annotationsGripsFill

- get annotationsGripsFill(): string
- set annotationsGripsFill(color: string): void

<!-- -->

- The fill color for the adorner drag handle

  inheritdoc  

  #### Returns string

- The fill color for the adorner drag handle

  inheritdoc  

  #### Parameters

  - ##### color: string

  #### Returns void

### annotationsGripsRadius

- get annotationsGripsRadius(): number
- set annotationsGripsRadius(radius: number): void

<!-- -->

- The radius of the adorner drag handle

  inheritdoc  

  #### Returns number

- The radius of the adorner drag handle

  inheritdoc  

  #### Parameters

  - ##### radius: number

  #### Returns void

### annotationsGripsStroke

- get annotationsGripsStroke(): string
- set annotationsGripsStroke(color: string): void

<!-- -->

- The stroke color for the adorner drag handle

  inheritdoc  

  #### Returns string

- The stroke color for the adorner drag handle

  inheritdoc  

  #### Parameters

  - ##### color: string

  #### Returns void

### axisFontFamily

- get axisFontFamily(): string
- set axisFontFamily(value: string): void

<!-- -->

- Gets the axisFontSize for the [LineAnnotation](https://www.scichart.com/documentation/js/v4/typedoc/classes/lineannotation.html)

  #### Returns string

- Sets the axisFontSize for the [LineAnnotation](https://www.scichart.com/documentation/js/v4/typedoc/classes/lineannotation.html)

  #### Parameters

  - ##### value: string

  #### Returns void

### axisFontSize

- get axisFontSize(): number
- set axisFontSize(value: number): void

<!-- -->

- Gets the axisFontSize for the [LineAnnotation](https://www.scichart.com/documentation/js/v4/typedoc/classes/lineannotation.html)

  #### Returns number

- Sets the axisFontSize for the [LineAnnotation](https://www.scichart.com/documentation/js/v4/typedoc/classes/lineannotation.html)

  #### Parameters

  - ##### value: number

  #### Returns void

### axisLabelFill

- get axisLabelFill(): string
- set axisLabelFill(value: string): void

<!-- -->

- Gets the axisLabelFill for the [LineAnnotation](https://www.scichart.com/documentation/js/v4/typedoc/classes/lineannotation.html)

  #### Returns string

- Sets the axisLabelFill for the [LineAnnotation](https://www.scichart.com/documentation/js/v4/typedoc/classes/lineannotation.html)

  #### Parameters

  - ##### value: string

  #### Returns void

### axisLabelStroke

- get axisLabelStroke(): string
- set axisLabelStroke(value: string): void

<!-- -->

- Gets the axisLabelStroke for the [LineAnnotation](https://www.scichart.com/documentation/js/v4/typedoc/classes/lineannotation.html)

  #### Returns string

- Sets the axisLabelStroke for the [LineAnnotation](https://www.scichart.com/documentation/js/v4/typedoc/classes/lineannotation.html)

  #### Parameters

  - ##### value: string

  #### Returns void

### clipping

- get clipping(): string
- set clipping(value: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eannotationclippingmode.html" class="tsd-signature-type">EAnnotationClippingMode</a> \| string): void

<!-- -->

- Gets or sets mode or custom rule for clipping

  #### Returns string

- Gets or sets mode or custom rule for clipping

  #### Parameters

  - ##### value: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eannotationclippingmode.html" class="tsd-signature-type">EAnnotationClippingMode</a> \| string

  #### Returns void

### dragPoints

- get dragPoints(): keyof <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/edragginggrippoint.html" class="tsd-signature-type">EDraggingGripPoint</a>\[\]
- set dragPoints(dragPoints: keyof <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/edragginggrippoint.html" class="tsd-signature-type">EDraggingGripPoint</a>\[\]): void

<!-- -->

- Get the dragging points that should be enabled for this annotation

  #### Returns keyof <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/edragginggrippoint.html" class="tsd-signature-type">EDraggingGripPoint</a>\[\]

- Set the dragging points that should be enabled for this annotation

  #### Parameters

  - ##### dragPoints: keyof <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/edragginggrippoint.html" class="tsd-signature-type">EDraggingGripPoint</a>\[\]

  #### Returns void

### isDraggingStarted

- get isDraggingStarted(): boolean

<!-- -->

- #### Returns boolean

### isEditable

- get isEditable(): boolean
- set isEditable(isEditable: boolean): void

<!-- -->

- inheritdoc  

  #### Returns boolean

- inheritdoc  

  #### Parameters

  - ##### isEditable: boolean

  #### Returns void

### isHidden

- get isHidden(): boolean
- set isHidden(isHidden: boolean): void

<!-- -->

- inheritdoc  

  #### Returns boolean

- inheritdoc  

  #### Parameters

  - ##### isHidden: boolean

  #### Returns void

### isHovered

- get isHovered(): boolean
- set isHovered(value: boolean): void

<!-- -->

- Defines if the entity is hovered

  inheritdoc  

  #### Returns boolean

- Defines if the entity is hovered

  inheritdoc  

  #### Parameters

  - ##### value: boolean

  #### Returns void

### isSelected

- get isSelected(): boolean
- set isSelected(value: boolean): void

<!-- -->

- inheritdoc  

  #### Returns boolean

- inheritdoc  

  #### Parameters

  - ##### value: boolean

  #### Returns void

### isVerticalChart

- get isVerticalChart(): boolean

<!-- -->

- inheritdoc  

  #### Returns boolean

### isVisible

- get isVisible(): boolean
- set isVisible(value: boolean): void

<!-- -->

- #### Returns boolean

- #### Parameters

  - ##### value: boolean

  #### Returns void

### labelPlacement

- get labelPlacement(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/elabelplacement.html" class="tsd-signature-type">ELabelPlacement</a>
- set labelPlacement(value: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/elabelplacement.html" class="tsd-signature-type">ELabelPlacement</a>): void

<!-- -->

- Gets the labelPlacement for the [LineAnnotation](https://www.scichart.com/documentation/js/v4/typedoc/classes/lineannotation.html)

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/elabelplacement.html" class="tsd-signature-type">ELabelPlacement</a>

- Sets the labelPlacement for the [LineAnnotation](https://www.scichart.com/documentation/js/v4/typedoc/classes/lineannotation.html)

  #### Parameters

  - ##### value: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/elabelplacement.html" class="tsd-signature-type">ELabelPlacement</a>

  #### Returns void

### labelValue

- get labelValue(): string
- set labelValue(value: string): void

<!-- -->

- Gets the labelValue for the [LineAnnotation](https://www.scichart.com/documentation/js/v4/typedoc/classes/lineannotation.html)

  #### Returns string

- Sets the labelValue for the [LineAnnotation](https://www.scichart.com/documentation/js/v4/typedoc/classes/lineannotation.html)

  #### Parameters

  - ##### value: string

  #### Returns void

### opacity

- get opacity(): number
- set opacity(opacity: number): void

<!-- -->

- Sets an opacity override for the entire annotation, from 0..1

  inheritdoc  

  #### Returns number

- Sets an opacity override for the entire annotation, from 0..1

  inheritdoc  

  #### Parameters

  - ##### opacity: number

  #### Returns void

### parentSurface

- get parentSurface(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html" class="tsd-signature-type">SciChartSurface</a>
- set parentSurface(parentSurface: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html" class="tsd-signature-type">SciChartSurface</a>): void

<!-- -->

- inheritdoc  

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html" class="tsd-signature-type">SciChartSurface</a>

- inheritdoc  

  #### Parameters

  - ##### parentSurface: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html" class="tsd-signature-type">SciChartSurface</a>

  #### Returns void

### resizeDirections

- get resizeDirections(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/exydirection.html" class="tsd-signature-type">EXyDirection</a>
- set resizeDirections(value: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/exydirection.html" class="tsd-signature-type">EXyDirection</a>): void

<!-- -->

- inheritdoc  

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/exydirection.html" class="tsd-signature-type">EXyDirection</a>

- inheritdoc  

  #### Parameters

  - ##### value: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/exydirection.html" class="tsd-signature-type">EXyDirection</a>

  #### Returns void

### selectionBoxDelta

- get selectionBoxDelta(): number
- set selectionBoxDelta(delta: number): void

<!-- -->

- How much bigger the selection box is than the bounding box of the annotation, in pixels

  inheritdoc  

  #### Returns number

- How much bigger the selection box is than the bounding box of the annotation, in pixels

  inheritdoc  

  #### Parameters

  - ##### delta: number

  #### Returns void

### selectionBoxStroke

- get selectionBoxStroke(): string
- set selectionBoxStroke(color: string): void

<!-- -->

- The stroke color for the adorner selection box

  inheritdoc  

  #### Returns string

- The stroke color for the adorner selection box

  inheritdoc  

  #### Parameters

  - ##### color: string

  #### Returns void

### selectionBoxThickness

- get selectionBoxThickness(): number
- set selectionBoxThickness(delta: number): void

<!-- -->

- The thickness of the selection box line

  inheritdoc  

  #### Returns number

- The thickness of the selection box line

  inheritdoc  

  #### Parameters

  - ##### delta: number

  #### Returns void

### showLabel

- get showLabel(): boolean
- set showLabel(value: boolean): void

<!-- -->

- Gets the showLabel for the [LineAnnotation](https://www.scichart.com/documentation/js/v4/typedoc/classes/lineannotation.html)

  #### Returns boolean

- Sets the showLabel for the [LineAnnotation](https://www.scichart.com/documentation/js/v4/typedoc/classes/lineannotation.html)

  #### Parameters

  - ##### value: boolean

  #### Returns void

### stroke

- get stroke(): string
- set stroke(htmlColorCode: string): void

<!-- -->

- Gets the stroke for the [LineAnnotation](https://www.scichart.com/documentation/js/v4/typedoc/classes/lineannotation.html)

  remarks  
  Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

  #### Returns string

- Sets the stroke for the [LineAnnotation](https://www.scichart.com/documentation/js/v4/typedoc/classes/lineannotation.html)

  remarks  
  Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

  #### Parameters

  - ##### htmlColorCode: string

  #### Returns void

### strokeDashArray

- get strokeDashArray(): number\[\]
- set strokeDashArray(value: number\[\]): void

<!-- -->

- Gets the strokeDashArray for the [LineAnnotation](https://www.scichart.com/documentation/js/v4/typedoc/classes/lineannotation.html)

  #### Returns number\[\]

- Sets the strokeDashArray for the [LineAnnotation](https://www.scichart.com/documentation/js/v4/typedoc/classes/lineannotation.html)

  #### Parameters

  - ##### value: number\[\]

  #### Returns void

### strokeThickness

- get strokeThickness(): number
- set strokeThickness(value: number): void

<!-- -->

- Gets the strokeThickness for the [LineAnnotation](https://www.scichart.com/documentation/js/v4/typedoc/classes/lineannotation.html)

  #### Returns number

- Sets the strokeThickness for the [LineAnnotation](https://www.scichart.com/documentation/js/v4/typedoc/classes/lineannotation.html)

  #### Parameters

  - ##### value: number

  #### Returns void

### Protected svgAdornerRoot

- get svgAdornerRoot(): SVGSVGElement

<!-- -->

- #### Returns SVGSVGElement

### x1

- get x1(): number
- set x1(x1: number): void

<!-- -->

- inheritdoc  

  #### Returns number

- inheritdoc  

  #### Parameters

  - ##### x1: number

  #### Returns void

### x2

- get x2(): number
- set x2(x2: number): void

<!-- -->

- x2 property is not supported for VerticalLineAnnotation

  #### Returns number

- x2 property is not supported for VerticalLineAnnotation

  #### Parameters

  - ##### x2: number

  #### Returns void

### xAxis

- get xAxis(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase2d.html" class="tsd-signature-type">AxisBase2D</a> \| undefined

<!-- -->

- Gets the bound [XAxis](https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase2d.html) for this [IAnnotation](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iannotation.html).

  inheritdoc  

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase2d.html" class="tsd-signature-type">AxisBase2D</a> \| undefined

### xAxisId

- get xAxisId(): string \| undefined
- set xAxisId(xAxisId: string): void

<!-- -->

- inheritdoc  

  #### Returns string \| undefined

- inheritdoc  

  #### Parameters

  - ##### xAxisId: string

  #### Returns void

### xCoordinateMode

- get xCoordinateMode(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/ecoordinatemode.html" class="tsd-signature-type">ECoordinateMode</a>
- set xCoordinateMode(xCoordinateMode: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/ecoordinatemode.html" class="tsd-signature-type">ECoordinateMode</a>): void

<!-- -->

- The X-Coordinate mode. See [ECoordinateMode](https://www.scichart.com/documentation/js/v4/typedoc/enums/ecoordinatemode.html) for a list of values

  inheritdoc  

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/ecoordinatemode.html" class="tsd-signature-type">ECoordinateMode</a>

- The X-Coordinate mode. See [ECoordinateMode](https://www.scichart.com/documentation/js/v4/typedoc/enums/ecoordinatemode.html) for a list of values

  inheritdoc  

  #### Parameters

  - ##### xCoordinateMode: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/ecoordinatemode.html" class="tsd-signature-type">ECoordinateMode</a>

  #### Returns void

### y1

- get y1(): number
- set y1(y1: number): void

<!-- -->

- inheritdoc  

  #### Returns number

- inheritdoc  

  #### Parameters

  - ##### y1: number

  #### Returns void

### y2

- get y2(): number
- set y2(y2: number): void

<!-- -->

- y2 property is not supported for VerticalLineAnnotation

  #### Returns number

- y2 property is not supported for VerticalLineAnnotation

  #### Parameters

  - ##### y2: number

  #### Returns void

### yAxis

- get yAxis(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase2d.html" class="tsd-signature-type">AxisBase2D</a> \| undefined

<!-- -->

- Gets the bound [YAxis](https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase2d.html) for this [IAnnotation](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iannotation.html).

  inheritdoc  

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase2d.html" class="tsd-signature-type">AxisBase2D</a> \| undefined

### yAxisId

- get yAxisId(): string \| undefined
- set yAxisId(yAxisId: string): void

<!-- -->

- inheritdoc  

  #### Returns string \| undefined

- inheritdoc  

  #### Parameters

  - ##### yAxisId: string

  #### Returns void

### yCoordinateMode

- get yCoordinateMode(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/ecoordinatemode.html" class="tsd-signature-type">ECoordinateMode</a>
- set yCoordinateMode(yCoordinateMode: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/ecoordinatemode.html" class="tsd-signature-type">ECoordinateMode</a>): void

<!-- -->

- The Y-Coordinate mode. See [ECoordinateMode](https://www.scichart.com/documentation/js/v4/typedoc/enums/ecoordinatemode.html) for a list of values

  inheritdoc  

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/ecoordinatemode.html" class="tsd-signature-type">ECoordinateMode</a>

- The Y-Coordinate mode. See [ECoordinateMode](https://www.scichart.com/documentation/js/v4/typedoc/enums/ecoordinatemode.html) for a list of values

  inheritdoc  

  #### Parameters

  - ##### yCoordinateMode: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/ecoordinatemode.html" class="tsd-signature-type">ECoordinateMode</a>

  #### Returns void

## Methods

### Protected applyPixelRatioToDragDist

- applyPixelRatioToDragDist(dist: number): number

<!-- -->

- #### Parameters

  - ##### dist: number

  #### Returns number

### Protected applySvgClipping

- applySvgClipping(svgString: string, clipping: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eannotationclippingmode.html" class="tsd-signature-type">EAnnotationClippingMode</a> \| string): string

<!-- -->

- #### Parameters

  - ##### svgString: string

  - ##### clipping: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eannotationclippingmode.html" class="tsd-signature-type">EAnnotationClippingMode</a> \| string

  #### Returns string

### calcDragDistance

- calcDragDistance(xyValues: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/point.html" class="tsd-signature-type">Point</a>): void

<!-- -->

- #### Parameters

  - ##### xyValues: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/point.html" class="tsd-signature-type">Point</a>

  #### Returns void

### canDragPoint

- canDragPoint(dragPoint: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/edragginggrippoint.html" class="tsd-signature-type">EDraggingGripPoint</a>): boolean

<!-- -->

- Override this to disable drag behaviour for certain dragging points

  #### Parameters

  - ##### dragPoint: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/edragginggrippoint.html" class="tsd-signature-type">EDraggingGripPoint</a>

  #### Returns boolean

### checkIsClickedOnAnnotation

- checkIsClickedOnAnnotation(x: number, y: number): boolean

<!-- -->

- #### Parameters

  - ##### x: number

  - ##### y: number

  #### Returns boolean

### Protected checkIsClickedOnAnnotationInternal

- checkIsClickedOnAnnotationInternal(x: number, y: number): boolean

<!-- -->

- #### Parameters

  - ##### x: number

  - ##### y: number

  #### Returns boolean

### checkIsPointWithinClipArea

- checkIsPointWithinClipArea(mousePoint: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/point.html" class="tsd-signature-type">Point</a>): boolean

<!-- -->

- #### Parameters

  - ##### mousePoint: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/point.html" class="tsd-signature-type">Point</a>

  #### Returns boolean

### checkIsWithinBounds

- checkIsWithinBounds(args: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/modifiermouseargs.html" class="tsd-signature-type">ModifierMouseArgs</a>): boolean

<!-- -->

- Calculates if the annotation is hovered with the specified args

  #### Parameters

  - ##### args: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/modifiermouseargs.html" class="tsd-signature-type">ModifierMouseArgs</a>

  #### Returns boolean

### click

- click(args: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/modifiermouseargs.html" class="tsd-signature-type">ModifierMouseArgs</a>, selectOnClick: boolean): boolean

<!-- -->

- Called internally. Send a click to the annotation if the point is in bounds, raising the clicked event and optionally selecting the annotation.

  #### Parameters

  - ##### args: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/modifiermouseargs.html" class="tsd-signature-type">ModifierMouseArgs</a>

  - ##### selectOnClick: boolean

  #### Returns boolean

### clickToSelect

- clickToSelect(args: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/modifiermouseargs.html" class="tsd-signature-type">ModifierMouseArgs</a>): boolean

<!-- -->

- Called internally. Select the annotation if the point is in bounds. Does not raise the clicked event

  #### Parameters

  - ##### args: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/modifiermouseargs.html" class="tsd-signature-type">ModifierMouseArgs</a>

  #### Returns boolean

### Protected convertCartesianToPolar

- convertCartesianToPolar(x: number, y: number): { x: number; y: number }

<!-- -->

- For Polar surface converts Cartesian Coordinates to Polar, otherwise returns the original values

  #### Parameters

  - ##### x: number

  - ##### y: number

  #### Returns { x: number; y: number }

  - ##### x: number

  - ##### y: number

### Protected convertFromCoordinate

- convertFromCoordinate(value: number, calculator: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html" class="tsd-signature-type">CoordinateCalculatorBase</a>, coordinateMode: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/ecoordinatemode.html" class="tsd-signature-type">ECoordinateMode</a>): number

<!-- -->

- Converts an absolute coordinate to a value which could be in form of DataValue, Pixel, or Relative coordinate

  #### Parameters

  - ##### value: number

    an absolute coordinate to convert

  - ##### calculator: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html" class="tsd-signature-type">CoordinateCalculatorBase</a>

    the [CoordinateCalculatorBase](https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html) which will do the transformation

  - ##### coordinateMode: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/ecoordinatemode.html" class="tsd-signature-type">ECoordinateMode</a>

    the expected [ECoordinateMode](https://www.scichart.com/documentation/js/v4/typedoc/enums/ecoordinatemode.html) of the converted point

  #### Returns number

  the data-value, pixel, or relative value accordingly to the coordinateMode

### Protected convertPolarToCartesian

- convertPolarToCartesian(x: number, y: number, isDomAnnotation?: boolean): { x: number; y: number }

<!-- -->

- For Polar surface converts Polar Coordinates to Cartesian, otherwise returns the original values

  #### Parameters

  - ##### x: number

    is always angle for Polar

  - ##### y: number

    is always radius for Polar

  - ##### Default value isDomAnnotation: boolean = false

  #### Returns { x: number; y: number }

  - ##### x: number

  - ##### y: number

### delete

- delete(): void

<!-- -->

- Deletes native (WebAssembly) memory used by this type, after which it cannot be used.

  remarks  
  Call .delete() before finishing with the object to ensure that WebAssmembly memory leaks do not occur.

  All elements within SciChart's High Performance <a href="https://www.scichart.com/javascript-chart-features" class="external">Realtime JavaScript Charts</a> which implement [IDeletable](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html) must be deleted manually to free native (WebAssembly) memory

  #### Returns void

### Protected deleteAdorner

- deleteAdorner(): void

<!-- -->

- #### Returns void

### drawWithContext

- drawWithContext(renderContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/webglrendercontext2d.html" class="tsd-signature-type">WebGlRenderContext2D</a>, xCalc: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html" class="tsd-signature-type">CoordinateCalculatorBase</a>, yCalc: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html" class="tsd-signature-type">CoordinateCalculatorBase</a>, seriesViewRect: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>, surfaceViewRect: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>, chartViewRect: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>): void

<!-- -->

- inheritdoc  

  #### Parameters

  - ##### renderContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/webglrendercontext2d.html" class="tsd-signature-type">WebGlRenderContext2D</a>

  - ##### xCalc: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html" class="tsd-signature-type">CoordinateCalculatorBase</a>

  - ##### yCalc: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html" class="tsd-signature-type">CoordinateCalculatorBase</a>

  - ##### seriesViewRect: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>

  - ##### surfaceViewRect: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>

  - ##### chartViewRect: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>

  #### Returns void

### Protected getAbsoluteCoordinates

- getAbsoluteCoordinates(point: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/point.html" class="tsd-signature-type">Point</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/point.html" class="tsd-signature-type">Point</a>

<!-- -->

- Calculates coordinates in pixels of the specified Point. Uses the [xCoordinateMode](https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#xcoordinatemode) (or [yCoordinateMode](https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#ycoordinatemode) for vertical chart)

  #### Parameters

  - ##### point: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/point.html" class="tsd-signature-type">Point</a>

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/point.html" class="tsd-signature-type">Point</a>

### Protected getAbsoluteHorizontalCoordinate

- getAbsoluteHorizontalCoordinate(value: number): number

<!-- -->

- Calculates coordinates in pixels of the specified Point. Uses the [xCoordinateMode](https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#xcoordinatemode) (or [yCoordinateMode](https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#ycoordinatemode) for vertical chart)

  #### Parameters

  - ##### value: number

  #### Returns number

### Protected getAbsoluteVerticalCoordinate

- getAbsoluteVerticalCoordinate(value: number): number

<!-- -->

- Calculates coordinate in pixels of the specified value in the vertical dimension. Uses the [yCoordinateMode](https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#ycoordinatemode) (or [xCoordinateMode](https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#xcoordinatemode) for vertical chart)

  #### Parameters

  - ##### value: number

  #### Returns number

### getAdornerAnnotationBorders

- getAdornerAnnotationBorders(ordered?: boolean, applyDelta?: boolean): { x1: number; x2: number; y1: number; y2: number }

<!-- -->

- Returns annotation borders for the [AdornerLayer](https://www.scichart.com/documentation/js/v4/typedoc/classes/adornerlayer.html) which has the size of the whole canvas

  #### Parameters

  - ##### Default value ordered: boolean = false

    flag to return x and y values in ascending order

  - ##### Default value applyDelta: boolean = false

  #### Returns { x1: number; x2: number; y1: number; y2: number }

  - ##### x1: number

  - ##### x2: number

  - ##### y1: number

  - ##### y2: number

### getAnnotationBorders

- getAnnotationBorders(ordered?: boolean, applyDelta?: boolean): { x1: number; x2: number; y1: number; y2: number }

<!-- -->

- Returns annotationBorders

  #### Parameters

  - ##### Default value ordered: boolean = false

    flag to return x and y values in ascending order, where x1 \<= x2 and y1 \<= y2

  - ##### Default value applyDelta: boolean = false

  #### Returns { x1: number; x2: number; y1: number; y2: number }

  - ##### x1: number

  - ##### x2: number

  - ##### y1: number

  - ##### y2: number

### getAnnotationGripSvg

- getAnnotationGripSvg(x: number, y: number): string

<!-- -->

- Get svg for the adorner grip handles for standard annotations

  #### Parameters

  - ##### x: number

  - ##### y: number

  #### Returns string

### Protected getClipPath

- getClipPath(clipping: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eannotationclippingmode.html" class="tsd-signature-type">EAnnotationClippingMode</a> \| string): string

<!-- -->

- #### Parameters

  - ##### clipping: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eannotationclippingmode.html" class="tsd-signature-type">EAnnotationClippingMode</a> \| string

  #### Returns string

### getClippingRect

- getClippingRect(clipping: string \| <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eannotationclippingmode.html" class="tsd-signature-type">EAnnotationClippingMode</a>, seriesViewRect: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>, surfaceViewRect: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>, chartViewRect: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>

<!-- -->

- #### Parameters

  - ##### clipping: string \| <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eannotationclippingmode.html" class="tsd-signature-type">EAnnotationClippingMode</a>

  - ##### seriesViewRect: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>

  - ##### surfaceViewRect: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>

  - ##### chartViewRect: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rect.html" class="tsd-signature-type">Rect</a>

### Protected getCoordinate

- getCoordinate(value: number, calculator: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html" class="tsd-signature-type">CoordinateCalculatorBase</a>, coordinateMode: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/ecoordinatemode.html" class="tsd-signature-type">ECoordinateMode</a>): number

<!-- -->

- Converts a value (e.g. from [x1](https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#x1-1), [x2](https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#x2), [y1](https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#y1-1) or [y2](https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#y2)) into a pixel coordinate

  #### Parameters

  - ##### value: number

    the value to convert

  - ##### calculator: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html" class="tsd-signature-type">CoordinateCalculatorBase</a>

    the [CoordinateCalculatorBase](https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html) which will do the transformation

  - ##### coordinateMode: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/ecoordinatemode.html" class="tsd-signature-type">ECoordinateMode</a>

    the [ECoordinateMode](https://www.scichart.com/documentation/js/v4/typedoc/enums/ecoordinatemode.html) to apply

  #### Returns number

  the pixel coordinate

### getRenderLayer

- getRenderLayer(): number

<!-- -->

- The render layer grouping within which the annotation will be draw. Defaults to EDefaultRenderLayer.AnnotationsAboveSeriesLayer

  #### Returns number

### getRenderNextTo

- getRenderNextTo(): { offset: number; renderable: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iorderedrenderable.html" class="tsd-signature-type">IOrderedRenderable</a> \| string }

<!-- -->

- #### Returns { offset: number; renderable: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iorderedrenderable.html" class="tsd-signature-type">IOrderedRenderable</a> \| string }

  - ##### offset: number

  - ##### renderable: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iorderedrenderable.html" class="tsd-signature-type">IOrderedRenderable</a> \| string

### getRenderOrder

- getRenderOrder(): number

<!-- -->

- #### Returns number

### Protected getResolvedCoordinate

- getResolvedCoordinate(xCalc: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html" class="tsd-signature-type">CoordinateCalculatorBase</a>, yCalc: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html" class="tsd-signature-type">CoordinateCalculatorBase</a>, x: number, y: number, getX: boolean): number

<!-- -->

- Returns the pixel coordinate depending whether the chart is vertical

  #### Parameters

  - ##### xCalc: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html" class="tsd-signature-type">CoordinateCalculatorBase</a>

    the X [CoordinateCalculatorBase](https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html) which will do the transformation

  - ##### yCalc: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html" class="tsd-signature-type">CoordinateCalculatorBase</a>

    the Y [CoordinateCalculatorBase](https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html) which will do the transformation

  - ##### x: number

    the x value

  - ##### y: number

    the y value

  - ##### getX: boolean

    true to return the x coordinate. false for y.

  #### Returns number

  the pixel coordinate

### getSurfaceRenderOrder

- getSurfaceRenderOrder(): number

<!-- -->

- Allows an annotation to be treated as if it was drawn on a surface with a different draw order than its actual parent surface

  #### Returns number

### Protected getValue

- getValue(value: number, calculator: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html" class="tsd-signature-type">CoordinateCalculatorBase</a>, coordinateMode: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/ecoordinatemode.html" class="tsd-signature-type">ECoordinateMode</a>): number

<!-- -->

- Converts a pixel coordinate back to a value

  #### Parameters

  - ##### value: number

    coordinate or dataValue to convert

  - ##### calculator: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html" class="tsd-signature-type">CoordinateCalculatorBase</a>

    the [CoordinateCalculatorBase](https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html) which will do the transformation

  - ##### coordinateMode: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/ecoordinatemode.html" class="tsd-signature-type">ECoordinateMode</a>

    the [ECoordinateMode](https://www.scichart.com/documentation/js/v4/typedoc/enums/ecoordinatemode.html) to apply

  #### Returns number

  the data-value or value

### Protected getValuesFromCoordinates

- getValuesFromCoordinates(point: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/point.html" class="tsd-signature-type">Point</a>, translateToSeriesViewRect: boolean): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/point.html" class="tsd-signature-type">Point</a>

<!-- -->

- Transforms an absolute coordinates point to the corresponding value point. The value point has x and y converted accordingly to the the coordinate modes [xCoordinateMode](https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#xcoordinatemode) and [yCoordinateMode](https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#ycoordinatemode)

  #### Parameters

  - ##### point: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/point.html" class="tsd-signature-type">Point</a>

  - ##### translateToSeriesViewRect: boolean

    defines if the coordinates should be projected from the Canvas to SeriesViewRect

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/point.html" class="tsd-signature-type">Point</a>

  a point with coordinates [ECoordinateMode](https://www.scichart.com/documentation/js/v4/typedoc/enums/ecoordinatemode.html)

### Protected getX1Coordinate

- getX1Coordinate(xCalc: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html" class="tsd-signature-type">CoordinateCalculatorBase</a>, yCalc: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html" class="tsd-signature-type">CoordinateCalculatorBase</a>): number

<!-- -->

- Returns the pixel X1 coordinate

  #### Parameters

  - ##### xCalc: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html" class="tsd-signature-type">CoordinateCalculatorBase</a>

    the X [CoordinateCalculatorBase](https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html) which will do the transformation

  - ##### yCalc: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html" class="tsd-signature-type">CoordinateCalculatorBase</a>

    the Y [CoordinateCalculatorBase](https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html) which will do the transformation

  #### Returns number

  the pixel X1 coordinate

### Protected getX2Coordinate

- getX2Coordinate(xCalc: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html" class="tsd-signature-type">CoordinateCalculatorBase</a>, yCalc: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html" class="tsd-signature-type">CoordinateCalculatorBase</a>): number

<!-- -->

- Returns the pixel X2 coordinate

  #### Parameters

  - ##### xCalc: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html" class="tsd-signature-type">CoordinateCalculatorBase</a>

    the X [CoordinateCalculatorBase](https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html) which will do the transformation

  - ##### yCalc: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html" class="tsd-signature-type">CoordinateCalculatorBase</a>

    the Y [CoordinateCalculatorBase](https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html) which will do the transformation

  #### Returns number

  the pixel X2 coordinate

### Protected getXYCoordinatesFromValues

- getXYCoordinatesFromValues(xyDataPoint: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/point.html" class="tsd-signature-type">Point</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/point.html" class="tsd-signature-type">Point</a>

<!-- -->

- #### Parameters

  - ##### xyDataPoint: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/point.html" class="tsd-signature-type">Point</a>

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/point.html" class="tsd-signature-type">Point</a>

### Protected getY1Coordinate

- getY1Coordinate(xCalc: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html" class="tsd-signature-type">CoordinateCalculatorBase</a>, yCalc: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html" class="tsd-signature-type">CoordinateCalculatorBase</a>): number

<!-- -->

- Returns the pixel Y1 coordinate

  #### Parameters

  - ##### xCalc: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html" class="tsd-signature-type">CoordinateCalculatorBase</a>

    the X [CoordinateCalculatorBase](https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html) which will do the transformation

  - ##### yCalc: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html" class="tsd-signature-type">CoordinateCalculatorBase</a>

    the Y [CoordinateCalculatorBase](https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html) which will do the transformation

  #### Returns number

  the pixel Y1 coordinate

### Protected getY2Coordinate

- getY2Coordinate(xCalc: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html" class="tsd-signature-type">CoordinateCalculatorBase</a>, yCalc: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html" class="tsd-signature-type">CoordinateCalculatorBase</a>): number

<!-- -->

- Returns the pixel Y2 coordinate

  #### Parameters

  - ##### xCalc: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html" class="tsd-signature-type">CoordinateCalculatorBase</a>

    the X [CoordinateCalculatorBase](https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html) which will do the transformation

  - ##### yCalc: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html" class="tsd-signature-type">CoordinateCalculatorBase</a>

    the Y [CoordinateCalculatorBase](https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html) which will do the transformation

  #### Returns number

  the pixel Y2 coordinate

### hover

- hover(options: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ihoveroptions.html" class="tsd-signature-type">IHoverOptions</a>): void

<!-- -->

- Sends hover/leave action to the annotation

  #### Parameters

  - ##### options: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ihoveroptions.html" class="tsd-signature-type">IHoverOptions</a>

  #### Returns void

### linkAxes

- linkAxes(): void

<!-- -->

- Used internally - Sets references to X and Y axes

  #### Returns void

### Protected notifyPropertyChanged

- notifyPropertyChanged(propertyName: string): void

<!-- -->

- #### Parameters

  - ##### propertyName: string

  #### Returns void

### onAttach

- onAttach(scs: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html" class="tsd-signature-type">SciChartSurface</a>): void

<!-- -->

- description  
  Called when the annotation is attached to a parent SciChartSurface.

  #### Parameters

  - ##### scs: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html" class="tsd-signature-type">SciChartSurface</a>

  #### Returns void

### onDetach

- onDetach(): void

<!-- -->

- description  
  Called when the annotation is detached from a parent SciChartSurface.

  #### Returns void

### onDpiChanged

- onDpiChanged(args: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tdpichangedeventargs" class="tsd-signature-type">TDpiChangedEventArgs</a>): void

<!-- -->

- instance  

  #### Parameters

  - ##### args: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tdpichangedeventargs" class="tsd-signature-type">TDpiChangedEventArgs</a>

  #### Returns void

### onDragAdorner

- onDragAdorner(args: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/modifiermouseargs.html" class="tsd-signature-type">ModifierMouseArgs</a>): void

<!-- -->

- #### Parameters

  - ##### args: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/modifiermouseargs.html" class="tsd-signature-type">ModifierMouseArgs</a>

  #### Returns void

### onDragEnded

- onDragEnded(): void

<!-- -->

- #### Returns void

### onDragStarted

- onDragStarted(args: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/modifiermouseargs.html" class="tsd-signature-type">ModifierMouseArgs</a>): boolean

<!-- -->

- #### Parameters

  - ##### args: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/modifiermouseargs.html" class="tsd-signature-type">ModifierMouseArgs</a>

  #### Returns boolean

### resumeInvalidate

- resumeInvalidate(): void

<!-- -->

- Internal use. If isHidden,x1,y1 have change since suspendInvalidate was called, call invalidateParent

  #### Returns void

### Protected setAnnotationBorders

- setAnnotationBorders(x1: number, x2: number, y1: number, y2: number): void

<!-- -->

- Sets annotationBorders For renderContext annotations it is scaled and for SVG annotations it is not For example if we have a macbook with retina display and canvas.width = 1600px, canvas.height = 1200px, canvas.style.width = 800px, canvas.style.height = 600px If we have [BoxAnnotation](https://www.scichart.com/documentation/js/v4/typedoc/classes/boxannotation.html) (renderContext) which takes 50% width and height, located in the left-top corner it should have annotationBorders as follows x1 = 0, x2 = 800, y1 = 0, y2 = 600 But if we have [CustomAnnotation](https://www.scichart.com/documentation/js/v4/typedoc/classes/customannotation.html) (SVG) which takes 50% width and height, located in the left-top corner it should have annotationBorders as follows x1 = 0, x2 = 400, y1 = 0, y2 = 300

  #### Parameters

  - ##### x1: number

  - ##### x2: number

  - ##### y1: number

  - ##### y2: number

  #### Returns void

### setRenderLayer

- setRenderLayer(value: number \| <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/edefaultrenderlayer.html" class="tsd-signature-type">EDefaultRenderLayer</a>): void

<!-- -->

- The render layer grouping within which the series will be draw. Defaults to EDefaultRenderLayer.AnnotationsAboveSeriesLayer

  #### Parameters

  - ##### value: number \| <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/edefaultrenderlayer.html" class="tsd-signature-type">EDefaultRenderLayer</a>

  #### Returns void

### setRenderNextTo

- setRenderNextTo(renderable: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iorderedrenderable.html" class="tsd-signature-type">IOrderedRenderable</a> \| string, offset?: number): void

<!-- -->

- #### Parameters

  - ##### renderable: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iorderedrenderable.html" class="tsd-signature-type">IOrderedRenderable</a> \| string

  - ##### Default value offset: number = 0

  #### Returns void

### setRenderOrder

- setRenderOrder(value: number): void

<!-- -->

- #### Parameters

  - ##### value: number

  #### Returns void

### setSurfaceRenderOrder

- setSurfaceRenderOrder(value: number): void

<!-- -->

- Allows an annotation to be treated as if it was drawn on a surface with a different draw order than its actual parent surface

  #### Parameters

  - ##### value: number

  #### Returns void

### suspendInvalidate

- suspendInvalidate(): void

<!-- -->

- Internal use. Captures the state of isHidden,x1,y1 and prevents invalidateParent being called on change to these properties

  #### Returns void

### svgStringAdornerTemplate

- svgStringAdornerTemplate(x1: number, y1: number, x2: number, y2: number): string

<!-- -->

- #### Parameters

  - ##### x1: number

  - ##### y1: number

  - ##### x2: number

  - ##### y2: number

  #### Returns string

### toJSON

- toJSON(): { options: {}; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eannotationtype.html" class="tsd-signature-type">EAnnotationType</a> }

<!-- -->

- Convert the object to a definition that can be serialized to JSON, or used directly with the builder api

  #### Returns { options: {}; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eannotationtype.html" class="tsd-signature-type">EAnnotationType</a> }

  - ##### options: {}

  - ##### type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eannotationtype.html" class="tsd-signature-type">EAnnotationType</a>

### Protected updateAdornerInner

- updateAdornerInner(): void

<!-- -->

- #### Returns void

## Object literals

### Protected annotationBorders

annotationBorders: object

the annotation absolute coordinates

### x1

x1: number = 0

### x2

x2: number = 0

### y1

y1: number = 0

### y2

y2: number = 0

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
