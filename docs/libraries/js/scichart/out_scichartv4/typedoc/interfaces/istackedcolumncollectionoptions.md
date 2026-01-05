<img src="out_scichartv4/typedoc/interfaces/istackedcolumncollectionoptions_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/istackedcolumncollectionoptions.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [IStackedColumnCollectionOptions](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/istackedcolumncollectionoptions.html)

# Interface IStackedColumnCollectionOptions

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

Options to pass to the [StackedColumnCollection](https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#stackedcolumncollection) constructor

### Hierarchy

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibasestackedcollectionoptions.html" class="tsd-signature-type">IBaseStackedCollectionOptions</a>
  - IStackedColumnCollectionOptions

## Index

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/istackedcolumncollectionoptions.html#datalabels" class="tsd-kind-icon">dataLabels</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/istackedcolumncollectionoptions.html#datapointwidth" class="tsd-kind-icon">dataPointWidth</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/istackedcolumncollectionoptions.html#datapointwidthmode" class="tsd-kind-icon">dataPointWidthMode</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/istackedcolumncollectionoptions.html#isonehundredpercent" class="tsd-kind-icon">isOneHundredPercent</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/istackedcolumncollectionoptions.html#isvisible" class="tsd-kind-icon">isVisible</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/istackedcolumncollectionoptions.html#renderlayer" class="tsd-kind-icon">renderLayer</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/istackedcolumncollectionoptions.html#rendernextto" class="tsd-kind-icon">renderNextTo</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/istackedcolumncollectionoptions.html#renderorder" class="tsd-kind-icon">renderOrder</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/istackedcolumncollectionoptions.html#spacing" class="tsd-kind-icon">spacing</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/istackedcolumncollectionoptions.html#surfacerenderorder" class="tsd-kind-icon">surfaceRenderOrder</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/istackedcolumncollectionoptions.html#xaxisid" class="tsd-kind-icon">xAxisId</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/istackedcolumncollectionoptions.html#yaxisid" class="tsd-kind-icon">yAxisId</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/istackedcolumncollectionoptions.html#yrangemode" class="tsd-kind-icon">yRangeMode</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/istackedcolumncollectionoptions.html#zeroliney" class="tsd-kind-icon">zeroLineY</a>

## Properties

### Optional dataLabels

dataLabels: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedcollectiondatalabelprovider.html" class="tsd-signature-type">StackedCollectionDataLabelProvider</a>

Options to pass to the DataLabelProvider. Set a style with font and size to enable per-point text for this series.

### Optional dataPointWidth

dataPointWidth: number

Sets a value used to calculate the width of columns. By default the value is treated as relative, valid values range from 0.0 - 1.0. For grouped columns, this is the width of the group, not the columns within the group. To specify if the value should be treated as relative, absolute, or based on range use [dataPointWidthMode](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/istackedcolumncollectionoptions.html#datapointwidthmode)

### Optional dataPointWidthMode

dataPointWidthMode: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/edatapointwidthmode.html" class="tsd-signature-type">EDataPointWidthMode</a>

Gets or sets the mode which determines how dataPointWidth is interpreted. Available values are [EDataPointWidthMode](https://www.scichart.com/documentation/js/v4/typedoc/enums/edatapointwidthmode.html). Default Relative.

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

### Optional spacing

spacing: number

The spacing between columns in pixels

### Optional surfaceRenderOrder

surfaceRenderOrder: number

An override to allow the series to appear to be drawn on a different surface to the one it is attached to. Only relevant when using subcharts

### Optional xAxisId

xAxisId: string

Sets the bound [XAxis](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html) for this [BaseRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries.html).

remarks  
Does a lookup search on [SciChartSurface.xAxes](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#xaxes) collection by Id matching [this.xAxisId](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/istackedcolumncollectionoptions.html#xaxisid)

### Optional yAxisId

yAxisId: string

Sets the bound [YAxis](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html) for this [BaseRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries.html).

remarks  
Does a lookup search on [SciChartSurface.yAxes](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#yaxes) collection by Id matching [this.yAxisId](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/istackedcolumncollectionoptions.html#yaxisid)

### Optional yRangeMode

yRangeMode: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eyrangemode.html" class="tsd-signature-type">EYRangeMode</a>

Determines whether the y range for this series should consider only the visible data (the default), or include the drawn points just outside the visible range

### Optional zeroLineY

zeroLineY: number

the Zero-line Y, the Y-value where the mountain crosses zero and inverts. Default is 0

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
