<img src="out_scichartv4/typedoc/interfaces/iscichart3ddefinition_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iscichart3ddefinition.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [ISciChart3DDefinition](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iscichart3ddefinition.html)

# Interface ISciChart3DDefinition

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

A definition that can be used to build a 3D Chart using [chartBuilder.buildChart](https://www.scichart.com/documentation/js/v4/typedoc/index.html#chartbuilder.buildchart) or [chartBuilder.build3DChart](https://www.scichart.com/documentation/js/v4/typedoc/index.html#chartbuilder.build3dchart)

### Hierarchy

- ISciChart3DDefinition

## Index

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iscichart3ddefinition.html#createsingle" class="tsd-kind-icon">createSingle</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iscichart3ddefinition.html#modifiers" class="tsd-kind-icon">modifiers</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iscichart3ddefinition.html#oncreated" class="tsd-kind-icon">onCreated</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iscichart3ddefinition.html#series" class="tsd-kind-icon">series</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iscichart3ddefinition.html#shareddata" class="tsd-kind-icon">sharedData</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iscichart3ddefinition.html#surface" class="tsd-kind-icon">surface</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iscichart3ddefinition.html#xaxis" class="tsd-kind-icon">xAxis</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iscichart3ddefinition.html#yaxis" class="tsd-kind-icon">yAxis</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iscichart3ddefinition.html#zaxis" class="tsd-kind-icon">zAxis</a>

## Properties

### Optional createSingle

createSingle: boolean

Whether to create this chart using a dedicated webassembly context. See [SciChart3DSurface.createSingle](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#createsingle)

### Optional modifiers

modifiers: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tmodifier3ddefinition" class="tsd-signature-type">TModifier3DDefinition</a>\[\] \| <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tmodifier3ddefinition" class="tsd-signature-type">TModifier3DDefinition</a>

One or an array of [TModifier3DDefinition](https://www.scichart.com/documentation/js/v4/typedoc/index.html#tmodifier3ddefinition) to describe modifiers to apply to the chart

### Optional onCreated

onCreated: ((surface: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html" class="tsd-signature-type">SciChart3DSurface</a>) =\> Promise\<void\>) \| string

A function, or name of a registered OnCreated function that will be run after the chart is built, receiving the sciChartSurface as a parameter

### Optional series

series: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tseriesdefinition3d" class="tsd-signature-type">TSeriesDefinition3D</a>\[\] \| <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tseriesdefinition3d" class="tsd-signature-type">TSeriesDefinition3D</a>

One or an array of [TSeriesDefinition3D](https://www.scichart.com/documentation/js/v4/typedoc/index.html#tseriesdefinition3d) to describe the renderable series

### Optional sharedData

sharedData: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tshareddatadefinition" class="tsd-signature-type">TSharedDataDefinition</a>

A [TSharedDataDefinition](https://www.scichart.com/documentation/js/v4/typedoc/index.html#tshareddatadefinition) object that defines data that can be referenced by the series

### Optional surface

surface: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/i3dsurfaceoptions.html" class="tsd-signature-type">I3DSurfaceOptions</a>

Optional [I2DSurfaceOptions](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/i2dsurfaceoptions.html) to pass to the surface

### Optional xAxis

xAxis: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#taxis3ddefinition" class="tsd-signature-type">TAxis3DDefinition</a>

One [TAxis3DDefinition](https://www.scichart.com/documentation/js/v4/typedoc/index.html#taxis3ddefinition) to describe the X Axis.

### Optional yAxis

yAxis: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#taxis3ddefinition" class="tsd-signature-type">TAxis3DDefinition</a>

One [TAxis3DDefinition](https://www.scichart.com/documentation/js/v4/typedoc/index.html#taxis3ddefinition) to describe the Y Axis.

### Optional zAxis

zAxis: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#taxis3ddefinition" class="tsd-signature-type">TAxis3DDefinition</a>

One [TAxis3DDefinition](https://www.scichart.com/documentation/js/v4/typedoc/index.html#taxis3ddefinition) to describe the Z Axis.

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
