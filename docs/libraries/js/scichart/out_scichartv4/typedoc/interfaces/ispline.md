<img src="out_scichartv4/typedoc/interfaces/ispline_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ispline.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [ISpline](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ispline.html)

# Interface ISpline

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

### Hierarchy

- ISpline

### Implemented by

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/splinebandrenderableseries.html" class="tsd-signature-type">SplineBandRenderableSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/splinelinerenderableseries.html" class="tsd-signature-type">SplineLineRenderableSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/splinemountainrenderableseries.html" class="tsd-signature-type">SplineMountainRenderableSeries</a>

## Index

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ispline.html#interpolationpoints" class="tsd-kind-icon">interpolationPoints</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ispline.html#isspline" class="tsd-kind-icon">isSpline</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ispline.html#xsplinevalues" class="tsd-kind-icon">xSplineValues</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ispline.html#ysplinevalues" class="tsd-kind-icon">ySplineValues</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ispline.html#onsplinefailure" class="tsd-kind-icon">onSplineFailure</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ispline.html#updatesplinevalues" class="tsd-kind-icon">updateSplineValues</a>

## Properties

### interpolationPoints

interpolationPoints: number

the interpolationPoints being used for the Spline

### isSpline

isSpline: boolean

it returns true for splines

### xSplineValues

xSplineValues: SCRTDoubleVector

X spline values

### ySplineValues

ySplineValues: SCRTDoubleVector

Y spline values

## Methods

### onSplineFailure

- onSplineFailure(): void

<!-- -->

- Called if the spline cannot be calculated. By default it falls back to the original data

  #### Returns void

### updateSplineValues

- updateSplineValues(): void

<!-- -->

- Updates spline values

  #### Returns void

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
