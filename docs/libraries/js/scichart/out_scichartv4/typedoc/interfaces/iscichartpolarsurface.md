<img src="out_scichartv4/typedoc/interfaces/iscichartpolarsurface_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iscichartpolarsurface.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [ISciChartPolarSurface](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iscichartpolarsurface.html)

# Interface ISciChartPolarSurface

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

### Hierarchy

- ISciChartPolarSurface

### Implemented by

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartpolarsubsurface.html" class="tsd-signature-type">SciChartPolarSubSurface</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartpolarsurface.html" class="tsd-signature-type">SciChartPolarSurface</a>

## Index

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iscichartpolarsurface.html#xaxes" class="tsd-kind-icon">xAxes</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iscichartpolarsurface.html#yaxes" class="tsd-kind-icon">yAxes</a>

## Properties

### Readonly xAxes

xAxes: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/observablearray.html" class="tsd-signature-type">ObservableArray</a>\<<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polaraxisbase.html" class="tsd-signature-type">PolarAxisBase</a>\>

summary  
Gets the collection of [PolarAxisBase](https://www.scichart.com/documentation/js/v4/typedoc/classes/polaraxisbase.html) - the X Axis on a [SciChartPolarSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartpolarsurface.html)

description  
A [SciChartPolarSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartpolarsurface.html) can have one to many [XAxes](https://www.scichart.com/documentation/js/v4/typedoc/classes/polaraxisbase.html).

A Polar Axis must be either Angular (around the circumference) or Radial (distance from the center) by setting {@link PolarAxisBase.isAngluar}. The X Axis can be angular or radial, but there must be a y axis of the other sort and each series must refer to one radial and one angular axis.

[AxisBase2D.axisAlignment](https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase2d.html#axisalignment) affects where the axis title is placed for an angular axis. [AxisBase2D.isInnerAxis](https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase2d.html#isinneraxis) works as expected for the angular axis. For the radial axis, isInnerAxis determines which side the ticks and labels are placed. isInnerAxis: false means clockwise of the radial line (ie below the horizontal line to the right for the default with startAngle:0). isInnerAxis: true means anticlockwise

Series and annotations may be linked to an axis via the [AxisCore.id](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#id), [BaseRenderableSeries.xAxisId](https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries.html#xaxisid) and [AnnotationBase.xAxisId](https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationbase.html#xaxisid) property.

remarks  
Adding an Axis to the chart causes it to automatically redraw. Note that Axis by default do not zoom to fit data. See the [AxisBase2D.autoRange](https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase2d.html#autorange) property for more information.

### Readonly yAxes

yAxes: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/observablearray.html" class="tsd-signature-type">ObservableArray</a>\<<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polaraxisbase.html" class="tsd-signature-type">PolarAxisBase</a>\>

summary  
Gets the collection of [PolarAxisBase](https://www.scichart.com/documentation/js/v4/typedoc/classes/polaraxisbase.html) - the Y Axis on a [SciChartPolarSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartpolarsurface.html)

description  
A [SciChartPolarSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartpolarsurface.html) can have one to many [YAxes](https://www.scichart.com/documentation/js/v4/typedoc/classes/polaraxisbase.html).

A Polar Axis must be either Angular (around the circumference) or Radial (distance from the center) by setting {@link PolarAxisBase.isAngluar}. The Y Axis can be angular or radial, but there must be a y axis of the other sort and each series must refer to one radial and one angular axis.

[AxisBase2D.axisAlignment](https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase2d.html#axisalignment) affects where the axis title is placed for an angular axis. [AxisBase2D.isInnerAxis](https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase2d.html#isinneraxis) works as expected for the angular axis. For the radial axis, isInnerAxis determines which side the ticks and labels are placed. isInnerAxis: false means clockwise of the radial line (ie below the horizontal line to the right for the default with startAngle:0). isInnerAxis: true means anticlockwise

Series and annotations may be linked to an axis via the [AxisCore.id](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#id), [BaseRenderableSeries.yAxisId](https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries.html#yaxisid) and [AnnotationBase.yAxisId](https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationbase.html#yaxisid) property.

remarks  
Adding an Axis to the chart causes it to automatically redraw. Note that Axis by default do not zoom to fit data. See the [AxisBase2D.autoRange](https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase2d.html#autorange) property for more information.

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
