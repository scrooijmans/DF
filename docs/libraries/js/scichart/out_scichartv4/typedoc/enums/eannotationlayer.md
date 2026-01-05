<img src="out_scichartv4/typedoc/enums/eannotationlayer_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eannotationlayer.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [EAnnotationLayer](https://www.scichart.com/documentation/js/v4/typedoc/enums/eannotationlayer.html)

# Enumeration EAnnotationLayer

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

Defines the layer where [Annotations](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iannotation.html) are drawn when added to the [SciChartSurface.annotations](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#annotations) collection

## Index

### Enumeration members

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eannotationlayer.html#abovechart" class="tsd-kind-icon">AboveChart</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eannotationlayer.html#background" class="tsd-kind-icon">Background</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eannotationlayer.html#belowchart" class="tsd-kind-icon">BelowChart</a>

## Enumeration members

### AboveChart

AboveChart: = "AboveChart"

The [Annotation](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iannotation.html) will be displayed above the chart series and grid

remarks  
This is the default value for [Annotations](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iannotation.html)

### Background

Background: = "Background"

The [Annotation](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iannotation.html) will be displayed below the chart grid lines, bands, and axes.

remarks  
Use this for custom background behind the chart.

### BelowChart

BelowChart: = "BelowChart"

The [Annotation](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iannotation.html) will be displayed below the chart series and grid.

remarks  
Doesn't work with SVG (Custom) annotations.

Use this for watermarks, e.g. showing an image or text behind the chart. The Grid lines and Axis Bands will show over the annotation, so consider setting [AxisCore.axisBandsFill](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#axisbandsfill) to a semi-transparent color to avoid this.

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
