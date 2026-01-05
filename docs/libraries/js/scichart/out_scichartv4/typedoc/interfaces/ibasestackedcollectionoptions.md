<img src="out_scichartv4/typedoc/interfaces/ibasestackedcollectionoptions_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibasestackedcollectionoptions.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [IBaseStackedCollectionOptions](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibasestackedcollectionoptions.html)

# Interface IBaseStackedCollectionOptions

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

Options to pass to the [BaseStackedCollection](https://www.scichart.com/documentation/js/v4/typedoc/classes/basestackedcollection.html) constructor

### Hierarchy

- IBaseStackedCollectionOptions
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/istackedcolumncollectionoptions.html" class="tsd-signature-type">IStackedColumnCollectionOptions</a>

## Index

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibasestackedcollectionoptions.html#isonehundredpercent" class="tsd-kind-icon">isOneHundredPercent</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibasestackedcollectionoptions.html#isvisible" class="tsd-kind-icon">isVisible</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibasestackedcollectionoptions.html#renderlayer" class="tsd-kind-icon">renderLayer</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibasestackedcollectionoptions.html#rendernextto" class="tsd-kind-icon">renderNextTo</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibasestackedcollectionoptions.html#renderorder" class="tsd-kind-icon">renderOrder</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibasestackedcollectionoptions.html#surfacerenderorder" class="tsd-kind-icon">surfaceRenderOrder</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibasestackedcollectionoptions.html#xaxisid" class="tsd-kind-icon">xAxisId</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibasestackedcollectionoptions.html#yaxisid" class="tsd-kind-icon">yAxisId</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibasestackedcollectionoptions.html#yrangemode" class="tsd-kind-icon">yRangeMode</a>

## Properties

### Optional isOneHundredPercent

isOneHundredPercent: boolean

Sets 100% mode. When true, the stacked group becomes a 100% stacked chart

### Optional isVisible

isVisible: boolean

When true, the series is visible and drawn

### Optional renderLayer

renderLayer: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/edefaultrenderlayer.html" class="tsd-signature-type">EDefaultRenderLayer</a> \| number

The layer to draw the series on. Defaults to SeriesRenderLayer

### Optional renderNextTo

renderNextTo: { offset: number; renderable: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iorderedrenderable.html" class="tsd-signature-type">IOrderedRenderable</a> \| string }

Another series or annotation to draw relative to. The offset can be positive or negative and can (and usually should) be fractional

#### Type declaration

- ##### offset: number

- ##### renderable: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iorderedrenderable.html" class="tsd-signature-type">IOrderedRenderable</a> \| string

### Optional renderOrder

renderOrder: number

The draw order. Larger numbers draw on top. Defaults to the position of the series in the renderableSeries array.

### Optional surfaceRenderOrder

surfaceRenderOrder: number

An override to allow the series to appear to be drawn on a different surface to the one it is attached to. Only relevant when using subcharts

### Optional xAxisId

xAxisId: string

Sets the bound [XAxis](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html) for this [BaseRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries.html).

remarks  
Does a lookup search on [SciChartSurface.xAxes](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#xaxes) collection by Id matching [this.xAxisId](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibasestackedcollectionoptions.html#xaxisid)

### Optional yAxisId

yAxisId: string

Sets the bound [YAxis](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html) for this [BaseRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries.html).

remarks  
Does a lookup search on [SciChartSurface.yAxes](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#yaxes) collection by Id matching [this.yAxisId](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibasestackedcollectionoptions.html#yaxisid)

### Optional yRangeMode

yRangeMode: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eyrangemode.html" class="tsd-signature-type">EYRangeMode</a>

Determines whether the y range for this series should consider only the visible data (the default), or include the drawn points just outside the visible range

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
