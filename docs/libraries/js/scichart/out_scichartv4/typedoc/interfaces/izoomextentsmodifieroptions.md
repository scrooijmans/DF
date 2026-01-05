<img src="out_scichartv4/typedoc/interfaces/izoomextentsmodifieroptions_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/izoomextentsmodifieroptions.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [IZoomExtentsModifierOptions](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/izoomextentsmodifieroptions.html)

# Interface IZoomExtentsModifierOptions

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

Optional parameters used to configure a [ZoomExtentsModifier](https://www.scichart.com/documentation/js/v4/typedoc/classes/zoomextentsmodifier.html) at construct time

### Hierarchy

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ichartmodifierbaseoptions.html" class="tsd-signature-type">IChartModifierBaseOptions</a>
  - IZoomExtentsModifierOptions
    - <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipolarzoomextentsmodifieroptions.html" class="tsd-signature-type">IPolarZoomExtentsModifierOptions</a>

## Index

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/izoomextentsmodifieroptions.html#animationduration" class="tsd-kind-icon">animationDuration</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/izoomextentsmodifieroptions.html#applytoaxes" class="tsd-kind-icon">applyToAxes</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/izoomextentsmodifieroptions.html#applytoseriesviewrect" class="tsd-kind-icon">applyToSeriesViewRect</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/izoomextentsmodifieroptions.html#easingfunction" class="tsd-kind-icon">easingFunction</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/izoomextentsmodifieroptions.html#excludedseriesids" class="tsd-kind-icon">excludedSeriesIds</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/izoomextentsmodifieroptions.html#excludedxaxisids" class="tsd-kind-icon">excludedXAxisIds</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/izoomextentsmodifieroptions.html#excludedyaxisids" class="tsd-kind-icon">excludedYAxisIds</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/izoomextentsmodifieroptions.html#executecondition" class="tsd-kind-icon">executeCondition</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/izoomextentsmodifieroptions.html#id" class="tsd-kind-icon">id</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/izoomextentsmodifieroptions.html#includedseriesids" class="tsd-kind-icon">includedSeriesIds</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/izoomextentsmodifieroptions.html#includedxaxisids" class="tsd-kind-icon">includedXAxisIds</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/izoomextentsmodifieroptions.html#includedyaxisids" class="tsd-kind-icon">includedYAxisIds</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/izoomextentsmodifieroptions.html#isanimated" class="tsd-kind-icon">isAnimated</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/izoomextentsmodifieroptions.html#modifiergroup" class="tsd-kind-icon">modifierGroup</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/izoomextentsmodifieroptions.html#onzoomextents" class="tsd-kind-icon">onZoomExtents</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/izoomextentsmodifieroptions.html#secondaryexecutecondition" class="tsd-kind-icon">secondaryExecuteCondition</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/izoomextentsmodifieroptions.html#xaxisid" class="tsd-kind-icon">xAxisId</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/izoomextentsmodifieroptions.html#xydirection" class="tsd-kind-icon">xyDirection</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/izoomextentsmodifieroptions.html#yaxisid" class="tsd-kind-icon">yAxisId</a>

## Properties

### Optional animationDuration

animationDuration: number

Defines the duration of animations when zooming in milliseconds

### Optional applyToAxes

applyToAxes: boolean

Whether the modifier applies when the mouse is over the axes. Default true.

### Optional applyToSeriesViewRect

applyToSeriesViewRect: boolean

Whether the modifier applies when the mouse is over the area where series are drawn (ie not over the axes). Default true.

### Optional easingFunction

easingFunction: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#teasingfn" class="tsd-signature-type">TEasingFn</a> \| string

Defines the easing function for animation. See [TEasingFn](https://www.scichart.com/documentation/js/v4/typedoc/index.html#teasingfn) for a range of functions

### Optional excludedSeriesIds

excludedSeriesIds: string\[\]

A list of renderable series to exclude from this modifier

remarks  
Also see [ChartModifierBase2D.includedSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/chartmodifierbase2d.html#includedseries) which has methods to include or exclude a series by instance

### Optional excludedXAxisIds

excludedXAxisIds: string\[\]

A list of X axes to exclude from this modifier

remarks  
Also see [ChartModifierBase2D.includedXAxes](https://www.scichart.com/documentation/js/v4/typedoc/classes/chartmodifierbase2d.html#includedxaxes) which has methods to include or exclude an axis by instance

### Optional excludedYAxisIds

excludedYAxisIds: string\[\]

A list of Y axes to exclude from this modifier

remarks  
Also see [ChartModifierBase2D.includedYAxes](https://www.scichart.com/documentation/js/v4/typedoc/classes/chartmodifierbase2d.html#includedyaxes) which has methods to include or exclude an axis by instance

### Optional executeCondition

executeCondition: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tmodifierexecutecondition" class="tsd-signature-type">TModifierExecuteCondition</a>

The primary action execute condition that modifier should respond to

### Optional id

id: string

A unique Id for the [ChartModifierBase2D](https://www.scichart.com/documentation/js/v4/typedoc/classes/chartmodifierbase2d.html)

### Optional includedSeriesIds

includedSeriesIds: string\[\]

A list of renderable series to include to this modifier

remarks  
Also see [ChartModifierBase2D.includedSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/chartmodifierbase2d.html#includedseries) which has methods to include or exclude a series by instance

### Optional includedXAxisIds

includedXAxisIds: string\[\]

A list of X axes to include to this modifier

remarks  
Also see [ChartModifierBase2D.includedXAxes](https://www.scichart.com/documentation/js/v4/typedoc/classes/chartmodifierbase2d.html#includedxaxes) which has methods to include or exclude an axis by instance

### Optional includedYAxisIds

includedYAxisIds: string\[\]

A list of Y axes to include this modifier

remarks  
Also see [ChartModifierBase2D.includedYAxes](https://www.scichart.com/documentation/js/v4/typedoc/classes/chartmodifierbase2d.html#includedyaxes) which has methods to include or exclude an axis by instance

### Optional isAnimated

isAnimated: boolean

When true, the Zoom operations are animated. See also [animationDuration](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/izoomextentsmodifieroptions.html#animationduration) and [easingFunction](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/izoomextentsmodifieroptions.html#easingfunction)

### Optional modifierGroup

modifierGroup: string

Defines the Modifier Group string - a grouping by ID for sharing mouse events across charts

### Optional onZoomExtents

onZoomExtents: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tzoomextentscallback" class="tsd-signature-type">TZoomExtentsCallback</a> \| string

A function to execute when zoomExtents is activated. If this exists and returns false, the builtin behaviour is ignored;

### Optional secondaryExecuteCondition

secondaryExecuteCondition: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tmodifierexecutecondition" class="tsd-signature-type">TModifierExecuteCondition</a>

The secondary action execute condition that modifier should respond to

### Optional xAxisId

xAxisId: string

The XAxis Id to be used by annotations internal to the modifier. Set if you have multiple x axes and need to distinguish between horizontal/vertical, or stacked axes

### Optional xyDirection

xyDirection: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/exydirection.html" class="tsd-signature-type">EXyDirection</a>

Defines the [Xy Direction](https://www.scichart.com/documentation/js/v4/typedoc/enums/exydirection.html) - whether the modifier works in X, Y or XY or neither direction, for vertical charts the behaviour could be inverted, for example for vertical chart with [RubberBandXyZoomModifier](https://www.scichart.com/documentation/js/v4/typedoc/classes/rubberbandxyzoommodifier.html) and [EXyDirection.XDirection](https://www.scichart.com/documentation/js/v4/typedoc/enums/exydirection.html#xdirection) the modifier works on Y axis

### Optional yAxisId

yAxisId: string

The YAxis Id to be used by annotations internal to the modifier. Set if you have multiple y axes and need the modifier to use something other than the first one.

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
