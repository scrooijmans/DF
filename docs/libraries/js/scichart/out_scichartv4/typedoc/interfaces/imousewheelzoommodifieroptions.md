<img src="out_scichartv4/typedoc/interfaces/imousewheelzoommodifieroptions_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/imousewheelzoommodifieroptions.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [IMouseWheelZoomModifierOptions](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/imousewheelzoommodifieroptions.html)

# Interface IMouseWheelZoomModifierOptions

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

Options for passing to the constructor of [MouseWheelZoomModifier](https://www.scichart.com/documentation/js/v4/typedoc/classes/mousewheelzoommodifier.html)

### Hierarchy

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ichartmodifierbaseoptions.html" class="tsd-signature-type">IChartModifierBaseOptions</a>
  - IMouseWheelZoomModifierOptions

## Index

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/imousewheelzoommodifieroptions.html#actiontype" class="tsd-kind-icon">actionType</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/imousewheelzoommodifieroptions.html#applytoaxes" class="tsd-kind-icon">applyToAxes</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/imousewheelzoommodifieroptions.html#applytoseriesviewrect" class="tsd-kind-icon">applyToSeriesViewRect</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/imousewheelzoommodifieroptions.html#excludedseriesids" class="tsd-kind-icon">excludedSeriesIds</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/imousewheelzoommodifieroptions.html#excludedxaxisids" class="tsd-kind-icon">excludedXAxisIds</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/imousewheelzoommodifieroptions.html#excludedyaxisids" class="tsd-kind-icon">excludedYAxisIds</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/imousewheelzoommodifieroptions.html#executecondition" class="tsd-kind-icon">executeCondition</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/imousewheelzoommodifieroptions.html#growfactor" class="tsd-kind-icon">growFactor</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/imousewheelzoommodifieroptions.html#id" class="tsd-kind-icon">id</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/imousewheelzoommodifieroptions.html#includedseriesids" class="tsd-kind-icon">includedSeriesIds</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/imousewheelzoommodifieroptions.html#includedxaxisids" class="tsd-kind-icon">includedXAxisIds</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/imousewheelzoommodifieroptions.html#includedyaxisids" class="tsd-kind-icon">includedYAxisIds</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/imousewheelzoommodifieroptions.html#modifiergroup" class="tsd-kind-icon">modifierGroup</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/imousewheelzoommodifieroptions.html#secondaryexecutecondition" class="tsd-kind-icon">secondaryExecuteCondition</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/imousewheelzoommodifieroptions.html#xaxisid" class="tsd-kind-icon">xAxisId</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/imousewheelzoommodifieroptions.html#xydirection" class="tsd-kind-icon">xyDirection</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/imousewheelzoommodifieroptions.html#yaxisid" class="tsd-kind-icon">yAxisId</a>

## Properties

### Optional actionType

actionType: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eactiontype.html" class="tsd-signature-type">EActionType</a>

Defines whether the Mouse Wheel zooms or pans. See [EActionType](https://www.scichart.com/documentation/js/v4/typedoc/enums/eactiontype.html) for options

### Optional applyToAxes

applyToAxes: boolean

Whether the modifier applies when the mouse is over the axes. Default true.

### Optional applyToSeriesViewRect

applyToSeriesViewRect: boolean

Whether the modifier applies when the mouse is over the chart area (ie not over the axes). Default true.

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

### Optional growFactor

growFactor: number

Modifies the speed of mousewheel zoom, for example growFactor = 0.001 means each mousewheel 'click' zooms the chart 0.1%

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

### Optional modifierGroup

modifierGroup: string

Defines the Modifier Group string - a grouping by ID for sharing mouse events across charts

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
