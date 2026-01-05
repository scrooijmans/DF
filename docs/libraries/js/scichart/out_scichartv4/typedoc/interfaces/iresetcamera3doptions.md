<img src="out_scichartv4/typedoc/interfaces/iresetcamera3doptions_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iresetcamera3doptions.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [IResetCamera3DOptions](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iresetcamera3doptions.html)

# Interface IResetCamera3DOptions

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

Optional parameters passed to the constructor of [ResetCamera3DModifier](https://www.scichart.com/documentation/js/v4/typedoc/classes/resetcamera3dmodifier.html) to configure it

### Hierarchy

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ichartmodifierbase3doptions.html" class="tsd-signature-type">IChartModifierBase3DOptions</a>
  - IResetCamera3DOptions

## Index

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iresetcamera3doptions.html#animationduration" class="tsd-kind-icon">animationDuration</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iresetcamera3doptions.html#destination" class="tsd-kind-icon">destination</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iresetcamera3doptions.html#easingfunction" class="tsd-kind-icon">easingFunction</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iresetcamera3doptions.html#excludedseriesids" class="tsd-kind-icon">excludedSeriesIds</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iresetcamera3doptions.html#executecondition" class="tsd-kind-icon">executeCondition</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iresetcamera3doptions.html#id" class="tsd-kind-icon">id</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iresetcamera3doptions.html#includedseriesids" class="tsd-kind-icon">includedSeriesIds</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iresetcamera3doptions.html#isanimated" class="tsd-kind-icon">isAnimated</a>

## Properties

### Optional animationDuration

animationDuration: number

Defines the duration of animations when zooming in milliseconds

### Optional destination

destination: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tcamerastate" class="tsd-signature-type">TCameraState</a>

The camera settings to reset to. Defaults to the camera state when the modifier was attached to the chart.

### Optional easingFunction

easingFunction: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#teasingfn" class="tsd-signature-type">TEasingFn</a> \| string

Defines the easing function for animation. See [TEasingFn](https://www.scichart.com/documentation/js/v4/typedoc/index.html#teasingfn) for a range of functions

### Optional excludedSeriesIds

excludedSeriesIds: string\[\]

A list of 3D renderable series to exclude from this modifier

remarks  
Also see [ChartModifierBase3D.includedSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/chartmodifierbase3d.html#includedseries) which has methods to include or exclude a series by instance

### Optional executeCondition

executeCondition: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tmodifierexecutecondition" class="tsd-signature-type">TModifierExecuteCondition</a>

The primary action execute condition that modifier should respond to

### Optional id

id: string

A unique Id for the [ChartModifierBase3D](https://www.scichart.com/documentation/js/v4/typedoc/classes/chartmodifierbase3d.html)

### Optional includedSeriesIds

includedSeriesIds: string\[\]

A list of 3D renderable series to include to this modifier

remarks  
Also see [ChartModifierBase3D.includedSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/chartmodifierbase3d.html#includedseries) which has methods to include or exclude a series by instance

### Optional isAnimated

isAnimated: boolean

When true, the Zoom operations are animated. See also [animationDuration](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iresetcamera3doptions.html#animationduration) and [easingFunction](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iresetcamera3doptions.html#easingfunction)

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
