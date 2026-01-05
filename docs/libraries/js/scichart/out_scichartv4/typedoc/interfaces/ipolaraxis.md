<img src="out_scichartv4/typedoc/interfaces/ipolaraxis_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipolaraxis.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [IPolarAxis](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipolaraxis.html)

# Interface IPolarAxis

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

### Hierarchy

- IPolarAxis

### Implemented by

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polaraxisbase.html" class="tsd-signature-type">PolarAxisBase</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarcategoryaxis.html" class="tsd-signature-type">PolarCategoryAxis</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarnumericaxis.html" class="tsd-signature-type">PolarNumericAxis</a>

## Index

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipolaraxis.html#isangular" class="tsd-kind-icon">isAngular</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipolaraxis.html#startangle" class="tsd-kind-icon">startAngle</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipolaraxis.html#xcenteroffset" class="tsd-kind-icon">xCenterOffset</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipolaraxis.html#ycenteroffset" class="tsd-kind-icon">yCenterOffset</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipolaraxis.html#gettransform" class="tsd-kind-icon">getTransform</a>

## Properties

### isAngular

isAngular: boolean

Gets or sets whether this polar axis is angular or radial. The X and Y axes for a series must be different modes.

### startAngle

startAngle: number

For an Angular axis, gets or sets the angle corresponding to the start of the axis. 0 is horizontally right. For a Radial axis, gets or sets the angle at which the axis is drawn

### xCenterOffset

xCenterOffset: number

Gets or sets the x offset in pixels for the center of the polar chart. Set this for the angular axis and the radial axis will use the same value.

### yCenterOffset

yCenterOffset: number

Gets or sets the y offset in pixels for the center of the polar chart. Set this for the angular axis and the radial axis will use the same value.

## Methods

### getTransform

- getTransform(): SCRTCoordinateTransform

<!-- -->

- #### Returns SCRTCoordinateTransform

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
