<img src="out_scichartv4/typedoc/interfaces/iaxisparams_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iaxisparams.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [IAxisParams](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iaxisparams.html)

# Interface IAxisParams

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

Interface to minimal set of parameters which define an [Axis](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html) in SciChart

### Hierarchy

- IAxisParams

### Implemented by

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase2d.html" class="tsd-signature-type">AxisBase2D</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase3d.html" class="tsd-signature-type">AxisBase3D</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html" class="tsd-signature-type">AxisCore</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/categoryaxis.html" class="tsd-signature-type">CategoryAxis</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/categoryaxisbase.html" class="tsd-signature-type">CategoryAxisBase</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datetimenumericaxis.html" class="tsd-signature-type">DateTimeNumericAxis</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/logarithmicaxis.html" class="tsd-signature-type">LogarithmicAxis</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numericaxis.html" class="tsd-signature-type">NumericAxis</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numericaxis3d.html" class="tsd-signature-type">NumericAxis3D</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polaraxisbase.html" class="tsd-signature-type">PolarAxisBase</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarcategoryaxis.html" class="tsd-signature-type">PolarCategoryAxis</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarnumericaxis.html" class="tsd-signature-type">PolarNumericAxis</a>

## Index

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iaxisparams.html#majordelta" class="tsd-kind-icon">majorDelta</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iaxisparams.html#minordelta" class="tsd-kind-icon">minorDelta</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iaxisparams.html#visiblerange" class="tsd-kind-icon">visibleRange</a>

## Properties

### majorDelta

majorDelta: number

The MajorDelta is the spacing between major gridlines and axis labels.

remarks  
This is internally computed via the [Delta Calculator](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#deltacalculator), however it can be explicitly set here in which case you should also set [AxisCore.minorDelta](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#minordelta) and [AxisCore.autoTicks](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#autoticks) = false.

It is also possible to override and create custom implementations of the [DeltaCalculator](https://www.scichart.com/documentation/js/v4/typedoc/classes/deltacalculator.html) for full control over axis gridline spacing.

### minorDelta

minorDelta: number

The MinorDelta is the spacing between minor gridlines.

remarks  
This is internally computed via the [Delta Calculator](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#deltacalculator), however it can be explicitly set here in which case you should also set [AxisCore.majorDelta](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#majordelta) and [AxisCore.autoTicks](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#autoticks) = false.

It is also possible to override and create custom implementations of the [DeltaCalculator](https://www.scichart.com/documentation/js/v4/typedoc/classes/deltacalculator.html) for full control over axis gridline spacing.

### visibleRange

visibleRange: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

The VisibleRange is the range of the Axis (min to max).

description  
For example, if you have data-values from 0 to 100 in your [DataSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html), but you only want to show values from 15-25 on the axis, then set the visibleRange as follows:

``` ts
axis.visibleRange = new NumberRange(15, 25);
```

remarks  
The visibleRange is a data-value for [NumericAxis](https://www.scichart.com/documentation/js/v4/typedoc/enums/eaxistype.html#numericaxis), [NumericAxis3D](https://www.scichart.com/documentation/js/v4/typedoc/enums/eaxistype.html#numericaxis3d) but refers to an **index** to the data for [CategoryAxis](https://www.scichart.com/documentation/js/v4/typedoc/enums/eaxistype.html#categoryaxis) types.

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
